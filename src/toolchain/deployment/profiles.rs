#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Profiles
//!
//! Path: `src/toolchain/deployment/profiles.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Adds deployment-specific settings to the canonical toolchain profile
//! system. Deployment profiles describe reusable deployment intent and
//! policy references without implementing deployment execution.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - DeploymentProfileSettings.
//! - DeploymentProfileDefinition.
//! - ResolvedDeploymentProfile.
//! - DeploymentProfileRegistry, as an adapter over ProfileRegistry.
//! - Serialization and validation of deployment profile settings.
//!
//! ## Reused authorities
//!
//! - `configuration::profiles::ProfileDefinition` owns generic profiles.
//! - `configuration::profiles::ProfileRegistry` owns registration,
//!   inheritance, cycle detection and generic resolution.
//! - `configuration::profiles::ResolvedProfile` owns resolved overlays.
//! - `configuration::config::ToolchainConfig` owns configuration storage.
//! - `configuration::config::ConfigSectionKey` owns section identities.
//! - `ids::ProfileId` owns profile identity.
//! - `errors::ToolchainError` owns toolchain errors.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Source grammar, AST or semantic analysis.
//! - Requirement or capability evaluation.
//! - Target discovery or target selection.
//! - Physical resource discovery or allocation.
//! - Deployment planning or infrastructure provisioning.
//! - Provider execution.
//! - Runtime execution or scheduling.
//! - Operational budget accounting.
//! - Authentication, authorization or secret management.
//! - Artifact creation, verification or storage.
//!
//! ## Semantic boundary
//!
//! A deployment profile stores intent. It does not prove that a deployment
//! is feasible, authorized, supported or available.
//!
//! Requirements, capabilities, preferences, constraints and hints remain
//! distinct. In particular, a profile cannot assert that a provider or
//! machine actually possesses a capability.
//!
//! ## Scalability
//!
//! No fixed limits are imposed on profile count, properties, metadata,
//! deployment domains, targets, providers, machines, processors,
//! accelerators, memory, storage, quantum resources or distributed nodes.
//!
//! Collections grow subject to available memory and representational
//! constraints. Operational budgets remain owned by their canonical
//! configuration and execution authorities.
//!
//! ## Configuration storage
//!
//! Deployment-specific settings are stored inside the generic profile's
//! namespaced configuration sections. The generic profile registry remains
//! the only profile registry.
//!
//! The section key is stable and independent of any particular provider,
//! hardware architecture, deployment environment or computing domain.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::toolchain::configuration::config::{
    ConfigSectionKey,
    ToolchainConfig,
};
use crate::toolchain::configuration::profiles::{
    ProfileDefinition,
    ProfileRegistry,
    ResolvedProfile,
};
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};
use crate::toolchain::ids::ProfileId;

/// Stable configuration section containing deployment profile settings.
const DEPLOYMENT_PROFILE_SECTION: &str = "toolchain.deployment.profile";

/// Current serialized deployment-profile settings schema version.
///
/// This versions the settings payload, not the Zamani language, deployment
/// protocol, provider API or generic profile registry.
pub const DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// DEPLOYMENT PROFILE SETTINGS
// ============================================================================

