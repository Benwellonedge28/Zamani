
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Configuration Policies
//!
//! Path: src/toolchain/configuration/policies.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines reusable, composable, extensible toolchain configuration
//! policies and deterministic resolution of their configuration properties.
//!
//! This module is the configuration-policy boundary of the Zamani toolchain.
//! It does not create another compiler planner, semantic policy evaluator,
//! security authorization engine, resource allocator, or runtime scheduler.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - PolicyId.
//! - PolicyRevision.
//! - PolicyPropertyKey.
//! - PolicyScope.
//! - PolicyDefinition.
//! - PolicySet.
//! - PolicyProperty.
//! - ResolvedPolicy.
//! - Policy validation.
//! - Policy composition.
//! - Deterministic configuration resolution.
//! - Policy-property provenance.
//! - ToolchainConfig integration.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Source-level policy parsing.
//! - AST or semantic policy interpretation.
//! - Requirement evaluation.
//! - Capability discovery.
//! - Preference scoring.
//! - Constraint solving.
//! - Hint interpretation.
//! - Feasibility assessment.
//! - Security authorization enforcement.
//! - Cryptographic operations.
//! - Compiler optimization planning.
//! - Compiler pass registration or execution.
//! - Hardware discovery.
//! - Target selection.
//! - Resource allocation.
//! - Physical placement or routing.
//! - Runtime scheduling.
//! - Backend generation.
//!
//! ## Authoritative integrations
//!
//! Toolchain configuration:
//!     crate::toolchain::configuration::config
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Compiler optimization planning:
//!     crate::compiler::optimization::planning
//!
//! Compiler optimization policies:
//!     crate::compiler::optimization::policies
//!
//! Source policy grammar:
//!     grammar/policies/policy.g4
//!
//! Policy requirement grammar:
//!     grammar/policies/requirements.g4
//!
//! Resource requirements:
//!     grammar/core/requirements.g4
//!
//! Resource constraints:
//!     grammar/core/constraints.g4
//!
//! Resource budgets:
//!     grammar/resources/budgets.g4
//!
//! Resource preferences:
//!     grammar/resources/preferences.g4
//!
//! Resource hints:
//!     grammar/resources/hints.g4
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Security authorization:
//!     Authoritative security and authorization services.
//!
//! ## Policy semantics
//!
//! A policy is declarative governing configuration.
//!
//! A policy property is opaque to this module. Its meaning is established
//! by the subsystem that owns and validates that property.
//!
//! This module resolves configuration values. It does not decide whether
//! an operation is permitted, a requirement is satisfied, a target is
//! supported, or a computation is feasible.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Policy identities, scopes, properties and collections are open-world.
//!
//! No universal ceilings are imposed on:
//!
//! - Policy count.
//! - Policy-property count.
//! - Namespace depth.
//! - Computational domains.
//! - Hardware targets.
//! - CPU, GPU, FPGA or QPU counts.
//! - Qubit counts.
//! - Memory or storage capacity.
//! - Register width.
//! - Tensor rank.
//! - Distributed participants.
//! - Build-task counts.
//!
//! Actual processing and storage are subject to available resources,
//! representational constraints and explicitly configured operational
//! budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Ordered maps are used for policy identities and properties.
//!
//! Resolution is independent of insertion order.
//!
//! Equal-priority conflicting values are rejected rather than selected
//! according to an accidental iteration order.
//!
//! ## Security
//!
//! This module performs no filesystem access, network access, environment
//! lookup, process execution, hardware discovery or authorization.
//!
//! Policy configuration must not contain credentials, private keys,
//! authentication tokens or other secrets.
//!
//! Consumers must validate policy properties before relying on them.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::toolchain::configuration::config::{
    ConfigSectionKey,
    ToolchainConfig,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

/// Configuration section containing toolchain policy definitions.
///
/// This is an extensible configuration namespace, not a closed list of
/// supported computing domains or policy implementations.
pub const TOOLCHAIN_POLICIES_SECTION: &str = "toolchain.policies";

