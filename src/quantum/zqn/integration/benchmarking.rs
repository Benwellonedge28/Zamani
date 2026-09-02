//! Zamani Quantum Noise (ZQN)
//! Benchmarking Integration Boundary
//!
//! # Purpose
//!
//! This module is the stable integration boundary between:
//!
//! ```text
//! quantum::zqn
//!      │
//!      │ physical noise, uncertainty, calibration,
//!      │ characterization and observations
//!      ▼
//! zqn::integration::benchmarking
//!      │
//!      │ normalized benchmark observations
//!      ▼
//! quantum::benchmarking
//! ```
//!
//! The module deliberately does NOT implement benchmark protocols.
//!
//! Benchmark methodology remains owned by `quantum::benchmarking`.
//! ZQN remains the owner of noise semantics.
//!
//! # Architectural rule
//!
//! ```text
//! Benchmarking asks:
//!     "What was observed and how statistically reliable is it?"
//!
//! ZQN answers:
//!     "What physical noise/uncertainty model or observation describes it?"
//! ```
//!
//! This file connects those domains without transferring ownership.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the ZQN-to-benchmark observation boundary;
//! - immutable observation envelopes;
//! - target/calibration/model identity references;
//! - canonical quantum-resource references;
//! - operation identity references;
//! - measurement provenance;
//! - realization/approximation declarations;
//! - deterministic observation ordering;
//! - validation of integration-level invariants;
//! - conversion of ZQN observations into a benchmark-neutral representation;
//! - benchmark-facing collection/sink traits;
//! - partial-result-safe ingestion;
//! - integration-level resource policies.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - Quantum Volume methodology;
//! - randomized benchmarking methodology;
//! - XEB methodology;
//! - circuit generation;
//! - benchmark workload generation;
//! - statistical estimators;
//! - confidence-interval algorithms;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - calibration mathematics;
//! - quantum-channel mathematics;
//! - noise-model mathematics;
//! - canonical Quantum IR;
//! - serialization formats;
//! - vendor SDKs;
//! - network transport;
//! - credentials;
//! - process-global state.
//!
//! # Canonical identities
//!
//! Logical and physical qubit identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module must never introduce another `QubitId` or `PhysicalQubitId`.
//!
//! Operation identity likewise comes from the canonical IR identity subsystem.
//!
//! # Write once, scale everywhere
//!
//! There is no architectural maximum for:
//!
//! - qubits;
//! - physical resources;
//! - operations;
//! - observations;
//! - benchmark dimensions;
//! - experiments;
//! - shots;
//! - targets;
//! - resources per observation.
//!
//! Collections are dynamically sized.
//!
//! Resource limits, when required, are supplied explicitly by the caller.
//!
//! Therefore:
//!
//! ```text
//! tiny machine
//!      │
//!      ├── same API
//!      │
//!      ▼
//! large machine
//!      │
//!      ├── same API
//!      │
//!      ▼
//! distributed machine
//!      │
//!      └── same API
//! ```
//!
//! "Infinity" means that this API contains no artificial finite machine-size
//! ceiling. Actual execution remains constrained by available memory, storage,
//! compute, hardware, transport and caller-selected resource policies.
//!
//! # Determinism
//!
//! This module:
//!
//! - has no global mutable state;
//! - has no hidden random generator;
//! - does not read the system clock;
//! - does not perform implicit I/O;
//! - uses deterministic ordered collections;
//! - preserves caller-provided identities;
//! - never derives semantic identity from memory addresses;
//! - never uses thread identity as semantic identity.
//!
//! The same observation sequence supplied to the same ingestion policy produces
//! the same normalized representation.
//!
//! # Statistical semantics
//!
//! A benchmark observation is not represented merely as a floating-point
//! number.
//!
//! The integration envelope can retain:
//!
//! ```text
//! value
//! uncertainty
//! confidence
//! sample count
//! method
//! unit
//! realization mode
//! target identity
//! calibration identity
//! noise-model identity
//! provenance
//! ```
//!
//! The actual statistical estimator remains outside this module.
//!
//! # Approximation semantics
//!
//! ZQN-to-benchmark integration must never silently downgrade semantics.
//!
//! Every observation may explicitly declare:
//!
//! ```text
//! Exact
//! Approximate
//! Bounded
//! Statistical
//! ```
//!
//! An approximation carries an explicit error tolerance.
//!
//! A bound carries an explicit error bound.
//!
//! A statistical result carries an explicit confidence level.
//!
//! # Partial execution
//!
//! Hardware can fail after collecting part of a benchmark.
//!
//! Consequently, the ingestion API accepts observations individually and does
//! not require an entire benchmark to be present in memory before processing.
//!
//! A caller may therefore implement:
//!
//! ```text
//! hardware execution
//!       │
//!       ├── observation 1 ──► sink
//!       ├── observation 2 ──► sink
//!       ├── observation 3 ──► sink
//!       │
//!       └── hardware failure
//!                  │
//!                  ▼
//!             partial result
//! ```
//!
//! No previously accepted observation needs to be discarded merely because a
//! later observation failed.
//!
//! # Integration with ZQN
//!
//! ZQN characterization/calibration modules can construct
//! `BenchmarkObservation` values from their validated observations.
//!
//! The integration layer deliberately accepts a trait-based provider rather
//! than depending on concrete future ZQN characterization implementations.
//! This prevents circular dependencies.
//!
//! # Integration with quantum::benchmarking
//!
//! The benchmarking subsystem can consume:
//!
//! ```text
//! BenchmarkObservation
//! BenchmarkObservationBatch
//! BenchmarkObservationSource
//! BenchmarkObservationSink
//! BenchmarkTarget
//! BenchmarkResource
//! ```
//!
//! The benchmark protocol remains responsible for interpreting those
//! observations.
//!
//! # Integration with Quantum Volume
//!
//! `volume_estimator.rs` remains a pure mathematical/statistical component.
//!
//! This module may supply it indirectly through a future protocol layer:
//!
//! ```text
//! ZQN observation
//!      │
//!      ▼
//! benchmarking observation
//!      │
//!      ▼
//! QV protocol
//!      │
//!      ▼
//! volume_estimator
//! ```
//!
//! The QV estimator itself does not need to depend on ZQN.
//!
//! # Integration with calibration
//!
//! A benchmark observation may carry an opaque calibration identity and
//! revision. It must never silently select "the newest" calibration.
//!
//! This agrees with the ZQN calibration contract: calibration snapshots are
//! immutable and explicit, and benchmark results should record which snapshot
//! was active.
//!
//! # Integration with characterization
//!
//! Characterization produces physical observations.
//!
//! This module provides the benchmark-facing envelope for those observations.
//!
//! Characterization methodology remains owned by:
//!
//! ```text
//! quantum::zqn::characterization
//! ```
//!
//! # Integration with hardware
//!
//! Hardware adapters provide observations through provider-neutral structures.
//!
//! This file contains no vendor API and no transport implementation.
//!
//! # Integration with routing/scheduling
//!
//! Routing and scheduling may provide operation/resource context that becomes
//! part of an observation. This module does not calculate routing or scheduling
//! costs.
//!
//! # Integration with QEC
//!
//! Physical and logical error observations may be benchmarked through the same
//! envelope. QEC remains responsible for decoding, correction and logical
//! fault-tolerance semantics.
//!
//! # Serialization
//!
//! This module intentionally defines an in-memory contract only.
//!
//! `zqn::io` or the benchmarking reporting subsystem owns serialization.
//!
//! Internal Rust layout must never become the external wire-format contract.
//!
//! # Security
//!
//! Observation data can be untrusted when loaded from:
//!
//! - hardware;
//! - files;
//! - remote services;
//! - benchmark artifacts;
//! - CI;
//! - user-provided configurations.
//!
//! Validation therefore rejects:
//!
//! - empty required identities;
//! - non-finite numerical values;
//! - invalid probabilities;
//! - invalid confidence levels;
//! - invalid uncertainty bounds;
//! - inconsistent sample counts;
//! - duplicate resources;
//! - invalid resource limits.
//!
//! No observation can execute code.
//!
//! # Resource safety
//!
//! This module does not impose semantic maximums.
//!
//! `BenchmarkIntegrationLimits` provides caller-selected safety policies.
//!
//! `None` means that this integration layer does not impose that particular
//! limit.
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
//! - no unsafe code.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! 1. it compiles independently once `integration/mod.rs` exposes it;
//! 2. it does not depend on future benchmarking modules;
//! 3. it uses canonical IR identities;
//! 4. it contains no vendor APIs;
//! 5. it contains no benchmark methodology;
//! 6. it contains no global mutable state;
//! 7. it has deterministic collection ordering;
//! 8. it explicitly represents uncertainty;
//! 9. it explicitly represents approximation;
//! 10. it supports partial observation ingestion;
//! 11. it has no semantic machine-size ceiling;
//! 12. all externally supplied values are validated;
//! 13. later benchmarking/ZQN modules can consume it without changing this
//!     contract.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Schema
// =============================================================================

