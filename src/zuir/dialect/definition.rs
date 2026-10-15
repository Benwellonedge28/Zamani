
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Definitions
//!
//! Path: `src/zuir/dialect/definition.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Defines validated, versioned manifests for ZUIR dialects. A dialect
//! declares its identity, version, exported operations and types, attribute
//! schema, interfaces, dependencies, compatibility requirements, and trust
//! expectations.
//!
//! This module defines declarations, not executable operation behavior.
//! Detailed operation schemas belong to `dialect/operation.rs`; global
//! registration and conflict detection belong to `dialect/registry.rs`.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/version.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/Zamani.g4`
//!
//! ## SCALABILITY AND SAFETY
//!
//! Declarations use dynamically sized collections. No fixed number of
//! operations, types, attributes, interfaces, dependencies, devices, qubits,
//! processors, registers, or other computational resources is imposed.
//!
//! Validation checks declared data, not whether a target has sufficient
//! physical resources. Target feasibility is a downstream responsibility.
//!
//! This module uses safe Rust exclusively. Deserialized values must be
//! validated before registration or use.
//!
//! ## INTEGRATION
//!
//! `dialect/operation.rs` owns detailed operation schemas.
//! `dialect/registry.rs` owns global namespace and dependency resolution.
//! `dialect/versioning.rs` owns migration and version-compatibility actions.
//! `schema/attributes.rs` owns attribute validation.
//! `schema/types.rs` owns schema-level names and types.
//! The verifier owns whole-program semantic validation.
//!
//! ## IMPORTANT
//!
//! A manifest is descriptive data, not authorization to execute extension
//! code. Trust, capabilities, policy approval, and executable implementation
//! loading must be checked independently.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::schema::attributes::{
    AttributeDefinition, AttributeError, AttributeKey, AttributeSchema,
    UnknownAttributePolicy,
};
use crate::zuir::schema::types::{QualifiedName, SchemaType, SchemaTypeError};
use crate::zuir::version::DialectVersion;

// =============================================================================
// Dialect identity and exported declarations
// =============================================================================

/// A validated dialect identifier.
///
/// The identifier is a qualified name, not a source-language keyword or a
/// vendor-specific hardware identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DialectName(QualifiedName);

impl DialectName {
    /// Construct a dialect name from an existing qualified name.
    pub fn new(name: QualifiedName) -> Result<Self, DialectDefinitionError> {
        name.validate()?;
        Ok(Self(name))
    }

    /// Parse a canonical name such as `core`, `quantum`, or `vendor::dialect`.
    pub fn parse(value: &str) -> Result<Self, DialectDefinitionError> {
        Self::new(QualifiedName::parse(value)?)
    }

    /// Return the underlying qualified name.
    #[must_use]
    pub fn qualified_name(&self) -> &QualifiedName {
        &self.0
    }

    /// Return the canonical display representation.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        self.0.canonical_string()
    }

    /// Revalidate invariants after deserialization.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.0.validate()?;
        Ok(())
    }
}

impl fmt::Display for DialectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A qualified export name within a dialect.
///
/// The export name is qualified independently of the dialect name so that
/// references remain unambiguous when multiple dialects are imported.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DialectExportName(QualifiedName);

impl DialectExportName {
    /// Construct a validated export name.
    pub fn new(name: QualifiedName) -> Result<Self, DialectDefinitionError> {
        name.validate()?;
        Ok(Self(name))
    }

    /// Parse a qualified export name.
    pub fn parse(value: &str) -> Result<Self, DialectDefinitionError> {
        Self::new(QualifiedName::parse(value)?)
    }

    /// Return the qualified name.
    #[must_use]
    pub fn qualified_name(&self) -> &QualifiedName {
        &self.0
    }

    /// Return the canonical name.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        self.0.canonical_string()
    }

    /// Validate this export name.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.0.validate()?;
        Ok(())
    }
}

impl fmt::Display for DialectExportName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Category of an exported dialect declaration.
///
/// This is a declaration category, not a closed list of executable
/// computational operations. New operation names remain open-ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportKind {
    Operation,
    Type,
    Interface,
}

