
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Profile Resolution
//!
//! Path: `src/toolchain/build/profiles.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Resolves the effective configuration for one logical build operation.
//!
//! The resolution order is:
//!
//! 1. Existing toolchain configuration.
//! 2. Project manifest configuration.
//! 3. Selected, inherited configuration profile.
//! 4. Explicit build-operation overrides.
//!
//! Configuration sections are replaced atomically. This module does not
//! invent recursive merge semantics for configuration owned by other
//! subsystems.
//!
//! ## Canonical integrations
//!
//! - `toolchain::build::request::BuildRequest`
//! - `toolchain::configuration::config::ToolchainConfig`
//! - `toolchain::configuration::config::ConfigSectionKey`
//! - `toolchain::configuration::profiles::ProfileRegistry`
//! - `toolchain::configuration::profiles::ResolvedProfile`
//! - `toolchain::api::OperationalBudgets`
//! - `toolchain::ids::ProfileId`
//! - `toolchain::errors::ToolchainError`
//! - `toolchain::result::ToolchainResult`
//!
//! ## Ownership boundaries
//!
//! This module owns build-operation configuration composition.
//!
//! It does not own:
//!
//! - Source-language compilation-profile parsing.
//! - AST or semantic profile interpretation.
//! - Requirements, Capabilities, Preferences, Constraints or Hints.
//! - Target discovery or target selection.
//! - Capability matching.
//! - Feasibility determination.
//! - Compiler optimization implementation.
//! - Compiler pass registration or execution.
//! - Physical resource allocation.
//! - Backend selection or code generation.
//! - Runtime scheduling or execution.
//!
//! ## Grammar boundary
//!
//! `grammar/compile/profiles.g4` defines source-language profile syntax.
//! Its semantic interpretation remains with the frontend and compiler.
//!
//! This module resolves toolchain configuration profiles only.
//!
//! ## POCO-REAF
//!
//! Profiles are open-world configuration collections.
//!
//! No universal maximum is imposed on:
//!
//! - Profiles.
//! - Profile sections.
//! - Configuration properties.
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - CPUs, GPUs, FPGAs, QPUs or nodes.
//! - Qubits.
//! - Memory or storage.
//! - Tensor dimensions.
//! - Distributed participants.
//!
//! Actual operation is subject to available resources, Rust's
//! representational constraints and explicitly configured budgets.
//!
//! Literal infinite execution or storage is not promised.
//!
//! ## Determinism
//!
//! All configuration sections and their origins are maintained in
//! deterministic key order.
//!
//! Profile inheritance is resolved by the canonical ProfileRegistry.
//!
//! No timestamps, randomness, environment inspection, filesystem access,
//! network access, device discovery or implicit host-target selection
//! occur in this module.
//!
//! ## Security
//!
//! This module does not authenticate callers or authorize operations.
//!
//! Configuration profiles are not security policies.
//!
//! Credential values must not be introduced through build-profile
//! configuration. Credential management belongs to the designated
//! authentication providers.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::toolchain::api::OperationalBudgets;

use crate::toolchain::build::request::BuildRequest;

use crate::toolchain::configuration::config::{
    ConfigSectionKey,
    ToolchainConfig,
};

use crate::toolchain::configuration::profiles::{
    ProfileName,
    ProfileRegistry,
    ResolvedProfile,
};

use crate::toolchain::errors::ToolchainError;

use crate::toolchain::ids::ProfileId;

use crate::toolchain::result::ToolchainResult;

// ============================================================================
// CONFIGURATION ORIGINS
// ============================================================================

/// Identifies the configuration layer that supplied an effective setting.
///
/// The ordering is part of the build-profile contract.
///
/// Later layers have higher precedence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum BuildProfileSource {
    /// Original toolchain configuration.
    ToolchainConfiguration,

    /// Project manifest configuration.
    ///
    /// The manifest adapter supplies these values. This module does not
    /// implement another TOML parser.
    ProjectManifest,

    /// Resolved configuration profile, including inherited sections.
    ConfigurationProfile,

    /// Explicit build-operation override.
    BuildOverride,
}

// ============================================================================
// MANIFEST CONFIGURATION ADAPTER
// ============================================================================

