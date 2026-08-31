//! Zamani Quantum IR — Universal Initialization / State-Preparation Semantics
//!
//! Canonical, hardware-independent representation of quantum initialization
//! and state-preparation intent.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This file answers:
//!
//!     "What quantum state does the program require these logical resources
//!      to be prepared in before subsequent computation?"
//!
//! It does NOT answer:
//!
//! - which physical qubit is used;
//! - which hardware channel is used;
//! - which pulse implements the preparation;
//! - which calibration is used;
//! - which native gate decomposition is selected;
//! - which simulator representation is used;
//! - which state-vector storage format is used;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how optimization is performed;
//! - how QEC encodes the state;
//! - how a backend executes the preparation;
//! - how a QPU allocates physical resources;
//! - how source-language syntax is parsed.
//!
//! Those responsibilities belong to downstream layers.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program is written once at the semantic level.
//!
//! Initialization therefore contains:
//!
//! - no fixed number of qubits;
//! - no fixed register size;
//! - no fixed state-vector size;
//! - no fixed hardware architecture;
//! - no fixed topology;
//! - no vendor-specific initialization primitive;
//! - no simulator implementation.
//!
//! A program containing one target and a program containing an arbitrarily
//! large finite target set use the same representation.
//!
//! Concrete limits belong to explicit resource/security policies.
//!
//! ============================================================================
//! CANONICAL QUBIT IDENTITY
//! ============================================================================
//!
//! All logical initialization targets use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This file deliberately does NOT define another qubit identifier.
//!
//! `QubitId` means logical program identity.
//!
//! Physical placement belongs to mapping/routing/hardware.
//!
//! ============================================================================
//! INITIALIZATION VS RESET
//! ============================================================================
//!
//! `reset.rs` owns the canonical operation:
//!
//!     q -> |0>
//!
//! `initialization.rs` owns richer state-preparation semantics:
//!
//!     q -> |1>
//!     q -> |+>
//!     q -> arbitrary basis state
//!     q -> arbitrary state vector
//!     q -> product state
//!     q -> stabilizer state
//!     q -> graph state
//!     q -> encoded logical state
//!     q -> custom/extension-defined state
//!
//! Reset is therefore a special semantic operation, not a replacement for
//! this initialization model.
//!
//! ============================================================================
//! NO SIMULATOR STATE
//! ============================================================================
//!
//! A `StateVector` stored here is a PROGRAM INPUT / STATE-PREPARATION
//! DESCRIPTION.
//!
//! It is NOT:
//!
//! - simulator memory;
//! - a mutable wavefunction;
//! - a density-matrix simulator;
//! - a probability engine;
//! - an execution state;
//! - a QPU memory buffer.
//!
//! The simulator/backend may consume the description and construct its own
//! execution representation.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module deliberately avoids calculations such as:
//!
//!     1usize << fixed_qubit_count
//!
//! as architectural limits.
//!
//! When a representation intrinsically requires a concrete materialized
//! state-vector, its size is validated against the supplied data and the host
//! representation. The IR itself does not declare a maximum number of qubits.
//!
//! Sparse/symbolic/custom representations should be used when a dense
//! state-vector would be inappropriate.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Target ordering supplied by the source is preserved.
//!
//! A deterministic canonical target ordering is also provided for serializers,
//! hashing and analysis.
//!
//! The semantic state-preparation specification is immutable after construction.
//!
//! ============================================================================
//! SERIALIZATION CONTRACT
//! ============================================================================
//!
//! Stable schema identity:
//!
//!     zamani.quantum.ir.quantum.initialization
//!
//! Schema version:
//!
//!     1
//!
//! Serialization belongs to `serialization.rs`.
//!
//! This file only guarantees deterministic field/accessor semantics.
//!
//! ============================================================================
//! HASHING CONTRACT
//! ============================================================================
//!
//! This file does not implement cryptographic hashing.
//!
//! Canonical hashing belongs to the IR hashing layer.
//!
//! A canonical hash should include:
//!
//! - schema identity;
//! - schema version;
//! - canonical target set;
//! - preparation specification;
//! - semantically relevant preparation metadata.
//!
//! It must exclude:
//!
//! - memory addresses;
//! - process IDs;
//! - allocation addresses;
//! - nondeterministic metadata.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `quantum::ir::qubit`
//!     Supplies canonical `QubitId`.
//!
//! `quantum::ir::reset`
//!     Owns canonical reset semantics. Initialization must not redefine reset.
//!
//! `quantum::ir::instruction`
//!     May embed/reference `Initialization` as an initialization instruction.
//!
//! `quantum::ir::operation`
//!     Owns the universal operation container.
//!
//! `quantum::ir::program`
//!     Owns program ordering, declarations and namespaces.
//!
//! `quantum::ir::validation`
//!     Performs whole-program namespace and semantic validation.
//!
//! `quantum::ir::serialization`
//!     Owns persistence and canonical encoding.
//!
//! `quantum::ir::hash`
//!     Owns canonical content hashing.
//!
//! `quantum::ir::analysis`
//!     Reads targets and preparation dependencies.
//!
//! `quantum::ir::mapping`
//!     Resolves logical targets to physical resources.
//!
//! `quantum::ir::scheduling`
//!     Determines execution timing.
//!
//! `quantum::hardware`
//!     Determines whether and how a target supports a preparation.
//!
//! `quantum::simulator`
//!     Interprets preparation semantics.
//!
//! `quantum::qec`
//!     May consume encoded/logical preparation semantics.
//!
//! `quantum::backend`
//!     Lowers the semantic preparation into target-specific execution.
//!
//! ============================================================================
//! FILE-COMPLETION GUARANTEE
//! ============================================================================
//!
//! This file owns:
//!
//! - initialization schema identity;
//! - logical target representation;
//! - deterministic target access;
//! - preparation-state vocabulary;
//! - computational basis preparation;
//! - single-qubit eigenstate preparation;
//! - product-state preparation;
//! - dense state-vector preparation description;
//! - density-operator preparation description;
//! - stabilizer preparation description;
//! - graph-state preparation description;
//! - encoded/logical preparation references;
//! - custom/extension preparation references;
//! - local validation;
//! - normalization validation;
//! - duplicate-target detection;
//! - namespace validation;
//! - local errors;
//! - local result types;
//! - local tests.
//!
//! Later modules should consume this contract rather than changing the meaning
//! of canonical initialization.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable semantic schema identifier for initialization.
pub const INITIALIZATION_SCHEMA_ID: &str =
    "zamani.quantum.ir.quantum.initialization";

