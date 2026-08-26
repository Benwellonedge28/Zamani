//! Zamani Quantum Benchmarking — Execution Batching
//!
//! Production batching and execution-plan construction for the quantum
//! benchmarking subsystem.
//!
//! # Responsibility
//!
//! This module transforms validated
//! [`BenchmarkExecutionRequest`] values into deterministic, resource-bounded
//! execution batches.
//!
//! It owns:
//!
//! - batch configuration;
//! - backend/mode/priority compatibility grouping;
//! - deterministic request ordering;
//! - maximum-circuit enforcement;
//! - maximum-shot enforcement;
//! - maximum-parallelism enforcement;
//! - duplicate request detection;
//! - deterministic batch identity;
//! - deterministic batch fingerprinting;
//! - retry-safety classification;
//! - idempotency requirements;
//! - partial-batch accounting;
//! - batch-level validation;
//! - immutable batch plans;
//! - stable batch statistics.
//!
//! It does NOT own:
//!
//! - circuit generation;
//! - circuit compilation;
//! - routing;
//! - scheduling;
//! - backend capability discovery;
//! - provider communication;
//! - actual execution;
//! - sleeping/backoff;
//! - retry execution;
//! - result analysis;
//! - benchmark metrics;
//! - scientific pass/fail decisions.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! BenchmarkExperiment
//!        │
//!        ▼
//! BenchmarkExecutionRequest
//!        │
//!        ▼
//! execution::batching
//!        │
//!        ├──────── deterministic BatchPlan
//!        │
//!        ▼
//! execution::executor
//!        │
//!        ├──────── simulator
//!        ├──────── emulator
//!        ├──────── QPU
//!        └──────── external backend
//!        │
//!        ▼
//! execution::response
//! ```
//!
//! # Important boundary
//!
//! Batching is a planning operation.
//!
//! A batch is NOT assumed to be a provider job. Different backend adapters
//! may translate one `ExecutionBatch` into:
//!
//! - one provider submission;
//! - several provider submissions;
//! - one local execution task;
//! - several parallel tasks.
//!
//! The executor owns that translation.
//!
//! # Determinism
//!
//! Batch construction is deterministic for the same:
//!
//! - requests;
//! - batching policy;
//! - request identities;
//! - request fingerprints.
//!
//! Requests are sorted by stable request identity before batch assignment.
//! This prevents insertion order from changing the resulting batch layout.
//!
//! # Retry safety
//!
//! A batch does not perform retries itself.
//!
//! It does, however, explicitly classify retry safety so that the executor
//! cannot accidentally treat every batch as safely retryable.
//!
//! In particular:
//!
//! - requests with no retries require no retry protection;
//! - requests with retries and an idempotency key are retry-safe at the
//!   request-contract level;
//! - requests with retries but no idempotency key are classified as requiring
//!   backend-specific idempotency semantics;
//! - a batch containing incompatible retry semantics is rejected rather than
//!   silently making duplicate remote execution possible.
//!
//! # Partial execution
//!
//! Batching must never imply atomicity.
//!
//! A provider may successfully execute requests 0..N and fail request N+1.
//! The executor/response layer is responsible for preserving partial results.
//!
//! This module therefore preserves request membership and stable ordering so
//! partial execution can be mapped back to the originating request IDs.
//!
//! # Resource safety
//!
//! Batch construction enforces both:
//!
//! - global [`BenchmarkLimits`];
//! - local [`BatchingConfig`] limits.
//!
//! No batch may exceed:
//!
//! - maximum circuit count;
//! - maximum aggregate shots;
//! - maximum estimated metadata bytes;
//! - maximum parallelism.
//!
//! The limits are checked before a batch is materialized.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! No nightly features.
//! No unsafe code.
//!
//! # Serialization
//!
//! Batch plans are serializable because they form part of the benchmark
//! execution manifest/audit boundary.
//!
//! Request objects themselves remain the authoritative request definitions.
//! The batch contains cloned requests so the plan is self-contained and can
//! be persisted or inspected without relying on mutable external state.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! core::experiment
//!       │
//!       ▼
//! execution::request
//!       │
//!       ▼
//! execution::batching
//! ```
//!
//! Downstream:
//!
//! ```text
//! ExecutionBatch
//!       │
//!       ▼
//! execution::executor
//!       │
//!       ▼
//! execution::response
//! ```
//!
//! Related modules:
//!
//! - `core::limits` provides global resource limits.
//! - `execution::request` provides immutable validated requests.
//! - `execution::executor` consumes batch plans.
//! - `execution::response` records partial/completed/failed execution.
//!
//! This module deliberately remains independent of individual benchmark
//! protocols such as Quantum Volume, RB, XEB, QEC, VQE, and QAOA.

#![deny(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::request::{
    BackendSelection,
    BenchmarkExecutionRequest,
    ExecutionMode,
    ExecutionPriority,
};

use super::super::core::errors::BenchmarkError;
use super::super::core::limits::{
    BenchmarkLimits,
    LimitError,
};

// =============================================================================
// Schema
// =============================================================================

/// Current serialized batching schema version.
///
/// Increment this only when the serialized semantic contract changes.
pub const BATCHING_SCHEMA_VERSION: u16 = 1;

/// Stable schema identifier.
pub const BATCHING_SCHEMA_ID: &str =
    "zamani.quantum.benchmark.execution.batching";

/// Maximum length of a generated batch identifier.
pub const MAX_BATCH_ID_LENGTH: usize = 128;

/// Maximum number of batches produced by one planner invocation.
///
/// This is an additional defensive ceiling. The authoritative workload
/// ceiling remains `BenchmarkLimits::max_circuits`.
pub const MAX_BATCHES_PER_PLAN: usize = 1_000_000;

/// Maximum configured aggregate shots in one batch.
///
/// This is intentionally independent from the per-circuit shot limit in
/// `BenchmarkLimits`.
pub const DEFAULT_MAX_BATCH_SHOTS: u64 = 100_000_000;

/// Default maximum estimated metadata bytes in one batch.
pub const DEFAULT_MAX_BATCH_METADATA_BYTES: u64 = 16 * 1024 * 1024;

/// Default maximum circuits in one batch.
///
/// The actual value is additionally constrained by
/// `BenchmarkLimits::max_circuits`.
pub const DEFAULT_MAX_BATCH_CIRCUITS: usize = 1_024;

/// Maximum number of requests that may be supplied to one planner invocation
/// before the defensive batch-count calculation is performed.
pub const DEFAULT_MAX_PLAN_REQUESTS: usize = 1_000_000;

/// Maximum individual request ID length accepted by this module.
///
/// The request module already validates the actual identifier. This ceiling
/// prevents pathological deserialized values from entering batch hashing.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

