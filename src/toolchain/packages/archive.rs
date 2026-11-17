#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Archive
//!
//! Path: `src/toolchain/packages/archive.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the versioned `.zpkg` archive container and its streaming
//! reader and writer.
//!
//! The archive format is independent of the computational domain.
//! It can carry source files, libraries, native artifacts, WebAssembly,
//! quantum artifacts, HDL designs, AI models, datasets, firmware,
//! distributed application components, and future package content.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Archive magic and format version.
//! - Archive header representation.
//! - Canonical archive entry ordering.
//! - Archive framing and length encoding.
//! - Streaming archive writing.
//! - Streaming archive reading.
//! - Structural archive validation.
//! - Archive-level truncation and trailing-data detection.
//!
//! This module does not own:
//!
//! - Package identity or package metadata.
//! - Package manifest parsing or serialization.
//! - Package-content selection.
//! - Filesystem traversal.
//! - Extraction destinations.
//! - Package installation.
//! - Content hashing or digest verification.
//! - Cryptographic signing or signature verification.
//! - Publisher authentication or trust.
//! - Dependency resolution.
//! - Compatibility evaluation.
//! - Compression or encryption.
//! - Registry access or network downloading.
//! - Compiler, backend, hardware or runtime execution.
//!
//! ## Format version 1
//!
//! All integer fields are unsigned little-endian values.
//!
//! Archive layout:
//!
//! ```text
//! +----------------------+------------------------------+
//! | Field                | Representation               |
//! +----------------------+------------------------------+
//! | Magic                | 8 bytes: ZPKG\\r\\n\\x1A\\n      |
//! | Major version        | u16                          |
//! | Minor version        | u16                          |
//! | Feature flags        | u64                          |
//! | Manifest length      | u128                         |
//! | Entry count          | u128                         |
//! | Manifest             | manifest-length bytes       |
//! | Entry 1              | path + chunk stream          |
//! | Entry 2              | path + chunk stream          |
//! | ...                  | ...                          |
//! +----------------------+------------------------------+
//! ```
//!
//! Each entry is encoded as:
//!
//! ```text
//! +----------------------+------------------------------+
//! | Path length          | u128                         |
//! | UTF-8 portable path  | path-length bytes            |
//! | Chunk length         | u64                          |
//! | Chunk bytes          | chunk-length bytes           |
//! | ...                  | ...                          |
//! | Zero chunk length    | u64 (entry terminator)       |
//! +----------------------+------------------------------+
//! ```
//!
//! A zero-length chunk terminates an entry. Consequently, empty files
//! are represented without ambiguity.
//!
//! Chunk framing permits streaming content whose final size is not
//! known before reading it. There is no archive-wide or file-wide
//! application-level size ceiling in this format.
//!
//! The u128 length fields and the host's address space remain finite
//! representation constraints, not language or hardware limits.
//!
//! ## Determinism
//!
//! The writer follows the deterministic `BTreeMap` ordering exposed by
//! `Package::contents()`.
//!
//! The reader requires strictly increasing paths. This rejects duplicate
//! entries and non-canonical ordering without retaining a second
//! collection of all archive paths.
//!
//! ## Security
//!
//! Archive paths are revalidated through `PackagePath::new`.
//!
//! This is structural validation only. The extraction subsystem must
//! independently enforce destination containment, symlink policy,
//! filesystem race protection, configured extraction budgets and
//! platform-specific path safety.
//!
//! Archive framing does not establish authenticity or integrity.
//! `packages::verification` and the canonical security subsystem own
//! those checks.
//!
//! ## Manifest boundary
//!
//! The manifest is an opaque byte stream here. Its canonical encoding
//! and decoding belong to `packages::metadata`.
//!
//! The caller must provide a manifest that describes the same package
//! whose content inventory is passed to the writer.
//!
//! ## Integration
//!
//! Consumed by:
//!
//! - `packages::creation`
//! - `packages::extraction`
//! - `packages::verification`
//! - `packages::installation`
//! - `registries::downloading`
//! - `registries::publishing`
//!
//! `packages::metadata` produces and consumes the manifest byte stream.
//!
//! `packages::validation` validates the decoded package model.
//!
//! ## Scalability
//!
//! The archive format has no arbitrary maximum for:
//!
//! - Number of package entries.
//! - Total archive size.
//! - Individual content size.
//! - Manifest size.
//! - Path length.
//! - Computing domains.
//! - Artifact formats.
//! - Target descriptions.
//!
//! I/O uses a reusable configurable buffer. Its size controls I/O
//! granularity; it does not cap the size of an archive or entry.
//!
//! Allocation failures and representational overflows are reported.
//!
//! Literal infinite storage or computation is not promised.

use std::io::{self, Read, Write};
use std::num::NonZeroUsize;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use super::model::{Package, PackagePath};

// ============================================================================
// FORMAT CONSTANTS
// ============================================================================

