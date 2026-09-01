//! Zamani Quantum Noise (ZQN) — Characterization Observations.
//!
//! # Ownership
//!
//! This file owns the canonical representation of raw and minimally
//! structured observations produced by ZQN characterization experiments.
//!
//! It owns:
//!
//! - observation payloads;
//! - measurement outcomes;
//! - outcome histograms;
//! - per-shot observations;
//! - scalar and complex samples;
//! - observation resource scope;
//! - observation timing supplied by the executor;
//! - observation provenance references;
//! - observation contracts;
//! - observation validation;
//! - deterministic observation ordering;
//! - bounded observation batches;
//! - streaming observation sinks;
//! - explicit observation resource limits.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - characterization protocols;
//! - experiment generation;
//! - circuit generation;
//! - canonical quantum IR;
//! - quantum state storage;
//! - quantum channels;
//! - noise models;
//! - statistical estimators;
//! - confidence intervals;
//! - Bayesian inference;
//! - tomography reconstruction;
//! - calibration mathematics;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - hardware communication;
//! - simulator implementation;
//! - random-number generation;
//! - persistence;
//! - serialization wire formats;
//! - vendor APIs.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! CharacterizationProtocol
//!          |
//!          v
//!     ExperimentPlan
//!          |
//!          v
//!   generator/executor
//!          |
//!          v
//!   +-----------------------+
//!   |   Observation         |
//!   |   this module         |
//!   +-----------+-----------+
//!               |
//!        +------+-------+
//!        |              |
//!        v              v
//!    estimator       persistence
//!        |
//!        v
//! characterization result
//!        |
//!        v
//! ZQN noise / calibration
//! ```
//!
//! # Core principle
//!
//! An observation is evidence, not an estimate.
//!
//! For example, a histogram containing:
//!
//! ```text
//! 00 -> 497
//! 01 -> 503
//! ```
//!
//! is raw evidence. It is NOT automatically a probability estimate,
//! confidence interval, fidelity, error rate, or noise parameter.
//!
//! Those interpretations belong to `estimator.rs` and related analysis
//! modules.
//!
//! # Canonical identity boundary
//!
//! ZQN object identities are owned by:
//!
//! ```text
//! crate::quantum::zqn::core::ids
//! ```
//!
//! Quantum resource identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Therefore this file MUST NOT define:
//!
//! ```text
//! ObservationId
//! ExperimentId
//! CharacterizationId
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! It uses the canonical definitions instead.
//!
//! # Scalability
//!
//! There is no semantic maximum for:
//!
//! - number of observations;
//! - number of shots;
//! - number of outcome symbols;
//! - number of distinct outcomes;
//! - number of characterized resources;
//! - number of experiments;
//! - number of repetitions;
//! - characterization duration;
//! - observation payload size.
//!
//! An in-memory `Vec` or `BTreeMap` is a materialization choice, not a
//! semantic machine-size limit.
//!
//! Large workloads MUST be capable of using:
//!
//! ```text
//! executor
//!     |
//!     v
//! ObservationSink
//!     |
//!     +--> bounded batches
//!     +--> persistent store
//!     +--> distributed stream
//!     +--> online estimator
//! ```
//!
//! This permits the same observation contract to operate from tiny devices
//! to arbitrarily large finite workloads, subject only to explicit resource
//! availability and policy.
//!
//! # Determinism
//!
//! This module:
//!
//! - does not create random numbers;
//! - does not use a global RNG;
//! - does not use global mutable state;
//! - does not read the system clock;
//! - does not generate identifiers automatically;
//! - does not depend on hash-map iteration order.
//!
//! Timestamps, seeds, execution identities and randomness provenance are
//! explicit caller-supplied data.
//!
//! `BTreeMap` is used where deterministic ordering matters.
//!
//! # Numerical safety
//!
//! Invalid floating-point observations are rejected.
//!
//! In particular:
//!
//! ```text
//! NaN
//! +∞
//! -∞
//! ```
//!
//! are never silently converted, clamped, normalized, or discarded.
//!
//! Integer arithmetic uses checked operations where overflow is possible.
//!
//! # Resource safety
//!
//! Observation limits are explicit policy.
//!
//! They are NOT architectural limits.
//!
//! ```text
//! ObservationLimits::default()
//!         |
//!         +--> no ZQN-imposed limit
//!
//! ObservationLimits { ... }
//!         |
//!         +--> caller-selected resource governance
//! ```
//!
//! This prevents accidental allocation bombs while preserving scalability.
//!
//! # Serialization
//!
//! This file defines semantic data structures, not a wire format.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! quantum::zqn::io
//! ```
//!
//! Therefore changing Rust struct layout does not implicitly become a
//! serialization compatibility change.
//!
//! # Integration
//!
//! Protocol:
//!
//! ```text
//! characterization::protocol
//!          |
//!          | ObservationRequirements
//!          v
//! characterization::observation
//! ```
//!
//! Execution:
//!
//! ```text
//! runtime / simulator / hardware adapter
//!          |
//!          | Observation
//!          v
//! characterization::observation
//! ```
//!
//! Estimation:
//!
//! ```text
//! Observation
//!      |
//!      v
//! characterization::estimator
//! ```
//!
//! Calibration:
//!
//! ```text
//! Observation
//!      |
//!      v
//! characterization result
//!      |
//!      v
//! calibration
//! ```
//!
//! Benchmarking:
//!
//! ```text
//! Observation
//!      |
//!      v
//! benchmarking
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler
//! enforced for this file.

