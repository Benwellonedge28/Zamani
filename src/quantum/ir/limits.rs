//! Zamani Quantum IR — Resource Limits
//!
//! Defines the resource-safety policy for the hardware-independent quantum IR.
//!
//! `QuantumIrLimits` is intentionally independent from the rest of the IR.
//! It does not depend on `Gate`, `QuantumCircuit`, `Measurement`, `Qubit`, or
//! any downstream compiler stage.
//!
//! The limits are consumed by:
//!
//! - circuit construction;
//! - gate validation;
//! - measurement validation;
//! - whole-circuit validation;
//! - circuit analysis;
//! - deserialization/replay;
//! - optimization passes;
//! - frontend lowering;
//! - external/untrusted IR ingestion.
//!
//! Design goals:
//!
//! 1. Prevent attacker-controlled or malformed IR from causing uncontrolled
//!    allocation or computational work.
//! 2. Make resource policy explicit and deterministic.
//! 3. Avoid hidden "unlimited" defaults.
//! 4. Perform all checks without allocation.
//! 5. Make arithmetic overflow-safe.
//! 6. Keep this module independent so its API can be frozen before the rest
//!    of the IR is integrated.
//!
//! Resource limits are policy, not circuit semantics. A backend may impose
//! stricter limits later, but the canonical IR must never silently exceed the
//! limits selected for the IR operation.

use std::fmt;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced while validating or applying IR resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsError {
    /// A configured limit is internally invalid.
    InvalidConfiguration {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid value.
        value: usize,
    },

    /// A requested resource count exceeds its configured limit.
    ResourceExceeded {
        /// Name of the resource.
        resource: &'static str,

        /// Requested amount.
        requested: usize,

        /// Maximum permitted amount.
        maximum: usize,
    },

    /// An addition required for a resource calculation overflowed `usize`.
    ArithmeticOverflow {
        /// Name of the calculation/resource.
        resource: &'static str,
    },

    /// A multiplication required for a resource calculation overflowed
    /// `usize`.
    ArithmeticMultiplicationOverflow {
        /// Name of the calculation/resource.
        resource: &'static str,
    },
}

impl fmt::Display for LimitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, value } => {
                write!(
                    f,
                    "invalid quantum IR limit `{field}`: value {value}"
                )
            }

            Self::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "quantum IR resource limit exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "arithmetic overflow while checking quantum IR resource \
                     `{resource}`"
                )
            }

            Self::ArithmeticMultiplicationOverflow { resource } => {
                write!(
                    f,
                    "arithmetic multiplication overflow while checking \
                     quantum IR resource `{resource}`"
                )
            }
        }
    }
}

impl std::error::Error for LimitsError {}

// -----------------------------------------------------------------------------
// Quantum IR limits
// -----------------------------------------------------------------------------

/// Resource-safety limits for the canonical Zamani quantum IR.
///
/// All values represent hard upper bounds. There is intentionally no implicit
/// "unlimited" sentinel.
///
/// A value of zero is valid for ordinary resource limits and means that the
/// corresponding resource is prohibited. For example, `max_operations = 0`
/// permits only an empty circuit.
///
/// The two execution-budget limits,
/// `max_validation_steps` and `max_analysis_steps`, must be non-zero because
/// validation and analysis must have at least one permitted unit of work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumIrLimits {
    /// Maximum number of logical qubits in one IR object.
    max_qubits: usize,

    /// Maximum number of classical bits in one IR object.
    max_classical_bits: usize,

    /// Maximum number of operations in one circuit.
    max_operations: usize,

    /// Maximum number of qubit operands accepted by one operation.
    max_operands: usize,

    /// Maximum number of parameters accepted by one operation.
    max_parameters: usize,

    /// Maximum number of metadata bytes associated with one circuit.
    ///
    /// This is measured as UTF-8 byte length, not character count.
    max_metadata_bytes: usize,

    /// Maximum logical circuit depth.
    max_depth: usize,

    /// Maximum number of measurement operations in one circuit.
    max_measurements: usize,

    /// Maximum number of barrier operations in one circuit.
    max_barriers: usize,

    /// Maximum number of abstract validation steps permitted for one
    /// validation invocation.
    ///
    /// This is a deterministic work budget, not a wall-clock timeout.
    max_validation_steps: usize,

    /// Maximum number of abstract analysis steps permitted for one analysis
    /// invocation.
    ///
    /// This is a deterministic work budget, not a wall-clock timeout.
    max_analysis_steps: usize,
}

