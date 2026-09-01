//! Zamani Quantum Noise (ZQN) — Quantum Channel Abstraction
//!
//! This module defines the representation-independent semantic abstraction for
//! quantum channels.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the canonical `QuantumChannel` trait;
//! - channel identity;
//! - channel domain and support metadata;
//! - input/output subsystem descriptions;
//! - channel representation classification;
//! - channel composition semantics at the abstraction level;
//! - channel validation contracts;
//! - channel capability requirements;
//! - channel approximation/exactness contracts;
//! - channel provenance-independent semantic metadata;
//! - representation-independent channel errors;
//! - deterministic semantic channel descriptors.
//!
//! This file does NOT own:
//!
//! - Kraus matrices;
//! - Choi matrices;
//! - Pauli transfer matrices;
//! - density matrices;
//! - state-vector simulation;
//! - numerical linear algebra;
//! - Monte Carlo sampling;
//! - Lindblad integration;
//! - pulse simulation;
//! - hardware APIs;
//! - calibration storage;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - frontend parsing;
//! - serialization formats;
//! - backend execution.
//!
//! Those responsibilities belong to the corresponding ZQN subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             │ semantic operation
//!                             ▼
//!                    ┌────────────────────┐
//!                    │       ZQN          │
//!                    │                    │
//!                    │ QuantumChannel     │
//!                    └─────────┬──────────┘
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!           Kraus             Choi            Pauli
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              │
//!                              ▼
//!                       simulator/runtime
//! ```
//!
//! The channel abstraction is deliberately independent of any particular
//! mathematical representation.
//!
//! # Canonical qubit identity
//!
//! Whenever a channel explicitly refers to logical qubits, this module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! It does NOT define another `QubitId`.
//!
//! The canonical IR owns logical and physical quantum-resource identity.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no:
//!
//! - maximum qubit count;
//! - maximum channel arity;
//! - maximum matrix dimension;
//! - maximum operation count;
//! - vendor-specific limit;
//! - technology-specific limit;
//! - fixed gate set;
//! - fixed topology;
//! - fixed numerical representation.
//!
//! A channel's size is derived from its declared support and subsystem
//! dimensions, subject only to explicit caller/runtime resource limits.
//!
//! "Infinity" therefore means:
//!
//! > No artificial finite machine-size ceiling is encoded into the channel
//! > semantics.
//!
//! Actual computation remains bounded by available memory, CPU/GPU resources,
//! distributed resources, execution policy, numerical representation and target
//! capabilities.
//!
//! # Mathematical contract
//!
//! A physical quantum channel is normally a completely-positive,
//! trace-preserving (CPTP) map. However, this abstraction also supports
//! explicitly identified intermediate/non-physical maps so that validation and
//! compiler pipelines can represent incomplete constructions without silently
//! claiming physical validity.
//!
//! Implementations must therefore explicitly report whether they are:
//!
//! - physically validated;
//! - physically unvalidated;
//! - known non-physical;
//! - approximate;
//! - exact.
//!
//! No implementation may silently turn an invalid or approximate channel into
//! an apparently exact physical channel.
//!
//! # Representation independence
//!
//! The abstraction supports representations such as:
//!
//! - Kraus;
//! - Choi;
//! - Pauli transfer;
//! - Liouville/superoperator;
//! - stochastic;
//! - Lindblad generator;
//! - process matrix;
//! - future representations.
//!
//! This file does not assume any particular representation is universally
//! superior.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! Unsafe Rust is explicitly forbidden.
//!
//! # Thread safety
//!
//! The semantic channel contracts are designed so implementations can be
//! `Send + Sync` whenever their contained representation is also `Send + Sync`.
//!
//! This file does not require global mutable state and does not maintain global
//! caches or RNG state.
//!
//! # Determinism
//!
//! Channel semantics are deterministic descriptions.
//!
//! Random sampling is deliberately NOT part of this abstraction because random
//! execution belongs to `zqn::simulation` and must use an explicit execution
//! context and seed policy.
//!
//! A channel therefore never owns a hidden RNG.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! channel::QuantumChannel
//!      │
//!      ├──────────────► simulation
//!      ├──────────────► propagation
//!      ├──────────────► calibration
//!      ├──────────────► characterization
//!      ├──────────────► routing
//!      ├──────────────► scheduling
//!      ├──────────────► QEC
//!      └──────────────► hardware/runtime adapters
//! ```
//!
//! Downstream systems consume this abstraction. They must not create competing
//! channel semantics.
//!
//! # Future implementation files
//!
//! The intended consumers are:
//!
//! ```text
//! channel/representation.rs
//! channel/kraus.rs
//! channel/choi.rs
//! channel/process_matrix.rs
//! channel/pauli.rs
//! channel/stochastic.rs
//! channel/lindblad.rs
//! channel/thermal.rs
//! channel/amplitude.rs
//! channel/phase.rs
//! channel/depolarizing.rs
//! channel/generalized.rs
//! channel/composition.rs
//! ```
//!
//! None of those implementation files should need to modify the semantic
//! contract defined here merely because another representation is added.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. every channel implementation can implement `QuantumChannel`;
//! 2. no channel implementation needs a competing channel trait;
//! 3. logical qubits use canonical `QubitId`;
//! 4. no machine-size constant is present;
//! 5. no vendor-specific behavior is present;
//! 6. exactness/approximation is explicit;
//! 7. physical validity is explicit;
//! 8. composition has representation-independent semantics;
//! 9. resource requirements are declarative;
//! 10. errors are explicit and structured;
//! 11. deterministic semantic identity is available;
//! 12. sampling is not hidden inside channel semantics;
//! 13. no unsafe code is required;
//! 14. Rust 1.97/1.97.1 remains sufficient.
//!
//! # Example
//!
//! A concrete Kraus implementation will eventually look conceptually like:
//!
//! ```text
//! KrausChannel
//!     │
//!     └── implements QuantumChannel
//!             │
//!             ├── input_support()
//!             ├── output_support()
//!             ├── representation()
//!             ├── validation()
//!             └── descriptor()
//! ```
//!
//! The exact mathematical representation remains outside this file.

