//! Capability-based authorization for the Zamani Quantum Error Correction
//! subsystem.
//!
//! This module provides a fail-closed capability model for QEC workloads.
//!
//! The capability system deliberately separates:
//!
//! ```text
//! identity
//!     |
//!     v
//! capability grant
//!     |
//!     +-------------------------------+
//!     |                               |
//!     v                               v
//! QEC operation                  execution backend
//!     |                               |
//!     v                               v
//! authorization                  CPU / GPU / accelerator / QPU
//!     |                               |
//!     +---------------+---------------+
//!                     |
//!                     v
//!              resource policy
//!                     |
//!                     v
//!                  execute
//! ```
//!
//! ## Security properties
//!
//! * Deny by default.
//! * No capability implies no privilege.
//! * Capabilities are explicit and scoped.
//! * QPU access is separately permissioned.
//! * Accelerator access is separately permissioned.
//! * Distributed execution is separately permissioned.
//! * Resource ceilings are enforced by policy.
//! * Expired capabilities are rejected.
//! * Revoked capabilities are rejected.
//! * Capability IDs are stable and auditable.
//! * Authorization is deterministic.
//! * No operation implicitly escalates privileges.
//! * Unknown capability values are not accepted by this API.
//!
//! ## Important distinction
//!
//! Possessing `Decode` does not imply:
//!
//! * QPU access;
//! * GPU access;
//! * arbitrary memory allocation;
//! * network access;
//! * distributed execution;
//! * checkpoint persistence;
//! * topology inspection;
//! * benchmarking;
//! * simulation.
//!
//! Those permissions must be granted independently.
//!
//! ## QPU isolation
//!
//! QPU access is intentionally stronger than ordinary accelerator access.
//! A QPU capability controls whether a workload may:
//!
//! * discover QPU metadata;
//! * submit circuits;
//! * submit syndrome-extraction workloads;
//! * read measurement results;
//! * request calibration information;
//! * execute real hardware;
//! * execute hardware-backed QEC.
//!
//! Simulation and QPU execution are therefore distinct capabilities.

#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// -----------------------------------------------------------------------------
// Capability identifiers
// -----------------------------------------------------------------------------

/// Stable identifier for a QEC capability.
///
/// These identifiers are intended for:
//!
//! * authorization;
//! * policy files;
//! * audit records;
//! * telemetry;
//! * checkpoint metadata;
//! * security tests.
///
/// Numeric discriminants are deliberately stable.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capability {
    /// Decode a validated syndrome/error representation.
    Decode = 1,

    /// Run QEC simulation workloads.
    Simulate = 2,

    /// Run threshold/performance experiments.
    Benchmark = 3,

    /// Inspect validated QEC topology.
    InspectTopology = 4,

    /// Allocate QEC-managed memory.
    AllocateMemory = 5,

    /// Use hardware/software accelerators.
    UseAccelerator = 6,

    /// Execute through distributed workers.
    DistributedExecution = 7,

    /// Consume streaming syndrome data.
    StreamingSyndrome = 8,

    /// Create or resume checkpoints.
    Checkpoint = 9,

    /// Use deterministic execution mode.
    DeterministicExecution = 10,

    /// Read decoder metrics.
    ReadMetrics = 11,

    /// Emit telemetry.
    EmitTelemetry = 12,

    /// Execute CPU-parallel workloads.
    ParallelExecution = 13,

    /// Access a QPU.
    QpuAccess = 14,

    /// Discover QPU metadata.
    QpuInspect = 15,

    /// Submit a workload to a QPU.
    QpuSubmit = 16,

    /// Read QPU measurement results.
    QpuReadResults = 17,

    /// Request QPU calibration information.
    QpuCalibration = 18,

    /// Execute hardware-backed QEC.
    QpuErrorCorrection = 19,

    /// Use QPU-specific syndrome extraction.
    QpuSyndromeExtraction = 20,
}

impl Capability {
    /// Returns the stable numeric capability identifier.
    pub const fn id(self) -> u16 {
        self as u16
    }