/// Canonical eight-byte `.zpkg` archive signature.
pub const ARCHIVE_MAGIC: [u8; 8] = *b"ZPKG\r\n\x1A\n";

/// Supported archive-format major version.
pub const ARCHIVE_FORMAT_MAJOR: u16 = 1;

/// Supported archive-format minor version.
pub const ARCHIVE_FORMAT_MINOR: u16 = 0;

/// Default reusable I/O buffer size.
///
/// This is an I/O performance default, not a maximum archive or file size.
pub const DEFAULT_IO_BUFFER_SIZE: usize = 64 * 1024;

/// Archive writer and reader operation identifier.
const OPERATION_ARCHIVE: &str = "package.archive";

/// Archive writer operation identifier.
const OPERATION_ARCHIVE_WRITE: &str = "package.archive.write";

/// Archive reader operation identifier.
const OPERATION_ARCHIVE_READ: &str = "package.archive.read";

// ============================================================================
// FORMAT VERSION
// ============================================================================

/// Version of the `.zpkg` container format.
///
/// This version is independent of package release versions, Zamani
/// language versions, compiler versions, IR versions and toolchain API
/// versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchiveFormatVersion {
    major: u16,
    minor: u16,
}

impl ArchiveFormatVersion {
    /// Return the currently supported archive-format version.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            major: ARCHIVE_FORMAT_MAJOR,
            minor: ARCHIVE_FORMAT_MINOR,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }
}

// ============================================================================
// I/O OPTIONS
// ============================================================================

/// Configurable archive I/O options.
///
/// The buffer size controls how much data is processed per I/O operation.
/// It does not restrict total archive size, entry size or manifest size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveIoOptions {
    buffer_size: NonZeroUsize,
}

impl ArchiveIoOptions {
    /// Construct options using a nonzero reusable buffer size.
    pub fn new(buffer_size: usize) -> Result<Self> {
        let buffer_size = NonZeroUsize::new(buffer_size).ok_or_else(|| {
            ToolchainError::invalid_input(
                "archive I/O buffer size must be greater than zero",
            )
            .with_operation(OPERATION_ARCHIVE)
        })?;

        Ok(Self { buffer_size })
    }

    /// Return the configured I/O buffer size.
    #[must_use]
    pub const fn buffer_size(self) -> usize {
        self.buffer_size.get()
    }
}

impl Default for ArchiveIoOptions {
    fn default() -> Self {
        Self {
            buffer_size: NonZeroUsize::new(DEFAULT_IO_BUFFER_SIZE)
                .expect("the default archive buffer size is nonzero"),
        }
    }
}

// ============================================================================
// ARCHIVE HEADER
// ============================================================================

/// Validated archive header.
///
/// The header describes container framing. It does not prove that the
/// manifest, package contents, signatures or compatibility declarations
/// are valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveHeader {
    version: ArchiveFormatVersion,
    manifest_length: u128,
    entry_count: u128,
}

impl ArchiveHeader {
    /// Return the archive-format version.
    #[must_use]
    pub const fn version(&self) -> ArchiveFormatVersion {
        self.version
    }

    /// Return the encoded manifest length.
    #[must_use]
    pub const fn manifest_length(&self) -> u128 {
        self.manifest_length
    }

    /// Return the declared number of content entries.
    #[must_use]
    pub const fn entry_count(&self) -> u128 {
        self.entry_count
    }
}

// ============================================================================
// WRITE SUMMARY
// ============================================================================

/// Summary of a successfully written archive.
///
/// Counts describe bytes of manifest and content payload, not total
/// container bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveWriteSummary {
    manifest_bytes: u128,
    content_bytes: u128,
    entry_count: u128,
}

impl ArchiveWriteSummary {
    /// Return the number of manifest bytes written.
    #[must_use]
    pub const fn manifest_bytes(&self) -> u128 {
        self.manifest_bytes
    }

    /// Return the number of content bytes written.
    #[must_use]
    pub const fn content_bytes(&self) -> u128 {
        self.content_bytes
    }

    /// Return the number of content entries written.
    #[must_use]
    pub const fn entry_count(&self) -> u128 {
        self.entry_count
    }
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn archive_io_error(error: io::Error, context: &'static str) -> ToolchainError {
    ToolchainError::from(error)
        .with_operation(OPERATION_ARCHIVE)
        .with_context(context)
}

fn invalid_archive(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(OPERATION_ARCHIVE_READ)
}

fn archive_resource_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::resource_exhausted(message)
        .with_operation(OPERATION_ARCHIVE)
}

fn allocate_buffer(size: usize) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    buffer.try_reserve_exact(size).map_err(|error| {
        archive_resource_error(format!(
            "unable to allocate archive I/O buffer: {error}"
        ))
    })?;

    buffer.resize(size, 0);

    Ok(buffer)
}

// ============================================================================
// INTEGER ENCODING
// ============================================================================

