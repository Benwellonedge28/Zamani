//! Zamani Quantum Algorithms — Shared Types and Contracts
//!
//! Production-grade, backend-independent data contracts shared by the
//! quantum-algorithm subsystem.
//!
//! # Architectural boundary
//!
//! This module owns algorithm-level data contracts:
//!
//! - algorithm identity and versioning;
//! - reproducibility metadata;
//! - classical optimization parameter vectors;
//! - strongly typed quantum-algorithm scalar values;
//! - execution configuration;
//! - resource limits;
//! - measurement-count representation;
//! - execution metadata;
//! - algorithm metadata;
//! - stable validation helpers.
//!
//! This module deliberately does NOT own:
//!
//! - quantum gates;
//! - quantum circuits;
//! - qubit topology;
//! - routing;
//! - transpilation;
//! - hardware;
//! - backend execution;
//! - error-correction;
//! - optimizer implementations;
//! - objective implementations.
//!
//! Those responsibilities belong to their respective Quantum IR, routing,
//! transpilation, backend, QEC, optimizer, and objective subsystems.
//!
//! # Important distinction
//!
//! [`ParameterVector`] represents classical algorithm/optimization parameters.
//!
//! It is intentionally different from `quantum::ir::Parameter`, which owns
//! circuit-level parameter semantics.
//!
//! The intended flow is:
//!
//! ```text
//! Algorithm parameter vector
//!          │
//!          ▼
//!       Ansatz
//!          │
//!          ▼
//!   quantum::ir::Parameter
//!          │
//!          ▼
//!    QuantumCircuit
//! ```
//!
//! # Determinism
//!
//! Randomness is never obtained from global mutable state. Algorithms and
//! executors receive explicit seeds through [`ExecutionConfig`].
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! No nightly features are required.
//!
//! # Dependency contract
//!
//! This module depends only on:
//!
//! ```text
//! algorithms::error
//! ```
//!
//! It must not depend on `execution.rs`, `objective.rs`, `optimizer.rs`,
//! `variational.rs`, or any concrete algorithm implementation.
//!
//! This makes `types.rs` the stable shared contract layer for the entire
//! algorithms subsystem.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroU64;
use std::time::Duration;

use super::error::{AlgorithmError, Result};

// =============================================================================
// Constants
// =============================================================================

/// Maximum number of classical algorithm parameters accepted by default.
///
/// This is deliberately finite so malformed or adversarial input cannot
/// allocate an unbounded parameter vector.
pub const DEFAULT_MAX_PARAMETERS: u64 = 1_000_000;

/// Maximum number of qubits accepted by the default algorithm contract.
///
/// Backends may impose lower limits. They must never silently exceed this
/// algorithm-level limit.
pub const DEFAULT_MAX_QUBITS: u64 = 1_000_000;

/// Maximum number of circuit gates an algorithm may request by default.
pub const DEFAULT_MAX_GATES: u64 = 10_000_000;

/// Maximum logical circuit depth an algorithm may request by default.
pub const DEFAULT_MAX_DEPTH: u64 = 1_000_000;

/// Maximum number of optimizer iterations by default.
pub const DEFAULT_MAX_ITERATIONS: u64 = 1_000_000;

/// Maximum number of objective evaluations by default.
pub const DEFAULT_MAX_OBJECTIVE_EVALUATIONS: u64 = 10_000_000;

/// Maximum number of gradient evaluations by default.
pub const DEFAULT_MAX_GRADIENT_EVALUATIONS: u64 = 10_000_000;

/// Maximum number of measurement shots by default.
pub const DEFAULT_MAX_SHOTS: u64 = 1_000_000_000;

/// Maximum parameter magnitude allowed by default.
///
/// Algorithms may impose stricter mathematical limits.
pub const DEFAULT_MAX_PARAMETER_MAGNITUDE: f64 = 1.0e12;

/// Maximum UTF-8 byte length of a backend identifier.
pub const MAX_BACKEND_ID_BYTES: usize = 256;

/// Maximum UTF-8 byte length of a backend version.
pub const MAX_BACKEND_VERSION_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an algorithm metadata field.
pub const MAX_METADATA_FIELD_BYTES: usize = 1024;

