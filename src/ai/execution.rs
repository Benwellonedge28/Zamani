
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Execution
//!
//! Path: src/ai/execution.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the backend-independent execution boundary for AI computation.
//!
//! This module accepts an already validated AI computation and delegates
//! its realization to an explicitly supplied execution dispatcher.
//!
//! The dispatcher integrates with the existing distributed, hybrid,
//! compiler, backend and runtime infrastructure.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! - grammar/spec/ai.md
//! - grammar/ai/README.md
//! - grammar/ai/ai.g4
//! - grammar/ai/inference.g4
//! - grammar/ai/training.g4
//! - grammar/execution/execution-context.g4
//! - src/ai/error.rs
//! - src/ai/config.rs
//! - src/ai/context.rs
//! - src/ai/requirements.rs
//! - src/ai/registry.rs
//! - src/ai/capabilities.rs
//! - src/zuir/
//! - src/distributed/execution/
//! - src/hybrid/
//! - src/runtime/
//!
//! ## OWNS
//!
//! - AI execution requests;
//! - AI execution workload contracts;
//! - AI execution preflight outcomes;
//! - AI execution dispatch contracts;
//! - AI execution receipts;
//! - AI execution result envelopes;
//! - execution-boundary validation;
//! - execution failure translation.
//!
//! ## DOES NOT OWN
//!
//! - Source syntax or AST construction;
//! - semantic analysis;
//! - type inference;
//! - canonical AI operation registration;
//! - capability discovery;
//! - capability evidence;
//! - resource feasibility implementation;
//! - physical resource allocation;
//! - hardware discovery;
//! - device selection;
//! - scheduling;
//! - routing;
//! - quantum error correction;
//! - tensor storage;
//! - numerical kernels;
//! - model training algorithms;
//! - model inference implementations;
//! - ZUIR or domain-specific IR definitions;
//! - distributed transport;
//! - runtime lifecycle;
//! - security enforcement;
//! - checkpoint persistence.
//!
//! ## EXECUTION ARCHITECTURE
//!
//! Validated AI computation
//!          |
//!          v
//!    AI execution request
//!          |
//!          v
//!    Request validation
//!          |
//!          v
//!    Exact operation resolution
//!          |
//!          v
//!    Execution preflight
//!          |
//!          +--> capability assessment
//!          +--> resource feasibility
//!          +--> authorization
//!          +--> compatibility
//!          |
//!          v
//!    Execution dispatcher
//!          |
//!          +--> local execution
//!          +--> classical execution
//!          +--> accelerator execution
//!          +--> quantum-assisted execution
//!          +--> hybrid execution
//!          +--> distributed execution
//!          +--> future execution domains
//!          |
//!          v
//!    Execution receipt
//!          |
//!          v
//!    Validated execution result
//!
//! ## POCO-REAF
//!
//! This module introduces no universal maximum for:
//!
//! - AI models;
//! - model parameters;
//! - tensor rank;
//! - tensor dimensions;
//! - tensor elements;
//! - training steps;
//! - inference operations;
//! - pipeline stages;
//! - agents;
//! - workers;
//! - accelerators;
//! - devices;
//! - distributed nodes;
//! - memory;
//! - execution duration.
//!
//! Explicit operational budgets remain enforceable by their owning
//! configuration, resource and runtime infrastructure.
//!
//! Unlimited configuration does not promise infinite physical resources.
//!
//! A target that cannot satisfy mandatory requirements must not silently
//! change the meaning of an AI program.
//!
//! ## DETERMINISM
//!
//! Request identity and operation version are explicit.
//!
//! The exact registered operation version is resolved.
//!
//! No implicit operation-version substitution is performed.
//!
//! Execution determinism is a property established by the actual
//! execution provider and declared numerical contract.
//!
//! ## CONCURRENCY
//!
//! No global mutable state is introduced.
//!
//! Execution dependencies are explicitly injected.
//!
//! Shared providers must implement their own synchronization guarantees.
//!
//! ## FAILURE SEMANTICS
//!
//! Unsupported, infeasible and indeterminate execution are distinct.
//!
//! An execution failure must never be converted into a successful result.
//!
//! Distributed execution with an unknown outcome must preserve that
//! uncertainty and must not be retried automatically.
//!
//! ## FREEZE CRITERIA
//!
//! - Exact AI operation resolution works.
//! - Incomplete requirements are rejected.
//! - Preflight is mandatory.
//! - Indeterminate eligibility cannot execute.
//! - Cancellation and deadlines propagate.
//! - Provider errors preserve their causes.
//! - Provenance requirements are enforced.
//! - No duplicate IR or backend authority exists.
//! - All execution paths are covered by integration tests.
//! - No unsafe Rust or artificial hardware ceilings exist.

