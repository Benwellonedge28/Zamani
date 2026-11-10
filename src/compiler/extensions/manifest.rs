
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extension Manifest
//!
//! File: src/compiler/extensions/manifest.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the versioned, provider-neutral manifest describing an
//! independently developed Zamani compiler extension.
//!
//! A manifest describes what an extension declares that it provides.
//! It does not prove that the extension actually implements those
//! declarations.
//!
//! ## Canonical ownership
//!
//! Provider identity and lifecycle:
//!     crate::compiler::extensions::provider
//!
//! Semantic extension contracts:
//!     crate::frontend::semantic::domains::extensions
//!
//! ZUIR schema extension contracts:
//!     crate::zuir::schema::extensions
//!
//! Compiler extension discovery:
//!     crate::compiler::extensions::discovery
//!
//! Compiler extension compatibility:
//!     crate::compiler::extensions::compatibility
//!
//! Extension admission and validation:
//!     crate::compiler::extensions::validation
//!
//! Supply-chain verification:
//!     crate::compiler::security::supply_chain
//!
//! Provider registration:
//!     crate::compiler::extensions::registry
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Extension manifest data.
//! - Manifest schema version.
//! - Provider metadata declarations.
//! - Provider API compatibility declarations.
//! - Provider dependencies.
//! - Contribution declarations.
//! - Extension-point declarations.
//! - Manifest parsing and serialization.
//! - Manifest structural validation.
//! - Conversion into the canonical ProviderDescriptor.
//!
//! This module does not own:
//!
//! - Provider implementation.
//! - Provider registration.
//! - Provider discovery.
//! - Executable loading.
//! - Sandboxing.
//! - Publisher authentication.
//! - Signature verification.
//! - Capability authorization.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! ## Security
//!
//! Manifests are untrusted input.
//!
//! Successful parsing or validation does not establish publisher
//! identity, code integrity, authorization, or implementation safety.
//!
//! Security admission must be completed before executable providers
//! are activated.
//!
//! ## Scalability
//!
//! No universal extension-count, provider-count, contribution-count,
//! dependency-count, computing-domain-count, or hardware-count limit
//! is imposed.
//!
//! Collections grow according to available resources.
//!
//! Input budgets are supplied by callers and are not semantic limits.
//!
//! Literal infinite input or computation is not promised.
//!
//! ## Determinism
//!
//! Ordered collections are used for stable serialization and
//! deterministic provider descriptors.
//!
//! Input declaration order does not determine provider identity.
//!
//! ## Freeze criteria
//!
//! - Manifest identities are validated.
//! - Version declarations are validated.
//! - Duplicate dependencies are rejected.
//! - Duplicate contribution categories are rejected.
//! - Duplicate extension points are rejected.
//! - Unsupported schema versions are rejected.
//! - ProviderDescriptor is constructed through its canonical API.
//! - Untrusted input can be processed under configurable budgets.
//! - No unsafe Rust is used.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::provider::{
    ContributionKind,
    ExtensionPointId,
    ProviderApiCompatibility,
    ProviderDependency,
    ProviderDescriptor,
    ProviderError,
    ProviderId,
    ProviderVersion,
    ProviderVersionRequirement,
    validate_provider_compatibility,
};

/// Stable module identity.
pub const MANIFEST_MODULE_NAME: &str =
    "zamani.compiler.extensions.manifest";

/// Current manifest format major version.
pub const MANIFEST_FORMAT_MAJOR: u32 = 1;

/// Current manifest format minor version.
pub const MANIFEST_FORMAT_MINOR: u32 = 0;

/// Default manifest filename.
pub const MANIFEST_FILE_NAME: &str = "zamani-extension.toml";

// ============================================================================
// MANIFEST FORMAT VERSION
// ============================================================================

/// Version of the serialized extension manifest format.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Compiler API version.
/// - Provider implementation version.
/// - ZUIR schema version.
/// - Backend API version.
/// - Runtime version.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct ManifestFormatVersion {
    /// Major schema version.
    pub major: u32,

    /// Minor schema version.
    pub minor: u32,
}

