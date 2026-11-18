
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Environment Configuration
//!
//! Path: src/toolchain/environments/configuration.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. Purpose
//!
//! Defines explicit, immutable configuration for a Zamani development,
//! compilation, build or execution environment.
//!
//! This module coordinates environment configuration without creating
//! competing configuration, hardware, compiler, backend or runtime
//! authorities.
//!
//! It provides:
//!
//! - An explicitly identified environment configuration.
//! - Explicit platform-provider selection.
//! - Reuse of the canonical ToolchainConfig.
//! - Extensible, namespaced environment metadata.
//! - Deterministic configuration overlays.
//! - Structural configuration validation.
//! - Immutable configuration after construction.
//!
//! ## 2. Ownership
//!
//! This module owns:
//!
//! - EnvironmentConfiguration.
//! - EnvironmentConfigurationBuilder.
//! - Environment-specific descriptive attributes.
//! - Explicit environment-provider selection.
//! - Environment configuration overlay behavior.
//! - Environment configuration structural validation.
//!
//! This module does NOT own:
//!
//! - Operating-system discovery.
//! - Hardware discovery.
//! - Device enumeration.
//! - Target discovery or target selection.
//! - Capability verification.
//! - Requirement evaluation.
//! - Resource allocation or reservation.
//! - External executable discovery.
//! - External process execution.
//! - Environment fingerprint algorithms.
//! - Compiler configuration semantics.
//! - Backend configuration semantics.
//! - Runtime configuration semantics.
//! - Manifest parsing.
//! - Grammar parsing.
//! - Source-language semantics.
//! - Scheduling or deployment execution.
//!
//! ## 3. Canonical authorities
//!
//! Environment identity:
//!     crate::toolchain::ids::EnvironmentId
//!
//! Profile identity:
//!     crate::toolchain::ids::ProfileId
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Aggregate toolchain configuration:
//!     crate::toolchain::configuration::config::ToolchainConfig
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Platform observation:
//!     crate::zur::platforms::provider
//!
//! Host environment representation:
//!     crate::zur::platforms::host
//!
//! External toolchain discovery:
//!     crate::backend::toolchain
//!
//! Compiler environment identity:
//!     crate::compiler::reproducibility::environment
//!
//! Backend reproducibility:
//!     crate::backend::reproducibility
//!
//! ## 4. Configuration model
//!
//! An EnvironmentConfiguration represents explicit configuration intent.
//!
//! It does not represent a verified or currently available environment.
//!
//! Provider selection does not imply provider initialization.
//!
//! Provider initialization does not imply provider availability.
//!
//! Provider availability does not imply capability satisfaction.
//!
//! Capability satisfaction does not imply workload feasibility.
//!
//! Workload feasibility does not imply successful execution.
//!
//! These are independent stages owned by their respective subsystems.
//!
//! ## 5. Configuration categories
//!
//! This module preserves the distinction between:
//!
//! Requirement
//! Capability
//! Preference
//! Constraint
//! Hint
//! Budget
//!
//! Domain-specific meanings are interpreted by their authoritative
//! semantic, compiler, backend, security or runtime consumers.
//!
//! This module must not silently promote a preference into a requirement,
//! or reinterpret a resource requirement as a physical reservation.
//!
//! ## 6. POCO-REAF and scalability
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! No fixed maximum is imposed on:
//!
//! - Environment configurations.
//! - Platform providers.
//! - Environment attributes.
//! - Computing domains.
//! - Targets.
//! - Devices.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory.
//! - Storage.
//! - Tensor dimensions.
//! - Hardware resource counts.
//!
//! Collection growth is subject to available memory and representational
//! constraints.
//!
//! Operational budgets must be explicitly configured and enforced by
//! their owning execution authorities.
//!
//! Literal infinite physical resources or computation are not promised.
//!
//! ## 7. Determinism
//!
//! Environment attributes are stored in BTreeMap.
//!
//! Overlay operations are deterministic.
//!
//! No timestamps, process identifiers, memory addresses, random values,
//! implicit environment-variable reads or filesystem enumeration
//! participate in configuration identity.
//!
//! This module does not calculate fingerprints.
//!
//! Fingerprint construction belongs to the canonical compiler and
//! backend reproducibility subsystems.
//!
//! ## 8. Security
//!
//! This module performs no filesystem, network, process or hardware I/O.
//!
//! It does not read or modify process environment variables.
//!
//! Credentials, passwords, private keys, access tokens and other secrets
//! must not be inserted into descriptive attributes.
//!
//! Credential references and secret retrieval belong to authorized
//! security and credential providers.
//!
//! A provider identifier is a selection reference, not proof of provider
//! authenticity.
//!
//! ## 9. Integration
//!
//! Upstream:
//!
//! - ToolchainConfig.
//! - EnvironmentId.
//! - ProviderId.
//! - Explicit environment configuration supplied by the caller.
//!
//! Downstream:
//!
//! - environments/discovery.rs.
//! - environments/fingerprints.rs.
//! - environments/validation.rs.
//! - build/inputs.rs.
//! - build/plan.rs.
//! - targets/compatibility.rs.
//! - external_tools/resolution.rs.
//! - deployment/planning.rs.
//!
//! Downstream consumers must use the explicit configuration and obtain
//! observations and verification evidence from their canonical owners.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use crate::stdlib::provider::ProviderId;

