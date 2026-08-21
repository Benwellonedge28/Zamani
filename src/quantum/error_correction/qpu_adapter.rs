//! Zamani Quantum Error Correction — physical QPU adapter boundary.
//!
//! # Ownership
//!
//! `qpu_adapter.rs` owns the classical boundary between the QEC subsystem and
//! an externally managed physical quantum processor.
//!
//! It owns:
//!
//! - QPU submission request representation;
//! - QPU circuit representation required for syndrome extraction;
//! - backend capability preflight;
//! - QPU capability authorization preflight;
//! - canonical QPU resource preflight;
//! - cancellation checks around physical execution;
//! - conversion of validated QPU measurements into `Syndrome`;
//! - measurement completeness and ordering validation;
//! - QPU execution metadata;
//! - explicit separation between submission and result retrieval;
//! - fail-closed QPU execution semantics.
//!
//! It does NOT own:
//!
//! - QEC topology (`surface_code.rs`);
//! - stabilizer mathematics (`stabilizer.rs`);
//! - decoding (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - logical equivalence (`logical.rs` / `logical_equivalence.rs`);
//! - QPU credentials;
//! - private keys;
//! - API tokens;
//! - network clients;
//! - authentication;
//! - transport retry policy;
//! - scheduler policy;
//! - runtime resource accounting;
//! - memory allocation;
//! - telemetry transport;
//! - decoder execution.
//!
//! # Integration contract
//!
//! ```text
//! QEC CODE / SURFACE CODE
//!          |
//!          v
//! SyndromeExtractionCircuit
//!          |
//!          v
//! QpuAdapter
//!          |
//!          +--------------------------+
//!          |                          |
//!          v                          v
//! capability authorization       resource preflight
//!          |                          |
//!          +------------+-------------+
//!                       |
//!                       v
//!                QpuTransport
//!                       |
//!                       v
//!                 Physical QPU
//!                       |
//!                       v
//!              Raw QPU measurements
//!                       |
//!                       v
//!              validation / completeness
//!                       |
//!                       v
//!                   Syndrome
//!                       |
//!                       v
//!               decoding_graph.rs
//!                       |
//!                +------+------+
 //!                |             |
//!                v             v
//!              MWPM       Union-Find
//!                |             |
//!                +------+------+
 //!                       |
//!                       v
//!                  Pauli frame
//!                       |
//!                       v
//!                 logical outcome
//! ```
//!
//! # Security boundary
//!
//! The adapter intentionally receives a `QpuTransport` implementation rather
//! than credentials or a network client.
//!
//! This means:
//!
//! - credentials cannot accidentally become part of a QEC request;
//! - decoders never receive credentials;
//! - configuration cannot grant QPU authority;
//! - `QpuAccess` does not imply submission;
//! - submission does not imply result access;
//! - calibration access remains independent;
//! - physical execution is impossible without an explicit transport.
//!
//! # Capability contract
//!
//! Syndrome extraction requires all of:
//!
//! - `Capability::QpuAccess`;
//! - `Capability::QpuSubmit`;
//! - `Capability::QpuReadResults`;
//! - `Capability::QpuSyndromeExtraction`.
//!
//! Optional calibration access requires:
//!
//! - `Capability::QpuCalibration`.
//!
//! The adapter never treats a configuration flag as authority.
//!
//! # Resource contract
//!
//! `QecLimits` remains the canonical declarative policy.
//!
//! QPU-specific admission checks use:
//!
//! - `max_qubits`;
//! - `max_rounds`;
//! - `max_qpu_shots`;
//! - `max_qpu_circuits`;
//! - `max_stabilizers`;
//! - `max_syndrome_events`.
//!
//! Backend-specific limits may impose stricter capacity, but cannot replace
//! `QecLimits`.
//!
//! # Cancellation contract
//!
//! Cancellation is checked:
//!
//! 1. before validation;
//! 2. before submission;
//! 3. immediately after submission;
//! 4. before result retrieval;
//! 5. after result retrieval;
//! 6. before syndrome construction.
//!
//! The adapter never forcefully terminates a physical QPU operation. Physical
//! cancellation, when supported, belongs to the transport implementation.
//!
//! # Determinism
//!
//! The adapter does not manufacture deterministic QPU measurements.
//!
//! It preserves:
//!
//! - request identity;
//! - circuit identity;
//! - round number;
//! - measurement ordering;
//! - backend identity;
//! - seed metadata when supplied by the transport.
//!
//! Reproducibility of actual hardware execution belongs to `replay.rs` and the
//! backend/QPU implementation.
//!
//! # Failure policy
//!
//! All failures are fail-closed.
//!
//! Invalid measurements are never silently converted into syndrome bits.
//! Missing measurements are never interpreted as zero.
//! Duplicate stabilizer measurements are rejected.
//! Unexpected stabilizers are rejected.
//! Capability failures occur before physical submission.
//! Resource failures occur before physical submission.
//!
//! # Rust compatibility
//!
//! This file targets Rust 1.97.1 and intentionally uses stable standard
//! library functionality only.
//!
//! # Future integration
//!
//! This file is already prepared for:
//!
//! - `syndrome_extractor.rs`;
//! - `resource_estimator.rs`;
//! - `metrics.rs`;
//! - `telemetry.rs`;
//! - `checkpoint.rs`;
//! - `replay.rs`;
//! - concrete QPU backends.
//!
//! Those modules may consume the contracts here without changing this file.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::backend::{BackendCapabilities, BackendKind};
use super::cancellation::CancellationToken;
use super::capabilities::{Capability, CapabilitySet};
use super::errors::{
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;
use super::syndrome::{
    MeasurementConfidence,
    MeasurementRound,
    MeasurementTimestamp,
    StabilizerId,
    Syndrome,
    SyndromeMeasurement,
    SyndromeOptions,
};

// ============================================================================
// Stable constants
// ============================================================================

/// Stable adapter contract version.
pub const QPU_ADAPTER_VERSION: &str = "1.0.0";

/// Stable operation identifier for hardware syndrome extraction.
pub const QPU_SYNDROME_EXTRACTION_OPERATION: &str =
    "qpu.syndrome_extraction";

/// Stable operation identifier for circuit submission.
pub const QPU_CIRCUIT_SUBMISSION_OPERATION: &str =
    "qpu.circuit_submission";

/// Stable operation identifier for result retrieval.
pub const QPU_RESULT_RETRIEVAL_OPERATION: &str =
    "qpu.result_retrieval";

// ============================================================================
// QPU execution identity
// ============================================================================

/// Stable identity of a physical QPU backend.
///
/// Credentials, endpoint URLs, tokens and private authentication material are
/// deliberately excluded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QpuIdentity {
    /// Stable provider-independent backend identifier.
    backend_id: String,

    /// Backend implementation/provider name.
    provider: String,

    /// Hardware revision or backend generation.
    revision: String,
}

