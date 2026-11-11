
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Storage
//!
//! Path: `src/stdlib/data/storage.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, provider-independent contracts for persistent and
//! externally managed data storage.
//!
//! This module provides:
//!
//! - opaque logical storage keys;
//! - opaque storage versions;
//! - storage metadata;
//! - conditional write semantics;
//! - streaming read and write interfaces;
//! - paginated object listing;
//! - an extensible storage-provider contract;
//! - a shareable storage-provider handle.
//!
//! ## Architectural ownership
//!
//! This module owns storage API contracts only.
//!
//! It does not own:
//!
//! - filesystem implementation;
//! - database implementation;
//! - object-store implementation;
//! - distributed storage placement;
//! - replication or consensus;
//! - physical memory allocation;
//! - device memory or DMA;
//! - hardware discovery;
//! - scheduling;
//! - tensor storage;
//! - canonical data semantics;
//! - compiler intermediate representations.
//!
//! Those responsibilities remain with their respective repository
//! authorities and registered providers.
//!
//! ## Integration
//!
//! Buffer ownership and byte views:
//!
//! `crate::stdlib::data::buffers`
//!
//! Standard-library errors:
//!
//! `crate::stdlib::error`
//!
//! Operation context:
//!
//! `crate::stdlib::context`
//!
//! Filesystem compatibility:
//!
//! `crate::stdlib::fs`
//!
//! Serialization compatibility:
//!
//! `crate::stdlib::serialize`
//!
//! Canonical tensor storage:
//!
//! `crate::classical::collections`
//!
//! Runtime and provider integration:
//!
//! `crate::runtime` and the applicable provider/HAL authorities.
//!
//! ## Scalability
//!
//! No arbitrary universal storage-size, object-count, key-count,
//! partition-count, replica-count, or provider-count limit is introduced.
//!
//! Streaming interfaces avoid requiring an entire stored object to fit
//! into one contiguous allocation.
//!
//! Paginated listing avoids requiring a provider to return every object
//! in one response.
//!
//! Actual limits remain determined by:
//!
//! - the provider's implementation;
//! - platform address-space and representation limits;
//! - available storage and memory;
//! - explicitly configured operational budgets;
//! - the capabilities and constraints of the execution environment.
//!
//! Physical resource limits must never be misrepresented as language
//! capability limits.
//!
//! ## Provider guarantees
//!
//! A provider must document its consistency, durability, visibility,
//! concurrency, and failure guarantees.
//!
//! This module does not silently assume that every provider offers
//! identical guarantees.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No global mutable registry.
//! No hidden provider selection.
//! No implicit retries.
//! No printing or logging.
//! No hardware-specific assumptions.

use core::fmt;
use core::num::NonZeroUsize;

use std::io::{Read, Write};
use std::sync::Arc;

use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// STORAGE KEY
// ============================================================================

/// An opaque logical identifier for a stored object.
///
/// A storage key is interpreted by the selected provider.
///
/// This type deliberately does not impose filesystem path semantics.
/// A provider may use keys as object identifiers, database keys,
/// logical paths, or another documented naming scheme.
///
/// Keys must not be empty.
///
/// No arbitrary key-length ceiling is imposed by this API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageKey(String);

impl StorageKey {
    /// Creates a storage key.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when the key is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(storage_error(
                ErrorKind::InvalidInput,
                "storage key must not be empty",
                "storage.key.create",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the key as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for StorageKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for StorageKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

// ============================================================================
// STORAGE VERSION
// ============================================================================

/// An opaque version or concurrency token supplied by a storage provider.
///
/// A version may represent an object generation, revision, entity tag,
/// transaction version, or another provider-defined identifier.
///
/// The value is not interpreted by this module.
///
/// Versions must not be assumed to be numeric, globally unique, ordered,
/// or portable between providers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageVersion(String);

impl StorageVersion {
    /// Creates a provider-defined version token.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when the token is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(storage_error(
                ErrorKind::InvalidInput,
                "storage version must not be empty",
                "storage.version.create",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque version token.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the version and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for StorageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// STORAGE METADATA
// ============================================================================

/// Metadata describing a stored object.
///
/// The byte length uses `u64` so that metadata can describe objects
/// larger than a single process can materialize as a contiguous buffer.
///
/// This does not promise that every provider or platform can represent
/// every possible `u64` length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageMetadata {
    /// Stored object length in bytes.
    pub length: u64,

    /// Provider-defined object version.
    pub version: StorageVersion,

