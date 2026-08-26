//! Zamani Quantum Benchmarking — Crosstalk Characterization
//!
//! Production-grade protocol-level crosstalk analysis.
//!
//! # Purpose
//!
//! This module defines the canonical protocol contract for measuring
//! context-dependent quantum-operation degradation caused by simultaneous
//! operations, spectator activity, measurement activity, control activity,
//! or another explicitly declared execution context.
//!
//! The module is deliberately broader than simultaneous randomized
//! benchmarking (SRB). SRB is one important source of observations, but
//! crosstalk can also be characterized from:
//!
//! - direct randomized benchmarking;
//! - cycle benchmarking;
//! - layer-fidelity experiments;
//! - spectator-qubit experiments;
//! - mid-circuit-measurement experiments;
//! - gate-set tomography;
//! - idle/spectator tomography;
//! - application-level paired experiments;
//! - future Zamani benchmark protocols.
//!
//! # Architectural boundary
//!
//! This file owns:
//!
//! 1. crosstalk experiment configuration;
//! 2. actor/victim/spectator context representation;
//! 3. paired baseline/context observations;
//! 4. validation of experimental comparability;
//! 5. binomial success/error statistics;
//! 6. degradation and relative-degradation metrics;
//! 7. confidence intervals;
//! 8. statistical significance indicators;
//! 9. correlated/joint-error diagnostics when supplied;
//! 10. aggregation across contexts;
//! 11. stable result/provenance metadata;
//! 12. execution/generation integration contracts.
//!
//! This file does NOT:
//!
//! - generate quantum circuits;
//! - generate Clifford sequences;
//! - execute circuits;
//! - select hardware;
//! - access calibration;
//! - access topology;
//! - perform routing;
//! - perform scheduling;
//! - lower Quantum IR;
//! - implement provider SDKs;
//! - implement randomized benchmarking;
//! - duplicate the canonical regression engine;
//! - print diagnostics;
//! - maintain global state.
//!
//! # Scientific model
//!
//! Crosstalk is treated as a context-dependent change in an observable.
//!
//! For a victim operation/system:
//!
//! ```text
//! isolated_error      = e_base
//! contextual_error    = e_context
//!
//! absolute_degradation
//!     = e_context - e_base
//!
//! relative_degradation
//!     = (e_context - e_base) / e_base
//! ```
//!
//! A positive degradation means that the contextual experiment performs
//! worse than the isolated baseline.
//!
//! The same comparison can be made in success-probability space:
//!
//! ```text
//! success_degradation
//!     = p_context - p_base
//! ```
//!
//! Since higher success probability is better, a negative value indicates
//! degradation.
//!
//! The protocol therefore exposes both:
//!
//! - success-probability degradation;
//! - error-rate degradation.
//!
//! They must not be conflated.
//!
//! # Context model
//!
//! Crosstalk is fundamentally contextual. A context can contain:
//!
//! - one or more simultaneously active qubits;
//! - one or more simultaneously active gates;
//! - spectator qubits;
//! - measurement operations;
//! - reset operations;
//! - frequency/control domains;
//! - a named workload;
//! - a topology relationship;
//! - arbitrary backend-specific metadata.
//!
//! The protocol never assumes that crosstalk is necessarily caused by
//! spatial adjacency. The execution layer supplies the actual context.
//!
//! # Statistical policy
//!
//! The implementation:
//!
//! - rejects NaN and infinity;
//! - rejects invalid probabilities;
//! - rejects successes > shots;
//! - rejects zero-shot observations;
//! - rejects mismatched experimental conditions;
//! - rejects duplicate context IDs;
//! - rejects duplicate victim IDs within a context;
//! - rejects invalid confidence levels;
//! - uses Wilson intervals for individual binomial proportions;
//! - uses a conservative normal approximation for paired difference
//!   confidence intervals;
//! - reports the standard-error/z-score diagnostic separately;
//! - never silently discards observations.
//!
//! The difference confidence interval is intentionally described as an
//! approximate interval. Exact paired/binomial methods can be added later
//! through the statistics subsystem without changing the public crosstalk
//! experiment/result model.
//!
//! # Scientific limitations
//!
//! A statistically significant contextual degradation does not by itself
//! identify the physical mechanism.
//!
//! Possible causes include:
//!
//! - coherent control errors;
//! - residual coupling;
//! - frequency collisions;
//! - microwave cross-driving;
//! - leakage;
//! - correlated stochastic noise;
//! - shared-control electronics;
//! - measurement-induced disturbance;
//! - thermal effects;
//! - compiler/routing interactions;
//! - scheduling changes;
//! - calibration drift.
//!
//! The result therefore says that a contextual difference was observed.
//! It does not automatically label that difference as a particular physical
//! mechanism.
//!
//! # Relationship to simultaneous randomized benchmarking
//!
//! `protocols/simultaneous_rb.rs` already provides a dedicated SRB protocol.
//! That implementation should remain responsible for SRB-specific decay
//! fitting and interpretation.
//!
//! This file provides the crosstalk-specific comparison layer:
//!
//! ```text
//!                 SRB
//!                  │
//!                  ▼
//!        baseline/context error
//!                  │
//!                  ▼
//!             crosstalk.rs
//!                  │
//!       ┌──────────┼───────────┐
//!       ▼          ▼           ▼
//! degradation   significance  correlation
//! ```
//!
//! Therefore `crosstalk.rs` does not re-fit SRB exponential decay curves.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! generators / compiler / scheduler
//!      │
//!      ▼
//! execution::executor
//!      │
//!      ▼
//! raw observations
//!      │
//!      ├───────────────┐
//!      │               │
//!      ▼               ▼
//! protocols::         other
//! simultaneous_rb     characterization
//!      │               │
//!      └───────┬───────┘
//!              ▼
//!       protocols::crosstalk
//!              │
//!       ┌──────┼────────┐
//!       ▼      ▼        ▼
//!    metrics  analysis reporting
//! ```
//!
//! Future integration:
//!
//! - `generators/*` creates contextual circuits;
//! - `execution/*` executes them;
//! - `protocols/simultaneous_rb.rs` may produce SRB-derived error estimates;
//! - `statistics/*` may provide more sophisticated hypothesis tests;
//! - `core::metric` receives the resulting metrics;
//! - `core::result` wraps the protocol result;
//! - `reporting/*` serializes it;
//! - `analysis/*` compares it against baselines;
//! - `registry/*` registers the protocol;
//! - `stdlib::quantum` exposes the Zamani-language API.
//!
//! No change to the mathematical semantics of this file is required when
//! those integrations are added.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//!
//! # Dependencies
//!
//! Only dependencies already present in Zamani are used:
//!
//! - serde;
//! - standard library.
//!
//! No new crate is required.
//!
//! # References
//!
//! Crosstalk benchmarking is supported by established work on:
//!
//! - simultaneous randomized benchmarking;
//! - context-dependent error characterization;
//! - simultaneous gate-set tomography;
//! - spectator and measurement-induced crosstalk.
//!
//! The implementation intentionally keeps the protocol-neutral comparison
//! layer separate from any one experimental technique.

#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// ============================================================================
// Public protocol identity
// ============================================================================

/// Stable machine-readable protocol identifier.
pub const CROSSTALK_BENCHMARK_ID: &str = "crosstalk";

/// Stable result schema version.
pub const CROSSTALK_RESULT_SCHEMA_VERSION: u32 = 1;

/// Stable experiment schema version.
pub const CROSSTALK_EXPERIMENT_SCHEMA_VERSION: u32 = 1;

/// Stable algorithm identifier.
///
/// Change this if the mathematical interpretation of the result changes.
pub const CROSSTALK_ALGORITHM_ID: &str = "zamani.crosstalk.contextual_degradation.v1";

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Minimum supported confidence level.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.50;

/// Maximum supported confidence level.
///
/// Values extremely close to one create unstable normal quantiles and are
/// intentionally bounded.
pub const MAX_CONFIDENCE_LEVEL: f64 = 0.999_999_999_999;

/// Maximum number of contexts accepted by one result.
pub const DEFAULT_MAX_CONTEXTS: usize = 65_536;

/// Maximum number of victims in one context.
pub const DEFAULT_MAX_VICTIMS_PER_CONTEXT: usize = 65_536;

/// Maximum number of observations attached to one context.
pub const DEFAULT_MAX_OBSERVATIONS_PER_CONTEXT: usize = 65_536;

/// Maximum number of metadata entries attached to one context.
pub const DEFAULT_MAX_METADATA_ENTRIES: usize = 1_024;

