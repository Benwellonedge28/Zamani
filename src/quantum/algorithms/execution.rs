//! Zamani Quantum Algorithms — Backend-Independent Execution Contract.
//!
//! This module defines the stable boundary between algorithm orchestration and
//! quantum execution.
//!
//! Architectural flow:
//!
//! ```text
//! algorithm
//!     │
//!     ▼
//! ExecutionRequest
//!     │
//!     ▼
//! QuantumExecutor
//!     │
//!     ├── simulator
//!     ├── CPU executor
//!     ├── GPU executor
//!     ├── QPU executor
//!     └── remote executor
//!     │
//!     ▼
//! ExecutionResult
//! ```
//!
//! # Responsibility
//!
//! `execution.rs` owns:
//!
//! - execution requests;
//! - execution modes;
//! - backend-independent execution results;
//! - execution-result validation;
//! - the `QuantumExecutor` trait;
//! - deterministic-execution enforcement;
//! - execution convenience functions;
//! - execution-to-algorithm error boundaries.
//!
//! # This module does NOT own
//!
//! It deliberately does not own:
//!
//! - quantum gate definitions;
//! - circuit storage;
//! - circuit mutation;
//! - logical-to-physical routing;
//! - transpilation;
//! - hardware topology;
//! - device calibration;
//! - QPU credentials;
//! - vendor APIs;
//! - error-correction decoding;
//! - optimizer implementations;
//! - objective implementations;
//! - persistence;
//! - telemetry transport.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Quantum IR integration
//!
//! Algorithms operate on the canonical `quantum::ir::QuantumCircuit`.
//!
//! `execution.rs` does not duplicate the IR's circuit, gate, qubit, or
//! validation semantics. The IR remains authoritative for logical circuit
//! validity.
//!
//! ```text
//! Algorithm
//!     │
//!     ▼
//! QuantumCircuit
//!     │
//!     ▼
//! canonical IR validation
//!     │
//!     ▼
//! ExecutionRequest
//!     │
//!     ▼
//! QuantumExecutor
//! ```
//!
//! # Determinism
//!
//! A deterministic request is a contract, not merely a hint.
//!
//! When `ExecutionConfig::deterministic` is true, the executor result must
//! explicitly report deterministic execution. Otherwise this module rejects
//! the result with `AlgorithmError::DeterminismViolation`.
//!
//! Randomness is never created by this module.
//!
//! Seeds are supplied explicitly through `ExecutionConfig`.
//!
//! # Resource safety
//!
//! Algorithm-level limits are checked before an execution request crosses the
//! backend boundary. Backends may impose stricter limits, but they must never
//! silently exceed the algorithm-level contract.
//!
//! # Mutation boundary
//!
//! `ExecutionRequest` owns its circuit and exposes it only through `&QuantumCircuit`.
//!
//! An executor therefore cannot mutate the caller's circuit through this API.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
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
use std::time::Duration;

use crate::quantum::ir::QuantumCircuit;

use super::error::{AlgorithmError, Result};
use super::types::{
    ExecutionConfig,
    ExecutionDigests,
    ExecutionMetadata,
    ExpectationValue,
    MeasurementCounts,
    Probability,
    QubitCount,
    ResourceKind,
    ShotCount,
};

// =============================================================================
// Execution mode
// =============================================================================

/// Logical execution mode requested by an algorithm.
///
/// This describes what result information is required. It does not identify
/// a simulator, QPU, vendor, operating system, or transport mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    /// Execute the circuit and return measurement information.
    Measurement,

    /// Execute the circuit and return an expectation value.
    Expectation,

    /// Execute the circuit without requiring a measurement or expectation
    /// result at this abstraction boundary.
    ///
    /// This is useful for state-preparation-oriented flows and leaves the
    /// concrete state representation to a future backend-specific extension.
    StatePreparation,
}

impl ExecutionMode {
    /// Returns the stable machine-readable execution mode identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::Expectation => "expectation",
            Self::StatePreparation => "state_preparation",
        }
    }
}

impl fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Execution request
// =============================================================================

