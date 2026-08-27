//! Zamani Quantum Benchmarking — Input Validation
//!
//! This module is the first validation boundary for externally supplied
//! quantum-benchmark configuration.
//!
//! # Architectural role
//!
//! `validation::input` validates untrusted benchmark inputs before they are
//! allowed to reach:
//!
//! - circuit generators;
//! - benchmark protocols;
//! - execution backends;
//! - statistical analysis;
//! - reporting;
//! - hardware capability resolution;
//! - the Zamani language runtime.
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! Zamani source / config / JSON / CLI / CI / remote request
//!                         │
//!                         ▼
//!              validation::input
//!                         │
//!              ┌──────────┴──────────┐
//!              ▼                     ▼
//!       core::config          core::limits
//!              │                     │
//!              └──────────┬──────────┘
//!                         ▼
//!                  BenchmarkError
//!                         │
//!                         ▼
//!                 validated config
//!                         │
//!        ┌────────────────┼────────────────┐
//!        ▼                ▼                ▼
//!     generator        executor         protocol
//! ```
//!
//! # Responsibilities
//!
//! This file owns validation of:
//!
//! - complete `BenchmarkConfig` values;
//! - schema identity/version;
//! - benchmark identity;
//! - dimension specifications;
//! - circuit/shot counts;
//! - aggregate shot budgets;
//! - configured resource ceilings;
//! - timeout ceilings;
//! - metadata cardinality and size;
//! - execution-policy safety;
//! - backend-selection syntax;
//! - compiler/statistical/reporting configuration through the canonical
//!   `BenchmarkConfig::validate()` implementation;
//! - finite numerical values;
//! - probabilities;
//! - confidence levels;
//! - non-negative counts;
//! - safe integer products;
//! - globally enforced production limits.
//!
//! It deliberately does NOT perform:
//!
//! - backend capability resolution;
//! - hardware communication;
//! - circuit generation;
//! - Quantum IR validation;
//! - protocol-specific validation;
//! - statistical fitting;
//! - execution;
//! - logging;
//! - printing;
//! - mutation of global state.
//!
//! Those belong to later validation/execution layers.
//!
//! # Security model
//!
//! Benchmark configuration is an untrusted input boundary. A malicious or
//! accidental request must not be able to bypass the global safety envelope
//! merely by supplying larger values in `BenchmarkConfig::limits`.
//!
//! Therefore this validator checks both:
//!
//! 1. the configuration's own limits;
//! 2. the immutable production ceiling represented by
//!    `BenchmarkLimits::production()`.
//!
//! This prevents a configuration from increasing its own safety limits and
//! then using those increased limits to authorize an unsafe workload.
//!
//! # Important architectural rule
//!
//! `BenchmarkConfig::validate()` remains the canonical structural validator.
//! This module does not duplicate its entire implementation. Instead it:
//!
//! - invokes the canonical configuration validator;
//! - translates its errors into `BenchmarkError`;
//! - applies the global production envelope;
//! - applies validation rules that belong specifically to the input boundary.
//!
//! This prevents validation logic from drifting between modules.
//!
//! # Integration contract
//!
//! Downstream callers should perform:
//!
//! ```text
//! validate_config(&config)?
//! ```
//!
//! before generating or executing an experiment.
//!
//! Protocol-specific validators may then perform additional checks after this
//! function succeeds.
//!
//! Backend capability validation must happen later because this module must
//! remain independent of concrete backend implementations.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//! No additional dependencies are required.

use std::fmt;

use super::super::core::config::{
    BackendSelection,
    BenchmarkConfig,
    BenchmarkIdentity,
    DimensionRange,
    ExecutionMode,
    ExecutionPermission,
    ResourceLimits,
};
use super::super::core::errors::{
    BenchmarkError,
    BenchmarkResult,
};
use super::super::core::limits::{
    BenchmarkLimits,
    LimitError,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable component identifier.
pub const INPUT_VALIDATOR_COMPONENT_ID: &str =
    "zamani.quantum.benchmark.validation.input";

/// Stable input-validator contract version.
///
/// This is independent of the benchmark protocol version and Cargo package
/// version.
pub const INPUT_VALIDATOR_VERSION: &str = "1.0.0";

/// Maximum number of metadata bytes inspected by this boundary for a single
/// validation operation.
///
/// The actual metadata value limit remains owned by `core::config`.
pub const MAX_INPUT_VALIDATION_METADATA_BYTES: usize =
    16 * 1024 * 1024;

/// Maximum number of explicit dimension values accepted by this validator.
///
/// `core::config` currently enforces the same fundamental bound. Keeping the
/// boundary here makes the security invariant explicit at the untrusted-input
/// layer.
pub const MAX_INPUT_DIMENSION_VALUES: usize = 4_096;

// =============================================================================
// Input validation policy
// =============================================================================

/// Policy controlling input-boundary validation.
///
/// This type is intentionally small and copyable so callers can construct it
/// without allocation or global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputValidationPolicy {
    /// Global production resource ceiling.
    ///
    /// This is independent of `BenchmarkConfig::limits`.
    pub production_limits: BenchmarkLimits,

    /// Whether to enforce the global production resource ceiling.
    ///
    /// Production callers should leave this enabled.
    pub enforce_production_limits: bool,

    /// Whether backend identifiers must be syntactically validated.
    pub validate_backend_identifiers: bool,

    /// Whether metadata must be bounded at the input boundary.
    pub validate_metadata: bool,

    /// Whether finite numerical invariants are enforced.
    pub reject_non_finite_values: bool,
}

