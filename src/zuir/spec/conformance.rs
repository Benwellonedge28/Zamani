
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Conformance Contract
//!
//! Path: `src/zuir/spec/conformance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define a deterministic, extensible framework for expressing, executing,
//! aggregating, and reporting conformance requirements for ZUIR and its
//! integrations.
//!
//! This module is a conformance framework, not a substitute for the
//! implementation-owned verifier, parser, serializer, dialect registry,
//! pass manager, or lowering system.
//!
//! A requirement passes only when its owning check supplies positive
//! evidence. Missing checks, unsupported checks, incomplete evidence,
//! and exhausted work budgets never count as success.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/spec/invariants.rs`
//! - `src/zuir/spec/compatibility.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Normative specifications define the required behavior. The owning
//! implementation modules supply executable checks and evidence.
//!
//! ## OWNS
//!
//! - Conformance domains, requirement descriptors, and evidence categories.
//! - Check outcomes and deterministic report aggregation.
//! - A reusable check interface and execution context.
//! - Suite validation, explicit work budgeting, and reporting.
//! - Adapters that expose the existing core invariant catalogue as
//!   conformance requirements.
//!
//! ## DOES_NOT_OWN
//!
//! - Concrete IR nodes, builders, or verification algorithms.
//! - Source grammar, tokenization, parsing, AST construction, or semantics.
//! - The canonical invariant catalogue or compatibility policy.
//! - Serialization, schema migration, or fingerprint algorithms.
//! - Target capability discovery, physical allocation, routing, or scheduling.
//! - A claim that a subsystem conforms before its checks actually run.
//!
//! ## SCALABILITY
//!
//! No universal limit is imposed on programs, operations, qubits, processors,
//! devices, nodes, tensor dimensions, register widths, or dialect count.
//!
//! The suite may accept an explicit operational check budget. Exhaustion is
//! reported separately from conformance failure. The process remains subject
//! to available memory, time, address space, and operating-system resources.
//!
//! ## DETERMINISM
//!
//! Requirements and findings are sorted by stable identifiers when
//! canonical ordering is requested. The framework uses no clock, random
//! selection, environment inspection, hardware discovery, or global state.
//!
//! Individual checks must declare and uphold their own determinism contract.
//!
//! ## FREEZE CRITERIA
//!
//! - Unknown or missing evidence never produces a passing result.
//! - Duplicate requirement identifiers are rejected.
//! - Required and optional requirements are distinguishable.
//! - Resource exhaustion is distinct from a conformance failure.
//! - The core invariant catalogue remains the single source of invariant IDs.
//! - A report is reproducibly ordered.
//! - No unsafe code or hardware-capacity assumptions are introduced.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// ============================================================================
// Stable conformance domains
// ============================================================================

/// The subsystem or contract area covered by a conformance requirement.
///
/// These domains organize evidence; they do not establish that the
/// corresponding implementation subsystem already exists or passes.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConformanceDomain {
    SourceGrammar,
    Ast,
    SemanticModel,
    PublicApi,
    Schema,
    Types,
    Values,
    Structure,
    ControlFlow,
    Effects,
    Ownership,
    Resources,
    Capabilities,
    Dialects,
    Extensions,
    Compatibility,
    Serialization,
    Provenance,
    Determinism,
    Transformations,
    Lowering,
    Classical,
    Quantum,
    Hdl,
    Distributed,
    Interoperability,
    Scalability,
    Safety,
    Integration,
    Tooling,
}

impl ConformanceDomain {
    /// Stable machine-readable domain identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceGrammar => "source_grammar",
            Self::Ast => "ast",
            Self::SemanticModel => "semantic_model",
            Self::PublicApi => "public_api",
            Self::Schema => "schema",
            Self::Types => "types",
            Self::Values => "values",
            Self::Structure => "structure",
            Self::ControlFlow => "control_flow",
            Self::Effects => "effects",
            Self::Ownership => "ownership",
            Self::Resources => "resources",
            Self::Capabilities => "capabilities",
            Self::Dialects => "dialects",
            Self::Extensions => "extensions",
            Self::Compatibility => "compatibility",
            Self::Serialization => "serialization",
            Self::Provenance => "provenance",
            Self::Determinism => "determinism",
            Self::Transformations => "transformations",
            Self::Lowering => "lowering",
            Self::Classical => "classical",
            Self::Quantum => "quantum",
            Self::Hdl => "hdl",
            Self::Distributed => "distributed",
            Self::Interoperability => "interoperability",
            Self::Scalability => "scalability",
            Self::Safety => "safety",
            Self::Integration => "integration",
            Self::Tooling => "tooling",
        }
    }
}

