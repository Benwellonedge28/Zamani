
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Diagnostic Aggregation
//!
//! Aggregates structured toolchain diagnostics while preserving completeness,
//! deterministic ordering, provenance and explicit resource-budget outcomes.
//!
//! This module does not own diagnostic construction, source maps, rendering,
//! serialization envelopes, compiler diagnostics or operation execution.
//!
//! The canonical diagnostic record is defined in `diagnostic.rs`.
//!
//! # Scalability
//!
//! The default policy has no configured diagnostic-count ceiling.
//! Memory consumption is therefore bounded by available resources unless
//! the caller explicitly configures a retention budget.
//!
//! A configured retention budget is an operational policy, not a language
//! or hardware capability limit.
//!
//! Allocation failure, invalid input or retention truncation must never be
//! reported as complete aggregation.
//!
//! # Determinism
//!
//! Final ordering is independent of hash-map iteration order.
//! Deduplication uses a hash only to locate candidates; exact structural
//! equality determines whether two diagnostics are duplicates.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;

use super::diagnostic::{
    DiagnosticFix,
    DiagnosticLabel,
    DiagnosticNote,
    DiagnosticSeverity,
    DiagnosticValidationError,
    DiagnosticValue,
    ToolchainDiagnostic,
};

// ============================================================================
// POLICY
// ============================================================================

/// Controls diagnostic retention and duplicate handling.
///
/// `max_diagnostics = None` means no configured count limit.
///
/// A configured limit controls retained diagnostic records only. It does
/// not change the meaning of the program, the language, or the capabilities
/// of any computing domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregationPolicy {
    max_diagnostics: Option<usize>,
    deduplicate: bool,
}

impl AggregationPolicy {
    /// Create a policy without a configured diagnostic-count limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_diagnostics: None,
            deduplicate: true,
        }
    }

    /// Create a policy with an explicit retention budget.
    ///
    /// A zero limit is valid and retains no diagnostic records.
    #[must_use]
    pub const fn with_max_diagnostics(max_diagnostics: usize) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
            deduplicate: true,
        }
    }

    /// Set whether exact duplicate diagnostics should be removed.
    #[must_use]
    pub const fn with_deduplication(mut self, enabled: bool) -> Self {
        self.deduplicate = enabled;
        self
    }

    /// Return the configured retention limit.
    ///
    /// `None` means unlimited retention subject to available resources.
    #[must_use]
    pub const fn max_diagnostics(&self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Return whether exact duplicate detection is enabled.
    #[must_use]
    pub const fn deduplication_enabled(&self) -> bool {
        self.deduplicate
    }
}

impl Default for AggregationPolicy {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// STATUS
// ============================================================================

/// Describes whether aggregation retained all diagnostics presented to it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AggregationStatus {
    /// No known aggregation loss or invalid input occurred.
    #[default]
    Complete,

    /// Aggregation may be incomplete because of a budget, invalid input,
    /// allocation failure or another explicitly reported condition.
    Incomplete,
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Diagnostic aggregation counters.
///
/// Counters use `u64` so their serialized and reported representation does
/// not depend on the width of the host machine.
///
/// Counter overflow is reported as an aggregation error. It is never
/// silently wrapped.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AggregationStatistics {
    observed: u64,
    retained: u64,
    duplicates: u64,
    dropped: u64,
    invalid: u64,
}

impl AggregationStatistics {
    /// Number of diagnostics submitted to the aggregator.
    #[must_use]
    pub const fn observed(&self) -> u64 {
        self.observed
    }

    /// Number of diagnostics retained.
    #[must_use]
    pub const fn retained(&self) -> u64 {
        self.retained
    }

    /// Number of exact duplicates not retained.
    #[must_use]
    pub const fn duplicates(&self) -> u64 {
        self.duplicates
    }

    /// Number of diagnostics omitted because of retention policy.
    #[must_use]
    pub const fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Number of invalid diagnostic records rejected.
    #[must_use]
    pub const fn invalid(&self) -> u64 {
        self.invalid
    }
}

// ============================================================================
// INSERTION RESULT
// ============================================================================

