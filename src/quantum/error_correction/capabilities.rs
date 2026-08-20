//! Zamani Quantum Error Correction — capability authorization boundary.
//!
//! This module owns authorization for QEC operations.
//!
//! # Ownership
//!
//! `capabilities.rs` owns:
//!
//! - capability identifiers;
//! - capability sets;
//! - capability requirements;
//! - capability grants;
//! - attenuation/delegation;
//! - expiration;
//! - monotonic revocation;
//! - backend capability requirements;
//! - QPU operation requirements;
//! - resource-aware authorization preflight;
//! - compatibility with `configuration.rs` capability requirements.
//!
//! This module does NOT own:
//!
//! - resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - validation of mathematical QEC objects (`validation.rs`);
//! - backend execution (`backend.rs`);
//! - physical QPU I/O (`qpu_adapter.rs`);
//! - telemetry transport (`telemetry.rs`);
//! - credentials or private keys.
//!
//! # Security model
//!
//! ```text
//!                    QEC REQUEST
//!                         |
//!                         v
//!                 validate input
//!                         |
//!                         v
//!                    QecConfig
//!                         |
//!                         v
//!                capability requirement
//!                         |
//!                         v
//!              CapabilityAuthority
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!       capability     expiration     revocation
//!       membership       check           check
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!                         v
//!                  resource preflight
//!                         |
//!                         v
//!                      execute
//! ```
//!
//! # Security invariants
//!
//! 1. Authorization is deny-by-default.
//! 2. Capability possession never bypasses `QecLimits`.
//! 3. `QpuAccess` does not imply `QpuSubmit`.
//! 4. `QpuSubmit` does not imply `QpuReadResults`.
//! 5. `QpuCalibration` does not imply submission.
//! 6. QPU capabilities are independent from GPU/accelerator capabilities.
//! 7. Backend support and authorization are separate concepts.
//! 8. Delegation can only attenuate privileges.
//! 9. Delegation cannot create a capability absent from the parent.
//! 10. Expired grants are rejected.
//! 11. Revoked grants remain revoked.
//! 12. Authorization is deterministic.
//! 13. Resource authorization is performed against canonical `QecLimits`.
//! 14. This module never allocates execution resources.
//! 15. This module never accesses QPU credentials.
//! 16. Capability requirements in `configuration.rs` are requirements,
//!     not authority.
//! 17. Configuration booleans cannot silently grant runtime privileges.
//!
//! # Rust compatibility
//!
//! The implementation intentionally uses stable standard-library APIs and is
//! compatible with the repository's Rust 1.97.1 target.
//!
//! # Integration contract
//!
//! `configuration.rs`
//!     -> `CapabilityConfig`
//!     -> `CapabilitySet::from_config()`
//!
//! `backend.rs`
//!     -> `Capability::required_for_backend()`
//!
//! `qpu_adapter.rs`
//!     -> `QpuOperation::required_capabilities()`
//!
//! `decoder.rs`
//!     -> `Capability::Decode`
//!
//! `streaming.rs`
//!     -> `Capability::StreamingSyndrome`
//!
//! `partition.rs` / `distributed.rs`
//!     -> `Capability::DistributedExecution`
//!
//! `checkpoint.rs`
//!     -> `Capability::Checkpoint`
//!
//! `limits.rs`
//!     -> `ResourceRequest`
//!     -> `CapabilityAuthority::authorize_with_resources()`
//!
//! `errors.rs`
//!     -> `QecError::CapabilityDenied` / resource errors.
//!
//! No later module needs to modify this file merely to add its own execution
//! path. New operations should select an existing capability or, if a truly
//! new security domain exists, add a new stable capability identifier here.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::backend::BackendKind;
use super::configuration::CapabilityConfig;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::{LimitKind, QecLimits};

// ============================================================================
// Capability identifiers
// ============================================================================

/// Stable identifier for a QEC authorization capability.
///
/// Numeric identifiers are stable and must not be reused for a different
/// security meaning once persisted in checkpoints, audit records or policy
/// documents.
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
    Serialize,
    Deserialize,
)]
pub enum Capability {
    /// Decode a validated QEC input.
    Decode = 1,

    /// Run a QEC simulation.
    Simulate = 2,

    /// Run benchmark/threshold experiments.
    Benchmark = 3,

    /// Inspect validated QEC topology.
    InspectTopology = 4,

    /// Request QEC-managed memory.
    AllocateMemory = 5,

    /// Use an accelerator.
    UseAccelerator = 6,

    /// Execute through distributed workers.
    DistributedExecution = 7,

    /// Consume incremental syndrome data.
    StreamingSyndrome = 8,

    /// Create or restore checkpoints.
    Checkpoint = 9,

    /// Require deterministic execution.
    DeterministicExecution = 10,

    /// Read QEC metrics.
    ReadMetrics = 11,

