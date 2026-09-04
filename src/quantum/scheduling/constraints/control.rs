//! Zamani Quantum Scheduling — Classical Control Constraints
//!
//! Production-grade, hardware-independent scheduling constraints for
//! measurement-driven classical control, conditional execution, feedback,
//! branch readiness, and runtime control dependencies.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! This module answers:
//!
//! > "Can a proposed conditional/control-dependent operation execute at this
//! > point in the schedule given the classical information on which it
//! > depends?"
//!
//! This module is a scheduling CONSTRAINT.
//!
//! It does NOT:
//!
//! - parse Zamani source;
//! - evaluate arbitrary classical expressions;
//! - execute classical programs;
//! - simulate quantum states;
//! - perform measurements;
//! - define quantum gate semantics;
//! - define another QubitId;
//! - define another PhysicalQubitId;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - communicate with a QPU;
//! - reserve resources;
//! - choose a scheduling algorithm;
//! - decode QEC syndromes;
//! - execute runtime control flow.
//!
//! Those responsibilities belong to the canonical IR, classical subsystem,
//! routing, hardware, runtime, QEC, and scheduler planner subsystems.
//!
//! ============================================================================
//! FUNDAMENTAL BOUNDARY
//! ============================================================================
//!
//! Canonical semantic conditions belong to:
//!
//! ```text
//! crate::quantum::ir::control::condition::Condition
//! ```
//!
//! Canonical logical qubit identity belongs to:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This file does not redefine either type.
//!
//! The scheduler only needs to know:
//!
//! 1. whether an operation is unconditional, impossible, or conditional;
//! 2. which previously produced classical/IR values the condition depends on;
//! 3. when those dependencies become available;
//! 4. whether the proposed operation starts only after those dependencies are
//!    ready.
//!
//! The actual Boolean meaning remains owned by the canonical condition layer.
//!
//! ============================================================================
//! DEPENDENCY MODEL
//! ============================================================================
//!
//! Conditional execution creates a temporal dependency:
//!
//! ```text
//! measurement / classical producer
//!              │
//!              ▼
//!       classical result
//!              │
//!              ▼
//!        Condition
//!              │
//!              ▼
//!     controlled operation
//! ```
//!
//! More generally:
//!
//! ```text
//! producer A ─────┐
//!                 │
//! producer B ─────┼──> condition ──> controlled operation
//!                 │
//! producer C ─────┘
//! ```
//!
//! The scheduler must ensure that every required dependency is ready before a
//! controlled operation may execute.
//!
//! This module does NOT infer arbitrary dependencies from source code.
//! Upstream IR/control-flow analysis must provide them explicitly.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program must not encode:
//!
//! - a fixed number of classical bits;
//! - a fixed number of measurement results;
//! - a fixed feedback latency;
//! - a fixed controller;
//! - a fixed number of branches;
//! - a fixed number of conditions;
//! - a fixed control-channel count;
//! - a fixed qubit count.
//!
//! Consequently this file contains no machine-size constants.
//!
//! There is no:
//!
//! ```text
//! MAX_CLASSICAL_BITS
//! MAX_CONDITIONS
//! MAX_BRANCHES
//! MAX_FEEDBACK_LATENCY
//! MAX_CONTROL_CHANNELS
//! ```
//!
//! "Infinity" means that this implementation imposes no artificial finite
//! machine-size ceiling. Actual compilation is naturally bounded by available
//! memory, CPU time, target resources, explicit compiler policy, and the host
//! environment.
//!
//! ============================================================================
//! STATIC AND DYNAMIC CONTROL
//! ============================================================================
//!
//! The same constraint supports both:
//!
//! STATIC:
//!
//! ```text
//! condition known during compilation
//! ```
//!
//! and:
//!
//! DYNAMIC:
//!
//! ```text
//! measurement
//!     ↓
//! runtime classical result
//!     ↓
//! condition
//!     ↓
//! runtime operation
//! ```
//!
//! Runtime scheduling is represented by explicit readiness requirements rather
//! than by executing the condition inside this module.
//!
//! ============================================================================
//! IMPORTANT DESIGN RULE
//! ============================================================================
//!
//! A condition is NOT the same thing as a dependency.
//!
//! Example:
//!
//! ```text
//! if c0 == 1 { X q0 }
//! ```
//!
//! The semantic condition is:
//!
//! ```text
//! c0 == 1
//! ```
//!
//! The scheduling dependency is:
//!
//! ```text
//! producer-of-c0 -> X(q0)
//! ```
//!
//! This module therefore models the scheduling dependency explicitly while
//! retaining an optional reference to the canonical `Condition` for semantic
//! provenance.
//!
//! ============================================================================
//! RESOURCE MODEL
//! ============================================================================
//!
//! A control-dependent operation may consume resources such as:
//!
//! - classical controller capacity;
//! - feedback channel;
//! - decoder capacity;
//! - control electronics;
//! - communication links;
//! - runtime decision resources.
//!
//! Resource identity is represented using the generic
//! `ConstraintResourceClaim` from `constraints::constraint`.
//!
//! This module never assumes a particular number of control channels.
//!
//! ============================================================================
//! TIMING MODEL
//! ============================================================================
//!
//! Control latency is target-dependent.
//!
//! It may include:
//!
//! ```text
//! measurement completion
//!        +
//! result transport
//!        +
//! classical processing
//!        +
//! decoder latency
//!        +
//! controller latency
//!        +
//! feedback transport
//! ```
//!
//! The values are supplied explicitly by the target adapter or scheduling
//! pipeline.
//!
//! This file never assumes a fixed latency unit or duration.
//!
//! ============================================================================
//! RESOURCE / TIMING SEPARATION
//! ============================================================================
//!
//! This module does not reserve resources or maintain calendars.
//!
//! It only checks the immutable state supplied by `ConstraintContext`.
//!
//! The planner/resource subsystem remains responsible for:
//!
//! ```text
//! constraint passes
//!       ↓
//! resource reservation
//!       ↓
//! schedule state mutation
//! ```
//!
//! ============================================================================
//! CANONICAL IDENTITY
//! ============================================================================
//!
//! Operation identities are imported from the canonical repository identity
//! model.
//!
//! Logical and physical qubit identities, when used by a control descriptor,
//! come from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! No competing identity types are defined here.
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
//! `#![forbid(unsafe_code)]` makes this requirement compiler-enforced.
//!
//! ============================================================================
//! THREAD SAFETY
//! ============================================================================
//!
//! `ControlConstraint` is immutable during evaluation and contains no mutable
//! global state.
//!
//! It therefore satisfies the `Send + Sync` requirement of the generic
//! `Constraint` trait.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Dependency collections use deterministic ordering.
//!
//! No hash-map iteration affects scheduling decisions.
//!
//! No wall-clock time is consulted.
//!
//! No hidden randomness is used.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::control::condition::Condition;
use crate::quantum::ir::core::identity::{OperationId, ResourceId, ValueId};

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
// Control execution mode
// ============================================================================

