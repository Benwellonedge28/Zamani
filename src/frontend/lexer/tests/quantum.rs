"src/frontend/lexer/tests/quantum.rs"

#![forbid(unsafe_code)]

//! Production conformance tests for Zamani quantum-literal recognition.
//!
//! File: src/frontend/lexer/tests/quantum.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Normative references:
//! - grammar/lexer/quantum-literals.g4
//! - grammar/lexer/literals.g4
//! - grammar/lexer/lexer.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/spec/lexical.md
//! - grammar/specification/lexical.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//!
//! Implementation under test:
//! - src/frontend/lexer/quantum.rs
//! - src/frontend/lexer/cursor.rs
//! - src/frontend/lexer/config.rs
//!
//! Integration:
//! Register this module in src/frontend/lexer/tests/mod.rs:
//
//!     mod quantum;
//
//! These tests call the production recognizer directly. They do not create
//! a second quantum lexer, enumerate quantum operations, or define quantum
//! semantic behavior.
//!
//! Architectural invariants:
//! - The compact literal vocabulary follows the normative grammar.
//! - Failed recognition does not consume source.
//! - Successful recognition consumes exactly one literal.
//! - Original spelling and UTF-8 byte positions are preserved.
//! - Resource budgets are implementation policies, not language semantics.
//! - Quantum operations, registers, circuits, and qubit references are not
//!   implemented by this literal recognizer.
//! - The recognizer does not allocate quantum states or choose hardware.
//! - Safe Rust only; no unsafe code.
//!
//! Scalability:
//! The finite set of compact state symbols is syntax, not a limit on qubits,
//! registers, circuits, quantum operations, hardware, or computational scale.
//! The recognizer imposes no arbitrary language-level capacity ceiling.

use super::super::config::ResourceLimit;
use super::super::cursor::{Cursor, CursorPosition};
use super::super::quantum::{
    scan_quantum_literal,
    scan_quantum_literal_with_limit,
    QuantumLiteral,
    QuantumLiteralError,
    QuantumLiteralErrorKind,
    QuantumStateSymbol,
};

/// Assert that a complete source string is recognized as the expected
/// canonical quantum literal.
fn assert_literal(source: &str, expected: QuantumStateSymbol) {
    let mut cursor = Cursor::new(source);

    let literal = scan_quantum_literal(&mut cursor)
        .unwrap_or_else(|error| {
            panic!("valid quantum literal {source:?} failed: {error}")
        })
        .unwrap_or_else(|| {
            panic!("expected quantum literal recognition for {source:?}")
        });

    assert_eq!(literal.symbol(), expected);
    assert_eq!(literal.byte_range(), 0..source.len());
    assert_eq!(literal.byte_len(), source.len());
    assert_eq!(
        literal.spelling(&cursor).expect("source range must be valid"),
        source
    );
    assert!(cursor.is_eof());
}

/// Assert that a source prefix is not recognized and remains untouched.
fn assert_not_literal_without_consumption(source: &str) {
    let mut cursor = Cursor::new(source);
    let original_position = cursor.position();
    let original_remaining = cursor.remaining();

    let result = scan_quantum_literal(&mut cursor)
        .expect("unrecognized input is not itself a recognizer error");

    assert!(
        result.is_none(),
        "malformed or unsupported input was recognized: {source:?}"
    );
    assert_eq!(cursor.position(), original_position);
    assert_eq!(cursor.remaining(), original_remaining);
}

/// Assert that a valid literal is followed by untouched source.
fn assert_literal_prefix(
    source: &str,
    expected_spelling: &str,
    expected_symbol: QuantumStateSymbol,
    remaining: &str,
) {
    let mut cursor = Cursor::new(source);

    let literal = scan_quantum_literal(&mut cursor)
        .expect("recognition should not fail")
        .expect("valid literal prefix should be recognized");

    assert_eq!(literal.symbol(), expected_symbol);
    assert_eq!(
        literal.spelling(&cursor).expect("source range must be valid"),
        expected_spelling
    );
    assert_eq!(cursor.remaining(), remaining);
}

