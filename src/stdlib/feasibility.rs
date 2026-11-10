
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Feasibility Assessment
//!
//! Path: `src/stdlib/feasibility.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a target-independent representation of evidence-based
//! feasibility assessments.
//!
//! This module records what available evidence establishes about whether
//! mandatory program requirements and constraints can be satisfied.
//!
//! It does not independently solve arbitrary semantic expressions,
//! evaluate every resource model, discover hardware, select providers,
//! allocate resources, schedule work, route computations, or execute
//! programs.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Feasibility status.
//! - Finding impact.
//! - Structured feasibility evidence.
//! - Individual feasibility findings.
//! - Ordered feasibility reports.
//! - Deterministic report views.
//! - Aggregate status calculation.
//! - Standard-library capability-requirement assessment.
//!
//! This module does NOT own:
//!
//! - Source grammar or parsing.
//! - AST construction or source spans.
//! - Semantic name resolution.
//! - Canonical semantic requirements.
//! - Canonical semantic capability requirements.
//! - Resource quantities, units, or expressions.
//! - Constraint expression evaluation.
//! - Resource constraint solving.
//! - Preference evaluation or ranking.
//! - Hint interpretation.
//! - Authorization.
//! - Hardware discovery.
//! - Device identity.
//! - Resource reservation or allocation.
//! - Provider selection.
//! - Compilation or lowering.
//! - Quantum routing or QEC.
//! - HDL synthesis or timing closure.
//! - Distributed placement.
//! - Runtime scheduling or execution.
//!
//! Those responsibilities remain with their repository authorities.
//!
//! ## Repository integration
//!
//! Standard-library capability contracts:
//!     `src/stdlib/capability.rs`
//!
//! Standard-library requirements:
//!     `src/stdlib/requirement.rs`
//!
//! Standard-library constraints:
//!     `src/stdlib/constraint.rs`
//!
//! Standard-library preferences:
//!     `src/stdlib/preference.rs`
//!
//! Standard-library hints:
//!     `src/stdlib/hint.rs`
//!
//! Canonical semantic capability model:
//!     `src/frontend/semantic/capabilities/model.rs`
//!
//! Canonical semantic capability requirements:
//!     `src/frontend/semantic/capabilities/requirements.rs`
//!
//! Canonical semantic capability evidence:
//!     `src/frontend/semantic/capabilities/evidence.rs`
//!
//! Canonical semantic resource model:
//!     `src/frontend/semantic/resources/model.rs`
//!
//! Canonical semantic resource constraints:
//!     `src/frontend/semantic/resources/constraints.rs`
//!
//! Backend requirement representation:
//!     `src/backend/requirements.rs`
//!
//! Compiler planning and realization:
//!     `src/compiler/`
//!
//! Runtime and physical resource realization:
//!     `src/runtime/` and HAL/provider implementations.
//!
//! ## Fundamental distinctions
//!
//! Feasible:
//!     Available evidence establishes that all assessed mandatory
//!     conditions are satisfied.
//!
//! Infeasible:
//!     Evidence establishes that at least one mandatory condition cannot
//!     be satisfied.
//!
//! Unsupported:
//!     An authoritative assessment explicitly establishes that a required
//!     operation or capability is unsupported in the assessed scope.
//!
//! Unknown:
//!     Evidence is missing, incomplete, conditional, conflicting, or
//!     insufficient to establish a decisive result.
//!
//! Absence of evidence is not proof of unavailability.
//! Unavailability is not automatically proof of permanent unsupportedness.
//! A preference or hint cannot establish feasibility.
//!
//! ## Aggregate status precedence
//!
//! The report applies the following precedence:
//!
//! 1. Any blocking finding produces `Infeasible`.
//! 2. Otherwise, any explicit unsupported finding produces `Unsupported`.
//! 3. Otherwise, any unknown finding produces `Unknown`.
//! 4. Otherwise, at least one substantive satisfied finding produces
//!    `Feasible`.
//! 5. An empty or advisory-only report produces `Unknown`.
//!
//! This precedence preserves proven violations even when other findings
//! are unsupported or unresolved.
//!
//! ## POCO-REAF
//!
//! Feasibility is independent of a closed list of machine architectures,
//! vendors, processors, accelerators, devices, or computational domains.
//!
//! It can represent findings about:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - Hybrid computation.
//! - Distributed computation.
//! - AI and tensor computation.
//! - Networking.
//! - Scientific computation.
//! - Embedded and edge computation.
//! - Future computational domains.
//!
//! This module imposes no arbitrary language-level maximum on:
//!
//! - Finding count.
//! - Evidence count.
//! - Requirement count.
//! - Domain count.
//! - Namespace depth.
//! - Subject identifier length.
//! - Report size.
//!
//! Collections grow according to available memory and explicitly
//! configured operational budgets outside this module.
//!
//! This does not promise literal infinite storage or computation.
//!
//! ## Determinism
//!
//! Reports preserve insertion order.
//! Duplicate findings are preserved.
//! Canonicalized views sort findings deterministically without changing
//! the original report.
//! Capability snapshots already provide deterministic identity ordering.
//!
//! ## Error handling
//!
//! Invalid local assessment records return `FeasibilityError`.
//! An unknown, unsupported, or infeasible assessment is a valid result,
//! not a construction error.