    /// Optional media type or content-type description.
    pub content_type: Option<String>,
}

impl StorageMetadata {
    /// Creates storage metadata.
    #[must_use]
    pub fn new(
        length: u64,
        version: StorageVersion,
        content_type: Option<String>,
    ) -> Self {
        Self {
            length,
            version,
            content_type,
        }
    }
}

// ============================================================================
// WRITE CONDITIONS
// ============================================================================

/// A condition that a provider must evaluate when publishing a write.
///
/// Conditions must be checked atomically with the write operation.
/// A provider must not implement these conditions as an unsafe
/// check-then-write sequence that permits concurrent races.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WriteCondition {
    /// Write without requiring a particular existing version.
    ///
    /// The provider's documented overwrite behavior applies.
    Any,

    /// Create the object only if it does not already exist.
    MustNotExist,

    /// Replace the object only if its current version matches.
    MustMatch(StorageVersion),
}

// ============================================================================
// WRITE OPTIONS
// ============================================================================

/// Options controlling the publication of a stored object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageWriteOptions {
    /// Conditional write requirement.
    pub condition: WriteCondition,

    /// Optional content type.
    pub content_type: Option<String>,
}

impl Default for StorageWriteOptions {
    fn default() -> Self {
        Self {
            condition: WriteCondition::Any,
            content_type: None,
        }
    }
}

impl StorageWriteOptions {
    /// Creates default write options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requires that the object does not already exist.
    #[must_use]
    pub fn create_only(mut self) -> Self {
        self.condition = WriteCondition::MustNotExist;
        self
    }

    /// Requires the existing object's version to match.
    #[must_use]
    pub fn if_version_matches(
        mut self,
        version: StorageVersion,
    ) -> Self {
        self.condition = WriteCondition::MustMatch(version);
        self
    }

    /// Sets the content type.
    #[must_use]
    pub fn with_content_type(
        mut self,
        content_type: impl Into<String>,
    ) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

// ============================================================================
// STORAGE READER
// ============================================================================

/// A streaming reader for a stored object.
///
/// The reader exposes the metadata associated with the object version
/// from which the stream was opened.
///
/// Implementations must ensure that the stream follows the provider's
/// documented consistency contract.
///
/// A reader must not silently switch to another object version midway
/// through a stream unless that behavior is explicitly documented.
pub trait StorageReader: Read + Send {
    /// Returns metadata for the object version being read.
    fn metadata(&self) -> &StorageMetadata;
}

// ============================================================================
// STORAGE WRITER
// ============================================================================

/// A streaming writer for a stored object.
///
/// Writing bytes does not publish the object.
///
/// The object becomes visible according to the provider's documented
/// publication contract only after `commit` succeeds.
///
/// Implementations must ensure that:
///
/// - failed writes do not report successful publication;
/// - conditional write requirements are enforced;
/// - commit failures are reported;
/// - abort does not publish incomplete content;
/// - dropping an uncommitted writer does not report a successful commit.
///
/// Providers should use temporary objects, transactions, or equivalent
/// mechanisms when needed to satisfy their documented guarantees.
pub trait StorageWriter: Write + Send {
    /// Publishes the completed object and returns its metadata.
    ///
    /// This consumes the writer so that it cannot be committed twice.
    ///
    /// The provider must document the visibility and durability
    /// guarantees represented by a successful return.
    fn commit(self: Box<Self>) -> Result<StorageMetadata>;

    /// Explicitly abandons the pending write.
    ///
    /// This consumes the writer.
    ///
    /// The provider must not publish incomplete content as a successful
    /// result of this operation.
    fn abort(self: Box<Self>) -> Result<()>;
}

// ============================================================================
// PAGINATED LISTING
// ============================================================================

/// An opaque continuation token for paginated storage listing.
///
/// The token is generated and interpreted by the provider.
///
/// It must not be modified or interpreted by callers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoragePageToken(String);

impl StoragePageToken {
    /// Creates a continuation token.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when the token is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(storage_error(
                ErrorKind::InvalidInput,
                "storage page token must not be empty",
                "storage.page_token.create",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque continuation token.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the token and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Parameters for a paginated storage listing.
///
/// `page_size` is a caller-selected request, not a universal maximum.
/// Providers may reject unsupported page sizes according to their
/// documented operational policy.
///
/// A missing page size leaves the choice to the provider.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StorageListRequest {
    /// Optional logical key prefix.
    pub prefix: Option<StorageKey>,

    /// Optional requested number of entries in a page.
    pub page_size: Option<NonZeroUsize>,

    /// Optional provider-issued continuation token.
    pub continuation: Option<StoragePageToken>,
}

impl StorageListRequest {
    /// Creates an unrestricted listing request.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts the listing to a provider-defined key prefix.
    #[must_use]
    pub fn with_prefix(mut self, prefix: StorageKey) -> Self {
        self.prefix = Some(prefix);
        self
    }