fn write_u16<W: Write>(writer: &mut W, value: u16) -> Result<()> {
    writer
        .write_all(&value.to_le_bytes())
        .map_err(|error| archive_io_error(error, "writing archive u16"))
}

fn write_u64<W: Write>(writer: &mut W, value: u64) -> Result<()> {
    writer
        .write_all(&value.to_le_bytes())
        .map_err(|error| archive_io_error(error, "writing archive u64"))
}

fn write_u128<W: Write>(writer: &mut W, value: u128) -> Result<()> {
    writer
        .write_all(&value.to_le_bytes())
        .map_err(|error| archive_io_error(error, "writing archive u128"))
}

fn read_u16<R: Read>(reader: &mut R) -> Result<u16> {
    let mut bytes = [0_u8; 2];

    reader
        .read_exact(&mut bytes)
        .map_err(|error| archive_io_error(error, "reading archive u16"))?;

    Ok(u16::from_le_bytes(bytes))
}

fn read_u64<R: Read>(reader: &mut R) -> Result<u64> {
    let mut bytes = [0_u8; 8];

    reader
        .read_exact(&mut bytes)
        .map_err(|error| archive_io_error(error, "reading archive u64"))?;

    Ok(u64::from_le_bytes(bytes))
}

fn read_u128<R: Read>(reader: &mut R) -> Result<u128> {
    let mut bytes = [0_u8; 16];

    reader
        .read_exact(&mut bytes)
        .map_err(|error| archive_io_error(error, "reading archive u128"))?;

    Ok(u128::from_le_bytes(bytes))
}

// ============================================================================
// EXACT-LENGTH STREAM COPYING
// ============================================================================

fn copy_exact<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    mut remaining: u128,
    buffer: &mut [u8],
    context: &'static str,
) -> Result<u128> {
    let expected = remaining;
    let mut copied = 0_u128;

    while remaining > 0 {
        let requested = if remaining > buffer.len() as u128 {
            buffer.len()
        } else {
            usize::try_from(remaining).map_err(|_| {
                archive_resource_error(
                    "archive copy length cannot be represented by this host",
                )
            })?
        };

        let count = reader
            .read(&mut buffer[..requested])
            .map_err(|error| archive_io_error(error, context))?;

        if count == 0 {
            return Err(invalid_archive(format!(
                "{context}: input ended before the declared length \
                 of {expected} bytes"
            )));
        }

        writer
            .write_all(&buffer[..count])
            .map_err(|error| archive_io_error(error, context))?;

        let count = count as u128;

        remaining = remaining.checked_sub(count).ok_or_else(|| {
            ToolchainError::new(
                crate::toolchain::errors::ToolchainErrorKind::Internal,
                "archive copy remaining-length invariant failed",
            )
            .with_operation(OPERATION_ARCHIVE)
        })?;

        copied = copied.checked_add(count).ok_or_else(|| {
            archive_resource_error("archive byte counter overflowed")
        })?;
    }

    Ok(copied)
}

// ============================================================================
// ARCHIVE WRITER
// ============================================================================

/// Write a `.zpkg` archive using the default I/O options.
///
/// # Arguments
///
/// - `writer`: destination byte stream.
/// - `package`: validated package model and ordered content inventory.
/// - `manifest`: canonical manifest byte stream produced by
///   `packages::metadata`.
/// - `manifest_length`: exact number of manifest bytes.
/// - `open_content`: callback that opens the byte stream for a package path.
///
/// The manifest must describe the supplied package. This function does
/// not encode or decode package metadata.
///
/// Content streams are opened in the deterministic order of
/// `Package::contents()`.
///
/// A successful return means the archive bytes were written to the
/// supplied writer. It does not mean they were persisted atomically.
/// Callers that write files should use a staging file and commit it only
/// after this function succeeds.
///
/// # Errors
///
/// Returns an error for invalid package models, I/O failures, content
/// length mismatches, manifest length mismatches or counter overflow.
pub fn write_archive<W, M, F, R>(
    writer: &mut W,
    package: &Package,
    manifest: &mut M,
    manifest_length: u128,
    open_content: F,
) -> Result<ArchiveWriteSummary>
where
    W: Write,
    M: Read,
    F: FnMut(&PackagePath) -> Result<R>,
    R: Read,
{
    write_archive_with_options(
        writer,
        package,
        manifest,
        manifest_length,
        open_content,
        ArchiveIoOptions::default(),
    )
}

