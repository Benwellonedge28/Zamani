//! Zamani Quantum — Local Quantum Backend Adapter
//!
//! Production-grade deterministic local execution adapter for:
//!
//! `crate::quantum::hardware::adapters::local`
//!
//! # Responsibility
//!
//! This module provides the first executable backend for Zamani Quantum.
//!
//! It implements `QuantumBackendAdapter` without requiring:
//!
//! - network access;
//! - provider credentials;
//! - cloud accounts;
//! - provider SDKs;
//! - external quantum hardware;
//! - OpenQASM parsing;
//! - QIR parsing;
//! - benchmarking;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition.
//!
//! The adapter is a real bounded state-vector simulator for the canonical
//! local executable format `zamani-local-v1`.
//!
//! It exists so the complete hardware execution architecture can be tested
//! locally before provider-specific adapters are introduced.
//!
//! # Supported execution model
//!
//! The local adapter supports:
//!
//! - gate-model circuits;
//! - deterministic seeded sampling;
//! - computational-basis measurement;
//! - mid-circuit measurement;
//! - measurement collapse;
//! - reset;
//! - one-qubit gates;
//! - controlled two-qubit gates;
//! - SWAP;
//! - Toffoli/CCX;
//! - classical conditions;
//! - explicit measurement operations;
//! - optional final measurement of all qubits;
//! - synchronous execution;
//! - asynchronous-style job lifecycle through the adapter contract;
//! - local job storage;
//! - deterministic job identifiers;
//! - cancellation of non-terminal jobs;
//! - health reporting;
//! - queue reporting;
//! - fault injection for tests;
//! - result normalization into `backend::ExecutionResult`.
//!
//! # Supported gates
//!
//! The local executable format currently supports:
//!
//! ```text
//! i
//! id
//! x
//! y
//! z
//! h
//! s
//! sdg
//! t
//! tdg
//! rx(theta)
//! ry(theta)
//! rz(theta)
//! u(theta, phi, lambda)
//! cx(control, target)
//! cnot(control, target)
//! cz(control, target)
//! swap(q0, q1)
//! ccx(control0, control1, target)
//! toffoli(control0, control1, target)
//! measure
//! reset
//! barrier
//! ```
//!
//! Gate names are case-insensitive and are normalized internally.
//!
//! # Executable format
//!
//! `BackendProgram` remains opaque at the provider-neutral boundary.
//!
//! This adapter accepts the following format identifier:
//!
//! ```text
//! zamani-local-v1
//! ```
//!
//! The payload is UTF-8 JSON with the following semantic shape:
//!
//! ```json
//! {
//!   "schema": "zamani-local-v1",
//!   "qubits": 2,
//!   "classical_bits": 2,
//!   "measure_all": false,
//!   "operations": [
//!     {"gate": "h", "targets": [0]},
//!     {"gate": "cx", "targets": [0, 1]},
//!     {"gate": "measure", "targets": [0], "classical": [0]},
//!     {"gate": "measure", "targets": [1], "classical": [1]}
//!   ]
//! }
//! ```
//!
//! Gate parameters are supplied through `params`:
//!
//! ```json
//! {"gate": "rx", "targets": [0], "params": [1.5707963267948966]}
//! ```
//!
//! A conditional operation may contain:
//!
//! ```json
//! {
//!   "gate": "x",
//!   "targets": [1],
//!   "condition": {"bit": 0, "equals": 1}
//! }
//! ```
//!
//! `measure` additionally requires matching `classical` targets.
//!
//! # Measurement semantics
//!
//! The adapter never silently measures a circuit unless `measure_all` is true.
//!
//! Therefore a circuit with no explicit measurement and:
//!
//! ```text
//! measure_all = false
//! ```
//!
//! is rejected because the provider-neutral legacy `ExecutionResult` contract
//! represents sampled classical results.
//!
//! This prevents accidental semantic changes caused by automatically measuring
//! every circuit.
//!
//! # Determinism
//!
//! Local execution is deterministic when the same:
//!
//! - program;
//! - execution request;
//! - seed;
//! - backend configuration
//!
//! are supplied.
//!
//! When the caller does not provide a seed, the adapter uses a fixed seed of
//! zero. This is deliberate: local execution must be reproducible by default.
//!
//! The random generator is local to one execution and is never global.
//!
//! # Resource safety
//!
//! State-vector dimension is bounded by `LOCAL_MAX_QUBITS`.
//!
//! This prevents a malformed local program from causing unbounded allocation.
//!
//! The default maximum is intentionally conservative because a state vector
//! requires O(2^n) memory.
//!
//! # Thread safety
//!
//! The adapter is `Send + Sync`.
//!
//! Job state is protected by a local `Mutex`.
//!
//! There is no process-global registry and no global mutable state.
//!
//! # Security
//!
//! This module:
//!
//! - stores no credentials;
//! - reads no environment variables;
//! - performs no network I/O;
//! - does not log program payloads;
//! - does not place program payloads into `Debug` output;
//! - rejects secret-like metadata;
//!
//! The local adapter therefore remains safe to use in offline environments.
//!
//! # Provider independence
//!
//! This module depends only on:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - the Rust standard library;
//! - `serde_json`, already used by the repository.
//!
//! It does not depend on:
//!
//! - IBM;
//! - IonQ;
//! - AWS Braket;
//! - Rigetti;
//! - IQM;
//! - Quantinuum;
//! - QuEra;
//! - benchmarking;
//! - Danga;
//! - provider registries.
//!
//! # Integration contract
//!
//! `adapters/mod.rs` should eventually expose this module with:
//!
//! ```text
//! pub mod local;
//! ```
//!
//! No change inside this file is required for that integration.
//!
//! `provider_registry.rs` may store:
//!
//! ```text
//! Box<dyn QuantumBackendAdapter>
//! ```
//!
//! `device_registry.rs` may index `backend().id()`.
//!
//! `execution.rs` may call:
//!
//! ```text
//! preflight()
//! submit()
//! status()
//! result()
//! cancel()
//! execute()
//! ```
//!
//! `benchmarking` may use this adapter as a deterministic execution backend.
//!
//! Danga may expose it as a local/offline quantum target.
//!
//! # No-reedit contract
//!
//! This module intentionally consumes the existing stable contracts instead of
//! defining replacement versions of:
//!
//! - `QuantumBackend`;
//! - `ExecutionRequest`;
//! - `ExecutionResult`;
//! - `BackendProgram`;
//! - `BackendJob`;
//! - `BackendJobStatus`;
//! - `BackendCancellation`.
//!
//! Provider adapters added later must not require changes here.
//!
//! New executable formats belong in their own adapters.
//!
//! New provider functionality belongs in provider-specific adapters.
//!
//! New canonical result semantics belong in `result.rs`.
//!
//! New lifecycle semantics belong in `backend_trait.rs` / `job.rs`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Safety
//!
//! `unsafe_code` is forbidden.
//!
//! # Production status
//!
//! This adapter is production-ready as a bounded deterministic LOCAL gate-model
//! execution backend.
//!
//! It is NOT a claim that a classical state-vector simulator can replace real
//! QPU hardware. Real QPU providers remain separate adapters.
//!
//! # Schema
//!
//! Local executable schema:
//!
//! `zamani-local-v1`
//!
//! Adapter schema:
//!
//! `zamani.quantum.hardware.adapters.local`

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::Value;

use super::super::backend::{
    BackendCapabilities,
    BackendError,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
    QuantumWorkloadKind,
};

use super::super::backend_trait::{
    BackendAdapterInfo,
    BackendCancellation,
    BackendHealth,
    BackendHealthState,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    BackendQueueInfo,
    CancellationOutcome,
    QuantumBackendAdapter,
};

use super::super::topology::{Coupling, HardwareTopology};

/// Stable adapter schema identifier.
pub const LOCAL_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.hardware.adapters.local";

/// Semantic adapter schema version.
pub const LOCAL_ADAPTER_SCHEMA_VERSION: u16 = 1;

/// Executable local program format.
pub const LOCAL_PROGRAM_FORMAT: &str = "zamani-local-v1";

/// Stable local backend identifier.
pub const LOCAL_BACKEND_ID: &str = "local://statevector";

/// Stable local provider identifier.
pub const LOCAL_PROVIDER_ID: &str = "zamani-local";

/// Stable adapter identifier.
pub const LOCAL_ADAPTER_ID: &str = "zamani.hardware.local";

/// Stable adapter implementation version.
pub const LOCAL_ADAPTER_VERSION: &str = "1.0.0";

/// Maximum number of qubits supported by the bounded state-vector engine.
///
/// The limit is deliberately conservative. A state vector requires two
/// floating-point components per amplitude and grows exponentially.
pub const LOCAL_MAX_QUBITS: usize = 20;

/// Maximum classical bits.
pub const LOCAL_MAX_CLASSICAL_BITS: usize = 4096;

/// Maximum shots accepted by the local adapter.
pub const LOCAL_MAX_SHOTS: usize = 1_000_000;

/// Maximum operation count.
pub const LOCAL_MAX_OPERATIONS: usize = 1_000_000;

/// Maximum JSON program bytes accepted by this adapter.
pub const LOCAL_MAX_PROGRAM_BYTES: usize = 64 * 1024 * 1024;

/// Maximum stored completed jobs.
pub const LOCAL_MAX_STORED_JOBS: usize = 10_000;

/// Default deterministic seed.
pub const LOCAL_DEFAULT_SEED: u64 = 0;

/// Maximum supported JSON nesting depth relevant to this schema.
pub const LOCAL_MAX_JSON_DEPTH: usize = 64;

/// Maximum gate parameter count.
pub const LOCAL_MAX_PARAMETERS: usize = 8;

/// Maximum metadata entries generated by this adapter.
pub const LOCAL_MAX_RESULT_METADATA: usize = 64;