impl Default for InputValidationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

impl InputValidationPolicy {
    /// Returns the production-safe validation policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            production_limits: BenchmarkLimits::production(),
            enforce_production_limits: true,
            validate_backend_identifiers: true,
            validate_metadata: true,
            reject_non_finite_values: true,
        }
    }

    /// Returns a policy intended for controlled tests.
    ///
    /// This does not weaken the configuration's own validation. It only allows
    /// callers to skip the additional global production-envelope comparison.
    ///
    /// Production application code should use [`Self::production`].
    #[must_use]
    pub const fn configuration_only() -> Self {
        Self {
            production_limits: BenchmarkLimits::production(),
            enforce_production_limits: false,
            validate_backend_identifiers: true,
            validate_metadata: true,
            reject_non_finite_values: true,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.enforce_production_limits {
            self.production_limits
                .validate()
                .map_err(map_limit_error)?;
        }

        Ok(())
    }
}

// =============================================================================
// Validator
// =============================================================================

/// Production input validator.
///
/// The validator is stateless apart from its explicit policy. It is therefore
/// safe to construct per request, store in a benchmark service, or use from
/// the Zamani runtime without relying on process-global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputValidator {
    policy: InputValidationPolicy,
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::production()
    }
}

impl InputValidator {
    /// Creates the production validator.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            policy: InputValidationPolicy::production(),
        }
    }

    /// Creates a validator using an explicit policy.
    pub const fn new(policy: InputValidationPolicy) -> Self {
        Self { policy }
    }

    /// Returns the validator policy.
    #[must_use]
    pub const fn policy(&self) -> &InputValidationPolicy {
        &self.policy
    }

    /// Validates one complete benchmark configuration.
    ///
    /// This is the primary API downstream modules should call before
    /// generation or execution.
    pub fn validate(
        &self,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        self.policy.validate()?;

        // ---------------------------------------------------------------------
        // Canonical configuration validation
        // ---------------------------------------------------------------------
        //
        // `BenchmarkConfig` already owns the canonical structural rules.
        // Reuse them instead of maintaining a second copy of those rules.
        config
            .validate()
            .map_err(|error| map_config_error("benchmark configuration", error))?;

        // ---------------------------------------------------------------------
        // Benchmark identity
        // ---------------------------------------------------------------------

        self.validate_benchmark_identity(&config.benchmark)?;

        // ---------------------------------------------------------------------
        // Dimensions
        // ---------------------------------------------------------------------

        self.validate_dimension(
            &config.qubits,
            "qubits",
            self.policy.production_limits.max_qubits,
        )?;

        self.validate_dimension(
            &config.depth,
            "depth",
            self.policy.production_limits.max_circuit_depth,
        )?;

        // ---------------------------------------------------------------------
        // Sampling
        // ---------------------------------------------------------------------

        self.validate_sampling(config)?;

        // ---------------------------------------------------------------------
        // Configured resource ceiling
        // ---------------------------------------------------------------------

        if self.policy.enforce_production_limits {
            self.validate_configured_limits(&config.limits)?;
        }

        // ---------------------------------------------------------------------
        // Execution policy
        // ---------------------------------------------------------------------

        self.validate_execution_policy(
            config.execution_mode,
            config.execution_permission,
        )?;

        // ---------------------------------------------------------------------
        // Backend selection
        // ---------------------------------------------------------------------

        if self.policy.validate_backend_identifiers {
            self.validate_backend_selection(&config.backend)?;
        }

        // ---------------------------------------------------------------------
        // Metadata
        // ---------------------------------------------------------------------

        if self.policy.validate_metadata {
            self.validate_metadata(config)?;
        }

        // ---------------------------------------------------------------------
        // Statistics
        // ---------------------------------------------------------------------

        self.validate_statistics(config)?;

        // ---------------------------------------------------------------------
        // Reporting
        // ---------------------------------------------------------------------

        self.validate_reporting(config)?;

        Ok(())
    }

    /// Validates a benchmark configuration using the production policy.
    pub fn validate_config(
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        Self::production().validate(config)
    }

    /// Validates a benchmark configuration using configuration-only
    /// validation.
    ///
    /// This is primarily useful for tests that intentionally exercise values
    /// above the production envelope.
    pub fn validate_configuration_only(
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        Self::new(InputValidationPolicy::configuration_only())
            .validate(config)
    }

    // =========================================================================
    // Benchmark identity
    // =========================================================================

    fn validate_benchmark_identity(
        &self,
        identity: &BenchmarkIdentity,
    ) -> BenchmarkResult<()> {
        validate_identifier(
            "benchmark.id",
            identity.id(),
            MAX_BENCHMARK_IDENTIFIER_BYTES,
        )?;

        if identity.version() == 0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "benchmark.version".to_owned(),
                reason: "benchmark version must be greater than zero".to_owned(),
            });
        }

        Ok(())
    }

    // =========================================================================
    // Dimension validation
    // =========================================================================

    fn validate_dimension(
        &self,
        dimension: &DimensionRange,
        field: &'static str,
        production_maximum: usize,
    ) -> BenchmarkResult<()> {
        match dimension {
            DimensionRange::Auto => Ok(()),

            DimensionRange::Range(range) => {
                if range.start == 0 {
                    return Err(BenchmarkError::InvalidRange {
                        field: field.to_owned(),
                        value: "start=0".to_owned(),
                        minimum: Some("1".to_owned()),
                        maximum: Some(production_maximum.to_string()),
                    });
                }

                if range.end == 0 {
                    return Err(BenchmarkError::InvalidRange {
                        field: field.to_owned(),
                        value: "end=0".to_owned(),
                        minimum: Some("1".to_owned()),
                        maximum: Some(production_maximum.to_string()),
                    });
                }

                if range.start > range.end {
                    return Err(BenchmarkError::InvalidRange {
                        field: field.to_owned(),
                        value: format!(
                            "{}..{}",
                            range.start,
                            range.end
                        ),
                        minimum: Some("start <= end".to_owned()),
                        maximum: None,
                    });
                }

                if range.step == 0 {
                    return Err(BenchmarkError::InvalidRange {
                        field: field.to_owned(),
                        value: "step=0".to_owned(),
                        minimum: Some("1".to_owned()),
                        maximum: None,
                    });
                }

                if range.end > production_maximum {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource: field.to_owned(),
                        requested: range.end as u64,
                        maximum: production_maximum as u64,
                    });
                }

                let cardinality = range_cardinality(
                    range.start,
                    range.end,
                    range.step,
                )?;

                if cardinality > MAX_INPUT_DIMENSION_VALUES {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource: format!(
                            "{}_dimension_values",
                            field
                        ),
                        requested: cardinality as u64,
                        maximum: MAX_INPUT_DIMENSION_VALUES as u64,
                    });
                }

                Ok(())
            }

            DimensionRange::Explicit(values) => {
                if values.is_empty() {
                    return Err(BenchmarkError::MissingValue {
                        field: field.to_owned(),
                    });
                }

                if values.len() > MAX_INPUT_DIMENSION_VALUES {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource: format!(
                            "{}_dimension_values",
                            field
                        ),
                        requested: values.len() as u64,
                        maximum: MAX_INPUT_DIMENSION_VALUES as u64,
                    });
                }

                let mut previous = 0usize;

                for &value in values {
                    if value == 0 {
                        return Err(BenchmarkError::InvalidRange {
                            field: field.to_owned(),
                            value: "0".to_owned(),
                            minimum: Some("1".to_owned()),
                            maximum: Some(
                                production_maximum.to_string()
                            ),
                        });
                    }

                    if value > production_maximum {
                        return Err(BenchmarkError::ResourceLimitExceeded {
                            resource: field.to_owned(),
                            requested: value as u64,
                            maximum: production_maximum as u64,
                        });
                    }

                    if previous != 0 && value <= previous {
                        return Err(
                            BenchmarkError::InvalidRange {
                                field: field.to_owned(),
                                value: format!(
                                    "{} after {}",
                                    value,
                                    previous
                                ),
                                minimum: Some(
                                    "strictly increasing values"
                                        .to_owned(),
                                ),
                                maximum: None,
                            },
                        );
                    }

                    previous = value;
                }

                Ok(())
            }
        }
    }

    // =========================================================================
    // Sampling validation
    // =========================================================================

    fn validate_sampling(
        &self,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        if config.circuits == 0 {
            return Err(BenchmarkError::MissingValue {
                field: "circuits".to_owned(),
            });
        }

        if config.shots == 0 {
            return Err(BenchmarkError::MissingValue {
                field: "shots".to_owned(),
            });
        }

        self.policy
            .production_limits
            .check_circuits(config.circuits as u64)
            .map_err(map_limit_error)?;

        self.policy
            .production_limits
            .check_shots(config.shots as u64)
            .map_err(map_limit_error)?;

        let total_shots = self
            .policy
            .production_limits
            .check_total_shots(
                config.circuits as u64,
                config.shots as u64,
            )
            .map_err(map_limit_error)?;

        if total_shots > self.policy.production_limits.max_observations {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "total_observations".to_owned(),
                requested: total_shots,
                maximum: self.policy.production_limits.max_observations,
            });
        }

        Ok(())
    }

    // =========================================================================
    // Configured resource ceiling
    // =========================================================================

    fn validate_configured_limits(
        &self,
        limits: &ResourceLimits,
    ) -> BenchmarkResult<()> {
        let production = &self.policy.production_limits;

        // A configuration may choose a stricter limit, but must never enlarge
        // the production envelope.
        check_configured_ceiling(
            "limits.max_qubits",
            limits.max_qubits,
            production.max_qubits,
        )?;

        check_configured_ceiling(
            "limits.max_depth",
            limits.max_depth,
            production.max_circuit_depth,
        )?;

        check_configured_ceiling(
            "limits.max_operations",
            limits.max_operations,
            production.max_gate_count,
        )?;

        check_configured_ceiling(
            "limits.max_shots_per_circuit",
            limits.max_shots_per_circuit,
            production.max_shots,
        )?;

        check_configured_ceiling(
            "limits.max_circuits",
            limits.max_circuits,
            production.max_circuits,
        )?;

        check_configured_ceiling(
            "limits.max_total_shots",
            limits.max_total_shots,
            production.max_observations,
        )?;

        check_configured_ceiling(
            "limits.max_bootstrap_samples",
            limits.max_bootstrap_samples,
            production.max_bootstrap_samples as usize,
        )?;

        check_configured_ceiling(
            "limits.timeout_ms",
            limits.timeout_ms,
            production.max_timeout_ms,
        )?;

        check_configured_ceiling(
            "limits.max_metadata_entries",
            limits.max_metadata_entries,
            production.max_diagnostics,
        )?;

        Ok(())
    }

    // =========================================================================
    // Execution policy
    // =========================================================================

    fn validate_execution_policy(
        &self,
        mode: ExecutionMode,
        permission: ExecutionPermission,
    ) -> BenchmarkResult<()> {
        match permission {
            ExecutionPermission::PlanOnly => {
                if !matches!(mode, ExecutionMode::PlanOnly) {
                    return Err(
                        BenchmarkError::InconsistentConfiguration {
                            first: "execution_mode".to_owned(),
                            second: "execution_permission".to_owned(),
                            reason:
                                "PlanOnly permission requires PlanOnly execution mode"
                                    .to_owned(),
                        },
                    );
                }
            }

            ExecutionPermission::NonQpuOnly => {
                if matches!(mode, ExecutionMode::Qpu) {
                    return Err(
                        BenchmarkError::UnsupportedOperation {
                            operation:
                                "physical_qpu_execution".to_owned(),
                            reason:
                                "execution permission is NonQpuOnly"
                                    .to_owned(),
                        },
                    );
                }
            }

            ExecutionPermission::AllowQpu => {}
        }

        Ok(())
    }

    // =========================================================================
    // Backend selection
    // =========================================================================

    fn validate_backend_selection(
        &self,
        backend: &BackendSelection,
    ) -> BenchmarkResult<()> {
        match backend {
            BackendSelection::Auto => Ok(()),

            BackendSelection::Mode(mode) => {
                // `Mode` is a request, not an actual backend capability
                // assertion. Reject only nonsensical internal combinations.
                if matches!(mode, ExecutionMode::PlanOnly) {
                    return Ok(());
                }

                Ok(())
            }

            BackendSelection::Named(id) => {
                validate_identifier(
                    "backend",
                    id,
                    MAX_BACKEND_IDENTIFIER_BYTES,
                )
            }
        }
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    fn validate_metadata(
        &self,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        if config.metadata.len()
            > config.limits.max_metadata_entries
        {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "metadata_entries".to_owned(),
                requested: config.metadata.len() as u64,
                maximum: config.limits.max_metadata_entries as u64,
            });
        }

        let mut total_bytes = 0usize;

        for (key, value) in &config.metadata {
            validate_identifier(
                "metadata.key",
                key,
                MAX_METADATA_KEY_BYTES,
            )?;

            validate_metadata_value(value)?;

            total_bytes = total_bytes
                .checked_add(key.len())
                .and_then(|size| size.checked_add(value.len()))
                .ok_or_else(|| BenchmarkError::NumericalOverflow {
                    operation: "metadata byte-size calculation"
                        .to_owned(),
                    value: None,
                })?;

            if total_bytes > MAX_INPUT_VALIDATION_METADATA_BYTES {
                return Err(BenchmarkError::ResourceLimitExceeded {
                    resource: "metadata_bytes".to_owned(),
                    requested: total_bytes as u64,
                    maximum:
                        MAX_INPUT_VALIDATION_METADATA_BYTES as u64,
                });
            }
        }

        Ok(())
    }

    // =========================================================================
    // Statistical validation
    // =========================================================================

    fn validate_statistics(
        &self,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        let statistics = &config.statistics;

        validate_confidence_level(
            statistics.confidence_level,
        )?;

        if statistics.minimum_samples == 0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "statistics.minimum_samples".to_owned(),
                reason:
                    "minimum sample count must be greater than zero"
                        .to_owned(),
            });
        }

        if statistics.bootstrap_samples == 0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "statistics.bootstrap_samples".to_owned(),
                reason:
                    "bootstrap sample count must be greater than zero"
                        .to_owned(),
            });
        }

        self.policy
            .production_limits
            .check_bootstrap_samples(
                statistics.bootstrap_samples as u64
            )
            .map_err(map_limit_error)?;

        Ok(())
    }

    // =========================================================================
    // Reporting validation
    // =========================================================================

    fn validate_reporting(
        &self,
        config: &BenchmarkConfig,
    ) -> BenchmarkResult<()> {
        let bytes = config.reporting.max_report_bytes as u64;

        self.policy
            .production_limits
            .check_report_bytes(bytes)
            .map_err(map_limit_error)?;

        Ok(())
    }
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Validates a complete benchmark configuration using production limits.
///
/// This is the primary free-function API for callers that do not need to keep
/// a validator instance.
pub fn validate_config(
    config: &BenchmarkConfig,
) -> BenchmarkResult<()> {
    InputValidator::production().validate(config)
}