use crate::toolchain::configuration::config::ToolchainConfig;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

use crate::toolchain::ids::EnvironmentId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "toolchain.environments.configuration";

const OPERATION_VALIDATE: &str =
    "toolchain.environments.configuration.validate";

const OPERATION_OVERLAY: &str =
    "toolchain.environments.configuration.merge_overlay";

// ============================================================================
// ENVIRONMENT ATTRIBUTE KEY
// ============================================================================

/// Validated, open-world key for descriptive environment configuration.
///
/// Keys are case-sensitive and preserved exactly as supplied.
///
/// Namespaced keys are recommended, for example:
///
/// - `environment.region`
/// - `environment.execution.mode`
/// - `environment.container.reference`
/// - `provider.configuration.reference`
/// - `deployment.environment.class`
/// - `vendor.extension.configuration`
///
/// These examples are identifiers, not built-in configuration semantics.
///
/// No computing domain or provider inventory is hard-coded.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentAttributeKey(String);

impl EnvironmentAttributeKey {
    /// Creates and validates an environment attribute key.
    ///
    /// The key must not be empty, whitespace-only or contain control
    /// characters.
    ///
    /// No arbitrary maximum key length is imposed.
    pub fn new(
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let value = value.into();

        validate_text(&value, "environment attribute key")?;

        Ok(Self(value))
    }

    /// Returns the exact key supplied during construction.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for EnvironmentAttributeKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for EnvironmentAttributeKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// ============================================================================
// ENVIRONMENT CONFIGURATION
// ============================================================================

/// Immutable, explicitly identified environment configuration.
///
/// This type describes configuration intent for one logical environment.
///
/// It reuses the canonical ToolchainConfig instead of creating another
/// configuration aggregate.
///
/// The environment identifier is mandatory. This prevents configurations
/// for different logical environments from being silently merged.
///
/// An optional platform provider identifies the provider explicitly selected
/// by the caller. It does not initialize or invoke that provider.
///
/// Descriptive attributes are open-world metadata. Their interpretation
/// belongs to the subsystem that owns the corresponding configuration.
///
/// # Important distinctions
///
/// This structure is not:
///
/// - A HostEnvironment observation.
/// - A hardware inventory.
/// - A capability report.
/// - A resource reservation.
/// - A compiler configuration.
/// - A runtime configuration.
/// - A deployment result.
///
/// Those representations remain with their canonical owners.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentConfiguration {
    environment_id: EnvironmentId,

    platform_provider: Option<ProviderId>,

    toolchain: ToolchainConfig,

    attributes: BTreeMap<EnvironmentAttributeKey, String>,
}

