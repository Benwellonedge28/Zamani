//! Production resource limits for Zamani quantum benchmarking.
//!
//! This module is the resource-safety boundary for the benchmarking
//! subsystem. It does not execute circuits, allocate benchmark workloads, or
//! perform statistical analysis. Instead, it defines explicit upper bounds
//! that every benchmark configuration, generator, executor, sampler,
//! statistics implementation, and reporter can validate against.
//!
//! # Architectural role
//!
//! ```text
//! Benchmark configuration
//!          │
//!          ▼
//!   benchmarking::core::limits
//!          │
//!   ┌──────┼──────────┬───────────┐
//!   ▼      ▼          ▼           ▼
//! generators execution statistics reporting
//! ```
//!
//! The limits type deliberately has no dependency on protocol modules. This
//! keeps it usable by Quantum Volume, randomized benchmarking, XEB, QEC,
//! application benchmarks, simulators, hardware executors, and future
//! benchmark families without creating dependency cycles.
//!
//! # Integration contract
//!
//! Later benchmarking modules should consume [`BenchmarkLimits`] rather than
//! defining protocol-local safety ceilings. In particular:
//!
//! - `core/config.rs` owns user-facing benchmark configuration and validates
//!   requested values against these limits.
//! - `generators/*` must call the corresponding `check_*` method before
//!   creating potentially large workloads.
//! - `execution/*` must enforce shot, circuit, parallelism, result-size, and
//!   timeout limits before submission.
//! - `statistics/*` must enforce sample, bootstrap, and iteration limits.
//! - `reporting/*` must enforce report-size limits before materializing output.
//! - `core/errors.rs` may wrap [`LimitError`] into the subsystem's canonical
//!   error hierarchy without changing this file.
//!
//! The methods in this file are intentionally deterministic and side-effect
//! free. They do not perform allocations, logging, I/O, sleeping, or global
//! state mutation.
//!
//! # Security model
//!
//! Benchmark inputs can originate from Zamani source code, configuration
//! files, CI jobs, external benchmark definitions, or remote hardware
//! services. Limits therefore protect against accidental and adversarial
//! resource exhaustion. A limit is a safety boundary, not an estimate of what
//! a particular backend can physically support.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021. No nightly features are required.

use std::fmt;
use std::time::Duration;

/// Hard upper bound on a single duration represented by the limits API.
///
/// Keeping this value finite prevents configuration arithmetic such as
/// `timeout_ms * retry_count` from overflowing an unbounded integer domain.
pub const MAX_DURATION_MS: u64 = 86_400_000;

/// Default maximum number of logical/physical qubits in one benchmark
/// workload.
pub const DEFAULT_MAX_QUBITS: usize = 4_096;

/// Default maximum circuit depth.
pub const DEFAULT_MAX_CIRCUIT_DEPTH: usize = 1_000_000;

/// Default maximum number of two-qubit gates in one circuit.
pub const DEFAULT_MAX_TWO_QUBIT_GATES: usize = 10_000_000;

/// Default maximum number of total gates in one circuit.
pub const DEFAULT_MAX_GATE_COUNT: usize = 100_000_000;

/// Default maximum number of measurement shots for one circuit.
pub const DEFAULT_MAX_SHOTS: u64 = 10_000_000;

/// Default maximum number of circuits in one benchmark experiment.
pub const DEFAULT_MAX_CIRCUITS: u64 = 1_000_000;

/// Default maximum number of logical benchmark experiments in one run.
pub const DEFAULT_MAX_EXPERIMENTS: u64 = 100_000;

/// Default maximum number of independently seeded randomized workloads.
pub const DEFAULT_MAX_RANDOM_SEEDS: u64 = 1_000_000;

/// Default maximum number of observations retained by one benchmark result.
pub const DEFAULT_MAX_OBSERVATIONS: u64 = 100_000_000;