// =============================================================================
// Batch retry safety
// =============================================================================

/// Retry-safety classification for an execution batch.
///
/// This classification is descriptive and is consumed by the executor.
/// `ExecutionBatch` never performs the retry itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchRetrySafety {
    /// No request in the batch requests a retry.
    NoRetryRequired,

    /// All retry-enabled requests carry idempotency keys.
    Idempotent,

    /// Retry safety depends on backend/provider semantics.
    ///
    /// The executor must establish that the backend guarantees idempotent
    /// submission or equivalent deduplication before retrying.
    BackendDependent,

    /// The batch must not be automatically retried.
    ///
    /// This is reserved for explicitly unsafe combinations.
    DoNotRetry,
}

impl BatchRetrySafety {
    /// Returns whether an executor may retry without obtaining additional
    /// backend-specific information.
    #[must_use]
    pub const fn safe_without_backend_confirmation(self) -> bool {
        matches!(
            self,
            Self::NoRetryRequired | Self::Idempotent
        )
    }

    /// Returns whether backend confirmation is required.
    #[must_use]
    pub const fn requires_backend_confirmation(self) -> bool {
        matches!(self, Self::BackendDependent)
    }

    /// Returns whether automatic retry is forbidden.
    #[must_use]
    pub const fn is_forbidden(self) -> bool {
        matches!(self, Self::DoNotRetry)
    }
}

// =============================================================================
// Batch grouping key
// =============================================================================

/// Stable compatibility key used when grouping execution requests.
///
/// Only properties whose mismatch can change the semantics of a backend
/// submission are included here.
///
/// Request IDs, circuit IDs, tags and metadata are intentionally excluded from
/// the grouping key because they identify work inside the batch rather than
/// defining backend compatibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BatchGroupingKey {
    backend: String,
    execution_mode: ExecutionMode,
    priority: ExecutionPriority,
    timeout_ms: Option<u64>,
}

impl BatchGroupingKey {
    fn from_request(request: &BenchmarkExecutionRequest) -> Self {
        Self {
            backend: canonical_backend(request.backend()),
            execution_mode: request.execution_mode(),
            priority: request.priority(),
            timeout_ms: request.timeout().map(|value| value.timeout_ms),
        }
    }

    fn canonical_form(&self) -> String {
        format!(
            "backend={};mode={};priority={};timeout_ms={}",
            self.backend,
            execution_mode_name(self.execution_mode),
            priority_name(self.priority),
            self.timeout_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
        )
    }
}

// =============================================================================
// Batching configuration
// =============================================================================

/// Production batching policy.
///
/// This policy controls how many independently validated execution requests
/// may be placed into one execution batch.
///
/// It does not override backend capability limits. Backend capability checks
/// happen in the executor/backend layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchingConfig {
    /// Maximum number of requests/circuits in one batch.
    pub max_batch_circuits: usize,

    /// Maximum aggregate shots across all requests in one batch.
    pub max_batch_shots: u64,

    /// Maximum estimated metadata bytes represented by one batch.
    pub max_batch_metadata_bytes: u64,

    /// Maximum number of batches that may be generated in one planning call.
    pub max_batches_per_plan: usize,

    /// Maximum number of input requests accepted by one planning call.
    pub max_plan_requests: usize,

    /// Whether retry-enabled requests without idempotency keys may be placed
    /// into batches.
    ///
    /// If true, the resulting batch is classified as `BackendDependent`.
    /// The executor must obtain backend confirmation before retrying it.
    ///
    /// The default is false because remote duplicate execution can be
    /// expensive or scientifically invalid.
    pub allow_backend_dependent_retries: bool,
}

impl Default for BatchingConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl BatchingConfig {
    /// Returns the production batching policy.
    pub const fn production() -> Self {
        Self {
            max_batch_circuits: DEFAULT_MAX_BATCH_CIRCUITS,
            max_batch_shots: DEFAULT_MAX_BATCH_SHOTS,
            max_batch_metadata_bytes: DEFAULT_MAX_BATCH_METADATA_BYTES,
            max_batches_per_plan: MAX_BATCHES_PER_PLAN,
            max_plan_requests: DEFAULT_MAX_PLAN_REQUESTS,
            allow_backend_dependent_retries: false,
        }
    }

    /// Creates a conservative single-request batching policy.
    ///
    /// This is useful for providers that do not support multi-circuit
    /// submissions.
    pub const fn single_request() -> Self {
        Self {
            max_batch_circuits: 1,
            max_batch_shots: DEFAULT_MAX_BATCH_SHOTS,
            max_batch_metadata_bytes: DEFAULT_MAX_BATCH_METADATA_BYTES,
            max_batches_per_plan: MAX_BATCHES_PER_PLAN,
            max_plan_requests: DEFAULT_MAX_PLAN_REQUESTS,
            allow_backend_dependent_retries: false,
        }
    }

    /// Validates the batching policy against global benchmark limits.
    pub fn validate_against(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), BatchingError> {
        limits
            .validate()
            .map_err(BatchingError::Limit)?;

        if self.max_batch_circuits == 0 {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batch_circuits",
                reason: "must be greater than zero",
            });
        }

        if self.max_batch_shots == 0 {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batch_shots",
                reason: "must be greater than zero",
            });
        }

        if self.max_batch_metadata_bytes == 0 {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batch_metadata_bytes",
                reason: "must be greater than zero",
            });
        }

        if self.max_batches_per_plan == 0 {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batches_per_plan",
                reason: "must be greater than zero",
            });
        }

        if self.max_plan_requests == 0 {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_plan_requests",
                reason: "must be greater than zero",
            });
        }

        if self.max_batch_circuits > limits.max_circuits as usize {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batch_circuits",
                reason: "cannot exceed global max_circuits",
            });
        }

        if self.max_batches_per_plan > limits.max_circuits as usize {
            return Err(BatchingError::InvalidConfiguration {
                field: "max_batches_per_plan",
                reason: "cannot exceed the maximum number of circuits",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Batch statistics
// =============================================================================

/// Immutable statistics describing one execution batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchStatistics {
    /// Number of requests/circuits in the batch.
    pub circuit_count: usize,

    /// Aggregate requested shots.
    pub total_shots: u64,

    /// Estimated metadata bytes represented by the requests.
    pub estimated_metadata_bytes: u64,

    /// Maximum requested shots among requests in the batch.
    pub maximum_request_shots: u64,

    /// Minimum requested shots among requests in the batch.
    pub minimum_request_shots: u64,

    /// Number of retry-enabled requests.
    pub retry_enabled_requests: usize,

    /// Number of requests carrying idempotency keys.
    pub idempotent_requests: usize,
}

impl BatchStatistics {
    fn from_requests(
        requests: &[BenchmarkExecutionRequest],
    ) -> Result<Self, BatchingError> {
        if requests.is_empty() {
            return Err(BatchingError::EmptyBatch);
        }

        let mut total_shots = 0u64;
        let mut metadata_bytes = 0u64;
        let mut maximum_request_shots = 0u64;
        let mut minimum_request_shots = u64::MAX;
        let mut retry_enabled_requests = 0usize;
        let mut idempotent_requests = 0usize;

        for request in requests {
            let shots = u64::try_from(request.shots())
                .map_err(|_| BatchingError::ArithmeticOverflow {
                    resource: "request shots",
                })?;

            total_shots = total_shots
                .checked_add(shots)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "aggregate batch shots",
                })?;

            let request_metadata_bytes = estimate_request_metadata_bytes(request)?;

            metadata_bytes = metadata_bytes
                .checked_add(request_metadata_bytes)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "aggregate batch metadata",
                })?;

            maximum_request_shots = maximum_request_shots.max(shots);
            minimum_request_shots = minimum_request_shots.min(shots);

            if request.retry_policy().max_retries > 0 {
                retry_enabled_requests += 1;
            }

            if request.idempotency_key().is_some() {
                idempotent_requests += 1;
            }
        }

        Ok(Self {
            circuit_count: requests.len(),
            total_shots,
            estimated_metadata_bytes: metadata_bytes,
            maximum_request_shots,
            minimum_request_shots,
            retry_enabled_requests,
            idempotent_requests,
        })
    }
}

