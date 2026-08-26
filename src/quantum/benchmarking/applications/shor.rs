//! Zamani Quantum Benchmarking — Shor Application Benchmark.
//!
//! Production application-level benchmark definition for Shor's integer
//! factorization algorithm.
//!
//! # Purpose
//!
//! This module defines the benchmarking contract for Shor's algorithm without
//! taking ownership of quantum execution or Quantum IR semantics.
//!
//! It owns:
//!
//! - Shor benchmark identity;
//! - Shor benchmark versioning;
//! - integer-factorization problem validation;
//! - deterministic benchmark-instance construction;
//! - coprimality validation;
//! - order-finding problem definition;
//! - classical reduction from factoring to order finding;
//! - continued-fraction order recovery;
//! - candidate-order validation;
//! - classical factor extraction;
//! - factor-result validation;
//! - measurement/sample analysis;
//! - success/failure classification;
//! - deterministic logical resource estimation;
//! - benchmark workload construction;
//! - benchmark metadata;
//! - application parameters;
//! - reproducibility-safe generation;
//! - bounded arithmetic;
//! - production safety checks;
//! - unit tests for the mathematical layer.
//!
//! It deliberately does NOT own:
//!
//! - quantum circuit semantics;
//! - modular-exponentiation circuit construction;
//! - QFT implementation;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - backend selection;
//! - QPU communication;
//! - simulator implementation;
//! - decoder implementation;
//! - result persistence;
//! - report serialization;
//! - universal benchmark orchestration.
//!
//! Those responsibilities belong to the surrounding Zamani quantum
//! architecture.
//!
//! # Architectural position
//!
//! ```text
//! ShorBenchmarkConfig
//!         │
//!         ▼
//! ShorBenchmarkGenerator
//!         │
//!         ▼
//! ApplicationGenerationRequest
//!         │
//!         ▼
//! ApplicationWorkload
//!         │
//!         ├──────────────► future Shor Quantum-IR generator
//!         │
//!         ▼
//! BenchmarkExperiment
//!         │
//!         ▼
//! BenchmarkExecutor
//!         │
//!         ▼
//! normalized quantum observations
//!         │
//!         ▼
//! Shor measurement analysis
//!         │
//!         ▼
//! ShorBenchmarkResult
//! ```
//!
//! # Critical architectural boundary
//!
//! Shor is not just "a circuit that factors N".
//!
//! The algorithm has two logically distinct components:
//!
//! 1. classical reduction and factor extraction;
//! 2. quantum order finding.
//!
//! The classical component is:
//!
//! ```text
//! choose a
//!   │
//!   ├── gcd(a, N) != 1 ──► non-trivial factor immediately
//!   │
//!   └── gcd(a, N) == 1
//!             │
//!             ▼
//!       quantum order finding
//!             │
//!             ▼
//!             r
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//!    r odd       r even
//!       │           │
//!       ▼           ▼
//!     retry    a^(r/2) mod N
//!                       │
//!                 ┌─────┴─────┐
//!                 ▼           ▼
//!                -1          other
//!                 │           │
//!                 ▼           ▼
//!               retry      gcd extraction
//! ```
//!
//! This module owns that classical reduction because it is part of the
//! benchmark's scientific semantics.
//!
//! The quantum order-finding circuit itself belongs to the quantum algorithm /
//! Quantum IR generation layer.
//!
//! # Benchmark modes
//!
//! The benchmark supports three conceptually different modes:
//!
//! - `Reference`:
//!   small-instance classical reference/verification;
//!
//! - `QuantumReady`:
//!   produce the complete Shor application workload and quantum-order-finding
//!   metadata, ready for a future Quantum-IR circuit generator;
//!
//! - `ResourceEstimation`:
//!   estimate logical resources without constructing an exponentially large
//!   classical state space or pretending that an implementation-specific
//!   circuit decomposition is canonical.
//!
//! This distinction is important for production benchmarking.
//!
//! A classical reference factorization is **not** reported as quantum
//! performance.
//!
//! # Scientific semantics
//!
//! For an odd composite integer `N`, choose an integer `a` satisfying:
//!
//! ```text
//! 1 < a < N
//! gcd(a, N) = 1
//! ```
//!
//! The order `r` is the least positive integer satisfying:
//!
//! ```text
//! a^r ≡ 1 (mod N)
//! ```
//!
//! If `r` is even and:
//!
//! ```text
//! a^(r/2) != -1 (mod N)
//! ```
//!
//! then:
//!
//! ```text
//! p = gcd(a^(r/2) - 1, N)
//! q = gcd(a^(r/2) + 1, N)
//! ```
//!
//! are non-trivial factors of `N`.
//!
//! The quantum component estimates the order. The benchmark therefore must
//! distinguish:
//!
//! - factorization success;
//! - order-recovery success;
//! - quantum measurement success;
//! - classical post-processing success;
//! - retry requirement.
//!
//! These are not interchangeable metrics.
//!
//! # Measurement semantics
//!
//! The order-finding phase register contains a measurement `y` interpreted as
//! an integer in:
//!
//! ```text
//! 0 <= y < 2^t
//! ```
//!
//! where `t` is the phase-estimation precision in bits.
//!
//! A candidate fraction is obtained from:
//!
//! ```text
//! y / 2^t ≈ k / r\n//! ```
//!
//! Continued fractions recover candidate denominators. Every candidate must
//! then be validated directly against modular exponentiation before being
//! accepted as an order.
//!
//! The implementation never treats a continued-fraction denominator as a
//! guaranteed order.
//!
//! # Bit ordering
//!
//! This module accepts measurement values already normalized by the execution
//! layer.
//!
//! It does not reverse bitstrings, infer backend endianness, or interpret
//! vendor-specific result formats.
//!
//! If a backend produces a bitstring, the execution/observation layer must
//! convert it into the canonical integer measurement before calling the
//! Shor analysis functions.
//!
//! # Resource accounting
//!
//! This file reports only algorithm-level logical resources.
//!
//! It does NOT claim an exact physical gate count.
//!
//! In particular, modular exponentiation can be implemented using substantially
//! different arithmetic constructions, ancilla strategies, decomposition
//! choices, and compilation strategies.
//!
//! Consequently this file exposes:
//!
//! - phase-register qubits;
//! - work-register qubits;
//! - abstract algorithmic qubits;
//! - phase-estimation precision;
//! - modular-exponentiation repetitions;
//! - controlled modular multiplications;
//! - classical post-processing requirements;
//! - conservative structural complexity estimates.
//!
//! Exact native gate counts belong to the compiled circuit and hardware
//! benchmarking layers.
//!
//! # Reproducibility
//!
//! Identical:
//!
//! ```text
//! benchmark version
//! application ID
//! instance ID
//! N
//! a
//! precision
//! generation revision
//! ```
//!
//! produce identical semantic benchmark definitions.
//!
//! This module never uses:
//!
//! - system time;
//! - process ID;
//! - pointer addresses;
//! - thread IDs;
//! - global RNG state;
//! - network state;
//! - filesystem state.
//!
//! # Security/resource model
//!
//! Shor benchmark parameters can eventually originate from the Zamani
//! language, configuration files, CI, remote benchmark requests, or APIs.
//!
//! Therefore this module:
//!
//! - validates all integers;
//! - rejects zero and trivial inputs;
//! - rejects even values from the general Shor path;
//! - validates odd composite inputs;
//! - bounds integer bit length;
//! - bounds precision;
//! - bounds retry attempts;
//! - uses checked arithmetic where allocation/resource calculations are
//!   involved;
//! - avoids constructing `2^n`-sized classical state spaces;
//! - uses modular exponentiation rather than ordinary exponentiation;
//! - does not execute caller-provided code;
//! - does not perform I/O;
//! - does not allocate based on unchecked exponential quantities.
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
//! No unsafe code.
//! No additional dependencies.
//!
//! # Integration contract
//!
//! This module integrates with:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorCapability
//!     └── ApplicationGeneratorDescriptor
//!
//! benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     └── WorkloadId
//!
//! benchmarking::core::errors
//!     ├── BenchmarkError
//!     └── BenchmarkResult
//! ```
//!
//! The generated `ApplicationWorkload` deliberately does not require a
//! QuantumCircuit. That is supported by the canonical workload model for
//! application-level resource-estimation workloads.
//!
//! A future Shor Quantum-IR generator can attach a `CircuitWorkload` without
//! changing the Shor problem, mathematical analysis, or result semantics
//! defined here.
//!
//! # External benchmark alignment
//!
//! QED-C classifies Shor's algorithm as a Level-4 application benchmark.
//! Zamani therefore treats Shor as an application benchmark rather than as a
//! generic device-characterization protocol.
//!
//! Current QCVV literature likewise emphasizes separating characterization,
//! verification/validation, and benchmarking rather than collapsing them into
//! one scalar.
//!
//! # References
//!
//! - P. W. Shor, "Algorithms for quantum computation: discrete logarithms and
//!   factoring", FOCS 1994.
//! - M. A. Nielsen and I. L. Chuang, Quantum Computation and Quantum
//!   Information.
//! - QED-C Application-Oriented Performance Benchmarks for Quantum Computing.
//! - Current quantum characterization, verification, and validation literature.
//!
//! No network access is performed by this module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    WorkloadId,
};
use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
    ApplicationGenerationRequest,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable machine-readable benchmark identifier.
