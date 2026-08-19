//! Zamani Quantum Error-Correction Configuration
//!
//! Production-grade configuration boundary for the QEC subsystem.
//!
//! Design:
//!
//!     Untrusted Configuration
//!              |
//!              v
//!     Parse / Construct
//!              |
//!              v
//!     Structural Validation
//!              |
//!              v
//!     Resource / Security Validation
//!              |
//!              v
//!     Validated QecConfig
//!              |
//!              +----------------------+
//!              |                      |
//!              v                      v
//!        Local Execution         QPU Execution
//!              |                      |
//!       CPU/GPU/Accelerator     Hardware Adapter
//!              |                      |
//!              +----------+-----------+
//!                         |
//!                         v
//!                    QEC Pipeline
//!
//! This module intentionally owns configuration policy rather than
//! implementing decoding, scheduling, backend execution, or capability
//! authorization itself.
//!
//! Important:
//! - Configuration validation must happen before expensive allocation.
//! - No configuration may imply unlimited resources.
//! - QPU support is backend-neutral and vendor-neutral.
//! - Secrets, credentials, API tokens, and private keys do not belong here.
//! - Remote QPU access is explicitly opt-in.
//! - Configuration must be serializable and versionable.
//! - Invalid configuration returns an error instead of panicking.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Current configuration schema version.
///
/// Persisted configurations should record this value so future versions
/// can reject or migrate incompatible configuration data explicitly.
pub const CONFIGURATION_SCHEMA_VERSION: u32 = 1;

/// Maximum representable timeout used by this module.
///
/// This is deliberately finite so malformed external values cannot express
/// an accidental "effectively infinite" timeout.
pub const MAX_TIMEOUT_MS: u64 = 7 * 24 * 60 * 60 * 1_000;

/// Maximum permitted parallelism.
pub const MAX_PARALLELISM: u32 = 1_000_000;

/// Maximum permitted QPU shots in one configured operation.
pub const MAX_QPU_SHOTS: u64 = 1_000_000_000;

/// Maximum permitted checkpoint interval.
pub const MAX_CHECKPOINT_INTERVAL_EVENTS: u64 = 10_000_000_000;

/// Configuration result type.
pub type ConfigurationResult<T> = Result<T, ConfigurationError>;

/// Production configuration for the complete QEC execution pipeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QecConfig {
    /// Persistent schema version.
    pub schema_version: u32,

    /// Human-readable configuration name.
    pub name: String,

    /// Global resource limits.
    pub limits: QecLimits,

    /// Decoder configuration.
    pub decoder: DecoderConfig,

    /// Numerical safety policy.
    pub numerical: NumericalPolicy,

    /// Execution backend policy.
    pub backend: BackendConfig,

    /// Parallel execution policy.
    pub parallelism: ParallelismConfig,

    /// Deterministic execution policy.
    pub determinism: DeterminismConfig,

    /// Checkpoint/resume policy.
    pub checkpointing: CheckpointConfig,

    /// Streaming syndrome policy.
    pub streaming: StreamingConfig,

    /// Partitioning policy.
    pub partitioning: PartitionConfig,

    /// Distributed execution policy.
    pub distributed: DistributedConfig,

    /// Scheduler policy.
    pub scheduler: SchedulerConfig,

    /// Memory-management policy.
    pub memory: MemoryConfig,

    /// Cache policy.
    pub cache: CacheConfig,

    /// Telemetry policy.
    pub telemetry: TelemetryConfig,

    /// Security policy.
    pub security: SecurityConfig,

    /// Capability requirements.
    pub capabilities: CapabilityConfig,

    /// QPU execution policy.
    pub qpu: QpuConfig,
}

impl Default for QecConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl QecConfig {
    /// Conservative production configuration.
    ///
    /// This configuration intentionally avoids unbounded resources.
    pub fn production() -> Self {
        Self {
            schema_version: CONFIGURATION_SCHEMA_VERSION,
            name: "zamani-qec-production".to_string(),
            limits: QecLimits::default(),
            decoder: DecoderConfig::default(),
            numerical: NumericalPolicy::default(),
            backend: BackendConfig::default(),
            parallelism: ParallelismConfig::default(),
            determinism: DeterminismConfig::default(),
            checkpointing: CheckpointConfig::default(),
            streaming: StreamingConfig::default(),
            partitioning: PartitionConfig::default(),
            distributed: DistributedConfig::default(),
            scheduler: SchedulerConfig::default(),
            memory: MemoryConfig::default(),
            cache: CacheConfig::default(),
            telemetry: TelemetryConfig::default(),
            security: SecurityConfig::default(),
            capabilities: CapabilityConfig::default(),
            qpu: QpuConfig::default(),
        }
    }

    /// Deterministic local configuration suitable for reproducible tests.
    pub fn deterministic_test() -> Self {
        let mut config = Self::production();

        config.determinism.enabled = true;
        config.determinism.seed = Some(0x5A4D_414E_4951_4543);
        config.parallelism.max_workers = 1;
        config.parallelism.deterministic_reductions = true;
        config.streaming.deterministic_order = true;

        config
    }