/// Version of the in-memory ZQN benchmarking integration contract.
pub const BENCHMARKING_INTEGRATION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type for benchmarking integration operations.
pub type BenchmarkingIntegrationResult<T> = Result<T, BenchmarkingIntegrationError>;

/// Errors produced at the ZQN/benchmarking integration boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkingIntegrationError {
    /// A required identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// A supplied string was too large for the configured integration policy.
    IdentifierTooLarge {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// A numerical value was NaN or infinite.
    NonFiniteValue {
        field: &'static str,
        value: f64,
    },

    /// A probability was outside [0, 1].
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// A confidence level was outside (0, 1).
    InvalidConfidence {
        value: f64,
    },

    /// An uncertainty was negative.
    NegativeUncertainty {
        field: &'static str,
        value: f64,
    },

    /// An uncertainty exceeds the declared value range.
    InconsistentInterval {
        value: f64,
        uncertainty: f64,
    },

    /// A sample count was zero where samples are required.
    ZeroSampleCount,

    /// A resource appeared more than once.
    DuplicateResource,

    /// A logical qubit was mapped to multiple physical resources.
    DuplicateLogicalQubit {
        qubit: QubitId,
    },

    /// A physical qubit was mapped to multiple logical resources in one
    /// point-in-time mapping.
    DuplicatePhysicalQubit {
        qubit: PhysicalQubitId,
    },

    /// A requested operation was not valid.
    InvalidOperationId,

    /// A caller-provided collection exceeded its explicit policy.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: u128,
        maximum: u128,
    },

    /// An observation did not satisfy its declared invariants.
    InvalidObservation {
        reason: String,
    },

    /// A batch was inconsistent.
    InvalidBatch {
        reason: String,
    },
}

impl fmt::Display for BenchmarkingIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{} must not be empty", field)
            }

            Self::IdentifierTooLarge {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{} length {} exceeds configured maximum {}",
                    field, length, maximum
                )
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "{} must be finite, got {}",
                    field, value
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "{} must be finite and in [0, 1], got {}",
                    field, value
                )
            }

            Self::InvalidConfidence { value } => {
                write!(
                    formatter,
                    "confidence level must be finite and in (0, 1), got {}",
                    value
                )
            }

            Self::NegativeUncertainty { field, value } => {
                write!(
                    formatter,
                    "{} must not be negative, got {}",
                    field, value
                )
            }

            Self::InconsistentInterval {
                value,
                uncertainty,
            } => {
                write!(
                    formatter,
                    "value {} and uncertainty {} form an invalid interval",
                    value, uncertainty
                )
            }

            Self::ZeroSampleCount => {
                write!(formatter, "sample count must be greater than zero")
            }

            Self::DuplicateResource => {
                write!(formatter, "the same resource was supplied more than once")
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical qubit {:?} occurs more than once in a mapping",
                    qubit
                )
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "physical qubit {:?} is assigned more than once",
                    qubit
                )
            }

            Self::InvalidOperationId => {
                write!(formatter, "operation identity is invalid")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{} requested value {} exceeds configured maximum {}",
                    resource, requested, maximum
                )
            }

            Self::InvalidObservation { reason } => {
                write!(formatter, "invalid benchmark observation: {}", reason)
            }

            Self::InvalidBatch { reason } => {
                write!(formatter, "invalid benchmark observation batch: {}", reason)
            }
        }
    }
}

