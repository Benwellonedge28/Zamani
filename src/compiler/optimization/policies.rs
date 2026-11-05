#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization Policies.
//!
//! Path:
//!     src/compiler/optimization/policies.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! Define reusable, composable, extensible optimization policies for
//! the Zamani compiler.
//!
//! This module resolves policy configuration into the existing
//! `OptimizationPlanningPolicy` defined in `planning.rs`.
//!
//! It does not implement a second optimization planner.
//!
//! # Architectural authorities
//!
//! - `planning.rs`: canonical optimization planning.
//! - `crate::zuir::pass::contracts`: canonical pass contracts.
//! - `crate::zuir::pass::registry`: canonical pass registration.
//! - `crate::zuir::pass::manager`: canonical pass execution.
//! - `crate::compiler::passes`: compiler pass integration.
//! - `crate::optimizer`: existing optimization subsystem.
//! - `grammar/policies/policy.g4`: source-level policy syntax.
//! - `grammar/ai/optimization.g4`: AI optimization intent.
//!
//! # Ownership
//!
//! This module owns:
//! - Compiler optimization policy definitions.
//! - Policy identity and version.
//! - Policy precedence.
//! - Policy composition.
//! - Policy resolution.
//! - Extensible policy properties.
//! - Conversion into the existing planning policy.
//!
//! It does not own:
//! - Source-language policy semantics.
//! - Semantic analysis.
//! - Transformation legality.
//! - Pass registration or execution.
//! - Cost estimation.
//! - IR verification.
//! - Hardware discovery or selection.
//! - Resource allocation.
//! - Runtime scheduling.
//!
//! # Policy distinctions
//!
//! Requirements, constraints, preferences, hints and capabilities
//! remain distinct semantic concepts.
//!
//! This module only resolves compiler execution configuration.
//!
//! # Scalability
//!
//! No artificial universal limits are imposed on:
//! - Policy count.
//! - Policy properties.
//! - Resource dimensions.
//! - Pass count.
//! - Computing domains.
//! - Hardware targets.
//!
//! Resource budgets are optional operational restrictions.
//! They are not language-level restrictions.
//!
//! # Safety
//!
//! Safe Rust only. Unsafe Rust is forbidden.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::planning::{
    OptimizationPlanningContext,
    OptimizationPlanningPolicy,
    PlanningResult,
    ResourceId,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Stable public optimization-policy API version.
pub const OPTIMIZATION_POLICIES_API_VERSION: u16 = 1;

// ============================================================================
// COMMON RESULT
// ============================================================================

/// Result type for optimization-policy operations.
pub type PolicyResult<T> = Result<T, PolicyError>;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable, extensible optimization-policy identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    /// Creates a policy identity.
    ///
    /// Empty identifiers and control characters are rejected.
    /// No arbitrary identifier-length limit is imposed.
    pub fn new(value: impl Into<String>) -> PolicyResult<Self> {
        let value = value.into();

        validate_identifier(&value, "policy")?;

        Ok(Self(value))
    }

    /// Returns the policy identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Version of an optimization policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyVersion {
    /// Major version.
    pub major: u64,

    /// Minor version.
    pub minor: u64,

    /// Patch version.
    pub patch: u64,
}

impl PolicyVersion {
    /// Creates a policy version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Default for PolicyVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

// ============================================================================
// POLICY VALUES
// ============================================================================

/// Extensible value used by additional policy properties.
///
/// These values are configuration data. Their meaning is defined by
/// the subsystem that registers and consumes the corresponding key.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyValue {
    /// Boolean value.
    Boolean(bool),

    /// Signed integer value.
    Integer(i128),

    /// Unsigned integer value.
    Unsigned(u128),

    /// Floating-point value.
    ///
    /// NaN and infinity are rejected during policy validation.
    Number(f64),

    /// Text value.
    Text(String),

    /// Sequence of policy values.
    Sequence(Vec<PolicyValue>),
}

