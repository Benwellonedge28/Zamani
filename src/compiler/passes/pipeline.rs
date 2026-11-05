
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Pipeline
//!
//! Path: src/compiler/passes/pipeline.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide a compiler-facing, explicitly ordered, validated sequence of
//! compiler passes.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! This module delegates pass execution to:
//!
//!     crate::zuir::pass::manager
//!
//! It uses the compiler-facing compatibility facade:
//!
//!     crate::compiler::passes::manager
//!
//! Pass registration and lookup remain owned by:
//!
//!     crate::zuir::pass::registry
//!
//! Canonical pass contracts remain owned by:
//!
//!     crate::zuir::pass::contracts
//!
//! This module MUST NOT implement another pass execution engine.
//!
//! ---------------------------------------------------------------------------
//! OWNERSHIP
//! ---------------------------------------------------------------------------
//!
//! OWNS:
//!
//! - Ordered compiler pipeline composition.
//! - Compiler pipeline preflight validation.
//! - Pipeline-level pass descriptor snapshots.
//! - Pipeline-level determinism declaration checks.
//! - Compiler-facing pipeline execution entry point.
//! - Pipeline validation reports.
//!
//! DOES NOT OWN:
//!
//! - Pass implementation or pass algorithms.
//! - Canonical pass identities or canonical descriptors.
//! - Pass registration infrastructure.
//! - IR definitions or IR verification algorithms.
//! - Analysis computation or analysis result storage.
//! - Analysis invalidation algorithms.
//! - Optimization algorithms.
//! - Semantic analysis.
//! - Target discovery or hardware selection.
//! - Physical allocation, scheduling, or routing.
//! - Runtime execution.
//!
//! ---------------------------------------------------------------------------
//! EXECUTION MODEL
//! ---------------------------------------------------------------------------
//!
//! Passes execute in the order in which they are added.
//!
//! No implicit sorting, parallel execution, dependency reordering, or
//! hardware-specific scheduling is performed.
//!
//! The underlying pass manager performs:
//!
//! - pass execution;
//! - cancellation checks;
//! - configured execution-budget checks;
//! - verification boundaries;
//! - required-analysis availability checks;
//! - analysis invalidation;
//! - execution reporting.
//!
//! A failed pass may have mutated the IR before returning its error.
//! Consequently, callers MUST NOT assume that IR is unchanged or valid
//! after an unsuccessful pipeline execution.
//!
//! The caller must discard the IR or verify it again before reuse.
//!
//! ---------------------------------------------------------------------------
//! SCALABILITY
//! ---------------------------------------------------------------------------
//!
//! No universal pass-count limit is introduced.
//!
//! The pipeline uses dynamically sized collections.
//!
//! Actual resource consumption is governed by:
//!
//! - available memory;
//! - available processing capacity;
//! - configured execution budgets;
//! - the underlying IR and pass implementations;
//! - actual platform and representation limits.
//!
//! No hardware inventory or computational-domain enumeration is introduced.
//!
//! ---------------------------------------------------------------------------
//! DETERMINISM
//! ---------------------------------------------------------------------------
//!
//! The pipeline preserves explicit pass order.
//!
//! When the manager configuration requires deterministic passes, this
//! module rejects descriptors that do not declare deterministic behavior.
//!
//! A determinism declaration is a contract, not a proof that arbitrary
//! pass implementation code is deterministic.
//!
//! ---------------------------------------------------------------------------
//! ANALYSIS CONTRACT
//! ---------------------------------------------------------------------------
//!
//! Required analyses are checked by the underlying manager against the
//! analysis state available at execution time.
//!
//! This module does not pretend that a pass descriptor's invalidation list
//! describes analyses produced by that pass.
//!
//! Analysis providers must mark analyses available only after successfully
//! computing and storing valid results for the current IR state.
//!
//! ---------------------------------------------------------------------------
//! INTEGRATION
//! ---------------------------------------------------------------------------
//!
//! Compiler facade:
//!     crate::compiler::mod
//!
//! Compiler pass manager:
//!     crate::compiler::passes::manager
//!
//! Compiler pass registry:
//!     crate::compiler::passes::registry
//!
//! Canonical ZUIR pass manager:
//!     crate::zuir::pass::manager
//!
//! Canonical ZUIR pass registry:
//!     crate::zuir::pass::registry
//!
//! Canonical pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Canonical IR:
//!     crate::zuir
//!
//! Domain IR ownership remains with the corresponding repository modules.
//!
//! ---------------------------------------------------------------------------
//! COMPATIBILITY
//! ---------------------------------------------------------------------------
//!
//! The existing operational manager uses legacy pass types. This module
//! intentionally uses the explicitly named Operational* exports from the
//! compiler manager facade.
//!
//! It does not confuse those operational types with the canonical types
//! exported by contracts.rs.
//!
//! ---------------------------------------------------------------------------
//! SAFETY
//! ---------------------------------------------------------------------------
//!
//! Unsafe Rust is forbidden.