use core::fmt;
use std::error::Error;

use super::capability::{
    CapabilityId,
    CapabilitySnapshot,
    CapabilityStatus,
};

// ============================================================================
// FEASIBILITY STATUS
// ============================================================================

/// Aggregate feasibility status for an assessed set of conditions.
///
/// The status is derived from the findings in a [`FeasibilityReport`].
///
/// It is not a claim that a physical device has been reserved or that
/// execution will necessarily succeed later.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FeasibilityStatus {
    /// Available evidence establishes that the assessed mandatory
    /// conditions are satisfied.
    Feasible,

    /// Evidence establishes that at least one mandatory condition
    /// cannot be satisfied.
    Infeasible,

    /// An authoritative assessment explicitly establishes that a
    /// required operation or capability is unsupported in scope.
    Unsupported,

    /// Available evidence is insufficient to establish a decisive result.
    Unknown,
}

impl FeasibilityStatus {
    /// Returns whether the assessment establishes feasibility.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Returns whether the assessment establishes infeasibility.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Returns whether the assessment explicitly establishes unsupportedness.
    #[must_use]
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::Unsupported)
    }

    /// Returns whether the assessment remains unresolved.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether this status establishes that the requested
    /// realization cannot currently be accepted.
    ///
    /// This does not mean that the source program is semantically invalid.
    #[must_use]
    pub const fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Infeasible | Self::Unsupported)
    }
}

impl fmt::Display for FeasibilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// FINDING IMPACT
// ============================================================================

/// Describes how an individual finding contributes to aggregate feasibility.
///
/// The impact is independent of the domain-specific meaning of the finding.
/// For example, a capability assessment, a resource constraint assessment,
/// and a quantum-domain assessment can all use the same impact contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FeasibilityImpact {
    /// Evidence establishes that the assessed mandatory condition holds.
    Satisfied,

    /// Evidence establishes a violation of a mandatory condition.
    ///
    /// A blocking finding contributes to `Infeasible`.
    Blocking,

    /// Evidence explicitly establishes that a required operation or
    /// capability is unsupported in the assessed scope.
    Unsupported,

    /// Evidence is missing, conditional, conflicting, or insufficient.
    ///
    /// An unknown finding prevents a positive feasibility conclusion.
    Unknown,

    /// Non-binding information that does not determine feasibility.
    ///
    /// Advisory findings must not change aggregate feasibility status.
    Advisory,
}

impl FeasibilityImpact {
    /// Returns whether this impact establishes a mandatory-condition violation.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Blocking)
    }

    /// Returns whether this impact explicitly establishes unsupportedness.
    #[must_use]
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::Unsupported)
    }

    /// Returns whether this impact requires additional evidence.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether this impact is advisory and non-binding.
    #[must_use]
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::Advisory)
    }
}

// ============================================================================
// EVIDENCE
// ============================================================================

/// A validated evidence record supporting an individual finding.
///
/// Evidence describes where an assessment came from and what the source
/// established. It does not independently establish trustworthiness,
/// authorization, freshness, or correctness.
///
/// The responsible semantic, compiler, runtime, or provider subsystem
/// remains responsible for validating evidence according to its own rules.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FeasibilityEvidence {
    source: String,
    description: String,
    reference: Option<String>,
}

