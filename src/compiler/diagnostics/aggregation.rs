
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler Diagnostic Aggregation
//!
//! Path: src/compiler/diagnostics/aggregation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Aggregate canonical diagnostics produced by different compilation
//! stages into one deterministic compilation report.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - compiler-invocation diagnostic accumulation;
//! - diagnostic retention policies;
//! - duplicate-report indexing;
//! - aggregation statistics;
//! - aggregation completeness;
//! - deterministic final ordering.
//!
//! This module does not own:
//! - Diagnostic definitions;
//! - diagnostic codes;
//! - diagnostic severity;
//! - diagnostic categories;
//! - source spans;
//! - source maps;
//! - semantic validation;
//! - parser recovery;
//! - diagnostic rendering;
//! - JSON serialization;
//! - compilation success or failure decisions.
//!
//! CANONICAL AUTHORITIES
//!
//! Diagnostic:
//! crate::frontend::semantic::diagnostics
//!
//! Diagnostic identity:
//! Diagnostic::same_report_identity
//!
//! Diagnostic ordering:
//! crate::frontend::semantic::diagnostics::ordering
//!
//! Source spans:
//! crate::frontend::ast::node::source::span
//!
//! Reporting:
//! crate::compiler::diagnostics::reporting
//!
//! Rendering:
//! crate::compiler::diagnostics::rendering
//!
//! SCALABILITY
//!
//! No universal diagnostic-count ceiling is imposed.
//!
//! An optional retention budget is an operational policy.
//!
//! Actual capacity is bounded by available memory and implementation
//! representation limits.
//!
//! Exhausting a budget must produce an explicitly incomplete aggregate.
//!
//! DETERMINISM
//!
//! Diagnostic identity is checked using the canonical frontend identity.
//!
//! Hash-map iteration is never used for reporting order.
//!
//! Duplicate representatives are selected using the canonical
//! diagnostic comparator.
//!
//! Final reports use canonical frontend ordering.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No global mutable state.
//! No hardware or runtime access.
//! No filesystem or network access.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::num::NonZeroUsize;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

use crate::frontend::semantic::diagnostics::{
    compare_diagnostics,
    sort_diagnostics,
    Diagnostic,
};

/// Immutable policy for one compiler diagnostic aggregation operation.
///
/// This policy controls retention and duplicate handling only.
///
/// It does not change diagnostic meaning, severity, source coordinates,
/// semantic validity, or target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AggregationPolicy {
    /// Maximum number of unique diagnostics retained.
    ///
    /// None means no explicitly configured retention limit.
    ///
    /// This is an operational budget, not a language-level limit.
    pub max_diagnostics: Option<NonZeroUsize>,

    /// Whether duplicate report identities are collapsed.
    pub deduplicate: bool,
}

impl Default for AggregationPolicy {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl AggregationPolicy {
    /// Create an unlimited-retention policy.
    ///
    /// Unlimited means no configured diagnostic-count ceiling.
    /// It does not promise infinite physical memory.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_diagnostics: None,
            deduplicate: true,
        }
    }

    /// Create a policy with an explicit retention budget.
    #[must_use]
    pub const fn with_limit(
        max_diagnostics: NonZeroUsize,
    ) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
            deduplicate: true,
        }
    }

    /// Enable or disable duplicate collapsing.
    #[must_use]
    pub const fn with_deduplication(
        mut self,
        enabled: bool,
    ) -> Self {
        self.deduplicate = enabled;
        self
    }
}

/// Completion state of diagnostic aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregationStatus {
    /// Every submitted diagnostic was retained or deduplicated.
    Complete,

    /// At least one diagnostic could not be retained or accounting
    /// could not be completed.
    Incomplete,
}