    /// Emit QEC telemetry.
    EmitTelemetry = 12,

    /// Use CPU parallelism.
    ParallelExecution = 13,

    /// Enter the QPU authorization domain.
    QpuAccess = 14,

    /// Inspect QPU metadata.
    QpuInspect = 15,

    /// Submit work to a QPU.
    QpuSubmit = 16,

    /// Read QPU measurement results.
    QpuReadResults = 17,

    /// Read QPU calibration information.
    QpuCalibration = 18,

    /// Perform hardware-backed QEC.
    QpuErrorCorrection = 19,

    /// Perform hardware syndrome extraction.
    QpuSyndromeExtraction = 20,

    /// Perform explicitly authorized remote execution.
    RemoteExecution = 21,
}

impl Capability {
    /// Stable numeric identifier.
    #[must_use]
    pub const fn id(self) -> u16 {
        self as u16
    }

    /// Stable machine-readable capability name.
    #[must_use]
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
            Self::RemoteExecution => "qec.remote_execution",
        }
    }

    /// Whether this capability belongs to the QPU security domain.
    #[must_use]
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

    /// Whether the capability can directly cause physical hardware work.
    #[must_use]
    pub const fn can_execute_hardware(self) -> bool {
        matches!(
            self,
            Self::QpuSubmit
                | Self::QpuErrorCorrection
                | Self::QpuSyndromeExtraction
        )
    }

    /// Returns the minimum backend capability.
    ///
    /// `None` means the backend itself requires no additional capability
    /// beyond the operation-specific capability.
    #[must_use]
    pub const fn required_for_backend(
        backend: BackendKind,
    ) -> Option<Self> {
        match backend {
            BackendKind::ParallelCpu => {
                Some(Self::ParallelExecution)
            }

            BackendKind::Gpu | BackendKind::Accelerator => {
                Some(Self::UseAccelerator)
            }

            BackendKind::Distributed => {
                Some(Self::DistributedExecution)
            }

            BackendKind::Qpu => Some(Self::QpuAccess),

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

/// Deterministic capability set.
///
/// `BTreeSet` guarantees:
///
/// - no duplicate capabilities;
/// - stable iteration order;
/// - deterministic serialization;
/// - deterministic subset/intersection operations.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
)]
pub struct CapabilitySet {
    inner: BTreeSet<Capability>,
}

impl CapabilitySet {
    /// Creates an empty set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    /// Creates a set from capabilities.
    #[must_use]
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

    /// Creates a set from the existing configuration contract.
    ///
    /// The configuration booleans are requirements only. Converting them to
    /// a set does not itself grant authority.
    #[must_use]
    pub fn from_config(
        config: &CapabilityConfig,
    ) -> Self {
        let mut set = Self::new();

        macro_rules! add {
            ($field:ident, $capability:expr) => {
                if config.$field {
                    set.insert($capability);
                }
            };
        }

        add!(decode, Capability::Decode);
        add!(simulate, Capability::Simulate);
        add!(benchmark, Capability::Benchmark);
        add!(
            inspect_topology,
            Capability::InspectTopology
        );
        add!(
            allocate_memory,
            Capability::AllocateMemory
        );
        add!(
            accelerator,
            Capability::UseAccelerator
        );
        add!(
            distributed_execution,
            Capability::DistributedExecution
        );
        add!(
            streaming_syndrome,
            Capability::StreamingSyndrome
        );
        add!(checkpoint, Capability::Checkpoint);
        add!(
            deterministic_execution,
            Capability::DeterministicExecution
        );
        add!(read_metrics, Capability::ReadMetrics);
        add!(
            emit_telemetry,
            Capability::EmitTelemetry
        );
        add!(
            parallel_execution,
            Capability::ParallelExecution
        );
        add!(qpu_access, Capability::QpuAccess);
        add!(qpu_inspect, Capability::QpuInspect);
        add!(qpu_submit, Capability::QpuSubmit);
        add!(
            qpu_read_results,
            Capability::QpuReadResults
        );
        add!(
            qpu_calibration,
            Capability::QpuCalibration
        );
        add!(
            qpu_error_correction,
            Capability::QpuErrorCorrection
        );
        add!(
            qpu_syndrome_extraction,
            Capability::QpuSyndromeExtraction
        );
        add!(
            remote_execution,
            Capability::RemoteExecution
        );

        set
    }

    /// Inserts a capability.
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

    /// Checks whether a capability exists.
    #[must_use]
    pub fn contains(
        &self,
        capability: Capability,
    ) -> bool {
        self.inner.contains(&capability)
    }