impl FeasibilityEvidence {
    /// Creates a validated evidence record.
    ///
    /// # Errors
    ///
    /// Returns [`FeasibilityError`] if the source or description is invalid.
    pub fn new(
        source: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        let source = source.into();
        let description = description.into();

        validate_text(&source, "evidence source")?;
        validate_text(&description, "evidence description")?;

        Ok(Self {
            source,
            description,
            reference: None,
        })
    }

    /// Adds an optional stable reference to the originating evidence.
    ///
    /// The reference may identify a source node, provider record,
    /// evidence object, report, or another object understood by the
    /// owning integration.
    ///
    /// This module does not interpret the reference.
    ///
    /// # Errors
    ///
    /// Returns [`FeasibilityError`] if the reference is invalid.
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        let reference = reference.into();

        validate_text(&reference, "evidence reference")?;

        self.reference = Some(reference);

        Ok(self)
    }

    /// Returns the evidence source identifier.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the evidence description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the optional evidence reference.
    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }
}

// ============================================================================
// FINDINGS
// ============================================================================

/// One evidence-based feasibility finding.
///
/// `category` is an open-world identifier supplied by the responsible
/// subsystem. It may identify requirement, capability, resource, security,
/// quantum, HDL, distributed, or future-domain assessments.
///
/// `subject` optionally identifies the specific condition being assessed.
///
/// This type does not parse or interpret either value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    category: String,
    subject: Option<String>,
    impact: FeasibilityImpact,
    message: String,
    evidence: Vec<FeasibilityEvidence>,
}

impl FeasibilityFinding {
    /// Creates a validated finding.
    ///
    /// # Errors
    ///
    /// Returns [`FeasibilityError`] if the category, subject, or message
    /// is invalid.
    pub fn new(
        category: impl Into<String>,
        subject: Option<String>,
        impact: FeasibilityImpact,
        message: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        let category = category.into();
        let message = message.into();

        validate_text(&category, "finding category")?;
        validate_text(&message, "finding message")?;

        if let Some(subject) = subject.as_deref() {
            validate_text(subject, "finding subject")?;
        }

        Ok(Self {
            category,
            subject,
            impact,
            message,
            evidence: Vec::new(),
        })
    }

    /// Attaches evidence while preserving the finding.
    #[must_use]
    pub fn with_evidence(
        mut self,
        evidence: FeasibilityEvidence,
    ) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Attaches multiple evidence records in their supplied order.
    pub fn extend_evidence(
        &mut self,
        evidence: impl IntoIterator<Item = FeasibilityEvidence>,
    ) {
        self.evidence.extend(evidence);
    }

    /// Returns the open-world finding category.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns the optional assessed subject.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Returns the finding impact.
    #[must_use]
    pub const fn impact(&self) -> FeasibilityImpact {
        self.impact
    }