impl std::error::Error for BenchmarkingIntegrationError {}

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable opaque identity for a benchmark target.
///
/// This is deliberately not a vendor/device-name abstraction. A target may be
/// a simulator, emulator, physical QPU, logical-QPU service, analog machine,
/// distributed system, or future quantum technology.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BenchmarkTargetId(String);

impl BenchmarkTargetId {
    /// Creates a target identity.
    pub fn new(value: impl Into<String>) -> BenchmarkingIntegrationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "target_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the underlying stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BenchmarkTargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque identity for a ZQN noise model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoiseModelRef(String);

impl NoiseModelRef {
    /// Creates a noise-model identity.
    pub fn new(value: impl Into<String>) -> BenchmarkingIntegrationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "noise_model_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Opaque identity for a calibration snapshot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalibrationRef(String);

impl CalibrationRef {
    /// Creates a calibration identity.
    pub fn new(value: impl Into<String>) -> BenchmarkingIntegrationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "calibration_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Opaque identity for benchmark provenance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceRef(String);

impl ProvenanceRef {
    /// Creates a provenance identity.
    pub fn new(value: impl Into<String>) -> BenchmarkingIntegrationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "provenance_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Resource identity
// =============================================================================

/// A quantum resource participating in a benchmark observation.
///
/// Qubits use the canonical Quantum IR identities. Non-qubit resources remain
/// extensible through an opaque namespace/id pair.
///
/// This allows the same benchmark integration boundary to represent future
/// modalities without inventing additional ZQN resource identity types.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BenchmarkResource {
    /// Logical quantum resource.
    LogicalQubit(QubitId),

    /// Physical quantum resource.
    PhysicalQubit(PhysicalQubitId),

    /// Extensible non-qubit resource.
    Opaque {
        /// Resource namespace.
        namespace: String,

        /// Stable resource identifier within that namespace.
        id: String,
    },
}

impl BenchmarkResource {
    /// Creates an opaque resource reference.
    pub fn opaque(
        namespace: impl Into<String>,
        id: impl Into<String>,
    ) -> BenchmarkingIntegrationResult<Self> {
        let namespace = namespace.into();
        let id = id.into();

        if namespace.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "resource.namespace",
            });
        }

        if id.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "resource.id",
            });
        }

        Ok(Self::Opaque { namespace, id })
    }
}

// =============================================================================
// Logical-to-physical mapping
// =============================================================================

/// A deterministic logical-to-physical mapping snapshot.
///
/// The mapping belongs to the compilation/routing/hardware boundary; this
/// integration type only transports the mapping into benchmark provenance.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BenchmarkResourceMapping {
    logical_to_physical: BTreeMap<QubitId, PhysicalQubitId>,
}

impl BenchmarkResourceMapping {
    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a logical-to-physical mapping.
    ///
    /// A physical resource can belong to only one logical resource in a
    /// point-in-time mapping.
    pub fn insert(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> BenchmarkingIntegrationResult<()> {
        if self.logical_to_physical.contains_key(&logical) {
            return Err(
                BenchmarkingIntegrationError::DuplicateLogicalQubit {
                    qubit: logical,
                },
            );
        }

        if self.logical_to_physical.values().any(|value| *value == physical) {
            return Err(
                BenchmarkingIntegrationError::DuplicatePhysicalQubit {
                    qubit: physical,
                },
            );
        }

        self.logical_to_physical.insert(logical, physical);
        Ok(())
    }

    /// Returns the physical resource assigned to a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Returns the number of mapped resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Iterates in deterministic logical-qubit order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&QubitId, &PhysicalQubitId)> {
        self.logical_to_physical.iter()
    }
}

// =============================================================================
// Measurement values
// =============================================================================

/// A benchmark measurement value.
///
/// A benchmark observation may represent a probability, count, duration,
/// error estimate, fidelity, or another scalar quantity. The semantic metric
/// identifier determines interpretation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BenchmarkMeasurement {
    /// Scalar value without an intrinsic [0, 1] constraint.
    Scalar(f64),

    /// Probability constrained to [0, 1].
    Probability(f64),

    /// Non-negative sample/event count.
    Count(u128),
}

impl BenchmarkMeasurement {
    /// Validates the measurement.
    pub fn validate(
        &self,
        field: &'static str,
    ) -> BenchmarkingIntegrationResult<()> {
        match self {
            Self::Scalar(value) => validate_finite(field, *value),

            Self::Probability(value) => {
                validate_probability(field, *value)
            }

            Self::Count(_) => Ok(()),
        }
    }

    /// Returns a scalar representation when one exists.
    #[must_use]
    pub fn as_f64(self) -> Option<f64> {
        match self {
            Self::Scalar(value) | Self::Probability(value) => Some(value),
            Self::Count(value) => {
                if value <= u128::from(u64::MAX) {
                    Some(value as f64)
                } else {
                    None
                }
            }
        }
    }
}

// =============================================================================
// Uncertainty
// =============================================================================

/// Explicit uncertainty attached to a benchmark measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BenchmarkUncertainty {
    /// No uncertainty was supplied.
    None,

    /// Symmetric absolute uncertainty.
    Absolute(f64),

    /// Explicit lower/upper interval.
    Interval {
        /// Lower bound.
        lower: f64,

        /// Upper bound.
        upper: f64,
    },

    /// Statistical confidence interval.
    ConfidenceInterval {
        /// Lower bound.
        lower: f64,

        /// Upper bound.
        upper: f64,

        /// Confidence level in (0, 1).
        confidence: f64,
    },
}

