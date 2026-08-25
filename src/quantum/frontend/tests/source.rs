//! Production contract tests for `quantum::frontend::core::source`.
//!
//! This test module intentionally tests the source-location API from outside
//! `core::source`'s implementation module. The purpose is to freeze the
//! public contract that the lexer, parser, AST, validator, diagnostics,
//! lowering, importers, exporters, IDE tooling, and future frontend formats
//! must consume.
//!
//! # Integration contract
//!
//! This file is registered by `src/quantum/frontend/mod.rs` with:
//!
//! ```rust
//! #[cfg(test)]
//! #[path = "tests/source.rs"]
//! mod source_contract_tests;
//! ```
//!
//! No implementation details from `core/source.rs` are accessed here.
//! Consequently, these tests remain valid if the internal source-map
//! implementation changes while the public contract remains stable.
//!
//! # Frozen coordinate contract
//!
//! - Source IDs are stable within a `SourceMap`.
//! - Byte offsets are zero-based.
//! - Spans are half-open: `[start, end)`.
//! - Lines are one-based.
//! - Columns are one-based Unicode-scalar columns.
//! - UTF-8 byte offsets must lie on UTF-8 character boundaries.
//! - EOF is a valid position.
//! - Empty input has an EOF position of line 1, column 1.
//! - LF, CRLF, and standalone CR are line terminators.
//! - CRLF is one logical line terminator, not two.
//! - A trailing line terminator creates a final empty logical line.
//! - A span is always associated with exactly one source.
//! - A span from another source must never resolve against the current source.
//! - Source-map lookup is deterministic and side-effect free.
//!
//! # Security contract
//!
//! These tests also freeze the source-location layer's security properties:
//!
//! - malformed ranges are rejected;
//! - invalid UTF-8 boundaries are rejected;
//! - foreign source spans cannot expose another source;
//! - unknown source IDs return structured lookup errors;
//! - arithmetic helpers do not wrap;
//! - source locations never require filesystem or network access.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97 / Rust 1.97.1.
//! Stable Rust only. No external dependencies.

use crate::quantum::frontend::core::source::{
    ColumnNumber,
    LineColumn,
    LineNumber,
    SourceFile,
    SourceFileError,
    SourceId,
    SourceLookupError,
    SourceMap,
    SourceOffset,
    SourcePosition,
    SourceSpan,
    SourceSpanError,
};

fn make_source(id: u32, name: &str, text: &str) -> SourceFile {
    SourceFile::new(SourceId::from_raw(id), name, text)
        .expect("test source must satisfy the source contract")
}

fn offset(value: usize) -> SourceOffset {
    SourceOffset::try_from_usize(value)
        .expect("test offset must fit the frontend coordinate model")
}

fn span(id: u32, start: usize, end: usize) -> SourceSpan {
    SourceSpan::from_usize(SourceId::from_raw(id), start, end)
        .expect("test span must satisfy the half-open range contract")
}

// ============================================================================
// SourceId
// ============================================================================

#[test]
fn source_id_is_losslessly_represented() {
    for raw in [0_u32, 1, 7, 42, u32::MAX] {
        let id = SourceId::from_raw(raw);

        assert_eq!(id.as_raw(), raw);
        assert_eq!(SourceId::from_raw(id.as_raw()), id);
    }
}

#[test]
fn source_id_display_is_deterministic() {
    assert_eq!(
        SourceId::from_raw(0).to_string(),
        "source#0"
    );
    assert_eq!(
        SourceId::from_raw(42).to_string(),
        "source#42"
    );
    assert_eq!(
        SourceId::from_raw(u32::MAX).to_string(),
        "source#4294967295"
    );
}

#[test]
fn source_ids_are_orderable_and_hashable() {
    let first = SourceId::from_raw(1);
    let second = SourceId::from_raw(2);

    assert!(first < second);
    assert!(second > first);
    assert_eq!(first, SourceId::from_raw(1));
    assert_ne!(first, second);

    let mut ids = [second, first];
    ids.sort();

    assert_eq!(ids, [first, second]);
}

// ============================================================================
// SourceOffset
// ============================================================================

#[test]
fn source_offset_round_trips() {
    for raw in [0_u32, 1, 10, 1024, u32::MAX] {
        let value = SourceOffset::from_raw(raw);

        assert_eq!(value.as_raw(), raw);
        assert_eq!(value.as_usize(), raw as usize);
    }
}

#[test]
fn source_offset_zero_is_detected() {
    assert!(SourceOffset::from_raw(0).is_zero());
    assert!(!SourceOffset::from_raw(1).is_zero());
}

#[test]
fn source_offset_checked_addition_is_non_wrapping() {
    assert_eq!(
        SourceOffset::from_raw(10).checked_add(5),
        Some(SourceOffset::from_raw(15))
    );

    assert_eq!(
        SourceOffset::from_raw(u32::MAX).checked_add(1),
        None
    );

    assert_eq!(
        SourceOffset::from_raw(u32::MAX - 1).checked_add(1),
        Some(SourceOffset::from_raw(u32::MAX))
    );
}