/// Numerical tolerance used when validating probabilities.
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance used when comparing confidence levels.
const CONFIDENCE_EPSILON: f64 = 1.0e-15;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by crosstalk configuration, validation and analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrosstalkError {
    /// No contexts were supplied.
    NoContexts,

    /// Context ID is empty.
    EmptyContextId,

    /// Context ID is too long.
    ContextIdTooLong {
        /// Supplied length.
        length: usize,
    },

    /// Context IDs must be unique.
    DuplicateContextId {
        /// Duplicate identifier.
        context_id: String,
    },

    /// Context metadata exceeds the configured limit.
    TooManyMetadataEntries {
        /// Actual number.
        actual: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// A context has no victims.
    NoVictims {
        /// Context identifier.
        context_id: String,
    },

    /// A victim identifier is empty.
    EmptyVictimId,

    /// Victim IDs must be unique within a context.
    DuplicateVictimId {
        /// Duplicate victim.
        victim_id: String,
    },

    /// Too many victims.
    TooManyVictims {
        /// Actual number.
        actual: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Invalid qubit identifier.
    InvalidQubitId {
        /// Qubit index.
        qubit: usize,
    },

    /// The same qubit is assigned as both victim and active context.
    VictimContextOverlap {
        /// Victim identifier.
        victim_id: String,

        /// Overlapping qubit.
        qubit: usize,
    },

    /// Context contains duplicate qubit identifiers.
    DuplicateContextQubit {
        /// Duplicate qubit.
        qubit: usize,
    },

    /// Empty observation set.
    EmptyObservations {
        /// Context identifier.
        context_id: String,
    },

    /// Too many observations.
    TooManyObservations {
        /// Actual number.
        actual: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Zero shots are not meaningful.
    ZeroShots,

    /// Successes cannot exceed shots.
    SuccessesExceedShots {
        /// Successful shots.
        successes: u64,

        /// Total shots.
        shots: u64,
    },

    /// Probability is outside [0, 1].
    InvalidProbability {
        /// Invalid value.
        value: f64,
    },

    /// Error rate is outside [0, 1].
    InvalidErrorRate {
        /// Invalid value.
        value: f64,
    },

    /// A floating-point value is NaN or infinite.
    NonFiniteValue {
        /// Field name.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel {
        /// Invalid confidence.
        value: f64,
    },

    /// Baseline and contextual observations are incompatible.
    IncompatibleConditions {
        /// Human-readable reason.
        reason: String,
    },

    /// Baseline observation is missing.
    MissingBaseline {
        /// Victim identifier.
        victim_id: String,
    },

    /// Contextual observation is missing.
    MissingContextObservation {
        /// Victim identifier.
        victim_id: String,
    },

    /// Baseline error is zero, making relative degradation undefined.
    UndefinedRelativeDegradation,

    /// The normal quantile could not be calculated.
    InvalidNormalQuantile {
        /// Confidence level.
        confidence: f64,
    },

    /// An arithmetic result became non-finite.
    NonFiniteStatistic {
        /// Name of statistic.
        name: &'static str,
    },

    /// An aggregate cannot be calculated.
    EmptyAggregation,

    /// A supplied weight is invalid.
    InvalidWeight {
        /// Victim identifier.
        victim_id: String,

        /// Invalid weight.
        weight: f64,
    },

    /// Aggregation weights sum to zero.
    ZeroTotalWeight,

    /// An observation belongs to an unsupported model.
    UnsupportedObservationModel {
        /// Model name.
        model: String,
    },

    /// The context is missing an explicit experimental-condition identity.
    MissingConditionIdentity,

    /// Baseline and contextual condition identities do not match.
    ConditionIdentityMismatch {
        /// Baseline identity.
        baseline: String,

        /// Context identity.
        contextual: String,
    },
}

impl fmt::Display for CrosstalkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoContexts => {
                write!(formatter, "crosstalk analysis requires at least one context")
            }

            Self::EmptyContextId => {
                write!(formatter, "crosstalk context ID cannot be empty")
            }

            Self::ContextIdTooLong { length } => {
                write!(
                    formatter,
                    "crosstalk context ID is too long: {} characters",
                    length
                )
            }

            Self::DuplicateContextId { context_id } => {
                write!(formatter, "duplicate crosstalk context ID '{}'", context_id)
            }

            Self::TooManyMetadataEntries { actual, maximum } => {
                write!(
                    formatter,
                    "context metadata contains {} entries; maximum is {}",
                    actual, maximum
                )
            }

            Self::NoVictims { context_id } => {
                write!(
                    formatter,
                    "crosstalk context '{}' has no victim targets",
                    context_id
                )
            }

            Self::EmptyVictimId => {
                write!(formatter, "crosstalk victim ID cannot be empty")
            }

            Self::DuplicateVictimId { victim_id } => {
                write!(formatter, "duplicate victim ID '{}'", victim_id)
            }

            Self::TooManyVictims { actual, maximum } => {
                write!(
                    formatter,
                    "context contains {} victims; maximum is {}",
                    actual, maximum
                )
            }

            Self::InvalidQubitId { qubit } => {
                write!(formatter, "invalid qubit identifier {}", qubit)
            }

            Self::VictimContextOverlap { victim_id, qubit } => {
                write!(
                    formatter,
                    "victim '{}' overlaps contextual qubit {}",
                    victim_id, qubit
                )
            }

            Self::DuplicateContextQubit { qubit } => {
                write!(
                    formatter,
                    "context contains duplicate qubit identifier {}",
                    qubit
                )
            }

            Self::EmptyObservations { context_id } => {
                write!(
                    formatter,
                    "context '{}' has no observations",
                    context_id
                )
            }

            Self::TooManyObservations { actual, maximum } => {
                write!(
                    formatter,
                    "context contains {} observations; maximum is {}",
                    actual, maximum
                )
            }

            Self::ZeroShots => {
                write!(formatter, "an observation must contain at least one shot")
            }

            Self::SuccessesExceedShots { successes, shots } => {
                write!(
                    formatter,
                    "success count {} exceeds shot count {}",
                    successes, shots
                )
            }

            Self::InvalidProbability { value } => {
                write!(
                    formatter,
                    "probability must be in [0, 1], got {}",
                    value
                )
            }

            Self::InvalidErrorRate { value } => {
                write!(
                    formatter,
                    "error rate must be in [0, 1], got {}",
                    value
                )
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "{} must be finite, got {}",
                    field, value
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level {} is outside [{}, {}]",
                    value,
                    MIN_CONFIDENCE_LEVEL,
                    MAX_CONFIDENCE_LEVEL
                )
            }

            Self::IncompatibleConditions { reason } => {
                write!(formatter, "incompatible experimental conditions: {}", reason)
            }

            Self::MissingBaseline { victim_id } => {
                write!(
                    formatter,
                    "baseline observation is missing for victim '{}'",
                    victim_id
                )
            }

            Self::MissingContextObservation { victim_id } => {
                write!(
                    formatter,
                    "contextual observation is missing for victim '{}'",
                    victim_id
                )
            }

            Self::UndefinedRelativeDegradation => {
                write!(
                    formatter,
                    "relative degradation is undefined because baseline error is zero"
                )
            }

            Self::InvalidNormalQuantile { confidence } => {
                write!(
                    formatter,
                    "unable to calculate normal quantile for confidence {}",
                    confidence
                )
            }

            Self::NonFiniteStatistic { name } => {
                write!(
                    formatter,
                    "crosstalk statistic '{}' is non-finite",
                    name
                )
            }

            Self::EmptyAggregation => {
                write!(formatter, "cannot aggregate an empty crosstalk result")
            }

            Self::InvalidWeight { victim_id, weight } => {
                write!(
                    formatter,
                    "weight for victim '{}' must be finite and positive, got {}",
                    victim_id, weight
                )
            }

            Self::ZeroTotalWeight => {
                write!(formatter, "crosstalk aggregation weights sum to zero")
            }

            Self::UnsupportedObservationModel { model } => {
                write!(
                    formatter,
                    "unsupported crosstalk observation model '{}'",
                    model
                )
            }

            Self::MissingConditionIdentity => {
                write!(
                    formatter,
                    "baseline and contextual experiments require condition identities"
                )
            }

            Self::ConditionIdentityMismatch {
                baseline,
                contextual,
            } => {
                write!(
                    formatter,
                    "condition identity mismatch: baseline='{}', contextual='{}'",
                    baseline, contextual
                )
            }
        }
    }
}

impl Error for CrosstalkError {}

// ============================================================================
// Crosstalk context
// ============================================================================

/// The role of a contextual operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextRole {
    /// An operation intentionally executed simultaneously with the victim.
    ActiveOperation,

    /// A qubit deliberately left idle while other operations execute.
    Spectator,

    /// A measurement operation acting concurrently or immediately before/after
    /// the victim experiment.
    Measurement,

    /// A reset operation acting in the contextual region.
    Reset,

    /// An externally defined control operation.
    Control,

    /// A compiler/scheduler-induced context.
    CompilerSchedule,

    /// An application-defined context.
    Custom,
}

