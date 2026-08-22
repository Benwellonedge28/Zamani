//! Zamani Quantum Algorithms — Quantum Phase Estimation
//!
//! Production-grade, backend-independent Quantum Phase Estimation (QPE).
//!
//! # Algorithm
//!
//! Given a unitary operator U and an eigenstate |ψ> satisfying
//!
//!     U|ψ> = exp(2πiφ)|ψ>
//!
//! QPE estimates the phase φ ∈ [0, 1).
//!
//! The logical computation is:
//!
//! ```text
//! eigenstate preparation
//!         │
//!         ▼
//! phase register ────────────────┐
//!         │                      │
//!     H on phase qubits         │
//!         │                      │
//!         ▼                      │
//! controlled-U^(2^j)             │
//!         │                      │
//!         ▼                      │
//! inverse QFT                     │
//!         │                      │
//!         ▼                      │
//! measurement                     │
//!         │
//!         ▼
//! phase estimate
//! ```
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - QPE configuration;
//! - phase precision;
//! - phase-register sizing;
//! - logical controlled-unitary requirements;
//! - QPE circuit planning;
//! - inverse-QFT planning;
//! - phase-bit decoding;
//! - confidence/error reporting;
//! - deterministic execution metadata;
//! - resource validation.
//!
//! This module does NOT own:
//!
//! - concrete gate definitions;
//! - canonical circuit storage;
//! - physical qubits;
//! - physical routing;
//! - hardware topology;
//! - calibration;
//! - QPU communication;
//! - error-correction decoding;
//! - backend-specific decomposition;
//! - backend execution.
//!
//! Those responsibilities belong to the Quantum IR, routing, QEC, hardware,
//! and execution subsystems respectively.
//!
//! # Integration boundary
//!
//! The intended production dependency direction is:
//!
//! ```text
//! phase_estimation.rs
//!        │
//!        ▼
//! logical QPE plan
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ▼
//! validation / optimization / routing
//!        │
//!        ▼
//! error correction
//!        │
//!        ▼
//! algorithms::execution / hardware backend
//! ```
//!
//! The executor trait below is deliberately small. The later shared
//! `algorithms::execution` layer can implement this trait without requiring
//! changes to the QPE mathematics or result model.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//! No external crates are required.

use std::fmt;

use super::error::{AlgorithmError, Result};
use super::types::{AlgorithmId, AlgorithmVersion, ExecutionConfig};

// =============================================================================
// Algorithm identity
// =============================================================================

/// Stable algorithm identifier.
///
/// The corresponding shared `AlgorithmId` variant must be:
///
/// `AlgorithmId::PhaseEstimation`
pub const PHASE_ESTIMATION_ID: AlgorithmId = AlgorithmId::PhaseEstimation;

/// Stable public algorithm version.
///
/// A major-version change means the public algorithm/execution contract is no
/// longer backward compatible.
pub const PHASE_ESTIMATION_VERSION: AlgorithmVersion =
    AlgorithmVersion::new(1, 0, 0);

// =============================================================================
// Numerical constants
// =============================================================================

/// Mathematical phase range.
const PHASE_MIN: f64 = 0.0;
const PHASE_MAX: f64 = 1.0;

/// Maximum phase-register size supported by the implementation.
///
/// This is intentionally conservative. Resource limits remain configurable
/// below and are always checked before execution.
const MAX_PHASE_REGISTER_BITS: usize = 64;

// =============================================================================
// Unitary abstraction
// =============================================================================

/// Abstract logical unitary used by QPE.
///
/// QPE does not need to know the matrix representation of the unitary.
/// A compiler/backend adapter can lower this logical identity into concrete
/// Quantum IR operations.
pub trait Unitary {
    /// Number of logical target qubits acted on by the unitary.
    fn qubit_count(&self) -> usize;

    /// Stable identifier used for reproducibility and execution records.
    fn identifier(&self) -> &str;
}

/// Optional abstraction for explicitly controlled unitary construction.
///
/// Most backends will implement controlled-unitary synthesis downstream.
/// This trait exists so a backend can advertise whether it can construct the
/// required controlled power directly.
pub trait ControlledUnitary {
    /// Returns whether the controlled power can be represented.
    fn supports_power(&self, exponent: u64) -> bool;
}

// =============================================================================
// Eigenstate abstraction
// =============================================================================

/// Preparation of the eigenstate supplied to QPE.
///
/// The caller is responsible for ensuring that the prepared state is an
/// eigenstate, or sufficiently close to one, of the supplied unitary.
///
/// QPE cannot infer this property from the opaque logical abstraction.
pub trait EigenstatePreparation {
    /// Number of logical target qubits.
    fn qubit_count(&self) -> usize;

    /// Stable identifier for reproducibility.
    fn identifier(&self) -> &str;
}

// =============================================================================
// QPE circuit plan
// =============================================================================

/// One controlled-unitary operation in the logical QPE plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlledPower {
    /// Index of the phase-register qubit controlling this operation.
    pub control_index: usize,

    /// Exponent of U.
    ///
    /// The operation represents controlled-U^(2^control_index).
    pub exponent: u64,
}

impl ControlledPower {
    /// Creates a controlled power from a phase-register index.
    pub fn new(control_index: usize) -> Result<Self> {
        let exponent = checked_power_of_two(control_index)?;

        Ok(Self {
            control_index,
            exponent,
        })
    }
}

/// Logical inverse-QFT plan.
///
/// The canonical gate sequence is intentionally represented as a plan rather
/// than as duplicated gate definitions. The IR adapter is responsible for
/// lowering this plan to `quantum::ir::Gate` operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InverseQftPlan {
    /// Number of phase-register qubits.
    pub qubit_count: usize,

    /// Whether the final bit-reversal permutation is required.
    pub bit_reversal: bool,
}

