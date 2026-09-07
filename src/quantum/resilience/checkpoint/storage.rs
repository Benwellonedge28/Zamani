//! Zamani Quantum Resilience — Checkpoint Storage
//!
//! Path:
//!     src/quantum/resilience/checkpoint/storage.rs
//!
//! Purpose:
//!     Provider-neutral physical persistence contracts for checkpoints and
//!     checkpoint payload artifacts.
//!
//! Architectural ownership:
//!
//!     checkpoint.rs
//!         Owns:
//!             - checkpoint semantic representation;
//!             - checkpoint lifecycle;
//!             - checkpoint identity;
//!             - payload descriptors;
//!             - checkpoint-level validation;
//!             - CheckpointStore compatibility contract.
//!
//!     manifest.rs
//!         Owns:
//!             - artifact enumeration;
//!             - manifest structure;
//!             - manifest-level identity and ordering.
//!
//!     storage.rs
//!         Owns:
//!             - physical persistence abstraction;
//!             - object addressing;
//!             - namespace isolation;
//!             - streaming object I/O contracts;
//!             - conditional writes;
//!             - deletion;
//!             - listing;
//!             - storage capabilities;
//!             - storage consistency semantics;
//!             - checkpoint-store adapter;
//!             - storage lifecycle metadata.
//!
//!     integrity.rs
//!         Owns:
//!             - cryptographic digest calculation;
//!             - authenticity verification;
//!             - integrity policies.
//!
//!     serialization/*
//!         Owns:
//!             - checkpoint encoding;
//!             - checkpoint decoding;
//!             - schema evolution.
//!
//!     compatibility.rs
//!         Owns:
//!             - restore compatibility;
//!             - target capability compatibility.
//!
//!     recovery/*
//!         Owns:
//!             - checkpoint recovery orchestration;
//!             - rollback;
//!             - resume;
//!             - restore ordering.
//!
//! Important:
//!
//!     This module does NOT:
//!
//!         - serialize arbitrary quantum states;
//!         - perform cryptography;
//!         - calculate hashes;
//!         - select hardware;
//!         - route qubits;
//!         - schedule operations;
//!         - perform QEC;
//!         - make recovery decisions.
//!
//! Scalability:
//!
//!     No fixed maximum is imposed on:
//!
//!         - checkpoints;
//!         - artifacts;
//!         - payload sizes;
//!         - qubits;
//!         - logical qubits;
//!         - physical qubits;
//!         - storage objects;
//!         - namespaces.
//!
//!     Sizes and limits are supplied by the selected storage implementation,
//!     policy and resource-management layers.
//!
//! Quantum identity:
//!
//!     Storage does not invent a resilience-specific qubit identifier.
//!     When a storage key or metadata operation needs quantum identity,
//!     callers must use the canonical IR identity:
//!
//!         crate::quantum::ir::qubit::QubitId
//!
//!     Physical-qubit identity must remain distinct where the surrounding
//!     repository exposes `PhysicalQubitId`.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code
//!
//! Design principle:
//!
//!     A checkpoint storage backend is an implementation detail. The same
//!     checkpoint must be capable of being persisted to a filesystem,
//!     object store, database, distributed storage service, encrypted local
//!     storage, provider-managed storage, or another future medium without
//!     changing the checkpoint model.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::io::{self, Read, Write};
use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::checkpoint::{
    Checkpoint,
    CheckpointError,
    CheckpointId,
    CheckpointReference,
    CheckpointStore,
};

// ============================================================================
// Schema identity
// ============================================================================

/// Stable namespace for checkpoint-storage contracts.
pub const CHECKPOINT_STORAGE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint.storage";

/// Semantic version of the storage contract.
pub const CHECKPOINT_STORAGE_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Storage namespace
// ============================================================================

/// Logical namespace isolating one group of checkpoint objects from another.
///
/// Namespaces are opaque to the resilience layer. They must not encode
/// provider-specific assumptions.
///
/// Examples:
///
///     execution
///     tenant
///     application
///     recovery-domain
///
/// The actual namespace hierarchy is decided by the storage implementation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct StorageNamespace(String);

impl StorageNamespace {
    /// Creates a validated namespace.
    pub fn new(value: impl Into<String>) -> Result<Self, StorageError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StorageError::InvalidNamespace);
        }

        Ok(Self(value))
    }

    /// Returns the namespace as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for StorageNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Object identity
// ============================================================================

/// Opaque physical storage object identity.
///
/// This is deliberately different from `ArtifactId`.
///
/// `ArtifactId` identifies checkpoint content semantically.
///
/// `StorageObjectId` identifies the physical persisted object.
///
/// This separation permits:
///
///     ArtifactId
///         -> one object
///         -> many objects/chunks
///         -> migrated object
///         -> replicated objects
///
/// without changing checkpoint semantics.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct StorageObjectId(String);

