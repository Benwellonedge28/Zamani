//! Zamani Quantum Noise (ZQN) — Channel Representation Contracts.
//!
//! This module defines the representation-independent vocabulary used to
//! describe mathematical representations of quantum channels.
//!
//! # Architectural role
//!
//! `quantum::zqn::channel::representation` is a dependency-low channel layer.
//!
//! It answers:
//!
//! > "What mathematical representation is being used to express this
//! > quantum process, and what are its structural dimensions and guarantees?"
//!
//! It does NOT implement:
//!
//! - quantum-channel mathematics;
//! - Kraus operator storage;
//! - Choi matrix storage;
//! - process-matrix storage;
//! - Pauli-transfer matrices;
//! - stochastic-map storage;
//! - Lindblad integration;
//! - numerical linear algebra;
//! - simulation;
//! - sampling;
//! - hardware APIs;
//! - target-specific representations;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - measurement execution;
//! - quantum-resource identity.
//!
//! Those responsibilities belong to the corresponding ZQN modules or
//! integration layers.
//!
//! # Representation architecture
//!
//! ```text
//!                 QuantumChannel
//!                       │
//!                       ▼
//!              ChannelRepresentation
//!                       │
//!          ┌────────────┼─────────────┐
//!          ▼            ▼             ▼
//!       Kraus          Choi       ProcessMatrix
//!          │            │             │
//!          ├────────────┼─────────────┤
//!          ▼            ▼             ▼
//!       PauliTransfer  Stochastic   Lindblad
//!                       │
//!                       ▼
//!                  representation
//! ```
//!
//! The representation is metadata/contract information. Concrete numerical
//! objects belong to their respective representation modules.
//!
//! # Write once, scale everywhere
//!
//! This module intentionally does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_QUDITS
//! MAX_MATRIX_SIZE
//! MAX_KRAUS_OPERATORS
//! MAX_CHANNEL_DIMENSION
//! MAX_PROCESS_DIMENSION
//! ```
//!
//! No semantic upper bound is imposed by this file.
//!
//! Dimensions are represented as portable mathematical quantities and may be
//! arbitrarily large subject to the actual resources available to the caller.
//!
//! Runtime admission and allocation policy belong to `zqn::core::limits` and
//! the runtime/resource-management layers.
//!
//! # Quantum identity
//!
//! This module intentionally does not define `QubitId`, `PhysicalQubitId`, or
//! an equivalent identity.
//!
//! The canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A representation describes mathematical dimensions, not ownership of
//! physical resources.
//!
//! Higher-level channel/noise integration may associate a representation with
//! canonical IR resources, but that association must remain outside this
//! low-level representation contract.
//!
//! # Generality
//!
//! A production quantum system cannot assume every computation is a collection
//! of two-level qubits.
//!
//! This representation contract therefore supports:
//!
//! - qubits;
//! - qudits;
//! - logical quantum resources;
//! - bosonic modes;
//! - continuous-variable truncations;
//! - subsystem composites;
//! - rectangular maps where mathematically valid;
//! - distributed/composite channels;
//! - future quantum modalities.
//!
//! The representation layer does not decide which physical modality a
//! dimension represents.
//!
//! # Determinism
//!
//! All structural metadata defined here is deterministic.
//!
//! It contains no:
//!
//! - random number generators;
//! - wall-clock timestamps;
//! - process IDs;
//! - memory addresses;
//! - thread IDs;
//! - hash-map iteration;
//! - global mutable state.
//!
//! # Resource safety
//!
//! This module performs only checked structural arithmetic.
//!
//! In particular, methods that derive matrix element counts, operator counts,
//! or tensor dimensions return errors instead of allowing integer overflow.
//!
//! This module never allocates a matrix merely to inspect its dimensions.
//!
//! # Numerical semantics
//!
//! This module deliberately does not assume `f64`.
//!
//! Numerical precision belongs to the concrete representation or numerical
//! backend.
//!
//! A representation descriptor can therefore be used with:
//!
//! - exact arithmetic;
//! - arbitrary precision arithmetic;
//! - floating-point arithmetic;
//! - interval arithmetic;
//! - symbolic representations;
//! - sampled representations.
//!
//! # Approximation
//!
//! A representation descriptor does not silently imply that a conversion is
//! exact.
//!
//! Conversion policy belongs to the conversion/target layers.
//!
//! A future conversion API should explicitly distinguish:
//!
//! ```text
//! Exact
//! Approximate { tolerance }
//! Bounded { error_bound }
//! Statistical { confidence }
//! Unsupported
//! ```
//!
//! This file provides enough structural information for those layers to make
//! that decision without changing this file.
//!
//! # Serialization
//!
//! The enums and descriptor types are serializable semantic values.
//!
//! Their Rust memory layout is NOT the wire format.
//!
//! `serde` is used only for stable field-based serialization. A higher-level
//! ZQN schema module remains responsible for schema versioning, canonical
//! serialization, migration, and compatibility policy.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ```text
//! channel/representation.rs
//!          │
//!     ┌────┼─────────────┐
//!     ▼    ▼             ▼
//! kraus  choi     process_matrix
//!     │    │             │
//!     └────┼─────────────┘
//!          ▼
//! channel/channel.rs
//!          │
//!     ┌────┼─────────────┐
//!     ▼    ▼             ▼
//! simulation target   propagation
//! ```
//!
//! The representation module must remain independent of those consumers.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. every supported representation has a stable semantic identity;
//! 2. representation properties are explicit;
//! 3. input/output subsystem dimensions are represented without fixed-size
//!    assumptions;
//! 4. derived dimensions use checked arithmetic;
//! 5. matrix/tensor element counts cannot silently overflow;
//! 6. no concrete numerical matrix is required;
//! 7. no qubit identity is duplicated;
//! 8. no vendor/backend knowledge exists here;
//! 9. no artificial machine-size ceiling exists;
//! 10. representation compatibility can be inspected without allocation;
//! 11. serialization is deterministic at the field level;
//! 12. no unsafe code exists;
//! 13. the API is usable independently by future channel implementations;
//! 14. later representation implementations do not require modifying this
//!     file merely to use its contracts;
//! 15. adding a larger quantum machine does not require changing this file;
//! 16. adding a new representation is an explicit API extension rather than a
//!     hidden reinterpretation of an existing representation.
//!
//! # Testing
//!
//! Unit tests at the bottom of this file verify:
//!
//! - representation classification;
//! - dimension validation;
//! - checked dimension products;
//! - matrix-size derivation;
//! - operator-size derivation;
//! - deterministic display;
//! - serialization round trips;
//! - no artificial size ceiling;
//! - representation capability classification.
//!
//! Mathematical validation of actual channel contents belongs to the concrete
//! representation modules.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};

