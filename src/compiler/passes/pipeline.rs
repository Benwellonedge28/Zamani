
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Pipeline
//!
//! Path: `src/compiler/passes/pipeline.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Compose and validate an explicitly ordered sequence of compiler passes.
//! Pass execution is delegated to the existing ZUIR pass manager.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Ordered pipeline composition.
//! - Immutable snapshots of registered pass descriptors.
//! - Pipeline-level structural validation.
//! - Pipeline-level determinism reporting.
//! - The compiler-facing pipeline execution API.
//!
//! This module does not own:
//! - Pass identities or canonical pass contracts.
//! - Pass implementations or transformation algorithms.
//! - Pass execution or analysis invalidation.
//! - Analysis computation or analysis-result storage.
//! - IR definitions or verification algorithms.
//! - Semantic analysis or semantic-preservation proofs.
//! - Optimization algorithms.
//! - Hardware discovery, physical scheduling, or runtime execution.
//!
//! AUTHORITY
//!
//! The operational pass manager remains:
//! `crate::zuir::pass::manager`
//!
//! The compiler-facing compatibility exports remain:
//! `crate::compiler::passes::manager`
//!
//! Canonical pass contracts remain:
//! `crate::zuir::pass::contracts`
//!
//! The current ZUIR manager still exposes legacy operational pass types.
//! This module deliberately uses the explicitly named Operational* aliases
//! until the manager and registry have completed their canonical migration.
//!
//! EXECUTION ORDER
//!
//! Passes execute in registration order. This module does not sort passes,
//! infer dependencies, reorder passes, or execute them concurrently.
//!
//! Required analyses are checked by the underlying manager at execution
//! time. Analysis results must be computed by their authoritative providers.
//!
//! FAILURE CONTRACT
//!
//! A pass may mutate IR before returning an error. Consequently, after a
//! failed execution, callers must not assume that the IR is unchanged or
//! valid. They must discard it or run the appropriate verifier before reuse.
//!
//! SCALABILITY
//!
//! No arbitrary pass-count, IR-size, hardware-count, or domain-count limit
//! is introduced. Dynamic collections grow according to available resources.
//! An absent configured budget does not promise infinite resources.
//!
//! DETERMINISM
//!
//! Registration order is preserved. Determinism declarations are checked
//! against the manager configuration. A declaration is a pass contract,
//! not proof that arbitrary implementation code is deterministic.
//!
//! SAFETY
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

// -----------------------------------------------------------------------------
// Registered pass descriptor snapshots
// -----------------------------------------------------------------------------

/// Immutable metadata captured when a pass is registered.
///
/// The snapshot is intentionally independent of the pass implementation.
/// The pipeline does not allow a registered pass to change its identity,
/// version, analysis declarations, or determinism declaration afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStep {
    descriptor: OperationalPassDescriptor,
}

impl PipelineStep {
    /// Return the registered pass identity.
    #[must_use]
    pub fn id(&self) -> &OperationalPassId {
        &self.descriptor.id
    }

    /// Return the registered pass version.
    #[must_use]
    pub fn version(&self) -> OperationalPassVersion {
        self.descriptor.version
    }

    /// Return the immutable descriptor snapshot.
    #[must_use]
    pub fn descriptor(&self) -> &OperationalPassDescriptor {
        &self.descriptor
    }

    /// Return the registered pass description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.descriptor.description
    }
}

// -----------------------------------------------------------------------------
// Pipeline validation report
// -----------------------------------------------------------------------------

/// Structural and policy assessment of a pipeline.
///
/// This report validates declarations. It does not prove that pass
/// implementations preserve semantics or are actually deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineValidationReport {
    /// Number of registered passes.
    pub pass_count: usize,

    /// Nondeterministic pass identities in execution order.
    pub nondeterministic_passes: Vec<OperationalPassId>,
}

