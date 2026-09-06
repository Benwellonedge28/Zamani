//! Zamani Quantum Resilience — Compensation Recovery
//!
//! Path:
//!     src/quantum/resilience/recovery/compensation.rs
//!
//! Purpose:
//!     Provider-independent execution contract for mathematically defined
//!     compensating recovery actions.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//!     Execution
//!         |
//!         v
//!     Detection
//!         |
//!         v
//!     Diagnosis
//!         |
//!         v
//!     Policy
//!         |
//!         v
//!     Planning
//!         |
//!         v
//!     RecoveryAction::Compensate
//!         |
//!         v
//!     CompensationExecutor          <-- this module
//!         |
//!         +--> canonical quantum IR
//!         +--> execution/runtime provider
//!         +--> capability validation
//!         +--> authorization
//!         +--> semantic validation
//!         |
//!         v
//!     Verification
//!         |
//!         +--> ACCEPT
//!         +--> REJECT
//!         +--> REPLAN
//!         +--> ESCALATE
//!
//! ============================================================================
//! RESPONSIBILITY
//! ============================================================================
//!
//! This module defines the contract and safety machinery for compensation.
//!
//! It owns:
//!
//! - compensation request validation;
//! - compensation plan representation;
//! - compensation operation classification;
//! - precondition requirements;
//! - semantic intent declaration;
//! - execution contract;
//! - compensation lifecycle;
//! - post-compensation verification contract;
//! - deterministic execution requirements;
//! - bounded execution accounting;
//! - provenance metadata;
//! - failure classification;
//! - provider-neutral integration boundaries.
//!
//! It does NOT own:
//!
//! - fault detection;
//! - fault diagnosis;
//! - recovery planning;
//! - policy selection;
//! - canonical quantum IR definition;
//! - quantum gate definitions;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - error mitigation;
//! - hardware discovery;
//! - backend selection;
//! - checkpoint storage;
//! - semantic verification implementation;
//! - provider SDKs;
//! - physical control;
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! ============================================================================
//! CRITICAL QUANTUM INVARIANT
//! ============================================================================
//!
//! Compensation is NOT generic rollback.
//!
//! A classical system can sometimes perform:
//!
//!     operation -> inverse(operation)
//!
//! Quantum execution requires considerably stronger conditions.
//!
//! A compensation operation is valid only when the planner/execution system
//! establishes that the requested compensation preserves the declared
//! computation semantics under the current execution state.
//!
//! This module therefore NEVER assumes that:
//!
//!     "operation B happened after operation A"
//!
//! implies:
//!
//!     "B^-1 restores the previous quantum state."
//!
//! In particular, this module must not:
//!
//! - clone arbitrary unknown quantum state;
//! - serialize arbitrary unknown quantum state;
//! - invent an inverse for a non-invertible operation;
//! - reverse measurement as though measurement were unitary;
//! - reverse irreversible reset as though it were reversible;
//! - undo decoherence;
//! - undo leakage merely by applying an inverse gate;
//! - infer quantum-state recovery from classical metadata alone;
//! - claim semantic equivalence without verification.
//!
//! Valid compensation may instead be based on:
//!
//! - a mathematically established inverse unitary;
//! - a verified inverse circuit;
//! - a known algebraic compensating transformation;
//! - a classical correction associated with a measurement boundary;
//! - a protocol-specific correction;
//! - a fault-tolerant logical correction;
//! - an application-level compensation defined by the program semantics;
//! - another explicitly declared semantic transformation.
//!
//! The actual mathematical validity must be established by the appropriate
//! compiler/IR/QEC/verification subsystem.
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! This file contains no:
//!
//! - maximum qubit count;
//! - maximum device count;
//! - fixed topology;
//! - fixed backend;
//! - provider name;
//! - fixed gate set;
//! - fixed QEC code;
//! - fixed retry count;
//! - fixed timeout;
//! - fixed number of operations;
//! - fixed machine size;
//! - static quantum-resource arrays.
//!
//! Compensation plans refer to opaque identities and canonical program/IR
//! identities rather than physical-machine assumptions.
//!
//! Actual resource limits are supplied by:
//!
//! - hardware capabilities;
//! - runtime capabilities;
//! - policy;
//! - resource availability;
//! - execution budgets;
//! - QEC capabilities;
//! - compiler capabilities;
//! - security authorization.
//!
//! Therefore this module imposes no artificial finite quantum-machine size.
//!
//! "Infinite scale" means that compensation introduces no artificial upper
//! bound. Actual execution remains limited only by the resources and
//! capabilities available to the deployment.
//!
//! ============================================================================
//! CANONICAL QUANTUM IDENTITY
//! ============================================================================
//!
//! This module deliberately does not define QubitId.
//!
//! Compensation operates at the semantic operation/plan boundary.
//!
//! If a concrete implementation needs qubit identity, it MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and MUST NOT introduce a resilience-local QubitId.
//!
//! This keeps quantum identity canonical across:
//!
//!     IR
//!     optimization
//!     routing
//!     scheduling
//!     QEC
//!     recovery
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! planning/action.rs
//!     Provides RecoveryAction::Compensate.
//!
//! planning/plan.rs
//!     Supplies immutable compensation intent and ordering.
//!
//! planning/feasibility.rs
//!     Establishes that the requested compensation can execute.
//!
//! policy/*
//!     Determines whether compensation is allowed.
//!
//! diagnosis/*
//!     Provides the reason/context requiring compensation.
//!
//! quantum::ir
//!     Remains the canonical semantic representation.
//!
//! quantum::optimization
//!     May produce/validate transformed canonical operations.
//!
//! quantum::routing
//!     Remains responsible for physical realization.
//!
//! quantum::scheduling
//!     Remains responsible for execution ordering/timing.
//!
//! quantum::qec
//!     Remains responsible for logical/fault-tolerant corrections.
//!
//! quantum::hardware
//!     Provides capabilities and execution.
//!
//! verification/*
//!     Determines whether compensation preserved required semantics.
//!
//! state/*
//!     Owns durable execution/recovery state.
//!
//! telemetry/*
//!     Consumes compensation lifecycle events.
//!
//! history/*
//!     Records compensation outcomes.
//!
//! coordination/*
//!     Owns distributed ownership/leases.
//!
//! recovery/recoverer.rs
//!     Orchestrates compensation alongside other recovery actions.
//!
//! recovery/rollback.rs
//!     Handles restoration to an earlier valid state; it must not be
//!     conflated with compensation.
//!
//! recovery/resume.rs
//!     Handles continuation from a valid boundary.
//!
//! recovery/migration.rs
//!     Handles movement to another execution environment.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Rust 2021.
//! Rust 1.97 / 1.97.1.
//! No unsafe code.
//! No unsafe FFI.
//! No raw pointers.
//! No credentials.
//! No provider secrets.
//! No executable callbacks in serializable compensation plans.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for compensation requests/results.
pub const COMPENSATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.compensation";

