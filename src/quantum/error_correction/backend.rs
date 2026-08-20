//! Zamani Quantum Error Correction — execution backend abstraction.
//!
//! This module defines the execution boundary between QEC mathematics and
//! execution environments.
//!
//! Architectural contract:
//!
//! ```text
//!                     QecConfig
//!                         │
//!                         ▼
//!                     QecLimits
//!                         │
//!                         ▼
//!                 Backend preflight
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!       CPU/GPU       Simulator        QPU adapter
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                    QEC execution
//!                         │
//!                         ▼
//!                 structured result
//! ```
//!
//! Important rules:
//!
//! * `QecLimits` is the canonical resource policy.
//! * This module does not define a competing global resource policy.
//! * Backend capabilities are explicit.
//! * Workloads are validated before execution.
//! * Resource estimates are checked before execution.
//! * Cancellation is cooperative.
//! * Deterministic execution is explicit.
//! * QPU device/network I/O is NOT performed here.
//! * Physical QPU execution must occur through a dedicated adapter.
//! * Backend implementations must never silently exceed declared limits.
//! * Public failures use the unified `QecError` boundary.
//!
//! The backend layer is therefore a control-plane and execution abstraction,
//! not a hardware driver.

#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::errors::{
    DecoderKind,
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::{
    LimitError,
    LimitKind,
    QecLimits,
};

// ============================================================================
// Backend kind
// ============================================================================

/// Execution class used by a QEC workload.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum BackendKind {
    Cpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
    Simulator,
    Emulator,
    Qpu,
    Custom,
}

impl BackendKind {
    pub const fn is_physical_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

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

    pub const fn may_be_remote(self) -> bool {
        matches!(self, Self::Distributed | Self::Qpu)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::ParallelCpu => "parallel_cpu",
            Self::Gpu => "gpu",
            Self::Accelerator => "accelerator",
            Self::Distributed => "distributed",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for BackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Backend status
// ============================================================================

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

// ============================================================================
// Determinism
// ============================================================================

/// Requested determinism policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismPolicy {
    /// Nondeterministic execution is permitted.
    Allow,

    /// Deterministic execution should be used when supported.
    Require,

    /// Execution must be rejected if determinism cannot be guaranteed.
    Strict,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self::Require
    }
}

// ============================================================================
// Cancellation
// ============================================================================

/// Cooperative cancellation state.
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
/// The QEC cancellation module can provide an implementation of this trait.
/// Keeping the trait here also allows backend implementations to remain
/// independent of a particular cancellation primitive.
pub trait CancellationToken: Send + Sync {
    fn state(&self) -> CancellationState;

    fn is_cancelled(&self) -> bool {
        self.state().is_cancelled()
    }
}

/// Cancellation implementation used when the caller does not require
/// cancellation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCancellation;

impl CancellationToken for NoCancellation {
    fn state(&self) -> CancellationState {
        CancellationState::Active
    }
}

// ============================================================================
// Backend capabilities
// ============================================================================

/// Explicit capabilities advertised by an execution backend.
///
/// Capabilities are deliberately independent from authorization. A backend
/// may technically support an operation while the caller's capability set
/// does not permit it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    pub qec_execution: bool,
    pub syndrome_generation: bool,
    pub decoding: bool,
    pub simulation: bool,
    pub pauli_frame: bool,

    pub streaming: bool,
    pub partitioning: bool,
    pub distributed: bool,
    pub acceleration: bool,

    pub checkpointing: bool,
    pub cancellation: bool,
    pub deterministic_execution: bool,

    pub physical_qpu: bool,
    pub calibration: bool,

    pub mid_circuit_measurement: bool,
    pub reset: bool,
    pub measurement: bool,
    pub dynamic_circuits: bool,
    pub classical_control: bool,

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

    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.native_operations
            .insert(normalize_name(&operation.into()));
        self
    }

    pub fn with_operations<I, S>(
        mut self,
        operations: I,
    ) -> Self
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

// ============================================================================
// Backend resource limits
// ============================================================================

