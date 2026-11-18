
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Profile Integration
//!
//! Path: src/toolchain/targets/profiles.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the typed toolchain-facing representation of target-specific
//! settings attached to the existing generic toolchain profile system.
//!
//! This module does not create another profile registry, target registry,
//! target descriptor, capability model, or target-selection engine.
//!
//! # Architectural authorities
//!
//! Generic profile identity and inheritance:
//!     crate::toolchain::ids::ProfileId
//!     crate::toolchain::configuration::profiles
//!
//! Target identity and metadata:
//!     crate::compiler::configuration::target
//!
//! Backend target descriptions:
//!     crate::backend::target
//!
//! Backend capabilities and evidence:
//!     crate::backend::capabilities
//!
//! Workload requirements:
//!     crate::backend::requirements
//!
//! Target discovery and feasibility:
//!     crate::compiler::planning
//!
//! Toolchain configuration:
//!     crate::toolchain::configuration::config
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Source-level compilation profiles:
//!     grammar/compile/profiles.g4
//!
//! Source-level target intent:
//!     grammar/compile/target.g4
//!
//! Target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! # Ownership
//!
//! This module owns:
//!
//! - TargetProfileSettings.
//! - The target-profile configuration section contract.
//! - Target-reference validation through canonical compiler TargetId.
//! - Namespaced provider-specific configuration sections.
//! - Serialization and deserialization validation.
//! - Adapters to existing ProfileDefinition and ResolvedProfile.
//! - Explicit target-scope inspection.
//! - Target-profile schema versioning.
//!
//! # Non-ownership
//!
//! This module does not own:
//!
//! - Generic profile identities.
//! - Profile names.
//! - Profile registration.
//! - Profile inheritance.
//! - Profile cycle detection.
//! - Source-language profile declarations.
//! - Source-language target declarations.
//! - Target discovery.
//! - Target registration.
//! - Target descriptors.
//! - Hardware discovery.
//! - Capability declarations.
//! - Capability evidence.
//! - Requirement evaluation.
//! - Constraint solving.
//! - Preference scoring.
//! - Feasibility assessment.
//! - Resource accounting.
//! - Physical allocation.
//! - Device selection.
//! - Backend selection.
//! - Runtime execution.
//!
//! # Fundamental distinctions
//!
//! A target reference is not a target descriptor.
//!
//! A target descriptor is not a capability declaration.
//!
//! A capability declaration is not evidence of current availability.
//!
//! A valid target profile is not proof of feasibility.
//!
//! An unspecified target scope is not proof that every target is supported.
//!
//! # POCO-REAF
//!
//! Target identities and provider configuration sections are open-world.
//!
//! No universal hardware or computational ceilings are introduced.
//!
//! There are no fixed limits on targets, providers, devices, CPUs, GPUs,
//! FPGAs, QPUs, nodes, qubits, memory, storage, tensor dimensions or
//! computational domains.
//!
//! Collection sizes are constrained only by actual implementation resources,
//! representational limits and explicitly configured operational budgets.
//!
//! Literal infinite physical resources or execution are not promised.
//!
//! # Determinism
//!
//! BTreeSet and BTreeMap preserve deterministic ordering.
//!
//! Profile settings do not depend on registration order, wall-clock time,
//! randomness, host inspection or implicit target discovery.
//!
//! # Security
//!
//! This module performs no filesystem, network, environment, process,
//! credential, hardware or runtime operations.
//!
//! Provider configuration is declarative data. It must be validated by
//! the provider that owns its interpretation before it is trusted.
//!
//! Configuration profiles do not grant authorization.
//!
//! # Integration contract
//!
//! 1. Generic profile definitions remain in configuration::profiles.
//! 2. Target identities remain in compiler::configuration::target.
//! 3. Target-profile settings are stored in a dedicated namespaced
//!    ConfigSectionKey.
//! 4. Existing profile inheritance resolves the settings.
//! 5. A child profile replaces the target-profile section atomically.
//! 6. Provider-specific sections remain owned by their providers.
//! 7. Target discovery and capability matching happen downstream.
//! 8. Unknown target information remains unknown.
//! 9. This module does not select or allocate hardware.
//! 10. The targets composition module exports this implementation.

use std::collections::{BTreeMap, BTreeSet};

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::compiler::configuration::target::TargetId;

use crate::toolchain::configuration::config::ConfigSectionKey;