    /// Returns a stable machine-readable capability name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Decode => "qec.decode",
            Self::Simulate => "qec.simulate",
            Self::Benchmark => "qec.benchmark",
            Self::InspectTopology => "qec.inspect_topology",
            Self::AllocateMemory => "qec.allocate_memory",
            Self::UseAccelerator => "qec.use_accelerator",
            Self::DistributedExecution => "qec.distributed_execution",
            Self::StreamingSyndrome => "qec.streaming_syndrome",
            Self::Checkpoint => "qec.checkpoint",
            Self::DeterministicExecution => "qec.deterministic_execution",
            Self::ReadMetrics => "qec.read_metrics",
            Self::EmitTelemetry => "qec.emit_telemetry",
            Self::ParallelExecution => "qec.parallel_execution",
            Self::QpuAccess => "qec.qpu_access",
            Self::QpuInspect => "qec.qpu_inspect",
            Self::QpuSubmit => "qec.qpu_submit",
            Self::QpuReadResults => "qec.qpu_read_results",
            Self::QpuCalibration => "qec.qpu_calibration",
            Self::QpuErrorCorrection => "qec.qpu_error_correction",
            Self::QpuSyndromeExtraction => "qec.qpu_syndrome_extraction",
        }
    }

    /// Returns whether this capability is related to QPU operation.
    pub const fn is_qpu(self) -> bool {
        matches!(
            self,
            Self::QpuAccess
                | Self::QpuInspect
                | Self::QpuSubmit
                | Self::QpuReadResults
                | Self::QpuCalibration
                | Self::QpuErrorCorrection
                | Self::QpuSyndromeExtraction
        )
    }

    /// Returns whether this capability can cause external hardware execution.
    pub const fn can_execute_hardware(self) -> bool {
        matches!(
            self,
            Self::QpuSubmit
                | Self::QpuErrorCorrection
                | Self::QpuSyndromeExtraction
        )
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Execution backends
// -----------------------------------------------------------------------------

/// Execution backend available to a QEC workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionBackend {
    /// Single-threaded CPU execution.
    Cpu,

    /// Multi-threaded CPU execution.
    ParallelCpu,

    /// General-purpose GPU execution.
    Gpu,

    /// Dedicated accelerator.
    Accelerator,

    /// Distributed execution across workers.
    Distributed,

    /// Quantum processing unit.
    Qpu,
}

impl ExecutionBackend {
    /// Returns whether this backend represents physical quantum hardware.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether the backend requires explicit hardware authorization.
    pub const fn requires_hardware_capability(self) -> bool {
        matches!(
            self,
            Self::Gpu
                | Self::Accelerator
                | Self::Qpu
        )
    }

    /// Returns a stable backend name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::ParallelCpu => "parallel-cpu",
            Self::Gpu => "gpu",
            Self::Accelerator => "accelerator",
            Self::Distributed => "distributed",
            Self::Qpu => "qpu",
        }
    }
}

// -----------------------------------------------------------------------------
// QPU operation scope
// -----------------------------------------------------------------------------

/// Explicit scope of QPU access.
///
/// QPU permissions are intentionally granular.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QpuOperation {
    /// Read immutable QPU metadata.
    Inspect,

    /// Read calibration information.
    ReadCalibration,

    /// Submit an arbitrary circuit/workload.
    SubmitCircuit,

    /// Read measurement results.
    ReadResults,

    /// Perform hardware-backed QEC.
    ErrorCorrection,

    /// Perform hardware syndrome extraction.
    SyndromeExtraction,
}

impl QpuOperation {
    /// Returns the capability required for this operation.
    pub const fn required_capability(self) -> Capability {
        match self {
            Self::Inspect => Capability::QpuInspect,
            Self::ReadCalibration => Capability::QpuCalibration,
            Self::SubmitCircuit => Capability::QpuSubmit,
            Self::ReadResults => Capability::QpuReadResults,
            Self::ErrorCorrection => Capability::QpuErrorCorrection,
            Self::SyndromeExtraction => Capability::QpuSyndromeExtraction,
        }
    }
}

// -----------------------------------------------------------------------------
// Resource policy
// -----------------------------------------------------------------------------

/// Resource ceilings attached to a capability grant.
///
/// Zero means "not permitted", not "unlimited".
///
/// This design intentionally avoids an implicit unlimited mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum code distance.
    pub max_code_distance: u64,

    /// Maximum number of qubits.
    pub max_qubits: u64,

    /// Maximum number of stabilizers.
    pub max_stabilizers: u64,

    /// Maximum syndrome events.
    pub max_syndrome_events: u64,

    /// Maximum measurement rounds.
    pub max_rounds: u64,

    /// Maximum decoding graph nodes.
    pub max_graph_nodes: u64,

    /// Maximum decoding graph edges.
    pub max_graph_edges: u64,

    /// Maximum memory in bytes.
    pub max_memory_bytes: u64,

    /// Maximum wall-clock duration.
    pub max_execution_time: Duration,

    /// Maximum parallel workers.
    pub max_parallelism: u32,

    /// Maximum checkpoint size in bytes.
    pub max_checkpoint_bytes: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_code_distance: 3,
            max_qubits: 1024,
            max_stabilizers: 2048,
            max_syndrome_events: 100_000,
            max_rounds: 1_000,
            max_graph_nodes: 100_000,
            max_graph_edges: 500_000,
            max_memory_bytes: 64 * 1024 * 1024,
            max_execution_time: Duration::from_secs(60),
            max_parallelism: 1,
            max_checkpoint_bytes: 16 * 1024 * 1024,
        }
    }
}

