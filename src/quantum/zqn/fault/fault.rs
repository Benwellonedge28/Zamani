//! Zamani Quantum Noise (ZQN) — Canonical Fault Semantics.
//!
//! This module defines the canonical, backend-independent representation of a
//! realized quantum fault.
//!
//! # Architectural role
//!
//! A `Fault` is an event/condition that represents an unwanted physical or
//! logical deviation associated with a quantum resource, operation,
//! measurement, preparation, reset, transport event, time interval, or other
//! ZQN-defined location.
//!
//! This module answers:
//!
//! > "What fault occurred, where did it occur, and what semantic effect does
//! > it represent?"
//!
//! It owns:
//!
//! - the immutable `Fault` value;
//! - fault classification;
//! - fault location vocabulary;
//! - fault effect vocabulary;
//! - optional operation association;
//! - optional temporal interval;
//! - optional probability/weight annotation where explicitly meaningful;
//! - fault identity;
//! - deterministic canonical ordering;
//! - validation of the structural invariants of an individual fault;
//! - conversion-independent inspection APIs.
//!
//! It does NOT own:
//!
//! - canonical quantum program semantics;
//! - `QubitId` or `PhysicalQubitId` definitions;
//! - quantum channels;
//! - probability distributions;
//! - noise-model generation;
//! - random-number generation;
//! - calibration;
//! - characterization;
//! - syndrome decoding;
//! - logical correction;
//! - routing;
//! - scheduling policy;
//! - hardware APIs;
//! - QPU credentials;
//! - backend execution;
//! - benchmarking;
//! - serialization formats.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical quantum-resource identity
//!
//! ZQN MUST use the canonical Quantum IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module therefore imports those types directly.
//!
//! It MUST NOT define:
//!
//! ```text
//! struct QubitId(...);
//! struct PhysicalQubitId(...);
//! ```
//!
//! A numerical equality between a logical and physical identifier does not
//! make them semantically interchangeable.
//!
//! The repository's canonical qubit module explicitly establishes
//! `QubitId` as the logical identity and `PhysicalQubitId` as the physical
//! identity. 
//!
//! # ZQN object identity
//!
//! Fault identity is different from quantum-resource identity.
//!
//! `FaultId` identifies a ZQN fault object.
//!
//! It does NOT identify:
//!
//! - a qubit;
//! - an array position;
//! - a hardware address;
//! - a memory address;
//! - an execution slot;
//! - a fault count.
//!
//! `FaultId` is supplied by the caller/owning generator/registry according to
//! its identity policy. This module does not maintain a global ID allocator.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_CORRELATED_QUBITS
//! MAX_OPERATIONS
//! ```
//!
//! in this module.
//!
//! A `Fault` can describe one resource or a composite resource set of any
//! size representable by the selected storage representation and permitted by
//! the caller's explicit resource policy.
//!
//! "Infinity" in Zamani means that the semantic model does not encode an
//! artificial finite machine-size ceiling. Actual execution remains bounded
//! by available memory, storage, CPU/GPU resources, distributed capacity,
//! target capabilities, and explicit resource policies.
//!
//! # Important distinction: fault versus noise
//!
//! ```text
//! NoiseModel
//!     describes a process/law that may generate deviations.
//!
//! Fault
//!     represents a particular realized deviation/event.
//!
//! QuantumChannel
//!     describes a physical transformation/channel.
//! ```
//!
//! A fault therefore MUST NOT become a replacement for the channel model.
//!
//! A stochastic noise model may produce zero, one, or many faults.
//!
//! A deterministic execution may directly construct a fault without using
//! random sampling.
//!
//! # Important distinction: fault versus error-correction semantics
//!
//! ZQN owns physical/abstract fault semantics.
//!
//! QEC owns:
//!
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - logical-fault analysis;
//! - code-specific semantics.
//!
//! A QEC adapter may consume `Fault`, but `Fault` must remain independent of
//! the decoder and code implementation.
//!
//! The existing QEC noise subsystem already documents that syndrome decoding,
//! logical correction, backend execution, scheduling, and related concerns
//! are outside its physical-noise ownership. ZQN provides the reusable
//! cross-subsystem representation rather than duplicating QEC-specific
//! semantics. 
//!
//! # Fault identity versus semantic equality
//!
//! Two different `FaultId`s may describe semantically equivalent faults.
//!
//! Conversely, two faults with the same location and effect may have
//! different identities because they came from different executions,
//! experiments, calibration snapshots, or provenance domains.
//!
//! Therefore:
//!
//! ```text
//! PartialEq/Eq
//!     identity + complete stored value equality
//!
//! semantic comparison
//!     explicit caller-level comparison
//! ```
//!
//! This module does not silently equate fault identity with physical
//! equivalence.
//!
//! # Determinism
//!
//! `Fault` contains no hidden randomness.
//!
//! It does not:
//!
//! - call a global RNG;
//! - access thread-local RNG state;
//! - use system time implicitly;
//! - inspect memory addresses;
//! - depend on hash-map iteration order;
//! - allocate global state.
//!
//! Fault generation belongs to the noise/sampling layer.
//!
//! Once constructed, a `Fault` is deterministic and immutable.
//!
//! # Canonical ordering
//!
//! Faults implement `Ord` so deterministic consumers can canonicalize and
//! compare collections without depending on hash-map iteration order.
//!
//! Ordering is an implementation-level deterministic ordering only.
//!
//! Consumers MUST NOT interpret fault ordering as:
//!
//! - temporal ordering;
//! - hardware topology ordering;
//! - execution priority;
//! - severity ordering.
//!
//! Temporal order is represented explicitly by `FaultTiming`.
//!
//! # Resource safety
//!
//! This module does not allocate implicitly during ordinary inspection.
//!
//! Composite locations use caller-owned collections and do not recursively
//! allocate hidden structures.
//!
//! Validation is proportional to the explicitly supplied fault structure.
//!
//! Callers processing untrusted streams must apply their own
//! `ZqnLimits`/execution resource policy before materializing arbitrarily
//! large collections.
//!
//! This file does not encode such policy because semantic validation and
//! resource policy are separate responsibilities.
//!
//! # Numerical safety
//!
//! No floating-point probability is stored directly in `Fault`.
//!
//! Where a probabilistic weight is needed, use the canonical ZQN probability
//! abstraction from `crate::quantum::zqn::probability` at the integration
//! boundary.
//!
//! This avoids silently introducing a second probability representation.
//!
//! # Serialization
//!
//! This module intentionally does not depend on serde or any serialization
//! framework.
//!
//! `Fault` is a semantic value. The future ZQN `io` subsystem owns:
//!
//! - schema;
//! - encoding;
//! - canonical serialization;
//! - compatibility;
//! - migration.
//!
//! A serialized fault MUST preserve:
//!
//! - fault identity;
//! - classification;
//! - location identity/domain;
//! - effect;
//! - operation association;
//! - timing;
//! - optional annotations/weight;
//! - schema/version context supplied by the IO layer.
//!
//! Serialization MUST NOT collapse logical and physical qubit identity into
//! an untyped integer.
//!
//! # Thread safety
//!
//! `Fault` is immutable after construction.
//!
//! The types in this file contain no interior mutability or global state.
//!
//! They are therefore suitable for concurrent use when placed inside
//! thread-safe containers and execution contexts.
//!
//! # Security
//!
//! Fault values are data, not capabilities.
//!
//! Possessing a `FaultId`, `QubitId`, or `PhysicalQubitId` MUST NOT grant:
//!
//! - QPU access;
//! - hardware control;
//! - credentials;
//! - calibration write access;
//! - execution authorization.
//!
//! Authorization belongs to the surrounding capability/security subsystem.
//!
//! Untrusted fault streams MUST be processed under explicit resource limits.
//!
//! # Rust compatibility
//!
//! This file targets:
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
//! quantum::ir::qubit
//!          │
//!          ├── QubitId
//!          └── PhysicalQubitId
//!
//! zqn::core::ids
//!          │
//!          └── FaultId
//!
//!          ▼
//! zqn::fault::fault
//!          │
//!          ├── Fault
//!          ├── FaultLocation
//!          ├── FaultClassification
//!          ├── FaultEffect
//!          └── FaultTiming
//!          │
//!          ├──────────────┬───────────────┬──────────────┐
//!          ▼              ▼               ▼              ▼
//!       noise          channel          QEC          simulation
//!          │                              │              │
//!          └──────────────────────────────┼──────────────┘
//!                                         ▼
//!                                      runtime
//! ```
//!
//! Downstream modules should consume this contract rather than redefine a
//! competing fault representation.
//!
//! # Future integration files
//!
//! `fault/location.rs` should provide additional location-oriented helpers
//! and re-export the canonical `FaultLocation` type when appropriate.
//!
//! `fault/classification.rs` should provide classification predicates,
//! grouping and analysis helpers without changing the fundamental
//! classification values defined here.
//!
//! `fault/correlated.rs` should provide scalable correlated-fault builders
//! and generators using `Fault`/`FaultLocation` rather than defining a
//! competing fault object.
//!
//! `fault/leakage.rs`, `fault/erasure.rs`, and `fault/loss.rs` should provide
//! specialized constructors/validation for their respective effect kinds.
//!
//! `noise/*` should generate/realize faults.
//!
//! `channel/*` should represent channels, not individual fault events.
//!
//! `integration/qec.rs` should adapt faults into QEC-specific representations.
//!
//! `integration/ir.rs` should associate faults with canonical IR operations
//! without making the IR depend on ZQN.
//!
//! `integration/routing.rs` and `integration/scheduling.rs` may consume
//! fault/error cost information but must not redefine fault semantics.
//!
//! `io/*` owns serialization.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. `Fault` is immutable;
//! 2. logical and physical qubit identities use canonical IR types;
//! 3. no machine-size ceiling is encoded;
//! 4. no global RNG/state exists;
//! 5. structural validation is deterministic;
//! 6. standard fault categories are represented;
//! 7. future fault categories can be represented without changing the
//!    identity model;
//! 8. downstream systems can consume the type without depending on concrete
//!    generators;
//! 9. the module compiles without unsafe code;
//! 10. the module requires no later modification merely because specialized
//!     fault helpers are added.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::{FaultId, ZqnIdValue};