/// Write a `.zpkg` archive using explicit I/O options.
///
/// This is the configurable counterpart to [`write_archive`].
pub fn write_archive_with_options<W, M, F, R>(
    writer: &mut W,
    package: &Package,
    manifest: &mut M,
    manifest_length: u128,
    mut open_content: F,
    options: ArchiveIoOptions,
) -> Result<ArchiveWriteSummary>
where
    W: Write,
    M: Read,
    F: FnMut(&PackagePath) -> Result<R>,
    R: Read,
{
    package
        .validate()
        .map_err(|error| error.with_operation(OPERATION_ARCHIVE_WRITE))?;

    let mut buffer = allocate_buffer(options.buffer_size())?;

    // Header.
    writer
        .write_all(&ARCHIVE_MAGIC)
        .map_err(|error| archive_io_error(error, "writing archive magic"))?;

    write_u16(writer, ARCHIVE_FORMAT_MAJOR)?;
    write_u16(writer, ARCHIVE_FORMAT_MINOR)?;

    // Version 1.0 defines no optional feature flags.
    write_u64(writer, 0)?;

    write_u128(writer, manifest_length)?;
    write_u128(writer, package.contents().len() as u128)?;

    // Manifest.
    let manifest_bytes = copy_exact(
        manifest,
        writer,
        manifest_length,
        &mut buffer,
        "writing package manifest",
    )?;

    // Reject a manifest source that supplied more bytes than declared.
    let mut extra = [0_u8; 1];

    let extra_count = manifest
        .read(&mut extra)
        .map_err(|error| archive_io_error(error, "checking manifest length"))?;

    if extra_count != 0 {
        return Err(ToolchainError::validation_failed(
            "manifest stream contains more bytes than its declared length",
        )
        .with_operation(OPERATION_ARCHIVE_WRITE));
    }

    let mut content_bytes = 0_u128;

    // BTreeMap iteration provides canonical path ordering.
    for (path, content) in package.contents() {
        let path_bytes = path.as_str().as_bytes();

        write_u128(writer, path_bytes.len() as u128)?;

        writer
            .write_all(path_bytes)
            .map_err(|error| archive_io_error(error, "writing package path"))?;

        let mut source = open_content(path).map_err(|error| {
            error
                .with_operation(OPERATION_ARCHIVE_WRITE)
                .with_context(format!("opening package content `{path}`"))
        })?;

        let mut entry_bytes = 0_u128;

        loop {
            let count = source
                .read(&mut buffer)
                .map_err(|error| archive_io_error(
                    error,
                    "reading package content stream",
                ))?;

            if count == 0 {
                break;
            }

            let chunk_length = u64::try_from(count).map_err(|_| {
                archive_resource_error(
                    "archive I/O chunk length cannot be represented as u64",
                )
            })?;

            write_u64(writer, chunk_length)?;

            writer
                .write_all(&buffer[..count])
                .map_err(|error| archive_io_error(
                    error,
                    "writing package content chunk",
                ))?;

            let count = count as u128;

            entry_bytes = entry_bytes.checked_add(count).ok_or_else(|| {
                archive_resource_error("package entry byte count overflowed")
            })?;

            content_bytes = content_bytes.checked_add(count).ok_or_else(|| {
                archive_resource_error("archive content byte count overflowed")
            })?;
        }

        if let Some(expected) = content.declared_size() {
            if entry_bytes != expected {
                return Err(ToolchainError::validation_failed(format!(
                    "package content `{path}` has {entry_bytes} bytes, \
                     but its model declares {expected} bytes"
                ))
                .with_operation(OPERATION_ARCHIVE_WRITE));
            }
        }

        // Zero-length chunk terminates this entry.
        write_u64(writer, 0)?;
    }

    Ok(ArchiveWriteSummary {
        manifest_bytes,
        content_bytes,
        entry_count: package.contents().len() as u128,
    })
}

// ============================================================================
// ARCHIVE READER
// ============================================================================

/// Streaming reader for a `.zpkg` archive.
///
/// The reader validates the archive header immediately. The caller must
/// consume the manifest, consume every entry, and call [`Self::finish`]
/// to establish that the complete container has no trailing bytes.
///
/// It does not decode the package manifest or extract content to disk.
pub struct ArchiveReader<R> {
    inner: R,
    header: ArchiveHeader,
    manifest_remaining: u128,
    entries_remaining: u128,
    last_path: Option<PackagePath>,
    active_entry: bool,
    poisoned: bool,
    options: ArchiveIoOptions,
}

impl<R: Read> ArchiveReader<R> {
    /// Construct a reader using the default I/O options.
    pub fn new(reader: R) -> Result<Self> {
        Self::new_with_options(reader, ArchiveIoOptions::default())
    }

