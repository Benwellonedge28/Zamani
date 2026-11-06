
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Persistent Incremental Cache
//!
//! Path: src/compiler/incremental/persistence.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Persist compiler cache entries across compiler processes and restarts.
//!
//! This module complements the existing in-memory BackendCache. It does not
//! introduce another cache-key algorithm, fingerprint algorithm, dependency
//! graph, invalidation engine, semantic cache, or artifact representation.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Persistent cache directory management.
//! - Versioned on-disk entry framing.
//! - Durable cache-entry writes.
//! - Cache-entry loading.
//! - Persistent payload integrity verification.
//! - Persistent entry removal and clearing.
//! - Persistent storage configuration and errors.
//!
//! This module does not own:
//! - Cache-key construction.
//! - Source or IR fingerprint construction.
//! - Semantic dependency discovery.
//! - Dependency graph mutation.
//! - Invalidation planning.
//! - Compilation scheduling.
//! - Artifact semantic verification.
//! - Target discovery or hardware allocation.
//! - Runtime execution.
//!
//! AUTHORITATIVE INTEGRATIONS
//!
//! - backend/cache.rs: CacheKey and in-memory BackendCache.
//! - compiler/incremental/cache.rs: compiler-facing cache integration.
//! - compiler/incremental/fingerprints.rs: compiler fingerprints.
//! - zuir/incremental/cache_keys.rs: canonical ZUIR cache-key derivation.
//! - zuir/fingerprint/stable_hash.rs: canonical ZUIR fingerprinting.
//! - zuir/incremental/dependencies.rs: dependency graph.
//! - zuir/incremental/invalidation.rs: invalidation planning.
//! - frontend/semantic/incremental/: semantic cache ownership.
//!
//! PERSISTENCE CONTRACT
//!
//! Every stored entry contains:
//! - A fixed magic identifier.
//! - An on-disk format version.
//! - The complete cache-key digest.
//! - The payload length.
//! - A domain-separated SHA-256 payload digest.
//! - The artifact payload.
//!
//! An entry is returned only when its format, key, length, and digest
//! all validate.
//!
//! A corrupt entry is an explicit error, not a successful cache hit.
//!
//! Writes use a temporary file in the destination directory followed by
//! an atomic hard-link publication. This prevents readers from observing
//! partially written entries and prevents concurrent writers from
//! silently replacing an existing entry.
//!
//! Filesystems that do not support hard links cannot use this persistence
//! implementation. The caller must treat that as a cache-storage failure
//! and continue compilation without persistence if its policy permits.
//!
//! SCALABILITY
//!
//! There are no language-level limits on program size, IR size, number of
//! artifacts, hardware devices, processors, qubits, or dependency count.
//!
//! An optional per-entry byte budget is an operational storage policy,
//! not a semantic or hardware capacity limit.
//!
//! Actual operation remains subject to filesystem capacity, process
//! address space, available memory, operating-system limits, and I/O.
//!
//! CONCURRENCY
//!
//! Clones of one PersistentCompilationCache share an in-process mutex.
//! Entry publication is atomic for readers and uses create-new temporary
//! files to avoid temporary-name collisions.
//!
//! The filesystem publication operation does not provide a cross-process
//! transaction for clear/remove versus store. Such races can cause a cache
//! miss, but cannot change program semantics.
//!
//! A cache key must identify every input capable of changing its payload.
//! Different payloads for the same key are reported as conflicts.
//!
//! SECURITY
//!
//! Cache keys and SHA-256 digests provide identity and integrity checking.
//! They do not authenticate the producer or establish authorization.
//! Untrusted cached artifacts still require normal artifact validation,
//! compatibility checking, and security verification before use.
//!
//! No cached payload should be executed merely because its digest matches.

use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use sha2::{Digest, Sha256};

use crate::backend::cache::CacheKey;

// -----------------------------------------------------------------------------
// ON-DISK FORMAT
// -----------------------------------------------------------------------------

/// Persistent cache file magic.
///
/// Eight bytes identify this file format independently of its extension.
const FILE_MAGIC: &[u8; 8] = b"ZMCACHE\0";

/// Persistent cache file-format version.
///
/// Bump this when the envelope layout or interpretation changes incompatibly.
pub const PERSISTENCE_FORMAT_VERSION: u32 = 1;