    /// Requests a non-zero page size.
    #[must_use]
    pub fn with_page_size(mut self, page_size: NonZeroUsize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Continues a previous listing operation.
    #[must_use]
    pub fn with_continuation(
        mut self,
        token: StoragePageToken,
    ) -> Self {
        self.continuation = Some(token);
        self
    }
}

/// One object and its metadata in a storage listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageEntry {
    /// Logical object key.
    pub key: StorageKey,

    /// Metadata for the listed object.
    pub metadata: StorageMetadata,
}

/// One page of storage listing results.
///
/// `next` is absent when the provider reports that the listing is complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoragePage {
    /// Entries returned by the provider.
    pub entries: Vec<StorageEntry>,

    /// Provider-issued continuation token.
    pub next: Option<StoragePageToken>,
}

// ============================================================================
// STORAGE PROVIDER CONTRACT
// ============================================================================

/// Provider contract for persistent or externally managed storage.
///
/// Implementations may use:
///
/// - local filesystems;
/// - databases;
/// - object stores;
/// - distributed storage systems;
/// - embedded storage;
/// - remote storage services;
/// - future storage technologies.
///
/// This trait does not prescribe a storage vendor or physical topology.
///
/// # Concurrency
///
/// Implementations must be safe to share between threads.
///
/// Implementations must document their consistency and concurrency
/// guarantees.
///
/// # Conditional writes
///
/// `open_write` must enforce the supplied write condition atomically
/// when the write is committed. A condition conflict must be reported
/// as an error rather than silently overwritten.
///
/// # Streaming
///
/// Readers and writers must support incremental I/O. Implementations
/// must not require the complete object to be materialized in memory.
///
/// # Listing
///
/// Listing must honor the provider's documented prefix and continuation
/// semantics. Continuation tokens must be bound to the relevant listing
/// context according to the provider's contract.
pub trait StorageProvider: Send + Sync + 'static {
    /// Returns a stable descriptive provider identifier.
    ///
    /// This identifier is metadata, not proof of trust or availability.
    fn provider_id(&self) -> &str;

    /// Opens an object for streaming reads.
    ///
    /// # Errors
    ///
    /// Returns a structured standard-library error when the object
    /// cannot be opened or the provider cannot satisfy the operation.
    fn open_read(
        &self,
        key: &StorageKey,
    ) -> Result<Box<dyn StorageReader>>;

    /// Opens an object for streaming writes.
    ///
    /// The returned writer does not publish the object until `commit`
    /// succeeds.
    ///
    /// The supplied condition must be enforced atomically.
    fn open_write(
        &self,
        key: &StorageKey,
        options: &StorageWriteOptions,
    ) -> Result<Box<dyn StorageWriter>>;

    /// Returns metadata for an object, or `None` if it does not exist.
    fn metadata(
        &self,
        key: &StorageKey,
    ) -> Result<Option<StorageMetadata>>;

    /// Deletes an object.
    ///
    /// Returns `true` when an object was deleted and `false` when
    /// the provider confirms that no object existed.
    ///
    /// Provider-specific authorization and retention policies apply.
    fn delete(
        &self,
        key: &StorageKey,
    ) -> Result<bool>;

    /// Lists one page of objects.
    ///
    /// The provider controls the interpretation of key prefixes,
    /// continuation tokens, and page-size support.
    fn list(
        &self,
        request: &StorageListRequest,
    ) -> Result<StoragePage>;
}

// ============================================================================
// STORAGE HANDLE
// ============================================================================

/// A shareable handle to a registered storage-provider implementation.
///
/// Cloning this handle shares the provider through `Arc`.
///
/// This type does not discover, select, initialize, or register providers.
/// Those responsibilities belong to the registry and runtime integration.
#[derive(Clone)]
pub struct Storage {
    provider: Arc<dyn StorageProvider>,
}

impl Storage {
    /// Creates a storage handle from an existing provider.
    #[must_use]
    pub fn new<P>(provider: P) -> Self
    where
        P: StorageProvider,
    {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Creates a storage handle from a shared provider.
    #[must_use]
    pub fn from_shared(
        provider: Arc<dyn StorageProvider>,
    ) -> Self {
        Self { provider }
    }

    /// Returns the provider's descriptive identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.provider.provider_id()
    }

    /// Opens an object for streaming reads.
    pub fn open_read(
        &self,
        key: &StorageKey,
    ) -> Result<Box<dyn StorageReader>> {
        self.provider.open_read(key)
    }

