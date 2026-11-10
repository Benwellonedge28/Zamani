
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Requirement Infrastructure
//!
//! Path: `src/stdlib/requirement.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent identities and collections for mandatory
//! requirements used by standard-library APIs and integration boundaries.
//!
//! This module deliberately provides a small foundational contract. It does
//! not attempt to replace the canonical semantic requirement models in the
//! frontend.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - open-world requirement identities;
//! - validated requirement declarations;
//! - optional human-readable rationale;
//! - ordered requirement collections;
//! - deterministic collection views;
//! - duplicate declaration reporting.
//!
//! This module does NOT own:
//!
//! - source syntax or parsing;
//! - AST nodes or source spans;
//! - semantic name resolution;
//! - capability definitions or capability discovery;
//! - resource quantities, units, or resource expressions;
//! - preferences, constraints, or hints;
//! - requirement expression evaluation;
//! - satisfiability or feasibility analysis;
//! - target selection;
//! - hardware discovery or allocation;
//! - backend selection;
//! - scheduling, routing, or execution.
//!
//! Those responsibilities remain with their existing repository authorities.
//!
//! ## Integration boundaries
//!
//! Source grammar:
//!   `grammar/core/requirements.g4`
//!
//! Resource grammar:
//!   `grammar/resources/requirements.g4`
//!
//! Policy adapter:
//!   `grammar/policies/requirements.g4`
//!
//! Canonical capability semantics:
//!   `src/frontend/semantic/capabilities/model.rs`
//!
//! Canonical resource semantics:
//!   `src/frontend/semantic/resources/model.rs`
//!
//! Backend-specific compatibility requirements:
//!   `src/backend/requirements.rs`
//!
//! This module can be used by adapters that translate canonical semantic
//! requirements into standard-library requirement identities. Such adapters
//! must preserve the meaning and provenance of the original semantic values.
//!
//! ## Fundamental distinctions
//!
//! Requirement:
//!   A mandatory condition that a valid realization must satisfy.
//!
//! Capability:
//!   An ability that an implementation or environment can provide.
//!
//! Preference:
//!   A desirable property that is not mandatory.
//!
//! Constraint:
//!   A mandatory restriction on an otherwise valid realization.
//!
//! Hint:
//!   Non-binding guidance for downstream implementation.
//!
//! Feasibility:
//!   An evidence-based assessment of whether requirements can be satisfied.
//!
//! These concepts must not be silently converted into one another.
//!
//! ## POCO-REAF
//!
//! Requirement identities are open-world. New computational domains,
//! capabilities, resource categories, and future technologies do not require
//! changes to a closed Rust enum in this module.
//!
//! A requirement describes intent. It does not select a CPU, GPU, FPGA, QPU,
//! ASIC, device, provider, machine, node, topology, or execution location.
//!
//! This module imposes no language-level maximum on:
//!
//! - requirement count;
//! - namespace depth;
//! - requirement-name length;
//! - collection size;
//! - number of computational domains.
//!
//! Actual memory and processing requirements remain finite and depend on
//! available resources. Operational budgets belong to explicit compiler,
//! analysis-session, or runtime configuration; they are not semantic limits.
//!
//! ## Determinism
//!
//! Requirement identity is determined only by its namespace and name.
//! Requirement collections preserve insertion order.
//! Canonicalized views use deterministic key ordering.
//! Duplicate requirements are preserved rather than silently discarded.
//!
//! ## Error handling
//!
//! Constructors reject malformed identities and rationale text.
//! Ordinary invalid input returns a structured error rather than panicking.
//!
//! This module does not report a requirement as unsupported or infeasible.
//! Those are downstream compatibility and feasibility results.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

// ============================================================================
// Requirement identity
// ============================================================================

