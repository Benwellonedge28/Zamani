
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Diagnostic Aggregation
//!
//! Path: `src/compiler/diagnostics/aggregation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Aggregate structured diagnostics emitted by compilation stages into one
//! deterministic compilation report.
//!
//! The aggregation layer:
//!
//! - accepts diagnostics from multiple compiler stages;
//! - reuses the canonical frontend semantic Diagnostic record;
//! - preserves source locations and structured diagnostic information;
//! - optionally merges duplicate report identities;
//! - deterministically selects the representative of duplicate reports;
//! - applies an explicitly configured diagnostic-retention budget;
//! - records dropped diagnostics;
//! - distinguishes complete from incomplete aggregation;
//! - provides deterministic final reporting order;
//! - reports allocation and accounting failures explicitly.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - compiler-invocation diagnostic accumulation;
//! - compiler-level aggregation policy;
//! - aggregation statistics;
//! - aggregation completion status;
//! - deterministic merging of duplicate retained diagnostics.
//!
//! It does not own:
//!
//! - the Diagnostic record;
//! - diagnostic codes, severity, or phase definitions;
//! - source spans or source maps;
//! - diagnostic ordering policy;
//! - semantic validation;
//! - parser or lexer recovery;
//! - diagnostic rendering;
//! - diagnostic serialization;
//! - diagnostic suppression policy;
//! - semantic-session budgets;
//! - target feasibility or resource allocation.
//!
//! Canonical diagnostic records remain owned by:
//!
//! `crate::frontend::semantic::diagnostics`
//!
//! Canonical diagnostic ordering and deduplication remain owned by:
//!
//! `crate::frontend::semantic::diagnostics::ordering`
//!
//! ## Integration boundary
//!
//! The compiler may receive findings from frontend, semantic, IR, lowering,
//! optimization, planning, and backend stages.
//!
//! Those producers must convert their stage-specific diagnostic records into
//! the canonical diagnostic API through explicit, loss-aware adapters.
//!
//! This module does not pretend that independently defined parser, lexer,
//! backend, or domain diagnostic records are already interchangeable.
//!
//! ## Scalability
//!
//! No fixed diagnostic-count ceiling is introduced.
//!
//! `max_diagnostics: None` means no explicit retention budget.
//! `Some(limit)` is an invocation-specific operational policy.
//!
//! A retention limit is not a language limit, hardware limit, or semantic
//! restriction. When the budget is exhausted, aggregation becomes incomplete.
//!
//! No diagnostic is silently treated as retained after it has been dropped.
//!
//! The implementation uses fallible reservation for its principal
//! collections and checked accounting for counters.
//!
//! ## Determinism
//!
//! The internal identity index is never iterated to produce output.
//!
//! Duplicate representatives are selected using the canonical diagnostic
//! comparator, not arrival order.
//!
//! Final output is sorted using the canonical diagnostic ordering function.
//!
//! Consequently, independent-stage completion order does not determine the
//! final report order or the selected duplicate representative.
//!
//! ## Failure policy
//!
//! - Diagnostic-budget exhaustion marks aggregation incomplete.
//! - Collection allocation failure marks aggregation incomplete.
//! - Counter overflow is an explicit aggregation error.
//! - Invalid diagnostic codes remain the responsibility of the producer or
//!   publication boundary.
//! - An incomplete aggregate must never be represented as a complete report.
//!
//! ## Rust and safety
//!
//! Rust 2021, Rust 1.97+.
//! No unsafe code.
//! No global mutable state.
//! No filesystem, network, hardware, or runtime access.

use core::fmt;
use core::num::NonZeroUsize;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use crate::frontend::semantic::diagnostics::{
    compare_diagnostics,
    sort_diagnostics,
    Diagnostic,
    DiagnosticCategory,
    DiagnosticCode,
    DiagnosticPhase,
    DiagnosticSeverity,
};

use crate::frontend::ast::node::source::Span;

/// Immutable policy for one compiler diagnostic aggregation operation.
///
/// This policy controls diagnostic retention and duplicate handling only.
/// It does not change diagnostic meaning, severity, source coordinates,
/// compiler validity, or target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AggregationPolicy {
    /// Maximum number of unique diagnostics retained.
    ///
    /// `None` means no explicit retention limit.
    ///
    /// When deduplication is disabled, this limit applies to every retained
    /// diagnostic, including repeated report identities.
    pub max_diagnostics: Option<NonZeroUsize>,

    /// Whether repeated report identities should be merged.
    ///
    /// When enabled, diagnostics with the same canonical report identity
    /// occupy one retained entry. Their representative is selected
    /// deterministically using the canonical diagnostic comparator.
    pub deduplicate: bool,
}