    /// Number of capabilities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns stable capability iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Capability> {
        self.inner.iter()
    }

    /// Returns stable vector representation.
    #[must_use]
    pub fn to_vec(&self) -> Vec<Capability> {
        self.inner.iter().copied().collect()
    }

    /// Returns whether all required capabilities are present.
    #[must_use]
    pub fn contains_all(
        &self,
        required: &CapabilitySet,
    ) -> bool {
        required
            .inner
            .iter()
            .all(|capability| self.contains(*capability))
    }

    /// Returns the intersection with another set.
    #[must_use]
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

    /// Returns a strictly attenuated capability set.
    ///
    /// Delegation is always:
    ///
    /// `child = parent ∩ requested`
    #[must_use]
    pub fn attenuate(
        &self,
        requested: &CapabilitySet,
    ) -> CapabilitySet {
        self.intersection(requested)
    }

    /// Returns whether any QPU capability is present.
    #[must_use]
    pub fn contains_qpu_capability(&self) -> bool {
        self.inner
            .iter()
            .any(|capability| capability.is_qpu())
    }

    /// Returns whether any hardware-execution capability is present.
    #[must_use]
    pub fn can_execute_hardware(&self) -> bool {
        self.inner
            .iter()
            .any(|capability| capability.can_execute_hardware())
    }
}

// ============================================================================
// QPU operation model
// ============================================================================

/// Explicit QPU operation scope.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum QpuOperation {
    /// Inspect immutable QPU metadata.
    Inspect,

    /// Read calibration information.
    ReadCalibration,

    /// Submit a circuit.
    SubmitCircuit,

    /// Read measurement results.
    ReadResults,

    /// Perform hardware-backed QEC.
    ErrorCorrection,

    /// Perform hardware syndrome extraction.
    SyndromeExtraction,
}

impl QpuOperation {
    /// Returns the operation-specific capability.
    #[must_use]
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

    /// Returns the complete requirement for this operation.
    ///
    /// Every QPU operation requires both:
    ///
    /// - `QpuAccess`;
    /// - its operation-specific capability.
    #[must_use]
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
// Generic operation requirements
// ============================================================================

/// Standard QEC operation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum QecOperation {
    Decode,
    Simulate,
    Benchmark,
    InspectTopology,
    AllocateMemory,
    Stream,
    Checkpoint,
    DeterministicExecution,
    ReadMetrics,
    EmitTelemetry,
    ParallelExecution,
    DistributedExecution,
    UseAccelerator,
    RemoteExecution,
}

impl QecOperation {
    /// Returns the capability required by the operation.
    #[must_use]
    pub const fn required_capability(
        self,
    ) -> Capability {
        match self {
            Self::Decode => Capability::Decode,
            Self::Simulate => Capability::Simulate,
            Self::Benchmark => Capability::Benchmark,
            Self::InspectTopology => {
                Capability::InspectTopology
            }
            Self::AllocateMemory => {
                Capability::AllocateMemory
            }
            Self::Stream => {
                Capability::StreamingSyndrome
            }
            Self::Checkpoint => Capability::Checkpoint,
            Self::DeterministicExecution => {
                Capability::DeterministicExecution
            }
            Self::ReadMetrics => Capability::ReadMetrics,
            Self::EmitTelemetry => {
                Capability::EmitTelemetry
            }
            Self::ParallelExecution => {
                Capability::ParallelExecution
            }
            Self::DistributedExecution => {
                Capability::DistributedExecution
            }
            Self::UseAccelerator => {
                Capability::UseAccelerator
            }
            Self::RemoteExecution => {
                Capability::RemoteExecution
            }
        }
    }

    /// Converts the operation into a capability set.
    #[must_use]
    pub fn required_capabilities(
        self,
    ) -> CapabilitySet {
        CapabilitySet::from_iter([
            self.required_capability(),
        ])
    }
}

// ============================================================================
// Resource request
// ============================================================================

/// Resource requirements presented to authorization preflight.
///
/// This structure is deliberately a request, not a policy.
///
/// `QecLimits` remains the only production resource ceiling.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
)]
pub struct ResourceRequest {
    pub code_distance: usize,
    pub qubits: usize,
    pub stabilizers: usize,
    pub syndrome_events: usize,
    pub rounds: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub memory_bytes: u64,
    pub decoder_time_ns: u64,
    pub parallelism: usize,
    pub checkpoint_size_bytes: u64,
    pub partitions: usize,
    pub stream_buffer_events: usize,
    pub decoder_iterations: usize,
    pub stabilizer_weight: usize,
    pub logical_operator_weight: usize,
    pub qubits_per_partition: usize,
    pub qpu_shots: u64,
    pub qpu_circuits: u64,
    pub verification_operations: u64,
}

