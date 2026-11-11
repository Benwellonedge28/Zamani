
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Data Formats
//!
//! Path: `src/stdlib/data/formats.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides the public format-discovery and registration facade for
//! the Zamani data subsystem.
//!
//! This module exposes the existing serialization format infrastructure
//! through a focused data-format API.
//!
//! It supports extensible formats for:
//!
//! - Classical data.
//! - Scientific data.
//! - AI and machine-learning data.
//! - Quantum data.
//! - Hardware-description data.
//! - Hybrid data.
//! - Distributed data.
//! - Networking and interoperability.
//! - Future computational domains.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - The public FormatCatalog facade.
//! - Format descriptor inspection.
//! - Media-type matching.
//! - Access to registered format providers.
//! - Format registration delegation.
//! - Format removal delegation.
//!
//! This module does NOT own:
//!
//! - Format identifiers.
//! - Format descriptor definitions.
//! - Serialization providers.
//! - Deserialization providers.
//! - Provider registry implementation.
//! - Encoding algorithms.
//! - Decoding algorithms.
//! - Schema definitions.
//! - Schema compatibility policy.
//! - Canonical data representations.
//! - Buffer allocation.
//! - Persistent storage.
//! - Filesystem operations.
//! - Network transport.
//! - Hardware selection.
//! - Runtime scheduling.
//! - Physical resource allocation.
//! - Compiler intermediate representations.
//!
//! Those responsibilities remain with their authoritative modules.
//!
//! ## AUTHORITATIVE IMPLEMENTATION
//!
//! The canonical format and serialization implementation is:
//!
//! `crate::stdlib::data::serialization`
//!
//! In particular, that module owns:
//!
//! - SerializationFormatId.
//! - FormatDescriptor.
//! - SerializationProvider.
//! - SerializationRegistry.
//! - SerializationEngine.
//! - SerializationOptions.
//! - SerializationReport.
//!
//! This module delegates to those existing types.
//!
//! It MUST NOT introduce a second provider registry or competing
//! format identifier.
//!
//! ## GRAMMAR INTEGRATION
//!
//! The data grammar composition authority is:
//!
//! `grammar/data/data.g4`
//!
//! The serialization syntax authority is:
//!
//! `grammar/data/serialization.g4`
//!
//! The deserialization syntax authority is:
//!
//! `grammar/data/deserialization.g4`
//!
//! The universal grammar composition remains owned by:
//!
//! `grammar/Zamani.g4`
//!
//! These grammars describe source-language syntax and intent.
//!
//! They do not instantiate providers, open files, decode bytes,
//! choose hardware, or execute serialization operations.
//!
//! A frontend and semantic adapter must translate validated
//! source-language constructs into the canonical data API.
//!
//! This module does not parse grammar constructs or create another AST.
//!
//! ## POCO-REAF
//!
//! Format identities are open-world.
//!
//! New formats can be registered without editing a closed enumeration
//! or adding a vendor-specific branch to this module.
//!
//! Format providers may be implemented by independent components.
//!
//! A format is not tied to a particular:
//!
//! - CPU architecture.
//! - Operating system.
//! - GPU.
//! - FPGA.
//! - ASIC.
//! - QPU.
//! - Distributed platform.
//! - Storage vendor.
//! - Network vendor.
//! - Cloud provider.
//!
//! The format describes data representation and interoperability,
//! not the physical execution target.
//!
//! ## SCALABILITY
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Registered formats.
//! - Providers.
//! - Media types.
//! - Format identifier lengths.
//! - Descriptor metadata.
//! - Data size.
//! - Stream length.
//! - Schema count.
//! - Computational domains.
//!
//! Actual operation limits depend on available resources,
//! platform representation, provider implementation and
//! explicitly configured operational budgets.
//!
//! No finite implementation promises literal infinite memory
//! or computation.
//!
//! ## CONCURRENCY
//!
//! FormatCatalog shares the existing SerializationRegistry.
//!
//! Registry synchronization and provider lifetime are handled by
//! SerializationRegistry.
//!
//! Provider callbacks are not executed by this module.
//!
//! ## ERROR HANDLING
//!
//! All fallible registration and lookup operations use the canonical
//! standard-library Result and Error types.
//!
//! Duplicate registration, missing formats and registry synchronization
//! failures are delegated to SerializationRegistry.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No global mutable state.
//! No hidden provider selection.
//! No implicit provider initialization.
//! No implicit format fallback.
//! No printing or logging.
//!
//! ## COMPATIBILITY
//!
//! This module is a facade over the existing serialization contracts.
//!
//! Changes to the underlying format representation must preserve
//! the public re-exports and facade behavior where compatibility
//! is promised.

