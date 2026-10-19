
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL logical timing analysis
//!
//! Path: `src/hdl/timing/analysis.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/timing.g4`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/reset.g4`
//! - `grammar/hdl/sequential.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/timing/clock.rs`
//! - `src/hdl/timing/reset.rs`
//! - `src/hdl/timing/domain.rs`
//! - `src/hdl/timing/constraints.rs`
//! - `src/hdl/timing/crossing.rs`
//!
//! ## Ownership
//!
//! This module aggregates results from logical timing checks into a stable,
//! deterministic report. It detects contradictory results for the same
//! check and source node, preserves source references, and reports incomplete
//! work when a configured operational budget prevents complete processing.
//!
//! It does not parse HDL, evaluate canonical expressions, resolve names,
//! establish physical delays, perform physical static timing analysis,
//! prove timing closure, insert synchronizers, select targets, allocate
//! hardware, or construct HDLIR.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The semantic and timing adapters translate their existing results into
//! `TimingCheck` values. This module consumes those values without requiring
//! the adapters to replace their canonical models.
//!
//! In particular:
//!
//! - `clock.rs` remains authoritative for the logical clock model.
//! - `reset.rs` remains authoritative for reset declarations.
//! - `domain.rs` remains authoritative for logical clock domains.
//! - `constraints.rs` remains authoritative for source-level timing intent.
//! - `crossing.rs` remains authoritative for its CDC assessment.
//! - `src/frontend/semantic/domains/hdl.rs` remains authoritative for HDL
//!   semantic validation.
//!
//! A successful report means that all submitted checks were processed and
//! none reported failure or contradiction. It does not imply physical timing
//! closure or prove properties that no submitted check established.
//!
//! ## Scalability
//!
//! There are no language-level limits on the number of timing checks,
//! source nodes, clocks, domains, paths, or hardware resources.
//!
//! The iterator API avoids requiring callers to materialize a second input
//! collection. Optional work and finding budgets are operational policies,
//! not language restrictions. Exhausting a budget produces an incomplete
//! report; it never produces a complete-success result.
//!
//! Memory usage is proportional to retained findings and distinct check
//! identities. Callers compiling exceptionally large designs should configure
//! budgets appropriate to their environment and retain the incomplete status.
//!
//! ## Determinism
//!
//! Findings are sorted by source node, stable check code, severity, and message.
//! Contradiction detection uses ordered keys. Input order therefore does not
//! affect the final finding order. If equivalent inputs contain conflicting
//! outcomes, the contradiction is reported explicitly.
//!
//! ## Freeze criteria
//!
//! Freeze this API when adapters from semantic validation, clock/reset/domain
//! analysis, CDC assessment, and timing constraints have conformance tests;
//! incomplete reports cannot be mistaken for complete success; and repository
//! formatting, checking, testing, and Clippy all pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

/// Version of the public logical timing-analysis result contract.
///
/// This is not the Zamani language, grammar, or HDLIR version.
pub const HDL_TIMING_ANALYSIS_VERSION: u16 = 1;

/// Stable identifier for a timing check.
///
/// Codes are open-ended so independently developed analysis providers can
/// introduce checks without modifying a closed enum. Providers should use
/// namespaced values such as `zamani.clock.period` or `vendor_x.setup`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimingCheckCode(String);

impl TimingCheckCode {
    /// Creates a check code without changing its spelling.
    ///
    /// Empty values and whitespace/control characters are rejected. Whether
    /// a code is recognized or supported belongs to its provider and policy.
    pub fn new(value: impl Into<String>) -> Result<Self, TimingAnalysisError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TimingAnalysisError::InvalidCheckCode {
                reason: "code must not be empty",
            });
        }

        if value.chars().any(char::is_whitespace) {
            return Err(TimingAnalysisError::InvalidCheckCode {
                reason: "code must not contain whitespace",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(TimingAnalysisError::InvalidCheckCode {
                reason: "code must not contain control characters",
            });
        }

        Ok(Self(value))
    }

    /// Returns the exact code spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TimingCheckCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Outcome reported by one logical timing check.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingCheckOutcome {
    /// The provider established that this particular check passed.
    Pass,

    /// The provider established that this particular check failed.
    Fail,

    /// The provider could not establish pass or fail.
    ///
    /// This includes unresolved symbolic expressions, missing required
    /// evidence, and analyses that cannot prove the property.
    Inconclusive,
}

/// Severity assigned to an aggregated timing finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingFindingSeverity {
    /// Informational finding that does not itself indicate a failure.
    Info,

    /// A warning requiring attention but not classified as a proven failure.
    Warning,

    /// A check failure or contradictory check result.
    Error,
}