#![forbid(unsafe_code)]

use crate::quantum::ir::qubit::QubitId;
use std::fmt;

// =============================================================================
// Result and error model
// =============================================================================

/// Result type used by the representation-independent channel API.
pub type ChannelResult<T> = Result<T, ChannelError>;

/// Errors that can occur while constructing, validating, composing or
/// interrogating a quantum channel.
///
/// This type deliberately describes semantic failures rather than failures of
/// a particular numerical library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
    /// The channel has no valid support.
    EmptySupport,

    /// The same canonical qubit appears more than once in a channel support.
    DuplicateQubit(QubitId),

    /// A subsystem dimension is invalid.
    InvalidDimension {
        /// Position of the subsystem in the channel support.
        index: usize,

        /// Invalid dimension.
        dimension: usize,
    },

    /// Input and output domains are incompatible.
    DomainMismatch {
        /// Number of input subsystems.
        input_arity: usize,

        /// Number of output subsystems.
        output_arity: usize,
    },

    /// A requested representation is unsupported.
    UnsupportedRepresentation(ChannelRepresentation),

    /// A requested channel operation is unsupported.
    UnsupportedOperation(ChannelOperation),

    /// The channel is known not to satisfy the requested physical property.
    NotPhysical(&'static str),

    /// The channel has not been validated sufficiently for the requested
    /// operation.
    NotValidated,

    /// An approximation was required but the policy forbids it.
    ApproximationRequired,

    /// The requested approximation tolerance is invalid.
    InvalidTolerance,

    /// The requested error bound is invalid.
    InvalidErrorBound,

    /// A composition cannot be represented without violating an explicit
    /// semantic constraint.
    IncompatibleComposition,

    /// A declared channel identity is invalid.
    InvalidIdentity,

    /// A required capability is unavailable.
    MissingCapability(ChannelCapability),

    /// A resource requirement cannot be represented by the requested execution
    /// environment.
    ResourceRequirementUnavailable,

    /// The channel contains a non-finite semantic parameter.
    NonFiniteParameter,

    /// A channel parameter is outside its declared domain.
    ParameterOutOfRange,

    /// The implementation cannot prove a requested property.
    PropertyUndetermined(&'static str),
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySupport => write!(f, "quantum channel support is empty"),

            Self::DuplicateQubit(qubit) => {
                write!(f, "quantum channel contains duplicate qubit {:?}", qubit)
            }

            Self::InvalidDimension { index, dimension } => write!(
                f,
                "invalid subsystem dimension {} at support position {}",
                dimension, index
            ),

            Self::DomainMismatch {
                input_arity,
                output_arity,
            } => write!(
                f,
                "channel input/output arity mismatch: input={}, output={}",
                input_arity, output_arity
            ),

            Self::UnsupportedRepresentation(representation) => {
                write!(f, "unsupported channel representation: {}", representation)
            }

            Self::UnsupportedOperation(operation) => {
                write!(f, "unsupported channel operation: {}", operation)
            }

            Self::NotPhysical(reason) => {
                write!(f, "channel is not physically valid: {}", reason)
            }

            Self::NotValidated => {
                write!(f, "channel has not been sufficiently validated")
            }

            Self::ApproximationRequired => {
                write!(f, "requested operation requires an approximation")
            }

            Self::InvalidTolerance => {
                write!(f, "invalid approximation tolerance")
            }

            Self::InvalidErrorBound => {
                write!(f, "invalid channel error bound")
            }

            Self::IncompatibleComposition => {
                write!(f, "channels cannot be composed under the requested semantics")
            }

            Self::InvalidIdentity => {
                write!(f, "invalid quantum channel identity")
            }

            Self::MissingCapability(capability) => {
                write!(f, "required channel capability is unavailable: {}", capability)
            }

            Self::ResourceRequirementUnavailable => {
                write!(f, "required channel resources are unavailable")
            }

            Self::NonFiniteParameter => {
                write!(f, "channel contains a non-finite parameter")
            }

            Self::ParameterOutOfRange => {
                write!(f, "channel parameter is outside its declared domain")
            }

            Self::PropertyUndetermined(property) => {
                write!(f, "channel property could not be determined: {}", property)
            }
        }
    }
}

impl std::error::Error for ChannelError {}

// =============================================================================
// Channel identity
// =============================================================================

/// Stable semantic identity for a channel.
///
/// `ChannelId` is deliberately opaque.
///
/// The value is not a hardware address, memory address, array index, vendor
/// identifier or machine-size indicator.
///
/// A future canonical identity subsystem may construct IDs from canonical
/// semantic hashes. This abstraction intentionally does not prescribe the hash
/// algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChannelId(u128);

impl ChannelId {
    /// Creates a channel identity from an explicitly supplied stable value.
    ///
    /// This constructor does not imply that the supplied value is globally
    /// unique. Callers responsible for canonical identity must provide a
    /// collision-resistant value according to their identity policy.
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    /// Returns the underlying opaque identity value.
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

// =============================================================================
// Subsystem identity
// =============================================================================

/// Semantic identity of a channel subsystem.
///
/// Qubit-based channels MUST use the canonical `QubitId`.
///
/// The additional variants allow the channel abstraction to remain useful for
/// future quantum technologies without introducing competing qubit identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelSubsystemId {
    /// Canonical Zamani logical qubit.
    Qubit(QubitId),