// ============================================================================
// Portable dimension type
// ============================================================================

/// Portable mathematical dimension used by ZQN representation descriptors.
///
/// A dimension is not a machine allocation size and therefore must not be
/// interpreted as a `usize` without an explicit checked conversion.
///
/// `u128` is used so structural arithmetic can remain independent of the host
/// architecture.
///
/// This type does not impose a semantic maximum on a quantum system. It merely
/// defines the portable finite integer domain available to this Rust
/// implementation.
///
/// Actual allocation/execution feasibility belongs to resource policy and
/// runtime layers.
pub type RepresentationDimension = u128;

/// Portable count of mathematical elements.
///
/// This is deliberately distinct from [`RepresentationDimension`] because a
/// matrix may contain the square/product of dimensions.
pub type RepresentationElementCount = u128;

/// Portable count of representation operators.
///
/// For example, a Kraus representation can contain multiple operators.
pub type RepresentationOperatorCount = u128;

// ============================================================================
// Representation kind
// ============================================================================

/// Canonical semantic kinds of quantum-channel representation.
///
/// This enum identifies the mathematical representation, not its storage
/// implementation.
///
/// Adding a new representation is a deliberate API extension. Existing
/// variants must not silently change meaning.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum ChannelRepresentationKind {
    /// Operator-sum representation using Kraus operators.
    Kraus,

    /// Choi-Jamiołkowski representation.
    Choi,

    /// General process-matrix representation.
    ProcessMatrix,

    /// Pauli transfer / Pauli-Liouville representation.
    PauliTransfer,

    /// Stochastic classical map representation.
    ///
    /// This is useful for channels whose semantics are intentionally
    /// restricted to a classical/stochastic subspace.
    Stochastic,

    /// Lindblad/GKSL generator representation.
    Lindblad,

    /// Superoperator representation.
    Superoperator,

    /// Liouville-space representation.
    Liouville,

    /// Generic tensor representation.
    ///
    /// This is intentionally semantic rather than tied to a particular tensor
    /// library.
    Tensor,

    /// Symbolic representation.
    ///
    /// Used when a channel is represented symbolically rather than by a
    /// materialized numerical object.
    Symbolic,

    /// Sampled/empirical representation.
    ///
    /// This represents a channel or process statistically rather than as an
    /// exact mathematical operator object.
    Sampled,

    /// User-defined extension identified by a stable semantic name.
    Extension,
}

impl ChannelRepresentationKind {
    /// Returns the stable semantic identifier for this representation kind.
    ///
    /// These strings are intended for diagnostics, configuration, logging and
    /// future schema layers. They are not Rust type names.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Kraus => "kraus",
            Self::Choi => "choi",
            Self::ProcessMatrix => "process_matrix",
            Self::PauliTransfer => "pauli_transfer",
            Self::Stochastic => "stochastic",
            Self::Lindblad => "lindblad",
            Self::Superoperator => "superoperator",
            Self::Liouville => "liouville",
            Self::Tensor => "tensor",
            Self::Symbolic => "symbolic",
            Self::Sampled => "sampled",
            Self::Extension => "extension",
        }
    }

    /// Returns true when the representation is naturally matrix/operator based.
    #[must_use]
    pub const fn is_matrix_like(self) -> bool {
        match self {
            Self::Kraus
            | Self::Choi
            | Self::ProcessMatrix
            | Self::PauliTransfer
            | Self::Stochastic
            | Self::Superoperator
            | Self::Liouville => true,

            Self::Lindblad
            | Self::Tensor
            | Self::Symbolic
            | Self::Sampled
            | Self::Extension => false,
        }
    }

    /// Returns true when the representation is naturally operator-valued.
    #[must_use]
    pub const fn is_operator_based(self) -> bool {
        match self {
            Self::Kraus
            | Self::Lindblad
            | Self::Superoperator
            | Self::Liouville => true,

            Self::Choi
            | Self::ProcessMatrix
            | Self::PauliTransfer
            | Self::Stochastic
            | Self::Tensor
            | Self::Symbolic
            | Self::Sampled
            | Self::Extension => false,
        }
    }

    /// Returns true when the representation can naturally describe sampled
    /// empirical behavior without requiring a complete exact channel matrix.
    #[must_use]
    pub const fn supports_empirical_form(self) -> bool {
        matches!(
            self,
            Self::Stochastic | Self::Sampled | Self::Extension
        )
    }

    /// Returns true when this representation can naturally express a
    /// continuous-time generator.
    #[must_use]
    pub const fn supports_generator_semantics(self) -> bool {
        matches!(self, Self::Lindblad | Self::Extension)
    }

    /// Returns true when the representation can naturally describe arbitrary
    /// channels rather than only a restricted channel family.
    #[must_use]
    pub const fn is_general_purpose(self) -> bool {
        matches!(
            self,
            Self::Kraus
                | Self::Choi
                | Self::ProcessMatrix
                | Self::Superoperator
                | Self::Liouville
                | Self::Tensor
                | Self::Symbolic
                | Self::Extension
        )
    }
}

impl fmt::Display for ChannelRepresentationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Representation fidelity semantics
// ============================================================================

/// Semantic exactness of a representation.
///
/// This does not claim that a numerical implementation has infinite precision.
/// It describes the intended mathematical relationship between the
/// representation and the channel semantics.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum RepresentationExactness {
    /// The representation is intended to encode the exact mathematical
    /// channel semantics.
    Exact,

    /// The representation is explicitly approximate.
    Approximate,

    /// The representation is known only within a declared bound.
    Bounded,

    /// The representation describes statistical observations rather than an
    /// exact channel.
    Statistical,

    /// The representation's exactness has not yet been established.
    Unknown,
}

