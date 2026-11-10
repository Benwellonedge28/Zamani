
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler
//!
//! File: src/compiler/extensions/provider.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Defines the canonical, provider-neutral contract for compiler
//! extensions participating in the Zamani compilation ecosystem.
//!
//! Providers allow independently developed implementations to
//! integrate with compiler-owned extension points without requiring
//! modifications to the compiler composition root.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Provider identity.
//! - Provider version.
//! - Provider API compatibility.
//! - Provider descriptors.
//! - Symbolic contribution categories.
//! - Symbolic extension-point declarations.
//! - Provider dependency declarations.
//! - Provider lifecycle contracts.
//! - Provider initialization context.
//! - Structured provider lifecycle errors.
//!
//! This module does not own:
//!
//! - Provider registration storage.
//! - Provider discovery.
//! - Extension loading.
//! - Dynamic library loading.
//! - Plugin sandboxing.
//! - Supply-chain verification.
//! - Backend registration.
//! - Pass registration.
//! - Target registration.
//! - Grammar extension registration.
//! - Semantic analysis.
//! - Capability definitions.
//! - Intermediate representations.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Routing.
//! - Code generation.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Backend registration:
//!     crate::backend::registry
//!
//! Compiler backend integration:
//!     crate::compiler::backends
//!
//! Compiler pass integration:
//!     crate::compiler::passes
//!
//! Canonical pass infrastructure:
//!     crate::zuir::pass
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Grammar extensions:
//!     crate::frontend::parser::extensions
//!
//! Domain grammar extensions:
//!     crate::frontend::parser::domains::extensions
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Universal IR:
//!     crate::zuir
//!
//! Supply-chain assurance:
//!     crate::compiler::security::supply_chain
//!
//! Runtime execution:
//!     crate::runtime
//!
//! # Provider versus implementation
//!
//! A provider is an independently identifiable implementation
//! participating in the extension ecosystem.
//!
//! A provider is not automatically:
//!
//! - A backend.
//! - A compiler pass.
//! - A target.
//! - A hardware device.
//! - A grammar extension.
//! - A runtime.
//!
//! Specialized interfaces remain owned by their respective
//! canonical subsystems.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! New providers must integrate through stable contracts rather
//! than requiring changes to existing computing-domain enums.
//!
//! Provider identity and contribution categories are open-world.
//!
//! No fixed number of providers, domains, extension points,
//! capabilities, targets or computational technologies is
//! established by this module.
//!
//! # Scalability
//!
//! This module introduces no arbitrary universal capacity limits.
//!
//! Actual operational limits are determined by:
//!
//! - Available memory.
//! - Address-space constraints.
//! - Configured operational budgets.
//! - Operating-system constraints.
//! - Provider implementation capabilities.
//! - Host lifecycle policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Security
//!
//! Provider metadata is not a credential store.
//!
//! This module never accepts:
//!
//! - API keys.
//! - Passwords.
//! - Private keys.
//! - Authentication tokens.
//! - Cookies.
//! - Authorization headers.
//!
//! Provider registration does not establish trust.
//!
//! Provider initialization must occur only after the appropriate
//! admission, compatibility and security checks have succeeded.
//!
//! This trait does not sandbox untrusted code.
//!
//! Untrusted extensions require an appropriate isolation boundary.
//!
//! # Determinism
//!
//! Descriptor collections use ordered standard-library collections.
//!
//! Provider registration order must not determine semantic meaning.
//!
//! Dependency resolution and provider activation ordering belong
//! to the canonical extension registry and lifecycle manager.
//!
//! # Integration rule
//!
//! Adding a new provider must not require modifying this file.
//!
//! Provider-specific functionality belongs in provider adapters
//! and the specialized extension interfaces they implement.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler provider subsystem.
pub const PROVIDER_MODULE_NAME: &str =
    "zamani.compiler.extensions.provider";

/// Version of the common compiler provider API.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler composition version.
/// - ZUIR schema version.
/// - Backend API version.
/// - Runtime version.
pub const PROVIDER_API_MAJOR: u64 = 1;