/// Default maximum serialized/raw result size in bytes.
pub const DEFAULT_MAX_RESULT_BYTES: u64 = 4 * 1024 * 1024 * 1024;

/// Default maximum size of one in-memory observation payload.
pub const DEFAULT_MAX_OBSERVATION_BYTES: u64 = 2 * 1024 * 1024 * 1024;

/// Default maximum report size in bytes.
pub const DEFAULT_MAX_REPORT_BYTES: u64 = 512 * 1024 * 1024;

/// Default maximum bootstrap/resampling iterations.
pub const DEFAULT_MAX_BOOTSTRAP_SAMPLES: u64 = 10_000_000;

/// Default maximum statistical fitting/optimization iterations.
pub const DEFAULT_MAX_STATISTICAL_ITERATIONS: u64 = 1_000_000;

/// Default maximum concurrent benchmark tasks.
pub const DEFAULT_MAX_PARALLELISM: usize = 256;

/// Default execution timeout in milliseconds.
pub const DEFAULT_TIMEOUT_MS: u64 = 3_600_000;

/// Default maximum retry attempts for one execution request.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default maximum number of extensible dimensions in a result.
pub const DEFAULT_MAX_DIMENSIONS: usize = 4_096;

/// Default maximum number of metrics in one result.
pub const DEFAULT_MAX_METRICS: usize = 16_384;

/// Default maximum warnings/errors retained in one result.
pub const DEFAULT_MAX_DIAGNOSTICS: usize = 4_096;

/// Error returned when a requested benchmark resource exceeds a configured
/// production limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitError {
    /// A resource was requested with a zero value where positive work is
    /// required.
    ZeroValue {
        /// Name of the resource.
        resource: &'static str,
    },

    /// A requested value exceeds the configured maximum.
    Exceeded {
        /// Name of the resource.
        resource: &'static str,
        /// Requested value.
        requested: u64,
        /// Configured maximum.
        maximum: u64,
    },

    /// A resource calculation overflowed.
    ArithmeticOverflow {
        /// Name of the calculated resource.
        resource: &'static str,
    },

    /// A timeout is outside the supported finite range.
    InvalidTimeout {
        /// Requested timeout in milliseconds.
        milliseconds: u64,
    },
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroValue { resource } => {
                write!(
                    f,
                    "benchmark limit '{resource}' requires a non-zero value"
                )
            }

            Self::Exceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "benchmark resource '{resource}' exceeds limit: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "benchmark resource calculation overflowed for '{resource}'"
                )
            }

            Self::InvalidTimeout { milliseconds } => {
                write!(
                    f,
                    "benchmark timeout must be greater than zero and at most \
                     {MAX_DURATION_MS} ms; got {milliseconds} ms"
                )
            }
        }
    }
}

impl std::error::Error for LimitError {}

/// Complete resource-safety policy for one benchmark run.
///
/// All fields are explicit so the policy can later be incorporated into
/// `core::config`, `core::provenance`, and serialized benchmark manifests
/// without hidden process-global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkLimits {
    /// Maximum qubits accepted by one circuit.
    pub max_qubits: usize,

    /// Maximum circuit depth.
    pub max_circuit_depth: usize,

    /// Maximum total gate count in one circuit.
    pub max_gate_count: usize,

    /// Maximum two-qubit gate count in one circuit.
    pub max_two_qubit_gates: usize,

    /// Maximum shots for one circuit.
    pub max_shots: u64,

    /// Maximum circuits in one experiment.
    pub max_circuits: u64,

    /// Maximum benchmark experiments in one run.
    pub max_experiments: u64,

    /// Maximum independently seeded randomized workloads.
    pub max_random_seeds: u64,

    /// Maximum retained observations.
    pub max_observations: u64,

    /// Maximum serialized/raw result size.
    pub max_result_bytes: u64,

    /// Maximum individual in-memory observation payload.
    pub max_observation_bytes: u64,

    /// Maximum generated report size.
    pub max_report_bytes: u64,

    /// Maximum bootstrap/resampling iterations.
    pub max_bootstrap_samples: u64,

    /// Maximum statistical fitting/optimization iterations.
    pub max_statistical_iterations: u64,

    /// Maximum concurrent benchmark tasks.
    pub max_parallelism: usize,

    /// Maximum execution timeout in milliseconds.
    pub max_timeout_ms: u64,

    /// Maximum retry attempts for one execution request.
    pub max_retries: u32,

    /// Maximum extensible dimensions in one result.
    pub max_dimensions: usize,

    /// Maximum metrics in one result.
    pub max_metrics: usize,

    /// Maximum retained diagnostics.
    pub max_diagnostics: usize,
}

