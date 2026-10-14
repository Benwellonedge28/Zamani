#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Extensions
//!
//! Canonical path: `src/zuir/schema/extensions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Defines the declarative contracts by which independently developed
//! extensions describe their identity, version, compatibility requirements,
//! dependencies, and contributions to ZUIR schemas.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! The grammar owns source syntax. Semantic analysis owns resolved program
//! meaning. This module owns declarative extension contracts only.
//!
//! OWNS
//!
//! - Extension identity and version requirements.
//! - Extension manifests and compatibility declarations.
//! - Extension dependency and contribution descriptions.
//! - Structural manifest validation.
//! - Deterministic dependency and contribution ordering.
//! - Structured extension-contract errors.
//!
//! DOES NOT OWN
//!
//! - Source syntax, AST nodes, name resolution, or semantic type checking.
//! - Runtime loading, executable plugin code, or sandbox implementation.
//! - Namespace authorization, signatures, trust, or access control.
//! - Dialect registration or operation execution.
//! - Complete-program serialization or canonical hashing.
//! - Target selection, physical resource allocation, or lowering.
//!
//! INTEGRATION
//!
//! Uses `schema::types::QualifiedName` and no concrete IR node.
//! The dialect/extension registry consumes validated manifests, checks
//! namespace authorization and dependency resolution, and registers actual
//! schemas and trusted hooks separately.
//!
//! SCALABILITY
//!
//! No universal extension-count, operation-count, device-count, qubit-count,
//! register-width, tensor-rank, or topology-size ceiling is imposed.
//! Collections are dynamically sized. Callers processing untrusted manifests
//! must apply configured byte, nesting, collection, and execution budgets.
//!
//! SAFETY
//!
//! Safe Rust only. A valid manifest is descriptive data, not authorization
//! to execute code or access resources.
//!
//! VALIDATION BOUNDARY
//!
//! Serde deserialization can construct values without running constructors.
//! Consumers MUST call `validate()` on untrusted or decoded manifests.
//! Local validation cannot establish trust, verify signatures, resolve
//! dependencies, or establish that an extension is safe to execute.
//!
//! FREEZE CRITERIA
//!
//! - All identity and version invariants are explicit.
//! - Duplicate dependencies and contributions are rejected.
//! - Unknown future contribution kinds remain representable.
//! - Invalid decoded manifests are rejected by explicit validation.
//! - Extension identity does not imply namespace authorization.
//! - No fixed hardware limits or unsafe implementation are introduced.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

use super::types::{QualifiedName, SchemaTypeError};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating extension contracts.
///
/// These errors are independent of diagnostic rendering, the IR verifier,
/// extension loading, and runtime execution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExtensionError {
    /// A qualified name or one of its components is invalid.
    InvalidName(SchemaTypeError),

    /// The extension has no declared identity.
    EmptyIdentity,

    /// A version requirement has an invalid range.
    InvalidVersionRequirement {
        /// Explanation of the invalid requirement.
        reason: &'static str,
    },

    /// A dependency refers to the extension itself.
    SelfDependency(QualifiedName),

    /// The manifest contains the same dependency more than once.
    DuplicateDependency(QualifiedName),

    /// The manifest contains the same contribution more than once.
    DuplicateContribution {
        /// Contribution category.
        kind: ContributionKind,
        /// Qualified contribution name.
        name: QualifiedName,
    },

    /// The declared namespace is not a child of the extension identity.
    ContributionOutsideNamespace {
        /// Extension identity.
        extension: QualifiedName,
        /// Invalid contribution.
        contribution: QualifiedName,
    },

    /// The manifest declares an incompatible version range.
    InvalidCompatibility {
        /// Explanation of the invalid compatibility declaration.
        reason: &'static str,
    },

    /// The manifest's format version is unsupported.
    UnsupportedManifestFormat {
        /// Encountered format version.
        found: u32,
    },

    /// A required textual field is empty or invalid.
    InvalidText {
        /// Name of the field.
        field: &'static str,
    },

    /// A numeric field cannot be represented or incremented safely.
    NumericOverflow {
        /// Name of the field.
        field: &'static str,
    },
}

