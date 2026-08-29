//! Zamani Quantum Memory — Representation and Storage Migration
//!
//! Production-grade, provider-neutral migration orchestration for
//! `quantum::memory`.
//!
//! # Mission
//!
//! This module defines the canonical contract for moving quantum-memory
//! resources between:
//!
//! - quantum-state representations;
//! - host/device storage locations;
//! - local/distributed memory domains;
//! - simulator/emulator execution domains;
//! - provider-owned/backend-native memory;
//! - remote execution resources;
//! - future quantum-memory technologies.
//!
//! # Critical architectural rule
//!
//! Migration is NOT synonymous with copying amplitudes.
//!
//! A source or destination may be:
//!
//! - a dense state vector;
//! - a density matrix;
//! - a stabilizer/tableau;
//! - a sparse state;
//! - an MPS/tensor network;
//! - a backend-native opaque state;
//! - a photonic/continuous-variable representation;
//! - an annealing/analog representation;
//! - a remote QPU execution resource;
//! - another provider-defined representation.
//!
//! A real QPU may expose no readable quantum state at all. Therefore this
//! module never assumes that `source -> destination` means "read amplitudes,
//! allocate a vector, and copy bytes". Provider/backend implementations must
//! explicitly advertise the migration semantics they support.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                       execution/runtime
//!                              |
//!                              v
//!                    quantum::memory::state
//!                              |
//!                              v
//!                   quantum::memory::migration
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!     representation       storage             execution
//!     conversion           migration           migration
//!          |                   |                   |
//!          v                   v                   v
//!   state-vector <->      CPU <-> GPU       simulator <-> QPU*
//!   density      <->      CPU <-> distributed       |
//!   stabilizer   <->      GPU <-> remote            |
//!   sparse       <->      local <-> external        |
//!   tensor       <->                                 |
//!   backend-native                                  v
//!                                            provider adapter
//!
//! * only when the provider/backend explicitly supports the operation.
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - migration plans;
//! - migration policy;
//! - migration requirements;
//! - source/destination descriptors;
//! - capability negotiation;
//! - resource estimation;
//! - validation before allocation;
//! - migration lifecycle;
//! - transactional commit/rollback semantics;
//! - migration receipts/provenance;
//! - deterministic migration planning;
//! - cancellation boundaries;
//! - provider-neutral migration traits;
//! - migration graph/path selection;
//! - lossless/lossy conversion policy;
//! - fidelity/tolerance requirements;
//! - QPU/backend restrictions.
//!
//! It does NOT own:
//!
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer algorithms;
//! - tensor-network algorithms;
//! - GPU kernels;
//! - distributed communication;
//! - hardware provider SDKs;
//! - routing;
//! - scheduling;
//! - compiler parsing;
//! - QEC decoding;
//! - benchmark protocols;
//! - serialization formats.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Integration contract
//!
//! Foundational modules:
//!
//! ```text
//! types.rs
//! errors.rs
//! state.rs
//! limits.rs
//! representation.rs
//! allocator.rs
//! budget.rs
//! reservation.rs
//! coherence.rs
//! synchronization.rs
//! ```
//!
//! Consumers:
//!
//! ```text
//! state_vector.rs
//! density_matrix.rs
//! stabilizer.rs
//! sparse.rs
//! tensor_network.rs
//! backend_state.rs
//! gpu.rs
//! distributed.rs
//! snapshot.rs
//! checkpoint.rs
//! diagnostics.rs
//! telemetry.rs
//! runtime
//! hardware
//! QEC
//! benchmarking
//! ```
//!
//! # Important completion invariant
//!
//! Later memory modules must implement the traits defined here rather than
//! modifying this file merely to add another representation or hardware
//! provider.
//!
//! Adding a new representation should normally require implementing a
//! migration capability/provider, not changing the migration engine.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This module intentionally contains no unsafe Rust.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! Provider implementations are also expected to expose safe Rust contracts.
//!
//! # Provider neutrality
//!
//! This file must never import:
//!
//! - IBM SDKs;
//! - IonQ SDKs;
//! - Quantinuum SDKs;
//! - Rigetti SDKs;
//! - IQM SDKs;
//! - Pasqal SDKs;
//! - D-Wave SDKs;
//! - AWS Braket SDKs;
//! - Google/Cirq SDKs;
//! - CUDA APIs;
//! - ROCm APIs;
//! - Metal APIs;
//! - vendor-specific device-pointer types.
//!
//! Such integrations belong below `quantum::hardware::adapters` or the
//! appropriate memory provider.
//!
//! # Quantum correctness
//!
//! Migration policy distinguishes:
//!
//! - exact/lossless conversion;
//! - numerically equivalent conversion;
//! - fidelity-bounded conversion;
//! - intentionally lossy conversion;
//! - execution-only migration.
//!
//! The migration engine MUST NOT silently perform a lossy conversion.
//!
//! # Resource correctness
//!
//! Migration MUST follow:
//!
//! ```text
//! describe
//!    |
//!    v
//! validate
//!    |
//!    v
//! estimate
//!    |
//!    v
//! reserve
//!    |
//!    v
//! prepare
//!    |
//!    v
//! transfer/convert
//!    |
//!    v
//! verify
//!    |
//!    v
//! commit
//! ```
//!
//! If any stage before commit fails, the destination must not become the
//! authoritative state.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::time::Duration;

use super::errors::MemoryError;
use super::state::{
    StateExecutionDomain,
    StateRepresentationName,
    StateStorageLocation,
};
use super::types::{
    ByteCount,
    MemoryId,
    QubitCount,
    StateId,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable migration schema identifier.
pub const MIGRATION_SCHEMA_ID: &str = "zamani.quantum.memory.migration";

/// Semantic major version of this migration contract.
pub const MIGRATION_SCHEMA_MAJOR: u16 = 1;

/// Semantic minor version of this migration contract.
pub const MIGRATION_SCHEMA_MINOR: u16 = 0;

/// Semantic patch version of this migration contract.
pub const MIGRATION_SCHEMA_PATCH: u16 = 0;

/// Complete migration schema version.
pub const MIGRATION_SCHEMA_VERSION: (u16, u16, u16) = (
    MIGRATION_SCHEMA_MAJOR,
    MIGRATION_SCHEMA_MINOR,
    MIGRATION_SCHEMA_PATCH,
);

/// Default maximum migration path length.
///
/// This prevents pathological provider/path searches.
pub const DEFAULT_MAX_PATH_LENGTH: usize = 16;

/// Default maximum number of candidate migration paths considered.
pub const DEFAULT_MAX_CANDIDATE_PATHS: usize = 64;

/// Default numerical tolerance for migration verification.
///
/// This is only a default. Production callers should normally provide an
/// explicit numerical policy appropriate to the representation and workload.
pub const DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance.
pub const DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Result aliases
// =============================================================================

/// Canonical result type for migration operations.
pub type MigrationResult<T> = Result<T, MemoryError>;

// =============================================================================
// Migration identifiers
// =============================================================================

/// Stable identifier for a migration operation.
///
/// This identifier is intentionally independent of `OperationId` because a
/// migration is memory infrastructure, not a quantum-program operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MigrationId(u64);

impl MigrationId {
    /// Creates a migration identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns whether the identifier is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for MigrationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "migration-{}", self.0)
    }
}

/// Stable identifier for a migration transaction.
///
/// A transaction ID is distinct from a migration ID because retries or
/// recovery attempts may create multiple transactions for the same logical
/// migration request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MigrationTransactionId(u64);

impl MigrationTransactionId {
    /// Creates a transaction identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for MigrationTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "migration-tx-{}", self.0)
    }
}

// =============================================================================
// Migration semantics
// =============================================================================

