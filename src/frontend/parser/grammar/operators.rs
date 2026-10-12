
#![forbid(unsafe_code)]

//! Parser-side operator classification for Zamani.
//!
//! File: src/frontend/parser/grammar/operators.rs
//! Rust: 1.97 or later
//! Edition: 2021
//!
//! # Purpose
//!
//! Bridge canonical lexer tokens to the parser's operator-precedence
//! contract. This module allows grammar productions to ask whether a token
//! can act as a prefix, infix, postfix, or ternary operator without
//! duplicating token-spelling or precedence tables.
//!
//! # Authority
//!
//! - `grammar/lexer/operators.g4` owns lexical operator spellings.
//! - `grammar/lexer/operators.md` specifies lexical operator identity.
//! - `grammar/expressions/precedence.md` owns precedence and associativity.
//! - `crate::frontend::lexer::token_kind::TokenKind` owns token identities.
//! - `crate::frontend::parser::precedence::PrecedenceTable` supplies the
//!   canonical and extension-defined precedence entries.
//! - Expression productions decide where each operator is syntactically
//!   valid.
//! - Semantic analysis determines operator meaning and type legality.
//!
//! # Does not own
//!
//! - Lexical scanning or operator-spelling registration.
//! - A second operator spelling-to-token mapping.
//! - The canonical precedence hierarchy.
//! - Expression parsing or AST construction.
//! - Type checking, overload resolution, or effect analysis.
//! - Quantum, HDL, hardware, runtime, or backend semantics.
//! - Target discovery or resource allocation.
//!
//! # Integration contract
//!
//! `grammar/expressions.rs` uses `OperatorCandidates` to inspect the
//! available fixities for the current token.
//!
//! `precedence.rs` remains the sole owner of binding powers and
//! associativity. The caller supplies the active table so language profiles
//! and approved extensions can use their configured definitions.
//!
//! The `spelling` argument must be the original token lexeme. It is passed
//! through to `PrecedenceTable::lookup`, which prefers an exact
//! spelling-specific extension entry before the token-kind fallback.
//!
//! This module does not infer an operator from arbitrary source text.
//! A spelling must already have been tokenized by the canonical lexer.
//!
//! # Scalability and safety
//!
//! There is no fixed operator-count limit here. Operator definitions live
//! in the precedence table and can be extended under that table's
//! registration policy. This module defines no hardware-capacity limits.
//!
//! No unsafe Rust is used.

use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::precedence::{
    Associativity, BindingPower, OperatorFixity, OperatorSpec, PrecedenceLevel,
    PrecedenceTable,
};

/// The syntactic roles that a token can occupy.
///
/// A single token kind can support multiple roles. For example, `+` can
/// be prefix addition or infix addition. The active precedence table, not
/// this module, determines which roles are registered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorCandidates<'a> {
    prefix: Option<&'a OperatorSpec>,
    infix: Option<&'a OperatorSpec>,
    postfix: Option<&'a OperatorSpec>,
    ternary: Option<&'a OperatorSpec>,
}

impl<'a> OperatorCandidates<'a> {
    /// Looks up all supported syntactic roles for a canonical token.
    ///
    /// `spelling` must be the exact lexeme from the original token.
    /// Exact spelling-specific definitions take precedence over fallback
    /// definitions according to `PrecedenceTable::lookup`.
    #[must_use]
    pub fn new(
        kind: TokenKind,
        spelling: &str,
        table: &'a PrecedenceTable,
    ) -> Self {
        Self {
            prefix: table.lookup(kind, OperatorFixity::Prefix, spelling),
            infix: table.lookup(kind, OperatorFixity::Infix, spelling),
            postfix: table.lookup(kind, OperatorFixity::Postfix, spelling),
            ternary: table.lookup(kind, OperatorFixity::Ternary, spelling),
        }
    }