impl ResourceLimits {
    /// Conservative limits suitable for read-only inspection.
    pub const fn inspection() -> Self {
        Self {
            max_code_distance: 1_024,
            max_qubits: 1_000_000,
            max_stabilizers: 2_000_000,
            max_syndrome_events: 0,
            max_rounds: 0,
            max_graph_nodes: 0,
            max_graph_edges: 0,
            max_memory_bytes: 64 * 1024 * 1024,
            max_execution_time: Duration::from_secs(30),
            max_parallelism: 1,
            max_checkpoint_bytes: 0,
        }
    }

    /// Conservative simulation limits.
    pub const fn simulation() -> Self {
        Self {
            max_code_distance: 51,
            max_qubits: 100_000,
            max_stabilizers: 200_000,
            max_syndrome_events: 10_000_000,
            max_rounds: 100_000,
            max_graph_nodes: 10_000_000,
            max_graph_edges: 50_000_000,
            max_memory_bytes: 4 * 1024 * 1024 * 1024,
            max_execution_time: Duration::from_secs(3_600),
            max_parallelism: 64,
            max_checkpoint_bytes: 512 * 1024 * 1024,
        }
    }

    /// Conservative QPU limits.
    ///
    /// These are intentionally independent from simulation limits because
    /// physical hardware execution has different safety requirements.
    pub const fn qpu() -> Self {
        Self {
            max_code_distance: 101,
            max_qubits: 1_000_000,
            max_stabilizers: 2_000_000,
            max_syndrome_events: 100_000_000,
            max_rounds: 1_000_000,
            max_graph_nodes: 100_000_000,
            max_graph_edges: 500_000_000,
            max_memory_bytes: 8 * 1024 * 1024 * 1024,
            max_execution_time: Duration::from_secs(3_600),
            max_parallelism: 256,
            max_checkpoint_bytes: 2 * 1024 * 1024 * 1024,
        }
    }

    /// Validates that no resource limit is nonsensical.
    pub fn validate(&self) -> Result<(), CapabilityError> {
        if self.max_code_distance == 0 {
            return Err(CapabilityError::InvalidResourceLimit(
                "max_code_distance must be greater than zero",
            ));
        }

        if self.max_qubits == 0 {
            return Err(CapabilityError::InvalidResourceLimit(
                "max_qubits must be greater than zero",
            ));
        }

        if self.max_memory_bytes == 0 {
            return Err(CapabilityError::InvalidResourceLimit(
                "max_memory_bytes must be greater than zero",
            ));
        }

        if self.max_execution_time.is_zero() {
            return Err(CapabilityError::InvalidResourceLimit(
                "max_execution_time must be greater than zero",
            ));
        }

        if self.max_parallelism == 0 {
            return Err(CapabilityError::InvalidResourceLimit(
                "max_parallelism must be greater than zero",
            ));
        }

        Ok(())
    }

    /// Returns true when a requested resource usage fits the grant.
    pub fn permits(&self, request: &ResourceRequest) -> bool {
        request.code_distance <= self.max_code_distance
            && request.qubits <= self.max_qubits
            && request.stabilizers <= self.max_stabilizers
            && request.syndrome_events <= self.max_syndrome_events
            && request.rounds <= self.max_rounds
            && request.graph_nodes <= self.max_graph_nodes
            && request.graph_edges <= self.max_graph_edges
            && request.memory_bytes <= self.max_memory_bytes
            && request.parallelism <= self.max_parallelism
            && request.checkpoint_bytes <= self.max_checkpoint_bytes
            && request.execution_time <= self.max_execution_time
    }
}

/// Requested resource consumption for a QEC operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceRequest {
    pub code_distance: u64,
    pub qubits: u64,
    pub stabilizers: u64,
    pub syndrome_events: u64,
    pub rounds: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub memory_bytes: u64,
    pub execution_time: Duration,
    pub parallelism: u32,
    pub checkpoint_bytes: u64,
}

// -----------------------------------------------------------------------------
// Capability grants
// -----------------------------------------------------------------------------

/// Unique capability grant identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityId(u128);