use std::sync::Arc;

use crate::stdlib::error::Result;

use super::serialization::{
    FormatDescriptor,
    SerializationFormatId,
    SerializationProvider,
    SerializationRegistry,
};

// ============================================================================
// PUBLIC FORMAT TYPES
// ============================================================================

/// Open-world identity of a serialization format.
///
/// This is re-exported from the authoritative serialization module.
/// It is not a second format identifier implementation.
pub use super::serialization::SerializationFormatId as FormatId;

/// Canonical descriptor for a registered format provider.
///
/// This is re-exported from the authoritative serialization module.
pub use super::serialization::FormatDescriptor as FormatInfo;

/// Canonical serialization provider contract.
///
/// A provider must implement actual encoding and decoding behavior.
/// This alias does not introduce a second provider trait.
pub use super::serialization::SerializationProvider as FormatProvider;

/// Canonical serialization provider registry.
///
/// This alias does not introduce a second registry implementation.
pub use super::serialization::SerializationRegistry as FormatRegistry;

// ============================================================================
// FORMAT CATALOG
// ============================================================================

/// Public facade for discovering and managing registered data formats.
///
/// `FormatCatalog` shares the underlying serialization registry.
///
/// Cloning a catalog does not duplicate the registry. Cloned catalogs
/// observe registrations and removals performed through the same
/// shared registry.
///
/// The catalog does not automatically register built-in formats.
/// Actual format implementations must be supplied by providers.
///
/// # Examples
///
/// ```
/// use zamani_compiler::stdlib::data::formats::{
///     FormatCatalog,
///     FormatId,
/// };
///
/// let catalog = FormatCatalog::new();
///
/// let format = FormatId::new("organization.example.format")
///     .expect("format identifier is valid");
///
/// assert!(catalog.descriptor(&format).is_err());
/// ```
#[derive(Clone, Default)]
pub struct FormatCatalog {
    registry: SerializationRegistry,
}