/// Maximum UTF-8 byte length of a measurement bitstring.
pub const MAX_MEASUREMENT_KEY_BYTES: usize = 1_048_576;

// =============================================================================
// Algorithm identity
// =============================================================================

/// Stable identifier for a quantum algorithm family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlgorithmId {
    /// Generic variational quantum algorithm orchestration.
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

// =============================================================================
// Algorithm version
// =============================================================================

/// Semantic version of an algorithm contract/implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlgorithmVersion {
    /// Breaking API/semantic changes.
    pub major: u16,

    /// Backward-compatible functionality.
    pub minor: u16,

    /// Backward-compatible bug fixes.
    pub patch: u16,
}

impl AlgorithmVersion {
    /// Creates a semantic version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Initial production contract version.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Algorithm metadata
// =============================================================================

/// Stable metadata identifying an algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmMetadata {
    /// Algorithm family.
    pub algorithm: AlgorithmId,

    /// Algorithm implementation/contract version.
    pub version: AlgorithmVersion,

    /// Optional implementation name.
    pub implementation: Option<String>,
}

impl AlgorithmMetadata {
    /// Creates metadata for an algorithm.
    pub fn new(
        algorithm: AlgorithmId,
        version: AlgorithmVersion,
    ) -> Self {
        Self {
            algorithm,
            version,
            implementation: None,
        }
    }

    /// Adds an implementation identifier.
    pub fn with_implementation<S: Into<String>>(
        mut self,
        implementation: S,
    ) -> Result<Self> {
        let implementation = implementation.into();

        validate_text_field(
            &implementation,
            MAX_METADATA_FIELD_BYTES,
            "implementation",
        )?;

        self.implementation = Some(implementation);

        Ok(self)
    }
}

// =============================================================================
// Strongly typed scalar values
// =============================================================================

/// Number of logical/algorithm-visible qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QubitCount(NonZeroU64);

impl QubitCount {
    /// Creates a positive qubit count.
    pub fn new(value: u64) -> Result<Self> {
        let value = NonZeroU64::new(value).ok_or_else(|| {
            AlgorithmError::InvalidQubitCount {
                value,
            }
        })?;

        if value.get() > DEFAULT_MAX_QUBITS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "qubits".to_string(),
                requested: value.get(),
                limit: DEFAULT_MAX_QUBITS,
            });
        }

        Ok(Self(value))
    }

    /// Returns the count as `u64`.
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for QubitCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Number of measurement shots.
///
/// A shot count must always be positive. Algorithms that do not require
/// sampling should use `ExecutionConfig::shots = None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShotCount(NonZeroU64);

impl ShotCount {
    /// Creates a positive shot count.
    pub fn new(value: u64) -> Result<Self> {
        let value = NonZeroU64::new(value).ok_or_else(|| {
            AlgorithmError::InvalidConfiguration {
                field: "shots".to_string(),
                reason: "shot count must be greater than zero".to_string(),
            }
        })?;

        if value.get() > DEFAULT_MAX_SHOTS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "shots".to_string(),
                requested: value.get(),
                limit: DEFAULT_MAX_SHOTS,
            });
        }

        Ok(Self(value))
    }

    /// Returns the shot count.
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for ShotCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Stable random seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seed(u64);

impl Seed {
    /// Creates a seed.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the seed.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// Index of a classical algorithm parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterIndex(usize);

impl ParameterIndex {
    /// Creates an index.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the index.
    pub const fn get(self) -> usize {
        self.0
    }
}

/// A finite probability in the closed interval `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Creates a validated probability.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(AlgorithmError::NonFiniteValue {
                field: "probability".to_string(),
                value,
            });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(AlgorithmError::InvalidParameter {
                name: "probability".to_string(),
                reason: format!(
                    "must be in [0, 1], got {value}"
                ),
            });
        }

        Ok(Self(value))
    }

    /// Returns the probability.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite expectation value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ExpectationValue(f64);

