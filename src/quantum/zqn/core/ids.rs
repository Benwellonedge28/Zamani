//! Zamani Quantum Noise (ZQN) — Canonical Identity Types
//!
//! This module owns identity types that are specific to the ZQN subsystem.
//!
//! # Architectural responsibility
//!
//! `ids.rs` answers:
//!
//! > "Which ZQN object, observation, model, calibration snapshot, experiment,
//! > or noise realization is being referred to?"
//!
//! It owns identifiers for ZQN-domain objects.
//!
//! It does NOT own quantum-resource identity.
//!
//! The canonical quantum-resource identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN MUST NOT define another `QubitId`, `PhysicalQubitId`, or equivalent
//! wrapper merely for convenience.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - `NoiseModelId`;
//! - `ChannelId`;
//! - `FaultId`;
//! - `NoiseApplicationId`;
//! - `NoiseSnapshotId`;
//! - `CalibrationId`;
//! - `CharacterizationId`;
//! - `ExperimentId`;
//! - `ObservationId`;
//! - `NoiseRealizationId`;
//! - `CorrelationId`;
//! - `NoiseParameterId`;
//! - `DistributionId`;
//! - `ErrorBudgetId`;
//! - `NoiseProfileId`;
//! - `ZqnObjectId`;
//! - canonical aliases/re-exports for the IR's logical and physical qubit IDs.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - logical-qubit semantics;
//! - physical-qubit semantics;
//! - hardware existence;
//! - hardware topology;
//! - device allocation;
//! - routing;
//! - scheduling;
//! - calibration values;
//! - quantum channels;
//! - probabilities;
//! - noise models;
//! - fault semantics;
//! - execution;
//! - random-number generation.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical qubit identity
//!
//! The Zamani quantum architecture explicitly establishes
//! `quantum::ir::qubit` as the canonical qubit identity boundary.
//!
//! Therefore this module imports:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! and does not recreate either type.
//!
//! This is important because two integer values can be numerically identical
//! while belonging to different semantic identity domains.
//!
//! For example:
//!
//! ```text
//! QubitId::new(7)
//! PhysicalQubitId::new(7)
//! ```
//!
//! are intentionally different types even though both contain the value `7`.
//!
//! A ZQN object that refers to a quantum resource must use the appropriate
//! canonical IR type.
//!
//! # Write once, scale everywhere
//!
//! ZQN identity types have no semantic machine-size limit.
//!
//! The identifier representation is an implementation detail of identity;
//! it is not a declaration of the number of qubits, devices, channels,
//! experiments, or noise events that Zamani can represent.
//!
//! No identifier type in this module contains:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_FAULTS
//! MAX_EXPERIMENTS
//! MAX_DEVICES
//! MAX_OPERATIONS
//! ```
//!
//! Resource limits belong to resource-policy/capability layers.
//!
//! An identifier may identify any object representable by the host platform
//! and the surrounding execution/storage infrastructure.
//!
//! "Infinity" in the Zamani architecture means that no artificial finite
//! machine-size ceiling is encoded into the language or ZQN semantic model.
//!
//! It does not claim that an individual host, distributed system, storage
//! system, or quantum processor has infinite resources.
//!
//! # Identity versus existence
//!
//! Constructing an identifier does NOT prove that the referenced object exists.
//!
//! For example:
//!
//! ```text
//! PhysicalQubitId::new(10_000)
//! ```
//!
//! does not prove that a device has physical qubit 10,000.
//!
//! Hardware capability/availability validation belongs to `quantum::hardware`.
//!
//! Likewise:
//!
//! ```text
//! NoiseModelId::new(...)
//! ```
//!
//! does not mean that a corresponding model has been registered.
//!
//! Registry, lookup, lifecycle and existence semantics belong to the subsystem
//! that owns those resources.
//!
//! # Identity model
//!
//! ZQN uses two complementary identity layers:
//!
//! ```text
//! semantic identity
//!     │
//!     ├── canonical IR QubitId
//!     └── canonical IR PhysicalQubitId
//!
//! ZQN object identity
//!     │
//!     ├── NoiseModelId
//!     ├── ChannelId
//!     ├── FaultId
//!     ├── CalibrationId
//!     ├── ExperimentId
//!     └── ...
//! ```
//!
//! This separation prevents ZQN from becoming a second semantic IR.
//!
//! # Stable identity
//!
//! ZQN IDs are opaque value types.
//!
//! Their numeric representation MUST NOT be interpreted by consumers as:
//!
//! - ordering of creation;
//! - hardware location;
//! - memory address;
//! - array index;
//! - qubit count;
//! - execution priority.
//!
//! Ordering is provided only where useful for deterministic collections and
//! canonical serialization.
//!
//! Equality means identity equality, not semantic equality of the object
//! represented by the identifier.
//!
//! Two different IDs may refer to semantically equivalent objects.
//!
//! Conversely, two objects with similar descriptions may intentionally have
//! different identities because their provenance/configuration is different.
//!
//! # Determinism
//!
//! This module performs no random generation.
//!
//! IDs are explicit values.
//!
//! It does not depend on:
//!
//! - thread-local RNGs;
//! - global RNGs;
//! - system time;
//! - process IDs;
//! - memory addresses;
//! - hash-map iteration order.
//!
//! Deterministic execution therefore remains controlled by the ZQN
//! reproducibility subsystem rather than by this identity layer.
//!
//! # Serialization
//!
//! The identity types deliberately do not require a serialization framework.
//!
//! This keeps this foundational module independent of optional serialization
//! infrastructure and compatible with the repository's current dependency
//! direction.
//!
//! The canonical serialization layer may encode IDs using their numeric value.
//!
//! Serialization MUST preserve:
//!
//! ```text
//! type/domain + identifier value
//! ```
//!
//! It must never collapse different identity domains into a single untyped
//! integer.
//!
//! For example, a serialized logical qubit and serialized physical qubit must
//! remain distinguishable by their schema/type context.
//!
//! # Hashing
//!
//! All IDs implement `Hash` so they can be used by:
//!
//! - deterministic maps;
//! - sets;
//! - caches;
//! - registries;
//! - dependency graphs;
//! - provenance indexes.
//!
//! Hash equality follows identity equality.
//!
//! Hashes MUST NOT be treated as persistent serialized identities.
//!
//! # Ordering
//!
//! IDs implement `Ord` and `PartialOrd` for deterministic iteration and
//! canonical output.
//!
//! Consumers MUST NOT interpret this ordering as semantic ordering.
//!
//! The ordering is an implementation-level deterministic ordering only.
//!
//! # Thread safety
//!
//! All ZQN identity types are immutable value types.
//!
//! They contain no interior mutability and no global state.
//!
//! Consequently they are suitable for concurrent use when the surrounding
//! containers/context are also thread-safe.
//!
//! # Safety
//!
//! This file contains no unsafe Rust.
//!
//! `#![forbid(unsafe_code)]` makes accidental introduction of unsafe code a
//! compile-time error.
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
//! - no unsafe code;
//! - no external dependency requirements.
//!
//! # Integration contract
//!
//! ZQN consumers should use:
//!
//! ```text
//! crate::quantum::zqn::core::ids::NoiseModelId
//! crate::quantum::zqn::core::ids::ChannelId
//! crate::quantum::zqn::core::ids::FaultId
//! ```
//!
//! for ZQN objects.
//!
//! They should use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! for quantum-resource identity.
//!
//! If a ZQN API needs to refer to a qubit, its signature should therefore
//! contain the canonical IR type directly.
//!
//! Example:
//!
//! ```text
//! fn noise_for_qubit(
//!     qubit: crate::quantum::ir::qubit::QubitId,
//! ) -> ZqnResult<...>
//! ```
//!
//! not:
//!
//! ```text
//! fn noise_for_qubit(
//!     qubit: ZqnQubitId,
//! ) -> ...
//! ```
//!
//! # Dependency direction
//!
//! The intended dependency is:
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! quantum::zqn::core::ids
//!        │
//!        ├── probability
//!        ├── channel
//!        ├── fault
//!        ├── noise
//!        ├── calibration
//!        ├── characterization
//!        ├── simulation
//!        ├── propagation
//!        ├── target
//!        └── integration
//! ```
//!
//! `ids.rs` must remain independent of all those downstream ZQN modules.
//!
//! This allows this file to be completed and stabilized before the rest of
//! ZQN is implemented.
//!
//! # No circular dependency
//!
//! This module MUST NOT import:
//!
//! ```text
//! zqn::noise
//! zqn::channel
//! zqn::fault
//! zqn::calibration
//! zqn::simulation
//! zqn::target
//! ```
//!
//! Those modules may depend on these identity types.
//!
//! # Identity allocation
//!
//! This module intentionally does not provide a global allocator or registry.
//!
//! There is no:
//!
//! ```text
//! static GLOBAL_ID_COUNTER
//! ```
//!
//! and no hidden mutable identity service.
//!
//! Identity allocation policy belongs to the object-owning subsystem.
//!
//! A subsystem may derive IDs from:
//!
//! - deterministic program construction;
//! - explicit caller-provided values;
//! - stable hashes;
//! - execution context;
//! - persistent registries;
//! - distributed coordination.
//!
//! Such policies must remain outside this foundational identity vocabulary.
//!
//! # Collision policy
//!
//! These types do not claim global uniqueness merely because they are typed.
//!
//! A caller-provided `NoiseModelId` can collide with another independently
//! constructed `NoiseModelId` if the caller chooses the same value.
//!
//! Global uniqueness, persistence and namespace scope are responsibilities of
//! the owning registry/provenance layer.
//!
//! This distinction is deliberate: identity vocabulary and identity allocation
//! are different architectural responsibilities.
//!
//! # Distributed systems
//!
//! The types are intentionally opaque and value-based so they can cross
//! process boundaries without carrying pointers or backend handles.
//!
//! Distributed uniqueness must be established by the owning subsystem.
//!
//! `ids.rs` itself performs no network coordination.
//!
//! # Security
//!
//! IDs are not authentication credentials.
//!
//! Possession of an ID MUST NOT grant access to:
//!
//! - a QPU;
//! - calibration data;
//! - private experiments;
//! - execution credentials;
//! - encrypted data;
//! - hardware controls.
//!
//! Authorization belongs to the surrounding capability/security subsystem.
//!
//! # Testing contract
//!
//! Tests for this file should establish:
//!
//! 1. IDs preserve equality semantics;
//! 2. IDs preserve hashing semantics;
//! 3. IDs remain distinct across ZQN domains;
//! 4. conversions preserve values;
//! 5. display/debug output is deterministic;
//! 6. canonical IR qubit identities remain the canonical types;
//! 7. no arithmetic overflow is possible through helper APIs;
//! 8. no global mutable state is introduced;
//! 9. IDs remain cheap immutable value types;
//! 10. compilation succeeds with Rust 1.97/1.97.1.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::fmt;