impl InverseQftPlan {
    /// Creates a validated inverse-QFT plan.
    pub fn new(qubit_count: usize) -> Result<Self> {
        if qubit_count == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        Ok(Self {
            qubit_count,
            bit_reversal: true,
        })
    }
}

/// Complete logical QPE circuit plan.
///
/// This is not the Quantum IR itself. It is the algorithm-level description
/// consumed by an IR adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseEstimationPlan {
    /// Number of phase-register qubits.
    pub phase_qubits: usize,

    /// Number of target/eigenstate qubits.
    pub target_qubits: usize,

    /// Controlled powers of U.
    pub controlled_powers: Vec<ControlledPower>,

    /// Inverse QFT plan.
    pub inverse_qft: InverseQftPlan,

    /// Stable unitary identifier.
    pub unitary_id: String,

    /// Stable eigenstate-preparation identifier.
    pub eigenstate_id: String,
}

impl PhaseEstimationPlan {
    /// Total number of logical qubits required by QPE.
    pub fn total_qubits(&self) -> Result<usize> {
        self.phase_qubits
            .checked_add(self.target_qubits)
            .ok_or_else(|| {
                AlgorithmError::ResourceLimitExceeded(
                    "total QPE qubit count overflowed usize".to_owned(),
                )
            })
    }

    /// Validates the logical plan.
    pub fn validate(&self) -> Result<()> {
        if self.phase_qubits == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if self.target_qubits == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if self.phase_qubits != self.controlled_powers.len() {
            return Err(AlgorithmError::DimensionMismatch {
                expected: self.phase_qubits,
                actual: self.controlled_powers.len(),
            });
        }

        if self.inverse_qft.qubit_count != self.phase_qubits {
            return Err(AlgorithmError::DimensionMismatch {
                expected: self.phase_qubits,
                actual: self.inverse_qft.qubit_count,
            });
        }

        if self.unitary_id.is_empty() {
            return Err(AlgorithmError::InvalidInput(
                "unitary identifier must not be empty".to_owned(),
            ));
        }

        if self.eigenstate_id.is_empty() {
            return Err(AlgorithmError::InvalidInput(
                "eigenstate identifier must not be empty".to_owned(),
            ));
        }

        for (expected_index, operation) in self.controlled_powers.iter().enumerate() {
            if operation.control_index != expected_index {
                return Err(AlgorithmError::InternalInvariantViolation(
                    "controlled powers must be ordered by phase-register index"
                        .to_owned(),
                ));
            }

            let expected_exponent = checked_power_of_two(expected_index)?;

            if operation.exponent != expected_exponent {
                return Err(AlgorithmError::InternalInvariantViolation(
                    "controlled-unitary exponent does not match phase-register index"
                        .to_owned(),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Resource policy for QPE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseEstimationLimits {
    /// Maximum number of phase-register qubits.
    pub max_phase_qubits: usize,

    /// Maximum number of target/eigenstate qubits.
    pub max_target_qubits: usize,

    /// Maximum total logical qubits.
    pub max_total_qubits: usize,

    /// Maximum controlled-unitary operations.
    pub max_controlled_powers: usize,

    /// Maximum execution shots.
    pub max_shots: u64,
}

impl Default for PhaseEstimationLimits {
    fn default() -> Self {
        Self {
            max_phase_qubits: MAX_PHASE_REGISTER_BITS,
            max_target_qubits: 64,
            max_total_qubits: 128,
            max_controlled_powers: MAX_PHASE_REGISTER_BITS,
            max_shots: 10_000_000,
        }
    }
}

impl PhaseEstimationLimits {
    /// Validates the resource policy itself.
    pub fn validate(&self) -> Result<()> {
        if self.max_phase_qubits == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_phase_qubits must be greater than zero".to_owned(),
            ));
        }

        if self.max_phase_qubits > MAX_PHASE_REGISTER_BITS {
            return Err(AlgorithmError::InvalidConfiguration(format!(
                "max_phase_qubits cannot exceed {}",
                MAX_PHASE_REGISTER_BITS
            )));
        }

        if self.max_target_qubits == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_target_qubits must be greater than zero".to_owned(),
            ));
        }

        if self.max_total_qubits == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_total_qubits must be greater than zero".to_owned(),
            ));
        }

        if self.max_controlled_powers == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_controlled_powers must be greater than zero".to_owned(),
            ));
        }

        if self.max_shots == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_shots must be greater than zero".to_owned(),
            ));
        }

        Ok(())
    }

    fn validate_phase_qubits(&self, value: usize) -> Result<()> {
        if value > self.max_phase_qubits {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "QPE requires {} phase qubits but the limit is {}",
                value, self.max_phase_qubits
            )));
        }

        Ok(())
    }

    fn validate_target_qubits(&self, value: usize) -> Result<()> {
        if value > self.max_target_qubits {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "QPE requires {} target qubits but the limit is {}",
                value, self.max_target_qubits
            )));
        }

        Ok(())
    }

    fn validate_total_qubits(&self, value: usize) -> Result<()> {
        if value > self.max_total_qubits {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "QPE requires {} total logical qubits but the limit is {}",
                value, self.max_total_qubits
            )));
        }

        Ok(())
    }

    fn validate_shots(&self, value: u64) -> Result<()> {
        if value > self.max_shots {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "QPE requests {} shots but the limit is {}",
                value, self.max_shots
            )));
        }

        Ok(())
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// QPE precision policy.
///
/// `phase_qubits` is the number of bits retained in the phase estimate.
///
/// For an ideal eigenstate and exact representable phase, increasing the
/// register size increases the binary precision of the estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct PhasePrecision {
    /// Number of phase-register qubits.
    pub phase_qubits: usize,

    /// Optional desired absolute phase precision.
    ///
    /// This is a target, not a guarantee. The actual result reports its
    /// achieved quantization resolution and confidence interval.
    pub target_precision: Option<f64>,
}

