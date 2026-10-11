#![forbid(unsafe_code)]

//! Contract tests for `src/frontend/lexer/cursor.rs`.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the public cursor contract independently of token classification,
//! scanning policy, parser behavior, and computational domain.
//!
//! # Integration
//!
//! These tests exercise the canonical `Cursor`, `CursorPosition`,
//! `CursorCheckpoint`, and `CursorError` types. They intentionally do not
//! depend on token kinds, keyword registries, dialects, scanner internals,
//! diagnostics formatting, or target-specific resource limits.
//!
//! Register this file from `src/frontend/lexer/tests/mod.rs` with:
//!
//!     mod cursor;
//!
//! The parent test module should itself be registered by the crate's test
//! configuration. Do not duplicate cursor implementations in the tests.
//!
//! # Invariants under test
//!
//! - UTF-8 byte offsets are zero-based.
//! - Lines and Unicode-scalar columns are one-based.
//! - Cursor operations never split a UTF-8 scalar.
//! - CRLF represents one logical line break.
//! - CR, LF, U+2028, and U+2029 are recognized as line terminators.
//! - Failed matching and failed restoration do not mutate cursor state.
//! - Checkpoints preserve CRLF bookkeeping.
//! - Slicing validates bounds and UTF-8 boundaries.
//! - EOF is deterministic and is not itself an error.
//! - Behavior does not depend on source-language computational domain.
//!
//! These tests impose no language-level source-size or hardware-capacity
//! limits. Stress tests use finite, test-generated input because test
//! execution is necessarily bounded by the host environment.

use crate::frontend::lexer::cursor::{Cursor, CursorError, CursorPosition};

// ---------------------------------------------------------------------------
// Initial state and EOF
// ---------------------------------------------------------------------------

#[test]
fn empty_source_starts_at_origin_and_is_at_eof() {
    let cursor = Cursor::new("");

    assert_eq!(cursor.source(), "");
    assert_eq!(cursor.source_len_bytes(), 0);
    assert_eq!(cursor.remaining(), "");
    assert!(cursor.is_eof());
    assert_eq!(cursor.peek(), None);
    assert_eq!(cursor.peek_n(0), None);
    assert_eq!(cursor.byte_offset(), 0);
    assert_eq!(cursor.line(), 1);
    assert_eq!(cursor.column(), 1);
    assert_eq!(
        cursor.position(),
        CursorPosition::new(0, 1, 1)
    );
}

#[test]
fn eof_consumption_is_stable_and_returns_none() {
    let mut cursor = Cursor::new("x");

    assert_eq!(cursor.consume(), Some('x'));
    assert!(cursor.is_eof());

    let at_eof = cursor.position();

    assert_eq!(cursor.consume(), None);
    assert_eq!(cursor.consume(), None);
    assert_eq!(cursor.position(), at_eof);
    assert_eq!(cursor.remaining(), "");
    assert!(cursor.is_eof());
}

#[test]
fn eof_does_not_count_as_an_extra_character() {
    let mut cursor = Cursor::new("ab");

    assert_eq!(cursor.consume_while(char::is_ascii_alphabetic), 2);
    assert!(cursor.is_eof());
    assert_eq!(cursor.position(), CursorPosition::new(2, 1, 3));
}

// ---------------------------------------------------------------------------
// UTF-8 and source coordinates
// ---------------------------------------------------------------------------

#[test]
fn byte_offsets_and_columns_use_different_units() {
    let mut cursor = Cursor::new("aλ界");

    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));

    assert_eq!(cursor.consume(), Some('a'));
    assert_eq!(cursor.position(), CursorPosition::new(1, 1, 2));

    assert_eq!(cursor.consume(), Some('λ'));
    assert_eq!(cursor.position(), CursorPosition::new(3, 1, 3));

    assert_eq!(cursor.consume(), Some('界'));
    assert_eq!(cursor.position(), CursorPosition::new(6, 1, 4));

    assert!(cursor.is_eof());
}

#[test]
fn peek_and_lookahead_count_unicode_scalars() {
    let cursor = Cursor::new("aλ界");

    assert_eq!(cursor.peek(), Some('a'));
    assert_eq!(cursor.peek_n(0), Some('a'));
    assert_eq!(cursor.peek_n(1), Some('λ'));
    assert_eq!(cursor.peek_n(2), Some('界'));
    assert_eq!(cursor.peek_n(3), None);

    // Lookahead must not change the source position.
    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
}

