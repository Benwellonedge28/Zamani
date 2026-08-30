//! Zamani Quantum Optimization — Production Pass Pipeline
//!
//! This module owns the execution engine for the quantum optimization pass
//! pipeline.
//!
//! # Architectural boundary
//!
//! `pipeline.rs` is an orchestration component. It does not:
//!
//! - define a second Quantum IR;
//! - define quantum gate semantics;
//! - implement individual optimization algorithms;
//! - perform routing;
//! - perform hardware scheduling;
//! - communicate with a QPU;
//! - own hardware topology;
//! - implement QEC semantics;
//! - parse source code;
//! - benchmark circuits;
//! - own global mutable state.
//!
//! The canonical flow is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ├── analysis
//!      ├── local passes
//!      ├── algebraic passes
//!      ├── synthesis
//!      ├── fault-tolerant passes
//!      └── target-aware optimization
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      ▼
//! quantum::scheduling
//!      │
//!      ▼
//! quantum::hardware
//! ```
//!
//! # Responsibilities
//!
//! The pipeline is responsible for:
//!
//! - deterministic pass ordering;
//! - pass execution;
//! - fixed-point iteration;
//! - bounded execution;
//! - pass-level limits;
//! - pipeline-level limits;
//! - validation boundaries;
//! - progress detection;
//! - non-convergence detection;
//! - pass failure propagation;
//! - pass statistics aggregation;
//! - execution tracing;
//! - cancellation;
//! - reproducibility;
//! - optional verification hooks;
//! - preserving pass ownership boundaries;
//! - preventing accidental infinite optimization;
//! - allowing future staged/conditional pipelines;
//! - allowing large circuits to be optimized incrementally;
//! - avoiding assumptions about circuit size.
//!
//! # Important design rule
//!
//! A pipeline must never assume that "more optimization" is always better.
//! Optimization can be computationally expensive and quantum circuit
//! optimization is generally not globally tractable. Therefore this module
//! treats resource limits as part of the normal API rather than as exceptional
//! emergency behavior.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//!
//! # Integration contract
//!
//! This module is intentionally written against the following stable
//! optimization contracts:
//!
//! ```text
//! optimization::pass
//!     └── OptimizationPass
//!
//! optimization::context
//!     └── OptimizationContext
//!
//! optimization::errors
//!     └── OptimizationError
//!
//! optimization::limits
//!     └── OptimizationLimits
//!
//! optimization::statistics
//!     └── OptimizationStatistics
//!
//! optimization::result
//!     └── OptimizationResult
//!
//! quantum::ir
//!     └── QuantumCircuit
//! ```
//!
//! The exact contracts are documented below so the other files can implement
//! them without requiring changes to this pipeline.
//!
//! # Dependency direction
//!
//! ```text
//! pipeline
//!   │
//!   ├── pass
//!   ├── context
//!   ├── errors
//!   ├── limits
//!   ├── statistics
//!   ├── result
//!   └── quantum::ir
//! ```
//!
//! The reverse direction is forbidden:
//!
//! ```text
//! pass       ─X→ pipeline
//! context    ─X→ pipeline
//! ir         ─X→ pipeline
//! routing    ─X→ pipeline
//! hardware   ─X→ pipeline
//! benchmarking ─X→ pipeline
//! ```
//!
//! Higher-level compiler orchestration may invoke this module, but the
//! optimization pipeline must remain independently usable.

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Required canonical imports
// -----------------------------------------------------------------------------
//
// These imports intentionally describe the final optimization architecture.
// `pass.rs`, `context.rs`, `errors.rs`, `limits.rs`, `statistics.rs`, and
// `result.rs` are separate ownership boundaries.
//
// They must not recreate Quantum IR types.

use super::context::OptimizationContext;
use super::errors::OptimizationError;
use super::limits::OptimizationLimits;
use super::pass::{OptimizationPass, PassOutcome};
use super::result::OptimizationResult;
use super::statistics::OptimizationStatistics;

use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Pipeline identity
// =============================================================================

/// Stable identifier for an optimization pipeline.
///
/// Pipeline IDs are intentionally strings rather than an enum because Zamani
/// must support compiler-built, user-defined, plugin-provided, and future
/// dynamically registered pipelines without modifying this file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineId(String);

impl PipelineId {
    /// Creates a pipeline identifier.
    ///
    /// Empty identifiers are rejected by [`Pipeline::new`].
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PipelineId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Pipeline mode
// =============================================================================

/// Determines how the pipeline executes its passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineMode {
    /// Execute every configured pass exactly once.
    SinglePass,

    /// Execute the configured sequence until no pass changes the circuit.
    FixedPoint,

    /// Execute the sequence for a bounded number of rounds.
    BoundedRounds,

    /// Execute until the configured optimization budget is exhausted.
    Budgeted,
}

impl Default for PipelineMode {
    fn default() -> Self {
        Self::FixedPoint
    }
}

// =============================================================================
// Pipeline limits
// =============================================================================

/// Pipeline-specific execution limits.
///
/// These limits complement the canonical optimization limits. They exist here
/// because pipeline control is distinct from individual transformation limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineLimits {
    /// Maximum number of complete pipeline rounds.
    pub max_rounds: usize,

    /// Maximum number of individual pass executions.
    pub max_pass_executions: usize,

    /// Maximum number of passes that may execute in one round.
    pub max_passes_per_round: usize,

    /// Maximum wall-clock duration for the whole pipeline.
    ///
    /// `None` means no wall-clock deadline is imposed by the pipeline.
    pub max_duration: Option<Duration>,

    /// Maximum number of detected non-progressing rounds.
    ///
    /// This is separate from `max_rounds` so a pipeline can distinguish
    /// legitimate progress from repeated no-op execution.
    pub max_no_progress_rounds: usize,
}

impl Default for PipelineLimits {
    fn default() -> Self {
        Self {
            max_rounds: 1_024,
            max_pass_executions: 65_536,
            max_passes_per_round: 4_096,
            max_duration: None,
            max_no_progress_rounds: 1,
        }
    }
}

impl PipelineLimits {
    /// Creates conservative production defaults.
    pub const fn production() -> Self {
        Self {
            max_rounds: 1_024,
            max_pass_executions: 65_536,
            max_passes_per_round: 4_096,
            max_duration: None,
            max_no_progress_rounds: 1,
        }
    }