// =============================================================================
// Execution batch
// =============================================================================

/// A deterministic, immutable group of execution requests.
///
/// The executor consumes this object but owns actual backend submission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionBatch {
    /// Batching schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: String,

    /// Stable batch identifier.
    pub batch_id: String,

    /// Requests contained in this batch.
    ///
    /// Requests are sorted by request ID.
    pub requests: Vec<BenchmarkExecutionRequest>,

    /// Aggregate batch statistics.
    pub statistics: BatchStatistics,

    /// Stable retry-safety classification.
    pub retry_safety: BatchRetrySafety,

    /// Stable grouping key used to create the batch.
    grouping_key: String,

    /// SHA-256 fingerprint over the canonical batch representation.
    fingerprint: String,
}

impl ExecutionBatch {
    /// Creates a batch from validated requests.
    ///
    /// Requests are cloned and deterministically sorted by request ID.
    pub fn new(
        mut requests: Vec<BenchmarkExecutionRequest>,
        limits: &BenchmarkLimits,
        config: &BatchingConfig,
    ) -> Result<Self, BatchingError> {
        config.validate_against(limits)?;

        if requests.is_empty() {
            return Err(BatchingError::EmptyBatch);
        }

        if requests.len() > config.max_batch_circuits {
            return Err(BatchingError::BatchCircuitLimitExceeded {
                requested: requests.len(),
                maximum: config.max_batch_circuits,
            });
        }

        let first_key = BatchGroupingKey::from_request(&requests[0]);

        for request in &requests {
            request
                .validate_against(limits)
                .map_err(|error| {
                    BatchingError::InvalidRequest {
                        request_id: request.request_id().to_string(),
                        message: error.to_string(),
                    }
                })?;

            let key = BatchGroupingKey::from_request(request);

            if key != first_key {
                return Err(BatchingError::IncompatibleRequests {
                    request_id: request.request_id().to_string(),
                    reason: "backend, execution mode, priority, or timeout differs",
                });
            }

            validate_request_id_length(request)?;
        }

        requests.sort_by(|left, right| {
            left.request_id().cmp(right.request_id())
        });

        ensure_unique_request_ids(&requests)?;

        let statistics = BatchStatistics::from_requests(&requests)?;

        limits
            .check_circuits(
                u64::try_from(statistics.circuit_count).map_err(|_| {
                    BatchingError::ArithmeticOverflow {
                        resource: "batch circuit count",
                    }
                })?,
            )
            .map_err(BatchingError::Limit)?;

        if statistics.total_shots > config.max_batch_shots {
            return Err(BatchingError::BatchShotLimitExceeded {
                requested: statistics.total_shots,
                maximum: config.max_batch_shots,
            });
        }

        if statistics.estimated_metadata_bytes
            > config.max_batch_metadata_bytes
        {
            return Err(BatchingError::BatchMetadataLimitExceeded {
                requested: statistics.estimated_metadata_bytes,
                maximum: config.max_batch_metadata_bytes,
            });
        }

        let retry_safety =
            determine_retry_safety(&requests, config)?;

        let grouping_key = first_key.canonical_form();

        let fingerprint = calculate_batch_fingerprint(
            &requests,
            &grouping_key,
            retry_safety,
        )?;

        let batch_id = format!("batch-{}", &fingerprint[..32]);

        validate_batch_id(&batch_id)?;

        Ok(Self {
            schema_version: BATCHING_SCHEMA_VERSION,
            schema_id: BATCHING_SCHEMA_ID.to_owned(),
            batch_id,
            requests,
            statistics,
            retry_safety,
            grouping_key,
            fingerprint,
        })
    }

    /// Returns the stable batch ID.
    pub fn batch_id(&self) -> &str {
        &self.batch_id
    }

    /// Returns all requests in deterministic order.
    pub fn requests(&self) -> &[BenchmarkExecutionRequest] {
        &self.requests
    }

    /// Returns batch statistics.
    pub const fn statistics(&self) -> BatchStatistics {
        self.statistics
    }

    /// Returns retry-safety classification.
    pub const fn retry_safety(&self) -> BatchRetrySafety {
        self.retry_safety
    }

    /// Returns the canonical grouping key.
    pub fn grouping_key(&self) -> &str {
        &self.grouping_key
    }

    /// Returns the deterministic batch fingerprint.
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns whether this batch contains exactly one request.
    pub const fn is_single_request(&self) -> bool {
        self.statistics.circuit_count == 1
    }

    /// Returns whether this batch contains multiple requests.
    pub const fn is_multi_request(&self) -> bool {
        self.statistics.circuit_count > 1
    }

    /// Returns all request IDs in deterministic order.
    pub fn request_ids(&self) -> Vec<String> {
        self.requests
            .iter()
            .map(|request| request.request_id().to_string())
            .collect()
    }

    /// Finds a request by stable request ID.
    pub fn find_request(
        &self,
        request_id: &str,
    ) -> Option<&BenchmarkExecutionRequest> {
        self.requests
            .binary_search_by(|request| {
                request.request_id().as_str().cmp(request_id)
            })
            .ok()
            .map(|index| &self.requests[index])
    }