impl fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => {
                write!(f, "invalid extension name: {error:?}")
            }
            Self::EmptyIdentity => {
                f.write_str("extension identity must not be empty")
            }
            Self::InvalidVersionRequirement { reason } => {
                write!(f, "invalid version requirement: {reason}")
            }
            Self::SelfDependency(name) => {
                write!(f, "extension cannot depend on itself: {name}")
            }
            Self::DuplicateDependency(name) => {
                write!(f, "duplicate extension dependency: {name}")
            }
            Self::DuplicateContribution { kind, name } => {
                write!(f, "duplicate {kind:?} contribution: {name}")
            }
            Self::ContributionOutsideNamespace {
                extension,
                contribution,
            } => {
                write!(
                    f,
                    "contribution {contribution} is outside extension namespace {extension}"
                )
            }
            Self::InvalidCompatibility { reason } => {
                write!(f, "invalid extension compatibility: {reason}")
            }
            Self::UnsupportedManifestFormat { found } => {
                write!(f, "unsupported extension manifest format: {found}")
            }
            Self::InvalidText { field } => {
                write!(f, "invalid or empty extension field: {field}")
            }
            Self::NumericOverflow { field } => {
                write!(f, "numeric overflow in extension field: {field}")
            }
        }
    }
}

impl std::error::Error for ExtensionError {}

impl From<SchemaTypeError> for ExtensionError {
    fn from(value: SchemaTypeError) -> Self {
        Self::InvalidName(value)
    }
}

// =============================================================================
// Versioning
// =============================================================================

/// An extension version using a three-component semantic version.
///
/// Versions are ordered lexicographically by major, minor, then patch.
/// Version components are unbounded by hardware-specific policy but are
/// represented by `u64`, so values outside that representation are rejected
/// during parsing rather than truncated.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ExtensionVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ExtensionVersion {
    /// Construct a version from its components.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }

    /// Parse exactly `major.minor.patch`.
    ///
    /// Leading/trailing whitespace, empty components, signs, suffixes,
    /// missing components, and extra components are rejected.
    pub fn parse(value: &str) -> Result<Self, ExtensionError> {
        let mut parts = value.split('.');

        let major = parse_version_component(parts.next(), "major")?;
        let minor = parse_version_component(parts.next(), "minor")?;
        let patch = parse_version_component(parts.next(), "patch")?;

        if parts.next().is_some() {
            return Err(ExtensionError::InvalidText {
                field: "version",
            });
        }

        Ok(Self::new(major, minor, patch))
    }
}

fn parse_version_component(
    value: Option<&str>,
    field: &'static str,
) -> Result<u64, ExtensionError> {
    let value = value.ok_or(ExtensionError::InvalidText { field })?;

    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(ExtensionError::InvalidText { field });
    }

    value
        .parse::<u64>()
        .map_err(|_| ExtensionError::NumericOverflow { field })
}

impl fmt::Display for ExtensionVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A requirement imposed on the version of a dependency.
///
/// This deliberately implements a small, stable subset of version matching.
/// It does not pretend to implement every package-manager version syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum VersionRequirement {
    /// The dependency must have exactly this version.
    Exact(ExtensionVersion),

    /// The dependency must have the same major version and a version at
    /// least as new as the specified version.
    CompatibleMajor(ExtensionVersion),

    /// The dependency must be at least the specified version.
    AtLeast(ExtensionVersion),

    /// The dependency must not be newer than the specified version.
    AtMost(ExtensionVersion),

    /// Any version is accepted, subject to the consumer's trust and policy.
    Any,
}

impl VersionRequirement {
    /// Validate this requirement.
    pub fn validate(self) -> Result<(), ExtensionError> {
        // Every current variant has a well-formed representation.
        // Keep this method as the central validation boundary so future
        // variants can add constraints without changing callers.
        Ok(())
    }

    /// Check whether a candidate version satisfies the requirement.
    #[must_use]
    pub fn matches(self, candidate: ExtensionVersion) -> bool {
        match self {
            Self::Exact(required) => candidate == required,
            Self::CompatibleMajor(required) => {
                candidate.major() == required.major() && candidate >= required
            }
            Self::AtLeast(required) => candidate >= required,
            Self::AtMost(maximum) => candidate <= maximum,
            Self::Any => true,
        }
    }
}

// =============================================================================
// Compatibility declarations
// =============================================================================

/// Compatibility requirement for the ZUIR schema contract.
///
/// This is independent of the Zamani source-language edition and independent
/// of a particular target's capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaCompatibility {
    minimum: ExtensionVersion,
    maximum: Option<ExtensionVersion>,
}

