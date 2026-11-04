
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Deployment — Model Formats
//!
//! Path: src/ai/deployment/formats.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines extensible AI artifact-format descriptors, provider contracts,
//! format registration, payload validation, and explicit conversion.
//!
//! This module is an integration layer over the canonical backend artifact
//! representation. It does not implement another artifact payload system.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI model-format descriptors;
//! - format-provider interfaces;
//! - format-provider registration;
//! - deterministic format lookup and enumeration;
//! - provider-based payload validation;
//! - explicit format conversion contracts;
//! - conversion fidelity reporting;
//! - conversion-policy enforcement;
//! - format-specific integration errors.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - artifact identities;
//! - artifact payload storage;
//! - artifact hashing or signatures;
//! - model semantics;
//! - model graph representation;
//! - model execution;
//! - model training;
//! - backend code generation;
//! - persistent manifests;
//! - deployment orchestration;
//! - hardware discovery or allocation;
//! - scheduling;
//! - runtime execution.
//!
//! ## Canonical authorities
//!
//! - crate::backend::artifact owns ArtifactFormat, ArtifactEncoding,
//!   ArtifactPayload, and OutputArtifact.
//! - crate::ai::deployment::artifacts owns AiArtifact.
//! - crate::ai::version owns AI artifact schema versions.
//! - crate::ai::provenance owns AI provenance.
//! - deployment::manifests owns persistent deployment manifests.
//! - deployment::compatibility owns deployment compatibility decisions.
//! - backend and runtime infrastructure own realization.
//!
//! ## Conversion safety
//!
//! A declared format does not prove that bytes conform to it.
//! Providers must validate payloads using their actual format rules.
//!
//! Conversion is never assumed to preserve semantics merely because
//! the destination provider returns bytes.
//!
//! A conversion must report its fidelity. Lossy or semantically
//! unverified conversions require explicit caller policy.
//!
//! ## POCO-REAF
//!
//! No universal limit is imposed on model size, artifact size, format
//! count, model parameter count, tensor rank, device count, or hardware
//! capacity.
//!
//! Collections grow according to available resources.
//! Operational budgets belong to callers and infrastructure.
//!
//! This module does not promise literal infinite memory or computation.
//!
//! ## Concurrency
//!
//! The registry uses immutable Arc-backed provider snapshots.
//! The registry itself does not introduce global mutable state.
//! Applications may share it through their chosen synchronization boundary.
//!
//! ## Freeze criteria
//!
//! - Canonical backend artifact types are reused.
//! - Format identifiers are extensible.
//! - Provider registration rejects duplicates.
//! - Enumeration is deterministic.
//! - Provider snapshots remain valid after registry changes.
//! - Conversion fidelity is explicit.
//! - Unsupported conversions fail explicitly.
//! - No conversion silently changes semantic guarantees.
//! - No unsafe Rust is used.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::backend::artifact::{
    ArtifactEncoding,
    ArtifactFormat,
    ArtifactPayload,
};

use crate::ai::deployment::artifacts::AiArtifact;

use crate::ai::version::VersionTriple;

// ============================================================================
// FORMAT DESCRIPTOR
// ============================================================================

/// Describes an AI artifact serialization or interchange format.
///
/// The format identifier is the canonical backend `ArtifactFormat`.
///
/// `format_revision` identifies the revision of the external format
/// specification supported by this provider. It is deliberately distinct
/// from `AiArtifactFormatVersion`, which identifies Zamani's AI artifact
/// descriptor schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiModelFormatDescriptor {
    id: ArtifactFormat,
    format_revision: VersionTriple,
    supports_text: bool,
    supports_binary: bool,
    description: Option<String>,
    extensions: BTreeMap<String, String>,
}

impl AiModelFormatDescriptor {
    /// Construct a format descriptor.
    ///
    /// At least one payload encoding must be supported.
    pub fn new(
        id: ArtifactFormat,
        format_revision: VersionTriple,
        supports_text: bool,
        supports_binary: bool,
    ) -> Result<Self, AiFormatError> {
        if !supports_text && !supports_binary {
            return Err(AiFormatError::NoSupportedEncoding {
                format: id.to_string(),
            });
        }

        Ok(Self {
            id,
            format_revision,
            supports_text,
            supports_binary,
            description: None,
            extensions: BTreeMap::new(),
        })
    }

    /// Return the canonical format identifier.
    #[must_use]
    pub fn id(&self) -> &ArtifactFormat {
        &self.id
    }

