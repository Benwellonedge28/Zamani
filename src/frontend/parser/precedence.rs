
#![forbid(unsafe_code)]

//! Canonical operator precedence and associativity for the Zamani parser.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Authority
//!
//! `grammar/expressions/precedence.md` is the normative specification for
//! expression precedence and associativity. This module implements its
//! canonical hierarchy; it does not replace that specification.
//!
//! # Responsibilities
//!
//! - Define the canonical precedence levels.
//! - Represent operator fixity and associativity.
//! - Provide the standard precedence table.
//! - Support spelling-specific precedence entries for approved extensions.
//! - Reject conflicting registrations deterministically.
//! - Provide binding powers for Pratt/precedence-climbing parsers.
//! - Keep parsing precedence independent of semantic interpretation.
//!
//! # Does not own
//!
//! - Token identities or lexical recognition.
//! - Operator spelling-to-token mappings.
//! - Grammar productions or expression parsing.
//! - AST construction, type checking, or overload resolution.
//! - Quantum, HDL, hardware, or execution semantics.
//! - Target selection, resource allocation, or scheduling.
//!
//! # Integration
//!
//! - `crate::frontend::lexer::token_kind::TokenKind` is the canonical token
//!   identity used by this table.
//! - `grammar/operators.rs` maps lexer tokens and lexemes to operator roles.
//! - `grammar/expressions.rs` uses binding powers to construct the required
//!   expression grouping.
//! - `extensions.rs` and `registry.rs` may provide approved extension
//!   definitions; they must not silently override existing definitions.
//! - `ast_adapter.rs` preserves the grouping selected by the parser.
//! - Semantic analysis determines the meaning and validity of operators.
//!
//! This file deliberately does not import parser modules that depend on it.
//! That keeps the precedence contract independently compilable and avoids
//! circular module dependencies.
//!
//! # Scalability and safety
//!
//! The table has no fixed operator-count limit. Its storage grows with the
//! registered definitions and is subject to ordinary host resource limits.
//! It defines no limits on operands, registers, qubits, tensor dimensions,
//! devices, or computing capacity. Parser safety budgets belong in the
//! parser's configurable resource-limit layer.
//!
//! No unsafe Rust is used.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::lexer::token_kind::TokenKind;

/// Canonical expression precedence, from weakest to strongest binding.
///
/// The numeric ranks are an implementation of the normative ordering in
/// `grammar/expressions/precedence.md`. They are not serialized IDs and
/// must not be interpreted as machine widths or resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PrecedenceLevel {
    Assignment = 1,
    Conditional = 2,
    Range = 3,
    LogicalOr = 4,
    LogicalAnd = 5,
    BitwiseOr = 6,
    BitwiseXor = 7,
    BitwiseAnd = 8,
    Equality = 9,
    Relational = 10,
    Shift = 11,
    Additive = 12,
    Multiplicative = 13,
    Prefix = 14,
    Postfix = 15,
    Primary = 16,
}

impl PrecedenceLevel {
    /// Returns the numeric rank used by the Pratt binding-power interface.
    ///
    /// Larger values bind more tightly. This rank is local to the parser's
    /// precedence contract and is not a stable external serialization value.
    #[must_use]
    pub const fn rank(self) -> u16 {
        self as u16
    }
}

/// Syntactic position in which an operator may appear.
///
/// One token kind can have more than one fixity. For example, `-` may be
/// prefix negation or infix subtraction. The grammar decides which role is
/// valid at a particular source position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatorFixity {
    Prefix,
    Infix,
    Postfix,
    /// A three-operand form such as `condition ? then_value : else_value`.
    ///
    /// The token represented by the key is the conditional introducer.
    /// Parsing the separator and branches remains the grammar's job.
    Ternary,
}

/// Grouping rule for operators at the same precedence level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Associativity {
    /// Group from left to right: `a - b - c` means `(a - b) - c`.
    Left,
    /// Group from right to left: `a = b = c` means `a = (b = c)`.
    Right,
    /// A second same-level operator must be rejected unless the grammar
    /// explicitly defines a chained form.
    NonAssociative,
    /// Used for forms, such as postfix operators, whose associativity is
    /// governed by their grammar production rather than binary grouping.
    None,
}

/// Binding powers for a Pratt or precedence-climbing parser.
///
/// For left-associative operators, `left` is the operator rank and `right`
/// is the next rank. For right-associative operators, both are the operator
/// rank. Non-associative operators use the left-associative pair but require
/// the expression parser to reject an unparenthesized same-level chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingPower {
    left: u16,
    right: u16,
}