// ============================================================================
// Canonical quantum-resource identities
// ============================================================================
//
// IMPORTANT:
//
// These are aliases/re-exports of the canonical IR types.
//
// They are NOT new types.
//
// Do not replace these with locally defined structs/enums.
//
// The repository's quantum IR explicitly establishes `quantum::ir::qubit`
// as the authoritative logical/physical qubit identity boundary.

/// Canonical logical-qubit identity owned by the Quantum IR.
///
/// This is a direct re-export of
/// [`crate::quantum::ir::qubit::QubitId`].
///
/// ZQN does not define another logical-qubit identifier.
pub use crate::quantum::ir::qubit::QubitId;

/// Canonical physical-qubit identity owned by the Quantum IR.
///
/// This is a direct re-export of
/// [`crate::quantum::ir::qubit::PhysicalQubitId`].
///
/// ZQN does not define another physical-qubit identifier.
pub use crate::quantum::ir::qubit::PhysicalQubitId;

// ============================================================================
// Shared ZQN identifier representation
// ============================================================================

/// Internal representation used by ZQN identity types.
///
/// `u64` provides a stable, portable value representation without tying the
/// identity to a machine pointer or memory address.
///
/// This value is an identifier, not a resource count.
///
/// No ZQN semantic rule interprets the maximum representable value as a
/// maximum number of quantum resources.
///
/// The owning subsystem remains responsible for establishing uniqueness.
pub type ZqnIdValue = u64;

