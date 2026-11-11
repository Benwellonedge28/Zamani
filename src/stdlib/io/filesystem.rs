
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Filesystem
//!
//! Path: `src/stdlib/io/filesystem.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a portable, provider-oriented filesystem API for Zamani.
//!
//! The module supports:
//!
//! - Opening files.
//! - Creating files.
//! - Reading and writing files.
//! - Creating directories.
//! - Listing directories incrementally.
//! - Reading file and directory metadata.
//! - Reading symbolic links.
//! - Canonicalizing paths.
//! - Copying and renaming filesystem objects.
//! - Creating hard links.
//! - Removing files and directories.
//! - Reading and changing read-only permissions.
//! - Streaming atomic file replacement where supported.
//! - Cooperative cancellation.
//! - Structured error propagation.
//! - Injectable filesystem providers.
//!
//! ## Architectural ownership
//!
//! This module owns filesystem API contracts and the host filesystem
//! provider.
//!
//! It does not own:
//!
//! - Portable language-level path identity.
//! - Source-language semantics.
//! - Filesystem effect analysis.
//! - Runtime scheduling.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Device memory.
//! - Network transports.
//! - Distributed storage placement.
//! - Database transactions.
//! - Tensor storage.
//! - Serialization formats.
//! - Compiler intermediate representations.
//! - Security policy definition.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ## Integration
//!
//! Structured errors:
//! `crate::stdlib::error`
//!
//! Operation context:
//! `crate::stdlib::context`
//!
//! Byte streams:
//! `crate::stdlib::io::streams`
//!
//! Legacy filesystem API:
//! `crate::stdlib::fs`
//!
//! Storage contracts:
//! `crate::stdlib::data::storage`
//!
//! Runtime and authorization:
//! `crate::runtime` and the applicable security/HAL providers.
//!
//! ## Portability
//!
//! `std::path::Path` represents a path interpreted by the selected
//! filesystem provider. It is not a universal serialized path format.
//!
//! Portable path identity and cross-platform path conversion belong
//! in the separate `io::paths` API.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - File size.
//! - Directory entry count.
//! - Path length.
//! - Number of open files.
//! - Number of filesystem operations.
//! - Number of filesystem providers.
//! - Number of directories.
//!
//! Directory listing is incremental.
//!
//! Atomic file writing accepts a reader and does not require the
//! complete file to be materialized in memory.
//!
//! Actual limits are determined by available resources, the host
//! operating system, the filesystem, the selected provider and
//! explicitly configured operational policies.
//!
//! No finite implementation promises literal infinite storage.
//!
//! ## Atomicity and durability
//!
//! `atomic_write_from` writes a temporary file in the destination
//! directory, flushes it, synchronizes its file contents, and then
//! renames it into place.
//!
//! The provider reports failures rather than silently falling back
//! to a non-atomic replacement.
//!
//! Atomic visibility and durable persistence are different guarantees.
//! A successful return does not universally guarantee that directory
//! metadata has reached persistent physical storage.
//!
//! ## Security
//!
//! This module does not pretend that path validation alone provides
//! filesystem sandboxing.
//!
//! Authorization, sandboxing, mount restrictions, symlink policies,
//! and capability enforcement must be supplied by the runtime or
//! the selected filesystem provider.
//!
//! ## Cancellation
//!
//! Cancellation is cooperative.
//!
//! Operations check the supplied context before beginning.
//!
//! A cancellation request cannot forcibly interrupt an operating
//! system call that is already blocking.
//!
//! Directory iteration checks cancellation between entries.
//!
//! ## Safety
//!
//! Safe Rust only.

use core::fmt;

