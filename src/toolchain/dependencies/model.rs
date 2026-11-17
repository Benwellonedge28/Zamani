
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Model
//!
//! Path: src/toolchain/dependencies/model.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical toolchain-facing representation of package
//! dependency declarations and package coordinates.
//!
//! This module is the foundational data-model boundary consumed by:
//!
//! - dependencies::constraints
//! - dependencies::resolver
//! - dependencies::graph
//! - dependencies::sources
//! - dependencies::features
//! - dependencies::compatibility
//! - dependencies::lockfile
//! - manifest::adapter
//! - packages
//! - registries
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Dependency names and aliases.
//! - Dependency declarations.
//! - Dependency source declarations.
//! - Optional dependency intent.
//! - Version-requirement preservation.
//! - Requested feature names.
//! - Capability requirement identifiers.
//! - Extensible dependency roles.
//! - Dependency metadata.
//! - Package coordinates.
//!
//! This module does not own:
//!
//! - TOML parsing.
//! - Source-language parsing.
//! - Semantic analysis.
//! - Version requirement interpretation.
//! - Version resolution.
//! - Registry access.
//! - Filesystem access.
//! - Dependency graph algorithms.
//! - Feature unification.
//! - Package downloading.
//! - Package installation.
//! - Package signing.
//! - Hardware discovery.
//! - Target selection.
//! - Compiler IR.
//! - Runtime scheduling.
//! - Physical resource allocation.
//!
//! ## Architectural boundaries
//!
//! A dependency is a package-level relationship.
//!
//! It is not a compiler IR dependency, an artifact dependency,
//! a physical resource dependency, or a runtime scheduling edge.
//!
//! Package dependency graphs remain separate from:
//!
//! - ZUIR dependency graphs.
//! - Compiler incremental dependency graphs.
//! - Artifact dependency manifests.
//! - Hardware connectivity graphs.
//! - Distributed execution graphs.
//!
//! ## Versioning
//!
//! Version requirements are retained as opaque, validated text.
//!
//! This module does not assume semantic versioning, a fixed number
//! of version components, or a particular version comparison policy.
//!
//! Interpretation belongs to dependencies::constraints.
//!
//! ## Portability
//!
//! Dependency declarations do not identify a physical target.
//!
//! A package may contain classical, quantum, HDL, AI, hybrid,
//! distributed, networking, cryptographic, or future-domain content.
//!
//! This module does not enumerate those domains.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Dependencies.
//! - Features.
//! - Capabilities.
//! - Metadata entries.
//! - Identifier lengths.
//! - Version component counts.
//! - Package coordinates.
//! - Computing domains.
//!
//! Collections grow according to available memory and allocator
//! capacity. Actual operations remain subject to operating-system
//! limits and explicitly configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Sets and maps use BTreeSet and BTreeMap.
//!
//! Equivalent validated declarations therefore have deterministic
//! collection iteration order.
//!
//! This does not claim that dependency resolution itself is
//! deterministic. That guarantee belongs to the resolver.
//!
//! ## Security
//!
//! This module performs no I/O, network access, process execution,
//! environment inspection, credential access, or expression evaluation.
//!
//! Paths are preserved as declarations. Their canonicalization,
//! authorization, containment and existence checks belong to
//! dependencies::sources.
//!
//! External strings are validated without silent normalization.
//!
//! ## Freeze contract
//!
//! Downstream files may depend on these public types and accessors.
//!
//! New dependency source schemes and metadata conventions must be
//! introduced through extensible identifiers rather than closed
//! vendor-specific enumerations.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_MODEL: &str = "dependency.model";
const OPERATION_VALIDATION: &str = "dependency.model.validate";

// ============================================================================
// SHARED IDENTIFIER VALIDATION
// ============================================================================

/// Validate nonempty externally supplied identifier text.
///
/// No arbitrary identifier-length ceiling is imposed.
///
/// Leading and trailing whitespace is rejected instead of silently
/// trimming or changing the supplied identifier.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    if value.trim() != value {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain leading or trailing whitespace"
                ),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not contain control characters"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    Ok(())
}

