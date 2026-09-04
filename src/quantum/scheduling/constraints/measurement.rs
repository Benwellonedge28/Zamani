//! Zamani Quantum Scheduling — Measurement Constraints
//!
//! Production measurement/readout constraints for the generic quantum
//! scheduling framework.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! This module answers:
//!
//! > "Can this proposed measurement-related scheduling decision be admitted
//! > under the measurement/readout constraints supplied by the target?"
//!
//! This module is deliberately a CONSTRAINT implementation.
//!
//! It does NOT:
//!
//! - parse Zamani source;
//! - define quantum measurement semantics;
//! - define another QuantumCircuit;
//! - define another QuantumOperation;
//! - define another QubitId;
//! - define another PhysicalQubitId;
//! - discover hardware;
//! - query a QPU;
//! - perform measurement;
//! - generate measurement probabilities;
//! - perform QEC decoding;
//! - perform routing;
//! - choose a scheduling algorithm;
//! - own a resource calendar;
//! - reserve hardware resources;
//! - communicate with a backend.
//!
//! Those responsibilities remain in the canonical IR, hardware, routing,
//! planning, runtime, simulator, and QEC subsystems.
//!
//! ============================================================================
//! CANONICAL BOUNDARIES
//! ============================================================================
//!
//! Quantum qubit identity comes from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Scheduler operation/resource identity comes from the scheduler/repository
//! identity model already used by `constraints::constraint`.
//!
//! Canonical measurement semantics remain owned by:
//!
//! ```text
//! crate::quantum::ir::quantum::measurement
//! ```
//!
//! This module only consumes scheduling-facing information.
//!
//! ============================================================================
//! WHY THIS MODULE IS SEPARATE
//! ============================================================================
//!
//! Measurement scheduling is substantially more complicated than merely
//! checking whether a qubit is busy.
//!
//! A target may impose constraints on:
//!
//! - readout resource capacity;
//! - simultaneous measurement capacity;
//! - physical qubit occupancy;
//! - measurement channel sharing;
//! - resource exclusivity;
//! - measurement overlap;
//! - measurement duration;
//! - measurement windows;
//! - destructive measurement;
//! - non-destructive measurement;
//! - reset-after-measurement;
//! - classical result readiness;
//! - measurement grouping;
//! - joint measurement resources;
//! - custom measurement resource requirements.
//!
//! The actual physical interpretation of those resources belongs to the target
//! adapter. This module only enforces the supplied abstract constraints.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program must not encode a machine's number of readout channels,
//! maximum simultaneous measurements, qubit count, or measurement duration.
//!
//! Therefore this module contains NO constants such as:
//!
//! ```text
//! MAX_MEASUREMENTS
//! MAX_READOUT_CHANNELS
//! MAX_QUBITS
//! MEASUREMENT_DURATION_NS
//! MAX_SIMULTANEOUS_READOUT
//! ```
//!
//! Target-dependent values are supplied explicitly when constructing the
//! constraint.
//!
//! "Infinity" means that this implementation contains no artificial finite
//! machine-size ceiling. Actual compilation remains bounded by available
//! memory, CPU time, explicit compiler limits, and target resources.
//!
//! ============================================================================
//! MEASUREMENT SEMANTICS BOUNDARY
//! ============================================================================
//!
//! The canonical IR already supports measurement semantics substantially more
//! broadly than a single-qubit Z-basis operation.
//!
//! The canonical measurement model includes concepts such as:
//!
//! - projective measurement;
//! - X/Y/Z observables;
//! - generalized measurement;
//! - weak measurement;
//! - continuous measurement;
//! - joint Pauli-product measurement;
//! - destructive measurement;
//! - non-destructive measurement;
//! - reset-after-measurement intent.
//!
//! This scheduler constraint does not reinterpret those semantics.
//!
//! Instead, an upstream adapter converts the canonical measurement operation
//! into a scheduling descriptor.
//!
//! This keeps:
//!
//! ```text
//! quantum::ir::quantum::measurement
//!             │
//!             ▼
//! scheduling adapter
//!             │
//!             ▼
//! MeasurementDescriptor
//!             │
//!             ▼
//! MeasurementConstraint
//! ```
//!
//! The same scheduler can therefore support future measurement semantics
//! without modifying this file merely because a new semantic observable is
//! added to the canonical IR.
//!
//! ============================================================================
//! RESOURCE MODEL
//! ============================================================================
//!
//! A measurement may consume arbitrary scheduler resources.
//!
//! Examples:
//!
//! - readout channel;
//! - resonator;
//! - detector;
//! - optical collection path;
//! - control/readout electronics;
//! - classical result-processing capacity;
//! - communication resource;
//! - custom target-defined resource.
//!
//! Resource identity is supplied through `ConstraintResourceClaim`.
//!
//! Capacity and resource ownership remain outside this module.
//!
//! ============================================================================
//! IMPORTANT LIMITATION OF GENERIC CANDIDATE
//! ============================================================================
//!
//! The generic `SchedulingCandidate` intentionally does not contain a canonical
//! quantum operation object. That is the correct architectural boundary.
//!
//! Consequently this constraint does not attempt to inspect a private or
//! competing operation representation.
//!
//! Instead the adapter supplies `MeasurementDescriptor` metadata for the
//! measurement candidate.
//!
//! This avoids coupling the generic scheduler to the exact current shape of
//! the quantum IR.
//!
//! ============================================================================
//! THREAD SAFETY
//! ============================================================================
//!
//! The constraint is immutable after construction and implements the generic
//! `Constraint` contract, which requires `Send + Sync`.
//!
//! No interior mutable state is used.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! All collections exposed by this module preserve deterministic ordering.
//!
//! No hash-map iteration is used for decisions.
//!
//! No randomness is used.
//!
//! No wall-clock time is consulted.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! Stable Rust.
//! No nightly features.
//! No unsafe.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::constraint::{
    Constraint,
    ConstraintApplicability,
    ConstraintContext,
    ConstraintId,
    ConstraintKind,
    ConstraintPhase,
    ConstraintResourceClaim,
    ConstraintSeverity,
    ConstraintViolation,
};