impl ExpectationValue {
    /// Creates a finite expectation value.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(AlgorithmError::NonFiniteValue {
                field: "expectation_value".to_string(),
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the expectation value.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ExpectationValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite objective value.
///
/// The algorithms subsystem assumes minimization unless an algorithm
/// explicitly defines another optimization direction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ObjectiveValue(f64);

impl ObjectiveValue {
    /// Creates a finite objective value.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(AlgorithmError::NonFiniteValue {
                field: "objective".to_string(),
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the objective value.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ObjectiveValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Finite energy estimate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy(f64);

impl Energy {
    /// Creates a finite energy.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(AlgorithmError::NonFiniteValue {
                field: "energy".to_string(),
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the energy.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

// =============================================================================
// Parameter vector
// =============================================================================

/// Canonical classical parameter vector used by variational algorithms.
///
/// This type is deliberately separate from `quantum::ir::Parameter`.
///
/// Unlike the IR parameter abstraction, this vector represents concrete
/// numerical optimizer state. Every stored value must therefore be finite.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterVector {
    values: Vec<f64>,
}

impl ParameterVector {
    /// Creates a parameter vector.
    ///
    /// Empty vectors are allowed because some algorithms do not require
    /// classical parameters. Algorithms that require at least one parameter
    /// must call [`Self::require_non_empty`].
    pub fn new(values: Vec<f64>) -> Result<Self> {
        if values.len() as u64 > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "parameters".to_string(),
                requested: values.len() as u64,
                limit: DEFAULT_MAX_PARAMETERS,
            });
        }

        for (index, value) in values.iter().copied().enumerate() {
            validate_finite_parameter(index, value)?;
        }

        Ok(Self {
            values,
        })
    }

    /// Creates a vector of zero-valued parameters.
    pub fn zeros(count: usize) -> Result<Self> {
        if count as u64 > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "parameters".to_string(),
                requested: count as u64,
                limit: DEFAULT_MAX_PARAMETERS,
            });
        }

        Self::new(vec![0.0; count])
    }

    /// Creates a vector filled with the same finite value.
    pub fn filled(count: usize, value: f64) -> Result<Self> {
        validate_parameter_magnitude(value)?;

        Self::new(vec![value; count])
    }

    /// Returns the number of parameters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no parameters.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Requires at least one parameter.
    pub fn require_non_empty(&self) -> Result<()> {
        if self.is_empty() {
            return Err(AlgorithmError::EmptyInput {
                what: "parameter vector".to_string(),
            });
        }

        Ok(())
    }

    /// Returns the parameter values without allocation.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    /// Returns mutable parameter storage.
    ///
    /// Callers are responsible for preserving the finite-value invariant.
    /// Prefer [`Self::set`] for externally supplied values.
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        &mut self.values
    }

    /// Returns a parameter by index.
    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Returns a parameter by typed index.
    pub fn get_indexed(&self, index: ParameterIndex) -> Option<f64> {
        self.get(index.get())
    }

    /// Sets one parameter after validation.
    pub fn set(&mut self, index: usize, value: f64) -> Result<()> {
        validate_finite_parameter(index, value)?;

        let slot = self.values.get_mut(index).ok_or_else(|| {
            AlgorithmError::InvalidParameter {
                name: format!("parameter[{index}]"),
                reason: "parameter index is out of bounds".to_string(),
            }
        })?;

        *slot = value;

        Ok(())
    }

    /// Sets one typed parameter index.
    pub fn set_indexed(
        &mut self,
        index: ParameterIndex,
        value: f64,
    ) -> Result<()> {
        self.set(index.get(), value)
    }

    /// Validates every stored parameter.
    pub fn validate(&self) -> Result<()> {
        if self.values.len() as u64 > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "parameters".to_string(),
                requested: self.values.len() as u64,
                limit: DEFAULT_MAX_PARAMETERS,
            });
        }

        for (index, value) in self.values.iter().copied().enumerate() {
            validate_finite_parameter(index, value)?;
        }

        Ok(())
    }

    /// Returns an owned copy with one parameter changed.
    pub fn with_value(
        &self,
        index: usize,
        value: f64,
    ) -> Result<Self> {
        let mut result = self.clone();
        result.set(index, value)?;
        Ok(result)
    }

    /// Returns an iterator over parameters.
    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.values.iter()
    }

    /// Returns the maximum absolute parameter magnitude.
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

// =============================================================================
// Algorithm resource limits
// =============================================================================

