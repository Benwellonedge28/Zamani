//! Zamani Quantum Error Correction — capability-based authorization.
//!
//! This module is the security boundary between an already-validated QEC
//! workload and execution privileges.
//!
//! Architectural contract:
//!
//! ```text
//!                         UNTRUSTED REQUEST
//!                               │
//!                               ▼
//!                         QecConfig
//!                               │
//!                               ▼
//!                         QecLimits
//!                               │
//!                               ▼
//!                    Capability authorization
//!                               │
//!              ┌────────────────┼────────────────┐
//!              │                │                │
//!              ▼                ▼                ▼
//!          CPU decode      Accelerator        QPU adapter
//!              │                │                │
//!              └────────────────┼────────────────┘
//!                               ▼
//!                         Resource preflight
//!                               │
//!                               ▼
//!                            Execute
//! ```
//!
//! ## Security invariants
//!
//! * Deny by default.
//! * No capability implies no privilege.
//! * Capabilities are explicit and independently scoped.
//! * Capability possession never implies arbitrary resource usage.
//! * `QpuAccess` never implies `QpuSubmit`.
//! * `QpuSubmit` never implies `QpuReadResults`.
//! * QPU capabilities are never inherited from GPU/accelerator capabilities.
//! * Backend support and authorization are separate concepts.
//! * `QecLimits` is the canonical resource policy.
//! * This module does not define a competing resource-limit model.
//! * Expired grants are rejected.
//! * Revoked grants are rejected.
//! * Revocation is monotonic.
//! * Delegated capabilities can only be attenuated.
//! * Authorization is deterministic.
//! * No capability operation silently escalates privileges.
//! * Deterministic execution is explicitly represented.
//! * Resource preflight occurs before expensive execution.
//! * Credentials, private keys, network addresses and device secrets do not
//!   belong in this module.
//!
//! QPU execution remains a control-plane authorization concern here.
//! Actual physical QPU I/O belongs in `qpu_adapter.rs` / the backend adapter.

#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::backend::BackendKind;
use super::configuration::QecLimits;
use super::errors::{QecError, QecResult, ResourceKind};

// ============================================================================
// Compatibility
// ============================================================================

/// Canonical execution backend.
///
/// `backend.rs` owns the actual backend model. This alias prevents this
/// authorization layer from creating a competing backend enumeration.
pub use super::backend::BackendKind as ExecutionBackend;

// ============================================================================
// Capability identifiers
// ============================================================================

/// Stable identifier for a QEC capability.
///
/// Numeric IDs are intentionally stable because they may appear in:
///
/// * authorization records;
/// * audit logs;
/// * telemetry;
/// * checkpoints;
/// * security tests;
/// * policy files.
#[repr(u16)]
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
pub enum Capability {
    /// Decode a validated syndrome/error representation.
    Decode = 1,

    /// Execute QEC simulation.
    Simulate = 2,

    /// Execute threshold/performance experiments.
    Benchmark = 3,

    /// Inspect validated QEC topology.
    InspectTopology = 4,

    /// Request QEC-managed memory.
    AllocateMemory = 5,

    /// Use a hardware/software accelerator.
    UseAccelerator = 6,

    /// Execute through distributed workers.
    DistributedExecution = 7,

    /// Consume incremental syndrome data.
    StreamingSyndrome = 8,

    /// Create/resume QEC checkpoints.
    Checkpoint = 9,

    /// Request deterministic execution.
    DeterministicExecution = 10,

    /// Read decoder/QEC metrics.
    ReadMetrics = 11,

    /// Emit telemetry.
    EmitTelemetry = 12,

    /// Use CPU parallelism.
    ParallelExecution = 13,

    /// Establish QPU authorization context.
    QpuAccess = 14,

    /// Inspect QPU metadata.
    QpuInspect = 15,

    /// Submit a circuit/workload to a QPU.
    QpuSubmit = 16,

    /// Read QPU measurement results.
    QpuReadResults = 17,

    /// Read calibration information.
    QpuCalibration = 18,

    /// Perform hardware-backed QEC.
    QpuErrorCorrection = 19,

    /// Perform hardware syndrome extraction.
    QpuSyndromeExtraction = 20,
}

impl Capability {
    /// Stable numeric identifier.
    pub const fn id(self) -> u16 {
        self as u16
    }

    /// Stable machine-readable name.
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
            Self::QpuSyndromeExtraction => {
                "qec.qpu_syndrome_extraction"
            }
        }
    }

    /// Whether the capability belongs to the QPU security domain.
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

    /// Whether the capability may cause physical hardware execution.
    pub const fn can_execute_hardware(self) -> bool {
        matches!(
            self,
            Self::QpuSubmit
                | Self::QpuErrorCorrection
                | Self::QpuSyndromeExtraction
        )
    }

    /// Required capability for a backend.
    pub const fn required_for_backend(
        backend: BackendKind,
    ) -> Option<Self> {
        match backend {
            BackendKind::ParallelCpu => {
                Some(Self::ParallelExecution)
            }

            BackendKind::Gpu
            | BackendKind::Accelerator => {
                Some(Self::UseAccelerator)
            }

            BackendKind::Distributed => {
                Some(Self::DistributedExecution)
            }

            BackendKind::Qpu => {
                Some(Self::QpuAccess)
            }

            BackendKind::Cpu
            | BackendKind::Simulator
            | BackendKind::Emulator
            | BackendKind::Custom => None,
        }
    }
}