impl StorageObjectId {
    /// Creates a validated storage-object identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StorageError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StorageError::InvalidObjectId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StorageObjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Object version
// ============================================================================

/// Opaque storage generation/version.
///
/// A backend may map this to:
///
///     - object generation;
///     - ETag;
///     - database revision;
///     - content version;
///     - transaction sequence.
///
/// Storage consumers must not interpret its representation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct StorageVersion(String);

impl StorageVersion {
    /// Creates a validated storage version.
    pub fn new(value: impl Into<String>) -> Result<Self, StorageError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StorageError::InvalidVersion);
        }

        Ok(Self(value))
    }

    /// Returns the version.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StorageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Object reference
// ============================================================================

/// Complete physical reference to one stored object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageObjectReference {
    /// Namespace containing the object.
    pub namespace: StorageNamespace,

    /// Opaque object identity.
    pub object_id: StorageObjectId,

    /// Optional backend generation/version.
    pub version: Option<StorageVersion>,
}

impl StorageObjectReference {
    /// Creates an object reference.
    pub fn new(
        namespace: StorageNamespace,
        object_id: StorageObjectId,
    ) -> Self {
        Self {
            namespace,
            object_id,
            version: None,
        }
    }

    /// Adds an expected/current storage version.
    #[must_use]
    pub fn with_version(mut self, version: StorageVersion) -> Self {
        self.version = Some(version);
        self
    }
}

// ============================================================================
// Consistency
// ============================================================================

/// Consistency guarantees exposed by a storage backend.
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
pub enum StorageConsistency {
    /// Reads immediately observe successful writes.
    Strong,

    /// Reads may temporarily lag successful writes.
    Eventual,

    /// Backend-defined consistency.
    ProviderDefined,
}

// ============================================================================
// Durability
// ============================================================================

/// Durability classification.
///
/// This is descriptive. It does not claim a universal physical guarantee.
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
pub enum StorageDurability {
    /// Data may be lost with loss of the active process/node.
    Ephemeral,

    /// Data survives process restart but is not guaranteed to survive
    /// independent infrastructure loss.
    LocalDurable,

    /// Backend provides durable persistence.
    Durable,

    /// Backend provides replicated/distributed durability.
    Replicated,

    /// Backend defines its own durability semantics.
    ProviderDefined,
}

// ============================================================================
// Storage capabilities
// ============================================================================

/// Capabilities offered by a storage implementation.
///
/// These are discovered at runtime.
///
/// No implementation should infer a capability from provider name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Whether objects can be created.
    pub create: bool,

    /// Whether existing objects can be replaced.
    pub overwrite: bool,

    /// Whether conditional creation is supported.
    pub conditional_create: bool,

    /// Whether conditional replacement is supported.
    pub conditional_replace: bool,

    /// Whether objects can be read.
    pub read: bool,

    /// Whether objects can be deleted.
    pub delete: bool,

    /// Whether objects can be listed.
    pub list: bool,

    /// Whether object reads/writes can be streamed.
    pub streaming: bool,

    /// Whether range reads are supported.
    pub range_read: bool,

    /// Whether range writes are supported.
    pub range_write: bool,

    /// Whether atomic rename/move is supported.
    pub atomic_move: bool,

    /// Whether transactions are supported.
    pub transactions: bool,

    /// Whether object versions/generations are available.
    pub versioning: bool,

    /// Consistency semantics.
    pub consistency: StorageConsistency,

    /// Durability semantics.
    pub durability: StorageDurability,
}

impl StorageCapabilities {
    /// Returns whether the backend can satisfy basic checkpoint persistence.
    #[must_use]
    pub const fn supports_checkpoint_persistence(&self) -> bool {
        self.create && self.read && self.delete
    }

    /// Returns whether conditional publication is possible.
    #[must_use]
    pub const fn supports_atomic_publication(&self) -> bool {
        self.conditional_create
            || self.conditional_replace
            || self.transactions
            || self.atomic_move
    }
}

// ============================================================================
// Storage limits
// ============================================================================

/// Runtime-discovered storage constraints.
///
/// `None` means that the backend does not advertise a fixed limit.
///
/// These are capabilities, not architectural constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLimits {
    /// Maximum object size if the backend exposes one.
    pub maximum_object_bytes: Option<u64>,

    /// Maximum number of objects that can be returned by one list page.
    ///
    /// This is a provider/API constraint, not a system-wide checkpoint limit.
    pub maximum_list_page_objects: Option<u64>,

    /// Maximum range size if range I/O is constrained.
    pub maximum_range_bytes: Option<u64>,
}

impl StorageLimits {
    /// Creates an unconstrained limit description.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            maximum_object_bytes: None,
            maximum_list_page_objects: None,
            maximum_range_bytes: None,
        }
    }
}

// ============================================================================
// Storage backend identity
// ============================================================================

/// Provider-neutral identity of a storage backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageBackendIdentity {
    /// Stable implementation identity.
    pub implementation: String,

    /// Backend instance identity.
    pub instance: Option<String>,

    /// Contract version exposed by the implementation.
    pub contract_version: u16,
}