use core::fmt;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ai::context::AiContext;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
    OperationOutcome,
    RetryDisposition,
};

use crate::ai::registry::AiOperationDescriptor;

use crate::ai::requirements::AiRequirements;

use crate::ai::version::AiOperationSetVersion;

use crate::frontend::semantic::domains::ai::QualifiedName;

use crate::distributed::execution::executor::ExecutionControl;

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the in-memory AI execution API.
///
/// This is not the Zamani language version, AI semantic version,
/// ZUIR version, artifact version or backend ABI version.
pub const AI_EXECUTION_API_VERSION: u16 = 1;

// ============================================================================
// EXECUTION IDENTIFIERS
// ============================================================================

/// Stable identity of an individual AI execution request.
///
/// The caller supplies this identifier so that request identity remains
/// consistent across logging, tracing, distributed dispatch and recovery.
///
/// This identifier does not itself guarantee exactly-once execution.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiExecutionId(String);

impl AiExecutionId {
    /// Construct a validated execution identifier.
    ///
    /// Identifiers are preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is empty or contains
    /// control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        if value.trim().is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(
                AiError::new(
                    AiErrorKind::InvalidInput,
                    "invalid AI execution identifier",
                )
                .with_phase(AiErrorPhase::Construction)
                .with_outcome(OperationOutcome::NotStarted),
            );
        }

        Ok(Self(value))
    }

    /// Return the exact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AiExecutionId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EXECUTABLE WORKLOAD CONTRACT
// ============================================================================

/// Contract implemented by an adapter over an authoritative,
/// validated AI computation.
///
/// The workload may reference:
///
/// - a canonical ZUIR module or operation;
/// - an existing AI model definition;
/// - a validated AI inference computation;
/// - a validated training computation;
/// - an AI reasoning computation;
/// - a neural-symbolic computation;
/// - an AI component of a hybrid graph.
///
/// This trait does not define another intermediate representation.
///
/// Implementations must retain the original canonical IR identities
/// and semantic provenance.
///
/// The workload is borrowed by the execution interface. Its storage
/// and lifetime remain under the ownership of its caller.
pub trait AiExecutable: Send + Sync {

    /// Return the canonical registered AI operation name.
    fn operation_name(&self) -> &QualifiedName;

    /// Return the exact operation-set version.
    ///
    /// No automatic substitution is permitted.
    fn operation_version(&self) -> AiOperationSetVersion;

    /// Return the canonical AI requirements associated with this work.
    ///
    /// These requirements must be complete before execution.
    fn requirements(&self) -> &AiRequirements;

    /// Return an optional opaque model identity.
    ///
    /// This must be an identity supplied by the authoritative model
    /// subsystem, not a newly generated execution-local model identity.
    fn model_reference(&self) -> Option<&str> {
        None
    }

    /// Validate workload invariants before execution.
    ///
    /// Implementations should delegate to their authoritative model,
    /// graph, ZUIR and semantic validation services.
    ///
    /// This method must not allocate hardware or execute computation.
    fn validate(&self) -> AiResult<()>;
}

// ============================================================================
// EXECUTION REQUEST
// ============================================================================

/// Immutable request to execute an AI computation.
///
/// The request contains computational intent and references to
/// established AI structures.
///
/// It does not contain physical device identifiers or backend handles.
pub struct AiExecutionRequest<'work, W: AiExecutable> {
    id: AiExecutionId,
    workload: &'work W,
}

impl<'work, W: AiExecutable> AiExecutionRequest<'work, W> {