// ---------------------------------------------------------------------------
// Canonical grammar forms
// ---------------------------------------------------------------------------

#[test]
fn recognizes_zero_state_literal() {
    assert_literal("|0⟩", QuantumStateSymbol::Zero);
}

#[test]
fn recognizes_one_state_literal() {
    assert_literal("|1⟩", QuantumStateSymbol::One);
}

#[test]
fn recognizes_plus_state_literal() {
    assert_literal("|+⟩", QuantumStateSymbol::Plus);
}

#[test]
fn recognizes_minus_state_literal() {
    assert_literal("|-⟩", QuantumStateSymbol::Minus);
}

#[test]
fn canonical_spellings_match_the_grammar_vocabulary() {
    let cases = [
        (QuantumStateSymbol::Zero, "|0⟩"),
        (QuantumStateSymbol::One, "|1⟩"),
        (QuantumStateSymbol::Plus, "|+⟩"),
        (QuantumStateSymbol::Minus, "|-⟩"),
    ];

    for (symbol, spelling) in cases {
        assert_eq!(symbol.canonical_spelling(), spelling);
        assert_eq!(symbol.as_char(), spelling.chars().nth(1).unwrap());
        assert_literal(spelling, symbol);
    }
}

#[test]
fn every_supported_literal_is_recognized_deterministically() {
    let cases = [
        ("|0⟩", QuantumStateSymbol::Zero),
        ("|1⟩", QuantumStateSymbol::One),
        ("|+⟩", QuantumStateSymbol::Plus),
        ("|-⟩", QuantumStateSymbol::Minus),
    ];

    for (source, expected) in cases {
        for _ in 0..8 {
            let mut cursor = Cursor::new(source);

            let literal = scan_quantum_literal(&mut cursor)
                .expect("recognition should be deterministic")
                .expect("canonical literal should be recognized");

            assert_eq!(literal.symbol(), expected);
            assert_eq!(literal.byte_range(), 0..source.len());
            assert!(cursor.is_eof());
        }
    }
}

// ---------------------------------------------------------------------------
// Negative cases and malformed notation
// ---------------------------------------------------------------------------

