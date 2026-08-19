//! Zamani Quantum Error Correction — Execution Backend Abstraction.
//!
//! Production-grade execution abstraction for QEC workloads.
//!
//! Supported execution classes:
//!
//! ```text
//! QEC Backend
//! ├── CPU
//! ├── Parallel CPU
//! ├── GPU
//! ├── Accelerator
//! ├── Distributed
//! ├── Simulator
//! ├── Emulator
//! ├── QPU
//! └── Custom
//! ```
//!
//! Design principles:
//!
//! * QEC algorithms remain independent of execution hardware.
//! * Physical QPU access is adapter-based and never implicit.
//! * Backends must validate workloads before execution.
//! * Resource limits are explicit.
//! * Cancellation is cooperative.
//! * Deterministic execution can be requested and verified.
//! * Backend capabilities are explicit rather than inferred.
//! * Backend failures become structured errors.
//! * No unsafe code is required.
//! * No backend is allowed to silently exceed its declared limits.
//! * QPU execution is separated from QPU device/network I/O.
//!
//! The module deliberately does not perform network or device I/O.
//! Concrete providers should implement `QecBackend` and adapt their
//! simulator, accelerator, distributed runtime, or QPU provider to it.

#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Duration;

// -----------------------------------------------------------------------------
// Backend kind
// -----------------------------------------------------------------------------

/// Execution class used by a QEC workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendKind {
    /// Single-threaded CPU execution.
    Cpu,

    /// Multi-threaded CPU execution.
    ParallelCpu,

    /// GPU execution.
    Gpu,

    /// Dedicated quantum/AI/FPGA/etc. accelerator.
    Accelerator,

    /// Multi-process or multi-node execution.
    Distributed,

    /// Pure mathematical quantum simulator.
    Simulator,

    /// Hardware-faithful software emulator.
    Emulator,

    /// Physical quantum processing unit.
    Qpu,

    /// Application-specific backend.
    Custom,
}

impl BackendKind {
    /// Returns whether this backend represents physical quantum hardware.
    pub const fn is_physical_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether this backend is software-only.
    pub const fn is_software(self) -> bool {
        matches!(
            self,
            Self::Cpu
                | Self::ParallelCpu
                | Self::Gpu
                | Self::Accelerator
                | Self::Distributed
                | Self::Simulator
                | Self::Emulator
        )
    }

    /// Returns whether the backend may execute outside the local process.
    pub const fn may_be_remote(self) -> bool {
        matches!(self, Self::Distributed | Self::Qpu)
    }
}

// -----------------------------------------------------------------------------
// Operational status
// -----------------------------------------------------------------------------

/// Operational state of an execution backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStatus {
    Available,
    Busy,
    Degraded,
    Maintenance,
    Offline,
    Unavailable,
}

impl BackendStatus {
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Offline | Self::Unavailable)
    }
}

// -----------------------------------------------------------------------------
// Determinism
// -----------------------------------------------------------------------------

/// Requested determinism policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismPolicy {
    /// Backend may use nondeterministic execution.
    Allow,

    /// Backend must provide deterministic ordering/results where possible.
    Require,

    /// Backend must reject execution if deterministic execution cannot be
    /// guaranteed.
    Strict,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self::Require
    }
}

// -----------------------------------------------------------------------------
// Cancellation
// -----------------------------------------------------------------------------

/// Cooperative cancellation state.
///
/// Backend implementations should check this token at safe interruption
/// points. They must not leave shared mutable state partially committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationState {
    Active,
    Requested,
}

impl CancellationState {
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Requested)
    }
}

/// Minimal cancellation contract.
///
/// Concrete implementations may wrap an atomic flag, scheduler token,
/// distributed cancellation channel, or application cancellation primitive.
pub trait CancellationToken: Send + Sync {
    fn state(&self) -> CancellationState;

    fn is_cancelled(&self) -> bool {
        self.state().is_cancelled()
    }
}

/// A permanently active token.
///
/// Useful for callers that do not require cancellation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCancellation;

impl CancellationToken for NoCancellation {
    fn state(&self) -> CancellationState {
        CancellationState::Active
    }
}

// -----------------------------------------------------------------------------
// Resource policy
// -----------------------------------------------------------------------------

/// Explicit backend resource limits.
///
/// A value of `None` means that this particular resource has no limit imposed
/// by the backend layer. This does NOT imply infinite resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendResourceLimits {
    pub max_qubits: Option<usize>,
    pub max_stabilizers: Option<usize>,
    pub max_syndrome_events: Option<usize>,
    pub max_graph_nodes: Option<usize>,
    pub max_graph_edges: Option<usize>,
    pub max_rounds: Option<usize>,
    pub max_parallelism: Option<usize>,
    pub max_memory_bytes: Option<u64>,
    pub max_wall_time: Option<Duration>,
    pub max_cpu_time: Option<Duration>,
    pub max_shots: Option<usize>,
    pub max_checkpoint_bytes: Option<u64>,
}

impl Default for BackendResourceLimits {
    fn default() -> Self {
        Self {
            max_qubits: None,
            max_stabilizers: None,
            max_syndrome_events: None,
            max_graph_nodes: None,
            max_graph_edges: None,
            max_rounds: None,
            max_parallelism: None,
            max_memory_bytes: None,
            max_wall_time: None,
            max_cpu_time: None,
            max_shots: None,
            max_checkpoint_bytes: None,
        }
    }
}