/// Result of submitting one diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertOutcome {
    /// The diagnostic was retained.
    Inserted,

    /// An exactly equal diagnostic was already retained.
    Duplicate,

    /// The configured retention budget prevented retention.
    DroppedByPolicy,
}

// ============================================================================
// ALLOCATION CLASSIFICATION
// ============================================================================

/// Identifies the collection whose allocation reservation failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationCollection {
    /// Storage for retained diagnostic records.
    DiagnosticStorage,

    /// Hash index used for duplicate candidate lookup.
    IdentityIndex,

    /// Candidate indices associated with one diagnostic hash.
    IdentityBucket,
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while aggregating diagnostics.
#[derive(Debug)]
#[non_exhaustive]
pub enum AggregationError {
    /// A submitted diagnostic violated its local invariants.
    InvalidDiagnostic(DiagnosticValidationError),

    /// A collection could not reserve the required additional capacity.
    AllocationFailure {
        /// Collection that could not reserve capacity.
        collection: AllocationCollection,
    },

    /// An aggregation counter exceeded its representable range.
    CounterOverflow {
        /// Counter that could not be incremented.
        counter: &'static str,
    },

    /// An internal hash-index entry referred to a missing diagnostic.
    IdentityIndexInvariant,
}

impl fmt::Display for AggregationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDiagnostic(error) => {
                write!(formatter, "invalid toolchain diagnostic: {error}")
            }
            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for diagnostic aggregation: \
                     {collection:?}"
                )
            }
            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "diagnostic aggregation counter overflow: {counter}"
                )
            }
            Self::IdentityIndexInvariant => {
                formatter.write_str(
                    "diagnostic identity index refers to a missing record",
                )
            }
        }
    }
}

impl Error for AggregationError {}

// ============================================================================
// AGGREGATED RESULT
// ============================================================================

/// Final aggregated diagnostics and their completeness information.
///
/// An incomplete result remains useful for reporting, but consumers must
/// not treat it as proof that every submitted diagnostic was retained.
#[derive(Clone, Debug, PartialEq)]
pub struct AggregatedToolchainDiagnostics {
    diagnostics: Vec<ToolchainDiagnostic>,
    status: AggregationStatus,
    statistics: AggregationStatistics,
}

impl AggregatedToolchainDiagnostics {
    /// Return retained diagnostics in deterministic order.
    #[must_use]
    pub fn diagnostics(&self) -> &[ToolchainDiagnostic] {
        &self.diagnostics
    }

    /// Consume the result and return its diagnostic records.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<ToolchainDiagnostic> {
        self.diagnostics
    }

    /// Return aggregation completeness.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Whether all submitted records were processed without known loss.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.status, AggregationStatus::Complete)
    }

    /// Return aggregation statistics.
    #[must_use]
    pub const fn statistics(&self) -> AggregationStatistics {
        self.statistics
    }

    /// Number of retained diagnostic records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostic records were retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

// ============================================================================
// AGGREGATOR
// ============================================================================

/// Aggregates toolchain diagnostic records.
///
/// The aggregator is intentionally independent of compiler execution,
/// semantic analysis, hardware selection and runtime scheduling.
///
/// It can receive diagnostics from workspace, manifest, dependency,
/// build, task, package, registry, external-tool, target, deployment,
/// security and future toolchain subsystems.
///
/// The aggregator itself is not internally synchronized. Parallel producers
/// should submit through an owning synchronization or channel boundary.
/// Deterministic final ordering does not depend on producer completion order.
#[derive(Debug)]
pub struct ToolchainDiagnosticAggregator {
    diagnostics: Vec<ToolchainDiagnostic>,
    identities: HashMap<u64, Vec<usize>>,
    policy: AggregationPolicy,
    status: AggregationStatus,
    statistics: AggregationStatistics,
}

impl ToolchainDiagnosticAggregator {
    /// Create an aggregator using the default unlimited policy.
    #[must_use]
    pub fn new() -> Self {
        Self::with_policy(AggregationPolicy::default())
    }

    /// Create an aggregator with an explicit policy.
    #[must_use]
    pub fn with_policy(policy: AggregationPolicy) -> Self {
        Self {
            diagnostics: Vec::new(),
            identities: HashMap::new(),
            policy,
            status: AggregationStatus::Complete,
            statistics: AggregationStatistics::default(),
        }
    }

