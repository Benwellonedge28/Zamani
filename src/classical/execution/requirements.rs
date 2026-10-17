
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution Requirements
//!
//! Path: `src/classical/execution/requirements.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Aggregate canonical semantic resource requirements, resource constraints,
//! and capability requirements for classical execution without redefining
//! their semantic meaning.
//!
//! This module is an integration boundary between the finalized frontend
//! semantic model and downstream execution planning.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/spec/resources.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/classical/execution/plan.rs`
//!
//! The grammar owns syntax. The normative specifications govern semantics.
//! The frontend semantic models own canonical requirement representations.
//! This module owns only their aggregation and completeness status.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - `ExecutionRequirements`;
//! - provenance wrappers for capability requirements;
//! - requirement extraction/completeness status;
//! - structured aggregation errors;
//! - deterministic source-order iteration;
//! - validation that the requirement collection is complete before a
//!   consumer treats it as a complete input to execution planning.
//!
//! This module does not own:
//!
//! - source syntax, AST construction, name resolution, or type checking;
//! - canonical resource kinds, quantities, units, or resource constraints;
//! - canonical capability identities, versions, or properties;
//! - symbolic expression evaluation or dimensional analysis;
//! - target capability inventories or resource availability;
//! - target feasibility, placement, allocation, or scheduling;
//! - canonical ZUIR operations or another intermediate representation;
//! - runtime execution or fallback decisions.
//!
//! ## Integration contract
//!
//! Frontend semantic analysis supplies validated canonical resource and
//! capability values. AST-to-semantic conversion remains in the frontend.
//!
//! `ExecutionRequirements` retains those values without narrowing symbolic
//! quantities to host-sized integers or silently merging declarations.
//!
//! The downstream execution plan and feasibility services may consume this
//! collection through an explicit adapter. They must preserve the canonical
//! semantic values and their provenance.
//!
//! `ResourceIntent` continues to distinguish required, preferred, allowed,
//! forbidden, hint, and budget intent. This module does not collapse those
//! intents into a single boolean.
//!
//! ## Completeness
//!
//! An incomplete collection means that requirement extraction or semantic
//! preparation did not finish successfully. Consumers must not treat it as
//! a complete account of program requirements.
//!
//! A symbolic quantity does not, by itself, make a collection incomplete.
//! Symbolic quantities are valid semantic values and remain unresolved until
//! the appropriate semantic expression service can evaluate them.
//!
//! ## Scalability and determinism
//!
//! No universal hardware, requirement-count, capability-count, or resource
//! quantity ceiling is imposed here. Collections grow with input and the
//! resources actually available. Allocation failure and explicit operational
//! budget exhaustion remain real implementation constraints.
//!
//! Requirements are retained in insertion/source order. Duplicate entries
//! are preserved: identical-looking declarations may have distinct origins
//! or semantics. This module never silently deduplicates requirements.
//!
//! ## Safety
//!
//! This module performs no I/O, hardware discovery, global mutation, resource
//! allocation, or execution. It contains no unsafe Rust.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;
use crate::frontend::semantic::resources::model::{
    ResourceConstraint, ResourceRequirement,
};

/// Result type for classical execution requirement operations.
pub type ExecutionRequirementsResult<T> = Result<T, ExecutionRequirementsError>;

/// Whether the collected requirements are complete enough to be consumed as
/// the complete result of semantic requirement preparation.
///
/// `Complete` means this collection has no recorded extraction failures.
/// It does not mean that requirements have been satisfied by a target.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RequirementsCompleteness {
    /// No extraction or preparation failures have been recorded.
    #[default]
    Complete,

    /// One or more requirement inputs could not be fully prepared.
    Incomplete,
}

/// Stable classification of a requirement-preparation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RequirementIssueKind {
    /// A source declaration was structurally or semantically invalid.
    InvalidDeclaration,

    /// A resource kind, capability, name, or other reference was unresolved.
    UnresolvedReference,

    /// A required semantic operation could not be completed.
    AnalysisFailure,

    /// A configured operational budget was exhausted.
    BudgetExhausted,

    /// A required semantic input was unavailable.
    MissingInput,

    /// An extension or feature could not be interpreted by the current
    /// implementation.
    UnsupportedFeature,
}

impl RequirementIssueKind {
    /// Return the stable machine-readable issue code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidDeclaration => "classical.requirements.invalid_declaration",
            Self::UnresolvedReference => "classical.requirements.unresolved_reference",
            Self::AnalysisFailure => "classical.requirements.analysis_failure",
            Self::BudgetExhausted => "classical.requirements.budget_exhausted",
            Self::MissingInput => "classical.requirements.missing_input",
            Self::UnsupportedFeature => "classical.requirements.unsupported_feature",
        }
    }
}

/// A recorded issue that prevented complete requirement preparation.
///
/// The source node is optional because some failures arise in synthesized
/// semantic data or in integration boundaries rather than directly at a
/// source declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementIssue {
    kind: RequirementIssueKind,
    source_node: Option<NodeId>,
    message: String,
}