impl BackendResourceLimits {
    pub const fn unlimited() -> Self {
        Self {
            max_qubits: None,
            max_stabilizers: None,
            max_syndrome_events: None,
            max_graph_nodes: None,
            max_graph_edges: None,
            max_rounds: None,
            max_parallelism: None,
            max_memory_bytes: None,
            max_wall_time: None,
            max_cpu_time: None,
            max_shots: None,
            max_checkpoint_bytes: None,
        }
    }

    pub fn validate(&self) -> Result<(), BackendError> {
        if matches!(self.max_parallelism, Some(0))
            || matches!(self.max_qubits, Some(0))
            || matches!(self.max_stabilizers, Some(0))
            || matches!(self.max_syndrome_events, Some(0))
            || matches!(self.max_graph_nodes, Some(0))
            || matches!(self.max_graph_edges, Some(0))
            || matches!(self.max_rounds, Some(0))
            || matches!(self.max_shots, Some(0))
        {
            return Err(BackendError::InvalidResourceLimit(
                "positive resource limits must be greater than zero",
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Backend capabilities
// -----------------------------------------------------------------------------

/// Explicit execution capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// Backend can execute QEC workloads.
    pub qec_execution: bool,

    /// Backend can execute syndrome generation.
    pub syndrome_generation: bool,

    /// Backend can execute decoding.
    pub decoding: bool,

    /// Backend can execute simulation.
    pub simulation: bool,

    /// Backend can apply/track Pauli-frame corrections.
    pub pauli_frame: bool,

    /// Backend can perform streaming syndrome processing.
    pub streaming: bool,

    /// Backend can partition workloads.
    pub partitioning: bool,

    /// Backend can execute distributed workloads.
    pub distributed: bool,

    /// Backend supports GPU/accelerator-style parallel kernels.
    pub acceleration: bool,

    /// Backend supports checkpoint/resume.
    pub checkpointing: bool,

    /// Backend supports cooperative cancellation.
    pub cancellation: bool,

    /// Backend can guarantee deterministic execution.
    pub deterministic_execution: bool,

    /// Backend supports physical QPU execution.
    pub physical_qpu: bool,

    /// Backend can expose hardware calibration information.
    pub calibration: bool,

    /// Backend supports mid-circuit measurement.
    pub mid_circuit_measurement: bool,

    /// Backend supports reset.
    pub reset: bool,

    /// Backend supports measurement.
    pub measurement: bool,

    /// Backend supports dynamic circuits.
    pub dynamic_circuits: bool,

    /// Backend supports classical feedback/control.
    pub classical_control: bool,

    /// Native operations understood by the backend.
    pub native_operations: BTreeSet<String>,
}

impl Default for BackendCapabilities {
    fn default() -> Self {
        Self {
            qec_execution: true,
            syndrome_generation: false,
            decoding: true,
            simulation: false,
            pauli_frame: true,
            streaming: false,
            partitioning: false,
            distributed: false,
            acceleration: false,
            checkpointing: false,
            cancellation: true,
            deterministic_execution: true,
            physical_qpu: false,
            calibration: false,
            mid_circuit_measurement: false,
            reset: false,
            measurement: false,
            dynamic_circuits: false,
            classical_control: false,
            native_operations: BTreeSet::new(),
        }
    }
}

impl BackendCapabilities {
    pub fn supports_operation(&self, operation: &str) -> bool {
        self.native_operations
            .contains(&normalize_name(operation))
    }

    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.native_operations
            .insert(normalize_name(&operation.into()));
        self
    }

    pub fn with_operations<I, S>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for operation in operations {
            self.native_operations
                .insert(normalize_name(&operation.into()));
        }

        self
    }
}

// -----------------------------------------------------------------------------
// Topology
// -----------------------------------------------------------------------------

/// Logical QEC backend topology.
///
/// This is deliberately independent of the broader hardware topology type so
/// the QEC subsystem can validate decoding/syndrome workloads without
/// requiring a physical-device implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendTopology {
    pub qubit_count: usize,
    pub edges: BTreeSet<(usize, usize)>,
}

impl BackendTopology {
    pub fn new(qubit_count: usize) -> Result<Self, BackendError> {
        if qubit_count == 0 {
            return Err(BackendError::ZeroQubits);
        }

        Ok(Self {
            qubit_count,
            edges: BTreeSet::new(),
        })
    }

    pub fn add_edge(
        &mut self,
        a: usize,
        b: usize,
    ) -> Result<(), BackendError> {
        if a >= self.qubit_count {
            return Err(BackendError::InvalidQubit {
                qubit: a,
                qubit_count: self.qubit_count,
            });
        }

        if b >= self.qubit_count {
            return Err(BackendError::InvalidQubit {
                qubit: b,
                qubit_count: self.qubit_count,
            });
        }

        if a == b {
            return Err(BackendError::InvalidTopology(
                "self-loops are not valid backend connections",
            ));
        }

        let edge = if a < b { (a, b) } else { (b, a) };

        self.edges.insert(edge);

        Ok(())
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        let edge = if a < b { (a, b) } else { (b, a) };

        self.edges.contains(&edge)
    }

