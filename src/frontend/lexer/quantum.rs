
#![forbid(unsafe_code)]

//! Canonical quantum-literal recognizer for the Zamani frontend.
//!
//! File: src/frontend/lexer/quantum.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Normative lexical authority
//!
//! - `grammar/lexer/quantum-literals.g4`
//! - `grammar/lexer/literals.g4`
//! - `grammar/spec/lexical.md`
//!
//! # Scope
//!
//! This module recognizes only the compact quantum-state literal forms
//! defined by the current grammar:
//!
//! - `|0⟩`
//! - `|1⟩`
//! - `|+⟩`
//! - `|-⟩`
//!
//! It deliberately does not recognize quantum gates, qubit identifiers,
//! arbitrary symbolic states, circuits, observables, or hardware constructs.
//! Those belong to the appropriate grammar, parser, semantic, and IR layers.
//!
//! # Integration
//!
//! - `cursor.rs` owns source traversal and source positions.
//! - `config.rs` owns `ResourceLimit`.
//! - `token_kind.rs` owns `TokenKind::QuantumLiteral`.
//! - `token.rs` owns the emitted token representation.
//! - `scanner.rs` invokes this recognizer before ordinary operator matching.
//! - `diagnostics.rs` and `recovery.rs` own diagnostic presentation and
//!   recovery policy.
//! - `quantum::ir` remains the downstream quantum semantic boundary.
//!
//! # Scalability and portability
//!
//! This module imposes no limit on qubit count, circuit size, number of
//! literals, state dimension, hardware capacity, or execution resources.
//! The four symbols below are the specified vocabulary of this particular
//! literal notation, not a limit on the quantum operations or resources
//! available to Zamani programs.
//!
//! All source spelling is preserved. No hardware discovery, target selection,
//! quantum-state construction, normalization, or execution occurs here.

use core::fmt;
use core::ops::Range;

use super::config::ResourceLimit;
use super::cursor::{Cursor, CursorPosition};

/// The compact state symbol encoded by a canonical quantum-state literal.
///
/// This finite set mirrors the current normative literal grammar.
/// It does not enumerate quantum operations or hardware capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantumStateSymbol {
    /// `|0⟩`
    Zero,

    /// `|1⟩`
    One,

    /// `|+⟩`
    Plus,

    /// `|-⟩`
    Minus,
}

impl QuantumStateSymbol {
    /// Returns the symbol between the opening bar and closing ket.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Plus => '+',
            Self::Minus => '-',
        }
    }

    /// Returns the canonical spelling of the complete literal.
    #[must_use]
    pub const fn canonical_spelling(self) -> &'static str {
        match self {
            Self::Zero => "|0⟩",
            Self::One => "|1⟩",
            Self::Plus => "|+⟩",
            Self::Minus => "|-⟩",
        }
    }
}

/// A successfully recognized compact quantum-state literal.
///
/// The spelling is borrowed from the original source. The recognizer does
/// not allocate a second copy of the literal or construct a quantum state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumLiteral {
    symbol: QuantumStateSymbol,
    start: CursorPosition,
    end: CursorPosition,
}

impl QuantumLiteral {
    /// Creates a recognized literal result.
    ///
    /// This constructor is primarily intended for tests and adapters.
    /// Callers constructing a value directly must ensure its positions
    /// accurately describe a valid canonical literal.
    #[must_use]
    pub const fn new(
        symbol: QuantumStateSymbol,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Self {
        Self { symbol, start, end }
    }

    /// Returns the compact state symbol.
    #[must_use]
    pub const fn symbol(self) -> QuantumStateSymbol {
        self.symbol
    }

    /// Returns the beginning of the literal.
    #[must_use]
    pub const fn start(self) -> CursorPosition {
        self.start
    }

    /// Returns the position immediately after the literal.
    #[must_use]
    pub const fn end(self) -> CursorPosition {
        self.end
    }

    /// Returns the half-open UTF-8 byte range in the source.
    #[must_use]
    pub const fn byte_range(self) -> Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }

    /// Returns the UTF-8 byte length of the literal.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        self.end.byte_offset() - self.start.byte_offset()
    }

    /// Returns the exact source spelling from the cursor's source.
    ///
    /// The cursor may have advanced since this literal was scanned; slicing
    /// is based on the stored absolute source positions.
    pub fn spelling<'src>(
        self,
        cursor: &Cursor<'src>,
    ) -> Result<&'src str, super::cursor::CursorError> {
        cursor.slice(self.byte_range())
    }
}

