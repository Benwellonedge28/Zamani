
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Component Manifests
//!
//! Path: src/stdlib/manifest.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines validated, target-independent metadata contracts for
//! standard-library components.
//!
//! A manifest describes a component and the conditions under which
//! it may participate in a standard-library composition.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Component identity.
//! - Component classification.
//! - Manifest schema version.
//! - Component implementation version.
//! - Component API version.
//! - Component metadata.
//! - Dependency declarations.
//! - Dependency version requirements.
//! - Required and provided capability declarations.
//! - Declarative lifecycle policies.
//! - Manifest validation.
//! - Cross-manifest dependency validation.
//! - Dependency-cycle detection.
//!
//! This module does NOT own:
//!
//! - Provider registration.
//! - Provider implementation.
//! - Capability discovery.
//! - Semantic analysis.
//! - Resource allocation.
//! - Hardware discovery.
//! - Provider selection.
//! - Component initialization.
//! - Component shutdown execution.
//! - Runtime scheduling.
//! - Hardware routing.
//! - Compiler or IR representations.
//!
//! ## Authoritative integrations
//!
//! - crate::stdlib::version
//! - crate::stdlib::capability
//! - crate::stdlib::contract
//! - crate::stdlib::provider
//! - crate::stdlib::registry
//! - crate::stdlib::lifecycle (future lifecycle consumer)
//!
//! Canonical semantic authorities:
//!
//! - crate::frontend::semantic
//! - crate::compiler
//! - crate::zuir
//! - crate::runtime
//!
//! ## Fundamental distinctions
//!
//! A component dependency is not a provider registration.
//!
//! A capability declaration is not proof of capability availability.
//!
//! A lifecycle declaration is not lifecycle execution.
//!
//! A version match is not proof of semantic compatibility.
//!
//! A valid manifest is not proof that a component can execute.
//!
//! ## POCO-REAF
//!
//! Component identities, component kinds and dependencies are
//! extensible.
//!
//! This module does not enumerate:
//!
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - QPUs;
//! - processors;
//! - devices;
//! - nodes;
//! - qubits;
//! - memory capacities;
//! - tensor dimensions;
//! - computational domains.
//!
//! No arbitrary universal component-count or dependency-count
//! ceiling is introduced.
//!
//! Actual storage and processing requirements depend on available
//! resources and explicitly configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Determinism
//!
//! Component metadata uses BTreeMap.
//!
//! Component manifests preserve their declared dependency order.
//!
//! Validation results use deterministic component ordering.
//!
//! ## Serialization
//!
//! This module defines the typed in-memory manifest contract.
//!
//! Serialization formats, schema migrations, canonical encoding,
//! and package-file parsing belong to the serialization and
//! package-management authorities.
//!
//! Untrusted serialized manifests must be validated before use.

use core::cmp::Ordering;
use core::fmt;

use std::collections::{
    BTreeMap,
    BTreeSet,
    VecDeque,
};
use std::error::Error as StdError;

use super::capability::CapabilityId;
use super::version::Version;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable, extensible identity of a standard-library component.
///
/// Identifiers use lowercase ASCII dot-separated segments.
///
/// Examples:
///
/// - stdlib.foundation
/// - stdlib.foundation.collections
/// - stdlib.quantum
/// - stdlib.hdl
/// - stdlib.hybrid
/// - stdlib.distributed
/// - stdlib.ai
/// - stdlib.future_domain
///
/// These examples do not constitute a closed list.
///
/// No arbitrary maximum identifier length or namespace depth exists.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId(String);

impl ComponentId {
    /// Constructs a validated component identity.
    ///
    /// # Errors
    ///
    /// Returns ManifestError if the identity is malformed.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ManifestError> {
        let value = value.into();

        validate_identifier(&value, "component identifier")?;

        Ok(Self(value))
    }

    /// Returns the exact component identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<&str> for ComponentId {
    type Error = ManifestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for ComponentId {
    type Error = ManifestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// COMPONENT KIND
// ============================================================================

/// Open-world classification of a standard-library component.
///
/// Examples include:
///
/// - foundation
/// - collection
/// - scientific
/// - quantum
/// - hdl
/// - hybrid
/// - distributed
/// - ai
/// - networking
/// - security
/// - storage
/// - integration
/// - extension
///
/// New kinds can be introduced without modifying this module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentKind(String);

impl ComponentKind {
    /// Constructs a validated component kind.
    ///
    /// # Errors
    ///
    /// Returns ManifestError for an invalid kind.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ManifestError> {
        let value = value.into();

        validate_identifier(&value, "component kind")?;

        Ok(Self(value))
    }

    /// Returns the component kind.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// VERSION REQUIREMENTS
// ============================================================================

/// One boundary in a semantic version requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionBound {
    version: Version,
    inclusive: bool,
}

impl VersionBound {
    /// Constructs a version boundary.
    #[must_use]
    pub fn new(
        version: Version,
        inclusive: bool,
    ) -> Self {
        Self {
            version,
            inclusive,
        }
    }