/// Domain separator for persistent payload integrity digests.
const PAYLOAD_DOMAIN: &[u8] =
    b"zamani.compiler.persistent-cache-payload.v1\0";

/// Number of bytes in a SHA-256 digest.
const DIGEST_LENGTH: usize = 32;

/// Fixed header size:
///
/// magic             8 bytes
/// format version    4 bytes
/// cache key        32 bytes
/// payload length   16 bytes
/// payload digest   32 bytes
const HEADER_LENGTH: usize = 8 + 4 + 32 + 16 + 32;

/// Extension used for persistent cache entries.
const ENTRY_EXTENSION: &str = "zcache";

/// Process-local unique temporary-file counter.
static TEMPORARY_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

// -----------------------------------------------------------------------------
// CONFIGURATION
// -----------------------------------------------------------------------------

/// Operational policy for persistent cache storage.
///
/// `max_entry_bytes: None` means there is no cache-imposed per-entry
/// byte limit. It does not mean that the filesystem has unlimited capacity.
///
/// `sync_on_commit` requests a file synchronization before publication.
/// This improves durability but can reduce write throughput.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistenceConfig {
    /// Optional maximum payload size for an individual entry.
    pub max_entry_bytes: Option<u64>,

    /// Synchronize the completed temporary file before publication.
    pub sync_on_commit: bool,
}

impl PersistenceConfig {
    /// Create an unrestricted per-entry configuration.
    ///
    /// Filesystem and process resource limits still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_entry_bytes: None,
            sync_on_commit: true,
        }
    }

    /// Create a configuration with an explicit per-entry budget.
    #[must_use]
    pub const fn bounded(
        max_entry_bytes: u64,
        sync_on_commit: bool,
    ) -> Self {
        Self {
            max_entry_bytes: Some(max_entry_bytes),
            sync_on_commit,
        }
    }

    fn accepts(self, payload_length: u64) -> bool {
        match self.max_entry_bytes {
            Some(limit) => payload_length <= limit,
            None => true,
        }
    }
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self::unbounded()
    }
}

// -----------------------------------------------------------------------------
// PUBLIC RESULTS
// -----------------------------------------------------------------------------

/// Result of a persistent insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistOutcome {
    /// A new persistent entry was published.
    Stored,

    /// The identical entry was already present.
    AlreadyPresent,
}

// -----------------------------------------------------------------------------
// ERRORS
// -----------------------------------------------------------------------------

/// Persistent-cache operation that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceOperation {
    CreateDirectory,
    ReadDirectory,
    OpenEntry,
    ReadEntry,
    CreateTemporaryFile,
    WriteTemporaryFile,
    SynchronizeTemporaryFile,
    PublishEntry,
    RemoveTemporaryFile,
    RemoveEntry,
    ClearDirectory,
}

/// Errors produced by persistent cache operations.
#[derive(Debug)]
pub enum PersistenceError {
    /// An underlying filesystem operation failed.
    Io {
        operation: PersistenceOperation,
        path: PathBuf,
        source: io::Error,
    },

    /// The configured cache root is not a directory.
    InvalidRoot(PathBuf),

    /// The entry does not contain a recognized persistent-cache format.
    InvalidFormat(PathBuf),

    /// The entry uses an unsupported format version.
    UnsupportedFormatVersion {
        path: PathBuf,
        found: u32,
        supported: u32,
    },

    /// The entry's stored key does not match the requested key.
    KeyMismatch(PathBuf),

    /// The entry's declared payload length does not match its file length.
    LengthMismatch(PathBuf),

    /// The payload exceeds the configured per-entry budget.
    EntryTooLarge {
        actual: u64,
        maximum: u64,
    },

    /// The payload cannot be represented or allocated by this process.
    AllocationFailure,

    /// The persistent payload digest does not match its contents.
    IntegrityFailure(PathBuf),

    /// The same cache key already identifies a different payload.
    ///
    /// This normally indicates incomplete cache-key inputs, nondeterministic
    /// output, or an identity collision. It must not be silently overwritten.
    ConflictingContent(PathBuf),

    /// The temporary-file sequence cannot advance without wrapping.
    TemporaryNameExhausted,