impl EnvironmentConfiguration {
    /// Creates an empty configuration for an explicitly supplied
    /// environment identity.
    ///
    /// No platform provider, target, operating system, hardware device,
    /// computing domain or execution mode is selected implicitly.
    ///
    /// The resulting configuration contains an empty canonical
    /// ToolchainConfig.
    #[must_use]
    pub fn new(environment_id: EnvironmentId) -> Self {
        Self {
            environment_id,
            platform_provider: None,
            toolchain: ToolchainConfig::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Creates a builder for an explicitly identified environment.
    #[must_use]
    pub fn builder(
        environment_id: EnvironmentId,
    ) -> EnvironmentConfigurationBuilder {
        EnvironmentConfigurationBuilder::new(environment_id)
    }

    /// Returns the canonical environment identity.
    #[must_use]
    pub fn environment_id(&self) -> &EnvironmentId {
        &self.environment_id
    }

    /// Returns the explicitly selected platform provider, if present.
    ///
    /// An absent provider means that no platform provider has been
    /// selected by this configuration.
    ///
    /// It does not mean that every platform is available.
    #[must_use]
    pub fn platform_provider(&self) -> Option<&ProviderId> {
        self.platform_provider.as_ref()
    }

    /// Returns the canonical aggregate toolchain configuration.
    ///
    /// Compiler, backend, security and domain-specific sections remain
    /// owned by their respective configuration consumers.
    #[must_use]
    pub fn toolchain(&self) -> &ToolchainConfig {
        &self.toolchain
    }

    /// Returns all descriptive environment attributes.
    ///
    /// Iteration order is deterministic.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<EnvironmentAttributeKey, String> {
        &self.attributes
    }

    /// Returns one descriptive attribute by its exact key.
    #[must_use]
    pub fn attribute(
        &self,
        key: &EnvironmentAttributeKey,
    ) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    /// Returns whether the configuration has no explicit environment
    /// provider or descriptive attributes and its aggregate toolchain
    /// configuration is empty.
    ///
    /// The mandatory environment identity is not considered a configurable
    /// setting for this query.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.platform_provider.is_none()
            && self.toolchain.is_empty()
            && self.attributes.is_empty()
    }

    /// Returns the number of descriptive attributes.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Returns whether a descriptive attribute exists.
    #[must_use]
    pub fn contains_attribute(
        &self,
        key: &EnvironmentAttributeKey,
    ) -> bool {
        self.attributes.contains_key(key)
    }

    /// Returns whether a platform provider was explicitly selected.
    #[must_use]
    pub fn has_platform_provider(&self) -> bool {
        self.platform_provider.is_some()
    }

    /// Returns the explicitly configured project root, if any.
    ///
    /// This delegates to the canonical ToolchainConfig.
    #[must_use]
    pub fn project_root(&self) -> Option<&Path> {
        self.toolchain.project_root()
    }

    /// Validates the structural configuration contract.
    ///
    /// This operation does not access the filesystem, invoke providers,
    /// inspect hardware, discover external tools, evaluate capabilities,
    /// allocate resources or establish workload feasibility.
    ///
    /// It validates the aggregate ToolchainConfig and descriptive
    /// attribute structure.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.toolchain.validate().map_err(|source| {
            ToolchainError::invalid_configuration(
                "embedded toolchain configuration is invalid",
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT)
            .with_source(source)
        })?;

        for (key, value) in &self.attributes {
            validate_text(
                key.as_str(),
                "environment attribute key",
            )?;

            validate_text(
                value,
                "environment attribute value",
            )?;
        }

        Ok(())
    }

    /// Creates a new configuration by applying another configuration
    /// as an explicit overlay.
    ///
    /// Both configurations must identify the same logical environment.
    ///
    /// Overlay rules:
    ///
    /// - The environment identity must match.
    /// - A supplied provider replaces the existing provider.
    /// - An absent overlay provider preserves the existing provider.
    /// - Canonical ToolchainConfig sections use its existing merge rules.
    /// - Overlay attributes replace matching base attributes.
    /// - Unrelated base attributes remain unchanged.
    ///
    /// An overlay cannot implicitly clear the platform provider.
    /// Use `without_platform_provider` on a builder when explicit removal
    /// is required.
    ///
    /// This operation does not mutate either input.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The environment identities differ.
    /// - The underlying ToolchainConfig overlay fails validation.
    /// - The resulting configuration is structurally invalid.
    pub fn merge_overlay(
        &self,
        overlay: &Self,
    ) -> ToolchainResult<Self> {
        if self.environment_id != overlay.environment_id {
            return Err(
                ToolchainError::invalid_configuration(
                    "cannot merge configurations belonging to different \
                     environment identities",
                )
                .with_operation(OPERATION_OVERLAY)
                .with_subject(format!(
                    "{} != {}",
                    self.environment_id,
                    overlay.environment_id
                ))
                .with_context(COMPONENT),
            );
        }

        let toolchain = self
            .toolchain
            .merge_overlay(&overlay.toolchain)
            .map_err(|source| {
                ToolchainError::invalid_configuration(
                    "canonical toolchain configuration overlay failed",
                )
                .with_operation(OPERATION_OVERLAY)
                .with_context(COMPONENT)
                .with_source(source)
            })?;

        let mut attributes = self.attributes.clone();

        attributes.extend(
            overlay
                .attributes
                .iter()
                .map(|(key, value)| {
                    (key.clone(), value.clone())
                }),
        );

        let merged = Self {
            environment_id: self.environment_id.clone(),
            platform_provider: overlay
                .platform_provider
                .clone()
                .or_else(|| self.platform_provider.clone()),
            toolchain,
            attributes,
        };

        merged.validate()?;

        Ok(merged)
    }
}

