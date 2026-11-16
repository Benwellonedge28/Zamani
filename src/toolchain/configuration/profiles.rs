
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Configuration Profiles
//!
//! Path: `src/toolchain/configuration/profiles.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines reusable, named toolchain configuration profiles, profile
//! inheritance, deterministic resolution and integration with
//! `ToolchainConfig`.
//!
//! A profile is a reusable collection of configuration sections.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - ProfileName.
//! - ProfileDefinition.
//! - ProfileRegistry.
//! - ResolvedProfile.
//! - Profile inheritance validation.
//! - Profile cycle detection.
//! - Deterministic profile composition.
//! - Configuration-section overlays.
//! - Profile serialization and deserialization.
//! - ToolchainConfig integration.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Source-language profile declarations or parsing.
//! - Frontend AST or semantic profile interpretation.
//! - Requirement evaluation.
//! - Capability discovery.
//! - Preference scoring.
//! - Constraint solving.
//! - Feasibility determination.
//! - Security authorization.
//! - Resource allocation.
//! - Hardware discovery.
//! - Physical target selection.
//! - Compiler optimization planning.
//! - Compiler pass registration or execution.
//! - Runtime scheduling.
//! - Backend generation.
//!
//! ## Authoritative integrations
//!
//! Toolchain identities:
//!     crate::toolchain::ids::ProfileId
//!
//! Toolchain configuration:
//!     crate::toolchain::configuration::config
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Configuration policies:
//!     crate::toolchain::configuration::policies
//!
//! Compiler optimization planning:
//!     crate::compiler::optimization::planning
//!
//! Project manifest:
//!     crate::zamani_project_config
//!
//! Source profile grammar:
//!     grammar/compile/profiles.g4
//!
//! ## Composition semantics
//!
//! - A profile may extend zero or more parent profiles.
//! - All referenced parents must exist.
//! - Inheritance cycles are rejected.
//! - Parent profiles are resolved before their child.
//! - Multiple parents are composed deterministically.
//! - Conflicting values from different parents are rejected.
//! - Identical values inherited from multiple parents are accepted.
//! - A child profile explicitly overrides inherited values.
//! - Disabled profiles cannot be resolved.
//!
//! Parent order in source declarations does not create an implicit
//! precedence rule. Conflicting parent values must be resolved explicitly
//! in the child profile.
//!
//! ## Configuration semantics
//!
//! Profile sections are opaque JSON configuration values.
//!
//! Their meaning is established and validated by their owning subsystem.
//!
//! This module does not reinterpret Requirements, Capabilities,
//! Preferences, Constraints, Hints or Budgets.
//!
//! ## POCO-REAF and scalability
//!
//! Profiles and their properties are open-world.
//!
//! This module defines no arbitrary maximum for:
//!
//! - Number of profiles.
//! - Number of profile parents.
//! - Number of inherited profiles.
//! - Number of configuration sections.
//! - Number of configuration properties.
//! - Number of computing domains.
//! - Number of providers.
//! - Number of targets.
//! - CPU, GPU, FPGA, QPU or node counts.
//! - Qubit counts.
//! - Memory or storage capacity.
//! - Tensor rank.
//!
//! Processing and storage are bounded by available implementation resources
//! and representational constraints.
//!
//! Graph traversal is iterative and does not consume call-stack depth
//! proportional to the inheritance graph.
//!
//! ## Security
//!
//! This module performs no filesystem, network, environment, process,
//! credential, hardware or authorization operations.
//!
//! Configuration profiles must not be used as an authorization mechanism.
//! Secrets must be managed by the designated credential providers.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize};

use crate::toolchain::configuration::config::{
    ConfigSectionKey,
    ToolchainConfig,
};
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};
use crate::toolchain::ids::ProfileId;

/// Configuration section containing the serialized profile registry.
const PROFILE_REGISTRY_SECTION: &str = "toolchain.profiles";

// ============================================================================
// PROFILE NAME
// ============================================================================

/// A validated human-readable profile name.
///
/// Names are case-sensitive.
///
/// The accepted syntax is deliberately open to qualified names so that
/// profiles can be namespaced by projects, organizations and providers.
///
/// Examples:
///
/// - `development`
/// - `production`
/// - `portable`
/// - `quantum.portable`
/// - `vendor::accelerated`
///
/// A profile name is not a profile identity. Use [`ProfileId`] for identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProfileName(String);