/// Major semantic version of this initialization contract.
pub const INITIALIZATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by initialization construction and local validation.
pub type InitializationResult<T> = Result<T, InitializationError>;

// ============================================================================
// COMPLEX AMPLITUDE
// ============================================================================

/// Hardware-independent complex amplitude.
///
/// This is a semantic value used to describe a requested state.
///
/// It does not represent simulator storage.
///
/// `f64` is used because this IR module must remain dependency-free and
/// compatible with stable Rust 1.97.1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexAmplitude {
    real: f64,
    imaginary: f64,
}

impl ComplexAmplitude {
    /// Creates a complex amplitude.
    #[must_use]
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    /// Creates a real amplitude.
    #[must_use]
    pub const fn real(value: f64) -> Self {
        Self {
            real: value,
            imaginary: 0.0,
        }
    }

    /// Returns the real component.
    #[must_use]
    pub const fn real_part(self) -> f64 {
        self.real
    }

    /// Returns the imaginary component.
    #[must_use]
    pub const fn imaginary_part(self) -> f64 {
        self.imaginary
    }

    /// Returns the squared magnitude.
    #[must_use]
    pub fn norm_squared(self) -> f64 {
        self.real.mul_add(self.real, self.imaginary * self.imaginary)
    }

    /// Returns whether both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.real.is_finite() && self.imaginary.is_finite()
    }

    /// Returns whether this is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.real == 0.0 && self.imaginary == 0.0
    }
}

// ============================================================================
// SINGLE-QUBIT STATE
// ============================================================================

/// Canonical single-qubit preparation states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SingleQubitState {
    /// Computational basis |0>.
    Zero,

    /// Computational basis |1>.
    One,

    /// Pauli-X eigenstate |+>.
    Plus,

    /// Pauli-X eigenstate |->.
    Minus,

    /// Pauli-Y positive eigenstate |+i>.
    PlusI,

    /// Pauli-Y negative eigenstate |-i>.
    MinusI,
}

impl SingleQubitState {
    /// Stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::PlusI => "plus_i",
            Self::MinusI => "minus_i",
        }
    }
}

impl fmt::Display for SingleQubitState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// BASIS BIT
// ============================================================================

/// One computational-basis value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BasisBit {
    /// Logical |0>.
    Zero,

    /// Logical |1>.
    One,
}

impl BasisBit {
    /// Returns the numeric basis value.
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }
}

impl From<bool> for BasisBit {
    fn from(value: bool) -> Self {
        if value {
            Self::One
        } else {
            Self::Zero
        }
    }
}

// ============================================================================
// BASIS STATE
// ============================================================================

/// Computational-basis state for an arbitrary finite logical target set.
///
/// The bit sequence is ordered exactly like the initialization targets.
///
/// For example:
///
/// ```text
/// targets = [q0, q1, q2]
/// bits    = [One, Zero, One]
/// ```
///
/// means:
///
/// ```text
/// q0 -> |1>
/// q1 -> |0>
/// q2 -> |1>
/// ```
///
/// This is a semantic description and does not allocate simulator state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComputationalBasisState {
    bits: Vec<BasisBit>,
}

impl ComputationalBasisState {
    /// Creates a computational-basis state.
    pub fn new(bits: Vec<BasisBit>) -> Self {
        Self { bits }
    }

    /// Creates a basis state from booleans.
    pub fn from_bools<I>(bits: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Self {
            bits: bits.into_iter().map(BasisBit::from).collect(),
        }
    }

    /// Creates the all-zero state for `count` targets.
    pub fn zeros(count: usize) -> Self {
        Self {
            bits: vec![BasisBit::Zero; count],
        }
    }

    /// Creates the all-one state for `count` targets.
    pub fn ones(count: usize) -> Self {
        Self {
            bits: vec![BasisBit::One; count],
        }
    }

    /// Number of logical qubits represented.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the state has no basis bits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns the basis bits in semantic order.
    #[must_use]
    pub fn bits(&self) -> &[BasisBit] {
        &self.bits
    }

    /// Returns one basis bit.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BasisBit> {
        self.bits.get(index).copied()
    }
}

// ============================================================================
// STATE VECTOR
// ============================================================================

/// Dense pure-state preparation description.
///
/// The vector length must be a power of two. The number of logical qubits is
/// inferred from the vector length.
///
/// Normalization is deliberately validated through an explicit caller-supplied
/// tolerance rather than a hidden global epsilon.
///
/// This avoids baking a numerical policy into the semantic IR.
#[derive(Debug, Clone, PartialEq)]
pub struct StateVector {
    amplitudes: Vec<ComplexAmplitude>,
}