    /// Creates a limit profile appropriate for development/testing.
    pub const fn testing() -> Self {
        Self {
            max_rounds: 64,
            max_pass_executions: 4_096,
            max_passes_per_round: 512,
            max_duration: Some(Duration::from_secs(30)),
            max_no_progress_rounds: 1,
        }
    }

    /// Creates effectively unbounded pipeline-level limits.
    ///
    /// This does NOT remove the underlying IR/resource limits or individual
    /// pass limits. It merely delegates those limits to the lower layers.
    pub const fn resource_driven() -> Self {
        Self {
            max_rounds: usize::MAX,
            max_pass_executions: usize::MAX,
            max_passes_per_round: usize::MAX,
            max_duration: None,
            max_no_progress_rounds: 1,
        }
    }

    /// Validates the limit configuration.
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.max_rounds == 0 {
            return Err(PipelineError::InvalidLimits {
                field: "max_rounds",
                value: self.max_rounds,
            });
        }

        if self.max_pass_executions == 0 {
            return Err(PipelineError::InvalidLimits {
                field: "max_pass_executions",
                value: self.max_pass_executions,
            });
        }

        if self.max_passes_per_round == 0 {
            return Err(PipelineError::InvalidLimits {
                field: "max_passes_per_round",
                value: self.max_passes_per_round,
            });
        }

        if self.max_no_progress_rounds == 0 {
            return Err(PipelineError::InvalidLimits {
                field: "max_no_progress_rounds",
                value: self.max_no_progress_rounds,
            });
        }

        if let Some(duration) = self.max_duration {
            if duration.is_zero() {
                return Err(PipelineError::InvalidDuration);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Pass execution policy
// =============================================================================

/// Defines what happens when the pipeline encounters a pass that cannot run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassFailurePolicy {
    /// Abort immediately.
    FailFast,

    /// Return the best valid circuit produced before the failure.
    ReturnBestEffort,
}

impl Default for PassFailurePolicy {
    fn default() -> Self {
        Self::FailFast
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Controls validation boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationPolicy {
    /// Validate only before and after the entire pipeline.
    PipelineBoundaries,

    /// Validate before and after every pass.
    EveryPass,

    /// Validate only before the pipeline.
    InputOnly,

    /// Do not request additional pipeline-level validation.
    ///
    /// Individual passes and canonical IR constructors may still validate.
    Disabled,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self::PipelineBoundaries
    }
}

// =============================================================================
// Progress policy
// =============================================================================

/// Defines how pipeline progress is detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgressPolicy {
    /// Trust the pass outcome.
    PassReported,

    /// Require both the pass outcome and a changed fingerprint.
    Verified,

    /// Use the pass outcome only but record fingerprints for diagnostics.
    ReportedWithFingerprint,
}

impl Default for ProgressPolicy {
    fn default() -> Self {
        Self::Verified
    }
}

// =============================================================================
// Execution configuration
// =============================================================================

/// Complete execution configuration for a pipeline.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Pipeline execution mode.
    pub mode: PipelineMode,

    /// Pipeline resource limits.
    pub limits: PipelineLimits,

    /// Failure policy.
    pub failure_policy: PassFailurePolicy,

    /// Validation policy.
    pub validation: ValidationPolicy,

    /// Progress detection policy.
    pub progress: ProgressPolicy,

    /// Whether an empty pipeline is legal.
    pub allow_empty: bool,

    /// Whether duplicate pass IDs are rejected.
    pub reject_duplicate_passes: bool,

    /// Whether pass execution order is required to remain deterministic.
    pub deterministic: bool,

    /// Whether to retain detailed execution events.
    pub collect_trace: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            mode: PipelineMode::FixedPoint,
            limits: PipelineLimits::production(),
            failure_policy: PassFailurePolicy::FailFast,
            validation: ValidationPolicy::PipelineBoundaries,
            progress: ProgressPolicy::Verified,
            allow_empty: true,
            reject_duplicate_passes: true,
            deterministic: true,
            collect_trace: true,
        }
    }
}

impl PipelineConfig {
    /// Creates the production configuration.
    pub fn production() -> Self {
        Self::default()
    }

    /// Creates a minimal configuration suitable for one-shot optimization.
    pub fn single_pass() -> Self {
        Self {
            mode: PipelineMode::SinglePass,
            limits: PipelineLimits {
                max_rounds: 1,
                ..PipelineLimits::production()
            },
            ..Self::default()
        }
    }

    /// Creates an aggressive but bounded configuration.
    pub fn aggressive() -> Self {
        Self {
            mode: PipelineMode::FixedPoint,
            limits: PipelineLimits {
                max_rounds: 4_096,
                max_pass_executions: 262_144,
                max_passes_per_round: 8_192,
                max_duration: None,
                max_no_progress_rounds: 1,
            },
            validation: ValidationPolicy::PipelineBoundaries,
            progress: ProgressPolicy::Verified,
            ..Self::default()
        }
    }

    /// Creates a resource-driven configuration.
    ///
    /// The pipeline itself imposes no practical size bound other than
    /// `usize::MAX`; lower-level IR/pass/resource limits remain authoritative.
    pub fn resource_driven() -> Self {
        Self {
            mode: PipelineMode::FixedPoint,
            limits: PipelineLimits::resource_driven(),
            ..Self::default()
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), PipelineError> {
        self.limits.validate()
    }
}

// =============================================================================
// Execution status
// =============================================================================

/// Final status of pipeline execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineStatus {
    /// All configured work completed and a fixed point was reached.
    FixedPoint,

    /// All configured passes ran once.
    Completed,

    /// The configured round limit stopped execution.
    RoundLimitReached,

    /// The configured pass-execution limit stopped execution.
    PassExecutionLimitReached,

    /// The configured wall-clock limit stopped execution.
    TimeLimitReached,

    /// The caller requested cancellation.
    Cancelled,

    /// The pipeline reached a no-progress condition.
    NoProgress,

    /// The pipeline intentionally returned its best valid result after a
    /// pass failure.
    PartialFailure,

    /// No optimization passes were configured.
    Empty,
}