// ============================================================================
// Measurement operation classification
// ============================================================================

/// Scheduling-level classification of a measurement operation.
///
/// This is NOT a replacement for the canonical quantum measurement semantic
/// model. It only tells the scheduler what scheduling behaviour is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MeasurementSchedulingKind {
    /// Ordinary projective measurement.
    Projective,

    /// Generalized/POVM measurement.
    Generalized,

    /// Weak measurement.
    Weak,

    /// Continuous measurement.
    Continuous,

    /// Joint or parity-style measurement.
    Joint,
}

impl MeasurementSchedulingKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Projective => "projective",
            Self::Generalized => "generalized",
            Self::Weak => "weak",
            Self::Continuous => "continuous",
            Self::Joint => "joint",
        }
    }
}

impl fmt::Display for MeasurementSchedulingKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Measurement result mode
// ============================================================================

/// Describes how a measurement affects the measured quantum state from the
/// scheduler's perspective.
///
/// This does not implement the quantum collapse semantics. It only identifies
/// whether the scheduler must preserve the possibility of subsequent use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MeasurementStateMode {
    /// The measured state remains logically usable after measurement.
    NonDestructive,

    /// The measured state is consumed by the measurement.
    Destructive,
}

impl MeasurementStateMode {
    /// Returns whether subsequent operations may consume the measured qubit
    /// without an explicit reinitialization/reset dependency.
    #[must_use]
    pub const fn is_non_destructive(self) -> bool {
        matches!(self, Self::NonDestructive)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonDestructive => "non_destructive",
            Self::Destructive => "destructive",
        }
    }
}

// ============================================================================
// Classical result behaviour
// ============================================================================

/// Describes scheduler-visible classical result behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MeasurementResultMode {
    /// A classical result becomes available after measurement completion.
    Deferred,

    /// The target exposes a result at measurement completion with no additional
    /// scheduler-visible processing delay.
    Immediate,

    /// A target-defined processing stage is required before consumers may
    /// proceed.
    ProcessingRequired,
}

impl MeasurementResultMode {
    /// Returns whether an explicit result-processing stage is expected.
    #[must_use]
    pub const fn requires_processing(self) -> bool {
        matches!(self, Self::ProcessingRequired)
    }
}

// ============================================================================
// Measurement descriptor
// ============================================================================

/// Scheduling-facing description of one measurement operation.
///
/// This structure is deliberately separate from canonical quantum IR.
///
/// The upstream IR adapter is responsible for constructing it from the
/// canonical measurement operation and target information.
///
/// No hardware-specific names or physical timing units are embedded here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementDescriptor {
    /// Logical qubits involved in the measurement.
    logical_qubits: Vec<QubitId>,

    /// Physical qubits involved after routing, if known.
    physical_qubits: Vec<PhysicalQubitId>,

    /// Semantic scheduling class.
    kind: MeasurementSchedulingKind,

    /// Whether measurement consumes the quantum state.
    state_mode: MeasurementStateMode,

    /// How the classical result becomes available.
    result_mode: MeasurementResultMode,

    /// Whether the measurement requests a reset/reinitialization phase.
    reset_after: bool,

    /// Resource claims needed for measurement.
    resource_claims: Vec<ConstraintResourceClaim>,
}

impl MeasurementDescriptor {
    /// Creates a measurement descriptor.
    ///
    /// Validation is intentionally separate so adapters can construct the
    /// descriptor incrementally and then validate it before scheduling.
    #[must_use]
    pub fn new(
        logical_qubits: Vec<QubitId>,
        physical_qubits: Vec<PhysicalQubitId>,
        kind: MeasurementSchedulingKind,
        state_mode: MeasurementStateMode,
        result_mode: MeasurementResultMode,
        reset_after: bool,
        resource_claims: Vec<ConstraintResourceClaim>,
    ) -> Self {
        Self {
            logical_qubits,
            physical_qubits,
            kind,
            state_mode,
            result_mode,
            reset_after,
            resource_claims,
        }
    }

    /// Returns the logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns the physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns the scheduling kind.
    #[must_use]
    pub const fn kind(&self) -> MeasurementSchedulingKind {
        self.kind
    }

    /// Returns the state behaviour.
    #[must_use]
    pub const fn state_mode(&self) -> MeasurementStateMode {
        self.state_mode
    }

    /// Returns the classical-result behaviour.
    #[must_use]
    pub const fn result_mode(&self) -> MeasurementResultMode {
        self.result_mode
    }

    /// Returns whether reset-after-measurement is requested.
    #[must_use]
    pub const fn reset_after(&self) -> bool {
        self.reset_after
    }

    /// Returns resource claims.
    #[must_use]
    pub fn resource_claims(&self) -> &[ConstraintResourceClaim] {
        &self.resource_claims
    }

    /// Returns the number of logical qubits involved.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the number of physical qubits involved.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Returns whether this measurement has no logical targets.
    #[must_use]
    pub fn has_no_logical_targets(&self) -> bool {
        self.logical_qubits.is_empty()
    }

    /// Returns whether this measurement has physical targets.
    #[must_use]
    pub fn has_physical_targets(&self) -> bool {
        !self.physical_qubits.is_empty()
    }

