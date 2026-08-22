//! Zamani Quantum Algorithms — Stable Shared Types and Contracts.
//!
//! This module contains the backend-independent data contracts shared by the
//! complete `quantum::algorithms` subsystem.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - algorithm identity;
//! - algorithm semantic versioning;
//! - algorithm metadata;
//! - strongly typed scalar values;
//! - classical algorithm parameter vectors;
//! - execution configuration;
//! - deterministic execution configuration;
//! - resource limits;
//! - measurement-count representation;
//! - backend-neutral execution metadata;
//! - replay/provenance digest metadata;
//! - common validation helpers.
//!
//! This module deliberately does NOT own:
//!
//! - quantum gates;
//! - quantum circuits;
//! - qubit topology;
//! - routing;
//! - transpilation;
//! - hardware;
//! - backend implementations;
//! - error correction;
//! - optimizer implementations;
//! - objective implementations;
//! - persistence;
//! - telemetry transport.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Dependency contract
//!
//! ```text
//! algorithms::error
//!        │
//!        ▼
//! algorithms::types
//!        │
//!        ├──────────────► execution
//!        ├──────────────► objective
//!        ├──────────────► optimizer
//!        └──────────────► algorithms
//! ```
//!
//! `types.rs` must remain independent from those consumers.
//!
//! # Parameter distinction
//!
//! [`ParameterVector`] contains concrete classical numerical values used by
//! algorithm orchestration and optimization.
//!
//! It is deliberately distinct from `quantum::ir::Parameter`, which belongs
//! to the quantum IR and represents circuit-level parameter semantics.
//!
//! ```text
//! ParameterVector
//!       │
//!       ▼
//!      Ansatz
//!       │
//!       ▼
//! quantum::ir::Parameter
//!       │
//!       ▼
//! QuantumCircuit
//! ```
//!
//! # Determinism
//!
//! This module stores deterministic execution requirements and explicit seeds.
//! It never creates randomness and never claims that an execution is
//! deterministic merely because a deterministic flag was requested.
//!
//! The executor is responsible for proving/reporting that the deterministic
//! contract was actually satisfied.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! No nightly features.
//! No external dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroU64;
use std::time::Duration;

use super::error::{AlgorithmError, AlgorithmResource, Result};

// ============================================================================
// Global contract limits
// ============================================================================

/// Maximum number of classical algorithm parameters.
pub const DEFAULT_MAX_PARAMETERS: u64 = 1_000_000;

/// Maximum number of logical qubits accepted by the algorithm layer.
pub const DEFAULT_MAX_QUBITS: u64 = 1_000_000;

/// Maximum generated logical gate count.
pub const DEFAULT_MAX_GATES: u64 = 10_000_000;

/// Maximum logical circuit depth.
pub const DEFAULT_MAX_DEPTH: u64 = 1_000_000;

/// Maximum optimizer iterations.
pub const DEFAULT_MAX_ITERATIONS: u64 = 1_000_000;

/// Maximum objective evaluations.
pub const DEFAULT_MAX_OBJECTIVE_EVALUATIONS: u64 = 10_000_000;

/// Maximum gradient evaluations.
pub const DEFAULT_MAX_GRADIENT_EVALUATIONS: u64 = 10_000_000;

/// Maximum logical circuit executions.
pub const DEFAULT_MAX_CIRCUIT_EXECUTIONS: u64 = 10_000_000;

/// Maximum measurement shots.
pub const DEFAULT_MAX_SHOTS: u64 = 1_000_000_000;

/// Maximum optimizer steps.
pub const DEFAULT_MAX_OPTIMIZER_STEPS: u64 = 10_000_000;

/// Maximum parameter absolute magnitude.
pub const DEFAULT_MAX_PARAMETER_MAGNITUDE: f64 = 1.0e12;

/// Maximum backend identifier length in UTF-8 bytes.
pub const MAX_BACKEND_ID_BYTES: usize = 256;

/// Maximum backend-version length in UTF-8 bytes.
pub const MAX_BACKEND_VERSION_BYTES: usize = 128;

/// Maximum implementation identifier length.
pub const MAX_IMPLEMENTATION_BYTES: usize = 1024;

/// Maximum algorithm metadata string length.
pub const MAX_METADATA_BYTES: usize = 1024;

/// Maximum measurement-state key length.
pub const MAX_MEASUREMENT_KEY_BYTES: usize = 1_048_576;

/// Maximum digest string length.
pub const MAX_DIGEST_BYTES: usize = 512;

// ============================================================================
// Algorithm identity
// ============================================================================

/// Stable identifier for a quantum algorithm family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlgorithmId {
    /// Generic variational algorithm orchestration.
    Variational,

    /// Variational Quantum Eigensolver.
    Vqe,

    /// Quantum Approximate Optimization Algorithm.
    Qaoa,

    /// Grover search.
    Grover,

    /// Amplitude amplification.
    AmplitudeAmplification,

    /// Amplitude estimation.
    AmplitudeEstimation,

    /// Quantum Phase Estimation.
    PhaseEstimation,
}

