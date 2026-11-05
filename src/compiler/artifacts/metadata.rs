
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Artifact Metadata
//!
//! Canonical path: `src/compiler/artifacts/metadata.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the namespacing, typed access, validation, and merging
//! contracts for compiler artifact metadata.
//!
//! This module builds on the canonical `ArtifactMetadata` storage
//! defined in `artifact.rs`. It does not introduce another metadata
//! storage representation.
//!
//! ## OWNS
//!
//! - Metadata namespace identifiers.
//! - Metadata key identifiers.
//! - Namespaced key parsing and formatting.
//! - Typed JSON-value access over the canonical string storage.
//! - Configurable metadata validation policies.
//! - Structured metadata validation reports.
//! - Conflict-aware metadata merging.
//! - Metadata-specific errors.
//!
//! ## DOES NOT OWN
//!
//! - The underlying metadata storage.
//! - Artifact identity, kind, format, or payload.
//! - Artifact dependencies or manifest composition.
//! - Semantic types, effects, capabilities, or requirements.
//! - Source AST or grammar definitions.
//! - ZUIR or domain-specific IR.
//! - Portability classification.
//! - Artifact integrity verification.
//! - Cryptographic signing or authorization.
//! - Stable artifact wire serialization.
//! - External storage or runtime execution.
//!
//! ## CANONICAL STORAGE
//!
//! `artifact.rs` owns `ArtifactMetadata`, which stores ordered
//! string key/value pairs.
//!
//! This module operates on that type and never creates a competing
//! metadata map.
//!
//! ## SCALABILITY
//!
//! No universal limit is imposed on:
//!
//! - Metadata entry count.
//! - Namespace count.
//! - Namespace length.
//! - Key length.
//! - Value length.
//! - Artifact count.
//! - Hardware count.
//! - Device count.
//! - Resource count.
//!
//! Optional limits are explicit validation-policy settings.
//! They are operational safeguards, not language or artifact limits.
//!
//! Actual memory and processing capacity remain finite.
//!
//! ## DETERMINISM
//!
//! The canonical storage uses `BTreeMap`, so iteration is ordered.
//! This module does not claim to define the artifact wire format.
//! Stable serialization belongs to `serialization.rs`.
//!
//! ## INTEGRATION
//!
//! `artifact.rs`:
//!     Supplies the canonical ArtifactMetadata storage.
//!
//! `manifest.rs`:
//!     May use metadata validation when validating manifest metadata.
//!
//! `portability.rs`:
//!     May inspect descriptive metadata but owns portability decisions.
//!
//! `validation.rs`:
//!     May compose metadata validation with artifact validation.
//!
//! `serialization.rs`:
//!     Owns stable artifact encoding and decoding.
//!
//! `src/zuir/serialization/`:
//!     Remains the canonical ZUIR serialization subsystem.
//!
//! `grammar/`:
//!     Defines source syntax and semantic boundaries, not artifact metadata.
//!
//! No downstream module is required by this module.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::compiler::artifacts::artifact::ArtifactMetadata;

// ============================================================================
// Constants
// ============================================================================

/// Namespace reserved for metadata defined by the artifact subsystem.
///
/// This is a namespace convention, not a closed registry of all
/// namespaces that artifacts may use.
pub const ARTIFACT_METADATA_NAMESPACE: &str = "zamani.artifact";

/// Namespace reserved for metadata produced by compiler implementations.
pub const COMPILER_METADATA_NAMESPACE: &str = "zamani.compiler";

/// Namespace reserved for metadata schema information.
pub const METADATA_SCHEMA_NAMESPACE: &str = "zamani.metadata";

/// Current metadata contract version.
///
/// This is independent of:
/// - Zamani language version.
/// - Compiler release version.
/// - ZUIR version.
/// - Artifact format version.
/// - Backend version.
/// - Target/runtime version.
pub const METADATA_CONTRACT_VERSION: u32 = 1;

// ============================================================================
// Errors
// ============================================================================

