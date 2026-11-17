
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Extraction
//!
//! Path: `src/toolchain/packages/extraction.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Safely extract versioned `.zpkg` archives into a newly created
//! destination directory.
//!
//! Extraction is streaming. Neither the complete archive nor an
//! individual package entry is required to fit in memory.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Extraction options and operational budgets.
//! - Extraction verification callback contracts.
//! - Streaming archive-to-filesystem extraction.
//! - Portable destination path construction.
//! - Filesystem entry-type validation.
//! - Extraction byte and entry accounting.
//! - Private staging-directory management.
//! - Destination publication.
//! - Extraction reports.
//!
//! This module does not own:
//!
//! - Archive framing or serialization.
//! - Package metadata encoding or decoding.
//! - Package identity allocation.
//! - Package dependency resolution.
//! - Package installation.
//! - Registry downloading.
//! - Cryptographic algorithms.
//! - Signature creation or trust policy.
//! - Compiler or backend execution.
//! - Hardware discovery.
//! - Runtime execution.
//!
//! ## Security
//!
//! Archive paths are validated using the canonical PackagePath model
//! and additional host-portability checks.
//!
//! Only regular files and automatically created parent directories
//! are extracted. Archive entries cannot create symbolic links,
//! hard links, devices, sockets or other special filesystem objects.
//!
//! Existing destination directories are never merged or overwritten.
//!
//! Extraction takes place in a newly created sibling staging directory.
//! The destination becomes visible only after the archive has been
//! completely consumed, verified and validated.
//!
//! The staging directory is private to the extraction operation.
//! Callers must not expose it to untrusted concurrent filesystem actors.
//!
//! Standard-library path checks cannot provide the same race-free
//! directory-handle guarantees as platform-specific openat/O_NOFOLLOW
//! implementations. A hostile process with write access to the staging
//! parent or staging directory is outside this module's isolation
//! guarantee. Production deployments must protect that filesystem
//! boundary through operating-system permissions or an appropriate
//! filesystem provider.
//!
//! ## Verification
//!
//! Archive framing is not proof of authenticity or content integrity.
//!
//! An explicit ExtractionVerifier is mandatory.
//!
//! The verifier validates the decoded manifest and each extracted file.
//! It must delegate signature, trust, digest and package-compatibility
//! decisions to their canonical authorities.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Package size.
//! - Individual file size.
//! - Number of package entries.
//! - Number of computing domains.
//! - Number of artifacts.
//! - Hardware resources described by package metadata.
//!
//! Optional operational budgets are configurable.
//!
//! Memory consumption is proportional to the configured I/O buffer,
//! current path, verifier working state and filesystem implementation,
//! not to the total archive or file size.
//!
//! Actual capacity remains subject to available resources and the
//! representational limits of the host filesystem.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Integration
//!
//! Consumes:
//!
//! - `packages::archive::ArchiveReader`
//! - `packages::model::PackagePath`
//! - `toolchain::errors`
//! - `toolchain::result`
//!
//! The package installation subsystem may consume the resulting
//! extraction report and destination after successful verification.
//!
//! Extraction does not install packages or resolve dependencies.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::Result;

use super::archive::{
    ArchiveIoOptions,
    ArchiveReader,
    DEFAULT_IO_BUFFER_SIZE,
};

use super::model::PackagePath;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_EXTRACT: &str = "package.extract";

const INTERNAL_DIRECTORY: &str = ".zamani-internal";

const INTERNAL_MANIFEST: &str = "manifest";

// ============================================================================
// STAGING IDENTIFIER
// ============================================================================

/// Process-local counter used to generate collision-resistant staging names.
///
/// Filesystem creation remains authoritative: every staging directory is
/// created exclusively, and an existing name is never reused.
static STAGING_SEQUENCE: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// EXTRACTION OPTIONS
// ============================================================================

/// Configurable operational budgets for package extraction.
///
/// Every optional budget is a policy limit, not a language, package-format
/// or hardware-capacity limit.
///
/// `None` means that this particular budget is not imposed by the caller.
///
/// The operating system and filesystem still enforce their actual limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractionBudgets {
    max_entries: Option<u128>,
    max_manifest_bytes: Option<u128>,
    max_entry_bytes: Option<u128>,
    max_total_content_bytes: Option<u128>,
}