impl QuantumIrLimits {
    // -------------------------------------------------------------------------
    // Production defaults
    // -------------------------------------------------------------------------

    /// Production-oriented default limits.
    ///
    /// These values deliberately bound memory and computational work while
    /// leaving enough capacity for substantial quantum programs.
    ///
    /// Applications embedding Zamani may select stricter limits explicitly.
    pub const DEFAULT_MAX_QUBITS: usize = 4096;

    /// Default maximum number of classical bits.
    pub const DEFAULT_MAX_CLASSICAL_BITS: usize = 4096;

    /// Default maximum number of operations.
    pub const DEFAULT_MAX_OPERATIONS: usize = 1_000_000;

    /// Default maximum number of operands per operation.
    pub const DEFAULT_MAX_OPERANDS: usize = 64;

    /// Default maximum number of parameters per operation.
    pub const DEFAULT_MAX_PARAMETERS: usize = 16;

    /// Default maximum metadata size in bytes.
    pub const DEFAULT_MAX_METADATA_BYTES: usize = 64 * 1024;

    /// Default maximum logical circuit depth.
    pub const DEFAULT_MAX_DEPTH: usize = 1_000_000;

    /// Default maximum number of measurements.
    pub const DEFAULT_MAX_MEASUREMENTS: usize = 4096;

    /// Default maximum number of barriers.
    pub const DEFAULT_MAX_BARRIERS: usize = 4096;

    /// Default validation work budget.
    ///
    /// This is intentionally larger than the operation limit because a
    /// validator may perform more than one bounded check per operation.
    pub const DEFAULT_MAX_VALIDATION_STEPS: usize = 10_000_000;

    /// Default analysis work budget.
    pub const DEFAULT_MAX_ANALYSIS_STEPS: usize = 10_000_000;

    /// Creates the production default resource policy.
    pub const fn production() -> Self {
        Self {
            max_qubits: Self::DEFAULT_MAX_QUBITS,
            max_classical_bits: Self::DEFAULT_MAX_CLASSICAL_BITS,
            max_operations: Self::DEFAULT_MAX_OPERATIONS,
            max_operands: Self::DEFAULT_MAX_OPERANDS,
            max_parameters: Self::DEFAULT_MAX_PARAMETERS,
            max_metadata_bytes: Self::DEFAULT_MAX_METADATA_BYTES,
            max_depth: Self::DEFAULT_MAX_DEPTH,
            max_measurements: Self::DEFAULT_MAX_MEASUREMENTS,
            max_barriers: Self::DEFAULT_MAX_BARRIERS,
            max_validation_steps: Self::DEFAULT_MAX_VALIDATION_STEPS,
            max_analysis_steps: Self::DEFAULT_MAX_ANALYSIS_STEPS,
        }
    }

    // -------------------------------------------------------------------------
    // Constructors
    // -------------------------------------------------------------------------

    /// Creates a new limits policy.
    ///
    /// This constructor does not silently clamp values. Invalid configurations
    /// are rejected by [`QuantumIrLimits::validate`].
    pub const fn new(
        max_qubits: usize,
        max_classical_bits: usize,
        max_operations: usize,
        max_operands: usize,
        max_parameters: usize,
        max_metadata_bytes: usize,
        max_depth: usize,
        max_measurements: usize,
        max_barriers: usize,
        max_validation_steps: usize,
        max_analysis_steps: usize,
    ) -> Self {
        Self {
            max_qubits,
            max_classical_bits,
            max_operations,
            max_operands,
            max_parameters,
            max_metadata_bytes,
            max_depth,
            max_measurements,
            max_barriers,
            max_validation_steps,
            max_analysis_steps,
        }
    }

