
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Distributed Requirements
//!
//! Path: `src/distributed/requirements.rs`
//!
//! ## Purpose
//!
//! Collect and preserve the target-independent requirements that govern a
//! distributed computation. This module is an aggregation and validation
//! boundary, not a second semantic model.
//!
//! ## Authority
//!
//! - `grammar/spec/distributed.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - The canonical frontend AST and semantic facts.
//!
//! ## Ownership
//!
//! This module owns:
//! - The collection of requirements needed by distributed planning.
//! - Distributed-specific, namespaced realization directives.
//! - The distinction between complete and incomplete requirement analysis.
//! - Validation of its own aggregation records.
//!
//! It does not own:
//! - Source syntax, AST definitions, or name resolution.
//! - Resource quantities, resource kinds, or resource constraints.
//! - Capability identities, capability properties, or version semantics.
//! - Target capability evidence or hardware inventories.
//! - Resource allocation, feasibility, placement, routing, or scheduling.
//! - ZUIR, ZCIR, `quantum::ir`, HDLIR, backend, networking, or runtime APIs.
//!
//! Those concepts remain owned by their existing repository modules.
//!
//! ## Scalability
//!
//! No universal limit is imposed on the number of requirements, constraints,
//! preferences, hints, or extension directives. Collections grow according
//! to available memory and any explicit operational budget imposed by the
//! calling analysis session.
//!
//! Rust collections, address spaces, and execution resources are finite.
//! This module does not claim literal infinite capacity or guarantee that
//! every target can execute every program.
//!
//! ## Semantics
//!
//! - A requirement must be satisfied for the requested realization to be
//!   valid.
//! - A constraint restricts the set of admissible realizations.
//! - A preference ranks otherwise admissible realizations.
//! - A hint provides non-binding implementation guidance.
//!
//! A preference or hint must never silently become a mandatory requirement.
//! An unresolved requirement must never be reported as satisfied.
//! Missing target evidence must never be treated as proof of compatibility.
//!
//! ## Integration
//!
//! Resource requirements are represented by the canonical
//! `frontend::semantic::resources::model::ResourceRequirement`.
//! Resource constraints are represented by the canonical
//! `frontend::semantic::resources::model::ResourceConstraint`.
//! Capability predicates are represented by the canonical
//! `frontend::semantic::capabilities::model::CapabilityRequirement`.
//!
//! This module preserves these values instead of copying their fields into
//! competing distributed-specific resource or capability structures.
//!
//! It performs no hardware discovery, target selection, resource allocation,
//! network I/O, runtime initialization, or IR transformation.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceRequirement,
};

/// Version of the distributed requirement aggregation API.
///
/// This is an in-memory API version, not a language or wire-protocol version.
pub const DISTRIBUTED_REQUIREMENTS_API_VERSION: u16 = 1;

/// The semantic strength of a capability predicate in a distributed plan.
///
/// The associated `CapabilityRequirement` remains the canonical predicate.
/// This enum specifies how the distributed planning layer must interpret it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum RequirementStrength {
    /// The realization must satisfy the capability predicate.
    Required,

    /// Prefer a realization satisfying the predicate, but do not reject an
    /// otherwise valid realization solely because the preference is unmet.
    Preferred,

    /// The capability may be used when available and permitted.
    Allowed,

    /// The capability must not be used by the requested realization.
    Forbidden,
}

impl fmt::Display for RequirementStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Required => "required",
            Self::Preferred => "preferred",
            Self::Allowed => "allowed",
            Self::Forbidden => "forbidden",
        })
    }
}

/// Provenance attached to an aggregated distributed requirement.
///
/// Provenance identifies where a fact came from. It is not a physical node
/// identity, target identity, or proof that the fact has been validated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RequirementProvenance {
    source_node: Option<NodeId>,
}

impl RequirementProvenance {
    /// Creates provenance for a known AST node.
    #[must_use]
    pub const fn from_node(source_node: NodeId) -> Self {
        Self {
            source_node: Some(source_node),
        }
    }

    /// Creates provenance for a fact with no directly associated AST node.
    ///
    /// This is appropriate for derived semantic facts only when the producer
    /// has retained their original provenance elsewhere.
    #[must_use]
    pub const fn derived() -> Self {
        Self { source_node: None }
    }

    /// Returns the associated AST node, if available.
    #[must_use]
    pub const fn source_node(self) -> Option<NodeId> {
        self.source_node
    }
}

impl Default for RequirementProvenance {
    fn default() -> Self {
        Self::derived()
    }
}

/// A canonical capability predicate with its distributed interpretation and
/// source provenance.
///
/// The predicate is owned by the semantic capability model. This wrapper
/// does not assert that a target provides the capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityEntry {
    predicate: CapabilityRequirement,
    strength: RequirementStrength,
    provenance: RequirementProvenance,
}