/// Current semantic schema version.
pub const COMPENSATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Stable identifiers
// ============================================================================

/// Stable identity of the execution being compensated.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates a validated execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "execution_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identity of one compensation operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompensationId(Arc<str>);

impl CompensationId {
    /// Creates a validated compensation identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "compensation_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for CompensationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identity of the source operation/cause.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationId(Arc<str>);

impl OperationId {
    /// Creates a validated operation identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "operation_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for OperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Opaque authorization reference.
///
/// The reference is not a credential.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizationRef(Arc<str>);

impl AuthorizationRef {
    /// Creates a validated authorization reference.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "authorization_ref",
            });
        }

        Ok(Self(value))
    }

    /// Returns the opaque reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for AuthorizationRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Semantic identity
// ============================================================================

/// Stable identity of the canonical program whose semantics must be preserved.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramIdentity(Arc<str>);

impl ProgramIdentity {
    /// Creates a program identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "program_identity",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ProgramIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identity of the canonical IR representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrIdentity(Arc<str>);

impl IrIdentity {
    /// Creates an IR identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "ir_identity",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for IrIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Compensation kind
// ============================================================================

/// Mathematical/semantic basis of a compensation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompensationKind {
    /// Apply a mathematically valid inverse unitary.
    InverseUnitary,

    /// Apply a verified inverse circuit.
    InverseCircuit,

    /// Apply an algebraically equivalent correction.
    AlgebraicCorrection,

    /// Apply a protocol-defined correction.
    ProtocolCorrection,

    /// Apply a logical/QEC correction.
    LogicalCorrection,

    /// Apply a classical correction associated with a measurement boundary.
    ClassicalCorrection,

    /// Apply an application-defined semantic compensation.
    ApplicationDefined,
}

impl CompensationKind {
    /// Returns the stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InverseUnitary => "inverse_unitary",
            Self::InverseCircuit => "inverse_circuit",
            Self::AlgebraicCorrection => "algebraic_correction",
            Self::ProtocolCorrection => "protocol_correction",
            Self::LogicalCorrection => "logical_correction",
            Self::ClassicalCorrection => "classical_correction",
            Self::ApplicationDefined => "application_defined",
        }
    }
}

impl fmt::Display for CompensationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Execution boundary
// ============================================================================

/// Semantic boundary at which compensation is permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompensationBoundary {
    /// Compensation is applied inside an explicitly validated circuit region.
    CircuitRegion,

    /// Compensation begins at a classical control boundary.
    ClassicalBoundary,

    /// Compensation is associated with a measurement boundary.
    MeasurementBoundary,

    /// Compensation is associated with a logical/QEC boundary.
    LogicalBoundary,

    /// Compensation is defined by an execution protocol.
    ProtocolBoundary,

    /// Compensation is defined by application semantics.
    ApplicationBoundary,
}

impl CompensationBoundary {
    /// Returns the stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CircuitRegion => "circuit_region",
            Self::ClassicalBoundary => "classical_boundary",
            Self::MeasurementBoundary => "measurement_boundary",
            Self::LogicalBoundary => "logical_boundary",
            Self::ProtocolBoundary => "protocol_boundary",
            Self::ApplicationBoundary => "application_boundary",
        }
    }
}

impl fmt::Display for CompensationBoundary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Mathematical validity
// ============================================================================

/// Level of mathematical evidence supporting the compensation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MathematicalValidity {
    /// Validity has been established by a trusted canonical transformation
    /// subsystem.
    Proven,

    /// Validity has been established but depends on an explicit external
    /// assumption that must be checked before execution.
    Conditional,

    /// Validity has not been established.
    Unknown,

    /// The proposed compensation is known to be invalid.
    Invalid,
}

impl MathematicalValidity {
    /// Returns whether the compensation is safe to submit for execution.
    #[must_use]
    pub const fn executable(self) -> bool {
        matches!(self, Self::Proven | Self::Conditional)
    }
}

// ============================================================================
// Semantic effect
// ============================================================================

/// Declares the intended semantic effect of compensation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticEffect {
    /// Restores the declared precondition of the compensated operation.
    RestorePriorInvariant,

    /// Cancels an explicitly invertible transformation.
    CancelTransformation,

    /// Applies a logical correction without claiming physical state reversal.
    CorrectLogicalState,

    /// Corrects classical interpretation after measurement.
    CorrectClassicalInterpretation,

    /// Applies a protocol-level corrective transformation.
    CorrectProtocolState,

    /// Applies an application-defined semantic correction.
    ApplyApplicationCorrection,
}

impl SemanticEffect {
    /// Returns the stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestorePriorInvariant => "restore_prior_invariant",
            Self::CancelTransformation => "cancel_transformation",
            Self::CorrectLogicalState => "correct_logical_state",
            Self::CorrectClassicalInterpretation => "correct_classical_interpretation",
            Self::CorrectProtocolState => "correct_protocol_state",
            Self::ApplyApplicationCorrection => "apply_application_correction",
        }
    }
}

// ============================================================================
// Verification requirement
// ============================================================================

