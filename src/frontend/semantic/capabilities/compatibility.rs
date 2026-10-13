
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend — Capability Compatibility
//!
//! Path: `src/frontend/semantic/capabilities/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Evaluate whether scoped capability evidence satisfies the capability
//! requirements collected by semantic analysis.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/spec/quantum.md`
//! - `grammar/spec/hdl.md`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/semantic/capabilities/requirements.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//!
//! ## OWNS
//!
//! - Comparing collected requirements with evidence for an explicit scope.
//! - Reporting satisfaction, incompatibility, missing evidence, incomplete
//!   evidence, and contradictory evidence as distinct outcomes.
//! - Aggregating outcomes without losing requirement source provenance.
//! - Validating evidence records before using them for compatibility analysis.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing, AST definitions, name resolution, or capability registration.
//! - Authentication or verification of evidence origins.
//! - Authorization, security policy, or runtime permission checks.
//! - Physical hardware discovery, resource allocation, placement, routing,
//!   scheduling, or target lowering.
//! - Source-language validity or semantic type correctness.
//! - Classical IR, `quantum::ir`, HDL IR, or runtime execution.
//!
//! ## SEMANTIC CONTRACT
//!
//! A compatibility result is scoped to the `EvidenceSubject` supplied by the
//! caller. Evidence for different subjects is never combined.
//!
//! A requirement is satisfied only when the evidence model establishes that
//! its capability, version constraint, and required properties are satisfied.
//! Missing evidence is not proof of support. Conflicting evidence is not
//! silently resolved by insertion order.
//!
//! Compatibility is not a proof that evidence is authentic, that a target is
//! authorized, or that the program can successfully execute. Those questions
//! require separate services and downstream checks.
//!
//! An incompatibility with a particular scope does not, by itself, make the
//! source program semantically invalid.
//!
//! ## SCALABILITY AND DETERMINISM
//!
//! There is no fixed maximum number of requirements, capability names, domains,
//! evidence records, or target profiles in this module. Collections grow with
//! available memory. The caller's semantic session is responsible for
//! enforcing any configured operational budgets.
//!
//! Results follow the deterministic source-node order of
//! `CapabilityRequirements`. No hardware catalogue, global mutable registry,
//! random state, or target-specific capacity constant is used.
//!
//! This module cannot process literally infinite input; it introduces no
//! arbitrary language-level capacity ceiling.
//!
//! ## INTEGRATION
//!
//! Input:
//!   `CapabilityRequirements`, `EvidenceSubject`, `CapabilityEvidenceSet`
//!
//! Output:
//!   `CapabilityCompatibilityReport`
//!
//! This module consumes the canonical capability model and evidence model.
//! It does not duplicate their types or change their ownership.
//!
//! ## FREEZE CRITERIA
//!
//! - Missing, satisfied, incompatible, indeterminate, and conflicting evidence
//!   remain distinguishable.
//! - Evidence is validated before evaluation, including evidence that may have
//!   been deserialized.
//! - Every result retains its source AST node and span.
//! - Output ordering is deterministic.
//! - Empty input is not presented as proof of compatibility.
//! - The compatibility tests and relevant workspace checks pass.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::span::Span;
use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidenceError,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    EvidenceSubject,
};
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;
use crate::frontend::semantic::capabilities::requirements::{
    CapabilityRequirementRecord,
    CapabilityRequirements,
};

/// Stable architectural identifier for this module.
pub const CAPABILITY_COMPATIBILITY_MODULE_NAME: &str =
    "zamani.frontend.semantic.capabilities.compatibility";

/// Version of this module's report contract.
///
/// This is not the Zamani language version, capability-definition version,
/// evidence schema version, or target-profile version.
pub const CAPABILITY_COMPATIBILITY_SCHEMA_VERSION: u16 = 1;

/// Result of evaluating one requirement against evidence in one scope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityCompatibilityStatus {
    /// Relevant evidence establishes that the requirement is satisfied.
    Satisfied,

    /// Relevant evidence establishes non-support or a known mismatch.
    Incompatible,

    /// No evidence exists for this capability in the requested scope.
    NoEvidence,

    /// Relevant evidence exists but does not establish a final answer.
    Indeterminate,

    /// Evidence contains contradictory support and non-support claims.
    ConflictingEvidence,
}

impl CapabilityCompatibilityStatus {
    /// Whether the evidence establishes satisfaction of this requirement.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether the evidence establishes an incompatibility.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Whether the outcome needs more evidence or conflict resolution.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::NoEvidence
                | Self::Indeterminate
                | Self::ConflictingEvidence
        )
    }
}