impl Default for BenchmarkLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl BenchmarkLimits {
    /// Returns the deterministic production policy.
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_circuit_depth: DEFAULT_MAX_CIRCUIT_DEPTH,
            max_gate_count: DEFAULT_MAX_GATE_COUNT,
            max_two_qubit_gates: DEFAULT_MAX_TWO_QUBIT_GATES,
            max_shots: DEFAULT_MAX_SHOTS,
            max_circuits: DEFAULT_MAX_CIRCUITS,
            max_experiments: DEFAULT_MAX_EXPERIMENTS,
            max_random_seeds: DEFAULT_MAX_RANDOM_SEEDS,
            max_observations: DEFAULT_MAX_OBSERVATIONS,
            max_result_bytes: DEFAULT_MAX_RESULT_BYTES,
            max_observation_bytes: DEFAULT_MAX_OBSERVATION_BYTES,
            max_report_bytes: DEFAULT_MAX_REPORT_BYTES,
            max_bootstrap_samples: DEFAULT_MAX_BOOTSTRAP_SAMPLES,
            max_statistical_iterations: DEFAULT_MAX_STATISTICAL_ITERATIONS,
            max_parallelism: DEFAULT_MAX_PARALLELISM,
            max_timeout_ms: DEFAULT_TIMEOUT_MS,
            max_retries: DEFAULT_MAX_RETRIES,
            max_dimensions: DEFAULT_MAX_DIMENSIONS,
            max_metrics: DEFAULT_MAX_METRICS,
            max_diagnostics: DEFAULT_MAX_DIAGNOSTICS,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> Result<(), LimitError> {
        self.require_positive(self.max_qubits, "max_qubits")?;
        self.require_positive(self.max_circuit_depth, "max_circuit_depth")?;
        self.require_positive(self.max_gate_count, "max_gate_count")?;
        self.require_positive(
            self.max_two_qubit_gates,
            "max_two_qubit_gates",
        )?;
        self.require_positive(self.max_shots, "max_shots")?;
        self.require_positive(self.max_circuits, "max_circuits")?;
        self.require_positive(self.max_experiments, "max_experiments")?;
        self.require_positive(self.max_random_seeds, "max_random_seeds")?;
        self.require_positive(self.max_observations, "max_observations")?;
        self.require_positive(self.max_result_bytes, "max_result_bytes")?;
        self.require_positive(
            self.max_observation_bytes,
            "max_observation_bytes",
        )?;
        self.require_positive(self.max_report_bytes, "max_report_bytes")?;
        self.require_positive(
            self.max_bootstrap_samples,
            "max_bootstrap_samples",
        )?;
        self.require_positive(
            self.max_statistical_iterations,
            "max_statistical_iterations",
        )?;
        self.require_positive(self.max_parallelism, "max_parallelism")?;
        self.require_positive(self.max_timeout_ms, "max_timeout_ms")?;
        self.require_positive(self.max_dimensions, "max_dimensions")?;
        self.require_positive(self.max_metrics, "max_metrics")?;
        self.require_positive(self.max_diagnostics, "max_diagnostics")?;

        if self.max_timeout_ms > MAX_DURATION_MS {
            return Err(LimitError::InvalidTimeout {
                milliseconds: self.max_timeout_ms,
            });
        }

        Ok(())
    }