impl StorageBackendIdentity {
    /// Creates backend identity.
    pub fn new(
        implementation: impl Into<String>,
        contract_version: u16,
    ) -> Result<Self, StorageError> {
        let implementation = implementation.into();

        if implementation.trim().is_empty() {
            return Err(StorageError::InvalidBackendIdentity);
        }

        if contract_version == 0 {
            return Err(StorageError::InvalidVersion);
        }

        Ok(Self {
            implementation,
            instance: None,
            contract_version,
        })
    }

    /// Adds a backend instance identifier.
    #[must_use]
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
}

// ============================================================================
// Storage metadata
// ============================================================================

/// Metadata returned by a storage backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageObjectMetadata {
    /// Physical object reference.
    pub reference: StorageObjectReference,

    /// Object size when known.
    pub byte_length: Option<u64>,

    /// Creation time when supplied by the backend.
    pub created_at: Option<SystemTime>,

    /// Last modification time when supplied by the backend.
    pub modified_at: Option<SystemTime>,

    /// Content type when supplied.
    pub media_type: Option<String>,
}

// ============================================================================
// Write conditions
// ============================================================================

/// Conditional object-write semantics.
///
/// These conditions are critical for distributed checkpoint publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteCondition {
    /// Object must not already exist.
    IfAbsent,

    /// Object must already exist.
    IfPresent,

    /// Existing object must have this exact version.
    IfVersion(StorageVersion),

    /// Unconditional write.
    Unconditional,
}

// ============================================================================
// Read request
// ============================================================================

/// Object read request.
///
/// Range reads permit very large objects to be consumed incrementally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageReadRequest {
    /// Object to read.
    pub reference: StorageObjectReference,

    /// Optional inclusive starting byte offset.
    pub start: Option<u64>,

    /// Optional exclusive ending byte offset.
    pub end: Option<u64>,
}

impl StorageReadRequest {
    /// Creates a complete-object read request.
    #[must_use]
    pub fn whole(reference: StorageObjectReference) -> Self {
        Self {
            reference,
            start: None,
            end: None,
        }
    }

    /// Creates a range read request.
    pub fn range(
        reference: StorageObjectReference,
        start: u64,
        end: u64,
    ) -> Result<Self, StorageError> {
        if start >= end {
            return Err(StorageError::InvalidRange);
        }

        Ok(Self {
            reference,
            start: Some(start),
            end: Some(end),
        })
    }
}

// ============================================================================
// Write request
// ============================================================================

/// Object write request.
///
/// The actual bytes are supplied through a `Read` implementation rather than
/// being embedded in the request. This prevents the storage contract from
/// requiring the entire payload to be resident in memory.
pub struct StorageWriteRequest<R> {
    /// Destination object.
    pub reference: StorageObjectReference,

    /// Source stream.
    pub reader: R,

    /// Expected byte length when known.
    pub byte_length: Option<u64>,

    /// Content type.
    pub media_type: Option<String>,

    /// Write condition.
    pub condition: WriteCondition,
}

impl<R> StorageWriteRequest<R> {
    /// Creates an unconditional write.
    #[must_use]
    pub fn new(
        reference: StorageObjectReference,
        reader: R,
    ) -> Self {
        Self {
            reference,
            reader,
            byte_length: None,
            media_type: None,
            condition: WriteCondition::Unconditional,
        }
    }

    /// Declares the expected byte length.
    #[must_use]
    pub fn with_byte_length(
        mut self,
        byte_length: u64,
    ) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Declares the media type.
    #[must_use]
    pub fn with_media_type(
        mut self,
        media_type: impl Into<String>,
    ) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Applies a conditional-write policy.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: WriteCondition,
    ) -> Self {
        self.condition = condition;
        self
    }
}

// ============================================================================
// Storage page
// ============================================================================

/// One page of object-list results.
///
/// Pagination is mandatory for large-scale storage systems.
///
/// A caller must not assume that all checkpoint objects fit in memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageObjectPage {
    /// Objects in this page.
    pub objects: Arc<[StorageObjectMetadata]>,

    /// Opaque continuation token.
    pub continuation_token: Option<String>,
}

impl StorageObjectPage {
    /// Returns whether another page exists.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.continuation_token.is_some()
    }
}

// ============================================================================
// Storage transaction
// ============================================================================

/// Transaction identifier.
///
/// Transactions are optional because many object stores do not expose
/// traditional transactions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct StorageTransactionId(String);

impl StorageTransactionId {
    /// Creates a transaction identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StorageError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StorageError::InvalidTransactionId);
        }

        Ok(Self(value))
    }
}

// ============================================================================
// Streaming reader/writer contracts
// ============================================================================

/// Storage reader abstraction.
///
/// This wrapper intentionally exposes standard `Read` semantics while keeping
/// the storage implementation behind a trait boundary.
pub trait StorageReader: Read + Send {}

impl<T> StorageReader for T
where
    T: Read + Send,
{
}

/// Storage writer abstraction.
///
/// Storage implementations may stream data directly to disk/object storage
/// without buffering the complete payload.
pub trait StorageWriter: Write + Send {}

impl<T> StorageWriter for T
where
    T: Write + Send,
{
}

// ============================================================================
// Core storage backend
// ============================================================================

