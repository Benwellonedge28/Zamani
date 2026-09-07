//! Transaction-safe persistence contracts for the resilience state subsystem.
//!
//! # Responsibility
//!
//! This module owns the persistence *contract* for resilience state. It does
//! not own:
//!
//! - quantum IR;
//! - logical/physical qubit identity;
//! - machine lifecycle semantics;
//! - execution semantics;
//! - checkpoint storage;
//! - serialization formats;
//! - cryptographic primitives;
//! - filesystem/database implementations.
//!
//! Those concerns belong to their respective subsystems.
//!
//! The persistence layer provides:
//!
//! - stable state keys;
//! - opaque serialized payload storage;
//! - schema-version metadata;
//! - monotonically increasing revisions;
//! - optimistic concurrency control;
//! - atomic replacement semantics;
//! - explicit deletion semantics;
//! - deterministic listing;
//! - integrity metadata;
//! - bounded metadata validation;
//! - an in-memory reference implementation for tests;
//! - typed errors suitable for integration with the resilience error layer.
//!
//! # Architectural position
//!
//! ```text
//! state/machine.rs
//! state/execution.rs
//! state/logical.rs
//! state/physical.rs
//! state/recovery.rs
//!          |
//!          v
//! state/persistence.rs
//!          |
//!          +--------------------+
//!          |                    |
//!          v                    v
//! serialization/*        checkpoint/storage.rs
//!          |
//!          v
//! concrete durable storage
//! ```
//!
//! This module intentionally depends on no concrete storage provider.
//!
//! # Important quantum-state rule
//!
//! Persistence must never imply that an arbitrary live quantum state can be
//! serialized and restored. This module persists *software-visible resilience
//! state*. Actual quantum-state checkpointing is governed by the checkpoint
//! subsystem and the capabilities of the execution target.
//!
//! # Concurrency model
//!
//! The store uses optimistic concurrency. A caller reads revision `R`, builds a
//! new state, and writes it only if the currently persisted revision is still
//! `R`. A conflicting writer therefore receives an explicit conflict rather
//! than silently overwriting newer state.
//!
//! # Scalability
//!
//! There are no fixed qubit counts, provider identifiers, retry counts,
//! payload sizes, machine sizes, or collection capacities in this module.
//!
//! Concrete storage implementations are responsible for deciding how large a
//! payload or state collection they can physically support.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

/// Stable identifier for a persisted resilience-state object.
///
/// The value is deliberately opaque to this module. Allocation of identifiers
/// belongs to the caller/coordinator rather than this storage abstraction.
///
/// `u128` provides a large identifier space without imposing a machine-size
/// limit on quantum resources.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PersistenceId(u128);

impl PersistenceId {
    /// Creates an identifier from its raw representation.
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }
}

impl From<u128> for PersistenceId {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl fmt::Display for PersistenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:032x}", self.0)
    }
}

/// Monotonically increasing revision of a persisted object.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PersistenceRevision(u64);

impl PersistenceRevision {
    /// Revision of an object that has not yet been persisted.
    pub const INITIAL: Self = Self(0);

    /// Creates a revision from a raw value.
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw revision.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    fn next(self) -> Result<Self, PersistenceError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(PersistenceError::RevisionExhausted)
    }
}

impl Default for PersistenceRevision {
    fn default() -> Self {
        Self::INITIAL
    }
}

impl From<u64> for PersistenceRevision {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl fmt::Display for PersistenceRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Logical namespace for persisted objects.
///
/// A namespace is intentionally represented as caller-defined text instead of
/// an enum. This prevents the persistence layer from becoming a registry of
/// every future resilience state type.
///
/// Examples of namespaces can be:
///
/// - `resilience.machine`
/// - `resilience.execution`
/// - `resilience.logical`
/// - `resilience.physical`
/// - `resilience.recovery`
///
/// The constants are convenience values only; implementations may define
/// additional namespaces.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StateNamespace(String);

impl StateNamespace {
    /// Namespace for global resilience machine state.
    pub const MACHINE: &'static str = "resilience.machine";

    /// Namespace for execution state.
    pub const EXECUTION: &'static str = "resilience.execution";

    /// Namespace for logical-resource state.
    pub const LOGICAL: &'static str = "resilience.logical";