impl FormatCatalog {
    /// Creates an empty format catalog.
    ///
    /// No codecs or providers are implicitly installed.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: SerializationRegistry::new(),
        }
    }

    /// Creates a format catalog using an existing serialization registry.
    ///
    /// This is useful when serialization, deserialization and format
    /// discovery must share the same provider registrations.
    #[must_use]
    pub fn from_registry(registry: SerializationRegistry) -> Self {
        Self { registry }
    }

    /// Returns a reference to the authoritative serialization registry.
    ///
    /// The returned registry can be shared with SerializationEngine.
    #[must_use]
    pub fn registry(&self) -> &SerializationRegistry {
        &self.registry
    }

    /// Registers a format provider.
    ///
    /// Registration is delegated to SerializationRegistry.
    ///
    /// Duplicate format identifiers are rejected by the underlying
    /// registry. Existing providers are never silently replaced.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The format identifier is already registered.
    /// - The registry cannot be accessed safely.
    pub fn register<P>(&self, provider: P) -> Result<()>
    where
        P: SerializationProvider,
    {
        self.registry.register(provider)
    }

    /// Registers a shared format provider.
    ///
    /// This supports provider implementations managed by another
    /// subsystem or application component.
    ///
    /// # Errors
    ///
    /// Returns an error if registration fails.
    pub fn register_shared(
        &self,
        provider: Arc<dyn SerializationProvider>,
    ) -> Result<()> {
        self.registry.register_shared(provider)
    }

    /// Resolves a registered provider by format identifier.
    ///
    /// The returned shared provider handle remains valid even if
    /// the provider is subsequently removed from the registry.
    ///
    /// # Errors
    ///
    /// Returns an error if the format is not registered or the
    /// underlying registry cannot be accessed.
    pub fn resolve(
        &self,
        id: &SerializationFormatId,
    ) -> Result<Arc<dyn SerializationProvider>> {
        self.registry.resolve(id)
    }

    /// Returns the descriptor for a registered format.
    ///
    /// The descriptor is cloned from the registered provider.
    /// This prevents callers from retaining a reference tied to
    /// an internal provider lock or registry operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the format is not registered or
    /// registry access fails.
    pub fn descriptor(
        &self,
        id: &SerializationFormatId,
    ) -> Result<FormatDescriptor> {
        let provider = self.registry.resolve(id)?;

        Ok(provider.descriptor().clone())
    }

    /// Removes a format provider from future registry lookups.
    ///
    /// Existing provider handles remain valid.
    ///
    /// Removal does not forcibly terminate operations already using
    /// a previously resolved provider handle.
    ///
    /// # Errors
    ///
    /// Returns an error if the format is not registered or
    /// registry access fails.
    pub fn unregister(
        &self,
        id: &SerializationFormatId,
    ) -> Result<Arc<dyn SerializationProvider>> {
        self.registry.unregister(id)
    }

    /// Returns the number of currently registered format providers.
    ///
    /// This is an observation, not a capacity limit.
    ///
    /// # Errors
    ///
    /// Returns an error if registry access fails.
    pub fn provider_count(&self) -> Result<usize> {
        self.registry.provider_count()
    }

    /// Returns whether a format is currently registered.
    ///
    /// # Errors
    ///
    /// Returns an error if registry access fails.
    pub fn contains(
        &self,
        id: &SerializationFormatId,
    ) -> Result<bool> {
        match self.registry.resolve(id) {
            Ok(_) => Ok(true),
            Err(error) => {
                // Only a missing provider means that the format is absent.
                // Other errors, such as a poisoned lock, must propagate.
                if error.kind()
                    == crate::stdlib::error::ErrorKind::NotFound
                {
                    Ok(false)
                } else {
                    Err(error)
                }
            }
        }
    }
}

// ============================================================================
// MEDIA-TYPE MATCHING
// ============================================================================

/// Determines whether a format descriptor declares support for a media type.
///
/// Media-type comparison is case-insensitive for the type and subtype.
/// Optional parameters on the requested media type are ignored for
/// matching purposes.
///
/// For example:
///
/// - `application/example`
/// - `Application/Example`
/// - `application/example; charset=utf-8`
///
/// match the same declared base media type.
///
/// This function does not perform content sniffing or prove that
/// incoming bytes actually conform to the declared media type.
///
/// Invalid or empty media-type values return `false`.
#[must_use]
pub fn supports_media_type(
    descriptor: &FormatDescriptor,
    requested_media_type: &str,
) -> bool {
    let Some(requested) = base_media_type(requested_media_type) else {
        return false;
    };

    descriptor.media_types.iter().any(|declared| {
        base_media_type(declared)
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(requested))
    })
}