#![forbid(unsafe_code)]

use core::fmt;
use std::collections::BTreeMap;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::characterization::protocol::{
    CharacterizationRequirements, ObservationRequirements, ProtocolError, ProtocolId,
};
use crate::quantum::zqn::core::ids::{
    CalibrationId, CharacterizationId, ExperimentId, ObservationId, ZqnIdValue,
};

/// Result type for observation operations.
pub type ObservationResult<T> = Result<T, ObservationError>;

/// Errors produced by the observation layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObservationError {
    /// Observation structure is invalid.
    InvalidObservation,

    /// A referenced identity is invalid.
    InvalidIdentifier,

    /// A floating-point observation is non-finite.
    InvalidNumericValue,

    /// A count is zero or otherwise invalid.
    InvalidCount,

    /// Timestamp/timing information is invalid.
    InvalidTiming,

    /// Resource scope is invalid.
    InvalidResourceScope,

    /// Measurement outcome is invalid.
    InvalidOutcome,

    /// Two logically identical outcome entries were supplied where
    /// duplicates are forbidden.
    DuplicateOutcome,

    /// Integer arithmetic overflow occurred.
    Overflow,

    /// Metadata fields disagree.
    InconsistentMetadata,

    /// Observation is intentionally or accidentally incomplete.
    IncompleteObservation,

    /// Payload cannot be interpreted under its declared contract.
    UnsupportedPayload,

    /// Explicit resource governance was exceeded.
    ResourceLimitExceeded,

    /// A protocol-level contract error was encountered.
    Protocol(ProtocolError),
}

impl fmt::Display for ObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidObservation => "invalid characterization observation",
            Self::InvalidIdentifier => "invalid observation identifier",
            Self::InvalidNumericValue => "non-finite observation value",
            Self::InvalidCount => "invalid observation count",
            Self::InvalidTiming => "invalid observation timing",
            Self::InvalidResourceScope => "invalid observation resource scope",
            Self::InvalidOutcome => "invalid measurement outcome",
            Self::DuplicateOutcome => "duplicate measurement outcome",
            Self::Overflow => "observation arithmetic overflow",
            Self::InconsistentMetadata => "inconsistent observation metadata",
            Self::IncompleteObservation => "observation is incomplete",
            Self::UnsupportedPayload => "unsupported observation payload",
            Self::ResourceLimitExceeded => "observation resource limit exceeded",
            Self::Protocol(_) => "invalid characterization protocol contract",
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ObservationError {}

impl From<ProtocolError> for ObservationError {
    fn from(error: ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

// ============================================================================
// Schema identity
// ============================================================================

/// Stable semantic schema identifier.
pub const OBSERVATION_SCHEMA_ID: &str =
    "zamani.quantum.zqn.characterization.observation";

/// Semantic version of the observation model defined by this file.
///
/// This is NOT the global ZQN version and is NOT the serialization format
/// version.
pub const OBSERVATION_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// Resource identity
// ============================================================================

/// A quantum resource whose behavior is represented by an observation.
///
/// Qubit identities are deliberately taken from the canonical quantum IR.
///
/// The `ZqnResource` and `Named` variants allow characterization of future
/// resource modalities without requiring this file to be edited for every
/// new quantum technology.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObservedResource {
    /// Canonical logical qubit identity.
    LogicalQubit(QubitId),

    /// Canonical physical qubit identity.
    PhysicalQubit(PhysicalQubitId),

    /// Opaque ZQN-domain resource identity.
    ZqnResource(ZqnIdValue),

    /// Named external resource whose semantic interpretation is supplied by
    /// the target/integration layer.
    Named(String),
}

impl ObservedResource {
    /// Validates the resource identity without asserting that the resource
    /// actually exists on a target.
    pub fn validate(&self) -> ObservationResult<()> {
        if let Self::Named(name) = self {
            if name.trim().is_empty() {
                return Err(ObservationError::InvalidResourceScope);
            }
        }

        Ok(())
    }
}

/// Resource scope represented by an observation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObservationScope {
    /// Explicitly represented resources.
    ///
    /// This collection is allowed to be empty when `aggregate` is true.
    pub resources: Vec<ObservedResource>,

    /// Indicates that the observation describes an aggregate target-defined
    /// scope rather than enumerating every resource.
    pub aggregate: bool,
}

impl ObservationScope {
    /// Validates the scope.
    pub fn validate(&self) -> ObservationResult<()> {
        for resource in &self.resources {
            resource.validate()?;
        }

        Ok(())
    }

    /// Returns whether the scope has no explicit or aggregate meaning.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty() && !self.aggregate
    }
}

// ============================================================================
// Timing
// ============================================================================