impl Default for AggregationPolicy {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl AggregationPolicy {
    /// Create an aggregation policy without an explicit retention ceiling.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_diagnostics: None,
            deduplicate: true,
        }
    }

    /// Create a policy with an explicit diagnostic-retention budget.
    #[must_use]
    pub const fn with_limit(max_diagnostics: NonZeroUsize) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
            deduplicate: true,
        }
    }

    /// Enable or disable duplicate merging.
    #[must_use]
    pub const fn with_deduplication(mut self, enabled: bool) -> Self {
        self.deduplicate = enabled;
        self
    }
}

/// Completion status of diagnostic aggregation.
///
/// `Complete` means all submitted diagnostics were retained or merged.
/// `Incomplete` means at least one diagnostic could not be retained or
/// accounting could not be completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregationStatus {
    Complete,
    Incomplete,
}

impl AggregationStatus {
    /// Whether aggregation retained or merged every submitted diagnostic.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Outcome of submitting one diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertOutcome {
    /// A new diagnostic was retained.
    Retained,

    /// The diagnostic matched an existing report identity and was merged.
    DuplicateMerged,

    /// The diagnostic could not be retained because the configured budget
    /// had already been reached.
    DroppedByBudget,
}

/// Counters describing one aggregation operation.
///
/// `submitted` counts calls to `push` that successfully entered accounting.
/// `retained` is the number of diagnostics in the final aggregate.
/// `dropped` counts diagnostics that could not be retained.
/// `duplicates` counts submitted diagnostics merged into an existing entry.
///
/// The counters are observational metadata. They do not define compilation
/// validity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AggregationStatistics {
    submitted: u64,
    dropped: u64,
    duplicates: u64,
    retained: usize,
}

impl AggregationStatistics {
    /// Number of diagnostics submitted to the aggregator.
    #[must_use]
    pub const fn submitted(&self) -> u64 {
        self.submitted
    }

    /// Number of diagnostics retained in the final aggregate.
    #[must_use]
    pub const fn retained(&self) -> usize {
        self.retained
    }

    /// Number of diagnostics that were not retained.
    #[must_use]
    pub const fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Number of duplicate submissions merged into retained diagnostics.
    #[must_use]
    pub const fn duplicates(&self) -> u64 {
        self.duplicates
    }
}

/// The collection that failed to reserve additional capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationCollection {
    Diagnostics,
    IdentityIndex,
}

impl fmt::Display for AllocationCollection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Diagnostics => formatter.write_str("diagnostic collection"),
            Self::IdentityIndex => formatter.write_str("diagnostic identity index"),
        }
    }
}

/// Errors encountered while aggregating diagnostics.
///
/// These are compiler infrastructure failures, not source-language
/// diagnostics. The compiler orchestration layer must preserve this
/// distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AggregationError {
    /// A statistics counter could not be incremented without overflow.
    CounterOverflow {
        counter: &'static str,
    },

    /// A collection could not reserve the capacity required for insertion.
    AllocationFailure {
        collection: AllocationCollection,
    },
}

impl fmt::Display for AggregationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow { counter } => {
                write!(formatter, "diagnostic aggregation counter overflow: {counter}")
            }
            Self::AllocationFailure { collection } => {
                write!(formatter, "unable to reserve memory for {collection}")
            }
        }
    }
}

impl std::error::Error for AggregationError {}

/// The identity used to determine whether two diagnostics describe the same
/// report.
///
/// This deliberately matches `Diagnostic::same_report_identity`.
///
/// Supplemental details such as labels, notes, help, fixes, and metadata do
/// not change report identity. They are used by the canonical comparator to
/// select a deterministic representative.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiagnosticIdentity {
    code: DiagnosticCode,
    phase: DiagnosticPhase,
    category: DiagnosticCategory,
    severity: DiagnosticSeverity,
    primary_span: Option<Span>,
    message: String,
}