use crate::toolchain::configuration::profiles::{
    ProfileDefinition,
    ResolvedProfile,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

// ============================================================================
// MODULE IDENTITY AND VERSIONING
// ============================================================================

/// Stable architectural identity of this module.
pub const TARGET_PROFILES_MODULE_NAME: &str =
    "zamani.toolchain.targets.profiles";

/// Public API version of target-profile integration.
pub const TARGET_PROFILES_API_VERSION: u16 = 1;

/// Current serialized target-profile settings schema.
pub const TARGET_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Canonical configuration section containing target-profile settings.
///
/// This section is interpreted only by this module.
///
/// It is not a target registry or a compiler target descriptor.
pub const TARGET_PROFILE_SECTION_NAME: &str =
    "toolchain.targets.profile";

// ============================================================================
// CONFIGURATION SECTION
// ============================================================================

/// Construct the canonical target-profile configuration-section key.
///
/// The key is validated by the existing ToolchainConfig section authority.
pub fn target_profile_section_key() -> ToolchainResult<ConfigSectionKey> {
    ConfigSectionKey::new(TARGET_PROFILE_SECTION_NAME)
}

// ============================================================================
// TARGET PROFILE SETTINGS
// ============================================================================

/// Typed target-specific settings attached to an existing toolchain profile.
///
/// This structure deliberately has no independent profile identity or name.
///
/// Its identity, name, registration, enabled state and inheritance belong
/// to the existing ProfileDefinition and ProfileRegistry.
///
/// # Target references
///
/// `target_references` contains symbolic references to compiler-owned
/// TargetId values.
///
/// An empty set means that this profile makes no explicit target-reference
/// declaration.
///
/// It does not mean that all targets are available or compatible.
///
/// A non-empty set records the target identities explicitly referenced
/// by the configuration. It does not establish that those targets exist.
///
/// # Provider sections
///
/// Provider-specific settings are stored using canonical configuration
/// section keys.
///
/// Their contents are opaque JSON values here. Their owning subsystems
/// must validate their meaning.
///
/// # Inheritance
///
/// This structure does not implement inheritance.
///
/// Existing ProfileRegistry resolution composes the enclosing generic
/// profile definitions.
///
/// The target-profile section is an atomic configuration section. A child
/// profile that supplies this section replaces the inherited section rather
/// than recursively merging provider-owned settings.
///
/// Provider-specific recursive merging must be implemented by the owner
/// of the corresponding configuration section.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetProfileSettings {
    /// Version of the serialized target-profile settings schema.
    schema_version: u32,

    /// Symbolic references to canonical compiler target identities.
    ///
    /// The strings are validated using compiler::configuration::target::TargetId.
    #[serde(default)]
    target_references: BTreeSet<String>,

    /// Namespaced provider-specific configuration sections.
    ///
    /// Interpretation belongs to the corresponding provider.
    #[serde(default)]
    provider_sections: BTreeMap<ConfigSectionKey, Value>,
}

/// Strict wire representation used for deserialization.
///
/// Deserialized data must pass the same validation as programmatically
/// constructed settings.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetProfileSettingsWire {
    schema_version: u32,

    #[serde(default)]
    target_references: BTreeSet<String>,

    #[serde(default)]
    provider_sections: BTreeMap<ConfigSectionKey, Value>,
}

impl<'de> Deserialize<'de> for TargetProfileSettings {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = TargetProfileSettingsWire::deserialize(deserializer)?;

        let settings = Self {
            schema_version: wire.schema_version,
            target_references: wire.target_references,
            provider_sections: wire.provider_sections,
        };

        settings.validate().map_err(de::Error::custom)?;