impl QpuIdentity {
    /// Creates a validated QPU identity.
    pub fn new(
        backend_id: impl Into<String>,
        provider: impl Into<String>,
        revision: impl Into<String>,
    ) -> QecResult<Self> {
        let backend_id = normalize_identifier(backend_id.into());
        let provider = normalize_identifier(provider.into());
        let revision = normalize_identifier(revision.into());

        if backend_id.is_empty() {
            return Err(invalid_input(
                "QPU backend identifier must not be empty",
            ));
        }

        if provider.is_empty() {
            return Err(invalid_input(
                "QPU provider identifier must not be empty",
            ));
        }

        if revision.is_empty() {
            return Err(invalid_input(
                "QPU revision identifier must not be empty",
            ));
        }

        Ok(Self {
            backend_id,
            provider,
            revision,
        })
    }

    /// Returns the stable backend identifier.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns the hardware/backend revision.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

// ============================================================================
// Circuit operation
// ============================================================================

/// Backend-independent QPU circuit operation.
///
/// This intentionally contains only the semantic operations required by the
/// QEC adapter. Mapping these operations to native gates belongs to the QPU
/// transport/backend implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QpuOperation {
    /// Reset a qubit.
    Reset {
        qubit: usize,
    },

    /// Single-qubit semantic operation.
    SingleQubit {
        name: String,
        qubit: usize,
    },

    /// Two-qubit semantic operation.
    TwoQubit {
        name: String,
        control: usize,
        target: usize,
    },

    /// Measure a stabilizer.
    MeasureStabilizer {
        stabilizer: StabilizerId,
        qubit: usize,
    },

    /// Barrier used to preserve execution ordering.
    Barrier,
}

impl QpuOperation {
    /// Returns the operation's semantic identifier.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Reset { .. } => "reset",
            Self::SingleQubit { .. } => "single_qubit",
            Self::TwoQubit { .. } => "two_qubit",
            Self::MeasureStabilizer { .. } => "measure_stabilizer",
            Self::Barrier => "barrier",
        }
    }
}

// ============================================================================
// Syndrome extraction circuit
// ============================================================================

/// Backend-independent syndrome-extraction circuit.
///
/// `qpu_adapter.rs` validates this representation but does not compile it to
/// native hardware instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeExtractionCircuit {
    qubit_count: usize,
    rounds: usize,
    operations: Vec<QpuOperation>,
    measured_stabilizers: Vec<StabilizerId>,
}

impl SyndromeExtractionCircuit {
    /// Creates a syndrome-extraction circuit.
    pub fn new(
        qubit_count: usize,
        rounds: usize,
        operations: Vec<QpuOperation>,
        measured_stabilizers: Vec<StabilizerId>,
    ) -> QecResult<Self> {
        if qubit_count == 0 {
            return Err(invalid_input(
                "QPU circuit must contain at least one qubit",
            ));
        }

        if rounds == 0 {
            return Err(invalid_input(
                "QPU circuit must contain at least one round",
            ));
        }

        if measured_stabilizers.is_empty() {
            return Err(invalid_input(
                "QPU circuit must measure at least one stabilizer",
            ));
        }

        let mut stabilizers = BTreeSet::new();

        for stabilizer in &measured_stabilizers {
            if !stabilizers.insert(*stabilizer) {
                return Err(invalid_syndrome(format!(
                    "duplicate stabilizer {} in QPU circuit",
                    stabilizer
                )));
            }
        }

        validate_operations(
            qubit_count,
            &operations,
            &stabilizers,
        )?;

        Ok(Self {
            qubit_count,
            rounds,
            operations,
            measured_stabilizers,
        })
    }