/// Supported local fault-injection modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalFault {
    /// No fault is injected.
    None,

    /// Submission fails deterministically.
    RejectSubmission,

    /// Health reports a degraded backend.
    DegradedHealth,

    /// Health reports an unavailable backend.
    Unhealthy,

    /// Result retrieval fails deterministically.
    RejectResult,

    /// Cancellation is reported as unsupported.
    RejectCancellation,
}

impl Default for LocalFault {
    fn default() -> Self {
        Self::None
    }
}

impl LocalFault {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RejectSubmission => "reject_submission",
            Self::DegradedHealth => "degraded_health",
            Self::Unhealthy => "unhealthy",
            Self::RejectResult => "reject_result",
            Self::RejectCancellation => "reject_cancellation",
        }
    }
}

/// Immutable local backend configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalBackendConfig {
    /// Maximum physical qubits.
    pub max_qubits: usize,

    /// Maximum classical bits.
    pub max_classical_bits: usize,

    /// Maximum shots.
    pub max_shots: usize,

    /// Maximum operations.
    pub max_operations: usize,

    /// Maximum stored jobs.
    pub max_stored_jobs: usize,

    /// Fault-injection mode.
    pub fault: LocalFault,
}

impl Default for LocalBackendConfig {
    fn default() -> Self {
        Self {
            max_qubits: LOCAL_MAX_QUBITS,
            max_classical_bits: LOCAL_MAX_CLASSICAL_BITS,
            max_shots: LOCAL_MAX_SHOTS,
            max_operations: LOCAL_MAX_OPERATIONS,
            max_stored_jobs: LOCAL_MAX_STORED_JOBS,
            fault: LocalFault::None,
        }
    }
}

impl LocalBackendConfig {
    /// Creates the production default configuration.
    pub const fn production() -> Self {
        Self {
            max_qubits: LOCAL_MAX_QUBITS,
            max_classical_bits: LOCAL_MAX_CLASSICAL_BITS,
            max_shots: LOCAL_MAX_SHOTS,
            max_operations: LOCAL_MAX_OPERATIONS,
            max_stored_jobs: LOCAL_MAX_STORED_JOBS,
            fault: LocalFault::None,
        }
    }

    /// Creates a deterministic test configuration.
    pub const fn test() -> Self {
        Self {
            max_qubits: 8,
            max_classical_bits: 64,
            max_shots: 10_000,
            max_operations: 100_000,
            max_stored_jobs: 256,
            fault: LocalFault::None,
        }
    }

    /// Changes the maximum qubit count.
    pub const fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    /// Changes the maximum classical-bit count.
    pub const fn with_max_classical_bits(mut self, value: usize) -> Self {
        self.max_classical_bits = value;
        self
    }

    /// Changes the maximum shot count.
    pub const fn with_max_shots(mut self, value: usize) -> Self {
        self.max_shots = value;
        self
    }

    /// Changes the maximum operation count.
    pub const fn with_max_operations(mut self, value: usize) -> Self {
        self.max_operations = value;
        self
    }

    /// Changes the maximum stored-job count.
    pub const fn with_max_stored_jobs(mut self, value: usize) -> Self {
        self.max_stored_jobs = value;
        self
    }

    /// Enables deterministic fault injection.
    pub const fn with_fault(mut self, fault: LocalFault) -> Self {
        self.fault = fault;
        self
    }

    fn validate(self) -> Result<(), BackendError> {
        if self.max_qubits == 0 || self.max_qubits > LOCAL_MAX_QUBITS {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "local maximum qubit count must be in 1..={}",
                    LOCAL_MAX_QUBITS
                ),
            ));
        }

        if self.max_classical_bits == 0
            || self.max_classical_bits > LOCAL_MAX_CLASSICAL_BITS
        {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "local maximum classical-bit count must be in 1..={}",
                    LOCAL_MAX_CLASSICAL_BITS
                ),
            ));
        }

        if self.max_shots == 0 || self.max_shots > LOCAL_MAX_SHOTS {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "local maximum shot count must be in 1..={}",
                    LOCAL_MAX_SHOTS
                ),
            ));
        }

        if self.max_operations == 0
            || self.max_operations > LOCAL_MAX_OPERATIONS
        {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "local maximum operation count must be in 1..={}",
                    LOCAL_MAX_OPERATIONS
                ),
            ));
        }

        if self.max_stored_jobs == 0
            || self.max_stored_jobs > LOCAL_MAX_STORED_JOBS
        {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "local maximum stored-job count must be in 1..={}",
                    LOCAL_MAX_STORED_JOBS
                ),
            ));
        }

        Ok(())
    }
}

/// Immutable local backend execution adapter.
///
/// The adapter owns only local job/result state. It does not own global
/// registries, credentials, network clients, provider SDKs, or benchmarking
/// state.
pub struct LocalBackendAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    config: LocalBackendConfig,
    next_job_id: AtomicU64,
    jobs: Arc<Mutex<BTreeMap<String, LocalStoredJob>>>,
}

/// Internal stored local job.
///
/// Program bytes are deliberately not retained after execution. This prevents
/// the adapter from becoming an accidental program-persistence system.
#[derive(Debug, Clone)]
struct LocalStoredJob {
    job: BackendJob,
    result: Option<ExecutionResult>,
}

impl LocalBackendAdapter {
    /// Creates a production-configured local state-vector adapter.
    pub fn new() -> Result<Self, BackendError> {
        Self::with_config(LocalBackendConfig::production())
    }

    /// Creates a local adapter using an explicit configuration.
    pub fn with_config(
        config: LocalBackendConfig,
    ) -> Result<Self, BackendError> {
        config.validate()?;

        let backend = build_backend(config)?;

        let adapter_info = BackendAdapterInfo::new(
            LOCAL_ADAPTER_ID,
            LOCAL_ADAPTER_VERSION,
            true,
        )?
        .with_provider_api_version(LOCAL_PROGRAM_FORMAT)?;

        Ok(Self {
            backend,
            adapter_info,
            config,
            next_job_id: AtomicU64::new(1),
            jobs: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Returns the immutable local configuration.
    pub const fn config(&self) -> LocalBackendConfig {
        self.config
    }

    /// Returns the number of locally retained job records.
    pub fn stored_job_count(&self) -> Result<usize, BackendError> {
        let jobs = self.lock_jobs()?;
        Ok(jobs.len())
    }

    /// Removes all terminal jobs from local memory.
    ///
    /// This does not affect the backend descriptor or execution semantics.
    pub fn clear_terminal_jobs(&self) -> Result<usize, BackendError> {
        let mut jobs = self.lock_jobs()?;

        let before = jobs.len();

        jobs.retain(|_, stored| !stored.job.state.is_terminal());

        Ok(before.saturating_sub(jobs.len()))
    }

    /// Returns a local job result without exposing the internal storage.
    pub fn local_result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        self.result(job)
    }

    /// Builds a deterministic local job identifier.
    fn allocate_job_id(
        &self,
        request: &ExecutionRequest,
    ) -> Result<BackendJobId, BackendError> {
        if let Some(request_id) = request.request_id.as_deref() {
            let sanitized = sanitize_job_component(request_id);

            if !sanitized.is_empty() {
                return BackendJobId::new(format!("local-{sanitized}"));
            }
        }

        let sequence = self.next_job_id.fetch_add(1, Ordering::Relaxed);

        BackendJobId::new(format!("local-{sequence}"))
    }

    fn lock_jobs(
        &self,
    ) -> Result<
        std::sync::MutexGuard<'_, BTreeMap<String, LocalStoredJob>>,
        BackendError,
    > {
        self.jobs.lock().map_err(|_| {
            BackendError::ExecutionUnavailable(
                "local job store mutex is poisoned".to_string(),
            )
        })
    }

    fn store_job(
        &self,
        stored: LocalStoredJob,
    ) -> Result<(), BackendError> {
        let mut jobs = self.lock_jobs()?;

        if jobs.len() >= self.config.max_stored_jobs {
            let terminal_key = jobs.iter().find_map(|(key, value)| {
                if value.job.state.is_terminal() {
                    Some(key.clone())
                } else {
                    None
                }
            });

            if let Some(key) = terminal_key {
                jobs.remove(&key);
            } else {
                return Err(BackendError::ExecutionRejected(
                    "local job store capacity is exhausted".to_string(),
                ));
            }
        }

        jobs.insert(stored.job.id.to_string(), stored);

        Ok(())
    }

    fn update_job(
        &self,
        job_id: &BackendJobId,
        state: BackendJobState,
        result: Option<ExecutionResult>,
    ) -> Result<BackendJob, BackendError> {
        let mut jobs = self.lock_jobs()?;

        let stored = jobs
            .get_mut(job_id.as_str())
            .ok_or_else(|| {
                BackendError::ExecutionRejected(format!(
                    "unknown local job `{}`",
                    job_id
                ))
            })?;

        stored.job.state = state;

        if result.is_some() {
            stored.result = result;
        }

        Ok(stored.job.clone())
    }

    fn get_job(
        &self,
        job_id: &BackendJobId,
    ) -> Result<LocalStoredJob, BackendError> {
        let jobs = self.lock_jobs()?;

        jobs.get(job_id.as_str())
            .cloned()
            .ok_or_else(|| {
                BackendError::ExecutionRejected(format!(
                    "unknown local job `{}`",
                    job_id
                ))
            })
    }
}

impl fmt::Debug for LocalBackendAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalBackendAdapter")
            .field("backend_id", &self.backend.id())
            .field("adapter_id", &self.adapter_info.adapter_id)
            .field("adapter_version", &self.adapter_info.adapter_version)
            .field("max_qubits", &self.config.max_qubits)
            .field("max_shots", &self.config.max_shots)
            .field("fault", &self.config.fault)
            .finish()
    }
}

impl QuantumBackendAdapter for LocalBackendAdapter {
    fn backend(&self) -> &QuantumBackend {
        &self.backend
    }