    /// Validate the entire configuration before execution.
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.schema_version != CONFIGURATION_SCHEMA_VERSION {
            return Err(ConfigurationError::UnsupportedSchemaVersion {
                expected: CONFIGURATION_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }

        if self.name.trim().is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "name",
                reason: "configuration name must not be empty",
            });
        }

        self.limits.validate()?;
        self.decoder.validate()?;
        self.numerical.validate()?;
        self.backend.validate()?;
        self.parallelism.validate()?;
        self.determinism.validate()?;
        self.checkpointing.validate()?;
        self.streaming.validate()?;
        self.partitioning.validate()?;
        self.distributed.validate()?;
        self.scheduler.validate()?;
        self.memory.validate()?;
        self.cache.validate()?;
        self.telemetry.validate()?;
        self.security.validate()?;
        self.capabilities.validate()?;
        self.qpu.validate()?;

        self.validate_cross_component_invariants()
    }

    /// Validate relationships between otherwise-valid configuration sections.
    fn validate_cross_component_invariants(&self) -> ConfigurationResult<()> {
        if self.memory.max_bytes > self.limits.max_memory_bytes {
            return Err(ConfigurationError::InvalidValue {
                field: "memory.max_bytes",
                reason: "memory budget exceeds limits.max_memory_bytes",
            });
        }

        if self.parallelism.max_workers > self.limits.max_parallelism {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.max_workers",
                reason: "parallelism exceeds limits.max_parallelism",
            });
        }

        if self.streaming.buffer_capacity_events > self.limits.max_syndrome_events {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.buffer_capacity_events",
                reason: "streaming buffer exceeds maximum syndrome events",
            });
        }

        if self.partitioning.enabled && self.partitioning.partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.partitions",
                reason: "enabled partitioning requires at least one partition",
            });
        }

        if self.distributed.enabled && !self.partitioning.enabled {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.enabled",
                reason: "distributed execution requires partitioning to be enabled",
            });
        }

        if self.backend.requires_qpu() && !self.qpu.enabled {
            return Err(ConfigurationError::BackendRequiresQpu);
        }

        if self.backend.requires_accelerator() && !self.capabilities.accelerator {
            return Err(ConfigurationError::InvalidValue {
                field: "capabilities.accelerator",
                reason: "accelerated backend requires accelerator capability",
            });
        }

        if self.backend.requires_distributed() && !self.capabilities.distributed_execution {
            return Err(ConfigurationError::InvalidValue {
                field: "capabilities.distributed_execution",
                reason: "distributed backend requires distributed capability",
            });
        }

        if self.determinism.enabled && !self.parallelism.deterministic_reductions {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.deterministic_reductions",
                reason: "deterministic mode requires deterministic reductions",
            });
        }

        if self.qpu.enabled && !self.capabilities.qpu_execution {
            return Err(ConfigurationError::InvalidValue {
                field: "capabilities.qpu_execution",
                reason: "QPU execution requires explicit QPU capability",
            });
        }

        if self.qpu.allow_remote && !self.security.allow_remote_execution {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.allow_remote",
                reason: "remote QPU execution is disabled by security policy",
            });
        }

        if self.qpu.enabled
            && self.qpu.shots > self.limits.max_qpu_shots
        {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.shots",
                reason: "QPU shots exceed configured QPU resource limit",
            });
        }

        Ok(())
    }
}

/// Global resource limits.
///
/// These limits are intentionally finite and configurable. They prevent
/// constructors or external workloads from converting large integer inputs
/// into catastrophic allocations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QecLimits {
    pub max_code_distance: u64,
    pub max_qubits: u64,
    pub max_stabilizers: u64,
    pub max_syndrome_events: u64,
    pub max_rounds: u64,
    pub max_graph_nodes: u64,
    pub max_graph_edges: u64,
    pub max_memory_bytes: u64,
    pub max_decoder_time_ms: u64,
    pub max_parallelism: u32,
    pub max_checkpoint_size_bytes: u64,
    pub max_qpu_shots: u64,
    pub max_qpu_circuits: u64,
}

impl Default for QecLimits {
    fn default() -> Self {
        Self {
            max_code_distance: 1_000,
            max_qubits: 10_000_000,
            max_stabilizers: 10_000_000,
            max_syndrome_events: 100_000_000,
            max_rounds: 10_000_000,
            max_graph_nodes: 100_000_000,
            max_graph_edges: 500_000_000,
            max_memory_bytes: 16 * 1024 * 1024 * 1024,
            max_decoder_time_ms: 24 * 60 * 60 * 1_000,
            max_parallelism: 256,
            max_checkpoint_size_bytes: 4 * 1024 * 1024 * 1024,
            max_qpu_shots: MAX_QPU_SHOTS,
            max_qpu_circuits: 1_000_000,
        }
    }
}