#[test]
fn source_offset_checked_distance_is_directional() {
    let start = SourceOffset::from_raw(5);
    let end = SourceOffset::from_raw(12);

    assert_eq!(start.checked_distance(end), Some(7));
    assert_eq!(end.checked_distance(start), None);
    assert_eq!(start.checked_distance(start), Some(0));
}

#[test]
fn source_offset_from_usize_is_checked() {
    assert_eq!(
        SourceOffset::try_from_usize(0),
        Ok(SourceOffset::from_raw(0))
    );

    assert_eq!(
        SourceOffset::try_from_usize(42),
        Ok(SourceOffset::from_raw(42))
    );

    // This value is not representable by the compact u32 coordinate model.
    // On 32-bit platforms it is intentionally not expressible as usize, so
    // this assertion is guarded below by the native usize width.
    #[cfg(target_pointer_width = "64")]
    assert_eq!(
        SourceOffset::try_from_usize((u32::MAX as usize) + 1),
        Err(SourceSpanError::OffsetOverflow)
    );
}

// ============================================================================
// LineNumber / ColumnNumber / LineColumn
// ============================================================================

#[test]
fn_line_and_column_numbers_are_one_based() {
    assert_eq!(LineNumber::FIRST.as_raw(), 1);
    assert_eq!(ColumnNumber::FIRST.as_raw(), 1);

    assert_eq!(
        LineNumber::FIRST.checked_next(),
        Some(LineNumber::from_raw(2))
    );

    assert_eq!(
        ColumnNumber::FIRST.checked_next(),
        Some(ColumnNumber::from_raw(2))
    );
}

#[test]
fn line_and_column_checked_increment_does_not_wrap() {
    assert_eq!(
        LineNumber::from_raw(u32::MAX).checked_next(),
        None
    );

    assert_eq!(
        ColumnNumber::from_raw(u32::MAX).checked_next(),
        None
    );
}

#[test]
fn line_column_is_deterministic_and_displayable() {
    let location = LineColumn::new(
        LineNumber::from_raw(17),
        ColumnNumber::from_raw(9),
    );

    assert_eq!(location.line().as_raw(), 17);
    assert_eq!(location.column().as_raw(), 9);
    assert_eq!(location.to_string(), "17:9");
}

// ============================================================================
// SourceSpan construction and algebra
// ============================================================================

#[test]
fn source_span_is_half_open() {
    let value = span(1, 2, 5);

    assert_eq!(value.start().as_raw(), 2);
    assert_eq!(value.end().as_raw(), 5);
    assert_eq!(value.len_bytes(), 3);

    assert!(value.contains(SourceOffset::from_raw(2)));
    assert!(value.contains(SourceOffset::from_raw(3)));
    assert!(value.contains(SourceOffset::from_raw(4)));

    assert!(!value.contains(SourceOffset::from_raw(1)));
    assert!(!value.contains(SourceOffset::from_raw(5)));
}

#[test]
fn empty_span_is_valid() {
    let point = SourceSpan::point(
        SourceId::from_raw(3),
        SourceOffset::from_raw(11),
    );

    assert!(point.is_empty());
    assert_eq!(point.len_bytes(), 0);
    assert_eq!(point.start(), point.end());

    // Half-open semantics mean that a zero-width span contains no byte.
    assert!(!point.contains(SourceOffset::from_raw(11)));
}

#[test]
fn source_span_rejects_reversed_ranges() {
    assert_eq!(
        SourceSpan::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(9),
            SourceOffset::from_raw(3),
        ),
        Err(SourceSpanError::ReversedRange)
    );
}

#[test]
fn source_span_accepts_equal_start_and_end() {
    let point = SourceSpan::new(
        SourceId::from_raw(1),
        SourceOffset::from_raw(9),
        SourceOffset::from_raw(9),
    )
    .expect("zero-width spans are valid");

    assert!(point.is_empty());
}

#[test]
fn source_span_platform_range_conversion_is_lossless() {
    let value = SourceSpan::from_usize(
        SourceId::from_raw(7),
        12,
        27,
    )
    .expect("valid range");

    assert_eq!(value.as_range_usize(), 12..27);
}

#[test]
fn source_span_contains_span_is_source_scoped() {
    let outer = span(1, 2, 20);
    let inner = span(1, 5, 10);
    let foreign = span(2, 5, 10);

    assert!(outer.contains_span(inner));
    assert!(!outer.contains_span(foreign));
}

#[test]
fn source_span_contains_span_accepts_equal_spans() {
    let value = span(1, 5, 10);

    assert!(value.contains_span(value));
}

#[test]
fn source_span_contains_span_respects_half_open_end() {
    let outer = span(1, 5, 10);

    assert!(outer.contains_span(span(1, 5, 10)));
    assert!(!outer.contains_span(span(1, 5, 11)));
    assert!(!outer.contains_span(span(1, 10, 11)));
}

