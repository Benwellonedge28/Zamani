
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Serialization
//!
//! Path: `src/stdlib/data/serialization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, provider-independent serialization and
//! deserialization contracts.
//!
//! The module supports:
//!
//! - open-world format identifiers;
//! - schema identifiers and version references;
//! - extensible serialization options;
//! - streaming encoding and decoding;
//! - format-provider registration and lookup;
//! - thread-safe provider sharing;
//! - structured operation reports;
//! - integration with the standard-library error system.
//!
//! ## Architectural ownership
//!
//! This module owns serialization API contracts and provider dispatch.
//!
//! It does not own:
//!
//! - the Zamani AST;
//! - semantic type definitions;
//! - schema semantics;
//! - canonical data representations;
//! - tensor storage;
//! - quantum state semantics;
//! - HDL or hardware representations;
//! - filesystem implementation;
//! - database implementation;
//! - storage placement;
//! - distributed replication;
//! - compression algorithms;
//! - cryptographic algorithms;
//! - hardware selection;
//! - scheduling;
//! - runtime execution.
//!
//! Those responsibilities remain with their canonical repository
//! authorities and registered providers.
//!
//! ## Streaming and scalability
//!
//! The provider interface uses `Read` and `Write`.
//!
//! A codec can process arbitrarily long streams incrementally,
//! subject to the limits of its implementation and available resources.
//!
//! This module introduces no universal:
//!
//! - input-size limit;
//! - output-size limit;
//! - schema-count limit;
//! - format-count limit;
//! - provider-count limit;
//! - field-count limit;
//! - record-count limit;
//! - tensor-rank limit;
//! - device-count limit.
//!
//! A platform's address space, allocator, storage, and operational
//! budgets remain real constraints. They are not language-level limits.
//!
//! ## Integration
//!
//! Standard-library errors:
//!
//! `crate::stdlib::error`
//!
//! Byte buffers:
//!
//! `crate::stdlib::data::buffers`
//!
//! Storage:
//!
//! `crate::stdlib::data::storage`
//!
//! Legacy serialization compatibility:
//!
//! `crate::stdlib::serialize`
//!
//! Canonical tensor and domain representations:
//!
//! `crate::classical::collections`,
//! `crate::quantum`, `crate::hdl`, `crate::ai`
//!
//! Grammar authority:
//!
//! `grammar/data/serialization.g4`
//!
//! The grammar defines source-level serialization intent.
//! This module implements runtime-facing codec contracts.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No hidden provider selection.
//! No implicit retries.
//! No global mutable registry.
//! No logging or printing.
//! No placeholder success results.

use core::fmt;

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};

use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// An open-world serialization format identifier.
///
/// The identifier is interpreted by a registered provider.
///
/// Examples include `json`, `cbor`, `protobuf`, and
/// organization-specific format identifiers.
///
/// This type does not enumerate the supported formats.
///
/// Empty identifiers are rejected. No universal identifier-length
/// ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SerializationFormatId(String);

impl SerializationFormatId {
    /// Creates a serialization format identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(serialization_error(
                ErrorKind::InvalidInput,
                "serialization format identifier must not be empty",
                "serialization.format_id.create",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SerializationFormatId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for SerializationFormatId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SerializationFormatId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

// ============================================================================
// SCHEMA IDENTIFIERS
// ============================================================================

/// An open-world schema identifier.
///
/// The identifier is descriptive and is interpreted by the applicable
/// schema authority or serialization provider.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaId(String);

impl SchemaId {
    /// Creates a schema identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(serialization_error(
                ErrorKind::InvalidInput,
                "schema identifier must not be empty",
                "serialization.schema_id.create",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SchemaId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A schema reference used by a serialization operation.
///
/// Schema version identity is opaque. It is not assumed to be numeric,
/// ordered, or globally comparable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaReference {
    /// Schema identity.
    pub id: SchemaId,

    /// Optional provider- or schema-authority-defined version.
    pub version: Option<String>,
}

impl SchemaReference {
    /// Creates a schema reference without a specified version.
    #[must_use]
    pub fn new(id: SchemaId) -> Self {
        Self {
            id,
            version: None,
        }
    }

    /// Adds an opaque schema version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

// ============================================================================
// FORMAT DESCRIPTOR
// ============================================================================

/// Describes a serialization format provider.
///
/// These fields describe provider claims. They do not independently
/// establish that a provider is correct, secure, or trustworthy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatDescriptor {
    /// Open-world format identifier.
    pub id: SerializationFormatId,

    /// Human-readable format name.
    pub display_name: String,

    /// Media types understood by the provider.
    pub media_types: Vec<String>,

