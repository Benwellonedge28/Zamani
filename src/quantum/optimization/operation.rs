//! Zamani Quantum Optimization — Operation Semantics
//!
//! This module provides the optimization-layer semantic view of a canonical
//! Quantum IR gate.
//!
//! # Architectural boundary
//!
//! `operation.rs` MUST NOT define another quantum operation representation.
//!
//! The canonical operation representation is:
//!
//! ```text
//! quantum::ir::Gate
//!       │
//!       ▼
//! optimization::operation
//!       │
//!       ├── semantic classification
//!       ├── optimization properties
//!       ├── inverse relationships
//!       ├── parameter characteristics
//!       ├── resource characteristics
//!       └── transformation preconditions
//! ```
//!
//! The optimizer may inspect an operation through this module, but the
//! canonical semantic object remains `quantum::ir::Gate`.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - operation semantic classification;
//! - operation family classification;
//! - structural property queries;
//! - unitary/non-unitary classification;
//! - Clifford/non-Clifford classification;
//! - parameterization classification;
//! - controlled-operation classification;
//! - diagonal-operation classification;
//! - permutation/swap classification;
//! - measurement/reset/barrier classification;
//! - identity classification;
//! - inverse relationships;
//! - conservative self-inverse detection;
//! - parameter sensitivity metadata;
//! - optimization safety boundaries;
//! - deterministic operation descriptions;
//! - zero-allocation property inspection where practical.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - mutate gates;
//! - mutate circuits;
//! - perform rewrites;
//! - perform pattern matching;
//! - perform commutation analysis;
//! - calculate circuit depth;
//! - calculate global circuit cost;
//! - perform synthesis;
//! - perform routing;
//! - perform scheduling;
//! - communicate with hardware;
//! - execute quantum programs;
//! - perform semantic equivalence checking.
//!
//! Those responsibilities belong to other optimization/compiler layers.
//!
//! # Canonical IR integration
//!
//! This module consumes:
//!
//! ```text
//! crate::quantum::ir::Gate
//! crate::quantum::ir::GateKind
//! crate::quantum::ir::OperationId
//! crate::quantum::ir::Parameter
//! ```
//!
//! It intentionally does not depend on `QuantumCircuit`, because operation
//! semantics should remain usable for standalone gates, circuit analyses,
//! rewrite matching, diagnostics, and future operation views.
//!
//! # Stability contract
//!
//! Public types in this file are designed to be stable optimization contracts.
//! New optimization passes should depend on these semantic queries instead of
//! duplicating `GateKind` matching logic.
//!
//! # Safety
//!
//! This module uses no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features or external dependencies are required.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateKind,
    OperationId,
    Parameter,
};

// =============================================================================
// Operation family
// =============================================================================

/// Broad semantic family of a canonical quantum operation.
///
/// A family is intentionally less precise than [`OperationProperties`].
/// Callers that need multiple simultaneous properties should use
/// [`OperationProperties`] instead.
///
/// The classification is deterministic and hardware-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationFamily {
    /// Mathematical identity operation.
    Identity,

    /// Pauli operation such as X, Y, or Z.
    Pauli,

    /// Clifford operation.
    Clifford,

    /// Parameterized rotation or phase operation.
    Rotation,

    /// Controlled operation.
    Controlled,

    /// Qubit permutation operation such as SWAP.
    Permutation,

    /// General unitary operation that does not fit a more specific family.
    Unitary,

    /// Measurement operation.
    Measurement,

    /// Reset operation.
    Reset,

    /// Compiler synchronization/semantic boundary.
    Barrier,
}

impl OperationFamily {
    /// Returns whether this family represents a unitary operation.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::Pauli
                | Self::Clifford
                | Self::Rotation
                | Self::Controlled
                | Self::Permutation
                | Self::Unitary
        )
    }

    /// Returns whether this family is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(self) -> bool {
        !self.is_unitary()
    }
}

// =============================================================================
// Parameter semantics
// =============================================================================

/// Parameterization category of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterClass {
    /// The operation has no parameters.
    None,

    /// Every parameter is a concrete constant.
    Constant,

    /// At least one parameter is symbolic or contains a symbolic expression.
    Symbolic,
}

impl ParameterClass {
    /// Returns whether the operation has at least one parameter.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether all parameters are statically known constants.
    #[must_use]
    pub const fn is_constant(self) -> bool {
        matches!(self, Self::Constant)
    }

    /// Returns whether symbolic information is present.
    #[must_use]
    pub const fn is_symbolic(self) -> bool {
        matches!(self, Self::Symbolic)
    }
}

// =============================================================================
// Inverse semantics
// =============================================================================

/// Relationship between an operation and its mathematical inverse.
///
/// This is deliberately descriptive rather than transformational. It tells
/// an optimizer how an inverse relationship works without constructing a new
/// gate and without mutating the canonical IR.
///
/// Parameterized inverse construction belongs to a later rewrite/decomposition
/// layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InverseKind {
    /// The operation is its own inverse.
    SelfInverse,

    /// The operation has a fixed named inverse represented by another gate
    /// kind.
    Fixed(GateKind),

    /// The operation has the same gate kind but its parameters must be
    /// negated.
    NegateParameters,

    /// The inverse exists mathematically but cannot be represented by this
    /// metadata alone.
    General,

    /// No inverse exists because the operation is non-unitary.
    None,
}