/// Deployment-specific intent and policy settings stored in a generic
/// toolchain profile.
///
/// The maps are open-world. Their keys are namespaced identifiers interpreted
/// by their authoritative semantic, policy, environment or provider
/// subsystem.
///
/// Values are opaque JSON data at this layer. A successful structural
/// validation here does not establish that a value is semantically valid
/// for a particular deployment.
///
/// # Semantic categories
///
/// - `requirements`: conditions the deployment must satisfy.
/// - `constraints`: mandatory restrictions on acceptable realizations.
/// - `preferences`: desirable properties that do not become requirements.
/// - `hints`: advisory information that must not be treated as a guarantee.
/// - `policy_references`: references to canonical policy definitions.
/// - `lifecycle`: lifecycle intent interpreted by deployment orchestration.
/// - `rollout`: rollout intent interpreted by deployment orchestration.
/// - `availability`: availability intent, not a reservation or guarantee.
/// - `portability`: portability intent, not proof of universal support.
/// - `recovery`: recovery intent interpreted by its owning subsystem.
/// - `observability`: observability intent interpreted by its owning subsystem.
/// - `parameters`: non-secret deployment parameters.
/// - `provider_options`: provider-scoped extension data, validated by the
///   selected provider before use.
///
/// Capabilities are deliberately not stored as claims in this structure.
/// Actual capabilities must come from trusted capability authorities.
///
/// Credentials, tokens, private keys and other secrets must not be stored
/// in these settings. Use the canonical credential and security providers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentProfileSettings {
    /// Version of this serialized settings schema.
    pub schema_version: u32,

    /// Mandatory deployment requirements.
    #[serde(default)]
    pub requirements: BTreeMap<String, serde_json::Value>,

    /// Mandatory deployment constraints.
    #[serde(default)]
    pub constraints: BTreeMap<String, serde_json::Value>,

    /// Desirable deployment preferences.
    #[serde(default)]
    pub preferences: BTreeMap<String, serde_json::Value>,

    /// Advisory deployment hints.
    #[serde(default)]
    pub hints: BTreeMap<String, serde_json::Value>,

    /// References to canonical policy definitions.
    #[serde(default)]
    pub policy_references: BTreeMap<String, serde_json::Value>,

    /// Deployment lifecycle intent.
    #[serde(default)]
    pub lifecycle: BTreeMap<String, serde_json::Value>,

    /// Deployment rollout intent.
    #[serde(default)]
    pub rollout: BTreeMap<String, serde_json::Value>,

    /// Availability intent.
    #[serde(default)]
    pub availability: BTreeMap<String, serde_json::Value>,

    /// Portability intent.
    #[serde(default)]
    pub portability: BTreeMap<String, serde_json::Value>,

    /// Recovery intent.
    #[serde(default)]
    pub recovery: BTreeMap<String, serde_json::Value>,

    /// Observability intent.
    #[serde(default)]
    pub observability: BTreeMap<String, serde_json::Value>,

    /// Non-secret deployment parameters.
    #[serde(default)]
    pub parameters: BTreeMap<String, serde_json::Value>,

    /// Provider-scoped extension settings.
    ///
    /// These values are opaque to the deployment-profile subsystem.
    /// A provider must validate its own options before relying on them.
    #[serde(default)]
    pub provider_options: BTreeMap<String, serde_json::Value>,
}

impl Default for DeploymentProfileSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl DeploymentProfileSettings {
    /// Construct empty deployment settings using the current schema version.
    ///
    /// Empty settings express no additional deployment requirements,
    /// preferences, constraints or provider-specific options.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_version: DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION,
            requirements: BTreeMap::new(),
            constraints: BTreeMap::new(),
            preferences: BTreeMap::new(),
            hints: BTreeMap::new(),
            policy_references: BTreeMap::new(),
            lifecycle: BTreeMap::new(),
            rollout: BTreeMap::new(),
            availability: BTreeMap::new(),
            portability: BTreeMap::new(),
            recovery: BTreeMap::new(),
            observability: BTreeMap::new(),
            parameters: BTreeMap::new(),
            provider_options: BTreeMap::new(),
        }
    }

    /// Validate the structural contract of these settings.
    ///
    /// This checks schema compatibility and property-key structure. It does
    /// not interpret domain-specific values or evaluate deployment feasibility.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.schema_version
            != DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION
        {
            return Err(invalid_settings(format!(
                "unsupported deployment profile settings schema version `{}`; supported version is `{}`",
                self.schema_version,
                DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION,
            )));
        }

        validate_property_map("requirements", &self.requirements)?;
        validate_property_map("constraints", &self.constraints)?;
        validate_property_map("preferences", &self.preferences)?;
        validate_property_map("hints", &self.hints)?;
        validate_property_map("policy_references", &self.policy_references)?;
        validate_property_map("lifecycle", &self.lifecycle)?;
        validate_property_map("rollout", &self.rollout)?;
        validate_property_map("availability", &self.availability)?;
        validate_property_map("portability", &self.portability)?;
        validate_property_map("recovery", &self.recovery)?;
        validate_property_map("observability", &self.observability)?;
        validate_property_map("parameters", &self.parameters)?;
        validate_property_map("provider_options", &self.provider_options)?;

        Ok(())
    }

    /// Return whether every deployment settings collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
            && self.constraints.is_empty()
            && self.preferences.is_empty()
            && self.hints.is_empty()
            && self.policy_references.is_empty()
            && self.lifecycle.is_empty()
            && self.rollout.is_empty()
            && self.availability.is_empty()
            && self.portability.is_empty()
            && self.recovery.is_empty()
            && self.observability.is_empty()
            && self.parameters.is_empty()
            && self.provider_options.is_empty()
    }
}