    pub fn validate(&self) -> Result<(), BackendError> {
        if self.qubit_count == 0 {
            return Err(BackendError::ZeroQubits);
        }

        for &(a, b) in &self.edges {
            if a >= self.qubit_count || b >= self.qubit_count {
                return Err(BackendError::InvalidTopology(
                    "topology edge references an invalid qubit",
                ));
            }

            if a == b {
                return Err(BackendError::InvalidTopology(
                    "topology contains a self-loop",
                ));
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Backend metadata
// -----------------------------------------------------------------------------

/// Stable backend identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendMetadata {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub version: String,
    pub kind: BackendKind,
    pub status: BackendStatus,

    /// Backend-specific immutable metadata.
    pub properties: BTreeMap<String, String>,
}

impl BackendMetadata {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
        version: impl Into<String>,
        kind: BackendKind,
    ) -> Result<Self, BackendError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(BackendError::InvalidBackendId);
        }

        Ok(Self {
            id,
            name: name.into(),
            provider: provider.into(),
            version: version.into(),
            kind,
            status: BackendStatus::Available,
            properties: BTreeMap::new(),
        })
    }

    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.properties.insert(key.into(), value.into());
    }
}

// -----------------------------------------------------------------------------
// Workload requirements
// -----------------------------------------------------------------------------

/// Resource requirements for a QEC workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecWorkload {
    pub qubits: usize,
    pub stabilizers: usize,
    pub syndrome_events: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub rounds: usize,
    pub parallelism: usize,
    pub memory_bytes: u64,
    pub shots: usize,

    /// Operations required by the workload.
    pub operations: BTreeSet<String>,

    /// Whether the workload requires deterministic execution.
    pub requires_determinism: bool,

    /// Whether the workload requires physical hardware.
    pub requires_qpu: bool,

    /// Whether the workload requires calibration data.
    pub requires_calibration: bool,

    /// Whether execution may be streamed.
    pub streaming: bool,

    /// Whether the workload is partitionable.
    pub partitionable: bool,

    /// Whether the workload can be distributed.
    pub distributable: bool,

    /// Whether cancellation must be supported.
    pub cancellable: bool,
}

impl Default for QecWorkload {
    fn default() -> Self {
        Self {
            qubits: 1,
            stabilizers: 0,
            syndrome_events: 0,
            graph_nodes: 0,
            graph_edges: 0,
            rounds: 0,
            parallelism: 1,
            memory_bytes: 0,
            shots: 1,
            operations: BTreeSet::new(),
            requires_determinism: false,
            requires_qpu: false,
            requires_calibration: false,
            streaming: false,
            partitionable: false,
            distributable: false,
            cancellable: true,
        }
    }
}

impl QecWorkload {
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operations
            .insert(normalize_name(&operation.into()));
        self
    }

    pub fn requires_operation(&self, operation: &str) -> bool {
        self.operations
            .contains(&normalize_name(operation))
    }
}

// -----------------------------------------------------------------------------
// Backend configuration
// -----------------------------------------------------------------------------

/// Immutable execution configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendConfig {
    pub determinism: DeterminismPolicy,
    pub limits: BackendResourceLimits,

    /// Maximum number of retries permitted by an execution adapter.
    pub max_retries: u32,

    /// Whether degraded backends may execute.
    pub allow_degraded: bool,

    /// Whether fallback to another backend is permitted.
    pub allow_fallback: bool,

    /// Whether physical QPU execution requires an explicit capability.
    pub require_explicit_qpu_capability: bool,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            determinism: DeterminismPolicy::Require,
            limits: BackendResourceLimits::default(),
            max_retries: 0,
            allow_degraded: false,
            allow_fallback: false,
            require_explicit_qpu_capability: true,
        }
    }
}

impl BackendConfig {
    pub fn validate(&self) -> Result<(), BackendError> {
        self.limits.validate()
    }
}

// -----------------------------------------------------------------------------
// QPU-specific execution information
// -----------------------------------------------------------------------------

/// Physical QPU state.
///
/// This deliberately contains descriptive state only. It does not open
/// network connections or communicate with a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuInfo {
    pub device_id: String,
    pub provider: String,
    pub architecture: String,
    pub calibration_id: Option<String>,
    pub calibration_version: Option<String>,
    pub queue_depth: Option<usize>,
    pub available_qubits: usize,
    pub supported_native_operations: BTreeSet<String>,
    pub topology: BackendTopology,
}