impl ContextRole {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveOperation => "active_operation",
            Self::Spectator => "spectator",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Control => "control",
            Self::CompilerSchedule => "compiler_schedule",
            Self::Custom => "custom",
        }
    }
}

/// A contextual operation or environmental condition.
///
/// The actual circuit/gate identity is intentionally opaque to this protocol.
/// The generator/executor layer can store a stable identifier here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrosstalkContext {
    /// Stable context identifier.
    pub id: String,

    /// Semantic role of the context.
    pub role: ContextRole,

    /// Stable operation identifier, if applicable.
    pub operation_id: Option<String>,

    /// Physical or logical qubits affected by the context.
    pub qubits: Vec<usize>,

    /// Optional arbitrary metadata.
    pub metadata: BTreeMap<String, String>,
}

impl CrosstalkContext {
    /// Creates a validated contextual operation.
    pub fn new(
        id: impl Into<String>,
        role: ContextRole,
        operation_id: Option<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, CrosstalkError> {
        let context = Self {
            id: id.into(),
            role,
            operation_id,
            qubits,
            metadata: BTreeMap::new(),
        };

        context.validate()?;
        Ok(context)
    }

    /// Adds metadata while preserving the validation contract.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, CrosstalkError> {
        self.metadata.insert(key.into(), value.into());

        if self.metadata.len() > DEFAULT_MAX_METADATA_ENTRIES {
            return Err(CrosstalkError::TooManyMetadataEntries {
                actual: self.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_ENTRIES,
            });
        }

        Ok(self)
    }

    /// Validates the contextual operation.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        validate_identifier(&self.id, "context")?;

        let mut seen = BTreeSet::new();

        for &qubit in &self.qubits {
            if !seen.insert(qubit) {
                return Err(CrosstalkError::DuplicateContextQubit { qubit });
            }
        }

        if let Some(operation_id) = &self.operation_id {
            validate_identifier(operation_id, "operation")?;
        }

        if self.metadata.len() > DEFAULT_MAX_METADATA_ENTRIES {
            return Err(CrosstalkError::TooManyMetadataEntries {
                actual: self.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_ENTRIES,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Victim definition
// ============================================================================

/// A victim operation/system whose performance is being measured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrosstalkVictim {
    /// Stable victim identifier.
    pub id: String,

    /// Physical or logical qubits occupied by the victim.
    pub qubits: Vec<usize>,

    /// Stable operation/gate identifier when available.
    pub operation_id: Option<String>,

    /// Optional metadata.
    pub metadata: BTreeMap<String, String>,
}

impl CrosstalkVictim {
    /// Creates a victim.
    pub fn new(
        id: impl Into<String>,
        qubits: Vec<usize>,
        operation_id: Option<String>,
    ) -> Result<Self, CrosstalkError> {
        let victim = Self {
            id: id.into(),
            qubits,
            operation_id,
            metadata: BTreeMap::new(),
        };

        victim.validate()?;
        Ok(victim)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, CrosstalkError> {
        self.metadata.insert(key.into(), value.into());

        if self.metadata.len() > DEFAULT_MAX_METADATA_ENTRIES {
            return Err(CrosstalkError::TooManyMetadataEntries {
                actual: self.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_ENTRIES,
            });
        }

        Ok(self)
    }

    /// Validates this victim.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        validate_identifier(&self.id, "victim")?;

        if self.qubits.is_empty() {
            return Err(CrosstalkError::InvalidQubitId { qubit: 0 });
        }

        let mut seen = BTreeSet::new();

        for &qubit in &self.qubits {
            if !seen.insert(qubit) {
                return Err(CrosstalkError::DuplicateContextQubit { qubit });
            }
        }

        if let Some(operation_id) = &self.operation_id {
            validate_identifier(operation_id, "operation")?;
        }

        if self.metadata.len() > DEFAULT_MAX_METADATA_ENTRIES {
            return Err(CrosstalkError::TooManyMetadataEntries {
                actual: self.metadata.len(),
                maximum: DEFAULT_MAX_METADATA_ENTRIES,
            });
        }

        Ok(())
    }

    /// Returns whether this victim overlaps contextual qubits.
    #[must_use]
    pub fn overlaps_context(&self, context: &CrosstalkContext) -> bool {
        self.qubits
            .iter()
            .any(|qubit| context.qubits.contains(qubit))
    }
}

// ============================================================================
// Experimental condition identity
// ============================================================================

/// Identity of the experimental conditions that must remain comparable.
///
/// Crosstalk is a paired/contextual measurement. If the baseline and
/// contextual experiments differ in uncontrolled dimensions, the resulting
/// difference may be attributable to something other than crosstalk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionIdentity {
    /// Stable workload/circuit family identifier.
    pub workload_id: String,

    /// Stable compiler configuration identifier.
    pub compiler_id: String,

    /// Stable routing configuration identifier.
    pub routing_id: String,

    /// Stable scheduling configuration identifier.
    pub scheduling_id: String,

    /// Stable calibration identity.
    pub calibration_id: String,

    /// Stable backend identity.
    pub backend_id: String,

    /// Optional measurement configuration identity.
    pub measurement_id: String,

    /// Optional user-defined condition hash.
    pub condition_hash: Option<String>,
}

impl ConditionIdentity {
    /// Validates the condition identity.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        let fields = [
            ("workload_id", self.workload_id.as_str()),
            ("compiler_id", self.compiler_id.as_str()),
            ("routing_id", self.routing_id.as_str()),
            ("scheduling_id", self.scheduling_id.as_str()),
            ("calibration_id", self.calibration_id.as_str()),
            ("backend_id", self.backend_id.as_str()),
            ("measurement_id", self.measurement_id.as_str()),
        ];

        for (name, value) in fields {
            if value.trim().is_empty() {
                return Err(CrosstalkError::MissingConditionIdentity);
            }

            validate_identifier(value, name)?;
        }

        if let Some(hash) = &self.condition_hash {
            validate_identifier(hash, "condition_hash")?;
        }

        Ok(())
    }

    /// Returns whether two conditions are comparable.
    #[must_use]
    pub fn is_comparable_with(&self, other: &Self) -> bool {
        self.workload_id == other.workload_id
            && self.compiler_id == other.compiler_id
            && self.routing_id == other.routing_id
            && self.scheduling_id == other.scheduling_id
            && self.calibration_id == other.calibration_id
            && self.backend_id == other.backend_id
            && self.measurement_id == other.measurement_id
            && self.condition_hash == other.condition_hash
    }
}

// ============================================================================
// Observation
// ============================================================================

/// One binomial measurement observation.
///
/// A crosstalk result should normally be derived from the same observable
/// under isolated and contextual conditions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkObservation {
    /// Number of accepted/successful outcomes.
    pub successes: u64,

    /// Total number of shots.
    pub shots: u64,

    /// Optional external weight for aggregation.
    pub weight: Option<f64>,
}

impl CrosstalkObservation {
    /// Creates an observation.
    pub fn new(successes: u64, shots: u64) -> Result<Self, CrosstalkError> {
        let observation = Self {
            successes,
            shots,
            weight: None,
        };

        observation.validate()?;
        Ok(observation)
    }

    /// Sets an explicit aggregation weight.
    pub fn with_weight(mut self, weight: f64) -> Result<Self, CrosstalkError> {
        validate_finite(weight, "observation.weight")?;

        if weight <= 0.0 {
            return Err(CrosstalkError::InvalidWeight {
                victim_id: "<observation>".to_owned(),
                weight,
            });
        }

        self.weight = Some(weight);
        Ok(self)
    }

    /// Validates the observation.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        if self.shots == 0 {
            return Err(CrosstalkError::ZeroShots);
        }

        if self.successes > self.shots {
            return Err(CrosstalkError::SuccessesExceedShots {
                successes: self.successes,
                shots: self.shots,
            });
        }

        if let Some(weight) = self.weight {
            validate_finite(weight, "observation.weight")?;

            if weight <= 0.0 {
                return Err(CrosstalkError::InvalidWeight {
                    victim_id: "<observation>".to_owned(),
                    weight,
                });
            }
        }

        Ok(())
    }

    /// Returns empirical success probability.
    pub fn success_probability(&self) -> Result<f64, CrosstalkError> {
        self.validate()?;

        let probability = self.successes as f64 / self.shots as f64;

        validate_probability(probability)?;
        Ok(probability)
    }

    /// Returns empirical error probability.
    pub fn error_rate(&self) -> Result<f64, CrosstalkError> {
        let error = 1.0 - self.success_probability()?;

        validate_error_rate(error)?;
        Ok(error)
    }
}