    /// Construct an AI execution request.
    ///
    /// The request is not considered executable until the engine
    /// validates its workload and requirements.
    pub fn new(
        id: AiExecutionId,
        workload: &'work W,
    ) -> Self {
        Self {
            id,
            workload,
        }
    }

    /// Return the request identity.
    #[must_use]
    pub fn id(&self) -> &AiExecutionId {
        &self.id
    }

    /// Return the referenced workload.
    #[must_use]
    pub fn workload(&self) -> &'work W {
        self.workload
    }

    /// Return the requested operation.
    #[must_use]
    pub fn operation_name(&self) -> &QualifiedName {
        self.workload.operation_name()
    }

    /// Return the exact requested operation version.
    #[must_use]
    pub fn operation_version(&self) -> AiOperationSetVersion {
        self.workload.operation_version()
    }
}

// ============================================================================
// EXECUTION PREFLIGHT
// ============================================================================

/// Result of execution preflight.
///
/// Eligibility must be established before dispatch.
///
/// An indeterminate assessment is never equivalent to eligibility.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiExecutionAssessment {

    /// The execution infrastructure has established eligibility.
    Eligible {
        /// Explanation of the assessment.
        explanation: String,
    },

    /// Mandatory requirements cannot be satisfied.
    Ineligible {
        /// Explanation of the incompatibility.
        reason: String,
    },

    /// Available evidence is insufficient to establish eligibility.
    Indeterminate {
        /// Explanation of the missing evidence.
        reason: String,
    },
}

impl AiExecutionAssessment {

    /// Construct an eligible assessment.
    #[must_use]
    pub fn eligible(
        explanation: impl Into<String>,
    ) -> Self {
        Self::Eligible {
            explanation: explanation.into(),
        }
    }

    /// Construct an ineligible assessment.
    #[must_use]
    pub fn ineligible(
        reason: impl Into<String>,
    ) -> Self {
        Self::Ineligible {
            reason: reason.into(),
        }
    }

    /// Construct an indeterminate assessment.
    #[must_use]
    pub fn indeterminate(
        reason: impl Into<String>,
    ) -> Self {
        Self::Indeterminate {
            reason: reason.into(),
        }
    }

    /// Return whether eligibility has been established.
    #[must_use]
    pub const fn is_eligible(&self) -> bool {
        matches!(self, Self::Eligible { .. })
    }
}

// ============================================================================
// EXECUTION PROVENANCE
// ============================================================================

/// References to provenance generated by authoritative execution
/// and provenance infrastructure.
///
/// These references do not replace the canonical provenance graph.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AiExecutionProvenance {

    /// Reference to the originating canonical computation.
    pub computation_reference: Option<String>,

    /// Reference to the execution provenance record.
    pub execution_reference: Option<String>,

    /// Reference to the model lineage.
    pub model_reference: Option<String>,

    /// Reference to the data lineage.
    pub data_reference: Option<String>,
}

impl AiExecutionProvenance {

    /// Return whether an authoritative execution provenance reference
    /// is available.
    #[must_use]
    pub fn is_recorded(&self) -> bool {
        self.execution_reference
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
    }
}

// ============================================================================
// EXECUTION RECEIPT
// ============================================================================

/// Result returned by an execution dispatcher after actual execution.
///
/// The receipt describes an execution outcome. It is not proof of
/// semantic correctness beyond the guarantees established by the
/// authoritative verifier and provider.
pub struct AiExecutionReceipt<O> {

    /// Actual output returned by the execution provider.
    pub output: O,

    /// Provenance references supplied by the authoritative provider.
    pub provenance: AiExecutionProvenance,

    /// Optional opaque execution-realization reference.
    ///
    /// This may identify a distributed job, hybrid execution or
    /// runtime execution record.
    ///
    /// It must not be interpreted as a physical hardware identity.
    pub realization_reference: Option<String>,
}

impl<O> AiExecutionReceipt<O> {

    /// Construct an execution receipt.
    #[must_use]
    pub fn new(
        output: O,
        provenance: AiExecutionProvenance,
    ) -> Self {
        Self {
            output,
            provenance,
            realization_reference: None,
        }
    }

    /// Attach an opaque realization reference.
    #[must_use]
    pub fn with_realization_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Self {
        self.realization_reference = Some(reference.into());
        self
    }
}