#[test]
fn source_span_overlap_is_source_scoped() {
    let first = span(1, 2, 8);
    let overlapping = span(1, 6, 12);
    let adjacent = span(1, 8, 14);
    let foreign = span(2, 6, 12);

    assert!(first.overlaps(overlapping));
    assert!(!first.overlaps(adjacent));
    assert!(!first.overlaps(foreign));
}

#[test]
fn source_span_overlap_is_symmetric() {
    let first = span(1, 2, 8);
    let second = span(1, 6, 12);

    assert_eq!(
        first.overlaps(second),
        second.overlaps(first)
    );
}

#[test]
fn empty_spans_do_not_report_overlap() {
    let empty = SourceSpan::point(
        SourceId::from_raw(1),
        SourceOffset::from_raw(5),
    );
    let value = span(1, 2, 8);

    assert!(!empty.overlaps(value));
    assert!(!value.overlaps(empty));
}

#[test]
fn source_span_union_is_source_scoped() {
    let first = span(1, 10, 20);
    let second = span(1, 30, 40);

    let result = first
        .union(second)
        .expect("same-source spans can be united");

    assert_eq!(result.source_id(), SourceId::from_raw(1));
    assert_eq!(result.start().as_raw(), 10);
    assert_eq!(result.end().as_raw(), 40);

    let foreign = span(2, 30, 40);

    assert!(first.union(foreign).is_none());
}

#[test]
fn source_span_union_is_commutative_for_bounds() {
    let first = span(1, 30, 40);
    let second = span(1, 10, 20);

    let left = first.union(second).unwrap();
    let right = second.union(first).unwrap();

    assert_eq!(left, right);
    assert_eq!(left.start().as_raw(), 10);
    assert_eq!(left.end().as_raw(), 40);
}

#[test]
fn source_span_union_handles_nested_spans() {
    let outer = span(1, 2, 20);
    let inner = span(1, 7, 11);

    assert_eq!(outer.union(inner), Some(outer));
    assert_eq!(inner.union(outer), Some(outer));
}

#[test]
fn source_span_display_is_stable() {
    let value = span(7, 12, 31);

    assert_eq!(
        value.to_string(),
        "source#7:12..31"
    );
}

// ============================================================================
// SourceFile construction and identity
// ============================================================================

#[test]
fn source_file_preserves_identity_name_and_text() {
    let source = make_source(
        7,
        "program.qasm",
        "OPENQASM 3.1;",
    );

    assert_eq!(source.id(), SourceId::from_raw(7));
    assert_eq!(source.name(), "program.qasm");
    assert_eq!(source.text(), "OPENQASM 3.1;");
    assert_eq!(source.len_bytes(), 13);
    assert!(!source.is_empty());
}

#[test]
fn empty_source_is_supported() {
    let source = make_source(0, "empty.qasm", "");

    assert!(source.is_empty());
    assert_eq!(source.len_bytes(), 0);
    assert_eq!(source.line_count(), 0);
    assert_eq!(source.text(), "");
}

#[test]
fn source_line_count_is_zero_only_for_empty_source() {
    let empty = make_source(0, "empty", "");
    let non_empty = make_source(1, "non-empty", "x");

    assert_eq!(empty.line_count(), 0);
    assert_eq!(non_empty.line_count(), 1);
}

#[test]
fn source_line_starts_are_exposed_in_stable_order() {
    let source = make_source(
        0,
        "test",
        "one\ntwo\nthree",
    );

    assert_eq!(
        source.line_starts(),
        &[0, 4, 8]
    );
}

#[test]
fn source_line_start_rejects_zero_line_number() {
    let source = make_source(0, "test", "one");

    assert_eq!(
        source.line_start(LineNumber::from_raw(0)),
        None
    );
}

#[test]
fn source_line_start_rejects_line_after_end() {
    let source = make_source(0, "test", "one");

    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        None
    );
}

// ============================================================================
// Line indexing
// ============================================================================

#[test]
fn line_index_handles_lf() {
    let source = make_source(
        0,
        "lf",
        "one\ntwo\nthree",
    );

    assert_eq!(source.line_count(), 3);

    assert_eq!(
        source.line_start(LineNumber::from_raw(1)),
        Some(offset(0))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(4))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(3)),
        Some(offset(8))
    );
}

#[test]
fn line_index_handles_crlf_as_one_terminator() {
    let source = make_source(
        0,
        "crlf",
        "one\r\ntwo\r\nthree",
    );

    assert_eq!(source.line_count(), 3);

    assert_eq!(
        source.line_start(LineNumber::from_raw(1)),
        Some(offset(0))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(5))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(3)),
        Some(offset(10))
    );
}

#[test]
fn line_index_handles_standalone_cr() {
    let source = make_source(
        0,
        "cr",
        "one\rtwo\rthree",
    );

    assert_eq!(source.line_count(), 3);

    assert_eq!(
        source.line_start(LineNumber::from_raw(1)),
        Some(offset(0))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(4))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(3)),
        Some(offset(8))
    );
}

