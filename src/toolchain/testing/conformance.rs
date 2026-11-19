
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Conformance Framework
//!
//! Path: src/toolchain/testing/conformance.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a reusable, evidence-based framework for evaluating whether
//! Zamani toolchain implementations conform to their applicable
//! architectural, language, compiler, portability, security and
//! operational contracts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Conformance requirement descriptions.
//! - Conformance check identities.
//! - Evidence references and provenance.
//! - Evidence outcome classification.
//! - Conformance finding aggregation.
//! - Conformance report validation.
//! - Fail-closed conformance decisions.
//! - Configurable conformance suites.
//! - A baseline Zamani toolchain conformance profile.
//!
//! This module does not own:
//!
//! - Language grammar or language semantics.
//! - Semantic capability or resource analysis.
//! - Compiler correctness verification algorithms.
//! - Backend implementation verification algorithms.
//! - Physical hardware discovery.
//! - Runtime execution verification.
//! - Cryptographic signature verification.
//! - Artifact portability assessment.
//! - Package compatibility assessment.
//! - Dependency resolution.
//! - External process execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! Architecture:
//!     grammar/DESIGN.md
//!
//! POCO-REAF:
//!     grammar/specification/poco-reaf.md
//!
//! Scalability:
//!     grammar/specification/scalability-model.md
//!
//! Semantic boundaries:
//!     grammar/validation/semantic-boundaries.md
//!
//! Grammar composition:
//!     grammar/Zamani.g4
//!
//! Lexer:
//!     grammar/antlr/ZamaniLexer.g4
//!
//! Parser:
//!     grammar/antlr/ZamaniParser.g4
//!
//! Grammar inventory:
//!     grammar/MANIFEST.yaml
//!
//! Compiler:
//!     crate::compiler
//!
//! Backend:
//!     crate::backend
//!
//! Artifact portability:
//!     crate::compiler::artifacts::portability
//!
//! Build planning:
//!     crate::toolchain::build::plan
//!
//! Build execution:
//!     crate::toolchain::build::executor
//!
//! Compatibility:
//!     crate::toolchain::api::compatibility
//!
//! Manifest:
//!     crate::zamani_project_config
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! ## Critical conformance rule
//!
//! Conformance requires evidence.
//!
//! An absent check, missing evidence, inconclusive evidence or
//! unverified claim MUST NOT be interpreted as success.
//!
//! A successful result means that the supplied evidence supports
//! the specific requirement under its stated authority and scope.
//!
//! This module records and evaluates evidence. It does not
//! independently authenticate external evidence or prove that
//! an evidence producer is trustworthy.
//!
//! ## POCO-REAF
//!
//! One logical source program may have multiple compatible
//! target-specific realizations.
//!
//! Conformance must preserve the distinction between:
//!
//! - Source-language validity.
//! - Semantic validity.
//! - Compilation feasibility.
//! - Target compatibility.
//! - Artifact verification.
//! - Portability verification.
//! - Runtime availability.
//! - Successful execution.
//!
//! Passing one stage does not establish the others.
//!
//! ## Scalability
//!
//! No application-defined universal maximum is imposed on:
//!
//! - Conformance requirements.
//! - Evidence records.
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - Build tasks.
//! - Projects.
//! - Dependencies.
//! - Artifacts.
//! - Distributed participants.
//! - Hardware resources.
//!
//! Collections grow according to available resources and
//! Rust representation limits.
//!
//! Explicit operational budgets may be applied by callers.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - Evidence is treated as untrusted input.
//! - Evidence identifiers must be structurally validated.
//! - Duplicate check identities are rejected.
//! - Unknown evidence references are reported.
//! - Evidence from a different authority cannot satisfy a check.
//! - Credentials and secrets must not be stored in evidence.
//! - A failure cannot become a passing result.
//!
//! ## Concurrency
//!
//! This module has no global mutable state.
//!
//! It introduces no worker pool, background thread,
//! scheduler or resource allocator.
//!
//! Independent suites and reports can be evaluated concurrently.
//!
//! ## Versioning
//!
//! The conformance contract has its own version.
//!
//! It does not redefine language, grammar, compiler, IR,
//! ABI, manifest, package or provider versions.

// ============================================================================
// CONTRACT IDENTITY
// ============================================================================

/// Stable identity of the conformance contract.
pub const CONFORMANCE_CONTRACT_ID: &str =
    "zamani.toolchain.testing.conformance";

/// Major version of the conformance contract.
pub const CONFORMANCE_CONTRACT_MAJOR: u32 = 1;

/// Minor version of the conformance contract.
pub const CONFORMANCE_CONTRACT_MINOR: u32 = 0;

/// Patch version of the conformance contract.
pub const CONFORMANCE_CONTRACT_PATCH: u32 = 0;