/// Hard execution/resource limits applied to algorithm invocations.
///
/// These limits are safety boundaries, not performance hints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmLimits {
    /// Maximum logical qubits.
    pub max_qubits: u64,

    /// Maximum generated gate count.
    pub max_gates: u64,

    /// Maximum logical circuit depth.
    pub max_depth: u64,

    /// Maximum classical optimizer iterations.
    pub max_iterations: u64,

    /// Maximum objective evaluations.
    pub max_objective_evaluations: u64,

    /// Maximum gradient evaluations.
    pub max_gradient_evaluations: u64,

    /// Maximum measurement shots.
    pub max_shots: u64,

    /// Maximum number of classical parameters.
    pub max_parameters: u64,
}

impl Default for AlgorithmLimits {
    fn default() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_gates: DEFAULT_MAX_GATES,
            max_depth: DEFAULT_MAX_DEPTH,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            max_objective_evaluations:
                DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
            max_gradient_evaluations:
                DEFAULT_MAX_GRADIENT_EVALUATIONS,
            max_shots: DEFAULT_MAX_SHOTS,
            max_parameters: DEFAULT_MAX_PARAMETERS,
        }
    }
}

impl AlgorithmLimits {
    /// Creates a default production limit set.
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
            max_shots: DEFAULT_MAX_SHOTS,
            max_parameters: DEFAULT_MAX_PARAMETERS,
        }
    }

    /// Validates the limit set.
    pub fn validate(&self) -> Result<()> {
        validate_positive_limit(
            self.max_qubits,
            "max_qubits",
        )?;
        validate_positive_limit(
            self.max_gates,
            "max_gates",
        )?;
        validate_positive_limit(
            self.max_depth,
            "max_depth",
        )?;
        validate_positive_limit(
            self.max_iterations,
            "max_iterations",
        )?;
        validate_positive_limit(
            self.max_objective_evaluations,
            "max_objective_evaluations",
        )?;
        validate_positive_limit(
            self.max_gradient_evaluations,
            "max_gradient_evaluations",
        )?;
        validate_positive_limit(
            self.max_shots,
            "max_shots",
        )?;
        validate_positive_limit(
            self.max_parameters,
            "max_parameters",
        )?;

        if self.max_qubits > DEFAULT_MAX_QUBITS {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_qubits".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_QUBITS
                ),
            });
        }

        if self.max_gates > DEFAULT_MAX_GATES {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_gates".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_GATES
                ),
            });
        }

        if self.max_depth > DEFAULT_MAX_DEPTH {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_depth".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_DEPTH
                ),
            });
        }

        if self.max_iterations > DEFAULT_MAX_ITERATIONS {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_iterations".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_ITERATIONS
                ),
            });
        }

        if self.max_objective_evaluations
            > DEFAULT_MAX_OBJECTIVE_EVALUATIONS
        {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_objective_evaluations".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_OBJECTIVE_EVALUATIONS
                ),
            });
        }

        if self.max_gradient_evaluations
            > DEFAULT_MAX_GRADIENT_EVALUATIONS
        {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_gradient_evaluations".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_GRADIENT_EVALUATIONS
                ),
            });
        }

        if self.max_shots > DEFAULT_MAX_SHOTS {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_shots".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_SHOTS
                ),
            });
        }

        if self.max_parameters > DEFAULT_MAX_PARAMETERS {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "max_parameters".to_string(),
                reason: format!(
                    "cannot exceed global maximum {}",
                    DEFAULT_MAX_PARAMETERS
                ),
            });
        }

        Ok(())
    }

    /// Checks a requested resource against a named limit.
    pub fn check(
        &self,
        resource: ResourceKind,
        requested: u64,
    ) -> Result<()> {
        let limit = self.limit_for(resource);

        if requested > limit {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: resource.as_str().to_string(),
                requested,
                limit,
            });
        }

        Ok(())
    }

    /// Returns the configured limit for a resource.
    pub const fn limit_for(
        &self,
        resource: ResourceKind,
    ) -> u64 {
        match resource {
            ResourceKind::Qubits => self.max_qubits,
            ResourceKind::Gates => self.max_gates,
            ResourceKind::Depth => self.max_depth,
            ResourceKind::Iterations => self.max_iterations,
            ResourceKind::ObjectiveEvaluations => {
                self.max_objective_evaluations
            }
            ResourceKind::GradientEvaluations => {
                self.max_gradient_evaluations
            }
            ResourceKind::Shots => self.max_shots,
            ResourceKind::Parameters => self.max_parameters,
        }
    }
}