    /// Return the active aggregation policy.
    #[must_use]
    pub fn policy(&self) -> &AggregationPolicy {
        &self.policy
    }

    /// Return current aggregation status.
    #[must_use]
    pub const fn status(&self) -> AggregationStatus {
        self.status
    }

    /// Return current statistics.
    #[must_use]
    pub const fn statistics(&self) -> AggregationStatistics {
        self.statistics
    }

    /// Return the number of retained diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostics are currently retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Explicitly mark the aggregation as incomplete.
    ///
    /// Use this when an upstream producer knows that it could not provide
    /// its complete diagnostic stream.
    pub fn mark_incomplete(&mut self) {
        self.status = AggregationStatus::Incomplete;
    }

    /// Submit one diagnostic for validation, deduplication and retention.
    ///
    /// Invalid records are rejected and mark the aggregate incomplete.
    /// Retention-budget exhaustion also marks the aggregate incomplete.
    ///
    /// Exact duplicate detection compares the complete diagnostic record,
    /// not just its code or message. Diagnostics with different source
    /// labels, notes, fixes, provenance or metadata remain distinct.
    pub fn push(
        &mut self,
        diagnostic: ToolchainDiagnostic,
    ) -> Result<InsertOutcome, AggregationError> {
        self.increment_counter("observed", &mut self.statistics.observed)?;

        if let Err(error) = diagnostic.validate() {
            self.status = AggregationStatus::Incomplete;
            self.increment_counter("invalid", &mut self.statistics.invalid)?;
            return Err(AggregationError::InvalidDiagnostic(error));
        }

        if self.policy.deduplicate {
            let identity = diagnostic_identity(&diagnostic);

            let lookup = self.identities.get(&identity).map(|indices| {
                let mut duplicate = false;
                let mut invalid_index = false;

                for &index in indices {
                    match self.diagnostics.get(index) {
                        Some(existing) => {
                            if existing == &diagnostic {
                                duplicate = true;
                                break;
                            }
                        }
                        None => {
                            invalid_index = true;
                            break;
                        }
                    }
                }

                (duplicate, invalid_index)
            });

            if let Some((duplicate, invalid_index)) = lookup {
                if invalid_index {
                    self.status = AggregationStatus::Incomplete;
                    return Err(AggregationError::IdentityIndexInvariant);
                }

                if duplicate {
                    self.increment_counter(
                        "duplicates",
                        &mut self.statistics.duplicates,
                    )?;

                    return Ok(InsertOutcome::Duplicate);
                }
            }
        }

        if self.at_retention_limit() {
            self.status = AggregationStatus::Incomplete;
            self.increment_counter("dropped", &mut self.statistics.dropped)?;
            return Ok(InsertOutcome::DroppedByPolicy);
        }

        if self.policy.deduplicate {
            let identity = diagnostic_identity(&diagnostic);
            self.insert_indexed(identity, diagnostic)?;
        } else {
            self.insert_unindexed(diagnostic)?;
        }

        self.statistics.retained = match u64::try_from(self.diagnostics.len()) {
            Ok(value) => value,
            Err(_) => {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::CounterOverflow {
                    counter: "retained",
                });
            }
        };

