//! Zamani Quantum Error Correction — Cross-Module Validation.
//!
//! This module is the validation boundary between potentially untrusted or
//! externally constructed QEC objects and the core decoding algorithms.
//!
//! Design principles
//! -----------------
//!
//! 1. Validation must happen before expensive decoding.
//! 2. Validation must never panic on malformed input.
//! 3. Validation must be deterministic.
//! 4. Validation must not silently repair invalid data.
//! 5. Local modules remain responsible for their own invariants.
//! 6. This module verifies cross-module consistency.
//! 7. Validation must not allocate proportional to an attacker-controlled
//!    value without first checking configured limits.
//!
//! The intended pipeline is:
//!
//! ```text
//! Untrusted / external input
//!          |
//!          v
//!     QEC validation
//!          |
//!     +----+----+
//!     |         |
//!   valid     invalid
//!     |         |
//!     v         v
//!  decoder    QecValidationError
//! ```
//!
//! This module deliberately does not implement decoding, simulation, noise
//! generation, graph construction, or correction. It only establishes that
//! objects entering those systems satisfy their declared invariants.

use std::collections::BTreeSet;
use std::fmt;

use super::stabilizer::{
    PauliString,
    QubitIndex,
    StabilizerGroup,
};
use super::surface_code::{
    SurfaceCode,
};
use super::syndrome::{
    DetectionEvent,
    Syndrome,
    StabilizerId,
};

// ============================================================================
// Validation limits
// ============================================================================

/// Conservative validation-level default for the maximum number of data
/// qubits accepted by one validation operation.
///
/// This is intentionally a validation guard rather than a universal QEC
/// hardware limit. A future `limits.rs` configuration layer can override the
/// policy used by higher-level APIs.
pub const DEFAULT_MAX_QUBITS: usize = 100_000_000;

/// Default maximum number of stabilizers accepted by one validation
/// operation.
pub const DEFAULT_MAX_STABILIZERS: usize = 100_000_000;

/// Default maximum number of syndrome measurements validated in one object.
pub const DEFAULT_MAX_SYNDROME_MEASUREMENTS: usize = 1_000_000;

/// Default maximum number of detection events validated in one batch.
pub const DEFAULT_MAX_DETECTION_EVENTS: usize = 10_000_000;

/// Default maximum stabilizer support size.
///
/// Surface-code stabilizers normally have weight two or four, but this
/// validator remains generic enough for future QEC families.
pub const DEFAULT_MAX_STABILIZER_WEIGHT: usize = 1_000_000;

// ============================================================================
// Validation policy
// ============================================================================

/// Resource and structural limits used during validation.
///
/// Validation is deliberately policy-driven. Algorithms should not contain
/// hidden resource assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationLimits {
    /// Maximum number of data qubits.
    pub max_qubits: usize,

    /// Maximum number of stabilizers.
    pub max_stabilizers: usize,

    /// Maximum syndrome measurements.
    pub max_syndrome_measurements: usize,

    /// Maximum detection events in one validation batch.
    pub max_detection_events: usize,

    /// Maximum support size for one stabilizer/operator.
    pub max_stabilizer_weight: usize,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_stabilizers: DEFAULT_MAX_STABILIZERS,
            max_syndrome_measurements:
                DEFAULT_MAX_SYNDROME_MEASUREMENTS,
            max_detection_events:
                DEFAULT_MAX_DETECTION_EVENTS,
            max_stabilizer_weight:
                DEFAULT_MAX_STABILIZER_WEIGHT,
        }
    }
}