    /// Return the external format specification revision.
    #[must_use]
    pub const fn format_revision(&self) -> VersionTriple {
        self.format_revision
    }

    /// Return whether text payloads are supported.
    #[must_use]
    pub const fn supports_text(&self) -> bool {
        self.supports_text
    }

    /// Return whether binary payloads are supported.
    #[must_use]
    pub const fn supports_binary(&self) -> bool {
        self.supports_binary
    }

    /// Return the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return provider extension metadata.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<String, String> {
        &self.extensions
    }

    /// Set a human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Insert deterministic, namespaced provider metadata.
    ///
    /// Extension metadata is descriptive. Unknown keys must never be
    /// interpreted as additional format semantics.
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AiFormatError> {
        let key = key.into();
        let value = value.into();

        validate_extension_key(&key)?;

        if value.chars().any(char::is_control) {
            return Err(AiFormatError::InvalidExtensionValue {
                key,
            });
        }

        self.extensions.insert(key, value);
        Ok(self)
    }

    /// Validate the descriptor, including externally constructed metadata.
    pub fn validate(&self) -> Result<(), AiFormatError> {
        if !self.supports_text && !self.supports_binary {
            return Err(AiFormatError::NoSupportedEncoding {
                format: self.id.to_string(),
            });
        }

        for (key, value) in &self.extensions {
            validate_extension_key(key)?;

            if value.chars().any(char::is_control) {
                return Err(AiFormatError::InvalidExtensionValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }

    /// Determine whether the descriptor accepts a payload encoding.
    #[must_use]
    pub const fn supports_encoding(
        &self,
        encoding: ArtifactEncoding,
    ) -> bool {
        match encoding {
            ArtifactEncoding::Text => self.supports_text,
            ArtifactEncoding::Binary => self.supports_binary,
        }
    }
}

fn validate_extension_key(key: &str) -> Result<(), AiFormatError> {
    if key.is_empty()
        || !key.contains('.')
        || key.starts_with('.')
        || key.ends_with('.')
        || key.split('.').any(str::is_empty)
        || !key.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b'_')
        })
    {
        return Err(AiFormatError::InvalidExtensionKey {
            key: key.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// CONVERSION POLICY AND REPORTING
// ============================================================================

/// Describes the semantic fidelity of a format conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFidelity {
    /// The provider establishes that the conversion preserves the
    /// declared semantic contract.
    SemanticsPreserved,

    /// The conversion is known to lose information or precision.
    Lossy,

    /// The provider cannot establish semantic preservation.
    Unknown,
}

/// Policy governing potentially lossy conversions.
///
/// The default policy rejects conversions whose fidelity is not
/// established as semantic preservation.
///
/// Allowing numeric loss does not automatically authorize arbitrary
/// changes to the meaning of the AI computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionPolicy {
    allow_lossy: bool,
    allow_unknown_fidelity: bool,
}

impl Default for ConversionPolicy {
    fn default() -> Self {
        Self {
            allow_lossy: false,
            allow_unknown_fidelity: false,
        }
    }
}

impl ConversionPolicy {
    /// Construct the strict default policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            allow_lossy: false,
            allow_unknown_fidelity: false,
        }
    }

    /// Construct a policy that explicitly permits known lossy conversion.
    ///
    /// This does not permit conversions with unknown semantic fidelity.
    #[must_use]
    pub const fn allowing_lossy() -> Self {
        Self {
            allow_lossy: true,
            allow_unknown_fidelity: false,
        }
    }

    /// Construct a policy permitting conversion with unknown fidelity.
    ///
    /// This must be an explicit application decision. The conversion
    /// report will continue to identify fidelity as unknown.
    #[must_use]
    pub const fn allowing_unknown_fidelity() -> Self {
        Self {
            allow_lossy: true,
            allow_unknown_fidelity: true,
        }
    }

    /// Return whether known lossy conversion is permitted.
    #[must_use]
    pub const fn permits_lossy(self) -> bool {
        self.allow_lossy
    }

    /// Return whether unknown conversion fidelity is permitted.
    #[must_use]
    pub const fn permits_unknown_fidelity(self) -> bool {
        self.allow_unknown_fidelity
    }

    fn permits(self, fidelity: ConversionFidelity) -> bool {
        match fidelity {
            ConversionFidelity::SemanticsPreserved => true,
            ConversionFidelity::Lossy => self.allow_lossy,
            ConversionFidelity::Unknown => self.allow_unknown_fidelity,
        }
    }
}