/// Validate an identifier that represents a qualified textual name.
///
/// The model intentionally does not implement a competing qualified-name
/// grammar. The canonical frontend/grammar remains authoritative for
/// source-language qualified names.
///
/// This check only protects the toolchain boundary from malformed text.
fn validate_name(
    value: &str,
    field: &'static str,
) -> Result<()> {
    validate_identifier(value, field)?;

    if value.contains('\0') {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} contains a NUL character"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    Ok(())
}

// ============================================================================
// DEPENDENCY NAME
// ============================================================================

/// Canonical textual name of a dependency.
///
/// This is a package/module-facing identifier, not a filesystem path,
/// registry URL, physical device identifier, or compiler IR value.
///
/// Qualified-name syntax remains owned by the canonical grammar.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyName(String);

impl DependencyName {
    /// Construct a validated dependency name.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_name(&value, "dependency name")?;

        Ok(Self(value))
    }

    /// Borrow the canonical dependency name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the dependency name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for DependencyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for DependencyName {
    type Error = ToolchainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for DependencyName {
    type Error = ToolchainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

// ============================================================================
// DEPENDENCY ALIAS
// ============================================================================

/// Optional local name used to refer to a dependency.
///
/// An alias does not change the identity of the referenced package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyAlias(String);

impl DependencyAlias {
    /// Construct a validated dependency alias.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_name(&value, "dependency alias")?;

        Ok(Self(value))
    }

    /// Borrow the alias.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencyAlias {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXTENSIBLE DEPENDENCY ROLE
// ============================================================================

/// Extensible classification of a dependency relationship.
///
/// The value is deliberately not a closed enum.
///
/// Examples may include:
///
/// - `normal`
/// - `development`
/// - `build`
/// - `interface`
/// - `tooling`
/// - Future ecosystem-specific roles.
///
/// The dependency resolver and manifest policy determine the meaning
/// of registered roles.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyRole(String);

impl DependencyRole {
    /// Construct a dependency role.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "dependency role")?;

        Ok(Self(value))
    }

    /// Borrow the role identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencyRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// VERSION REQUIREMENT
// ============================================================================

/// Opaque package-version requirement.
///
/// This type preserves the exact supplied version expression.
///
/// It deliberately does not parse or compare versions.
///
/// The constraints subsystem owns interpretation and compatibility.
///
/// Examples:
///
/// - `1.0.0`
/// - `>=1`
/// - `^2.0`
/// - A version expression belonging to a registered ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionRequirement(String);

impl VersionRequirement {
    /// Construct a validated version requirement.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "version requirement")?;

        Ok(Self(value))
    }

    /// Borrow the original version requirement.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VersionRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SOURCE IDENTIFIERS
// ============================================================================

/// Extensible identifier for a dependency source scheme.
///
/// Examples include `path`, `registry`, `git`, `url`, and
/// future registered source schemes.
///
/// This identifier does not establish that a source is available
/// or trusted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencySourceKind(String);

impl DependencySourceKind {
    /// Construct a validated source-kind identifier.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "dependency source kind")?;

        Ok(Self(value))
    }

    /// Borrow the source-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencySourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DEPENDENCY SOURCE
// ============================================================================

/// Declared origin of a dependency.
///
/// A source declaration is intent only.
///
/// It does not access the filesystem, resolve a registry, download
/// content, or authorize execution.
///
/// The sources subsystem owns those operations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DependencySource {
    /// A dependency declared using a local filesystem path.
    ///
    /// The path is retained exactly as supplied.
    ///
    /// It is not canonicalized, opened, or checked for existence here.
    Path(PathBuf),

    /// A dependency declared through a named registry.
    ///
    /// `registry` is optional because a configured default registry
    /// may be selected by the source-resolution policy.
    Registry {
        /// Optional registry identity or configured registry name.
        registry: Option<String>,
    },

    /// A dependency declared through a Git repository.
    ///
    /// The repository and reference are preserved without network access.
    Git {
        /// Repository location.
        repository: String,

        /// Optional branch, tag, revision, or registered reference.
        reference: Option<String>,
    },

    /// A dependency declared through a URL-based source.
    ///
    /// URL parsing, transport restrictions and integrity checks belong
    /// to the source-resolution and security subsystems.
    Url {
        /// Original URL text.
        location: String,
    },

    /// A dependency declared through an extensible source provider.
    ///
    /// This permits future source schemes without changing the model.
    Custom {
        /// Registered source scheme.
        kind: DependencySourceKind,

        /// Opaque provider-specific source location.
        location: String,

        /// Optional provider-specific reference.
        reference: Option<String>,
    },
}

