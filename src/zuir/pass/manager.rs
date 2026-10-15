//! ZUIR pass manager.
//!
//! # Responsibility
//!
//! Owns pass registration, deterministic pipeline order, execution budgets,
//! cancellation, pass execution records, verification boundaries, and
//! analysis invalidation notifications.
//!
//! # Architectural boundaries
//!
//! - The manager does not define ZUIR operations or types.
//! - The manager does not implement verification algorithms.
//! - The manager does not implement optimization algorithms.
//! - The manager does not allocate physical hardware resources.
//! - Passes must preserve program semantics according to their contracts.
//! - Unknown operations must be preserved or rejected explicitly by passes;
//!   they must never be silently reinterpreted.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//!
//! # Integration
//!
//! This is a contract-first implementation. When the repository's canonical
//! `pass/contracts.rs`, `pass/invalidation.rs`, and `pass/determinism.rs`
//! APIs are established, their types should own the corresponding contracts
//! and this module should import them instead of defining duplicate types.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Stable pass identity and metadata
// -----------------------------------------------------------------------------

/// Stable, namespaced pass identity.
///
/// Use names such as `core.canonicalize` or `quantum.lower_logical_ops`.
/// A pass name must not depend on hardware inventory or registration order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(String);

impl PassId {
    pub fn new(name: impl Into<String>) -> Result<Self, PassError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(PassError::InvalidPassId(
                "pass identifier must not be empty".into(),
            ));
        }

        if name.contains(char::is_whitespace) {
            return Err(PassError::InvalidPassId(format!(
                "pass identifier must not contain whitespace: {name:?}"
            )));
        }

        if !name.contains('.') {
            return Err(PassError::InvalidPassId(format!(
                "pass identifier must be namespaced: {name:?}"
            )));
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Explicit semantic version for a pass implementation.
///
/// Pipeline persistence should record this version alongside the pass ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl PassVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }
}

/// Describes the contract under which a pass may execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassDescriptor {
    pub id: PassId,
    pub version: PassVersion,
    pub description: String,
    pub required_analyses: BTreeSet<String>,
    pub invalidated_analyses: BTreeSet<String>,
    pub preserves_all_analyses: bool,
    pub requires_verified_input: bool,
    pub verifies_output: bool,
    pub deterministic: bool,
}

impl PassDescriptor {
    pub fn new(
        id: PassId,
        version: PassVersion,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            version,
            description: description.into(),
            required_analyses: BTreeSet::new(),
            invalidated_analyses: BTreeSet::new(),
            preserves_all_analyses: false,
            requires_verified_input: true,
            verifies_output: true,
            deterministic: true,
        }
    }
}

// -----------------------------------------------------------------------------
// Errors and diagnostics
// -----------------------------------------------------------------------------

/// Pass-manager failure category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassErrorKind {
    InvalidPassId,
    DuplicatePass,
    UnknownPass,
    InvalidConfiguration,
    Cancelled,
    ExecutionLimitExceeded,
    TimeBudgetExceeded,
    MissingAnalysis,
    PassFailed,
    VerificationFailed,
    NondeterministicPassRejected,
}

/// Structured pass-manager error.
///
/// `message` is diagnostic text, not a stable machine-readable error code.
/// Callers should branch on `kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassError {
    pub kind: PassErrorKind,
    pub pass: Option<PassId>,
    pub message: String,
}

impl PassError {
    pub fn new(kind: PassErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            pass: None,
            message: message.into(),
        }
    }

    pub fn for_pass(
        kind: PassErrorKind,
        pass: PassId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            message: message.into(),
        }
    }

    pub fn with_pass_if_missing(mut self, pass: &PassId) -> Self {
        if self.pass.is_none() {
            self.pass = Some(pass.clone());
        }
        self
    }
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.pass {
            Some(pass) => write!(f, "{:?} in pass {pass}: {}", self.kind, self.message),
            None => write!(f, "{:?}: {}", self.kind, self.message),
        }
    }
}

impl std::error::Error for PassError {}

// -----------------------------------------------------------------------------
// Verification integration
// -----------------------------------------------------------------------------

/// Verification boundary implemented by the canonical ZUIR verifier.
///
/// The verifier owns the actual structural, type, control-flow, effect,
/// ownership, resource, and dialect checks. The manager only orchestrates it.
pub trait IrVerifier<I>: Send + Sync {
    fn verify(&self, ir: &I) -> Result<(), PassError>;
}

/// Optional verification policy.
///
/// A missing verifier is not treated as successful verification. If a pass
/// requires verified input or output verification is enabled, execution fails
/// closed when no verifier has been configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationPolicy {
    Disabled,
    BeforeRequiredPasses,
    BeforeAndAfterEachPass,
}