impl ResourceRequest {
    /// Empty request.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            code_distance: 0,
            qubits: 0,
            stabilizers: 0,
            syndrome_events: 0,
            rounds: 0,
            graph_nodes: 0,
            graph_edges: 0,
            memory_bytes: 0,
            decoder_time_ns: 0,
            parallelism: 0,
            checkpoint_size_bytes: 0,
            partitions: 0,
            stream_buffer_events: 0,
            decoder_iterations: 0,
            stabilizer_weight: 0,
            logical_operator_weight: 0,
            qubits_per_partition: 0,
            qpu_shots: 0,
            qpu_circuits: 0,
            verification_operations: 0,
        }
    }

    /// Returns the first policy violation.
    #[must_use]
    pub fn first_violation(
        &self,
        limits: &QecLimits,
    ) -> Option<(LimitKind, u128, u128)> {
        let checks = [
            (
                LimitKind::CodeDistance,
                self.code_distance as u128,
                limits.max_code_distance as u128,
            ),
            (
                LimitKind::Qubits,
                self.qubits as u128,
                limits.max_qubits as u128,
            ),
            (
                LimitKind::Stabilizers,
                self.stabilizers as u128,
                limits.max_stabilizers as u128,
            ),
            (
                LimitKind::SyndromeEvents,
                self.syndrome_events as u128,
                limits.max_syndrome_events as u128,
            ),
            (
                LimitKind::MeasurementRounds,
                self.rounds as u128,
                limits.max_rounds as u128,
            ),
            (
                LimitKind::GraphNodes,
                self.graph_nodes as u128,
                limits.max_graph_nodes as u128,
            ),
            (
                LimitKind::GraphEdges,
                self.graph_edges as u128,
                limits.max_graph_edges as u128,
            ),
            (
                LimitKind::MemoryBytes,
                self.memory_bytes as u128,
                limits.max_memory_bytes as u128,
            ),
            (
                LimitKind::DecoderTimeNs,
                self.decoder_time_ns as u128,
                limits.max_decoder_time_ns as u128,
            ),
            (
                LimitKind::Parallelism,
                self.parallelism as u128,
                limits.max_parallelism as u128,
            ),
            (
                LimitKind::CheckpointSizeBytes,
                self.checkpoint_size_bytes as u128,
                limits.max_checkpoint_size_bytes as u128,
            ),
            (
                LimitKind::Partitions,
                self.partitions as u128,
                limits.max_partitions as u128,
            ),
            (
                LimitKind::StreamBufferEvents,
                self.stream_buffer_events as u128,
                limits.max_stream_buffer_events as u128,
            ),
            (
                LimitKind::DecoderIterations,
                self.decoder_iterations as u128,
                limits.max_decoder_iterations as u128,
            ),
            (
                LimitKind::StabilizerWeight,
                self.stabilizer_weight as u128,
                limits.max_stabilizer_weight as u128,
            ),
            (
                LimitKind::LogicalOperatorWeight,
                self.logical_operator_weight as u128,
                limits.max_logical_operator_weight as u128,
            ),
            (
                LimitKind::QubitsPerPartition,
                self.qubits_per_partition as u128,
                limits.max_qubits_per_partition as u128,
            ),
            (
                LimitKind::QpuShots,
                self.qpu_shots as u128,
                limits.max_qpu_shots as u128,
            ),
            (
                LimitKind::QpuCircuits,
                self.qpu_circuits as u128,
                limits.max_qpu_circuits as u128,
            ),
            (
                LimitKind::VerificationOperations,
                self.verification_operations as u128,
                limits.max_verification_operations as u128,
            ),
        ];

        checks
            .into_iter()
            .find(|(_, requested, maximum)| {
                *requested > *maximum
            })
    }

    /// Checks the request against canonical limits.
    pub fn validate_against(
        &self,
        limits: &QecLimits,
    ) -> QecResult<()> {
        if let Some((resource, requested, maximum)) =
            self.first_violation(limits)
        {
            return Err(QecError::ResourceLimitExceeded {
                resource: resource_to_error_kind(resource),
                requested,
                current: 0,
                limit: maximum,
                message: format!(
                    "resource request exceeds canonical QEC limit: {resource}"
                ),
            });
        }

        Ok(())
    }
}