/// Current minor version of the common compiler provider API.
pub const PROVIDER_API_MINOR: u64 = 0;

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable, globally namespaced identity of an extension provider.
///
/// Provider identifiers are opaque to the compiler.
///
/// Examples:
///
/// - `org.example.compiler`
/// - `vendor.quantum.provider`
/// - `community.optimization`
/// - `organization.future.compute`
///
/// These are examples, not reserved identifiers.
///
/// Provider identity is independent of:
///
/// - Provider implementation language.
/// - Hardware architecture.
/// - Computing domain.
/// - Endpoint URL.
/// - Credentials.
/// - Provider version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    /// Creates a validated provider identifier.
    ///
    /// Identifiers consist of non-empty dot-separated segments.
    ///
    /// Each segment accepts ASCII letters, digits, underscores
    /// and hyphens.
    ///
    /// Identifiers are case-sensitive.
    ///
    /// # Errors
    ///
    /// Returns `ProviderError` for invalid identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidProviderId,
                "provider identifier must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidProviderId,
                "provider identifier must not contain surrounding whitespace",
            ));
        }

        let valid = value.split('.').all(|segment| {
            !segment.is_empty()
                && segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric()
                        || byte == b'_'
                        || byte == b'-'
                })
        });

        if !valid {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidProviderId,
                "provider identifier must contain valid dot-separated ASCII segments",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical provider identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// PROVIDER VERSION
// ============================================================================

/// Semantic version of an extension provider.
///
/// Version numbers are metadata belonging to the provider
/// implementation, not the compiler API.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct ProviderVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ProviderVersion {
    /// Constructs a provider version.
    #[must_use]
    pub const fn new(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for ProviderVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// PROVIDER API COMPATIBILITY
// ============================================================================

/// Declares the compiler provider API versions supported
/// by a provider.
///
/// A provider must declare compatibility explicitly.
///
/// A major-version mismatch is incompatible.
///
/// Minor compatibility is expressed as an inclusive interval.
///
/// An absent maximum permits later minor versions of the
/// same major version, subject to the API compatibility policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderApiCompatibility {
    major: u64,
    minimum_minor: u64,
    maximum_minor: Option<u64>,
}

impl ProviderApiCompatibility {
    /// Constructs an API compatibility interval.
    ///
    /// # Errors
    ///
    /// Returns an error when the maximum minor version is
    /// lower than the minimum minor version.
    pub fn new(
        major: u64,
        minimum_minor: u64,
        maximum_minor: Option<u64>,
    ) -> Result<Self, ProviderError> {
        if maximum_minor.is_some_and(
            |maximum| maximum < minimum_minor,
        ) {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidCompatibilityRange,
                "maximum supported minor version is below the minimum",
            ));
        }

        Ok(Self {
            major,
            minimum_minor,
            maximum_minor,
        })
    }

    /// Tests whether this declaration supports a host API version.
    #[must_use]
    pub const fn supports(
        self,
        major: u64,
        minor: u64,
    ) -> bool {
        if major != self.major || minor < self.minimum_minor {
            return false;
        }

        match self.maximum_minor {
            Some(maximum) => minor <= maximum,
            None => true,
        }
    }

    /// Returns the supported major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minimum supported minor version.
    #[must_use]
    pub const fn minimum_minor(self) -> u64 {
        self.minimum_minor
    }

    /// Returns the optional maximum supported minor version.
    #[must_use]
    pub const fn maximum_minor(self) -> Option<u64> {
        self.maximum_minor
    }
}

// ============================================================================
// PROVIDER DEPENDENCY REQUIREMENTS
// ============================================================================

/// A version interval for a required provider.
///
/// The minimum is inclusive.
///
/// The optional maximum is exclusive.
///
/// This representation allows dependencies to be checked
/// without requiring a particular third-party versioning crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderVersionRequirement {
    minimum_inclusive: ProviderVersion,
    maximum_exclusive: Option<ProviderVersion>,
}