/// A stable, open-world identity for a requirement.
///
/// The namespace identifies the semantic area or extension that defines the
/// requirement. The name identifies the requirement within that namespace.
///
/// Examples:
///
/// - `quantum` + `dynamic_control`
/// - `classical` + `parallel_execution`
/// - `execution` + `deterministic`
/// - `security` + `trusted_execution`
/// - `distributed` + `collective_communication`
/// - `future.computing` + `new_execution_model`
///
/// Namespaces and names are not selected from a closed list. New domains and
/// extensions can introduce identities without modifying this type.
///
/// Identity components are preserved exactly. This type does not perform
/// name resolution, case folding, alias expansion, or normalization.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementKey {
    namespace: String,
    name: String,
}

impl RequirementKey {
    /// Constructs a validated requirement identity.
    ///
    /// Both components must be non-empty, must not contain leading or
    /// trailing whitespace, and must not contain control characters.
    ///
    /// No arbitrary maximum component length is imposed.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(
            &namespace,
            RequirementComponent::Namespace,
        )?;

        validate_component(
            &name,
            RequirementComponent::Name,
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Returns the namespace exactly as stored.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unqualified requirement name exactly as stored.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully qualified textual identity.
    ///
    /// This is intended for display, diagnostics, and interchange.
    /// Structured identity comparisons should use `RequirementKey` itself.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Checks identity without allocating a temporary key.
    #[must_use]
    pub fn matches(
        &self,
        namespace: &str,
        name: &str,
    ) -> bool {
        self.namespace == namespace && self.name == name
    }
}

impl fmt::Display for RequirementKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// Requirement declaration
// ============================================================================

/// A mandatory, named requirement declaration.
///
/// A `Requirement` identifies a condition that must be satisfied. It does not
/// contain a duplicate representation of the condition's domain-specific
/// semantics.
///
/// For example, the canonical semantic capability model owns capability
/// version constraints and typed capability properties. The canonical
/// semantic resource model owns resource quantities, units, and symbolic
/// resource expressions.
///
/// This type can identify such a requirement at a standard-library boundary,
/// but it does not replace those canonical models.
///
/// A requirement is mandatory by definition. Non-mandatory preferences,
/// permissions, advisory hints, and other policy roles must not be represented
/// by silently weakening a `Requirement`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Requirement {
    key: RequirementKey,
    rationale: Option<String>,
}

impl Requirement {
    /// Constructs a requirement with no additional rationale.
    #[must_use]
    pub fn new(key: RequirementKey) -> Self {
        Self {
            key,
            rationale: None,
        }
    }

    /// Constructs a requirement from namespace and name components.
    pub fn from_parts(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        Ok(Self::new(RequirementKey::new(
            namespace,
            name,
        )?))
    }

    /// Returns the requirement identity.
    #[must_use]
    pub fn key(&self) -> &RequirementKey {
        &self.key
    }

    /// Returns the optional rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Adds a non-empty human-readable rationale.
    ///
    /// Rationale is explanatory metadata. It must not be used to change the
    /// meaning of the requirement or to smuggle in target-selection policy.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let rationale = rationale.into();

        validate_rationale(&rationale)?;

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Returns the stable identity used for deterministic ordering.
    ///
    /// Rationale is intentionally excluded because it does not define
    /// requirement identity.
    #[must_use]
    pub fn canonical_key(&self) -> &RequirementKey {
        &self.key
    }
}

impl fmt::Display for Requirement {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.key.fmt(formatter)
    }
}

// ============================================================================
// Requirement collection
// ============================================================================

/// An ordered collection of mandatory requirements.
///
/// This collection intentionally preserves:
///
/// - insertion order;
/// - duplicate declarations;
/// - independently sourced declarations;
/// - potentially contradictory declarations.
///
/// It is not a solver, normalizer, satisfiability checker, or feasibility
/// engine. Removing duplicates or resolving contradictions here would destroy
/// information that downstream semantic and feasibility services may need.
///
/// There is no fixed collection capacity. Growth is governed by available
/// memory and any explicitly configured operational budget outside this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementSet {
    requirements: Vec<Requirement>,
}

impl Default for RequirementSet {
    fn default() -> Self {
        Self::new()
    }
}