/// Resource category subject to an algorithm limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Qubits,
    Gates,
    Depth,
    Iterations,
    ObjectiveEvaluations,
    GradientEvaluations,
    Shots,
    Parameters,
}

impl ResourceKind {
    /// Returns a stable resource name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qubits => "qubits",
            Self::Gates => "gates",
            Self::Depth => "depth",
            Self::Iterations => "iterations",
            Self::ObjectiveEvaluations => "objective_evaluations",
            Self::GradientEvaluations => "gradient_evaluations",
            Self::Shots => "shots",
            Self::Parameters => "parameters",
        }
    }
}

// =============================================================================
// Execution configuration
// =============================================================================

/// Configuration controlling deterministic and resource-bounded execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionConfig {
    /// Number of measurement shots.
    ///
    /// `None` means that the execution mode does not require sampling.
    pub shots: Option<ShotCount>,

    /// Explicit execution seed.
    pub seed: Option<Seed>,

    /// Explicit optimizer/random-process seed.
    ///
    /// Keeping this separate from `seed` allows execution randomness and
    /// classical optimizer randomness to be reproduced independently.
    pub optimization_seed: Option<Seed>,

    /// Requires reproducible behavior.
    pub deterministic: bool,

    /// Maximum algorithm resource consumption.
    pub limits: AlgorithmLimits,

    /// Optional wall-clock execution limit.
    ///
    /// This is represented as a `Duration` internally and is interpreted by
    /// the execution layer. This module does not perform timing itself.
    pub timeout: Option<Duration>,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            shots: None,
            seed: None,
            optimization_seed: None,
            deterministic: true,
            limits: AlgorithmLimits::default(),
            timeout: None,
        }
    }
}

impl ExecutionConfig {
    /// Creates a deterministic production configuration.
    pub fn deterministic() -> Self {
        Self::default()
    }

    /// Creates a stochastic-capable configuration.
    pub fn nondeterministic() -> Self {
        Self {
            deterministic: false,
            ..Self::default()
        }
    }

    /// Sets the execution seed.
    pub const fn with_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the optimization seed.
    pub const fn with_optimization_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.optimization_seed = Some(seed);
        self
    }

    /// Sets the requested shot count.
    pub fn with_shots(
        mut self,
        shots: ShotCount,
    ) -> Result<Self> {
        if shots.get() > self.limits.max_shots {
            return Err(AlgorithmError::ResourceLimitExceeded {
                resource: "shots".to_string(),
                requested: shots.get(),
                limit: self.limits.max_shots,
            });
        }

        self.shots = Some(shots);

        Ok(self)
    }

    /// Sets algorithm resource limits.
    pub fn with_limits(
        mut self,
        limits: AlgorithmLimits,
    ) -> Result<Self> {
        limits.validate()?;

        if let Some(shots) = self.shots {
            if shots.get() > limits.max_shots {
                return Err(AlgorithmError::ResourceLimitExceeded {
                    resource: "shots".to_string(),
                    requested: shots.get(),
                    limit: limits.max_shots,
                });
            }
        }

        self.limits = limits;

        Ok(self)
    }

    /// Sets an optional timeout.
    pub const fn with_timeout(
        mut self,
        timeout: Duration,
    ) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Validates the complete execution configuration.
    pub fn validate(&self) -> Result<()> {
        self.limits.validate()?;

        if let Some(shots) = self.shots {
            self.limits.check(
                ResourceKind::Shots,
                shots.get(),
            )?;
        }

        if self.deterministic
            && self.seed.is_none()
        {
            // A deterministic execution does not strictly require a caller
            // supplied seed when the backend is itself deterministic. We
            // therefore do not reject this configuration.
        }

        Ok(())
    }
}

// =============================================================================
// Measurement counts
// =============================================================================

/// Deterministic measurement-count collection.
///
/// `BTreeMap` is intentionally used instead of `HashMap` so iteration order
/// is stable and reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasurementCounts {
    counts: BTreeMap<String, u64>,
}