pub const SHOR_BENCHMARK_ID: &str = "shor";

/// Stable application identifier.
pub const SHOR_APPLICATION_ID: &str = "shor";

/// Human-readable benchmark name.
pub const SHOR_NAME: &str = "Shor Integer Factorization";

/// Semantic benchmark version.
pub const SHOR_BENCHMARK_VERSION: u32 = 1;

/// Stable generator revision.
pub const SHOR_GENERATOR_REVISION: u32 = 1;

/// Generator implementation version.
pub const SHOR_GENERATOR_VERSION: &str = "1.0.0";

/// Result schema version for Shor-specific result objects.
pub const SHOR_RESULT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Safety/resource limits
// =============================================================================

/// Maximum modulus bit length supported by the benchmark definition.
///
/// This is deliberately a benchmark-layer safety limit, not a theoretical
/// limitation of Shor's algorithm.
///
/// Larger instances should be admitted only after the surrounding benchmark
/// limits and resource-estimation architecture explicitly support them.
pub const MAX_SHOR_BITS: usize = 4096;

/// Maximum phase-estimation precision accepted by this module.
///
/// Precision beyond this value is rejected before `1 << precision`-style
/// calculations can become accidental resource explosions.
pub const MAX_PHASE_PRECISION_BITS: usize = 8192;

/// Maximum number of classical post-processing attempts.
pub const MAX_SHOR_ATTEMPTS: usize = 4096;

/// Maximum number of explicitly requested candidate orders to inspect.
pub const MAX_ORDER_CANDIDATES: usize = 4096;

/// Maximum encoded application parameter length used by this module.
pub const MAX_SHOR_PARAMETER_BYTES: usize = 256;

/// Minimum composite integer accepted by the general Shor benchmark.
pub const MIN_SHOR_MODULUS: u64 = 4;

/// Maximum `u64` modulus supported by the exact classical mathematical helper.
///
/// This bound is not used to claim that the quantum benchmark is restricted
/// to 64-bit factoring. It only defines the integer representation used by the
/// dependency-free reference implementation in this Rust file.
pub const MAX_REFERENCE_MODULUS: u64 = u64::MAX;

// =============================================================================
// Benchmark mode
// =============================================================================

/// Execution/benchmarking mode for Shor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShorBenchmarkMode {
    /// Small-instance classical reference/verification mode.
    Reference,

    /// Quantum-order-finding benchmark definition.
    QuantumReady,

    /// Algorithm-level logical resource estimation.
    ResourceEstimation,
}

impl ShorBenchmarkMode {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::QuantumReady => "quantum_ready",
            Self::ResourceEstimation => "resource_estimation",
        }
    }
}

impl fmt::Display for ShorBenchmarkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Success classification
// =============================================================================

/// Classification of one Shor order/factorization attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShorAttemptStatus {
    /// A non-trivial factor was obtained directly from gcd(a, N).
    DirectGcdFactor,

    /// The recovered order was invalid.
    InvalidOrder,

    /// The recovered order was odd and therefore cannot be used directly.
    OddOrder,

    /// The recovered order gives `a^(r/2) == -1 (mod N)`, requiring retry.
    TrivialSquareRoot,

    /// The recovered order produced non-trivial factors.
    Factored,

    /// The classical post-processing did not obtain factors.
    RetryRequired,
}

impl ShorAttemptStatus {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectGcdFactor => "direct_gcd_factor",
            Self::InvalidOrder => "invalid_order",
            Self::OddOrder => "odd_order",
            Self::TrivialSquareRoot => "trivial_square_root",
            Self::Factored => "factored",
            Self::RetryRequired => "retry_required",
        }
    }
}

impl fmt::Display for ShorAttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Shor problem
// =============================================================================

/// Validated Shor factorization problem.
///
/// `N` must be an odd composite integer.
///
/// The direct even-number case is intentionally represented by
/// [`ShorProblem::from_modulus`] returning a dedicated validation error. This
/// keeps the general Shor benchmark mathematically explicit instead of
/// silently treating trivial classical preprocessing as quantum performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShorProblem {
    modulus: u64,
}

impl ShorProblem {
    /// Creates a validated Shor problem.
    pub fn new(modulus: u64) -> BenchmarkResult<Self> {
        validate_modulus(modulus)?;

        Ok(Self { modulus })
    }

    /// Returns the integer to factor.
    #[must_use]
    pub const fn modulus(self) -> u64 {
        self.modulus
    }