#[test]
fn mixed_line_terminators_are_supported() {
    let source = make_source(
        0,
        "mixed",
        "a\nb\r\nc\rd",
    );

    assert_eq!(source.line_count(), 4);

    assert_eq!(
        source.line_start(LineNumber::from_raw(1)),
        Some(offset(0))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(2))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(3)),
        Some(offset(5))
    );
    assert_eq!(
        source.line_start(LineNumber::from_raw(4)),
        Some(offset(7))
    );
}

#[test]
fn trailing_lf_creates_final_empty_line() {
    let source = make_source(
        0,
        "trailing-lf",
        "one\n",
    );

    assert_eq!(source.line_count(), 2);
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(4))
    );
}

#[test]
fn trailing_crlf_creates_final_empty_line() {
    let source = make_source(
        0,
        "trailing-crlf",
        "one\r\n",
    );

    assert_eq!(source.line_count(), 2);
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(5))
    );
}

#[test]
fn trailing_cr_creates_final_empty_line() {
    let source = make_source(
        0,
        "trailing-cr",
        "one\r",
    );

    assert_eq!(source.line_count(), 2);
    assert_eq!(
        source.line_start(LineNumber::from_raw(2)),
        Some(offset(4))
    );
}

// ============================================================================
// Source positions
// ============================================================================

#[test]
fn first_position_is_line_one_column_one() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let position = source
        .position_at(offset(0))
        .expect("offset zero is valid");

    assert_eq!(position.source_id(), SourceId::from_raw(0));
    assert_eq!(position.offset(), offset(0));
    assert_eq!(position.line(), LineNumber::from_raw(1));
    assert_eq!(position.column(), ColumnNumber::from_raw(1));
}

#[test]
fn positions_are_one_based_for_ascii() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let position = source
        .position_at(offset(2))
        .unwrap();

    assert_eq!(position.line().as_raw(), 1);
    assert_eq!(position.column().as_raw(), 3);
}

#[test]
fn position_after_lf_starts_new_line() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let position = source
        .position_at(offset(4))
        .unwrap();

    assert_eq!(position.line().as_raw(), 2);
    assert_eq!(position.column().as_raw(), 1);
}

#[test]
fn position_after_crlf_starts_new_line() {
    let source = make_source(
        0,
        "test",
        "abc\r\ndef",
    );

    let position = source
        .position_at(offset(5))
        .unwrap();

    assert_eq!(position.line().as_raw(), 2);
    assert_eq!(position.column().as_raw(), 1);
}

#[test]
fn position_after_standalone_cr_starts_new_line() {
    let source = make_source(
        0,
        "test",
        "abc\rdef",
    );

    let position = source
        .position_at(offset(4))
        .unwrap();

    assert_eq!(position.line().as_raw(), 2);
    assert_eq!(position.column().as_raw(), 1);
}

#[test]
fn eof_is_a_valid_position() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let eof = offset(source.text().len());

    let position = source
        .position_at(eof)
        .expect("EOF must be a valid position");

    assert_eq!(position.offset(), eof);
    assert_eq!(position.line().as_raw(), 2);
    assert_eq!(position.column().as_raw(), 4);
}

#[test]
fn eof_after_trailing_newline_is_final_empty_line() {
    let source = make_source(
        0,
        "test",
        "abc\n",
    );

    let eof = offset(source.text().len());

    let position = source
        .position_at(eof)
        .expect("EOF must be valid");

    assert_eq!(position.line().as_raw(), 2);
    assert_eq!(position.column().as_raw(), 1);
}

#[test]
fn empty_source_eof_is_line_one_column_one() {
    let source = make_source(
        0,
        "empty",
        "",
    );

    let position = source
        .position_at(offset(0))
        .expect("empty-source EOF must be valid");

    assert_eq!(position.offset(), offset(0));
    assert_eq!(position.line(), LineNumber::FIRST);
    assert_eq!(position.column(), ColumnNumber::FIRST);
}

#[test]
fn out_of_bounds_positions_are_rejected() {
    let source = make_source(
        0,
        "test",
        "abc",
    );

    assert!(
        source
            .position_at(offset(4))
            .is_none()
    );
}

#[test]
fn position_display_is_stable() {
    let position = SourcePosition::new(
        SourceId::from_raw(3),
        SourceOffset::from_raw(12),
        LineNumber::from_raw(4),
        ColumnNumber::from_raw(7),
    );

    assert_eq!(
        position.to_string(),
        "source#3:4:7"
    );
}

#[test]
fn line_column_is_recoverable_from_position() {
    let source = make_source(
        9,
        "test",
        "abc\ndef",
    );

    let position = source
        .position_at(offset(4))
        .unwrap();

    assert_eq!(
        position.line_column(),
        LineColumn::new(
            LineNumber::from_raw(2),
            ColumnNumber::from_raw(1),
        )
    );
}

// ============================================================================
// Unicode / UTF-8
// ============================================================================