    /// Namespace for physical-resource state.
    pub const PHYSICAL: &'static str = "resilience.physical";

    /// Namespace for recovery-operation state.
    pub const RECOVERY: &'static str = "resilience.recovery";

    /// Creates a namespace.
    ///
    /// The namespace must not be empty or contain control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, PersistenceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PersistenceError::InvalidNamespace(
                "namespace must not be empty".to_owned(),
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(PersistenceError::InvalidNamespace(
                "namespace must not contain control characters".to_owned(),
            ));
        }

        Ok(Self(value))
    }

    /// Returns the namespace text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for StateNamespace {
    type Error = PersistenceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for StateNamespace {
    type Error = PersistenceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for StateNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable key identifying one persisted state object.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StateKey {
    namespace: StateNamespace,
    id: PersistenceId,
}

impl StateKey {
    /// Creates a state key.
    #[must_use]
    pub fn new(namespace: StateNamespace, id: PersistenceId) -> Self {
        Self { namespace, id }
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &StateNamespace {
        &self.namespace
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> PersistenceId {
        self.id
    }
}

/// Schema identity attached to persisted bytes.
///
/// Serialization itself remains the responsibility of
/// `resilience::serialization`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion {
    major: u32,
    minor: u32,
}

impl SchemaVersion {
    /// Creates a schema version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

/// Identifies the algorithm used for a payload digest.
///
/// The persistence layer does not implement hashing. It merely carries the
/// integrity metadata produced by the serialization/checkpoint/security layer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntegrityAlgorithm(String);

impl IntegrityAlgorithm {
    /// Creates an integrity algorithm identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, PersistenceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PersistenceError::InvalidIntegrityMetadata(
                "integrity algorithm must not be empty".to_owned(),
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(PersistenceError::InvalidIntegrityMetadata(
                "integrity algorithm must not contain control characters".to_owned(),
            ));
        }

        Ok(Self(value))
    }

    /// Returns the algorithm identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Integrity metadata associated with a persisted payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegrityMetadata {
    algorithm: IntegrityAlgorithm,
    digest: Vec<u8>,
}

impl IntegrityMetadata {
    /// Creates integrity metadata.
    ///
    /// The digest is copied so the persisted representation remains owned by
    /// the state object.
    pub fn new(
        algorithm: IntegrityAlgorithm,
        digest: impl Into<Vec<u8>>,
    ) -> Result<Self, PersistenceError> {
        let digest = digest.into();

        if digest.is_empty() {
            return Err(PersistenceError::InvalidIntegrityMetadata(
                "integrity digest must not be empty".to_owned(),
            ));
        }

        Ok(Self { algorithm, digest })
    }

    /// Returns the digest algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &IntegrityAlgorithm {
        &self.algorithm
    }

    /// Returns the digest bytes.
    #[must_use]
    pub fn digest(&self) -> &[u8] {
        &self.digest
    }
}

/// Persisted state object.
///
/// The payload is intentionally opaque. Serialization/deserialization belongs
/// to `resilience::serialization`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistedState {
    key: StateKey,
    revision: PersistenceRevision,
    schema: SchemaVersion,
    payload: Vec<u8>,
    integrity: Option<IntegrityMetadata>,
}

impl PersistedState {
    /// Creates a new persisted-state value.
    ///
    /// The revision supplied here is normally obtained from a prior read.
    /// The store assigns the next revision during a successful write.
    pub fn new(
        key: StateKey,
        revision: PersistenceRevision,
        schema: SchemaVersion,
        payload: impl Into<Vec<u8>>,
        integrity: Option<IntegrityMetadata>,
    ) -> Result<Self, PersistenceError> {
        let payload = payload.into();

        if payload.is_empty() {
            return Err(PersistenceError::EmptyPayload);
        }

        Ok(Self {
            key,
            revision,
            schema,
            payload,
            integrity,
        })
    }

    /// Returns the state key.
    #[must_use]
    pub fn key(&self) -> &StateKey {
        &self.key
    }