/// Return the conformance contract identity.
#[must_use]
pub const fn contract_id() -> &'static str {
    CONFORMANCE_CONTRACT_ID
}

/// Return the conformance contract version.
#[must_use]
pub const fn contract_version() -> (u32, u32, u32) {
    (
        CONFORMANCE_CONTRACT_MAJOR,
        CONFORMANCE_CONTRACT_MINOR,
        CONFORMANCE_CONTRACT_PATCH,
    )
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Errors raised when constructing or evaluating a conformance suite.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceError {
    /// A required identifier or descriptive field is invalid.
    InvalidInput {
        field: String,
        reason: String,
    },

    /// A check identity already exists in the suite.
    DuplicateCheck {
        check_id: ConformanceCheckId,
    },

    /// An evidence record has no corresponding requirement.
    UnknownCheck {
        check_id: ConformanceCheckId,
    },

    /// Multiple evidence records were supplied for one check.
    DuplicateEvidence {
        check_id: ConformanceCheckId,
    },

    /// Evidence identifies a different authority from the requirement.
    AuthorityMismatch {
        check_id: ConformanceCheckId,
        expected: String,
        actual: String,
    },

    /// A report contains contradictory or structurally invalid data.
    InvalidReport {
        reason: String,
    },
}

impl std::fmt::Display for ConformanceError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidInput { field, reason } => {
                write!(
                    formatter,
                    "invalid conformance {field}: {reason}"
                )
            }

            Self::DuplicateCheck { check_id } => {
                write!(
                    formatter,
                    "duplicate conformance check: {check_id}"
                )
            }

            Self::UnknownCheck { check_id } => {
                write!(
                    formatter,
                    "evidence references an unknown check: {check_id}"
                )
            }

            Self::DuplicateEvidence { check_id } => {
                write!(
                    formatter,
                    "duplicate evidence for check: {check_id}"
                )
            }

            Self::AuthorityMismatch {
                check_id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "authority mismatch for {check_id}: \
                     expected '{expected}', received '{actual}'"
                )
            }

            Self::InvalidReport { reason } => {
                write!(
                    formatter,
                    "invalid conformance report: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ConformanceError {}

/// Result type for conformance-framework operations.
pub type ConformanceResult<T> =
    std::result::Result<T, ConformanceError>;

// ============================================================================
// INPUT VALIDATION
// ============================================================================

fn validate_text(
    field: &str,
    value: &str,
) -> ConformanceResult<()> {
    if value.trim().is_empty() {
        return Err(ConformanceError::InvalidInput {
            field: field.to_owned(),
            reason: "value must not be empty".to_owned(),
        });
    }

    if value != value.trim() {
        return Err(ConformanceError::InvalidInput {
            field: field.to_owned(),
            reason: "surrounding whitespace is not permitted".to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ConformanceError::InvalidInput {
            field: field.to_owned(),
            reason: "control characters are not permitted".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// OPEN CONFORMANCE IDENTITIES
// ============================================================================

/// Stable, open-ended identity for one conformance requirement.
///
/// The identifier is not a closed enumeration.
///
/// Future domains, providers and specifications may introduce
/// additional identifiers without modifying this type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ConformanceCheckId(String);

impl ConformanceCheckId {
    /// Construct a validated conformance check identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ConformanceResult<Self> {
        let value = value.into();

        validate_text("check identifier", &value)?;

        Ok(Self(value))
    }

    /// Return the underlying identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConformanceCheckId {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Identity of a specification or architectural authority.
///
/// Examples include grammar specification paths, repository
/// contract identifiers and versioned provider contracts.
///
/// Authority identifiers remain open-ended.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ConformanceAuthority(String);

impl ConformanceAuthority {
    /// Construct an authority identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ConformanceResult<Self> {
        let value = value.into();

        validate_text("authority", &value)?;

        Ok(Self(value))
    }

    /// Return the authority identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConformanceAuthority {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// REQUIREMENT CLASSIFICATION
// ============================================================================

/// Importance of a conformance requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementLevel {
    /// Mandatory requirement.
    ///
    /// Failure or unresolved evidence prevents a conformant result.
    Mandatory,

    /// Strongly recommended requirement.
    ///
    /// Its failure is reported but does not independently invalidate
    /// mandatory conformance.
    Recommended,
}

/// Applicability of a conformance requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Applicability {
    /// The requirement applies to the evaluated implementation.
    Applicable,

    /// The requirement is explicitly outside the evaluated scope.
    ///
    /// This state requires a documented rationale.
    NotApplicable,
}

/// Evidence outcome supplied by an authoritative evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceOutcome {
    /// The evidence supports satisfaction of the requirement.
    Satisfied,

    /// The evidence demonstrates a violation.
    Violated,

    /// The available evidence cannot establish either satisfaction
    /// or violation.
    Inconclusive,

    /// The authority explicitly establishes that the requirement
    /// does not apply to the evaluated scope.
    NotApplicable,
}

/// Final state of one evaluated conformance check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FindingStatus {
    /// The applicable requirement has sufficient supporting evidence.
    Passed,

    /// The requirement has been demonstrably violated.
    Failed,

    /// Required evidence is missing or inconclusive.
    Blocked,

    /// The requirement is explicitly outside the evaluated scope.
    NotApplicable,
}

/// Overall state of a conformance evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConformanceStatus {
    /// All mandatory applicable requirements have passed.
    Conformant,

    /// At least one mandatory requirement has failed.
    NonConformant,

    /// No mandatory failure is established, but mandatory evidence
    /// remains missing, inconclusive or otherwise unresolved.
    Inconclusive,
}

// ============================================================================
// REQUIREMENT DEFINITION
// ============================================================================

/// One immutable conformance requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceRequirement {
    id: ConformanceCheckId,
    authority: ConformanceAuthority,
    title: String,
    statement: String,
    level: RequirementLevel,
}

impl ConformanceRequirement {
    /// Create a conformance requirement.
    pub fn new(
        id: ConformanceCheckId,
        authority: ConformanceAuthority,
        title: impl Into<String>,
        statement: impl Into<String>,
        level: RequirementLevel,
    ) -> ConformanceResult<Self> {
        let title = title.into();
        let statement = statement.into();

        validate_text("requirement title", &title)?;
        validate_text("requirement statement", &statement)?;

        Ok(Self {
            id,
            authority,
            title,
            statement,
            level,
        })
    }

    /// Return the requirement identity.
    #[must_use]
    pub fn id(&self) -> &ConformanceCheckId {
        &self.id
    }

    /// Return the normative authority.
    #[must_use]
    pub fn authority(&self) -> &ConformanceAuthority {
        &self.authority
    }

    /// Return the requirement title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the requirement statement.
    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Return the requirement importance.
    #[must_use]
    pub const fn level(&self) -> RequirementLevel {
        self.level
    }
}

// ============================================================================
// EVIDENCE
// ============================================================================

/// Evidence supporting one conformance decision.
///
/// Evidence references identify the source of the assertion.
///
/// This type does not authenticate the referenced source.
/// External verification and trust decisions remain with their
/// respective authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceEvidence {
    check_id: ConformanceCheckId,
    authority: ConformanceAuthority,
    outcome: EvidenceOutcome,
    reference: String,
    method: String,
    revision: String,
    rationale: String,
}