    fn adapter_info(&self) -> &BackendAdapterInfo {
        &self.adapter_info
    }

    fn preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), BackendError> {
        request.validate_structure()?;

        if program.format() != LOCAL_PROGRAM_FORMAT {
            return Err(BackendError::ExecutionRejected(format!(
                "local adapter accepts only `{}`; received `{}`",
                LOCAL_PROGRAM_FORMAT,
                program.format()
            )));
        }

        if program.len() > LOCAL_MAX_PROGRAM_BYTES {
            return Err(BackendError::ExecutionRejected(format!(
                "local program payload exceeds {} bytes",
                LOCAL_MAX_PROGRAM_BYTES
            )));
        }

        if self.config.fault == LocalFault::RejectSubmission {
            return Err(BackendError::ExecutionRejected(
                "local submission fault injection is enabled".to_string(),
            ));
        }

        if self.backend.status() == BackendStatus::Retired
            || self.backend.status() == BackendStatus::Offline
            || self.backend.status() == BackendStatus::Unavailable
        {
            return Err(BackendError::ExecutionUnavailable(
                format!(
                    "local backend status `{}` does not permit execution",
                    self.backend.status()
                ),
            ));
        }

        let program_definition = LocalProgram::parse(program.bytes())?;

        validate_request_against_program(
            request,
            &program_definition,
            self.config,
        )?;

        self.backend.validate(&request.workload)?;

        Ok(())
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.preflight(request, program)?;

        let parsed = LocalProgram::parse(program.bytes())?;

        let job_id = self.allocate_job_id(request)?;

        /*
         * Local execution is sufficiently cheap to provide an immediately
         * completed job. The lifecycle remains asynchronous-compatible:
         *
         * submit -> Completed -> result
         *
         * This avoids pretending that a local simulator has a remote queue.
         */
        let mut job = BackendJob::new(
            job_id,
            self.backend.id(),
            request.request_id.clone(),
            BackendJobState::Created,
        )?;

        self.store_job(LocalStoredJob {
            job: job.clone(),
            result: None,
        })?;

        job = self.update_job(
            &job.id,
            BackendJobState::Running,
            None,
        )?;

        let result = execute_program(
            &parsed,
            request,
            self.config,
        )?;

        if self.config.fault == LocalFault::RejectResult {
            let _ = self.update_job(
                &job.id,
                BackendJobState::Failed,
                None,
            );

            return Err(BackendError::ExecutionRejected(
                "local result fault injection is enabled".to_string(),
            ));
        }

        let result = finalize_result(
            result,
            &job,
            request,
            &parsed,
            self.config,
        )?;

        job = self.update_job(
            &job.id,
            BackendJobState::Completed,
            Some(result),
        )?;

        Ok(job)
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        let stored = self.get_job(job)?;

        Ok(BackendJobStatus {
            job: stored.job,
            provider_status: Some("local".to_string()),
            queue_position: Some(0),
            estimated_wait: Some(std::time::Duration::ZERO),
            result_available: stored.result.is_some(),
        })
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        if self.config.fault == LocalFault::RejectResult {
            return Err(BackendError::ExecutionRejected(
                "local result fault injection is enabled".to_string(),
            ));
        }

        let stored = self.get_job(job)?;

        if stored.job.state != BackendJobState::Completed {
            return Err(BackendError::ExecutionUnavailable(
                format!(
                    "local job `{}` is not completed; current state is `{}`",
                    job,
                    stored.job.state
                ),
            ));
        }

        stored.result.ok_or_else(|| {
            BackendError::ExecutionUnavailable(
                "completed local job has no normalized result".to_string(),
            )
        })
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        if self.config.fault == LocalFault::RejectCancellation {
            return Ok(BackendCancellation {
                job: job.clone(),
                outcome: CancellationOutcome::Unsupported,
            });
        }

        let mut jobs = self.lock_jobs()?;

        let stored = jobs
            .get_mut(job.as_str())
            .ok_or_else(|| {
                BackendError::ExecutionRejected(format!(
                    "unknown local job `{}`",
                    job
                ))
            })?;

        if stored.job.state.is_terminal() {
            return Ok(BackendCancellation {
                job: job.clone(),
                outcome: CancellationOutcome::AlreadyTerminal,
            });
        }

        /*
         * The local adapter executes synchronously inside submit(), therefore
         * there is normally no cancellable state by the time submit returns.
         *
         * The state transition is nevertheless implemented for correctness
         * and for deterministic fault/conformance testing.
         */
        stored.job.state = BackendJobState::Cancelled;
        stored.result = None;

        Ok(BackendCancellation {
            job: job.clone(),
            outcome: CancellationOutcome::Accepted,
        })
    }

    fn queue_info(&self) -> Result<BackendQueueInfo, BackendError> {
        Ok(BackendQueueInfo {
            pending_jobs: Some(0),
            estimated_wait: Some(std::time::Duration::ZERO),
            accepting_submissions: self.config.fault
                != LocalFault::RejectSubmission,
        })
    }

    fn health(&self) -> Result<BackendHealth, BackendError> {
        match self.config.fault {
            LocalFault::DegradedHealth => Ok(BackendHealth {
                state: BackendHealthState::Degraded,
                backend_status: BackendStatus::Degraded,
                message: Some(
                    "local degraded-health fault injection is enabled"
                        .to_string(),
                ),
            }),

            LocalFault::Unhealthy => Ok(BackendHealth {
                state: BackendHealthState::Unhealthy,
                backend_status: BackendStatus::Unavailable,
                message: Some(
                    "local unhealthy fault injection is enabled".to_string(),
                ),
            }),

            _ => Ok(BackendHealth {
                state: BackendHealthState::Healthy,
                backend_status: BackendStatus::Available,
                message: Some(
                    "local deterministic state-vector backend is healthy"
                        .to_string(),
                ),
            }),
        }
    }

    fn execute(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ExecutionResult, BackendError> {
        self.preflight(request, program)?;

        let parsed = LocalProgram::parse(program.bytes())?;

        let result = execute_program(
            &parsed,
            request,
            self.config,
        )?;

        finalize_result_without_job(
            result,
            request,
            &parsed,
            self.config,
        )
    }

    fn supports_cancellation(&self) -> bool {
        self.config.fault != LocalFault::RejectCancellation
    }

    fn supports_queue_info(&self) -> bool {
        true
    }

    fn supports_synchronous_execution(&self) -> bool {
        true
    }
}

/// Marks the local adapter as having passed the adapter-level contract.
///
/// The local implementation is deliberately self-contained and therefore can
/// serve as the reference backend for the provider-neutral conformance suite.
impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for LocalBackendAdapter
{
}

// =============================================================================
// Backend construction
// =============================================================================