impl ManifestFormatVersion {
    /// Current supported format.
    pub const CURRENT: Self = Self {
        major: MANIFEST_FORMAT_MAJOR,
        minor: MANIFEST_FORMAT_MINOR,
    };

    /// Construct a manifest format version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Check whether this implementation supports the format.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.major == MANIFEST_FORMAT_MAJOR
            && self.minor <= MANIFEST_FORMAT_MINOR
    }
}

// ============================================================================
// MANIFEST INPUT BUDGET
// ============================================================================

/// Optional operational limits for processing untrusted manifests.
///
/// These are operational safeguards, not language or provider
/// capacity restrictions.
///
/// An absent limit means that this module imposes no limit for
/// that particular dimension.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ManifestInputBudget {
    /// Maximum serialized input size in bytes.
    pub maximum_input_bytes: Option<usize>,

    /// Maximum number of declared collection entries.
    pub maximum_collection_items: Option<usize>,

    /// Maximum individual textual field size in bytes.
    pub maximum_text_bytes: Option<usize>,

    /// Maximum extension metadata nesting depth.
    pub maximum_metadata_depth: Option<usize>,

    /// Maximum extension metadata nodes.
    pub maximum_metadata_nodes: Option<usize>,
}

impl ManifestInputBudget {
    /// Creates an unlimited manifest input budget.
    ///
    /// Callers processing untrusted packages should provide
    /// explicit operational budgets.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            maximum_input_bytes: None,
            maximum_collection_items: None,
            maximum_text_bytes: None,
            maximum_metadata_depth: None,
            maximum_metadata_nodes: None,
        }
    }

    fn check_limit(
        actual: usize,
        maximum: Option<usize>,
        field: &'static str,
    ) -> Result<(), ManifestError> {
        if maximum.is_some_and(|limit| actual > limit) {
            return Err(ManifestError::BudgetExceeded {
                field,
                actual,
                maximum: maximum.unwrap_or_default(),
            });
        }

        Ok(())
    }

    fn validate_input_size(
        &self,
        input: &str,
    ) -> Result<(), ManifestError> {
        Self::check_limit(
            input.len(),
            self.maximum_input_bytes,
            "input_bytes",
        )
    }

    fn validate_text(
        &self,
        value: &str,
        field: &'static str,
    ) -> Result<(), ManifestError> {
        Self::check_limit(
            value.len(),
            self.maximum_text_bytes,
            field,
        )
    }

    fn validate_collection(
        &self,
        length: usize,
        field: &'static str,
    ) -> Result<(), ManifestError> {
        Self::check_limit(
            length,
            self.maximum_collection_items,
            field,
        )
    }
}

// ============================================================================
// PROVIDER API DECLARATION
// ============================================================================

/// Serialized declaration of supported compiler provider API versions.
///
/// The compiler API version is independent of the extension's
/// implementation version.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManifestApiCompatibility {
    /// Supported API major version.
    pub major: u64,

    /// Minimum supported minor version, inclusive.
    pub minimum_minor: u64,

    /// Optional maximum supported minor version, inclusive.
    ///
    /// An absent maximum declares compatibility with later minor
    /// versions subject to the host compatibility policy.
    pub maximum_minor: Option<u64>,
}

impl ManifestApiCompatibility {
    /// Convert into the canonical provider API compatibility type.
    pub fn to_provider_compatibility(
        &self,
    ) -> Result<ProviderApiCompatibility, ManifestError> {
        ProviderApiCompatibility::new(
            self.major,
            self.minimum_minor,
            self.maximum_minor,
        )
        .map_err(ManifestError::Provider)
    }

    /// Validate the declaration.
    pub fn validate(&self) -> Result<(), ManifestError> {
        self.to_provider_compatibility().map(|_| ())
    }
}

// ============================================================================
// PROVIDER DEPENDENCY MANIFEST
// ============================================================================