impl Default for RepresentationExactness {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for RepresentationExactness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => formatter.write_str("exact"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Bounded => formatter.write_str("bounded"),
            Self::Statistical => formatter.write_str("statistical"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// ============================================================================
// Representation algebra
// ============================================================================

/// Mathematical operations supported by a representation.
///
/// This is capability metadata, not implementation code.
///
/// A representation implementation may support additional optimized operations
/// internally, but it must not claim support through this metadata unless the
/// operation is semantically valid.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum RepresentationOperation {
    /// Composition of two compatible channels.
    Composition,

    /// Tensor product of independent channels.
    TensorProduct,

    /// Application to a compatible state/operator representation.
    Application,

    /// Conversion to another representation.
    Conversion,

    /// Validation of representation invariants.
    Validation,

    /// Adjoint/dual operation where mathematically defined.
    Adjoint,

    /// Partial trace where mathematically defined.
    PartialTrace,

    /// Sampling/empirical execution.
    Sampling,

    /// Generator evolution.
    Evolution,

    /// Differentiation with respect to parameters.
    Differentiation,
}

impl fmt::Display for RepresentationOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Composition => formatter.write_str("composition"),
            Self::TensorProduct => formatter.write_str("tensor_product"),
            Self::Application => formatter.write_str("application"),
            Self::Conversion => formatter.write_str("conversion"),
            Self::Validation => formatter.write_str("validation"),
            Self::Adjoint => formatter.write_str("adjoint"),
            Self::PartialTrace => formatter.write_str("partial_trace"),
            Self::Sampling => formatter.write_str("sampling"),
            Self::Evolution => formatter.write_str("evolution"),
            Self::Differentiation => formatter.write_str("differentiation"),
        }
    }
}

// ============================================================================
// Representation capabilities
// ============================================================================

/// Declares semantic capabilities of a channel representation.
///
/// This structure intentionally contains no backend-specific information.
///
/// A target/backend may have a different capability structure. Target
/// compatibility is determined by `zqn::target`, not here.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct RepresentationCapabilities {
    /// Whether the representation can express arbitrary input/output
    /// dimensions rather than requiring equal dimensions.
    pub supports_rectangular_maps: bool,

    /// Whether the representation naturally supports correlated/multi-resource
    /// channels.
    pub supports_correlated_resources: bool,

    /// Whether the representation can express time-dependent behavior.
    pub supports_time_dependence: bool,

    /// Whether the representation naturally supports continuous-time
    /// generator semantics.
    pub supports_continuous_time: bool,

    /// Whether the representation can be sampled directly.
    pub supports_sampling: bool,

    /// Whether composition is a first-class operation.
    pub supports_composition: bool,

    /// Whether tensor-product construction is a first-class operation.
    pub supports_tensor_product: bool,

    /// Whether direct conversion to another representation can be meaningful.
    pub supports_conversion: bool,

    /// Whether the representation can preserve symbolic parameters.
    pub supports_symbolic_parameters: bool,

    /// Whether the representation can describe empirical/statistical data.
    pub supports_statistical_data: bool,
}

impl RepresentationCapabilities {
    /// Capabilities commonly associated with a general operator-sum channel.
    #[must_use]
    pub const fn kraus() -> Self {
        Self {
            supports_rectangular_maps: true,
            supports_correlated_resources: true,
            supports_time_dependence: true,
            supports_continuous_time: false,
            supports_sampling: true,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: false,
        }
    }

    /// Capabilities commonly associated with a Choi representation.
    #[must_use]
    pub const fn choi() -> Self {
        Self {
            supports_rectangular_maps: true,
            supports_correlated_resources: true,
            supports_time_dependence: false,
            supports_continuous_time: false,
            supports_sampling: false,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: false,
        }
    }

    /// Capabilities commonly associated with a process matrix.
    #[must_use]
    pub const fn process_matrix() -> Self {
        Self {
            supports_rectangular_maps: true,
            supports_correlated_resources: true,
            supports_time_dependence: true,
            supports_continuous_time: false,
            supports_sampling: true,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: false,
        }
    }

    /// Capabilities commonly associated with a Pauli-transfer
    /// representation.
    #[must_use]
    pub const fn pauli_transfer() -> Self {
        Self {
            supports_rectangular_maps: false,
            supports_correlated_resources: true,
            supports_time_dependence: true,
            supports_continuous_time: false,
            supports_sampling: true,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: false,
        }
    }

    /// Capabilities commonly associated with a stochastic representation.
    #[must_use]
    pub const fn stochastic() -> Self {
        Self {
            supports_rectangular_maps: true,
            supports_correlated_resources: true,
            supports_time_dependence: true,
            supports_continuous_time: false,
            supports_sampling: true,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: true,
        }
    }

    /// Capabilities commonly associated with a Lindblad generator.
    #[must_use]
    pub const fn lindblad() -> Self {
        Self {
            supports_rectangular_maps: false,
            supports_correlated_resources: true,
            supports_time_dependence: true,
            supports_continuous_time: true,
            supports_sampling: true,
            supports_composition: true,
            supports_tensor_product: true,
            supports_conversion: true,
            supports_symbolic_parameters: true,
            supports_statistical_data: false,
        }
    }

    /// Returns the capabilities associated with the supplied representation
    /// kind.
    ///
    /// Extension representations intentionally receive conservative defaults.
    #[must_use]
    pub const fn for_kind(kind: ChannelRepresentationKind) -> Self {
        match kind {
            ChannelRepresentationKind::Kraus => Self::kraus(),
            ChannelRepresentationKind::Choi => Self::choi(),
            ChannelRepresentationKind::ProcessMatrix => Self::process_matrix(),
            ChannelRepresentationKind::PauliTransfer => Self::pauli_transfer(),
            ChannelRepresentationKind::Stochastic => Self::stochastic(),
            ChannelRepresentationKind::Lindblad => Self::lindblad(),

            ChannelRepresentationKind::Superoperator
            | ChannelRepresentationKind::Liouville
            | ChannelRepresentationKind::Tensor
            | ChannelRepresentationKind::Symbolic
            | ChannelRepresentationKind::Sampled
            | ChannelRepresentationKind::Extension => Self {
                supports_rectangular_maps: true,
                supports_correlated_resources: true,
                supports_time_dependence: true,
                supports_continuous_time: false,
                supports_sampling: true,
                supports_composition: true,
                supports_tensor_product: true,
                supports_conversion: true,
                supports_symbolic_parameters: true,
                supports_statistical_data: true,
            },
        }
    }
}