    /// Whether the provider supports incremental streaming.
    pub streaming: bool,

    /// Whether the provider supports schema-aware operations.
    pub schema_aware: bool,

    /// Whether the provider claims deterministic encoding when
    /// given equivalent inputs and equivalent options.
    pub deterministic_encoding: bool,
}

impl FormatDescriptor {
    /// Creates a format descriptor.
    #[must_use]
    pub fn new(
        id: SerializationFormatId,
        display_name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            media_types: Vec::new(),
            streaming: false,
            schema_aware: false,
            deterministic_encoding: false,
        }
    }

    /// Adds a supported media type.
    #[must_use]
    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_types.push(media_type.into());
        self
    }

    /// Declares incremental streaming support.
    #[must_use]
    pub fn with_streaming(mut self, enabled: bool) -> Self {
        self.streaming = enabled;
        self
    }

    /// Declares schema-awareness.
    #[must_use]
    pub fn with_schema_support(mut self, enabled: bool) -> Self {
        self.schema_aware = enabled;
        self
    }

    /// Declares deterministic encoding behavior.
    #[must_use]
    pub fn with_deterministic_encoding(mut self, enabled: bool) -> Self {
        self.deterministic_encoding = enabled;
        self
    }
}

// ============================================================================
// SERIALIZATION OPTIONS
// ============================================================================

/// Options supplied to a serialization or deserialization provider.
///
/// The `properties` map is an extension mechanism for format-specific
/// options. A provider must document the properties it recognizes.
///
/// Unknown properties must not silently change the meaning of an
/// operation. Providers should reject unsupported mandatory properties.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SerializationOptions {
    /// Optional schema reference.
    pub schema: Option<SchemaReference>,

    /// Whether the provider should use its documented canonical
    /// representation, if it supports one.
    pub canonical: bool,

    /// Open-world provider-specific options.
    pub properties: BTreeMap<String, String>,
}

impl SerializationOptions {
    /// Creates default serialization options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Supplies a schema reference.
    #[must_use]
    pub fn with_schema(mut self, schema: SchemaReference) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Requests canonical representation where supported.
    #[must_use]
    pub fn with_canonical_representation(mut self, enabled: bool) -> Self {
        self.canonical = enabled;
        self
    }

    /// Adds a provider-specific property.
    ///
    /// Properties are open-world and are not interpreted by this module.
    #[must_use]
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

// ============================================================================
// OPERATION REPORT
// ============================================================================

/// Describes a completed serialization or deserialization operation.
///
/// Byte counts are reported by the provider. Providers must report
/// counts accurately according to their documented stream contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializationReport {
    /// Format used for the operation.
    pub format: SerializationFormatId,

    /// Number of source bytes consumed.
    pub input_bytes: u64,

    /// Number of destination bytes produced.
    pub output_bytes: u64,

    /// Schema reference associated with the operation, when applicable.
    pub schema: Option<SchemaReference>,
}

impl SerializationReport {
    /// Creates an operation report.
    #[must_use]
    pub fn new(
        format: SerializationFormatId,
        input_bytes: u64,
        output_bytes: u64,
        schema: Option<SchemaReference>,
    ) -> Self {
        Self {
            format,
            input_bytes,
            output_bytes,
            schema,
        }
    }
}

// ============================================================================
// FORMAT PROVIDER
// ============================================================================

/// Implements one serialization format.
///
/// A provider must implement real encoding and decoding behavior.
/// It must never report success after returning placeholder or
/// fabricated output.
///
/// Both operations are streaming interfaces. Providers should process
/// input incrementally instead of requiring complete input materialization.
///
/// The meaning of the source bytes is established by the caller's
/// canonical data adapter. This trait does not invent a second Zamani
/// value representation.
///
/// Providers must document:
///
/// - the byte representation they consume and produce;
/// - schema handling and compatibility behavior;
/// - canonicalization guarantees;
/// - deterministic behavior, if promised;
/// - resource and operational budgets;
/// - error and partial-output behavior;
/// - security properties and limitations.
pub trait SerializationProvider: Send + Sync + 'static {
    /// Returns the provider's immutable format descriptor.
    fn descriptor(&self) -> &FormatDescriptor;

    /// Encodes a source stream into the provider's format.
    ///
    /// The provider must return an error if encoding fails.
    ///
    /// The destination may contain partial output after an error.
    /// Callers requiring atomic publication should write to a
    /// transactional or temporary destination and publish only
    /// after this operation succeeds.
    fn encode(
        &self,
        source: &mut dyn Read,
        destination: &mut dyn Write,
        options: &SerializationOptions,
    ) -> Result<SerializationReport>;

    /// Decodes a provider-format stream into the caller's
    /// canonical data representation.
    ///
    /// The provider must validate the input according to its documented
    /// format and schema contract.
    ///
    /// The destination may contain partial output after an error.
    fn decode(
        &self,
        source: &mut dyn Read,
        destination: &mut dyn Write,
        options: &SerializationOptions,
    ) -> Result<SerializationReport>;
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// A thread-safe registry of serialization format providers.
///
/// Registration is open-world. Adding a new format does not require
/// changing a closed enum or editing this module.
///
/// Provider lookup returns a shared handle. The registry lock is not
/// held while a provider performs encoding or decoding.
#[derive(Clone, Default)]
pub struct SerializationRegistry {
    providers: Arc<
        RwLock<BTreeMap<SerializationFormatId, Arc<dyn SerializationProvider>>>,
    >,
}