impl InverseKind {
    /// Returns whether an inverse exists.
    #[must_use]
    pub const fn exists(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether the operation is self-inverse.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        matches!(self, Self::SelfInverse)
    }

    /// Returns a fixed inverse gate kind when one is directly representable.
    #[must_use]
    pub const fn fixed_kind(self) -> Option<GateKind> {
        match self {
            Self::Fixed(kind) => Some(kind),
            Self::SelfInverse
            | Self::NegateParameters
            | Self::General
            | Self::None => None,
        }
    }
}

// =============================================================================
// Optimization properties
// =============================================================================

/// Immutable semantic properties of a canonical quantum operation.
///
/// This is intentionally a compact value type. It contains no references and
/// can therefore be copied freely by analysis and rewrite infrastructure.
///
/// Properties may overlap. For example, CZ is simultaneously:
///
/// - unitary;
/// - Clifford;
/// - controlled;
/// - diagonal;
/// - two-qubit.
///
/// Callers must not treat these properties as mutually exclusive categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationProperties {
    unitary: bool,
    identity: bool,
    clifford: bool,
    non_clifford: bool,
    parameterized: bool,
    symbolic: bool,
    constant_parameters: bool,
    controlled: bool,
    diagonal: bool,
    permutation: bool,
    pauli: bool,
    rotation: bool,
    measurement: bool,
    reset: bool,
    barrier: bool,
    multi_qubit: bool,
    self_inverse: bool,
    has_classical_target: bool,
}

impl OperationProperties {
    /// Creates a complete property set.
    #[allow(clippy::too_many_arguments)]
    const fn new(
        unitary: bool,
        identity: bool,
        clifford: bool,
        non_clifford: bool,
        parameterized: bool,
        symbolic: bool,
        constant_parameters: bool,
        controlled: bool,
        diagonal: bool,
        permutation: bool,
        pauli: bool,
        rotation: bool,
        measurement: bool,
        reset: bool,
        barrier: bool,
        multi_qubit: bool,
        self_inverse: bool,
        has_classical_target: bool,
    ) -> Self {
        Self {
            unitary,
            identity,
            clifford,
            non_clifford,
            parameterized,
            symbolic,
            constant_parameters,
            controlled,
            diagonal,
            permutation,
            pauli,
            rotation,
            measurement,
            reset,
            barrier,
            multi_qubit,
            self_inverse,
            has_classical_target,
        }
    }

    /// Returns whether the operation is unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        self.unitary
    }

    /// Returns whether the operation is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(self) -> bool {
        !self.unitary
    }

    /// Returns whether the operation is an identity.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.identity
    }

    /// Returns whether the operation is Clifford.
    #[must_use]
    pub const fn is_clifford(self) -> bool {
        self.clifford
    }

    /// Returns whether the operation is conservatively classified as
    /// non-Clifford.
    #[must_use]
    pub const fn is_non_clifford(self) -> bool {
        self.non_clifford
    }

    /// Returns whether the operation has parameters.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.parameterized
    }

    /// Returns whether the operation contains symbolic parameters.
    #[must_use]
    pub const fn is_symbolic(self) -> bool {
        self.symbolic
    }

    /// Returns whether all operation parameters are concrete constants.
    #[must_use]
    pub const fn has_constant_parameters(self) -> bool {
        self.constant_parameters
    }

    /// Returns whether the operation is controlled.
    #[must_use]
    pub const fn is_controlled(self) -> bool {
        self.controlled
    }

    /// Returns whether the operation is diagonal in the computational basis.
    #[must_use]
    pub const fn is_diagonal(self) -> bool {
        self.diagonal
    }

    /// Returns whether the operation is a qubit permutation.
    #[must_use]
    pub const fn is_permutation(self) -> bool {
        self.permutation
    }

    /// Returns whether the operation is a Pauli operation.
    #[must_use]
    pub const fn is_pauli(self) -> bool {
        self.pauli
    }

    /// Returns whether the operation is a rotation/phase operation.
    #[must_use]
    pub const fn is_rotation(self) -> bool {
        self.rotation
    }

    /// Returns whether the operation is measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        self.measurement
    }

    /// Returns whether the operation is reset.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        self.reset
    }

    /// Returns whether the operation is a barrier.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        self.barrier
    }

    /// Returns whether the operation acts on more than one qubit.
    #[must_use]
    pub const fn is_multi_qubit(self) -> bool {
        self.multi_qubit
    }

    /// Returns whether the operation is conservatively known to be
    /// self-inverse.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        self.self_inverse
    }

    /// Returns whether the operation has a classical destination.
    #[must_use]
    pub const fn has_classical_target(self) -> bool {
        self.has_classical_target
    }
}

// =============================================================================
// Optimization safety
// =============================================================================

/// Conservative transformation-safety classification.
///
/// This allows optimization passes to make decisions without assuming that
/// every mathematically valid transformation is safe across every semantic
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationSafety {
    /// Ordinary local algebraic optimization is safe subject to the normal
    /// dependency analysis of the enclosing pass.
    SafeLocal,

    /// Optimization is possible, but movement/rewrite requires dependency or
    /// commutation analysis.
    RequiresDependencyAnalysis,

    /// The operation is a semantic boundary and must not be crossed by a
    /// generic local optimizer.
    SemanticBoundary,

    /// The operation is non-unitary and requires specialized handling.
    NonUnitary,

    /// The operation's semantics are known but a generic optimizer must defer
    /// transformation to a specialized pass.
    Specialized,
}