fn build_backend(
    config: LocalBackendConfig,
) -> Result<QuantumBackend, BackendError> {
    let metadata = BackendMetadata::new(
        LOCAL_BACKEND_ID,
        "Zamani Local State Vector",
        LOCAL_PROVIDER_ID,
        LOCAL_ADAPTER_VERSION,
        BackendKind::Simulator,
    )
    .with_region("local");

    let mut capabilities = BackendCapabilities::new();

    capabilities.measurement = true;
    capabilities.reset = true;
    capabilities.mid_circuit_measurement = true;
    capabilities.classical_control = true;
    capabilities.dynamic_circuits = true;
    capabilities.arbitrary_single_qubit_rotations = true;
    capabilities.parameterized_gates = true;
    capabilities.three_qubit_operations = true;
    capabilities.multi_qubit_operations = false;
    capabilities.parallel_operations = false;
    capabilities.batch_execution = false;
    capabilities.streaming_results = false;
    capabilities.cancellation = true;
    capabilities.queue_information = true;
    capabilities.deterministic_seeding = true;
    capabilities.state_vector_results = true;
    capabilities.expectation_value_results = false;
    capabilities.calibration_data = false;
    capabilities.timing_information = true;
    capabilities.topology_information = true;
    capabilities.native_instruction_set = true;

    capabilities.native_gates = [
        "i",
        "id",
        "x",
        "y",
        "z",
        "h",
        "s",
        "sdg",
        "t",
        "tdg",
        "rx",
        "ry",
        "rz",
        "u",
        "cx",
        "cnot",
        "cz",
        "swap",
        "ccx",
        "toffoli",
        "measure",
        "reset",
        "barrier",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();

    let limits = BackendLimits::unlimited()
        .with_max_qubits(config.max_qubits)
        .with_max_classical_bits(config.max_classical_bits)
        .with_max_shots(config.max_shots)
        .with_max_operations(config.max_operations)
        .with_max_concurrent_jobs(1)
        .with_max_batch_size(1);

    let topology = fully_connected_topology(config.max_qubits)?;

    QuantumBackend::new(
        metadata,
        capabilities,
        limits,
        topology,
    )
}

fn fully_connected_topology(
    qubits: usize,
) -> Result<HardwareTopology, BackendError> {
    let mut couplings = Vec::new();

    let pair_count = qubits
        .checked_mul(qubits.saturating_sub(1))
        .and_then(|value| value.checked_div(2))
        .ok_or_else(|| {
            BackendError::ExecutionRejected(
                "local topology pair-count overflow".to_string(),
            )
        })?;

    couplings.reserve(pair_count);

    for source in 0..qubits {
        for target in (source + 1)..qubits {
            couplings.push(Coupling::bidirectional(source, target));
        }
    }

    Ok(HardwareTopology::from_couplings(
        qubits,
        couplings,
    )?)
}

// =============================================================================
// Local program model
// =============================================================================

#[derive(Debug, Clone)]
struct LocalProgram {
    schema: String,
    qubits: usize,
    classical_bits: usize,
    measure_all: bool,
    operations: Vec<LocalOperation>,
}

#[derive(Debug, Clone)]
struct LocalOperation {
    instruction: LocalInstruction,
    targets: Vec<usize>,
    params: Vec<f64>,
    classical: Vec<usize>,
    condition: Option<ClassicalCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalInstruction {
    Identity,
    X,
    Y,
    Z,
    H,
    S,
    Sdg,
    T,
    Tdg,
    Rx,
    Ry,
    Rz,
    U,
    Cx,
    Cz,
    Swap,
    Ccx,
    Measure,
    Reset,
    Barrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ClassicalCondition {
    bit: usize,
    equals: u8,
}

impl LocalProgram {
    fn parse(bytes: &[u8]) -> Result<Self, BackendError> {
        if bytes.is_empty() {
            return Err(BackendError::ExecutionRejected(
                "local program is empty".to_string(),
            ));
        }

        if bytes.len() > LOCAL_MAX_PROGRAM_BYTES {
            return Err(BackendError::ExecutionRejected(
                "local program payload is too large".to_string(),
            ));
        }

        let text = std::str::from_utf8(bytes).map_err(|_| {
            BackendError::ExecutionRejected(
                "local program must be valid UTF-8 JSON".to_string(),
            )
        })?;

        let root: Value = serde_json::from_str(text).map_err(|error| {
            BackendError::ExecutionRejected(format!(
                "invalid local JSON program: {}",
                sanitize_error_text(&error.to_string())
            ))
        })?;

        validate_json_depth(&root, 0)?;

        let object = root.as_object().ok_or_else(|| {
            BackendError::ExecutionRejected(
                "local program root must be a JSON object".to_string(),
            )
        })?;

        let schema = required_string(object, "schema")?;

        if schema != LOCAL_PROGRAM_FORMAT {
            return Err(BackendError::ExecutionRejected(format!(
                "unsupported local program schema `{}`",
                sanitize_identifier_text(&schema)
            )));
        }

        let qubits = required_usize(object, "qubits")?;
        let classical_bits = optional_usize(object, "classical_bits")?
            .unwrap_or(0);

        let measure_all = optional_bool(object, "measure_all")?
            .unwrap_or(false);

        if qubits == 0 || qubits > LOCAL_MAX_QUBITS {
            return Err(BackendError::ExecutionRejected(format!(
                "local program qubit count must be in 1..={}",
                LOCAL_MAX_QUBITS
            )));
        }

        if classical_bits > LOCAL_MAX_CLASSICAL_BITS {
            return Err(BackendError::ExecutionRejected(format!(
                "local program classical-bit count exceeds {}",
                LOCAL_MAX_CLASSICAL_BITS
            )));
        }

        let operations_value = object
            .get("operations")
            .ok_or_else(|| {
                BackendError::ExecutionRejected(
                    "local program requires `operations`".to_string(),
                )
            })?;

        let operations_array =
            operations_value.as_array().ok_or_else(|| {
                BackendError::ExecutionRejected(
                    "`operations` must be a JSON array".to_string(),
                )
            })?;

        if operations_array.is_empty() {
            return Err(BackendError::ExecutionRejected(
                "local program must contain at least one operation"
                    .to_string(),
            ));
        }

        if operations_array.len() > LOCAL_MAX_OPERATIONS {
            return Err(BackendError::ExecutionRejected(format!(
                "local operation count exceeds {}",
                LOCAL_MAX_OPERATIONS
            )));
        }

        let mut operations = Vec::with_capacity(operations_array.len());

        for value in operations_array {
            operations.push(parse_operation(
                value,
                qubits,
                classical_bits,
            )?);
        }

        let has_measurement = operations.iter().any(|operation| {
            matches!(operation.instruction, LocalInstruction::Measure)
        });

        if !has_measurement && !measure_all {
            return Err(BackendError::ExecutionRejected(
                "local circuit has no measurement; add a `measure` \
                 operation or set `measure_all` to true"
                    .to_string(),
            ));
        }

        if measure_all && classical_bits < qubits {
            return Err(BackendError::ExecutionRejected(
                "`measure_all` requires classical_bits >= qubits"
                    .to_string(),
            ));
        }

        Ok(Self {
            schema,
            qubits,
            classical_bits,
            measure_all,
            operations,
        })
    }
}

// =============================================================================
// JSON parsing helpers
// =============================================================================

fn parse_operation(
    value: &Value,
    qubits: usize,
    classical_bits: usize,
) -> Result<LocalOperation, BackendError> {
    let object = value.as_object().ok_or_else(|| {
        BackendError::ExecutionRejected(
            "each local operation must be a JSON object".to_string(),
        )
    })?;

    let gate = required_string(object, "gate")?;

    let instruction = parse_instruction(&gate)?;

    let targets = parse_usize_array(
        object.get("targets"),
        "targets",
    )?;

    let params = parse_f64_array(
        object.get("params"),
        "params",
    )?;

    let classical = parse_usize_array(
        object.get("classical"),
        "classical",
    )?;

    let condition = parse_condition(
        object.get("condition"),
    )?;

    if params.len() > LOCAL_MAX_PARAMETERS {
        return Err(BackendError::ExecutionRejected(
            "too many gate parameters".to_string(),
        ));
    }

    for target in &targets {
        if *target >= qubits {
            return Err(BackendError::ExecutionRejected(format!(
                "operation target {} is outside 0..{}",
                target,
                qubits.saturating_sub(1)
            )));
        }
    }

    for bit in &classical {
        if *bit >= classical_bits {
            return Err(BackendError::ExecutionRejected(format!(
                "classical target {} is outside 0..{}",
                bit,
                classical_bits.saturating_sub(1)
            )));
        }
    }

    if let Some(condition) = condition {
        if condition.bit >= classical_bits {
            return Err(BackendError::ExecutionRejected(format!(
                "condition bit {} is outside classical register",
                condition.bit
            )));
        }
    }

    validate_operation_shape(
        instruction,
        &targets,
        &params,
        &classical,
    )?;

    Ok(LocalOperation {
        instruction,
        targets,
        params,
        classical,
        condition,
    })
}

fn parse_instruction(
    gate: &str,
) -> Result<LocalInstruction, BackendError> {
    let normalized = gate.trim().to_ascii_lowercase();

    match normalized.as_str() {
        "i" | "id" | "identity" => Ok(LocalInstruction::Identity),

        "x" => Ok(LocalInstruction::X),

        "y" => Ok(LocalInstruction::Y),

        "z" => Ok(LocalInstruction::Z),

        "h" => Ok(LocalInstruction::H),

        "s" => Ok(LocalInstruction::S),

        "sdg" | "sdag" => Ok(LocalInstruction::Sdg),

        "t" => Ok(LocalInstruction::T),

        "tdg" | "tdag" => Ok(LocalInstruction::Tdg),

        "rx" => Ok(LocalInstruction::Rx),

        "ry" => Ok(LocalInstruction::Ry),

        "rz" => Ok(LocalInstruction::Rz),

        "u" | "u3" => Ok(LocalInstruction::U),

        "cx" | "cnot" => Ok(LocalInstruction::Cx),

        "cz" => Ok(LocalInstruction::Cz),

        "swap" => Ok(LocalInstruction::Swap),

        "ccx" | "toffoli" => Ok(LocalInstruction::Ccx),

        "measure" | "measurement" => Ok(LocalInstruction::Measure),

        "reset" => Ok(LocalInstruction::Reset),

        "barrier" => Ok(LocalInstruction::Barrier),

        _ => Err(BackendError::ExecutionRejected(format!(
            "unsupported local instruction `{}`",
            sanitize_identifier_text(&normalized)
        ))),
    }
}

fn validate_operation_shape(
    instruction: LocalInstruction,
    targets: &[usize],
    params: &[f64],
    classical: &[usize],
) -> Result<(), BackendError> {
    let expected_targets = match instruction {
        LocalInstruction::Identity
        | LocalInstruction::X
        | LocalInstruction::Y
        | LocalInstruction::Z
        | LocalInstruction::H
        | LocalInstruction::S
        | LocalInstruction::Sdg
        | LocalInstruction::T
        | LocalInstruction::Tdg
        | LocalInstruction::Rx
        | LocalInstruction::Ry
        | LocalInstruction::Rz
        | LocalInstruction::U
        | LocalInstruction::Reset => 1,

        LocalInstruction::Cx
        | LocalInstruction::Cz
        | LocalInstruction::Swap => 2,

        LocalInstruction::Ccx => 3,

        LocalInstruction::Measure => {
            if targets.is_empty() {
                return Err(BackendError::ExecutionRejected(
                    "measure requires at least one target".to_string(),
                ));
            }

            if classical.len() != targets.len() {
                return Err(BackendError::ExecutionRejected(
                    "measure requires one classical target per \
                     quantum target"
                        .to_string(),
                ));
            }

            return Ok(());
        }

        LocalInstruction::Barrier => {
            if targets.is_empty() {
                return Err(BackendError::ExecutionRejected(
                    "barrier requires at least one target".to_string(),
                ));
            }

            if !params.is_empty() || !classical.is_empty() {
                return Err(BackendError::ExecutionRejected(
                    "barrier does not accept params or classical targets"
                        .to_string(),
                ));
            }

            return Ok(());
        }
    };

    if targets.len() != expected_targets {
        return Err(BackendError::ExecutionRejected(format!(
            "instruction requires {} target(s), received {}",
            expected_targets,
            targets.len()
        )));
    }

    let expected_params = match instruction {
        LocalInstruction::Rx
        | LocalInstruction::Ry
        | LocalInstruction::Rz => 1,

        LocalInstruction::U => 3,

        _ => 0,
    };

    if params.len() != expected_params {
        return Err(BackendError::ExecutionRejected(format!(
            "instruction requires {} parameter(s), received {}",
            expected_params,
            params.len()
        )));
    }

    if !classical.is_empty() {
        return Err(BackendError::ExecutionRejected(
            "classical targets are allowed only on measure operations"
                .to_string(),
        ));
    }

    Ok(())
}

fn parse_condition(
    value: Option<&Value>,
) -> Result<Option<ClassicalCondition>, BackendError> {
    let Some(value) = value else {
        return Ok(None);
    };

    let object = value.as_object().ok_or_else(|| {
        BackendError::ExecutionRejected(
            "`condition` must be a JSON object".to_string(),
        )
    })?;

    let bit = required_usize(object, "bit")?;
    let equals = required_usize(object, "equals")?;

    if equals > 1 {
        return Err(BackendError::ExecutionRejected(
            "condition `equals` must be 0 or 1".to_string(),
        ));
    }

    Ok(Some(ClassicalCondition {
        bit,
        equals: equals as u8,
    }))
}

fn parse_usize_array(
    value: Option<&Value>,
    field: &str,
) -> Result<Vec<usize>, BackendError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };

    let array = value.as_array().ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "`{}` must be a JSON array",
            field
        ))
    })?;

    if array.len() > LOCAL_MAX_QUBITS.max(LOCAL_MAX_CLASSICAL_BITS) {
        return Err(BackendError::ExecutionRejected(format!(
            "`{}` contains too many elements",
            field
        )));
    }

    let mut result = Vec::with_capacity(array.len());

    for value in array {
        let number = value.as_u64().ok_or_else(|| {
            BackendError::ExecutionRejected(format!(
                "`{}` must contain only unsigned integers",
                field
            ))
        })?;

        let number = usize::try_from(number).map_err(|_| {
            BackendError::ExecutionRejected(format!(
                "`{}` integer does not fit platform usize",
                field
            ))
        })?;

        result.push(number);
    }

    Ok(result)
}