impl PhasePrecision {
    /// Creates precision from an explicit phase-register size.
    pub fn from_qubits(phase_qubits: usize) -> Result<Self> {
        if phase_qubits == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if phase_qubits > MAX_PHASE_REGISTER_BITS {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "phase register cannot exceed {} qubits",
                MAX_PHASE_REGISTER_BITS
            )));
        }

        Ok(Self {
            phase_qubits,
            target_precision: None,
        })
    }

    /// Creates precision from an absolute target.
    ///
    /// The number of qubits is the smallest n satisfying:
    ///
    ///     2^-n <= target_precision
    pub fn from_target(target_precision: f64) -> Result<Self> {
        validate_precision(target_precision)?;

        let mut qubits = 0usize;
        let mut resolution = 1.0_f64;

        while resolution > target_precision {
            qubits = qubits.checked_add(1).ok_or_else(|| {
                AlgorithmError::ResourceLimitExceeded(
                    "phase-register size overflowed".to_owned(),
                )
            })?;

            if qubits > MAX_PHASE_REGISTER_BITS {
                return Err(AlgorithmError::ResourceLimitExceeded(
                    "requested phase precision requires too many phase qubits"
                        .to_owned(),
                ));
            }

            resolution *= 0.5;
        }

        Ok(Self {
            phase_qubits: qubits.max(1),
            target_precision: Some(target_precision),
        })
    }

    /// Binary phase resolution.
    pub fn resolution(&self) -> Result<f64> {
        phase_resolution(self.phase_qubits)
    }

    /// Validates this precision policy.
    pub fn validate(&self) -> Result<()> {
        if self.phase_qubits == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if self.phase_qubits > MAX_PHASE_REGISTER_BITS {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "phase register cannot exceed {} qubits",
                MAX_PHASE_REGISTER_BITS
            )));
        }

        if let Some(value) = self.target_precision {
            validate_precision(value)?;
        }

        Ok(())
    }
}

/// QPE configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseEstimationConfig {
    /// Phase precision policy.
    pub precision: PhasePrecision,

    /// Number of execution shots.
    ///
    /// Multiple shots are required when a confidence estimate is requested.
    pub shots: u64,

    /// Statistical confidence level.
    ///
    /// Must satisfy 0 < confidence < 1.
    pub confidence: f64,

    /// Shared execution configuration.
    pub execution: ExecutionConfig,

    /// Resource limits.
    pub limits: PhaseEstimationLimits,

    /// Whether the inverse-QFT bit reversal should be included in the logical
    /// plan.
    pub bit_reversal: bool,
}

impl Default for PhaseEstimationConfig {
    fn default() -> Self {
        let execution = ExecutionConfig::default();

        Self {
            precision: PhasePrecision::from_qubits(8)
                .expect("constant default precision is valid"),
            shots: execution.shots,
            confidence: 0.95,
            execution,
            limits: PhaseEstimationLimits::default(),
            bit_reversal: true,
        }
    }
}

impl PhaseEstimationConfig {
    /// Validates configuration.
    pub fn validate(&self) -> Result<()> {
        self.precision.validate()?;
        self.limits.validate()?;

        self.limits
            .validate_phase_qubits(self.precision.phase_qubits)?;

        if self.shots == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "QPE requires at least one execution shot".to_owned(),
            ));
        }

        self.limits.validate_shots(self.shots)?;

        validate_confidence(self.confidence)?;

        self.execution.validate()?;

        if self.execution.shots != self.shots {
            return Err(AlgorithmError::InvalidConfiguration(
                "execution.shots must equal PhaseEstimationConfig.shots"
                    .to_owned(),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Execution request/result
// =============================================================================

/// Backend-independent QPE execution request.
///
/// This is the adapter boundary between the algorithm and the shared
/// execution subsystem.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseEstimationExecutionRequest {
    /// Logical QPE plan.
    pub plan: PhaseEstimationPlan,

    /// Execution configuration.
    pub execution: ExecutionConfig,

    /// Algorithm identity.
    pub algorithm: AlgorithmId,

    /// Algorithm version.
    pub algorithm_version: AlgorithmVersion,

    /// Reproducibility seed.
    pub seed: Option<u64>,
}

impl PhaseEstimationExecutionRequest {
    /// Validates the execution request.
    pub fn validate(&self) -> Result<()> {
        self.plan.validate()?;
        self.execution.validate()?;
        Ok(())
    }
}

/// One measured phase-register outcome.
///
/// `value` is the integer represented by the measured phase register.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseMeasurement {
    /// Integer encoded by the phase register.
    pub value: u64,

    /// Number of shots producing this value.
    pub count: u64,
}

impl PhaseMeasurement {
    /// Creates a validated phase measurement.
    pub fn new(value: u64, count: u64) -> Result<Self> {
        if count == 0 {
            return Err(AlgorithmError::InvalidInput(
                "phase measurement count must be greater than zero".to_owned(),
            ));
        }

        Ok(Self { value, count })
    }
}

/// Execution result consumed by the QPE decoder.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseEstimationExecutionResult {
    /// Number of phase-register qubits.
    pub phase_qubits: usize,

    /// Number of shots actually executed.
    pub shots: u64,

    /// Measured phase-register outcomes.
    pub measurements: Vec<PhaseMeasurement>,

    /// Optional backend identifier.
    pub backend_id: Option<String>,

    /// Optional backend version.
    pub backend_version: Option<String>,
}