    /// Number of physical qubits required.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Number of extraction rounds.
    #[must_use]
    pub const fn rounds(&self) -> usize {
        self.rounds
    }

    /// Semantic circuit operations.
    #[must_use]
    pub fn operations(&self) -> &[QpuOperation] {
        &self.operations
    }

    /// Stabilizers expected in the returned measurement set.
    #[must_use]
    pub fn measured_stabilizers(&self) -> &[StabilizerId] {
        &self.measured_stabilizers
    }

    /// Number of semantic operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
}

// ============================================================================
// QPU measurement
// ============================================================================

/// One raw-but-validated QPU stabilizer measurement.
///
/// The measurement remains backend-neutral. The adapter does not interpret
/// physical voltage/current/readout values; transports must convert those to a
/// validated syndrome bit before returning this structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QpuMeasurement {
    stabilizer: StabilizerId,
    value: bool,
    confidence: MeasurementConfidence,
}

impl QpuMeasurement {
    /// Creates a QPU measurement.
    #[must_use]
    pub const fn new(
        stabilizer: StabilizerId,
        value: bool,
        confidence: MeasurementConfidence,
    ) -> Self {
        Self {
            stabilizer,
            value,
            confidence,
        }
    }

    /// Returns the measured stabilizer.
    #[must_use]
    pub const fn stabilizer(self) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the syndrome bit.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }

    /// Returns the measurement confidence.
    #[must_use]
    pub const fn confidence(self) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// QPU request
// ============================================================================

/// Immutable QPU execution request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuExecutionRequest {
    circuit: SyndromeExtractionCircuit,
    shots: u64,
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,
}

impl QpuExecutionRequest {
    /// Creates a validated request.
    pub fn new(
        circuit: SyndromeExtractionCircuit,
        shots: u64,
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
    ) -> QecResult<Self> {
        if shots == 0 {
            return Err(invalid_input(
                "QPU execution request must contain at least one shot",
            ));
        }

        Ok(Self {
            circuit,
            shots,
            round,
            timestamp,
        })
    }

    /// Returns the circuit.
    #[must_use]
    pub fn circuit(&self) -> &SyndromeExtractionCircuit {
        &self.circuit
    }

    /// Returns requested shots.
    #[must_use]
    pub const fn shots(&self) -> u64 {
        self.shots
    }

    /// Returns the logical syndrome round.
    #[must_use]
    pub const fn round(&self) -> MeasurementRound {
        self.round
    }

    /// Returns the backend-independent timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> MeasurementTimestamp {
        self.timestamp
    }
}

// ============================================================================
// QPU execution result
// ============================================================================

/// Successful QPU execution result.
///
/// This contains only validated QEC-relevant measurements and execution
/// metadata. It does not contain credentials or transport secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuExecutionResult {
    request_id: String,
    shots_completed: u64,
    measurements: Vec<QpuMeasurement>,
    backend_identity: QpuIdentity,
    elapsed: Duration,
}

impl QpuExecutionResult {
    /// Creates a successful result.
    pub fn new(
        request_id: impl Into<String>,
        shots_completed: u64,
        measurements: Vec<QpuMeasurement>,
        backend_identity: QpuIdentity,
        elapsed: Duration,
    ) -> QecResult<Self> {
        let request_id = normalize_identifier(request_id.into());

        if request_id.is_empty() {
            return Err(invalid_input(
                "QPU request identifier must not be empty",
            ));
        }

        if shots_completed == 0 {
            return Err(invalid_input(
                "QPU result must report at least one completed shot",
            ));
        }

        Ok(Self {
            request_id,
            shots_completed,
            measurements,
            backend_identity,
            elapsed,
        })
    }

    /// Returns the request identifier.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Returns completed shots.
    #[must_use]
    pub const fn shots_completed(&self) -> u64 {
        self.shots_completed
    }

    /// Returns measurements.
    #[must_use]
    pub fn measurements(&self) -> &[QpuMeasurement] {
        &self.measurements
    }

    /// Returns backend identity.
    #[must_use]
    pub fn backend_identity(&self) -> &QpuIdentity {
        &self.backend_identity
    }

    /// Returns physical execution duration.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

// ============================================================================
// Transport error
// ============================================================================

/// Error returned by a concrete QPU transport.
///
/// The transport owns provider-specific failure details. The adapter converts
/// it into the canonical `QecError::QpuFailure` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuTransportError {
    operation: String,
    message: String,
}

