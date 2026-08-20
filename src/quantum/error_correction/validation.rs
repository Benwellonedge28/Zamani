//! Zamani Quantum Error Correction — Cross-Module Validation.
//!
//! # Architectural contract
//!
//! `validation.rs` is the cross-module validation boundary for QEC.
//!
//! It verifies that already-constructed or externally supplied QEC objects
//! satisfy the invariants required by downstream algorithms.
//!
//! ```text
//!                 QecLimits
//!                    │
//!                    ▼
//!             validation.rs
//!                    │
//!       ┌────────────┼────────────┐
//!       ▼            ▼            ▼
//!  SurfaceCode   Stabilizers   Syndrome
//!       │            │            │
//!       └────────────┼────────────┘
//!                    ▼
//!             validated input
//!                    │
//!                    ▼
//!             decoder / QPU
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - cross-module invariant validation;
//! - validation reports;
//! - validation-specific diagnostics;
//! - resource-policy preflight through `QecLimits`;
//! - canonical conversion to `QecError`.
//!
//! This module does NOT own:
//!
//! - resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - cancellation state (`cancellation.rs`);
//! - decoder algorithms;
//! - topology construction;
//! - syndrome construction;
//! - authorization;
//! - telemetry;
//! - QPU execution.
//!
//! # Critical architectural rule
//!
//! `QecLimits` is the single source of truth for production resource policy.
//!
//! `ValidationLimits` is retained only as a type alias for source
//! compatibility. It is NOT a second policy structure.
//!
//! # Integration
//!
//! `limits.rs`
//!     Owns all declarative resource ceilings.
//!
//! `errors.rs`
//!     Owns the canonical public QEC error boundary.
//!
//! `memory.rs`
//!     Owns memory admission/allocation enforcement.
//!
//! `resources.rs`
//!     Owns runtime resource accounting.
//!
//! `surface_code.rs`
//!     Owns mathematical surface-code topology.
//!
//! `stabilizer.rs`
//!     Owns Pauli/stabilizer algebra.
//!
//! `syndrome.rs`
//!     Owns syndrome and detection-event representations.
//!
//! `decoder.rs`
//!     Must validate inputs before decoder execution.
//!
//! `qpu_adapter.rs`
//!     Must validate QPU-derived objects before they enter decoding.
//!
//! `streaming.rs`
//!     Must use the same `QecLimits` policy for bounded streams.
//!
//! `partition.rs`
//!     Must use the same policy for partition admission.
//!
//! `checkpoint.rs`
//!     Must validate restored objects before execution.
//!
//! No later module should create another independent validation-limit system.
//!
//! # Rust
//!
//! Target: Rust 1.97.1.
//!
//! `unsafe` is forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeSet;