/// The category of quantum-literal recognition failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum QuantumLiteralErrorKind {
    /// A recognized literal exceeds the configured UTF-8 byte budget.
    ByteBudgetExceeded,

    /// The cursor did not advance to the expected source position.
    PositionInvariant,
}

impl fmt::Display for QuantumLiteralErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ByteBudgetExceeded => {
                formatter.write_str(
                    "quantum literal exceeds the configured byte budget",
                )
            }
            Self::PositionInvariant => {
                formatter.write_str(
                    "quantum literal source-position invariant failed",
                )
            }
        }
    }
}

/// A structured quantum-literal recognition error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumLiteralError {
    kind: QuantumLiteralErrorKind,
    start: CursorPosition,
    end: CursorPosition,
}

impl QuantumLiteralError {
    fn new(
        kind: QuantumLiteralErrorKind,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Self {
        Self { kind, start, end }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn kind(self) -> QuantumLiteralErrorKind {
        self.kind
    }

    /// Returns the literal's starting position.
    #[must_use]
    pub const fn start(self) -> CursorPosition {
        self.start
    }

    /// Returns the position reached when the error was detected.
    #[must_use]
    pub const fn end(self) -> CursorPosition {
        self.end
    }

    /// Returns the half-open UTF-8 byte range associated with the error.
    #[must_use]
    pub const fn byte_range(self) -> Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }
}

impl fmt::Display for QuantumLiteralError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {} (ended at {})",
            self.kind, self.start, self.end
        )
    }
}

impl std::error::Error for QuantumLiteralError {}

/// Recognizes a canonical compact quantum-state literal at the current
/// cursor position.
///
/// Returns `Ok(None)` without consuming input when the source does not begin
/// with one of the four literal spellings specified by the grammar.
///
/// On success, consumes exactly one literal and returns its source positions.
/// A successful recognition is lexical only; semantic validation happens
/// downstream.
///
/// This convenience function does not impose an explicit literal-size
/// budget. Use `scan_quantum_literal_with_limit` when the caller requires one.
pub fn scan_quantum_literal(
    cursor: &mut Cursor<'_>,
) -> Result<Option<QuantumLiteral>, QuantumLiteralError> {
    scan_quantum_literal_with_limit(cursor, ResourceLimit::Unlimited)
}

/// Recognizes a canonical compact quantum-state literal with a configurable
/// UTF-8 byte budget.
///
/// The budget is an implementation resource policy, not a language-level
/// limit on quantum state size, qubit count, or circuit complexity.
///
/// # Cursor behavior
///
/// - No recognized literal: input remains unchanged.
/// - Valid literal within budget: consumes the literal.
/// - Valid literal exceeding budget: consumes the literal and returns an
///   explicit budget error.
/// - Malformed notation: input remains unchanged so the canonical scanner
///   can classify the characters according to the full lexical grammar.
///
/// This recognizer intentionally does not classify malformed notation as a
/// valid quantum literal.
pub fn scan_quantum_literal_with_limit(
    cursor: &mut Cursor<'_>,
    byte_limit: ResourceLimit,
) -> Result<Option<QuantumLiteral>, QuantumLiteralError> {
    let spelling_and_symbol = if cursor.starts_with("|0⟩") {
        Some(("|0⟩", QuantumStateSymbol::Zero))
    } else if cursor.starts_with("|1⟩") {
        Some(("|1⟩", QuantumStateSymbol::One))
    } else if cursor.starts_with("|+⟩") {
        Some(("|+⟩", QuantumStateSymbol::Plus))
    } else if cursor.starts_with("|-⟩") {
        Some(("|-⟩", QuantumStateSymbol::Minus))
    } else {
        None
    };

    let Some((spelling, symbol)) = spelling_and_symbol else {
        return Ok(None);
    };

    let start = cursor.position();

    if !cursor.consume_str(spelling) {
        return Err(QuantumLiteralError::new(
            QuantumLiteralErrorKind::PositionInvariant,
            start,
            cursor.position(),
        ));
    }

    let end = cursor.position();
    let literal = QuantumLiteral::new(symbol, start, end);

    if !byte_limit.permits(literal.byte_len()) {
        return Err(QuantumLiteralError::new(
            QuantumLiteralErrorKind::ByteBudgetExceeded,
            start,
            end,
        ));
    }

    Ok(Some(literal))
}

#[cfg(test)]
mod tests {
    use super::{
        scan_quantum_literal, scan_quantum_literal_with_limit,
        QuantumLiteralErrorKind, QuantumStateSymbol,
    };
    use crate::frontend::lexer::config::ResourceLimit;
    use crate::frontend::lexer::cursor::Cursor;