    /// Returns the human-readable explanation.
    ///
    /// Consumers should inspect [`Self::impact`] rather than parse this text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns attached evidence in insertion order.
    #[must_use]
    pub fn evidence(&self) -> &[FeasibilityEvidence] {
        &self.evidence
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// An ordered collection of feasibility findings.
///
/// A report preserves:
///
/// - Finding insertion order.
/// - Duplicate findings.
/// - Findings from independent evidence sources.
/// - Contradictory evidence.
/// - Unknown and unsupported findings.
/// - Advisory information.
///
/// The report does not deduplicate, solve, normalize, or reinterpret findings.
///
/// There is no fixed report capacity. Collection growth is governed by
/// available memory and explicit operational budgets outside this module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FeasibilityReport {
    findings: Vec<FeasibilityFinding>,
}

impl FeasibilityReport {
    /// Creates an empty report.
    ///
    /// An empty report has `Unknown` status because no mandatory condition
    /// has been assessed.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            findings: Vec::new(),
        }
    }

    /// Creates a report from existing findings.
    ///
    /// The supplied order and duplicate findings are preserved.
    #[must_use]
    pub fn from_findings(
        findings: impl IntoIterator<Item = FeasibilityFinding>,
    ) -> Self {
        Self {
            findings: findings.into_iter().collect(),
        }
    }

    /// Adds a finding.
    pub fn push(&mut self, finding: FeasibilityFinding) {
        self.findings.push(finding);
    }

    /// Adds a finding and returns the updated report.
    #[must_use]
    pub fn with_finding(
        mut self,
        finding: FeasibilityFinding,
    ) -> Self {
        self.push(finding);
        self
    }

    /// Appends another report, preserving order and duplicates.
    pub fn merge(&mut self, other: Self) {
        self.findings.extend(other.findings);
    }

    /// Appends findings from an iterator.
    pub fn extend(
        &mut self,
        findings: impl IntoIterator<Item = FeasibilityFinding>,
    ) {
        self.findings.extend(findings);
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.findings.len()
    }

    /// Returns whether the report has no findings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// Returns findings in insertion order.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Returns an iterator over findings in insertion order.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, FeasibilityFinding> {
        self.findings.iter()
    }

    /// Consumes the report and returns findings in insertion order.
    #[must_use]
    pub fn into_findings(self) -> Vec<FeasibilityFinding> {
        self.findings
    }

    /// Returns the aggregate feasibility status.
    ///
    /// Precedence:
    ///
    /// 1. Blocking findings produce `Infeasible`.
    /// 2. Explicit unsupported findings produce `Unsupported`.
    /// 3. Unknown findings produce `Unknown`.
    /// 4. At least one substantive satisfied finding produces `Feasible`.
    /// 5. Empty or advisory-only reports produce `Unknown`.
    #[must_use]
    pub fn status(&self) -> FeasibilityStatus {
        let mut has_blocking = false;
        let mut has_unsupported = false;
        let mut has_unknown = false;
        let mut has_satisfied = false;

        for finding in &self.findings {
            match finding.impact() {
                FeasibilityImpact::Blocking => {
                    has_blocking = true;
                }
                FeasibilityImpact::Unsupported => {
                    has_unsupported = true;
                }
                FeasibilityImpact::Unknown => {
                    has_unknown = true;
                }
                FeasibilityImpact::Satisfied => {
                    has_satisfied = true;
                }
                FeasibilityImpact::Advisory => {}
            }
        }

        if has_blocking {
            FeasibilityStatus::Infeasible
        } else if has_unsupported {
            FeasibilityStatus::Unsupported
        } else if has_unknown || !has_satisfied {
            FeasibilityStatus::Unknown
        } else {
            FeasibilityStatus::Feasible
        }
    }

    /// Returns whether the report establishes feasibility.
    #[must_use]
    pub fn is_feasible(&self) -> bool {
        self.status().is_feasible()
    }

    /// Returns whether the report establishes infeasibility.
    #[must_use]
    pub fn is_infeasible(&self) -> bool {
        self.status().is_infeasible()
    }

    /// Returns whether the report establishes unsupportedness.
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        self.status().is_unsupported()
    }

    /// Returns whether the report remains unresolved.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.status().is_unknown()
    }

    /// Returns whether the report establishes that acceptance is blocked.
    ///
    /// Unknown reports are not classified as proven violations, but they
    /// must not be treated as proof of feasibility.
    #[must_use]
    pub fn blocks_acceptance(&self) -> bool {
        self.status().blocks_acceptance()
    }

    /// Returns whether the report contains any blocking finding.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {
        self.findings.iter().any(|finding| {
            finding.impact().is_blocking()
        })
    }

    /// Returns whether the report contains explicit unsupported findings.
    #[must_use]
    pub fn has_unsupported_findings(&self) -> bool {
        self.findings.iter().any(|finding| {
            finding.impact().is_unsupported()
        })
    }

    /// Returns whether the report requires additional evidence.
    #[must_use]
    pub fn needs_more_evidence(&self) -> bool {
        self.findings.iter().any(|finding| {
            finding.impact().is_unknown()
        }) || self.is_empty()
    }

    /// Returns whether the report contains at least one substantive finding.
    ///
    /// Advisory-only and empty reports are not complete assessments.
    #[must_use]
    pub fn has_substantive_findings(&self) -> bool {
        self.findings.iter().any(|finding| {
            !finding.impact().is_advisory()
        })
    }

    /// Returns whether every recorded assessment is decisive.
    ///
    /// A report is complete when it is non-empty, contains at least one
    /// substantive finding, and contains no unknown findings.
    ///
    /// `Infeasible` and `Unsupported` reports can be complete: they are
    /// decisive negative assessments.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.is_empty()
            && self.has_substantive_findings()
            && !self.needs_more_evidence()
    }

    /// Counts findings having the specified impact.
    #[must_use]
    pub fn count_impact(
        &self,
        impact: FeasibilityImpact,
    ) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.impact() == impact)
            .count()
    }

    /// Returns a deterministically ordered view without modifying the report.
    ///
    /// Findings with equal sort keys retain their relative insertion order.
    #[must_use]
    pub fn canonicalized(&self) -> Vec<&FeasibilityFinding> {
        let mut findings: Vec<&FeasibilityFinding> =
            self.findings.iter().collect();

        findings.sort_by(|left, right| {
            left.category()
                .cmp(right.category())
                .then_with(|| left.subject().cmp(&right.subject()))
                .then_with(|| left.impact().cmp(&right.impact()))
                .then_with(|| left.message().cmp(right.message()))
                .then_with(|| left.evidence().cmp(right.evidence()))
        });

        findings
    }

    /// Creates a report by assessing mandatory capability requirements
    /// against a standard-library capability snapshot.
    ///
    /// Each supplied capability identity is treated as a separate mandatory
    /// obligation. Duplicate identities are deliberately preserved.
    ///
    /// Capability availability is assessed only within the supplied
    /// snapshot. It does not reserve resources or guarantee future runtime
    /// availability.
    ///
    /// An absent capability entry is `Unknown`, not `Unsupported`.
    ///
    /// The snapshot's `Unavailable` state produces a blocking finding.
    /// The snapshot does not have an explicit unsupported state; therefore
    /// this method never invents an unsupported result.
    ///
    /// # Errors
    ///
    /// Returns [`FeasibilityError`] if an evidence record cannot be
    /// represented as a valid feasibility evidence record.
    pub fn from_capability_requirements<'a>(
        snapshot: &CapabilitySnapshot,
        requirements: impl IntoIterator<Item = &'a CapabilityId>,
    ) -> Result<Self, FeasibilityError> {
        let mut report = Self::new();

        for requirement in requirements {
            let subject = requirement.as_str().to_owned();

            match snapshot.get(requirement) {
                None => {
                    report.push(FeasibilityFinding::new(
                        "capability.missing_evidence",
                        Some(subject),
                        FeasibilityImpact::Unknown,
                        "No capability state was supplied. Absence of a snapshot entry does not establish that the capability is unsupported.",
                    )?);
                }

                Some(state) => {
                    let (category, impact, message) = match state.status() {
                        CapabilityStatus::Available => (
                            "capability.available",
                            FeasibilityImpact::Satisfied,
                            "The capability snapshot establishes availability.",
                        ),

                        CapabilityStatus::Unavailable => (
                            "capability.unavailable",
                            FeasibilityImpact::Blocking,
                            "The capability snapshot establishes unavailability in the assessed scope.",
                        ),

                        CapabilityStatus::Conditional => (
                            "capability.conditional",
                            FeasibilityImpact::Unknown,
                            "The capability is conditional. Its conditions must be evaluated by the responsible subsystem.",
                        ),

                        CapabilityStatus::Unknown => (
                            "capability.unknown",
                            FeasibilityImpact::Unknown,
                            "The capability snapshot does not establish availability or unavailability.",
                        ),
                    };

                    let mut finding = FeasibilityFinding::new(
                        category,
                        Some(subject),
                        impact,
                        message,
                    )?;

                    for evidence in state.evidence() {
                        finding.extend_evidence([
                            FeasibilityEvidence::new(
                                evidence.source(),
                                evidence.description(),
                            )?,
                        ]);
                    }

                    report.push(finding);
                }
            }
        }

        Ok(report)
    }
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Identifies invalid local feasibility data.
///
/// These errors concern malformed assessment records. They are not
/// feasibility outcomes and must not be confused with `Infeasible`,
/// `Unsupported`, or `Unknown`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FeasibilityError {
    /// A required text field was empty or contained only whitespace.
    EmptyField(&'static str),

    /// A text field contained leading or trailing whitespace.
    SurroundingWhitespace(&'static str),

    /// A text field contained a control character.
    ControlCharacter(&'static str),
}

impl fmt::Display for FeasibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => {
                write!(formatter, "{field} must not be empty")
            }
            Self::SurroundingWhitespace(field) => {
                write!(
                    formatter,
                    "{field} must not contain leading or trailing whitespace"
                )
            }
            Self::ControlCharacter(field) => {
                write!(
                    formatter,
                    "{field} must not contain control characters"
                )
            }
        }
    }
}