impl DependencySource {
    /// Construct a local-path source declaration.
    pub fn path(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();

        if path.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "dependency path must not be empty",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(Self::Path(path))
    }

    /// Construct a registry source declaration.
    pub fn registry(registry: Option<String>) -> Result<Self> {
        if let Some(value) = registry.as_deref() {
            validate_identifier(value, "registry identifier")?;
        }

        Ok(Self::Registry { registry })
    }

    /// Construct a Git source declaration.
    pub fn git(
        repository: impl Into<String>,
        reference: Option<String>,
    ) -> Result<Self> {
        let repository = repository.into();

        validate_identifier(&repository, "Git repository")?;

        if let Some(value) = reference.as_deref() {
            validate_identifier(value, "Git reference")?;
        }

        Ok(Self::Git {
            repository,
            reference,
        })
    }

    /// Construct a URL source declaration.
    ///
    /// This validates structural text only. It does not validate
    /// transport security or authorize network access.
    pub fn url(location: impl Into<String>) -> Result<Self> {
        let location = location.into();

        validate_identifier(&location, "dependency URL")?;

        Ok(Self::Url { location })
    }

    /// Construct a custom registered source declaration.
    pub fn custom(
        kind: DependencySourceKind,
        location: impl Into<String>,
        reference: Option<String>,
    ) -> Result<Self> {
        let location = location.into();

        validate_identifier(&location, "custom dependency location")?;

        if let Some(value) = reference.as_deref() {
            validate_identifier(value, "custom dependency reference")?;
        }

        Ok(Self::Custom {
            kind,
            location,
            reference,
        })
    }

    /// Return whether this is a local-path declaration.
    #[must_use]
    pub const fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    /// Return the declared path, if this is a local-path source.
    #[must_use]
    pub fn as_path(&self) -> Option<&Path> {
        match self {
            Self::Path(path) => Some(path.as_path()),
            _ => None,
        }
    }
}

// ============================================================================
// CAPABILITY REQUIREMENT
// ============================================================================

/// Extensible capability identifier requested by a dependency.
///
/// This is a requirement identifier, not a physical device identity.
///
/// Capability interpretation belongs to the canonical capability
/// and compatibility authorities.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyCapability(String);

impl DependencyCapability {
    /// Construct a validated capability identifier.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_name(&value, "dependency capability")?;

        Ok(Self(value))
    }

    /// Borrow the capability identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencyCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DEPENDENCY METADATA
// ============================================================================

/// Namespaced metadata associated with a dependency declaration.
///
/// Metadata values are preserved as strings at this foundational
/// boundary. Structured metadata formats may be introduced by a
/// separately versioned metadata contract.
///
/// Metadata is not interpreted as executable code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyMetadata {
    key: String,
    value: String,
}

impl DependencyMetadata {
    /// Construct a metadata entry.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(&key, "dependency metadata key")?;

        if value.chars().any(char::is_control) {
            return Err(
                ToolchainError::invalid_input(
                    "dependency metadata value contains control characters",
                )
                .with_operation(OPERATION_VALIDATION),
            );
        }

        Ok(Self { key, value })
    }

    /// Borrow the metadata key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Borrow the metadata value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// DEPENDENCY DECLARATION
// ============================================================================

/// Canonical validated declaration of one direct package dependency.
///
/// The declaration records requested intent. It does not identify
/// a resolved package release.
///
/// Construct instances through [`DependencyDeclaration::new`]
/// and its validated builder methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyDeclaration {
    name: DependencyName,
    alias: Option<DependencyAlias>,
    version: Option<VersionRequirement>,
    source: Option<DependencySource>,
    role: Option<DependencyRole>,
    optional: bool,
    requested_features: BTreeSet<String>,
    required_capabilities: BTreeSet<DependencyCapability>,
    metadata: BTreeMap<String, String>,
}

