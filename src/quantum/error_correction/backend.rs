//! Zamani Quantum Error Correction — execution backend boundary.
//!
//! # Ownership
//!
//! `backend.rs` owns the abstraction between QEC orchestration and an
//! execution environment.
//!
//! It owns:
//!
//! - backend identity;
//! - backend kind;
//! - backend status;
//! - backend capabilities;
//! - physical/backend capacity;
//! - logical topology metadata;
//! - workload description;
//! - backend admission/preflight;
//! - backend execution context;
//! - backend execution result;
//! - backend-local diagnostics.
//!
//! It does NOT own:
//!
//! - canonical QEC resource policy;
//! - runtime resource accounting;
//! - memory allocation;
//! - cancellation state;
//! - authorization;
//! - decoder algorithms;
//! - QPU credentials;
//! - QPU network I/O;
//! - telemetry transport;
//! - checkpoint persistence;
//! - distributed coordination.
//!
//! # Integration contract
//!
//! ```text
//!                         QecConfig
//!                            │
//!                            ▼
//!                         QecLimits
//!                            │
//!                            ▼
//!                    Backend preflight
//!                            │
//!              ┌─────────────┼─────────────┐
//!              │             │             │
//!              ▼             ▼             ▼
//!            CPU/GPU     Simulator       QPU adapter
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                    backend execution
//!                            │
//!                            ▼
//!                 BackendExecutionResult
//!                            │
//!             ┌──────────────┼──────────────┐
//!             ▼              ▼              ▼
//!          decoder        metrics       resources
//! ```
//!
//! # Important architectural rules
//!
//! 1. `QecLimits` is the single canonical declarative QEC policy.
//! 2. Backend capacity may restrict execution further, but may not replace
//!    `QecLimits`.
//! 3. `resources.rs` owns runtime accounting.
//! 4. `memory.rs` owns memory reservation/enforcement.
//! 5. `cancellation.rs` owns cancellation.
//! 6. `capabilities.rs` owns authorization.
//! 7. A backend advertises capabilities; it does not grant authority.
//! 8. Physical QPU execution must occur through a dedicated QPU adapter.
//! 9. Credentials and authentication material must never enter this module.
//! 10. Backend admission must happen before execution.
//! 11. Expensive backend implementations must poll cancellation.
//! 12. Deterministic execution must be explicit.
//! 13. Backend-specific limits are capacity metadata, not a second QEC policy.
//! 14. Public failures cross the canonical `QecError` boundary.
//! 15. No backend may silently exceed declared capacity.
//! 16. Rust 1.97.1 compatible.
//!
//! # Dependency direction
//!
//! ```text
//! errors.rs ───────────────┐
//!                          │
//! limits.rs ───────────────┼──► backend.rs
//!                          │
//! cancellation.rs ─────────┘
//!
//! configuration.rs ──► backend preflight
//!
//! backend.rs ──► decoder.rs
//! backend.rs ──► simulation.rs
//! backend.rs ──► surface_coder.rs
//! backend.rs ──► qpu_adapter.rs
//! backend.rs ──► scheduler.rs
//! ```
//!
//! `backend.rs` must not depend on concrete decoder implementations.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::{LimitKind, QecLimits};

// ============================================================================
// Backend kind
// ============================================================================

/// Execution environment used by a QEC workload.
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
    /// Single-threaded or ordinary CPU execution.
    Cpu,

    /// Explicit parallel CPU execution.
    ParallelCpu,

    /// GPU-accelerated execution.
    Gpu,

    /// Hardware accelerator other than a general CPU/GPU.
    Accelerator,

    /// Distributed classical execution.
    Distributed,

    /// Classical QEC simulation.
    Simulator,

    /// Hardware/emulator execution that behaves as an execution backend
    /// without representing physical QPU submission.
    Emulator,

    /// Physical quantum processing unit.
    Qpu,

    /// Application-defined backend.
    Custom,
}

impl BackendKind {
    /// Returns true only for physical QPU execution.
    pub const fn is_physical_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns true for non-QPU execution environments.
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

    /// Returns true for backend types that may involve remote execution.
    pub const fn may_be_remote(self) -> bool {
        matches!(self, Self::Distributed | Self::Qpu)
    }

    /// Stable machine-readable backend identifier.
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

/// Operational state of a backend.
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
    /// Whether admission is permitted.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Whether the backend is permanently unavailable for the current
    /// backend instance.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Offline | Self::Unavailable)
    }
}

// ============================================================================
// Determinism
// ============================================================================