impl QecLimits {
    pub fn validate(&self) -> ConfigurationResult<()> {
        check_nonzero("limits.max_code_distance", self.max_code_distance)?;
        check_nonzero("limits.max_qubits", self.max_qubits)?;
        check_nonzero("limits.max_stabilizers", self.max_stabilizers)?;
        check_nonzero("limits.max_syndrome_events", self.max_syndrome_events)?;
        check_nonzero("limits.max_rounds", self.max_rounds)?;
        check_nonzero("limits.max_graph_nodes", self.max_graph_nodes)?;
        check_nonzero("limits.max_graph_edges", self.max_graph_edges)?;
        check_nonzero("limits.max_memory_bytes", self.max_memory_bytes)?;
        check_nonzero("limits.max_decoder_time_ms", self.max_decoder_time_ms)?;
        check_nonzero("limits.max_parallelism", self.max_parallelism)?;
        check_nonzero(
            "limits.max_checkpoint_size_bytes",
            self.max_checkpoint_size_bytes,
        )?;

        if self.max_decoder_time_ms > MAX_TIMEOUT_MS {
            return Err(ConfigurationError::InvalidValue {
                field: "limits.max_decoder_time_ms",
                reason: "decoder time exceeds hard safety ceiling",
            });
        }

        if self.max_parallelism > MAX_PARALLELISM {
            return Err(ConfigurationError::InvalidValue {
                field: "limits.max_parallelism",
                reason: "parallelism exceeds hard safety ceiling",
            });
        }

        if self.max_qpu_shots == 0 || self.max_qpu_shots > MAX_QPU_SHOTS {
            return Err(ConfigurationError::InvalidValue {
                field: "limits.max_qpu_shots",
                reason: "QPU shot limit is outside the supported safety range",
            });
        }

        Ok(())
    }
}

/// Decoder selection and algorithm policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecoderConfig {
    pub strategy: DecoderStrategy,
    pub max_iterations: u64,
    pub use_soft_information: bool,
    pub enable_post_selection: bool,
    pub allow_fallback: bool,
    pub fallback_strategy: Option<DecoderStrategy>,
    pub logical_failure_policy: LogicalFailurePolicy,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            strategy: DecoderStrategy::Mwpm,
            max_iterations: 1_000_000,
            use_soft_information: true,
            enable_post_selection: false,
            allow_fallback: true,
            fallback_strategy: Some(DecoderStrategy::UnionFind),
            logical_failure_policy: LogicalFailurePolicy::Report,
        }
    }
}

impl DecoderConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.max_iterations == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "decoder.max_iterations",
                reason: "maximum decoder iterations must be greater than zero",
            });
        }

        if self.allow_fallback && self.fallback_strategy.is_none() {
            return Err(ConfigurationError::InvalidValue {
                field: "decoder.fallback_strategy",
                reason: "fallback strategy must be configured when fallback is enabled",
            });
        }

        Ok(())
    }
}

/// Supported decoder families.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DecoderStrategy {
    Mwpm,
    UnionFind,
    BeliefPropagation,
    BeliefPropagationMwpm,
    TensorNetwork,
    LookupTable,
    Custom,
}

/// Policy for logical decoding failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogicalFailurePolicy {
    Report,
    Retry,
    Fallback,
    Abort,
}

/// Numerical safety policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericalPolicy {
    pub reject_nan: bool,
    pub reject_infinity: bool,
    pub reject_negative_zero: bool,
    pub strict_probability_validation: bool,
    pub probability_epsilon: f64,
    pub weight_epsilon: f64,
    pub overflow_policy: OverflowPolicy,
    pub underflow_policy: UnderflowPolicy,
    pub floating_point_mode: FloatingPointMode,
}

impl Default for NumericalPolicy {
    fn default() -> Self {
        Self {
            reject_nan: true,
            reject_infinity: true,
            reject_negative_zero: false,
            strict_probability_validation: true,
            probability_epsilon: 1e-15,
            weight_epsilon: 1e-15,
            overflow_policy: OverflowPolicy::Error,
            underflow_policy: UnderflowPolicy::Clamp,
            floating_point_mode: FloatingPointMode::Strict,
        }
    }
}