/// A reference to an export supplied by a dialect.
///
/// Detailed operation schemas and semantic type definitions are owned by
/// their respective modules. This manifest-level reference lets the registry
/// resolve declarations without duplicating those definitions here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DialectExport {
    name: DialectExportName,
    kind: ExportKind,
    /// Optional stable schema identifier or revision label.
    ///
    /// This is descriptive metadata. The registry must resolve it against
    /// the authoritative schema and must not infer executable behavior from
    /// this string.
    schema_revision: Option<String>,
    description: Option<String>,
}

impl DialectExport {
    /// Construct an export declaration.
    pub fn new(
        name: DialectExportName,
        kind: ExportKind,
    ) -> Result<Self, DialectDefinitionError> {
        name.validate()?;
        Ok(Self {
            name,
            kind,
            schema_revision: None,
            description: None,
        })
    }

    /// Attach an explicit schema revision label.
    #[must_use]
    pub fn with_schema_revision(mut self, revision: impl Into<String>) -> Self {
        self.schema_revision = Some(revision.into());
        self
    }

    /// Attach descriptive documentation.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the export name.
    #[must_use]
    pub fn name(&self) -> &DialectExportName {
        &self.name
    }

    /// Return the export category.
    #[must_use]
    pub const fn kind(&self) -> ExportKind {
        self.kind
    }

    /// Return the optional schema revision.
    #[must_use]
    pub fn schema_revision(&self) -> Option<&str> {
        self.schema_revision.as_deref()
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate this declaration independently of registry state.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.name.validate()?;
        validate_optional_text("schema revision", self.schema_revision.as_deref())?;
        validate_optional_text("description", self.description.as_deref())?;
        Ok(())
    }
}

// =============================================================================
// Dependency and compatibility declarations
// =============================================================================

/// Required or optional dependency on another dialect.
///
/// Dependency resolution is performed by the registry. A declaration cannot
/// claim that a dependency is installed merely because it appears here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectDependency {
    dialect: DialectName,
    version: VersionRequirement,
    optional: bool,
}

impl DialectDependency {
    /// Construct a dialect dependency.
    pub fn new(
        dialect: DialectName,
        version: VersionRequirement,
        optional: bool,
    ) -> Result<Self, DialectDefinitionError> {
        dialect.validate()?;
        version.validate()?;

        Ok(Self {
            dialect,
            version,
            optional,
        })
    }

    /// Return the required dialect.
    #[must_use]
    pub fn dialect(&self) -> &DialectName {
        &self.dialect
    }

    /// Return the version requirement.
    #[must_use]
    pub fn version_requirement(&self) -> &VersionRequirement {
        &self.version
    }

    /// Whether absence of this dependency is permitted.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }

    /// Validate this dependency.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.dialect.validate()?;
        self.version.validate()
    }
}

/// Version requirement for a dialect dependency.
///
/// This is intentionally independent of the registry's installed-version
/// state. The registry interprets the requirement against the actual version
/// and the compatibility policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VersionRequirement {
    /// Require the exact version.
    Exact {
        major: u32,
        minor: u32,
        patch: u32,
    },
    /// Require the same major version and at least the stated minor/patch.
    SameMajorAtLeast {
        major: u32,
        minor: u32,
        patch: u32,
    },
    /// Accept any version allowed by the registry's explicit policy.
    RegistryPolicy,
}

impl VersionRequirement {
    /// Create an exact version requirement.
    #[must_use]
    pub const fn exact(major: u32, minor: u32, patch: u32) -> Self {
        Self::Exact { major, minor, patch }
    }

    /// Create a same-major minimum version requirement.
    #[must_use]
    pub const fn same_major_at_least(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self::SameMajorAtLeast { major, minor, patch }
    }

    /// Defer compatibility to the registry's declared policy.
    #[must_use]
    pub const fn registry_policy() -> Self {
        Self::RegistryPolicy
    }

    /// Validate the requirement's internal representation.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        // All numeric components are valid u32 values. Compatibility and
        // migration decisions require an installed version and belong to
        // dialect/versioning.rs or the registry.
        Ok(())
    }
}