impl ProfileName {
    /// Construct and validate a profile name.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_profile_name(&value)?;

        Ok(Self(value))
    }

    /// Return the profile name as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the name and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ProfileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for ProfileName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'de> Deserialize<'de> for ProfileName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

fn validate_profile_name(value: &str) -> ToolchainResult<()> {
    if value.is_empty() {
        return Err(invalid_profile("profile name cannot be empty"));
    }

    if value.trim() != value {
        return Err(invalid_profile(
            "profile name cannot contain surrounding whitespace",
        ));
    }

    let mut characters = value.chars();

    let first = characters
        .next()
        .ok_or_else(|| invalid_profile("profile name cannot be empty"))?;

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(invalid_profile(
            "profile name must begin with an ASCII letter or underscore",
        ));
    }

    if !characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '-' | '.' | ':')
    }) {
        return Err(invalid_profile(
            "profile name contains unsupported characters",
        ));
    }

    if value.contains(":::") || value.ends_with(':') {
        return Err(invalid_profile(
            "profile name contains an invalid namespace separator",
        ));
    }

    Ok(())
}

// ============================================================================
// PROFILE DEFINITION
// ============================================================================

/// A reusable toolchain configuration profile.
///
/// A profile contains:
///
/// - A stable toolchain profile identity.
/// - A validated human-readable name.
/// - Optional descriptive metadata.
/// - An enabled/disabled state.
/// - Zero or more parent profile identities.
/// - Namespaced configuration sections.
///
/// The profile does not contain compiler implementation objects, physical
/// device assignments, runtime scheduling instructions or authorization
/// grants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileDefinition {
    id: ProfileId,
    name: ProfileName,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(default = "default_enabled")]
    enabled: bool,

    #[serde(default)]
    parents: BTreeSet<ProfileId>,

    #[serde(default)]
    sections: BTreeMap<ConfigSectionKey, serde_json::Value>,
}

const fn default_enabled() -> bool {
    true
}

impl ProfileDefinition {
    /// Create a new enabled profile without parents or configuration sections.
    pub fn new(
        id: ProfileId,
        name: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let profile = Self {
            id,
            name: ProfileName::new(name)?,
            description: None,
            enabled: true,
            parents: BTreeSet::new(),
            sections: BTreeMap::new(),
        };

        profile.validate()?;

        Ok(profile)
    }