impl StateVector {
    /// Creates a state-vector description.
    ///
    /// This validates:
    ///
    /// - non-empty vector;
    /// - power-of-two length;
    /// - finite amplitudes.
    ///
    /// It intentionally does not choose a normalization tolerance.
    pub fn new(
        amplitudes: Vec<ComplexAmplitude>,
    ) -> InitializationResult<Self> {
        if amplitudes.is_empty() {
            return Err(InitializationError::EmptyStateVector);
        }

        if !amplitudes.len().is_power_of_two() {
            return Err(InitializationError::StateVectorLengthNotPowerOfTwo {
                length: amplitudes.len(),
            });
        }

        if let Some(index) = amplitudes.iter().position(
            |amplitude| !amplitude.is_finite(),
        ) {
            return Err(InitializationError::NonFiniteAmplitude { index });
        }

        Ok(Self { amplitudes })
    }

    /// Returns all amplitudes.
    #[must_use]
    pub fn amplitudes(&self) -> &[ComplexAmplitude] {
        &self.amplitudes
    }

    /// Returns the number of amplitudes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.amplitudes.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.amplitudes.is_empty()
    }

    /// Returns the number of logical qubits represented by this vector.
    ///
    /// Construction guarantees that the length is a power of two.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.amplitudes.len().trailing_zeros() as usize
    }

    /// Calculates the squared norm.
    #[must_use]
    pub fn norm_squared(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|amplitude| amplitude.norm_squared())
            .sum()
    }

    /// Validates normalization using a caller-supplied absolute tolerance.
    ///
    /// No numerical tolerance is hard-coded into the IR.
    pub fn validate_normalized(
        &self,
        tolerance: f64,
    ) -> Result<(), InitializationError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(InitializationError::InvalidTolerance);
        }

        let norm_squared = self.norm_squared();

        if !norm_squared.is_finite() {
            return Err(InitializationError::NonFiniteNorm);
        }

        if (norm_squared - 1.0).abs() > tolerance {
            return Err(InitializationError::StateNotNormalized {
                norm_squared,
                tolerance,
            });
        }

        Ok(())
    }
}

// ============================================================================
// DENSITY OPERATOR
// ============================================================================

/// Dense density-operator preparation description.
///
/// Entries are stored row-major:
///
/// ```text
/// rho[row * dimension + column]
/// ```
///
/// The representation is intentionally semantic. It does not imply a
/// particular simulator or hardware representation.
#[derive(Debug, Clone, PartialEq)]
pub struct DensityOperator {
    elements: Vec<ComplexAmplitude>,
}

impl DensityOperator {
    /// Creates a density-operator description.
    ///
    /// The element count must be a perfect square whose dimension is a power
    /// of two.
    pub fn new(
        elements: Vec<ComplexAmplitude>,
    ) -> InitializationResult<Self> {
        if elements.is_empty() {
            return Err(InitializationError::EmptyDensityOperator);
        }

        let dimension = perfect_square_root(elements.len())
            .ok_or(InitializationError::DensityMatrixSizeInvalid {
                elements: elements.len(),
            })?;

        if !dimension.is_power_of_two() {
            return Err(InitializationError::DensityMatrixDimensionNotPowerOfTwo {
                dimension,
            });
        }

        if let Some(index) = elements.iter().position(
            |element| !element.is_finite(),
        ) {
            return Err(InitializationError::NonFiniteDensityElement {
                index,
            });
        }

        Ok(Self { elements })
    }

    /// Returns the row-major density-operator elements.
    #[must_use]
    pub fn elements(&self) -> &[ComplexAmplitude] {
        &self.elements
    }

    /// Returns the matrix dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        perfect_square_root(self.elements.len())
            .expect("DensityOperator invariant violated")
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.dimension().trailing_zeros() as usize
    }

    /// Returns whether the diagonal entries sum to approximately one.
    ///
    /// The tolerance is explicitly supplied by the caller.
    pub fn validate_trace(
        &self,
        tolerance: f64,
    ) -> Result<(), InitializationError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(InitializationError::InvalidTolerance);
        }

        let dimension = self.dimension();
        let mut trace = 0.0;

        for index in 0..dimension {
            let element = self.elements[index * dimension + index];

            if element.imaginary_part() != 0.0 {
                return Err(InitializationError::DensityTraceNotReal {
                    index,
                });
            }

            trace += element.real_part();
        }

        if !trace.is_finite() {
            return Err(InitializationError::NonFiniteTrace);
        }

        if (trace - 1.0).abs() > tolerance {
            return Err(InitializationError::DensityTraceInvalid {
                trace,
                tolerance,
            });
        }

        Ok(())
    }
}

// ============================================================================
// PRODUCT STATE
// ============================================================================

/// Tensor-product preparation description.
///
/// Each target receives one independent single-qubit preparation state.
///
/// This is compact and avoids materializing the exponentially larger dense
/// state vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductState {
    states: Vec<SingleQubitState>,
}

impl ProductState {
    /// Creates a product-state description.
    #[must_use]
    pub fn new(states: Vec<SingleQubitState>) -> Self {
        Self { states }
    }

    /// Creates an all-zero product state.
    pub fn zeros(count: usize) -> Self {
        Self {
            states: vec![SingleQubitState::Zero; count],
        }
    }