impl SchemaCompatibility {
    /// Create a compatibility interval.
    ///
    /// Both bounds are inclusive. If no maximum is specified, the interval
    /// has no declared upper bound.
    pub fn new(
        minimum: ExtensionVersion,
        maximum: Option<ExtensionVersion>,
    ) -> Result<Self, ExtensionError> {
        if maximum.is_some_and(|upper| upper < minimum) {
            return Err(ExtensionError::InvalidCompatibility {
                reason: "maximum version is below minimum version",
            });
        }

        Ok(Self { minimum, maximum })
    }

    /// Create an interval that accepts one exact version.
    #[must_use]
    pub const fn exact(version: ExtensionVersion) -> Self {
        Self {
            minimum: version,
            maximum: Some(version),
        }
    }

    /// Return the inclusive lower bound.
    #[must_use]
    pub const fn minimum(self) -> ExtensionVersion {
        self.minimum
    }

    /// Return the optional inclusive upper bound.
    #[must_use]
    pub const fn maximum(self) -> Option<ExtensionVersion> {
        self.maximum
    }

    /// Check a version against this interval.
    #[must_use]
    pub fn accepts(self, candidate: ExtensionVersion) -> bool {
        candidate >= self.minimum
            && self.maximum.is_none_or(|upper| candidate <= upper)
    }

    /// Revalidate a decoded value.
    pub fn validate(self) -> Result<(), ExtensionError> {
        if self.maximum.is_some_and(|upper| upper < self.minimum) {
            return Err(ExtensionError::InvalidCompatibility {
                reason: "maximum version is below minimum version",
            });
        }
        Ok(())
    }
}

// =============================================================================
// Contribution declarations
// =============================================================================

/// The category of a contribution made by an extension.
///
/// These categories describe schema-level contributions. They do not form
/// a closed list of computational operations. New operation names and
/// domain-specific entities are expressed by their qualified names.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ContributionKind {
    /// Operation schema.
    Operation,

    /// Schema-level or semantic type declaration.
    Type,

    /// Attribute schema.
    Attribute,

    /// Metadata schema.
    Metadata,

    /// Operation interface or reusable contract.
    Interface,

    /// Region or control-flow contract.
    RegionContract,

    /// Dialect-specific analysis declaration.
    Analysis,

    /// Transformation or lowering declaration.
    Lowering,

    /// An extension-defined contribution not yet understood by this version.
    ///
    /// This value is descriptive only. It does not authorize execution.
    Custom,
}

/// One named contribution to a ZUIR schema or dialect.
///
/// `name` must be qualified and must fall under the extension's declared
/// namespace. The registry remains responsible for checking collisions,
/// reserved namespaces, registration privileges, and actual schema validity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExtensionContribution {
    kind: ContributionKind,
    name: QualifiedName,
    schema_version: ExtensionVersion,
    required: bool,
}

impl ExtensionContribution {
    /// Construct a contribution declaration.
    pub fn new(
        kind: ContributionKind,
        name: QualifiedName,
        schema_version: ExtensionVersion,
        required: bool,
    ) -> Result<Self, ExtensionError> {
        name.validate()?;

        Ok(Self {
            kind,
            name,
            schema_version,
            required,
        })
    }

    /// Return the contribution category.
    #[must_use]
    pub const fn kind(&self) -> ContributionKind {
        self.kind
    }

    /// Return the qualified contribution name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the contribution's schema version.
    #[must_use]
    pub const fn schema_version(&self) -> ExtensionVersion {
        self.schema_version
    }

    /// Whether consumers must understand this contribution to accept the
    /// manifest for the requested use.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Validate the contribution's local invariants.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        self.name.validate()?;
        Ok(())
    }
}

// =============================================================================
// Dependency declarations
// =============================================================================

/// A dependency on another extension.
///
/// Dependencies are data. Resolution, cycle detection, version negotiation,
/// trust evaluation, and load ordering belong to the extension registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDependency {
    extension: QualifiedName,
    requirement: VersionRequirement,
    optional: bool,
}

impl ExtensionDependency {
    /// Construct a dependency declaration.
    pub fn new(
        extension: QualifiedName,
        requirement: VersionRequirement,
        optional: bool,
    ) -> Result<Self, ExtensionError> {
        extension.validate()?;
        requirement.validate()?;

        Ok(Self {
            extension,
            requirement,
            optional,
        })
    }