// -----------------------------------------------------------------------------
// Execution limits and cancellation
// -----------------------------------------------------------------------------

/// Explicit compilation budget.
///
/// `None` means no manager-level limit for that dimension. It does not mean
/// infinite memory or unlimited operating-system resources.
#[derive(Debug, Clone)]
pub struct ExecutionBudget {
    pub max_pass_executions: Option<NonZeroUsize>,
    pub max_elapsed: Option<Duration>,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        Self {
            max_pass_executions: None,
            max_elapsed: None,
        }
    }
}

/// Thread-safe cooperative cancellation token.
///
/// Pass implementations must check cancellation during long-running work.
/// The manager cannot forcibly interrupt a pass that does not cooperate.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

// -----------------------------------------------------------------------------
// Analysis availability and invalidation
// -----------------------------------------------------------------------------

/// Analysis state visible to a pass.
///
/// Analysis implementations and result storage belong in `analysis/`.
/// The manager tracks availability, not analysis algorithms or result values.
#[derive(Debug, Clone, Default)]
pub struct AnalysisState {
    available: BTreeSet<String>,
}

impl AnalysisState {
    pub fn contains(&self, analysis: &str) -> bool {
        self.available.contains(analysis)
    }

    pub fn mark_available(&mut self, analysis: impl Into<String>) {
        self.available.insert(analysis.into());
    }

    pub fn mark_unavailable(&mut self, analysis: &str) {
        self.available.remove(analysis);
    }

    pub fn clear(&mut self) {
        self.available.clear();
    }

    pub fn available(&self) -> &BTreeSet<String> {
        &self.available
    }
}

/// Result of a successful pass execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOutcome {
    /// Whether the pass changed the IR.
    pub changed: bool,

    /// Analyses the pass guarantees remain valid.
    ///
    /// This is a correctness assertion by the pass implementation, not
    /// something the manager can prove automatically.
    pub preserved_analyses: BTreeSet<String>,
}

impl PassOutcome {
    pub fn unchanged() -> Self {
        Self {
            changed: false,
            preserved_analyses: BTreeSet::new(),
        }
    }

    pub fn changed() -> Self {
        Self {
            changed: true,
            preserved_analyses: BTreeSet::new(),
        }
    }

    pub fn preserving(
        changed: bool,
        analyses: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            changed,
            preserved_analyses: analyses.into_iter().collect(),
        }
    }
}

// -----------------------------------------------------------------------------
// Pass execution context
// -----------------------------------------------------------------------------

/// Context supplied to every pass.
///
/// The context deliberately contains no target-specific hardware assumptions.
pub struct PassContext {
    cancellation: CancellationToken,
    started_at: Instant,
    budget: ExecutionBudget,
    execution_index: usize,
    deterministic_mode: bool,
    analyses: AnalysisState,
}

impl PassContext {
    pub fn cancellation(&self) -> &CancellationToken {
        &self.cancellation
    }

    pub fn check_cancelled(&self) -> Result<(), PassError> {
        if self.cancellation.is_cancelled() {
            return Err(PassError::new(
                PassErrorKind::Cancelled,
                "pass execution was cancelled",
            ));
        }
        Ok(())
    }

    pub fn check_budget(&self) -> Result<(), PassError> {
        self.check_cancelled()?;

        if let Some(limit) = self.budget.max_elapsed {
            if self.started_at.elapsed() >= limit {
                return Err(PassError::new(
                    PassErrorKind::TimeBudgetExceeded,
                    "configured pass-pipeline time budget exceeded",
                ));
            }
        }

        Ok(())
    }

    pub fn execution_index(&self) -> usize {
        self.execution_index
    }

    pub fn deterministic_mode(&self) -> bool {
        self.deterministic_mode
    }

    pub fn analyses(&self) -> &AnalysisState {
        &self.analyses
    }

    /// Mark an analysis as available after its owning analysis provider
    /// has successfully computed and stored its result.
    pub fn mark_analysis_available(&mut self, name: impl Into<String>) {
        self.analyses.mark_available(name);
    }

    /// Remove an analysis from the available set.
    pub fn mark_analysis_unavailable(&mut self, name: &str) {
        self.analyses.mark_unavailable(name);
    }
}

// -----------------------------------------------------------------------------
// Pass contract
// -----------------------------------------------------------------------------

/// A transformation or analysis pass over a particular IR representation.
///
/// Implementations must:
///
/// - declare a stable namespaced ID and version;
/// - validate their own domain-specific preconditions;
/// - preserve semantic meaning or return an explicit error;
/// - report whether they changed the IR;
/// - accurately report preserved analyses;
/// - cooperate with cancellation and time-budget checks;
/// - avoid relying on hardware sizes fixed into the manager.
///
/// A pass that mutates IR and then returns an error may leave the IR partially
/// changed. Callers must discard or re-verify that IR before reuse. Passes
/// requiring atomic mutation must implement their own staging/commit strategy.
pub trait Pass<I>: Send {
    fn descriptor(&self) -> PassDescriptor;