/// Immutable request submitted to a quantum execution backend.
///
/// The request owns the logical circuit. The executor receives only an
/// immutable reference and therefore cannot mutate the circuit through the
/// execution boundary.
///
/// All validation that can be performed without a backend is performed before
/// a request is constructed.
#[derive(Debug, Clone)]
pub struct ExecutionRequest {
    circuit: QuantumCircuit,
    config: ExecutionConfig,
    mode: ExecutionMode,

    /// Optional algorithm identifier for diagnostics and provenance.
    algorithm: Option<String>,

    /// Optional operation identifier for diagnostics.
    operation: Option<String>,

    /// Optional backend-neutral observable identifier.
    ///
    /// The mathematical observable representation belongs to the appropriate
    /// algorithm/IR abstraction. This field is context only.
    requested_observable: Option<String>,
}

impl ExecutionRequest {
    /// Creates a measurement execution request.
    ///
    /// Measurement execution requires a positive shot count.
    pub fn measurement(
        circuit: QuantumCircuit,
        config: ExecutionConfig,
    ) -> Result<Self> {
        Self::new(circuit, config, ExecutionMode::Measurement)
    }

    /// Creates an expectation-value execution request.
    pub fn expectation(
        circuit: QuantumCircuit,
        config: ExecutionConfig,
    ) -> Result<Self> {
        Self::new(circuit, config, ExecutionMode::Expectation)
    }

    /// Creates a state-preparation execution request.
    pub fn state_preparation(
        circuit: QuantumCircuit,
        config: ExecutionConfig,
    ) -> Result<Self> {
        Self::new(circuit, config, ExecutionMode::StatePreparation)
    }

    /// Creates an execution request for an explicit execution mode.
    pub fn new(
        circuit: QuantumCircuit,
        config: ExecutionConfig,
        mode: ExecutionMode,
    ) -> Result<Self> {
        config.validate()?;

        validate_circuit(&circuit, &config)?;

        if mode == ExecutionMode::Measurement
            && config.shots.is_none()
        {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "shots".to_string(),
                message:
                    "measurement execution requires a positive shot count"
                        .to_string(),
            });
        }

        Ok(Self {
            circuit,
            config,
            mode,
            algorithm: None,
            operation: None,
            requested_observable: None,
        })
    }

    /// Associates the request with an algorithm identifier.
    pub fn with_algorithm<S: Into<String>>(
        mut self,
        algorithm: S,
    ) -> Result<Self> {
        self.algorithm = Some(validate_text(
            algorithm.into(),
            "algorithm",
            256,
        )?);

        Ok(self)
    }

    /// Associates the request with an algorithm operation.
    pub fn with_operation<S: Into<String>>(
        mut self,
        operation: S,
    ) -> Result<Self> {
        self.operation = Some(validate_text(
            operation.into(),
            "operation",
            256,
        )?);

        Ok(self)
    }

    /// Associates an observable context identifier with an expectation
    /// request.
    ///
    /// The observable's mathematical representation is deliberately not
    /// duplicated here.
    pub fn with_observable<S: Into<String>>(
        mut self,
        observable: S,
    ) -> Result<Self> {
        if self.mode != ExecutionMode::Expectation {
            return Err(AlgorithmError::UnsupportedOperation {
                operation: "requested_observable".to_string(),
                message:
                    "an observable may only be attached to an expectation request"
                        .to_string(),
            });
        }

        self.requested_observable = Some(validate_text(
            observable.into(),
            "observable",
            256,
        )?);

        Ok(self)
    }

    /// Returns the immutable logical circuit.
    pub fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Returns the execution configuration.
    pub const fn config(&self) -> &ExecutionConfig {
        &self.config
    }

    /// Returns the execution mode.
    pub const fn mode(&self) -> ExecutionMode {
        self.mode
    }

    /// Returns the optional algorithm identifier.
    pub fn algorithm(&self) -> Option<&str> {
        self.algorithm.as_deref()
    }

    /// Returns the optional operation identifier.
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the optional observable identifier.
    pub fn requested_observable(&self) -> Option<&str> {
        self.requested_observable.as_deref()
    }

    /// Returns the logical qubit count using the canonical algorithm type.
    pub fn qubit_count(&self) -> Result<QubitCount> {
        let count = u64::try_from(self.circuit.num_qubits())
            .map_err(|_| AlgorithmError::InvalidQubitCount {
                count: self.circuit.num_qubits(),
                message:
                    "logical qubit count cannot be represented as u64"
                        .to_string(),
            })?;

        QubitCount::new(count)
    }

    /// Returns the requested measurement-shot count.
    pub const fn shots(&self) -> Option<ShotCount> {
        self.config.shots
    }

    /// Returns the configured timeout.
    pub const fn timeout(&self) -> Option<Duration> {
        self.config.timeout
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Backend-neutral result of one logical-circuit execution.
///
/// The result intentionally contains only algorithm-level execution data.
/// Backend-specific state, device calibration, pulse information, and vendor
/// objects must remain behind the executor implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    /// Backend-neutral execution metadata.
    metadata: ExecutionMetadata,

    /// Measurement counts returned by the executor.
    measurement_counts: Option<MeasurementCounts>,

    /// Probability distribution returned by the executor.
    ///
    /// `BTreeMap` guarantees deterministic iteration order.
    probabilities: BTreeMap<String, Probability>,

    /// Expectation value returned by the executor.
    expectation: Option<ExpectationValue>,

    /// Number of logical circuit executions performed.
    circuit_executions: u64,

    /// Number of shots represented by the result.
    shots_executed: Option<ShotCount>,

    /// Reproducibility/provenance digests.
    digests: ExecutionDigests,
}