impl ValidationLimits {
    /// Validates the validation policy itself.
    pub fn validate(self) -> Result<(), ValidationError> {
        if self.max_qubits == 0 {
            return Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_qubits",
                },
            );
        }

        if self.max_stabilizers == 0 {
            return Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_stabilizers",
                },
            );
        }

        if self.max_syndrome_measurements == 0 {
            return Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_syndrome_measurements",
                },
            );
        }

        if self.max_detection_events == 0 {
            return Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_detection_events",
                },
            );
        }

        if self.max_stabilizer_weight == 0 {
            return Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_stabilizer_weight",
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Validation errors
// ============================================================================

/// Unified validation error for cross-module QEC invariants.
///
/// Module-specific errors remain available from their respective modules.
/// This type provides a stable boundary for callers that need one validation
/// result type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The validation policy itself is invalid.
    InvalidValidationLimit {
        field: &'static str,
    },

    /// An object exceeds a configured resource boundary.
    ResourceLimitExceeded {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },

    /// The surface-code distance is invalid.
    InvalidCodeDistance {
        distance: usize,
    },

    /// The number of qubits implied by the code is inconsistent.
    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// A qubit identifier is outside the declared qubit space.
    QubitOutOfRange {
        qubit: QubitIndex,
        num_qubits: usize,
    },

    /// A qubit appears more than once in a support list.
    DuplicateQubit {
        qubit: QubitIndex,
    },

    /// A stabilizer identifier appears more than once.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// A stabilizer support is empty.
    EmptyStabilizer {
        stabilizer: StabilizerId,
    },

    /// A stabilizer support exceeds the configured limit.
    StabilizerWeightExceeded {
        stabilizer: StabilizerId,
        limit: usize,
        actual: usize,
    },

    /// Two stabilizer generators do not commute.
    NonCommutingStabilizers {
        first: usize,
        second: usize,
    },

    /// A Pauli operator has the wrong number of qubits.
    OperatorDimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// A logical operator is the identity.
    IdentityLogicalOperator {
        name: &'static str,
    },

    /// Logical X and Z do not have the required anticommutation relation.
    InvalidLogicalAnticommutation,

    /// A logical operator does not commute with a stabilizer.
    LogicalOperatorViolatesStabilizer {
        stabilizer: usize,
        logical: &'static str,
    },

    /// Syndrome size exceeds the configured bound.
    SyndromeSizeExceeded {
        limit: usize,
        actual: usize,
    },

    /// Syndrome identifiers are inconsistent with the expected stabilizer
    /// set.
    SyndromeStabilizerMismatch,

    /// A detection-event batch exceeds the configured bound.
    DetectionEventCountExceeded {
        limit: usize,
        actual: usize,
    },

    /// Detection event references a stabilizer that is not part of the
    /// validated stabilizer set.
    UnknownDetectionStabilizer {
        stabilizer: StabilizerId,
    },

    /// An event is marked inactive. Detection-event streams should only
    /// contain actual events.
    InactiveDetectionEvent,

    /// A wrapped module-level validation error.
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
            Self::InvalidValidationLimit { field } => {
                write!(
                    f,
                    "invalid validation limit: {field} must be non-zero"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    f,
                    "{resource} exceeds validation limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::InvalidCodeDistance { distance } => {
                write!(
                    f,
                    "invalid surface-code distance: {distance}"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "qubit-count mismatch: expected={expected}, actual={actual}"
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
                    "stabilizers {first} and {second} anticommute"
                )
            }

            Self::OperatorDimensionMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "operator dimension mismatch: \
                     expected={expected}, actual={actual}"
                )
            }

            Self::IdentityLogicalOperator { name } => {
                write!(
                    f,
                    "logical operator {name} is identity"
                )
            }

            Self::InvalidLogicalAnticommutation => {
                write!(
                    f,
                    "logical X and logical Z do not anticommute"
                )
            }

            Self::LogicalOperatorViolatesStabilizer {
                stabilizer,
                logical,
            } => {
                write!(
                    f,
                    "logical operator {logical} anticommutes \
                     with stabilizer {stabilizer}"
                )
            }

            Self::SyndromeSizeExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "syndrome exceeds validation limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::SyndromeStabilizerMismatch => {
                write!(
                    f,
                    "syndrome stabilizer set is inconsistent"
                )
            }

            Self::DetectionEventCountExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "detection-event batch exceeds validation limit: \
                     limit={limit}, actual={actual}"
                )
            }

            Self::UnknownDetectionStabilizer {
                stabilizer,
            } => {
                write!(
                    f,
                    "detection event references unknown stabilizer {stabilizer}"
                )
            }

            Self::InactiveDetectionEvent => {
                write!(
                    f,
                    "detection-event batch contains an inactive event"
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
// Validation report
// ============================================================================

/// Deterministic summary produced by a successful validation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationReport {
    /// Number of validated data qubits.
    pub qubits: usize,

    /// Number of validated stabilizers.
    pub stabilizers: usize,

    /// Number of validated syndrome measurements, if applicable.
    pub syndrome_measurements: usize,

    /// Number of validated detection events, if applicable.
    pub detection_events: usize,
}

impl ValidationReport {
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
// Surface-code validation
// ============================================================================

/// Validates a complete surface code and all cross-object invariants.
///
/// This is the preferred validation entry point before passing a
/// `SurfaceCode` into expensive decoding or simulation.
pub fn validate_surface_code(
    code: &SurfaceCode,
) -> Result<ValidationReport, ValidationError> {
    validate_surface_code_with_limits(
        code,
        ValidationLimits::default(),
    )
}

/// Validates a surface code using an explicit resource policy.
pub fn validate_surface_code_with_limits(
    code: &SurfaceCode,
    limits: ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let distance = code.distance();

    if distance < 3
        || distance % 2 == 0
    {
        return Err(
            ValidationError::InvalidCodeDistance {
                distance,
            },
        );
    }

    let qubits =
        code.num_data_qubits();

    let stabilizers =
        code.num_stabilizers();

    if qubits > limits.max_qubits {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: "data qubits",
                limit: limits.max_qubits,
                actual: qubits,
            },
        );
    }

    if stabilizers
        > limits.max_stabilizers
    {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: "stabilizers",
                limit: limits.max_stabilizers,
                actual: stabilizers,
            },
        );
    }

    let expected_qubits =
        distance
            .checked_mul(distance)
            .ok_or(
                ValidationError::ResourceLimitExceeded {
                    resource: "surface-code qubits",
                    limit: limits.max_qubits,
                    actual: usize::MAX,
                },
            )?;

    if qubits != expected_qubits {
        return Err(
            ValidationError::QubitCountMismatch {
                expected: expected_qubits,
                actual: qubits,
            },
        );
    }

    // Validate every data-qubit index and coordinate mapping.
    let mut seen_qubits =
        BTreeSet::new();

    for qubit in
        code.data_qubits()
    {
        let index =
            qubit.index();

        if index.index()
            >= qubits
        {
            return Err(
                ValidationError::QubitOutOfRange {
                    qubit: index,
                    num_qubits: qubits,
                },
            );
        }

        if !seen_qubits.insert(index)
        {
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

        if mapped
            != qubit.coordinate()
        {
            return Err(
                ValidationError::ModuleError {
                    module: "surface_code",
                    message: format!(
                        "qubit {index} coordinate mapping is inconsistent"
                    ),
                },
            );
        }
    }

    if seen_qubits.len()
        != qubits
    {
        return Err(
            ValidationError::QubitCountMismatch {
                expected: qubits,
                actual: seen_qubits.len(),
            },
        );
    }

    // Validate explicit stabilizer topology.
    let mut seen_stabilizers =
        BTreeSet::new();

    for stabilizer in
        code.stabilizers()
    {
        let id =
            stabilizer.id();

        if !seen_stabilizers.insert(id)
        {
            return Err(
                ValidationError::DuplicateStabilizer {
                    stabilizer:
                        StabilizerId::new(id),
                },
            );
        }

        let support =
            stabilizer.support();

        if support.is_empty() {
            return Err(
                ValidationError::EmptyStabilizer {
                    stabilizer:
                        StabilizerId::new(id),
                },
            );
        }

        if support.len()
            > limits.max_stabilizer_weight
        {
            return Err(
                ValidationError::StabilizerWeightExceeded {
                    stabilizer:
                        StabilizerId::new(id),
                    limit:
                        limits.max_stabilizer_weight,
                    actual:
                        support.len(),
                },
            );
        }

        let mut local =
            BTreeSet::new();

        for &qubit in
            support
        {
            if !local.insert(qubit)
            {
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

    // Delegate algebraic validation to the canonical stabilizer subsystem.
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

    // Logical operators are part of the surface-code contract.
    let logical_x =
        code.logical_x();

    let logical_z =
        code.logical_z();

    if logical_x.operator()
        .is_identity()
    {
        return Err(
            ValidationError::IdentityLogicalOperator {
                name: logical_x.name(),
            },
        );
    }

    if logical_z.operator()
        .is_identity()
    {
        return Err(
            ValidationError::IdentityLogicalOperator {
                name: logical_z.name(),
            },
        );
    }

    validate_operator_dimension(
        logical_x.operator(),
        qubits,
    )?;

    validate_operator_dimension(
        logical_z.operator(),
        qubits,
    )?;

    if !logical_x
        .operator()
        .anticommutes_with(
            logical_z.operator(),
        )
        .map_err(|error| {
            ValidationError::ModuleError {
                module: "stabilizer",
                message: error.to_string(),
            }
        })?
    {
        return Err(
            ValidationError::InvalidLogicalAnticommutation,
        );
    }

    for generator in
        group.generators()
    {
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

/// Validates a stabilizer group independently of a particular code family.
pub fn validate_stabilizer_group(
    group: &StabilizerGroup,
    limits: ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let qubits =
        group.num_qubits();

    let stabilizers =
        group.len();

    if qubits > limits.max_qubits {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: "stabilizer-group qubits",
                limit: limits.max_qubits,
                actual: qubits,
            },
        );
    }

    if stabilizers
        > limits.max_stabilizers
    {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: "stabilizer-group generators",
                limit: limits.max_stabilizers,
                actual: stabilizers,
            },
        );
    }

    group.validate()
        .map_err(|error| {
            ValidationError::ModuleError {
                module: "stabilizer",
                message: error.to_string(),
            }
        })?;

    let mut ids =
        BTreeSet::new();

    for generator in
        group.generators()
    {
        if !ids.insert(
            generator.id(),
        ) {
            return Err(
                ValidationError::DuplicateStabilizer {
                    stabilizer:
                        StabilizerId::new(
                            generator.id(),
                        ),
                },
            );
        }

        let operator =
            generator.operator();

        validate_operator_dimension(
            operator,
            qubits,
        )?;

        let weight =
            operator.weight();

        if weight == 0 {
            return Err(
                ValidationError::EmptyStabilizer {
                    stabilizer:
                        StabilizerId::new(
                            generator.id(),
                        ),
                },
            );
        }

        if weight
            > limits.max_stabilizer_weight
        {
            return Err(
                ValidationError::StabilizerWeightExceeded {
                    stabilizer:
                        StabilizerId::new(
                            generator.id(),
                        ),
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
// Pauli operator validation
// ============================================================================

/// Validates a Pauli string against a declared qubit count.
pub fn validate_pauli_string(
    operator: &PauliString,
    num_qubits: usize,
    limits: ValidationLimits,
) -> Result<(), ValidationError> {
    limits.validate()?;

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource: "operator qubits",
                limit: limits.max_qubits,
                actual: num_qubits,
            },
        );
    }

    validate_operator_dimension(
        operator,
        num_qubits,
    )
}

/// Internal dimension check shared by operator validators.
fn validate_operator_dimension(
    operator: &PauliString,
    expected: usize,
) -> Result<(), ValidationError> {
    let actual =
        operator.num_qubits();

    if actual != expected {
        return Err(
            ValidationError::OperatorDimensionMismatch {
                expected,
                actual,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Syndrome validation
// ============================================================================

/// Validates a complete syndrome against a validated stabilizer group.
pub fn validate_syndrome(
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
    limits: ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    let measurement_count =
        syndrome.len();

    if measurement_count
        > limits.max_syndrome_measurements
    {
        return Err(
            ValidationError::SyndromeSizeExceeded {
                limit:
                    limits.max_syndrome_measurements,
                actual:
                    measurement_count,
            },
        );
    }

    validate_stabilizer_group(
        stabilizers,
        limits,
    )?;

    let expected_ids: BTreeSet<StabilizerId> =
        stabilizers
            .generators()
            .iter()
            .map(|generator| {
                StabilizerId::new(
                    generator.id(),
                )
            })
            .collect();

    let actual_ids: BTreeSet<StabilizerId> =
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

/// Validates a detection-event batch against a stabilizer group.
///
/// Detection events must reference known stabilizers and must represent
/// active events. The batch size is checked before deeper validation.
pub fn validate_detection_events(
    events: &[DetectionEvent],
    stabilizers: &StabilizerGroup,
    limits: ValidationLimits,
) -> Result<ValidationReport, ValidationError> {
    limits.validate()?;

    if events.len()
        > limits.max_detection_events
    {
        return Err(
            ValidationError::DetectionEventCountExceeded {
                limit:
                    limits.max_detection_events,
                actual:
                    events.len(),
            },
        );
    }

    validate_stabilizer_group(
        stabilizers,
        limits,
    )?;

    let known: BTreeSet<StabilizerId> =
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

    for event in
        events
    {
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

        // One stabilizer should occur at most once in a single event batch.
        if !seen.insert((
            event.round().value(),
            stabilizer,
        )) {
            return Err(
                ValidationError::DuplicateStabilizer {
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
// Combined validation
// ============================================================================

/// Validates the complete surface-code → stabilizer → syndrome pipeline.
///
/// This is the recommended high-level entry point when all three objects are
/// available before decoding.
pub fn validate_pipeline(
    code: &SurfaceCode,
    syndrome: &Syndrome,
    limits: ValidationLimits,
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
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_limits_are_valid() {
        assert!(
            ValidationLimits::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_limit_is_rejected() {
        let limits =
            ValidationLimits {
                max_qubits: 0,
                ..ValidationLimits::default()
            };

        assert_eq!(
            limits.validate(),
            Err(
                ValidationError::InvalidValidationLimit {
                    field: "max_qubits",
                }
            )
        );
    }

    #[test]
    fn pauli_dimension_is_checked() {
        let operator =
            PauliString::identity(5);

        assert!(
            validate_pauli_string(
                &operator,
                4,
                ValidationLimits::default(),
            )
            .is_err()
        );

        assert!(
            validate_pauli_string(
                &operator,
                5,
                ValidationLimits::default(),
            )
            .is_ok()
        );
    }

    #[test]
    fn surface_code_validation_succeeds_for_distance_three() {
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
            "validation failed: {result:?}"
        );

        let report =
            result.expect(
                "validated code should \
                 produce a report",
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
    fn stabilizer_validation_is_deterministic() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 surface code \
                     should construct",
                );

        let first =
            validate_surface_code(
                &code,
            );

        let second =
            validate_surface_code(
                &code,
            );

        assert_eq!(
            first,
            second
        );
    }
}