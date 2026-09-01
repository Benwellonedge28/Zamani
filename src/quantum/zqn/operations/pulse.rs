//! Zamani Quantum Noise (ZQN) — Pulse-Level Noise Integration
//!
//! Path:
//!
//!     src/quantum/zqn/operations/pulse.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines the ZQN-side semantic attachment between a canonical
//! Zamani pulse operation and physical uncertainty/noise associated with that
//! pulse.
//!
//! The canonical pulse itself remains owned by:
//!
//!     crate::quantum::ir::pulse
//!
//! In particular, this module does NOT redefine:
//!
//! - Pulse;
//! - PulseId;
//! - PulseKind;
//! - PulseTarget;
//! - PulseDuration;
//! - waveform semantics;
//! - frame semantics;
//! - pulse scheduling;
//! - hardware channels;
//! - physical control electronics.
//!
//! Instead, this module answers:
//!
//!     "How is ZQN noise associated with an existing canonical pulse?"
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - pulse-noise binding semantics;
//! - pulse-noise phase semantics;
//! - pulse noise target scoping;
//! - references to ZQN noise/channel/calibration identities;
//! - explicit pulse-noise metadata;
//! - local validation of pulse-noise bindings;
//! - deterministic structural representation;
//! - validation of a binding against a canonical PulseId;
//! - explicit approximation semantics at the pulse-noise boundary.
//!
//! This file does NOT own:
//!
//! - canonical pulse semantics;
//! - waveform generation;
//! - waveform storage;
//! - physical channel allocation;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration values;
//! - noise-channel mathematics;
//! - noise-model implementation;
//! - random sampling;
//! - simulator state;
//! - QEC decoding;
//! - backend execution;
//! - provider SDKs;
//! - credentials;
//! - global resource limits.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//!                  Zamani source
//!                       |
//!                       v
//!                canonical quantum IR
//!                       |
//!                       v
//!              quantum::ir::pulse::Pulse
//!                       |
//!                       | canonical PulseId
//!                       v
//!       +-----------------------------------+
//!       | zqn::operations::pulse           |
//!       |                                   |
//!       | PulseNoiseBinding                 |
//!       | PulseNoiseTarget                  |
//!       | PulseNoisePhase                   |
//!       | PulseNoiseSource                  |
//!       +----------------+------------------+
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!        zqn::noise             zqn::channel
//!              |                   |
//!              +---------+---------+
//!                        |
//!                        v
//!                 zqn::calibration
//!                        |
//!                        v
//!                 target lowering
//!                        |
//!              +---------+----------+
//!              |                    |
//!              v                    v
//!          simulator             hardware
//! ```
//!
//! The dependency direction is intentional:
//!
//! canonical pulse semantics
//!         ↓
//! ZQN pulse-noise attachment
//!         ↓
//! noise/channel/calibration resolution
//!         ↓
//! target-specific realization
//!
//! ============================================================================
//! CANONICAL IDENTITY CONTRACT
//! ============================================================================
//!
//! Pulse identity is owned by the canonical IR:
//!
//!     crate::quantum::ir::identity::PulseId
//!
//! Logical qubit identity is owned by:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Physical qubit identity is owned by:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! ZQN MUST NOT define:
//!
//!     ZqnPulseId
//!     ZqnQubitId
//!     ZqnPhysicalQubitId
//!
//! merely for convenience.
//!
//! ============================================================================
//! WHY THIS IS A BINDING RATHER THAN A SECOND PULSE
//! ============================================================================
//!
//! The canonical pulse already contains:
//!
//! - pulse identity;
//! - pulse kind;
//! - targets;
//! - duration;
//! - waveform references;
//! - channel references;
//! - frame references;
//! - calibration references;
//! - pulse-local parameters;
//! - pulse dependencies.
//!
//! Duplicating those values inside ZQN would create two sources of truth.
//!
//! Therefore this module stores the canonical PulseId and only the additional
//! noise semantics.
//!
//! A runtime/noise engine resolves the PulseId back to the canonical Pulse
//! before executing the binding.
//!
//! This prevents stale copies of:
//!
//!     duration
//!     waveform
//!     frame
//!     target
//!     pulse kind
//!
//! from silently disagreeing with the canonical IR.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! The same Zamani program must remain expressible across:
//!
//! - different machine sizes;
//! - different pulse architectures;
//! - different control technologies;
//! - different simulator representations;
//! - different hardware targets;
//! - future quantum modalities.
//!
//! Consequently this module contains:
//!
//! - no maximum pulse count;
//! - no maximum target count;
//! - no maximum qubit count;
//! - no fixed pulse duration;
//! - no fixed waveform size;
//! - no fixed hardware clock;
//! - no fixed channel topology;
//! - no vendor-specific pulse names;
//! - no provider-specific branches.
//!
//! ============================================================================
//! SCALABILITY CONTRACT
//! ============================================================================
//!
//! There is deliberately no semantic upper bound on:
//!
//! - number of pulse bindings;
//! - number of logical qubits;
//! - number of targets in an explicit binding;
//! - number of pulses in a program;
//! - number of noise models;
//! - number of channels;
//! - number of calibrations;
//! - pulse duration;
//! - sequence length.
//!
//! Actual resource limits belong to explicit resource policies owned by the
//! execution/compiler environment.
//!
//! A large target set is therefore data, not a reason to introduce a constant
//! such as:
//!
//!     MAX_PULSE_TARGETS
//!
//! If an execution environment cannot process a requested binding, it must
//! reject it through an explicit resource/capability policy rather than by
//! changing the semantic definition of this type.
//!
//! "Infinity" therefore means:
//!
//!     no artificial finite semantic machine-size ceiling.
//!
//! It does NOT mean that a finite machine has infinite memory or execution
//! capacity.
//!
//! ============================================================================
//! DETERMINISM CONTRACT
//! ============================================================================
//!
//! This module is completely deterministic.
//!
//! It:
//!
//! - performs no sampling;
//! - owns no RNG;
//! - reads no wall-clock time;
//! - creates no global state;
//! - creates no process-global cache;
//! - does not depend on hash-map iteration order.
//!
//! Explicit target collections preserve caller order.
//!
//! Metadata uses BTreeMap so iteration is deterministic.
//!
//! Any stochastic realization must be performed later by the ZQN noise or
//! simulation subsystem using an explicit deterministic execution context.
//!
//! ============================================================================
//! RESOURCE-SAFETY CONTRACT
//! ============================================================================
//!
//! Semantic validity and resource policy are deliberately separate.
//!
//! This module does not impose a hard-coded machine-size limit.
//!
//! Callers processing untrusted input should apply their ZQN/runtime resource
//! policy before materializing arbitrarily large target collections.
//!
//! This type itself performs only the allocations explicitly requested by the
//! caller.
//!
//! No hidden allocation proportional to machine size is performed.
//!
//! ============================================================================
//! NUMERICAL CONTRACT
//! ============================================================================
//!
//! This module does not store floating-point pulse timing.
//!
//! The canonical Pulse owns its duration representation.
//!
//! Consequently there is no conversion here from floating-point seconds to
//! hardware ticks and no rounding of canonical pulse duration.
//!
//! Noise models that require a physical duration must resolve the canonical
//! Pulse and use its authoritative duration.
//!
//! ============================================================================
//! APPROXIMATION CONTRACT
//! ============================================================================
//!
//! A pulse-noise binding can explicitly declare whether its requested noise
//! semantics are:
//!
//! - exact;
//! - approximate within a caller-declared tolerance;
//! - bounded by a caller-declared error bound;
//! - statistical with a declared confidence;
//! - unsupported.
//!
//! This prevents a downstream target from silently replacing a requested
//! physical noise process with a simpler one.
//!
//! ============================================================================
//! SERIALIZATION CONTRACT
//! ============================================================================
//!
//! This module defines semantic data structures but does not define the
//! external serialization format.
//!
//! The canonical ZQN I/O layer owns schema encoding/versioning.
//!
//! Serialized bindings must preserve:
//!
//! - pulse identity;
//! - phase;
//! - target scope;
//! - source identity;
//! - calibration identity;
//! - approximation contract;
//! - metadata.
//!
//! Rust field layout MUST NOT become the external schema contract.
//!
//! ============================================================================
//! THREAD-SAFETY CONTRACT
//! ============================================================================
//!
//! All structures in this file are immutable after construction.
//!
//! They contain no interior mutability and no global state.
//!
//! The owned data types are therefore naturally safe to share between
//! concurrent readers when their surrounding ZQN registries/context are also
//! thread-safe.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Producers:
//!
//! - canonical pulse lowering;
//! - pulse-aware optimization;
//! - pulse scheduling/lowering;
//! - calibration-aware compilation;
//! - characterization pipelines;
//! - explicit user/program noise specifications.
//!
//! Consumers:
//!
//! - zqn::noise;
//! - zqn::channel;
//! - zqn::calibration;
//! - zqn::simulation;
//! - zqn::target;
//! - zqn::integration::ir;
//! - zqn::integration::scheduling;
//! - zqn::integration::hardware;
//! - benchmarking;
//! - QEC adapters where pulse-level faults are relevant.
//!
//! The binding does not directly invoke those consumers. It provides the
//! stable semantic contract they consume.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::identity::{
    CalibrationId,
    ChannelId,
    PulseId,
};