impl AlgorithmId {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Variational => "variational",
            Self::Vqe => "vqe",
            Self::Qaoa => "qaoa",
            Self::Grover => "grover",
            Self::AmplitudeAmplification => "amplitude_amplification",
            Self::AmplitudeEstimation => "amplitude_estimation",
            Self::PhaseEstimation => "phase_estimation",
        }
    }
}

impl fmt::Display for AlgorithmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Algorithm version
// ============================================================================

/// Semantic version of an algorithm contract or implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlgorithmVersion {
    /// Breaking semantic/API changes.
    pub major: u16,

    /// Backward-compatible functionality.
    pub minor: u16,

    /// Backward-compatible bug fixes.
    pub patch: u16,
}

impl AlgorithmVersion {
    /// Creates a semantic version.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Initial production algorithm contract.
    #[must_use]
    pub const fn initial() -> Self {
        Self::new(1, 0, 0)
    }
}

impl Default for AlgorithmVersion {
    fn default() -> Self {
        Self::initial()
    }
}

impl fmt::Display for AlgorithmVersion {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// Algorithm metadata
// ============================================================================

/// Stable metadata identifying an algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmMetadata {
    /// Algorithm family.
    pub algorithm: AlgorithmId,

    /// Algorithm contract/implementation version.
    pub version: AlgorithmVersion,

    /// Optional implementation identifier.
    pub implementation: Option<String>,
}

impl AlgorithmMetadata {
    /// Creates metadata for an algorithm.
    #[must_use]
    pub const fn new(
        algorithm: AlgorithmId,
        version: AlgorithmVersion,
    ) -> Self {
        Self {
            algorithm,
            version,
            implementation: None,
        }
    }

    /// Associates an implementation identifier.
    pub fn with_implementation<S: Into<String>>(
        mut self,
        implementation: S,
    ) -> Result<Self> {
        let implementation = implementation.into();

        validate_text(
            &implementation,
            MAX_IMPLEMENTATION_BYTES,
            "implementation",
        )?;

        self.implementation = Some(implementation);

        Ok(self)
    }

    /// Returns the stable algorithm identifier.
    #[must_use]
    pub const fn algorithm_id(&self) -> AlgorithmId {
        self.algorithm
    }

    /// Returns the algorithm version.
    #[must_use]
    pub const fn version(&self) -> AlgorithmVersion {
        self.version
    }
}

// ============================================================================
// Strongly typed scalar values
// ============================================================================

/// Number of logical/algorithm-visible qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QubitCount(NonZeroU64);

impl QubitCount {
    /// Creates a validated positive qubit count.
    pub fn new(value: u64) -> Result<Self> {
        let non_zero = NonZeroU64::new(value).ok_or_else(|| {
            AlgorithmError::InvalidQubitCount {
                count: 0,
                message: "qubit count must be greater than zero".to_string(),
            }
        })?;

        if value > DEFAULT_MAX_QUBITS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Qubits,
                requested: u128::from(value),
                limit: u128::from(DEFAULT_MAX_QUBITS),
                message: format!(
                    "qubit count exceeds global maximum {}",
                    DEFAULT_MAX_QUBITS
                ),
            });
        }

        Ok(Self(non_zero))
    }

    /// Returns the number of qubits.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for QubitCount {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Number of measurement shots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShotCount(NonZeroU64);

impl ShotCount {
    /// Creates a validated positive shot count.
    pub fn new(value: u64) -> Result<Self> {
        let non_zero = NonZeroU64::new(value).ok_or_else(|| {
            AlgorithmError::InvalidConfiguration {
                field: "shots".to_string(),
                message: "shot count must be greater than zero".to_string(),
            }
        })?;

        if value > DEFAULT_MAX_SHOTS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Shots,
                requested: u128::from(value),
                limit: u128::from(DEFAULT_MAX_SHOTS),
                message: format!(
                    "shot count exceeds global maximum {}",
                    DEFAULT_MAX_SHOTS
                ),
            });
        }

        Ok(Self(non_zero))
    }

    /// Returns the shot count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for ShotCount {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Stable explicit random seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seed(u64);

impl Seed {
    /// Creates a seed.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw seed.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// Typed classical parameter index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterIndex(usize);

impl ParameterIndex {
    /// Creates an index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the index.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

/// Probability constrained to `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Creates a validated probability.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(AlgorithmError::NonFiniteValue {
                context: "probability".to_string(),
                index: None,
                value,
                message: "probability must be finite".to_string(),
            });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(AlgorithmError::InvalidParameter {
                index: None,
                value: Some(value),
                message: format!(
                    "probability must be within [0, 1], got {value}"
                ),
            });
        }

        Ok(Self(value))
    }

    /// Returns the probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Probability {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite expectation value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ExpectationValue(f64);