impl MeasurementCounts {
    /// Creates an empty count collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates counts from a map after validation.
    pub fn from_map(
        counts: BTreeMap<String, u64>,
    ) -> Result<Self> {
        let mut result = Self::new();

        for (state, count) in counts {
            result.insert(state, count)?;
        }

        Ok(result)
    }

    /// Inserts/replaces a measurement count.
    pub fn insert<S: Into<String>>(
        &mut self,
        state: S,
        count: u64,
    ) -> Result<()> {
        let state = state.into();

        validate_measurement_key(&state)?;

        self.counts.insert(state, count);

        Ok(())
    }

    /// Returns the count for a state.
    pub fn get(
        &self,
        state: &str,
    ) -> Option<u64> {
        self.counts.get(state).copied()
    }

    /// Returns the number of distinct observed states.
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns whether no states were observed.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Returns total shots represented by these counts.
    pub fn total_shots(&self) -> u64 {
        self.counts
            .values()
            .copied()
            .fold(0u64, u64::saturating_add)
    }

    /// Returns deterministic iteration over measurement states.
    pub fn iter(
        &self,
    ) -> std::collections::btree_map::Iter<'_, String, u64> {
        self.counts.iter()
    }

    /// Validates the complete count collection.
    pub fn validate(&self) -> Result<()> {
        for (state, count) in &self.counts {
            validate_measurement_key(state)?;

            if *count == 0 {
                return Err(AlgorithmError::InvalidInput {
                    field: "measurement_counts".to_string(),
                    reason: format!(
                        "state '{state}' has a zero count"
                    ),
                });
            }
        }

        Ok(())
    }

    /// Returns the state with the largest observed count.
    ///
    /// Ties are resolved lexicographically because the underlying map is
    /// ordered. This makes the result deterministic.
    pub fn most_likely(
        &self,
    ) -> Option<(&str, u64)> {
        self.counts
            .iter()
            .max_by(|(state_a, count_a), (state_b, count_b)| {
                count_a
                    .cmp(count_b)
                    .then_with(|| state_b.cmp(state_a))
            })
            .map(|(state, count)| {
                (state.as_str(), *count)
            })
    }
}

// =============================================================================
// Execution metadata
// =============================================================================

/// Backend-neutral metadata describing one execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionMetadata {
    /// Backend identifier.
    pub backend_id: String,

    /// Backend implementation version.
    pub backend_version: Option<String>,

    /// Number of shots actually executed.
    pub shots: Option<ShotCount>,

    /// Seed actually used by the execution layer.
    pub seed: Option<Seed>,

    /// Whether the backend reports deterministic execution.
    pub deterministic: bool,
}

impl ExecutionMetadata {
    /// Creates execution metadata.
    pub fn new<S: Into<String>>(
        backend_id: S,
        deterministic: bool,
    ) -> Result<Self> {
        let backend_id = backend_id.into();

        validate_text_field(
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

    /// Adds a backend version.
    pub fn with_backend_version<S: Into<String>>(
        mut self,
        version: S,
    ) -> Result<Self> {
        let version = version.into();

        validate_text_field(
            &version,
            MAX_BACKEND_VERSION_BYTES,
            "backend_version",
        )?;

        self.backend_version = Some(version);

        Ok(self)
    }

    /// Adds actual shot information.
    pub fn with_shots(
        mut self,
        shots: ShotCount,
    ) -> Self {
        self.shots = Some(shots);
        self
    }

    /// Adds the actual execution seed.
    pub const fn with_seed(
        mut self,
        seed: Seed,
    ) -> Self {
        self.seed = Some(seed);
        self
    }
}

// =============================================================================
// Algorithm execution record
// =============================================================================

/// Stable identifiers used by replay/provenance systems.
///
/// The algorithms layer does not calculate the digest itself. A later
/// execution/replay subsystem may populate these values from canonical
/// serialization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionDigests {
    /// Digest of the logical algorithm input.
    pub input_digest: Option<String>,

    /// Digest of the generated logical circuit.
    pub circuit_digest: Option<String>,

    /// Digest of the final algorithm result.
    pub result_digest: Option<String>,
}

impl ExecutionDigests {
    /// Creates empty digest metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an input digest.
    pub fn with_input_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Self {
        self.input_digest = Some(digest.into());
        self
    }