use std::collections::BTreeSet;
use std::sync::Arc;

use super::manager::{
    CancellationToken,
    IrVerifier,
    OperationalPass,
    OperationalPassDescriptor,
    OperationalPassError,
    OperationalPassErrorKind,
    OperationalPassId,
    OperationalPassOutcome,
    OperationalPassVersion,
    PassManager,
    PassManagerConfig,
    PipelineReport,
};

// ---------------------------------------------------------------------------
// Public pipeline types
// ---------------------------------------------------------------------------

/// A validated snapshot of a pass's declared pipeline metadata.
///
/// This is an operational descriptor snapshot from the current pass-manager
/// API. It is not a second canonical pass descriptor definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStep {
    descriptor: OperationalPassDescriptor,
}

impl PipelineStep {
    /// Return the pass identity.
    #[must_use]
    pub fn id(&self) -> &OperationalPassId {
        &self.descriptor.id
    }

    /// Return the pass version.
    #[must_use]
    pub fn version(&self) -> OperationalPassVersion {
        self.descriptor.version
    }

    /// Return the pass descriptor snapshot.
    #[must_use]
    pub fn descriptor(&self) -> &OperationalPassDescriptor {
        &self.descriptor
    }

    /// Return the human-readable pass description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.descriptor.description
    }
}

/// Result of validating a pipeline before execution.
///
/// This reports declarations and structural properties. It does not prove
/// that a pass implementation is semantically correct or deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineValidationReport {
    /// Number of registered pipeline steps.
    pub pass_count: usize,

    /// Passes that do not declare deterministic behavior.
    ///
    /// Their order is the pipeline execution order.
    pub nondeterministic_passes: Vec<OperationalPassId>,
}

impl PipelineValidationReport {
    /// Whether all passes declare deterministic behavior.
    #[must_use]
    pub fn all_passes_declare_determinism(&self) -> bool {
        self.nondeterministic_passes.is_empty()
    }

    /// Number of passes that do not declare deterministic behavior.
    #[must_use]
    pub fn nondeterministic_count(&self) -> usize {
        self.nondeterministic_passes.len()
    }
}

// ---------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------

/// Ordered, validated compiler pass pipeline.
///
/// The pipeline owns the operational pass manager and delegates all
/// execution to that manager.
///
/// # IR independence
///
/// `I` is the IR representation accepted by the registered passes.
/// This type does not impose a particular IR representation.
///
/// # Pass ownership
///
/// Pass implementations are moved into the underlying manager when
/// registered. The pipeline retains immutable descriptor snapshots for
/// inspection and preflight validation.
///
/// # Ordering
///
/// The order of `add_pass` calls is the execution order.
///
/// The pipeline does not sort passes by name, domain, target, or version.
pub struct PassPipeline<I> {
    manager: PassManager<I>,
    config: PassManagerConfig,

    steps: Vec<PipelineStep>,
    registered_ids: BTreeSet<OperationalPassId>,

    verifier: Option<Arc<dyn IrVerifier<I>>>,
    cancellation: CancellationToken,
    initial_analyses: BTreeSet<String>,
}