/// Minimum verification requirement following compensation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationRequirement {
    /// Canonical semantic verification is mandatory.
    Semantic,

    /// Semantic and result verification are mandatory.
    SemanticAndResult,

    /// Full verification including provenance/integrity is mandatory.
    Full,

    /// Compensation must not be accepted automatically.
    ExternalAcceptance,
}

impl VerificationRequirement {
    /// Returns whether semantic verification is required.
    #[must_use]
    pub const fn requires_semantic(self) -> bool {
        true
    }
}

// ============================================================================
// Resource requirements
// ============================================================================

/// Provider-neutral resource requirement.
///
/// This is intentionally generic. The resource subsystem decides what the
/// resource means.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRequirement {
    resource_class: Arc<str>,
    quantity: u64,
    unit: Arc<str>,
}

impl ResourceRequirement {
    /// Creates a resource requirement.
    pub fn new(
        resource_class: impl Into<Arc<str>>,
        quantity: u64,
        unit: impl Into<Arc<str>>,
    ) -> Result<Self, CompensationError> {
        let resource_class = resource_class.into();
        let unit = unit.into();

        if resource_class.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "resource_class",
            });
        }

        if unit.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "resource_unit",
            });
        }

        Ok(Self {
            resource_class,
            quantity,
            unit,
        })
    }

    /// Returns the resource class.
    #[must_use]
    pub fn resource_class(&self) -> &str {
        self.resource_class.as_ref()
    }

    /// Returns the requested quantity.
    #[must_use]
    pub const fn quantity(&self) -> u64 {
        self.quantity
    }

    /// Returns the unit.
    #[must_use]
    pub fn unit(&self) -> &str {
        self.unit.as_ref()
    }
}

// ============================================================================
// Compensation operation reference
// ============================================================================

/// Reference to the canonical operation transformation that implements the
/// compensation.
///
/// The actual operation/circuit remains owned by the canonical IR/compiler
/// subsystem.
///
/// This module stores identity and semantic metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompensationOperation {
    /// Identity of the original operation.
    source_operation: OperationId,

    /// Identity of the resulting canonical transformation.
    compensation_operation: OperationId,

    /// Identity of the canonical IR containing the transformation.
    ir_identity: IrIdentity,

    /// Mathematical validity supplied by the transformation subsystem.
    mathematical_validity: MathematicalValidity,

    /// Declared semantic effect.
    semantic_effect: SemanticEffect,

    /// Boundary at which the compensation is valid.
    boundary: CompensationBoundary,

    /// Stable transformation proof/reference.
    proof_reference: Arc<str>,
}

impl CompensationOperation {
    /// Creates a compensation operation reference.
    pub fn new(
        source_operation: OperationId,
        compensation_operation: OperationId,
        ir_identity: IrIdentity,
        mathematical_validity: MathematicalValidity,
        semantic_effect: SemanticEffect,
        boundary: CompensationBoundary,
        proof_reference: impl Into<Arc<str>>,
    ) -> Result<Self, CompensationError> {
        let proof_reference = proof_reference.into();

        if proof_reference.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "proof_reference",
            });
        }

        if mathematical_validity == MathematicalValidity::Invalid {
            return Err(CompensationError::MathematicallyInvalid);
        }

        Ok(Self {
            source_operation,
            compensation_operation,
            ir_identity,
            mathematical_validity,
            semantic_effect,
            boundary,
            proof_reference,
        })
    }

    /// Returns the source operation.
    #[must_use]
    pub fn source_operation(&self) -> &OperationId {
        &self.source_operation
    }

    /// Returns the compensation operation.
    #[must_use]
    pub fn compensation_operation(&self) -> &OperationId {
        &self.compensation_operation
    }

    /// Returns the IR identity.
    #[must_use]
    pub fn ir_identity(&self) -> &IrIdentity {
        &self.ir_identity
    }

    /// Returns the mathematical validity.
    #[must_use]
    pub const fn mathematical_validity(&self) -> MathematicalValidity {
        self.mathematical_validity
    }

    /// Returns the semantic effect.
    #[must_use]
    pub const fn semantic_effect(&self) -> SemanticEffect {
        self.semantic_effect
    }

    /// Returns the valid execution boundary.
    #[must_use]
    pub const fn boundary(&self) -> CompensationBoundary {
        self.boundary
    }

    /// Returns the proof/reference identity.
    #[must_use]
    pub fn proof_reference(&self) -> &str {
        self.proof_reference.as_ref()
    }
}

// ============================================================================
// Compensation request
// ============================================================================

/// Immutable request to execute one compensation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationRequest {
    compensation_id: CompensationId,
    execution_id: ExecutionId,
    program_identity: ProgramIdentity,
    operation: CompensationOperation,
    kind: CompensationKind,
    authorization: AuthorizationRef,
    verification_requirement: VerificationRequirement,
    resource_requirements: Vec<ResourceRequirement>,
    deterministic: bool,
}

impl CompensationRequest {
    /// Creates a compensation request.
    pub fn new(
        compensation_id: CompensationId,
        execution_id: ExecutionId,
        program_identity: ProgramIdentity,
        operation: CompensationOperation,
        kind: CompensationKind,
        authorization: AuthorizationRef,
        verification_requirement: VerificationRequirement,
        resource_requirements: Vec<ResourceRequirement>,
        deterministic: bool,
    ) -> Result<Self, CompensationError> {
        if !operation.mathematical_validity().executable() {
            return Err(CompensationError::MathematicalValidityUnavailable);
        }

        if resource_requirements.iter().any(|requirement| {
            requirement.resource_class().is_empty()
                || requirement.unit().is_empty()
        }) {
            return Err(CompensationError::InvalidResourceRequirement);
        }

        Ok(Self {
            compensation_id,
            execution_id,
            program_identity,
            operation,
            kind,
            authorization,
            verification_requirement,
            resource_requirements,
            deterministic,
        })
    }