    /// Returns the individual single-qubit states.
    #[must_use]
    pub fn states(&self) -> &[SingleQubitState] {
        &self.states
    }

    /// Number of logical qubits represented.
    #[must_use]
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Returns whether the product state is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

// ============================================================================
// STABILIZER STATE
// ============================================================================

/// Symbolic stabilizer-state preparation description.
///
/// A concrete stabilizer tableau implementation belongs in a dedicated
/// stabilizer/QEC subsystem. The canonical initialization layer therefore
/// stores an explicit semantic reference rather than duplicating that system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StabilizerState {
    /// Stable semantic identifier for the stabilizer description.
    identifier: String,
}

impl StabilizerState {
    /// Creates a stabilizer-state reference.
    pub fn new(identifier: impl Into<String>) -> InitializationResult<Self> {
        let identifier = identifier.into();

        if identifier.is_empty() {
            return Err(InitializationError::EmptyStateIdentifier);
        }

        Ok(Self { identifier })
    }

    /// Returns the semantic stabilizer identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

// ============================================================================
// GRAPH STATE
// ============================================================================

/// Graph-state preparation description.
///
/// The graph is represented using logical target positions rather than
/// physical hardware connectivity.
///
/// A backend may implement the graph state through a native primitive or
/// decompose it into entangling operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphState {
    edges: Vec<(usize, usize)>,
}

impl GraphState {
    /// Creates a graph-state description.
    ///
    /// Vertex indices refer to positions in the initialization target list.
    pub fn new(
        edges: Vec<(usize, usize)>,
    ) -> InitializationResult<Self> {
        let mut seen = BTreeSet::new();

        for &(left, right) in &edges {
            if left == right {
                return Err(InitializationError::GraphSelfEdge {
                    vertex: left,
                });
            }

            let edge = if left < right {
                (left, right)
            } else {
                (right, left)
            };

            if !seen.insert(edge) {
                return Err(InitializationError::DuplicateGraphEdge {
                    left: edge.0,
                    right: edge.1,
                });
            }
        }

        Ok(Self {
            edges: seen.into_iter().collect(),
        })
    }

    /// Returns canonical graph edges.
    #[must_use]
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Validates all graph vertices against the target count.
    pub fn validate_target_count(
        &self,
        target_count: usize,
    ) -> Result<(), InitializationError> {
        for &(left, right) in &self.edges {
            if left >= target_count {
                return Err(InitializationError::GraphVertexOutOfRange {
                    vertex: left,
                    target_count,
                });
            }

            if right >= target_count {
                return Err(InitializationError::GraphVertexOutOfRange {
                    vertex: right,
                    target_count,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// ENCODED / LOGICAL STATE
// ============================================================================

/// Semantic reference to an encoded or logical quantum state.
///
/// The concrete code, decoder, lattice, block representation, or QEC
/// implementation belongs to the QEC subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncodedState {
    /// Stable logical state identifier.
    identifier: String,
}

impl EncodedState {
    /// Creates an encoded-state reference.
    pub fn new(identifier: impl Into<String>) -> InitializationResult<Self> {
        let identifier = identifier.into();

        if identifier.is_empty() {
            return Err(InitializationError::EmptyStateIdentifier);
        }

        Ok(Self { identifier })
    }

    /// Returns the semantic identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

// ============================================================================
// CUSTOM STATE
// ============================================================================

/// Extensible state-preparation reference.
///
/// This is the escape hatch for future quantum architectures.
///
/// Unknown/custom preparation semantics must remain explicit rather than being
/// silently discarded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomPreparation {
    /// Extension/dialect-qualified semantic name.
    identifier: String,

    /// Deterministic opaque payload owned by the corresponding extension.
    payload: Vec<u8>,
}

impl CustomPreparation {
    /// Creates an explicit custom preparation description.
    pub fn new(
        identifier: impl Into<String>,
        payload: Vec<u8>,
    ) -> InitializationResult<Self> {
        let identifier = identifier.into();

        if identifier.is_empty() {
            return Err(InitializationError::EmptyStateIdentifier);
        }

        Ok(Self {
            identifier,
            payload,
        })
    }

    /// Returns the extension-qualified identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the opaque extension payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

// ============================================================================
// PREPARATION SPECIFICATION
// ============================================================================

/// Complete semantic state-preparation specification.
///
/// This is intentionally extensible and is not equivalent to a finite gate
/// list.
#[derive(Debug, Clone, PartialEq)]
pub enum PreparationSpec {
    /// Prepare every target in |0>.
    Zero,

    /// Prepare every target in |1>.
    One,

    /// Prepare each target in an explicitly specified computational basis
    /// state.
    ComputationalBasis(ComputationalBasisState),

    /// Prepare each target independently in a single-qubit state.
    Product(ProductState),

    /// Prepare a dense pure state.
    StateVector(StateVector),

    /// Prepare a density operator.
    DensityOperator(DensityOperator),

    /// Prepare a stabilizer state through a semantic reference.
    Stabilizer(StabilizerState),

    /// Prepare a graph state.
    Graph(GraphState),

    /// Prepare an encoded/logical state.
    Encoded(EncodedState),

    /// Prepare an extension-defined/custom state.
    Custom(CustomPreparation),
}

impl PreparationSpec {
    /// Returns a stable semantic kind identifier.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::One => "one",
            Self::ComputationalBasis(_) => "computational_basis",
            Self::Product(_) => "product",
            Self::StateVector(_) => "state_vector",
            Self::DensityOperator(_) => "density_operator",
            Self::Stabilizer(_) => "stabilizer",
            Self::Graph(_) => "graph",
            Self::Encoded(_) => "encoded",
            Self::Custom(_) => "custom",
        }
    }
}

impl Default for PreparationSpec {
    fn default() -> Self {
        Self::Zero
    }
}

// ============================================================================
// INITIALIZATION OPTIONS
// ============================================================================

/// Semantic options controlling initialization behavior.
///
/// These options do not select hardware implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitializationOptions {
    /// Whether equivalent implementation strategies are permitted.
    allow_equivalent_implementation: bool,

    /// Whether a target compiler may synthesize the requested preparation
    /// rather than requiring a native preparation primitive.
    allow_synthesis: bool,
}

impl Default for InitializationOptions {
    fn default() -> Self {
        Self {
            allow_equivalent_implementation: true,
            allow_synthesis: true,
        }
    }
}

impl InitializationOptions {
    /// Creates default options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_equivalent_implementation: true,
            allow_synthesis: true,
        }
    }

    /// Sets whether equivalent implementations are permitted.
    #[must_use]
    pub const fn with_equivalent_implementation(
        self,
        allowed: bool,
    ) -> Self {
        Self {
            allow_equivalent_implementation: allowed,
            allow_synthesis: self.allow_synthesis,
        }
    }

    /// Sets whether synthesis is permitted.
    #[must_use]
    pub const fn with_synthesis(
        self,
        allowed: bool,
    ) -> Self {
        Self {
            allow_equivalent_implementation: self.allow_equivalent_implementation,
            allow_synthesis: allowed,
        }
    }

    /// Returns whether equivalent implementations are permitted.
    #[must_use]
    pub const fn allows_equivalent_implementation(self) -> bool {
        self.allow_equivalent_implementation
    }

    /// Returns whether synthesis is permitted.
    #[must_use]
    pub const fn allows_synthesis(self) -> bool {
        self.allow_synthesis
    }
}

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Canonical state-initialization operation.
///
/// An `Initialization` contains logical targets and semantic preparation
/// intent. It contains no physical placement or execution information.
#[derive(Debug, Clone, PartialEq)]
pub struct Initialization {
    targets: Vec<QubitId>,
    preparation: PreparationSpec,
    options: InitializationOptions,
}

impl Initialization {
    /// Creates an initialization operation.
    ///
    /// Target order is preserved.
    ///
    /// Duplicate targets are rejected.
    pub fn new(
        targets: Vec<QubitId>,
        preparation: PreparationSpec,
    ) -> InitializationResult<Self> {
        Self::with_options(
            targets,
            preparation,
            InitializationOptions::default(),
        )
    }