impl DiagnosticIdentity {
    fn from_diagnostic(diagnostic: &Diagnostic) -> Self {
        Self {
            code: diagnostic.code().clone(),
            phase: diagnostic.phase().clone(),
            category: diagnostic.category().clone(),
            severity: diagnostic.severity(),
            primary_span: diagnostic.primary_span(),
            message: diagnostic.message().to_owned(),
        }
    }
}

/// Mutable accumulator for one compiler invocation.
///
/// This type is intentionally invocation-local. It has no global state and
/// can be owned by a compilation session.
///
/// It is not internally synchronized. Parallel compiler workers should send
/// diagnostics to the owning session or merge worker-local aggregates at a
/// defined synchronization boundary.
#[derive(Debug)]
pub struct DiagnosticAggregator {
    policy: AggregationPolicy,
    diagnostics: Vec<Diagnostic>,
    identities: HashMap<DiagnosticIdentity, usize>,
    statistics: AggregationStatistics,
    status: AggregationStatus,
}

impl Default for DiagnosticAggregator {
    fn default() -> Self {
        Self::new(AggregationPolicy::default())
    }
}

impl DiagnosticAggregator {
    /// Create an empty aggregator using the supplied policy.
    #[must_use]
    pub fn new(policy: AggregationPolicy) -> Self {
        Self {
            policy,
            diagnostics: Vec::new(),
            identities: HashMap::new(),
            statistics: AggregationStatistics::default(),
            status: AggregationStatus::Complete,
        }
    }

    /// Return the immutable aggregation policy.
    #[must_use]
    pub const fn policy(&self) -> AggregationPolicy {
        self.policy
    }

    /// Return the current completion status.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Return the current statistics.
    ///
    /// `retained` is refreshed when the aggregate is finalized.
    #[must_use]
    pub const fn statistics(&self) -> &AggregationStatistics {
        &self.statistics
    }

    /// Return the number of diagnostics currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostics are currently retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Submit one structured diagnostic.
    ///
    /// Duplicate detection occurs before the retention-budget check. Thus a
    /// duplicate of an already retained diagnostic can still contribute
    /// supplemental information when the retention budget is full.
    ///
    /// When deduplication is enabled and two diagnostics have the same
    /// report identity, the canonical comparator selects the representative.
    /// This makes the result independent of which worker submitted the
    /// duplicate first.
    ///
    /// A dropped diagnostic marks the aggregate incomplete.
    pub fn push(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<InsertOutcome, AggregationError> {
        self.increment_submitted()?;

        if self.policy.deduplicate {
            let identity = DiagnosticIdentity::from_diagnostic(&diagnostic);

            if let Some(&existing_index) = self.identities.get(&identity) {
                self.increment_duplicates()?;

                let existing = &self.diagnostics[existing_index];

                if compare_diagnostics(&diagnostic, existing).is_lt() {
                    self.diagnostics[existing_index] = diagnostic;
                }

                return Ok(InsertOutcome::DuplicateMerged);
            }

            if self.at_retention_limit() {
                self.record_drop()?;
                return Ok(InsertOutcome::DroppedByBudget);
            }

            self.reserve_diagnostic_capacity()?;
            self.reserve_identity_capacity()?;

            let index = self.diagnostics.len();
            self.diagnostics.push(diagnostic);
            self.identities.insert(identity, index);

            return Ok(InsertOutcome::Retained);
        }

        if self.at_retention_limit() {
            self.record_drop()?;
            return Ok(InsertOutcome::DroppedByBudget);
        }

        self.reserve_diagnostic_capacity()?;
        self.diagnostics.push(diagnostic);

        Ok(InsertOutcome::Retained)
    }

    /// Submit diagnostics from another compilation stage.
    ///
    /// The iterator is processed in its supplied order. Each accepted item
    /// follows the same duplicate, budget, and failure rules as `push`.
    ///
    /// If an infrastructure error occurs, processing stops and the error is
    /// returned. The aggregate remains marked incomplete.
    pub fn extend<I>(
        &mut self,
        diagnostics: I,
    ) -> Result<(), AggregationError>
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        for diagnostic in diagnostics {
            self.push(diagnostic)?;
        }

        Ok(())
    }