fn parse_f64_array(
    value: Option<&Value>,
    field: &str,
) -> Result<Vec<f64>, BackendError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };

    let array = value.as_array().ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "`{}` must be a JSON array",
            field
        ))
    })?;

    let mut result = Vec::with_capacity(array.len());

    for value in array {
        let number = value.as_f64().ok_or_else(|| {
            BackendError::ExecutionRejected(format!(
                "`{}` must contain only JSON numbers",
                field
            ))
        })?;

        if !number.is_finite() {
            return Err(BackendError::ExecutionRejected(format!(
                "`{}` contains a non-finite value",
                field
            )));
        }

        result.push(number);
    }

    Ok(result)
}

fn required_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<String, BackendError> {
    let value = object.get(field).ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "local program requires `{}`",
            field
        ))
    })?;

    let string = value.as_str().ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "`{}` must be a string",
            field
        ))
    })?;

    if string.trim().is_empty() {
        return Err(BackendError::ExecutionRejected(format!(
            "`{}` cannot be empty",
            field
        )));
    }

    Ok(string.to_string())
}

fn required_usize(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<usize, BackendError> {
    let value = object.get(field).ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "local program requires `{}`",
            field
        ))
    })?;

    let number = value.as_u64().ok_or_else(|| {
        BackendError::ExecutionRejected(format!(
            "`{}` must be an unsigned integer",
            field
        ))
    })?;

    usize::try_from(number).map_err(|_| {
        BackendError::ExecutionRejected(format!(
            "`{}` does not fit platform usize",
            field
        ))
    })
}