impl NumericalPolicy {
    pub fn validate(&self) -> ConfigurationResult<()> {
        validate_finite_nonnegative(
            "numerical.probability_epsilon",
            self.probability_epsilon,
        )?;

        validate_finite_nonnegative(
            "numerical.weight_epsilon",
            self.weight_epsilon,
        )?;

        if self.probability_epsilon >= 1.0 {
            return Err(ConfigurationError::InvalidValue {
                field: "numerical.probability_epsilon",
                reason: "probability epsilon must be less than one",
            });
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OverflowPolicy {
    Error,
    Saturate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnderflowPolicy {
    Error,
    Clamp,
    Zero,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FloatingPointMode {
    Strict,
    Reproducible,
    Fast,
}

/// Execution backend configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendConfig {
    pub kind: BackendKind,
    pub device_id: Option<String>,
    pub require_hardware: bool,
    pub allow_software_fallback: bool,
    pub maximum_in_flight_operations: u32,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            kind: BackendKind::Cpu,
            device_id: None,
            require_hardware: false,
            allow_software_fallback: true,
            maximum_in_flight_operations: 1,
        }
    }
}

impl BackendConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.maximum_in_flight_operations == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.maximum_in_flight_operations",
                reason: "must be greater than zero",
            });
        }

        if matches!(self.kind, BackendKind::Qpu) && self.device_id.is_none() {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.device_id",
                reason: "QPU backend requires an explicit device identifier",
            });
        }

        if self.require_hardware && self.allow_software_fallback {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.allow_software_fallback",
                reason: "software fallback conflicts with require_hardware",
            });
        }

        Ok(())
    }

    pub fn requires_qpu(&self) -> bool {
        matches!(self.kind, BackendKind::Qpu)
    }

    pub fn requires_accelerator(&self) -> bool {
        matches!(
            self.kind,
            BackendKind::Gpu
                | BackendKind::Accelerator
                | BackendKind::QpuHybrid
        )
    }

    pub fn requires_distributed(&self) -> bool {
        matches!(self.kind, BackendKind::Distributed)
    }
}

/// Supported execution backends.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BackendKind {
    Cpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
    Qpu,
    QpuHybrid,
}

/// Parallelism configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParallelismConfig {
    pub enabled: bool,
    pub max_workers: u32,
    pub deterministic_reductions: bool,
    pub chunk_size: usize,
    pub work_stealing: bool,
}

impl Default for ParallelismConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_workers: 8,
            deterministic_reductions: true,
            chunk_size: 4_096,
            work_stealing: true,
        }
    }
}

impl ParallelismConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.max_workers == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.max_workers",
                reason: "must be greater than zero",
            });
        }

        if self.max_workers > MAX_PARALLELISM {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.max_workers",
                reason: "exceeds hard safety ceiling",
            });
        }

        if self.chunk_size == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.chunk_size",
                reason: "must be greater than zero",
            });
        }

        Ok(())
    }
}

/// Deterministic execution configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeterminismConfig {
    pub enabled: bool,
    pub seed: Option<u64>,
    pub deterministic_scheduling: bool,
    pub deterministic_reductions: bool,
    pub deterministic_serialization: bool,
}

impl Default for DeterminismConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            seed: None,
            deterministic_scheduling: true,
            deterministic_reductions: true,
            deterministic_serialization: true,
        }
    }
}

impl DeterminismConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled && !self.deterministic_scheduling {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism.deterministic_scheduling",
                reason: "deterministic mode requires deterministic scheduling",
            });
        }

        if self.enabled && !self.deterministic_reductions {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism.deterministic_reductions",
                reason: "deterministic mode requires deterministic reductions",
            });
        }

        Ok(())
    }
}

/// Checkpoint configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckpointConfig {
    pub enabled: bool,
    pub interval_events: u64,
    pub max_size_bytes: u64,
    pub retain_count: u32,
    pub integrity: CheckpointIntegrity,
    pub compression: CheckpointCompression,
    pub require_compatible_schema: bool,
    pub allow_resume: bool,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_events: 1_000_000,
            max_size_bytes: 512 * 1024 * 1024,
            retain_count: 3,
            integrity: CheckpointIntegrity::Sha256,
            compression: CheckpointCompression::Zstd,
            require_compatible_schema: true,
            allow_resume: true,
        }
    }
}

impl CheckpointConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled {
            if self.interval_events == 0 {
                return Err(ConfigurationError::InvalidValue {
                    field: "checkpointing.interval_events",
                    reason: "must be greater than zero when checkpointing is enabled",
                });
            }

            if self.interval_events > MAX_CHECKPOINT_INTERVAL_EVENTS {
                return Err(ConfigurationError::InvalidValue {
                    field: "checkpointing.interval_events",
                    reason: "checkpoint interval exceeds hard safety ceiling",
                });
            }

            if self.max_size_bytes == 0 {
                return Err(ConfigurationError::InvalidValue {
                    field: "checkpointing.max_size_bytes",
                    reason: "checkpoint size must be greater than zero",
                });
            }

            if self.retain_count == 0 {
                return Err(ConfigurationError::InvalidValue {
                    field: "checkpointing.retain_count",
                    reason: "retain_count must be greater than zero",
                });
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CheckpointIntegrity {
    Sha256,
    Sha3_256,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CheckpointCompression {
    None,
    Zstd,
}

/// Streaming syndrome configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub buffer_capacity_events: u64,
    pub batch_size: u64,
    pub backpressure: BackpressurePolicy,
    pub deterministic_order: bool,
    pub drop_on_overflow: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_capacity_events: 1_000_000,
            batch_size: 4_096,
            backpressure: BackpressurePolicy::Block,
            deterministic_order: true,
            drop_on_overflow: false,
        }
    }
}