        Ok(InsertOutcome::Inserted)
    }

    /// Submit a sequence of diagnostics.
    ///
    /// Processing stops at the first error. Diagnostics already accepted
    /// remain retained, and the aggregator preserves its incomplete status
    /// whenever an earlier operation established it.
    pub fn extend<I>(
        &mut self,
        diagnostics: I,
    ) -> Result<(), AggregationError>
    where
        I: IntoIterator<Item = ToolchainDiagnostic>,
    {
        for diagnostic in diagnostics {
            let _ = self.push(diagnostic)?;
        }

        Ok(())
    }

    /// Finish aggregation and return a deterministically ordered result.
    ///
    /// The result's status records any known loss or incomplete upstream
    /// input. Sorting does not change completeness.
    #[must_use]
    pub fn finish(mut self) -> AggregatedToolchainDiagnostics {
        sort_diagnostics(&mut self.diagnostics);

        AggregatedToolchainDiagnostics {
            diagnostics: self.diagnostics,
            status: self.status,
            statistics: self.statistics,
        }
    }

    fn at_retention_limit(&self) -> bool {
        match self.policy.max_diagnostics {
            Some(limit) => self.diagnostics.len() >= limit,
            None => false,
        }
    }

    fn increment_counter(
        &mut self,
        name: &'static str,
        counter: &mut u64,
    ) -> Result<(), AggregationError> {
        match counter.checked_add(1) {
            Some(next) => {
                *counter = next;
                Ok(())
            }
            None => {
                self.status = AggregationStatus::Incomplete;
                Err(AggregationError::CounterOverflow {
                    counter: name,
                })
            }
        }
    }

    fn insert_unindexed(
        &mut self,
        diagnostic: ToolchainDiagnostic,
    ) -> Result<(), AggregationError> {
        if self.diagnostics.try_reserve(1).is_err() {
            self.status = AggregationStatus::Incomplete;
            return Err(AggregationError::AllocationFailure {
                collection: AllocationCollection::DiagnosticStorage,
            });
        }

        self.diagnostics.push(diagnostic);
        Ok(())
    }

    fn insert_indexed(
        &mut self,
        identity: u64,
        diagnostic: ToolchainDiagnostic,
    ) -> Result<(), AggregationError> {
        let index = self.diagnostics.len();

        if self.identities.contains_key(&identity) {
            let reservation_failed = self
                .identities
                .get_mut(&identity)
                .map(|indices| indices.try_reserve(1).is_err())
                .unwrap_or(true);

            if reservation_failed {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityBucket,
                });
            }

            if self.diagnostics.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::DiagnosticStorage,
                });
            }

            let Some(indices) = self.identities.get_mut(&identity) else {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::IdentityIndexInvariant);
            };

            self.diagnostics.push(diagnostic);
            indices.push(index);
        } else {
            if self.identities.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityIndex,
                });
            }

            let mut indices = Vec::new();

            if indices.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::IdentityBucket,
                });
            }

            if self.diagnostics.try_reserve(1).is_err() {
                self.status = AggregationStatus::Incomplete;
                return Err(AggregationError::AllocationFailure {
                    collection: AllocationCollection::DiagnosticStorage,
                });
            }

            indices.push(index);
            self.diagnostics.push(diagnostic);

            let _ = self.identities.insert(identity, indices);
        }

        Ok(())
    }
}

impl Default for ToolchainDiagnosticAggregator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PUBLIC DETERMINISTIC ORDERING
// ============================================================================

/// Sort diagnostics into deterministic presentation order.
///
/// Ordering is:
///
/// 1. Higher severity first.
/// 2. Diagnostics with source locations before diagnostics without them.
/// 3. Canonical source-span order.
/// 4. Stable diagnostic code.
/// 5. Category.
/// 6. Phase.
/// 7. Message.
/// 8. Origin and supplemental diagnostic content.
///
/// This ordering is independent of hash-map iteration order.
///
/// It does not claim to represent execution chronology.
pub fn sort_diagnostics(diagnostics: &mut [ToolchainDiagnostic]) {
    diagnostics.sort_by(compare_diagnostics);
}

/// Compare two diagnostics using deterministic presentation ordering.
pub fn compare_diagnostics(
    left: &ToolchainDiagnostic,
    right: &ToolchainDiagnostic,
) -> Ordering {
    severity_rank(left.severity())
        .cmp(&severity_rank(right.severity()))
        .then_with(|| compare_optional_spans(
            left.primary_span(),
            right.primary_span(),
        ))
        .then_with(|| left.code().cmp(right.code()))
        .then_with(|| left.category().cmp(right.category()))
        .then_with(|| left.phase().cmp(right.phase()))
        .then_with(|| left.message().cmp(right.message()))
        .then_with(|| {
            left.origin()
                .component()
                .cmp(right.origin().component())
        })
        .then_with(|| {
            left.origin()
                .operation()
                .cmp(&right.origin().operation())
        })
        .then_with(|| {
            left.origin()
                .subject()
                .cmp(&right.origin().subject())
        })
        .then_with(|| {
            left.origin()
                .provider()
                .cmp(&right.origin().provider())
        })
        .then_with(|| compare_labels(left.labels(), right.labels()))
        .then_with(|| compare_notes(left.notes(), right.notes()))
        .then_with(|| left.help().cmp(&right.help()))
        .then_with(|| compare_fixes(left.fixes(), right.fixes()))
        .then_with(|| compare_metadata(
            left.metadata(),
            right.metadata(),
        ))
}