    /// Adds a circuit digest.
    pub fn with_circuit_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Self {
        self.circuit_digest = Some(digest.into());
        self
    }

    /// Adds a result digest.
    pub fn with_result_digest<S: Into<String>>(
        mut self,
        digest: S,
    ) -> Self {
        self.result_digest = Some(digest.into());
        self
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_finite_parameter(
    index: usize,
    value: f64,
) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::InvalidParameter {
            name: format!("parameter[{index}]"),
            reason: format!(
                "value must be finite, got {value}"
            ),
        });
    }

    validate_parameter_magnitude(value)
}

fn validate_parameter_magnitude(
    value: f64,
) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue {
            field: "parameter".to_string(),
            value,
        });
    }

    if value.abs() > DEFAULT_MAX_PARAMETER_MAGNITUDE {
        return Err(AlgorithmError::InvalidParameter {
            name: "parameter".to_string(),
            reason: format!(
                "absolute magnitude exceeds maximum {}",
                DEFAULT_MAX_PARAMETER_MAGNITUDE
            ),
        });
    }

    Ok(())
}

fn validate_positive_limit(
    value: u64,
    field: &'static str,
) -> Result<()> {
    if value == 0 {
        return Err(AlgorithmError::InvalidConfiguration {
            field: field.to_string(),
            reason: "limit must be greater than zero".to_string(),
        });
    }

    Ok(())
}

fn validate_text_field(
    value: &str,
    max_bytes: usize,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(AlgorithmError::InvalidInput {
            field: field.to_string(),
            reason: "value cannot be empty".to_string(),
        });
    }

    if value.len() > max_bytes {
        return Err(AlgorithmError::InvalidInput {
            field: field.to_string(),
            reason: format!(
                "UTF-8 length exceeds maximum of {max_bytes} bytes"
            ),
        });
    }

    Ok(())
}