    /// An opaque subsystem belonging to a target modality other than the
    /// canonical qubit namespace.
    ///
    /// The owner of the corresponding target/modality must define the meaning
    /// of this identifier.
    Opaque(u128),
}

impl ChannelSubsystemId {
    /// Creates a canonical qubit subsystem identifier.
    #[must_use]
    pub const fn qubit(id: QubitId) -> Self {
        Self::Qubit(id)
    }

    /// Creates an opaque subsystem identifier.
    #[must_use]
    pub const fn opaque(id: u128) -> Self {
        Self::Opaque(id)
    }

    /// Returns the canonical qubit identifier when this is a qubit subsystem.
    #[must_use]
    pub const fn as_qubit(self) -> Option<QubitId> {
        match self {
            Self::Qubit(id) => Some(id),
            Self::Opaque(_) => None,
        }
    }
}

// =============================================================================
// Subsystem description
// =============================================================================

/// Description of one input or output subsystem of a quantum channel.
///
/// The `dimension` is the Hilbert-space dimension of the subsystem.
///
/// Examples:
///
/// - qubit: dimension 2;
/// - qutrit: dimension 3;
/// - qudit: dimension N;
/// - bosonic truncation: explicitly supplied truncation dimension.
///
/// The dimension is semantic data and is never hard-coded by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChannelSubsystem {
    /// Semantic subsystem identity.
    id: ChannelSubsystemId,

    /// Hilbert-space dimension.
    dimension: usize,
}

impl ChannelSubsystem {
    /// Creates a validated subsystem descriptor.
    pub fn new(id: ChannelSubsystemId, dimension: usize) -> ChannelResult<Self> {
        if dimension < 2 {
            return Err(ChannelError::InvalidDimension {
                index: 0,
                dimension,
            });
        }

        Ok(Self { id, dimension })
    }

    /// Returns the subsystem identity.
    #[must_use]
    pub const fn id(self) -> ChannelSubsystemId {
        self.id
    }

    /// Returns the Hilbert-space dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the qubit identifier when this subsystem is a canonical qubit.
    #[must_use]
    pub const fn qubit(self) -> Option<QubitId> {
        self.id.as_qubit()
    }
}

// =============================================================================
// Channel support
// =============================================================================

/// Immutable description of the channel's input/output support.
///
/// The channel abstraction does not require input and output supports to have
/// the same cardinality. This permits future non-square maps and explicit
/// preparation/measurement-like channels while retaining a strict validation
/// contract.
///
/// For ordinary quantum channels representing a physical process from one
/// Hilbert space to another, input and output dimensions must be compatible with
/// the concrete mathematical representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelSupport {
    input: Vec<ChannelSubsystem>,
    output: Vec<ChannelSubsystem>,
}

impl ChannelSupport {
    /// Creates a support descriptor.
    ///
    /// Both input and output collections must be non-empty.
    ///
    /// Duplicate subsystem identities are rejected because a single subsystem
    /// cannot occupy the same tensor-product slot twice.
    pub fn new(
        input: Vec<ChannelSubsystem>,
        output: Vec<ChannelSubsystem>,
    ) -> ChannelResult<Self> {
        if input.is_empty() || output.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        validate_unique_support(&input)?;
        validate_unique_support(&output)?;

        Ok(Self { input, output })
    }

    /// Creates a channel support for the same subsystem set on input and
    /// output.
    pub fn square(subsystems: Vec<ChannelSubsystem>) -> ChannelResult<Self> {
        Self::new(subsystems.clone(), subsystems)
    }

    /// Returns the input subsystem collection.
    #[must_use]
    pub fn input(&self) -> &[ChannelSubsystem] {
        &self.input
    }

    /// Returns the output subsystem collection.
    #[must_use]
    pub fn output(&self) -> &[ChannelSubsystem] {
        &self.output
    }

    /// Returns the number of input subsystems.
    #[must_use]
    pub fn input_arity(&self) -> usize {
        self.input.len()
    }

    /// Returns the number of output subsystems.
    #[must_use]
    pub fn output_arity(&self) -> usize {
        self.output.len()
    }

    /// Returns the total input Hilbert-space dimension.
    ///
    /// Returns `None` if the product cannot be represented by `usize`.
    #[must_use]
    pub fn input_dimension(&self) -> Option<usize> {
        checked_dimension_product(&self.input)
    }

    /// Returns the total output Hilbert-space dimension.
    ///
    /// Returns `None` if the product cannot be represented by `usize`.
    #[must_use]
    pub fn output_dimension(&self) -> Option<usize> {
        checked_dimension_product(&self.output)
    }