/// A serialized dependency on another compiler extension provider.
///
/// The minimum version is inclusive.
/// The optional maximum version is exclusive.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManifestDependency {
    /// Canonical provider identifier.
    pub provider_id: String,

    /// Inclusive minimum provider version.
    ///
    /// Format: major.minor.patch
    pub minimum_version: String,

    /// Optional exclusive maximum provider version.
    ///
    /// Format: major.minor.patch
    pub maximum_version_exclusive: Option<String>,
}

impl ManifestDependency {
    /// Convert the declaration into the canonical provider dependency.
    pub fn to_provider_dependency(
        &self,
    ) -> Result<ProviderDependency, ManifestError> {
        let id = ProviderId::new(self.provider_id.clone())
            .map_err(ManifestError::Provider)?;

        let minimum = parse_provider_version(
            &self.minimum_version,
        )?;

        let maximum = self
            .maximum_version_exclusive
            .as_deref()
            .map(parse_provider_version)
            .transpose()?;

        let requirement = ProviderVersionRequirement::new(
            minimum,
            maximum,
        )
        .map_err(ManifestError::Provider)?;

        Ok(ProviderDependency::new(id, requirement))
    }
}

// ============================================================================
// EXTENSION MANIFEST
// ============================================================================

/// Complete declarative manifest for one compiler extension provider.
///
/// All fields describe provider declarations, not verified
/// implementation capabilities.
///
/// Unknown top-level fields are retained in `extensions` to permit
/// namespaced metadata without forcing changes to this module.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// Manifest serialization format.
    pub format: ManifestFormatVersion,

    /// Globally namespaced provider identity.
    pub id: String,

    /// Provider implementation version.
    ///
    /// Format: major.minor.patch
    pub version: String,

    /// Supported compiler provider API.
    pub api: ManifestApiCompatibility,

    /// Human-readable provider name.
    pub name: String,

    /// Human-readable description.
    #[serde(default)]
    pub description: String,

    /// Declared contribution categories.
    ///
    /// Categories are open-world identifiers.
    #[serde(default)]
    pub contributions: Vec<String>,

    /// Symbolic compiler extension points.
    #[serde(default)]
    pub extension_points: Vec<String>,

    /// Provider dependencies.
    #[serde(default)]
    pub dependencies: Vec<ManifestDependency>,

    /// Provider authors or organizations.
    #[serde(default)]
    pub authors: Vec<String>,

    /// Optional license identifier.
    #[serde(default)]
    pub license: Option<String>,

    /// Namespaced provider-specific descriptive metadata.
    ///
    /// This metadata has no automatic execution or authorization
    /// semantics.
    #[serde(default)]
    pub metadata: BTreeMap<String, Value>,

    /// Additional top-level fields retained for forward-compatible
    /// descriptive metadata.
    ///
    /// Unknown fields must never be interpreted as authorization
    /// or as permission to execute code.
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