    /// Consume the aggregator and produce its final deterministic report.
    ///
    /// The final vector is ordered by the canonical semantic diagnostic
    /// comparator. This method does not reinterpret errors as successful
    /// compilation or infer semantic validity.
    #[must_use]
    pub fn finish(mut self) -> AggregatedDiagnostics {
        sort_diagnostics(&mut self.diagnostics);

        self.statistics.retained = self.diagnostics.len();

        AggregatedDiagnostics {
            diagnostics: self.diagnostics,
            status: self.status,
            statistics: self.statistics,
        }
    }

    fn at_retention_limit(&self) -> bool {
        self.policy
            .max_diagnostics
            .is_some_and(|limit| self.diagnostics.len() >= limit.get())
    }

    fn reserve_diagnostic_capacity(
        &mut self,
    ) -> Result<(), AggregationError> {
        if self.diagnostics.try_reserve(1).is_err() {
            self.status = AggregationStatus::Incomplete;
            self.record_drop()?;

            return Err(AggregationError::AllocationFailure {
                collection: AllocationCollection::Diagnostics,
            });
        }

        Ok(())
    }

    fn reserve_identity_capacity(
        &mut self,
    ) -> Result<(), AggregationError> {
        if self.identities.try_reserve(1).is_err() {
            self.status = AggregationStatus::Incomplete;
            self.record_drop()?;

            return Err(AggregationError::AllocationFailure {
                collection: AllocationCollection::IdentityIndex,
            });
        }

        Ok(())
    }

    fn increment_submitted(&mut self) -> Result<(), AggregationError> {
        match self.statistics.submitted.checked_add(1) {
            Some(value) => {
                self.statistics.submitted = value;
                Ok(())
            }
            None => {
                self.status = AggregationStatus::Incomplete;

                Err(AggregationError::CounterOverflow {
                    counter: "submitted",
                })
            }
        }
    }

    fn increment_duplicates(&mut self) -> Result<(), AggregationError> {
        match self.statistics.duplicates.checked_add(1) {
            Some(value) => {
                self.statistics.duplicates = value;
                Ok(())
            }
            None => {
                self.status = AggregationStatus::Incomplete;

                Err(AggregationError::CounterOverflow {
                    counter: "duplicates",
                })
            }
        }
    }

    fn record_drop(&mut self) -> Result<(), AggregationError> {
        self.status = AggregationStatus::Incomplete;

        match self.statistics.dropped.checked_add(1) {
            Some(value) => {
                self.statistics.dropped = value;
                Ok(())
            }
            None => Err(AggregationError::CounterOverflow {
                counter: "dropped",
            }),
        }
    }
}

/// Final immutable result of diagnostic aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedDiagnostics {
    diagnostics: Vec<Diagnostic>,
    status: AggregationStatus,
    statistics: AggregationStatistics,
}

impl AggregatedDiagnostics {
    /// Return the diagnostics in deterministic reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Consume the report and return its ordered diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Return the aggregation completion status.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Whether aggregation completed without dropping diagnostics.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Return aggregation statistics.
    #[must_use]
    pub const fn statistics(&self) -> &AggregationStatistics {
        &self.statistics
    }

    /// Whether at least one retained diagnostic has error severity.
    ///
    /// This is not a substitute for the compilation pipeline's final
    /// validity or completion state.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Whether at least one retained diagnostic has fatal severity.
    #[must_use]
    pub fn has_fatal_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_fatal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diagnostic(code: &str, message: &str) -> Diagnostic {
        Diagnostic::new(
            code,
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            message,
            None,
        )
    }

    #[test]
    fn empty_aggregation_is_complete() {
        let result = DiagnosticAggregator::default().finish();

        assert!(result.is_complete());
        assert!(result.diagnostics().is_empty());
        assert_eq!(result.statistics().submitted(), 0);
        assert_eq!(result.statistics().retained(), 0);
        assert_eq!(result.statistics().dropped(), 0);
    }