    /// Construct a reader using explicit I/O options.
    pub fn new_with_options(
        mut reader: R,
        options: ArchiveIoOptions,
    ) -> Result<Self> {
        let mut magic = [0_u8; ARCHIVE_MAGIC.len()];

        reader
            .read_exact(&mut magic)
            .map_err(|error| archive_io_error(error, "reading archive magic"))?;

        if magic != ARCHIVE_MAGIC {
            return Err(invalid_archive("invalid .zpkg archive signature"));
        }

        let major = read_u16(&mut reader)?;
        let minor = read_u16(&mut reader)?;
        let flags = read_u64(&mut reader)?;

        if major != ARCHIVE_FORMAT_MAJOR {
            return Err(ToolchainError::incompatible_version(format!(
                "unsupported .zpkg major version {major}; \
                 this implementation supports {ARCHIVE_FORMAT_MAJOR}"
            ))
            .with_operation(OPERATION_ARCHIVE_READ));
        }

        if minor > ARCHIVE_FORMAT_MINOR {
            return Err(ToolchainError::incompatible_version(format!(
                "unsupported .zpkg minor version {minor}; \
                 this implementation supports up to {ARCHIVE_FORMAT_MINOR}"
            ))
            .with_operation(OPERATION_ARCHIVE_READ));
        }

        if flags != 0 {
            return Err(ToolchainError::unsupported_operation(format!(
                "archive declares unsupported feature flags {flags:#x}"
            ))
            .with_operation(OPERATION_ARCHIVE_READ));
        }

        let manifest_length = read_u128(&mut reader)?;
        let entry_count = read_u128(&mut reader)?;

        let header = ArchiveHeader {
            version: ArchiveFormatVersion { major, minor },
            manifest_length,
            entry_count,
        };

        Ok(Self {
            inner: reader,
            header,
            manifest_remaining: manifest_length,
            entries_remaining: entry_count,
            last_path: None,
            active_entry: false,
            poisoned: false,
            options,
        })
    }

    /// Return the validated archive header.
    #[must_use]
    pub const fn header(&self) -> ArchiveHeader {
        self.header
    }

    /// Copy the complete manifest to a destination stream.
    ///
    /// This method streams the manifest and does not allocate a buffer
    /// proportional to its total size.
    ///
    /// It must be called before [`Self::next_entry`].
    ///
    /// If the destination fails after the archive stream has advanced,
    /// the reader becomes unusable and must be discarded.
    pub fn copy_manifest_to<W: Write>(
        &mut self,
        destination: &mut W,
    ) -> Result<u128> {
        if self.poisoned {
            return Err(invalid_archive(
                "archive reader is unusable after an earlier read failure",
            ));
        }

        if self.active_entry {
            return Err(invalid_archive(
                "cannot read the manifest while an archive entry is active",
            ));
        }

        if self.manifest_remaining == 0 {
            return Ok(0);
        }

        let mut buffer = allocate_buffer(self.options.buffer_size())?;
        let mut copied = 0_u128;

        while self.manifest_remaining > 0 {
            let requested = if self.manifest_remaining > buffer.len() as u128 {
                buffer.len()
            } else {
                usize::try_from(self.manifest_remaining).map_err(|_| {
                    archive_resource_error(
                        "manifest length cannot be represented by this host",
                    )
                })?
            };

            let count = match self.inner.read(&mut buffer[..requested]) {
                Ok(count) => count,
                Err(error) => {
                    self.poisoned = true;

                    return Err(archive_io_error(
                        error,
                        "reading package manifest",
                    ));
                }
            };

            if count == 0 {
                self.poisoned = true;

                return Err(invalid_archive(
                    "archive ended before the complete manifest was read",
                ));
            }

            if let Err(error) = destination.write_all(&buffer[..count]) {
                self.poisoned = true;

                return Err(archive_io_error(
                    error,
                    "writing decoded manifest destination",
                ));
            }

            let count = count as u128;

            self.manifest_remaining -= count;

            copied = copied.checked_add(count).ok_or_else(|| {
                archive_resource_error("manifest byte counter overflowed")
            })?;
        }

        Ok(copied)
    }

    /// Open the next archive entry as a streaming reader.
    ///
    /// The returned entry must be fully consumed or explicitly finished
    /// before the next entry can be opened.
    ///
    /// Returns `Ok(None)` when all declared entries have been consumed.
    ///
    /// Paths are validated and must be in strictly increasing lexical
    /// order. This rejects duplicate and non-canonical entry ordering.
    pub fn next_entry(&mut self) -> Result<Option<ArchiveEntryReader<'_, R>>> {
        if self.poisoned {
            return Err(invalid_archive(
                "archive reader is unusable after an earlier read failure",
            ));
        }

        if self.active_entry {
            return Err(invalid_archive(
                "the previous archive entry has not been completed",
            ));
        }

        if self.manifest_remaining != 0 {
            return Err(invalid_archive(
                "the package manifest must be consumed before archive entries",
            ));
        }

        if self.entries_remaining == 0 {
            return Ok(None);
        }

        let path_length = read_u128(&mut self.inner)?;

        let path_length = usize::try_from(path_length).map_err(|_| {
            archive_resource_error(
                "archive path length cannot be represented by this host",
            )
        })?;

        if path_length == 0 {
            self.poisoned = true;

            return Err(invalid_archive(
                "archive entry path must not be empty",
            ));
        }

        let mut path_bytes = Vec::new();