// ============================================================================
// Generic ZQN object identity
// ============================================================================

/// Generic identity for a ZQN-owned object.
///
/// `ZqnObjectId` is useful at generic integration boundaries where the
/// concrete ZQN object category is carried separately by the surrounding
/// schema/type.
///
/// Prefer the more specific typed IDs when the object category is statically
/// known.
///
/// # Example
///
/// ```text
/// ZqnObjectId::new(42)
/// ```
///
/// does not mean:
///
/// ```text
/// noise model #42
/// ```
///
/// The surrounding type/schema determines what object category the value
/// identifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZqnObjectId(ZqnIdValue);

impl ZqnObjectId {
    /// Creates an object identity from an explicit value.
    ///
    /// Construction does not establish existence or uniqueness.
    #[must_use]
    pub const fn new(value: ZqnIdValue) -> Self {
        Self(value)
    }

    /// Returns the raw identifier value.
    #[must_use]
    pub const fn value(self) -> ZqnIdValue {
        self.0
    }
}

impl From<ZqnIdValue> for ZqnObjectId {
    fn from(value: ZqnIdValue) -> Self {
        Self::new(value)
    }
}

impl From<ZqnObjectId> for ZqnIdValue {
    fn from(value: ZqnObjectId) -> Self {
        value.value()
    }
}