    /// Opens an object for streaming writes.
    ///
    /// The caller must explicitly call `commit` to publish the object.
    pub fn open_write(
        &self,
        key: &StorageKey,
        options: &StorageWriteOptions,
    ) -> Result<Box<dyn StorageWriter>> {
        self.provider.open_write(key, options)
    }

    /// Returns metadata for an object, or `None` when absent.
    pub fn metadata(
        &self,
        key: &StorageKey,
    ) -> Result<Option<StorageMetadata>> {
        self.provider.metadata(key)
    }

    /// Deletes an object.
    pub fn delete(&self, key: &StorageKey) -> Result<bool> {
        self.provider.delete(key)
    }

    /// Lists one page of objects.
    pub fn list(
        &self,
        request: &StorageListRequest,
    ) -> Result<StoragePage> {
        self.provider.list(request)
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a standard-library storage error.
///
/// The standard-library error module remains the authority for error
/// classification, causality, context, and retryability.
fn storage_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> Error {
    Error::new(kind, message)
        .in_component("stdlib.data.storage")
        .during_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_key_rejects_empty_value() {
        let result = StorageKey::new("");

        assert!(result.is_err());

        let error = result.expect_err("empty key must fail");

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            error.component(),
            Some("stdlib.data.storage")
        );
    }

    #[test]
    fn storage_key_preserves_provider_defined_value() {
        let key = StorageKey::new("datasets/experiment/record-01")
            .expect("valid key");

        assert_eq!(
            key.as_str(),
            "datasets/experiment/record-01"
        );
    }

    #[test]
    fn storage_version_rejects_empty_value() {
        assert!(StorageVersion::new("").is_err());
    }

    #[test]
    fn storage_version_preserves_opaque_value() {
        let version = StorageVersion::new("revision:alpha-17")
            .expect("valid version");

        assert_eq!(version.as_str(), "revision:alpha-17");
    }

    #[test]
    fn default_write_options_allow_provider_defined_behavior() {
        let options = StorageWriteOptions::default();

        assert_eq!(options.condition, WriteCondition::Any);
        assert_eq!(options.content_type, None);
    }

    #[test]
    fn create_only_write_condition_is_preserved() {
        let options = StorageWriteOptions::new()
            .create_only()
            .with_content_type("application/octet-stream");

        assert_eq!(
            options.condition,
            WriteCondition::MustNotExist
        );

        assert_eq!(
            options.content_type.as_deref(),
            Some("application/octet-stream")
        );
    }

    #[test]
    fn version_condition_is_preserved() {
        let version = StorageVersion::new("v1")
            .expect("valid version");

        let options = StorageWriteOptions::new()
            .if_version_matches(version.clone());

        assert_eq!(
            options.condition,
            WriteCondition::MustMatch(version)
        );
    }

    #[test]
    fn listing_request_supports_provider_pagination() {
        let prefix = StorageKey::new("datasets/")
            .expect("valid prefix");

        let token = StoragePageToken::new("continuation-1")
            .expect("valid token");

        let request = StorageListRequest::new()
            .with_prefix(prefix.clone())
            .with_page_size(
                NonZeroUsize::new(128).expect("non-zero page size")
            )
            .with_continuation(token.clone());

        assert_eq!(request.prefix, Some(prefix));
        assert_eq!(
            request.page_size.map(NonZeroUsize::get),
            Some(128)
        );
        assert_eq!(request.continuation, Some(token));
    }

    #[test]
    fn listing_request_can_leave_page_size_to_provider() {
        let request = StorageListRequest::new();

        assert_eq!(request.page_size, None);
        assert_eq!(request.prefix, None);
        assert_eq!(request.continuation, None);
    }

    #[test]
    fn metadata_preserves_large_object_length() {
        let version = StorageVersion::new("generation-1")
            .expect("valid version");

        let metadata = StorageMetadata::new(
            u64::MAX,
            version,
            Some("application/octet-stream".to_owned()),
        );

        assert_eq!(metadata.length, u64::MAX);
    }

    #[test]
    fn page_can_represent_final_and_continuing_results() {
        let version = StorageVersion::new("v1")
            .expect("valid version");

        let entry = StorageEntry {
            key: StorageKey::new("records/one")
                .expect("valid key"),
            metadata: StorageMetadata::new(
                12,
                version,
                None,
            ),
        };

        let final_page = StoragePage {
            entries: vec![entry.clone()],
            next: None,
        };

        assert_eq!(final_page.entries.len(), 1);
        assert!(final_page.next.is_none());

        let continuing_page = StoragePage {
            entries: vec![entry],
            next: Some(
                StoragePageToken::new("next-page")
                    .expect("valid token")
            ),
        };

        assert!(continuing_page.next.is_some());
    }
}