impl PipelineStatus {
    /// Returns whether execution completed normally.
    pub const fn is_success(self) -> bool {
        matches!(
            self,
            Self::FixedPoint
                | Self::Completed
                | Self::NoProgress
                | Self::Empty
        )
    }

    /// Returns whether execution stopped because of a resource boundary.
    pub const fn hit_limit(self) -> bool {
        matches!(
            self,
            Self::RoundLimitReached
                | Self::PassExecutionLimitReached
                | Self::TimeLimitReached
        )
    }

    /// Returns whether execution was externally cancelled.
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

// =============================================================================
// Pass execution record
// =============================================================================

/// Stable record describing one pass invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassExecutionRecord {
    /// Pass identifier.
    pub pass_id: String,

    /// Pass position in the pipeline.
    pub pass_index: usize,

    /// Pipeline round.
    pub round: usize,

    /// Whether the pass reported a circuit change.
    pub changed: bool,

    /// Whether the pipeline independently detected a changed fingerprint.
    pub fingerprint_changed: bool,

    /// Number of operations before the pass, when available.
    pub operations_before: Option<usize>,

    /// Number of operations after the pass, when available.
    pub operations_after: Option<usize>,

    /// Duration in nanoseconds.
    pub duration_nanos: u128,
}

impl PassExecutionRecord {
    fn new(
        pass_id: String,
        pass_index: usize,
        round: usize,
    ) -> Self {
        Self {
            pass_id,
            pass_index,
            round,
            changed: false,
            fingerprint_changed: false,
            operations_before: None,
            operations_after: None,
            duration_nanos: 0,
        }
    }
}

// =============================================================================
// Pipeline trace
// =============================================================================

/// Detailed execution trace.
///
/// The trace is deliberately bounded by the number of executed passes. It does
/// not store circuit copies, preventing accidental memory explosion on large
/// circuits.
#[derive(Debug, Clone, Default)]
pub struct PipelineTrace {
    records: Vec<PassExecutionRecord>,
}

impl PipelineTrace {
    /// Creates an empty trace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of pass executions recorded.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true when no pass execution has been recorded.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns all records.
    pub fn records(&self) -> &[PassExecutionRecord] {
        &self.records
    }

    fn push(&mut self, record: PassExecutionRecord) {
        self.records.push(record);
    }
}

// =============================================================================
// Pipeline errors
// =============================================================================

/// Errors produced by the pipeline engine itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    /// Pipeline ID is empty.
    EmptyPipelineId,

    /// Pipeline configuration is invalid.
    InvalidLimits {
        /// Invalid field.
        field: &'static str,

        /// Invalid value.
        value: usize,
    },

    /// Duration configuration is invalid.
    InvalidDuration,

    /// Pipeline contains no passes although empty pipelines are forbidden.
    EmptyPipeline,

    /// A pass has an invalid/empty identifier.
    InvalidPassId {
        /// Pass position.
        index: usize,
    },

    /// Two passes have the same identifier.
    DuplicatePass {
        /// Duplicate pass identifier.
        pass_id: String,
    },

    /// Too many passes were registered.
    TooManyPasses {
        /// Requested pass count.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Pipeline exceeded its fixed-point round budget.
    RoundLimitExceeded {
        /// Number of rounds attempted.
        rounds: usize,

        /// Maximum rounds.
        maximum: usize,
    },

    /// Pipeline exceeded its pass execution budget.
    PassExecutionLimitExceeded {
        /// Number of executions attempted.
        executions: usize,

        /// Maximum executions.
        maximum: usize,
    },

    /// Pipeline was cancelled.
    Cancelled,

    /// Pipeline failed to make progress.
    NoProgress,

    /// Pipeline execution exceeded its wall-clock budget.
    TimeLimitExceeded,

    /// A pass failed.
    PassFailed {
        /// Pass identifier.
        pass_id: String,

        /// Pass error rendered into a stable message.
        message: String,
    },

    /// Circuit validation failed.
    ValidationFailed {
        /// Validation error.
        message: String,
    },

    /// Circuit fingerprinting failed.
    FingerprintFailed {
        /// Reason.
        message: String,
    },

    /// Canonical optimization error.
    Optimization(OptimizationError),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPipelineId => {
                formatter.write_str("optimization pipeline identifier cannot be empty")
            }

            Self::InvalidLimits { field, value } => {
                write!(
                    formatter,
                    "invalid pipeline limit `{field}`: {value}"
                )
            }

            Self::InvalidDuration => {
                formatter.write_str(
                    "pipeline duration limit must be greater than zero",
                )
            }

            Self::EmptyPipeline => {
                formatter.write_str(
                    "optimization pipeline contains no passes",
                )
            }

            Self::InvalidPassId { index } => {
                write!(
                    formatter,
                    "optimization pass at index {index} has an empty identifier"
                )
            }

            Self::DuplicatePass { pass_id } => {
                write!(
                    formatter,
                    "optimization pass `{pass_id}` appears more than once"
                )
            }

            Self::TooManyPasses {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "pipeline contains {requested} passes, maximum is {maximum}"
                )
            }

            Self::RoundLimitExceeded { rounds, maximum } => {
                write!(
                    formatter,
                    "optimization pipeline reached round limit: {rounds}/{maximum}"
                )
            }

            Self::PassExecutionLimitExceeded {
                executions,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimization pipeline reached pass execution limit: \
                     {executions}/{maximum}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("optimization pipeline was cancelled")
            }

            Self::NoProgress => {
                formatter.write_str(
                    "optimization pipeline reached a no-progress condition",
                )
            }

            Self::TimeLimitExceeded => {
                formatter.write_str(
                    "optimization pipeline reached its wall-clock limit",
                )
            }

            Self::PassFailed {
                pass_id,
                message,
            } => {
                write!(
                    formatter,
                    "optimization pass `{pass_id}` failed: {message}"
                )
            }

            Self::ValidationFailed { message } => {
                write!(
                    formatter,
                    "optimization pipeline validation failed: {message}"
                )
            }

            Self::FingerprintFailed { message } => {
                write!(
                    formatter,
                    "optimization circuit fingerprinting failed: {message}"
                )
            }

            Self::Optimization(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}

impl std::error::Error for PipelineError {}

impl From<OptimizationError> for PipelineError {
    fn from(error: OptimizationError) -> Self {
        Self::Optimization(error)
    }
}