impl ExpectationValue {
    /// Creates a finite expectation value.
    pub fn new(value: f64) -> Result<Self> {
        validate_finite_scalar(
            value,
            "expectation_value",
        )?;

        Ok(Self(value))
    }

    /// Returns the value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ExpectationValue {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite objective value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ObjectiveValue(f64);

impl ObjectiveValue {
    /// Creates a finite objective value.
    pub fn new(value: f64) -> Result<Self> {
        validate_finite_scalar(
            value,
            "objective",
        )?;

        Ok(Self(value))
    }

    /// Returns the value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ObjectiveValue {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite energy estimate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy(f64);

impl Energy {
    /// Creates a finite energy estimate.
    pub fn new(value: f64) -> Result<Self> {
        validate_finite_scalar(
            value,
            "energy",
        )?;

        Ok(Self(value))
    }

    /// Returns the value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Energy {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.get().fmt(f)
    }
}

// ============================================================================
// Parameter vector
// ============================================================================

/// Canonical classical parameter vector used by variational algorithms.
///
/// The vector owns concrete optimizer values. Every stored value is required
/// to be finite and bounded.
///
/// Empty vectors are allowed because some quantum algorithms have no classical
/// parameters. Algorithms requiring parameters must explicitly call
/// [`ParameterVector::require_non_empty`].
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterVector {
    values: Vec<f64>,
}

impl ParameterVector {
    /// Creates a validated parameter vector.
    pub fn new(values: Vec<f64>) -> Result<Self> {
        let length = u64::try_from(values.len()).map_err(|_| {
            AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: values.len() as u128,
                limit: u128::from(DEFAULT_MAX_PARAMETERS),
                message: "parameter vector length cannot be represented"
                    .to_string(),
            }
        })?;

        if length > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: u128::from(length),
                limit: u128::from(DEFAULT_MAX_PARAMETERS),
                message: format!(
                    "parameter vector exceeds maximum {}",
                    DEFAULT_MAX_PARAMETERS
                ),
            });
        }

        for (index, value) in values.iter().copied().enumerate() {
            validate_parameter(index, value)?;
        }

        Ok(Self { values })
    }

    /// Creates a zero-valued parameter vector.
    pub fn zeros(count: usize) -> Result<Self> {
        let count_u64 = u64::try_from(count).map_err(|_| {
            AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: count as u128,
                limit: u128::from(DEFAULT_MAX_PARAMETERS),
                message: "parameter count cannot be represented".to_string(),
            }
        })?;

        if count_u64 > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: u128::from(count_u64),
                limit: u128::from(DEFAULT_MAX_PARAMETERS),
                message: format!(
                    "parameter count exceeds maximum {}",
                    DEFAULT_MAX_PARAMETERS
                ),
            });
        }

        Self::new(vec![0.0; count])
    }

    /// Creates a vector filled with one validated value.
    pub fn filled(
        count: usize,
        value: f64,
    ) -> Result<Self> {
        validate_parameter(0, value)?;
        Self::new(vec![value; count])
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Requires at least one parameter.
    pub fn require_non_empty(&self) -> Result<()> {
        if self.values.is_empty() {
            return Err(AlgorithmError::InvalidInput {
                message: "parameter vector cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    /// Returns immutable parameter storage.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    /// Returns one parameter safely.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Returns one typed parameter safely.
    #[must_use]
    pub fn get_indexed(
        &self,
        index: ParameterIndex,
    ) -> Option<f64> {
        self.get(index.get())
    }

    /// Sets one parameter after validating the new value.
    pub fn set(
        &mut self,
        index: usize,
        value: f64,
    ) -> Result<()> {
        validate_parameter(index, value)?;

        let slot = self.values.get_mut(index).ok_or_else(|| {
            AlgorithmError::InvalidParameter {
                index: Some(index),
                value: Some(value),
                message: "parameter index is out of bounds".to_string(),
            }
        })?;

        *slot = value;

        Ok(())
    }

    /// Sets one typed parameter.
    pub fn set_indexed(
        &mut self,
        index: ParameterIndex,
        value: f64,
    ) -> Result<()> {
        self.set(index.get(), value)
    }

    /// Returns an owned copy with one parameter replaced.
    pub fn with_value(
        &self,
        index: usize,
        value: f64,
    ) -> Result<Self> {
        let mut result = self.clone();
        result.set(index, value)?;
        Ok(result)
    }

    /// Validates the complete vector.
    pub fn validate(&self) -> Result<()> {
        if self.values.len() as u64 > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: self.values.len() as u128,
                limit: u128::from(DEFAULT_MAX_PARAMETERS),
                message: "parameter vector exceeds configured maximum"
                    .to_string(),
            });
        }

        for (index, value) in self.values.iter().copied().enumerate() {
            validate_parameter(index, value)?;
        }

        Ok(())
    }

    /// Returns an iterator over parameter values.
    #[must_use]
    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.values.iter()
    }

    /// Returns the largest absolute parameter magnitude.
    #[must_use]
    pub fn max_abs(&self) -> f64 {
        self.values
            .iter()
            .copied()
            .map(f64::abs)
            .fold(0.0, f64::max)
    }
}