// ============================================================================
// Dimension specification
// ============================================================================

/// Input/output Hilbert-space dimensions of a channel.
///
/// A channel maps an input operator/state space into an output operator/state
/// space. The input and output Hilbert dimensions therefore belong explicitly
/// in the representation contract.
///
/// No fixed qubit count is assumed.
///
/// Examples:
///
/// ```text
/// one qubit       -> 2 -> 2
/// one qutrit      -> 3 -> 3
/// qubit-to-qutrit -> 2 -> 3
/// composite       -> product of constituent dimensions
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ChannelDimensions {
    /// Input Hilbert-space dimension.
    pub input: RepresentationDimension,

    /// Output Hilbert-space dimension.
    pub output: RepresentationDimension,
}

impl ChannelDimensions {
    /// Constructs validated channel dimensions.
    ///
    /// Dimensions must be non-zero because a zero-dimensional Hilbert space
    /// does not represent a valid quantum subsystem.
    pub const fn new(
        input: RepresentationDimension,
        output: RepresentationDimension,
    ) -> ZqnResult<Self> {
        if input == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                "channel input dimension must be greater than zero",
            ));
        }

        if output == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                "channel output dimension must be greater than zero",
            ));
        }

        Ok(Self { input, output })
    }

    /// Returns true when input and output dimensions are equal.
    #[must_use]
    pub const fn is_square(self) -> bool {
        self.input == self.output
    }

    /// Returns the number of elements in a rectangular operator mapping
    /// between the input and output Hilbert spaces.
    ///
    /// This uses checked arithmetic.
    pub const fn operator_element_count(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        match self.input.checked_mul(self.output) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "channel operator element count overflowed",
            )),
        }
    }

    /// Returns the input operator-space dimension `d_in²`.
    pub const fn input_operator_space_dimension(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        match self.input.checked_mul(self.input) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "input operator-space dimension overflowed",
            )),
        }
    }

    /// Returns the output operator-space dimension `d_out²`.
    pub const fn output_operator_space_dimension(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        match self.output.checked_mul(self.output) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "output operator-space dimension overflowed",
            )),
        }
    }

    /// Returns the dimension of a general linear superoperator mapping
    /// between input and output operator spaces.
    ///
    /// This is:
    ///
    /// ```text
    /// d_out² × d_in²
    /// ```
    ///
    /// The result is checked.
    pub const fn superoperator_element_count(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        let output = match self.output.checked_mul(self.output) {
            Some(value) => value,
            None => {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::SizeOverflow,
                    "output operator-space dimension overflowed",
                ));
            }
        };

        let input = match self.input.checked_mul(self.input) {
            Some(value) => value,
            None => {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::SizeOverflow,
                    "input operator-space dimension overflowed",
                ));
            }
        };

        match output.checked_mul(input) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "superoperator element count overflowed",
            )),
        }
    }

    /// Returns the number of elements in a Choi-style matrix for this channel.
    ///
    /// The conventional Choi matrix dimension is:
    ///
    /// ```text
    /// d_out × d_in
    /// ```
    ///
    /// and therefore contains:
    ///
    /// ```text
    /// (d_out × d_in)²
    /// ```
    ///
    /// elements.
    pub const fn choi_element_count(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        let dimension = match self.input.checked_mul(self.output) {
            Some(value) => value,
            None => {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::SizeOverflow,
                    "Choi matrix dimension overflowed",
                ));
            }
        };

        match dimension.checked_mul(dimension) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "Choi matrix element count overflowed",
            )),
        }
    }

    /// Returns the Hilbert-space dimension of the composite input/output
    /// pairing.
    ///
    /// This is useful for descriptors without requiring matrix allocation.
    pub const fn product_dimension(
        self,
    ) -> ZqnResult<RepresentationDimension> {
        match self.input.checked_mul(self.output) {
            Some(value) => Ok(value),
            None => Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::SizeOverflow,
                "input/output dimension product overflowed",
            )),
        }
    }
}

// ============================================================================
// Subsystem dimension construction
// ============================================================================

/// Multiplies a sequence of subsystem dimensions using checked arithmetic.
///
/// This is deliberately generic and does not know whether the resources are:
///
/// - qubits;
/// - qutrits;
/// - bosonic truncations;
/// - logical qubits;
/// - physical qubits;
/// - other quantum resources.
///
/// The caller remains responsible for associating the dimensions with actual
/// canonical IR resources.
///
/// An empty sequence is rejected because it does not describe a non-empty
/// quantum subsystem.
pub fn product_dimensions<I>(
    dimensions: I,
) -> ZqnResult<RepresentationDimension>
where
    I: IntoIterator<Item = RepresentationDimension>,
{
    let mut result = 1u128;
    let mut saw_dimension = false;

    for dimension in dimensions {
        saw_dimension = true;

        if dimension == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                "subsystem dimension must be greater than zero",
            ));
        }

        result = match result.checked_mul(dimension) {
            Some(value) => value,
            None => {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::SizeOverflow,
                    "composite subsystem dimension overflowed",
                ));
            }
        };
    }

    if !saw_dimension {
        return Err(ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::InvalidRepresentation,
            "at least one subsystem dimension is required",
        ));
    }

    Ok(result)
}

/// Returns the Hilbert dimension of `count` identical subsystems, each having
/// dimension `dimension`.
///
/// This is checked and does not allocate.
pub fn repeated_dimension_product(
    dimension: RepresentationDimension,
    count: RepresentationDimension,
) -> ZqnResult<RepresentationDimension> {
    if dimension == 0 {
        return Err(ZqnError::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::ChannelDimensionMismatch,
            "subsystem dimension must be greater than zero",
        ));
    }

    if count == 0 {
        return Err(ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::InvalidRepresentation,
            "subsystem count must be greater than zero",
        ));
    }

    let mut result = 1u128;
    let mut remaining = count;

    while remaining > 0 {
        result = match result.checked_mul(dimension) {
            Some(value) => value,
            None => {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::SizeOverflow,
                    "repeated subsystem dimension overflowed",
                ));
            }
        };

        remaining -= 1;
    }

    Ok(result)
}