impl ExtractionBudgets {
    /// Construct an extraction budget configuration.
    #[must_use]
    pub const fn new(
        max_entries: Option<u128>,
        max_manifest_bytes: Option<u128>,
        max_entry_bytes: Option<u128>,
        max_total_content_bytes: Option<u128>,
    ) -> Self {
        Self {
            max_entries,
            max_manifest_bytes,
            max_entry_bytes,
            max_total_content_bytes,
        }
    }

    /// Return the optional entry-count budget.
    #[must_use]
    pub const fn max_entries(self) -> Option<u128> {
        self.max_entries
    }

    /// Return the optional manifest-size budget.
    #[must_use]
    pub const fn max_manifest_bytes(self) -> Option<u128> {
        self.max_manifest_bytes
    }

    /// Return the optional individual-entry-size budget.
    #[must_use]
    pub const fn max_entry_bytes(self) -> Option<u128> {
        self.max_entry_bytes
    }

    /// Return the optional total extracted-content budget.
    #[must_use]
    pub const fn max_total_content_bytes(self) -> Option<u128> {
        self.max_total_content_bytes
    }
}

impl Default for ExtractionBudgets {
    fn default() -> Self {
        Self {
            max_entries: None,
            max_manifest_bytes: None,
            max_entry_bytes: None,
            max_total_content_bytes: None,
        }
    }
}

/// Options controlling one extraction operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractionOptions {
    io_buffer_size: usize,
    budgets: ExtractionBudgets,
}

impl ExtractionOptions {
    /// Construct extraction options.
    ///
    /// A zero-sized I/O buffer is invalid.
    ///
    /// Buffer size controls I/O granularity only. It does not restrict
    /// the size of an archive or an extracted file.
    pub fn new(
        io_buffer_size: usize,
        budgets: ExtractionBudgets,
    ) -> Result<Self> {
        if io_buffer_size == 0 {
            return Err(
                ToolchainError::invalid_input(
                    "extraction I/O buffer size must be greater than zero",
                )
                .with_operation(OPERATION_EXTRACT),
            );
        }

        Ok(Self {
            io_buffer_size,
            budgets,
        })
    }

    /// Return the configured I/O buffer size.
    #[must_use]
    pub const fn io_buffer_size(self) -> usize {
        self.io_buffer_size
    }

    /// Return the extraction budgets.
    #[must_use]
    pub const fn budgets(self) -> ExtractionBudgets {
        self.budgets
    }
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            io_buffer_size: DEFAULT_IO_BUFFER_SIZE,
            budgets: ExtractionBudgets {
                max_entries: None,
                max_manifest_bytes: None,
                max_entry_bytes: None,
                max_total_content_bytes: None,
            },
        }
    }
}

// ============================================================================
// VERIFICATION CONTRACT
// ============================================================================

/// Verification authority required by package extraction.
///
/// Implementations must validate the package manifest and every extracted
/// entry before the extraction is published.
///
/// The manifest callback must validate package identity, metadata schema,
/// declared archive contents and applicable signatures/trust evidence.
///
/// The entry callback must validate the content against the manifest's
/// applicable integrity and compatibility declarations.
///
/// Cryptographic operations must be delegated to the canonical security
/// and package-verification authorities.
///
/// Implementations must not modify the supplied files.
///
/// Paths supplied to this trait refer only to files inside the private
/// staging directory of the current extraction operation.
///
/// # Failure
///
/// Returning an error aborts extraction. The staging directory is removed
/// on a best-effort basis and the destination is not published.
pub trait ExtractionVerifier {
    /// Validate the decoded package manifest.
    ///
    /// The manifest is stored as a regular file in the staging directory.
    ///
    /// Implementations should stream it when possible instead of loading
    /// arbitrarily large manifests into memory.
    fn verify_manifest(
        &mut self,
        manifest_path: &Path,
    ) -> Result<()>;

    /// Verify one completely extracted regular file.
    ///
    /// `package_path` is the canonical package-relative path.
    ///
    /// `staged_file` is the corresponding file inside the private staging
    /// directory.
    ///
    /// Implementations may stream the file to calculate and verify its
    /// declared digest.
    fn verify_entry(
        &mut self,
        package_path: &PackagePath,
        staged_file: &Path,
    ) -> Result<()>;
}