    /// Returns the boundary version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns whether the boundary is inclusive.
    #[must_use]
    pub const fn is_inclusive(&self) -> bool {
        self.inclusive
    }
}

/// A semantic version compatibility requirement.
///
/// Build metadata is ignored during precedence comparisons,
/// following Semantic Versioning 2.0.0.
///
/// Examples:
///
/// - Any version.
/// - Exactly 1.2.3.
/// - At least 1.2.0.
/// - Greater than 1.2.0.
/// - Less than 3.0.0.
/// - Between 1.0.0 inclusive and 2.0.0 exclusive.
///
/// The requirement applies to semantic version precedence,
/// not complete version-string identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionRequirement {
    lower: Option<VersionBound>,
    upper: Option<VersionBound>,
}

impl VersionRequirement {
    /// Accepts every valid semantic version.
    #[must_use]
    pub const fn any() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }

    /// Requires the specified semantic version.
    #[must_use]
    pub fn exact(version: Version) -> Self {
        Self {
            lower: Some(VersionBound::new(
                version.clone(),
                true,
            )),
            upper: Some(VersionBound::new(
                version,
                true,
            )),
        }
    }

    /// Requires a version greater than or equal to the boundary.
    #[must_use]
    pub fn at_least(version: Version) -> Self {
        Self {
            lower: Some(VersionBound::new(version, true)),
            upper: None,
        }
    }

    /// Requires a version strictly greater than the boundary.
    #[must_use]
    pub fn greater_than(version: Version) -> Self {
        Self {
            lower: Some(VersionBound::new(version, false)),
            upper: None,
        }
    }

    /// Requires a version less than or equal to the boundary.
    #[must_use]
    pub fn at_most(version: Version) -> Self {
        Self {
            lower: None,
            upper: Some(VersionBound::new(version, true)),
        }
    }

    /// Requires a version strictly less than the boundary.
    #[must_use]
    pub fn less_than(version: Version) -> Self {
        Self {
            lower: None,
            upper: Some(VersionBound::new(version, false)),
        }
    }

    /// Constructs a bounded version interval.
    ///
    /// # Errors
    ///
    /// Returns ManifestError if the interval is contradictory.
    pub fn between(
        lower: VersionBound,
        upper: VersionBound,
    ) -> Result<Self, ManifestError> {
        let requirement = Self {
            lower: Some(lower),
            upper: Some(upper),
        };

        requirement.validate()?;

        Ok(requirement)
    }

    /// Returns the lower version boundary.
    #[must_use]
    pub fn lower_bound(&self) -> Option<&VersionBound> {
        self.lower.as_ref()
    }

    /// Returns the upper version boundary.
    #[must_use]
    pub fn upper_bound(&self) -> Option<&VersionBound> {
        self.upper.as_ref()
    }

    /// Validates the version interval.
    ///
    /// # Errors
    ///
    /// Returns ManifestError for an impossible interval.
    pub fn validate(&self) -> Result<(), ManifestError> {
        let (Some(lower), Some(upper)) =
            (&self.lower, &self.upper)
        else {
            return Ok(());
        };

        match lower.version.precedence_cmp(&upper.version) {
            Ordering::Greater => {
                Err(ManifestError::InvalidVersionRange)
            }

            Ordering::Equal
                if !lower.inclusive || !upper.inclusive =>
            {
                Err(ManifestError::InvalidVersionRange)
            }

            _ => Ok(()),
        }
    }

    /// Checks whether a version satisfies this requirement.
    #[must_use]
    pub fn matches(&self, candidate: &Version) -> bool {
        if let Some(lower) = &self.lower {
            match candidate.precedence_cmp(&lower.version) {
                Ordering::Less => return false,

                Ordering::Equal if !lower.inclusive => {
                    return false;
                }

                _ => {}
            }
        }

        if let Some(upper) = &self.upper {
            match candidate.precedence_cmp(&upper.version) {
                Ordering::Greater => return false,

                Ordering::Equal if !upper.inclusive => {
                    return false;
                }

                _ => {}
            }
        }

        true
    }
}

impl Default for VersionRequirement {
    fn default() -> Self {
        Self::any()
    }
}

// ============================================================================
// COMPONENT DEPENDENCIES
// ============================================================================

/// A dependency declaration between two standard-library components.
///
/// A dependency describes a component relationship.
///
/// It does not identify a provider, device or execution target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentDependency {
    target: ComponentId,
    component_version: VersionRequirement,
    api_version: VersionRequirement,
    optional: bool,
    rationale: Option<String>,
}

impl ComponentDependency {
    /// Creates a required dependency with unrestricted version ranges.
    #[must_use]
    pub fn new(target: ComponentId) -> Self {
        Self {
            target,
            component_version: VersionRequirement::any(),
            api_version: VersionRequirement::any(),
            optional: false,
            rationale: None,
        }
    }

    /// Sets the required component implementation version.
    #[must_use]
    pub fn component_version(
        mut self,
        requirement: VersionRequirement,
    ) -> Self {
        self.component_version = requirement;
        self
    }