// ============================================================================
// Fault domain
// ============================================================================

/// Semantic identity domain of a fault location.
///
/// The domain is explicit so that a logical resource can never be silently
/// interpreted as a physical resource merely because both use an integer-like
/// identifier internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultResourceDomain {
    /// Logical Quantum IR resource.
    Logical,

    /// Physical Quantum IR resource.
    Physical,

    /// ZQN-defined non-qubit resource.
    Zqn,

    /// External/technology-defined resource whose identity is carried by a
    /// ZQN object identifier.
    External,
}

impl fmt::Display for FaultResourceDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical => formatter.write_str("logical"),
            Self::Physical => formatter.write_str("physical"),
            Self::Zqn => formatter.write_str("zqn"),
            Self::External => formatter.write_str("external"),
        }
    }
}

// ============================================================================
// Fault location
// ============================================================================

/// Location at which a fault applies.
///
/// The location model is deliberately broader than "qubit".
///
/// This permits ZQN to represent faults associated with:
///
/// - logical qubits;
/// - physical qubits;
/// - operations;
/// - measurements;
/// - preparations;
/// - reset;
/// - transport;
/// - links;
/// - composite resources;
/// - future quantum technologies.
///
/// No fixed arity is assumed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultLocation {
    /// A logical qubit from the canonical Quantum IR.
    LogicalQubit(QubitId),

    /// A physical qubit from the canonical Quantum IR.
    PhysicalQubit(PhysicalQubitId),

    /// A ZQN-owned object/resource.
    ZqnResource(ZqnIdValue),

    /// An external/technology-specific resource represented by an opaque
    /// ZQN-domain value.
    ExternalResource(ZqnIdValue),

    /// A quantum operation identified by a ZQN operation/object identity.
    ///
    /// The actual canonical operation type remains owned by the IR.
    Operation(FaultOperationId),

    /// A measurement resource.
    Measurement(FaultOperationId),

    /// A preparation resource.
    Preparation(FaultOperationId),

    /// A reset resource.
    Reset(FaultOperationId),

    /// A transport/communication resource.
    Transport(FaultOperationId),

    /// A composite location containing multiple independently typed
    /// resources.
    ///
    /// The vector is caller-owned and is therefore naturally bounded by
    /// available resources. There is no semantic maximum encoded here.
    Composite(Vec<FaultLocation>),
}