#[test]
fn consuming_a_unicode_scalar_never_splits_its_utf8_encoding() {
    let mut cursor = Cursor::new("λx");

    assert_eq!(cursor.consume(), Some('λ'));
    assert_eq!(cursor.byte_offset(), 'λ'.len_utf8());
    assert_eq!(cursor.remaining(), "x");

    assert_eq!(cursor.consume(), Some('x'));
    assert_eq!(cursor.remaining(), "");
    assert!(cursor.is_eof());
}

#[test]
fn source_and_remaining_are_exact_borrowed_text() {
    let source = String::from("alpha λ 界");
    let mut cursor = Cursor::new(source.as_str());

    assert_eq!(cursor.source(), source.as_str());
    assert_eq!(cursor.remaining(), source.as_str());

    assert_eq!(cursor.consume_while(|ch| ch.is_ascii_alphabetic()), 5);
    assert_eq!(cursor.remaining(), " λ 界");
    assert_eq!(cursor.source(), "alpha λ 界");
}

// ---------------------------------------------------------------------------
// Character matching and exact-prefix consumption
// ---------------------------------------------------------------------------

#[test]
fn peek_is_does_not_advance() {
    let cursor = Cursor::new("λ");

    assert!(cursor.peek_is('λ'));
    assert!(!cursor.peek_is('x'));
    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
}

#[test]
fn consume_if_advances_only_on_a_match() {
    let mut cursor = Cursor::new("λx");

    assert!(!cursor.consume_if('x'));
    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));

    assert!(cursor.consume_if('λ'));
    assert_eq!(cursor.position(), CursorPosition::new(2, 1, 2));
    assert_eq!(cursor.peek(), Some('x'));
}

#[test]
fn starts_with_matches_exact_text_without_advancing() {
    let cursor = Cursor::new("quantum::operation");

    assert!(cursor.starts_with("quantum"));
    assert!(cursor.starts_with("quantum::"));
    assert!(!cursor.starts_with("Quantum"));
    assert!(!cursor.starts_with("quantum: "));
    assert!(!cursor.starts_with("quantum::operation;"));

    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
}

#[test]
fn consume_str_consumes_exact_ascii_and_unicode_prefixes() {
    let mut cursor = Cursor::new("quantum::λ + value");

    assert!(cursor.consume_str("quantum::"));
    assert_eq!(cursor.byte_offset(), "quantum::".len());
    assert_eq!(cursor.remaining(), "λ + value");

    assert!(cursor.consume_str("λ"));
    assert_eq!(cursor.remaining(), " + value");

    assert_eq!(
        cursor.position(),
        CursorPosition::new("quantum::λ".len(), 1, 10)
    );
}

#[test]
fn failed_consume_str_does_not_change_any_position_state() {
    let mut cursor = Cursor::new("quantum::λ");

    assert!(cursor.consume_str("quantum"));
    let before = cursor.checkpoint();

    assert!(!cursor.consume_str("::x"));
    assert!(!cursor.consume_str("λx"));
    assert!(!cursor.consume_str("QUANTUM"));

    assert_eq!(cursor.checkpoint(), before);
    assert_eq!(cursor.remaining(), "::λ");
}

#[test]
fn empty_prefix_matches_without_advancing() {
    let mut cursor = Cursor::new("source");
    let before = cursor.checkpoint();

    assert!(cursor.starts_with(""));
    assert!(cursor.consume_str(""));

    assert_eq!(cursor.checkpoint(), before);
    assert_eq!(cursor.remaining(), "source");
}

// ---------------------------------------------------------------------------
// consume_while behavior
// ---------------------------------------------------------------------------

#[test]
fn consume_while_returns_unicode_scalar_count_not_byte_count() {
    let mut cursor = Cursor::new("λλ界!");

    let consumed = cursor.consume_while(|ch| ch != '!');

    assert_eq!(consumed, 3);
    assert_eq!(cursor.remaining(), "!");
    assert_eq!(
        cursor.position(),
        CursorPosition::new("λλ界".len(), 1, 4)
    );
}

#[test]
fn consume_while_stops_before_the_first_nonmatching_scalar() {
    let mut cursor = Cursor::new("abc123");

    let consumed = cursor.consume_while(char::is_ascii_alphabetic);

    assert_eq!(consumed, 3);
    assert_eq!(cursor.remaining(), "123");
    assert_eq!(cursor.position(), CursorPosition::new(3, 1, 4));
}