/// Semantic class of a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MigrationKind {
    /// Convert one in-memory quantum-state representation to another.
    RepresentationConversion,

    /// Move a representation between storage locations.
    StorageTransfer,

    /// Convert and move at the same time.
    RepresentationAndStorage,

    /// Move a backend-native resource without requiring state extraction.
    BackendNativeTransfer,

    /// Transfer a resource between distributed execution domains.
    DistributedTransfer,

    /// Move a workload/state into a provider-managed execution resource.
    ExecutionHandoff,

    /// Provider-specific migration whose semantics are opaque to the core.
    ProviderDefined,
}

impl MigrationKind {
    /// Returns whether this kind can potentially change representation.
    pub const fn changes_representation(self) -> bool {
        matches!(
            self,
            Self::RepresentationConversion
                | Self::RepresentationAndStorage
                | Self::ExecutionHandoff
                | Self::ProviderDefined
        )
    }

    /// Returns whether this kind can potentially change storage location.
    pub const fn changes_storage(self) -> bool {
        matches!(
            self,
            Self::StorageTransfer
                | Self::RepresentationAndStorage
                | Self::BackendNativeTransfer
                | Self::DistributedTransfer
                | Self::ExecutionHandoff
                | Self::ProviderDefined
        )
    }
}

impl fmt::Display for MigrationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::RepresentationConversion => "representation_conversion",
            Self::StorageTransfer => "storage_transfer",
            Self::RepresentationAndStorage => "representation_and_storage",
            Self::BackendNativeTransfer => "backend_native_transfer",
            Self::DistributedTransfer => "distributed_transfer",
            Self::ExecutionHandoff => "execution_handoff",
            Self::ProviderDefined => "provider_defined",
        };

        f.write_str(value)
    }
}

/// Whether a migration is allowed to change the mathematical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FidelityRequirement {
    /// Exact representation semantics are required.
    Exact,

    /// Numerically equivalent state is required within the supplied tolerance.
    NumericallyEquivalent,

    /// Destination fidelity must meet the supplied fidelity threshold.
    FidelityBounded,

    /// Approximation is allowed according to the provider's declared policy.
    Approximate,

    /// No state equivalence is requested because this is an execution handoff.
    ExecutionOnly,
}

impl FidelityRequirement {
    /// Returns whether a state comparison is required.
    pub const fn requires_state_verification(self) -> bool {
        !matches!(self, Self::ExecutionOnly)
    }

    /// Returns whether approximation is explicitly allowed.
    pub const fn allows_approximation(self) -> bool {
        matches!(self, Self::Approximate | Self::FidelityBounded)
    }
}

impl fmt::Display for FidelityRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Exact => "exact",
            Self::NumericallyEquivalent => "numerically_equivalent",
            Self::FidelityBounded => "fidelity_bounded",
            Self::Approximate => "approximate",
            Self::ExecutionOnly => "execution_only",
        };

        f.write_str(value)
    }
}

/// Policy controlling whether a migration may destroy or invalidate the
/// source after a successful commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SourceRetention {
    /// Keep the source unchanged.
    Keep,

    /// The provider may release source resources after successful commit.
    ReleaseAfterCommit,

    /// Source ownership is transferred to the destination/provider.
    TransferOwnership,
}

impl fmt::Display for SourceRetention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Keep => "keep",
            Self::ReleaseAfterCommit => "release_after_commit",
            Self::TransferOwnership => "transfer_ownership",
        };

        f.write_str(value)
    }
}

/// Policy controlling whether a migration may pass through intermediate
/// representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathPolicy {
    /// Only a direct provider-supported migration is allowed.
    DirectOnly,

    /// Intermediate representations are allowed.
    AllowIntermediate,

    /// Intermediate representations are allowed, but only when every step
    /// satisfies the requested fidelity/resource constraints.
    AllowValidatedIntermediate,
}

impl fmt::Display for PathPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::DirectOnly => "direct_only",
            Self::AllowIntermediate => "allow_intermediate",
            Self::AllowValidatedIntermediate => "allow_validated_intermediate",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Migration endpoint
// =============================================================================

/// Provider-neutral description of a migration endpoint.
///
/// This structure intentionally contains no raw pointer, device pointer,
/// provider SDK object, network socket, or credential.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MigrationEndpoint {
    /// Logical state identity when one exists.
    state_id: Option<StateId>,

    /// Owning memory resource when one exists.
    memory_id: Option<MemoryId>,

    /// Number of logical qubits represented by the endpoint.
    qubit_count: QubitCount,

    /// State representation name.
    representation: StateRepresentationName,

    /// Storage location.
    storage: StateStorageLocation,

    /// Execution domain.
    execution_domain: StateExecutionDomain,

    /// Optional provider/backend identity.
    ///
    /// This is an opaque public identifier, never a credential.
    provider_id: Option<String>,

    /// Optional backend/device identity.
    ///
    /// This is metadata only and must not contain secrets.
    backend_id: Option<String>,
}

impl MigrationEndpoint {
    /// Maximum provider identifier length.
    pub const MAX_PROVIDER_ID_LENGTH: usize = 256;

    /// Maximum backend identifier length.
    pub const MAX_BACKEND_ID_LENGTH: usize = 256;

    /// Creates a local endpoint.
    pub fn new(
        qubit_count: QubitCount,
        representation: StateRepresentationName,
        storage: StateStorageLocation,
        execution_domain: StateExecutionDomain,
    ) -> MigrationResult<Self> {
        if qubit_count.is_zero() {
            return Err(invalid_argument("migration endpoint must contain at least one qubit"));
        }

        if storage.is_external()
            && !matches!(
                execution_domain,
                StateExecutionDomain::Qpu
                    | StateExecutionDomain::RemoteSimulator
                    | StateExecutionDomain::HardwareEmulator
                    | StateExecutionDomain::Custom
            )
        {
            return Err(invalid_argument(
                "external/opaque storage requires an external-capable execution domain",
            ));
        }

        Ok(Self {
            state_id: None,
            memory_id: None,
            qubit_count,
            representation,
            storage,
            execution_domain,
            provider_id: None,
            backend_id: None,
        })
    }

    /// Attaches a state identity.
    pub fn with_state_id(mut self, state_id: StateId) -> Self {
        self.state_id = Some(state_id);
        self
    }

    /// Attaches a memory-resource identity.
    pub fn with_memory_id(mut self, memory_id: MemoryId) -> Self {
        self.memory_id = Some(memory_id);
        self
    }

    /// Attaches a provider identifier.
    pub fn with_provider_id(mut self, provider_id: impl Into<String>) -> MigrationResult<Self> {
        let value = provider_id.into();
        validate_identifier(&value, Self::MAX_PROVIDER_ID_LENGTH, "provider")?;
        self.provider_id = Some(value);
        Ok(self)
    }

    /// Attaches a backend/device identifier.
    pub fn with_backend_id(mut self, backend_id: impl Into<String>) -> MigrationResult<Self> {
        let value = backend_id.into();
        validate_identifier(&value, Self::MAX_BACKEND_ID_LENGTH, "backend")?;
        self.backend_id = Some(value);
        Ok(self)
    }

    /// Returns the state identity.
    pub const fn state_id(&self) -> Option<StateId> {
        self.state_id
    }

    /// Returns the memory identity.
    pub const fn memory_id(&self) -> Option<MemoryId> {
        self.memory_id
    }

    /// Returns the qubit count.
    pub const fn qubit_count(&self) -> QubitCount {
        self.qubit_count
    }

    /// Returns the representation.
    pub fn representation(&self) -> &StateRepresentationName {
        &self.representation
    }

    /// Returns the storage location.
    pub const fn storage(&self) -> StateStorageLocation {
        self.storage
    }

    /// Returns the execution domain.
    pub const fn execution_domain(&self) -> StateExecutionDomain {
        self.execution_domain
    }