// ============================================================================
// ENVIRONMENT CONFIGURATION BUILDER
// ============================================================================

/// Mutable builder for an immutable EnvironmentConfiguration.
///
/// The builder is intended for one configuration-construction operation.
///
/// Once `build` succeeds, the resulting configuration can be shared
/// immutably across toolchain services.
///
/// Builder methods do not perform filesystem access, provider discovery,
/// process execution, hardware inspection or resource allocation.
#[derive(Debug, Clone)]
pub struct EnvironmentConfigurationBuilder {
    environment_id: EnvironmentId,

    platform_provider: Option<ProviderId>,

    toolchain: ToolchainConfig,

    attributes: BTreeMap<EnvironmentAttributeKey, String>,
}

impl EnvironmentConfigurationBuilder {
    /// Creates a builder for the supplied environment identity.
    #[must_use]
    pub fn new(environment_id: EnvironmentId) -> Self {
        Self {
            environment_id,
            platform_provider: None,
            toolchain: ToolchainConfig::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the explicitly selected platform provider.
    ///
    /// The provider is not initialized or invoked.
    ///
    /// Selection is configuration intent only.
    #[must_use]
    pub fn with_platform_provider(
        mut self,
        provider: ProviderId,
    ) -> Self {
        self.platform_provider = Some(provider);
        self
    }

    /// Removes an explicitly selected platform provider.
    ///
    /// This is an explicit operation. Absence of a provider must not be
    /// interpreted as automatic provider selection.
    #[must_use]
    pub fn without_platform_provider(mut self) -> Self {
        self.platform_provider = None;
        self
    }

    /// Replaces the embedded canonical ToolchainConfig.
    ///
    /// The supplied configuration is validated when `build` is called.
    #[must_use]
    pub fn with_toolchain_config(
        mut self,
        configuration: ToolchainConfig,
    ) -> Self {
        self.toolchain = configuration;
        self
    }

    /// Inserts or replaces a descriptive environment attribute.
    ///
    /// Attribute keys and values must not be empty, whitespace-only
    /// or contain control characters.
    ///
    /// The previous value is returned when replacing an existing key.
    ///
    /// Attributes must not contain credentials, private keys, access
    /// tokens or other secret material.
    pub fn with_attribute(
        mut self,
        key: EnvironmentAttributeKey,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "environment attribute value",
        )?;

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Inserts or replaces a descriptive attribute using a string key.
    ///
    /// The key is validated before insertion.
    pub fn with_named_attribute(
        self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.with_attribute(
            EnvironmentAttributeKey::new(key)?,
            value,
        )
    }

    /// Removes a descriptive attribute.
    ///
    /// The supplied key must be constructed through
    /// EnvironmentAttributeKey.
    pub fn without_attribute(
        mut self,
        key: &EnvironmentAttributeKey,
    ) -> Self {
        self.attributes.remove(key);
        self
    }

    /// Builds and validates the immutable configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the canonical aggregate configuration
    /// or any descriptive attribute is structurally invalid.
    pub fn build(self) -> ToolchainResult<EnvironmentConfiguration> {
        let configuration = EnvironmentConfiguration {
            environment_id: self.environment_id,
            platform_provider: self.platform_provider,
            toolchain: self.toolchain,
            attributes: self.attributes,
        };

        configuration.validate()?;

        Ok(configuration)
    }
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

/// Validates descriptive environment text.
///
/// This function deliberately does not impose:
///
/// - A fixed maximum string length.
/// - A closed list of attribute names.
/// - A fixed list of providers.
/// - A fixed list of computing domains.
/// - Hardware-specific syntax.
/// - Operating-system-specific syntax.
///
/// The consuming authority is responsible for interpreting values
/// belonging to its own configuration namespace.
fn validate_text(
    value: &str,
    description: &'static str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(
            ToolchainError::invalid_configuration(
                format!("{description} must not be empty or whitespace-only"),
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_configuration(
                format!("{description} must not contain control characters"),
            )
            .with_operation(OPERATION_VALIDATE)
            .with_context(COMPONENT),
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::provider::ProviderId;

    use crate::toolchain::ids::{
        EnvironmentId,
        IdNamespace,
    };

    fn environment_id(value: u128) -> EnvironmentId {
        EnvironmentId::new(
            IdNamespace::new("zamani.environment")
                .expect("valid namespace"),
            value,
        )
        .expect("valid environment identity")
    }

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("valid provider identity")
    }

    fn key(value: &str) -> EnvironmentAttributeKey {
        EnvironmentAttributeKey::new(value)
            .expect("valid attribute key")
    }

    fn configuration() -> EnvironmentConfiguration {
        EnvironmentConfiguration::new(environment_id(1))
    }

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn creates_empty_configuration_without_implicit_selections() {
        let configuration = configuration();

        assert!(configuration.is_empty());
        assert!(!configuration.has_platform_provider());
        assert!(configuration.platform_provider().is_none());
        assert!(configuration.attributes().is_empty());
        assert!(configuration.toolchain().is_empty());
        assert_eq!(
            configuration.attribute_count(),
            0
        );
    }

    #[test]
    fn preserves_explicit_environment_identity() {
        let identity = environment_id(42);

        let configuration =
            EnvironmentConfiguration::new(identity.clone());

        assert_eq!(
            configuration.environment_id(),
            &identity
        );
    }

    #[test]
    fn builder_constructs_valid_configuration() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("attribute")
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.attribute(&key("environment.region")),
            Some("region-a")
        );
    }

    // ------------------------------------------------------------------------
    // Provider selection
    // ------------------------------------------------------------------------

    #[test]
    fn provider_selection_is_explicit() {
        let provider = provider_id("org.zamani.platform.native");

        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_platform_provider(provider.clone())
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.platform_provider(),
            Some(&provider)
        );
    }