    /// Validates internal consistency.
    ///
    /// This validation does not validate qubit namespace bounds because the
    /// descriptor intentionally does not own the complete quantum program.
    pub fn validate(&self) -> Result<(), MeasurementDescriptorError> {
        if self.logical_qubits.is_empty() {
            return Err(
                MeasurementDescriptorError::NoLogicalTargets,
            );
        }

        if self
            .logical_qubits
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(
                MeasurementDescriptorError::DuplicateLogicalQubit,
            );
        }

        if self
            .physical_qubits
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(
                MeasurementDescriptorError::DuplicatePhysicalQubit,
            );
        }

        let mut resources = BTreeSet::new();

        for claim in &self.resource_claims {
            if claim.is_zero() {
                continue;
            }

            if !resources.insert(claim.resource()) {
                return Err(
                    MeasurementDescriptorError::DuplicateResourceClaim {
                        resource: claim.resource(),
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for MeasurementDescriptor {
    fn default() -> Self {
        Self::new(
            Vec::new(),
            Vec::new(),
            MeasurementSchedulingKind::Projective,
            MeasurementStateMode::NonDestructive,
            MeasurementResultMode::Deferred,
            false,
            Vec::new(),
        )
    }
}

// ============================================================================
// Descriptor errors
// ============================================================================

/// Errors produced while validating a scheduling measurement descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MeasurementDescriptorError {
    /// The measurement contains no logical target.
    NoLogicalTargets,

    /// A logical qubit occurs more than once.
    DuplicateLogicalQubit,

    /// A physical qubit occurs more than once.
    DuplicatePhysicalQubit,

    /// A resource is claimed more than once.
    DuplicateResourceClaim {
        /// Duplicated resource.
        resource: crate::quantum::ir::core::identity::ResourceId,
    },
}

impl fmt::Display for MeasurementDescriptorError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NoLogicalTargets => {
                formatter.write_str(
                    "measurement has no logical targets",
                )
            }

            Self::DuplicateLogicalQubit => {
                formatter.write_str(
                    "measurement contains a duplicate logical qubit",
                )
            }

            Self::DuplicatePhysicalQubit => {
                formatter.write_str(
                    "measurement contains a duplicate physical qubit",
                )
            }

            Self::DuplicateResourceClaim { resource } => {
                write!(
                    formatter,
                    "measurement claims resource {} more than once",
                    resource
                )
            }
        }
    }
}

impl std::error::Error for MeasurementDescriptorError {}

// ============================================================================
// Measurement constraint configuration
// ============================================================================

/// Configuration for the measurement scheduling constraint.
///
/// All target-specific values are supplied by the caller.
///
/// There are deliberately no machine-size constants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementConstraintConfig {
    /// Maximum number of logical qubits that one measurement operation may
    /// explicitly target.
    ///
    /// `None` means unrestricted by this constraint.
    max_qubits_per_measurement: Option<usize>,

    /// Maximum number of physical qubits that one measurement operation may
    /// explicitly target.
    ///
    /// `None` means unrestricted by this constraint.
    max_physical_qubits_per_measurement: Option<usize>,

    /// Whether physical qubit identities are required after routing.
    require_physical_targets: bool,

    /// Whether at least one scheduler resource claim is required.
    require_resource_claim: bool,

    /// Whether destructive measurement is permitted.
    allow_destructive: bool,

    /// Whether continuous measurement is permitted.
    allow_continuous: bool,

    /// Whether generalized measurement is permitted.
    allow_generalized: bool,

    /// Whether joint measurement is permitted.
    allow_joint: bool,

    /// Whether reset-after-measurement intent is permitted.
    allow_reset_after: bool,

    /// Whether a classical result must be produced.
    require_classical_result: bool,
}

impl Default for MeasurementConstraintConfig {
    fn default() -> Self {
        Self {
            max_qubits_per_measurement: None,
            max_physical_qubits_per_measurement: None,
            require_physical_targets: false,
            require_resource_claim: false,
            allow_destructive: true,
            allow_continuous: true,
            allow_generalized: true,
            allow_joint: true,
            allow_reset_after: true,
            require_classical_result: true,
        }
    }
}