impl StreamingConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled {
            if self.buffer_capacity_events == 0 {
                return Err(ConfigurationError::InvalidValue {
                    field: "streaming.buffer_capacity_events",
                    reason: "must be greater than zero",
                });
            }

            if self.batch_size == 0 {
                return Err(ConfigurationError::InvalidValue {
                    field: "streaming.batch_size",
                    reason: "must be greater than zero",
                });
            }

            if self.drop_on_overflow
                && matches!(self.backpressure, BackpressurePolicy::Block)
            {
                return Err(ConfigurationError::InvalidValue {
                    field: "streaming.drop_on_overflow",
                    reason: "cannot drop events while using blocking backpressure",
                });
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BackpressurePolicy {
    Block,
    Reject,
    SpillToCheckpoint,
}

/// Partitioning configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionConfig {
    pub enabled: bool,
    pub partitions: u32,
    pub max_events_per_partition: u64,
    pub overlap_events: u64,
    pub preserve_boundaries: bool,
    pub boundary_reconciliation: BoundaryReconciliation,
}

impl Default for PartitionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            partitions: 1,
            max_events_per_partition: 10_000_000,
            overlap_events: 1_024,
            preserve_boundaries: true,
            boundary_reconciliation: BoundaryReconciliation::GlobalMatching,
        }
    }
}

impl PartitionConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled && self.partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.partitions",
                reason: "must be greater than zero",
            });
        }

        if self.enabled && !self.preserve_boundaries {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.preserve_boundaries",
                reason: "QEC partitioning cannot disable boundary preservation",
            });
        }

        if self.max_events_per_partition == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.max_events_per_partition",
                reason: "must be greater than zero",
            });
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BoundaryReconciliation {
    GlobalMatching,
    Hierarchical,
    Exact,
}

/// Distributed execution configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributedConfig {
    pub enabled: bool,
    pub coordinator: CoordinatorMode,
    pub worker_count: u32,
    pub require_authenticated_workers: bool,
    pub require_encrypted_transport: bool,
    pub max_in_flight_partitions: u32,
    pub retry_count: u32,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            coordinator: CoordinatorMode::Local,
            worker_count: 1,
            require_authenticated_workers: true,
            require_encrypted_transport: true,
            max_in_flight_partitions: 8,
            retry_count: 2,
        }
    }
}

impl DistributedConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled && self.worker_count == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.worker_count",
                reason: "must be greater than zero",
            });
        }

        if self.enabled && self.max_in_flight_partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.max_in_flight_partitions",
                reason: "must be greater than zero",
            });
        }

        if self.enabled && !self.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "distributed QEC transport must use encryption".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CoordinatorMode {
    Local,
    External,
    Embedded,
}

/// Scheduler configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub max_queued_jobs: usize,
    pub max_running_jobs: u32,
    pub admission_policy: AdmissionPolicy,
    pub priority_policy: PriorityPolicy,
    pub enable_deadlines: bool,
    pub enable_cancellation: bool,
    pub enable_backpressure: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_queued_jobs: 1_024,
            max_running_jobs: 8,
            admission_policy: AdmissionPolicy::ResourceAware,
            priority_policy: PriorityPolicy::CriticalFirst,
            enable_deadlines: true,
            enable_cancellation: true,
            enable_backpressure: true,
        }
    }
}

impl SchedulerConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.max_queued_jobs == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "scheduler.max_queued_jobs",
                reason: "must be greater than zero",
            });
        }

        if self.max_running_jobs == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "scheduler.max_running_jobs",
                reason: "must be greater than zero",
            });
        }

        if !self.enable_cancellation {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "production QEC scheduling must support cancellation".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AdmissionPolicy {
    ResourceAware,
    Strict,
    BestEffort,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PriorityPolicy {
    CriticalFirst,
    Fair,
    FIFO,
}

/// Memory management policy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub max_bytes: u64,
    pub reserve_bytes: u64,
    pub arena_enabled: bool,
    pub sparse_allocation: bool,
    pub bounded_buffers: bool,
    pub eviction_policy: EvictionPolicy,
    pub track_peak_usage: bool,
    pub fail_on_budget_exhaustion: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_bytes: 8 * 1024 * 1024 * 1024,
            reserve_bytes: 256 * 1024 * 1024,
            arena_enabled: true,
            sparse_allocation: true,
            bounded_buffers: true,
            eviction_policy: EvictionPolicy::LeastRecentlyUsed,
            track_peak_usage: true,
            fail_on_budget_exhaustion: true,
        }
    }
}

