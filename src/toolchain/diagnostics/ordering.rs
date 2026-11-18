
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Diagnostic Ordering
//!
//! File: `src/toolchain/diagnostics/ordering.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provide the canonical deterministic ordering for
//! [`ToolchainDiagnostic`] records.
//!
//! This module compares structured diagnostic data only. It does not parse
//! rendered messages, inspect the filesystem, access source maps, execute
//! providers, or depend on thread completion order.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The canonical toolchain diagnostic comparator.
//! - In-place sorting of toolchain diagnostic slices.
//! - Deterministic comparison of nested diagnostic metadata.
//!
//! It does not own:
//!
//! - Diagnostic construction or validation.
//! - Diagnostic identity or deduplication.
//! - Aggregation budgets or completeness.
//! - Source-map lookup or source-coordinate conversion.
//! - Frontend/compiler diagnostic conversion.
//! - Rendering or report serialization.
//!
//! ## Ordering contract
//!
//! The order preserves the existing toolchain aggregation contract:
//!
//! 1. Higher severity first.
//! 2. Source-located diagnostics before source-less diagnostics.
//! 3. Canonical source span: source identity, start byte, end byte.
//! 4. Stable diagnostic code.
//! 5. Diagnostic category.
//! 6. Diagnostic phase.
//! 7. Message.
//! 8. Origin/provenance.
//! 9. Labels, notes, help and fixes.
//! 10. Structured metadata.
//! 11. Diagnostic schema version as a final tie-breaker.
//!
//! This is a presentation order, not execution chronology.
//! It is independent of insertion order, hash-map iteration order, worker
//! scheduling, machine architecture and target hardware.
//!
//! The grammar diagnostic specification recommends deterministic ordering
//! based on source identity/location, phase, severity and stable code. The
//! toolchain's established severity-first policy is retained as an explicit
//! toolchain presentation contract; it must not be confused with the
//! frontend's independent diagnostic ordering policy.
//!
//! ## Scalability
//!
//! Sorting is in place and does not construct a second diagnostic vector.
//! No diagnostic-count, source-count, hardware-count, domain-count or
//! machine-size ceiling is imposed here.
//!
//! Sorting requires CPU time proportional to the number of records and
//! their structured comparison costs. Actual capacity is bounded by the
//! process's address space and available resources.
//!
//! ## Floating-point metadata
//!
//! Metadata can contain `f64` values. Ordinary partial comparison is not a
//! valid sorting relation for NaN. `total_cmp` is used to establish a
//! deterministic total ordering, including signed zero and NaN bit patterns.
//!
//! ## Integration
//!
//! - `diagnostic.rs` owns `ToolchainDiagnostic` and its field accessors.
//! - `aggregation.rs` owns collection, budgets, completeness and deduplication.
//! - `mod.rs` declares and exports this module.
//! - `rendering.rs` consumes already ordered diagnostics.
//! - `reporting.rs` serializes the ordered report.
//!
//! The canonical source span is reused directly:
//! `crate::frontend::ast::node::source::span::Span`.
//!
//! ## Safety
//!
//! Safe Rust only. No `unsafe` code is used.

use core::cmp::Ordering;
use std::collections::BTreeMap;

use super::diagnostic::{
    DiagnosticFix,
    DiagnosticLabel,
    DiagnosticNote,
    DiagnosticValue,
    ToolchainDiagnostic,
};