impl SerializationRegistry {
    /// Creates an empty provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a provider.
    ///
    /// Duplicate format identifiers are rejected. Registration does
    /// not replace an existing provider implicitly.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is already registered or
    /// the registry lock is poisoned.
    pub fn register<P>(&self, provider: P) -> Result<()>
    where
        P: SerializationProvider,
    {
        self.register_shared(Arc::new(provider))
    }

    /// Registers a shared provider implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is already registered or
    /// the registry lock is poisoned.
    pub fn register_shared(
        &self,
        provider: Arc<dyn SerializationProvider>,
    ) -> Result<()> {
        let id = provider.descriptor().id.clone();

        let mut providers = self.providers.write().map_err(|_| {
            serialization_error(
                ErrorKind::Concurrency,
                "serialization provider registry lock is poisoned",
                "serialization.registry.register",
            )
        })?;

        if providers.contains_key(&id) {
            return Err(serialization_error(
                ErrorKind::AlreadyExists,
                format!("serialization provider already registered: {id}"),
                "serialization.registry.register",
            ));
        }

        providers.insert(id, provider);
        Ok(())
    }

    /// Resolves a provider by format identifier.
    ///
    /// The returned handle remains valid if the provider is subsequently
    /// removed from the registry.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider is not registered or the
    /// registry lock is poisoned.
    pub fn resolve(
        &self,
        id: &SerializationFormatId,
    ) -> Result<Arc<dyn SerializationProvider>> {
        let providers = self.providers.read().map_err(|_| {
            serialization_error(
                ErrorKind::Concurrency,
                "serialization provider registry lock is poisoned",
                "serialization.registry.resolve",
            )
        })?;

        providers.get(id).cloned().ok_or_else(|| {
            serialization_error(
                ErrorKind::NotFound,
                format!("no serialization provider registered for format: {id}"),
                "serialization.registry.resolve",
            )
        })
    }

    /// Removes a provider from future registry lookups.
    ///
    /// Existing shared provider handles remain valid.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider is not registered or the
    /// registry lock is poisoned.
    pub fn unregister(
        &self,
        id: &SerializationFormatId,
    ) -> Result<Arc<dyn SerializationProvider>> {
        let mut providers = self.providers.write().map_err(|_| {
            serialization_error(
                ErrorKind::Concurrency,
                "serialization provider registry lock is poisoned",
                "serialization.registry.unregister",
            )
        })?;

        providers.remove(id).ok_or_else(|| {
            serialization_error(
                ErrorKind::NotFound,
                format!("no serialization provider registered for format: {id}"),
                "serialization.registry.unregister",
            )
        })
    }

    /// Returns the number of registered providers.
    ///
    /// This is an observation, not a capacity limit.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn provider_count(&self) -> Result<usize> {
        let providers = self.providers.read().map_err(|_| {
            serialization_error(
                ErrorKind::Concurrency,
                "serialization provider registry lock is poisoned",
                "serialization.registry.count",
            )
        })?;

        Ok(providers.len())
    }
}

// ============================================================================
// SERIALIZATION ENGINE
// ============================================================================

/// Provider-dispatch interface for serialization operations.
///
/// The engine does not choose a hardware target or create a codec.
/// It resolves the explicitly requested format and delegates the
/// operation to its registered provider.
#[derive(Clone)]
pub struct SerializationEngine {
    registry: SerializationRegistry,
}

impl SerializationEngine {
    /// Creates an engine using the supplied provider registry.
    #[must_use]
    pub fn new(registry: SerializationRegistry) -> Self {
        Self { registry }
    }

    /// Returns the registry used by this engine.
    #[must_use]
    pub fn registry(&self) -> &SerializationRegistry {
        &self.registry
    }