/// Errors raised by metadata operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetadataError {
    /// A namespace or key is invalid.
    InvalidIdentifier {
        field: &'static str,
        value: String,
        reason: String,
    },

    /// A key cannot be represented in the namespaced storage format.
    InvalidStorageKey {
        value: String,
    },

    /// A metadata entry already exists with a different value.
    ConflictingValue {
        key: String,
    },

    /// A metadata value could not be serialized.
    Serialization {
        message: String,
    },

    /// A metadata value could not be deserialized.
    Deserialization {
        message: String,
    },
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                field,
                value,
                reason,
            } => {
                write!(
                    f,
                    "invalid metadata {field} `{value}`: {reason}"
                )
            }

            Self::InvalidStorageKey { value } => {
                write!(
                    f,
                    "invalid namespaced metadata storage key `{value}`"
                )
            }

            Self::ConflictingValue { key } => {
                write!(
                    f,
                    "conflicting metadata value for key `{key}`"
                )
            }

            Self::Serialization { message } => {
                write!(
                    f,
                    "metadata serialization failed: {message}"
                )
            }

            Self::Deserialization { message } => {
                write!(
                    f,
                    "metadata deserialization failed: {message}"
                )
            }
        }
    }
}

impl Error for MetadataError {}

/// Result type for metadata operations.
pub type MetadataResult<T> = Result<T, MetadataError>;

// ============================================================================
// Identifier validation
// ============================================================================