impl ProviderVersionRequirement {
    /// Constructs a provider version requirement.
    ///
    /// # Errors
    ///
    /// Rejects an empty or reversed version interval.
    pub fn new(
        minimum_inclusive: ProviderVersion,
        maximum_exclusive: Option<ProviderVersion>,
    ) -> Result<Self, ProviderError> {
        if maximum_exclusive.is_some_and(
            |maximum| maximum <= minimum_inclusive,
        ) {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidDependencyVersionRange,
                "maximum exclusive provider version must exceed the minimum",
            ));
        }

        Ok(Self {
            minimum_inclusive,
            maximum_exclusive,
        })
    }

    /// Creates a requirement accepting one exact version.
    #[must_use]
    pub const fn exact(
        version: ProviderVersion,
    ) -> Self {
        Self {
            minimum_inclusive: version,
            maximum_exclusive: None,
        }
    }

    /// Tests whether a provider version satisfies this requirement.
    #[must_use]
    pub const fn accepts(
        self,
        version: ProviderVersion,
    ) -> bool {
        if version < self.minimum_inclusive {
            return false;
        }

        match self.maximum_exclusive {
            Some(maximum) => version < maximum,
            None => version == self.minimum_inclusive
                || version > self.minimum_inclusive,
        }
    }

    /// Returns the inclusive minimum version.
    #[must_use]
    pub const fn minimum_inclusive(
        self,
    ) -> ProviderVersion {
        self.minimum_inclusive
    }

    /// Returns the optional exclusive maximum version.
    #[must_use]
    pub const fn maximum_exclusive(
        self,
    ) -> Option<ProviderVersion> {
        self.maximum_exclusive
    }
}

/// A dependency on another registered provider.
///
/// Dependency identity is independent of provider implementation
/// and registration order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDependency {
    provider_id: ProviderId,
    version_requirement: ProviderVersionRequirement,
}

impl ProviderDependency {
    /// Constructs a provider dependency.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        version_requirement: ProviderVersionRequirement,
    ) -> Self {
        Self {
            provider_id,
            version_requirement,
        }
    }

    /// Returns the required provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the required version interval.
    #[must_use]
    pub const fn version_requirement(
        &self,
    ) -> ProviderVersionRequirement {
        self.version_requirement
    }
}

// ============================================================================
// SYMBOLIC EXTENSION CONTRIBUTION IDENTITY
// ============================================================================

/// Open-world identity of a contribution category.
///
/// This is intentionally a string-backed identifier rather than
/// a closed enumeration of current computing technologies.
///
/// Examples:
///
/// - `backend`
/// - `compiler.pass`
/// - `target.discovery`
/// - `grammar.extension`
/// - `domain.integration`
/// - `analysis.provider`
/// - `future.computing.extension`
///
/// These examples are not reserved categories.
///
/// The authoritative specialized subsystem determines the
/// meaning of each contribution category.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContributionKind(String);

impl ContributionKind {
    /// Creates a validated contribution category.
    ///
    /// The identifier uses dot-separated ASCII segments.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidContributionKind,
                "contribution category must be non-empty and trimmed",
            ));
        }

        let valid = value.split('.').all(|segment| {
            !segment.is_empty()
                && segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric()
                        || byte == b'_'
                        || byte == b'-'
                })
        });

        if !valid {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidContributionKind,
                "contribution category contains an invalid identifier segment",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical category.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContributionKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SYMBOLIC EXTENSION POINT IDENTITY
// ============================================================================

/// Stable symbolic identity of a compiler extension point.
///
/// This identity is independent of grammar-specific enum variants.
///
/// The frontend grammar remains authoritative for source-level
/// extension-point syntax and interpretation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionPointId(String);

impl ExtensionPointId {
    /// Constructs an extension-point identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is invalid.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidExtensionPoint,
                "extension-point identity must be non-empty and trimmed",
            ));
        }

        let valid = value.split("::").all(|segment| {
            !segment.is_empty()
                && segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric()
                        || byte == b'_'
                        || byte == b'-'
                        || byte == b'.'
                })
        });

        if !valid {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidExtensionPoint,
                "extension-point identity contains an invalid segment",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical extension-point identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExtensionPointId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable declarative metadata describing an extension provider.
///
/// A descriptor does not contain executable code, credentials,
/// hardware handles, or runtime resources.
///
/// The descriptor is captured and validated by the extension
/// registry before provider activation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptor {
    id: ProviderId,
    version: ProviderVersion,
    api_compatibility: ProviderApiCompatibility,
    name: String,
    description: String,
    contribution_kinds: BTreeSet<ContributionKind>,
    extension_points: BTreeSet<ExtensionPointId>,
    dependencies: BTreeMap<ProviderId, ProviderVersionRequirement>,
}