// ============================================================================
// Observation collection
// ============================================================================

/// A complete observation for one victim under one experimental context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VictimObservation {
    /// Victim identifier.
    pub victim_id: String,

    /// Experimental condition identity.
    pub condition: ConditionIdentity,

    /// One or more repeated measurements.
    pub observations: Vec<CrosstalkObservation>,

    /// Optional sequence/circuit-depth label.
///
/// This is deliberately opaque. If the input comes from RB, it can represent
/// an already-aggregated RB result rather than individual RB sequence lengths.
    pub experiment_label: Option<String>,
}

impl VictimObservation {
    /// Creates a victim observation.
    pub fn new(
        victim_id: impl Into<String>,
        condition: ConditionIdentity,
        observations: Vec<CrosstalkObservation>,
    ) -> Result<Self, CrosstalkError> {
        let value = Self {
            victim_id: victim_id.into(),
            condition,
            observations,
            experiment_label: None,
        };

        value.validate()?;
        Ok(value)
    }

    /// Sets an optional experiment label.
    #[must_use]
    pub fn with_experiment_label(mut self, label: impl Into<String>) -> Self {
        self.experiment_label = Some(label.into());
        self
    }

    /// Validates the observation collection.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        validate_identifier(&self.victim_id, "victim")?;
        self.condition.validate()?;

        if self.observations.is_empty() {
            return Err(CrosstalkError::EmptyObservations {
                context_id: self.victim_id.clone(),
            });
        }

        if self.observations.len() > DEFAULT_MAX_OBSERVATIONS_PER_CONTEXT {
            return Err(CrosstalkError::TooManyObservations {
                actual: self.observations.len(),
                maximum: DEFAULT_MAX_OBSERVATIONS_PER_CONTEXT,
            });
        }

        for observation in &self.observations {
            observation.validate()?;
        }

        Ok(())
    }

    /// Pools all observations into one binomial observation.
    ///
    /// Pooling is appropriate when all observations represent the same
    /// condition and observable.
    pub fn pooled(&self) -> Result<CrosstalkObservation, CrosstalkError> {
        self.validate()?;

        let mut successes = 0_u64;
        let mut shots = 0_u64;

        for observation in &self.observations {
            successes = successes.checked_add(observation.successes).ok_or(
                CrosstalkError::NonFiniteStatistic {
                    name: "pooled_successes",
                },
            )?;

            shots = shots.checked_add(observation.shots).ok_or(
                CrosstalkError::NonFiniteStatistic {
                    name: "pooled_shots",
                },
            )?;
        }

        CrosstalkObservation::new(successes, shots)
    }
}

// ============================================================================
// Experiment configuration
// ============================================================================

/// Crosstalk experiment configuration.
///
/// This structure is execution-neutral. The generator/executor layer interprets
/// it to produce actual circuits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkConfig {
    /// Schema version.
    pub schema_version: u32,

    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark configuration identity.
    pub experiment_id: String,

    /// Victim operations.
    pub victims: Vec<CrosstalkVictim>,

    /// Contextual conditions.
    pub contexts: Vec<CrosstalkContext>,

    /// Statistical confidence level.
    pub confidence_level: f64,

    /// Maximum contexts accepted by this configuration.
    pub max_contexts: usize,

    /// Maximum victims accepted by this configuration.
    pub max_victims_per_context: usize,

    /// Whether contextual qubits may overlap victim qubits.
    ///
    /// Default is false because the normal crosstalk interpretation treats
    /// contextual operations as external to the victim.
    pub allow_victim_context_overlap: bool,

    /// Stable algorithm identifier.
    pub algorithm_id: String,
}

impl Default for CrosstalkConfig {
    fn default() -> Self {
        Self {
            schema_version: CROSSTALK_EXPERIMENT_SCHEMA_VERSION,
            benchmark_id: CROSSTALK_BENCHMARK_ID.to_owned(),
            experiment_id: "crosstalk-experiment".to_owned(),
            victims: Vec::new(),
            contexts: Vec::new(),
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            max_contexts: DEFAULT_MAX_CONTEXTS,
            max_victims_per_context: DEFAULT_MAX_VICTIMS_PER_CONTEXT,
            allow_victim_context_overlap: false,
            algorithm_id: CROSSTALK_ALGORITHM_ID.to_owned(),
        }
    }
}

impl CrosstalkConfig {
    /// Creates an empty configuration.
    #[must_use]
    pub fn new(experiment_id: impl Into<String>) -> Self {
        Self {
            experiment_id: experiment_id.into(),
            ..Self::default()
        }
    }

    /// Adds a victim.
    pub fn add_victim(
        &mut self,
        victim: CrosstalkVictim,
    ) -> Result<(), CrosstalkError> {
        victim.validate()?;

        if self.victims.iter().any(|existing| existing.id == victim.id) {
            return Err(CrosstalkError::DuplicateVictimId {
                victim_id: victim.id,
            });
        }

        self.victims.push(victim);
        Ok(())
    }

    /// Adds a contextual condition.
    pub fn add_context(
        &mut self,
        context: CrosstalkContext,
    ) -> Result<(), CrosstalkError> {
        context.validate()?;

        if self.contexts.len() >= self.max_contexts {
            return Err(CrosstalkError::TooManyObservations {
                actual: self.contexts.len() + 1,
                maximum: self.max_contexts,
            });
        }

        if self
            .contexts
            .iter()
            .any(|existing| existing.id == context.id)
        {
            return Err(CrosstalkError::DuplicateContextId {
                context_id: context.id,
            });
        }

        self.contexts.push(context);
        Ok(())
    }

    /// Validates the complete experiment configuration.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        if self.schema_version == 0 {
            return Err(CrosstalkError::IncompatibleConditions {
                reason: "schema version cannot be zero".to_owned(),
            });
        }

        validate_identifier(&self.benchmark_id, "benchmark_id")?;
        validate_identifier(&self.experiment_id, "experiment_id")?;
        validate_identifier(&self.algorithm_id, "algorithm_id")?;

        validate_confidence_level(self.confidence_level)?;

        if self.victims.is_empty() {
            return Err(CrosstalkError::NoVictims {
                context_id: self.experiment_id.clone(),
            });
        }

        if self.contexts.is_empty() {
            return Err(CrosstalkError::NoContexts);
        }

        if self.contexts.len() > self.max_contexts {
            return Err(CrosstalkError::TooManyObservations {
                actual: self.contexts.len(),
                maximum: self.max_contexts,
            });
        }

        let mut context_ids = BTreeSet::new();

        for context in &self.contexts {
            context.validate()?;

            if !context_ids.insert(context.id.clone()) {
                return Err(CrosstalkError::DuplicateContextId {
                    context_id: context.id.clone(),
                });
            }

            for victim in &self.victims {
                if !self.allow_victim_context_overlap && victim.overlaps_context(context) {
                    let qubit = victim
                        .qubits
                        .iter()
                        .copied()
                        .find(|q| context.qubits.contains(q))
                        .unwrap_or(0);

                    return Err(CrosstalkError::VictimContextOverlap {
                        victim_id: victim.id.clone(),
                        qubit,
                    });
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Baseline/context pair
// ============================================================================

/// Paired baseline and contextual observations for one victim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkPair {
    /// Victim definition.
    pub victim: CrosstalkVictim,

    /// Isolated baseline observation.
    pub baseline: VictimObservation,

    /// Contextual observation.
    pub contextual: VictimObservation,
}

impl CrosstalkPair {
    /// Creates and validates a paired comparison.
    pub fn new(
        victim: CrosstalkVictim,
        baseline: VictimObservation,
        contextual: VictimObservation,
    ) -> Result<Self, CrosstalkError> {
        let pair = Self {
            victim,
            baseline,
            contextual,
        };

        pair.validate()?;
        Ok(pair)
    }

    /// Validates comparability.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        self.victim.validate()?;
        self.baseline.validate()?;
        self.contextual.validate()?;

        if self.baseline.victim_id != self.victim.id {
            return Err(CrosstalkError::MissingBaseline {
                victim_id: self.victim.id.clone(),
            });
        }

        if self.contextual.victim_id != self.victim.id {
            return Err(CrosstalkError::MissingContextObservation {
                victim_id: self.victim.id.clone(),
            });
        }

        if !self
            .baseline
            .condition
            .is_comparable_with(&self.contextual.condition)
        {
            return Err(CrosstalkError::ConditionIdentityMismatch {
                baseline: self.baseline.condition.workload_id.clone(),
                contextual: self.contextual.condition.workload_id.clone(),
            });
        }

        Ok(())
    }

    /// Calculates the crosstalk result for this victim.
    pub fn analyze(
        &self,
        context_id: impl Into<String>,
        confidence_level: f64,
    ) -> Result<CrosstalkVictimResult, CrosstalkError> {
        self.validate()?;
        validate_confidence_level(confidence_level)?;

        let baseline = self.baseline.pooled()?;
        let contextual = self.contextual.pooled()?;

        analyze_pair(
            context_id.into(),
            &self.victim.id,
            baseline,
            contextual,
            confidence_level,
        )
    }
}

// ============================================================================
// Confidence interval
// ============================================================================

/// A confidence interval.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    /// Point estimate.
    pub estimate: f64,

    /// Lower endpoint.
    pub lower: f64,

    /// Upper endpoint.
    pub upper: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Whether the interval is an approximation.
    pub approximate: bool,
}