impl PolicyValue {
    /// Validates this value and all nested values.
    pub fn validate(&self) -> PolicyResult<()> {
        match self {
            Self::Boolean(_) => Ok(()),

            Self::Integer(_) => Ok(()),

            Self::Unsigned(_) => Ok(()),

            Self::Number(value) => {
                if !value.is_finite() {
                    return Err(PolicyError::InvalidPolicyValue {
                        reason: "numeric policy values must be finite".to_owned(),
                    });
                }

                Ok(())
            }

            Self::Text(value) => {
                if value.chars().any(char::is_control) {
                    return Err(PolicyError::InvalidPolicyValue {
                        reason: "policy text contains control characters"
                            .to_owned(),
                    });
                }

                Ok(())
            }

            Self::Sequence(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// POLICY PATCH
// ============================================================================

/// Partial optimization-policy configuration.
///
/// An absent boolean leaves the previously resolved value unchanged.
///
/// Resource entries use `Some(value)` to establish a budget and `None`
/// to remove an inherited budget for that resource dimension.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PolicyPatch {
    /// Whether nondeterministic transformations are permitted.
    pub allow_nondeterministic: Option<bool>,

    /// Whether stochastic transformations are permitted.
    pub allow_stochastic: Option<bool>,

    /// Whether approximate transformations are permitted.
    pub allow_approximate: Option<bool>,

    /// Whether complete effect information is mandatory.
    pub require_complete_effects: Option<bool>,

    /// Whether preservation evidence is mandatory.
    pub require_preservation_evidence: Option<bool>,

    /// Whether verified input is mandatory.
    pub require_verified_input: Option<bool>,

    /// Whether verified output is mandatory.
    pub require_verified_output: Option<bool>,

    /// Whether every candidate needs a cost estimate.
    pub require_cost_estimate: Option<bool>,

    /// Whether the resulting optimization plan must be reproducible.
    pub require_reproducible_plan: Option<bool>,

    /// Resource budgets.
    ///
    /// A missing key leaves the inherited budget unchanged.
    /// `Some(value)` sets or replaces the budget.
    /// `None` removes an inherited budget.
    pub resource_budgets: BTreeMap<ResourceId, Option<u128>>,

    /// Additional extensible policy properties.
    ///
    /// A value replaces an inherited value.
    /// `None` removes an inherited property.
    pub extensions: BTreeMap<String, Option<PolicyValue>>,
}

impl PolicyPatch {
    /// Validates the policy patch.
    pub fn validate(&self) -> PolicyResult<()> {
        for key in self.extensions.keys() {
            validate_identifier(key, "policy extension")?;
        }

        for value in self.extensions.values().flatten() {
            value.validate()?;
        }

        Ok(())
    }

    /// Returns whether the patch contains no overrides.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.allow_nondeterministic.is_none()
            && self.allow_stochastic.is_none()
            && self.allow_approximate.is_none()
            && self.require_complete_effects.is_none()
            && self.require_preservation_evidence.is_none()
            && self.require_verified_input.is_none()
            && self.require_verified_output.is_none()
            && self.require_cost_estimate.is_none()
            && self.require_reproducible_plan.is_none()
            && self.resource_budgets.is_empty()
            && self.extensions.is_empty()
    }
}

// ============================================================================
// POLICY DEFINITION
// ============================================================================

/// One independently identifiable optimization policy.
///
/// Precedence determines composition order:
///
/// - Lower precedence is applied first.
/// - Higher precedence is applied later.
/// - When precedence is equal, policy IDs provide deterministic ordering.
///
/// This makes composition reproducible without relying on registration
/// order or hash-map iteration.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPolicy {
    /// Stable policy identity.
    pub id: PolicyId,

    /// Policy version.
    pub version: PolicyVersion,

    /// Policy precedence.
    pub precedence: i64,

    /// Policy overrides.
    pub patch: PolicyPatch,

    /// Human-readable policy description.
    pub description: String,
}

impl OptimizationPolicy {
    /// Creates an optimization policy.
    pub fn new(
        id: PolicyId,
        precedence: i64,
    ) -> Self {
        Self {
            id,
            version: PolicyVersion::default(),
            precedence,
            patch: PolicyPatch::default(),
            description: String::new(),
        }
    }