impl BindingPower {
    /// Creates a binding-power pair.
    ///
    /// This constructor intentionally permits arbitrary ranks so that
    /// extensions can use their validated precedence contract. Use
    /// `OperatorSpec::new` to derive a pair from a standard precedence level.
    #[must_use]
    pub const fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }

    /// Returns the left binding power.
    #[must_use]
    pub const fn left(self) -> u16 {
        self.left
    }

    /// Returns the right binding power.
    #[must_use]
    pub const fn right(self) -> u16 {
        self.right
    }
}

/// Identity of a precedence entry.
///
/// An entry with `spelling: None` is the fallback for a token kind and fixity.
/// An entry with `Some(spelling)` applies only to that exact spelling and
/// takes precedence over the fallback entry during lookup.
///
/// Spelling-specific entries let an approved extension use an existing
/// lexical token kind without forcing the core parser to enumerate every
/// extension operator. The lexer remains responsible for deciding which
/// token kind a spelling produces.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperatorKey {
    kind: TokenKind,
    fixity: OperatorFixity,
    spelling: Option<String>,
}

impl OperatorKey {
    /// Creates a token-kind-wide key.
    #[must_use]
    pub const fn by_kind(kind: TokenKind, fixity: OperatorFixity) -> Self {
        Self {
            kind,
            fixity,
            spelling: None,
        }
    }

    /// Creates a spelling-specific key.
    pub fn by_spelling(
        kind: TokenKind,
        fixity: OperatorFixity,
        spelling: impl Into<String>,
    ) -> Result<Self, PrecedenceError> {
        let spelling = spelling.into();
        validate_spelling(&spelling)?;

        Ok(Self {
            kind,
            fixity,
            spelling: Some(spelling),
        })
    }

    /// Returns the canonical lexer token kind.
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the syntactic fixity.
    #[must_use]
    pub const fn fixity(&self) -> OperatorFixity {
        self.fixity
    }

    /// Returns the exact spelling for an extension entry, if present.
    #[must_use]
    pub fn spelling(&self) -> Option<&str> {
        self.spelling.as_deref()
    }
}

/// One precedence and associativity definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorSpec {
    key: OperatorKey,
    precedence: PrecedenceLevel,
    associativity: Associativity,
}

impl OperatorSpec {
    /// Creates a token-kind-wide precedence definition.
    pub fn new(
        kind: TokenKind,
        fixity: OperatorFixity,
        precedence: PrecedenceLevel,
        associativity: Associativity,
    ) -> Result<Self, PrecedenceError> {
        Self::from_key(
            OperatorKey::by_kind(kind, fixity),
            precedence,
            associativity,
        )
    }

    /// Creates a precedence definition for one exact operator spelling.
    pub fn for_spelling(
        kind: TokenKind,
        fixity: OperatorFixity,
        spelling: impl Into<String>,
        precedence: PrecedenceLevel,
        associativity: Associativity,
    ) -> Result<Self, PrecedenceError> {
        Self::from_key(
            OperatorKey::by_spelling(kind, fixity, spelling)?,
            precedence,
            associativity,
        )
    }

    fn from_key(
        key: OperatorKey,
        precedence: PrecedenceLevel,
        associativity: Associativity,
    ) -> Result<Self, PrecedenceError> {
        validate_associativity(key.fixity(), associativity)?;

        Ok(Self {
            key,
            precedence,
            associativity,
        })
    }

    /// Returns this entry's key.
    #[must_use]
    pub fn key(&self) -> &OperatorKey {
        &self.key
    }

