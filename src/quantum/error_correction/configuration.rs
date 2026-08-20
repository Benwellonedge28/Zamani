//! Zamani Quantum Error-Correction Configuration
//!
//! Production configuration boundary for the QEC subsystem.
//!
//! # Ownership
//!
//! `configuration.rs` owns:
//!
//! - configuration composition;
//! - configuration schema/version;
//! - cross-component policy validation;
//! - declarative execution requirements;
//! - configuration serialization/deserialization;
//! - configuration-level invariants.
//!
//! It does NOT own:
//!
//! - canonical resource ceilings (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - capability authority (`capabilities.rs`);
//! - backend execution (`backend.rs`);
//! - QPU I/O (`qpu_adapter.rs`);
//! - decoder algorithms (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - scheduler execution;
//! - telemetry transport.
//!
//! # Architectural contract
//!
//! ```text
//!                         Untrusted configuration
//!                                  |
//!                                  v
//!                              QecConfig
//!                                  |
//!              +-------------------+-------------------+
//!              |                   |                   |
//!              v                   v                   v
//!          QecLimits          Security            Capabilities
//!          limits.rs          policy              requirements
//!              |                   |                   |
//!              +-------------------+-------------------+
//!                                  |
//!                                  v
//!                       Local validation
//!                                  |
//!                                  v
//!                    Cross-component validation
//!                                  |
//!                                  v
//!                         Backend preflight
//!                                  |
//!                                  v
//!                       Runtime admission
//!                                  |
//!                                  v
//!                        ResourceManager
//!                                  |
//!                                  v
//!                           QEC execution
//! ```
//!
//! # Important security rule
//!
//! `CapabilityConfig` contains requirements only. It never grants authority.
//! Actual authorization is performed by `capabilities.rs`.
//!
//! Configuration must never contain:
//!
//! - passwords;
//! - private keys;
//! - API tokens;
//! - QPU credentials;
//! - authentication secrets;
//! - network credentials.
//!
//! # Resource policy rule
//!
//! `QecLimits` is the single canonical QEC resource policy.
//!
//! This module may reject a configuration because a local policy is internally
//! inconsistent, but it must not create a second production resource-limit
//! system.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and stable standard-library APIs.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use super::backend::BackendKind;
use super::limits::QecLimits;

// ============================================================================
// Schema
// ============================================================================

/// Serialized configuration schema version.
///
/// Increment when the meaning or structure of the configuration changes.
pub const CONFIGURATION_SCHEMA_VERSION: u32 = 3;

/// Maximum externally supplied timeout.
///
/// This is a configuration-input safety ceiling, not a runtime resource
/// policy. Runtime execution remains governed by scheduler/resource policy.
pub const MAX_TIMEOUT_MS: u64 = 7 * 24 * 60 * 60 * 1_000;

/// Maximum externally supplied worker count.
///
/// Canonical QEC capacity remains `QecLimits::max_parallelism`.
pub const MAX_PARALLELISM: usize = 1_000_000;

/// Maximum QPU shots accepted by the configuration parser.
///
/// Actual admission is additionally constrained by `QecLimits`.
pub const MAX_QPU_SHOTS: u64 = 1_000_000_000;

/// Maximum checkpoint interval accepted by configuration.
pub const MAX_CHECKPOINT_INTERVAL_EVENTS: u64 = 10_000_000_000;

/// Result returned by configuration validation.
pub type ConfigurationResult<T> = Result<T, ConfigurationError>;

// ============================================================================
// Root configuration
// ============================================================================

/// Complete declarative QEC execution configuration.
///
/// A `QecConfig` contains policy and requirements only. Constructing or
/// validating one never allocates runtime resources.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QecConfig {
    /// Configuration schema version.
    pub schema_version: u32,

    /// Human-readable configuration identifier.
    pub name: String,

    /// Canonical QEC resource policy.
    pub limits: QecLimits,

    /// Decoder policy.
    pub decoder: DecoderConfig,

    /// Numerical policy.
    pub numerical: NumericalPolicy,

    /// Backend policy.
    pub backend: BackendConfig,

    /// Classical parallelism policy.
    pub parallelism: ParallelismConfig,

    /// Deterministic execution policy.
    pub determinism: DeterminismConfig,

    /// Checkpoint/resume policy.
    pub checkpointing: CheckpointConfig,

    /// Streaming policy.
    pub streaming: StreamingConfig,

    /// Partitioning policy.
    pub partitioning: PartitionConfig,

    /// Distributed execution policy.
    pub distributed: DistributedConfig,

    /// Scheduler policy.
    pub scheduler: SchedulerConfig,

    /// Memory policy.
    pub memory: MemoryConfig,

    /// Cache policy.
    pub cache: CacheConfig,

    /// Telemetry/observability policy.
    pub telemetry: TelemetryConfig,

    /// Security policy.
    pub security: SecurityConfig,

    /// Capability requirements.
    ///
    /// These are requirements, not authority.
    pub capabilities: CapabilityConfig,

    /// Physical QPU policy.
    pub qpu: QpuConfig,
}