use crate::quantum::ir::pulse::Pulse;

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::zqn::core::ids::NoiseModelId;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic identifier for the ZQN pulse-noise binding contract.
pub const PULSE_NOISE_SCHEMA_ID: &str = "zamani.quantum.zqn.operations.pulse";

/// Major semantic version.
///
/// Breaking changes to the meaning of this module require a major-version
/// change.
pub const PULSE_NOISE_SCHEMA_MAJOR: u16 = 1;

/// Minor semantic version.
pub const PULSE_NOISE_SCHEMA_MINOR: u16 = 0;

/// Patch semantic version.
pub const PULSE_NOISE_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// Result
// ============================================================================

/// Result type for pulse-noise operations.
pub type PulseNoiseResult<T> = Result<T, PulseNoiseError>;

// ============================================================================
// Approximation
// ============================================================================

/// Explicit accuracy contract for a pulse-noise binding.
///
/// A downstream target MUST NOT silently change an `Exact` request into an
/// approximate representation.
///
/// Approximation/lowering policy belongs to the target layer, but its result
/// must remain explicitly represented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PulseNoiseAccuracy {
    /// Requested semantics must be represented exactly.
    Exact,

    /// Approximation is allowed up to the supplied absolute tolerance.
    Approximate {
        /// Maximum declared approximation tolerance.
        tolerance: f64,
    },

    /// Approximation is allowed when the resulting error is bounded by the
    /// supplied error bound.
    Bounded {
        /// Maximum declared error bound.
        error_bound: f64,
    },

    /// The realization is statistical and must provide at least the declared
    /// confidence level.
    Statistical {
        /// Confidence in the closed interval `(0, 1)`.
        confidence: f64,
    },

    /// No compatible realization is currently declared.
    Unsupported,
}