use std::ffi::OsString;
use std::fs::{
    self,
    File,
    Metadata,
    OpenOptions as HostOpenOptions,
    Permissions,
};
use std::io::{
    self,
    Read,
    Seek,
    Write,
};
use std::path::{
    Path,
    PathBuf,
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::SystemTime;

use crate::stdlib::context::OperationContext;
use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// IDENTIFICATION
// ============================================================================

/// Stable standard-library component identifier.
pub const COMPONENT: &str = "stdlib.io.filesystem";

/// File-open operation identifier.
pub const OPEN_OPERATION: &str = "filesystem.open";

/// File-create operation identifier.
pub const CREATE_OPERATION: &str = "filesystem.create";

/// File-read operation identifier.
pub const READ_OPERATION: &str = "filesystem.read";

/// File-write operation identifier.
pub const WRITE_OPERATION: &str = "filesystem.write";

/// Directory-list operation identifier.
pub const READ_DIRECTORY_OPERATION: &str = "filesystem.read_directory";

/// Metadata operation identifier.
pub const METADATA_OPERATION: &str = "filesystem.metadata";

/// Atomic-write operation identifier.
pub const ATOMIC_WRITE_OPERATION: &str = "filesystem.atomic_write";

// ============================================================================
// ERROR CONVERSION
// ============================================================================

/// Converts an operating-system filesystem error into a structured
/// Zamani error.
///
/// The original operating-system error is retained as the causal source.
fn convert_io_error(
    error: io::Error,
    operation: &'static str,
) -> Error {
    let kind = match error.kind() {
        io::ErrorKind::NotFound => ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
        io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
        io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
        io::ErrorKind::InvalidData => ErrorKind::InvalidData,
        io::ErrorKind::TimedOut => ErrorKind::TimedOut,
        io::ErrorKind::Interrupted => ErrorKind::Cancelled,
        io::ErrorKind::WouldBlock => ErrorKind::ResourceUnavailable,
        io::ErrorKind::OutOfMemory => ErrorKind::ResourceUnavailable,
        _ => ErrorKind::Io,
    };

    Error::new(kind, error.to_string())
        .in_component(COMPONENT)
        .during_operation(operation)
        .with_source(error)
}

/// Constructs a structured filesystem error.
fn filesystem_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
        .during_operation(operation)
}

/// Checks whether the operation has been cancelled.
fn check_context(
    context: &OperationContext,
) -> Result<()> {
    context.check_cancelled()
}

// ============================================================================
// FILE HANDLE CONTRACT
// ============================================================================

/// A provider-owned file handle.
///
/// The handle supports reading, writing and seeking.
///
/// The standard library does not require every filesystem provider
/// to use an operating-system file descriptor.
///
/// Providers may implement virtual, remote, embedded or other
/// compatible file handles.
pub trait FileHandle: Read + Write + Seek + Send {}

impl<T> FileHandle for T
where
    T: Read + Write + Seek + Send,
{
}

// ============================================================================
// FILE OPEN OPTIONS
// ============================================================================

/// Options controlling how a file is opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileOpenOptions {
    /// Permit reading.
    pub read: bool,

    /// Permit writing.
    pub write: bool,

    /// Append writes to the end of the file.
    pub append: bool,

    /// Create the file when it does not exist.
    pub create: bool,

    /// Create only when the file does not already exist.
    pub create_new: bool,

    /// Truncate an existing file when opening for writing.
    pub truncate: bool,
}

impl Default for FileOpenOptions {
    fn default() -> Self {
        Self {
            read: true,
            write: false,
            append: false,
            create: false,
            create_new: false,
            truncate: false,
        }
    }
}

impl FileOpenOptions {
    /// Creates options for reading an existing file.
    #[must_use]
    pub const fn read() -> Self {
        Self {
            read: true,
            write: false,
            append: false,
            create: false,
            create_new: false,
            truncate: false,
        }
    }

    /// Creates options for writing an existing file.
    #[must_use]
    pub const fn write() -> Self {
        Self {
            read: false,
            write: true,
            append: false,
            create: false,
            create_new: false,
            truncate: false,
        }
    }

    /// Creates options for appending to a file.
    #[must_use]
    pub const fn append() -> Self {
        Self {
            read: false,
            write: true,
            append: true,
            create: true,
            create_new: false,
            truncate: false,
        }
    }

    /// Enables reading.
    #[must_use]
    pub const fn with_read(mut self, enabled: bool) -> Self {
        self.read = enabled;
        self
    }

    /// Enables writing.
    #[must_use]
    pub const fn with_write(mut self, enabled: bool) -> Self {
        self.write = enabled;
        self
    }