impl Default for QecConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl QecConfig {
    /// Creates the conservative production configuration.
    #[must_use]
    pub fn production() -> Self {
        Self {
            schema_version: CONFIGURATION_SCHEMA_VERSION,
            name: "zamani-qec-production".to_owned(),

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

    /// Creates a deterministic configuration suitable for reproducible tests.
    #[must_use]
    pub fn deterministic_test() -> Self {
        let mut config = Self::production();

        config.name = "zamani-qec-deterministic-test".to_owned();

        config.determinism.enabled = true;
        config.determinism.seed = Some(0x5A4D_414E_4951_4543);
        config.determinism.deterministic_scheduling = true;
        config.determinism.deterministic_reductions = true;
        config.determinism.deterministic_serialization = true;

        config.capabilities.deterministic_execution = true;

        config.parallelism.enabled = false;
        config.parallelism.max_workers = 1;
        config.parallelism.deterministic_reductions = true;
        config.parallelism.work_stealing = false;

        config.streaming.deterministic_order = true;

        config
    }

    /// Validates the complete configuration.
    ///
    /// This function is pure:
    ///
    /// - no allocation;
    /// - no workers;
    /// - no QPU access;
    /// - no network access;
    /// - no resource reservation.
    pub fn validate(&self) -> ConfigurationResult<()> {
        self.validate_schema()?;
        self.validate_identity()?;

        self.limits
            .validate()
            .map_err(ConfigurationError::LimitPolicy)?;

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

    /// Returns an error if the configuration is invalid.
    pub fn validate_or_error(&self) -> ConfigurationResult<()> {
        self.validate()
    }

    /// Returns the configuration schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the selected backend.
    #[must_use]
    pub const fn backend_kind(&self) -> BackendKind {
        self.backend.kind
    }

    /// Returns whether physical QPU execution is required.
    #[must_use]
    pub const fn requires_qpu(&self) -> bool {
        self.backend.kind == BackendKind::Qpu
    }

    /// Returns whether deterministic execution is required.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.determinism.enabled
    }

    /// Returns whether remote execution is permitted.
    #[must_use]
    pub const fn permits_remote_execution(&self) -> bool {
        self.security.allow_remote_execution
    }

    /// Returns whether streaming is enabled.
    #[must_use]
    pub const fn streaming_enabled(&self) -> bool {
        self.streaming.enabled
    }

    /// Returns whether partitioning is enabled.
    #[must_use]
    pub const fn partitioning_enabled(&self) -> bool {
        self.partitioning.enabled
    }

    /// Returns whether distributed execution is enabled.
    #[must_use]
    pub const fn distributed_enabled(&self) -> bool {
        self.distributed.enabled
    }

    /// Returns whether checkpointing is enabled.
    #[must_use]
    pub const fn checkpointing_enabled(&self) -> bool {
        self.checkpointing.enabled
    }

    /// Returns a copy of the canonical resource policy after validation.
    ///
    /// This does not reserve resources.
    pub fn validated_limits(&self) -> ConfigurationResult<QecLimits> {
        self.limits
            .validate()
            .map_err(ConfigurationError::LimitPolicy)?;

        Ok(self.limits)
    }

    /// Returns whether a capability requirement is enabled.
    ///
    /// This does not authorize the caller.
    #[must_use]
    pub fn requires_capability(&self, capability: CapabilityName) -> bool {
        match capability {
            CapabilityName::Decode => self.capabilities.decode,
            CapabilityName::Simulate => self.capabilities.simulate,
            CapabilityName::Benchmark => self.capabilities.benchmark,
            CapabilityName::InspectTopology => {
                self.capabilities.inspect_topology
            }
            CapabilityName::AllocateMemory => {
                self.capabilities.allocate_memory
            }
            CapabilityName::Accelerator => self.capabilities.accelerator,
            CapabilityName::DistributedExecution => {
                self.capabilities.distributed_execution
            }
            CapabilityName::StreamingSyndrome => {
                self.capabilities.streaming_syndrome
            }
            CapabilityName::Checkpoint => self.capabilities.checkpoint,
            CapabilityName::DeterministicExecution => {
                self.capabilities.deterministic_execution
            }
            CapabilityName::ReadMetrics => self.capabilities.read_metrics,
            CapabilityName::EmitTelemetry => {
                self.capabilities.emit_telemetry
            }
            CapabilityName::ParallelExecution => {
                self.capabilities.parallel_execution
            }
            CapabilityName::QpuAccess => self.capabilities.qpu_access,
            CapabilityName::QpuInspect => self.capabilities.qpu_inspect,
            CapabilityName::QpuSubmit => self.capabilities.qpu_submit,
            CapabilityName::QpuReadResults => {
                self.capabilities.qpu_read_results
            }
            CapabilityName::QpuCalibration => {
                self.capabilities.qpu_calibration
            }
            CapabilityName::QpuErrorCorrection => {
                self.capabilities.qpu_error_correction
            }
            CapabilityName::QpuSyndromeExtraction => {
                self.capabilities.qpu_syndrome_extraction
            }
            CapabilityName::RemoteExecution => {
                self.capabilities.remote_execution
            }
        }
    }

    fn validate_schema(&self) -> ConfigurationResult<()> {
        if self.schema_version != CONFIGURATION_SCHEMA_VERSION {
            return Err(
                ConfigurationError::UnsupportedSchemaVersion {
                    expected: CONFIGURATION_SCHEMA_VERSION,
                    found: self.schema_version,
                },
            );
        }

        Ok(())
    }

    fn validate_identity(&self) -> ConfigurationResult<()> {
        let name = self.name.trim();

        if name.is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "name",
                reason: "configuration name must not be empty",
            });
        }

        if name.len() > 256 {
            return Err(ConfigurationError::InvalidValue {
                field: "name",
                reason: "configuration name exceeds 256 bytes",
            });
        }