impl FaultLocation {
    /// Creates a logical-qubit location using the canonical IR identity.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit location using the canonical IR identity.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a ZQN-resource location.
    #[must_use]
    pub const fn zqn_resource(id: ZqnIdValue) -> Self {
        Self::ZqnResource(id)
    }

    /// Creates an external-resource location.
    #[must_use]
    pub const fn external_resource(id: ZqnIdValue) -> Self {
        Self::ExternalResource(id)
    }

    /// Creates an operation location.
    #[must_use]
    pub const fn operation(id: FaultOperationId) -> Self {
        Self::Operation(id)
    }

    /// Creates a measurement location.
    #[must_use]
    pub const fn measurement(id: FaultOperationId) -> Self {
        Self::Measurement(id)
    }

    /// Creates a preparation location.
    #[must_use]
    pub const fn preparation(id: FaultOperationId) -> Self {
        Self::Preparation(id)
    }

    /// Creates a reset location.
    #[must_use]
    pub const fn reset(id: FaultOperationId) -> Self {
        Self::Reset(id)
    }

    /// Creates a transport location.
    #[must_use]
    pub const fn transport(id: FaultOperationId) -> Self {
        Self::Transport(id)
    }

    /// Creates a composite location.
    ///
    /// Validation rejects an empty composite because an empty location does
    /// not identify where a fault occurred.
    pub fn composite(locations: Vec<FaultLocation>) -> ZqnResult<Self> {
        if locations.is_empty() {
            return Err(
                ZqnError::invalid_fault_location(
                    "composite fault location cannot be empty",
                ),
            );
        }

        let location = Self::Composite(locations);

        location.validate()?;

        Ok(location)
    }

    /// Returns the logical qubit when this is directly a logical-qubit
    /// location.
    #[must_use]
    pub const fn logical_qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the physical qubit when this is directly a physical-qubit
    /// location.
    #[must_use]
    pub const fn physical_qubit_id(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the resource domain of this location.
    #[must_use]
    pub const fn resource_domain(&self) -> FaultResourceDomain {
        match self {
            Self::LogicalQubit(_) => FaultResourceDomain::Logical,
            Self::PhysicalQubit(_) => FaultResourceDomain::Physical,
            Self::ZqnResource(_) => FaultResourceDomain::Zqn,
            Self::ExternalResource(_) => FaultResourceDomain::External,
            Self::Operation(_)
            | Self::Measurement(_)
            | Self::Preparation(_)
            | Self::Reset(_)
            | Self::Transport(_) => FaultResourceDomain::Zqn,
            Self::Composite(_) => FaultResourceDomain::Zqn,
        }
    }

    /// Returns whether this location contains multiple child locations.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        matches!(self, Self::Composite(_))
    }

    /// Validates the structural invariants of this location.
    pub fn validate(&self) -> ZqnResult<()> {
        match self {
            Self::Composite(locations) => {
                if locations.is_empty() {
                    return Err(
                        ZqnError::invalid_fault_location(
                            "composite fault location cannot be empty",
                        ),
                    );
                }

                for location in locations {
                    location.validate()?;
                }

                // Duplicate exact locations are semantically ambiguous for
                // one Fault. A correlated fault should represent one
                // occurrence per resource unless a higher-level model
                // explicitly represents multiplicity.
                for index in 0..locations.len() {
                    for other in (index + 1)..locations.len() {
                        if locations[index] == locations[other] {
                            return Err(
                                ZqnError::invalid_fault_location(
                                    "composite fault location contains duplicate resources",
                                ),
                            );
                        }
                    }
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}

impl fmt::Display for FaultLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "logical:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical:{id}"),
            Self::ZqnResource(id) => write!(formatter, "zqn:{id}"),
            Self::ExternalResource(id) => write!(formatter, "external:{id}"),
            Self::Operation(id) => write!(formatter, "operation:{id}"),
            Self::Measurement(id) => write!(formatter, "measurement:{id}"),
            Self::Preparation(id) => write!(formatter, "preparation:{id}"),
            Self::Reset(id) => write!(formatter, "reset:{id}"),
            Self::Transport(id) => write!(formatter, "transport:{id}"),
            Self::Composite(locations) => {
                formatter.write_str("composite:[")?;

                for (index, location) in locations.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(",")?;
                    }

                    write!(formatter, "{location}")?;
                }

                formatter.write_str("]")
            }
        }
    }
}

// ============================================================================
// Operation identity
// ============================================================================

/// Identity used when associating a fault with an operation-like object.
///
/// This is intentionally separate from the canonical IR operation value.
///
/// The operation's semantic definition remains owned by Quantum IR.
///
/// This identifier merely provides a stable cross-layer association key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FaultOperationId(ZqnIdValue);

impl FaultOperationId {
    /// Creates an operation association identifier.
    ///
    /// This does not assert that the operation exists.
    #[must_use]
    pub const fn new(value: ZqnIdValue) -> Self {
        Self(value)
    }

    /// Returns the underlying association value.
    #[must_use]
    pub const fn value(self) -> ZqnIdValue {
        self.0
    }
}

impl From<ZqnIdValue> for FaultOperationId {
    fn from(value: ZqnIdValue) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for FaultOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "op:{}", self.0)
    }
}

// ============================================================================
// Fault classification
// ============================================================================