impl QpuTransportError {
    /// Creates a transport error.
    #[must_use]
    pub fn new(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Returns the failed operation.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for QpuTransportError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.operation,
            self.message
        )
    }
}

impl std::error::Error for QpuTransportError {}

// ============================================================================
// QPU transport contract
// ============================================================================

/// Physical QPU transport boundary.
///
/// A concrete provider implementation must implement this trait.
///
/// Credentials, network clients, authentication and retry state remain inside
/// the implementation and never cross into QEC orchestration.
///
/// The transport is responsible for:
///
/// - converting semantic circuit operations to native hardware operations;
/// - authenticating with the provider;
/// - submitting physical work;
/// - polling provider state if necessary;
/// - retrieving provider results;
/// - converting provider measurements into `QpuMeasurement`.
///
/// The transport must not:
///
/// - bypass capability checks;
/// - bypass QEC resource limits;
/// - silently change the requested circuit;
/// - return duplicate stabilizers;
/// - return measurements for unrequested stabilizers.
pub trait QpuTransport: Send + Sync {
    /// Returns immutable backend identity.
    fn identity(&self) -> QecResult<QpuIdentity>;

    /// Returns technical capabilities advertised by the backend.
    fn capabilities(&self) -> QecResult<BackendCapabilities>;

    /// Submits a validated semantic QEC circuit.
    ///
    /// Submission authority is checked by `QpuAdapter` before this method is
    /// called.
    fn submit(
        &self,
        request: &QpuExecutionRequest,
        cancellation: &CancellationToken,
    ) -> Result<String, QpuTransportError>;

    /// Retrieves the result associated with a previously submitted request.
    ///
    /// Result-read authority is checked by `QpuAdapter` before this method is
    /// called.
    fn read_results(
        &self,
        request_id: &str,
        cancellation: &CancellationToken,
    ) -> Result<QpuExecutionResult, QpuTransportError>;
}

// ============================================================================
// Adapter configuration
// ============================================================================

/// Explicit configuration consumed by the QPU adapter.
///
/// This is deliberately smaller than `QecConfig`. `configuration.rs` owns the
/// complete application configuration; this structure is the execution
/// boundary passed into the adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuAdapterConfig {
    /// Canonical QEC resource policy.
    pub limits: QecLimits,

    /// Required caller capabilities.
    pub capabilities: CapabilitySet,

    /// Whether deterministic execution is required.
    pub deterministic: bool,
}

impl QpuAdapterConfig {
    /// Creates a QPU adapter configuration.
    #[must_use]
    pub fn new(
        limits: QecLimits,
        capabilities: CapabilitySet,
    ) -> Self {
        Self {
            limits,
            capabilities,
            deterministic: false,
        }
    }

    /// Requires deterministic backend execution.
    #[must_use]
    pub const fn require_determinism(
        mut self,
    ) -> Self {
        self.deterministic = true;
        self
    }

    /// Validates the adapter policy.
    pub fn validate(&self) -> QecResult<()> {
        self.limits.validate().map_err(|error| {
            invalid_input(format!(
                "invalid QEC limits for QPU adapter: {error}"
            ))
        })?;

        Ok(())
    }
}

// ============================================================================
// Adapter
// ============================================================================

/// Secure physical-QPU adapter.
///
/// The adapter is intentionally transport-agnostic.
pub struct QpuAdapter {
    transport: Arc<dyn QpuTransport>,
    config: QpuAdapterConfig,
}

impl fmt::Debug for QpuAdapter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("QpuAdapter")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl QpuAdapter {
    /// Creates an adapter after validating its policy and backend boundary.
    pub fn new(
        transport: Arc<dyn QpuTransport>,
        config: QpuAdapterConfig,
    ) -> QecResult<Self> {
        config.validate()?;

        let capabilities = transport.capabilities().map_err(|error| {
            qpu_failure(
                "backend_capabilities",
                error.to_string(),
            )
        })?;

        validate_qpu_backend(&capabilities)?;

        Ok(Self {
            transport,
            config,
        })
    }

    /// Returns the adapter configuration.
    #[must_use]
    pub fn config(&self) -> &QpuAdapterConfig {
        &self.config
    }

    /// Returns the physical backend identity.
    pub fn identity(&self) -> QecResult<QpuIdentity> {
        self.transport.identity().map_err(|error| {
            qpu_failure(
                "backend_identity",
                error.to_string(),
            )
        })
    }