impl ExecutionResult {
    /// Creates an empty execution result with valid backend metadata.
    pub fn new(
        metadata: ExecutionMetadata,
    ) -> Result<Self> {
        if metadata.backend_id.is_empty() {
            return Err(AlgorithmError::InvalidInput {
                message: "backend identifier cannot be empty"
                    .to_string(),
            });
        }

        Ok(Self {
            metadata,
            measurement_counts: None,
            probabilities: BTreeMap::new(),
            expectation: None,
            circuit_executions: 1,
            shots_executed: None,
            digests: ExecutionDigests::new(),
        })
    }

    /// Adds measurement counts.
    pub fn with_measurement_counts(
        mut self,
        counts: MeasurementCounts,
    ) -> Result<Self> {
        counts.validate()?;

        let total = counts.total_shots();

        if total == 0 {
            return Err(AlgorithmError::InvalidInput {
                message:
                    "measurement counts cannot represent zero shots"
                        .to_string(),
            });
        }

        self.measurement_counts = Some(counts);

        self.shots_executed = Some(
            ShotCount::new(total)?,
        );

        Ok(self)
    }

    /// Adds an expectation value.
    pub fn with_expectation(
        mut self,
        expectation: ExpectationValue,
    ) -> Self {
        self.expectation = Some(expectation);
        self
    }

    /// Adds a probability distribution.
    ///
    /// Probabilities are required to be normalized within a strict numerical
    /// tolerance.
    pub fn with_probabilities(
        mut self,
        probabilities: BTreeMap<String, Probability>,
    ) -> Result<Self> {
        validate_probabilities(&probabilities)?;

        self.probabilities = probabilities;

        Ok(self)
    }

    /// Sets the number of circuit executions.
    pub fn with_circuit_executions(
        mut self,
        count: u64,
    ) -> Result<Self> {
        if count == 0 {
            return Err(AlgorithmError::InvalidInput {
                message:
                    "circuit execution count must be greater than zero"
                        .to_string(),
            });
        }

        self.circuit_executions = count;

        Ok(self)
    }

    /// Sets explicit shot accounting.
    pub fn with_shots_executed(
        mut self,
        shots: ShotCount,
    ) -> Self {
        self.shots_executed = Some(shots);
        self
    }

    /// Sets reproducibility/provenance digests.
    pub fn with_digests(
        mut self,
        digests: ExecutionDigests,
    ) -> Self {
        self.digests = digests;
        self
    }

    /// Returns execution metadata.
    pub const fn metadata(
        &self,
    ) -> &ExecutionMetadata {
        &self.metadata
    }