    /// The in-process synchronization primitive was poisoned.
    LockPoisoned,
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                formatter,
                "persistent cache {operation:?} failed for {}: {source}",
                path.display()
            ),

            Self::InvalidRoot(path) => write!(
                formatter,
                "persistent cache root is not a directory: {}",
                path.display()
            ),

            Self::InvalidFormat(path) => write!(
                formatter,
                "invalid persistent cache entry format: {}",
                path.display()
            ),

            Self::UnsupportedFormatVersion {
                path,
                found,
                supported,
            } => write!(
                formatter,
                "unsupported persistent cache format {found} in {}; \
                 supported format is {supported}",
                path.display()
            ),

            Self::KeyMismatch(path) => write!(
                formatter,
                "persistent cache key mismatch: {}",
                path.display()
            ),

            Self::LengthMismatch(path) => write!(
                formatter,
                "persistent cache payload length mismatch: {}",
                path.display()
            ),

            Self::EntryTooLarge { actual, maximum } => write!(
                formatter,
                "persistent cache entry size {actual} exceeds configured \
                 maximum {maximum}"
            ),

            Self::AllocationFailure => {
                formatter.write_str(
                    "unable to reserve memory for persistent cache payload"
                )
            }

            Self::IntegrityFailure(path) => write!(
                formatter,
                "persistent cache integrity verification failed: {}",
                path.display()
            ),

            Self::ConflictingContent(path) => write!(
                formatter,
                "different payload already exists for persistent cache key: {}",
                path.display()
            ),

            Self::TemporaryNameExhausted => {
                formatter.write_str(
                    "persistent cache temporary-file identifier exhausted"
                )
            }

            Self::LockPoisoned => {
                formatter.write_str(
                    "persistent cache synchronization lock is poisoned"
                )
            }
        }
    }
}

impl Error for PersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

// -----------------------------------------------------------------------------
// INTERNAL STATE
// -----------------------------------------------------------------------------

struct StoreInner {
    root: PathBuf,
    config: PersistenceConfig,
    operation_lock: Mutex<()>,
}

/// Thread-safe persistent compilation cache.
///
/// Clones share the same directory, configuration, and in-process
/// operation lock.
#[derive(Clone)]
pub struct PersistentCompilationCache {
    inner: Arc<StoreInner>,
}