    /// Performs complete physical syndrome extraction.
    ///
    /// The returned `Syndrome` is ready for decoding-graph construction.
    pub fn extract_syndrome(
        &self,
        circuit: SyndromeExtractionCircuit,
        shots: u64,
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
        cancellation: &CancellationToken,
    ) -> QecResult<Syndrome> {
        cancellation.check()?;

        self.preflight_circuit(&circuit, shots)?;

        cancellation.check()?;

        let request = QpuExecutionRequest::new(
            circuit,
            shots,
            round,
            timestamp,
        )?;

        let submitted_at = Instant::now();

        let request_id = self
            .transport
            .submit(&request, cancellation)
            .map_err(|error| {
                qpu_failure(
                    QPU_CIRCUIT_SUBMISSION_OPERATION,
                    error.to_string(),
                )
            })?;

        cancellation.check()?;

        let result = self
            .transport
            .read_results(
                &request_id,
                cancellation,
            )
            .map_err(|error| {
                qpu_failure(
                    QPU_RESULT_RETRIEVAL_OPERATION,
                    error.to_string(),
                )
            })?;

        cancellation.check()?;

        self.validate_result(
            &request,
            &result,
            submitted_at.elapsed(),
        )?;

        cancellation.check()?;

        self.result_to_syndrome(
            &request,
            &result,
        )
    }

    /// Performs capability/resource/backend preflight without submitting work.
    pub fn preflight(
        &self,
        circuit: &SyndromeExtractionCircuit,
        shots: u64,
    ) -> QecResult<()> {
        self.preflight_capabilities()?;
        self.preflight_circuit(circuit, shots)
    }

    /// Returns the backend technical capabilities.
    pub fn backend_capabilities(
        &self,
    ) -> QecResult<BackendCapabilities> {
        self.transport.capabilities().map_err(|error| {
            qpu_failure(
                "backend_capabilities",
                error.to_string(),
            )
        })
    }

    fn preflight_capabilities(&self) -> QecResult<()> {
        require_capability(
            &self.config.capabilities,
            Capability::QpuAccess,
        )?;

        require_capability(
            &self.config.capabilities,
            Capability::QpuSubmit,
        )?;

        require_capability(
            &self.config.capabilities,
            Capability::QpuReadResults,
        )?;

        require_capability(
            &self.config.capabilities,
            Capability::QpuSyndromeExtraction,
        )?;

        if self.config.deterministic {
            require_capability(
                &self.config.capabilities,
                Capability::DeterministicExecution,
            )?;
        }

        let backend = self.backend_capabilities()?;

        validate_qpu_backend(&backend)?;

        if self.config.deterministic
            && !backend.deterministic_execution
        {
            return Err(unsupported_configuration(
                "deterministic QPU execution",
                "the selected physical backend does not guarantee deterministic execution",
            ));
        }

        Ok(())
    }

    fn preflight_circuit(
        &self,
        circuit: &SyndromeExtractionCircuit,
        shots: u64,
    ) -> QecResult<()> {
        self.preflight_capabilities()?;

        if shots == 0 {
            return Err(invalid_input(
                "QPU shot count must be greater than zero",
            ));
        }

        if shots > self.config.limits.max_qpu_shots {
            return Err(resource_limit(
                ResourceKind::QpuShots,
                shots as u128,
                0,
                self.config.limits.max_qpu_shots as u128,
                "requested QPU shots exceed the canonical QEC limit",
            ));
        }

        if self.config.limits.max_qpu_circuits < 1 {
            return Err(resource_limit(
                ResourceKind::QpuCircuits,
                1,
                0,
                self.config.limits.max_qpu_circuits as u128,
                "QPU circuit execution is disabled by resource policy",
            ));
        }

        if circuit.qubit_count()
            > self.config.limits.max_qubits
        {
            return Err(resource_limit(
                ResourceKind::Qubits,
                circuit.qubit_count() as u128,
                0,
                self.config.limits.max_qubits as u128,
                "QPU circuit requires more qubits than permitted",
            ));
        }

        if circuit.rounds()
            > self.config.limits.max_rounds
        {
            return Err(resource_limit(
                ResourceKind::MeasurementRounds,
                circuit.rounds() as u128,
                0,
                self.config.limits.max_rounds as u128,
                "QPU circuit requires more rounds than permitted",
            ));
        }

        if circuit.measured_stabilizers().len()
            > self.config.limits.max_stabilizers
        {
            return Err(resource_limit(
                ResourceKind::Stabilizers,
                circuit.measured_stabilizers().len() as u128,
                0,
                self.config.limits.max_stabilizers as u128,
                "QPU circuit measures more stabilizers than permitted",
            ));
        }

        if circuit.measured_stabilizers().len()
            > self.config.limits.max_syndrome_events
        {
            return Err(resource_limit(
                ResourceKind::SyndromeEvents,
                circuit.measured_stabilizers().len() as u128,
                0,
                self.config.limits.max_syndrome_events as u128,
                "QPU circuit exceeds the syndrome-event policy",
            ));
        }

        let backend = self.backend_capabilities()?;

        if let Some(max_qubits) = backend_qpu_qubit_capacity(&backend) {
            if circuit.qubit_count() > max_qubits {
                return Err(resource_limit(
                    ResourceKind::Qubits,
                    circuit.qubit_count() as u128,
                    0,
                    max_qubits as u128,
                    "QPU circuit exceeds backend qubit capacity",
                ));
            }
        }

        if !backend.measurement {
            return Err(unsupported_configuration(
                "QPU measurement",
                "selected backend does not advertise measurement support",
            ));
        }

        if !backend.mid_circuit_measurement {
            return Err(unsupported_configuration(
                "QPU syndrome extraction",
                "selected backend does not advertise mid-circuit measurement support",
            ));
        }

        if !backend.reset {
            return Err(unsupported_configuration(
                "QPU syndrome extraction",
                "selected backend does not advertise qubit reset support",
            ));
        }

        Ok(())
    }