    /// Returns the token kind.
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.key.kind()
    }

    /// Returns the fixity.
    #[must_use]
    pub const fn fixity(&self) -> OperatorFixity {
        self.key.fixity()
    }

    /// Returns the precedence level.
    #[must_use]
    pub const fn precedence(&self) -> PrecedenceLevel {
        self.precedence
    }

    /// Returns the associativity.
    #[must_use]
    pub const fn associativity(&self) -> Associativity {
        self.associativity
    }

    /// Returns Pratt binding powers for infix and ternary operators.
    ///
    /// Prefix parsing should use `prefix_operand_binding_power`; postfix
    /// parsing should use `postfix_binding_power`. Primary expressions are
    /// parsed by the primary grammar and have no operator binding pair.
    #[must_use]
    pub const fn binding_power(&self) -> Option<BindingPower> {
        let rank = self.precedence.rank();

        match self.key.fixity() {
            OperatorFixity::Infix | OperatorFixity::Ternary => {
                match self.associativity {
                    Associativity::Right => Some(BindingPower::new(rank, rank)),
                    Associativity::Left | Associativity::NonAssociative => {
                        Some(BindingPower::new(rank, rank + 1))
                    }
                    Associativity::None => None,
                }
            }
            OperatorFixity::Prefix | OperatorFixity::Postfix => None,
        }
    }

    /// Returns the minimum binding power for the operand of a prefix
    /// operator, or `None` when this definition is not a prefix operator.
    #[must_use]
    pub const fn prefix_operand_binding_power(&self) -> Option<u16> {
        if matches!(self.key.fixity(), OperatorFixity::Prefix) {
            Some(self.precedence.rank())
        } else {
            None
        }
    }

    /// Returns the binding power used by a postfix parse loop, or `None`
    /// when this definition is not a postfix operator.
    #[must_use]
    pub const fn postfix_binding_power(&self) -> Option<u16> {
        if matches!(self.key.fixity(), OperatorFixity::Postfix) {
            Some(self.precedence.rank())
        } else {
            None
        }
    }
}

/// Errors produced while building or extending a precedence table.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrecedenceError {
    /// An extension supplied an empty or whitespace-containing spelling.
    InvalidSpelling {
        spelling: String,
        reason: &'static str,
    },
    /// A key has already been registered.
    DuplicateDefinition {
        key: OperatorKey,
    },
    /// The associativity is not valid for this syntactic fixity.
    InvalidAssociativity {
        fixity: OperatorFixity,
        associativity: Associativity,
    },
}

impl fmt::Display for PrecedenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpelling { spelling, reason } => {
                write!(f, "invalid operator spelling {spelling:?}: {reason}")
            }
            Self::DuplicateDefinition { key } => {
                write!(
                    f,
                    "duplicate precedence definition for token {:?}, fixity {:?}, spelling {:?}",
                    key.kind(),
                    key.fixity(),
                    key.spelling()
                )
            }
            Self::InvalidAssociativity {
                fixity,
                associativity,
            } => write!(
                f,
                "associativity {associativity:?} is invalid for {fixity:?} operators"
            ),
        }
    }
}

impl Error for PrecedenceError {}

/// Ordered, deterministic precedence definitions.
///
/// Construct a standard table with [`PrecedenceTable::standard`], then
/// register approved spelling-specific or token-kind-wide extension entries
/// before sharing the table immutably with parse sessions.
///
/// Lookup first checks the exact `(token kind, fixity, spelling)` key and
/// then falls back to `(token kind, fixity, no spelling)`.
#[derive(Debug, Clone, Default)]
pub struct PrecedenceTable {
    entries: BTreeMap<OperatorKey, OperatorSpec>,
}