impl fmt::Display for CapabilityCompatibilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Satisfied => formatter.write_str("satisfied"),
            Self::Incompatible => formatter.write_str("incompatible"),
            Self::NoEvidence => formatter.write_str("no evidence"),
            Self::Indeterminate => formatter.write_str("indeterminate"),
            Self::ConflictingEvidence => {
                formatter.write_str("conflicting evidence")
            }
        }
    }
}

/// Compatibility result for one source-level requirement.
///
/// Source provenance is retained so diagnostics can point back to the
/// originating AST construct. The requirement itself is the canonical
/// semantic model value, not a second compatibility-specific representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityCompatibilityRecord {
    source_node: NodeId,
    source_span: Span,
    requirement: CapabilityRequirement,
    status: CapabilityCompatibilityStatus,
}

impl CapabilityCompatibilityRecord {
    fn from_requirement(
        record: &CapabilityRequirementRecord,
        status: CapabilityCompatibilityStatus,
    ) -> Self {
        Self {
            source_node: record.source_node(),
            source_span: record.source_span().clone(),
            requirement: record.requirement().clone(),
            status,
        }
    }

    /// Return the originating AST node identity.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the source span of the requirement.
    #[must_use]
    pub const fn source_span(&self) -> &Span {
        &self.source_span
    }

    /// Return the canonical requirement that was evaluated.
    #[must_use]
    pub const fn requirement(&self) -> &CapabilityRequirement {
        &self.requirement
    }

    /// Return the evidence-based compatibility status.
    #[must_use]
    pub const fn status(&self) -> CapabilityCompatibilityStatus {
        self.status
    }
}

/// Deterministic counts of requirement outcomes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilityCompatibilitySummary {
    satisfied: usize,
    incompatible: usize,
    no_evidence: usize,
    indeterminate: usize,
    conflicting_evidence: usize,
}

impl CapabilityCompatibilitySummary {
    fn include(&mut self, status: CapabilityCompatibilityStatus) {
        match status {
            CapabilityCompatibilityStatus::Satisfied => {
                self.satisfied += 1;
            }
            CapabilityCompatibilityStatus::Incompatible => {
                self.incompatible += 1;
            }
            CapabilityCompatibilityStatus::NoEvidence => {
                self.no_evidence += 1;
            }
            CapabilityCompatibilityStatus::Indeterminate => {
                self.indeterminate += 1;
            }
            CapabilityCompatibilityStatus::ConflictingEvidence => {
                self.conflicting_evidence += 1;
            }
        }
    }

    /// Number of requirements whose evidence establishes satisfaction.
    #[must_use]
    pub const fn satisfied(&self) -> usize {
        self.satisfied
    }

    /// Number of requirements with an established incompatibility.
    #[must_use]
    pub const fn incompatible(&self) -> usize {
        self.incompatible
    }

    /// Number of requirements with no matching evidence.
    #[must_use]
    pub const fn no_evidence(&self) -> usize {
        self.no_evidence
    }

    /// Number of requirements with incomplete evidence.
    #[must_use]
    pub const fn indeterminate(&self) -> usize {
        self.indeterminate
    }

    /// Number of requirements with contradictory evidence.
    #[must_use]
    pub const fn conflicting_evidence(&self) -> usize {
        self.conflicting_evidence
    }

    /// Total number of evaluated requirements.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.satisfied
            + self.incompatible
            + self.no_evidence
            + self.indeterminate
            + self.conflicting_evidence
    }

    /// Whether every evaluated requirement has evidence establishing support.
    ///
    /// Returns `false` for an empty report: evaluating zero requirements is
    /// not evidence that a target satisfies a non-empty program's needs.
    #[must_use]
    pub const fn all_satisfied(&self) -> bool {
        self.total() > 0
            && self.satisfied == self.total()
    }

    /// Whether at least one requirement is known to be incompatible.
    #[must_use]
    pub const fn has_incompatibilities(&self) -> bool {
        self.incompatible > 0
    }

    /// Whether any requirement remains unresolved.
    #[must_use]
    pub const fn has_unresolved_requirements(&self) -> bool {
        self.no_evidence > 0
            || self.indeterminate > 0
            || self.conflicting_evidence > 0
    }
}

/// Results of evaluating a requirement set against one evidence scope.
///
/// A report records evidence-based outcomes only. It does not certify source
/// validity, authenticity of evidence, authorization, or execution success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityCompatibilityReport {
    subject: String,
    records: Vec<CapabilityCompatibilityRecord>,
    summary: CapabilityCompatibilitySummary,
}

impl CapabilityCompatibilityReport {
    fn new(subject: &EvidenceSubject) -> Self {
        Self {
            subject: subject.as_str().to_owned(),
            records: Vec::new(),
            summary: CapabilityCompatibilitySummary::default(),
        }
    }

    fn push(&mut self, record: CapabilityCompatibilityRecord) {
        self.summary.include(record.status());
        self.records.push(record);
    }