impl RequirementSet {
    /// Creates an empty requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Creates a collection from an iterator.
    ///
    /// The iterator's order and duplicate entries are preserved.
    #[must_use]
    pub fn from_requirements(
        requirements: impl IntoIterator<Item = Requirement>,
    ) -> Self {
        Self {
            requirements: requirements.into_iter().collect(),
        }
    }

    /// Adds one validated requirement.
    pub fn push(
        &mut self,
        requirement: Requirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Adds a requirement and returns the updated collection.
    #[must_use]
    pub fn with(
        mut self,
        requirement: Requirement,
    ) -> Self {
        self.push(requirement);
        self
    }

    /// Appends another collection, preserving its order and duplicates.
    pub fn extend(
        &mut self,
        other: Self,
    ) {
        self.requirements.extend(other.requirements);
    }

    /// Appends requirements from an iterator.
    ///
    /// Existing entries are not deduplicated or reordered.
    pub fn extend_from(
        &mut self,
        requirements: impl IntoIterator<Item = Requirement>,
    ) {
        self.requirements.extend(requirements);
    }

    /// Returns the number of stored declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the collection contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns requirements in insertion order.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, Requirement> {
        self.requirements.iter()
    }

    /// Returns a requirement by insertion-order index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Requirement> {
        self.requirements.get(index)
    }

    /// Returns requirements belonging to a namespace.
    ///
    /// The returned iterator preserves insertion order.
    pub fn for_namespace<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = &'a Requirement> + 'a {
        self.requirements
            .iter()
            .filter(move |requirement| {
                requirement.key().namespace() == namespace
            })
    }

    /// Returns whether a requirement with the given identity is present.
    ///
    /// This checks declaration presence only. It does not establish that the
    /// requirement is satisfied.
    #[must_use]
    pub fn contains(
        &self,
        key: &RequirementKey,
    ) -> bool {
        self.requirements
            .iter()
            .any(|requirement| requirement.key() == key)
    }

    /// Removes all declarations.
    ///
    /// This is an explicit collection operation. It does not mutate any
    /// semantic model or source declaration.
    pub fn clear(&mut self) {
        self.requirements.clear();
    }

    /// Consumes the collection and returns its declarations in insertion
    /// order.
    #[must_use]
    pub fn into_vec(self) -> Vec<Requirement> {
        self.requirements
    }

    /// Returns a deterministically ordered view without mutating this
    /// collection.
    ///
    /// Duplicate declarations remain present. When two declarations have the
    /// same identity, their rationale is used as a deterministic tie-breaker.
    #[must_use]
    pub fn canonicalized(&self) -> Vec<&Requirement> {
        let mut result: Vec<&Requirement> =
            self.requirements.iter().collect();

        result.sort_by(|left, right| {
            left.key()
                .cmp(right.key())
                .then_with(|| {
                    left.rationale().cmp(&right.rationale())
                })
        });

        result
    }

    /// Reports groups of duplicate requirement identities.
    ///
    /// The returned indices refer to the original insertion order.
    ///
    /// Duplicate reporting does not imply that the declarations are
    /// contradictory or redundant. Those judgments belong to semantic
    /// analysis and feasibility services.
    #[must_use]
    pub fn duplicate_groups(
        &self,
    ) -> Vec<(RequirementKey, Vec<usize>)> {
        let mut groups: BTreeMap<
            RequirementKey,
            Vec<usize>,
        > = BTreeMap::new();

        for (index, requirement) in
            self.requirements.iter().enumerate()
        {
            groups
                .entry(requirement.key().clone())
                .or_default()
                .push(index);
        }

        groups
            .into_iter()
            .filter(|(_, indices)| indices.len() > 1)
            .collect()
    }
}

impl IntoIterator for RequirementSet {
    type Item = Requirement;
    type IntoIter = std::vec::IntoIter<Requirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.into_iter()
    }
}