#[test]
fn rejects_unsupported_state_symbols_without_consuming_input() {
    for source in [
        "|2⟩",
        "|3⟩",
        "|a⟩",
        "|ψ⟩",
        "|alpha⟩",
        "|00⟩",
        "|01⟩",
        "|10⟩",
        "|11⟩",
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn rejects_malformed_delimiters_without_consuming_input() {
    for source in [
        "",
        "|",
        "||",
        "|0",
        "|1",
        "|+",
        "|-",
        "|0>",
        "|1>",
        "|+>",
        "|->",
        "0⟩",
        "1⟩",
        "⟨0|",
        "⟨1|",
        "<0|",
        "|0⟩>",
        "||0⟩",
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn rejects_whitespace_inside_the_canonical_literal() {
    for source in [
        "| 0⟩",
        "|0 ⟩",
        "| 1⟩",
        "|+ ⟩",
        "|- ⟩",
        "|0\t⟩",
        "|0\n⟩",
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn rejects_ascii_lookalikes_for_the_unicode_ket_delimiter() {
    for source in [
        "|0>",
        "|1>",
        "|+>",
        "|->",
        "|0}",
        "|0]",
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn rejects_noncanonical_unicode_variants() {
    // The grammar requires the exact closing ket character U+27E9.
    // Visually similar characters must not be silently normalized.
    for source in [
        "|0〉", // U+3009, right angle bracket
        "|0❯", // U+276F, heavy right-pointing angle quotation mark
        "|0›", // U+203A, single right-pointing angle quotation mark
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn does_not_treat_a_general_bra_as_a_compact_ket_literal() {
    assert_not_literal_without_consumption("⟨0|");
    assert_not_literal_without_consumption("⟨1|");
}

#[test]
fn does_not_consume_a_valid_looking_prefix_of_malformed_notation() {
    for source in [
        "|00⟩",
        "|01⟩",
        "|0>",
        "|0x",
        "|+ ",
    ] {
        let mut cursor = Cursor::new(source);
        let before = cursor.position();

        let result = scan_quantum_literal(&mut cursor)
            .expect("malformed input should not cause a recognizer error");

        assert!(result.is_none(), "unexpected match for {source:?}");
        assert_eq!(cursor.position(), before);
        assert_eq!(cursor.remaining(), source);
    }
}

// ---------------------------------------------------------------------------
// Source preservation and cursor behavior
// ---------------------------------------------------------------------------

#[test]
fn successful_recognition_consumes_exactly_one_literal() {
    assert_literal_prefix(
        "|0⟩ + value",
        "|0⟩",
        QuantumStateSymbol::Zero,
        " + value",
    );

    assert_literal_prefix(
        "|1⟩; next",
        "|1⟩",
        QuantumStateSymbol::One,
        "; next",
    );

    assert_literal_prefix(
        "|+⟩\nnext",
        "|+⟩",
        QuantumStateSymbol::Plus,
        "\nnext",
    );

    assert_literal_prefix(
        "|-⟩,next",
        "|-⟩",
        QuantumStateSymbol::Minus,
        ",next",
    );
}

#[test]
fn adjacent_literals_can_be_scanned_independently() {
    let source = "|0⟩|1⟩|+⟩|-⟩";
    let mut cursor = Cursor::new(source);

    let expected = [
        QuantumStateSymbol::Zero,
        QuantumStateSymbol::One,
        QuantumStateSymbol::Plus,
        QuantumStateSymbol::Minus,
    ];

    for symbol in expected {
        let literal = scan_quantum_literal(&mut cursor)
            .expect("adjacent literal should scan")
            .expect("each canonical literal should be recognized");

        assert_eq!(literal.symbol(), symbol);
        assert_eq!(
            literal.spelling(&cursor).expect("valid source range"),
            symbol.canonical_spelling()
        );
    }

    assert!(cursor.is_eof());
}

#[test]
fn literals_separated_by_whitespace_can_be_scanned_sequentially() {
    let mut cursor = Cursor::new("|0⟩ |1⟩");

    let first = scan_quantum_literal(&mut cursor)
        .expect("first literal should scan")
        .expect("first literal should be recognized");

    assert_eq!(first.symbol(), QuantumStateSymbol::Zero);
    assert_eq!(cursor.remaining(), " |1⟩");

    let space = cursor.consume();
    assert_eq!(space, Some(' '));

    let second = scan_quantum_literal(&mut cursor)
        .expect("second literal should scan")
        .expect("second literal should be recognized");

    assert_eq!(second.symbol(), QuantumStateSymbol::One);
    assert!(cursor.is_eof());
}

#[test]
fn source_after_a_literal_is_not_consumed() {
    for (source, expected_remaining, expected_symbol) in [
        ("|0⟩identifier", "identifier", QuantumStateSymbol::Zero),
        ("|1⟩123", "123", QuantumStateSymbol::One),
        ("|+⟩+", "+", QuantumStateSymbol::Plus),
        ("|-⟩-", "-", QuantumStateSymbol::Minus),
    ] {
        let mut cursor = Cursor::new(source);

        let literal = scan_quantum_literal(&mut cursor)
            .expect("literal prefix should scan")
            .expect("literal prefix should be recognized");

        assert_eq!(literal.symbol(), expected_symbol);
        assert_eq!(cursor.remaining(), expected_remaining);
    }
}

#[test]
fn byte_ranges_are_half_open_and_start_at_the_current_cursor() {
    let mut cursor = Cursor::new("prefix |0⟩ suffix");

    for _ in "prefix ".chars() {
        assert!(cursor.consume().is_some());
    }

    let start = cursor.byte_offset();
    let literal = scan_quantum_literal(&mut cursor)
        .expect("literal should scan")
        .expect("literal should be recognized");

    assert_eq!(literal.start().byte_offset(), start);
    assert_eq!(
        literal.end().byte_offset(),
        start + "|0⟩".len()
    );
    assert_eq!(
        literal.byte_range(),
        start..start + "|0⟩".len()
    );
    assert_eq!(
        literal.spelling(&cursor).expect("valid source range"),
        "|0⟩"
    );
    assert_eq!(cursor.remaining(), " suffix");
}

#[test]
fn byte_length_counts_utf8_bytes_not_unicode_scalar_values() {
    for (source, symbol) in [
        ("|0⟩", QuantumStateSymbol::Zero),
        ("|1⟩", QuantumStateSymbol::One),
        ("|+⟩", QuantumStateSymbol::Plus),
        ("|-⟩", QuantumStateSymbol::Minus),
    ] {
        let mut cursor = Cursor::new(source);

        let literal = scan_quantum_literal(&mut cursor)
            .expect("literal should scan")
            .expect("literal should be recognized");

        assert_eq!(literal.symbol(), symbol);
        assert_eq!(literal.byte_len(), source.len());
        assert_eq!(literal.byte_len(), literal.spelling(&cursor).unwrap().len());
        assert!(literal.byte_len() > source.chars().count());
    }
}

#[test]
fn positions_after_unicode_prefixes_remain_utf8_correct() {
    let source = "λ |+⟩";
    let mut cursor = Cursor::new(source);

    assert_eq!(cursor.consume(), Some('λ'));
    assert_eq!(cursor.consume(), Some(' '));

    let start = cursor.position();
    assert_eq!(start.byte_offset(), "λ ".len());
    assert_eq!(start.line(), 1);
    assert_eq!(start.column(), 3);

    let literal = scan_quantum_literal(&mut cursor)
        .expect("literal should scan")
        .expect("literal should be recognized");

    assert_eq!(literal.symbol(), QuantumStateSymbol::Plus);
    assert_eq!(literal.start(), start);
    assert_eq!(
        literal.end().byte_offset(),
        "λ |+⟩".len()
    );
    assert_eq!(cursor.remaining(), "");
}

#[test]
fn literal_recognition_does_not_normalize_source_text() {
    let canonical = "|0⟩";
    let lookalike = "|0〉";

    assert_ne!(canonical, lookalike);
    assert_literal(canonical, QuantumStateSymbol::Zero);
    assert_not_literal_without_consumption(lookalike);
}

// ---------------------------------------------------------------------------
// Resource-budget behavior
// ---------------------------------------------------------------------------

#[test]
fn unlimited_budget_accepts_every_canonical_literal() {
    for source in ["|0⟩", "|1⟩", "|+⟩", "|-⟩"] {
        let mut cursor = Cursor::new(source);

        let literal = scan_quantum_literal_with_limit(
            &mut cursor,
            ResourceLimit::Unlimited,
        )
        .expect("unlimited budget should accept a valid literal")
        .expect("canonical literal should be recognized");

        assert_eq!(literal.byte_len(), source.len());
        assert!(cursor.is_eof());
    }
}

#[test]
fn exact_byte_budget_accepts_the_literal() {
    for source in ["|0⟩", "|1⟩", "|+⟩", "|-⟩"] {
        let mut cursor = Cursor::new(source);
        let budget = ResourceLimit::at_most(source.len())
            .expect("literal byte length is nonzero");

        let literal = scan_quantum_literal_with_limit(&mut cursor, budget)
            .expect("exact budget should accept the literal")
            .expect("literal should be recognized");

        assert_eq!(literal.byte_len(), source.len());
        assert!(cursor.is_eof());
    }
}

#[test]
fn budget_one_byte_below_literal_length_is_reported() {
    for source in ["|0⟩", "|1⟩", "|+⟩", "|-⟩"] {
        let mut cursor = Cursor::new(source);
        let budget = ResourceLimit::at_most(source.len() - 1)
            .expect("one byte below a nonempty literal is positive");

        let error = scan_quantum_literal_with_limit(&mut cursor, budget)
            .expect_err("undersized budget must be reported");

        assert_eq!(
            error.kind(),
            QuantumLiteralErrorKind::ByteBudgetExceeded
        );
        assert_eq!(error.byte_range(), 0..source.len());

        // The recognizer's documented contract consumes the recognized
        // literal before reporting that it exceeded the configured budget.
        assert!(cursor.is_eof());
    }
}

#[test]
fn budget_failure_has_consistent_source_positions() {
    let source = "|+⟩";
    let mut cursor = Cursor::new(source);
    let budget = ResourceLimit::at_most(1)
        .expect("positive budget is valid");

    let error = scan_quantum_literal_with_limit(&mut cursor, budget)
        .expect_err("literal must exceed the budget");

    assert_eq!(error.kind(), QuantumLiteralErrorKind::ByteBudgetExceeded);
    assert_eq!(error.start(), CursorPosition::new(0, 1, 1));
    assert_eq!(error.end().byte_offset(), source.len());
    assert_eq!(error.byte_range(), 0..source.len());
    assert!(cursor.is_eof());
}

#[test]
fn zero_is_not_a_valid_explicit_resource_budget() {
    assert!(ResourceLimit::at_most(0).is_none());
}

#[test]
fn resource_limits_do_not_change_literal_meaning() {
    let source = "|1⟩";

    let mut unlimited_cursor = Cursor::new(source);
    let unlimited = scan_quantum_literal(&mut unlimited_cursor)
        .expect("unlimited scan should succeed")
        .expect("literal should be recognized");

    let mut exact_cursor = Cursor::new(source);
    let exact = scan_quantum_literal_with_limit(
        &mut exact_cursor,
        ResourceLimit::at_most(source.len()).unwrap(),
    )
    .expect("exact-budget scan should succeed")
    .expect("literal should be recognized");

    assert_eq!(unlimited.symbol(), exact.symbol());
    assert_eq!(unlimited.byte_range(), exact.byte_range());
    assert_eq!(
        unlimited.spelling(&unlimited_cursor).unwrap(),
        exact.spelling(&exact_cursor).unwrap()
    );
}

// ---------------------------------------------------------------------------
// Error contracts
// ---------------------------------------------------------------------------

#[test]
fn byte_budget_error_has_a_stable_kind_and_display_message() {
    let source = "|0⟩";
    let mut cursor = Cursor::new(source);

    let error = scan_quantum_literal_with_limit(
        &mut cursor,
        ResourceLimit::at_most(1).unwrap(),
    )
    .expect_err("budget must be exceeded");

    assert_eq!(
        error.kind(),
        QuantumLiteralErrorKind::ByteBudgetExceeded
    );
    assert!(
        error.to_string().contains("byte budget"),
        "display message should explain the resource-policy failure"
    );
}

#[test]
fn unrecognized_input_is_not_reported_as_a_budget_error() {
    let mut cursor = Cursor::new("|2⟩");

    let result = scan_quantum_literal_with_limit(
        &mut cursor,
        ResourceLimit::at_most(1).unwrap(),
    )
    .expect("unrecognized input should not be a budget error");

    assert!(result.is_none());
    assert_eq!(cursor.byte_offset(), 0);
}

#[test]
fn successful_literal_ranges_never_exceed_source_length() {
    for source in ["|0⟩", "prefix |1⟩", "λ|-⟩suffix"] {
        let mut cursor = Cursor::new(source);

        if source.starts_with("prefix ") {
            for _ in "prefix ".chars() {
                cursor.consume();
            }
        } else if source.starts_with('λ') {
            cursor.consume();
        }

        if source.starts_with("λ") {
            // The test input has no separator between the prefix and literal.
            // The cursor is now exactly at the start of the ket notation.
        }

        let literal = scan_quantum_literal(&mut cursor)
            .expect("recognition should not fail")
            .expect("test source should contain a literal at the cursor");

        let range = literal.byte_range();
        assert!(range.start <= range.end);
        assert!(range.end <= source.len());
        assert!(source.is_char_boundary(range.start));
        assert!(source.is_char_boundary(range.end));
    }
}

// ---------------------------------------------------------------------------
// Separation of lexical and semantic responsibilities
// ---------------------------------------------------------------------------

#[test]
fn state_symbol_is_only_a_compact_lexical_classification() {
    let cases = [
        ("|0⟩", QuantumStateSymbol::Zero),
        ("|1⟩", QuantumStateSymbol::One),
        ("|+⟩", QuantumStateSymbol::Plus),
        ("|-⟩", QuantumStateSymbol::Minus),
    ];

    for (source, expected_symbol) in cases {
        let mut cursor = Cursor::new(source);

        let literal = scan_quantum_literal(&mut cursor)
            .expect("literal recognition should succeed")
            .expect("literal should be recognized");

        assert_eq!(literal.symbol(), expected_symbol);

        // The literal recognizer only classifies source notation. It does not
        // allocate qubits, construct state vectors, choose simulators, or
        // lower operations to quantum::ir.
        assert_eq!(
            literal.spelling(&cursor).unwrap(),
            expected_symbol.canonical_spelling()
        );
    }
}

#[test]
fn arbitrary_quantum_operation_names_are_not_quantum_literals() {
    for source in [
        "H",
        "X",
        "Y",
        "Z",
        "CNOT",
        "SWAP",
        "custom_operation",
        "vendor_operation",
    ] {
        assert_not_literal_without_consumption(source);
    }
}

#[test]
fn qubit_references_are_not_quantum_literals() {
    for source in ["q", "q0", "q[0]", "q[1]", "register[index]"] {
        assert_not_literal_without_consumption(source);
    }
}

// ---------------------------------------------------------------------------
// Long streams and deterministic repeated scanning
// ---------------------------------------------------------------------------

#[test]
fn many_adjacent_literals_can_be_scanned_without_a_fixed_count_limit() {
    let source = "|0⟩".repeat(1024);
    let mut cursor = Cursor::new(&source);
    let mut count = 0usize;

    while !cursor.is_eof() {
        let before = cursor.byte_offset();

        let literal = scan_quantum_literal(&mut cursor)
            .expect("repeated literal scan should not fail")
            .expect("each concatenated segment is a valid literal");

        assert_eq!(literal.symbol(), QuantumStateSymbol::Zero);
        assert_eq!(literal.byte_len(), "|0⟩".len());
        assert!(
            cursor.byte_offset() > before,
            "successful recognition must make forward progress"
        );

        count += 1;
    }

    assert_eq!(count, 1024);
    assert_eq!(cursor.byte_offset(), source.len());
}

#[test]
fn mixed_literal_stream_is_repeatable() {
    let source = "|0⟩|1⟩|+⟩|-⟩|1⟩|0⟩";

    fn scan_all(source: &str) -> Vec<(QuantumStateSymbol, std::ops::Range<usize>)> {
        let mut cursor = Cursor::new(source);
        let mut results = Vec::new();

        while !cursor.is_eof() {
            let literal = scan_quantum_literal(&mut cursor)
                .expect("literal stream should scan")
                .expect("stream contains only valid literals");

            results.push((literal.symbol(), literal.byte_range()));
        }

        results
    }

    let first = scan_all(source);

    for _ in 0..8 {
        assert_eq!(scan_all(source), first);
    }
}

#[test]
fn literal_error_types_implement_standard_error_contracts() {
    fn assert_error<E: std::error::Error>() {}

    assert_error::<QuantumLiteralError>();
    assert_error::<QuantumLiteralErrorKind>();
}

// Keep the recognized result type referenced explicitly so this test module
// guards the public result contract against accidental API removal.
#[allow(dead_code)]
fn quantum_literal_result_contract(
    literal: QuantumLiteral,
) -> (QuantumStateSymbol, std::ops::Range<usize>, usize) {
    (
        literal.symbol(),
        literal.byte_range(),
        literal.byte_len(),
    )
}