    /// Returns the provider identifier, if present.
    pub fn provider_id(&self) -> Option<&str> {
        self.provider_id.as_deref()
    }

    /// Returns the backend identifier, if present.
    pub fn backend_id(&self) -> Option<&str> {
        self.backend_id.as_deref()
    }

    /// Returns whether this endpoint is provider-owned.
    pub const fn is_provider_owned(&self) -> bool {
        self.storage.is_external()
    }

    /// Returns whether the endpoint is a physical-QPU endpoint.
    pub const fn is_qpu(&self) -> bool {
        self.execution_domain.is_qpu()
    }

    /// Returns whether two endpoints represent the same abstract location.
    pub fn same_location_as(&self, other: &Self) -> bool {
        self.storage == other.storage
            && self.execution_domain == other.execution_domain
            && self.provider_id == other.provider_id
            && self.backend_id == other.backend_id
    }

    /// Validates endpoint consistency.
    pub fn validate(&self) -> MigrationResult<()> {
        if self.qubit_count.is_zero() {
            return Err(invalid_argument("migration endpoint has zero qubits"));
        }

        if self.is_qpu() && !self.is_provider_owned() {
            return Err(invalid_argument(
                "a QPU endpoint must use provider-owned or opaque storage",
            ));
        }

        if self.is_provider_owned()
            && self.provider_id.is_none()
            && self.execution_domain != StateExecutionDomain::Custom
        {
            return Err(invalid_argument(
                "provider-owned endpoint requires a provider identifier",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Migration requirements
// =============================================================================

/// Numerical tolerance used during migration verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MigrationTolerance {
    /// Absolute error tolerance.
    pub absolute: f64,

    /// Relative error tolerance.
    pub relative: f64,

    /// Optional minimum fidelity for fidelity-based verification.
    pub minimum_fidelity: Option<f64>,
}

impl MigrationTolerance {
    /// Creates a tolerance configuration.
    pub const fn new(absolute: f64, relative: f64) -> Self {
        Self {
            absolute,
            relative,
            minimum_fidelity: None,
        }
    }

    /// Creates the default tolerance.
    pub const fn default_policy() -> Self {
        Self::new(
            DEFAULT_ABSOLUTE_TOLERANCE,
            DEFAULT_RELATIVE_TOLERANCE,
        )
    }

    /// Sets the minimum required fidelity.
    pub const fn with_minimum_fidelity(mut self, fidelity: f64) -> Self {
        self.minimum_fidelity = Some(fidelity);
        self
    }

    /// Validates the tolerance.
    pub fn validate(self) -> MigrationResult<()> {
        if !self.absolute.is_finite()
            || !self.relative.is_finite()
            || self.absolute < 0.0
            || self.relative < 0.0
        {
            return Err(invalid_argument(
                "migration tolerances must be finite and non-negative",
            ));
        }

        if let Some(fidelity) = self.minimum_fidelity {
            if !fidelity.is_finite() || !(0.0..=1.0).contains(&fidelity) {
                return Err(invalid_argument(
                    "minimum migration fidelity must be finite and within [0, 1]",
                ));
            }
        }

        Ok(())
    }
}

/// Resource estimate for one migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationResourceEstimate {
    /// Temporary bytes required during migration.
    pub temporary_bytes: ByteCount,

    /// Destination bytes required after migration.
    pub destination_bytes: ByteCount,

    /// Total additional reservation required before execution.
    pub reservation_bytes: ByteCount,

    /// Estimated peak additional bytes.
    pub peak_additional_bytes: ByteCount,

    /// Whether the migration requires distributed communication.
    pub requires_network: bool,

    /// Whether the migration requires accelerator/device resources.
    pub requires_device: bool,

    /// Whether the migration requires provider/backend resources.
    pub requires_backend: bool,
}

impl MigrationResourceEstimate {
    /// Creates a zero resource estimate.
    pub const fn zero() -> Self {
        Self {
            temporary_bytes: ByteCount::ZERO,
            destination_bytes: ByteCount::ZERO,
            reservation_bytes: ByteCount::ZERO,
            peak_additional_bytes: ByteCount::ZERO,
            requires_network: false,
            requires_device: false,
            requires_backend: false,
        }
    }

    /// Returns the total persistent + temporary reservation.
    pub const fn total_reserved(self) -> ByteCount {
        self.reservation_bytes
    }
}

// =============================================================================
// Migration request
// =============================================================================

/// Complete immutable migration request.
#[derive(Debug, Clone)]
pub struct MigrationRequest {
    /// Stable logical migration identity.
    id: MigrationId,

    /// Source endpoint.
    source: MigrationEndpoint,

    /// Destination endpoint.
    destination: MigrationEndpoint,

    /// Migration semantic kind.
    kind: MigrationKind,

    /// Fidelity requirement.
    fidelity: FidelityRequirement,

    /// Numerical tolerance.
    tolerance: MigrationTolerance,

    /// Source retention policy.
    source_retention: SourceRetention,

    /// Path selection policy.
    path_policy: PathPolicy,

    /// Maximum permitted path length.
    max_path_length: usize,

    /// Maximum number of candidate paths considered.
    max_candidate_paths: usize,

    /// Whether the migration may require an intermediate temporary state.
    allow_temporary_storage: bool,

    /// Whether provider/backend execution is allowed.
    allow_backend_execution: bool,

    /// Whether distributed communication is allowed.
    allow_distributed: bool,

    /// Whether device/accelerator resources are allowed.
    allow_device: bool,

    /// Optional deadline.
    deadline: Option<Duration>,

    /// Optional caller-visible operation label.
    label: Option<String>,
}

impl MigrationRequest {
    /// Creates a migration request with safe defaults.
    pub fn new(
        id: MigrationId,
        source: MigrationEndpoint,
        destination: MigrationEndpoint,
        kind: MigrationKind,
    ) -> MigrationResult<Self> {
        source.validate()?;
        destination.validate()?;

        if source.qubit_count() != destination.qubit_count() {
            return Err(invalid_argument(
                "source and destination qubit counts must match",
            ));
        }

        if source.same_location_as(&destination)
            && source.representation() == destination.representation()
        {
            return Err(invalid_argument(
                "migration source and destination are identical",
            ));
        }

        Ok(Self {
            id,
            source,
            destination,
            kind,
            fidelity: FidelityRequirement::Exact,
            tolerance: MigrationTolerance::default_policy(),
            source_retention: SourceRetention::Keep,
            path_policy: PathPolicy::DirectOnly,
            max_path_length: DEFAULT_MAX_PATH_LENGTH,
            max_candidate_paths: DEFAULT_MAX_CANDIDATE_PATHS,
            allow_temporary_storage: true,
            allow_backend_execution: true,
            allow_distributed: true,
            allow_device: true,
            deadline: None,
            label: None,
        })
    }

    /// Sets fidelity semantics.
    pub const fn with_fidelity(mut self, fidelity: FidelityRequirement) -> Self {
        self.fidelity = fidelity;
        self
    }

    /// Sets numerical tolerances.
    pub const fn with_tolerance(mut self, tolerance: MigrationTolerance) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Sets source-retention policy.
    pub const fn with_source_retention(mut self, retention: SourceRetention) -> Self {
        self.source_retention = retention;
        self
    }

    /// Sets path policy.
    pub const fn with_path_policy(mut self, policy: PathPolicy) -> Self {
        self.path_policy = policy;
        self
    }

    /// Sets maximum path length.
    pub fn with_max_path_length(mut self, value: usize) -> MigrationResult<Self> {
        if value == 0 || value > DEFAULT_MAX_PATH_LENGTH {
            return Err(invalid_argument(
                "migration path length is outside the supported safety bound",
            ));
        }

        self.max_path_length = value;
        Ok(self)
    }

    /// Sets maximum candidate paths.
    pub fn with_max_candidate_paths(mut self, value: usize) -> MigrationResult<Self> {
        if value == 0 || value > DEFAULT_MAX_CANDIDATE_PATHS {
            return Err(invalid_argument(
                "migration candidate-path limit is outside the supported safety bound",
            ));
        }

        self.max_candidate_paths = value;
        Ok(self)
    }

    /// Enables/disables temporary storage.
    pub const fn with_temporary_storage(mut self, enabled: bool) -> Self {
        self.allow_temporary_storage = enabled;
        self
    }

    /// Enables/disables provider/backend execution.
    pub const fn with_backend_execution(mut self, enabled: bool) -> Self {
        self.allow_backend_execution = enabled;
        self
    }

    /// Enables/disables distributed migration.
    pub const fn with_distributed(mut self, enabled: bool) -> Self {
        self.allow_distributed = enabled;
        self
    }

    /// Enables/disables accelerator/device migration.
    pub const fn with_device(mut self, enabled: bool) -> Self {
        self.allow_device = enabled;
        self
    }

    /// Sets an optional deadline.
    pub const fn with_deadline(mut self, deadline: Option<Duration>) -> Self {
        self.deadline = deadline;
        self
    }

    /// Sets a caller-visible label.
    pub fn with_label(mut self, label: impl Into<String>) -> MigrationResult<Self> {
        let value = label.into();

        validate_identifier(&value, 256, "migration label")?;

        self.label = Some(value);
        Ok(self)
    }

    /// Returns the migration ID.
    pub const fn id(&self) -> MigrationId {
        self.id
    }

    /// Returns the source endpoint.
    pub fn source(&self) -> &MigrationEndpoint {
        &self.source
    }

    /// Returns the destination endpoint.
    pub fn destination(&self) -> &MigrationEndpoint {
        &self.destination
    }

    /// Returns the migration kind.
    pub const fn kind(&self) -> MigrationKind {
        self.kind
    }

    /// Returns the fidelity requirement.
    pub const fn fidelity(&self) -> FidelityRequirement {
        self.fidelity
    }

    /// Returns the numerical tolerance.
    pub const fn tolerance(&self) -> MigrationTolerance {
        self.tolerance
    }

    /// Returns source-retention semantics.
    pub const fn source_retention(&self) -> SourceRetention {
        self.source_retention
    }

    /// Returns path policy.
    pub const fn path_policy(&self) -> PathPolicy {
        self.path_policy
    }

    /// Returns whether temporary storage is allowed.
    pub const fn allows_temporary_storage(&self) -> bool {
        self.allow_temporary_storage
    }

    /// Returns whether backend execution is allowed.
    pub const fn allows_backend_execution(&self) -> bool {
        self.allow_backend_execution
    }

    /// Returns whether distributed migration is allowed.
    pub const fn allows_distributed(&self) -> bool {
        self.allow_distributed
    }

    /// Returns whether device migration is allowed.
    pub const fn allows_device(&self) -> bool {
        self.allow_device
    }

    /// Returns the optional deadline.
    pub const fn deadline(&self) -> Option<Duration> {
        self.deadline
    }

    /// Returns the optional label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Validates the complete request.
    pub fn validate(&self) -> MigrationResult<()> {
        if self.id.is_zero() {
            return Err(invalid_argument("migration ID must be non-zero"));
        }

        self.source.validate()?;
        self.destination.validate()?;
        self.tolerance.validate()?;

        if self.source.qubit_count() != self.destination.qubit_count() {
            return Err(invalid_argument(
                "migration source and destination qubit counts differ",
            ));
        }

        if self.fidelity.requires_state_verification()
            && (self.source.is_qpu() || self.destination.is_qpu())
        {
            return Err(invalid_argument(
                "state-equivalence verification cannot be required for an opaque QPU endpoint",
            ));
        }

        if !self.allow_backend_execution
            && (self.source.is_provider_owned() || self.destination.is_provider_owned())
        {
            return Err(invalid_argument(
                "backend execution is disabled for a provider-owned migration",
            ));
        }

        if !self.allow_distributed
            && (self.source.storage() == StateStorageLocation::Distributed
                || self.destination.storage() == StateStorageLocation::Distributed)
        {
            return Err(invalid_argument(
                "distributed migration is disabled by policy",
            ));
        }

        if !self.allow_device
            && (self.source.storage().is_device() || self.destination.storage().is_device())
        {
            return Err(invalid_argument(
                "device migration is disabled by policy",
            ));
        }

        if !self.allow_temporary_storage && self.path_policy != PathPolicy::DirectOnly {
            return Err(invalid_argument(
                "intermediate migration paths require temporary storage",
            ));
        }

        if matches!(self.source_retention, SourceRetention::TransferOwnership)
            && self.source.is_provider_owned()
        {
            return Err(invalid_argument(
                "ownership transfer from provider-owned state requires explicit provider semantics",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Migration capabilities
// =============================================================================

/// Capability advertised by a migration provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MigrationCapabilities(u64);

impl MigrationCapabilities {
    /// Direct representation conversion.
    pub const REPRESENTATION_CONVERSION: u64 = 1 << 0;

    /// Host/device transfer.
    pub const STORAGE_TRANSFER: u64 = 1 << 1;

    /// Distributed transfer.
    pub const DISTRIBUTED_TRANSFER: u64 = 1 << 2;

    /// Backend-native transfer.
    pub const BACKEND_NATIVE_TRANSFER: u64 = 1 << 3;

    /// Remote execution handoff.
    pub const EXECUTION_HANDOFF: u64 = 1 << 4;

    /// Exact/lossless conversion.
    pub const EXACT_CONVERSION: u64 = 1 << 5;

    /// Numerically verified conversion.
    pub const NUMERICAL_VERIFICATION: u64 = 1 << 6;

    /// Fidelity-bounded conversion.
    pub const FIDELITY_BOUNDED_CONVERSION: u64 = 1 << 7;

    /// Approximate conversion.
    pub const APPROXIMATE_CONVERSION: u64 = 1 << 8;

    /// Transactional rollback.
    pub const TRANSACTIONAL_ROLLBACK: u64 = 1 << 9;

    /// Cancellation before commit.
    pub const CANCELLATION: u64 = 1 << 10;

    /// Device migration.
    pub const DEVICE_MIGRATION: u64 = 1 << 11;

    /// Provider-owned opaque state.
    pub const OPAQUE_STATE: u64 = 1 << 12;

    /// Creates an empty capability set.
    pub const const fn empty() -> Self {
        Self(0)
    }

    /// Creates a capability set from raw bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns raw bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Adds a capability.
    pub const fn with(mut self, capability: u64) -> Self {
        self.0 |= capability;
        self
    }

    /// Tests a capability.
    pub const fn contains(self, capability: u64) -> bool {
        self.0 & capability == capability
    }

    /// Returns whether all requested capabilities are present.
    pub const fn contains_all(self, capabilities: u64) -> bool {
        self.contains(capabilities)
    }
}

impl Default for MigrationCapabilities {
    fn default() -> Self {
        Self::empty()
    }
}

// =============================================================================
// Migration cost
// =============================================================================

/// Relative migration cost used for deterministic path selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationCost {
    /// Abstract integer cost.
    ///
    /// Providers may choose any deterministic scale. The value is not a
    /// duration and must never be interpreted as one.
    pub value: u64,

    /// Estimated transferred bytes.
    pub transferred_bytes: ByteCount,

    /// Estimated temporary bytes.
    pub temporary_bytes: ByteCount,

    /// Whether network resources are required.
    pub network_required: bool,

    /// Whether accelerator/device resources are required.
    pub device_required: bool,

    /// Whether provider/backend resources are required.
    pub backend_required: bool,
}

impl MigrationCost {
    /// Creates a zero cost.
    pub const fn zero() -> Self {
        Self {
            value: 0,
            transferred_bytes: ByteCount::ZERO,
            temporary_bytes: ByteCount::ZERO,
            network_required: false,
            device_required: false,
            backend_required: false,
        }
    }

    /// Creates a simple scalar cost.
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            transferred_bytes: ByteCount::ZERO,
            temporary_bytes: ByteCount::ZERO,
            network_required: false,
            device_required: false,
            backend_required: false,
        }
    }

    /// Checked cost addition.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let value = match self.value.checked_add(rhs.value) {
            Some(value) => value,
            None => return None,
        };

        let transferred_bytes = match self.transferred_bytes.checked_add(rhs.transferred_bytes) {
            Some(value) => value,
            None => return None,
        };

        let temporary_bytes = match self.temporary_bytes.checked_add(rhs.temporary_bytes) {
            Some(value) => value,
            None => return None,
        };

        Some(Self {
            value,
            transferred_bytes,
            temporary_bytes,
            network_required: self.network_required || rhs.network_required,
            device_required: self.device_required || rhs.device_required,
            backend_required: self.backend_required || rhs.backend_required,
        })
    }
}

// =============================================================================
// Migration provider contract
// =============================================================================

/// Provider-neutral prepared migration handle.
///
/// A prepared migration must not become authoritative until `commit` is
/// called by the migration engine.
pub trait PreparedMigration: Send {
    /// Returns the resource estimate reserved by the prepared migration.
    fn resource_estimate(&self) -> MigrationResourceEstimate;

    /// Returns whether the migration has already been committed.
    fn is_committed(&self) -> bool;

    /// Returns whether rollback is still possible.
    fn can_rollback(&self) -> bool;
}

/// Provider-neutral migration provider.
///
/// This is the critical integration boundary for all state representations,
/// GPU implementations, distributed memory providers, simulators and QPUs.
///
/// Implementors must not expose unsafe pointers or vendor SDK types through
/// this trait.
///
/// The provider owns the actual representation conversion/transfer. The
/// migration engine owns policy, validation and lifecycle ordering.
pub trait MigrationProvider: Send + Sync {
    /// Human-readable provider name.
    fn name(&self) -> &str;

    /// Capabilities exposed by this provider.
    fn capabilities(&self) -> MigrationCapabilities;

    /// Returns whether the provider can directly satisfy the request.
    fn can_migrate(&self, request: &MigrationRequest) -> MigrationResult<bool>;

    /// Estimates the resources required by a direct migration.
    fn estimate(
        &self,
        request: &MigrationRequest,
    ) -> MigrationResult<MigrationResourceEstimate>;

    /// Returns the relative cost of a direct migration.
    fn cost(&self, request: &MigrationRequest) -> MigrationResult<MigrationCost>;

    /// Prepares a migration.
    ///
    /// Implementations must reserve/prepare destination resources but must not
    /// make the destination authoritative yet.
    fn prepare(
        &self,
        request: &MigrationRequest,
    ) -> MigrationResult<Box<dyn PreparedMigration>>;

    /// Performs the actual transfer/conversion for a prepared migration.
    ///
    /// This may perform CPU, GPU, distributed, simulator, or provider-specific
    /// work. The provider must obey the request's fidelity policy.
    fn execute(
        &self,
        request: &MigrationRequest,
        prepared: &mut dyn PreparedMigration,
    ) -> MigrationResult<()>;

    /// Verifies the prepared destination.
    ///
    /// For opaque QPU/backend resources this may mean provider-level
    /// compatibility/receipt verification rather than state-vector comparison.
    fn verify(
        &self,
        request: &MigrationRequest,
        prepared: &dyn PreparedMigration,
    ) -> MigrationResult<MigrationVerification>;

    /// Commits the prepared destination as authoritative.
    fn commit(
        &self,
        request: &MigrationRequest,
        prepared: Box<dyn PreparedMigration>,
    ) -> MigrationResult<MigrationCommit>;

    /// Rolls back an uncommitted migration.
    fn rollback(
        &self,
        request: &MigrationRequest,
        prepared: Box<dyn PreparedMigration>,
    ) -> MigrationResult<()>;

    /// Cancels a migration when cancellation is still supported.
    ///
    /// The default implementation refuses cancellation rather than claiming
    /// it is safe.
    fn cancel(
        &self,
        _request: &MigrationRequest,
        _prepared: &mut dyn PreparedMigration,
    ) -> MigrationResult<()> {
        Err(invalid_argument(
            "migration provider does not support cancellation",
        ))
    }
}

// =============================================================================
// Verification
// =============================================================================

/// Result of migration verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MigrationVerification {
    /// Provider established exact equivalence.
    Exact,

    /// Provider established numerical equivalence.
    NumericallyEquivalent {
        /// Maximum observed numerical error.
        max_error: f64,
    },

    /// Provider established the requested minimum fidelity.
    FidelitySatisfied {
        /// Measured/estimated fidelity.
        fidelity: f64,
    },

    /// Provider verified execution/backend compatibility without state access.
    ProviderVerified,

    /// Verification was intentionally skipped under explicit policy.
    Skipped,
}

impl MigrationVerification {
    /// Returns whether verification satisfies the supplied request.
    pub fn satisfies(
        self,
        request: &MigrationRequest,
    ) -> MigrationResult<()> {
        match request.fidelity() {
            FidelityRequirement::Exact => {
                if matches!(self, Self::Exact) {
                    Ok(())
                } else {
                    Err(invalid_argument(
                        "migration did not provide the exact verification required",
                    ))
                }
            }

            FidelityRequirement::NumericallyEquivalent => match self {
                Self::Exact => Ok(()),
                Self::NumericallyEquivalent { max_error } => {
                    if max_error <= request.tolerance().absolute {
                        Ok(())
                    } else {
                        Err(invalid_argument(
                            "migration numerical error exceeds requested tolerance",
                        ))
                    }
                }
                _ => Err(invalid_argument(
                    "migration did not provide numerical verification",
                )),
            },

            FidelityRequirement::FidelityBounded => match self {
                Self::Exact | Self::NumericallyEquivalent { .. } => Ok(()),
                Self::FidelitySatisfied { fidelity } => {
                    let required = request
                        .tolerance()
                        .minimum_fidelity
                        .unwrap_or(1.0);

                    if fidelity >= required {
                        Ok(())
                    } else {
                        Err(invalid_argument(
                            "migration fidelity is below the requested threshold",
                        ))
                    }
                }
                _ => Err(invalid_argument(
                    "migration did not provide fidelity verification",
                )),
            },

            FidelityRequirement::Approximate => {
                if matches!(
                    self,
                    Self::Exact
                        | Self::NumericallyEquivalent { .. }
                        | Self::FidelitySatisfied { .. }
                ) {
                    Ok(())
                } else {
                    Err(invalid_argument(
                        "migration provider did not provide an acceptable verification result",
                    ))
                }
            }

            FidelityRequirement::ExecutionOnly => {
                if matches!(self, Self::ProviderVerified | Self::Skipped) {
                    Ok(())
                } else {
                    Ok(())
                }
            }
        }
    }
}

// =============================================================================
// Commit receipt
// =============================================================================

/// Result of a successful migration commit.
#[derive(Debug, Clone)]
pub struct MigrationCommit {
    /// Migration identifier.
    migration_id: MigrationId,

    /// Destination endpoint.
    destination: MigrationEndpoint,

    /// Whether source ownership was released.
    source_released: bool,

    /// Whether the provider considers the destination authoritative.
    destination_authoritative: bool,

    /// Actual/declared resource usage.
    resource_estimate: MigrationResourceEstimate,

    /// Verification result.
    verification: MigrationVerification,
}

impl MigrationCommit {
    /// Creates a commit receipt.
    pub fn new(
        migration_id: MigrationId,
        destination: MigrationEndpoint,
        source_released: bool,
        destination_authoritative: bool,
        resource_estimate: MigrationResourceEstimate,
        verification: MigrationVerification,
    ) -> MigrationResult<Self> {
        if migration_id.is_zero() {
            return Err(invalid_argument(
                "migration commit requires a non-zero migration ID",
            ));
        }

        if !destination_authoritative {
            return Err(invalid_argument(
                "successful migration commit must make destination authoritative",
            ));
        }

        Ok(Self {
            migration_id,
            destination,
            source_released,
            destination_authoritative,
            resource_estimate,
            verification,
        })
    }

    /// Returns migration ID.
    pub const fn migration_id(&self) -> MigrationId {
        self.migration_id
    }

    /// Returns destination.
    pub fn destination(&self) -> &MigrationEndpoint {
        &self.destination
    }

    /// Returns whether source was released.
    pub const fn source_released(&self) -> bool {
        self.source_released
    }

    /// Returns whether destination is authoritative.
    pub const fn destination_authoritative(&self) -> bool {
        self.destination_authoritative
    }

    /// Returns resource estimate.
    pub const fn resource_estimate(&self) -> MigrationResourceEstimate {
        self.resource_estimate
    }

    /// Returns verification.
    pub const fn verification(&self) -> MigrationVerification {
        self.verification
    }
}

// =============================================================================
// Migration status
// =============================================================================

/// Lifecycle state of a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MigrationStatus {
    /// Request has been created.
    Created,

    /// Request passed validation.
    Validated,

    /// Provider path has been selected.
    Planned,

    /// Resources have been reserved/prepared.
    Prepared,

    /// Transfer/conversion is executing.
    Executing,

    /// Destination has been verified but not committed.
    Verified,

    /// Destination is authoritative.
    Committed,

    /// Migration was rolled back.
    RolledBack,

    /// Migration was cancelled.
    Cancelled,

    /// Migration failed.
    Failed,
}

impl MigrationStatus {
    /// Returns whether the migration is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Committed
                | Self::RolledBack
                | Self::Cancelled
                | Self::Failed
        )
    }

    /// Returns whether the destination may be authoritative.
    pub const fn destination_may_be_authoritative(self) -> bool {
        matches!(self, Self::Committed)
    }
}