/// Stable API version for this module.
pub const POLICY_CONFIGURATION_API_VERSION: u16 = 1;

// ============================================================================
// POLICY IDENTITY
// ============================================================================

/// Unique textual identity of a toolchain configuration policy.
///
/// Policy identities are open-world. Third-party providers and future
/// computing domains can introduce new identities without modifying
/// this module.
///
/// Examples:
///
/// - `build.reproducible`
/// - `dependencies.integrity`
/// - `security.trust`
/// - `execution.deterministic`
/// - `quantum.realization`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `future.domain.policy`
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PolicyId(String);

impl PolicyId {
    /// Construct and validate a policy identity.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identifier(&value, "policy identity")?;

        Ok(Self(value))
    }

    /// Return the canonical identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identity and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// POLICY REVISION
// ============================================================================

/// Monotonically identified revision of an individual policy.
///
/// A policy revision is independent of:
///
/// - Toolchain API versions.
/// - Manifest schema versions.
/// - Package-format versions.
/// - Compiler versions.
/// - Language versions.
/// - Provider API versions.
///
/// Revision zero is reserved for an unspecified revision and is rejected
/// when constructing a policy definition.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PolicyRevision(u64);

impl PolicyRevision {
    /// Construct a nonzero policy revision.
    pub fn new(value: u64) -> ToolchainResult<Self> {
        if value == 0 {
            return Err(invalid_policy(
                "policy revision must be greater than zero",
            ));
        }

        Ok(Self(value))
    }

    /// Return the numeric revision.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

// ============================================================================
// POLICY PROPERTY IDENTITIES
// ============================================================================

/// Validated, open-world identity for a policy property.
///
/// Property identities are namespaced and independent of any particular
/// computing domain.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PolicyPropertyKey(String);

impl PolicyPropertyKey {
    /// Construct and validate a property identity.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identifier(&value, "policy property")?;

        Ok(Self(value))
    }

    /// Return the canonical property identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyPropertyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// POLICY SCOPE
// ============================================================================

/// Scope in which a policy definition participates in resolution.
///
/// Target scopes are opaque identifiers. This module does not maintain
/// a target registry or interpret target semantics.
///
/// Global policies participate in every resolution.
///
/// Target-specific policies participate only when the requested target
/// exactly matches their scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "target", rename_all = "snake_case")]
pub enum PolicyScope {
    /// Policy applies to every resolution context.
    Global,

    /// Policy applies only to the exact supplied target identity.
    Target(String),
}

impl PolicyScope {
    /// Construct a global policy scope.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Construct a target-specific scope.
    pub fn target(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identifier(&value, "policy target scope")?;

        Ok(Self::Target(value))
    }

    /// Determine whether this scope applies to a resolution context.
    #[must_use]
    pub fn applies_to(&self, target: Option<&str>) -> bool {
        match self {
            Self::Global => true,
            Self::Target(expected) => target == Some(expected.as_str()),
        }
    }

    /// Return whether this is a target-specific scope.
    #[must_use]
    pub const fn is_target_specific(&self) -> bool {
        matches!(self, Self::Target(_))
    }

    /// Return the target identity when this is a target-specific scope.
    #[must_use]
    pub fn target_identity(&self) -> Option<&str> {
        match self {
            Self::Global => None,
            Self::Target(value) => Some(value),
        }
    }

    fn validate(&self) -> ToolchainResult<()> {
        if let Self::Target(value) = self {
            validate_identifier(value, "policy target scope")?;
        }

        Ok(())
    }
}

// ============================================================================
// POLICY DEFINITION
// ============================================================================

/// One declarative toolchain policy.
///
/// Properties are intentionally stored as JSON values. Their semantic
/// interpretation belongs to the registered consuming subsystem.
///
/// A definition is immutable from outside this module after construction.
/// To change a policy, construct and insert a new definition.
///
/// Policy priority is an explicit signed integer. It does not represent
/// a hardware resource count, machine limit or computational capacity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyDefinition {
    id: PolicyId,
    revision: PolicyRevision,
    priority: i64,
    enabled: bool,
    scope: PolicyScope,
    properties: BTreeMap<PolicyPropertyKey, serde_json::Value>,
}