    /// Returns all canonical logical qubits participating in the input.
    ///
    /// Non-qubit subsystems are ignored.
    #[must_use]
    pub fn input_qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.input.iter().filter_map(|subsystem| subsystem.qubit())
    }

    /// Returns all canonical logical qubits participating in the output.
    ///
    /// Non-qubit subsystems are ignored.
    #[must_use]
    pub fn output_qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.output.iter().filter_map(|subsystem| subsystem.qubit())
    }

    /// Returns whether input and output describe the same ordered subsystem
    /// dimensions.
    #[must_use]
    pub fn is_square(&self) -> bool {
        self.input.len() == self.output.len()
            && self
                .input
                .iter()
                .zip(self.output.iter())
                .all(|(input, output)| input.dimension() == output.dimension())
    }

    /// Performs representation-independent structural validation.
    pub fn validate(&self) -> ChannelResult<()> {
        if self.input.is_empty() || self.output.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        validate_unique_support(&self.input)?;
        validate_unique_support(&self.output)?;

        for (index, subsystem) in self.input.iter().enumerate() {
            if subsystem.dimension() < 2 {
                return Err(ChannelError::InvalidDimension {
                    index,
                    dimension: subsystem.dimension(),
                });
            }
        }

        for (index, subsystem) in self.output.iter().enumerate() {
            if subsystem.dimension() < 2 {
                return Err(ChannelError::InvalidDimension {
                    index,
                    dimension: subsystem.dimension(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Representation classification
// =============================================================================

/// Mathematical representation used by a concrete channel implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelRepresentation {
    /// Representation by Kraus operators.
    Kraus,

    /// Choi-Jamiołkowski representation.
    Choi,

    /// Pauli transfer / Pauli-Liouville representation.
    PauliTransfer,

    /// Generic Liouville/superoperator representation.
    Liouville,

    /// Classical stochastic representation.
    Stochastic,

    /// Continuous-time Lindblad generator.
    Lindblad,

    /// General process-matrix representation.
    ProcessMatrix,

    /// Representation supplied by an external/future modality.
    Custom,
}

impl fmt::Display for ChannelRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Kraus => "Kraus",
            Self::Choi => "Choi",
            Self::PauliTransfer => "PauliTransfer",
            Self::Liouville => "Liouville",
            Self::Stochastic => "Stochastic",
            Self::Lindblad => "Lindblad",
            Self::ProcessMatrix => "ProcessMatrix",
            Self::Custom => "Custom",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Channel operations
// =============================================================================

/// Operations that may be requested against a channel abstraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelOperation {
    /// Compose one channel after another.
    Compose,

    /// Tensor-product two independent channels.
    TensorProduct,

    /// Convert to another supported representation.
    ConvertRepresentation,

    /// Validate physicality.
    ValidatePhysicality,

    /// Apply the channel to a supported state representation.
    Apply,

    /// Obtain a representation-independent descriptor.
    Describe,

    /// Estimate an error/fidelity property.
    Estimate,
}

impl fmt::Display for ChannelOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Compose => "Compose",
            Self::TensorProduct => "TensorProduct",
            Self::ConvertRepresentation => "ConvertRepresentation",
            Self::ValidatePhysicality => "ValidatePhysicality",
            Self::Apply => "Apply",
            Self::Describe => "Describe",
            Self::Estimate => "Estimate",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Physical validity
// =============================================================================

/// Physical validity state of a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelPhysicality {
    /// The implementation has established the required physical conditions.
    Validated,

    /// The channel is structurally valid but physicality has not yet been
    /// established.
    Unvalidated,

    /// The implementation has established that the channel is not physical.
    NonPhysical,

    /// Physicality is mathematically conditional on an explicit approximation
    /// or parameter assumption.
    Conditional,
}

impl ChannelPhysicality {
    /// Returns whether the channel has been positively validated as physical.
    #[must_use]
    pub const fn is_validated(self) -> bool {
        matches!(self, Self::Validated)
    }

    /// Returns whether physicality is explicitly known to be invalid.
    #[must_use]
    pub const fn is_non_physical(self) -> bool {
        matches!(self, Self::NonPhysical)
    }
}

// =============================================================================
// Exactness / approximation
// =============================================================================

/// Semantic accuracy contract of a channel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelAccuracy {
    /// The channel represents the declared semantics without approximation.
    Exact,

    /// The channel is an approximation with a caller-declared tolerance.
    Approximate {
        /// Maximum declared approximation tolerance.
        tolerance: f64,
    },

    /// The channel has a declared bound on the induced representation error.
    Bounded {
        /// Maximum declared error bound.
        error_bound: f64,
    },

    /// The channel is statistically characterized.
    Statistical {
        /// Confidence level in `[0, 1]`.
        confidence: f64,
    },
}

impl ChannelAccuracy {
    /// Constructs an exact accuracy contract.
    #[must_use]
    pub const fn exact() -> Self {
        Self::Exact
    }

    /// Constructs an approximate accuracy contract.
    pub fn approximate(tolerance: f64) -> ChannelResult<Self> {
        validate_finite_non_negative(tolerance, ChannelError::InvalidTolerance)?;

        Ok(Self::Approximate { tolerance })
    }

    /// Constructs a bounded accuracy contract.
    pub fn bounded(error_bound: f64) -> ChannelResult<Self> {
        validate_finite_non_negative(error_bound, ChannelError::InvalidErrorBound)?;

        Ok(Self::Bounded { error_bound })
    }

    /// Constructs a statistical accuracy contract.
    pub fn statistical(confidence: f64) -> ChannelResult<Self> {
        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err(ChannelError::InvalidTolerance);
        }

        Ok(Self::Statistical { confidence })
    }

    /// Returns the declared numerical bound when one exists.
    #[must_use]
    pub fn bound(self) -> Option<f64> {
        match self {
            Self::Exact => Some(0.0),
            Self::Approximate { tolerance } => Some(tolerance),
            Self::Bounded { error_bound } => Some(error_bound),
            Self::Statistical { .. } => None,
        }
    }

    /// Returns whether this is an exact contract.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// Capabilities that a concrete implementation may require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelCapability {
    /// Arbitrary multi-subsystem channel support.
    ArbitraryArity,

    /// Non-square input/output maps.
    NonSquare,

    /// Correlated multi-subsystem noise.
    CorrelatedNoise,

    /// Time-dependent channel parameters.
    TimeDependent,

    /// Continuous-time channel dynamics.
    ContinuousTime,

    /// Non-Markovian/memory-bearing behavior.
    NonMarkovian,

    /// Leakage outside the computational subspace.
    Leakage,

    /// Loss/erasure behavior.
    Loss,

    /// Classical stochastic channel semantics.
    Stochastic,

    /// Exact mathematical representation.
    ExactRepresentation,

    /// Approximate representation with explicit error bounds.
    Approximation,

    /// Physicality verification.
    PhysicalValidation,
}

impl fmt::Display for ChannelCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::ArbitraryArity => "arbitrary-arity",
            Self::NonSquare => "non-square",
            Self::CorrelatedNoise => "correlated-noise",
            Self::TimeDependent => "time-dependent",
            Self::ContinuousTime => "continuous-time",
            Self::NonMarkovian => "non-Markovian",
            Self::Leakage => "leakage",
            Self::Loss => "loss",
            Self::Stochastic => "stochastic",
            Self::ExactRepresentation => "exact-representation",
            Self::Approximation => "approximation",
            Self::PhysicalValidation => "physical-validation",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Channel resource requirements
// =============================================================================

/// Resource requirements associated with a channel operation.
///
/// These values are advisory semantic requirements, not machine-size limits.
///
/// `None` means that this abstraction cannot determine the requirement without
/// materializing the concrete representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChannelResourceRequirements {
    /// Number of scalar elements required by the chosen representation when
    /// known.
    pub scalar_elements: Option<u128>,

    /// Number of bytes required by the chosen representation when known.
    pub memory_bytes: Option<u128>,

    /// Arithmetic operation estimate when known.
    pub arithmetic_operations: Option<u128>,
}