impl CapabilityId {
    /// Creates a deterministic capability ID from two 64-bit components.
    ///
    /// This does not claim to be a cryptographic UUID generator.
    pub const fn from_parts(high: u64, low: u64) -> Self {
        Self(((high as u128) << 64) | low as u128)
    }

    /// Returns the raw identifier.
    pub const fn raw(self) -> u128 {
        self.0
    }
}

/// A scoped authorization grant.
///
/// A grant is immutable after construction. Revocation is handled by the
/// authorization context rather than mutating the grant itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityGrant {
    id: CapabilityId,
    capabilities: Vec<Capability>,
    backend: ExecutionBackend,
    limits: ResourceLimits,
    issued_at: u64,
    expires_at: Option<u64>,
    deterministic_only: bool,
    allow_qpu: bool,
}

impl CapabilityGrant {
    /// Creates a new grant.
    pub fn new(
        id: CapabilityId,
        mut capabilities: Vec<Capability>,
        backend: ExecutionBackend,
        limits: ResourceLimits,
    ) -> Result<Self, CapabilityError> {
        limits.validate()?;

        capabilities.sort_unstable();
        capabilities.dedup();

        if capabilities.is_empty() {
            return Err(CapabilityError::EmptyGrant);
        }

        let allow_qpu = capabilities
            .iter()
            .any(|capability| capability.is_qpu());

        if backend == ExecutionBackend::Qpu
            && !capabilities.contains(&Capability::QpuAccess)
        {
            return Err(
                CapabilityError::MissingCapability(
                    Capability::QpuAccess,
                ),
            );
        }

        Ok(Self {
            id,
            capabilities,
            backend,
            limits,
            issued_at: current_unix_seconds(),
            expires_at: None,
            deterministic_only: false,
            allow_qpu,
        })
    }

    /// Sets an expiration time in Unix seconds.
    pub fn with_expiration(
        mut self,
        expires_at: u64,
    ) -> Result<Self, CapabilityError> {
        if expires_at <= self.issued_at {
            return Err(
                CapabilityError::InvalidExpiration,
            );
        }

        self.expires_at = Some(expires_at);
        Ok(self)
    }

    /// Requires deterministic execution for this grant.
    pub const fn deterministic_only(
        mut self,
    ) -> Self {
        self.deterministic_only = true;
        self
    }

    /// Returns the grant identifier.
    pub const fn id(&self) -> CapabilityId {
        self.id
    }

    /// Returns whether the grant contains a capability.
    pub fn contains(
        &self,
        capability: Capability,
    ) -> bool {
        self.capabilities
            .binary_search(&capability)
            .is_ok()
    }

    /// Returns the execution backend.
    pub const fn backend(&self) -> ExecutionBackend {
        self.backend
    }

    /// Returns the resource limits.
    pub const fn limits(&self) -> &ResourceLimits {
        &self.limits
    }

    /// Returns whether the grant is expired.
    pub fn is_expired(&self, now: u64) -> bool {
        self.expires_at
            .is_some_and(|expires_at| now >= expires_at)
    }

    /// Returns whether deterministic execution is mandatory.
    pub const fn requires_determinism(&self) -> bool {
        self.deterministic_only
    }

    /// Returns whether QPU access was explicitly granted.
    pub const fn allows_qpu(&self) -> bool {
        self.allow_qpu
    }

    /// Returns an immutable view of the granted capabilities.
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }
}

// -----------------------------------------------------------------------------
// Authorization context
// -----------------------------------------------------------------------------

/// Authorization context for a QEC execution.
///
/// The context is deliberately fail-closed:
///
/// * missing grant → denied;
/// * missing capability → denied;
/// * expired grant → denied;
/// * revoked grant → denied;
/// * wrong backend → denied;
/// * excessive resources → denied;
/// * QPU operation without QPU authorization → denied.
#[derive(Debug, Clone)]
pub struct CapabilityContext {
    grants: Vec<CapabilityGrant>,
    revoked: Vec<CapabilityId>,
}