fn validate_property_map(
    category: &str,
    properties: &BTreeMap<String, serde_json::Value>,
) -> ToolchainResult<()> {
    for key in properties.keys() {
        if key.trim().is_empty() {
            return Err(invalid_settings(format!(
                "deployment profile `{category}` contains an empty property key"
            )));
        }

        if key.trim() != key {
            return Err(invalid_settings(format!(
                "deployment profile `{category}` property key `{key}` contains surrounding whitespace"
            )));
        }

        if key.chars().any(char::is_control) {
            return Err(invalid_settings(format!(
                "deployment profile `{category}` property key contains control characters"
            )));
        }
    }

    Ok(())
}

// ============================================================================
// DEPLOYMENT PROFILE DEFINITION
// ============================================================================

/// A generic toolchain profile carrying deployment-specific settings.
///
/// The generic [`ProfileDefinition`] remains authoritative for identity,
/// name, description, enabled state, inheritance and configuration sections.
///
/// This wrapper provides typed access to the deployment settings section.
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentProfileDefinition {
    base: ProfileDefinition,
    settings: DeploymentProfileSettings,
}

impl DeploymentProfileDefinition {
    /// Create a new deployment profile.
    ///
    /// The deployment settings section is inserted immediately. Therefore,
    /// a newly created deployment profile is structurally complete and can
    /// pass validation without requiring a later initialization step.
    pub fn new(
        id: ProfileId,
        name: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let mut base = ProfileDefinition::new(id, name)?;
        let settings = DeploymentProfileSettings::new();

        let key = deployment_profile_section_key()?;
        let value = serialize_settings(&settings)?;

        base.set_section(key, value);

        let profile = Self { base, settings };
        profile.validate()?;

        Ok(profile)
    }

    /// Adapt an existing generic profile into a deployment profile.
    ///
    /// If the generic profile already contains deployment settings, they
    /// are decoded and validated. If the section is absent, default empty
    /// deployment settings are inserted into the generic profile.
    ///
    /// Invalid existing deployment settings are rejected rather than
    /// silently overwritten.
    pub fn from_definition(
        mut base: ProfileDefinition,
    ) -> ToolchainResult<Self> {
        let key = deployment_profile_section_key()?;

        let settings = match base.sections().get(&key) {
            Some(value) => deserialize_settings(value)?,

            None => {
                let settings = DeploymentProfileSettings::new();
                let value = serialize_settings(&settings)?;

                base.set_section(key, value);

                settings
            }
        };

        let profile = Self { base, settings };
        profile.validate()?;

        Ok(profile)
    }

    /// Return the underlying canonical generic profile.
    #[must_use]
    pub fn base(&self) -> &ProfileDefinition {
        &self.base
    }

    /// Return mutable access to the underlying generic profile.
    ///
    /// The caller must preserve the generic profile's invariants. Call
    /// [`Self::validate`] before registration or persistence.
    #[must_use]
    pub fn base_mut(&mut self) -> &mut ProfileDefinition {
        &mut self.base
    }

    /// Consume the wrapper and return its canonical generic profile.
    ///
    /// The deployment settings section remains embedded in the returned
    /// generic profile.
    #[must_use]
    pub fn into_base(self) -> ProfileDefinition {
        self.base
    }