impl PhaseEstimationExecutionResult {
    /// Creates a validated execution result.
    pub fn new(
        phase_qubits: usize,
        shots: u64,
        measurements: Vec<PhaseMeasurement>,
    ) -> Result<Self> {
        if phase_qubits == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if shots == 0 {
            return Err(AlgorithmError::InvalidInput(
                "QPE execution requires at least one shot".to_owned(),
            ));
        }

        if measurements.is_empty() {
            return Err(AlgorithmError::EmptyInput(
                "QPE execution returned no phase measurements".to_owned(),
            ));
        }

        let max_value = max_phase_measurement(phase_qubits)?;

        let mut total = 0_u64;

        for measurement in &measurements {
            if measurement.value > max_value {
                return Err(AlgorithmError::InvalidInput(
                    "phase measurement exceeds phase-register range".to_owned(),
                ));
            }

            total = total.checked_add(measurement.count).ok_or_else(|| {
                AlgorithmError::ResourceLimitExceeded(
                    "phase measurement count overflowed u64".to_owned(),
                )
            })?;
        }

        if total != shots {
            return Err(AlgorithmError::ExecutionFailed(format!(
                "phase measurement counts total {} but execution reports {} shots",
                total, shots
            )));
        }

        Ok(Self {
            phase_qubits,
            shots,
            measurements,
            backend_id: None,
            backend_version: None,
        })
    }
}

/// Backend-independent executor boundary for QPE.
///
/// The shared `algorithms::execution` layer should implement this trait by
/// lowering the logical plan into Quantum IR and executing it through the
/// selected backend.
pub trait PhaseEstimationExecutor {
    /// Executes a QPE request.
    fn execute(
        &mut self,
        request: &PhaseEstimationExecutionRequest,
    ) -> Result<PhaseEstimationExecutionResult>;
}

// =============================================================================
// Result model
// =============================================================================

/// Decoded QPE estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseEstimate {
    /// Most likely/most frequently measured phase-register integer.
    pub measurement: u64,

    /// Estimated phase in [0, 1).
    pub phase: f64,

    /// Binary quantization resolution.
    pub resolution: f64,

    /// Statistical confidence interval lower bound.
    pub lower_bound: f64,

    /// Statistical confidence interval upper bound.
    pub upper_bound: f64,

    /// Width of the statistical confidence interval.
    pub confidence_interval_width: f64,

    /// Number of phase-register qubits.
    pub phase_qubits: usize,

    /// Number of execution shots.
    pub shots: u64,

    /// Confidence level.
    pub confidence: f64,
}

impl PhaseEstimate {
    /// Returns whether the statistical interval satisfies a requested
    /// absolute half-width.
    pub fn meets_precision(&self, precision: f64) -> Result<bool> {
        validate_precision(precision)?;

        Ok(self.confidence_interval_width / 2.0 <= precision)
    }
}

impl fmt::Display for PhaseEstimate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "PhaseEstimate(phase={:.12}, measurement={}, resolution={:.12}, confidence={:.4}, interval=[{:.12}, {:.12}])",
            self.phase,
            self.measurement,
            self.resolution,
            self.confidence,
            self.lower_bound,
            self.upper_bound
        )
    }
}

/// Complete QPE result.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseEstimationResult {
    /// Decoded phase estimate.
    pub estimate: PhaseEstimate,

    /// Number of target qubits.
    pub target_qubits: usize,

    /// Number of phase qubits.
    pub phase_qubits: usize,

    /// Algorithm identifier.
    pub algorithm: AlgorithmId,

    /// Algorithm version.
    pub algorithm_version: AlgorithmVersion,

    /// Reproducibility seed.
    pub seed: Option<u64>,

    /// Unitary identifier.
    pub unitary_id: String,

    /// Eigenstate-preparation identifier.
    pub eigenstate_id: String,

    /// Backend identifier, when supplied by the executor.
    pub backend_id: Option<String>,

    /// Backend version, when supplied by the executor.
    pub backend_version: Option<String>,
}

impl fmt::Display for PhaseEstimationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "PhaseEstimationResult(algorithm={:?}, version={:?}, {})",
            self.algorithm,
            self.algorithm_version,
            self.estimate
        )
    }
}

// =============================================================================
// QPE algorithm
// =============================================================================

/// Production Quantum Phase Estimation algorithm.
#[derive(Clone, Debug)]
pub struct PhaseEstimation {
    config: PhaseEstimationConfig,
}