impl PolicyDefinition {
    /// Construct a policy definition with no properties.
    pub fn new(
        id: PolicyId,
        revision: PolicyRevision,
        priority: i64,
        scope: PolicyScope,
    ) -> ToolchainResult<Self> {
        scope.validate()?;

        Ok(Self {
            id,
            revision,
            priority,
            enabled: true,
            scope,
            properties: BTreeMap::new(),
        })
    }

    /// Return the policy identity.
    #[must_use]
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Return the policy revision.
    #[must_use]
    pub const fn revision(&self) -> PolicyRevision {
        self.revision
    }

    /// Return the policy priority.
    #[must_use]
    pub const fn priority(&self) -> i64 {
        self.priority
    }

    /// Return whether this policy is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Return the policy scope.
    #[must_use]
    pub const fn scope(&self) -> &PolicyScope {
        &self.scope
    }

    /// Return all policy properties.
    #[must_use]
    pub fn properties(
        &self,
    ) -> &BTreeMap<PolicyPropertyKey, serde_json::Value> {
        &self.properties
    }

    /// Set the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set the policy priority.
    pub fn set_priority(&mut self, priority: i64) {
        self.priority = priority;
    }

    /// Replace the policy scope after validating it.
    pub fn set_scope(
        &mut self,
        scope: PolicyScope,
    ) -> ToolchainResult<()> {
        scope.validate()?;
        self.scope = scope;
        Ok(())
    }

    /// Insert or replace a policy property.
    ///
    /// Returns the previous value when replacing an existing property.
    pub fn set_property(
        &mut self,
        key: PolicyPropertyKey,
        value: serde_json::Value,
    ) -> Option<serde_json::Value> {
        self.properties.insert(key, value)
    }

    /// Remove a policy property.
    ///
    /// Returns the previous value if it existed.
    pub fn remove_property(
        &mut self,
        key: &PolicyPropertyKey,
    ) -> Option<serde_json::Value> {
        self.properties.remove(key)
    }

    /// Retrieve a property by its canonical identity.
    #[must_use]
    pub fn property(
        &self,
        key: &PolicyPropertyKey,
    ) -> Option<&serde_json::Value> {
        self.properties.get(key)
    }

    /// Validate the complete definition.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier(self.id.as_str(), "policy identity")?;
        PolicyRevision::new(self.revision.get())?;
        self.scope.validate()?;

        for key in self.properties.keys() {
            validate_identifier(key.as_str(), "policy property")?;
        }

        Ok(())
    }
}

// ============================================================================
// POLICY COLLECTION
// ============================================================================

/// Deterministic collection of toolchain policy definitions.
///
/// Policy identity is unique within this collection.
///
/// Policies are stored in identity order, not insertion order.
///
/// This collection does not contain a closed enumeration of policy kinds,
/// domains, targets, providers or computing technologies.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySet {
    policies: BTreeMap<PolicyId, PolicyDefinition>,
}

impl PolicySet {
    /// Construct an empty policy collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of policy definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }

    /// Return all policy definitions in deterministic identity order.
    #[must_use]
    pub fn policies(&self) -> &BTreeMap<PolicyId, PolicyDefinition> {
        &self.policies
    }

    /// Retrieve a policy by identity.
    #[must_use]
    pub fn get(&self, id: &PolicyId) -> Option<&PolicyDefinition> {
        self.policies.get(id)
    }

    /// Insert or replace a policy.
    ///
    /// The complete definition is validated before mutation.
    ///
    /// Returns the previous definition when replacing an existing identity.
    pub fn insert(
        &mut self,
        definition: PolicyDefinition,
    ) -> ToolchainResult<Option<PolicyDefinition>> {
        definition.validate()?;

        let id = definition.id().clone();

        Ok(self.policies.insert(id, definition))
    }