/// Compatibility declaration for a dialect manifest.
///
/// This records the ZUIR schema and format versions understood by the
/// dialect. It does not itself perform a migration or authorize interpreting
/// an unsupported version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectCompatibility {
    minimum_zuir_schema: VersionRequirement,
    minimum_zuir_format: VersionRequirement,
    allow_unknown_exports: bool,
}

impl DialectCompatibility {
    /// Construct a compatibility declaration.
    pub fn new(
        minimum_zuir_schema: VersionRequirement,
        minimum_zuir_format: VersionRequirement,
        allow_unknown_exports: bool,
    ) -> Result<Self, DialectDefinitionError> {
        minimum_zuir_schema.validate()?;
        minimum_zuir_format.validate()?;

        Ok(Self {
            minimum_zuir_schema,
            minimum_zuir_format,
            allow_unknown_exports,
        })
    }

    /// Return the minimum schema requirement.
    #[must_use]
    pub const fn minimum_zuir_schema(&self) -> &VersionRequirement {
        &self.minimum_zuir_schema
    }

    /// Return the minimum format requirement.
    #[must_use]
    pub const fn minimum_zuir_format(&self) -> &VersionRequirement {
        &self.minimum_zuir_format
    }

    /// Whether unknown exports may be preserved as inert declarations.
    #[must_use]
    pub const fn allows_unknown_exports(&self) -> bool {
        self.allow_unknown_exports
    }

    /// Validate this compatibility declaration.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.minimum_zuir_schema.validate()?;
        self.minimum_zuir_format.validate()
    }
}

// =============================================================================
// Trust and implementation declarations
// =============================================================================

/// Trust expectation for a dialect implementation.
///
/// This is policy input, not a substitute for sandboxing, capability checks,
/// signature verification, or authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialectTrust {
    /// Declarative metadata only; no executable extension is implied.
    DeclarativeOnly,
    /// Implementation must be explicitly approved by host policy.
    HostApproved,
    /// Implementation must satisfy the host's verified-extension policy.
    VerifiedExtension,
}

/// Description of the implementation boundary for a dialect.
///
/// Backend identifiers are qualified names rather than hard-coded vendor or
/// device enums. Actual lowering support must be resolved downstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectImplementation {
    trust: DialectTrust,
    lowering_interfaces: BTreeSet<QualifiedName>,
    required_capabilities: BTreeSet<QualifiedName>,
}

impl DialectImplementation {
    /// Create an implementation declaration.
    pub fn new(trust: DialectTrust) -> Self {
        Self {
            trust,
            lowering_interfaces: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
        }
    }

    /// Declare an interface required for lowering.
    pub fn require_lowering_interface(
        &mut self,
        interface: QualifiedName,
    ) -> Result<(), DialectDefinitionError> {
        interface.validate()?;
        self.lowering_interfaces.insert(interface);
        Ok(())
    }

    /// Declare a required capability by qualified name.
    pub fn require_capability(
        &mut self,
        capability: QualifiedName,
    ) -> Result<(), DialectDefinitionError> {
        capability.validate()?;
        self.required_capabilities.insert(capability);
        Ok(())
    }

    /// Return the trust expectation.
    #[must_use]
    pub const fn trust(&self) -> DialectTrust {
        self.trust
    }

    /// Return lowering interfaces in deterministic order.
    pub fn lowering_interfaces(&self) -> impl Iterator<Item = &QualifiedName> {
        self.lowering_interfaces.iter()
    }

    /// Return required capabilities in deterministic order.
    pub fn required_capabilities(&self) -> impl Iterator<Item = &QualifiedName> {
        self.required_capabilities.iter()
    }

    /// Validate this implementation declaration.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        for name in &self.lowering_interfaces {
            name.validate()?;
        }
        for name in &self.required_capabilities {
            name.validate()?;
        }
        Ok(())
    }
}

// =============================================================================
// Dialect definition
// =============================================================================