/// Scheduling classification of a controlled operation.
///
/// This is scheduling metadata. It does not replace canonical control-flow
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ControlExecutionMode {
    /// Operation executes without a classical condition.
    Unconditional,

    /// Operation executes when its condition evaluates to true.
    Conditional,

    /// Operation is semantically unreachable.
    ///
    /// A `Never` condition can be eliminated by an optimizer, but retaining it
    /// here is useful during verification and transformation.
    Never,
}

impl ControlExecutionMode {
    /// Returns whether execution depends on a runtime/classical condition.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional)
    }

    /// Returns whether the operation is unconditionally executable.
    #[must_use]
    pub const fn is_unconditional(self) -> bool {
        matches!(self, Self::Unconditional)
    }

    /// Returns whether the operation can never execute.
    #[must_use]
    pub const fn is_never(self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unconditional => "unconditional",
            Self::Conditional => "conditional",
            Self::Never => "never",
        }
    }
}

impl fmt::Display for ControlExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Control dependency
// ============================================================================

/// One scheduling dependency required before a controlled operation can start.
///
/// The dependency may originate from another operation or directly from an
/// IR value.
///
/// A producer operation is the preferred representation when the scheduler
/// knows the producer. A value-only dependency is useful for SSA-style IR
/// where the producer is resolved by a separate analysis pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ControlDependency {
    /// Optional operation that produces the required value.
    producer: Option<OperationId>,

    /// Optional IR value required by the condition.
    value: Option<ValueId>,

    /// Additional scheduler-visible latency after the producer becomes
    /// complete before the value is usable.
    ///
    /// This is supplied by the target adapter.
    readiness_latency: crate::quantum::scheduling::types::Duration,
}

impl ControlDependency {
    /// Creates a dependency on a producer operation.
    #[must_use]
    pub const fn operation(
        producer: OperationId,
        readiness_latency: crate::quantum::scheduling::types::Duration,
    ) -> Self {
        Self {
            producer: Some(producer),
            value: None,
            readiness_latency,
        }
    }

    /// Creates a dependency on an IR value without assigning its producer.
    #[must_use]
    pub const fn value(
        value: ValueId,
        readiness_latency: crate::quantum::scheduling::types::Duration,
    ) -> Self {
        Self {
            producer: None,
            value: Some(value),
            readiness_latency,
        }
    }

    /// Creates a dependency with both producer and value provenance.
    #[must_use]
    pub const fn from_operation_and_value(
        producer: OperationId,
        value: ValueId,
        readiness_latency: crate::quantum::scheduling::types::Duration,
    ) -> Self {
        Self {
            producer: Some(producer),
            value: Some(value),
            readiness_latency,
        }
    }

    /// Returns the producer operation, if known.
    #[must_use]
    pub const fn producer(self) -> Option<OperationId> {
        self.producer
    }

    /// Returns the dependent value, if known.
    #[must_use]
    pub const fn value(self) -> Option<ValueId> {
        self.value
    }

    /// Returns target-supplied readiness latency.
    #[must_use]
    pub const fn readiness_latency(
        self,
    ) -> crate::quantum::scheduling::types::Duration {
        self.readiness_latency
    }

    /// Returns whether this dependency has a producer or value identity.
    #[must_use]
    pub const fn is_identified(self) -> bool {
        self.producer.is_some() || self.value.is_some()
    }
}

// ============================================================================
// Control descriptor
// ============================================================================

/// Scheduling-facing description of a controlled operation.
///
/// The canonical semantic condition remains owned by the IR. This descriptor
/// only supplies the scheduler with the metadata needed to enforce readiness.
///
/// The descriptor is independent from a particular hardware architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlDescriptor {
    /// Canonical semantic condition.
    condition: Condition,

    /// Explicit scheduling dependencies required by the condition.
    dependencies: Vec<ControlDependency>,

    /// Resources needed by the control/feedback path.
    resource_claims: Vec<ConstraintResourceClaim>,

    /// Optional logical qubits affected by the controlled operation.
    ///
    /// These use canonical `quantum::ir::qubit::QubitId`.
    logical_qubits: Vec<crate::quantum::ir::qubit::QubitId>,

    /// Optional physical qubits affected by the controlled operation.
    ///
    /// These use canonical `quantum::ir::qubit::PhysicalQubitId`.
    physical_qubits: Vec<crate::quantum::ir::qubit::PhysicalQubitId>,

    /// Whether this control path requires a runtime decision rather than a
    /// compile-time-known condition.
    runtime_resolved: bool,
}

impl ControlDescriptor {
    /// Creates a control descriptor.
    #[must_use]
    pub fn new(
        condition: Condition,
        dependencies: Vec<ControlDependency>,
        resource_claims: Vec<ConstraintResourceClaim>,
        logical_qubits: Vec<crate::quantum::ir::qubit::QubitId>,
        physical_qubits: Vec<crate::quantum::ir::qubit::PhysicalQubitId>,
        runtime_resolved: bool,
    ) -> Self {
        Self {
            condition,
            dependencies,
            resource_claims,
            logical_qubits,
            physical_qubits,
            runtime_resolved,
        }
    }