impl fmt::Display for Capability {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ============================================================================
// Capability sets
// ============================================================================

/// Deterministic set of capabilities.
///
/// A set is used instead of a `Vec` so:
///
/// * duplicates are impossible;
/// * authorization order is deterministic;
/// * subset checks are straightforward;
/// * attenuation is explicit.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
)]
pub struct CapabilitySet {
    inner: BTreeSet<Capability>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    pub const fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    /// Creates a set from an iterator.
    pub fn from_iter<I>(
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        Self {
            inner: capabilities.into_iter().collect(),
        }
    }

    /// Adds a capability.
    pub fn insert(
        &mut self,
        capability: Capability,
    ) -> bool {
        self.inner.insert(capability)
    }

    /// Removes a capability.
    pub fn remove(
        &mut self,
        capability: Capability,
    ) -> bool {
        self.inner.remove(&capability)
    }

    /// Checks membership.
    pub fn contains(
        &self,
        capability: Capability,
    ) -> bool {
        self.inner.contains(&capability)
    }

    /// Number of capabilities.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator in stable order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Capability> {
        self.inner.iter()
    }

    /// Returns a vector in stable order.
    pub fn to_vec(&self) -> Vec<Capability> {
        self.inner.iter().copied().collect()
    }

    /// Returns true if this set contains every capability in `required`.
    pub fn contains_all(
        &self,
        required: &CapabilitySet,
    ) -> bool {
        required
            .inner
            .iter()
            .all(|capability| self.contains(*capability))
    }

    /// Returns the intersection with another capability set.
    pub fn intersection(
        &self,
        other: &CapabilitySet,
    ) -> CapabilitySet {
        CapabilitySet {
            inner: self
                .inner
                .intersection(&other.inner)
                .copied()
                .collect(),
        }
    }

    /// Returns whether this set contains any QPU capability.
    pub fn contains_qpu_capability(&self) -> bool {
        self.inner.iter().any(|capability| {
            capability.is_qpu()
        })
    }

    /// Returns a reduced set suitable for delegation.
    ///
    /// Delegation is an attenuation operation:
    ///
    /// ```text
    /// parent capabilities
    ///          │
    ///          ▼
    ///       requested
    ///          │
    ///          ▼
    /// intersection
    ///          │
    ///          ▼
    /// child capabilities
    /// ```
    pub fn attenuate(
        &self,
        requested: &CapabilitySet,
    ) -> CapabilitySet {
        self.intersection(requested)
    }
}

// ============================================================================
// QPU operations
// ============================================================================

/// Explicit QPU operation scope.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum QpuOperation {
    /// Inspect immutable device metadata.
    Inspect,

    /// Read calibration information.
    ReadCalibration,

    /// Submit an arbitrary circuit/workload.
    SubmitCircuit,

    /// Read measurement results.
    ReadResults,

    /// Execute hardware-backed QEC.
    ErrorCorrection,

    /// Execute hardware syndrome extraction.
    SyndromeExtraction,
}

impl QpuOperation {
    /// Capability required by this operation.
    pub const fn required_capability(
        self,
    ) -> Capability {
        match self {
            Self::Inspect => Capability::QpuInspect,
            Self::ReadCalibration => {
                Capability::QpuCalibration
            }
            Self::SubmitCircuit => Capability::QpuSubmit,
            Self::ReadResults => {
                Capability::QpuReadResults
            }
            Self::ErrorCorrection => {
                Capability::QpuErrorCorrection
            }
            Self::SyndromeExtraction => {
                Capability::QpuSyndromeExtraction
            }
        }
    }

    /// Returns the complete capability requirement.
    ///
    /// Every QPU operation requires `QpuAccess` in addition to its
    /// operation-specific permission.
    pub fn required_capabilities(
        self,
    ) -> CapabilitySet {
        CapabilitySet::from_iter([
            Capability::QpuAccess,
            self.required_capability(),
        ])
    }
}

// ============================================================================
// Resource request
// ============================================================================

/// Resource requirements supplied to the capability preflight boundary.
///
/// This type deliberately mirrors canonical `QecLimits` dimensions instead
/// of defining another resource-policy structure.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
pub struct ResourceRequest {
    pub code_distance: u64,
    pub qubits: u64,
    pub stabilizers: u64,
    pub syndrome_events: u64,
    pub rounds: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub memory_bytes: u64,
    pub decoder_time_ms: u64,
    pub parallelism: u32,
    pub checkpoint_bytes: u64,
    pub qpu_shots: u64,
    pub qpu_circuits: u64,

    /// Whether the execution has been configured for deterministic behavior.
    ///
    /// This must be supplied by `configuration.rs` / the execution context.
    /// It is intentionally not inferred from resource usage.
    pub deterministic: bool,
}

impl ResourceRequest {
    /// Empty resource request.
    pub const fn empty() -> Self {
        Self::default()
    }