/// Explicit execution timing attached to an observation.
///
/// Time is supplied by the execution environment. This type never reads the
/// system clock.
///
/// The clock domain is an opaque string because different targets may expose
/// different synchronized or local clocks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationTiming {
    /// Clock-domain identifier.
    pub clock_domain: String,

    /// Start timestamp in the declared clock domain.
    pub start_nanos: u128,

    /// Duration in nanoseconds.
    pub duration_nanos: u128,
}

impl ObservationTiming {
    /// Creates timing metadata.
    pub fn new<S>(
        clock_domain: S,
        start_nanos: u128,
        duration_nanos: u128,
    ) -> ObservationResult<Self>
    where
        S: Into<String>,
    {
        let clock_domain = clock_domain.into();

        if clock_domain.trim().is_empty() {
            return Err(ObservationError::InvalidTiming);
        }

        Ok(Self {
            clock_domain,
            start_nanos,
            duration_nanos,
        })
    }

    /// Validates timing metadata.
    pub fn validate(&self) -> ObservationResult<()> {
        if self.clock_domain.trim().is_empty() {
            return Err(ObservationError::InvalidTiming);
        }

        Ok(())
    }
}

// ============================================================================
// Measurement outcomes
// ============================================================================

/// Representation-independent discrete measurement outcome.
///
/// A `Vec<u8>` is used instead of `u64`, `u128`, or a fixed-size bitset so
/// outcome width is data rather than an architectural limit.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscreteOutcome {
    /// Symbols/bits in the declared measurement order.
    pub symbols: Vec<u8>,
}

impl DiscreteOutcome {
    /// Creates a non-empty outcome.
    pub fn new(symbols: Vec<u8>) -> ObservationResult<Self> {
        if symbols.is_empty() {
            return Err(ObservationError::InvalidOutcome);
        }

        Ok(Self { symbols })
    }

    /// Number of outcome symbols.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Whether the outcome contains no symbols.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

/// One counted measurement outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutcomeCount {
    pub outcome: DiscreteOutcome,
    pub count: u64,
}

impl OutcomeCount {
    pub fn validate(&self) -> ObservationResult<()> {
        if self.count == 0 {
            return Err(ObservationError::InvalidCount);
        }

        Ok(())
    }
}

/// Deterministically ordered measurement histogram.
///
/// The map key is the complete outcome, not an integer encoding of the
/// outcome. This avoids hidden register-width assumptions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutcomeHistogram {
    outcomes: BTreeMap<DiscreteOutcome, u64>,
}

impl OutcomeHistogram {
    /// Creates an empty histogram.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or accumulates an outcome count.
    pub fn insert(
        &mut self,
        outcome: DiscreteOutcome,
        count: u64,
    ) -> ObservationResult<()> {
        if count == 0 {
            return Err(ObservationError::InvalidCount);
        }

        let previous = self.outcomes.get(&outcome).copied().unwrap_or(0);

        let updated = previous
            .checked_add(count)
            .ok_or(ObservationError::Overflow)?;

        self.outcomes.insert(outcome, updated);

        Ok(())
    }

    /// Adds multiple counts.
    pub fn extend<I>(&mut self, entries: I) -> ObservationResult<()>
    where
        I: IntoIterator<Item = OutcomeCount>,
    {
        for entry in entries {
            entry.validate()?;
            self.insert(entry.outcome, entry.count)?;
        }

        Ok(())
    }

    /// Returns a count for an outcome.
    pub fn get(&self, outcome: &DiscreteOutcome) -> Option<u64> {
        self.outcomes.get(outcome).copied()
    }

    /// Number of distinct outcomes.
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    /// Whether no outcomes were recorded.
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Total number of recorded shots.
    pub fn total_count(&self) -> ObservationResult<u64> {
        self.outcomes.values().try_fold(0_u64, |total, count| {
            total
                .checked_add(*count)
                .ok_or(ObservationError::Overflow)
        })
    }

    /// Deterministic iterator.
    pub fn iter(&self) -> impl Iterator<Item = (&DiscreteOutcome, &u64)> {
        self.outcomes.iter()
    }

    /// Consumes the histogram into deterministic entries.
    pub fn into_entries(self) -> Vec<OutcomeCount> {
        self.outcomes
            .into_iter()
            .map(|(outcome, count)| OutcomeCount { outcome, count })
            .collect()
    }
}

// ============================================================================
// Numeric observations
// ============================================================================

/// Finite scalar observation.
///
/// This is an observed value, not a statistical estimate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalarSample {
    pub value: f64,
}

impl ScalarSample {
    pub fn new(value: f64) -> ObservationResult<Self> {
        if !value.is_finite() {
            return Err(ObservationError::InvalidNumericValue);
        }

        Ok(Self { value })
    }
}

/// Finite complex observation.
///
/// Kept dependency-free instead of requiring a complex-number crate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComplexSample {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexSample {
    pub fn new(real: f64, imaginary: f64) -> ObservationResult<Self> {
        if !real.is_finite() || !imaginary.is_finite() {
            return Err(ObservationError::InvalidNumericValue);
        }

        Ok(Self { real, imaginary })
    }
}

// ============================================================================
// Per-shot observations
// ============================================================================

/// One raw shot-level payload.
#[derive(Clone, Debug, PartialEq)]
pub enum ShotPayload {
    /// Discrete measurement result.
    Discrete(DiscreteOutcome),