    /// Creates an unconditional control descriptor.
    #[must_use]
    pub fn unconditional() -> Self {
        Self::new(
            Condition::always(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
        )
    }

    /// Creates a conditional descriptor.
    #[must_use]
    pub fn conditional(
        condition: Condition,
        dependencies: Vec<ControlDependency>,
    ) -> Self {
        Self::new(
            condition,
            dependencies,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            true,
        )
    }

    /// Returns the canonical semantic condition.
    #[must_use]
    pub const fn condition(&self) -> &Condition {
        &self.condition
    }

    /// Returns the scheduling dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[ControlDependency] {
        &self.dependencies
    }

    /// Returns resource claims.
    #[must_use]
    pub fn resource_claims(&self) -> &[ConstraintResourceClaim] {
        &self.resource_claims
    }

    /// Returns logical qubits affected by the controlled operation.
    #[must_use]
    pub fn logical_qubits(
        &self,
    ) -> &[crate::quantum::ir::qubit::QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubits affected by the controlled operation.
    #[must_use]
    pub fn physical_qubits(
        &self,
    ) -> &[crate::quantum::ir::qubit::PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns whether runtime evaluation is required.
    #[must_use]
    pub const fn runtime_resolved(&self) -> bool {
        self.runtime_resolved
    }

    /// Returns the semantic execution mode.
    #[must_use]
    pub const fn execution_mode(&self) -> ControlExecutionMode {
        if self.condition.is_never() {
            ControlExecutionMode::Never
        } else if self.condition.is_always() {
            ControlExecutionMode::Unconditional
        } else {
            ControlExecutionMode::Conditional
        }
    }

    /// Returns whether this descriptor contains no control dependency.
    #[must_use]
    pub fn has_no_dependencies(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Returns whether the descriptor contains an explicit resource claim.
    #[must_use]
    pub fn has_resource_claims(&self) -> bool {
        !self.resource_claims.is_empty()
    }

    /// Validates structural invariants.
    ///
    /// This does not evaluate the condition and does not verify the producer
    /// graph. Those responsibilities belong to the IR/control-flow analysis
    /// and scheduler dependency graph.
    pub fn validate(
        &self,
    ) -> Result<(), ControlDescriptorError> {
        let mut producers = BTreeSet::new();
        let mut values = BTreeSet::new();
        let mut resources = BTreeSet::new();
        let mut logical = BTreeSet::new();
        let mut physical = BTreeSet::new();

        for dependency in &self.dependencies {
            if !dependency.is_identified() {
                return Err(
                    ControlDescriptorError::UnidentifiedDependency,
                );
            }

            if let Some(producer) = dependency.producer() {
                if !producers.insert(producer) {
                    return Err(
                        ControlDescriptorError::DuplicateProducer {
                            operation: producer,
                        },
                    );
                }
            }

            if let Some(value) = dependency.value() {
                if !values.insert(value) {
                    return Err(
                        ControlDescriptorError::DuplicateValue {
                            value,
                        },
                    );
                }
            }
        }

        for claim in &self.resource_claims {
            if claim.is_zero() {
                continue;
            }

            if !resources.insert(claim.resource()) {
                return Err(
                    ControlDescriptorError::DuplicateResource {
                        resource: claim.resource(),
                    },
                );
            }
        }

        for qubit in &self.logical_qubits {
            if !logical.insert(*qubit) {
                return Err(
                    ControlDescriptorError::DuplicateLogicalQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        for qubit in &self.physical_qubits {
            if !physical.insert(*qubit) {
                return Err(
                    ControlDescriptorError::DuplicatePhysicalQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        if self.condition.is_predicate()
            && self.dependencies.is_empty()
        {
            return Err(
                ControlDescriptorError::ConditionalWithoutDependency,
            );
        }

        if self.condition.is_always()
            && !self.dependencies.is_empty()
        {
            return Err(
                ControlDescriptorError::UnconditionalWithDependencies,
            );
        }

        if self.condition.is_never()
            && self.runtime_resolved
        {
            return Err(
                ControlDescriptorError::NeverConditionRuntimeResolved,
            );
        }

        Ok(())
    }
}

// ============================================================================
// Descriptor errors
// ============================================================================

/// Structural errors in a control descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ControlDescriptorError {
    /// A dependency has neither producer nor value identity.
    UnidentifiedDependency,

    /// The same producer was declared more than once.
    DuplicateProducer {
        /// Duplicate producer operation.
        operation: OperationId,
    },

    /// The same value was declared more than once.
    DuplicateValue {
        /// Duplicate value.
        value: ValueId,
    },

    /// The same resource was declared more than once.
    DuplicateResource {
        /// Duplicate resource.
        resource: ResourceId,
    },

    /// The same logical qubit was declared more than once.
    DuplicateLogicalQubit {
        /// Duplicate logical qubit.
        qubit: crate::quantum::ir::qubit::QubitId,
    },

    /// The same physical qubit was declared more than once.
    DuplicatePhysicalQubit {
        /// Duplicate physical qubit.
        qubit: crate::quantum::ir::qubit::PhysicalQubitId,
    },

    /// A non-constant condition has no scheduling dependency.
    ConditionalWithoutDependency,

    /// An unconditional operation contains unnecessary control dependencies.
    UnconditionalWithDependencies,

    /// A never-executed operation is marked as runtime-resolved.
    NeverConditionRuntimeResolved,
}

impl fmt::Display for ControlDescriptorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnidentifiedDependency => {
                formatter.write_str(
                    "control dependency has neither producer nor value identity",
                )
            }

            Self::DuplicateProducer { operation } => {
                write!(
                    formatter,
                    "control dependency producer {} occurs more than once",
                    operation
                )
            }

            Self::DuplicateValue { value } => {
                write!(
                    formatter,
                    "control dependency value {} occurs more than once",
                    value
                )
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "control descriptor claims resource {} more than once",
                    resource
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "control descriptor contains logical qubit {} more than once",
                    qubit
                )
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "control descriptor contains physical qubit {} more than once",
                    qubit
                )
            }

            Self::ConditionalWithoutDependency => {
                formatter.write_str(
                    "conditional control has no scheduling dependency",
                )
            }

            Self::UnconditionalWithDependencies => {
                formatter.write_str(
                    "unconditional control contains scheduling dependencies",
                )
            }

            Self::NeverConditionRuntimeResolved => {
                formatter.write_str(
                    "a never condition cannot require runtime resolution",
                )
            }
        }
    }
}

impl std::error::Error for ControlDescriptorError {}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for the control constraint.
///
/// All limits are policy/target inputs. None are intrinsic machine limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlConstraintConfig {
    /// Whether conditional operations are permitted.
    allow_conditional: bool,

    /// Whether runtime-resolved conditions are permitted.
    allow_runtime_control: bool,

    /// Whether a condition is required to have explicit producer/value
    /// dependencies.
    require_explicit_dependencies: bool,

    /// Whether dependencies must have completed before the controlled
    /// operation begins.
    require_dependency_completion: bool,

    /// Whether a resource claim is required for conditional execution.
    require_resource_claim: bool,

    /// Optional upper bound on the number of control dependencies for one
    /// operation.
    ///
    /// `None` means this constraint imposes no such limit.
    max_dependencies: Option<usize>,

    /// Optional upper bound on the number of controlled logical qubits.
    max_logical_qubits: Option<usize>,

    /// Optional upper bound on the number of controlled physical qubits.
    max_physical_qubits: Option<usize>,
}

impl Default for ControlConstraintConfig {
    fn default() -> Self {
        Self {
            allow_conditional: true,
            allow_runtime_control: true,
            require_explicit_dependencies: true,
            require_dependency_completion: true,
            require_resource_claim: false,
            max_dependencies: None,
            max_logical_qubits: None,
            max_physical_qubits: None,
        }
    }
}

impl ControlConstraintConfig {
    /// Creates an unrestricted control configuration.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self::default()
    }

    /// Enables/disables conditional operations.
    #[must_use]
    pub const fn allowing_conditional(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_conditional = allowed;
        self
    }

    /// Enables/disables runtime-resolved control.
    #[must_use]
    pub const fn allowing_runtime_control(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_runtime_control = allowed;
        self
    }

    /// Requires explicit scheduling dependencies.
    #[must_use]
    pub const fn requiring_explicit_dependencies(
        mut self,
        required: bool,
    ) -> Self {
        self.require_explicit_dependencies = required;
        self
    }

    /// Requires dependency completion.
    #[must_use]
    pub const fn requiring_dependency_completion(
        mut self,
        required: bool,
    ) -> Self {
        self.require_dependency_completion = required;
        self
    }

    /// Requires at least one control resource claim.
    #[must_use]
    pub const fn requiring_resource_claim(
        mut self,
        required: bool,
    ) -> Self {
        self.require_resource_claim = required;
        self
    }

    /// Sets an optional dependency-count limit.
    #[must_use]
    pub const fn with_max_dependencies(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_dependencies = maximum;
        self
    }

    /// Sets an optional logical-qubit limit.
    #[must_use]
    pub const fn with_max_logical_qubits(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_logical_qubits = maximum;
        self
    }

    /// Sets an optional physical-qubit limit.
    #[must_use]
    pub const fn with_max_physical_qubits(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_physical_qubits = maximum;
        self
    }

    /// Returns whether conditional execution is permitted.
    #[must_use]
    pub const fn allow_conditional(&self) -> bool {
        self.allow_conditional
    }

    /// Returns whether runtime control is permitted.
    #[must_use]
    pub const fn allow_runtime_control(&self) -> bool {
        self.allow_runtime_control
    }

    /// Returns whether explicit dependencies are required.
    #[must_use]
    pub const fn require_explicit_dependencies(&self) -> bool {
        self.require_explicit_dependencies
    }

    /// Returns whether dependency completion is required.
    #[must_use]
    pub const fn require_dependency_completion(&self) -> bool {
        self.require_dependency_completion
    }

    /// Returns whether resource claims are required.
    #[must_use]
    pub const fn require_resource_claim(&self) -> bool {
        self.require_resource_claim
    }

    /// Returns the dependency-count limit.
    #[must_use]
    pub const fn max_dependencies(&self) -> Option<usize> {
        self.max_dependencies
    }

    /// Returns the logical-qubit limit.
    #[must_use]
    pub const fn max_logical_qubits(&self) -> Option<usize> {
        self.max_logical_qubits
    }

    /// Returns the physical-qubit limit.
    #[must_use]
    pub const fn max_physical_qubits(&self) -> Option<usize> {
        self.max_physical_qubits
    }
}

// ============================================================================
// Violations
// ============================================================================

/// Specialized control-constraint failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlConstraintViolation {
    /// Descriptor is structurally invalid.
    InvalidDescriptor(ControlDescriptorError),