    /// Validates the request itself.
    pub fn validate(
        &self,
    ) -> Result<(), CapabilityError> {
        let values = [
            self.code_distance,
            self.qubits,
            self.stabilizers,
            self.syndrome_events,
            self.rounds,
            self.graph_nodes,
            self.graph_edges,
            self.memory_bytes,
            self.decoder_time_ms,
            self.checkpoint_bytes,
            self.qpu_shots,
            self.qpu_circuits,
        ];

        if values.iter().any(|value| *value == u64::MAX) {
            return Err(
                CapabilityError::InvalidResourceRequest,
            );
        }

        Ok(())
    }

    /// Checks the request against canonical QEC limits.
    pub fn permitted_by(
        &self,
        limits: &QecLimits,
    ) -> bool {
        self.code_distance
            <= limits.max_code_distance
            && self.qubits <= limits.max_qubits
            && self.stabilizers <= limits.max_stabilizers
            && self.syndrome_events
                <= limits.max_syndrome_events
            && self.rounds <= limits.max_rounds
            && self.graph_nodes
                <= limits.max_graph_nodes
            && self.graph_edges
                <= limits.max_graph_edges
            && self.memory_bytes
                <= limits.max_memory_bytes
            && self.decoder_time_ms
                <= limits.max_decoder_time_ms
            && self.parallelism
                <= limits.max_parallelism
            && self.checkpoint_bytes
                <= limits.max_checkpoint_size_bytes
            && self.qpu_shots
                <= limits.max_qpu_shots
            && self.qpu_circuits
                <= limits.max_qpu_circuits
    }

    /// Returns the first violated canonical resource dimension.
    pub fn first_violation(
        &self,
        limits: &QecLimits,
    ) -> Option<(ResourceKind, u128, u128)> {
        let checks = [
            (
                ResourceKind::CodeDistance,
                self.code_distance,
                limits.max_code_distance,
            ),
            (
                ResourceKind::Qubits,
                self.qubits,
                limits.max_qubits,
            ),
            (
                ResourceKind::Stabilizers,
                self.stabilizers,
                limits.max_stabilizers,
            ),
            (
                ResourceKind::SyndromeEvents,
                self.syndrome_events,
                limits.max_syndrome_events,
            ),
            (
                ResourceKind::MeasurementRounds,
                self.rounds,
                limits.max_rounds,
            ),
            (
                ResourceKind::GraphNodes,
                self.graph_nodes,
                limits.max_graph_nodes,
            ),
            (
                ResourceKind::GraphEdges,
                self.graph_edges,
                limits.max_graph_edges,
            ),
            (
                ResourceKind::MemoryBytes,
                self.memory_bytes,
                limits.max_memory_bytes,
            ),
            (
                ResourceKind::Parallelism,
                self.parallelism as u64,
                limits.max_parallelism as u64,
            ),
            (
                ResourceKind::CheckpointSize,
                self.checkpoint_bytes,
                limits.max_checkpoint_size_bytes,
            ),
            (
                ResourceKind::QpuShots,
                self.qpu_shots,
                limits.max_qpu_shots,
            ),
            (
                ResourceKind::QpuCircuits,
                self.qpu_circuits,
                limits.max_qpu_circuits,
            ),
        ];

        checks
            .into_iter()
            .find(|(_, requested, maximum)| {
                requested > maximum
            })
            .map(|(kind, requested, maximum)| {
                (
                    kind,
                    requested as u128,
                    maximum as u128,
                )
            })
    }
}

// ============================================================================
// Capability grants
// ============================================================================

/// Stable capability-grant identifier.
///
/// This identifier is an authorization identifier, not a cryptographic
/// identity and not a UUID generator.
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
pub struct CapabilityId(u128);

impl CapabilityId {
    /// Constructs an identifier from two stable 64-bit components.
    pub const fn from_parts(
        high: u64,
        low: u64,
    ) -> Self {
        Self(((high as u128) << 64) | low as u128)
    }

    /// Returns the raw identifier.
    pub const fn raw(
        self,
    ) -> u128 {
        self.0
    }
}

/// Immutable authorization grant.
///
/// A grant binds:
///
/// * capabilities;
/// * backend scope;
/// * canonical QEC limits;
/// * expiration;
/// * deterministic-execution requirement.
///
/// Revocation is maintained by `CapabilityContext`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CapabilityGrant {
    id: CapabilityId,
    capabilities: CapabilitySet,
    backend: BackendKind,
    limits: QecLimits,
    issued_at: u64,
    expires_at: Option<u64>,
    deterministic_required: bool,
}

impl CapabilityGrant {
    /// Creates a capability grant.
    pub fn new(
        id: CapabilityId,
        capabilities: CapabilitySet,
        backend: BackendKind,
        limits: QecLimits,
    ) -> Result<Self, CapabilityError> {
        if capabilities.is_empty() {
            return Err(
                CapabilityError::EmptyGrant,
            );
        }

        limits.validate().map_err(
            CapabilityError::InvalidCanonicalLimits,
        )?;

        Self::validate_backend_requirements(
            &capabilities,
            backend,
        )?;

        let deterministic_required =
            capabilities.contains(
                Capability::DeterministicExecution,
            );

        Ok(Self {
            id,
            capabilities,
            backend,
            limits,
            issued_at: current_unix_seconds(),
            expires_at: None,
            deterministic_required,
        })
    }