impl ConformanceEvidence {
    /// Construct an evidence record.
    ///
    /// `reference` should identify an actual test, report, specification
    /// section, commit, build record or verification artifact.
    ///
    /// `method` explains how the evidence was obtained.
    ///
    /// `revision` identifies the implementation or contract revision
    /// to which the evidence applies.
    ///
    /// `rationale` records the result and relevant limitations.
    pub fn new(
        check_id: ConformanceCheckId,
        authority: ConformanceAuthority,
        outcome: EvidenceOutcome,
        reference: impl Into<String>,
        method: impl Into<String>,
        revision: impl Into<String>,
        rationale: impl Into<String>,
    ) -> ConformanceResult<Self> {
        let reference = reference.into();
        let method = method.into();
        let revision = revision.into();
        let rationale = rationale.into();

        validate_text("evidence reference", &reference)?;
        validate_text("evidence method", &method)?;
        validate_text("evidence revision", &revision)?;
        validate_text("evidence rationale", &rationale)?;

        Ok(Self {
            check_id,
            authority,
            outcome,
            reference,
            method,
            revision,
            rationale,
        })
    }

    /// Return the associated requirement identity.
    #[must_use]
    pub fn check_id(&self) -> &ConformanceCheckId {
        &self.check_id
    }

    /// Return the evidence authority.
    #[must_use]
    pub fn authority(&self) -> &ConformanceAuthority {
        &self.authority
    }

    /// Return the evidence outcome.
    #[must_use]
    pub const fn outcome(&self) -> EvidenceOutcome {
        self.outcome
    }

    /// Return the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Return the verification method.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Return the associated implementation or contract revision.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Return the evidence rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

// ============================================================================
// FINDINGS
// ============================================================================

/// One evaluated conformance finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceFinding {
    requirement: ConformanceRequirement,
    status: FindingStatus,
    evidence: Option<ConformanceEvidence>,
    explanation: String,
}

impl ConformanceFinding {
    /// Return the evaluated requirement.
    #[must_use]
    pub fn requirement(&self) -> &ConformanceRequirement {
        &self.requirement
    }