impl OptimizationSafety {
    /// Returns whether the operation may participate in ordinary local
    /// rewrites.
    #[must_use]
    pub const fn allows_local_rewrite(self) -> bool {
        matches!(
            self,
            Self::SafeLocal | Self::RequiresDependencyAnalysis
        )
    }

    /// Returns whether the operation is a semantic boundary.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
        matches!(self, Self::SemanticBoundary)
    }
}

// =============================================================================
// Operation descriptor
// =============================================================================

/// Stable, allocation-free semantic descriptor for one canonical gate.
///
/// This descriptor deliberately contains only scalar metadata. It is suitable
/// for:
///
/// - pattern matching;
/// - pass dispatch;
/// - analysis caches;
/// - statistics;
/// - debugging;
/// - diagnostics;
/// - cost-model classification.
///
/// It does not own the underlying gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationDescriptor {
    family: OperationFamily,
    parameter_class: ParameterClass,
    inverse: InverseKind,
    properties: OperationProperties,
    safety: OptimizationSafety,
    operand_count: usize,
    parameter_count: usize,
}

impl OperationDescriptor {
    /// Creates the descriptor for a canonical gate.
    #[must_use]
    pub fn from_gate(gate: &Gate) -> Self {
        let kind = gate.kind();
        let properties = classify_properties(gate);
        let parameter_class = classify_parameters(gate);
        let family = classify_family(gate);
        let inverse = classify_inverse(gate);
        let safety = classify_safety(gate);

        Self {
            family,
            parameter_class,
            inverse,
            properties,
            safety,
            operand_count: gate.qubits().len(),
            parameter_count: gate.parameters().len(),
        }
    }

    /// Returns the broad semantic family.
    #[must_use]
    pub const fn family(self) -> OperationFamily {
        self.family
    }

    /// Returns parameterization information.
    #[must_use]
    pub const fn parameter_class(self) -> ParameterClass {
        self.parameter_class
    }

    /// Returns inverse information.
    #[must_use]
    pub const fn inverse(self) -> InverseKind {
        self.inverse
    }

    /// Returns all operation properties.
    #[must_use]
    pub const fn properties(self) -> OperationProperties {
        self.properties
    }

    /// Returns optimization safety classification.
    #[must_use]
    pub const fn safety(self) -> OptimizationSafety {
        self.safety
    }

    /// Returns the number of logical qubit operands.
    #[must_use]
    pub const fn operand_count(self) -> usize {
        self.operand_count
    }

    /// Returns the number of parameters.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameter_count
    }

    /// Returns whether this operation is unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        self.properties.is_unitary()
    }

    /// Returns whether this operation is a semantic boundary.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
        self.safety.is_boundary()
    }
}

// =============================================================================
// Operation view
// =============================================================================

/// Borrowed optimization view of a canonical IR gate.
///
/// `OperationView` is the main type future optimization passes should use when
/// they need both the canonical gate and its semantic classification.
///
/// The underlying gate remains owned by the canonical IR/circuit.
///
/// No mutation is exposed through this type.
#[derive(Debug, Clone, Copy)]
pub struct OperationView<'a> {
    id: Option<OperationId>,
    gate: &'a Gate,
    descriptor: OperationDescriptor,
}

impl<'a> OperationView<'a> {
    /// Creates a view without an operation identity.
    ///
    /// This is useful for standalone gate analysis and unit tests.
    #[must_use]
    pub fn new(gate: &'a Gate) -> Self {
        Self {
            id: None,
            gate,
            descriptor: OperationDescriptor::from_gate(gate),
        }
    }

    /// Creates a view associated with a canonical IR operation identity.
    #[must_use]
    pub fn with_id(id: OperationId, gate: &'a Gate) -> Self {
        Self {
            id: Some(id),
            gate,
            descriptor: OperationDescriptor::from_gate(gate),
        }
    }

    /// Returns the optional canonical operation identity.
    #[must_use]
    pub const fn id(self) -> Option<OperationId> {
        self.id
    }