impl IntoIterator for ParameterVector {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

// ============================================================================
// Algorithm resource limits
// ============================================================================

/// Hard resource limits for one algorithm invocation.
///
/// These are safety boundaries. They are not performance hints.
///
/// Backend-specific limits may be lower but must never silently permit the
/// algorithm to exceed these limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmLimits {
    /// Maximum logical qubits.
    pub max_qubits: u64,

    /// Maximum logical gate count.
    pub max_gates: u64,

    /// Maximum logical circuit depth.
    pub max_depth: u64,

    /// Maximum algorithm iterations.
    pub max_iterations: u64,

    /// Maximum objective evaluations.
    pub max_objective_evaluations: u64,

    /// Maximum gradient evaluations.
    pub max_gradient_evaluations: u64,

    /// Maximum circuit executions.
    pub max_circuit_executions: u64,

    /// Maximum measurement shots.
    pub max_shots: u64,

    /// Maximum classical parameters.
    pub max_parameters: u64,

    /// Maximum optimizer steps.
    pub max_optimizer_steps: u64,
}

impl Default for AlgorithmLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl AlgorithmLimits {
    /// Returns the default production limit set.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_gates: DEFAULT_MAX_GATES,
            max_depth: DEFAULT_MAX_DEPTH,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            max_objective_evaluations:
                DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
            max_gradient_evaluations:
                DEFAULT_MAX_GRADIENT_EVALUATIONS,
            max_circuit_executions:
                DEFAULT_MAX_CIRCUIT_EXECUTIONS,
            max_shots: DEFAULT_MAX_SHOTS,
            max_parameters: DEFAULT_MAX_PARAMETERS,
            max_optimizer_steps:
                DEFAULT_MAX_OPTIMIZER_STEPS,
        }
    }

    /// Validates all configured limits.
    pub fn validate(&self) -> Result<()> {
        validate_limit(
            AlgorithmResource::Qubits,
            self.max_qubits,
            DEFAULT_MAX_QUBITS,
        )?;

        validate_limit(
            AlgorithmResource::Gates,
            self.max_gates,
            DEFAULT_MAX_GATES,
        )?;

        validate_limit(
            AlgorithmResource::CircuitDepth,
            self.max_depth,
            DEFAULT_MAX_DEPTH,
        )?;

        validate_limit(
            AlgorithmResource::Iterations,
            self.max_iterations,
            DEFAULT_MAX_ITERATIONS,
        )?;

        validate_limit(
            AlgorithmResource::ObjectiveEvaluations,
            self.max_objective_evaluations,
            DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
        )?;

        validate_limit(
            AlgorithmResource::GradientEvaluations,
            self.max_gradient_evaluations,
            DEFAULT_MAX_GRADIENT_EVALUATIONS,
        )?;

        validate_limit(
            AlgorithmResource::CircuitExecutions,
            self.max_circuit_executions,
            DEFAULT_MAX_CIRCUIT_EXECUTIONS,
        )?;

        validate_limit(
            AlgorithmResource::Shots,
            self.max_shots,
            DEFAULT_MAX_SHOTS,
        )?;

        validate_limit(
            AlgorithmResource::Parameters,
            self.max_parameters,
            DEFAULT_MAX_PARAMETERS,
        )?;

        validate_limit(
            AlgorithmResource::OptimizerSteps,
            self.max_optimizer_steps,
            DEFAULT_MAX_OPTIMIZER_STEPS,
        )?;

        Ok(())
    }

    /// Returns the configured limit for a resource.
    #[must_use]
    pub const fn limit_for(
        &self,
        resource: AlgorithmResource,
    ) -> Option<u64> {
        match resource {
            AlgorithmResource::Qubits => Some(self.max_qubits),
            AlgorithmResource::Gates => Some(self.max_gates),
            AlgorithmResource::CircuitDepth => Some(self.max_depth),
            AlgorithmResource::Shots => Some(self.max_shots),
            AlgorithmResource::Iterations => Some(self.max_iterations),
            AlgorithmResource::ObjectiveEvaluations => {
                Some(self.max_objective_evaluations)
            }
            AlgorithmResource::GradientEvaluations => {
                Some(self.max_gradient_evaluations)
            }
            AlgorithmResource::CircuitExecutions => {
                Some(self.max_circuit_executions)
            }
            AlgorithmResource::OptimizerSteps => {
                Some(self.max_optimizer_steps)
            }
            AlgorithmResource::Parameters => {
                Some(self.max_parameters)
            }
            AlgorithmResource::MemoryBytes
            | AlgorithmResource::Time
            | AlgorithmResource::Custom => None,
        }
    }

    /// Checks a resource request against this limit set.
    pub fn check(
        &self,
        resource: AlgorithmResource,
        requested: u64,
    ) -> Result<()> {
        let Some(limit) = self.limit_for(resource) else {
            return Ok(());
        };

        if requested > limit {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource,
                requested: u128::from(requested),
                limit: u128::from(limit),
                message: format!(
                    "{} requested {}, limit {}",
                    resource.as_str(),
                    requested,
                    limit
                ),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Execution configuration
// ============================================================================

/// Configuration controlling one algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionConfig {
    /// Requested measurement shots.
    ///
    /// `None` means the operation does not require sampling at this
    /// abstraction boundary.
    pub shots: Option<ShotCount>,

    /// Explicit execution randomness seed.
    pub seed: Option<Seed>,

    /// Explicit optimizer/classical randomness seed.
    pub optimization_seed: Option<Seed>,

    /// Requires reproducible execution behavior.
    ///
    /// This is a requirement, not proof that execution is deterministic.
    pub deterministic: bool,

    /// Resource safety limits.
    pub limits: AlgorithmLimits,

    /// Optional wall-clock timeout.
    ///
    /// Timing/enforcement belongs to the execution/algorithm layer.
    pub timeout: Option<Duration>,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            shots: None,
            seed: None,
            optimization_seed: None,
            deterministic: true,
            limits: AlgorithmLimits::production(),
            timeout: None,
        }
    }
}