fn validate_measurement_key(
    state: &str,
) -> Result<()> {
    if state.is_empty() {
        return Err(AlgorithmError::InvalidInput {
            field: "measurement_state".to_string(),
            reason: "measurement state cannot be empty".to_string(),
        });
    }

    if state.len() > MAX_MEASUREMENT_KEY_BYTES {
        return Err(AlgorithmError::InvalidInput {
            field: "measurement_state".to_string(),
            reason: format!(
                "measurement state exceeds maximum of {} bytes",
                MAX_MEASUREMENT_KEY_BYTES
            ),
        });
    }

    if !state.bytes().all(|byte| {
        byte == b'0' || byte == b'1'
    }) {
        return Err(AlgorithmError::InvalidInput {
            field: "measurement_state".to_string(),
            reason:
                "measurement state must contain only '0' and '1'"
                    .to_string(),
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
    fn algorithm_version_formats_deterministically() {
        let version =
            AlgorithmVersion::new(1, 2, 3);

        assert_eq!(
            version.to_string(),
            "1.2.3"
        );
    }

    #[test]
    fn qubit_count_rejects_zero() {
        assert!(QubitCount::new(0).is_err());
    }

    #[test]
    fn qubit_count_accepts_valid_value() {
        let count =
            QubitCount::new(32).unwrap();

        assert_eq!(count.get(), 32);
    }

    #[test]
    fn shot_count_rejects_zero() {
        assert!(ShotCount::new(0).is_err());
    }

    #[test]
    fn probabilities_are_bounded() {
        assert!(Probability::new(0.0).is_ok());
        assert!(Probability::new(1.0).is_ok());
        assert!(Probability::new(-0.1).is_err());
        assert!(Probability::new(1.1).is_err());
    }

    #[test]
    fn probabilities_reject_non_finite_values() {
        assert!(
            Probability::new(f64::NAN).is_err()
        );
        assert!(
            Probability::new(f64::INFINITY).is_err()
        );
    }

    #[test]
    fn parameter_vectors_allow_empty_when_algorithm_permits_it() {
        let parameters =
            ParameterVector::new(Vec::new()).unwrap();

        assert!(parameters.is_empty());
        assert!(
            parameters.require_non_empty().is_err()
        );
    }

    #[test]
    fn parameter_vectors_reject_non_finite_values() {
        assert!(
            ParameterVector::new(vec![f64::NAN])
                .is_err()
        );

        assert!(
            ParameterVector::new(vec![f64::INFINITY])
                .is_err()
        );
    }

    #[test]
    fn parameter_vectors_preserve_order() {
        let parameters =
            ParameterVector::new(vec![1.0, 2.0, 3.0])
                .unwrap();

        assert_eq!(
            parameters.as_slice(),
            &[1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn parameter_vectors_support_safe_mutation() {
        let mut parameters =
            ParameterVector::new(vec![0.0, 1.0])
                .unwrap();

        parameters.set(1, 2.0).unwrap();

        assert_eq!(
            parameters.get(1),
            Some(2.0)
        );

        assert!(
            parameters.set(5, 1.0).is_err()
        );

        assert!(
            parameters.set(0, f64::NAN).is_err()
        );
    }

    #[test]
    fn execution_config_is_deterministic_by_default() {
        let config =
            ExecutionConfig::default();

        assert!(config.deterministic);
        assert!(config.seed.is_none());
        assert!(config.optimization_seed.is_none());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn execution_config_accepts_explicit_seeds() {
        let config =
            ExecutionConfig::deterministic()
                .with_seed(Seed::new(42))
                .with_optimization_seed(
                    Seed::new(7),
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
    fn execution_config_rejects_shots_above_limit() {
        let limits = AlgorithmLimits {
            max_shots: 10,
            ..AlgorithmLimits::default()
        };

        let config =
            ExecutionConfig::default()
                .with_limits(limits)
                .unwrap();

        assert!(
            config
                .clone()
                .with_shots(ShotCount::new(11).unwrap())
                .is_err()
        );
    }

    #[test]
    fn measurement_counts_are_deterministic() {
        let mut counts =
            MeasurementCounts::new();

        counts.insert("10", 3).unwrap();
        counts.insert("01", 5).unwrap();
        counts.insert("11", 2).unwrap();

        let states: Vec<&str> = counts
            .iter()
            .map(|(state, _)| state.as_str())
            .collect();

        assert_eq!(
            states,
            vec!["01", "10", "11"]
        );
    }

    #[test]
    fn measurement_counts_reject_non_binary_states() {
        let mut counts =
            MeasurementCounts::new();

        assert!(
            counts.insert("02", 1).is_err()
        );

        assert!(
            counts.insert("", 1).is_err()
        );
    }

    #[test]
    fn measurement_counts_reject_zero_counts() {
        let mut counts =
            MeasurementCounts::new();

        counts.insert("00", 0).unwrap();

        assert!(counts.validate().is_err());
    }

    #[test]
    fn most_likely_result_is_deterministic_on_ties() {
        let mut counts =
            MeasurementCounts::new();

        counts.insert("00", 5).unwrap();
        counts.insert("01", 5).unwrap();

        let result =
            counts.most_likely().unwrap();

        assert_eq!(result, ("00", 5));
    }

    #[test]
    fn limits_reject_zero_values() {
        let limits = AlgorithmLimits {
            max_qubits: 0,
            ..AlgorithmLimits::default()
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn resource_checks_are_centralized() {
        let limits = AlgorithmLimits {
            max_qubits: 8,
            ..AlgorithmLimits::default()
        };

        assert!(
            limits
                .check(ResourceKind::Qubits, 8)
                .is_ok()
        );

        assert!(
            limits
                .check(ResourceKind::Qubits, 9)
                .is_err()
        );
    }

    #[test]
    fn metadata_validates_implementation_name() {
        let metadata =
            AlgorithmMetadata::new(
                AlgorithmId::Vqe,
                AlgorithmVersion::initial(),
            )
            .with_implementation("reference")
            .unwrap();

        assert_eq!(
            metadata.implementation.as_deref(),
            Some("reference")
        );
    }

    #[test]
    fn execution_metadata_validates_backend_identity() {
        let metadata =
            ExecutionMetadata::new(
                "reference-simulator",
                true,
            )
            .unwrap()
            .with_backend_version("1.0.0")
            .unwrap()
            .with_seed(Seed::new(42));

        assert_eq!(
            metadata.backend_id,
            "reference-simulator"
        );

        assert_eq!(
            metadata.backend_version.as_deref(),
            Some("1.0.0")
        );

        assert_eq!(
            metadata.seed,
            Some(Seed::new(42))
        );
    }
}