    /// Returns the underlying canonical gate.
    #[must_use]
    pub const fn gate(self) -> &'a Gate {
        self.gate
    }

    /// Returns the canonical gate kind.
    #[must_use]
    pub const fn kind(self) -> GateKind {
        self.gate.kind()
    }

    /// Returns the operation descriptor.
    #[must_use]
    pub const fn descriptor(self) -> OperationDescriptor {
        self.descriptor
    }

    /// Returns all logical operands.
    #[must_use]
    pub fn qubits(self) -> &'a [crate::quantum::ir::QubitId] {
        self.gate.qubits()
    }

    /// Returns all canonical parameters.
    #[must_use]
    pub fn parameters(self) -> &'a [Parameter] {
        self.gate.parameters()
    }

    /// Returns the classical target, if present.
    #[must_use]
    pub const fn classical_target(self) -> Option<usize> {
        self.gate.classical_target()
    }

    /// Returns the operation family.
    #[must_use]
    pub const fn family(self) -> OperationFamily {
        self.descriptor.family()
    }

    /// Returns the parameter class.
    #[must_use]
    pub const fn parameter_class(self) -> ParameterClass {
        self.descriptor.parameter_class()
    }

    /// Returns the inverse relationship.
    #[must_use]
    pub const fn inverse(self) -> InverseKind {
        self.descriptor.inverse()
    }

    /// Returns optimization safety.
    #[must_use]
    pub const fn safety(self) -> OptimizationSafety {
        self.descriptor.safety()
    }

    /// Returns whether this is unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        self.descriptor.is_unitary()
    }

    /// Returns whether this is an identity.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.descriptor.properties().is_identity()
    }

    /// Returns whether this is Clifford.
    #[must_use]
    pub const fn is_clifford(self) -> bool {
        self.descriptor.properties().is_clifford()
    }

    /// Returns whether this is conservatively non-Clifford.
    #[must_use]
    pub const fn is_non_clifford(self) -> bool {
        self.descriptor.properties().is_non_clifford()
    }

    /// Returns whether this is parameterized.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.descriptor.properties().is_parameterized()
    }

    /// Returns whether this contains symbolic parameters.
    #[must_use]
    pub const fn is_symbolic(self) -> bool {
        self.descriptor.properties().is_symbolic()
    }

    /// Returns whether this is controlled.
    #[must_use]
    pub const fn is_controlled(self) -> bool {
        self.descriptor.properties().is_controlled()
    }

    /// Returns whether this is diagonal.
    #[must_use]
    pub const fn is_diagonal(self) -> bool {
        self.descriptor.properties().is_diagonal()
    }

    /// Returns whether this is a permutation.
    #[must_use]
    pub const fn is_permutation(self) -> bool {
        self.descriptor.properties().is_permutation()
    }

    /// Returns whether this is a measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        self.descriptor.properties().is_measurement()
    }

    /// Returns whether this is reset.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        self.descriptor.properties().is_reset()
    }

    /// Returns whether this is a barrier.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        self.descriptor.properties().is_barrier()
    }

    /// Returns whether this is multi-qubit.
    #[must_use]
    pub const fn is_multi_qubit(self) -> bool {
        self.descriptor.properties().is_multi_qubit()
    }

    /// Returns whether this is self-inverse.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        self.descriptor.properties().is_self_inverse()
    }

    /// Returns whether this operation has a classical target.
    #[must_use]
    pub const fn has_classical_target(self) -> bool {
        self.descriptor.properties().has_classical_target()
    }

    /// Returns whether the operation can safely participate in a generic
    /// local rewrite.
    ///
    /// This is intentionally conservative. A `true` result does not authorize
    /// movement across other operations; dependency and commutation analysis
    /// remain mandatory for such transformations.
    #[must_use]
    pub const fn allows_local_rewrite(self) -> bool {
        self.safety().allows_local_rewrite()
    }

    /// Returns whether this operation has no symbolic parameters.
    #[must_use]
    pub const fn is_parameter_bound(self) -> bool {
        !self.is_symbolic()
    }

    /// Returns a human-readable canonical operation name.
    ///
    /// This is intended for diagnostics and stable internal rule labels, not
    /// serialization of the complete IR.
    #[must_use]
    pub const fn name(self) -> &'static str {
        operation_name(self.kind())
    }
}

// =============================================================================
// Public classification functions
// =============================================================================

/// Classifies a canonical gate into an [`OperationDescriptor`].
#[must_use]
pub fn describe(gate: &Gate) -> OperationDescriptor {
    OperationDescriptor::from_gate(gate)
}

/// Creates an immutable optimization view over a canonical gate.
#[must_use]
pub fn view(gate: &Gate) -> OperationView<'_> {
    OperationView::new(gate)
}

/// Creates an immutable optimization view with a canonical operation ID.
#[must_use]
pub fn view_with_id(id: OperationId, gate: &Gate) -> OperationView<'_> {
    OperationView::with_id(id, gate)
}

/// Returns the semantic family of a canonical gate.
#[must_use]
pub fn family(gate: &Gate) -> OperationFamily {
    describe(gate).family()
}

/// Returns the inverse relationship of a canonical gate.
#[must_use]
pub fn inverse_kind(gate: &Gate) -> InverseKind {
    describe(gate).inverse()
}

/// Returns all optimization properties of a canonical gate.
#[must_use]
pub fn properties(gate: &Gate) -> OperationProperties {
    describe(gate).properties()
}

/// Returns the optimization safety classification of a canonical gate.
#[must_use]
pub fn safety(gate: &Gate) -> OptimizationSafety {
    describe(gate).safety()
}

// =============================================================================
// Internal classification
// =============================================================================

fn classify_parameters(gate: &Gate) -> ParameterClass {
    let parameters = gate.parameters();

    if parameters.is_empty() {
        return ParameterClass::None;
    }

    if parameters.iter().all(Parameter::is_constant) {
        ParameterClass::Constant
    } else {
        ParameterClass::Symbolic
    }
}

fn classify_family(gate: &Gate) -> OperationFamily {
    let kind = gate.kind();

    if kind == GateKind::I {
        return OperationFamily::Identity;
    }

    if kind.is_measurement() {
        return OperationFamily::Measurement;
    }

    if kind.is_reset() {
        return OperationFamily::Reset;
    }

    if kind.is_barrier() {
        return OperationFamily::Barrier;
    }

    if is_pauli_kind(kind) {
        return OperationFamily::Pauli;
    }

    if is_rotation_kind(kind) {
        return OperationFamily::Rotation;
    }

    if is_permutation_kind(kind) {
        return OperationFamily::Permutation;
    }

    if is_controlled_kind(kind) {
        return OperationFamily::Controlled;
    }

    if kind.is_clifford() {
        return OperationFamily::Clifford;
    }

    OperationFamily::Unitary
}

