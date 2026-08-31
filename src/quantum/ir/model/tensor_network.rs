//! Zamani Quantum IR — Tensor-Network Model
//!
//! Production-grade, hardware-independent semantic representation of
//! tensor-network quantum computation.
//!
//! # Architectural role
//!
//! This module belongs to:
//!
//!     quantum::ir::model::tensor_network
//!
//! It describes the *meaning and structure* of a tensor-network computation.
//! It is deliberately independent of:
//!
//! - simulators;
//! - dense state-vector storage;
//! - tensor contraction engines;
//! - BLAS/LAPACK;
//! - GPUs;
//! - CPUs;
//! - QPUs;
//! - vendor SDKs;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - network transport;
//! - numerical optimization algorithms.
//!
//! The existing simulator-side implementation under
//! `quantum::memory::tensor_network` is therefore intentionally not reused
//! here. That implementation owns numerical MPS state storage; this module
//! owns the canonical IR-level semantic model.
//!
//! # Universal model
//!
//! A tensor network is represented as:
//!
//! ```text
//! TensorNetwork
//!     ├── tensors
//!     │     ├── TensorNode
//!     │     └── TensorIndex attachments
//!     │
//!     ├── indices
//!     │     ├── physical/open indices
//!     │     └── virtual/contracted indices
//!     │
//!     ├── qubit bindings
//!     ├── outputs
//!     ├── inputs
//!     ├── contraction intent
//!     ├── approximation policy
//!     ├── topology metadata
//!     └── extensible attributes
//! ```
//!
//! This representation is intentionally more general than MPS.
//!
//! It can describe:
//!
//! - Matrix Product States;
//! - Matrix Product Operators;
//! - Tensor Train representations;
//! - Tree Tensor Networks;
//! - Projected Entangled Pair States;
//! - projected entangled pair operators;
//! - MERA-like networks;
//! - arbitrary tensor graphs;
//! - tensor-network representations of quantum circuits;
//! - tensor-network representations of operators;
//! - tensor-network representations of states;
//! - hybrid tensor networks;
//! - future tensor-network architectures.
//!
//! # Important scalability rule
//!
//! There is no semantic maximum number of:
//!
//! - tensors;
//! - indices;
//! - qubits;
//! - tensor rank;
//! - bond dimensions;
//! - network edges;
//! - open indices;
//! - contraction steps.
//!
//! Concrete limits belong to explicit compilation/execution policy.
//!
//! Rust collection/address-space limits and available memory necessarily
//! constrain a particular process, but those constraints are not encoded as
//! quantum-machine limits in this module.
//!
//! # Qubit identity
//!
//! Physical quantum-system attachment uses the canonical:
//!
//!     quantum::ir::qubit::QubitId
//!
//! This module never creates a second QubitId type.
//!
//! Tensor-network index identity is deliberately different from qubit
//! identity:
//!
//!     QubitId       = semantic quantum-system identity
//!     TensorIndexId = tensor-network graph identity
//!
//! One qubit may participate in multiple tensor-network representations or
//! multiple physical/operator legs, so conflating these namespaces would be
//! incorrect.
//!
//! # Numerical separation
//!
//! Tensor values are NOT stored as `Vec<Complex>` here.
//!
//! This is intentional.
//!
//! The canonical IR should not become a hidden simulator or force every
//! tensor-network workload to materialize potentially enormous numerical
//! tensors.
//!
//! Numerical data may instead be represented by:
//!
//! - symbolic expressions;
//! - parameter references;
//! - external tensor handles;
//! - generated tensor definitions;
//! - constants;
//! - opaque provider-neutral payload references;
//! - downstream numerical storage.
//!
//! # Rust
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! This module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! Required canonical dependency:
//!
//!     quantum::ir::qubit::QubitId
//!
//! No dependency is taken on:
//!
//!     quantum::memory
//!     quantum::hardware
//!     quantum::simulator
//!     quantum::backend
//!
//! The parent `model` module should expose this module without changing any
//! of its public types.
//!
//! A later contraction engine may consume this representation.
//! A simulator may lower it into `quantum::memory`.
//! An optimizer may transform it while preserving semantic equivalence.
//! A serializer may persist it.
//! A validator may validate it.
//!
//! None of those downstream systems are dependencies of this file.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::qubit::QubitId;

// ============================================================================
// Schema identity
// ============================================================================

/// Stable schema identifier for the tensor-network IR model.
pub const TENSOR_NETWORK_MODEL_SCHEMA_ID: &str =
    "zamani.quantum.ir.model.tensor_network";

/// Semantic version of this model contract.
///
/// This is intentionally independent from the global IR version. The global
/// IR version remains authoritative for the complete IR. This value identifies
/// the internal tensor-network schema.
pub const TENSOR_NETWORK_MODEL_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Result
// ============================================================================

/// Result type used by this module.
pub type TensorNetworkResult<T> = Result<T, TensorNetworkError>;

// ============================================================================
// Stable identifiers
// ============================================================================

/// Stable identity of a tensor-network object.
///
/// This namespace is intentionally independent from `QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TensorNetworkId(u64);

impl TensorNetworkId {
    /// Creates an identifier from its stable numeric value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for TensorNetworkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tn{}", self.0)
    }
}

/// Stable tensor-node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TensorId(u64);

impl TensorId {
    /// Creates an identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for TensorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tensor{}", self.0)
    }
}

/// Stable tensor-network index identifier.
///
/// An index is a graph edge. It can be:
///
/// - open;
/// - contracted;
/// - external;
/// - physical;
/// - virtual;
/// - symbolic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TensorIndexId(u64);

impl TensorIndexId {
    /// Creates an identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for TensorIndexId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{}", self.0)
    }
}

// ============================================================================
// Tensor-network semantic kind
// ============================================================================

/// High-level semantic role of a tensor network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TensorNetworkKind {
    /// Quantum state representation.
    State,

    /// Quantum operator representation.
    Operator,

    /// Quantum channel / superoperator representation.
    Channel,

    /// Tensor-network representation of a quantum circuit.
    Circuit,

    /// Tensor-network representation of a measurement process.
    Measurement,

    /// Hybrid state/operator network.
    Hybrid,

    /// General mathematical tensor network.
    General,
}

impl TensorNetworkKind {
    /// Returns a stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::State => "state",
            Self::Operator => "operator",
            Self::Channel => "channel",
            Self::Circuit => "circuit",
            Self::Measurement => "measurement",
            Self::Hybrid => "hybrid",
            Self::General => "general",
        }
    }
}