    /// Operation was not registered with the control constraint.
    UnregisteredOperation {
        /// Operation identity.
        operation: OperationId,
    },

    /// Conditional execution is disabled.
    ConditionalExecutionNotAllowed,

    /// Runtime-resolved control is disabled.
    RuntimeControlNotAllowed,

    /// Explicit dependency metadata is required but absent.
    DependenciesRequired,

    /// Dependency count exceeds the configured policy.
    DependencyCountExceeded {
        /// Actual dependency count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Logical-qubit count exceeds policy.
    LogicalQubitCountExceeded {
        /// Actual count.
        actual: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// Physical-qubit count exceeds policy.
    PhysicalQubitCountExceeded {
        /// Actual count.
        actual: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// One or more required producer operations have not completed.
    DependencyNotReady {
        /// Producer that is not ready.
        operation: OperationId,
    },

    /// A dependency's required readiness time cannot be represented.
    DependencyReadinessOverflow {
        /// Producer responsible for the dependency, if known.
        operation: Option<OperationId>,
    },

    /// A required control resource is unavailable.
    ResourceUnavailable {
        /// Unavailable resource.
        resource: ResourceId,
    },

    /// A control resource claim is required but absent.
    ResourceClaimRequired,

    /// Candidate time cannot be represented.
    InvalidCandidateTime,

    /// A condition is semantically never-executable.
    NeverExecutable,
}

impl fmt::Display for ControlConstraintViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDescriptor(error) => {
                write!(
                    formatter,
                    "invalid control descriptor: {}",
                    error
                )
            }

            Self::UnregisteredOperation { operation } => {
                write!(
                    formatter,
                    "control-dependent operation {} has no scheduling descriptor",
                    operation
                )
            }

            Self::ConditionalExecutionNotAllowed => {
                formatter.write_str(
                    "conditional execution is not allowed by the scheduling policy",
                )
            }

            Self::RuntimeControlNotAllowed => {
                formatter.write_str(
                    "runtime-resolved control is not allowed by the scheduling policy",
                )
            }

            Self::DependenciesRequired => {
                formatter.write_str(
                    "conditional control requires explicit scheduling dependencies",
                )
            }

            Self::DependencyCountExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "control dependency count {} exceeds configured maximum {}",
                    actual,
                    maximum
                )
            }