        Ok(())
    }

    fn validate_cross_component_invariants(&self) -> ConfigurationResult<()> {
        self.validate_resource_relationships()?;
        self.validate_execution_relationships()?;
        self.validate_determinism_relationships()?;
        self.validate_streaming_relationships()?;
        self.validate_partition_relationships()?;
        self.validate_distributed_relationships()?;
        self.validate_scheduler_relationships()?;
        self.validate_checkpoint_relationships()?;
        self.validate_cache_relationships()?;
        self.validate_backend_relationships()?;
        self.validate_qpu_relationships()?;
        self.validate_telemetry_relationships()?;
        self.validate_security_relationships()?;

        Ok(())
    }

    fn validate_resource_relationships(&self) -> ConfigurationResult<()> {
        if self.memory.max_bytes > self.limits.max_memory_bytes {
            return Err(ConfigurationError::LimitMismatch {
                field: "memory.max_bytes",
                configured: self.memory.max_bytes as u128,
                maximum: self.limits.max_memory_bytes as u128,
            });
        }

        if self.memory.reserve_bytes >= self.memory.max_bytes {
            return Err(ConfigurationError::InvalidValue {
                field: "memory.reserve_bytes",
                reason: "reserve must be smaller than memory budget",
            });
        }

        if self.cache.enabled
            && self.cache.max_bytes > self.memory.max_bytes
        {
            return Err(ConfigurationError::InvalidValue {
                field: "cache.max_bytes",
                reason: "cache budget cannot exceed memory budget",
            });
        }

        if self.checkpointing.enabled
            && self.checkpointing.max_size_bytes
                > self.limits.max_checkpoint_size_bytes
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "checkpointing.max_size_bytes",
                configured: self.checkpointing.max_size_bytes as u128,
                maximum: self.limits.max_checkpoint_size_bytes as u128,
            });
        }

        if self.checkpointing.enabled
            && self.checkpointing.max_size_bytes
                > self.memory.max_bytes
        {
            return Err(ConfigurationError::InvalidValue {
                field: "checkpointing.max_size_bytes",
                reason: "checkpoint cannot exceed configured memory budget",
            });
        }

        if self.parallelism.max_workers > self.limits.max_parallelism {
            return Err(ConfigurationError::LimitMismatch {
                field: "parallelism.max_workers",
                configured: self.parallelism.max_workers as u128,
                maximum: self.limits.max_parallelism as u128,
            });
        }

        if self.decoder.max_iterations
            > self.limits.max_decoder_iterations as u64
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "decoder.max_iterations",
                configured: self.decoder.max_iterations as u128,
                maximum: self.limits.max_decoder_iterations as u128,
            });
        }

        if self.streaming.enabled
            && self.streaming.buffer_capacity_events
                > self.limits.max_stream_buffer_events as u64
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "streaming.buffer_capacity_events",
                configured: self.streaming.buffer_capacity_events as u128,
                maximum: self.limits.max_stream_buffer_events as u128,
            });
        }

        if self.partitioning.enabled
            && self.partitioning.partitions as usize
                > self.limits.max_partitions
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "partitioning.partitions",
                configured: self.partitioning.partitions as u128,
                maximum: self.limits.max_partitions as u128,
            });
        }

        if self.partitioning.enabled
            && self.partitioning.max_events_per_partition
                > self.limits.max_syndrome_events as u64
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "partitioning.max_events_per_partition",
                configured: self.partitioning.max_events_per_partition
                    as u128,
                maximum: self.limits.max_syndrome_events as u128,
            });
        }

        if self.qpu.enabled
            && self.qpu.shots > self.limits.max_qpu_shots
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "qpu.shots",
                configured: self.qpu.shots as u128,
                maximum: self.limits.max_qpu_shots as u128,
            });
        }

        if self.qpu.enabled
            && self.qpu.max_circuits > self.limits.max_qpu_circuits
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "qpu.max_circuits",
                configured: self.qpu.max_circuits as u128,
                maximum: self.limits.max_qpu_circuits as u128,
            });
        }

        Ok(())
    }

    fn validate_execution_relationships(&self) -> ConfigurationResult<()> {
        if self.backend.maximum_in_flight_operations == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.maximum_in_flight_operations",
                reason: "must be greater than zero",
            });
        }

        if self.backend.maximum_in_flight_operations as usize
            > self.parallelism.max_workers
        {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.maximum_in_flight_operations",
                reason:
                    "in-flight backend operations cannot exceed configured workers",
            });
        }

        if self.scheduler.max_running_jobs as usize
            > self.parallelism.max_workers
        {
            return Err(ConfigurationError::InvalidValue {
                field: "scheduler.max_running_jobs",
                reason:
                    "running jobs cannot exceed configured parallelism",
            });
        }

        if self.distributed.enabled
            && self.distributed.worker_count as usize
                > self.parallelism.max_workers
        {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.worker_count",
                reason:
                    "distributed workers cannot exceed configured parallelism",
            });
        }

        Ok(())
    }

    fn validate_determinism_relationships(&self) -> ConfigurationResult<()> {
        if !self.determinism.enabled {
            return Ok(());
        }

        if !self.determinism.deterministic_scheduling {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism.deterministic_scheduling",
                reason:
                    "deterministic mode requires deterministic scheduling",
            });
        }

        if !self.determinism.deterministic_reductions {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism.deterministic_reductions",
                reason:
                    "deterministic mode requires deterministic reductions",
            });
        }

        if !self.determinism.deterministic_serialization {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism.deterministic_serialization",
                reason:
                    "deterministic mode requires deterministic serialization",
            });
        }

        if !self.capabilities.deterministic_execution {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.deterministic_execution",
            });
        }

        if !self.parallelism.deterministic_reductions {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.deterministic_reductions",
                reason:
                    "deterministic mode requires deterministic parallel reductions",
            });
        }

        if self.parallelism.work_stealing {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.work_stealing",
                reason:
                    "deterministic mode cannot use unrestricted work stealing",
            });
        }

        if self.streaming.enabled && !self.streaming.deterministic_order {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.deterministic_order",
                reason:
                    "deterministic mode requires deterministic stream ordering",
            });
        }

        Ok(())
    }

    fn validate_streaming_relationships(&self) -> ConfigurationResult<()> {
        if !self.streaming.enabled {
            return Ok(());
        }

        if !self.capabilities.streaming_syndrome {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.streaming_syndrome",
            });
        }

        if self.streaming.batch_size
            > self.streaming.buffer_capacity_events
        {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.batch_size",
                reason: "batch size cannot exceed stream buffer capacity",
            });
        }

        if self.streaming.drop_on_overflow
            && self.streaming.backpressure == BackpressurePolicy::Block
        {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.drop_on_overflow",
                reason:
                    "events cannot be dropped when backpressure is Block",
            });
        }

        if self.streaming.backpressure == BackpressurePolicy::SpillToCheckpoint
            && !self.checkpointing.enabled
        {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.backpressure",
                reason:
                    "SpillToCheckpoint requires checkpointing",
            });
        }

        Ok(())
    }

    fn validate_partition_relationships(&self) -> ConfigurationResult<()> {
        if !self.partitioning.enabled {
            if self.partitioning.partitions != 1 {
                return Err(ConfigurationError::InvalidValue {
                    field: "partitioning.partitions",
                    reason:
                        "disabled partitioning must specify exactly one partition",
                });
            }

            return Ok(());
        }

        if !self.partitioning.preserve_boundaries {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "partitioned QEC must preserve mathematical boundaries",
            ));
        }

        if self.partitioning.overlap_events
            >= self.partitioning.max_events_per_partition
        {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.overlap_events",
                reason:
                    "partition overlap must be smaller than partition capacity",
            });
        }

        Ok(())
    }

    fn validate_distributed_relationships(&self) -> ConfigurationResult<()> {
        if !self.distributed.enabled {
            return Ok(());
        }

        if !self.partitioning.enabled {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.enabled",
                reason:
                    "distributed execution requires partitioning",
            });
        }

        if self.backend.kind != BackendKind::Distributed {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.kind",
                reason:
                    "distributed execution requires Distributed backend",
            });
        }

        if !self.capabilities.distributed_execution {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.distributed_execution",
            });
        }

        if !self.distributed.require_authenticated_workers {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "distributed workers must be authenticated",
            ));
        }

        if !self.distributed.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "distributed transport must be encrypted",
            ));
        }

        if self.distributed.max_in_flight_partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.max_in_flight_partitions",
                reason: "must be greater than zero",
            });
        }

        if self.distributed.max_in_flight_partitions as usize
            > self.limits.max_partitions
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "distributed.max_in_flight_partitions",
                configured: self.distributed.max_in_flight_partitions
                    as u128,
                maximum: self.limits.max_partitions as u128,
            });
        }

        Ok(())
    }

    fn validate_scheduler_relationships(&self) -> ConfigurationResult<()> {
        if !self.scheduler.enabled {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "QEC scheduler must remain enabled",
            ));
        }

        if !self.scheduler.enable_cancellation {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "QEC scheduler must support cancellation",
            ));
        }

        if !self.scheduler.enable_backpressure {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "QEC scheduler must support backpressure",
            ));
        }

        if self.scheduler.max_running_jobs as usize
            > self.limits.max_parallelism
        {
            return Err(ConfigurationError::LimitMismatch {
                field: "scheduler.max_running_jobs",
                configured: self.scheduler.max_running_jobs as u128,
                maximum: self.limits.max_parallelism as u128,
            });
        }

        Ok(())
    }

    fn validate_checkpoint_relationships(&self) -> ConfigurationResult<()> {
        if !self.checkpointing.enabled {
            return Ok(());
        }

        if !self.capabilities.checkpoint {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.checkpoint",
            });
        }

        if !self.security.verify_checkpoints {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "checkpoint integrity verification cannot be disabled",
            ));
        }

        if !self.checkpointing.require_compatible_schema {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "checkpoint schema compatibility must be enforced",
            ));
        }

        if !self.checkpointing.allow_resume {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "checkpointing must retain resume capability",
            ));
        }

        Ok(())
    }

    fn validate_cache_relationships(&self) -> ConfigurationResult<()> {
        if !self.cache.enabled {
            return Ok(());
        }

        if !self.cache.verify_integrity {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "cache integrity verification cannot be disabled",
            ));
        }

        if !self.cache.recompute_on_corruption {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "corrupt cache entries must be discarded and recomputed",
            ));
        }

        Ok(())
    }

    fn validate_backend_relationships(&self) -> ConfigurationResult<()> {
        match self.backend.kind {
            BackendKind::Cpu
            | BackendKind::Simulator
            | BackendKind::Emulator
            | BackendKind::Custom => {
                if self.backend.require_hardware {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.require_hardware",
                        reason:
                            "selected backend is not a physical hardware backend",
                    });
                }
            }

            BackendKind::ParallelCpu => {
                if !self.capabilities.parallel_execution {
                    return Err(ConfigurationError::CapabilityRequired {
                        capability: "qec.parallel_execution",
                    });
                }
            }

            BackendKind::Gpu | BackendKind::Accelerator => {
                if !self.capabilities.accelerator {
                    return Err(ConfigurationError::CapabilityRequired {
                        capability: "qec.use_accelerator",
                    });
                }
            }

            BackendKind::Distributed => {
                if !self.distributed.enabled {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.kind",
                        reason:
                            "Distributed backend requires distributed execution",
                    });
                }
            }

            BackendKind::Qpu => {
                if !self.qpu.enabled {
                    return Err(ConfigurationError::BackendRequiresQpu);
                }

                if !self.backend.require_hardware {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.require_hardware",
                        reason:
                            "physical QPU backend must require hardware",
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_qpu_relationships(&self) -> ConfigurationResult<()> {
        if !self.qpu.enabled {
            if self.backend.kind == BackendKind::Qpu {
                return Err(ConfigurationError::BackendRequiresQpu);
            }

            return Ok(());
        }

        if self.backend.kind != BackendKind::Qpu {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.enabled",
                reason:
                    "QPU configuration requires BackendKind::Qpu",
            });
        }

        let backend_device = self.backend.device_id.as_deref().unwrap_or("").trim();
        let qpu_device = self.qpu.device_id.as_deref().unwrap_or("").trim();

        if backend_device.is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.device_id",
                reason: "QPU backend requires a device ID",
            });
        }

        if qpu_device.is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.device_id",
                reason: "QPU configuration requires a device ID",
            });
        }

        if backend_device != qpu_device {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.device_id",
                reason:
                    "QPU device ID must match backend device ID",
            });
        }

        if !self.capabilities.qpu_access {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_access",
            });
        }

        if !self.capabilities.qpu_inspect {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_inspect",
            });
        }

        if !self.capabilities.qpu_submit {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_submit",
            });
        }

        if !self.capabilities.qpu_read_results {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_read_results",
            });
        }

        if !self.capabilities.qpu_syndrome_extraction {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_syndrome_extraction",
            });
        }

        if !self.capabilities.qpu_error_correction {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_error_correction",
            });
        }

        if self.qpu.verify_calibration
            && !self.capabilities.qpu_calibration
        {
            return Err(ConfigurationError::CapabilityRequired {
                capability: "qec.qpu_calibration",
            });
        }

        if self.qpu.allow_remote {
            if !self.security.allow_remote_execution {
                return Err(ConfigurationError::SecurityPolicyViolation(
                    "remote QPU execution is disabled by security policy",
                ));
            }

            if !self.capabilities.remote_execution {
                return Err(ConfigurationError::CapabilityRequired {
                    capability: "qec.remote_execution",
                });
            }

            if !self.qpu.require_authenticated_device {
                return Err(ConfigurationError::SecurityPolicyViolation(
                    "remote QPU requires authenticated device",
                ));
            }

            if !self.qpu.require_encrypted_transport {
                return Err(ConfigurationError::SecurityPolicyViolation(
                    "remote QPU requires encrypted transport",
                ));
            }

            if self.qpu.adapter == QpuAdapter::LocalHardware {
                return Err(ConfigurationError::InvalidValue {
                    field: "qpu.adapter",
                    reason:
                        "LocalHardware cannot be used for remote QPU execution",
                });
            }
        } else if self.qpu.adapter == QpuAdapter::RemoteHardware {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.adapter",
                reason:
                    "RemoteHardware requires qpu.allow_remote",
            });
        }

        if self.qpu.allow_simulator_fallback {
            if self.qpu.execution_policy
                == QpuExecutionPolicy::RequireHardware
            {
                return Err(ConfigurationError::InvalidValue {
                    field: "qpu.allow_simulator_fallback",
                    reason:
                        "simulator fallback conflicts with RequireHardware",
                });
            }

            if !self.backend.allow_software_fallback {
                return Err(ConfigurationError::InvalidValue {
                    field: "backend.allow_software_fallback",
                    reason:
                        "QPU simulator fallback requires backend software fallback",
                });
            }
        }

        if self.qpu.execution_policy
            == QpuExecutionPolicy::AllowSimulatorFallback
            && !self.qpu.allow_simulator_fallback
        {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.execution_policy",
                reason:
                    "AllowSimulatorFallback requires allow_simulator_fallback",
            });
        }

        Ok(())
    }

    fn validate_telemetry_relationships(&self) -> ConfigurationResult<()> {
        if self.telemetry.export_remote {
            if !self.security.allow_remote_execution {
                return Err(ConfigurationError::SecurityPolicyViolation(
                    "remote telemetry requires explicit remote execution policy",
                ));
            }

            if !self.security.protect_sensitive_metadata {
                return Err(ConfigurationError::SecurityPolicyViolation(
                    "remote telemetry requires sensitive metadata protection",
                ));
            }
        }

        if self.telemetry.include_qpu_statistics && self.qpu.enabled {
            if !self.capabilities.read_metrics {
                return Err(ConfigurationError::CapabilityRequired {
                    capability: "qec.read_metrics",
                });
            }
        }

        Ok(())
    }

    fn validate_security_relationships(&self) -> ConfigurationResult<()> {
        if !self.security.validate_external_input {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "external configuration validation cannot be disabled",
            ));
        }

        if !self.security.reject_malformed_input {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "malformed input must be rejected",
            ));
        }

        if !self.security.reject_resource_bombs {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "resource-bomb protection cannot be disabled",
            ));
        }

        if !self.security.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "encrypted transport is required for production QEC",
            ));
        }

        if !self.security.protect_sensitive_metadata {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "sensitive metadata protection cannot be disabled",
            ));
        }

        if !self.security.audit_capability_usage {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "capability auditing cannot be disabled",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Capability names
// ============================================================================

/// Stable configuration-level capability names.
///
/// These map directly to the capability identifiers owned by
/// `capabilities.rs` without importing that module and creating a cyclic
/// dependency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityName {
    Decode,
    Simulate,
    Benchmark,
    InspectTopology,
    AllocateMemory,
    Accelerator,
    DistributedExecution,
    StreamingSyndrome,
    Checkpoint,
    DeterministicExecution,
    ReadMetrics,
    EmitTelemetry,
    ParallelExecution,
    QpuAccess,
    QpuInspect,
    QpuSubmit,
    QpuReadResults,
    QpuCalibration,
    QpuErrorCorrection,
    QpuSyndromeExtraction,
    RemoteExecution,
}