use super::errors::{
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::{
    LimitError,
    LimitKind,
    QecLimits,
};
use super::stabilizer::{
    PauliString,
    QubitIndex,
    StabilizerGroup,
};
use super::surface_code::SurfaceCode;
use super::syndrome::{
    DetectionEvent,
    StabilizerId,
    Syndrome,
};

// ============================================================================
// Compatibility
// ============================================================================

/// Compatibility alias for the former validation-specific policy.
///
/// This is intentionally an alias rather than a new structure.
///
/// `QecLimits` remains the only production resource policy.
pub type ValidationLimits = QecLimits;

// ============================================================================
// Validation errors
// ============================================================================

/// Cross-module validation error.
///
/// This type describes structural and semantic validation failures before
/// they cross the public QEC API boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The canonical resource policy is invalid.
    InvalidLimits {
        message: String,
    },

    /// A resource request exceeds the canonical QEC policy.
    ResourceLimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    /// Surface-code distance is mathematically invalid.
    InvalidCodeDistance {
        distance: usize,
    },

    /// A code's physical-qubit count does not match its declared topology.
    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// A qubit identifier is outside the declared qubit domain.
    QubitOutOfRange {
        qubit: QubitIndex,
        num_qubits: usize,
    },

    /// A qubit occurs more than once in a support.
    DuplicateQubit {
        qubit: QubitIndex,
    },

    /// A stabilizer identifier occurs more than once.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// A stabilizer has no support.
    EmptyStabilizer {
        stabilizer: StabilizerId,
    },

    /// A stabilizer exceeds the canonical stabilizer-weight limit.
    StabilizerWeightExceeded {
        stabilizer: StabilizerId,
        limit: usize,
        actual: usize,
    },

    /// Two stabilizers fail to commute.
    NonCommutingStabilizers {
        first: usize,
        second: usize,
    },

    /// A Pauli operator has the wrong number of qubits.
    OperatorDimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// A Pauli operator exceeds its permitted weight.
    OperatorWeightExceeded {
        limit: usize,
        actual: usize,
    },

    /// A logical operator is identity.
    IdentityLogicalOperator {
        name: &'static str,
    },

    /// Logical X and Z do not anticommute.
    InvalidLogicalAnticommutation,

    /// A logical operator fails to commute with a stabilizer.
    LogicalOperatorViolatesStabilizer {
        stabilizer: usize,
        logical: &'static str,
    },

    /// Syndrome exceeds the canonical event limit.
    SyndromeSizeExceeded {
        limit: usize,
        actual: usize,
    },

    /// Syndrome stabilizer domain differs from the validated stabilizer set.
    SyndromeStabilizerMismatch,

    /// Detection-event batch exceeds the canonical event limit.
    DetectionEventCountExceeded {
        limit: usize,
        actual: usize,
    },

    /// Detection event references an unknown stabilizer.
    UnknownDetectionStabilizer {
        stabilizer: StabilizerId,
    },

    /// A detection event is not active.
    InactiveDetectionEvent,

    /// Two events occupy the same round/stabilizer slot.
    DuplicateDetectionEvent {
        round: u64,
        stabilizer: StabilizerId,
    },

    /// A lower-level QEC module rejected its own invariant.
    ModuleError {
        module: &'static str,
        message: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidLimits { message } => {
                write!(f, "invalid QEC limits: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "resource limit exceeded: {resource}, \
                     requested={requested}, maximum={maximum}"
                )
            }

            Self::InvalidCodeDistance { distance } => {
                write!(
                    f,
                    "invalid surface-code distance: {distance}; \
                     distance must be odd and >= 3"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "qubit-count mismatch: expected={expected}, \
                     actual={actual}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside range 0..{num_qubits}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "duplicate qubit in support: {qubit}"
                )
            }

            Self::DuplicateStabilizer { stabilizer } => {
                write!(
                    f,
                    "duplicate stabilizer identifier: {stabilizer}"
                )
            }

            Self::EmptyStabilizer { stabilizer } => {
                write!(
                    f,
                    "stabilizer {stabilizer} has empty support"
                )
            }

            Self::StabilizerWeightExceeded {
                stabilizer,
                limit,
                actual,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} exceeds weight limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::NonCommutingStabilizers {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} do not commute"
                )
            }

            Self::OperatorDimensionMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operator dimension mismatch: expected={expected}, \
                     actual={actual}"
                )
            }

            Self::OperatorWeightExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "operator weight exceeds limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::IdentityLogicalOperator { name } => {
                write!(
                    f,
                    "logical operator {name} is identity"
                )
            }

            Self::InvalidLogicalAnticommutation => {
                f.write_str(
                    "logical X and logical Z do not anticommute",
                )
            }

            Self::LogicalOperatorViolatesStabilizer {
                stabilizer,
                logical,
            } => {
                write!(
                    f,
                    "logical operator {logical} does not commute \
                     with stabilizer {stabilizer}"
                )
            }

            Self::SyndromeSizeExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "syndrome exceeds limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::SyndromeStabilizerMismatch => {
                f.write_str(
                    "syndrome stabilizer domain is inconsistent",
                )
            }

            Self::DetectionEventCountExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "detection-event count exceeds limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::UnknownDetectionStabilizer {
                stabilizer,
            } => {
                write!(
                    f,
                    "detection event references unknown \
                     stabilizer {stabilizer}"
                )
            }

            Self::InactiveDetectionEvent => {
                f.write_str(
                    "detection event is inactive",
                )
            }

            Self::DuplicateDetectionEvent {
                round,
                stabilizer,
            } => {
                write!(
                    f,
                    "duplicate detection event at round {round}, \
                     stabilizer {stabilizer}"
                )
            }

            Self::ModuleError {
                module,
                message,
            } => {
                write!(
                    f,
                    "{module} validation failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

// ============================================================================
// Canonical error integration
// ============================================================================

impl From<LimitError> for ValidationError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::InvalidLimit {
                resource,
                value,
            } => Self::InvalidLimits {
                message: format!(
                    "{resource} has invalid value {value}"
                ),
            },

            LimitError::Exceeded {
                resource,
                requested,
                maximum,
            } => Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            },

            LimitError::ArithmeticOverflow {
                resource,
            } => Self::ModuleError {
                module: "limits",
                message: format!(
                    "arithmetic overflow while validating {resource}"
                ),
            },

            LimitError::InconsistentLimits {
                resource,
                related_resource,
                reason,
            } => Self::InvalidLimits {
                message: format!(
                    "{resource} conflicts with \
                     {related_resource}: {reason}"
                ),
            },

            LimitError::UnsupportedSchema {
                found,
                expected,
            } => Self::InvalidLimits {
                message: format!(
                    "unsupported limits schema: \
                     found={found}, expected={expected}"
                ),
            },
        }
    }
}