impl CapabilityEntry {
    /// Creates an entry around a canonical semantic capability predicate.
    #[must_use]
    pub const fn new(
        predicate: CapabilityRequirement,
        strength: RequirementStrength,
        provenance: RequirementProvenance,
    ) -> Self {
        Self {
            predicate,
            strength,
            provenance,
        }
    }

    /// Returns the canonical semantic capability predicate.
    #[must_use]
    pub const fn predicate(&self) -> &CapabilityRequirement {
        &self.predicate
    }

    /// Returns how the distributed planner must interpret the predicate.
    #[must_use]
    pub const fn strength(&self) -> RequirementStrength {
        self.strength
    }

    /// Returns the originating source information.
    #[must_use]
    pub const fn provenance(&self) -> RequirementProvenance {
        self.provenance
    }
}

/// A distributed-specific, open-world directive identity.
///
/// Names are namespaced so independently developed domains can extend the
/// requirement contract without adding variants to a closed Rust enum.
/// The namespace and name are identifiers, not source-language syntax.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DirectiveKey {
    namespace: String,
    name: String,
}

impl DirectiveKey {
    /// Constructs a validated directive key.
    ///
    /// Both components must be non-empty and must not contain control
    /// characters. The exact spelling is preserved; no case folding,
    /// aliasing, or implicit namespace resolution is performed.
    ///
    /// # Errors
    ///
    /// Returns `RequirementError::InvalidDirectiveKey` if either component
    /// is empty or contains a control character.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let namespace = namespace.into();
        let name = name.into();

        if !is_valid_component(&namespace) || !is_valid_component(&name) {
            return Err(RequirementError::InvalidDirectiveKey {
                namespace,
                name,
            });
        }

        Ok(Self { namespace, name })
    }

    /// Returns the directive namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the directive name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable display form `namespace:name`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for DirectiveKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

fn is_valid_component(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

/// A typed payload for a distributed-specific directive.
///
/// Values intentionally avoid assumptions about particular hardware,
/// providers, or computational domains. Domain-specific interpretation and
/// validation belong to the registered owner of the directive namespace.
///
/// `Text` and `Symbolic` are not interchangeable: a symbolic value must not
/// be treated as a resolved literal without the relevant semantic service.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DirectiveValue {
    /// A literal textual value.
    Text(String),

    /// A boolean value.
    Boolean(bool),

    /// A symbolic expression or unresolved semantic value.
    ///
    /// This is an opaque reference supplied by the owning semantic producer,
    /// not a new expression language or evaluator.
    Symbolic(String),
}

impl DirectiveValue {
    /// Creates a textual directive value.
    ///
    /// Empty strings are allowed because an owning directive schema may
    /// assign them a meaning. The schema owner must validate that meaning.
    #[must_use]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a symbolic directive value.
    #[must_use]
    pub fn symbolic(value: impl Into<String>) -> Self {
        Self::Symbolic(value.into())
    }
}

/// The meaning of an open-world distributed directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DirectiveKind {
    /// A mandatory condition on a valid realization.
    Requirement,

    /// A restriction on admissible realizations.
    Constraint,

    /// A non-mandatory ranking criterion.
    Preference,

    /// Non-binding implementation guidance.
    Hint,
}

impl fmt::Display for DirectiveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Requirement => "requirement",
            Self::Constraint => "constraint",
            Self::Preference => "preference",
            Self::Hint => "hint",
        })
    }
}

/// A distributed-specific directive with explicit strength and provenance.
///
/// The key's namespace owner is responsible for defining the value schema,
/// interpretation, compatibility, and versioning rules. Unknown directives
/// may be preserved for forwarding, but must not be assumed to be satisfied
/// or silently discarded by a component that claims to enforce them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedDirective {
    key: DirectiveKey,
    kind: DirectiveKind,
    value: DirectiveValue,
    provenance: RequirementProvenance,
}

impl DistributedDirective {
    /// Constructs a directive.
    #[must_use]
    pub const fn new(
        key: DirectiveKey,
        kind: DirectiveKind,
        value: DirectiveValue,
        provenance: RequirementProvenance,
    ) -> Self {
        Self {
            key,
            kind,
            value,
            provenance,
        }
    }

    /// Returns the directive key.
    #[must_use]
    pub const fn key(&self) -> &DirectiveKey {
        &self.key
    }

    /// Returns the directive kind.
    #[must_use]
    pub const fn kind(&self) -> DirectiveKind {
        self.kind
    }

    /// Returns the directive value.
    #[must_use]
    pub const fn value(&self) -> &DirectiveValue {
        &self.value
    }

