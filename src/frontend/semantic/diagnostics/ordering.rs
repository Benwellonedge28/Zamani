
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Deterministic ordering and conservative deduplication of Zamani diagnostics.
//!
//! File: `src/frontend/semantic/diagnostics/ordering.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provide one canonical ordering policy for structured semantic diagnostics.
//! Ordering must not depend on hash-map iteration, thread completion order,
//! memory addresses, wall-clock time, diagnostic rendering, or target hardware.
//!
//! # Authority and integration
//!
//! - `diagnostic.rs` owns the `Diagnostic` record and its structured accessors.
//! - `codes.rs` owns diagnostic identifiers and categories.
//! - `labels.rs` and the canonical AST source infrastructure own label/span
//!   representations.
//! - This module owns comparison, sorting, and report-identity deduplication.
//! - The semantic session owns diagnostic collection and operational budgets.
//! - The semantic pipeline owns the distinction between complete, invalid,
//!   recovered, and incomplete analysis.
//!
//! The canonical span type is `crate::frontend::ast::node::source::Span`.
//! This module does not define an alternative diagnostic or source model.
//!
//! # Ordering contract
//!
//! Diagnostics are ordered by:
//!
//! 1. source location (source ID, start byte, end byte); source-less
//!    diagnostics precede source-located diagnostics;
//! 2. phase, severity, code, category, and message;
//! 3. labels, notes, help, fixes, recovery classification, and metadata.
//!
//! The final fields are deterministic tie-breakers. They prevent differences
//! in structured diagnostic details from being ordered according to whichever
//! worker happened to finish first. Two records identical in every observable
//! field compare equal.
//!
//! Severity follows the `Ord` implementation of `DiagnosticSeverity`.
//! The ordering is stable API behavior; changing it should be treated as a
//! diagnostic compatibility change.
//!
//! # Deduplication contract
//!
//! Deduplication uses `Diagnostic::same_report_identity`. It only removes
//! repeated reports with the same code, phase, category, severity, primary
//! span, and message. Different locations or messages remain separate.
//! Labels, notes, fixes, recovery classification, and metadata do not change
//! report identity; they are supplemental details of the same report.
//!
//! Sorting before deduplication ensures that reports with the same identity
//! are adjacent, even if their supplemental details differ.
//!
//! # Scalability and resource policy
//!
//! No hardware, domain, source-size, or diagnostic-count ceiling is defined
//! here. The caller's available memory and CPU time necessarily bound actual
//! processing. The semantic session may impose explicit, configurable
//! diagnostic/work budgets; exhausting a budget must be represented as
//! incomplete analysis, not successful complete analysis.
//!
//! Sorting uses in-place unstable sorting with a total deterministic
//! comparator. It does not allocate a second diagnostic vector. Deduplication
//! compacts the sorted vector in place.
//!
//! # Safety and error handling
//!
//! This module uses safe Rust only. Comparison does not parse display strings,
//! perform I/O, mutate diagnostics, or invoke domain-specific logic.
//! Malformed or empty codes remain sortable; code validity is a separate
//! diagnostic/API-boundary concern.
//!
//! # Freeze criteria
//!
//! - All ordering keys come from structured diagnostic fields.
//! - Distinct source locations and messages are not collapsed.
//! - Supplemental fields provide deterministic tie-breaking.
//! - Sorting and deduplication are independent of insertion order.
//! - No hardware-specific limits or unsafe code are introduced.
//! - Unit and integration tests pass on the supported Rust toolchain.

use core::cmp::Ordering;
use std::collections::BTreeMap;

use super::diagnostic::{
    Diagnostic, DiagnosticValue, FixIt, FixItEdit,
};
use crate::frontend::ast::node::source::Span;