    /// Enables append mode.
    #[must_use]
    pub const fn with_append(mut self, enabled: bool) -> Self {
        self.append = enabled;
        self
    }

    /// Enables creation when the file does not exist.
    #[must_use]
    pub const fn with_create(mut self, enabled: bool) -> Self {
        self.create = enabled;
        self
    }

    /// Enables exclusive creation.
    #[must_use]
    pub const fn with_create_new(mut self, enabled: bool) -> Self {
        self.create_new = enabled;
        self
    }

    /// Enables truncation.
    #[must_use]
    pub const fn with_truncate(mut self, enabled: bool) -> Self {
        self.truncate = enabled;
        self
    }

    /// Validates the requested access mode.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error if no access mode is enabled.
    pub fn validate(self) -> Result<()> {
        if !self.read && !self.write && !self.append {
            return Err(filesystem_error(
                ErrorKind::InvalidInput,
                "at least one file access mode must be enabled",
                OPEN_OPERATION,
            ));
        }

        if self.truncate && !self.write && !self.append {
            return Err(filesystem_error(
                ErrorKind::InvalidInput,
                "file truncation requires write access",
                OPEN_OPERATION,
            ));
        }

        if self.create_new && self.create {
            return Err(filesystem_error(
                ErrorKind::InvalidInput,
                "exclusive creation and ordinary creation cannot both be requested",
                OPEN_OPERATION,
            ));
        }

        Ok(())
    }
}

// ============================================================================
// FILESYSTEM METADATA
// ============================================================================

/// Portable metadata exposed by the filesystem API.
///
/// Metadata fields describe the provider's observed filesystem object.
/// They do not imply that every filesystem supports every property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemMetadata {
    /// Object length in bytes.
    ///
    /// For directories, the meaning is provider and platform dependent.
    pub length: u64,

    /// Whether the object is a regular file.
    pub is_file: bool,

    /// Whether the object is a directory.
    pub is_directory: bool,

    /// Whether the object is a symbolic link.
    pub is_symlink: bool,

    /// Whether the object is read-only according to the provider.
    pub read_only: bool,

    /// Last modification time, when available.
    pub modified: Option<SystemTime>,

    /// Last access time, when available.
    pub accessed: Option<SystemTime>,

    /// Creation time, when available.
    pub created: Option<SystemTime>,
}

impl FilesystemMetadata {
    /// Constructs metadata from host filesystem metadata.
    fn from_host(metadata: &Metadata, follow_symlinks: bool) -> Self {
        let file_type = metadata.file_type();

        Self {
            length: metadata.len(),
            is_file: metadata.is_file(),
            is_directory: metadata.is_dir(),
            is_symlink: !follow_symlinks && file_type.is_symlink(),
            read_only: metadata.permissions().readonly(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
            created: metadata.created().ok(),
        }
    }
}

// ============================================================================
// DIRECTORY ENTRIES
// ============================================================================

/// One observed entry in a directory listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Provider-interpreted path of this entry.
    pub path: PathBuf,

    /// Metadata for the entry.
    pub metadata: FilesystemMetadata,
}

// ============================================================================
// FILESYSTEM PROVIDER CONTRACT
// ============================================================================

/// Filesystem provider contract.
///
/// Providers may implement:
///
/// - Host filesystems.
/// - Sandboxed filesystems.
/// - Virtual filesystems.
/// - Embedded filesystems.
/// - Remote filesystem gateways.
/// - In-memory filesystems.
/// - Future filesystem environments.
///
/// Provider methods must return actual operation results.
///
/// A provider must not report success for an operation that it
/// did not perform.
///
/// Filesystem authorization and sandbox enforcement remain
/// provider/runtime responsibilities.
pub trait FilesystemProvider: Send + Sync {
    /// Opens a file using the supplied options.
    fn open(
        &self,
        path: &Path,
        options: FileOpenOptions,
    ) -> Result<Box<dyn FileHandle>>;

    /// Creates a directory.
    fn create_dir(&self, path: &Path) -> Result<()>;