impl BenchmarkUncertainty {
    /// Validates the uncertainty.
    pub fn validate(&self) -> BenchmarkingIntegrationResult<()> {
        match self {
            Self::None => Ok(()),

            Self::Absolute(value) => {
                if !value.is_finite() {
                    return Err(BenchmarkingIntegrationError::NonFiniteValue {
                        field: "uncertainty",
                        value: *value,
                    });
                }

                if *value < 0.0 {
                    return Err(
                        BenchmarkingIntegrationError::NegativeUncertainty {
                            field: "uncertainty",
                            value: *value,
                        },
                    );
                }

                Ok(())
            }

            Self::Interval { lower, upper } => {
                validate_finite("uncertainty.lower", *lower)?;
                validate_finite("uncertainty.upper", *upper)?;

                if lower > upper {
                    return Err(
                        BenchmarkingIntegrationError::InconsistentInterval {
                            value: *lower,
                            uncertainty: *upper,
                        },
                    );
                }

                Ok(())
            }

            Self::ConfidenceInterval {
                lower,
                upper,
                confidence,
            } => {
                validate_finite("uncertainty.lower", *lower)?;
                validate_finite("uncertainty.upper", *upper)?;

                if lower > upper {
                    return Err(
                        BenchmarkingIntegrationError::InconsistentInterval {
                            value: *lower,
                            uncertainty: *upper,
                        },
                    );
                }

                validate_confidence(*confidence)
            }
        }
    }
}

// =============================================================================
// Realization semantics
// =============================================================================

/// Declares how faithfully the observation represents the requested semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RealizationMode {
    /// Observation corresponds to the requested semantics without an
    /// approximation declaration.
    Exact,

    /// Observation uses an explicit approximation tolerance.
    Approximate {
        /// Maximum declared approximation error.
        tolerance: f64,
    },

    /// Observation carries an explicit deterministic error bound.
    Bounded {
        /// Upper error bound.
        error_bound: f64,
    },

    /// Observation is statistically estimated.
    Statistical {
        /// Confidence associated with the statistical claim.
        confidence: f64,
    },
}

impl RealizationMode {
    /// Validates the realization contract.
    pub fn validate(&self) -> BenchmarkingIntegrationResult<()> {
        match self {
            Self::Exact => Ok(()),

            Self::Approximate { tolerance } => {
                validate_non_negative_finite("tolerance", *tolerance)
            }

            Self::Bounded { error_bound } => {
                validate_non_negative_finite("error_bound", *error_bound)
            }

            Self::Statistical { confidence } => {
                validate_confidence(*confidence)
            }
        }
    }
}

// =============================================================================
// Observation metadata
// =============================================================================

/// Immutable identity/context attached to an observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkObservationContext {
    /// Target on which the observation was produced.
    pub target: BenchmarkTargetId,

    /// Optional calibration snapshot.
    pub calibration: Option<CalibrationRef>,

    /// Optional ZQN noise model identity.
    pub noise_model: Option<NoiseModelRef>,

    /// Optional provenance identity.
    pub provenance: Option<ProvenanceRef>,

    /// Optional benchmark-defined execution identity.
    ///
    /// This remains opaque so this module does not create a second benchmark
    /// experiment identity system.
    pub execution_id: Option<String>,
}