// ============================================================================
// Topology
// ============================================================================

/// Recognized tensor-network topology.
///
/// `General` is intentionally available so the enum is not a closed universe
/// of tensor-network research.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TensorNetworkTopology {
    /// One-dimensional chain.
    MatrixProduct,

    /// Tree-structured tensor network.
    Tree,

    /// Lattice / projected entangled-pair topology.
    ProjectedEntangledPair,

    /// Multiscale entanglement-renormalization style topology.
    MultiscaleEntanglementRenormalization,

    /// Arbitrary graph.
    General,
}

impl TensorNetworkTopology {
    /// Returns a stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatrixProduct => "matrix_product",
            Self::Tree => "tree",
            Self::ProjectedEntangledPair => "projected_entangled_pair",
            Self::MultiscaleEntanglementRenormalization => {
                "multiscale_entanglement_renormalization"
            }
            Self::General => "general",
        }
    }
}

// ============================================================================
// Index semantics
// ============================================================================

/// Semantic role of a tensor-network index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TensorIndexRole {
    /// Physical index associated with a quantum system.
    Physical,

    /// Virtual/internal contraction index.
    Virtual,

    /// External input index.
    Input,

    /// External output index.
    Output,

    /// Classical/environmental index.
    Classical,

    /// Measurement index.
    Measurement,

    /// Unspecified/general index.
    General,
}

impl TensorIndexRole {
    /// Returns whether the index is part of the external tensor-network
    /// interface.
    #[must_use]
    pub const fn is_open_role(self) -> bool {
        matches!(
            self,
            Self::Physical
                | Self::Input
                | Self::Output
                | Self::Classical
                | Self::Measurement
        )
    }

    /// Returns whether the index is normally contracted internally.
    #[must_use]
    pub const fn is_virtual_role(self) -> bool {
        matches!(self, Self::Virtual)
    }
}

// ============================================================================
// Index dimension
// ============================================================================

/// Dimension of a tensor-network index.
///
/// Dimensions are represented symbolically or concretely.
///
/// A concrete `usize` is appropriate for a local compilation representation;
/// it is not a machine-size limit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IndexDimension {
    /// Known finite dimension.
    Finite(u64),

    /// Dimension represented by a named symbolic value.
    Symbolic(String),

    /// Dimension determined by an external resource/type.
    External(String),
}

impl IndexDimension {
    /// Creates a finite dimension.
    pub fn finite(value: u64) -> TensorNetworkResult<Self> {
        if value == 0 {
            return Err(TensorNetworkError::InvalidDimension { value });
        }

        Ok(Self::Finite(value))
    }

    /// Creates a symbolic dimension.
    pub fn symbolic(name: impl Into<String>) -> TensorNetworkResult<Self> {
        let name = name.into();

        validate_identifier(&name)?;

        Ok(Self::Symbolic(name))
    }

    /// Creates an external dimension reference.
    pub fn external(name: impl Into<String>) -> TensorNetworkResult<Self> {
        let name = name.into();

        validate_identifier(&name)?;

        Ok(Self::External(name))
    }

    /// Returns the concrete dimension when known.
    #[must_use]
    pub fn finite_value(&self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(*value),
            Self::Symbolic(_) | Self::External(_) => None,
        }
    }
}

// ============================================================================
// Tensor index
// ============================================================================

/// A graph index belonging to a tensor network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorIndex {
    id: TensorIndexId,
    dimension: IndexDimension,
    role: TensorIndexRole,
    name: Option<String>,
    qubit: Option<QubitId>,
    endpoints: BTreeSet<TensorId>,
    attributes: BTreeMap<String, TensorAttribute>,
}

impl TensorIndex {
    /// Creates a new index.
    pub fn new(
        id: TensorIndexId,
        dimension: IndexDimension,
        role: TensorIndexRole,
    ) -> TensorNetworkResult<Self> {
        Ok(Self {
            id,
            dimension,
            role,
            name: None,
            qubit: None,
            endpoints: BTreeSet::new(),
            attributes: BTreeMap::new(),
        })
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> TensorIndexId {
        self.id
    }

    /// Returns the dimension.
    #[must_use]
    pub fn dimension(&self) -> &IndexDimension {
        &self.dimension
    }

    /// Returns the semantic role.
    #[must_use]
    pub const fn role(&self) -> TensorIndexRole {
        self.role
    }

    /// Returns the optional name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the attached logical qubit, if any.
    #[must_use]
    pub const fn qubit(&self) -> Option<QubitId> {
        self.qubit
    }

    /// Returns the tensor endpoints.
    #[must_use]
    pub fn endpoints(&self) -> &BTreeSet<TensorId> {
        &self.endpoints
    }

    /// Returns whether the index is open.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.endpoints.len() <= 1
    }

    /// Returns whether the index is contracted.
    #[must_use]
    pub fn is_contracted(&self) -> bool {
        self.endpoints.len() == 2
    }

    /// Sets a stable human-readable name.
    pub fn set_name(&mut self, name: impl Into<String>) -> TensorNetworkResult<()> {
        let name = name.into();

        validate_identifier(&name)?;

        self.name = Some(name);

        Ok(())
    }

    /// Associates this physical index with a canonical logical qubit.
    pub fn bind_qubit(&mut self, qubit: QubitId) -> TensorNetworkResult<()> {
        if !self.role.is_open_role() {
            return Err(TensorNetworkError::InvalidIndexBinding {
                index: self.id,
                reason: "only external/physical-style indices may bind a logical qubit",
            });
        }

        self.qubit = Some(qubit);

        Ok(())
    }

    /// Adds an endpoint.
    pub fn add_endpoint(&mut self, tensor: TensorId) -> TensorNetworkResult<()> {
        if self.endpoints.contains(&tensor) {
            return Err(TensorNetworkError::DuplicateEndpoint {
                index: self.id,
                tensor,
            });
        }

        if self.endpoints.len() >= 2 {
            return Err(TensorNetworkError::TooManyEndpoints {
                index: self.id,
            });
        }

        self.endpoints.insert(tensor);

        Ok(())
    }

    /// Removes an endpoint.
    pub fn remove_endpoint(&mut self, tensor: TensorId) -> bool {
        self.endpoints.remove(&tensor)
    }

    /// Adds an extensible attribute.
    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: TensorAttribute,
    ) -> TensorNetworkResult<()> {
        let name = name.into();

        validate_attribute_name(&name)?;

        self.attributes.insert(name, value);

        Ok(())
    }

    /// Returns attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, TensorAttribute> {
        &self.attributes
    }
}

