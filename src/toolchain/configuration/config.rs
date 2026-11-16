
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Configuration Foundation
//!
//! Path: src/toolchain/configuration/config.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical aggregate configuration for the Zamani toolchain.
//!
//! The configuration is shared by project management, workspace management,
//! dependency resolution, build orchestration, package management, registries,
//! targets, external tools, tasks, deployment, security and developer APIs.
//!
//! This module provides a stable configuration container without creating
//! competing implementations of compiler, backend, runtime, hardware,
//! resource-budget or semantic configuration.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - ToolchainConfig.
//! - ConfigSectionKey.
//! - Configuration construction.
//! - Configuration overlays.
//! - Configuration serialization.
//! - Configuration deserialization.
//! - Structural configuration validation.
//! - Typed access to extensible configuration sections.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - CompilerConfig or CompilationTarget.
//! - BackendConfig or backend resource budgets.
//! - Runtime configuration or scheduling.
//! - Physical hardware configuration.
//! - Target discovery or capability discovery.
//! - Resource allocation.
//! - Project manifest parsing.
//! - Security-policy evaluation.
//! - Build-profile implementation.
//! - External executable discovery or execution.
//! - Semantic requirements or constraints.
//! - Language-level resource limits.
//!
//! ## Authoritative integrations
//!
//! Toolchain identities:
//!     crate::toolchain::ids
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Compiler configuration:
//!     crate::compiler::configuration
//!
//! Backend configuration:
//!     crate::backend::config
//!
//! External toolchain discovery:
//!     crate::backend::toolchain
//!
//! Process execution:
//!     crate::backend::process
//!
//! Project manifests:
//!     crate::zamani_project_config
//!
//! ## Configuration ownership model
//!
//! ToolchainConfig is an aggregate.
//!
//! Domain-specific configuration sections are interpreted by their
//! authoritative consuming subsystems.
//!
//! For example:
//!
//!     compiler.optimization
//!         -> compiler configuration
//!
//!     backend.verification
//!         -> backend configuration
//!
//!     security.trust
//!         -> toolchain security policy
//!
//!     quantum.provider
//!         -> quantum provider integration
//!
//!     hdl.synthesis
//!         -> HDL provider integration
//!
//!     distributed.execution
//!         -> distributed execution integration
//!
//! These are namespaced configuration keys, not hard-coded supported
//! technologies or implementations.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal limits on:
//!
//! - CPU count.
//! - GPU count.
//! - FPGA count.
//! - QPU count.
//! - Qubit count.
//! - Memory capacity.
//! - Storage capacity.
//! - Register width.
//! - Tensor rank.
//! - Distributed participants.
//! - Build tasks.
//! - Targets.
//! - Providers.
//! - Computational domains.
//! - Configuration sections.
//!
//! Configuration collection sizes are bounded only by available
//! implementation resources and representational constraints.
//!
//! An absent configuration value does not mean that a resource is infinite.
//!
//! An absent target declaration does not establish a universal target.
//!
//! An absent requirement does not establish that every target is compatible.
//!
//! ## Resource semantics
//!
//! Requirement, Capability, Preference, Constraint, Hint and Budget
//! are separate concepts.
//!
//! This module preserves configuration data. It does not reinterpret
//! one category as another.
//!
//! Operational budgets are interpreted and enforced by their respective
//! configuration and execution authorities.
//!
//! ## Determinism
//!
//! Configuration sections use BTreeMap.
//!
//! Consequently:
//!
//! - Key iteration is deterministic.
//! - Serialized section ordering is deterministic.
//! - Configuration overlays have deterministic behavior.
//! - Configuration fingerprints can consume a stable representation.
//!
//! Deterministic serialization does not, by itself, guarantee a reproducible
//! build. The compiler, backend, providers, dependencies and environment
//! must establish that guarantee.
//!
//! ## Security
//!
//! This module performs no filesystem access, network access, process
//! execution, environment-variable lookup or hardware discovery.
//!
//! Paths are stored as supplied. Canonicalization and authorization belong
//! to the filesystem/security boundary that consumes them.
//!
//! Credentials, private keys and access tokens should not be stored in
//! ordinary configuration values. Use the designated credential providers.
//!
//! Unknown configuration sections are preserved rather than silently
//! discarded. Their interpretation and authorization remain the responsibility
//! of the consuming subsystem.
//!
//! Deserialization performs structural validation but does not certify
//! that the configuration is authorized or operationally feasible.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::toolchain::errors::{ToolchainError, ToolchainResult};
use crate::toolchain::ids::ProfileId;