impl Error for FeasibilityError {}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), FeasibilityError> {
    if value.trim().is_empty() {
        return Err(FeasibilityError::EmptyField(field));
    }

    if value.trim() != value {
        return Err(FeasibilityError::SurroundingWhitespace(field));
    }

    if value.chars().any(char::is_control) {
        return Err(FeasibilityError::ControlCharacter(field));
    }

    Ok(())
}

// ============================================================================
// ITERATION
// ============================================================================

impl IntoIterator for FeasibilityReport {
    type Item = FeasibilityFinding;
    type IntoIter = std::vec::IntoIter<FeasibilityFinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.findings.into_iter()
    }
}

impl<'a> IntoIterator for &'a FeasibilityReport {
    type Item = &'a FeasibilityFinding;
    type IntoIter = core::slice::Iter<'a, FeasibilityFinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.findings.iter()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        Capability,
        CapabilityEvidence,
        CapabilityKind,
        CapabilityState,
    };

    fn finding(
        category: &str,
        impact: FeasibilityImpact,
    ) -> FeasibilityFinding {
        FeasibilityFinding::new(
            category,
            None,
            impact,
            "test assessment",
        )
        .expect("test finding should be valid")
    }

    fn capability_id(value: &str) -> CapabilityId {
        CapabilityId::new(value)
            .expect("test capability identity should be valid")
    }

    fn capability_state(
        id: &str,
        status: CapabilityStatus,
    ) -> CapabilityState {
        let capability = Capability::new(
            capability_id(id),
            CapabilityKind::new("computational")
                .expect("test capability kind should be valid"),
        );

        CapabilityState::new(capability, status)
    }

    #[test]
    fn empty_report_is_unknown() {
        let report = FeasibilityReport::new();

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
        assert!(!report.is_feasible());
        assert!(!report.is_complete());
        assert!(report.needs_more_evidence());
    }

    #[test]
    fn satisfied_findings_establish_feasibility() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "requirement.satisfied",
                FeasibilityImpact::Satisfied,
            ))
            .with_finding(finding(
                "constraint.satisfied",
                FeasibilityImpact::Satisfied,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
        assert!(report.is_feasible());
        assert!(report.is_complete());
    }

    #[test]
    fn blocking_finding_establishes_infeasibility() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "requirement.satisfied",
                FeasibilityImpact::Satisfied,
            ))
            .with_finding(finding(
                "constraint.violated",
                FeasibilityImpact::Blocking,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Infeasible);
        assert!(report.blocks_acceptance());
        assert!(report.is_complete());
    }

    #[test]
    fn explicit_unsupported_finding_is_not_unknown() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "domain.unsupported",
                FeasibilityImpact::Unsupported,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Unsupported);
        assert!(report.is_unsupported());
        assert!(report.is_complete());
    }

    #[test]
    fn unknown_finding_prevents_positive_feasibility() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "requirement.satisfied",
                FeasibilityImpact::Satisfied,
            ))
            .with_finding(finding(
                "resource.unknown",
                FeasibilityImpact::Unknown,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
        assert!(!report.is_feasible());
        assert!(!report.is_complete());
    }

    #[test]
    fn advisory_finding_does_not_change_feasibility() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "requirement.satisfied",
                FeasibilityImpact::Satisfied,
            ))
            .with_finding(finding(
                "optimization.advisory",
                FeasibilityImpact::Advisory,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn advisory_only_report_is_unknown() {
        let report = FeasibilityReport::new()
            .with_finding(finding(
                "optimization.advisory",
                FeasibilityImpact::Advisory,
            ));

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
        assert!(!report.is_complete());
    }

    #[test]
    fn blocking_takes_precedence_over_unsupported_and_unknown() {
        let report = FeasibilityReport::from_findings([
            finding("domain.unsupported", FeasibilityImpact::Unsupported),
            finding("resource.unknown", FeasibilityImpact::Unknown),
            finding("constraint.violated", FeasibilityImpact::Blocking),
        ]);

        assert_eq!(report.status(), FeasibilityStatus::Infeasible);
    }

    #[test]
    fn unavailable_capability_is_blocking() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(capability_state(
                "classical.parallel_execution",
                CapabilityStatus::Unavailable,
            ))
            .expect("unique capability should be inserted");

        let required = capability_id("classical.parallel_execution");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Infeasible);
        assert_eq!(report.len(), 1);
    }

    #[test]
    fn available_capability_satisfies_its_assessed_requirement() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(capability_state(
                "quantum.measurement",
                CapabilityStatus::Available,
            ))
            .expect("unique capability should be inserted");

        let required = capability_id("quantum.measurement");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn absent_capability_is_unknown_not_unsupported() {
        let snapshot = CapabilitySnapshot::new();
        let required = capability_id("future.computing.operation");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
        assert!(!report.is_unsupported());
    }

    #[test]
    fn conditional_capability_remains_unresolved() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(capability_state(
                "hdl.synthesis",
                CapabilityStatus::Conditional,
            ))
            .expect("unique capability should be inserted");

        let required = capability_id("hdl.synthesis");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
        assert!(report.needs_more_evidence());
    }

    #[test]
    fn unknown_capability_state_is_not_unavailable() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(capability_state(
                "distributed.collective_communication",
                CapabilityStatus::Unknown,
            ))
            .expect("unique capability should be inserted");

        let required =
            capability_id("distributed.collective_communication");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Unknown);
    }

    #[test]
    fn duplicate_capability_requirements_are_preserved() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(capability_state(
                "classical.parallel_execution",
                CapabilityStatus::Available,
            ))
            .expect("unique capability should be inserted");

        let required = capability_id("classical.parallel_execution");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required, &required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.len(), 2);
        assert_eq!(report.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn capability_evidence_is_preserved() {
        let mut snapshot = CapabilitySnapshot::new();

        let evidence = CapabilityEvidence::new(
            "runtime.provider",
            "Provider reported measurement support.",
        )
        .expect("test evidence should be valid");

        let state = capability_state(
            "quantum.measurement",
            CapabilityStatus::Available,
        )
        .with_evidence(evidence);

        snapshot
            .insert(state)
            .expect("unique capability should be inserted");

        let required = capability_id("quantum.measurement");

        let report = FeasibilityReport::from_capability_requirements(
            &snapshot,
            [&required],
        )
        .expect("capability assessment should succeed");

        assert_eq!(report.findings()[0].evidence().len(), 1);
        assert_eq!(
            report.findings()[0].evidence()[0].source(),
            "runtime.provider",
        );
    }

    #[test]
    fn report_merge_preserves_findings() {
        let mut first = FeasibilityReport::new();
        first.push(finding(
            "requirement.satisfied",
            FeasibilityImpact::Satisfied,
        ));

        let second = FeasibilityReport::new()
            .with_finding(finding(
                "constraint.satisfied",
                FeasibilityImpact::Satisfied,
            ));

        first.merge(second);

        assert_eq!(first.len(), 2);
        assert_eq!(first.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let report = FeasibilityReport::from_findings([
            FeasibilityFinding::new(
                "zeta",
                None,
                FeasibilityImpact::Satisfied,
                "second",
            )
            .expect("valid finding"),
            FeasibilityFinding::new(
                "alpha",
                None,
                FeasibilityImpact::Satisfied,
                "first",
            )
            .expect("valid finding"),
        ]);

        let canonical = report.canonicalized();

        assert_eq!(canonical[0].category(), "alpha");
        assert_eq!(canonical[1].category(), "zeta");
    }

    #[test]
    fn invalid_finding_text_is_rejected() {
        assert!(matches!(
            FeasibilityFinding::new(
                "",
                None,
                FeasibilityImpact::Unknown,
                "message",
            ),
            Err(FeasibilityError::EmptyField("finding category")),
        ));

        assert!(matches!(
            FeasibilityFinding::new(
                "resource.unknown",
                None,
                FeasibilityImpact::Unknown,
                " message",
            ),
            Err(FeasibilityError::SurroundingWhitespace(
                "finding message"
            )),
        ));
    }

    #[test]
    fn invalid_evidence_is_rejected() {
        assert!(matches!(
            FeasibilityEvidence::new(
                "provider",
                "",
            ),
            Err(FeasibilityError::EmptyField(
                "evidence description"
            )),
        ));
    }

    #[test]
    fn unsupported_and_unknown_are_not_infeasible() {
        let unsupported = FeasibilityReport::new()
            .with_finding(finding(
                "domain.unsupported",
                FeasibilityImpact::Unsupported,
            ));

        let unknown = FeasibilityReport::new()
            .with_finding(finding(
                "resource.unknown",
                FeasibilityImpact::Unknown,
            ));

        assert!(!unsupported.is_infeasible());
        assert!(!unknown.is_infeasible());
        assert!(unsupported.blocks_acceptance());
        assert!(!unknown.blocks_acceptance());
    }
}