    /// Validates this policy.
    pub fn validate(&self) -> PolicyResult<()> {
        validate_identifier(self.id.as_str(), "policy")?;

        if self.description.chars().any(char::is_control) {
            return Err(PolicyError::InvalidPolicyValue {
                reason: "policy description contains control characters"
                    .to_owned(),
            });
        }

        self.patch.validate()
    }
}

// ============================================================================
// POLICY COLLECTION
// ============================================================================

/// Collection of independently registered optimization policies.
///
/// Policy identity must be unique within this collection.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptimizationPolicySet {
    policies: BTreeMap<PolicyId, OptimizationPolicy>,
}

impl OptimizationPolicySet {
    /// Creates an empty policy collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a policy.
    ///
    /// Duplicate identities are rejected rather than silently replaced.
    pub fn insert(
        &mut self,
        policy: OptimizationPolicy,
    ) -> PolicyResult<()> {
        policy.validate()?;

        let id = policy.id.clone();

        if self.policies.contains_key(&id) {
            return Err(PolicyError::DuplicatePolicy {
                id: id.to_string(),
            });
        }

        self.policies.insert(id, policy);

        Ok(())
    }

    /// Removes a policy by identity.
    pub fn remove(
        &mut self,
        id: &PolicyId,
    ) -> Option<OptimizationPolicy> {
        self.policies.remove(id)
    }

    /// Returns a policy by identity.
    #[must_use]
    pub fn get(&self, id: &PolicyId) -> Option<&OptimizationPolicy> {
        self.policies.get(id)
    }

    /// Returns the number of registered policies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }

    /// Iterates over policies in identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&PolicyId, &OptimizationPolicy)> {
        self.policies.iter()
    }

    /// Resolves the complete collection into one effective policy.
    ///
    /// Resolution order:
    ///
    /// 1. Existing conservative planning defaults.
    /// 2. Policies in ascending precedence order.
    /// 3. Policy identity as deterministic tie-breaker.
    ///
    /// A later policy overrides only fields explicitly supplied by
    /// its patch.
    pub fn resolve(&self) -> PolicyResult<ResolvedOptimizationPolicy> {
        let mut policies: Vec<&OptimizationPolicy> =
            self.policies.values().collect();

        policies.sort_by(|left, right| {
            left.precedence
                .cmp(&right.precedence)
                .then_with(|| left.id.cmp(&right.id))
        });

        let mut resolved = ResolvedOptimizationPolicy::default();

        for policy in policies {
            policy.validate()?;

            resolved.apply_patch(&policy.patch);

            resolved.applied_policies.push(policy.id.clone());
        }

        resolved.validate()?;

        Ok(resolved)
    }
}

// ============================================================================
// RESOLVED POLICY
// ============================================================================

/// Fully resolved optimization policy.
///
/// The `planning` field is the existing canonical planning-policy type.
/// This module does not introduce a replacement planning policy.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedOptimizationPolicy {
    /// Canonical planning policy.
    pub planning: OptimizationPlanningPolicy,

    /// Whether the final optimization plan must be reproducible.
    pub require_reproducible_plan: bool,

    /// Additional registered policy properties.
    pub extensions: BTreeMap<String, PolicyValue>,

    /// Policies applied during resolution, in application order.
    pub applied_policies: Vec<PolicyId>,
}

impl Default for ResolvedOptimizationPolicy {
    fn default() -> Self {
        Self {
            planning: OptimizationPlanningPolicy::default(),
            require_reproducible_plan: true,
            extensions: BTreeMap::new(),
            applied_policies: Vec::new(),
        }
    }
}

