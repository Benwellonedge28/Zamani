//! # Zamani AST Node Metadata
//!
//! Production-grade metadata attached to native Zamani AST nodes.
//!
//! ## Architectural position
//!
//! This module belongs to the **native Zamani AST** and therefore deliberately
//! contains no knowledge of:
//!
//! - quantum hardware;
//! - quantum gates;
//! - QIR;
//! - OpenQASM;
//! - MLIR;
//! - LLVM;
//! - CPUs, GPUs, TPUs, FPGAs or QPUs;
//! - vendors;
//! - backend instruction sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - semantic symbol resolution;
//! - inferred types;
//! - target resources.
//!
//! Metadata describes information associated with the *source/AST representation*.
//! Meaning that depends on semantic analysis belongs in later compiler phases.
//!
//! ## Design goals
//!
//! This type is designed for:
//!
//! - arbitrary program size subject to externally configured resource limits;
//! - deterministic behavior;
//! - deterministic serialization;
//! - extensible metadata keys;
//! - namespaced metadata;
//! - source documentation;
//! - parser/tooling annotations;
//! - generated-node/tool provenance payloads without coupling to a particular
//!   provenance implementation;
//! - lossless preservation of metadata unknown to a consumer;
//! - safe cloning and sharing through ordinary Rust ownership;
//! - no `unsafe` code;
//! - no hidden machine-size limits.
//!
//! ## Important scalability rule
//!
//! This module intentionally does **not** define constants such as:
//!
//! ```text
//! MAX_METADATA_ITEMS
//! MAX_METADATA_BYTES
//! MAX_DOCUMENTATION_LENGTH
//! ```
//!
//! Such limits, when required for untrusted input, belong to an explicitly
//! configured compiler policy/validation layer.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   |
//!   v
//! native AST node
//!   |
//!   +--> NodeMetadata
//!   |
//!   v
//! structural validation
//!   |
//!   v
//! semantic analysis
//!   |
//!   v
//! Semantic Model
//!   |
//!   v
//! ZUIR
//! ```
//!
//! Metadata must survive AST transformations whenever the transformation's
//! contract says the resulting node preserves source/tooling provenance.
//!
//! Metadata must never be used as an implicit channel for backend semantics.
//!
//! ## Dependency contract
//!
//! This file depends only on the Rust standard library and `serde`.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum modules;
//! - hardware modules;
//! - optimization;
//! - scheduling;
//! - execution;
//! - runtime;
//! - backend implementations.
//!
//! The module is intentionally usable before the remainder of the AST tree is
//! implemented.
//!
//! ## Serialization contract
//!
//! `serde` derives are used for structural serialization. The metadata model
//! contains only explicitly represented data; no memory addresses, pointers,
//! process-local identifiers, or implementation-defined values are serialized.
//!
//! `BTreeMap` is used for arbitrary metadata objects so serialized key ordering
//! is deterministic.
//!
//! Schema/version policy belongs to the AST serialization layer rather than
//! this foundational data structure. The constant below identifies the local
//! metadata representation version and must not be confused with the language,
//! compiler, AST, or serialization protocol version.
//!
//! ## Thread-safety
//!
//! All contained types are `Send + Sync` when their contained `String`/standard
//! library values are, which they are. No interior mutable global state is used.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Version of the in-memory `NodeMetadata` data model.
///
/// This is deliberately separate from:
///
/// - Zamani language version;
/// - AST schema version;
/// - serialized-file version;
/// - compiler version;
/// - extension version.
///
/// Increment this only when the public metadata model itself changes in a
/// compatibility-relevant way.
pub const NODE_METADATA_SCHEMA_VERSION: u16 = 1;

/// A reserved namespace for metadata produced by Zamani's own frontend
/// tooling.
///
/// This is only an identifier convention. It does not grant semantic meaning
/// to metadata.
pub const ZAMANI_METADATA_NAMESPACE: &str = "zamani";