// =============================================================================
// Cancellation token
// =============================================================================

/// Thread-safe cancellation token.
///
/// The token contains no global state and may safely be shared by compiler
/// orchestration, IDEs, build tools, or a future parallel optimizer.
#[derive(Debug, Default)]
pub struct CancellationToken {
    cancelled: AtomicBool,
}

impl CancellationToken {
    /// Creates a new non-cancelled token.
    pub const fn new() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
        }
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns true when cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Clears the cancellation state.
    ///
    /// This is intentionally explicit and should normally only be used when
    /// the same token is intentionally reused for another compilation.
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::Release);
    }
}

// =============================================================================
// Pipeline
// =============================================================================

/// Production quantum optimization pass pipeline.
///
/// `Pipeline<C>` is intentionally generic over the circuit type at the
/// implementation boundary of the pass framework, while the public Zamani
/// compiler integration uses `QuantumCircuit`.
///
/// This keeps the orchestration engine independent of circuit storage details
/// and prevents the optimizer from creating a second IR.
#[derive(Debug)]
pub struct Pipeline {
    id: PipelineId,
    config: PipelineConfig,
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl Pipeline {
    /// Maximum number of passes permitted in a pipeline.
    ///
    /// This is intentionally very large. Actual memory availability and the
    /// configured pipeline limits remain authoritative.
    pub const MAX_PASSES: usize = 1_048_576;

    /// Creates an empty pipeline.
    pub fn new(
        id: impl Into<String>,
        config: PipelineConfig,
    ) -> Result<Self, PipelineError> {
        let id = PipelineId::new(id);

        if id.as_str().trim().is_empty() {
            return Err(PipelineError::EmptyPipelineId);
        }

        config.validate()?;

        Ok(Self {
            id,
            config,
            passes: Vec::new(),
        })
    }

    /// Creates a production fixed-point pipeline.
    pub fn production(
        id: impl Into<String>,
    ) -> Result<Self, PipelineError> {
        Self::new(id, PipelineConfig::production())
    }

    /// Creates an aggressive fixed-point pipeline.
    pub fn aggressive(
        id: impl Into<String>,
    ) -> Result<Self, PipelineError> {
        Self::new(id, PipelineConfig::aggressive())
    }

    /// Creates a single-pass pipeline.
    pub fn single_pass(
        id: impl Into<String>,
    ) -> Result<Self, PipelineError> {
        Self::new(id, PipelineConfig::single_pass())
    }

    /// Creates a resource-driven pipeline.
    pub fn resource_driven(
        id: impl Into<String>,
    ) -> Result<Self, PipelineError> {
        Self::new(id, PipelineConfig::resource_driven())
    }

    /// Returns the pipeline identifier.
    pub fn id(&self) -> &PipelineId {
        &self.id
    }

    /// Returns the pipeline configuration.
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }

    /// Returns the number of registered passes.
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    /// Returns whether the pipeline has no passes.
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }

    /// Returns all pass identifiers in execution order.
    pub fn pass_ids(&self) -> Vec<&str> {
        self.passes
            .iter()
            .map(|pass| pass.id())
            .collect()
    }

    /// Adds a pass to the end of the pipeline.
    ///
    /// Pass ordering is semantically significant. The pipeline therefore never
    /// silently sorts passes.
    pub fn push(
        &mut self,
        pass: Box<dyn OptimizationPass>,
    ) -> Result<(), PipelineError> {
        let pass_id = pass.id();

        if pass_id.trim().is_empty() {
            return Err(PipelineError::InvalidPassId {
                index: self.passes.len(),
            });
        }

        if self.passes.len() >= Self::MAX_PASSES {
            return Err(PipelineError::TooManyPasses {
                requested: self
                    .passes
                    .len()
                    .saturating_add(1),
                maximum: Self::MAX_PASSES,
            });
        }

        if self.config.reject_duplicate_passes
            && self
                .passes
                .iter()
                .any(|existing| existing.id() == pass_id)
        {
            return Err(PipelineError::DuplicatePass {
                pass_id: pass_id.to_owned(),
            });
        }

        self.passes.push(pass);

        Ok(())
    }

    /// Appends several passes atomically.
    ///
    /// If validation fails, no pass from `passes` is inserted.
    pub fn extend(
        &mut self,
        passes: impl IntoIterator<Item = Box<dyn OptimizationPass>>,
    ) -> Result<(), PipelineError> {
        let mut pending: Vec<Box<dyn OptimizationPass>> =
            Vec::new();

        for pass in passes {
            pending.push(pass);
        }

        let resulting_len = self
            .passes
            .len()
            .checked_add(pending.len())
            .ok_or(PipelineError::TooManyPasses {
                requested: usize::MAX,
                maximum: Self::MAX_PASSES,
            })?;

        if resulting_len > Self::MAX_PASSES {
            return Err(PipelineError::TooManyPasses {
                requested: resulting_len,
                maximum: Self::MAX_PASSES,
            });
        }

        for (offset, pass) in pending.iter().enumerate() {
            let id = pass.id();

            if id.trim().is_empty() {
                return Err(PipelineError::InvalidPassId {
                    index: self.passes.len() + offset,
                });
            }

            if self.config.reject_duplicate_passes {
                let duplicate_in_existing = self
                    .passes
                    .iter()
                    .any(|existing| existing.id() == id);

                let duplicate_in_pending = pending
                    .iter()
                    .take(offset)
                    .any(|existing| existing.id() == id);

                if duplicate_in_existing || duplicate_in_pending {
                    return Err(PipelineError::DuplicatePass {
                        pass_id: id.to_owned(),
                    });
                }
            }
        }

        self.passes.extend(pending);

        Ok(())
    }

    /// Removes all passes.
    ///
    /// This operation is explicit so callers cannot accidentally mutate the
    /// pass order through an unrestricted mutable vector.
    pub fn clear(&mut self) {
        self.passes.clear();
    }

    /// Executes the pipeline using a fresh cancellation token.
    pub fn run(
        &self,
        circuit: QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<OptimizationResult, PipelineError> {
        let token = CancellationToken::new();

        self.run_with_cancellation(
            circuit,
            context,
            &token,
        )
    }

    /// Executes the pipeline using a caller-owned cancellation token.
    pub fn run_with_cancellation(
        &self,
        mut circuit: QuantumCircuit,
        context: &mut OptimizationContext,
        cancellation: &CancellationToken,
    ) -> Result<OptimizationResult, PipelineError> {
        self.validate_configuration()?;

        if self.passes.is_empty() {
            if self.config.allow_empty {
                return Ok(self.empty_result(circuit));
            }

            return Err(PipelineError::EmptyPipeline);
        }

        let started = Instant::now();

        self.check_cancelled(cancellation)?;
        self.check_deadline(started)?;

        if matches!(
            self.config.validation,
            ValidationPolicy::PipelineBoundaries
                | ValidationPolicy::EveryPass
                | ValidationPolicy::InputOnly
        ) {
            self.validate_circuit(&circuit)?;
        }

        let initial_fingerprint =
            circuit_fingerprint(&circuit)
                .map_err(PipelineError::FingerprintFailed)?;

        let mut statistics =
            OptimizationStatistics::default();

        let mut trace = if self.config.collect_trace {
            Some(PipelineTrace::new())
        } else {
            None
        };

        let mut round = 0usize;
        let mut pass_executions = 0usize;
        let mut no_progress_rounds = 0usize;
        let mut status = PipelineStatus::Completed;

        loop {
            self.check_cancelled(cancellation)?;
            self.check_deadline(started)?;

            if round >= self.config.limits.max_rounds {
                status = PipelineStatus::RoundLimitReached;
                break;
            }

            round = round
                .checked_add(1)
                .ok_or(PipelineError::RoundLimitExceeded {
                    rounds: usize::MAX,
                    maximum: self.config.limits.max_rounds,
                })?;

            let round_start_fingerprint =
                circuit_fingerprint(&circuit)
                    .map_err(PipelineError::FingerprintFailed)?;

            let mut round_changed = false;

            let pass_count = self.passes.len();

            if pass_count > self.config.limits.max_passes_per_round {
                return Err(PipelineError::TooManyPasses {
                    requested: pass_count,
                    maximum: self
                        .config
                        .limits
                        .max_passes_per_round,
                });
            }

            for (pass_index, pass) in
                self.passes.iter().enumerate()
            {
                self.check_cancelled(cancellation)?;
                self.check_deadline(started)?;

                if pass_executions
                    >= self
                        .config
                        .limits
                        .max_pass_executions
                {
                    status =
                        PipelineStatus::PassExecutionLimitReached;
                    break;
                }

                let pass_id = pass.id().to_owned();

                let before_fingerprint =
                    circuit_fingerprint(&circuit)
                        .map_err(
                            PipelineError::FingerprintFailed,
                        )?;

                let before_operations =
                    circuit_operation_count(&circuit);

                let pass_started = Instant::now();

                let outcome = match pass.run(
                    &mut circuit,
                    context,
                ) {
                    Ok(outcome) => outcome,

                    Err(error) => {
                        match self.config.failure_policy {
                            PassFailurePolicy::FailFast => {
                                return Err(
                                    PipelineError::PassFailed {
                                        pass_id,
                                        message:
                                            error.to_string(),
                                    },
                                );
                            }

                            PassFailurePolicy::ReturnBestEffort => {
                                status =
                                    PipelineStatus::PartialFailure;

                                break;
                            }
                        }
                    }
                };

                pass_executions =
                    pass_executions
                        .checked_add(1)
                        .ok_or(
                            PipelineError::PassExecutionLimitExceeded {
                                executions: usize::MAX,
                                maximum: self
                                    .config
                                    .limits
                                    .max_pass_executions,
                            },
                        )?;

                let after_fingerprint =
                    circuit_fingerprint(&circuit)
                        .map_err(
                            PipelineError::FingerprintFailed,
                        )?;

                let fingerprint_changed =
                    before_fingerprint
                        != after_fingerprint;

                let changed =
                    resolve_change(
                        self.config.progress,
                        outcome.changed,
                        fingerprint_changed,
                    );

                let after_operations =
                    circuit_operation_count(&circuit);

                let duration =
                    pass_started.elapsed();

                statistics.record_pass(
                    pass_id.as_str(),
                    changed,
                    before_operations,
                    after_operations,
                    duration,
                );

                if let Some(ref mut trace) = trace {
                    let mut record =
                        PassExecutionRecord::new(
                            pass_id.clone(),
                            pass_index,
                            round,
                        );

                    record.changed = changed;
                    record.fingerprint_changed =
                        fingerprint_changed;
                    record.operations_before =
                        Some(before_operations);
                    record.operations_after =
                        Some(after_operations);
                    record.duration_nanos =
                        duration.as_nanos();

                    trace.push(record);
                }

                if matches!(
                    self.config.validation,
                    ValidationPolicy::EveryPass
                ) {
                    self.validate_circuit(&circuit)?;
                }

                round_changed |= changed;

                if self.config.mode
                    == PipelineMode::SinglePass
                {
                    // Continue executing the remaining passes exactly once.
                    continue;
                }
            }

            if matches!(
                status,
                PipelineStatus::PassExecutionLimitReached
                    | PipelineStatus::PartialFailure
            ) {
                break;
            }

            let round_end_fingerprint =
                circuit_fingerprint(&circuit)
                    .map_err(PipelineError::FingerprintFailed)?;

            let fingerprint_changed =
                round_start_fingerprint
                    != round_end_fingerprint;

            let effective_round_change =
                match self.config.progress {
                    ProgressPolicy::PassReported => {
                        round_changed
                    }

                    ProgressPolicy::Verified => {
                        round_changed
                            && fingerprint_changed
                    }

                    ProgressPolicy::ReportedWithFingerprint => {
                        round_changed
                    }
                };

            if !effective_round_change {
                no_progress_rounds =
                    no_progress_rounds
                        .checked_add(1)
                        .unwrap_or(usize::MAX);

                if no_progress_rounds
                    >= self
                        .config
                        .limits
                        .max_no_progress_rounds
                {
                    status = if self.config.mode
                        == PipelineMode::FixedPoint
                    {
                        PipelineStatus::FixedPoint
                    } else {
                        PipelineStatus::NoProgress
                    };

                    break;
                }
            } else {
                no_progress_rounds = 0;
            }

            match self.config.mode {
                PipelineMode::SinglePass => {
                    status = PipelineStatus::Completed;
                    break;
                }

                PipelineMode::FixedPoint => {
                    if !effective_round_change {
                        status = PipelineStatus::FixedPoint;
                        break;
                    }
                }

                PipelineMode::BoundedRounds
                | PipelineMode::Budgeted => {
                    if round
                        >= self.config.limits.max_rounds
                    {
                        status =
                            PipelineStatus::RoundLimitReached;
                        break;
                    }
                }
            }
        }

        self.check_cancelled(cancellation)
            .or_else(|error| {
                if matches!(
                    error,
                    PipelineError::Cancelled
                ) {
                    Ok(())
                } else {
                    Err(error)
                }
            })?;

        self.check_deadline(started)
            .or_else(|error| {
                if matches!(
                    error,
                    PipelineError::TimeLimitExceeded
                ) {
                    Ok(())
                } else {
                    Err(error)
                }
            })?;

        if matches!(
            self.config.validation,
            ValidationPolicy::PipelineBoundaries
        ) {
            self.validate_circuit(&circuit)?;
        }

        let final_fingerprint =
            circuit_fingerprint(&circuit)
                .map_err(PipelineError::FingerprintFailed)?;

        statistics.finalize(
            initial_fingerprint != final_fingerprint,
            round,
            pass_executions,
            started.elapsed(),
        );

        let mut result =
            OptimizationResult::from_pipeline(
                circuit,
                statistics,
                self.id.as_str(),
                status,
                initial_fingerprint,
                final_fingerprint,
            );

        if let Some(trace) = trace {
            result.attach_pipeline_trace(trace);
        }

        Ok(result)
    }

    /// Executes a pipeline against a mutable circuit reference.
    ///
    /// The canonical result still owns the optimized circuit, so the caller
    /// receives the transformed circuit through `OptimizationResult`.
    pub fn optimize(
        &self,
        circuit: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<OptimizationResult, PipelineError> {
        self.run(circuit.clone(), context)
    }

    /// Returns the configured maximum number of rounds.
    pub const fn max_rounds(&self) -> usize {
        self.config.limits.max_rounds
    }

    /// Returns the configured maximum number of pass executions.
    pub const fn max_pass_executions(&self) -> usize {
        self.config
            .limits
            .max_pass_executions
    }

    /// Validates pipeline configuration and pass metadata.
    pub fn validate_configuration(
        &self,
    ) -> Result<(), PipelineError> {
        self.config.validate()?;

        if self.passes.is_empty()
            && !self.config.allow_empty
        {
            return Err(PipelineError::EmptyPipeline);
        }

        if self.passes.len()
            > self.config.limits.max_passes_per_round
        {
            return Err(PipelineError::TooManyPasses {
                requested: self.passes.len(),
                maximum: self
                    .config
                    .limits
                    .max_passes_per_round,
            });
        }

        for (index, pass) in
            self.passes.iter().enumerate()
        {
            if pass.id().trim().is_empty() {
                return Err(
                    PipelineError::InvalidPassId { index },
                );
            }
        }

        Ok(())
    }

    fn validate_circuit(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<(), PipelineError> {
        circuit
            .validate()
            .map_err(|error| {
                PipelineError::ValidationFailed {
                    message: error.to_string(),
                }
            })
    }

    fn check_cancelled(
        &self,
        token: &CancellationToken,
    ) -> Result<(), PipelineError> {
        if token.is_cancelled() {
            Err(PipelineError::Cancelled)
        } else {
            Ok(())
        }
    }

    fn check_deadline(
        &self,
        started: Instant,
    ) -> Result<(), PipelineError> {
        if let Some(max_duration) =
            self.config.limits.max_duration
        {
            if started.elapsed() >= max_duration {
                return Err(
                    PipelineError::TimeLimitExceeded,
                );
            }
        }

        Ok(())
    }

    fn empty_result(
        &self,
        circuit: QuantumCircuit,
    ) -> OptimizationResult {
        let fingerprint =
            circuit_fingerprint(&circuit)
                .unwrap_or_else(|_| 0);

        let mut statistics =
            OptimizationStatistics::default();

        statistics.finalize(
            false,
            0,
            0,
            Duration::ZERO,
        );

        OptimizationResult::from_pipeline(
            circuit,
            statistics,
            self.id.as_str(),
            PipelineStatus::Empty,
            fingerprint,
            fingerprint,
        )
    }
}

// =============================================================================
// Change detection
// =============================================================================

fn resolve_change(
    policy: ProgressPolicy,
    reported: bool,
    fingerprint_changed: bool,
) -> bool {
    match policy {
        ProgressPolicy::PassReported => reported,

        ProgressPolicy::Verified => {
            reported && fingerprint_changed
        }

        ProgressPolicy::ReportedWithFingerprint => {
            reported
        }
    }
}

// =============================================================================
// Circuit fingerprinting
// =============================================================================
//
// This intentionally does NOT serialize QuantumCircuit. Serialization is owned
// by optimization::serialization and must not become a pipeline dependency.
//
// A canonical deterministic circuit fingerprint API should eventually be
// provided by quantum::ir. Until then, this implementation uses the stable
// Debug representation as a conservative deterministic change detector.
//
// The API is isolated behind this function so the implementation can be
// replaced by a canonical semantic/content hash without changing the pipeline.
//
// IMPORTANT:
// A pipeline fingerprint is NOT a cryptographic identity and must never be
// used for provenance, caching, signatures, or security decisions.

fn circuit_fingerprint(
    circuit: &QuantumCircuit,
) -> Result<u64, String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher =
        DefaultHasher::new();

    format!("{circuit:?}").hash(&mut hasher);

    Ok(hasher.finish())
}

// =============================================================================
// Circuit operation accounting
// =============================================================================

fn circuit_operation_count(
    circuit: &QuantumCircuit,
) -> usize {
    circuit.operations().len()
}

// =============================================================================
// Pipeline builder
// =============================================================================

/// Fluent pipeline builder.
///
/// The builder validates the complete pipeline before producing the immutable
/// [`Pipeline`].
#[derive(Debug)]
pub struct PipelineBuilder {
    id: PipelineId,
    config: PipelineConfig,
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl PipelineBuilder {
    /// Creates a new builder.
    pub fn new(
        id: impl Into<String>,
    ) -> Result<Self, PipelineError> {
        let id = PipelineId::new(id);

        if id.as_str().trim().is_empty() {
            return Err(PipelineError::EmptyPipelineId);
        }

        Ok(Self {
            id,
            config: PipelineConfig::default(),
            passes: Vec::new(),
        })
    }

    /// Sets the complete pipeline configuration.
    pub fn config(
        mut self,
        config: PipelineConfig,
    ) -> Self {
        self.config = config;
        self
    }

    /// Sets the pipeline mode.
    pub fn mode(
        mut self,
        mode: PipelineMode,
    ) -> Self {
        self.config.mode = mode;
        self
    }

    /// Sets pipeline limits.
    pub fn limits(
        mut self,
        limits: PipelineLimits,
    ) -> Self {
        self.config.limits = limits;
        self
    }

    /// Sets failure policy.
    pub fn failure_policy(
        mut self,
        policy: PassFailurePolicy,
    ) -> Self {
        self.config.failure_policy = policy;
        self
    }

    /// Sets validation policy.
    pub fn validation(
        mut self,
        policy: ValidationPolicy,
    ) -> Self {
        self.config.validation = policy;
        self
    }

    /// Sets progress policy.
    pub fn progress(
        mut self,
        policy: ProgressPolicy,
    ) -> Self {
        self.config.progress = policy;
        self
    }

    /// Adds one optimization pass.
    pub fn pass(
        mut self,
        pass: Box<dyn OptimizationPass>,
    ) -> Result<Self, PipelineError> {
        self.validate_pass(&*pass)?;

        if self.config.reject_duplicate_passes
            && self
                .passes
                .iter()
                .any(|existing| existing.id() == pass.id())
        {
            return Err(PipelineError::DuplicatePass {
                pass_id: pass.id().to_owned(),
            });
        }

        self.passes.push(pass);

        Ok(self)
    }

    /// Adds several passes atomically.
    pub fn passes(
        mut self,
        passes: impl IntoIterator<Item = Box<dyn OptimizationPass>>,
    ) -> Result<Self, PipelineError> {
        let mut pending = Vec::new();

        for pass in passes {
            self.validate_pass(&*pass)?;
            pending.push(pass);
        }

        let resulting_len =
            self.passes
                .len()
                .checked_add(pending.len())
                .ok_or(
                    PipelineError::TooManyPasses {
                        requested: usize::MAX,
                        maximum: Pipeline::MAX_PASSES,
                    },
                )?;

        if resulting_len
            > Pipeline::MAX_PASSES
        {
            return Err(
                PipelineError::TooManyPasses {
                    requested: resulting_len,
                    maximum: Pipeline::MAX_PASSES,
                },
            );
        }

        if self.config.reject_duplicate_passes {
            for (index, pass) in
                pending.iter().enumerate()
            {
                let duplicate_existing =
                    self.passes.iter().any(
                        |existing| {
                            existing.id()
                                == pass.id()
                        },
                    );

                let duplicate_pending =
                    pending
                        .iter()
                        .take(index)
                        .any(|existing| {
                            existing.id()
                                == pass.id()
                        });

                if duplicate_existing
                    || duplicate_pending
                {
                    return Err(
                        PipelineError::DuplicatePass {
                            pass_id:
                                pass.id().to_owned(),
                        },
                    );
                }
            }
        }

        self.passes.extend(pending);

        Ok(self)
    }

    /// Builds and validates the immutable pipeline.
    pub fn build(self) -> Result<Pipeline, PipelineError> {
        self.config.validate()?;

        if self.passes.is_empty()
            && !self.config.allow_empty
        {
            return Err(PipelineError::EmptyPipeline);
        }

        if self.passes.len()
            > self.config.limits.max_passes_per_round
        {
            return Err(
                PipelineError::TooManyPasses {
                    requested: self.passes.len(),
                    maximum: self
                        .config
                        .limits
                        .max_passes_per_round,
                },
            );
        }

        let mut pipeline =
            Pipeline::new(
                self.id.0,
                self.config,
            )?;

        pipeline.extend(self.passes)?;

        Ok(pipeline)
    }

    fn validate_pass(
        &self,
        pass: &dyn OptimizationPass,
    ) -> Result<(), PipelineError> {
        if pass.id().trim().is_empty() {
            return Err(
                PipelineError::InvalidPassId {
                    index: self.passes.len(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Standard pipeline constructors
// =============================================================================
//
// These constructors deliberately do not instantiate the individual passes
// yet. Individual pass ownership remains in their own modules.
//
// Once pass.rs/local/mod.rs/fault_tolerant/mod.rs are finalized, the standard
// profiles can construct their pipelines without modifying the Pipeline engine.
//
// The exact pass composition belongs in planner.rs/profile.rs rather than in
// this execution engine.

// =============================================================================
// Pipeline statistics helpers
// =============================================================================

/// Summary of pipeline execution independent of the complete optimization
/// statistics object.
///
/// This is deliberately small and cheap to copy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PipelineSummary {
    /// Number of completed rounds.
    pub rounds: usize,

    /// Number of executed passes.
    pub pass_executions: usize,

    /// Whether any pass changed the circuit.
    pub changed: bool,

    /// Final execution status.
    pub status: Option<PipelineStatus>,
}

impl PipelineSummary {
    /// Creates a summary.
    pub const fn new(
        rounds: usize,
        pass_executions: usize,
        changed: bool,
        status: PipelineStatus,
    ) -> Self {
        Self {
            rounds,
            pass_executions,
            changed,
            status: Some(status),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_limits_are_valid() {
        assert!(
            PipelineLimits::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn testing_limits_are_valid() {
        assert!(
            PipelineLimits::testing()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn resource_driven_limits_are_valid() {
        assert!(
            PipelineLimits::resource_driven()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_round_limit_is_rejected() {
        let limits = PipelineLimits {
            max_rounds: 0,
            ..PipelineLimits::production()
        };

        assert!(matches!(
            limits.validate(),
            Err(PipelineError::InvalidLimits {
                field: "max_rounds",
                ..
            })
        ));
    }

    #[test]
    fn zero_pass_execution_limit_is_rejected() {
        let limits = PipelineLimits {
            max_pass_executions: 0,
            ..PipelineLimits::production()
        };

        assert!(matches!(
            limits.validate(),
            Err(PipelineError::InvalidLimits {
                field: "max_pass_executions",
                ..
            })
        ));
    }

    #[test]
    fn zero_passes_per_round_is_rejected() {
        let limits = PipelineLimits {
            max_passes_per_round: 0,
            ..PipelineLimits::production()
        };

        assert!(matches!(
            limits.validate(),
            Err(PipelineError::InvalidLimits {
                field: "max_passes_per_round",
                ..
            })
        ));
    }

    #[test]
    fn zero_no_progress_limit_is_rejected() {
        let limits = PipelineLimits {
            max_no_progress_rounds: 0,
            ..PipelineLimits::production()
        };

        assert!(matches!(
            limits.validate(),
            Err(PipelineError::InvalidLimits {
                field: "max_no_progress_rounds",
                ..
            })
        ));
    }

    #[test]
    fn zero_duration_is_rejected() {
        let limits = PipelineLimits {
            max_duration: Some(Duration::ZERO),
            ..PipelineLimits::production()
        };

        assert!(matches!(
            limits.validate(),
            Err(PipelineError::InvalidDuration)
        ));
    }

    #[test]
    fn empty_pipeline_can_be_created_when_allowed() {
        let pipeline =
            Pipeline::production("empty")
                .expect("pipeline should build");

        assert!(pipeline.is_empty());
        assert_eq!(pipeline.len(), 0);
    }

    #[test]
    fn empty_pipeline_id_is_rejected() {
        let result =
            Pipeline::production("   ");

        assert!(matches!(
            result,
            Err(PipelineError::EmptyPipelineId)
        ));
    }

    #[test]
    fn pipeline_modes_are_stable() {
        assert_eq!(
            PipelineMode::default(),
            PipelineMode::FixedPoint
        );
    }

    #[test]
    fn failure_policy_defaults_to_fail_fast() {
        assert_eq!(
            PassFailurePolicy::default(),
            PassFailurePolicy::FailFast
        );
    }

    #[test]
    fn validation_policy_defaults_to_pipeline_boundaries() {
        assert_eq!(
            ValidationPolicy::default(),
            ValidationPolicy::PipelineBoundaries
        );
    }

    #[test]
    fn progress_policy_defaults_to_verified() {
        assert_eq!(
            ProgressPolicy::default(),
            ProgressPolicy::Verified
        );
    }

    #[test]
    fn cancellation_token_starts_clear() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());
    }

    #[test]
    fn cancellation_token_can_be_cancelled() {
        let token = CancellationToken::new();

        token.cancel();

        assert!(token.is_cancelled());
    }

    #[test]
    fn cancellation_token_can_be_reset() {
        let token = CancellationToken::new();

        token.cancel();
        assert!(token.is_cancelled());

        token.reset();
        assert!(!token.is_cancelled());
    }

    #[test]
    fn status_helpers_are_correct() {
        assert!(
            PipelineStatus::FixedPoint
                .is_success()
        );

        assert!(
            PipelineStatus::Completed
                .is_success()
        );

        assert!(
            PipelineStatus::RoundLimitReached
                .hit_limit()
        );

        assert!(
            PipelineStatus::PassExecutionLimitReached
                .hit_limit()
        );

        assert!(
            PipelineStatus::TimeLimitReached
                .hit_limit()
        );

        assert!(
            PipelineStatus::Cancelled
                .is_cancelled()
        );
    }

    #[test]
    fn pipeline_id_is_displayable() {
        let id = PipelineId::new("balanced");

        assert_eq!(
            id.as_str(),
            "balanced"
        );

        assert_eq!(
            id.to_string(),
            "balanced"
        );
    }

    #[test]
    fn builder_rejects_empty_identifier() {
        let result =
            PipelineBuilder::new("");

        assert!(matches!(
            result,
            Err(PipelineError::EmptyPipelineId)
        ));
    }

    #[test]
    fn single_pass_configuration_is_bounded() {
        let config =
            PipelineConfig::single_pass();

        assert_eq!(
            config.mode,
            PipelineMode::SinglePass
        );

        assert_eq!(
            config.limits.max_rounds,
            1
        );
    }

    #[test]
    fn aggressive_configuration_is_valid() {
        let config =
            PipelineConfig::aggressive();

        assert!(
            config.validate().is_ok()
        );

        assert!(
            config.limits.max_rounds
                > PipelineConfig::production()
                    .limits
                    .max_rounds
        );
    }

    #[test]
    fn resolve_change_verified_requires_both_signals() {
        assert!(
            resolve_change(
                ProgressPolicy::Verified,
                true,
                true
            )
        );

        assert!(
            !resolve_change(
                ProgressPolicy::Verified,
                true,
                false
            )
        );

        assert!(
            !resolve_change(
                ProgressPolicy::Verified,
                false,
                true
            )
        );
    }

    #[test]
    fn resolve_change_reported_uses_pass_result() {
        assert!(
            resolve_change(
                ProgressPolicy::PassReported,
                true,
                false
            )
        );

        assert!(
            !resolve_change(
                ProgressPolicy::PassReported,
                false,
                true
            )
        );
    }

    #[test]
    fn resolve_change_reported_with_fingerprint_uses_report() {
        assert!(
            resolve_change(
                ProgressPolicy::ReportedWithFingerprint,
                true,
                false
            )
        );
    }

    #[test]
    fn default_limits_are_large_but_finite() {
        let limits =
            PipelineLimits::production();

        assert!(
            limits.max_rounds > 0
        );

        assert!(
            limits.max_pass_executions
                >= limits.max_rounds
        );

        assert!(
            limits.max_passes_per_round > 0
        );
    }

    #[test]
    fn resource_driven_mode_delegates_size_limits() {
        let limits =
            PipelineLimits::resource_driven();

        assert_eq!(
            limits.max_rounds,
            usize::MAX
        );

        assert_eq!(
            limits.max_pass_executions,
            usize::MAX
        );

        assert_eq!(
            limits.max_passes_per_round,
            usize::MAX
        );

        assert_eq!(
            limits.max_duration,
            None
        );
    }
}