    /// Scalar acquisition.
    Scalar(ScalarSample),

    /// Complex acquisition.
    Complex(ComplexSample),

    /// Target-specific data whose interpretation is declared by the
    /// observation contract.
    Opaque(Vec<u8>),
}

impl ShotPayload {
    pub fn validate(&self) -> ObservationResult<()> {
        match self {
            Self::Discrete(outcome) => {
                if outcome.is_empty() {
                    return Err(ObservationError::InvalidOutcome);
                }
            }

            Self::Scalar(sample) => {
                if !sample.value.is_finite() {
                    return Err(ObservationError::InvalidNumericValue);
                }
            }

            Self::Complex(sample) => {
                if !sample.real.is_finite()
                    || !sample.imaginary.is_finite()
                {
                    return Err(ObservationError::InvalidNumericValue);
                }
            }

            Self::Opaque(_) => {}
        }

        Ok(())
    }
}

/// A bounded batch of per-shot observations.
///
/// Batching is an implementation/resource-management mechanism. It does not
/// impose a semantic maximum on the total number of shots.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShotBatch {
    /// Index of the first shot represented by this batch.
    pub first_shot_index: u64,

    /// Raw shot payloads.
    pub shots: Vec<ShotPayload>,
}

impl ShotBatch {
    pub fn validate(&self) -> ObservationResult<()> {
        let length =
            u64::try_from(self.shots.len()).map_err(|_| ObservationError::Overflow)?;

        self.first_shot_index
            .checked_add(length)
            .ok_or(ObservationError::Overflow)?;

        for shot in &self.shots {
            shot.validate()?;
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.shots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shots.is_empty()
    }
}

// ============================================================================
// Observation payload
// ============================================================================

/// Raw measurement payload.
#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementPayload {
    /// Aggregated measurement counts.
    Counts(OutcomeHistogram),

    /// Raw per-shot measurements.
    PerShot(ShotBatch),
}

impl MeasurementPayload {
    pub fn validate(&self) -> ObservationResult<()> {
        match self {
            Self::Counts(histogram) => {
                if histogram.is_empty() {
                    return Err(ObservationError::InvalidObservation);
                }

                histogram.total_count()?;
            }

            Self::PerShot(batch) => {
                batch.validate()?;
            }
        }

        Ok(())
    }

    /// Number of represented shots when the payload has a shot-count meaning.
    pub fn shot_count(&self) -> ObservationResult<u64> {
        match self {
            Self::Counts(histogram) => histogram.total_count(),

            Self::PerShot(batch) => u64::try_from(batch.shots.len())
                .map_err(|_| ObservationError::Overflow),
        }
    }
}

/// Raw characterization payload.
///
/// The payload is deliberately broader than measurement counts so that the
/// same observation abstraction can serve qubit, qudit, analog, bosonic,
//! photonic, continuous-variable, pulse, and future characterization modes.
#[derive(Clone, Debug, PartialEq)]
pub enum ObservationPayload {
    Measurement(MeasurementPayload),
    Scalar(ScalarSample),
    Complex(ComplexSample),

    /// Target/modality-specific payload.
    ///
    /// The interpretation MUST be specified by the observation contract.
    Opaque {
        media_type: String,
        bytes: Vec<u8>,
    },
}