    #[test]
    fn recognizes_zero_state() {
        assert_literal("|0⟩", QuantumStateSymbol::Zero);
    }

    #[test]
    fn recognizes_one_state() {
        assert_literal("|1⟩", QuantumStateSymbol::One);
    }

    #[test]
    fn recognizes_plus_state() {
        assert_literal("|+⟩", QuantumStateSymbol::Plus);
    }

    #[test]
    fn recognizes_minus_state() {
        assert_literal("|-⟩", QuantumStateSymbol::Minus);
    }

    fn assert_literal(source: &str, expected: QuantumStateSymbol) {
        let mut cursor = Cursor::new(source);

        let literal = scan_quantum_literal(&mut cursor)
            .expect("valid literal should not fail")
            .expect("literal should be recognized");

        assert_eq!(literal.symbol(), expected);
        assert_eq!(literal.byte_len(), source.len());
        assert_eq!(
            literal.spelling(&cursor).expect("valid source range"),
            source
        );
        assert!(cursor.is_eof());
    }

    #[test]
    fn rejects_unsupported_state_symbols_without_consuming_input() {
        for source in ["|2⟩", "|a⟩", "|00⟩", "|01⟩", "|0>", "|0"] {
            let mut cursor = Cursor::new(source);
            let before = cursor.position();

            let result = scan_quantum_literal(&mut cursor)
                .expect("unrecognized notation is not a recognizer error");

            assert!(result.is_none(), "unexpected literal: {source}");
            assert_eq!(
                cursor.position(),
                before,
                "malformed input was consumed: {source}"
            );
        }
    }

    #[test]
    fn does_not_recognize_a_bra_as_a_ket_literal() {
        let mut cursor = Cursor::new("⟨0|");

        assert!(
            scan_quantum_literal(&mut cursor)
                .expect("recognition should succeed")
                .is_none()
        );

        assert_eq!(cursor.byte_offset(), 0);
    }

    #[test]
    fn adjacent_literals_are_recognized_independently() {
        let mut cursor = Cursor::new("|0⟩|1⟩");

        let first = scan_quantum_literal(&mut cursor)
            .expect("first literal should scan")
            .expect("first literal should be recognized");
        let second = scan_quantum_literal(&mut cursor)
            .expect("second literal should scan")
            .expect("second literal should be recognized");

        assert_eq!(first.symbol(), QuantumStateSymbol::Zero);
        assert_eq!(second.symbol(), QuantumStateSymbol::One);
        assert!(cursor.is_eof());
    }

    #[test]
    fn following_source_is_not_consumed() {
        let mut cursor = Cursor::new("|0⟩ + value");

        let _ = scan_quantum_literal(&mut cursor)
            .expect("literal should scan")
            .expect("literal should be recognized");

        assert_eq!(cursor.remaining(), " + value");
    }

    #[test]
    fn byte_budget_accepts_exact_length() {
        let source = "|0⟩";
        let mut cursor = Cursor::new(source);
        let budget = ResourceLimit::at_most(source.len())
            .expect("literal length is a positive budget");

        let literal = scan_quantum_literal_with_limit(&mut cursor, budget)
            .expect("literal is within budget")
            .expect("literal should be recognized");

        assert_eq!(literal.byte_len(), source.len());
    }

    #[test]
    fn byte_budget_rejects_a_smaller_budget() {
        let source = "|0⟩";
        let mut cursor = Cursor::new(source);
        let budget = ResourceLimit::at_most(1)
            .expect("positive budget is valid");

        let error = scan_quantum_literal_with_limit(&mut cursor, budget)
            .expect_err("literal exceeds the configured budget");

        assert_eq!(
            error.kind(),
            QuantumLiteralErrorKind::ByteBudgetExceeded
        );
        assert!(cursor.is_eof());
    }

    #[test]
    fn unlimited_budget_accepts_literal() {
        let mut cursor = Cursor::new("|+⟩");

        let literal = scan_quantum_literal(&mut cursor)
            .expect("unlimited budget should accept the literal")
            .expect("literal should be recognized");

        assert_eq!(literal.symbol(), QuantumStateSymbol::Plus);
    }

    #[test]
    fn literal_byte_length_accounts_for_unicode_encoding() {
        let mut cursor = Cursor::new("|-⟩");

        let literal = scan_quantum_literal(&mut cursor)
            .expect("literal should scan")
            .expect("literal should be recognized");

        assert_eq!(literal.byte_len(), "|-⟩".len());
        assert!(literal.byte_len() > 3);
    }
}