    /// Return the exact evidence scope evaluated.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Return the number of evaluated requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether there were no requirements to evaluate.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Iterate over results in deterministic source-node order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &CapabilityCompatibilityRecord> {
        self.records.iter()
    }

    /// Return the aggregate counts.
    #[must_use]
    pub const fn summary(&self) -> &CapabilityCompatibilitySummary {
        &self.summary
    }

    /// Whether every requirement is established as satisfied.
    ///
    /// Empty reports return `false`; callers must not interpret an empty
    /// requirement set as evidence about an execution environment.
    #[must_use]
    pub const fn all_satisfied(&self) -> bool {
        self.summary.all_satisfied()
    }

    /// Find the result associated with a source AST node.
    #[must_use]
    pub fn get(
        &self,
        source_node: NodeId,
    ) -> Option<&CapabilityCompatibilityRecord> {
        self.records
            .binary_search_by_key(&source_node, |record| record.source_node())
            .ok()
            .map(|index| &self.records[index])
    }
}

/// Failure while preparing capability evidence for compatibility evaluation.
///
/// These failures indicate malformed evidence data, not an incompatibility
/// between a program and a target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityCompatibilityError {
    /// One evidence record violates its model invariants.
    InvalidEvidence(CapabilityEvidenceError),
}

impl fmt::Display for CapabilityCompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEvidence(error) => {
                write!(
                    formatter,
                    "cannot evaluate invalid capability evidence: {error}"
                )
            }
        }
    }
}

impl Error for CapabilityCompatibilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidEvidence(error) => Some(error),
        }
    }
}

impl From<CapabilityEvidenceError> for CapabilityCompatibilityError {
    fn from(error: CapabilityEvidenceError) -> Self {
        Self::InvalidEvidence(error)
    }
}

/// Evaluate all collected requirements against evidence for one explicit scope.
///
/// Requirements are evaluated independently. This function does not merge
/// requirements from different AST nodes, infer unprovided evidence, resolve
/// aliases, or select another scope automatically.
///
/// Evidence records are validated before evaluation because deserialization
/// may construct values without invoking their checked constructors.
///
/// # Errors
///
/// Returns an error if any evidence record fails validation. No partial report
/// is returned.
///
/// # Determinism
///
/// Output order follows the `BTreeMap`-backed `CapabilityRequirements`
/// iteration order, which is ordered by source AST node identity.
pub fn evaluate_compatibility(
    requirements: &CapabilityRequirements,
    subject: &EvidenceSubject,
    evidence: &CapabilityEvidenceSet,
) -> Result<CapabilityCompatibilityReport, CapabilityCompatibilityError> {
    for record in evidence.iter() {
        record.validate()?;
    }

    let mut report = CapabilityCompatibilityReport::new(subject);

    for requirement_record in requirements.iter() {
        let evidence_result = evidence.evaluate(
            subject,
            requirement_record.requirement(),
        );

        let status = map_evidence_result(evidence_result);
        report.push(CapabilityCompatibilityRecord::from_requirement(
            requirement_record,
            status,
        ));
    }

    Ok(report)
}