fn severity_rank(severity: DiagnosticSeverity) -> u8 {
    match severity {
        DiagnosticSeverity::Fatal => 0,
        DiagnosticSeverity::Error => 1,
        DiagnosticSeverity::Warning => 2,
        DiagnosticSeverity::Information => 3,
        DiagnosticSeverity::Note => 4,
        DiagnosticSeverity::Hint => 5,
    }
}

fn compare_optional_spans(
    left: Option<crate::frontend::ast::node::source::span::Span>,
    right: Option<crate::frontend::ast::node::source::span::Span>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn compare_labels(
    left: &[DiagnosticLabel],
    right: &[DiagnosticLabel],
) -> Ordering {
    compare_slices(left, right, |a, b| {
        a.span()
            .cmp(&b.span())
            .then_with(|| a.message().cmp(b.message()))
    })
}

fn compare_notes(
    left: &[DiagnosticNote],
    right: &[DiagnosticNote],
) -> Ordering {
    compare_slices(left, right, |a, b| {
        a.message().cmp(b.message())
    })
}

fn compare_fixes(
    left: &[DiagnosticFix],
    right: &[DiagnosticFix],
) -> Ordering {
    compare_slices(left, right, compare_fixes_one)
}

fn compare_fixes_one(
    left: &DiagnosticFix,
    right: &DiagnosticFix,
) -> Ordering {
    left.title()
        .cmp(right.title())
        .then_with(|| left.is_safe().cmp(&right.is_safe()))
        .then_with(|| {
            compare_slices(left.edits(), right.edits(), |a, b| {
                a.span()
                    .cmp(&b.span())
                    .then_with(|| a.replacement().cmp(b.replacement()))
            })
        })
}

fn compare_metadata(
    left: &std::collections::BTreeMap<String, DiagnosticValue>,
    right: &std::collections::BTreeMap<String, DiagnosticValue>,
) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    loop {
        match (left_iter.next(), right_iter.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some((left_key, left_value)), Some((right_key, right_value))) => {
                let ordering = left_key
                    .cmp(right_key)
                    .then_with(|| compare_values(left_value, right_value));

                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }
    }
}