/// Determinism requirement for backend admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismPolicy {
    /// Determinism is not required.
    Allow,

    /// Deterministic execution is required if this backend advertises it.
    Require,

    /// Execution must be rejected unless the backend explicitly guarantees
    /// deterministic behavior.
    Strict,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self::Require
    }
}

// ============================================================================
// Backend capabilities
// ============================================================================

/// Capabilities advertised by a backend.
///
/// These are technical capabilities, not authorization grants.
///
/// Authorization remains the responsibility of `capabilities.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// General QEC execution.
    pub qec_execution: bool,

    /// Backend can generate syndrome data itself.
    pub syndrome_generation: bool,

    /// Backend can execute decoding workloads.
    pub decoding: bool,

    /// Backend supports statistical simulation.
    pub simulation: bool,

    /// Backend can maintain/apply a Pauli frame.
    pub pauli_frame: bool,

    /// Incremental streaming execution.
    pub streaming: bool,

    /// Partition-aware execution.
    pub partitioning: bool,

    /// Distributed execution.
    pub distributed: bool,

    /// Hardware acceleration.
    pub acceleration: bool,

    /// Checkpoint-aware execution.
    pub checkpointing: bool,

    /// Cooperative cancellation.
    pub cancellation: bool,

    /// Guaranteed deterministic execution.
    pub deterministic_execution: bool,

    /// Physical QPU execution.
    pub physical_qpu: bool,

    /// Calibration information.
    pub calibration: bool,

    /// Mid-circuit measurement.
    pub mid_circuit_measurement: bool,

    /// Qubit reset.
    pub reset: bool,

    /// Measurement.
    pub measurement: bool,

    /// Dynamic circuits.
    pub dynamic_circuits: bool,

    /// Classical control.
    pub classical_control: bool,

    /// Backend-native operation identifiers.
    ///
    /// Stored in canonical normalized form.
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
    /// Returns whether a native operation is supported.
    pub fn supports_operation(&self, operation: &str) -> bool {
        self.native_operations
            .contains(&normalize_name(operation))
    }

    /// Adds one native operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.native_operations
            .insert(normalize_name(&operation.into()));

        self
    }

    /// Adds multiple native operations.
    #[must_use]
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

    /// Validates whether this capability set can satisfy a workload.
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
// Backend capacity
// ============================================================================

/// Physical or implementation-specific capacity.
///
/// This is NOT the canonical QEC policy.
///
/// `QecLimits` remains authoritative for QEC admission. These values can
/// only impose additional backend-specific restrictions.
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
        Self::unlimited()
    }
}

impl BackendResourceLimits {
    /// Creates a capacity description with no backend-specific restriction.
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

    /// Validates the capacity description itself.
    pub fn validate(&self) -> Result<(), BackendError> {
        let values = [
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

        if values.iter().flatten().any(|value| *value == 0) {
            return Err(BackendError::InvalidResourceLimit(
                "backend capacity values must be greater than zero",
            ));
        }

        Ok(())
    }

    fn check_usize(
        &self,
        resource: LimitKind,
        requested: usize,
        maximum: Option<usize>,
    ) -> Result<(), BackendError> {
        if let Some(maximum) = maximum {
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
        maximum: Option<u64>,
    ) -> Result<(), BackendError> {
        if let Some(maximum) = maximum {
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

    /// Validates workload against physical/backend capacity.
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
            workload.shots,
            self.max_shots,
        )?;

        Ok(())
    }
}

// ============================================================================
// Backend topology
// ============================================================================

/// Logical backend topology.
///
/// This contains topology information only. It deliberately contains no
/// credentials, network endpoints, authentication material, or vendor
/// sessions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendTopology {
    pub qubit_count: usize,
    pub edges: BTreeSet<(usize, usize)>,
}

impl BackendTopology {
    /// Creates an empty topology with `qubit_count` qubits.
    pub fn new(qubit_count: usize) -> Result<Self, BackendError> {
        if qubit_count == 0 {
            return Err(BackendError::ZeroQubits);
        }

        Ok(Self {
            qubit_count,
            edges: BTreeSet::new(),
        })
    }

    /// Adds an undirected connection.
    pub fn add_edge(
        &mut self,
        a: usize,
        b: usize,
    ) -> Result<(), BackendError> {
        self.validate_qubit(a)?;
        self.validate_qubit(b)?;

        if a == b {
            return Err(BackendError::InvalidTopology(
                "self-loops are not valid backend connections",
            ));
        }

        let edge = canonical_edge(a, b);

        self.edges.insert(edge);

        Ok(())
    }