impl MeasurementConstraintConfig {
    /// Creates a configuration with unrestricted semantic limits.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self::default()
    }

    /// Sets the maximum logical measurement arity.
    #[must_use]
    pub const fn with_max_qubits(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_qubits_per_measurement = maximum;
        self
    }

    /// Sets the maximum physical measurement arity.
    #[must_use]
    pub const fn with_max_physical_qubits(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_physical_qubits_per_measurement = maximum;
        self
    }

    /// Requires routed physical targets.
    #[must_use]
    pub const fn requiring_physical_targets(
        mut self,
        required: bool,
    ) -> Self {
        self.require_physical_targets = required;
        self
    }

    /// Requires at least one measurement resource claim.
    #[must_use]
    pub const fn requiring_resource_claim(
        mut self,
        required: bool,
    ) -> Self {
        self.require_resource_claim = required;
        self
    }

    /// Enables or disables destructive measurements.
    #[must_use]
    pub const fn allowing_destructive(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_destructive = allowed;
        self
    }

    /// Enables or disables continuous measurements.
    #[must_use]
    pub const fn allowing_continuous(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_continuous = allowed;
        self
    }

    /// Enables or disables generalized measurements.
    #[must_use]
    pub const fn allowing_generalized(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_generalized = allowed;
        self
    }

    /// Enables or disables joint measurements.
    #[must_use]
    pub const fn allowing_joint(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_joint = allowed;
        self
    }

    /// Enables or disables reset-after-measurement intent.
    #[must_use]
    pub const fn allowing_reset_after(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_reset_after = allowed;
        self
    }

    /// Requires a classical result.
    #[must_use]
    pub const fn requiring_classical_result(
        mut self,
        required: bool,
    ) -> Self {
        self.require_classical_result = required;
        self
    }

    /// Returns the logical-arity limit.
    #[must_use]
    pub const fn max_qubits_per_measurement(
        &self,
    ) -> Option<usize> {
        self.max_qubits_per_measurement
    }

    /// Returns the physical-arity limit.
    #[must_use]
    pub const fn max_physical_qubits_per_measurement(
        &self,
    ) -> Option<usize> {
        self.max_physical_qubits_per_measurement
    }

    /// Returns whether physical targets are required.
    #[must_use]
    pub const fn require_physical_targets(
        &self,
    ) -> bool {
        self.require_physical_targets
    }

    /// Returns whether a resource claim is required.
    #[must_use]
    pub const fn require_resource_claim(
        &self,
    ) -> bool {
        self.require_resource_claim
    }

    /// Returns whether destructive measurement is allowed.
    #[must_use]
    pub const fn allow_destructive(&self) -> bool {
        self.allow_destructive
    }

    /// Returns whether continuous measurement is allowed.
    #[must_use]
    pub const fn allow_continuous(&self) -> bool {
        self.allow_continuous
    }

    /// Returns whether generalized measurement is allowed.
    #[must_use]
    pub const fn allow_generalized(&self) -> bool {
        self.allow_generalized
    }

    /// Returns whether joint measurement is allowed.
    #[must_use]
    pub const fn allow_joint(&self) -> bool {
        self.allow_joint
    }

    /// Returns whether reset-after-measurement is allowed.
    #[must_use]
    pub const fn allow_reset_after(&self) -> bool {
        self.allow_reset_after
    }

    /// Returns whether a classical result is required.
    #[must_use]
    pub const fn require_classical_result(&self) -> bool {
        self.require_classical_result
    }
}

// ============================================================================
// Measurement constraint
// ============================================================================

/// Production measurement/readout constraint.
///
/// The constraint is immutable and can safely be shared between scheduler
/// threads.
///
/// Measurement descriptors are registered by operation identity so the
/// generic `ConstraintContext` remains independent of canonical quantum IR
/// layout.
#[derive(Debug, Clone)]
pub struct MeasurementConstraint {
    id: ConstraintId,
    name: String,
    severity: ConstraintSeverity,
    config: MeasurementConstraintConfig,

    /// Operation IDs known to be measurement operations.
    ///
    /// A `BTreeSet` provides deterministic lookup and iteration.
    measurement_operations:
        BTreeSet<crate::quantum::ir::core::identity::OperationId>,

    /// Per-operation scheduling descriptors.
    descriptors:
        std::collections::BTreeMap<
            crate::quantum::ir::core::identity::OperationId,
            MeasurementDescriptor,
        >,
}

impl MeasurementConstraint {
    /// Creates an empty measurement constraint.
    ///
    /// Descriptors may be registered before scheduling starts.
    #[must_use]
    pub fn new(
        id: ConstraintId,
        config: MeasurementConstraintConfig,
    ) -> Self {
        Self {
            id,
            name: String::from("measurement"),
            severity: ConstraintSeverity::Error,
            config,
            measurement_operations: BTreeSet::new(),
            descriptors: std::collections::BTreeMap::new(),
        }
    }

    /// Creates an empty measurement constraint with the standard production
    /// severity.
    #[must_use]
    pub fn production(id: ConstraintId) -> Self {
        Self::new(
            id,
            MeasurementConstraintConfig::default(),
        )
    }