#[test]
fn consume_while_calls_predicate_on_the_first_nonmatch_but_not_at_eof() {
    let mut cursor = Cursor::new("ab!");

    let mut visited = Vec::new();
    let consumed = cursor.consume_while(|ch| {
        visited.push(ch);
        ch.is_ascii_alphabetic()
    });

    assert_eq!(consumed, 2);
    assert_eq!(visited, vec!['a', 'b', '!']);
    assert_eq!(cursor.remaining(), "!");
}

#[test]
fn consume_while_at_eof_does_not_call_predicate() {
    let mut cursor = Cursor::new("");
    let mut calls = 0usize;

    let consumed = cursor.consume_while(|_| {
        calls += 1;
        true
    });

    assert_eq!(consumed, 0);
    assert_eq!(calls, 0);
    assert!(cursor.is_eof());
}

#[test]
fn consume_while_with_immediate_nonmatch_does_not_advance() {
    let mut cursor = Cursor::new("123abc");
    let before = cursor.checkpoint();

    let consumed = cursor.consume_while(char::is_ascii_alphabetic);

    assert_eq!(consumed, 0);
    assert_eq!(cursor.checkpoint(), before);
}

// ---------------------------------------------------------------------------
// Line terminators and coordinate invariants
// ---------------------------------------------------------------------------

#[test]
fn standalone_lf_advances_one_line() {
    let mut cursor = Cursor::new("a\nb");

    assert_eq!(cursor.consume(), Some('a'));
    assert_eq!(cursor.position(), CursorPosition::new(1, 1, 2));

    assert!(cursor.consume_newline());
    assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

    assert_eq!(cursor.consume(), Some('b'));
    assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
}

#[test]
fn standalone_cr_advances_one_line() {
    let mut cursor = Cursor::new("a\rb");

    assert_eq!(cursor.consume(), Some('a'));
    assert!(cursor.consume_newline());

    assert_eq!(cursor.remaining(), "b");
    assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

    assert_eq!(cursor.consume(), Some('b'));
    assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
}

#[test]
fn crlf_consumed_as_one_logical_newline() {
    let mut cursor = Cursor::new("a\r\nb");

    assert_eq!(cursor.consume(), Some('a'));
    assert!(cursor.consume_newline());

    assert_eq!(cursor.remaining(), "b");
    assert_eq!(cursor.position(), CursorPosition::new(3, 2, 1));

    assert_eq!(cursor.consume(), Some('b'));
    assert_eq!(cursor.position(), CursorPosition::new(4, 2, 2));
}

#[test]
fn crlf_consumed_one_scalar_at_a_time_still_advances_only_one_line() {
    let mut cursor = Cursor::new("\r\nx");

    assert_eq!(cursor.consume(), Some('\r'));
    assert_eq!(cursor.position(), CursorPosition::new(1, 2, 1));

    assert_eq!(cursor.consume(), Some('\n'));
    assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

    assert_eq!(cursor.consume(), Some('x'));
    assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
}

#[test]
fn unicode_line_separator_advances_one_line() {
    let mut cursor = Cursor::new("a\u{2028}b");

    assert_eq!(cursor.consume(), Some('a'));
    assert!(cursor.consume_newline());

    assert_eq!(cursor.remaining(), "b");
    assert_eq!(
        cursor.position(),
        CursorPosition::new("a\u{2028}".len(), 2, 1)
    );
}

#[test]
fn unicode_paragraph_separator_advances_one_line() {
    let mut cursor = Cursor::new("a\u{2029}b");

    assert_eq!(cursor.consume(), Some('a'));
    assert!(cursor.consume_newline());

    assert_eq!(cursor.remaining(), "b");
    assert_eq!(
        cursor.position(),
        CursorPosition::new("a\u{2029}".len(), 2, 1)
    );
}

#[test]
fn consume_newline_returns_false_without_advancing_on_non_newline() {
    let mut cursor = Cursor::new("x\n");
    let before = cursor.checkpoint();

    assert!(!cursor.consume_newline());
    assert_eq!(cursor.checkpoint(), before);
    assert_eq!(cursor.remaining(), "x\n");
}

#[test]
fn consume_newline_returns_false_at_eof() {
    let mut cursor = Cursor::new("");

    let before = cursor.checkpoint();

    assert!(!cursor.consume_newline());
    assert_eq!(cursor.checkpoint(), before);
    assert!(cursor.is_eof());
}