impl From<ValidationError> for QecError {
    fn from(error: ValidationError) -> Self {
        match error {
            ValidationError::InvalidLimits { message } => {
                QecError::InvalidInput { message }
            }

            ValidationError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                QecError::ResourceLimitExceeded {
                    resource: match resource {
                        LimitKind::CodeDistance =>
                            ResourceKind::CodeDistance,
                        LimitKind::Qubits =>
                            ResourceKind::Qubits,
                        LimitKind::Stabilizers =>
                            ResourceKind::Stabilizers,
                        LimitKind::SyndromeEvents =>
                            ResourceKind::SyndromeEvents,
                        LimitKind::MeasurementRounds =>
                            ResourceKind::MeasurementRounds,
                        LimitKind::GraphNodes =>
                            ResourceKind::GraphNodes,
                        LimitKind::GraphEdges =>
                            ResourceKind::GraphEdges,
                        LimitKind::MemoryBytes =>
                            ResourceKind::MemoryBytes,
                        LimitKind::DecoderTimeNs =>
                            ResourceKind::Time,
                        LimitKind::Parallelism =>
                            ResourceKind::Parallelism,
                        LimitKind::CheckpointSizeBytes =>
                            ResourceKind::CheckpointSize,
                        LimitKind::Partitions =>
                            ResourceKind::Partitions,
                        LimitKind::StreamBufferEvents =>
                            ResourceKind::StreamBuffer,
                        LimitKind::DecoderIterations =>
                            ResourceKind::DecoderIterations,
                        LimitKind::StabilizerWeight =>
                            ResourceKind::StabilizerWeight,
                        LimitKind::LogicalOperatorWeight =>
                            ResourceKind::LogicalWeight,
                        LimitKind::QubitsPerPartition =>
                            ResourceKind::Qubits,
                        LimitKind::QpuShots =>
                            ResourceKind::QpuShots,
                        LimitKind::QpuCircuits =>
                            ResourceKind::QpuCircuits,
                        LimitKind::VerificationOperations =>
                            ResourceKind::Operations,
                    },
                    requested,
                    current: 0,
                    limit: maximum,
                    message:
                        "QEC validation resource preflight failed"
                            .to_owned(),
                }
            }

            ValidationError::InvalidCodeDistance {
                distance,
            } => {
                QecError::InvalidTopology {
                    message: format!(
                        "invalid surface-code distance {distance}"
                    ),
                }
            }

            ValidationError::QubitCountMismatch {
                expected,
                actual,
            } => {
                QecError::InvalidTopology {
                    message: format!(
                        "qubit count mismatch: \
                         expected={expected}, actual={actual}"
                    ),
                }
            }

            ValidationError::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                QecError::InvalidTopology {
                    message: format!(
                        "qubit {qubit} outside domain \
                         0..{num_qubits}"
                    ),
                }
            }