impl DependencyDeclaration {
    /// Create a dependency declaration with a required name.
    ///
    /// Other properties are optional and must be explicitly supplied
    /// when required by the project or resolution policy.
    pub fn new(name: DependencyName) -> Self {
        Self {
            name,
            alias: None,
            version: None,
            source: None,
            role: None,
            optional: false,
            requested_features: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Construct a declaration from textual dependency-name input.
    pub fn named(name: impl Into<String>) -> Result<Self> {
        Ok(Self::new(DependencyName::new(name)?))
    }

    /// Set the dependency alias.
    #[must_use]
    pub fn with_alias(mut self, alias: DependencyAlias) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Set the version requirement.
    #[must_use]
    pub fn with_version(
        mut self,
        version: VersionRequirement,
    ) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the dependency source.
    #[must_use]
    pub fn with_source(mut self, source: DependencySource) -> Self {
        self.source = Some(source);
        self
    }

    /// Set the dependency role.
    #[must_use]
    pub fn with_role(mut self, role: DependencyRole) -> Self {
        self.role = Some(role);
        self
    }

    /// Mark this dependency as optional.
    #[must_use]
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    /// Add a requested feature.
    ///
    /// Duplicate feature declarations are rejected rather than
    /// silently normalized.
    pub fn with_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self> {
        let feature = feature.into();

        validate_identifier(&feature, "dependency feature")?;

        if !self.requested_features.insert(feature.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate dependency feature '{feature}'"),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.name.to_string()),
            );
        }

        Ok(self)
    }

    /// Add several requested features.
    pub fn with_features<I, S>(
        mut self,
        features: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for feature in features {
            self = self.with_feature(feature)?;
        }

        Ok(self)
    }

    /// Add a required capability.
    ///
    /// Duplicate requirements are rejected to expose malformed
    /// or accidentally repeated input.
    pub fn with_capability(
        mut self,
        capability: DependencyCapability,
    ) -> Result<Self> {
        if !self.required_capabilities.insert(capability.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "duplicate dependency capability '{capability}'"
                    ),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.name.to_string()),
            );
        }

        Ok(self)
    }

    /// Add a metadata entry.
    ///
    /// Metadata keys must be unique within one declaration.
    pub fn with_metadata(
        mut self,
        metadata: DependencyMetadata,
    ) -> Result<Self> {
        let key = metadata.key;
        let value = metadata.value;

        if self.metadata.contains_key(&key) {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate dependency metadata key '{key}'"),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.name.to_string()),
            );
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Return the dependency name.
    #[must_use]
    pub fn name(&self) -> &DependencyName {
        &self.name
    }

    /// Return the optional local alias.
    #[must_use]
    pub fn alias(&self) -> Option<&DependencyAlias> {
        self.alias.as_ref()
    }

    /// Return the optional version requirement.
    #[must_use]
    pub fn version(&self) -> Option<&VersionRequirement> {
        self.version.as_ref()
    }

    /// Return the optional source declaration.
    #[must_use]
    pub fn source(&self) -> Option<&DependencySource> {
        self.source.as_ref()
    }

    /// Return the optional dependency role.
    #[must_use]
    pub fn role(&self) -> Option<&DependencyRole> {
        self.role.as_ref()
    }

    /// Return whether the dependency is optional.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }

    /// Return requested features in deterministic order.
    #[must_use]
    pub fn requested_features(&self) -> &BTreeSet<String> {
        &self.requested_features
    }

    /// Return required capabilities in deterministic order.
    #[must_use]
    pub fn required_capabilities(
        &self,
    ) -> &BTreeSet<DependencyCapability> {
        &self.required_capabilities
    }

    /// Return metadata in deterministic key order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return the effective local lookup name.
    ///
    /// The alias is used when present. Otherwise the canonical
    /// dependency name is returned.
    ///
    /// This does not change package identity.
    #[must_use]
    pub fn local_name(&self) -> &str {
        self.alias
            .as_ref()
            .map(DependencyAlias::as_str)
            .unwrap_or_else(|| self.name.as_str())
    }

    /// Validate the complete declaration.
    ///
    /// This verifies structural consistency only.
    ///
    /// It does not resolve versions, access sources, validate provider
    /// capabilities, or determine package compatibility.
    pub fn validate(&self) -> Result<()> {
        validate_name(self.name.as_str(), "dependency name")?;

        if let Some(alias) = &self.alias {
            validate_name(alias.as_str(), "dependency alias")?;
        }

        if let Some(version) = &self.version {
            validate_identifier(
                version.as_str(),
                "dependency version requirement",
            )?;
        }

        if let Some(role) = &self.role {
            validate_identifier(role.as_str(), "dependency role")?;
        }

        for feature in &self.requested_features {
            validate_identifier(feature, "dependency feature")?;
        }

        for capability in &self.required_capabilities {
            validate_name(
                capability.as_str(),
                "dependency capability",
            )?;
        }

        for (key, value) in &self.metadata {
            validate_identifier(key, "dependency metadata key")?;

            if value.chars().any(char::is_control) {
                return Err(
                    ToolchainError::validation_failed(
                        format!(
                            "dependency metadata '{key}' contains \
                             control characters"
                        ),
                    )
                    .with_operation(OPERATION_VALIDATION)
                    .with_subject(self.name.to_string()),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// PACKAGE COORDINATE
// ============================================================================

/// Logical coordinate identifying a package release request.
///
/// The coordinate is independent of physical package storage.
///
/// Version requirements and source declarations remain separate from
/// the package identity allocated by the package-management subsystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageCoordinate {
    name: DependencyName,
    version: Option<VersionRequirement>,
    source: Option<DependencySource>,
}

impl PackageCoordinate {
    /// Construct a package coordinate.
    pub fn new(
        name: DependencyName,
        version: Option<VersionRequirement>,
        source: Option<DependencySource>,
    ) -> Self {
        Self {
            name,
            version,
            source,
        }
    }

    /// Construct a package coordinate from a dependency declaration.
    #[must_use]
    pub fn from_dependency(
        dependency: &DependencyDeclaration,
    ) -> Self {
        Self {
            name: dependency.name.clone(),
            version: dependency.version.clone(),
            source: dependency.source.clone(),
        }
    }

    /// Return the package name.
    #[must_use]
    pub fn name(&self) -> &DependencyName {
        &self.name
    }

    /// Return the requested version.
    #[must_use]
    pub fn version(&self) -> Option<&VersionRequirement> {
        self.version.as_ref()
    }

    /// Return the declared source.
    #[must_use]
    pub fn source(&self) -> Option<&DependencySource> {
        self.source.as_ref()
    }
}

// ============================================================================
// MANIFEST ADAPTER INTEGRATION
// ============================================================================

/// Convert an already-normalized manifest dependency into the
/// foundational dependency declaration.
///
/// The canonical manifest parser remains owned by
/// `crate::zamani_project_config`.
///
/// The manifest adapter remains responsible for normalization.
/// This conversion does not parse TOML or resolve dependencies.
impl TryFrom<&crate::toolchain::manifest::adapter::ManifestDependency>
    for DependencyDeclaration
{
    type Error = ToolchainError;

    fn try_from(
        dependency: &crate::toolchain::manifest::adapter::ManifestDependency,
    ) -> Result<Self> {
        let mut declaration =
            DependencyDeclaration::named(dependency.name())?;

        if let Some(version) = dependency.version_requirement() {
            declaration = declaration.with_version(
                VersionRequirement::new(version)?,
            );
        }

        if let Some(path) = dependency.path() {
            declaration = declaration.with_source(
                DependencySource::path(path.to_path_buf())?,
            );
        }

        declaration = declaration.with_features(
            dependency.features().iter().cloned(),
        )?;

        declaration.validate()?;

        Ok(declaration)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dependency_name(value: &str) -> DependencyName {
        DependencyName::new(value).expect("valid dependency name")
    }

    #[test]
    fn dependency_name_rejects_empty_input() {
        assert!(DependencyName::new("").is_err());
    }

    #[test]
    fn dependency_name_rejects_whitespace_normalization() {
        assert!(DependencyName::new(" example ").is_err());
    }

    #[test]
    fn dependency_name_preserves_qualified_names() {
        let name = dependency_name("org::zamani::quantum");

        assert_eq!(name.as_str(), "org::zamani::quantum");
    }

    #[test]
    fn aliases_do_not_change_package_identity() {
        let declaration = DependencyDeclaration::new(
            dependency_name("org::zamani::math"),
        )
        .with_alias(
            DependencyAlias::new("math").expect("valid alias"),
        );

        assert_eq!(
            declaration.name().as_str(),
            "org::zamani::math",
        );

        assert_eq!(declaration.local_name(), "math");
    }

    #[test]
    fn arbitrary_version_schemes_are_preserved() {
        let original = "vendor-version:alpha.2026.r2";

        let version =
            VersionRequirement::new(original).expect("valid version text");

        assert_eq!(version.as_str(), original);
    }

    #[test]
    fn dependency_features_are_deterministic() {
        let declaration = DependencyDeclaration::new(
            dependency_name("org::zamani::ml"),
        )
        .with_features(["training", "inference", "tensor"])
        .expect("valid features");

        let features: Vec<&str> = declaration
            .requested_features()
            .iter()
            .map(String::as_str)
            .collect();

        assert_eq!(
            features,
            vec!["inference", "tensor", "training"],
        );
    }

    #[test]
    fn duplicate_features_are_rejected() {
        let result = DependencyDeclaration::new(
            dependency_name("org::zamani::ml"),
        )
        .with_features(["training", "training"]);

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_capabilities_are_rejected() {
        let capability = DependencyCapability::new(
            "quantum.measurement",
        )
        .expect("valid capability");

        let result = DependencyDeclaration::new(
            dependency_name("org::zamani::quantum"),
        )
        .with_capability(capability.clone())
        .expect("first capability")
        .with_capability(capability);

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_metadata_keys_are_rejected() {
        let first = DependencyMetadata::new(
            "license",
            "MIT",
        )
        .expect("valid metadata");

        let second = DependencyMetadata::new(
            "license",
            "Apache-2.0",
        )
        .expect("valid metadata");

        let result = DependencyDeclaration::new(
            dependency_name("org::zamani::core"),
        )
        .with_metadata(first)
        .expect("first metadata")
        .with_metadata(second);

        assert!(result.is_err());
    }

    #[test]
    fn local_paths_are_preserved_without_filesystem_access() {
        let original = PathBuf::from("../libraries/math");

        let source = DependencySource::path(original.clone())
            .expect("valid path");

        assert_eq!(source.as_path(), Some(original.as_path()));
    }

    #[test]
    fn custom_sources_are_extensible() {
        let kind = DependencySourceKind::new(
            "future.package.transport",
        )
        .expect("valid source kind");

        let source = DependencySource::custom(
            kind,
            "provider-specific-location",
            Some("revision-42".to_owned()),
        )
        .expect("valid custom source");

        assert!(matches!(source, DependencySource::Custom { .. }));
    }

    #[test]
    fn package_coordinate_preserves_dependency_identity() {
        let declaration = DependencyDeclaration::new(
            dependency_name("org::zamani::classical"),
        )
        .with_version(
            VersionRequirement::new("^1.2").expect("valid version"),
        );

        let coordinate =
            PackageCoordinate::from_dependency(&declaration);

        assert_eq!(
            coordinate.name().as_str(),
            "org::zamani::classical",
        );

        assert_eq!(
            coordinate.version().map(VersionRequirement::as_str),
            Some("^1.2"),
        );
    }

    #[test]
    fn optionality_is_explicit() {
        let declaration = DependencyDeclaration::new(
            dependency_name("org::zamani::optional"),
        )
        .optional(true);

        assert!(declaration.is_optional());
    }

    #[test]
    fn declaration_validation_succeeds_for_valid_input() {
        let declaration = DependencyDeclaration::new(
            dependency_name("org::zamani::hybrid"),
        )
        .with_version(
            VersionRequirement::new(">=1.0").expect("valid version"),
        )
        .with_source(
            DependencySource::registry(None)
                .expect("valid registry source"),
        )
        .with_features(["classical", "quantum"])
        .expect("valid features")
        .with_capability(
            DependencyCapability::new("hybrid.execution")
                .expect("valid capability"),
        )
        .expect("valid capability")
        .with_metadata(
            DependencyMetadata::new("ecosystem", "zamani")
                .expect("valid metadata"),
        )
        .expect("valid metadata");

        declaration.validate().expect("valid declaration");
    }
}