fn resource_to_error_kind(
    kind: LimitKind,
) -> ResourceKind {
    match kind {
        LimitKind::CodeDistance => {
            ResourceKind::CodeDistance
        }
        LimitKind::Qubits => ResourceKind::Qubits,
        LimitKind::Stabilizers => {
            ResourceKind::Stabilizers
        }
        LimitKind::SyndromeEvents => {
            ResourceKind::SyndromeEvents
        }
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

// ============================================================================
// Capability grants
// ============================================================================

/// Stable identifier for a capability grant.
///
/// This is an authorization-record identifier, not a secret and not a
/// cryptographic credential.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct GrantId(pub u128);

impl GrantId {
    /// Creates a grant identifier from an explicit value.
    ///
    /// Callers that require cryptographically random identifiers should
    /// generate the value outside this module using an approved RNG.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

impl fmt::Display for GrantId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

/// Capability grant.
///
/// A grant is valid only when:
///
/// - it is known by the authority;
/// - it has not been revoked;
/// - it has not expired;
/// - the requested operation is contained in its capabilities.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct CapabilityGrant {
    pub id: GrantId,
    pub capabilities: CapabilitySet,
    pub issued_at_unix_seconds: u64,
    pub expires_at_unix_seconds: Option<u64>,
    pub parent: Option<GrantId>,
}

impl CapabilityGrant {
    /// Creates a non-expiring grant.
    #[must_use]
    pub const fn new(
        id: GrantId,
        capabilities: CapabilitySet,
        issued_at_unix_seconds: u64,
    ) -> Self {
        Self {
            id,
            capabilities,
            issued_at_unix_seconds,
            expires_at_unix_seconds: None,
            parent: None,
        }
    }

    /// Creates a time-limited grant.
    pub fn with_expiry(
        id: GrantId,
        capabilities: CapabilitySet,
        issued_at_unix_seconds: u64,
        expires_at_unix_seconds: u64,
    ) -> Result<Self, CapabilityError> {
        if expires_at_unix_seconds
            <= issued_at_unix_seconds
        {
            return Err(
                CapabilityError::InvalidGrantLifetime,
            );
        }

        Ok(Self {
            id,
            capabilities,
            issued_at_unix_seconds,
            expires_at_unix_seconds: Some(
                expires_at_unix_seconds,
            ),
            parent: None,
        })
    }

    /// Returns whether the grant has expired at `now`.
    #[must_use]
    pub fn is_expired(
        &self,
        now_unix_seconds: u64,
    ) -> bool {
        match self.expires_at_unix_seconds {
            Some(expiry) => now_unix_seconds >= expiry,
            None => false,
        }
    }

    /// Creates an attenuated child grant.
    ///
    /// A child can never possess a capability that the parent does not have.
    pub fn attenuate(
        &self,
        child_id: GrantId,
        requested: &CapabilitySet,
        issued_at_unix_seconds: u64,
        expires_at_unix_seconds: Option<u64>,
    ) -> Result<Self, CapabilityError> {
        if self.is_expired(issued_at_unix_seconds) {
            return Err(
                CapabilityError::ExpiredGrant {
                    grant_id: self.id,
                },
            );
        }

        let capabilities =
            self.capabilities.attenuate(requested);

        if let Some(expiry) = expires_at_unix_seconds {
            if expiry <= issued_at_unix_seconds {
                return Err(
                    CapabilityError::InvalidGrantLifetime,
                );
            }

            if let Some(parent_expiry) =
                self.expires_at_unix_seconds
            {
                if expiry > parent_expiry {
                    return Err(
                        CapabilityError::DelegationBeyondParentExpiry,
                    );
                }
            }
        }

        Ok(Self {
            id: child_id,
            capabilities,
            issued_at_unix_seconds,
            expires_at_unix_seconds:
                expires_at_unix_seconds.or(
                    self.expires_at_unix_seconds,
                ),
            parent: Some(self.id),
        })
    }
}

// ============================================================================
// Authorization authority
// ============================================================================

/// In-process capability authority.
///
/// The authority owns the grant registry and revocation set.
///
/// It does not create cryptographic identity. Identity/authentication belongs
/// to the surrounding security system. This component answers one narrower
/// question:
///
/// > "Does this already-authenticated execution context possess the requested
/// > QEC capability?"
#[derive(Debug, Default)]
pub struct CapabilityAuthority {
    grants: BTreeMap<GrantId, CapabilityGrant>,
    revoked: BTreeSet<GrantId>,
}

impl CapabilityAuthority {
    /// Creates an empty authority.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a grant.
    ///
    /// Reusing an existing grant ID is rejected. This prevents accidental
    /// replacement of an existing authorization record.
    pub fn register(
        &mut self,
        grant: CapabilityGrant,
    ) -> Result<(), CapabilityError> {
        if self.grants.contains_key(&grant.id) {
            return Err(
                CapabilityError::DuplicateGrant {
                    grant_id: grant.id,
                },
            );
        }

        if self.revoked.contains(&grant.id) {
            return Err(
                CapabilityError::GrantAlreadyRevoked {
                    grant_id: grant.id,
                },
            );
        }

        self.grants.insert(grant.id, grant);

        Ok(())
    }

    /// Revokes a grant permanently.
    ///
    /// Revocation is monotonic: there is intentionally no un-revoke API.
    pub fn revoke(
        &mut self,
        grant_id: GrantId,
    ) -> Result<(), CapabilityError> {
        if !self.grants.contains_key(&grant_id) {
            return Err(
                CapabilityError::UnknownGrant { grant_id },
            );
        }

        self.revoked.insert(grant_id);

        Ok(())
    }

    /// Checks whether a grant has been revoked.
    #[must_use]
    pub fn is_revoked(
        &self,
        grant_id: GrantId,
    ) -> bool {
        self.revoked.contains(&grant_id)
    }

    /// Returns a registered grant.
    #[must_use]
    pub fn grant(
        &self,
        grant_id: GrantId,
    ) -> Option<&CapabilityGrant> {
        self.grants.get(&grant_id)
    }

    /// Authorizes a capability requirement.
    pub fn authorize(
        &self,
        grant_id: GrantId,
        required: &CapabilitySet,
        now_unix_seconds: u64,
    ) -> QecResult<()> {
        let grant = self
            .grants
            .get(&grant_id)
            .ok_or_else(|| {
                QecError::CapabilityDenied {
                    capability: required_names(required),
                    operation: "authorization".to_owned(),
                    message: format!(
                        "unknown capability grant: {grant_id}"
                    ),
                }
            })?;

        if self.revoked.contains(&grant_id) {
            return Err(QecError::CapabilityDenied {
                capability: required_names(required),
                operation: "authorization".to_owned(),
                message: format!(
                    "capability grant has been revoked: {grant_id}"
                ),
            });
        }

        if grant.is_expired(now_unix_seconds) {
            return Err(QecError::CapabilityDenied {
                capability: required_names(required),
                operation: "authorization".to_owned(),
                message: format!(
                    "capability grant has expired: {grant_id}"
                ),
            });
        }

        if !grant.capabilities.contains_all(required) {
            return Err(QecError::CapabilityDenied {
                capability: required_names(required),
                operation: "authorization".to_owned(),
                message:
                    "required capability is not present in the grant"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Performs authorization and canonical resource preflight together.
    ///
    /// Authorization does not replace runtime accounting. A later execution
    /// layer must still reserve actual resources through `resources.rs`.
    pub fn authorize_with_resources(
        &self,
        grant_id: GrantId,
        required: &CapabilitySet,
        request: &ResourceRequest,
        limits: &QecLimits,
        now_unix_seconds: u64,
    ) -> QecResult<()> {
        self.authorize(
            grant_id,
            required,
            now_unix_seconds,
        )?;

        request.validate_against(limits)?;

        Ok(())
    }

    /// Authorizes a backend.
    pub fn authorize_backend(
        &self,
        grant_id: GrantId,
        backend: BackendKind,
        now_unix_seconds: u64,
    ) -> QecResult<()> {
        let required =
            Capability::required_for_backend(backend)
                .map(|capability| {
                    CapabilitySet::from_iter([capability])
                })
                .unwrap_or_else(CapabilitySet::new);

        if required.is_empty() {
            return Ok(());
        }

        self.authorize(
            grant_id,
            &required,
            now_unix_seconds,
        )
    }

    /// Authorizes an explicit QPU operation.
    pub fn authorize_qpu(
        &self,
        grant_id: GrantId,
        operation: QpuOperation,
        now_unix_seconds: u64,
    ) -> QecResult<()> {
        self.authorize(
            grant_id,
            &operation.required_capabilities(),
            now_unix_seconds,
        )
    }

    /// Delegates an attenuated grant.
    ///
    /// The parent grant must be registered and valid at delegation time.
    pub fn delegate(
        &mut self,
        parent_id: GrantId,
        child_id: GrantId,
        requested: &CapabilitySet,
        issued_at_unix_seconds: u64,
        expires_at_unix_seconds: Option<u64>,
    ) -> Result<(), CapabilityError> {
        let parent = self
            .grants
            .get(&parent_id)
            .ok_or(
                CapabilityError::UnknownGrant {
                    grant_id: parent_id,
                },
            )?;

        if self.revoked.contains(&parent_id) {
            return Err(
                CapabilityError::RevokedGrant {
                    grant_id: parent_id,
                },
            );
        }

        let child = parent.attenuate(
            child_id,
            requested,
            issued_at_unix_seconds,
            expires_at_unix_seconds,
        )?;

        self.register(child)
    }

    /// Returns the number of registered grants.
    #[must_use]
    pub fn grant_count(&self) -> usize {
        self.grants.len()
    }

    /// Returns the number of revoked grants.
    #[must_use]
    pub fn revoked_count(&self) -> usize {
        self.revoked.len()
    }
}

fn required_names(
    required: &CapabilitySet,
) -> String {
    required
        .iter()
        .map(|capability| capability.name())
        .collect::<Vec<_>>()
        .join(",")
}

// ============================================================================
// Capability preflight
// ============================================================================

/// Result of authorization preflight.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct AuthorizationDecision {
    pub granted: bool,
    pub capabilities: CapabilitySet,
}

impl AuthorizationDecision {
    /// Creates an allowed decision.
    #[must_use]
    pub fn allow(
        capabilities: CapabilitySet,
    ) -> Self {
        Self {
            granted: true,
            capabilities,
        }
    }

    /// Creates a denied decision.
    #[must_use]
    pub fn deny() -> Self {
        Self {
            granted: false,
            capabilities: CapabilitySet::new(),
        }
    }
}

// ============================================================================
// Configuration integration
// ============================================================================

/// Converts configuration requirements into an authorization set.
///
/// This function intentionally does not create a grant.
///
/// `CapabilityConfig` describes required capabilities; an authenticated
/// execution context must still provide an authorized `CapabilityGrant`.
#[must_use]
pub fn requirements_from_config(
    config: &CapabilityConfig,
) -> CapabilitySet {
    CapabilitySet::from_config(config)
}

/// Checks whether a configuration requirement set is satisfied by a grant.
pub fn authorize_config(
    authority: &CapabilityAuthority,
    grant_id: GrantId,
    config: &CapabilityConfig,
    now_unix_seconds: u64,
) -> QecResult<()> {
    let required =
        requirements_from_config(config);

    authority.authorize(
        grant_id,
        &required,
        now_unix_seconds,
    )
}

// ============================================================================
// Current time helper
// ============================================================================

/// Returns current Unix time in seconds.
///
/// This is intentionally a small boundary helper. Deterministic tests should
/// supply explicit timestamps to authorization functions instead of calling
/// this function.
pub fn current_unix_seconds() -> Result<u64, CapabilityError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| CapabilityError::ClockBeforeUnixEpoch)
}

// ============================================================================
// Capability errors
// ============================================================================

/// Local errors produced by the capability authority.
///
/// Public operation boundaries should convert these to `QecError` where
/// appropriate. Authorization itself already returns `QecError` for denied
/// execution.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum CapabilityError {
    UnknownGrant {
        grant_id: GrantId,
    },

    DuplicateGrant {
        grant_id: GrantId,
    },

    GrantAlreadyRevoked {
        grant_id: GrantId,
    },

    RevokedGrant {
        grant_id: GrantId,
    },

    ExpiredGrant {
        grant_id: GrantId,
    },

    InvalidGrantLifetime,

    DelegationBeyondParentExpiry,

    InvalidResourceRequest,

    ClockBeforeUnixEpoch,
}

impl fmt::Display for CapabilityError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnknownGrant { grant_id } => {
                write!(f, "unknown capability grant: {grant_id}")
            }

            Self::DuplicateGrant { grant_id } => {
                write!(
                    f,
                    "capability grant already exists: {grant_id}"
                )
            }

            Self::GrantAlreadyRevoked { grant_id } => {
                write!(
                    f,
                    "capability grant has already been revoked: {grant_id}"
                )
            }

            Self::RevokedGrant { grant_id } => {
                write!(
                    f,
                    "capability grant has been revoked: {grant_id}"
                )
            }

            Self::ExpiredGrant { grant_id } => {
                write!(
                    f,
                    "capability grant has expired: {grant_id}"
                )
            }

            Self::InvalidGrantLifetime => {
                f.write_str(
                    "capability grant expiration must be after issuance",
                )
            }

            Self::DelegationBeyondParentExpiry => {
                f.write_str(
                    "delegated grant cannot outlive its parent grant",
                )
            }

            Self::InvalidResourceRequest => {
                f.write_str("invalid QEC resource request")
            }

            Self::ClockBeforeUnixEpoch => {
                f.write_str(
                    "system clock is before the Unix epoch",
                )
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn grant(
        capabilities: CapabilitySet,
    ) -> CapabilityGrant {
        CapabilityGrant::new(
            GrantId::new(1),
            capabilities,
            100,
        )
    }

    #[test]
    fn capability_ids_are_stable() {
        assert_eq!(Capability::Decode.id(), 1);
        assert_eq!(Capability::QpuAccess.id(), 14);
        assert_eq!(Capability::QpuSubmit.id(), 16);
        assert_eq!(
            Capability::QpuReadResults.id(),
            17
        );
    }

    #[test]
    fn capability_names_are_stable() {
        assert_eq!(
            Capability::Decode.name(),
            "qec.decode"
        );

        assert_eq!(
            Capability::QpuSubmit.name(),
            "qec.qpu_submit"
        );
    }

    #[test]
    fn empty_set_denies_everything() {
        let set = CapabilitySet::new();

        assert!(!set.contains(Capability::Decode));
        assert!(!set.contains(Capability::QpuSubmit));
    }

    #[test]
    fn attenuation_cannot_escalate() {
        let parent = CapabilitySet::from_iter([
            Capability::Decode,
            Capability::ReadMetrics,
        ]);

        let requested = CapabilitySet::from_iter([
            Capability::Decode,
            Capability::QpuSubmit,
        ]);

        let child = parent.attenuate(&requested);

        assert!(child.contains(Capability::Decode));
        assert!(!child.contains(Capability::QpuSubmit));
        assert!(!child.contains(Capability::ReadMetrics));
    }

    #[test]
    fn qpu_submit_requires_access_and_submit() {
        let required =
            QpuOperation::SubmitCircuit
                .required_capabilities();

        assert!(
            required.contains(Capability::QpuAccess)
        );

        assert!(
            required.contains(Capability::QpuSubmit)
        );

        assert!(
            !required.contains(
                Capability::QpuReadResults
            )
        );
    }

    #[test]
    fn qpu_read_results_is_independent_from_submit() {
        let submit =
            QpuOperation::SubmitCircuit
                .required_capabilities();

        let read =
            QpuOperation::ReadResults
                .required_capabilities();

        assert!(
            submit.contains(Capability::QpuSubmit)
        );

        assert!(
            !submit.contains(
                Capability::QpuReadResults
            )
        );

        assert!(
            read.contains(
                Capability::QpuReadResults
            )
        );

        assert!(
            !read.contains(Capability::QpuSubmit)
        );
    }

    #[test]
    fn configuration_conversion_preserves_requirements() {
        let mut config =
            CapabilityConfig::default();

        config.qpu_access = true;
        config.qpu_submit = true;

        let set =
            CapabilitySet::from_config(&config);

        assert!(set.contains(
            Capability::QpuAccess
        ));

        assert!(set.contains(
            Capability::QpuSubmit
        ));
    }

    #[test]
    fn grant_authorization_succeeds() {
        let capabilities =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        let mut authority =
            CapabilityAuthority::new();

        authority
            .register(grant(capabilities))
            .expect("grant registration");

        let required =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        assert!(
            authority
                .authorize(
                    GrantId::new(1),
                    &required,
                    101,
                )
                .is_ok()
        );
    }

    #[test]
    fn missing_capability_is_denied() {
        let capabilities =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        let mut authority =
            CapabilityAuthority::new();

        authority
            .register(grant(capabilities))
            .expect("grant registration");

        let required =
            CapabilitySet::from_iter([
                Capability::QpuSubmit,
            ]);

        assert!(
            authority
                .authorize(
                    GrantId::new(1),
                    &required,
                    101,
                )
                .is_err()
        );
    }

    #[test]
    fn revoked_grant_is_denied() {
        let capabilities =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        let mut authority =
            CapabilityAuthority::new();

        authority
            .register(grant(capabilities))
            .expect("grant registration");

        authority
            .revoke(GrantId::new(1))
            .expect("revocation");

        let required =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        assert!(
            authority
                .authorize(
                    GrantId::new(1),
                    &required,
                    101,
                )
                .is_err()
        );
    }

    #[test]
    fn expired_grant_is_denied() {
        let grant =
            CapabilityGrant::with_expiry(
                GrantId::new(1),
                CapabilitySet::from_iter([
                    Capability::Decode,
                ]),
                100,
                200,
            )
            .expect("valid lifetime");

        let mut authority =
            CapabilityAuthority::new();

        authority
            .register(grant)
            .expect("grant registration");

        let required =
            CapabilitySet::from_iter([
                Capability::Decode,
            ]);

        assert!(
            authority
                .authorize(
                    GrantId::new(1),
                    &required,
                    200,
                )
                .is_err()
        );
    }

    #[test]
    fn delegation_is_attenuated() {
        let parent =
            CapabilityGrant::new(
                GrantId::new(1),
                CapabilitySet::from_iter([
                    Capability::Decode,
                    Capability::ReadMetrics,
                ]),
                100,
            );

        let child =
            parent
                .attenuate(
                    GrantId::new(2),
                    &CapabilitySet::from_iter([
                        Capability::Decode,
                        Capability::QpuSubmit,
                    ]),
                    101,
                    None,
                )
                .expect("delegation");

        assert!(
            child
                .capabilities
                .contains(Capability::Decode)
        );

        assert!(
            !child
                .capabilities
                .contains(
                    Capability::QpuSubmit
                )
        );
    }

    #[test]
    fn resource_request_uses_canonical_limits() {
        let limits = QecLimits::default();

        let request = ResourceRequest {
            qubits: limits.max_qubits + 1,
            ..ResourceRequest::empty()
        };

        assert!(
            request
                .first_violation(&limits)
                .is_some()
        );
    }

    #[test]
    fn backend_requirement_is_explicit() {
        assert_eq!(
            Capability::required_for_backend(
                BackendKind::Qpu
            ),
            Some(Capability::QpuAccess)
        );

        assert_eq!(
            Capability::required_for_backend(
                BackendKind::ParallelCpu
            ),
            Some(Capability::ParallelExecution)
        );

        assert_eq!(
            Capability::required_for_backend(
                BackendKind::Cpu
            ),
            None
        );
    }
}