fn optional_usize(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<usize>, BackendError> {
    match object.get(field) {
        Some(value) => {
            let number = value.as_u64().ok_or_else(|| {
                BackendError::ExecutionRejected(format!(
                    "`{}` must be an unsigned integer",
                    field
                ))
            })?;

            Ok(Some(usize::try_from(number).map_err(|_| {
                BackendError::ExecutionRejected(format!(
                    "`{}` does not fit platform usize",
                    field
                ))
            })?))
        }

        None => Ok(None),
    }
}

fn optional_bool(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<bool>, BackendError> {
    match object.get(field) {
        Some(value) => Ok(Some(value.as_bool().ok_or_else(|| {
            BackendError::ExecutionRejected(format!(
                "`{}` must be boolean",
                field
            ))
        })?)),

        None => Ok(None),
    }
}

fn validate_json_depth(
    value: &Value,
    depth: usize,
) -> Result<(), BackendError> {
    if depth > LOCAL_MAX_JSON_DEPTH {
        return Err(BackendError::ExecutionRejected(
            "local program JSON nesting is too deep".to_string(),
        ));
    }

    match value {
        Value::Array(values) => {
            for value in values {
                validate_json_depth(value, depth + 1)?;
            }
        }

        Value::Object(values) => {
            for value in values.values() {
                validate_json_depth(value, depth + 1)?;
            }
        }

        _ => {}
    }

    Ok(())
}

// =============================================================================
// Request/program compatibility
// =============================================================================

fn validate_request_against_program(
    request: &ExecutionRequest,
    program: &LocalProgram,
    config: LocalBackendConfig,
) -> Result<(), BackendError> {
    let circuit = &request.workload.circuit;

    if program.qubits > config.max_qubits {
        return Err(BackendError::QubitLimitExceeded {
            requested: program.qubits,
            maximum: config.max_qubits,
        });
    }

    if circuit.qubit_count != 0
        && circuit.qubit_count != program.qubits
    {
        return Err(BackendError::ExecutionRejected(format!(
            "request requires {} qubits but local program declares {}",
            circuit.qubit_count,
            program.qubits
        )));
    }

    if program.classical_bits > config.max_classical_bits {
        return Err(BackendError::ClassicalBitLimitExceeded {
            requested: program.classical_bits,
            maximum: config.max_classical_bits,
        });
    }

    if circuit.classical_bit_count != 0
        && circuit.classical_bit_count != program.classical_bits
    {
        return Err(BackendError::ExecutionRejected(format!(
            "request requires {} classical bits but local program \
             declares {}",
            circuit.classical_bit_count,
            program.classical_bits
        )));
    }

    if program.operations.len() > config.max_operations {
        return Err(BackendError::OperationLimitExceeded {
            requested: program.operations.len(),
            maximum: config.max_operations,
        });
    }

    if circuit.operation_count != 0
        && circuit.operation_count != program.operations.len()
    {
        return Err(BackendError::ExecutionRejected(format!(
            "request requires {} operations but local program contains {}",
            circuit.operation_count,
            program.operations.len()
        )));
    }

    if circuit.shots == 0 {
        return Err(BackendError::InvalidShots);
    }

    if circuit.shots > config.max_shots {
        return Err(BackendError::ShotLimitExceeded {
            requested: circuit.shots,
            maximum: config.max_shots,
        });
    }

    match request.workload.kind {
        QuantumWorkloadKind::GateCircuit
        | QuantumWorkloadKind::DynamicCircuit
        | QuantumWorkloadKind::Sampling => {}

        kind => {
            return Err(BackendError::UnsupportedWorkload {
                workload: kind.as_str(),
            });
        }
    }

    if request.workload.requires_topology
        && !config.max_qubits.eq(&program.qubits)
    {
        return Err(BackendError::TopologyUnavailable);
    }

    if request.workload.requires_calibration {
        return Err(BackendError::CalibrationUnavailable);
    }

    if request.workload.requires_fresh_calibration {
        return Err(BackendError::FreshCalibrationRequired);
    }

    if request.seed.is_some()
        && !request
            .workload
            .required_capabilities
            .is_empty()
    {
        /*
         * Capability validation remains authoritative in backend.rs.
         * This branch deliberately does not duplicate capability semantics.
         */
    }

    Ok(())
}

// =============================================================================
// State-vector simulator
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    const ONE: Self = Self {
        real: 1.0,
        imaginary: 0.0,
    };

    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    fn magnitude_squared(self) -> f64 {
        self.real.mul_add(
            self.real,
            self.imaginary * self.imaginary,
        )
    }

    fn scale(self, factor: f64) -> Self {
        Self {
            real: self.real * factor,
            imaginary: self.imaginary * factor,
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real
                - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary
                + self.imaginary * other.real,
        }
    }
}

struct StateVector {
    qubits: usize,
    amplitudes: Vec<Complex>,
}

impl StateVector {
    fn new(qubits: usize) -> Result<Self, BackendError> {
        if qubits == 0 || qubits > LOCAL_MAX_QUBITS {
            return Err(BackendError::ExecutionRejected(
                format!(
                    "state-vector qubit count must be in 1..={}",
                    LOCAL_MAX_QUBITS
                ),
            ));
        }

        let dimension = 1usize
            .checked_shl(qubits as u32)
            .ok_or_else(|| {
                BackendError::ExecutionRejected(
                    "state-vector dimension overflow".to_string(),
                )
            })?;

        let mut amplitudes = vec![Complex::ZERO; dimension];

        amplitudes[0] = Complex::ONE;

        Ok(Self {
            qubits,
            amplitudes,
        })
    }

    fn apply_single(
        &mut self,
        target: usize,
        a: Complex,
        b: Complex,
        c: Complex,
        d: Complex,
    ) -> Result<(), BackendError> {
        self.validate_qubit(target)?;

        let mask = 1usize << target;

        let dimension = self.amplitudes.len();

        let mut base = 0usize;

        while base < dimension {
            let mut offset = 0usize;

            while offset < mask {
                let i0 = base + offset;
                let i1 = i0 | mask;

                let v0 = self.amplitudes[i0];
                let v1 = self.amplitudes[i1];

                self.amplitudes[i0] =
                    a.mul(v0).add(b.mul(v1));

                self.amplitudes[i1] =
                    c.mul(v0).add(d.mul(v1));

                offset += 1;
            }

            base += mask << 1;
        }

        self.renormalize()?;

        Ok(())
    }

    fn apply_controlled(
        &mut self,
        control: usize,
        target: usize,
        matrix: [[Complex; 2]; 2],
    ) -> Result<(), BackendError> {
        self.validate_qubit(control)?;
        self.validate_qubit(target)?;

        if control == target {
            return Err(BackendError::ExecutionRejected(
                "controlled operation cannot use the same control \
                 and target"
                    .to_string(),
            ));
        }

        let control_mask = 1usize << control;
        let target_mask = 1usize << target;

        let dimension = self.amplitudes.len();

        for index in 0..dimension {
            if index & control_mask == 0
                || index & target_mask != 0
            {
                continue;
            }

            let i0 = index;
            let i1 = index | target_mask;

            let v0 = self.amplitudes[i0];
            let v1 = self.amplitudes[i1];

            self.amplitudes[i0] =
                matrix[0][0].mul(v0)
                    .add(matrix[0][1].mul(v1));

            self.amplitudes[i1] =
                matrix[1][0].mul(v0)
                    .add(matrix[1][1].mul(v1));
        }

        self.renormalize()?;

        Ok(())
    }

    fn apply_swap(
        &mut self,
        first: usize,
        second: usize,
    ) -> Result<(), BackendError> {
        self.validate_qubit(first)?;
        self.validate_qubit(second)?;

        if first == second {
            return Err(BackendError::ExecutionRejected(
                "SWAP requires two distinct qubits".to_string(),
            ));
        }

        let first_mask = 1usize << first;
        let second_mask = 1usize << second;

        for index in 0..self.amplitudes.len() {
            let first_bit = index & first_mask != 0;
            let second_bit = index & second_mask != 0;

            if first_bit == second_bit {
                continue;
            }

            let swapped =
                index ^ first_mask ^ second_mask;

            if index < swapped {
                self.amplitudes.swap(index, swapped);
            }
        }

        self.renormalize()?;

        Ok(())
    }

    fn apply_ccx(
        &mut self,
        control0: usize,
        control1: usize,
        target: usize,
    ) -> Result<(), BackendError> {
        self.validate_qubit(control0)?;
        self.validate_qubit(control1)?;
        self.validate_qubit(target)?;

        if control0 == control1
            || control0 == target
            || control1 == target
        {
            return Err(BackendError::ExecutionRejected(
                "CCX requires three distinct qubits".to_string(),
            ));
        }

        let mask0 = 1usize << control0;
        let mask1 = 1usize << control1;
        let target_mask = 1usize << target;

        for index in 0..self.amplitudes.len() {
            if index & mask0 == 0
                || index & mask1 == 0
                || index & target_mask != 0
            {
                continue;
            }

            let other = index | target_mask;

            if index < other {
                self.amplitudes.swap(index, other);
            }
        }

        self.renormalize()?;

        Ok(())
    }

    fn measure(
        &mut self,
        qubit: usize,
        rng: &mut DeterministicRng,
    ) -> Result<u8, BackendError> {
        self.validate_qubit(qubit)?;

        let mask = 1usize << qubit;

        let probability_one = self
            .amplitudes
            .iter()
            .enumerate()
            .filter_map(|(index, amplitude)| {
                if index & mask != 0 {
                    Some(amplitude.magnitude_squared())
                } else {
                    None
                }
            })
            .sum::<f64>();

        if !probability_one.is_finite()
            || probability_one < -1.0e-12
            || probability_one > 1.0 + 1.0e-12
        {
            return Err(BackendError::ExecutionRejected(
                "state-vector probability became invalid".to_string(),
            ));
        }

        let probability_one =
            probability_one.clamp(0.0, 1.0);

        let random = rng.next_f64();

        let outcome = if random < probability_one {
            1
        } else {
            0
        };

        let probability =
            if outcome == 1 {
                probability_one
            } else {
                1.0 - probability_one
            };

        if probability <= 1.0e-15 {
            return Err(BackendError::ExecutionRejected(
                "measurement selected a zero-probability branch"
                    .to_string(),
            ));
        }

        let inverse_norm = 1.0 / probability.sqrt();

        for (index, amplitude) in
            self.amplitudes.iter_mut().enumerate()
        {
            let bit_is_one = index & mask != 0;

            if (outcome == 1) != bit_is_one {
                *amplitude = Complex::ZERO;
            } else {
                *amplitude =
                    amplitude.scale(inverse_norm);
            }
        }

        self.renormalize()?;

        Ok(outcome)
    }

    fn reset(
        &mut self,
        qubit: usize,
        rng: &mut DeterministicRng,
    ) -> Result<(), BackendError> {
        let outcome = self.measure(qubit, rng)?;

        if outcome == 1 {
            self.apply_single(
                qubit,
                Complex::ZERO,
                Complex::ONE,
                Complex::ONE,
                Complex::ZERO,
            )?;
        }

        Ok(())
    }

    fn sample_computational_basis(
        &self,
        rng: &mut DeterministicRng,
    ) -> Result<usize, BackendError> {
        let mut random = rng.next_f64();

        let mut accumulated = 0.0;

        for (index, amplitude) in
            self.amplitudes.iter().enumerate()
        {
            let probability = amplitude.magnitude_squared();

            if !probability.is_finite() || probability < 0.0 {
                return Err(BackendError::ExecutionRejected(
                    "state-vector contains invalid probability"
                        .to_string(),
                ));
            }

            accumulated += probability;

            if random <= accumulated {
                return Ok(index);
            }
        }

        /*
         * Floating-point accumulation can end a few ulps below one.
         * Returning the final basis state is safe because the vector was
         * normalized immediately before sampling.
         */
        random = 0.0;

        let _ = random;

        Ok(self.amplitudes.len().saturating_sub(1))
    }

    fn renormalize(&mut self) -> Result<(), BackendError> {
        let norm = self
            .amplitudes
            .iter()
            .map(|amplitude| amplitude.magnitude_squared())
            .sum::<f64>();

        if !norm.is_finite() || norm <= 1.0e-15 {
            return Err(BackendError::ExecutionRejected(
                "state-vector norm became invalid".to_string(),
            ));
        }

        let inverse_norm = 1.0 / norm.sqrt();

        for amplitude in &mut self.amplitudes {
            *amplitude = amplitude.scale(inverse_norm);
        }

        Ok(())
    }

    fn validate_qubit(
        &self,
        qubit: usize,
    ) -> Result<(), BackendError> {
        if qubit >= self.qubits {
            return Err(BackendError::ExecutionRejected(format!(
                "qubit {} is outside state-vector range 0..{}",
                qubit,
                self.qubits.saturating_sub(1)
            )));
        }

        Ok(())
    }
}

// =============================================================================
// Gate execution
// =============================================================================

fn execute_program(
    program: &LocalProgram,
    request: &ExecutionRequest,
    config: LocalBackendConfig,
) -> Result<ExecutionResult, BackendError> {
    let shots = request.workload.circuit.shots;

    if shots == 0 {
        return Err(BackendError::InvalidShots);
    }

    if shots > config.max_shots {
        return Err(BackendError::ShotLimitExceeded {
            requested: shots,
            maximum: config.max_shots,
        });
    }

    let seed = request
        .seed
        .unwrap_or(LOCAL_DEFAULT_SEED);

    let mut counts = BTreeMap::<String, usize>::new();

    for shot in 0..shots {
        let shot_seed =
            derive_shot_seed(seed, shot as u64);

        let mut rng =
            DeterministicRng::new(shot_seed);

        let mut state =
            StateVector::new(program.qubits)?;

        let mut classical =
            vec![0u8; program.classical_bits];

        for operation in &program.operations {
            if !condition_matches(
                operation.condition,
                &classical,
            ) {
                continue;
            }

            execute_operation(
                &mut state,
                operation,
                &mut classical,
                &mut rng,
            )?;
        }

        if program.measure_all {
            for qubit in 0..program.qubits {
                let classical_bit = qubit;

                if classical_bit >= classical.len() {
                    return Err(
                        BackendError::ClassicalBitLimitExceeded {
                            requested: program.qubits,
                            maximum: config.max_classical_bits,
                        },
                    );
                }

                classical[classical_bit] =
                    state.measure(qubit, &mut rng)?;
            }
        }

        let bitstring =
            classical_bitstring(&classical);

        let entry =
            counts.entry(bitstring).or_insert(0);

        *entry = entry.checked_add(1).ok_or(
            BackendError::ResultCountOverflow,
        )?;
    }

    let mut result =
        ExecutionResult::empty(
            LOCAL_BACKEND_ID,
            shots,
        )?;

    for (bitstring, count) in counts {
        result.insert_count(bitstring, count)?;
    }

    result.validate()?;

    Ok(result)
}

fn execute_operation(
    state: &mut StateVector,
    operation: &LocalOperation,
    classical: &mut [u8],
    rng: &mut DeterministicRng,
) -> Result<(), BackendError> {
    match operation.instruction {
        LocalInstruction::Identity => {}

        LocalInstruction::X => {
            state.apply_single(
                operation.targets[0],
                Complex::ZERO,
                Complex::ONE,
                Complex::ONE,
                Complex::ZERO,
            )?;
        }

        LocalInstruction::Y => {
            state.apply_single(
                operation.targets[0],
                Complex::ZERO,
                Complex::new(0.0, -1.0),
                Complex::new(0.0, 1.0),
                Complex::ZERO,
            )?;
        }

        LocalInstruction::Z => {
            state.apply_single(
                operation.targets[0],
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(-1.0, 0.0),
            )?;
        }

        LocalInstruction::H => {
            let factor = 1.0 / 2.0_f64.sqrt();

            state.apply_single(
                operation.targets[0],
                Complex::new(factor, 0.0),
                Complex::new(factor, 0.0),
                Complex::new(factor, 0.0),
                Complex::new(-factor, 0.0),
            )?;
        }

        LocalInstruction::S => {
            state.apply_single(
                operation.targets[0],
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(0.0, 1.0),
            )?;
        }

        LocalInstruction::Sdg => {
            state.apply_single(
                operation.targets[0],
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(0.0, -1.0),
            )?;
        }

        LocalInstruction::T => {
            let angle =
                std::f64::consts::FRAC_PI_4;

            state.apply_single(
                operation.targets[0],
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(
                    angle.cos(),
                    angle.sin(),
                ),
            )?;
        }

        LocalInstruction::Tdg => {
            let angle =
                -std::f64::consts::FRAC_PI_4;

            state.apply_single(
                operation.targets[0],
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(
                    angle.cos(),
                    angle.sin(),
                ),
            )?;
        }

        LocalInstruction::Rx => {
            let theta = operation.params[0];

            let half = theta / 2.0;

            let cosine = half.cos();

            let sine = half.sin();

            state.apply_single(
                operation.targets[0],
                Complex::new(cosine, 0.0),
                Complex::new(0.0, -sine),
                Complex::new(0.0, -sine),
                Complex::new(cosine, 0.0),
            )?;
        }

        LocalInstruction::Ry => {
            let theta = operation.params[0];

            let half = theta / 2.0;

            let cosine = half.cos();

            let sine = half.sin();

            state.apply_single(
                operation.targets[0],
                Complex::new(cosine, 0.0),
                Complex::new(-sine, 0.0),
                Complex::new(sine, 0.0),
                Complex::new(cosine, 0.0),
            )?;
        }

        LocalInstruction::Rz => {
            let theta = operation.params[0];

            let half = theta / 2.0;

            state.apply_single(
                operation.targets[0],
                Complex::new(
                    (-half).cos(),
                    (-half).sin(),
                ),
                Complex::ZERO,
                Complex::ZERO,
                Complex::new(
                    half.cos(),
                    half.sin(),
                ),
            )?;
        }

        LocalInstruction::U => {
            let theta = operation.params[0];
            let phi = operation.params[1];
            let lambda = operation.params[2];

            apply_u(
                state,
                operation.targets[0],
                theta,
                phi,
                lambda,
            )?;
        }

        LocalInstruction::Cx => {
            state.apply_controlled(
                operation.targets[0],
                operation.targets[1],
                [
                    [Complex::ZERO, Complex::ONE],
                    [Complex::ONE, Complex::ZERO],
                ],
            )?;
        }

        LocalInstruction::Cz => {
            state.apply_controlled(
                operation.targets[0],
                operation.targets[1],
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(-1.0, 0.0)],
                ],
            )?;
        }

        LocalInstruction::Swap => {
            state.apply_swap(
                operation.targets[0],
                operation.targets[1],
            )?;
        }

        LocalInstruction::Ccx => {
            state.apply_ccx(
                operation.targets[0],
                operation.targets[1],
                operation.targets[2],
            )?;
        }

        LocalInstruction::Measure => {
            for (index, qubit) in
                operation.targets.iter().enumerate()
            {
                let result =
                    state.measure(*qubit, rng)?;

                let classical_bit =
                    operation.classical[index];

                classical[classical_bit] = result;
            }
        }

        LocalInstruction::Reset => {
            state.reset(
                operation.targets[0],
                rng,
            )?;
        }

        LocalInstruction::Barrier => {
            /*
             * Barrier has no mathematical effect in this local state-vector
             * engine. Timing/scheduling semantics belong to the hardware
             * timing/scheduling layers.
             */
        }
    }

    Ok(())
}