    /// Return the final finding status.
    #[must_use]
    pub const fn status(&self) -> FindingStatus {
        self.status
    }

    /// Return supporting evidence when available.
    #[must_use]
    pub fn evidence(&self) -> Option<&ConformanceEvidence> {
        self.evidence.as_ref()
    }

    /// Return the finding explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

// ============================================================================
// CONFORMANCE SUITE
// ============================================================================

/// Configurable collection of independently owned requirements.
///
/// The suite does not execute compilers, parsers, hardware providers
/// or external processes.
///
/// Those systems supply evidence through their authoritative
/// verification and testing interfaces.
#[derive(Debug, Clone, Default)]
pub struct ConformanceSuite {
    requirements: std::collections::BTreeMap<
        ConformanceCheckId,
        ConformanceRequirement,
    >,
}

impl ConformanceSuite {
    /// Create an empty conformance suite.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a requirement.
    ///
    /// Duplicate identifiers are rejected instead of silently
    /// replacing existing requirements.
    pub fn register(
        &mut self,
        requirement: ConformanceRequirement,
    ) -> ConformanceResult<()> {
        let id = requirement.id.clone();

        if self.requirements.contains_key(&id) {
            return Err(ConformanceError::DuplicateCheck {
                check_id: id,
            });
        }

        self.requirements.insert(id, requirement);

        Ok(())
    }

    /// Return the number of registered requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Return whether the suite contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Return a requirement by its identity.
    #[must_use]
    pub fn get(
        &self,
        id: &ConformanceCheckId,
    ) -> Option<&ConformanceRequirement> {
        self.requirements.get(id)
    }

    /// Iterate through requirements in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ConformanceRequirement> {
        self.requirements.values()
    }

    /// Evaluate the suite against supplied evidence.
    ///
    /// Missing evidence becomes Blocked.
    ///
    /// Violating evidence becomes Failed.
    ///
    /// Inconclusive evidence becomes Blocked.
    ///
    /// Evidence from an unrelated authority is rejected.
    ///
    /// Evidence for unknown requirements is rejected.
    ///
    /// Duplicate evidence is rejected.
    ///
    /// No missing check is automatically considered passed.
    pub fn evaluate<I>(
        &self,
        evidence: I,
    ) -> ConformanceResult<ConformanceReport>
    where
        I: IntoIterator<Item = ConformanceEvidence>,
    {
        let mut evidence_by_id =
            std::collections::BTreeMap::new();

        for item in evidence {
            let id = item.check_id.clone();

            let requirement = self
                .requirements
                .get(&id)
                .ok_or_else(|| {
                    ConformanceError::UnknownCheck {
                        check_id: id.clone(),
                    }
                })?;

            if requirement.authority != item.authority {
                return Err(
                    ConformanceError::AuthorityMismatch {
                        check_id: id,
                        expected: requirement
                            .authority
                            .as_str()
                            .to_owned(),
                        actual: item
                            .authority
                            .as_str()
                            .to_owned(),
                    },
                );
            }

            if evidence_by_id.contains_key(&id) {
                return Err(
                    ConformanceError::DuplicateEvidence {
                        check_id: id,
                    },
                );
            }

            evidence_by_id.insert(id, item);
        }

        let mut findings = Vec::new();

        findings
            .try_reserve(self.requirements.len())
            .map_err(|error| {
                ConformanceError::InvalidInput {
                    field: "report allocation".to_owned(),
                    reason: error.to_string(),
                }
            })?;

        for requirement in self.requirements.values() {
            let evidence = evidence_by_id
                .remove(&requirement.id)
                ;

            let (status, explanation) = match evidence.as_ref() {
                None => (
                    FindingStatus::Blocked,
                    "required conformance evidence is missing"
                        .to_owned(),
                ),

                Some(item) => match item.outcome {
                    EvidenceOutcome::Satisfied => (
                        FindingStatus::Passed,
                        item.rationale.clone(),
                    ),

                    EvidenceOutcome::Violated => (
                        FindingStatus::Failed,
                        item.rationale.clone(),
                    ),

                    EvidenceOutcome::Inconclusive => (
                        FindingStatus::Blocked,
                        item.rationale.clone(),
                    ),

                    EvidenceOutcome::NotApplicable => (
                        FindingStatus::NotApplicable,
                        item.rationale.clone(),
                    ),
                },
            };

            findings.push(ConformanceFinding {
                requirement: requirement.clone(),
                status,
                evidence,
                explanation,
            });
        }

        ConformanceReport::from_findings(findings)
    }
}

// ============================================================================
// CONFORMANCE REPORT
// ============================================================================