    /// Returns the compensation identity.
    #[must_use]
    pub fn compensation_id(&self) -> &CompensationId {
        &self.compensation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the program identity.
    #[must_use]
    pub fn program_identity(&self) -> &ProgramIdentity {
        &self.program_identity
    }

    /// Returns the operation reference.
    #[must_use]
    pub fn operation(&self) -> &CompensationOperation {
        &self.operation
    }

    /// Returns the compensation kind.
    #[must_use]
    pub const fn kind(&self) -> CompensationKind {
        self.kind
    }

    /// Returns the authorization reference.
    #[must_use]
    pub fn authorization(&self) -> &AuthorizationRef {
        &self.authorization
    }

    /// Returns the verification requirement.
    #[must_use]
    pub const fn verification_requirement(&self) -> VerificationRequirement {
        self.verification_requirement
    }

    /// Returns resource requirements.
    #[must_use]
    pub fn resource_requirements(&self) -> &[ResourceRequirement] {
        &self.resource_requirements
    }

    /// Returns whether deterministic execution is required.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }
}

// ============================================================================
// Capability assessment
// ============================================================================

/// Result of checking whether compensation can execute on the current target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityAssessment {
    compatible: bool,
    capability_identity: Arc<str>,
    missing_capabilities: Vec<Arc<str>>,
    degraded: bool,
}

impl CapabilityAssessment {
    /// Creates a capability assessment.
    pub fn new(
        compatible: bool,
        capability_identity: impl Into<Arc<str>>,
        missing_capabilities: Vec<Arc<str>>,
        degraded: bool,
    ) -> Result<Self, CompensationError> {
        let capability_identity = capability_identity.into();

        if capability_identity.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "capability_identity",
            });
        }

        if compatible && !missing_capabilities.is_empty() {
            return Err(CompensationError::CapabilityAssessmentContradiction);
        }

        Ok(Self {
            compatible,
            capability_identity,
            missing_capabilities,
            degraded,
        })
    }

    /// Returns whether compensation is supported.
    #[must_use]
    pub const fn compatible(&self) -> bool {
        self.compatible
    }

    /// Returns the capability snapshot identity.
    #[must_use]
    pub fn capability_identity(&self) -> &str {
        self.capability_identity.as_ref()
    }

    /// Returns capabilities that are missing.
    #[must_use]
    pub fn missing_capabilities(&self) -> &[Arc<str>] {
        &self.missing_capabilities
    }

    /// Returns whether execution would be degraded.
    #[must_use]
    pub const fn degraded(&self) -> bool {
        self.degraded
    }
}

// ============================================================================
// Authorization assessment
// ============================================================================

/// Result of authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorizationAssessment {
    /// Compensation is authorized.
    Authorized,

    /// Compensation is not authorized.
    Denied,
}

// ============================================================================
// Compensation lifecycle
// ============================================================================

/// Lifecycle state for one compensation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompensationState {
    /// Request has been created but not validated.
    Created,

    /// Request is being validated.
    Validating,

    /// Compensation has been authorized.
    Authorized,

    /// Target capabilities are being checked.
    CheckingCapabilities,

    /// Compensation is being prepared.
    Preparing,

    /// Compensation is being executed.
    Executing,

    /// Result is being verified.
    Verifying,

    /// Compensation has been accepted.
    Accepted,

    /// Compensation completed but execution remains degraded.
    Degraded,

    /// Compensation was rejected.
    Rejected,

    /// Compensation requires a new recovery plan.
    NeedsReplan,

    /// Automatic recovery cannot safely continue.
    Escalated,

    /// Compensation operation failed.
    Failed,
}

impl CompensationState {
    /// Returns whether this is a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::Degraded
                | Self::Rejected
                | Self::NeedsReplan
                | Self::Escalated
                | Self::Failed
        )
    }
}

// ============================================================================
// Execution handle
// ============================================================================

/// Opaque handle to an execution created/modified by compensation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompensationExecutionHandle(Arc<str>);

impl CompensationExecutionHandle {
    /// Creates a handle.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CompensationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "execution_handle",
            });
        }

        Ok(Self(value))
    }

    /// Returns the handle.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// ============================================================================
// Verification
// ============================================================================

/// Result supplied by the verification subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompensationVerification {
    /// Semantic correctness established.
    Accepted {
        confidence_basis: Arc<str>,
    },

    /// Semantics are preserved but operation is degraded.
    AcceptedDegraded {
        reason: Arc<str>,
        confidence_basis: Arc<str>,
    },

    /// Compensation must be replanned.
    NeedsReplan {
        reason: Arc<str>,
    },

    /// Compensation cannot be accepted.
    Rejected {
        reason: Arc<str>,
    },
}