    /// Returns the source provenance.
    #[must_use]
    pub const fn provenance(&self) -> RequirementProvenance {
        self.provenance
    }
}

/// The completeness status of requirement analysis.
///
/// Incomplete analysis must never be treated as proof that a program has no
/// additional requirements or that a realization is feasible.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementAnalysisStatus {
    /// All requirement sources declared to the collector were processed.
    Complete,

    /// Analysis could not establish a complete requirement set.
    ///
    /// Each reason should be actionable and should identify the failed
    /// semantic stage or source when that information is available.
    Incomplete {
        /// Reasons the analysis could not be completed.
        reasons: Vec<String>,
    },
}

impl RequirementAnalysisStatus {
    /// Returns whether analysis completed without known omissions.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Errors produced by this aggregation module.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementError {
    /// A directive key contains an invalid component.
    InvalidDirectiveKey {
        /// The supplied namespace.
        namespace: String,
        /// The supplied directive name.
        name: String,
    },

    /// An incomplete requirement set was used as if it were complete.
    IncompleteAnalysis {
        /// Known reasons analysis is incomplete.
        reasons: Vec<String>,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDirectiveKey { namespace, name } => write!(
                f,
                "invalid distributed directive key: namespace {:?}, name {:?}; \
                 components must be non-empty and contain no control characters",
                namespace, name
            ),
            Self::IncompleteAnalysis { reasons } => {
                f.write_str("distributed requirement analysis is incomplete")?;
                for reason in reasons {
                    write!(f, "; {reason}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for RequirementError {}

/// The complete target-independent requirement envelope for distributed work.
///
/// This type preserves canonical semantic resource and capability objects.
/// It does not prove that a target exists, has sufficient capacity, or is
/// authorized to execute the requested computation.
///
/// Entries are retained in insertion/source order. Duplicates are preserved
/// intentionally: two source declarations may express separate obligations,
/// and deciding whether they can be combined is a semantic operation owned by
/// the relevant analysis service.
///
/// The type has no universal count limit. An external analysis session may
/// apply an explicit operational budget; exceeding it must be reported as an
/// incomplete analysis rather than a successful, truncated result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedRequirements {
    capabilities: Vec<CapabilityEntry>,
    resources: Vec<ResourceRequirement>,
    resource_constraints: Vec<ResourceConstraint>,
    directives: Vec<DistributedDirective>,
    status: RequirementAnalysisStatus,
}

impl Default for DistributedRequirements {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedRequirements {
    /// Creates an empty, complete collection.
    ///
    /// An empty collection is meaningful only when the caller knows that all
    /// relevant requirement sources have been examined.
    #[must_use]
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
            resources: Vec::new(),
            resource_constraints: Vec::new(),
            directives: Vec::new(),
            status: RequirementAnalysisStatus::Complete,
        }
    }

    /// Returns capability predicates in insertion/source order.
    #[must_use]
    pub fn capabilities(&self) -> &[CapabilityEntry] {
        &self.capabilities
    }

    /// Returns canonical resource requirements in insertion/source order.
    #[must_use]
    pub fn resources(&self) -> &[ResourceRequirement] {
        &self.resources
    }

    /// Returns canonical resource constraints in insertion/source order.
    #[must_use]
    pub fn resource_constraints(&self) -> &[ResourceConstraint] {
        &self.resource_constraints
    }

    /// Returns distributed-specific directives in insertion/source order.
    #[must_use]
    pub fn directives(&self) -> &[DistributedDirective] {
        &self.directives
    }

    /// Returns the current analysis status.
    #[must_use]
    pub fn status(&self) -> &RequirementAnalysisStatus {
        &self.status
    }

    /// Returns whether the collection is known to be complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Adds a canonical capability predicate.
    ///
    /// The caller must have already performed the appropriate semantic
    /// validation and name resolution. This operation does not discover or
    /// verify target support.
    pub fn add_capability(&mut self, entry: CapabilityEntry) {
        self.capabilities.push(entry);
    }

    /// Adds a canonical semantic resource requirement.
    ///
    /// The resource model retains its own quantity, unit, intent, and source
    /// information. This method does not reinterpret or resolve them.
    pub fn add_resource(&mut self, requirement: ResourceRequirement) {
        self.resources.push(requirement);
    }

    /// Adds a canonical semantic resource constraint.
    ///
    /// Constraint evaluation remains the responsibility of the semantic
    /// constraint service or an authorized downstream feasibility service.
    pub fn add_resource_constraint(&mut self, constraint: ResourceConstraint) {
        self.resource_constraints.push(constraint);
    }

    /// Adds a validated distributed-specific directive.
    ///
    /// Validation of the directive's domain-specific value is delegated to
    /// the owning namespace/schema. This method validates the aggregation
    /// boundary only.
    pub fn add_directive(&mut self, directive: DistributedDirective) {
        self.directives.push(directive);
    }

    /// Marks analysis incomplete and records why.
    ///
    /// Calling this method never clears earlier failures. Multiple stages may
    /// contribute reasons; preserving them prevents a later successful stage
    /// from hiding an earlier omission.
    pub fn mark_incomplete(&mut self, reason: impl Into<String>) {
        let reason = reason.into();

        match &mut self.status {
            RequirementAnalysisStatus::Complete => {
                self.status = RequirementAnalysisStatus::Incomplete {
                    reasons: vec![reason],
                };
            }
            RequirementAnalysisStatus::Incomplete { reasons } => {
                reasons.push(reason);
            }
        }
    }

    /// Consumes the collection and returns it only when analysis is complete.
    ///
    /// This gate prevents partial semantic analysis from being accidentally
    /// promoted to a valid planning input.
    ///
    /// # Errors
    ///
    /// Returns `RequirementError::IncompleteAnalysis` when any requirement
    /// source was unresolved, omitted, or otherwise not fully analyzed.
    pub fn into_complete(self) -> Result<Self, RequirementError> {
        match &self.status {
            RequirementAnalysisStatus::Complete => Ok(self),
            RequirementAnalysisStatus::Incomplete { reasons } => {
                Err(RequirementError::IncompleteAnalysis {
                    reasons: reasons.clone(),
                })
            }
        }
    }

    /// Returns whether the collection contains no entries.
    ///
    /// This does not establish that the source program has no requirements;
    /// check [`Self::is_complete`] as well.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
            && self.resources.is_empty()
            && self.resource_constraints.is_empty()
            && self.directives.is_empty()
    }

    /// Returns the total number of collected entries.
    ///
    /// The return type is `usize` because it counts elements in this
    /// in-memory Rust collection; it is not a semantic capacity limit.
    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.capabilities
            .len()
            .saturating_add(self.resources.len())
            .saturating_add(self.resource_constraints.len())
            .saturating_add(self.directives.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_collection_is_empty_and_complete() {
        let requirements = DistributedRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.is_complete());
        assert_eq!(requirements.entry_count(), 0);
    }

    #[test]
    fn incomplete_analysis_cannot_be_promoted_to_complete() {
        let mut requirements = DistributedRequirements::new();
        requirements.mark_incomplete("capability name was unresolved");

        assert!(!requirements.is_complete());

        let error = requirements
            .into_complete()
            .expect_err("incomplete analysis must be rejected");

        assert!(matches!(
            error,
            RequirementError::IncompleteAnalysis { .. }
        ));
    }

    #[test]
    fn multiple_incomplete_reasons_are_preserved() {
        let mut requirements = DistributedRequirements::new();
        requirements.mark_incomplete("resource expression unresolved");
        requirements.mark_incomplete("capability analysis incomplete");

        match requirements.status() {
            RequirementAnalysisStatus::Incomplete { reasons } => {
                assert_eq!(reasons.len(), 2);
                assert_eq!(reasons[0], "resource expression unresolved");
                assert_eq!(reasons[1], "capability analysis incomplete");
            }
            RequirementAnalysisStatus::Complete => {
                panic!("expected incomplete status");
            }
        }
    }

    #[test]
    fn directive_key_preserves_extension_names() {
        let key = DirectiveKey::new("future.distributed", "locality-policy")
            .expect("valid open-world directive key");

        assert_eq!(key.namespace(), "future.distributed");
        assert_eq!(key.name(), "locality-policy");
        assert_eq!(
            key.qualified_name(),
            "future.distributed:locality-policy"
        );
    }

    #[test]
    fn directive_key_rejects_empty_or_control_components() {
        assert!(DirectiveKey::new("", "policy").is_err());
        assert!(DirectiveKey::new("distributed", "").is_err());
        assert!(DirectiveKey::new("distributed", "bad\nname").is_err());
    }

    #[test]
    fn incomplete_reasons_survive_later_updates() {
        let mut requirements = DistributedRequirements::new();
        requirements.mark_incomplete("first failure");
        requirements.mark_incomplete("second failure");

        let error = requirements
            .into_complete()
            .expect_err("analysis remains incomplete");

        match error {
            RequirementError::IncompleteAnalysis { reasons } => {
                assert_eq!(reasons, vec!["first failure", "second failure"]);
            }
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn count_is_an_entry_count_not_a_capacity_policy() {
        let requirements = DistributedRequirements::new();

        assert_eq!(requirements.entry_count(), 0);
        assert!(requirements.is_complete());
    }
}