            ValidationError::DuplicateQubit { qubit } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "duplicate qubit {qubit}"
                    ),
                }
            }

            ValidationError::DuplicateStabilizer {
                stabilizer,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "duplicate stabilizer {stabilizer}"
                    ),
                }
            }

            ValidationError::EmptyStabilizer {
                stabilizer,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "empty stabilizer {stabilizer}"
                    ),
                }
            }

            ValidationError::StabilizerWeightExceeded {
                stabilizer,
                limit,
                actual,
            } => {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::StabilizerWeight,
                    requested: actual as u128,
                    current: 0,
                    limit: limit as u128,
                    message: format!(
                        "stabilizer {stabilizer} \
                         exceeds configured weight limit"
                    ),
                }
            }

            ValidationError::NonCommutingStabilizers {
                first,
                second,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "stabilizers {first} and {second} \
                         do not commute"
                    ),
                }
            }

            ValidationError::OperatorDimensionMismatch {
                expected,
                actual,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "operator dimension mismatch: \
                         expected={expected}, actual={actual}"
                    ),
                }
            }

            ValidationError::OperatorWeightExceeded {
                limit,
                actual,
            } => {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::LogicalWeight,
                    requested: actual as u128,
                    current: 0,
                    limit: limit as u128,
                    message:
                        "operator weight exceeds configured limit"
                            .to_owned(),
                }
            }

            ValidationError::IdentityLogicalOperator {
                name,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "logical operator {name} is identity"
                    ),
                }
            }

            ValidationError::InvalidLogicalAnticommutation => {
                QecError::InvalidStabilizer {
                    message:
                        "logical X and Z do not anticommute"
                            .to_owned(),
                }
            }

            ValidationError::LogicalOperatorViolatesStabilizer {
                stabilizer,
                logical,
            } => {
                QecError::InvalidStabilizer {
                    message: format!(
                        "logical operator {logical} \
                         violates stabilizer {stabilizer}"
                    ),
                }
            }

            ValidationError::SyndromeSizeExceeded {
                limit,
                actual,
            } => {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::SyndromeEvents,
                    requested: actual as u128,
                    current: 0,
                    limit: limit as u128,
                    message:
                        "syndrome exceeds configured limit"
                            .to_owned(),
                }
            }

            ValidationError::SyndromeStabilizerMismatch => {
                QecError::InvalidSyndrome {
                    message:
                        "syndrome stabilizer domain mismatch"
                            .to_owned(),
                }
            }

            ValidationError::DetectionEventCountExceeded {
                limit,
                actual,
            } => {
                QecError::ResourceLimitExceeded {
                    resource:
                        ResourceKind::SyndromeEvents,
                    requested: actual as u128,
                    current: 0,
                    limit: limit as u128,
                    message:
                        "detection-event count exceeds \
                         configured limit"
                            .to_owned(),
                }
            }

            ValidationError::UnknownDetectionStabilizer {
                stabilizer,
            } => {
                QecError::InvalidSyndrome {
                    message: format!(
                        "unknown detection-event stabilizer \
                         {stabilizer}"
                    ),
                }
            }

            ValidationError::InactiveDetectionEvent => {
                QecError::InvalidSyndrome {
                    message:
                        "inactive detection event supplied"
                            .to_owned(),
                }
            }

            ValidationError::DuplicateDetectionEvent {
                round,
                stabilizer,
            } => {
                QecError::InvalidSyndrome {
                    message: format!(
                        "duplicate detection event at \
                         round {round}, stabilizer {stabilizer}"
                    ),
                }
            }

            ValidationError::ModuleError {
                module,
                message,
            } => {
                QecError::InvalidInput {
                    message: format!(
                        "{module}: {message}"
                    ),
                }
            }
        }
    }
}

// ============================================================================
// Validation report
// ============================================================================

/// Deterministic summary returned by successful validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationReport {
    /// Number of validated data qubits.
    pub qubits: usize,

    /// Number of validated stabilizer generators.
    pub stabilizers: usize,

    /// Number of syndrome measurements.
    pub syndrome_measurements: usize,

    /// Number of detection events.
    pub detection_events: usize,
}

impl ValidationReport {
    /// Empty report for callers validating an object without topology.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            qubits: 0,
            stabilizers: 0,
            syndrome_measurements: 0,
            detection_events: 0,
        }
    }
}

// ============================================================================
// Resource preflight helpers
// ============================================================================

/// Validates the canonical QEC resource policy itself.
pub fn validate_limits(
    limits: &QecLimits,
) -> Result<(), ValidationError> {
    limits.validate()?;
    Ok(())
}

/// Validates an arbitrary resource request against canonical policy.
pub fn validate_resource_request(
    limits: &QecLimits,
    resource: LimitKind,
    requested: u128,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.check(resource, requested)?;
    Ok(())
}

/// Validates a code's primary resource requirements.
pub fn validate_code_resources(
    limits: &QecLimits,
    distance: usize,
    qubits: usize,
    stabilizers: usize,
) -> Result<(), ValidationError> {
    limits.validate()?;

    if distance < 3
        || distance % 2 == 0
    {
        return Err(
            ValidationError::InvalidCodeDistance {
                distance,
            },
        );
    }

    limits.validate_code_size(
        distance,
        qubits,
        stabilizers,
    )?;

    Ok(())
}

/// Validates a memory preflight request.
///
/// Actual reservation remains the responsibility of `memory.rs`.
pub fn validate_memory_request(
    limits: &QecLimits,
    bytes: u64,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_memory(bytes)?;
    Ok(())
}