    #[test]
    fn provider_selection_can_be_explicitly_removed() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_platform_provider(
                    provider_id("org.example.platform"),
                )
                .without_platform_provider()
                .build()
                .expect("configuration");

        assert!(!configuration.has_platform_provider());
    }

    // ------------------------------------------------------------------------
    // Attribute validation
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_attribute_keys() {
        assert!(
            EnvironmentAttributeKey::new("").is_err()
        );

        assert!(
            EnvironmentAttributeKey::new("   ").is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_keys() {
        assert!(
            EnvironmentAttributeKey::new(
                "environment\nregion",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_attribute_values() {
        let result =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "   ",
                );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_control_characters_in_values() {
        let result =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region\ninvalid",
                );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_open_world_attribute_names() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "future.photonic.execution.configuration",
                    "provider-defined-value",
                )
                .expect("future attribute")
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.attribute(
                &key("future.photonic.execution.configuration")
            ),
            Some("provider-defined-value")
        );
    }

    #[test]
    fn does_not_impose_arbitrary_attribute_length_limits() {
        let long_value = "x".repeat(100_000);

        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.extension",
                    long_value.clone(),
                )
                .expect("long attribute")
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.attribute(
                &key("environment.extension")
            ),
            Some(long_value.as_str())
        );
    }

    // ------------------------------------------------------------------------
    // Attribute collection behavior
    // ------------------------------------------------------------------------

    #[test]
    fn attributes_are_retrieved_by_exact_key() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("attribute")
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.attribute(&key("environment.region")),
            Some("region-a")
        );

        assert_eq!(
            configuration.attribute(&key("environment.zone")),
            None
        );
    }

    #[test]
    fn duplicate_attribute_insertion_replaces_explicitly() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("first")
                .with_named_attribute(
                    "environment.region",
                    "region-b",
                )
                .expect("replacement")
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.attribute(&key("environment.region")),
            Some("region-b")
        );

        assert_eq!(
            configuration.attribute_count(),
            1
        );
    }

    // ------------------------------------------------------------------------
    // Canonical ToolchainConfig integration
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_canonical_toolchain_configuration() {
        let mut toolchain = ToolchainConfig::new();

        toolchain
            .set_project_root("projects/example")
            .expect("valid project root");

        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_toolchain_config(toolchain)
                .build()
                .expect("configuration");

        assert_eq!(
            configuration.project_root(),
            Some(Path::new("projects/example"))
        );
    }

    #[test]
    fn exposes_canonical_toolchain_configuration_without_copying() {
        let configuration = configuration();

        assert!(configuration.toolchain().is_empty());
    }

    // ------------------------------------------------------------------------
    // Overlay behavior
    // ------------------------------------------------------------------------

    #[test]
    fn overlay_preserves_unmodified_attributes() {
        let base =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("base")
                .with_named_attribute(
                    "environment.zone",
                    "zone-a",
                )
                .expect("base")
                .build()
                .expect("configuration");

        let overlay =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-b",
                )
                .expect("overlay")
                .build()
                .expect("configuration");

        let merged = base
            .merge_overlay(&overlay)
            .expect("merge");

        assert_eq!(
            merged.attribute(&key("environment.region")),
            Some("region-b")
        );

        assert_eq!(
            merged.attribute(&key("environment.zone")),
            Some("zone-a")
        );
    }

    #[test]
    fn overlay_replaces_explicit_provider() {
        let base =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_platform_provider(
                    provider_id("org.example.platform.a"),
                )
                .build()
                .expect("base");

        let overlay =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_platform_provider(
                    provider_id("org.example.platform.b"),
                )
                .build()
                .expect("overlay");

        let merged = base
            .merge_overlay(&overlay)
            .expect("merge");

        assert_eq!(
            merged.platform_provider(),
            Some(&provider_id("org.example.platform.b"))
        );
    }

    #[test]
    fn absent_overlay_provider_preserves_base_provider() {
        let provider = provider_id("org.example.platform");

        let base =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_platform_provider(provider.clone())
                .build()
                .expect("base");

        let overlay = configuration();

        let merged = base
            .merge_overlay(&overlay)
            .expect("merge");

        assert_eq!(
            merged.platform_provider(),
            Some(&provider)
        );
    }

    #[test]
    fn rejects_overlay_between_different_environments() {
        let base = configuration();

        let overlay =
            EnvironmentConfiguration::new(environment_id(2));

        let result = base.merge_overlay(&overlay);

        let error = result.expect_err(
            "different environment identities must not merge"
        );

        assert_eq!(
            error.kind(),
            crate::toolchain::errors::ToolchainErrorKind::InvalidConfiguration
        );

        assert_eq!(
            error.operation(),
            Some(OPERATION_OVERLAY)
        );
    }

    #[test]
    fn overlay_does_not_mutate_original_configurations() {
        let base =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("base")
                .build()
                .expect("configuration");

        let overlay =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-b",
                )
                .expect("overlay")
                .build()
                .expect("configuration");

        let _merged = base
            .merge_overlay(&overlay)
            .expect("merge");

        assert_eq!(
            base.attribute(&key("environment.region")),
            Some("region-a")
        );

        assert_eq!(
            overlay.attribute(&key("environment.region")),
            Some("region-b")
        );
    }

    // ------------------------------------------------------------------------
    // Determinism
    // ------------------------------------------------------------------------

    #[test]
    fn attribute_iteration_is_deterministic() {
        let configuration =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute("z.extension", "z")
                .expect("attribute")
                .with_named_attribute("a.extension", "a")
                .expect("attribute")
                .with_named_attribute("m.extension", "m")
                .expect("attribute")
                .build()
                .expect("configuration");

        let keys: Vec<_> = configuration
            .attributes()
            .keys()
            .map(EnvironmentAttributeKey::as_str)
            .collect();

        assert_eq!(
            keys,
            vec![
                "a.extension",
                "m.extension",
                "z.extension"
            ]
        );
    }

    // ------------------------------------------------------------------------
    // Thread safety
    // ------------------------------------------------------------------------

    #[test]
    fn configuration_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<EnvironmentConfiguration>();
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    #[test]
    fn empty_configuration_is_structurally_valid() {
        assert!(configuration().validate().is_ok());
    }

    #[test]
    fn builder_returns_only_valid_configurations() {
        let result =
            EnvironmentConfiguration::builder(environment_id(1))
                .with_named_attribute(
                    "environment.region",
                    "region-a",
                )
                .expect("attribute")
                .build();

        assert!(result.is_ok());
    }
}