    /// Return the stable profile identity.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        &self.id
    }

    /// Return the validated profile name.
    #[must_use]
    pub fn name(&self) -> &ProfileName {
        &self.name
    }

    /// Return optional descriptive metadata.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return whether the profile is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Return the parent profile identities in deterministic order.
    #[must_use]
    pub fn parents(&self) -> &BTreeSet<ProfileId> {
        &self.parents
    }

    /// Return the configuration sections owned by this profile.
    #[must_use]
    pub fn sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, serde_json::Value> {
        &self.sections
    }

    /// Set or replace the profile description.
    ///
    /// An empty description is rejected. Pass `None` to clear it.
    pub fn set_description(
        &mut self,
        description: Option<String>,
    ) -> ToolchainResult<()> {
        if let Some(value) = &description {
            if value.trim().is_empty() {
                return Err(invalid_profile(
                    "profile description cannot be empty or whitespace",
                ));
            }
        }

        self.description = description;

        Ok(())
    }

    /// Set whether this profile is enabled.
    ///
    /// Disabled profiles remain registered but cannot be resolved.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Add a parent profile.
    ///
    /// Duplicate parent identities are harmless and are represented only
    /// once. Parent existence and cycle validity are checked by the registry.
    pub fn add_parent(&mut self, parent: ProfileId) {
        self.parents.insert(parent);
    }

    /// Remove a parent profile reference.
    ///
    /// Returns whether the parent was present.
    pub fn remove_parent(&mut self, parent: &ProfileId) -> bool {
        self.parents.remove(parent)
    }

    /// Replace the complete parent set.
    pub fn set_parents(
        &mut self,
        parents: impl IntoIterator<Item = ProfileId>,
    ) {
        self.parents = parents.into_iter().collect();
    }

    /// Set or replace one namespaced configuration section.
    ///
    /// The section value is opaque to this module. Its owning subsystem
    /// remains responsible for semantic validation.
    pub fn set_section(
        &mut self,
        key: ConfigSectionKey,
        value: serde_json::Value,
    ) -> Option<serde_json::Value> {
        self.sections.insert(key, value)
    }

    /// Remove one configuration section.
    pub fn remove_section(
        &mut self,
        key: &ConfigSectionKey,
    ) -> Option<serde_json::Value> {
        self.sections.remove(key)
    }

    /// Validate the local structure of this profile.
    ///
    /// Parent existence and inheritance cycles require registry context
    /// and are therefore validated by [`ProfileRegistry::validate`].
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_profile_name(self.name.as_str())?;

        if let Some(description) = &self.description {
            if description.trim().is_empty() {
                return Err(invalid_profile(
                    "profile description cannot be empty or whitespace",
                ));
            }
        }

        for key in self.sections.keys() {
            // ConfigSectionKey validates itself on construction and
            // deserialization. Reconstructing it also protects this
            // boundary if the representation changes in the future.
            ConfigSectionKey::new(key.as_str().to_owned())?;
        }

        if self.parents.contains(&self.id) {
            return Err(invalid_profile(
                "a profile cannot inherit directly from itself",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// RESOLVED PROFILE
// ============================================================================

/// A validated, flattened configuration profile.
///
/// `sections` contains the final values after deterministic inheritance
/// resolution.
///
/// `lineage` contains the unique profiles contributing to the result,
/// ordered with ancestors before descendants. The selected profile is last.
///
/// A resolved profile is configuration data, not proof of target feasibility,
/// authorization, resource availability or successful compilation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedProfile {
    id: ProfileId,
    name: ProfileName,
    sections: BTreeMap<ConfigSectionKey, serde_json::Value>,
    lineage: Vec<ProfileId>,
}

impl ResolvedProfile {
    /// Return the identity of the selected profile.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        &self.id
    }

    /// Return the selected profile's name.
    #[must_use]
    pub fn name(&self) -> &ProfileName {
        &self.name
    }

    /// Return the flattened configuration sections.
    #[must_use]
    pub fn sections(
        &self,
    ) -> &BTreeMap<ConfigSectionKey, serde_json::Value> {
        &self.sections
    }

    /// Return the inheritance lineage, ancestors first.
    #[must_use]
    pub fn lineage(&self) -> &[ProfileId] {
        &self.lineage
    }

    /// Retrieve one resolved configuration section.
    #[must_use]
    pub fn section(
        &self,
        key: &ConfigSectionKey,
    ) -> Option<&serde_json::Value> {
        self.sections.get(key)
    }

    /// Apply this resolved profile to an existing toolchain configuration.
    ///
    /// The selected profile identity is updated.
    ///
    /// Resolved sections replace matching sections in the configuration.
    /// Unrelated sections are preserved.
    ///
    /// This method does not clear sections that are absent from the profile.
    /// This avoids silently deleting configuration supplied by another
    /// configuration source or subsystem.
    pub fn apply_to(
        &self,
        config: &mut ToolchainConfig,
    ) -> ToolchainResult<()> {
        config.set_profile(self.id.clone());

        for (key, value) in &self.sections {
            config.set_section_value(key.clone(), value.clone());
        }

        config.validate()
    }
}

// ============================================================================
// PROFILE REGISTRY
// ============================================================================

/// Registry of reusable toolchain configuration profiles.
///
/// The primary map is keyed by stable [`ProfileId`].
///
/// A secondary name index provides efficient human-readable lookup.
///
/// The name index is never trusted without validation. Deserialization
/// validates that it exactly matches the primary profile map.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileRegistry {
    profiles: BTreeMap<ProfileId, ProfileDefinition>,

    /// Secondary name-to-identity lookup index.
    ///
    /// This index is derived from the authoritative `profiles` map and is
    /// validated after deserialization and before resolution.
    names: BTreeMap<String, ProfileId>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileRegistryWire {
    #[serde(default)]
    profiles: BTreeMap<ProfileId, ProfileDefinition>,

    #[serde(default)]
    names: BTreeMap<String, ProfileId>,
}

impl<'de> Deserialize<'de> for ProfileRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProfileRegistryWire::deserialize(deserializer)?;

        let registry = Self {
            profiles: wire.profiles,
            names: wire.names,
        };

        registry.validate().map_err(de::Error::custom)?;

        Ok(registry)
    }
}

impl ProfileRegistry {
    /// Construct an empty profile registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered profiles.
    #[must_use]
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    /// Return the authoritative profile map.
    #[must_use]
    pub fn profiles(
        &self,
    ) -> &BTreeMap<ProfileId, ProfileDefinition> {
        &self.profiles
    }