/// Validates a metadata identifier component.
///
/// Identifiers are preserved exactly as provided.
///
/// No arbitrary length limit is applied.
///
/// The colon character is reserved as the separator between
/// a metadata namespace and its key.
fn validate_component(
    value: &str,
    field: &'static str,
) -> MetadataResult<()> {
    if value.is_empty() {
        return Err(MetadataError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "value cannot be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(MetadataError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "leading or trailing whitespace is forbidden"
                .to_owned(),
        });
    }

    if value.contains(':') {
        return Err(MetadataError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "the colon character is reserved".to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(MetadataError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            reason: "control characters are forbidden".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// Namespace
// ============================================================================

/// An extensible metadata namespace.
///
/// Namespaces prevent unrelated subsystems from accidentally
/// assigning different meanings to the same metadata key.
///
/// Examples:
///
/// - `zamani.artifact`
/// - `zamani.compiler`
/// - `zamani.backend`
/// - `zamani.extension.example`
/// - `vendor.example.product`
///
/// These examples are conventions, not a closed inventory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataNamespace(String);

impl MetadataNamespace {
    /// Creates a validated namespace.
    pub fn new(value: impl Into<String>) -> MetadataResult<Self> {
        let value = value.into();

        validate_component(&value, "namespace")?;

        Ok(Self(value))
    }

    /// Returns the namespace identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns the artifact metadata namespace.
    pub fn artifact() -> Self {
        Self(ARTIFACT_METADATA_NAMESPACE.to_owned())
    }

    /// Returns the compiler metadata namespace.
    pub fn compiler() -> Self {
        Self(COMPILER_METADATA_NAMESPACE.to_owned())
    }
}

impl fmt::Display for MetadataNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for MetadataNamespace {
    type Error = MetadataError;

    fn try_from(value: &str) -> MetadataResult<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for MetadataNamespace {
    type Error = MetadataError;

    fn try_from(value: String) -> MetadataResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// Metadata key
// ============================================================================

/// A fully qualified metadata key.
///
/// Its canonical storage representation is:
///
/// `namespace:key`
///
/// The namespace and key are independently validated.
///
/// A namespace is not required to be registered in a central
/// enumeration. New extensions can introduce their own namespaces.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataKey {
    namespace: MetadataNamespace,
    name: String,
}

impl MetadataKey {
    /// Creates a namespaced metadata key.
    pub fn new(
        namespace: MetadataNamespace,
        name: impl Into<String>,
    ) -> MetadataResult<Self> {
        let name = name.into();

        validate_component(&name, "key")?;

        Ok(Self { namespace, name })
    }

    /// Parses a canonical `namespace:key` storage key.
    pub fn parse(value: &str) -> MetadataResult<Self> {
        let Some((namespace, name)) = value.split_once(':') else {
            return Err(MetadataError::InvalidStorageKey {
                value: value.to_owned(),
            });
        };

        if namespace.is_empty() || name.is_empty() {
            return Err(MetadataError::InvalidStorageKey {
                value: value.to_owned(),
            });
        }

        Self::new(
            MetadataNamespace::new(namespace.to_owned())?,
            name.to_owned(),
        )
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &MetadataNamespace {
        &self.namespace
    }

    /// Returns the unqualified key name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical storage representation.
    pub fn storage_key(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for MetadataKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

// ============================================================================
// Typed metadata access
// ============================================================================

/// Provides typed access over the canonical string-based metadata store.
///
/// Structured values are encoded as JSON strings.
///
/// This is an access convention only. The artifact serialization
/// subsystem remains responsible for the stable artifact wire format.
pub struct MetadataAccess;

impl MetadataAccess {
    /// Inserts or replaces a string value.
    pub fn insert_text(
        metadata: &mut ArtifactMetadata,
        key: &MetadataKey,
        value: impl Into<String>,
    ) -> MetadataResult<Option<String>> {
        metadata
            .insert(key.storage_key(), value.into())
            .map_err(|error| MetadataError::InvalidIdentifier {
                field: "storage key",
                value: key.storage_key(),
                reason: error.to_string(),
            })
    }

    /// Retrieves a string value.
    pub fn get_text<'a>(
        metadata: &'a ArtifactMetadata,
        key: &MetadataKey,
    ) -> Option<&'a str> {
        metadata.get(&key.storage_key())
    }

    /// Inserts a JSON-compatible value.
    ///
    /// The JSON string is stored in the existing canonical
    /// ArtifactMetadata map.
    pub fn insert_json(
        metadata: &mut ArtifactMetadata,
        key: &MetadataKey,
        value: &serde_json::Value,
    ) -> MetadataResult<Option<String>> {
        let encoded = serde_json::to_string(value).map_err(|error| {
            MetadataError::Serialization {
                message: error.to_string(),
            }
        })?;

        Self::insert_text(metadata, key, encoded)
    }

    /// Retrieves and parses a JSON-compatible value.
    pub fn get_json(
        metadata: &ArtifactMetadata,
        key: &MetadataKey,
    ) -> MetadataResult<Option<serde_json::Value>> {
        let Some(value) = Self::get_text(metadata, key) else {
            return Ok(None);
        };

        let parsed = serde_json::from_str(value).map_err(|error| {
            MetadataError::Deserialization {
                message: error.to_string(),
            }
        })?;

        Ok(Some(parsed))
    }

    /// Removes a namespaced metadata value.
    pub fn remove(
        metadata: &mut ArtifactMetadata,
        key: &MetadataKey,
    ) -> Option<String> {
        metadata.remove(&key.storage_key())
    }

    /// Checks whether a namespaced metadata value exists.
    pub fn contains(
        metadata: &ArtifactMetadata,
        key: &MetadataKey,
    ) -> bool {
        metadata.get(&key.storage_key()).is_some()
    }
}

// ============================================================================
// Validation policy
// ============================================================================

/// Explicit validation policy for artifact metadata.
///
/// Every limit is optional.
///
/// A missing limit means this module imposes no additional
/// restriction on that dimension.
///
/// These limits are operational safeguards and must not be
/// interpreted as universal language or hardware limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataValidationPolicy {
    max_entries: Option<usize>,
    max_key_bytes: Option<usize>,
    max_value_bytes: Option<usize>,
    allowed_namespaces: Option<BTreeSet<MetadataNamespace>>,
    require_namespaced_keys: bool,
}

impl MetadataValidationPolicy {
    /// Creates an unrestricted policy.
    pub fn unrestricted() -> Self {
        Self {
            max_entries: None,
            max_key_bytes: None,
            max_value_bytes: None,
            allowed_namespaces: None,
            require_namespaced_keys: false,
        }
    }

    /// Sets an optional metadata-entry budget.
    #[must_use]
    pub fn with_max_entries(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_entries = limit;
        self
    }

    /// Sets an optional storage-key byte budget.
    #[must_use]
    pub fn with_max_key_bytes(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_key_bytes = limit;
        self
    }

    /// Sets an optional value byte budget.
    #[must_use]
    pub fn with_max_value_bytes(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_value_bytes = limit;
        self
    }

    /// Restricts metadata to an explicitly supplied namespace set.
    ///
    /// `None` means that namespaces are unrestricted.
    ///
    /// An empty set means that no namespaced metadata is permitted.
    #[must_use]
    pub fn with_allowed_namespaces(
        mut self,
        namespaces: Option<BTreeSet<MetadataNamespace>>,
    ) -> Self {
        self.allowed_namespaces = namespaces;
        self
    }

    /// Requires every metadata key to use `namespace:key` form.
    #[must_use]
    pub fn requiring_namespaced_keys(
        mut self,
        required: bool,
    ) -> Self {
        self.require_namespaced_keys = required;
        self
    }
}

impl Default for MetadataValidationPolicy {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// ============================================================================
// Validation findings
// ============================================================================

/// Severity of a metadata validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataIssueSeverity {
    /// The finding does not invalidate metadata under the active policy.
    Warning,

    /// The finding violates the active policy.
    Error,
}

/// Classification of a metadata validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetadataIssueKind {
    /// A legacy or unqualified key was encountered.
    UnnamespacedKey,

    /// A namespaced key is malformed.
    InvalidNamespacedKey,

    /// The metadata entry count exceeds the configured budget.
    EntryBudgetExceeded,

    /// A key exceeds the configured byte budget.
    KeyBudgetExceeded,

    /// A value exceeds the configured byte budget.
    ValueBudgetExceeded,

    /// A namespace is not in the configured allow-list.
    NamespaceNotAllowed,
}

/// One metadata validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataIssue {
    severity: MetadataIssueSeverity,
    kind: MetadataIssueKind,
    key: Option<String>,
    message: String,
}

impl MetadataIssue {
    fn new(
        severity: MetadataIssueSeverity,
        kind: MetadataIssueKind,
        key: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            kind,
            key,
            message: message.into(),
        }
    }

    /// Returns the finding severity.
    pub const fn severity(&self) -> MetadataIssueSeverity {
        self.severity
    }

    /// Returns the finding classification.
    pub fn kind(&self) -> &MetadataIssueKind {
        &self.kind
    }

    /// Returns the affected storage key, if available.
    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    /// Returns the explanatory message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// Validation report
// ============================================================================

/// Result of validating canonical artifact metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MetadataValidationReport {
    checked_entries: usize,
    issues: Vec<MetadataIssue>,
}

impl MetadataValidationReport {
    /// Returns the number of entries examined.
    pub const fn checked_entries(&self) -> usize {
        self.checked_entries
    }

    /// Returns all findings in deterministic traversal order.
    pub fn issues(&self) -> &[MetadataIssue] {
        &self.issues
    }

    /// Returns whether the metadata satisfies the active policy.
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity() == MetadataIssueSeverity::Error)
    }

    /// Returns the number of errors.
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity() == MetadataIssueSeverity::Error
            })
            .count()
    }

    /// Returns the number of warnings.
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity() == MetadataIssueSeverity::Warning
            })
            .count()
    }
}