fn apply_u(
    state: &mut StateVector,
    target: usize,
    theta: f64,
    phi: f64,
    lambda: f64,
) -> Result<(), BackendError> {
    let theta_half = theta / 2.0;

    let cosine = theta_half.cos();

    let sine = theta_half.sin();

    let phase_lambda =
        Complex::new(lambda.cos(), lambda.sin());

    let phase_phi =
        Complex::new(phi.cos(), phi.sin());

    let phase_sum = Complex::new(
        (phi + lambda).cos(),
        (phi + lambda).sin(),
    );

    let a = Complex::new(cosine, 0.0);

    let b = phase_lambda.scale(-sine);

    let c = phase_phi.scale(sine);

    let d = phase_sum.scale(cosine);

    state.apply_single(
        target,
        a,
        b,
        c,
        d,
    )
}

// =============================================================================
// Result normalization
// =============================================================================

fn finalize_result(
    mut result: ExecutionResult,
    job: &BackendJob,
    request: &ExecutionRequest,
    program: &LocalProgram,
    config: LocalBackendConfig,
) -> Result<ExecutionResult, BackendError> {
    insert_result_metadata(
        &mut result,
        "adapter",
        LOCAL_ADAPTER_ID,
    )?;

    insert_result_metadata(
        &mut result,
        "adapter_version",
        LOCAL_ADAPTER_VERSION,
    )?;

    insert_result_metadata(
        &mut result,
        "backend_kind",
        BackendKind::Simulator.as_str(),
    )?;

    insert_result_metadata(
        &mut result,
        "program_format",
        LOCAL_PROGRAM_FORMAT,
    )?;

    insert_result_metadata(
        &mut result,
        "program_schema",
        &program.schema,
    )?;

    insert_result_metadata(
        &mut result,
        "job_id",
        job.id.as_str(),
    )?;

    insert_result_metadata(
        &mut result,
        "request_id",
        request.request_id
            .as_deref()
            .unwrap_or("unspecified"),
    )?;

    insert_result_metadata(
        &mut result,
        "qubits",
        &program.qubits.to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "classical_bits",
        &program.classical_bits.to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "operations",
        &program.operations.len().to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "seed",
        &request
            .seed
            .unwrap_or(LOCAL_DEFAULT_SEED)
            .to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "measure_all",
        if program.measure_all {
            "true"
        } else {
            "false"
        },
    )?;

    insert_result_metadata(
        &mut result,
        "fault",
        config.fault.as_str(),
    )?;

    result.validate()?;

    if !result.counts_match_shots() {
        return Err(BackendError::ExecutionRejected(
            "local execution produced incomplete shot accounting"
                .to_string(),
        ));
    }

    Ok(result)
}

fn finalize_result_without_job(
    mut result: ExecutionResult,
    request: &ExecutionRequest,
    program: &LocalProgram,
    config: LocalBackendConfig,
) -> Result<ExecutionResult, BackendError> {
    insert_result_metadata(
        &mut result,
        "adapter",
        LOCAL_ADAPTER_ID,
    )?;

    insert_result_metadata(
        &mut result,
        "adapter_version",
        LOCAL_ADAPTER_VERSION,
    )?;

    insert_result_metadata(
        &mut result,
        "backend_kind",
        BackendKind::Simulator.as_str(),
    )?;

    insert_result_metadata(
        &mut result,
        "program_format",
        LOCAL_PROGRAM_FORMAT,
    )?;

    insert_result_metadata(
        &mut result,
        "program_schema",
        &program.schema,
    )?;

    insert_result_metadata(
        &mut result,
        "request_id",
        request.request_id
            .as_deref()
            .unwrap_or("unspecified"),
    )?;

    insert_result_metadata(
        &mut result,
        "qubits",
        &program.qubits.to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "classical_bits",
        &program.classical_bits.to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "operations",
        &program.operations.len().to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "seed",
        &request
            .seed
            .unwrap_or(LOCAL_DEFAULT_SEED)
            .to_string(),
    )?;

    insert_result_metadata(
        &mut result,
        "measure_all",
        if program.measure_all {
            "true"
        } else {
            "false"
        },
    )?;

    insert_result_metadata(
        &mut result,
        "fault",
        config.fault.as_str(),
    )?;

    result.validate()?;

    if !result.counts_match_shots() {
        return Err(BackendError::ExecutionRejected(
            "local execution produced incomplete shot accounting"
                .to_string(),
        ));
    }

    Ok(result)
}

fn insert_result_metadata(
    result: &mut ExecutionResult,
    key: &str,
    value: &str,
) -> Result<(), BackendError> {
    if result.metadata.len() >= LOCAL_MAX_RESULT_METADATA
        && !result.metadata.contains_key(key)
    {
        return Err(BackendError::MetadataLimitExceeded {
            maximum: LOCAL_MAX_RESULT_METADATA,
        });
    }

    /*
     * Reuse the canonical result validator by inserting through the public
     * result map only after validating the key/value locally.
     */
    if key.trim().is_empty()
        || key.len() > 256
        || key.chars().any(char::is_control)
    {
        return Err(BackendError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    if value.trim().is_empty()
        || value.len() > 4096
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    if looks_secret_like(key) {
        return Err(BackendError::SecretLikeMetadata {
            key: key.to_string(),
        });
    }

    result
        .metadata
        .insert(key.to_string(), value.to_string());

    Ok(())
}

fn condition_matches(
    condition: Option<ClassicalCondition>,
    classical: &[u8],
) -> bool {
    let Some(condition) = condition else {
        return true;
    };

    classical
        .get(condition.bit)
        .copied()
        .map(|value| value == condition.equals)
        .unwrap_or(false)
}

fn classical_bitstring(classical: &[u8]) -> String {
    /*
     * Canonical result ordering is classical-register order:
     *
     * c[n-1] ... c[1] c[0]
     *
     * This matches the conventional textual representation where the highest
     * indexed bit is the left-most bit.
     */
    let mut result =
        String::with_capacity(classical.len());

    for value in classical.iter().rev() {
        result.push(if *value == 0 { '0' } else { '1' });
    }

    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}

// =============================================================================
// Deterministic random source
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };

        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        /*
         * SplitMix64.
         *
         * This is intentionally used only as a deterministic simulation
         * sampler. It is NOT a cryptographic random-number generator and must
         * never be used for security, key generation or cryptographic
         * protocols.
         */
        self.state = self.state.wrapping_add(
            0x9E37_79B9_7F4A_7C15,
        );

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(
            0xBF58_476D_1CE4_E5B9,
        );

        z = (z ^ (z >> 27)).wrapping_mul(
            0x94D0_49BB_1331_11EB,
        );

        z ^ (z >> 31)
    }

    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64();

        /*
         * Use the upper 53 bits to construct a uniformly distributed value in
         * [0, 1).
         */
        let mantissa = value >> 11;

        (mantissa as f64)
            * (1.0 / ((1u64 << 53) as f64))
    }
}