    /// Remove a policy by identity.
    ///
    /// Returns the removed definition when present.
    pub fn remove(
        &mut self,
        id: &PolicyId,
    ) -> Option<PolicyDefinition> {
        self.policies.remove(id)
    }

    /// Remove all policy definitions.
    pub fn clear(&mut self) {
        self.policies.clear();
    }

    /// Validate the complete collection.
    ///
    /// This method is intentionally called before resolution because
    /// deserialization can construct values without passing through
    /// the normal constructors.
    pub fn validate(&self) -> ToolchainResult<()> {
        for (key, definition) in &self.policies {
            definition.validate()?;

            if key != definition.id() {
                return Err(invalid_policy(
                    "policy map identity does not match its definition identity",
                ));
            }
        }

        Ok(())
    }

    /// Resolve policies for an optional target identity.
    ///
    /// Global policies always participate.
    ///
    /// Target-specific policies participate only for an exact target match.
    ///
    /// Disabled policies are ignored.
    ///
    /// For each property:
    ///
    /// 1. Higher priority takes precedence.
    /// 2. At equal priority, target-specific scope takes precedence over
    ///    global scope.
    /// 3. Equal-priority, equal-specificity conflicting values produce
    ///    an error.
    /// 4. Identical values are resolved deterministically by policy identity.
    ///
    /// This operation resolves configuration values only. It does not
    /// authorize an operation or establish target feasibility.
    pub fn resolve(
        &self,
        target: Option<&str>,
    ) -> ToolchainResult<ResolvedPolicy> {
        self.validate()?;

        if let Some(target) = target {
            validate_identifier(target, "resolution target")?;
        }

        let mut resolved =
            BTreeMap::<PolicyPropertyKey, PolicyProperty>::new();

        for definition in self.policies.values() {
            if !definition.is_enabled()
                || !definition.scope().applies_to(target)
            {
                continue;
            }

            for (key, value) in definition.properties() {
                let candidate = PolicyProperty {
                    value: value.clone(),
                    source: PolicyProvenance {
                        policy_id: definition.id().clone(),
                        revision: definition.revision(),
                        priority: definition.priority(),
                        scope: definition.scope().clone(),
                    },
                };

                match resolved.get(key) {
                    None => {
                        resolved.insert(key.clone(), candidate);
                    }

                    Some(current) => {
                        match compare_precedence(
                            &candidate.source,
                            &current.source,
                        ) {
                            Precedence::CandidateWins => {
                                resolved.insert(key.clone(), candidate);
                            }

                            Precedence::CurrentWins => {}

                            Precedence::Equal => {
                                if candidate.value != current.value {
                                    return Err(
                                        conflicting_policy_property(
                                            key,
                                            &candidate.source,
                                            &current.source,
                                        ),
                                    );
                                }

                                // The map is ordered by PolicyId.
                                // Keeping the existing entry therefore
                                // provides deterministic provenance when
                                // equivalent values have equal precedence.
                            }
                        }
                    }
                }
            }
        }

        Ok(ResolvedPolicy {
            target: target.map(str::to_owned),
            properties: resolved,
        })
    }

    /// Load policy definitions from the canonical ToolchainConfig.
    ///
    /// If the policy section is absent, an empty policy set is returned.
    ///
    /// The returned collection is validated before being exposed.
    pub fn from_config(
        config: &ToolchainConfig,
    ) -> ToolchainResult<Self> {
        let key = policy_section_key()?;

        let policies = config
            .section_as::<Self>(&key)?
            .unwrap_or_default();

        policies.validate()?;

        Ok(policies)
    }