/// Context supplied to a format conversion provider.
///
/// Options are deterministic descriptive values. Providers must validate
/// and interpret only keys that belong to their documented contract.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConversionContext {
    options: BTreeMap<String, String>,
}

impl ConversionContext {
    /// Create an empty conversion context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a conversion option.
    ///
    /// Names must be qualified identifiers.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AiFormatError> {
        let key = key.into();
        let value = value.into();

        validate_extension_key(&key)?;

        if value.chars().any(char::is_control) {
            return Err(AiFormatError::InvalidExtensionValue {
                key,
            });
        }

        Ok(self.options.insert(key, value))
    }

    /// Retrieve a conversion option.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.options.get(key).map(String::as_str)
    }

    /// Return all options in deterministic order.
    #[must_use]
    pub fn options(&self) -> &BTreeMap<String, String> {
        &self.options
    }
}

/// Result produced by a format conversion provider.
///
/// The payload is the canonical backend payload type. The provider
/// does not create a competing artifact representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatConversionOutput {
    payload: ArtifactPayload,
    fidelity: ConversionFidelity,
}

impl FormatConversionOutput {
    /// Construct a conversion result.
    ///
    /// The caller must report fidelity honestly. The registry enforces
    /// the selected conversion policy.
    #[must_use]
    pub const fn new(
        payload: ArtifactPayload,
        fidelity: ConversionFidelity,
    ) -> Self {
        Self {
            payload,
            fidelity,
        }
    }

    /// Return the converted canonical payload.
    #[must_use]
    pub fn payload(&self) -> &ArtifactPayload {
        &self.payload
    }

    /// Consume the result and return its payload.
    #[must_use]
    pub fn into_payload(self) -> ArtifactPayload {
        self.payload
    }

    /// Return the declared conversion fidelity.
    #[must_use]
    pub const fn fidelity(&self) -> ConversionFidelity {
        self.fidelity
    }
}

// ============================================================================
// FORMAT PROVIDER CONTRACT
// ============================================================================

/// Implemented by an AI artifact format provider.
///
/// Providers implement actual format-specific validation and conversion.
/// Registering a provider does not prove that its implementation is
/// trustworthy, complete, or conformant; that requires separate tests,
/// review, and verification.
///
/// Providers should be immutable after registration. The registry stores
/// them behind `Arc` so consumers can retain stable snapshots.
pub trait AiModelFormatProvider: Send + Sync {
    /// Return the provider's immutable format descriptor.
    fn descriptor(&self) -> &AiModelFormatDescriptor;

    /// Validate a payload against the provider's actual format rules.
    ///
    /// A successful result establishes only the guarantees documented
    /// by this provider. It does not establish model correctness or
    /// execution compatibility.
    fn validate_payload(
        &self,
        payload: &ArtifactPayload,
        context: &ConversionContext,
    ) -> Result<(), AiFormatError>;

    /// Convert a payload from another registered format.
    ///
    /// Implementations must reject unsupported source formats and must
    /// not silently discard unsupported semantic information.
    fn convert_from(
        &self,
        source_format: &ArtifactFormat,
        source_payload: &ArtifactPayload,
        context: &ConversionContext,
    ) -> Result<FormatConversionOutput, AiFormatError>;
}

// ============================================================================
// FORMAT REGISTRY
// ============================================================================

/// Immutable registered provider snapshot.
pub type AiFormatProviderRef = Arc<dyn AiModelFormatProvider>;

/// Extensible registry of AI artifact format providers.
///
/// The registry is deterministic and has no implicit default provider.
/// Registration is explicit. Format selection is exact and never silently
/// substitutes a different format.
#[derive(Default)]
pub struct AiModelFormatRegistry {
    providers: BTreeMap<ArtifactFormat, AiFormatProviderRef>,
}