/// One source-linked result produced by a timing analysis provider.
///
/// `source` must be the canonical AST identity of the declaration or
/// expression associated with the check. The coordinator does not invent
/// source spans or duplicate the frontend source map.
///
/// `related` may identify additional canonical AST nodes relevant to the
/// finding, for example both endpoints of a clock-domain relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimingCheck {
    source: NodeId,
    related: Vec<NodeId>,
    code: TimingCheckCode,
    outcome: TimingCheckOutcome,
    message: String,
}

impl TimingCheck {
    /// Creates a source-linked timing check result.
    ///
    /// The constructor validates structural input only. It does not verify
    /// that the check's conclusion is true; the producing analysis owns that
    /// responsibility.
    pub fn new(
        source: NodeId,
        code: TimingCheckCode,
        outcome: TimingCheckOutcome,
        message: impl Into<String>,
    ) -> Result<Self, TimingAnalysisError> {
        let message = message.into();

        if message.trim().is_empty() {
            return Err(TimingAnalysisError::EmptyMessage);
        }

        Ok(Self {
            source,
            related: Vec::new(),
            code,
            outcome,
            message,
        })
    }

    /// Adds related canonical source nodes.
    ///
    /// Duplicate related identities are removed and the final collection is
    /// sorted, making this field deterministic regardless of insertion order.
    #[must_use]
    pub fn with_related(mut self, related: impl IntoIterator<Item = NodeId>) -> Self {
        let mut related: Vec<_> = related.into_iter().collect();
        related.sort_unstable();
        related.dedup();
        self.related = related;
        self
    }

    /// Returns the primary canonical AST node.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns related canonical AST nodes.
    #[must_use]
    pub fn related(&self) -> &[NodeId] {
        &self.related
    }

    /// Returns the stable check code.
    #[must_use]
    pub fn code(&self) -> &TimingCheckCode {
        &self.code
    }

    /// Returns the provider-reported outcome.
    #[must_use]
    pub const fn outcome(&self) -> TimingCheckOutcome {
        self.outcome
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Operational budgets for one timing-analysis invocation.
///
/// `None` means that this coordinator imposes no limit for that dimension;
/// the caller may still apply a compilation-wide budget or cancellation
/// policy. These are operational safeguards, not HDL semantic limits.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TimingAnalysisOptions {
    /// Maximum number of input checks to consume.
    ///
    /// When reached while input remains, the result is incomplete.
    pub max_checks: Option<usize>,

    /// Maximum number of findings retained in the result.
    ///
    /// When a new finding cannot be retained, analysis stops and the result
    /// is incomplete. The implementation never silently drops that finding
    /// and labels the report complete.
    pub max_findings: Option<usize>,
}

/// Overall conclusion of an aggregated timing analysis.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingAnalysisConclusion {
    /// All submitted checks were processed and all passed.
    Pass,

    /// All submitted checks were processed, but at least one check failed
    /// or a contradiction was found.
    Fail,

    /// Analysis could not establish a complete pass/fail conclusion.
    Inconclusive,
}

/// Completion status, distinct from the analysis conclusion.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingAnalysisCompletion {
    /// Every submitted check was processed.
    Complete,

    /// Work stopped before all checks could be processed or all findings
    /// could be retained.
    Incomplete,
}

/// A deterministic timing finding.
///
/// Findings retain canonical AST identities, not copied source text or
/// invented source spans. A diagnostic adapter can resolve those identities
/// through the repository's canonical source/provenance infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimingFinding {
    source: NodeId,
    related: Vec<NodeId>,
    code: TimingCheckCode,
    severity: TimingFindingSeverity,
    message: String,
}

impl TimingFinding {
    /// Returns the primary source node.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns related source nodes.
    #[must_use]
    pub fn related(&self) -> &[NodeId] {
        &self.related
    }

    /// Returns the stable check code.
    #[must_use]
    pub fn code(&self) -> &TimingCheckCode {
        &self.code
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> TimingFindingSeverity {
        self.severity
    }

    /// Returns the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Result of aggregating logical timing checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimingAnalysisReport {
    completion: TimingAnalysisCompletion,
    conclusion: TimingAnalysisConclusion,
    findings: Vec<TimingFinding>,
    checks_processed: usize,
}

impl TimingAnalysisReport {
    /// Returns whether analysis processed all supplied checks.
    #[must_use]
    pub const fn completion(&self) -> TimingAnalysisCompletion {
        self.completion
    }

    /// Returns the aggregate conclusion.
    ///
    /// A `Pass` conclusion is meaningful as complete success only when
    /// `completion()` is `Complete`.
    #[must_use]
    pub const fn conclusion(&self) -> TimingAnalysisConclusion {
        self.conclusion
    }

    /// Returns findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[TimingFinding] {
        &self.findings
    }