// ============================================================================
// EXECUTION RESULT
// ============================================================================

/// Successfully completed AI execution.
///
/// The result contains the actual provider output and execution metadata.
///
/// It does not claim that every possible target can execute the workload.
pub struct AiExecutionResult<O> {

    id: AiExecutionId,

    operation: String,

    operation_version: AiOperationSetVersion,

    output: O,

    provenance: AiExecutionProvenance,

    realization_reference: Option<String>,

    elapsed: Duration,
}

impl<O> AiExecutionResult<O> {

    /// Return the execution identity.
    #[must_use]
    pub fn id(&self) -> &AiExecutionId {
        &self.id
    }

    /// Return the operation's qualified name.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Return the exact operation-set version.
    #[must_use]
    pub fn operation_version(&self) -> AiOperationSetVersion {
        self.operation_version
    }

    /// Borrow the actual execution output.
    #[must_use]
    pub fn output(&self) -> &O {
        &self.output
    }

    /// Consume the result and return its output.
    #[must_use]
    pub fn into_output(self) -> O {
        self.output
    }

    /// Return execution provenance references.
    #[must_use]
    pub fn provenance(&self) -> &AiExecutionProvenance {
        &self.provenance
    }

    /// Return the optional realization reference.
    #[must_use]
    pub fn realization_reference(&self) -> Option<&str> {
        self.realization_reference.as_deref()
    }

    /// Return elapsed execution-boundary time.
    ///
    /// This duration includes preflight and dispatch time.
    ///
    /// It is not a replacement for the authoritative runtime's
    /// detailed execution timing measurements.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

// ============================================================================
// EXECUTION DISPATCHER
// ============================================================================

/// Integration boundary for AI execution.
///
/// Implementations adapt validated AI workloads to the existing
/// execution infrastructure.
///
/// Possible implementations include:
///
/// - A local execution adapter.
/// - A classical execution adapter.
/// - A GPU or accelerator adapter.
/// - A quantum-assisted execution adapter.
/// - A hybrid execution adapter.
/// - A distributed execution adapter.
/// - A future execution-domain adapter.
///
/// The dispatcher does not have permission to weaken requirements.
///
/// ## Mandatory implementation responsibilities
///
/// `assess` must establish:
///
/// - capability eligibility;
/// - resource feasibility;
/// - compatibility;
/// - applicable authorization;
/// - numerical-contract compatibility;
/// - execution-policy compatibility.
///
/// `execute` must:
///
/// - revalidate volatile execution conditions;
/// - honor cancellation and deadlines;
/// - preserve semantic requirements;
/// - use the authoritative runtime/backend;
/// - preserve actual execution failures;
/// - preserve uncertain distributed outcomes;
/// - return a receipt only after actual successful execution.
///
/// The dispatcher must not fabricate successful outputs.
///
/// Retry decisions remain governed by idempotency, effects,
/// execution outcome and the existing resilience infrastructure.
pub trait AiExecutionDispatcher<W: AiExecutable>: Send + Sync {

    /// Concrete output type produced by the authoritative execution path.
    type Output: Send + Sync;

    /// Assess whether the supplied workload can execute.
    ///
    /// Returning `Indeterminate` does not authorize dispatch.
    fn assess(
        &self,
        workload: &W,
        operation: &AiOperationDescriptor,
    ) -> AiResult<AiExecutionAssessment>;

    /// Execute an eligible AI workload.
    ///
    /// `ExecutionControl` is the existing distributed execution-control
    /// contract. It provides cooperative cancellation and deadlines.
    ///
    /// Implementations integrating local or hybrid execution should
    /// propagate these controls to their owning runtime.
    fn execute(
        &self,
        workload: &W,
        operation: &AiOperationDescriptor,
        control: &ExecutionControl,
    ) -> AiResult<AiExecutionReceipt<Self::Output>>;
}

// ============================================================================
// EXECUTION ENGINE
// ============================================================================

/// Stateless coordinator for AI execution.
///
/// All mutable execution dependencies are injected.
///
/// The engine does not own provider registration, resource allocation,
/// scheduling, device selection or runtime lifecycle.
pub struct AiExecutionEngine;

impl AiExecutionEngine {