fn classify_properties(gate: &Gate) -> OperationProperties {
    let kind = gate.kind();
    let parameter_class = classify_parameters(gate);

    let unitary = kind.is_unitary();
    let identity = is_identity_kind(kind);
    let clifford = value_sensitive_clifford(gate);
    let non_clifford = unitary && !clifford && !identity;
    let parameterized = parameter_class.is_parameterized();
    let symbolic = parameter_class.is_symbolic();
    let constant_parameters = parameter_class.is_constant();
    let controlled = is_controlled_kind(kind);
    let diagonal = is_diagonal_kind(kind);
    let permutation = is_permutation_kind(kind);
    let pauli = is_pauli_kind(kind);
    let rotation = is_rotation_kind(kind);
    let measurement = kind.is_measurement();
    let reset = kind.is_reset();
    let barrier = kind.is_barrier();
    let multi_qubit = gate.qubits().len() > 1;
    let self_inverse = conservative_self_inverse(gate);
    let has_classical_target = gate.classical_target().is_some();

    OperationProperties::new(
        unitary,
        identity,
        clifford,
        non_clifford,
        parameterized,
        symbolic,
        constant_parameters,
        controlled,
        diagonal,
        permutation,
        pauli,
        rotation,
        measurement,
        reset,
        barrier,
        multi_qubit,
        self_inverse,
        has_classical_target,
    )
}

fn classify_inverse(gate: &Gate) -> InverseKind {
    let kind = gate.kind();

    if !kind.is_unitary() {
        return InverseKind::None;
    }

    if kind.is_self_inverse() {
        return InverseKind::SelfInverse;
    }

    match kind {
        GateKind::S => InverseKind::Fixed(GateKind::Sdg),
        GateKind::Sdg => InverseKind::Fixed(GateKind::S),

        GateKind::T => InverseKind::Fixed(GateKind::Tdg),
        GateKind::Tdg => InverseKind::Fixed(GateKind::T),

        GateKind::V => InverseKind::Fixed(GateKind::Vdg),
        GateKind::Vdg => InverseKind::Fixed(GateKind::V),

        GateKind::RX
        | GateKind::RY
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRX
        | GateKind::CRY
        | GateKind::CRZ => InverseKind::NegateParameters,

        // The inverse of U2/U3 is representable as another U gate but
        // requires parameter algebra rather than simple metadata.
        GateKind::U2 | GateKind::U3 => InverseKind::General,

        GateKind::I
        | GateKind::X
        | GateKind::Y
        | GateKind::Z
        | GateKind::H
        | GateKind::CX
        | GateKind::CY
        | GateKind::CZ
        | GateKind::CH
        | GateKind::SWAP
        | GateKind::ISWAP
        | GateKind::ECR
        | GateKind::CCX
        | GateKind::CSWAP
        | GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset => InverseKind::General,
    }
}

fn classify_safety(gate: &Gate) -> OptimizationSafety {
    let kind = gate.kind();

    if kind.is_measurement() || kind.is_reset() {
        return OptimizationSafety::NonUnitary;
    }

    if kind.is_barrier() {
        return OptimizationSafety::SemanticBoundary;
    }

    if kind.is_parameterized() {
        return OptimizationSafety::RequiresDependencyAnalysis;
    }

    if kind.is_unitary() {
        if is_pauli_kind(kind)
            || kind.is_self_inverse()
            || is_identity_kind(kind)
        {
            return OptimizationSafety::SafeLocal;
        }

        return OptimizationSafety::RequiresDependencyAnalysis;
    }

    OptimizationSafety::Specialized
}

// =============================================================================
// Gate-family predicates
// =============================================================================

fn is_identity_kind(kind: GateKind) -> bool {
    matches!(kind, GateKind::I)
}

fn is_pauli_kind(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::X | GateKind::Y | GateKind::Z
    )
}

fn is_rotation_kind(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
    )
}

fn is_controlled_kind(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
            | GateKind::CCX
            | GateKind::CSWAP
    )
}

fn is_permutation_kind(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::CSWAP
    )
}

fn is_diagonal_kind(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::I
            | GateKind::Z
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::CZ
            | GateKind::CRZ
    )
}

// =============================================================================
// Value-sensitive classification
// =============================================================================

/// Returns whether a gate is Clifford.
///
/// For fixed gate kinds this uses the canonical gate classification.
///
/// Parameterized gates are classified as Clifford only when their concrete
/// parameter values can be proven to represent a Clifford operation exactly.
///
/// This function is deliberately conservative. It never uses an arbitrary
/// floating-point tolerance to declare two angles mathematically equivalent.
///
/// That matters because an optimizer must never turn an approximate numerical
/// coincidence into an exact semantic rewrite.
fn value_sensitive_clifford(gate: &Gate) -> bool {
    let kind = gate.kind();

    if kind.is_clifford() {
        return true;
    }

    match kind {
        GateKind::RX
        | GateKind::RY
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRX
        | GateKind::CRY
        | GateKind::CRZ => {
            let parameters = gate.parameters();

            if parameters.len() != 1 {
                return false;
            }

            match parameters[0].as_constant() {
                Some(angle) => is_clifford_angle(angle),
                None => false,
            }
        }

        // U2/U3 have multiple parameters and require a complete matrix-level
        // or symbolic-normal-form analysis. This module intentionally does not
        // attempt that analysis.
        GateKind::U2 | GateKind::U3 => false,

        _ => false,
    }
}