impl ProviderDescriptor {
    /// Creates a provider descriptor.
    ///
    /// A provider must declare at least one contribution category.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is invalid.
    pub fn new(
        id: ProviderId,
        version: ProviderVersion,
        api_compatibility: ProviderApiCompatibility,
        name: impl Into<String>,
        description: impl Into<String>,
        contribution_kinds: impl IntoIterator<Item = ContributionKind>,
    ) -> Result<Self, ProviderError> {
        let name = name.into();
        let description = description.into();

        if name.trim().is_empty() {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidDescriptor,
                "provider display name must not be empty",
            )
            .with_provider(id.clone()));
        }

        if name.chars().any(char::is_control)
            || description.chars().any(char::is_control)
        {
            return Err(ProviderError::new(
                ProviderErrorKind::InvalidDescriptor,
                "provider display metadata contains control characters",
            )
            .with_provider(id.clone()));
        }

        let contribution_kinds: BTreeSet<_> =
            contribution_kinds.into_iter().collect();

        if contribution_kinds.is_empty() {
            return Err(ProviderError::new(
                ProviderErrorKind::NoContributionKinds,
                "provider must declare at least one contribution category",
            )
            .with_provider(id.clone()));
        }

        Ok(Self {
            id,
            version,
            api_compatibility,
            name,
            description,
            contribution_kinds,
            extension_points: BTreeSet::new(),
            dependencies: BTreeMap::new(),
        })
    }

    /// Adds a symbolic compiler extension point.
    ///
    /// Duplicate declarations are harmless and normalized
    /// by the ordered set.
    #[must_use]
    pub fn with_extension_point(
        mut self,
        extension_point: ExtensionPointId,
    ) -> Self {
        self.extension_points.insert(extension_point);
        self
    }

    /// Adds multiple symbolic extension points.
    #[must_use]
    pub fn with_extension_points(
        mut self,
        extension_points: impl IntoIterator<Item = ExtensionPointId>,
    ) -> Self {
        self.extension_points.extend(extension_points);
        self
    }

    /// Adds a provider dependency.
    ///
    /// A provider cannot depend on itself.
    ///
    /// Duplicate dependency declarations are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error for self-dependencies or duplicate dependencies.
    pub fn requiring(
        mut self,
        dependency: ProviderDependency,
    ) -> Result<Self, ProviderError> {
        if dependency.provider_id == self.id {
            return Err(ProviderError::new(
                ProviderErrorKind::SelfDependency,
                "a provider cannot depend on itself",
            )
            .with_provider(self.id.clone()));
        }

        let dependency_id = dependency.provider_id;

        if self.dependencies.contains_key(&dependency_id) {
            return Err(ProviderError::new(
                ProviderErrorKind::DuplicateDependency,
                format!(
                    "provider dependency `{dependency_id}` is already declared"
                ),
            )
            .with_provider(self.id.clone()));
        }

        self.dependencies.insert(
            dependency_id,
            dependency.version_requirement,
        );

        Ok(self)
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    /// Returns the provider version.
    #[must_use]
    pub const fn version(&self) -> ProviderVersion {
        self.version
    }

    /// Returns the provider API compatibility declaration.
    #[must_use]
    pub const fn api_compatibility(
        &self,
    ) -> ProviderApiCompatibility {
        self.api_compatibility
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the provider description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the declared contribution categories.
    #[must_use]
    pub fn contribution_kinds(
        &self,
    ) -> &BTreeSet<ContributionKind> {
        &self.contribution_kinds
    }

    /// Returns declared compiler extension points.
    #[must_use]
    pub fn extension_points(
        &self,
    ) -> &BTreeSet<ExtensionPointId> {
        &self.extension_points
    }

    /// Returns declared provider dependencies.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &BTreeMap<ProviderId, ProviderVersionRequirement> {
        &self.dependencies
    }

    /// Checks compatibility with a compiler provider API version.
    #[must_use]
    pub const fn supports_api(
        &self,
        major: u64,
        minor: u64,
    ) -> bool {
        self.api_compatibility.supports(major, minor)
    }

    /// Checks whether this provider declares a contribution category.
    #[must_use]
    pub fn supports_contribution(
        &self,
        contribution: &ContributionKind,
    ) -> bool {
        self.contribution_kinds.contains(contribution)
    }

    /// Checks whether this provider declares an extension point.
    #[must_use]
    pub fn supports_extension_point(
        &self,
        extension_point: &ExtensionPointId,
    ) -> bool {
        self.extension_points.contains(extension_point)
    }
}