/// Validates a decoding graph preflight request.
pub fn validate_graph_request(
    limits: &QecLimits,
    nodes: usize,
    edges: usize,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_graph(nodes, edges)?;
    Ok(())
}

/// Validates syndrome resource requirements.
pub fn validate_syndrome_resources(
    limits: &QecLimits,
    events: usize,
    rounds: usize,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_syndrome(events, rounds)?;
    Ok(())
}

/// Validates decoder work admission.
pub fn validate_decoder_resources(
    limits: &QecLimits,
    iterations: usize,
    time_ns: u64,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_decoder_work(
        iterations,
        time_ns,
    )?;
    Ok(())
}

/// Validates partition admission.
pub fn validate_partition_resources(
    limits: &QecLimits,
    partitions: usize,
    qubits_per_partition: usize,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_partition(
        partitions,
        qubits_per_partition,
    )?;
    Ok(())
}

/// Validates streaming buffer admission.
pub fn validate_stream_resources(
    limits: &QecLimits,
    buffer_events: usize,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_stream(buffer_events)?;
    Ok(())
}

/// Validates checkpoint size admission.
pub fn validate_checkpoint_resources(
    limits: &QecLimits,
    bytes: u64,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_checkpoint(bytes)?;
    Ok(())
}

/// Validates QPU shot/circuit admission.
pub fn validate_qpu_resources(
    limits: &QecLimits,
    shots: u64,
    circuits: u64,
) -> Result<(), ValidationError> {
    limits.validate()?;
    limits.validate_qpu(shots, circuits)?;
    Ok(())
}

// ============================================================================
// Surface-code validation
// ============================================================================

/// Validates a surface code using canonical default QEC policy.
pub fn validate_surface_code(
    code: &SurfaceCode,
) -> Result<ValidationReport, ValidationError> {
    validate_surface_code_with_limits(
        code,
        &QecLimits::default(),
    )
}