impl PulseNoiseAccuracy {
    /// Creates an approximate accuracy contract.
    pub fn approximate(tolerance: f64) -> PulseNoiseResult<Self> {
        validate_non_negative_finite(
            tolerance,
            "approximation tolerance",
        )?;

        Ok(Self::Approximate { tolerance })
    }

    /// Creates a bounded accuracy contract.
    pub fn bounded(error_bound: f64) -> PulseNoiseResult<Self> {
        validate_non_negative_finite(
            error_bound,
            "approximation error bound",
        )?;

        Ok(Self::Bounded { error_bound })
    }

    /// Creates a statistical accuracy contract.
    pub fn statistical(confidence: f64) -> PulseNoiseResult<Self> {
        if !confidence.is_finite() {
            return Err(PulseNoiseError::NonFiniteValue {
                field: "confidence",
            });
        }

        if !(0.0 < confidence && confidence < 1.0) {
            return Err(PulseNoiseError::InvalidConfidence {
                confidence,
            });
        }

        Ok(Self::Statistical { confidence })
    }

    /// Validates the accuracy contract.
    pub fn validate(&self) -> PulseNoiseResult<()> {
        match self {
            Self::Exact | Self::Unsupported => Ok(()),

            Self::Approximate { tolerance } => {
                validate_non_negative_finite(
                    *tolerance,
                    "approximation tolerance",
                )
            }

            Self::Bounded { error_bound } => {
                validate_non_negative_finite(
                    *error_bound,
                    "approximation error bound",
                )
            }

            Self::Statistical { confidence } => {
                if !confidence.is_finite() {
                    return Err(PulseNoiseError::NonFiniteValue {
                        field: "confidence",
                    });
                }

                if !(0.0 < *confidence && *confidence < 1.0) {
                    return Err(PulseNoiseError::InvalidConfidence {
                        confidence: *confidence,
                    });
                }

                Ok(())
            }
        }
    }

    /// Returns whether exact semantics were requested.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the contract explicitly allows approximation.
    #[must_use]
    pub const fn permits_approximation(self) -> bool {
        matches!(
            self,
            Self::Approximate { .. }
                | Self::Bounded { .. }
                | Self::Statistical { .. }
        )
    }
}

impl Default for PulseNoiseAccuracy {
    fn default() -> Self {
        Self::Exact
    }
}

// ============================================================================
// Noise phase
// ============================================================================

/// Semantic point at which pulse noise is associated with a canonical pulse.
///
/// These are semantic boundaries, not hardware pulse segments.
///
/// A single canonical pulse may lower into several hardware operations. The
/// target lowering layer decides how the selected phase maps onto those
/// operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PulseNoisePhase {
    /// Noise associated with conditions immediately before the pulse.
    Before,

    /// Noise associated with the pulse while it is active.
    During,

    /// Noise associated with conditions immediately after the pulse.
    After,

    /// Noise associated with the complete semantic pulse operation.
    WholeOperation,
}

impl Default for PulseNoisePhase {
    fn default() -> Self {
        Self::WholeOperation
    }
}

impl PulseNoisePhase {
    /// Returns whether this phase can depend on pulse duration.
    #[must_use]
    pub const fn may_depend_on_duration(self) -> bool {
        matches!(
            self,
            Self::During | Self::WholeOperation
        )
    }

    /// Returns whether this phase occurs before pulse execution.
    #[must_use]
    pub const fn is_before(self) -> bool {
        matches!(self, Self::Before)
    }

    /// Returns whether this phase occurs during pulse execution.
    #[must_use]
    pub const fn is_during(self) -> bool {
        matches!(self, Self::During)
    }

    /// Returns whether this phase occurs after pulse execution.
    #[must_use]
    pub const fn is_after(self) -> bool {
        matches!(self, Self::After)
    }

    /// Returns whether this phase covers the complete semantic operation.
    #[must_use]
    pub const fn is_whole_operation(self) -> bool {
        matches!(self, Self::WholeOperation)
    }
}

// ============================================================================
// Target scope
// ============================================================================

/// Logical scope to which pulse noise applies.
///
/// The target scope deliberately does not contain physical-qubit IDs.
///
/// Logical-to-physical mapping is owned by routing/mapping/hardware layers.
///
/// `Inherited` means the noise applies to the canonical pulse's own target
/// scope.
///
/// `Explicit` is useful when environmental/crosstalk noise applies only to a
/// subset of the resources involved in or affected by the pulse.
///
/// `Global` means the noise model resolves its scope from the execution
/// context rather than from an explicit logical-qubit collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PulseNoiseTarget {
    /// Use the canonical pulse's own target scope.
    Inherited,

    /// Apply to an explicit ordered collection of logical qubits.
    Explicit(Arc<[QubitId]>),

    /// Resolve scope from the execution environment.
    Global,
}

impl PulseNoiseTarget {
    /// Creates an inherited target scope.
    #[must_use]
    pub const fn inherited() -> Self {
        Self::Inherited
    }

    /// Creates a global target scope.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates an explicit logical-qubit target scope.
    ///
    /// The supplied ordering is preserved.
    ///
    /// Duplicate qubits are rejected because this type describes a resource
    /// scope rather than an ordered gate operand list.
    pub fn explicit<I>(
        qubits: I,
    ) -> PulseNoiseResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let qubits: Vec<QubitId> = qubits.into_iter().collect();