impl<'a> IntoIterator for &'a RequirementSet {
    type Item = &'a Requirement;
    type IntoIter = core::slice::Iter<'a, Requirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.iter()
    }
}

// ============================================================================
// Structured errors
// ============================================================================

/// The component of a requirement identity that failed validation.
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
pub enum RequirementComponent {
    /// Requirement namespace.
    Namespace,

    /// Requirement name.
    Name,

    /// Human-readable rationale.
    Rationale,
}

impl fmt::Display for RequirementComponent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
            Self::Rationale => formatter.write_str("rationale"),
        }
    }
}

/// Errors raised while constructing requirement identities or declarations.
///
/// These errors concern malformed local data only. They do not indicate
/// unsupported capabilities, unavailable resources, infeasibility, or
/// authorization failure.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementError {
    /// A required text component was empty or whitespace-only.
    EmptyComponent {
        component: RequirementComponent,
    },

    /// A component contained leading or trailing whitespace.
    SurroundingWhitespace {
        component: RequirementComponent,
    },

    /// A component contained a control character.
    ControlCharacter {
        component: RequirementComponent,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(
                    formatter,
                    "requirement {component} must not be empty"
                )
            }

            Self::SurroundingWhitespace { component } => {
                write!(
                    formatter,
                    "requirement {component} must not have \
                     leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { component } => {
                write!(
                    formatter,
                    "requirement {component} must not contain \
                     control characters"
                )
            }
        }
    }
}

impl Error for RequirementError {}

// ============================================================================
// Internal validation
// ============================================================================