    /// Returns the number of input checks consumed.
    #[must_use]
    pub const fn checks_processed(&self) -> usize {
        self.checks_processed
    }

    /// Returns true only when the report establishes complete success.
    #[must_use]
    pub const fn is_complete_success(&self) -> bool {
        matches!(self.completion, TimingAnalysisCompletion::Complete)
            && matches!(self.conclusion, TimingAnalysisConclusion::Pass)
    }
}

/// Errors caused by malformed coordinator input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimingAnalysisError {
    /// A check code was empty or structurally invalid.
    InvalidCheckCode {
        /// Reason the check code was rejected.
        reason: &'static str,
    },

    /// A check message was empty.
    EmptyMessage,
}

impl fmt::Display for TimingAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCheckCode { reason } => {
                write!(formatter, "invalid timing check code: {reason}")
            }
            Self::EmptyMessage => {
                formatter.write_str("timing check message must not be empty")
            }
        }
    }
}

impl Error for TimingAnalysisError {}

/// Aggregates logical timing-check results.
///
/// A contradictory result is reported when the same `(source node, check
/// code)` is reported as both `Pass` and `Fail`. An `Inconclusive` result does
/// not contradict either outcome: it means the provider could not establish
/// a conclusion. A contradiction is never resolved by choosing whichever
/// result happened to arrive first.
///
/// This function does not claim to discover checks that upstream providers
/// did not submit. Completeness means that the entire supplied iterator was
/// consumed within the configured budgets.
pub fn analyze_timing<I>(
    checks: I,
    options: &TimingAnalysisOptions,
) -> TimingAnalysisReport
where
    I: IntoIterator<Item = TimingCheck>,
{
    let mut findings = Vec::new();
    let mut seen: BTreeMap<(NodeId, TimingCheckCode), TimingCheckOutcome> =
        BTreeMap::new();
    let mut checks_processed = 0usize;
    let mut incomplete = false;
    let mut any_failure = false;
    let mut any_inconclusive = false;

    let mut iterator = checks.into_iter();

    loop {
        if options
            .max_checks
            .is_some_and(|limit| checks_processed >= limit)
        {
            // Distinguish an exact-boundary completion from truncated work.
            if iterator.next().is_some() {
                incomplete = true;
            }
            break;
        }

        let Some(check) = iterator.next() else {
            break;
        };

        checks_processed = checks_processed.saturating_add(1);

        let key = (check.source, check.code.clone());
        let previous = seen.get(&key).copied();

        let contradiction = matches!(
            (previous, check.outcome),
            (Some(TimingCheckOutcome::Pass), TimingCheckOutcome::Fail)
                | (Some(TimingCheckOutcome::Fail), TimingCheckOutcome::Pass)
        );

        if let Some(previous) = previous {
            if contradiction {
                any_failure = true;

                let finding = TimingFinding {
                    source: check.source,
                    related: check.related.clone(),
                    code: check.code.clone(),
                    severity: TimingFindingSeverity::Error,
                    message: format!(
                        "contradictory timing outcomes for check `{}`: \
                         previously reported {:?}, now reported {:?}",
                        check.code, previous, check.outcome
                    ),
                };

                if options
                    .max_findings
                    .is_some_and(|limit| findings.len() >= limit)
                {
                    incomplete = true;
                    break;
                }

                findings.push(finding);
            }
        } else {
            seen.insert(key, check.outcome);
        }

        let severity = match check.outcome {
            TimingCheckOutcome::Pass => TimingFindingSeverity::Info,
            TimingCheckOutcome::Fail => {
                any_failure = true;
                TimingFindingSeverity::Error
            }
            TimingCheckOutcome::Inconclusive => {
                any_inconclusive = true;
                TimingFindingSeverity::Warning
            }
        };

        if check.outcome != TimingCheckOutcome::Pass {
            let finding = TimingFinding {
                source: check.source,
                related: check.related,
                code: check.code,
                severity,
                message: check.message,
            };

            if options
                .max_findings
                .is_some_and(|limit| findings.len() >= limit)
            {
                incomplete = true;
                break;
            }

            findings.push(finding);
        }
    }

    findings.sort_by(|left, right| {
        (
            left.source,
            &left.code,
            left.severity,
            &left.message,
            &left.related,
        )
            .cmp(&(
                right.source,
                &right.code,
                right.severity,
                &right.message,
                &right.related,
            ))
    });

    let completion = if incomplete {
        TimingAnalysisCompletion::Incomplete
    } else {
        TimingAnalysisCompletion::Complete
    };

    let conclusion = if any_failure {
        TimingAnalysisConclusion::Fail
    } else if any_inconclusive || incomplete {
        TimingAnalysisConclusion::Inconclusive
    } else {
        TimingAnalysisConclusion::Pass
    };

    TimingAnalysisReport {
        completion,
        conclusion,
        findings,
        checks_processed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(
        source: u64,
        code: &str,
        outcome: TimingCheckOutcome,
        message: &str,
    ) -> TimingCheck {
        TimingCheck::new(
            NodeId::new(source).expect("nonzero test node ID"),
            TimingCheckCode::new(code).expect("valid test code"),
            outcome,
            message,
        )
        .expect("valid test check")
    }

    #[test]
    fn empty_input_is_complete_success() {
        let report = analyze_timing([], &TimingAnalysisOptions::default());

        assert_eq!(report.completion(), TimingAnalysisCompletion::Complete);
        assert_eq!(report.conclusion(), TimingAnalysisConclusion::Pass);
        assert!(report.is_complete_success());
        assert!(report.findings().is_empty());
    }

    #[test]
    fn failed_check_is_reported() {
        let report = analyze_timing(
            [check(
                1,
                "zamani.clock.period",
                TimingCheckOutcome::Fail,
                "clock period requirement was not satisfied",
            )],
            &TimingAnalysisOptions::default(),
        );

        assert_eq!(report.conclusion(), TimingAnalysisConclusion::Fail);
        assert_eq!(report.findings().len(), 1);
        assert_eq!(
            report.findings()[0].severity(),
            TimingFindingSeverity::Error
        );
        assert!(!report.is_complete_success());
    }

    #[test]
    fn inconclusive_check_is_not_a_pass() {
        let report = analyze_timing(
            [check(
                1,
                "zamani.clock.period",
                TimingCheckOutcome::Inconclusive,
                "period expression remains symbolic",
            )],
            &TimingAnalysisOptions::default(),
        );

        assert_eq!(
            report.conclusion(),
            TimingAnalysisConclusion::Inconclusive
        );
        assert!(!report.is_complete_success());
    }

    #[test]
    fn contradictory_results_are_reported() {
        let report = analyze_timing(
            [
                check(
                    1,
                    "zamani.clock.period",
                    TimingCheckOutcome::Pass,
                    "period is within the declared requirement",
                ),
                check(
                    1,
                    "zamani.clock.period",
                    TimingCheckOutcome::Fail,
                    "period exceeds the declared requirement",
                ),
            ],
            &TimingAnalysisOptions::default(),
        );

        assert_eq!(report.conclusion(), TimingAnalysisConclusion::Fail);
        assert!(report.findings().iter().any(|finding| {
            finding.message().contains("contradictory timing outcomes")
        }));
    }

    #[test]
    fn check_budget_never_reports_complete_success_when_truncated() {
        let checks = [
            check(1, "zamani.a", TimingCheckOutcome::Pass, "passed"),
            check(2, "zamani.b", TimingCheckOutcome::Pass, "passed"),
        ];

        let report = analyze_timing(
            checks,
            &TimingAnalysisOptions {
                max_checks: Some(1),
                max_findings: None,
            },
        );

        assert_eq!(report.checks_processed(), 1);
        assert_eq!(report.completion(), TimingAnalysisCompletion::Incomplete);
        assert_eq!(
            report.conclusion(),
            TimingAnalysisConclusion::Inconclusive
        );
        assert!(!report.is_complete_success());
    }

    #[test]
    fn finding_budget_never_silently_discards_a_failure() {
        let checks = [
            check(
                1,
                "zamani.a",
                TimingCheckOutcome::Fail,
                "first failure",
            ),
            check(
                2,
                "zamani.b",
                TimingCheckOutcome::Fail,
                "second failure",
            ),
        ];

        let report = analyze_timing(
            checks,
            &TimingAnalysisOptions {
                max_checks: None,
                max_findings: Some(1),
            },
        );

        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.completion(), TimingAnalysisCompletion::Incomplete);
        assert!(!report.is_complete_success());
    }

    #[test]
    fn findings_are_sorted_independently_of_input_order() {
        let first = check(2, "zamani.b", TimingCheckOutcome::Fail, "second");
        let second = check(1, "zamani.a", TimingCheckOutcome::Fail, "first");

        let report = analyze_timing(
            [first.clone(), second.clone()],
            &TimingAnalysisOptions::default(),
        );
        let reversed = analyze_timing(
            [second, first],
            &TimingAnalysisOptions::default(),
        );

        assert_eq!(report.findings(), reversed.findings());
    }

    #[test]
    fn invalid_codes_and_empty_messages_are_rejected() {
        assert!(TimingCheckCode::new(" ").is_err());
        assert!(TimingCheckCode::new("clock period").is_err());

        assert!(TimingCheck::new(
            NodeId::first(),
            TimingCheckCode::new("zamani.clock.period").unwrap(),
            TimingCheckOutcome::Pass,
            " "
        )
        .is_err());
    }
}