    /// Adds an expiration timestamp.
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

    /// Returns the grant ID.
    pub const fn id(
        &self,
    ) -> CapabilityId {
        self.id
    }

    /// Returns the capabilities.
    pub const fn capabilities(
        &self,
    ) -> &CapabilitySet {
        &self.capabilities
    }

    /// Returns the backend scope.
    pub const fn backend(
        &self,
    ) -> BackendKind {
        self.backend
    }

    /// Returns canonical QEC limits.
    pub const fn limits(
        &self,
    ) -> &QecLimits {
        &self.limits
    }

    /// Returns whether the grant has expired.
    pub fn is_expired(
        &self,
        now: u64,
    ) -> bool {
        self.expires_at
            .is_some_and(|expiry| now >= expiry)
    }

    /// Returns whether deterministic execution is required.
    pub const fn requires_determinism(
        &self,
    ) -> bool {
        self.deterministic_required
    }

    /// Checks capability membership.
    pub fn contains(
        &self,
        capability: Capability,
    ) -> bool {
        self.capabilities.contains(capability)
    }

    fn validate_backend_requirements(
        capabilities: &CapabilitySet,
        backend: BackendKind,
    ) -> Result<(), CapabilityError> {
        if let Some(required) =
            Capability::required_for_backend(backend)
        {
            if !capabilities.contains(required) {
                return Err(
                    CapabilityError::MissingCapability(
                        required,
                    ),
                );
            }
        }

        if backend != BackendKind::Qpu
            && capabilities
                .contains(Capability::QpuSubmit)
        {
            return Err(
                CapabilityError::QpuCapabilityRequiresQpuBackend,
            );
        }

        if backend != BackendKind::Qpu
            && capabilities.contains(
                Capability::QpuErrorCorrection,
            )
        {
            return Err(
                CapabilityError::QpuCapabilityRequiresQpuBackend,
            );
        }

        if backend != BackendKind::Qpu
            && capabilities.contains(
                Capability::QpuSyndromeExtraction,
            )
        {
            return Err(
                CapabilityError::QpuCapabilityRequiresQpuBackend,
            );
        }

        Ok(())
    }

    /// Creates an attenuated child grant.
    ///
    /// Delegation cannot:
    ///
    /// * add capabilities;
    /// * widen resource limits;
    /// * widen backend scope;
    /// * extend the parent's expiration.
    pub fn delegate(
        &self,
        child_id: CapabilityId,
        requested_capabilities: &CapabilitySet,
        requested_limits: &QecLimits,
        expires_at: Option<u64>,
    ) -> Result<Self, CapabilityError> {
        let capabilities = self
            .capabilities
            .attenuate(requested_capabilities);

        if capabilities.is_empty() {
            return Err(
                CapabilityError::EmptyDelegation,
            );
        }

        ensure_limits_are_not_wider(
            &self.limits,
            requested_limits,
        )?;

        if let Some(child_expiry) = expires_at {
            if let Some(parent_expiry) =
                self.expires_at
            {
                if child_expiry > parent_expiry {
                    return Err(
                        CapabilityError::DelegationExtendsExpiry,
                    );
                }
            }

            if child_expiry <= self.issued_at {
                return Err(
                    CapabilityError::InvalidExpiration,
                );
            }
        }

        let mut child =
            Self::new(
                child_id,
                capabilities,
                self.backend,
                requested_limits.clone(),
            )?;

        child.expires_at =
            expires_at.or(self.expires_at);

        Ok(child)
    }
}

// ============================================================================
// Authorization context
// ============================================================================

/// Runtime authorization context.
///
/// The context is fail-closed and monotonic with respect to revocation.
#[derive(
    Debug,
    Clone,
    Default,
)]
pub struct CapabilityContext {
    grants: Vec<CapabilityGrant>,
    revoked: BTreeSet<CapabilityId>,
}

impl CapabilityContext {
    /// Creates an empty authorization context.
    pub const fn new() -> Self {
        Self {
            grants: Vec::new(),
            revoked: BTreeSet::new(),
        }
    }

    /// Registers a grant.
    pub fn grant(
        &mut self,
        grant: CapabilityGrant,
    ) -> Result<(), CapabilityError> {
        if self.revoked.contains(&grant.id) {
            return Err(
                CapabilityError::GrantAlreadyRevoked(
                    grant.id,
                ),
            );
        }

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

        self.grants.sort_by_key(
            |grant| grant.id,
        );

        Ok(())
    }

    /// Monotonically revokes a grant.
    pub fn revoke(
        &mut self,
        id: CapabilityId,
    ) -> bool {
        self.revoked.insert(id)
    }

    /// Returns whether a grant is revoked.
    pub fn is_revoked(
        &self,
        id: CapabilityId,
    ) -> bool {
        self.revoked.contains(&id)
    }