#[test]
fn mixed_line_endings_produce_consistent_positions() {
    let source = "a\r\nb\rc\nd\u{2028}e\u{2029}f";
    let mut cursor = Cursor::new(source);

    let expected = [
        ('a', CursorPosition::new(1, 1, 2)),
        ('\r', CursorPosition::new(2, 2, 1)),
        ('\n', CursorPosition::new(3, 2, 1)),
        ('b', CursorPosition::new(4, 2, 2)),
        ('\r', CursorPosition::new(5, 3, 1)),
        ('c', CursorPosition::new(6, 3, 2)),
        ('\n', CursorPosition::new(7, 4, 1)),
        ('d', CursorPosition::new(8, 4, 2)),
        ('\u{2028}', CursorPosition::new(11, 5, 1)),
        ('e', CursorPosition::new(12, 5, 2)),
        ('\u{2029}', CursorPosition::new(15, 6, 1)),
        ('f', CursorPosition::new(16, 6, 2)),
    ];

    for (expected_char, expected_position) in expected {
        assert_eq!(cursor.consume(), Some(expected_char));
        assert_eq!(cursor.position(), expected_position);
    }

    assert!(cursor.is_eof());
    assert_eq!(cursor.byte_offset(), source.len());
    assert_eq!(cursor.line(), 6);
    assert_eq!(cursor.column(), 2);
}

// ---------------------------------------------------------------------------
// Checkpoint, restore, and speculative scanning
// ---------------------------------------------------------------------------

#[test]
fn checkpoint_and_restore_recover_the_complete_position() {
    let mut cursor = Cursor::new("ab\r\nλz");

    assert_eq!(cursor.consume(), Some('a'));
    let checkpoint = cursor.checkpoint();
    let saved_position = cursor.position();

    assert_eq!(cursor.consume(), Some('b'));
    assert!(cursor.consume_newline());
    assert_eq!(cursor.consume(), Some('λ'));
    assert_eq!(cursor.consume(), Some('z'));

    assert!(cursor.is_eof());

    cursor.restore(checkpoint).expect("checkpoint should be valid");

    assert_eq!(cursor.position(), saved_position);
    assert_eq!(cursor.remaining(), "b\r\nλz");
}

#[test]
fn checkpoint_between_cr_and_lf_preserves_newline_bookkeeping() {
    let mut cursor = Cursor::new("\r\nx");

    assert_eq!(cursor.consume(), Some('\r'));
    let between_cr_and_lf = cursor.checkpoint();

    assert_eq!(cursor.consume(), Some('\n'));
    assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

    cursor
        .restore(between_cr_and_lf)
        .expect("checkpoint between CR and LF should be restorable");

    assert_eq!(cursor.position(), CursorPosition::new(1, 2, 1));
    assert_eq!(cursor.remaining(), "\nx");

    assert_eq!(cursor.consume(), Some('\n'));
    assert_eq!(cursor.position(), CursorPosition::new(2, 2, 1));

    assert_eq!(cursor.consume(), Some('x'));
    assert_eq!(cursor.position(), CursorPosition::new(3, 2, 2));
}

#[test]
fn restore_to_initial_checkpoint_returns_to_source_start() {
    let mut cursor = Cursor::new("λ\r\nvalue");
    let beginning = cursor.checkpoint();

    while cursor.consume().is_some() {}

    cursor
        .restore(beginning)
        .expect("initial checkpoint should be valid");

    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
    assert_eq!(cursor.remaining(), "λ\r\nvalue");
    assert!(!cursor.is_eof());
}

#[test]
fn restore_rejects_checkpoint_from_a_different_source() {
    let first_source = String::from("alpha");
    let second_source = String::from("bravo");

    let first = Cursor::new(first_source.as_str());
    let mut second = Cursor::new(second_source.as_str());

    let foreign_checkpoint = first.checkpoint();
    let before = second.checkpoint();

    let result = second.restore(foreign_checkpoint);

    assert!(matches!(result, Err(CursorError::ForeignCheckpoint)));
    assert_eq!(second.checkpoint(), before);
}

#[test]
fn failed_restore_leaves_cursor_state_unchanged() {
    let source_a = String::from("source-a");
    let source_b = String::from("source-b");

    let foreign_cursor = Cursor::new(source_a.as_str());
    let mut cursor = Cursor::new(source_b.as_str());

    assert_eq!(cursor.consume(), Some('s'));
    let before = cursor.checkpoint();

    assert!(matches!(
        cursor.restore(foreign_cursor.checkpoint()),
        Err(CursorError::ForeignCheckpoint)
    ));

    assert_eq!(cursor.checkpoint(), before);
    assert_eq!(cursor.remaining(), "ource-b");
}