    /// Returns the maximum execution timeout as a `Duration`.
    pub fn timeout(&self) -> Result<Duration, LimitError> {
        self.check_timeout_ms(self.max_timeout_ms)?;
        Ok(Duration::from_millis(self.max_timeout_ms))
    }

    /// Validates a requested qubit count.
    pub fn check_qubits(&self, requested: usize) -> Result<(), LimitError> {
        self.check_usize(requested, self.max_qubits, "qubits")
    }

    /// Validates a requested circuit depth.
    pub fn check_circuit_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_circuit_depth,
            "circuit_depth",
        )
    }

    /// Validates a requested total gate count.
    pub fn check_gate_count(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(requested, self.max_gate_count, "gate_count")
    }

    /// Validates a requested two-qubit gate count.
    pub fn check_two_qubit_gates(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_two_qubit_gates,
            "two_qubit_gates",
        )
    }

    /// Validates a requested shot count.
    pub fn check_shots(&self, requested: u64) -> Result<(), LimitError> {
        self.check_u64(requested, self.max_shots, "shots")
    }

    /// Validates a requested circuit count.
    pub fn check_circuits(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(requested, self.max_circuits, "circuits")
    }

    /// Validates a requested experiment count.
    pub fn check_experiments(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_experiments,
            "experiments",
        )
    }

    /// Validates a requested random-seed count.
    pub fn check_random_seeds(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_random_seeds,
            "random_seeds",
        )
    }

    /// Validates a requested observation count.
    pub fn check_observations(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_observations,
            "observations",
        )
    }

    /// Validates a requested serialized/raw result size.
    pub fn check_result_bytes(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_result_bytes,
            "result_bytes",
        )
    }

    /// Validates a requested observation payload size.
    pub fn check_observation_bytes(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_observation_bytes,
            "observation_bytes",
        )
    }

    /// Validates a requested report size.
    pub fn check_report_bytes(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_report_bytes,
            "report_bytes",
        )
    }

    /// Validates bootstrap/resampling iterations.
    pub fn check_bootstrap_samples(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_bootstrap_samples,
            "bootstrap_samples",
        )
    }

    /// Validates statistical fitting/optimization iterations.
    pub fn check_statistical_iterations(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            requested,
            self.max_statistical_iterations,
            "statistical_iterations",
        )
    }

    /// Validates requested concurrency.
    pub fn check_parallelism(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_parallelism,
            "parallelism",
        )
    }

    /// Validates an execution timeout in milliseconds.
    pub fn check_timeout_ms(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        if requested == 0 {
            return Err(LimitError::ZeroValue {
                resource: "timeout_ms",
            });
        }

        if requested > self.max_timeout_ms {
            return Err(LimitError::Exceeded {
                resource: "timeout_ms",
                requested,
                maximum: self.max_timeout_ms,
            });
        }

        if requested > MAX_DURATION_MS {
            return Err(LimitError::InvalidTimeout {
                milliseconds: requested,
            });
        }

        Ok(())
    }

    /// Validates retry attempts.
    pub fn check_retries(&self, requested: u32) -> Result<(), LimitError> {
        if requested > self.max_retries {
            return Err(LimitError::Exceeded {
                resource: "retries",
                requested: requested as u64,
                maximum: self.max_retries as u64,
            });
        }

        Ok(())
    }

    /// Validates the number of result dimensions.
    pub fn check_dimensions(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_dimensions,
            "dimensions",
        )
    }

    /// Validates the number of metrics.
    pub fn check_metrics(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_metrics,
            "metrics",
        )
    }

    /// Validates the number of retained diagnostics.
    pub fn check_diagnostics(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            requested,
            self.max_diagnostics,
            "diagnostics",
        )
    }

    /// Performs an overflow-safe multiplication.
    pub fn checked_product(
        &self,
        resource: &'static str,
        left: u64,
        right: u64,
    ) -> Result<u64, LimitError> {
        left.checked_mul(right).ok_or(
            LimitError::ArithmeticOverflow { resource },
        )
    }

    /// Performs an overflow-safe multiplication and checks the result
    /// against an explicit maximum.
    pub fn check_product(
        &self,
        resource: &'static str,
        left: u64,
        right: u64,
        maximum: u64,
    ) -> Result<u64, LimitError> {
        let product = self.checked_product(resource, left, right)?;

        if product > maximum {
            return Err(LimitError::Exceeded {
                resource,
                requested: product,
                maximum,
            });
        }

        Ok(product)
    }

    /// Validates and calculates the total number of shots in an experiment.
    pub fn check_total_shots(
        &self,
        circuits: u64,
        shots_per_circuit: u64,
    ) -> Result<u64, LimitError> {
        self.check_circuits(circuits)?;
        self.check_shots(shots_per_circuit)?;

        self.checked_product(
            "total_shots",
            circuits,
            shots_per_circuit,
        )
    }

    /// Validates the basic resource envelope of a circuit.
    pub fn check_circuit(
        &self,
        qubits: usize,
        depth: usize,
        gate_count: usize,
        two_qubit_gates: usize,
    ) -> Result<(), LimitError> {
        self.check_qubits(qubits)?;
        self.check_circuit_depth(depth)?;
        self.check_gate_count(gate_count)?;
        self.check_two_qubit_gates(two_qubit_gates)?;

        if two_qubit_gates > gate_count {
            return Err(LimitError::Exceeded {
                resource: "two_qubit_gates_vs_gate_count",
                requested: two_qubit_gates as u64,
                maximum: gate_count as u64,
            });
        }

        Ok(())
    }

    /// Returns whether a requested qubit count is allowed.
    #[inline]
    pub fn allows_qubits(&self, requested: usize) -> bool {
        requested > 0 && requested <= self.max_qubits
    }

    /// Returns whether a requested shot count is allowed.
    #[inline]
    pub fn allows_shots(&self, requested: u64) -> bool {
        requested > 0 && requested <= self.max_shots
    }

    /// Returns whether a requested circuit count is allowed.
    #[inline]
    pub fn allows_circuits(&self, requested: u64) -> bool {
        requested > 0 && requested <= self.max_circuits
    }

    fn require_positive<T>(
        &self,
        value: T,
        resource: &'static str,
    ) -> Result<(), LimitError>
    where
        T: Into<u64> + Copy,
    {
        if value.into() == 0 {
            return Err(LimitError::ZeroValue { resource });
        }

        Ok(())
    }

    fn check_usize(
        &self,
        requested: usize,
        maximum: usize,
        resource: &'static str,
    ) -> Result<(), LimitError> {
        if requested == 0 {
            return Err(LimitError::ZeroValue { resource });
        }

        if requested > maximum {
            return Err(LimitError::Exceeded {
                resource,
                requested: requested as u64,
                maximum: maximum as u64,
            });
        }

        Ok(())
    }

    fn check_u64(
        &self,
        requested: u64,
        maximum: u64,
        resource: &'static str,
    ) -> Result<(), LimitError> {
        if requested == 0 {
            return Err(LimitError::ZeroValue { resource });
        }

        if requested > maximum {
            return Err(LimitError::Exceeded {
                resource,
                requested,
                maximum,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_limits_are_valid() {
        assert!(BenchmarkLimits::production().validate().is_ok());
    }

    #[test]
    fn default_equals_production() {
        assert_eq!(
            BenchmarkLimits::default(),
            BenchmarkLimits::production()
        );
    }

    #[test]
    fn zero_requests_are_rejected() {
        let limits = BenchmarkLimits::production();

        assert!(matches!(
            limits.check_qubits(0),
            Err(LimitError::ZeroValue {
                resource: "qubits"
            })
        ));

        assert!(matches!(
            limits.check_shots(0),
            Err(LimitError::ZeroValue {
                resource: "shots"
            })
        ));

        assert!(matches!(
            limits.check_timeout_ms(0),
            Err(LimitError::ZeroValue {
                resource: "timeout_ms"
            })
        ));
    }

    #[test]
    fn values_above_limits_are_rejected() {
        let limits = BenchmarkLimits::production();

        assert!(matches!(
            limits.check_qubits(DEFAULT_MAX_QUBITS + 1),
            Err(LimitError::Exceeded {
                resource: "qubits",
                ..
            })
        ));

        assert!(matches!(
            limits.check_shots(DEFAULT_MAX_SHOTS + 1),
            Err(LimitError::Exceeded {
                resource: "shots",
                ..
            })
        ));
    }

    #[test]
    fn timeout_is_bounded_and_convertible() {
        let limits = BenchmarkLimits::production();

        assert_eq!(
            limits
                .timeout()
                .expect("production timeout is valid"),
            Duration::from_millis(DEFAULT_TIMEOUT_MS)
        );

        let mut invalid = limits;
        invalid.max_timeout_ms = MAX_DURATION_MS + 1;

        assert!(matches!(
            invalid.validate(),
            Err(LimitError::InvalidTimeout { .. })
        ));
    }

    #[test]
    fn retries_are_bounded() {
        let limits = BenchmarkLimits::production();

        assert!(
            limits
                .check_retries(DEFAULT_MAX_RETRIES)
                .is_ok()
        );

        assert!(matches!(
            limits.check_retries(DEFAULT_MAX_RETRIES + 1),
            Err(LimitError::Exceeded {
                resource: "retries",
                ..
            })
        ));
    }

    #[test]
    fn checked_product_detects_overflow() {
        let limits = BenchmarkLimits::production();

        assert!(matches!(
            limits.checked_product(
                "total_shots",
                u64::MAX,
                2
            ),
            Err(LimitError::ArithmeticOverflow {
                resource: "total_shots"
            })
        ));
    }

    #[test]
    fn checked_product_enforces_maximum() {
        let limits = BenchmarkLimits::production();

        assert_eq!(
            limits
                .check_product(
                    "total_shots",
                    10,
                    20,
                    100
                )
                .unwrap(),
            100
        );

        assert!(matches!(
            limits.check_product(
                "total_shots",
                11,
                10,
                100
            ),
            Err(LimitError::Exceeded {
                resource: "total_shots",
                requested: 110,
                maximum: 100
            })
        ));
    }

    #[test]
    fn total_shots_checks_each_dimension() {
        let limits = BenchmarkLimits::production();

        assert_eq!(
            limits
                .check_total_shots(100, 1_000)
                .unwrap(),
            100_000
        );

        assert!(matches!(
            limits.check_total_shots(
                DEFAULT_MAX_CIRCUITS + 1,
                1
            ),
            Err(LimitError::Exceeded {
                resource: "circuits",
                ..
            })
        ));
    }

    #[test]
    fn circuit_envelope_rejects_inconsistent_gate_counts() {
        let limits = BenchmarkLimits::production();

        assert!(matches!(
            limits.check_circuit(
                10,
                20,
                5,
                6
            ),
            Err(LimitError::Exceeded {
                resource: "two_qubit_gates_vs_gate_count",
                requested: 6,
                maximum: 5
            })
        ));
    }

    #[test]
    fn allows_helpers_match_validation_semantics() {
        let limits = BenchmarkLimits::production();

        assert!(limits.allows_qubits(1));
        assert!(limits.allows_qubits(DEFAULT_MAX_QUBITS));
        assert!(!limits.allows_qubits(0));
        assert!(!limits.allows_qubits(
            DEFAULT_MAX_QUBITS + 1
        ));

        assert!(limits.allows_shots(1));
        assert!(!limits.allows_shots(0));
        assert!(!limits.allows_shots(
            DEFAULT_MAX_SHOTS + 1
        ));
    }
}