// ============================================================================
// PROVIDER HOST CONTEXT
// ============================================================================

/// Immutable host information supplied during provider lifecycle
/// operations.
///
/// This context deliberately exposes no global registry, mutable
/// compiler state, physical device handle, or credential store.
///
/// Specialized integration handles must be supplied through their
/// authoritative subsystem interfaces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderContext {
    provider_id: ProviderId,
    host_api_major: u64,
    host_api_minor: u64,
}

impl ProviderContext {
    /// Constructs the provider host context.
    ///
    /// The lifecycle manager supplies the identity of the provider
    /// being initialized and the actual host API version.
    #[must_use]
    pub const fn new(
        provider_id: ProviderId,
        host_api_major: u64,
        host_api_minor: u64,
    ) -> Self {
        Self {
            provider_id,
            host_api_major,
            host_api_minor,
        }
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the host API major version.
    #[must_use]
    pub const fn host_api_major(&self) -> u64 {
        self.host_api_major
    }

    /// Returns the host API minor version.
    #[must_use]
    pub const fn host_api_minor(&self) -> u64 {
        self.host_api_minor
    }
}

// ============================================================================
// PROVIDER ERROR MODEL
// ============================================================================

/// Machine-readable category for provider contract failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProviderErrorKind {
    /// Invalid provider identity.
    InvalidProviderId,

    /// Invalid provider descriptor.
    InvalidDescriptor,

    /// Invalid provider API compatibility range.
    InvalidCompatibilityRange,

    /// Provider does not declare contribution categories.
    NoContributionKinds,

    /// Invalid contribution category.
    InvalidContributionKind,

    /// Invalid extension-point identity.
    InvalidExtensionPoint,

    /// Invalid provider dependency version interval.
    InvalidDependencyVersionRange,

    /// A provider depends on itself.
    SelfDependency,

    /// A dependency is declared more than once.
    DuplicateDependency,

    /// Provider does not support the host API.
    IncompatibleHostApi,

    /// Provider initialization failed.
    InitializationFailed,

    /// Provider shutdown failed.
    ShutdownFailed,

    /// Provider lifecycle context does not match its identity.
    ContextMismatch,

    /// Provider operation was cancelled.
    Cancelled,

    /// Provider operation exceeded an operational budget.
    ResourceLimitExceeded,

    /// Provider encountered an internal failure.
    InternalFailure,
}

/// Structured error produced by the provider contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderError {
    kind: ProviderErrorKind,
    message: String,
    provider_id: Option<ProviderId>,
}

impl ProviderError {
    /// Creates a structured provider error.
    #[must_use]
    pub fn new(
        kind: ProviderErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            provider_id: None,
        }
    }

    /// Associates an error with its provider.
    #[must_use]
    pub fn with_provider(
        mut self,
        provider_id: ProviderId,
    ) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> ProviderErrorKind {
        self.kind
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the associated provider identity.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }
}

impl fmt::Display for ProviderError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{:?}: {}",
            self.kind,
            self.message
        )?;

        if let Some(provider_id) = &self.provider_id {
            write!(
                formatter,
                " [provider: {provider_id}]"
            )?;
        }

        Ok(())
    }
}

impl Error for ProviderError {}

// ============================================================================
// PROVIDER LIFECYCLE CONTRACT
// ============================================================================