impl ConfidenceInterval {
    /// Creates a validated interval.
    pub fn new(
        estimate: f64,
        lower: f64,
        upper: f64,
        confidence_level: f64,
        approximate: bool,
    ) -> Result<Self, CrosstalkError> {
        validate_finite(estimate, "interval.estimate")?;
        validate_finite(lower, "interval.lower")?;
        validate_finite(upper, "interval.upper")?;
        validate_confidence_level(confidence_level)?;

        if lower > upper {
            return Err(CrosstalkError::IncompatibleConditions {
                reason: "confidence interval lower bound exceeds upper bound".to_owned(),
            });
        }

        Ok(Self {
            estimate,
            lower,
            upper,
            confidence_level,
            approximate,
        })
    }

    /// Returns whether zero lies inside the interval.
    #[must_use]
    pub fn contains_zero(&self) -> bool {
        self.lower <= 0.0 && self.upper >= 0.0
    }

    /// Returns interval width.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}

// ============================================================================
// Significance
// ============================================================================

/// Statistical significance classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Significance {
    /// The confidence interval contains zero.
    NotDetected,

    /// The confidence interval excludes zero in the harmful direction.
    Detected,

    /// The confidence interval excludes zero in the beneficial direction.
    Improvement,

    /// A statistic could not be classified.
    Indeterminate,
}

impl Significance {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotDetected => "not_detected",
            Self::Detected => "detected",
            Self::Improvement => "improvement",
            Self::Indeterminate => "indeterminate",
        }
    }
}

// ============================================================================
// Victim result
// ============================================================================

/// Crosstalk result for one victim in one contextual condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkVictimResult {
    /// Context identifier.
    pub context_id: String,

    /// Victim identifier.
    pub victim_id: String,

    /// Baseline success probability.
    pub baseline_success_probability: f64,

    /// Contextual success probability.
    pub contextual_success_probability: f64,

    /// Baseline error rate.
    pub baseline_error_rate: f64,

    /// Contextual error rate.
    pub contextual_error_rate: f64,

    /// Success-probability change:
    ///
    /// `contextual_success - baseline_success`.
    pub success_degradation: f64,

    /// Error-rate change:
    ///
    /// `contextual_error - baseline_error`.
    pub error_degradation: f64,

    /// Relative error degradation:
    ///
    /// `(contextual_error - baseline_error) / baseline_error`.
    ///
    /// `None` means the baseline error rate is exactly zero.
    pub relative_error_degradation: Option<f64>,

    /// Baseline success confidence interval.
    pub baseline_success_interval: ConfidenceInterval,

    /// Contextual success confidence interval.
    pub contextual_success_interval: ConfidenceInterval,

    /// Approximate confidence interval for the success difference.
    pub success_difference_interval: ConfidenceInterval,

    /// Approximate confidence interval for the error difference.
    pub error_difference_interval: ConfidenceInterval,

    /// Statistical classification.
    pub significance: Significance,

    /// Baseline shots.
    pub baseline_shots: u64,

    /// Contextual shots.
    pub contextual_shots: u64,

    /// Stable algorithm identifier.
    pub algorithm_id: String,
}

impl CrosstalkVictimResult {
    /// Returns true when contextual execution is worse according to the error
    /// metric.
    #[must_use]
    pub fn degraded(&self) -> bool {
        self.error_degradation > 0.0
    }

    /// Returns true when contextual execution improved the error metric.
    #[must_use]
    pub fn improved(&self) -> bool {
        self.error_degradation < 0.0
    }
}

// ============================================================================
// Context result
// ============================================================================

/// Complete result for one contextual condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkContextResult {
    /// Context definition.
    pub context: CrosstalkContext,

    /// Per-victim results.
    pub victims: Vec<CrosstalkVictimResult>,

    /// Weighted mean error degradation.
    pub mean_error_degradation: f64,

    /// Weighted mean relative error degradation where defined.
    pub mean_relative_error_degradation: Option<f64>,

    /// Fraction of victims with detected harmful degradation.
    pub degraded_victim_fraction: f64,

    /// Stable algorithm identifier.
    pub algorithm_id: String,
}

impl CrosstalkContextResult {
    /// Returns number of victims.
    #[must_use]
    pub fn victim_count(&self) -> usize {
        self.victims.len()
    }
}

// ============================================================================
// Correlation diagnostic
// ============================================================================

/// Optional joint-error diagnostic.
///
/// This is useful when the execution protocol supplies:
///
/// - victim A success;
/// - victim B success;
/// - joint success.
///
/// It allows the caller to detect whether contextual errors are correlated
/// beyond a simple product-of-marginals model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CorrelationDiagnostic {
    /// Probability that victim A succeeds.
    pub p_a: f64,

    /// Probability that victim B succeeds.
    pub p_b: f64,

    /// Probability that both succeed.
    pub p_ab: f64,

    /// Product-of-marginals expectation.
    pub expected_independent_joint_success: f64,

    /// Joint-success excess:
    ///
    /// `p_ab - p_a * p_b`.
    pub joint_success_excess: f64,

    /// Pearson-style covariance of Bernoulli success indicators.
    pub covariance: f64,

    /// Normalized correlation coefficient where defined.
    pub correlation_coefficient: Option<f64>,
}

impl CorrelationDiagnostic {
    /// Calculates a correlation diagnostic.
    pub fn new(
        p_a: f64,
        p_b: f64,
        p_ab: f64,
    ) -> Result<Self, CrosstalkError> {
        validate_probability(p_a)?;
        validate_probability(p_b)?;
        validate_probability(p_ab)?;

        let expected = p_a * p_b;
        let excess = p_ab - expected;
        let covariance = excess;

        let denominator = (p_a * (1.0 - p_a) * p_b * (1.0 - p_b)).sqrt();

        let correlation_coefficient = if denominator > 0.0 {
            Some(covariance / denominator)
        } else {
            None
        };

        if let Some(value) = correlation_coefficient {
            validate_finite(value, "correlation_coefficient")?;
        }

        Ok(Self {
            p_a,
            p_b,
            p_ab,
            expected_independent_joint_success: expected,
            joint_success_excess: excess,
            covariance,
            correlation_coefficient,
        })
    }
}

// ============================================================================
// Complete result
// ============================================================================

/// Complete crosstalk benchmark result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkResult {
    /// Result schema version.
    pub schema_version: u32,

    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Experiment identifier.
    pub experiment_id: String,

    /// Algorithm identifier.
    pub algorithm_id: String,

    /// Statistical confidence level.
    pub confidence_level: f64,

    /// Context results.
    pub contexts: Vec<CrosstalkContextResult>,

    /// Optional pairwise correlation diagnostics.
    pub correlations: BTreeMap<String, CorrelationDiagnostic>,
}

impl CrosstalkResult {
    /// Creates a result from context results.
    pub fn new(
        experiment_id: impl Into<String>,
        confidence_level: f64,
        contexts: Vec<CrosstalkContextResult>,
    ) -> Result<Self, CrosstalkError> {
        validate_confidence_level(confidence_level)?;

        if contexts.is_empty() {
            return Err(CrosstalkError::EmptyAggregation);
        }

        Ok(Self {
            schema_version: CROSSTALK_RESULT_SCHEMA_VERSION,
            benchmark_id: CROSSTALK_BENCHMARK_ID.to_owned(),
            experiment_id: experiment_id.into(),
            algorithm_id: CROSSTALK_ALGORITHM_ID.to_owned(),
            confidence_level,
            contexts,
            correlations: BTreeMap::new(),
        })
    }

    /// Adds a correlation diagnostic.
    pub fn add_correlation(
        &mut self,
        id: impl Into<String>,
        diagnostic: CorrelationDiagnostic,
    ) -> Result<(), CrosstalkError> {
        let id = id.into();
        validate_identifier(&id, "correlation_id")?;

        self.correlations.insert(id, diagnostic);
        Ok(())
    }