// =============================================================================
// Migration plan
// =============================================================================

/// One provider-backed step in a migration path.
#[derive(Debug, Clone)]
pub struct MigrationStep {
    /// Provider name.
    provider_name: String,

    /// Source endpoint for this step.
    source: MigrationEndpoint,

    /// Destination endpoint for this step.
    destination: MigrationEndpoint,

    /// Cost.
    cost: MigrationCost,

    /// Resource estimate.
    resources: MigrationResourceEstimate,
}

impl MigrationStep {
    /// Creates a migration step.
    pub fn new(
        provider_name: impl Into<String>,
        source: MigrationEndpoint,
        destination: MigrationEndpoint,
        cost: MigrationCost,
        resources: MigrationResourceEstimate,
    ) -> MigrationResult<Self> {
        let provider_name = provider_name.into();

        validate_identifier(&provider_name, 256, "migration provider")?;

        source.validate()?;
        destination.validate()?;

        if source.qubit_count() != destination.qubit_count() {
            return Err(invalid_argument(
                "migration step source and destination qubit counts differ",
            ));
        }

        Ok(Self {
            provider_name,
            source,
            destination,
            cost,
            resources,
        })
    }

    /// Returns provider name.
    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    /// Returns source.
    pub fn source(&self) -> &MigrationEndpoint {
        &self.source
    }