/// Broad semantic classification of a fault.
///
/// These are semantic categories rather than hardware-vendor categories.
///
/// `Custom` provides forward compatibility for future quantum technologies
/// and fault classes without requiring the core `Fault` structure to change.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultClassification {
    /// Preparation/state-initialization fault.
    Preparation,

    /// Gate/operation fault.
    Gate,

    /// Reset fault.
    Reset,

    /// Measurement/readout fault.
    Measurement,

    /// Idle/decoherence fault.
    Idle,

    /// Leakage out of the intended computational/subspace model.
    Leakage,

    /// Erasure event.
    Erasure,

    /// Loss event.
    Loss,

    /// Transport/motion/communication fault.
    Transport,

    /// Crosstalk/interference fault.
    Crosstalk,

    /// Correlated fault involving multiple resources.
    Correlated,

    /// Coherent/control fault.
    Coherent,

    /// Timing-related fault.
    Timing,

    /// Calibration/drift-related fault.
    Calibration,

    /// Fault associated with a logical operation/resource.
    Logical,

    /// Fault whose semantics are explicitly user/technology defined.
    Custom(String),
}

impl FaultClassification {
    /// Returns whether the classification represents a multi-resource
    /// correlation.
    #[must_use]
    pub const fn is_correlated(&self) -> bool {
        matches!(self, Self::Correlated)
    }

    /// Returns whether the classification represents a loss/erasure-like
    /// event.
    #[must_use]
    pub const fn is_loss_like(&self) -> bool {
        matches!(self, Self::Erasure | Self::Loss)
    }

    /// Returns whether the classification is a standard built-in category.
    #[must_use]
    pub const fn is_standard(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }

    /// Returns a stable semantic name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Preparation => "preparation",
            Self::Gate => "gate",
            Self::Reset => "reset",
            Self::Measurement => "measurement",
            Self::Idle => "idle",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
            Self::Loss => "loss",
            Self::Transport => "transport",
            Self::Crosstalk => "crosstalk",
            Self::Correlated => "correlated",
            Self::Coherent => "coherent",
            Self::Timing => "timing",
            Self::Calibration => "calibration",
            Self::Logical => "logical",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Validates the classification.
    pub fn validate(&self) -> ZqnResult<()> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(
                    ZqnError::invalid_fault_classification(
                        "custom fault classification cannot be empty",
                    ),
                );
            }
        }

        Ok(())
    }
}

impl fmt::Display for FaultClassification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Pauli effect
// ============================================================================

/// Single-qubit Pauli effect represented as a realized fault.
///
/// `Identity` is intentionally not considered a fault effect.
///
/// Identity may be useful inside channel representations, but a realized
/// physical fault must describe an actual deviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PauliEffect {
    /// Bit-flip effect.
    X,

    /// Bit-and-phase-flip effect.
    Y,

    /// Phase-flip effect.
    Z,
}

impl fmt::Display for PauliEffect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => formatter.write_str("X"),
            Self::Y => formatter.write_str("Y"),
            Self::Z => formatter.write_str("Z"),
        }
    }
}

// ============================================================================
// Fault effect
// ============================================================================

/// Semantic effect represented by a realized fault.
///
/// This type intentionally contains standard physical effects while retaining
/// an extensible `Custom` variant for future technologies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultEffect {
    /// A single-resource Pauli effect.
    Pauli(PauliEffect),

    /// A coherent control/operation deviation.
    ///
    /// The parameter is an opaque semantic value rather than a hardware
    /// parameter schema. Domain-specific parameterization belongs to the
    /// channel/calibration subsystem.
    Coherent(ZqnIdValue),

    /// Leakage into a state/subspace outside the intended model.
    Leakage {
        /// Opaque leakage destination/state class.
        destination: ZqnIdValue,
    },

    /// Erasure of the resource.
    Erasure,

    /// Loss of the resource/excitation.
    Loss,

    /// Readout/assignment fault.
    Readout {
        /// Observed/assigned value.
        assigned_value: bool,
    },

    /// Timing deviation.
    Timing {
        /// Signed duration deviation in abstract time units.
        ///
        /// The unit is defined by the scheduling/runtime integration layer.
        delta: i128,
    },

    /// Generic state/resource corruption.
    Corruption(ZqnIdValue),

    /// User/technology-specific fault effect.
    Custom(String),
}

impl FaultEffect {
    /// Returns whether this effect is an actual fault rather than identity.
    #[must_use]
    pub fn is_nontrivial(&self) -> bool {
        true
    }

    /// Returns whether this effect represents loss of the resource.
    #[must_use]
    pub const fn is_loss(&self) -> bool {
        matches!(self, Self::Loss | Self::Erasure)
    }

    /// Returns whether this effect represents leakage.
    #[must_use]
    pub const fn is_leakage(&self) -> bool {
        matches!(self, Self::Leakage { .. })
    }

    /// Returns a stable semantic category.
    #[must_use]
    pub fn category(&self) -> &'static str {
        match self {
            Self::Pauli(_) => "pauli",
            Self::Coherent(_) => "coherent",
            Self::Leakage { .. } => "leakage",
            Self::Erasure => "erasure",
            Self::Loss => "loss",
            Self::Readout { .. } => "readout",
            Self::Timing { .. } => "timing",
            Self::Corruption(_) => "corruption",
            Self::Custom(_) => "custom",
        }
    }

    /// Validates the effect.
    pub fn validate(&self) -> ZqnResult<()> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(
                    ZqnError::invalid_fault(
                        "custom fault effect cannot be empty",
                    ),
                );
            }
        }

        Ok(())
    }
}

impl fmt::Display for FaultEffect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pauli(effect) => write!(formatter, "pauli:{effect}"),
            Self::Coherent(id) => write!(formatter, "coherent:{id}"),
            Self::Leakage { destination } => {
                write!(formatter, "leakage:{destination}")
            }
            Self::Erasure => formatter.write_str("erasure"),
            Self::Loss => formatter.write_str("loss"),
            Self::Readout { assigned_value } => {
                write!(formatter, "readout:{assigned_value}")
            }
            Self::Timing { delta } => write!(formatter, "timing:{delta}"),
            Self::Corruption(id) => write!(formatter, "corruption:{id}"),
            Self::Custom(value) => write!(formatter, "custom:{value}"),
        }
    }
}

// ============================================================================
// Fault timing
// ============================================================================