    /// Creates a directory and any missing parent directories.
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Returns metadata while following symbolic links.
    fn metadata(&self, path: &Path) -> Result<FilesystemMetadata>;

    /// Returns metadata without following the final symbolic link.
    fn symlink_metadata(&self, path: &Path) -> Result<FilesystemMetadata>;

    /// Lists a directory incrementally.
    ///
    /// The returned iterator must not eagerly materialize the
    /// entire directory listing.
    fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = Result<DirectoryEntry>> + Send>>;

    /// Copies a file and returns the number of bytes copied.
    fn copy(
        &self,
        from: &Path,
        to: &Path,
    ) -> Result<u64>;

    /// Renames a filesystem object.
    fn rename(
        &self,
        from: &Path,
        to: &Path,
    ) -> Result<()>;

    /// Creates a hard link.
    fn hard_link(
        &self,
        existing: &Path,
        link: &Path,
    ) -> Result<()>;

    /// Reads a symbolic link without following it.
    fn read_link(&self, path: &Path) -> Result<PathBuf>;

    /// Resolves a path to its canonical host/provider representation.
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;

    /// Removes a file.
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// Removes an empty directory.
    fn remove_dir(&self, path: &Path) -> Result<()>;

    /// Removes a directory tree.
    ///
    /// This is destructive and must not be used as an implicit
    /// fallback for `remove_dir`.
    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    /// Changes the read-only permission flag.
    ///
    /// This is a portable subset of filesystem permissions.
    /// More detailed access-control policies are provider-specific.
    fn set_read_only(
        &self,
        path: &Path,
        read_only: bool,
    ) -> Result<()>;

    /// Streams content into a temporary file and atomically publishes it.
    ///
    /// Implementations must return an error if they cannot provide
    /// their documented atomic replacement guarantee.
    ///
    /// The input reader is consumed incrementally.
    fn atomic_write_from(
        &self,
        path: &Path,
        input: &mut dyn Read,
    ) -> Result<FilesystemMetadata>;
}

// ============================================================================
// HOST FILESYSTEM PROVIDER
// ============================================================================

/// Host operating-system filesystem provider.
///
/// This provider uses Rust's real filesystem APIs.
///
/// It does not fabricate file contents, metadata, permissions,
/// directory entries or successful writes.
#[derive(Debug, Default, Clone, Copy)]
pub struct HostFilesystemProvider;

impl HostFilesystemProvider {
    /// Creates a host filesystem provider.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts standard-library open options into host open options.
    fn host_open_options(
        options: FileOpenOptions,
    ) -> Result<HostOpenOptions> {
        options.validate()?;

        let mut host = HostOpenOptions::new();

        host.read(options.read)
            .write(options.write)
            .append(options.append)
            .create(options.create)
            .create_new(options.create_new)
            .truncate(options.truncate);

        Ok(host)
    }
}

impl FilesystemProvider for HostFilesystemProvider {
    fn open(
        &self,
        path: &Path,
        options: FileOpenOptions,
    ) -> Result<Box<dyn FileHandle>> {
        let host_options = Self::host_open_options(options)?;

        let file = host_options
            .open(path)
            .map_err(|error| convert_io_error(error, OPEN_OPERATION))?;

        Ok(Box::new(file))
    }

    fn create_dir(&self, path: &Path) -> Result<()> {
        fs::create_dir(path)
            .map_err(|error| convert_io_error(error, CREATE_OPERATION))
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .map_err(|error| convert_io_error(error, CREATE_OPERATION))
    }

    fn metadata(&self, path: &Path) -> Result<FilesystemMetadata> {
        let metadata = fs::metadata(path)
            .map_err(|error| convert_io_error(error, METADATA_OPERATION))?;

        Ok(FilesystemMetadata::from_host(&metadata, true))
    }

    fn symlink_metadata(&self, path: &Path) -> Result<FilesystemMetadata> {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| convert_io_error(error, METADATA_OPERATION))?;