/// Manifest-provided build configuration.
///
/// This is an adapter value, not another Zamani.toml parser or manifest
/// schema.
///
/// The canonical manifest subsystem is responsible for translating
/// validated manifest settings into these namespaced sections.
///
/// The map is deliberately open-ended so new computing domains and
/// provider-specific configuration can be integrated without changing
/// this module.
#[derive(Clone, Debug, Default, PartialEq)]
#[must_use]
pub struct ManifestBuildSettings {
    sections: BTreeMap<ConfigSectionKey, Value>,
}

impl ManifestBuildSettings {
    /// Construct empty manifest build settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a manifest configuration section.
    ///
    /// Returns the previous value, if one existed.
    pub fn set_section(
        &mut self,
        key: ConfigSectionKey,
        value: Value,
    ) -> Option<Value> {
        self.sections.insert(key, value)
    }

    /// Remove one manifest configuration section.
    pub fn remove_section(
        &mut self,
        key: &ConfigSectionKey,
    ) -> Option<Value> {
        self.sections.remove(key)
    }

    /// Retrieve a manifest configuration section.
    #[must_use]
    pub fn section(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&Value> {
        self.sections.get(key)
    }

    /// Borrow all manifest configuration sections.
    #[must_use]
    pub fn sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, Value> {
        &self.sections
    }

    /// Return whether no manifest settings were supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Return the number of manifest sections.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sections.len()
    }
}

// ============================================================================
// EXPLICIT BUILD OVERRIDES
// ============================================================================

/// Configuration explicitly supplied for one build operation.
///
/// These settings have the highest configuration-section precedence.
///
/// They do not modify the original toolchain configuration, manifest,
/// or registered profile.
///
/// The value of each section is interpreted by its owning subsystem.
#[derive(Clone, Debug, Default, PartialEq)]
#[must_use]
pub struct BuildProfileOverrides {
    sections: BTreeMap<ConfigSectionKey, Value>,
}

impl BuildProfileOverrides {
    /// Construct an empty override collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace an explicit build override.
    ///
    /// Returns the previous value, if one existed.
    pub fn set(
        &mut self,
        key: ConfigSectionKey,
        value: Value,
    ) -> Option<Value> {
        self.sections.insert(key, value)
    }

    /// Remove an explicit override.
    pub fn remove(
        &mut self,
        key: &ConfigSectionKey,
    ) -> Option<Value> {
        self.sections.remove(key)
    }

    /// Retrieve an explicit override.
    #[must_use]
    pub fn get(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&Value> {
        self.sections.get(key)
    }

    /// Borrow all explicit overrides.
    #[must_use]
    pub fn sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, Value> {
        &self.sections
    }

    /// Return whether no overrides were supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Return the number of explicit overrides.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sections.len()
    }
}

// ============================================================================
// BUILD PROFILE INPUTS
// ============================================================================

/// All configuration inputs required to resolve one build profile.
///
/// References are borrowed so resolving a profile does not consume or
/// mutate the caller's canonical configuration or profile registry.
#[derive(Debug)]
pub struct BuildProfileInputs<'a> {
    /// Existing toolchain configuration.
    pub configuration: &'a ToolchainConfig,

    /// Canonical registered configuration profiles.
    pub registry: &'a ProfileRegistry,

    /// Validated manifest build settings.
    pub manifest: &'a ManifestBuildSettings,

    /// Explicit settings supplied for this build operation.
    pub overrides: &'a BuildProfileOverrides,
}

impl<'a> BuildProfileInputs<'a> {
    /// Construct a complete build-profile resolution context.
    #[must_use]
    pub fn new(
        configuration: &'a ToolchainConfig,
        registry: &'a ProfileRegistry,
        manifest: &'a ManifestBuildSettings,
        overrides: &'a BuildProfileOverrides,
    ) -> Self {
        Self {
            configuration,
            registry,
            manifest,
            overrides,
        }
    }
}

// ============================================================================
// PROFILE SELECTION
// ============================================================================

/// Records where the selected profile identity originated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProfileSelectionSource {
    /// The build request explicitly selected the profile.
    BuildRequest,

    /// The profile was inherited from toolchain configuration.
    ToolchainConfiguration,
}

/// Result of determining which profile should be resolved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedBuildProfile {
    id: ProfileId,
    source: ProfileSelectionSource,
}