/// Immutable aggregate of conformance findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReport {
    findings: Vec<ConformanceFinding>,
    status: ConformanceStatus,
    mandatory_passed: usize,
    mandatory_failed: usize,
    mandatory_blocked: usize,
    recommended_failed: usize,
    recommended_blocked: usize,
    not_applicable: usize,
}

impl ConformanceReport {
    fn from_findings(
        findings: Vec<ConformanceFinding>,
    ) -> ConformanceResult<Self> {
        let mut mandatory_passed = 0;
        let mut mandatory_failed = 0;
        let mut mandatory_blocked = 0;
        let mut recommended_failed = 0;
        let mut recommended_blocked = 0;
        let mut not_applicable = 0;

        for finding in &findings {
            match (
                finding.requirement.level,
                finding.status,
            ) {
                (
                    RequirementLevel::Mandatory,
                    FindingStatus::Passed,
                ) => mandatory_passed += 1,

                (
                    RequirementLevel::Mandatory,
                    FindingStatus::Failed,
                ) => mandatory_failed += 1,

                (
                    RequirementLevel::Mandatory,
                    FindingStatus::Blocked,
                ) => mandatory_blocked += 1,

                (
                    RequirementLevel::Recommended,
                    FindingStatus::Failed,
                ) => recommended_failed += 1,

                (
                    RequirementLevel::Recommended,
                    FindingStatus::Blocked,
                ) => recommended_blocked += 1,

                (_, FindingStatus::NotApplicable) => {
                    not_applicable += 1;
                }

                (
                    RequirementLevel::Recommended,
                    FindingStatus::Passed,
                ) => {}
            }
        }

        let status = if mandatory_failed > 0 {
            ConformanceStatus::NonConformant
        } else if mandatory_blocked > 0 {
            ConformanceStatus::Inconclusive
        } else {
            ConformanceStatus::Conformant
        };

        Ok(Self {
            findings,
            status,
            mandatory_passed,
            mandatory_failed,
            mandatory_blocked,
            recommended_failed,
            recommended_blocked,
            not_applicable,
        })
    }

    /// Return the overall conformance status.
    #[must_use]
    pub const fn status(&self) -> ConformanceStatus {
        self.status
    }

    /// Return whether all mandatory requirements passed.
    #[must_use]
    pub const fn is_conformant(&self) -> bool {
        matches!(
            self.status,
            ConformanceStatus::Conformant
        )
    }