/// Common compiler extension provider.
///
/// This trait defines provider identity and lifecycle integration.
///
/// It does not replace specialized interfaces such as:
///
/// - `Backend`.
/// - `GrammarExtension`.
/// - `TargetDiscoveryProvider`.
/// - Canonical ZUIR pass contracts.
///
/// Providers implement the relevant specialized interfaces
/// separately.
///
/// # Thread safety
///
/// Implementations must be `Send + Sync`.
///
/// A provider may be shared between independent compiler sessions.
///
/// Mutable per-compilation state must not be stored in the provider
/// object without appropriate synchronization and isolation.
///
/// # Lifecycle
///
/// The lifecycle manager owns initialization and shutdown ordering.
///
/// It must:
///
/// 1. Validate the descriptor.
/// 2. Check host API compatibility.
/// 3. Resolve provider dependencies.
/// 4. Complete security admission.
/// 5. Initialize dependencies before dependants.
/// 6. Initialize the provider.
/// 7. Prevent use after failed initialization.
/// 8. Shut down initialized providers according to dependency order.
///
/// Initialization and shutdown must be explicit operations.
///
/// Implementations should make initialization safe against
/// accidental repeated calls.
///
/// # Security
///
/// This trait does not provide a sandbox.
///
/// Untrusted provider code must not be activated in-process
/// without an appropriate security decision and isolation boundary.
pub trait ExtensionProvider: Send + Sync {
    /// Returns immutable provider metadata.
    ///
    /// The descriptor must remain stable for the provider's lifetime.
    fn descriptor(&self) -> &ProviderDescriptor;

    /// Initializes the provider after host validation and admission.
    ///
    /// The context identity must match `descriptor().id()`.
    ///
    /// # Errors
    ///
    /// Returns a structured error if initialization fails.
    fn initialize(
        &self,
        context: &ProviderContext,
    ) -> Result<(), ProviderError>;

    /// Shuts down an initialized provider.
    ///
    /// The lifecycle manager must not call this method for a provider
    /// whose initialization did not complete successfully.
    ///
    /// Implementations should release resources they own and should
    /// not shut down resources owned by another subsystem.
    ///
    /// # Errors
    ///
    /// Returns a structured error if shutdown fails.
    fn shutdown(
        &self,
        context: &ProviderContext,
    ) -> Result<(), ProviderError>;
}

// ============================================================================
// PROVIDER VALIDATION HELPERS
// ============================================================================

/// Validates a provider descriptor against a host API version.
///
/// This operation does not initialize the provider.
///
/// It does not establish trust, availability or capability support.
///
/// # Errors
///
/// Returns an error if the provider API compatibility declaration
/// does not accept the host API version.
pub fn validate_provider_compatibility(
    descriptor: &ProviderDescriptor,
    host_api_major: u64,
    host_api_minor: u64,
) -> Result<(), ProviderError> {
    if !descriptor.supports_api(
        host_api_major,
        host_api_minor,
    ) {
        return Err(
            ProviderError::new(
                ProviderErrorKind::IncompatibleHostApi,
                format!(
                    "provider supports API {:?}, but host requires {}.{}",
                    descriptor.api_compatibility(),
                    host_api_major,
                    host_api_minor
                ),
            )
            .with_provider(descriptor.id().clone()),
        );
    }

    Ok(())
}

/// Validates that a provider lifecycle context belongs to
/// the provider described by the descriptor.
///
/// # Errors
///
/// Returns an error when the identities do not match.
pub fn validate_provider_context(
    descriptor: &ProviderDescriptor,
    context: &ProviderContext,
) -> Result<(), ProviderError> {
    if descriptor.id() != context.provider_id() {
        return Err(
            ProviderError::new(
                ProviderErrorKind::ContextMismatch,
                "provider context identity does not match provider descriptor",
            )
            .with_provider(descriptor.id().clone()),
        );
    }

    validate_provider_compatibility(
        descriptor,
        context.host_api_major(),
        context.host_api_minor(),
    )
}

// ============================================================================
// MODULE HELPERS
// ============================================================================

/// Returns the stable provider module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    PROVIDER_MODULE_NAME
}

/// Returns the current provider API major version.
#[must_use]
pub const fn api_major() -> u64 {
    PROVIDER_API_MAJOR
}