impl PrecedenceTable {
    /// Creates an empty table.
    ///
    /// Use this for custom language profiles. Most parser configurations
    /// should begin with `standard()` and extend it deliberately.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the standard Zamani operator precedence table.
    ///
    /// The levels and associativity follow
    /// `grammar/expressions/precedence.md`.
    ///
    /// The returned table includes only operator forms represented by the
    /// current canonical `TokenKind` inventory. Calls, indexing, and member
    /// access are grammar-level postfix productions and use the shared
    /// `POSTFIX_PRECEDENCE` constant instead of pretending they are tokens.
    #[must_use]
    pub fn standard() -> Self {
        let mut table = Self::new();

        // Assignment: right associative.
        for kind in [
            TokenKind::Assign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::StarAssign,
            TokenKind::SlashAssign,
            TokenKind::ModuloAssign,
        ] {
            table.insert_standard(kind, OperatorFixity::Infix,
                PrecedenceLevel::Assignment, Associativity::Right);
        }

        // Ternary conditional introducer. The grammar owns the separator
        // and the branch parsing; this entry only provides its precedence.
        table.insert_standard(
            TokenKind::Question,
            OperatorFixity::Ternary,
            PrecedenceLevel::Conditional,
            Associativity::Right,
        );

        // Ranges are non-associative unless the normative grammar explicitly
        // introduces a chained-range form.
        for kind in [TokenKind::Range, TokenKind::RangeInclusive] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Range,
                Associativity::NonAssociative,
            );
        }

        table.insert_standard(
            TokenKind::LogicalOr,
            OperatorFixity::Infix,
            PrecedenceLevel::LogicalOr,
            Associativity::Left,
        );
        table.insert_standard(
            TokenKind::LogicalAnd,
            OperatorFixity::Infix,
            PrecedenceLevel::LogicalAnd,
            Associativity::Left,
        );
        table.insert_standard(
            TokenKind::BitOr,
            OperatorFixity::Infix,
            PrecedenceLevel::BitwiseOr,
            Associativity::Left,
        );
        table.insert_standard(
            TokenKind::BitXor,
            OperatorFixity::Infix,
            PrecedenceLevel::BitwiseXor,
            Associativity::Left,
        );
        table.insert_standard(
            TokenKind::BitAnd,
            OperatorFixity::Infix,
            PrecedenceLevel::BitwiseAnd,
            Associativity::Left,
        );

        for kind in [TokenKind::Equals, TokenKind::NotEquals] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Equality,
                Associativity::NonAssociative,
            );
        }

        for kind in [
            TokenKind::LessThan,
            TokenKind::LessThanOrEqual,
            TokenKind::GreaterThan,
            TokenKind::GreaterThanOrEqual,
        ] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Relational,
                Associativity::NonAssociative,
            );
        }

        for kind in [TokenKind::LeftShift, TokenKind::RightShift] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Shift,
                Associativity::Left,
            );
        }

        for kind in [TokenKind::Plus, TokenKind::Minus] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Additive,
                Associativity::Left,
            );
        }

        for kind in [TokenKind::Star, TokenKind::Slash, TokenKind::Modulo] {
            table.insert_standard(
                kind,
                OperatorFixity::Infix,
                PrecedenceLevel::Multiplicative,
                Associativity::Left,
            );
        }

        // Prefix and infix roles may coexist for the same token kind.
        for kind in [TokenKind::Plus, TokenKind::Minus] {
            table.insert_standard(
                kind,
                OperatorFixity::Prefix,
                PrecedenceLevel::Prefix,
                Associativity::Right,
            );
        }

        for kind in [TokenKind::LogicalNot, TokenKind::BitNot] {
            table.insert_standard(
                kind,
                OperatorFixity::Prefix,
                PrecedenceLevel::Prefix,
                Associativity::Right,
            );
        }

        table
    }

    fn insert_standard(
        &mut self,
        kind: TokenKind,
        fixity: OperatorFixity,
        precedence: PrecedenceLevel,
        associativity: Associativity,
    ) {
        // All standard definitions are compile-time-reviewed entries. Their
        // keys are distinct by token kind/fixity and their combinations are
        // validated by the public constructors and tests.
        let key = OperatorKey::by_kind(kind, fixity);
        let spec = OperatorSpec {
            key: key.clone(),
            precedence,
            associativity,
        };
        self.entries.insert(key, spec);
    }

    /// Registers a precedence definition.
    ///
    /// Registration is atomic: on conflict, the table is unchanged.
    /// Existing definitions are never silently replaced.
    pub fn register(&mut self, spec: OperatorSpec) -> Result<(), PrecedenceError> {
        let key = spec.key.clone();

        if self.entries.contains_key(&key) {
            return Err(PrecedenceError::DuplicateDefinition { key });
        }

        self.entries.insert(key, spec);
        Ok(())
    }

    /// Looks up an operator definition by token kind, fixity and source
    /// spelling. Exact spelling-specific entries take priority over the
    /// token-kind-wide fallback.
    #[must_use]
    pub fn lookup(
        &self,
        kind: TokenKind,
        fixity: OperatorFixity,
        spelling: &str,
    ) -> Option<&OperatorSpec> {
        let exact = OperatorKey {
            kind,
            fixity,
            spelling: Some(spelling.to_owned()),
        };

        self.entries.get(&exact).or_else(|| {
            self.entries.get(&OperatorKey::by_kind(kind, fixity))
        })
    }

    /// Looks up the fallback entry for a token kind and fixity, ignoring
    /// source spelling.
    #[must_use]
    pub fn lookup_kind(
        &self,
        kind: TokenKind,
        fixity: OperatorFixity,
    ) -> Option<&OperatorSpec> {
        self.entries.get(&OperatorKey::by_kind(kind, fixity))
    }

    /// Returns whether an exact key is registered.
    #[must_use]
    pub fn contains(&self, key: &OperatorKey) -> bool {
        self.entries.contains_key(key)
    }

    /// Returns the number of registered precedence definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the table has no definitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates definitions in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = &OperatorSpec> {
        self.entries.values()
    }
}