        Ok(FilesystemMetadata::from_host(&metadata, false))
    }

    fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = Result<DirectoryEntry>> + Send>> {
        let entries = fs::read_dir(path)
            .map_err(|error| convert_io_error(error, READ_DIRECTORY_OPERATION))?;

        let mapped = entries.map(|entry| {
            let entry = entry
                .map_err(|error| convert_io_error(error, READ_DIRECTORY_OPERATION))?;

            let path = entry.path();

            let metadata = entry
                .metadata()
                .map_err(|error| convert_io_error(error, METADATA_OPERATION))?;

            Ok(DirectoryEntry {
                path,
                metadata: FilesystemMetadata::from_host(&metadata, true),
            })
        });

        Ok(Box::new(mapped))
    }

    fn copy(
        &self,
        from: &Path,
        to: &Path,
    ) -> Result<u64> {
        fs::copy(from, to)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn rename(
        &self,
        from: &Path,
        to: &Path,
    ) -> Result<()> {
        fs::rename(from, to)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn hard_link(
        &self,
        existing: &Path,
        link: &Path,
    ) -> Result<()> {
        fs::hard_link(existing, link)
            .map_err(|error| convert_io_error(error, CREATE_OPERATION))
    }

    fn read_link(&self, path: &Path) -> Result<PathBuf> {
        fs::read_link(path)
            .map_err(|error| convert_io_error(error, METADATA_OPERATION))
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        fs::canonicalize(path)
            .map_err(|error| convert_io_error(error, METADATA_OPERATION))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn remove_dir(&self, path: &Path) -> Result<()> {
        fs::remove_dir(path)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn set_read_only(
        &self,
        path: &Path,
        read_only: bool,
    ) -> Result<()> {
        let metadata = fs::metadata(path)
            .map_err(|error| convert_io_error(error, METADATA_OPERATION))?;

        let mut permissions = metadata.permissions();

        permissions.set_readonly(read_only);

        fs::set_permissions(path, permissions)
            .map_err(|error| convert_io_error(error, WRITE_OPERATION))
    }

    fn atomic_write_from(
        &self,
        path: &Path,
        input: &mut dyn Read,
    ) -> Result<FilesystemMetadata> {
        atomic_write_host(path, input)
    }
}

// ============================================================================
// ATOMIC HOST FILE WRITING
// ============================================================================

/// Process-local sequence used to generate temporary file names.
///
/// This is a uniqueness aid, not a resource ceiling.
static TEMPORARY_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Removes an unpublished temporary file when an operation fails.
struct TemporaryFileGuard {
    path: PathBuf,
    published: bool,
}

impl TemporaryFileGuard {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            published: false,
        }
    }

    fn mark_published(&mut self) {
        self.published = true;
    }
}

impl Drop for TemporaryFileGuard {
    fn drop(&mut self) {
        if !self.published {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Constructs a temporary file name in the destination directory.
///
/// The temporary file is created with exclusive creation, so a
/// collision never causes an existing file to be overwritten.
fn temporary_path(
    destination: &Path,
    sequence: u64,
) -> Result<PathBuf> {
    let file_name = destination.file_name().ok_or_else(|| {
        filesystem_error(
            ErrorKind::InvalidInput,
            "atomic write destination must identify a file",
            ATOMIC_WRITE_OPERATION,
        )
    })?;

    let mut temporary_name = OsString::from(".");
    temporary_name.push(file_name);
    temporary_name.push(format!(
        ".zamani-tmp-{}-{}",
        std::process::id(),
        sequence,
    ));

    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    Ok(parent.join(temporary_name))
}

/// Atomically publishes streamed content on the host filesystem.
///
/// The temporary file is created in the destination directory so
/// that the final rename remains on the same filesystem.
///
/// The method does not silently fall back to a non-atomic copy.
fn atomic_write_host(
    destination: &Path,
    input: &mut dyn Read,
) -> Result<FilesystemMetadata> {
    let mut sequence = TEMPORARY_FILE_SEQUENCE.load(Ordering::Relaxed);

    let (temporary, mut file) = loop {
        let candidate = temporary_path(destination, sequence)?;

        sequence = TEMPORARY_FILE_SEQUENCE
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_add(1);

        match HostOpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                break (candidate, file);
            }

            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                // A collision is not a reason to overwrite an existing file.
                // Generate another candidate and retry.
                continue;
            }

            Err(error) => {
                return Err(convert_io_error(error, ATOMIC_WRITE_OPERATION));
            }
        }
    };

    let mut guard = TemporaryFileGuard::new(temporary.clone());

    io::copy(input, &mut file)
        .map_err(|error| convert_io_error(error, ATOMIC_WRITE_OPERATION))?;

    file.flush()
        .map_err(|error| convert_io_error(error, ATOMIC_WRITE_OPERATION))?;

    file.sync_all()
        .map_err(|error| convert_io_error(error, ATOMIC_WRITE_OPERATION))?;

    drop(file);

    fs::rename(&temporary, destination)
        .map_err(|error| convert_io_error(error, ATOMIC_WRITE_OPERATION))?;

    guard.mark_published();

    let metadata = fs::metadata(destination)
        .map_err(|error| convert_io_error(error, METADATA_OPERATION))?;

    Ok(FilesystemMetadata::from_host(&metadata, true))
}

// ============================================================================
// DIRECTORY LISTING
// ============================================================================

/// A cancellation-aware incremental directory listing.
///
/// Entries are retrieved as the iterator is advanced. The complete
/// directory is not collected into a single in-memory vector.
pub struct DirectoryListing {
    inner: Box<dyn Iterator<Item = Result<DirectoryEntry>> + Send>,
    context: OperationContext,
    finished: bool,
}

impl DirectoryListing {
    /// Creates a directory listing.
    fn new(
        inner: Box<dyn Iterator<Item = Result<DirectoryEntry>> + Send>,
        context: OperationContext,
    ) -> Self {
        Self {
            inner,
            context,
            finished: false,
        }
    }
}

impl Iterator for DirectoryListing {
    type Item = Result<DirectoryEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Err(error) = self.context.check_cancelled() {
            self.finished = true;
            return Some(Err(error));
        }

        match self.inner.next() {
            Some(entry) => Some(entry),
            None => {
                self.finished = true;
                None
            }
        }
    }
}