impl PipelineValidationReport {
    /// Whether every registered pass declares deterministic behavior.
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

// -----------------------------------------------------------------------------
// Frozen pass adapter
// -----------------------------------------------------------------------------

/// Wrap a pass with the descriptor captured at registration.
///
/// This adapter prevents a pass implementation from returning different
/// descriptors at different points in its lifetime.
///
/// The underlying implementation remains responsible for executing the pass.
/// This wrapper does not alter its IR behavior or analysis results.
struct FrozenPass<P> {
    descriptor: OperationalPassDescriptor,
    implementation: P,
}

impl<P> FrozenPass<P> {
    fn new(
        descriptor: OperationalPassDescriptor,
        implementation: P,
    ) -> Self {
        Self {
            descriptor,
            implementation,
        }
    }
}

impl<I, P> OperationalPass<I> for FrozenPass<P>
where
    P: OperationalPass<I>,
{
    fn descriptor(&self) -> OperationalPassDescriptor {
        self.descriptor.clone()
    }

    fn run(
        &mut self,
        ir: &mut I,
        context: &mut super::manager::PassContext,
    ) -> Result<OperationalPassOutcome, OperationalPassError> {
        self.implementation.run(ir, context)
    }
}

// -----------------------------------------------------------------------------
// Pipeline
// -----------------------------------------------------------------------------

/// Ordered, validated compiler pass pipeline.
///
/// `I` is the IR representation accepted by the registered passes. The
/// pipeline is representation-independent and does not define another IR.
///
/// Pass registration order is execution order.
///
/// The underlying ZUIR manager owns execution, verification, cancellation,
/// budget enforcement, required-analysis checks, and invalidation.
///
/// # Configuration lifecycle
///
/// The verifier, initial analysis declarations, and cancellation token must
/// be configured before the first pass is registered. This prevents a
/// configuration change from silently rebuilding the manager and losing
/// stateful pass implementations.
///
/// # Failure behavior
///
/// If `run` returns an error, the caller must treat the IR as potentially
/// modified and invalid. The manager does not promise transactional rollback.
pub struct PassPipeline<I> {
    manager: PassManager<I>,
    config: PassManagerConfig,

    /// Descriptor snapshots in registration/execution order.
    steps: Vec<PipelineStep>,

    /// Verifier shared with the underlying manager.
    verifier: Option<Arc<dyn IrVerifier<I>>>,

    /// Cooperative cancellation shared with the underlying manager.
    cancellation: CancellationToken,

    /// Analyses already computed for the initial IR.
    ///
    /// These are declarations of availability, not analysis results.
    initial_analyses: BTreeSet<String>,
}

impl<I> PassPipeline<I> {
    /// Create an empty pipeline with explicit manager configuration.
    #[must_use]
    pub fn new(config: PassManagerConfig) -> Self {
        let cancellation = CancellationToken::new();

        let manager = PassManager::new(config.clone())
            .with_cancellation(cancellation.clone());

        Self {
            manager,
            config,
            steps: Vec::new(),
            verifier: None,
            cancellation,
            initial_analyses: BTreeSet::new(),
        }
    }

    /// Create an empty pipeline with the existing manager defaults.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(PassManagerConfig::default())
    }

    /// Return the number of registered passes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Return whether the pipeline contains no passes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Iterate over descriptor snapshots in execution order.
    pub fn steps(&self) -> impl ExactSizeIterator<Item = &PipelineStep> {
        self.steps.iter()
    }

    /// Return the manager configuration.
    #[must_use]
    pub fn config(&self) -> &PassManagerConfig {
        &self.config
    }

    /// Return a clone of the cooperative cancellation token.
    ///
    /// The caller may cancel a running pipeline from another thread.
    /// Pass implementations must cooperate with cancellation.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Configure the verifier before registering any passes.
    ///
    /// The verifier is responsible for the actual IR validation rules.
    /// The pipeline only forwards it to the ZUIR manager.
    pub fn set_verifier(
        &mut self,
        verifier: Arc<dyn IrVerifier<I>>,
    ) -> Result<(), OperationalPassError> {
        self.ensure_unconfigured()?;

        self.verifier = Some(verifier);
        self.rebuild_manager();

        Ok(())
    }

    /// Declare analyses already computed for the initial IR.
    ///
    /// The caller must only provide an analysis identifier when a trusted
    /// analysis provider has actually computed and stored a valid result for
    /// the exact IR state that will be passed to `run`.
    ///
    /// This method does not compute analyses or create analysis results.
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