impl ExecutionConfig {
    /// Creates the default deterministic contract.
    #[must_use]
    pub fn deterministic() -> Self {
        Self::default()
    }

    /// Creates a configuration that permits stochastic execution.
    #[must_use]
    pub fn nondeterministic() -> Self {
        Self {
            deterministic: false,
            ..Self::default()
        }
    }

    /// Sets the execution seed.
    #[must_use]
    pub const fn with_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the optimizer seed.
    #[must_use]
    pub const fn with_optimization_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.optimization_seed = Some(seed);
        self
    }

    /// Sets a measurement-shot count.
    pub fn with_shots(
        mut self,
        shots: ShotCount,
    ) -> Result<Self> {
        self.limits.check(
            AlgorithmResource::Shots,
            shots.get(),
        )?;

        self.shots = Some(shots);

        Ok(self)
    }

    /// Replaces the resource limits.
    pub fn with_limits(
        mut self,
        limits: AlgorithmLimits,
    ) -> Result<Self> {
        limits.validate()?;

        if let Some(shots) = self.shots {
            limits.check(
                AlgorithmResource::Shots,
                shots.get(),
            )?;
        }

        self.limits = limits;

        Ok(self)
    }

    /// Sets an optional timeout.
    ///
    /// A zero timeout is rejected because it would make the configuration
    /// unusable for normal execution.
    pub fn with_timeout(
        mut self,
        timeout: Duration,
    ) -> Result<Self> {
        if timeout.is_zero() {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "timeout".to_string(),
                message: "timeout must be greater than zero".to_string(),
            });
        }

        self.timeout = Some(timeout);

        Ok(self)
    }

    /// Validates the complete execution configuration.
    pub fn validate(&self) -> Result<()> {
        self.limits.validate()?;

        if let Some(shots) = self.shots {
            self.limits.check(
                AlgorithmResource::Shots,
                shots.get(),
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Measurement counts
// ============================================================================

/// Deterministic measurement-count collection.
///
/// A `BTreeMap` is used deliberately so iteration order is stable and
/// reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasurementCounts {
    counts: BTreeMap<String, u64>,
}

impl MeasurementCounts {
    /// Creates an empty collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates validated counts from a map.
    pub fn from_map(
        counts: BTreeMap<String, u64>,
    ) -> Result<Self> {
        let mut result = Self::new();

        for (state, count) in counts {
            result.insert(state, count)?;
        }

        Ok(result)
    }

    /// Inserts or replaces one measurement-state count.
    pub fn insert<S: Into<String>>(
        &mut self,
        state: S,
        count: u64,
    ) -> Result<()> {
        let state = state.into();

        validate_measurement_state(&state)?;

        if count == 0 {
            return Err(AlgorithmError::InvalidInput {
                message: format!(
                    "measurement state '{state}' has zero count"
                ),
            });
        }

        self.counts.insert(state, count);

        Ok(())
    }

    /// Returns a count for one measurement state.
    #[must_use]
    pub fn get(
        &self,
        state: &str,
    ) -> Option<u64> {
        self.counts.get(state).copied()
    }

    /// Returns the number of distinct states.
    #[must_use]
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns whether there are no states.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Returns the total number of represented shots.
    ///
    /// Overflow is reported as an algorithm error rather than silently
    /// saturating or wrapping.
    pub fn total_shots(&self) -> Result<u64> {
        self.counts.values().try_fold(
            0u64,
            |total, count| {
                total.checked_add(*count).ok_or_else(|| {
                    AlgorithmError::ResourceLimitExceeded {
                        resource: AlgorithmResource::Shots,
                        requested: u128::from(u64::MAX)
                            + 1,
                        limit: u128::from(u64::MAX),
                        message:
                            "measurement shot count overflow".to_string(),
                    }
                })
            },
        )
    }

    /// Returns deterministic state/count iteration.
    #[must_use]
    pub fn iter(
        &self,
    ) -> std::collections::btree_map::Iter<'_, String, u64> {
        self.counts.iter()
    }

    /// Validates all measurement states and counts.
    pub fn validate(&self) -> Result<()> {
        for (state, count) in &self.counts {
            validate_measurement_state(state)?;

            if *count == 0 {
                return Err(AlgorithmError::InvalidInput {
                    message: format!(
                        "measurement state '{state}' has zero count"
                    ),
                });
            }
        }

        let _ = self.total_shots()?;

        Ok(())
    }

    /// Returns the most frequently observed state.
    ///
    /// Ties are resolved lexicographically in a deterministic manner.
    #[must_use]
    pub fn most_likely(
        &self,
    ) -> Option<(&str, u64)> {
        self.counts
            .iter()
            .max_by(
                |(state_a, count_a), (state_b, count_b)| {
                    count_a
                        .cmp(count_b)
                        .then_with(|| state_b.cmp(state_a))
                },
            )
            .map(|(state, count)| {
                (state.as_str(), *count)
            })
    }
}

// ============================================================================
// Execution metadata
// ============================================================================

/// Backend-neutral metadata describing one execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionMetadata {
    /// Stable backend identifier.
    pub backend_id: String,

    /// Backend implementation version.
    pub backend_version: Option<String>,

    /// Number of shots actually executed.
    pub shots: Option<ShotCount>,

    /// Actual execution seed, when one was used.
    pub seed: Option<Seed>,

    /// Whether the backend explicitly reports deterministic execution.
    ///
    /// This must be `true` before a deterministic request can be considered
    /// satisfied.
    pub deterministic: bool,
}