    /// Returns the prefix definition, if registered.
    #[must_use]
    pub const fn prefix(self) -> Option<&'a OperatorSpec> {
        self.prefix
    }

    /// Returns the infix definition, if registered.
    #[must_use]
    pub const fn infix(self) -> Option<&'a OperatorSpec> {
        self.infix
    }

    /// Returns the postfix definition, if registered.
    #[must_use]
    pub const fn postfix(self) -> Option<&'a OperatorSpec> {
        self.postfix
    }

    /// Returns the ternary definition, if registered.
    #[must_use]
    pub const fn ternary(self) -> Option<&'a OperatorSpec> {
        self.ternary
    }

    /// Returns whether at least one operator role is registered.
    #[must_use]
    pub const fn has_any_role(self) -> bool {
        self.prefix.is_some()
            || self.infix.is_some()
            || self.postfix.is_some()
            || self.ternary.is_some()
    }

    /// Returns whether the token is registered as a binary/infix operator.
    #[must_use]
    pub const fn is_infix(self) -> bool {
        self.infix.is_some()
    }

    /// Returns whether the token is registered as a prefix operator.
    #[must_use]
    pub const fn is_prefix(self) -> bool {
        self.prefix.is_some()
    }

    /// Returns whether the token is registered as a postfix operator.
    #[must_use]
    pub const fn is_postfix(self) -> bool {
        self.postfix.is_some()
    }

    /// Returns whether the token introduces a ternary expression.
    #[must_use]
    pub const fn is_ternary(self) -> bool {
        self.ternary.is_some()
    }
}

/// Looks up one operator definition by token identity, exact spelling,
/// and syntactic fixity.
///
/// This function deliberately does not require `TokenKind::is_operator()`:
/// some grammar-level operators or introducers may have a token identity
/// classified as punctuation by the current lexer. The precedence table
/// remains the authority for whether a syntactic role is actually defined.
///
/// A missing entry means that the requested role is not registered; it
/// does not mean the token is lexically invalid.
#[must_use]
pub fn lookup_operator<'a>(
    kind: TokenKind,
    spelling: &str,
    fixity: OperatorFixity,
    table: &'a PrecedenceTable,
) -> Option<&'a OperatorSpec> {
    table.lookup(kind, fixity, spelling)
}

/// Returns the canonical precedence level for a registered operator role.
///
/// This helper is intentionally metadata-only. It does not determine
/// whether the operator is legal in the current grammar production.
#[must_use]
pub fn operator_precedence(spec: &OperatorSpec) -> PrecedenceLevel {
    spec.precedence()
}

/// Returns the associativity defined for a registered operator role.
#[must_use]
pub fn operator_associativity(spec: &OperatorSpec) -> Associativity {
    spec.associativity()
}

/// Returns the Pratt binding powers for an infix or ternary operator.
///
/// Prefix and postfix operators use their dedicated binding-power
/// accessors on `OperatorSpec`.
#[must_use]
pub fn operator_binding_power(spec: &OperatorSpec) -> Option<BindingPower> {
    spec.binding_power()
}

/// Returns the operand binding power for a registered prefix operator.
#[must_use]
pub fn prefix_operand_binding_power(spec: &OperatorSpec) -> Option<u16> {
    spec.prefix_operand_binding_power()
}

/// Returns the binding power for a registered postfix operator.
#[must_use]
pub fn postfix_binding_power(spec: &OperatorSpec) -> Option<u16> {
    spec.postfix_binding_power()
}

/// Returns whether a registered infix definition is an assignment operator.
///
/// This is based on the configured precedence definition rather than a
/// second hard-coded list of assignment token kinds.
#[must_use]
pub fn is_assignment(spec: &OperatorSpec) -> bool {
    spec.fixity() == OperatorFixity::Infix
        && spec.precedence() == PrecedenceLevel::Assignment
}

/// Returns whether a registered infix definition forbids unparenthesized
/// same-precedence chaining.
///
/// The expression parser is responsible for enforcing this rule while
/// parsing a complete expression.
#[must_use]
pub fn is_non_associative(spec: &OperatorSpec) -> bool {
    spec.fixity() == OperatorFixity::Infix
        && spec.associativity() == Associativity::NonAssociative
}