impl ChannelResourceRequirements {
    /// Creates an empty/unknown resource requirement descriptor.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            scalar_elements: None,
            memory_bytes: None,
            arithmetic_operations: None,
        }
    }

    /// Creates a descriptor with explicitly known values.
    #[must_use]
    pub const fn known(
        scalar_elements: Option<u128>,
        memory_bytes: Option<u128>,
        arithmetic_operations: Option<u128>,
    ) -> Self {
        Self {
            scalar_elements,
            memory_bytes,
            arithmetic_operations,
        }
    }
}

// =============================================================================
// Channel descriptor
// =============================================================================

/// Representation-independent immutable description of a quantum channel.
///
/// This descriptor allows compilers, routers, schedulers, simulators and
/// hardware adapters to reason about a channel without knowing its concrete
/// mathematical storage.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelDescriptor {
    /// Stable semantic identity.
    pub id: ChannelId,

    /// Human-readable semantic name.
    pub name: Option<String>,

    /// Input/output support.
    pub support: ChannelSupport,

    /// Concrete mathematical representation.
    pub representation: ChannelRepresentation,

    /// Physical validity state.
    pub physicality: ChannelPhysicality,

    /// Accuracy contract.
    pub accuracy: ChannelAccuracy,

    /// Declared resource requirements.
    pub resources: ChannelResourceRequirements,
}

impl ChannelDescriptor {
    /// Creates a descriptor after validating its structural fields.
    pub fn new(
        id: ChannelId,
        name: Option<String>,
        support: ChannelSupport,
        representation: ChannelRepresentation,
        physicality: ChannelPhysicality,
        accuracy: ChannelAccuracy,
        resources: ChannelResourceRequirements,
    ) -> ChannelResult<Self> {
        support.validate()?;

        if let Some(name) = &name {
            if name.trim().is_empty() {
                return Err(ChannelError::InvalidIdentity);
            }
        }

        Ok(Self {
            id,
            name,
            support,
            representation,
            physicality,
            accuracy,
            resources,
        })
    }
}

// =============================================================================
// Composition contract
// =============================================================================

/// Describes the semantic relationship between two channels in a composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelComposition {
    /// Apply the first channel and then the second channel.
    Sequential,

    /// Apply both channels independently as a tensor product.
    TensorProduct,
}

// =============================================================================
// QuantumChannel trait
// =============================================================================

/// Canonical representation-independent quantum-channel contract.
///
/// Concrete mathematical representations such as Kraus, Choi, Pauli-transfer,
/// Lindblad and process-matrix channels implement this trait.
///
/// The trait intentionally exposes semantic operations rather than numerical
/// storage.
///
/// # Object safety
///
/// The core inspection API is object-safe, allowing channels to be stored as
/// `Box<dyn QuantumChannel>` where dynamic dispatch is appropriate.
///
/// Generic conversion/composition helpers are supplied through associated
/// functions and separate constructors rather than forcing every implementation
/// to expose a universal numerical representation.
pub trait QuantumChannel: fmt::Debug + Send + Sync {
    /// Returns the stable semantic channel identity.
    fn id(&self) -> ChannelId;

    /// Returns a representation-independent descriptor.
    fn descriptor(&self) -> ChannelDescriptor;

    /// Returns the channel's support.
    fn support(&self) -> &ChannelSupport;

    /// Returns the concrete mathematical representation.
    fn representation(&self) -> ChannelRepresentation;

    /// Returns the channel's physical validity state.
    fn physicality(&self) -> ChannelPhysicality;

    /// Returns the channel's accuracy contract.
    fn accuracy(&self) -> ChannelAccuracy;

    /// Returns the capabilities required by this channel.
    ///
    /// Implementations should return only capabilities that are semantically
    /// required, not capabilities merely available from the implementation.
    fn required_capabilities(&self) -> &[ChannelCapability];

    /// Returns the channel's resource requirements.
    fn resource_requirements(&self) -> ChannelResourceRequirements;

    /// Performs representation-independent structural validation.
    fn validate(&self) -> ChannelResult<()> {
        self.support().validate()?;

        if let Some(bound) = self.accuracy().bound() {
            if !bound.is_finite() || bound < 0.0 {
                return Err(ChannelError::InvalidErrorBound);
            }
        }

        Ok(())
    }