/// Conservative exact-angle Clifford test.
///
/// The test recognizes common exact floating-point representations of
/// multiples of pi/2 and pi/4 used by standard library constants and compiler
/// generated parameters.
///
/// It deliberately does not use a broad epsilon because approximate equality
/// is not sufficient for an exact compiler transformation.
///
/// `rem_euclid` is stable on Rust versions predating the required Rust 1.97.1
/// target. 4
fn is_clifford_angle(angle: f64) -> bool {
    if !angle.is_finite() {
        return false;
    }

    let half_pi = std::f64::consts::FRAC_PI_2;
    let quarter_pi = std::f64::consts::FRAC_PI_4;

    // Exact common representations.
    if angle == 0.0
        || angle == half_pi
        || angle == -half_pi
        || angle == quarter_pi
        || angle == -quarter_pi
    {
        return true;
    }

    // The angle is Clifford when it is an integer multiple of pi/2.
    //
    // The comparison remains exact with respect to the computed floating
    // representation: no epsilon-based approximation is introduced.
    let normalized = angle.rem_euclid(2.0 * std::f64::consts::PI);
    let quotient = normalized / half_pi;
    quotient.fract() == 0.0
}

/// Returns whether a concrete parameter is exactly an identity angle.
///
/// This helper is intentionally strict. It is suitable for recognizing
/// explicit zero parameters but should not be used as a general floating-point
/// equality relation.
#[must_use]
pub fn is_exact_zero_parameter(parameter: &Parameter) -> bool {
    matches!(parameter.as_constant(), Some(value) if value == 0.0)
}

// =============================================================================
// Self-inverse classification
// =============================================================================

/// Returns whether a canonical gate is conservatively known to be
/// self-inverse.
///
/// For parameterized rotations this does not claim self-inversion merely from
/// the gate kind. A parameter value may make a specific rotation self-inverse,
/// but that requires value-sensitive reasoning and belongs to a specialized
/// algebraic pass.
fn conservative_self_inverse(gate: &Gate) -> bool {
    let kind = gate.kind();

    if kind.is_self_inverse() {
        return true;
    }

    match kind {
        GateKind::RX
        | GateKind::RY
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRX
        | GateKind::CRY
        | GateKind::CRZ => {
            // R(axis, 0) is identity and therefore self-inverse, but only when
            // the zero is explicitly known.
            if gate.parameters().len() != 1 {
                return false;
            }

            is_exact_zero_parameter(&gate.parameters()[0])
        }

        _ => false,
    }
}

// =============================================================================
// Stable operation names
// =============================================================================

/// Returns a deterministic textual name for a canonical gate kind.
///
/// These names are compiler-internal semantic labels. They are intentionally
/// independent of source-language syntax and hardware-native spellings.
#[must_use]
pub const fn operation_name(kind: GateKind) -> &'static str {
    match kind {
        GateKind::I => "identity",

        GateKind::X => "x",
        GateKind::Y => "y",
        GateKind::Z => "z",
        GateKind::H => "h",
        GateKind::S => "s",
        GateKind::Sdg => "sdg",
        GateKind::T => "t",
        GateKind::Tdg => "tdg",
        GateKind::V => "v",
        GateKind::Vdg => "vdg",

        GateKind::RX => "rx",
        GateKind::RY => "ry",
        GateKind::RZ => "rz",
        GateKind::Phase => "phase",
        GateKind::U1 => "u1",
        GateKind::U2 => "u2",
        GateKind::U3 => "u3",

        GateKind::CX => "cx",
        GateKind::CY => "cy",
        GateKind::CZ => "cz",
        GateKind::CH => "ch",
        GateKind::SWAP => "swap",
        GateKind::ISWAP => "iswap",
        GateKind::ECR => "ecr",

        GateKind::CRX => "crx",
        GateKind::CRY => "cry",
        GateKind::CRZ => "crz",

        GateKind::CCX => "ccx",
        GateKind::CSWAP => "cswap",

        GateKind::Measure => "measure",
        GateKind::Barrier => "barrier",
        GateKind::Reset => "reset",
    }
}

// =============================================================================
// Public convenience predicates
// =============================================================================

/// Returns whether the gate is an identity operation.
#[must_use]
pub fn is_identity(gate: &Gate) -> bool {
    properties(gate).is_identity()
}

/// Returns whether the gate is a measurement.
#[must_use]
pub fn is_measurement(gate: &Gate) -> bool {
    properties(gate).is_measurement()
}

/// Returns whether the gate is a reset.
#[must_use]
pub fn is_reset(gate: &Gate) -> bool {
    properties(gate).is_reset()
}

/// Returns whether the gate is a barrier.
#[must_use]
pub fn is_barrier(gate: &Gate) -> bool {
    properties(gate).is_barrier()
}

/// Returns whether the gate is controlled.
#[must_use]
pub fn is_controlled(gate: &Gate) -> bool {
    properties(gate).is_controlled()
}

/// Returns whether the gate is diagonal.
#[must_use]
pub fn is_diagonal(gate: &Gate) -> bool {
    properties(gate).is_diagonal()
}

/// Returns whether the gate is Clifford under the conservative classification
/// rules of this module.
#[must_use]
pub fn is_clifford(gate: &Gate) -> bool {
    properties(gate).is_clifford()
}