    /// Authorizes one capability.
    pub fn authorize(
        &self,
        capability: Capability,
        backend: BackendKind,
        request: &ResourceRequest,
        now: u64,
    ) -> Result<(), CapabilityError> {
        let required =
            CapabilitySet::from_iter([capability]);

        self.authorize_set(
            &required,
            backend,
            request,
            now,
        )
    }

    /// Authorizes an entire capability set against one grant.
    ///
    /// Requiring one grant to satisfy the complete operation prevents
    /// privilege composition across unrelated grants.
    pub fn authorize_set(
        &self,
        required: &CapabilitySet,
        backend: BackendKind,
        request: &ResourceRequest,
        now: u64,
    ) -> Result<(), CapabilityError> {
        request.validate()?;

        if required.is_empty() {
            return Err(
                CapabilityError::EmptyRequirement,
            );
        }

        let grant = self
            .find_grant(
                required,
                backend,
                now,
            )?;

        if grant.requires_determinism()
            && !request.deterministic
        {
            return Err(
                CapabilityError::DeterminismRequired,
            );
        }

        if let Some((
            resource,
            requested,
            maximum,
        )) = request.first_violation(
            &grant.limits,
        ) {
            return Err(
                CapabilityError::ResourceLimitExceeded {
                    resource,
                    requested,
                    maximum,
                },
            );
        }

        Ok(())
    }

    /// Authorizes a QPU operation.
    ///
    /// Both `QpuAccess` and the operation-specific capability must exist
    /// within the same valid grant.
    pub fn authorize_qpu(
        &self,
        operation: QpuOperation,
        request: &ResourceRequest,
        now: u64,
    ) -> Result<(), CapabilityError> {
        let required =
            operation.required_capabilities();

        self.authorize_set(
            &required,
            BackendKind::Qpu,
            request,
            now,
        )
    }

    /// Locates a valid grant for a complete capability set.
    pub fn find_grant(
        &self,
        required: &CapabilitySet,
        backend: BackendKind,
        now: u64,
    ) -> Result<&CapabilityGrant, CapabilityError> {
        for grant in &self.grants {
            if self.revoked.contains(&grant.id) {
                continue;
            }

            if grant.backend != backend {
                continue;
            }

            if !grant.capabilities.contains_all(
                required,
            ) {
                continue;
            }

            if grant.is_expired(now) {
                continue;
            }

            return Ok(grant);
        }

        if self
            .grants
            .iter()
            .any(|grant| {
                grant.backend == backend
                    && grant
                        .capabilities
                        .contains_all(required)
            })
        {
            return Err(
                CapabilityError::CapabilityExpiredOrRevoked,
            );
        }

        Err(
            CapabilityError::MissingCapabilitySet(
                required.clone(),
            ),
        )
    }

    /// Returns the number of registered grants.
    pub fn grant_count(
        &self,
    ) -> usize {
        self.grants.len()
    }

    /// Returns the number of active grants.
    pub fn active_grant_count(
        &self,
        now: u64,
    ) -> usize {
        self.grants
            .iter()
            .filter(|grant| {
                !self.revoked.contains(
                    &grant.id,
                )
                    && !grant.is_expired(now)
            })
            .count()
    }
}

// ============================================================================
// High-level policies
// ============================================================================

/// Safe predefined capability policies.
///
/// These are convenience constructors, not authorization bypasses.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum CapabilityPolicy {
    CpuDecoder,
    ParallelDecoder,
    Simulation,
    Benchmark,
    TopologyInspection,
    AcceleratorDecoder,
    DistributedDecoder,
    QpuQec,
    QpuSyndromeExtraction,
}

impl CapabilityPolicy {
    /// Capabilities associated with this policy.
    pub fn capabilities(
        self,
    ) -> CapabilitySet {
        match self {
            Self::CpuDecoder => {
                CapabilitySet::from_iter([
                    Capability::Decode,
                    Capability::ReadMetrics,
                ])
            }

            Self::ParallelDecoder => {
                CapabilitySet::from_iter([
                    Capability::Decode,
                    Capability::ReadMetrics,
                    Capability::ParallelExecution,
                ])
            }

            Self::Simulation => {
                CapabilitySet::from_iter([
                    Capability::Simulate,
                    Capability::ReadMetrics,
                    Capability::DeterministicExecution,
                ])
            }

            Self::Benchmark => {
                CapabilitySet::from_iter([
                    Capability::Benchmark,
                    Capability::ReadMetrics,
                ])
            }

            Self::TopologyInspection => {
                CapabilitySet::from_iter([
                    Capability::InspectTopology,
                ])
            }

            Self::AcceleratorDecoder => {
                CapabilitySet::from_iter([
                    Capability::Decode,
                    Capability::ReadMetrics,
                    Capability::UseAccelerator,
                ])
            }

            Self::DistributedDecoder => {
                CapabilitySet::from_iter([
                    Capability::Decode,
                    Capability::ReadMetrics,
                    Capability::DistributedExecution,
                    Capability::ParallelExecution,
                ])
            }

            Self::QpuQec => {
                CapabilitySet::from_iter([
                    Capability::QpuAccess,
                    Capability::QpuInspect,
                    Capability::QpuSubmit,
                    Capability::QpuReadResults,
                    Capability::QpuErrorCorrection,
                ])
            }

            Self::QpuSyndromeExtraction => {
                CapabilitySet::from_iter([
                    Capability::QpuAccess,
                    Capability::QpuInspect,
                    Capability::QpuSubmit,
                    Capability::QpuReadResults,
                    Capability::QpuSyndromeExtraction,
                ])
            }
        }
    }