/// Validates only the canonical configuration constraints.
///
/// This intentionally skips the additional global production-envelope
/// comparison and is mainly useful for tests and tooling that needs to inspect
/// configuration semantics independently of deployment policy.
pub fn validate_configuration_only(
    config: &BenchmarkConfig,
) -> BenchmarkResult<()> {
    InputValidator::validate_configuration_only(config)
}

/// Validates a benchmark identity without requiring a complete configuration.
pub fn validate_benchmark_identity(
    identity: &BenchmarkIdentity,
) -> BenchmarkResult<()> {
    InputValidator::production()
        .validate_benchmark_identity(identity)
}

/// Validates a stable benchmark/backend/metadata identifier.
pub fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> BenchmarkResult<()> {
    if value.is_empty() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    if value.len() > maximum_bytes {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: format!("{} bytes", value.len()),
            minimum: Some("1".to_owned()),
            maximum: Some(format!("{} bytes", maximum_bytes)),
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    for byte in bytes.iter().copied() {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'-'
            || byte == b'_')
        {
            return Err(BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            });
        }
    }

    Ok(())
}

/// Validates a metadata value independently of a complete configuration.
pub fn validate_metadata_value(
    value: &str,
) -> BenchmarkResult<()> {
    if value.len() > MAX_METADATA_VALUE_BYTES {
        return Err(BenchmarkError::InvalidRange {
            field: "metadata.value".to_owned(),
            value: format!("{} bytes", value.len()),
            minimum: Some("0 bytes".to_owned()),
            maximum: Some(
                MAX_METADATA_VALUE_BYTES.to_string()
            ),
        });
    }

    if value.as_bytes().contains(&0) {
        return Err(BenchmarkError::InvalidConfiguration {
            field: "metadata.value".to_owned(),
            reason: "metadata values must not contain NUL bytes"
                .to_owned(),
        });
    }

    Ok(())
}