// ============================================================================
// FILESYSTEM SERVICE
// ============================================================================

/// Portable filesystem service.
///
/// The service combines a provider with an operation context.
///
/// Cloning a service shares the same provider and context.
///
/// The service does not perform hidden provider discovery.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use zamani::stdlib::io::filesystem::Filesystem;
///
/// let filesystem = Filesystem::host();
///
/// let result = filesystem.metadata(Path::new("Cargo.toml"));
///
/// // The result reflects the actual host filesystem.
/// assert!(result.is_ok());
/// ```
#[derive(Clone)]
pub struct Filesystem {
    provider: Arc<dyn FilesystemProvider>,
    context: OperationContext,
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Filesystem")
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

impl Filesystem {
    /// Creates a filesystem service backed by the host filesystem.
    #[must_use]
    pub fn host() -> Self {
        Self::with_provider_and_context(
            HostFilesystemProvider::new(),
            OperationContext::builder()
                .with_component(COMPONENT)
                .build(),
        )
    }

    /// Creates a filesystem service with a custom provider.
    #[must_use]
    pub fn with_provider<P>(provider: P) -> Self
    where
        P: FilesystemProvider + 'static,
    {
        Self::with_provider_and_context(
            provider,
            OperationContext::builder()
                .with_component(COMPONENT)
                .build(),
        )
    }

    /// Creates a filesystem service with a provider and context.
    #[must_use]
    pub fn with_provider_and_context<P>(
        provider: P,
        context: OperationContext,
    ) -> Self
    where
        P: FilesystemProvider + 'static,
    {
        Self {
            provider: Arc::new(provider),
            context,
        }
    }