    /// Sets the required public API version.
    #[must_use]
    pub fn api_version(
        mut self,
        requirement: VersionRequirement,
    ) -> Self {
        self.api_version = requirement;
        self
    }

    /// Marks this dependency as optional.
    #[must_use]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Adds explanatory dependency rationale.
    ///
    /// # Errors
    ///
    /// Returns ManifestError for invalid rationale.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, ManifestError> {
        let rationale = rationale.into();

        validate_description(&rationale, "dependency rationale")?;

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Returns the dependency target.
    #[must_use]
    pub fn target(&self) -> &ComponentId {
        &self.target
    }

    /// Returns the component version requirement.
    #[must_use]
    pub fn component_version_requirement(
        &self,
    ) -> &VersionRequirement {
        &self.component_version
    }

    /// Returns the API version requirement.
    #[must_use]
    pub fn api_version_requirement(
        &self,
    ) -> &VersionRequirement {
        &self.api_version
    }

    /// Returns whether this dependency is optional.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }

    /// Returns the optional rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

// ============================================================================
// LIFECYCLE DECLARATIONS
// ============================================================================

/// Declarative component initialization policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum InitializationPolicy {
    /// Initialize when the lifecycle manager requests it.
    OnDemand,

    /// Initialize during the configured startup phase.
    Eager,

    /// Initialization is managed by an external authority.
    External,
}

/// Declarative component shutdown policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ShutdownPolicy {
    /// Request orderly shutdown and cleanup.
    Graceful,

    /// Shutdown is performed by an external authority.
    External,

    /// The component declares no independent shutdown operation.
    NotApplicable,
}

/// Policy for handling component initialization failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LifecycleFailurePolicy {
    /// Prevent dependent components from initializing.
    AbortDependents,

    /// Disable the failed component and preserve independent components.
    DisableComponent,

    /// Allow independent components to continue.
    ContinueIndependent,
}

/// Declarative lifecycle information for a component.
///
/// This is metadata consumed by the lifecycle manager.
///
/// It does not execute initialization, shutdown or rollback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LifecycleDeclaration {
    initialization: InitializationPolicy,
    shutdown: ShutdownPolicy,
    failure: LifecycleFailurePolicy,
}

impl LifecycleDeclaration {
    /// Constructs a lifecycle declaration.
    #[must_use]
    pub const fn new(
        initialization: InitializationPolicy,
        shutdown: ShutdownPolicy,
        failure: LifecycleFailurePolicy,
    ) -> Self {
        Self {
            initialization,
            shutdown,
            failure,
        }
    }

    /// Returns the initialization policy.
    #[must_use]
    pub const fn initialization(&self) -> InitializationPolicy {
        self.initialization
    }

    /// Returns the shutdown policy.
    #[must_use]
    pub const fn shutdown(&self) -> ShutdownPolicy {
        self.shutdown
    }

    /// Returns the initialization failure policy.
    #[must_use]
    pub const fn failure(&self) -> LifecycleFailurePolicy {
        self.failure
    }
}

impl Default for LifecycleDeclaration {
    fn default() -> Self {
        Self {
            initialization: InitializationPolicy::OnDemand,
            shutdown: ShutdownPolicy::Graceful,
            failure: LifecycleFailurePolicy::AbortDependents,
        }
    }
}

// ============================================================================
// MANIFEST
// ============================================================================

/// Complete, validated metadata for a standard-library component.
///
/// A manifest is immutable after construction.
///
/// A new component version or contract must be published through
/// a newly constructed manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Manifest {
    id: ComponentId,
    kind: ComponentKind,

    format_version: Version,
    component_version: Version,
    api_version: Version,

    description: String,

    dependencies: Vec<ComponentDependency>,

    required_capabilities: Vec<CapabilityId>,
    provided_capabilities: Vec<CapabilityId>,

    lifecycle: LifecycleDeclaration,

    metadata: BTreeMap<String, String>,
}

impl Manifest {
    /// Creates a manifest builder.
    #[must_use]
    pub fn builder(
        id: ComponentId,
        kind: ComponentKind,
        format_version: Version,
        component_version: Version,
        api_version: Version,
    ) -> ManifestBuilder {
        ManifestBuilder::new(
            id,
            kind,
            format_version,
            component_version,
            api_version,
        )
    }

    /// Returns the component identity.
    #[must_use]
    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    /// Returns the component classification.
    #[must_use]
    pub fn kind(&self) -> &ComponentKind {
        &self.kind
    }

    /// Returns the manifest schema version.
    #[must_use]
    pub fn format_version(&self) -> &Version {
        &self.format_version
    }

    /// Returns the component implementation version.
    #[must_use]
    pub fn component_version(&self) -> &Version {
        &self.component_version
    }

    /// Returns the component public API version.
    #[must_use]
    pub fn api_version(&self) -> &Version {
        &self.api_version
    }