    /// Store this policy collection in ToolchainConfig.
    ///
    /// The collection is validated before modifying the configuration.
    ///
    /// Existing unrelated configuration sections are preserved.
    pub fn store_in_config(
        &self,
        config: &mut ToolchainConfig,
    ) -> ToolchainResult<Option<serde_json::Value>> {
        self.validate()?;

        config.set_section(policy_section_key()?, self)
    }

    /// Return the canonical configuration section key.
    pub fn section_key() -> ToolchainResult<ConfigSectionKey> {
        policy_section_key()
    }
}

// ============================================================================
// POLICY PROVENANCE
// ============================================================================

/// Origin of a resolved policy property.
///
/// This records which policy supplied the effective configuration value.
/// It is not cryptographic provenance or proof of policy authorization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyProvenance {
    policy_id: PolicyId,
    revision: PolicyRevision,
    priority: i64,
    scope: PolicyScope,
}

impl PolicyProvenance {
    /// Return the originating policy identity.
    #[must_use]
    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    /// Return the originating policy revision.
    #[must_use]
    pub const fn revision(&self) -> PolicyRevision {
        self.revision
    }

    /// Return the priority used during resolution.
    #[must_use]
    pub const fn priority(&self) -> i64 {
        self.priority
    }

    /// Return the originating scope.
    #[must_use]
    pub const fn scope(&self) -> &PolicyScope {
        &self.scope
    }
}

/// One resolved policy property and its origin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyProperty {
    value: serde_json::Value,
    source: PolicyProvenance,
}

impl PolicyProperty {
    /// Return the resolved value.
    #[must_use]
    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }

    /// Return the property's provenance.
    #[must_use]
    pub const fn source(&self) -> &PolicyProvenance {
        &self.source
    }
}

// ============================================================================
// RESOLVED POLICY
// ============================================================================

/// Immutable result of resolving a policy collection.
///
/// A resolved policy is a configuration snapshot. It is not an authorization
/// decision, capability snapshot, feasibility report or resource allocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedPolicy {
    target: Option<String>,
    properties: BTreeMap<PolicyPropertyKey, PolicyProperty>,
}

impl ResolvedPolicy {
    /// Return the target identity used for resolution.
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Return all resolved properties in deterministic order.
    #[must_use]
    pub fn properties(
        &self,
    ) -> &BTreeMap<PolicyPropertyKey, PolicyProperty> {
        &self.properties
    }

    /// Retrieve one resolved property.
    #[must_use]
    pub fn get(
        &self,
        key: &PolicyPropertyKey,
    ) -> Option<&PolicyProperty> {
        self.properties.get(key)
    }

    /// Return the number of resolved properties.
    #[must_use]
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Return whether no properties were resolved.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Return a property value without its provenance.
    #[must_use]
    pub fn value(
        &self,
        key: &PolicyPropertyKey,
    ) -> Option<&serde_json::Value> {
        self.get(key).map(PolicyProperty::value)
    }
}

// ============================================================================
// PRECEDENCE
// ============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Precedence {
    CandidateWins,
    CurrentWins,
    Equal,
}