impl<I> PassPipeline<I> {
    /// Construct an empty pipeline.
    ///
    /// The supplied manager configuration governs verification,
    /// determinism, cancellation-related execution behavior, and budgets.
    #[must_use]
    pub fn new(config: PassManagerConfig) -> Self {
        let cancellation = CancellationToken::new();

        let mut pipeline = Self {
            manager: PassManager::new(config.clone()),
            config,
            steps: Vec::new(),
            registered_ids: BTreeSet::new(),
            verifier: None,
            cancellation,
            initial_analyses: BTreeSet::new(),
        };

        pipeline.rebuild_manager();

        pipeline
    }

    /// Construct an empty pipeline with default manager configuration.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(PassManagerConfig::default())
    }

    /// Number of registered pipeline steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether the pipeline contains no passes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Return the immutable pipeline step snapshots in execution order.
    pub fn steps(&self) -> impl ExactSizeIterator<Item = &PipelineStep> {
        self.steps.iter()
    }

    /// Return the manager configuration.
    #[must_use]
    pub fn config(&self) -> &PassManagerConfig {
        &self.config
    }

    /// Return the cancellation token used by this pipeline.
    ///
    /// The token can be cloned by a caller and cancelled from another
    /// thread. Pass implementations must cooperate with cancellation.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Set the IR verifier before registering any passes.
    ///
    /// Configuration is intentionally frozen once passes are registered.
    /// This avoids rebuilding a manager and accidentally losing stateful
    /// pass implementations.
    pub fn set_verifier(
        &mut self,
        verifier: Arc<dyn IrVerifier<I>>,
    ) -> Result<(), OperationalPassError> {
        self.ensure_unconfigured()?;

        self.verifier = Some(verifier);
        self.rebuild_manager();

        Ok(())
    }

    /// Set the initial analysis availability before registering passes.
    ///
    /// The caller is responsible for ensuring that every declared initial
    /// analysis has actually been computed and stored for this exact IR.
    pub fn set_initial_analyses<S>(
        &mut self,
        analyses: impl IntoIterator<Item = S>,
    ) -> Result<(), OperationalPassError>
    where
        S: Into<String>,
    {
        self.ensure_unconfigured()?;

        let mut validated = BTreeSet::new();

        for analysis in analyses {
            let analysis = analysis.into();

            validate_analysis_id(&analysis)?;

            validated.insert(analysis);
        }

        self.initial_analyses = validated;
        self.rebuild_manager();

        Ok(())
    }

    /// Replace the cancellation token before registering any passes.
    pub fn set_cancellation_token(
        &mut self,
        cancellation: CancellationToken,
    ) -> Result<(), OperationalPassError> {
        self.ensure_unconfigured()?;

        self.cancellation = cancellation;
        self.rebuild_manager();

        Ok(())
    }

    /// Register a pass at the end of the pipeline.
    ///
    /// The descriptor is validated before the manager is mutated.
    ///
    /// A duplicate pass ID is rejected without replacing an existing pass.
    ///
    /// Registration order defines execution order.
    pub fn add_pass<P>(
        &mut self,
        pass: P,
    ) -> Result<(), OperationalPassError>
    where
        P: OperationalPass<I> + 'static,
    {
        let descriptor = pass.descriptor();

        validate_descriptor(&descriptor)?;

        if self.registered_ids.contains(&descriptor.id) {
            return Err(OperationalPassError::for_pass(
                OperationalPassErrorKind::DuplicatePass,
                descriptor.id,
                "the pipeline already contains a pass with this identity",
            ));
        }

        if self.config.deterministic_mode
            && self.config.reject_nondeterministic_passes
            && !descriptor.deterministic
        {
            return Err(OperationalPassError::for_pass(
                OperationalPassErrorKind::NondeterministicPassRejected,
                descriptor.id,
                "the pipeline configuration rejects passes that do not \
                 declare deterministic behavior",
            ));
        }

        // Reserve before transferring ownership to the manager. This keeps
        // the pipeline's descriptor collection from needing to grow after
        // successful manager registration.
        self.steps.try_reserve(1).map_err(|error| {
            OperationalPassError::for_pass(
                OperationalPassErrorKind::InvalidConfiguration,
                descriptor.id.clone(),
                format!("unable to reserve pipeline step capacity: {error}"),
            )
        })?;

        // The canonical operational manager remains the execution owner.
        // Its own registration checks remain authoritative.
        self.manager.add_pass(pass)?;

        self.registered_ids.insert(descriptor.id.clone());

        self.steps.push(PipelineStep { descriptor });

        Ok(())
    }

    /// Validate the pipeline's registered descriptor snapshots.
    ///
    /// This performs preflight structural and policy validation. Required
    /// analysis availability is checked by the manager at execution time,
    /// because analysis providers may establish availability dynamically.
    pub fn validate(&self) -> Result<PipelineValidationReport, OperationalPassError> {
        if self.steps.len() != self.registered_ids.len() {
            return Err(OperationalPassError::new(
                OperationalPassErrorKind::InvalidConfiguration,
                "pipeline step and pass identity counts differ",
            ));
        }

        let mut observed_ids = BTreeSet::new();
        let mut nondeterministic_passes = Vec::new();

        for step in &self.steps {
            let descriptor = &step.descriptor;

            validate_descriptor(descriptor)?;

            if !observed_ids.insert(descriptor.id.clone()) {
                return Err(OperationalPassError::for_pass(
                    OperationalPassErrorKind::DuplicatePass,
                    descriptor.id.clone(),
                    "pipeline contains a duplicate pass identity",
                ));
            }

            if self.config.deterministic_mode
                && self.config.reject_nondeterministic_passes
                && !descriptor.deterministic
            {
                return Err(OperationalPassError::for_pass(
                    OperationalPassErrorKind::NondeterministicPassRejected,
                    descriptor.id.clone(),
                    "pipeline contains a pass rejected by the active \
                     determinism policy",
                ));
            }

            if !descriptor.deterministic {
                nondeterministic_passes.push(descriptor.id.clone());
            }
        }

        if observed_ids != self.registered_ids {
            return Err(OperationalPassError::new(
                OperationalPassErrorKind::InvalidConfiguration,
                "pipeline pass identity index is inconsistent",
            ));
        }

        Ok(PipelineValidationReport {
            pass_count: self.steps.len(),
            nondeterministic_passes,
        })
    }

    /// Execute the pipeline against the supplied IR.
    ///
    /// Execution is delegated to the canonical ZUIR pass manager.
    ///
    /// # Failure behavior
    ///
    /// On failure, the IR MUST NOT be assumed to be unchanged or valid.
    /// A pass may have mutated it before returning an error.
    ///
    /// The caller must discard the IR or verify it again before reuse.
    ///
    /// # Verification
    ///
    /// The manager's verification policy is authoritative. If verification
    /// is required and no verifier is configured, execution fails closed.
    pub fn run(
        &mut self,
        ir: &mut I,
    ) -> Result<PipelineReport, OperationalPassError> {
        self.validate()?;

        self.manager.run(ir)
    }

    /// Return the configured initial analyses.
    ///
    /// This reports only the initial availability declaration. It does not
    /// expose or manufacture analysis results.
    #[must_use]
    pub fn initial_analyses(&self) -> &BTreeSet<String> {
        &self.initial_analyses
    }

    // -----------------------------------------------------------------------
    // Internal lifecycle helpers
    // -----------------------------------------------------------------------

    fn ensure_unconfigured(&self) -> Result<(), OperationalPassError> {
        if !self.steps.is_empty() {
            return Err(OperationalPassError::new(
                OperationalPassErrorKind::InvalidConfiguration,
                "pipeline execution configuration cannot be changed after \
                 pass registration has started",
            ));
        }

        Ok(())
    }

    fn rebuild_manager(&mut self) {
        let mut manager = PassManager::new(self.config.clone());

        if let Some(verifier) = &self.verifier {
            manager = manager.with_verifier(Arc::clone(verifier));
        }

        manager = manager.with_cancellation(self.cancellation.clone());

        manager = manager.with_initial_analyses(
            self.initial_analyses.iter().cloned(),
        );

        self.manager = manager;
    }
}