    #[test]
    fn retains_diagnostics_without_a_fixed_default_limit() {
        let mut aggregator = DiagnosticAggregator::default();

        for index in 0..1_000 {
            let code = format!("test.diagnostic.{index}");
            aggregator
                .push(diagnostic(&code, "test"))
                .expect("diagnostic should be retained");
        }

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.statistics().retained(), 1_000);
        assert_eq!(result.statistics().dropped(), 0);
    }

    #[test]
    fn explicit_budget_marks_aggregation_incomplete() {
        let policy = AggregationPolicy::with_limit(
            NonZeroUsize::new(2).expect("two is non-zero"),
        );

        let mut aggregator = DiagnosticAggregator::new(policy);

        assert_eq!(
            aggregator.push(diagnostic("test.one", "one")),
            Ok(InsertOutcome::Retained),
        );

        assert_eq!(
            aggregator.push(diagnostic("test.two", "two")),
            Ok(InsertOutcome::Retained),
        );

        assert_eq!(
            aggregator.push(diagnostic("test.three", "three")),
            Ok(InsertOutcome::DroppedByBudget),
        );

        let result = aggregator.finish();

        assert!(!result.is_complete());
        assert_eq!(result.statistics().submitted(), 3);
        assert_eq!(result.statistics().retained(), 2);
        assert_eq!(result.statistics().dropped(), 1);
    }

    #[test]
    fn duplicate_reports_are_merged() {
        let mut aggregator = DiagnosticAggregator::default();

        let first = diagnostic("test.duplicate", "same report");
        let second = first.clone();

        assert_eq!(
            aggregator.push(first),
            Ok(InsertOutcome::Retained),
        );

        assert_eq!(
            aggregator.push(second),
            Ok(InsertOutcome::DuplicateMerged),
        );

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.diagnostics().len(), 1);
        assert_eq!(result.statistics().duplicates(), 1);
        assert_eq!(result.statistics().submitted(), 2);
    }

    #[test]
    fn duplicate_merge_is_not_blocked_by_a_full_budget() {
        let policy = AggregationPolicy::with_limit(
            NonZeroUsize::new(1).expect("one is non-zero"),
        );

        let mut aggregator = DiagnosticAggregator::new(policy);

        let item = diagnostic("test.same", "same report");

        assert_eq!(
            aggregator.push(item.clone()),
            Ok(InsertOutcome::Retained),
        );

        assert_eq!(
            aggregator.push(item),
            Ok(InsertOutcome::DuplicateMerged),
        );

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.statistics().retained(), 1);
        assert_eq!(result.statistics().dropped(), 0);
        assert_eq!(result.statistics().duplicates(), 1);
    }

    #[test]
    fn distinct_report_identities_are_not_collapsed() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .push(diagnostic("test.first", "first"))
            .expect("first diagnostic");

        aggregator
            .push(diagnostic("test.second", "second"))
            .expect("second diagnostic");

        let result = aggregator.finish();

        assert_eq!(result.diagnostics().len(), 2);
    }

    #[test]
    fn final_order_is_canonical_and_deterministic() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .push(diagnostic("test.z", "z"))
            .expect("diagnostic");

        aggregator
            .push(diagnostic("test.a", "a"))
            .expect("diagnostic");

        aggregator
            .push(diagnostic("test.m", "m"))
            .expect("diagnostic");

        let result = aggregator.finish();

        let codes: Vec<&str> = result
            .diagnostics()
            .iter()
            .map(|item| item.code().as_str())
            .collect();

        assert_eq!(
            codes,
            vec!["test.a", "test.m", "test.z"],
        );
    }

    #[test]
    fn error_presence_is_reported_without_claiming_validity() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .push(diagnostic("test.error", "an error"))
            .expect("diagnostic");

        let result = aggregator.finish();

        assert!(result.has_errors());
        assert!(!result.has_fatal_errors());
    }

    #[test]
    fn disabling_deduplication_preserves_repeated_reports() {
        let policy = AggregationPolicy::unlimited()
            .with_deduplication(false);

        let mut aggregator = DiagnosticAggregator::new(policy);

        let item = diagnostic("test.repeated", "repeated");

        aggregator
            .push(item.clone())
            .expect("first diagnostic");

        aggregator
            .push(item)
            .expect("second diagnostic");

        let result = aggregator.finish();

        assert_eq!(result.diagnostics().len(), 2);
        assert_eq!(result.statistics().duplicates(), 0);
    }

    #[test]
    fn extension_accepts_diagnostics_from_multiple_stages() {
        let mut aggregator = DiagnosticAggregator::default();

        let frontend = vec![
            diagnostic("frontend.example", "frontend finding"),
        ];

        let backend = vec![
            diagnostic("backend.example", "backend finding"),
        ];

        aggregator.extend(frontend).expect("frontend diagnostics");
        aggregator.extend(backend).expect("backend diagnostics");

        let result = aggregator.finish();

        assert_eq!(result.diagnostics().len(), 2);
        assert_eq!(result.statistics().submitted(), 2);
    }
}