// ============================================================================
// Decoder
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecoderConfig {
    /// Primary decoder.
    pub strategy: DecoderStrategy,

    /// Maximum decoder iterations.
    pub max_iterations: u64,

    /// Permit confidence/weight information.
    pub use_soft_information: bool,

    /// Permit post-selection where supported.
    pub enable_post_selection: bool,

    /// Permit a secondary decoder if the primary cannot complete.
    pub allow_fallback: bool,

    /// Secondary decoder.
    pub fallback_strategy: Option<DecoderStrategy>,

    /// Logical-failure handling policy.
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
                reason: "must be greater than zero",
            });
        }

        if self.allow_fallback && self.fallback_strategy.is_none() {
            return Err(ConfigurationError::InvalidValue {
                field: "decoder.fallback_strategy",
                reason:
                    "fallback strategy is required when fallback is enabled",
            });
        }

        if let Some(fallback) = self.fallback_strategy {
            if fallback == self.strategy {
                return Err(ConfigurationError::InvalidValue {
                    field: "decoder.fallback_strategy",
                    reason:
                        "fallback strategy must differ from primary strategy",
                });
            }
        }

        if !self.allow_fallback && self.fallback_strategy.is_some() {
            return Err(ConfigurationError::InvalidValue {
                field: "decoder.fallback_strategy",
                reason:
                    "fallback strategy must be absent when fallback is disabled",
            });
        }

        Ok(())
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogicalFailurePolicy {
    Report,
    Retry,
    Fallback,
    Abort,
}