impl ResolvedOptimizationPolicy {
    /// Applies one policy patch.
    fn apply_patch(&mut self, patch: &PolicyPatch) {
        apply_bool(
            &mut self.planning.allow_nondeterministic,
            patch.allow_nondeterministic,
        );

        apply_bool(
            &mut self.planning.allow_stochastic,
            patch.allow_stochastic,
        );

        apply_bool(
            &mut self.planning.allow_approximate,
            patch.allow_approximate,
        );

        apply_bool(
            &mut self.planning.require_complete_effects,
            patch.require_complete_effects,
        );

        apply_bool(
            &mut self.planning.require_preservation_evidence,
            patch.require_preservation_evidence,
        );

        apply_bool(
            &mut self.planning.require_verified_input,
            patch.require_verified_input,
        );

        apply_bool(
            &mut self.planning.require_verified_output,
            patch.require_verified_output,
        );

        apply_bool(
            &mut self.planning.require_cost_estimate,
            patch.require_cost_estimate,
        );

        apply_bool(
            &mut self.require_reproducible_plan,
            patch.require_reproducible_plan,
        );

        for (resource, budget) in &patch.resource_budgets {
            match budget {
                Some(value) => {
                    self.planning
                        .resource_budgets
                        .insert(resource.clone(), *value);
                }

                None => {
                    self.planning.resource_budgets.remove(resource);
                }
            }
        }

        for (key, value) in &patch.extensions {
            match value {
                Some(value) => {
                    self.extensions.insert(key.clone(), value.clone());
                }

                None => {
                    self.extensions.remove(key);
                }
            }
        }
    }

    /// Validates the resolved policy.
    pub fn validate(&self) -> PolicyResult<()> {
        for (key, value) in &self.extensions {
            validate_identifier(key, "policy extension")?;
            value.validate()?;
        }

        Ok(())
    }

    /// Applies this resolved policy to an existing planning context.
    ///
    /// This updates the existing planning-policy field and reproducibility
    /// requirement. It does not modify candidates, IR, semantic information
    /// or registered passes.
    pub fn apply_to_context(
        &self,
        context: &mut OptimizationPlanningContext,
    ) -> PolicyResult<()> {
        self.validate()?;

        context.policy = self.planning.clone();
        context.require_reproducible_plan =
            self.require_reproducible_plan;

        Ok(())
    }

    /// Returns the canonical planning policy.
    #[must_use]
    pub fn planning_policy(&self) -> &OptimizationPlanningPolicy {
        &self.planning
    }

    /// Returns an extension property.
    #[must_use]
    pub fn extension(&self, key: &str) -> Option<&PolicyValue> {
        self.extensions.get(key)
    }
}

// ============================================================================
// HELPERS
// ============================================================================

fn apply_bool(target: &mut bool, override_value: Option<bool>) {
    if let Some(value) = override_value {
        *target = value;
    }
}

fn validate_identifier(
    value: &str,
    kind: &str,
) -> PolicyResult<()> {
    if value.trim().is_empty() {
        return Err(PolicyError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PolicyError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Optimization-policy errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    /// Invalid policy identifier.
    InvalidIdentifier {
        /// Identifier category.
        kind: String,
    },

    /// Duplicate policy identity.
    DuplicatePolicy {
        /// Duplicate identity.
        id: String,
    },

    /// Invalid policy value.
    InvalidPolicyValue {
        /// Explanation.
        reason: String,
    },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind } => {
                write!(formatter, "invalid {kind} identifier")
            }

            Self::DuplicatePolicy { id } => {
                write!(formatter, "duplicate optimization policy: {id}")
            }

            Self::InvalidPolicyValue { reason } => {
                write!(formatter, "invalid optimization policy value: {reason}")
            }
        }
    }
}