impl fmt::Display for ConformanceDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Requirement descriptors
// ============================================================================

/// Whether a requirement must pass for the requested conformance profile.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RequirementLevel {
    /// Failure, inconclusive evidence, or inability to run blocks acceptance.
    Required,

    /// Useful evidence that does not independently block acceptance.
    Optional,
}

/// The kind of evidence expected from a check.
///
/// Evidence categories describe the contract of a check; they do not
/// themselves prove that the evidence is valid.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    ExecutableCheck,
    PropertyTest,
    RegressionTest,
    GrammarConformance,
    RoundTrip,
    CompatibilityCheck,
    SemanticPreservation,
    DeterminismCheck,
    ResourceStress,
    FuzzCheck,
    IntegrationCheck,
    StaticInspection,
    ExternalValidation,
}

/// Stable descriptor for a single conformance requirement.
///
/// `id` must be globally unambiguous within a suite. Use namespaced IDs,
/// such as `zuir.serialization.round_trip`, rather than display labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceRequirement {
    /// Stable identifier; changing it requires an explicit compatibility
    /// decision because reports and automation may refer to it.
    pub id: String,

    /// Contract domain covered by the requirement.
    pub domain: ConformanceDomain,

    /// Whether the requirement blocks acceptance.
    pub level: RequirementLevel,

    /// Evidence category expected from its implementation-owned check.
    pub evidence: EvidenceKind,

    /// Normative statement describing the expected behavior.
    pub statement: String,

    /// Authoritative module, specification section, or registered owner.
    pub owner: String,
}

impl ConformanceRequirement {
    /// Construct a requirement after validating its stable identifier and
    /// required descriptive fields.
    pub fn new(
        id: impl Into<String>,
        domain: ConformanceDomain,
        level: RequirementLevel,
        evidence: EvidenceKind,
        statement: impl Into<String>,
        owner: impl Into<String>,
    ) -> Result<Self, SuiteDefinitionError> {
        let requirement = Self {
            id: id.into(),
            domain,
            level,
            evidence,
            statement: statement.into(),
            owner: owner.into(),
        };

        validate_requirement(&requirement)?;
        Ok(requirement)
    }
}

fn validate_requirement(
    requirement: &ConformanceRequirement,
) -> Result<(), SuiteDefinitionError> {
    if !is_valid_identifier(&requirement.id) {
        return Err(SuiteDefinitionError::InvalidRequirementId {
            id: requirement.id.clone(),
        });
    }

    if requirement.statement.trim().is_empty() {
        return Err(SuiteDefinitionError::EmptyStatement {
            id: requirement.id.clone(),
        });
    }

    if requirement.owner.trim().is_empty() {
        return Err(SuiteDefinitionError::EmptyOwner {
            id: requirement.id.clone(),
        });
    }

    Ok(())
}

/// Validate a stable, dot-separated identifier.
///
/// Each component must begin with an ASCII letter or underscore and may
/// then contain ASCII letters, digits, underscores, or hyphens.
/// Empty components and leading/trailing separators are rejected.
fn is_valid_identifier(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    value.split('.').all(|component| {
        let mut chars = component.chars();

        matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
            && chars.all(|c| {
                c.is_ascii_alphanumeric() || c == '_' || c == '-'
            })
    })
}

// ============================================================================
// Outcomes and findings
// ============================================================================

/// Outcome of one check.
///
/// `Inconclusive` is not success. `ResourceExhausted` does not assert that
/// the artifact is semantically invalid.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CheckOutcome {
    Passed,
    Failed,
    Inconclusive,
    Skipped,
    ResourceExhausted,
}

/// Severity assigned to a recorded finding.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
}

/// A machine-readable result from one conformance check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    /// Must match the descriptor ID of the check that produced this result.
    pub requirement_id: String,

    /// Explicit result of the check.
    pub outcome: CheckOutcome,

    /// Short stable classification code, such as `ZUIR-CONF-001`.
    pub code: String,

    /// Human-readable explanation; not a machine-stable interface.
    pub message: String,

    /// Additional context without requiring terminal or editor rendering.
    #[serde(default)]
    pub notes: Vec<String>,
}