    /// Retrieve a profile by stable identity.
    #[must_use]
    pub fn get(&self, id: &ProfileId) -> Option<&ProfileDefinition> {
        self.profiles.get(id)
    }

    /// Retrieve a profile by its exact, case-sensitive name.
    ///
    /// Lookup uses the secondary index rather than scanning every profile.
    #[must_use]
    pub fn get_by_name(
        &self,
        name: &str,
    ) -> Option<&ProfileDefinition> {
        self.names
            .get(name)
            .and_then(|id| self.profiles.get(id))
    }

    /// Register a profile or replace the definition with the same identity.
    ///
    /// Registration is transactional: validation and duplicate-name checks
    /// happen before either map is mutated.
    ///
    /// Replacing an existing identity may change its name. The old name
    /// index entry is removed and the new one is installed.
    ///
    /// A name already owned by a different profile identity is rejected.
    ///
    /// Returns the replaced definition, if one existed.
    pub fn register(
        &mut self,
        profile: ProfileDefinition,
    ) -> ToolchainResult<Option<ProfileDefinition>> {
        profile.validate()?;

        let id = profile.id().clone();
        let name = profile.name().as_str().to_owned();

        if let Some(existing_id) = self.names.get(&name) {
            if existing_id != &id {
                return Err(invalid_profile(format!(
                    "profile name `{name}` is already registered"
                )));
            }
        }

        if let Some(existing) = self.profiles.get(&id) {
            let old_name = existing.name().as_str();

            if old_name != name {
                // The new name is already known to be unclaimed.
                // Mutation begins only after all checks have passed.
            }
        }

        let previous = self.profiles.remove(&id);

        if let Some(previous_profile) = &previous {
            self.names.remove(previous_profile.name().as_str());
        }

        self.names.insert(name, id.clone());
        self.profiles.insert(id, profile);

        Ok(previous)
    }

    /// Remove a profile by identity.
    ///
    /// Removal is rejected while another registered profile refers to it
    /// as a parent. This prevents the registry from being left with dangling
    /// inheritance references.
    ///
    /// Returns the removed profile when successful.
    pub fn remove(
        &mut self,
        id: &ProfileId,
    ) -> ToolchainResult<Option<ProfileDefinition>> {
        if !self.profiles.contains_key(id) {
            return Ok(None);
        }

        if let Some(child) = self.profiles.values().find(|profile| {
            profile.parents().contains(id)
        }) {
            return Err(invalid_profile(format!(
                "cannot remove profile `{id}` because profile `{}` inherits from it",
                child.id()
            )));
        }

        let removed = self.profiles.remove(id);

        if let Some(profile) = &removed {
            self.names.remove(profile.name().as_str());
        }

        Ok(removed)
    }

    /// Remove all profiles and all name-index entries.
    pub fn clear(&mut self) {
        self.profiles.clear();
        self.names.clear();
    }

    /// Validate profile definitions, name-index integrity and inheritance.
    ///
    /// This validates the complete registry, including disabled profiles.
    /// A disabled profile must still have valid references and an acyclic
    /// inheritance graph.
    pub fn validate(&self) -> ToolchainResult<()> {
        let mut expected_names = BTreeMap::<String, ProfileId>::new();

        for (map_id, profile) in &self.profiles {
            profile.validate()?;

            if map_id != profile.id() {
                return Err(invalid_profile(
                    "profile registry key does not match its definition identity",
                ));
            }

            let name = profile.name().as_str().to_owned();

            if expected_names
                .insert(name.clone(), map_id.clone())
                .is_some()
            {
                return Err(invalid_profile(format!(
                    "duplicate profile name `{name}`"
                )));
            }
        }

        if expected_names != self.names {
            return Err(invalid_profile(
                "profile-name index does not match registered profile definitions",
            ));
        }

        self.validate_inheritance_graph()
    }