/// Returns the base media type without optional parameters.
///
/// The returned slice borrows from the input.
///
/// This helper deliberately does not allocate or impose a fixed
/// media-type length ceiling.
fn base_media_type(value: &str) -> Option<&str> {
    let base = value
        .split_once(';')
        .map_or(value, |(media_type, _)| media_type)
        .trim();

    if base.is_empty() || !base.contains('/') {
        return None;
    }

    let (type_name, subtype) = base.split_once('/')?;

    if type_name.trim().is_empty()
        || subtype.trim().is_empty()
        || subtype.contains('/')
    {
        return None;
    }

    Some(base)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Read, Write};

    use crate::stdlib::data::serialization::{
        SerializationOptions,
        SerializationReport,
    };

    struct TestProvider {
        descriptor: FormatDescriptor,
    }

    impl TestProvider {
        fn new(id: &str) -> Self {
            Self {
                descriptor: FormatDescriptor::new(
                    SerializationFormatId::new(id)
                        .expect("test format identifier is valid"),
                    "Test format",
                )
                .with_media_type("application/x-zamani-test")
                .with_streaming(true),
            }
        }

        fn copy_stream(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> Result<SerializationReport> {
            let bytes = std::io::copy(source, destination)
                .map_err(|error| {
                    crate::stdlib::error::Error::new(
                        crate::stdlib::error::ErrorKind::Io,
                        "test provider stream operation failed",
                    )
                    .with_source(error)
                    .in_component("stdlib.data.formats.tests")
                })?;

            Ok(SerializationReport::new(
                self.descriptor.id.clone(),
                bytes,
                bytes,
                options.schema.clone(),
            ))
        }
    }

    impl SerializationProvider for TestProvider {
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

    #[test]
    fn creates_an_empty_catalog() {
        let catalog = FormatCatalog::new();

        assert_eq!(
            catalog.provider_count().expect("registry is accessible"),
            0
        );
    }

    #[test]
    fn registers_and_resolves_a_provider() {
        let catalog = FormatCatalog::new();

        let provider = TestProvider::new("test.format");

        catalog
            .register(provider)
            .expect("provider registration succeeds");

        let id = SerializationFormatId::new("test.format")
            .expect("format identifier is valid");

        assert!(catalog.contains(&id).expect("lookup succeeds"));

        let resolved = catalog.resolve(&id)
            .expect("provider resolves");

        assert_eq!(resolved.descriptor().id, id);
    }

    #[test]
    fn exposes_registered_format_descriptors() {
        let catalog = FormatCatalog::new();

        catalog
            .register(TestProvider::new("test.descriptor"))
            .expect("provider registration succeeds");

        let id = SerializationFormatId::new("test.descriptor")
            .expect("format identifier is valid");

        let descriptor = catalog
            .descriptor(&id)
            .expect("descriptor lookup succeeds");

        assert_eq!(descriptor.display_name, "Test format");
        assert!(descriptor.streaming);
    }

    #[test]
    fn rejects_duplicate_format_registration() {
        let catalog = FormatCatalog::new();

        catalog
            .register(TestProvider::new("test.duplicate"))
            .expect("first registration succeeds");

        let result = catalog.register(TestProvider::new("test.duplicate"));

        assert!(result.is_err());
    }

    #[test]
    fn unregister_removes_future_lookup_access() {
        let catalog = FormatCatalog::new();

        catalog
            .register(TestProvider::new("test.removal"))
            .expect("provider registration succeeds");

        let id = SerializationFormatId::new("test.removal")
            .expect("format identifier is valid");

        let existing_handle = catalog
            .resolve(&id)
            .expect("provider resolves before removal");

        catalog
            .unregister(&id)
            .expect("provider removal succeeds");

        assert!(!catalog.contains(&id).expect("lookup succeeds"));

        // Previously resolved handles remain valid.
        assert_eq!(existing_handle.descriptor().id, id);
    }

    #[test]
    fn media_type_matching_is_case_insensitive() {
        let descriptor = TestProvider::new("test.media").descriptor;

        assert!(supports_media_type(
            &descriptor,
            "APPLICATION/X-ZAMANI-TEST"
        ));
    }

    #[test]
    fn media_type_matching_ignores_parameters() {
        let descriptor = TestProvider::new("test.parameters").descriptor;

        assert!(supports_media_type(
            &descriptor,
            "application/x-zamani-test; charset=utf-8"
        ));
    }

    #[test]
    fn invalid_media_types_do_not_match() {
        let descriptor = TestProvider::new("test.invalid-media").descriptor;

        assert!(!supports_media_type(&descriptor, ""));
        assert!(!supports_media_type(&descriptor, "application"));
        assert!(!supports_media_type(&descriptor, "/subtype"));
        assert!(!supports_media_type(&descriptor, "type/"));
        assert!(!supports_media_type(&descriptor, "type/sub/type"));
    }

    #[test]
    fn catalog_can_share_an_existing_registry() {
        let registry = SerializationRegistry::new();

        let first = FormatCatalog::from_registry(registry.clone());
        let second = FormatCatalog::from_registry(registry);

        first
            .register(TestProvider::new("test.shared"))
            .expect("provider registration succeeds");

        let id = SerializationFormatId::new("test.shared")
            .expect("format identifier is valid");

        assert!(second.contains(&id).expect("shared lookup succeeds"));
    }
}