    /// Backend associated with the policy.
    pub const fn backend(
        self,
    ) -> BackendKind {
        match self {
            Self::CpuDecoder
            | Self::Simulation
            | Self::Benchmark
            | Self::TopologyInspection => {
                BackendKind::Cpu
            }

            Self::ParallelDecoder => {
                BackendKind::ParallelCpu
            }

            Self::AcceleratorDecoder => {
                BackendKind::Accelerator
            }

            Self::DistributedDecoder => {
                BackendKind::Distributed
            }

            Self::QpuQec
            | Self::QpuSyndromeExtraction => {
                BackendKind::Qpu
            }
        }
    }

    /// Canonical limits for the policy.
    ///
    /// These are sourced from `QecLimits`; this module does not maintain a
    /// second independent resource-policy structure.
    pub fn limits(
        self,
    ) -> QecLimits {
        let mut limits =
            QecLimits::default();

        match self {
            Self::TopologyInspection => {
                limits.max_syndrome_events = 1;
                limits.max_rounds = 1;
                limits.max_graph_nodes = 1;
                limits.max_graph_edges = 1;
            }

            Self::Simulation => {
                limits.max_parallelism =
                    limits.max_parallelism.min(64);
            }

            Self::Benchmark => {
                limits.max_parallelism =
                    limits.max_parallelism.min(64);
            }

            Self::QpuQec
            | Self::QpuSyndromeExtraction => {
                limits.max_parallelism =
                    limits.max_parallelism.min(256);
            }

            _ => {}
        }

        limits
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Capability-layer error.
///
/// This remains specialized internally, but converts into the canonical
/// `QecError` at the public subsystem boundary.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum CapabilityError {
    EmptyGrant,

    EmptyRequirement,

    EmptyDelegation,

    MissingCapability(Capability),

    MissingCapabilitySet(CapabilitySet),

    QpuAccessDenied,

    QpuOperationDenied(QpuOperation),

    QpuCapabilityRequiresQpuBackend,

    DuplicateGrant(CapabilityId),

    GrantAlreadyRevoked(CapabilityId),

    CapabilityExpired(CapabilityId),

    CapabilityExpiredOrRevoked,

    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        maximum: u128,
    },

    InvalidResourceRequest,

    InvalidCanonicalLimits(String),

    InvalidExpiration,

    DeterminismRequired,

    DelegationExtendsExpiry,

    DelegationWidensLimits,

    DelegationChangesBackend,
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

            Self::EmptyRequirement => {
                f.write_str(
                    "authorization request contains no required capabilities",
                )
            }

            Self::EmptyDelegation => {
                f.write_str(
                    "delegation would produce an empty capability set",
                )
            }

            Self::MissingCapability(capability) => {
                write!(
                    f,
                    "missing required capability: {capability}"
                )
            }

            Self::MissingCapabilitySet(capabilities) => {
                write!(
                    f,
                    "missing required capability set: {:?}",
                    capabilities.to_vec()
                )
            }

            Self::QpuAccessDenied => {
                f.write_str(
                    "QPU access denied",
                )
            }

            Self::QpuOperationDenied(operation) => {
                write!(
                    f,
                    "QPU operation {operation:?} denied"
                )
            }

            Self::QpuCapabilityRequiresQpuBackend => {
                f.write_str(
                    "QPU execution capabilities require the QPU backend",
                )
            }

            Self::DuplicateGrant(id) => {
                write!(
                    f,
                    "capability grant {id:?} already exists"
                )
            }

            Self::GrantAlreadyRevoked(id) => {
                write!(
                    f,
                    "capability grant {id:?} is already revoked"
                )
            }

            Self::CapabilityExpired(id) => {
                write!(
                    f,
                    "capability grant {id:?} has expired"
                )
            }

            Self::CapabilityExpiredOrRevoked => {
                f.write_str(
                    "matching capability grant is expired or revoked",
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "resource {resource} requested={requested} maximum={maximum}"
                )
            }

            Self::InvalidResourceRequest => {
                f.write_str(
                    "invalid QEC resource request",
                )
            }