/// Compare two toolchain diagnostics using the canonical deterministic
/// presentation order.
///
/// The comparison covers every observable field of
/// [`ToolchainDiagnostic`], including supplemental details and schema
/// version. Consequently, diagnostics that differ in structured content
/// do not inherit arbitrary ordering from their insertion or worker
/// completion order.
///
/// Higher severity sorts first. A diagnostic with a primary source span
/// sorts before a source-less diagnostic.
///
/// # Determinism
///
/// This function performs no I/O, allocation, environment inspection,
/// rendering, or provider invocation.
///
/// # Examples
///
/// ```ignore
/// let order = compare_diagnostics(&left, &right);
/// assert_eq!(order, Ordering::Equal);
/// ```
#[must_use]
pub fn compare_diagnostics(
    left: &ToolchainDiagnostic,
    right: &ToolchainDiagnostic,
) -> Ordering {
    // DiagnosticSeverity derives Ord in increasing enum declaration order:
    // Hint < Note < Information < Warning < Error < Fatal.
    // Reverse the operands so the most severe record sorts first.
    right
        .severity()
        .cmp(&left.severity())
        .then_with(|| {
            compare_optional_spans(left.primary_span(), right.primary_span())
        })
        .then_with(|| left.code().cmp(right.code()))
        .then_with(|| left.category().cmp(right.category()))
        .then_with(|| left.phase().cmp(right.phase()))
        .then_with(|| left.message().cmp(right.message()))
        .then_with(|| compare_origins(left, right))
        .then_with(|| compare_labels(left.labels(), right.labels()))
        .then_with(|| compare_notes(left.notes(), right.notes()))
        .then_with(|| left.help().cmp(&right.help()))
        .then_with(|| compare_fixes(left.fixes(), right.fixes()))
        .then_with(|| compare_metadata(left.metadata(), right.metadata()))
        .then_with(|| left.schema_version().cmp(&right.schema_version()))
}

/// Sort toolchain diagnostics in canonical deterministic presentation order.
///
/// Sorting is performed in place. No second diagnostic collection is
/// constructed, and no diagnostic is discarded or modified.
///
/// The comparator is a total deterministic ordering over the structured
/// diagnostic fields. `sort_unstable_by` is therefore sufficient: stability
/// with respect to input order is unnecessary because distinct records have
/// deterministic structured tie-breakers.
///
/// # Complexity
///
/// For `n` diagnostics, comparison sorting is `O(n log n)` in the usual
/// case. The cost of an individual comparison depends on the structured
/// fields that must be examined.
///
/// This function imposes no fixed diagnostic-count ceiling.
pub fn sort_diagnostics(diagnostics: &mut [ToolchainDiagnostic]) {
    diagnostics.sort_unstable_by(compare_diagnostics);
}