// ---------------------------------------------------------------------------
// Validated slicing
// ---------------------------------------------------------------------------

#[test]
fn slice_returns_exact_source_ranges() {
    let cursor = Cursor::new("aλ界z");

    assert_eq!(cursor.slice(0..1).expect("valid ASCII range"), "a");
    assert_eq!(cursor.slice(1..3).expect("valid lambda range"), "λ");
    assert_eq!(cursor.slice(3..6).expect("valid ideograph range"), "界");
    assert_eq!(cursor.slice(6..7).expect("valid final range"), "z");
    assert_eq!(cursor.slice(0..7).expect("whole source range"), "aλ界z");
}

#[test]
fn slice_accepts_empty_ranges_at_valid_boundaries() {
    let cursor = Cursor::new("aλz");

    assert_eq!(cursor.slice(0..0).expect("empty start range"), "");
    assert_eq!(cursor.slice(1..1).expect("empty range"), "");
    assert_eq!(cursor.slice(3..3).expect("empty range"), "");
    assert_eq!(cursor.slice(4..4).expect("empty end range"), "");
}

#[test]
fn slice_rejects_reversed_ranges() {
    let cursor = Cursor::new("abc");

    let result = cursor.slice(2..1);

    assert!(matches!(
        result,
        Err(CursorError::InvalidRange {
            start: 2,
            end: 1,
            source_len: 3,
        })
    ));
}

#[test]
fn slice_rejects_ranges_beyond_source_length() {
    let cursor = Cursor::new("abc");

    let result = cursor.slice(0..4);

    assert!(matches!(
        result,
        Err(CursorError::InvalidRange {
            start: 0,
            end: 4,
            source_len: 3,
        })
    ));
}

#[test]
fn slice_rejects_non_character_boundary_start() {
    let cursor = Cursor::new("aλz");

    // The UTF-8 encoding of λ occupies byte offsets 1..3. Offset 2 is
    // inside its encoding and is not a valid source-slice boundary.
    let result = cursor.slice(2..3);

    assert!(matches!(
        result,
        Err(CursorError::NotCharBoundary { offset: 2 })
    ));
}

#[test]
fn slice_rejects_non_character_boundary_end() {
    let cursor = Cursor::new("aλz");

    let result = cursor.slice(1..2);

    assert!(matches!(
        result,
        Err(CursorError::NotCharBoundary { offset: 2 })
    ));
}

#[test]
fn slice_validation_does_not_change_cursor_state() {
    let mut cursor = Cursor::new("aλz");
    assert_eq!(cursor.consume(), Some('a'));

    let before = cursor.checkpoint();

    let _ = cursor.slice(2..3);
    let _ = cursor.slice(0..99);
    let _ = cursor.slice(3..1);

    assert_eq!(cursor.checkpoint(), before);
    assert_eq!(cursor.remaining(), "λz");
}

#[test]
fn slice_from_returns_text_consumed_since_checkpoint() {
    let mut cursor = Cursor::new("name::λ + value");

    let start = cursor.checkpoint();

    assert!(cursor.consume_str("name::"));
    assert_eq!(cursor.consume(), Some('λ'));

    let consumed = cursor
        .slice_from(start)
        .expect("checkpoint should precede the current position");

    assert_eq!(consumed, "name::λ");
    assert_eq!(cursor.remaining(), " + value");
}

#[test]
fn slice_from_rejects_a_checkpoint_after_current_position() {
    let mut cursor = Cursor::new("abcdef");

    assert_eq!(cursor.consume(), Some('a'));
    let later = cursor.checkpoint();

    assert_eq!(cursor.consume(), Some('b'));
    cursor.restore(later).expect("restore to later checkpoint");

    // Create a checkpoint at byte 2, then return to byte 1.
    assert_eq!(cursor.consume(), Some('c'));
    let after_c = cursor.checkpoint();

    cursor
        .restore(later)
        .expect("restore should return to byte 1");

    assert!(matches!(
        cursor.slice_from(after_c),
        Err(CursorError::InvalidRange {
            start: 2,
            end: 1,
            source_len: 6,
        })
    ));
}

#[test]
fn slice_from_rejects_a_foreign_checkpoint() {
    let source_a = String::from("alpha");
    let source_b = String::from("bravo");

    let first = Cursor::new(source_a.as_str());
    let mut second = Cursor::new(source_b.as_str());

    assert_eq!(second.consume(), Some('b'));

    assert!(matches!(
        second.slice_from(first.checkpoint()),
        Err(CursorError::ForeignCheckpoint)
    ));
}