    /// Creates limits with all resource values set to zero except the
    /// execution budgets.
    ///
    /// This is useful for deny-by-default policies and security tests.
    pub const fn deny_all() -> Self {
        Self {
            max_qubits: 0,
            max_classical_bits: 0,
            max_operations: 0,
            max_operands: 0,
            max_parameters: 0,
            max_metadata_bytes: 0,
            max_depth: 0,
            max_measurements: 0,
            max_barriers: 0,
            max_validation_steps: 1,
            max_analysis_steps: 1,
        }
    }

    // -------------------------------------------------------------------------
    // Builder-style configuration
    // -------------------------------------------------------------------------

    /// Sets the maximum logical qubit count.
    pub const fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    /// Sets the maximum classical-bit count.
    pub const fn with_max_classical_bits(mut self, value: usize) -> Self {
        self.max_classical_bits = value;
        self
    }

    /// Sets the maximum operation count.
    pub const fn with_max_operations(mut self, value: usize) -> Self {
        self.max_operations = value;
        self
    }

    /// Sets the maximum operand count per operation.
    pub const fn with_max_operands(mut self, value: usize) -> Self {
        self.max_operands = value;
        self
    }

    /// Sets the maximum parameter count per operation.
    pub const fn with_max_parameters(mut self, value: usize) -> Self {
        self.max_parameters = value;
        self
    }

    /// Sets the maximum metadata size in bytes.
    pub const fn with_max_metadata_bytes(mut self, value: usize) -> Self {
        self.max_metadata_bytes = value;
        self
    }

    /// Sets the maximum circuit depth.
    pub const fn with_max_depth(mut self, value: usize) -> Self {
        self.max_depth = value;
        self
    }

    /// Sets the maximum measurement count.
    pub const fn with_max_measurements(mut self, value: usize) -> Self {
        self.max_measurements = value;
        self
    }

    /// Sets the maximum barrier count.
    pub const fn with_max_barriers(mut self, value: usize) -> Self {
        self.max_barriers = value;
        self
    }

    /// Sets the maximum validation work budget.
    pub const fn with_max_validation_steps(mut self, value: usize) -> Self {
        self.max_validation_steps = value;
        self
    }