    fn run(
        &mut self,
        ir: &mut I,
        context: &mut PassContext,
    ) -> Result<PassOutcome, PassError>;
}

// -----------------------------------------------------------------------------
// Pipeline records
// -----------------------------------------------------------------------------

/// Record for one completed pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassExecutionRecord {
    pub id: PassId,
    pub version: PassVersion,
    pub changed: bool,
    pub invalidated_analyses: BTreeSet<String>,
}

/// Report returned after a pipeline completes successfully.
#[derive(Debug, Clone, Default)]
pub struct PipelineReport {
    pub records: Vec<PassExecutionRecord>,
    pub changed: bool,
    pub final_available_analyses: BTreeSet<String>,
}

impl PipelineReport {
    pub fn executed_passes(&self) -> usize {
        self.records.len()
    }
}

// -----------------------------------------------------------------------------
// Manager configuration
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PassManagerConfig {
    pub budget: ExecutionBudget,
    pub verification: VerificationPolicy,
    pub deterministic_mode: bool,
    pub reject_nondeterministic_passes: bool,
}

impl Default for PassManagerConfig {
    fn default() -> Self {
        Self {
            budget: ExecutionBudget::default(),
            verification: VerificationPolicy::BeforeRequiredPasses,
            deterministic_mode: true,
            reject_nondeterministic_passes: true,
        }
    }
}

// -----------------------------------------------------------------------------
// Pass manager
// -----------------------------------------------------------------------------

/// Ordered, extensible ZUIR pass pipeline.
///
/// Pass execution order is the order in which passes are registered.
/// No implicit parallel scheduling or reordering is performed.
pub struct PassManager<I> {
    config: PassManagerConfig,
    passes: Vec<Box<dyn Pass<I>>>,
    registered: BTreeMap<PassId, usize>,
    verifier: Option<Arc<dyn IrVerifier<I>>>,
    cancellation: CancellationToken,
    initial_analyses: AnalysisState,
}

impl<I> PassManager<I> {
    pub fn new(config: PassManagerConfig) -> Self {
        Self {
            config,
            passes: Vec::new(),
            registered: BTreeMap::new(),
            verifier: None,
            cancellation: CancellationToken::new(),
            initial_analyses: AnalysisState::default(),
        }
    }

    pub fn with_verifier(mut self, verifier: Arc<dyn IrVerifier<I>>) -> Self {
        self.verifier = Some(verifier);
        self
    }

    pub fn with_cancellation(mut self, token: CancellationToken) -> Self {
        self.cancellation = token;
        self
    }

    /// Declare analyses that have already been computed for the initial IR.
    ///
    /// The caller must only register an analysis after its provider has
    /// actually computed and stored a valid result for this exact IR state.
    pub fn with_initial_analyses(
        mut self,
        analyses: impl IntoIterator<Item = String>,
    ) -> Self {
        for analysis in analyses {
            self.initial_analyses.mark_available(analysis);
        }
        self
    }