fn map_evidence_result(
    result: CapabilityEvidenceResult,
) -> CapabilityCompatibilityStatus {
    match result {
        CapabilityEvidenceResult::Satisfied => {
            CapabilityCompatibilityStatus::Satisfied
        }
        CapabilityEvidenceResult::NotSatisfied => {
            CapabilityCompatibilityStatus::Incompatible
        }
        CapabilityEvidenceResult::NoEvidence => {
            CapabilityCompatibilityStatus::NoEvidence
        }
        CapabilityEvidenceResult::Indeterminate => {
            CapabilityCompatibilityStatus::Indeterminate
        }
        CapabilityEvidenceResult::ConflictingEvidence => {
            CapabilityCompatibilityStatus::ConflictingEvidence
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::capabilities::capability::Capability;
    use crate::frontend::ast::node::NodeMetadata;
    use crate::frontend::semantic::capabilities::evidence::{
        CapabilityEvidence,
        CapabilityEvidenceSet,
        CapabilityEvidenceStatus,
        EvidenceOrigin,
        EvidenceSubject,
    };
    use crate::frontend::semantic::capabilities::model::{
        CapabilityId,
        CapabilityVersion,
        CapabilityVersionConstraint,
    };
    use crate::frontend::semantic::capabilities::requirements::CapabilityRequirements;
    use crate::frontend::ast::node::source::span::SourceId;

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("test.target.profile")
            .expect("valid test subject")
    }

    fn origin(issuer: &str) -> EvidenceOrigin {
        EvidenceOrigin::new("test.fixture", issuer, None)
            .expect("valid test origin")
    }

    fn capability_id() -> CapabilityId {
        CapabilityId::new("zamani.quantum", "dynamic_control")
            .expect("valid capability identity")
    }

    fn requirements() -> CapabilityRequirements {
        let capability = Capability::without_metadata(
            NodeId::new(1).expect("valid test node ID"),
            Span::from_usize(
                SourceId::from_raw(1),
                0,
                12,
            )
            .expect("valid test span"),
            "zamani.quantum",
            "dynamic_control",
        )
        .expect("valid AST capability");

        CapabilityRequirements::from_ast([&capability])
            .expect("valid capability requirements")
    }

    fn supported_evidence(
        version: Option<CapabilityVersion>,
    ) -> CapabilityEvidence {
        CapabilityEvidence::new(
            capability_id(),
            subject(),
            CapabilityEvidenceStatus::Supported,
            version,
            std::collections::BTreeMap::new(),
            origin("test-provider"),
        )
        .expect("valid evidence")
    }

    fn evidence_set(
        evidence: impl IntoIterator<Item = CapabilityEvidence>,
    ) -> CapabilityEvidenceSet {
        let mut set = CapabilityEvidenceSet::new();
        set.extend(evidence).expect("valid evidence set");
        set
    }

    #[test]
    fn missing_evidence_is_not_treated_as_satisfied() {
        let requirements = requirements();
        let evidence = CapabilityEvidenceSet::new();

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("empty evidence is valid");

        assert_eq!(report.len(), 1);
        assert_eq!(
            report.iter().next().expect("one result").status(),
            CapabilityCompatibilityStatus::NoEvidence
        );
        assert!(!report.all_satisfied());
        assert!(report.summary().has_unresolved_requirements());
    }

    #[test]
    fn matching_evidence_satisfies_the_requirement() {
        let requirements = requirements();
        let evidence = evidence_set([supported_evidence(None)]);

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("valid evidence");

        assert!(report.all_satisfied());
        assert_eq!(
            report.summary().satisfied(),
            1
        );
    }

    #[test]
    fn evidence_from_another_scope_is_not_combined() {
        let requirements = requirements();
        let other_scope = EvidenceSubject::new("other.target.profile")
            .expect("valid other scope");

        let other_evidence = CapabilityEvidence::new(
            capability_id(),
            other_scope,
            CapabilityEvidenceStatus::Supported,
            None,
            std::collections::BTreeMap::new(),
            origin("other-provider"),
        )
        .expect("valid evidence");

        let evidence = evidence_set([other_evidence]);

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("valid evidence");

        assert_eq!(
            report.iter().next().expect("one result").status(),
            CapabilityCompatibilityStatus::NoEvidence
        );
    }

    #[test]
    fn source_provenance_is_preserved() {
        let requirements = requirements();
        let evidence = evidence_set([supported_evidence(None)]);

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("valid evidence");

        let result = report.iter().next().expect("one result");
        assert_eq!(result.source_node(), NodeId::new(1).expect("node ID"));
        assert_eq!(
            report.get(result.source_node()).map(|item| item.status()),
            Some(CapabilityCompatibilityStatus::Satisfied)
        );
    }

    #[test]
    fn empty_requirements_do_not_prove_compatibility() {
        let requirements = CapabilityRequirements::new();
        let evidence = CapabilityEvidenceSet::new();

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("valid empty evidence");

        assert!(report.is_empty());
        assert!(!report.all_satisfied());
        assert_eq!(report.summary().total(), 0);
    }

    #[test]
    fn version_mismatch_is_reported_as_incompatible() {
        let capability = Capability::with_version(
            NodeId::new(2).expect("valid test node ID"),
            Span::from_usize(
                SourceId::from_raw(1),
                0,
                12,
            )
            .expect("valid test span"),
            NodeMetadata::default(),
            "zamani.quantum",
            "dynamic_control",
            crate::frontend::ast::node::capabilities::capability::CapabilityVersion::new(
                2, 0, 0,
            ),
        )
        .expect("valid AST capability");

        let requirements = CapabilityRequirements::from_ast([&capability])
            .expect("valid requirements");

        let evidence = evidence_set([supported_evidence(Some(
            CapabilityVersion::new(1, 0, 0),
        ))]);

        let report = evaluate_compatibility(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("valid evidence");

        assert_eq!(
            report.iter().next().expect("one result").status(),
            CapabilityCompatibilityStatus::Incompatible
        );
        assert!(report.summary().has_incompatibilities());

        // Keep the version constraint API represented in this test module:
        // a version mismatch is an evidence compatibility result, not a
        // source-language validity decision.
        assert!(CapabilityVersionConstraint::AtLeast(
            CapabilityVersion::new(2, 0, 0)
        )
        .matches(CapabilityVersion::new(2, 1, 0)));
    }
}