    /// Returns the worst contextual degradation.
    pub fn worst_context(&self) -> Option<&CrosstalkContextResult> {
        self.contexts.iter().max_by(|a, b| {
            a.mean_error_degradation
                .partial_cmp(&b.mean_error_degradation)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the best contextual result.
    pub fn best_context(&self) -> Option<&CrosstalkContextResult> {
        self.contexts.iter().min_by(|a, b| {
            a.mean_error_degradation
                .partial_cmp(&b.mean_error_degradation)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the fraction of contexts with at least one statistically
    /// detected harmful victim.
    pub fn affected_context_fraction(&self) -> Result<f64, CrosstalkError> {
        if self.contexts.is_empty() {
            return Err(CrosstalkError::EmptyAggregation);
        }

        let affected = self
            .contexts
            .iter()
            .filter(|context| {
                context
                    .victims
                    .iter()
                    .any(|victim| victim.significance == Significance::Detected)
            })
            .count();

        let value = affected as f64 / self.contexts.len() as f64;
        validate_probability(value)?;
        Ok(value)
    }
}

// ============================================================================
// Public analysis entry points
// ============================================================================

/// Analyze one baseline/context pair.
pub fn analyze_pair(
    context_id: impl Into<String>,
    victim_id: impl Into<String>,
    baseline: CrosstalkObservation,
    contextual: CrosstalkObservation,
    confidence_level: f64,
) -> Result<CrosstalkVictimResult, CrosstalkError> {
    validate_confidence_level(confidence_level)?;
    baseline.validate()?;
    contextual.validate()?;

    let context_id = context_id.into();
    let victim_id = victim_id.into();

    validate_identifier(&context_id, "context")?;
    validate_identifier(&victim_id, "victim")?;

    let baseline_success = baseline.success_probability()?;
    let contextual_success = contextual.success_probability()?;

    let baseline_error = baseline.error_rate()?;
    let contextual_error = contextual.error_rate()?;

    let success_degradation = contextual_success - baseline_success;
    let error_degradation = contextual_error - baseline_error;

    validate_finite(success_degradation, "success_degradation")?;
    validate_finite(error_degradation, "error_degradation")?;

    let relative_error_degradation = if baseline_error > 0.0 {
        let value = error_degradation / baseline_error;
        validate_finite(value, "relative_error_degradation")?;
        Some(value)
    } else {
        None
    };

    let baseline_interval =
        wilson_interval(baseline.successes, baseline.shots, confidence_level)?;

    let contextual_interval =
        wilson_interval(contextual.successes, contextual.shots, confidence_level)?;

    let success_difference_interval = difference_interval(
        baseline_success,
        contextual_success,
        baseline.shots,
        contextual.shots,
        confidence_level,
    )?;

    let error_difference_interval = difference_interval(
        baseline_error,
        contextual_error,
        baseline.shots,
        contextual.shots,
        confidence_level,
    )?;

    let significance = classify_difference(&error_difference_interval);

    Ok(CrosstalkVictimResult {
        context_id,
        victim_id,
        baseline_success_probability: baseline_success,
        contextual_success_probability: contextual_success,
        baseline_error_rate: baseline_error,
        contextual_error_rate: contextual_error,
        success_degradation,
        error_degradation,
        relative_error_degradation,
        baseline_success_interval: baseline_interval,
        contextual_success_interval: contextual_interval,
        success_difference_interval,
        error_difference_interval,
        significance,
        baseline_shots: baseline.shots,
        contextual_shots: contextual.shots,
        algorithm_id: CROSSTALK_ALGORITHM_ID.to_owned(),
    })
}

/// Analyze all victim pairs for one context.
pub fn analyze_context(
    context: CrosstalkContext,
    victims: &[CrosstalkVictim],
    baseline: &[VictimObservation],
    contextual: &[VictimObservation],
    confidence_level: f64,
) -> Result<CrosstalkContextResult, CrosstalkError> {
    context.validate()?;
    validate_confidence_level(confidence_level)?;

    if victims.is_empty() {
        return Err(CrosstalkError::NoVictims {
            context_id: context.id.clone(),
        });
    }

    if victims.len() > DEFAULT_MAX_VICTIMS_PER_CONTEXT {
        return Err(CrosstalkError::TooManyVictims {
            actual: victims.len(),
            maximum: DEFAULT_MAX_VICTIMS_PER_CONTEXT,
        });
    }

    let mut victim_map = BTreeMap::new();

    for victim in victims {
        victim.validate()?;

        if victim_map
            .insert(victim.id.clone(), victim)
            .is_some()
        {
            return Err(CrosstalkError::DuplicateVictimId {
                victim_id: victim.id.clone(),
            });
        }
    }

    let mut baseline_map = BTreeMap::new();

    for observation in baseline {
        observation.validate()?;

        if baseline_map
            .insert(observation.victim_id.clone(), observation)
            .is_some()
        {
            return Err(CrosstalkError::IncompatibleConditions {
                reason: format!(
                    "duplicate baseline observation for victim '{}'",
                    observation.victim_id
                ),
            });
        }
    }

    let mut contextual_map = BTreeMap::new();

    for observation in contextual {
        observation.validate()?;

        if contextual_map
            .insert(observation.victim_id.clone(), observation)
            .is_some()
        {
            return Err(CrosstalkError::IncompatibleConditions {
                reason: format!(
                    "duplicate contextual observation for victim '{}'",
                    observation.victim_id
                ),
            });
        }
    }

    let mut results = Vec::with_capacity(victims.len());

    for victim in victims {
        let baseline_observation = baseline_map
            .get(&victim.id)
            .ok_or_else(|| CrosstalkError::MissingBaseline {
                victim_id: victim.id.clone(),
            })?;

        let contextual_observation = contextual_map
            .get(&victim.id)
            .ok_or_else(|| CrosstalkError::MissingContextObservation {
                victim_id: victim.id.clone(),
            })?;

        let pair = CrosstalkPair::new(
            victim.clone(),
            (*baseline_observation).clone(),
            (*contextual_observation).clone(),
        )?;

        results.push(pair.analyze(
            context.id.clone(),
            confidence_level,
        )?);
    }

    let mean_error_degradation = weighted_mean(
        &results
            .iter()
            .map(|result| {
                (
                    result.error_degradation,
                    result
                        .baseline_shots
                        .saturating_add(result.contextual_shots)
                        as f64,
                )
            })
            .collect::<Vec<_>>(),
    )?;

    let relative_values = results
        .iter()
        .filter_map(|result| {
            result
                .relative_error_degradation
                .map(|value| {
                    (
                        value,
                        result
                            .baseline_shots
                            .saturating_add(result.contextual_shots)
                            as f64,
                    )
                })
        })
        .collect::<Vec<_>>();

    let mean_relative_error_degradation = if relative_values.is_empty() {
        None
    } else {
        Some(weighted_mean(&relative_values)?)
    };

    let degraded_count = results
        .iter()
        .filter(|result| result.significance == Significance::Detected)
        .count();

    let degraded_fraction = degraded_count as f64 / results.len() as f64;

    validate_probability(degraded_fraction)?;

    Ok(CrosstalkContextResult {
        context,
        victims: results,
        mean_error_degradation,
        mean_relative_error_degradation,
        degraded_victim_fraction: degraded_fraction,
        algorithm_id: CROSSTALK_ALGORITHM_ID.to_owned(),
    })
}

/// Analyze a complete configured crosstalk experiment.
///
/// `baseline` and `contextual` contain observations indexed by context/victim
/// through the supplied `CrosstalkObservationRecord`.
pub fn analyze_experiment(
    config: &CrosstalkConfig,
    records: &[CrosstalkObservationRecord],
) -> Result<CrosstalkResult, CrosstalkError> {
    config.validate()?;

    if records.is_empty() {
        return Err(CrosstalkError::EmptyAggregation);
    }

    let mut contexts = Vec::with_capacity(config.contexts.len());

    for context in &config.contexts {
        let context_records = records
            .iter()
            .filter(|record| record.context_id == context.id)
            .collect::<Vec<_>>();

        if context_records.is_empty() {
            return Err(CrosstalkError::MissingContextObservation {
                victim_id: context.id.clone(),
            });
        }

        let mut baseline = Vec::new();
        let mut contextual = Vec::new();

        for record in context_records {
            if record.contextual {
                contextual.push(record.observation.clone());
            } else {
                baseline.push(record.observation.clone());
            }
        }

        contexts.push(analyze_context(
            context.clone(),
            &config.victims,
            &baseline,
            &contextual,
            config.confidence_level,
        )?);
    }

    CrosstalkResult::new(
        config.experiment_id.clone(),
        config.confidence_level,
        contexts,
    )
}

// ============================================================================
// Execution-neutral observation record
// ============================================================================

/// Normalized record produced by the execution layer.
///
/// The execution subsystem can convert backend-specific results into this
/// structure without knowing anything about crosstalk mathematics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkObservationRecord {
    /// Context identifier.
    pub context_id: String,

    /// Whether this record belongs to the contextual experiment.
    ///
    /// `false` means isolated baseline.
    pub contextual: bool,

    /// Victim observation.
    pub observation: VictimObservation,
}

impl CrosstalkObservationRecord {
    /// Creates a record.
    pub fn new(
        context_id: impl Into<String>,
        contextual: bool,
        observation: VictimObservation,
    ) -> Result<Self, CrosstalkError> {
        let record = Self {
            context_id: context_id.into(),
            contextual,
            observation,
        };

        record.validate()?;
        Ok(record)
    }

    /// Validates the record.
    pub fn validate(&self) -> Result<(), CrosstalkError> {
        validate_identifier(&self.context_id, "context")?;
        self.observation.validate()?;
        Ok(())
    }
}

// ============================================================================
// Wilson interval
// ============================================================================

/// Calculates a Wilson score interval for a binomial proportion.
///
/// This is used for individual success-probability estimates because it
/// behaves substantially better than the simple Wald interval near 0 and 1.
pub fn wilson_interval(
    successes: u64,
    shots: u64,
    confidence_level: f64,
) -> Result<ConfidenceInterval, CrosstalkError> {
    if shots == 0 {
        return Err(CrosstalkError::ZeroShots);
    }

    if successes > shots {
        return Err(CrosstalkError::SuccessesExceedShots {
            successes,
            shots,
        });
    }

    validate_confidence_level(confidence_level)?;

    let p = successes as f64 / shots as f64;
    let z = normal_quantile(0.5 + confidence_level / 2.0)?;

    let n = shots as f64;
    let z2 = z * z;

    let denominator = 1.0 + z2 / n;
    let centre = (p + z2 / (2.0 * n)) / denominator;

    let variance = (p * (1.0 - p) / n) + (z2 / (4.0 * n * n));

    let margin = z * variance.sqrt() / denominator;

    let lower = (centre - margin).max(0.0);
    let upper = (centre + margin).min(1.0);

    ConfidenceInterval::new(
        p,
        lower,
        upper,
        confidence_level,
        false,
    )
}

// ============================================================================
// Difference interval
// ============================================================================

/// Calculates an approximate confidence interval for the difference of two
/// independent binomial proportions.
///
/// The result is explicitly marked `approximate`.
///
/// For paired experiments with circuit-by-circuit matched outcomes, the
/// execution/statistics layer should eventually supply a paired estimator.
/// This function intentionally does not claim to be an exact paired test.
pub fn difference_interval(
    baseline: f64,
    contextual: f64,
    baseline_shots: u64,
    contextual_shots: u64,
    confidence_level: f64,
) -> Result<ConfidenceInterval, CrosstalkError> {
    validate_probability(baseline)?;
    validate_probability(contextual)?;

    if baseline_shots == 0 || contextual_shots == 0 {
        return Err(CrosstalkError::ZeroShots);
    }

    validate_confidence_level(confidence_level)?;

    let estimate = contextual - baseline;

    let variance = baseline * (1.0 - baseline) / baseline_shots as f64
        + contextual * (1.0 - contextual) / contextual_shots as f64;

    let z = normal_quantile(0.5 + confidence_level / 2.0)?;

    let standard_error = variance.sqrt();

    let margin = z * standard_error;

    let lower = estimate - margin;
    let upper = estimate + margin;

    ConfidenceInterval::new(
        estimate,
        lower,
        upper,
        confidence_level,
        true,
    )
}

// ============================================================================
// Significance
// ============================================================================

fn classify_difference(interval: &ConfidenceInterval) -> Significance {
    if interval.lower > 0.0 {
        Significance::Detected
    } else if interval.upper < 0.0 {
        Significance::Improvement
    } else {
        Significance::NotDetected
    }
}

// ============================================================================
// Aggregation
// ============================================================================

/// Weighted arithmetic mean.
pub fn weighted_mean(values: &[(f64, f64)]) -> Result<f64, CrosstalkError> {
    if values.is_empty() {
        return Err(CrosstalkError::EmptyAggregation);
    }

    let mut weighted_sum = 0.0;
    let mut total_weight = 0.0;

    for &(value, weight) in values {
        validate_finite(value, "aggregation.value")?;
        validate_finite(weight, "aggregation.weight")?;

        if weight <= 0.0 {
            return Err(CrosstalkError::ZeroTotalWeight);
        }

        weighted_sum += value * weight;
        total_weight += weight;
    }

    if total_weight <= 0.0 {
        return Err(CrosstalkError::ZeroTotalWeight);
    }

    let result = weighted_sum / total_weight;

    validate_finite(result, "weighted_mean")?;

    Ok(result)
}

// ============================================================================
// Normal quantile
// ============================================================================

/// Calculates the standard-normal inverse CDF.
///
/// This implementation uses the Peter John Acklam rational approximation.
/// It avoids adding a statistical dependency solely for this protocol.
///
/// Accuracy is sufficient for benchmark confidence intervals across the
/// supported confidence range.
fn normal_quantile(probability: f64) -> Result<f64, CrosstalkError> {
    validate_finite(probability, "normal_probability")?;

    if !(0.0 < probability && probability < 1.0) {
        return Err(CrosstalkError::InvalidNormalQuantile {
            confidence: probability,
        });
    }

    // Coefficients for the Acklam approximation.
    const A1: f64 = -3.969_683_028_665_376e1;
    const A2: f64 = 2.209_460_984_245_205e2;
    const A3: f64 = -2.759_285_104_469_687e2;
    const A4: f64 = 1.383_577_518_672_69e2;
    const A5: f64 = -3.066_479_806_614_716e1;
    const A6: f64 = 2.506_628_277_459_239;

    const B1: f64 = -5.447_609_879_822_406e1;
    const B2: f64 = 1.615_858_368_580_409e2;
    const B3: f64 = -1.556_989_798_598_866e2;
    const B4: f64 = 6.680_131_188_771_972e1;
    const B5: f64 = -1.328_068_155_288_572e1;

    const C1: f64 = -7.784_894_002_430_293e-3;
    const C2: f64 = -3.223_964_580_411_365e-1;
    const C3: f64 = -2.400_758_277_161_838;
    const C4: f64 = -2.549_732_539_343_734;
    const C5: f64 = 4.374_664_141_464_968;
    const C6: f64 = 2.938_163_982_698_783;

    const D1: f64 = 7.784_695_709_041_462e-3;
    const D2: f64 = 3.224_671_290_700_398e-1;
    const D3: f64 = 2.445_134_137_142_996;
    const D4: f64 = 3.754_408_661_907_416;

    const LOWER: f64 = 0.024_25;
    const UPPER: f64 = 1.0 - LOWER;

    let result = if probability < LOWER {
        let q = (-2.0 * probability.ln()).sqrt();

        (((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    } else if probability <= UPPER {
        let q = probability - 0.5;
        let r = q * q;

        (((((A1 * r + A2) * r + A3) * r + A4) * r + A5) * r + A6)
            * q
            / (((((B1 * r + B2) * r + B3) * r + B4) * r + B5) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - probability).ln()).sqrt();

        -(((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    };

    if result.is_finite() {
        Ok(result)
    } else {
        Err(CrosstalkError::InvalidNormalQuantile {
            confidence: probability,
        })
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), CrosstalkError> {
    if value.trim().is_empty() {
        return match field {
            "context" => Err(CrosstalkError::EmptyContextId),
            "victim" => Err(CrosstalkError::EmptyVictimId),
            _ => Err(CrosstalkError::MissingConditionIdentity),
        };
    }

    if value.len() > 512 {
        if field == "context" {
            return Err(CrosstalkError::ContextIdTooLong {
                length: value.len(),
            });
        }

        return Err(CrosstalkError::IncompatibleConditions {
            reason: format!(
                "{} identifier exceeds maximum length",
                field
            ),
        });
    }

    Ok(())
}

fn validate_finite(
    value: f64,
    field: &'static str,
) -> Result<(), CrosstalkError> {
    if !value.is_finite() {
        return Err(CrosstalkError::NonFiniteValue { field, value });
    }

    Ok(())
}

fn validate_probability(value: f64) -> Result<(), CrosstalkError> {
    validate_finite(value, "probability")?;

    if value < -UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(CrosstalkError::InvalidProbability { value });
    }

    Ok(())
}

fn validate_error_rate(value: f64) -> Result<(), CrosstalkError> {
    validate_finite(value, "error_rate")?;

    if value < -UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(CrosstalkError::InvalidErrorRate { value });
    }

    Ok(())
}

fn validate_confidence_level(
    value: f64,
) -> Result<(), CrosstalkError> {
    validate_finite(value, "confidence_level")?;

    if value + CONFIDENCE_EPSILON < MIN_CONFIDENCE_LEVEL
        || value - CONFIDENCE_EPSILON > MAX_CONFIDENCE_LEVEL
    {
        return Err(CrosstalkError::InvalidConfidenceLevel {
            value,
        });
    }

    Ok(())
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn condition() -> ConditionIdentity {
        ConditionIdentity {
            workload_id: "workload".to_owned(),
            compiler_id: "compiler".to_owned(),
            routing_id: "routing".to_owned(),
            scheduling_id: "schedule".to_owned(),
            calibration_id: "calibration".to_owned(),
            backend_id: "backend".to_owned(),
            measurement_id: "measurement".to_owned(),
            condition_hash: Some("condition".to_owned()),
        }
    }

    fn victim() -> CrosstalkVictim {
        CrosstalkVictim::new(
            "q0",
            vec![0],
            Some("x".to_owned()),
        )
        .expect("valid victim")
    }

    #[test]
    fn observation_probability_is_correct() {
        let observation =
            CrosstalkObservation::new(900, 1000).expect("valid observation");

        let probability =
            observation.success_probability().expect("probability");

        assert!((probability - 0.9).abs() < 1.0e-12);
    }

    #[test]
    fn observation_error_is_correct() {
        let observation =
            CrosstalkObservation::new(900, 1000).expect("valid observation");

        let error =
            observation.error_rate().expect("error");

        assert!((error - 0.1).abs() < 1.0e-12);
    }

    #[test]
    fn zero_shots_are_rejected() {
        let result = CrosstalkObservation::new(0, 0);

        assert!(matches!(
            result,
            Err(CrosstalkError::ZeroShots)
        ));
    }

    #[test]
    fn successes_above_shots_are_rejected() {
        let result = CrosstalkObservation::new(101, 100);

        assert!(matches!(
            result,
            Err(CrosstalkError::SuccessesExceedShots { .. })
        ));
    }

    #[test]
    fn wilson_interval_is_inside_unit_interval() {
        let interval =
            wilson_interval(900, 1000, 0.95).expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower <= interval.upper);
        assert!((interval.estimate - 0.9).abs() < 1.0e-12);
    }

    #[test]
    fn perfect_success_has_valid_interval() {
        let interval =
            wilson_interval(1000, 1000, 0.95).expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!((interval.estimate - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn difference_interval_contains_zero_for_equal_rates() {
        let interval =
            difference_interval(0.9, 0.9, 1000, 1000, 0.95)
                .expect("valid interval");

        assert!(interval.contains_zero());
    }

    #[test]
    fn crosstalk_is_detected_when_error_increases() {
        let baseline =
            CrosstalkObservation::new(9900, 10_000)
                .expect("baseline");

        let contextual =
            CrosstalkObservation::new(9700, 10_000)
                .expect("context");

        let result = analyze_pair(
            "context",
            "q0",
            baseline,
            contextual,
            0.95,
        )
        .expect("analysis");

        assert!(result.error_degradation > 0.0);
        assert!(result.success_degradation < 0.0);
        assert!(result.degraded());
    }

    #[test]
    fn crosstalk_improvement_is_detectable() {
        let baseline =
            CrosstalkObservation::new(9700, 10_000)
                .expect("baseline");

        let contextual =
            CrosstalkObservation::new(9900, 10_000)
                .expect("context");

        let result = analyze_pair(
            "context",
            "q0",
            baseline,
            contextual,
            0.95,
        )
        .expect("analysis");

        assert!(result.error_degradation < 0.0);
        assert!(result.success_degradation > 0.0);
        assert!(result.improved());
    }

    #[test]
    fn relative_degradation_is_calculated() {
        let baseline =
            CrosstalkObservation::new(9900, 10_000)
                .expect("baseline");

        let contextual =
            CrosstalkObservation::new(9800, 10_000)
                .expect("context");

        let result = analyze_pair(
            "context",
            "q0",
            baseline,
            contextual,
            0.95,
        )
        .expect("analysis");

        let relative =
            result
                .relative_error_degradation
                .expect("relative degradation");

        assert!((relative - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn zero_baseline_error_has_no_relative_degradation() {
        let baseline =
            CrosstalkObservation::new(10_000, 10_000)
                .expect("baseline");

        let contextual =
            CrosstalkObservation::new(9990, 10_000)
                .expect("context");

        let result = analyze_pair(
            "context",
            "q0",
            baseline,
            contextual,
            0.95,
        )
        .expect("analysis");

        assert!(result.relative_error_degradation.is_none());
    }

    #[test]
    fn context_cannot_overlap_victim_by_default() {
        let victim = victim();

        let context = CrosstalkContext::new(
            "active-q0",
            ContextRole::ActiveOperation,
            Some("x".to_owned()),
            vec![0],
        )
        .expect("valid context");

        let mut config = CrosstalkConfig::new("experiment");

        config
            .add_victim(victim)
            .expect("victim accepted");

        config
            .add_context(context)
            .expect("context accepted");

        assert!(matches!(
            config.validate(),
            Err(CrosstalkError::VictimContextOverlap { .. })
        ));
    }

    #[test]
    fn distinct_victim_and_context_are_valid() {
        let victim = victim();

        let context = CrosstalkContext::new(
            "active-q1",
            ContextRole::ActiveOperation,
            Some("x".to_owned()),
            vec![1],
        )
        .expect("valid context");

        let mut config = CrosstalkConfig::new("experiment");

        config
            .add_victim(victim)
            .expect("victim accepted");

        config
            .add_context(context)
            .expect("context accepted");

        assert!(config.validate().is_ok());
    }

    #[test]
    fn correlation_for_independent_probabilities_is_zero() {
        let diagnostic =
            CorrelationDiagnostic::new(0.9, 0.8, 0.72)
                .expect("valid correlation");

        assert!(diagnostic.joint_success_excess.abs() < 1.0e-12);
        assert!(diagnostic.covariance.abs() < 1.0e-12);
    }

    #[test]
    fn correlation_detects_positive_joint_excess() {
        let diagnostic =
            CorrelationDiagnostic::new(0.9, 0.8, 0.78)
                .expect("valid correlation");

        assert!(diagnostic.joint_success_excess > 0.0);
        assert!(
            diagnostic
                .correlation_coefficient
                .expect("correlation")
                > 0.0
        );
    }

    #[test]
    fn pooled_observation_is_correct() {
        let observation = VictimObservation::new(
            "q0",
            condition(),
            vec![
                CrosstalkObservation::new(90, 100)
                    .expect("observation"),
                CrosstalkObservation::new(180, 200)
                    .expect("observation"),
            ],
        )
        .expect("victim observation");

        let pooled =
            observation.pooled().expect("pooled");

        assert_eq!(pooled.successes, 270);
        assert_eq!(pooled.shots, 300);
    }

    #[test]
    fn normal_quantile_is_reasonable() {
        let q95 =
            normal_quantile(0.975).expect("quantile");

        assert!((q95 - 1.959963984).abs() < 1.0e-6);
    }

    #[test]
    fn weighted_mean_is_correct() {
        let value =
            weighted_mean(&[(0.1, 1.0), (0.3, 3.0)])
                .expect("mean");

        assert!((value - 0.25).abs() < 1.0e-12);
    }

    #[test]
    fn context_analysis_requires_matching_conditions() {
        let victim = victim();

        let mut contextual_condition = condition();
        contextual_condition.backend_id = "different-backend".to_owned();

        let baseline = vec![
            VictimObservation::new(
                "q0",
                condition(),
                vec![
                    CrosstalkObservation::new(990, 1000)
                        .expect("observation"),
                ],
            )
            .expect("baseline"),
        ];

        let contextual = vec![
            VictimObservation::new(
                "q0",
                contextual_condition,
                vec![
                    CrosstalkObservation::new(980, 1000)
                        .expect("observation"),
                ],
            )
            .expect("context"),
        ];

        let context = CrosstalkContext::new(
            "q1-active",
            ContextRole::ActiveOperation,
            Some("x".to_owned()),
            vec![1],
        )
        .expect("context");

        let result =
            analyze_context(
                context,
                &[victim],
                &baseline,
                &contextual,
                0.95,
            );

        assert!(matches!(
            result,
            Err(CrosstalkError::ConditionIdentityMismatch { .. })
        ));
    }
}