fn validate_component(
    value: &str,
    component: RequirementComponent,
) -> Result<(), RequirementError> {
    if value.trim().is_empty() {
        return Err(RequirementError::EmptyComponent {
            component,
        });
    }

    if value.trim() != value {
        return Err(RequirementError::SurroundingWhitespace {
            component,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(RequirementError::ControlCharacter {
            component,
        });
    }

    Ok(())
}

fn validate_rationale(
    value: &str,
) -> Result<(), RequirementError> {
    validate_component(
        value,
        RequirementComponent::Rationale,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(
        namespace: &str,
        name: &str,
    ) -> RequirementKey {
        RequirementKey::new(namespace, name)
            .expect("test requirement identity must be valid")
    }

    fn requirement(
        namespace: &str,
        name: &str,
    ) -> Requirement {
        Requirement::new(key(namespace, name))
    }

    #[test]
    fn constructs_namespaced_requirement_identity() {
        let value = key("quantum", "dynamic_control");

        assert_eq!(value.namespace(), "quantum");
        assert_eq!(value.name(), "dynamic_control");
        assert_eq!(
            value.qualified_name(),
            "quantum::dynamic_control"
        );
    }

    #[test]
    fn preserves_identity_without_normalization() {
        let value = key(
            "Future.Computing",
            "NewExecutionModel",
        );

        assert_eq!(
            value.namespace(),
            "Future.Computing"
        );
        assert_eq!(
            value.name(),
            "NewExecutionModel"
        );
    }

    #[test]
    fn accepts_new_and_future_namespaces() {
        for (namespace, name) in [
            ("classical", "parallel_execution"),
            ("quantum", "measurement"),
            ("hdl", "synthesis"),
            ("hybrid", "mixed_execution"),
            ("distributed", "collective_communication"),
            ("future.photonic", "optical_computation"),
            ("future.biological", "molecular_computation"),
            ("future", "new_domain"),
        ] {
            let value = key(namespace, name);

            assert_eq!(value.namespace(), namespace);
            assert_eq!(value.name(), name);
        }
    }

    #[test]
    fn rejects_empty_identity_components() {
        assert!(matches!(
            RequirementKey::new("", "execution"),
            Err(RequirementError::EmptyComponent {
                component: RequirementComponent::Namespace
            })
        ));

        assert!(matches!(
            RequirementKey::new("classical", ""),
            Err(RequirementError::EmptyComponent {
                component: RequirementComponent::Name
            })
        ));

        assert!(RequirementKey::new(" ", "execution").is_err());
        assert!(RequirementKey::new("classical", " ").is_err());
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert!(matches!(
            RequirementKey::new(" quantum", "execution"),
            Err(RequirementError::SurroundingWhitespace {
                component: RequirementComponent::Namespace
            })
        ));

        assert!(matches!(
            RequirementKey::new("classical", "execution "),
            Err(RequirementError::SurroundingWhitespace {
                component: RequirementComponent::Name
            })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        assert!(matches!(
            RequirementKey::new("classical\n", "execution"),
            Err(RequirementError::ControlCharacter {
                component: RequirementComponent::Namespace
            })
        ));

        assert!(matches!(
            RequirementKey::new("classical", "exec\u{0007}ution"),
            Err(RequirementError::ControlCharacter {
                component: RequirementComponent::Name
            })
        ));
    }

    #[test]
    fn identity_matching_does_not_allocate() {
        let value = key("distributed", "communication");

        assert!(value.matches(
            "distributed",
            "communication"
        ));

        assert!(!value.matches(
            "distributed",
            "storage"
        ));
    }

    #[test]
    fn creates_requirement_with_optional_rationale() {
        let value = requirement(
            "execution",
            "deterministic",
        )
        .with_rationale(
            "The operation requires deterministic behavior",
        )
        .expect("valid rationale");

        assert_eq!(
            value.key().qualified_name(),
            "execution::deterministic"
        );

        assert_eq!(
            value.rationale(),
            Some(
                "The operation requires deterministic behavior"
            )
        );
    }

    #[test]
    fn rejects_empty_rationale() {
        let result = requirement(
            "execution",
            "deterministic",
        )
        .with_rationale("   ");

        assert!(matches!(
            result,
            Err(RequirementError::EmptyComponent {
                component: RequirementComponent::Rationale
            })
        ));
    }

    #[test]
    fn requirement_identity_excludes_rationale() {
        let first = requirement(
            "quantum",
            "measurement",
        );

        let second = requirement(
            "quantum",
            "measurement",
        )
        .with_rationale("Required by this operation")
        .expect("valid rationale");

        assert_eq!(
            first.canonical_key(),
            second.canonical_key()
        );

        assert_ne!(first, second);
    }

    #[test]
    fn requirement_set_preserves_insertion_order() {
        let mut values = RequirementSet::new();

        values.push(requirement(
            "quantum",
            "measurement",
        ));

        values.push(requirement(
            "classical",
            "parallel_execution",
        ));

        assert_eq!(values.len(), 2);

        assert_eq!(
            values.get(0).unwrap().key().namespace(),
            "quantum"
        );

        assert_eq!(
            values.get(1).unwrap().key().namespace(),
            "classical"
        );
    }

    #[test]
    fn requirement_set_preserves_duplicates() {
        let repeated = requirement(
            "quantum",
            "measurement",
        );

        let mut values = RequirementSet::new();

        values.push(repeated.clone());
        values.push(repeated);

        assert_eq!(values.len(), 2);
        assert_eq!(
            values.duplicate_groups().len(),
            1
        );

        assert_eq!(
            values.duplicate_groups()[0].1,
            vec![0, 1]
        );
    }

    #[test]
    fn duplicate_reporting_preserves_distinct_namespaces() {
        let mut values = RequirementSet::new();

        values.push(requirement(
            "classical",
            "parallel",
        ));

        values.push(requirement(
            "quantum",
            "parallel",
        ));

        assert!(values.duplicate_groups().is_empty());
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let mut values = RequirementSet::new();

        values.push(requirement(
            "quantum",
            "measurement",
        ));

        values.push(requirement(
            "classical",
            "parallel",
        ));

        values.push(requirement(
            "distributed",
            "communication",
        ));

        let canonical = values.canonicalized();

        assert_eq!(
            canonical[0].key().namespace(),
            "classical"
        );

        assert_eq!(
            canonical[1].key().namespace(),
            "distributed"
        );

        assert_eq!(
            canonical[2].key().namespace(),
            "quantum"
        );

        // Canonicalization must not mutate insertion order.
        assert_eq!(
            values.get(0).unwrap().key().namespace(),
            "quantum"
        );
    }

    #[test]
    fn namespace_filter_preserves_insertion_order() {
        let mut values = RequirementSet::new();

        values.push(requirement(
            "quantum",
            "measurement",
        ));

        values.push(requirement(
            "classical",
            "parallel",
        ));

        values.push(requirement(
            "quantum",
            "dynamic_control",
        ));

        let quantum: Vec<&Requirement> =
            values.for_namespace("quantum").collect();

        assert_eq!(quantum.len(), 2);

        assert_eq!(
            quantum[0].key().name(),
            "measurement"
        );

        assert_eq!(
            quantum[1].key().name(),
            "dynamic_control"
        );
    }

    #[test]
    fn collection_extension_preserves_entries() {
        let mut first = RequirementSet::new();

        first.push(requirement(
            "classical",
            "parallel",
        ));

        let mut second = RequirementSet::new();

        second.push(requirement(
            "quantum",
            "measurement",
        ));

        second.push(requirement(
            "quantum",
            "measurement",
        ));

        first.extend(second);

        assert_eq!(first.len(), 3);
        assert_eq!(
            first.duplicate_groups().len(),
            1
        );
    }

    #[test]
    fn collection_supports_large_requirement_counts() {
        let mut values = RequirementSet::new();

        // This is a test workload, not a language or API limit.
        for index in 0..10_000 {
            values.push(requirement(
                "generated",
                &format!("requirement_{index}"),
            ));
        }

        assert_eq!(values.len(), 10_000);
        assert!(!values.is_empty());
    }

    #[test]
    fn identity_has_no_arbitrary_small_length_ceiling() {
        let long_name = "x".repeat(16_384);

        let value = RequirementKey::new(
            "future",
            long_name.clone(),
        )
        .expect("long identity must remain representable");

        assert_eq!(value.name(), long_name);
    }

    #[test]
    fn contains_checks_identity_not_satisfaction() {
        let mut values = RequirementSet::new();

        let expected = key(
            "security",
            "trusted_execution",
        );

        values.push(Requirement::new(
            expected.clone(),
        ));

        assert!(values.contains(&expected));

        assert!(!values.contains(&key(
            "security",
            "untrusted_execution",
        )));
    }

    #[test]
    fn empty_collection_has_consistent_behavior() {
        let mut values = RequirementSet::new();

        assert!(values.is_empty());
        assert_eq!(values.len(), 0);
        assert!(values.iter().next().is_none());
        assert!(values.canonicalized().is_empty());
        assert!(values.duplicate_groups().is_empty());

        values.clear();

        assert!(values.is_empty());
    }

    #[test]
    fn from_iterator_preserves_duplicates_and_order() {
        let values = RequirementSet::from_requirements([
            requirement("quantum", "measurement"),
            requirement("classical", "parallel"),
            requirement("quantum", "measurement"),
        ]);

        assert_eq!(values.len(), 3);

        assert_eq!(
            values.get(0).unwrap().key().namespace(),
            "quantum"
        );

        assert_eq!(
            values.get(1).unwrap().key().namespace(),
            "classical"
        );

        assert_eq!(
            values.duplicate_groups().len(),
            1
        );
    }

    #[test]
    fn owned_iteration_returns_all_declarations() {
        let values = RequirementSet::from_requirements([
            requirement("classical", "parallel"),
            requirement("quantum", "measurement"),
        ]);

        let collected: Vec<Requirement> =
            values.into_iter().collect();

        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn borrowed_iteration_does_not_consume_collection() {
        let values = RequirementSet::from_requirements([
            requirement("classical", "parallel"),
            requirement("quantum", "measurement"),
        ]);

        let count = (&values).into_iter().count();

        assert_eq!(count, 2);
        assert_eq!(values.len(), 2);
    }
}