// ============================================================================
// Tensor value specification
// ============================================================================

/// Provider-neutral specification of tensor numerical content.
///
/// This deliberately does not contain a dense array.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TensorValue {
    /// A named symbolic tensor.
    Symbol(String),

    /// A tensor generated by an external provider-neutral definition.
    External {
        /// Stable external resource identifier.
        reference: String,
    },

    /// A scalar constant.
    Scalar {
        /// Exact textual representation.
        ///
        /// Keeping the canonical value textual avoids forcing the IR to
        /// choose a floating-point precision.
        value: String,
    },

    /// A tensor generated by an operation/expression.
    Expression(TensorExpression),

    /// An opaque value preserved by the IR.
    Opaque {
        /// Extension namespace.
        namespace: String,

        /// Extension payload.
        payload: Vec<u8>,
    },
}

impl TensorValue {
    /// Creates a symbolic tensor reference.
    pub fn symbol(name: impl Into<String>) -> TensorNetworkResult<Self> {
        let name = name.into();

        validate_identifier(&name)?;

        Ok(Self::Symbol(name))
    }

    /// Creates an external tensor reference.
    pub fn external(reference: impl Into<String>) -> TensorNetworkResult<Self> {
        let reference = reference.into();

        if reference.is_empty() {
            return Err(TensorNetworkError::InvalidIdentifier {
                value: reference,
            });
        }

        Ok(Self::External { reference })
    }

    /// Creates a scalar from canonical textual representation.
    pub fn scalar(value: impl Into<String>) -> TensorNetworkResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TensorNetworkError::InvalidValue {
                reason: "tensor scalar cannot be empty",
            });
        }

        Ok(Self::Scalar { value })
    }
}

// ============================================================================
// Tensor expressions
// ============================================================================

/// Symbolic tensor construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TensorExpression {
    /// Tensor product.
    TensorProduct(Vec<TensorValue>),

    /// Elementwise sum.
    Sum(Vec<TensorValue>),

    /// Scalar multiplication.
    Scale {
        /// Scalar expression.
        scalar: String,

        /// Tensor operand.
        tensor: Box<TensorValue>,
    },

    /// Conjugation.
    Conjugate(Box<TensorValue>),

    /// Adjoint.
    Adjoint(Box<TensorValue>),

    /// User-defined tensor constructor.
    Custom {
        /// Stable namespace.
        namespace: String,

        /// Operation name.
        name: String,

        /// Arguments.
        arguments: Vec<TensorValue>,
    },
}

// ============================================================================
// Tensor semantic kind
// ============================================================================

/// Semantic role of a tensor node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TensorKind {
    /// State tensor.
    State,

    /// Operator tensor.
    Operator,

    /// Channel tensor.
    Channel,

    /// Gate tensor.
    Gate,

    /// Measurement tensor.
    Measurement,

    /// Boundary tensor.
    Boundary,

    /// Auxiliary/virtual tensor.
    Auxiliary,

    /// Generic tensor.
    General,
}

// ============================================================================
// Tensor node
// ============================================================================

/// A tensor node in the network graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorNode {
    id: TensorId,
    kind: TensorKind,
    indices: Vec<TensorIndexId>,
    value: TensorValue,
    name: Option<String>,
    attributes: BTreeMap<String, TensorAttribute>,
}

impl TensorNode {
    /// Creates a tensor node.
    pub fn new(
        id: TensorId,
        kind: TensorKind,
        indices: Vec<TensorIndexId>,
        value: TensorValue,
    ) -> TensorNetworkResult<Self> {
        let mut seen = BTreeSet::new();

        for index in &indices {
            if !seen.insert(*index) {
                return Err(TensorNetworkError::DuplicateTensorIndex {
                    tensor: id,
                    index: *index,
                });
            }
        }

        Ok(Self {
            id,
            kind,
            indices,
            value,
            name: None,
            attributes: BTreeMap::new(),
        })
    }

    /// Returns tensor identifier.
    #[must_use]
    pub const fn id(&self) -> TensorId {
        self.id
    }

    /// Returns tensor kind.
    #[must_use]
    pub const fn kind(&self) -> TensorKind {
        self.kind
    }

    /// Returns tensor indices in semantic tensor-axis order.
    #[must_use]
    pub fn indices(&self) -> &[TensorIndexId] {
        &self.indices
    }

    /// Returns the tensor value specification.
    #[must_use]
    pub fn value(&self) -> &TensorValue {
        &self.value
    }

    /// Returns the optional name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets a stable name.
    pub fn set_name(&mut self, name: impl Into<String>) -> TensorNetworkResult<()> {
        let name = name.into();

        validate_identifier(&name)?;

        self.name = Some(name);

        Ok(())
    }

    /// Sets an attribute.
    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: TensorAttribute,
    ) -> TensorNetworkResult<()> {
        let name = name.into();

        validate_attribute_name(&name)?;

        self.attributes.insert(name, value);

        Ok(())
    }

    /// Returns tensor attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, TensorAttribute> {
        &self.attributes
    }
}

// ============================================================================
// Attributes
// ============================================================================

/// Extensible tensor-network metadata.
///
/// Attributes are deliberately kept small and deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TensorAttribute {
    /// Boolean value.
    Bool(bool),

    /// Signed integer.
    Integer(i64),

    /// Unsigned integer.
    Unsigned(u64),

    /// Text value.
    String(String),

    /// Nested array.
    Array(Vec<TensorAttribute>),

    /// Nested key/value map.
    Map(BTreeMap<String, TensorAttribute>),

    /// Opaque extension payload.
    Opaque {
        /// Extension namespace.
        namespace: String,

        /// Raw payload.
        payload: Vec<u8>,
    },
}

// ============================================================================
// Physical tensor binding
// ============================================================================

/// Binding between a tensor-network physical index and a logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitIndexBinding {
    /// Tensor-network index.
    pub index: TensorIndexId,

    /// Canonical Zamani logical qubit.
    pub qubit: QubitId,
}

// ============================================================================
// Contraction semantics
// ============================================================================

/// Requested contraction strategy.
///
/// This is intent, not an implementation algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractionStrategy {
    /// Let the target compiler choose.
    Automatic,

    /// Preserve the existing graph as much as possible.
    PreserveTopology,

    /// Prefer a specified ordering.
    ExplicitOrder(Vec<TensorIndexId>),

    /// Named provider-neutral strategy.
    Named(String),
}