impl ExecutionMetadata {
    /// Creates backend-neutral metadata.
    pub fn new<S: Into<String>>(
        backend_id: S,
        deterministic: bool,
    ) -> Result<Self> {
        let backend_id = backend_id.into();

        validate_text(
            &backend_id,
            MAX_BACKEND_ID_BYTES,
            "backend_id",
        )?;

        Ok(Self {
            backend_id,
            backend_version: None,
            shots: None,
            seed: None,
            deterministic,
        })
    }

    /// Associates a backend version.
    pub fn with_backend_version<S: Into<String>>(
        mut self,
        version: S,
    ) -> Result<Self> {
        let version = version.into();

        validate_text(
            &version,
            MAX_BACKEND_VERSION_BYTES,
            "backend_version",
        )?;

        self.backend_version = Some(version);

        Ok(self)
    }

    /// Associates actual shot accounting.
    #[must_use]
    pub const fn with_shots(
        mut self,
        shots: ShotCount,
    ) -> Self {
        self.shots = Some(shots);
        self
    }

    /// Associates the actual execution seed.
    #[must_use]
    pub const fn with_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Validates metadata.
    pub fn validate(&self) -> Result<()> {
        validate_text(
            &self.backend_id,
            MAX_BACKEND_ID_BYTES,
            "backend_id",
        )?;

        if let Some(version) = &self.backend_version {
            validate_text(
                version,
                MAX_BACKEND_VERSION_BYTES,
                "backend_version",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Replay/provenance digests
// ============================================================================

/// Stable digest metadata used by replay and provenance systems.
///
/// `types.rs` stores the values but deliberately does not calculate hashes.
/// Canonical serialization and hashing belong to the replay/provenance layer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionDigests {
    /// Digest of the canonical algorithm input.
    pub input_digest: Option<String>,

    /// Digest of the canonical logical circuit.
    pub circuit_digest: Option<String>,

    /// Digest of the canonical algorithm result.
    pub result_digest: Option<String>,
}

impl ExecutionDigests {
    /// Creates empty digest metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            input_digest: None,
            circuit_digest: None,
            result_digest: None,
        }
    }

    /// Adds an input digest.
    pub fn with_input_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Result<Self> {
        let digest = digest.into();

        validate_digest(
            &digest,
            "input_digest",
        )?;

        self.input_digest = Some(digest);

        Ok(self)
    }

    /// Adds a circuit digest.
    pub fn with_circuit_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Result<Self> {
        let digest = digest.into();

        validate_digest(
            &digest,
            "circuit_digest",
        )?;

        self.circuit_digest = Some(digest);

        Ok(self)
    }

    /// Adds a result digest.
    pub fn with_result_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Result<Self> {
        let digest = digest.into();

        validate_digest(
            &digest,
            "result_digest",
        )?;

        self.result_digest = Some(digest);

        Ok(self)
    }

    /// Validates all supplied digests.
    pub fn validate(&self) -> Result<()> {
        if let Some(digest) = &self.input_digest {
            validate_digest(
                digest,
                "input_digest",
            )?;
        }

        if let Some(digest) = &self.circuit_digest {
            validate_digest(
                digest,
                "circuit_digest",
            )?;
        }

        if let Some(digest) = &self.result_digest {
            validate_digest(
                digest,
                "result_digest",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_finite_scalar(
    value: f64,
    context: &'static str,
) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue {
            context: context.to_string(),
            index: None,
            value,
            message: format!(
                "{context} must be finite"
            ),
        });
    }

    Ok(())
}

fn validate_parameter(
    index: usize,
    value: f64,
) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue {
            context: "parameter".to_string(),
            index: Some(index),
            value,
            message: format!(
                "parameter[{index}] must be finite"
            ),
        });
    }

    if value.abs() > DEFAULT_MAX_PARAMETER_MAGNITUDE {
        return Err(AlgorithmError::InvalidParameter {
            index: Some(index),
            value: Some(value),
            message: format!(
                "parameter[{index}] exceeds maximum absolute magnitude {}",
                DEFAULT_MAX_PARAMETER_MAGNITUDE
            ),
        });
    }

    Ok(())
}