    /// Returns whether two qubits are directly connected.
    pub fn connected(&self, a: usize, b: usize) -> bool {
        if a >= self.qubit_count || b >= self.qubit_count {
            return false;
        }

        self.edges.contains(&canonical_edge(a, b))
    }

    /// Validates the complete topology.
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

    fn validate_qubit(&self, qubit: usize) -> Result<(), BackendError> {
        if qubit >= self.qubit_count {
            return Err(BackendError::InvalidQubit {
                qubit,
                qubit_count: self.qubit_count,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Backend metadata
// ============================================================================

/// Stable backend identity and public metadata.
///
/// This structure must never contain secrets.
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
    /// Creates backend metadata.
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

    /// Adds public, non-secret metadata.
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

/// Declarative description of work requested from a backend.
///
/// This structure describes requirements. It does not reserve resources.
///
/// Runtime resource reservation remains the responsibility of `resources.rs`
/// and `memory.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecWorkload {
    /// Physical/data qubits required.
    pub qubits: usize,

    /// Stabilizer generators required.
    pub stabilizers: usize,

    /// Syndrome/detection events required.
    pub syndrome_events: usize,

    /// Decoding graph nodes.
    pub graph_nodes: usize,

    /// Decoding graph edges.
    pub graph_edges: usize,

    /// Measurement rounds.
    pub rounds: usize,

    /// Requested execution parallelism.
    pub parallelism: usize,

    /// Estimated memory requirement.
    pub memory_bytes: u64,

    /// Number of shots.
    pub shots: u64,

    /// Required native operations.
    pub operations: BTreeSet<String>,

    /// Whether deterministic execution is required by the workload itself.
    pub requires_determinism: bool,

    /// Whether a physical QPU is required.
    pub requires_qpu: bool,

    /// Whether calibration data is required.
    pub requires_calibration: bool,

    /// Whether bounded streaming is required.
    pub streaming: bool,

    /// Whether the workload must support partitioning.
    pub partitionable: bool,

    /// Whether the workload must support distributed execution.
    pub distributable: bool,

    /// Whether the workload requires cooperative cancellation.
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
    /// Adds one required native operation.
    #[must_use]
    pub fn operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operations
            .insert(normalize_name(&operation.into()));

        self
    }

    /// Adds multiple required native operations.
    #[must_use]
    pub fn operations<I, S>(
        mut self,
        operations: I,
    ) -> Self
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

    /// Requires physical QPU execution.
    #[must_use]
    pub fn requiring_qpu(mut self) -> Self {
        self.requires_qpu = true;
        self
    }

    /// Requires deterministic execution.
    #[must_use]
    pub fn requiring_determinism(mut self) -> Self {
        self.requires_determinism = true;
        self
    }

    /// Requires calibration data.
    #[must_use]
    pub fn requiring_calibration(mut self) -> Self {
        self.requires_calibration = true;
        self
    }

    /// Requires streaming.
    #[must_use]
    pub fn requiring_streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    /// Requires partitioning.
    #[must_use]
    pub fn requiring_partitioning(mut self) -> Self {
        self.partitionable = true;
        self
    }

    /// Requires distributed execution.
    #[must_use]
    pub fn requiring_distribution(mut self) -> Self {
        self.distributable = true;
        self
    }

    /// Sets the estimated memory requirement.
    #[must_use]
    pub fn estimated_memory(
        mut self,
        bytes: u64,
    ) -> Self {
        self.memory_bytes = bytes;
        self
    }

    /// Validates the workload independently of backend policy.
    pub fn validate(&self) -> Result<(), BackendError> {
        if self.qubits == 0 {
            return Err(BackendError::InvalidWorkload(
                "workload must contain at least one qubit",
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

/// Immutable admission result.
///
/// A successful preflight does NOT mean resources have been reserved.
/// Resource reservation happens later through `resources.rs`/`memory.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendPreflight {
    pub backend_id: String,
    pub backend_kind: BackendKind,
    pub workload: QecWorkload,
    pub deterministic: bool,
    pub qpu_execution: bool,
    pub estimated_memory_bytes: u64,
}

impl BackendPreflight {
    /// Returns whether the workload was admitted.
    ///
    /// Existence of this object represents successful admission.
    pub const fn approved(&self) -> bool {
        true
    }
}

// ============================================================================
// Backend execution context
// ============================================================================

/// Context passed into an actual backend execution.
///
/// The context does not own resources. It exposes policy and cancellation
/// boundaries that concrete implementations must obey.
#[derive(Clone)]
pub struct BackendExecutionContext {
    limits: QecLimits,
    cancellation: CancellationToken,
    deterministic: bool,
    seed: Option<u64>,
    started_at: Instant,
}

impl fmt::Debug for BackendExecutionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendExecutionContext")
            .field("limits", &self.limits)
            .field("deterministic", &self.deterministic)
            .field("seed", &self.seed)
            .field("started_at", &self.started_at)
            .finish_non_exhaustive()
    }
}

impl BackendExecutionContext {
    /// Creates an execution context.
    pub fn new(
        limits: QecLimits,
        cancellation: CancellationToken,
        deterministic: bool,
        seed: Option<u64>,
    ) -> Result<Self, BackendError> {
        limits
            .validate()
            .map_err(BackendError::ResourcePolicy)?;

        if deterministic && seed.is_none() {
            return Err(BackendError::InvalidDeterminism(
                "deterministic execution requires an explicit seed",
            ));
        }

        Ok(Self {
            limits,
            cancellation,
            deterministic,
            seed,
            started_at: Instant::now(),
        })
    }

    /// Creates a context for ordinary non-random deterministic work.
    ///
    /// A seed is not required when the workload itself has no stochastic
    /// component.
    pub fn deterministic_without_seed(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> Result<Self, BackendError> {
        limits
            .validate()
            .map_err(BackendError::ResourcePolicy)?;

        Ok(Self {
            limits,
            cancellation,
            deterministic: true,
            seed: None,
            started_at: Instant::now(),
        })
    }

    /// Returns canonical QEC limits.
    pub fn limits(&self) -> &QecLimits {
        &self.limits
    }

    /// Returns the cancellation token.
    pub fn cancellation(&self) -> &CancellationToken {
        &self.cancellation
    }

    /// Returns whether deterministic execution is active.
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns the deterministic seed, if one exists.
    pub const fn seed(&self) -> Option<u64> {
        self.seed
    }

    /// Returns elapsed backend execution time.
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Polls cancellation.
    pub fn check_cancellation(&self) -> QecResult<()> {
        self.cancellation.check()
    }

    /// Checks the configured backend execution time.
    pub fn check_time(&self) -> QecResult<()> {
        let elapsed = self.elapsed();

        let limit =
            Duration::from_nanos(self.limits.max_decoder_time_ns);

        if elapsed > limit {
            return Err(QecError::TimeLimitExceeded {
                elapsed_nanos: duration_to_nanos(elapsed),
                limit_nanos: duration_to_nanos(limit),
                message:
                    "backend execution exceeded the configured QEC time limit"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Performs all mandatory cooperative execution checks.
    pub fn check_execution_state(&self) -> QecResult<()> {
        self.check_cancellation()?;
        self.check_time()
    }
}

// ============================================================================
// Backend execution result
// ============================================================================

/// Backend-level execution result.
///
/// Decoder-specific information belongs in `decoder_result.rs`.
/// This structure only reports execution-level facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendExecutionResult {
    pub backend_id: String,
    pub backend_kind: BackendKind,

    pub deterministic: bool,

    pub shots: u64,
    pub rounds: usize,

    pub elapsed: Duration,

    pub estimated_memory_bytes: u64,

    /// Number of logical backend operations completed.
    pub operations_completed: u64,

    /// Whether execution reached normal completion.
    pub completed: bool,
}

impl BackendExecutionResult {
    /// Creates a completed result.
    pub fn completed(
        backend: &dyn QecBackend,
        workload: &QecWorkload,
        deterministic: bool,
        elapsed: Duration,
        operations_completed: u64,
    ) -> Self {
        Self {
            backend_id: backend.metadata().id.clone(),
            backend_kind: backend.metadata().kind,
            deterministic,
            shots: workload.shots,
            rounds: workload.rounds,
            elapsed,
            estimated_memory_bytes: workload.memory_bytes,
            operations_completed,
            completed: true,
        }
    }
}

// ============================================================================
// Backend trait
// ============================================================================

/// Common execution contract for every QEC backend.
///
/// Concrete implementations may be:
///
/// - CPU;
/// - parallel CPU;
/// - GPU;
/// - accelerator;
/// - simulator;
/// - emulator;
/// - distributed backend;
/// - QPU adapter;
/// - custom backend.
///
/// The trait deliberately contains no decoder-specific API.
pub trait QecBackend: Send + Sync {
    /// Stable backend metadata.
    fn metadata(&self) -> &BackendMetadata;

    /// Technical backend capabilities.
    fn capabilities(&self) -> &BackendCapabilities;

    /// Backend-specific physical/implementation capacity.
    fn resource_limits(&self) -> &BackendResourceLimits;

    /// Optional logical topology.
    fn topology(&self) -> Option<&BackendTopology> {
        None
    }

    /// Performs complete backend admission.
    ///
    /// Admission order:
    ///
    /// ```text
    /// workload validation
    ///        ↓
    /// QecLimits validation
    ///        ↓
    /// backend status
    ///        ↓
    /// capability validation
    ///        ↓
    /// backend capacity
    ///        ↓
    /// canonical QEC limits
    ///        ↓
    /// topology
    ///        ↓
    /// BackendPreflight
    /// ```
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

        self.resource_limits()
            .validate()
            .map_err(BackendError::into_qec_error)?;

        if !self.metadata().status.is_usable() {
            return Err(QecError::BackendFailure {
                backend: self.metadata().id.clone(),
                message: format!(
                    "backend status `{}` is not usable",
                    backend_status_name(self.metadata().status)
                ),
            });
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
                return Err(QecError::ResourceLimitExceeded {
                    resource: ResourceKind::Qubits,
                    requested: workload.qubits as u128,
                    current: 0,
                    limit: topology.qubit_count as u128,
                    message:
                        "workload exceeds backend topology capacity"
                            .to_owned(),
                });
            }
        }

        let deterministic = match determinism {
            DeterminismPolicy::Allow => false,
            DeterminismPolicy::Require
            | DeterminismPolicy::Strict => true,
        };

        if workload.requires_determinism && !deterministic {
            return Err(QecError::UnsupportedConfiguration {
                feature: "deterministic_execution".to_owned(),
                message:
                    "workload explicitly requires deterministic execution"
                        .to_owned(),
            });
        }

        Ok(BackendPreflight {
            backend_id: self.metadata().id.clone(),
            backend_kind: self.metadata().kind,
            workload: workload.clone(),
            deterministic,
            qpu_execution: workload.requires_qpu,
            estimated_memory_bytes: workload.memory_bytes,
        })
    }

    /// Executes an admitted workload.
    ///
    /// Implementations MUST:
    ///
    /// - validate the supplied context before expensive work;
    /// - poll cancellation at bounded intervals;
    /// - respect `QecLimits`;
    /// - avoid uncontrolled allocations;
    /// - never perform unauthorized QPU access;
    /// - return a structured result;
    /// - never turn cancellation into success.
    fn execute(
        &self,
        workload: &QecWorkload,
        context: &BackendExecutionContext,
    ) -> QecResult<BackendExecutionResult>;

    /// Convenience capability/admission query.
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
// Canonical QecLimits integration
// ============================================================================

/// Validates workload against the canonical QEC resource policy.
///
/// This is deliberately implemented here only as an adapter to `QecLimits`.
/// `backend.rs` does not create another policy structure.
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
        return Err(QecError::ResourceLimitExceeded {
            resource,
            requested: requested as u128,
            current: 0,
            limit: maximum as u128,
            message:
                "backend workload exceeds canonical QEC limits"
                    .to_owned(),
        });
    }

    Ok(())
}

fn check_u64_limit(
    resource: ResourceKind,
    requested: u64,
    maximum: u64,
) -> QecResult<()> {
    if requested > maximum {
        return Err(QecError::ResourceLimitExceeded {
            resource,
            requested: requested as u128,
            current: 0,
            limit: maximum as u128,
            message:
                "backend workload exceeds canonical QEC limits"
                    .to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// Backend errors
// ============================================================================

/// Backend-local error.
///
/// This is an implementation diagnostic boundary. Public APIs convert it to
/// `QecError`.
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

    InvalidDeterminism(&'static str),

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

    ResourcePolicy(super::limits::LimitError),

    ExecutionFailed(String),

    Internal(String),
}

impl BackendError {
    /// Converts backend diagnostics to the canonical QEC error boundary.
    pub fn into_qec_error(self) -> QecError {
        match self {
            Self::InvalidBackendId => QecError::InvalidInput {
                message:
                    "backend identifier must not be empty".to_owned(),
            },

            Self::InvalidWorkload(message) => QecError::InvalidInput {
                message: message.to_owned(),
            },

            Self::InvalidResourceLimit(message) => {
                QecError::InvalidInput {
                    message: message.to_owned(),
                }
            }

            Self::InvalidTopology(message) => {
                QecError::InvalidTopology {
                    message: message.to_owned(),
                }
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => QecError::InvalidTopology {
                message: format!(
                    "qubit index {qubit} is outside topology size {qubit_count}"
                ),
            },

            Self::ZeroQubits => QecError::InvalidTopology {
                message:
                    "backend contains zero qubits".to_owned(),
            },

            Self::Unsupported(message) => {
                QecError::UnsupportedConfiguration {
                    feature: "backend_capability".to_owned(),
                    message: message.to_owned(),
                }
            }

            Self::UnsupportedOperation(operation) => {
                QecError::UnsupportedConfiguration {
                    feature: "native_operation".to_owned(),
                    message: format!(
                        "backend does not support operation `{operation}`"
                    ),
                }
            }

            Self::InvalidDeterminism(message) => {
                QecError::UnsupportedConfiguration {
                    feature: "deterministic_execution".to_owned(),
                    message: message.to_owned(),
                }
            }

            Self::CapacityExceeded {
                resource,
                requested,
                maximum,
            } => QecError::ResourceLimitExceeded {
                resource: resource_to_resource_kind(resource),
                requested,
                current: 0,
                limit: maximum,
                message:
                    "backend-specific capacity exceeded".to_owned(),
            },

            Self::Cancelled => QecError::CancellationRequested {
                message:
                    "backend execution cancelled".to_owned(),
            },

            Self::TimeLimitExceeded { elapsed, limit } => {
                QecError::TimeLimitExceeded {
                    elapsed_nanos: duration_to_nanos(elapsed),
                    limit_nanos: duration_to_nanos(limit),
                    message:
                        "backend execution exceeded the configured time limit"
                            .to_owned(),
                }
            }

            Self::ResourcePolicy(error) => {
                QecError::InvalidInput {
                    message: error.to_string(),
                }
            }

            Self::ExecutionFailed(message) => {
                QecError::BackendFailure {
                    backend: "unknown".to_owned(),
                    message,
                }
            }

            Self::Internal(message) => {
                QecError::InternalInvariantViolation {
                    invariant: "backend_internal_state".to_owned(),
                    message,
                }
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
                write!(
                    f,
                    "invalid backend resource limit: {message}"
                )
            }

            Self::InvalidTopology(message) => {
                write!(f, "invalid backend topology: {message}")
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "invalid qubit {qubit}; backend contains \
                     {qubit_count} qubits"
                )
            }

            Self::ZeroQubits => {
                f.write_str("backend contains zero qubits")
            }

            Self::Unsupported(message) => {
                write!(
                    f,
                    "unsupported backend capability: {message}"
                )
            }

            Self::UnsupportedOperation(operation) => {
                write!(
                    f,
                    "unsupported backend operation: {operation}"
                )
            }

            Self::InvalidDeterminism(message) => {
                write!(
                    f,
                    "invalid determinism configuration: {message}"
                )
            }

            Self::CapacityExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "backend capacity exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::Cancelled => {
                f.write_str("backend execution cancelled")
            }

            Self::TimeLimitExceeded { elapsed, limit } => {
                write!(
                    f,
                    "backend time limit exceeded: \
                     elapsed {:?}, limit {:?}",
                    elapsed,
                    limit
                )
            }

            Self::ResourcePolicy(error) => {
                write!(
                    f,
                    "invalid QEC resource policy: {error}"
                )
            }

            Self::ExecutionFailed(message) => {
                write!(
                    f,
                    "backend execution failed: {message}"
                )
            }

            Self::Internal(message) => {
                write!(
                    f,
                    "backend internal error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for BackendError {}

impl From<super::limits::LimitError> for BackendError {
    fn from(
        value: super::limits::LimitError,
    ) -> Self {
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

fn canonical_edge(
    a: usize,
    b: usize,
) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn duration_to_nanos(duration: Duration) -> u64 {
    match u64::try_from(duration.as_nanos()) {
        Ok(value) => value,
        Err(_) => u64::MAX,
    }
}

fn resource_to_resource_kind(
    resource: LimitKind,
) -> ResourceKind {
    match resource {
        LimitKind::CodeDistance => ResourceKind::CodeDistance,
        LimitKind::Qubits => ResourceKind::Qubits,
        LimitKind::Stabilizers => ResourceKind::Stabilizers,
        LimitKind::SyndromeEvents => ResourceKind::SyndromeEvents,
        LimitKind::MeasurementRounds => {
            ResourceKind::MeasurementRounds
        }
        LimitKind::GraphNodes => ResourceKind::GraphNodes,
        LimitKind::GraphEdges => ResourceKind::GraphEdges,
        LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
        LimitKind::DecoderTimeNs => ResourceKind::Time,
        LimitKind::Parallelism => ResourceKind::Parallelism,
        LimitKind::CheckpointSizeBytes => {
            ResourceKind::CheckpointSize
        }
        LimitKind::Partitions => ResourceKind::Partitions,
        LimitKind::StreamBufferEvents => {
            ResourceKind::StreamBuffer
        }
        LimitKind::DecoderIterations => {
            ResourceKind::DecoderIterations
        }
        LimitKind::StabilizerWeight => {
            ResourceKind::StabilizerWeight
        }
        LimitKind::LogicalOperatorWeight => {
            ResourceKind::LogicalWeight
        }
        LimitKind::QubitsPerPartition => {
            ResourceKind::Qubits
        }
        LimitKind::QpuShots => ResourceKind::QpuShots,
        LimitKind::QpuCircuits => ResourceKind::QpuCircuits,
        LimitKind::VerificationOperations => {
            ResourceKind::Operations
        }
    }
}

fn backend_status_name(
    status: BackendStatus,
) -> &'static str {
    match status {
        BackendStatus::Available => "available",
        BackendStatus::Busy => "busy",
        BackendStatus::Degraded => "degraded",
        BackendStatus::Maintenance => "maintenance",
        BackendStatus::Offline => "offline",
        BackendStatus::Unavailable => "unavailable",
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cancellation::CancellationSource;

    struct TestBackend {
        metadata: BackendMetadata,
        capabilities: BackendCapabilities,
        limits: BackendResourceLimits,
        topology: Option<BackendTopology>,
    }

    impl TestBackend {
        fn cpu() -> Self {
            Self {
                metadata: BackendMetadata::new(
                    "test.cpu",
                    "Test CPU",
                    "Zamani",
                    "1",
                    BackendKind::Cpu,
                )
                .expect("valid backend metadata"),

                capabilities: BackendCapabilities::default()
                    .with_operation("decode")
                    .with_operation("syndrome"),

                limits: BackendResourceLimits::unlimited(),

                topology: None,
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
            &self.limits
        }

        fn topology(&self) -> Option<&BackendTopology> {
            self.topology.as_ref()
        }

        fn execute(
            &self,
            workload: &QecWorkload,
            context: &BackendExecutionContext,
        ) -> QecResult<BackendExecutionResult> {
            context.check_execution_state()?;

            Ok(BackendExecutionResult::completed(
                self,
                workload,
                context.deterministic(),
                context.elapsed(),
                1,
            ))
        }
    }

    #[test]
    fn backend_kind_classification_is_stable() {
        assert!(BackendKind::Qpu.is_physical_qpu());
        assert!(!BackendKind::Cpu.is_physical_qpu());

        assert!(BackendKind::Cpu.is_software());
        assert!(!BackendKind::Qpu.is_software());

        assert!(BackendKind::Distributed.may_be_remote());
        assert!(BackendKind::Qpu.may_be_remote());
    }

    #[test]
    fn backend_metadata_rejects_empty_id() {
        let result = BackendMetadata::new(
            "",
            "Test",
            "Zamani",
            "1",
            BackendKind::Cpu,
        );

        assert!(matches!(
            result,
            Err(BackendError::InvalidBackendId)
        ));
    }

    #[test]
    fn topology_canonicalizes_edges() {
        let mut topology =
            BackendTopology::new(4).expect("valid topology");

        topology
            .add_edge(3, 1)
            .expect("valid edge");

        assert!(topology.connected(1, 3));
        assert_eq!(topology.edges.len(), 1);
    }

    #[test]
    fn topology_rejects_invalid_qubit() {
        let mut topology =
            BackendTopology::new(2).expect("valid topology");

        assert!(matches!(
            topology.add_edge(0, 2),
            Err(BackendError::InvalidQubit { .. })
        ));
    }

    #[test]
    fn workload_rejects_zero_qubits() {
        let workload = QecWorkload {
            qubits: 0,
            ..QecWorkload::default()
        };

        assert!(workload.validate().is_err());
    }

    #[test]
    fn workload_normalizes_operations() {
        let workload =
            QecWorkload::default()
                .operation(" Controlled-X ");

        assert!(workload
            .operations
            .contains("controlled_x"));
    }

    #[test]
    fn capabilities_reject_missing_operation() {
        let backend = TestBackend::cpu();

        let workload =
            QecWorkload::default()
                .operation("mwpm");

        let result = backend.preflight(
            &workload,
            &QecLimits::default(),
            DeterminismPolicy::Require,
        );

        assert!(result.is_err());
    }

    #[test]
    fn preflight_accepts_supported_workload() {
        let backend = TestBackend::cpu();

        let workload =
            QecWorkload::default()
                .operation("decode");

        let result = backend.preflight(
            &workload,
            &QecLimits::default(),
            DeterminismPolicy::Require,
        );

        assert!(result.is_ok());

        let preflight =
            result.expect("preflight should succeed");

        assert!(preflight.approved());
        assert_eq!(
            preflight.backend_kind,
            BackendKind::Cpu
        );
    }

    #[test]
    fn canonical_limits_are_enforced() {
        let backend = TestBackend::cpu();

        let mut limits = QecLimits::default();
        limits.max_qubits = 2;

        let workload = QecWorkload {
            qubits: 3,
            ..QecWorkload::default()
        };

        let result = backend.preflight(
            &workload,
            &limits,
            DeterminismPolicy::Require,
        );

        assert!(matches!(
            result,
            Err(QecError::ResourceLimitExceeded {
                resource: ResourceKind::Qubits,
                ..
            })
        ));
    }

    #[test]
    fn backend_capacity_is_checked_separately_from_qec_policy() {
        let backend = TestBackend {
            metadata: BackendMetadata::new(
                "limited.cpu",
                "Limited CPU",
                "Zamani",
                "1",
                BackendKind::Cpu,
            )
            .expect("valid metadata"),

            capabilities: BackendCapabilities::default(),

            limits: BackendResourceLimits {
                max_qubits: Some(2),
                ..BackendResourceLimits::unlimited()
            },

            topology: None,
        };

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
    fn cancellation_is_not_duplicated_by_backend() {
        let (source, token) =
            CancellationSource::new_pair();

        source.cancel();

        let context =
            BackendExecutionContext::new(
                QecLimits::default(),
                token,
                false,
                None,
            )
            .expect("valid execution context");

        assert!(matches!(
            context.check_cancellation(),
            Err(QecError::CancellationRequested { .. })
        ));
    }

    #[test]
    fn deterministic_context_requires_seed_for_stochastic_execution() {
        let result =
            BackendExecutionContext::new(
                QecLimits::default(),
                CancellationSource::new().token(),
                true,
                None,
            );

        assert!(matches!(
            result,
            Err(BackendError::InvalidDeterminism(_))
        ));
    }

    #[test]
    fn deterministic_context_accepts_seed() {
        let context =
            BackendExecutionContext::new(
                QecLimits::default(),
                CancellationSource::new().token(),
                true,
                Some(42),
            )
            .expect("valid deterministic context");

        assert!(context.deterministic());
        assert_eq!(context.seed(), Some(42));
    }

    #[test]
    fn backend_execution_checks_cancellation_before_work() {
        let backend = TestBackend::cpu();

        let (source, token) =
            CancellationSource::new_pair();

        source.cancel();

        let context =
            BackendExecutionContext::new(
                QecLimits::default(),
                token,
                false,
                None,
            )
            .expect("valid context");

        let workload = QecWorkload::default();

        let result =
            backend.execute(&workload, &context);

        assert!(matches!(
            result,
            Err(QecError::CancellationRequested { .. })
        ));
    }

    #[test]
    fn backend_result_contains_execution_identity() {
        let backend = TestBackend::cpu();

        let workload =
            QecWorkload::default()
                .operation("decode");

        let context =
            BackendExecutionContext::new(
                QecLimits::default(),
                CancellationSource::new().token(),
                false,
                None,
            )
            .expect("valid context");

        let result =
            backend
                .execute(&workload, &context)
                .expect("execution should succeed");

        assert_eq!(result.backend_id, "test.cpu");
        assert_eq!(result.backend_kind, BackendKind::Cpu);
        assert!(result.completed);
    }

    #[test]
    fn topology_capacity_is_enforced() {
        let mut topology =
            BackendTopology::new(2)
                .expect("valid topology");

        topology
            .add_edge(0, 1)
            .expect("valid edge");

        let backend = TestBackend {
            metadata: BackendMetadata::new(
                "topology.cpu",
                "Topology CPU",
                "Zamani",
                "1",
                BackendKind::Cpu,
            )
            .expect("valid metadata"),

            capabilities: BackendCapabilities::default(),

            limits: BackendResourceLimits::unlimited(),

            topology: Some(topology),
        };

        let workload = QecWorkload {
            qubits: 3,
            ..QecWorkload::default()
        };

        let result = backend.preflight(
            &workload,
            &QecLimits::default(),
            DeterminismPolicy::Require,
        );

        assert!(matches!(
            result,
            Err(QecError::ResourceLimitExceeded {
                resource: ResourceKind::Qubits,
                ..
            })
        ));
    }
}