    fn validate_result(
        &self,
        request: &QpuExecutionRequest,
        result: &QpuExecutionResult,
        elapsed: Duration,
    ) -> QecResult<()> {
        if result.request_id().is_empty() {
            return Err(qpu_failure(
                "result_validation",
                "QPU returned an empty request identifier",
            ));
        }

        if result.shots_completed() == 0 {
            return Err(qpu_failure(
                "result_validation",
                "QPU reported zero completed shots",
            ));
        }

        if result.shots_completed() > request.shots() {
            return Err(qpu_failure(
                "result_validation",
                "QPU reported more completed shots than requested",
            ));
        }

        if result.measurements().len()
            > self.config.limits.max_syndrome_events
        {
            return Err(resource_limit(
                ResourceKind::SyndromeEvents,
                result.measurements().len() as u128,
                0,
                self.config.limits.max_syndrome_events as u128,
                "QPU result exceeds syndrome-event policy",
            ));
        }

        if elapsed.is_zero() && !result.elapsed().is_zero() {
            // The adapter does not reject a valid provider duration merely
            // because local elapsed timing is unavailable.
        }

        let expected: BTreeSet<StabilizerId> = request
            .circuit()
            .measured_stabilizers()
            .iter()
            .copied()
            .collect();

        let mut received = BTreeSet::new();

        for measurement in result.measurements() {
            if !expected.contains(&measurement.stabilizer()) {
                return Err(invalid_syndrome(format!(
                    "QPU returned unexpected stabilizer {}",
                    measurement.stabilizer()
                )));
            }

            if !received.insert(measurement.stabilizer()) {
                return Err(invalid_syndrome(format!(
                    "QPU returned duplicate measurement for stabilizer {}",
                    measurement.stabilizer()
                )));
            }
        }

        if received != expected {
            return Err(invalid_syndrome(
                "QPU result does not contain exactly one measurement for every requested stabilizer",
            ));
        }

        Ok(())
    }

    fn result_to_syndrome(
        &self,
        request: &QpuExecutionRequest,
        result: &QpuExecutionResult,
    ) -> QecResult<Syndrome> {
        let measurements = result
            .measurements()
            .iter()
            .map(|measurement| {
                SyndromeMeasurement::new(
                    measurement.stabilizer(),
                    measurement.value(),
                    measurement.confidence(),
                )
            });

        Syndrome::from_measurements(
            request.round(),
            request.timestamp(),
            measurements,
            SyndromeOptions::with_limits(
                self.config.limits,
            )
            .require_non_empty(),
        )
    }
}

// ============================================================================
// Capability helpers
// ============================================================================

fn require_capability(
    capabilities: &CapabilitySet,
    capability: Capability,
) -> QecResult<()> {
    if capabilities.contains(capability) {
        return Ok(());
    }

    Err(QecError::CapabilityDenied {
        capability: capability.name().to_owned(),
        operation: QPU_SYNDROME_EXTRACTION_OPERATION
            .to_owned(),
        message: format!(
            "required capability {} was not granted",
            capability.name()
        ),
    })
}

// ============================================================================
// Backend validation
// ============================================================================

fn validate_qpu_backend(
    capabilities: &BackendCapabilities,
) -> QecResult<()> {
    if !capabilities.physical_qpu {
        return Err(unsupported_configuration(
            "physical QPU execution",
            "backend does not advertise a physical QPU",
        ));
    }

    if !capabilities.qec_execution {
        return Err(unsupported_configuration(
            "QEC execution",
            "backend does not advertise QEC execution",
        ));
    }

    if !capabilities.cancellation {
        return Err(unsupported_configuration(
            "QPU cancellation",
            "backend does not advertise cooperative cancellation",
        ));
    }

    Ok(())
}

/// Extracts an optional backend-specific qubit capacity from native
/// operation metadata when a backend chooses to publish it.
///
/// Native operation names are not treated as authorization and therefore are
/// never interpreted as capabilities.
fn backend_qpu_qubit_capacity(
    _capabilities: &BackendCapabilities,
) -> Option<usize> {
    None
}

// ============================================================================
// Circuit validation
// ============================================================================