impl fmt::Display for ZqnObjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "zqn:{}", self.0)
    }
}

// ============================================================================
// Typed identifier macro
// ============================================================================
//
// Each semantic domain receives a distinct Rust type.
//
// This prevents accidental mixing:
//
//     NoiseModelId == ChannelId
//
// even if their underlying numeric values happen to be equal.
//
// The macro contains no unsafe code and introduces no global state.

macro_rules! define_zqn_id {
    (
        $(#[$meta:meta])*
        $name:ident,
        $prefix:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(ZqnIdValue);

        impl $name {
            /// Creates an identifier from an explicit value.
            ///
            /// Construction does not establish existence or uniqueness.
            #[must_use]
            pub const fn new(value: ZqnIdValue) -> Self {
                Self(value)
            }

            /// Returns the underlying identifier value.
            #[must_use]
            pub const fn value(self) -> ZqnIdValue {
                self.0
            }

            /// Returns the next representable identifier value.
            ///
            /// This helper is useful for deterministic local allocation but
            /// does not allocate or register an object.
            ///
            /// Returning `None` on overflow makes the operation safe and
            /// explicit rather than wrapping.
            #[must_use]
            pub const fn checked_next(self) -> Option<Self> {
                match self.0.checked_add(1) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns the canonical textual prefix for this identity domain.
            ///
            /// The prefix is presentation metadata only.
            #[must_use]
            pub const fn prefix() -> &'static str {
                $prefix
            }
        }

        impl From<ZqnIdValue> for $name {
            fn from(value: ZqnIdValue) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for ZqnIdValue {
            fn from(value: $name) -> Self {
                value.value()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}:{}", $prefix, self.0)
            }
        }
    };
}

// ============================================================================
// Noise-model identity
// ============================================================================

define_zqn_id!(
    /// Identity of a ZQN noise model.
    ///
    /// A `NoiseModelId` identifies a noise-model object within the namespace
    /// established by its owning registry/provenance system.
    ///
    /// It does not contain the model itself.
    ///
    /// It does not guarantee that a model is registered.
    ///
    /// It does not encode hardware size or target identity.
    NoiseModelId,
    "noise-model"
);

// ============================================================================
// Quantum-channel identity
// ============================================================================

define_zqn_id!(
    /// Identity of a ZQN quantum channel.
    ///
    /// The channel's mathematical representation belongs to the channel
    /// subsystem; this type identifies the channel object only.
    ChannelId,
    "channel"
);

// ============================================================================
// Fault identity
// ============================================================================

define_zqn_id!(
    /// Identity of a ZQN fault description or fault object.
    ///
    /// A fault ID is distinct from a physical qubit ID. The fault's location
    /// may refer to one or more canonical IR resources.
    FaultId,
    "fault"
);

// ============================================================================
// Noise-application identity
// ============================================================================

define_zqn_id!(
    /// Identity of a concrete application of noise semantics to an operation,
    /// resource, time interval, or other ZQN location.
    ///
    /// This is distinct from `NoiseModelId`: one model may produce many
    /// applications.
    NoiseApplicationId,
    "noise-application"
);

// ============================================================================
// Noise-snapshot identity
// ============================================================================

define_zqn_id!(
    /// Identity of an immutable noise snapshot.
    ///
    /// A snapshot may bind a model/configuration to a particular reproducible
    /// state without embedding the state in the ID itself.
    NoiseSnapshotId,
    "noise-snapshot"
);

// ============================================================================
// Calibration identity
// ============================================================================

define_zqn_id!(
    /// Identity of a calibration snapshot or calibration object.
    ///
    /// Calibration values themselves belong to `zqn::calibration`.
    CalibrationId,
    "calibration"
);

// ============================================================================
// Characterization identity
// ============================================================================

define_zqn_id!(
    /// Identity of a characterization result or characterization object.
    ///
    /// Experimental observations and statistical estimates belong to the
    /// characterization subsystem.
    CharacterizationId,
    "characterization"
);

// ============================================================================
// Experiment identity
// ============================================================================

define_zqn_id!(
    /// Identity of a noise-characterization or noise-analysis experiment.
    ExperimentId,
    "experiment"
);

// ============================================================================
// Observation identity
// ============================================================================

define_zqn_id!(
    /// Identity of a raw or processed ZQN observation.
    ///
    /// An observation may originate from simulation, characterization,
    /// execution, benchmarking, or another explicitly declared source.
    ObservationId,
    "observation"
);

// ============================================================================
// Noise-realization identity
// ============================================================================

define_zqn_id!(
    /// Identity of a concrete stochastic noise realization.
    ///
    /// This identifies the realization object; it is not itself an RNG seed.
    ///
    /// Reproducibility information belongs to the ZQN deterministic execution
    /// context.
    NoiseRealizationId,
    "noise-realization"
);

// ============================================================================
// Correlation identity
// ============================================================================

define_zqn_id!(
    /// Identity of a correlation definition/domain.
    ///
    /// Correlation semantics belong to `zqn::noise::correlation`,
    /// `spatial`, `temporal`, or related modules.
    CorrelationId,
    "correlation"
);

// ============================================================================
// Noise-parameter identity
// ============================================================================

define_zqn_id!(
    /// Identity of a parameter in a noise model or calibration specification.
    ///
    /// This is deliberately generic because ZQN must not hard-code a fixed
    /// vocabulary of physical parameters.
    NoiseParameterId,
    "noise-parameter"
);

// ============================================================================
// Distribution identity
// ============================================================================

define_zqn_id!(
    /// Identity of a probability distribution object used by ZQN.
    ///
    /// The mathematical distribution belongs to `zqn::probability`.
    DistributionId,
    "distribution"
);

// ============================================================================
// Error-budget identity
// ============================================================================

define_zqn_id!(
    /// Identity of an error-budget object.
    ///
    /// Error-budget semantics belong to `zqn::propagation::error_budget`.
    ErrorBudgetId,
    "error-budget"
);

// ============================================================================
// Noise-profile identity
// ============================================================================

define_zqn_id!(
    /// Identity of an aggregate noise profile.
    ///
    /// A profile can summarize or expose noise characteristics for routing,
    /// scheduling, target selection, analysis, or execution without becoming
    /// a replacement for the underlying `NoiseModel`.
    NoiseProfileId,
    "noise-profile"
);

// ============================================================================
// Identity-domain classification
// ============================================================================

/// Identifies the ZQN domain represented by a [`ZqnObjectId`].
///
/// This is useful at generic serialization, provenance and integration
/// boundaries.
///
/// It does not replace the strongly typed ID structures above.
///
/// Strongly typed IDs should be preferred whenever possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZqnIdKind {
    /// Generic ZQN object.
    Object,

    /// Noise model.
    NoiseModel,

    /// Quantum channel.
    Channel,

    /// Fault.
    Fault,

    /// Noise application.
    NoiseApplication,

    /// Noise snapshot.
    NoiseSnapshot,

    /// Calibration.
    Calibration,

    /// Characterization.
    Characterization,

    /// Experiment.
    Experiment,

    /// Observation.
    Observation,

    /// Concrete noise realization.
    NoiseRealization,

    /// Correlation definition.
    Correlation,

    /// Noise parameter.
    NoiseParameter,

    /// Probability distribution.
    Distribution,

    /// Error budget.
    ErrorBudget,

    /// Aggregate noise profile.
    NoiseProfile,
}