impl SelectedBuildProfile {
    /// Determine the selected profile.
    ///
    /// An explicit build-request selection takes precedence over the
    /// profile selected in the existing toolchain configuration.
    ///
    /// No default profile is invented.
    #[must_use]
    pub fn resolve(
        request: &BuildRequest,
        configuration: &ToolchainConfig,
    ) -> Option<Self> {
        if let Some(profile) = request.profile() {
            return Some(Self {
                id: profile.clone(),
                source: ProfileSelectionSource::BuildRequest,
            });
        }

        configuration.profile().cloned().map(|id| Self {
            id,
            source: ProfileSelectionSource::ToolchainConfiguration,
        })
    }

    /// Borrow the selected profile identity.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        &self.id
    }

    /// Return the source of the selection.
    #[must_use]
    pub const fn source(&self) -> ProfileSelectionSource {
        self.source
    }
}

// ============================================================================
// RESOLVED BUILD PROFILE
// ============================================================================

/// Immutable effective configuration for one logical build operation.
///
/// This is a resolved configuration snapshot.
///
/// It does not mean that:
///
/// - The selected targets exist.
/// - The requested capabilities are available.
/// - Resources have been allocated.
/// - The compiler has accepted the configuration.
/// - Compilation has succeeded.
/// - Artifacts have been produced.
/// - The resulting program can execute.
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct ResolvedBuildProfile {
    /// Effective aggregate toolchain configuration.
    configuration: ToolchainConfig,

    /// Selected profile identity, if a profile was selected.
    selected_profile: Option<ProfileId>,

    /// Selected profile's human-readable name.
    profile_name: Option<ProfileName>,

    /// Profile inheritance lineage, ancestors first.
    profile_lineage: Vec<ProfileId>,

    /// Effective source of each configuration section.
    origins: BTreeMap<ConfigSectionKey, BuildProfileSource>,

    /// Requested features preserved from the validated build request.
    features: std::collections::BTreeSet<String>,

    /// Explicit operational budgets preserved from the build request.
    budgets: OperationalBudgets,
}

impl ResolvedBuildProfile {
    /// Resolve an effective profile from all configuration layers.
    ///
    /// Resolution is performed in the following order:
    ///
    /// 1. Existing toolchain configuration.
    /// 2. Project manifest settings.
    /// 3. Selected configuration profile.
    /// 4. Explicit build-operation overrides.
    ///
    /// The original inputs remain unchanged.
    ///
    /// If a profile is selected but does not exist, is disabled, has
    /// invalid inheritance, or cannot be resolved, resolution fails.
    ///
    /// Unknown configuration sections are preserved. Their meaning
    /// must be validated by the owning subsystem before use.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The build request is structurally invalid.
    /// - The base configuration is invalid.
    /// - The selected profile cannot be resolved.
    /// - A resulting configuration is structurally invalid.
    pub fn resolve(
        request: &BuildRequest,
        inputs: BuildProfileInputs<'_>,
    ) -> ToolchainResult<Self> {
        request.validate().map_err(|error| {
            ToolchainError::validation_failed(
                format!("invalid build request: {error}"),
            )
            .with_operation("build.profiles.resolve")
        })?;

        inputs.configuration.validate()?;

        let selected = SelectedBuildProfile::resolve(
            request,
            inputs.configuration,
        );

        let resolved_profile = match &selected {
            Some(selection) => Some(
                inputs.registry.resolve(selection.id())?,
            ),
            None => None,
        };

        Self::compose(
            request,
            inputs,
            selected,
            resolved_profile,
        )
    }