    /// Returns the component description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns declared component dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[ComponentDependency] {
        &self.dependencies
    }

    /// Returns mandatory capability declarations.
    ///
    /// These describe what the component requires.
    ///
    /// They do not prove availability.
    #[must_use]
    pub fn required_capabilities(&self) -> &[CapabilityId] {
        &self.required_capabilities
    }

    /// Returns capabilities declared by the component.
    ///
    /// These declarations are descriptive and do not establish
    /// current availability.
    #[must_use]
    pub fn provided_capabilities(&self) -> &[CapabilityId] {
        &self.provided_capabilities
    }

    /// Returns lifecycle policy metadata.
    #[must_use]
    pub const fn lifecycle(&self) -> LifecycleDeclaration {
        self.lifecycle
    }

    /// Returns deterministic extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns a metadata value by key.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }
}

// ============================================================================
// MANIFEST BUILDER
// ============================================================================

/// Builder for validated standard-library manifests.
#[derive(Clone, Debug)]
pub struct ManifestBuilder {
    id: ComponentId,
    kind: ComponentKind,

    format_version: Version,
    component_version: Version,
    api_version: Version,

    description: Option<String>,

    dependencies: Vec<ComponentDependency>,

    required_capabilities: Vec<CapabilityId>,
    provided_capabilities: Vec<CapabilityId>,

    lifecycle: LifecycleDeclaration,

    metadata: BTreeMap<String, String>,
}

impl ManifestBuilder {
    /// Creates a manifest builder.
    #[must_use]
    pub fn new(
        id: ComponentId,
        kind: ComponentKind,
        format_version: Version,
        component_version: Version,
        api_version: Version,
    ) -> Self {
        Self {
            id,
            kind,
            format_version,
            component_version,
            api_version,
            description: None,
            dependencies: Vec::new(),
            required_capabilities: Vec::new(),
            provided_capabilities: Vec::new(),
            lifecycle: LifecycleDeclaration::default(),
            metadata: BTreeMap::new(),
        }
    }

    /// Sets the component description.
    pub fn description(
        &mut self,
        value: impl Into<String>,
    ) -> Result<&mut Self, ManifestError> {
        let value = value.into();

        validate_description(&value, "component description")?;

        self.description = Some(value);

        Ok(self)
    }

    /// Adds a component dependency.
    ///
    /// Duplicate dependencies are rejected during build.
    pub fn dependency(
        &mut self,
        dependency: ComponentDependency,
    ) -> &mut Self {
        self.dependencies.push(dependency);
        self
    }

    /// Declares a mandatory capability.
    ///
    /// Duplicate identities are rejected during build.
    pub fn requires_capability(
        &mut self,
        capability: CapabilityId,
    ) -> &mut Self {
        self.required_capabilities.push(capability);
        self
    }

    /// Declares a capability provided by this component.
    ///
    /// This is descriptive metadata, not live availability evidence.
    pub fn provides_capability(
        &mut self,
        capability: CapabilityId,
    ) -> &mut Self {
        self.provided_capabilities.push(capability);
        self
    }

    /// Sets the lifecycle declaration.
    pub fn lifecycle(
        &mut self,
        lifecycle: LifecycleDeclaration,
    ) -> &mut Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Adds extension metadata.
    ///
    /// Existing keys are rejected instead of being silently overwritten.
    ///
    /// # Errors
    ///
    /// Returns ManifestError for invalid or duplicate metadata.
    pub fn metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<&mut Self, ManifestError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        if self.metadata.contains_key(&key) {
            return Err(ManifestError::DuplicateMetadata(key));
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Validates and constructs the immutable manifest.
    ///
    /// # Errors
    ///
    /// Returns ManifestError if any declaration is invalid.
    pub fn build(self) -> Result<Manifest, ManifestError> {
        let description = self.description.ok_or(
            ManifestError::MissingDescription,
        )?;

        let mut dependencies = BTreeSet::new();

        for dependency in &self.dependencies {
            if dependency.target == self.id {
                return Err(ManifestError::SelfDependency(
                    self.id.clone(),
                ));
            }

            if !dependencies.insert(dependency.target.clone()) {
                return Err(ManifestError::DuplicateDependency(
                    dependency.target.clone(),
                ));
            }

            dependency.component_version.validate()?;
            dependency.api_version.validate()?;
        }

        validate_unique_capabilities(
            &self.required_capabilities,
        )?;

        validate_unique_capabilities(
            &self.provided_capabilities,
        )?;

        Ok(Manifest {
            id: self.id,
            kind: self.kind,
            format_version: self.format_version,
            component_version: self.component_version,
            api_version: self.api_version,
            description,
            dependencies: self.dependencies,
            required_capabilities: self.required_capabilities,
            provided_capabilities: self.provided_capabilities,
            lifecycle: self.lifecycle,
            metadata: self.metadata,
        })
    }
}

// ============================================================================
// MANIFEST VALIDATION REPORT
// ============================================================================

/// An unresolved optional component dependency.
///
/// An unresolved optional dependency is not automatically an error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedOptionalDependency {
    dependent: ComponentId,
    dependency: ComponentId,
}