    /// Returns measurement counts, if available.
    pub fn measurement_counts(
        &self,
    ) -> Option<&MeasurementCounts> {
        self.measurement_counts.as_ref()
    }

    /// Returns the probability distribution in deterministic key order.
    pub fn probabilities(
        &self,
    ) -> &BTreeMap<String, Probability> {
        &self.probabilities
    }

    /// Returns the expectation value, if available.
    pub const fn expectation(
        &self,
    ) -> Option<ExpectationValue> {
        self.expectation
    }

    /// Returns the number of executed logical circuits.
    pub const fn circuit_executions(
        &self,
    ) -> u64 {
        self.circuit_executions
    }

    /// Returns the number of executed shots, when meaningful.
    pub const fn shots_executed(
        &self,
    ) -> Option<ShotCount> {
        self.shots_executed
    }

    /// Returns reproducibility/provenance digests.
    pub const fn digests(
        &self,
    ) -> &ExecutionDigests {
        &self.digests
    }

    /// Validates this result against the original request.
    ///
    /// This is deliberately performed after backend execution so that an
    /// executor cannot silently violate the algorithm-level execution
    /// contract.
    pub fn validate_against(
        &self,
        request: &ExecutionRequest,
    ) -> Result<()> {
        if self.circuit_executions == 0 {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    message:
                        "execution result reports zero circuit executions"
                            .to_string(),
                },
            );
        }

        // A deterministic request must never be satisfied by a backend that
        // reports nondeterministic behavior.
        if request.config.deterministic
            && !self.metadata.deterministic
        {
            return Err(
                AlgorithmError::DeterminismViolation {
                    contract:
                        "deterministic execution".to_string(),
                    message:
                        "executor returned a non-deterministic result for a deterministic request"
                            .to_string(),
                },
            );
        }

        match request.mode {
            ExecutionMode::Measurement => {
                if self.measurement_counts.is_none()
                    && self.probabilities.is_empty()
                {
                    return Err(
                        AlgorithmError::ExecutionFailed {
                            backend: Some(
                                self.metadata
                                    .backend_id
                                    .clone(),
                            ),
                            operation:
                                "measurement".to_string(),
                            message:
                                "measurement execution returned neither counts nor probabilities"
                                    .to_string(),
                        },
                    );
                }

                if let Some(requested) =
                    request.config.shots
                {
                    if let Some(actual) =
                        self.shots_executed
                    {
                        if actual.get()
                            > requested.get()
                        {
                            return Err(
                                AlgorithmError::ResourceLimitExceeded {
                                    resource:
                                        "shots".to_string(),
                                    requested:
                                        actual.get() as u128,
                                    limit:
                                        requested.get() as u128,
                                    message:
                                        "executor returned more shots than requested"
                                            .to_string(),
                                },
                            );
                        }
                    }
                }
            }

            ExecutionMode::Expectation => {
                if self.expectation.is_none() {
                    return Err(
                        AlgorithmError::ExecutionFailed {
                            backend: Some(
                                self.metadata
                                    .backend_id
                                    .clone(),
                            ),
                            operation:
                                "expectation".to_string(),
                            message:
                                "expectation execution returned no expectation value"
                                    .to_string(),
                        },
                    );
                }
            }

            ExecutionMode::StatePreparation => {}
        }

        Ok(())
    }
}

// =============================================================================
// Executor contract
// =============================================================================

/// Backend-independent quantum execution interface.
///
/// Implementations can represent:
///
/// - deterministic simulators;
/// - stochastic simulators;
/// - CPU execution;
/// - GPU execution;
/// - local QPUs;
/// - remote QPUs;
/// - distributed quantum execution;
/// - future execution technologies.
///
/// The algorithms subsystem depends only on this contract.
pub trait QuantumExecutor {
    /// Executes one logical quantum-circuit request.
    ///
    /// The request and its circuit are immutable for the duration of this
    /// call.
    fn execute(
        &mut self,
        request: &ExecutionRequest,
    ) -> Result<ExecutionResult>;
}