        path_bytes.try_reserve_exact(path_length).map_err(|error| {
            archive_resource_error(format!(
                "unable to allocate archive path buffer: {error}"
            ))
        })?;

        path_bytes.resize(path_length, 0);

        if let Err(error) = self.inner.read_exact(&mut path_bytes) {
            self.poisoned = true;

            return Err(archive_io_error(
                error,
                "reading archive entry path",
            ));
        }

        let path_text = std::str::from_utf8(&path_bytes).map_err(|_| {
            self.poisoned = true;
            invalid_archive("archive entry path is not valid UTF-8")
        })?;

        let path = PackagePath::new(path_text).map_err(|error| {
            self.poisoned = true;

            error
                .with_operation(OPERATION_ARCHIVE_READ)
                .with_context("validating archive entry path")
        })?;

        if let Some(previous) = &self.last_path {
            if previous >= &path {
                self.poisoned = true;

                return Err(invalid_archive(format!(
                    "archive entries are not in strictly increasing path order: \
                     `{previous}` followed by `{path}`"
                )));
            }
        }

        self.last_path = Some(path.clone());
        self.active_entry = true;

        Ok(Some(ArchiveEntryReader {
            archive: self,
            path,
            chunk_remaining: 0,
            bytes_read: 0,
            finished: false,
        }))
    }

    /// Finish reading and validate the complete archive stream.
    ///
    /// This requires:
    ///
    /// - The complete manifest to have been consumed.
    /// - Every declared entry to have been completely consumed.
    /// - No incomplete entry to have been dropped.
    /// - No trailing bytes after the declared archive contents.
    ///
    /// The underlying reader is consumed by this operation.
    pub fn finish(mut self) -> Result<()> {
        if self.poisoned {
            return Err(invalid_archive(
                "cannot finish an archive reader after a previous read failure",
            ));
        }

        if self.manifest_remaining != 0 {
            return Err(invalid_archive(format!(
                "{} manifest bytes remain unread",
                self.manifest_remaining
            )));
        }

        if self.active_entry {
            return Err(invalid_archive(
                "an archive entry is still active",
            ));
        }

        if self.entries_remaining != 0 {
            return Err(invalid_archive(format!(
                "{} archive entries remain unread",
                self.entries_remaining
            )));
        }

        let mut trailing = [0_u8; 1];

        let count = self
            .inner
            .read(&mut trailing)
            .map_err(|error| archive_io_error(
                error,
                "checking for trailing archive data",
            ))?;

        if count != 0 {
            return Err(invalid_archive(
                "archive contains trailing bytes after its declared contents",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// ARCHIVE ENTRY READER
// ============================================================================

/// Streaming reader for one archive content entry.
///
/// The entry reader validates chunk framing while exposing the concatenated
/// content bytes through the standard [`Read`] interface.
///
/// Dropping an entry before its terminator is reached poisons the parent
/// archive reader. This prevents silently skipping unconsumed data.
pub struct ArchiveEntryReader<'a, R: Read> {
    archive: &'a mut ArchiveReader<R>,
    path: PackagePath,
    chunk_remaining: u64,
    bytes_read: u128,
    finished: bool,
}

impl<R: Read> ArchiveEntryReader<'_, R> {
    /// Return the validated package-relative path for this entry.
    #[must_use]
    pub fn path(&self) -> &PackagePath {
        &self.path
    }

    /// Return the number of content bytes consumed so far.
    #[must_use]
    pub const fn bytes_read(&self) -> u128 {
        self.bytes_read
    }

    /// Drain and finish this entry.
    ///
    /// Returns the total content byte count.
    ///
    /// This method does not retain the entry contents in memory.
    pub fn finish(mut self) -> Result<u128> {
        io::copy(&mut self, &mut io::sink()).map_err(|error| {
            archive_io_error(error, "draining archive entry")
        })?;

        if !self.finished {
            self.archive.poisoned = true;

            return Err(invalid_archive(
                "archive entry did not reach its terminator",
            ));
        }

        Ok(self.bytes_read)
    }

    fn fail<T>(&mut self, error: io::Error) -> io::Result<T> {
        self.archive.poisoned = true;
        self.archive.active_entry = false;
        Err(error)
    }
}

impl<R: Read> Read for ArchiveEntryReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() || self.finished {
            return Ok(0);
        }

        if self.chunk_remaining == 0 {
            let mut length_bytes = [0_u8; 8];

            if let Err(error) = self.archive.inner.read_exact(&mut length_bytes) {
                return self.fail(error);
            }

            let chunk_length = u64::from_le_bytes(length_bytes);

            if chunk_length == 0 {
                self.finished = true;
                self.archive.active_entry = false;

                if self.archive.entries_remaining == 0 {
                    return self.fail(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "archive entry count underflow",
                    ));
                }

                self.archive.entries_remaining -= 1;

                return Ok(0);
            }

            self.chunk_remaining = chunk_length;
        }

        let maximum = usize::try_from(self.chunk_remaining)
            .unwrap_or(usize::MAX);

        let requested = buffer.len().min(maximum);

        let count = match self.archive.inner.read(&mut buffer[..requested]) {
            Ok(0) => {
                return self.fail(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "archive ended inside a content chunk",
                ));
            }

            Ok(count) => count,

            Err(error) => return self.fail(error),
        };

        let count_u64 = u64::try_from(count).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "content read count cannot be represented as u64",
            )
        })?;

        self.chunk_remaining -= count_u64;

        self.bytes_read = match self.bytes_read.checked_add(count as u128) {
            Some(value) => value,

            None => {
                return self.fail(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "archive entry byte counter overflowed",
                ));
            }
        };

        Ok(count)
    }
}