impl BenchmarkObservationContext {
    /// Validates required identity fields.
    pub fn validate(
        &self,
    ) -> BenchmarkingIntegrationResult<()> {
        if self.target.as_str().trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "target_id",
            });
        }

        if let Some(execution_id) = &self.execution_id {
            if execution_id.trim().is_empty() {
                return Err(
                    BenchmarkingIntegrationError::EmptyIdentifier {
                        field: "execution_id",
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Observation
// =============================================================================

/// One immutable normalized ZQN observation suitable for benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkObservation {
    /// Integration schema version.
    pub schema_version: u16,

    /// Stable metric identifier.
    ///
    /// Examples include `gate_error`, `readout_error`, `fidelity`,
    /// `heavy_output_probability`, or a future metric defined by the owning
    /// benchmark subsystem.
    pub metric: String,

    /// Optional metric unit.
    pub unit: Option<String>,

    /// Measurement value.
    pub measurement: BenchmarkMeasurement,

    /// Explicit uncertainty.
    pub uncertainty: BenchmarkUncertainty,

    /// Realization semantics.
    pub realization: RealizationMode,

    /// Number of samples supporting this observation, when applicable.
    pub sample_count: Option<u128>,

    /// Optional logical operation identity.
    pub operation: Option<OperationId>,

    /// Resources involved in the observation.
    ///
    /// The set is deterministic and contains no duplicates.
    pub resources: BTreeSet<BenchmarkResource>,

    /// Optional logical-to-physical mapping active during the observation.
    pub mapping: Option<BenchmarkResourceMapping>,

    /// Observation context.
    pub context: BenchmarkObservationContext,

    /// Optional caller-defined sequence number.
    ///
    /// This is not a timestamp and does not use a global counter.
    pub sequence: Option<u128>,
}

impl BenchmarkObservation {
    /// Creates a new observation.
    pub fn new(
        metric: impl Into<String>,
        measurement: BenchmarkMeasurement,
        context: BenchmarkObservationContext,
    ) -> BenchmarkingIntegrationResult<Self> {
        let metric = metric.into();

        if metric.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "metric",
            });
        }

        context.validate()?;
        measurement.validate("measurement")?;

        Ok(Self {
            schema_version: BENCHMARKING_INTEGRATION_SCHEMA_VERSION,
            metric,
            unit: None,
            measurement,
            uncertainty: BenchmarkUncertainty::None,
            realization: RealizationMode::Exact,
            sample_count: None,
            operation: None,
            resources: BTreeSet::new(),
            mapping: None,
            context,
            sequence: None,
        })
    }

    /// Sets the metric unit.
    pub fn with_unit(
        mut self,
        unit: impl Into<String>,
    ) -> BenchmarkingIntegrationResult<Self> {
        let unit = unit.into();

        if unit.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "unit",
            });
        }

        self.unit = Some(unit);
        Ok(self)
    }

    /// Adds a resource.
    pub fn with_resource(
        mut self,
        resource: BenchmarkResource,
    ) -> BenchmarkingIntegrationResult<Self> {
        if !self.resources.insert(resource) {
            return Err(BenchmarkingIntegrationError::DuplicateResource);
        }

        Ok(self)
    }

    /// Adds several resources.
    pub fn with_resources<I>(
        mut self,
        resources: I,
    ) -> BenchmarkingIntegrationResult<Self>
    where
        I: IntoIterator<Item = BenchmarkResource>,
    {
        for resource in resources {
            if !self.resources.insert(resource) {
                return Err(BenchmarkingIntegrationError::DuplicateResource);
            }
        }

        Ok(self)
    }

    /// Associates an operation with the observation.
    pub fn with_operation(
        mut self,
        operation: OperationId,
    ) -> BenchmarkingIntegrationResult<Self> {
        self.operation = Some(operation);
        Ok(self)
    }

    /// Associates a resource mapping.
    pub fn with_mapping(
        mut self,
        mapping: BenchmarkResourceMapping,
    ) -> BenchmarkingIntegrationResult<Self> {
        self.mapping = Some(mapping);
        Ok(self)
    }

    /// Adds explicit uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: BenchmarkUncertainty,
    ) -> BenchmarkingIntegrationResult<Self> {
        uncertainty.validate()?;
        self.uncertainty = uncertainty;
        Ok(self)
    }

    /// Declares realization semantics.
    pub fn with_realization(
        mut self,
        realization: RealizationMode,
    ) -> BenchmarkingIntegrationResult<Self> {
        realization.validate()?;
        self.realization = realization;
        Ok(self)
    }

    /// Records the supporting sample count.
    pub fn with_sample_count(
        mut self,
        sample_count: u128,
    ) -> BenchmarkingIntegrationResult<Self> {
        if sample_count == 0 {
            return Err(BenchmarkingIntegrationError::ZeroSampleCount);
        }

        self.sample_count = Some(sample_count);
        Ok(self)
    }

    /// Records an explicit caller-defined sequence.
    pub fn with_sequence(
        mut self,
        sequence: u128,
    ) -> BenchmarkingIntegrationResult<Self> {
        self.sequence = Some(sequence);
        Ok(self)
    }

    /// Validates all observation invariants.
    pub fn validate(&self) -> BenchmarkingIntegrationResult<()> {
        if self.schema_version == 0 {
            return Err(
                BenchmarkingIntegrationError::InvalidObservation {
                    reason: "schema version must be non-zero".to_owned(),
                },
            );
        }

        if self.metric.trim().is_empty() {
            return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "metric",
            });
        }

        if let Some(unit) = &self.unit {
            if unit.trim().is_empty() {
                return Err(BenchmarkingIntegrationError::EmptyIdentifier {
                    field: "unit",
                });
            }
        }

        self.measurement.validate("measurement")?;
        self.uncertainty.validate()?;
        self.realization.validate()?;
        self.context.validate()?;

        if let Some(mapping) = &self.mapping {
            for (logical, physical) in mapping.iter() {
                if !logical.is_valid() {
                    return Err(
                        BenchmarkingIntegrationError::InvalidObservation {
                            reason: format!(
                                "logical qubit {:?} is invalid",
                                logical
                            ),
                        },
                    );
                }

                if !physical.is_valid() {
                    return Err(
                        BenchmarkingIntegrationError::InvalidObservation {
                            reason: format!(
                                "physical qubit {:?} is invalid",
                                physical
                            ),
                        },
                    );
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Batch
// =============================================================================

/// A bounded or unbounded-by-semantics collection of observations.
///
/// The collection itself has no architectural size ceiling. A caller can use
/// `BenchmarkIntegrationLimits` to impose an operational policy.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BenchmarkObservationBatch {
    observations: Vec<BenchmarkObservation>,
}

impl BenchmarkObservationBatch {
    /// Creates an empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a batch from validated observations.
    pub fn from_vec(
        observations: Vec<BenchmarkObservation>,
    ) -> BenchmarkingIntegrationResult<Self> {
        let batch = Self { observations };
        batch.validate()?;
        Ok(batch)
    }

    /// Appends an observation.
    pub fn push(
        &mut self,
        observation: BenchmarkObservation,
    ) -> BenchmarkingIntegrationResult<()> {
        observation.validate()?;
        self.observations.push(observation);
        Ok(())
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the batch is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Iterates in insertion order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &BenchmarkObservation> {
        self.observations.iter()
    }

    /// Consumes the batch and returns its observations.
    #[must_use]
    pub fn into_vec(self) -> Vec<BenchmarkObservation> {
        self.observations
    }

    /// Validates every observation.
    pub fn validate(&self) -> BenchmarkingIntegrationResult<()> {
        for observation in &self.observations {
            observation.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Integration limits
// =============================================================================

/// Caller-selected operational safety limits.
///
/// These are NOT semantic limits on Zamani or quantum hardware.
///
/// Every field is optional so that an application may choose the appropriate
/// policy for its environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BenchmarkIntegrationLimits {
    /// Maximum observations accepted by one batch.
    pub max_observations: Option<u128>,

    /// Maximum resources accepted by one observation.
    pub max_resources_per_observation: Option<u128>,

    /// Maximum bytes permitted for one identifier.
    pub max_identifier_bytes: Option<u128>,

    /// Maximum total sample count represented by one observation.
    pub max_sample_count: Option<u128>,
}

impl BenchmarkIntegrationLimits {
    /// Creates an unlimited integration policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_observations: None,
            max_resources_per_observation: None,
            max_identifier_bytes: None,
            max_sample_count: None,
        }
    }

    /// Validates one observation against the policy.
    pub fn validate_observation(
        &self,
        observation: &BenchmarkObservation,
    ) -> BenchmarkingIntegrationResult<()> {
        observation.validate()?;

        if let Some(maximum) = self.max_resources_per_observation {
            let requested = observation.resources.len() as u128;

            if requested > maximum {
                return Err(
                    BenchmarkingIntegrationError::ResourceLimitExceeded {
                        resource: "resources_per_observation",
                        requested,
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) = self.max_identifier_bytes {
            let metric_bytes = observation.metric.len() as u128;

            if metric_bytes > maximum {
                return Err(
                    BenchmarkingIntegrationError::IdentifierTooLarge {
                        field: "metric",
                        length: observation.metric.len(),
                        maximum: maximum.min(usize::MAX as u128) as usize,
                    },
                );
            }

            if let Some(unit) = &observation.unit {
                let bytes = unit.len() as u128;

                if bytes > maximum {
                    return Err(
                        BenchmarkingIntegrationError::IdentifierTooLarge {
                            field: "unit",
                            length: unit.len(),
                            maximum: maximum.min(usize::MAX as u128) as usize,
                        },
                    );
                }
            }
        }

        if let Some(maximum) = self.max_sample_count {
            if let Some(samples) = observation.sample_count {
                if samples > maximum {
                    return Err(
                        BenchmarkingIntegrationError::ResourceLimitExceeded {
                            resource: "sample_count",
                            requested: samples,
                            maximum,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates a complete batch against the policy.
    pub fn validate_batch(
        &self,
        batch: &BenchmarkObservationBatch,
    ) -> BenchmarkingIntegrationResult<()> {
        if let Some(maximum) = self.max_observations {
            let requested = batch.len() as u128;

            if requested > maximum {
                return Err(
                    BenchmarkingIntegrationError::ResourceLimitExceeded {
                        resource: "observations",
                        requested,
                        maximum,
                    },
                );
            }
        }

        for observation in batch.iter() {
            self.validate_observation(observation)?;
        }

        Ok(())
    }
}

// =============================================================================
// Observation source
// =============================================================================

/// Provider-neutral source of ZQN benchmark observations.
///
/// Implementations may wrap:
///
/// - characterization;
/// - hardware telemetry;
/// - simulation;
/// - calibration;
/// - QEC experiments;
/// - benchmark execution.
///
/// The trait does not prescribe how observations are generated.
pub trait BenchmarkObservationSource {
    /// Returns the next observation.
    ///
    /// `Ok(None)` means the source has been exhausted.
    fn next_observation(
        &mut self,
    ) -> BenchmarkingIntegrationResult<Option<BenchmarkObservation>>;
}

// =============================================================================
// Observation sink
// =============================================================================

/// Provider-neutral consumer of benchmark observations.
///
/// A benchmark implementation can implement this trait without ZQN needing to
/// depend on the concrete benchmarking implementation.
pub trait BenchmarkObservationSink {
    /// Accepts one validated observation.
    fn observe(
        &mut self,
        observation: BenchmarkObservation,
    ) -> BenchmarkingIntegrationResult<()>;
}

// =============================================================================
// Collector
// =============================================================================

/// In-memory observation collector.
///
/// This is useful for small/medium workloads and tests. For extremely large
/// workloads, prefer implementing `BenchmarkObservationSink` with a streaming
/// storage backend instead of materializing every observation.
#[derive(Debug, Clone)]
pub struct BenchmarkObservationCollector {
    limits: BenchmarkIntegrationLimits,
    batch: BenchmarkObservationBatch,
}

impl BenchmarkObservationCollector {
    /// Creates a collector with explicit resource policy.
    pub fn new(
        limits: BenchmarkIntegrationLimits,
    ) -> BenchmarkingIntegrationResult<Self> {
        Ok(Self {
            limits,
            batch: BenchmarkObservationBatch::new(),
        })
    }

    /// Returns the configured limits.
    #[must_use]
    pub const fn limits(&self) -> BenchmarkIntegrationLimits {
        self.limits
    }

    /// Returns the current number of observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.batch.len()
    }

    /// Returns whether no observations have been collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.batch.is_empty()
    }

    /// Returns an immutable view of collected observations.
    #[must_use]
    pub fn observations(&self) -> &BenchmarkObservationBatch {
        &self.batch
    }

    /// Consumes the collector and returns its observations.
    #[must_use]
    pub fn into_batch(self) -> BenchmarkObservationBatch {
        self.batch
    }
}

impl BenchmarkObservationSink for BenchmarkObservationCollector {
    fn observe(
        &mut self,
        observation: BenchmarkObservation,
    ) -> BenchmarkingIntegrationResult<()> {
        self.limits.validate_observation(&observation)?;

        if let Some(maximum) = self.limits.max_observations {
            let current = self.batch.len() as u128;

            if current >= maximum {
                return Err(
                    BenchmarkingIntegrationError::ResourceLimitExceeded {
                        resource: "observations",
                        requested: current.saturating_add(1),
                        maximum,
                    },
                );
            }
        }

        self.batch.push(observation)
    }
}

// =============================================================================
// Source → sink transfer
// =============================================================================

/// Streams observations from a source into a sink.
///
/// This is intentionally streaming: it never requires the complete benchmark
/// to exist in memory.
///
/// The returned count is the number of observations successfully transferred.
pub fn transfer_observations<S, K>(
    source: &mut S,
    sink: &mut K,
) -> BenchmarkingIntegrationResult<u128>
where
    S: BenchmarkObservationSource,
    K: BenchmarkObservationSink,
{
    let mut transferred = 0_u128;

    loop {
        match source.next_observation()? {
            Some(observation) => {
                sink.observe(observation)?;

                transferred = transferred.checked_add(1).ok_or_else(|| {
                    BenchmarkingIntegrationError::InvalidBatch {
                        reason: "observation transfer count overflow"
                            .to_owned(),
                    }
                })?;
            }

            None => break,
        }
    }

    Ok(transferred)
}

// =============================================================================
// Deterministic observation ordering
// =============================================================================

/// Deterministically sorts observations for reproducible reporting.
///
/// Ordering is based on:
///
/// 1. optional explicit sequence;
/// 2. target identity;
/// 3. metric;
/// 4. operation identity when available;
/// 5. resource set;
/// 6. calibration identity.
///
/// This function does not mutate semantic values.
pub fn sort_observations_deterministically(
    observations: &mut [BenchmarkObservation],
) {
    observations.sort_by(|left, right| {
        left.sequence
            .cmp(&right.sequence)
            .then_with(|| {
                left.context
                    .target
                    .cmp(&right.context.target)
            })
            .then_with(|| left.metric.cmp(&right.metric))
            .then_with(|| left.operation.cmp(&right.operation))
            .then_with(|| left.resources.cmp(&right.resources))
            .then_with(|| {
                left.context
                    .calibration
                    .cmp(&right.context.calibration)
            })
    });
}

// =============================================================================
// Metric aggregation helpers
// =============================================================================

/// Computes a deterministic arithmetic mean of scalar/probability
/// observations.
///
/// This is intentionally a minimal integration helper, not a replacement for
/// the benchmarking statistics subsystem.
///
/// Counts are not accepted because converting arbitrarily large `u128` counts
/// to `f64` would silently lose exactness.
pub fn arithmetic_mean(
    observations: &[BenchmarkObservation],
) -> BenchmarkingIntegrationResult<f64> {
    if observations.is_empty() {
        return Err(BenchmarkingIntegrationError::InvalidBatch {
            reason: "cannot calculate a mean of an empty observation set"
                .to_owned(),
        });
    }

    let mut sum = 0.0_f64;

    for observation in observations {
        let value = match observation.measurement {
            BenchmarkMeasurement::Scalar(value)
            | BenchmarkMeasurement::Probability(value) => value,

            BenchmarkMeasurement::Count(_) => {
                return Err(
                    BenchmarkingIntegrationError::InvalidBatch {
                        reason:
                            "arithmetic_mean does not implicitly convert \
                             u128 counts to f64"
                                .to_owned(),
                    },
                );
            }
        };

        if !value.is_finite() {
            return Err(BenchmarkingIntegrationError::NonFiniteValue {
                field: "measurement",
                value,
            });
        }

        sum += value;

        if !sum.is_finite() {
            return Err(
                BenchmarkingIntegrationError::NonFiniteValue {
                    field: "mean_accumulator",
                    value: sum,
                },
            );
        }
    }

    let count = observations.len() as f64;
    let mean = sum / count;

    if !mean.is_finite() {
        return Err(BenchmarkingIntegrationError::NonFiniteValue {
            field: "mean",
            value: mean,
        });
    }

    Ok(mean)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_finite(
    field: &'static str,
    value: f64,
) -> BenchmarkingIntegrationResult<()> {
    if !value.is_finite() {
        return Err(BenchmarkingIntegrationError::NonFiniteValue {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> BenchmarkingIntegrationResult<()> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(BenchmarkingIntegrationError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_non_negative_finite(
    field: &'static str,
    value: f64,
) -> BenchmarkingIntegrationResult<()> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(BenchmarkingIntegrationError::NegativeUncertainty {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_confidence(
    value: f64,
) -> BenchmarkingIntegrationResult<()> {
    if !value.is_finite() || !(0.0 < value && value < 1.0) {
        return Err(BenchmarkingIntegrationError::InvalidConfidence {
            value,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> BenchmarkTargetId {
        BenchmarkTargetId::new("test-target").expect("valid target")
    }

    fn context() -> BenchmarkObservationContext {
        BenchmarkObservationContext {
            target: target(),
            calibration: None,
            noise_model: None,
            provenance: None,
            execution_id: None,
        }
    }

    fn scalar_observation(
        value: f64,
    ) -> BenchmarkObservation {
        BenchmarkObservation::new(
            "test.metric",
            BenchmarkMeasurement::Scalar(value),
            context(),
        )
        .expect("valid observation")
    }

    #[test]
    fn rejects_empty_target_identity() {
        let result = BenchmarkTargetId::new("");

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::EmptyIdentifier {
                field: "target_id"
            })
        ));
    }

    #[test]
    fn rejects_non_finite_measurement() {
        let result = BenchmarkObservation::new(
            "metric",
            BenchmarkMeasurement::Scalar(f64::NAN),
            context(),
        );

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::NonFiniteValue {
                field: "measurement",
                ..
            })
        ));
    }

    #[test]
    fn rejects_invalid_probability() {
        let result = BenchmarkObservation::new(
            "metric",
            BenchmarkMeasurement::Probability(1.1),
            context(),
        );

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::InvalidProbability {
                field: "measurement",
                ..
            })
        ));
    }

    #[test]
    fn accepts_valid_probability() {
        let result = BenchmarkObservation::new(
            "metric",
            BenchmarkMeasurement::Probability(2.0 / 3.0),
            context(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_zero_sample_count() {
        let result = scalar_observation(0.5).with_sample_count(0);

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::ZeroSampleCount)
        ));
    }

    #[test]
    fn mapping_rejects_duplicate_logical_resource() {
        let mut mapping = BenchmarkResourceMapping::new();

        let logical = QubitId::new(0);
        let first = PhysicalQubitId::new(0);
        let second = PhysicalQubitId::new(1);

        assert!(mapping.insert(logical, first).is_ok());

        let result = mapping.insert(logical, second);

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::DuplicateLogicalQubit {
                ..
            })
        ));
    }

    #[test]
    fn mapping_rejects_duplicate_physical_resource() {
        let mut mapping = BenchmarkResourceMapping::new();

        let first = QubitId::new(0);
        let second = QubitId::new(1);
        let physical = PhysicalQubitId::new(0);

        assert!(mapping.insert(first, physical).is_ok());

        let result = mapping.insert(second, physical);

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::DuplicatePhysicalQubit {
                ..
            })
        ));
    }

    #[test]
    fn resource_set_is_deterministic() {
        let observation = scalar_observation(0.5)
            .with_resource(BenchmarkResource::LogicalQubit(
                QubitId::new(10),
            ))
            .expect("resource")
            .with_resource(BenchmarkResource::LogicalQubit(
                QubitId::new(2),
            ))
            .expect("resource");

        let resources: Vec<_> = observation.resources.iter().collect();

        assert_eq!(
            resources[0],
            &BenchmarkResource::LogicalQubit(QubitId::new(2))
        );
        assert_eq!(
            resources[1],
            &BenchmarkResource::LogicalQubit(QubitId::new(10))
        );
    }

    #[test]
    fn explicit_approximation_is_valid() {
        let observation = scalar_observation(0.5)
            .with_realization(RealizationMode::Approximate {
                tolerance: 1.0e-6,
            })
            .expect("valid approximation");

        assert_eq!(
            observation.realization,
            RealizationMode::Approximate {
                tolerance: 1.0e-6
            }
        );
    }

    #[test]
    fn invalid_approximation_is_rejected() {
        let result = scalar_observation(0.5).with_realization(
            RealizationMode::Approximate {
                tolerance: -1.0,
            },
        );

        assert!(matches!(
            result,
            Err(BenchmarkingIntegrationError::NegativeUncertainty {
                ..
            })
        ));
    }

    #[test]
    fn confidence_level_is_validated() {
        let valid = scalar_observation(0.5).with_realization(
            RealizationMode::Statistical {
                confidence: 0.95,
            },
        );

        assert!(valid.is_ok());

        let invalid = scalar_observation(0.5).with_realization(
            RealizationMode::Statistical {
                confidence: 1.0,
            },
        );

        assert!(matches!(
            invalid,
            Err(BenchmarkingIntegrationError::InvalidConfidence {
                ..
            })
        ));
    }

    #[test]
    fn collector_preserves_partial_results() {
        let limits = BenchmarkIntegrationLimits {
            max_observations: Some(2),
            ..BenchmarkIntegrationLimits::unlimited()
        };

        let mut collector =
            BenchmarkObservationCollector::new(limits)
                .expect("collector");

        collector
            .observe(scalar_observation(0.1))
            .expect("first");

        collector
            .observe(scalar_observation(0.2))
            .expect("second");

        assert_eq!(collector.len(), 2);
    }

    #[test]
    fn collector_enforces_explicit_limit() {
        let limits = BenchmarkIntegrationLimits {
            max_observations: Some(1),
            ..BenchmarkIntegrationLimits::unlimited()
        };

        let mut collector =
            BenchmarkObservationCollector::new(limits)
                .expect("collector");

        collector
            .observe(scalar_observation(0.1))
            .expect("first");

        let result = collector.observe(scalar_observation(0.2));

        assert!(matches!(
            result,
            Err(
                BenchmarkingIntegrationError::ResourceLimitExceeded {
                    resource: "observations",
                    ..
                }
            )
        ));
    }

    #[test]
    fn arithmetic_mean_is_deterministic() {
        let observations = vec![
            scalar_observation(0.2),
            scalar_observation(0.4),
            scalar_observation(0.6),
        ];

        let mean =
            arithmetic_mean(&observations).expect("mean");

        assert!((mean - 0.4).abs() < 1.0e-12);
    }

    #[test]
    fn deterministic_sorting_is_stable_for_equal_keys() {
        let mut observations = vec![
            scalar_observation(0.3).with_sequence(2).expect("sequence"),
            scalar_observation(0.1).with_sequence(1).expect("sequence"),
            scalar_observation(0.2).with_sequence(0).expect("sequence"),
        ];

        sort_observations_deterministically(&mut observations);

        assert_eq!(
            observations[0].measurement,
            BenchmarkMeasurement::Scalar(0.2)
        );

        assert_eq!(
            observations[1].measurement,
            BenchmarkMeasurement::Scalar(0.1)
        );

        assert_eq!(
            observations[2].measurement,
            BenchmarkMeasurement::Scalar(0.3)
        );
    }

    #[test]
    fn source_to_sink_transfer_is_streaming_contract() {
        struct Source {
            remaining: usize,
        }

        impl BenchmarkObservationSource for Source {
            fn next_observation(
                &mut self,
            ) -> BenchmarkingIntegrationResult<
                Option<BenchmarkObservation>,
            > {
                if self.remaining == 0 {
                    return Ok(None);
                }

                self.remaining -= 1;

                Ok(Some(scalar_observation(
                    self.remaining as f64,
                )))
            }
        }

        let limits = BenchmarkIntegrationLimits::unlimited();

        let mut source = Source { remaining: 3 };

        let mut sink =
            BenchmarkObservationCollector::new(limits)
                .expect("collector");

        let transferred =
            transfer_observations(&mut source, &mut sink)
                .expect("transfer");

        assert_eq!(transferred, 3);
        assert_eq!(sink.len(), 3);
    }

    #[test]
    fn canonical_qubit_ids_are_used() {
        let logical = QubitId::new(123);
        let physical = PhysicalQubitId::new(456);

        let observation = scalar_observation(0.25)
            .with_resource(BenchmarkResource::LogicalQubit(logical))
            .expect("logical resource")
            .with_resource(BenchmarkResource::PhysicalQubit(physical))
            .expect("physical resource");

        assert!(observation.resources.contains(
            &BenchmarkResource::LogicalQubit(logical)
        ));

        assert!(observation.resources.contains(
            &BenchmarkResource::PhysicalQubit(physical)
        ));
    }

    #[test]
    fn opaque_resources_allow_future_modalities() {
        let resource = BenchmarkResource::opaque(
            "future.quantum.resource",
            "resource-0",
        )
        .expect("opaque resource");

        let observation = scalar_observation(1.0)
            .with_resource(resource)
            .expect("resource");

        assert_eq!(observation.resources.len(), 1);
    }

    #[test]
    fn no_semantic_machine_limit_exists() {
        let mut mapping = BenchmarkResourceMapping::new();

        for index in 0_u64..1024_u64 {
            mapping
                .insert(
                    QubitId::new(index),
                    PhysicalQubitId::new(index),
                )
                .expect("unique mapping");
        }

        assert_eq!(mapping.len(), 1024);
    }

    #[test]
    fn uncertainty_interval_is_validated() {
        let valid = scalar_observation(0.5).with_uncertainty(
            BenchmarkUncertainty::Interval {
                lower: 0.4,
                upper: 0.6,
            },
        );

        assert!(valid.is_ok());

        let invalid = scalar_observation(0.5).with_uncertainty(
            BenchmarkUncertainty::Interval {
                lower: 0.7,
                upper: 0.6,
            },
        );

        assert!(matches!(
            invalid,
            Err(
                BenchmarkingIntegrationError::InconsistentInterval {
                    ..
                }
            )
        ));
    }

    #[test]
    fn observation_validation_is_idempotent() {
        let observation = scalar_observation(0.5)
            .with_uncertainty(BenchmarkUncertainty::Absolute(0.01))
            .expect("uncertainty")
            .with_realization(RealizationMode::Statistical {
                confidence: 0.95,
            })
            .expect("realization")
            .with_sample_count(100)
            .expect("samples");

        assert!(observation.validate().is_ok());
        assert!(observation.validate().is_ok());
    }
}