impl ObservationPayload {
    pub fn validate(&self) -> ObservationResult<()> {
        match self {
            Self::Measurement(payload) => payload.validate(),

            Self::Scalar(sample) => {
                if !sample.value.is_finite() {
                    return Err(ObservationError::InvalidNumericValue);
                }

                Ok(())
            }

            Self::Complex(sample) => {
                if !sample.real.is_finite()
                    || !sample.imaginary.is_finite()
                {
                    return Err(ObservationError::InvalidNumericValue);
                }

                Ok(())
            }

            Self::Opaque { media_type, .. } => {
                if media_type.trim().is_empty() {
                    return Err(ObservationError::UnsupportedPayload);
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Explicit observation provenance.
///
/// These are references, not copies of complete ZQN objects.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObservationProvenance {
    /// Characterization run that produced the observation.
    pub characterization: Option<CharacterizationId>,

    /// Experiment that produced the observation.
    pub experiment: Option<ExperimentId>,

    /// Calibration snapshot used during execution.
    pub calibration: Option<CalibrationId>,

    /// Target identity.
    pub target: Option<ZqnIdValue>,

    /// Execution identity.
    pub execution: Option<ZqnIdValue>,

    /// Randomness derivation domain, if applicable.
    pub randomness_domain: Option<String>,

    /// Explicit deterministic seed, if one was used.
    pub deterministic_seed: Option<u64>,
}

impl ObservationProvenance {
    pub fn validate(&self) -> ObservationResult<()> {
        if let Some(domain) = &self.randomness_domain {
            if domain.trim().is_empty() {
                return Err(ObservationError::InconsistentMetadata);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Observation contract
// ============================================================================

/// Declares how an observation payload must be interpreted.
///
/// This is deliberately separate from the payload itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationContract {
    /// Semantic schema identifier.
    pub schema_id: String,

    /// Semantic schema version.
    pub schema_version: u32,

    /// Protocol that requested this observation.
    pub protocol: ProtocolId,

    /// Payload category.
    pub payload_kind: PayloadKind,

    /// Whether raw information has been preserved.
    pub preserves_raw_data: bool,

    /// Whether ordering is deterministic.
    pub deterministic_ordering: bool,
}

/// Broad observation payload category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadKind {
    Measurement,
    Scalar,
    Complex,
    Opaque,
}

impl ObservationContract {
    /// Creates the current observation contract.
    pub fn new(protocol: ProtocolId, payload_kind: PayloadKind) -> Self {
        Self {
            schema_id: String::from(OBSERVATION_SCHEMA_ID),
            schema_version: OBSERVATION_SCHEMA_VERSION,
            protocol,
            payload_kind,
            preserves_raw_data: true,
            deterministic_ordering: true,
        }
    }

    pub fn validate(&self) -> ObservationResult<()> {
        if self.schema_id != OBSERVATION_SCHEMA_ID {
            return Err(ObservationError::InconsistentMetadata);
        }

        if self.schema_version == 0 {
            return Err(ObservationError::InconsistentMetadata);
        }

        if self.protocol.as_str().is_empty() {
            return Err(ObservationError::InvalidIdentifier);
        }

        if !self.preserves_raw_data {
            return Err(ObservationError::InconsistentMetadata);
        }

        Ok(())
    }
}

// ============================================================================
// Observation header
// ============================================================================

/// Canonical observation metadata.
///
/// This intentionally uses the canonical ZQN `ObservationId` from
/// `core::ids.rs`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationHeader {
    /// Canonical ZQN observation identity.
    pub id: ObservationId,

    /// Protocol identity.
    pub protocol: ProtocolId,

    /// Experiment index inside the protocol execution.
    pub experiment_index: u64,

    /// Repetition index inside the experiment.
    pub repetition_index: u64,

    /// Whether execution has completed for this observation.
    pub complete: bool,
}

impl ObservationHeader {
    pub fn validate(&self) -> ObservationResult<()> {
        if self.protocol.as_str().is_empty() {
            return Err(ObservationError::InvalidIdentifier);
        }

        if !self.complete {
            return Err(ObservationError::IncompleteObservation);
        }

        Ok(())
    }
}

// ============================================================================
// Canonical observation
// ============================================================================

/// Canonical raw characterization observation.
///
/// An `Observation` is evidence emitted by execution. It must not be
/// interpreted as an estimator result merely because it contains numeric
/// values.
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    /// Observation identity.
    pub header: ObservationHeader,

    /// Observation interpretation contract.
    pub contract: ObservationContract,

    /// Resources represented by this observation.
    pub scope: ObservationScope,

    /// Optional execution timing.
    pub timing: Option<ObservationTiming>,

    /// Scientific provenance.
    pub provenance: ObservationProvenance,

    /// Raw/minimally structured payload.
    pub payload: ObservationPayload,
}

impl Observation {
    /// Validates all local and cross-field invariants.
    pub fn validate(&self) -> ObservationResult<()> {
        self.header.validate()?;
        self.contract.validate()?;
        self.scope.validate()?;
        self.provenance.validate()?;
        self.payload.validate()?;

        if self.header.protocol != self.contract.protocol {
            return Err(ObservationError::InconsistentMetadata);
        }

        if let Some(timing) = &self.timing {
            timing.validate()?;
        }

        Ok(())
    }

    /// Returns the canonical observation identity.
    pub fn id(&self) -> ObservationId {
        self.header.id
    }

    /// Returns the protocol identity.
    pub fn protocol(&self) -> &ProtocolId {
        &self.header.protocol
    }

    /// Returns the experiment index.
    pub fn experiment_index(&self) -> u64 {
        self.header.experiment_index
    }

    /// Returns the repetition index.
    pub fn repetition_index(&self) -> u64 {
        self.header.repetition_index
    }
}

// ============================================================================
// Protocol integration
// ============================================================================

/// Validates an observation against the requirements declared by the
/// characterization protocol.
///
/// This is the primary integration boundary between `protocol.rs` and
/// `observation.rs`.
pub fn validate_against_protocol(
    observation: &Observation,
    requirements: &ObservationRequirements,
) -> ObservationResult<()> {
    observation.validate()?;

    if requirements.raw_measurements {
        match observation.payload {
            ObservationPayload::Measurement(_) => {}
            _ => return Err(ObservationError::UnsupportedPayload),
        }
    }

    if requirements.timing && observation.timing.is_none() {
        return Err(ObservationError::InvalidTiming);
    }

    if requirements.resource_identity
        && observation.scope.is_empty()
    {
        return Err(ObservationError::InvalidResourceScope);
    }

    if requirements.experiment_identity
        && observation.header.experiment_index == u64::MAX
    {
        return Err(ObservationError::InvalidIdentifier);
    }

    if requirements.randomness_provenance {
        let provenance_present = observation
            .provenance
            .randomness_domain
            .is_some();

        if !provenance_present {
            return Err(ObservationError::InconsistentMetadata);
        }
    }

    Ok(())
}

/// Validates that a protocol's general requirements are consistent with the
/// actual observation payload.
///
/// This does not perform statistical sufficiency analysis.
pub fn validate_characterization_requirements(
    observation: &Observation,
    requirements: &CharacterizationRequirements,
) -> ObservationResult<()> {
    observation.validate()?;

    if requirements.measurement {
        if !matches!(
            observation.payload,
            ObservationPayload::Measurement(_)
        ) {
            return Err(ObservationError::UnsupportedPayload);
        }
    }

    if requirements.resource_addressing
        && observation.scope.is_empty()
    {
        return Err(ObservationError::InvalidResourceScope);
    }

    Ok(())
}

// ============================================================================
// Observation batch
// ============================================================================

/// Materialized bounded observation batch.
///
/// This is intentionally finite and bounded by available memory. It is not
/// the only way observations are transported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObservationBatch {
    pub observations: Vec<Observation>,
}

impl ObservationBatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds and validates an observation.
    pub fn push(
        &mut self,
        observation: Observation,
    ) -> ObservationResult<()> {
        observation.validate()?;
        self.observations.push(observation);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.observations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Observation> {
        self.observations.iter()
    }

    pub fn validate(&self) -> ObservationResult<()> {
        for observation in &self.observations {
            observation.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// Streaming boundary
// ============================================================================

/// Consumer-side observation stream sink.
///
/// This is the main mechanism that prevents characterization from requiring
/// all observations to exist in memory simultaneously.
pub trait ObservationSink {
    /// Consumes one observation.
    fn push(
        &mut self,
        observation: Observation,
    ) -> ObservationResult<()>;

    /// Signals end-of-stream.
    fn finish(&mut self) -> ObservationResult<()> {
        Ok(())
    }
}

/// Simple in-memory sink for tests, small workloads and adapters.
#[derive(Clone, Debug, Default)]
pub struct InMemoryObservationSink {
    batch: ObservationBatch,
}

impl InMemoryObservationSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_batch(self) -> ObservationBatch {
        self.batch
    }
}

impl ObservationSink for InMemoryObservationSink {
    fn push(
        &mut self,
        observation: Observation,
    ) -> ObservationResult<()> {
        self.batch.push(observation)
    }
}

// ============================================================================
// Resource governance
// ============================================================================

/// Explicit resource policy for observation validation/materialization.
///
/// `None` means this module does not impose an additional limit.
///
/// It does NOT mean the physical system has infinite resources.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObservationLimits {
    /// Maximum outcome symbol count accepted during materialization.
    pub max_outcome_symbols: Option<u64>,

    /// Maximum number of distinct outcomes.
    pub max_distinct_outcomes: Option<u64>,

    /// Maximum shots represented by one observation payload.
    pub max_shots_per_observation: Option<u64>,

    /// Maximum observations in one materialized batch.
    pub max_batch_observations: Option<u64>,

    /// Maximum explicit resources in one observation.
    pub max_resources_per_observation: Option<u64>,

    /// Maximum opaque payload bytes.
    pub max_opaque_payload_bytes: Option<u64>,
}

impl ObservationLimits {
    /// Validates policy configuration.
    pub fn validate(&self) -> ObservationResult<()> {
        let limits = [
            self.max_outcome_symbols,
            self.max_distinct_outcomes,
            self.max_shots_per_observation,
            self.max_batch_observations,
            self.max_resources_per_observation,
            self.max_opaque_payload_bytes,
        ];

        if limits.iter().any(|limit| matches!(limit, Some(0))) {
            return Err(ObservationError::ResourceLimitExceeded);
        }

        Ok(())
    }

    #[inline]
    fn permits(
        limit: Option<u64>,
        value: u64,
    ) -> bool {
        match limit {
            Some(maximum) => value <= maximum,
            None => true,
        }
    }
}

/// Validates one observation against explicit resource limits.
pub fn validate_with_limits(
    observation: &Observation,
    limits: &ObservationLimits,
) -> ObservationResult<()> {
    limits.validate()?;
    observation.validate()?;

    let resources = u64::try_from(
        observation.scope.resources.len(),
    )
    .map_err(|_| ObservationError::Overflow)?;

    if !ObservationLimits::permits(
        limits.max_resources_per_observation,
        resources,
    ) {
        return Err(ObservationError::ResourceLimitExceeded);
    }

    match &observation.payload {
        ObservationPayload::Measurement(
            MeasurementPayload::Counts(histogram),
        ) => {
            let distinct = u64::try_from(histogram.len())
                .map_err(|_| ObservationError::Overflow)?;

            if !ObservationLimits::permits(
                limits.max_distinct_outcomes,
                distinct,
            ) {
                return Err(ObservationError::ResourceLimitExceeded);
            }

            for (outcome, _) in histogram.iter() {
                let width = u64::try_from(outcome.len())
                    .map_err(|_| ObservationError::Overflow)?;

                if !ObservationLimits::permits(
                    limits.max_outcome_symbols,
                    width,
                ) {
                    return Err(ObservationError::ResourceLimitExceeded);
                }
            }

            let shots = histogram.total_count()?;

            if !ObservationLimits::permits(
                limits.max_shots_per_observation,
                shots,
            ) {
                return Err(ObservationError::ResourceLimitExceeded);
            }
        }

        ObservationPayload::Measurement(
            MeasurementPayload::PerShot(batch),
        ) => {
            let shots = u64::try_from(batch.len())
                .map_err(|_| ObservationError::Overflow)?;

            if !ObservationLimits::permits(
                limits.max_shots_per_observation,
                shots,
            ) {
                return Err(ObservationError::ResourceLimitExceeded);
            }

            for shot in &batch.shots {
                if let ShotPayload::Discrete(outcome) = shot {
                    let width = u64::try_from(outcome.len())
                        .map_err(|_| ObservationError::Overflow)?;

                    if !ObservationLimits::permits(
                        limits.max_outcome_symbols,
                        width,
                    ) {
                        return Err(
                            ObservationError::ResourceLimitExceeded,
                        );
                    }
                }
            }
        }

        ObservationPayload::Opaque { bytes, .. } => {
            let size = u64::try_from(bytes.len())
                .map_err(|_| ObservationError::Overflow)?;

            if !ObservationLimits::permits(
                limits.max_opaque_payload_bytes,
                size,
            ) {
                return Err(ObservationError::ResourceLimitExceeded);
            }
        }

        ObservationPayload::Scalar(_)
        | ObservationPayload::Complex(_) => {}
    }

    Ok(())
}

/// Validates a materialized batch against explicit resource policy.
pub fn validate_batch_with_limits(
    batch: &ObservationBatch,
    limits: &ObservationLimits,
) -> ObservationResult<()> {
    limits.validate()?;

    let count = u64::try_from(batch.len())
        .map_err(|_| ObservationError::Overflow)?;

    if !ObservationLimits::permits(
        limits.max_batch_observations,
        count,
    ) {
        return Err(ObservationError::ResourceLimitExceeded);
    }

    for observation in &batch.observations {
        validate_with_limits(observation, limits)?;
    }

    Ok(())
}

// ============================================================================
// Deterministic ordering
// ============================================================================

/// Stable ordering key.
///
/// Observation IDs are included as a final tie-breaker so two observations
/// with identical experiment/repetition coordinates remain deterministically
/// ordered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObservationOrderKey {
    pub experiment_index: u64,
    pub repetition_index: u64,
    pub observation_id: ObservationId,
}

impl From<&Observation> for ObservationOrderKey {
    fn from(observation: &Observation) -> Self {
        Self {
            experiment_index: observation.experiment_index(),
            repetition_index: observation.repetition_index(),
            observation_id: observation.id(),
        }
    }
}

/// Sorts a materialized batch deterministically.
pub fn sort_deterministically(
    batch: &mut ObservationBatch,
) {
    batch
        .observations
        .sort_by_key(ObservationOrderKey::from);
}

// ============================================================================
// Construction
// ============================================================================

/// Constructs a fully validated observation.
///
/// This is the preferred constructor for execution adapters.
#[allow(clippy::too_many_arguments)]
pub fn make_observation(
    id: ObservationId,
    protocol: ProtocolId,
    experiment_index: u64,
    repetition_index: u64,
    scope: ObservationScope,
    payload: ObservationPayload,
    timing: Option<ObservationTiming>,
    provenance: ObservationProvenance,
) -> ObservationResult<Observation> {
    let payload_kind = match &payload {
        ObservationPayload::Measurement(_) => {
            PayloadKind::Measurement
        }
        ObservationPayload::Scalar(_) => PayloadKind::Scalar,
        ObservationPayload::Complex(_) => PayloadKind::Complex,
        ObservationPayload::Opaque { .. } => PayloadKind::Opaque,
    };

    let header = ObservationHeader {
        id,
        protocol: protocol.clone(),
        experiment_index,
        repetition_index,
        complete: true,
    };

    let observation = Observation {
        header,
        contract: ObservationContract::new(
            protocol,
            payload_kind,
        ),
        scope,
        timing,
        provenance,
        payload,
    };

    observation.validate()?;

    Ok(observation)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn protocol() -> ProtocolId {
        ProtocolId::new("test.observation")
            .expect("valid protocol ID")
    }

    fn observation_id(value: u64) -> ObservationId {
        ObservationId::new(value)
    }

    fn scalar_payload() -> ObservationPayload {
        ObservationPayload::Scalar(
            ScalarSample::new(1.0)
                .expect("finite scalar"),
        )
    }

    #[test]
    fn canonical_observation_id_is_used() {
        let id = observation_id(7);

        let observation = make_observation(
            id,
            protocol(),
            0,
            0,
            ObservationScope::default(),
            scalar_payload(),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        assert_eq!(observation.id(), id);
    }

    #[test]
    fn canonical_logical_qubit_identity_is_accepted() {
        let scope = ObservationScope {
            resources: vec![
                ObservedResource::LogicalQubit(
                    QubitId::new(7),
                ),
            ],
            aggregate: false,
        };

        assert!(scope.validate().is_ok());
    }

    #[test]
    fn canonical_physical_qubit_identity_is_accepted() {
        let scope = ObservationScope {
            resources: vec![
                ObservedResource::PhysicalQubit(
                    PhysicalQubitId::new(7),
                ),
            ],
            aggregate: false,
        };

        assert!(scope.validate().is_ok());
    }

    #[test]
    fn non_finite_scalar_is_rejected() {
        assert_eq!(
            ScalarSample::new(f64::NAN),
            Err(ObservationError::InvalidNumericValue)
        );

        assert_eq!(
            ScalarSample::new(f64::INFINITY),
            Err(ObservationError::InvalidNumericValue)
        );
    }

    #[test]
    fn non_finite_complex_value_is_rejected() {
        assert_eq!(
            ComplexSample::new(
                f64::INFINITY,
                0.0,
            ),
            Err(ObservationError::InvalidNumericValue)
        );
    }

    #[test]
    fn histogram_counts_are_checked() {
        let mut histogram = OutcomeHistogram::new();

        histogram
            .insert(
                DiscreteOutcome::new(vec![0])
                    .expect("valid outcome"),
                10,
            )
            .expect("valid count");

        histogram
            .insert(
                DiscreteOutcome::new(vec![1])
                    .expect("valid outcome"),
                20,
            )
            .expect("valid count");

        assert_eq!(
            histogram.total_count()
                .expect("valid total"),
            30
        );
    }

    #[test]
    fn histogram_overflow_is_rejected() {
        let outcome = DiscreteOutcome::new(vec![0])
            .expect("valid outcome");

        let mut histogram = OutcomeHistogram::new();

        histogram
            .insert(
                outcome.clone(),
                u64::MAX,
            )
            .expect("first insert");

        assert_eq!(
            histogram.insert(
                outcome,
                1,
            ),
            Err(ObservationError::Overflow)
        );
    }

    #[test]
    fn observation_is_validated_before_storage() {
        let observation = make_observation(
            observation_id(1),
            protocol(),
            0,
            0,
            ObservationScope::default(),
            scalar_payload(),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        let mut batch = ObservationBatch::new();

        batch
            .push(observation)
            .expect("observation accepted");

        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn observations_can_be_streamed() {
        let mut sink = InMemoryObservationSink::new();

        let observation = make_observation(
            observation_id(1),
            protocol(),
            0,
            0,
            ObservationScope::default(),
            scalar_payload(),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        sink.push(observation)
            .expect("push succeeds");

        sink.finish()
            .expect("finish succeeds");

        assert_eq!(
            sink.into_batch().len(),
            1
        );
    }

    #[test]
    fn deterministic_sorting_is_stable() {
        let first = make_observation(
            observation_id(2),
            protocol(),
            1,
            0,
            ObservationScope::default(),
            scalar_payload(),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        let second = make_observation(
            observation_id(1),
            protocol(),
            0,
            0,
            ObservationScope::default(),
            scalar_payload(),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        let mut batch = ObservationBatch {
            observations: vec![first, second],
        };

        sort_deterministically(&mut batch);

        assert_eq!(
            batch.observations[0].id(),
            observation_id(1)
        );

        assert_eq!(
            batch.observations[1].id(),
            observation_id(2)
        );
    }

    #[test]
    fn absent_limits_do_not_create_artificial_machine_limits() {
        let limits = ObservationLimits::default();

        assert!(
            ObservationLimits::permits(
                limits.max_shots_per_observation,
                u64::MAX,
            )
        );
    }

    #[test]
    fn explicit_limits_are_enforced() {
        let observation = make_observation(
            observation_id(1),
            protocol(),
            0,
            0,
            ObservationScope::default(),
            ObservationPayload::Measurement(
                MeasurementPayload::Counts({
                    let mut histogram =
                        OutcomeHistogram::new();

                    histogram
                        .insert(
                            DiscreteOutcome::new(vec![0])
                                .expect("outcome"),
                            10,
                        )
                        .expect("insert");

                    histogram
                }),
            ),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        let limits = ObservationLimits {
            max_shots_per_observation: Some(5),
            ..ObservationLimits::default()
        };

        assert_eq!(
            validate_with_limits(
                &observation,
                &limits,
            ),
            Err(
                ObservationError::ResourceLimitExceeded
            )
        );
    }

    #[test]
    fn protocol_observation_requirements_are_checked() {
        let observation = make_observation(
            observation_id(1),
            protocol(),
            0,
            0,
            ObservationScope {
                resources: vec![
                    ObservedResource::LogicalQubit(
                        QubitId::new(0),
                    ),
                ],
                aggregate: false,
            },
            ObservationPayload::Measurement(
                MeasurementPayload::Counts({
                    let mut histogram =
                        OutcomeHistogram::new();

                    histogram
                        .insert(
                            DiscreteOutcome::new(vec![0])
                                .expect("outcome"),
                            10,
                        )
                        .expect("insert");

                    histogram
                }),
            ),
            None,
            ObservationProvenance::default(),
        )
        .expect("valid observation");

        let requirements = ObservationRequirements {
            raw_measurements: true,
            resource_identity: true,
            experiment_identity: true,
            ..ObservationRequirements::default()
        };

        assert!(
            validate_against_protocol(
                &observation,
                &requirements,
            )
            .is_ok()
        );
    }
}