/// Validates a surface code against the canonical QEC resource policy.
///
/// This function verifies an existing topology. It does not construct or
/// allocate topology data.
pub fn validate_surface_code_with_limits(
    code: &SurfaceCode,
    limits: &QecLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let distance = code.distance();
    let qubits = code.num_data_qubits();
    let stabilizers = code.num_stabilizers();

    validate_code_resources(
        limits,
        distance,
        qubits,
        stabilizers,
    )?;

    let expected_qubits =
        distance
            .checked_mul(distance)
            .ok_or_else(|| {
                ValidationError::ModuleError {
                    module: "validation",
                    message:
                        "surface-code qubit count overflow"
                            .to_owned(),
                }
            })?;

    if qubits != expected_qubits {
        return Err(
            ValidationError::QubitCountMismatch {
                expected: expected_qubits,
                actual: qubits,
            },
        );
    }

    // The SurfaceCode object is deterministic and already exposes its
    // physical qubit representation. We nevertheless verify the domain and
    // coordinate mapping at the cross-module boundary.
    let mut seen_qubits =
        BTreeSet::new();

    for data_qubit in code.data_qubits() {
        let index = data_qubit.index();

        if index.index() >= qubits {
            return Err(
                ValidationError::QubitOutOfRange {
                    qubit: index,
                    num_qubits: qubits,
                },
            );
        }

        if !seen_qubits.insert(index) {
            return Err(
                ValidationError::DuplicateQubit {
                    qubit: index,
                },
            );
        }

        let mapped =
            code.coordinate_of(index)
                .map_err(|error| {
                    ValidationError::ModuleError {
                        module: "surface_code",
                        message: error.to_string(),
                    }
                })?;

        if mapped != data_qubit.coordinate() {
            return Err(
                ValidationError::ModuleError {
                    module: "surface_code",
                    message: format!(
                        "coordinate mapping for {index} \
                         is inconsistent"
                    ),
                },
            );
        }
    }

    if seen_qubits.len() != qubits {
        return Err(
            ValidationError::QubitCountMismatch {
                expected: qubits,
                actual: seen_qubits.len(),
            },
        );
    }

    // Validate explicit topology representation.
    let mut seen_stabilizers =
        BTreeSet::new();

    for stabilizer in code.stabilizers() {
        let id = stabilizer.id();
        let stabilizer_id =
            StabilizerId::new(id);

        if !seen_stabilizers.insert(id) {
            return Err(
                ValidationError::DuplicateStabilizer {
                    stabilizer: stabilizer_id,
                },
            );
        }

        let support =
            stabilizer.support();

        if support.is_empty() {
            return Err(
                ValidationError::EmptyStabilizer {
                    stabilizer: stabilizer_id,
                },
            );
        }

        if support.len()
            > limits.max_stabilizer_weight
        {
            return Err(
                ValidationError::StabilizerWeightExceeded {
                    stabilizer: stabilizer_id,
                    limit:
                        limits.max_stabilizer_weight,
                    actual: support.len(),
                },
            );
        }

        let mut local =
            BTreeSet::new();

        for &qubit in support {
            if !local.insert(qubit) {
                return Err(
                    ValidationError::DuplicateQubit {
                        qubit,
                    },
                );
            }

            if qubit.index()
                >= qubits
            {
                return Err(
                    ValidationError::QubitOutOfRange {
                        qubit,
                        num_qubits: qubits,
                    },
                );
            }
        }
    }

    if seen_stabilizers.len()
        != stabilizers
    {
        return Err(
            ValidationError::QubitCountMismatch {
                expected: stabilizers,
                actual: seen_stabilizers.len(),
            },
        );
    }

    // Delegate mathematical stabilizer validation to stabilizer.rs rather
    // than duplicating its algebra here.
    let group =
        code.stabilizer_group()
            .map_err(|error| {
                ValidationError::ModuleError {
                    module: "stabilizer",
                    message: error.to_string(),
                }
            })?;

    validate_stabilizer_group(
        &group,
        limits,
    )?;

    // Validate logical-normalizer invariants.
    let logical_x =
        code.logical_x();

    let logical_z =
        code.logical_z();

    if logical_x.operator().is_identity() {
        return Err(
            ValidationError::IdentityLogicalOperator {
                name: logical_x.name(),
            },
        );
    }

    if logical_z.operator().is_identity() {
        return Err(
            ValidationError::IdentityLogicalOperator {
                name: logical_z.name(),
            },
        );
    }

    validate_pauli_string(
        logical_x.operator(),
        qubits,
        limits,
    )?;

    validate_pauli_string(
        logical_z.operator(),
        qubits,
        limits,
    )?;

    let logical_anticommutes =
        logical_x
            .operator()
            .anticommutes_with(
                logical_z.operator(),
            )
            .map_err(|error| {
                ValidationError::ModuleError {
                    module: "stabilizer",
                    message: error.to_string(),
                }
            })?;

    if !logical_anticommutes {
        return Err(
            ValidationError::InvalidLogicalAnticommutation,
        );
    }

    for generator in group.generators() {
        for (name, logical) in [
            (
                logical_x.name(),
                logical_x.operator(),
            ),
            (
                logical_z.name(),
                logical_z.operator(),
            ),
        ] {
            let commutes =
                logical
                    .commutes_with(
                        generator.operator(),
                    )
                    .map_err(|error| {
                        ValidationError::ModuleError {
                            module: "stabilizer",
                            message:
                                error.to_string(),
                        }
                    })?;

            if !commutes {
                return Err(
                    ValidationError::LogicalOperatorViolatesStabilizer {
                        stabilizer:
                            generator.id(),
                        logical: name,
                    },
                );
            }
        }
    }

    Ok(ValidationReport {
        qubits,
        stabilizers,
        syndrome_measurements: 0,
        detection_events: 0,
    })
}

// ============================================================================
// Stabilizer validation
// ============================================================================

/// Validates a stabilizer group against canonical QEC limits.
///
/// Algebraic validation remains implemented by `stabilizer.rs`.
pub fn validate_stabilizer_group(
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let qubits = group.num_qubits();
    let stabilizers = group.len();

    if qubits > limits.max_qubits {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: qubits as u128,
                maximum: limits.max_qubits as u128,
            },
        );
    }

    if stabilizers
        > limits.max_stabilizers
    {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource:
                    LimitKind::Stabilizers,
                requested:
                    stabilizers as u128,
                maximum:
                    limits.max_stabilizers
                        as u128,
            },
        );
    }

    // Let the canonical algebra layer establish commutation, dimensions,
    // rank-related invariants, and internal consistency.
    group.validate()
        .map_err(|error| {
            ValidationError::ModuleError {
                module: "stabilizer",
                message: error.to_string(),
            }
        })?;

    let mut ids =
        BTreeSet::new();

    for generator in group.generators() {
        let id = generator.id();

        if !ids.insert(id) {
            return Err(
                ValidationError::DuplicateStabilizer {
                    stabilizer:
                        StabilizerId::new(id),
                },
            );
        }

        let operator =
            generator.operator();

        validate_pauli_string(
            operator,
            qubits,
            limits,
        )?;

        let weight =
            operator.weight();

        if weight == 0 {
            return Err(
                ValidationError::EmptyStabilizer {
                    stabilizer:
                        StabilizerId::new(id),
                },
            );
        }

        if weight
            > limits.max_stabilizer_weight
        {
            return Err(
                ValidationError::StabilizerWeightExceeded {
                    stabilizer:
                        StabilizerId::new(id),
                    limit:
                        limits.max_stabilizer_weight,
                    actual: weight,
                },
            );
        }
    }

    Ok(ValidationReport {
        qubits,
        stabilizers,
        syndrome_measurements: 0,
        detection_events: 0,
    })
}