#[test]
fn unicode_columns_count_scalars_not_bytes() {
    let source = make_source(
        0,
        "unicode",
        "aé中x",
    );

    // "aé中" occupies five UTF-8 bytes but three Unicode scalar values.
    let position = source
        .position_at(offset("aé中".len()))
        .unwrap();

    assert_eq!(position.line().as_raw(), 1);
    assert_eq!(position.column().as_raw(), 4);
}

#[test]
fn unicode_scalar_boundaries_are_valid() {
    let source = make_source(
        0,
        "unicode",
        "aé中x",
    );

    for value in [
        0,
        "a".len(),
        "aé".len(),
        "aé中".len(),
        "aé中x".len(),
    ] {
        assert!(
            source.position_at(offset(value)).is_some(),
            "expected UTF-8 boundary at byte offset {value}"
        );
    }
}

#[test]
fn offsets_inside_multibyte_utf8_scalars_are_rejected() {
    let source = make_source(
        0,
        "unicode",
        "éx",
    );

    // 'é' occupies bytes [0, 2), so byte 1 is not a character boundary.
    assert!(
        source
            .position_at(offset(1))
            .is_none()
    );
}

#[test]
fn unicode_slices_are_exact() {
    let source = make_source(
        0,
        "unicode",
        "aé中x",
    );

    let start = "a".len();
    let end = "aé中".len();

    let value = source
        .slice(span(0, start, end))
        .expect("valid Unicode boundaries");

    assert_eq!(value, "é中");
}

#[test]
fn spans_with_invalid_utf8_boundaries_are_not_sliced() {
    let source = make_source(
        0,
        "unicode",
        "éx",
    );

    let invalid = SourceSpan::new(
        SourceId::from_raw(0),
        SourceOffset::from_raw(1),
        SourceOffset::from_raw(2),
    )
    .unwrap();

    assert!(source.slice(invalid).is_none());
}

// ============================================================================
// Source spans and slicing
// ============================================================================

#[test]
fn source_slice_returns_exact_text() {
    let source = make_source(
        0,
        "program.qasm",
        "OPENQASM 3.1;",
    );

    let value = source
        .slice(span(0, 0, "OPENQASM".len()))
        .unwrap();

    assert_eq!(value, "OPENQASM");
}

#[test]
fn source_slice_accepts_empty_span_at_valid_boundary() {
    let source = make_source(
        0,
        "test",
        "abc",
    );

    let point = SourceSpan::point(
        SourceId::from_raw(0),
        offset(1),
    );

    assert_eq!(source.slice(point), Some(""));
}

#[test]
fn source_slice_accepts_eof_point() {
    let source = make_source(
        0,
        "test",
        "abc",
    );

    let point = SourceSpan::point(
        SourceId::from_raw(0),
        offset(3),
    );

    assert_eq!(source.slice(point), Some(""));
}

#[test]
fn source_slice_rejects_foreign_source_span() {
    let source = make_source(
        0,
        "test",
        "abc",
    );

    let foreign = span(1, 0, 1);

    assert!(source.slice(foreign).is_none());
}

#[test]
fn source_entire_span_covers_complete_text() {
    let source = make_source(
        17,
        "program.qasm",
        "OPENQASM 3.1;\nqubit[2] q;",
    );

    let entire = SourceSpan::entire(&source);

    assert_eq!(
        entire.source_id(),
        SourceId::from_raw(17)
    );
    assert_eq!(entire.start(), offset(0));
    assert_eq!(
        entire.end(),
        offset(source.text().len())
    );
    assert_eq!(
        source.slice(entire),
        Some(source.text())
    );
}

#[test]
fn start_and_end_positions_are_source_scoped() {
    let source = make_source(
        4,
        "test",
        "abc\ndef",
    );

    let value = span(4, 1, 5);

    let start = source
        .start_position(value)
        .unwrap();

    let end = source
        .end_position(value)
        .unwrap();

    assert_eq!(start.offset(), offset(1));
    assert_eq!(start.line().as_raw(), 1);
    assert_eq!(start.column().as_raw(), 2);

    assert_eq!(end.offset(), offset(5));
    assert_eq!(end.line().as_raw(), 2);
    assert_eq!(end.column().as_raw(), 2);
}

#[test]
fn foreign_span_positions_are_rejected() {
    let source = make_source(
        4,
        "test",
        "abc",
    );

    let foreign = span(9, 0, 2);

    assert!(source.start_position(foreign).is_none());
    assert!(source.end_position(foreign).is_none());
}

// ============================================================================
// Line spans and line text
// ============================================================================

#[test]
fn line_span_includes_lf_terminator() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let line = source
        .line_span_at(offset(1))
        .unwrap();

    assert_eq!(
        source.slice(line),
        Some("abc\n")
    );
}

#[test]
fn line_span_includes_complete_crlf_terminator() {
    let source = make_source(
        0,
        "test",
        "abc\r\ndef",
    );

    let line = source
        .line_span_at(offset(1))
        .unwrap();

    assert_eq!(
        source.slice(line),
        Some("abc\r\n")
    );
}