impl ZqnIdKind {
    /// Returns a stable schema-oriented name for this identity kind.
    ///
    /// These names are part of the textual/schema contract and should not be
    /// changed casually.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::NoiseModel => "noise-model",
            Self::Channel => "channel",
            Self::Fault => "fault",
            Self::NoiseApplication => "noise-application",
            Self::NoiseSnapshot => "noise-snapshot",
            Self::Calibration => "calibration",
            Self::Characterization => "characterization",
            Self::Experiment => "experiment",
            Self::Observation => "observation",
            Self::NoiseRealization => "noise-realization",
            Self::Correlation => "correlation",
            Self::NoiseParameter => "noise-parameter",
            Self::Distribution => "distribution",
            Self::ErrorBudget => "error-budget",
            Self::NoiseProfile => "noise-profile",
        }
    }
}

impl fmt::Display for ZqnIdKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Typed-to-generic identity conversion
// ============================================================================
//
// These conversions deliberately erase only the Rust type at an explicitly
// generic integration boundary. The caller must retain the corresponding
// `ZqnIdKind`.
//
// There is intentionally NO:
//     impl From<NoiseModelId> for ChannelId
//
// or any other cross-domain conversion.
//
// Such conversions would destroy the type-safety this module exists to
// provide.

/// A generic typed identity paired with its domain.
///
/// This is useful for generic registries, provenance records and serialization
/// boundaries where the concrete ID type is not statically known.
///
/// The identity value itself remains opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypedZqnId {
    kind: ZqnIdKind,
    value: ZqnIdValue,
}