// ============================================================================
// Operator count
// ============================================================================

/// Describes how many operators a representation contains.
///
/// `Unknown` is valid for lazy, symbolic, empirical, or externally supplied
/// representations where materializing/counting operators is not part of the
/// representation contract.
///
/// `Finite(n)` describes a known finite number of operators.
///
/// There is intentionally no `Infinite` variant: an actual finite execution
/// artifact must ultimately obey an explicit resource policy if materialized.
/// Symbolic representations can remain lazy without claiming an infinite
/// materialized collection.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum OperatorCount {
    /// A known finite number of operators.
    Finite(RepresentationOperatorCount),

    /// Operator count is intentionally not materialized/known.
    Unknown,
}

impl OperatorCount {
    /// Creates a finite operator count.
    ///
    /// Zero is rejected because a channel representation containing no
    /// operators cannot represent an operator-sum channel.
    pub const fn finite(
        count: RepresentationOperatorCount,
    ) -> ZqnResult<Self> {
        if count == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::InvalidRepresentation,
                "operator count must be greater than zero",
            ));
        }

        Ok(Self::Finite(count))
    }

    /// Returns the finite count, if known.
    #[must_use]
    pub const fn finite_count(self) -> Option<RepresentationOperatorCount> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unknown => None,
        }
    }

    /// Returns true when the operator count is known.
    #[must_use]
    pub const fn is_known(self) -> bool {
        matches!(self, Self::Finite(_))
    }
}

impl fmt::Display for OperatorCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(value) => value.fmt(formatter),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// ============================================================================
// Representation shape
// ============================================================================

/// Structural shape of a channel representation.
///
/// This structure intentionally describes mathematical shape rather than
/// storage layout.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct RepresentationShape {
    /// Channel input/output dimensions.
    pub channel: ChannelDimensions,

    /// Number of independent representation operators, if known.
    pub operators: OperatorCount,
}

impl RepresentationShape {
    /// Creates a validated representation shape.
    pub const fn new(
        channel: ChannelDimensions,
        operators: OperatorCount,
    ) -> Self {
        Self {
            channel,
            operators,
        }
    }

    /// Returns the number of elements in one rectangular operator.
    pub const fn operator_element_count(
        self,
    ) -> ZqnResult<RepresentationElementCount> {
        self.channel.operator_element_count()
    }

    /// Returns the number of elements in all known operators.
    ///
    /// If the operator count is unknown, this method returns `None`.
    pub const fn total_operator_elements(
        self,
    ) -> ZqnResult<Option<RepresentationElementCount>> {
        let operator_elements = match self.channel.operator_element_count() {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        match self.operators {
            OperatorCount::Finite(count) => {
                match operator_elements.checked_mul(count) {
                    Some(value) => Ok(Some(value)),
                    None => Err(ZqnError::new(
                        ZqnErrorKind::Representation,
                        ZqnErrorCode::SizeOverflow,
                        "total operator element count overflowed",
                    )),
                }
            }

            OperatorCount::Unknown => Ok(None),
        }
    }
}

// ============================================================================
// Representation descriptor
// ============================================================================

/// Canonical descriptor for a channel representation.
///
/// This is the primary object exported by this file.
///
/// It does not contain actual numerical channel data. Concrete representations
/// such as Kraus or Choi data should hold a descriptor or equivalent structural
/// information and use this contract for validation and interoperability.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ChannelRepresentation {
    /// Semantic representation kind.
    pub kind: ChannelRepresentationKind,

    /// Mathematical shape of the represented channel.
    pub shape: RepresentationShape,

    /// Mathematical exactness of the representation.
    pub exactness: RepresentationExactness,

    /// Declared representation capabilities.
    pub capabilities: RepresentationCapabilities,

    /// Optional stable extension identifier.
    ///
    /// This is used only when `kind == Extension`.
    ///
    /// The identifier is semantic and should be stable across processes and
    /// machines. It must not contain a memory address or process-local value.
    pub extension_id: Option<String>,
}

impl ChannelRepresentation {
    /// Creates a representation descriptor using the canonical capabilities
    /// associated with the representation kind.
    pub fn new(
        kind: ChannelRepresentationKind,
        shape: RepresentationShape,
        exactness: RepresentationExactness,
    ) -> ZqnResult<Self> {
        let extension_id = if matches!(kind, ChannelRepresentationKind::Extension) {
            None
        } else {
            None
        };

        let descriptor = Self {
            kind,
            shape,
            exactness,
            capabilities: RepresentationCapabilities::for_kind(kind),
            extension_id,
        };

        descriptor.validate()?;

        Ok(descriptor)
    }

    /// Creates an extension representation with an explicit stable identifier.
    pub fn extension(
        extension_id: String,
        shape: RepresentationShape,
        exactness: RepresentationExactness,
        capabilities: RepresentationCapabilities,
    ) -> ZqnResult<Self> {
        if extension_id.trim().is_empty() {
            return Err(ZqnError::new(
                ZqnErrorKind::Extension,
                ZqnErrorCode::InvalidRepresentation,
                "extension representation identifier must not be empty",
            ));
        }

        let descriptor = Self {
            kind: ChannelRepresentationKind::Extension,
            shape,
            exactness,
            capabilities,
            extension_id: Some(extension_id),
        };

        descriptor.validate()?;

        Ok(descriptor)
    }