// ---------------------------------------------------------------------------
// Domain neutrality and deterministic traversal
// ---------------------------------------------------------------------------

#[test]
fn cursor_treats_domain_specific_spellings_as_ordinary_source_text() {
    let source = concat!(
        "fn main() {}\n",
        "quantum operation vendor::custom;\n",
        "module hdl::fabric;\n",
        "tensor compute distributed network;\n",
    );

    let mut cursor = Cursor::new(source);
    let mut reconstructed = String::new();

    while let Some(ch) = cursor.consume() {
        reconstructed.push(ch);
    }

    assert_eq!(reconstructed, source);
    assert_eq!(cursor.byte_offset(), source.len());
    assert!(cursor.is_eof());
}

#[test]
fn repeated_traversals_produce_identical_positions_and_text() {
    let source = "aλ\r\nquantum\u{2028}hdl\u{2029}";

    fn traverse(source: &str) -> Vec<(char, CursorPosition)> {
        let mut cursor = Cursor::new(source);
        let mut output = Vec::new();

        while let Some(ch) = cursor.consume() {
            output.push((ch, cursor.position()));
        }

        output
    }

    assert_eq!(traverse(source), traverse(source));
}

#[test]
fn checkpoints_can_be_restored_repeatedly_without_position_drift() {
    let mut cursor = Cursor::new("λ\r\n界");
    let checkpoint = cursor.checkpoint();

    for _ in 0..3 {
        assert_eq!(cursor.consume(), Some('λ'));
        assert_eq!(cursor.consume(), Some('\r'));
        assert_eq!(cursor.consume(), Some('\n'));
        assert_eq!(cursor.consume(), Some('界'));

        assert_eq!(cursor.position(), CursorPosition::new(7, 2, 2));

        cursor
            .restore(checkpoint)
            .expect("checkpoint should remain valid");

        assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
    }
}

// ---------------------------------------------------------------------------
// Finite, resource-aware stress coverage
// ---------------------------------------------------------------------------

#[test]
fn long_ascii_input_preserves_offsets_and_columns() {
    // This test uses a generated input size rather than defining a lexer
    // maximum. Production limits belong to the caller's resource policy.
    let length = 32 * 1024;
    let source = "x".repeat(length);
    let mut cursor = Cursor::new(source.as_str());

    assert_eq!(cursor.consume_while(|ch| ch == 'x'), length);
    assert!(cursor.is_eof());
    assert_eq!(cursor.byte_offset(), length);
    assert_eq!(cursor.line(), 1);
    assert_eq!(cursor.column(), length + 1);
}

#[test]
fn long_unicode_input_preserves_byte_and_scalar_counts() {
    let scalar_count = 8 * 1024;
    let source = "界".repeat(scalar_count);
    let expected_bytes = source.len();
    let mut cursor = Cursor::new(source.as_str());

    assert_eq!(cursor.consume_while(|ch| ch == '界'), scalar_count);
    assert!(cursor.is_eof());
    assert_eq!(cursor.byte_offset(), expected_bytes);
    assert_eq!(cursor.line(), 1);
    assert_eq!(cursor.column(), scalar_count + 1);
}

#[test]
fn long_input_with_mixed_newlines_preserves_coordinates() {
    let repetitions = 2 * 1024;
    let source = "a\r\nb\rc\n".repeat(repetitions);
    let mut cursor = Cursor::new(source.as_str());

    while cursor.consume().is_some() {}

    assert!(cursor.is_eof());
    assert_eq!(cursor.byte_offset(), source.len());

    // Each repeated sequence has three line breaks:
    // CRLF, standalone CR, and standalone LF.
    assert_eq!(cursor.line(), repetitions * 3 + 1);
    assert_eq!(cursor.column(), 1);
}

#[test]
fn large_prefix_consumption_matches_character_by_character_consumption() {
    let source = format!("{}λ{}", "a".repeat(4096), "界".repeat(2048));

    let prefix_len = 4096;
    let prefix = &source[..prefix_len];

    let mut by_prefix = Cursor::new(source.as_str());
    let mut by_character = Cursor::new(source.as_str());

    assert!(by_prefix.consume_str(prefix));

    for _ in prefix.chars() {
        assert!(by_character.consume().is_some());
    }

    assert_eq!(by_prefix.position(), by_character.position());
    assert_eq!(by_prefix.remaining(), by_character.remaining());
    assert_eq!(by_prefix.checkpoint(), by_character.checkpoint());
}