impl TypedZqnId {
    /// Creates a typed generic identity.
    #[must_use]
    pub const fn new(kind: ZqnIdKind, value: ZqnIdValue) -> Self {
        Self { kind, value }
    }

    /// Returns the identity domain.
    #[must_use]
    pub const fn kind(self) -> ZqnIdKind {
        self.kind
    }

    /// Returns the underlying identity value.
    #[must_use]
    pub const fn value(self) -> ZqnIdValue {
        self.value
    }
}

impl fmt::Display for TypedZqnId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.kind, self.value)
    }
}

// ============================================================================
// TypedZqnId conversions
// ============================================================================

impl From<NoiseModelId> for TypedZqnId {
    fn from(value: NoiseModelId) -> Self {
        Self::new(ZqnIdKind::NoiseModel, value.value())
    }
}

impl From<ChannelId> for TypedZqnId {
    fn from(value: ChannelId) -> Self {
        Self::new(ZqnIdKind::Channel, value.value())
    }
}

impl From<FaultId> for TypedZqnId {
    fn from(value: FaultId) -> Self {
        Self::new(ZqnIdKind::Fault, value.value())
    }
}

impl From<NoiseApplicationId> for TypedZqnId {
    fn from(value: NoiseApplicationId) -> Self {
        Self::new(ZqnIdKind::NoiseApplication, value.value())
    }
}

impl From<NoiseSnapshotId> for TypedZqnId {
    fn from(value: NoiseSnapshotId) -> Self {
        Self::new(ZqnIdKind::NoiseSnapshot, value.value())
    }
}