    /// Returns whether a request belongs to this batch.
    pub fn contains_request(&self, request_id: &str) -> bool {
        self.find_request(request_id).is_some()
    }

    /// Returns a stable canonical representation.
    ///
    /// This representation is used for fingerprinting and auditing. It is
    /// deliberately independent from Rust's `Debug` implementation.
    pub fn canonical_form(&self) -> String {
        let mut output = String::new();

        append_field(
            &mut output,
            "schema_version",
            &self.schema_version.to_string(),
        );

        append_field(
            &mut output,
            "schema_id",
            &self.schema_id,
        );

        append_field(
            &mut output,
            "grouping_key",
            &self.grouping_key,
        );

        append_field(
            &mut output,
            "retry_safety",
            retry_safety_name(self.retry_safety),
        );

        for request in &self.requests {
            append_field(
                &mut output,
                "request",
                &request.canonical_form(),
            );
        }

        output
    }

    /// Recalculates and verifies the stored fingerprint.
    pub fn verify_fingerprint(&self) -> Result<(), BatchingError> {
        let calculated = calculate_batch_fingerprint(
            &self.requests,
            &self.grouping_key,
            self.retry_safety,
        )?;

        if calculated != self.fingerprint {
            return Err(BatchingError::FingerprintMismatch {
                batch_id: self.batch_id.clone(),
            });
        }

        Ok(())
    }