// ============================================================================
// CONFIGURATION SECTION IDENTITIES
// ============================================================================

/// Namespaced identifier for an extensible configuration section.
///
/// Examples:
///
/// - `compiler`
/// - `compiler.optimization`
/// - `backend.verification`
/// - `security.trust`
/// - `quantum::provider`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `future_domain.extension`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConfigSectionKey(String);

impl ConfigSectionKey {
    /// Construct and validate a configuration section key.
    ///
    /// Keys are case-sensitive.
    ///
    /// No arbitrary maximum key length is imposed.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_section_key(&value)?;

        Ok(Self(value))
    }

    /// Return the canonical key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the key and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ConfigSectionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for ConfigSectionKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for ConfigSectionKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ConfigSectionKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Validate an open-world configuration section identifier.
///
/// The allowed characters are deliberately independent of the available
/// computing domains.
///
/// This allows future domains and third-party extensions to introduce
/// new namespaced configuration sections without modifying this module.
fn validate_section_key(value: &str) -> ToolchainResult<()> {
    let mut characters = value.chars();

    let first = characters.next().ok_or_else(|| {
        ToolchainError::invalid_configuration(
            "configuration section key cannot be empty",
        )
    })?;

    if !first.is_ascii_alphanumeric() {
        return Err(ToolchainError::invalid_configuration(
            "configuration section key must begin with an ASCII letter or digit",
        ));
    }

    if value.trim() != value {
        return Err(ToolchainError::invalid_configuration(
            "configuration section key cannot contain surrounding whitespace",
        ));
    }

    if !characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(
                character,
                '.' | '_' | '-' | ':' | '/'
            )
    }) {
        return Err(ToolchainError::invalid_configuration(
            "configuration section key contains unsupported characters",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ToolchainError::invalid_configuration(
            "configuration section key cannot contain control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// CANONICAL CONFIGURATION
// ============================================================================

/// Canonical aggregate configuration for the Zamani toolchain.
///
/// This type stores configuration intent. It does not execute operations,
/// discover hardware, select a physical device or allocate resources.
///
/// Domain-specific configuration is stored in namespaced sections and
/// interpreted by the subsystem that owns the corresponding behavior.
///
/// # Configuration precedence
///
/// A complete configuration represents the effective configuration at
/// the point where it is passed to a toolchain service.
///
/// Configuration loading and source precedence belong to the configuration
/// loading and manifest integration layers.
///
/// `merge_overlay` provides an explicit overlay operation for already
/// constructed configurations.
///
/// # Project root
///
/// The optional project root is an explicitly supplied path.
///
/// This type never assumes that the process current working directory is
/// the project root.
///
/// # Profiles
///
/// Profile identities reuse the existing toolchain `ProfileId`.
///
/// Profile definitions and profile resolution belong to profiles.rs.
///
/// # Unknown sections
///
/// Unknown section identifiers and values are preserved. A consumer must
/// explicitly recognize and validate a section before relying on its
/// semantics.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolchainConfig {
    /// Optional explicitly selected toolchain profile.
    profile: Option<ProfileId>,

    /// Optional explicitly selected project root.
    project_root: Option<PathBuf>,

    /// Open-world, namespaced configuration sections.
    ///
    /// BTreeMap provides deterministic iteration and serialization order.
    sections: BTreeMap<ConfigSectionKey, serde_json::Value>,
}

/// Wire representation used to validate configuration during deserialization.
///
/// Unknown top-level fields are rejected. Unknown namespaced sections are
/// preserved because they may belong to future or third-party providers.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ToolchainConfigWire {
    #[serde(default)]
    profile: Option<ProfileId>,

    #[serde(default)]
    project_root: Option<PathBuf>,

    #[serde(default)]
    sections: BTreeMap<ConfigSectionKey, serde_json::Value>,
}

impl<'de> Deserialize<'de> for ToolchainConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ToolchainConfigWire::deserialize(deserializer)?;

        let config = Self {
            profile: wire.profile,
            project_root: wire.project_root,
            sections: wire.sections,
        };

        config
            .validate()
            .map_err(serde::de::Error::custom)?;

        Ok(config)
    }
}