/// Compare two diagnostics using the canonical reporting order.
///
/// The comparison is total over the observable fields of `Diagnostic`.
/// Identical diagnostics compare equal. This function does not modify either
/// diagnostic.
///
/// Source-less diagnostics sort before diagnostics with a primary span.
/// Source-located diagnostics sort by stable source ID and canonical UTF-8
/// byte offsets before other fields are considered.
#[must_use]
pub fn compare_diagnostics(left: &Diagnostic, right: &Diagnostic) -> Ordering {
    compare_optional_span(left.primary_span(), right.primary_span())
        .then_with(|| left.phase().cmp(right.phase()))
        .then_with(|| left.severity().cmp(&right.severity()))
        .then_with(|| left.code().cmp(right.code()))
        .then_with(|| left.category().cmp(right.category()))
        .then_with(|| left.message().cmp(right.message()))
        .then_with(|| compare_labels(left.labels(), right.labels()))
        .then_with(|| left.notes().cmp(right.notes()))
        .then_with(|| left.help().cmp(&right.help()))
        .then_with(|| compare_fixes(left.fixes(), right.fixes()))
        .then_with(|| left.recovery().cmp(&right.recovery()))
        .then_with(|| compare_metadata(left.metadata(), right.metadata()))
}

/// Sort diagnostics in canonical deterministic reporting order.
///
/// This function sorts in place. It does not discard diagnostics or change
/// their contents. The comparator includes structured tie-breakers so
/// diagnostics with different metadata or labels do not inherit arbitrary
/// input or worker-completion ordering.
///
/// # Complexity
///
/// For `n` diagnostics, sorting is `O(n log n)` comparisons in the usual case.
/// Comparison cost also depends on the lengths of structured fields that need
/// to be compared when earlier keys are equal. No fixed diagnostic-count
/// ceiling is imposed by this function.
pub fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_unstable_by(compare_diagnostics);
}

/// Remove repeated reports with the same report identity.
///
/// Diagnostics must be sorted before deduplication. This function sorts them
/// first, then removes adjacent reports for which
/// `Diagnostic::same_report_identity` returns `true`.
///
/// Diagnostics with different codes, phases, categories, severities, primary
/// spans, or messages are preserved. The surviving diagnostic is selected
/// deterministically by the supplemental tie-breakers in
/// [`compare_diagnostics`].
///
/// # Complexity
///
/// Sorting is `O(n log n)` in the usual case. The compaction pass is linear
/// after sorting. The vector is compacted in place; no second vector of
/// diagnostics is constructed.
pub fn deduplicate_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    sort_diagnostics(diagnostics);
    diagnostics.dedup_by(|left, right| left.same_report_identity(right));
}

/// Compare optional spans by source identity and half-open byte coordinates.
///
/// `None` sorts before `Some`. The span type guarantees `start <= end` for
/// spans created through its validated constructors.
#[must_use]
fn compare_optional_span(left: Option<Span>, right: Option<Span>) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => compare_span(left, right),
    }
}

/// Compare canonical spans without converting coordinates to `usize`.
#[must_use]
fn compare_span(left: Span, right: Span) -> Ordering {
    left.source()
        .as_raw()
        .cmp(&right.source().as_raw())
        .then_with(|| left.start_raw().cmp(&right.start_raw()))
        .then_with(|| left.end_raw().cmp(&right.end_raw()))
}

/// Compare slices lexicographically with an explicit structured comparator.
fn compare_slices<T>(
    left: &[T],
    right: &[T],
    mut compare_item: impl FnMut(&T, &T) -> Ordering,
) -> Ordering {
    for (left_item, right_item) in left.iter().zip(right.iter()) {
        let order = compare_item(left_item, right_item);
        if order != Ordering::Equal {
            return order;
        }
    }

    left.len().cmp(&right.len())
}

/// Compare secondary labels by source span, then label text.
fn compare_labels(
    left: &[super::diagnostic::DiagnosticLabel],
    right: &[super::diagnostic::DiagnosticLabel],
) -> Ordering {
    compare_slices(left, right, |left, right| {
        compare_span(left.span(), right.span())
            .then_with(|| left.message().cmp(right.message()))
    })
}