/// A namespace used for metadata supplied by external tools.
///
/// External tools should preferably use their own stable reverse-domain or
/// organization namespace rather than putting tool-specific data into the
/// `zamani` namespace.
pub const EXTERNAL_METADATA_NAMESPACE: &str = "tool";

/// A single metadata key.
///
/// Metadata keys are intentionally represented as a namespace plus name rather
/// than as a closed Rust enum. This allows future tooling and language
/// extensions to introduce metadata without modifying the AST core.
///
/// The key is immutable after construction.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetadataKey {
    namespace: String,
    name: String,
}

impl MetadataKey {
    /// Creates a metadata key.
    ///
    /// Both components must:
    ///
    /// - be non-empty;
    /// - contain no ASCII/control whitespace;
    /// - contain no `/`, `\\`, or `:`.
    ///
    /// No case folding or Unicode normalization is performed. The caller's
    /// exact spelling is preserved so metadata identity remains deterministic
    /// and lossless.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, MetadataKeyError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component("namespace", &namespace)?;
        validate_component("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the metadata namespace.
    #[inline]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the metadata name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical textual key.
    ///
    /// The returned value is:
    ///
    /// ```text
    /// namespace:name
    /// ```
    ///
    /// This allocates a `String`. Call `namespace()` and `name()` when allocation
    /// is unnecessary.
    pub fn canonical_name(&self) -> String {
        let mut result = String::with_capacity(
            self.namespace
                .len()
                .saturating_add(1)
                .saturating_add(self.name.len()),
        );

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }
}

impl fmt::Display for MetadataKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.namespace)?;
        formatter.write_str(":")?;
        formatter.write_str(&self.name)
    }
}

/// Errors produced when constructing an invalid metadata key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataKeyError {
    /// The namespace was empty.
    EmptyNamespace,

    /// The metadata name was empty.
    EmptyName,

    /// A namespace/name component contained a forbidden character.
    InvalidCharacter {
        /// Which component failed: `"namespace"` or `"name"`.
        component: &'static str,

        /// The offending character.
        character: char,
    },
}

impl fmt::Display for MetadataKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("metadata namespace must not be empty")
            }
            Self::EmptyName => formatter.write_str("metadata name must not be empty"),
            Self::InvalidCharacter {
                component,
                character,
            } => write!(
                formatter,
                "metadata {} contains forbidden character {:?}",
                component, character
            ),
        }
    }
}

impl std::error::Error for MetadataKeyError {}

fn validate_component(
    component: &'static str,
    value: &str,
) -> Result<(), MetadataKeyError> {
    if value.is_empty() {
        return Err(match component {
            "namespace" => MetadataKeyError::EmptyNamespace,
            _ => MetadataKeyError::EmptyName,
        });
    }

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
            || matches!(character, '/' | '\\' | ':')
        {
            return Err(MetadataKeyError::InvalidCharacter {
                component,
                character,
            });
        }
    }

    Ok(())
}

/// A deterministic, recursively extensible metadata value.
///
/// This is intentionally source/tool metadata rather than a semantic-value
/// representation.
///
/// Numeric values are represented using integer variants only. Floating-point
/// metadata is deliberately not represented here because IEEE-754 NaN and
/// signed-zero semantics make generic `Eq`/`Hash`/deterministic-value contracts
/// unnecessarily fragile.
///
/// Tools requiring exact floating-point information should store its canonical
/// textual representation as `Text`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetadataValue {
    /// Explicit absence of a value.
    Null,

    /// Boolean metadata.
    Boolean(bool),

    /// Signed integer metadata.
    Integer(i64),

    /// Arbitrary textual metadata.
    Text(String),

    /// Ordered metadata sequence.
    ///
    /// Ordering is preserved because order can itself be meaningful to tooling.
    List(Vec<Self>),

    /// Deterministically ordered metadata object.
    ///
    /// `BTreeMap` guarantees stable key ordering independent of hash-map
    /// randomization.
    Map(BTreeMap<String, Self>),
}