/// Optional backend-specific ceilings.
///
/// This is intentionally NOT the canonical QEC policy.
///
/// `QecLimits` remains authoritative. These values represent physical
/// backend capacity and can only further restrict an execution.
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
    pub max_shots: Option<u64>,
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
            max_shots: None,
            max_checkpoint_bytes: None,
        }
    }

    pub fn validate(&self) -> Result<(), BackendError> {
        let invalid = [
            self.max_qubits.map(|v| v as u128),
            self.max_stabilizers.map(|v| v as u128),
            self.max_syndrome_events.map(|v| v as u128),
            self.max_graph_nodes.map(|v| v as u128),
            self.max_graph_edges.map(|v| v as u128),
            self.max_rounds.map(|v| v as u128),
            self.max_parallelism.map(|v| v as u128),
            self.max_memory_bytes.map(u128::from),
            self.max_shots.map(u128::from),
            self.max_checkpoint_bytes.map(u128::from),
        ];

        if invalid.iter().flatten().any(|value| *value == 0) {
            return Err(BackendError::InvalidResourceLimit(
                "backend-specific limits must be greater than zero",
            ));
        }

        Ok(())
    }

    fn check_usize(
        &self,
        resource: LimitKind,
        requested: usize,
        limit: Option<usize>,
    ) -> Result<(), BackendError> {
        if let Some(maximum) = limit {
            if requested > maximum {
                return Err(BackendError::CapacityExceeded {
                    resource,
                    requested: requested as u128,
                    maximum: maximum as u128,
                });
            }
        }

        Ok(())
    }

    fn check_u64(
        &self,
        resource: LimitKind,
        requested: u64,
        limit: Option<u64>,
    ) -> Result<(), BackendError> {
        if let Some(maximum) = limit {
            if requested > maximum {
                return Err(BackendError::CapacityExceeded {
                    resource,
                    requested: requested as u128,
                    maximum: maximum as u128,
                });
            }
        }

        Ok(())
    }

    pub fn validate_workload(
        &self,
        workload: &QecWorkload,
    ) -> Result<(), BackendError> {
        self.check_usize(
            LimitKind::Qubits,
            workload.qubits,
            self.max_qubits,
        )?;

        self.check_usize(
            LimitKind::Stabilizers,
            workload.stabilizers,
            self.max_stabilizers,
        )?;

        self.check_usize(
            LimitKind::SyndromeEvents,
            workload.syndrome_events,
            self.max_syndrome_events,
        )?;

        self.check_usize(
            LimitKind::GraphNodes,
            workload.graph_nodes,
            self.max_graph_nodes,
        )?;

        self.check_usize(
            LimitKind::GraphEdges,
            workload.graph_edges,
            self.max_graph_edges,
        )?;

        self.check_usize(
            LimitKind::MeasurementRounds,
            workload.rounds,
            self.max_rounds,
        )?;

        self.check_usize(
            LimitKind::Parallelism,
            workload.parallelism,
            self.max_parallelism,
        )?;

        self.check_u64(
            LimitKind::MemoryBytes,
            workload.memory_bytes,
            self.max_memory_bytes,
        )?;

        self.check_u64(
            LimitKind::QpuShots,
            workload.shots as u64,
            self.max_shots,
        )?;

        Ok(())
    }
}

// ============================================================================
// Backend topology
// ============================================================================

/// Logical topology visible to the QEC backend.
///
/// It deliberately does not model credentials, network addresses, vendor
/// APIs, or device sessions.
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
        if a >= self.qubit_count || b >= self.qubit_count {
            return false;
        }

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
                    "topology contains an invalid qubit index",
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

// ============================================================================
// Backend metadata
// ============================================================================

/// Stable backend identity.
///
/// The metadata must never contain credentials, tokens, secrets, private keys,
/// or raw device authentication material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendMetadata {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub version: String,
    pub kind: BackendKind,
    pub status: BackendStatus,
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

// ============================================================================
// QEC workload
// ============================================================================