    /// Validates the complete batch independently of the planner.
    ///
    /// This method is intended to be called after deserialization before a
    /// persisted batch is trusted.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
        config: &BatchingConfig,
    ) -> Result<(), BatchingError> {
        if self.schema_version != BATCHING_SCHEMA_VERSION {
            return Err(BatchingError::UnsupportedSchemaVersion {
                actual: self.schema_version,
                expected: BATCHING_SCHEMA_VERSION,
            });
        }

        if self.schema_id != BATCHING_SCHEMA_ID {
            return Err(BatchingError::InvalidSchema {
                reason: "schema ID does not match the batching schema",
            });
        }

        let rebuilt = Self::new(
            self.requests.clone(),
            limits,
            config,
        )?;

        if rebuilt.batch_id != self.batch_id {
            return Err(BatchingError::FingerprintMismatch {
                batch_id: self.batch_id.clone(),
            });
        }

        if rebuilt.fingerprint != self.fingerprint {
            return Err(BatchingError::FingerprintMismatch {
                batch_id: self.batch_id.clone(),
            });
        }

        if rebuilt.retry_safety != self.retry_safety {
            return Err(BatchingError::InvalidSchema {
                reason: "stored retry-safety classification is inconsistent",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Execution batch plan
// =============================================================================

/// Complete deterministic execution plan produced by [`BatchPlanner`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchPlan {
    /// Batching schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: String,

    /// Immutable execution batches.
    pub batches: Vec<ExecutionBatch>,

    /// Number of input requests.
    pub request_count: usize,

    /// Total requested shots across all requests.
    pub total_shots: u64,

    /// Number of circuits represented by the plan.
    pub total_circuits: usize,

    /// Number of requests that requested retries.
    pub retry_enabled_requests: usize,

    /// Number of retry-enabled requests carrying idempotency keys.
    pub idempotent_retry_requests: usize,

    /// Stable plan fingerprint.
    pub fingerprint: String,
}

impl BatchPlan {
    fn new(
        batches: Vec<ExecutionBatch>,
        request_count: usize,
    ) -> Result<Self, BatchingError> {
        if batches.is_empty() && request_count != 0 {
            return Err(BatchingError::InvalidPlan {
                reason: "non-empty request set produced no batches",
            });
        }

        let mut total_shots = 0u64;
        let mut total_circuits = 0usize;
        let mut retry_enabled_requests = 0usize;
        let mut idempotent_retry_requests = 0usize;

        for batch in &batches {
            total_shots = total_shots
                .checked_add(batch.statistics.total_shots)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "plan total shots",
                })?;

            total_circuits = total_circuits
                .checked_add(batch.statistics.circuit_count)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "plan total circuits",
                })?;

            retry_enabled_requests = retry_enabled_requests
                .checked_add(
                    batch.statistics.retry_enabled_requests,
                )
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "plan retry request count",
                })?;

            idempotent_retry_requests =
                idempotent_retry_requests
                    .checked_add(
                        batch.statistics.idempotent_requests,
                    )
                    .ok_or(BatchingError::ArithmeticOverflow {
                        resource: "plan idempotent request count",
                    })?;
        }

        if total_circuits != request_count {
            return Err(BatchingError::InvalidPlan {
                reason: "batch circuit count does not equal request count",
            });
        }

        let fingerprint = calculate_plan_fingerprint(&batches)?;

        Ok(Self {
            schema_version: BATCHING_SCHEMA_VERSION,
            schema_id: BATCHING_SCHEMA_ID.to_owned(),
            batches,
            request_count,
            total_shots,
            total_circuits,
            retry_enabled_requests,
            idempotent_retry_requests,
            fingerprint,
        })
    }

    /// Returns all batches.
    pub fn batches(&self) -> &[ExecutionBatch] {
        &self.batches
    }

    /// Returns the number of batches.
    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }

    /// Returns whether no execution is required.
    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }

    /// Returns the plan fingerprint.
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Finds a batch containing a request.
    pub fn find_batch_for_request(
        &self,
        request_id: &str,
    ) -> Option<&ExecutionBatch> {
        self.batches
            .iter()
            .find(|batch| batch.contains_request(request_id))
    }

    /// Returns every request ID represented by the plan.
    pub fn request_ids(&self) -> Vec<String> {
        self.batches
            .iter()
            .flat_map(|batch| batch.request_ids())
            .collect()
    }

    /// Validates all batches and the complete plan.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
        config: &BatchingConfig,
    ) -> Result<(), BatchingError> {
        if self.schema_version != BATCHING_SCHEMA_VERSION {
            return Err(BatchingError::UnsupportedSchemaVersion {
                actual: self.schema_version,
                expected: BATCHING_SCHEMA_VERSION,
            });
        }

        if self.schema_id != BATCHING_SCHEMA_ID {
            return Err(BatchingError::InvalidSchema {
                reason: "schema ID does not match the batching schema",
            });
        }

        if self.batches.len() > config.max_batches_per_plan {
            return Err(BatchingError::PlanBatchLimitExceeded {
                requested: self.batches.len(),
                maximum: config.max_batches_per_plan,
            });
        }

        let mut ids = BTreeSet::new();

        for batch in &self.batches {
            batch.validate(limits, config)?;

            for request in batch.requests() {
                let id = request.request_id().to_string();

                if !ids.insert(id.clone()) {
                    return Err(BatchingError::DuplicateRequestId {
                        request_id: id,
                    });
                }
            }
        }

        let rebuilt = Self::new(
            self.batches.clone(),
            self.request_count,
        )?;

        if rebuilt.total_shots != self.total_shots
            || rebuilt.total_circuits != self.total_circuits
            || rebuilt.retry_enabled_requests
                != self.retry_enabled_requests
            || rebuilt.idempotent_retry_requests
                != self.idempotent_retry_requests
        {
            return Err(BatchingError::InvalidPlan {
                reason: "stored plan statistics are inconsistent",
            });
        }

        if rebuilt.fingerprint != self.fingerprint {
            return Err(BatchingError::FingerprintMismatch {
                batch_id: "plan".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Batch planner
// =============================================================================

/// Deterministic production batch planner.
///
/// The planner is stateless. Keeping it stateless makes it safe to construct
/// per benchmark execution and makes deterministic testing straightforward.
#[derive(Debug, Clone, Copy)]
pub struct BatchPlanner {
    limits: BenchmarkLimits,
    config: BatchingConfig,
}

impl BatchPlanner {
    /// Creates a planner using the supplied resource limits and batching
    /// configuration.
    pub fn new(
        limits: BenchmarkLimits,
        config: BatchingConfig,
    ) -> Result<Self, BatchingError> {
        config.validate_against(&limits)?;

        Ok(Self { limits, config })
    }

    /// Creates a planner using production limits and production batching
    /// defaults.
    pub fn production() -> Result<Self, BatchingError> {
        Self::new(
            BenchmarkLimits::production(),
            BatchingConfig::production(),
        )
    }

    /// Returns the global benchmark limits used by this planner.
    pub const fn limits(&self) -> BenchmarkLimits {
        self.limits
    }

    /// Returns the batching configuration.
    pub const fn config(&self) -> BatchingConfig {
        self.config
    }

    /// Plans execution batches deterministically.
    ///
    /// The supplied requests are never mutated.
    pub fn plan(
        &self,
        requests: &[BenchmarkExecutionRequest],
    ) -> Result<BatchPlan, BatchingError> {
        self.config.validate_against(&self.limits)?;

        if requests.len() > self.config.max_plan_requests {
            return Err(BatchingError::PlanRequestLimitExceeded {
                requested: requests.len(),
                maximum: self.config.max_plan_requests,
            });
        }

        if requests.is_empty() {
            return BatchPlan::new(Vec::new(), 0);
        }

        validate_all_requests(requests, &self.limits)?;

        ensure_unique_request_ids(requests)?;

        let mut sorted_requests = requests.to_vec();

        sorted_requests.sort_by(|left, right| {
            left.request_id().cmp(right.request_id())
        });

        let groups =
            group_requests(&sorted_requests);

        let mut batches = Vec::new();

        for group in groups {
            self.plan_group(&group, &mut batches)?;
        }

        if batches.len() > self.config.max_batches_per_plan {
            return Err(BatchingError::PlanBatchLimitExceeded {
                requested: batches.len(),
                maximum: self.config.max_batches_per_plan,
            });
        }

        BatchPlan::new(batches, requests.len())
    }

    fn plan_group(
        &self,
        requests: &[BenchmarkExecutionRequest],
        batches: &mut Vec<ExecutionBatch>,
    ) -> Result<(), BatchingError> {
        let mut current: Vec<BenchmarkExecutionRequest> = Vec::new();

        for request in requests {
            let would_exceed_circuits =
                current.len() >= self.config.max_batch_circuits;

            let current_shots =
                aggregate_shots(&current)?;

            let request_shots = u64::try_from(request.shots())
                .map_err(|_| BatchingError::ArithmeticOverflow {
                    resource: "request shots",
                })?;

            let combined_shots = current_shots
                .checked_add(request_shots)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "combined batch shots",
                })?;

            let current_metadata =
                aggregate_metadata_bytes(&current)?;

            let request_metadata =
                estimate_request_metadata_bytes(request)?;

            let combined_metadata = current_metadata
                .checked_add(request_metadata)
                .ok_or(BatchingError::ArithmeticOverflow {
                    resource: "combined batch metadata",
                })?;

            let would_exceed_shots =
                combined_shots > self.config.max_batch_shots;

            let would_exceed_metadata =
                combined_metadata
                    > self.config.max_batch_metadata_bytes;

            if !current.is_empty()
                && (would_exceed_circuits
                    || would_exceed_shots
                    || would_exceed_metadata)
            {
                let batch = ExecutionBatch::new(
                    std::mem::take(&mut current),
                    &self.limits,
                    &self.config,
                )?;

                batches.push(batch);

                if batches.len() > self.config.max_batches_per_plan {
                    return Err(BatchingError::PlanBatchLimitExceeded {
                        requested: batches.len(),
                        maximum: self.config.max_batches_per_plan,
                    });
                }
            }

            let request_shots =
                u64::try_from(request.shots()).map_err(|_| {
                    BatchingError::ArithmeticOverflow {
                        resource: "request shots",
                    }
                })?;

            let request_metadata =
                estimate_request_metadata_bytes(request)?;

            if request_shots > self.config.max_batch_shots {
                return Err(BatchingError::SingleRequestShotLimitExceeded {
                    request_id: request.request_id().to_string(),
                    requested: request_shots,
                    maximum: self.config.max_batch_shots,
                });
            }

            if request_metadata > self.config.max_batch_metadata_bytes {
                return Err(
                    BatchingError::SingleRequestMetadataLimitExceeded {
                        request_id: request.request_id().to_string(),
                        requested: request_metadata,
                        maximum: self.config.max_batch_metadata_bytes,
                    },
                );
            }

            current.push(request.clone());
        }

        if !current.is_empty() {
            let batch = ExecutionBatch::new(
                current,
                &self.limits,
                &self.config,
            )?;

            batches.push(batch);
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by batch construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchingError {
    /// No requests were supplied where a batch was required.
    EmptyBatch,

    /// The batching configuration is invalid.
    InvalidConfiguration {
        /// Configuration field.
        field: &'static str,

        /// Reason the value is invalid.
        reason: &'static str,
    },

    /// An execution request failed validation.
    InvalidRequest {
        /// Stable request identifier.
        request_id: String,

        /// Validation explanation.
        message: String,
    },

    /// Two requests cannot share a batch.
    IncompatibleRequests {
        /// Request that caused the incompatibility.
        request_id: String,

        /// Explanation.
        reason: &'static str,
    },

    /// Duplicate request identifiers were supplied.
    DuplicateRequestId {
        /// Duplicate identifier.
        request_id: String,
    },

    /// A batch would exceed its circuit-count limit.
    BatchCircuitLimitExceeded {
        /// Requested count.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// A batch would exceed its aggregate-shot limit.
    BatchShotLimitExceeded {
        /// Requested aggregate shots.
        requested: u64,

        /// Maximum aggregate shots.
        maximum: u64,
    },

    /// A batch would exceed its metadata limit.
    BatchMetadataLimitExceeded {
        /// Requested bytes.
        requested: u64,

        /// Maximum bytes.
        maximum: u64,
    },

    /// One request alone exceeds the batch shot limit.
    SingleRequestShotLimitExceeded {
        /// Request identifier.
        request_id: String,

        /// Requested shots.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// One request alone exceeds the batch metadata limit.
    SingleRequestMetadataLimitExceeded {
        /// Request identifier.
        request_id: String,

        /// Requested bytes.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// The complete plan exceeds the maximum number of input requests.
    PlanRequestLimitExceeded {
        /// Number supplied.
        requested: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// The complete plan exceeds the maximum number of batches.
    PlanBatchLimitExceeded {
        /// Number generated.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Resource being calculated.
        resource: &'static str,
    },

    /// A resource limit was violated.
    Limit(LimitError),

    /// Retry-enabled requests without idempotency protection are not allowed
    /// under the current batching policy.
    RetrySafetyViolation {
        /// Request identifier.
        request_id: String,
    },

    /// A generated batch identifier is invalid.
    InvalidBatchId,

    /// Stored/generated fingerprint does not match canonical data.
    FingerprintMismatch {
        /// Batch or plan identifier.
        batch_id: String,
    },

    /// Unsupported serialized schema version.
    UnsupportedSchemaVersion {
        /// Actual version.
        actual: u16,

        /// Expected current version.
        expected: u16,
    },

    /// Invalid serialized schema.
    InvalidSchema {
        /// Explanation.
        reason: &'static str,
    },

    /// Stored plan structure is inconsistent.
    InvalidPlan {
        /// Explanation.
        reason: &'static str,
    },
}

impl fmt::Display for BatchingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBatch => {
                f.write_str("execution batch cannot be empty")
            }

            Self::InvalidConfiguration { field, reason } => {
                write!(
                    f,
                    "invalid batching configuration '{field}': {reason}"
                )
            }

            Self::InvalidRequest {
                request_id,
                message,
            } => {
                write!(
                    f,
                    "execution request '{request_id}' is invalid: {message}"
                )
            }

            Self::IncompatibleRequests {
                request_id,
                reason,
            } => {
                write!(
                    f,
                    "request '{request_id}' is incompatible with its batch: {reason}"
                )
            }

            Self::DuplicateRequestId { request_id } => {
                write!(
                    f,
                    "duplicate execution request ID: {request_id}"
                )
            }

            Self::BatchCircuitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "batch contains {requested} circuits; maximum is {maximum}"
                )
            }

            Self::BatchShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "batch requests {requested} aggregate shots; maximum is {maximum}"
                )
            }

            Self::BatchMetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "batch metadata requires {requested} bytes; maximum is {maximum}"
                )
            }

            Self::SingleRequestShotLimitExceeded {
                request_id,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request '{request_id}' requires {requested} shots, exceeding batch maximum {maximum}"
                )
            }

            Self::SingleRequestMetadataLimitExceeded {
                request_id,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request '{request_id}' requires {requested} metadata bytes, exceeding batch maximum {maximum}"
                )
            }

            Self::PlanRequestLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "plan contains {requested} requests; maximum is {maximum}"
                )
            }

            Self::PlanBatchLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "plan contains {requested} batches; maximum is {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "batch arithmetic overflowed while calculating {resource}"
                )
            }

            Self::Limit(error) => {
                write!(f, "{error}")
            }

            Self::RetrySafetyViolation { request_id } => {
                write!(
                    f,
                    "request '{request_id}' enables retries without an idempotency key; backend-dependent retry semantics are disabled"
                )
            }

            Self::InvalidBatchId => {
                f.write_str("generated batch ID is invalid")
            }

            Self::FingerprintMismatch { batch_id } => {
                write!(
                    f,
                    "fingerprint mismatch for batch/plan '{batch_id}'"
                )
            }

            Self::UnsupportedSchemaVersion {
                actual,
                expected,
            } => {
                write!(
                    f,
                    "unsupported batching schema version {actual}; expected {expected}"
                )
            }

            Self::InvalidSchema { reason } => {
                write!(f, "invalid batching schema: {reason}")
            }

            Self::InvalidPlan { reason } => {
                write!(f, "invalid execution batch plan: {reason}")
            }
        }
    }
}