fn validate_operations(
    qubit_count: usize,
    operations: &[QpuOperation],
    measured_stabilizers: &BTreeSet<StabilizerId>,
) -> QecResult<()> {
    for operation in operations {
        match operation {
            QpuOperation::Reset { qubit }
            | QpuOperation::SingleQubit {
                qubit,
                ..
            } => {
                validate_qubit_index(
                    *qubit,
                    qubit_count,
                )?;
            }

            QpuOperation::TwoQubit {
                control,
                target,
                ..
            } => {
                validate_qubit_index(
                    *control,
                    qubit_count,
                )?;

                validate_qubit_index(
                    *target,
                    qubit_count,
                )?;

                if control == target {
                    return Err(invalid_input(
                        "two-qubit operation cannot use the same qubit as control and target",
                    ));
                }
            }

            QpuOperation::MeasureStabilizer {
                stabilizer,
                qubit,
            } => {
                validate_qubit_index(
                    *qubit,
                    qubit_count,
                )?;

                if !measured_stabilizers.contains(stabilizer) {
                    return Err(invalid_syndrome(format!(
                        "measurement operation references unregistered stabilizer {}",
                        stabilizer
                    )));
                }
            }

            QpuOperation::Barrier => {}
        }
    }

    Ok(())
}

fn validate_qubit_index(
    qubit: usize,
    qubit_count: usize,
) -> QecResult<()> {
    if qubit >= qubit_count {
        return Err(invalid_input(format!(
            "QPU qubit index {qubit} is outside circuit size {qubit_count}"
        )));
    }

    Ok(())
}

// ============================================================================
// Error constructors
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> QecError {
    QecError::InvalidInput {
        message: message.into(),
    }
}

fn invalid_syndrome(
    message: impl Into<String>,
) -> QecError {
    QecError::InvalidSyndrome {
        message: message.into(),
    }
}

fn unsupported_configuration(
    feature: impl Into<String>,
    message: impl Into<String>,
) -> QecError {
    QecError::UnsupportedConfiguration {
        feature: feature.into(),
        message: message.into(),
    }
}

fn qpu_failure(
    operation: impl Into<String>,
    message: impl Into<String>,
) -> QecError {
    QecError::QpuFailure {
        backend: "physical_qpu".to_owned(),
        operation: operation.into(),
        message: message.into(),
    }
}

fn resource_limit(
    resource: ResourceKind,
    requested: u128,
    current: u128,
    limit: u128,
    message: impl Into<String>,
) -> QecError {
    QecError::ResourceLimitExceeded {
        resource,
        requested,
        current,
        limit,
        message: message.into(),
    }
}