impl UnresolvedOptionalDependency {
    /// Returns the component declaring the dependency.
    #[must_use]
    pub fn dependent(&self) -> &ComponentId {
        &self.dependent
    }

    /// Returns the missing optional component.
    #[must_use]
    pub fn dependency(&self) -> &ComponentId {
        &self.dependency
    }
}

/// Result of validating a collection of component manifests.
///
/// This report establishes dependency consistency only.
///
/// It does not establish provider availability, capability
/// feasibility, authorization or successful execution.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ManifestValidationReport {
    unresolved_optional: Vec<UnresolvedOptionalDependency>,
    validated_components: usize,
}

impl ManifestValidationReport {
    /// Returns the number of validated components.
    #[must_use]
    pub const fn validated_components(&self) -> usize {
        self.validated_components
    }

    /// Returns unresolved optional dependencies.
    #[must_use]
    pub fn unresolved_optional_dependencies(
        &self,
    ) -> &[UnresolvedOptionalDependency] {
        &self.unresolved_optional
    }

    /// Returns whether all optional dependencies were resolved.
    #[must_use]
    pub fn all_optional_dependencies_resolved(&self) -> bool {
        self.unresolved_optional.is_empty()
    }
}

// ============================================================================
// CROSS-MANIFEST VALIDATION
// ============================================================================

/// Validates a collection of standard-library manifests.
///
/// Checks:
///
/// - Duplicate component identities.
/// - Missing mandatory dependencies.
/// - Component version compatibility.
/// - Public API version compatibility.
/// - Resolved optional dependency compatibility.
/// - Dependency cycles.
///
/// Missing optional dependencies are reported rather than rejected.
///
/// A dependency cycle is rejected because it prevents an unambiguous
/// dependency-aware lifecycle ordering.
///
/// The algorithm is iterative and does not use recursive graph traversal.
///
/// # Errors
///
/// Returns ManifestError when the component collection is inconsistent.
pub fn validate_manifest_set(
    manifests: &[Manifest],
) -> Result<ManifestValidationReport, ManifestError> {
    let mut index: BTreeMap<&ComponentId, &Manifest> =
        BTreeMap::new();

    for manifest in manifests {
        if index.insert(manifest.id(), manifest).is_some() {
            return Err(ManifestError::DuplicateManifest(
                manifest.id().clone(),
            ));
        }
    }

    let mut report = ManifestValidationReport {
        unresolved_optional: Vec::new(),
        validated_components: manifests.len(),
    };

    // Dependency graph:
    //
    // dependency -> dependent
    //
    // This orientation makes dependency-first validation possible
    // through Kahn's cycle-detection algorithm.

    let mut outgoing: BTreeMap<&ComponentId, Vec<&ComponentId>> =
        BTreeMap::new();

    let mut indegree: BTreeMap<&ComponentId, usize> =
        BTreeMap::new();

    for manifest in manifests {
        outgoing.entry(manifest.id()).or_default();
        indegree.insert(manifest.id(), 0);
    }

    for manifest in manifests {
        for dependency in manifest.dependencies() {
            let Some(target) = index.get(dependency.target()) else {
                if dependency.is_optional() {
                    report.unresolved_optional.push(
                        UnresolvedOptionalDependency {
                            dependent: manifest.id().clone(),
                            dependency: dependency.target().clone(),
                        },
                    );

                    continue;
                }

                return Err(ManifestError::MissingDependency {
                    component: manifest.id().clone(),
                    dependency: dependency.target().clone(),
                });
            };

            if !dependency
                .component_version_requirement()
                .matches(target.component_version())
            {
                return Err(ManifestError::ComponentVersionMismatch {
                    component: manifest.id().clone(),
                    dependency: dependency.target().clone(),
                    actual: target.component_version().to_string(),
                });
            }

            if !dependency
                .api_version_requirement()
                .matches(target.api_version())
            {
                return Err(ManifestError::ApiVersionMismatch {
                    component: manifest.id().clone(),
                    dependency: dependency.target().clone(),
                    actual: target.api_version().to_string(),
                });
            }

            outgoing
                .entry(dependency.target())
                .or_default()
                .push(manifest.id());

            if let Some(count) = indegree.get_mut(manifest.id()) {
                *count += 1;
            }
        }
    }

    // Deterministic Kahn cycle detection.

    let mut ready: BTreeSet<&ComponentId> = indegree
        .iter()
        .filter_map(|(id, count)| {
            (*count == 0).then_some(*id)
        })
        .collect();

    let mut processed = 0usize;

    while let Some(id) = ready.pop_first() {
        processed += 1;

        if let Some(dependents) = outgoing.get(id) {
            for dependent in dependents {
                if let Some(count) = indegree.get_mut(dependent) {
                    *count -= 1;

                    if *count == 0 {
                        ready.insert(dependent);
                    }
                }
            }
        }
    }

    if processed != manifests.len() {
        let unresolved = indegree
            .iter()
            .filter_map(|(id, count)| {
                (*count > 0).then_some((*id).clone())
            })
            .collect();

        return Err(ManifestError::DependencyCycle(unresolved));
    }

    report.unresolved_optional.sort_by(|left, right| {
        left.dependent
            .cmp(&right.dependent)
            .then_with(|| {
                left.dependency.cmp(&right.dependency)
            })
    });

    Ok(report)
}