/// Explicit timing information associated with a fault.
///
/// The representation uses signed integer abstract time units rather than
/// floating-point wall-clock values.
///
/// The concrete time unit is owned by the scheduling/runtime integration
/// layer.
///
/// `Instant` is an absolute execution-relative timestamp.
///
/// `Interval` represents `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultTiming {
    /// No explicit timing information is attached.
    Unspecified,

    /// Fault occurred at an execution-relative instant.
    Instant(i128),

    /// Fault occupies an execution-relative interval `[start, end)`.
    Interval {
        /// Start time.
        start: i128,

        /// End time, exclusive.
        end: i128,
    },
}

impl FaultTiming {
    /// Creates an instantaneous fault time.
    #[must_use]
    pub const fn instant(time: i128) -> Self {
        Self::Instant(time)
    }

    /// Creates an interval.
    ///
    /// Returns an error if `end < start`.
    pub const fn interval(
        start: i128,
        end: i128,
    ) -> ZqnResult<Self> {
        if end < start {
            return Err(
                ZqnError::invalid_fault(
                    "fault timing interval end precedes start",
                ),
            );
        }

        Ok(Self::Interval { start, end })
    }

    /// Returns whether timing information is present.
    #[must_use]
    pub const fn is_specified(self) -> bool {
        !matches!(self, Self::Unspecified)
    }

    /// Validates timing invariants.
    pub const fn validate(self) -> ZqnResult<()> {
        match self {
            Self::Unspecified | Self::Instant(_) => Ok(()),

            Self::Interval { start, end } => {
                if end < start {
                    return Err(
                        ZqnError::invalid_fault(
                            "fault timing interval end precedes start",
                        ),
                    );
                }

                Ok(())
            }
        }
    }
}

impl Default for FaultTiming {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl fmt::Display for FaultTiming {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => formatter.write_str("unspecified"),
            Self::Instant(time) => write!(formatter, "@{time}"),
            Self::Interval { start, end } => {
                write!(formatter, "[{start},{end})")
            }
        }
    }
}

// ============================================================================
// Fault metadata/annotation
// ============================================================================

/// Optional compact semantic annotation attached to a fault.
///
/// The annotation is intentionally a simple key/value pair rather than a
/// dependency on the global metadata subsystem.
///
/// The surrounding metadata/provenance layer may convert this representation
/// into richer metadata without changing `Fault`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FaultAnnotation {
    key: String,
    value: String,
}

impl FaultAnnotation {
    /// Creates a validated annotation.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZqnResult<Self> {
        let key = key.into();
        let value = value.into();

        if key.trim().is_empty() {
            return Err(
                ZqnError::invalid_fault(
                    "fault annotation key cannot be empty",
                ),
            );
        }

        Ok(Self { key, value })
    }

    /// Returns the annotation key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the annotation value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// Fault
// ============================================================================

/// Immutable canonical realized quantum fault.
///
/// A `Fault` represents one semantic fault event/condition.
///
/// It contains no hidden randomness and no hardware handles.
///
/// # Construction
///
/// Prefer [`Fault::new`] and then the explicit builder methods.
///
/// Every constructor validates the resulting object before returning it.
///
/// # Identity
///
/// `FaultId` identifies this ZQN fault object.
///
/// It is not the same thing as the fault's semantic contents.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fault {
    id: FaultId,
    classification: FaultClassification,
    location: FaultLocation,
    effect: FaultEffect,
    timing: FaultTiming,
    operation: Option<FaultOperationId>,
    annotation: Option<FaultAnnotation>,
}

impl Fault {
    /// Creates a canonical fault.
    ///
    /// The supplied `FaultId` is an identity chosen by the owning subsystem.
    ///
    /// No global ID generation occurs.
    pub fn new(
        id: FaultId,
        classification: FaultClassification,
        location: FaultLocation,
        effect: FaultEffect,
    ) -> ZqnResult<Self> {
        let fault = Self {
            id,
            classification,
            location,
            effect,
            timing: FaultTiming::Unspecified,
            operation: None,
            annotation: None,
        };

        fault.validate()?;

        Ok(fault)
    }

    /// Creates a fault with explicit timing.
    pub fn with_timing(
        id: FaultId,
        classification: FaultClassification,
        location: FaultLocation,
        effect: FaultEffect,
        timing: FaultTiming,
    ) -> ZqnResult<Self> {
        let fault = Self {
            id,
            classification,
            location,
            effect,
            timing,
            operation: None,
            annotation: None,
        };

        fault.validate()?;

        Ok(fault)
    }

    /// Creates a fault associated with an operation.
    pub fn with_operation(
        id: FaultId,
        classification: FaultClassification,
        location: FaultLocation,
        effect: FaultEffect,
        operation: FaultOperationId,
    ) -> ZqnResult<Self> {
        let fault = Self {
            id,
            classification,
            location,
            effect,
            timing: FaultTiming::Unspecified,
            operation: Some(operation),
            annotation: None,
        };

        fault.validate()?;

        Ok(fault)
    }

    /// Creates a fully specified fault.
    pub fn with_details(
        id: FaultId,
        classification: FaultClassification,
        location: FaultLocation,
        effect: FaultEffect,
        timing: FaultTiming,
        operation: Option<FaultOperationId>,
        annotation: Option<FaultAnnotation>,
    ) -> ZqnResult<Self> {
        let fault = Self {
            id,
            classification,
            location,
            effect,
            timing,
            operation,
            annotation,
        };

        fault.validate()?;

        Ok(fault)
    }

    /// Returns the fault identity.
    #[must_use]
    pub const fn id(&self) -> FaultId {
        self.id
    }

    /// Returns the fault classification.
    #[must_use]
    pub fn classification(&self) -> &FaultClassification {
        &self.classification
    }

    /// Returns the fault location.
    #[must_use]
    pub fn location(&self) -> &FaultLocation {
        &self.location
    }

    /// Returns the fault effect.
    #[must_use]
    pub fn effect(&self) -> &FaultEffect {
        &self.effect
    }

    /// Returns fault timing.
    #[must_use]
    pub const fn timing(&self) -> FaultTiming {
        self.timing
    }

    /// Returns the associated operation, if any.
    #[must_use]
    pub const fn operation(&self) -> Option<FaultOperationId> {
        self.operation
    }

    /// Returns the optional annotation.
    #[must_use]
    pub fn annotation(&self) -> Option<&FaultAnnotation> {
        self.annotation.as_ref()
    }