/// Compare optional primary spans.
///
/// Source-located diagnostics sort before source-less diagnostics. Two
/// source-less diagnostics compare equal at this stage and are ordered by
/// their remaining structured fields.
#[must_use]
fn compare_optional_spans(
    left: Option<crate::frontend::ast::node::source::span::Span>,
    right: Option<crate::frontend::ast::node::source::span::Span>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => compare_spans(left, right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Compare canonical source spans by stable source identity and UTF-8
/// byte offsets.
///
/// The source span's canonical representation is half-open:
/// `[start, end)`.
#[must_use]
fn compare_spans(
    left: crate::frontend::ast::node::source::span::Span,
    right: crate::frontend::ast::node::source::span::Span,
) -> Ordering {
    left.source()
        .as_raw()
        .cmp(&right.source().as_raw())
        .then_with(|| left.start_raw().cmp(&right.start_raw()))
        .then_with(|| left.end_raw().cmp(&right.end_raw()))
}

/// Compare all diagnostic-origin fields.
///
/// Origin is provenance, not diagnostic identity. It is nevertheless
/// included as a deterministic tie-breaker so records with different
/// provenance do not depend on input order.
fn compare_origins(
    left: &ToolchainDiagnostic,
    right: &ToolchainDiagnostic,
) -> Ordering {
    let left = left.origin();
    let right = right.origin();

    left.component()
        .cmp(right.component())
        .then_with(|| left.operation().cmp(&right.operation()))
        .then_with(|| left.subject().cmp(&right.subject()))
        .then_with(|| left.provider().cmp(&right.provider()))
}

/// Compare two slices lexicographically using a structured item comparator.
fn compare_slices<T>(
    left: &[T],
    right: &[T],
    mut compare_item: impl FnMut(&T, &T) -> Ordering,
) -> Ordering {
    for (left_item, right_item) in left.iter().zip(right.iter()) {
        let ordering = compare_item(left_item, right_item);

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    left.len().cmp(&right.len())
}

/// Compare secondary labels by canonical span, then label message.
fn compare_labels(
    left: &[DiagnosticLabel],
    right: &[DiagnosticLabel],
) -> Ordering {
    compare_slices(left, right, |left, right| {
        compare_spans(left.span(), right.span())
            .then_with(|| left.message().cmp(right.message()))
    })
}

/// Compare notes by their structured message content.
fn compare_notes(
    left: &[DiagnosticNote],
    right: &[DiagnosticNote],
) -> Ordering {
    compare_slices(left, right, |left, right| {
        left.message().cmp(right.message())
    })
}

/// Compare fixes by title, safety claim and ordered edits.
fn compare_fixes(
    left: &[DiagnosticFix],
    right: &[DiagnosticFix],
) -> Ordering {
    compare_slices(left, right, compare_one_fix)
}

/// Compare one fix and all of its edits.
fn compare_one_fix(
    left: &DiagnosticFix,
    right: &DiagnosticFix,
) -> Ordering {
    left.title()
        .cmp(right.title())
        .then_with(|| left.is_safe().cmp(&right.is_safe()))
        .then_with(|| {
            compare_slices(left.edits(), right.edits(), |left, right| {
                compare_spans(left.span(), right.span())
                    .then_with(|| {
                        left.replacement().cmp(right.replacement())
                    })
            })
        })
}

/// Compare ordered metadata maps without rendering their values.
///
/// `BTreeMap` supplies deterministic key iteration. Values are compared
/// recursively using their declared structured variants.
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

/// Compare typed diagnostic metadata values recursively.
///
/// Variant declaration order is the stable variant order. Floating-point
/// values use `total_cmp`, not partial comparison, so NaN never makes the
/// comparator return an inconsistent result.
fn compare_values(
    left: &DiagnosticValue,
    right: &DiagnosticValue,
) -> Ordering {
    use DiagnosticValue::{
        Boolean,
        Float,
        Null,
        Object,
        Sequence,
        Signed,
        Text,
        Unsigned,
    };

    match (left, right) {
        (Null, Null) => Ordering::Equal,
        (Null, _) => Ordering::Less,
        (_, Null) => Ordering::Greater,

        (Boolean(left), Boolean(right)) => left.cmp(right),
        (Boolean(_), _) => Ordering::Less,
        (_, Boolean(_)) => Ordering::Greater,

        (Signed(left), Signed(right)) => left.cmp(right),
        (Signed(_), _) => Ordering::Less,
        (_, Signed(_)) => Ordering::Greater,

        (Unsigned(left), Unsigned(right)) => left.cmp(right),
        (Unsigned(_), _) => Ordering::Less,
        (_, Unsigned(_)) => Ordering::Greater,

        (Float(left), Float(right)) => left.total_cmp(right),
        (Float(_), _) => Ordering::Less,
        (_, Float(_)) => Ordering::Greater,

        (Text(left), Text(right)) => left.cmp(right),
        (Text(_), _) => Ordering::Less,
        (_, Text(_)) => Ordering::Greater,

        (Sequence(left), Sequence(right)) => {
            compare_slices(left, right, compare_values)
        }
        (Sequence(_), _) => Ordering::Less,
        (_, Sequence(_)) => Ordering::Greater,

        (Object(left), Object(right)) => {
            compare_metadata(left, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceId,
        SourceOffset,
        Span,
    };
    use crate::toolchain::diagnostics::diagnostic::{
        DiagnosticCode,
        DiagnosticCategory,
        DiagnosticFix,
        DiagnosticFixEdit,
        DiagnosticLabel,
        DiagnosticNote,
        DiagnosticOrigin,
        DiagnosticPhase,
        DiagnosticSeverity,
        DiagnosticValue,
        ToolchainDiagnostic,
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
        severity: DiagnosticSeverity,
        message: &str,
    ) -> ToolchainDiagnostic {
        ToolchainDiagnostic::new(
            DiagnosticCode::new(code),
            severity,
            DiagnosticCategory::new("build"),
            DiagnosticPhase::new("build_execution"),
            message,
            DiagnosticOrigin::new("toolchain.build"),
        )
    }

    #[test]
    fn higher_severity_sorts_first() {
        let mut diagnostics = vec![
            diagnostic(
                "toolchain.warning",
                DiagnosticSeverity::Warning,
                "warning",
            ),
            diagnostic(
                "toolchain.fatal",
                DiagnosticSeverity::Fatal,
                "fatal",
            ),
            diagnostic(
                "toolchain.error",
                DiagnosticSeverity::Error,
                "error",
            ),
            diagnostic(
                "toolchain.hint",
                DiagnosticSeverity::Hint,
                "hint",
            ),
        ];

        sort_diagnostics(&mut diagnostics);

        assert_eq!(
            diagnostics
                .iter()
                .map(ToolchainDiagnostic::severity)
                .collect::<Vec<_>>(),
            vec![
                DiagnosticSeverity::Fatal,
                DiagnosticSeverity::Error,
                DiagnosticSeverity::Warning,
                DiagnosticSeverity::Hint,
            ]
        );
    }

    #[test]
    fn source_located_diagnostics_sort_before_source_less_diagnostics() {
        let located = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "located",
        )
        .with_primary_span(span(1, 0, 1));

        let source_less = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "source-less",
        );

        let mut diagnostics = vec![source_less, located];

        sort_diagnostics(&mut diagnostics);

        assert_eq!(diagnostics[0].message(), "located");
        assert_eq!(diagnostics[1].message(), "source-less");
    }

    #[test]
    fn source_identity_then_start_then_end_define_span_order() {
        let later_source = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "later source",
        )
        .with_primary_span(span(2, 0, 1));

        let later_offset = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "later offset",
        )
        .with_primary_span(span(1, 5, 6));

        let earlier_offset = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "earlier offset",
        )
        .with_primary_span(span(1, 1, 2));

        let mut diagnostics =
            vec![later_source, later_offset, earlier_offset];

        sort_diagnostics(&mut diagnostics);

        let messages = diagnostics
            .iter()
            .map(ToolchainDiagnostic::message)
            .collect::<Vec<_>>();

        assert_eq!(
            messages,
            vec!["earlier offset", "later offset", "later source"]
        );
    }

    #[test]
    fn stable_fields_break_ties_deterministically() {
        let first = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 4, 8));

        let second = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 4, 8))
        .with_note(DiagnosticNote::new("additional context"));

        let third = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 4, 8))
        .with_help("inspect the build configuration");

        let mut forward = vec![second.clone(), third.clone(), first.clone()];
        let mut reverse = vec![first, third, second];

        sort_diagnostics(&mut forward);
        sort_diagnostics(&mut reverse);

        assert_eq!(forward, reverse);
    }

    #[test]
    fn origin_is_a_deterministic_tie_breaker() {
        let first = diagnostic(
            "toolchain.provider.failed",
            DiagnosticSeverity::Error,
            "provider failed",
        )
        .with_primary_span(span(1, 2, 3));

        let second = ToolchainDiagnostic::new(
            "toolchain.provider.failed",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "provider failed",
            DiagnosticOrigin::new("toolchain.provider")
                .with_operation("provider.execute"),
        )
        .with_primary_span(span(1, 2, 3));

        let mut diagnostics = vec![second.clone(), first.clone()];
        sort_diagnostics(&mut diagnostics);

        assert_eq!(
            compare_diagnostics(&diagnostics[0], &diagnostics[1]),
            Ordering::Less
        );

        let mut reversed = vec![first, second];
        sort_diagnostics(&mut reversed);

        assert_eq!(diagnostics, reversed);
    }

    #[test]
    fn labels_and_fixes_are_structured_tie_breakers() {
        let base = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 2, 3));

        let with_label = base
            .clone()
            .with_label(DiagnosticLabel::new(
                span(1, 8, 10),
                "related declaration",
            ));

        let with_fix = base.with_fix(DiagnosticFix::new(
            "replace invalid value",
            vec![DiagnosticFixEdit::new(
                span(1, 8, 10),
                "valid",
            )],
            true,
        ));

        let mut diagnostics = vec![with_fix.clone(), with_label.clone()];
        sort_diagnostics(&mut diagnostics);

        let mut reversed = vec![with_label, with_fix];
        sort_diagnostics(&mut reversed);

        assert_eq!(diagnostics, reversed);
    }

    #[test]
    fn metadata_order_is_independent_of_insertion_order() {
        let mut first_metadata = BTreeMap::new();
        first_metadata.insert(
            "zeta".to_owned(),
            DiagnosticValue::Text("last".to_owned()),
        );
        first_metadata.insert(
            "alpha".to_owned(),
            DiagnosticValue::Signed(10),
        );

        let mut second_metadata = BTreeMap::new();
        second_metadata.insert(
            "alpha".to_owned(),
            DiagnosticValue::Signed(10),
        );
        second_metadata.insert(
            "zeta".to_owned(),
            DiagnosticValue::Text("last".to_owned()),
        );

        let first = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 2, 3))
        .with_metadata(first_metadata);

        let second = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_primary_span(span(1, 2, 3))
        .with_metadata(second_metadata);

        assert_eq!(compare_diagnostics(&first, &second), Ordering::Equal);
    }

    #[test]
    fn nested_metadata_is_compared_recursively() {
        let mut first_object = BTreeMap::new();
        first_object.insert(
            "nested".to_owned(),
            DiagnosticValue::Sequence(vec![
                DiagnosticValue::Boolean(true),
                DiagnosticValue::Float(1.0),
            ]),
        );

        let mut second_object = BTreeMap::new();
        second_object.insert(
            "nested".to_owned(),
            DiagnosticValue::Sequence(vec![
                DiagnosticValue::Boolean(true),
                DiagnosticValue::Float(2.0),
            ]),
        );

        let first = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_metadata(BTreeMap::from([(
            "context".to_owned(),
            DiagnosticValue::Object(first_object),
        )]));

        let second = diagnostic(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build failed",
        )
        .with_metadata(BTreeMap::from([(
            "context".to_owned(),
            DiagnosticValue::Object(second_object),
        )]));

        assert_eq!(compare_diagnostics(&first, &second), Ordering::Less);
    }

    #[test]
    fn floating_point_metadata_has_a_total_order() {
        let negative_zero = DiagnosticValue::Float(-0.0);
        let positive_zero = DiagnosticValue::Float(0.0);
        let not_a_number = DiagnosticValue::Float(f64::NAN);

        assert_eq!(
            compare_values(&negative_zero, &positive_zero),
            (-0.0_f64).total_cmp(&0.0_f64)
        );

        assert_eq!(
            compare_values(&not_a_number, &not_a_number),
            Ordering::Equal
        );
    }

    #[test]
    fn sorting_is_independent_of_input_permutation() {
        let diagnostics = vec![
            diagnostic(
                "toolchain.warning",
                DiagnosticSeverity::Warning,
                "warning",
            )
            .with_primary_span(span(2, 0, 1)),
            diagnostic(
                "toolchain.fatal",
                DiagnosticSeverity::Fatal,
                "fatal",
            ),
            diagnostic(
                "toolchain.error",
                DiagnosticSeverity::Error,
                "error",
            )
            .with_primary_span(span(1, 5, 9)),
            diagnostic(
                "toolchain.error",
                DiagnosticSeverity::Error,
                "another error",
            )
            .with_primary_span(span(1, 1, 2)),
        ];

        let mut first = diagnostics.clone();
        let mut second = diagnostics.into_iter().rev().collect::<Vec<_>>();

        sort_diagnostics(&mut first);
        sort_diagnostics(&mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn comparator_is_reflexive_and_antisymmetric_for_samples() {
        let diagnostics = vec![
            diagnostic(
                "A",
                DiagnosticSeverity::Error,
                "alpha",
            ),
            diagnostic(
                "B",
                DiagnosticSeverity::Warning,
                "beta",
            )
            .with_primary_span(span(1, 0, 1)),
            diagnostic(
                "A",
                DiagnosticSeverity::Fatal,
                "gamma",
            )
            .with_primary_span(span(2, 0, 1)),
        ];

        for left in &diagnostics {
            assert_eq!(compare_diagnostics(left, left), Ordering::Equal);

            for right in &diagnostics {
                assert_eq!(
                    compare_diagnostics(left, right),
                    compare_diagnostics(right, left).reverse()
                );
            }
        }
    }

    #[test]
    fn empty_input_is_valid() {
        let mut diagnostics = Vec::new();

        sort_diagnostics(&mut diagnostics);

        assert!(diagnostics.is_empty());
    }
}