    /// Sets the maximum analysis work budget.
    pub const fn with_max_analysis_steps(mut self, value: usize) -> Self {
        self.max_analysis_steps = value;
        self
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Maximum logical qubits.
    pub const fn max_qubits(&self) -> usize {
        self.max_qubits
    }

    /// Maximum classical bits.
    pub const fn max_classical_bits(&self) -> usize {
        self.max_classical_bits
    }

    /// Maximum operations.
    pub const fn max_operations(&self) -> usize {
        self.max_operations
    }

    /// Maximum operands per operation.
    pub const fn max_operands(&self) -> usize {
        self.max_operands
    }

    /// Maximum parameters per operation.
    pub const fn max_parameters(&self) -> usize {
        self.max_parameters
    }

    /// Maximum metadata bytes.
    pub const fn max_metadata_bytes(&self) -> usize {
        self.max_metadata_bytes
    }

    /// Maximum logical circuit depth.
    pub const fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Maximum measurements.
    pub const fn max_measurements(&self) -> usize {
        self.max_measurements
    }

    /// Maximum barriers.
    pub const fn max_barriers(&self) -> usize {
        self.max_barriers
    }

    /// Maximum validation steps.
    pub const fn max_validation_steps(&self) -> usize {
        self.max_validation_steps
    }

    /// Maximum analysis steps.
    pub const fn max_analysis_steps(&self) -> usize {
        self.max_analysis_steps
    }

    // -------------------------------------------------------------------------
    // Configuration validation
    // -------------------------------------------------------------------------

    /// Validates this limit configuration.
    ///
    /// Ordinary resource limits may be zero. A zero limit means the resource
    /// is prohibited.
    ///
    /// Validation and analysis budgets must be non-zero because a zero budget
    /// would make the corresponding operation impossible to execute.
    pub const fn validate(&self) -> Result<(), LimitsError> {
        if self.max_validation_steps == 0 {
            return Err(LimitsError::InvalidConfiguration {
                field: "max_validation_steps",
                value: self.max_validation_steps,
            });
        }

        if self.max_analysis_steps == 0 {
            return Err(LimitsError::InvalidConfiguration {
                field: "max_analysis_steps",
                value: self.max_analysis_steps,
            });
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Resource checks
    // -------------------------------------------------------------------------

    /// Checks a logical-qubit count.
    pub const fn check_qubits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "logical qubits",
            requested,
            self.max_qubits,
        )
    }

    /// Checks a classical-bit count.
    pub const fn check_classical_bits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "classical bits",
            requested,
            self.max_classical_bits,
        )
    }

    /// Checks an operation count.
    pub const fn check_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "operations",
            requested,
            self.max_operations,
        )
    }

    /// Checks an operand count for one operation.
    pub const fn check_operands(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "operands per operation",
            requested,
            self.max_operands,
        )
    }

    /// Checks a parameter count for one operation.
    pub const fn check_parameters(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "parameters per operation",
            requested,
            self.max_parameters,
        )
    }

    /// Checks metadata size in bytes.
    pub const fn check_metadata_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "metadata bytes",
            requested,
            self.max_metadata_bytes,
        )
    }

    /// Checks logical circuit depth.
    pub const fn check_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "circuit depth",
            requested,
            self.max_depth,
        )
    }

    /// Checks measurement count.
    pub const fn check_measurements(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "measurements",
            requested,
            self.max_measurements,
        )
    }

    /// Checks barrier count.
    pub const fn check_barriers(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "barriers",
            requested,
            self.max_barriers,
        )
    }

    /// Checks validation work.
    pub const fn check_validation_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "validation steps",
            requested,
            self.max_validation_steps,
        )
    }

    /// Checks analysis work.
    pub const fn check_analysis_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            "analysis steps",
            requested,
            self.max_analysis_steps,
        )
    }

    // -------------------------------------------------------------------------
    // Overflow-safe accounting
    // -------------------------------------------------------------------------

    /// Adds two resource quantities and verifies the result against a limit.
    ///
    /// This is useful for mutation operations such as:
    ///
    /// `current_operations + incoming_operations`.
    ///
    /// It rejects integer overflow rather than wrapping.
    pub const fn checked_add(
        resource: &'static str,
        current: usize,
        additional: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match current.checked_add(additional) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => Err(
                LimitsError::ResourceExceeded {
                    resource,
                    requested: total,
                    maximum,
                },
            ),

            None => Err(
                LimitsError::ArithmeticOverflow {
                    resource,
                },
            ),
        }
    }

    /// Multiplies two resource quantities and verifies the result against a
    /// limit.
    ///
    /// This is useful for conservative work estimates such as:
    ///
    /// `operations * operands`.
    ///
    /// It rejects integer overflow rather than wrapping.
    pub const fn checked_mul(
        resource: &'static str,
        left: usize,
        right: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match left.checked_mul(right) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => Err(
                LimitsError::ResourceExceeded {
                    resource,
                    requested: total,
                    maximum,
                },
            ),

            None => Err(
                LimitsError::ArithmeticMultiplicationOverflow {
                    resource,
                },
            ),
        }
    }

    /// Checks a requested resource amount against a maximum.
    const fn check(
        resource: &'static str,
        requested: usize,
        maximum: usize,
    ) -> Result<(), LimitsError> {
        if requested > maximum {
            return Err(
                LimitsError::ResourceExceeded {
                    resource,
                    requested,
                    maximum,
                },
            );
        }

        Ok(())
    }
}