// ============================================================================
// Numerical policy
// ============================================================================

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
                reason: "must be less than one",
            });
        }

        if self.weight_epsilon >= 1.0 {
            return Err(ConfigurationError::InvalidValue {
                field: "numerical.weight_epsilon",
                reason: "must be less than one",
            });
        }

        if self.reject_nan
            && self.floating_point_mode == FloatingPointMode::Fast
        {
            return Err(ConfigurationError::InvalidValue {
                field: "numerical.floating_point_mode",
                reason:
                    "Fast floating-point mode cannot guarantee NaN rejection",
            });
        }

        if self.reject_infinity
            && self.floating_point_mode == FloatingPointMode::Fast
        {
            return Err(ConfigurationError::InvalidValue {
                field: "numerical.floating_point_mode",
                reason:
                    "Fast floating-point mode cannot guarantee infinity rejection",
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

// ============================================================================
// Backend configuration
// ============================================================================

/// Backend configuration.
///
/// `BackendKind` is serialized through its stable textual representation here
/// because `backend.rs` owns the type and currently does not need to depend on
/// Serde merely for configuration.
#[derive(Clone, Debug, PartialEq)]
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

        if self.require_hardware && self.allow_software_fallback {
            return Err(ConfigurationError::InvalidValue {
                field: "backend.allow_software_fallback",
                reason:
                    "software fallback conflicts with require_hardware",
            });
        }

        match self.kind {
            BackendKind::Qpu => {
                let device_id =
                    self.device_id.as_deref().unwrap_or("").trim();

                if device_id.is_empty() {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.device_id",
                        reason:
                            "QPU backend requires an explicit device ID",
                    });
                }

                if !self.require_hardware {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.require_hardware",
                        reason:
                            "physical QPU backend must require hardware",
                    });
                }
            }

            BackendKind::Cpu
            | BackendKind::Simulator
            | BackendKind::Emulator
            | BackendKind::Custom => {
                if self.require_hardware {
                    return Err(ConfigurationError::InvalidValue {
                        field: "backend.require_hardware",
                        reason:
                            "software backend cannot require physical hardware",
                    });
                }
            }

            BackendKind::ParallelCpu
            | BackendKind::Gpu
            | BackendKind::Accelerator
            | BackendKind::Distributed => {}
        }

        Ok(())
    }

    #[must_use]
    pub const fn requires_qpu(&self) -> bool {
        self.kind == BackendKind::Qpu
    }

    #[must_use]
    pub const fn requires_accelerator(&self) -> bool {
        matches!(
            self.kind,
            BackendKind::Gpu | BackendKind::Accelerator
        )
    }

    #[must_use]
    pub const fn requires_distributed(&self) -> bool {
        self.kind == BackendKind::Distributed
    }

    #[must_use]
    pub const fn is_software(&self) -> bool {
        !matches!(self.kind, BackendKind::Qpu)
    }
}

impl Serialize for BackendConfig {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wire<'a> {
            kind: &'a str,
            device_id: &'a Option<String>,
            require_hardware: bool,
            allow_software_fallback: bool,
            maximum_in_flight_operations: u32,
        }

        let kind = self.kind.as_str();

        Wire {
            kind,
            device_id: &self.device_id,
            require_hardware: self.require_hardware,
            allow_software_fallback: self.allow_software_fallback,
            maximum_in_flight_operations:
                self.maximum_in_flight_operations,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BackendConfig {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: String,
            device_id: Option<String>,
            require_hardware: bool,
            allow_software_fallback: bool,
            maximum_in_flight_operations: u32,
        }

        let wire = Wire::deserialize(deserializer)?;

        let kind = parse_backend_kind(&wire.kind)
            .map_err(de::Error::custom)?;

        let config = Self {
            kind,
            device_id: wire.device_id,
            require_hardware: wire.require_hardware,
            allow_software_fallback: wire.allow_software_fallback,
            maximum_in_flight_operations:
                wire.maximum_in_flight_operations,
        };

        config.validate().map_err(de::Error::custom)?;

        Ok(config)
    }
}