impl From<CalibrationId> for TypedZqnId {
    fn from(value: CalibrationId) -> Self {
        Self::new(ZqnIdKind::Calibration, value.value())
    }
}

impl From<CharacterizationId> for TypedZqnId {
    fn from(value: CharacterizationId) -> Self {
        Self::new(ZqnIdKind::Characterization, value.value())
    }
}

impl From<ExperimentId> for TypedZqnId {
    fn from(value: ExperimentId) -> Self {
        Self::new(ZqnIdKind::Experiment, value.value())
    }
}

impl From<ObservationId> for TypedZqnId {
    fn from(value: ObservationId) -> Self {
        Self::new(ZqnIdKind::Observation, value.value())
    }
}

impl From<NoiseRealizationId> for TypedZqnId {
    fn from(value: NoiseRealizationId) -> Self {
        Self::new(ZqnIdKind::NoiseRealization, value.value())
    }
}

impl From<CorrelationId> for TypedZqnId {
    fn from(value: CorrelationId) -> Self {
        Self::new(ZqnIdKind::Correlation, value.value())
    }
}

impl From<NoiseParameterId> for TypedZqnId {
    fn from(value: NoiseParameterId) -> Self {
        Self::new(ZqnIdKind::NoiseParameter, value.value())
    }
}

impl From<DistributionId> for TypedZqnId {
    fn from(value: DistributionId) -> Self {
        Self::new(ZqnIdKind::Distribution, value.value())
    }
}

impl From<ErrorBudgetId> for TypedZqnId {
    fn from(value: ErrorBudgetId) -> Self {
        Self::new(ZqnIdKind::ErrorBudget, value.value())
    }
}

impl From<NoiseProfileId> for TypedZqnId {
    fn from(value: NoiseProfileId) -> Self {
        Self::new(ZqnIdKind::NoiseProfile, value.value())
    }
}

// ============================================================================
// Canonical resource references
// ============================================================================

/// Canonical ZQN reference to a quantum resource.
///
/// This enum intentionally uses the canonical IR identity types.
///
/// It does not create a new qubit identity domain.
///
/// Future resource kinds should be introduced only when the canonical IR has
/// an authoritative identity type for them or when the ZQN architecture
/// explicitly defines a resource identity that is genuinely ZQN-owned.
///
/// In particular, ZQN should not manufacture `QuditId`, `ModeId`, or other
/// quantum-resource identifiers here merely to anticipate future hardware.
///
/// Such identities require an architectural decision at the canonical IR
/// boundary first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumResourceRef {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl QuantumResourceRef {
    /// Returns the logical qubit if this is a logical-qubit reference.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            Self::PhysicalQubit(_) => None,
        }
    }

    /// Returns the physical qubit if this is a physical-qubit reference.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) => None,
            Self::PhysicalQubit(id) => Some(id),
        }
    }

    /// Returns whether this reference denotes a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this reference denotes a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

impl From<QubitId> for QuantumResourceRef {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for QuantumResourceRef {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl fmt::Display for QuantumResourceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "logical:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical:{id}"),
        }
    }
}

// ============================================================================
// Stable identity comparison helpers
// ============================================================================