    /// Returns the canonical logical qubit if this is a direct logical-qubit
    /// fault.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self.location {
            FaultLocation::LogicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the canonical physical qubit if this is a direct
    /// physical-qubit fault.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self.location {
            FaultLocation::PhysicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns whether this fault is associated with a logical resource.
    #[must_use]
    pub const fn is_logical(&self) -> bool {
        matches!(self.location, FaultLocation::LogicalQubit(_))
    }

    /// Returns whether this fault is associated with a physical resource.
    #[must_use]
    pub const fn is_physical(&self) -> bool {
        matches!(self.location, FaultLocation::PhysicalQubit(_))
    }

    /// Returns whether this fault is a composite/multi-resource fault.
    #[must_use]
    pub fn is_composite(&self) -> bool {
        self.location.is_composite()
    }

    /// Returns whether this fault is correlated according to its
    /// classification or composite location.
    #[must_use]
    pub fn is_correlated(&self) -> bool {
        self.classification.is_correlated()
            || self.location.is_composite()
    }

    /// Returns whether this fault represents leakage.
    #[must_use]
    pub fn is_leakage(&self) -> bool {
        self.classification == FaultClassification::Leakage
            || self.effect.is_leakage()
    }

    /// Returns whether this fault represents loss/erasure.
    #[must_use]
    pub fn is_loss_like(&self) -> bool {
        self.classification.is_loss_like()
            || self.effect.is_loss()
    }

    /// Returns whether this fault has explicit timing.
    #[must_use]
    pub const fn has_timing(&self) -> bool {
        self.timing.is_specified()
    }

    /// Returns a copy with a new timing value.
    ///
    /// The original fault remains unchanged.
    pub fn at_time(&self, timing: FaultTiming) -> ZqnResult<Self> {
        Self::with_details(
            self.id,
            self.classification.clone(),
            self.location.clone(),
            self.effect.clone(),
            timing,
            self.operation,
            self.annotation.clone(),
        )
    }

    /// Returns a copy associated with an operation.
    ///
    /// The original fault remains unchanged.
    pub fn associated_with(
        &self,
        operation: FaultOperationId,
    ) -> ZqnResult<Self> {
        Self::with_details(
            self.id,
            self.classification.clone(),
            self.location.clone(),
            self.effect.clone(),
            self.timing,
            Some(operation),
            self.annotation.clone(),
        )
    }

    /// Returns a copy with an annotation.
    ///
    /// The original fault remains unchanged.
    pub fn annotated(
        &self,
        annotation: FaultAnnotation,
    ) -> ZqnResult<Self> {
        Self::with_details(
            self.id,
            self.classification.clone(),
            self.location.clone(),
            self.effect.clone(),
            self.timing,
            self.operation,
            Some(annotation),
        )
    }

    /// Validates all structural invariants.
    pub fn validate(&self) -> ZqnResult<()> {
        self.classification.validate()?;
        self.location.validate()?;
        self.effect.validate()?;
        self.timing.validate()?;

        Self::validate_classification_effect(
            &self.classification,
            &self.effect,
        )?;

        Self::validate_classification_location(
            &self.classification,
            &self.location,
        )?;

        Ok(())
    }

    fn validate_classification_effect(
        classification: &FaultClassification,
        effect: &FaultEffect,
    ) -> ZqnResult<()> {
        let valid = match classification {
            FaultClassification::Preparation => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Gate => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Timing { .. }
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Reset => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Measurement => matches!(
                effect,
                FaultEffect::Readout { .. }
                    | FaultEffect::Pauli(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Idle => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Timing { .. }
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Leakage => {
                matches!(
                    effect,
                    FaultEffect::Leakage { .. }
                        | FaultEffect::Custom(_)
                )
            }

            FaultClassification::Erasure => {
                matches!(
                    effect,
                    FaultEffect::Erasure | FaultEffect::Custom(_)
                )
            }

            FaultClassification::Loss => {
                matches!(
                    effect,
                    FaultEffect::Loss | FaultEffect::Custom(_)
                )
            }

            FaultClassification::Transport => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Loss
                    | FaultEffect::Erasure
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Crosstalk => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Correlated => true,

            FaultClassification::Coherent => matches!(
                effect,
                FaultEffect::Coherent(_)
                    | FaultEffect::Pauli(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Timing => {
                matches!(
                    effect,
                    FaultEffect::Timing { .. }
                        | FaultEffect::Custom(_)
                )
            }

            FaultClassification::Calibration => matches!(
                effect,
                FaultEffect::Coherent(_)
                    | FaultEffect::Timing { .. }
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Logical => matches!(
                effect,
                FaultEffect::Pauli(_)
                    | FaultEffect::Coherent(_)
                    | FaultEffect::Corruption(_)
                    | FaultEffect::Custom(_)
            ),

            FaultClassification::Custom(_) => true,
        };

        if !valid {
            return Err(
                ZqnError::invalid_fault(
                    "fault classification is incompatible with its effect",
                ),
            );
        }

        Ok(())
    }

    fn validate_classification_location(
        classification: &FaultClassification,
        location: &FaultLocation,
    ) -> ZqnResult<()> {
        match classification {
            FaultClassification::Correlated => {
                if !location.is_composite() {
                    return Err(
                        ZqnError::invalid_correlated_fault(
                            "correlated fault requires a composite location",
                        ),
                    );
                }
            }

            FaultClassification::Crosstalk => {
                if !location.is_composite() {
                    return Err(
                        ZqnError::invalid_fault_location(
                            "crosstalk fault requires a composite location",
                        ),
                    );
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Returns a stable human-readable description.
    ///
    /// This is deliberately not a serialization protocol.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut result = format!(
            "fault:{}:{}:{}:{}",
            self.id,
            self.classification,
            self.location,
            self.effect
        );

        if self.timing.is_specified() {
            result.push_str(":time=");
            result.push_str(&self.timing.to_string());
        }

        if let Some(operation) = self.operation {
            result.push_str(":operation=");
            result.push_str(&operation.to_string());
        }

        if let Some(annotation) = &self.annotation {
            result.push_str(":");
            result.push_str(annotation.key());
            result.push('=');
            result.push_str(annotation.value());
        }

        result
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.describe())
    }
}

// ============================================================================
// Fault construction helpers
// ============================================================================

impl Fault {
    /// Constructs a logical-qubit Pauli fault.
    pub fn logical_pauli(
        id: FaultId,
        classification: FaultClassification,
        qubit: QubitId,
        effect: PauliEffect,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            classification,
            FaultLocation::LogicalQubit(qubit),
            FaultEffect::Pauli(effect),
        )
    }

    /// Constructs a physical-qubit Pauli fault.
    pub fn physical_pauli(
        id: FaultId,
        classification: FaultClassification,
        qubit: PhysicalQubitId,
        effect: PauliEffect,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            classification,
            FaultLocation::PhysicalQubit(qubit),
            FaultEffect::Pauli(effect),
        )
    }

    /// Constructs a leakage fault.
    pub fn leakage(
        id: FaultId,
        location: FaultLocation,
        destination: ZqnIdValue,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Leakage,
            location,
            FaultEffect::Leakage { destination },
        )
    }

    /// Constructs an erasure fault.
    pub fn erasure(
        id: FaultId,
        location: FaultLocation,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Erasure,
            location,
            FaultEffect::Erasure,
        )
    }

    /// Constructs a loss fault.
    pub fn loss(
        id: FaultId,
        location: FaultLocation,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Loss,
            location,
            FaultEffect::Loss,
        )
    }

    /// Constructs a readout fault.
    pub fn readout(
        id: FaultId,
        location: FaultLocation,
        assigned_value: bool,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Measurement,
            location,
            FaultEffect::Readout { assigned_value },
        )
    }