/// Returns whether the gate is conservatively non-Clifford.
#[must_use]
pub fn is_non_clifford(gate: &Gate) -> bool {
    properties(gate).is_non_clifford()
}

/// Returns whether the gate is parameterized.
#[must_use]
pub fn is_parameterized(gate: &Gate) -> bool {
    properties(gate).is_parameterized()
}

/// Returns whether the gate contains symbolic parameters.
#[must_use]
pub fn is_symbolic(gate: &Gate) -> bool {
    properties(gate).is_symbolic()
}

/// Returns whether the gate is known to be self-inverse.
#[must_use]
pub fn is_self_inverse(gate: &Gate) -> bool {
    properties(gate).is_self_inverse()
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for OperationFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Identity => "identity",
            Self::Pauli => "pauli",
            Self::Clifford => "clifford",
            Self::Rotation => "rotation",
            Self::Controlled => "controlled",
            Self::Permutation => "permutation",
            Self::Unitary => "unitary",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Barrier => "barrier",
        };

        f.write_str(value)
    }
}

impl fmt::Display for ParameterClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::None => "none",
            Self::Constant => "constant",
            Self::Symbolic => "symbolic",
        };

        f.write_str(value)
    }
}

impl fmt::Display for OptimizationSafety {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::SafeLocal => "safe-local",
            Self::RequiresDependencyAnalysis => "requires-dependency-analysis",
            Self::SemanticBoundary => "semantic-boundary",
            Self::NonUnitary => "non-unitary",
            Self::Specialized => "specialized",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parameter(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("test parameter must be finite")
    }