impl CheckResult {
    /// Create a result for a known requirement.
    pub fn new(
        requirement_id: impl Into<String>,
        outcome: CheckOutcome,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            requirement_id: requirement_id.into(),
            outcome,
            code: code.into(),
            message: message.into(),
            notes: Vec::new(),
        }
    }

    /// Add contextual information without changing the result classification.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    #[must_use]
    pub fn passed(requirement_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            requirement_id,
            CheckOutcome::Passed,
            "ZUIR-CONF-PASS",
            message,
        )
    }

    #[must_use]
    pub fn failed(requirement_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            requirement_id,
            CheckOutcome::Failed,
            "ZUIR-CONF-FAIL",
            message,
        )
    }

    #[must_use]
    pub fn inconclusive(
        requirement_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            requirement_id,
            CheckOutcome::Inconclusive,
            "ZUIR-CONF-INCONCLUSIVE",
            message,
        )
    }

    #[must_use]
    pub fn skipped(
        requirement_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            requirement_id,
            CheckOutcome::Skipped,
            "ZUIR-CONF-SKIPPED",
            message,
        )
    }

    #[must_use]
    pub fn resource_exhausted(
        requirement_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            requirement_id,
            CheckOutcome::ResourceExhausted,
            "ZUIR-CONF-RESOURCE-EXHAUSTED",
            message,
        )
    }
}

/// A finding associates a result with its normative requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceFinding {
    pub requirement: ConformanceRequirement,
    pub result: CheckResult,
}

impl ConformanceFinding {
    fn is_blocking(&self) -> bool {
        self.requirement.level == RequirementLevel::Required
            && self.result.outcome != CheckOutcome::Passed
    }
}

// ============================================================================
// Suite definition and validation
// ============================================================================

/// Errors in the conformance suite definition itself.
///
/// These errors are distinct from an artifact failing a conformance check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuiteDefinitionError {
    InvalidRequirementId { id: String },
    DuplicateRequirementId { id: String },
    EmptyStatement { id: String },
    EmptyOwner { id: String },
    MissingRequiredCheck { id: String },
    UnexpectedCheck { id: String },
    MismatchedCheckId {
        expected: String,
        actual: String,
    },
}