// ---------------------------------------------------------------------------
// Descriptor validation
// ---------------------------------------------------------------------------

/// Validate an operational pass descriptor before pipeline registration.
///
/// The canonical descriptor validator is used by the canonical contract
/// subsystem. The operational manager still exposes a separate legacy
/// descriptor type, so this adapter validates the fields available in that
/// existing operational contract.
///
/// This function does not prove semantic preservation.
fn validate_descriptor(
    descriptor: &OperationalPassDescriptor,
) -> Result<(), OperationalPassError> {
    if descriptor.description.trim().is_empty() {
        return Err(OperationalPassError::for_pass(
            OperationalPassErrorKind::InvalidConfiguration,
            descriptor.id.clone(),
            "pass description must not be empty",
        ));
    }

    if descriptor.preserves_all_analyses
        && !descriptor.invalidated_analyses.is_empty()
    {
        return Err(OperationalPassError::for_pass(
            OperationalPassErrorKind::InvalidConfiguration,
            descriptor.id.clone(),
            "a pass cannot preserve all analyses while declaring \
             analyses invalidated",
        ));
    }

    for analysis in descriptor
        .required_analyses
        .iter()
        .chain(descriptor.invalidated_analyses.iter())
    {
        validate_analysis_id(analysis).map_err(|error| {
            error.with_pass_if_missing(&descriptor.id)
        })?;
    }

    Ok(())
}