    /// Validates that the channel is physically usable under the current
    /// semantic contract.
    ///
    /// Concrete representations should override this method when they can
    /// establish complete positivity, trace preservation and other required
    /// conditions.
    fn validate_physicality(&self) -> ChannelResult<()> {
        match self.physicality() {
            ChannelPhysicality::Validated => Ok(()),

            ChannelPhysicality::Unvalidated => Err(ChannelError::NotValidated),

            ChannelPhysicality::NonPhysical => {
                Err(ChannelError::NotPhysical("implementation marked channel non-physical"))
            }

            ChannelPhysicality::Conditional => Err(ChannelError::NotValidated),
        }
    }

    /// Returns whether the channel is square.
    #[must_use]
    fn is_square(&self) -> bool {
        self.support().is_square()
    }

    /// Returns whether the channel has been validated as physical.
    #[must_use]
    fn is_physical(&self) -> bool {
        self.physicality().is_validated()
    }

    /// Returns whether the channel is exact.
    #[must_use]
    fn is_exact(&self) -> bool {
        self.accuracy().is_exact()
    }

    /// Returns the total input Hilbert-space dimension when representable by
    /// `usize`.
    #[must_use]
    fn input_dimension(&self) -> Option<usize> {
        self.support().input_dimension()
    }

    /// Returns the total output Hilbert-space dimension when representable by
    /// `usize`.
    #[must_use]
    fn output_dimension(&self) -> Option<usize> {
        self.support().output_dimension()
    }

    /// Returns canonical logical qubits in the input support.
    fn input_qubits(&self) -> Vec<QubitId> {
        self.support().input_qubits().collect()
    }

    /// Returns canonical logical qubits in the output support.
    fn output_qubits(&self) -> Vec<QubitId> {
        self.support().output_qubits().collect()
    }

    /// Returns whether this implementation supports a requested capability.
    ///
    /// This is derived from `required_capabilities()` only for semantic
    /// inspection and must not be used as a substitute for target capability
    /// negotiation.
    #[must_use]
    fn requires_capability(&self, capability: ChannelCapability) -> bool {
        self.required_capabilities().contains(&capability)
    }

    /// Returns a stable, representation-independent textual kind.
    ///
    /// This is deliberately not a serialization format.
    fn semantic_kind(&self) -> &'static str {
        "quantum_channel"
    }
}

// =============================================================================
// Composition validation
// =============================================================================

/// Validates whether two channels can be composed sequentially.
///
/// The function checks semantic domain compatibility without inspecting the
/// concrete mathematical representation.
pub fn validate_sequential_composition(
    first: &dyn QuantumChannel,
    second: &dyn QuantumChannel,
) -> ChannelResult<()> {
    first.validate()?;
    second.validate()?;

    let first_output = first.support().output();
    let second_input = second.support().input();

    if first_output.len() != second_input.len() {
        return Err(ChannelError::DomainMismatch {
            input_arity: first_output.len(),
            output_arity: second_input.len(),
        });
    }

    for (left, right) in first_output.iter().zip(second_input.iter()) {
        if left.dimension() != right.dimension() {
            return Err(ChannelError::IncompatibleComposition);
        }
    }

    Ok(())
}

/// Validates whether two channels can be composed as a tensor product.
///
/// Tensor-product composition permits unrelated subsystem supports but rejects
/// overlapping subsystem identities because overlap would represent correlated
/// or sequential semantics rather than an independent tensor product.
pub fn validate_tensor_product(
    first: &dyn QuantumChannel,
    second: &dyn QuantumChannel,
) -> ChannelResult<()> {
    first.validate()?;
    second.validate()?;

    validate_disjoint_support(first.support(), second.support())
}

// =============================================================================
// Support helpers
// =============================================================================

/// Validates that a support contains no duplicate subsystem identities.
fn validate_unique_support(support: &[ChannelSubsystem]) -> ChannelResult<()> {
    for (index, left) in support.iter().enumerate() {
        for right in support.iter().skip(index + 1) {
            if left.id() == right.id() {
                if let Some(qubit) = left.qubit() {
                    return Err(ChannelError::DuplicateQubit(qubit));
                }

                return Err(ChannelError::IncompatibleComposition);
            }
        }
    }

    Ok(())
}

/// Validates that two supports do not overlap.
fn validate_disjoint_support(
    first: &ChannelSupport,
    second: &ChannelSupport,
) -> ChannelResult<()> {
    for left in first
        .input()
        .iter()
        .chain(first.output().iter())
    {
        for right in second
            .input()
            .iter()
            .chain(second.output().iter())
        {
            if left.id() == right.id() {
                if let Some(qubit) = left.qubit() {
                    return Err(ChannelError::DuplicateQubit(qubit));
                }

                return Err(ChannelError::IncompatibleComposition);
            }
        }
    }

    Ok(())
}

/// Computes the product of subsystem dimensions without materializing any
/// matrix or allocating memory.
///
/// This is deliberately checked. A dimension overflow is represented as
/// `None` rather than wrapping.
fn checked_dimension_product(subsystems: &[ChannelSubsystem]) -> Option<usize> {
    subsystems
        .iter()
        .try_fold(1usize, |accumulator, subsystem| {
            accumulator.checked_mul(subsystem.dimension())
        })
}

/// Validates a non-negative finite floating-point value.
fn validate_finite_non_negative(
    value: f64,
    error: ChannelError,
) -> ChannelResult<()> {
    if !value.is_finite() {
        return Err(ChannelError::NonFiniteParameter);
    }

    if value < 0.0 {
        return Err(error);
    }

    Ok(())
}

// =============================================================================
// Public helper constructors
// =============================================================================