        Ok(settings)
    }
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl Default for TargetProfileSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetProfileSettings {
    /// Construct empty target-profile settings.
    ///
    /// No target is selected implicitly.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_version: TARGET_PROFILE_SCHEMA_VERSION,
            target_references: BTreeSet::new(),
            provider_sections: BTreeMap::new(),
        }
    }

    /// Return the serialized settings schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the target references in deterministic order.
    #[must_use]
    pub fn target_references(&self) -> &BTreeSet<String> {
        &self.target_references
    }

    /// Return provider-specific configuration sections.
    ///
    /// The returned map is ordered by canonical section identity.
    #[must_use]
    pub fn provider_sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, Value> {
        &self.provider_sections
    }

    /// Return whether the settings contain explicit target references.
    ///
    /// An empty result means unspecified target scope, not universal
    /// target support.
    #[must_use]
    pub fn has_explicit_target_references(&self) -> bool {
        !self.target_references.is_empty()
    }

    /// Return the number of explicit target references.
    #[must_use]
    pub fn target_reference_count(&self) -> usize {
        self.target_references.len()
    }

    /// Return the number of provider-specific sections.
    #[must_use]
    pub fn provider_section_count(&self) -> usize {
        self.provider_sections.len()
    }

    /// Return whether no target-specific settings have been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.target_references.is_empty()
            && self.provider_sections.is_empty()
    }

    // ========================================================================
    // TARGET REFERENCES
    // ========================================================================

    /// Add an explicitly referenced target.
    ///
    /// The identifier is validated using the canonical compiler TargetId.
    ///
    /// Duplicate references are harmless and are stored only once.
    ///
    /// This operation does not register, discover or validate the existence
    /// of a target.
    ///
    /// Returns whether a new reference was inserted.
    pub fn add_target_reference(
        &mut self,
        target: impl Into<String>,
    ) -> ToolchainResult<bool> {
        let target = target.into();

        validate_target_reference(&target)?;

        Ok(self.target_references.insert(target))
    }

    /// Remove an explicitly referenced target.
    ///
    /// Returns whether the reference existed.
    pub fn remove_target_reference(
        &mut self,
        target: &str,
    ) -> bool {
        self.target_references.remove(target)
    }

    /// Replace all explicit target references.
    ///
    /// Validation is transactional: the existing set is preserved if any
    /// supplied target identifier is invalid.
    pub fn set_target_references(
        &mut self,
        targets: impl IntoIterator<Item = String>,
    ) -> ToolchainResult<()> {
        let mut validated = BTreeSet::new();

        for target in targets {
            validate_target_reference(&target)?;
            validated.insert(target);
        }

        self.target_references = validated;

        Ok(())
    }

    /// Inspect whether this profile explicitly references a target.
    ///
    /// Returns:
    ///
    /// - `Some(true)` when the target is explicitly referenced.
    /// - `Some(false)` when the profile declares references but does not
    ///   include this target.
    /// - `None` when the profile makes no explicit target-reference
    ///   declaration.
    ///
    /// This is descriptive scope inspection only. It is not a compatibility
    /// or feasibility decision.
    pub fn references_target(
        &self,
        target: &str,
    ) -> ToolchainResult<Option<bool>> {
        validate_target_reference(target)?;

        if self.target_references.is_empty() {
            return Ok(None);
        }

        Ok(Some(self.target_references.contains(target)))
    }

    // ========================================================================
    // PROVIDER CONFIGURATION
    // ========================================================================

    /// Insert or replace one provider-owned configuration section.
    ///
    /// The key is validated by ConfigSectionKey.
    ///
    /// The value is opaque to this module.
    ///
    /// Returns the previous section value, if one existed.
    pub fn set_provider_section(
        &mut self,
        key: ConfigSectionKey,
        value: Value,
    ) -> Option<Value> {
        self.provider_sections.insert(key, value)
    }

    /// Retrieve one provider-specific configuration section.
    #[must_use]
    pub fn provider_section(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&Value> {
        self.provider_sections.get(key)
    }

    /// Remove one provider-specific configuration section.
    pub fn remove_provider_section(
        &mut self,
        key: &ConfigSectionKey,
    ) -> Option<Value> {
        self.provider_sections.remove(key)
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Validate all target-profile invariants.
    ///
    /// This validates structural data only.
    ///
    /// It does not establish target existence, capability compatibility,
    /// hardware availability, authorization or feasibility.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.schema_version != TARGET_PROFILE_SCHEMA_VERSION {
            return Err(configuration_error(format!(
                "unsupported target-profile schema version {}; \
                 supported version is {}",
                self.schema_version,
                TARGET_PROFILE_SCHEMA_VERSION
            )));
        }

        for target in &self.target_references {
            validate_target_reference(target)?;
        }

        for key in self.provider_sections.keys() {
            ConfigSectionKey::new(key.as_str().to_owned())?;

            if key.as_str() == TARGET_PROFILE_SECTION_NAME {
                return Err(configuration_error(
                    "target-profile settings cannot recursively contain \
                     their own reserved configuration section"
                ));
            }
        }

        Ok(())
    }

    // ========================================================================
    // SERIALIZATION
    // ========================================================================

    /// Serialize the validated settings into JSON configuration data.
    ///
    /// Ordered collections provide deterministic logical serialization.
    ///
    /// The final byte representation is determined by the selected
    /// serialization implementation.
    pub fn to_value(&self) -> ToolchainResult<Value> {
        self.validate()?;

        serde_json::to_value(self).map_err(|error| {
            configuration_error(format!(
                "failed to serialize target-profile settings: {error}"
            ))
        })
    }

    /// Construct settings from JSON configuration data.
    ///
    /// Unknown fields, unsupported schema versions and invalid target
    /// identifiers are rejected.
    pub fn from_value(value: Value) -> ToolchainResult<Self> {
        let settings: Self =
            serde_json::from_value(value).map_err(|error| {
                configuration_error(format!(
                    "invalid serialized target-profile settings: {error}"
                ))
            })?;

        settings.validate()?;

        Ok(settings)
    }
}