// ============================================================================
// Metadata validation
// ============================================================================

/// Validates metadata using an explicit policy.
///
/// Validation does not modify metadata.
///
/// Existing unqualified metadata keys remain compatible by default.
/// They become errors only when the caller requires namespaced keys.
///
/// Unknown namespaces are accepted unless an explicit allow-list
/// has been configured.
pub fn validate_metadata(
    metadata: &ArtifactMetadata,
    policy: &MetadataValidationPolicy,
) -> MetadataValidationReport {
    let mut report = MetadataValidationReport {
        checked_entries: metadata.len(),
        issues: Vec::new(),
    };

    if let Some(limit) = policy.max_entries {
        if metadata.len() > limit {
            report.issues.push(MetadataIssue::new(
                MetadataIssueSeverity::Error,
                MetadataIssueKind::EntryBudgetExceeded,
                None,
                format!(
                    "metadata contains {} entries; configured budget is {limit}",
                    metadata.len()
                ),
            ));
        }
    }

    for (storage_key, value) in metadata.iter() {
        if let Some(limit) = policy.max_key_bytes {
            if storage_key.len() > limit {
                report.issues.push(MetadataIssue::new(
                    MetadataIssueSeverity::Error,
                    MetadataIssueKind::KeyBudgetExceeded,
                    Some(storage_key.to_owned()),
                    format!(
                        "metadata key uses {} bytes; configured budget is {limit}",
                        storage_key.len()
                    ),
                ));
            }
        }

        if let Some(limit) = policy.max_value_bytes {
            if value.len() > limit {
                report.issues.push(MetadataIssue::new(
                    MetadataIssueSeverity::Error,
                    MetadataIssueKind::ValueBudgetExceeded,
                    Some(storage_key.to_owned()),
                    format!(
                        "metadata value uses {} bytes; configured budget is {limit}",
                        value.len()
                    ),
                ));
            }
        }

        let parsed = match MetadataKey::parse(storage_key) {
            Ok(key) => key,

            Err(_) => {
                let severity = if policy.require_namespaced_keys {
                    MetadataIssueSeverity::Error
                } else {
                    MetadataIssueSeverity::Warning
                };

                report.issues.push(MetadataIssue::new(
                    severity,
                    MetadataIssueKind::UnnamespacedKey,
                    Some(storage_key.to_owned()),
                    "metadata key is not in namespace:key form",
                ));

                continue;
            }
        };

        if let Some(allowed) = &policy.allowed_namespaces {
            if !allowed.contains(parsed.namespace()) {
                report.issues.push(MetadataIssue::new(
                    MetadataIssueSeverity::Error,
                    MetadataIssueKind::NamespaceNotAllowed,
                    Some(storage_key.to_owned()),
                    format!(
                        "namespace `{}` is not permitted by the active policy",
                        parsed.namespace()
                    ),
                ));
            }
        }
    }

    report
}