    /// Returns the revision.
    #[must_use]
    pub const fn revision(&self) -> PersistenceRevision {
        self.revision
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema(&self) -> SchemaVersion {
        self.schema
    }

    /// Returns the opaque serialized payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Returns optional integrity metadata.
    #[must_use]
    pub fn integrity(&self) -> Option<&IntegrityMetadata> {
        self.integrity.as_ref()
    }

    /// Consumes the object and returns its payload.
    #[must_use]
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    fn with_revision(mut self, revision: PersistenceRevision) -> Self {
        self.revision = revision;
        self
    }
}

/// Result of a successful persistence write.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistReceipt {
    key: StateKey,
    previous_revision: PersistenceRevision,
    revision: PersistenceRevision,
}

impl PersistReceipt {
    /// Returns the persisted key.
    #[must_use]
    pub fn key(&self) -> &StateKey {
        &self.key
    }

    /// Returns the previous revision.
    #[must_use]
    pub const fn previous_revision(&self) -> PersistenceRevision {
        self.previous_revision
    }

    /// Returns the newly assigned revision.
    #[must_use]
    pub const fn revision(&self) -> PersistenceRevision {
        self.revision
    }
}

/// Metadata returned when a state object is listed without loading its payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateMetadata {
    key: StateKey,
    revision: PersistenceRevision,
    schema: SchemaVersion,
}

impl StateMetadata {
    /// Creates metadata from a persisted state.
    #[must_use]
    pub fn from_state(state: &PersistedState) -> Self {
        Self {
            key: state.key.clone(),
            revision: state.revision,
            schema: state.schema,
        }
    }

    /// Returns the key.
    #[must_use]
    pub fn key(&self) -> &StateKey {
        &self.key
    }

    /// Returns the revision.
    #[must_use]
    pub const fn revision(&self) -> PersistenceRevision {
        self.revision
    }

    /// Returns the schema.
    #[must_use]
    pub const fn schema(&self) -> SchemaVersion {
        self.schema
    }
}

/// Persistence failures.
///
/// This type is deliberately independent of the concrete resilience error
/// hierarchy. `errors/error.rs` can wrap/map it without introducing a reverse
/// dependency from this low-level state component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PersistenceError {
    /// Namespace was invalid.
    InvalidNamespace(String),

    /// Integrity metadata was invalid.
    InvalidIntegrityMetadata(String),

    /// A persisted payload may not be empty.
    EmptyPayload,

    /// Requested state object does not exist.
    NotFound(StateKey),

    /// A conditional write found a newer revision than expected.
    Conflict {
        key: StateKey,
        expected: PersistenceRevision,
        actual: PersistenceRevision,
    },

    /// A revision cannot be incremented any further.
    RevisionExhausted,

    /// An attempt was made to delete a non-terminal or otherwise protected
    /// object when the concrete store enforces that policy.
    DeletionRejected(StateKey),

    /// Internal synchronization failed.
    Synchronization,

    /// Storage backend is unavailable.
    Unavailable(String),

    /// Storage backend rejected an operation.
    Backend(String),

    /// Persisted data is incompatible with the requested schema.
    IncompatibleSchema {
        key: StateKey,
        stored: SchemaVersion,
        requested: SchemaVersion,
    },
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNamespace(message) => {
                write!(formatter, "invalid persistence namespace: {message}")
            }
            Self::InvalidIntegrityMetadata(message) => {
                write!(formatter, "invalid integrity metadata: {message}")
            }
            Self::EmptyPayload => formatter.write_str("persisted payload must not be empty"),
            Self::NotFound(key) => write!(formatter, "persisted state not found: {key}"),
            Self::Conflict {
                key,
                expected,
                actual,
            } => write!(
                formatter,
                "persistence conflict for {key}: expected revision {expected}, \
                 actual revision {actual}"
            ),
            Self::RevisionExhausted => {
                formatter.write_str("persistence revision counter exhausted")
            }
            Self::DeletionRejected(key) => {
                write!(formatter, "deletion rejected for persisted state: {key}")
            }
            Self::Synchronization => {
                formatter.write_str("persistence synchronization failed")
            }
            Self::Unavailable(message) => {
                write!(formatter, "persistence backend unavailable: {message}")
            }
            Self::Backend(message) => {
                write!(formatter, "persistence backend failure: {message}")
            }
            Self::IncompatibleSchema {
                key,
                stored,
                requested,
            } => write!(
                formatter,
                "incompatible schema for {key}: stored {}.{}, requested {}.{}",
                stored.major(),
                stored.minor(),
                requested.major(),
                requested.minor()
            ),
        }
    }
}