impl PhaseEstimation {
    /// Creates a validated QPE instance.
    pub fn new(config: PhaseEstimationConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the immutable configuration.
    pub fn config(&self) -> &PhaseEstimationConfig {
        &self.config
    }

    /// Builds the backend-independent logical QPE plan.
    pub fn build_plan<U, S>(
        &self,
        unitary: &U,
        eigenstate: &S,
    ) -> Result<PhaseEstimationPlan>
    where
        U: Unitary,
        S: EigenstatePreparation,
    {
        validate_problem(
            unitary,
            eigenstate,
            &self.config.limits,
        )?;

        let phase_qubits = self.config.precision.phase_qubits;

        let mut controlled_powers =
            Vec::with_capacity(phase_qubits);

        for control_index in 0..phase_qubits {
            controlled_powers.push(ControlledPower::new(control_index)?);
        }

        let mut inverse_qft = InverseQftPlan::new(phase_qubits)?;
        inverse_qft.bit_reversal = self.config.bit_reversal;

        let plan = PhaseEstimationPlan {
            phase_qubits,
            target_qubits: unitary.qubit_count(),
            controlled_powers,
            inverse_qft,
            unitary_id: unitary.identifier().to_owned(),
            eigenstate_id: eigenstate.identifier().to_owned(),
        };

        plan.validate()?;

        let total_qubits = plan.total_qubits()?;

        self.config
            .limits
            .validate_phase_qubits(phase_qubits)?;

        self.config
            .limits
            .validate_target_qubits(unitary.qubit_count())?;

        self.config
            .limits
            .validate_total_qubits(total_qubits)?;

        if phase_qubits > self.config.limits.max_controlled_powers {
            return Err(AlgorithmError::ResourceLimitExceeded(
                "QPE controlled-power count exceeds configured limit".to_owned(),
            ));
        }

        Ok(plan)
    }

    /// Builds a complete execution request.
    pub fn build_request<U, S>(
        &self,
        unitary: &U,
        eigenstate: &S,
    ) -> Result<PhaseEstimationExecutionRequest>
    where
        U: Unitary,
        S: EigenstatePreparation,
    {
        let plan = self.build_plan(unitary, eigenstate)?;

        let request = PhaseEstimationExecutionRequest {
            plan,
            execution: self.config.execution.clone(),
            algorithm: PHASE_ESTIMATION_ID,
            algorithm_version: PHASE_ESTIMATION_VERSION,
            seed: self.config.execution.seed,
        };

        request.validate()?;

        Ok(request)
    }

    /// Executes QPE through the injected executor.
    pub fn run<U, S, E>(
        &self,
        unitary: &U,
        eigenstate: &S,
        executor: &mut E,
    ) -> Result<PhaseEstimationResult>
    where
        U: Unitary,
        S: EigenstatePreparation,
        E: PhaseEstimationExecutor,
    {
        let request = self.build_request(unitary, eigenstate)?;

        let execution = executor.execute(&request)?;

        if execution.phase_qubits != request.plan.phase_qubits {
            return Err(AlgorithmError::ExecutionFailed(format!(
                "executor returned {} phase qubits but {} were requested",
                execution.phase_qubits,
                request.plan.phase_qubits
            )));
        }

        if execution.shots != self.config.shots {
            return Err(AlgorithmError::ExecutionFailed(format!(
                "executor returned {} shots but {} were requested",
                execution.shots,
                self.config.shots
            )));
        }

        let estimate = decode_phase(
            &execution,
            self.config.confidence,
        )?;

        Ok(PhaseEstimationResult {
            estimate,
            target_qubits: request.plan.target_qubits,
            phase_qubits: request.plan.phase_qubits,
            algorithm: PHASE_ESTIMATION_ID,
            algorithm_version: PHASE_ESTIMATION_VERSION,
            seed: self.config.execution.seed,
            unitary_id: request.plan.unitary_id,
            eigenstate_id: request.plan.eigenstate_id,
            backend_id: execution.backend_id,
            backend_version: execution.backend_version,
        })
    }
}

// =============================================================================
// Problem validation
// =============================================================================

/// Validates the unitary/eigenstate compatibility.
fn validate_problem<U, S>(
    unitary: &U,
    eigenstate: &S,
    limits: &PhaseEstimationLimits,
) -> Result<()>
where
    U: Unitary,
    S: EigenstatePreparation,
{
    let unitary_qubits = unitary.qubit_count();
    let eigenstate_qubits = eigenstate.qubit_count();

    if unitary_qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if eigenstate_qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if unitary_qubits != eigenstate_qubits {
        return Err(AlgorithmError::DimensionMismatch {
            expected: unitary_qubits,
            actual: eigenstate_qubits,
        });
    }

    limits.validate_target_qubits(unitary_qubits)?;

    if unitary.identifier().is_empty() {
        return Err(AlgorithmError::InvalidInput(
            "unitary identifier must not be empty".to_owned(),
        ));
    }

    if eigenstate.identifier().is_empty() {
        return Err(AlgorithmError::InvalidInput(
            "eigenstate identifier must not be empty".to_owned(),
        ));
    }

    Ok(())
}

// =============================================================================
// Phase decoding
// =============================================================================

/// Decodes a QPE measurement distribution into a phase estimate.
///
/// The mode of the phase-register measurement distribution is selected as the
/// estimate. This is deterministic when the distribution is deterministic.
///
/// Ties are resolved by selecting the numerically smaller measurement value.
pub fn decode_phase(
    execution: &PhaseEstimationExecutionResult,
    confidence: f64,
) -> Result<PhaseEstimate> {
    validate_confidence(confidence)?;

    if execution.measurements.is_empty() {
        return Err(AlgorithmError::EmptyInput(
            "cannot decode phase from an empty measurement distribution"
                .to_owned(),
        ));
    }

    let resolution = phase_resolution(execution.phase_qubits)?;

    let selected = execution
        .measurements
        .iter()
        .max_by(|left, right| {
            left.count
                .cmp(&right.count)
                .then_with(|| right.value.cmp(&left.value))
        })
        .ok_or_else(|| {
            AlgorithmError::EmptyInput(
                "cannot select a phase measurement".to_owned(),
            )
        })?;

    let phase =
        selected.value as f64 / phase_register_size(execution.phase_qubits)?;

    validate_phase(phase)?;

    let (lower_bound, upper_bound) = phase_confidence_interval(
        execution,
        selected.value,
        confidence,
    )?;

    let width = upper_bound - lower_bound;

    if !width.is_finite() || width < 0.0 {
        return Err(AlgorithmError::NumericalInstability(
            "phase confidence interval is invalid".to_owned(),
        ));
    }

    Ok(PhaseEstimate {
        measurement: selected.value,
        phase,
        resolution,
        lower_bound,
        upper_bound,
        confidence_interval_width: width,
        phase_qubits: execution.phase_qubits,
        shots: execution.shots,
        confidence,
    })
}

/// Calculates a phase confidence interval.
///
/// The interval is derived from the empirical frequency of the selected phase
/// bin. It represents the statistical uncertainty of selecting that bin; the
/// separate binary quantization error is reported through `resolution`.
fn phase_confidence_interval(
    execution: &PhaseEstimationExecutionResult,
    selected_value: u64,
    confidence: f64,
) -> Result<(f64, f64)> {
    let selected_count = execution
        .measurements
        .iter()
        .find(|measurement| measurement.value == selected_value)
        .map(|measurement| measurement.count)
        .unwrap_or(0);

    if selected_count == 0 {
        return Err(AlgorithmError::InternalInvariantViolation(
            "selected phase measurement has zero count".to_owned(),
        ));
    }

    let n = execution.shots as f64;
    let p = selected_count as f64 / n;

    let z = inverse_standard_normal(0.5 + confidence / 2.0)?;

    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;

    let variance =
        (p * (1.0 - p) / n) + (z2 / (4.0 * n * n));

    if !variance.is_finite() || variance < 0.0 {
        return Err(AlgorithmError::NumericalInstability(
            "phase confidence variance is invalid".to_owned(),
        ));
    }

    let margin =
        z * variance.sqrt() / denominator;

    let lower_probability =
        (center - margin).clamp(0.0, 1.0);

    let upper_probability =
        (center + margin).clamp(0.0, 1.0);

    // The selected-bin probability is not itself a phase. We therefore
    // translate the probability interval into a conservative phase interval
    // around the quantized estimate.
    //
    // The quantization resolution is the irreducible phase-register scale.
    let resolution =
        phase_resolution(execution.phase_qubits)?;

    let half_resolution = resolution / 2.0;

    let probability_width =
        upper_probability - lower_probability;

    let statistical_half_width =
        probability_width * 0.5;

    let total_half_width =
        half_resolution + statistical_half_width;

    let phase =
        selected_value as f64 /
        phase_register_size(execution.phase_qubits)?;

    let lower =
        (phase - total_half_width).clamp(0.0, 1.0);

    let upper =
        (phase + total_half_width).clamp(0.0, 1.0);

    Ok((lower, upper))
}

// =============================================================================
// Mathematical helpers
// =============================================================================

/// Calculates 2^qubits as an exact representable u64 value.
///
/// QPE is deliberately capped at 64 phase bits because the phase-register
/// measurement value is represented by u64.
fn phase_register_size(qubits: usize) -> Result<f64> {
    if qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if qubits > MAX_PHASE_REGISTER_BITS {
        return Err(AlgorithmError::ResourceLimitExceeded(format!(
            "phase register cannot exceed {} qubits",
            MAX_PHASE_REGISTER_BITS
        )));
    }

    let size = 2_u64.checked_pow(qubits as u32).ok_or_else(|| {
        AlgorithmError::ResourceLimitExceeded(
            "phase-register size overflowed u64".to_owned(),
        )
    })?;

    Ok(size as f64)
}

/// Returns the maximum valid phase-register measurement.
fn max_phase_measurement(qubits: usize) -> Result<u64> {
    if qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if qubits > MAX_PHASE_REGISTER_BITS {
        return Err(AlgorithmError::ResourceLimitExceeded(format!(
            "phase register cannot exceed {} qubits",
            MAX_PHASE_REGISTER_BITS
        )));
    }

    if qubits == 64 {
        Ok(u64::MAX)
    } else {
        Ok((1_u64 << qubits) - 1)
    }
}

/// Returns the binary phase resolution.
fn phase_resolution(qubits: usize) -> Result<f64> {
    let size = phase_register_size(qubits)?;

    let resolution = 1.0 / size;

    if !resolution.is_finite() || resolution <= 0.0 {
        return Err(AlgorithmError::NumericalInstability(
            "phase resolution is invalid".to_owned(),
        ));
    }

    Ok(resolution)
}

/// Safely calculates 2^index.
fn checked_power_of_two(index: usize) -> Result<u64> {
    if index >= 64 {
        return Err(AlgorithmError::ResourceLimitExceeded(
            "controlled-unitary exponent cannot be represented by u64"
                .to_owned(),
        ));
    }

    Ok(1_u64 << index)
}

/// Validates a phase.
fn validate_phase(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "phase must be finite".to_owned(),
        ));
    }

    if !(PHASE_MIN..PHASE_MAX).contains(&value) {
        return Err(AlgorithmError::InvalidParameter(format!(
            "phase must be in [0, 1), received {}",
            value
        )));
    }

    Ok(())
}