/// Creates a canonical qubit subsystem descriptor.
///
/// This is the preferred constructor whenever a ZQN channel operates on a
/// canonical Zamani logical qubit.
pub fn qubit_subsystem(id: QubitId) -> ChannelSubsystem {
    // Qubits are two-dimensional by definition.
    //
    // This constant describes the mathematical dimension of a qubit, not a
    // maximum number of qubits in a machine.
    ChannelSubsystem {
        id: ChannelSubsystemId::Qubit(id),
        dimension: 2,
    }
}

/// Creates a generic subsystem descriptor.
///
/// This is useful for qudits, modes, bosonic truncations and future quantum
/// technologies whose subsystem dimension is not two.
pub fn subsystem(
    id: ChannelSubsystemId,
    dimension: usize,
) -> ChannelResult<ChannelSubsystem> {
    ChannelSubsystem::new(id, dimension)
}

/// Creates a square channel support over the supplied subsystems.
pub fn square_support(
    subsystems: Vec<ChannelSubsystem>,
) -> ChannelResult<ChannelSupport> {
    ChannelSupport::square(subsystems)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestChannel {
        descriptor: ChannelDescriptor,
        capabilities: Vec<ChannelCapability>,
    }

    impl TestChannel {
        fn new(
            id: ChannelId,
            support: ChannelSupport,
            representation: ChannelRepresentation,
        ) -> Self {
            let descriptor = ChannelDescriptor::new(
                id,
                None,
                support,
                representation,
                ChannelPhysicality::Validated,
                ChannelAccuracy::Exact,
                ChannelResourceRequirements::unknown(),
            )
            .expect("test channel descriptor must be valid");

            Self {
                descriptor,
                capabilities: Vec::new(),
            }
        }
    }

    impl QuantumChannel for TestChannel {
        fn id(&self) -> ChannelId {
            self.descriptor.id
        }

        fn descriptor(&self) -> ChannelDescriptor {
            self.descriptor.clone()
        }

        fn support(&self) -> &ChannelSupport {
            &self.descriptor.support
        }

        fn representation(&self) -> ChannelRepresentation {
            self.descriptor.representation
        }

        fn physicality(&self) -> ChannelPhysicality {
            self.descriptor.physicality
        }

        fn accuracy(&self) -> ChannelAccuracy {
            self.descriptor.accuracy
        }

        fn required_capabilities(&self) -> &[ChannelCapability] {
            &self.capabilities
        }

        fn resource_requirements(&self) -> ChannelResourceRequirements {
            self.descriptor.resources
        }
    }

    #[test]
    fn qubit_subsystem_uses_canonical_qubit_id() {
        let qubit = QubitId::new(7);
        let subsystem = qubit_subsystem(qubit);

        assert_eq!(subsystem.qubit(), Some(qubit));
        assert_eq!(subsystem.dimension(), 2);
    }

    #[test]
    fn generic_subsystem_accepts_non_qubit_dimension() {
        let subsystem =
            subsystem(ChannelSubsystemId::opaque(42), 3).expect("qutrit should be valid");

        assert_eq!(subsystem.dimension(), 3);
        assert_eq!(subsystem.qubit(), None);
    }

    #[test]
    fn dimension_one_is_rejected() {
        let result = subsystem(ChannelSubsystemId::opaque(1), 1);

        assert!(matches!(
            result,
            Err(ChannelError::InvalidDimension {
                index: 0,
                dimension: 1
            })
        ));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let result = subsystem(ChannelSubsystemId::opaque(1), 0);

        assert!(matches!(
            result,
            Err(ChannelError::InvalidDimension {
                index: 0,
                dimension: 0
            })
        ));
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubit = QubitId::new(3);

        let result = ChannelSupport::square(vec![
            qubit_subsystem(qubit),
            qubit_subsystem(qubit),
        ]);

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateQubit(found)) if found == qubit
        ));
    }

    #[test]
    fn different_qubits_are_valid() {
        let support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(0)),
            qubit_subsystem(QubitId::new(1)),
        ])
        .expect("different qubits should be valid");

        assert_eq!(support.input_arity(), 2);
        assert_eq!(support.output_arity(), 2);
        assert_eq!(support.input_dimension(), Some(4));
        assert_eq!(support.output_dimension(), Some(4));
        assert!(support.is_square());
    }

    #[test]
    fn mixed_modality_support_is_supported() {
        let support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(0)),
            subsystem(ChannelSubsystemId::opaque(100), 3).expect("valid qutrit"),
        ])
        .expect("mixed support should be valid");

        assert_eq!(support.input_dimension(), Some(6));
        assert_eq!(support.output_dimension(), Some(6));
    }

    #[test]
    fn overflowing_dimension_product_is_not_wrapped() {
        let first = subsystem(ChannelSubsystemId::opaque(1), usize::MAX)
            .expect("large semantic dimension is valid");
        let second =
            subsystem(ChannelSubsystemId::opaque(2), 2).expect("valid subsystem");

        let support =
            ChannelSupport::square(vec![first, second]).expect("support is structurally valid");

        assert_eq!(support.input_dimension(), None);
        assert_eq!(support.output_dimension(), None);
    }

    #[test]
    fn exact_accuracy_has_zero_error_bound() {
        let accuracy = ChannelAccuracy::exact();

        assert!(accuracy.is_exact());
        assert_eq!(accuracy.bound(), Some(0.0));
    }

    #[test]
    fn negative_tolerance_is_rejected() {
        let result = ChannelAccuracy::approximate(-1.0);

        assert!(matches!(result, Err(ChannelError::InvalidTolerance)));
    }

    #[test]
    fn non_finite_tolerance_is_rejected() {
        let result = ChannelAccuracy::approximate(f64::NAN);

        assert!(matches!(result, Err(ChannelError::NonFiniteParameter)));
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let result = ChannelAccuracy::statistical(1.5);

        assert!(matches!(result, Err(ChannelError::InvalidTolerance)));
    }

    #[test]
    fn valid_confidence_is_accepted() {
        let result = ChannelAccuracy::statistical(0.95);

        assert!(result.is_ok());
    }

    #[test]
    fn channel_id_is_opaque_and_stable() {
        let id = ChannelId::from_u128(42);

        assert_eq!(id.as_u128(), 42);
        assert_eq!(id.to_string(), "0000000000000000000000000000002a");
    }

    #[test]
    fn sequential_composition_requires_matching_dimensions() {
        let first_support = ChannelSupport::square(vec![qubit_subsystem(QubitId::new(0))])
            .expect("valid support");

        let second_support = ChannelSupport::square(vec![qubit_subsystem(QubitId::new(0))])
            .expect("valid support");

        let first = TestChannel::new(
            ChannelId::from_u128(1),
            first_support,
            ChannelRepresentation::Kraus,
        );

        let second = TestChannel::new(
            ChannelId::from_u128(2),
            second_support,
            ChannelRepresentation::Choi,
        );

        assert!(validate_sequential_composition(&first, &second).is_ok());
    }

    #[test]
    fn sequential_composition_rejects_different_dimensions() {
        let first_support = ChannelSupport::square(vec![
            subsystem(ChannelSubsystemId::opaque(1), 2).expect("valid subsystem"),
        ])
        .expect("valid support");

        let second_support = ChannelSupport::square(vec![
            subsystem(ChannelSubsystemId::opaque(2), 3).expect("valid subsystem"),
        ])
        .expect("valid support");

        let first = TestChannel::new(
            ChannelId::from_u128(1),
            first_support,
            ChannelRepresentation::Kraus,
        );

        let second = TestChannel::new(
            ChannelId::from_u128(2),
            second_support,
            ChannelRepresentation::Choi,
        );

        assert!(matches!(
            validate_sequential_composition(&first, &second),
            Err(ChannelError::IncompatibleComposition)
        ));
    }

    #[test]
    fn tensor_product_requires_disjoint_support() {
        let first_support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(0)),
        ])
        .expect("valid support");

        let second_support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(1)),
        ])
        .expect("valid support");

        let first = TestChannel::new(
            ChannelId::from_u128(1),
            first_support,
            ChannelRepresentation::Kraus,
        );

        let second = TestChannel::new(
            ChannelId::from_u128(2),
            second_support,
            ChannelRepresentation::Kraus,
        );

        assert!(validate_tensor_product(&first, &second).is_ok());
    }

    #[test]
    fn tensor_product_rejects_overlapping_support() {
        let first_support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(0)),
        ])
        .expect("valid support");

        let second_support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(0)),
        ])
        .expect("valid support");

        let first = TestChannel::new(
            ChannelId::from_u128(1),
            first_support,
            ChannelRepresentation::Kraus,
        );

        let second = TestChannel::new(
            ChannelId::from_u128(2),
            second_support,
            ChannelRepresentation::Kraus,
        );

        assert!(matches!(
            validate_tensor_product(&first, &second),
            Err(ChannelError::DuplicateQubit(qubit))
                if qubit == QubitId::new(0)
        ));
    }

    #[test]
    fn unvalidated_channel_does_not_claim_physicality() {
        let support =
            ChannelSupport::square(vec![qubit_subsystem(QubitId::new(0))])
                .expect("valid support");

        let descriptor = ChannelDescriptor::new(
            ChannelId::from_u128(7),
            None,
            support,
            ChannelRepresentation::Kraus,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
            ChannelResourceRequirements::unknown(),
        )
        .expect("descriptor should be valid");

        let channel = TestChannel {
            descriptor,
            capabilities: Vec::new(),
        };

        assert!(!channel.is_physical());
        assert!(matches!(
            channel.validate_physicality(),
            Err(ChannelError::NotValidated)
        ));
    }

    #[test]
    fn physical_channel_can_validate() {
        let support =
            ChannelSupport::square(vec![qubit_subsystem(QubitId::new(0))])
                .expect("valid support");

        let channel = TestChannel::new(
            ChannelId::from_u128(8),
            support,
            ChannelRepresentation::Kraus,
        );

        assert!(channel.validate().is_ok());
        assert!(channel.validate_physicality().is_ok());
        assert!(channel.is_physical());
        assert!(channel.is_exact());
    }

    #[test]
    fn input_and_output_qubits_are_exposed_from_canonical_ids() {
        let support = ChannelSupport::square(vec![
            qubit_subsystem(QubitId::new(4)),
            qubit_subsystem(QubitId::new(9)),
        ])
        .expect("valid support");

        let channel = TestChannel::new(
            ChannelId::from_u128(9),
            support,
            ChannelRepresentation::Kraus,
        );

        assert_eq!(
            channel.input_qubits(),
            vec![QubitId::new(4), QubitId::new(9)]
        );

        assert_eq!(
            channel.output_qubits(),
            vec![QubitId::new(4), QubitId::new(9)]
        );
    }

    #[test]
    fn descriptor_rejects_empty_name() {
        let support =
            ChannelSupport::square(vec![qubit_subsystem(QubitId::new(0))])
                .expect("valid support");

        let result = ChannelDescriptor::new(
            ChannelId::from_u128(1),
            Some(String::from("   ")),
            support,
            ChannelRepresentation::Kraus,
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            ChannelResourceRequirements::unknown(),
        );

        assert!(matches!(result, Err(ChannelError::InvalidIdentity)));
    }

    #[test]
    fn resource_requirements_are_unbounded_by_default() {
        let requirements = ChannelResourceRequirements::unknown();

        assert_eq!(requirements.scalar_elements, None);
        assert_eq!(requirements.memory_bytes, None);
        assert_eq!(requirements.arithmetic_operations, None);
    }
}