impl CompensationVerification {
    /// Returns whether the compensation can be accepted.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        matches!(
            self,
            Self::Accepted { .. } | Self::AcceptedDegraded { .. }
        )
    }

    /// Returns whether the result is degraded.
    #[must_use]
    pub const fn degraded(&self) -> bool {
        matches!(self, Self::AcceptedDegraded { .. })
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Immutable provenance record for compensation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationProvenance {
    compensation_id: CompensationId,
    execution_id: ExecutionId,
    program_identity: ProgramIdentity,
    ir_identity: IrIdentity,
    source_operation: OperationId,
    compensation_operation: OperationId,
    compensation_kind: CompensationKind,
    boundary: CompensationBoundary,
    mathematical_validity: MathematicalValidity,
    proof_reference: Arc<str>,
    capability_identity: Arc<str>,
}

impl CompensationProvenance {
    /// Creates provenance from a request and capability assessment.
    pub fn from_request(
        request: &CompensationRequest,
        capabilities: &CapabilityAssessment,
    ) -> Self {
        Self {
            compensation_id: request.compensation_id().clone(),
            execution_id: request.execution_id().clone(),
            program_identity: request.program_identity().clone(),
            ir_identity: request.operation().ir_identity().clone(),
            source_operation: request.operation().source_operation().clone(),
            compensation_operation: request
                .operation()
                .compensation_operation()
                .clone(),
            compensation_kind: request.kind(),
            boundary: request.operation().boundary(),
            mathematical_validity: request.operation().mathematical_validity(),
            proof_reference: Arc::from(request.operation().proof_reference()),
            capability_identity: Arc::from(capabilities.capability_identity()),
        }
    }

    /// Returns the compensation identity.
    #[must_use]
    pub fn compensation_id(&self) -> &CompensationId {
        &self.compensation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the program identity.
    #[must_use]
    pub fn program_identity(&self) -> &ProgramIdentity {
        &self.program_identity
    }

    /// Returns the IR identity.
    #[must_use]
    pub fn ir_identity(&self) -> &IrIdentity {
        &self.ir_identity
    }

    /// Returns the proof reference.
    #[must_use]
    pub fn proof_reference(&self) -> &str {
        self.proof_reference.as_ref()
    }
}

// ============================================================================
// Final result
// ============================================================================

/// Final result of a compensation attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationResult {
    compensation_id: CompensationId,
    execution_id: ExecutionId,
    state: CompensationState,
    execution: Option<CompensationExecutionHandle>,
    verification: Option<CompensationVerification>,
    provenance: CompensationProvenance,
    degraded: bool,
}

impl CompensationResult {
    /// Creates a result.
    fn new(
        request: &CompensationRequest,
        state: CompensationState,
        execution: Option<CompensationExecutionHandle>,
        verification: Option<CompensationVerification>,
        capabilities: &CapabilityAssessment,
    ) -> Self {
        Self {
            compensation_id: request.compensation_id().clone(),
            execution_id: request.execution_id().clone(),
            state,
            execution,
            verification,
            provenance: CompensationProvenance::from_request(
                request,
                capabilities,
            ),
            degraded: capabilities.degraded(),
        }
    }

    /// Returns the compensation identity.
    #[must_use]
    pub fn compensation_id(&self) -> &CompensationId {
        &self.compensation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the lifecycle state.
    #[must_use]
    pub const fn state(&self) -> CompensationState {
        self.state
    }

    /// Returns the execution handle, if one was created.
    #[must_use]
    pub fn execution(&self) -> Option<&CompensationExecutionHandle> {
        self.execution.as_ref()
    }

    /// Returns verification.
    #[must_use]
    pub fn verification(&self) -> Option<&CompensationVerification> {
        self.verification.as_ref()
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> &CompensationProvenance {
        &self.provenance
    }

    /// Returns whether execution is degraded.
    #[must_use]
    pub const fn degraded(&self) -> bool {
        self.degraded
    }

    /// Returns whether compensation was accepted.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        matches!(
            self.state,
            CompensationState::Accepted | CompensationState::Degraded
        )
    }
}

// ============================================================================
// Compensation environment contracts
// ============================================================================

/// Capability provider.
///
/// Implemented by the hardware/runtime integration layer.
pub trait CompensationCapabilityProvider: Send + Sync {
    /// Checks whether the current target can execute the compensation.
    fn assess(
        &self,
        request: &CompensationRequest,
    ) -> Result<CapabilityAssessment, CompensationError>;
}

/// Authorization provider.
///
/// Implemented by policy/security/coordination integration.
pub trait CompensationAuthorizer: Send + Sync {
    /// Authorizes the requested compensation.
    fn authorize(
        &self,
        request: &CompensationRequest,
    ) -> Result<AuthorizationAssessment, CompensationError>;
}

/// Execution provider.
///
/// Implemented by the runtime/HAL integration layer.
///
/// The provider MUST treat `compensation_id` as an idempotency identity.
///
/// Repeating the same request identity must not silently create conflicting
/// duplicate compensation operations.
pub trait CompensationRuntime: Send + Sync {
    /// Prepares the compensation.
    fn prepare(
        &self,
        request: &CompensationRequest,
    ) -> Result<(), CompensationError>;

    /// Executes the canonical compensation transformation.
    fn execute(
        &self,
        request: &CompensationRequest,
    ) -> Result<CompensationExecutionHandle, CompensationError>;

    /// Performs provider/runtime cleanup when execution did not complete.
    fn abort(
        &self,
        request: &CompensationRequest,
    ) -> Result<(), CompensationError>;
}

/// Semantic verifier.
///
/// Implemented by `quantum::resilience::verification`.
pub trait CompensationVerifier: Send + Sync {
    /// Verifies the result of compensation.
    fn verify(
        &self,
        request: &CompensationRequest,
        execution: &CompensationExecutionHandle,
    ) -> Result<CompensationVerification, CompensationError>;
}

/// Optional lifecycle observer.
///
/// Telemetry/history implementations can consume these events.
pub trait CompensationObserver: Send + Sync {
    /// Called whenever compensation changes lifecycle state.
    fn state_changed(
        &self,
        compensation_id: &CompensationId,
        state: CompensationState,
    );
}

/// Cancellation source.
///
/// Cancellation is deliberately injected rather than implemented using
/// global process state.
pub trait CompensationCancellation: Send + Sync {
    /// Returns whether the caller requested cancellation.
    fn is_cancelled(&self) -> bool;
}

// ============================================================================
// No-op cancellation/observer implementations
// ============================================================================

/// Cancellation source that never cancels.
#[derive(Debug, Default)]
pub struct NeverCancelled;

impl CompensationCancellation for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Observer that discards lifecycle events.
#[derive(Debug, Default)]
pub struct NoopCompensationObserver;

impl CompensationObserver for NoopCompensationObserver {
    fn state_changed(
        &self,
        _compensation_id: &CompensationId,
        _state: CompensationState,
    ) {
    }
}

// ============================================================================
// Executor
// ============================================================================

/// Production compensation executor.
///
/// The executor coordinates the contracts but does not implement quantum
/// transformations itself.
pub struct CompensationExecutor {
    capabilities: Arc<dyn CompensationCapabilityProvider>,
    authorizer: Arc<dyn CompensationAuthorizer>,
    runtime: Arc<dyn CompensationRuntime>,
    verifier: Arc<dyn CompensationVerifier>,
    observer: Arc<dyn CompensationObserver>,
    cancellation: Arc<dyn CompensationCancellation>,
}

impl CompensationExecutor {
    /// Creates a compensation executor from injected dependencies.
    ///
    /// Dependencies are explicit so the compensation layer remains:
    ///
    /// - provider-independent;
    /// - testable;
    /// - deterministic;
    /// - replaceable;
    /// - free of global state.
    #[must_use]
    pub fn new(
        capabilities: Arc<dyn CompensationCapabilityProvider>,
        authorizer: Arc<dyn CompensationAuthorizer>,
        runtime: Arc<dyn CompensationRuntime>,
        verifier: Arc<dyn CompensationVerifier>,
        observer: Arc<dyn CompensationObserver>,
        cancellation: Arc<dyn CompensationCancellation>,
    ) -> Self {
        Self {
            capabilities,
            authorizer,
            runtime,
            verifier,
            observer,
            cancellation,
        }
    }

    /// Executes one compensation request.
    ///
    /// The lifecycle is intentionally linear:
    ///
    ///     validate
    ///       -> authorize
    ///       -> capabilities
    ///       -> prepare
    ///       -> execute
    ///       -> verify
    ///       -> accept/reject
    ///
    /// There is no implicit retry loop.
    ///
    /// Retry decisions belong to:
    ///
    ///     policy/retry.rs
    ///     planning/planner.rs
    ///     recovery/recoverer.rs
    ///
    /// This method never accepts compensation solely because the runtime
    /// reported successful execution. Semantic verification is mandatory.
    pub fn execute(
        &self,
        request: &CompensationRequest,
    ) -> Result<CompensationResult, CompensationError> {
        self.validate_request(request)?;

        self.transition(request, CompensationState::Validating);

        self.check_cancellation()?;

        self.transition(request, CompensationState::Authorized);

        match self.authorizer.authorize(request)? {
            AuthorizationAssessment::Authorized => {}
            AuthorizationAssessment::Denied => {
                self.transition(request, CompensationState::Rejected);

                return Err(CompensationError::AuthorizationDenied);
            }
        }

        self.check_cancellation()?;

        self.transition(
            request,
            CompensationState::CheckingCapabilities,
        );

        let capabilities = self.capabilities.assess(request)?;

        if !capabilities.compatible() {
            self.transition(request, CompensationState::NeedsReplan);

            return Err(CompensationError::CapabilityUnavailable {
                missing: capabilities.missing_capabilities().len(),
            });
        }

        self.check_cancellation()?;

        self.transition(request, CompensationState::Preparing);

        if let Err(error) = self.runtime.prepare(request) {
            self.transition(request, CompensationState::Failed);

            return Err(error);
        }

        self.check_cancellation()?;

        self.transition(request, CompensationState::Executing);

        let execution = match self.runtime.execute(request) {
            Ok(execution) => execution,
            Err(error) => {
                let cleanup = self.runtime.abort(request);

                self.transition(request, CompensationState::Failed);

                if let Err(cleanup_error) = cleanup {
                    return Err(CompensationError::ExecutionAndCleanupFailed {
                        execution: Box::new(error),
                        cleanup: Box::new(cleanup_error),
                    });
                }

                return Err(error);
            }
        };

        // Once execution has produced an execution handle, verification is
        // mandatory even when cancellation has been requested. The system must
        // not abandon an already-mutated execution without determining whether
        // the resulting state is semantically valid.
        self.transition(request, CompensationState::Verifying);

        let verification = match self.verifier.verify(request, &execution) {
            Ok(verification) => verification,
            Err(error) => {
                let cleanup = self.runtime.abort(request);

                self.transition(request, CompensationState::Failed);

                if let Err(cleanup_error) = cleanup {
                    return Err(CompensationError::VerificationAndCleanupFailed {
                        verification: Box::new(error),
                        cleanup: Box::new(cleanup_error),
                    });
                }

                return Err(error);
            }
        };

        match &verification {
            CompensationVerification::Accepted { .. } => {
                self.transition(request, CompensationState::Accepted);

                Ok(CompensationResult::new(
                    request,
                    CompensationState::Accepted,
                    Some(execution),
                    Some(verification),
                    &capabilities,
                ))
            }

            CompensationVerification::AcceptedDegraded { .. } => {
                self.transition(request, CompensationState::Degraded);

                Ok(CompensationResult::new(
                    request,
                    CompensationState::Degraded,
                    Some(execution),
                    Some(verification),
                    &capabilities,
                ))
            }

            CompensationVerification::NeedsReplan { .. } => {
                let cleanup = self.runtime.abort(request);

                self.transition(request, CompensationState::NeedsReplan);

                if let Err(cleanup_error) = cleanup {
                    return Err(CompensationError::VerificationAndCleanupFailed {
                        verification: Box::new(
                            CompensationError::VerificationRequiresReplan,
                        ),
                        cleanup: Box::new(cleanup_error),
                    });
                }

                Ok(CompensationResult::new(
                    request,
                    CompensationState::NeedsReplan,
                    None,
                    Some(verification),
                    &capabilities,
                ))
            }

            CompensationVerification::Rejected { .. } => {
                let cleanup = self.runtime.abort(request);

                self.transition(request, CompensationState::Rejected);

                if let Err(cleanup_error) = cleanup {
                    return Err(CompensationError::VerificationAndCleanupFailed {
                        verification: Box::new(
                            CompensationError::VerificationRejected,
                        ),
                        cleanup: Box::new(cleanup_error),
                    });
                }

                Ok(CompensationResult::new(
                    request,
                    CompensationState::Rejected,
                    None,
                    Some(verification),
                    &capabilities,
                ))
            }
        }
    }

    fn validate_request(
        &self,
        request: &CompensationRequest,
    ) -> Result<(), CompensationError> {
        if request.compensation_id().as_str().is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "compensation_id",
            });
        }

        if request.execution_id().as_str().is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "execution_id",
            });
        }

        if request.program_identity().as_str().is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "program_identity",
            });
        }

        if request.authorization().as_str().is_empty() {
            return Err(CompensationError::InvalidIdentifier {
                field: "authorization_ref",
            });
        }

        if request.operation().mathematical_validity()
            == MathematicalValidity::Invalid
        {
            return Err(CompensationError::MathematicallyInvalid);
        }

        if request.operation().mathematical_validity()
            == MathematicalValidity::Unknown
        {
            return Err(CompensationError::MathematicalValidityUnavailable);
        }

        if request.operation().proof_reference().is_empty() {
            return Err(CompensationError::MissingProofReference);
        }

        if request.verification_requirement().requires_semantic() == false {
            return Err(CompensationError::SemanticVerificationRequired);
        }

        Ok(())
    }

    fn check_cancellation(&self) -> Result<(), CompensationError> {
        if self.cancellation.is_cancelled() {
            return Err(CompensationError::Cancelled);
        }

        Ok(())
    }

    fn transition(
        &self,
        request: &CompensationRequest,
        state: CompensationState,
    ) {
        self.observer
            .state_changed(request.compensation_id(), state);
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Complete provider-neutral compensation error taxonomy.
///
/// These errors are intentionally local to the compensation contract.
/// `errors/error.rs` can map them into the repository-wide
/// `ResilienceError` taxonomy without making this module depend on concrete
/// error implementation details.
#[derive(Debug)]
pub enum CompensationError {
    /// An identifier was empty.
    InvalidIdentifier {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// The requested compensation is mathematically invalid.
    MathematicallyInvalid,

    /// Mathematical validity was not established.
    MathematicalValidityUnavailable,

    /// No transformation proof/reference was supplied.
    MissingProofReference,

    /// Resource requirement was malformed.
    InvalidResourceRequirement,

    /// Capability assessment was internally contradictory.
    CapabilityAssessmentContradiction,

    /// Target does not support the compensation.
    CapabilityUnavailable {
        /// Number of missing capabilities.
        missing: usize,
    },

    /// Authorization denied the operation.
    AuthorizationDenied,

    /// Semantic verification is mandatory for compensation.
    SemanticVerificationRequired,

    /// Caller requested cancellation before the operation completed.
    Cancelled,

    /// Execution provider failed.
    ExecutionFailed,

    /// Execution failed and cleanup also failed.
    ExecutionAndCleanupFailed {
        /// Original execution error.
        execution: Box<Self>,

        /// Cleanup error.
        cleanup: Box<Self>,
    },

    /// Verification failed and cleanup also failed.
    VerificationAndCleanupFailed {
        /// Verification error.
        verification: Box<Self>,

        /// Cleanup error.
        cleanup: Box<Self>,
    },

    /// Verification explicitly rejected compensation.
    VerificationRejected,

    /// Verification requires another recovery plan.
    VerificationRequiresReplan,
}

impl fmt::Display for CompensationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid compensation identifier: {field}")
            }

            Self::MathematicallyInvalid => {
                formatter.write_str("compensation is mathematically invalid")
            }

            Self::MathematicalValidityUnavailable => {
                formatter.write_str(
                    "mathematical validity of compensation is unavailable",
                )
            }

            Self::MissingProofReference => {
                formatter.write_str(
                    "compensation proof/reference is missing",
                )
            }

            Self::InvalidResourceRequirement => {
                formatter.write_str("invalid compensation resource requirement")
            }

            Self::CapabilityAssessmentContradiction => {
                formatter.write_str(
                    "capability assessment contains contradictory values",
                )
            }

            Self::CapabilityUnavailable { missing } => {
                write!(
                    formatter,
                    "compensation target lacks {missing} required capabilities"
                )
            }

            Self::AuthorizationDenied => {
                formatter.write_str("compensation authorization denied")
            }

            Self::SemanticVerificationRequired => {
                formatter.write_str(
                    "semantic verification is required for compensation",
                )
            }

            Self::Cancelled => {
                formatter.write_str("compensation was cancelled")
            }

            Self::ExecutionFailed => {
                formatter.write_str("compensation execution failed")
            }

            Self::ExecutionAndCleanupFailed { .. } => {
                formatter.write_str(
                    "compensation execution and cleanup both failed",
                )
            }

            Self::VerificationAndCleanupFailed { .. } => {
                formatter.write_str(
                    "compensation verification and cleanup both failed",
                )
            }

            Self::VerificationRejected => {
                formatter.write_str(
                    "compensation was rejected by verification",
                )
            }

            Self::VerificationRequiresReplan => {
                formatter.write_str(
                    "compensation verification requires replanning",
                )
            }
        }
    }
}