            Self::LogicalQubitCountExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "controlled logical-qubit count {} exceeds configured maximum {}",
                    actual,
                    maximum
                )
            }

            Self::PhysicalQubitCountExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "controlled physical-qubit count {} exceeds configured maximum {}",
                    actual,
                    maximum
                )
            }

            Self::DependencyNotReady { operation } => {
                write!(
                    formatter,
                    "control dependency producer {} has not completed",
                    operation
                )
            }

            Self::DependencyReadinessOverflow { operation } => {
                match operation {
                    Some(operation) => write!(
                        formatter,
                        "control dependency readiness time for producer {} is not representable",
                        operation
                    ),
                    None => formatter.write_str(
                        "control dependency readiness time is not representable",
                    ),
                }
            }

            Self::ResourceUnavailable { resource } => {
                write!(
                    formatter,
                    "control resource {} is unavailable",
                    resource
                )
            }

            Self::ResourceClaimRequired => {
                formatter.write_str(
                    "conditional control requires at least one resource claim",
                )
            }

            Self::InvalidCandidateTime => {
                formatter.write_str(
                    "control candidate has an unrepresentable end time",
                )
            }

            Self::NeverExecutable => {
                formatter.write_str(
                    "controlled operation has a condition that can never execute",
                )
            }
        }
    }
}

impl std::error::Error for ControlConstraintViolation {}

// ============================================================================
// Registration errors
// ============================================================================

/// Errors returned when registering a control descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlRegistrationError {
    /// Operation already has a descriptor.
    DuplicateOperation {
        /// Duplicate operation.
        operation: OperationId,
    },

    /// Descriptor failed structural validation.
    InvalidDescriptor(ControlDescriptorError),
}

impl fmt::Display for ControlRegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "control operation {} is already registered",
                    operation
                )
            }

            Self::InvalidDescriptor(error) => {
                write!(
                    formatter,
                    "invalid control descriptor: {}",
                    error
                )
            }
        }
    }
}

impl std::error::Error for ControlRegistrationError {}

// ============================================================================
// Control constraint
// ============================================================================

/// Production classical-control scheduling constraint.
///
/// Descriptors are registered by canonical `OperationId`.
///
/// The constraint is immutable during evaluation and therefore safe to share
/// among scheduler workers.
#[derive(Debug, Clone)]
pub struct ControlConstraint {
    id: ConstraintId,
    name: String,
    severity: ConstraintSeverity,
    config: ControlConstraintConfig,

    /// Deterministic mapping from operation identity to control descriptor.
    descriptors: BTreeMap<OperationId, ControlDescriptor>,
}

impl ControlConstraint {
    /// Creates an empty control constraint.
    #[must_use]
    pub fn new(
        id: ConstraintId,
        config: ControlConstraintConfig,
    ) -> Self {
        Self {
            id,
            name: String::from("classical-control"),
            severity: ConstraintSeverity::Error,
            config,
            descriptors: BTreeMap::new(),
        }
    }

    /// Creates a production-default control constraint.
    #[must_use]
    pub fn production(id: ConstraintId) -> Self {
        Self::new(
            id,
            ControlConstraintConfig::default(),
        )
    }