#[test]
fn line_span_includes_standalone_cr_terminator() {
    let source = make_source(
        0,
        "test",
        "abc\rdef",
    );

    let line = source
        .line_span_at(offset(1))
        .unwrap();

    assert_eq!(
        source.slice(line),
        Some("abc\r")
    );
}

#[test]
fn final_line_without_terminator_is_returned_exactly() {
    let source = make_source(
        0,
        "test",
        "abc\ndef",
    );

    let line = source
        .line_span_at(offset(5))
        .unwrap();

    assert_eq!(
        source.slice(line),
        Some("def")
    );
}

#[test]
fn trailing_newline_line_span_is_empty() {
    let source = make_source(
        0,
        "test",
        "abc\n",
    );

    let line = source
        .line_span_at(offset(4))
        .unwrap();

    assert!(line.is_empty());
    assert_eq!(source.slice(line), Some(""));
}

#[test]
fn line_text_matches_line_span() {
    let source = make_source(
        0,
        "test",
        "first\nsecond\nthird",
    );

    for value in [
        offset(0),
        offset(2),
        offset(6),
        offset(9),
        offset(source.text().len()),
    ] {
        let span = source.line_span_at(value).unwrap();
        let text = source.line_text_at(value).unwrap();

        assert_eq!(
            text,
            source.slice(span).unwrap()
        );
    }
}

#[test]
fn line_text_at_eof_returns_final_line() {
    let source = make_source(
        0,
        "test",
        "first\nsecond",
    );

    let eof = offset(source.text().len());

    assert_eq!(
        source.line_text_at(eof),
        Some("second")
    );
}

#[test]
fn empty_source_has_empty_line_span() {
    let source = make_source(
        0,
        "empty",
        "",
    );

    let line = source
        .line_span_at(offset(0))
        .unwrap();

    assert_eq!(
        line,
        SourceSpan::point(
            SourceId::from_raw(0),
            offset(0),
        )
    );

    assert_eq!(
        source.line_text_at(offset(0)),
        Some("")
    );
}

// ============================================================================
// SourceMap
// ============================================================================

#[test]
fn source_map_starts_empty() {
    let map = SourceMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(
        map.iter().count(),
        0
    );
}

#[test]
fn source_map_assigns_monotonic_ids() {
    let mut map = SourceMap::new();

    let first = map
        .add("first.qasm", "h q[0];")
        .unwrap();

    let second = map
        .add("second.qasm", "x q[0];")
        .unwrap();

    let third = map
        .add("third.qasm", "z q[0];")
        .unwrap();

    assert_eq!(first.as_raw(), 0);
    assert_eq!(second.as_raw(), 1);
    assert_eq!(third.as_raw(), 2);
}

#[test]
fn source_map_assigns_distinct_ids_to_duplicate_sources() {
    let mut map = SourceMap::new();

    let first = map
        .add("a.qasm", "h q[0];")
        .unwrap();

    let second = map
        .add("a.qasm", "h q[0];")
        .unwrap();

    assert_ne!(first, second);
    assert_eq!(map.len(), 2);
}

#[test]
fn source_map_preserves_insertion_order() {
    let mut map = SourceMap::new();

    let first = map
        .add("first", "a")
        .unwrap();

    let second = map
        .add("second", "b")
        .unwrap();

    let ids = map
        .iter()
        .map(SourceFile::id)
        .collect::<Vec<_>>();

    assert_eq!(
        ids,
        vec![first, second]
    );
}

#[test]
fn source_map_get_returns_registered_source() {
    let mut map = SourceMap::new();

    let id = map
        .add("program.qasm", "OPENQASM 3.1;")
        .unwrap();

    let source = map
        .get(id)
        .expect("registered source must be retrievable");

    assert_eq!(source.id(), id);
    assert_eq!(source.name(), "program.qasm");
    assert_eq!(source.text(), "OPENQASM 3.1;");
}

#[test]
fn source_map_get_rejects_unknown_source() {
    let map = SourceMap::new();

    assert!(
        map.get(SourceId::from_raw(99)).is_none()
    );
}

#[test]
fn source_map_require_returns_structured_unknown_source_error() {
    let map = SourceMap::new();
    let id = SourceId::from_raw(99);

    assert_eq!(
        map.require(id),
        Err(SourceLookupError::UnknownSource(id))
    );
}

#[test]
fn source_lookup_error_display_is_stable() {
    let error = SourceLookupError::UnknownSource(
        SourceId::from_raw(99)
    );

    assert_eq!(
        error.to_string(),
        "unknown source: source#99"
    );
}

#[test]
fn source_map_slice_delegates_without_cross_source_access() {
    let mut map = SourceMap::new();

    let first = map
        .add("first", "abcdef")
        .unwrap();

    let second = map
        .add("second", "uvwxyz")
        .unwrap();

    assert_eq!(
        map.slice(span(first.as_raw(), 1, 4)),
        Some("bcd")
    );

    assert_eq!(
        map.slice(span(second.as_raw(), 1, 4)),
        Some("vwx")
    );

    // A span labelled with an unregistered source cannot resolve.
    assert!(
        map.slice(span(99, 0, 1)).is_none()
    );
}