    /// Validate that every parent exists and that the graph is acyclic.
    ///
    /// Uses Kahn's topological algorithm. It is iterative and does not
    /// recurse according to inheritance depth.
    fn validate_inheritance_graph(&self) -> ToolchainResult<()> {
        let mut indegree = BTreeMap::<ProfileId, usize>::new();
        let mut children =
            BTreeMap::<ProfileId, Vec<ProfileId>>::new();

        for id in self.profiles.keys() {
            indegree.insert(id.clone(), 0);
            children.entry(id.clone()).or_default();
        }

        for (id, profile) in &self.profiles {
            for parent in profile.parents() {
                if !self.profiles.contains_key(parent) {
                    return Err(invalid_profile(format!(
                        "profile `{id}` references missing parent `{parent}`"
                    )));
                }

                let count = indegree.get_mut(id).ok_or_else(|| {
                    invalid_profile(
                        "profile inheritance graph is internally inconsistent",
                    )
                })?;

                *count = count.checked_add(1).ok_or_else(|| {
                    invalid_profile(
                        "profile inheritance edge count exceeds representable capacity",
                    )
                })?;

                children
                    .entry(parent.clone())
                    .or_default()
                    .push(id.clone());
            }
        }

        let mut ready = VecDeque::<ProfileId>::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.push_back(id.clone());
            }
        }

        let mut visited = 0usize;

        while let Some(parent) = ready.pop_front() {
            visited = visited.checked_add(1).ok_or_else(|| {
                invalid_profile(
                    "profile graph node count exceeds representable capacity",
                )
            })?;

            if let Some(dependents) = children.get(&parent) {
                for child in dependents {
                    let degree = indegree.get_mut(child).ok_or_else(|| {
                        invalid_profile(
                            "profile inheritance graph is internally inconsistent",
                        )
                    })?;

                    *degree = degree.checked_sub(1).ok_or_else(|| {
                        invalid_profile(
                            "profile inheritance graph contains inconsistent edge counts",
                        )
                    })?;

                    if *degree == 0 {
                        ready.push_back(child.clone());
                    }
                }
            }
        }

        if visited != self.profiles.len() {
            return Err(invalid_profile(
                "profile inheritance graph contains a cycle",
            ));
        }

        Ok(())
    }

    /// Resolve one profile and all of its ancestors.
    ///
    /// The registry is fully validated before resolution.
    ///
    /// Disabled profiles cannot be resolved. A profile also cannot inherit
    /// from a disabled parent.
    ///
    /// Parent configuration conflicts are rejected. A child profile may
    /// explicitly override any inherited section.
    pub fn resolve(
        &self,
        id: &ProfileId,
    ) -> ToolchainResult<ResolvedProfile> {
        self.validate()?;

        let selected = self.profiles.get(id).ok_or_else(|| {
            ToolchainError::invalid_configuration(format!(
                "profile `{id}` is not registered"
            ))
        })?;

        if !selected.is_enabled() {
            return Err(invalid_profile(format!(
                "profile `{id}` is disabled"
            )));
        }

        let lineage = self.resolve_lineage(id)?;

        let mut sections =
            BTreeMap::<ConfigSectionKey, serde_json::Value>::new();

        for lineage_id in &lineage {
            let profile = self.profiles.get(lineage_id).ok_or_else(|| {
                invalid_profile(
                    "resolved profile lineage references a missing profile",
                )
            })?;

            if !profile.is_enabled() {
                return Err(invalid_profile(format!(
                    "profile `{id}` inherits from disabled profile `{}`",
                    profile.id()
                )));
            }

            // Parent profiles are composed in deterministic lineage order.
            // If two inherited branches provide different values for the
            // same section, resolution fails rather than silently choosing
            // one based on registration order.
            //
            // A profile's own values are applied after its ancestors and
            // therefore explicitly override inherited values.
            let is_selected = lineage_id == id;

            for (key, value) in profile.sections() {
                if is_selected {
                    sections.insert(key.clone(), value.clone());
                    continue;
                }

                match sections.get(key) {
                    Some(existing) if existing != value => {
                        return Err(invalid_profile(format!(
                            "conflicting inherited values for section `{key}` while resolving profile `{id}`; define an explicit value in the child profile"
                        )));
                    }

                    Some(_) => {}

                    None => {
                        sections.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        Ok(ResolvedProfile {
            id: selected.id().clone(),
            name: selected.name().clone(),
            sections,
            lineage,
        })
    }

    /// Produce an ancestor-first lineage using iterative depth-first traversal.
    ///
    /// The registry has already been checked for cycles. The explicit stack
    /// avoids recursion and the visited set avoids repeated traversal of
    /// shared ancestors.
    fn resolve_lineage(
        &self,
        selected: &ProfileId,
    ) -> ToolchainResult<Vec<ProfileId>> {
        let mut visited = BTreeSet::<ProfileId>::new();
        let mut lineage = Vec::<ProfileId>::new();

        let mut stack = vec![(selected.clone(), false)];

        while let Some((current, expanded)) = stack.pop() {
            if expanded {
                if visited.insert(current.clone()) {
                    lineage.push(current);
                }

                continue;
            }

            if visited.contains(&current) {
                continue;
            }

            let profile = self.profiles.get(&current).ok_or_else(|| {
                invalid_profile(format!(
                    "profile `{current}` is missing during inheritance resolution"
                ))
            })?;

            stack.push((current.clone(), true));

            // Reverse iteration because the stack is last-in-first-out.
            // BTreeSet gives deterministic parent identity ordering.
            for parent in profile.parents().iter().rev() {
                if !visited.contains(parent) {
                    stack.push((parent.clone(), false));
                }
            }
        }

        if lineage.last() != Some(selected) {
            return Err(invalid_profile(
                "profile lineage did not resolve the selected profile last",
            ));
        }

        Ok(lineage)
    }

    /// Load a registry from the canonical toolchain configuration section.
    ///
    /// A missing section produces an empty registry.
    ///
    /// The section is validated during deserialization.
    pub fn load_from_config(
        config: &ToolchainConfig,
    ) -> ToolchainResult<Self> {
        let key = profile_registry_key()?;

        let registry = config
            .section_as::<Self>(&key)?
            .unwrap_or_default();

        registry.validate()?;

        Ok(registry)
    }

    /// Store the complete registry in the canonical toolchain configuration.
    ///
    /// The registry is validated before mutation.
    pub fn store_in_config(
        &self,
        config: &mut ToolchainConfig,
    ) -> ToolchainResult<()> {
        self.validate()?;

        let key = profile_registry_key()?;

        config.set_section(key, self)?;

        config.validate()
    }
}

fn profile_registry_key() -> ToolchainResult<ConfigSectionKey> {
    ConfigSectionKey::new(PROFILE_REGISTRY_SECTION)
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_profile(
    message: impl Into<String>,
) -> ToolchainError {
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
            IdNamespace::new("test").expect("valid namespace"),
            value,
        )
        .expect("valid profile identity")
    }

    fn section(name: &str) -> ConfigSectionKey {
        ConfigSectionKey::new(name).expect("valid section key")
    }

    fn profile(
        id: u128,
        name: &str,
    ) -> ProfileDefinition {
        ProfileDefinition::new(profile_id(id), name)
            .expect("valid profile")
    }

    #[test]
    fn creates_valid_profile() {
        let definition = profile(1, "development");

        assert_eq!(definition.name().as_str(), "development");
        assert!(definition.is_enabled());
        assert!(definition.parents().is_empty());
        assert!(definition.sections().is_empty());
    }

    #[test]
    fn rejects_invalid_profile_names() {
        assert!(ProfileName::new("").is_err());
        assert!(ProfileName::new(" leading").is_err());
        assert!(ProfileName::new("trailing ").is_err());
        assert!(ProfileName::new("1invalid").is_err());
        assert!(ProfileName::new("invalid name").is_err());
    }

    #[test]
    fn accepts_qualified_profile_names() {
        assert!(ProfileName::new("quantum.portable").is_ok());
        assert!(ProfileName::new("vendor::portable").is_ok());
        assert!(ProfileName::new("_internal").is_ok());
    }

    #[test]
    fn registry_indexes_profiles_by_identity_and_name() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        assert!(registry.get(&profile_id(1)).is_some());
        assert!(registry.get_by_name("development").is_some());
        assert!(registry.get_by_name("Development").is_none());
    }

    #[test]
    fn rejects_duplicate_profile_names() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("first registration succeeds");

        assert!(registry
            .register(profile(2, "development"))
            .is_err());

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn replaces_profile_and_updates_name_index() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        registry
            .register(profile(1, "developer"))
            .expect("replacement succeeds");

        assert!(registry.get_by_name("development").is_none());
        assert!(registry.get_by_name("developer").is_some());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn rejects_replacement_with_another_profiles_name() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        registry
            .register(profile(2, "production"))
            .expect("registration succeeds");

        let result = registry.register(profile(1, "production"));

        assert!(result.is_err());
        assert!(registry.get_by_name("development").is_some());
        assert!(registry.get_by_name("production").is_some());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn validates_name_index() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        assert!(registry.validate().is_ok());

        registry.names.clear();

        assert!(registry.validate().is_err());
    }

    #[test]
    fn rejects_missing_parent() {
        let mut child = profile(2, "child");
        child.add_parent(profile_id(1));

        let mut registry = ProfileRegistry::new();

        registry
            .register(child)
            .expect("local registration succeeds");

        assert!(registry.validate().is_err());
    }

    #[test]
    fn rejects_direct_self_inheritance() {
        let mut definition = profile(1, "self");

        definition.add_parent(profile_id(1));

        assert!(definition.validate().is_err());
    }

    #[test]
    fn rejects_inheritance_cycles() {
        let mut first = profile(1, "first");
        first.add_parent(profile_id(2));

        let mut second = profile(2, "second");
        second.add_parent(profile_id(1));

        let mut registry = ProfileRegistry::new();

        registry.register(first).expect("registration succeeds");
        registry.register(second).expect("registration succeeds");

        assert!(registry.validate().is_err());
    }

    #[test]
    fn resolves_parent_before_child() {
        let key = section("compiler.optimization");

        let mut parent = profile(1, "parent");
        parent.set_section(key.clone(), serde_json::json!("balanced"));

        let mut child = profile(2, "child");
        child.add_parent(profile_id(1));

        let mut registry = ProfileRegistry::new();

        registry.register(parent).expect("parent registration");
        registry.register(child).expect("child registration");

        let resolved = registry
            .resolve(&profile_id(2))
            .expect("resolution succeeds");

        assert_eq!(
            resolved.section(&key),
            Some(&serde_json::json!("balanced"))
        );

        assert_eq!(
            resolved.lineage(),
            &[profile_id(1), profile_id(2)]
        );
    }

    #[test]
    fn child_overrides_inherited_section() {
        let key = section("compiler.optimization");

        let mut parent = profile(1, "parent");
        parent.set_section(key.clone(), serde_json::json!("balanced"));

        let mut child = profile(2, "child");
        child.add_parent(profile_id(1));
        child.set_section(key.clone(), serde_json::json!("aggressive"));

        let mut registry = ProfileRegistry::new();

        registry.register(parent).expect("parent registration");
        registry.register(child).expect("child registration");

        let resolved = registry
            .resolve(&profile_id(2))
            .expect("resolution succeeds");

        assert_eq!(
            resolved.section(&key),
            Some(&serde_json::json!("aggressive"))
        );
    }

    #[test]
    fn rejects_conflicting_multiple_parent_sections() {
        let key = section("compiler.optimization");

        let mut first = profile(1, "first");
        first.set_section(key.clone(), serde_json::json!("balanced"));

        let mut second = profile(2, "second");
        second.set_section(key.clone(), serde_json::json!("aggressive"));

        let mut child = profile(3, "child");
        child.add_parent(profile_id(1));
        child.add_parent(profile_id(2));

        let mut registry = ProfileRegistry::new();

        registry.register(first).expect("first registration");
        registry.register(second).expect("second registration");
        registry.register(child).expect("child registration");

        assert!(registry.resolve(&profile_id(3)).is_err());
    }

    #[test]
    fn accepts_identical_multiple_parent_sections() {
        let key = section("compiler.optimization");
        let value = serde_json::json!("balanced");

        let mut first = profile(1, "first");
        first.set_section(key.clone(), value.clone());

        let mut second = profile(2, "second");
        second.set_section(key.clone(), value.clone());

        let mut child = profile(3, "child");
        child.add_parent(profile_id(1));
        child.add_parent(profile_id(2));

        let mut registry = ProfileRegistry::new();

        registry.register(first).expect("first registration");
        registry.register(second).expect("second registration");
        registry.register(child).expect("child registration");

        let resolved = registry
            .resolve(&profile_id(3))
            .expect("identical parent values compose");

        assert_eq!(resolved.section(&key), Some(&value));
    }

    #[test]
    fn rejects_resolution_of_disabled_profile() {
        let mut definition = profile(1, "disabled");
        definition.set_enabled(false);

        let mut registry = ProfileRegistry::new();

        registry
            .register(definition)
            .expect("registration succeeds");

        assert!(registry.resolve(&profile_id(1)).is_err());
    }

    #[test]
    fn rejects_enabled_child_with_disabled_parent() {
        let mut parent = profile(1, "parent");
        parent.set_enabled(false);

        let mut child = profile(2, "child");
        child.add_parent(profile_id(1));

        let mut registry = ProfileRegistry::new();

        registry.register(parent).expect("parent registration");
        registry.register(child).expect("child registration");

        assert!(registry.resolve(&profile_id(2)).is_err());
    }

    #[test]
    fn prevents_removing_referenced_parent() {
        let mut parent = profile(1, "parent");

        let mut child = profile(2, "child");
        child.add_parent(profile_id(1));

        let mut registry = ProfileRegistry::new();

        registry.register(parent.clone()).expect("parent registration");
        registry.register(child).expect("child registration");

        assert!(registry.remove(&profile_id(1)).is_err());

        parent.set_description(Some("still registered".to_owned()))
            .expect("valid description");

        assert!(registry.get(&profile_id(1)).is_some());
    }

    #[test]
    fn removes_profile_and_name_index() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        let removed = registry
            .remove(&profile_id(1))
            .expect("removal succeeds");

        assert!(removed.is_some());
        assert!(registry.get_by_name("development").is_none());
        assert!(registry.is_empty());
    }

    #[test]
    fn applies_resolved_profile_without_removing_unrelated_sections() {
        let profile_key = section("compiler.optimization");
        let unrelated_key = section("security.trust");

        let mut definition = profile(1, "production");

        definition.set_section(
            profile_key.clone(),
            serde_json::json!("balanced"),
        );

        let mut registry = ProfileRegistry::new();

        registry
            .register(definition)
            .expect("registration succeeds");

        let resolved = registry
            .resolve(&profile_id(1))
            .expect("resolution succeeds");

        let mut config = ToolchainConfig::new();

        config.set_section(
            unrelated_key.clone(),
            &serde_json::json!("preserve"),
        )
        .expect("configuration section is serializable");

        resolved
            .apply_to(&mut config)
            .expect("profile application succeeds");

        assert_eq!(config.profile(), Some(&profile_id(1)));

        assert_eq!(
            config.section(&profile_key),
            Some(&serde_json::json!("balanced"))
        );

        assert_eq!(
            config.section(&unrelated_key),
            Some(&serde_json::json!("preserve"))
        );
    }

    #[test]
    fn stores_and_loads_registry_from_toolchain_config() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        let mut config = ToolchainConfig::new();

        registry
            .store_in_config(&mut config)
            .expect("registry storage succeeds");

        let loaded = ProfileRegistry::load_from_config(&config)
            .expect("registry loading succeeds");

        assert_eq!(loaded, registry);
    }

    #[test]
    fn missing_registry_section_loads_as_empty() {
        let config = ToolchainConfig::new();

        let registry = ProfileRegistry::load_from_config(&config)
            .expect("missing section is an empty registry");

        assert!(registry.is_empty());
    }

    #[test]
    fn serializes_and_deserializes_registry_deterministically() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(2, "production"))
            .expect("registration succeeds");

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        let first = serde_json::to_string(&registry)
            .expect("serialization succeeds");

        let decoded: ProfileRegistry =
            serde_json::from_str(&first).expect("deserialization succeeds");

        let second = serde_json::to_string(&decoded)
            .expect("serialization succeeds");

        assert_eq!(first, second);
        assert_eq!(decoded, registry);
    }

    #[test]
    fn rejects_deserialized_registry_with_stale_name_index() {
        let json = serde_json::json!({
            "profiles": {},
            "names": {
                "stale": "test:1"
            }
        });

        let result = serde_json::from_value::<ProfileRegistry>(json);

        assert!(result.is_err());
    }

    #[test]
    fn resolves_deep_inheritance_iteratively() {
        let mut registry = ProfileRegistry::new();

        let mut previous: Option<ProfileId> = None;

        for value in 1..=512_u128 {
            let mut definition = profile(
                value,
                &format!("profile_{value}"),
            );

            if let Some(parent) = previous.take() {
                definition.add_parent(parent);
            }

            previous = Some(profile_id(value));

            registry
                .register(definition)
                .expect("profile registration succeeds");
        }

        let resolved = registry
            .resolve(&profile_id(512))
            .expect("deep inheritance resolves");

        assert_eq!(resolved.lineage().len(), 512);
        assert_eq!(resolved.lineage().first(), Some(&profile_id(1)));
        assert_eq!(resolved.lineage().last(), Some(&profile_id(512)));
    }

    #[test]
    fn clearing_registry_clears_both_indexes() {
        let mut registry = ProfileRegistry::new();

        registry
            .register(profile(1, "development"))
            .expect("registration succeeds");

        registry.clear();

        assert!(registry.is_empty());
        assert!(registry.get_by_name("development").is_none());
        assert!(registry.validate().is_ok());
    }
}