// ============================================================================
// EXTRACTION REPORT
// ============================================================================

/// Immutable summary of a successfully published extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionReport {
    destination: PathBuf,
    archive_entries: u128,
    extracted_entries: u128,
    manifest_bytes: u128,
    extracted_content_bytes: u128,
}

impl ExtractionReport {
    /// Return the published destination directory.
    #[must_use]
    pub fn destination(&self) -> &Path {
        &self.destination
    }

    /// Return the number of entries declared by the archive.
    #[must_use]
    pub const fn archive_entries(&self) -> u128 {
        self.archive_entries
    }

    /// Return the number of successfully extracted entries.
    #[must_use]
    pub const fn extracted_entries(&self) -> u128 {
        self.extracted_entries
    }

    /// Return the number of manifest bytes consumed.
    #[must_use]
    pub const fn manifest_bytes(&self) -> u128 {
        self.manifest_bytes
    }

    /// Return the number of extracted content bytes.
    #[must_use]
    pub const fn extracted_content_bytes(&self) -> u128 {
        self.extracted_content_bytes
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn extraction_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION_EXTRACT)
}

fn extraction_io_error(
    error: io::Error,
    context: impl Into<String>,
) -> ToolchainError {
    ToolchainError::from(error)
        .with_operation(OPERATION_EXTRACT)
        .with_context(context)
}

fn extraction_budget_error(
    resource: &str,
    actual: u128,
    budget: u128,
) -> ToolchainError {
    extraction_error(
        ToolchainErrorKind::ResourceExhausted,
        format!(
            "package extraction {resource} budget exceeded: \
             required {actual}, configured budget {budget}"
        ),
    )
}

fn checked_add(
    left: u128,
    right: u128,
    description: &str,
) -> Result<u128> {
    left.checked_add(right).ok_or_else(|| {
        extraction_error(
            ToolchainErrorKind::ResourceExhausted,
            format!("{description} counter overflowed"),
        )
    })
}

// ============================================================================
// TEMPORARY STAGING DIRECTORY
// ============================================================================

/// Owns a newly created staging directory.
///
/// The directory is removed automatically unless publication succeeds.
struct StagingDirectory {
    path: PathBuf,
    published: bool,
}

impl StagingDirectory {
    /// Create a unique staging directory beside the requested destination.
    fn create(parent: &Path, destination_name: &str) -> Result<Self> {
        loop {
            let sequence = STAGING_SEQUENCE.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let staging_name = format!(
                ".{destination_name}.zamani-extract-{}-{sequence}",
                std::process::id(),
            );

            let candidate = parent.join(staging_name);

            match fs::create_dir(&candidate) {
                Ok(()) => {
                    return Ok(Self {
                        path: candidate,
                        published: false,
                    });
                }

                Err(error)
                    if error.kind() == io::ErrorKind::AlreadyExists =>
                {
                    // A collision is not permission to reuse a directory.
                    // Generate another name and retry.
                }

                Err(error) => {
                    return Err(extraction_io_error(
                        error,
                        "creating package extraction staging directory",
                    ));
                }
            }
        }
    }

    /// Return the staging directory path.
    fn path(&self) -> &Path {
        &self.path
    }

    /// Remove internal extraction-only data and atomically publish.
    fn publish(
        mut self,
        internal_directory: &Path,
        destination: &Path,
    ) -> Result<()> {
        fs::remove_dir_all(internal_directory).map_err(|error| {
            extraction_io_error(
                error,
                "removing internal extraction metadata",
            )
        })?;

        fs::rename(&self.path, destination).map_err(|error| {
            extraction_io_error(
                error,
                "publishing completed package extraction",
            )
        })?;

        self.published = true;

        Ok(())
    }
}