// ============================================================================
// Pauli validation
// ============================================================================

/// Validates a Pauli string against a declared qubit count.
pub fn validate_pauli_string(
    operator: &PauliString,
    num_qubits: usize,
    limits: &QecLimits,
) -> Result<(), ValidationError> {
    limits.validate()?;

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested:
                    num_qubits as u128,
                maximum:
                    limits.max_qubits as u128,
            },
        );
    }

    let actual =
        operator.num_qubits();

    if actual != num_qubits {
        return Err(
            ValidationError::OperatorDimensionMismatch {
                expected: num_qubits,
                actual,
            },
        );
    }

    let weight =
        operator.weight();

    if weight
        > limits.max_logical_operator_weight
    {
        return Err(
            ValidationError::OperatorWeightExceeded {
                limit:
                    limits.max_logical_operator_weight,
                actual: weight,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Syndrome validation
// ============================================================================

/// Validates a complete syndrome against a stabilizer group.
///
/// The syndrome and stabilizer domain must match exactly.
pub fn validate_syndrome(
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let measurement_count =
        syndrome.len();

    if measurement_count
        > limits.max_syndrome_events
    {
        return Err(
            ValidationError::SyndromeSizeExceeded {
                limit:
                    limits.max_syndrome_events,
                actual:
                    measurement_count,
            },
        );
    }

    validate_stabilizer_group(
        stabilizers,
        limits,
    )?;

    let expected_ids:
        BTreeSet<StabilizerId> =
        stabilizers
            .generators()
            .iter()
            .map(|generator| {
                StabilizerId::new(
                    generator.id(),
                )
            })
            .collect();

    let actual_ids:
        BTreeSet<StabilizerId> =
        syndrome
            .stabilizer_ids()
            .collect();

    if expected_ids != actual_ids {
        return Err(
            ValidationError::SyndromeStabilizerMismatch,
        );
    }

    Ok(ValidationReport {
        qubits:
            stabilizers.num_qubits(),
        stabilizers:
            stabilizers.len(),
        syndrome_measurements:
            measurement_count,
        detection_events: 0,
    })
}

// ============================================================================
// Detection-event validation
// ============================================================================

/// Validates detection events against a stabilizer group.
///
/// A detection-event identity is `(measurement_round, stabilizer_id)`.
///
/// The same identity cannot occur twice in one batch.
pub fn validate_detection_events(
    events: &[DetectionEvent],
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    if events.len()
        > limits.max_syndrome_events
    {
        return Err(
            ValidationError::DetectionEventCountExceeded {
                limit:
                    limits.max_syndrome_events,
                actual:
                    events.len(),
            },
        );
    }

    validate_stabilizer_group(
        stabilizers,
        limits,
    )?;

    let known:
        BTreeSet<StabilizerId> =
        stabilizers
            .generators()
            .iter()
            .map(|generator| {
                StabilizerId::new(
                    generator.id(),
                )
            })
            .collect();

    let mut seen =
        BTreeSet::new();

    for event in events {
        if !event.value() {
            return Err(
                ValidationError::InactiveDetectionEvent,
            );
        }

        let stabilizer =
            event.stabilizer();

        if !known.contains(
            &stabilizer,
        ) {
            return Err(
                ValidationError::UnknownDetectionStabilizer {
                    stabilizer,
                },
            );
        }

        let round =
            event.round().value();

        if !seen.insert((
            round,
            stabilizer,
        )) {
            return Err(
                ValidationError::DuplicateDetectionEvent {
                    round,
                    stabilizer,
                },
            );
        }
    }

    Ok(ValidationReport {
        qubits:
            stabilizers.num_qubits(),
        stabilizers:
            stabilizers.len(),
        syndrome_measurements: 0,
        detection_events:
            events.len(),
    })
}

// ============================================================================
// Combined pipeline validation
// ============================================================================

/// Validates the complete:
///
/// ```text
/// SurfaceCode
///     ↓
/// StabilizerGroup
///     ↓
/// Syndrome
///     ↓
/// decoder input
/// ```
///
/// This is the preferred high-level validation boundary before decoding.
pub fn validate_pipeline(
    code: &SurfaceCode,
    syndrome: &Syndrome,
    limits: &QecLimits,
) -> Result<ValidationReport, ValidationError> {
    let code_report =
        validate_surface_code_with_limits(
            code,
            limits,
        )?;

    let group =
        code.stabilizer_group()
            .map_err(|error| {
                ValidationError::ModuleError {
                    module: "stabilizer",
                    message: error.to_string(),
                }
            })?;

    let syndrome_report =
        validate_syndrome(
            syndrome,
            &group,
            limits,
        )?;

    Ok(ValidationReport {
        qubits:
            code_report.qubits,
        stabilizers:
            code_report.stabilizers,
        syndrome_measurements:
            syndrome_report
                .syndrome_measurements,
        detection_events: 0,
    })
}

// ============================================================================
// QecResult convenience API
// ============================================================================

/// Canonical-result wrapper for surface-code validation.
pub fn validate_surface_code_result(
    code: &SurfaceCode,
    limits: &QecLimits,
) -> QecResult<ValidationReport> {
    validate_surface_code_with_limits(
        code,
        limits,
    )
    .map_err(QecError::from)
}

/// Canonical-result wrapper for stabilizer validation.
pub fn validate_stabilizer_group_result(
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<ValidationReport> {
    validate_stabilizer_group(
        group,
        limits,
    )
    .map_err(QecError::from)
}

/// Canonical-result wrapper for syndrome validation.
pub fn validate_syndrome_result(
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<ValidationReport> {
    validate_syndrome(
        syndrome,
        stabilizers,
        limits,
    )
    .map_err(QecError::from)
}

/// Canonical-result wrapper for detection-event validation.
pub fn validate_detection_events_result(
    events: &[DetectionEvent],
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<ValidationReport> {
    validate_detection_events(
        events,
        stabilizers,
        limits,
    )
    .map_err(QecError::from)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_limits_are_valid() {
        assert!(
            QecLimits::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn validation_limits_is_only_an_alias() {
        let limits:
            ValidationLimits =
            QecLimits::default();

        assert_eq!(
            limits.schema_version,
            QecLimits::default()
                .schema_version
        );
    }

    #[test]
    fn zero_canonical_limit_is_rejected() {
        let limits =
            QecLimits {
                max_qubits: 0,
                ..QecLimits::default()
            };

        assert!(
            validate_limits(&limits)
                .is_err()
        );
    }

    #[test]
    fn pauli_dimension_is_checked() {
        let operator =
            PauliString::identity(5);

        let limits =
            QecLimits::default();

        assert!(
            validate_pauli_string(
                &operator,
                4,
                &limits,
            )
            .is_err()
        );

        assert!(
            validate_pauli_string(
                &operator,
                5,
                &limits,
            )
            .is_ok()
        );
    }

    #[test]
    fn distance_three_surface_code_validates() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 surface code \
                     should construct",
                );

        let result =
            validate_surface_code(
                &code,
            );

        assert!(
            result.is_ok(),
            "validation failed: {:?}",
            result.err()
        );

        let report =
            result.expect(
                "distance-3 validation \
                 should succeed",
            );

        assert_eq!(
            report.qubits,
            9
        );

        assert_eq!(
            report.stabilizers,
            8
        );
    }

    #[test]
    fn code_resource_validation_rejects_excessive_distance() {
        let limits =
            QecLimits {
                max_code_distance: 3,
                ..QecLimits::default()
            };

        let result =
            validate_code_resources(
                &limits,
                5,
                25,
                24,
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn code_resource_validation_rejects_excessive_qubits() {
        let limits =
            QecLimits {
                max_qubits: 4,
                max_stabilizers: 4,
                ..QecLimits::default()
            };

        let result =
            validate_code_resources(
                &limits,
                3,
                9,
                8,
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn resource_request_uses_canonical_policy() {
        let limits =
            QecLimits {
                max_qubits: 8,
                ..QecLimits::default()
            };

        assert!(
            validate_resource_request(
                &limits,
                LimitKind::Qubits,
                8,
            )
            .is_ok()
        );

        assert!(
            validate_resource_request(
                &limits,
                LimitKind::Qubits,
                9,
            )
            .is_err()
        );
    }
}