fn validate_limit(
    resource: AlgorithmResource,
    value: u64,
    global_maximum: u64,
) -> Result<()> {
    if value == 0 {
        return Err(AlgorithmError::InvalidConfiguration {
            field: resource.as_str().to_string(),
            message: "resource limit must be greater than zero".to_string(),
        });
    }

    if value > global_maximum {
        return Err(AlgorithmError::InvalidConfiguration {
            field: resource.as_str().to_string(),
            message: format!(
                "limit {} exceeds global maximum {}",
                value,
                global_maximum
            ),
        });
    }

    Ok(())
}

fn validate_text(
    value: &str,
    maximum_bytes: usize,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(AlgorithmError::InvalidInput {
            message: format!(
                "{field} cannot be empty"
            ),
        });
    }

    if value.len() > maximum_bytes {
        return Err(AlgorithmError::InvalidInput {
            message: format!(
                "{field} exceeds maximum UTF-8 length of {} bytes",
                maximum_bytes
            ),
        });
    }

    Ok(())
}

fn validate_measurement_state(
    state: &str,
) -> Result<()> {
    if state.is_empty() {
        return Err(AlgorithmError::InvalidInput {
            message:
                "measurement state cannot be empty".to_string(),
        });
    }

    if state.len() > MAX_MEASUREMENT_KEY_BYTES {
        return Err(AlgorithmError::InvalidInput {
            message: format!(
                "measurement state exceeds maximum length of {} bytes",
                MAX_MEASUREMENT_KEY_BYTES
            ),
        });
    }

    if !state
        .bytes()
        .all(|byte| byte == b'0' || byte == b'1')
    {
        return Err(AlgorithmError::InvalidInput {
            message:
                "measurement state must contain only '0' and '1'"
                    .to_string(),
        });
    }

    Ok(())
}