/// Compare fixes by title, safety claim, and their ordered edits.
fn compare_fixes(left: &[FixIt], right: &[FixIt]) -> Ordering {
    compare_slices(left, right, compare_fix)
}

fn compare_fix(left: &FixIt, right: &FixIt) -> Ordering {
    left.title()
        .cmp(right.title())
        .then_with(|| left.is_safe().cmp(&right.is_safe()))
        .then_with(|| compare_edits(left.edits(), right.edits()))
}

fn compare_edits(left: &[FixItEdit], right: &[FixItEdit]) -> Ordering {
    compare_slices(left, right, |left, right| {
        compare_span(left.span(), right.span())
            .then_with(|| left.replacement().cmp(right.replacement()))
    })
}

/// Compare ordered metadata maps without rendering values as text.
fn compare_metadata(
    left: &BTreeMap<String, DiagnosticValue>,
    right: &BTreeMap<String, DiagnosticValue>,
) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    loop {
        match (left_iter.next(), right_iter.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some((left_key, left_value)), Some((right_key, right_value))) => {
                let order = left_key
                    .cmp(right_key)
                    .then_with(|| compare_values(left_value, right_value));

                if order != Ordering::Equal {
                    return order;
                }
            }
        }
    }
}

/// Compare typed metadata values deterministically and recursively.
///
/// Variant order is the declared order of `DiagnosticValue`. Sequence values
/// compare lexicographically. Object values compare by their ordered keys and
/// corresponding values.
fn compare_values(left: &DiagnosticValue, right: &DiagnosticValue) -> Ordering {
    use DiagnosticValue::{
        Boolean, Integer, Null, Object, Sequence, Text, Unsigned,
    };

    match (left, right) {
        (Null, Null) => Ordering::Equal,
        (Null, _) => Ordering::Less,
        (_, Null) => Ordering::Greater,

        (Boolean(left), Boolean(right)) => left.cmp(right),
        (Boolean(_), _) => Ordering::Less,
        (_, Boolean(_)) => Ordering::Greater,

        (Integer(left), Integer(right)) => left.cmp(right),
        (Integer(_), _) => Ordering::Less,
        (_, Integer(_)) => Ordering::Greater,

        (Unsigned(left), Unsigned(right)) => left.cmp(right),
        (Unsigned(_), _) => Ordering::Less,
        (_, Unsigned(_)) => Ordering::Greater,

        (Text(left), Text(right)) => left.cmp(right),
        (Text(_), _) => Ordering::Less,
        (_, Text(_)) => Ordering::Greater,

        (Sequence(left), Sequence(right)) => {
            compare_slices(left, right, compare_values)
        }
        (Sequence(_), _) => Ordering::Less,
        (_, Sequence(_)) => Ordering::Greater,

        (Object(left), Object(right)) => compare_metadata(left, right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::source_id::SourceId;
    use crate::frontend::ast::node::source::span::SourceOffset;
    use crate::frontend::semantic::diagnostics::diagnostic::{
        DiagnosticCategory, DiagnosticPhase, DiagnosticSeverity,
    };

    fn span(source: u64, start: u64, end: u64) -> Span {
        Span::new(
            SourceId::from_raw(source),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must be valid")
    }

    fn diagnostic(
        code: &str,
        message: &str,
        location: Option<Span>,
    ) -> Diagnostic {
        Diagnostic::new(
            code,
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            message,
            location,
        )
    }

    #[test]
    fn source_less_diagnostics_sort_before_located_diagnostics() {
        let mut diagnostics = vec![
            diagnostic("ZM-SEM-TYPE-002", "located", Some(span(1, 0, 1))),
            diagnostic("ZM-SEM-TYPE-001", "source-less", None),
        ];

        sort_diagnostics(&mut diagnostics);

        assert_eq!(diagnostics[0].message(), "source-less");
        assert_eq!(diagnostics[1].message(), "located");
    }

    #[test]
    fn source_id_then_start_then_end_define_location_order() {
        let mut diagnostics = vec![
            diagnostic("C", "later source", Some(span(2, 0, 1))),
            diagnostic("B", "later offset", Some(span(1, 5, 6))),
            diagnostic("A", "earlier offset", Some(span(1, 1, 2))),
        ];

        sort_diagnostics(&mut diagnostics);

        let messages: Vec<&str> =
            diagnostics.iter().map(Diagnostic::message).collect();

        assert_eq!(
            messages,
            vec!["earlier offset", "later offset", "later source"]
        );
    }

    #[test]
    fn identical_headline_keys_use_structured_tie_breakers() {
        let base = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 4, 8)),
        );

        let with_note = base.clone().with_note("additional context");
        let with_help = base.clone().with_help("consider an explicit conversion");

        let mut first = vec![with_note.clone(), with_help.clone()];
        let mut second = vec![with_help, with_note];

        sort_diagnostics(&mut first);
        sort_diagnostics(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn metadata_is_a_deterministic_tie_breaker() {
        let low = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 4, 8)),
        )
        .with_metadata("constraint", "alpha");

        let high = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 4, 8)),
        )
        .with_metadata("constraint", "beta");

        let mut first = vec![high.clone(), low.clone()];
        let mut second = vec![low, high];

        sort_diagnostics(&mut first);
        sort_diagnostics(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn deduplication_preserves_distinct_locations_and_messages() {
        let same = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 4, 8)),
        );

        let different_location = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 9, 10)),
        );

        let different_message = diagnostic(
            "ZM-SEM-TYPE-001",
            "different mismatch",
            Some(span(1, 4, 8)),
        );

        let mut diagnostics = vec![
            same.clone(),
            different_location,
            same,
            different_message,
        ];

        deduplicate_diagnostics(&mut diagnostics);

        assert_eq!(diagnostics.len(), 3);
        assert!(diagnostics.windows(2).all(|pair| {
            !pair[0].same_report_identity(&pair[1])
        }));
    }

    #[test]
    fn deduplication_is_independent_of_input_order() {
        let first = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 2, 3)),
        )
        .with_note("first supplemental note");

        let same_identity = diagnostic(
            "ZM-SEM-TYPE-001",
            "mismatch",
            Some(span(1, 2, 3)),
        )
        .with_note("second supplemental note");

        let other = diagnostic(
            "ZM-SEM-NAME-001",
            "unresolved name",
            Some(span(1, 8, 9)),
        );

        let mut left = vec![first.clone(), other.clone(), same_identity.clone()];
        let mut right = vec![same_identity, first, other];

        deduplicate_diagnostics(&mut left);
        deduplicate_diagnostics(&mut right);

        assert_eq!(left, right);
    }

    #[test]
    fn empty_input_is_valid() {
        let mut diagnostics = Vec::new();

        sort_diagnostics(&mut diagnostics);
        deduplicate_diagnostics(&mut diagnostics);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn comparator_is_reflexive_antisymmetric_and_transitive_for_samples() {
        let diagnostics = vec![
            diagnostic("A", "alpha", None),
            diagnostic("B", "beta", Some(span(1, 0, 0))),
            diagnostic("A", "alpha", Some(span(1, 0, 1))),
            diagnostic("C", "gamma", Some(span(2, 0, 1))),
        ];

        for left in &diagnostics {
            assert_eq!(compare_diagnostics(left, left), Ordering::Equal);

            for right in &diagnostics {
                assert_eq!(
                    compare_diagnostics(left, right),
                    compare_diagnostics(right, left).reverse()
                );

                for third in &diagnostics {
                    if compare_diagnostics(left, right) != Ordering::Greater
                        && compare_diagnostics(right, third) != Ordering::Greater
                    {
                        assert_ne!(
                            compare_diagnostics(left, third),
                            Ordering::Greater
                        );
                    }
                }
            }
        }
    }
}
