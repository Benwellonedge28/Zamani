
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL Library Manifest
//!
//! Path: `src/hdl/libraries/manifest.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define a validated, versioned TOML format for declarative HDL library
//! metadata and adapt that metadata to the existing HDL library registry.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/versioning.md`
//! - `grammar/spec/compatibility.md`
//! - `grammar/spec/determinism.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/ast/node/paths/import.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/libraries/registry.rs`
//! - `src/hdl/error.rs`
//!
//! The HDL grammar owns source syntax. The frontend AST owns source-level
//! imports. Semantic analysis owns source meaning and name resolution.
//! The registry owns exact library identity matching and in-memory storage.
//! This module owns only the manifest-format contract and its adapter to the
//! existing registry descriptor.
//!
//! The HDL grammar does not currently define this TOML manifest format.
//! This file therefore does not claim that manifest syntax is Zamani source
//! syntax or that the format is already specified by the HDL grammar.
//!
//! ## Owns
//!
//! - The versioned HDL library manifest schema.
//! - Strict TOML deserialization and serialization.
//! - Validation through the existing registry descriptor contract.
//! - Conversion from manifest metadata to `HdlLibraryDescriptor`.
//! - An explicit convenience operation for registering a validated manifest.
//!
//! ## Does not own
//!
//! - HDL source parsing or source-level import syntax.
//! - AST construction, semantic name resolution, or type checking.
//! - Package acquisition, filesystem access, network access, or downloads.
//! - Dependency solving, version-range interpretation, or lockfiles.
//! - Artifact authentication, cryptographic verification, or trust policy.
//! - Persistent registry storage or cross-build identity stability.
//! - Target discovery, synthesis, placement, routing, or hardware allocation.
//! - Any intermediate representation implementation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Manifest format
//!
//! ```toml
//! schema_version = 1
//!
//! [library]
//! namespace = "example.org"
//! name = "logic"
//! version = "1.0.0"
//! origin = "workspace:logic"
//!
//! [[dependencies]]
//! namespace = "example.org"
//! name = "primitives"
//! version_requirement = "^1.0.0"
//! ```
//!
//! `schema_version` versions this manifest structure. It is not the Zamani
//! language version, HDL dialect version, library version, or HDLIR version.
//!
//! The library's `version` and each dependency's `version_requirement` remain
//! strings. Their package-versioning semantics belong to the versioning and
//! dependency-resolution contracts, not this parser.
//!
//! ## Validation and compatibility
//!
//! Unknown fields are rejected rather than silently ignored. Unsupported
//! schema versions are rejected explicitly. Required fields must be present.
//! Identity and dependency validation is delegated to the existing registry
//! types so that the manifest cannot introduce a competing validation model.
//!
//! ## Scalability
//!
//! No universal library-count, dependency-count, hardware-width, or design-size
//! ceiling is introduced. Collections grow according to available resources.
//! TOML parsing and allocation remain subject to the process's actual memory
//! and runtime resources; this module does not promise recovery from allocator
//! failure.
//!
//! ## Integration contract
//!
//! 1. Parse a manifest using `HdlLibraryManifest::from_toml`.
//! 2. Inspect or validate its metadata as needed.
//! 3. Convert it using `to_descriptor`, or call `register_into`.
//! 4. The existing `HdlLibraryRegistry` enforces duplicate identity rules and
//!    allocates the registry-local `HdlLibraryId`.
//!
//! The registry does not depend on this module. This one-way dependency keeps
//! manifest parsing independent of registry implementation details.
//!
//! The library composition module should expose this file as
//! `libraries::manifest` when that module is introduced.
//!
//! ## Freeze criteria
//!
//! - Schema version 1 is covered by positive and negative tests.
//! - Unknown keys and unsupported schema versions fail explicitly.
//! - Invalid identity and dependency metadata is rejected by the registry's
//!   canonical descriptor constructors.
//! - Serialization round-trips preserve the manifest's declared metadata.
//! - Duplicate library registration is reported by the registry.
//! - This module performs no file, network, target, or HDLIR operations.