    /// Return the required extension identity.
    #[must_use]
    pub fn extension(&self) -> &QualifiedName {
        &self.extension
    }

    /// Return the version requirement.
    #[must_use]
    pub const fn requirement(&self) -> VersionRequirement {
        self.requirement
    }

    /// Whether the dependency may be absent.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }

    /// Validate the dependency locally.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        self.extension.validate()?;
        self.requirement.validate()
    }
}

// =============================================================================
// Extension manifest
// =============================================================================

/// The format version of this manifest structure.
///
/// Increment when the serialized manifest contract changes incompatibly.
/// This is not the extension's own semantic version.
pub const EXTENSION_MANIFEST_FORMAT_VERSION: u32 = 1;

/// A declarative extension manifest.
///
/// The manifest describes what an extension claims to provide. It does not
/// establish that the extension is trusted, that its dependencies exist, or
/// that its contributions are valid executable implementations.
///
/// The dependency and contribution collections are sorted by their stable
/// derived ordering during validation. They are vectors rather than maps
/// keyed by structured names, so a serializer does not need to encode
/// `QualifiedName` as a JSON object key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionManifest {
    format_version: u32,
    identity: QualifiedName,
    version: ExtensionVersion,
    compatibility: SchemaCompatibility,
    dependencies: Vec<ExtensionDependency>,
    contributions: Vec<ExtensionContribution>,
    description: Option<String>,
}

impl ExtensionManifest {
    /// Construct a manifest with empty dependencies and contributions.
    pub fn new(
        identity: QualifiedName,
        version: ExtensionVersion,
        compatibility: SchemaCompatibility,
    ) -> Result<Self, ExtensionError> {
        identity.validate()?;
        compatibility.validate()?;

        Ok(Self {
            format_version: EXTENSION_MANIFEST_FORMAT_VERSION,
            identity,
            version,
            compatibility,
            dependencies: Vec::new(),
            contributions: Vec::new(),
            description: None,
        })
    }

    /// Return the manifest serialization format version.
    #[must_use]
    pub const fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Return the extension's qualified identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Return the extension's own version.
    #[must_use]
    pub const fn version(&self) -> ExtensionVersion {
        self.version
    }

    /// Return the declared ZUIR schema compatibility interval.
    #[must_use]
    pub const fn compatibility(&self) -> SchemaCompatibility {
        self.compatibility
    }

    /// Return dependencies in their declared order.
    ///
    /// Call `validate()` before relying on uniqueness.
    #[must_use]
    pub fn dependencies(&self) -> &[ExtensionDependency] {
        &self.dependencies
    }

    /// Return contributions in their declared order.
    ///
    /// Call `validate()` before relying on uniqueness and namespace
    /// consistency.
    #[must_use]
    pub fn contributions(&self) -> &[ExtensionContribution] {
        &self.contributions
    }

    /// Return the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set a human-readable description.
    ///
    /// Description text is descriptive metadata. It must never be treated
    /// as executable code or as a source of authorization.
    pub fn set_description(
        &mut self,
        description: Option<String>,
    ) -> Result<(), ExtensionError> {
        if description.as_ref().is_some_and(|text| text.trim().is_empty()) {
            return Err(ExtensionError::InvalidText {
                field: "description",
            });
        }

        self.description = description;
        Ok(())
    }

    /// Add a dependency, rejecting duplicate identities and self-dependency.
    pub fn add_dependency(
        &mut self,
        dependency: ExtensionDependency,
    ) -> Result<(), ExtensionError> {
        dependency.validate()?;

        if dependency.extension == self.identity {
            return Err(ExtensionError::SelfDependency(
                dependency.extension.clone(),
            ));
        }

        if self
            .dependencies
            .iter()
            .any(|existing| existing.extension == dependency.extension)
        {
            return Err(ExtensionError::DuplicateDependency(
                dependency.extension.clone(),
            ));
        }

        self.dependencies.push(dependency);
        self.dependencies.sort_by(|left, right| {
            left.extension
                .cmp(&right.extension)
                .then_with(|| left.optional.cmp(&right.optional))
        });

        Ok(())
    }