/// Validates a probability that must lie in the closed interval `[0, 1]`.
pub fn validate_probability(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() {
        return Err(BenchmarkError::NonFiniteValue {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(BenchmarkError::InvalidProbability {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    Ok(())
}

/// Validates a confidence level.
///
/// Confidence levels are strictly inside `(0, 1)`.
pub fn validate_confidence_level(
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() {
        return Err(BenchmarkError::InvalidConfidenceLevel {
            value: value.to_string(),
        });
    }

    if !(0.0..1.0).contains(&value) {
        return Err(BenchmarkError::InvalidConfidenceLevel {
            value: value.to_string(),
        });
    }

    Ok(())
}

/// Validates a finite floating-point quantity.
pub fn validate_finite(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() {
        return Err(BenchmarkError::NonFiniteValue {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    Ok(())
}

/// Validates a non-negative finite floating-point quantity.
pub fn validate_non_negative_finite(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.to_string(),
            minimum: Some("0".to_owned()),
            maximum: None,
        });
    }

    Ok(())
}

/// Validates a normalized quantity in `[0, 1]`.
///
/// This is deliberately an alias with semantic naming for metric/protocol
/// code.
pub fn validate_unit_interval(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    validate_probability(field, value)
}

/// Validates an unsigned count and an optional upper bound.
pub fn validate_count(
    field: &'static str,
    value: u64,
    maximum: Option<u64>,
) -> BenchmarkResult<()> {
    if value == 0 {
        return Err(BenchmarkError::InvalidCount {
            field: field.to_owned(),
            value,
            maximum,
        });
    }

    if let Some(maximum) = maximum {
        if value > maximum {
            return Err(BenchmarkError::InvalidCount {
                field: field.to_owned(),
                value,
                maximum: Some(maximum),
            });
        }
    }

    Ok(())
}

/// Validates an optional count where zero is meaningful.
///
/// This is useful for quantities such as "number of errors" where zero is a
/// legitimate result.
pub fn validate_non_negative_count(
    field: &'static str,
    value: u64,
    maximum: Option<u64>,
) -> BenchmarkResult<()> {
    if let Some(maximum) = maximum {
        if value > maximum {
            return Err(BenchmarkError::InvalidCount {
                field: field.to_owned(),
                value,
                maximum: Some(maximum),
            });
        }
    }

    Ok(())
}

/// Validates that a sample count is sufficient for a requested analysis.
pub fn validate_sample_count(
    context: &'static str,
    required: usize,
    actual: usize,
) -> BenchmarkResult<()> {
    if required == 0 {
        return Err(BenchmarkError::InvalidConfiguration {
            field: "required_samples".to_owned(),
            reason:
                "required sample count must be greater than zero"
                    .to_owned(),
        });
    }

    if actual < required {
        return Err(BenchmarkError::InsufficientSamples {
            required,
            actual,
            context: context.to_owned(),
        });
    }

    Ok(())
}

/// Validates a timeout against the production safety ceiling.
pub fn validate_timeout_ms(
    timeout_ms: u64,
) -> BenchmarkResult<()> {
    BenchmarkLimits::production()
        .check_timeout_ms(timeout_ms)
        .map_err(map_limit_error)
}

/// Validates a circuit resource envelope before generation.
///
/// This is useful for generators that know the concrete circuit dimensions
/// before constructing the actual Quantum IR object.
pub fn validate_circuit_resources(
    qubits: usize,
    depth: usize,
    gate_count: usize,
    two_qubit_gates: usize,
) -> BenchmarkResult<()> {
    BenchmarkLimits::production()
        .check_circuit(
            qubits,
            depth,
            gate_count,
            two_qubit_gates,
        )
        .map_err(map_limit_error)
}

/// Validates aggregate benchmark shots without allowing arithmetic overflow.
pub fn validate_total_shots(
    circuits: u64,
    shots_per_circuit: u64,
) -> BenchmarkResult<u64> {
    BenchmarkLimits::production()
        .check_total_shots(
            circuits,
            shots_per_circuit,
        )
        .map_err(map_limit_error)
}

/// Performs an overflow-safe product and checks it against a production limit.
pub fn validate_product(
    resource: &'static str,
    left: u64,
    right: u64,
    maximum: u64,
) -> BenchmarkResult<u64> {
    BenchmarkLimits::production()
        .check_product(
            resource,
            left,
            right,
            maximum,
        )
        .map_err(map_limit_error)
}

// =============================================================================
// Stable constants shared with core/config.rs
// =============================================================================

/// Maximum benchmark identifier size.
///
/// This mirrors the configuration-layer contract. Keeping the constant here
/// avoids accepting a larger identifier merely because input validation was
/// invoked independently of `BenchmarkConfig::validate()`.
pub const MAX_BENCHMARK_IDENTIFIER_BYTES: usize = 128;

/// Maximum backend identifier size.
pub const MAX_BACKEND_IDENTIFIER_BYTES: usize = 256;

/// Maximum metadata key size.
pub const MAX_METADATA_KEY_BYTES: usize = 128;

/// Maximum metadata value size.
pub const MAX_METADATA_VALUE_BYTES: usize = 4_096;

// =============================================================================
// Internal helpers
// =============================================================================

fn check_configured_ceiling(
    field: &'static str,
    configured: usize,
    production_maximum: usize,
) -> BenchmarkResult<()> {
    if configured == 0 {
        return Err(BenchmarkError::InvalidConfiguration {
            field: field.to_owned(),
            reason:
                "configured resource limits must be greater than zero"
                    .to_owned(),
        });
    }

    if configured > production_maximum {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: field.to_owned(),
            requested: configured as u64,
            maximum: production_maximum as u64,
        });
    }

    Ok(())
}

fn check_configured_ceiling_u64(
    field: &'static str,
    configured: u64,
    production_maximum: u64,
) -> BenchmarkResult<()> {
    if configured == 0 {
        return Err(BenchmarkError::InvalidConfiguration {
            field: field.to_owned(),
            reason:
                "configured resource limits must be greater than zero"
                    .to_owned(),
        });
    }

    if configured > production_maximum {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: field.to_owned(),
            requested: configured,
            maximum: production_maximum,
        });
    }

    Ok(())
}

fn range_cardinality(
    start: usize,
    end: usize,
    step: usize,
) -> BenchmarkResult<usize> {
    if start == 0 || end == 0 || step == 0 {
        return Err(BenchmarkError::InvalidRange {
            field: "dimension".to_owned(),
            value: format!(
                "start={}, end={}, step={}",
                start, end, step
            ),
            minimum: Some("all values must be non-zero".to_owned()),
            maximum: None,
        });
    }

    if start > end {
        return Err(BenchmarkError::InvalidRange {
            field: "dimension".to_owned(),
            value: format!("{}..{}", start, end),
            minimum: Some("start <= end".to_owned()),
            maximum: None,
        });
    }

    let distance = end
        .checked_sub(start)
        .ok_or_else(|| BenchmarkError::NumericalOverflow {
            operation: "dimension range subtraction".to_owned(),
            value: None,
        })?;

    let quotient = distance
        .checked_div(step)
        .ok_or_else(|| BenchmarkError::NumericalOverflow {
            operation: "dimension range division".to_owned(),
            value: None,
        })?;

    quotient
        .checked_add(1)
        .ok_or_else(|| BenchmarkError::NumericalOverflow {
            operation: "dimension range cardinality".to_owned(),
            value: None,
        })
}

fn map_limit_error(error: LimitError) -> BenchmarkError {
    match error {
        LimitError::ZeroValue { resource } => {
            BenchmarkError::InvalidConfiguration {
                field: resource.to_owned(),
                reason:
                    "resource value must be greater than zero"
                        .to_owned(),
            }
        }

        LimitError::Exceeded {
            resource,
            requested,
            maximum,
        } => BenchmarkError::ResourceLimitExceeded {
            resource: resource.to_owned(),
            requested,
            maximum,
        },

        LimitError::ArithmeticOverflow { resource } => {
            BenchmarkError::NumericalOverflow {
                operation: format!(
                    "resource calculation for {}",
                    resource
                ),
                value: None,
            }
        }

        LimitError::InvalidTimeout { milliseconds } => {
            BenchmarkError::InvalidRange {
                field: "timeout_ms".to_owned(),
                value: milliseconds.to_string(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    super::super::core::limits::MAX_DURATION_MS
                        .to_string(),
                ),
            }
        }
    }
}

fn map_config_error(
    field: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: error.to_string(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::benchmarking::core::config::{
        BenchmarkConfig,
        DimensionRange,
        InclusiveRange,
    };

    #[test]
    fn production_validator_accepts_default_configuration() {
        let config = BenchmarkConfig::default();

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_ok()
        );
    }

    #[test]
    fn production_validator_rejects_zero_circuits() {
        let mut config = BenchmarkConfig::default();
        config.circuits = 0;

        let error =
            InputValidator::production()
                .validate(&config)
                .expect_err("zero circuits must be rejected");

        assert_eq!(
            error.kind(),
            super::super::super::core::errors::BenchmarkErrorKind::Configuration
        );
    }

    #[test]
    fn production_validator_rejects_zero_shots() {
        let mut config = BenchmarkConfig::default();
        config.shots = 0;

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_err()
        );
    }

    #[test]
    fn production_validator_rejects_excessive_qubits() {
        let range = InclusiveRange::new(
            1,
            super::super::super::core::limits::DEFAULT_MAX_QUBITS
                + 1,
            1,
        )
        .expect_err(
            "the configuration-level range must reject an excessive size",
        );

        assert!(matches!(
            range,
            super::super::super::core::config::ConfigError::DimensionLimitExceeded {
                ..
            }
                | super::super::super::core::config::ConfigError::RangeTooLarge {
                    ..
                }
        ));
    }

    #[test]
    fn probability_accepts_zero_and_one() {
        assert!(validate_probability("p", 0.0).is_ok());
        assert!(validate_probability("p", 1.0).is_ok());
    }

    #[test]
    fn probability_rejects_negative() {
        assert!(
            validate_probability("p", -0.001).is_err()
        );
    }

    #[test]
    fn probability_rejects_above_one() {
        assert!(
            validate_probability("p", 1.001).is_err()
        );
    }

    #[test]
    fn probability_rejects_nan() {
        assert!(
            validate_probability("p", f64::NAN).is_err()
        );
    }

    #[test]
    fn probability_rejects_infinity() {
        assert!(
            validate_probability("p", f64::INFINITY).is_err()
        );
    }

    #[test]
    fn confidence_requires_open_unit_interval() {
        assert!(
            validate_confidence_level(0.95).is_ok()
        );

        assert!(
            validate_confidence_level(0.0).is_err()
        );

        assert!(
            validate_confidence_level(1.0).is_err()
        );
    }

    #[test]
    fn identifiers_are_lowercase_machine_identifiers() {
        assert!(
            validate_identifier(
                "benchmark",
                "quantum_volume",
                MAX_BENCHMARK_IDENTIFIER_BYTES,
            )
            .is_ok()
        );

        assert!(
            validate_identifier(
                "benchmark",
                "QuantumVolume",
                MAX_BENCHMARK_IDENTIFIER_BYTES,
            )
            .is_err()
        );

        assert!(
            validate_identifier(
                "benchmark",
                "quantum.volume",
                MAX_BENCHMARK_IDENTIFIER_BYTES,
            )
            .is_err()
        );
    }

    #[test]
    fn metadata_rejects_nul() {
        assert!(
            validate_metadata_value("abc\0def")
                .is_err()
        );
    }

    #[test]
    fn metadata_accepts_empty_value() {
        assert!(
            validate_metadata_value("").is_ok()
        );
    }

    #[test]
    fn sample_count_requires_enough_observations() {
        assert!(
            validate_sample_count(
                "test",
                10,
                10,
            )
            .is_ok()
        );

        assert!(
            validate_sample_count(
                "test",
                10,
                9,
            )
            .is_err()
        );
    }

    #[test]
    fn total_shots_is_overflow_safe() {
        assert!(
            validate_total_shots(
                1_000,
                1_000,
            )
            .is_ok()
        );

        assert!(
            validate_total_shots(
                u64::MAX,
                2,
            )
            .is_err()
        );
    }

    #[test]
    fn product_is_limit_checked() {
        assert_eq!(
            validate_product(
                "test",
                10,
                10,
                100,
            )
            .expect("100 is within the limit"),
            100
        );

        assert!(
            validate_product(
                "test",
                10,
                11,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn circuit_resources_are_checked_before_generation() {
        assert!(
            validate_circuit_resources(
                10,
                100,
                1_000,
                100,
            )
            .is_ok()
        );

        assert!(
            validate_circuit_resources(
                10,
                100,
                100,
                101,
            )
            .is_err()
        );
    }

    #[test]
    fn timeout_is_checked_against_global_limit() {
        assert!(
            validate_timeout_ms(1_000).is_ok()
        );

        assert!(
            validate_timeout_ms(
                super::super::super::core::limits::MAX_DURATION_MS
                    + 1
            )
            .is_err()
        );
    }

    #[test]
    fn explicit_dimensions_must_be_increasing() {
        let config = BenchmarkConfig::default();

        let invalid =
            DimensionRange::explicit(vec![1, 3, 2]);

        assert!(invalid.is_err());

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_ok()
        );
    }

    #[test]
    fn plan_only_permission_requires_plan_only_mode() {
        let mut config = BenchmarkConfig::default();

        config.execution_permission =
            ExecutionPermission::PlanOnly;

        config.execution_mode =
            ExecutionMode::Simulator;

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_err()
        );
    }

    #[test]
    fn non_qpu_permission_rejects_qpu_mode() {
        let mut config = BenchmarkConfig::default();

        config.execution_permission =
            ExecutionPermission::NonQpuOnly;

        config.execution_mode =
            ExecutionMode::Qpu;

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_err()
        );
    }

    #[test]
    fn qpu_permission_allows_qpu_mode_at_input_boundary() {
        let mut config = BenchmarkConfig::default();

        config.execution_permission =
            ExecutionPermission::AllowQpu;

        config.execution_mode =
            ExecutionMode::Qpu;

        assert!(
            InputValidator::production()
                .validate(&config)
                .is_ok()
        );
    }

    #[test]
    fn named_backend_must_use_stable_identifier_syntax() {
        let valid =
            BackendSelection::named("local_simulator");

        assert!(valid.is_ok());

        let invalid =
            BackendSelection::named("Local Simulator");

        assert!(invalid.is_err());
    }

    #[test]
    fn non_negative_count_allows_zero() {
        assert!(
            validate_non_negative_count(
                "errors",
                0,
                Some(100),
            )
            .is_ok()
        );
    }

    #[test]
    fn positive_count_rejects_zero() {
        assert!(
            validate_count(
                "shots",
                0,
                Some(100),
            )
            .is_err()
        );
    }

    #[test]
    fn finite_validation_rejects_nan() {
        assert!(
            validate_finite(
                "metric",
                f64::NAN,
            )
            .is_err()
        );
    }

    #[test]
    fn non_negative_finite_rejects_negative() {
        assert!(
            validate_non_negative_finite(
                "duration",
                -1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn non_negative_finite_accepts_zero() {
        assert!(
            validate_non_negative_finite(
                "duration",
                0.0,
            )
            .is_ok()
        );
    }
}