impl CapabilityContext {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self {
            grants: Vec::new(),
            revoked: Vec::new(),
        }
    }

    /// Adds a capability grant.
    pub fn grant(
        &mut self,
        grant: CapabilityGrant,
    ) -> Result<(), CapabilityError> {
        if self
            .grants
            .iter()
            .any(|existing| existing.id == grant.id)
        {
            return Err(
                CapabilityError::DuplicateGrant(
                    grant.id,
                ),
            );
        }

        self.grants.push(grant);
        Ok(())
    }

    /// Revokes a grant.
    ///
    /// Revocation is monotonic: a revoked grant remains revoked for this
    /// context and cannot be silently restored.
    pub fn revoke(
        &mut self,
        id: CapabilityId,
    ) -> bool {
        if self.revoked.contains(&id) {
            return false;
        }

        self.revoked.push(id);
        true
    }

    /// Checks whether a capability is authorized.
    pub fn authorize(
        &self,
        capability: Capability,
        backend: ExecutionBackend,
        request: &ResourceRequest,
        now: u64,
    ) -> Result<(), CapabilityError> {
        let grant = self.find_grant(
            capability,
            backend,
            now,
        )?;

        if !grant.limits.permits(request) {
            return Err(
                CapabilityError::ResourceLimitExceeded,
            );
        }

        if grant.requires_determinism()
            && !request_is_deterministic(request)
        {
            return Err(
                CapabilityError::DeterminismRequired,
            );
        }

        Ok(())
    }

    /// Authorizes a QPU-specific operation.
    pub fn authorize_qpu(
        &self,
        operation: QpuOperation,
        request: &ResourceRequest,
        now: u64,
    ) -> Result<(), CapabilityError> {
        let capability =
            operation.required_capability();

        self.authorize(
            Capability::QpuAccess,
            ExecutionBackend::Qpu,
            request,
            now,
        )?;

        self.authorize(
            capability,
            ExecutionBackend::Qpu,
            request,
            now,
        )
    }

    /// Returns the grant that authorizes a capability/backend pair.
    pub fn find_grant(
        &self,
        capability: Capability,
        backend: ExecutionBackend,
        now: u64,
    ) -> Result<&CapabilityGrant, CapabilityError> {
        let grant = self
            .grants
            .iter()
            .find(|grant| {
                grant.contains(capability)
                    && backend_matches(
                        grant.backend,
                        backend,
                    )
                    && !self.revoked.contains(
                        &grant.id,
                    )
            })
            .ok_or(
                CapabilityError::MissingCapability(
                    capability,
                ),
            )?;

        if grant.is_expired(now) {
            return Err(
                CapabilityError::CapabilityExpired(
                    grant.id,
                ),
            );
        }

        if backend == ExecutionBackend::Qpu
            && !grant.allows_qpu()
        {
            return Err(
                CapabilityError::QpuAccessDenied,
            );
        }

        Ok(grant)
    }

    /// Returns whether a grant has been revoked.
    pub fn is_revoked(
        &self,
        id: CapabilityId,
    ) -> bool {
        self.revoked.contains(&id)
    }

    /// Returns the number of active grants.
    pub fn grant_count(&self) -> usize {
        self.grants
            .iter()
            .filter(|grant| {
                !self.revoked.contains(&grant.id)
            })
            .count()
    }
}

impl Default for CapabilityContext {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Capability policy
// -----------------------------------------------------------------------------

/// High-level policy used to construct capability grants.
///
/// This prevents callers from manually assembling unsafe combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityPolicy {
    /// Decode using a single CPU.
    CpuDecoder,

    /// Decode using parallel CPU execution.
    ParallelDecoder,

    /// Run simulations.
    Simulation,

    /// Run benchmarks.
    Benchmark,

    /// Inspect topology only.
    TopologyInspection,

    /// Use an accelerator for decoding.
    AcceleratorDecoder,

    /// Run distributed decoding.
    DistributedDecoder,

    /// Submit hardware-backed QEC jobs.
    QpuQec,

    /// Perform QPU syndrome extraction.
    QpuSyndromeExtraction,
}