impl Default for ToolchainConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolchainConfig {
    /// Construct an empty toolchain configuration.
    ///
    /// No target, computing domain, compiler, backend, provider or physical
    /// machine is selected implicitly.
    #[must_use]
    pub fn new() -> Self {
        Self {
            profile: None,
            project_root: None,
            sections: BTreeMap::new(),
        }
    }

    /// Return the selected profile, if any.
    #[must_use]
    pub fn profile(&self) -> Option<&ProfileId> {
        self.profile.as_ref()
    }

    /// Set the selected profile.
    pub fn set_profile(&mut self, profile: ProfileId) {
        self.profile = Some(profile);
    }

    /// Clear the selected profile.
    pub fn clear_profile(&mut self) {
        self.profile = None;
    }

    /// Return the explicitly configured project root.
    #[must_use]
    pub fn project_root(&self) -> Option<&std::path::Path> {
        self.project_root.as_deref()
    }

    /// Set the explicitly configured project root.
    ///
    /// This method performs structural validation only.
    ///
    /// It does not check whether the directory exists or whether the caller
    /// has permission to access it.
    pub fn set_project_root(
        &mut self,
        project_root: impl Into<PathBuf>,
    ) -> ToolchainResult<()> {
        let project_root = project_root.into();

        if project_root.as_os_str().is_empty() {
            return Err(ToolchainError::invalid_configuration(
                "project root cannot be an empty path",
            ));
        }

        self.project_root = Some(project_root);

        Ok(())
    }

    /// Clear the configured project root.
    pub fn clear_project_root(&mut self) {
        self.project_root = None;
    }