/// Capability marker for executors that explicitly guarantee deterministic
/// behavior whenever a deterministic request is accepted.
///
/// This trait does not alter execution semantics. It is an additional static
/// capability contract for higher-level components.
pub trait DeterministicQuantumExecutor:
    QuantumExecutor
{
}

// =============================================================================
// Canonical execution entry points
// =============================================================================

/// Executes a request through the canonical algorithm execution boundary.
///
/// This function:
///
/// 1. validates the request configuration;
/// 2. invokes the backend;
/// 3. validates the backend result;
/// 4. rejects deterministic-contract violations;
/// 5. returns only a contract-valid result.
pub fn execute(
    executor: &mut dyn QuantumExecutor,
    request: &ExecutionRequest,
) -> Result<ExecutionResult> {
    request.config.validate()?;

    let result = executor.execute(request)?;

    result.validate_against(request)?;

    Ok(result)
}

/// Executes a measurement request.
pub fn execute_measurement(
    executor: &mut dyn QuantumExecutor,
    circuit: QuantumCircuit,
    config: ExecutionConfig,
) -> Result<ExecutionResult> {
    let request =
        ExecutionRequest::measurement(
            circuit,
            config,
        )?;

    execute(executor, &request)
}

/// Executes an expectation-value request and returns the expectation.
pub fn execute_expectation(
    executor: &mut dyn QuantumExecutor,
    circuit: QuantumCircuit,
    config: ExecutionConfig,
) -> Result<ExpectationValue> {
    let request =
        ExecutionRequest::expectation(
            circuit,
            config,
        )?;

    let result =
        execute(executor, &request)?;

    result.expectation().ok_or_else(|| {
        AlgorithmError::ExecutionFailed {
            backend: Some(
                result
                    .metadata
                    .backend_id
                    .clone(),
            ),
            operation:
                "expectation".to_string(),
            message:
                "executor returned no expectation value"
                    .to_string(),
        }
    })
}

// =============================================================================
// Circuit validation
// =============================================================================

/// Validates a logical circuit before it crosses the execution boundary.
///
/// The algorithm layer checks algorithm-level resource limits first, then
/// delegates circuit semantics to `quantum::ir`.
fn validate_circuit(
    circuit: &QuantumCircuit,
    config: &ExecutionConfig,
) -> Result<()> {
    let qubits =
        u64::try_from(circuit.num_qubits())
            .map_err(|_| {
                AlgorithmError::InvalidQubitCount {
                    count:
                        circuit.num_qubits(),
                    message:
                        "logical qubit count cannot be represented as u64"
                            .to_string(),
                }
            })?;

    let gates =
        u64::try_from(circuit.len())
            .map_err(|_| {
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        "gates".to_string(),
                    requested:
                        u128::MAX,
                    limit:
                        config
                            .limits
                            .max_gates
                            as u128,
                    message:
                        "gate count cannot be represented as u64"
                            .to_string(),
                }
            })?;

    config.limits.check(
        ResourceKind::Qubits,
        qubits,
    )?;

    config.limits.check(
        ResourceKind::Gates,
        gates,
    )?;

    // The Quantum IR owns circuit semantics. Do not duplicate gate,
    // measurement, namespace, version, or structural validation here.
    crate::quantum::ir::validate_circuit(
        circuit,
    )
    .map_err(|error| {
        AlgorithmError::InvalidCircuit {
            circuit: Some(
                circuit.id().to_string(),
            ),
            message: error.to_string(),
        }
    })?;

    Ok(())
}

// =============================================================================
// Probability validation
// =============================================================================

/// Validates a probability distribution.
///
/// The map is deterministic because callers provide a `BTreeMap`.
fn validate_probabilities(
    probabilities: &BTreeMap<String, Probability>,
) -> Result<()> {
    if probabilities.is_empty() {
        return Ok(());
    }

    let mut sum = 0.0f64;

    for probability in probabilities.values() {
        sum += probability.get();

        if !sum.is_finite() {
            return Err(
                AlgorithmError::NumericalInstability {
                    operation:
                        "probability normalization"
                            .to_string(),
                    message:
                        "probability sum became non-finite"
                            .to_string(),
                },
            );
        }
    }

    const NORMALIZATION_TOLERANCE: f64 =
        1.0e-12;

    if (sum - 1.0).abs()
        > NORMALIZATION_TOLERANCE
    {
        return Err(
            AlgorithmError::InvalidInput {
                message: format!(
                    "probability distribution is not normalized: sum={sum}"
                ),
            },
        );
    }

    Ok(())
}