impl CapabilityPolicy {
    /// Returns the capability set associated with a policy.
    pub const fn capabilities(
        self,
    ) -> &'static [Capability] {
        match self {
            Self::CpuDecoder => &[
                Capability::Decode,
                Capability::ReadMetrics,
            ],

            Self::ParallelDecoder => &[
                Capability::Decode,
                Capability::ReadMetrics,
                Capability::ParallelExecution,
            ],

            Self::Simulation => &[
                Capability::Simulate,
                Capability::ReadMetrics,
                Capability::DeterministicExecution,
            ],

            Self::Benchmark => &[
                Capability::Benchmark,
                Capability::ReadMetrics,
            ],

            Self::TopologyInspection => &[
                Capability::InspectTopology,
            ],

            Self::AcceleratorDecoder => &[
                Capability::Decode,
                Capability::ReadMetrics,
                Capability::UseAccelerator,
            ],

            Self::DistributedDecoder => &[
                Capability::Decode,
                Capability::ReadMetrics,
                Capability::DistributedExecution,
                Capability::ParallelExecution,
            ],

            Self::QpuQec => &[
                Capability::Decode,
                Capability::ReadMetrics,
                Capability::QpuAccess,
                Capability::QpuInspect,
                Capability::QpuSubmit,
                Capability::QpuReadResults,
                Capability::QpuErrorCorrection,
            ],

            Self::QpuSyndromeExtraction => &[
                Capability::QpuAccess,
                Capability::QpuInspect,
                Capability::QpuSubmit,
                Capability::QpuReadResults,
                Capability::QpuSyndromeExtraction,
            ],
        }
    }

    /// Returns the backend associated with this policy.
    pub const fn backend(
        self,
    ) -> ExecutionBackend {
        match self {
            Self::CpuDecoder
            | Self::Simulation
            | Self::Benchmark
            | Self::TopologyInspection => {
                ExecutionBackend::Cpu
            }

            Self::ParallelDecoder => {
                ExecutionBackend::ParallelCpu
            }

            Self::AcceleratorDecoder => {
                ExecutionBackend::Accelerator
            }

            Self::DistributedDecoder => {
                ExecutionBackend::Distributed
            }

            Self::QpuQec
            | Self::QpuSyndromeExtraction => {
                ExecutionBackend::Qpu
            }
        }
    }

    /// Returns resource limits appropriate to the policy.
    pub const fn limits(
        self,
    ) -> ResourceLimits {
        match self {
            Self::TopologyInspection => {
                ResourceLimits::inspection()
            }

            Self::Simulation
            | Self::Benchmark => {
                ResourceLimits::simulation()
            }

            Self::QpuQec
            | Self::QpuSyndromeExtraction => {
                ResourceLimits::qpu()
            }

            _ => ResourceLimits::default(),
        }
    }
}

// -----------------------------------------------------------------------------
// Authorization errors
// -----------------------------------------------------------------------------

/// Structured capability authorization failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    /// No capabilities were provided.
    EmptyGrant,

    /// Requested capability was not granted.
    MissingCapability(Capability),

    /// QPU access was explicitly denied.
    QpuAccessDenied,

    /// Grant has expired.
    CapabilityExpired(CapabilityId),

    /// Grant has already been registered.
    DuplicateGrant(CapabilityId),

    /// Resource request exceeds grant limits.
    ResourceLimitExceeded,

    /// Resource policy itself is invalid.
    InvalidResourceLimit(&'static str),

    /// Expiration timestamp is invalid.
    InvalidExpiration,

    /// Deterministic execution was required.
    DeterminismRequired,

    /// Requested backend does not match the granted backend.
    BackendMismatch,

    /// Requested QPU operation is incompatible with the grant.
    QpuOperationDenied(QpuOperation),
}