    /// Returns destination.
    pub fn destination(&self) -> &MigrationEndpoint {
        &self.destination
    }

    /// Returns cost.
    pub const fn cost(&self) -> MigrationCost {
        self.cost
    }

    /// Returns resources.
    pub const fn resources(&self) -> MigrationResourceEstimate {
        self.resources
    }
}

/// Complete migration plan.
///
/// The plan is immutable after construction.
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    /// Original request.
    request: MigrationRequest,

    /// Ordered migration steps.
    steps: Vec<MigrationStep>,

    /// Aggregate cost.
    total_cost: MigrationCost,

    /// Aggregate resource estimate.
    total_resources: MigrationResourceEstimate,
}

impl MigrationPlan {
    /// Creates a direct migration plan.
    pub fn direct(
        request: MigrationRequest,
        provider: &dyn MigrationProvider,
    ) -> MigrationResult<Self> {
        request.validate()?;

        if !provider.can_migrate(&request)? {
            return Err(invalid_argument(
                "migration provider cannot satisfy the requested migration",
            ));
        }

        let cost = provider.cost(&request)?;
        let resources = provider.estimate(&request)?;

        let step = MigrationStep::new(
            provider.name(),
            request.source().clone(),
            request.destination().clone(),
            cost,
            resources,
        )?;

        Self::from_steps(request, vec![step])
    }