    /// Return the stable profile identity.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        self.base.id()
    }

    /// Return the generic profile name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.base.name().as_str()
    }

    /// Return the deployment-specific settings.
    #[must_use]
    pub fn settings(&self) -> &DeploymentProfileSettings {
        &self.settings
    }

    /// Replace deployment settings and update the embedded generic section.
    ///
    /// The operation is transactional: invalid settings or serialization
    /// failure leaves the existing profile unchanged.
    pub fn set_settings(
        &mut self,
        settings: DeploymentProfileSettings,
    ) -> ToolchainResult<()> {
        settings.validate()?;

        let key = deployment_profile_section_key()?;
        let value = serialize_settings(&settings)?;

        self.base.set_section(key, value);
        self.settings = settings;

        Ok(())
    }

    /// Validate the generic profile and its deployment settings.
    ///
    /// This also verifies that the typed settings and serialized section
    /// have not diverged.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.base.validate()?;
        self.settings.validate()?;

        let key = deployment_profile_section_key()?;

        let embedded = self.base.sections().get(&key).ok_or_else(|| {
            invalid_settings(
                "deployment profile is missing its deployment settings section",
            )
        })?;

        let decoded = deserialize_settings(embedded)?;

        if decoded != self.settings {
            return Err(invalid_settings(
                "typed deployment settings do not match the embedded profile section",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// RESOLVED DEPLOYMENT PROFILE
// ============================================================================

/// A deployment profile after generic profile inheritance has been resolved.
///
/// The generic resolved profile remains available for other configuration
/// consumers. `settings` contains the deployment settings after inheritance
/// and overlay resolution.
///
/// A resolved deployment profile is not a deployment plan, capability
/// report, authorization decision or feasibility result.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedDeploymentProfile {
    base: ResolvedProfile,
    settings: DeploymentProfileSettings,
}

impl ResolvedDeploymentProfile {
    /// Return the canonical resolved generic profile.
    #[must_use]
    pub fn base(&self) -> &ResolvedProfile {
        &self.base
    }

    /// Return the selected profile identity.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        self.base.id()
    }

    /// Return the selected profile name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.base.name().as_str()
    }

    /// Return the resolved deployment settings.
    #[must_use]
    pub fn settings(&self) -> &DeploymentProfileSettings {
        &self.settings
    }

    /// Return the resolved inheritance lineage.
    #[must_use]
    pub fn lineage(&self) -> &[ProfileId] {
        self.base.lineage()
    }

    /// Return the underlying generic resolved profile.
    #[must_use]
    pub fn into_base(self) -> ResolvedProfile {
        self.base
    }
}

// ============================================================================
// DEPLOYMENT PROFILE REGISTRY
// ============================================================================

/// Deployment-facing adapter over the canonical generic profile registry.
///
/// This type deliberately contains no independent profile map or name index.
/// Registration, inheritance validation, cycle detection, deterministic
/// resolution and configuration persistence are delegated to
/// [`ProfileRegistry`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeploymentProfileRegistry(ProfileRegistry);

impl DeploymentProfileRegistry {
    /// Construct an empty deployment profile registry.
    #[must_use]
    pub fn new() -> Self {
        Self(ProfileRegistry::new())
    }

    /// Return the number of registered generic profiles.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return whether the registry contains no profiles.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Register a deployment profile.
    ///
    /// Generic profile identity and name uniqueness rules are enforced by
    /// the canonical registry.
    ///
    /// Returns the replaced generic profile, if one existed.
    pub fn register(
        &mut self,
        profile: DeploymentProfileDefinition,
    ) -> ToolchainResult<Option<ProfileDefinition>> {
        profile.validate()?;
        self.0.register(profile.into_base())
    }

    /// Register an existing generic profile as a deployment profile.
    ///
    /// A missing deployment settings section is initialized with defaults.
    /// Existing malformed deployment settings are rejected.
    ///
    /// Returns the replaced generic profile, if one existed.
    pub fn register_definition(
        &mut self,
        profile: ProfileDefinition,
    ) -> ToolchainResult<Option<ProfileDefinition>> {
        let deployment_profile =
            DeploymentProfileDefinition::from_definition(profile)?;

        self.register(deployment_profile)
    }

    /// Remove a profile by its canonical identity.
    ///
    /// The canonical registry rejects removal when another profile inherits
    /// from the selected profile.
    pub fn remove(
        &mut self,
        id: &ProfileId,
    ) -> ToolchainResult<Option<ProfileDefinition>> {
        self.0.remove(id)
    }

    /// Clear all registered profiles.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Validate the complete registry and all deployment settings.
    ///
    /// Generic inheritance validation is delegated to `ProfileRegistry`.
    /// Every profile is additionally checked for a valid deployment settings
    /// section.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.0.validate()?;