/// Canonical binding power for grammar-level postfix operations.
///
/// Calls, indexing, member access and other postfix productions may not each
/// have a dedicated token kind. Their shared binding level is therefore
/// available independently of the operator-token table.
pub const POSTFIX_PRECEDENCE: u16 = PrecedenceLevel::Postfix as u16;

/// Canonical binding power for primary/atomic expressions.
pub const PRIMARY_PRECEDENCE: u16 = PrecedenceLevel::Primary as u16;

/// Validates extension spellings without imposing a fixed spelling length.
fn validate_spelling(spelling: &str) -> Result<(), PrecedenceError> {
    if spelling.is_empty() {
        return Err(PrecedenceError::InvalidSpelling {
            spelling: spelling.to_owned(),
            reason: "spelling must not be empty",
        });
    }

    if spelling.trim() != spelling {
        return Err(PrecedenceError::InvalidSpelling {
            spelling: spelling.to_owned(),
            reason: "leading or trailing whitespace is not allowed",
        });
    }

    if spelling.chars().any(char::is_whitespace) {
        return Err(PrecedenceError::InvalidSpelling {
            spelling: spelling.to_owned(),
            reason: "whitespace is not allowed inside an operator spelling",
        });
    }

    if spelling.chars().any(char::is_control) {
        return Err(PrecedenceError::InvalidSpelling {
            spelling: spelling.to_owned(),
            reason: "control characters are not allowed",
        });
    }

    Ok(())
}