impl std::error::Error for PersistenceError {}

impl fmt::Display for StateKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.id)
    }
}

/// Backend-neutral persistence interface.
///
/// Implementations must provide atomic replacement semantics for `store`.
///
/// A successful conditional write must make the new object visible as one
/// logical operation. Partial state must never be observable through `load`.
pub trait StatePersistence: Send + Sync {
    /// Loads one state object.
    fn load(&self, key: &StateKey) -> Result<Option<PersistedState>, PersistenceError>;

    /// Stores a new or replacement state object.
    ///
    /// The supplied object's revision is interpreted as the caller's expected
    /// revision:
    ///
    /// - `0` means the object must not currently exist;
    /// - a non-zero revision means that exact revision must currently exist.
    ///
    /// A successful operation receives the next revision.
    fn store(&self, state: PersistedState) -> Result<PersistReceipt, PersistenceError>;

    /// Deletes a state object only if the caller still observes the supplied
    /// revision.
    ///
    /// Implementations may additionally require higher-level lifecycle checks
    /// before allowing deletion.
    fn delete(
        &self,
        key: &StateKey,
        expected_revision: PersistenceRevision,
    ) -> Result<(), PersistenceError>;

    /// Lists metadata for one namespace.
    ///
    /// Results must be deterministic. Implementations should avoid loading
    /// payloads for this operation.
    fn list(
        &self,
        namespace: &StateNamespace,
    ) -> Result<Vec<StateMetadata>, PersistenceError>;
}

/// Shared persistence handle used by resilience controllers and state owners.
///
/// `Arc` provides shared ownership. Synchronization remains an implementation
/// detail of the concrete persistence backend.
pub type SharedStatePersistence = Arc<dyn StatePersistence>;

/// Thread-safe in-memory persistence implementation.
///
/// This is intended for:
///
/// - unit tests;
/// - deterministic replay tests;
/// - simulator tests;
/// - fault-injection tests;
/// - local development.
///
/// It is **not** durable storage and must not be advertised as such.
#[derive(Clone, Default)]
pub struct InMemoryStatePersistence {
    objects: Arc<RwLock<BTreeMap<StateKey, PersistedState>>>,
}

impl InMemoryStatePersistence {
    /// Creates an empty in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of stored objects.
    pub fn len(&self) -> Result<usize, PersistenceError> {
        let guard = self
            .objects
            .read()
            .map_err(|_| PersistenceError::Synchronization)?;

        Ok(guard.len())
    }

    /// Returns whether the store contains no objects.
    pub fn is_empty(&self) -> Result<bool, PersistenceError> {
        Ok(self.len()? == 0)
    }

    /// Removes every object.
    ///
    /// This operation is intentionally named as a test/development primitive.
    /// Production controllers should not expose an equivalent destructive
    /// operation without explicit lifecycle authorization.
    pub fn clear(&self) -> Result<(), PersistenceError> {
        let mut guard = self
            .objects
            .write()
            .map_err(|_| PersistenceError::Synchronization)?;

        guard.clear();
        Ok(())
    }
}

impl StatePersistence for InMemoryStatePersistence {
    fn load(&self, key: &StateKey) -> Result<Option<PersistedState>, PersistenceError> {
        let guard = self
            .objects
            .read()
            .map_err(|_| PersistenceError::Synchronization)?;

        Ok(guard.get(key).cloned())
    }