use serde::{Deserialize, Serialize};

use crate::hdl::error::{HdlError, HdlResult};
use crate::hdl::ids::HdlLibraryId;
use crate::hdl::libraries::registry::{
    HdlLibraryDependency,
    HdlLibraryDescriptor,
    HdlLibraryRegistry,
};

/// Current supported version of the HDL library manifest schema.
///
/// This is independent of the HDL library's own version.
pub const HDL_LIBRARY_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Stable architectural name for this module.
pub const HDL_LIBRARY_MANIFEST_MODULE_NAME: &str = "zamani.hdl.libraries.manifest";

/// Raw TOML representation.
///
/// This private type is deliberately separate from the validated public
/// representation. Deserializing TOML cannot bypass public validation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    schema_version: u32,
    library: RawLibrary,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<RawDependency>,
}

/// Raw library metadata from the `[library]` table.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawLibrary {
    namespace: String,
    name: String,
    version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    origin: Option<String>,
}

/// Raw dependency metadata from a `[[dependencies]]` table.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawDependency {
    namespace: String,
    name: String,
    version_requirement: String,
}

/// Validated identity metadata for the library declared by a manifest.
///
/// The fields are private so callers cannot mutate validated data without
/// passing through the validation contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlLibraryManifestLibrary {
    namespace: String,
    name: String,
    version: String,
    origin: Option<String>,
}

impl HdlLibraryManifestLibrary {
    /// Construct validated library metadata.
    ///
    /// The identity rules are delegated to the canonical registry descriptor.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> HdlResult<Self> {
        let namespace = namespace.into();
        let name = name.into();
        let version = version.into();

        // Reuse the canonical identity validation instead of implementing a
        // second set of rules in the manifest layer.
        HdlLibraryDescriptor::new(
            namespace.clone(),
            name.clone(),
            version.clone(),
        )?;

        Ok(Self {
            namespace,
            name,
            version,
            origin: None,
        })
    }

    /// Return the exact namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the exact library name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the library version string.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the optional opaque origin description.
    #[must_use]
    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    /// Set an optional opaque origin description.
    ///
    /// This value is metadata only. It is never opened or treated as proof
    /// that an artifact exists or is trusted.
    pub fn with_origin(
        mut self,
        origin: impl Into<String>,
    ) -> HdlResult<Self> {
        let origin = origin.into();

        // Delegate origin validation to the existing descriptor contract.
        let descriptor = HdlLibraryDescriptor::new(
            self.namespace.clone(),
            self.name.clone(),
            self.version.clone(),
        )?
        .with_origin(origin.clone())?;

        // Keep the value exactly as accepted by the canonical descriptor.
        self.origin = descriptor.origin().map(str::to_owned);

        Ok(self)
    }
}

/// Validated dependency metadata declared by an HDL library manifest.
///
/// The version requirement remains opaque here. This type does not select a
/// dependency version, download a package, or implement a version solver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlLibraryManifestDependency {
    namespace: String,
    name: String,
    version_requirement: String,
}

impl HdlLibraryManifestDependency {
    /// Construct validated dependency metadata.
    ///
    /// Validation is delegated to the registry's existing dependency type.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version_requirement: impl Into<String>,
    ) -> HdlResult<Self> {
        let namespace = namespace.into();
        let name = name.into();
        let version_requirement = version_requirement.into();

        HdlLibraryDependency::new(
            namespace.clone(),
            name.clone(),
            version_requirement.clone(),
        )?;

        Ok(Self {
            namespace,
            name,
            version_requirement,
        })
    }

    /// Return the dependency namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the dependency name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the opaque version requirement.
    #[must_use]
    pub fn version_requirement(&self) -> &str {
        &self.version_requirement
    }

    fn to_registry_dependency(&self) -> HdlResult<HdlLibraryDependency> {
        HdlLibraryDependency::new(
            self.namespace.clone(),
            self.name.clone(),
            self.version_requirement.clone(),
        )
    }
}