/// Complete manifest for one ZUIR dialect.
///
/// The manifest is declarative. It does not contain arbitrary executable
/// callbacks, dynamically loaded code, or backend-specific instructions.
///
/// Export collections are keyed by canonical qualified name. Duplicate
/// exports are rejected at insertion time. After deserialization, call
/// [`DialectDefinition::validate`] before using the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectDefinition {
    name: DialectName,
    version: DialectVersion,
    description: Option<String>,
    exports: BTreeMap<DialectExportName, DialectExport>,
    dependencies: BTreeMap<DialectName, DialectDependency>,
    interfaces: BTreeSet<QualifiedName>,
    attributes: AttributeSchema,
    compatibility: DialectCompatibility,
    implementation: DialectImplementation,
    metadata: BTreeMap<String, String>,
}

impl DialectDefinition {
    /// Construct an empty dialect manifest with explicit compatibility and
    /// trust policy.
    pub fn new(
        name: DialectName,
        version: DialectVersion,
        compatibility: DialectCompatibility,
        implementation: DialectImplementation,
    ) -> Result<Self, DialectDefinitionError> {
        name.validate()?;
        compatibility.validate()?;
        implementation.validate()?;

        Ok(Self {
            name,
            version,
            description: None,
            exports: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            interfaces: BTreeSet::new(),
            attributes: AttributeSchema::new(UnknownAttributePolicy::Reject),
            compatibility,
            implementation,
            metadata: BTreeMap::new(),
        })
    }

    /// Set descriptive documentation for the dialect.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Replace the dialect's attribute schema.
    ///
    /// The schema is validated immediately and again when the full manifest
    /// is validated.
    pub fn set_attribute_schema(
        &mut self,
        attributes: AttributeSchema,
    ) -> Result<(), DialectDefinitionError> {
        attributes.validate()?;
        self.attributes = attributes;
        Ok(())
    }

    /// Add an attribute definition to the dialect's schema.
    ///
    /// Duplicate keys are rejected by the canonical schema implementation.
    pub fn add_attribute(
        &mut self,
        definition: AttributeDefinition,
    ) -> Result<(), DialectDefinitionError> {
        definition.validate()?;
        self.attributes.insert(definition)?;
        Ok(())
    }

    /// Declare an exported operation, type, or interface.
    ///
    /// Detailed operation schemas must be registered separately with the
    /// operation schema owner. This method records the dialect-level export.
    pub fn add_export(
        &mut self,
        export: DialectExport,
    ) -> Result<(), DialectDefinitionError> {
        export.validate()?;
        let key = export.name().clone();

        if self.exports.contains_key(&key) {
            return Err(DialectDefinitionError::DuplicateExport(key));
        }

        self.exports.insert(key, export);
        Ok(())
    }

    /// Declare a dependency on another dialect.
    ///
    /// Self-dependencies are rejected. Longer dependency cycles are detected
    /// by the registry because they require the complete dependency graph.
    pub fn add_dependency(
        &mut self,
        dependency: DialectDependency,
    ) -> Result<(), DialectDefinitionError> {
        dependency.validate()?;

        if dependency.dialect() == &self.name {
            return Err(DialectDefinitionError::SelfDependency(
                self.name.clone(),
            ));
        }

        let key = dependency.dialect().clone();
        if self.dependencies.contains_key(&key) {
            return Err(DialectDefinitionError::DuplicateDependency(key));
        }

        self.dependencies.insert(key, dependency);
        Ok(())
    }

    /// Declare an interface implemented or provided by this dialect.
    pub fn add_interface(
        &mut self,
        interface: QualifiedName,
    ) -> Result<(), DialectDefinitionError> {
        interface.validate()?;
        self.interfaces.insert(interface);
        Ok(())
    }