    /// Return whether the report has unresolved mandatory evidence.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        matches!(
            self.status,
            ConformanceStatus::Inconclusive
        )
    }

    /// Return the total number of evaluated requirements.
    #[must_use]
    pub fn total(&self) -> usize {
        self.findings.len()
    }

    /// Return mandatory passing findings.
    #[must_use]
    pub const fn mandatory_passed(&self) -> usize {
        self.mandatory_passed
    }

    /// Return mandatory failures.
    #[must_use]
    pub const fn mandatory_failed(&self) -> usize {
        self.mandatory_failed
    }

    /// Return unresolved mandatory findings.
    #[must_use]
    pub const fn mandatory_blocked(&self) -> usize {
        self.mandatory_blocked
    }

    /// Return recommended failures.
    #[must_use]
    pub const fn recommended_failed(&self) -> usize {
        self.recommended_failed
    }

    /// Return recommended unresolved findings.
    #[must_use]
    pub const fn recommended_blocked(&self) -> usize {
        self.recommended_blocked
    }

    /// Return explicitly non-applicable findings.
    #[must_use]
    pub const fn not_applicable(&self) -> usize {
        self.not_applicable
    }

    /// Return all findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[ConformanceFinding] {
        &self.findings
    }

    /// Find a finding by its requirement identity.
    #[must_use]
    pub fn finding(
        &self,
        id: &ConformanceCheckId,
    ) -> Option<&ConformanceFinding> {
        self.findings
            .iter()
            .find(|finding| finding.requirement.id == *id)
    }

    /// Return mandatory failed findings.
    pub fn failures(
        &self,
    ) -> impl Iterator<Item = &ConformanceFinding> {
        self.findings.iter().filter(|finding| {
            finding.requirement.level == RequirementLevel::Mandatory
                && finding.status == FindingStatus::Failed
        })
    }

    /// Return unresolved mandatory findings.
    pub fn blocked(
        &self,
    ) -> impl Iterator<Item = &ConformanceFinding> {
        self.findings.iter().filter(|finding| {
            finding.requirement.level == RequirementLevel::Mandatory
                && finding.status == FindingStatus::Blocked
        })
    }

    /// Validate internal report invariants.
    ///
    /// This is useful when reports are passed between services
    /// or reconstructed from external representations.
    pub fn validate(&self) -> ConformanceResult<()> {
        let mut ids = std::collections::BTreeSet::new();

        for finding in &self.findings {
            if !ids.insert(finding.requirement.id.clone()) {
                return Err(ConformanceError::InvalidReport {
                    reason: "duplicate requirement identity"
                        .to_owned(),
                });
            }

            if let Some(evidence) = &finding.evidence {
                if evidence.check_id != finding.requirement.id {
                    return Err(ConformanceError::InvalidReport {
                        reason: "evidence identity does not match finding"
                            .to_owned(),
                    });
                }

                if evidence.authority != finding.requirement.authority {
                    return Err(ConformanceError::InvalidReport {
                        reason: "evidence authority does not match requirement"
                            .to_owned(),
                    });
                }
            }

            match (
                finding.evidence.as_ref().map(|item| item.outcome),
                finding.status,
            ) {
                (None, FindingStatus::Blocked) => {}

                (
                    Some(EvidenceOutcome::Satisfied),
                    FindingStatus::Passed,
                ) => {}

                (
                    Some(EvidenceOutcome::Violated),
                    FindingStatus::Failed,
                ) => {}

                (
                    Some(EvidenceOutcome::Inconclusive),
                    FindingStatus::Blocked,
                ) => {}

                (
                    Some(EvidenceOutcome::NotApplicable),
                    FindingStatus::NotApplicable,
                ) => {}

                _ => {
                    return Err(
                        ConformanceError::InvalidReport {
                            reason: "finding status contradicts evidence"
                                .to_owned(),
                        },
                    );
                }
            }
        }

        let reconstructed =
            Self::from_findings(self.findings.clone())?;

        if reconstructed != *self {
            return Err(ConformanceError::InvalidReport {
                reason: "report aggregate counts or status are inconsistent"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// BASELINE ZAMANI TOOLCHAIN PROFILE
// ============================================================================

/// Construct the baseline Zamani toolchain conformance suite.
///
/// This profile checks architectural invariants rather than
/// enumerating every supported computing domain or vendor.
///
/// Every check requires separately supplied evidence.
///
/// Creating this suite does not imply that any requirement passes.
pub fn zamani_toolchain_profile() -> ConformanceResult<ConformanceSuite> {
    let mut suite = ConformanceSuite::new();

    let requirements = [
        (
            "architecture.ownership",
            "grammar/DESIGN.md",
            "Architectural ownership",
            "Each language, compiler, backend, hardware, runtime and toolchain responsibility has one canonical authority.",
        ),
        (
            "grammar.canonical_composition",
            "grammar/MANIFEST.yaml",
            "Canonical grammar composition",
            "Grammar composition, lexer and parser boundaries follow their canonical ownership contracts without competing universal grammar roots.",
        ),
        (
            "grammar.ast_semantic",
            "grammar/DESIGN.md",
            "AST and semantic integration",
            "Accepted source constructs map through the canonical frontend AST and semantic authorities.",
        ),
        (
            "semantic.requirements_capabilities",
            "grammar/specification/semantic-model.md",
            "Semantic intent preservation",
            "Requirements, capabilities, preferences, constraints and hints remain distinct and retain their semantic meaning.",
        ),
        (
            "semantic.ir_boundaries",
            "grammar/validation/semantic-boundaries.md",
            "Canonical IR boundaries",
            "Domain-specific computation uses its authoritative IR without the toolchain creating competing semantic or IR implementations.",
        ),
        (
            "poco_reaf.semantic_preservation",
            "grammar/specification/poco-reaf.md",
            "POCO-REAF semantic preservation",
            "Compatible target realizations preserve the specified program semantics and observable behavior.",
        ),
        (
            "portability.honest_feasibility",
            "grammar/specification/poco-reaf.md",
            "Honest portability and feasibility",
            "Target requests, advertised capabilities and successful planning are not misrepresented as proof of physical feasibility or execution.",
        ),
        (
            "scalability.no_universal_ceilings",
            "grammar/specification/scalability-model.md",
            "No artificial universal capacity ceilings",
            "No arbitrary universal hardware, resource, domain, project, target or computation-size ceiling is introduced.",
        ),
        (
            "scalability.operational_budgets",
            "grammar/specification/scalability-model.md",
            "Operational budget separation",
            "Operational budgets are explicit and configurable and do not redefine language-level semantic capacity.",
        ),
        (
            "compiler.backend.integration",
            "src/compiler/",
            "Compiler and backend integration",
            "Actual compilation and target-specific artifact production are delegated to the canonical compiler and backend authorities.",
        ),
        (
            "artifacts.integrity_provenance",
            "src/compiler/artifacts/",
            "Artifact integrity and provenance",
            "Artifacts retain verifiable identities, applicable target information, integrity evidence and provenance.",
        ),
        (
            "runtime.execution_boundary",
            "grammar/validation/semantic-boundaries.md",
            "Runtime ownership",
            "Toolchain build and deployment orchestration do not claim runtime execution or physical resource allocation.",
        ),
        (
            "providers.open_world",
            "src/backend/",
            "Open provider integration",
            "New compatible providers and computational domains can be integrated without closed universal provider or domain enumerations.",
        ),
        (
            "security.fail_closed",
            "grammar/DESIGN.md",
            "Fail-closed security and validation",
            "Unknown, invalid, untrusted or unverified inputs do not silently become trusted or successful outcomes.",
        ),
        (
            "reproducibility.determinism",
            "grammar/specification/poco-reaf.md",
            "Deterministic and reproducible operations",
            "Applicable dependency resolution, build inputs, outputs and provenance preserve their documented determinism and reproducibility contracts.",
        ),
        (
            "rust.memory_safety",
            "grammar/DESIGN.md",
            "Safe Rust implementation",
            "Production implementation uses safe Rust and prohibits unsafe code.",
        ),
    ];

    for (
        id,
        authority,
        title,
        statement,
    ) in requirements {
        let requirement = ConformanceRequirement::new(
            ConformanceCheckId::new(id)?,
            ConformanceAuthority::new(authority)?,
            title,
            statement,
            RequirementLevel::Mandatory,
        )?;

        suite.register(requirement)?;
    }

    Ok(suite)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn requirement(
        id: &str,
        level: RequirementLevel,
    ) -> ConformanceRequirement {
        ConformanceRequirement::new(
            ConformanceCheckId::new(id)
                .expect("valid check identifier"),
            ConformanceAuthority::new("grammar/DESIGN.md")
                .expect("valid authority"),
            "Test requirement",
            "The implementation satisfies the test contract.",
            level,
        )
        .expect("valid requirement")
    }

    fn evidence(
        id: &str,
        outcome: EvidenceOutcome,
    ) -> ConformanceEvidence {
        ConformanceEvidence::new(
            ConformanceCheckId::new(id)
                .expect("valid check identifier"),
            ConformanceAuthority::new("grammar/DESIGN.md")
                .expect("valid authority"),
            outcome,
            "tests/conformance/example.rs",
            "Automated verification",
            "test-revision",
            "Evidence produced by the test.",
        )
        .expect("valid evidence")
    }

    #[test]
    fn contract_identity_is_stable() {
        assert_eq!(
            contract_id(),
            "zamani.toolchain.testing.conformance"
        );

        assert_eq!(contract_version(), (1, 0, 0));
    }

    #[test]
    fn check_identifiers_are_open_ended() {
        let id = ConformanceCheckId::new(
            "future.domain.provider.capability",
        )
        .expect("future identifiers must be accepted");

        assert_eq!(
            id.as_str(),
            "future.domain.provider.capability"
        );
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert!(
            ConformanceCheckId::new("").is_err()
        );

        assert!(
            ConformanceAuthority::new(" ").is_err()
        );
    }

    #[test]
    fn registers_unique_requirements() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "architecture.ownership",
                RequirementLevel::Mandatory,
            ))
            .expect("first registration");

        assert_eq!(suite.len(), 1);

        assert!(
            matches!(
                suite.register(requirement(
                    "architecture.ownership",
                    RequirementLevel::Mandatory,
                )),
                Err(ConformanceError::DuplicateCheck { .. })
            )
        );
    }

    #[test]
    fn missing_mandatory_evidence_is_inconclusive() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate(std::iter::empty())
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::Inconclusive
        );

        assert!(!report.is_conformant());
        assert_eq!(report.mandatory_blocked(), 1);
        assert_eq!(report.mandatory_passed(), 0);
    }

    #[test]
    fn satisfied_mandatory_evidence_passes() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate([evidence(
                "mandatory.check",
                EvidenceOutcome::Satisfied,
            )])
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::Conformant
        );

        assert!(report.is_conformant());
        assert_eq!(report.mandatory_passed(), 1);
    }

    #[test]
    fn violated_mandatory_evidence_fails() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate([evidence(
                "mandatory.check",
                EvidenceOutcome::Violated,
            )])
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::NonConformant
        );

        assert_eq!(report.mandatory_failed(), 1);
    }

    #[test]
    fn inconclusive_evidence_never_passes() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate([evidence(
                "mandatory.check",
                EvidenceOutcome::Inconclusive,
            )])
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::Inconclusive
        );

        assert!(!report.is_conformant());
    }

    #[test]
    fn recommended_failure_does_not_hide_mandatory_status() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("mandatory requirement");

        suite
            .register(requirement(
                "recommended.check",
                RequirementLevel::Recommended,
            ))
            .expect("recommended requirement");

        let report = suite
            .evaluate([
                evidence(
                    "mandatory.check",
                    EvidenceOutcome::Satisfied,
                ),
                evidence(
                    "recommended.check",
                    EvidenceOutcome::Violated,
                ),
            ])
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::Conformant
        );

        assert_eq!(report.recommended_failed(), 1);
        assert!(report.is_conformant());
    }

    #[test]
    fn rejects_unknown_evidence_checks() {
        let suite = ConformanceSuite::new();

        assert!(
            matches!(
                suite.evaluate([evidence(
                    "unknown.check",
                    EvidenceOutcome::Satisfied,
                )]),
                Err(ConformanceError::UnknownCheck { .. })
            )
        );
    }

    #[test]
    fn rejects_duplicate_evidence() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        assert!(
            matches!(
                suite.evaluate([
                    evidence(
                        "mandatory.check",
                        EvidenceOutcome::Satisfied,
                    ),
                    evidence(
                        "mandatory.check",
                        EvidenceOutcome::Satisfied,
                    ),
                ]),
                Err(ConformanceError::DuplicateEvidence { .. })
            )
        );
    }

    #[test]
    fn rejects_authority_mismatch() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let wrong_authority = ConformanceEvidence::new(
            ConformanceCheckId::new("mandatory.check")
                .expect("check identity"),
            ConformanceAuthority::new("untrusted/authority")
                .expect("authority"),
            EvidenceOutcome::Satisfied,
            "test-report",
            "test",
            "revision",
            "test evidence",
        )
        .expect("evidence");

        assert!(
            matches!(
                suite.evaluate([wrong_authority]),
                Err(ConformanceError::AuthorityMismatch { .. })
            )
        );
    }

    #[test]
    fn not_applicable_requires_explicit_evidence() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate([evidence(
                "mandatory.check",
                EvidenceOutcome::NotApplicable,
            )])
            .expect("evaluation");

        assert_eq!(
            report.findings()[0].status(),
            FindingStatus::NotApplicable
        );

        assert_eq!(
            report.status(),
            ConformanceStatus::Conformant
        );
    }

    #[test]
    fn report_validation_reconstructs_aggregate_state() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "mandatory.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register requirement");

        let report = suite
            .evaluate([evidence(
                "mandatory.check",
                EvidenceOutcome::Satisfied,
            )])
            .expect("evaluation");

        assert!(report.validate().is_ok());
    }

    #[test]
    fn report_exposes_deterministic_findings() {
        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement(
                "zeta.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register zeta");

        suite
            .register(requirement(
                "alpha.check",
                RequirementLevel::Mandatory,
            ))
            .expect("register alpha");

        let report = suite
            .evaluate([
                evidence(
                    "zeta.check",
                    EvidenceOutcome::Satisfied,
                ),
                evidence(
                    "alpha.check",
                    EvidenceOutcome::Satisfied,
                ),
            ])
            .expect("evaluation");

        assert_eq!(
            report.findings()[0]
                .requirement()
                .id()
                .as_str(),
            "alpha.check"
        );

        assert_eq!(
            report.findings()[1]
                .requirement()
                .id()
                .as_str(),
            "zeta.check"
        );
    }

    #[test]
    fn baseline_profile_contains_mandatory_requirements() {
        let suite = zamani_toolchain_profile()
            .expect("baseline profile");

        assert!(!suite.is_empty());

        for requirement in suite.iter() {
            assert_eq!(
                requirement.level(),
                RequirementLevel::Mandatory
            );
        }
    }

    #[test]
    fn baseline_profile_does_not_claim_implementation_success() {
        let suite = zamani_toolchain_profile()
            .expect("baseline profile");

        let report = suite
            .evaluate(std::iter::empty())
            .expect("evaluation");

        assert_eq!(
            report.status(),
            ConformanceStatus::Inconclusive
        );

        assert_eq!(
            report.mandatory_blocked(),
            suite.len()
        );

        assert!(!report.is_conformant());
    }

    #[test]
    fn future_computational_domains_do_not_require_new_enums() {
        let id = ConformanceCheckId::new(
            "photonic.future_provider.integration",
        )
        .expect("open domain identity");

        let authority = ConformanceAuthority::new(
            "third-party/photonic-provider-contract",
        )
        .expect("open authority");

        let requirement = ConformanceRequirement::new(
            id.clone(),
            authority.clone(),
            "Photonic provider integration",
            "The provider satisfies its registered integration contract.",
            RequirementLevel::Mandatory,
        )
        .expect("requirement");

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("register future domain");

        let evidence = ConformanceEvidence::new(
            id,
            authority,
            EvidenceOutcome::Satisfied,
            "provider-conformance-report",
            "Independent provider conformance tests",
            "provider-revision",
            "Provider contract tests passed.",
        )
        .expect("evidence");

        let report = suite
            .evaluate([evidence])
            .expect("evaluation");

        assert!(report.is_conformant());
    }
}