// ============================================================================
// MANIFEST ERRORS
// ============================================================================

/// Errors produced by manifest construction and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ManifestError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// An identifier contains invalid syntax.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,

        /// Invalid identifier.
        value: String,
    },

    /// Required description was not supplied.
    MissingDescription,

    /// A description is invalid.
    InvalidDescription {
        /// Description field.
        field: &'static str,
    },

    /// Metadata key or value is invalid.
    InvalidMetadata {
        /// Invalid metadata key.
        key: String,
    },

    /// Metadata key already exists.
    DuplicateMetadata(String),

    /// A component declares itself as a dependency.
    SelfDependency(ComponentId),

    /// A dependency was declared more than once.
    DuplicateDependency(ComponentId),

    /// A capability was declared more than once.
    DuplicateCapability(CapabilityId),

    /// A dependency target is missing.
    MissingDependency {
        /// Component declaring the dependency.
        component: ComponentId,

        /// Missing dependency.
        dependency: ComponentId,
    },

    /// Component implementation version does not satisfy a dependency.
    ComponentVersionMismatch {
        /// Component declaring the dependency.
        component: ComponentId,

        /// Dependency target.
        dependency: ComponentId,

        /// Actual component version.
        actual: String,
    },

    /// Component API version does not satisfy a dependency.
    ApiVersionMismatch {
        /// Component declaring the dependency.
        component: ComponentId,

        /// Dependency target.
        dependency: ComponentId,

        /// Actual API version.
        actual: String,
    },

    /// Dependency version interval is contradictory.
    InvalidVersionRange,

    /// Two manifests have the same component identity.
    DuplicateManifest(ComponentId),

    /// Dependency graph contains a cycle.
    ///
    /// The list contains nodes remaining after cycle detection.
    /// It may include nodes downstream of a cycle.
    DependencyCycle(Vec<ComponentId>),
}

impl fmt::Display for ManifestError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(formatter, "invalid {field}: {value}")
            }

            Self::MissingDescription => {
                formatter.write_str(
                    "component description is required",
                )
            }

            Self::InvalidDescription { field } => {
                write!(formatter, "invalid {field}")
            }

            Self::InvalidMetadata { key } => {
                write!(formatter, "invalid metadata entry: {key}")
            }

            Self::DuplicateMetadata(key) => {
                write!(formatter, "duplicate metadata key: {key}")
            }

            Self::SelfDependency(id) => {
                write!(formatter, "{id} cannot depend on itself")
            }

            Self::DuplicateDependency(id) => {
                write!(formatter, "duplicate dependency: {id}")
            }

            Self::DuplicateCapability(id) => {
                write!(formatter, "duplicate capability: {id}")
            }

            Self::MissingDependency {
                component,
                dependency,
            } => {
                write!(
                    formatter,
                    "{component} requires missing component {dependency}"
                )
            }

            Self::ComponentVersionMismatch {
                component,
                dependency,
                actual,
            } => {
                write!(
                    formatter,
                    "{component} requires an incompatible \
                     version of {dependency}; actual: {actual}"
                )
            }

            Self::ApiVersionMismatch {
                component,
                dependency,
                actual,
            } => {
                write!(
                    formatter,
                    "{component} requires an incompatible \
                     API version of {dependency}; actual: {actual}"
                )
            }

            Self::InvalidVersionRange => {
                formatter.write_str(
                    "version requirement contains contradictory boundaries",
                )
            }

            Self::DuplicateManifest(id) => {
                write!(formatter, "duplicate component manifest: {id}")
            }

            Self::DependencyCycle(ids) => {
                write!(
                    formatter,
                    "component dependency graph contains a cycle: {ids:?}"
                )
            }
        }
    }
}

impl StdError for ManifestError {}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates an open-world dot-separated identifier.
///
/// No arbitrary length or segment-count ceiling is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ManifestError> {
    if value.is_empty() {
        return Err(ManifestError::EmptyIdentifier { field });
    }

    if value.trim() != value {
        return Err(ManifestError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    let valid = value.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'
                    || byte == b'-'
            })
    });

    if !valid {
        return Err(ManifestError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validates non-empty descriptive text.
fn validate_description(
    value: &str,
    field: &'static str,
) -> Result<(), ManifestError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(ManifestError::InvalidDescription { field });
    }

    Ok(())
}

/// Validates an extension metadata entry.
fn validate_metadata(
    key: &str,
    value: &str,
) -> Result<(), ManifestError> {
    if key.is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
        || value.chars().any(char::is_control)
    {
        return Err(ManifestError::InvalidMetadata {
            key: key.to_owned(),
        });
    }

    Ok(())
}