fn derive_shot_seed(
    seed: u64,
    shot: u64,
) -> u64 {
    let mut value =
        seed ^ shot.wrapping_mul(0xD134_2543_DE82_EF95);

    value ^= value >> 30;

    value = value.wrapping_mul(
        0xBF58_476D_1CE4_E5B9,
    );

    value ^= value >> 27;

    value = value.wrapping_mul(
        0x94D0_49BB_1331_11EB,
    );

    value ^ (value >> 31)
}

// =============================================================================
// Security/sanitization helpers
// =============================================================================

fn looks_secret_like(value: &str) -> bool {
    let normalized =
        value.to_ascii_lowercase();

    [
        "api_key",
        "apikey",
        "access_token",
        "authorization",
        "password",
        "private_key",
        "secret",
        "cookie",
        "refresh_token",
        "bearer",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
}

fn sanitize_identifier_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| {
            !character.is_control()
        })
        .take(512)
        .collect()
}

fn sanitize_job_component(value: &str) -> String {
    value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '-' | '_' | '.' | ':'
                )
        })
        .take(480)
        .collect()
}

fn sanitize_error_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| {
            !character.is_control()
        })
        .take(1024)
        .collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        qubits: usize,
        classical_bits: usize,
        operations: usize,
        shots: usize,
    ) -> ExecutionRequest {
        let circuit =
            super::super::super::backend::CircuitRequirements {
                qubit_count: qubits,
                logical_qubit_count: 0,
                circuit_depth: operations,
                operation_count: operations,
                classical_bit_count: classical_bits,
                shots,
                instructions: Default::default(),
                required_connections: Vec::new(),
                requires_measurement: true,
                requires_reset: false,
                requires_mid_circuit_measurement: false,
                requires_classical_control: false,
                requires_dynamic_circuit: false,
                requires_pulse_control: false,
                requires_analog_control: false,
                requires_annealing: false,
                requires_logical_qubits: false,
                requires_fault_tolerance: false,
                requires_deterministic_seed: false,
                requires_state_vector: false,
                requires_density_matrix: false,
                requires_expectation_values: false,
            };

        ExecutionRequest::new(circuit)
    }

    fn program(
        qubits: usize,
        classical_bits: usize,
        operations: &str,
    ) -> BackendProgram {
        let payload = format!(
            r#"{{
                "schema":"{}",
                "qubits":{},
                "classical_bits":{},
                "measure_all":false,
                "operations":[{}]
            }}"#,
            LOCAL_PROGRAM_FORMAT,
            qubits,
            classical_bits,
            operations
        );

        BackendProgram::new(
            LOCAL_PROGRAM_FORMAT,
            payload.into_bytes(),
        )
        .expect("valid local program")
    }

    #[test]
    fn production_adapter_constructs() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        assert_eq!(
            adapter.backend().id(),
            LOCAL_BACKEND_ID
        );

        assert_eq!(
            adapter.backend().kind(),
            BackendKind::Simulator
        );

        assert_eq!(
            adapter.adapter_info().adapter_id,
            LOCAL_ADAPTER_ID
        );
    }

    #[test]
    fn health_is_healthy_by_default() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let health =
            adapter.health().expect("health");

        assert_eq!(
            health.state,
            BackendHealthState::Healthy
        );

        assert_eq!(
            health.backend_status,
            BackendStatus::Available
        );
    }

    #[test]
    fn bell_state_is_deterministically_sampled() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let mut request =
            request(2, 2, 4, 128);

        request = request.with_seed(42);

        let program = program(
            2,
            2,
            r#"
                {"gate":"h","targets":[0]},
                {"gate":"cx","targets":[0,1]},
                {"gate":"measure","targets":[0],"classical":[0]},
                {"gate":"measure","targets":[1],"classical":[1]}
            "#,
        );

        adapter
            .preflight(&request, &program)
            .expect("preflight");

        let first =
            adapter.execute(&request, &program)
                .expect("execution");

        let second =
            adapter.execute(&request, &program)
                .expect("execution");

        assert_eq!(
            first.counts,
            second.counts
        );

        assert_eq!(
            first.counted_shots(),
            128
        );

        assert!(
            first
                .counts
                .keys()
                .all(|key| key == "00" || key == "11")
        );
    }

    #[test]
    fn x_gate_produces_one() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 2, 32);

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let result =
            adapter.execute(&request, &program)
                .expect("execution");

        assert_eq!(
            result.counts.get("1"),
            Some(&32)
        );
    }

    #[test]
    fn h_gate_produces_both_outcomes() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let mut request =
            request(1, 1, 2, 256);

        request = request.with_seed(123);

        let program = program(
            1,
            1,
            r#"
                {"gate":"h","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let result =
            adapter.execute(&request, &program)
                .expect("execution");

        assert!(
            result.counts.contains_key("0")
        );

        assert!(
            result.counts.contains_key("1")
        );

        assert_eq!(
            result.counted_shots(),
            256
        );
    }

    #[test]
    fn reset_returns_qubit_to_zero() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 3, 64);

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"reset","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let result =
            adapter.execute(&request, &program)
                .expect("execution");

        assert_eq!(
            result.counts.get("0"),
            Some(&64)
        );
    }

    #[test]
    fn classical_condition_is_honoured() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(2, 2, 4, 64);

        let program = program(
            2,
            2,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]},
                {
                    "gate":"x",
                    "targets":[1],
                    "condition":{"bit":0,"equals":1}
                },
                {"gate":"measure","targets":[1],"classical":[1]}
            "#,
        );

        let result =
            adapter.execute(&request, &program)
                .expect("execution");

        assert_eq!(
            result.counts.get("11"),
            Some(&64)
        );
    }

    #[test]
    fn missing_measurement_is_rejected() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 1, 1);

        let program = BackendProgram::new(
            LOCAL_PROGRAM_FORMAT,
            br#"{
                "schema":"zamani-local-v1",
                "qubits":1,
                "classical_bits":1,
                "measure_all":false,
                "operations":[
                    {"gate":"h","targets":[0]}
                ]
            }"#
            .to_vec(),
        )
        .expect("program construction");

        assert!(
            adapter
                .preflight(&request, &program)
                .is_err()
        );
    }

    #[test]
    fn measure_all_is_supported() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(2, 2, 2, 32);

        let program = BackendProgram::new(
            LOCAL_PROGRAM_FORMAT,
            br#"{
                "schema":"zamani-local-v1",
                "qubits":2,
                "classical_bits":2,
                "measure_all":true,
                "operations":[
                    {"gate":"x","targets":[1]}
                ]
            }"#
            .to_vec(),
        )
        .expect("program construction");

        let result =
            adapter.execute(&request, &program)
                .expect("execution");

        assert_eq!(
            result.counts.get("10"),
            Some(&32)
        );
    }

    #[test]
    fn unsupported_format_is_rejected() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 1, 1);

        let program =
            BackendProgram::new(
                "openqasm-3.1",
                b"OPENQASM 3.1;".to_vec(),
            )
            .expect("program construction");

        assert!(
            adapter
                .preflight(&request, &program)
                .is_err()
        );
    }

    #[test]
    fn unsupported_workload_is_rejected() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let mut request =
            request(1, 1, 2, 1);

        request.workload.kind =
            QuantumWorkloadKind::AnalogProgram;

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        assert!(
            adapter
                .preflight(&request, &program)
                .is_err()
        );
    }

    #[test]
    fn submit_creates_completed_job() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 2, 8);

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let job =
            adapter
                .submit(&request, &program)
                .expect("submission");

        assert_eq!(
            job.state,
            BackendJobState::Completed
        );

        let status =
            adapter
                .status(&job.id)
                .expect("status");

        assert!(
            status.result_available
        );

        let result =
            adapter
                .result(&job.id)
                .expect("result");

        assert_eq!(
            result.counts.get("1"),
            Some(&8)
        );
    }

    #[test]
    fn request_id_produces_stable_job_identity() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 2, 1)
                .with_request_id("test-job")
                .expect("request id");

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let job =
            adapter
                .submit(&request, &program)
                .expect("submission");

        assert_eq!(
            job.id.as_str(),
            "local-test-job"
        );
    }

    #[test]
    fn fault_injection_can_reject_submission() {
        let adapter =
            LocalBackendAdapter::with_config(
                LocalBackendConfig::test()
                    .with_fault(
                        LocalFault::RejectSubmission
                    ),
            )
            .expect("adapter construction");

        let request =
            request(1, 1, 2, 1);

        let program = program(
            1,
            1,
            r#"
                {"gate":"x","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        assert!(
            adapter
                .submit(&request, &program)
                .is_err()
        );
    }

    #[test]
    fn result_accounting_is_exact() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let request =
            request(1, 1, 2, 100);

        let program = program(
            1,
            1,
            r#"
                {"gate":"h","targets":[0]},
                {"gate":"measure","targets":[0],"classical":[0]}
            "#,
        );

        let result =
            adapter
                .execute(&request, &program)
                .expect("execution");

        assert!(
            result.counts_match_shots()
        );

        assert_eq!(
            result.counted_shots(),
            100
        );
    }

    #[test]
    fn queue_is_immediately_available() {
        let adapter =
            LocalBackendAdapter::new()
                .expect("adapter construction");

        let queue =
            adapter.queue_info()
                .expect("queue information");

        assert_eq!(
            queue.pending_jobs,
            Some(0)
        );

        assert_eq!(
            queue.estimated_wait,
            Some(std::time::Duration::ZERO)
        );

        assert!(
            queue.accepting_submissions
        );
    }
}