impl Error for PolicyError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn policy(
        id: &str,
        precedence: i64,
    ) -> OptimizationPolicy {
        OptimizationPolicy::new(
            PolicyId::new(id).expect("valid policy ID"),
            precedence,
        )
    }

    #[test]
    fn default_policy_is_conservative() {
        let resolved = OptimizationPolicySet::new()
            .resolve()
            .expect("empty policy set resolves");

        assert!(!resolved.planning.allow_nondeterministic);
        assert!(!resolved.planning.allow_stochastic);
        assert!(!resolved.planning.allow_approximate);

        assert!(resolved.planning.require_complete_effects);
        assert!(resolved.planning.require_preservation_evidence);
        assert!(resolved.planning.require_verified_input);
        assert!(resolved.planning.require_verified_output);

        assert!(resolved.require_reproducible_plan);
    }

    #[test]
    fn policies_resolve_by_precedence() {
        let mut policies = OptimizationPolicySet::new();

        let mut lower = policy("project.optimization", 10);
        lower.patch.allow_approximate = Some(true);

        let mut higher = policy("invocation.optimization", 20);
        higher.patch.allow_approximate = Some(false);

        policies.insert(higher).expect("insert higher policy");
        policies.insert(lower).expect("insert lower policy");

        let resolved = policies.resolve().expect("resolve policies");

        assert!(!resolved.planning.allow_approximate);

        assert_eq!(
            resolved.applied_policies,
            vec![
                PolicyId::new("project.optimization").unwrap(),
                PolicyId::new("invocation.optimization").unwrap(),
            ]
        );
    }

    #[test]
    fn equal_precedence_has_deterministic_identity_order() {
        let mut policies = OptimizationPolicySet::new();

        let mut alpha = policy("policy.alpha", 5);
        alpha.patch.allow_stochastic = Some(true);

        let mut beta = policy("policy.beta", 5);
        beta.patch.allow_stochastic = Some(false);

        policies.insert(beta).expect("insert beta");
        policies.insert(alpha).expect("insert alpha");

        let resolved = policies.resolve().expect("resolve policies");

        assert!(!resolved.planning.allow_stochastic);

        assert_eq!(
            resolved.applied_policies[0].as_str(),
            "policy.alpha"
        );

        assert_eq!(
            resolved.applied_policies[1].as_str(),
            "policy.beta"
        );
    }

    #[test]
    fn duplicate_policy_ids_are_rejected() {
        let mut policies = OptimizationPolicySet::new();

        policies
            .insert(policy("project.optimization", 1))
            .expect("first insertion succeeds");

        let result = policies.insert(policy("project.optimization", 2));

        assert!(matches!(
            result,
            Err(PolicyError::DuplicatePolicy { .. })
        ));
    }

    #[test]
    fn resource_budgets_can_be_added_and_removed() {
        let resource = ResourceId::new("memory.bytes")
            .expect("valid resource ID");

        let mut policies = OptimizationPolicySet::new();

        let mut first = policy("project.resources", 10);
        first.patch.resource_budgets.insert(
            resource.clone(),
            Some(4096),
        );

        let mut second = policy("invocation.resources", 20);
        second.patch.resource_budgets.insert(
            resource.clone(),
            None,
        );

        policies.insert(first).expect("insert first");
        policies.insert(second).expect("insert second");

        let resolved = policies.resolve().expect("resolve policies");

        assert!(
            !resolved
                .planning
                .resource_budgets
                .contains_key(&resource)
        );
    }

    #[test]
    fn extension_properties_are_extensible() {
        let mut policies = OptimizationPolicySet::new();

        let mut policy = policy("domain.extension", 10);

        policy.patch.extensions.insert(
            "quantum.error_model".to_owned(),
            Some(PolicyValue::Text("depolarizing".to_owned())),
        );

        policy.patch.extensions.insert(
            "tensor.precision".to_owned(),
            Some(PolicyValue::Text("adaptive".to_owned())),
        );

        policies.insert(policy).expect("insert policy");

        let resolved = policies.resolve().expect("resolve policy");

        assert_eq!(
            resolved.extension("quantum.error_model"),
            Some(&PolicyValue::Text("depolarizing".to_owned()))
        );

        assert_eq!(
            resolved.extension("tensor.precision"),
            Some(&PolicyValue::Text("adaptive".to_owned()))
        );
    }

    #[test]
    fn non_finite_extension_numbers_are_rejected() {
        let mut policies = OptimizationPolicySet::new();

        let mut policy = policy("invalid.numeric", 10);

        policy.patch.extensions.insert(
            "optimization.tolerance".to_owned(),
            Some(PolicyValue::Number(f64::NAN)),
        );

        assert!(matches!(
            policies.insert(policy),
            Err(PolicyError::InvalidPolicyValue { .. })
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        let result = PolicyId::new("invalid\npolicy");

        assert!(matches!(
            result,
            Err(PolicyError::InvalidIdentifier { .. })
        ));
    }
}