impl MemoryConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.max_bytes == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "memory.max_bytes",
                reason: "must be greater than zero",
            });
        }

        if self.reserve_bytes >= self.max_bytes {
            return Err(ConfigurationError::InvalidValue {
                field: "memory.reserve_bytes",
                reason: "reserve must be smaller than maximum memory",
            });
        }

        if !self.bounded_buffers {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "production QEC configuration requires bounded buffers".to_string(),
            ));
        }

        if !self.fail_on_budget_exhaustion {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "production QEC configuration must fail safely on memory exhaustion"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    None,
    LeastRecentlyUsed,
    LeastFrequentlyUsed,
}

/// Cache configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub max_bytes: u64,
    pub topology: bool,
    pub stabilizer_neighborhoods: bool,
    pub graph_templates: bool,
    pub boundary_information: bool,
    pub decoder_configuration: bool,
    pub verify_integrity: bool,
    pub recompute_on_corruption: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_bytes: 512 * 1024 * 1024,
            topology: true,
            stabilizer_neighborhoods: true,
            graph_templates: true,
            boundary_information: true,
            decoder_configuration: true,
            verify_integrity: true,
            recompute_on_corruption: true,
        }
    }
}

impl CacheConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.enabled && self.max_bytes == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "cache.max_bytes",
                reason: "must be greater than zero when caching is enabled",
            });
        }

        if self.enabled && !self.recompute_on_corruption {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "cache corruption must trigger discard/recomputation".to_string(),
            ));
        }

        Ok(())
    }
}

/// Telemetry configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub metrics: bool,
    pub traces: bool,
    pub events: bool,
    pub include_resource_usage: bool,
    pub include_decoder_statistics: bool,
    pub include_qpu_statistics: bool,
    pub export_remote: bool,
    pub sampling_rate: f64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: true,
            traces: false,
            events: true,
            include_resource_usage: true,
            include_decoder_statistics: true,
            include_qpu_statistics: true,
            export_remote: false,
            sampling_rate: 1.0,
        }
    }
}

impl TelemetryConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if !self.sampling_rate.is_finite()
            || self.sampling_rate < 0.0
            || self.sampling_rate > 1.0
        {
            return Err(ConfigurationError::InvalidValue {
                field: "telemetry.sampling_rate",
                reason: "sampling rate must be finite and between zero and one",
            });
        }

        if self.export_remote && !self.enabled {
            return Err(ConfigurationError::InvalidValue {
                field: "telemetry.export_remote",
                reason: "remote telemetry requires telemetry to be enabled",
            });
        }

        Ok(())
    }
}

/// Security configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub validate_external_input: bool,
    pub reject_malformed_input: bool,
    pub reject_resource_bombs: bool,
    pub allow_remote_execution: bool,
    pub require_authenticated_backends: bool,
    pub require_encrypted_transport: bool,
    pub verify_checkpoints: bool,
    pub reject_unknown_configuration_fields: bool,
    pub protect_sensitive_metadata: bool,
    pub audit_capability_usage: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            validate_external_input: true,
            reject_malformed_input: true,
            reject_resource_bombs: true,
            allow_remote_execution: false,
            require_authenticated_backends: true,
            require_encrypted_transport: true,
            verify_checkpoints: true,
            reject_unknown_configuration_fields: true,
            protect_sensitive_metadata: true,
            audit_capability_usage: true,
        }
    }
}

impl SecurityConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if !self.validate_external_input {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "external QEC configuration validation cannot be disabled".to_string(),
            ));
        }

        if !self.reject_malformed_input {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "malformed QEC input must be rejected".to_string(),
            ));
        }

        if !self.reject_resource_bombs {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "resource-bomb protection cannot be disabled".to_string(),
            ));
        }

        if !self.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "encrypted transport is required for production QEC".to_string(),
            ));
        }

        Ok(())
    }
}

/// Capability requirements.
///
/// These are requirements, not grants. Actual authorization belongs to the
/// capability subsystem.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityConfig {
    pub decode: bool,
    pub simulate: bool,
    pub benchmark: bool,
    pub inspect_topology: bool,
    pub allocate_memory: bool,
    pub accelerator: bool,
    pub distributed_execution: bool,
    pub qpu_execution: bool,
    pub remote_execution: bool,
}

impl Default for CapabilityConfig {
    fn default() -> Self {
        Self {
            decode: true,
            simulate: true,
            benchmark: false,
            inspect_topology: true,
            allocate_memory: true,
            accelerator: false,
            distributed_execution: false,
            qpu_execution: false,
            remote_execution: false,
        }
    }
}

impl CapabilityConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.qpu_execution && !self.allocate_memory {
            return Err(ConfigurationError::InvalidValue {
                field: "capabilities.allocate_memory",
                reason: "QPU execution requires resource-management capability",
            });
        }

        if self.remote_execution && !self.qpu_execution {
            // Remote execution may eventually be used by non-QPU backends,
            // but it must be explicitly represented by the backend/security
            // layer before being enabled.
        }

        Ok(())
    }
}