            Self::InvalidCanonicalLimits(message) => {
                write!(
                    f,
                    "invalid canonical QEC limits: {message}"
                )
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

            Self::DelegationExtendsExpiry => {
                f.write_str(
                    "delegated capability cannot outlive its parent",
                )
            }

            Self::DelegationWidensLimits => {
                f.write_str(
                    "delegated capability cannot widen resource limits",
                )
            }

            Self::DelegationChangesBackend => {
                f.write_str(
                    "delegated capability cannot change backend scope",
                )
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

// ============================================================================
// QecError integration
// ============================================================================

impl From<CapabilityError> for QecError {
    fn from(
        error: CapabilityError,
    ) -> Self {
        match error {
            CapabilityError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => QecError::ResourceLimitExceeded {
                resource,
                requested,
                current: 0,
                limit: maximum,
                message:
                    "QEC capability resource preflight rejected the request"
                        .to_string(),
            },

            CapabilityError::InvalidCanonicalLimits(
                message,
            ) => QecError::UnsupportedConfiguration {
                feature: "capability_limits".to_string(),
                message,
            },

            CapabilityError::InvalidResourceRequest
            | CapabilityError::EmptyGrant
            | CapabilityError::EmptyRequirement
            | CapabilityError::EmptyDelegation
            | CapabilityError::InvalidExpiration
            | CapabilityError::DelegationExtendsExpiry
            | CapabilityError::DelegationWidensLimits
            | CapabilityError::DelegationChangesBackend
            | CapabilityError::QpuCapabilityRequiresQpuBackend => {
                QecError::InvalidInput {
                    message: error.to_string(),
                }
            }

            CapabilityError::MissingCapability(_)
            | CapabilityError::MissingCapabilitySet(_)
            | CapabilityError::QpuAccessDenied
            | CapabilityError::QpuOperationDenied(_)
            | CapabilityError::DuplicateGrant(_)
            | CapabilityError::GrantAlreadyRevoked(_)
            | CapabilityError::CapabilityExpired(_)
            | CapabilityError::CapabilityExpiredOrRevoked
            | CapabilityError::DeterminismRequired => {
                QecError::UnsupportedConfiguration {
                    feature:
                        "qec_capability_authorization"
                            .to_string(),
                    message: error.to_string(),
                }
            }
        }
    }
}

// ============================================================================
// Construction helpers
// ============================================================================

/// Creates a policy-backed capability grant.
pub fn grant_for_policy(
    id: CapabilityId,
    policy: CapabilityPolicy,
) -> Result<CapabilityGrant, CapabilityError> {
    CapabilityGrant::new(
        id,
        policy.capabilities(),
        policy.backend(),
        policy.limits(),
    )
}

/// Creates a QPU QEC grant.
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

/// Authorizes an operation and converts failure directly to the canonical
/// QEC error boundary.
pub fn authorize_or_error(
    context: &CapabilityContext,
    required: &CapabilitySet,
    backend: BackendKind,
    request: &ResourceRequest,
    now: u64,
) -> QecResult<()> {
    context
        .authorize_set(
            required,
            backend,
            request,
            now,
        )
        .map_err(QecError::from)
}

// ============================================================================
// Helpers
// ============================================================================

fn ensure_limits_are_not_wider(
    parent: &QecLimits,
    child: &QecLimits,
) -> Result<(), CapabilityError> {
    if child.max_code_distance
        > parent.max_code_distance
        || child.max_qubits > parent.max_qubits
        || child.max_stabilizers
            > parent.max_stabilizers
        || child.max_syndrome_events
            > parent.max_syndrome_events
        || child.max_rounds > parent.max_rounds
        || child.max_graph_nodes
            > parent.max_graph_nodes
        || child.max_graph_edges
            > parent.max_graph_edges
        || child.max_memory_bytes
            > parent.max_memory_bytes
        || child.max_decoder_time_ms
            > parent.max_decoder_time_ms
        || child.max_parallelism
            > parent.max_parallelism
        || child.max_checkpoint_size_bytes
            > parent.max_checkpoint_size_bytes
        || child.max_qpu_shots
            > parent.max_qpu_shots
        || child.max_qpu_circuits
            > parent.max_qpu_circuits
    {
        return Err(
            CapabilityError::DelegationWidensLimits,
        );
    }

    Ok(())
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> CapabilityId {
        CapabilityId::from_parts(0, value)
    }

    fn request() -> ResourceRequest {
        ResourceRequest {
            code_distance: 3,
            qubits: 100,
            stabilizers: 100,
            syndrome_events: 100,
            rounds: 10,
            graph_nodes: 100,
            graph_edges: 200,
            memory_bytes: 1024 * 1024,
            decoder_time_ms: 100,
            parallelism: 1,
            checkpoint_bytes: 0,
            qpu_shots: 0,
            qpu_circuits: 0,
            deterministic: false,
        }
    }

    #[test]
    fn empty_grant_is_rejected() {
        let result = CapabilityGrant::new(
            id(1),
            CapabilitySet::new(),
            BackendKind::Cpu,
            QecLimits::default(),
        );

        assert_eq!(
            result,
            Err(CapabilityError::EmptyGrant)
        );
    }

    #[test]
    fn cpu_decoder_is_authorized() {
        let grant = grant_for_policy(
            id(1),
            CapabilityPolicy::CpuDecoder,
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        context
            .authorize(
                Capability::Decode,
                BackendKind::Cpu,
                &request(),
                current_unix_seconds(),
            )
            .unwrap();
    }

    #[test]
    fn missing_capability_is_denied() {
        let grant = grant_for_policy(
            id(1),
            CapabilityPolicy::CpuDecoder,
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        let result = context.authorize(
            Capability::Checkpoint,
            BackendKind::Cpu,
            &request(),
            current_unix_seconds(),
        );

        assert!(matches!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::Checkpoint
                )
            )
        ));
    }

    #[test]
    fn qpu_access_does_not_imply_submission() {
        let grant = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::QpuAccess,
            ]),
            BackendKind::Qpu,
            QecLimits::default(),
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        let result = context.authorize_qpu(
            QpuOperation::SubmitCircuit,
            &request(),
            current_unix_seconds(),
        );

        assert!(matches!(
            result,
            Err(
                CapabilityError::MissingCapabilitySet(_)
            )
        ));
    }