    fn store(&self, state: PersistedState) -> Result<PersistReceipt, PersistenceError> {
        let mut guard = self
            .objects
            .write()
            .map_err(|_| PersistenceError::Synchronization)?;

        let current = guard.get(&state.key);

        let expected = state.revision;

        let next_revision = match current {
            None => {
                if expected != PersistenceRevision::INITIAL {
                    return Err(PersistenceError::Conflict {
                        key: state.key.clone(),
                        expected,
                        actual: PersistenceRevision::INITIAL,
                    });
                }

                PersistenceRevision::INITIAL.next()?
            }
            Some(existing) => {
                if existing.revision != expected {
                    return Err(PersistenceError::Conflict {
                        key: state.key.clone(),
                        expected,
                        actual: existing.revision,
                    });
                }

                existing.revision.next()?
            }
        };

        let previous_revision = current
            .map(PersistedState::revision)
            .unwrap_or(PersistenceRevision::INITIAL);

        let key = state.key.clone();

        guard.insert(key.clone(), state.with_revision(next_revision));

        Ok(PersistReceipt {
            key,
            previous_revision,
            revision: next_revision,
        })
    }

    fn delete(
        &self,
        key: &StateKey,
        expected_revision: PersistenceRevision,
    ) -> Result<(), PersistenceError> {
        let mut guard = self
            .objects
            .write()
            .map_err(|_| PersistenceError::Synchronization)?;

        let existing = guard
            .get(key)
            .ok_or_else(|| PersistenceError::NotFound(key.clone()))?;

        if existing.revision != expected_revision {
            return Err(PersistenceError::Conflict {
                key: key.clone(),
                expected: expected_revision,
                actual: existing.revision,
            });
        }

        guard.remove(key);
        Ok(())
    }

    fn list(
        &self,
        namespace: &StateNamespace,
    ) -> Result<Vec<StateMetadata>, PersistenceError> {
        let guard = self
            .objects
            .read()
            .map_err(|_| PersistenceError::Synchronization)?;

        Ok(guard
            .range(
                StateKey::new(namespace.clone(), PersistenceId::from_u128(0))
                    ..=StateKey::new(namespace.clone(), PersistenceId::from_u128(u128::MAX)),
            )
            .map(|(_, state)| StateMetadata::from_state(state))
            .collect())
    }
}

/// A persistence transaction helper implementing optimistic concurrency.
///
/// The helper does not hold a lock between `load` and `commit`. This is
/// intentional: durable stores may be remote, distributed, or backed by a
/// database. Concurrency is resolved by the backend's conditional write.
pub struct StateTransaction {
    original: PersistedState,
    replacement: Option<PersistedState>,
}

impl StateTransaction {
    /// Starts a transaction from an existing state object.
    #[must_use]
    pub fn from_existing(state: PersistedState) -> Self {
        Self {
            original: state,
            replacement: None,
        }
    }

    /// Returns the original state.
    #[must_use]
    pub fn original(&self) -> &PersistedState {
        &self.original
    }

    /// Stages a replacement payload using the original revision as the
    /// optimistic-concurrency expectation.
    pub fn replace(
        &mut self,
        schema: SchemaVersion,
        payload: impl Into<Vec<u8>>,
        integrity: Option<IntegrityMetadata>,
    ) -> Result<(), PersistenceError> {
        let replacement = PersistedState::new(
            self.original.key.clone(),
            self.original.revision,
            schema,
            payload,
            integrity,
        )?;

        self.replacement = Some(replacement);
        Ok(())
    }

    /// Returns whether a replacement has been staged.
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.replacement.is_some()
    }

    /// Commits the staged replacement.
    pub fn commit(
        self,
        persistence: &dyn StatePersistence,
    ) -> Result<PersistReceipt, PersistenceError> {
        let replacement = self.replacement.ok_or_else(|| {
            PersistenceError::Backend("transaction has no staged replacement".to_owned())
        })?;

        persistence.store(replacement)
    }
}

/// Loads a state and creates an optimistic transaction.
///
/// This helper keeps the common read-modify-write pattern explicit and avoids
/// silently converting concurrent modifications into last-writer-wins writes.
pub fn begin_transaction(
    persistence: &dyn StatePersistence,
    key: &StateKey,
) -> Result<StateTransaction, PersistenceError> {
    let state = persistence
        .load(key)?
        .ok_or_else(|| PersistenceError::NotFound(key.clone()))?;

    Ok(StateTransaction::from_existing(state))
}