        if qubits.is_empty() {
            return Err(PulseNoiseError::EmptyTargetSet);
        }

        for pair in qubits.windows(2) {
            if pair[0] == pair[1] {
                return Err(PulseNoiseError::DuplicateTarget {
                    qubit: pair[0],
                });
            }
        }

        let mut seen = std::collections::BTreeSet::new();

        for qubit in &qubits {
            if !seen.insert(*qubit) {
                return Err(PulseNoiseError::DuplicateTarget {
                    qubit: *qubit,
                });
            }
        }

        Ok(Self::Explicit(Arc::from(qubits)))
    }

    /// Returns whether the scope inherits the canonical pulse target.
    #[must_use]
    pub const fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }

    /// Returns whether the scope is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether the scope is explicitly represented.
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    /// Returns explicitly represented logical qubits.
    ///
    /// Inherited/global scopes return an empty slice because their concrete
    /// scope must be resolved from another semantic object/context.
    #[must_use]
    pub fn explicit_qubits(&self) -> &[QubitId] {
        match self {
            Self::Inherited | Self::Global => &[],
            Self::Explicit(qubits) => qubits.as_ref(),
        }
    }

    /// Returns the number of explicitly represented logical qubits.
    #[must_use]
    pub fn explicit_qubit_count(&self) -> usize {
        self.explicit_qubits().len()
    }

    /// Validates the target scope.
    pub fn validate(&self) -> PulseNoiseResult<()> {
        match self {
            Self::Inherited | Self::Global => Ok(()),

            Self::Explicit(qubits) => {
                if qubits.is_empty() {
                    return Err(PulseNoiseError::EmptyTargetSet);
                }

                let mut seen = std::collections::BTreeSet::new();

                for qubit in qubits.iter() {
                    if !seen.insert(*qubit) {
                        return Err(PulseNoiseError::DuplicateTarget {
                            qubit: *qubit,
                        });
                    }
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Noise source
// ============================================================================

/// Identifies the source of noise associated with a pulse.
///
/// A direct channel is a concrete mathematical noise transformation.
///
/// A noise model is a higher-level context-dependent source that may inspect:
///
/// - pulse kind;
/// - pulse duration;
/// - waveform;
/// - frame;
/// - target;
/// - calibration;
/// - execution time;
/// - target capabilities;
/// - environmental context.
///
/// A calibration reference can accompany either source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PulseNoiseSource {
    /// Direct ZQN channel.
    Channel(ChannelId),

    /// Context-aware ZQN noise model.
    Model(NoiseModelId),
}

impl PulseNoiseSource {
    /// Creates a direct channel source.
    #[must_use]
    pub const fn channel(id: ChannelId) -> Self {
        Self::Channel(id)
    }

    /// Creates a context-aware noise-model source.
    #[must_use]
    pub const fn model(id: NoiseModelId) -> Self {
        Self::Model(id)
    }

    /// Returns the channel identity when this is a direct channel.
    #[must_use]
    pub const fn channel_id(self) -> Option<ChannelId> {
        match self {
            Self::Channel(id) => Some(id),
            Self::Model(_) => None,
        }
    }

    /// Returns the noise-model identity when this is a model source.
    #[must_use]
    pub const fn model_id(self) -> Option<NoiseModelId> {
        match self {
            Self::Channel(_) => None,
            Self::Model(id) => Some(id),
        }
    }

    /// Returns whether this source is a direct channel.
    #[must_use]
    pub const fn is_channel(self) -> bool {
        matches!(self, Self::Channel(_))
    }

    /// Returns whether this source is a model.
    #[must_use]
    pub const fn is_model(self) -> bool {
        matches!(self, Self::Model(_))
    }
}

// ============================================================================
// Metadata
// ============================================================================

/// Deterministic metadata associated with a pulse-noise binding.
///
/// Metadata is descriptive/contextual data.
///
/// It MUST NOT be used by core semantics as an implicit control channel.
///
/// Keys are ordered through `BTreeMap` so canonical iteration is deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PulseNoiseMetadata {
    values: BTreeMap<String, String>,
}

impl PulseNoiseMetadata {
    /// Creates an empty metadata collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a metadata value.
    ///
    /// Empty keys are rejected.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> PulseNoiseResult<Option<String>> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(PulseNoiseError::InvalidMetadataKey);
        }

        let value = value.into();

        if value.trim().is_empty() {
            return Err(PulseNoiseError::EmptyMetadataValue);
        }

        Ok(self.values.insert(key, value))
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// Removes a metadata value.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no metadata entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns deterministic metadata iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| {
                (key.as_str(), value.as_str())
            })
    }
}

// ============================================================================
// Binding errors
// ============================================================================

/// Errors local to the pulse-noise binding boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum PulseNoiseError {
    /// A required pulse identifier was not supplied.
    MissingPulseId,

    /// A pulse ID supplied by a binding did not match the canonical Pulse.
    PulseIdMismatch {
        /// Binding pulse identity.
        binding: PulseId,

        /// Canonical pulse identity.
        canonical: PulseId,
    },

    /// No noise source was supplied.
    MissingNoiseSource,

    /// Explicit target collection was empty.
    EmptyTargetSet,

    /// An explicit target appeared more than once.
    DuplicateTarget {
        /// Duplicated logical resource.
        qubit: QubitId,
    },

    /// A floating-point value was not finite.
    NonFiniteValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A tolerance or bound was negative.
    NegativeAccuracyBound {
        /// Semantic field.
        field: &'static str,
    },

    /// A statistical confidence was outside `(0, 1)`.
    InvalidConfidence {
        /// Supplied confidence.
        confidence: f64,
    },

    /// A metadata key was empty.
    InvalidMetadataKey,

    /// A metadata value was empty.
    EmptyMetadataValue,

    /// An unsupported semantic combination was requested.
    UnsupportedCombination {
        /// Explanation.
        reason: &'static str,
    },

    /// The binding violates a local structural invariant.
    InvalidStructure {
        /// Explanation.
        reason: &'static str,
    },
}