    /// Returns the bit length of `N`.
    #[must_use]
    pub const fn bit_length(self) -> usize {
        bit_length(self.modulus)
    }

    /// Returns whether `N` is prime.
    #[must_use]
    pub fn is_prime(self) -> bool {
        is_prime(self.modulus)
    }

    /// Returns the canonical number of phase-estimation bits.
    ///
    /// The standard order-finding construction uses approximately `2n` phase
    /// bits for an `n`-bit modulus.
    pub fn recommended_phase_precision(self) -> BenchmarkResult<usize> {
        self.bit_length()
            .checked_mul(2)
            .ok_or_else(|| {
                invalid_configuration(
                    "phase_precision",
                    "recommended phase precision overflowed",
                )
            })
    }

    /// Returns whether a proposed base is valid for the quantum order-finding
    /// portion.
    pub fn validate_base(self, base: u64) -> BenchmarkResult<()> {
        if base <= 1 {
            return Err(invalid_configuration(
                "base",
                "Shor base must be greater than one",
            ));
        }

        if base >= self.modulus {
            return Err(invalid_configuration(
                "base",
                "Shor base must be smaller than the modulus",
            ));
        }

        if gcd(base, self.modulus) != 1 {
            return Err(invalid_configuration(
                "base",
                "Shor order finding requires a coprime base",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Shor benchmark configuration
// =============================================================================

/// Production Shor benchmark configuration.
///
/// This contains Shor-specific semantic parameters. Generic benchmark
/// execution parameters such as backend, shots, timeout, routing, scheduling,
/// confidence policy, and compiler options belong to `core::config`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorBenchmarkConfig {
    /// Integer to factor.
    pub modulus: u64,

    /// Base used for order finding.
    pub base: u64,

    /// Number of phase-estimation bits.
    pub phase_precision: usize,

    /// Maximum classical retry attempts.
    pub max_attempts: usize,

    /// Benchmark execution mode.
    pub mode: ShorBenchmarkMode,

    /// Whether the classical reference path may be used to verify small
    /// instances.
    pub reference_verification: bool,
}

impl ShorBenchmarkConfig {
    /// Creates a validated Shor configuration.
    pub fn new(
        modulus: u64,
        base: u64,
    ) -> BenchmarkResult<Self> {
        let problem = ShorProblem::new(modulus)?;

        problem.validate_base(base)?;

        let phase_precision =
            problem.recommended_phase_precision()?;

        let config = Self {
            modulus,
            base,
            phase_precision,
            max_attempts: 64,
            mode: ShorBenchmarkMode::QuantumReady,
            reference_verification: true,
        };

        config.validate()?;

        Ok(config)
    }

    /// Sets phase-estimation precision.
    pub fn with_phase_precision(
        mut self,
        phase_precision: usize,
    ) -> BenchmarkResult<Self> {
        self.phase_precision = phase_precision;
        self.validate()?;
        Ok(self)
    }

    /// Sets the retry limit.
    pub fn with_max_attempts(
        mut self,
        max_attempts: usize,
    ) -> BenchmarkResult<Self> {
        self.max_attempts = max_attempts;
        self.validate()?;
        Ok(self)
    }

    /// Sets the benchmark mode.
    pub fn with_mode(
        mut self,
        mode: ShorBenchmarkMode,
    ) -> BenchmarkResult<Self> {
        self.mode = mode;
        self.validate()?;
        Ok(self)
    }

    /// Enables or disables the classical reference check.
    pub fn with_reference_verification(
        mut self,
        enabled: bool,
    ) -> Self {
        self.reference_verification = enabled;
        self
    }

    /// Validates the complete Shor configuration.
    pub fn validate(&self) -> BenchmarkResult<()> {
        let problem = ShorProblem::new(self.modulus)?;

        problem.validate_base(self.base)?;

        if self.phase_precision == 0 {
            return Err(invalid_configuration(
                "phase_precision",
                "phase precision must be greater than zero",
            ));
        }

        if self.phase_precision > MAX_PHASE_PRECISION_BITS {
            return Err(invalid_configuration(
                "phase_precision",
                "phase precision exceeds the Shor benchmark safety limit",
            ));
        }

        if self.max_attempts == 0 {
            return Err(invalid_configuration(
                "max_attempts",
                "maximum attempts must be greater than zero",
            ));
        }

        if self.max_attempts > MAX_SHOR_ATTEMPTS {
            return Err(invalid_configuration(
                "max_attempts",
                "maximum attempts exceeds the Shor benchmark safety limit",
            ));
        }

        if problem.bit_length() > MAX_SHOR_BITS {
            return Err(invalid_configuration(
                "modulus",
                "modulus bit length exceeds the Shor benchmark safety limit",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Logical resource estimation
// =============================================================================

/// Algorithm-level logical resource estimate for Shor.
///
/// These are deliberately not physical gate counts.
///
/// Different modular-arithmetic implementations can have radically different
/// ancilla requirements and gate decompositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShorResourceEstimate {
    /// Number of bits in the modulus.
    pub modulus_bits: usize,

    /// Number of phase-estimation qubits.
    pub phase_qubits: usize,

    /// Number of work-register qubits at the abstract arithmetic level.
    pub work_qubits: usize,

    /// Minimum abstract algorithmic qubits represented by these two registers.
    pub algorithmic_qubits: usize,

    /// Number of controlled modular-exponentiation steps.
    pub controlled_modular_multiplications: usize,

    /// Number of inverse-QFT output qubits.
    pub inverse_qft_qubits: usize,

    /// Classical post-processing input width.
    pub classical_measurement_bits: usize,
}

impl ShorResourceEstimate {
    /// Creates a resource estimate from a validated configuration.
    pub fn from_config(
        config: &ShorBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        let n = bit_length(config.modulus);

        let phase_qubits = config.phase_precision;

        let work_qubits = n;

        let algorithmic_qubits = phase_qubits
            .checked_add(work_qubits)
            .ok_or_else(|| {
                invalid_configuration(
                    "algorithmic_qubits",
                    "logical qubit count overflowed",
                )
            })?;

        let controlled_modular_multiplications =
            phase_qubits;

        Ok(Self {
            modulus_bits: n,
            phase_qubits,
            work_qubits,
            algorithmic_qubits,
            controlled_modular_multiplications,
            inverse_qft_qubits: phase_qubits,
            classical_measurement_bits: phase_qubits,
        })
    }
}

// =============================================================================
// Order result
// =============================================================================

/// Result of validating/recovering one multiplicative order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShorOrderResult {
    /// Measured phase numerator.
    pub measurement: u64,

    /// Phase-register precision.
    pub precision_bits: usize,

    /// Candidate numerator recovered from the continued fraction.
    pub numerator: u64,

    /// Candidate order.
    pub order: u64,

    /// Whether the candidate is actually a multiplicative order.
    pub valid: bool,
}

impl ShorOrderResult {
    /// Returns whether the order is usable by the factor-extraction stage.
    #[must_use]
    pub const fn usable(self) -> bool {
        self.valid && self.order > 0 && self.order % 2 == 0
    }
}

// =============================================================================
// Factor result
// =============================================================================

/// Validated factorization result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShorFactorResult {
    /// Original modulus.
    pub modulus: u64,

    /// First non-trivial factor.
    pub factor_a: u64,

    /// Second non-trivial factor.
    pub factor_b: u64,
}

impl ShorFactorResult {
    /// Creates a validated factorization result.
    pub fn new(
        modulus: u64,
        factor_a: u64,
        factor_b: u64,
    ) -> BenchmarkResult<Self> {
        if factor_a <= 1 || factor_b <= 1 {
            return Err(invalid_configuration(
                "factor",
                "Shor factors must be non-trivial",
            ));
        }

        if factor_a >= modulus || factor_b >= modulus {
            return Err(invalid_configuration(
                "factor",
                "Shor factors must be smaller than the modulus",
            ));
        }

        if factor_a
            .checked_mul(factor_b)
            != Some(modulus)
        {
            return Err(invalid_configuration(
                "factor",
                "factorization does not multiply back to the modulus",
            ));
        }

        Ok(Self {
            modulus,
            factor_a,
            factor_b,
        })
    }

    /// Returns the factors in canonical ascending order.
    #[must_use]
    pub fn ordered(self) -> (u64, u64) {
        if self.factor_a <= self.factor_b {
            (self.factor_a, self.factor_b)
        } else {
            (self.factor_b, self.factor_a)
        }
    }
}

// =============================================================================
// Attempt analysis
// =============================================================================

/// Detailed analysis of one Shor classical post-processing attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShorAttemptResult {
    /// Attempt status.
    pub status: ShorAttemptStatus,

    /// Original modulus.
    pub modulus: u64,

    /// Chosen base.
    pub base: u64,

    /// Candidate order, if one was available.
    pub order: Option<u64>,

    /// First factor, if found.
    pub factor_a: Option<u64>,

    /// Second factor, if found.
    pub factor_b: Option<u64>,
}

impl ShorAttemptResult {
    fn direct_factor(
        modulus: u64,
        base: u64,
        factor: u64,
    ) -> BenchmarkResult<Self> {
        let pair = ShorFactorResult::new(
            modulus,
            factor,
            modulus / factor,
        )?;

        Ok(Self {
            status: ShorAttemptStatus::DirectGcdFactor,
            modulus,
            base,
            order: None,
            factor_a: Some(pair.factor_a),
            factor_b: Some(pair.factor_b),
        })
    }

    fn retry(
        status: ShorAttemptStatus,
        modulus: u64,
        base: u64,
        order: Option<u64>,
    ) -> Self {
        Self {
            status,
            modulus,
            base,
            order,
            factor_a: None,
            factor_b: None,
        }
    }

    fn factored(
        modulus: u64,
        base: u64,
        order: u64,
        factor_a: u64,
        factor_b: u64,
    ) -> BenchmarkResult<Self> {
        let factors =
            ShorFactorResult::new(
                modulus,
                factor_a,
                factor_b,
            )?;

        Ok(Self {
            status: ShorAttemptStatus::Factored,
            modulus,
            base,
            order: Some(order),
            factor_a: Some(factors.factor_a),
            factor_b: Some(factors.factor_b),
        })
    }

    /// Returns whether this attempt produced a factorization.
    #[must_use]
    pub const fn succeeded(self) -> bool {
        matches!(
            self.status,
            ShorAttemptStatus::DirectGcdFactor
                | ShorAttemptStatus::Factored
        )
    }

    /// Returns validated factors when successful.
    #[must_use]
    pub fn factors(self) -> Option<(u64, u64)> {
        match (self.factor_a, self.factor_b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

// =============================================================================
// Benchmark result
// =============================================================================

/// Shor-specific benchmark result.
///
/// This result is intentionally independent from the universal
/// `core::result::BenchmarkResult`. The analysis layer can later map these
/// fields into universal metrics without changing the Shor mathematical
/// contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorBenchmarkResult {
    /// Result schema version.
    pub schema_version: u16,

    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Original modulus.
    pub modulus: u64,

    /// Modulus bit length.
    pub modulus_bits: usize,

    /// Chosen base.
    pub base: u64,

    /// Phase-estimation precision.
    pub phase_precision: usize,

    /// Number of analyzed measurements.
    pub samples: usize,

    /// Number of measurements producing a valid order.
    pub valid_order_samples: usize,

    /// Number of measurements producing an even usable order.
    pub usable_order_samples: usize,

    /// Number of successful factorization attempts.
    pub successful_factorizations: usize,

    /// Whether the benchmark produced a validated factorization.
    pub success: bool,

    /// Recovered factors when successful.
    pub factors: Option<(u64, u64)>,

    /// Estimated order success probability among analyzed measurements.
    ///
    /// This is an empirical measurement-quality quantity, not the theoretical
    /// probability of Shor's algorithm.
    pub observed_usable_order_probability: f64,

    /// Estimated factorization success probability among analyzed measurements.
    pub observed_factorization_probability: f64,
}

impl ShorBenchmarkResult {
    /// Creates an analyzed benchmark result.
    pub fn new(
        modulus: u64,
        base: u64,
        phase_precision: usize,
        samples: usize,
        valid_order_samples: usize,
        usable_order_samples: usize,
        successful_factorizations: usize,
        factors: Option<(u64, u64)>,
    ) -> BenchmarkResult<Self> {
        if samples == 0 {
            return Err(invalid_configuration(
                "samples",
                "Shor analysis requires at least one measurement",
            ));
        }

        if valid_order_samples > samples {
            return Err(invalid_configuration(
                "valid_order_samples",
                "valid order samples cannot exceed total samples",
            ));
        }

        if usable_order_samples > valid_order_samples {
            return Err(invalid_configuration(
                "usable_order_samples",
                "usable order samples cannot exceed valid order samples",
            ));
        }

        if successful_factorizations > samples {
            return Err(invalid_configuration(
                "successful_factorizations",
                "successful factorizations cannot exceed total samples",
            ));
        }

        if let Some((a, b)) = factors {
            ShorFactorResult::new(
                modulus,
                a,
                b,
            )?;
        }

        let observed_usable_order_probability =
            usable_order_samples as f64 / samples as f64;

        let observed_factorization_probability =
            successful_factorizations as f64 / samples as f64;

        Ok(Self {
            schema_version: SHOR_RESULT_SCHEMA_VERSION,
            benchmark_id: SHOR_BENCHMARK_ID,
            modulus,
            modulus_bits: bit_length(modulus),
            base,
            phase_precision,
            samples,
            valid_order_samples,
            usable_order_samples,
            successful_factorizations,
            success: factors.is_some(),
            factors,
            observed_usable_order_probability,
            observed_factorization_probability,
        })
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Shor application benchmark generator.
#[derive(Debug, Clone)]
pub struct ShorBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl ShorBenchmarkGenerator {
    /// Creates the standard Shor benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor =
            ApplicationGeneratorDescriptor::new(
                SHOR_BENCHMARK_ID,
                SHOR_APPLICATION_ID,
                SHOR_GENERATOR_VERSION,
                "Production Shor integer-factorization application benchmark",
            )?
            .with_capabilities([
                ApplicationGeneratorCapability::Deterministic,
                ApplicationGeneratorCapability::Parameterized,
                ApplicationGeneratorCapability::ScalableProblemSize,
                ApplicationGeneratorCapability::ExactSmallInstanceReference,
                ApplicationGeneratorCapability::ClassicallyVerifiable,
                ApplicationGeneratorCapability::ResourceEstimation,
                ApplicationGeneratorCapability::HardwareExecutable,
            ]);

        Ok(Self { descriptor })
    }

    /// Creates a generator with the standard descriptor.
    #[must_use]
    pub fn standard() -> Self {
        Self::new().expect(
            "the built-in Shor benchmark descriptor is statically valid",
        )
    }

    /// Returns the benchmark descriptor.
    #[must_use]
    pub fn descriptor_ref(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Creates a workload directly from a Shor configuration.
    pub fn workload_from_config(
        &self,
        config: &ShorBenchmarkConfig,
    ) -> BenchmarkResult<ApplicationWorkload> {
        config.validate()?;

        let instance_id =
            WorkloadId::new(format!(
                "{}_n{}",
                SHOR_APPLICATION_ID,
                config.modulus
            ))
            .map_err(BenchmarkError::from)?;

        let mut workload =
            ApplicationWorkload::new(
                SHOR_APPLICATION_ID,
                instance_id,
                config.modulus as usize,
            )
            .map_err(BenchmarkError::from)?;

        let parameters = [
            ApplicationParameter::new(
                "modulus",
                config.modulus.to_string(),
            ),
            ApplicationParameter::new(
                "base",
                config.base.to_string(),
            ),
            ApplicationParameter::new(
                "phase_precision",
                config.phase_precision.to_string(),
            ),
            ApplicationParameter::new(
                "max_attempts",
                config.max_attempts.to_string(),
            ),
            ApplicationParameter::new(
                "mode",
                config.mode.as_str(),
            ),
            ApplicationParameter::new(
                "reference_verification",
                config.reference_verification.to_string(),
            ),
        ];

        for parameter in parameters {
            let parameter =
                parameter.map_err(BenchmarkError::from)?;

            workload
                .add_parameter(parameter)
                .map_err(BenchmarkError::from)?;
        }

        Ok(workload)
    }

    /// Builds a workload directly from a generation request.
    pub fn workload_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.generate_workload(request)
    }
}

impl Default for ShorBenchmarkGenerator {
    fn default() -> Self {
        Self::standard()
    }
}

impl ApplicationBenchmarkGenerator for ShorBenchmarkGenerator {
    fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        ApplicationBenchmarkGenerator::validate(
            self,
            request,
        )?;

        if request.application_id()
            != SHOR_APPLICATION_ID
        {
            return Err(invalid_configuration(
                "application_id",
                "application generation request is not for the Shor benchmark",
            ));
        }

        if request.problem_size()
            > MAX_REFERENCE_MODULUS as usize
        {
            return Err(invalid_configuration(
                "problem_size",
                "problem size exceeds the exact reference integer representation",
            ));
        }

        let modulus =
            request.problem_size() as u64;

        ShorProblem::new(modulus)?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let modulus =
            request.problem_size() as u64;

        /*
         * The generic application-generation contract intentionally does not
         * impose a protocol-specific parameter encoding.
         *
         * For Shor, a request whose problem size is N receives the canonical
         * deterministic instance:
         *
         *   N = request.problem_size()
         *
         * and the smallest valid coprime base greater than one.
         *
         * This is deterministic and does not consume hidden RNG state.
         *
         * A Zamani-language benchmark can explicitly select another base by
         * constructing a ShorBenchmarkConfig and using workload_from_config().
         */
        let base =
            smallest_coprime_base(modulus)?;

        let config =
            ShorBenchmarkConfig::new(
                modulus,
                base,
            )?;

        self.workload_from_config(&config)
    }
}

// =============================================================================
// Classical mathematics
// =============================================================================

/// Computes the greatest common divisor.
#[must_use]
pub const fn gcd(
    mut a: u64,
    mut b: u64,
) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

/// Computes `(base^exponent) mod modulus` using binary exponentiation.
///
/// This function never constructs `base^exponent` directly.
pub fn modular_pow(
    mut base: u64,
    mut exponent: u64,
    modulus: u64,
) -> BenchmarkResult<u64> {
    if modulus == 0 {
        return Err(invalid_configuration(
            "modulus",
            "modulus must be non-zero",
        ));
    }

    if modulus == 1 {
        return Ok(0);
    }

    base %= modulus;

    let mut result = 1u64 % modulus;

    while exponent != 0 {
        if exponent & 1 == 1 {
            result =
                mul_mod(result, base, modulus)?;
        }

        exponent >>= 1;

        if exponent != 0 {
            base =
                mul_mod(base, base, modulus)?;
        }
    }

    Ok(result)
}

/// Overflow-safe modular multiplication.
///
/// This uses repeated doubling rather than `a * b`, making it safe for the
/// full `u64` reference domain without requiring a wider integer dependency.
pub fn mul_mod(
    mut a: u64,
    mut b: u64,
    modulus: u64,
) -> BenchmarkResult<u64> {
    if modulus == 0 {
        return Err(invalid_configuration(
            "modulus",
            "modulus must be non-zero",
        ));
    }

    a %= modulus;
    b %= modulus;

    let mut result = 0u64;

    while b != 0 {
        if b & 1 == 1 {
            result = add_mod(
                result,
                a,
                modulus,
            )?;
        }

        b >>= 1;

        if b != 0 {
            a = add_mod(
                a,
                a,
                modulus,
            )?;
        }
    }

    Ok(result)
}

/// Overflow-safe modular addition.
pub fn add_mod(
    a: u64,
    b: u64,
    modulus: u64,
) -> BenchmarkResult<u64> {
    if modulus == 0 {
        return Err(invalid_configuration(
            "modulus",
            "modulus must be non-zero",
        ));
    }

    let a = a % modulus;
    let b = b % modulus;

    if a >= modulus - b {
        Ok(a - (modulus - b))
    } else {
        Ok(a + b)
    }
}

/// Computes the multiplicative order of `base` modulo `modulus`.
///
/// This is a classical reference implementation only. It is intentionally
/// bounded and must never be interpreted as the quantum order-finding
/// implementation.
pub fn multiplicative_order(
    base: u64,
    modulus: u64,
) -> BenchmarkResult<Option<u64>> {
    if modulus < 2 {
        return Err(invalid_configuration(
            "modulus",
            "modulus must be at least two",
        ));
    }

    if base == 0 || base >= modulus {
        return Err(invalid_configuration(
            "base",
            "base must satisfy 1 <= base < modulus",
        ));
    }

    if gcd(base, modulus) != 1 {
        return Ok(None);
    }

    let mut value = 1u64;

    let mut order = 1u64;

    loop {
        value =
            mul_mod(value, base, modulus)?;

        if value == 1 {
            return Ok(Some(order));
        }

        if order >= modulus {
            /*
             * Euler's theorem guarantees that a coprime base has an order
             * less than modulus. Reaching this condition therefore means the
             * implementation encountered an invalid mathematical state.
             */
            return Err(invalid_configuration(
                "order",
                "multiplicative-order search exceeded its mathematical bound",
            ));
        }

        order += 1;
    }
}

/// Performs the classical factor-extraction step after a candidate order.
pub fn extract_factors(
    problem: ShorProblem,
    base: u64,
    order: u64,
) -> BenchmarkResult<ShorAttemptResult> {
    problem.validate_base(base)?;

    if order == 0 {
        return Ok(
            ShorAttemptResult::retry(
                ShorAttemptStatus::InvalidOrder,
                problem.modulus(),
                base,
                Some(order),
            ),
        );
    }

    if order % 2 != 0 {
        return Ok(
            ShorAttemptResult::retry(
                ShorAttemptStatus::OddOrder,
                problem.modulus(),
                base,
                Some(order),
            ),
        );
    }

    let half_order = order / 2;

    let square_root =
        modular_pow(
            base,
            half_order,
            problem.modulus(),
        )?;

    if square_root == 1
        || square_root
            == problem.modulus() - 1
    {
        return Ok(
            ShorAttemptResult::retry(
                ShorAttemptStatus::TrivialSquareRoot,
                problem.modulus(),
                base,
                Some(order),
            ),
        );
    }

    let minus_one_gcd =
        gcd(
            if square_root == 0 {
                problem.modulus() - 1
            } else {
                square_root - 1
            },
            problem.modulus(),
        );

    let plus_one_gcd =
        gcd(
            square_root + 1,
            problem.modulus(),
        );

    if is_nontrivial_factor(
        minus_one_gcd,
        problem.modulus(),
    ) && is_nontrivial_factor(
        plus_one_gcd,
        problem.modulus(),
    ) {
        return ShorAttemptResult::factored(
            problem.modulus(),
            base,
            order,
            minus_one_gcd,
            plus_one_gcd,
        );
    }

    Ok(
        ShorAttemptResult::retry(
            ShorAttemptStatus::RetryRequired,
            problem.modulus(),
            base,
            Some(order),
        ),
    )
}

// =============================================================================
// Continued-fraction order recovery
// =============================================================================

/// A rational number represented as a reduced numerator/denominator pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rational {
    /// Numerator.
    pub numerator: u64,

    /// Denominator.
    pub denominator: u64,
}

impl Rational {
    /// Creates a normalized rational number.
    pub fn new(
        numerator: u64,
        denominator: u64,
    ) -> BenchmarkResult<Self> {
        if denominator == 0 {
            return Err(invalid_configuration(
                "denominator",
                "rational denominator must be non-zero",
            ));
        }

        let divisor =
            gcd(numerator, denominator);

        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }
}

/// Returns continued-fraction convergents of `numerator / denominator`.
///
/// The returned vector is bounded by `MAX_ORDER_CANDIDATES`.
pub fn continued_fraction_convergents(
    numerator: u64,
    denominator: u64,
) -> BenchmarkResult<Vec<Rational>> {
    if denominator == 0 {
        return Err(invalid_configuration(
            "denominator",
            "continued fraction denominator must be non-zero",
        ));
    }

    let mut n = numerator;
    let mut d = denominator;

    let mut p_minus_two = 0u64;
    let mut p_minus_one = 1u64;

    let mut q_minus_two = 1u64;
    let mut q_minus_one = 0u64;

    let mut result =
        Vec::new();

    while d != 0
        && result.len()
            < MAX_ORDER_CANDIDATES
    {
        let quotient = n / d;
        let remainder = n % d;

        let p =
            quotient
                .checked_mul(p_minus_one)
                .and_then(|value| {
                    value.checked_add(
                        p_minus_two,
                    )
                })
                .ok_or_else(|| {
                    invalid_configuration(
                        "continued_fraction",
                        "continued-fraction numerator overflowed",
                    )
                })?;

        let q =
            quotient
                .checked_mul(q_minus_one)
                .and_then(|value| {
                    value.checked_add(
                        q_minus_two,
                    )
                })
                .ok_or_else(|| {
                    invalid_configuration(
                        "continued_fraction",
                        "continued-fraction denominator overflowed",
                    )
                })?;

        if q != 0 {
            result.push(
                Rational::new(p, q)?,
            );
        }

        p_minus_two = p_minus_one;
        p_minus_one = p;

        q_minus_two = q_minus_one;
        q_minus_one = q;

        n = d;
        d = remainder;
    }

    Ok(result)
}

/// Recovers a validated order from one normalized phase measurement.
///
/// The function tries every continued-fraction convergent denominator and
/// validates it by direct modular exponentiation.
///
/// This validation is essential: a continued-fraction denominator is a
/// *candidate*, not automatically the multiplicative order.
pub fn recover_order_from_measurement(
    problem: ShorProblem,
    base: u64,
    measurement: u64,
    precision_bits: usize,
) -> BenchmarkResult<Option<ShorOrderResult>> {
    problem.validate_base(base)?;

    if precision_bits == 0
        || precision_bits > MAX_PHASE_PRECISION_BITS
    {
        return Err(invalid_configuration(
            "precision_bits",
            "invalid phase-estimation precision",
        ));
    }

    if precision_bits >= 64 {
        return Err(invalid_configuration(
            "precision_bits",
            "the dependency-free reference measurement representation supports fewer than 64 phase bits",
        ));
    }

    let denominator =
        1u64
            .checked_shl(
                precision_bits as u32,
            )
            .ok_or_else(|| {
                invalid_configuration(
                    "precision_bits",
                    "phase denominator overflowed",
                )
            })?;

    if measurement >= denominator {
        return Err(invalid_configuration(
            "measurement",
            "phase measurement is outside the phase register range",
        ));
    }

    let convergents =
        continued_fraction_convergents(
            measurement,
            denominator,
        )?;

    for convergent in convergents {
        let candidate =
            convergent.denominator;

        if candidate == 0
            || candidate > problem.modulus()
        {
            continue;
        }

        if modular_pow(
            base,
            candidate,
            problem.modulus(),
        )? == 1
        {
            return Ok(Some(
                ShorOrderResult {
                    measurement,
                    precision_bits,
                    numerator:
                        convergent.numerator,
                    order: candidate,
                    valid: true,
                },
            ));
        }
    }

    Ok(None)
}

// =============================================================================
// Measurement analysis
// =============================================================================

/// Analyzes normalized phase measurements from a Shor experiment.
///
/// The caller must supply integer-valued phase measurements already normalized
/// by the execution layer.
///
/// Returns a Shor-specific result containing:
///
/// - valid-order sample count;
/// - usable-order sample count;
/// - successful factorization count;
/// - empirical probabilities;
/// - first validated factorization, if one was observed.
pub fn analyze_measurements<I>(
    problem: ShorProblem,
    base: u64,
    precision_bits: usize,
    measurements: I,
) -> BenchmarkResult<ShorBenchmarkResult>
where
    I: IntoIterator<Item = u64>,
{
    problem.validate_base(base)?;

    let mut samples = 0usize;
    let mut valid_orders = 0usize;
    let mut usable_orders = 0usize;
    let mut successful_factorizations = 0usize;
    let mut first_factors = None;

    for measurement in measurements {
        samples =
            samples
                .checked_add(1)
                .ok_or_else(|| {
                    invalid_configuration(
                        "samples",
                        "measurement count overflowed",
                    )
                })?;

        let order =
            recover_order_from_measurement(
                problem,
                base,
                measurement,
                precision_bits,
            )?;

        let Some(order) = order else {
            continue;
        };

        valid_orders =
            valid_orders
                .checked_add(1)
                .ok_or_else(|| {
                    invalid_configuration(
                        "valid_order_samples",
                        "valid-order count overflowed",
                    )
                })?;

        if !order.usable() {
            continue;
        }

        usable_orders =
            usable_orders
                .checked_add(1)
                .ok_or_else(|| {
                    invalid_configuration(
                        "usable_order_samples",
                        "usable-order count overflowed",
                    )
                })?;

        let attempt =
            extract_factors(
                problem,
                base,
                order.order,
            )?;

        if let Some(factors) =
            attempt.factors()
        {
            successful_factorizations =
                successful_factorizations
                    .checked_add(1)
                    .ok_or_else(|| {
                        invalid_configuration(
                            "successful_factorizations",
                            "factorization count overflowed",
                        )
                    })?;

            if first_factors.is_none() {
                first_factors =
                    Some(factors);
            }
        }
    }

    ShorBenchmarkResult::new(
        problem.modulus(),
        base,
        precision_bits,
        samples,
        valid_orders,
        usable_orders,
        successful_factorizations,
        first_factors,
    )
}

// =============================================================================
// Classical reference
// =============================================================================

/// Executes the complete classical reference reduction for a selected base.
///
/// This function exists for:
///
/// - small-instance verification;
/// - unit tests;
/// - correctness fixtures;
/// - validating the classical post-processing of quantum measurements.
///
/// It must never be interpreted as a measurement of quantum performance.
pub fn classical_reference(
    problem: ShorProblem,
    base: u64,
) -> BenchmarkResult<ShorAttemptResult> {
    problem.validate_base(base)?;

    let order =
        multiplicative_order(
            base,
            problem.modulus(),
        )?;

    let Some(order) = order else {
        return Ok(
            ShorAttemptResult::retry(
                ShorAttemptStatus::InvalidOrder,
                problem.modulus(),
                base,
                None,
            ),
        );
    };

    extract_factors(
        problem,
        base,
        order,
    )
}

/// Finds a deterministic valid coprime base.
///
/// This function is intentionally deterministic. It does not use an RNG.
pub fn smallest_coprime_base(
    modulus: u64,
) -> BenchmarkResult<u64> {
    validate_modulus(modulus)?;

    let mut base = 2u64;

    while base < modulus {
        if gcd(base, modulus) == 1 {
            return Ok(base);
        }

        base += 1;
    }

    Err(invalid_configuration(
        "base",
        "unable to find a coprime base",
    ))
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_modulus(
    modulus: u64,
) -> BenchmarkResult<()> {
    if modulus < MIN_SHOR_MODULUS {
        return Err(invalid_configuration(
            "modulus",
            "Shor modulus is too small",
        ));
    }

    if modulus > MAX_REFERENCE_MODULUS {
        return Err(invalid_configuration(
            "modulus",
            "modulus exceeds the supported reference representation",
        ));
    }

    if modulus % 2 == 0 {
        return Err(invalid_configuration(
            "modulus",
            "the general Shor benchmark requires an odd modulus; even inputs have an immediate classical factor",
        ));
    }

    if is_prime(modulus) {
        return Err(invalid_configuration(
            "modulus",
            "Shor factorization benchmark requires a composite modulus",
        ));
    }

    Ok(())
}

fn is_nontrivial_factor(
    factor: u64,
    modulus: u64,
) -> bool {
    factor > 1
        && factor < modulus
        && modulus % factor == 0
}

fn bit_length(value: u64) -> usize {
    if value == 0 {
        0
    } else {
        (u64::BITS - value.leading_zeros())
            as usize
    }
}

/// Deterministic primality test suitable for the `u64` reference layer.
///
/// This uses deterministic Miller-Rabin bases known to be sufficient for the
/// complete `u64` domain.
fn is_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    }

    const SMALL_PRIMES: [u64; 12] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    ];

    for prime in SMALL_PRIMES {
        if value == prime {
            return true;
        }

        if value % prime == 0 {
            return false;
        }
    }

    let mut d = value - 1;
    let mut s = 0u32;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    /*
     * Deterministic for all u64 values.
     *
     * Reference:
     * Jim Sinclair / deterministic Miller-Rabin bounds for 64-bit integers.
     */
    const BASES: [u64; 7] = [
        2,
        325,
        9_375,
        28_178,
        450_775,
        9_780_504,
        1_795_265_022,
    ];

    for base in BASES {
        if base % value == 0 {
            continue;
        }

        let mut x =
            match modular_pow(
                base % value,
                d,
                value,
            ) {
                Ok(value) => value,
                Err(_) => return false,
            };

        if x == 1 || x == value - 1 {
            continue;
        }

        let mut witness_found = true;

        for _ in 1..s {
            x =
                match mul_mod(
                    x,
                    x,
                    value,
                ) {
                    Ok(value) => value,
                    Err(_) => {
                        witness_found = false;
                        break;
                    }
                };

            if x == value - 1 {
                witness_found = false;
                break;
            }
        }

        if witness_found {
            return false;
        }
    }

    true
}

// =============================================================================
// Error conversion
// =============================================================================

fn invalid_configuration(
    field: &'static str,
    reason: &'static str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_is_correct() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(24, 54), 6);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(9, 0), 9);
    }

    #[test]
    fn modular_power_is_correct() {
        assert_eq!(
            modular_pow(2, 10, 1_000)
                .unwrap(),
            24
        );

        assert_eq!(
            modular_pow(7, 4, 15)
                .unwrap(),
            1
        );
    }

    #[test]
    fn modular_multiplication_avoids_overflow() {
        let modulus = u64::MAX - 58;

        let result =
            mul_mod(
                modulus - 2,
                modulus - 3,
                modulus,
            )
            .unwrap();

        assert_eq!(
            result,
            6
        );
    }

    #[test]
    fn rejects_even_modulus() {
        assert!(
            ShorProblem::new(15).is_ok()
        );

        assert!(
            ShorProblem::new(21).is_ok()
        );

        assert!(
            ShorProblem::new(15).is_ok()
        );

        assert!(
            ShorProblem::new(14).is_err()
        );
    }

    #[test]
    fn rejects_prime_modulus() {
        assert!(
            ShorProblem::new(13).is_err()
        );
    }

    #[test]
    fn validates_base() {
        let problem =
            ShorProblem::new(15)
                .unwrap();

        assert!(
            problem
                .validate_base(2)
                .is_ok()
        );

        assert!(
            problem
                .validate_base(5)
                .is_err()
        );
    }

    #[test]
    fn finds_order_for_15() {
        let order =
            multiplicative_order(
                2,
                15,
            )
            .unwrap();

        assert_eq!(
            order,
            Some(4)
        );
    }

    #[test]
    fn extracts_factors_from_order() {
        let problem =
            ShorProblem::new(15)
                .unwrap();

        let result =
            extract_factors(
                problem,
                2,
                4,
            )
            .unwrap();

        assert!(
            result.succeeded()
        );

        let factors =
            result
                .factors()
                .unwrap();

        assert_eq!(
            factors.0
                * factors.1,
            15
        );
    }

    #[test]
    fn odd_order_requires_retry() {
        let problem =
            ShorProblem::new(21)
                .unwrap();

        let result =
            extract_factors(
                problem,
                2,
                3,
            )
            .unwrap();

        assert_eq!(
            result.status,
            ShorAttemptStatus::OddOrder
        );
    }

    #[test]
    fn continued_fraction_convergents_are_valid() {
        let convergents =
            continued_fraction_convergents(
                1,
                4,
            )
            .unwrap();

        assert!(
            convergents
                .iter()
                .any(|value| {
                    value.numerator == 1
                        && value.denominator
                            == 4
                })
        );
    }

    #[test]
    fn recovers_order_from_exact_phase_measurement() {
        /*
         * For N=15 and a=2, r=4.
         *
         * With t=4 phase bits, y=4 represents:
         *
         *   y / 2^t = 4 / 16 = 1/4
         *
         * so the denominator 4 is the true order.
         */
        let problem =
            ShorProblem::new(15)
                .unwrap();

        let result =
            recover_order_from_measurement(
                problem,
                2,
                4,
                4,
            )
            .unwrap()
            .unwrap();

        assert_eq!(
            result.order,
            4
        );

        assert!(
            result.valid
        );
    }

    #[test]
    fn measurement_analysis_finds_factorization() {
        let problem =
            ShorProblem::new(15)
                .unwrap();

        let result =
            analyze_measurements(
                problem,
                2,
                4,
                [4u64],
            )
            .unwrap();

        assert!(
            result.success
        );

        assert_eq!(
            result.factors,
            Some((3, 5))
        );

        assert_eq!(
            result.samples,
            1
        );

        assert_eq!(
            result.usable_order_samples,
            1
        );

        assert_eq!(
            result.successful_factorizations,
            1
        );
    }

    #[test]
    fn classical_reference_factors_15() {
        let problem =
            ShorProblem::new(15)
                .unwrap();

        let result =
            classical_reference(
                problem,
                2,
            )
            .unwrap();

        assert!(
            result.succeeded()
        );

        let factors =
            result
                .factors()
                .unwrap();

        assert_eq!(
            factors.0
                * factors.1,
            15
        );
    }

    #[test]
    fn classical_reference_factors_21() {
        let problem =
            ShorProblem::new(21)
                .unwrap();

        let result =
            classical_reference(
                problem,
                2,
            )
            .unwrap();

        assert!(
            result.succeeded()
        );

        let factors =
            result
                .factors()
                .unwrap();

        assert_eq!(
            factors.0
                * factors.1,
            21
        );
    }

    #[test]
    fn factor_result_rejects_invalid_factors() {
        assert!(
            ShorFactorResult::new(
                15,
                1,
                15,
            )
            .is_err()
        );

        assert!(
            ShorFactorResult::new(
                15,
                3,
                4,
            )
            .is_err()
        );
    }

    #[test]
    fn resource_estimation_is_deterministic() {
        let config =
            ShorBenchmarkConfig::new(
                15,
                2,
            )
            .unwrap();

        let first =
            ShorResourceEstimate::from_config(
                &config,
            )
            .unwrap();

        let second =
            ShorResourceEstimate::from_config(
                &config,
            )
            .unwrap();

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn generator_has_stable_identity() {
        let generator =
            ShorBenchmarkGenerator::standard();

        assert_eq!(
            generator
                .descriptor()
                .generator_id(),
            SHOR_BENCHMARK_ID
        );

        assert_eq!(
            generator
                .descriptor()
                .application_id(),
            SHOR_APPLICATION_ID
        );

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::Deterministic
                )
        );
    }

    #[test]
    fn generator_creates_workload() {
        let generator =
            ShorBenchmarkGenerator::standard();

        /*
         * The application-generation contract represents problem size as
         * usize. This test therefore deliberately uses a small reference
         * instance.
         */
        let request =
            ApplicationGenerationRequest::new(
                SHOR_APPLICATION_ID,
                WorkloadId::new(
                    "shor_n15",
                )
                .unwrap(),
                15,
            )
            .unwrap();

        let workload =
            generator
                .generate_workload(
                    &request,
                )
                .unwrap();

        assert_eq!(
            workload
                .application_id(),
            SHOR_APPLICATION_ID
        );

        assert_eq!(
            workload
                .problem_size(),
            15
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "modulus"
                        && parameter.value()
                            == "15"
                })
        );
    }

    #[test]
    fn smallest_coprime_base_is_deterministic() {
        assert_eq!(
            smallest_coprime_base(15)
                .unwrap(),
            2
        );

        assert_eq!(
            smallest_coprime_base(21)
                .unwrap(),
            2
        );
    }

    #[test]
    fn bit_length_is_correct() {
        assert_eq!(
            bit_length(1),
            1
        );

        assert_eq!(
            bit_length(15),
            4
        );

        assert_eq!(
            bit_length(16),
            5
        );
    }

    #[test]
    fn primality_test_is_correct_for_reference_values() {
        assert!(
            is_prime(2)
        );

        assert!(
            is_prime(97)
        );

        assert!(
            !is_prime(1)
        );

        assert!(
            !is_prime(91)
        );
    }
}