// ============================================================================
// GENERIC PROFILE ADAPTERS
// ============================================================================

impl TargetProfileSettings {
    /// Read target-profile settings from an existing generic profile.
    ///
    /// The generic profile remains authoritative for:
    ///
    /// - ProfileId.
    /// - ProfileName.
    /// - Enabled state.
    /// - Parent relationships.
    /// - Inheritance.
    ///
    /// Missing target-profile settings produce an empty settings value.
    pub fn from_definition(
        profile: &ProfileDefinition,
    ) -> ToolchainResult<Self> {
        let key = target_profile_section_key()?;

        match profile.sections().get(&key) {
            Some(value) => Self::from_value(value.clone()),
            None => Ok(Self::new()),
        }
    }

    /// Read target-profile settings from an already resolved profile.
    ///
    /// Profile inheritance has already been handled by ProfileRegistry.
    ///
    /// This method does not resolve inheritance a second time.
    pub fn from_resolved(
        profile: &ResolvedProfile,
    ) -> ToolchainResult<Self> {
        let key = target_profile_section_key()?;

        match profile.section(&key) {
            Some(value) => Self::from_value(value.clone()),
            None => Ok(Self::new()),
        }
    }

    /// Replace the target-profile section of an existing generic profile.
    ///
    /// Other configuration sections are preserved.
    ///
    /// The generic profile remains responsible for its identity,
    /// registration, naming and inheritance.
    pub fn apply_to_definition(
        &self,
        profile: &mut ProfileDefinition,
    ) -> ToolchainResult<()> {
        self.validate()?;

        let key = target_profile_section_key()?;
        let value = self.to_value()?;

        let _previous = profile.set_section(key, value);

        profile.validate()
    }

    /// Remove target-profile settings from an existing generic profile.
    ///
    /// Other sections and profile relationships are preserved.
    ///
    /// Returns whether a target-profile section existed.
    pub fn remove_from_definition(
        profile: &mut ProfileDefinition,
    ) -> ToolchainResult<bool> {
        let key = target_profile_section_key()?;

        let removed = profile.remove_section(&key).is_some();

        profile.validate()?;

        Ok(removed)
    }
}

// ============================================================================
// TARGET REFERENCE VALIDATION
// ============================================================================

/// Validate a symbolic target reference through the canonical compiler API.
///
/// This intentionally does not duplicate TargetId's syntax rules.
fn validate_target_reference(
    value: &str,
) -> ToolchainResult<()> {
    TargetId::new(value.to_owned()).map_err(|error| {
        configuration_error(format!(
            "invalid canonical compiler target identifier `{value}`: {error}"
        ))
    })?;

    Ok(())
}

// ============================================================================
// ERROR ADAPTER
// ============================================================================

/// Construct a structured target-profile configuration error.
///
/// The toolchain error taxonomy remains authoritative.
fn configuration_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidConfiguration,
        message,
    )
}