fn parse_backend_kind(value: &str) -> Result<BackendKind, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "cpu" => Ok(BackendKind::Cpu),
        "parallel_cpu" => Ok(BackendKind::ParallelCpu),
        "gpu" => Ok(BackendKind::Gpu),
        "accelerator" => Ok(BackendKind::Accelerator),
        "distributed" => Ok(BackendKind::Distributed),
        "simulator" => Ok(BackendKind::Simulator),
        "emulator" => Ok(BackendKind::Emulator),
        "qpu" => Ok(BackendKind::Qpu),
        "custom" => Ok(BackendKind::Custom),
        other => Err(format!("unsupported backend kind `{other}`")),
    }
}

// ============================================================================
// Parallelism
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParallelismConfig {
    pub enabled: bool,
    pub max_workers: usize,
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
                reason:
                    "exceeds configuration input safety ceiling",
            });
        }

        if self.chunk_size == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.chunk_size",
                reason: "must be greater than zero",
            });
        }

        if !self.enabled && self.max_workers != 1 {
            return Err(ConfigurationError::InvalidValue {
                field: "parallelism.max_workers",
                reason:
                    "disabled parallelism must use one worker",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Determinism
// ============================================================================

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
        if self.enabled
            && (!self.deterministic_scheduling
                || !self.deterministic_reductions
                || !self.deterministic_serialization)
        {
            return Err(ConfigurationError::InvalidValue {
                field: "determinism",
                reason:
                    "enabled deterministic mode requires all deterministic controls",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Checkpointing
// ============================================================================

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
        if !self.enabled {
            return Ok(());
        }

        if self.interval_events == 0
            || self.interval_events > MAX_CHECKPOINT_INTERVAL_EVENTS
        {
            return Err(ConfigurationError::InvalidValue {
                field: "checkpointing.interval_events",
                reason:
                    "checkpoint interval is outside supported configuration bounds",
            });
        }

        if self.max_size_bytes == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "checkpointing.max_size_bytes",
                reason: "must be greater than zero",
            });
        }

        if self.retain_count == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "checkpointing.retain_count",
                reason: "must be greater than zero",
            });
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

// ============================================================================
// Streaming
// ============================================================================

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
        if !self.enabled {
            return Ok(());
        }

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

        if self.batch_size > self.buffer_capacity_events {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.batch_size",
                reason:
                    "batch size cannot exceed buffer capacity",
            });
        }

        if self.drop_on_overflow
            && self.backpressure == BackpressurePolicy::Block
        {
            return Err(ConfigurationError::InvalidValue {
                field: "streaming.drop_on_overflow",
                reason:
                    "cannot drop events with blocking backpressure",
            });
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

// ============================================================================
// Partitioning
// ============================================================================

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
            boundary_reconciliation:
                BoundaryReconciliation::GlobalMatching,
        }
    }
}

impl PartitionConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.partitions",
                reason: "must be greater than zero",
            });
        }

        if self.max_events_per_partition == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.max_events_per_partition",
                reason: "must be greater than zero",
            });
        }

        if self.overlap_events
            >= self.max_events_per_partition
        {
            return Err(ConfigurationError::InvalidValue {
                field: "partitioning.overlap_events",
                reason:
                    "overlap must be smaller than partition capacity",
            });
        }

        if self.enabled && !self.preserve_boundaries {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "partitioned QEC must preserve mathematical boundaries",
            ));
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

// ============================================================================
// Distributed execution
// ============================================================================

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
        if !self.enabled {
            return Ok(());
        }

        if self.worker_count == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.worker_count",
                reason: "must be greater than zero",
            });
        }

        if self.max_in_flight_partitions == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "distributed.max_in_flight_partitions",
                reason: "must be greater than zero",
            });
        }

        if !self.require_authenticated_workers {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "distributed workers must be authenticated",
            ));
        }

        if !self.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "distributed transport must be encrypted",
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

// ============================================================================
// Scheduler
// ============================================================================

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
                "QEC scheduling must support cancellation",
            ));
        }

        if !self.enable_backpressure {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "QEC scheduling must support backpressure",
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

// ============================================================================
// Memory
// ============================================================================

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
                reason:
                    "reserve must be smaller than memory budget",
            });
        }

        if !self.bounded_buffers {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "production QEC requires bounded buffers",
            ));
        }

        if !self.fail_on_budget_exhaustion {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "memory exhaustion must fail safely",
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

// ============================================================================
// Cache
// ============================================================================

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
        if !self.enabled {
            return Ok(());
        }

        if self.max_bytes == 0 {
            return Err(ConfigurationError::InvalidValue {
                field: "cache.max_bytes",
                reason: "must be greater than zero",
            });
        }

        if !self.verify_integrity {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "cache integrity verification cannot be disabled",
            ));
        }

        if !self.recompute_on_corruption {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "corrupt cache entries must be discarded and recomputed",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Telemetry
// ============================================================================

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
            || !(0.0..=1.0).contains(&self.sampling_rate)
        {
            return Err(ConfigurationError::InvalidValue {
                field: "telemetry.sampling_rate",
                reason:
                    "must be finite and between zero and one",
            });
        }

        if self.export_remote && !self.enabled {
            return Err(ConfigurationError::InvalidValue {
                field: "telemetry.export_remote",
                reason:
                    "remote telemetry requires telemetry to be enabled",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Security
// ============================================================================

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
                "external configuration validation cannot be disabled",
            ));
        }

        if !self.reject_malformed_input {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "malformed input must be rejected",
            ));
        }

        if !self.reject_resource_bombs {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "resource-bomb protection cannot be disabled",
            ));
        }

        if !self.require_encrypted_transport {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "encrypted transport is required for production QEC",
            ));
        }

        if !self.protect_sensitive_metadata {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "sensitive metadata protection cannot be disabled",
            ));
        }

        if !self.audit_capability_usage {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "capability auditing cannot be disabled",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Capability requirements
// ============================================================================

/// Declarative capability requirements.
///
/// These values never grant authority. Runtime authorization remains owned by
/// `capabilities.rs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityConfig {
    pub decode: bool,
    pub simulate: bool,
    pub benchmark: bool,
    pub inspect_topology: bool,
    pub allocate_memory: bool,
    pub accelerator: bool,
    pub distributed_execution: bool,
    pub streaming_syndrome: bool,
    pub checkpoint: bool,
    pub deterministic_execution: bool,
    pub read_metrics: bool,
    pub emit_telemetry: bool,
    pub parallel_execution: bool,
    pub qpu_access: bool,
    pub qpu_inspect: bool,
    pub qpu_submit: bool,
    pub qpu_read_results: bool,
    pub qpu_calibration: bool,
    pub qpu_error_correction: bool,
    pub qpu_syndrome_extraction: bool,
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
            streaming_syndrome: true,
            checkpoint: true,
            deterministic_execution: false,
            read_metrics: true,
            emit_telemetry: true,
            parallel_execution: true,

            qpu_access: false,
            qpu_inspect: false,
            qpu_submit: false,
            qpu_read_results: false,
            qpu_calibration: false,
            qpu_error_correction: false,
            qpu_syndrome_extraction: false,

            remote_execution: false,
        }
    }
}