impl Default for QuantumIrLimits {
    fn default() -> Self {
        Self::production()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_defaults_are_valid() {
        let limits = QuantumIrLimits::production();

        assert!(
            limits.validate().is_ok()
        );
    }

    #[test]
    fn default_matches_production_policy() {
        assert_eq!(
            QuantumIrLimits::default(),
            QuantumIrLimits::production()
        );
    }

    #[test]
    fn production_limits_are_non_zero_for_execution_budgets() {
        let limits = QuantumIrLimits::production();

        assert!(
            limits.max_validation_steps() > 0
        );

        assert!(
            limits.max_analysis_steps() > 0
        );
    }

    #[test]
    fn zero_resource_limits_are_valid() {
        let limits = QuantumIrLimits::deny_all();

        assert!(
            limits.validate().is_ok()
        );

        assert!(
            limits.check_qubits(0).is_ok()
        );

        assert!(
            limits.check_operations(0).is_ok()
        );

        assert!(
            limits.check_metadata_bytes(0).is_ok()
        );
    }

    #[test]
    fn zero_validation_budget_is_invalid() {
        let limits =
            QuantumIrLimits::production()
                .with_max_validation_steps(0);

        assert_eq!(
            limits.validate(),
            Err(
                LimitsError::InvalidConfiguration {
                    field: "max_validation_steps",
                    value: 0,
                }
            )
        );
    }

    #[test]
    fn zero_analysis_budget_is_invalid() {
        let limits =
            QuantumIrLimits::production()
                .with_max_analysis_steps(0);

        assert_eq!(
            limits.validate(),
            Err(
                LimitsError::InvalidConfiguration {
                    field: "max_analysis_steps",
                    value: 0,
                }
            )
        );
    }

    #[test]
    fn resource_at_limit_is_accepted() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(8);

        assert!(
            limits.check_qubits(8).is_ok()
        );
    }

    #[test]
    fn resource_above_limit_is_rejected() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(8);