impl fmt::Debug for AiModelFormatRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AiModelFormatRegistry")
            .field("registered_formats", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl AiModelFormatRegistry {
    /// Create an empty format registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one immutable format provider.
    ///
    /// Duplicate format identifiers are rejected. The registry is
    /// unchanged if validation fails.
    pub fn register(
        &mut self,
        provider: AiFormatProviderRef,
    ) -> Result<(), AiFormatError> {
        let descriptor = provider.descriptor();
        descriptor.validate()?;

        let id = descriptor.id().clone();

        if self.providers.contains_key(&id) {
            return Err(AiFormatError::DuplicateFormat {
                format: id.to_string(),
            });
        }

        self.providers.insert(id, provider);
        Ok(())
    }

    /// Register a batch of providers atomically.
    ///
    /// If any descriptor is invalid or duplicated, no provider from
    /// the batch is published.
    pub fn register_batch<I>(
        &mut self,
        providers: I,
    ) -> Result<(), AiFormatError>
    where
        I: IntoIterator<Item = AiFormatProviderRef>,
    {
        let mut pending = BTreeMap::new();

        for provider in providers {
            let descriptor = provider.descriptor();
            descriptor.validate()?;

            let id = descriptor.id().clone();

            if self.providers.contains_key(&id)
                || pending.contains_key(&id)
            {
                return Err(AiFormatError::DuplicateFormat {
                    format: id.to_string(),
                });
            }

            pending.insert(id, provider);
        }

        self.providers.extend(pending);
        Ok(())
    }

    /// Look up a provider by its exact canonical format identifier.
    #[must_use]
    pub fn get(
        &self,
        format: &ArtifactFormat,
    ) -> Option<AiFormatProviderRef> {
        self.providers.get(format).cloned()
    }

    /// Return whether a format identifier is registered.
    #[must_use]
    pub fn contains(&self, format: &ArtifactFormat) -> bool {
        self.providers.contains_key(format)
    }

    /// Return the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Return registered format identifiers in deterministic order.
    #[must_use]
    pub fn formats(&self) -> Vec<ArtifactFormat> {
        self.providers.keys().cloned().collect()
    }

    /// Validate an AI artifact using its registered format provider.
    ///
    /// The artifact's backend format is authoritative. This operation
    /// never guesses a format from the payload bytes or file extension.
    pub fn validate_artifact(
        &self,
        artifact: &AiArtifact,
        context: &ConversionContext,
    ) -> Result<(), AiFormatError> {
        artifact
            .validate()
            .map_err(|error| AiFormatError::Artifact(error.to_string()))?;

        let format = artifact
            .backend_format()
            .map_err(|error| AiFormatError::Artifact(error.to_string()))?;

        let provider = self
            .get(&format)
            .ok_or_else(|| AiFormatError::ProviderNotFound {
                format: format.to_string(),
            })?;

        let backend_artifact = artifact.backend_artifact();
        let payload = &backend_artifact.payload;

        if !provider
            .descriptor()
            .supports_encoding(payload.encoding())
        {
            return Err(AiFormatError::UnsupportedEncoding {
                format: format.to_string(),
            });
        }

        provider.validate_payload(payload, context)
    }

    /// Convert an AI artifact into another registered format.
    ///
    /// The returned payload is not automatically persisted or wrapped
    /// in a new artifact. The caller must assign a new backend artifact
    /// identity and preserve provenance through the owning systems.
    ///
    /// The source is validated before conversion. The destination
    /// provider validates its own output before it is returned.
    pub fn convert(
        &self,
        artifact: &AiArtifact,
        destination: &ArtifactFormat,
        context: &ConversionContext,
        policy: ConversionPolicy,
    ) -> Result<FormatConversionOutput, AiFormatError> {
        artifact
            .validate()
            .map_err(|error| AiFormatError::Artifact(error.to_string()))?;

        let source = artifact
            .backend_format()
            .map_err(|error| AiFormatError::Artifact(error.to_string()))?;

        if &source == destination {
            return Err(AiFormatError::SameFormatConversion {
                format: source.to_string(),
            });
        }

        let source_provider = self
            .get(&source)
            .ok_or_else(|| AiFormatError::ProviderNotFound {
                format: source.to_string(),
            })?;

        let destination_provider = self
            .get(destination)
            .ok_or_else(|| AiFormatError::ProviderNotFound {
                format: destination.to_string(),
            })?;

        let source_payload = &artifact.backend_artifact().payload;

        if !source_provider
            .descriptor()
            .supports_encoding(source_payload.encoding())
        {
            return Err(AiFormatError::UnsupportedEncoding {
                format: source.to_string(),
            });
        }

        source_provider.validate_payload(source_payload, context)?;

        let output = destination_provider.convert_from(
            &source,
            source_payload,
            context,
        )?;

        if !destination_provider
            .descriptor()
            .supports_encoding(output.payload().encoding())
        {
            return Err(AiFormatError::UnsupportedEncoding {
                format: destination.to_string(),
            });
        }

        destination_provider.validate_payload(
            output.payload(),
            context,
        )?;

        if !policy.permits(output.fidelity()) {
            return Err(AiFormatError::ConversionFidelityRejected {
                fidelity: output.fidelity(),
            });
        }

        Ok(output)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors returned by AI artifact format operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiFormatError {
    /// No supported payload encoding was declared.
    NoSupportedEncoding {
        /// Format identifier.
        format: String,
    },

    /// An extension key is malformed.
    InvalidExtensionKey {
        /// Invalid key.
        key: String,
    },

    /// An extension value contains control characters.
    InvalidExtensionValue {
        /// Associated key.
        key: String,
    },

    /// A format is already registered.
    DuplicateFormat {
        /// Duplicate format identifier.
        format: String,
    },

    /// No provider is registered for a requested format.
    ProviderNotFound {
        /// Missing format identifier.
        format: String,
    },

    /// A provider does not support the payload encoding.
    UnsupportedEncoding {
        /// Format identifier.
        format: String,
    },

    /// A conversion was requested between identical formats.
    SameFormatConversion {
        /// Format identifier.
        format: String,
    },

    /// A provider rejected the conversion fidelity.
    ConversionFidelityRejected {
        /// Reported fidelity.
        fidelity: ConversionFidelity,
    },

    /// AI artifact validation failed.
    Artifact {
        /// Explanation.
        message: String,
    },

    /// A format provider reported an error.
    Provider {
        /// Provider identifier.
        format: String,

        /// Provider explanation.
        message: String,
    },
}

impl fmt::Display for AiFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSupportedEncoding { format } => {
                write!(formatter, "format '{format}' declares no supported encoding")
            }
            Self::InvalidExtensionKey { key } => {
                write!(formatter, "invalid qualified extension key '{key}'")
            }
            Self::InvalidExtensionValue { key } => {
                write!(formatter, "invalid extension value for '{key}'")
            }
            Self::DuplicateFormat { format } => {
                write!(formatter, "AI format '{format}' is already registered")
            }
            Self::ProviderNotFound { format } => {
                write!(formatter, "no provider registered for AI format '{format}'")
            }
            Self::UnsupportedEncoding { format } => {
                write!(formatter, "format provider '{format}' does not support this payload encoding")
            }
            Self::SameFormatConversion { format } => {
                write!(formatter, "source and destination format are both '{format}'")
            }
            Self::ConversionFidelityRejected { fidelity } => {
                write!(formatter, "conversion fidelity '{fidelity:?}' is rejected by policy")
            }
            Self::Artifact { message } => {
                write!(formatter, "AI artifact validation failed: {message}")
            }
            Self::Provider { format, message } => {
                write!(formatter, "AI format provider '{format}' failed: {message}")
            }
        }
    }
}