fn normalize_identifier(
    value: String,
) -> String {
    value.trim().to_owned()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct MockQpu {
        identity: QpuIdentity,
        capabilities: BackendCapabilities,
        result: QpuExecutionResult,
    }

    impl QpuTransport for MockQpu {
        fn identity(&self) -> QecResult<QpuIdentity> {
            Ok(self.identity.clone())
        }

        fn capabilities(
            &self,
        ) -> QecResult<BackendCapabilities> {
            Ok(self.capabilities.clone())
        }

        fn submit(
            &self,
            _request: &QpuExecutionRequest,
            cancellation: &CancellationToken,
        ) -> Result<String, QpuTransportError> {
            cancellation.check().map_err(|error| {
                QpuTransportError::new(
                    QPU_CIRCUIT_SUBMISSION_OPERATION,
                    error.to_string(),
                )
            })?;

            Ok(self.result.request_id().to_owned())
        }

        fn read_results(
            &self,
            request_id: &str,
            cancellation: &CancellationToken,
        ) -> Result<QpuExecutionResult, QpuTransportError> {
            cancellation.check().map_err(|error| {
                QpuTransportError::new(
                    QPU_RESULT_RETRIEVAL_OPERATION,
                    error.to_string(),
                )
            })?;

            if request_id != self.result.request_id() {
                return Err(QpuTransportError::new(
                    QPU_RESULT_RETRIEVAL_OPERATION,
                    "unknown request identifier",
                ));
            }

            Ok(self.result.clone())
        }
    }

    fn test_capabilities() -> CapabilitySet {
        CapabilitySet::from_iter([
            Capability::QpuAccess,
            Capability::QpuSubmit,
            Capability::QpuReadResults,
            Capability::QpuSyndromeExtraction,
        ])
    }

    fn test_backend_capabilities() -> BackendCapabilities {
        BackendCapabilities {
            qec_execution: true,
            syndrome_generation: true,
            decoding: false,
            simulation: false,
            pauli_frame: false,
            streaming: false,
            partitioning: false,
            distributed: false,
            acceleration: false,
            checkpointing: false,
            cancellation: true,
            deterministic_execution: true,
            physical_qpu: true,
            calibration: false,
            mid_circuit_measurement: true,
            reset: true,
            measurement: true,
            dynamic_circuits: true,
            classical_control: true,
            native_operations: BTreeSet::new(),
        }
    }

    fn test_circuit() -> SyndromeExtractionCircuit {
        SyndromeExtractionCircuit::new(
            2,
            1,
            vec![
                QpuOperation::Reset { qubit: 0 },
                QpuOperation::Reset { qubit: 1 },
                QpuOperation::MeasureStabilizer {
                    stabilizer: StabilizerId::new(0),
                    qubit: 0,
                },
            ],
            vec![StabilizerId::new(0)],
        )
        .expect("test circuit must be valid")
    }

    fn test_adapter() -> QpuAdapter {
        let identity = QpuIdentity::new(
            "mock-qpu",
            "test-provider",
            "v1",
        )
        .expect("identity");

        let round =
            MeasurementRound::new(0).expect("round");

        let timestamp =
            MeasurementTimestamp::new(0)
                .expect("timestamp");

        let measurement =
            QpuMeasurement::new(
                StabilizerId::new(0),
                true,
                MeasurementConfidence::FULL,
            );

        let result = QpuExecutionResult::new(
            "request-1",
            1,
            vec![measurement],
            identity.clone(),
            Duration::from_millis(1),
        )
        .expect("result");

        let transport = MockQpu {
            identity,
            capabilities: test_backend_capabilities(),
            result,
        };

        let config = QpuAdapterConfig::new(
            QecLimits::default(),
            test_capabilities(),
        );

        let adapter =
            QpuAdapter::new(
                Arc::new(transport),
                config,
            )
            .expect("adapter");

        let _ = round;
        let _ = timestamp;

        adapter
    }

    #[test]
    fn adapter_accepts_authorized_qpu() {
        let adapter = test_adapter();

        let circuit = test_circuit();

        adapter
            .preflight(&circuit, 1)
            .expect("preflight");
    }

    #[test]
    fn adapter_rejects_missing_submit_capability() {
        let identity = QpuIdentity::new(
            "mock-qpu",
            "test-provider",
            "v1",
        )
        .expect("identity");

        let result = QpuExecutionResult::new(
            "request-1",
            1,
            Vec::new(),
            identity.clone(),
            Duration::from_millis(1),
        )
        .expect("result");

        let transport = MockQpu {
            identity,
            capabilities: test_backend_capabilities(),
            result,
        };

        let capabilities = CapabilitySet::from_iter([
            Capability::QpuAccess,
            Capability::QpuReadResults,
            Capability::QpuSyndromeExtraction,
        ]);

        let config = QpuAdapterConfig::new(
            QecLimits::default(),
            capabilities,
        );

        let adapter =
            QpuAdapter::new(
                Arc::new(transport),
                config,
            )
            .expect("adapter");

        let error = adapter
            .preflight(&test_circuit(), 1)
            .expect_err("missing submit capability must fail");

        assert!(matches!(
            error,
            QecError::CapabilityDenied { .. }
        ));
    }

    #[test]
    fn adapter_rejects_unavailable_physical_qpu() {
        let identity = QpuIdentity::new(
            "mock-qpu",
            "test-provider",
            "v1",
        )
        .expect("identity");

        let result = QpuExecutionResult::new(
            "request-1",
            1,
            Vec::new(),
            identity.clone(),
            Duration::from_millis(1),
        )
        .expect("result");

        let mut capabilities =
            test_backend_capabilities();

        capabilities.physical_qpu = false;

        let transport = MockQpu {
            identity,
            capabilities,
            result,
        };

        let config = QpuAdapterConfig::new(
            QecLimits::default(),
            test_capabilities(),
        );

        let error =
            QpuAdapter::new(
                Arc::new(transport),
                config,
            )
            .expect_err(
                "non-QPU backend must be rejected",
            );

        assert!(matches!(
            error,
            QecError::UnsupportedConfiguration { .. }
        ));
    }

    #[test]
    fn adapter_rejects_duplicate_stabilizers() {
        let result =
            SyndromeExtractionCircuit::new(
                2,
                1,
                vec![
                    QpuOperation::MeasureStabilizer {
                        stabilizer: StabilizerId::new(0),
                        qubit: 0,
                    },
                ],
                vec![
                    StabilizerId::new(0),
                    StabilizerId::new(0),
                ],
            );

        assert!(result.is_err());
    }

    #[test]
    fn adapter_rejects_invalid_qubit_indices() {
        let result =
            SyndromeExtractionCircuit::new(
                1,
                1,
                vec![
                    QpuOperation::Reset { qubit: 2 },
                ],
                vec![StabilizerId::new(0)],
            );

        assert!(result.is_err());
    }

    #[test]
    fn adapter_rejects_same_qubit_two_qubit_operation() {
        let result =
            SyndromeExtractionCircuit::new(
                2,
                1,
                vec![
                    QpuOperation::TwoQubit {
                        name: "entangle".to_owned(),
                        control: 0,
                        target: 0,
                    },
                ],
                vec![StabilizerId::new(0)],
            );

        assert!(result.is_err());
    }

    #[test]
    fn adapter_preserves_fail_closed_cancellation() {
        let adapter = test_adapter();

        let source = super::super::cancellation::CancellationSource::new();
        let token = source.token();

        source.cancel();

        let round =
            MeasurementRound::new(0).expect("round");

        let timestamp =
            MeasurementTimestamp::new(0)
                .expect("timestamp");

        let result = adapter.extract_syndrome(
            test_circuit(),
            1,
            round,
            timestamp,
            &token,
        );

        assert!(matches!(
            result,
            Err(QecError::CancellationRequested { .. })
        ));
    }
}