impl MetadataValue {
    /// Creates textual metadata.
    #[inline]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a metadata list.
    #[inline]
    pub fn list(values: Vec<Self>) -> Self {
        Self::List(values)
    }

    /// Creates a metadata object.
    #[inline]
    pub fn map(values: BTreeMap<String, Self>) -> Self {
        Self::Map(values)
    }

    /// Returns true when this value is `Null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns the contained string, if this is textual metadata.
    #[inline]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the contained integer, if this is integer metadata.
    #[inline]
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the contained boolean, if this is boolean metadata.
    #[inline]
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the contained list, if this is list metadata.
    #[inline]
    pub fn as_list(&self) -> Option<&[Self]> {
        match self {
            Self::List(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the contained map, if this is object metadata.
    #[inline]
    pub fn as_map(&self) -> Option<&BTreeMap<String, Self>> {
        match self {
            Self::Map(values) => Some(values),
            _ => None,
        }
    }
}

impl From<bool> for MetadataValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for MetadataValue {
    #[inline]
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for MetadataValue {
    #[inline]
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for MetadataValue {
    #[inline]
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// Source/tooling-oriented flags attached to an AST node.
///
/// Flags are deliberately broad and structural. They do not describe semantic
/// properties such as types, capabilities, effects or hardware requirements.
///
/// Additional information that cannot be represented by these flags belongs in
/// namespaced `MetadataKey`/`MetadataValue` entries.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetadataFlags {
    /// Node was generated rather than directly parsed from user source.
    generated: bool,

    /// Node was produced by a syntax transformation/desugaring step.
    transformed: bool,

    /// Node originated from an external/imported representation.
    imported: bool,

    /// Node contains or participates in recovered parser syntax.
    recovered: bool,

    /// Node is associated with documentation intended for tooling.
    documented: bool,
}

impl MetadataFlags {
    /// Creates empty flags.
    #[inline]
    pub const fn new() -> Self {
        Self {
            generated: false,
            transformed: false,
            imported: false,
            recovered: false,
            documented: false,
        }
    }

    /// Returns whether the node is generated.
    #[inline]
    pub const fn generated(self) -> bool {
        self.generated
    }

    /// Returns whether the node was transformed.
    #[inline]
    pub const fn transformed(self) -> bool {
        self.transformed
    }

    /// Returns whether the node was imported.
    #[inline]
    pub const fn imported(self) -> bool {
        self.imported
    }

    /// Returns whether the node was recovered from malformed source.
    #[inline]
    pub const fn recovered(self) -> bool {
        self.recovered
    }

    /// Returns whether documentation is attached.
    #[inline]
    pub const fn documented(self) -> bool {
        self.documented
    }

    /// Sets the generated flag.
    #[inline]
    pub const fn with_generated(mut self, value: bool) -> Self {
        self.generated = value;
        self
    }

    /// Sets the transformed flag.
    #[inline]
    pub const fn with_transformed(mut self, value: bool) -> Self {
        self.transformed = value;
        self
    }

    /// Sets the imported flag.
    #[inline]
    pub const fn with_imported(mut self, value: bool) -> Self {
        self.imported = value;
        self
    }

    /// Sets the recovered flag.
    #[inline]
    pub const fn with_recovered(mut self, value: bool) -> Self {
        self.recovered = value;
        self
    }

    /// Sets the documented flag.
    #[inline]
    pub const fn with_documented(mut self, value: bool) -> Self {
        self.documented = value;
        self
    }
}

/// Documentation associated with an AST node.
///
/// Documentation remains source/tooling data. It is not interpreted as a
/// semantic annotation by this module.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Documentation {
    /// Optional short summary.
    summary: Option<String>,

    /// Optional long-form documentation.
    body: Option<String>,
}

impl Documentation {
    /// Creates empty documentation.
    #[inline]
    pub const fn new() -> Self {
        Self {
            summary: None,
            body: None,
        }
    }

    /// Creates documentation containing only a summary.
    pub fn summary(value: impl Into<String>) -> Self {
        Self {
            summary: Some(value.into()),
            body: None,
        }
    }

    /// Creates documentation containing summary and body.
    pub fn new_with_body(
        summary: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            summary: Some(summary.into()),
            body: Some(body.into()),
        }
    }

    /// Returns the summary, if present.
    #[inline]
    pub fn summary_text(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// Returns the body, if present.
    #[inline]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    /// Returns true when no documentation is stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.summary.is_none() && self.body.is_none()
    }

    /// Sets/replaces the summary.
    pub fn set_summary(&mut self, value: impl Into<String>) {
        self.summary = Some(value.into());
    }

    /// Sets/replaces the body.
    pub fn set_body(&mut self, value: impl Into<String>) {
        self.body = Some(value.into());
    }

    /// Removes the summary and returns it.
    #[inline]
    pub fn take_summary(&mut self) -> Option<String> {
        self.summary.take()
    }

    /// Removes the body and returns it.
    #[inline]
    pub fn take_body(&mut self) -> Option<String> {
        self.body.take()
    }
}

/// Metadata attached to one native Zamani AST node.
///
/// `NodeMetadata` intentionally contains no `NodeId` or `Span`. Those belong
/// to the common node abstraction and source infrastructure respectively.
///
/// This separation prevents metadata from becoming a second, competing node
/// identity/source-location system.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Metadata schema version.
    schema_version: u16,

    /// Structural/tooling flags.
    flags: MetadataFlags,

    /// Optional source-level documentation.
    documentation: Option<Documentation>,

    /// Arbitrary namespaced metadata.
    ///
    /// A `BTreeMap` is used instead of `HashMap` to make iteration and
    /// serialization deterministic.
    entries: BTreeMap<MetadataKey, MetadataValue>,
}

impl NodeMetadata {
    /// Creates empty metadata using the current metadata schema version.
    #[inline]
    pub fn new() -> Self {
        Self {
            schema_version: NODE_METADATA_SCHEMA_VERSION,
            flags: MetadataFlags::new(),
            documentation: None,
            entries: BTreeMap::new(),
        }
    }

    /// Returns the metadata schema version.
    #[inline]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns metadata flags.
    #[inline]
    pub const fn flags(&self) -> MetadataFlags {
        self.flags
    }

    /// Returns mutable metadata flags.
    #[inline]
    pub fn flags_mut(&mut self) -> &mut MetadataFlags {
        &mut self.flags
    }

    /// Replaces metadata flags.
    #[inline]
    pub fn set_flags(&mut self, flags: MetadataFlags) {
        self.flags = flags;
    }

    /// Returns documentation.
    #[inline]
    pub fn documentation(&self) -> Option<&Documentation> {
        self.documentation.as_ref()
    }

    /// Returns mutable documentation.
    #[inline]
    pub fn documentation_mut(&mut self) -> Option<&mut Documentation> {
        self.documentation.as_mut()
    }

    /// Replaces documentation.
    #[inline]
    pub fn set_documentation(&mut self, documentation: Documentation) {
        self.documentation = Some(documentation);
        self.flags = self.flags.with_documented(true);
    }

    /// Removes and returns documentation.
    pub fn take_documentation(&mut self) -> Option<Documentation> {
        let documentation = self.documentation.take();

        if documentation.is_some() {
            self.flags = self.flags.with_documented(false);
        }

        documentation
    }

    /// Returns the number of arbitrary metadata entries.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no arbitrary metadata entries exist.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over metadata entries in deterministic key order.
    #[inline]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetadataKey, &MetadataValue)> {
        self.entries.iter()
    }

    /// Inserts metadata.
    ///
    /// Returns the previous value associated with the key, if one existed.
    ///
    /// Replacement is explicit and deterministic.
    pub fn insert(
        &mut self,
        key: MetadataKey,
        value: impl Into<MetadataValue>,
    ) -> Option<MetadataValue> {
        self.entries.insert(key, value.into())
    }

    /// Inserts metadata using namespace and name components.
    ///
    /// This is the preferred convenience API for parser/tool integrations.
    pub fn insert_named(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<MetadataValue>,
    ) -> Result<Option<MetadataValue>, MetadataKeyError> {
        let key = MetadataKey::new(namespace, name)?;
        Ok(self.insert(key, value))
    }

    /// Gets metadata by key.
    #[inline]
    pub fn get(&self, key: &MetadataKey) -> Option<&MetadataValue> {
        self.entries.get(key)
    }

    /// Gets metadata using namespace and name components.
    pub fn get_named(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<&MetadataValue>, MetadataKeyError> {
        let key = MetadataKey::new(namespace, name)?;
        Ok(self.get(&key))
    }

    /// Removes metadata by key.
    #[inline]
    pub fn remove(&mut self, key: &MetadataKey) -> Option<MetadataValue> {
        self.entries.remove(key)
    }

    /// Removes metadata using namespace and name components.
    pub fn remove_named(
        &mut self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<MetadataValue>, MetadataKeyError> {
        let key = MetadataKey::new(namespace, name)?;
        Ok(self.remove(&key))
    }

    /// Removes all arbitrary metadata.
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    /// Returns an iterator over all metadata belonging to one namespace.
    ///
    /// Since keys are ordered lexicographically, iteration is deterministic.
    pub fn namespace_entries<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = (&'a MetadataKey, &'a MetadataValue)> {
        self.entries
            .iter()
            .filter(move |(key, _)| key.namespace() == namespace)
    }

    /// Merges metadata from another node.
    ///
    /// Conflict policy:
    ///
    /// - flags are combined conservatively;
    /// - documentation from `other` replaces existing documentation when
    ///   present;
    /// - metadata entries from `other` replace entries with identical keys.
    ///
    /// This operation never performs semantic interpretation.
    pub fn merge(&mut self, other: &Self) {
        self.flags.generated |= other.flags.generated;
        self.flags.transformed |= other.flags.transformed;
        self.flags.imported |= other.flags.imported;
        self.flags.recovered |= other.flags.recovered;
        self.flags.documented |= other.flags.documented;

        if let Some(documentation) = &other.documentation {
            self.documentation = Some(documentation.clone());
        }

        self.entries
            .extend(other.entries.iter().map(|(key, value)| {
                (key.clone(), value.clone())
            }));
    }

    /// Consumes another metadata object and merges it without requiring a
    /// second clone of the incoming values.
    pub fn merge_owned(&mut self, other: Self) {
        self.flags.generated |= other.flags.generated;
        self.flags.transformed |= other.flags.transformed;
        self.flags.imported |= other.flags.imported;
        self.flags.recovered |= other.flags.recovered;
        self.flags.documented |= other.flags.documented;

        if other.documentation.is_some() {
            self.documentation = other.documentation;
        }

        self.entries.extend(other.entries);
    }

    /// Returns a deterministic iterator suitable for serialization, hashing
    /// and snapshot testing.
    #[inline]
    pub fn ordered_entries(
        &self,
    ) -> impl Iterator<Item = (&MetadataKey, &MetadataValue)> {
        self.entries.iter()
    }

    /// Returns whether this metadata contains anything that needs to be
    /// serialized beyond the schema version.
    #[inline]
    pub fn has_content(&self) -> bool {
        !self.entries.is_empty()
            || self.documentation.is_some()
            || self.flags != MetadataFlags::new()
    }
}

impl From<MetadataFlags> for NodeMetadata {
    fn from(flags: MetadataFlags) -> Self {
        let mut metadata = Self::new();
        metadata.flags = flags;
        metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_metadata_uses_current_schema() {
        let metadata = NodeMetadata::new();

        assert_eq!(
            metadata.schema_version(),
            NODE_METADATA_SCHEMA_VERSION
        );
        assert_eq!(metadata.len(), 0);
        assert!(metadata.documentation().is_none());
        assert!(!metadata.has_content());
    }

    #[test]
    fn metadata_keys_are_namespaced() {
        let key = MetadataKey::new("zamani", "test").expect("valid key");

        assert_eq!(key.namespace(), "zamani");
        assert_eq!(key.name(), "test");
        assert_eq!(key.canonical_name(), "zamani:test");
        assert_eq!(key.to_string(), "zamani:test");
    }

    #[test]
    fn invalid_metadata_keys_are_rejected() {
        assert_eq!(
            MetadataKey::new("", "name"),
            Err(MetadataKeyError::EmptyNamespace)
        );

        assert_eq!(
            MetadataKey::new("namespace", ""),
            Err(MetadataKeyError::EmptyName)
        );

        assert_eq!(
            MetadataKey::new("namespace:value", "name"),
            Err(MetadataKeyError::InvalidCharacter {
                component: "namespace",
                character: ':',
            })
        );

        assert_eq!(
            MetadataKey::new("namespace", "bad/name"),
            Err(MetadataKeyError::InvalidCharacter {
                component: "name",
                character: '/',
            })
        );
    }

    #[test]
    fn metadata_entries_are_deterministically_ordered() {
        let mut metadata = NodeMetadata::new();

        metadata
            .insert_named("z", "last", "3")
            .expect("valid key");

        metadata
            .insert_named("a", "first", "1")
            .expect("valid key");

        metadata
            .insert_named("m", "middle", "2")
            .expect("valid key");

        let names: Vec<String> = metadata
            .ordered_entries()
            .map(|(key, _)| key.canonical_name())
            .collect();

        assert_eq!(
            names,
            vec![
                "a:first".to_owned(),
                "m:middle".to_owned(),
                "z:last".to_owned(),
            ]
        );
    }

    #[test]
    fn metadata_replaces_existing_value_deterministically() {
        let mut metadata = NodeMetadata::new();

        let first = metadata
            .insert_named("zamani", "key", "first")
            .expect("valid key");

        assert!(first.is_none());

        let second = metadata
            .insert_named("zamani", "key", "second")
            .expect("valid key");

        assert_eq!(
            second,
            Some(MetadataValue::Text("first".to_owned()))
        );

        let value = metadata
            .get_named("zamani", "key")
            .expect("valid key")
            .expect("value exists");

        assert_eq!(value.as_text(), Some("second"));
    }

    #[test]
    fn documentation_sets_documented_flag() {
        let mut metadata = NodeMetadata::new();

        metadata.set_documentation(Documentation::summary("Example"));

        assert!(metadata.documentation().is_some());
        assert!(metadata.flags().documented());
        assert!(metadata.has_content());
    }

    #[test]
    fn taking_documentation_clears_documented_flag() {
        let mut metadata = NodeMetadata::new();

        metadata.set_documentation(Documentation::summary("Example"));
        assert!(metadata.flags().documented());

        let documentation = metadata.take_documentation();

        assert!(documentation.is_some());
        assert!(!metadata.flags().documented());
    }

    #[test]
    fn metadata_flags_are_independently_composable() {
        let flags = MetadataFlags::new()
            .with_generated(true)
            .with_transformed(true)
            .with_imported(true)
            .with_recovered(true)
            .with_documented(true);

        assert!(flags.generated());
        assert!(flags.transformed());
        assert!(flags.imported());
        assert!(flags.recovered());
        assert!(flags.documented());
    }

    #[test]
    fn merge_replaces_conflicting_entries() {
        let mut first = NodeMetadata::new();
        first
            .insert_named("tool", "mode", "old")
            .expect("valid key");

        let mut second = NodeMetadata::new();
        second
            .insert_named("tool", "mode", "new")
            .expect("valid key");

        second
            .insert_named("tool", "extra", true)
            .expect("valid key");

        first.merge(&second);

        assert_eq!(
            first
                .get_named("tool", "mode")
                .expect("valid key")
                .and_then(MetadataValue::as_text),
            Some("new")
        );

        assert_eq!(
            first
                .get_named("tool", "extra")
                .expect("valid key")
                .and_then(MetadataValue::as_boolean),
            Some(true)
        );
    }

    #[test]
    fn merge_combines_structural_flags() {
        let mut first = NodeMetadata::new();
        first.set_flags(
            MetadataFlags::new().with_generated(true),
        );

        let mut second = NodeMetadata::new();
        second.set_flags(
            MetadataFlags::new().with_imported(true),
        );

        first.merge(&second);

        assert!(first.flags().generated());
        assert!(first.flags().imported());
    }

    #[test]
    fn nested_metadata_values_are_supported() {
        let mut object = BTreeMap::new();

        object.insert(
            "kind".to_owned(),
            MetadataValue::Text("example".to_owned()),
        );

        object.insert(
            "enabled".to_owned(),
            MetadataValue::Boolean(true),
        );

        object.insert(
            "items".to_owned(),
            MetadataValue::List(vec![
                MetadataValue::Integer(1),
                MetadataValue::Integer(2),
                MetadataValue::Integer(3),
            ]),
        );

        let value = MetadataValue::Map(object);

        let map = value.as_map().expect("map");
        assert_eq!(
            map.get("kind").and_then(MetadataValue::as_text),
            Some("example")
        );
    }

    #[test]
    fn serialization_round_trip_preserves_metadata() {
        let mut metadata = NodeMetadata::new();

        metadata.set_flags(
            MetadataFlags::new()
                .with_generated(true)
                .with_transformed(true),
        );

        metadata.set_documentation(Documentation::new_with_body(
            "Summary",
            "Body",
        ));

        metadata
            .insert_named("zamani", "purpose", "testing")
            .expect("valid key");

        metadata
            .insert_named("tool", "enabled", true)
            .expect("valid key");

        let encoded =
            serde_json::to_string(&metadata).expect("serialize metadata");

        let decoded: NodeMetadata =
            serde_json::from_str(&encoded).expect("deserialize metadata");

        assert_eq!(decoded, metadata);
    }

    #[test]
    fn serialization_is_deterministic_for_equivalent_maps() {
        let mut first = NodeMetadata::new();
        first
            .insert_named("b", "second", "2")
            .expect("valid key");
        first
            .insert_named("a", "first", "1")
            .expect("valid key");

        let mut second = NodeMetadata::new();
        second
            .insert_named("a", "first", "1")
            .expect("valid key");
        second
            .insert_named("b", "second", "2")
            .expect("valid key");

        let first_json =
            serde_json::to_string(&first).expect("serialize first");

        let second_json =
            serde_json::to_string(&second).expect("serialize second");

        assert_eq!(first_json, second_json);
    }

    #[test]
    fn namespace_iteration_is_deterministic() {
        let mut metadata = NodeMetadata::new();

        metadata
            .insert_named("tool", "z", 3_i64)
            .expect("valid key");

        metadata
            .insert_named("zamani", "a", 1_i64)
            .expect("valid key");

        metadata
            .insert_named("tool", "a", 2_i64)
            .expect("valid key");

        let keys: Vec<String> = metadata
            .namespace_entries("tool")
            .map(|(key, _)| key.canonical_name())
            .collect();

        assert_eq!(
            keys,
            vec![
                "tool:a".to_owned(),
                "tool:z".to_owned(),
            ]
        );
    }

    #[test]
    fn empty_documentation_is_detected() {
        let documentation = Documentation::new();

        assert!(documentation.is_empty());
        assert!(documentation.summary_text().is_none());
        assert!(documentation.body().is_none());
    }
}