impl AggregationStatus {
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Result of submitting one diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertOutcome {
    /// A new diagnostic was retained.
    Retained,

    /// An existing report identity was found and deduplicated.
    ///
    /// The canonical comparator determines the retained representative.
    DuplicateMerged,

    /// The diagnostic was rejected by the configured retention budget.
    DroppedByBudget,
}

/// Statistics for one aggregation operation.
///
/// submitted:
/// Number of successfully accounted submissions.
///
/// retained:
/// Number of diagnostics in the final aggregate.
///
/// dropped:
/// Number of diagnostics that could not be retained.
///
/// duplicates:
/// Number of duplicate submissions collapsed.
///
/// These counters do not establish semantic validity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AggregationStatistics {
    submitted: u64,
    dropped: u64,
    duplicates: u64,
    retained: usize,
}

impl AggregationStatistics {
    #[must_use]
    pub const fn submitted(&self) -> u64 {
        self.submitted
    }

    #[must_use]
    pub const fn retained(&self) -> usize {
        self.retained
    }

    #[must_use]
    pub const fn dropped(&self) -> u64 {
        self.dropped
    }

    #[must_use]
    pub const fn duplicates(&self) -> u64 {
        self.duplicates
    }
}

/// Identifies the collection that failed to reserve memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationCollection {
    Diagnostics,
    IdentityIndex,
}

impl fmt::Display for AllocationCollection {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Diagnostics => {
                formatter.write_str("diagnostic collection")
            }
            Self::IdentityIndex => {
                formatter.write_str("diagnostic identity index")
            }
        }
    }
}

/// Infrastructure errors encountered during aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AggregationError {
    /// A statistics counter exceeded its representation.
    CounterOverflow {
        counter: &'static str,
    },

    /// Memory reservation failed.
    AllocationFailure {
        collection: AllocationCollection,
    },
}

impl fmt::Display for AggregationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "diagnostic aggregation counter overflow: {counter}"
                )
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for {collection}"
                )
            }
        }
    }
}

impl std::error::Error for AggregationError {}

/// Compute an internal hash of the canonical report identity.
///
/// IMPORTANT:
///
/// The hash is only an indexing optimization.
///
/// Hash equality never establishes diagnostic identity.
/// Every candidate is checked with Diagnostic::same_report_identity.
///
/// The hash is not serialized and is not part of the public API.
/// Its value is not required to remain stable across Rust versions.
fn report_identity_hash(
    diagnostic: &Diagnostic,
) -> u64 {
    let mut hasher = DefaultHasher::new();

    diagnostic.code().hash(&mut hasher);
    diagnostic.phase().hash(&mut hasher);
    diagnostic.category().hash(&mut hasher);
    diagnostic.severity().hash(&mut hasher);
    diagnostic.primary_span().hash(&mut hasher);
    diagnostic.message().hash(&mut hasher);

    hasher.finish()
}

/// Mutable diagnostic accumulator for one compilation invocation.
///
/// The aggregator is deliberately not internally synchronized.
///
/// Parallel compiler workers should maintain independent accumulators
/// and transfer their diagnostics to the owning compilation session
/// through an explicit synchronization boundary.
///
/// The final ordering is independent of submission order.
#[derive(Debug)]
pub struct DiagnosticAggregator {
    policy: AggregationPolicy,

    diagnostics: Vec<Diagnostic>,

    /// Internal hash -> candidate diagnostic positions.
    ///
    /// Candidate positions are always verified using the canonical
    /// diagnostic identity method.
    ///
    /// The index is not iterated to produce output.
    identities: HashMap<u64, Vec<usize>>,

    statistics: AggregationStatistics,

    status: AggregationStatus,
}

impl Default for DiagnosticAggregator {
    fn default() -> Self {
        Self::new(AggregationPolicy::default())
    }
}

impl DiagnosticAggregator {
    /// Create an empty aggregator.
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

    /// Return the aggregation completion state.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Return the current statistics.
    ///
    /// The retained counter is finalized by finish().
    #[must_use]
    pub const fn statistics(&self) -> &AggregationStatistics {
        &self.statistics
    }