/// Production QPU configuration.
///
/// This configuration is intentionally vendor-neutral. Vendor-specific
/// credentials, SDK handles, network clients, and private authentication
/// material must remain outside persistent QEC configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QpuConfig {
    pub enabled: bool,

    /// Hardware adapter identifier.
    pub adapter: QpuAdapter,

    /// Device identifier supplied by the adapter.
    pub device_id: Option<String>,

    /// Number of shots requested for a circuit.
    pub shots: u64,

    /// Maximum circuits submitted by one QEC operation.
    pub max_circuits: u64,

    /// Whether the adapter must verify that calibration data is available.
    pub verify_calibration: bool,

    /// Optional calibration epoch/version required by the workload.
    pub required_calibration_epoch: Option<String>,

    /// Reset qubits before execution.
    pub reset_before_run: bool,

    /// Readout strategy.
    pub readout_mode: QpuReadoutMode,

    /// QPU measurement timeout.
    pub measurement_timeout_ms: u64,

    /// Queue/admission timeout.
    pub queue_timeout_ms: u64,

    /// Hardware execution policy.
    pub execution_policy: QpuExecutionPolicy,

    /// Connectivity model.
    pub connectivity: QpuConnectivity,

    /// QPU error model.
    pub error_model: QpuErrorModel,

    /// Permit remote hardware.
    pub allow_remote: bool,

    /// Require hardware-level authentication.
    pub require_authenticated_device: bool,

    /// Require encrypted communication.
    pub require_encrypted_transport: bool,

    /// Permit software simulation fallback.
    pub allow_simulator_fallback: bool,

    /// Require the QPU result to be verified before being accepted.
    pub verify_results: bool,

    /// Maximum number of concurrent QPU jobs.
    pub max_in_flight_jobs: u32,
}

impl Default for QpuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            adapter: QpuAdapter::Custom,
            device_id: None,
            shots: 1_024,
            max_circuits: 1,
            verify_calibration: true,
            required_calibration_epoch: None,
            reset_before_run: true,
            readout_mode: QpuReadoutMode::Counts,
            measurement_timeout_ms: 60_000,
            queue_timeout_ms: 300_000,
            execution_policy: QpuExecutionPolicy::RejectIfUnavailable,
            connectivity: QpuConnectivity::HardwareDefined,
            error_model: QpuErrorModel::HardwareReported,
            allow_remote: false,
            require_authenticated_device: true,
            require_encrypted_transport: true,
            allow_simulator_fallback: false,
            verify_results: true,
            max_in_flight_jobs: 1,
        }
    }
}

impl QpuConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let device_id = self.device_id.as_deref().unwrap_or("").trim();

        if device_id.is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.device_id",
                reason: "enabled QPU execution requires an explicit device ID",
            });
        }

        if self.shots == 0 || self.shots > MAX_QPU_SHOTS {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.shots",
                reason: "QPU shots are outside the supported safety range",
            });
        }

        if self.max_circuits == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.max_circuits",
                reason: "must be greater than zero",
            });
        }

        if self.max_in_flight_jobs == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.max_in_flight_jobs",
                reason: "must be greater than zero",
            });
        }

        validate_timeout(
            "qpu.measurement_timeout_ms",
            self.measurement_timeout_ms,
        )?;

        validate_timeout(
            "qpu.queue_timeout_ms",
            self.queue_timeout_ms,
        )?;

        if self.allow_remote && !self.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "remote QPU execution requires encrypted transport".to_string(),
            ));
        }

        if self.allow_remote && !self.require_authenticated_device {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "remote QPU execution requires authenticated devices".to_string(),
            ));
        }

        if self.allow_simulator_fallback
            && matches!(
                self.execution_policy,
                QpuExecutionPolicy::RequireHardware
            )
        {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.allow_simulator_fallback",
                reason: "simulator fallback conflicts with RequireHardware",
            });
        }

        Ok(())
    }
}

/// Vendor-neutral QPU adapter selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuAdapter {
    Custom,
    LocalHardware,
    RemoteHardware,
    IBM,
    IonTrap,
    Superconducting,
    NeutralAtom,
    Photonic,
    TrappedIon,
}

/// QPU execution availability policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuExecutionPolicy {
    RequireHardware,
    RejectIfUnavailable,
    AllowSimulatorFallback,
}

/// QPU readout representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuReadoutMode {
    Counts,
    Samples,
    Probabilities,
    RawBitstrings,
}

/// QPU physical connectivity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuConnectivity {
    HardwareDefined,
    AllToAll,
    Line,
    Grid,
    HeavyHex,
    Custom,
}

/// QPU error-model source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuErrorModel {
    HardwareReported,
    Configured,
    CalibrationDerived,
    None,
}