    /// Validates the complete structural descriptor.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.shape.channel.input == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                "channel input dimension must be greater than zero",
            ));
        }

        if self.shape.channel.output == 0 {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                "channel output dimension must be greater than zero",
            ));
        }

        if let OperatorCount::Finite(count) = self.shape.operators {
            if count == 0 {
                return Err(ZqnError::new(
                    ZqnErrorKind::Representation,
                    ZqnErrorCode::InvalidRepresentation,
                    "finite operator count must be greater than zero",
                ));
            }
        }

        match self.kind {
            ChannelRepresentationKind::Extension => {
                match self.extension_id.as_deref() {
                    Some(identifier) if !identifier.trim().is_empty() => {}
                    _ => {
                        return Err(ZqnError::new(
                            ZqnErrorKind::Extension,
                            ZqnErrorCode::InvalidRepresentation,
                            "extension representation requires a stable identifier",
                        ));
                    }
                }
            }

            _ => {
                if self.extension_id.is_some() {
                    return Err(ZqnError::new(
                        ZqnErrorKind::Representation,
                        ZqnErrorCode::InvalidRepresentation,
                        "built-in representations must not carry an extension identifier",
                    ));
                }
            }
        }

        if !self.capabilities.supports_rectangular_maps
            && !self.shape.channel.is_square()
        {
            return Err(ZqnError::new(
                ZqnErrorKind::Representation,
                ZqnErrorCode::ChannelDimensionMismatch,
                "representation does not support rectangular channel dimensions",
            ));
        }

        Ok(())
    }

    /// Returns the semantic representation identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        match (&self.kind, self.extension_id.as_deref()) {
            (ChannelRepresentationKind::Extension, Some(identifier)) => identifier,
            _ => self.kind.as_str(),
        }
    }

    /// Returns true when this descriptor represents a square channel.
    #[must_use]
    pub const fn is_square(&self) -> bool {
        self.shape.channel.is_square()
    }

    /// Returns the channel dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> ChannelDimensions {
        self.shape.channel
    }

    /// Returns the declared operator count.
    #[must_use]
    pub const fn operator_count(&self) -> OperatorCount {
        self.shape.operators
    }

    /// Returns the total number of mathematical operator elements when known.
    pub const fn total_operator_elements(
        &self,
    ) -> ZqnResult<Option<RepresentationElementCount>> {
        self.shape.total_operator_elements()
    }

    /// Returns the number of elements in the corresponding Choi matrix.
    pub const fn choi_element_count(
        &self,
    ) -> ZqnResult<RepresentationElementCount> {
        self.shape.channel.choi_element_count()
    }

    /// Returns the number of elements in a generic superoperator matrix.
    pub const fn superoperator_element_count(
        &self,
    ) -> ZqnResult<RepresentationElementCount> {
        self.shape.channel.superoperator_element_count()
    }

    /// Returns whether the representation claims support for an operation.
    #[must_use]
    pub const fn supports(
        &self,
        operation: RepresentationOperation,
    ) -> bool {
        match operation {
            RepresentationOperation::Composition => {
                self.capabilities.supports_composition
            }

            RepresentationOperation::TensorProduct => {
                self.capabilities.supports_tensor_product
            }

            RepresentationOperation::Application => true,

            RepresentationOperation::Conversion => {
                self.capabilities.supports_conversion
            }

            RepresentationOperation::Validation => true,

            RepresentationOperation::Adjoint => {
                self.capabilities.supports_conversion
            }

            RepresentationOperation::PartialTrace => {
                self.capabilities.supports_conversion
            }

            RepresentationOperation::Sampling => {
                self.capabilities.supports_sampling
            }

            RepresentationOperation::Evolution => {
                self.capabilities.supports_continuous_time
            }

            RepresentationOperation::Differentiation => {
                self.capabilities.supports_symbolic_parameters
            }
        }
    }

    /// Returns true if this representation may express correlations between
    /// multiple resources.
    #[must_use]
    pub const fn supports_correlations(&self) -> bool {
        self.capabilities.supports_correlated_resources
    }

    /// Returns true if this representation can express time-dependent channel
    /// behavior.
    #[must_use]
    pub const fn supports_time_dependence(&self) -> bool {
        self.capabilities.supports_time_dependence
    }

    /// Returns true if this representation supports continuous-time semantics.
    #[must_use]
    pub const fn supports_continuous_time(&self) -> bool {
        self.capabilities.supports_continuous_time
    }
}

impl fmt::Display for ChannelRepresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{} -> {}; operators={}; exactness={}]",
            self.identifier(),
            self.shape.channel.input,
            self.shape.channel.output,
            self.shape.operators,
            self.exactness,
        )
    }
}

// ============================================================================
// Compatibility
// ============================================================================

/// Result of structural compatibility inspection between two representations.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum RepresentationCompatibility {
    /// Representations have compatible structural dimensions and may be
    /// candidates for conversion.
    Compatible,

    /// The dimensions are incompatible.
    DimensionMismatch,

    /// The requested operation is not supported by the representation.
    Unsupported,

    /// The representations require an explicit approximation policy.
    ApproximationRequired,
}

impl fmt::Display for RepresentationCompatibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible => formatter.write_str("compatible"),
            Self::DimensionMismatch => formatter.write_str("dimension_mismatch"),
            Self::Unsupported => formatter.write_str("unsupported"),
            Self::ApproximationRequired => {
                formatter.write_str("approximation_required")
            }
        }
    }
}

/// Inspects whether two channel representations have compatible channel
/// dimensions.
///
/// This function does not perform a conversion and does not allocate.
#[must_use]
pub const fn check_dimension_compatibility(
    source: ChannelDimensions,
    target: ChannelDimensions,
) -> RepresentationCompatibility {
    if source.input == target.input && source.output == target.output {
        RepresentationCompatibility::Compatible
    } else {
        RepresentationCompatibility::DimensionMismatch
    }
}

/// Inspects whether a representation can be used for a requested operation.
///
/// This is deliberately a structural check only. Mathematical validation of
/// concrete channel data remains the responsibility of the representation
/// implementation.
#[must_use]
pub const fn check_operation_support(
    representation: &ChannelRepresentation,
    operation: RepresentationOperation,
) -> RepresentationCompatibility {
    if representation.supports(operation) {
        RepresentationCompatibility::Compatible
    } else {
        RepresentationCompatibility::Unsupported
    }
}

// ============================================================================
// Standard representation descriptors
// ============================================================================

/// Creates a canonical Kraus representation descriptor.
///
/// The actual Kraus operators are owned by `channel::kraus`.
pub fn kraus_descriptor(
    dimensions: ChannelDimensions,
    operators: OperatorCount,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(dimensions, operators);

    ChannelRepresentation::new(
        ChannelRepresentationKind::Kraus,
        shape,
        exactness,
    )
}

/// Creates a canonical Choi representation descriptor.
///
/// The actual Choi data are owned by `channel::choi`.
pub fn choi_descriptor(
    dimensions: ChannelDimensions,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(
        dimensions,
        OperatorCount::Finite(1),
    );

    ChannelRepresentation::new(
        ChannelRepresentationKind::Choi,
        shape,
        exactness,
    )
}