impl fmt::Debug for PersistentCompilationCache {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PersistentCompilationCache")
            .field("root", &self.inner.root)
            .field("config", &self.inner.config)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// CONSTRUCTION
// -----------------------------------------------------------------------------

impl PersistentCompilationCache {
    /// Open or create a persistent compilation-cache directory.
    ///
    /// The directory is canonicalized after creation so later operations
    /// use a stable absolute root.
    ///
    /// This operation does not delete old entries or perform implicit
    /// garbage collection.
    pub fn open(
        root: impl AsRef<Path>,
        config: PersistenceConfig,
    ) -> Result<Self, PersistenceError> {
        let requested_root = root.as_ref();

        fs::create_dir_all(requested_root).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::CreateDirectory,
                path: requested_root.to_path_buf(),
                source,
            }
        })?;

        let metadata = fs::metadata(requested_root).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadDirectory,
                path: requested_root.to_path_buf(),
                source,
            }
        })?;

        if !metadata.is_dir() {
            return Err(PersistenceError::InvalidRoot(
                requested_root.to_path_buf(),
            ));
        }

        let root = fs::canonicalize(requested_root).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadDirectory,
                path: requested_root.to_path_buf(),
                source,
            }
        })?;

        Ok(Self {
            inner: Arc::new(StoreInner {
                root,
                config,
                operation_lock: Mutex::new(()),
            }),
        })
    }

    /// Return the canonical persistent-cache directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.inner.root
    }

    /// Return the active persistent-cache configuration.
    #[must_use]
    pub const fn config(&self) -> PersistenceConfig {
        self.inner.config
    }

    fn lock(&self) -> Result<MutexGuard<'_, ()>, PersistenceError> {
        self.inner
            .operation_lock
            .lock()
            .map_err(|_| PersistenceError::LockPoisoned)
    }

    fn entry_path(&self, key: &CacheKey) -> PathBuf {
        self.inner
            .root
            .join(format!("{}.{}", key.to_hex(), ENTRY_EXTENSION))
    }

    // -------------------------------------------------------------------------
    // LOADING
    // -------------------------------------------------------------------------

    /// Load and verify one persistent cache entry.
    ///
    /// Returns:
    /// - `Ok(Some(payload))` when a complete valid entry exists.
    /// - `Ok(None)` when no entry exists.
    /// - `Err(...)` when the entry is corrupt, incompatible, or inaccessible.
    ///
    /// The caller must still validate the artifact's semantic format,
    /// compatibility, provenance, and target requirements before reuse.
    pub fn load(
        &self,
        key: &CacheKey,
    ) -> Result<Option<Vec<u8>>, PersistenceError> {
        let path = self.entry_path(key);

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(source) => {
                return Err(PersistenceError::Io {
                    operation: PersistenceOperation::OpenEntry,
                    path,
                    source,
                });
            }
        };

        let metadata = file.metadata().map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadEntry,
                path: path.clone(),
                source,
            }
        })?;

        let file_length = metadata.len();

        if file_length < HEADER_LENGTH as u64 {
            return Err(PersistenceError::InvalidFormat(path));
        }

        let mut header = [0_u8; HEADER_LENGTH];

        file.read_exact(&mut header).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadEntry,
                path: path.clone(),
                source,
            }
        })?;

        if &header[0..8] != FILE_MAGIC {
            return Err(PersistenceError::InvalidFormat(path));
        }

        let found_version = u32::from_be_bytes(
            header[8..12]
                .try_into()
                .map_err(|_| PersistenceError::InvalidFormat(path.clone()))?,
        );

        if found_version != PERSISTENCE_FORMAT_VERSION {
            return Err(PersistenceError::UnsupportedFormatVersion {
                path,
                found: found_version,
                supported: PERSISTENCE_FORMAT_VERSION,
            });
        }

        if header[12..44] != *key.as_bytes() {
            return Err(PersistenceError::KeyMismatch(path));
        }

        let declared_length = u128::from_be_bytes(
            header[44..60]
                .try_into()
                .map_err(|_| PersistenceError::InvalidFormat(path.clone()))?,
        );

        let payload_length = u64::try_from(declared_length)
            .map_err(|_| PersistenceError::LengthMismatch(path.clone()))?;

        if !self.inner.config.accepts(payload_length) {
            return Err(PersistenceError::EntryTooLarge {
                actual: payload_length,
                maximum: self.inner.config.max_entry_bytes.unwrap_or(0),
            });
        }

        let expected_file_length = (HEADER_LENGTH as u64)
            .checked_add(payload_length)
            .ok_or_else(|| PersistenceError::LengthMismatch(path.clone()))?;

        if file_length != expected_file_length {
            return Err(PersistenceError::LengthMismatch(path));
        }

        let payload_size = usize::try_from(payload_length)
            .map_err(|_| PersistenceError::AllocationFailure)?;

        let mut payload = Vec::new();

        payload
            .try_reserve_exact(payload_size)
            .map_err(|_| PersistenceError::AllocationFailure)?;

        payload.resize(payload_size, 0);

        file.read_exact(&mut payload).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadEntry,
                path: path.clone(),
                source,
            }
        })?;

        // Reject trailing data even if a file changes while being read.
        let mut trailing = [0_u8; 1];

        if file.read(&mut trailing).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ReadEntry,
                path: path.clone(),
                source,
            }
        })? != 0
        {
            return Err(PersistenceError::LengthMismatch(path));
        }

        let expected_digest = &header[60..92];
        let actual_digest = payload_digest(key, &payload);

        if expected_digest != actual_digest.as_slice() {
            return Err(PersistenceError::IntegrityFailure(path));
        }

        Ok(Some(payload))
    }

    /// Test whether a valid persistent entry exists.
    ///
    /// This verifies the complete payload rather than merely checking
    /// whether the corresponding file exists.
    pub fn contains(
        &self,
        key: &CacheKey,
    ) -> Result<bool, PersistenceError> {
        Ok(self.load(key)?.is_some())
    }

    // -------------------------------------------------------------------------
    // STORAGE
    // -------------------------------------------------------------------------

    /// Persist a successfully compiled and validated artifact.
    ///
    /// This method does not determine whether the artifact is semantically
    /// valid. The compiler pipeline must perform that validation first.
    ///
    /// If the key already exists:
    /// - Identical payloads return `AlreadyPresent`.
    /// - Different payloads return `ConflictingContent`.
    ///
    /// Existing entries are never silently replaced.
    pub fn store(
        &self,
        key: &CacheKey,
        payload: &[u8],
    ) -> Result<PersistOutcome, PersistenceError> {
        let payload_length = u64::try_from(payload.len())
            .map_err(|_| PersistenceError::AllocationFailure)?;

        if !self.inner.config.accepts(payload_length) {
            return Err(PersistenceError::EntryTooLarge {
                actual: payload_length,
                maximum: self.inner.config.max_entry_bytes.unwrap_or(0),
            });
        }

        let _guard = self.lock()?;
        let destination = self.entry_path(key);

        // Check for an existing entry before creating a new one.
        if let Some(existing) = self.load(key)? {
            return if existing == payload {
                Ok(PersistOutcome::AlreadyPresent)
            } else {
                Err(PersistenceError::ConflictingContent(destination))
            };
        }

        let (temporary_path, mut temporary_file) =
            self.create_temporary_file(key)?;

        let digest = payload_digest(key, payload);

        let write_result = (|| -> io::Result<()> {
            temporary_file.write_all(FILE_MAGIC)?;

            temporary_file.write_all(
                &PERSISTENCE_FORMAT_VERSION.to_be_bytes(),
            )?;

            temporary_file.write_all(key.as_bytes())?;

            temporary_file.write_all(
                &(payload_length as u128).to_be_bytes(),
            )?;

            temporary_file.write_all(&digest)?;

            temporary_file.write_all(payload)?;

            if self.inner.config.sync_on_commit {
                temporary_file.sync_all()?;
            }

            Ok(())
        })();

        if let Err(source) = write_result {
            drop(temporary_file);
            let _ = fs::remove_file(&temporary_path);

            return Err(PersistenceError::Io {
                operation: PersistenceOperation::WriteTemporaryFile,
                path: temporary_path,
                source,
            });
        }

        drop(temporary_file);

        // A hard link publishes the fully written temporary file without
        // exposing an incomplete destination file. It also prevents a
        // concurrent writer from replacing an existing destination.
        let publication = fs::hard_link(&temporary_path, &destination);

        // The temporary name is no longer needed after publication attempt.
        let cleanup_result = fs::remove_file(&temporary_path);

        if let Err(source) = cleanup_result {
            if source.kind() != io::ErrorKind::NotFound {
                return Err(PersistenceError::Io {
                    operation: PersistenceOperation::RemoveTemporaryFile,
                    path: temporary_path,
                    source,
                });
            }
        }

        match publication {
            Ok(()) => Ok(PersistOutcome::Stored),

            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                match self.load(key)? {
                    Some(existing) if existing == payload => {
                        Ok(PersistOutcome::AlreadyPresent)
                    }

                    Some(_) => {
                        Err(PersistenceError::ConflictingContent(
                            destination,
                        ))
                    }

                    None => Err(PersistenceError::Io {
                        operation: PersistenceOperation::PublishEntry,
                        path: destination,
                        source: error,
                    }),
                }
            }

            Err(source) => Err(PersistenceError::Io {
                operation: PersistenceOperation::PublishEntry,
                path: destination,
                source,
            }),
        }
    }

    // -------------------------------------------------------------------------
    // REMOVAL
    // -------------------------------------------------------------------------

    /// Remove one persistent entry.
    ///
    /// Returns false when the entry did not exist.
    pub fn remove(
        &self,
        key: &CacheKey,
    ) -> Result<bool, PersistenceError> {
        let _guard = self.lock()?;
        let path = self.entry_path(key);

        match fs::remove_file(&path) {
            Ok(()) => Ok(true),

            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Ok(false)
            }

            Err(source) => Err(PersistenceError::Io {
                operation: PersistenceOperation::RemoveEntry,
                path,
                source,
            }),
        }
    }

    /// Remove persistent entries belonging to this cache directory.
    ///
    /// Unrelated files and directories are left untouched.
    ///
    /// This method is intended for explicit cache-management operations.
    /// It does not modify source files, dependency graphs, semantic facts,
    /// IR, or program meaning.
    pub fn clear(&self) -> Result<usize, PersistenceError> {
        let _guard = self.lock()?;

        let entries = fs::read_dir(&self.inner.root).map_err(|source| {
            PersistenceError::Io {
                operation: PersistenceOperation::ClearDirectory,
                path: self.inner.root.clone(),
                source,
            }
        })?;

        let mut removed = 0_usize;

        for entry in entries {
            let entry = entry.map_err(|source| PersistenceError::Io {
                operation: PersistenceOperation::ClearDirectory,
                path: self.inner.root.clone(),
                source,
            })?;

            let path = entry.path();

            if path.extension().and_then(|value| value.to_str())
                != Some(ENTRY_EXTENSION)
            {
                continue;
            }

            let file_type = entry.file_type().map_err(|source| {
                PersistenceError::Io {
                    operation: PersistenceOperation::ClearDirectory,
                    path: path.clone(),
                    source,
                }
            })?;

            // Do not follow symbolic links or remove directories.
            if !file_type.is_file() {
                continue;
            }

            fs::remove_file(&path).map_err(|source| {
                PersistenceError::Io {
                    operation: PersistenceOperation::ClearDirectory,
                    path,
                    source,
                }
            })?;

            removed = removed
                .checked_add(1)
                .ok_or(PersistenceError::AllocationFailure)?;
        }

        Ok(removed)
    }

    // -------------------------------------------------------------------------
    // TEMPORARY FILE MANAGEMENT
    // -------------------------------------------------------------------------

    fn create_temporary_file(
        &self,
        key: &CacheKey,
    ) -> Result<(PathBuf, File), PersistenceError> {
        loop {
            let sequence = TEMPORARY_FILE_COUNTER
                .fetch_update(
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                    |current| current.checked_add(1),
                )
                .map_err(|_| PersistenceError::TemporaryNameExhausted)?;

            let filename = format!(
                ".{}.{}.{}.tmp",
                key.to_hex(),
                std::process::id(),
                sequence
            );

            let path = self.inner.root.join(filename);

            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(file) => return Ok((path, file)),

                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    // A stale temporary file or a previous process with a
                    // reused process identifier may occupy this name.
                    // Advance the sequence and try another unique name.
                    continue;
                }

                Err(source) => {
                    return Err(PersistenceError::Io {
                        operation:
                            PersistenceOperation::CreateTemporaryFile,
                        path,
                        source,
                    });
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// INTEGRITY
// -----------------------------------------------------------------------------

/// Compute a domain-separated digest over the key and payload.
///
/// Including the key prevents a valid payload file from being copied to
/// another cache-key filename and accepted as a different artifact.
fn payload_digest(key: &CacheKey, payload: &[u8]) -> [u8; DIGEST_LENGTH] {
    let mut hasher = Sha256::new();

    hasher.update(PAYLOAD_DOMAIN);
    hasher.update(key.as_bytes());
    hasher.update((payload.len() as u128).to_be_bytes());
    hasher.update(payload);

    hasher.finalize().into()
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::cache::{CacheKey, CacheKeyParts};

    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIRECTORY_COUNTER: AtomicU64 =
        AtomicU64::new(0);

    fn test_directory() -> PathBuf {
        let sequence = TEST_DIRECTORY_COUNTER.fetch_add(
            1,
            Ordering::Relaxed,
        );

        std::env::temp_dir().join(format!(
            "zamani-persistence-test-{}-{}",
            std::process::id(),
            sequence
        ))
    }

    fn test_key(input: &str) -> CacheKey {
        CacheKey::from_parts(CacheKeyParts {
            schema_version: "test-schema-v1",
            input_fingerprint: input,
            compiler_fingerprint: "compiler-v1",
            backend_id: "test-stage",
            target_id: "target-independent",
            config_fingerprint: "config-v1",
            toolchain_fingerprint: None,
            dependency_fingerprint: "dependencies-v1",
            artifact_kind: "test-artifact",
        })
        .expect("test cache key must be valid")
    }

    fn open_test_cache(
        directory: &Path,
    ) -> PersistentCompilationCache {
        PersistentCompilationCache::open(
            directory,
            PersistenceConfig::default(),
        )
        .expect("test cache must open")
    }

    #[test]
    fn stores_and_loads_an_entry() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("input-a");

        assert_eq!(
            cache.store(&key, b"verified-artifact").unwrap(),
            PersistOutcome::Stored
        );

        assert_eq!(
            cache.load(&key).unwrap().unwrap(),
            b"verified-artifact"
        );

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn entry_survives_cache_reopening() {
        let directory = test_directory();
        let key = test_key("persistent-input");

        {
            let cache = open_test_cache(&directory);
            cache.store(&key, b"persistent-artifact").unwrap();
        }

        {
            let cache = open_test_cache(&directory);

            assert_eq!(
                cache.load(&key).unwrap().unwrap(),
                b"persistent-artifact"
            );
        }

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn missing_entry_is_a_cache_miss() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);

        assert!(cache.load(&test_key("missing")).unwrap().is_none());

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn identical_repeated_insert_is_idempotent() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("same-input");

        assert_eq!(
            cache.store(&key, b"same-artifact").unwrap(),
            PersistOutcome::Stored
        );

        assert_eq!(
            cache.store(&key, b"same-artifact").unwrap(),
            PersistOutcome::AlreadyPresent
        );

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn conflicting_payload_for_same_key_is_rejected() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("same-input");

        cache.store(&key, b"first-artifact").unwrap();

        assert!(matches!(
            cache.store(&key, b"different-artifact"),
            Err(PersistenceError::ConflictingContent(_))
        ));

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn different_keys_have_independent_entries() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);

        let first = test_key("first-input");
        let second = test_key("second-input");

        cache.store(&first, b"first").unwrap();
        cache.store(&second, b"second").unwrap();

        assert_eq!(cache.load(&first).unwrap().unwrap(), b"first");
        assert_eq!(cache.load(&second).unwrap().unwrap(), b"second");

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn configured_entry_budget_is_enforced() {
        let directory = test_directory();

        let cache = PersistentCompilationCache::open(
            &directory,
            PersistenceConfig::bounded(4, false),
        )
        .unwrap();

        assert!(matches!(
            cache.store(&test_key("large"), b"12345"),
            Err(PersistenceError::EntryTooLarge { .. })
        ));

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn removing_an_entry_is_idempotent() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("remove");

        cache.store(&key, b"artifact").unwrap();

        assert!(cache.remove(&key).unwrap());
        assert!(!cache.remove(&key).unwrap());
        assert!(cache.load(&key).unwrap().is_none());

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn clear_removes_entries_but_preserves_unrelated_files() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);

        let first = test_key("first");
        let second = test_key("second");

        cache.store(&first, b"one").unwrap();
        cache.store(&second, b"two").unwrap();

        let unrelated = directory.join("keep.txt");
        fs::write(&unrelated, b"keep").unwrap();

        assert_eq!(cache.clear().unwrap(), 2);

        assert!(cache.load(&first).unwrap().is_none());
        assert!(cache.load(&second).unwrap().is_none());
        assert_eq!(fs::read(&unrelated).unwrap(), b"keep");

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn payload_corruption_is_rejected() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("corruption");

        cache.store(&key, b"original").unwrap();

        let path = cache.entry_path(&key);
        let mut bytes = fs::read(&path).unwrap();

        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;

        fs::write(&path, bytes).unwrap();

        assert!(matches!(
            cache.load(&key),
            Err(PersistenceError::IntegrityFailure(_))
        ));

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn truncated_entry_is_rejected() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("truncated");

        cache.store(&key, b"artifact").unwrap();

        let path = cache.entry_path(&key);
        let bytes = fs::read(&path).unwrap();

        fs::write(&path, &bytes[..HEADER_LENGTH - 1]).unwrap();

        assert!(matches!(
            cache.load(&key),
            Err(PersistenceError::InvalidFormat(_))
        ));

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn clones_share_the_same_persistent_directory() {
        let directory = test_directory();
        let first = open_test_cache(&directory);
        let second = first.clone();
        let key = test_key("shared");

        first.store(&key, b"shared-artifact").unwrap();

        assert_eq!(
            second.load(&key).unwrap().unwrap(),
            b"shared-artifact"
        );

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn empty_payloads_are_supported() {
        let directory = test_directory();
        let cache = open_test_cache(&directory);
        let key = test_key("empty");

        cache.store(&key, b"").unwrap();

        assert_eq!(cache.load(&key).unwrap().unwrap(), b"");

        let _ = fs::remove_dir_all(directory);
    }
}