impl RequirementIssue {
    /// Construct a requirement issue.
    ///
    /// The message is intended for diagnostics. Consumers should branch on
    /// `kind()` rather than parsing the message.
    #[must_use]
    pub fn new(
        kind: RequirementIssueKind,
        source_node: Option<NodeId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_node,
            message: message.into(),
        }
    }

    /// Return the stable issue classification.
    #[must_use]
    pub const fn kind(&self) -> RequirementIssueKind {
        self.kind
    }

    /// Return the originating AST node, if known.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }

    /// Return the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the stable machine-readable issue code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.as_str()
    }
}

/// A canonical capability requirement with optional source provenance.
///
/// `CapabilityRequirement` remains the canonical semantic value. This wrapper
/// adds provenance because the capability model intentionally does not own
/// AST identity or source locations.
///
/// A `None` source node is permitted for requirements introduced by a
/// language-defined rule or a trusted semantic transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRequirementEntry {
    requirement: CapabilityRequirement,
    source_node: Option<NodeId>,
}

impl CapabilityRequirementEntry {
    /// Construct a capability requirement entry.
    #[must_use]
    pub const fn new(
        requirement: CapabilityRequirement,
        source_node: Option<NodeId>,
    ) -> Self {
        Self {
            requirement,
            source_node,
        }
    }

    /// Return the canonical capability requirement.
    #[must_use]
    pub const fn requirement(&self) -> &CapabilityRequirement {
        &self.requirement
    }

    /// Return the originating AST node, if known.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }
}

/// Canonical, target-independent requirements for classical execution.
///
/// This is a semantic input aggregate, not an executable plan, resource
/// reservation, target description, or proof of feasibility.
///
/// Resource and capability values retain their canonical frontend types.
/// Collections preserve insertion order and duplicate declarations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExecutionRequirements {
    resource_requirements: Vec<ResourceRequirement>,
    resource_constraints: Vec<ResourceConstraint>,
    capability_requirements: Vec<CapabilityRequirementEntry>,
    issues: Vec<RequirementIssue>,
    completeness: RequirementsCompleteness,
}

impl ExecutionRequirements {
    /// Create an empty, complete requirement collection.
    ///
    /// Empty means that no requirements have been added to this collection;
    /// it does not assert that an arbitrary program has no requirements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the current completeness state.
    #[must_use]
    pub const fn completeness(&self) -> RequirementsCompleteness {
        self.completeness
    }

    /// Return whether no preparation failures have been recorded.
    ///
    /// This does not indicate target feasibility or capability availability.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completeness, RequirementsCompleteness::Complete)
    }

    /// Append a canonical resource requirement.
    ///
    /// The original intent, unit, quantity, scope, and source provenance
    /// remain owned by the canonical semantic model. Symbolic quantities are
    /// retained without conversion to `usize`, `u64`, or `u128`.
    pub fn add_resource_requirement(&mut self, requirement: ResourceRequirement) {
        self.resource_requirements.push(requirement);
    }

    /// Append a canonical resource constraint.
    ///
    /// Constraint interpretation and evaluation belong to the appropriate
    /// semantic constraint service or a downstream feasibility operation.
    pub fn add_resource_constraint(&mut self, constraint: ResourceConstraint) {
        self.resource_constraints.push(constraint);
    }

    /// Append a canonical capability requirement with optional provenance.
    ///
    /// Repeated entries are preserved instead of being silently deduplicated.
    /// Capability compatibility and evidence evaluation are separate tasks.
    pub fn add_capability_requirement(
        &mut self,
        requirement: CapabilityRequirement,
        source_node: Option<NodeId>,
    ) {
        self.capability_requirements
            .push(CapabilityRequirementEntry::new(requirement, source_node));
    }

    /// Record an issue and mark the collection incomplete.
    ///
    /// This operation is intended for the caller that owns requirement
    /// preparation. It does not decide how diagnostics are rendered.
    pub fn record_issue(&mut self, issue: RequirementIssue) {
        self.issues.push(issue);
        self.completeness = RequirementsCompleteness::Incomplete;
    }

    /// Return resource requirements in insertion/source order.
    #[must_use]
    pub fn resource_requirements(&self) -> &[ResourceRequirement] {
        &self.resource_requirements
    }

    /// Return resource constraints in insertion/source order.
    #[must_use]
    pub fn resource_constraints(&self) -> &[ResourceConstraint] {
        &self.resource_constraints
    }

    /// Return capability requirements in insertion/source order.
    #[must_use]
    pub fn capability_requirements(&self) -> &[CapabilityRequirementEntry] {
        &self.capability_requirements
    }

    /// Return all recorded preparation issues in insertion order.
    #[must_use]
    pub fn issues(&self) -> &[RequirementIssue] {
        &self.issues
    }

    /// Return the number of resource requirements.
    #[must_use]
    pub fn resource_requirement_count(&self) -> usize {
        self.resource_requirements.len()
    }

    /// Return the number of resource constraints.
    #[must_use]
    pub fn resource_constraint_count(&self) -> usize {
        self.resource_constraints.len()
    }

    /// Return the number of capability requirements.
    #[must_use]
    pub fn capability_requirement_count(&self) -> usize {
        self.capability_requirements.len()
    }

    /// Return the number of recorded preparation issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Check whether this collection may be consumed as a complete result.
    ///
    /// A successful result certifies only that this aggregate has no recorded
    /// preparation issues. It does not certify that symbolic quantities have
    /// been evaluated or that any execution target is feasible.
    pub fn ensure_complete(&self) -> ExecutionRequirementsResult<()> {
        if self.is_complete() {
            Ok(())
        } else {
            Err(ExecutionRequirementsError::Incomplete {
                issue_count: self.issues.len(),
            })
        }
    }

    /// Return a deterministic view of resource requirements sorted by their
    /// canonical resource-model sort key.
    ///
    /// This returns references and does not mutate the source-order collection.
    /// Equal sort keys retain their relative source order through stable
    /// sorting. The key is a presentation/order key, not semantic equivalence.
    #[must_use]
    pub fn resources_in_canonical_order(&self) -> Vec<&ResourceRequirement> {
        let mut resources: Vec<_> = self.resource_requirements.iter().collect();
        resources.sort_by_key(|requirement| requirement.canonical_sort_key());
        resources
    }

    /// Merge another requirement collection into this one.
    ///
    /// Entries and issues are appended in source order. Incompleteness is
    /// contagious: merging an incomplete collection cannot produce a
    /// complete result.
    ///
    /// This operation does not deduplicate, resolve conflicts, evaluate
    /// symbolic quantities, or establish target feasibility.
    pub fn append(&mut self, other: Self) {
        self.resource_requirements
            .extend(other.resource_requirements);
        self.resource_constraints
            .extend(other.resource_constraints);
        self.capability_requirements
            .extend(other.capability_requirements);
        self.issues.extend(other.issues);

        if other.completeness == RequirementsCompleteness::Incomplete {
            self.completeness = RequirementsCompleteness::Incomplete;
        }
    }
}