/// Rejects duplicate capability declarations.
fn validate_unique_capabilities(
    capabilities: &[CapabilityId],
) -> Result<(), ManifestError> {
    let mut seen = BTreeSet::new();

    for capability in capabilities {
        if !seen.insert(capability) {
            return Err(ManifestError::DuplicateCapability(
                capability.clone(),
            ));
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn version(value: &str) -> Version {
        Version::parse(value).expect("valid version")
    }

    fn component(value: &str) -> ComponentId {
        ComponentId::new(value).expect("valid component")
    }

    fn kind(value: &str) -> ComponentKind {
        ComponentKind::new(value).expect("valid kind")
    }

    fn manifest(
        id: &str,
        component_version: &str,
        api_version: &str,
    ) -> ManifestBuilder {
        Manifest::builder(
            component(id),
            kind("foundation"),
            version("1.0.0"),
            version(component_version),
            version(api_version),
        )
    }

    fn build(
        id: &str,
        component_version: &str,
        api_version: &str,
    ) -> Manifest {
        let mut builder = manifest(
            id,
            component_version,
            api_version,
        );

        builder
            .description(format!("Component {id}"))
            .expect("valid description");

        builder.build().expect("valid manifest")
    }

    #[test]
    fn component_identity_is_extensible() {
        let id = component("stdlib.future.execution");

        assert_eq!(
            id.as_str(),
            "stdlib.future.execution"
        );
    }

    #[test]
    fn component_identity_rejects_invalid_segments() {
        assert!(ComponentId::new("").is_err());
        assert!(ComponentId::new(".quantum").is_err());
        assert!(ComponentId::new("quantum.").is_err());
        assert!(ComponentId::new("quantum..hdl").is_err());
        assert!(ComponentId::new("Quantum").is_err());
    }

    #[test]
    fn version_requirement_accepts_any_version() {
        assert!(VersionRequirement::any().matches(
            &version("999999999999999999999.0.0")
        ));
    }

    #[test]
    fn exact_version_uses_semver_precedence() {
        let requirement =
            VersionRequirement::exact(version("1.0.0+build.1"));

        assert!(requirement.matches(
            &version("1.0.0+build.2")
        ));
    }

    #[test]
    fn lower_bound_is_enforced() {
        let requirement =
            VersionRequirement::at_least(version("2.0.0"));

        assert!(!requirement.matches(&version("1.9.9")));
        assert!(requirement.matches(&version("2.0.0")));
        assert!(requirement.matches(&version("3.0.0")));
    }

    #[test]
    fn exclusive_boundaries_are_enforced() {
        let requirement =
            VersionRequirement::between(
                VersionBound::new(version("1.0.0"), false),
                VersionBound::new(version("2.0.0"), false),
            )
            .expect("valid range");

        assert!(!requirement.matches(&version("1.0.0")));
        assert!(requirement.matches(&version("1.5.0")));
        assert!(!requirement.matches(&version("2.0.0")));
    }

    #[test]
    fn contradictory_version_range_is_rejected() {
        let result = VersionRequirement::between(
            VersionBound::new(version("3.0.0"), true),
            VersionBound::new(version("2.0.0"), true),
        );

        assert_eq!(
            result,
            Err(ManifestError::InvalidVersionRange)
        );
    }

    #[test]
    fn component_manifest_requires_description() {
        let result = manifest(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        )
        .build();

        assert_eq!(
            result.err(),
            Some(ManifestError::MissingDescription)
        );
    }

    #[test]
    fn manifest_preserves_independent_versions() {
        let value = build(
            "stdlib.foundation",
            "3.0.0",
            "2.0.0",
        );

        assert_eq!(
            value.format_version().to_string(),
            "1.0.0"
        );

        assert_eq!(
            value.component_version().to_string(),
            "3.0.0"
        );

        assert_eq!(
            value.api_version().to_string(),
            "2.0.0"
        );
    }

    #[test]
    fn duplicate_dependencies_are_rejected() {
        let mut builder = manifest(
            "stdlib.numeric",
            "1.0.0",
            "1.0.0",
        );

        builder
            .description("Numeric foundation")
            .unwrap();

        builder.dependency(ComponentDependency::new(
            component("stdlib.foundation"),
        ));

        builder.dependency(ComponentDependency::new(
            component("stdlib.foundation"),
        ));

        assert!(matches!(
            builder.build(),
            Err(ManifestError::DuplicateDependency(_))
        ));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut builder = manifest(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Foundation").unwrap();

        builder.dependency(ComponentDependency::new(
            component("stdlib.foundation"),
        ));

        assert!(matches!(
            builder.build(),
            Err(ManifestError::SelfDependency(_))
        ));
    }

    #[test]
    fn duplicate_metadata_is_rejected() {
        let mut builder = manifest(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Foundation").unwrap();

        builder.metadata("owner", "foundation").unwrap();

        assert!(matches!(
            builder.metadata("owner", "another"),
            Err(ManifestError::DuplicateMetadata(_))
        ));
    }

    #[test]
    fn lifecycle_defaults_to_on_demand() {
        let value = build(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        );

        assert_eq!(
            value.lifecycle().initialization(),
            InitializationPolicy::OnDemand
        );
    }

    #[test]
    fn duplicate_manifest_identity_is_rejected() {
        let first = build(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        );

        let second = build(
            "stdlib.foundation",
            "2.0.0",
            "2.0.0",
        );

        assert!(matches!(
            validate_manifest_set(&[first, second]),
            Err(ManifestError::DuplicateManifest(_))
        ));
    }

    #[test]
    fn missing_required_dependency_is_rejected() {
        let mut builder = manifest(
            "stdlib.numeric",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Numeric").unwrap();

        builder.dependency(ComponentDependency::new(
            component("stdlib.foundation"),
        ));

        let numeric = builder.build().unwrap();

        assert!(matches!(
            validate_manifest_set(&[numeric]),
            Err(ManifestError::MissingDependency { .. })
        ));
    }

    #[test]
    fn missing_optional_dependency_is_reported() {
        let mut builder = manifest(
            "stdlib.quantum",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Quantum").unwrap();

        builder.dependency(
            ComponentDependency::new(
                component("stdlib.optional_accelerator"),
            )
            .optional(),
        );

        let quantum = builder.build().unwrap();

        let report =
            validate_manifest_set(&[quantum]).unwrap();

        assert_eq!(
            report.unresolved_optional_dependencies().len(),
            1
        );
    }

    #[test]
    fn compatible_dependency_versions_are_accepted() {
        let foundation = build(
            "stdlib.foundation",
            "2.0.0",
            "1.5.0",
        );

        let mut builder = manifest(
            "stdlib.numeric",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Numeric").unwrap();

        builder.dependency(
            ComponentDependency::new(
                component("stdlib.foundation"),
            )
            .component_version(
                VersionRequirement::at_least(
                    version("2.0.0"),
                ),
            )
            .api_version(
                VersionRequirement::at_least(
                    version("1.4.0"),
                ),
            ),
        );

        let numeric = builder.build().unwrap();

        assert!(validate_manifest_set(&[
            foundation,
            numeric,
        ]).is_ok());
    }

    #[test]
    fn incompatible_api_version_is_rejected() {
        let foundation = build(
            "stdlib.foundation",
            "2.0.0",
            "1.0.0",
        );

        let mut builder = manifest(
            "stdlib.numeric",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Numeric").unwrap();

        builder.dependency(
            ComponentDependency::new(
                component("stdlib.foundation"),
            )
            .api_version(
                VersionRequirement::at_least(
                    version("2.0.0"),
                ),
            ),
        );

        let numeric = builder.build().unwrap();

        assert!(matches!(
            validate_manifest_set(&[
                foundation,
                numeric,
            ]),
            Err(ManifestError::ApiVersionMismatch { .. })
        ));
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let mut first = manifest(
            "stdlib.first",
            "1.0.0",
            "1.0.0",
        );

        first.description("First").unwrap();

        first.dependency(ComponentDependency::new(
            component("stdlib.second"),
        ));

        let mut second = manifest(
            "stdlib.second",
            "1.0.0",
            "1.0.0",
        );

        second.description("Second").unwrap();

        second.dependency(ComponentDependency::new(
            component("stdlib.first"),
        ));

        let first = first.build().unwrap();
        let second = second.build().unwrap();

        assert!(matches!(
            validate_manifest_set(&[first, second]),
            Err(ManifestError::DependencyCycle(_))
        ));
    }

    #[test]
    fn acyclic_dependency_graph_is_accepted() {
        let foundation = build(
            "stdlib.foundation",
            "1.0.0",
            "1.0.0",
        );

        let mut numeric_builder = manifest(
            "stdlib.numeric",
            "1.0.0",
            "1.0.0",
        );

        numeric_builder.description("Numeric").unwrap();

        numeric_builder.dependency(ComponentDependency::new(
            component("stdlib.foundation"),
        ));

        let numeric = numeric_builder.build().unwrap();

        let report =
            validate_manifest_set(&[
                foundation,
                numeric,
            ])
            .unwrap();

        assert_eq!(report.validated_components(), 2);
    }

    #[test]
    fn capability_declarations_use_canonical_identity() {
        let mut builder = manifest(
            "stdlib.quantum",
            "1.0.0",
            "1.0.0",
        );

        builder.description("Quantum").unwrap();

        builder.requires_capability(
            CapabilityId::new("quantum.computation").unwrap(),
        );

        builder.provides_capability(
            CapabilityId::new("quantum.circuit_support").unwrap(),
        );

        let value = builder.build().unwrap();

        assert_eq!(
            value.required_capabilities().len(),
            1
        );

        assert_eq!(
            value.provided_capabilities().len(),
            1
        );
    }

    #[test]
    fn very_large_version_components_are_supported() {
        let large = version(
            "999999999999999999999999999999999999999999.0.0",
        );

        let requirement =
            VersionRequirement::exact(large.clone());

        assert!(requirement.matches(&large));
    }
}