    /// Return the number of registered configuration sections.
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Return whether no configuration sections are registered.
    #[must_use]
    pub fn sections_are_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Return whether the configuration contains no explicit settings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profile.is_none()
            && self.project_root.is_none()
            && self.sections.is_empty()
    }

    /// Return an immutable reference to the complete section map.
    #[must_use]
    pub fn sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, serde_json::Value> {
        &self.sections
    }

    /// Check whether a configuration section exists.
    #[must_use]
    pub fn contains_section(&self, key: &ConfigSectionKey) -> bool {
        self.sections.contains_key(key)
    }

    /// Retrieve a configuration section as an untyped JSON value.
    #[must_use]
    pub fn section(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&serde_json::Value> {
        self.sections.get(key)
    }

    /// Retrieve a configuration section as a strongly typed value.
    ///
    /// The consuming subsystem owns the target type and its semantic
    /// validation.
    ///
    /// Successful deserialization proves only that the stored value can
    /// be represented by the requested Rust type.
    pub fn section_as<T>(
        &self,
        key: &ConfigSectionKey,
    ) -> ToolchainResult<Option<T>>
    where
        T: DeserializeOwned,
    {
        let Some(value) = self.sections.get(key) else {
            return Ok(None);
        };

        serde_json::from_value(value.clone())
            .map(Some)
            .map_err(|source| {
                ToolchainError::invalid_configuration(
                    format!(
                        "configuration section `{key}` cannot be decoded \
                         into the requested type: {source}"
                    ),
                )
                .with_source(source)
            })
    }

    /// Insert or replace a configuration section.
    ///
    /// The key must be valid.
    ///
    /// The value must be serializable by serde_json.
    ///
    /// The previous value is returned when replacing an existing section.
    pub fn set_section<T>(
        &mut self,
        key: ConfigSectionKey,
        value: &T,
    ) -> ToolchainResult<Option<serde_json::Value>>
    where
        T: Serialize + ?Sized,
    {
        let value = serde_json::to_value(value).map_err(|source| {
            ToolchainError::invalid_configuration(
                format!(
                    "configuration section `{key}` cannot be serialized: \
                     {source}"
                ),
            )
            .with_source(source)
        })?;

        Ok(self.sections.insert(key, value))
    }

    /// Insert or replace a configuration section using an existing JSON value.
    ///
    /// This is useful for providers and integrations that already use
    /// serde_json as their serialization boundary.
    pub fn set_section_value(
        &mut self,
        key: ConfigSectionKey,
        value: serde_json::Value,
    ) -> Option<serde_json::Value> {
        self.sections.insert(key, value)
    }

    /// Remove a configuration section.
    ///
    /// Returns the previous value when the section existed.
    pub fn remove_section(
        &mut self,
        key: &ConfigSectionKey,
    ) -> Option<serde_json::Value> {
        self.sections.remove(key)
    }

    /// Apply another complete configuration as an explicit overlay.
    ///
    /// Overlay behavior:
    ///
    /// - A supplied profile replaces the existing profile.
    /// - An absent overlay profile preserves the existing profile.
    /// - A supplied project root replaces the existing project root.
    /// - An absent overlay project root preserves the existing root.
    /// - Overlay sections replace matching base sections.
    /// - Unrelated base sections are preserved.
    ///
    /// An overlay cannot clear the profile or project root by supplying None.
    /// Use `clear_profile` or `clear_project_root` for explicit removal.
    ///
    /// The method does not mutate either input.
    pub fn merge_overlay(
        &self,
        overlay: &Self,
    ) -> ToolchainResult<Self> {
        let mut merged = self.clone();

        if let Some(profile) = &overlay.profile {
            merged.profile = Some(profile.clone());
        }

        if let Some(project_root) = &overlay.project_root {
            merged.project_root = Some(project_root.clone());
        }

        merged.sections.extend(
            overlay.sections.iter().map(|(key, value)| {
                (key.clone(), value.clone())
            }),
        );

        merged.validate()?;

        Ok(merged)
    }

    /// Validate the structural integrity of this configuration.
    ///
    /// This method does not:
    ///
    /// - Resolve profile inheritance.
    /// - Validate profile availability.
    /// - Interpret domain-specific settings.
    /// - Validate security policies.
    /// - Validate resource budgets.
    /// - Discover external tools.
    /// - Inspect the filesystem.
    /// - Discover hardware.
    /// - Verify target feasibility.
    /// - Allocate resources.
    ///
    /// Those operations belong to the corresponding integration modules.
    pub fn validate(&self) -> ToolchainResult<()> {
        if let Some(project_root) = &self.project_root {
            if project_root.as_os_str().is_empty() {
                return Err(ToolchainError::invalid_configuration(
                    "project root cannot be an empty path",
                ));
            }
        }

        for key in self.sections.keys() {
            validate_section_key(key.as_str())?;
        }

        Ok(())
    }

    /// Serialize the complete configuration to deterministic JSON.
    ///
    /// BTreeMap guarantees deterministic ordering of section keys.
    pub fn to_json(&self) -> ToolchainResult<String> {
        self.validate()?;

        serde_json::to_string_pretty(self).map_err(|source| {
            ToolchainError::invalid_configuration(
                format!(
                    "failed to serialize toolchain configuration: {source}"
                ),
            )
            .with_source(source)
        })
    }

    /// Deserialize a configuration from JSON.
    ///
    /// Deserialization includes structural validation.
    ///
    /// Unknown top-level fields are rejected.
    ///
    /// Unknown configuration sections are retained.
    pub fn from_json(input: &str) -> ToolchainResult<Self> {
        serde_json::from_str(input).map_err(|source| {
            ToolchainError::invalid_configuration(
                format!(
                    "failed to deserialize toolchain configuration: {source}"
                ),
            )
            .with_source(source)
        })
    }

    /// Create a builder for constructing a configuration.
    #[must_use]
    pub fn builder() -> ToolchainConfigBuilder {
        ToolchainConfigBuilder::new()
    }
}