    /// Creates a plan from validated steps.
    pub fn from_steps(
        request: MigrationRequest,
        steps: Vec<MigrationStep>,
    ) -> MigrationResult<Self> {
        request.validate()?;

        if steps.is_empty() {
            return Err(invalid_argument(
                "migration plan must contain at least one step",
            ));
        }

        if steps.len() > request.max_path_length {
            return Err(invalid_argument(
                "migration plan exceeds the configured maximum path length",
            ));
        }

        if matches!(request.path_policy, PathPolicy::DirectOnly) && steps.len() != 1 {
            return Err(invalid_argument(
                "direct-only migration policy permits exactly one step",
            ));
        }

        validate_path(&request, &steps)?;

        let mut total_cost = MigrationCost::zero();
        let mut total_resources = MigrationResourceEstimate::zero();

        for step in &steps {
            total_cost = total_cost
                .checked_add(step.cost())
                .ok_or_else(|| invalid_argument("migration cost overflow"))?;

            total_resources = combine_resources(total_resources, step.resources())?;
        }

        Ok(Self {
            request,
            steps,
            total_cost,
            total_resources,
        })
    }

    /// Returns the original request.
    pub fn request(&self) -> &MigrationRequest {
        &self.request
    }

    /// Returns migration steps.
    pub fn steps(&self) -> &[MigrationStep] {
        &self.steps
    }

    /// Returns aggregate cost.
    pub const fn total_cost(&self) -> MigrationCost {
        self.total_cost
    }

    /// Returns aggregate resource estimate.
    pub const fn total_resources(&self) -> MigrationResourceEstimate {
        self.total_resources
    }

    /// Returns whether this is a direct migration.
    pub fn is_direct(&self) -> bool {
        self.steps.len() == 1
    }
}

// =============================================================================
// Migration receipt
// =============================================================================

/// Immutable record returned after successful migration.
#[derive(Debug, Clone)]
pub struct MigrationReceipt {
    /// Migration identifier.
    migration_id: MigrationId,

    /// Final status.
    status: MigrationStatus,

    /// Final destination.
    destination: MigrationEndpoint,

    /// Verification result.
    verification: MigrationVerification,

    /// Aggregate resource estimate.
    resources: MigrationResourceEstimate,

    /// Aggregate cost.
    cost: MigrationCost,

    /// Whether source was retained.
    source_retained: bool,

    /// Number of completed provider steps.
    completed_steps: usize,
}

impl MigrationReceipt {
    /// Creates a successful receipt.
    fn committed(
        plan: &MigrationPlan,
        verification: MigrationVerification,
        source_retained: bool,
        completed_steps: usize,
    ) -> Self {
        Self {
            migration_id: plan.request.id(),
            status: MigrationStatus::Committed,
            destination: plan.request.destination().clone(),
            verification,
            resources: plan.total_resources(),
            cost: plan.total_cost(),
            source_retained,
            completed_steps,
        }
    }