fn compare_values(
    left: &DiagnosticValue,
    right: &DiagnosticValue,
) -> Ordering {
    let tag_order = value_tag(left).cmp(&value_tag(right));

    if tag_order != Ordering::Equal {
        return tag_order;
    }

    match (left, right) {
        (DiagnosticValue::Null, DiagnosticValue::Null) => Ordering::Equal,

        (DiagnosticValue::Boolean(a), DiagnosticValue::Boolean(b)) => {
            a.cmp(b)
        }

        (DiagnosticValue::Signed(a), DiagnosticValue::Signed(b)) => {
            a.cmp(b)
        }

        (DiagnosticValue::Unsigned(a), DiagnosticValue::Unsigned(b)) => {
            a.cmp(b)
        }

        (DiagnosticValue::Float(a), DiagnosticValue::Float(b)) => {
            a.total_cmp(b)
        }

        (DiagnosticValue::Text(a), DiagnosticValue::Text(b)) => {
            a.cmp(b)
        }

        (DiagnosticValue::Sequence(a), DiagnosticValue::Sequence(b)) => {
            compare_slices(a, b, compare_values)
        }

        (DiagnosticValue::Object(a), DiagnosticValue::Object(b)) => {
            let mut left_iter = a.iter();
            let mut right_iter = b.iter();

            loop {
                match (left_iter.next(), right_iter.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (
                        Some((left_key, left_value)),
                        Some((right_key, right_value)),
                    ) => {
                        let ordering = left_key
                            .cmp(right_key)
                            .then_with(|| {
                                compare_values(left_value, right_value)
                            });

                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                }
            }
        }
    }
}

fn value_tag(value: &DiagnosticValue) -> u8 {
    match value {
        DiagnosticValue::Null => 0,
        DiagnosticValue::Boolean(_) => 1,
        DiagnosticValue::Signed(_) => 2,
        DiagnosticValue::Unsigned(_) => 3,
        DiagnosticValue::Float(_) => 4,
        DiagnosticValue::Text(_) => 5,
        DiagnosticValue::Sequence(_) => 6,
        DiagnosticValue::Object(_) => 7,
    }
}

fn compare_slices<T, F>(
    left: &[T],
    right: &[T],
    mut compare: F,
) -> Ordering
where
    F: FnMut(&T, &T) -> Ordering,
{
    for (left_item, right_item) in left.iter().zip(right.iter()) {
        let ordering = compare(left_item, right_item);

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    left.len().cmp(&right.len())
}

// ============================================================================
// DUPLICATE IDENTITY
// ============================================================================

/// Compute an in-process hash used only to locate possible duplicates.
///
/// The hash is not a public diagnostic identifier, is not serialized and
/// must not be used as a reproducible build fingerprint.
///
/// Exact equality is always checked after candidate lookup, so hash
/// collisions cannot cause distinct diagnostics to be discarded.
fn diagnostic_identity(diagnostic: &ToolchainDiagnostic) -> u64 {
    let mut hasher = DefaultHasher::new();

    diagnostic.schema_version().hash(&mut hasher);
    diagnostic.code().hash(&mut hasher);
    diagnostic.severity().hash(&mut hasher);
    diagnostic.category().hash(&mut hasher);
    diagnostic.phase().hash(&mut hasher);
    diagnostic.message().hash(&mut hasher);
    diagnostic.primary_span().hash(&mut hasher);

    diagnostic.origin().component().hash(&mut hasher);
    diagnostic.origin().operation().hash(&mut hasher);
    diagnostic.origin().subject().hash(&mut hasher);
    diagnostic.origin().provider().hash(&mut hasher);

    for label in diagnostic.labels() {
        label.span().hash(&mut hasher);
        label.message().hash(&mut hasher);
    }

    for note in diagnostic.notes() {
        note.message().hash(&mut hasher);
    }

    diagnostic.help().hash(&mut hasher);

    for fix in diagnostic.fixes() {
        fix.title().hash(&mut hasher);
        fix.is_safe().hash(&mut hasher);

        for edit in fix.edits() {
            edit.span().hash(&mut hasher);
            edit.replacement().hash(&mut hasher);
        }
    }

    for (key, value) in diagnostic.metadata() {
        key.hash(&mut hasher);
        hash_value(value, &mut hasher);
    }

    hasher.finish()
}

fn hash_value<H: Hasher>(value: &DiagnosticValue, state: &mut H) {
    match value {
        DiagnosticValue::Null => {
            0_u8.hash(state);
        }

        DiagnosticValue::Boolean(value) => {
            1_u8.hash(state);
            value.hash(state);
        }

        DiagnosticValue::Signed(value) => {
            2_u8.hash(state);
            value.hash(state);
        }

        DiagnosticValue::Unsigned(value) => {
            3_u8.hash(state);
            value.hash(state);
        }

        DiagnosticValue::Float(value) => {
            4_u8.hash(state);

            // DiagnosticValue uses PartialEq. Positive and negative zero
            // compare equal, so they must produce the same hash.
            let bits = if *value == 0.0 {
                0_u64
            } else {
                value.to_bits()
            };

            bits.hash(state);
        }

        DiagnosticValue::Text(value) => {
            5_u8.hash(state);
            value.hash(state);
        }

        DiagnosticValue::Sequence(values) => {
            6_u8.hash(state);
            values.len().hash(state);

            for item in values {
                hash_value(item, state);
            }
        }

        DiagnosticValue::Object(values) => {
            7_u8.hash(state);
            values.len().hash(state);

            for (key, item) in values {
                key.hash(state);
                hash_value(item, state);
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceId,
        SourceOffset,
        Span,
    };

    use super::super::diagnostic::{
        DiagnosticCode,
        DiagnosticOrigin,
    };

    fn origin() -> DiagnosticOrigin {
        DiagnosticOrigin::new("toolchain.build")
            .with_operation("build.execute")
            .with_subject("project:example")
    }

    fn span(start: u64, end: u64) -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must be valid")
    }

    fn diagnostic(
        code: &str,
        severity: DiagnosticSeverity,
        message: &str,
    ) -> ToolchainDiagnostic {
        ToolchainDiagnostic::new(
            DiagnosticCode::new(code),
            severity,
            "build",
            "build_execution",
            message,
            origin(),
        )
    }

    #[test]
    fn default_policy_has_no_artificial_limit() {
        let policy = AggregationPolicy::default();

        assert_eq!(policy.max_diagnostics(), None);
        assert!(policy.deduplication_enabled());
    }

    #[test]
    fn retains_a_valid_diagnostic() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let outcome = aggregator
            .push(diagnostic(
                "toolchain.build.failed",
                DiagnosticSeverity::Error,
                "Build failed.",
            ))
            .expect("diagnostic should be accepted");

        assert_eq!(outcome, InsertOutcome::Inserted);
        assert_eq!(aggregator.len(), 1);
        assert_eq!(aggregator.status(), AggregationStatus::Complete);
    }

    #[test]
    fn removes_only_exact_duplicates() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let first = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        );

        let second = first.clone();

        assert_eq!(
            aggregator.push(first).expect("first insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(
            aggregator.push(second).expect("duplicate insertion"),
            InsertOutcome::Duplicate
        );

        let result = aggregator.finish();

        assert_eq!(result.len(), 1);
        assert_eq!(result.statistics().duplicates(), 1);
    }

    #[test]
    fn diagnostics_with_different_metadata_are_not_duplicates() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let mut first = diagnostic(
            "toolchain.provider.failed",
            DiagnosticSeverity::Error,
            "Provider failed.",
        );

        let mut second = first.clone();

        let _ = first.insert_metadata(
            "provider",
            DiagnosticValue::from("provider-a"),
        );

        let _ = second.insert_metadata(
            "provider",
            DiagnosticValue::from("provider-b"),
        );

        assert_eq!(
            aggregator.push(first).expect("first insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(
            aggregator.push(second).expect("second insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(aggregator.len(), 2);
    }

    #[test]
    fn diagnostics_with_different_labels_are_not_duplicates() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let first = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        )
        .with_label(
            DiagnosticLabel::new(span(1, 2), "First location."),
        );

        let second = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        )
        .with_label(
            DiagnosticLabel::new(span(2, 3), "Second location."),
        );

        assert_eq!(
            aggregator.push(first).expect("first insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(
            aggregator.push(second).expect("second insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(aggregator.len(), 2);
    }

    #[test]
    fn retention_budget_marks_result_incomplete() {
        let policy = AggregationPolicy::with_max_diagnostics(1);

        let mut aggregator =
            ToolchainDiagnosticAggregator::with_policy(policy);

        let first = diagnostic(
            "toolchain.build.first",
            DiagnosticSeverity::Error,
            "First failure.",
        );

        let second = diagnostic(
            "toolchain.build.second",
            DiagnosticSeverity::Error,
            "Second failure.",
        );

        assert_eq!(
            aggregator.push(first).expect("first insertion"),
            InsertOutcome::Inserted
        );

        assert_eq!(
            aggregator.push(second).expect("second insertion"),
            InsertOutcome::DroppedByPolicy
        );

        let result = aggregator.finish();

        assert_eq!(result.len(), 1);
        assert_eq!(result.status(), AggregationStatus::Incomplete);
        assert_eq!(result.statistics().dropped(), 1);
    }

    #[test]
    fn zero_retention_budget_is_explicitly_incomplete() {
        let policy = AggregationPolicy::with_max_diagnostics(0);

        let mut aggregator =
            ToolchainDiagnosticAggregator::with_policy(policy);

        let outcome = aggregator
            .push(diagnostic(
                "toolchain.build.failed",
                DiagnosticSeverity::Error,
                "Build failed.",
            ))
            .expect("budget exhaustion is an insertion outcome");

        assert_eq!(outcome, InsertOutcome::DroppedByPolicy);

        let result = aggregator.finish();

        assert!(result.is_empty());
        assert!(!result.is_complete());
    }

    #[test]
    fn invalid_diagnostic_is_rejected_and_marks_incomplete() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let invalid = ToolchainDiagnostic::new(
            "",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "Build failed.",
            origin(),
        );

        let result = aggregator.push(invalid);

        assert!(matches!(
            result,
            Err(AggregationError::InvalidDiagnostic(_))
        ));

        assert_eq!(aggregator.status(), AggregationStatus::Incomplete);
        assert_eq!(aggregator.statistics().invalid(), 1);
    }

    #[test]
    fn upstream_incompleteness_is_preserved() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        aggregator.mark_incomplete();

        let _ = aggregator
            .push(diagnostic(
                "toolchain.build.failed",
                DiagnosticSeverity::Error,
                "Build failed.",
            ))
            .expect("valid diagnostic");

        let result = aggregator.finish();

        assert_eq!(result.status(), AggregationStatus::Incomplete);
    }

    #[test]
    fn ordering_is_deterministic_and_severity_first() {
        let mut diagnostics = vec![
            diagnostic(
                "toolchain.warning",
                DiagnosticSeverity::Warning,
                "Warning.",
            ),
            diagnostic(
                "toolchain.error",
                DiagnosticSeverity::Error,
                "Error.",
            ),
            diagnostic(
                "toolchain.fatal",
                DiagnosticSeverity::Fatal,
                "Fatal.",
            ),
        ];

        sort_diagnostics(&mut diagnostics);

        assert_eq!(
            diagnostics[0].severity(),
            DiagnosticSeverity::Fatal
        );

        assert_eq!(
            diagnostics[1].severity(),
            DiagnosticSeverity::Error
        );

        assert_eq!(
            diagnostics[2].severity(),
            DiagnosticSeverity::Warning
        );
    }

    #[test]
    fn source_spans_have_deterministic_order() {
        let earlier = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        )
        .with_primary_span(span(1, 4));

        let later = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        )
        .with_primary_span(span(10, 14));

        assert_eq!(
            compare_diagnostics(&earlier, &later),
            Ordering::Less
        );
    }

    #[test]
    fn hash_collisions_do_not_remove_distinct_diagnostics() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let first = diagnostic(
            "toolchain.build.first",
            DiagnosticSeverity::Error,
            "First failure.",
        );

        let second = diagnostic(
            "toolchain.build.second",
            DiagnosticSeverity::Error,
            "Second failure.",
        );

        assert_ne!(first, second);

        let _ = aggregator.push(first).expect("first insertion");
        let _ = aggregator.push(second).expect("second insertion");

        assert_eq!(aggregator.len(), 2);
    }

    #[test]
    fn disabling_deduplication_retains_repeated_records() {
        let policy = AggregationPolicy::unlimited()
            .with_deduplication(false);

        let mut aggregator =
            ToolchainDiagnosticAggregator::with_policy(policy);

        let item = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "Build failed.",
        );

        let _ = aggregator
            .push(item.clone())
            .expect("first insertion");

        let _ = aggregator
            .push(item)
            .expect("second insertion");

        let result = aggregator.finish();

        assert_eq!(result.len(), 2);
        assert_eq!(result.statistics().duplicates(), 0);
    }

    #[test]
    fn metadata_float_zero_hashing_respects_partial_equality() {
        let mut positive = diagnostic(
            "toolchain.metadata.test",
            DiagnosticSeverity::Information,
            "Metadata test.",
        );

        let mut negative = positive.clone();

        let _ = positive.insert_metadata(
            "value",
            DiagnosticValue::Float(0.0),
        );

        let _ = negative.insert_metadata(
            "value",
            DiagnosticValue::Float(-0.0),
        );

        assert_eq!(positive, negative);

        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let _ = aggregator
            .push(positive)
            .expect("positive zero insertion");

        assert_eq!(
            aggregator
                .push(negative)
                .expect("negative zero duplicate"),
            InsertOutcome::Duplicate
        );
    }
}