fn validate_digest(
    digest: &str,
    field: &'static str,
) -> Result<()> {
    validate_text(
        digest,
        MAX_DIGEST_BYTES,
        field,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithm_ids_are_stable() {
        assert_eq!(
            AlgorithmId::Vqe.as_str(),
            "vqe"
        );
        assert_eq!(
            AlgorithmId::Qaoa.as_str(),
            "qaoa"
        );
        assert_eq!(
            AlgorithmId::Grover.as_str(),
            "grover"
        );
    }

    #[test]
    fn algorithm_version_is_stable() {
        let version =
            AlgorithmVersion::new(1, 2, 3);

        assert_eq!(
            version.to_string(),
            "1.2.3"
        );
    }

    #[test]
    fn qubit_count_rejects_zero() {
        assert!(
            QubitCount::new(0).is_err()
        );
    }

    #[test]
    fn qubit_count_accepts_valid_value() {
        let count =
            QubitCount::new(32).unwrap();

        assert_eq!(
            count.get(),
            32
        );
    }

    #[test]
    fn shot_count_rejects_zero() {
        assert!(
            ShotCount::new(0).is_err()
        );
    }

    #[test]
    fn probability_is_bounded() {
        assert!(
            Probability::new(0.0).is_ok()
        );
        assert!(
            Probability::new(1.0).is_ok()
        );
        assert!(
            Probability::new(-0.1).is_err()
        );
        assert!(
            Probability::new(1.1).is_err()
        );
    }

    #[test]
    fn probability_rejects_non_finite_values() {
        assert!(
            Probability::new(f64::NAN).is_err()
        );
        assert!(
            Probability::new(f64::INFINITY).is_err()
        );
        assert!(
            Probability::new(f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn parameter_vector_preserves_order() {
        let parameters =
            ParameterVector::new(
                vec![1.0, 2.0, 3.0],
            )
            .unwrap();

        assert_eq!(
            parameters.as_slice(),
            &[1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn parameter_vector_allows_empty() {
        let parameters =
            ParameterVector::new(
                Vec::new(),
            )
            .unwrap();

        assert!(
            parameters.is_empty()
        );

        assert!(
            parameters
                .require_non_empty()
                .is_err()
        );
    }

    #[test]
    fn parameter_vector_rejects_nan() {
        assert!(
            ParameterVector::new(
                vec![f64::NAN]
            )
            .is_err()
        );
    }

    #[test]
    fn parameter_vector_rejects_infinity() {
        assert!(
            ParameterVector::new(
                vec![f64::INFINITY]
            )
            .is_err()
        );

        assert!(
            ParameterVector::new(
                vec![f64::NEG_INFINITY]
            )
            .is_err()
        );
    }

    #[test]
    fn parameter_vector_supports_safe_mutation() {
        let mut parameters =
            ParameterVector::new(
                vec![0.0, 1.0],
            )
            .unwrap();

        parameters
            .set(1, 2.0)
            .unwrap();

        assert_eq!(
            parameters.get(1),
            Some(2.0)
        );

        assert!(
            parameters
                .set(5, 1.0)
                .is_err()
        );

        assert!(
            parameters
                .set(0, f64::NAN)
                .is_err()
        );
    }

    #[test]
    fn default_limits_validate() {
        assert!(
            AlgorithmLimits::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn resource_limits_reject_zero() {
        let mut limits =
            AlgorithmLimits::default();

        limits.max_qubits = 0;

        assert!(
            limits.validate().is_err()
        );
    }

    #[test]
    fn resource_limits_reject_global_overflow() {
        let mut limits =
            AlgorithmLimits::default();

        limits.max_qubits =
            DEFAULT_MAX_QUBITS + 1;

        assert!(
            limits.validate().is_err()
        );
    }

    #[test]
    fn execution_is_deterministic_by_contract_default() {
        let config =
            ExecutionConfig::default();

        assert!(
            config.deterministic
        );

        assert!(
            config.validate().is_ok()
        );
    }

    #[test]
    fn deterministic_and_optimizer_seeds_are_independent() {
        let config =
            ExecutionConfig::deterministic()
                .with_seed(
                    Seed::new(42)
                )
                .with_optimization_seed(
                    Seed::new(7)
                );

        assert_eq!(
            config.seed,
            Some(Seed::new(42))
        );

        assert_eq!(
            config.optimization_seed,
            Some(Seed::new(7))
        );
    }

    #[test]
    fn timeout_zero_is_rejected() {
        assert!(
            ExecutionConfig::default()
                .with_timeout(
                    Duration::ZERO
                )
                .is_err()
        );
    }

    #[test]
    fn measurement_counts_are_deterministic() {
        let mut counts =
            MeasurementCounts::new();

        counts
            .insert("10", 3)
            .unwrap();

        counts
            .insert("01", 5)
            .unwrap();

        assert_eq!(
            counts.most_likely(),
            Some(("01", 5))
        );
    }

    #[test]
    fn measurement_counts_reject_invalid_state() {
        let mut counts =
            MeasurementCounts::new();

        assert!(
            counts
                .insert("02", 1)
                .is_err()
        );
    }

    #[test]
    fn measurement_counts_reject_zero_count() {
        let mut counts =
            MeasurementCounts::new();

        assert!(
            counts
                .insert("00", 0)
                .is_err()
        );
    }

    #[test]
    fn measurement_counts_total_shots_is_checked() {
        let mut counts =
            MeasurementCounts::new();

        counts
            .insert("00", 10)
            .unwrap();

        counts
            .insert("01", 20)
            .unwrap();

        assert_eq!(
            counts.total_shots().unwrap(),
            30
        );
    }

    #[test]
    fn execution_metadata_requires_backend_id() {
        assert!(
            ExecutionMetadata::new(
                "",
                true
            )
            .is_err()
        );
    }

    #[test]
    fn execution_metadata_accepts_backend_id() {
        let metadata =
            ExecutionMetadata::new(
                "reference-simulator",
                true,
            )
            .unwrap();

        assert_eq!(
            metadata.backend_id,
            "reference-simulator"
        );
        assert!(
            metadata.deterministic
        );
    }

    #[test]
    fn digest_metadata_validates_values() {
        let digests =
            ExecutionDigests::new()
                .with_input_digest(
                    "abc123"
                )
                .unwrap()
                .with_circuit_digest(
                    "def456"
                )
                .unwrap()
                .with_result_digest(
                    "ghi789"
                )
                .unwrap();

        assert!(
            digests.validate().is_ok()
        );
    }
}