        assert_eq!(
            limits.check_qubits(9),
            Err(
                LimitsError::ResourceExceeded {
                    resource: "logical qubits",
                    requested: 9,
                    maximum: 8,
                }
            )
        );
    }

    #[test]
    fn operation_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_operations(4);

        assert!(
            limits.check_operations(4).is_ok()
        );

        assert!(
            matches!(
                limits.check_operations(5),
                Err(
                    LimitsError::ResourceExceeded {
                        resource: "operations",
                        requested: 5,
                        maximum: 4,
                    }
                )
            )
        );
    }

    #[test]
    fn operand_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_operands(2);

        assert!(
            limits.check_operands(2).is_ok()
        );

        assert!(
            limits.check_operands(3).is_err()
        );
    }

    #[test]
    fn parameter_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_parameters(4);

        assert!(
            limits.check_parameters(4).is_ok()
        );

        assert!(
            limits.check_parameters(5).is_err()
        );
    }

    #[test]
    fn metadata_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_metadata_bytes(1024);

        assert!(
            limits.check_metadata_bytes(1024).is_ok()
        );

        assert!(
            limits.check_metadata_bytes(1025).is_err()
        );
    }

    #[test]
    fn depth_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_depth(100);

        assert!(
            limits.check_depth(100).is_ok()
        );

        assert!(
            limits.check_depth(101).is_err()
        );
    }

    #[test]
    fn measurement_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_measurements(8);

        assert!(
            limits.check_measurements(8).is_ok()
        );

        assert!(
            limits.check_measurements(9).is_err()
        );
    }

    #[test]
    fn barrier_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_barriers(8);

        assert!(
            limits.check_barriers(8).is_ok()
        );

        assert!(
            limits.check_barriers(9).is_err()
        );
    }

    #[test]
    fn validation_work_budget_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_validation_steps(100);

        assert!(
            limits.check_validation_steps(100).is_ok()
        );

        assert!(
            limits.check_validation_steps(101).is_err()
        );
    }

    #[test]
    fn analysis_work_budget_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_analysis_steps(100);

        assert!(
            limits.check_analysis_steps(100).is_ok()
        );

        assert!(
            limits.check_analysis_steps(101).is_err()
        );
    }

    #[test]
    fn checked_add_accepts_exact_limit() {
        let result =
            QuantumIrLimits::checked_add(
                "operations",
                5,
                5,
                10,
            );

        assert_eq!(result, Ok(10));
    }

    #[test]
    fn checked_add_rejects_limit_excess() {
        let result =
            QuantumIrLimits::checked_add(
                "operations",
                6,
                5,
                10,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ResourceExceeded {
                    resource: "operations",
                    requested: 11,
                    maximum: 10,
                }
            )
        );
    }

    #[test]
    fn checked_add_rejects_integer_overflow() {
        let result =
            QuantumIrLimits::checked_add(
                "operations",
                usize::MAX,
                1,
                usize::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ArithmeticOverflow {
                    resource: "operations",
                }
            )
        );
    }

    #[test]
    fn checked_mul_accepts_exact_limit() {
        let result =
            QuantumIrLimits::checked_mul(
                "validation work",
                10,
                10,
                100,
            );

        assert_eq!(result, Ok(100));
    }

    #[test]
    fn checked_mul_rejects_limit_excess() {
        let result =
            QuantumIrLimits::checked_mul(
                "validation work",
                11,
                10,
                100,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ResourceExceeded {
                    resource: "validation work",
                    requested: 110,
                    maximum: 100,
                }
            )
        );
    }

    #[test]
    fn checked_mul_rejects_integer_overflow() {
        let result =
            QuantumIrLimits::checked_mul(
                "analysis work",
                usize::MAX,
                2,
                usize::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ArithmeticMultiplicationOverflow {
                    resource: "analysis work",
                }
            )
        );
    }

    #[test]
    fn builder_methods_are_composable() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(32)
                .with_max_classical_bits(32)
                .with_max_operations(1024)
                .with_max_operands(8)
                .with_max_parameters(4)
                .with_max_metadata_bytes(4096)
                .with_max_depth(512)
                .with_max_measurements(32)
                .with_max_barriers(32)
                .with_max_validation_steps(10_000)
                .with_max_analysis_steps(10_000);

        assert_eq!(
            limits.max_qubits(),
            32
        );

        assert_eq!(
            limits.max_classical_bits(),
            32
        );

        assert_eq!(
            limits.max_operations(),
            1024
        );

        assert_eq!(
            limits.max_operands(),
            8
        );

        assert_eq!(
            limits.max_parameters(),
            4
        );

        assert_eq!(
            limits.max_metadata_bytes(),
            4096
        );

        assert_eq!(
            limits.max_depth(),
            512
        );

        assert_eq!(
            limits.max_measurements(),
            32
        );

        assert_eq!(
            limits.max_barriers(),
            32
        );

        assert_eq!(
            limits.max_validation_steps(),
            10_000
        );

        assert_eq!(
            limits.max_analysis_steps(),
            10_000
        );

        assert!(
            limits.validate().is_ok()
        );
    }

    #[test]
    fn checks_are_allocation_free() {
        let limits =
            QuantumIrLimits::production();

        assert!(
            limits.check_qubits(1).is_ok()
        );

        assert!(
            limits.check_classical_bits(1).is_ok()
        );

        assert!(
            limits.check_operations(1).is_ok()
        );

        assert!(
            limits.check_operands(1).is_ok()
        );

        assert!(
            limits.check_parameters(1).is_ok()
        );

        assert!(
            limits.check_metadata_bytes(1).is_ok()
        );

        assert!(
            limits.check_depth(1).is_ok()
        );

        assert!(
            limits.check_measurements(1).is_ok()
        );

        assert!(
            limits.check_barriers(1).is_ok()
        );

        assert!(
            limits.check_validation_steps(1).is_ok()
        );

        assert!(
            limits.check_analysis_steps(1).is_ok()
        );
    }
}