    /// Constructs a timing fault.
    pub fn timing(
        id: FaultId,
        location: FaultLocation,
        delta: i128,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Timing,
            location,
            FaultEffect::Timing { delta },
        )
    }

    /// Constructs a correlated fault.
    pub fn correlated(
        id: FaultId,
        locations: Vec<FaultLocation>,
        effect: FaultEffect,
    ) -> ZqnResult<Self> {
        let location = FaultLocation::composite(locations)?;

        Self::new(
            id,
            FaultClassification::Correlated,
            location,
            effect,
        )
    }

    /// Constructs a crosstalk fault.
    pub fn crosstalk(
        id: FaultId,
        locations: Vec<FaultLocation>,
        effect: FaultEffect,
    ) -> ZqnResult<Self> {
        let location = FaultLocation::composite(locations)?;

        Self::new(
            id,
            FaultClassification::Crosstalk,
            location,
            effect,
        )
    }

    /// Constructs a custom fault.
    pub fn custom(
        id: FaultId,
        classification: impl Into<String>,
        location: FaultLocation,
        effect: impl Into<String>,
    ) -> ZqnResult<Self> {
        Self::new(
            id,
            FaultClassification::Custom(classification.into()),
            location,
            FaultEffect::Custom(effect.into()),
        )
    }
}

// ============================================================================
// Canonical fault predicates
// ============================================================================

impl Fault {
    /// Returns whether the fault affects a canonical logical qubit directly.
    #[must_use]
    pub fn targets_logical_qubit(&self, qubit: QubitId) -> bool {
        self.logical_qubit() == Some(qubit)
    }