    /// Returns migration ID.
    pub const fn migration_id(&self) -> MigrationId {
        self.migration_id
    }

    /// Returns final status.
    pub const fn status(&self) -> MigrationStatus {
        self.status
    }

    /// Returns final destination.
    pub fn destination(&self) -> &MigrationEndpoint {
        &self.destination
    }

    /// Returns verification.
    pub const fn verification(&self) -> MigrationVerification {
        self.verification
    }

    /// Returns resource estimate.
    pub const fn resources(&self) -> MigrationResourceEstimate {
        self.resources
    }

    /// Returns aggregate cost.
    pub const fn cost(&self) -> MigrationCost {
        self.cost
    }

    /// Returns whether the source remains available.
    pub const fn source_retained(&self) -> bool {
        self.source_retained
    }

    /// Returns completed step count.
    pub const fn completed_steps(&self) -> usize {
        self.completed_steps
    }
}

// =============================================================================
// Migration engine
// =============================================================================

/// Provider-neutral migration engine.
///
/// The engine owns orchestration and policy. Providers own the actual
/// representation/storage operation.
pub struct MigrationEngine<'a> {
    providers: &'a [&'a dyn MigrationProvider],
}

impl<'a> MigrationEngine<'a> {
    /// Creates a migration engine over a fixed provider set.
    ///
    /// The provider slice is borrowed and never modified by the engine.
    pub const fn new(providers: &'a [&'a dyn MigrationProvider]) -> Self {
        Self { providers }
    }

    /// Returns the registered provider count.
    pub const fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Finds a direct provider for a request.
    pub fn find_direct_provider(
        &self,
        request: &MigrationRequest,
    ) -> MigrationResult<usize> {
        request.validate()?;

        let mut index = 0usize;

        while index < self.providers.len() {
            let provider = self.providers[index];

            if provider.can_migrate(request)? {
                return Ok(index);
            }

            index += 1;
        }

        Err(invalid_argument(
            "no registered migration provider can satisfy the request",
        ))
    }

    /// Builds a deterministic direct migration plan.
    ///
    /// When several providers support the request, the provider with the
    /// lowest deterministic cost is selected. Ties are broken by provider
    /// name and then registration index.
    pub fn plan_direct(
        &self,
        request: MigrationRequest,
    ) -> MigrationResult<MigrationPlan> {
        request.validate()?;

        let mut best: Option<(usize, MigrationCost)> = None;

        let mut index = 0usize;

        while index < self.providers.len() {
            let provider = self.providers[index];

            if provider.can_migrate(&request)? {
                let cost = provider.cost(&request)?;

                let replace = match best {
                    None => true,
                    Some((best_index, best_cost)) => {
                        cost < best_cost
                            || (cost == best_cost
                                && provider.name()
                                    < self.providers[best_index].name())
                            || (cost == best_cost
                                && provider.name()
                                    == self.providers[best_index].name()
                                && index < best_index)
                    }
                };

                if replace {
                    best = Some((index, cost));
                }
            }

            index += 1;
        }

        let (index, _) = best.ok_or_else(|| {
            invalid_argument(
                "no migration provider can satisfy the requested migration",
            )
        })?;

        MigrationPlan::direct(request, self.providers[index])
    }

    /// Executes a direct migration transactionally.
    ///
    /// The provider's destination is not considered authoritative until its
    /// `commit` operation succeeds.
    pub fn execute_direct(
        &self,
        request: MigrationRequest,
    ) -> MigrationResult<MigrationReceipt> {
        let plan = self.plan_direct(request)?;

        self.execute_plan(&plan)
    }

    /// Executes an already validated migration plan.
    pub fn execute_plan(
        &self,
        plan: &MigrationPlan,
    ) -> MigrationResult<MigrationReceipt> {
        plan.request.validate()?;

        if plan.steps.is_empty() {
            return Err(invalid_argument(
                "cannot execute an empty migration plan",
            ));
        }

        if plan.steps.len() != 1 {
            return Err(invalid_argument(
                "multi-step migration execution requires an explicit chained provider transaction",
            ));
        }

        let step = &plan.steps[0];

        let provider = self
            .providers
            .iter()
            .copied()
            .find(|provider| provider.name() == step.provider_name())
            .ok_or_else(|| invalid_argument("migration plan provider is not registered"))?;

        if !provider.can_migrate(plan.request())? {
            return Err(invalid_argument(
                "migration provider no longer satisfies the migration request",
            ));
        }

        let mut prepared = provider.prepare(plan.request())?;

        let mut status = MigrationStatus::Prepared;

        if status.is_terminal() {
            return Err(invalid_argument(
                "migration entered an invalid terminal state before execution",
            ));
        }

        status = MigrationStatus::Executing;

        if let Err(error) = provider.execute(plan.request(), prepared.as_mut()) {
            let _ = provider.rollback(plan.request(), prepared);
            return Err(error);
        }

        let verification = match provider.verify(plan.request(), prepared.as_ref()) {
            Ok(value) => value,
            Err(error) => {
                let _ = provider.rollback(plan.request(), prepared);
                return Err(error);
            }
        };

        verification.satisfies(plan.request())?;

        status = MigrationStatus::Verified;

        if status != MigrationStatus::Verified {
            let _ = provider.rollback(plan.request(), prepared);
            return Err(invalid_argument(
                "migration could not reach the verification state",
            ));
        }

        let commit = match provider.commit(plan.request(), prepared) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let source_retained = !commit.source_released();

        Ok(MigrationReceipt::committed(
            plan,
            commit.verification(),
            source_retained,
            1,
        ))
    }
}

// =============================================================================
// Path validation
// =============================================================================

fn validate_path(
    request: &MigrationRequest,
    steps: &[MigrationStep],
) -> MigrationResult<()> {
    if steps.is_empty() {
        return Err(invalid_argument(
            "migration path cannot be empty",
        ));
    }

    if steps[0].source() != request.source() {
        return Err(invalid_argument(
            "first migration step does not start at the requested source",
        ));
    }

    let final_step = steps
        .last()
        .ok_or_else(|| invalid_argument("migration path has no final step"))?;

    if final_step.destination() != request.destination() {
        return Err(invalid_argument(
            "last migration step does not end at the requested destination",
        ));
    }

    let mut index = 1usize;

    while index < steps.len() {
        if steps[index - 1].destination() != steps[index].source() {
            return Err(invalid_argument(
                "migration path contains disconnected steps",
            ));
        }

        index += 1;
    }

    if !request.allows_temporary_storage() && steps.len() > 1 {
        return Err(invalid_argument(
            "migration path requires temporary storage but it is disabled",
        ));
    }

    Ok(())
}

// =============================================================================
// Resource arithmetic
// =============================================================================

fn combine_resources(
    left: MigrationResourceEstimate,
    right: MigrationResourceEstimate,
) -> MigrationResult<MigrationResourceEstimate> {
    let temporary_bytes = left
        .temporary_bytes
        .checked_add(right.temporary_bytes)
        .ok_or_else(|| invalid_argument("migration temporary-byte estimate overflow"))?;

    let destination_bytes = left
        .destination_bytes
        .checked_add(right.destination_bytes)
        .ok_or_else(|| invalid_argument("migration destination-byte estimate overflow"))?;

    let reservation_bytes = left
        .reservation_bytes
        .checked_add(right.reservation_bytes)
        .ok_or_else(|| invalid_argument("migration reservation-byte estimate overflow"))?;

    let peak_additional_bytes = left
        .peak_additional_bytes
        .checked_add(right.peak_additional_bytes)
        .ok_or_else(|| invalid_argument("migration peak-byte estimate overflow"))?;

    Ok(MigrationResourceEstimate {
        temporary_bytes,
        destination_bytes,
        reservation_bytes,
        peak_additional_bytes,
        requires_network: left.requires_network || right.requires_network,
        requires_device: left.requires_device || right.requires_device,
        requires_backend: left.requires_backend || right.requires_backend,
    })
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    max_length: usize,
    kind: &str,
) -> MigrationResult<()> {
    if value.is_empty()
        || value.len() > max_length
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(invalid_argument_owned(format!(
            "{kind} identifier is empty, too long, whitespace-padded, or contains control characters",
        )));
    }

    Ok(())
}