impl<R: Read> Drop for ArchiveEntryReader<'_, R> {
    fn drop(&mut self) {
        if !self.finished {
            self.archive.poisoned = true;
            self.archive.active_entry = false;
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::io::Cursor;

    use crate::toolchain::dependencies::constraints::VersionSchemeId;
    use crate::toolchain::dependencies::model::DependencyName;
    use crate::toolchain::ids::{IdNamespace, PackageId};

    use super::super::model::{
        PackageContent,
        PackageContentKind,
        PackageIdentity,
        PackageType,
        PackageVersion,
    };

    fn package_with_contents() -> Package {
        let identity = PackageIdentity::new(
            PackageId::new(
                IdNamespace::new("zamani.package")
                    .expect("valid namespace"),
                1,
            )
            .expect("valid package ID"),
            DependencyName::new("org::zamani::archive-test")
                .expect("valid package name"),
        );

        let version = PackageVersion::new(
            VersionSchemeId::new("semver")
                .expect("valid version scheme"),
            "1.0.0",
        )
        .expect("valid package version");

        let package = Package::new(
            identity,
            version,
            PackageType::new("library").expect("valid package type"),
        );

        let first = PackageContent::new(
            PackagePath::new("src/main.zm").expect("valid path"),
            PackageContentKind::new("source").expect("valid kind"),
        )
        .with_declared_size(11);

        let second = PackageContent::new(
            PackagePath::new("resources/data.bin").expect("valid path"),
            PackageContentKind::new("resource").expect("valid kind"),
        )
        .with_declared_size(4);

        package
            .with_content(first)
            .expect("add first content")
            .with_content(second)
            .expect("add second content")
    }

    fn write_test_archive() -> Vec<u8> {
        let package = package_with_contents();

        let manifest_bytes = b"canonical-package-manifest";
        let mut manifest = Cursor::new(manifest_bytes.as_slice());

        let mut content = BTreeMap::new();

        content.insert(
            "src/main.zm".to_owned(),
            b"hello world".to_vec(),
        );

        content.insert(
            "resources/data.bin".to_owned(),
            vec![0, 1, 2, 3],
        );

        let mut archive = Vec::new();

        write_archive_with_options(
            &mut archive,
            &package,
            &mut manifest,
            manifest_bytes.len() as u128,
            |path| {
                let bytes = content
                    .get(path.as_str())
                    .cloned()
                    .ok_or_else(|| {
                        ToolchainError::not_found(
                            "test content was not found",
                        )
                    })?;

                Ok(Cursor::new(bytes))
            },
            ArchiveIoOptions::new(3).expect("valid I/O options"),
        )
        .expect("write archive");

        archive
    }

    #[test]
    fn archive_round_trip_streams_manifest_and_entries() {
        let bytes = write_test_archive();

        let mut reader = ArchiveReader::new_with_options(
            Cursor::new(bytes),
            ArchiveIoOptions::new(5).expect("valid options"),
        )
        .expect("open archive");

        assert_eq!(
            reader.header().version(),
            ArchiveFormatVersion::current()
        );

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        assert_eq!(
            manifest,
            b"canonical-package-manifest"
        );

        let mut entries = BTreeMap::new();

        while let Some(mut entry) =
            reader.next_entry().expect("next archive entry")
        {
            let path = entry.path().as_str().to_owned();
            let mut content = Vec::new();

            entry
                .read_to_end(&mut content)
                .expect("read entry content");

            entries.insert(path, content);
        }

        assert_eq!(
            entries.get("src/main.zm"),
            Some(&b"hello world".to_vec())
        );

        assert_eq!(
            entries.get("resources/data.bin"),
            Some(&vec![0, 1, 2, 3])
        );

        reader.finish().expect("finish archive");
    }

    #[test]
    fn writer_uses_deterministic_content_order() {
        let first = write_test_archive();
        let second = write_test_archive();

        assert_eq!(first, second);
    }

    #[test]
    fn writer_rejects_declared_content_size_mismatch() {
        let package = package_with_contents();
        let manifest_bytes = b"manifest";
        let mut manifest = Cursor::new(manifest_bytes.as_slice());

        let mut archive = Vec::new();

        let result = write_archive(
            &mut archive,
            &package,
            &mut manifest,
            manifest_bytes.len() as u128,
            |path| {
                let bytes = if path.as_str() == "src/main.zm" {
                    b"short".to_vec()
                } else {
                    vec![0, 1, 2, 3]
                };

                Ok(Cursor::new(bytes))
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn reader_rejects_invalid_magic() {
        let result = ArchiveReader::new(Cursor::new(
            b"NOT-ZPKG".to_vec(),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn reader_rejects_unsupported_major_version() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&ARCHIVE_MAGIC);
        bytes.extend_from_slice(&2_u16.to_le_bytes());
        bytes.extend_from_slice(&0_u16.to_le_bytes());
        bytes.extend_from_slice(&0_u64.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());

        let result = ArchiveReader::new(Cursor::new(bytes));

        assert!(result.is_err());
    }

    #[test]
    fn reader_rejects_unknown_feature_flags() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&ARCHIVE_MAGIC);
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MAJOR.to_le_bytes());
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MINOR.to_le_bytes());
        bytes.extend_from_slice(&1_u64.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());

        let result = ArchiveReader::new(Cursor::new(bytes));

        assert!(result.is_err());
    }

    #[test]
    fn reader_rejects_trailing_bytes() {
        let mut bytes = write_test_archive();
        bytes.push(0xAA);

        let mut reader =
            ArchiveReader::new(Cursor::new(bytes)).expect("open archive");

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        while let Some(entry) =
            reader.next_entry().expect("next archive entry")
        {
            entry.finish().expect("finish entry");
        }

        assert!(reader.finish().is_err());
    }

    #[test]
    fn dropping_an_incomplete_entry_poisoned_the_reader() {
        let bytes = write_test_archive();

        let mut reader =
            ArchiveReader::new(Cursor::new(bytes)).expect("open archive");

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        {
            let _entry = reader
                .next_entry()
                .expect("next archive entry")
                .expect("entry exists");
        }

        assert!(reader.next_entry().is_err());
        assert!(reader.finish().is_err());
    }

    #[test]
    fn reader_rejects_path_traversal() {
        let path = b"../outside";

        let mut bytes = Vec::new();

        bytes.extend_from_slice(&ARCHIVE_MAGIC);
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MAJOR.to_le_bytes());
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MINOR.to_le_bytes());
        bytes.extend_from_slice(&0_u64.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());
        bytes.extend_from_slice(&1_u128.to_le_bytes());
        bytes.extend_from_slice(&(path.len() as u128).to_le_bytes());
        bytes.extend_from_slice(path);

        let mut reader =
            ArchiveReader::new(Cursor::new(bytes)).expect("open archive");

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        assert!(reader.next_entry().is_err());
    }

    #[test]
    fn reader_rejects_noncanonical_entry_order() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&ARCHIVE_MAGIC);
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MAJOR.to_le_bytes());
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MINOR.to_le_bytes());
        bytes.extend_from_slice(&0_u64.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());
        bytes.extend_from_slice(&2_u128.to_le_bytes());

        for path in [b"z.txt".as_slice(), b"a.txt".as_slice()] {
            bytes.extend_from_slice(&(path.len() as u128).to_le_bytes());
            bytes.extend_from_slice(path);
            bytes.extend_from_slice(&0_u64.to_le_bytes());
        }

        let mut reader =
            ArchiveReader::new(Cursor::new(bytes)).expect("open archive");

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        let first = reader
            .next_entry()
            .expect("first entry")
            .expect("first exists");

        first.finish().expect("finish first entry");

        assert!(reader.next_entry().is_err());
    }

    #[test]
    fn empty_content_entry_is_supported() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&ARCHIVE_MAGIC);
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MAJOR.to_le_bytes());
        bytes.extend_from_slice(&ARCHIVE_FORMAT_MINOR.to_le_bytes());
        bytes.extend_from_slice(&0_u64.to_le_bytes());
        bytes.extend_from_slice(&0_u128.to_le_bytes());
        bytes.extend_from_slice(&1_u128.to_le_bytes());

        let path = b"empty.bin";

        bytes.extend_from_slice(&(path.len() as u128).to_le_bytes());
        bytes.extend_from_slice(path);
        bytes.extend_from_slice(&0_u64.to_le_bytes());

        let mut reader =
            ArchiveReader::new(Cursor::new(bytes)).expect("open archive");

        let mut manifest = Vec::new();

        reader
            .copy_manifest_to(&mut manifest)
            .expect("read manifest");

        let mut entry = reader
            .next_entry()
            .expect("next entry")
            .expect("entry exists");

        assert_eq!(entry.path().as_str(), "empty.bin");

        let mut content = Vec::new();

        entry
            .read_to_end(&mut content)
            .expect("read empty entry");

        assert!(content.is_empty());

        drop(entry);

        reader.finish().expect("finish archive");
    }
}