/// Configuration errors.
///
/// These are intentionally configuration-focused. The future `errors.rs`
/// module can map these into the global `QecError` hierarchy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigurationError {
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },
    UnsupportedSchemaVersion {
        expected: u32,
        found: u32,
    },
    BackendRequiresQpu,
    SecurityPolicyViolation(String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => {
                write!(formatter, "invalid configuration field `{field}`: {reason}")
            }

            Self::UnsupportedSchemaVersion { expected, found } => {
                write!(
                    formatter,
                    "unsupported configuration schema version: expected {expected}, found {found}"
                )
            }

            Self::BackendRequiresQpu => {
                write!(formatter, "selected backend requires QPU configuration")
            }

            Self::SecurityPolicyViolation(reason) => {
                write!(formatter, "security policy violation: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigurationError {}

fn check_nonzero(field: &'static str, value: u64) -> ConfigurationResult<()> {
    if value == 0 {
        return Err(ConfigurationError::InvalidValue {
            field,
            reason: "value must be greater than zero",
        });
    }

    Ok(())
}

fn validate_finite_nonnegative(
    field: &'static str,
    value: f64,
) -> ConfigurationResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(ConfigurationError::InvalidValue {
            field,
            reason: "value must be finite and non-negative",
        });
    }

    Ok(())
}

fn validate_timeout(
    field: &'static str,
    value: u64,
) -> ConfigurationResult<()> {
    if value == 0 {
        return Err(ConfigurationError::InvalidValue {
            field,
            reason: "timeout must be greater than zero",
        });
    }

    if value > MAX_TIMEOUT_MS {
        return Err(ConfigurationError::InvalidValue {
            field,
            reason: "timeout exceeds hard safety ceiling",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_configuration_is_valid() {
        let config = QecConfig::production();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn deterministic_configuration_is_valid() {
        let config = QecConfig::deterministic_test();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn zero_memory_is_rejected() {
        let mut config = QecConfig::production();
        config.limits.max_memory_bytes = 0;

        assert!(config.validate().is_err());
    }

    #[test]
    fn memory_budget_cannot_exceed_global_limit() {
        let mut config = QecConfig::production();
        config.memory.max_bytes = config.limits.max_memory_bytes + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn deterministic_mode_requires_deterministic_reductions() {
        let mut config = QecConfig::production();

        config.determinism.enabled = true;
        config.parallelism.deterministic_reductions = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_execution_requires_partitioning() {
        let mut config = QecConfig::production();

        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_execution_requires_encryption() {
        let mut config = QecConfig::production();

        config.partitioning.enabled = true;
        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;
        config.distributed.require_encrypted_transport = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_requires_explicit_capability() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Qpu;
        config.backend.device_id = Some("qpu-0".to_string());
        config.qpu.enabled = true;
        config.qpu.device_id = Some("qpu-0".to_string());

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_configuration_can_be_enabled_safely() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Qpu;
        config.backend.device_id = Some("qpu-0".to_string());

        config.capabilities.qpu_execution = true;

        config.qpu.enabled = true;
        config.qpu.device_id = Some("qpu-0".to_string());
        config.qpu.shots = 10_000;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn remote_qpu_requires_security_policy() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Qpu;
        config.backend.device_id = Some("remote-qpu".to_string());

        config.capabilities.qpu_execution = true;
        config.qpu.enabled = true;
        config.qpu.device_id = Some("remote-qpu".to_string());
        config.qpu.allow_remote = true;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_shot_bomb_is_rejected() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Qpu;
        config.backend.device_id = Some("qpu-0".to_string());

        config.capabilities.qpu_execution = true;
        config.qpu.enabled = true;
        config.qpu.device_id = Some("qpu-0".to_string());
        config.qpu.shots = MAX_QPU_SHOTS + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_probability_epsilon_is_rejected() {
        let mut config = QecConfig::production();
        config.numerical.probability_epsilon = f64::NAN;

        assert!(config.validate().is_err());
    }

    #[test]
    fn malformed_streaming_configuration_is_rejected() {
        let mut config = QecConfig::production();

        config.streaming.enabled = true;
        config.streaming.batch_size = 0;

        assert!(config.validate().is_err());
    }

    #[test]
    fn cache_corruption_must_be_recoverable() {
        let mut config = QecConfig::production();

        config.cache.recompute_on_corruption = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_round_trips_through_json() {
        let config = QecConfig::production();

        let serialized =
            serde_json::to_string(&config).expect("configuration should serialize");

        let restored: QecConfig =
            serde_json::from_str(&serialized).expect("configuration should deserialize");

        assert_eq!(config, restored);
        assert!(restored.validate().is_ok());
    }

    #[test]
    fn schema_version_is_checked() {
        let mut config = QecConfig::production();
        config.schema_version += 1;

        match config.validate() {
            Err(ConfigurationError::UnsupportedSchemaVersion { .. }) => {}
            other => panic!("unexpected validation result: {other:?}"),
        }
    }

    #[test]
    fn qpu_hybrid_requires_accelerator_capability() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::QpuHybrid;
        config.capabilities.accelerator = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn memory_reserve_cannot_equal_budget() {
        let mut config = QecConfig::production();

        config.memory.reserve_bytes = config.memory.max_bytes;

        assert!(config.validate().is_err());
    }
}