/// Approximation policy for transformations that may discard information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproximationPolicy {
    /// Whether approximation is allowed.
    pub allow_approximation: bool,

    /// Maximum permitted discarded weight, if known.
    pub maximum_discarded_weight: Option<String>,

    /// Maximum bond dimension, if specified.
    pub maximum_bond_dimension: Option<u64>,

    /// Optional relative error bound.
    pub relative_error: Option<String>,

    /// Optional absolute error bound.
    pub absolute_error: Option<String>,
}

impl Default for ApproximationPolicy {
    fn default() -> Self {
        Self {
            allow_approximation: false,
            maximum_discarded_weight: None,
            maximum_bond_dimension: None,
            relative_error: None,
            absolute_error: None,
        }
    }
}

impl ApproximationPolicy {
    /// Validates the policy.
    pub fn validate(&self) -> TensorNetworkResult<()> {
        if self
            .maximum_bond_dimension
            .is_some_and(|dimension| dimension == 0)
        {
            return Err(TensorNetworkError::InvalidApproximationPolicy);
        }

        if !self.allow_approximation
            && (self.maximum_discarded_weight.is_some()
                || self.maximum_bond_dimension.is_some()
                || self.relative_error.is_some()
                || self.absolute_error.is_some())
        {
            return Err(TensorNetworkError::InvalidApproximationPolicy);
        }

        Ok(())
    }
}

// ============================================================================
// Contraction outputs
// ============================================================================

/// Desired semantic output of a tensor-network contraction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TensorNetworkOutput {
    /// A scalar.
    Scalar,

    /// An open tensor with selected indices.
    OpenTensor(Vec<TensorIndexId>),

    /// A state over selected qubit indices.
    QuantumState(Vec<QubitId>),

    /// An operator over selected qubits.
    QuantumOperator {
        /// Input qubits.
        inputs: Vec<QubitId>,

        /// Output qubits.
        outputs: Vec<QubitId>,
    },

    /// Keep the network uncontracted.
    Network,

    /// Named external result.
    Named(String),
}

// ============================================================================
// Tensor-network metadata
// ============================================================================

/// Metadata describing a tensor network without changing its semantics.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TensorNetworkMetadata {
    /// Optional user-facing name.
    pub name: Option<String>,

    /// Optional documentation.
    pub description: Option<String>,

    /// Optional semantic annotations.
    pub attributes: BTreeMap<String, TensorAttribute>,

    /// Arbitrary extension namespaces.
    pub extensions: BTreeMap<String, Vec<u8>>,
}

// ============================================================================
// Main TensorNetwork
// ============================================================================

/// Canonical tensor-network semantic model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorNetwork {
    id: TensorNetworkId,
    kind: TensorNetworkKind,
    topology: TensorNetworkTopology,

    tensors: BTreeMap<TensorId, TensorNode>,
    indices: BTreeMap<TensorIndexId, TensorIndex>,

    qubit_bindings: BTreeMap<QubitId, TensorIndexId>,

    outputs: Vec<TensorNetworkOutput>,

    contraction_strategy: ContractionStrategy,
    approximation_policy: ApproximationPolicy,

    metadata: TensorNetworkMetadata,
}

impl TensorNetwork {
    /// Creates an empty tensor network.
    pub fn new(
        id: TensorNetworkId,
        kind: TensorNetworkKind,
        topology: TensorNetworkTopology,
    ) -> Self {
        Self {
            id,
            kind,
            topology,
            tensors: BTreeMap::new(),
            indices: BTreeMap::new(),
            qubit_bindings: BTreeMap::new(),
            outputs: Vec::new(),
            contraction_strategy: ContractionStrategy::Automatic,
            approximation_policy: ApproximationPolicy::default(),
            metadata: TensorNetworkMetadata::default(),
        }
    }

    /// Returns the stable network identifier.
    #[must_use]
    pub const fn id(&self) -> TensorNetworkId {
        self.id
    }

    /// Returns the semantic kind.
    #[must_use]
    pub const fn kind(&self) -> TensorNetworkKind {
        self.kind
    }

    /// Returns the topology declaration.
    #[must_use]
    pub const fn topology(&self) -> TensorNetworkTopology {
        self.topology
    }

    /// Returns all tensor nodes.
    #[must_use]
    pub fn tensors(&self) -> &BTreeMap<TensorId, TensorNode> {
        &self.tensors
    }

    /// Returns all graph indices.
    #[must_use]
    pub fn indices(&self) -> &BTreeMap<TensorIndexId, TensorIndex> {
        &self.indices
    }

    /// Returns qubit-to-index bindings.
    #[must_use]
    pub fn qubit_bindings(&self) -> &BTreeMap<QubitId, TensorIndexId> {
        &self.qubit_bindings
    }

    /// Returns network outputs.
    #[must_use]
    pub fn outputs(&self) -> &[TensorNetworkOutput] {
        &self.outputs
    }

    /// Returns contraction strategy.
    #[must_use]
    pub fn contraction_strategy(&self) -> &ContractionStrategy {
        &self.contraction_strategy
    }