impl ExtensionManifest {
    /// Construct a new manifest from its required declarations.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
        api: ManifestApiCompatibility,
        name: impl Into<String>,
        contributions: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            format: ManifestFormatVersion::CURRENT,
            id: id.into(),
            version: version.into(),
            api,
            name: name.into(),
            description: String::new(),
            contributions: contributions.into_iter().collect(),
            extension_points: Vec::new(),
            dependencies: Vec::new(),
            authors: Vec::new(),
            license: None,
            metadata: BTreeMap::new(),
            extensions: BTreeMap::new(),
        }
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the provider implementation version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the manifest format.
    #[must_use]
    pub const fn format(&self) -> ManifestFormatVersion {
        self.format
    }

    /// Add a description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }

    /// Add extension points.
    #[must_use]
    pub fn with_extension_points(
        mut self,
        extension_points: impl IntoIterator<Item = String>,
    ) -> Self {
        self.extension_points.extend(extension_points);
        self
    }

    /// Add dependencies.
    #[must_use]
    pub fn with_dependencies(
        mut self,
        dependencies: impl IntoIterator<Item = ManifestDependency>,
    ) -> Self {
        self.dependencies.extend(dependencies);
        self
    }

    /// Add authors.
    #[must_use]
    pub fn with_authors(
        mut self,
        authors: impl IntoIterator<Item = String>,
    ) -> Self {
        self.authors.extend(authors);
        self
    }

    /// Add namespaced descriptive metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Parse a TOML manifest using caller-supplied operational budgets.
    pub fn from_toml_with_budget(
        input: &str,
        budget: &ManifestInputBudget,
    ) -> Result<Self, ManifestError> {
        budget.validate_input_size(input)?;

        let manifest: Self = toml::from_str(input)
            .map_err(|error| ManifestError::Parse {
                format: "TOML",
                message: error.to_string(),
            })?;

        manifest.validate_with_budget(budget)?;

        Ok(manifest)
    }

    /// Parse TOML without imposing additional local input budgets.
    ///
    /// Callers processing untrusted packages should prefer
    /// `from_toml_with_budget`.
    pub fn from_toml(
        input: &str,
    ) -> Result<Self, ManifestError> {
        Self::from_toml_with_budget(
            input,
            &ManifestInputBudget::unlimited(),
        )
    }

    /// Parse a JSON manifest using caller-supplied operational budgets.
    pub fn from_json_with_budget(
        input: &str,
        budget: &ManifestInputBudget,
    ) -> Result<Self, ManifestError> {
        budget.validate_input_size(input)?;

        let manifest: Self = serde_json::from_str(input)
            .map_err(|error| ManifestError::Parse {
                format: "JSON",
                message: error.to_string(),
            })?;

        manifest.validate_with_budget(budget)?;

        Ok(manifest)
    }

    /// Parse JSON without imposing additional local input budgets.
    pub fn from_json(
        input: &str,
    ) -> Result<Self, ManifestError> {
        Self::from_json_with_budget(
            input,
            &ManifestInputBudget::unlimited(),
        )
    }

    /// Serialize the manifest to TOML.
    pub fn to_toml(&self) -> Result<String, ManifestError> {
        self.validate()?;

        toml::to_string_pretty(self)
            .map_err(|error| ManifestError::Serialization {
                format: "TOML",
                message: error.to_string(),
            })
    }

    /// Serialize the manifest to JSON.
    pub fn to_json(&self) -> Result<String, ManifestError> {
        self.validate()?;

        serde_json::to_string_pretty(self)
            .map_err(|error| ManifestError::Serialization {
                format: "JSON",
                message: error.to_string(),
            })
    }

    /// Validate the manifest's structural and provider declarations.
    ///
    /// This method does not establish publisher trust, resolve
    /// dependencies against an installed registry, or authorize
    /// executable provider code.
    pub fn validate(&self) -> Result<(), ManifestError> {
        self.validate_with_budget(
            &ManifestInputBudget::unlimited(),
        )
    }

    /// Validate using explicit operational budgets.
    pub fn validate_with_budget(
        &self,
        budget: &ManifestInputBudget,
    ) -> Result<(), ManifestError> {
        if !self.format.is_supported() {
            return Err(ManifestError::UnsupportedFormat {
                found: self.format,
                supported: ManifestFormatVersion::CURRENT,
            });
        }

        budget.validate_text(&self.id, "provider_id")?;
        budget.validate_text(&self.version, "provider_version")?;
        budget.validate_text(&self.name, "provider_name")?;
        budget.validate_text(&self.description, "description")?;

        if self.name.trim().is_empty() {
            return Err(ManifestError::InvalidField {
                field: "name",
                reason: "provider name must not be empty",
            });
        }

        if self.name.chars().any(char::is_control)
            || self.description.chars().any(char::is_control)
        {
            return Err(ManifestError::InvalidField {
                field: "metadata",
                reason: "control characters are not permitted",
            });
        }

        let _id = ProviderId::new(self.id.clone())
            .map_err(ManifestError::Provider)?;

        parse_provider_version(&self.version)?;

        self.api.validate()?;

        budget.validate_collection(
            self.contributions.len(),
            "contributions",
        )?;

        budget.validate_collection(
            self.extension_points.len(),
            "extension_points",
        )?;

        budget.validate_collection(
            self.dependencies.len(),
            "dependencies",
        )?;

        budget.validate_collection(
            self.authors.len(),
            "authors",
        )?;

        budget.validate_collection(
            self.metadata.len(),
            "metadata",
        )?;

        validate_unique_strings(
            &self.contributions,
            "contributions",
            budget,
        )?;

        validate_unique_strings(
            &self.extension_points,
            "extension_points",
            budget,
        )?;

        validate_unique_strings(
            &self.authors,
            "authors",
            budget,
        )?;

        if self.contributions.is_empty() {
            return Err(ManifestError::InvalidField {
                field: "contributions",
                reason: "at least one contribution category is required",
            });
        }

        for contribution in &self.contributions {
            budget.validate_text(
                contribution,
                "contribution",
            )?;

            ContributionKind::new(contribution.clone())
                .map_err(ManifestError::Provider)?;
        }

        for extension_point in &self.extension_points {
            budget.validate_text(
                extension_point,
                "extension_point",
            )?;

            ExtensionPointId::new(extension_point.clone())
                .map_err(ManifestError::Provider)?;
        }

        for author in &self.authors {
            budget.validate_text(author, "author")?;

            if author.trim().is_empty()
                || author.chars().any(char::is_control)
            {
                return Err(ManifestError::InvalidField {
                    field: "authors",
                    reason: "author entries must be non-empty and contain no control characters",
                });
            }
        }

        if let Some(license) = &self.license {
            budget.validate_text(license, "license")?;

            if license.trim().is_empty()
                || license.chars().any(char::is_control)
            {
                return Err(ManifestError::InvalidField {
                    field: "license",
                    reason: "license identifier must be non-empty",
                });
            }
        }

        validate_dependencies(&self.id, &self.dependencies, budget)?;

        validate_metadata(
            &self.metadata,
            budget,
        )?;

        validate_metadata(
            &self.extensions,
            budget,
        )?;

        Ok(())
    }

    /// Construct the canonical provider descriptor.
    ///
    /// This operation validates the manifest before conversion.
    ///
    /// It does not register or initialize the provider.
    pub fn to_provider_descriptor(
        &self,
    ) -> Result<ProviderDescriptor, ManifestError> {
        self.validate()?;

        let id = ProviderId::new(self.id.clone())
            .map_err(ManifestError::Provider)?;

        let version = parse_provider_version(&self.version)?;

        let api = self.api.to_provider_compatibility()?;

        let contributions = self
            .contributions
            .iter()
            .map(|value| {
                ContributionKind::new(value.clone())
                    .map_err(ManifestError::Provider)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut descriptor = ProviderDescriptor::new(
            id,
            version,
            api,
            self.name.clone(),
            self.description.clone(),
            contributions,
        )
        .map_err(ManifestError::Provider)?;

        let points = self
            .extension_points
            .iter()
            .map(|value| {
                ExtensionPointId::new(value.clone())
                    .map_err(ManifestError::Provider)
            })
            .collect::<Result<Vec<_>, _>>()?;

        descriptor = descriptor.with_extension_points(points);

        for dependency in &self.dependencies {
            descriptor = descriptor
                .requiring(
                    dependency.to_provider_dependency()?,
                )
                .map_err(ManifestError::Provider)?;
        }

        Ok(descriptor)
    }

    /// Validate the manifest against an actual compiler provider API.
    ///
    /// Returns a canonical descriptor only if the declared API
    /// compatibility includes the supplied host version.
    pub fn validate_for_host(
        &self,
        host_api_major: u64,
        host_api_minor: u64,
    ) -> Result<ProviderDescriptor, ManifestError> {
        let descriptor = self.to_provider_descriptor()?;

        validate_provider_compatibility(
            &descriptor,
            host_api_major,
            host_api_minor,
        )
        .map_err(ManifestError::Provider)?;

        Ok(descriptor)
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn parse_provider_version(
    value: &str,
) -> Result<ProviderVersion, ManifestError> {
    let mut components = value.split('.');

    let major = parse_version_component(
        components.next(),
    )?;

    let minor = parse_version_component(
        components.next(),
    )?;

    let patch = parse_version_component(
        components.next(),
    )?;

    if components.next().is_some() {
        return Err(ManifestError::InvalidField {
            field: "version",
            reason: "version must contain exactly three numeric components",
        });
    }

    Ok(ProviderVersion::new(major, minor, patch))
}

fn parse_version_component(
    value: Option<&str>,
) -> Result<u64, ManifestError> {
    let value = value.ok_or(ManifestError::InvalidField {
        field: "version",
        reason: "missing version component",
    })?;

    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(ManifestError::InvalidField {
            field: "version",
            reason: "version components must contain decimal digits",
        });
    }

    value.parse::<u64>().map_err(|_| {
        ManifestError::InvalidField {
            field: "version",
            reason: "version component exceeds its numeric representation",
        }
    })
}

fn validate_unique_strings(
    values: &[String],
    field: &'static str,
    budget: &ManifestInputBudget,
) -> Result<(), ManifestError> {
    let mut unique = BTreeSet::new();

    for value in values {
        budget.validate_text(value, field)?;

        if value.trim().is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(ManifestError::InvalidField {
                field,
                reason: "entries must be non-empty and contain no control characters",
            });
        }

        if !unique.insert(value.as_str()) {
            return Err(ManifestError::DuplicateEntry {
                field,
                value: value.clone(),
            });
        }
    }

    Ok(())
}

fn validate_dependencies(
    owner_id: &str,
    dependencies: &[ManifestDependency],
    budget: &ManifestInputBudget,
) -> Result<(), ManifestError> {
    let mut identities = BTreeSet::new();

    let owner = ProviderId::new(owner_id.to_owned())
        .map_err(ManifestError::Provider)?;

    for dependency in dependencies {
        budget.validate_text(
            &dependency.provider_id,
            "dependency.provider_id",
        )?;

        budget.validate_text(
            &dependency.minimum_version,
            "dependency.minimum_version",
        )?;

        if let Some(maximum) = &dependency.maximum_version_exclusive {
            budget.validate_text(
                maximum,
                "dependency.maximum_version_exclusive",
            )?;
        }

        let converted = dependency.to_provider_dependency()?;

        if converted.provider_id() == &owner {
            return Err(ManifestError::SelfDependency {
                provider_id: owner_id.to_owned(),
            });
        }

        if !identities.insert(
            converted.provider_id().clone(),
        ) {
            return Err(ManifestError::DuplicateEntry {
                field: "dependencies",
                value: dependency.provider_id.clone(),
            });
        }
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, Value>,
    budget: &ManifestInputBudget,
) -> Result<(), ManifestError> {
    let mut pending: Vec<(&Value, usize)> = metadata
        .values()
        .map(|value| (value, 0))
        .collect();

    let mut visited = 0usize;

    while let Some((value, depth)) = pending.pop() {
        visited = visited.checked_add(1).ok_or(
            ManifestError::CounterOverflow,
        )?;

        ManifestInputBudget::check_limit(
            visited,
            budget.maximum_metadata_nodes,
            "metadata_nodes",
        )?;

        ManifestInputBudget::check_limit(
            depth,
            budget.maximum_metadata_depth,
            "metadata_depth",
        )?;

        match value {
            Value::Array(values) => {
                for child in values {
                    pending.push((child, depth.saturating_add(1)));
                }
            }

            Value::Object(values) => {
                for child in values.values() {
                    pending.push((child, depth.saturating_add(1)));
                }
            }

            _ => {}
        }
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured extension manifest errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ManifestError {
    /// Unsupported manifest schema version.
    UnsupportedFormat {
        /// Encountered version.
        found: ManifestFormatVersion,

        /// Highest supported version.
        supported: ManifestFormatVersion,
    },

    /// Invalid manifest field.
    InvalidField {
        /// Field name.
        field: &'static str,

        /// Reason for rejection.
        reason: &'static str,
    },

    /// Duplicate declaration.
    DuplicateEntry {
        /// Collection containing the duplicate.
        field: &'static str,

        /// Duplicate identity.
        value: String,
    },

    /// Provider declares a dependency on itself.
    SelfDependency {
        /// Provider identity.
        provider_id: String,
    },

    /// Operational budget exceeded.
    BudgetExceeded {
        /// Budget dimension.
        field: &'static str,

        /// Actual observed quantity.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Counter overflow.
    CounterOverflow,

    /// Manifest parsing failure.
    Parse {
        /// Input format.
        format: &'static str,

        /// Parser error.
        message: String,
    },

    /// Manifest serialization failure.
    Serialization {
        /// Output format.
        format: &'static str,

        /// Serializer error.
        message: String,
    },

    /// Canonical provider contract rejected the declaration.
    Provider(ProviderError),
}

impl fmt::Display for ManifestError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnsupportedFormat {
                found,
                supported,
            } => write!(
                formatter,
                "unsupported manifest format {}.{}; supported format is {}.{}",
                found.major,
                found.minor,
                supported.major,
                supported.minor
            ),

            Self::InvalidField { field, reason } => {
                write!(
                    formatter,
                    "invalid manifest field `{field}`: {reason}"
                )
            }

            Self::DuplicateEntry { field, value } => {
                write!(
                    formatter,
                    "duplicate manifest entry in `{field}`: {value}"
                )
            }

            Self::SelfDependency { provider_id } => {
                write!(
                    formatter,
                    "provider `{provider_id}` cannot depend on itself"
                )
            }

            Self::BudgetExceeded {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "manifest operational budget `{field}` exceeded: {actual} > {maximum}"
            ),

            Self::CounterOverflow => {
                formatter.write_str("manifest validation counter overflow")
            }

            Self::Parse { format, message } => {
                write!(
                    formatter,
                    "failed to parse {format} extension manifest: {message}"
                )
            }

            Self::Serialization { format, message } => {
                write!(
                    formatter,
                    "failed to serialize {format} extension manifest: {message}"
                )
            }

            Self::Provider(error) => {
                write!(formatter, "provider contract error: {error}")
            }
        }
    }
}

impl std::error::Error for ManifestError {}

// ============================================================================
// MODULE HELPERS
// ============================================================================

/// Returns the canonical manifest module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    MANIFEST_MODULE_NAME
}

/// Returns the current manifest format version.
#[must_use]
pub const fn format_version() -> ManifestFormatVersion {
    ManifestFormatVersion::CURRENT
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn api() -> ManifestApiCompatibility {
        ManifestApiCompatibility {
            major: super::super::provider::PROVIDER_API_MAJOR,
            minimum_minor: 0,
            maximum_minor: None,
        }
    }

    fn manifest() -> ExtensionManifest {
        ExtensionManifest::new(
            "example.compiler",
            "1.0.0",
            api(),
            "Example Compiler Provider",
            vec![
                "compiler.pass".to_owned(),
            ],
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.extensions.manifest"
        );
    }

    #[test]
    fn current_manifest_version_is_supported() {
        assert!(
            ManifestFormatVersion::CURRENT.is_supported()
        );
    }

    #[test]
    fn rejects_unsupported_major_version() {
        assert!(
            !ManifestFormatVersion::new(2, 0).is_supported()
        );
    }

    #[test]
    fn validates_basic_manifest() {
        assert!(manifest().validate().is_ok());
    }

    #[test]
    fn rejects_empty_provider_identity() {
        let mut value = manifest();
        value.id.clear();

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_invalid_provider_version() {
        let mut value = manifest();
        value.version = "1.0".to_owned();

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_contributions() {
        let mut value = manifest();

        value.contributions = vec![
            "compiler.pass".to_owned(),
            "compiler.pass".to_owned(),
        ];

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_extension_points() {
        let value = manifest()
            .with_extension_points([
                "quantum::operation".to_owned(),
                "quantum::operation".to_owned(),
            ]);

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_self_dependency() {
        let value = manifest().with_dependencies([
            ManifestDependency {
                provider_id: "example.compiler".to_owned(),
                minimum_version: "1.0.0".to_owned(),
                maximum_version_exclusive: None,
            },
        ]);

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_dependencies() {
        let dependency = ManifestDependency {
            provider_id: "example.foundation".to_owned(),
            minimum_version: "1.0.0".to_owned(),
            maximum_version_exclusive: None,
        };

        let value = manifest().with_dependencies([
            dependency.clone(),
            dependency,
        ]);

        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_reversed_dependency_ranges() {
        let value = manifest().with_dependencies([
            ManifestDependency {
                provider_id: "example.foundation".to_owned(),
                minimum_version: "2.0.0".to_owned(),
                maximum_version_exclusive: Some(
                    "1.0.0".to_owned(),
                ),
            },
        ]);

        assert!(value.validate().is_err());
    }

    #[test]
    fn converts_to_canonical_provider_descriptor() {
        let value = manifest()
            .with_extension_points([
                "classical::optimization".to_owned(),
                "quantum::operation".to_owned(),
                "hdl::synthesis".to_owned(),
                "distributed::transport".to_owned(),
                "future::computing::extension".to_owned(),
            ]);

        let descriptor = value
            .to_provider_descriptor()
            .expect("valid provider descriptor");

        assert_eq!(
            descriptor.id().as_str(),
            "example.compiler"
        );

        assert_eq!(
            descriptor.version(),
            ProviderVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn validates_actual_host_api_compatibility() {
        let value = manifest();

        assert!(
            value
                .validate_for_host(
                    super::super::provider::PROVIDER_API_MAJOR,
                    super::super::provider::PROVIDER_API_MINOR,
                )
                .is_ok()
        );
    }

    #[test]
    fn rejects_incompatible_host_api() {
        let value = manifest();

        assert!(
            value
                .validate_for_host(
                    super::super::provider::PROVIDER_API_MAJOR + 1,
                    0,
                )
                .is_err()
        );
    }

    #[test]
    fn serializes_and_parses_json() {
        let original = manifest()
            .with_description("Example provider");

        let serialized = original
            .to_json()
            .expect("JSON serialization");

        let restored = ExtensionManifest::from_json(
            &serialized,
        )
        .expect("JSON parsing");

        assert_eq!(original, restored);
    }

    #[test]
    fn serializes_and_parses_toml() {
        let original = manifest()
            .with_description("Example provider");

        let serialized = original
            .to_toml()
            .expect("TOML serialization");

        let restored = ExtensionManifest::from_toml(
            &serialized,
        )
        .expect("TOML parsing");

        assert_eq!(original, restored);
    }

    #[test]
    fn enforces_configured_input_budget() {
        let input = manifest()
            .to_json()
            .expect("JSON serialization");

        let budget = ManifestInputBudget {
            maximum_input_bytes: Some(1),
            ..ManifestInputBudget::default()
        };

        assert!(
            ExtensionManifest::from_json_with_budget(
                &input,
                &budget,
            )
            .is_err()
        );
    }

    #[test]
    fn retains_unknown_extension_metadata() {
        let mut value = manifest();

        value.extensions.insert(
            "future_provider_feature".to_owned(),
            Value::String("declarative".to_owned()),
        );

        let serialized = value.to_json().unwrap();

        let restored =
            ExtensionManifest::from_json(&serialized).unwrap();

        assert_eq!(value, restored);
    }

    #[test]
    fn manifest_metadata_does_not_imply_trust() {
        let value = manifest();

        assert!(value.validate().is_ok());

        // Structural validity does not provide a provider signature,
        // authenticated publisher identity, or execution authorization.
    }

    #[test]
    fn provider_contribution_categories_are_open_world() {
        let value = ExtensionManifest::new(
            "future.provider",
            "1.0.0",
            api(),
            "Future Provider",
            vec![
                "future.photonic.compute".to_owned(),
                "future.molecular.compute".to_owned(),
                "future.neuromorphic.compute".to_owned(),
            ],
        );

        assert!(value.validate().is_ok());
    }
}