    fn gate(
        kind: GateKind,
        qubits: usize,
        parameters: Vec<Parameter>,
    ) -> Gate {
        let operands = (0..qubits)
            .map(crate::quantum::ir::QubitId::new)
            .collect::<Vec<_>>();

        Gate::new(
            kind,
            operands,
            parameters,
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn identity_is_classified_correctly() {
        let gate = gate(GateKind::I, 1, Vec::new());
        let descriptor = describe(&gate);

        assert_eq!(
            descriptor.family(),
            OperationFamily::Identity
        );
        assert!(descriptor.properties().is_identity());
        assert!(descriptor.properties().is_unitary());
        assert!(descriptor.properties().is_self_inverse());
    }

    #[test]
    fn pauli_is_classified_correctly() {
        let gate = gate(GateKind::X, 1, Vec::new());
        let descriptor = describe(&gate);

        assert_eq!(
            descriptor.family(),
            OperationFamily::Pauli
        );
        assert!(descriptor.properties().is_pauli());
        assert!(descriptor.properties().is_clifford());
        assert!(descriptor.properties().is_unitary());
        assert!(descriptor.properties().is_self_inverse());
    }

    #[test]
    fn controlled_gate_is_classified_correctly() {
        let gate = gate(GateKind::CX, 2, Vec::new());
        let descriptor = describe(&gate);

        assert_eq!(
            descriptor.family(),
            OperationFamily::Controlled
        );
        assert!(descriptor.properties().is_controlled());
        assert!(descriptor.properties().is_clifford());
        assert!(descriptor.properties().is_multi_qubit());
        assert!(descriptor.properties().is_self_inverse());
    }

    #[test]
    fn diagonal_gate_is_classified_correctly() {
        let gate = gate(GateKind::CZ, 2, Vec::new());
        let descriptor = describe(&gate);

        assert!(descriptor.properties().is_diagonal());
        assert!(descriptor.properties().is_controlled());
        assert!(descriptor.properties().is_clifford());
    }

    #[test]
    fn rotation_parameterization_is_classified() {
        let gate = gate(
            GateKind::RZ,
            1,
            vec![parameter(0.5)],
        );

        let descriptor = describe(&gate);

        assert_eq!(
            descriptor.family(),
            OperationFamily::Rotation
        );
        assert_eq!(
            descriptor.parameter_class(),
            ParameterClass::Constant
        );
        assert!(descriptor.properties().is_parameterized());
        assert!(descriptor.properties().has_constant_parameters());
    }

    #[test]
    fn symbolic_rotation_is_detected() {
        let symbolic = Parameter::symbol("theta")
            .expect("symbol should be valid");

        let gate = gate(
            GateKind::RZ,
            1,
            vec![symbolic],
        );

        let descriptor = describe(&gate);

        assert_eq!(
            descriptor.parameter_class(),
            ParameterClass::Symbolic
        );
        assert!(descriptor.properties().is_symbolic());
    }

    #[test]
    fn fixed_inverse_pairs_are_correct() {
        let s = gate(GateKind::S, 1, Vec::new());
        let sdg = gate(GateKind::Sdg, 1, Vec::new());

        assert_eq!(
            inverse_kind(&s),
            InverseKind::Fixed(GateKind::Sdg)
        );

        assert_eq!(
            inverse_kind(&sdg),
            InverseKind::Fixed(GateKind::S)
        );
    }

    #[test]
    fn_t_inverse_pair_is_correct() {
        let t = gate(GateKind::T, 1, Vec::new());
        let tdg = gate(GateKind::Tdg, 1, Vec::new());

        assert_eq!(
            inverse_kind(&t),
            InverseKind::Fixed(GateKind::Tdg)
        );

        assert_eq!(
            inverse_kind(&tdg),
            InverseKind::Fixed(GateKind::T)
        );
    }

    #[test]
    fn rotation_inverse_requires_parameter_negation() {
        let gate = gate(
            GateKind::RX,
            1,
            vec![parameter(0.25)],
        );

        assert_eq!(
            inverse_kind(&gate),
            InverseKind::NegateParameters
        );
    }

    #[test]
    fn measurement_is_a_non_unitary_boundary() {
        let gate = gate(
            GateKind::Measure,
            1,
            Vec::new(),
        );

        let descriptor = describe(&gate);

        assert!(!descriptor.properties().is_unitary());
        assert!(descriptor.properties().is_measurement());
        assert_eq!(
            descriptor.safety(),
            OptimizationSafety::NonUnitary
        );
    }

    #[test]
    fn reset_is_a_non_unitary_boundary() {
        let gate = gate(
            GateKind::Reset,
            1,
            Vec::new(),
        );

        let descriptor = describe(&gate);

        assert!(!descriptor.properties().is_unitary());
        assert!(descriptor.properties().is_reset());
        assert_eq!(
            descriptor.safety(),
            OptimizationSafety::NonUnitary
        );
    }

    #[test]
    fn barrier_is_a_semantic_boundary() {
        let gate = gate(
            GateKind::Barrier,
            1,
            Vec::new(),
        );

        let descriptor = describe(&gate);

        assert!(!descriptor.properties().is_unitary());
        assert!(descriptor.properties().is_barrier());
        assert!(descriptor.safety().is_boundary());
        assert!(!descriptor.safety().allows_local_rewrite());
    }

    #[test]
    fn operation_view_does_not_own_the_gate() {
        let gate = gate(GateKind::H, 1, Vec::new());
        let view = OperationView::new(&gate);

        assert_eq!(view.kind(), GateKind::H);
        assert_eq!(view.name(), "h");
        assert!(view.is_clifford());
        assert!(view.is_self_inverse());
    }

    #[test]
    fn operation_view_can_carry_canonical_identity() {
        let gate = gate(GateKind::X, 1, Vec::new());
        let id = OperationId::new(17);

        let view = OperationView::with_id(id, &gate);

        assert_eq!(view.id(), Some(id));
        assert_eq!(view.kind(), GateKind::X);
    }

    #[test]
    fn operation_names_are_stable() {
        assert_eq!(
            operation_name(GateKind::CX),
            "cx"
        );
        assert_eq!(
            operation_name(GateKind::RZ),
            "rz"
        );
        assert_eq!(
            operation_name(GateKind::Measure),
            "measure"
        );
    }

    #[test]
    fn zero_parameter_is_recognized_exactly() {
        let parameter = parameter(0.0);

        assert!(
            is_exact_zero_parameter(&parameter)
        );
    }

    #[test]
    fn nonzero_parameter_is_not_zero() {
        let parameter = parameter(0.25);

        assert!(
            !is_exact_zero_parameter(&parameter)
        );
    }

    #[test]
    fn common_clifford_angles_are_recognized() {
        assert!(
            is_clifford_angle(
                std::f64::consts::FRAC_PI_2
            )
        );

        assert!(
            is_clifford_angle(
                -std::f64::consts::FRAC_PI_2
            )
        );

        assert!(
            is_clifford_angle(
                std::f64::consts::FRAC_PI_4
            )
        );
    }

    #[test]
    fn arbitrary_angle_is_not_assumed_clifford() {
        assert!(
            !is_clifford_angle(0.123456789)
        );
    }

    #[test]
    fn parameter_class_none_for_non_parameterized_gate() {
        let gate = gate(GateKind::H, 1, Vec::new());

        assert_eq!(
            classify_parameters(&gate),
            ParameterClass::None
        );
    }

    #[test]
    fn parameter_class_constant_for_bound_gate() {
        let gate = gate(
            GateKind::RZ,
            1,
            vec![parameter(1.0)],
        );

        assert_eq!(
            classify_parameters(&gate),
            ParameterClass::Constant
        );
    }

    #[test]
    fn parameter_class_symbolic_for_symbolic_gate() {
        let parameter = Parameter::symbol("theta")
            .expect("symbol should be valid");

        let gate = gate(
            GateKind::RZ,
            1,
            vec![parameter],
        );

        assert_eq!(
            classify_parameters(&gate),
            ParameterClass::Symbolic
        );
    }

    #[test]
    fn iswap_is_permutation_family() {
        let gate = gate(
            GateKind::ISWAP,
            2,
            Vec::new(),
        );

        assert_eq!(
            family(&gate),
            OperationFamily::Permutation
        );
        assert!(
            properties(&gate).is_permutation()
        );
    }

    #[test]
    fn t_is_non_clifford() {
        let gate = gate(
            GateKind::T,
            1,
            Vec::new(),
        );

        assert!(
            properties(&gate).is_non_clifford()
        );
        assert!(
            !properties(&gate).is_clifford()
        );
    }

    #[test]
    fn s_is_clifford() {
        let gate = gate(
            GateKind::S,
            1,
            Vec::new(),
        );

        assert!(
            properties(&gate).is_clifford()
        );
        assert!(
            !properties(&gate).is_non_clifford()
        );
    }

    #[test]
    fn operation_descriptor_is_copyable() {
        let gate = gate(GateKind::H, 1, Vec::new());
        let descriptor = describe(&gate);
        let copied = descriptor;

        assert_eq!(
            descriptor,
            copied
        );
    }
}