/// Errors returned by requirement aggregation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExecutionRequirementsError {
    /// Requirement preparation was incomplete.
    Incomplete {
        /// Number of recorded preparation issues.
        issue_count: usize,
    },
}

impl fmt::Display for ExecutionRequirementsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incomplete { issue_count } => write!(
                formatter,
                "classical execution requirements are incomplete: \
                 {issue_count} preparation issue(s) recorded"
            ),
        }
    }
}

impl Error for ExecutionRequirementsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_collection_is_empty_and_complete() {
        let requirements = ExecutionRequirements::new();

        assert!(requirements.is_complete());
        assert_eq!(requirements.completeness(), RequirementsCompleteness::Complete);
        assert_eq!(requirements.resource_requirement_count(), 0);
        assert_eq!(requirements.resource_constraint_count(), 0);
        assert_eq!(requirements.capability_requirement_count(), 0);
        assert_eq!(requirements.issue_count(), 0);
        assert!(requirements.ensure_complete().is_ok());
    }

    #[test]
    fn recording_an_issue_marks_the_collection_incomplete() {
        let mut requirements = ExecutionRequirements::new();

        requirements.record_issue(RequirementIssue::new(
            RequirementIssueKind::UnresolvedReference,
            None,
            "A resource reference could not be resolved.",
        ));

        assert!(!requirements.is_complete());
        assert!(matches!(
            requirements.ensure_complete(),
            Err(ExecutionRequirementsError::Incomplete { issue_count: 1 })
        ));
    }

    #[test]
    fn issue_codes_are_stable_and_machine_readable() {
        let issue = RequirementIssue::new(
            RequirementIssueKind::BudgetExhausted,
            None,
            "The configured analysis budget was exhausted.",
        );

        assert_eq!(
            issue.code(),
            "classical.requirements.budget_exhausted"
        );
        assert_eq!(issue.kind(), RequirementIssueKind::BudgetExhausted);
    }

    #[test]
    fn merging_incomplete_requirements_preserves_incompleteness() {
        let mut complete = ExecutionRequirements::new();
        let mut incomplete = ExecutionRequirements::new();

        incomplete.record_issue(RequirementIssue::new(
            RequirementIssueKind::AnalysisFailure,
            None,
            "Semantic preparation did not finish.",
        ));

        complete.append(incomplete);

        assert!(!complete.is_complete());
        assert_eq!(complete.issue_count(), 1);
    }

    #[test]
    fn merging_does_not_drop_independent_issues() {
        let mut left = ExecutionRequirements::new();
        let mut right = ExecutionRequirements::new();

        left.record_issue(RequirementIssue::new(
            RequirementIssueKind::MissingInput,
            None,
            "Missing semantic input A.",
        ));
        right.record_issue(RequirementIssue::new(
            RequirementIssueKind::UnsupportedFeature,
            None,
            "Unsupported semantic feature B.",
        ));

        left.append(right);

        assert_eq!(left.issue_count(), 2);
        assert_eq!(
            left.issues()[0].kind(),
            RequirementIssueKind::MissingInput
        );
        assert_eq!(
            left.issues()[1].kind(),
            RequirementIssueKind::UnsupportedFeature
        );
    }

    #[test]
    fn empty_collections_can_be_cloned_and_compared() {
        let original = ExecutionRequirements::new();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