/// Persists a brand-new state object.
///
/// The object must not already exist. This function makes the creation
/// expectation explicit instead of using an unconditional overwrite.
pub fn create(
    persistence: &dyn StatePersistence,
    key: StateKey,
    schema: SchemaVersion,
    payload: impl Into<Vec<u8>>,
    integrity: Option<IntegrityMetadata>,
) -> Result<PersistReceipt, PersistenceError> {
    let state = PersistedState::new(
        key,
        PersistenceRevision::INITIAL,
        schema,
        payload,
        integrity,
    )?;

    persistence.store(state)
}

/// Replaces an existing state object using its observed revision.
///
/// This is the preferred operation for state/machine.rs,
/// state/execution.rs, state/logical.rs, state/physical.rs and
/// state/recovery.rs after they have serialized a new snapshot.
pub fn replace(
    persistence: &dyn StatePersistence,
    current: &PersistedState,
    schema: SchemaVersion,
    payload: impl Into<Vec<u8>>,
    integrity: Option<IntegrityMetadata>,
) -> Result<PersistReceipt, PersistenceError> {
    let state = PersistedState::new(
        current.key.clone(),
        current.revision,
        schema,
        payload,
        integrity,
    )?;

    persistence.store(state)
}

/// Deletes an object only when the caller has the current revision.
///
/// This function deliberately does not provide a force-delete operation.
pub fn delete(
    persistence: &dyn StatePersistence,
    current: &PersistedState,
) -> Result<(), PersistenceError> {
    persistence.delete(&current.key, current.revision)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn namespace() -> StateNamespace {
        StateNamespace::new("test.state").expect("valid namespace")
    }

    fn key(value: u128) -> StateKey {
        StateKey::new(namespace(), PersistenceId::from_u128(value))
    }

    fn payload(value: u8) -> Vec<u8> {
        vec![value]
    }

    #[test]
    fn namespace_rejects_empty_values() {
        let result = StateNamespace::new("");

        assert!(matches!(result, Err(PersistenceError::InvalidNamespace(_))));
    }

    #[test]
    fn namespace_rejects_control_characters() {
        let result = StateNamespace::new("test\nstate");

        assert!(matches!(result, Err(PersistenceError::InvalidNamespace(_))));
    }

    #[test]
    fn persistence_id_is_opaque_and_orderable() {
        let first = PersistenceId::from_u128(1);
        let second = PersistenceId::from_u128(2);

        assert!(first < second);
        assert_eq!(first.as_u128(), 1);
    }

    #[test]
    fn new_object_starts_at_revision_one() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        let receipt = create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        assert_eq!(receipt.previous_revision(), PersistenceRevision::INITIAL);
        assert_eq!(receipt.revision(), PersistenceRevision::from_u64(1));

        let loaded = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        assert_eq!(loaded.revision(), PersistenceRevision::from_u64(1));
    }

    #[test]
    fn creation_rejects_existing_object() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("first creation succeeds");

        let result = create(
            &store,
            object_key,
            SchemaVersion::new(1, 0),
            payload(2),
            None,
        );

        assert!(matches!(result, Err(PersistenceError::Conflict { .. })));
    }

    #[test]
    fn replacement_requires_current_revision() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let current = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        let receipt = replace(
            &store,
            &current,
            SchemaVersion::new(1, 1),
            payload(2),
            None,
        )
        .expect("replacement succeeds");

        assert_eq!(receipt.revision(), PersistenceRevision::from_u64(2));

        let updated = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        assert_eq!(updated.payload(), &[2]);
        assert_eq!(updated.revision(), PersistenceRevision::from_u64(2));
    }

    #[test]
    fn stale_writer_is_rejected() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let stale = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        replace(
            &store,
            &stale,
            SchemaVersion::new(1, 1),
            payload(2),
            None,
        )
        .expect("first replacement succeeds");

        let result = replace(
            &store,
            &stale,
            SchemaVersion::new(1, 2),
            payload(3),
            None,
        );

        assert!(matches!(
            result,
            Err(PersistenceError::Conflict {
                expected: PersistenceRevision(1),
                actual: PersistenceRevision(2),
                ..
            })
        ));
    }

    #[test]
    fn transaction_is_optimistic() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let mut transaction =
            begin_transaction(&store, &object_key).expect("transaction begins");

        assert!(!transaction.is_dirty());

        transaction
            .replace(SchemaVersion::new(1, 1), payload(2), None)
            .expect("replacement stages");

        assert!(transaction.is_dirty());

        let receipt = transaction.commit(&store).expect("commit succeeds");

        assert_eq!(receipt.revision(), PersistenceRevision::from_u64(2));
    }

    #[test]
    fn delete_requires_current_revision() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let current = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        delete(&store, &current).expect("delete succeeds");

        assert!(
            store
                .load(&object_key)
                .expect("load succeeds")
                .is_none()
        );
    }

    #[test]
    fn stale_delete_is_rejected() {
        let store = InMemoryStatePersistence::new();
        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let stale = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        replace(
            &store,
            &stale,
            SchemaVersion::new(1, 1),
            payload(2),
            None,
        )
        .expect("replacement succeeds");

        let result = delete(&store, &stale);

        assert!(matches!(result, Err(PersistenceError::Conflict { .. })));
    }

    #[test]
    fn list_is_namespace_scoped_and_deterministic() {
        let store = InMemoryStatePersistence::new();

        let first = StateKey::new(namespace(), PersistenceId::from_u128(1));
        let second = StateKey::new(namespace(), PersistenceId::from_u128(2));
        let other_namespace =
            StateNamespace::new("other.state").expect("valid namespace");
        let third = StateKey::new(other_namespace, PersistenceId::from_u128(3));

        create(
            &store,
            second.clone(),
            SchemaVersion::new(1, 0),
            payload(2),
            None,
        )
        .expect("creation succeeds");

        create(
            &store,
            first.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        create(
            &store,
            third,
            SchemaVersion::new(1, 0),
            payload(3),
            None,
        )
        .expect("creation succeeds");

        let entries = store
            .list(&namespace())
            .expect("listing succeeds");

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].key().id(), PersistenceId::from_u128(1));
        assert_eq!(entries[1].key().id(), PersistenceId::from_u128(2));
    }

    #[test]
    fn empty_payload_is_rejected() {
        let store = InMemoryStatePersistence::new();

        let result = create(
            &store,
            key(1),
            SchemaVersion::new(1, 0),
            Vec::<u8>::new(),
            None,
        );

        assert!(matches!(result, Err(PersistenceError::EmptyPayload)));
    }

    #[test]
    fn integrity_metadata_requires_a_digest() {
        let algorithm =
            IntegrityAlgorithm::new("test-digest").expect("valid algorithm");

        let result = IntegrityMetadata::new(algorithm, Vec::<u8>::new());

        assert!(matches!(
            result,
            Err(PersistenceError::InvalidIntegrityMetadata(_))
        ));
    }

    #[test]
    fn integrity_metadata_is_preserved() {
        let store = InMemoryStatePersistence::new();
        let algorithm =
            IntegrityAlgorithm::new("test-digest").expect("valid algorithm");

        let integrity =
            IntegrityMetadata::new(algorithm, vec![1, 2, 3]).expect("valid integrity");

        let object_key = key(1);

        create(
            &store,
            object_key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            Some(integrity.clone()),
        )
        .expect("creation succeeds");

        let loaded = store
            .load(&object_key)
            .expect("load succeeds")
            .expect("object exists");

        assert_eq!(loaded.integrity(), Some(&integrity));
    }

    #[test]
    fn shared_store_supports_concurrent_owners() {
        let store = InMemoryStatePersistence::new();

        let key = key(1);

        create(
            &store,
            key.clone(),
            SchemaVersion::new(1, 0),
            payload(1),
            None,
        )
        .expect("creation succeeds");

        let first = store
            .load(&key)
            .expect("load succeeds")
            .expect("object exists");

        let second = store
            .load(&key)
            .expect("load succeeds")
            .expect("object exists");

        replace(
            &store,
            &first,
            SchemaVersion::new(1, 1),
            payload(2),
            None,
        )
        .expect("first writer succeeds");

        let result = replace(
            &store,
            &second,
            SchemaVersion::new(1, 2),
            payload(3),
            None,
        );

        assert!(matches!(result, Err(PersistenceError::Conflict { .. })));
    }

    #[test]
    fn persistence_id_display_is_fixed_width_hex() {
        let id = PersistenceId::from_u128(0xab);

        assert_eq!(
            id.to_string(),
            "000000000000000000000000000000ab"
        );
    }
}