/// Returns the current provider API minor version.
#[must_use]
pub const fn api_minor() -> u64 {
    PROVIDER_API_MINOR
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("valid provider identity")
    }

    fn contribution(value: &str) -> ContributionKind {
        ContributionKind::new(value)
            .expect("valid contribution category")
    }

    fn compatibility() -> ProviderApiCompatibility {
        ProviderApiCompatibility::new(
            PROVIDER_API_MAJOR,
            0,
            None,
        )
        .expect("valid API compatibility")
    }

    fn descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            provider_id("example.compiler"),
            ProviderVersion::new(1, 0, 0),
            compatibility(),
            "Example Provider",
            "Example compiler extension provider",
            [contribution("compiler.pass")],
        )
        .expect("valid provider descriptor")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.extensions.provider"
        );
    }

    #[test]
    fn provider_api_version_is_available() {
        assert_ne!(api_major(), 0);
    }

    #[test]
    fn accepts_namespaced_provider_ids() {
        assert_eq!(
            provider_id("vendor.quantum.provider").as_str(),
            "vendor.quantum.provider"
        );
    }

    #[test]
    fn rejects_invalid_provider_ids() {
        for value in [
            "",
            ".vendor",
            "vendor.",
            "vendor..provider",
            "vendor provider",
            " vendor",
            "vendor ",
        ] {
            assert!(
                ProviderId::new(value).is_err(),
                "invalid provider identity accepted: {value:?}"
            );
        }
    }

    #[test]
    fn provider_versions_have_numeric_ordering() {
        assert!(
            ProviderVersion::new(1, 10, 0)
                > ProviderVersion::new(1, 9, 9)
        );
    }

    #[test]
    fn provider_api_compatibility_checks_major_and_minor() {
        let api = ProviderApiCompatibility::new(
            1,
            2,
            Some(5),
        )
        .expect("valid compatibility");

        assert!(api.supports(1, 2));
        assert!(api.supports(1, 5));
        assert!(!api.supports(1, 6));
        assert!(!api.supports(2, 2));
        assert!(!api.supports(1, 1));
    }

    #[test]
    fn rejects_reversed_api_compatibility() {
        assert!(
            ProviderApiCompatibility::new(
                1,
                5,
                Some(3)
            )
            .is_err()
        );
    }

    #[test]
    fn exact_provider_version_requirement_is_exact() {
        let version = ProviderVersion::new(1, 2, 3);
        let requirement = ProviderVersionRequirement::exact(version);

        assert!(requirement.accepts(version));

        assert!(
            !requirement.accepts(
                ProviderVersion::new(1, 2, 4)
            )
        );

        assert!(
            !requirement.accepts(
                ProviderVersion::new(1, 2, 2)
            )
        );
    }

    #[test]
    fn provider_version_ranges_are_checked() {
        let requirement = ProviderVersionRequirement::new(
            ProviderVersion::new(1, 0, 0),
            Some(ProviderVersion::new(2, 0, 0)),
        )
        .expect("valid version interval");

        assert!(
            requirement.accepts(
                ProviderVersion::new(1, 5, 0)
            )
        );

        assert!(
            !requirement.accepts(
                ProviderVersion::new(2, 0, 0)
            )
        );
    }

    #[test]
    fn rejects_empty_provider_version_ranges() {
        assert!(
            ProviderVersionRequirement::new(
                ProviderVersion::new(2, 0, 0),
                Some(ProviderVersion::new(2, 0, 0)),
            )
            .is_err()
        );
    }

    #[test]
    fn contribution_categories_are_open_world() {
        let categories = [
            "backend",
            "compiler.pass",
            "target.discovery",
            "grammar.extension",
            "future.computing.provider",
        ];

        for category in categories {
            assert!(
                ContributionKind::new(category).is_ok()
            );
        }
    }

    #[test]
    fn rejects_invalid_contribution_categories() {
        for category in [
            "",
            ".backend",
            "backend.",
            "backend..pass",
            "backend pass",
        ] {
            assert!(
                ContributionKind::new(category).is_err()
            );
        }
    }

    #[test]
    fn extension_points_are_symbolic() {
        for value in [
            "quantum::operation",
            "classical::numeric",
            "hdl::module",
            "distributed::transport",
            "future::computing::facility",
        ] {
            assert!(
                ExtensionPointId::new(value).is_ok()
            );
        }
    }

    #[test]
    fn descriptor_preserves_provider_identity_and_version() {
        let descriptor = descriptor();

        assert_eq!(
            descriptor.id().as_str(),
            "example.compiler"
        );

        assert_eq!(
            descriptor.version(),
            ProviderVersion::new(1, 0, 0)
        );

        assert_eq!(
            descriptor.name(),
            "Example Provider"
        );
    }

    #[test]
    fn descriptor_rejects_missing_contribution_categories() {
        let result = ProviderDescriptor::new(
            provider_id("example.empty"),
            ProviderVersion::new(1, 0, 0),
            compatibility(),
            "Empty Provider",
            "",
            [],
        );

        assert!(matches!(
            result,
            Err(ProviderError {
                kind: ProviderErrorKind::NoContributionKinds,
                ..
            })
        ));
    }

    #[test]
    fn descriptor_supports_multiple_extension_points() {
        let first = ExtensionPointId::new(
            "quantum::operation"
        )
        .expect("valid extension point");

        let second = ExtensionPointId::new(
            "quantum::measurement"
        )
        .expect("valid extension point");

        let descriptor = descriptor()
            .with_extension_points([
                first.clone(),
                second.clone(),
            ]);

        assert!(
            descriptor.supports_extension_point(&first)
        );

        assert!(
            descriptor.supports_extension_point(&second)
        );
    }

    #[test]
    fn descriptor_rejects_self_dependency() {
        let dependency = ProviderDependency::new(
            provider_id("example.compiler"),
            ProviderVersionRequirement::exact(
                ProviderVersion::new(1, 0, 0),
            ),
        );

        assert!(
            descriptor().requiring(dependency).is_err()
        );
    }

    #[test]
    fn descriptor_rejects_duplicate_dependencies() {
        let dependency_id = provider_id("example.foundation");

        let requirement = ProviderVersionRequirement::exact(
            ProviderVersion::new(1, 0, 0),
        );

        let descriptor = descriptor()
            .requiring(
                ProviderDependency::new(
                    dependency_id.clone(),
                    requirement,
                )
            )
            .expect("first dependency");

        assert!(
            descriptor.requiring(
                ProviderDependency::new(
                    dependency_id,
                    requirement,
                )
            ).is_err()
        );
    }

    #[test]
    fn descriptor_supports_declared_contribution_categories() {
        let descriptor = descriptor();

        assert!(
            descriptor.supports_contribution(
                &contribution("compiler.pass")
            )
        );

        assert!(
            !descriptor.supports_contribution(
                &contribution("backend")
            )
        );
    }

    #[test]
    fn validates_compatible_provider() {
        let descriptor = descriptor();

        assert!(
            validate_provider_compatibility(
                &descriptor,
                PROVIDER_API_MAJOR,
                PROVIDER_API_MINOR,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_incompatible_provider() {
        let descriptor = descriptor();

        let result = validate_provider_compatibility(
            &descriptor,
            PROVIDER_API_MAJOR + 1,
            0,
        );

        assert!(matches!(
            result,
            Err(ProviderError {
                kind: ProviderErrorKind::IncompatibleHostApi,
                ..
            })
        ));
    }

    #[test]
    fn validates_matching_provider_context() {
        let descriptor = descriptor();

        let context = ProviderContext::new(
            descriptor.id().clone(),
            PROVIDER_API_MAJOR,
            PROVIDER_API_MINOR,
        );

        assert!(
            validate_provider_context(
                &descriptor,
                &context,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_mismatched_provider_context() {
        let descriptor = descriptor();

        let context = ProviderContext::new(
            provider_id("another.provider"),
            PROVIDER_API_MAJOR,
            PROVIDER_API_MINOR,
        );

        assert!(matches!(
            validate_provider_context(
                &descriptor,
                &context,
            ),
            Err(ProviderError {
                kind: ProviderErrorKind::ContextMismatch,
                ..
            })
        ));
    }

    #[test]
    fn provider_error_preserves_identity_and_category() {
        let error = ProviderError::new(
            ProviderErrorKind::InitializationFailed,
            "initialization failed",
        )
        .with_provider(provider_id("example.compiler"));

        assert_eq!(
            error.kind(),
            ProviderErrorKind::InitializationFailed
        );

        assert_eq!(
            error.provider_id().unwrap().as_str(),
            "example.compiler"
        );

        assert!(
            error.to_string().contains("initialization failed")
        );
    }
}