/// Validated, versioned HDL library manifest.
///
/// The manifest describes library metadata and declared dependencies. It does
/// not imply that the dependencies are available, compatible, trusted, or
/// installed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlLibraryManifest {
    schema_version: u32,
    library: HdlLibraryManifestLibrary,
    dependencies: Vec<HdlLibraryManifestDependency>,
}

impl HdlLibraryManifest {
    /// Construct a new manifest using the currently supported schema version.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> HdlResult<Self> {
        let manifest = Self {
            schema_version: HDL_LIBRARY_MANIFEST_SCHEMA_VERSION,
            library: HdlLibraryManifestLibrary::new(namespace, name, version)?,
            dependencies: Vec::new(),
        };

        manifest.validate()?;
        Ok(manifest)
    }

    /// Parse and validate a TOML manifest.
    ///
    /// This method parses an in-memory string only. It performs no filesystem
    /// access, network access, dependency acquisition, or registration.
    pub fn from_toml(source: &str) -> HdlResult<Self> {
        let raw: RawManifest = toml::from_str(source).map_err(|error| {
            HdlError::invalid_input(
                "hdl.library.manifest.toml",
                format!("could not parse manifest TOML: {error}"),
            )
        })?;

        if raw.schema_version != HDL_LIBRARY_MANIFEST_SCHEMA_VERSION {
            return Err(HdlError::invalid_input(
                "hdl.library.manifest.schema_version",
                format!(
                    "unsupported manifest schema version {}; this implementation supports {}",
                    raw.schema_version,
                    HDL_LIBRARY_MANIFEST_SCHEMA_VERSION,
                ),
            ));
        }

        let mut library = HdlLibraryManifestLibrary::new(
            raw.library.namespace,
            raw.library.name,
            raw.library.version,
        )?;

        if let Some(origin) = raw.library.origin {
            library = library.with_origin(origin)?;
        }

        let mut dependencies = Vec::with_capacity(raw.dependencies.len());

        for dependency in raw.dependencies {
            dependencies.push(HdlLibraryManifestDependency::new(
                dependency.namespace,
                dependency.name,
                dependency.version_requirement,
            )?);
        }

        let manifest = Self {
            schema_version: raw.schema_version,
            library,
            dependencies,
        };

        manifest.validate()?;
        Ok(manifest)
    }

    /// Return the manifest schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the declared library metadata.
    #[must_use]
    pub fn library(&self) -> &HdlLibraryManifestLibrary {
        &self.library
    }

    /// Return dependencies in their declared order.
    #[must_use]
    pub fn dependencies(&self) -> &[HdlLibraryManifestDependency] {
        &self.dependencies
    }

    /// Return whether the manifest declares no dependencies.
    #[must_use]
    pub fn has_no_dependencies(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Attach an optional opaque origin description to the library.
    pub fn with_origin(
        mut self,
        origin: impl Into<String>,
    ) -> HdlResult<Self> {
        self.library = self.library.with_origin(origin)?;
        self.validate()?;
        Ok(self)
    }

    /// Replace the declared dependencies after validation.
    ///
    /// The canonical registry descriptor rejects duplicate dependencies
    /// referring to the same namespace and library name. This method applies
    /// that same rule before returning the new manifest.
    pub fn with_dependencies(
        mut self,
        dependencies: Vec<HdlLibraryManifestDependency>,
    ) -> HdlResult<Self> {
        self.dependencies = dependencies;
        self.validate()?;
        Ok(self)
    }

    /// Validate the manifest against the supported schema and registry
    /// descriptor contract.
    pub fn validate(&self) -> HdlResult<()> {
        if self.schema_version != HDL_LIBRARY_MANIFEST_SCHEMA_VERSION {
            return Err(HdlError::invalid_input(
                "hdl.library.manifest.schema_version",
                format!(
                    "unsupported manifest schema version {}; this implementation supports {}",
                    self.schema_version,
                    HDL_LIBRARY_MANIFEST_SCHEMA_VERSION,
                ),
            ));
        }

        let _ = self.to_descriptor()?;
        Ok(())
    }

    /// Convert this manifest into the canonical registry descriptor.
    ///
    /// Library identity, origin, dependency metadata, and duplicate dependency
    /// checks are delegated to the existing registry types.
    pub fn to_descriptor(&self) -> HdlResult<HdlLibraryDescriptor> {
        let mut descriptor = HdlLibraryDescriptor::new(
            self.library.namespace.clone(),
            self.library.name.clone(),
            self.library.version.clone(),
        )?;

        if let Some(origin) = self.library.origin() {
            descriptor = descriptor.with_origin(origin.to_owned())?;
        }

        let dependencies = self
            .dependencies
            .iter()
            .map(HdlLibraryManifestDependency::to_registry_dependency)
            .collect::<HdlResult<Vec<_>>>()?;

        descriptor.with_dependencies(dependencies)
    }

    /// Register this manifest in an existing in-memory HDL library registry.
    ///
    /// The registry owns duplicate library identity checks and registry-local
    /// ID allocation. This operation does not resolve or fetch dependencies.
    pub fn register_into(
        &self,
        registry: &mut HdlLibraryRegistry,
    ) -> HdlResult<HdlLibraryId> {
        registry.register(self.to_descriptor()?)
    }

    /// Serialize this manifest to TOML.
    ///
    /// Serialization is deterministic for the same manifest value and schema.
    /// It does not create or write a file.
    pub fn to_toml(&self) -> HdlResult<String> {
        self.validate()?;

        let raw = RawManifest {
            schema_version: self.schema_version,
            library: RawLibrary {
                namespace: self.library.namespace.clone(),
                name: self.library.name.clone(),
                version: self.library.version.clone(),
                origin: self.library.origin.clone(),
            },
            dependencies: self
                .dependencies
                .iter()
                .map(|dependency| RawDependency {
                    namespace: dependency.namespace.clone(),
                    name: dependency.name.clone(),
                    version_requirement: dependency.version_requirement.clone(),
                })
                .collect(),
        };

        toml::to_string_pretty(&raw).map_err(|error| {
            HdlError::invalid_input(
                "hdl.library.manifest.serialization",
                format!("could not serialize manifest to TOML: {error}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_MANIFEST: &str = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
"#;

    #[test]
    fn parses_minimal_manifest() {
        let manifest =
            HdlLibraryManifest::from_toml(MINIMAL_MANIFEST).expect("valid manifest");

        assert_eq!(manifest.schema_version(), 1);
        assert_eq!(manifest.library().namespace(), "example.org");
        assert_eq!(manifest.library().name(), "logic");
        assert_eq!(manifest.library().version(), "1.0.0");
        assert!(manifest.has_no_dependencies());
    }

    #[test]
    fn rejects_invalid_toml() {
        let error = HdlLibraryManifest::from_toml(
            "schema_version = [not valid TOML",
        )
        .expect_err("malformed TOML must fail");

        assert!(error.message().contains("TOML"));
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        let source = r#"
schema_version = 99

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn rejects_unknown_manifest_fields() {
        let source = r#"
schema_version = 1
unexpected = "not supported"

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn rejects_unknown_library_fields() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
hardware_device = "device-0"
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn rejects_invalid_library_identity() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic name"
version = "1.0.0"
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn parses_dependencies_and_converts_to_descriptor() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
origin = "workspace:logic"

[[dependencies]]
namespace = "example.org"
name = "primitives"
version_requirement = "^1.0.0"
"#;

        let manifest =
            HdlLibraryManifest::from_toml(source).expect("valid manifest");
        let descriptor = manifest
            .to_descriptor()
            .expect("descriptor conversion succeeds");

        assert_eq!(descriptor.identity().namespace(), "example.org");
        assert_eq!(descriptor.identity().name(), "logic");
        assert_eq!(descriptor.identity().version(), "1.0.0");
        assert_eq!(descriptor.origin(), Some("workspace:logic"));
        assert_eq!(descriptor.dependencies().len(), 1);
        assert_eq!(
            descriptor.dependencies()[0].version_requirement(),
            "^1.0.0",
        );
    }

    #[test]
    fn rejects_duplicate_dependency_names() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"

[[dependencies]]
namespace = "example.org"
name = "primitives"
version_requirement = "^1.0.0"

[[dependencies]]
namespace = "example.org"
name = "primitives"
version_requirement = "^2.0.0"
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn rejects_invalid_dependency_metadata() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"

[[dependencies]]
namespace = "example.org"
name = "primitives"
version_requirement = ""
"#;

        assert!(HdlLibraryManifest::from_toml(source).is_err());
    }

    #[test]
    fn toml_round_trip_preserves_manifest_metadata() {
        let source = r#"
schema_version = 1

[library]
namespace = "example.org"
name = "logic"
version = "1.0.0"
origin = "workspace:logic"

[[dependencies]]
namespace = "example.org"
name = "primitives"
version_requirement = "^1.0.0"
"#;

        let original =
            HdlLibraryManifest::from_toml(source).expect("valid manifest");
        let serialized = original.to_toml().expect("serialization succeeds");
        let reparsed =
            HdlLibraryManifest::from_toml(&serialized).expect("round-trip parses");

        assert_eq!(original, reparsed);
    }

    #[test]
    fn registration_uses_the_existing_registry_contract() {
        let manifest = HdlLibraryManifest::new(
            "example.org",
            "logic",
            "1.0.0",
        )
        .expect("valid manifest");

        let mut registry = HdlLibraryRegistry::new();
        let id = manifest
            .register_into(&mut registry)
            .expect("first registration succeeds");

        assert_eq!(registry.len(), 1);
        assert!(registry.get_by_id(id).is_some());

        let duplicate_error = manifest
            .register_into(&mut registry)
            .expect_err("duplicate identity must be rejected");

        assert!(!duplicate_error.message().is_empty());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn failed_registration_does_not_replace_existing_library() {
        let original = HdlLibraryManifest::new(
            "example.org",
            "logic",
            "1.0.0",
        )
        .expect("valid manifest");

        let replacement = HdlLibraryManifest::new(
            "example.org",
            "logic",
            "1.0.0",
        )
        .expect("valid manifest")
        .with_origin("different-origin")
        .expect("valid origin");

        let mut registry = HdlLibraryRegistry::new();
        let id = original
            .register_into(&mut registry)
            .expect("registration succeeds");

        assert!(replacement.register_into(&mut registry).is_err());

        let stored = registry.get_by_id(id).expect("original remains registered");
        assert_eq!(stored.origin(), None);
    }

    #[test]
    fn rejects_empty_or_whitespace_origin() {
        let manifest =
            HdlLibraryManifest::new("example.org", "logic", "1.0.0")
                .expect("valid manifest");

        assert!(manifest.clone().with_origin("").is_err());
        assert!(manifest.with_origin("  ").is_err());
    }

    #[test]
    fn manifest_schema_version_is_not_library_version() {
        let manifest =
            HdlLibraryManifest::new("example.org", "logic", "2026.10")
                .expect("valid manifest");

        assert_eq!(
            manifest.schema_version(),
            HDL_LIBRARY_MANIFEST_SCHEMA_VERSION,
        );
        assert_eq!(manifest.library().version(), "2026.10");
    }
}