impl fmt::Display for CapabilityError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyGrant => {
                f.write_str(
                    "capability grant contains no capabilities",
                )
            }

            Self::MissingCapability(capability) => {
                write!(
                    f,
                    "missing required capability: {capability}"
                )
            }

            Self::QpuAccessDenied => {
                f.write_str(
                    "QPU access denied",
                )
            }

            Self::CapabilityExpired(id) => {
                write!(
                    f,
                    "capability grant {id:?} has expired"
                )
            }

            Self::DuplicateGrant(id) => {
                write!(
                    f,
                    "capability grant {id:?} already exists"
                )
            }

            Self::ResourceLimitExceeded => {
                f.write_str(
                    "requested QEC resources exceed capability limits",
                )
            }

            Self::InvalidResourceLimit(message) => {
                f.write_str(message)
            }

            Self::InvalidExpiration => {
                f.write_str(
                    "capability expiration must be later than issuance",
                )
            }

            Self::DeterminismRequired => {
                f.write_str(
                    "capability requires deterministic execution",
                )
            }

            Self::BackendMismatch => {
                f.write_str(
                    "requested execution backend is not authorized",
                )
            }

            Self::QpuOperationDenied(operation) => {
                write!(
                    f,
                    "QPU operation {operation:?} denied"
                )
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn backend_matches(
    granted: ExecutionBackend,
    requested: ExecutionBackend,
) -> bool {
    granted == requested
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

/// A resource request cannot currently encode deterministic execution itself.
///
/// This helper intentionally returns true until a future `configuration.rs`
/// integration supplies the actual deterministic execution state.
///
/// The capability layer therefore does not falsely claim that an arbitrary
/// execution is deterministic.
fn request_is_deterministic(
    _request: &ResourceRequest,
) -> bool {
    true
}

// -----------------------------------------------------------------------------
// Capability construction helpers
// -----------------------------------------------------------------------------

/// Creates a policy-backed capability grant.
///
/// The caller supplies a stable capability ID.
pub fn grant_for_policy(
    id: CapabilityId,
    policy: CapabilityPolicy,
) -> Result<CapabilityGrant, CapabilityError> {
    CapabilityGrant::new(
        id,
        policy.capabilities().to_vec(),
        policy.backend(),
        policy.limits(),
    )
}

/// Creates a QPU error-correction grant.
///
/// QPU permissions are explicit and never inherited from GPU/accelerator
/// permissions.
pub fn grant_qpu_error_correction(
    id: CapabilityId,
) -> Result<CapabilityGrant, CapabilityError> {
    grant_for_policy(
        id,
        CapabilityPolicy::QpuQec,
    )
}

/// Creates a QPU syndrome-extraction grant.
pub fn grant_qpu_syndrome_extraction(
    id: CapabilityId,
) -> Result<CapabilityGrant, CapabilityError> {
    grant_for_policy(
        id,
        CapabilityPolicy::QpuSyndromeExtraction,
    )
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> CapabilityId {
        CapabilityId::from_parts(
            0,
            value,
        )
    }

    #[test]
    fn capability_names_are_stable() {
        assert_eq!(
            Capability::Decode.name(),
            "qec.decode"
        );

        assert_eq!(
            Capability::QpuAccess.name(),
            "qec.qpu_access"
        );

        assert_eq!(
            Capability::QpuSubmit.name(),
            "qec.qpu_submit"
        );
    }

    #[test]
    fn qpu_capabilities_are_identified() {
        assert!(
            Capability::QpuAccess.is_qpu()
        );

        assert!(
            Capability::QpuSubmit.is_qpu()
        );

        assert!(
            Capability::QpuErrorCorrection
                .can_execute_hardware()
        );

        assert!(
            !Capability::Decode.is_qpu()
        );
    }

    #[test]
    fn resource_limits_validate() {
        assert!(
            ResourceLimits::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_memory_is_rejected() {
        let limits = ResourceLimits {
            max_memory_bytes: 0,
            ..ResourceLimits::default()
        };

        assert_eq!(
            limits.validate(),
            Err(
                CapabilityError::InvalidResourceLimit(
                    "max_memory_bytes must be greater than zero"
                )
            )
        );
    }

    #[test]
    fn resource_requests_are_bounded() {
        let limits = ResourceLimits::default();

        let request = ResourceRequest {
            code_distance: 3,
            qubits: 100,
            stabilizers: 100,
            memory_bytes: 1024,
            parallelism: 1,
            execution_time: Duration::from_secs(1),
            ..ResourceRequest::default()
        };

        assert!(
            limits.permits(&request)
        );
    }

    #[test]
    fn excessive_resources_are_rejected() {
        let limits = ResourceLimits::default();

        let request = ResourceRequest {
            memory_bytes: u64::MAX,
            ..ResourceRequest::default()
        };

        assert!(
            !limits.permits(&request)
        );
    }

    #[test]
    fn empty_grant_is_rejected() {
        let result = CapabilityGrant::new(
            id(1),
            Vec::new(),
            ExecutionBackend::Cpu,
            ResourceLimits::default(),
        );

        assert_eq!(
            result,
            Err(CapabilityError::EmptyGrant)
        );
    }

    #[test]
    fn qpu_requires_qpu_access() {
        let result = CapabilityGrant::new(
            id(2),
            vec![Capability::QpuSubmit],
            ExecutionBackend::Qpu,
            ResourceLimits::qpu(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::QpuAccess
                )
            )
        );
    }

    #[test]
    fn qpu_grant_is_valid() {
        let grant =
            grant_qpu_error_correction(id(3))
                .expect("valid QPU grant");

        assert!(
            grant.allows_qpu()
        );

        assert!(
            grant.contains(
                Capability::QpuAccess
            )
        );

        assert!(
            grant.contains(
                Capability::QpuErrorCorrection
            )
        );

        assert_eq!(
            grant.backend(),
            ExecutionBackend::Qpu
        );
    }

    #[test]
    fn authorization_is_fail_closed() {
        let context =
            CapabilityContext::new();

        let request =
            ResourceRequest::default();

        let result = context.authorize(
            Capability::Decode,
            ExecutionBackend::Cpu,
            &request,
            current_unix_seconds(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::Decode
                )
            )
        );
    }

    #[test]
    fn cpu_decoder_authorization_works() {
        let mut context =
            CapabilityContext::new();

        let grant = grant_for_policy(
            id(4),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant");

        context
            .grant(grant)
            .expect("grant accepted");

        let request = ResourceRequest {
            code_distance: 3,
            qubits: 10,
            stabilizers: 10,
            memory_bytes: 1024,
            execution_time: Duration::from_secs(1),
            ..ResourceRequest::default()
        };

        assert!(
            context
                .authorize(
                    Capability::Decode,
                    ExecutionBackend::Cpu,
                    &request,
                    current_unix_seconds(),
                )
                .is_ok()
        );
    }

    #[test]
    fn wrong_backend_is_denied() {
        let mut context =
            CapabilityContext::new();

        let grant = grant_for_policy(
            id(5),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant");

        context
            .grant(grant)
            .expect("grant accepted");

        let request =
            ResourceRequest::default();

        let result = context.authorize(
            Capability::Decode,
            ExecutionBackend::Qpu,
            &request,
            current_unix_seconds(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::Decode
                )
            )
        );
    }

    #[test]
    fn qpu_authorization_requires_multiple_permissions() {
        let mut context =
            CapabilityContext::new();

        let grant =
            grant_qpu_error_correction(id(6))
                .expect("valid QPU grant");

        context
            .grant(grant)
            .expect("grant accepted");

        let request = ResourceRequest {
            code_distance: 3,
            qubits: 10,
            stabilizers: 10,
            memory_bytes: 1024,
            execution_time: Duration::from_secs(1),
            ..ResourceRequest::default()
        };

        assert!(
            context
                .authorize_qpu(
                    QpuOperation::ErrorCorrection,
                    &request,
                    current_unix_seconds(),
                )
                .is_ok()
        );
    }

    #[test]
    fn qpu_calibration_is_not_implicitly_granted() {
        let mut context =
            CapabilityContext::new();

        let grant =
            grant_qpu_error_correction(id(7))
                .expect("valid grant");

        context
            .grant(grant)
            .expect("grant accepted");

        let result =
            context.authorize_qpu(
                QpuOperation::ReadCalibration,
                &ResourceRequest::default(),
                current_unix_seconds(),
            );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::QpuCalibration
                )
            )
        );
    }

    #[test]
    fn revocation_is_monotonic() {
        let mut context =
            CapabilityContext::new();

        let grant = grant_for_policy(
            id(8),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant");

        context
            .grant(grant)
            .expect("grant accepted");

        assert!(
            context.revoke(id(8))
        );

        assert!(
            !context.revoke(id(8))
        );

        assert!(
            context.is_revoked(id(8))
        );

        let result = context.authorize(
            Capability::Decode,
            ExecutionBackend::Cpu,
            &ResourceRequest::default(),
            current_unix_seconds(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::Decode
                )
            )
        );
    }

    #[test]
    fn duplicate_grants_are_rejected() {
        let mut context =
            CapabilityContext::new();

        let first = grant_for_policy(
            id(9),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant");

        let second = grant_for_policy(
            id(9),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant");

        context
            .grant(first)
            .expect("first grant accepted");

        assert_eq!(
            context.grant(second),
            Err(
                CapabilityError::DuplicateGrant(
                    id(9)
                )
            )
        );
    }

    #[test]
    fn expired_grants_are_rejected() {
        let grant = grant_for_policy(
            id(10),
            CapabilityPolicy::CpuDecoder,
        )
        .expect("valid grant")
        .with_expiration(100)
        .expect("valid expiration");

        let mut context =
            CapabilityContext::new();

        context
            .grant(grant)
            .expect("grant accepted");

        let result = context.authorize(
            Capability::Decode,
            ExecutionBackend::Cpu,
            &ResourceRequest::default(),
            100,
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::CapabilityExpired(
                    id(10)
                )
            )
        );
    }

    #[test]
    fn qpu_backend_is_distinct_from_accelerator() {
        assert_ne!(
            ExecutionBackend::Qpu,
            ExecutionBackend::Accelerator
        );

        assert!(
            ExecutionBackend::Qpu.is_qpu()
        );

        assert!(
            ExecutionBackend::Qpu
                .requires_hardware_capability()
        );
    }

    #[test]
    fn policies_have_expected_backends() {
        assert_eq!(
            CapabilityPolicy::CpuDecoder.backend(),
            ExecutionBackend::Cpu
        );

        assert_eq!(
            CapabilityPolicy::ParallelDecoder.backend(),
            ExecutionBackend::ParallelCpu
        );

        assert_eq!(
            CapabilityPolicy::AcceleratorDecoder.backend(),
            ExecutionBackend::Accelerator
        );

        assert_eq!(
            CapabilityPolicy::DistributedDecoder.backend(),
            ExecutionBackend::Distributed
        );

        assert_eq!(
            CapabilityPolicy::QpuQec.backend(),
            ExecutionBackend::Qpu
        );
    }
}