#[test]
fn source_map_position_lookup_is_source_scoped() {
    let mut map = SourceMap::new();

    let first = map
        .add("first", "abc\ndef")
        .unwrap();

    let second = map
        .add("second", "xyz\nuvw")
        .unwrap();

    let first_position = map
        .position_at(first, offset(4))
        .unwrap();

    let second_position = map
        .position_at(second, offset(4))
        .unwrap();

    assert_eq!(
        first_position.source_id(),
        first
    );
    assert_eq!(
        second_position.source_id(),
        second
    );

    assert_eq!(
        first_position.line(),
        LineNumber::from_raw(2)
    );
    assert_eq!(
        second_position.line(),
        LineNumber::from_raw(2)
    );
}

#[test]
fn source_map_line_span_lookup_is_source_scoped() {
    let mut map = SourceMap::new();

    let id = map
        .add("test", "abc\ndef")
        .unwrap();

    let line = map
        .line_span_at(id, offset(4))
        .unwrap();

    assert_eq!(
        map.slice(line),
        Some("def")
    );
}

#[test]
fn source_map_iter_is_stable_across_repeated_reads() {
    let mut map = SourceMap::new();

    map.add("a", "one").unwrap();
    map.add("b", "two").unwrap();
    map.add("c", "three").unwrap();

    let first = map
        .iter()
        .map(|source| (source.id(), source.name(), source.text()))
        .collect::<Vec<_>>();

    let second = map
        .iter()
        .map(|source| (source.id(), source.name(), source.text()))
        .collect::<Vec<_>>();

    assert_eq!(first, second);
}

// ============================================================================
// Error contracts
// ============================================================================

#[test]
fn source_span_errors_are_stable_and_implement_error() {
    let reversed = SourceSpanError::ReversedRange;
    let overflow = SourceSpanError::OffsetOverflow;

    assert_eq!(
        reversed.to_string(),
        "source span start offset exceeds end offset"
    );

    assert_eq!(
        overflow.to_string(),
        "source offset exceeds the frontend supported range"
    );

    fn assert_error<T: std::error::Error>() {}

    assert_error::<SourceSpanError>();
    assert_error::<SourceFileError>();
    assert_error::<SourceLookupError>();
}

#[test]
fn source_file_errors_have_stable_messages() {
    assert_eq!(
        SourceFileError::SourceTooLarge.to_string(),
        "source file is too large for the frontend source model"
    );

    assert_eq!(
        SourceFileError::TooManyLines.to_string(),
        "source file contains too many lines for the frontend source model"
    );
}

// ============================================================================
// Determinism
// ============================================================================

#[test]
fn identical_source_inputs_produce_identical_locations() {
    let first = make_source(
        5,
        "program.qasm",
        "OPENQASM 3.1;\nqubit[2] q;\n",
    );

    let second = make_source(
        5,
        "program.qasm",
        "OPENQASM 3.1;\nqubit[2] q;\n",
    );

    assert_eq!(first.id(), second.id());
    assert_eq!(first.name(), second.name());
    assert_eq!(first.text(), second.text());
    assert_eq!(
        first.line_starts(),
        second.line_starts()
    );

    for value in [
        0,
        1,
        10,
        first.text().len(),
    ] {
        assert_eq!(
            first.position_at(offset(value)),
            second.position_at(offset(value))
        );
    }
}