/// Creates a canonical process-matrix representation descriptor.
pub fn process_matrix_descriptor(
    dimensions: ChannelDimensions,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(
        dimensions,
        OperatorCount::Finite(1),
    );

    ChannelRepresentation::new(
        ChannelRepresentationKind::ProcessMatrix,
        shape,
        exactness,
    )
}

/// Creates a canonical Pauli-transfer representation descriptor.
///
/// The actual Pauli basis and matrix are owned by `channel::pauli`.
pub fn pauli_transfer_descriptor(
    dimensions: ChannelDimensions,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(
        dimensions,
        OperatorCount::Finite(1),
    );

    ChannelRepresentation::new(
        ChannelRepresentationKind::PauliTransfer,
        shape,
        exactness,
    )
}

/// Creates a canonical stochastic representation descriptor.
pub fn stochastic_descriptor(
    dimensions: ChannelDimensions,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(
        dimensions,
        OperatorCount::Finite(1),
    );

    ChannelRepresentation::new(
        ChannelRepresentationKind::Stochastic,
        shape,
        exactness,
    )
}

/// Creates a canonical Lindblad representation descriptor.
///
/// The actual generator parameters/operators are owned by
/// `channel::lindblad`.
pub fn lindblad_descriptor(
    dimensions: ChannelDimensions,
    exactness: RepresentationExactness,
) -> ZqnResult<ChannelRepresentation> {
    let shape = RepresentationShape::new(
        dimensions,
        OperatorCount::Unknown,
    );

    ChannelRepresentation::new(
        ChannelRepresentationKind::Lindblad,
        shape,
        exactness,
    )
}

// ============================================================================
// Checked conversion helpers
// ============================================================================

/// Converts a portable representation dimension into the host allocation
/// index type.
///
/// This function is deliberately explicit.
///
/// Mathematical ZQN dimensions are not silently cast to `usize`, because doing
/// so would turn a scalable semantic value into an architecture-dependent
/// allocation value.
///
/// Callers should use this only at the actual allocation boundary and only
/// after `ZqnLimits`/runtime admission has approved the request.
pub fn dimension_to_usize(
    dimension: RepresentationDimension,
) -> ZqnResult<usize> {
    usize::try_from(dimension).map_err(|_| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            "representation dimension cannot be represented by the host allocation index type",
        )
    })
}