impl std::error::Error for BatchingError {}

impl From<BatchingError> for BenchmarkError {
    fn from(error: BatchingError) -> Self {
        BenchmarkError::InvalidExecutionRequest {
            message: error.to_string(),
        }
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

fn validate_all_requests(
    requests: &[BenchmarkExecutionRequest],
    limits: &BenchmarkLimits,
) -> Result<(), BatchingError> {
    for request in requests {
        request
            .validate_against(limits)
            .map_err(|error| BatchingError::InvalidRequest {
                request_id: request.request_id().to_string(),
                message: error.to_string(),
            })?;

        validate_request_id_length(request)?;
    }

    Ok(())
}

fn validate_request_id_length(
    request: &BenchmarkExecutionRequest,
) -> Result<(), BatchingError> {
    if request.request_id().as_str().len() > MAX_REQUEST_ID_LENGTH {
        return Err(BatchingError::InvalidRequest {
            request_id: request.request_id().to_string(),
            message: "request ID exceeds the batching fingerprint limit"
                .to_owned(),
        });
    }

    Ok(())
}

fn ensure_unique_request_ids(
    requests: &[BenchmarkExecutionRequest],
) -> Result<(), BatchingError> {
    let mut ids = BTreeSet::new();

    for request in requests {
        let id = request.request_id().to_string();

        if !ids.insert(id.clone()) {
            return Err(BatchingError::DuplicateRequestId {
                request_id: id,
            });
        }
    }

    Ok(())
}

fn group_requests(
    requests: &[BenchmarkExecutionRequest],
) -> Vec<Vec<BenchmarkExecutionRequest>> {
    let mut groups: BTreeMap<
        BatchGroupingKey,
        Vec<BenchmarkExecutionRequest>,
    > = BTreeMap::new();

    for request in requests {
        let key = BatchGroupingKey::from_request(request);

        groups
            .entry(key)
            .or_default()
            .push(request.clone());
    }

    groups.into_values().collect()
}

fn aggregate_shots(
    requests: &[BenchmarkExecutionRequest],
) -> Result<u64, BatchingError> {
    requests.iter().try_fold(0u64, |total, request| {
        let shots = u64::try_from(request.shots())
            .map_err(|_| BatchingError::ArithmeticOverflow {
                resource: "request shots",
            })?;

        total
            .checked_add(shots)
            .ok_or(BatchingError::ArithmeticOverflow {
                resource: "aggregate batch shots",
            })
    })
}

fn aggregate_metadata_bytes(
    requests: &[BenchmarkExecutionRequest],
) -> Result<u64, BatchingError> {
    requests.iter().try_fold(0u64, |total, request| {
        let bytes = estimate_request_metadata_bytes(request)?;

        total
            .checked_add(bytes)
            .ok_or(BatchingError::ArithmeticOverflow {
                resource: "aggregate batch metadata",
            })
    })
}

/// Estimates only metadata that travels with the execution request.
///
/// This is deliberately not a circuit-size estimate because the request layer
/// references a `CircuitId` rather than embedding the circuit representation.
fn estimate_request_metadata_bytes(
    request: &BenchmarkExecutionRequest,
) -> Result<u64, BatchingError> {
    let mut bytes = 0u64;

    let add_len = |total: &mut u64, value: &str| {
        let len = u64::try_from(value.len()).map_err(|_| {
            BatchingError::ArithmeticOverflow {
                resource: "metadata length",
            }
        })?;

        *total = total.checked_add(len).ok_or(
            BatchingError::ArithmeticOverflow {
                resource: "metadata size",
            },
        )?;

        Ok::<(), BatchingError>(())
    };

    add_len(&mut bytes, request.request_id().as_str())?;
    add_len(&mut bytes, &request.experiment_id().to_string())?;
    add_len(&mut bytes, &request.circuit_id().to_string())?;

    for tag in request.tags() {
        add_len(&mut bytes, tag)?;
    }

    for (key, value) in request.metadata() {
        add_len(&mut bytes, key)?;
        add_len(&mut bytes, value)?;
    }

    if let Some(idempotency_key) = request.idempotency_key() {
        add_len(&mut bytes, idempotency_key)?;
    }

    // Account for fixed-width scalar fields conservatively.
    bytes = bytes
        .checked_add(128)
        .ok_or(BatchingError::ArithmeticOverflow {
            resource: "fixed request metadata",
        })?;

    Ok(bytes)
}

fn determine_retry_safety(
    requests: &[BenchmarkExecutionRequest],
    config: &BatchingConfig,
) -> Result<BatchRetrySafety, BatchingError> {
    let mut retry_enabled = false;
    let mut all_retry_enabled_are_idempotent = true;

    for request in requests {
        let retries_enabled =
            request.retry_policy().max_retries > 0;

        if !retries_enabled {
            continue;
        }

        retry_enabled = true;

        if request.idempotency_key().is_none() {
            all_retry_enabled_are_idempotent = false;

            if !config.allow_backend_dependent_retries {
                return Err(BatchingError::RetrySafetyViolation {
                    request_id: request.request_id().to_string(),
                });
            }
        }
    }

    if !retry_enabled {
        return Ok(BatchRetrySafety::NoRetryRequired);
    }

    if all_retry_enabled_are_idempotent {
        Ok(BatchRetrySafety::Idempotent)
    } else {
        Ok(BatchRetrySafety::BackendDependent)
    }
}

fn canonical_backend(backend: &BackendSelection) -> String {
    match backend {
        BackendSelection::Default => "default".to_owned(),
        BackendSelection::Id(id) => format!("id:{id}"),
    }
}

fn calculate_batch_fingerprint(
    requests: &[BenchmarkExecutionRequest],
    grouping_key: &str,
    retry_safety: BatchRetrySafety,
) -> Result<String, BatchingError> {
    let mut canonical = String::new();

    append_field(
        &mut canonical,
        "schema_version",
        &BATCHING_SCHEMA_VERSION.to_string(),
    );

    append_field(
        &mut canonical,
        "schema_id",
        BATCHING_SCHEMA_ID,
    );

    append_field(
        &mut canonical,
        "grouping_key",
        grouping_key,
    );

    append_field(
        &mut canonical,
        "retry_safety",
        retry_safety_name(retry_safety),
    );

    for request in requests {
        append_field(
            &mut canonical,
            "request",
            &request.canonical_form(),
        );
    }

    Ok(sha256_hex(canonical.as_bytes()))
}

fn calculate_plan_fingerprint(
    batches: &[ExecutionBatch],
) -> Result<String, BatchingError> {
    let mut canonical = String::new();

    append_field(
        &mut canonical,
        "schema_version",
        &BATCHING_SCHEMA_VERSION.to_string(),
    );

    append_field(
        &mut canonical,
        "schema_id",
        BATCHING_SCHEMA_ID,
    );

    for batch in batches {
        append_field(
            &mut canonical,
            "batch",
            batch.fingerprint(),
        );
    }

    Ok(sha256_hex(canonical.as_bytes()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);

    let mut output = String::with_capacity(64);

    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }

    output
}

fn append_field(
    output: &mut String,
    name: &str,
    value: &str,
) {
    output.push_str(name);
    output.push('=');

    // Length-prefixing prevents ambiguous concatenations such as:
    // ("ab", "c") versus ("a", "bc").
    output.push_str(&value.len().to_string());
    output.push(':');
    output.push_str(value);
    output.push('\n');
}

fn validate_batch_id(batch_id: &str) -> Result<(), BatchingError> {
    if batch_id.is_empty()
        || batch_id.len() > MAX_BATCH_ID_LENGTH
        || !batch_id.starts_with("batch-")
    {
        return Err(BatchingError::InvalidBatchId);
    }

    if !batch_id
        .chars()
        .all(|character| {
            character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
        })
    {
        return Err(BatchingError::InvalidBatchId);
    }

    Ok(())
}

fn execution_mode_name(mode: ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::Simulator => "simulator",
        ExecutionMode::Emulator => "emulator",
        ExecutionMode::Qpu => "qpu",
        ExecutionMode::Auto => "auto",
    }
}

fn priority_name(priority: ExecutionPriority) -> &'static str {
    match priority {
        ExecutionPriority::Background => "background",
        ExecutionPriority::Normal => "normal",
        ExecutionPriority::High => "high",
        ExecutionPriority::Critical => "critical",
    }
}