    /// Number of diagnostics currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether the aggregator contains no diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Submit one canonical diagnostic.
    ///
    /// Duplicate detection occurs before the retention-budget check.
    ///
    /// Therefore an already-retained diagnostic can still be recognized
    /// when the retention budget is full.
    ///
    /// Duplicate identity is established exclusively by the canonical
    /// frontend Diagnostic::same_report_identity method.
    ///
    /// When duplicate collapsing is enabled, the canonical comparator
    /// chooses the representative. Supplemental fields belonging only
    /// to the discarded duplicate are not independently merged.
    ///
    /// A future lossless supplemental-information merge requires a
    /// corresponding canonical Diagnostic API change.
    pub fn push(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<InsertOutcome, AggregationError> {
        self.increment_submitted()?;

        if self.policy.deduplicate {
            self.push_with_deduplication(diagnostic)
        } else {
            self.push_without_deduplication(diagnostic)
        }
    }

    /// Submit diagnostics produced by another compiler stage.
    ///
    /// Processing stops on the first infrastructure error.
    ///
    /// Previously accepted diagnostics remain in the aggregate.
    /// The caller must preserve the returned infrastructure error.
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

    /// Consume the aggregator and produce a deterministic report.
    ///
    /// Sorting is delegated to the canonical frontend implementation.
    ///
    /// This operation does not decide compilation success.
    #[must_use]
    pub fn finish(
        mut self,
    ) -> AggregatedDiagnostics {
        sort_diagnostics(&mut self.diagnostics);

        self.statistics.retained = self.diagnostics.len();

        AggregatedDiagnostics {
            diagnostics: self.diagnostics,
            status: self.status,
            statistics: self.statistics,
        }
    }

    fn push_with_deduplication(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<InsertOutcome, AggregationError> {
        let identity_hash = report_identity_hash(&diagnostic);

        if let Some(indices) = self.identities.get(&identity_hash) {
            if let Some(&existing_index) = indices.iter().find(
                |&&index| {
                    self.diagnostics[index]
                        .same_report_identity(&diagnostic)
                },
            ) {
                self.increment_duplicates()?;

                let existing = &self.diagnostics[existing_index];

                if compare_diagnostics(
                    &diagnostic,
                    existing,
                ).is_lt() {
                    self.diagnostics[existing_index] = diagnostic;
                }

                return Ok(InsertOutcome::DuplicateMerged);
            }
        }

        if self.at_retention_limit() {
            self.record_drop()?;
            return Ok(InsertOutcome::DroppedByBudget);
        }

        self.reserve_diagnostic_capacity()?;

        let index_exists = self.identities.contains_key(&identity_hash);

        if index_exists {
            let indices = self
                .identities
                .get_mut(&identity_hash)
                .expect("identity existence was checked");

            if indices.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                self.record_drop()?;

                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityIndex,
                });
            }

            let diagnostic_index = self.diagnostics.len();

            self.diagnostics.push(diagnostic);
            indices.push(diagnostic_index);
        } else {
            if self.identities.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                self.record_drop()?;

                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityIndex,
                });
            }

            let mut indices = Vec::new();

            if indices.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                self.record_drop()?;

                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityIndex,
                });
            }

            let diagnostic_index = self.diagnostics.len();

            self.diagnostics.push(diagnostic);
            indices.push(diagnostic_index);

            self.identities.insert(identity_hash, indices);
        }

        Ok(InsertOutcome::Retained)
    }

    fn push_without_deduplication(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<InsertOutcome, AggregationError> {
        if self.at_retention_limit() {
            self.record_drop()?;
            return Ok(InsertOutcome::DroppedByBudget);
        }

        self.reserve_diagnostic_capacity()?;

        self.diagnostics.push(diagnostic);

        Ok(InsertOutcome::Retained)
    }

    fn at_retention_limit(&self) -> bool {
        self.policy
            .max_diagnostics
            .is_some_and(|limit| {
                self.diagnostics.len() >= limit.get()
            })
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

    fn increment_submitted(
        &mut self,
    ) -> Result<(), AggregationError> {
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

    fn increment_duplicates(
        &mut self,
    ) -> Result<(), AggregationError> {
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

    fn record_drop(
        &mut self,
    ) -> Result<(), AggregationError> {
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

/// Immutable finalized aggregation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedDiagnostics {
    diagnostics: Vec<Diagnostic>,
    status: AggregationStatus,
    statistics: AggregationStatistics,
}

impl AggregatedDiagnostics {
    /// Borrow diagnostics in canonical reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Consume the result and return its diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Return the aggregation completion state.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Whether every submitted diagnostic was accounted for.
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
    /// This does not establish compilation validity.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(Diagnostic::is_error)
    }

    /// Whether at least one retained diagnostic has fatal severity.
    #[must_use]
    pub fn has_fatal_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(Diagnostic::is_fatal)
    }

    /// Whether the aggregate contains no retained diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Number of retained diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Iterate over retained diagnostics.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::diagnostics::{
        DiagnosticCategory,
        DiagnosticPhase,
        DiagnosticSeverity,
    };

    fn diagnostic(
        code: &str,
        message: &str,
    ) -> Diagnostic {
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
        assert!(result.is_empty());
        assert_eq!(result.statistics().submitted(), 0);
        assert_eq!(result.statistics().retained(), 0);
        assert_eq!(result.statistics().dropped(), 0);
    }

    #[test]
    fn retains_without_a_fixed_default_limit() {
        let mut aggregator = DiagnosticAggregator::default();

        for index in 0..1_000 {
            let code = format!("test.diagnostic.{index}");

            aggregator
                .push(diagnostic(&code, "test"))
                .expect("diagnostic should be retained");
        }

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.len(), 1_000);
        assert_eq!(result.statistics().dropped(), 0);
    }

    #[test]
    fn configured_budget_marks_result_incomplete() {
        let policy = AggregationPolicy::with_limit(
            NonZeroUsize::new(2).expect("two is nonzero"),
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
    fn duplicate_reports_are_collapsed() {
        let mut aggregator = DiagnosticAggregator::default();

        let first = diagnostic(
            "test.duplicate",
            "same report",
        );

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
        assert_eq!(result.len(), 1);
        assert_eq!(result.statistics().duplicates(), 1);
        assert_eq!(result.statistics().submitted(), 2);
    }

    #[test]
    fn duplicate_detection_works_when_budget_is_full() {
        let policy = AggregationPolicy::with_limit(
            NonZeroUsize::new(1).expect("one is nonzero"),
        );

        let mut aggregator = DiagnosticAggregator::new(policy);

        let item = diagnostic("test.same", "same report");

        aggregator
            .push(item.clone())
            .expect("first report");

        assert_eq!(
            aggregator.push(item),
            Ok(InsertOutcome::DuplicateMerged),
        );

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.len(), 1);
        assert_eq!(result.statistics().dropped(), 0);
    }

    #[test]
    fn distinct_identities_are_preserved() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .push(diagnostic("test.first", "first"))
            .expect("first");

        aggregator
            .push(diagnostic("test.second", "second"))
            .expect("second");

        let result = aggregator.finish();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn canonical_order_is_deterministic() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator.push(diagnostic("test.z", "z")).unwrap();
        aggregator.push(diagnostic("test.a", "a")).unwrap();
        aggregator.push(diagnostic("test.m", "m")).unwrap();

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
    fn error_detection_does_not_claim_compilation_validity() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .push(diagnostic("test.error", "an error"))
            .unwrap();

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

        aggregator.push(item.clone()).unwrap();
        aggregator.push(item).unwrap();

        let result = aggregator.finish();

        assert_eq!(result.len(), 2);
        assert_eq!(result.statistics().duplicates(), 0);
    }

    #[test]
    fn different_compilation_stages_can_submit_diagnostics() {
        let mut aggregator = DiagnosticAggregator::default();

        aggregator
            .extend(vec![
                diagnostic("frontend.example", "frontend finding"),
            ])
            .unwrap();

        aggregator
            .extend(vec![
                diagnostic("backend.example", "backend finding"),
            ])
            .unwrap();

        let result = aggregator.finish();

        assert_eq!(result.len(), 2);
        assert_eq!(result.statistics().submitted(), 2);
    }
}