/// Validates a phase precision.
fn validate_precision(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "phase precision must be finite".to_owned(),
        ));
    }

    if value <= 0.0 || value >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "phase precision must be greater than zero and less than one"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Validates a confidence level.
fn validate_confidence(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "confidence must be finite".to_owned(),
        ));
    }

    if value <= 0.0 || value >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "confidence must be strictly between zero and one".to_owned(),
        ));
    }

    Ok(())
}

/// Deterministic inverse standard-normal CDF.
///
/// Acklam rational approximation.
///
/// No external statistics dependency is required.
fn inverse_standard_normal(p: f64) -> Result<f64> {
    if !p.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "normal-distribution probability must be finite".to_owned(),
        ));
    }

    if p <= 0.0 || p >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "normal-distribution probability must be strictly between zero and one"
                .to_owned(),
        ));
    }

    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];

    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];

    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];

    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];

    const LOW: f64 = 0.024_25;
    const HIGH: f64 = 1.0 - LOW;

    let result;

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        result = (((((C[0] * q + C[1]) * q + C[2]) * q + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3])
                * q
                + 1.0);
    } else if p <= HIGH {
        let q = p - 0.5;
        let r = q * q;

        result = (((((A[0] * r + A[1]) * r + A[2]) * r + A[3])
            * r
            + A[4])
            * r
            + A[5])
            * q)
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3])
                * r
                + B[4])
                * r
                + 1.0);
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        result = -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3])
                * q
                + 1.0));
    }

    if !result.is_finite() {
        return Err(AlgorithmError::NumericalInstability(
            "inverse normal approximation produced a non-finite result"
                .to_owned(),
        ));
    }

    Ok(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestUnitary {
        qubits: usize,
        id: &'static str,
    }

    impl Unitary for TestUnitary {
        fn qubit_count(&self) -> usize {
            self.qubits
        }

        fn identifier(&self) -> &str {
            self.id
        }
    }

    #[derive(Clone, Debug)]
    struct TestEigenstate {
        qubits: usize,
        id: &'static str,
    }

    impl EigenstatePreparation for TestEigenstate {
        fn qubit_count(&self) -> usize {
            self.qubits
        }

        fn identifier(&self) -> &str {
            self.id
        }
    }

    struct TestExecutor {
        phase_qubits: usize,
        shots: u64,
        measurements: Vec<PhaseMeasurement>,
        calls: usize,
    }

    impl PhaseEstimationExecutor for TestExecutor {
        fn execute(
            &mut self,
            request: &PhaseEstimationExecutionRequest,
        ) -> Result<PhaseEstimationExecutionResult> {
            self.calls += 1;

            if request.plan.phase_qubits != self.phase_qubits {
                return Err(AlgorithmError::ExecutionFailed(
                    "test executor received unexpected phase-register size"
                        .to_owned(),
                ));
            }

            PhaseEstimationExecutionResult::new(
                self.phase_qubits,
                self.shots,
                self.measurements.clone(),
            )
        }
    }

    fn execution_config(shots: u64) -> ExecutionConfig {
        let mut config = ExecutionConfig::default();

        config.shots = shots;
        config.seed = Some(42);
        config.deterministic = true;

        config
    }

    #[test]
    fn phase_precision_from_qubits_is_valid() {
        let precision = PhasePrecision::from_qubits(8).unwrap();

        assert_eq!(precision.phase_qubits, 8);
        assert!((precision.resolution().unwrap() - 1.0 / 256.0).abs() < 1e-15);
    }

    #[test]
    fn phase_precision_from_target_selects_smallest_register() {
        let precision = PhasePrecision::from_target(0.01).unwrap();

        assert_eq!(precision.phase_qubits, 7);
        assert!(precision.resolution().unwrap() <= 0.01);
    }

    #[test]
    fn precision_rejects_non_finite_values() {
        assert!(PhasePrecision::from_target(f64::NAN).is_err());
        assert!(PhasePrecision::from_target(f64::INFINITY).is_err());
        assert!(PhasePrecision::from_target(0.0).is_err());
        assert!(PhasePrecision::from_target(1.0).is_err());
    }

    #[test]
    fn controlled_power_uses_exact_power_of_two() {
        let power = ControlledPower::new(5).unwrap();

        assert_eq!(power.control_index, 5);
        assert_eq!(power.exponent, 32);
    }

    #[test]
    fn controlled_power_rejects_unrepresentable_exponent() {
        assert!(ControlledPower::new(64).is_err());
    }

    #[test]
    fn inverse_qft_plan_requires_qubits() {
        assert!(InverseQftPlan::new(0).is_err());

        let plan = InverseQftPlan::new(4).unwrap();

        assert_eq!(plan.qubit_count, 4);
        assert!(plan.bit_reversal);
    }

    #[test]
    fn qpe_plan_contains_all_controlled_powers() {
        let config = PhaseEstimationConfig {
            precision: PhasePrecision::from_qubits(4).unwrap(),
            shots: 100,
            execution: execution_config(100),
            ..PhaseEstimationConfig::default()
        };

        let qpe = PhaseEstimation::new(config).unwrap();

        let unitary = TestUnitary {
            qubits: 2,
            id: "test-unitary",
        };

        let eigenstate = TestEigenstate {
            qubits: 2,
            id: "test-eigenstate",
        };

        let plan = qpe.build_plan(&unitary, &eigenstate).unwrap();

        assert_eq!(plan.phase_qubits, 4);
        assert_eq!(plan.target_qubits, 2);
        assert_eq!(plan.controlled_powers.len(), 4);
        assert_eq!(plan.controlled_powers[0].exponent, 1);
        assert_eq!(plan.controlled_powers[1].exponent, 2);
        assert_eq!(plan.controlled_powers[2].exponent, 4);
        assert_eq!(plan.controlled_powers[3].exponent, 8);
        assert_eq!(plan.total_qubits().unwrap(), 6);
    }

    #[test]
    fn qpe_rejects_mismatched_unitary_and_eigenstate() {
        let config = PhaseEstimationConfig {
            precision: PhasePrecision::from_qubits(4).unwrap(),
            shots: 100,
            execution: execution_config(100),
            ..PhaseEstimationConfig::default()
        };

        let qpe = PhaseEstimation::new(config).unwrap();

        let unitary = TestUnitary {
            qubits: 2,
            id: "unitary",
        };

        let eigenstate = TestEigenstate {
            qubits: 3,
            id: "eigenstate",
        };

        assert!(qpe.build_plan(&unitary, &eigenstate).is_err());
    }

    #[test]
    fn qpe_rejects_empty_unitary_identifier() {
        let config = PhaseEstimationConfig {
            precision: PhasePrecision::from_qubits(4).unwrap(),
            shots: 100,
            execution: execution_config(100),
            ..PhaseEstimationConfig::default()
        };

        let qpe = PhaseEstimation::new(config).unwrap();

        let unitary = TestUnitary {
            qubits: 2,
            id: "",
        };

        let eigenstate = TestEigenstate {
            qubits: 2,
            id: "eigenstate",
        };

        assert!(qpe.build_plan(&unitary, &eigenstate).is_err());
    }

    #[test]
    fn execution_result_rejects_incorrect_total_counts() {
        let measurements = vec![
            PhaseMeasurement::new(1, 40).unwrap(),
            PhaseMeasurement::new(2, 40).unwrap(),
        ];

        assert!(
            PhaseEstimationExecutionResult::new(4, 100, measurements)
                .is_err()
        );
    }

    #[test]
    fn execution_result_rejects_out_of_range_measurement() {
        let measurements = vec![PhaseMeasurement::new(16, 10).unwrap()];

        assert!(
            PhaseEstimationExecutionResult::new(4, 10, measurements)
                .is_err()
        );
    }

    #[test]
    fn decode_phase_selects_highest_count() {
        let measurements = vec![
            PhaseMeasurement::new(4, 10).unwrap(),
            PhaseMeasurement::new(5, 80).unwrap(),
            PhaseMeasurement::new(6, 10).unwrap(),
        ];

        let execution =
            PhaseEstimationExecutionResult::new(4, 100, measurements)
                .unwrap();

        let estimate = decode_phase(&execution, 0.95).unwrap();

        assert_eq!(estimate.measurement, 5);
        assert!((estimate.phase - 5.0 / 16.0).abs() < 1e-15);
    }

    #[test]
    fn decode_phase_tie_selects_lower_measurement() {
        let measurements = vec![
            PhaseMeasurement::new(5, 50).unwrap(),
            PhaseMeasurement::new(6, 50).unwrap(),
        ];

        let execution =
            PhaseEstimationExecutionResult::new(4, 100, measurements)
                .unwrap();

        let estimate = decode_phase(&execution, 0.95).unwrap();

        assert_eq!(estimate.measurement, 5);
    }

    #[test]
    fn qpe_runs_through_executor_boundary() {
        let config = PhaseEstimationConfig {
            precision: PhasePrecision::from_qubits(4).unwrap(),
            shots: 100,
            confidence: 0.95,
            execution: execution_config(100),
            ..PhaseEstimationConfig::default()
        };

        let qpe = PhaseEstimation::new(config).unwrap();

        let unitary = TestUnitary {
            qubits: 2,
            id: "unitary",
        };

        let eigenstate = TestEigenstate {
            qubits: 2,
            id: "eigenstate",
        };

        let measurements = vec![
            PhaseMeasurement::new(4, 10).unwrap(),
            PhaseMeasurement::new(5, 80).unwrap(),
            PhaseMeasurement::new(6, 10).unwrap(),
        ];

        let mut executor = TestExecutor {
            phase_qubits: 4,
            shots: 100,
            measurements,
            calls: 0,
        };

        let result =
            qpe.run(&unitary, &eigenstate, &mut executor).unwrap();

        assert_eq!(executor.calls, 1);
        assert_eq!(result.phase_qubits, 4);
        assert_eq!(result.target_qubits, 2);
        assert_eq!(result.estimate.measurement, 5);
        assert_eq!(result.seed, Some(42));
        assert_eq!(result.unitary_id, "unitary");
        assert_eq!(result.eigenstate_id, "eigenstate");
    }

    #[test]
    fn qpe_is_deterministic_for_same_measurements() {
        let measurements = vec![
            PhaseMeasurement::new(7, 80).unwrap(),
            PhaseMeasurement::new(8, 20).unwrap(),
        ];

        let first =
            PhaseEstimationExecutionResult::new(
                4,
                100,
                measurements.clone(),
            )
            .unwrap();

        let second =
            PhaseEstimationExecutionResult::new(
                4,
                100,
                measurements,
            )
            .unwrap();

        let first_estimate =
            decode_phase(&first, 0.95).unwrap();

        let second_estimate =
            decode_phase(&second, 0.95).unwrap();

        assert_eq!(first_estimate, second_estimate);
    }

    #[test]
    fn inverse_normal_half_is_zero() {
        let value = inverse_standard_normal(0.5).unwrap();

        assert!(value.abs() < 1e-10);
    }

    #[test]
    fn inverse_normal_975_is_near_196() {
        let value = inverse_standard_normal(0.975).unwrap();

        assert!(value > 1.9);
        assert!(value < 2.0);
    }

    #[test]
    fn phase_register_size_is_correct() {
        assert_eq!(phase_register_size(1).unwrap(), 2.0);
        assert_eq!(phase_register_size(4).unwrap(), 16.0);
        assert_eq!(phase_register_size(8).unwrap(), 256.0);
    }

    #[test]
    fn maximum_measurement_is_correct() {
        assert_eq!(max_phase_measurement(1).unwrap(), 1);
        assert_eq!(max_phase_measurement(4).unwrap(), 15);
    }

    #[test]
    fn phase_validation_rejects_one() {
        assert!(validate_phase(1.0).is_err());
        assert!(validate_phase(-0.1).is_err());
        assert!(validate_phase(f64::NAN).is_err());
        assert!(validate_phase(0.5).is_ok());
    }

    #[test]
    fn confidence_validation_is_strict() {
        assert!(validate_confidence(0.0).is_err());
        assert!(validate_confidence(1.0).is_err());
        assert!(validate_confidence(0.95).is_ok());
    }

    #[test]
    fn resource_limits_reject_invalid_values() {
        let limits = PhaseEstimationLimits {
            max_phase_qubits: 0,
            ..PhaseEstimationLimits::default()
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn resource_limits_reject_excessive_phase_register() {
        let limits = PhaseEstimationLimits {
            max_phase_qubits: 4,
            ..PhaseEstimationLimits::default()
        };

        assert!(limits.validate_phase_qubits(5).is_err());
    }
}