fn retry_safety_name(
    safety: BatchRetrySafety,
) -> &'static str {
    match safety {
        BatchRetrySafety::NoRetryRequired => "no_retry_required",
        BatchRetrySafety::Idempotent => "idempotent",
        BatchRetrySafety::BackendDependent => "backend_dependent",
        BatchRetrySafety::DoNotRetry => "do_not_retry",
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::CircuitId;
    use crate::quantum::benchmarking::core::provenance::ExperimentId;

    fn request(
        id: &str,
        shots: usize,
    ) -> BenchmarkExecutionRequest {
        let request_id = super::super::request::ExecutionRequestId::new(id)
            .expect("test request ID must be valid");

        BenchmarkExecutionRequest::new(
            request_id,
            ExperimentId::new("experiment-test")
                .expect("test experiment ID must be valid"),
            CircuitId::new("circuit-test")
                .expect("test circuit ID must be valid"),
            shots,
        )
        .expect("test request must be valid")
    }

    #[test]
    fn empty_plan_is_valid() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let plan = planner
            .plan(&[])
            .expect("empty plan must succeed");

        assert!(plan.is_empty());
        assert_eq!(plan.batch_count(), 0);
        assert_eq!(plan.request_count, 0);
    }

    #[test]
    fn one_request_produces_one_batch() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let requests = vec![request("request-1", 100)];

        let plan = planner
            .plan(&requests)
            .expect("planning must succeed");

        assert_eq!(plan.batch_count(), 1);
        assert_eq!(plan.total_circuits, 1);
        assert_eq!(plan.total_shots, 100);
    }

    #[test]
    fn compatible_requests_are_batched() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let requests = vec![
            request("request-2", 100),
            request("request-1", 200),
            request("request-3", 300),
        ];

        let plan = planner
            .plan(&requests)
            .expect("planning must succeed");

        assert_eq!(plan.batch_count(), 1);
        assert_eq!(plan.total_circuits, 3);
        assert_eq!(plan.total_shots, 600);

        assert_eq!(
            plan.batches()[0].request_ids(),
            vec![
                "request-1".to_owned(),
                "request-2".to_owned(),
                "request-3".to_owned(),
            ]
        );
    }

    #[test]
    fn request_order_does_not_change_fingerprint() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let a = request("request-a", 100);
        let b = request("request-b", 200);

        let first = planner
            .plan(&[a.clone(), b.clone()])
            .expect("first plan must succeed");

        let second = planner
            .plan(&[b, a])
            .expect("second plan must succeed");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn duplicate_request_ids_are_rejected() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let requests = vec![
            request("duplicate", 100),
            request("duplicate", 200),
        ];

        let result = planner.plan(&requests);

        assert!(matches!(
            result,
            Err(BatchingError::DuplicateRequestId { .. })
        ));
    }

    #[test]
    fn excessive_batch_shots_split_batches() {
        let limits = BenchmarkLimits::production();

        let config = BatchingConfig {
            max_batch_circuits: 100,
            max_batch_shots: 500,
            max_batch_metadata_bytes:
                DEFAULT_MAX_BATCH_METADATA_BYTES,
            max_batches_per_plan: MAX_BATCHES_PER_PLAN,
            max_plan_requests: DEFAULT_MAX_PLAN_REQUESTS,
            allow_backend_dependent_retries: false,
        };

        let planner =
            BatchPlanner::new(limits, config)
                .expect("planner must be valid");

        let requests = vec![
            request("request-1", 300),
            request("request-2", 300),
        ];

        let plan = planner
            .plan(&requests)
            .expect("planning must succeed");

        assert_eq!(plan.batch_count(), 2);
        assert_eq!(plan.total_shots, 600);
    }

    #[test]
    fn retry_without_idempotency_is_rejected_by_default() {
        let request = request("retry-request", 100)
            .with_retry_policy(
                super::super::request::RetryPolicy::new(
                    2,
                    100,
                    1_000,
                )
                .expect("retry policy must be valid"),
            )
            .expect("request retry policy must be valid");

        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let result = planner.plan(&[request]);

        assert!(matches!(
            result,
            Err(BatchingError::RetrySafetyViolation { .. })
        ));
    }

    #[test]
    fn idempotent_retry_is_accepted() {
        let request = request("retry-request", 100)
            .with_retry_policy(
                super::super::request::RetryPolicy::new(
                    2,
                    100,
                    1_000,
                )
                .expect("retry policy must be valid"),
            )
            .expect("request retry policy must be valid")
            .with_idempotency_key("retry-key-1")
            .expect("idempotency key must be valid");

        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let plan = planner
            .plan(&[request])
            .expect("planning must succeed");

        assert_eq!(
            plan.batches()[0].retry_safety(),
            BatchRetrySafety::Idempotent
        );
    }

    #[test]
    fn batch_fingerprint_verifies() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let plan = planner
            .plan(&[
                request("request-a", 100),
                request("request-b", 200),
            ])
            .expect("planning must succeed");

        plan.validate(
            &planner.limits(),
            &planner.config(),
        )
        .expect("plan must validate");

        for batch in plan.batches() {
            batch
                .verify_fingerprint()
                .expect("fingerprint must verify");
        }
    }

    #[test]
    fn find_request_uses_stable_id() {
        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let plan = planner
            .plan(&[
                request("request-a", 100),
                request("request-b", 200),
            ])
            .expect("planning must succeed");

        let batch = plan
            .find_batch_for_request("request-b")
            .expect("request must exist");

        assert!(
            batch.find_request("request-b").is_some()
        );

        assert!(
            batch.find_request("does-not-exist").is_none()
        );
    }

    #[test]
    fn single_request_policy_disables_multi_request_batches() {
        let planner = BatchPlanner::new(
            BenchmarkLimits::production(),
            BatchingConfig::single_request(),
        )
        .expect("planner must be valid");

        let plan = planner
            .plan(&[
                request("request-a", 100),
                request("request-b", 100),
            ])
            .expect("planning must succeed");

        assert_eq!(plan.batch_count(), 2);

        assert!(
            plan.batches()
                .iter()
                .all(ExecutionBatch::is_single_request)
        );
    }

    #[test]
    fn grouping_by_backend_is_deterministic() {
        let request_a = request("request-a", 100)
            .with_backend(
                BackendSelection::id("backend-a")
                    .expect("backend ID must be valid"),
            )
            .expect("backend selection must be valid");

        let request_b = request("request-b", 100)
            .with_backend(
                BackendSelection::id("backend-b")
                    .expect("backend ID must be valid"),
            )
            .expect("backend selection must be valid");

        let planner =
            BatchPlanner::production().expect("planner must be valid");

        let plan = planner
            .plan(&[request_b, request_a])
            .expect("planning must succeed");

        assert_eq!(plan.batch_count(), 2);

        assert_eq!(
            plan.batches()[0].requests()[0]
                .backend()
                .backend_id(),
            Some("backend-a")
        );

        assert_eq!(
            plan.batches()[1].requests()[0]
                .backend()
                .backend_id(),
            Some("backend-b")
        );
    }
}