    /// Execute a validated AI workload.
    ///
    /// The method performs:
    ///
    /// 1. Request and workload validation.
    /// 2. Requirement completeness validation.
    /// 3. Exact AI operation registration resolution.
    /// 4. Provider eligibility assessment.
    /// 5. Cancellation and deadline validation.
    /// 6. Actual execution dispatch.
    /// 7. Provenance validation.
    /// 8. Result construction.
    ///
    /// The exact operation-set version comes from the workload.
    ///
    /// No operation version is silently substituted.
    ///
    /// # Errors
    ///
    /// Returns a structured AI error for invalid input, incomplete
    /// requirements, unregistered operations, infeasible execution,
    /// indeterminate eligibility, cancellation, deadlines, provider
    /// failures or missing mandatory provenance.
    pub fn execute<'work, 'services, W, D>(
        context: &AiContext<'services>,
        request: &AiExecutionRequest<'work, W>,
        dispatcher: &D,
        control: &ExecutionControl,
    ) -> AiResult<AiExecutionResult<D::Output>>
    where
        W: AiExecutable,
        D: AiExecutionDispatcher<W>,
    {
        let started = Instant::now();

        control.check().map_err(|error| {
            AiError::from_source(
                AiErrorKind::Cancelled,
                "AI execution was cancelled or its deadline expired",
                error,
            )
            .with_phase(AiErrorPhase::Execution)
            .with_outcome(OperationOutcome::NotStarted)
        })?;

        request.workload.validate()?;

        if !request.workload.requirements().is_complete() {
            return Err(
                AiError::new(
                    AiErrorKind::RequirementUnsatisfied,
                    "AI execution requires complete computational requirements",
                )
                .with_phase(AiErrorPhase::Validation)
                .with_outcome(OperationOutcome::NotStarted),
            );
        }

        // Resolve the exact registered operation version.
        //
        // The registry is descriptive metadata. Resolution does not
        // establish that a physical implementation exists.
        let operation = context
            .registry()
            .resolve(
                request.operation_name(),
                request.operation_version(),
            )
            .map_err(|error| {
                AiError::from_source(
                    AiErrorKind::UnsupportedOperation,
                    "requested AI operation is not registered",
                    error,
                )
                .with_phase(AiErrorPhase::Validation)
                .with_operation(
                    format!(
                        "{}:{}",
                        request.operation_name().namespace,
                        request.operation_name().name,
                    ),
                )
                .with_outcome(OperationOutcome::NotStarted)
            })?;

        // Ensure that the resolved operation belongs to the
        // requested AI operation category and version.
        operation.validate().map_err(|error| {
            AiError::from_source(
                AiErrorKind::InvalidOperation,
                "registered AI operation metadata is invalid",
                error,
            )
            .with_phase(AiErrorPhase::Validation)
            .with_outcome(OperationOutcome::NotStarted)
        })?;

        // Provider and execution infrastructure must establish
        // actual feasibility and authorization.
        let assessment = dispatcher.assess(
            request.workload,
            &operation,
        )?;

        match assessment {
            AiExecutionAssessment::Eligible { .. } => {}

            AiExecutionAssessment::Ineligible { reason } => {
                return Err(
                    AiError::new(
                        AiErrorKind::ResourceInfeasible,
                        format!(
                            "AI execution requirements cannot be satisfied: {reason}"
                        ),
                    )
                    .with_phase(AiErrorPhase::ResourceAnalysis)
                    .with_outcome(OperationOutcome::NotStarted)
                    .with_operation(
                        format!(
                            "{}:{}",
                            operation.name().namespace,
                            operation.name().name,
                        ),
                    ),
                );
            }

            AiExecutionAssessment::Indeterminate { reason } => {
                return Err(
                    AiError::new(
                        AiErrorKind::CapabilityUnavailable,
                        format!(
                            "AI execution eligibility could not be established: {reason}"
                        ),
                    )
                    .with_phase(AiErrorPhase::CapabilityNegotiation)
                    .with_retry(RetryDisposition::AfterConditionChange)
                    .with_outcome(OperationOutcome::NotStarted),
                );
            }
        }

        // Recheck control immediately before dispatch.
        control.check().map_err(|error| {
            AiError::from_source(
                AiErrorKind::Cancelled,
                "AI execution was cancelled before dispatch",
                error,
            )
            .with_phase(AiErrorPhase::Execution)
            .with_outcome(OperationOutcome::NotStarted)
        })?;

        // The dispatcher performs actual execution through the
        // authoritative backend, hybrid or distributed infrastructure.
        let receipt = dispatcher.execute(
            request.workload,
            &operation,
            control,
        )?;

        // A provider must not return an invalid success after cancellation
        // has already been observed.
        //
        // If execution has completed, the outcome remains Completed.
        // The completed output is retained only in the receipt owned
        // by the current operation; an error is returned to the caller.
        control.check().map_err(|error| {
            AiError::from_source(
                AiErrorKind::DeadlineExceeded,
                "execution completed but cancellation or deadline was observed at result validation",
                error,
            )
            .with_phase(AiErrorPhase::Execution)
            .with_outcome(OperationOutcome::Completed)
        })?;

        // Enforce configured provenance requirements.
        //
        // Provenance collection and persistence remain the responsibility
        // of the authoritative provenance and execution services.
        if context.provenance_required()
            && !receipt.provenance.is_recorded()
        {
            return Err(
                AiError::new(
                    AiErrorKind::IntegrityFailure,
                    "AI execution completed without the required execution provenance reference",
                )
                .with_phase(AiErrorPhase::Execution)
                .with_outcome(OperationOutcome::Completed)
                .with_operation(
                    format!(
                        "{}:{}",
                        operation.name().namespace,
                        operation.name().name,
                    ),
                ),
            );
        }

        let elapsed = started.elapsed();

        Ok(AiExecutionResult {
            id: request.id.clone(),

            operation: format!(
                "{}:{}",
                operation.name().namespace,
                operation.name().name,
            ),

            operation_version: operation.version(),

            output: receipt.output,

            provenance: receipt.provenance,

            realization_reference: receipt.realization_reference,

            elapsed,
        })
    }
}

// ============================================================================
// EXECUTION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_identifier_rejects_empty_values() {
        assert!(AiExecutionId::new("").is_err());
        assert!(AiExecutionId::new("   ").is_err());
    }

    #[test]
    fn execution_identifier_rejects_control_characters() {
        assert!(AiExecutionId::new("request\ninvalid").is_err());
    }

    #[test]
    fn execution_identifier_preserves_valid_identity() {
        let id = AiExecutionId::new("training/run/001")
            .expect("valid identifier");

        assert_eq!(id.as_str(), "training/run/001");
    }

    #[test]
    fn eligible_assessment_is_explicit() {
        let assessment =
            AiExecutionAssessment::eligible(
                "all mandatory execution requirements were established",
            );

        assert!(assessment.is_eligible());
    }

    #[test]
    fn ineligible_assessment_is_not_eligible() {
        let assessment =
            AiExecutionAssessment::ineligible(
                "required numerical capability is unavailable",
            );

        assert!(!assessment.is_eligible());
    }

    #[test]
    fn indeterminate_assessment_is_not_eligible() {
        let assessment =
            AiExecutionAssessment::indeterminate(
                "capability evidence is incomplete",
            );

        assert!(!assessment.is_eligible());
    }

    #[test]
    fn provenance_requires_an_execution_reference() {
        let empty = AiExecutionProvenance::default();

        assert!(!empty.is_recorded());

        let recorded = AiExecutionProvenance {
            execution_reference: Some(
                "execution/record/001".to_owned(),
            ),
            ..Default::default()
        };

        assert!(recorded.is_recorded());
    }

    #[test]
    fn receipt_preserves_realization_reference() {
        let receipt = AiExecutionReceipt::new(
            42_u64,
            AiExecutionProvenance::default(),
        )
        .with_realization_reference("realization/001");

        assert_eq!(
            receipt.realization_reference.as_deref(),
            Some("realization/001"),
        );

        assert_eq!(receipt.output, 42);
    }

    #[test]
    fn api_version_is_independent_of_hardware() {
        assert_eq!(AI_EXECUTION_API_VERSION, 1);
    }
}