/// Validate an extensible analysis identifier.
///
/// Identifiers are namespaced by convention but are not restricted to a
/// closed list of computational domains.
fn validate_analysis_id(
    analysis: &str,
) -> Result<(), OperationalPassError> {
    if analysis.is_empty()
        || analysis.trim() != analysis
        || analysis
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
    {
        return Err(OperationalPassError::new(
            OperationalPassErrorKind::InvalidConfiguration,
            format!("invalid analysis identifier: {analysis:?}"),
        ));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct IdentityVerifier;

    impl IrVerifier<u64> for IdentityVerifier {
        fn verify(&self, _ir: &u64) -> Result<(), OperationalPassError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct AddPass {
        descriptor: OperationalPassDescriptor,
        amount: u64,
    }

    impl OperationalPass<u64> for AddPass {
        fn descriptor(&self) -> OperationalPassDescriptor {
            self.descriptor.clone()
        }

        fn run(
            &mut self,
            ir: &mut u64,
            context: &mut super::super::manager::PassContext,
        ) -> Result<OperationalPassOutcome, OperationalPassError> {
            context.check_budget()?;

            *ir = ir.checked_add(self.amount).ok_or_else(|| {
                OperationalPassError::new(
                    OperationalPassErrorKind::PassFailed,
                    "test integer overflow",
                )
            })?;

            Ok(OperationalPassOutcome::changed())
        }
    }

    fn add_pass(name: &str, amount: u64) -> AddPass {
        AddPass {
            descriptor: OperationalPassDescriptor::new(
                OperationalPassId::new(name)
                    .expect("test pass ID must be valid"),
                OperationalPassVersion::new(1, 0, 0),
                "Test arithmetic pass",
            ),
            amount,
        }
    }

    fn verified_pipeline() -> PassPipeline<u64> {
        let mut pipeline = PassPipeline::with_defaults();

        pipeline
            .set_verifier(Arc::new(IdentityVerifier))
            .expect("verifier configuration should succeed");

        pipeline
    }

    #[test]
    fn pipeline_starts_empty() {
        let pipeline = PassPipeline::<u64>::with_defaults();

        assert!(pipeline.is_empty());
        assert_eq!(pipeline.len(), 0);
    }

    #[test]
    fn preserves_registration_order() {
        let mut pipeline = verified_pipeline();

        pipeline.add_pass(add_pass("test.add_two", 2)).unwrap();
        pipeline.add_pass(add_pass("test.add_three", 3)).unwrap();

        let mut value = 10_u64;

        let report = pipeline.run(&mut value).unwrap();

        assert_eq!(value, 15);
        assert_eq!(report.executed_passes(), 2);
        assert!(report.changed);
    }

    #[test]
    fn rejects_duplicate_pass_identity() {
        let mut pipeline = verified_pipeline();

        pipeline.add_pass(add_pass("test.duplicate", 1)).unwrap();

        let result = pipeline.add_pass(add_pass("test.duplicate", 2));

        assert!(result.is_err());
        assert_eq!(pipeline.len(), 1);
    }

    #[test]
    fn rejects_invalid_analysis_identifiers() {
        let mut pipeline = verified_pipeline();

        let mut pass = add_pass("test.invalid_analysis", 1);

        pass.descriptor
            .required_analyses
            .insert("invalid analysis name".to_owned());

        assert!(pipeline.add_pass(pass).is_err());
        assert!(pipeline.is_empty());
    }

    #[test]
    fn rejects_contradictory_analysis_declarations() {
        let mut pipeline = verified_pipeline();

        let mut pass = add_pass("test.contradictory", 1);

        pass.descriptor.preserves_all_analyses = true;
        pass.descriptor
            .invalidated_analyses
            .insert("core.cfg".to_owned());

        assert!(pipeline.add_pass(pass).is_err());
        assert!(pipeline.is_empty());
    }

    #[test]
    fn reports_nondeterministic_passes_when_policy_allows_them() {
        let config = PassManagerConfig {
            deterministic_mode: false,
            reject_nondeterministic_passes: false,
            ..PassManagerConfig::default()
        };

        let mut pipeline = PassPipeline::<u64>::new(config);
        let mut pass = add_pass("test.nondeterministic", 1);
        pass.descriptor.deterministic = false;

        pipeline.add_pass(pass).unwrap();

        let validation = pipeline.validate().unwrap();

        assert_eq!(validation.pass_count, 1);
        assert_eq!(validation.nondeterministic_count(), 1);
    }

    #[test]
    fn strict_determinism_policy_rejects_nondeterministic_passes() {
        let mut pipeline = verified_pipeline();

        let mut pass = add_pass("test.nondeterministic", 1);
        pass.descriptor.deterministic = false;

        assert!(pipeline.add_pass(pass).is_err());
        assert!(pipeline.is_empty());
    }

    #[test]
    fn execution_fails_closed_when_verification_is_required() {
        let mut pipeline = PassPipeline::<u64>::with_defaults();

        pipeline.add_pass(add_pass("test.needs_verifier", 1)).unwrap();

        let mut value = 0_u64;

        let error = pipeline
            .run(&mut value)
            .expect_err("missing verifier must fail closed");

        assert_eq!(
            error.kind,
            OperationalPassErrorKind::VerificationFailed
        );
    }

    #[test]
    fn configuration_cannot_change_after_registration() {
        let mut pipeline = verified_pipeline();

        pipeline.add_pass(add_pass("test.configured", 1)).unwrap();

        let result = pipeline.set_initial_analyses(["core.cfg"]);

        assert!(result.is_err());
    }

    #[test]
    fn cancellation_token_is_shared_with_pipeline_manager() {
        let mut pipeline = verified_pipeline();

        let token = pipeline.cancellation_token();

        assert!(!token.is_cancelled());

        token.cancel();

        assert!(token.is_cancelled());

        let mut value = 0_u64;

        let error = pipeline
            .run(&mut value)
            .expect_err("cancelled execution must fail");

        assert_eq!(error.kind, OperationalPassErrorKind::Cancelled);
    }

    #[test]
    fn validation_reports_empty_pipeline() {
        let pipeline = PassPipeline::<u64>::with_defaults();

        let report = pipeline.validate().unwrap();

        assert_eq!(report.pass_count, 0);
        assert!(report.all_passes_declare_determinism());
    }
}