impl QpuInfo {
    pub fn validate(&self) -> Result<(), BackendError> {
        if self.device_id.trim().is_empty() {
            return Err(BackendError::InvalidQpu(
                "QPU device ID cannot be empty",
            ));
        }

        if self.provider.trim().is_empty() {
            return Err(BackendError::InvalidQpu(
                "QPU provider cannot be empty",
            ));
        }

        if self.available_qubits == 0 {
            return Err(BackendError::InvalidQpu(
                "QPU must expose at least one available qubit",
            ));
        }

        self.topology.validate()?;

        if self.topology.qubit_count < self.available_qubits {
            return Err(BackendError::InvalidQpu(
                "available qubit count exceeds topology size",
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Resource accounting
// -----------------------------------------------------------------------------

/// Runtime resource accounting for one backend execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendResourceUsage {
    pub allocated_memory_bytes: u64,
    pub peak_memory_bytes: u64,
    pub cpu_time: Duration,
    pub wall_time: Duration,
    pub qubits: usize,
    pub stabilizers: usize,
    pub syndrome_events: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub rounds: usize,
    pub decoder_iterations: usize,
    pub parallel_workers: usize,
    pub shots: usize,
}

impl Default for BackendResourceUsage {
    fn default() -> Self {
        Self {
            allocated_memory_bytes: 0,
            peak_memory_bytes: 0,
            cpu_time: Duration::ZERO,
            wall_time: Duration::ZERO,
            qubits: 0,
            stabilizers: 0,
            syndrome_events: 0,
            graph_nodes: 0,
            graph_edges: 0,
            rounds: 0,
            decoder_iterations: 0,
            parallel_workers: 0,
            shots: 0,
        }
    }
}

impl BackendResourceUsage {
    pub fn validate_against(
        &self,
        limits: &BackendResourceLimits,
    ) -> Result<(), BackendError> {
        check_limit(
            self.qubits,
            limits.max_qubits,
            BackendError::QubitLimitExceeded,
        )?;

        check_limit(
            self.stabilizers,
            limits.max_stabilizers,
            BackendError::StabilizerLimitExceeded,
        )?;

        check_limit(
            self.syndrome_events,
            limits.max_syndrome_events,
            BackendError::SyndromeEventLimitExceeded,
        )?;

        check_limit(
            self.graph_nodes,
            limits.max_graph_nodes,
            BackendError::GraphNodeLimitExceeded,
        )?;

        check_limit(
            self.graph_edges,
            limits.max_graph_edges,
            BackendError::GraphEdgeLimitExceeded,
        )?;

        check_limit(
            self.rounds,
            limits.max_rounds,
            BackendError::RoundLimitExceeded,
        )?;

        check_limit(
            self.parallel_workers,
            limits.max_parallelism,
            BackendError::ParallelismLimitExceeded,
        )?;

        if let Some(max) = limits.max_memory_bytes {
            if self.peak_memory_bytes > max {
                return Err(BackendError::MemoryLimitExceeded {
                    requested: self.peak_memory_bytes,
                    maximum: max,
                });
            }
        }

        if let Some(max) = limits.max_wall_time {
            if self.wall_time > max {
                return Err(BackendError::WallTimeLimitExceeded);
            }
        }

        if let Some(max) = limits.max_cpu_time {
            if self.cpu_time > max {
                return Err(BackendError::CpuTimeLimitExceeded);
            }
        }

        if let Some(max) = limits.max_shots {
            if self.shots > max {
                return Err(BackendError::ShotLimitExceeded {
                    requested: self.shots,
                    maximum: max,
                });
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Execution request
// -----------------------------------------------------------------------------

/// A validated request envelope passed to a backend.
#[derive(Debug, Clone)]
pub struct BackendExecutionRequest {
    pub workload: QecWorkload,
    pub config: BackendConfig,
}

impl BackendExecutionRequest {
    pub fn new(
        workload: QecWorkload,
        config: BackendConfig,
    ) -> Result<Self, BackendError> {
        config.validate()?;

        Ok(Self { workload, config })
    }
}

// -----------------------------------------------------------------------------
// Execution result
// -----------------------------------------------------------------------------

/// Generic QEC execution result.
///
/// Backend-specific information belongs in `metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendExecutionResult {
    pub backend_id: String,
    pub kind: BackendKind,

    pub success: bool,
    pub cancelled: bool,

    pub deterministic: bool,

    pub logical_failure: bool,

    pub correction_count: usize,
    pub detection_event_count: usize,
    pub matching_count: usize,

    pub resource_usage: BackendResourceUsage,

    /// Stable backend-specific metadata.
    pub metadata: BTreeMap<String, String>,
}

impl BackendExecutionResult {
    pub fn success(
        backend_id: impl Into<String>,
        kind: BackendKind,
    ) -> Self {
        Self {
            backend_id: backend_id.into(),
            kind,
            success: true,
            cancelled: false,
            deterministic: true,
            logical_failure: false,
            correction_count: 0,
            detection_event_count: 0,
            matching_count: 0,
            resource_usage: BackendResourceUsage::default(),
            metadata: BTreeMap::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// Backend trait
// -----------------------------------------------------------------------------

/// Core QEC backend contract.
///
/// Implementations may target:
///
/// * CPU
/// * parallel CPU
/// * GPU
/// * FPGA/ASIC/other accelerator
/// * distributed workers
/// * simulator
/// * emulator
/// * physical QPU
///
/// The trait contains no device/network assumptions.
pub trait QecBackend: Send + Sync {
    /// Backend metadata.
    fn metadata(&self) -> &BackendMetadata;

    /// Backend capabilities.
    fn capabilities(&self) -> &BackendCapabilities;

    /// Backend resource limits.
    fn limits(&self) -> &BackendResourceLimits;

    /// Backend topology, if applicable.
    fn topology(&self) -> Option<&BackendTopology>;

    /// Validate a workload without executing it.
    fn validate(
        &self,
        request: &BackendExecutionRequest,
    ) -> Result<(), BackendError>;

    /// Execute a validated workload.
    fn execute(
        &self,
        request: &BackendExecutionRequest,
        cancellation: &dyn CancellationToken,
    ) -> Result<BackendExecutionResult, BackendError>;

    /// Whether deterministic execution is guaranteed.
    fn guarantees_determinism(&self) -> bool {
        self.capabilities().deterministic_execution
    }

    /// Whether this is a physical QPU backend.
    fn is_qpu(&self) -> bool {
        self.metadata().kind.is_physical_qpu()
    }
}

// -----------------------------------------------------------------------------
// Backend descriptor
// -----------------------------------------------------------------------------

/// Reusable backend descriptor.
///
/// Useful for registration, scheduling, discovery, and routing.
#[derive(Debug, Clone)]
pub struct BackendDescriptor {
    pub metadata: BackendMetadata,
    pub capabilities: BackendCapabilities,
    pub limits: BackendResourceLimits,
    pub topology: Option<BackendTopology>,
    pub qpu_info: Option<QpuInfo>,
}

impl BackendDescriptor {
    pub fn validate(&self) -> Result<(), BackendError> {
        self.limits.validate()?;

        if let Some(topology) = &self.topology {
            topology.validate()?;
        }

        if let Some(qpu) = &self.qpu_info {
            qpu.validate()?;

            if self.metadata.kind != BackendKind::Qpu {
                return Err(BackendError::InvalidQpu(
                    "QPU information supplied for a non-QPU backend",
                ));
            }
        }

        if self.metadata.kind == BackendKind::Qpu
            && !self.capabilities.physical_qpu
        {
            return Err(BackendError::InvalidQpu(
                "QPU backend must explicitly advertise physical_qpu capability",
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

/// Validate a workload against a backend descriptor.
///
/// This function is deliberately pure and deterministic.
pub fn validate_workload(
    backend: &BackendDescriptor,
    request: &BackendExecutionRequest,
) -> Result<(), BackendError> {
    backend.validate()?;
    request.config.validate()?;

    let workload = &request.workload;

    if workload.qubits == 0 {
        return Err(BackendError::ZeroQubits);
    }

    check_limit(
        workload.qubits,
        backend.limits.max_qubits,
        BackendError::QubitLimitExceeded,
    )?;

    check_limit(
        workload.stabilizers,
        backend.limits.max_stabilizers,
        BackendError::StabilizerLimitExceeded,
    )?;

    check_limit(
        workload.syndrome_events,
        backend.limits.max_syndrome_events,
        BackendError::SyndromeEventLimitExceeded,
    )?;

    check_limit(
        workload.graph_nodes,
        backend.limits.max_graph_nodes,
        BackendError::GraphNodeLimitExceeded,
    )?;

    check_limit(
        workload.graph_edges,
        backend.limits.max_graph_edges,
        BackendError::GraphEdgeLimitExceeded,
    )?;

    check_limit(
        workload.rounds,
        backend.limits.max_rounds,
        BackendError::RoundLimitExceeded,
    )?;

    check_limit(
        workload.parallelism,
        backend.limits.max_parallelism,
        BackendError::ParallelismLimitExceeded,
    )?;

    if let Some(max) = backend.limits.max_memory_bytes {
        if workload.memory_bytes > max {
            return Err(BackendError::MemoryLimitExceeded {
                requested: workload.memory_bytes,
                maximum: max,
            });
        }
    }

    if let Some(max) = backend.limits.max_shots {
        if workload.shots > max {
            return Err(BackendError::ShotLimitExceeded {
                requested: workload.shots,
                maximum: max,
            });
        }
    }

    let capabilities = &backend.capabilities;

    if workload.requires_determinism
        && !capabilities.deterministic_execution
    {
        return Err(BackendError::DeterminismUnavailable);
    }

    if workload.streaming && !capabilities.streaming {
        return Err(BackendError::UnsupportedFeature("streaming"));
    }

    if workload.partitionable && !capabilities.partitioning {
        return Err(BackendError::UnsupportedFeature("partitioning"));
    }

    if workload.distributable && !capabilities.distributed {
        return Err(BackendError::UnsupportedFeature("distributed execution"));
    }

    if workload.cancellable && !capabilities.cancellation {
        return Err(BackendError::UnsupportedFeature("cancellation"));
    }

    if workload.requires_calibration && !capabilities.calibration {
        return Err(BackendError::CalibrationUnavailable);
    }

    if workload.requires_qpu {
        if backend.metadata.kind != BackendKind::Qpu {
            return Err(BackendError::QpuRequired);
        }

        if !capabilities.physical_qpu {
            return Err(BackendError::QpuCapabilityUnavailable);
        }
    }

    for operation in &workload.operations {
        if !capabilities.supports_operation(operation) {
            return Err(BackendError::UnsupportedOperation {
                operation: operation.clone(),
            });
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Backend registry
// -----------------------------------------------------------------------------

/// Deterministic registry of backend descriptors.
///
/// Registration is metadata-only. It does not establish device connections.
#[derive(Debug, Default, Clone)]
pub struct BackendRegistry {
    backends: BTreeMap<String, BackendDescriptor>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        backend: BackendDescriptor,
    ) -> Result<(), BackendError> {
        backend.validate()?;

        let id = backend.metadata.id.clone();

        if self.backends.contains_key(&id) {
            return Err(BackendError::DuplicateBackend(id));
        }

        self.backends.insert(id, backend);

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&BackendDescriptor> {
        self.backends.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.backends.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.backends.len()
    }

    pub fn is_empty(&self) -> bool {
        self.backends.is_empty()
    }

    pub fn list(&self) -> impl Iterator<Item = &BackendDescriptor> {
        self.backends.values()
    }

    /// Return deterministic backend candidates for a workload.
    pub fn candidates(
        &self,
        workload: &QecWorkload,
    ) -> Vec<&BackendDescriptor> {
        self.backends
            .values()
            .filter(|backend| {
                let request = BackendExecutionRequest {
                    workload: workload.clone(),
                    config: BackendConfig::default(),
                };

                validate_workload(backend, &request).is_ok()
            })
            .collect()
    }
}

// -----------------------------------------------------------------------------
// QPU adapter contract
// -----------------------------------------------------------------------------

/// Adapter boundary for physical QPUs.
///
/// This trait intentionally separates:
///
/// ```text
/// QEC
///   ↓
/// QpuBackend
///   ↓
/// provider adapter
///   ↓
/// actual device/network API
/// ```
///
/// from the QEC algorithms themselves.
///
/// A provider implementation is responsible for authentication, transport,
/// device queues, calibration retrieval, submission, polling, and provider
/// specific failures.
pub trait QpuBackend: QecBackend {
    /// Immutable description of the physical QPU.
    fn qpu_info(&self) -> &QpuInfo;

    /// Returns whether current calibration data is usable.
    fn calibration_is_current(&self) -> bool;

    /// Returns whether the physical device is currently accepting work.
    fn accepts_work(&self) -> bool;

    /// Provider-side execution identifier, if one exists.
    fn execution_id(&self) -> Option<&str> {
        None
    }
}

// -----------------------------------------------------------------------------
// Backend capability constructors
// -----------------------------------------------------------------------------

/// Conservative CPU backend capabilities.
pub fn cpu_capabilities() -> BackendCapabilities {
    BackendCapabilities {
        qec_execution: true,
        syndrome_generation: true,
        decoding: true,
        simulation: true,
        pauli_frame: true,
        streaming: true,
        partitioning: true,
        distributed: false,
        acceleration: false,
        checkpointing: true,
        cancellation: true,
        deterministic_execution: true,
        physical_qpu: false,
        calibration: false,
        mid_circuit_measurement: false,
        reset: false,
        measurement: true,
        dynamic_circuits: false,
        classical_control: false,
        native_operations: BTreeSet::new(),
    }
}

/// Parallel CPU capabilities.
pub fn parallel_cpu_capabilities() -> BackendCapabilities {
    let mut capabilities = cpu_capabilities();

    capabilities.partitioning = true;
    capabilities.streaming = true;
    capabilities.checkpointing = true;
    capabilities.deterministic_execution = true;

    capabilities
}

/// GPU capabilities.
pub fn gpu_capabilities() -> BackendCapabilities {
    let mut capabilities = cpu_capabilities();

    capabilities.acceleration = true;
    capabilities.partitioning = true;
    capabilities.streaming = true;
    capabilities.deterministic_execution = true;

    capabilities
}

/// Generic accelerator capabilities.
pub fn accelerator_capabilities() -> BackendCapabilities {
    let mut capabilities = gpu_capabilities();

    capabilities.checkpointing = false;

    capabilities
}

/// Distributed backend capabilities.
pub fn distributed_capabilities() -> BackendCapabilities {
    let mut capabilities = parallel_cpu_capabilities();

    capabilities.distributed = true;
    capabilities.partitioning = true;
    capabilities.streaming = true;
    capabilities.checkpointing = true;

    capabilities
}

/// Simulator capabilities.
pub fn simulator_capabilities() -> BackendCapabilities {
    let mut capabilities = cpu_capabilities();

    capabilities.simulation = true;
    capabilities.deterministic_execution = true;

    capabilities
}

/// Emulator capabilities.
pub fn emulator_capabilities() -> BackendCapabilities {
    let mut capabilities = simulator_capabilities();

    capabilities.calibration = true;

    capabilities
}

/// Conservative physical QPU capabilities.
///
/// Concrete QPU providers should construct a more precise capability profile
/// from their actual device contract.
pub fn qpu_capabilities() -> BackendCapabilities {
    BackendCapabilities {
        qec_execution: true,
        syndrome_generation: true,
        decoding: false,
        simulation: false,
        pauli_frame: true,
        streaming: true,
        partitioning: false,
        distributed: false,
        acceleration: false,
        checkpointing: false,
        cancellation: true,
        deterministic_execution: false,
        physical_qpu: true,
        calibration: true,
        mid_circuit_measurement: true,
        reset: true,
        measurement: true,
        dynamic_circuits: true,
        classical_control: true,
        native_operations: BTreeSet::new(),
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Unified backend-layer error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    BackendUnavailable {
        backend_id: String,
        status: BackendStatus,
    },

    InvalidBackendId,

    DuplicateBackend(String),

    ZeroQubits,

    InvalidQubit {
        qubit: usize,
        qubit_count: usize,
    },

    InvalidTopology(&'static str),

    InvalidQpu(&'static str),

    InvalidResourceLimit(&'static str),

    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    StabilizerLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    SyndromeEventLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    GraphNodeLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    GraphEdgeLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    RoundLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    ParallelismLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    MemoryLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    WallTimeLimitExceeded,

    CpuTimeLimitExceeded,

    UnsupportedOperation {
        operation: String,
    },

    UnsupportedFeature(&'static str),

    DeterminismUnavailable,

    CalibrationUnavailable,

    QpuRequired,

    QpuCapabilityUnavailable,

    CancellationRequested,

    ExecutionFailure(String),

    UnsupportedBackendKind(BackendKind),
}

impl fmt::Display for BackendError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::BackendUnavailable {
                backend_id,
                status,
            } => write!(
                f,
                "backend '{}' is unavailable: {:?}",
                backend_id,
                status
            ),

            Self::InvalidBackendId => {
                write!(f, "backend ID cannot be empty")
            }

            Self::DuplicateBackend(id) => {
                write!(f, "backend '{}' is already registered", id)
            }

            Self::ZeroQubits => {
                write!(f, "QEC workload must contain at least one qubit")
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => write!(
                f,
                "qubit {} is outside range 0..{}",
                qubit,
                qubit_count.saturating_sub(1)
            ),

            Self::InvalidTopology(message) => {
                write!(f, "invalid backend topology: {}", message)
            }

            Self::InvalidQpu(message) => {
                write!(f, "invalid QPU configuration: {}", message)
            }

            Self::InvalidResourceLimit(message) => {
                write!(f, "invalid resource limit: {}", message)
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests {} qubits; limit is {}",
                requested,
                maximum
            ),

            Self::StabilizerLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests {} stabilizers; limit is {}",
                requested,
                maximum
            ),

            Self::SyndromeEventLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests {} syndrome events; limit is {}",
                requested,
                maximum
            ),

            Self::GraphNodeLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC graph requests {} nodes; limit is {}",
                requested,
                maximum
            ),

            Self::GraphEdgeLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC graph requests {} edges; limit is {}",
                requested,
                maximum
            ),

            Self::RoundLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests {} rounds; limit is {}",
                requested,
                maximum
            ),

            Self::ParallelismLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests parallelism {}; limit is {}",
                requested,
                maximum
            ),

            Self::MemoryLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requires {} bytes; memory limit is {} bytes",
                requested,
                maximum
            ),

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "QEC workload requests {} shots; limit is {}",
                requested,
                maximum
            ),

            Self::WallTimeLimitExceeded => {
                write!(f, "QEC workload exceeded wall-time limit")
            }

            Self::CpuTimeLimitExceeded => {
                write!(f, "QEC workload exceeded CPU-time limit")
            }

            Self::UnsupportedOperation { operation } => {
                write!(
                    f,
                    "backend does not support operation '{}'",
                    operation
                )
            }

            Self::UnsupportedFeature(feature) => {
                write!(f, "backend does not support {}", feature)
            }

            Self::DeterminismUnavailable => {
                write!(
                    f,
                    "requested deterministic execution cannot be guaranteed"
                )
            }

            Self::CalibrationUnavailable => {
                write!(
                    f,
                    "required QPU calibration information is unavailable"
                )
            }

            Self::QpuRequired => {
                write!(f, "workload explicitly requires a physical QPU")
            }

            Self::QpuCapabilityUnavailable => {
                write!(
                    f,
                    "backend is not authorized/capable of physical QPU execution"
                )
            }

            Self::CancellationRequested => {
                write!(f, "QEC backend execution was cancelled")
            }

            Self::ExecutionFailure(message) => {
                write!(f, "backend execution failure: {}", message)
            }

            Self::UnsupportedBackendKind(kind) => {
                write!(f, "unsupported backend kind: {:?}", kind)
            }
        }
    }
}

impl std::error::Error for BackendError {}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn check_limit<F>(
    requested: usize,
    maximum: Option<usize>,
    constructor: F,
) -> Result<(), BackendError>
where
    F: FnOnce(usize, usize) -> BackendError,
{
    if let Some(maximum) = maximum {
        if requested > maximum {
            return Err(constructor(requested, maximum));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_kinds_are_classified_correctly() {
        assert!(BackendKind::Qpu.is_physical_qpu());
        assert!(!BackendKind::Cpu.is_physical_qpu());
        assert!(BackendKind::Distributed.may_be_remote());
        assert!(BackendKind::Qpu.may_be_remote());
        assert!(BackendKind::Cpu.is_software());
    }

    #[test]
    fn topology_rejects_invalid_qubits() {
        let mut topology =
            BackendTopology::new(2)
                .expect("valid topology");

        let result =
            topology.add_edge(0, 2);

        assert!(matches!(
            result,
            Err(BackendError::InvalidQubit { .. })
        ));
    }

    #[test]
    fn topology_normalizes_edges() {
        let mut topology =
            BackendTopology::new(2)
                .expect("valid topology");

        topology
            .add_edge(1, 0)
            .expect("valid edge");

        assert!(topology.connected(0, 1));
        assert!(topology.connected(1, 0));
        assert_eq!(topology.edges.len(), 1);
    }

    #[test]
    fn zero_qubit_workload_is_rejected() {
        let backend = BackendDescriptor {
            metadata: BackendMetadata::new(
                "cpu",
                "CPU",
                "Zamani",
                "1.0",
                BackendKind::Cpu,
            )
            .expect("metadata"),

            capabilities: cpu_capabilities(),

            limits: BackendResourceLimits::default(),

            topology: Some(
                BackendTopology::new(4)
                    .expect("topology"),
            ),

            qpu_info: None,
        };

        let workload = QecWorkload {
            qubits: 0,
            ..QecWorkload::default()
        };

        let request =
            BackendExecutionRequest::new(
                workload,
                BackendConfig::default(),
            )
            .expect("configuration");

        assert!(matches!(
            validate_workload(&backend, &request),
            Err(BackendError::ZeroQubits)
        ));
    }

    #[test]
    fn memory_limit_is_enforced() {
        let backend = BackendDescriptor {
            metadata: BackendMetadata::new(
                "cpu",
                "CPU",
                "Zamani",
                "1.0",
                BackendKind::Cpu,
            )
            .expect("metadata"),

            capabilities: cpu_capabilities(),

            limits: BackendResourceLimits {
                max_memory_bytes: Some(1024),
                ..BackendResourceLimits::default()
            },

            topology: Some(
                BackendTopology::new(8)
                    .expect("topology"),
            ),

            qpu_info: None,
        };

        let workload = QecWorkload {
            qubits: 4,
            memory_bytes: 2048,
            ..QecWorkload::default()
        };

        let request =
            BackendExecutionRequest::new(
                workload,
                BackendConfig::default(),
            )
            .expect("configuration");

        assert!(matches!(
            validate_workload(&backend, &request),
            Err(BackendError::MemoryLimitExceeded { .. })
        ));
    }

    #[test]
    fn qpu_workload_requires_qpu_backend() {
        let backend = BackendDescriptor {
            metadata: BackendMetadata::new(
                "cpu",
                "CPU",
                "Zamani",
                "1.0",
                BackendKind::Cpu,
            )
            .expect("metadata"),

            capabilities: cpu_capabilities(),

            limits: BackendResourceLimits::unlimited(),

            topology: Some(
                BackendTopology::new(8)
                    .expect("topology"),
            ),

            qpu_info: None,
        };

        let workload = QecWorkload {
            qubits: 4,
            requires_qpu: true,
            ..QecWorkload::default()
        };

        let request =
            BackendExecutionRequest::new(
                workload,
                BackendConfig::default(),
            )
            .expect("configuration");

        assert!(matches!(
            validate_workload(&backend, &request),
            Err(BackendError::QpuRequired)
        ));
    }

    #[test]
    fn qpu_descriptor_requires_physical_qpu_capability() {
        let descriptor = BackendDescriptor {
            metadata: BackendMetadata::new(
                "qpu0",
                "Physical QPU",
                "Provider",
                "1.0",
                BackendKind::Qpu,
            )
            .expect("metadata"),

            capabilities: BackendCapabilities::default(),

            limits: BackendResourceLimits::unlimited(),

            topology: Some(
                BackendTopology::new(5)
                    .expect("topology"),
            ),

            qpu_info: None,
        };

        assert!(matches!(
            descriptor.validate(),
            Err(BackendError::InvalidQpu(_))
        ));
    }

    #[test]
    fn qpu_info_validates() {
        let topology =
            BackendTopology::new(5)
                .expect("topology");

        let qpu = QpuInfo {
            device_id: "qpu-0".to_string(),
            provider: "ZamaniProvider".to_string(),
            architecture: "surface-code".to_string(),
            calibration_id: Some("cal-1".to_string()),
            calibration_version: Some("1".to_string()),
            queue_depth: Some(2),
            available_qubits: 5,
            supported_native_operations:
                BTreeSet::new(),
            topology,
        };

        assert!(qpu.validate().is_ok());
    }

    #[test]
    fn registry_rejects_duplicates() {
        let descriptor = || BackendDescriptor {
            metadata: BackendMetadata::new(
                "cpu",
                "CPU",
                "Zamani",
                "1.0",
                BackendKind::Cpu,
            )
            .expect("metadata"),

            capabilities: cpu_capabilities(),

            limits: BackendResourceLimits::unlimited(),

            topology: Some(
                BackendTopology::new(4)
                    .expect("topology"),
            ),

            qpu_info: None,
        };

        let mut registry =
            BackendRegistry::new();

        registry
            .register(descriptor())
            .expect("first registration");

        assert!(matches!(
            registry.register(descriptor()),
            Err(BackendError::DuplicateBackend(_))
        ));
    }

    #[test]
    fn deterministic_policy_defaults_to_require() {
        assert_eq!(
            BackendConfig::default().determinism,
            DeterminismPolicy::Require
        );
    }

    #[test]
    fn cancellation_token_defaults_to_active() {
        let token = NoCancellation;

        assert!(!token.is_cancelled());
    }

    #[test]
    fn resource_usage_detects_memory_exhaustion() {
        let limits = BackendResourceLimits {
            max_memory_bytes: Some(100),
            ..BackendResourceLimits::default()
        };

        let usage = BackendResourceUsage {
            peak_memory_bytes: 101,
            ..BackendResourceUsage::default()
        };

        assert!(matches!(
            usage.validate_against(&limits),
            Err(BackendError::MemoryLimitExceeded { .. })
        ));
    }

    #[test]
    fn qpu_capabilities_are_explicit() {
        let capabilities =
            qpu_capabilities();

        assert!(capabilities.physical_qpu);
        assert!(capabilities.calibration);
        assert!(capabilities.measurement);
        assert!(capabilities.reset);
        assert!(capabilities.mid_circuit_measurement);
        assert!(capabilities.dynamic_circuits);

        // Physical hardware should not be assumed deterministic.
        assert!(!capabilities.deterministic_execution);
    }

    #[test]
    fn operation_names_are_normalized() {
        let capabilities =
            BackendCapabilities::default()
                .with_operation(" MWPM ");

        assert!(
            capabilities.supports_operation("mwpm")
        );
    }
}