    /// Compose already selected and resolved profile layers.
    ///
    /// Keeping this operation separate makes the precedence rules
    /// explicit and independently testable.
    fn compose(
        request: &BuildRequest,
        inputs: BuildProfileInputs<'_>,
        selected: Option<SelectedBuildProfile>,
        resolved_profile: Option<ResolvedProfile>,
    ) -> ToolchainResult<Self> {
        let mut configuration = inputs.configuration.clone();

        let mut origins =
            BTreeMap::<ConfigSectionKey, BuildProfileSource>::new();

        // Layer 1: existing toolchain configuration.
        for key in configuration.sections().keys() {
            origins.insert(
                key.clone(),
                BuildProfileSource::ToolchainConfiguration,
            );
        }

        // Layer 2: validated project manifest settings.
        for (key, value) in inputs.manifest.sections() {
            configuration.set_section_value(
                key.clone(),
                value.clone(),
            );

            origins.insert(
                key.clone(),
                BuildProfileSource::ProjectManifest,
            );
        }

        // Layer 3: resolved profile and inherited settings.
        //
        // The canonical ProfileRegistry is the only authority for
        // profile inheritance, cycle detection and conflict detection.
        let (selected_profile, profile_name, profile_lineage) =
            if let Some(profile) = resolved_profile {
                profile.apply_to(&mut configuration)?;

                for key in profile.sections().keys() {
                    origins.insert(
                        key.clone(),
                        BuildProfileSource::ConfigurationProfile,
                    );
                }

                (
                    Some(profile.id().clone()),
                    Some(profile.name().clone()),
                    profile.lineage().to_vec(),
                )
            } else {
                (None, None, Vec::new())
            };

        // Layer 4: explicit operation-level overrides.
        for (key, value) in inputs.overrides.sections() {
            configuration.set_section_value(
                key.clone(),
                value.clone(),
            );

            origins.insert(
                key.clone(),
                BuildProfileSource::BuildOverride,
            );
        }

        // Validate the final aggregate after every layer has been applied.
        configuration.validate()?;

        // The final profile identity must agree with the selected profile.
        if let Some(selection) = &selected {
            if configuration.profile() != Some(selection.id()) {
                return Err(
                    ToolchainError::validation_failed(
                        "effective configuration profile identity does not match the selected profile",
                    )
                    .with_operation("build.profiles.resolve"),
                );
            }
        }

        // Preserve an explicitly configured profile when no new
        // selection was required.
        if selected.is_none() && configuration.profile().is_some() {
            return Err(
                ToolchainError::validation_failed(
                    "configuration contains a profile identity without a resolved profile",
                )
                .with_operation("build.profiles.resolve"),
            );
        }

        Ok(Self {
            configuration,
            selected_profile,
            profile_name,
            profile_lineage,
            origins,
            features: request.features().clone(),
            budgets: request.budgets().clone(),
        })
    }

    /// Borrow the effective toolchain configuration.
    #[must_use]
    pub fn configuration(&self) -> &ToolchainConfig {
        &self.configuration
    }

    /// Consume this resolved profile and return the effective configuration.
    ///
    /// The returned configuration has already passed structural validation.
    #[must_use]
    pub fn into_configuration(self) -> ToolchainConfig {
        self.configuration
    }

    /// Borrow the selected profile identity.
    #[must_use]
    pub fn selected_profile(&self) -> Option<&ProfileId> {
        self.selected_profile.as_ref()
    }

    /// Borrow the selected profile name.
    #[must_use]
    pub fn profile_name(&self) -> Option<&ProfileName> {
        self.profile_name.as_ref()
    }

    /// Borrow the profile inheritance lineage.
    #[must_use]
    pub fn profile_lineage(&self) -> &[ProfileId] {
        &self.profile_lineage
    }

    /// Retrieve the origin of one effective configuration section.
    #[must_use]
    pub fn origin(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<BuildProfileSource> {
        self.origins.get(key).copied()
    }

    /// Borrow the complete deterministic configuration-origin map.
    #[must_use]
    pub fn origins(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, BuildProfileSource> {
        &self.origins
    }

    /// Borrow the requested feature selection.
    ///
    /// Feature availability and dependency feature unification remain
    /// the responsibility of dependency and manifest resolution.
    #[must_use]
    pub fn features(&self) -> &std::collections::BTreeSet<String> {
        &self.features
    }

    /// Borrow the operational budgets requested by the build operation.
    ///
    /// Budgets are preserved, not interpreted or enforced here.
    #[must_use]
    pub fn budgets(&self) -> &OperationalBudgets {
        &self.budgets
    }

    /// Retrieve one effective configuration section.
    #[must_use]
    pub fn section(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&Value> {
        self.configuration.section(key)
    }

    /// Return the number of effective configuration sections.
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.configuration.section_count()
    }

    /// Return whether the effective configuration has no sections.
    #[must_use]
    pub fn sections_are_empty(&self) -> bool {
        self.configuration.sections_are_empty()
    }

    /// Validate the effective configuration again at an integration boundary.
    ///
    /// This does not validate domain-specific configuration semantics.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.configuration.validate()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::{
        ProjectLocator,
        ProjectScope,
        TargetSelection,
    };

    use crate::toolchain::ids::IdNamespace;

    fn profile_id(value: u128) -> ProfileId {
        ProfileId::new(
            IdNamespace::new("test.toolchain.profile")
                .expect("valid namespace"),
            value,
        )
        .expect("valid profile ID")
    }

    fn section(value: &str) -> ConfigSectionKey {
        ConfigSectionKey::new(value)
            .expect("valid configuration section")
    }

    fn request(profile: Option<ProfileId>) -> BuildRequest {
        let scope = ProjectScope::new(
            ProjectLocator::from_path("examples/minimal")
                .expect("valid project path"),
        )
        .expect("valid project scope");

        let targets = TargetSelection::resolve_from_capabilities(
            Vec::new(),
        )
        .expect("valid target intent");

        let request = BuildRequest::new(scope, targets)
            .expect("valid build request");

        match profile {
            Some(id) => request.with_profile(id),
            None => request,
        }
    }

    fn resolve<'a>(
        request: &BuildRequest,
        config: &'a ToolchainConfig,
        registry: &'a ProfileRegistry,
        manifest: &'a ManifestBuildSettings,
        overrides: &'a BuildProfileOverrides,
    ) -> ToolchainResult<ResolvedBuildProfile> {
        ResolvedBuildProfile::resolve(
            request,
            BuildProfileInputs::new(
                config,
                registry,
                manifest,
                overrides,
            ),
        )
    }