    /// Replace the cancellation token before pass registration.
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
    /// The pass descriptor is captured and validated before ownership is
    /// transferred to the underlying manager.
    ///
    /// The captured descriptor is frozen by `FrozenPass`.
    ///
    /// Duplicate pass identities are rejected by the existing manager.
    /// Registration order determines execution order.
    pub fn add_pass<P>(
        &mut self,
        pass: P,
    ) -> Result<(), OperationalPassError>
    where
        P: OperationalPass<I> + 'static,
    {
        let descriptor = pass.descriptor();

        validate_descriptor(&descriptor)?;

        if self.steps.iter().any(|step| step.id() == &descriptor.id) {
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
                "the active pipeline policy rejects passes that do not \
                 declare deterministic behavior",
            ));
        }

        // Reserve the snapshot slot before registering with the manager.
        // Once manager registration succeeds, pushing the snapshot does not
        // require another Vec capacity allocation.
        self.steps.try_reserve(1).map_err(|error| {
            OperationalPassError::for_pass(
                OperationalPassErrorKind::InvalidConfiguration,
                descriptor.id.clone(),
                format!(
                    "unable to reserve pipeline descriptor storage: {error}"
                ),
            )
        })?;

        let frozen_pass = FrozenPass::new(descriptor.clone(), pass);

        // The ZUIR manager remains the single owner of pass execution.
        self.manager.add_pass(frozen_pass)?;

        self.steps.push(PipelineStep { descriptor });

        Ok(())
    }

    /// Validate registered pass descriptors and pipeline bookkeeping.
    ///
    /// This is a structural preflight check. Required-analysis availability
    /// is checked by the manager against the actual analysis state when the
    /// pipeline executes.
    pub fn validate(
        &self,
    ) -> Result<PipelineValidationReport, OperationalPassError> {
        if self.manager.pass_count() != self.steps.len() {
            return Err(OperationalPassError::new(
                OperationalPassErrorKind::InvalidConfiguration,
                "pipeline descriptor count does not match the underlying \
                 pass-manager registration count",
            ));
        }

        let mut observed_ids = BTreeSet::new();
        let mut nondeterministic_passes = Vec::new();

        for step in &self.steps {
            let descriptor = step.descriptor();

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

        Ok(PipelineValidationReport {
            pass_count: self.steps.len(),
            nondeterministic_passes,
        })
    }

    /// Execute the registered passes in registration order.
    ///
    /// Execution is delegated entirely to the existing ZUIR pass manager.
    ///
    /// The manager performs:
    /// - cancellation checks;
    /// - configured execution-budget checks;
    /// - required-analysis availability checks;
    /// - configured IR verification;
    /// - analysis invalidation;
    /// - execution reporting.
    ///
    /// # Important
    ///
    /// On error, the IR may have been partially mutated by a pass. The caller
    /// must discard it or verify it again before reuse.
    pub fn run(
        &mut self,
        ir: &mut I,
    ) -> Result<PipelineReport, OperationalPassError> {
        self.validate()?;

        self.manager.run(ir)
    }

    /// Return the identifiers declared available for the initial IR.
    ///
    /// This is only an availability declaration. It is not evidence that
    /// the corresponding analysis result exists or is valid.
    #[must_use]
    pub fn initial_analyses(&self) -> &BTreeSet<String> {
        &self.initial_analyses
    }

    // -------------------------------------------------------------------------
    // Internal lifecycle
    // -------------------------------------------------------------------------

    fn ensure_unconfigured(
        &self,
    ) -> Result<(), OperationalPassError> {
        if !self.steps.is_empty() {
            return Err(OperationalPassError::new(
                OperationalPassErrorKind::InvalidConfiguration,
                "pipeline configuration cannot be changed after pass \
                 registration has started",
            ));
        }

        Ok(())
    }

    /// Rebuild the empty manager after a pre-registration configuration
    /// change. This function is only called while no pass has been registered.
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

// -----------------------------------------------------------------------------
// Descriptor and analysis identifier validation
// -----------------------------------------------------------------------------

/// Validate the legacy operational descriptor exposed by the existing
/// ZUIR manager.
///
/// The canonical `contracts.rs` descriptor validator remains authoritative
/// for canonical descriptors. This adapter validates the fields present in
/// the operational descriptor currently accepted by the manager.
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
            "a pass cannot preserve all analyses while also declaring \
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
/// Analysis names are not restricted to a closed list of domains.
/// Namespaced names are recommended, for example:
///
/// - `core.control_flow`
/// - `classical.alias_analysis`
/// - `quantum.logical_liveness`
/// - `hdl.timing`
/// - `tensor.shape`
/// - `distributed.consistency`
///
/// This function intentionally imposes no arbitrary length limit.
fn validate_analysis_id(
    analysis: &str,
) -> Result<(), OperationalPassError> {
    if analysis.is_empty()
        || analysis.trim() != analysis
        || analysis
            .chars()
            .any(|character| {
                character.is_whitespace() || character.is_control()
            })
    {
        return Err(OperationalPassError::new(
            OperationalPassErrorKind::InvalidConfiguration,
            format!("invalid analysis identifier: {analysis:?}"),
        ));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct IdentityVerifier;

    impl IrVerifier<u64> for IdentityVerifier {
        fn verify(
            &self,
            _ir: &u64,
        ) -> Result<(), OperationalPassError> {
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
                    "test arithmetic overflow",
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
        assert_eq!(pipeline.manager.pass_count(), 1);
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
    fn reports_nondeterministic_passes_when_allowed() {
        let config = PassManagerConfig {
            deterministic_mode: false,
            reject_nondeterministic_passes: false,
            ..PassManagerConfig::default()
        };

        let mut pipeline = PassPipeline::<u64>::new(config);
        let mut pass = add_pass("test.nondeterministic", 1);

        pass.descriptor.deterministic = false;
        pipeline.add_pass(pass).unwrap();

        let report = pipeline.validate().unwrap();

        assert_eq!(report.pass_count, 1);
        assert_eq!(report.nondeterministic_count(), 1);
    }

    #[test]
    fn rejects_nondeterministic_passes_under_strict_policy() {
        let mut pipeline = verified_pipeline();
        let mut pass = add_pass("test.nondeterministic", 1);

        pass.descriptor.deterministic = false;

        assert!(pipeline.add_pass(pass).is_err());
        assert!(pipeline.is_empty());
    }

    #[test]
    fn execution_fails_closed_when_required_verifier_is_missing() {
        let mut pipeline = PassPipeline::<u64>::with_defaults();

        pipeline
            .add_pass(add_pass("test.requires_verification", 1))
            .unwrap();

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

        assert!(pipeline.set_initial_analyses(["core.cfg"]).is_err());
    }

    #[test]
    fn cancellation_token_is_shared_with_manager() {
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
    fn validates_empty_pipeline() {
        let pipeline = PassPipeline::<u64>::with_defaults();
        let report = pipeline.validate().unwrap();

        assert_eq!(report.pass_count, 0);
        assert!(report.all_passes_declare_determinism());
    }

    #[test]
    fn freezes_descriptor_identity_after_registration() {
        struct ChangingDescriptorPass {
            first: OperationalPassDescriptor,
            second: OperationalPassDescriptor,
        }

        impl OperationalPass<u64> for ChangingDescriptorPass {
            fn descriptor(&self) -> OperationalPassDescriptor {
                self.first.clone()
            }

            fn run(
                &mut self,
                _ir: &mut u64,
                _context: &mut super::super::manager::PassContext,
            ) -> Result<OperationalPassOutcome, OperationalPassError> {
                // The implementation's own descriptor state is irrelevant
                // after registration; FrozenPass returns the captured one.
                self.first = self.second.clone();
                Ok(OperationalPassOutcome::unchanged())
            }
        }

        let mut pipeline = verified_pipeline();

        let first = OperationalPassDescriptor::new(
            OperationalPassId::new("test.frozen").unwrap(),
            OperationalPassVersion::new(1, 0, 0),
            "Original descriptor",
        );

        let second = OperationalPassDescriptor::new(
            OperationalPassId::new("test.changed").unwrap(),
            OperationalPassVersion::new(9, 0, 0),
            "Changed descriptor",
        );

        pipeline
            .add_pass(ChangingDescriptorPass { first, second })
            .unwrap();

        let mut value = 0_u64;
        pipeline.run(&mut value).unwrap();

        let step = pipeline.steps().next().unwrap();

        assert_eq!(step.id().as_str(), "test.frozen");
        assert_eq!(step.version(), OperationalPassVersion::new(1, 0, 0));
    }
}