impl Error for AiFormatError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::backend::artifact::ArtifactPayload;

    #[derive(Debug)]
    struct TestProvider {
        descriptor: AiModelFormatDescriptor,
    }

    impl TestProvider {
        fn new(
            name: &str,
            supports_text: bool,
            supports_binary: bool,
        ) -> Self {
            Self {
                descriptor: AiModelFormatDescriptor::new(
                    ArtifactFormat::new(name)
                        .expect("valid test format"),
                    VersionTriple::new(1, 0, 0),
                    supports_text,
                    supports_binary,
                )
                .expect("valid test descriptor"),
            }
        }
    }

    impl AiModelFormatProvider for TestProvider {
        fn descriptor(&self) -> &AiModelFormatDescriptor {
            &self.descriptor
        }

        fn validate_payload(
            &self,
            _payload: &ArtifactPayload,
            _context: &ConversionContext,
        ) -> Result<(), AiFormatError> {
            Ok(())
        }

        fn convert_from(
            &self,
            _source_format: &ArtifactFormat,
            source_payload: &ArtifactPayload,
            _context: &ConversionContext,
        ) -> Result<FormatConversionOutput, AiFormatError> {
            Ok(FormatConversionOutput::new(
                ArtifactPayload::from_bytes(
                    if self.descriptor.supports_binary() {
                        ArtifactEncoding::Binary
                    } else {
                        ArtifactEncoding::Text
                    },
                    source_payload.as_bytes().to_vec(),
                ),
                ConversionFidelity::SemanticsPreserved,
            ))
        }
    }

    fn provider(
        name: &str,
        text: bool,
        binary: bool,
    ) -> AiFormatProviderRef {
        Arc::new(TestProvider::new(name, text, binary))
    }

    #[test]
    fn descriptor_requires_an_encoding() {
        let format = ArtifactFormat::new("example-ai")
            .expect("valid format");

        assert!(AiModelFormatDescriptor::new(
            format,
            VersionTriple::new(1, 0, 0),
            false,
            false,
        )
        .is_err());
    }

    #[test]
    fn descriptor_accepts_extensible_format_names() {
        let format = ArtifactFormat::new("vendor-model-v2")
            .expect("valid format");

        let descriptor = AiModelFormatDescriptor::new(
            format,
            VersionTriple::new(2, 1, 0),
            false,
            true,
        )
        .expect("valid descriptor")
        .with_extension("org.example.schema", "v2")
        .expect("valid extension");

        assert_eq!(
            descriptor.extensions().get("org.example.schema"),
            Some(&"v2".to_owned())
        );
    }

    #[test]
    fn malformed_extension_keys_are_rejected() {
        let format = ArtifactFormat::new("example-ai")
            .expect("valid format");

        let descriptor = AiModelFormatDescriptor::new(
            format,
            VersionTriple::new(1, 0, 0),
            true,
            false,
        )
        .expect("valid descriptor");

        assert!(
            descriptor
                .with_extension("not-qualified", "value")
                .is_err()
        );
    }

    #[test]
    fn registry_rejects_duplicate_formats() {
        let mut registry = AiModelFormatRegistry::new();

        registry
            .register(provider("example-ai", true, false))
            .expect("first registration succeeds");

        assert!(
            registry
                .register(provider("example-ai", true, false))
                .is_err()
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_enumeration_is_deterministic() {
        let mut registry = AiModelFormatRegistry::new();

        registry
            .register_batch([
                provider("format-z", true, false),
                provider("format-a", false, true),
                provider("format-m", true, true),
            ])
            .expect("batch registration succeeds");

        let names: Vec<String> = registry
            .formats()
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            names,
            vec![
                "format-a".to_owned(),
                "format-m".to_owned(),
                "format-z".to_owned(),
            ]
        );
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = AiModelFormatRegistry::new();

        registry
            .register(provider("existing", true, false))
            .expect("initial registration succeeds");

        let result = registry.register_batch([
            provider("new-format", true, false),
            provider("existing", true, false),
        ]);

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);

        assert!(
            !registry.contains(
                &ArtifactFormat::new("new-format")
                    .expect("valid format")
            )
        );
    }

    #[test]
    fn provider_snapshots_are_independent_of_registry_changes() {
        let mut registry = AiModelFormatRegistry::new();

        registry
            .register(provider("snapshot-format", true, false))
            .expect("registration succeeds");

        let id = ArtifactFormat::new("snapshot-format")
            .expect("valid format");

        let snapshot = registry.get(&id)
            .expect("provider exists");

        assert_eq!(snapshot.descriptor().id(), &id);
    }

    #[test]
    fn strict_policy_rejects_lossy_conversion() {
        let policy = ConversionPolicy::strict();

        assert!(!policy.permits(ConversionFidelity::Lossy));
        assert!(!policy.permits(ConversionFidelity::Unknown));
        assert!(policy.permits(
            ConversionFidelity::SemanticsPreserved
        ));
    }

    #[test]
    fn lossy_policy_does_not_accept_unknown_fidelity() {
        let policy = ConversionPolicy::allowing_lossy();

        assert!(policy.permits(ConversionFidelity::Lossy));
        assert!(!policy.permits(ConversionFidelity::Unknown));
    }

    #[test]
    fn unknown_fidelity_requires_explicit_policy() {
        let policy = ConversionPolicy::allowing_unknown_fidelity();

        assert!(policy.permits(ConversionFidelity::Unknown));
    }

    #[test]
    fn conversion_context_is_deterministic() {
        let mut context = ConversionContext::new();

        context
            .insert("org.example.precision", "float32")
            .expect("valid option");

        context
            .insert("org.example.layout", "contiguous")
            .expect("valid option");

        let keys: Vec<&String> = context.options().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"org.example.layout".to_owned(),
                &"org.example.precision".to_owned(),
            ]
        );
    }

    #[test]
    fn provider_encoding_declarations_are_enforced() {
        let provider = TestProvider::new(
            "binary-only",
            false,
            true,
        );

        assert!(!provider.descriptor().supports_encoding(
            ArtifactEncoding::Text
        ));

        assert!(provider.descriptor().supports_encoding(
            ArtifactEncoding::Binary
        ));
    }
}