#[test]
fn source_map_lookup_is_deterministic() {
    let mut first = SourceMap::new();
    let mut second = SourceMap::new();

    for (name, text) in [
        ("a.qasm", "h q[0];"),
        ("b.qasm", "x q[0];"),
        ("c.qasm", "measure q[0] -> c[0];"),
    ] {
        first.add(name, text).unwrap();
        second.add(name, text).unwrap();
    }

    let first_values = first
        .iter()
        .map(|source| {
            (
                source.id(),
                source.name(),
                source.text(),
                source.line_starts(),
            )
        })
        .collect::<Vec<_>>();

    let second_values = second
        .iter()
        .map(|source| {
            (
                source.id(),
                source.name(),
                source.text(),
                source.line_starts(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(first_values, second_values);
}

// ============================================================================
// Boundary matrix
// ============================================================================

#[test]
fn every_ascii_boundary_in_a_source_is_resolvable() {
    let source = make_source(
        0,
        "ascii",
        "OPENQASM 3;\nqubit q;\n",
    );

    for value in 0..=source.text().len() {
        assert!(
            source.position_at(offset(value)).is_some(),
            "ASCII boundary {value} must resolve"
        );
    }
}

#[test]
fn every_utf8_boundary_in_unicode_source_is_resolvable() {
    let text = "αβγ quantum 中心";
    let source = make_source(
        0,
        "unicode",
        text,
    );

    for (index, _) in text.char_indices() {
        assert!(
            source.position_at(offset(index)).is_some(),
            "UTF-8 character boundary {index} must resolve"
        );
    }

    assert!(
        source
            .position_at(offset(text.len()))
            .is_some(),
        "EOF must resolve"
    );
}

#[test]
fn positions_never_move_backward_in_the_source() {
    let source = make_source(
        0,
        "test",
        "one\né中\ntwo\r\nthree",
    );

    let mut previous = None;

    for value in 0..=source.text().len() {
        let current = source.position_at(offset(value));

        if let Some(current) = current {
            if let Some(previous) = previous {
                assert!(
                    current.offset() >= previous.offset(),
                    "positions must be monotonic"
                );
            }

            previous = Some(current);
        }
    }
}

#[test]
fn line_numbers_never_decrease_with_increasing_offsets() {
    let source = make_source(
        0,
        "test",
        "one\né\n中\r\ntwo",
    );

    let mut previous_line = LineNumber::FIRST;

    for value in 0..=source.text().len() {
        if let Some(position) = source.position_at(offset(value)) {
            assert!(
                position.line() >= previous_line,
                "line numbers must be monotonic"
            );

            previous_line = position.line();
        }
    }
}

// ============================================================================
// Frontend integration invariants
// ============================================================================

#[test]
fn source_span_is_suitable_as_the_shared_frontend_location_type() {
    // This compile-time-style usage deliberately mirrors what lexer/parser/
    // validator/diagnostic/lowering code will do.
    let source_id = SourceId::from_raw(12);
    let source = make_source(
        source_id.as_raw(),
        "program.qasm",
        "OPENQASM 3.1;",
    );

    let token_span = SourceSpan::new(
        source_id,
        offset(0),
        offset(8),
    )
    .unwrap();

    let ast_span = SourceSpan::entire(&source);

    let diagnostic_span = token_span
        .union(ast_span)
        .expect("all spans belong to the same source");

    assert_eq!(
        diagnostic_span.source_id(),
        source_id
    );

    assert_eq!(
        source.slice(token_span),
        Some("OPENQASM")
    );
}

#[test]
fn source_identity_does_not_depend_on_display_name() {
    let first = make_source(
        1,
        "same-name.qasm",
        "h q[0];",
    );

    let second = make_source(
        2,
        "same-name.qasm",
        "h q[0];",
    );

    assert_ne!(first.id(), second.id());
    assert_eq!(first.name(), second.name());
    assert_eq!(first.text(), second.text());

    let first_span = SourceSpan::entire(&first);
    let second_span = SourceSpan::entire(&second);

    assert!(first_span.union(second_span).is_none());
}

#[test]
fn source_identity_prevents_cross_file_span_merging() {
    let first = make_source(
        1,
        "a.qasm",
        "h q[0];",
    );

    let second = make_source(
        2,
        "b.qasm",
        "x q[0];",
    );

    assert!(
        SourceSpan::entire(&first)
            .union(SourceSpan::entire(&second))
            .is_none()
    );
}

#[test]
fn source_location_model_is_panic_free_for_normal_invalid_queries() {
    let source = make_source(
        0,
        "test",
        "abcé\nxyz",
    );

    let invalid_offsets = [
        offset(source.text().len() + 1),
        offset(source.text().len() + 10),
    ];

    for value in invalid_offsets {
        assert!(source.position_at(value).is_none());
        assert!(source.line_span_at(value).is_none());
        assert!(source.line_text_at(value).is_none());
    }

    let foreign = SourceSpan::point(
        SourceId::from_raw(99),
        offset(0),
    );

    assert!(source.slice(foreign).is_none());
    assert!(source.start_position(foreign).is_none());
    assert!(source.end_position(foreign).is_none());
}

#[test]
fn source_file_is_clone_stable() {
    let original = make_source(
        8,
        "program.qasm",
        "OPENQASM 3.1;\nqubit[2] q;",
    );

    let clone = original.clone();

    assert_eq!(original.id(), clone.id());
    assert_eq!(original.name(), clone.name());
    assert_eq!(original.text(), clone.text());
    assert_eq!(
        original.line_starts(),
        clone.line_starts()
    );

    for value in [
        0,
        1,
        5,
        original.text().len(),
    ] {
        assert_eq!(
            original.position_at(offset(value)),
            clone.position_at(offset(value))
        );
    }
}

#[test]
fn source_map_is_clone_stable() {
    let mut original = SourceMap::new();

    original.add("a", "one\n").unwrap();
    original.add("b", "two").unwrap();

    let clone = original.clone();

    assert_eq!(original.len(), clone.len());

    let original_values = original
        .iter()
        .map(|source| {
            (
                source.id(),
                source.name(),
                source.text(),
                source.line_starts(),
            )
        })
        .collect::<Vec<_>>();

    let clone_values = clone
        .iter()
        .map(|source| {
            (
                source.id(),
                source.name(),
                source.text(),
                source.line_starts(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(original_values, clone_values);
}