// ============================================================================
// Metadata merge
// ============================================================================

/// Defines conflict behavior when combining metadata maps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataMergeStrategy {
    /// Reject the merge when the same key has different values.
    RejectConflicts,

    /// Preserve the destination value when a conflict occurs.
    KeepExisting,

    /// Replace the destination value with the incoming value.
    ReplaceExisting,
}

/// Summary of a metadata merge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MetadataMergeReport {
    inserted: usize,
    replaced: usize,
    retained: usize,
}

impl MetadataMergeReport {
    /// Returns the number of new entries.
    pub const fn inserted(&self) -> usize {
        self.inserted
    }

    /// Returns the number of replaced entries.
    pub const fn replaced(&self) -> usize {
        self.replaced
    }

    /// Returns the number of existing entries retained.
    pub const fn retained(&self) -> usize {
        self.retained
    }
}

/// Merges source metadata into destination metadata.
///
/// The operation is deterministic because both metadata maps
/// iterate in key order.
///
/// Under `RejectConflicts`, the destination is not modified if
/// a conflicting value is found.
///
/// No entry-count or value-size ceiling is imposed here.
pub fn merge_metadata(
    destination: &mut ArtifactMetadata,
    source: &ArtifactMetadata,
    strategy: MetadataMergeStrategy,
) -> MetadataResult<MetadataMergeReport> {
    // Validate all conflicts before mutation. This gives
    // RejectConflicts transactional behavior.
    if strategy == MetadataMergeStrategy::RejectConflicts {
        for (key, incoming) in source.iter() {
            if let Some(existing) = destination.get(key) {
                if existing != incoming {
                    return Err(MetadataError::ConflictingValue {
                        key: key.to_owned(),
                    });
                }
            }
        }
    }

    let mut report = MetadataMergeReport::default();

    for (key, incoming) in source.iter() {
        match destination.get(key) {
            None => {
                destination
                    .insert(key.to_owned(), incoming.to_owned())
                    .map_err(|error| MetadataError::InvalidIdentifier {
                        field: "storage key",
                        value: key.to_owned(),
                        reason: error.to_string(),
                    })?;

                report.inserted = report.inserted.saturating_add(1);
            }

            Some(existing) if existing == incoming => {
                report.retained = report.retained.saturating_add(1);
            }

            Some(_) => match strategy {
                MetadataMergeStrategy::RejectConflicts => {
                    // All conflicts were checked before mutation.
                    return Err(MetadataError::ConflictingValue {
                        key: key.to_owned(),
                    });
                }

                MetadataMergeStrategy::KeepExisting => {
                    report.retained = report.retained.saturating_add(1);
                }

                MetadataMergeStrategy::ReplaceExisting => {
                    destination
                        .insert(key.to_owned(), incoming.to_owned())
                        .map_err(|error| MetadataError::InvalidIdentifier {
                            field: "storage key",
                            value: key.to_owned(),
                            reason: error.to_string(),
                        })?;

                    report.replaced = report.replaced.saturating_add(1);
                }
            },
        }
    }

    Ok(report)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(namespace: &str, name: &str) -> MetadataKey {
        MetadataKey::new(
            MetadataNamespace::new(namespace).expect("valid namespace"),
            name,
        )
        .expect("valid metadata key")
    }

    #[test]
    fn namespace_and_key_round_trip() {
        let original = key("zamani.compiler", "optimization");

        let encoded = original.storage_key();
        let decoded = MetadataKey::parse(&encoded)
            .expect("valid storage key");

        assert_eq!(original, decoded);
    }

    #[test]
    fn rejects_empty_namespace() {
        assert!(MetadataNamespace::new("").is_err());
    }

    #[test]
    fn rejects_empty_key() {
        let namespace = MetadataNamespace::new("zamani.compiler")
            .expect("valid namespace");

        assert!(MetadataKey::new(namespace, "").is_err());
    }

    #[test]
    fn rejects_reserved_separator_inside_components() {
        assert!(MetadataNamespace::new("zamani:compiler").is_err());

        let namespace = MetadataNamespace::new("zamani.compiler")
            .expect("valid namespace");

        assert!(MetadataKey::new(namespace, "build:mode").is_err());
    }

    #[test]
    fn supports_future_namespaces() {
        let future = key(
            "future.photonic.computing",
            "realization",
        );

        assert_eq!(
            future.storage_key(),
            "future.photonic.computing:realization"
        );
    }

    #[test]
    fn typed_text_access_uses_canonical_storage() {
        let mut metadata = ArtifactMetadata::new();

        let metadata_key = key(
            ARTIFACT_METADATA_NAMESPACE,
            "description",
        );

        MetadataAccess::insert_text(
            &mut metadata,
            &metadata_key,
            "Portable artifact",
        )
        .expect("insert succeeds");

        assert_eq!(
            MetadataAccess::get_text(&metadata, &metadata_key),
            Some("Portable artifact")
        );

        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn json_values_round_trip() {
        let mut metadata = ArtifactMetadata::new();

        let metadata_key = key(
            ARTIFACT_METADATA_NAMESPACE,
            "custom-properties",
        );

        let original = serde_json::json!({
            "enabled": true,
            "revision": 7,
            "description": "future extension"
        });

        MetadataAccess::insert_json(
            &mut metadata,
            &metadata_key,
            &original,
        )
        .expect("JSON insertion succeeds");

        let restored = MetadataAccess::get_json(
            &metadata,
            &metadata_key,
        )
        .expect("JSON retrieval succeeds")
        .expect("value exists");

        assert_eq!(restored, original);
    }

    #[test]
    fn missing_json_value_returns_none() {
        let metadata = ArtifactMetadata::new();

        let metadata_key = key(
            ARTIFACT_METADATA_NAMESPACE,
            "missing",
        );

        assert_eq!(
            MetadataAccess::get_json(&metadata, &metadata_key)
                .expect("lookup succeeds"),
            None
        );
    }

    #[test]
    fn default_policy_accepts_legacy_keys() {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert("legacy-key", "legacy-value")
            .expect("legacy key is accepted by canonical storage");

        let report = validate_metadata(
            &metadata,
            &MetadataValidationPolicy::default(),
        );

        assert!(report.is_valid());
        assert_eq!(report.warning_count(), 1);
    }

    #[test]
    fn strict_policy_rejects_legacy_keys() {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert("legacy-key", "legacy-value")
            .expect("legacy key is accepted by canonical storage");

        let policy = MetadataValidationPolicy::unrestricted()
            .requiring_namespaced_keys(true);

        let report = validate_metadata(&metadata, &policy);

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn validates_entry_budget() {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert("one", "1")
            .expect("insert succeeds");

        metadata
            .insert("two", "2")
            .expect("insert succeeds");

        let policy = MetadataValidationPolicy::unrestricted()
            .with_max_entries(Some(1));

        let report = validate_metadata(&metadata, &policy);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == &MetadataIssueKind::EntryBudgetExceeded
        }));
    }

    #[test]
    fn validates_value_byte_budget() {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert("description", "a longer description")
            .expect("insert succeeds");

        let policy = MetadataValidationPolicy::unrestricted()
            .with_max_value_bytes(Some(5));

        let report = validate_metadata(&metadata, &policy);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == &MetadataIssueKind::ValueBudgetExceeded
        }));
    }

    #[test]
    fn validates_namespace_allow_list() {
        let mut metadata = ArtifactMetadata::new();

        let metadata_key = key("zamani.compiler", "mode");

        MetadataAccess::insert_text(
            &mut metadata,
            &metadata_key,
            "release",
        )
        .expect("insert succeeds");

        let allowed = BTreeSet::from([
            MetadataNamespace::new("zamani.artifact")
                .expect("valid namespace"),
        ]);

        let policy = MetadataValidationPolicy::unrestricted()
            .with_allowed_namespaces(Some(allowed));

        let report = validate_metadata(&metadata, &policy);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == &MetadataIssueKind::NamespaceNotAllowed
        }));
    }

    #[test]
    fn merge_inserts_new_entries() {
        let mut destination = ArtifactMetadata::new();
        let mut source = ArtifactMetadata::new();

        destination
            .insert("existing", "one")
            .expect("insert succeeds");

        source
            .insert("new", "two")
            .expect("insert succeeds");

        let report = merge_metadata(
            &mut destination,
            &source,
            MetadataMergeStrategy::RejectConflicts,
        )
        .expect("merge succeeds");

        assert_eq!(report.inserted(), 1);
        assert_eq!(destination.get("new"), Some("two"));
    }

    #[test]
    fn reject_conflicts_does_not_partially_mutate_destination() {
        let mut destination = ArtifactMetadata::new();
        let mut source = ArtifactMetadata::new();

        destination
            .insert("conflict", "old")
            .expect("insert succeeds");

        source
            .insert("another", "new")
            .expect("insert succeeds");

        source
            .insert("conflict", "different")
            .expect("insert succeeds");

        let original = destination.clone();

        let result = merge_metadata(
            &mut destination,
            &source,
            MetadataMergeStrategy::RejectConflicts,
        );

        assert!(result.is_err());
        assert_eq!(destination, original);
    }

    #[test]
    fn keep_existing_preserves_conflicting_values() {
        let mut destination = ArtifactMetadata::new();
        let mut source = ArtifactMetadata::new();

        destination
            .insert("mode", "debug")
            .expect("insert succeeds");

        source
            .insert("mode", "release")
            .expect("insert succeeds");

        let report = merge_metadata(
            &mut destination,
            &source,
            MetadataMergeStrategy::KeepExisting,
        )
        .expect("merge succeeds");

        assert_eq!(report.retained(), 1);
        assert_eq!(destination.get("mode"), Some("debug"));
    }

    #[test]
    fn replace_existing_replaces_conflicting_values() {
        let mut destination = ArtifactMetadata::new();
        let mut source = ArtifactMetadata::new();

        destination
            .insert("mode", "debug")
            .expect("insert succeeds");

        source
            .insert("mode", "release")
            .expect("insert succeeds");

        let report = merge_metadata(
            &mut destination,
            &source,
            MetadataMergeStrategy::ReplaceExisting,
        )
        .expect("merge succeeds");

        assert_eq!(report.replaced(), 1);
        assert_eq!(destination.get("mode"), Some("release"));
    }

    #[test]
    fn no_fixed_identifier_length_is_imposed() {
        let long_namespace = format!(
            "future.{}",
            "extension".repeat(512)
        );

        let namespace = MetadataNamespace::new(long_namespace.clone())
            .expect("no arbitrary namespace length limit");

        let metadata_key = MetadataKey::new(
            namespace,
            "descriptor",
        )
        .expect("valid metadata key");

        assert!(metadata_key.storage_key().starts_with(&long_namespace));
    }
}