// ============================================================================
// CONFIGURATION BUILDER
// ============================================================================

/// Builder for constructing validated toolchain configurations.
///
/// The builder provides a convenient construction API while keeping the
/// resulting configuration subject to the same validation contract.
///
/// No filesystem, network, process or hardware operations are performed.
#[derive(Clone, Debug, Default)]
pub struct ToolchainConfigBuilder {
    config: ToolchainConfig,
}

impl ToolchainConfigBuilder {
    /// Create a new configuration builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ToolchainConfig::new(),
        }
    }

    /// Select a toolchain profile.
    #[must_use]
    pub fn profile(mut self, profile: ProfileId) -> Self {
        self.config.set_profile(profile);
        self
    }

    /// Set an explicit project root.
    ///
    /// Invalid paths are reported by `build`.
    #[must_use]
    pub fn project_root(
        mut self,
        project_root: impl Into<PathBuf>,
    ) -> Self {
        self.config.project_root = Some(project_root.into());
        self
    }

    /// Add or replace a namespaced configuration section.
    ///
    /// The key is validated before the builder is updated.
    pub fn section<T>(
        mut self,
        key: impl Into<String>,
        value: &T,
    ) -> ToolchainResult<Self>
    where
        T: Serialize + ?Sized,
    {
        let key = ConfigSectionKey::new(key)?;

        self.config.set_section(key, value)?;

        Ok(self)
    }

    /// Add or replace a section from an existing JSON value.
    pub fn section_value(
        mut self,
        key: impl Into<String>,
        value: serde_json::Value,
    ) -> ToolchainResult<Self> {
        let key = ConfigSectionKey::new(key)?;

        self.config.set_section_value(key, value);

        Ok(self)
    }

    /// Validate and construct the final configuration.
    pub fn build(self) -> ToolchainResult<ToolchainConfig> {
        self.config.validate()?;

        Ok(self.config)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct ExampleSettings {
        enabled: bool,
        strategy: String,
    }

    fn key(value: &str) -> ConfigSectionKey {
        ConfigSectionKey::new(value).expect("valid configuration key")
    }

    #[test]
    fn default_configuration_is_empty() {
        let config = ToolchainConfig::default();

        assert!(config.is_empty());
        assert_eq!(config.section_count(), 0);
        assert!(config.profile().is_none());
        assert!(config.project_root().is_none());
    }

    #[test]
    fn accepts_open_world_namespaced_keys() {
        for value in [
            "compiler",
            "compiler.optimization",
            "backend.verification",
            "quantum::provider",
            "hdl.synthesis",
            "distributed.execution",
            "future_domain.extension",
        ] {
            assert!(
                ConfigSectionKey::new(value).is_ok(),
                "expected valid key: {value}"
            );
        }
    }

    #[test]
    fn rejects_empty_section_keys() {
        assert!(ConfigSectionKey::new("").is_err());
    }

    #[test]
    fn rejects_whitespace_in_section_keys() {
        assert!(ConfigSectionKey::new("compiler settings").is_err());
        assert!(ConfigSectionKey::new(" compiler").is_err());
        assert!(ConfigSectionKey::new("compiler ").is_err());
    }

    #[test]
    fn rejects_invalid_section_key_characters() {
        assert!(ConfigSectionKey::new("compiler$settings").is_err());
        assert!(ConfigSectionKey::new("compiler.settings!").is_err());
        assert!(ConfigSectionKey::new("compiler\nsettings").is_err());
    }

    #[test]
    fn preserves_case_sensitive_section_names() {
        let lower = key("provider.backend");
        let upper = key("Provider.Backend");

        assert_ne!(lower, upper);
    }

    #[test]
    fn inserts_and_retrieves_sections() {
        let mut config = ToolchainConfig::new();

        let settings = ExampleSettings {
            enabled: true,
            strategy: "adaptive".to_owned(),
        };

        config
            .set_section(key("example.provider"), &settings)
            .expect("insert section");

        let retrieved: Option<ExampleSettings> = config
            .section_as(&key("example.provider"))
            .expect("decode section");

        assert_eq!(retrieved, Some(settings));
    }

    #[test]
    fn replacement_returns_previous_value() {
        let mut config = ToolchainConfig::new();

        config.set_section(
            key("example"),
            &serde_json::json!({"value": 1}),
        )
        .expect("first insertion");

        let previous = config
            .set_section(
                key("example"),
                &serde_json::json!({"value": 2}),
            )
            .expect("replacement");

        assert_eq!(
            previous,
            Some(serde_json::json!({"value": 1}))
        );

        assert_eq!(
            config.section(&key("example")),
            Some(&serde_json::json!({"value": 2}))
        );
    }

    #[test]
    fn removes_sections() {
        let mut config = ToolchainConfig::new();

        config.set_section(
            key("example"),
            &serde_json::json!({"enabled": true}),
        )
        .expect("insert");

        let removed = config.remove_section(&key("example"));

        assert!(removed.is_some());
        assert!(!config.contains_section(&key("example")));
    }

    #[test]
    fn missing_sections_return_none() {
        let config = ToolchainConfig::new();

        assert!(
            config
                .section_as::<ExampleSettings>(&key("missing"))
                .expect("missing section is not an error")
                .is_none()
        );
    }

    #[test]
    fn typed_deserialization_errors_are_reported() {
        let mut config = ToolchainConfig::new();

        config
            .set_section(
                key("example"),
                &serde_json::json!({"enabled": "not-a-boolean"}),
            )
            .expect("insert");

        assert!(
            config
                .section_as::<ExampleSettings>(&key("example"))
                .is_err()
        );
    }

    #[test]
    fn configuration_overlay_preserves_unrelated_sections() {
        let mut base = ToolchainConfig::new();

        base.set_section(
            key("compiler"),
            &serde_json::json!({"optimization": "normal"}),
        )
        .expect("base compiler");

        base.set_section(
            key("security"),
            &serde_json::json!({"verification": true}),
        )
        .expect("base security");

        let mut overlay = ToolchainConfig::new();

        overlay.set_section(
            key("compiler"),
            &serde_json::json!({"optimization": "aggressive"}),
        )
        .expect("overlay compiler");

        let merged = base
            .merge_overlay(&overlay)
            .expect("merge overlay");

        assert_eq!(
            merged.section(&key("compiler")),
            Some(&serde_json::json!({"optimization": "aggressive"}))
        );

        assert_eq!(
            merged.section(&key("security")),
            Some(&serde_json::json!({"verification": true}))
        );

        // Inputs remain unchanged.
        assert_eq!(
            base.section(&key("compiler")),
            Some(&serde_json::json!({"optimization": "normal"}))
        );
    }

    #[test]
    fn overlay_does_not_implicitly_clear_profile_or_root() {
        let mut base = ToolchainConfig::new();

        base.set_project_root("project").expect("project root");

        let overlay = ToolchainConfig::new();

        let merged = base
            .merge_overlay(&overlay)
            .expect("merge overlay");

        assert_eq!(
            merged.project_root(),
            Some(std::path::Path::new("project"))
        );
    }

    #[test]
    fn explicit_project_root_can_be_cleared() {
        let mut config = ToolchainConfig::new();

        config
            .set_project_root("project")
            .expect("project root");

        config.clear_project_root();

        assert!(config.project_root().is_none());
    }

    #[test]
    fn explicit_profile_can_be_cleared() {
        let mut config = ToolchainConfig::new();

        config.clear_profile();

        assert!(config.profile().is_none());
    }

    #[test]
    fn rejects_empty_project_root() {
        let mut config = ToolchainConfig::new();

        assert!(config.set_project_root(PathBuf::new()).is_err());
    }

    #[test]
    fn serializes_and_deserializes_configuration() {
        let mut config = ToolchainConfig::new();

        config
            .set_section(
                key("future.provider"),
                &serde_json::json!({
                    "enabled": true,
                    "configuration": {
                        "strategy": "adaptive"
                    }
                }),
            )
            .expect("insert");

        let encoded = config.to_json().expect("serialize");

        let decoded = ToolchainConfig::from_json(&encoded)
            .expect("deserialize");

        assert_eq!(config, decoded);
    }

    #[test]
    fn deserialization_rejects_invalid_section_keys() {
        let encoded = r#"
        {
            "sections": {
                "invalid$key": {
                    "enabled": true
                }
            }
        }
        "#;

        assert!(ToolchainConfig::from_json(encoded).is_err());
    }

    #[test]
    fn deserialization_rejects_unknown_top_level_fields() {
        let encoded = r#"
        {
            "unknown_top_level_field": true
        }
        "#;

        assert!(ToolchainConfig::from_json(encoded).is_err());
    }

    #[test]
    fn preserves_unknown_future_configuration_sections() {
        let encoded = r#"
        {
            "sections": {
                "future.computing.domain": {
                    "enabled": true,
                    "mode": "experimental"
                }
            }
        }
        "#;

        let config = ToolchainConfig::from_json(encoded)
            .expect("future section should be preserved");

        assert!(
            config.contains_section(
                &key("future.computing.domain")
            )
        );
    }

    #[test]
    fn configuration_iteration_is_deterministic() {
        let mut config = ToolchainConfig::new();

        config
            .set_section(key("z.domain"), &serde_json::json!(1))
            .expect("insert z");

        config
            .set_section(key("a.domain"), &serde_json::json!(2))
            .expect("insert a");

        let keys: Vec<&str> = config
            .sections()
            .keys()
            .map(ConfigSectionKey::as_str)
            .collect();

        assert_eq!(keys, vec!["a.domain", "z.domain"]);
    }

    #[test]
    fn builder_creates_valid_configuration() {
        let config = ToolchainConfig::builder()
            .project_root("project")
            .expect("valid section key")
            .section(
                "compiler.optimization",
                &serde_json::json!({
                    "mode": "normal"
                }),
            )
            .expect("valid section")
            .build()
            .expect("valid configuration");

        assert_eq!(
            config.project_root(),
            Some(std::path::Path::new("project"))
        );

        assert!(
            config.contains_section(
                &key("compiler.optimization")
            )
        );
    }

    #[test]
    fn configuration_does_not_select_a_target_implicitly() {
        let config = ToolchainConfig::new();

        assert!(config.section(&key("targets")).is_none());
        assert!(config.section(&key("backend.target")).is_none());
    }

    #[test]
    fn configuration_does_not_impose_hardware_limits() {
        let mut config = ToolchainConfig::new();

        config
            .set_section(
                key("resource.requirements"),
                &serde_json::json!({
                    "quantum.logical_qubits": "program_defined",
                    "compute.parallelism": "workload_defined",
                    "memory.capacity": "requirement_defined",
                    "distributed.participants": "workload_defined"
                }),
            )
            .expect("insert open-world resource intent");

        assert!(config.validate().is_ok());
    }
}