    /// Sets the diagnostic name.
    ///
    /// The name has no semantic or hardware meaning.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) {
        self.name = name.into();
    }

    /// Sets the violation severity.
    pub fn set_severity(
        &mut self,
        severity: ConstraintSeverity,
    ) {
        self.severity = severity;
    }

    /// Returns the configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> &MeasurementConstraintConfig {
        &self.config
    }

    /// Registers a measurement descriptor.
    ///
    /// The operation identity comes from the canonical repository identity
    /// model; this module does not allocate it.
    pub fn register(
        &mut self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        descriptor: MeasurementDescriptor,
    ) -> Result<(), MeasurementRegistrationError> {
        descriptor
            .validate()
            .map_err(
                MeasurementRegistrationError::InvalidDescriptor,
            )?;

        if self
            .measurement_operations
            .contains(&operation)
        {
            return Err(
                MeasurementRegistrationError::DuplicateOperation {
                    operation,
                },
            );
        }

        self.measurement_operations.insert(operation);
        self.descriptors.insert(operation, descriptor);

        Ok(())
    }

    /// Registers or replaces a descriptor for an operation.
    ///
    /// This is useful for incremental/dynamic compilation where an operation's
    /// target mapping becomes known later.
    pub fn upsert(
        &mut self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        descriptor: MeasurementDescriptor,
    ) -> Result<(), MeasurementRegistrationError> {
        descriptor
            .validate()
            .map_err(
                MeasurementRegistrationError::InvalidDescriptor,
            )?;

        self.measurement_operations.insert(operation);
        self.descriptors.insert(operation, descriptor);

        Ok(())
    }

    /// Returns a descriptor for an operation.
    #[must_use]
    pub fn descriptor(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
    ) -> Option<&MeasurementDescriptor> {
        self.descriptors.get(&operation)
    }

    /// Returns whether an operation is registered as a measurement.
    #[must_use]
    pub fn contains(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
    ) -> bool {
        self.measurement_operations
            .contains(&operation)
    }

    /// Returns the number of registered measurement operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.measurement_operations.len()
    }

    /// Returns whether no measurement operations are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.measurement_operations.is_empty()
    }

    /// Validates a measurement descriptor independently of a scheduler
    /// context.
    pub fn validate_descriptor(
        &self,
        descriptor: &MeasurementDescriptor,
    ) -> Result<(), MeasurementConstraintViolation> {
        descriptor.validate().map_err(
            MeasurementConstraintViolation::InvalidDescriptor,
        )?;

        if let Some(maximum) =
            self.config.max_qubits_per_measurement
        {
            if descriptor.logical_qubit_count() > maximum {
                return Err(
                    MeasurementConstraintViolation::LogicalArityExceeded {
                        actual: descriptor.logical_qubit_count(),
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) =
            self.config.max_physical_qubits_per_measurement
        {
            if descriptor.physical_qubit_count() > maximum {
                return Err(
                    MeasurementConstraintViolation::PhysicalArityExceeded {
                        actual: descriptor.physical_qubit_count(),
                        maximum,
                    },
                );
            }
        }

        if self.config.require_physical_targets
            && !descriptor.has_physical_targets()
        {
            return Err(
                MeasurementConstraintViolation::PhysicalTargetsRequired,
            );
        }

        if self.config.require_resource_claim
            && !descriptor
                .resource_claims()
                .iter()
                .any(|claim| !claim.is_zero())
        {
            return Err(
                MeasurementConstraintViolation::ResourceClaimRequired,
            );
        }

        if descriptor.state_mode
            == MeasurementStateMode::Destructive
            && !self.config.allow_destructive
        {
            return Err(
                MeasurementConstraintViolation::DestructiveMeasurementNotAllowed,
            );
        }

        if descriptor.kind
            == MeasurementSchedulingKind::Continuous
            && !self.config.allow_continuous
        {
            return Err(
                MeasurementConstraintViolation::ContinuousMeasurementNotAllowed,
            );
        }

        if descriptor.kind
            == MeasurementSchedulingKind::Generalized
            && !self.config.allow_generalized
        {
            return Err(
                MeasurementConstraintViolation::GeneralizedMeasurementNotAllowed,
            );
        }

        if descriptor.kind
            == MeasurementSchedulingKind::Joint
            && !self.config.allow_joint
        {
            return Err(
                MeasurementConstraintViolation::JointMeasurementNotAllowed,
            );
        }

        if descriptor.reset_after
            && !self.config.allow_reset_after
        {
            return Err(
                MeasurementConstraintViolation::ResetAfterMeasurementNotAllowed,
            );
        }

        if self.config.require_classical_result
            && matches!(
                descriptor.result_mode,
                MeasurementResultMode::Deferred
                    | MeasurementResultMode::Immediate
                    | MeasurementResultMode::ProcessingRequired
            )
        {
            // All current result modes represent a classical result.
            // This branch intentionally documents the invariant while keeping
            // future result modes extensible.
        }

        Ok(())
    }

    /// Evaluates a registered measurement operation directly.
    ///
    /// This helper is useful for adapters and tests that have a descriptor but
    /// do not yet have a complete scheduler candidate.
    pub fn validate_operation(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
    ) -> Result<(), MeasurementConstraintViolation> {
        let descriptor =
            self.descriptor(operation)
                .ok_or(
                    MeasurementConstraintViolation::UnregisteredMeasurement {
                        operation,
                    },
                )?;

        self.validate_descriptor(descriptor)
    }

    /// Returns the logical qubits of a registered measurement.
    #[must_use]
    pub fn logical_qubits(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
    ) -> Option<&[QubitId]> {
        self.descriptor(operation)
            .map(MeasurementDescriptor::logical_qubits)
    }

    /// Returns the physical qubits of a registered measurement.
    #[must_use]
    pub fn physical_qubits(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
    ) -> Option<&[PhysicalQubitId]> {
        self.descriptor(operation)
            .map(MeasurementDescriptor::physical_qubits)
    }

    /// Checks for overlap with already scheduled measurement reservations.
    ///
    /// This method deliberately uses only the generic immutable scheduling
    /// state. Resource capacity remains the responsibility of the generic
    /// resource subsystem.
    fn validate_qubit_overlap(
        &self,
        context: &ConstraintContext<'_>,
        descriptor: &MeasurementDescriptor,
    ) -> Result<(), MeasurementConstraintViolation> {
        let candidate = context.candidate();

        for reservation in context.state().reservations() {
            if reservation.operation() == Some(candidate.operation()) {
                continue;
            }

            let Some(reservation_end) =
                reservation.checked_end()
            else {
                continue;
            };

            let Some(candidate_end) =
                candidate.checked_end()
            else {
                return Err(
                    MeasurementConstraintViolation::InvalidCandidateTime,
                );
            };

            if candidate.start() >= reservation_end
                || reservation.start() >= candidate_end
            {
                continue;
            }

            // The generic reservation view identifies resources, but it does
            // not identify qubits. Therefore resource overlap is handled by
            // resource claims and the generic resource constraint.
            //
            // We intentionally do not infer a qubit conflict from an unrelated
            // resource ID. Such inference would be unsafe and target-specific.
            let _ = descriptor;
        }

        Ok(())
    }

    /// Converts an internal violation into the generic scheduler violation.
    fn to_constraint_violation(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        violation: MeasurementConstraintViolation,
        context: Option<&ConstraintContext<'_>>,
    ) -> ConstraintViolation {
        let reason = violation.to_string();

        let mut result = ConstraintViolation::new(
            self.id,
            ConstraintKind::Measurement,
            self.severity,
            reason,
        )
        .with_operation(operation);

        if let Some(context) = context {
            result = result.with_timing(
                context.candidate().start(),
                context.candidate().duration(),
            );

            if let Some(qubit) =
                context.candidate().logical_qubits().first()
            {
                result =
                    result.with_logical_qubit(*qubit);
            }

            if let Some(qubit) =
                context.candidate().physical_qubits().first()
            {
                result =
                    result.with_physical_qubit(*qubit);
            }

            if let Some(claim) =
                context.candidate().resource_claims().first()
            {
                result =
                    result.with_resource(claim.resource());
            }
        }

        result
    }
}

impl Constraint for MeasurementConstraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Measurement
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    fn applies(
        &self,
        context: &ConstraintContext<'_>,
    ) -> ConstraintApplicability {
        if self.contains(context.candidate().operation()) {
            ConstraintApplicability::Applicable
        } else {
            ConstraintApplicability::NotApplicable
        }
    }

    fn supports_phase(
        &self,
        phase: ConstraintPhase,
    ) -> bool {
        matches!(
            phase,
            ConstraintPhase::Planning
                | ConstraintPhase::PreCommit
                | ConstraintPhase::PostCommit
                | ConstraintPhase::Verification
                | ConstraintPhase::Runtime
        )
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        let operation = context.candidate().operation();

        let Some(descriptor) =
            self.descriptor(operation)
        else {
            return Err(
                self.to_constraint_violation(
                    operation,
                    MeasurementConstraintViolation::
                        UnregisteredMeasurement {
                            operation,
                        },
                    Some(context),
                ),
            );
        };

        if let Err(violation) =
            self.validate_descriptor(descriptor)
        {
            return Err(
                self.to_constraint_violation(
                    operation,
                    violation,
                    Some(context),
                ),
            );
        }

        if context
            .candidate()
            .duration()
            .is_zero()
            && descriptor.kind
                != MeasurementSchedulingKind::Continuous
        {
            return Err(
                self.to_constraint_violation(
                    operation,
                    MeasurementConstraintViolation::
                        ZeroDurationMeasurement,
                    Some(context),
                ),
            );
        }

        if descriptor.result_mode
            == MeasurementResultMode::ProcessingRequired
            && context.phase()
                == ConstraintPhase::PostCommit
        {
            // The actual classical processing duration is intentionally not
            // invented here. A dedicated control/feedback constraint must
            // model that target-specific latency.
        }

        if let Err(violation) =
            self.validate_qubit_overlap(
                context,
                descriptor,
            )
        {
            return Err(
                self.to_constraint_violation(
                    operation,
                    violation,
                    Some(context),
                ),
            );
        }

        for claim in context
            .candidate()
            .resource_claims()
        {
            if context
                .state()
                .is_resource_unavailable(
                    claim.resource(),
                )
            {
                return Err(
                    self.to_constraint_violation(
                        operation,
                        MeasurementConstraintViolation::
                            ResourceUnavailable {
                                resource: claim.resource(),
                            },
                        Some(context),
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Registration errors
// ============================================================================

/// Errors encountered while registering measurement operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MeasurementRegistrationError {
    /// The operation is already registered.
    DuplicateOperation {
        /// Duplicate operation identity.
        operation:
            crate::quantum::ir::core::identity::OperationId,
    },

    /// The supplied descriptor is invalid.
    InvalidDescriptor(
        MeasurementDescriptorError,
    ),
}

impl fmt::Display for MeasurementRegistrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateOperation {
                operation,
            } => {
                write!(
                    formatter,
                    "measurement operation {} is already registered",
                    operation
                )
            }

            Self::InvalidDescriptor(error) => {
                write!(
                    formatter,
                    "invalid measurement descriptor: {}",
                    error
                )
            }
        }
    }
}

impl std::error::Error for MeasurementRegistrationError {}

// ============================================================================
// Measurement violations
// ============================================================================

/// Specialized measurement constraint failures.
///
/// These remain independent of the scheduler's higher-level error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MeasurementConstraintViolation {
    /// Descriptor is structurally invalid.
    InvalidDescriptor(
        MeasurementDescriptorError,
    ),

    /// Operation was expected to be a measurement but has no descriptor.
    UnregisteredMeasurement {
        /// Operation identity.
        operation:
            crate::quantum::ir::core::identity::OperationId,
    },

    /// Logical measurement arity exceeds the configured target constraint.
    LogicalArityExceeded {
        /// Actual number of logical targets.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Physical measurement arity exceeds the configured target constraint.
    PhysicalArityExceeded {
        /// Actual number of physical targets.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The target requires physical targets but none are available.
    PhysicalTargetsRequired,

    /// The target requires a scheduler resource claim.
    ResourceClaimRequired,

    /// Destructive measurement is not permitted.
    DestructiveMeasurementNotAllowed,

    /// Continuous measurement is not permitted.
    ContinuousMeasurementNotAllowed,

    /// Generalized measurement is not permitted.
    GeneralizedMeasurementNotAllowed,

    /// Joint measurement is not permitted.
    JointMeasurementNotAllowed,

    /// Reset-after-measurement is not permitted.
    ResetAfterMeasurementNotAllowed,

    /// Measurement duration is zero when a real measurement interval is
    /// required.
    ZeroDurationMeasurement,

    /// Candidate time arithmetic failed.
    InvalidCandidateTime,

    /// A requested measurement resource is currently unavailable.
    ResourceUnavailable {
        /// Resource that cannot currently be used.
        resource:
            crate::quantum::ir::core::identity::ResourceId,
    },
}

impl fmt::Display for MeasurementConstraintViolation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDescriptor(error) => {
                write!(
                    formatter,
                    "invalid measurement descriptor: {}",
                    error
                )
            }

            Self::UnregisteredMeasurement {
                operation,
            } => {
                write!(
                    formatter,
                    "measurement operation {} has no scheduling descriptor",
                    operation
                )
            }

            Self::LogicalArityExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "measurement logical arity {} exceeds configured maximum {}",
                    actual,
                    maximum
                )
            }

            Self::PhysicalArityExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "measurement physical arity {} exceeds configured maximum {}",
                    actual,
                    maximum
                )
            }

            Self::PhysicalTargetsRequired => {
                formatter.write_str(
                    "measurement requires physical targets",
                )
            }

            Self::ResourceClaimRequired => {
                formatter.write_str(
                    "measurement requires at least one resource claim",
                )
            }

            Self::DestructiveMeasurementNotAllowed => {
                formatter.write_str(
                    "destructive measurement is not allowed by the scheduling policy",
                )
            }

            Self::ContinuousMeasurementNotAllowed => {
                formatter.write_str(
                    "continuous measurement is not allowed by the scheduling policy",
                )
            }

            Self::GeneralizedMeasurementNotAllowed => {
                formatter.write_str(
                    "generalized measurement is not allowed by the scheduling policy",
                )
            }

            Self::JointMeasurementNotAllowed => {
                formatter.write_str(
                    "joint measurement is not allowed by the scheduling policy",
                )
            }

            Self::ResetAfterMeasurementNotAllowed => {
                formatter.write_str(
                    "reset-after-measurement is not allowed by the scheduling policy",
                )
            }

            Self::ZeroDurationMeasurement => {
                formatter.write_str(
                    "measurement has zero duration",
                )
            }

            Self::InvalidCandidateTime => {
                formatter.write_str(
                    "measurement candidate has an unrepresentable end time",
                )
            }

            Self::ResourceUnavailable {
                resource,
            } => {
                write!(
                    formatter,
                    "measurement resource {} is unavailable",
                    resource
                )
            }
        }
    }
}