// ============================================================================
// MODULE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::configuration::profiles::ProfileDefinition;

    use crate::toolchain::ids::{
        IdNamespace,
        ProfileId,
    };

    fn profile_id() -> ProfileId {
        ProfileId::new(
            IdNamespace::new("zamani.test")
                .expect("valid namespace"),
            1,
        )
        .expect("valid profile ID")
    }

    fn profile() -> ProfileDefinition {
        ProfileDefinition::new(
            profile_id(),
            "portable",
        )
        .expect("valid generic profile")
    }

    fn target(value: &str) -> String {
        value.to_owned()
    }

    // ========================================================================
    // CONSTRUCTION
    // ========================================================================

    #[test]
    fn empty_settings_are_valid() {
        let settings = TargetProfileSettings::new();

        assert!(settings.is_empty());
        assert_eq!(
            settings.schema_version(),
            TARGET_PROFILE_SCHEMA_VERSION
        );
    }

    #[test]
    fn default_settings_are_valid() {
        let settings = TargetProfileSettings::default();

        assert!(settings.validate().is_ok());
    }

    // ========================================================================
    // TARGET REFERENCES
    // ========================================================================

    #[test]
    fn valid_target_reference_is_accepted() {
        let mut settings = TargetProfileSettings::new();

        assert!(
            settings
                .add_target_reference("aarch64")
                .expect("valid target")
        );

        assert!(
            settings
                .target_references()
                .contains("aarch64")
        );
    }

    #[test]
    fn duplicate_target_reference_is_idempotent() {
        let mut settings = TargetProfileSettings::new();

        assert!(
            settings
                .add_target_reference("aarch64")
                .expect("valid target")
        );

        assert!(
            !settings
                .add_target_reference("aarch64")
                .expect("valid duplicate")
        );

        assert_eq!(settings.target_reference_count(), 1);
    }

    #[test]
    fn invalid_target_reference_is_rejected() {
        let mut settings = TargetProfileSettings::new();

        assert!(
            settings
                .add_target_reference("invalid target")
                .is_err()
        );

        assert!(settings.target_references().is_empty());
    }

    #[test]
    fn target_reference_replacement_is_transactional() {
        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("aarch64")
            .expect("valid target");

        let result = settings.set_target_references([
            target("wasm32"),
            target("invalid target"),
        ]);

        assert!(result.is_err());

        assert!(
            settings
                .target_references()
                .contains("aarch64")
        );

        assert!(
            !settings
                .target_references()
                .contains("wasm32")
        );
    }

    #[test]
    fn empty_target_scope_is_not_universal_support() {
        let settings = TargetProfileSettings::new();

        assert_eq!(
            settings
                .references_target("aarch64")
                .expect("valid target"),
            None
        );
    }

    #[test]
    fn explicit_target_scope_can_be_inspected() {
        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("aarch64")
            .expect("valid target");

        assert_eq!(
            settings
                .references_target("aarch64")
                .expect("valid target"),
            Some(true)
        );

        assert_eq!(
            settings
                .references_target("wasm32")
                .expect("valid target"),
            Some(false)
        );
    }

    #[test]
    fn removing_target_reference_is_explicit() {
        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("aarch64")
            .expect("valid target");

        assert!(settings.remove_target_reference("aarch64"));

        assert!(!settings.remove_target_reference("aarch64"));
    }

    // ========================================================================
    // PROVIDER SECTIONS
    // ========================================================================

    #[test]
    fn provider_section_is_stored_and_retrieved() {
        let mut settings = TargetProfileSettings::new();

        let key = ConfigSectionKey::new(
            "backend.example.options",
        )
        .expect("valid section key");

        let value = serde_json::json!({
            "mode": "portable"
        });

        let _previous = settings.set_provider_section(
            key.clone(),
            value.clone(),
        );

        assert_eq!(
            settings.provider_section(&key),
            Some(&value)
        );
    }

    #[test]
    fn provider_section_can_be_removed() {
        let mut settings = TargetProfileSettings::new();

        let key = ConfigSectionKey::new(
            "backend.example.options",
        )
        .expect("valid section key");

        let value = serde_json::json!({
            "mode": "portable"
        });

        let _previous = settings.set_provider_section(
            key.clone(),
            value,
        );

        assert!(
            settings
                .remove_provider_section(&key)
                .is_some()
        );

        assert!(
            settings
                .remove_provider_section(&key)
                .is_none()
        );
    }

    // ========================================================================
    // GENERIC PROFILE INTEGRATION
    // ========================================================================

    #[test]
    fn settings_round_trip_through_generic_profile() {
        let mut profile = profile();

        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("aarch64")
            .expect("valid target");

        let key = ConfigSectionKey::new(
            "backend.example.options",
        )
        .expect("valid section key");

        let _previous = settings.set_provider_section(
            key.clone(),
            serde_json::json!({
                "optimization": "portable"
            }),
        );

        settings
            .apply_to_definition(&mut profile)
            .expect("apply settings");

        let restored = TargetProfileSettings::from_definition(
            &profile,
        )
        .expect("restore settings");

        assert_eq!(restored, settings);
    }

    #[test]
    fn absent_profile_section_returns_empty_settings() {
        let profile = profile();

        let settings = TargetProfileSettings::from_definition(
            &profile,
        )
        .expect("empty settings");

        assert!(settings.is_empty());
    }

    #[test]
    fn removing_settings_preserves_generic_profile() {
        let mut profile = profile();

        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("wasm32")
            .expect("valid target");

        settings
            .apply_to_definition(&mut profile)
            .expect("apply settings");

        assert!(
            TargetProfileSettings::remove_from_definition(
                &mut profile
            )
            .expect("remove settings")
        );

        assert!(
            TargetProfileSettings::from_definition(
                &profile
            )
            .expect("read settings")
            .is_empty()
        );

        assert_eq!(profile.name().as_str(), "portable");
        assert!(profile.is_enabled());
    }

    // ========================================================================
    // SERIALIZATION
    // ========================================================================

    #[test]
    fn serialization_round_trip_is_deterministic() {
        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("wasm32")
            .expect("valid target");

        settings
            .add_target_reference("aarch64")
            .expect("valid target");

        let first = serde_json::to_string(&settings)
            .expect("serialize");

        let restored: TargetProfileSettings =
            serde_json::from_str(&first)
                .expect("deserialize");

        let second = serde_json::to_string(&restored)
            .expect("serialize again");

        assert_eq!(first, second);
        assert_eq!(settings, restored);
    }

    #[test]
    fn unsupported_schema_version_is_rejected() {
        let value = serde_json::json!({
            "schema_version": TARGET_PROFILE_SCHEMA_VERSION + 1,
            "target_references": [],
            "provider_sections": {}
        });

        assert!(
            TargetProfileSettings::from_value(value).is_err()
        );
    }

    #[test]
    fn malformed_target_reference_is_rejected_during_deserialization() {
        let value = serde_json::json!({
            "schema_version": TARGET_PROFILE_SCHEMA_VERSION,
            "target_references": ["invalid target"],
            "provider_sections": {}
        });

        assert!(
            TargetProfileSettings::from_value(value).is_err()
        );
    }

    #[test]
    fn unknown_top_level_fields_are_rejected() {
        let value = serde_json::json!({
            "schema_version": TARGET_PROFILE_SCHEMA_VERSION,
            "target_references": [],
            "provider_sections": {},
            "unknown": true
        });

        assert!(
            TargetProfileSettings::from_value(value).is_err()
        );
    }

    // ========================================================================
    // RESERVED SECTION
    // ========================================================================

    #[test]
    fn reserved_section_cannot_be_nested() {
        let mut settings = TargetProfileSettings::new();

        let key = ConfigSectionKey::new(
            TARGET_PROFILE_SECTION_NAME,
        )
        .expect("valid section key");

        let _previous = settings.set_provider_section(
            key,
            serde_json::Value::Null,
        );

        assert!(settings.validate().is_err());
    }

    // ========================================================================
    // DOMAIN EXTENSIBILITY
    // ========================================================================

    #[test]
    fn target_references_are_not_limited_to_a_closed_catalogue() {
        let mut settings = TargetProfileSettings::new();

        for value in [
            "aarch64",
            "wasm32",
            "quantum:provider:processor",
            "photonic:provider:device",
            "vendor:accelerator:model",
            "future.architecture",
        ] {
            settings
                .add_target_reference(value)
                .expect("valid open-world target reference");
        }

        assert_eq!(settings.target_reference_count(), 6);
    }

    #[test]
    fn settings_do_not_claim_target_feasibility() {
        let mut settings = TargetProfileSettings::new();

        settings
            .add_target_reference("quantum:provider:processor")
            .expect("valid target");

        // A symbolic reference is not evidence that a target exists.
        // Capability matching and feasibility remain downstream.
        assert!(
            settings
                .references_target("quantum:provider:processor")
                .expect("valid target")
                .unwrap_or(false)
        );
    }
}