    #[test]
    fn qpu_submission_requires_qpu_backend() {
        let result = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::QpuAccess,
                Capability::QpuSubmit,
            ]),
            BackendKind::Cpu,
            QecLimits::default(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::QpuCapabilityRequiresQpuBackend
            )
        );
    }

    #[test]
    fn accelerator_requires_accelerator_capability() {
        let result = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Decode,
            ]),
            BackendKind::Gpu,
            QecLimits::default(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::UseAccelerator
                )
            )
        );
    }

    #[test]
    fn distributed_requires_distributed_capability() {
        let result = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Decode,
            ]),
            BackendKind::Distributed,
            QecLimits::default(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::DistributedExecution
                )
            )
        );
    }

    #[test]
    fn parallel_cpu_requires_parallel_capability() {
        let result = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Decode,
            ]),
            BackendKind::ParallelCpu,
            QecLimits::default(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::MissingCapability(
                    Capability::ParallelExecution
                )
            )
        );
    }

    #[test]
    fn resource_preflight_is_enforced() {
        let mut limits =
            QecLimits::default();

        limits.max_qubits = 10;

        let grant = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Decode,
            ]),
            BackendKind::Cpu,
            limits,
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        let mut workload = request();
        workload.qubits = 11;

        let result = context.authorize(
            Capability::Decode,
            BackendKind::Cpu,
            &workload,
            current_unix_seconds(),
        );

        assert!(matches!(
            result,
            Err(
                CapabilityError::ResourceLimitExceeded {
                    resource: ResourceKind::Qubits,
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_capability_requires_deterministic_request() {
        let grant = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Simulate,
                Capability::DeterministicExecution,
            ]),
            BackendKind::Cpu,
            QecLimits::default(),
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        let result = context.authorize(
            Capability::Simulate,
            BackendKind::Cpu,
            &request(),
            current_unix_seconds(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::DeterminismRequired
            )
        );
    }

    #[test]
    fn deterministic_capability_accepts_deterministic_request() {
        let grant = CapabilityGrant::new(
            id(1),
            CapabilitySet::from_iter([
                Capability::Simulate,
                Capability::DeterministicExecution,
            ]),
            BackendKind::Cpu,
            QecLimits::default(),
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        let mut workload = request();
        workload.deterministic = true;

        context
            .authorize(
                Capability::Simulate,
                BackendKind::Cpu,
                &workload,
                current_unix_seconds(),
            )
            .unwrap();
    }

    #[test]
    fn revocation_is_monotonic() {
        let grant = grant_for_policy(
            id(1),
            CapabilityPolicy::CpuDecoder,
        )
        .unwrap();

        let mut context =
            CapabilityContext::new();

        context.grant(grant).unwrap();

        assert!(context.revoke(id(1)));
        assert!(!context.revoke(id(1)));

        let result = context.authorize(
            Capability::Decode,
            BackendKind::Cpu,
            &request(),
            current_unix_seconds(),
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::CapabilityExpiredOrRevoked
            )
        );
    }

    #[test]
    fn delegation_can_only_attenuate() {
        let parent = grant_for_policy(
            id(1),
            CapabilityPolicy::QpuQec,
        )
        .unwrap();

        let requested =
            CapabilitySet::from_iter([
                Capability::QpuAccess,
                Capability::QpuReadResults,
            ]);

        let child_limits =
            parent.limits().clone();

        let child = parent
            .delegate(
                id(2),
                &requested,
                &child_limits,
                None,
            )
            .unwrap();

        assert!(
            child.contains(
                Capability::QpuAccess
            )
        );

        assert!(
            child.contains(
                Capability::QpuReadResults
            )
        );

        assert!(
            !child.contains(
                Capability::QpuSubmit
            )
        );
    }

    #[test]
    fn delegation_cannot_widen_limits() {
        let parent = grant_for_policy(
            id(1),
            CapabilityPolicy::CpuDecoder,
        )
        .unwrap();

        let mut child_limits =
            parent.limits().clone();

        child_limits.max_qubits += 1;

        let result = parent.delegate(
            id(2),
            parent.capabilities(),
            &child_limits,
            None,
        );

        assert_eq!(
            result,
            Err(
                CapabilityError::DelegationWidensLimits
            )
        );
    }

    #[test]
    fn capability_set_attenuation_is_intersection() {
        let parent =
            CapabilitySet::from_iter([
                Capability::Decode,
                Capability::ReadMetrics,
                Capability::QpuAccess,
            ]);

        let requested =
            CapabilitySet::from_iter([
                Capability::ReadMetrics,
                Capability::QpuSubmit,
            ]);

        let child =
            parent.attenuate(&requested);

        assert!(
            child.contains(
                Capability::ReadMetrics
            )
        );

        assert!(
            !child.contains(
                Capability::QpuSubmit
            )
        );
    }
}