impl CapabilityConfig {
    pub fn validate(&self) -> ConfigurationResult<()> {
        if self.qpu_submit && !self.qpu_access {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_submit",
                requires: "qec.qpu_access",
            });
        }

        if self.qpu_read_results && !self.qpu_access {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_read_results",
                requires: "qec.qpu_access",
            });
        }

        if self.qpu_inspect && !self.qpu_access {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_inspect",
                requires: "qec.qpu_access",
            });
        }

        if self.qpu_calibration && !self.qpu_inspect {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_calibration",
                requires: "qec.qpu_inspect",
            });
        }

        if self.qpu_error_correction
            && !self.qpu_syndrome_extraction
        {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_error_correction",
                requires: "qec.qpu_syndrome_extraction",
            });
        }

        if self.qpu_error_correction && !self.qpu_access {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_error_correction",
                requires: "qec.qpu_access",
            });
        }

        if self.qpu_syndrome_extraction && !self.qpu_access {
            return Err(ConfigurationError::CapabilityDependency {
                capability: "qec.qpu_syndrome_extraction",
                requires: "qec.qpu_access",
            });
        }

        Ok(())
    }
}

// ============================================================================
// QPU
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QpuConfig {
    pub enabled: bool,
    pub adapter: QpuAdapter,
    pub device_id: Option<String>,
    pub shots: u64,
    pub max_circuits: u64,
    pub verify_calibration: bool,
    pub required_calibration_epoch: Option<String>,
    pub reset_before_run: bool,
    pub readout_mode: QpuReadoutMode,
    pub measurement_timeout_ms: u64,
    pub queue_timeout_ms: u64,
    pub execution_policy: QpuExecutionPolicy,
    pub connectivity: QpuConnectivity,
    pub error_model: QpuErrorModel,
    pub allow_remote: bool,
    pub require_authenticated_device: bool,
    pub require_encrypted_transport: bool,
    pub allow_simulator_fallback: bool,
    pub verify_results: bool,
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

            execution_policy:
                QpuExecutionPolicy::RejectIfUnavailable,

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

        let device_id =
            self.device_id.as_deref().unwrap_or("").trim();

        if device_id.is_empty() {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.device_id",
                reason:
                    "enabled QPU execution requires a device ID",
            });
        }

        if self.shots == 0 || self.shots > MAX_QPU_SHOTS {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.shots",
                reason:
                    "QPU shot count is outside configuration bounds",
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

        if self.max_in_flight_jobs as u64 > self.max_circuits {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.max_in_flight_jobs",
                reason:
                    "in-flight jobs cannot exceed maximum circuit count",
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

        if self.allow_remote
            && !self.require_authenticated_device
        {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "remote QPU requires authenticated device",
            ));
        }

        if self.allow_remote
            && !self.require_encrypted_transport
        {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "remote QPU requires encrypted transport",
            ));
        }

        if self.allow_simulator_fallback
            && self.execution_policy
                == QpuExecutionPolicy::RequireHardware
        {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.allow_simulator_fallback",
                reason:
                    "simulator fallback conflicts with RequireHardware",
            });
        }

        if self.execution_policy
            == QpuExecutionPolicy::AllowSimulatorFallback
            && !self.allow_simulator_fallback
        {
            return Err(ConfigurationError::InvalidValue {
                field: "qpu.execution_policy",
                reason:
                    "AllowSimulatorFallback requires simulator fallback",
            });
        }

        if !self.verify_results {
            return Err(ConfigurationError::SecurityPolicyViolation(
                "QPU result verification cannot be disabled",
            ));
        }

        Ok(())
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuExecutionPolicy {
    RequireHardware,
    RejectIfUnavailable,
    AllowSimulatorFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuReadoutMode {
    Counts,
    Samples,
    Probabilities,
    RawBitstrings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuConnectivity {
    HardwareDefined,
    AllToAll,
    Line,
    Grid,
    HeavyHex,
    Custom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum QpuErrorModel {
    HardwareReported,
    Configured,
    CalibrationDerived,
    None,
}

// ============================================================================
// Errors
// ============================================================================

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

    CapabilityRequired {
        capability: &'static str,
    },

    CapabilityDependency {
        capability: &'static str,
        requires: &'static str,
    },

    LimitMismatch {
        field: &'static str,
        configured: u128,
        maximum: u128,
    },

    SecurityPolicyViolation(&'static str),

    LimitPolicy(super::limits::LimitError),
}

impl fmt::Display for ConfigurationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => {
                write!(
                    formatter,
                    "invalid configuration field `{field}`: {reason}"
                )
            }

            Self::UnsupportedSchemaVersion {
                expected,
                found,
            } => {
                write!(
                    formatter,
                    "unsupported configuration schema version: \
                     expected {expected}, found {found}"
                )
            }

            Self::BackendRequiresQpu => {
                write!(
                    formatter,
                    "selected backend requires QPU configuration"
                )
            }

            Self::CapabilityRequired { capability } => {
                write!(
                    formatter,
                    "required capability is not configured: {capability}"
                )
            }

            Self::CapabilityDependency {
                capability,
                requires,
            } => {
                write!(
                    formatter,
                    "capability `{capability}` requires `{requires}`"
                )
            }

            Self::LimitMismatch {
                field,
                configured,
                maximum,
            } => {
                write!(
                    formatter,
                    "configuration field `{field}` exceeds \
                     canonical QEC limit: configured {configured}, \
                     maximum {maximum}"
                )
            }

            Self::SecurityPolicyViolation(reason) => {
                write!(
                    formatter,
                    "security policy violation: {reason}"
                )
            }

            Self::LimitPolicy(error) => {
                write!(
                    formatter,
                    "invalid QEC resource policy: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigurationError {}

// ============================================================================
// Helpers
// ============================================================================

fn validate_finite_nonnegative(
    field: &'static str,
    value: f64,
) -> ConfigurationResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(ConfigurationError::InvalidValue {
            field,
            reason:
                "value must be finite and non-negative",
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
            reason:
                "timeout exceeds configuration input safety ceiling",
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_qpu_config() -> QecConfig {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Qpu;
        config.backend.device_id = Some("qpu-0".to_owned());
        config.backend.require_hardware = true;
        config.backend.allow_software_fallback = false;

        config.capabilities.qpu_access = true;
        config.capabilities.qpu_inspect = true;
        config.capabilities.qpu_submit = true;
        config.capabilities.qpu_read_results = true;
        config.capabilities.qpu_calibration = true;
        config.capabilities.qpu_syndrome_extraction = true;
        config.capabilities.qpu_error_correction = true;

        config.qpu.enabled = true;
        config.qpu.device_id = Some("qpu-0".to_owned());

        config
    }

    #[test]
    fn production_configuration_is_valid() {
        assert!(QecConfig::production().validate().is_ok());
    }

    #[test]
    fn deterministic_configuration_is_valid() {
        assert!(QecConfig::deterministic_test().validate().is_ok());
    }

    #[test]
    fn invalid_schema_is_rejected() {
        let mut config = QecConfig::production();
        config.schema_version += 1;

        assert!(matches!(
            config.validate(),
            Err(
                ConfigurationError::UnsupportedSchemaVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_resource_policy_is_rejected() {
        let mut config = QecConfig::production();

        config.limits.max_memory_bytes = 0;

        assert!(matches!(
            config.validate(),
            Err(ConfigurationError::LimitPolicy(_))
        ));
    }

    #[test]
    fn memory_cannot_exceed_canonical_limit() {
        let mut config = QecConfig::production();

        config.memory.max_bytes =
            config.limits.max_memory_bytes.saturating_add(1);

        assert!(config.validate().is_err());
    }

    #[test]
    fn cache_cannot_exceed_memory_budget() {
        let mut config = QecConfig::production();

        config.cache.max_bytes =
            config.memory.max_bytes.saturating_add(1);

        assert!(config.validate().is_err());
    }

    #[test]
    fn decoder_cannot_exceed_canonical_iteration_limit() {
        let mut config = QecConfig::production();

        config.decoder.max_iterations =
            config.limits.max_decoder_iterations as u64 + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn streaming_batch_cannot_exceed_buffer() {
        let mut config = QecConfig::production();

        config.streaming.batch_size =
            config.streaming.buffer_capacity_events + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn deterministic_mode_requires_capability() {
        let mut config = QecConfig::production();

        config.determinism.enabled = true;
        config.capabilities.deterministic_execution = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn deterministic_mode_rejects_work_stealing() {
        let mut config = QecConfig::deterministic_test();

        config.parallelism.work_stealing = true;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_requires_partitioning() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Distributed;
        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_requires_distributed_backend() {
        let mut config = QecConfig::production();

        config.partitioning.enabled = true;
        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_requires_authentication() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Distributed;
        config.partitioning.enabled = true;
        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;
        config.distributed.require_authenticated_workers = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn distributed_requires_encryption() {
        let mut config = QecConfig::production();

        config.backend.kind = BackendKind::Distributed;
        config.partitioning.enabled = true;
        config.distributed.enabled = true;
        config.capabilities.distributed_execution = true;
        config.distributed.require_encrypted_transport = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_requires_qpu_backend() {
        let mut config = QecConfig::production();

        config.qpu.enabled = true;
        config.qpu.device_id = Some("qpu-0".to_owned());

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_requires_complete_capability_chain() {
        let mut config = valid_qpu_config();

        config.capabilities.qpu_submit = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_device_ids_must_match() {
        let mut config = valid_qpu_config();

        config.qpu.device_id = Some("different-device".to_owned());

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_configuration_can_be_authorized() {
        assert!(valid_qpu_config().validate().is_ok());
    }

    #[test]
    fn qpu_shot_limit_is_enforced() {
        let mut config = valid_qpu_config();

        config.qpu.shots =
            config.limits.max_qpu_shots.saturating_add(1);

        assert!(config.validate().is_err());
    }

    #[test]
    fn remote_qpu_requires_remote_policy() {
        let mut config = valid_qpu_config();

        config.qpu.allow_remote = true;
        config.qpu.adapter = QpuAdapter::RemoteHardware;

        assert!(config.validate().is_err());
    }

    #[test]
    fn remote_qpu_requires_remote_capability() {
        let mut config = valid_qpu_config();

        config.qpu.allow_remote = true;
        config.qpu.adapter = QpuAdapter::RemoteHardware;
        config.security.allow_remote_execution = true;
        config.capabilities.remote_execution = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn cache_integrity_cannot_be_disabled() {
        let mut config = QecConfig::production();

        config.cache.verify_integrity = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn corrupt_cache_must_be_recomputable() {
        let mut config = QecConfig::production();

        config.cache.recompute_on_corruption = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn checkpoint_size_cannot_exceed_limit() {
        let mut config = QecConfig::production();

        config.checkpointing.max_size_bytes =
            config.limits.max_checkpoint_size_bytes + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn checkpoint_spill_requires_checkpointing() {
        let mut config = QecConfig::production();

        config.checkpointing.enabled = false;
        config.streaming.backpressure =
            BackpressurePolicy::SpillToCheckpoint;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_result_verification_cannot_be_disabled() {
        let mut config = valid_qpu_config();

        config.qpu.verify_results = false;

        assert!(config.validate().is_err());
    }

    #[test]
    fn qpu_simulator_fallback_conflict_is_rejected() {
        let mut config = valid_qpu_config();

        config.qpu.allow_simulator_fallback = true;
        config.qpu.execution_policy =
            QpuExecutionPolicy::RequireHardware;

        assert!(config.validate().is_err());
    }

    #[test]
    fn backend_configuration_round_trip_preserves_kind() {
        let config = valid_qpu_config();

        let encoded =
            serde_json::to_string(&config.backend)
                .expect("backend should serialize");

        let decoded: BackendConfig =
            serde_json::from_str(&encoded)
                .expect("backend should deserialize");

        assert_eq!(config.backend, decoded);
    }

    #[test]
    fn json_round_trip_preserves_configuration() {
        let config = QecConfig::production();

        let encoded =
            serde_json::to_string(&config)
                .expect("configuration should serialize");

        let decoded: QecConfig =
            serde_json::from_str(&encoded)
                .expect("configuration should deserialize");

        assert_eq!(config, decoded);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn capability_requirements_do_not_grant_authority() {
        let config = QecConfig::production();

        assert!(config.capabilities.decode);
        assert!(config.capabilities.allocate_memory);

        // This test documents the architectural contract:
        // these booleans are declarative requirements only.
        assert!(config.requires_capability(CapabilityName::Decode));
        assert!(
            config.requires_capability(
                CapabilityName::AllocateMemory
            )
        );
    }

    #[test]
    fn disabled_partitioning_requires_one_partition() {
        let mut config = QecConfig::production();

        config.partitioning.partitions = 2;

        assert!(config.validate().is_err());
    }

    #[test]
    fn scheduler_cannot_exceed_parallelism() {
        let mut config = QecConfig::production();

        config.scheduler.max_running_jobs =
            config.parallelism.max_workers as u32 + 1;

        assert!(config.validate().is_err());
    }

    #[test]
    fn backend_inflight_cannot_exceed_parallelism() {
        let mut config = QecConfig::production();

        config.backend.maximum_in_flight_operations =
            config.parallelism.max_workers as u32 + 1;

        assert!(config.validate().is_err());
    }
}