/// Compares two identity values without assigning semantic meaning to their
/// numeric ordering.
///
/// This helper exists primarily for generic deterministic infrastructure.
#[must_use]
pub const fn compare_id_values(
    left: ZqnIdValue,
    right: ZqnIdValue,
) -> std::cmp::Ordering {
    left.cmp(&right)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_id_round_trips() {
        let id = ZqnObjectId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(ZqnIdValue::from(id), 42);
        assert_eq!(ZqnObjectId::from(42), id);
    }

    #[test]
    fn typed_ids_are_value_stable() {
        let id = NoiseModelId::new(17);

        assert_eq!(id.value(), 17);
        assert_eq!(NoiseModelId::from(17), id);
        assert_eq!(ZqnIdValue::from(id), 17);
    }

    #[test]
    fn typed_ids_are_domain_specific() {
        let model = NoiseModelId::new(1);
        let channel = ChannelId::new(1);

        assert_eq!(model.value(), channel.value());
        assert_ne!(
            TypedZqnId::from(model),
            TypedZqnId::from(channel)
        );
    }

    #[test]
    fn typed_identity_preserves_domain() {
        let id = FaultId::new(91);
        let typed = TypedZqnId::from(id);

        assert_eq!(typed.kind(), ZqnIdKind::Fault);
        assert_eq!(typed.value(), 91);
    }

    #[test]
    fn display_is_deterministic() {
        assert_eq!(
            NoiseModelId::new(7).to_string(),
            "noise-model:7"
        );

        assert_eq!(
            ChannelId::new(8).to_string(),
            "channel:8"
        );

        assert_eq!(
            FaultId::new(9).to_string(),
            "fault:9"
        );
    }

    #[test]
    fn checked_next_does_not_wrap() {
        let id = NoiseModelId::new(ZqnIdValue::MAX);

        assert_eq!(id.checked_next(), None);

        let first = NoiseModelId::new(41);

        assert_eq!(
            first.checked_next(),
            Some(NoiseModelId::new(42))
        );
    }

    #[test]
    fn quantum_resource_ref_preserves_identity_domain() {
        let logical = QubitId::new(3);
        let physical = PhysicalQubitId::new(3);

        let logical_ref = QuantumResourceRef::from(logical);
        let physical_ref = QuantumResourceRef::from(physical);

        assert!(logical_ref.is_logical());
        assert!(!logical_ref.is_physical());
        assert_eq!(logical_ref.logical_qubit(), Some(logical));
        assert_eq!(logical_ref.physical_qubit(), None);

        assert!(physical_ref.is_physical());
        assert!(!physical_ref.is_logical());
        assert_eq!(physical_ref.logical_qubit(), None);
        assert_eq!(physical_ref.physical_qubit(), Some(physical));
    }

    #[test]
    fn quantum_resource_display_is_deterministic() {
        assert_eq!(
            QuantumResourceRef::from(QubitId::new(4)).to_string(),
            "logical:q4"
        );

        assert_eq!(
            QuantumResourceRef::from(PhysicalQubitId::new(5)).to_string(),
            "physical:p5"
        );
    }

    #[test]
    fn id_kind_names_are_stable() {
        assert_eq!(ZqnIdKind::NoiseModel.as_str(), "noise-model");
        assert_eq!(ZqnIdKind::Channel.as_str(), "channel");
        assert_eq!(ZqnIdKind::Fault.as_str(), "fault");
        assert_eq!(
            ZqnIdKind::NoiseApplication.as_str(),
            "noise-application"
        );
        assert_eq!(
            ZqnIdKind::NoiseRealization.as_str(),
            "noise-realization"
        );
    }

    #[test]
    fn ids_are_orderable_for_deterministic_collections() {
        let first = ChannelId::new(1);
        let second = ChannelId::new(2);

        assert!(first < second);
    }

    #[test]
    fn distinct_zqn_domains_do_not_compare_equal() {
        let channel = TypedZqnId::from(ChannelId::new(100));
        let fault = TypedZqnId::from(FaultId::new(100));

        assert_ne!(channel, fault);
    }

    #[test]
    fn canonical_qubit_types_are_used_directly() {
        let logical = QubitId::new(12);
        let physical = PhysicalQubitId::new(24);

        assert_eq!(logical.index(), 12);
        assert_eq!(physical.index(), 24);
    }

    #[test]
    fn no_implicit_cross_domain_conversion_exists() {
        //
        // This test is intentionally compile-time/documentational.
        //
        // There must be no:
        //
        // From<NoiseModelId> for ChannelId
        // From<ChannelId> for FaultId
        // From<QubitId> for PhysicalQubitId
        //
        // The type system itself enforces those boundaries.
        //
        let model = NoiseModelId::new(1);
        let channel = ChannelId::new(1);

        assert_eq!(model.value(), channel.value());
        assert_ne!(
            TypedZqnId::from(model).kind(),
            TypedZqnId::from(channel).kind()
        );
    }
}