/// Declarative description of the resources and capabilities required by a
/// QEC operation.
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
    pub shots: u64,

    pub operations: BTreeSet<String>,

    pub requires_determinism: bool,
    pub requires_qpu: bool,
    pub requires_calibration: bool,

    pub streaming: bool,
    pub partitionable: bool,
    pub distributable: bool,
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
    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.operations
            .insert(normalize_name(&operation.into()));

        self
    }

    pub fn operations<I, S>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for operation in operations {
            self.operations
                .insert(normalize_name(&operation.into()));
        }

        self
    }

    pub fn requires_qpu(mut self) -> Self {
        self.requires_qpu = true;
        self
    }

    pub fn requires_determinism(mut self) -> Self {
        self.requires_determinism = true;
        self
    }

    pub fn requires_calibration(mut self) -> Self {
        self.requires_calibration = true;
        self
    }

    pub fn streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    pub fn partitionable(mut self) -> Self {
        self.partitionable = true;
        self
    }

    pub fn distributable(mut self) -> Self {
        self.distributable = true;
        self
    }

    pub fn estimated_memory(mut self, bytes: u64) -> Self {
        self.memory_bytes = bytes;
        self
    }

    pub fn validate(&self) -> Result<(), BackendError> {
        if self.qubits == 0 {
            return Err(BackendError::InvalidWorkload(
                "QEC workload must contain at least one qubit",
            ));
        }

        if self.parallelism == 0 {
            return Err(BackendError::InvalidWorkload(
                "parallelism must be greater than zero",
            ));
        }

        if self.shots == 0 {
            return Err(BackendError::InvalidWorkload(
                "shots must be greater than zero",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Backend preflight
// ============================================================================

/// Immutable result of backend validation.
///
/// This object is intentionally separate from runtime resource accounting.
/// `ResourceManager` remains responsible for actual reservations/consumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendPreflight {
    pub backend_id: String,
    pub backend_kind: BackendKind,
    pub workload: QecWorkload,
    pub deterministic: bool,
    pub qpu_execution: bool,
    pub estimated_memory_bytes: u64,
    pub approved: bool,
}

impl BackendPreflight {
    pub const fn approved(&self) -> bool {
        self.approved
    }
}

// ============================================================================
// Backend execution context
// ============================================================================

/// Execution context supplied to backend implementations.
///
/// The context carries the canonical resource policy and cancellation token
/// without giving the backend implicit authority over unrelated application
/// state.
pub struct BackendExecutionContext {
    limits: QecLimits,
    cancellation: Arc<dyn CancellationToken>,
    started_at: Instant,
}

impl fmt::Debug for BackendExecutionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendExecutionContext")
            .field("limits", &self.limits)
            .field("started_at", &self.started_at)
            .finish_non_exhaustive()
    }
}

impl BackendExecutionContext {
    pub fn new(
        limits: QecLimits,
        cancellation: Arc<dyn CancellationToken>,
    ) -> Result<Self, BackendError> {
        limits.validate().map_err(BackendError::from)?;

        Ok(Self {
            limits,
            cancellation,
            started_at: Instant::now(),
        })
    }

    pub fn limits(&self) -> &QecLimits {
        &self.limits
    }

    pub fn cancellation(&self) -> &dyn CancellationToken {
        self.cancellation.as_ref()
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn check_cancellation(&self) -> Result<(), BackendError> {
        if self.cancellation.is_cancelled() {
            return Err(BackendError::Cancelled);
        }

        Ok(())
    }

    pub fn check_time(&self) -> Result<(), BackendError> {
        let elapsed = self.started_at.elapsed();

        let limit = Duration::from_nanos(
            self.limits.max_decoder_time_ns,
        );

        if elapsed > limit {
            return Err(BackendError::TimeLimitExceeded {
                elapsed,
                limit,
            });
        }

        Ok(())
    }

    pub fn check_execution_state(&self) -> Result<(), BackendError> {
        self.check_cancellation()?;
        self.check_time()
    }
}

// ============================================================================
// Backend trait
// ============================================================================

/// Core QEC backend interface.
///
/// The backend does not receive raw application credentials or implicitly
/// acquire QPU authority. QPU adapters should implement the hardware-specific
/// portion outside this abstraction.
pub trait QecBackend: Send + Sync {
    /// Stable backend metadata.
    fn metadata(&self) -> &BackendMetadata;

    /// Backend capabilities.
    fn capabilities(&self) -> &BackendCapabilities;

    /// Physical/backend capacity.
    fn resource_limits(&self) -> &BackendResourceLimits;

    /// Optional logical topology.
    fn topology(&self) -> Option<&BackendTopology> {
        None
    }

    /// Validate and preflight a workload.
    fn preflight(
        &self,
        workload: &QecWorkload,
        limits: &QecLimits,
        determinism: DeterminismPolicy,
    ) -> QecResult<BackendPreflight> {
        workload
            .validate()
            .map_err(BackendError::into_qec_error)?;

        limits
            .validate()
            .map_err(BackendError::into_qec_error)?;

        if !self.metadata().status.is_usable() {
            return Err(QecError::unsupported(
                "backend_status",
                format!(
                    "backend `{}` is not currently usable",
                    self.metadata().id
                ),
            ));
        }

        self.capabilities()
            .validate_workload(workload, determinism)
            .map_err(BackendError::into_qec_error)?;

        self.resource_limits()
            .validate_workload(workload)
            .map_err(BackendError::into_qec_error)?;

        validate_against_qec_limits(workload, limits)?;

        if let Some(topology) = self.topology() {
            topology
                .validate()
                .map_err(BackendError::into_qec_error)?;

            if workload.qubits > topology.qubit_count {
                return Err(QecError::resource_limit(
                    ResourceKind::Qubits,
                    workload.qubits as u128,
                    0,
                    topology.qubit_count as u128,
                    "workload exceeds backend topology",
                ));
            }
        }

        let deterministic = match determinism {
            DeterminismPolicy::Allow => false,
            DeterminismPolicy::Require | DeterminismPolicy::Strict => true,
        };

        Ok(BackendPreflight {
            backend_id: self.metadata().id.clone(),
            backend_kind: self.metadata().kind,
            workload: workload.clone(),
            deterministic,
            qpu_execution: workload.requires_qpu,
            estimated_memory_bytes: workload.memory_bytes,
            approved: true,
        })
    }

    /// Execute a preflighted workload.
    ///
    /// Concrete decoders should normally live in `decoder.rs`, `mwpm.rs`,
    /// `union_find.rs`, or another specialized module. This trait provides
    /// the backend boundary for orchestration.
    fn execute(
        &self,
        workload: &QecWorkload,
        context: &BackendExecutionContext,
    ) -> QecResult<BackendExecutionResult>;

    /// Whether the backend can execute this workload without execution.
    fn supports(
        &self,
        workload: &QecWorkload,
        limits: &QecLimits,
        determinism: DeterminismPolicy,
    ) -> bool {
        self.preflight(workload, limits, determinism)
            .is_ok()
    }
}

// ============================================================================
// Execution result
// ============================================================================

/// Backend-level execution result.
///
/// Decoder-specific results should wrap or extend this rather than replacing
/// it with unrelated backend status types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendExecutionResult {
    pub backend_id: String,
    pub backend_kind: BackendKind,

    pub deterministic: bool,

    pub shots: u64,
    pub rounds: usize,

    pub elapsed: Duration,
    pub estimated_memory_bytes: u64,

    pub cancelled: bool,
}

impl BackendExecutionResult {
    pub fn completed(
        backend: &dyn QecBackend,
        workload: &QecWorkload,
        deterministic: bool,
        elapsed: Duration,
    ) -> Self {
        Self {
            backend_id: backend.metadata().id.clone(),
            backend_kind: backend.metadata().kind,
            deterministic,
            shots: workload.shots,
            rounds: workload.rounds,
            elapsed,
            estimated_memory_bytes: workload.memory_bytes,
            cancelled: false,
        }
    }
}

// ============================================================================
// Capability validation
// ============================================================================

impl BackendCapabilities {
    pub fn validate_workload(
        &self,
        workload: &QecWorkload,
        determinism: DeterminismPolicy,
    ) -> Result<(), BackendError> {
        if !self.qec_execution {
            return Err(BackendError::Unsupported(
                "backend does not support QEC execution",
            ));
        }

        if workload.requires_qpu && !self.physical_qpu {
            return Err(BackendError::Unsupported(
                "workload requires physical QPU execution",
            ));
        }

        if workload.requires_calibration && !self.calibration {
            return Err(BackendError::Unsupported(
                "workload requires calibration data",
            ));
        }

        if workload.streaming && !self.streaming {
            return Err(BackendError::Unsupported(
                "workload requires streaming support",
            ));
        }

        if workload.partitionable && !self.partitioning {
            return Err(BackendError::Unsupported(
                "workload requires partitioning support",
            ));
        }

        if workload.distributable && !self.distributed {
            return Err(BackendError::Unsupported(
                "workload requires distributed execution",
            ));
        }

        if workload.cancellable && !self.cancellation {
            return Err(BackendError::Unsupported(
                "workload requires cancellation support",
            ));
        }

        match determinism {
            DeterminismPolicy::Allow => {}

            DeterminismPolicy::Require
                if !self.deterministic_execution =>
            {
                return Err(BackendError::Unsupported(
                    "deterministic execution is required",
                ));
            }

            DeterminismPolicy::Strict
                if !self.deterministic_execution =>
            {
                return Err(BackendError::Unsupported(
                    "strict deterministic execution is required",
                ));
            }

            _ => {}
        }

        for operation in &workload.operations {
            if !self.supports_operation(operation) {
                return Err(BackendError::UnsupportedOperation(
                    operation.clone(),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Canonical QecLimits integration
// ============================================================================

fn validate_against_qec_limits(
    workload: &QecWorkload,
    limits: &QecLimits,
) -> QecResult<()> {
    check_usize_limit(
        ResourceKind::Qubits,
        workload.qubits,
        limits.max_qubits,
    )?;

    check_usize_limit(
        ResourceKind::Stabilizers,
        workload.stabilizers,
        limits.max_stabilizers,
    )?;

    check_usize_limit(
        ResourceKind::SyndromeEvents,
        workload.syndrome_events,
        limits.max_syndrome_events,
    )?;

    check_usize_limit(
        ResourceKind::GraphNodes,
        workload.graph_nodes,
        limits.max_graph_nodes,
    )?;

    check_usize_limit(
        ResourceKind::GraphEdges,
        workload.graph_edges,
        limits.max_graph_edges,
    )?;

    check_usize_limit(
        ResourceKind::MeasurementRounds,
        workload.rounds,
        limits.max_rounds,
    )?;

    check_usize_limit(
        ResourceKind::Parallelism,
        workload.parallelism,
        limits.max_parallelism,
    )?;

    check_u64_limit(
        ResourceKind::MemoryBytes,
        workload.memory_bytes,
        limits.max_memory_bytes,
    )?;

    check_u64_limit(
        ResourceKind::QpuShots,
        workload.shots,
        limits.max_qpu_shots,
    )?;

    Ok(())
}

fn check_usize_limit(
    resource: ResourceKind,
    requested: usize,
    maximum: usize,
) -> QecResult<()> {
    if requested > maximum {
        return Err(QecError::resource_limit(
            resource,
            requested as u128,
            0,
            maximum as u128,
            "backend workload exceeds canonical QEC limits",
        ));
    }

    Ok(())
}

fn check_u64_limit(
    resource: ResourceKind,
    requested: u64,
    maximum: u64,
) -> QecResult<()> {
    if requested > maximum {
        return Err(QecError::resource_limit(
            resource,
            requested as u128,
            0,
            maximum as u128,
            "backend workload exceeds canonical QEC limits",
        ));
    }

    Ok(())
}

// ============================================================================
// Backend errors
// ============================================================================

/// Backend-local diagnostic error.
///
/// Public/high-level backend APIs convert this into `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    InvalidBackendId,

    InvalidWorkload(&'static str),

    InvalidResourceLimit(&'static str),

    InvalidTopology(&'static str),

    InvalidQubit {
        qubit: usize,
        qubit_count: usize,
    },

    ZeroQubits,

    Unsupported(&'static str),

    UnsupportedOperation(String),

    CapacityExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    Cancelled,

    TimeLimitExceeded {
        elapsed: Duration,
        limit: Duration,
    },

    ResourcePolicy(LimitError),

    ExecutionFailed(String),

    Internal(String),
}

impl BackendError {
    pub fn into_qec_error(self) -> QecError {
        match self {
            Self::InvalidBackendId => QecError::invalid_input(
                "backend identifier must not be empty",
            ),

            Self::InvalidWorkload(message) => {
                QecError::invalid_input(message)
            }

            Self::InvalidResourceLimit(message) => {
                QecError::invalid_input(message)
            }

            Self::InvalidTopology(message) => {
                QecError::invalid_topology(message)
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => QecError::invalid_topology(format!(
                "qubit index {qubit} is outside topology size {qubit_count}"
            )),

            Self::ZeroQubits => {
                QecError::invalid_topology("backend contains zero qubits")
            }

            Self::Unsupported(message) => {
                QecError::unsupported("backend_capability", message)
            }

            Self::UnsupportedOperation(operation) => {
                QecError::unsupported(
                    "native_operation",
                    format!(
                        "backend does not support operation `{operation}`"
                    ),
                )
            }

            Self::CapacityExceeded {
                resource,
                requested,
                maximum,
            } => QecError::resource_limit(
                resource_to_qec_kind(resource),
                requested,
                0,
                maximum,
                "backend capacity exceeded",
            ),

            Self::Cancelled => {
                QecError::cancelled("backend execution cancelled")
            }

            Self::TimeLimitExceeded { elapsed, limit } => {
                QecError::time_limit(
                    duration_to_nanos(elapsed),
                    duration_to_nanos(limit),
                    "backend execution exceeded the configured time limit",
                )
            }

            Self::ResourcePolicy(error) => {
                QecError::invalid_input(error.to_string())
            }

            Self::ExecutionFailed(message) => {
                QecError::decoder_failure(
                    DecoderKind::Custom,
                    message,
                )
            }

            Self::Internal(message) => {
                QecError::invariant("backend_internal_state", message)
            }
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBackendId => {
                f.write_str("invalid backend identifier")
            }

            Self::InvalidWorkload(message) => {
                write!(f, "invalid workload: {message}")
            }

            Self::InvalidResourceLimit(message) => {
                write!(f, "invalid backend resource limit: {message}")
            }

            Self::InvalidTopology(message) => {
                write!(f, "invalid backend topology: {message}")
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => write!(
                f,
                "invalid qubit {qubit}; backend contains {qubit_count} qubits"
            ),

            Self::ZeroQubits => {
                f.write_str("backend contains zero qubits")
            }

            Self::Unsupported(message) => {
                write!(f, "unsupported backend capability: {message}")
            }

            Self::UnsupportedOperation(operation) => {
                write!(
                    f,
                    "unsupported backend operation: {operation}"
                )
            }

            Self::CapacityExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "backend capacity exceeded for {resource}: \
                 requested {requested}, maximum {maximum}"
            ),

            Self::Cancelled => f.write_str("backend execution cancelled"),

            Self::TimeLimitExceeded { elapsed, limit } => write!(
                f,
                "backend time limit exceeded: elapsed {:?}, limit {:?}",
                elapsed,
                limit
            ),

            Self::ResourcePolicy(error) => {
                write!(f, "invalid QEC resource policy: {error}")
            }

            Self::ExecutionFailed(message) => {
                write!(f, "backend execution failed: {message}")
            }

            Self::Internal(message) => {
                write!(f, "backend internal error: {message}")
            }
        }
    }
}

impl std::error::Error for BackendError {}

impl From<LimitError> for BackendError {
    fn from(value: LimitError) -> Self {
        Self::ResourcePolicy(value)
    }
}

// ============================================================================
// Utility functions
// ============================================================================

fn normalize_name(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .replace([' ', '-'], "_")
}

fn duration_to_nanos(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn resource_to_qec_kind(resource: LimitKind) -> ResourceKind {
    match resource {
        LimitKind::CodeDistance => ResourceKind::CodeDistance,
        LimitKind::Qubits => ResourceKind::Qubits,
        LimitKind::Stabilizers => ResourceKind::Stabilizers,
        LimitKind::SyndromeEvents => ResourceKind::SyndromeEvents,
        LimitKind::MeasurementRounds => ResourceKind::MeasurementRounds,
        LimitKind::GraphNodes => ResourceKind::GraphNodes,
        LimitKind::GraphEdges => ResourceKind::GraphEdges,
        LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
        LimitKind::DecoderTimeNs => ResourceKind::Custom,
        LimitKind::Parallelism => ResourceKind::Parallelism,
        LimitKind::CheckpointSizeBytes => ResourceKind::CheckpointSize,
        LimitKind::Partitions => ResourceKind::Partitions,
        LimitKind::StreamBufferEvents => ResourceKind::StreamBuffer,
        LimitKind::DecoderIterations => ResourceKind::DecoderIterations,
        LimitKind::StabilizerWeight => ResourceKind::Custom,
        LimitKind::LogicalOperatorWeight => ResourceKind::Custom,
        LimitKind::QubitsPerPartition => ResourceKind::Custom,
        LimitKind::QpuShots => ResourceKind::QpuShots,
        LimitKind::QpuCircuits => ResourceKind::QpuCircuits,
        LimitKind::VerificationOperations => ResourceKind::Custom,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBackend {
        metadata: BackendMetadata,
        capabilities: BackendCapabilities,
        resources: BackendResourceLimits,
        topology: BackendTopology,
    }

    impl TestBackend {
        fn new() -> Self {
            let metadata = BackendMetadata::new(
                "test.cpu",
                "Test CPU",
                "Zamani",
                "1.0",
                BackendKind::Cpu,
            )
            .expect("valid metadata");

            let topology =
                BackendTopology::new(1024).expect("valid topology");

            Self {
                metadata,
                capabilities: BackendCapabilities::default(),
                resources: BackendResourceLimits::default(),
                topology,
            }
        }
    }

    impl QecBackend for TestBackend {
        fn metadata(&self) -> &BackendMetadata {
            &self.metadata
        }

        fn capabilities(&self) -> &BackendCapabilities {
            &self.capabilities
        }

        fn resource_limits(&self) -> &BackendResourceLimits {
            &self.resources
        }

        fn topology(&self) -> Option<&BackendTopology> {
            Some(&self.topology)
        }

        fn execute(
            &self,
            workload: &QecWorkload,
            context: &BackendExecutionContext,
        ) -> QecResult<BackendExecutionResult> {
            context
                .check_execution_state()
                .map_err(BackendError::into_qec_error)?;

            Ok(BackendExecutionResult::completed(
                self,
                workload,
                true,
                context.elapsed(),
            ))
        }
    }

    #[test]
    fn workload_rejects_zero_parallelism() {
        let mut workload = QecWorkload::default();
        workload.parallelism = 0;

        assert!(workload.validate().is_err());
    }

    #[test]
    fn topology_rejects_self_loop() {
        let mut topology =
            BackendTopology::new(4).expect("valid topology");

        assert!(topology.add_edge(1, 1).is_err());
    }

    #[test]
    fn canonical_limits_are_enforced() {
        let backend = TestBackend::new();

        let mut limits = QecLimits::default();
        limits.max_qubits = 2;

        let mut workload = QecWorkload::default();
        workload.qubits = 3;

        let result =
            backend.preflight(
                &workload,
                &limits,
                DeterminismPolicy::Require,
            );

        assert!(result.is_err());
    }

    #[test]
    fn backend_capacity_can_be_stricter_than_qec_limits() {
        let mut backend = TestBackend::new();
        backend.resources.max_qubits = Some(2);

        let workload = QecWorkload {
            qubits: 3,
            ..QecWorkload::default()
        };

        let result = backend.preflight(
            &workload,
            &QecLimits::default(),
            DeterminismPolicy::Require,
        );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_preflight_is_explicit() {
        let backend = TestBackend::new();

        let workload =
            QecWorkload::default().requires_determinism();

        let result = backend
            .preflight(
                &workload,
                &QecLimits::default(),
                DeterminismPolicy::Strict,
            )
            .expect("preflight should succeed");

        assert!(result.deterministic);
    }

    #[test]
    fn execution_checks_cancellation_and_time() {
        let backend = TestBackend::new();

        let context = BackendExecutionContext::new(
            QecLimits::default(),
            Arc::new(NoCancellation),
        )
        .expect("valid context");

        let workload = QecWorkload::default();

        let result = backend
            .execute(&workload, &context)
            .expect("execution should succeed");

        assert_eq!(result.backend_kind, BackendKind::Cpu);
        assert!(!result.cancelled);
    }

    #[test]
    fn operation_names_are_normalized() {
        let capabilities = BackendCapabilities::default()
            .with_operation("CZ Gate");

        assert!(capabilities.supports_operation("cz_gate"));
        assert!(capabilities.supports_operation("CZ-GATE"));
    }
}