fn validate_associativity(
    fixity: OperatorFixity,
    associativity: Associativity,
) -> Result<(), PrecedenceError> {
    let valid = match fixity {
        OperatorFixity::Prefix => associativity == Associativity::Right,
        OperatorFixity::Postfix => {
            matches!(associativity, Associativity::Left | Associativity::None)
        }
        OperatorFixity::Infix => {
            matches!(
                associativity,
                Associativity::Left
                    | Associativity::Right
                    | Associativity::NonAssociative
            )
        }
        OperatorFixity::Ternary => {
            matches!(associativity, Associativity::Right)
        }
    };

    if valid {
        Ok(())
    } else {
        Err(PrecedenceError::InvalidAssociativity {
            fixity,
            associativity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_table_contains_canonical_precedence_order() {
        let table = PrecedenceTable::standard();

        let assignment = table
            .lookup_kind(TokenKind::Assign, OperatorFixity::Infix)
            .expect("assignment must be registered");
        let addition = table
            .lookup_kind(TokenKind::Plus, OperatorFixity::Infix)
            .expect("addition must be registered");
        let multiplication = table
            .lookup_kind(TokenKind::Star, OperatorFixity::Infix)
            .expect("multiplication must be registered");
        let prefix_minus = table
            .lookup_kind(TokenKind::Minus, OperatorFixity::Prefix)
            .expect("prefix minus must be registered");

        assert!(assignment.precedence() < addition.precedence());
        assert!(addition.precedence() < multiplication.precedence());
        assert!(multiplication.precedence() < prefix_minus.precedence());
        assert_eq!(
            table.lookup_kind(TokenKind::Plus, OperatorFixity::Prefix)
                .expect("prefix plus must be registered")
                .precedence(),
            PrecedenceLevel::Prefix
        );
    }

    #[test]
    fn assignment_is_right_associative() {
        let table = PrecedenceTable::standard();
        let spec = table
            .lookup_kind(TokenKind::Assign, OperatorFixity::Infix)
            .expect("assignment must be registered");

        assert_eq!(spec.associativity(), Associativity::Right);
        let bp = spec.binding_power().expect("infix binding powers");
        assert_eq!(bp.left(), bp.right());
    }

    #[test]
    fn additive_operators_are_left_associative() {
        let table = PrecedenceTable::standard();
        let spec = table
            .lookup_kind(TokenKind::Minus, OperatorFixity::Infix)
            .expect("subtraction must be registered");

        assert_eq!(spec.associativity(), Associativity::Left);
        let bp = spec.binding_power().expect("infix binding powers");
        assert_eq!(bp.right(), bp.left() + 1);
    }

    #[test]
    fn comparisons_are_non_associative() {
        let table = PrecedenceTable::standard();

        for kind in [
            TokenKind::Equals,
            TokenKind::NotEquals,
            TokenKind::LessThan,
            TokenKind::GreaterThanOrEqual,
        ] {
            let spec = table
                .lookup_kind(kind, OperatorFixity::Infix)
                .expect("comparison must be registered");
            assert_eq!(spec.associativity(), Associativity::NonAssociative);
        }
    }

    #[test]
    fn ranges_are_non_associative() {
        let table = PrecedenceTable::standard();

        for kind in [TokenKind::Range, TokenKind::RangeInclusive] {
            assert_eq!(
                table
                    .lookup_kind(kind, OperatorFixity::Infix)
                    .expect("range must be registered")
                    .associativity(),
                Associativity::NonAssociative
            );
        }
    }

    #[test]
    fn same_token_kind_can_have_prefix_and_infix_roles() {
        let table = PrecedenceTable::standard();

        assert!(table
            .lookup_kind(TokenKind::Minus, OperatorFixity::Prefix)
            .is_some());
        assert!(table
            .lookup_kind(TokenKind::Minus, OperatorFixity::Infix)
            .is_some());
    }

    #[test]
    fn exact_extension_entry_overrides_fallback_without_mutating_it() {
        let mut table = PrecedenceTable::standard();
        let fallback_before = table
            .lookup_kind(TokenKind::BitXor, OperatorFixity::Infix)
            .expect("bitwise XOR fallback")
            .precedence();

        let extension = OperatorSpec::for_spelling(
            TokenKind::BitXor,
            OperatorFixity::Infix,
            "⊕",
            PrecedenceLevel::Additive,
            Associativity::Left,
        )
        .expect("valid extension definition");

        table.register(extension).expect("unique key");

        assert_eq!(
            table
                .lookup(TokenKind::BitXor, OperatorFixity::Infix, "⊕")
                .expect("exact extension definition")
                .precedence(),
            PrecedenceLevel::Additive
        );
        assert_eq!(
            table
                .lookup(TokenKind::BitXor, OperatorFixity::Infix, "^")
                .expect("fallback definition")
                .precedence(),
            fallback_before
        );
    }

    #[test]
    fn duplicate_registration_is_rejected_without_replacement() {
        let mut table = PrecedenceTable::standard();
        let original_len = table.len();

        let duplicate = OperatorSpec::new(
            TokenKind::Plus,
            OperatorFixity::Infix,
            PrecedenceLevel::Assignment,
            Associativity::Right,
        )
        .expect("definition itself is valid");

        assert!(matches!(
            table.register(duplicate),
            Err(PrecedenceError::DuplicateDefinition { .. })
        ));
        assert_eq!(table.len(), original_len);
        assert_eq!(
            table
                .lookup_kind(TokenKind::Plus, OperatorFixity::Infix)
                .expect("original plus definition")
                .precedence(),
            PrecedenceLevel::Additive
        );
    }

    #[test]
    fn invalid_spelling_is_rejected() {
        assert!(matches!(
            OperatorKey::by_spelling(TokenKind::BitXor, OperatorFixity::Infix, ""),
            Err(PrecedenceError::InvalidSpelling { .. })
        ));
        assert!(matches!(
            OperatorKey::by_spelling(TokenKind::BitXor, OperatorFixity::Infix, "a b"),
            Err(PrecedenceError::InvalidSpelling { .. })
        ));
    }

    #[test]
    fn invalid_fixity_associativity_combination_is_rejected() {
        assert!(matches!(
            OperatorSpec::new(
                TokenKind::Plus,
                OperatorFixity::Prefix,
                PrecedenceLevel::Prefix,
                Associativity::Left
            ),
            Err(PrecedenceError::InvalidAssociativity { .. })
        ));
    }

    #[test]
    fn iteration_order_is_deterministic() {
        let table = PrecedenceTable::standard();
        let first: Vec<OperatorKey> =
            table.iter().map(|spec| spec.key().clone()).collect();
        let second: Vec<OperatorKey> =
            table.iter().map(|spec| spec.key().clone()).collect();

        assert_eq!(first, second);
        assert!(first.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn postfix_and_primary_levels_are_shared_constants() {
        assert_eq!(
            POSTFIX_PRECEDENCE,
            PrecedenceLevel::Postfix.rank()
        );
        assert_eq!(
            PRIMARY_PRECEDENCE,
            PrecedenceLevel::Primary.rank()
        );
        assert!(PRIMARY_PRECEDENCE > POSTFIX_PRECEDENCE);
    }
}