    /// Returns whether the fault affects a canonical physical qubit directly.
    #[must_use]
    pub fn targets_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.physical_qubit() == Some(qubit)
    }

    /// Returns whether the fault has the supplied classification.
    #[must_use]
    pub fn has_classification(
        &self,
        classification: &FaultClassification,
    ) -> bool {
        &self.classification == classification
    }

    /// Returns whether the fault has the supplied effect category.
    #[must_use]
    pub fn has_effect_category(&self, category: &str) -> bool {
        self.effect.category() == category
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fault_id(value: ZqnIdValue) -> FaultId {
        FaultId::new(value)
    }

    #[test]
    fn logical_fault_uses_canonical_ir_qubit_id() {
        let qubit = QubitId::new(7);

        let fault = Fault::logical_pauli(
            fault_id(1),
            FaultClassification::Gate,
            qubit,
            PauliEffect::X,
        )
        .expect("valid logical Pauli fault");

        assert_eq!(fault.logical_qubit(), Some(qubit));
        assert!(fault.is_logical());
        assert!(!fault.is_physical());
    }

    #[test]
    fn physical_fault_uses_canonical_ir_physical_qubit_id() {
        let qubit = PhysicalQubitId::new(17);

        let fault = Fault::physical_pauli(
            fault_id(2),
            FaultClassification::Gate,
            qubit,
            PauliEffect::Z,
        )
        .expect("valid physical Pauli fault");

        assert_eq!(fault.physical_qubit(), Some(qubit));
        assert!(fault.is_physical());
        assert!(!fault.is_logical());
    }

    #[test]
    fn logical_and_physical_domains_are_not_interchangeable() {
        let logical = FaultLocation::logical_qubit(QubitId::new(3));
        let physical =
            FaultLocation::physical_qubit(PhysicalQubitId::new(3));

        assert_ne!(logical, physical);
        assert_ne!(
            logical.resource_domain(),
            physical.resource_domain()
        );
    }

    #[test]
    fn empty_composite_location_is_rejected() {
        let result = FaultLocation::composite(Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_composite_resources_are_rejected() {
        let qubit = QubitId::new(4);

        let result = FaultLocation::composite(vec![
            FaultLocation::logical_qubit(qubit),
            FaultLocation::logical_qubit(qubit),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn correlated_fault_requires_composite_location() {
        let result = Fault::new(
            fault_id(3),
            FaultClassification::Correlated,
            FaultLocation::logical_qubit(QubitId::new(0)),
            FaultEffect::Pauli(PauliEffect::X),
        );

        assert!(result.is_err());
    }

    #[test]
    fn crosstalk_requires_composite_location() {
        let result = Fault::new(
            fault_id(4),
            FaultClassification::Crosstalk,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(0),
            ),
            FaultEffect::Pauli(PauliEffect::Y),
        );

        assert!(result.is_err());
    }

    #[test]
    fn leakage_classification_requires_leakage_effect() {
        let valid = Fault::new(
            fault_id(5),
            FaultClassification::Leakage,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(0),
            ),
            FaultEffect::Leakage { destination: 9 },
        );

        assert!(valid.is_ok());

        let invalid = Fault::new(
            fault_id(6),
            FaultClassification::Leakage,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(0),
            ),
            FaultEffect::Pauli(PauliEffect::X),
        );

        assert!(invalid.is_err());
    }

    #[test]
    fn erasure_classification_requires_erasure_effect() {
        let valid = Fault::erasure(
            fault_id(7),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
        );

        assert!(valid.is_ok());
    }

    #[test]
    fn loss_classification_requires_loss_effect() {
        let valid = Fault::loss(
            fault_id(8),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(2),
            ),
        );

        assert!(valid.is_ok());
    }

    #[test]
    fn readout_fault_is_measurement_classified() {
        let fault = Fault::readout(
            fault_id(9),
            FaultLocation::measurement(
                FaultOperationId::new(11),
            ),
            true,
        )
        .expect("valid readout fault");

        assert_eq!(
            fault.classification(),
            &FaultClassification::Measurement
        );
    }

    #[test]
    fn timing_interval_rejects_reverse_bounds() {
        let result = FaultTiming::interval(10, 9);

        assert!(result.is_err());
    }

    #[test]
    fn timing_interval_accepts_equal_bounds() {
        let result = FaultTiming::interval(10, 10);

        assert!(result.is_ok());
    }

    #[test]
    fn fault_is_immutable_through_copying() {
        let original = Fault::physical_pauli(
            fault_id(10),
            FaultClassification::Gate,
            PhysicalQubitId::new(4),
            PauliEffect::X,
        )
        .expect("valid fault");

        let timed = original
            .at_time(FaultTiming::instant(100))
            .expect("valid timing");

        assert!(!original.has_timing());
        assert!(timed.has_timing());
        assert_eq!(original.id(), timed.id());
    }

    #[test]
    fn operation_association_does_not_change_fault_identity() {
        let original = Fault::physical_pauli(
            fault_id(11),
            FaultClassification::Gate,
            PhysicalQubitId::new(5),
            PauliEffect::Y,
        )
        .expect("valid fault");

        let associated = original
            .associated_with(FaultOperationId::new(77))
            .expect("valid association");

        assert_eq!(original.id(), associated.id());
        assert_eq!(
            associated.operation(),
            Some(FaultOperationId::new(77))
        );
    }

    #[test]
    fn custom_fault_is_forward_compatible() {
        let fault = Fault::custom(
            fault_id(12),
            "future_quantum_fault",
            FaultLocation::external_resource(100),
            "future_effect",
        );

        assert!(fault.is_ok());
    }

    #[test]
    fn empty_custom_classification_is_rejected() {
        let result = Fault::custom(
            fault_id(13),
            "   ",
            FaultLocation::external_resource(101),
            "effect",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_custom_effect_is_rejected() {
        let result = Fault::custom(
            fault_id(14),
            "custom",
            FaultLocation::external_resource(102),
            "   ",
        );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_display_is_stable() {
        let fault = Fault::physical_pauli(
            fault_id(15),
            FaultClassification::Gate,
            PhysicalQubitId::new(8),
            PauliEffect::Z,
        )
        .expect("valid fault");

        assert_eq!(
            fault.to_string(),
            "fault:fault:15:gate:physical:p8:pauli:Z"
        );
    }

    #[test]
    fn ordering_is_deterministic() {
        let first = Fault::physical_pauli(
            fault_id(1),
            FaultClassification::Gate,
            PhysicalQubitId::new(0),
            PauliEffect::X,
        )
        .expect("valid fault");

        let second = Fault::physical_pauli(
            fault_id(2),
            FaultClassification::Gate,
            PhysicalQubitId::new(0),
            PauliEffect::X,
        )
        .expect("valid fault");

        assert!(first < second);
    }

    #[test]
    fn no_semantic_machine_size_limit_is_encoded() {
        let logical = QubitId::new(usize::MAX);
        let physical = PhysicalQubitId::new(usize::MAX);

        let logical_fault = Fault::logical_pauli(
            fault_id(u64::MAX),
            FaultClassification::Gate,
            logical,
            PauliEffect::X,
        );

        let physical_fault = Fault::physical_pauli(
            fault_id(u64::MAX),
            FaultClassification::Gate,
            physical,
            PauliEffect::Z,
        );

        assert!(logical_fault.is_ok());
        assert!(physical_fault.is_ok());
    }

    #[test]
    fn canonical_qubit_values_are_not_recreated_by_zqn() {
        let logical = QubitId::new(1);
        let physical = PhysicalQubitId::new(1);

        let logical_location = FaultLocation::logical_qubit(logical);
        let physical_location =
            FaultLocation::physical_qubit(physical);

        assert_eq!(
            logical_location.logical_qubit_id(),
            Some(logical)
        );

        assert_eq!(
            physical_location.physical_qubit_id(),
            Some(physical)
        );
    }

    #[test]
    fn annotation_requires_non_empty_key() {
        let result = FaultAnnotation::new("", "value");

        assert!(result.is_err());
    }

    #[test]
    fn annotation_preserves_value() {
        let annotation =
            FaultAnnotation::new("source", "simulation")
                .expect("valid annotation");

        assert_eq!(annotation.key(), "source");
        assert_eq!(annotation.value(), "simulation");
    }

    #[test]
    fn full_fault_validation_succeeds() {
        let annotation =
            FaultAnnotation::new("source", "characterization")
                .expect("valid annotation");

        let fault = Fault::with_details(
            fault_id(16),
            FaultClassification::Measurement,
            FaultLocation::measurement(
                FaultOperationId::new(200),
            ),
            FaultEffect::Readout {
                assigned_value: true,
            },
            FaultTiming::instant(500),
            Some(FaultOperationId::new(200)),
            Some(annotation),
        )
        .expect("valid complete fault");

        assert!(fault.validate().is_ok());
        assert!(fault.has_timing());
        assert!(fault.operation().is_some());
        assert!(fault.annotation().is_some());
    }
}