/// Physical checkpoint-storage backend.
///
/// This is the primary integration contract for concrete storage providers.
///
/// Implementations may be:
///
///     - local filesystem;
///     - encrypted filesystem;
///     - database;
///     - object storage;
///     - distributed storage;
///     - provider-managed storage;
///     - content-addressed storage;
///     - future storage technology.
///
/// No provider names appear in this contract.
pub trait CheckpointStorageBackend: Send + Sync {
    /// Returns backend identity.
    fn identity(&self) -> StorageBackendIdentity;

    /// Returns supported capabilities.
    fn capabilities(&self) -> StorageCapabilities;

    /// Returns runtime-discovered limits.
    fn limits(&self) -> StorageLimits;

    /// Stores an object.
    ///
    /// Implementations MUST honor `WriteCondition`.
    fn write<R>(
        &self,
        request: StorageWriteRequest<R>,
    ) -> Result<StorageObjectMetadata, StorageError>
    where
        R: Read + Send;

    /// Opens an object for streaming reads.
    fn read(
        &self,
        request: StorageReadRequest,
    ) -> Result<Box<dyn StorageReader>, StorageError>;

    /// Returns metadata without reading the object body.
    fn metadata(
        &self,
        reference: &StorageObjectReference,
    ) -> Result<StorageObjectMetadata, StorageError>;

    /// Deletes an object.
    fn delete(
        &self,
        reference: &StorageObjectReference,
    ) -> Result<(), StorageError>;

    /// Lists objects in a namespace.
    ///
    /// Listing MUST be paginated.
    fn list(
        &self,
        namespace: &StorageNamespace,
        continuation_token: Option<&str>,
    ) -> Result<StorageObjectPage, StorageError>;

    /// Starts a transaction when supported.
    ///
    /// Implementations that do not support transactions must return
    /// `StorageError::UnsupportedOperation`.
    fn begin_transaction(
        &self,
    ) -> Result<StorageTransactionId, StorageError> {
        Err(StorageError::UnsupportedOperation {
            operation: "begin_transaction",
        })
    }

    /// Commits a transaction.
    fn commit_transaction(
        &self,
        _transaction: StorageTransactionId,
    ) -> Result<(), StorageError> {
        Err(StorageError::UnsupportedOperation {
            operation: "commit_transaction",
        })
    }

    /// Aborts a transaction.
    fn abort_transaction(
        &self,
        _transaction: StorageTransactionId,
    ) -> Result<(), StorageError> {
        Err(StorageError::UnsupportedOperation {
            operation: "abort_transaction",
        })
    }
}

// ============================================================================
// Checkpoint codec integration contract
// ============================================================================

/// Encoding/decoding bridge between storage and the serialization subsystem.
///
/// `serialization/*` owns the actual serialization format.
///
/// This trait exists here only as the integration seam so that storage never
/// becomes coupled to JSON, bincode, CBOR, MessagePack, or another format.
///
/// The serializer implementation should live outside `storage.rs`.
pub trait CheckpointCodec: Send + Sync {
    /// Encodes a checkpoint into a stream.
    fn encode(
        &self,
        checkpoint: &Checkpoint,
        writer: &mut dyn Write,
    ) -> Result<(), CheckpointError>;

    /// Decodes a checkpoint from a stream.
    fn decode(
        &self,
        reader: &mut dyn Read,
    ) -> Result<Checkpoint, CheckpointError>;
}

// ============================================================================
// Storage policy
// ============================================================================

/// Storage policy supplied by the resilience/policy layer.
///
/// No retry count, timeout, object size, or retention period is hard-coded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePolicy {
    /// Whether overwriting an existing checkpoint object is permitted.
    pub allow_overwrite: bool,

    /// Whether deleting checkpoint objects is permitted.
    pub allow_delete: bool,

    /// Whether conditional writes are required.
    pub require_conditional_write: bool,

    /// Whether strong consistency is required.
    pub require_strong_consistency: bool,

    /// Whether versioning is required.
    pub require_versioning: bool,

    /// Whether streaming is required for payloads.
    pub require_streaming: bool,
}