    /// Add a contribution, rejecting duplicate kind/name pairs and names
    /// outside the extension's namespace.
    pub fn add_contribution(
        &mut self,
        contribution: ExtensionContribution,
    ) -> Result<(), ExtensionError> {
        contribution.validate()?;

        if !is_within_namespace(&self.identity, contribution.name()) {
            return Err(ExtensionError::ContributionOutsideNamespace {
                extension: self.identity.clone(),
                contribution: contribution.name().clone(),
            });
        }

        if self.contributions.iter().any(|existing| {
            existing.kind == contribution.kind && existing.name == contribution.name
        }) {
            return Err(ExtensionError::DuplicateContribution {
                kind: contribution.kind,
                name: contribution.name.clone(),
            });
        }

        self.contributions.push(contribution);
        self.contributions.sort();

        Ok(())
    }

    /// Validate all manifest invariants.
    ///
    /// This method MUST be called after deserializing a manifest from an
    /// untrusted source. Constructors alone cannot protect that boundary.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        if self.format_version != EXTENSION_MANIFEST_FORMAT_VERSION {
            return Err(ExtensionError::UnsupportedManifestFormat {
                found: self.format_version,
            });
        }

        self.identity.validate()?;
        self.compatibility.validate()?;

        if self
            .description
            .as_ref()
            .is_some_and(|description| description.trim().is_empty())
        {
            return Err(ExtensionError::InvalidText {
                field: "description",
            });
        }

        let mut dependency_names = BTreeSet::new();

        for dependency in &self.dependencies {
            dependency.validate()?;

            if dependency.extension == self.identity {
                return Err(ExtensionError::SelfDependency(
                    dependency.extension.clone(),
                ));
            }

            if !dependency_names.insert(dependency.extension.clone()) {
                return Err(ExtensionError::DuplicateDependency(
                    dependency.extension.clone(),
                ));
            }
        }

        let mut contribution_names = BTreeSet::new();

        for contribution in &self.contributions {
            contribution.validate()?;

            if !is_within_namespace(&self.identity, contribution.name()) {
                return Err(ExtensionError::ContributionOutsideNamespace {
                    extension: self.identity.clone(),
                    contribution: contribution.name().clone(),
                });
            }

            let key = (contribution.kind, contribution.name.clone());

            if !contribution_names.insert(key) {
                return Err(ExtensionError::DuplicateContribution {
                    kind: contribution.kind,
                    name: contribution.name.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Return whether `candidate` is the extension identity itself or is nested
/// beneath it at a qualified-name segment boundary.
///
/// Segment comparison avoids unsafe string-prefix checks such as treating
/// `vendor::math_extra` as part of `vendor::math`.
fn is_within_namespace(
    namespace: &QualifiedName,
    candidate: &QualifiedName,
) -> bool {
    let namespace_segments = namespace.segments();
    let candidate_segments = candidate.segments();

    candidate_segments.len() >= namespace_segments.len()
        && candidate_segments
            .iter()
            .zip(namespace_segments.iter())
            .all(|(candidate, namespace)| candidate == namespace)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid qualified name")
    }

    fn compatibility() -> SchemaCompatibility {
        SchemaCompatibility::new(
            ExtensionVersion::new(1, 0, 0),
            Some(ExtensionVersion::new(2, 0, 0)),
        )
        .expect("valid compatibility range")
    }

    fn manifest() -> ExtensionManifest {
        ExtensionManifest::new(
            name("vendor_example::math"),
            ExtensionVersion::new(1, 2, 0),
            compatibility(),
        )
        .expect("valid manifest")
    }

    #[test]
    fn parses_strict_three_component_versions() {
        assert_eq!(
            ExtensionVersion::parse("1.2.3").expect("valid version"),
            ExtensionVersion::new(1, 2, 3)
        );

        assert!(ExtensionVersion::parse("1.2").is_err());
        assert!(ExtensionVersion::parse("1.2.3.4").is_err());
        assert!(ExtensionVersion::parse("01.2.3").is_err());
        assert!(ExtensionVersion::parse("-1.2.3").is_err());
        assert!(ExtensionVersion::parse("1.2.x").is_err());
    }

    #[test]
    fn version_requirements_are_explicit() {
        let required = ExtensionVersion::new(2, 1, 0);

        assert!(VersionRequirement::CompatibleMajor(required)
            .matches(ExtensionVersion::new(2, 1, 1)));
        assert!(!VersionRequirement::CompatibleMajor(required)
            .matches(ExtensionVersion::new(3, 0, 0)));
        assert!(VersionRequirement::AtLeast(required)
            .matches(ExtensionVersion::new(3, 0, 0)));
        assert!(!VersionRequirement::AtMost(required)
            .matches(ExtensionVersion::new(2, 1, 1)));
    }

    #[test]
    fn rejects_invalid_compatibility_interval() {
        assert!(SchemaCompatibility::new(
            ExtensionVersion::new(2, 0, 0),
            Some(ExtensionVersion::new(1, 0, 0)),
        )
        .is_err());
    }

    #[test]
    fn accepts_a_contribution_inside_the_extension_namespace() {
        let mut value = manifest();

        value
            .add_contribution(
                ExtensionContribution::new(
                    ContributionKind::Operation,
                    name("vendor_example::math::vector_add"),
                    ExtensionVersion::new(1, 0, 0),
                    true,
                )
                .expect("valid contribution"),
            )
            .expect("contribution should be accepted");

        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_a_contribution_outside_the_extension_namespace() {
        let mut value = manifest();

        let contribution = ExtensionContribution::new(
            ContributionKind::Operation,
            name("vendor_example::math_extra::vector_add"),
            ExtensionVersion::new(1, 0, 0),
            true,
        )
        .expect("valid qualified name");

        assert!(matches!(
            value.add_contribution(contribution),
            Err(ExtensionError::ContributionOutsideNamespace { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_dependencies() {
        let mut value = manifest();

        let dependency = ExtensionDependency::new(
            name("vendor_example::base"),
            VersionRequirement::Any,
            false,
        )
        .expect("valid dependency");

        value
            .add_dependency(dependency.clone())
            .expect("first dependency");

        assert!(matches!(
            value.add_dependency(dependency),
            Err(ExtensionError::DuplicateDependency(_))
        ));
    }

    #[test]
    fn rejects_self_dependency() {
        let mut value = manifest();

        let dependency = ExtensionDependency::new(
            name("vendor_example::math"),
            VersionRequirement::Any,
            false,
        )
        .expect("valid dependency declaration");

        assert!(matches!(
            value.add_dependency(dependency),
            Err(ExtensionError::SelfDependency(_))
        ));
    }

    #[test]
    fn rejects_duplicate_contributions() {
        let mut value = manifest();

        let contribution = ExtensionContribution::new(
            ContributionKind::Type,
            name("vendor_example::math::Vector"),
            ExtensionVersion::new(1, 0, 0),
            true,
        )
        .expect("valid contribution");

        value
            .add_contribution(contribution.clone())
            .expect("first contribution");

        assert!(matches!(
            value.add_contribution(contribution),
            Err(ExtensionError::DuplicateContribution { .. })
        ));
    }

    #[test]
    fn manifest_round_trips_through_json() {
        let mut value = manifest();

        value
            .set_description(Some("Portable mathematical operations".to_owned()))
            .expect("valid description");

        value
            .add_dependency(
                ExtensionDependency::new(
                    name("vendor_example::base"),
                    VersionRequirement::CompatibleMajor(
                        ExtensionVersion::new(1, 0, 0),
                    ),
                    false,
                )
                .expect("valid dependency"),
            )
            .expect("dependency accepted");

        value
            .add_contribution(
                ExtensionContribution::new(
                    ContributionKind::Operation,
                    name("vendor_example::math::vector_add"),
                    ExtensionVersion::new(1, 0, 0),
                    true,
                )
                .expect("valid contribution"),
            )
            .expect("contribution accepted");

        let encoded = serde_json::to_string(&value).expect("serialize manifest");
        let decoded: ExtensionManifest =
            serde_json::from_str(&encoded).expect("deserialize manifest");

        decoded.validate().expect("decoded manifest validates");
        assert_eq!(decoded, value);
    }

    #[test]
    fn validation_rejects_unknown_manifest_format() {
        let mut value = manifest();
        value.format_version = EXTENSION_MANIFEST_FORMAT_VERSION + 1;

        assert!(matches!(
            value.validate(),
            Err(ExtensionError::UnsupportedManifestFormat { .. })
        ));
    }

    #[test]
    fn contribution_namespace_matching_uses_segments() {
        assert!(is_within_namespace(
            &name("vendor_example::math"),
            &name("vendor_example::math::Vector"),
        ));

        assert!(!is_within_namespace(
            &name("vendor_example::math"),
            &name("vendor_example::math_extra::Vector"),
        ));
    }
}