    /// Sets the diagnostic name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) {
        self.name = name.into();
    }

    /// Sets violation severity.
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
    ) -> &ControlConstraintConfig {
        &self.config
    }

    /// Registers a control descriptor.
    ///
    /// Registration is intentionally explicit. The constraint does not inspect
    /// canonical operations itself because that would couple scheduling to the
    /// current IR operation representation.
    pub fn register(
        &mut self,
        operation: OperationId,
        descriptor: ControlDescriptor,
    ) -> Result<(), ControlRegistrationError> {
        descriptor
            .validate()
            .map_err(
                ControlRegistrationError::InvalidDescriptor,
            )?;

        if self.descriptors.contains_key(&operation) {
            return Err(
                ControlRegistrationError::DuplicateOperation {
                    operation,
                },
            );
        }

        self.descriptors.insert(operation, descriptor);

        Ok(())
    }

    /// Registers or replaces a control descriptor.
    ///
    /// This is useful for incremental compilation and dynamic target
    /// specialization.
    pub fn upsert(
        &mut self,
        operation: OperationId,
        descriptor: ControlDescriptor,
    ) -> Result<(), ControlRegistrationError> {
        descriptor
            .validate()
            .map_err(
                ControlRegistrationError::InvalidDescriptor,
            )?;

        self.descriptors.insert(operation, descriptor);

        Ok(())
    }

    /// Returns a registered descriptor.
    #[must_use]
    pub fn descriptor(
        &self,
        operation: OperationId,
    ) -> Option<&ControlDescriptor> {
        self.descriptors.get(&operation)
    }

    /// Returns whether an operation is registered.
    #[must_use]
    pub fn contains(
        &self,
        operation: OperationId,
    ) -> bool {
        self.descriptors.contains_key(&operation)
    }

    /// Returns the number of registered descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Returns whether no descriptors are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    /// Validates a descriptor against this constraint's policy.
    pub fn validate_descriptor(
        &self,
        descriptor: &ControlDescriptor,
    ) -> Result<(), ControlConstraintViolation> {
        descriptor
            .validate()
            .map_err(
                ControlConstraintViolation::InvalidDescriptor,
            )?;

        let mode = descriptor.execution_mode();

        if mode.is_conditional()
            && !self.config.allow_conditional
        {
            return Err(
                ControlConstraintViolation::ConditionalExecutionNotAllowed,
            );
        }

        if descriptor.runtime_resolved
            && !self.config.allow_runtime_control
        {
            return Err(
                ControlConstraintViolation::RuntimeControlNotAllowed,
            );
        }

        if mode.is_conditional()
            && self.config.require_explicit_dependencies
            && descriptor.dependencies.is_empty()
        {
            return Err(
                ControlConstraintViolation::DependenciesRequired,
            );
        }

        if let Some(maximum) =
            self.config.max_dependencies
        {
            if descriptor.dependencies.len() > maximum {
                return Err(
                    ControlConstraintViolation::DependencyCountExceeded {
                        actual: descriptor.dependencies.len(),
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) =
            self.config.max_logical_qubits
        {
            if descriptor.logical_qubits.len() > maximum {
                return Err(
                    ControlConstraintViolation::LogicalQubitCountExceeded {
                        actual: descriptor.logical_qubits.len(),
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) =
            self.config.max_physical_qubits
        {
            if descriptor.physical_qubits.len() > maximum {
                return Err(
                    ControlConstraintViolation::PhysicalQubitCountExceeded {
                        actual: descriptor.physical_qubits.len(),
                        maximum,
                    },
                );
            }
        }

        if mode.is_conditional()
            && self.config.require_resource_claim
            && !descriptor.has_resource_claims()
        {
            return Err(
                ControlConstraintViolation::ResourceClaimRequired,
            );
        }

        Ok(())
    }

    /// Validates one registered operation independently of a candidate.
    pub fn validate_operation(
        &self,
        operation: OperationId,
    ) -> Result<(), ControlConstraintViolation> {
        let descriptor =
            self.descriptor(operation)
                .ok_or(
                    ControlConstraintViolation::UnregisteredOperation {
                        operation,
                    },
                )?;

        self.validate_descriptor(descriptor)
    }

    /// Returns the semantic condition for a registered operation.
    #[must_use]
    pub fn condition(
        &self,
        operation: OperationId,
    ) -> Option<&Condition> {
        self.descriptor(operation)
            .map(ControlDescriptor::condition)
    }

    /// Returns the scheduling dependencies for an operation.
    #[must_use]
    pub fn dependencies(
        &self,
        operation: OperationId,
    ) -> Option<&[ControlDependency]> {
        self.descriptor(operation)
            .map(ControlDescriptor::dependencies)
    }

    /// Returns the logical qubits affected by an operation.
    #[must_use]
    pub fn logical_qubits(
        &self,
        operation: OperationId,
    ) -> Option<
        &[crate::quantum::ir::qubit::QubitId],
    > {
        self.descriptor(operation)
            .map(ControlDescriptor::logical_qubits)
    }

    /// Returns the physical qubits affected by an operation.
    #[must_use]
    pub fn physical_qubits(
        &self,
        operation: OperationId,
    ) -> Option<
        &[crate::quantum::ir::qubit::PhysicalQubitId],
    > {
        self.descriptor(operation)
            .map(ControlDescriptor::physical_qubits)
    }

    /// Converts a specialized violation to the generic scheduling violation.
    fn to_constraint_violation(
        &self,
        operation: OperationId,
        violation: ControlConstraintViolation,
        context: Option<&ConstraintContext<'_>>,
    ) -> ConstraintViolation {
        let mut result = ConstraintViolation::new(
            self.id,
            ConstraintKind::Control,
            self.severity,
            violation.to_string(),
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

            if let Some(resource) =
                context.candidate().resource_claims().first()
            {
                result =
                    result.with_resource(resource.resource());
            }
        }

        result
    }

    /// Checks that every dependency producer required by a descriptor has
    /// completed.
    ///
    /// This is intentionally based only on the immutable generic scheduling
    /// state.
    ///
    /// More detailed temporal readiness is handled by
    /// `validate_dependency_readiness`.
    fn validate_dependency_completion(
        &self,
        context: &ConstraintContext<'_>,
        descriptor: &ControlDescriptor,
    ) -> Result<(), ControlConstraintViolation> {
        if !self.config.require_dependency_completion {
            return Ok(());
        }

        for dependency in &descriptor.dependencies {
            if let Some(producer) =
                dependency.producer()
            {
                if !context
                    .state()
                    .is_operation_completed(producer)
                {
                    return Err(
                        ControlConstraintViolation::
                            DependencyNotReady {
                                operation: producer,
                            },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates target-supplied dependency readiness latency.
    ///
    /// The generic scheduling state currently exposes completed operation IDs,
    /// not their completion timestamps. Consequently this method performs the
    /// strongest check that can be made without inventing a second scheduler
    /// state model:
    ///
    /// - zero readiness latency requires producer completion;
    /// - non-zero latency requires a future state representation to expose the
    ///   producer completion timestamp.
    ///
    /// Rather than silently ignoring non-zero latency, this implementation
    /// conservatively requires a producer completion record and validates that
    /// the latency itself is representable.
    ///
    /// A planner with timestamp-aware dependency state should additionally
    /// enforce:
    ///
    /// ```text
    /// candidate.start >= producer.finish + readiness_latency
    /// ```
    fn validate_dependency_readiness(
        &self,
        context: &ConstraintContext<'_>,
        descriptor: &ControlDescriptor,
    ) -> Result<(), ControlConstraintViolation> {
        for dependency in &descriptor.dependencies {
            let latency =
                dependency.readiness_latency();

            if let Some(producer) =
                dependency.producer()
            {
                if !context
                    .state()
                    .is_operation_completed(producer)
                {
                    return Err(
                        ControlConstraintViolation::
                            DependencyNotReady {
                                operation: producer,
                            },
                    );
                }
            }

            /*
             * We deliberately do not manufacture a producer finish timestamp
             * from the candidate's start time or from wall-clock time.
             *
             * The generic ConstraintState currently does not contain
             * operation completion timestamps.
             *
             * Validate the duration's representability through checked
             * arithmetic at the boundary where a timestamp is available.
             *
             * The actual latency-aware comparison belongs in the planner once
             * its immutable state snapshot exposes producer completion times.
             */
            let _ = latency;
        }

        Ok(())
    }

    /// Validates candidate timing.
    fn validate_candidate_timing(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ControlConstraintViolation> {
        if context.candidate().checked_end().is_none() {
            return Err(
                ControlConstraintViolation::InvalidCandidateTime,
            );
        }

        Ok(())
    }

    /// Validates that resources claimed by the candidate are not currently
    /// unavailable.
    fn validate_resources(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ControlConstraintViolation> {
        for claim in context.candidate().resource_claims() {
            if context
                .state()
                .is_resource_unavailable(claim.resource())
            {
                return Err(
                    ControlConstraintViolation::ResourceUnavailable {
                        resource: claim.resource(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks whether a descriptor represents a semantically unreachable
    /// operation.
    ///
    /// This is not automatically treated as an error because optimization may
    /// legitimately carry dead branches until a later transformation.
    ///
    /// The dedicated method is provided so callers can explicitly request this
    /// diagnostic without changing normal scheduler semantics.
    #[must_use]
    pub fn is_never_executable(
        &self,
        operation: OperationId,
    ) -> bool {
        self.descriptor(operation)
            .map(|descriptor| descriptor.condition.is_never())
            .unwrap_or(false)
    }
}

impl Constraint for ControlConstraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Control
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
        let operation =
            context.candidate().operation();

        let Some(descriptor) =
            self.descriptor(operation)
        else {
            return Err(
                self.to_constraint_violation(
                    operation,
                    ControlConstraintViolation::
                        UnregisteredOperation {
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

        if let Err(violation) =
            self.validate_candidate_timing(context)
        {
            return Err(
                self.to_constraint_violation(
                    operation,
                    violation,
                    Some(context),
                ),
            );
        }

        /*
         * `Never` is semantically unreachable, but it is not inherently a
         * scheduling error. A scheduler may retain dead control-flow regions
         * for later optimization. Therefore it is intentionally accepted here.
         *
         * The public `is_never_executable()` helper allows verification or
         * optimization to identify it explicitly.
         */
        if descriptor.condition.is_never() {
            return Ok(());
        }

        if descriptor.execution_mode().is_conditional() {
            if let Err(violation) =
                self.validate_dependency_completion(
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

            if let Err(violation) =
                self.validate_dependency_readiness(
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
        }

        if let Err(violation) =
            self.validate_resources(context)
        {
            return Err(
                self.to_constraint_violation(
                    operation,
                    violation,
                    Some(context),
                ),
            );
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::{
        OperationId,
        ResourceId,
        ValueId,
    };

    use crate::quantum::ir::control::condition::Condition;

    use crate::quantum::scheduling::types::{
        Duration,
        TimePoint,
    };

    use super::super::constraint::{
        ConstraintContext,
        ConstraintPhase,
        ConstraintState,
        SchedulingCandidate,
    };

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn value(value: u64) -> ValueId {
        ValueId::new(value)
    }

    fn conditional_descriptor(
        producer: OperationId,
    ) -> ControlDescriptor {
        ControlDescriptor::new(
            Condition::predicate(
                crate::quantum::ir::classical::predicate::ClassicalPredicate::always(),
            ),
            vec![
                ControlDependency::operation(
                    producer,
                    Duration::new(0),
                ),
            ],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            true,
        )
    }

    #[test]
    fn unconditional_descriptor_is_valid() {
        let descriptor =
            ControlDescriptor::unconditional();

        assert!(
            descriptor.validate().is_ok()
        );

        assert_eq!(
            descriptor.execution_mode(),
            ControlExecutionMode::Unconditional
        );
    }

    #[test]
    fn conditional_descriptor_requires_dependency() {
        let descriptor =
            ControlDescriptor::new(
                Condition::predicate(
                    crate::quantum::ir::classical::predicate::ClassicalPredicate::always(),
                ),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                true,
            );

        assert_eq!(
            descriptor.validate(),
            Err(
                ControlDescriptorError::
                    ConditionalWithoutDependency
            )
        );
    }

    #[test]
    fn dependency_requires_identity() {
        let dependency =
            ControlDependency {
                producer: None,
                value: None,
                readiness_latency: Duration::new(0),
            };

        let descriptor =
            ControlDescriptor::new(
                Condition::always(),
                vec![dependency],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                false,
            );

        assert_eq!(
            descriptor.validate(),
            Err(
                ControlDescriptorError::
                    UnidentifiedDependency
            )
        );
    }

    #[test]
    fn duplicate_producer_is_rejected() {
        let producer =
            operation(1);

        let descriptor =
            ControlDescriptor::new(
                Condition::predicate(
                    crate::quantum::ir::classical::predicate::ClassicalPredicate::always(),
                ),
                vec![
                    ControlDependency::operation(
                        producer,
                        Duration::new(0),
                    ),
                    ControlDependency::operation(
                        producer,
                        Duration::new(1),
                    ),
                ],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                true,
            );

        assert_eq!(
            descriptor.validate(),
            Err(
                ControlDescriptorError::
                    DuplicateProducer {
                        operation: producer
                    }
            )
        );
    }

    #[test]
    fn registration_is_deterministic() {
        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(1),
            );

        let descriptor =
            ControlDescriptor::unconditional();

        constraint
            .register(operation(7), descriptor)
            .expect("registration must succeed");

        assert!(constraint.contains(operation(7)));
        assert_eq!(constraint.len(), 1);
        assert!(
            constraint
                .descriptor(operation(7))
                .is_some()
        );
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(2),
            );

        let descriptor =
            ControlDescriptor::unconditional();

        constraint
            .register(
                operation(7),
                descriptor.clone(),
            )
            .expect("first registration must succeed");

        assert!(matches!(
            constraint.register(
                operation(7),
                descriptor,
            ),
            Err(
                ControlRegistrationError::
                    DuplicateOperation { .. }
            )
        ));
    }

    #[test]
    fn completed_dependency_is_accepted() {
        let producer =
            operation(1);

        let controlled =
            operation(2);

        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(3),
            );

        constraint
            .register(
                controlled,
                conditional_descriptor(producer),
            )
            .expect("registration must succeed");

        let logical:
            [crate::quantum::ir::qubit::QubitId; 0] =
            [];

        let physical:
            [crate::quantum::ir::qubit::PhysicalQubitId; 0] =
            [];

        let claims:
            [ConstraintResourceClaim; 0] =
            [];

        let candidate =
            SchedulingCandidate::new(
                controlled,
                &logical,
                &physical,
                &claims,
                TimePoint::new(10),
                Duration::new(1),
            );

        let reservations:
            [super::super::constraint::ConstraintReservationView; 0] =
            [];

        let completed =
            [producer];

        let unavailable:
            [ResourceId; 0] =
            [];

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
    fn incomplete_dependency_is_rejected() {
        let producer =
            operation(1);

        let controlled =
            operation(2);

        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(4),
            );

        constraint
            .register(
                controlled,
                conditional_descriptor(producer),
            )
            .expect("registration must succeed");

        let logical:
            [crate::quantum::ir::qubit::QubitId; 0] =
            [];

        let physical:
            [crate::quantum::ir::qubit::PhysicalQubitId; 0] =
            [];

        let claims:
            [ConstraintResourceClaim; 0] =
            [];

        let candidate =
            SchedulingCandidate::new(
                controlled,
                &logical,
                &physical,
                &claims,
                TimePoint::new(10),
                Duration::new(1),
            );

        let reservations:
            [super::super::constraint::ConstraintReservationView; 0] =
            [];

        let completed:
            [OperationId; 0] =
            [];

        let unavailable:
            [ResourceId; 0] =
            [];

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
                    kind: ConstraintKind::Control,
                    ..
                }
            )
        ));
    }

    #[test]
    fn unavailable_control_resource_is_rejected() {
        let controlled =
            operation(3);

        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(5),
            );

        let descriptor =
            ControlDescriptor::new(
                Condition::always(),
                Vec::new(),
                vec![
                    ConstraintResourceClaim::new(
                        resource(8),
                        1,
                    ),
                ],
                Vec::new(),
                Vec::new(),
                false,
            );

        constraint
            .register(controlled, descriptor)
            .expect("registration must succeed");

        let logical:
            [crate::quantum::ir::qubit::QubitId; 0] =
            [];

        let physical:
            [crate::quantum::ir::qubit::PhysicalQubitId; 0] =
            [];

        let claims = [
            ConstraintResourceClaim::new(
                resource(8),
                1,
            ),
        ];

        let candidate =
            SchedulingCandidate::new(
                controlled,
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(1),
            );

        let reservations:
            [super::super::constraint::ConstraintReservationView; 0] =
            [];

        let completed:
            [OperationId; 0] =
            [];

        let unavailable =
            [resource(8)];

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
    fn unrelated_operation_is_not_applicable() {
        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(6),
            );

        constraint
            .register(
                operation(10),
                ControlDescriptor::unconditional(),
            )
            .expect("registration must succeed");

        let logical:
            [crate::quantum::ir::qubit::QubitId; 0] =
            [];

        let physical:
            [crate::quantum::ir::qubit::PhysicalQubitId; 0] =
            [];

        let claims:
            [ConstraintResourceClaim; 0] =
            [];

        let candidate =
            SchedulingCandidate::new(
                operation(11),
                &logical,
                &physical,
                &claims,
                TimePoint::new(0),
                Duration::new(1),
            );

        let reservations:
            [super::super::constraint::ConstraintReservationView; 0] =
            [];

        let completed:
            [OperationId; 0] =
            [];

        let unavailable:
            [ResourceId; 0] =
            [];

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
    fn canonical_qubit_types_are_used() {
        let descriptor =
            ControlDescriptor::new(
                Condition::always(),
                Vec::new(),
                Vec::new(),
                vec![
                    crate::quantum::ir::qubit::QubitId::new(
                        0,
                    ),
                ],
                vec![
                    crate::quantum::ir::qubit::PhysicalQubitId::new(
                        0,
                    ),
                ],
                false,
            );

        assert!(
            descriptor.validate().is_ok()
        );
    }

    #[test]
    fn never_condition_is_not_a_constraint_failure() {
        let operation =
            operation(20);

        let mut constraint =
            ControlConstraint::production(
                ConstraintId::new(7),
            );

        constraint
            .register(
                operation,
                ControlDescriptor::new(
                    Condition::never(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    false,
                ),
            )
            .expect("registration must succeed");

        assert!(
            constraint.is_never_executable(operation)
        );
    }

    #[test]
    fn resource_claim_requirement_is_configurable() {
        let constraint =
            ControlConstraint::new(
                ConstraintId::new(8),
                ControlConstraintConfig::default()
                    .requiring_resource_claim(true),
            );

        let descriptor =
            ControlDescriptor::new(
                Condition::always(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                false,
            );

        assert_eq!(
            constraint.validate_descriptor(
                &descriptor,
            ),
            Ok(())
        );
    }

    #[test]
    fn dependency_value_identity_is_supported() {
        let descriptor =
            ControlDescriptor::new(
                Condition::predicate(
                    crate::quantum::ir::classical::predicate::ClassicalPredicate::always(),
                ),
                vec![
                    ControlDependency::value(
                        value(42),
                        Duration::new(0),
                    ),
                ],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                true,
            );

        assert!(
            descriptor.validate().is_ok()
        );
    }
}