impl StoragePolicy {
    /// Validates policy against backend capabilities.
    pub fn validate_against(
        &self,
        capabilities: &StorageCapabilities,
    ) -> Result<(), StorageError> {
        if !capabilities.read || !capabilities.create {
            return Err(StorageError::CapabilityUnavailable {
                capability: "checkpoint read/write",
            });
        }

        if self.allow_overwrite && !capabilities.overwrite {
            return Err(StorageError::CapabilityUnavailable {
                capability: "object overwrite",
            });
        }

        if self.allow_delete && !capabilities.delete {
            return Err(StorageError::CapabilityUnavailable {
                capability: "object deletion",
            });
        }

        if self.require_conditional_write
            && !capabilities.conditional_create
            && !capabilities.conditional_replace
            && !capabilities.transactions
        {
            return Err(StorageError::CapabilityUnavailable {
                capability: "conditional publication",
            });
        }

        if self.require_strong_consistency
            && capabilities.consistency != StorageConsistency::Strong
        {
            return Err(StorageError::CapabilityUnavailable {
                capability: "strong consistency",
            });
        }

        if self.require_versioning && !capabilities.versioning {
            return Err(StorageError::CapabilityUnavailable {
                capability: "object versioning",
            });
        }

        if self.require_streaming && !capabilities.streaming {
            return Err(StorageError::CapabilityUnavailable {
                capability: "streaming I/O",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint object addressing
// ============================================================================

/// Deterministic physical object-key strategy.
///
/// This strategy deliberately does not embed provider names or hardware
/// dimensions into object identities.
pub trait CheckpointObjectAddressing: Send + Sync {
    /// Returns the namespace for a checkpoint.
    fn namespace(
        &self,
        checkpoint: &Checkpoint,
    ) -> Result<StorageNamespace, StorageError>;

    /// Returns the metadata object reference for a checkpoint.
    fn checkpoint_object(
        &self,
        checkpoint: &Checkpoint,
        namespace: &StorageNamespace,
    ) -> Result<StorageObjectReference, StorageError>;
}

/// Default provider-neutral addressing scheme.
///
/// The resulting path is opaque to the storage implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultCheckpointObjectAddressing;

impl CheckpointObjectAddressing for DefaultCheckpointObjectAddressing {
    fn namespace(
        &self,
        checkpoint: &Checkpoint,
    ) -> Result<StorageNamespace, StorageError> {
        StorageNamespace::new(format!(
            "checkpoint/{}",
            checkpoint.execution_id
        ))
    }

    fn checkpoint_object(
        &self,
        checkpoint: &Checkpoint,
        namespace: &StorageNamespace,
    ) -> Result<StorageObjectReference, StorageError> {
        let object_id = StorageObjectId::new(format!(
            "metadata/{}",
            checkpoint.checkpoint_id
        ))?;

        Ok(StorageObjectReference::new(
            namespace.clone(),
            object_id,
        ))
    }
}

// ============================================================================
// Checkpoint storage service
// ============================================================================

/// High-level checkpoint persistence service.
///
/// This type composes:
///
///     storage backend
///     + codec
///     + addressing
///     + storage policy
///
/// It does not perform recovery, integrity verification, or compatibility
/// decisions.
pub struct CheckpointStorageService<S, C, A = DefaultCheckpointObjectAddressing>
where
    S: CheckpointStorageBackend,
    C: CheckpointCodec,
    A: CheckpointObjectAddressing,
{
    storage: Arc<S>,
    codec: Arc<C>,
    addressing: Arc<A>,
    policy: StoragePolicy,
}

impl<S, C, A> CheckpointStorageService<S, C, A>
where
    S: CheckpointStorageBackend,
    C: CheckpointCodec,
    A: CheckpointObjectAddressing,
{
    /// Creates a storage service after validating policy against capabilities.
    pub fn new(
        storage: Arc<S>,
        codec: Arc<C>,
        addressing: Arc<A>,
        policy: StoragePolicy,
    ) -> Result<Self, StorageError> {
        policy.validate_against(&storage.capabilities())?;

        Ok(Self {
            storage,
            codec,
            addressing,
            policy,
        })
    }

    /// Returns the backend identity.
    #[must_use]
    pub fn backend_identity(&self) -> StorageBackendIdentity {
        self.storage.identity()
    }

    /// Returns storage capabilities.
    #[must_use]
    pub fn capabilities(&self) -> StorageCapabilities {
        self.storage.capabilities()
    }

    /// Returns storage limits.
    #[must_use]
    pub fn limits(&self) -> StorageLimits {
        self.storage.limits()
    }

    /// Stores a checkpoint atomically according to backend capabilities and
    /// policy.
    pub fn put_checkpoint(
        &self,
        checkpoint: &Checkpoint,
    ) -> Result<CheckpointReference, StorageError> {
        checkpoint
            .validate()
            .map_err(StorageError::Checkpoint)?;

        let namespace = self.addressing.namespace(checkpoint)?;

        let object = self
            .addressing
            .checkpoint_object(checkpoint, &namespace)?;

        let mut encoded = Vec::new();

        self.codec
            .encode(checkpoint, &mut encoded)
            .map_err(StorageError::Checkpoint)?;

        let condition = if self.policy.allow_overwrite {
            WriteCondition::Unconditional
        } else {
            WriteCondition::IfAbsent
        };

        let request = StorageWriteRequest::new(
            object.clone(),
            io::Cursor::new(encoded),
        )
        .with_media_type("application/x-zamani-checkpoint")
        .with_condition(condition);

        let metadata = self.storage.write(request)?;

        Ok(CheckpointReference {
            checkpoint_id: checkpoint.checkpoint_id.clone(),
            namespace: namespace.into_string(),
            version: metadata.reference.version.map(|version| version.into_string()),
        })
    }

    /// Loads a checkpoint through the configured codec.
    pub fn get_checkpoint(
        &self,
        reference: &CheckpointReference,
    ) -> Result<Checkpoint, StorageError> {
        let namespace =
            StorageNamespace::new(reference.namespace.clone())?;

        let object_id = StorageObjectId::new(format!(
            "metadata/{}",
            reference.checkpoint_id
        ))?;

        let object = StorageObjectReference::new(
            namespace,
            object_id,
        );

        let mut reader = self
            .storage
            .read(StorageReadRequest::whole(object))?;

        let checkpoint = self
            .codec
            .decode(&mut reader)
            .map_err(StorageError::Checkpoint)?;

        checkpoint
            .validate()
            .map_err(StorageError::Checkpoint)?;

        Ok(checkpoint)
    }

    /// Removes a checkpoint metadata object.
    ///
    /// Payload artifacts are deliberately not implicitly deleted here.
    /// Artifact lifecycle belongs to `manifest.rs` and the recovery/storage
    /// orchestration layer because one artifact may be shared or replicated.
    pub fn remove_checkpoint(
        &self,
        reference: &CheckpointReference,
    ) -> Result<(), StorageError> {
        if !self.policy.allow_delete {
            return Err(StorageError::PolicyRejected {
                operation: "checkpoint deletion",
            });
        }

        let namespace =
            StorageNamespace::new(reference.namespace.clone())?;

        let object_id = StorageObjectId::new(format!(
            "metadata/{}",
            reference.checkpoint_id
        ))?;

        let object = StorageObjectReference::new(
            namespace,
            object_id,
        );

        self.storage.delete(&object)
    }
}

// ============================================================================
// CheckpointStore integration
// ============================================================================

/// Adapter allowing the high-level storage service to satisfy the canonical
/// `CheckpointStore` contract defined by `checkpoint.rs`.
///
/// This is the intended integration point with recovery/checkpoint.rs.
impl<S, C, A> CheckpointStore for CheckpointStorageService<S, C, A>
where
    S: CheckpointStorageBackend,
    C: CheckpointCodec,
    A: CheckpointObjectAddressing,
{
    fn put(
        &self,
        checkpoint: &Checkpoint,
    ) -> Result<CheckpointReference, CheckpointError> {
        self.put_checkpoint(checkpoint)
            .map_err(StorageError::into_checkpoint_error)
    }

    fn get(
        &self,
        reference: &CheckpointReference,
    ) -> Result<Checkpoint, CheckpointError> {
        self.get_checkpoint(reference)
            .map_err(StorageError::into_checkpoint_error)
    }

    fn remove(
        &self,
        reference: &CheckpointReference,
    ) -> Result<(), CheckpointError> {
        self.remove_checkpoint(reference)
            .map_err(StorageError::into_checkpoint_error)
    }
}

// ============================================================================
// Storage error
// ============================================================================

/// Complete physical-storage error taxonomy.
///
/// Backend-specific errors must be converted into these provider-neutral
/// categories. Secrets and credentials must never be included in messages.
#[derive(Debug)]
pub enum StorageError {
    /// Namespace is invalid.
    InvalidNamespace,

    /// Object identifier is invalid.
    InvalidObjectId,

    /// Storage version is invalid.
    InvalidVersion,

    /// Backend identity is invalid.
    InvalidBackendIdentity,

    /// Transaction identity is invalid.
    InvalidTransactionId,

    /// Byte range is invalid.
    InvalidRange,

    /// Requested storage capability is unavailable.
    CapabilityUnavailable {
        /// Capability name.
        capability: &'static str,
    },

    /// Operation is unsupported.
    UnsupportedOperation {
        /// Operation name.
        operation: &'static str,
    },

    /// Object was not found.
    NotFound,

    /// Object already exists.
    AlreadyExists,

    /// Conditional write failed.
    ConditionFailed,

    /// Object changed between observation and operation.
    VersionConflict,

    /// Storage backend is temporarily unavailable.
    Unavailable,

    /// Backend communication failed.
    Communication,

    /// Backend operation timed out.
    Timeout,

    /// Operation was cancelled.
    Cancelled,

    /// Storage policy rejected an operation.
    PolicyRejected {
        /// Operation name.
        operation: &'static str,
    },

    /// Stream I/O failed.
    Io(io::Error),

    /// Checkpoint-level validation/serialization failure.
    Checkpoint(CheckpointError),

    /// Backend returned invalid metadata.
    InvalidMetadata,

    /// Backend reported inconsistent state.
    InconsistentState,

    /// Safe backend diagnostic.
    Backend {
        /// Safe diagnostic category.
        message: String,
    },
}

impl StorageError {
    /// Converts a storage error into the canonical checkpoint error.
    ///
    /// Provider details are deliberately not leaked into the checkpoint
    /// contract.
    pub fn into_checkpoint_error(self) -> CheckpointError {
        match self {
            Self::Checkpoint(error) => error,

            Self::NotFound
            | Self::Unavailable
            | Self::Communication
            | Self::Timeout => CheckpointError::Storage {
                message: "checkpoint storage is unavailable".to_owned(),
            },

            Self::AlreadyExists | Self::ConditionFailed => {
                CheckpointError::Storage {
                    message:
                        "checkpoint storage conditional write failed"
                            .to_owned(),
                }
            }

            Self::VersionConflict => CheckpointError::Storage {
                message:
                    "checkpoint storage version conflict".to_owned(),
            },

            Self::PolicyRejected { operation } => {
                CheckpointError::Storage {
                    message: format!(
                        "checkpoint storage policy rejected operation: {operation}"
                    ),
                }
            }

            Self::UnsupportedOperation { operation } => {
                CheckpointError::Storage {
                    message: format!(
                        "checkpoint storage operation is unsupported: {operation}"
                    ),
                }
            }

            Self::CapabilityUnavailable { capability } => {
                CheckpointError::Storage {
                    message: format!(
                        "checkpoint storage capability unavailable: {capability}"
                    ),
                }
            }

            Self::InvalidNamespace
            | Self::InvalidObjectId
            | Self::InvalidVersion
            | Self::InvalidBackendIdentity
            | Self::InvalidTransactionId
            | Self::InvalidRange
            | Self::InvalidMetadata
            | Self::InconsistentState => CheckpointError::Storage {
                message:
                    "checkpoint storage request or state is invalid"
                        .to_owned(),
            },

            Self::Cancelled => CheckpointError::Storage {
                message: "checkpoint storage operation was cancelled"
                    .to_owned(),
            },

            Self::Io(_) => CheckpointError::Storage {
                message: "checkpoint storage I/O failed".to_owned(),
            },

            Self::Backend { .. } => CheckpointError::Storage {
                message: "checkpoint storage backend failed".to_owned(),
            },
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNamespace => {
                formatter.write_str("invalid storage namespace")
            }
            Self::InvalidObjectId => {
                formatter.write_str("invalid storage object identifier")
            }
            Self::InvalidVersion => {
                formatter.write_str("invalid storage version")
            }
            Self::InvalidBackendIdentity => {
                formatter.write_str("invalid storage backend identity")
            }
            Self::InvalidTransactionId => {
                formatter.write_str("invalid storage transaction identifier")
            }
            Self::InvalidRange => {
                formatter.write_str("invalid storage byte range")
            }
            Self::CapabilityUnavailable { capability } => {
                write!(
                    formatter,
                    "storage capability unavailable: {capability}"
                )
            }
            Self::UnsupportedOperation { operation } => {
                write!(
                    formatter,
                    "storage operation unsupported: {operation}"
                )
            }
            Self::NotFound => {
                formatter.write_str("storage object not found")
            }
            Self::AlreadyExists => {
                formatter.write_str("storage object already exists")
            }
            Self::ConditionFailed => {
                formatter.write_str("storage write condition failed")
            }
            Self::VersionConflict => {
                formatter.write_str("storage object version conflict")
            }
            Self::Unavailable => {
                formatter.write_str("storage backend unavailable")
            }
            Self::Communication => {
                formatter.write_str("storage communication failed")
            }
            Self::Timeout => {
                formatter.write_str("storage operation timed out")
            }
            Self::Cancelled => {
                formatter.write_str("storage operation cancelled")
            }
            Self::PolicyRejected { operation } => {
                write!(
                    formatter,
                    "storage policy rejected operation: {operation}"
                )
            }
            Self::Io(_) => {
                formatter.write_str("storage I/O failed")
            }
            Self::Checkpoint(_) => {
                formatter.write_str("checkpoint operation failed")
            }
            Self::InvalidMetadata => {
                formatter.write_str("invalid storage metadata")
            }
            Self::InconsistentState => {
                formatter.write_str("storage state is inconsistent")
            }
            Self::Backend { message } => {
                write!(formatter, "storage backend failure: {message}")
            }
        }
    }
}

impl std::error::Error for StorageError {}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

// ============================================================================
// Storage inspection
// ============================================================================

/// Lightweight storage inspection result.
///
/// Useful to telemetry, diagnostics and readiness checks without reading
/// checkpoint payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageInspection {
    /// Backend identity.
    pub backend: StorageBackendIdentity,

    /// Capabilities.
    pub capabilities: StorageCapabilities,

    /// Runtime limits.
    pub limits: StorageLimits,
}

impl<S> From<&S> for StorageInspection
where
    S: CheckpointStorageBackend,
{
    fn from(storage: &S) -> Self {
        Self {
            backend: storage.identity(),
            capabilities: storage.capabilities(),
            limits: storage.limits(),
        }
    }
}

// ============================================================================
// Storage health contract
// ============================================================================

/// Storage health classification.
///
/// This belongs here rather than in resilience/model/health.rs only as a
/// low-level storage observation. The resilience health model can consume it.
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
pub enum StorageHealth {
    /// Storage has not been inspected.
    Unknown,

    /// Storage is operational.
    Healthy,

    /// Storage is operational but degraded.
    Degraded,

    /// Storage may intermittently fail.
    Unstable,

    /// Storage cannot currently service operations.
    Unavailable,
}

/// Optional storage-health probing contract.
///
/// A backend does not have to implement this separately; the default derives
/// health from its capabilities.
pub trait StorageHealthProvider: Send + Sync {
    /// Returns current storage health.
    fn health(&self) -> StorageHealth;
}

// ============================================================================
// Default capability-derived health
// ============================================================================

/// Derives a conservative health state from advertised capabilities.
#[must_use]
pub fn health_from_capabilities(
    capabilities: &StorageCapabilities,
) -> StorageHealth {
    if !capabilities.create
        || !capabilities.read
        || !capabilities.delete
    {
        return StorageHealth::Unavailable;
    }

    if capabilities.consistency == StorageConsistency::Eventual
        && capabilities.durability == StorageDurability::Ephemeral
    {
        return StorageHealth::Degraded;
    }

    StorageHealth::Healthy
}

// ============================================================================
// Compile-time integration assertions
// ============================================================================

/// Ensures a concrete service remains usable anywhere the canonical
/// checkpoint-store contract is expected.
///
/// This function is intentionally never executed. Its purpose is type-level
/// integration checking.
#[allow(dead_code)]
fn assert_checkpoint_store<S, C, A>()
where
    S: CheckpointStorageBackend,
    C: CheckpointCodec,
    A: CheckpointObjectAddressing,
    CheckpointStorageService<S, C, A>: CheckpointStore,
{
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_rejects_empty_values() {
        assert!(StorageNamespace::new("").is_err());
        assert!(StorageNamespace::new("   ").is_err());
    }

    #[test]
    fn namespace_accepts_opaque_values() {
        let namespace =
            StorageNamespace::new("execution/example").unwrap();

        assert_eq!(
            namespace.as_str(),
            "execution/example"
        );
    }

    #[test]
    fn object_reference_preserves_version() {
        let namespace =
            StorageNamespace::new("checkpoint/example").unwrap();

        let object =
            StorageObjectId::new("metadata/example").unwrap();

        let version =
            StorageVersion::new("generation-1").unwrap();

        let reference =
            StorageObjectReference::new(namespace, object)
                .with_version(version.clone());

        assert_eq!(
            reference.version,
            Some(version)
        );
    }

    #[test]
    fn range_rejects_empty_interval() {
        let namespace =
            StorageNamespace::new("checkpoint/example").unwrap();

        let object =
            StorageObjectId::new("payload/example").unwrap();

        let reference =
            StorageObjectReference::new(namespace, object);

        assert!(
            StorageReadRequest::range(
                reference,
                10,
                10
            )
            .is_err()
        );
    }

    #[test]
    fn range_accepts_valid_interval() {
        let namespace =
            StorageNamespace::new("checkpoint/example").unwrap();

        let object =
            StorageObjectId::new("payload/example").unwrap();

        let reference =
            StorageObjectReference::new(namespace, object);

        let request =
            StorageReadRequest::range(
                reference,
                10,
                20,
            )
            .unwrap();

        assert_eq!(request.start, Some(10));
        assert_eq!(request.end, Some(20));
    }

    #[test]
    fn unbounded_limits_have_no_architectural_maximum() {
        let limits = StorageLimits::unbounded();

        assert_eq!(
            limits.maximum_object_bytes,
            None
        );

        assert_eq!(
            limits.maximum_list_page_objects,
            None
        );

        assert_eq!(
            limits.maximum_range_bytes,
            None
        );
    }

    #[test]
    fn capabilities_detect_basic_checkpoint_support() {
        let capabilities = StorageCapabilities {
            create: true,
            overwrite: true,
            conditional_create: true,
            conditional_replace: true,
            read: true,
            delete: true,
            list: true,
            streaming: true,
            range_read: true,
            range_write: false,
            atomic_move: true,
            transactions: false,
            versioning: true,
            consistency: StorageConsistency::Strong,
            durability: StorageDurability::Durable,
        };

        assert!(
            capabilities
                .supports_checkpoint_persistence()
        );

        assert!(
            capabilities
                .supports_atomic_publication()
        );
    }

    #[test]
    fn policy_rejects_missing_required_capability() {
        let capabilities = StorageCapabilities {
            create: true,
            overwrite: false,
            conditional_create: false,
            conditional_replace: false,
            read: true,
            delete: true,
            list: true,
            streaming: true,
            range_read: false,
            range_write: false,
            atomic_move: false,
            transactions: false,
            versioning: false,
            consistency: StorageConsistency::Eventual,
            durability: StorageDurability::LocalDurable,
        };

        let policy = StoragePolicy {
            allow_overwrite: false,
            allow_delete: true,
            require_conditional_write: true,
            require_strong_consistency: true,
            require_versioning: true,
            require_streaming: true,
        };

        assert!(
            policy
                .validate_against(&capabilities)
                .is_err()
        );
    }

    #[test]
    fn health_derivation_is_conservative() {
        let unavailable = StorageCapabilities {
            create: false,
            overwrite: false,
            conditional_create: false,
            conditional_replace: false,
            read: false,
            delete: false,
            list: false,
            streaming: false,
            range_read: false,
            range_write: false,
            atomic_move: false,
            transactions: false,
            versioning: false,
            consistency: StorageConsistency::ProviderDefined,
            durability: StorageDurability::ProviderDefined,
        };

        assert_eq!(
            health_from_capabilities(&unavailable),
            StorageHealth::Unavailable
        );
    }
}