fn compare_precedence(
    candidate: &PolicyProvenance,
    current: &PolicyProvenance,
) -> Precedence {
    match candidate.priority.cmp(&current.priority) {
        std::cmp::Ordering::Greater => Precedence::CandidateWins,

        std::cmp::Ordering::Less => Precedence::CurrentWins,

        std::cmp::Ordering::Equal => {
            match candidate
                .scope
                .is_target_specific()
                .cmp(&current.scope.is_target_specific())
            {
                std::cmp::Ordering::Greater => Precedence::CandidateWins,

                std::cmp::Ordering::Less => Precedence::CurrentWins,

                std::cmp::Ordering::Equal => Precedence::Equal,
            }
        }
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validate an open-world policy identity or scope.
///
/// The grammar is deliberately independent of supported computational
/// domains and vendor names.
///
/// Allowed characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Period.
/// - Underscore.
/// - Hyphen.
/// - Colon.
/// - Forward slash.
///
/// Identifiers must begin with an ASCII letter or digit.
///
/// No arbitrary maximum identifier length is imposed.
fn validate_identifier(
    value: &str,
    description: &str,
) -> ToolchainResult<()> {
    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return Err(invalid_policy(format!(
            "{description} cannot be empty"
        )));
    };

    if !first.is_ascii_alphanumeric() {
        return Err(invalid_policy(format!(
            "{description} must begin with an ASCII letter or digit"
        )));
    }

    if value.trim() != value {
        return Err(invalid_policy(format!(
            "{description} cannot contain surrounding whitespace"
        )));
    }

    if !characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(
                character,
                '.' | '_' | '-' | ':' | '/'
            )
    }) {
        return Err(invalid_policy(format!(
            "{description} contains unsupported characters"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_policy(format!(
            "{description} cannot contain control characters"
        )));
    }

    Ok(())
}

fn policy_section_key() -> ToolchainResult<ConfigSectionKey> {
    ConfigSectionKey::new(TOOLCHAIN_POLICIES_SECTION)
}

fn invalid_policy(message: impl Into<String>) -> ToolchainError {
    ToolchainError::invalid_configuration(message)
        .with_operation("toolchain.configuration.policies")
}

fn conflicting_policy_property(
    key: &PolicyPropertyKey,
    candidate: &PolicyProvenance,
    current: &PolicyProvenance,
) -> ToolchainError {
    invalid_policy(format!(
        "conflicting values for policy property `{key}` \
         at equal precedence between policies `{}` and `{}`",
        candidate.policy_id,
        current.policy_id,
    ))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn policy_id(value: &str) -> PolicyId {
        PolicyId::new(value).expect("valid policy identity")
    }

    fn revision(value: u64) -> PolicyRevision {
        PolicyRevision::new(value).expect("valid revision")
    }

    fn property_key(value: &str) -> PolicyPropertyKey {
        PolicyPropertyKey::new(value).expect("valid property")
    }

    fn global_policy(
        id: &str,
        priority: i64,
        value: serde_json::Value,
    ) -> PolicyDefinition {
        let mut policy = PolicyDefinition::new(
            policy_id(id),
            revision(1),
            priority,
            PolicyScope::global(),
        )
        .expect("valid policy");

        policy.set_property(property_key("build.reproducible"), value);

        policy
    }

    // ------------------------------------------------------------------------
    // Identity validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_open_world_policy_identities() {
        for value in [
            "build.reproducible",
            "quantum::realization",
            "hdl.synthesis",
            "distributed/execution",
            "future.domain.policy",
        ] {
            assert!(PolicyId::new(value).is_ok());
        }
    }

    #[test]
    fn rejects_empty_policy_identity() {
        assert!(PolicyId::new("").is_err());
    }

    #[test]
    fn rejects_whitespace_policy_identity() {
        assert!(PolicyId::new(" build.policy").is_err());
        assert!(PolicyId::new("build.policy ").is_err());
    }

    #[test]
    fn rejects_invalid_policy_identity_characters() {
        assert!(PolicyId::new("build policy").is_err());
        assert!(PolicyId::new("build.policy!").is_err());
    }

    #[test]
    fn rejects_zero_revision() {
        assert!(PolicyRevision::new(0).is_err());
    }

    // ------------------------------------------------------------------------
    // Scope
    // ------------------------------------------------------------------------

    #[test]
    fn global_scope_applies_to_every_target() {
        assert!(PolicyScope::global().applies_to(None));
        assert!(PolicyScope::global().applies_to(Some("target.a")));
        assert!(PolicyScope::global().applies_to(Some("target.b")));
    }

    #[test]
    fn target_scope_requires_exact_match() {
        let scope =
            PolicyScope::target("target.a").expect("valid scope");

        assert!(scope.applies_to(Some("target.a")));
        assert!(!scope.applies_to(Some("target.b")));
        assert!(!scope.applies_to(None));
    }

    // ------------------------------------------------------------------------
    // Policy collections
    // ------------------------------------------------------------------------

    #[test]
    fn inserts_and_retrieves_policies() {
        let mut set = PolicySet::new();

        let definition = global_policy(
            "build.reproducible",
            10,
            serde_json::json!(true),
        );

        set.insert(definition).expect("insert");

        assert_eq!(set.len(), 1);

        assert!(set.get(&policy_id("build.reproducible")).is_some());
    }

    #[test]
    fn replaces_existing_policy_identity_explicitly() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.reproducible",
            10,
            serde_json::json!(true),
        ))
        .expect("first insertion");

        let previous = set
            .insert(global_policy(
                "build.reproducible",
                20,
                serde_json::json!(false),
            ))
            .expect("replacement");

        assert!(previous.is_some());
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn removal_preserves_other_policies() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.first",
            1,
            serde_json::json!(true),
        ))
        .expect("insert");

        set.insert(global_policy(
            "build.second",
            2,
            serde_json::json!(false),
        ))
        .expect("insert");

        assert!(set.remove(&policy_id("build.first")).is_some());
        assert_eq!(set.len(), 1);
    }

    // ------------------------------------------------------------------------
    // Precedence
    // ------------------------------------------------------------------------

    #[test]
    fn higher_priority_wins() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.first",
            10,
            serde_json::json!(false),
        ))
        .expect("insert");

        set.insert(global_policy(
            "build.second",
            20,
            serde_json::json!(true),
        ))
        .expect("insert");

        let resolved = set.resolve(None).expect("resolve");

        assert_eq!(
            resolved.value(&property_key("build.reproducible")),
            Some(&serde_json::json!(true)),
        );
    }

    #[test]
    fn target_specific_scope_wins_at_equal_priority() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.global",
            10,
            serde_json::json!(false),
        ))
        .expect("insert");

        let mut specific = PolicyDefinition::new(
            policy_id("build.target"),
            revision(1),
            10,
            PolicyScope::target("target.a").expect("scope"),
        )
        .expect("policy");

        specific.set_property(
            property_key("build.reproducible"),
            serde_json::json!(true),
        );

        set.insert(specific).expect("insert");

        let resolved = set
            .resolve(Some("target.a"))
            .expect("resolve");

        assert_eq!(
            resolved.value(&property_key("build.reproducible")),
            Some(&serde_json::json!(true)),
        );
    }

    #[test]
    fn unrelated_target_policy_does_not_participate() {
        let mut set = PolicySet::new();

        let mut specific = PolicyDefinition::new(
            policy_id("build.target"),
            revision(1),
            100,
            PolicyScope::target("target.a").expect("scope"),
        )
        .expect("policy");

        specific.set_property(
            property_key("build.reproducible"),
            serde_json::json!(true),
        );

        set.insert(specific).expect("insert");

        assert!(
            set.resolve(Some("target.b"))
                .expect("resolve")
                .is_empty()
        );
    }

    #[test]
    fn conflicting_equal_precedence_values_are_rejected() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.first",
            10,
            serde_json::json!(true),
        ))
        .expect("insert");

        set.insert(global_policy(
            "build.second",
            10,
            serde_json::json!(false),
        ))
        .expect("insert");

        assert!(set.resolve(None).is_err());
    }

    #[test]
    fn identical_equal_precedence_values_are_deterministic() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.first",
            10,
            serde_json::json!(true),
        ))
        .expect("insert");

        set.insert(global_policy(
            "build.second",
            10,
            serde_json::json!(true),
        ))
        .expect("insert");

        let first = set.resolve(None).expect("resolve");

        let second = set.resolve(None).expect("resolve");

        assert_eq!(first, second);

        assert_eq!(
            first
                .get(&property_key("build.reproducible"))
                .expect("property")
                .source()
                .policy_id(),
            &policy_id("build.first"),
        );
    }

    #[test]
    fn disabled_policies_are_ignored() {
        let mut set = PolicySet::new();

        let mut definition = global_policy(
            "build.disabled",
            100,
            serde_json::json!(true),
        );

        definition.set_enabled(false);

        set.insert(definition).expect("insert");

        assert!(set.resolve(None).expect("resolve").is_empty());
    }

    // ------------------------------------------------------------------------
    // Provenance
    // ------------------------------------------------------------------------

    #[test]
    fn resolved_property_preserves_provenance() {
        let mut set = PolicySet::new();

        set.insert(global_policy(
            "build.reproducible",
            30,
            serde_json::json!(true),
        ))
        .expect("insert");

        let resolved = set.resolve(None).expect("resolve");

        let property = resolved
            .get(&property_key("build.reproducible"))
            .expect("property");

        assert_eq!(
            property.source().policy_id(),
            &policy_id("build.reproducible"),
        );

        assert_eq!(property.source().priority(), 30);
        assert_eq!(property.source().revision().get(), 1);
    }

    // ------------------------------------------------------------------------
    // Configuration integration
    // ------------------------------------------------------------------------

    #[test]
    fn stores_and_loads_policy_set_from_toolchain_config() {
        let mut config = ToolchainConfig::new();

        let mut policies = PolicySet::new();

        policies
            .insert(global_policy(
                "build.reproducible",
                10,
                serde_json::json!(true),
            ))
            .expect("insert");

        policies
            .store_in_config(&mut config)
            .expect("store");

        let loaded =
            PolicySet::from_config(&config).expect("load");

        assert_eq!(loaded, policies);
    }

    #[test]
    fn absent_policy_configuration_produces_empty_set() {
        let config = ToolchainConfig::new();

        let policies =
            PolicySet::from_config(&config).expect("load");

        assert!(policies.is_empty());
    }

    #[test]
    fn unrelated_configuration_sections_are_preserved() {
        let mut config = ToolchainConfig::new();

        let other_key =
            ConfigSectionKey::new("compiler.optimization")
                .expect("key");

        config
            .set_section(&other_key, &serde_json::json!({"enabled": true}))
            .expect("store");

        let policies = PolicySet::new();

        policies
            .store_in_config(&mut config)
            .expect("store");

        assert!(config.contains_section(&other_key));
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn policy_set_serialization_round_trips() {
        let mut original = PolicySet::new();

        original
            .insert(global_policy(
                "build.reproducible",
                10,
                serde_json::json!(true),
            ))
            .expect("insert");

        let encoded =
            serde_json::to_string(&original).expect("serialize");

        let decoded: PolicySet =
            serde_json::from_str(&encoded).expect("deserialize");

        decoded.validate().expect("validate");

        assert_eq!(original, decoded);
    }

    #[test]
    fn deserialized_invalid_policy_is_rejected_before_resolution() {
        let encoded = r#"
        {
            "policies": {
                "": {
                    "id": "",
                    "revision": 0,
                    "priority": 10,
                    "enabled": true,
                    "scope": {
                        "kind": "global"
                    },
                    "properties": {}
                }
            }
        }
        "#;

        let decoded: PolicySet =
            serde_json::from_str(encoded).expect("structural decoding");

        assert!(decoded.resolve(None).is_err());
    }

    // ------------------------------------------------------------------------
    // Open-world behavior
    // ------------------------------------------------------------------------

    #[test]
    fn supports_future_domain_properties_without_code_changes() {
        let mut set = PolicySet::new();

        let mut definition = PolicyDefinition::new(
            policy_id("future.domain.policy"),
            revision(1),
            10,
            PolicyScope::global(),
        )
        .expect("policy");

        definition.set_property(
            property_key("future.domain.new_capability"),
            serde_json::json!({
                "required": true,
                "version": "1.0"
            }),
        );

        set.insert(definition).expect("insert");

        let resolved = set.resolve(None).expect("resolve");

        assert!(
            resolved
                .get(&property_key("future.domain.new_capability"))
                .is_some()
        );
    }
}