    /// Returns approximation policy.
    #[must_use]
    pub fn approximation_policy(&self) -> &ApproximationPolicy {
        &self.approximation_policy
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &TensorNetworkMetadata {
        &self.metadata
    }

    /// Adds a tensor-network index.
    pub fn add_index(&mut self, index: TensorIndex) -> TensorNetworkResult<()> {
        if self.indices.contains_key(&index.id()) {
            return Err(TensorNetworkError::DuplicateIndex {
                index: index.id(),
            });
        }

        if let Some(qubit) = index.qubit() {
            if let Some(existing) = self.qubit_bindings.get(&qubit) {
                if *existing != index.id() {
                    return Err(TensorNetworkError::QubitAlreadyBound { qubit: qubit });
                }
            }
        }

        self.indices.insert(index.id(), index);

        Ok(())
    }

    /// Adds a tensor node.
    pub fn add_tensor(&mut self, tensor: TensorNode) -> TensorNetworkResult<()> {
        if self.tensors.contains_key(&tensor.id()) {
            return Err(TensorNetworkError::DuplicateTensor {
                tensor: tensor.id(),
            });
        }

        for index_id in tensor.indices() {
            if !self.indices.contains_key(index_id) {
                return Err(TensorNetworkError::UnknownIndex {
                    index: *index_id,
                });
            }
        }

        self.tensors.insert(tensor.id(), tensor);

        Ok(())
    }

    /// Connects a tensor to an existing index.
    ///
    /// This is deliberately separate from `add_tensor`, allowing a caller to
    /// construct either the tensor graph or index graph first.
    pub fn connect(
        &mut self,
        tensor: TensorId,
        index: TensorIndexId,
    ) -> TensorNetworkResult<()> {
        if !self.tensors.contains_key(&tensor) {
            return Err(TensorNetworkError::UnknownTensor { tensor });
        }

        let index_value = self
            .indices
            .get_mut(&index)
            .ok_or(TensorNetworkError::UnknownIndex { index })?;

        index_value.add_endpoint(tensor)?;

        Ok(())
    }

    /// Binds a logical qubit to a physical tensor-network index.
    pub fn bind_qubit(
        &mut self,
        qubit: QubitId,
        index: TensorIndexId,
    ) -> TensorNetworkResult<()> {
        let index_value = self
            .indices
            .get_mut(&index)
            .ok_or(TensorNetworkError::UnknownIndex { index })?;

        if !index_value.role().is_open_role() {
            return Err(TensorNetworkError::InvalidIndexBinding {
                index,
                reason: "logical qubits must bind to open/physical-style indices",
            });
        }

        if let Some(existing) = self.qubit_bindings.get(&qubit) {
            if *existing != index {
                return Err(TensorNetworkError::QubitAlreadyBound { qubit });
            }
        }

        if let Some(existing_qubit) = index_value.qubit() {
            if existing_qubit != qubit {
                return Err(TensorNetworkError::IndexAlreadyBound { index });
            }
        }

        index_value.bind_qubit(qubit)?;

        self.qubit_bindings.insert(qubit, index);

        Ok(())
    }

    /// Adds an output declaration.
    pub fn add_output(&mut self, output: TensorNetworkOutput) {
        self.outputs.push(output);
    }

    /// Sets the contraction strategy.
    pub fn set_contraction_strategy(&mut self, strategy: ContractionStrategy) {
        self.contraction_strategy = strategy;
    }

    /// Sets the approximation policy.
    pub fn set_approximation_policy(
        &mut self,
        policy: ApproximationPolicy,
    ) -> TensorNetworkResult<()> {
        policy.validate()?;

        self.approximation_policy = policy;

        Ok(())
    }

    /// Returns a mutable metadata reference.
    pub fn metadata_mut(&mut self) -> &mut TensorNetworkMetadata {
        &mut self.metadata
    }

    /// Validates the complete graph.
    pub fn validate(&self) -> TensorNetworkResult<()> {
        self.validate_identifiers()?;
        self.validate_indices()?;
        self.validate_tensors()?;
        self.validate_bindings()?;
        self.validate_outputs()?;
        self.approximation_policy.validate()?;
        self.validate_topology()?;

        Ok(())
    }

    fn validate_identifiers(&self) -> TensorNetworkResult<()> {
        for tensor in self.tensors.values() {
            if let Some(name) = tensor.name() {
                validate_identifier(name)?;
            }

            for name in tensor.attributes().keys() {
                validate_attribute_name(name)?;
            }
        }

        for index in self.indices.values() {
            if let Some(name) = index.name() {
                validate_identifier(name)?;
            }

            for name in index.attributes().keys() {
                validate_attribute_name(name)?;
            }
        }

        Ok(())
    }

    fn validate_indices(&self) -> TensorNetworkResult<()> {
        for index in self.indices.values() {
            let endpoint_count = index.endpoints().len();

            if endpoint_count > 2 {
                return Err(TensorNetworkError::TooManyEndpoints {
                    index: index.id(),
                });
            }

            if index.role() == TensorIndexRole::Virtual && endpoint_count != 2 {
                return Err(TensorNetworkError::InvalidVirtualIndex {
                    index: index.id(),
                });
            }

            if index.role().is_open_role() && endpoint_count > 1 {
                return Err(TensorNetworkError::InvalidOpenIndex {
                    index: index.id(),
                });
            }

            for endpoint in index.endpoints() {
                if !self.tensors.contains_key(endpoint) {
                    return Err(TensorNetworkError::UnknownTensor {
                        tensor: *endpoint,
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_tensors(&self) -> TensorNetworkResult<()> {
        for tensor in self.tensors.values() {
            for index_id in tensor.indices() {
                let index = self
                    .indices
                    .get(index_id)
                    .ok_or(TensorNetworkError::UnknownIndex {
                        index: *index_id,
                    })?;

                if !index.endpoints().contains(&tensor.id()) {
                    return Err(TensorNetworkError::DisconnectedTensorIndex {
                        tensor: tensor.id(),
                        index: *index_id,
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_bindings(&self) -> TensorNetworkResult<()> {
        for (qubit, index_id) in &self.qubit_bindings {
            let index = self
                .indices
                .get(index_id)
                .ok_or(TensorNetworkError::UnknownIndex { index: *index_id })?;

            if index.qubit() != Some(*qubit) {
                return Err(TensorNetworkError::BindingMismatch {
                    qubit: *qubit,
                    index: *index_id,
                });
            }

            if !index.role().is_open_role() {
                return Err(TensorNetworkError::InvalidIndexBinding {
                    index: *index_id,
                    reason: "qubit binding requires an open/physical-style index",
                });
            }
        }

        Ok(())
    }

    fn validate_outputs(&self) -> TensorNetworkResult<()> {
        for output in &self.outputs {
            match output {
                TensorNetworkOutput::OpenTensor(indices) => {
                    for index in indices {
                        if !self.indices.contains_key(index) {
                            return Err(TensorNetworkError::UnknownIndex { index: *index });
                        }
                    }
                }

                TensorNetworkOutput::QuantumState(qubits) => {
                    self.validate_qubit_outputs(qubits)?;
                }

                TensorNetworkOutput::QuantumOperator { inputs, outputs } => {
                    self.validate_qubit_outputs(inputs)?;
                    self.validate_qubit_outputs(outputs)?;
                }

                TensorNetworkOutput::Scalar
                | TensorNetworkOutput::Network
                | TensorNetworkOutput::Named(_) => {}
            }
        }

        Ok(())
    }

    fn validate_qubit_outputs(&self, qubits: &[QubitId]) -> TensorNetworkResult<()> {
        let mut seen = BTreeSet::new();

        for qubit in qubits {
            if !seen.insert(*qubit) {
                return Err(TensorNetworkError::DuplicateOutputQubit { qubit: *qubit });
            }

            if !self.qubit_bindings.contains_key(qubit) {
                return Err(TensorNetworkError::UnboundOutputQubit { qubit: *qubit });
            }
        }

        Ok(())
    }

    fn validate_topology(&self) -> TensorNetworkResult<()> {
        match self.topology {
            TensorNetworkTopology::MatrixProduct => self.validate_matrix_product(),
            TensorNetworkTopology::Tree => self.validate_tree(),
            TensorNetworkTopology::ProjectedEntangledPair
            | TensorNetworkTopology::MultiscaleEntanglementRenormalization
            | TensorNetworkTopology::General => Ok(()),
        }
    }

    fn validate_matrix_product(&self) -> TensorNetworkResult<()> {
        if self.tensors.is_empty() {
            return Ok(());
        }

        let mut physical_tensors = Vec::new();

        for tensor in self.tensors.values() {
            let has_physical_index = tensor.indices().iter().any(|index_id| {
                self.indices
                    .get(index_id)
                    .map(|index| index.role() == TensorIndexRole::Physical)
                    .unwrap_or(false)
            });

            if has_physical_index {
                physical_tensors.push(tensor.id());
            }
        }

        if physical_tensors.len() <= 1 {
            return Ok(());
        }

        // A matrix-product topology must form a connected chain. We do not
        // require a particular physical-qubit numbering here; ordering belongs
        // to the semantic graph/index structure.
        if !is_connected_subset(&self.tensors, &self.indices, &physical_tensors) {
            return Err(TensorNetworkError::InvalidTopology {
                reason: "matrix-product topology must be connected",
            });
        }

        Ok(())
    }

    fn validate_tree(&self) -> TensorNetworkResult<()> {
        if self.tensors.len() <= 1 {
            return Ok(());
        }

        let edges = self
            .indices
            .values()
            .filter(|index| index.is_contracted())
            .count();

        if edges + 1 != self.tensors.len() {
            return Err(TensorNetworkError::InvalidTopology {
                reason: "tree topology must contain exactly tensor_count - 1 contracted edges",
            });
        }

        Ok(())
    }

    /// Returns the number of tensor nodes.
    #[must_use]
    pub fn tensor_count(&self) -> usize {
        self.tensors.len()
    }

    /// Returns the number of graph indices.
    #[must_use]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// Returns the number of contracted indices.
    #[must_use]
    pub fn contracted_index_count(&self) -> usize {
        self.indices
            .values()
            .filter(|index| index.is_contracted())
            .count()
    }

    /// Returns the number of open indices.
    #[must_use]
    pub fn open_index_count(&self) -> usize {
        self.indices.values().filter(|index| index.is_open()).count()
    }

    /// Returns whether the network has no tensors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tensors.is_empty()
    }
}

// ============================================================================
// Construction helpers
// ============================================================================

/// Creates a single-qubit product-state tensor network.
///
/// This creates semantic structure only. No numerical amplitudes are
/// materialized.
pub fn product_state(
    network_id: TensorNetworkId,
    qubits: impl IntoIterator<Item = QubitId>,
) -> TensorNetworkResult<TensorNetwork> {
    let qubits: Vec<QubitId> = qubits.into_iter().collect();

    let mut network = TensorNetwork::new(
        network_id,
        TensorNetworkKind::State,
        TensorNetworkTopology::MatrixProduct,
    );

    let mut next_tensor = 0u64;
    let mut next_index = 0u64;

    for qubit in qubits {
        let physical_index = TensorIndex::new(
            TensorIndexId::new(next_index),
            IndexDimension::finite(2)?,
            TensorIndexRole::Physical,
        )?;

        next_index = next_index
            .checked_add(1)
            .ok_or(TensorNetworkError::IdentifierOverflow)?;

        let physical_id = physical_index.id();

        network.add_index(physical_index)?;
        network.bind_qubit(qubit, physical_id)?;

        let tensor = TensorNode::new(
            TensorId::new(next_tensor),
            TensorKind::State,
            vec![physical_id],
            TensorValue::symbol(format!("|0>_{qubit}"))?,
        )?;

        next_tensor = next_tensor
            .checked_add(1)
            .ok_or(TensorNetworkError::IdentifierOverflow)?;

        network.add_tensor(tensor)?;
    }

    network.validate()?;

    Ok(network)
}

/// Creates an empty general tensor network.
#[must_use]
pub fn empty_network(
    network_id: TensorNetworkId,
    kind: TensorNetworkKind,
) -> TensorNetwork {
    TensorNetwork::new(network_id, kind, TensorNetworkTopology::General)
}

// ============================================================================
// Structural queries
// ============================================================================

/// Returns the degree of a tensor in the graph.
pub fn tensor_degree(
    network: &TensorNetwork,
    tensor: TensorId,
) -> TensorNetworkResult<usize> {
    if !network.tensors.contains_key(&tensor) {
        return Err(TensorNetworkError::UnknownTensor { tensor });
    }

    Ok(network
        .indices
        .values()
        .filter(|index| index.endpoints().contains(&tensor))
        .count())
}

/// Returns all tensor neighbors of a tensor in deterministic order.
pub fn tensor_neighbors(
    network: &TensorNetwork,
    tensor: TensorId,
) -> TensorNetworkResult<Vec<TensorId>> {
    if !network.tensors.contains_key(&tensor) {
        return Err(TensorNetworkError::UnknownTensor { tensor });
    }

    let mut neighbors = BTreeSet::new();

    for index in network.indices.values() {
        if index.endpoints().contains(&tensor) {
            for endpoint in index.endpoints() {
                if *endpoint != tensor {
                    neighbors.insert(*endpoint);
                }
            }
        }
    }

    Ok(neighbors.into_iter().collect())
}

// ============================================================================
// Errors
// ============================================================================

/// Canonical error vocabulary for this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorNetworkError {
    /// Identifier overflow.
    IdentifierOverflow,

    /// Invalid identifier.
    InvalidIdentifier {
        /// Invalid value.
        value: String,
    },

    /// Invalid attribute name.
    InvalidAttributeName {
        /// Invalid name.
        value: String,
    },

    /// Invalid index dimension.
    InvalidDimension {
        /// Invalid dimension.
        value: u64,
    },

    /// Duplicate tensor.
    DuplicateTensor {
        /// Tensor identifier.
        tensor: TensorId,
    },

    /// Duplicate index.
    DuplicateIndex {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// Unknown tensor.
    UnknownTensor {
        /// Tensor identifier.
        tensor: TensorId,
    },

    /// Unknown index.
    UnknownIndex {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// Duplicate tensor index.
    DuplicateTensorIndex {
        /// Tensor identifier.
        tensor: TensorId,

        /// Index identifier.
        index: TensorIndexId,
    },

    /// Duplicate endpoint.
    DuplicateEndpoint {
        /// Index identifier.
        index: TensorIndexId,

        /// Tensor identifier.
        tensor: TensorId,
    },

    /// Too many endpoints.
    TooManyEndpoints {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// Invalid index binding.
    InvalidIndexBinding {
        /// Index identifier.
        index: TensorIndexId,

        /// Reason.
        reason: &'static str,
    },

    /// Index already has a different qubit.
    IndexAlreadyBound {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// Qubit already has another tensor-network index.
    QubitAlreadyBound {
        /// Logical qubit.
        qubit: QubitId,
    },

    /// A virtual index has invalid endpoint count.
    InvalidVirtualIndex {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// An open index is connected to multiple tensors.
    InvalidOpenIndex {
        /// Index identifier.
        index: TensorIndexId,
    },

    /// Tensor/index graph mismatch.
    DisconnectedTensorIndex {
        /// Tensor identifier.
        tensor: TensorId,

        /// Index identifier.
        index: TensorIndexId,
    },

    /// Qubit/index binding mismatch.
    BindingMismatch {
        /// Qubit.
        qubit: QubitId,

        /// Index.
        index: TensorIndexId,
    },

    /// Duplicate output qubit.
    DuplicateOutputQubit {
        /// Qubit.
        qubit: QubitId,
    },

    /// Output qubit has no network binding.
    UnboundOutputQubit {
        /// Qubit.
        qubit: QubitId,
    },

    /// Invalid topology.
    InvalidTopology {
        /// Explanation.
        reason: &'static str,
    },

    /// Invalid approximation policy.
    InvalidApproximationPolicy,

    /// Invalid tensor value.
    InvalidValue {
        /// Explanation.
        reason: &'static str,
    },
}

impl fmt::Display for TensorNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentifierOverflow => {
                write!(f, "tensor-network identifier overflow")
            }

            Self::InvalidIdentifier { value } => {
                write!(f, "invalid tensor-network identifier `{value}`")
            }

            Self::InvalidAttributeName { value } => {
                write!(f, "invalid tensor-network attribute name `{value}`")
            }

            Self::InvalidDimension { value } => {
                write!(f, "invalid tensor-network dimension `{value}`")
            }

            Self::DuplicateTensor { tensor } => {
                write!(f, "duplicate tensor `{tensor}`")
            }

            Self::DuplicateIndex { index } => {
                write!(f, "duplicate tensor-network index `{index}`")
            }

            Self::UnknownTensor { tensor } => {
                write!(f, "unknown tensor `{tensor}`")
            }

            Self::UnknownIndex { index } => {
                write!(f, "unknown tensor-network index `{index}`")
            }

            Self::DuplicateTensorIndex { tensor, index } => {
                write!(f, "tensor `{tensor}` contains duplicate index `{index}`")
            }

            Self::DuplicateEndpoint { index, tensor } => {
                write!(f, "index `{index}` already contains endpoint `{tensor}`")
            }

            Self::TooManyEndpoints { index } => {
                write!(f, "index `{index}` has more than two endpoints")
            }

            Self::InvalidIndexBinding { index, reason } => {
                write!(f, "invalid binding for index `{index}`: {reason}")
            }

            Self::IndexAlreadyBound { index } => {
                write!(f, "index `{index}` is already bound")
            }

            Self::QubitAlreadyBound { qubit } => {
                write!(f, "qubit `{qubit}` is already bound to another tensor index")
            }

            Self::InvalidVirtualIndex { index } => {
                write!(f, "virtual index `{index}` must have exactly two endpoints")
            }

            Self::InvalidOpenIndex { index } => {
                write!(f, "open index `{index}` cannot have multiple endpoints")
            }

            Self::DisconnectedTensorIndex { tensor, index } => {
                write!(f, "tensor `{tensor}` does not connect through index `{index}`")
            }

            Self::BindingMismatch { qubit, index } => {
                write!(f, "qubit `{qubit}` does not match index `{index}` binding")
            }

            Self::DuplicateOutputQubit { qubit } => {
                write!(f, "duplicate output qubit `{qubit}`")
            }

            Self::UnboundOutputQubit { qubit } => {
                write!(f, "output qubit `{qubit}` has no tensor-network binding")
            }

            Self::InvalidTopology { reason } => {
                write!(f, "invalid tensor-network topology: {reason}")
            }

            Self::InvalidApproximationPolicy => {
                write!(f, "invalid tensor-network approximation policy")
            }

            Self::InvalidValue { reason } => {
                write!(f, "invalid tensor-network value: {reason}")
            }
        }
    }
}

impl std::error::Error for TensorNetworkError {}

// ============================================================================
// Internal validation helpers
// ============================================================================

fn validate_identifier(value: &str) -> TensorNetworkResult<()> {
    if value.is_empty() {
        return Err(TensorNetworkError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    let mut characters = value.chars();

    let first = characters
        .next()
        .ok_or_else(|| TensorNetworkError::InvalidIdentifier {
            value: value.to_owned(),
        })?;

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(TensorNetworkError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    if characters.any(|character| {
        !(character == '_' || character == '-' || character.is_ascii_alphanumeric())
    }) {
        return Err(TensorNetworkError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_attribute_name(value: &str) -> TensorNetworkResult<()> {
    if value.is_empty() {
        return Err(TensorNetworkError::InvalidAttributeName {
            value: value.to_owned(),
        });
    }

    if value.contains(char::is_whitespace) {
        return Err(TensorNetworkError::InvalidAttributeName {
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn is_connected_subset(
    tensors: &BTreeMap<TensorId, TensorNode>,
    indices: &BTreeMap<TensorIndexId, TensorIndex>,
    subset: &[TensorId],
) -> bool {
    if subset.len() <= 1 {
        return true;
    }

    let allowed: BTreeSet<TensorId> = subset.iter().copied().collect();

    let start = subset[0];
    let mut visited = BTreeSet::new();
    let mut queue = vec![start];

    while let Some(current) = queue.pop() {
        if !visited.insert(current) {
            continue;
        }

        if let Some(tensor) = tensors.get(&current) {
            for index_id in tensor.indices() {
                if let Some(index) = indices.get(index_id) {
                    for endpoint in index.endpoints() {
                        if allowed.contains(endpoint) && !visited.contains(endpoint) {
                            queue.push(*endpoint);
                        }
                    }
                }
            }
        }
    }

    visited.len() == allowed.len()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_distinct_namespaces() {
        let tensor = TensorId::new(1);
        let index = TensorIndexId::new(1);
        let network = TensorNetworkId::new(1);

        assert_eq!(tensor.value(), 1);
        assert_eq!(index.value(), 1);
        assert_eq!(network.value(), 1);
    }

    #[test]
    fn finite_dimension_rejects_zero() {
        assert!(IndexDimension::finite(0).is_err());
        assert!(IndexDimension::finite(2).is_ok());
    }

    #[test]
    fn symbolic_dimension_is_valid() {
        assert!(IndexDimension::symbolic("N").is_ok());
        assert!(IndexDimension::symbolic("1bad").is_err());
    }

    #[test]
    fn product_state_uses_canonical_qubit_ids() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let network = product_state(TensorNetworkId::new(0), [q0, q1]).unwrap();

        assert_eq!(network.tensor_count(), 2);
        assert_eq!(network.index_count(), 2);
        assert_eq!(network.qubit_bindings().get(&q0), Some(&TensorIndexId::new(0)));
        assert_eq!(network.qubit_bindings().get(&q1), Some(&TensorIndexId::new(1)));
    }

    #[test]
    fn virtual_index_requires_two_endpoints() {
        let mut network = empty_network(TensorNetworkId::new(0), TensorNetworkKind::General);

        let index = TensorIndex::new(
            TensorIndexId::new(0),
            IndexDimension::finite(4).unwrap(),
            TensorIndexRole::Virtual,
        )
        .unwrap();

        network.add_index(index).unwrap();

        let a = TensorNode::new(
            TensorId::new(0),
            TensorKind::Auxiliary,
            vec![TensorIndexId::new(0)],
            TensorValue::symbol("A").unwrap(),
        )
        .unwrap();

        let b = TensorNode::new(
            TensorId::new(1),
            TensorKind::Auxiliary,
            vec![TensorIndexId::new(0)],
            TensorValue::symbol("B").unwrap(),
        )
        .unwrap();

        network.add_tensor(a).unwrap();
        network.add_tensor(b).unwrap();

        network.connect(TensorId::new(0), TensorIndexId::new(0)).unwrap();
        network.connect(TensorId::new(1), TensorIndexId::new(0)).unwrap();

        assert!(network.validate().is_ok());
    }

    #[test]
    fn open_physical_index_cannot_have_two_endpoints() {
        let mut network = empty_network(TensorNetworkId::new(0), TensorNetworkKind::State);

        let index = TensorIndex::new(
            TensorIndexId::new(0),
            IndexDimension::finite(2).unwrap(),
            TensorIndexRole::Physical,
        )
        .unwrap();

        network.add_index(index).unwrap();

        let a = TensorNode::new(
            TensorId::new(0),
            TensorKind::State,
            vec![TensorIndexId::new(0)],
            TensorValue::symbol("A").unwrap(),
        )
        .unwrap();

        let b = TensorNode::new(
            TensorId::new(1),
            TensorKind::State,
            vec![TensorIndexId::new(0)],
            TensorValue::symbol("B").unwrap(),
        )
        .unwrap();

        network.add_tensor(a).unwrap();
        network.add_tensor(b).unwrap();

        network.connect(TensorId::new(0), TensorIndexId::new(0)).unwrap();
        assert!(network.connect(TensorId::new(1), TensorIndexId::new(0)).is_err());
    }

    #[test]
    fn qubit_cannot_bind_to_two_indices() {
        let mut network = empty_network(TensorNetworkId::new(0), TensorNetworkKind::State);

        let q = QubitId::new(0);

        for id in 0..2 {
            network
                .add_index(
                    TensorIndex::new(
                        TensorIndexId::new(id),
                        IndexDimension::finite(2).unwrap(),
                        TensorIndexRole::Physical,
                    )
                    .unwrap(),
                )
                .unwrap();
        }

        network.bind_qubit(q, TensorIndexId::new(0)).unwrap();

        assert!(network.bind_qubit(q, TensorIndexId::new(1)).is_err());
    }

    #[test]
    fn deterministic_neighbors() {
        let mut network = empty_network(TensorNetworkId::new(0), TensorNetworkKind::General);

        for id in 0..3 {
            network
                .add_index(
                    TensorIndex::new(
                        TensorIndexId::new(id),
                        IndexDimension::finite(2).unwrap(),
                        TensorIndexRole::Virtual,
                    )
                    .unwrap(),
                )
                .unwrap();
        }

        let tensors = [
            TensorNode::new(
                TensorId::new(0),
                TensorKind::General,
                vec![TensorIndexId::new(0), TensorIndexId::new(1)],
                TensorValue::symbol("A").unwrap(),
            )
            .unwrap(),
            TensorNode::new(
                TensorId::new(1),
                TensorKind::General,
                vec![TensorIndexId::new(0), TensorIndexId::new(2)],
                TensorValue::symbol("B").unwrap(),
            )
            .unwrap(),
            TensorNode::new(
                TensorId::new(2),
                TensorKind::General,
                vec![TensorIndexId::new(2), TensorIndexId::new(1)],
                TensorValue::symbol("C").unwrap(),
            )
            .unwrap(),
        ];

        for tensor in tensors {
            network.add_tensor(tensor).unwrap();
        }

        for index in 0..3 {
            let endpoints = match index {
                0 => [TensorId::new(0), TensorId::new(1)],
                1 => [TensorId::new(0), TensorId::new(2)],
                2 => [TensorId::new(1), TensorId::new(2)],
                _ => unreachable!(),
            };

            network
                .connect(endpoints[0], TensorIndexId::new(index))
                .unwrap();
            network
                .connect(endpoints[1], TensorIndexId::new(index))
                .unwrap();
        }

        assert_eq!(
            tensor_neighbors(&network, TensorId::new(0)).unwrap(),
            vec![TensorId::new(1), TensorId::new(2)]
        );
    }

    #[test]
    fn approximation_policy_rejects_constraints_when_disabled() {
        let policy = ApproximationPolicy {
            allow_approximation: false,
            maximum_discarded_weight: Some("1e-10".to_owned()),
            ..ApproximationPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn empty_network_is_valid() {
        let network = empty_network(TensorNetworkId::new(7), TensorNetworkKind::General);

        assert!(network.validate().is_ok());
        assert!(network.is_empty());
    }
}