        for profile in self.0.profiles().values() {
            DeploymentProfileDefinition::from_definition(profile.clone())?
                .validate()?;
        }

        Ok(())
    }

    /// Resolve one deployment profile and its ancestors.
    ///
    /// Generic profile conflicts, missing parents, cycles and disabled
    /// profiles are handled by the canonical profile registry.
    pub fn resolve(
        &self,
        id: &ProfileId,
    ) -> ToolchainResult<ResolvedDeploymentProfile> {
        self.validate()?;

        let base = self.0.resolve(id)?;
        let key = deployment_profile_section_key()?;

        let value = base.sections().get(&key).ok_or_else(|| {
            invalid_settings(
                "resolved profile is missing deployment settings",
            )
        })?;

        let settings = deserialize_settings(value)?;

        Ok(ResolvedDeploymentProfile { base, settings })
    }

    /// Load deployment profiles from the canonical toolchain configuration.
    ///
    /// A configuration without a generic profile registry produces an empty
    /// deployment profile registry.
    ///
    /// The generic registry is loaded by its canonical implementation.
    pub fn load_from_config(
        config: &ToolchainConfig,
    ) -> ToolchainResult<Self> {
        let registry = ProfileRegistry::load_from_config(config)?;
        let deployment_registry = Self(registry);

        deployment_registry.validate()?;

        Ok(deployment_registry)
    }

    /// Store deployment profiles in the canonical toolchain configuration.
    ///
    /// This stores the generic registry through its canonical persistence
    /// implementation. It does not create a second deployment registry
    /// configuration section.
    pub fn store_in_config(
        &self,
        config: &mut ToolchainConfig,
    ) -> ToolchainResult<()> {
        self.validate()?;
        self.0.store_in_config(config)
    }

    /// Borrow the canonical generic registry.
    ///
    /// This is provided for integrations that need generic profile metadata
    /// or must compose deployment profiles with other configuration services.
    #[must_use]
    pub fn as_generic_registry(&self) -> &ProfileRegistry {
        &self.0
    }
}

// ============================================================================
// SERIALIZATION AND SECTION HELPERS
// ============================================================================

fn deployment_profile_section_key() -> ToolchainResult<ConfigSectionKey> {
    ConfigSectionKey::new(DEPLOYMENT_PROFILE_SECTION)
}

fn serialize_settings(
    settings: &DeploymentProfileSettings,
) -> ToolchainResult<serde_json::Value> {
    settings.validate()?;

    serde_json::to_value(settings).map_err(|source| {
        ToolchainError::invalid_configuration(
            "could not serialize deployment profile settings",
        )
        .with_source(source)
    })
}

fn deserialize_settings(
    value: &serde_json::Value,
) -> ToolchainResult<DeploymentProfileSettings> {
    let settings: DeploymentProfileSettings =
        serde_json::from_value(value.clone()).map_err(|source| {
            ToolchainError::invalid_configuration(
                "deployment profile contains malformed settings",
            )
            .with_source(source)
        })?;

    settings.validate()?;

    Ok(settings)
}