impl fmt::Display for SuiteDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirementId { id } => {
                write!(f, "invalid conformance requirement identifier: {id:?}")
            }
            Self::DuplicateRequirementId { id } => {
                write!(f, "duplicate conformance requirement identifier: {id}")
            }
            Self::EmptyStatement { id } => {
                write!(f, "conformance requirement {id} has an empty statement")
            }
            Self::EmptyOwner { id } => {
                write!(f, "conformance requirement {id} has no owner")
            }
            Self::MissingRequiredCheck { id } => {
                write!(f, "required conformance check is missing: {id}")
            }
            Self::UnexpectedCheck { id } => {
                write!(f, "check is registered without a suite requirement: {id}")
            }
            Self::MismatchedCheckId { expected, actual } => {
                write!(
                    f,
                    "check identifier mismatch: expected {expected}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for SuiteDefinitionError {}

/// A validated collection of conformance requirements.
///
/// This type stores declarative requirements only. Executable checks are
/// supplied separately by the subsystem that owns their implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceSuite {
    pub id: String,
    pub requirements: Vec<ConformanceRequirement>,
}

impl ConformanceSuite {
    pub fn new(
        id: impl Into<String>,
        requirements: Vec<ConformanceRequirement>,
    ) -> Result<Self, SuiteDefinitionError> {
        let id = id.into();

        if !is_valid_identifier(&id) {
            return Err(SuiteDefinitionError::InvalidRequirementId { id });
        }

        let mut seen = BTreeSet::new();

        for requirement in &requirements {
            validate_requirement(requirement)?;

            if !seen.insert(requirement.id.clone()) {
                return Err(SuiteDefinitionError::DuplicateRequirementId {
                    id: requirement.id.clone(),
                });
            }
        }

        Ok(Self { id, requirements })
    }

    /// Return requirements in stable identifier order without mutating the
    /// suite's authored order.
    #[must_use]
    pub fn requirements_sorted(&self) -> Vec<&ConformanceRequirement> {
        let mut requirements: Vec<_> = self.requirements.iter().collect();
        requirements.sort_by(|left, right| left.id.cmp(&right.id));
        requirements
    }

    #[must_use]
    pub fn requirement(&self, id: &str) -> Option<&ConformanceRequirement> {
        self.requirements.iter().find(|requirement| requirement.id == id)
    }

    /// Validate that the supplied check IDs exactly match the suite's
    /// requirements. Missing optional checks are not silently accepted:
    /// every requirement must have a registered check or be explicitly
    /// reported as skipped by the execution layer.
    pub fn validate_check_ids<'a>(
        &self,
        check_ids: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), SuiteDefinitionError> {
        let expected: BTreeSet<&str> = self
            .requirements
            .iter()
            .map(|requirement| requirement.id.as_str())
            .collect();

        let mut actual = BTreeSet::new();

        for id in check_ids {
            if !expected.contains(id) {
                return Err(SuiteDefinitionError::UnexpectedCheck {
                    id: id.to_owned(),
                });
            }

            if !actual.insert(id) {
                return Err(SuiteDefinitionError::DuplicateRequirementId {
                    id: id.to_owned(),
                });
            }
        }

        for requirement in &self.requirements {
            if requirement.level == RequirementLevel::Required
                && !actual.contains(requirement.id.as_str())
            {
                return Err(SuiteDefinitionError::MissingRequiredCheck {
                    id: requirement.id.clone(),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Execution context and check interface
// ============================================================================

/// Explicit operational budget for running one suite.
///
/// `None` means that this suite configuration imposes no check-count limit.
/// It does not imply unlimited memory, time, or process resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceBudget {
    pub max_checks: Option<usize>,
}

impl ConformanceBudget {
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_checks: None }
    }

    #[must_use]
    pub const fn bounded(max_checks: usize) -> Self {
        Self {
            max_checks: Some(max_checks),
        }
    }

    #[must_use]
    pub const fn permits(self, completed_checks: usize) -> bool {
        match self.max_checks {
            Some(limit) => completed_checks < limit,
            None => true,
        }
    }
}

impl Default for ConformanceBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Invocation context supplied to checks.
///
/// Artifact inspection is intentionally abstract: a subsystem can create
/// a context adapter for its IR, grammar fixtures, serialized artifacts,
/// or target-lowering pipeline without making this module depend on those
/// concrete types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceContext {
    /// Stable name for the tested implementation or artifact profile.
    pub profile_id: String,

    /// Explicit budget for this invocation.
    pub budget: ConformanceBudget,

    /// Whether the caller expects executable production conformance.
    ///
    /// A check must not interpret this flag as permission to accept unknown
    /// semantics or skip required verification.
    pub production_acceptance: bool,

    /// Deterministic key/value context supplied by the caller.
    ///
    /// Values must not contain secrets. These fields are informational unless
    /// a check explicitly declares that it consumes them.
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

impl ConformanceContext {
    pub fn new(profile_id: impl Into<String>) -> Self {
        Self {
            profile_id: profile_id.into(),
            budget: ConformanceBudget::unlimited(),
            production_acceptance: true,
            attributes: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_budget(mut self, budget: ConformanceBudget) -> Self {
        self.budget = budget;
        self
    }

    #[must_use]
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Executable check owned by the relevant implementation subsystem.
///
/// Implementations must:
/// - return the exact requirement ID declared by `requirement`;
/// - report unsupported or missing evidence as inconclusive or skipped;
/// - report budget exhaustion explicitly;
/// - avoid treating a successful parse or decode as proof of semantic validity;
/// - preserve the source or artifact's defined semantics.
pub trait ConformanceCheck: Send + Sync {
    fn requirement(&self) -> &ConformanceRequirement;

    fn run(&self, context: &ConformanceContext) -> CheckResult;
}

// ============================================================================
// Reports and acceptance
// ============================================================================

/// Aggregate status for an executed conformance suite.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceStatus {
    /// Every required requirement passed.
    Passed,

    /// One or more required checks failed.
    Failed,

    /// Required evidence is incomplete or inconclusive.
    Inconclusive,

    /// A required check could not run because its budget was exhausted.
    ResourceExhausted,

    /// The report does not contain enough evidence to establish acceptance.
    Incomplete,
}

/// Summary counts derived from report findings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceCounts {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub inconclusive: usize,
    pub skipped: usize,
    pub resource_exhausted: usize,
    pub required: usize,
    pub optional: usize,
    pub required_blocking: usize,
}

/// Complete conformance report.
///
/// Findings are retained individually so callers can diagnose failures
/// without collapsing different contract violations into a boolean.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceReport {
    pub suite_id: String,
    pub profile_id: String,
    pub status: ConformanceStatus,
    pub counts: ConformanceCounts,
    pub findings: Vec<ConformanceFinding>,
}

impl ConformanceReport {
    pub fn new(
        suite_id: impl Into<String>,
        profile_id: impl Into<String>,
        findings: Vec<ConformanceFinding>,
    ) -> Self {
        let mut report = Self {
            suite_id: suite_id.into(),
            profile_id: profile_id.into(),
            status: ConformanceStatus::Incomplete,
            counts: ConformanceCounts::default(),
            findings,
        };

        report.sort_deterministically();
        report.recompute();
        report
    }

    /// Sort findings by requirement ID and stable result code.
    ///
    /// Duplicate findings are retained and will be treated as an incomplete
    /// report by `validate_integrity`; they are never silently discarded.
    pub fn sort_deterministically(&mut self) {
        self.findings.sort_by(|left, right| {
            left.requirement
                .id
                .cmp(&right.requirement.id)
                .then_with(|| left.result.code.cmp(&right.result.code))
                .then_with(|| left.result.message.cmp(&right.result.message))
        });
    }

    /// Recompute counts and the aggregate status from the recorded findings.
    pub fn recompute(&mut self) {
        let mut counts = ConformanceCounts {
            total: self.findings.len(),
            ..ConformanceCounts::default()
        };

        let mut has_required_failure = false;
        let mut has_required_inconclusive = false;
        let mut has_required_exhaustion = false;
        let mut has_required_skipped = false;

        for finding in &self.findings {
            match finding.requirement.level {
                RequirementLevel::Required => counts.required += 1,
                RequirementLevel::Optional => counts.optional += 1,
            }

            match finding.result.outcome {
                CheckOutcome::Passed => counts.passed += 1,
                CheckOutcome::Failed => {
                    counts.failed += 1;
                    has_required_failure |=
                        finding.requirement.level == RequirementLevel::Required;
                }
                CheckOutcome::Inconclusive => {
                    counts.inconclusive += 1;
                    has_required_inconclusive |=
                        finding.requirement.level == RequirementLevel::Required;
                }
                CheckOutcome::Skipped => {
                    counts.skipped += 1;
                    has_required_skipped |=
                        finding.requirement.level == RequirementLevel::Required;
                }
                CheckOutcome::ResourceExhausted => {
                    counts.resource_exhausted += 1;
                    has_required_exhaustion |=
                        finding.requirement.level == RequirementLevel::Required;
                }
            }

            if finding.is_blocking() {
                counts.required_blocking += 1;
            }
        }

        self.counts = counts;

        self.status = if has_required_failure {
            ConformanceStatus::Failed
        } else if has_required_exhaustion {
            ConformanceStatus::ResourceExhausted
        } else if has_required_inconclusive {
            ConformanceStatus::Inconclusive
        } else if has_required_skipped || counts.required == 0 {
            ConformanceStatus::Incomplete
        } else {
            ConformanceStatus::Passed
        };
    }

    /// Validate report integrity against a suite.
    ///
    /// This verifies coverage and identity only. It does not re-run checks or
    /// independently establish the truth of a check's evidence.
    pub fn validate_integrity(
        &self,
        suite: &ConformanceSuite,
    ) -> Result<(), SuiteDefinitionError> {
        if self.suite_id != suite.id {
            return Err(SuiteDefinitionError::UnexpectedCheck {
                id: format!(
                    "report suite {:?} does not match {:?}",
                    self.suite_id, suite.id
                ),
            });
        }

        let mut seen = BTreeSet::new();

        for finding in &self.findings {
            validate_requirement(&finding.requirement)?;

            if !seen.insert(finding.requirement.id.clone()) {
                return Err(SuiteDefinitionError::DuplicateRequirementId {
                    id: finding.requirement.id.clone(),
                });
            }

            let expected = suite
                .requirement(&finding.requirement.id)
                .ok_or_else(|| SuiteDefinitionError::UnexpectedCheck {
                    id: finding.requirement.id.clone(),
                })?;

            if expected != &finding.requirement {
                return Err(SuiteDefinitionError::MismatchedCheckId {
                    expected: expected.id.clone(),
                    actual: finding.requirement.id.clone(),
                });
            }

            if finding.result.requirement_id != finding.requirement.id {
                return Err(SuiteDefinitionError::MismatchedCheckId {
                    expected: finding.requirement.id.clone(),
                    actual: finding.result.requirement_id.clone(),
                });
            }
        }

        for requirement in &suite.requirements {
            if requirement.level == RequirementLevel::Required
                && !seen.contains(&requirement.id)
            {
                return Err(SuiteDefinitionError::MissingRequiredCheck {
                    id: requirement.id.clone(),
                });
            }
        }

        Ok(())
    }

    /// True only when required checks establish conformance.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self.status, ConformanceStatus::Passed)
    }

    /// Return findings for a particular contract domain.
    pub fn findings_for(
        &self,
        domain: ConformanceDomain,
    ) -> impl Iterator<Item = &ConformanceFinding> {
        self.findings
            .iter()
            .filter(move |finding| finding.requirement.domain == domain)
    }
}

// ============================================================================
// Suite execution
// ============================================================================

/// Execute a validated suite using its registered checks.
///
/// Required checks that are missing from the registry are rejected before
/// execution. If the explicit check budget is exhausted, remaining required
/// checks are reported as resource-exhausted rather than passed.
///
/// Checks are executed in stable requirement-ID order. The caller is
/// responsible for ensuring that the check implementations do not depend
/// on unspecified external state.
pub fn run_suite(
    suite: &ConformanceSuite,
    context: &ConformanceContext,
    checks: &[&dyn ConformanceCheck],
) -> Result<ConformanceReport, SuiteDefinitionError> {
    let mut check_by_id: BTreeMap<&str, &dyn ConformanceCheck> = BTreeMap::new();

    for check in checks {
        let id = check.requirement().id.as_str();

        if suite.requirement(id).is_none() {
            return Err(SuiteDefinitionError::UnexpectedCheck {
                id: id.to_owned(),
            });
        }

        if check_by_id.insert(id, *check).is_some() {
            return Err(SuiteDefinitionError::DuplicateRequirementId {
                id: id.to_owned(),
            });
        }
    }

    for requirement in &suite.requirements {
        if requirement.level == RequirementLevel::Required
            && !check_by_id.contains_key(requirement.id.as_str())
        {
            return Err(SuiteDefinitionError::MissingRequiredCheck {
                id: requirement.id.clone(),
            });
        }
    }

    let mut findings = Vec::with_capacity(suite.requirements.len());
    let mut completed_checks = 0usize;

    for requirement in suite.requirements_sorted() {
        let Some(check) = check_by_id.get(requirement.id.as_str()).copied() else {
            findings.push(ConformanceFinding {
                requirement: requirement.clone(),
                result: CheckResult::skipped(
                    requirement.id.clone(),
                    "No executable check is registered for this optional requirement.",
                ),
            });
            continue;
        };

        if !context.budget.permits(completed_checks) {
            findings.push(ConformanceFinding {
                requirement: requirement.clone(),
                result: CheckResult::resource_exhausted(
                    requirement.id.clone(),
                    "The conformance suite exhausted its configured check budget.",
                ),
            });
            continue;
        }

        let result = check.run(context);

        if result.requirement_id != requirement.id {
            return Err(SuiteDefinitionError::MismatchedCheckId {
                expected: requirement.id.clone(),
                actual: result.requirement_id,
            });
        }

        completed_checks = completed_checks.saturating_add(1);

        findings.push(ConformanceFinding {
            requirement: requirement.clone(),
            result,
        });
    }

    Ok(ConformanceReport::new(
        suite.id.clone(),
        context.profile_id.clone(),
        findings,
    ))
}

// ============================================================================
// Adapter for the authoritative core invariant catalogue
// ============================================================================

/// Convert the existing core invariant catalogue into declarative
/// conformance requirements.
///
/// This does not run the invariant verifier. The generated requirements
/// still need implementation-owned checks registered with `run_suite`.
#[must_use]
pub fn core_invariant_requirements() -> Vec<ConformanceRequirement> {
    use crate::zuir::spec::invariants::CORE_INVARIANTS;

    CORE_INVARIANTS
        .iter()
        .map(|invariant| {
            let (domain, evidence) = match invariant.owner {
                crate::zuir::spec::invariants::VerificationOwner::Types => {
                    (ConformanceDomain::Types, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::ControlFlow
                | crate::zuir::spec::invariants::VerificationOwner::Dominance => {
                    (ConformanceDomain::ControlFlow, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Effects => {
                    (ConformanceDomain::Effects, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Ownership => {
                    (ConformanceDomain::Ownership, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Resources => {
                    (ConformanceDomain::Resources, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Dialects
                | crate::zuir::spec::invariants::VerificationOwner::ExtensionVerifier => {
                    (ConformanceDomain::Dialects, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Serialization => {
                    (ConformanceDomain::Serialization, EvidenceKind::RoundTrip)
                }
                crate::zuir::spec::invariants::VerificationOwner::Provenance => {
                    (ConformanceDomain::Provenance, EvidenceKind::ExecutableCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::Transformations => {
                    (ConformanceDomain::Transformations, EvidenceKind::SemanticPreservation)
                }
                crate::zuir::spec::invariants::VerificationOwner::Lowering => {
                    (ConformanceDomain::Lowering, EvidenceKind::SemanticPreservation)
                }
                crate::zuir::spec::invariants::VerificationOwner::NumericSemantics => {
                    (ConformanceDomain::Values, EvidenceKind::PropertyTest)
                }
                crate::zuir::spec::invariants::VerificationOwner::Concurrency => {
                    (ConformanceDomain::Integration, EvidenceKind::IntegrationCheck)
                }
                crate::zuir::spec::invariants::VerificationOwner::DefUse
                | crate::zuir::spec::invariants::VerificationOwner::Structural
                | crate::zuir::spec::invariants::VerificationOwner::FullVerification => {
                    (ConformanceDomain::Structure, EvidenceKind::ExecutableCheck)
                }
            };

            ConformanceRequirement {
                id: invariant.id.to_owned(),
                domain,
                level: RequirementLevel::Required,
                evidence,
                statement: invariant.statement.to_owned(),
                owner: invariant.owner.module_path().to_owned(),
            }
        })
        .collect()
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn requirement(
        id: &str,
        level: RequirementLevel,
    ) -> ConformanceRequirement {
        ConformanceRequirement::new(
            id,
            ConformanceDomain::Structure,
            level,
            EvidenceKind::ExecutableCheck,
            "The implementation satisfies the declared contract.",
            "test::check",
        )
        .expect("test requirement must be valid")
    }

    struct FixedCheck {
        requirement: ConformanceRequirement,
        outcome: CheckOutcome,
    }

    impl ConformanceCheck for FixedCheck {
        fn requirement(&self) -> &ConformanceRequirement {
            &self.requirement
        }

        fn run(&self, _context: &ConformanceContext) -> CheckResult {
            CheckResult::new(
                self.requirement.id.clone(),
                self.outcome,
                "TEST-CHECK",
                "Test result.",
            )
        }
    }

    #[test]
    fn rejects_invalid_requirement_identifiers() {
        assert!(ConformanceRequirement::new(
            "zuir..structure",
            ConformanceDomain::Structure,
            RequirementLevel::Required,
            EvidenceKind::ExecutableCheck,
            "Statement",
            "test::owner",
        )
        .is_err());

        assert!(ConformanceRequirement::new(
            "",
            ConformanceDomain::Structure,
            RequirementLevel::Required,
            EvidenceKind::ExecutableCheck,
            "Statement",
            "test::owner",
        )
        .is_err());
    }

    #[test]
    fn rejects_duplicate_requirements() {
        let result = ConformanceSuite::new(
            "zuir.core",
            vec![
                requirement("zuir.structure", RequirementLevel::Required),
                requirement("zuir.structure", RequirementLevel::Required),
            ],
        );

        assert!(matches!(
            result,
            Err(SuiteDefinitionError::DuplicateRequirementId { .. })
        ));
    }

    #[test]
    fn required_failure_blocks_acceptance() {
        let req = requirement("zuir.structure", RequirementLevel::Required);
        let suite = ConformanceSuite::new("zuir.core", vec![req.clone()])
            .expect("suite should be valid");

        let check = FixedCheck {
            requirement: req,
            outcome: CheckOutcome::Failed,
        };

        let context = ConformanceContext::new("test-profile");
        let report = run_suite(&suite, &context, &[&check])
            .expect("suite execution should complete");

        assert_eq!(report.status, ConformanceStatus::Failed);
        assert!(!report.is_accepted());
    }

    #[test]
    fn inconclusive_required_check_is_not_a_pass() {
        let req = requirement("zuir.structure", RequirementLevel::Required);
        let suite = ConformanceSuite::new("zuir.core", vec![req.clone()])
            .expect("suite should be valid");

        let check = FixedCheck {
            requirement: req,
            outcome: CheckOutcome::Inconclusive,
        };

        let report = run_suite(
            &suite,
            &ConformanceContext::new("test-profile"),
            &[&check],
        )
        .expect("suite execution should complete");

        assert_eq!(report.status, ConformanceStatus::Inconclusive);
        assert!(!report.is_accepted());
    }

    #[test]
    fn budget_exhaustion_is_not_semantic_failure_or_success() {
        let first = requirement("zuir.first", RequirementLevel::Required);
        let second = requirement("zuir.second", RequirementLevel::Required);

        let suite = ConformanceSuite::new(
            "zuir.core",
            vec![second.clone(), first.clone()],
        )
        .expect("suite should be valid");

        let first_check = FixedCheck {
            requirement: first,
            outcome: CheckOutcome::Passed,
        };

        let second_check = FixedCheck {
            requirement: second,
            outcome: CheckOutcome::Passed,
        };

        let context = ConformanceContext::new("test-profile")
            .with_budget(ConformanceBudget::bounded(1));

        let report = run_suite(
            &suite,
            &context,
            &[&second_check, &first_check],
        )
        .expect("suite execution should complete");

        assert_eq!(report.status, ConformanceStatus::ResourceExhausted);
        assert_eq!(report.counts.passed, 1);
        assert_eq!(report.counts.resource_exhausted, 1);
        assert!(!report.is_accepted());
    }

    #[test]
    fn findings_are_sorted_deterministically() {
        let a = requirement("zuir.alpha", RequirementLevel::Required);
        let b = requirement("zuir.beta", RequirementLevel::Required);

        let suite = ConformanceSuite::new(
            "zuir.core",
            vec![b.clone(), a.clone()],
        )
        .expect("suite should be valid");

        let a_check = FixedCheck {
            requirement: a,
            outcome: CheckOutcome::Passed,
        };

        let b_check = FixedCheck {
            requirement: b,
            outcome: CheckOutcome::Passed,
        };

        let report = run_suite(
            &suite,
            &ConformanceContext::new("test-profile"),
            &[&b_check, &a_check],
        )
        .expect("suite execution should complete");

        let ids: Vec<_> = report
            .findings
            .iter()
            .map(|finding| finding.requirement.id.as_str())
            .collect();

        assert_eq!(ids, vec!["zuir.alpha", "zuir.beta"]);
        assert_eq!(report.status, ConformanceStatus::Passed);
    }

    #[test]
    fn missing_required_check_is_rejected() {
        let req = requirement("zuir.required", RequirementLevel::Required);
        let suite = ConformanceSuite::new("zuir.core", vec![req])
            .expect("suite should be valid");

        let result = run_suite(
            &suite,
            &ConformanceContext::new("test-profile"),
            &[],
        );

        assert!(matches!(
            result,
            Err(SuiteDefinitionError::MissingRequiredCheck { .. })
        ));
    }

    #[test]
    fn optional_skipped_check_does_not_hide_required_failure() {
        let required = requirement("zuir.required", RequirementLevel::Required);
        let optional = requirement("zuir.optional", RequirementLevel::Optional);

        let suite = ConformanceSuite::new(
            "zuir.core",
            vec![required.clone(), optional],
        )
        .expect("suite should be valid");

        let check = FixedCheck {
            requirement: required,
            outcome: CheckOutcome::Passed,
        };

        let report = run_suite(
            &suite,
            &ConformanceContext::new("test-profile"),
            &[&check],
        )
        .expect("suite execution should complete");

        assert_eq!(report.status, ConformanceStatus::Passed);
        assert_eq!(report.counts.skipped, 1);
    }

    #[test]
    fn report_integrity_rejects_mismatched_result_ids() {
        let req = requirement("zuir.structure", RequirementLevel::Required);
        let suite = ConformanceSuite::new("zuir.core", vec![req.clone()])
            .expect("suite should be valid");

        let report = ConformanceReport::new(
            "zuir.core",
            "test-profile",
            vec![ConformanceFinding {
                requirement: req,
                result: CheckResult::passed("zuir.other", "Incorrect ID."),
            }],
        );

        assert!(matches!(
            report.validate_integrity(&suite),
            Err(SuiteDefinitionError::MismatchedCheckId { .. })
        ));
    }

    #[test]
    fn core_invariant_adapter_uses_authoritative_catalogue() {
        let requirements = core_invariant_requirements();
        assert!(!requirements.is_empty());

        let ids: BTreeSet<_> = requirements
            .iter()
            .map(|requirement| requirement.id.as_str())
            .collect();

        assert_eq!(ids.len(), requirements.len());
        assert!(requirements.iter().all(|requirement| {
            requirement.level == RequirementLevel::Required
                && !requirement.owner.is_empty()
        }));
    }

    #[test]
    fn absent_budget_does_not_impose_a_check_count_limit() {
        let budget = ConformanceBudget::unlimited();

        assert!(budget.permits(0));
        assert!(budget.permits(usize::MAX));
    }
}