impl std::error::Error for CompensationError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> (
        CompensationId,
        ExecutionId,
        ProgramIdentity,
        OperationId,
        OperationId,
        IrIdentity,
        AuthorizationRef,
    ) {
        (
            CompensationId::new("comp-1").expect("valid compensation id"),
            ExecutionId::new("exec-1").expect("valid execution id"),
            ProgramIdentity::new("program-1").expect("valid program id"),
            OperationId::new("operation-1").expect("valid source operation"),
            OperationId::new("operation-1-inverse")
                .expect("valid compensation operation"),
            IrIdentity::new("ir-1").expect("valid ir identity"),
            AuthorizationRef::new("auth-1").expect("valid authorization"),
        )
    }

    fn request() -> CompensationRequest {
        let (
            compensation_id,
            execution_id,
            program_identity,
            source_operation,
            compensation_operation,
            ir_identity,
            authorization,
        ) = ids();

        let operation = CompensationOperation::new(
            source_operation,
            compensation_operation,
            ir_identity,
            MathematicalValidity::Proven,
            SemanticEffect::CancelTransformation,
            CompensationBoundary::CircuitRegion,
            "proof-1",
        )
        .expect("valid compensation operation");

        CompensationRequest::new(
            compensation_id,
            execution_id,
            program_identity,
            operation,
            CompensationKind::InverseCircuit,
            authorization,
            VerificationRequirement::Full,
            Vec::new(),
            true,
        )
        .expect("valid request")
    }

    #[derive(Debug)]
    struct TestCapabilities;

    impl CompensationCapabilityProvider for TestCapabilities {
        fn assess(
            &self,
            _request: &CompensationRequest,
        ) -> Result<CapabilityAssessment, CompensationError> {
            CapabilityAssessment::new(true, "capabilities-1", Vec::new(), false)
        }
    }

    #[derive(Debug)]
    struct TestAuthorizer;

    impl CompensationAuthorizer for TestAuthorizer {
        fn authorize(
            &self,
            _request: &CompensationRequest,
        ) -> Result<AuthorizationAssessment, CompensationError> {
            Ok(AuthorizationAssessment::Authorized)
        }
    }

    #[derive(Debug)]
    struct TestRuntime;

    impl CompensationRuntime for TestRuntime {
        fn prepare(
            &self,
            _request: &CompensationRequest,
        ) -> Result<(), CompensationError> {
            Ok(())
        }

        fn execute(
            &self,
            _request: &CompensationRequest,
        ) -> Result<CompensationExecutionHandle, CompensationError> {
            CompensationExecutionHandle::new("execution-result-1")
        }

        fn abort(
            &self,
            _request: &CompensationRequest,
        ) -> Result<(), CompensationError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestVerifier;

    impl CompensationVerifier for TestVerifier {
        fn verify(
            &self,
            _request: &CompensationRequest,
            _execution: &CompensationExecutionHandle,
        ) -> Result<CompensationVerification, CompensationError> {
            Ok(CompensationVerification::Accepted {
                confidence_basis: Arc::from("deterministic-test"),
            })
        }
    }

    #[test]
    fn valid_compensation_is_accepted_only_after_verification() {
        let executor = CompensationExecutor::new(
            Arc::new(TestCapabilities),
            Arc::new(TestAuthorizer),
            Arc::new(TestRuntime),
            Arc::new(TestVerifier),
            Arc::new(NoopCompensationObserver),
            Arc::new(NeverCancelled),
        );

        let result = executor.execute(&request()).expect("accepted");

        assert_eq!(result.state(), CompensationState::Accepted);
        assert!(result.accepted());
        assert!(result.verification().is_some());
    }

    #[test]
    fn unknown_mathematical_validity_is_rejected() {
        let (
            compensation_id,
            execution_id,
            program_identity,
            source_operation,
            compensation_operation,
            ir_identity,
            authorization,
        ) = ids();

        let operation = CompensationOperation::new(
            source_operation,
            compensation_operation,
            ir_identity,
            MathematicalValidity::Unknown,
            SemanticEffect::CancelTransformation,
            CompensationBoundary::CircuitRegion,
            "proof-unknown",
        )
        .expect("construction is allowed");

        let result = CompensationRequest::new(
            compensation_id,
            execution_id,
            program_identity,
            operation,
            CompensationKind::InverseCircuit,
            authorization,
            VerificationRequirement::Semantic,
            Vec::new(),
            true,
        );

        assert!(matches!(
            result,
            Err(CompensationError::MathematicalValidityUnavailable)
        ));
    }

    #[test]
    fn mathematically_invalid_compensation_cannot_be_constructed() {
        let (
            _compensation_id,
            _execution_id,
            _program_identity,
            source_operation,
            compensation_operation,
            ir_identity,
            _authorization,
        ) = ids();

        let result = CompensationOperation::new(
            source_operation,
            compensation_operation,
            ir_identity,
            MathematicalValidity::Invalid,
            SemanticEffect::CancelTransformation,
            CompensationBoundary::CircuitRegion,
            "invalid-proof",
        );

        assert!(matches!(
            result,
            Err(CompensationError::MathematicallyInvalid)
        ));
    }

    #[test]
    fn cancellation_is_checked_before_execution() {
        #[derive(Debug)]
        struct Cancelled;

        impl CompensationCancellation for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let executor = CompensationExecutor::new(
            Arc::new(TestCapabilities),
            Arc::new(TestAuthorizer),
            Arc::new(TestRuntime),
            Arc::new(TestVerifier),
            Arc::new(NoopCompensationObserver),
            Arc::new(Cancelled),
        );

        let result = executor.execute(&request());

        assert!(matches!(result, Err(CompensationError::Cancelled)));
    }

    #[test]
    fn capability_failure_does_not_execute_compensation() {
        #[derive(Debug)]
        struct Unsupported;

        impl CompensationCapabilityProvider for Unsupported {
            fn assess(
                &self,
                _request: &CompensationRequest,
            ) -> Result<CapabilityAssessment, CompensationError> {
                CapabilityAssessment::new(
                    false,
                    "capabilities-unsupported",
                    vec![Arc::from("required-operation")],
                    false,
                )
            }
        }

        let executor = CompensationExecutor::new(
            Arc::new(Unsupported),
            Arc::new(TestAuthorizer),
            Arc::new(TestRuntime),
            Arc::new(TestVerifier),
            Arc::new(NoopCompensationObserver),
            Arc::new(NeverCancelled),
        );

        let result = executor.execute(&request());

        assert!(matches!(
            result,
            Err(CompensationError::CapabilityUnavailable { .. })
        ));
    }

    #[test]
    fn resource_requirements_are_dynamic() {
        let (
            compensation_id,
            execution_id,
            program_identity,
            source_operation,
            compensation_operation,
            ir_identity,
            authorization,
        ) = ids();

        let operation = CompensationOperation::new(
            source_operation,
            compensation_operation,
            ir_identity,
            MathematicalValidity::Proven,
            SemanticEffect::CorrectLogicalState,
            CompensationBoundary::LogicalBoundary,
            "proof-logical",
        )
        .expect("valid operation");

        let requirement =
            ResourceRequirement::new("logical_resource", 128, "units")
                .expect("valid resource");

        let request = CompensationRequest::new(
            compensation_id,
            execution_id,
            program_identity,
            operation,
            CompensationKind::LogicalCorrection,
            authorization,
            VerificationRequirement::Full,
            vec![requirement],
            true,
        )
        .expect("valid request");

        assert_eq!(request.resource_requirements().len(), 1);
        assert_eq!(
            request.resource_requirements()[0].quantity(),
            128
        );
    }
}