    /// Add non-semantic descriptive metadata.
    ///
    /// Metadata is inert. It must not be used to bypass verification,
    /// capability authorization, or semantic operation definitions.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), DialectDefinitionError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        self.metadata.insert(key, value);
        Ok(())
    }

    /// Return the dialect name.
    #[must_use]
    pub fn name(&self) -> &DialectName {
        &self.name
    }

    /// Return the dialect version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.version
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return exports in canonical name order.
    pub fn exports(&self) -> impl Iterator<Item = &DialectExport> {
        self.exports.values()
    }

    /// Look up an export by name.
    #[must_use]
    pub fn export(&self, name: &DialectExportName) -> Option<&DialectExport> {
        self.exports.get(name)
    }

    /// Return dependencies in canonical dialect-name order.
    pub fn dependencies(&self) -> impl Iterator<Item = &DialectDependency> {
        self.dependencies.values()
    }

    /// Look up a dependency by dialect name.
    #[must_use]
    pub fn dependency(&self, name: &DialectName) -> Option<&DialectDependency> {
        self.dependencies.get(name)
    }

    /// Return declared interfaces in canonical order.
    pub fn interfaces(&self) -> impl Iterator<Item = &QualifiedName> {
        self.interfaces.iter()
    }

    /// Return the dialect's attribute schema.
    #[must_use]
    pub fn attribute_schema(&self) -> &AttributeSchema {
        &self.attributes
    }

    /// Return the compatibility declaration.
    #[must_use]
    pub const fn compatibility(&self) -> &DialectCompatibility {
        &self.compatibility
    }

    /// Return the implementation declaration.
    #[must_use]
    pub const fn implementation(&self) -> &DialectImplementation {
        &self.implementation
    }

    /// Return descriptive metadata in canonical key order.
    pub fn metadata(&self) -> impl Iterator<Item = (&str, &str)> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Validate the complete manifest, including values produced by
    /// deserialization.
    ///
    /// This validates local invariants only. The registry must additionally
    /// validate dependency resolution, namespace collisions, compatibility
    /// against installed dialects, and trust policy.
    pub fn validate(&self) -> Result<(), DialectDefinitionError> {
        self.name.validate()?;
        validate_optional_text("description", self.description.as_deref())?;
        self.compatibility.validate()?;
        self.implementation.validate()?;
        self.attributes.validate()?;

        for (key, export) in &self.exports {
            export.validate()?;
            if key != export.name() {
                return Err(DialectDefinitionError::ExportKeyMismatch);
            }
        }

        for (key, dependency) in &self.dependencies {
            dependency.validate()?;
            if key != dependency.dialect() {
                return Err(DialectDefinitionError::DependencyKeyMismatch);
            }
            if dependency.dialect() == &self.name {
                return Err(DialectDefinitionError::SelfDependency(
                    self.name.clone(),
                ));
            }
        }

        for interface in &self.interfaces {
            interface.validate()?;
        }

        for key in self.metadata.keys() {
            validate_metadata_key(key)?;
        }

        Ok(())
    }

    /// Return the number of declared exports.
    #[must_use]
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }

    /// Return the number of declared dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Return whether the manifest declares an export with the given kind.
    #[must_use]
    pub fn has_export(
        &self,
        name: &DialectExportName,
        kind: ExportKind,
    ) -> bool {
        self.exports
            .get(name)
            .is_some_and(|export| export.kind() == kind)
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// Structured local validation failures for dialect manifests.
///
/// Registry-wide failures are reported by the registry and must not be
/// disguised as local manifest errors.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DialectDefinitionError {
    #[error("invalid qualified schema name: {0}")]
    InvalidQualifiedName(#[from] SchemaTypeError),

    #[error("invalid attribute declaration: {0}")]
    InvalidAttribute(#[from] AttributeError),

    #[error("duplicate dialect export: {0}")]
    DuplicateExport(DialectExportName),

    #[error("duplicate dialect dependency: {0}")]
    DuplicateDependency(DialectName),

    #[error("dialect cannot depend on itself: {0}")]
    SelfDependency(DialectName),

    #[error("dialect export map key does not match the export declaration")]
    ExportKeyMismatch,

    #[error("dialect dependency map key does not match the dependency declaration")]
    DependencyKeyMismatch,

    #[error("invalid empty {0}")]
    EmptyText(&'static str),

    #[error("invalid {0}: contains a prohibited control character")]
    InvalidControlCharacter(&'static str),

    #[error("invalid metadata key: {0}")]
    InvalidMetadataKey(String),
}

fn validate_optional_text(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), DialectDefinitionError> {
    let Some(value) = value else {
        return Ok(());
    };

    if value.trim().is_empty() {
        return Err(DialectDefinitionError::EmptyText(field));
    }

    if value.chars().any(char::is_control) {
        return Err(DialectDefinitionError::InvalidControlCharacter(field));
    }

    Ok(())
}

fn validate_metadata_key(value: &str) -> Result<(), DialectDefinitionError> {
    if value.is_empty() || value.trim() != value {
        return Err(DialectDefinitionError::InvalidMetadataKey(
            value.to_owned(),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(DialectDefinitionError::InvalidMetadataKey(
            value.to_owned(),
        ));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::version::INITIAL_DIALECT_VERSION;

    fn compatibility() -> DialectCompatibility {
        DialectCompatibility::new(
            VersionRequirement::registry_policy(),
            VersionRequirement::registry_policy(),
            false,
        )
        .expect("valid compatibility declaration")
    }

    fn definition(name: &str) -> DialectDefinition {
        DialectDefinition::new(
            DialectName::parse(name).expect("valid dialect name"),
            INITIAL_DIALECT_VERSION,
            compatibility(),
            DialectImplementation::new(DialectTrust::DeclarativeOnly),
        )
        .expect("valid dialect")
    }

    #[test]
    fn creates_and_validates_empty_dialect() {
        let dialect = definition("core");
        assert!(dialect.validate().is_ok());
        assert_eq!(dialect.export_count(), 0);
        assert_eq!(dialect.dependency_count(), 0);
    }

    #[test]
    fn accepts_open_world_operation_names() {
        let mut dialect = definition("quantum");

        let export = DialectExport::new(
            DialectExportName::parse("quantum::custom_operation")
                .expect("valid export"),
            ExportKind::Operation,
        )
        .expect("valid declaration");

        dialect.add_export(export).expect("unique export");
        assert_eq!(dialect.export_count(), 1);
        assert!(dialect.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_exports() {
        let mut dialect = definition("core");
        let name = DialectExportName::parse("core::calculate")
            .expect("valid export name");

        dialect
            .add_export(
                DialectExport::new(name.clone(), ExportKind::Operation)
                    .expect("valid declaration"),
            )
            .expect("first export");

        let result = dialect.add_export(
            DialectExport::new(name, ExportKind::Operation)
                .expect("valid declaration"),
        );

        assert!(matches!(
            result,
            Err(DialectDefinitionError::DuplicateExport(_))
        ));
    }

    #[test]
    fn rejects_self_dependency() {
        let mut dialect = definition("core");
        let dependency = DialectDependency::new(
            DialectName::parse("core").expect("valid name"),
            VersionRequirement::registry_policy(),
            false,
        )
        .expect("valid dependency");

        assert!(matches!(
            dialect.add_dependency(dependency),
            Err(DialectDefinitionError::SelfDependency(_))
        ));
    }

    #[test]
    fn accepts_symbolic_future_capabilities() {
        let mut implementation =
            DialectImplementation::new(DialectTrust::HostApproved);

        implementation
            .require_capability(
                QualifiedName::parse("future::accelerator_capability")
                    .expect("valid capability"),
            )
            .expect("valid capability declaration");

        assert_eq!(
            implementation.required_capabilities().count(),
            1
        );
        assert!(implementation.validate().is_ok());
    }

    #[test]
    fn rejects_empty_description_during_validation() {
        let mut dialect = definition("core");
        dialect.description = Some("  ".to_owned());

        assert!(matches!(
            dialect.validate(),
            Err(DialectDefinitionError::EmptyText("description"))
        ));
    }

    #[test]
    fn metadata_iteration_is_deterministic() {
        let mut dialect = definition("core");
        dialect
            .insert_metadata("zeta", "2")
            .expect("valid metadata");
        dialect
            .insert_metadata("alpha", "1")
            .expect("valid metadata");

        let keys: Vec<&str> = dialect.metadata().map(|(key, _)| key).collect();
        assert_eq!(keys, vec!["alpha", "zeta"]);
    }
}