    /// Creates an initialization operation with explicit semantic options.
    pub fn with_options(
        targets: Vec<QubitId>,
        preparation: PreparationSpec,
        options: InitializationOptions,
    ) -> InitializationResult<Self> {
        validate_targets(&targets)?;

        let initialization = Self {
            targets,
            preparation,
            options,
        };

        initialization.validate()?;

        Ok(initialization)
    }

    /// Creates all-zero initialization.
    pub fn zero(
        targets: Vec<QubitId>,
    ) -> InitializationResult<Self> {
        Self::new(targets, PreparationSpec::Zero)
    }

    /// Creates all-one initialization.
    pub fn one(
        targets: Vec<QubitId>,
    ) -> InitializationResult<Self> {
        Self::new(targets, PreparationSpec::One)
    }

    /// Creates computational-basis initialization.
    pub fn computational_basis(
        targets: Vec<QubitId>,
        bits: Vec<BasisBit>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::ComputationalBasis(
                ComputationalBasisState::new(bits),
            ),
        )
    }

    /// Creates product-state initialization.
    pub fn product(
        targets: Vec<QubitId>,
        states: Vec<SingleQubitState>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::Product(
                ProductState::new(states),
            ),
        )
    }

    /// Creates dense state-vector initialization.
    pub fn state_vector(
        targets: Vec<QubitId>,
        amplitudes: Vec<ComplexAmplitude>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::StateVector(
                StateVector::new(amplitudes)?,
            ),
        )
    }

    /// Creates density-operator initialization.
    pub fn density_operator(
        targets: Vec<QubitId>,
        elements: Vec<ComplexAmplitude>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::DensityOperator(
                DensityOperator::new(elements)?,
            ),
        )
    }

    /// Creates stabilizer-state initialization.
    pub fn stabilizer(
        targets: Vec<QubitId>,
        identifier: impl Into<String>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::Stabilizer(
                StabilizerState::new(identifier)?,
            ),
        )
    }

    /// Creates graph-state initialization.
    pub fn graph(
        targets: Vec<QubitId>,
        edges: Vec<(usize, usize)>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::Graph(
                GraphState::new(edges)?,
            ),
        )
    }

    /// Creates encoded/logical-state initialization.
    pub fn encoded(
        targets: Vec<QubitId>,
        identifier: impl Into<String>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::Encoded(
                EncodedState::new(identifier)?,
            ),
        )
    }

    /// Creates extension-defined initialization.
    pub fn custom(
        targets: Vec<QubitId>,
        identifier: impl Into<String>,
        payload: Vec<u8>,
    ) -> InitializationResult<Self> {
        Self::new(
            targets,
            PreparationSpec::Custom(
                CustomPreparation::new(identifier, payload)?,
            ),
        )
    }

    /// Returns the source-preserved logical target list.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns the number of initialization targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether no targets are present.
    ///
    /// Normally always false because construction rejects empty targets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns the preparation specification.
    #[must_use]
    pub fn preparation(&self) -> &PreparationSpec {
        &self.preparation
    }

    /// Returns initialization options.
    #[must_use]
    pub const fn options(&self) -> InitializationOptions {
        self.options
    }

    /// Returns targets in canonical deterministic `QubitId` order.
    #[must_use]
    pub fn canonical_targets(&self) -> Vec<QubitId> {
        let mut targets = self.targets.clone();
        targets.sort_unstable();
        targets
    }

    /// Validates the complete locally knowable initialization contract.
    pub fn validate(&self) -> Result<(), InitializationError> {
        validate_targets(&self.targets)?;

        match &self.preparation {
            PreparationSpec::Zero
            | PreparationSpec::One => Ok(()),

            PreparationSpec::ComputationalBasis(state) => {
                if state.len() != self.target_count() {
                    return Err(
                        InitializationError::TargetPreparationArityMismatch {
                            targets: self.target_count(),
                            preparation: state.len(),
                        },
                    );
                }

                Ok(())
            }

            PreparationSpec::Product(state) => {
                if state.len() != self.target_count() {
                    return Err(
                        InitializationError::TargetPreparationArityMismatch {
                            targets: self.target_count(),
                            preparation: state.len(),
                        },
                    );
                }

                Ok(())
            }

            PreparationSpec::StateVector(state) => {
                if state.qubit_count() != self.target_count() {
                    return Err(
                        InitializationError::StateVectorQubitCountMismatch {
                            targets: self.target_count(),
                            state_qubits: state.qubit_count(),
                        },
                    );
                }

                Ok(())
            }

            PreparationSpec::DensityOperator(state) => {
                if state.qubit_count() != self.target_count() {
                    return Err(
                        InitializationError::DensityOperatorQubitCountMismatch {
                            targets: self.target_count(),
                            state_qubits: state.qubit_count(),
                        },
                    );
                }

                Ok(())
            }

            PreparationSpec::Stabilizer(_) => Ok(()),

            PreparationSpec::Graph(graph) => {
                graph.validate_target_count(self.target_count())
            }

            PreparationSpec::Encoded(_) => Ok(()),

            PreparationSpec::Custom(_) => Ok(()),
        }
    }

    /// Validates logical targets against an explicitly declared namespace.
    ///
    /// `logical_qubits` is a policy/context value, not an architectural limit.
    pub fn validate_namespace(
        &self,
        logical_qubits: usize,
    ) -> Result<(), InitializationError> {
        for &qubit in &self.targets {
            if qubit.index() >= logical_qubits {
                return Err(InitializationError::TargetOutOfRange {
                    qubit,
                    logical_qubits,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by local initialization construction and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum InitializationError {
    /// Initialization has no targets.
    EmptyTargets,

    /// A logical target occurs more than once.
    DuplicateTarget {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A target is outside an explicitly supplied logical namespace.
    TargetOutOfRange {
        /// Invalid target.
        qubit: QubitId,

        /// Size of the supplied logical namespace.
        logical_qubits: usize,
    },

    /// The preparation arity does not match the number of targets.
    TargetPreparationArityMismatch {
        /// Number of logical targets.
        targets: usize,

        /// Number represented by the preparation.
        preparation: usize,
    },

    /// State-vector input is empty.
    EmptyStateVector,

    /// State-vector length is not a power of two.
    StateVectorLengthNotPowerOfTwo {
        /// Invalid vector length.
        length: usize,
    },

    /// A state-vector amplitude is non-finite.
    NonFiniteAmplitude {
        /// Index of the invalid amplitude.
        index: usize,
    },

    /// State-vector normalization is invalid under the supplied tolerance.
    StateNotNormalized {
        /// Calculated squared norm.
        norm_squared: f64,

        /// Caller-provided tolerance.
        tolerance: f64,
    },

    /// Calculated state-vector norm is non-finite.
    NonFiniteNorm,

    /// The caller supplied an invalid normalization tolerance.
    InvalidTolerance,

    /// Density operator is empty.
    EmptyDensityOperator,

    /// Density-operator element count is not a square.
    DensityMatrixSizeInvalid {
        /// Number of supplied elements.
        elements: usize,
    },

    /// Density-matrix dimension is not a power of two.
    DensityMatrixDimensionNotPowerOfTwo {
        /// Invalid dimension.
        dimension: usize,
    },

    /// A density-operator element is non-finite.
    NonFiniteDensityElement {
        /// Invalid element index.
        index: usize,
    },

    /// Density trace contains an imaginary component.
    DensityTraceNotReal {
        /// Diagonal element index.
        index: usize,
    },

    /// Density trace is non-finite.
    NonFiniteTrace,

    /// Density trace is not one within the supplied tolerance.
    DensityTraceInvalid {
        /// Calculated trace.
        trace: f64,

        /// Caller-provided tolerance.
        tolerance: f64,
    },

    /// A symbolic state identifier is empty.
    EmptyStateIdentifier,

    /// Graph contains a self-edge.
    GraphSelfEdge {
        /// Invalid vertex.
        vertex: usize,
    },

    /// Graph contains the same undirected edge twice.
    DuplicateGraphEdge {
        /// First vertex.
        left: usize,

        /// Second vertex.
        right: usize,
    },

    /// Graph references a target position that does not exist.
    GraphVertexOutOfRange {
        /// Invalid vertex.
        vertex: usize,

        /// Number of initialization targets.
        target_count: usize,
    },
}

impl fmt::Display for InitializationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyTargets => {
                formatter.write_str(
                    "initialization requires at least one logical qubit target",
                )
            }

            Self::DuplicateTarget { qubit } => {
                write!(
                    formatter,
                    "initialization contains duplicate logical qubit {qubit}"
                )
            }

            Self::TargetOutOfRange {
                qubit,
                logical_qubits,
            } => {
                write!(
                    formatter,
                    "logical initialization target {qubit} is outside logical namespace 0..{logical_qubits}"
                )
            }

            Self::TargetPreparationArityMismatch {
                targets,
                preparation,
            } => {
                write!(
                    formatter,
                    "initialization target count {targets} does not match preparation arity {preparation}"
                )
            }

            Self::EmptyStateVector => {
                formatter.write_str("state vector cannot be empty")
            }

            Self::StateVectorLengthNotPowerOfTwo { length } => {
                write!(
                    formatter,
                    "state-vector length {length} must be a power of two"
                )
            }

            Self::NonFiniteAmplitude { index } => {
                write!(
                    formatter,
                    "state-vector amplitude at index {index} is not finite"
                )
            }

            Self::StateNotNormalized {
                norm_squared,
                tolerance,
            } => {
                write!(
                    formatter,
                    "state vector is not normalized: norm squared {norm_squared}, tolerance {tolerance}"
                )
            }

            Self::NonFiniteNorm => {
                formatter.write_str("state-vector norm is not finite")
            }

            Self::InvalidTolerance => {
                formatter.write_str(
                    "normalization tolerance must be finite and non-negative",
                )
            }

            Self::EmptyDensityOperator => {
                formatter.write_str("density operator cannot be empty")
            }

            Self::DensityMatrixSizeInvalid { elements } => {
                write!(
                    formatter,
                    "density operator with {elements} elements is not a square matrix"
                )
            }

            Self::DensityMatrixDimensionNotPowerOfTwo { dimension } => {
                write!(
                    formatter,
                    "density-operator dimension {dimension} must be a power of two"
                )
            }

            Self::NonFiniteDensityElement { index } => {
                write!(
                    formatter,
                    "density-operator element at index {index} is not finite"
                )
            }

            Self::DensityTraceNotReal { index } => {
                write!(
                    formatter,
                    "density-operator diagonal element at index {index} has an imaginary component"
                )
            }

            Self::NonFiniteTrace => {
                formatter.write_str("density-operator trace is not finite")
            }

            Self::DensityTraceInvalid {
                trace,
                tolerance,
            } => {
                write!(
                    formatter,
                    "density-operator trace {trace} is not one within tolerance {tolerance}"
                )
            }

            Self::EmptyStateIdentifier => {
                formatter.write_str("state identifier cannot be empty")
            }

            Self::GraphSelfEdge { vertex } => {
                write!(
                    formatter,
                    "graph state cannot contain self-edge at vertex {vertex}"
                )
            }

            Self::DuplicateGraphEdge { left, right } => {
                write!(
                    formatter,
                    "graph state contains duplicate edge ({left}, {right})"
                )
            }

            Self::GraphVertexOutOfRange {
                vertex,
                target_count,
            } => {
                write!(
                    formatter,
                    "graph state vertex {vertex} is outside target range 0..{target_count}"
                )
            }
        }
    }
}

impl std::error::Error for InitializationError {}

// ============================================================================
// INTERNAL HELPERS
// ============================================================================

/// Validates target existence/uniqueness without imposing an architectural
/// machine-size limit.
fn validate_targets(
    targets: &[QubitId],
) -> Result<(), InitializationError> {
    if targets.is_empty() {
        return Err(InitializationError::EmptyTargets);
    }

    let mut seen = BTreeSet::new();

    for &qubit in targets {
        if !seen.insert(qubit) {
            return Err(InitializationError::DuplicateTarget { qubit });
        }
    }

    Ok(())
}

/// Returns the integer square root if `value` is a perfect square.
///
/// This implementation avoids floating-point conversion and therefore avoids
/// architecture-dependent rounding behavior.
fn perfect_square_root(value: usize) -> Option<usize> {
    if value == 0 {
        return Some(0);
    }

    let mut low = 1usize;
    let mut high = value;

    while low <= high {
        let middle = low + (high - low) / 2;

        match middle.checked_mul(middle) {
            Some(square) if square == value => return Some(middle),
            Some(square) if square < value => {
                low = middle.saturating_add(1);
            }
            _ => {
                if middle == 0 {
                    break;
                }

                high = middle - 1;
            }
        }
    }

    None
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn zero_initialization_works() {
        let initialization =
            Initialization::zero(vec![q(0), q(1), q(2)])
                .expect("valid initialization");

        assert_eq!(initialization.target_count(), 3);
        assert_eq!(
            initialization.preparation().kind_name(),
            "zero"
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let result = Initialization::zero(vec![q(0), q(0)]);

        assert!(matches!(
            result,
            Err(InitializationError::DuplicateTarget {
                qubit
            }) if qubit == q(0)
        ));
    }

    #[test]
    fn empty_targets_are_rejected() {
        let result = Initialization::zero(Vec::new());

        assert!(matches!(
            result,
            Err(InitializationError::EmptyTargets)
        ));
    }

    #[test]
    fn basis_state_must_match_target_count() {
        let result = Initialization::computational_basis(
            vec![q(0), q(1)],
            vec![BasisBit::One],
        );

        assert!(matches!(
            result,
            Err(
                InitializationError::TargetPreparationArityMismatch {
                    targets: 2,
                    preparation: 1
                }
            )
        ));
    }

    #[test]
    fn product_state_must_match_target_count() {
        let result = Initialization::product(
            vec![q(0), q(1)],
            vec![SingleQubitState::Zero],
        );

        assert!(matches!(
            result,
            Err(
                InitializationError::TargetPreparationArityMismatch {
                    targets: 2,
                    preparation: 1
                }
            )
        ));
    }

    #[test]
    fn state_vector_requires_power_of_two_length() {
        let result = StateVector::new(vec![
            ComplexAmplitude::real(1.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
        ]);

        assert!(matches!(
            result,
            Err(
                InitializationError::StateVectorLengthNotPowerOfTwo {
                    length: 3
                }
            )
        ));
    }

    #[test]
    fn state_vector_qubit_count_is_inferred() {
        let state = StateVector::new(vec![
            ComplexAmplitude::real(1.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
        ])
        .expect("valid state vector");

        assert_eq!(state.qubit_count(), 2);
    }

    #[test]
    fn state_vector_normalization_is_explicit() {
        let state = StateVector::new(vec![
            ComplexAmplitude::real(1.0),
            ComplexAmplitude::real(0.0),
        ])
        .expect("valid state vector");

        assert!(
            state.validate_normalized(0.0).is_ok()
        );
    }

    #[test]
    fn state_vector_normalization_can_fail() {
        let state = StateVector::new(vec![
            ComplexAmplitude::real(2.0),
            ComplexAmplitude::real(0.0),
        ])
        .expect("structurally valid state vector");

        assert!(matches!(
            state.validate_normalized(0.0),
            Err(InitializationError::StateNotNormalized { .. })
        ));
    }

    #[test]
    fn state_vector_target_count_is_checked() {
        let result = Initialization::state_vector(
            vec![q(0)],
            vec![
                ComplexAmplitude::real(1.0),
                ComplexAmplitude::real(0.0),
                ComplexAmplitude::real(0.0),
                ComplexAmplitude::real(0.0),
            ],
        );

        assert!(matches!(
            result,
            Err(
                InitializationError::StateVectorQubitCountMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn density_operator_dimension_is_inferred() {
        let state = DensityOperator::new(vec![
            ComplexAmplitude::real(1.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
        ])
        .expect("valid density operator");

        assert_eq!(state.dimension(), 2);
        assert_eq!(state.qubit_count(), 1);
    }

    #[test]
    fn density_trace_can_be_validated() {
        let state = DensityOperator::new(vec![
            ComplexAmplitude::real(1.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
            ComplexAmplitude::real(0.0),
        ])
        .expect("valid density operator");

        assert!(state.validate_trace(0.0).is_ok());
    }

    #[test]
    fn graph_state_rejects_self_edges() {
        let result = GraphState::new(vec![(0, 0)]);

        assert!(matches!(
            result,
            Err(InitializationError::GraphSelfEdge {
                vertex: 0
            })
        ));
    }

    #[test]
    fn graph_state_canonicalizes_edges() {
        let graph =
            GraphState::new(vec![(2, 0), (3, 1)])
                .expect("valid graph");

        assert_eq!(
            graph.edges(),
            &[(0, 2), (1, 3)]
        );
    }

    #[test]
    fn graph_state_validates_target_count() {
        let graph =
            GraphState::new(vec![(0, 2)])
                .expect("valid graph");

        assert!(matches!(
            graph.validate_target_count(2),
            Err(
                InitializationError::GraphVertexOutOfRange {
                    vertex: 2,
                    target_count: 2
                }
            )
        ));
    }

    #[test]
    fn canonical_targets_are_deterministic() {
        let initialization =
            Initialization::zero(vec![q(7), q(2), q(4)])
                .expect("valid initialization");

        assert_eq!(
            initialization.canonical_targets(),
            vec![q(2), q(4), q(7)]
        );

        // Source order remains unchanged.
        assert_eq!(
            initialization.targets(),
            &[q(7), q(2), q(4)]
        );
    }

    #[test]
    fn namespace_validation_is_explicit() {
        let initialization =
            Initialization::zero(vec![q(0), q(3)])
                .expect("valid initialization");

        assert!(
            initialization.validate_namespace(4).is_ok()
        );

        assert!(matches!(
            initialization.validate_namespace(3),
            Err(
                InitializationError::TargetOutOfRange {
                    qubit,
                    logical_qubits: 3
                }
            ) if qubit == q(3)
        ));
    }

    #[test]
    fn encoded_state_is_supported_without_qec_dependency() {
        let initialization =
            Initialization::encoded(
                vec![q(0), q(1)],
                "surface_code.logical_zero",
            )
            .expect("valid encoded state");

        assert_eq!(
            initialization.preparation().kind_name(),
            "encoded"
        );
    }

    #[test]
    fn custom_state_preserves_extension_payload() {
        let initialization =
            Initialization::custom(
                vec![q(0)],
                "future.architecture.state",
                vec![1, 2, 3, 4],
            )
            .expect("valid custom state");

        match initialization.preparation() {
            PreparationSpec::Custom(custom) => {
                assert_eq!(
                    custom.identifier(),
                    "future.architecture.state"
                );

                assert_eq!(
                    custom.payload(),
                    &[1, 2, 3, 4]
                );
            }

            _ => panic!("expected custom preparation"),
        }
    }
}