impl fmt::Display for PulseNoiseError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingPulseId => {
                formatter.write_str(
                    "pulse-noise binding requires a canonical PulseId",
                )
            }

            Self::PulseIdMismatch {
                binding,
                canonical,
            } => {
                write!(
                    formatter,
                    "pulse-noise binding targets pulse {binding}, \
                     but canonical pulse is {canonical}"
                )
            }

            Self::MissingNoiseSource => {
                formatter.write_str(
                    "pulse-noise binding requires a noise source",
                )
            }

            Self::EmptyTargetSet => {
                formatter.write_str(
                    "explicit pulse-noise target set must not be empty",
                )
            }

            Self::DuplicateTarget { qubit } => {
                write!(
                    formatter,
                    "duplicate logical pulse-noise target {qubit}"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "{field} must be finite"
                )
            }

            Self::NegativeAccuracyBound { field } => {
                write!(
                    formatter,
                    "{field} must not be negative"
                )
            }

            Self::InvalidConfidence { confidence } => {
                write!(
                    formatter,
                    "confidence {confidence} must be strictly between 0 and 1"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str(
                    "pulse-noise metadata key must not be empty",
                )
            }

            Self::EmptyMetadataValue => {
                formatter.write_str(
                    "pulse-noise metadata value must not be empty",
                )
            }

            Self::UnsupportedCombination { reason } => {
                write!(
                    formatter,
                    "unsupported pulse-noise combination: {reason}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid pulse-noise structure: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for PulseNoiseError {}

// ============================================================================
// Pulse-noise binding
// ============================================================================

/// Immutable semantic binding between a canonical pulse and ZQN noise.
///
/// This is the primary public type of this module.
///
/// It deliberately stores the canonical `PulseId` rather than copying the
/// complete canonical `Pulse`.
///
/// This gives ZQN one authoritative pulse definition:
///
///     quantum::ir::pulse::Pulse
///
/// while this module owns only the additional noise semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct PulseNoiseBinding {
    pulse_id: PulseId,
    phase: PulseNoisePhase,
    target: PulseNoiseTarget,
    source: PulseNoiseSource,
    calibration: Option<CalibrationId>,
    accuracy: PulseNoiseAccuracy,
    metadata: PulseNoiseMetadata,
}

impl PulseNoiseBinding {
    /// Creates a pulse-noise binding.
    ///
    /// The canonical pulse must be supplied separately when the binding is
    /// validated or executed.
    pub fn new(
        pulse_id: PulseId,
        source: PulseNoiseSource,
    ) -> PulseNoiseResult<Self> {
        let binding = Self {
            pulse_id,
            phase: PulseNoisePhase::WholeOperation,
            target: PulseNoiseTarget::Inherited,
            source,
            calibration: None,
            accuracy: PulseNoiseAccuracy::Exact,
            metadata: PulseNoiseMetadata::new(),
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Creates a binding directly from a canonical Pulse.
    ///
    /// Only its identity is retained. The canonical Pulse remains the source
    /// of truth for pulse semantics.
    pub fn from_pulse(
        pulse: &Pulse,
        source: PulseNoiseSource,
    ) -> PulseNoiseResult<Self> {
        Self::new(pulse.id(), source)
    }

    /// Sets the semantic noise phase.
    #[must_use]
    pub const fn with_phase(
        mut self,
        phase: PulseNoisePhase,
    ) -> Self {
        self.phase = phase;
        self
    }

    /// Sets the target scope.
    pub fn with_target(
        mut self,
        target: PulseNoiseTarget,
    ) -> PulseNoiseResult<Self> {
        target.validate()?;
        self.target = target;
        self.validate()?;
        Ok(self)
    }

    /// Sets an explicit calibration identity.
    #[must_use]
    pub const fn with_calibration(
        mut self,
        calibration: CalibrationId,
    ) -> Self {
        self.calibration = Some(calibration);
        self
    }

    /// Sets the accuracy contract.
    pub fn with_accuracy(
        mut self,
        accuracy: PulseNoiseAccuracy,
    ) -> PulseNoiseResult<Self> {
        accuracy.validate()?;
        self.accuracy = accuracy;
        self.validate()?;
        Ok(self)
    }

    /// Inserts deterministic metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> PulseNoiseResult<Self> {
        self.metadata.insert(key, value)?;
        self.validate()?;
        Ok(self)
    }

    /// Returns the canonical PulseId.
    #[must_use]
    pub const fn pulse_id(&self) -> PulseId {
        self.pulse_id
    }

    /// Returns the semantic phase.
    #[must_use]
    pub const fn phase(&self) -> PulseNoisePhase {
        self.phase
    }

    /// Returns the target scope.
    #[must_use]
    pub fn target(&self) -> &PulseNoiseTarget {
        &self.target
    }

    /// Returns the noise source.
    #[must_use]
    pub const fn source(&self) -> PulseNoiseSource {
        self.source
    }

    /// Returns the optional calibration identity.
    #[must_use]
    pub const fn calibration(&self) -> Option<CalibrationId> {
        self.calibration
    }

    /// Returns the explicit accuracy contract.
    #[must_use]
    pub const fn accuracy(&self) -> PulseNoiseAccuracy {
        self.accuracy
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub const fn metadata(&self) -> &PulseNoiseMetadata {
        &self.metadata
    }

    /// Validates the local binding invariants.
    pub fn validate(&self) -> PulseNoiseResult<()> {
        self.target.validate()?;
        self.accuracy.validate()?;

        if self.source.is_model() && self.source.model_id().is_none() {
            return Err(PulseNoiseError::InvalidStructure {
                reason: "model source must contain a NoiseModelId",
            });
        }

        if self.source.is_channel() && self.source.channel_id().is_none() {
            return Err(PulseNoiseError::InvalidStructure {
                reason: "channel source must contain a ChannelId",
            });
        }

        Ok(())
    }

    /// Validates that this binding refers to the supplied canonical Pulse.
    ///
    /// This intentionally checks identity only.
    ///
    /// Pulse semantics such as duration, target, waveform, frame and kind are
    /// resolved from the canonical Pulse by the consuming integration layer.
    pub fn validate_against(
        &self,
        pulse: &Pulse,
    ) -> PulseNoiseResult<()> {
        if self.pulse_id != pulse.id() {
            return Err(PulseNoiseError::PulseIdMismatch {
                binding: self.pulse_id,
                canonical: pulse.id(),
            });
        }

        self.validate()
    }

    /// Returns a deterministic view of explicit target resources.
    ///
    /// `Inherited` and `Global` return an empty slice because their actual
    /// scope must be resolved by the integration/execution layer.
    #[must_use]
    pub fn explicit_qubits(&self) -> &[QubitId] {
        self.target.explicit_qubits()
    }

    /// Returns whether the binding can be represented exactly.
    #[must_use]
    pub const fn requires_exact_realization(&self) -> bool {
        self.accuracy.is_exact()
    }

    /// Returns whether the binding explicitly permits approximation.
    #[must_use]
    pub const fn permits_approximation(&self) -> bool {
        self.accuracy.permits_approximation()
    }
}

// ============================================================================
// Convenience constructors
// ============================================================================

impl PulseNoiseBinding {
    /// Creates channel noise covering the whole canonical pulse operation.
    pub fn channel(
        pulse: &Pulse,
        channel: ChannelId,
    ) -> PulseNoiseResult<Self> {
        Self::from_pulse(
            pulse,
            PulseNoiseSource::channel(channel),
        )
    }

    /// Creates model-driven noise covering the whole canonical pulse.
    pub fn model(
        pulse: &Pulse,
        model: NoiseModelId,
    ) -> PulseNoiseResult<Self> {
        Self::from_pulse(
            pulse,
            PulseNoiseSource::model(model),
        )
    }

    /// Creates channel noise explicitly scoped to logical qubits.
    pub fn channel_on_qubits<I>(
        pulse: &Pulse,
        channel: ChannelId,
        qubits: I,
    ) -> PulseNoiseResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let target = PulseNoiseTarget::explicit(qubits)?;

        Self::channel(pulse, channel)?
            .with_target(target)
    }

    /// Creates model-driven noise explicitly scoped to logical qubits.
    pub fn model_on_qubits<I>(
        pulse: &Pulse,
        model: NoiseModelId,
        qubits: I,
    ) -> PulseNoiseResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let target = PulseNoiseTarget::explicit(qubits)?;

        Self::model(pulse, model)?
            .with_target(target)
    }
}

// ============================================================================
// Canonical pulse integration view
// ============================================================================

/// Read-only integration view pairing a canonical Pulse with its ZQN binding.
///
/// This type does not own a second copy of the pulse.
///
/// It is useful at compiler/runtime boundaries where both objects must be
/// passed together.
#[derive(Debug, Clone, Copy)]
pub struct PulseNoiseView<'a> {
    pulse: &'a Pulse,
    binding: &'a PulseNoiseBinding,
}

impl<'a> PulseNoiseView<'a> {
    /// Creates a validated integration view.
    pub fn new(
        pulse: &'a Pulse,
        binding: &'a PulseNoiseBinding,
    ) -> PulseNoiseResult<Self> {
        binding.validate_against(pulse)?;

        Ok(Self {
            pulse,
            binding,
        })
    }

    /// Returns the canonical pulse.
    #[must_use]
    pub const fn pulse(&self) -> &'a Pulse {
        self.pulse
    }

    /// Returns the ZQN binding.
    #[must_use]
    pub const fn binding(&self) -> &'a PulseNoiseBinding {
        self.binding
    }

    /// Returns the canonical PulseId.
    #[must_use]
    pub const fn pulse_id(&self) -> PulseId {
        self.binding.pulse_id()
    }

    /// Returns the noise phase.
    #[must_use]
    pub const fn phase(&self) -> PulseNoisePhase {
        self.binding.phase()
    }

    /// Returns the noise source.
    #[must_use]
    pub const fn source(&self) -> PulseNoiseSource {
        self.binding.source()
    }

    /// Returns the optional calibration identity.
    #[must_use]
    pub const fn calibration(&self) -> Option<CalibrationId> {
        self.binding.calibration()
    }

    /// Returns the requested accuracy contract.
    #[must_use]
    pub const fn accuracy(&self) -> PulseNoiseAccuracy {
        self.binding.accuracy()
    }

    /// Returns the noise target scope.
    #[must_use]
    pub fn target(&self) -> &PulseNoiseTarget {
        self.binding.target()
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> PulseNoiseResult<()> {
    if !value.is_finite() {
        return Err(PulseNoiseError::NonFiniteValue {
            field,
        });
    }

    if value < 0.0 {
        return Err(PulseNoiseError::NegativeAccuracyBound {
            field,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::{
        ChannelId,
        NoiseModelId,
        PulseId,
    };

    use crate::quantum::ir::pulse::{
        Pulse,
        PulseKind,
        PulseTarget,
    };

    use crate::quantum::ir::qubit::QubitId;

    fn pulse() -> Pulse {
        Pulse::new(
            PulseId::new(1),
            PulseKind::Barrier,
        )
    }

    #[test]
    fn module_is_no_unsafe() {
        let binding = PulseNoiseBinding::channel(
            &pulse(),
            ChannelId::new(1),
        )
        .expect("channel binding must be valid");

        assert_eq!(
            binding.pulse_id(),
            PulseId::new(1),
        );
    }

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            PULSE_NOISE_SCHEMA_ID,
            "zamani.quantum.zqn.operations.pulse",
        );

        assert_eq!(
            PULSE_NOISE_SCHEMA_MAJOR,
            1,
        );

        assert_eq!(
            PULSE_NOISE_SCHEMA_MINOR,
            0,
        );

        assert_eq!(
            PULSE_NOISE_SCHEMA_PATCH,
            0,
        );
    }

    #[test]
    fn channel_source_is_distinguishable() {
        let source =
            PulseNoiseSource::channel(
                ChannelId::new(7),
            );

        assert!(source.is_channel());
        assert!(!source.is_model());

        assert_eq!(
            source.channel_id(),
            Some(ChannelId::new(7)),
        );

        assert_eq!(
            source.model_id(),
            None,
        );
    }

    #[test]
    fn model_source_is_distinguishable() {
        let source =
            PulseNoiseSource::model(
                NoiseModelId::new(7),
            );

        assert!(source.is_model());
        assert!(!source.is_channel());

        assert_eq!(
            source.model_id(),
            Some(NoiseModelId::new(7)),
        );

        assert_eq!(
            source.channel_id(),
            None,
        );
    }

    #[test]
    fn inherited_target_is_default() {
        let binding = PulseNoiseBinding::channel(
            &pulse(),
            ChannelId::new(1),
        )
        .expect("binding must be valid");

        assert!(
            binding.target().is_inherited()
        );

        assert_eq!(
            binding.explicit_qubits(),
            &[],
        );
    }

    #[test]
    fn explicit_targets_use_canonical_qubit_ids() {
        let q0 = QubitId::new(0);
        let q3 = QubitId::new(3);

        let target =
            PulseNoiseTarget::explicit(
                [q0, q3],
            )
            .expect("unique targets must be valid");

        assert_eq!(
            target.explicit_qubits(),
            &[q0, q3],
        );
    }

    #[test]
    fn explicit_target_order_is_preserved() {
        let q4 = QubitId::new(4);
        let q1 = QubitId::new(1);

        let target =
            PulseNoiseTarget::explicit(
                [q4, q1],
            )
            .expect("unique targets must be valid");

        assert_eq!(
            target.explicit_qubits(),
            &[q4, q1],
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let q0 = QubitId::new(0);

        let result =
            PulseNoiseTarget::explicit(
                [q0, q0],
            );

        assert_eq!(
            result,
            Err(
                PulseNoiseError::DuplicateTarget {
                    qubit: q0,
                },
            ),
        );
    }

    #[test]
    fn empty_explicit_targets_are_rejected() {
        let result =
            PulseNoiseTarget::explicit(
                std::iter::empty::<QubitId>(),
            );

        assert_eq!(
            result,
            Err(
                PulseNoiseError::EmptyTargetSet,
            ),
        );
    }

    #[test]
    fn canonical_pulse_identity_is_validated() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel(
                &canonical,
                ChannelId::new(2),
            )
            .expect("binding must be valid");

        assert!(
            binding
                .validate_against(&canonical)
                .is_ok()
        );
    }

    #[test]
    fn mismatched_pulse_identity_is_rejected() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::new(
                PulseId::new(999),
                PulseNoiseSource::channel(
                    ChannelId::new(2),
                ),
            )
            .expect("binding itself is structurally valid");

        let result =
            binding.validate_against(
                &canonical,
            );

        assert!(matches!(
            result,
            Err(
                PulseNoiseError::PulseIdMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn default_phase_is_whole_operation() {
        let binding =
            PulseNoiseBinding::channel(
                &pulse(),
                ChannelId::new(1),
            )
            .expect("binding must be valid");

        assert_eq!(
            binding.phase(),
            PulseNoisePhase::WholeOperation,
        );
    }

    #[test]
    fn phase_can_be_changed_without_mutating_pulse() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel(
                &canonical,
                ChannelId::new(1),
            )
            .expect("binding must be valid")
            .with_phase(
                PulseNoisePhase::During,
            );

        assert_eq!(
            binding.phase(),
            PulseNoisePhase::During,
        );

        assert_eq!(
            canonical.id(),
            PulseId::new(1),
        );
    }

    #[test]
    fn exact_accuracy_is_default() {
        let binding =
            PulseNoiseBinding::channel(
                &pulse(),
                ChannelId::new(1),
            )
            .expect("binding must be valid");

        assert!(
            binding
                .accuracy()
                .is_exact()
        );

        assert!(
            binding
                .requires_exact_realization()
        );

        assert!(
            !binding
                .permits_approximation()
        );
    }

    #[test]
    fn negative_tolerance_is_rejected() {
        let result =
            PulseNoiseAccuracy::approximate(
                -1.0,
            );

        assert!(matches!(
            result,
            Err(
                PulseNoiseError::NegativeAccuracyBound {
                    ..
                }
            )
        ));
    }

    #[test]
    fn nan_tolerance_is_rejected() {
        let result =
            PulseNoiseAccuracy::approximate(
                f64::NAN,
            );

        assert!(matches!(
            result,
            Err(
                PulseNoiseError::NonFiniteValue {
                    ..
                }
            )
        ));
    }

    #[test]
    fn infinite_tolerance_is_rejected() {
        let result =
            PulseNoiseAccuracy::approximate(
                f64::INFINITY,
            );

        assert!(matches!(
            result,
            Err(
                PulseNoiseError::NonFiniteValue {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_tolerance_is_allowed() {
        let result =
            PulseNoiseAccuracy::approximate(
                0.0,
            )
            .expect(
                "zero tolerance is finite and non-negative",
            );

        assert_eq!(
            result,
            PulseNoiseAccuracy::Approximate {
                tolerance: 0.0,
            },
        );
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        assert!(
            PulseNoiseAccuracy::statistical(
                0.0,
            )
            .is_err()
        );

        assert!(
            PulseNoiseAccuracy::statistical(
                1.0,
            )
            .is_err()
        );

        assert!(
            PulseNoiseAccuracy::statistical(
                f64::NAN,
            )
            .is_err()
        );
    }

    #[test]
    fn valid_statistical_accuracy_is_accepted() {
        let accuracy =
            PulseNoiseAccuracy::statistical(
                0.99,
            )
            .expect(
                "0.99 is a valid confidence",
            );

        assert!(
            accuracy
                .permits_approximation()
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata =
            PulseNoiseMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata must be valid");

        metadata
            .insert("a", "first")
            .expect("metadata must be valid");

        let keys: Vec<&str> =
            metadata
                .iter()
                .map(|(key, _)| key)
                .collect();

        assert_eq!(
            keys,
            vec!["a", "z"],
        );
    }

    #[test]
    fn empty_metadata_key_is_rejected() {
        let mut metadata =
            PulseNoiseMetadata::new();

        let result =
            metadata.insert("   ", "value");

        assert_eq!(
            result,
            Err(
                PulseNoiseError::InvalidMetadataKey,
            ),
        );
    }

    #[test]
    fn empty_metadata_value_is_rejected() {
        let mut metadata =
            PulseNoiseMetadata::new();

        let result =
            metadata.insert("key", "   ");

        assert_eq!(
            result,
            Err(
                PulseNoiseError::EmptyMetadataValue,
            ),
        );
    }

    #[test]
    fn metadata_replacement_is_explicit() {
        let mut metadata =
            PulseNoiseMetadata::new();

        assert_eq!(
            metadata
                .insert("key", "first")
                .expect("first insert"),
            None,
        );

        assert_eq!(
            metadata
                .insert("key", "second")
                .expect("replacement"),
            Some("first".to_owned()),
        );

        assert_eq!(
            metadata.get("key"),
            Some("second"),
        );
    }

    #[test]
    fn binding_can_use_explicit_targets() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel_on_qubits(
                &canonical,
                ChannelId::new(3),
                [
                    QubitId::new(1),
                    QubitId::new(4),
                ],
            )
            .expect(
                "explicit target binding must be valid",
            );

        assert_eq!(
            binding.explicit_qubits(),
            &[
                QubitId::new(1),
                QubitId::new(4),
            ],
        );
    }

    #[test]
    fn binding_can_use_model_noise() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::model(
                &canonical,
                NoiseModelId::new(5),
            )
            .expect(
                "model binding must be valid",
            );

        assert!(binding.source().is_model());
    }

    #[test]
    fn binding_can_reference_calibration() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel(
                &canonical,
                ChannelId::new(3),
            )
            .expect("binding must be valid")
            .with_calibration(
                CalibrationId::new(9),
            );

        assert_eq!(
            binding.calibration(),
            Some(CalibrationId::new(9)),
        );
    }

    #[test]
    fn binding_can_declare_approximation() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel(
                &canonical,
                ChannelId::new(3),
            )
            .expect("binding must be valid")
            .with_accuracy(
                PulseNoiseAccuracy::approximate(
                    1.0e-9,
                )
                .expect(
                    "positive finite tolerance",
                ),
            )
            .expect(
                "accuracy contract must be valid",
            );

        assert!(
            binding.permits_approximation()
        );

        assert!(
            !binding.requires_exact_realization()
        );
    }

    #[test]
    fn integration_view_validates_identity() {
        let canonical = pulse();

        let binding =
            PulseNoiseBinding::channel(
                &canonical,
                ChannelId::new(3),
            )
            .expect("binding must be valid");

        let view =
            PulseNoiseView::new(
                &canonical,
                &binding,
            )
            .expect(
                "matching pulse and binding must validate",
            );

        assert_eq!(
            view.pulse_id(),
            canonical.id(),
        );

        assert_eq!(
            view.source().channel_id(),
            Some(ChannelId::new(3)),
        );
    }

    #[test]
    fn pulse_target_can_remain_independent() {
        let target =
            PulseNoiseTarget::global();

        assert!(target.is_global());

        let canonical_target =
            PulseTarget::global();

        assert!(
            canonical_target.is_global()
        );
    }
}