    /// Creates a filesystem service with a shared provider.
    #[must_use]
    pub fn with_shared_provider(
        provider: Arc<dyn FilesystemProvider>,
        context: OperationContext,
    ) -> Self {
        Self {
            provider,
            context,
        }
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Checks cancellation before a filesystem operation.
    fn check(&self) -> Result<()> {
        check_context(&self.context)
    }

    /// Opens a file using the supplied options.
    pub fn open(
        &self,
        path: impl AsRef<Path>,
        options: FileOpenOptions,
    ) -> Result<Box<dyn FileHandle>> {
        self.check()?;
        options.validate()?;
        self.provider.open(path.as_ref(), options)
    }

    /// Opens a file for reading.
    pub fn open_read(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Box<dyn FileHandle>> {
        self.open(path, FileOpenOptions::read())
    }

    /// Creates or truncates a file for writing.
    pub fn create_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Box<dyn FileHandle>> {
        self.open(
            path,
            FileOpenOptions::write()
                .with_create(true)
                .with_truncate(true),
        )
    }

    /// Creates a new file without replacing an existing file.
    pub fn create_new_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Box<dyn FileHandle>> {
        self.open(
            path,
            FileOpenOptions::write().with_create_new(true),
        )
    }

    /// Creates a directory.
    pub fn create_dir(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;
        self.provider.create_dir(path.as_ref())
    }

    /// Creates a directory and missing parents.
    pub fn create_dir_all(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;
        self.provider.create_dir_all(path.as_ref())
    }

    /// Returns metadata while following symbolic links.
    pub fn metadata(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<FilesystemMetadata> {
        self.check()?;
        self.provider.metadata(path.as_ref())
    }

    /// Returns metadata without following the final symbolic link.
    pub fn symlink_metadata(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<FilesystemMetadata> {
        self.check()?;
        self.provider.symlink_metadata(path.as_ref())
    }

    /// Opens an incremental directory listing.
    pub fn read_dir(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<DirectoryListing> {
        self.check()?;

        let inner = self.provider.read_dir(path.as_ref())?;

        Ok(DirectoryListing::new(
            inner,
            self.context.clone(),
        ))
    }

    /// Copies a file and returns the number of bytes copied.
    pub fn copy(
        &self,
        from: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> Result<u64> {
        self.check()?;

        self.provider.copy(
            from.as_ref(),
            to.as_ref(),
        )
    }

    /// Renames a filesystem object.
    pub fn rename(
        &self,
        from: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;

        self.provider.rename(
            from.as_ref(),
            to.as_ref(),
        )
    }

    /// Creates a hard link.
    pub fn hard_link(
        &self,
        existing: impl AsRef<Path>,
        link: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;

        self.provider.hard_link(
            existing.as_ref(),
            link.as_ref(),
        )
    }

    /// Reads a symbolic link.
    pub fn read_link(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<PathBuf> {
        self.check()?;
        self.provider.read_link(path.as_ref())
    }

    /// Canonicalizes a path.
    pub fn canonicalize(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<PathBuf> {
        self.check()?;
        self.provider.canonicalize(path.as_ref())
    }

    /// Removes a file.
    pub fn remove_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;
        self.provider.remove_file(path.as_ref())
    }

    /// Removes an empty directory.
    pub fn remove_dir(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;
        self.provider.remove_dir(path.as_ref())
    }

    /// Removes a directory tree.
    ///
    /// This operation is explicitly destructive.
    pub fn remove_dir_all(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        self.check()?;
        self.provider.remove_dir_all(path.as_ref())
    }

    /// Changes the read-only flag.
    pub fn set_read_only(
        &self,
        path: impl AsRef<Path>,
        read_only: bool,
    ) -> Result<()> {
        self.check()?;

        self.provider.set_read_only(
            path.as_ref(),
            read_only,
        )
    }

    /// Atomically writes content from a reader.
    ///
    /// Content is streamed. It is not first collected into a
    /// potentially large in-memory buffer.
    pub fn atomic_write_from(
        &self,
        path: impl AsRef<Path>,
        input: &mut dyn Read,
    ) -> Result<FilesystemMetadata> {
        self.check()?;

        self.provider.atomic_write_from(
            path.as_ref(),
            input,
        )
    }

    /// Atomically writes a byte slice.
    ///
    /// This convenience method uses the same provider contract
    /// as streaming atomic writes.
    pub fn atomic_write(
        &self,
        path: impl AsRef<Path>,
        contents: &[u8],
    ) -> Result<FilesystemMetadata> {
        let mut input = io::Cursor::new(contents);

        self.atomic_write_from(path, &mut input)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Read, Write};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn temporary_directory() -> PathBuf {
        loop {
            let sequence = TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);

            let path = std::env::temp_dir().join(format!(
                "zamani-filesystem-test-{}-{}",
                std::process::id(),
                sequence,
            ));

            match fs::create_dir(&path) {
                Ok(()) => return path,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(error) => panic!("cannot create test directory: {error}"),
            }
        }
    }

    #[test]
    fn host_provider_creates_and_reads_a_file() {
        let directory = temporary_directory();
        let path = directory.join("sample.txt");

        let filesystem = Filesystem::host();

        {
            let mut file = filesystem.create_file(&path).unwrap();
            file.write_all(b"Zamani filesystem").unwrap();
            file.flush().unwrap();
        }

        let mut file = filesystem.open_read(&path).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        assert_eq!(content, "Zamani filesystem");

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn create_new_does_not_replace_existing_file() {
        let directory = temporary_directory();
        let path = directory.join("existing.txt");

        fs::write(&path, b"original").unwrap();

        let filesystem = Filesystem::host();

        let result = filesystem.create_new_file(&path);

        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(), b"original");

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn metadata_reports_real_file_information() {
        let directory = temporary_directory();
        let path = directory.join("metadata.txt");

        fs::write(&path, b"12345").unwrap();

        let filesystem = Filesystem::host();
        let metadata = filesystem.metadata(&path).unwrap();

        assert!(metadata.is_file);
        assert_eq!(metadata.length, 5);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn directory_listing_is_incremental_and_returns_entries() {
        let directory = temporary_directory();

        fs::write(directory.join("one.txt"), b"1").unwrap();
        fs::write(directory.join("two.txt"), b"2").unwrap();

        let filesystem = Filesystem::host();

        let entries = filesystem
            .read_dir(&directory)
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();

        assert_eq!(entries.len(), 2);

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn atomic_write_publishes_complete_content() {
        let directory = temporary_directory();
        let path = directory.join("atomic.txt");

        let filesystem = Filesystem::host();

        let metadata = filesystem
            .atomic_write(&path, b"complete content")
            .unwrap();

        assert!(metadata.is_file);
        assert_eq!(
            fs::read(&path).unwrap(),
            b"complete content"
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn atomic_write_replaces_existing_content_when_supported() {
        let directory = temporary_directory();
        let path = directory.join("replace.txt");

        fs::write(&path, b"old").unwrap();

        let filesystem = Filesystem::host();

        let result = filesystem.atomic_write(&path, b"new");

        if result.is_ok() {
            assert_eq!(fs::read(&path).unwrap(), b"new");
        } else {
            // A provider/platform that cannot replace an existing
            // destination atomically must report failure.
            assert_eq!(fs::read(&path).unwrap(), b"old");
        }

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn invalid_open_options_are_rejected() {
        let options = FileOpenOptions {
            read: false,
            write: false,
            append: false,
            create: false,
            create_new: false,
            truncate: false,
        };

        assert!(options.validate().is_err());
    }

    #[test]
    fn create_directory_and_remove_directory_work() {
        let directory = temporary_directory();
        let nested = directory.join("nested");

        let filesystem = Filesystem::host();

        filesystem.create_dir(&nested).unwrap();

        let metadata = filesystem.metadata(&nested).unwrap();

        assert!(metadata.is_directory);

        filesystem.remove_dir(&nested).unwrap();

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn copy_and_rename_report_real_results() {
        let directory = temporary_directory();

        let source = directory.join("source.txt");
        let copied = directory.join("copied.txt");
        let renamed = directory.join("renamed.txt");

        fs::write(&source, b"copy me").unwrap();

        let filesystem = Filesystem::host();

        assert_eq!(
            filesystem.copy(&source, &copied).unwrap(),
            7
        );

        filesystem.rename(&copied, &renamed).unwrap();

        assert_eq!(
            fs::read(&renamed).unwrap(),
            b"copy me"
        );

        fs::remove_dir_all(directory).unwrap();
    }
}