    fn setup() -> (
        ToolchainConfig,
        ProfileRegistry,
        ManifestBuildSettings,
        BuildProfileOverrides,
    ) {
        (
            ToolchainConfig::new(),
            ProfileRegistry::new(),
            ManifestBuildSettings::new(),
            BuildProfileOverrides::new(),
        )
    }

    #[test]
    fn resolves_without_an_implicit_profile() {
        let (config, registry, manifest, overrides) = setup();

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("empty configuration is valid");

        assert!(result.selected_profile().is_none());
        assert!(result.profile_lineage().is_empty());
        assert!(result.sections_are_empty());
    }

    #[test]
    fn request_profile_overrides_configuration_profile() {
        let mut config = ToolchainConfig::new();
        let request_id = profile_id(1);
        let config_id = profile_id(2);

        config.set_profile(config_id.clone());

        let mut registry = ProfileRegistry::new();

        registry.register(
            crate::toolchain::configuration::profiles::ProfileDefinition::new(
                request_id.clone(),
                "request_profile",
            )
            .expect("valid profile"),
        )
        .expect("register profile");

        registry.register(
            crate::toolchain::configuration::profiles::ProfileDefinition::new(
                config_id,
                "configuration_profile",
            )
            .expect("valid profile"),
        )
        .expect("register profile");

        let manifest = ManifestBuildSettings::new();
        let overrides = BuildProfileOverrides::new();

        let result = resolve(
            &request(Some(request_id.clone())),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("request-selected profile resolves");

        assert_eq!(
            result.selected_profile(),
            Some(&request_id),
        );
    }

    #[test]
    fn resolves_configuration_profile_when_request_has_none() {
        let mut config = ToolchainConfig::new();
        let id = profile_id(3);

        config.set_profile(id.clone());

        let mut registry = ProfileRegistry::new();

        registry.register(
            crate::toolchain::configuration::profiles::ProfileDefinition::new(
                id.clone(),
                "configured",
            )
            .expect("valid profile"),
        )
        .expect("register profile");

        let manifest = ManifestBuildSettings::new();
        let overrides = BuildProfileOverrides::new();

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configured profile resolves");

        assert_eq!(result.selected_profile(), Some(&id));
    }

    #[test]
    fn rejects_an_unregistered_selected_profile() {
        let (config, registry, manifest, overrides) = setup();

        let result = resolve(
            &request(Some(profile_id(99))),
            &config,
            &registry,
            &manifest,
            &overrides,
        );

        assert!(result.is_err());
    }

    #[test]
    fn manifest_overrides_base_configuration() {
        let (mut config, registry, mut manifest, overrides) = setup();

        let key = section("compiler.optimization");

        config.set_section_value(
            key.clone(),
            Value::String("base".to_owned()),
        );

        manifest.set_section(
            key.clone(),
            Value::String("manifest".to_owned()),
        );

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(
            result.section(&key),
            Some(&Value::String("manifest".to_owned())),
        );

        assert_eq!(
            result.origin(&key),
            Some(BuildProfileSource::ProjectManifest),
        );
    }

    #[test]
    fn explicit_override_has_highest_precedence() {
        let (mut config, registry, mut manifest, mut overrides) = setup();

        let key = section("compiler.optimization");

        config.set_section_value(
            key.clone(),
            Value::String("base".to_owned()),
        );

        manifest.set_section(
            key.clone(),
            Value::String("manifest".to_owned()),
        );

        overrides.set(
            key.clone(),
            Value::String("operation".to_owned()),
        );

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(
            result.section(&key),
            Some(&Value::String("operation".to_owned())),
        );

        assert_eq!(
            result.origin(&key),
            Some(BuildProfileSource::BuildOverride),
        );
    }

    #[test]
    fn profile_overrides_manifest_and_base_configuration() {
        let (mut config, mut registry, mut manifest, overrides) = setup();

        let id = profile_id(4);
        let key = section("compiler.optimization");

        config.set_section_value(
            key.clone(),
            Value::String("base".to_owned()),
        );

        manifest.set_section(
            key.clone(),
            Value::String("manifest".to_owned()),
        );

        let mut definition =
            crate::toolchain::configuration::profiles::ProfileDefinition::new(
                id,
                "release",
            )
            .expect("valid profile");

        definition.set_section(
            key.clone(),
            Value::String("profile".to_owned()),
        );

        registry.register(definition)
            .expect("register profile");

        let result = resolve(
            &request(Some(profile_id(4))),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(
            result.section(&key),
            Some(&Value::String("profile".to_owned())),
        );

        assert_eq!(
            result.origin(&key),
            Some(BuildProfileSource::ConfigurationProfile),
        );
    }

    #[test]
    fn unrelated_sections_are_preserved() {
        let (mut config, registry, mut manifest, mut overrides) = setup();

        let first = section("compiler.optimization");
        let second = section("quantum.provider");
        let third = section("distributed.execution");

        config.set_section_value(
            first.clone(),
            Value::String("base".to_owned()),
        );

        manifest.set_section(
            second.clone(),
            Value::String("manifest".to_owned()),
        );

        overrides.set(
            third.clone(),
            Value::String("operation".to_owned()),
        );

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert!(result.section(&first).is_some());
        assert!(result.section(&second).is_some());
        assert!(result.section(&third).is_some());
    }

    #[test]
    fn records_configuration_origins() {
        let (mut config, registry, mut manifest, mut overrides) = setup();

        let base = section("compiler");
        let project = section("project.build");
        let operation = section("backend.options");

        config.set_section_value(base.clone(), Value::Null);

        manifest.set_section(project.clone(), Value::Null);

        overrides.set(operation.clone(), Value::Null);

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(
            result.origin(&base),
            Some(BuildProfileSource::ToolchainConfiguration),
        );

        assert_eq!(
            result.origin(&project),
            Some(BuildProfileSource::ProjectManifest),
        );

        assert_eq!(
            result.origin(&operation),
            Some(BuildProfileSource::BuildOverride),
        );
    }

    #[test]
    fn does_not_mutate_original_configuration() {
        let (mut config, registry, manifest, overrides) = setup();

        let key = section("compiler.optimization");

        config.set_section_value(
            key.clone(),
            Value::String("original".to_owned()),
        );

        let original = config.clone();

        let _result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(config, original);
    }

    #[test]
    fn preserves_build_request_features() {
        let (config, registry, manifest, overrides) = setup();

        let features = std::collections::BTreeSet::from([
            "classical.parallel".to_owned(),
            "quantum.simulation".to_owned(),
        ]);

        let build_request = request(None)
            .with_features(features.clone())
            .expect("valid features");

        let result = resolve(
            &build_request,
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(result.features(), &features);
    }

    #[test]
    fn preserves_explicit_zero_budgets() {
        let (config, registry, manifest, overrides) = setup();

        let key = crate::toolchain::api::BudgetKey::new(
            "build.wall_time_ms",
        )
        .expect("valid budget key");

        let mut budgets = OperationalBudgets::new();
        budgets.set(key.clone(), 0);

        let build_request = request(None)
            .with_budgets(budgets)
            .expect("valid budgets");

        let result = resolve(
            &build_request,
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("configuration resolves");

        assert_eq!(result.budgets().get(&key), Some(0));
    }

    #[test]
    fn does_not_interpret_unknown_domain_sections() {
        let (config, registry, mut manifest, overrides) = setup();

        let key = section("future_computing_domain.configuration");

        manifest.set_section(
            key.clone(),
            serde_json::json!({
                "provider": "future.provider",
                "options": {
                    "mode": "portable"
                }
            }),
        );

        let result = resolve(
            &request(None),
            &config,
            &registry,
            &manifest,
            &overrides,
        )
        .expect("unknown sections are preserved");

        assert!(result.section(&key).is_some());
    }
}