/// Converts a portable element count into a host allocation index type.
///
/// This is an allocation-boundary operation and does not impose a ZQN
/// architectural limit.
pub fn element_count_to_usize(
    count: RepresentationElementCount,
) -> ZqnResult<usize> {
    usize::try_from(count).map_err(|_| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            "representation element count cannot be represented by the host allocation index type",
        )
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_dimensions_reject_zero_input() {
        let result = ChannelDimensions::new(0, 2);

        assert!(result.is_err());
    }

    #[test]
    fn channel_dimensions_reject_zero_output() {
        let result = ChannelDimensions::new(2, 0);

        assert!(result.is_err());
    }

    #[test]
    fn channel_dimensions_accept_non_qubit_dimensions() {
        let dimensions = ChannelDimensions::new(3, 5)
            .expect("3 -> 5 should be structurally valid");

        assert_eq!(dimensions.input, 3);
        assert_eq!(dimensions.output, 5);
        assert!(!dimensions.is_square());
    }

    #[test]
    fn square_dimension_is_detected() {
        let dimensions = ChannelDimensions::new(16, 16)
            .expect("16 -> 16 should be structurally valid");

        assert!(dimensions.is_square());
    }

    #[test]
    fn operator_element_count_is_checked() {
        let dimensions = ChannelDimensions::new(4, 8)
            .expect("4 -> 8 should be structurally valid");

        assert_eq!(
            dimensions
                .operator_element_count()
                .expect("4 * 8 must fit"),
            32
        );
    }

    #[test]
    fn operator_space_dimension_is_checked() {
        let dimensions = ChannelDimensions::new(4, 8)
            .expect("4 -> 8 should be structurally valid");

        assert_eq!(
            dimensions
                .input_operator_space_dimension()
                .expect("4 * 4 must fit"),
            16
        );

        assert_eq!(
            dimensions
                .output_operator_space_dimension()
                .expect("8 * 8 must fit"),
            64
        );
    }

    #[test]
    fn superoperator_element_count_is_correct() {
        let dimensions = ChannelDimensions::new(2, 4)
            .expect("2 -> 4 should be structurally valid");

        assert_eq!(
            dimensions
                .superoperator_element_count()
                .expect("superoperator size must fit"),
            64
        );
    }

    #[test]
    fn choi_element_count_is_correct() {
        let dimensions = ChannelDimensions::new(2, 4)
            .expect("2 -> 4 should be structurally valid");

        assert_eq!(
            dimensions
                .choi_element_count()
                .expect("Choi size must fit"),
            64
        );
    }

    #[test]
    fn dimension_product_supports_arbitrary_subsystem_dimensions() {
        let result = product_dimensions([2, 3, 5, 7])
            .expect("finite product should succeed");

        assert_eq!(result, 210);
    }

    #[test]
    fn dimension_product_rejects_empty_input() {
        let result = product_dimensions(core::iter::empty());

        assert!(result.is_err());
    }

    #[test]
    fn dimension_product_rejects_zero_dimension() {
        let result = product_dimensions([2, 0, 3]);

        assert!(result.is_err());
    }

    #[test]
    fn repeated_dimension_product_is_correct() {
        let result = repeated_dimension_product(2, 10)
            .expect("2^10 must fit");

        assert_eq!(result, 1024);
    }

    #[test]
    fn repeated_dimension_product_rejects_zero_count() {
        let result = repeated_dimension_product(2, 0);

        assert!(result.is_err());
    }

    #[test]
    fn finite_operator_count_rejects_zero() {
        let result = OperatorCount::finite(0);

        assert!(result.is_err());
    }

    #[test]
    fn finite_operator_count_is_known() {
        let count = OperatorCount::finite(17)
            .expect("17 operators must be valid");

        assert!(count.is_known());
        assert_eq!(count.finite_count(), Some(17));
    }

    #[test]
    fn unknown_operator_count_is_not_materialized() {
        let count = OperatorCount::Unknown;

        assert!(!count.is_known());
        assert_eq!(count.finite_count(), None);
    }

    #[test]
    fn kraus_descriptor_is_valid() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("qubit channel dimensions must be valid");

        let descriptor = kraus_descriptor(
            dimensions,
            OperatorCount::finite(4)
                .expect("four operators must be valid"),
            RepresentationExactness::Exact,
        )
        .expect("Kraus descriptor must be valid");

        assert_eq!(
            descriptor.kind,
            ChannelRepresentationKind::Kraus
        );

        assert_eq!(
            descriptor.operator_count().finite_count(),
            Some(4)
        );
    }

    #[test]
    fn choi_descriptor_is_valid() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("qubit channel dimensions must be valid");

        let descriptor = choi_descriptor(
            dimensions,
            RepresentationExactness::Exact,
        )
        .expect("Choi descriptor must be valid");

        assert_eq!(
            descriptor.kind,
            ChannelRepresentationKind::Choi
        );

        assert_eq!(
            descriptor.identifier(),
            "choi"
        );
    }

    #[test]
    fn lindblad_descriptor_can_have_unknown_operator_count() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("qubit dimensions must be valid");

        let descriptor = lindblad_descriptor(
            dimensions,
            RepresentationExactness::Exact,
        )
        .expect("Lindblad descriptor must be valid");

        assert_eq!(
            descriptor.operator_count(),
            OperatorCount::Unknown
        );

        assert!(
            descriptor
                .supports_continuous_time()
        );
    }

    #[test]
    fn extension_requires_identifier() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("dimensions must be valid");

        let result = ChannelRepresentation::extension(
            String::new(),
            RepresentationShape::new(
                dimensions,
                OperatorCount::Finite(1),
            ),
            RepresentationExactness::Unknown,
            RepresentationCapabilities::for_kind(
                ChannelRepresentationKind::Extension,
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn extension_descriptor_is_valid() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("dimensions must be valid");

        let descriptor = ChannelRepresentation::extension(
            "future.quantum.rep.v1".to_owned(),
            RepresentationShape::new(
                dimensions,
                OperatorCount::Unknown,
            ),
            RepresentationExactness::Unknown,
            RepresentationCapabilities::for_kind(
                ChannelRepresentationKind::Extension,
            ),
        )
        .expect("extension descriptor must be valid");

        assert_eq!(
            descriptor.identifier(),
            "future.quantum.rep.v1"
        );
    }

    #[test]
    fn dimension_compatibility_accepts_equal_dimensions() {
        let left = ChannelDimensions::new(4, 4)
            .expect("dimensions must be valid");

        let right = ChannelDimensions::new(4, 4)
            .expect("dimensions must be valid");

        assert_eq!(
            check_dimension_compatibility(left, right),
            RepresentationCompatibility::Compatible
        );
    }

    #[test]
    fn dimension_compatibility_rejects_different_dimensions() {
        let left = ChannelDimensions::new(4, 4)
            .expect("dimensions must be valid");

        let right = ChannelDimensions::new(4, 8)
            .expect("dimensions must be valid");

        assert_eq!(
            check_dimension_compatibility(left, right),
            RepresentationCompatibility::DimensionMismatch
        );
    }

    #[test]
    fn representation_operation_support_is_reported() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("dimensions must be valid");

        let descriptor = kraus_descriptor(
            dimensions,
            OperatorCount::finite(2)
                .expect("operator count must be valid"),
            RepresentationExactness::Exact,
        )
        .expect("descriptor must be valid");

        assert_eq!(
            check_operation_support(
                &descriptor,
                RepresentationOperation::Composition,
            ),
            RepresentationCompatibility::Compatible
        );
    }

    #[test]
    fn representation_display_is_deterministic() {
        let dimensions = ChannelDimensions::new(2, 2)
            .expect("dimensions must be valid");

        let descriptor = kraus_descriptor(
            dimensions,
            OperatorCount::finite(2)
                .expect("operator count must be valid"),
            RepresentationExactness::Exact,
        )
        .expect("descriptor must be valid");

        assert_eq!(
            descriptor.to_string(),
            "kraus [2 -> 2; operators=2; exactness=exact]"
        );
    }

    #[test]
    fn serialization_round_trip_preserves_descriptor() {
        let dimensions = ChannelDimensions::new(2, 3)
            .expect("dimensions must be valid");

        let descriptor = kraus_descriptor(
            dimensions,
            OperatorCount::finite(5)
                .expect("operator count must be valid"),
            RepresentationExactness::Approximate,
        )
        .expect("descriptor must be valid");

        let encoded = serde_json::to_string(&descriptor)
            .expect("descriptor must serialize");

        let decoded: ChannelRepresentation =
            serde_json::from_str(&encoded)
                .expect("descriptor must deserialize");

        assert_eq!(descriptor, decoded);
    }

    #[test]
    fn huge_finite_dimension_is_not_rejected_as_a_semantic_machine_limit() {
        let dimensions = ChannelDimensions::new(
            u128::MAX,
            u128::MAX,
        );

        /*
         * The constructor itself only validates that dimensions are non-zero.
         * It must not impose an arbitrary architectural maximum.
         *
         * Derived quantities can legitimately fail with checked arithmetic.
         */
        assert!(dimensions.is_ok());

        let dimensions =
            dimensions.expect("u128::MAX is a valid finite semantic dimension");

        assert!(
            dimensions
                .operator_element_count()
                .is_err()
        );
    }

    #[test]
    fn host_conversion_is_explicit() {
        let result = dimension_to_usize(2);

        assert_eq!(
            result.expect("2 must fit usize"),
            2usize
        );
    }

    #[test]
    fn representation_kind_strings_are_stable() {
        assert_eq!(
            ChannelRepresentationKind::Kraus.as_str(),
            "kraus"
        );

        assert_eq!(
            ChannelRepresentationKind::Choi.as_str(),
            "choi"
        );

        assert_eq!(
            ChannelRepresentationKind::PauliTransfer.as_str(),
            "pauli_transfer"
        );

        assert_eq!(
            ChannelRepresentationKind::Lindblad.as_str(),
            "lindblad"
        );
    }
}