impl std::error::Error for MeasurementConstraintViolation {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::{
        OperationId,
        ResourceId,
    };

    use crate::quantum::scheduling::types::{
        Duration,
        ReservationId,
        TimePoint,
    };

    use super::super::constraint::{
        ConstraintContext,
        ConstraintPhase,
        ConstraintState,
        SchedulingCandidate,
    };

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn physical_q(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn descriptor() -> MeasurementDescriptor {
        MeasurementDescriptor::new(
            vec![q(0)],
            vec![physical_q(0)],
            MeasurementSchedulingKind::Projective,
            MeasurementStateMode::NonDestructive,
            MeasurementResultMode::Deferred,
            false,
            vec![
                ConstraintResourceClaim::new(
                    resource(1),
                    1,
                ),
            ],
        )
    }

    #[test]
    fn descriptor_accepts_single_target() {
        assert!(descriptor().validate().is_ok());
    }

    #[test]
    fn descriptor_rejects_duplicate_logical_qubit() {
        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0), q(0)],
                vec![physical_q(0)],
                MeasurementSchedulingKind::Projective,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert_eq!(
            descriptor.validate(),
            Err(
                MeasurementDescriptorError::
                    DuplicateLogicalQubit
            )
        );
    }

    #[test]
    fn descriptor_rejects_duplicate_physical_qubit() {
        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0), q(1)],
                vec![physical_q(0), physical_q(0)],
                MeasurementSchedulingKind::Joint,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert_eq!(
            descriptor.validate(),
            Err(
                MeasurementDescriptorError::
                    DuplicatePhysicalQubit
            )
        );
    }

    #[test]
    fn configuration_can_be_unrestricted() {
        let config =
            MeasurementConstraintConfig::unrestricted();

        assert_eq!(
            config.max_qubits_per_measurement(),
            None
        );
        assert!(config.allow_destructive());
        assert!(config.allow_continuous());
        assert!(config.allow_generalized());
        assert!(config.allow_joint());
    }

    #[test]
    fn configuration_can_impose_explicit_target_limit() {
        let config =
            MeasurementConstraintConfig::default()
                .with_max_qubits(Some(1));

        let constraint =
            MeasurementConstraint::new(
                ConstraintId::new(1),
                config,
            );

        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0), q(1)],
                vec![
                    physical_q(0),
                    physical_q(1),
                ],
                MeasurementSchedulingKind::Joint,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert_eq!(
            constraint
                .validate_descriptor(&descriptor),
            Err(
                MeasurementConstraintViolation::
                    LogicalArityExceeded {
                        actual: 2,
                        maximum: 1,
                    }
            )
        );
    }

    #[test]
    fn destructive_measurement_can_be_rejected() {
        let config =
            MeasurementConstraintConfig::default()
                .allowing_destructive(false);

        let constraint =
            MeasurementConstraint::new(
                ConstraintId::new(2),
                config,
            );

        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0)],
                vec![physical_q(0)],
                MeasurementSchedulingKind::Projective,
                MeasurementStateMode::Destructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert_eq!(
            constraint
                .validate_descriptor(&descriptor),
            Err(
                MeasurementConstraintViolation::
                    DestructiveMeasurementNotAllowed
            )
        );
    }

    #[test]
    fn continuous_measurement_can_be_rejected() {
        let config =
            MeasurementConstraintConfig::default()
                .allowing_continuous(false);

        let constraint =
            MeasurementConstraint::new(
                ConstraintId::new(3),
                config,
            );

        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0)],
                vec![physical_q(0)],
                MeasurementSchedulingKind::Continuous,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert_eq!(
            constraint
                .validate_descriptor(&descriptor),
            Err(
                MeasurementConstraintViolation::
                    ContinuousMeasurementNotAllowed
            )
        );
    }

    #[test]
    fn registration_is_deterministic() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(4),
            );

        constraint
            .register(operation(7), descriptor())
            .expect("registration must succeed");

        assert!(constraint.contains(operation(7)));
        assert_eq!(constraint.len(), 1);
        assert!(constraint.descriptor(operation(7)).is_some());
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(5),
            );

        constraint
            .register(operation(7), descriptor())
            .expect("first registration must succeed");

        assert!(matches!(
            constraint.register(
                operation(7),
                descriptor(),
            ),
            Err(
                MeasurementRegistrationError::
                    DuplicateOperation { .. }
            )
        ));
    }

    #[test]
    fn candidate_evaluation_succeeds_for_valid_measurement() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(6),
            );

        let op = operation(10);

        constraint
            .register(op, descriptor())
            .expect("registration must succeed");

        let logical =
            [q(0)];

        let physical =
            [physical_q(0)];

        let claims = [
            ConstraintResourceClaim::new(
                resource(1),
                1,
            ),
        ];

        let candidate =
            SchedulingCandidate::new(
                op,
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(1),
            );

        let reservations: [ConstraintReservationView; 0] =
            [];

        let completed: [OperationId; 0] = [];

        let unavailable: [ResourceId; 0] = [];

        let state =
            ConstraintState::new(
                &reservations,
                &completed,
                &unavailable,
            );

        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        assert!(
            constraint.evaluate(&context).is_ok()
        );
    }

    #[test]
    fn zero_duration_projective_measurement_is_rejected() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(7),
            );

        let op = operation(11);

        constraint
            .register(op, descriptor())
            .expect("registration must succeed");

        let logical =
            [q(0)];

        let physical =
            [physical_q(0)];

        let claims = [
            ConstraintResourceClaim::new(
                resource(1),
                1,
            ),
        ];

        let candidate =
            SchedulingCandidate::new(
                op,
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(0),
            );

        let reservations: [ConstraintReservationView; 0] =
            [];

        let completed: [OperationId; 0] = [];

        let unavailable: [ResourceId; 0] = [];

        let state =
            ConstraintState::new(
                &reservations,
                &completed,
                &unavailable,
            );

        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        assert!(matches!(
            constraint.evaluate(&context),
            Err(
                ConstraintViolation {
                    kind: ConstraintKind::Measurement,
                    ..
                }
            )
        ));
    }

    #[test]
    fn unavailable_measurement_resource_is_rejected() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(8),
            );

        let op = operation(12);

        constraint
            .register(op, descriptor())
            .expect("registration must succeed");

        let logical =
            [q(0)];

        let physical =
            [physical_q(0)];

        let claims = [
            ConstraintResourceClaim::new(
                resource(1),
                1,
            ),
        ];

        let candidate =
            SchedulingCandidate::new(
                op,
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(1),
            );

        let reservations: [ConstraintReservationView; 0] =
            [];

        let completed: [OperationId; 0] = [];

        let unavailable =
            [resource(1)];

        let state =
            ConstraintState::new(
                &reservations,
                &completed,
                &unavailable,
            );

        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        assert!(constraint.evaluate(&context).is_err());
    }

    #[test]
    fn unrelated_operations_are_not_applicable() {
        let mut constraint =
            MeasurementConstraint::production(
                ConstraintId::new(9),
            );

        constraint
            .register(operation(20), descriptor())
            .expect("registration must succeed");

        let logical =
            [q(0)];

        let physical =
            [physical_q(0)];

        let claims =
            [ConstraintResourceClaim::new(
                resource(1),
                1,
            )];

        let candidate =
            SchedulingCandidate::new(
                operation(21),
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(1),
            );

        let reservations: [ConstraintReservationView; 0] =
            [];

        let completed: [OperationId; 0] = [];

        let unavailable: [ResourceId; 0] = [];

        let state =
            ConstraintState::new(
                &reservations,
                &completed,
                &unavailable,
            );

        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        assert_eq!(
            constraint.applies(&context),
            ConstraintApplicability::NotApplicable
        );
    }

    #[test]
    fn runtime_phase_is_supported() {
        let constraint =
            MeasurementConstraint::production(
                ConstraintId::new(10),
            );

        assert!(
            constraint.supports_phase(
                ConstraintPhase::Runtime
            )
        );
    }

    #[test]
    fn resource_claims_are_dynamic() {
        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0)],
                vec![physical_q(0)],
                MeasurementSchedulingKind::Projective,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                vec![
                    ConstraintResourceClaim::new(
                        resource(1),
                        1,
                    ),
                    ConstraintResourceClaim::new(
                        resource(2),
                        1,
                    ),
                    ConstraintResourceClaim::new(
                        resource(3),
                        1,
                    ),
                ],
            );

        assert_eq!(
            descriptor.resource_claims().len(),
            3
        );
    }

    #[test]
    fn joint_measurement_supports_arbitrary_arity() {
        let descriptor =
            MeasurementDescriptor::new(
                vec![q(0), q(1), q(2), q(3)],
                vec![
                    physical_q(0),
                    physical_q(1),
                    physical_q(2),
                    physical_q(3),
                ],
                MeasurementSchedulingKind::Joint,
                MeasurementStateMode::NonDestructive,
                MeasurementResultMode::Deferred,
                false,
                Vec::new(),
            );

        assert!(descriptor.validate().is_ok());
        assert_eq!(
            descriptor.logical_qubit_count(),
            4
        );
    }

    #[test]
    fn reservation_view_can_be_constructed_without_hardware_types() {
        let reservation =
            ConstraintReservationView::new(
                ReservationId::new(1),
                Some(operation(1)),
                resource(1),
                TimePoint::new(0),
                Duration::new(10),
                1,
            );

        assert_eq!(
            reservation.checked_end(),
            Some(TimePoint::new(10))
        );
    }
}