fn invalid_settings(message: impl Into<String>) -> ToolchainError {
    ToolchainError::invalid_configuration(message)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    fn profile_id(value: u128) -> ProfileId {
        ProfileId::new(
            IdNamespace::new("deployment_test")
                .expect("valid identifier namespace"),
            value,
        )
        .expect("valid profile identity")
    }

    fn deployment_profile(
        value: u128,
        name: &str,
    ) -> DeploymentProfileDefinition {
        DeploymentProfileDefinition::new(profile_id(value), name)
            .expect("valid deployment profile")
    }

    #[test]
    fn new_profile_contains_default_deployment_settings() {
        let profile = deployment_profile(1, "portable");

        assert!(profile.settings().is_empty());
        assert!(profile.validate().is_ok());

        let key = deployment_profile_section_key()
            .expect("valid section key");

        assert!(profile.base().sections().contains_key(&key));
    }

    #[test]
    fn adapts_generic_profile_without_losing_its_identity() {
        let generic = ProfileDefinition::new(
            profile_id(2),
            "generic",
        )
        .expect("valid generic profile");

        let adapted =
            DeploymentProfileDefinition::from_definition(generic)
                .expect("adaptation succeeds");

        assert_eq!(adapted.id(), &profile_id(2));
        assert_eq!(adapted.name(), "generic");
        assert!(adapted.settings().is_empty());
        assert!(adapted.validate().is_ok());
    }

    #[test]
    fn settings_update_is_embedded_in_generic_profile() {
        let mut profile = deployment_profile(3, "constrained");

        let mut settings = DeploymentProfileSettings::new();
        settings.constraints.insert(
            "environment.region".to_owned(),
            serde_json::json!("example-region"),
        );

        profile
            .set_settings(settings.clone())
            .expect("settings update succeeds");

        assert_eq!(profile.settings(), &settings);
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn rejects_unknown_settings_schema_version() {
        let mut settings = DeploymentProfileSettings::new();
        settings.schema_version =
            DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION + 1;

        assert!(settings.validate().is_err());
    }

    #[test]
    fn rejects_empty_property_keys() {
        let mut settings = DeploymentProfileSettings::new();
        settings.requirements.insert(
            String::new(),
            serde_json::Value::Bool(true),
        );

        assert!(settings.validate().is_err());
    }

    #[test]
    fn registry_uses_generic_profile_registration_and_resolution() {
        let mut registry = DeploymentProfileRegistry::new();

        let mut profile = deployment_profile(4, "portable");
        let mut settings = DeploymentProfileSettings::new();

        settings.preferences.insert(
            "deployment.portability".to_owned(),
            serde_json::json!("prefer-portable"),
        );

        profile
            .set_settings(settings.clone())
            .expect("settings update succeeds");

        registry
            .register(profile)
            .expect("registration succeeds");

        let resolved = registry
            .resolve(&profile_id(4))
            .expect("resolution succeeds");

        assert_eq!(resolved.settings(), &settings);
        assert_eq!(resolved.id(), &profile_id(4));
        assert_eq!(resolved.lineage(), &[profile_id(4)]);
    }

    #[test]
    fn supports_generic_profile_inheritance() {
        let mut registry = DeploymentProfileRegistry::new();

        let parent = deployment_profile(5, "base");
        registry
            .register(parent)
            .expect("parent registration succeeds");

        let mut child = deployment_profile(6, "child");
        child.base_mut().add_parent(profile_id(5));

        registry
            .register(child)
            .expect("child registration succeeds");

        let resolved = registry
            .resolve(&profile_id(6))
            .expect("inherited profile resolves");

        assert_eq!(
            resolved.lineage(),
            &[profile_id(5), profile_id(6)]
        );
    }

    #[test]
    fn stores_and_loads_through_canonical_toolchain_config() {
        let mut config = ToolchainConfig::new();
        let mut registry = DeploymentProfileRegistry::new();

        registry
            .register(deployment_profile(7, "stored"))
            .expect("registration succeeds");

        registry
            .store_in_config(&mut config)
            .expect("storage succeeds");

        let loaded = DeploymentProfileRegistry::load_from_config(&config)
            .expect("loading succeeds");

        assert_eq!(loaded, registry);
    }

    #[test]
    fn rejects_malformed_embedded_deployment_settings() {
        let mut generic = ProfileDefinition::new(
            profile_id(8),
            "malformed",
        )
        .expect("valid generic profile");

        let key = deployment_profile_section_key()
            .expect("valid section key");

        generic.set_section(
            key,
            serde_json::json!({
                "schema_version": "not-a-number"
            }),
        );

        assert!(
            DeploymentProfileDefinition::from_definition(generic).is_err()
        );
    }

    #[test]
    fn rejects_registry_profiles_with_divergent_typed_settings() {
        let mut profile = deployment_profile(9, "divergent");

        profile.settings.requirements.insert(
            "runtime.requirement".to_owned(),
            serde_json::json!(true),
        );

        assert!(profile.validate().is_err());
    }

    #[test]
    fn does_not_claim_capabilities_or_feasibility() {
        let settings = DeploymentProfileSettings::new();

        assert!(settings.is_empty());
        assert!(settings.validate().is_ok());

        // Capability discovery and feasibility are intentionally not
        // inferred from the absence or presence of profile settings.
    }
}