/// Returns the active standard operator-precedence table.
///
/// This convenience constructor delegates all operator definitions to
/// `PrecedenceTable::standard()`. Parsers using dialects or extensions
/// should instead retain and pass their configured table.
#[must_use]
pub fn standard_precedence() -> PrecedenceTable {
    PrecedenceTable::standard()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_has_prefix_and_infix_roles() {
        let table = PrecedenceTable::standard();
        let roles = OperatorCandidates::new(TokenKind::Plus, "+", &table);

        assert!(roles.is_prefix());
        assert!(roles.is_infix());
        assert!(!roles.is_postfix());
        assert!(!roles.is_ternary());

        let prefix = roles.prefix().expect("prefix plus must be registered");
        assert_eq!(prefix.precedence(), PrecedenceLevel::Prefix);
        assert_eq!(
            prefix.associativity(),
            Associativity::Right
        );

        let infix = roles.infix().expect("infix plus must be registered");
        assert_eq!(infix.precedence(), PrecedenceLevel::Additive);
        assert_eq!(infix.associativity(), Associativity::Left);
    }

    #[test]
    fn assignment_is_right_associative() {
        let table = PrecedenceTable::standard();
        let spec = lookup_operator(
            TokenKind::Assign,
            "=",
            OperatorFixity::Infix,
            &table,
        )
        .expect("assignment must be registered");

        assert!(is_assignment(spec));
        assert_eq!(spec.associativity(), Associativity::Right);

        let powers = operator_binding_power(spec)
            .expect("infix assignment must have binding powers");
        assert_eq!(powers.left(), powers.right());
    }

    #[test]
    fn arithmetic_binding_powers_follow_precedence_order() {
        let table = PrecedenceTable::standard();

        let additive = lookup_operator(
            TokenKind::Plus,
            "+",
            OperatorFixity::Infix,
            &table,
        )
        .expect("addition must be registered");

        let multiplicative = lookup_operator(
            TokenKind::Star,
            "*",
            OperatorFixity::Infix,
            &table,
        )
        .expect("multiplication must be registered");

        let additive_power =
            operator_binding_power(additive).expect("infix binding power");
        let multiplicative_power =
            operator_binding_power(multiplicative).expect("infix binding power");

        assert!(additive_power.left() < multiplicative_power.left());
        assert!(additive_power.right() < multiplicative_power.right());
    }

    #[test]
    fn comparisons_are_non_associative() {
        let table = PrecedenceTable::standard();

        for (kind, spelling) in [
            (TokenKind::Equals, "=="),
            (TokenKind::NotEquals, "!="),
            (TokenKind::LessThan, "<"),
            (TokenKind::LessThanOrEqual, "<="),
            (TokenKind::GreaterThan, ">"),
            (TokenKind::GreaterThanOrEqual, ">="),
        ] {
            let spec = lookup_operator(
                kind,
                spelling,
                OperatorFixity::Infix,
                &table,
            )
            .expect("comparison must be registered");

            assert!(is_non_associative(spec), "{spelling}");
        }
    }

    #[test]
    fn ranges_are_non_associative() {
        let table = PrecedenceTable::standard();

        for (kind, spelling) in [
            (TokenKind::Range, ".."),
            (TokenKind::RangeInclusive, "..="),
        ] {
            let spec = lookup_operator(
                kind,
                spelling,
                OperatorFixity::Infix,
                &table,
            )
            .expect("range operator must be registered");

            assert!(is_non_associative(spec));
            assert_eq!(spec.precedence(), PrecedenceLevel::Range);
        }
    }

    #[test]
    fn conditional_introducer_is_ternary_not_infix() {
        let table = PrecedenceTable::standard();
        let roles = OperatorCandidates::new(TokenKind::Question, "?", &table);

        assert!(roles.is_ternary());
        assert!(!roles.is_infix());

        let spec = roles.ternary().expect("conditional must be registered");
        assert_eq!(spec.precedence(), PrecedenceLevel::Conditional);
        assert_eq!(spec.associativity(), Associativity::Right);
    }

    #[test]
    fn unknown_role_is_not_guessed_from_spelling() {
        let table = PrecedenceTable::standard();

        let roles =
            OperatorCandidates::new(TokenKind::Identifier, "custom_op", &table);

        assert!(!roles.has_any_role());
    }

    #[test]
    fn token_can_be_checked_without_assuming_its_lexical_category() {
        // The lookup contract is table-driven. It does not independently
        // reject tokens based on their lexical category.
        let table = PrecedenceTable::standard();

        let spec = lookup_operator(
            TokenKind::Question,
            "?",
            OperatorFixity::Ternary,
            &table,
        );

        assert!(spec.is_some());
    }
}