    /// Encodes a stream using the requested format.
    ///
    /// # Errors
    ///
    /// Returns a structured error if provider resolution or encoding fails.
    pub fn encode(
        &self,
        format: &SerializationFormatId,
        source: &mut dyn Read,
        destination: &mut dyn Write,
        options: &SerializationOptions,
    ) -> Result<SerializationReport> {
        let provider = self.registry.resolve(format)?;
        let report = provider.encode(source, destination, options)?;

        validate_report(format, options, &report, "serialization.encode")?;

        Ok(report)
    }

    /// Decodes a stream using the requested format.
    ///
    /// # Errors
    ///
    /// Returns a structured error if provider resolution or decoding fails.
    pub fn decode(
        &self,
        format: &SerializationFormatId,
        source: &mut dyn Read,
        destination: &mut dyn Write,
        options: &SerializationOptions,
    ) -> Result<SerializationReport> {
        let provider = self.registry.resolve(format)?;
        let report = provider.decode(source, destination, options)?;

        validate_report(format, options, &report, "serialization.decode")?;

        Ok(report)
    }
}

// ============================================================================
// REPORT VALIDATION
// ============================================================================

fn validate_report(
    requested_format: &SerializationFormatId,
    options: &SerializationOptions,
    report: &SerializationReport,
    operation: &'static str,
) -> Result<()> {
    if &report.format != requested_format {
        return Err(serialization_error(
            ErrorKind::ContractViolation,
            "provider returned a report for a different serialization format",
            operation,
        ));
    }

    if let Some(requested_schema) = &options.schema {
        if report.schema.as_ref() != Some(requested_schema) {
            return Err(serialization_error(
                ErrorKind::ContractViolation,
                "provider report does not match the requested schema reference",
                operation,
            ));
        }
    }

    Ok(())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn serialization_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> Error {
    Error::new(kind, message)
        .in_component("stdlib.data.serialization")
        .during_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct IdentityProvider {
        descriptor: FormatDescriptor,
    }

    impl IdentityProvider {
        fn new() -> Self {
            Self {
                descriptor: FormatDescriptor::new(
                    SerializationFormatId::new("test.identity")
                        .expect("valid format identifier"),
                    "Identity test codec",
                )
                .with_streaming(true)
                .with_schema_support(true)
                .with_deterministic_encoding(true),
            }
        }

        fn copy_stream(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> Result<SerializationReport> {
            let input_bytes = std::io::copy(source, destination).map_err(|error| {
                Error::new(ErrorKind::Io, "serialization stream operation failed")
                    .in_component("stdlib.data.serialization")
                    .during_operation("serialization.test.copy")
                    .with_source(error)
            })?;

            Ok(SerializationReport::new(
                self.descriptor.id.clone(),
                input_bytes,
                input_bytes,
                options.schema.clone(),
            ))
        }
    }

    impl SerializationProvider for IdentityProvider {
        fn descriptor(&self) -> &FormatDescriptor {
            &self.descriptor
        }

        fn encode(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> Result<SerializationReport> {
            self.copy_stream(source, destination, options)
        }

        fn decode(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> Result<SerializationReport> {
            self.copy_stream(source, destination, options)
        }
    }

    fn test_engine() -> SerializationEngine {
        let registry = SerializationRegistry::new();
        registry
            .register(IdentityProvider::new())
            .expect("provider registration succeeds");

        SerializationEngine::new(registry)
    }

    #[test]
    fn rejects_empty_format_identifier() {
        assert!(SerializationFormatId::new("").is_err());
    }

    #[test]
    fn accepts_open_world_format_identifiers() {
        let id = SerializationFormatId::new(
            "organization.future.custom-format",
        )
        .expect("valid format identifier");

        assert_eq!(id.as_str(), "organization.future.custom-format");
    }

    #[test]
    fn rejects_empty_schema_identifier() {
        assert!(SchemaId::new("").is_err());
    }

    #[test]
    fn preserves_schema_version() {
        let schema = SchemaReference::new(
            SchemaId::new("organization.dataset").expect("valid schema"),
        )
        .with_version("revision:2026-alpha");

        assert_eq!(
            schema.version.as_deref(),
            Some("revision:2026-alpha")
        );
    }

    #[test]
    fn preserves_extensible_options() {
        let options = SerializationOptions::new()
            .with_canonical_representation(true)
            .with_property("compression", "provider-defined")
            .with_property("encoding", "provider-defined");

        assert!(options.canonical);
        assert_eq!(
            options.properties.get("compression").map(String::as_str),
            Some("provider-defined")
        );
    }

    #[test]
    fn registers_and_resolves_provider() {
        let registry = SerializationRegistry::new();

        registry
            .register(IdentityProvider::new())
            .expect("registration succeeds");

        let id = SerializationFormatId::new("test.identity")
            .expect("valid identifier");

        let provider = registry.resolve(&id).expect("provider exists");

        assert_eq!(provider.descriptor().id, id);
        assert_eq!(registry.provider_count(), Ok(1));
    }

    #[test]
    fn rejects_duplicate_provider_registration() {
        let registry = SerializationRegistry::new();

        registry
            .register(IdentityProvider::new())
            .expect("first registration succeeds");

        let result = registry.register(IdentityProvider::new());

        assert!(matches!(
            result,
            Err(error) if error.kind() == ErrorKind::AlreadyExists
        ));
    }

    #[test]
    fn unregister_removes_provider_from_future_lookups() {
        let registry = SerializationRegistry::new();

        registry
            .register(IdentityProvider::new())
            .expect("registration succeeds");

        let id = SerializationFormatId::new("test.identity")
            .expect("valid identifier");

        let removed = registry.unregister(&id).expect("provider removed");

        assert_eq!(removed.descriptor().id, id);
        assert_eq!(registry.provider_count(), Ok(0));
        assert!(registry.resolve(&id).is_err());
    }

    #[test]
    fn encoding_streams_through_registered_provider() {
        let engine = test_engine();
        let format = SerializationFormatId::new("test.identity")
            .expect("valid format");

        let input = b"Zamani portable serialization";
        let mut source = Cursor::new(input.as_slice());
        let mut destination = Vec::new();

        let report = engine
            .encode(
                &format,
                &mut source,
                &mut destination,
                &SerializationOptions::new(),
            )
            .expect("encoding succeeds");

        assert_eq!(destination, input);
        assert_eq!(report.input_bytes, input.len() as u64);
        assert_eq!(report.output_bytes, input.len() as u64);
        assert_eq!(report.format, format);
    }

    #[test]
    fn decoding_streams_through_registered_provider() {
        let engine = test_engine();
        let format = SerializationFormatId::new("test.identity")
            .expect("valid format");

        let input = b"encoded data";
        let mut source = Cursor::new(input.as_slice());
        let mut destination = Vec::new();

        let report = engine
            .decode(
                &format,
                &mut source,
                &mut destination,
                &SerializationOptions::new(),
            )
            .expect("decoding succeeds");

        assert_eq!(destination, input);
        assert_eq!(report.input_bytes, input.len() as u64);
    }

    #[test]
    fn schema_is_carried_through_provider_contract() {
        let engine = test_engine();
        let format = SerializationFormatId::new("test.identity")
            .expect("valid format");

        let schema = SchemaReference::new(
            SchemaId::new("zamani.data.record").expect("valid schema"),
        )
        .with_version("1");

        let options = SerializationOptions::new().with_schema(schema.clone());

        let mut source = Cursor::new(b"record".as_slice());
        let mut destination = Vec::new();

        let report = engine
            .encode(&format, &mut source, &mut destination, &options)
            .expect("encoding succeeds");

        assert_eq!(report.schema, Some(schema));
    }

    #[test]
    fn rejects_unregistered_format() {
        let engine = SerializationEngine::new(SerializationRegistry::new());
        let format = SerializationFormatId::new("unknown.format")
            .expect("valid identifier");

        let mut source = Cursor::new(b"input".as_slice());
        let mut destination = Vec::new();

        assert!(engine
            .encode(
                &format,
                &mut source,
                &mut destination,
                &SerializationOptions::new(),
            )
            .is_err());
    }

    #[test]
    fn report_rejects_provider_format_mismatch() {
        let requested = SerializationFormatId::new("requested.format")
            .expect("valid identifier");

        let returned = SerializationReport::new(
            SerializationFormatId::new("different.format")
                .expect("valid identifier"),
            1,
            1,
            None,
        );

        assert!(validate_report(
            &requested,
            &SerializationOptions::new(),
            &returned,
            "serialization.test",
        )
        .is_err());
    }

    #[test]
    fn report_rejects_schema_mismatch() {
        let format = SerializationFormatId::new("test.identity")
            .expect("valid identifier");

        let requested_schema = SchemaReference::new(
            SchemaId::new("schema.requested").expect("valid schema"),
        );

        let returned_schema = SchemaReference::new(
            SchemaId::new("schema.different").expect("valid schema"),
        );

        let options =
            SerializationOptions::new().with_schema(requested_schema);

        let report =
            SerializationReport::new(format.clone(), 1, 1, Some(returned_schema));

        assert!(validate_report(
            &format,
            &options,
            &report,
            "serialization.test",
        )
        .is_err());
    }
}