impl Drop for StagingDirectory {
    fn drop(&mut self) {
        if !self.published {
            // Best-effort cleanup. Never replace the original operation
            // error with a cleanup error from Drop.
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

// ============================================================================
// DESTINATION VALIDATION
// ============================================================================

/// Resolve the requested destination into an absolute sibling publication
/// path.
///
/// The parent must already exist and must resolve to a real directory.
///
/// The destination itself must not exist.
fn validate_destination(destination: &Path) -> Result<(PathBuf, String)> {
    if destination.as_os_str().is_empty() {
        return Err(
            ToolchainError::invalid_input(
                "package extraction destination must not be empty",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    let requested = if destination.is_absolute() {
        destination.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| {
                extraction_io_error(
                    error,
                    "resolving extraction destination",
                )
            })?
            .join(destination)
    };

    let file_name = requested.file_name().ok_or_else(|| {
        ToolchainError::invalid_input(
            "extraction destination must identify a directory name",
        )
        .with_operation(OPERATION_EXTRACT)
    })?;

    let destination_name = file_name.to_str().ok_or_else(|| {
        ToolchainError::invalid_input(
            "extraction destination name must be valid Unicode",
        )
        .with_operation(OPERATION_EXTRACT)
    })?;

    if destination_name.is_empty()
        || destination_name == "."
        || destination_name == ".."
    {
        return Err(
            ToolchainError::invalid_input(
                "invalid extraction destination name",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    let parent = requested.parent().ok_or_else(|| {
        ToolchainError::invalid_input(
            "extraction destination has no parent directory",
        )
        .with_operation(OPERATION_EXTRACT)
    })?;

    let canonical_parent = fs::canonicalize(parent).map_err(|error| {
        extraction_io_error(
            error,
            "resolving extraction destination parent",
        )
    })?;

    let parent_metadata = fs::metadata(&canonical_parent).map_err(|error| {
        extraction_io_error(
            error,
            "checking extraction destination parent",
        )
    })?;

    if !parent_metadata.is_dir() {
        return Err(
            ToolchainError::invalid_input(
                "extraction destination parent is not a directory",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    let destination = canonical_parent.join(file_name);

    match fs::symlink_metadata(&destination) {
        Ok(_) => {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "extraction destination already exists",
                )
                .with_operation(OPERATION_EXTRACT),
            );
        }

        Err(error) if error.kind() == io::ErrorKind::NotFound => {}

        Err(error) => {
            return Err(extraction_io_error(
                error,
                "checking extraction destination",
            ));
        }
    }

    Ok((destination, destination_name.to_owned()))
}

// ============================================================================
// PORTABLE PATH VALIDATION
// ============================================================================

/// Reject path components that have special meanings on supported
/// filesystem families.
///
/// This prevents package paths from becoming platform-dependent through
/// Windows drive syntax, device names or trailing-dot/space normalization.
fn validate_portable_segment(segment: &str) -> Result<()> {
    if segment.is_empty()
        || segment == "."
        || segment == ".."
        || segment.contains(':')
        || segment.contains('\\')
        || segment.ends_with('.')
        || segment.ends_with(' ')
    {
        return Err(
            ToolchainError::invalid_input(
                "package path contains a non-portable filesystem component",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    let base = segment
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();

    let reserved = matches!(
        base.as_str(),
        "CON" | "PRN" | "AUX" | "NUL"
    ) || matches!(
        base.as_str(),
        "COM1" | "COM2" | "COM3" | "COM4" | "COM5"
            | "COM6" | "COM7" | "COM8" | "COM9"
            | "LPT1" | "LPT2" | "LPT3" | "LPT4"
            | "LPT5" | "LPT6" | "LPT7" | "LPT8"
            | "LPT9"
    );

    if reserved {
        return Err(
            ToolchainError::invalid_input(
                "package path contains a reserved filesystem device name",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    Ok(())
}

/// Convert a canonical package path into a host-relative path.
///
/// The internal extraction directory is reserved and cannot be supplied
/// by an archive entry.
fn package_relative_path(package_path: &PackagePath) -> Result<PathBuf> {
    let value = package_path.as_str();

    if value == INTERNAL_DIRECTORY
        || value.starts_with(&format!("{INTERNAL_DIRECTORY}/"))
    {
        return Err(
            ToolchainError::invalid_input(
                "package path uses a reserved extraction-internal namespace",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    let mut relative = PathBuf::new();

    for segment in value.split('/') {
        validate_portable_segment(segment)?;

        let mut components = Path::new(segment).components();

        match components.next() {
            Some(Component::Normal(component)) => {
                relative.push(component);
            }

            _ => {
                return Err(
                    ToolchainError::invalid_input(
                        "package path contains a non-normal filesystem component",
                    )
                    .with_operation(OPERATION_EXTRACT),
                );
            }
        }

        if components.next().is_some() {
            return Err(
                ToolchainError::invalid_input(
                    "package path segment expands into multiple host components",
                )
                .with_operation(OPERATION_EXTRACT),
            );
        }
    }

    Ok(relative)
}

// ============================================================================
// SAFE PARENT DIRECTORY CREATION
// ============================================================================

/// Create parent directories one component at a time.
///
/// Existing symbolic links and non-directory objects are rejected.
///
/// The staging directory must remain inaccessible to untrusted concurrent
/// writers while extraction is running.
fn create_parent_directories(
    staging_root: &Path,
    relative_file: &Path,
) -> Result<PathBuf> {
    let parent = relative_file.parent().unwrap_or_else(|| Path::new(""));

    let mut current = staging_root.to_path_buf();

    for component in parent.components() {
        let name = match component {
            Component::Normal(name) => name,

            Component::CurDir => continue,

            _ => {
                return Err(
                    ToolchainError::invalid_input(
                        "unsafe parent component in extracted path",
                    )
                    .with_operation(OPERATION_EXTRACT),
                );
            }
        };

        current.push(name);

        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink()
                    || !metadata.is_dir()
                {
                    return Err(
                        ToolchainError::validation_failed(
                            "extraction parent is not a real directory",
                        )
                        .with_operation(OPERATION_EXTRACT),
                    );
                }
            }

            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fs::create_dir(&current).map_err(|error| {
                    extraction_io_error(
                        error,
                        "creating extracted package parent directory",
                    )
                })?;

                let metadata =
                    fs::symlink_metadata(&current).map_err(|error| {
                        extraction_io_error(
                            error,
                            "checking newly created package directory",
                        )
                    })?;

                if metadata.file_type().is_symlink()
                    || !metadata.is_dir()
                {
                    return Err(
                        ToolchainError::validation_failed(
                            "new extraction parent is not a real directory",
                        )
                        .with_operation(OPERATION_EXTRACT),
                    );
                }
            }

            Err(error) => {
                return Err(extraction_io_error(
                    error,
                    "checking extracted package parent directory",
                ));
            }
        }
    }

    Ok(staging_root.join(relative_file))
}

// ============================================================================
// FILE CREATION
// ============================================================================

/// Create a new regular file without overwriting an existing filesystem
/// object.
fn create_extracted_file(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .read(false)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            extraction_io_error(
                error,
                "creating extracted package file exclusively",
            )
        })?;

    let metadata = file.metadata().map_err(|error| {
        extraction_io_error(
            error,
            "checking extracted package file",
        )
    })?;

    if !metadata.is_file() {
        return Err(
            ToolchainError::validation_failed(
                "extracted package destination is not a regular file",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    Ok(file)
}

// ============================================================================
// MANIFEST EXTRACTION
// ============================================================================

/// Stream the complete archive manifest into the staging directory.
fn extract_manifest<R: Read>(
    archive: &mut ArchiveReader<R>,
    internal_directory: &Path,
    budgets: ExtractionBudgets,
) -> Result<u128> {
    let declared = archive.header().manifest_length();

    if let Some(maximum) = budgets.max_manifest_bytes {
        if declared > maximum {
            return Err(extraction_budget_error(
                "manifest-size",
                declared,
                maximum,
            ));
        }
    }

    let manifest_path = internal_directory.join(INTERNAL_MANIFEST);

    let mut file = create_extracted_file(&manifest_path)?;

    let copied = archive.copy_manifest_to(&mut file)?;

    file.flush().map_err(|error| {
        extraction_io_error(error, "flushing extracted package manifest")
    })?;

    file.sync_all().map_err(|error| {
        extraction_io_error(error, "synchronizing extracted package manifest")
    })?;

    if copied != declared {
        return Err(
            ToolchainError::validation_failed(
                "manifest byte count differs from archive header",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    Ok(copied)
}

// ============================================================================
// ENTRY STREAMING
// ============================================================================

/// Stream one archive entry into its staging file.
///
/// Returns the exact number of content bytes written.
fn extract_entry<R: Read>(
    mut entry: super::archive::ArchiveEntryReader<'_, R>,
    staging_root: &Path,
    options: ExtractionOptions,
    total_content_bytes: u128,
) -> Result<(PackagePath, PathBuf, u128)> {
    let package_path = entry.path().clone();

    let relative = package_relative_path(&package_path)?;

    let staged_file = create_parent_directories(
        staging_root,
        &relative,
    )?;

    let mut file = create_extracted_file(&staged_file)?;

    let mut buffer = Vec::new();

    buffer
        .try_reserve_exact(options.io_buffer_size)
        .map_err(|error| {
            extraction_error(
                ToolchainErrorKind::ResourceExhausted,
                format!("unable to allocate extraction buffer: {error}"),
            )
        })?;

    buffer.resize(options.io_buffer_size, 0);

    let budgets = options.budgets;

    let mut entry_bytes = 0_u128;

    loop {
        let count = entry.read(&mut buffer).map_err(|error| {
            extraction_io_error(
                error,
                format!("reading package entry `{package_path}`"),
            )
        })?;

        if count == 0 {
            break;
        }

        let count = count as u128;

        let next_entry_bytes = checked_add(
            entry_bytes,
            count,
            "individual extracted entry",
        )?;

        if let Some(maximum) = budgets.max_entry_bytes {
            if next_entry_bytes > maximum {
                return Err(extraction_budget_error(
                    "individual-entry-size",
                    next_entry_bytes,
                    maximum,
                ));
            }
        }

        let next_total = checked_add(
            total_content_bytes,
            next_entry_bytes,
            "total extracted content",
        )?;

        if let Some(maximum) = budgets.max_total_content_bytes {
            if next_total > maximum {
                return Err(extraction_budget_error(
                    "total-content-size",
                    next_total,
                    maximum,
                ));
            }
        }

        let write_length = usize::try_from(count).map_err(|_| {
            extraction_error(
                ToolchainErrorKind::ResourceExhausted,
                "extraction write length cannot be represented by this host",
            )
        })?;

        file.write_all(&buffer[..write_length]).map_err(|error| {
            extraction_io_error(
                error,
                format!("writing extracted package entry `{package_path}`"),
            )
        })?;

        entry_bytes = next_entry_bytes;
    }

    // The reader must reach the explicit archive entry terminator.
    let framed_bytes = entry.finish()?;

    if framed_bytes != entry_bytes {
        return Err(
            ToolchainError::validation_failed(
                "archive entry framing and extracted byte counts differ",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    file.flush().map_err(|error| {
        extraction_io_error(
            error,
            format!("flushing extracted package entry `{package_path}`"),
        )
    })?;

    file.sync_all().map_err(|error| {
        extraction_io_error(
            error,
            format!("synchronizing extracted package entry `{package_path}`"),
        )
    })?;

    drop(file);

    Ok((package_path, staged_file, entry_bytes))
}

// ============================================================================
// MAIN EXTRACTION API
// ============================================================================

/// Extract a `.zpkg` archive into a new destination directory.
///
/// The archive reader must represent the complete archive, starting at
/// its first byte.
///
/// The verifier must validate the manifest and every extracted entry.
///
/// The destination must not already exist. Its parent directory must
/// already exist.
///
/// The operation:
///
/// 1. Validates the archive header.
/// 2. Creates a private sibling staging directory.
/// 3. Streams and validates the manifest.
/// 4. Extracts each regular-file entry.
/// 5. Verifies every extracted file.
/// 6. Validates archive completion and absence of trailing bytes.
/// 7. Removes internal staging data.
/// 8. Publishes the completed directory.
///
/// A failed operation does not publish the destination.
///
/// Staging cleanup is best-effort if the filesystem itself fails.
///
/// # Errors
///
/// Returns an error for:
///
/// - Invalid or unsupported archive framing.
/// - Invalid package paths.
/// - Invalid destination paths.
/// - Existing destination directories.
/// - Symbolic links or non-directory parent components.
/// - Filesystem failures.
/// - Verification failures.
/// - Configured budget exhaustion.
/// - Truncated entries.
/// - Duplicate or non-canonical archive paths.
/// - Trailing archive data.
/// - Resource exhaustion.
///
/// # Security
///
/// No archive-controlled symlink, hard link, device or other special
/// filesystem entry is created.
///
/// The caller must provide an appropriately protected staging parent
/// and an authoritative verifier.
pub fn extract_archive<R, V>(
    reader: R,
    destination: impl AsRef<Path>,
    verifier: &mut V,
) -> Result<ExtractionReport>
where
    R: Read,
    V: ExtractionVerifier,
{
    extract_archive_with_options(
        reader,
        destination,
        verifier,
        ExtractionOptions::default(),
    )
}

/// Extract a `.zpkg` archive with explicit resource budgets and I/O options.
pub fn extract_archive_with_options<R, V>(
    reader: R,
    destination: impl AsRef<Path>,
    verifier: &mut V,
    options: ExtractionOptions,
) -> Result<ExtractionReport>
where
    R: Read,
    V: ExtractionVerifier,
{
    let (destination, destination_name) =
        validate_destination(destination.as_ref())?;

    let archive_options =
        ArchiveIoOptions::new(options.io_buffer_size)?;

    let mut archive = ArchiveReader::new_with_options(
        reader,
        archive_options,
    )?;

    let header = archive.header();

    if let Some(maximum) = options.budgets.max_entries {
        if header.entry_count() > maximum {
            return Err(extraction_budget_error(
                "entry-count",
                header.entry_count(),
                maximum,
            ));
        }
    }

    let parent = destination.parent().ok_or_else(|| {
        ToolchainError::invalid_input(
            "canonical extraction destination has no parent",
        )
        .with_operation(OPERATION_EXTRACT)
    })?;

    let staging =
        StagingDirectory::create(parent, &destination_name)?;

    let staging_root = staging.path().to_path_buf();

    let internal_directory =
        staging_root.join(INTERNAL_DIRECTORY);

    fs::create_dir(&internal_directory).map_err(|error| {
        extraction_io_error(
            error,
            "creating extraction-internal directory",
        )
    })?;

    // Manifest is consumed before the first archive entry.
    let manifest_bytes = extract_manifest(
        &mut archive,
        &internal_directory,
        options.budgets,
    )?;

    let manifest_path =
        internal_directory.join(INTERNAL_MANIFEST);

    verifier
        .verify_manifest(&manifest_path)
        .map_err(|error| {
            error
                .with_operation(OPERATION_EXTRACT)
                .with_context("verifying package manifest before extraction")
        })?;

    let mut extracted_entries = 0_u128;
    let mut extracted_content_bytes = 0_u128;

    loop {
        let entry = match archive.next_entry()? {
            Some(entry) => entry,
            None => break,
        };

        let (package_path, staged_file, entry_bytes) =
            extract_entry(
                entry,
                &staging_root,
                options,
                extracted_content_bytes,
            )?;

        verifier
            .verify_entry(&package_path, &staged_file)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_EXTRACT)
                    .with_context(format!(
                        "verifying extracted package entry `{package_path}`"
                    ))
            })?;

        extracted_content_bytes = checked_add(
            extracted_content_bytes,
            entry_bytes,
            "total extracted content",
        )?;

        extracted_entries = checked_add(
            extracted_entries,
            1,
            "extracted entry",
        )?;
    }

    // The archive reader checks that every entry was consumed and that
    // no undeclared trailing data exists.
    archive.finish()?;

    if extracted_entries != header.entry_count() {
        return Err(
            ToolchainError::validation_failed(
                "extracted entry count differs from archive header",
            )
            .with_operation(OPERATION_EXTRACT),
        );
    }

    // Publication occurs only after complete verification and framing
    // validation. The internal manifest is never part of the package.
    staging.publish(
        &internal_directory,
        &destination,
    )?;

    Ok(ExtractionReport {
        destination,
        archive_entries: header.entry_count(),
        extracted_entries,
        manifest_bytes,
        extracted_content_bytes,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    struct AcceptingVerifier;

    impl ExtractionVerifier for AcceptingVerifier {
        fn verify_manifest(
            &mut self,
            manifest_path: &Path,
        ) -> Result<()> {
            let metadata = fs::metadata(manifest_path).map_err(|error| {
                extraction_io_error(error, "reading test manifest")
            })?;

            if !metadata.is_file() {
                return Err(ToolchainError::validation_failed(
                    "manifest is not a regular file",
                ));
            }

            Ok(())
        }

        fn verify_entry(
            &mut self,
            _package_path: &PackagePath,
            staged_file: &Path,
        ) -> Result<()> {
            let metadata = fs::symlink_metadata(staged_file).map_err(
                |error| {
                    extraction_io_error(
                        error,
                        "reading test extracted entry",
                    )
                },
            )?;

            if metadata.file_type().is_symlink()
                || !metadata.is_file()
            {
                return Err(ToolchainError::validation_failed(
                    "test entry is not a regular file",
                ));
            }

            Ok(())
        }
    }

    #[test]
    fn extraction_options_reject_zero_buffer() {
        assert!(
            ExtractionOptions::new(
                0,
                ExtractionBudgets::default(),
            )
            .is_err()
        );
    }

    #[test]
    fn budgets_preserve_unbounded_defaults() {
        let budgets = ExtractionBudgets::default();

        assert_eq!(budgets.max_entries(), None);
        assert_eq!(budgets.max_manifest_bytes(), None);
        assert_eq!(budgets.max_entry_bytes(), None);
        assert_eq!(budgets.max_total_content_bytes(), None);
    }

    #[test]
    fn portable_paths_reject_parent_traversal() {
        assert!(PackagePath::new("../outside").is_err());
        assert!(PackagePath::new("a/../outside").is_err());
        assert!(PackagePath::new("/absolute").is_err());
    }

    #[test]
    fn portable_paths_reject_windows_device_names() {
        let path = PackagePath::new("CON.txt")
            .expect("model permits a portable logical path");

        assert!(package_relative_path(&path).is_err());
    }

    #[test]
    fn portable_paths_reject_internal_namespace() {
        let path = PackagePath::new(
            ".zamani-internal/manifest",
        )
        .expect("model path is structurally valid");

        assert!(package_relative_path(&path).is_err());
    }

    #[test]
    fn portable_paths_accept_nested_regular_files() {
        let path = PackagePath::new(
            "src/classical/example.zm",
        )
        .expect("valid package path");

        let relative = package_relative_path(&path)
            .expect("portable package path");

        assert_eq!(
            relative,
            PathBuf::from("src")
                .join("classical")
                .join("example.zm"),
        );
    }

    #[test]
    fn byte_counters_detect_overflow() {
        assert!(checked_add(u128::MAX, 1, "test").is_err());
        assert_eq!(checked_add(10, 20, "test").unwrap(), 30);
    }

    #[test]
    fn destination_must_not_already_exist() {
        let root = std::env::temp_dir().join(format!(
            "zamani-extraction-test-{}-{}",
            std::process::id(),
            STAGING_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        ));

        fs::create_dir(&root).expect("create test root");

        let result = validate_destination(&root);

        assert!(result.is_err());

        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn staging_directory_is_removed_when_not_published() {
        let root = std::env::temp_dir().join(format!(
            "zamani-staging-test-{}-{}",
            std::process::id(),
            STAGING_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        ));

        fs::create_dir(&root).expect("create root");

        let staging = StagingDirectory::create(
            &root,
            "output",
        )
        .expect("create staging");

        let staging_path = staging.path().to_path_buf();

        assert!(staging_path.is_dir());

        drop(staging);

        assert!(!staging_path.exists());

        fs::remove_dir_all(root).expect("remove root");
    }

    #[test]
    fn empty_input_is_not_reported_as_success() {
        let root = std::env::temp_dir().join(format!(
            "zamani-empty-test-{}-{}",
            std::process::id(),
            STAGING_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        ));

        fs::create_dir(&root).expect("create root");

        let destination = root.join("output");

        let mut verifier = AcceptingVerifier;

        let result = extract_archive(
            Cursor::new(Vec::<u8>::new()),
            &destination,
            &mut verifier,
        );

        assert!(result.is_err());
        assert!(!destination.exists());

        fs::remove_dir_all(root).expect("remove root");
    }
}