// =============================================================================
// Text validation
// =============================================================================

/// Validates bounded textual execution metadata.
///
/// Text fields are identifiers rather than arbitrary user documents. Control
/// characters are therefore rejected.
fn validate_text(
    value: String,
    field: &str,
    maximum_bytes: usize,
) -> Result<String> {
    if value.is_empty() {
        return Err(
            AlgorithmError::InvalidInput {
                message: format!(
                    "{field} cannot be empty"
                ),
            },
        );
    }

    if value.len() > maximum_bytes {
        return Err(
            AlgorithmError::ResourceLimitExceeded {
                resource:
                    format!("{field}_bytes"),
                requested:
                    value.len() as u128,
                limit:
                    maximum_bytes as u128,
                message:
                    format!(
                        "{field} exceeds its maximum byte length"
                    ),
            },
        );
    }

    if value.chars().any(char::is_control)
    {
        return Err(
            AlgorithmError::InvalidInput {
                message:
                    format!(
                        "{field} contains control characters"
                    ),
            },
        );
    }

    Ok(value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        QuantumCircuit,
        QuantumIrLimits,
    };

    // -------------------------------------------------------------------------
    // Test executor
    // -------------------------------------------------------------------------

    struct MockExecutor;

    impl QuantumExecutor for MockExecutor {
        fn execute(
            &mut self,
            request: &ExecutionRequest,
        ) -> Result<ExecutionResult> {
            let metadata =
                ExecutionMetadata::new(
                    "test-executor",
                    true,
                )?;

            let result =
                ExecutionResult::new(
                    metadata,
                )?;

            match request.mode() {
                ExecutionMode::Measurement => {
                    let mut counts =
                        MeasurementCounts::new();

                    counts.insert(
                        "0",
                        1,
                    )?;

                    result
                        .with_measurement_counts(
                            counts,
                        )
                }

                ExecutionMode::Expectation => {
                    Ok(
                        result.with_expectation(
                            ExpectationValue::new(
                                0.5,
                            )?,
                        ),
                    )
                }

                ExecutionMode::StatePreparation => {
                    Ok(result)
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Test circuit
    // -------------------------------------------------------------------------

    fn test_circuit() -> QuantumCircuit {
        QuantumCircuit::try_new_with_limits(
            1,
            1,
            QuantumIrLimits::production(),
        )
        .expect("valid test circuit")
    }

    // -------------------------------------------------------------------------
    // Request validation
    // -------------------------------------------------------------------------

    #[test]
    fn measurement_request_requires_shots()
    {
        let request =
            ExecutionRequest::measurement(
                test_circuit(),
                ExecutionConfig::default(),
            );

        assert!(request.is_err());
    }

    #[test]
    fn expectation_request_does_not_require_shots()
    {
        let request =
            ExecutionRequest::expectation(
                test_circuit(),
                ExecutionConfig::deterministic(),
            );

        assert!(request.is_ok());
    }

    // -------------------------------------------------------------------------
    // Deterministic execution
    // -------------------------------------------------------------------------

    #[test]
    fn deterministic_execution_is_verified()
    {
        let config =
            ExecutionConfig::deterministic()
                .with_shots(
                    ShotCount::new(1)
                        .expect(
                            "valid shot count",
                        ),
                )
                .expect(
                    "valid execution configuration",
                );

        let request =
            ExecutionRequest::measurement(
                test_circuit(),
                config,
            )
            .expect(
                "valid execution request",
            );

        let mut executor =
            MockExecutor;

        let result =
            execute(
                &mut executor,
                &request,
            )
            .expect(
                "execution succeeds",
            );

        assert!(
            result
                .measurement_counts()
                .is_some()
        );

        assert!(
            result
                .metadata()
                .deterministic
        );
    }

    // -------------------------------------------------------------------------
    // Expectation execution
    // -------------------------------------------------------------------------

    #[test]
    fn expectation_execution_returns_value()
    {
        let request =
            ExecutionRequest::expectation(
                test_circuit(),
                ExecutionConfig::deterministic(),
            )
            .expect(
                "valid expectation request",
            );

        let mut executor =
            MockExecutor;

        let result =
            execute(
                &mut executor,
                &request,
            )
            .expect(
                "execution succeeds",
            );

        assert_eq!(
            result
                .expectation()
                .expect(
                    "expectation value",
                )
                .get(),
            0.5
        );
    }

    // -------------------------------------------------------------------------
    // Determinism violation
    // -------------------------------------------------------------------------

    struct NondeterministicExecutor;

    impl QuantumExecutor
        for NondeterministicExecutor
    {
        fn execute(
            &mut self,
            _request: &ExecutionRequest,
        ) -> Result<ExecutionResult> {
            let metadata =
                ExecutionMetadata::new(
                    "nondeterministic-test",
                    false,
                )?;

            ExecutionResult::new(
                metadata,
            )
        }
    }

    #[test]
    fn deterministic_request_rejects_nondeterministic_backend()
    {
        let request =
            ExecutionRequest::state_preparation(
                test_circuit(),
                ExecutionConfig::deterministic(),
            )
            .expect(
                "valid request",
            );

        let mut executor =
            NondeterministicExecutor;

        let result =
            execute(
                &mut executor,
                &request,
            );

        assert!(
            matches!(
                result,
                Err(
                    AlgorithmError::DeterminismViolation {
                        ..
                    }
                )
            )
        );
    }

    // -------------------------------------------------------------------------
    // Probability validation
    // -------------------------------------------------------------------------

    #[test]
    fn normalized_probabilities_are_accepted()
    {
        let mut probabilities =
            BTreeMap::new();

        probabilities.insert(
            "0".to_string(),
            Probability::new(
                0.5,
            )
            .expect(
                "valid probability",
            ),
        );

        probabilities.insert(
            "1".to_string(),
            Probability::new(
                0.5,
            )
            .expect(
                "valid probability",
            ),
        );

        let metadata =
            ExecutionMetadata::new(
                "test",
                true,
            )
            .expect(
                "valid metadata",
            );

        let result =
            ExecutionResult::new(
                metadata,
            )
            .expect(
                "valid result",
            )
            .with_probabilities(
                probabilities,
            );

        assert!(result.is_ok());
    }

    #[test]
    fn unnormalized_probabilities_are_rejected()
    {
        let mut probabilities =
            BTreeMap::new();

        probabilities.insert(
            "0".to_string(),
            Probability::new(
                0.4,
            )
            .expect(
                "valid probability",
            ),
        );

        probabilities.insert(
            "1".to_string(),
            Probability::new(
                0.4,
            )
            .expect(
                "valid probability",
            ),
        );

        let metadata =
            ExecutionMetadata::new(
                "test",
                true,
            )
            .expect(
                "valid metadata",
            );

        let result =
            ExecutionResult::new(
                metadata,
            )
            .expect(
                "valid result",
            )
            .with_probabilities(
                probabilities,
            );

        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Measurement accounting
    // -------------------------------------------------------------------------

    #[test]
    fn measurement_counts_record_shots()
    {
        let mut counts =
            MeasurementCounts::new();

        counts
            .insert(
                "00",
                3,
            )
            .expect(
                "valid count",
            );

        counts
            .insert(
                "11",
                2,
            )
            .expect(
                "valid count",
            );

        let metadata =
            ExecutionMetadata::new(
                "test",
                true,
            )
            .expect(
                "valid metadata",
            );

        let result =
            ExecutionResult::new(
                metadata,
            )
            .expect(
                "valid result",
            )
            .with_measurement_counts(
                counts,
            )
            .expect(
                "valid measurement result",
            );

        assert_eq!(
            result
                .shots_executed()
                .expect(
                    "shot accounting",
                )
                .get(),
            5
        );
    }
}