    pub fn config(&self) -> &PassManagerConfig {
        &self.config
    }

    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }

    pub fn registered_passes(&self) -> impl Iterator<Item = &PassId> {
        self.registered.keys()
    }

    /// Register a pass and reject duplicate identities.
    ///
    /// Registration order defines execution order. If a future pipeline format
    /// is persisted, it must store pass IDs and versions, not vector positions.
    pub fn add_pass(
        &mut self,
        pass: impl Pass<I> + 'static,
    ) -> Result<(), PassError> {
        let descriptor = pass.descriptor();

        if descriptor.description.trim().is_empty() {
            return Err(PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                descriptor.id,
                "pass description must not be empty",
            ));
        }

        if self.registered.contains_key(&descriptor.id) {
            return Err(PassError::for_pass(
                PassErrorKind::DuplicatePass,
                descriptor.id,
                "a pass with this ID is already registered",
            ));
        }

        if self.config.deterministic_mode
            && self.config.reject_nondeterministic_passes
            && !descriptor.deterministic
        {
            return Err(PassError::for_pass(
                PassErrorKind::NondeterministicPassRejected,
                descriptor.id,
                "nondeterministic pass rejected by manager policy",
            ));
        }

        let index = self.passes.len();
        self.registered.insert(descriptor.id, index);
        self.passes.push(Box::new(pass));
        Ok(())
    }

    /// Run the registered pipeline.
    ///
    /// On error, the IR must not be assumed valid or unchanged. A pass may
    /// have mutated it before returning an error. The caller must discard it
    /// or run the canonical verifier before any subsequent use.
    pub fn run(&mut self, ir: &mut I) -> Result<PipelineReport, PassError> {
        let started_at = Instant::now();
        let mut analyses = self.initial_analyses.clone();
        let mut report = PipelineReport::default();

        for index in 0..self.passes.len() {
            self.check_manager_budget(started_at, index)?;

            // Capture descriptor before mutable pass execution.
            let descriptor = self.passes[index].descriptor();

            if self.config.deterministic_mode
                && self.config.reject_nondeterministic_passes
                && !descriptor.deterministic
            {
                return Err(PassError::for_pass(
                    PassErrorKind::NondeterministicPassRejected,
                    descriptor.id,
                    "pipeline contains a nondeterministic pass",
                ));
            }

            for required in &descriptor.required_analyses {
                if !analyses.contains(required) {
                    return Err(PassError::for_pass(
                        PassErrorKind::MissingAnalysis,
                        descriptor.id.clone(),
                        format!("required analysis is unavailable: {required}"),
                    ));
                }
            }

            let must_verify_before = match self.config.verification {
                VerificationPolicy::Disabled => false,
                VerificationPolicy::BeforeRequiredPasses => {
                    descriptor.requires_verified_input
                }
                VerificationPolicy::BeforeAndAfterEachPass => true,
            };

            if must_verify_before {
                self.verify_ir(ir, &descriptor.id)?;
            }

            let mut context = PassContext {
                cancellation: self.cancellation.clone(),
                started_at,
                budget: self.config.budget.clone(),
                execution_index: index,
                deterministic_mode: self.config.deterministic_mode,
                analyses: analyses.clone(),
            };

            context.check_budget()?;

            let outcome = self.passes[index]
                .run(ir, &mut context)
                .map_err(|error| error.with_pass_if_missing(&descriptor.id))?;

            context.check_budget()?;

            let must_verify_after = match self.config.verification {
                VerificationPolicy::Disabled => false,
                VerificationPolicy::BeforeRequiredPasses => {
                    descriptor.verifies_output
                }
                VerificationPolicy::BeforeAndAfterEachPass => true,
            };

            if must_verify_after {
                self.verify_ir(ir, &descriptor.id)?;
            }

            // Invalidate analyses after successful mutation. Preserved analyses
            // are retained only when the pass explicitly guarantees preservation.
            let mut invalidated = BTreeSet::new();

            if outcome.changed && !descriptor.preserves_all_analyses {
                let before = analyses.available().clone();

                for analysis in before {
                    let explicitly_preserved =
                        outcome.preserved_analyses.contains(&analysis);

                    let explicitly_invalidated =
                        descriptor.invalidated_analyses.contains(&analysis);

                    if explicitly_invalidated || !explicitly_preserved {
                        analyses.mark_unavailable(&analysis);
                        invalidated.insert(analysis);
                    }
                }
            }

            // A no-change pass must not invalidate analyses merely because
            // its descriptor lists possible invalidations.
            report.changed |= outcome.changed;
            report.records.push(PassExecutionRecord {
                id: descriptor.id,
                version: descriptor.version,
                changed: outcome.changed,
                invalidated_analyses: invalidated,
            });
        }

        report.final_available_analyses = analyses.available().clone();
        Ok(report)
    }

    fn check_manager_budget(
        &self,
        started_at: Instant,
        next_index: usize,
    ) -> Result<(), PassError> {
        if self.cancellation.is_cancelled() {
            return Err(PassError::new(
                PassErrorKind::Cancelled,
                "pipeline execution was cancelled",
            ));
        }

        if let Some(limit) = self.config.budget.max_pass_executions {
            if next_index >= limit.get() {
                return Err(PassError::new(
                    PassErrorKind::ExecutionLimitExceeded,
                    "configured pass-execution limit reached",
                ));
            }
        }

        if let Some(limit) = self.config.budget.max_elapsed {
            if started_at.elapsed() >= limit {
                return Err(PassError::new(
                    PassErrorKind::TimeBudgetExceeded,
                    "configured pipeline time budget exceeded",
                ));
            }
        }

        Ok(())
    }

    fn verify_ir(&self, ir: &I, pass: &PassId) -> Result<(), PassError> {
        let verifier = self.verifier.as_ref().ok_or_else(|| {
            PassError::for_pass(
                PassErrorKind::VerificationFailed,
                pass.clone(),
                "verification is required but no verifier was configured",
            )
        })?;

        verifier
            .verify(ir)
            .map_err(|error| {
                PassError::for_pass(
                    PassErrorKind::VerificationFailed,
                    pass.clone(),
                    error.to_string(),
                )
            })
    }
}