fn invalid_argument(message: &'static str) -> MemoryError {
    MemoryError::invalid_argument(message)
}

fn invalid_argument_owned(message: String) -> MemoryError {
    // The canonical error API accepts owned diagnostic context. If the
    // foundational error implementation later changes its internal storage,
    // this migration module remains isolated behind this helper.
    MemoryError::invalid_argument(message)
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPrepared {
        estimate: MigrationResourceEstimate,
        committed: bool,
        rollbackable: bool,
    }

    impl PreparedMigration for MockPrepared {
        fn resource_estimate(&self) -> MigrationResourceEstimate {
            self.estimate
        }

        fn is_committed(&self) -> bool {
            self.committed
        }

        fn can_rollback(&self) -> bool {
            self.rollbackable
        }
    }

    struct MockProvider;

    impl MigrationProvider for MockProvider {
        fn name(&self) -> &str {
            "test-provider"
        }

        fn capabilities(&self) -> MigrationCapabilities {
            MigrationCapabilities::empty()
                .with(MigrationCapabilities::REPRESENTATION_CONVERSION)
                .with(MigrationCapabilities::STORAGE_TRANSFER)
                .with(MigrationCapabilities::EXACT_CONVERSION)
                .with(MigrationCapabilities::TRANSACTIONAL_ROLLBACK)
        }

        fn can_migrate(
            &self,
            request: &MigrationRequest,
        ) -> MigrationResult<bool> {
            Ok(
                request.source().representation()
                    != request.destination().representation()
                    || request.source().storage()
                        != request.destination().storage(),
            )
        }

        fn estimate(
            &self,
            _request: &MigrationRequest,
        ) -> MigrationResult<MigrationResourceEstimate> {
            Ok(MigrationResourceEstimate {
                temporary_bytes: ByteCount::new(1024),
                destination_bytes: ByteCount::new(1024),
                reservation_bytes: ByteCount::new(2048),
                peak_additional_bytes: ByteCount::new(2048),
                requires_network: false,
                requires_device: false,
                requires_backend: false,
            })
        }

        fn cost(
            &self,
            _request: &MigrationRequest,
        ) -> MigrationResult<MigrationCost> {
            Ok(MigrationCost::new(1))
        }

        fn prepare(
            &self,
            request: &MigrationRequest,
        ) -> MigrationResult<Box<dyn PreparedMigration>> {
            Ok(Box::new(MockPrepared {
                estimate: self.estimate(request)?,
                committed: false,
                rollbackable: true,
            }))
        }

        fn execute(
            &self,
            _request: &MigrationRequest,
            _prepared: &mut dyn PreparedMigration,
        ) -> MigrationResult<()> {
            Ok(())
        }

        fn verify(
            &self,
            _request: &MigrationRequest,
            _prepared: &dyn PreparedMigration,
        ) -> MigrationResult<MigrationVerification> {
            Ok(MigrationVerification::Exact)
        }

        fn commit(
            &self,
            request: &MigrationRequest,
            prepared: Box<dyn PreparedMigration>,
        ) -> MigrationResult<MigrationCommit> {
            MigrationCommit::new(
                request.id(),
                request.destination().clone(),
                false,
                true,
                prepared.resource_estimate(),
                MigrationVerification::Exact,
            )
        }

        fn rollback(
            &self,
            _request: &MigrationRequest,
            _prepared: Box<dyn PreparedMigration>,
        ) -> MigrationResult<()> {
            Ok(())
        }
    }

    fn endpoint(
        representation: &str,
        storage: StateStorageLocation,
        domain: StateExecutionDomain,
    ) -> MigrationEndpoint {
        MigrationEndpoint::new(
            QubitCount::new(2),
            StateRepresentationName::new(representation).expect("valid representation"),
            storage,
            domain,
        )
        .expect("valid endpoint")
    }

    #[test]
    fn endpoint_rejects_zero_qubits() {
        let result = MigrationEndpoint::new(
            QubitCount::ZERO,
            StateRepresentationName::new("state_vector").expect("valid representation"),
            StateStorageLocation::Host,
            StateExecutionDomain::LocalSimulator,
        );

        assert!(result.is_err());
    }

    #[test]
    fn request_rejects_identical_endpoints() {
        let source = endpoint(
            "state_vector",
            StateStorageLocation::Host,
            StateExecutionDomain::LocalSimulator,
        );

        let result = MigrationRequest::new(
            MigrationId::new(1),
            source.clone(),
            source,
            MigrationKind::StorageTransfer,
        );

        assert!(result.is_err());
    }

    #[test]
    fn direct_plan_is_deterministic() {
        let source = endpoint(
            "state_vector",
            StateStorageLocation::Host,
            StateExecutionDomain::LocalSimulator,
        );

        let destination = endpoint(
            "state_vector",
            StateStorageLocation::Device,
            StateExecutionDomain::LocalSimulator,
        );

        let request = MigrationRequest::new(
            MigrationId::new(1),
            source,
            destination,
            MigrationKind::StorageTransfer,
        )
        .expect("valid request");

        let provider = MockProvider;
        let plan =
            MigrationPlan::direct(request, &provider).expect("plan must succeed");

        assert!(plan.is_direct());
        assert_eq!(plan.steps().len(), 1);
    }

    #[test]
    fn engine_executes_transaction() {
        let source = endpoint(
            "state_vector",
            StateStorageLocation::Host,
            StateExecutionDomain::LocalSimulator,
        );

        let destination = endpoint(
            "state_vector",
            StateStorageLocation::Device,
            StateExecutionDomain::LocalSimulator,
        );

        let request = MigrationRequest::new(
            MigrationId::new(2),
            source,
            destination,
            MigrationKind::StorageTransfer,
        )
        .expect("valid request");

        let provider = MockProvider;
        let providers: [&dyn MigrationProvider; 1] = [&provider];
        let engine = MigrationEngine::new(&providers);

        let receipt = engine
            .execute_direct(request)
            .expect("migration must succeed");

        assert_eq!(receipt.status(), MigrationStatus::Committed);
        assert_eq!(receipt.completed_steps(), 1);
        assert!(receipt.destination().storage().is_device());
    }

    #[test]
    fn qpu_endpoint_requires_provider_owned_storage() {
        let result = MigrationEndpoint::new(
            QubitCount::new(2),
            StateRepresentationName::new("backend_native").expect("valid representation"),
            StateStorageLocation::Host,
            StateExecutionDomain::Qpu,
        );

        assert!(result.is_err());
    }

    #[test]
    fn execution_only_allows_provider_verification() {
        let source = endpoint(
            "backend_native",
            StateStorageLocation::Remote,
            StateExecutionDomain::Qpu,
        );

        let destination = endpoint(
            "backend_native",
            StateStorageLocation::Remote,
            StateExecutionDomain::Qpu,
        );

        let result = MigrationRequest::new(
            MigrationId::new(3),
            source,
            destination,
            MigrationKind::ExecutionHandoff,
        );

        assert!(result.is_err());
    }

    #[test]
    fn tolerance_rejects_invalid_fidelity() {
        let result = MigrationTolerance::default_policy()
            .with_minimum_fidelity(1.5)
            .validate();

        assert!(result.is_err());
    }
}