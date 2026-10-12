
#![forbid(unsafe_code)]

//! Canonical token sets for the Zamani parser.
//!
//! File: `src/frontend/parser/token_sets.rs`
//! Rust: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide shared token classifications for grammar entry checks, predictive
//! parsing, delimiter handling, and error recovery. This module uses the
//! canonical lexer `TokenKind`; it does not define another token vocabulary.
//!
//! # Ownership
//!
//! This module owns:
//! - named core token sets used by parser productions;
//! - reusable predicates for common grammar decisions;
//! - a dynamic `TokenSet` for extension-defined FIRST/FOLLOW-like sets;
//! - the canonical mapping between opening and closing delimiters;
//! - documented synchronization sets for parser recovery.
//!
//! This module does not own:
//! - tokenization, token spellings, or token metadata;
//! - operator spelling or operator precedence;
//! - grammar productions or parser state;
//! - AST construction, name resolution, or semantic validation;
//! - extension registration or extension precedence;
//! - hardware capabilities, resource allocation, or target selection.
//!
//! # Integration contract
//!
//! - `crate::frontend::lexer::token_kind::TokenKind` is authoritative.
//! - `grammar/` and the normative language specification are authoritative
//!   for which constructs are legal Zamani syntax.
//! - `token_cursor.rs` supplies tokens to grammar productions.
//! - `grammar/*.rs` may use the start sets to select productions.
//! - `recovery.rs` may use synchronization sets to find a safe continuation.
//! - `precedence.rs` remains the authority for operator binding powers.
//! - `ast_adapter.rs` and semantic analysis remain downstream.
//!
//! A token set is a parsing aid, not a claim that a construct is legal in
//! every language edition or dialect. Configuration and grammar productions
//! must still enforce the active language contract.
//!
//! # Scalability
//!
//! Core sets describe syntax categories, not device inventories. Dynamic
//! sets can grow as resources permit. No set defines a maximum number of
//! qubits, devices, processors, tensor dimensions, nodes, or extensions.
//!
//! # Maintenance rule
//!
//! When a canonical token kind or normative grammar production changes,
//! review the affected sets and their tests in the same change. Do not add
//! token spellings here; spellings belong to the lexer registries.

use std::collections::BTreeSet;
use std::fmt;

use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Core grammar start sets
// -----------------------------------------------------------------------------
//
// These sets use canonical TokenKind values. Keep them aligned with the
// normative specification and the existing lexer vocabulary. Contextual
// restrictions remain the responsibility of the production that consumes
// the token.

/// Tokens that may begin a primary, prefix, or block-like expression.
///
/// This is intentionally a syntactic candidate set. A grammar production
/// must still determine whether the token is legal in its current context.
pub const EXPRESSION_START: &[TokenKind] = &[
    TokenKind::Identifier,
    TokenKind::IntegerLiteral,
    TokenKind::FloatLiteral,
    TokenKind::StringLiteral,
    TokenKind::CharacterLiteral,
    TokenKind::BooleanLiteral,
    TokenKind::QuantumLiteral,
    TokenKind::DurationLiteral,
    TokenKind::SizeLiteral,
    TokenKind::HardwareLiteral,
    TokenKind::MtsLiteral,
    TokenKind::LeftParen,
    TokenKind::LeftBrace,
    TokenKind::LeftBracket,
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::LogicalNot,
    TokenKind::BitNot,
    TokenKind::BitAnd,
    TokenKind::Star,
    TokenKind::KeywordTrue,
    TokenKind::KeywordFalse,
    TokenKind::KeywordNil,
    TokenKind::KeywordThis,
    TokenKind::KeywordSelf,
    TokenKind::KeywordSuper,
    TokenKind::KeywordNew,
    TokenKind::KeywordIf,
    TokenKind::KeywordMatch,
    TokenKind::KeywordLoop,
    TokenKind::KeywordAsync,
    TokenKind::KeywordAwait,
    TokenKind::KeywordSpawn,
    TokenKind::KeywordTry,
    TokenKind::KeywordQuantum,
];

/// Tokens that may begin a declaration or declaration-like item.
///
/// This includes modifier tokens where the grammar permits them before a
/// declaration. It does not imply that every modifier is valid on every item.
pub const DECLARATION_START: &[TokenKind] = &[
    TokenKind::KeywordLet,
    TokenKind::KeywordVar,
    TokenKind::KeywordMut,
    TokenKind::KeywordConst,
    TokenKind::KeywordFn,
    TokenKind::KeywordModule,
    TokenKind::KeywordType,
    TokenKind::KeywordStruct,
    TokenKind::KeywordEnum,
    TokenKind::KeywordTrait,
    TokenKind::KeywordImpl,
    TokenKind::KeywordClass,
    TokenKind::KeywordInterface,
    TokenKind::KeywordRecord,
    TokenKind::KeywordEffect,
    TokenKind::KeywordQuantum,
    TokenKind::KeywordCircuit,
    TokenKind::KeywordAgent,
    TokenKind::KeywordLanguage,
    TokenKind::KeywordImport,
    TokenKind::KeywordExport,
    TokenKind::KeywordUse,
    TokenKind::KeywordPublic,
    TokenKind::KeywordPrivate,
    TokenKind::KeywordProtected,
    TokenKind::KeywordStatic,
    TokenKind::KeywordAbstract,
    TokenKind::KeywordVirtual,
];

/// Tokens that may begin a statement.
///
/// Expression statements are represented by the expression-start set and
/// must be selected by the enclosing statement grammar when permitted.
pub const STATEMENT_START: &[TokenKind] = &[
    TokenKind::KeywordLet,
    TokenKind::KeywordVar,
    TokenKind::KeywordMut,
    TokenKind::KeywordConst,
    TokenKind::KeywordReturn,
    TokenKind::KeywordIf,
    TokenKind::KeywordFor,
    TokenKind::KeywordWhile,
    TokenKind::KeywordLoop,
    TokenKind::KeywordBreak,
    TokenKind::KeywordContinue,
    TokenKind::KeywordMatch,
    TokenKind::KeywordThrow,
    TokenKind::KeywordYield,
    TokenKind::KeywordSpawn,
    TokenKind::KeywordTry,
    TokenKind::LeftBrace,
];

/// Tokens that may begin a type expression.
///
/// Type grammar must resolve context-sensitive interpretations such as
/// generic arguments versus relational operators.
pub const TYPE_START: &[TokenKind] = &[
    TokenKind::Identifier,
    TokenKind::LeftParen,
    TokenKind::LeftBracket,
    TokenKind::BitAnd,
    TokenKind::Star,
    TokenKind::KeywordFn,
    TokenKind::KeywordVoid,
    TokenKind::KeywordInt,
    TokenKind::KeywordFloat,
    TokenKind::KeywordBool,
    TokenKind::KeywordStr,
    TokenKind::KeywordStringType,
    TokenKind::KeywordCharType,
    TokenKind::KeywordSelf,
    TokenKind::KeywordThis,
    TokenKind::KeywordQuantum,
    TokenKind::KeywordQubit,
];

/// Tokens that may begin a pattern.
///
/// Pattern productions decide whether a particular literal, path, or
/// aggregate form is valid in the current match or binding context.
pub const PATTERN_START: &[TokenKind] = &[
    TokenKind::Identifier,
    TokenKind::IntegerLiteral,
    TokenKind::FloatLiteral,
    TokenKind::StringLiteral,
    TokenKind::CharacterLiteral,
    TokenKind::BooleanLiteral,
    TokenKind::KeywordTrue,
    TokenKind::KeywordFalse,
    TokenKind::KeywordNil,
    TokenKind::LeftParen,
    TokenKind::LeftBracket,
    TokenKind::LeftBrace,
    TokenKind::BitAnd,
    TokenKind::Minus,
];

/// Tokens that may begin a top-level source item.
///
/// Attributes and annotations are included because they may prefix an item
/// under the grammar. Documentation comments are not included: they are
/// usually trivia or a separate documentation element, depending on policy.
pub const ITEM_START: &[TokenKind] = &[
    TokenKind::Annotation,
    TokenKind::NanoAnnotation,
    TokenKind::At,
    TokenKind::Hash,
    TokenKind::KeywordLet,
    TokenKind::KeywordVar,
    TokenKind::KeywordConst,
    TokenKind::KeywordFn,
    TokenKind::KeywordModule,
    TokenKind::KeywordImport,
    TokenKind::KeywordExport,
    TokenKind::KeywordUse,
    TokenKind::KeywordType,
    TokenKind::KeywordStruct,
    TokenKind::KeywordEnum,
    TokenKind::KeywordTrait,
    TokenKind::KeywordImpl,
    TokenKind::KeywordClass,
    TokenKind::KeywordInterface,
    TokenKind::KeywordRecord,
    TokenKind::KeywordEffect,
    TokenKind::KeywordQuantum,
    TokenKind::KeywordCircuit,
    TokenKind::KeywordAgent,
    TokenKind::KeywordLanguage,
    TokenKind::KeywordPublic,
    TokenKind::KeywordPrivate,
    TokenKind::KeywordProtected,
    TokenKind::KeywordStatic,
    TokenKind::KeywordAbstract,
    TokenKind::KeywordVirtual,
];

/// Tokens that may begin an attribute or annotation.
pub const ATTRIBUTE_START: &[TokenKind] = &[
    TokenKind::Annotation,
    TokenKind::NanoAnnotation,
    TokenKind::At,
    TokenKind::Hash,
];

/// Tokens that may begin a literal expression.
pub const LITERAL_START: &[TokenKind] = &[
    TokenKind::IntegerLiteral,
    TokenKind::FloatLiteral,
    TokenKind::StringLiteral,
    TokenKind::CharacterLiteral,
    TokenKind::BooleanLiteral,
    TokenKind::QuantumLiteral,
    TokenKind::DurationLiteral,
    TokenKind::SizeLiteral,
    TokenKind::HardwareLiteral,
    TokenKind::MtsLiteral,
    TokenKind::KeywordTrue,
    TokenKind::KeywordFalse,
    TokenKind::KeywordNil,
];

// -----------------------------------------------------------------------------
// Delimiters and separators
// -----------------------------------------------------------------------------

/// Opening grouping tokens recognized by the shared grammar infrastructure.
pub const OPENING_DELIMITERS: &[TokenKind] = &[
    TokenKind::LeftParen,
    TokenKind::LeftBrace,
    TokenKind::LeftBracket,
];

/// Closing grouping tokens recognized by the shared grammar infrastructure.
pub const CLOSING_DELIMITERS: &[TokenKind] = &[
    TokenKind::RightParen,
    TokenKind::RightBrace,
    TokenKind::RightBracket,
];

/// Tokens that commonly separate arguments, parameters, fields, or items.
pub const SEPARATOR_TOKENS: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::Semicolon,
];

/// Tokens that can terminate a statement or item in supported grammar forms.
///
/// Whether a semicolon is mandatory, optional, or forbidden is decided by
/// the active grammar and language edition.
pub const STATEMENT_TERMINATORS: &[TokenKind] = &[
    TokenKind::Semicolon,
    TokenKind::RightBrace,
    TokenKind::Eof,
];

/// Common tokens that may follow a parsed expression.
///
/// This set is intentionally broader than a single expression production;
/// the caller must select the correct context-specific follow set.
pub const EXPRESSION_FOLLOW: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::Semicolon,
    TokenKind::Colon,
    TokenKind::RightParen,
    TokenKind::RightBracket,
    TokenKind::RightBrace,
    TokenKind::FatArrow,
    TokenKind::KeywordElse,
    TokenKind::KeywordIn,
    TokenKind::Eof,
];

/// Common tokens that may follow a parsed type expression.
pub const TYPE_FOLLOW: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::Semicolon,
    TokenKind::Colon,
    TokenKind::Assign,
    TokenKind::RightParen,
    TokenKind::RightBracket,
    TokenKind::RightBrace,
    TokenKind::GreaterThan,
    TokenKind::FatArrow,
    TokenKind::KeywordWhere,
    TokenKind::Eof,
];

/// Common tokens that may follow an argument or parameter.
pub const ARGUMENT_FOLLOW: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::RightParen,
    TokenKind::Eof,
];

/// Common tokens that may follow a declaration parameter.
pub const PARAMETER_FOLLOW: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::RightParen,
    TokenKind::Colon,
    TokenKind::Eof,
];

// -----------------------------------------------------------------------------
// Recovery synchronization sets
// -----------------------------------------------------------------------------

/// General recovery tokens that are likely to establish a safe continuation.
///
/// Recovery must always guarantee progress. A synchronization token should
/// only be consumed when the recovery algorithm's ownership contract says
/// it is safe to do so.
pub const SYNCHRONIZATION: &[TokenKind] = &[
    TokenKind::Semicolon,
    TokenKind::RightBrace,
    TokenKind::RightParen,
    TokenKind::RightBracket,
    TokenKind::KeywordCase,
    TokenKind::KeywordWhen,
    TokenKind::KeywordElse,
    TokenKind::KeywordCatch,
    TokenKind::KeywordFinally,
    TokenKind::Eof,
];

/// Tokens that commonly mark the beginning of a subsequent top-level item.
///
/// Use this set when recovering at source-unit boundaries. A recovery routine
/// should avoid consuming a candidate start token if it can resume parsing
/// from that token directly.
pub const TOP_LEVEL_SYNCHRONIZATION: &[TokenKind] = &[
    TokenKind::Annotation,
    TokenKind::NanoAnnotation,
    TokenKind::At,
    TokenKind::Hash,
    TokenKind::KeywordLet,
    TokenKind::KeywordVar,
    TokenKind::KeywordConst,
    TokenKind::KeywordFn,
    TokenKind::KeywordModule,
    TokenKind::KeywordImport,
    TokenKind::KeywordExport,
    TokenKind::KeywordUse,
    TokenKind::KeywordType,
    TokenKind::KeywordStruct,
    TokenKind::KeywordEnum,
    TokenKind::KeywordTrait,
    TokenKind::KeywordImpl,
    TokenKind::KeywordClass,
    TokenKind::KeywordInterface,
    TokenKind::KeywordRecord,
    TokenKind::KeywordEffect,
    TokenKind::KeywordQuantum,
    TokenKind::KeywordCircuit,
    TokenKind::KeywordAgent,
    TokenKind::KeywordLanguage,
    TokenKind::Eof,
];

// -----------------------------------------------------------------------------
// Public predicates
// -----------------------------------------------------------------------------

/// Returns whether `kind` belongs to a static core token set.
#[must_use]
pub fn is_in(kind: TokenKind, set: &[TokenKind]) -> bool {
    set.contains(&kind)
}

/// Returns whether `kind` may begin an expression.
#[must_use]
pub fn is_expression_start(kind: TokenKind) -> bool {
    is_in(kind, EXPRESSION_START)
}

/// Returns whether `kind` may begin a declaration.
#[must_use]
pub fn is_declaration_start(kind: TokenKind) -> bool {
    is_in(kind, DECLARATION_START)
}

/// Returns whether `kind` may begin a statement.
#[must_use]
pub fn is_statement_start(kind: TokenKind) -> bool {
    is_in(kind, STATEMENT_START) || is_expression_start(kind)
}

/// Returns whether `kind` may begin a type expression.
#[must_use]
pub fn is_type_start(kind: TokenKind) -> bool {
    is_in(kind, TYPE_START)
}

/// Returns whether `kind` may begin a pattern.
#[must_use]
pub fn is_pattern_start(kind: TokenKind) -> bool {
    is_in(kind, PATTERN_START)
}

/// Returns whether `kind` may begin a top-level item.
#[must_use]
pub fn is_item_start(kind: TokenKind) -> bool {
    is_in(kind, ITEM_START)
}

/// Returns whether `kind` may begin an attribute or annotation.
#[must_use]
pub fn is_attribute_start(kind: TokenKind) -> bool {
    is_in(kind, ATTRIBUTE_START)
}

/// Returns whether `kind` is an opening grouping delimiter.
#[must_use]
pub fn is_opening_delimiter(kind: TokenKind) -> bool {
    is_in(kind, OPENING_DELIMITERS)
}

/// Returns whether `kind` is a closing grouping delimiter.
#[must_use]
pub fn is_closing_delimiter(kind: TokenKind) -> bool {
    is_in(kind, CLOSING_DELIMITERS)
}

/// Returns the matching closing delimiter for an opening delimiter.
///
/// Non-opening tokens return `None`; they are not silently reinterpreted.
#[must_use]
pub const fn matching_closer(open: TokenKind) -> Option<TokenKind> {
    match open {
        TokenKind::LeftParen => Some(TokenKind::RightParen),
        TokenKind::LeftBrace => Some(TokenKind::RightBrace),
        TokenKind::LeftBracket => Some(TokenKind::RightBracket),
        _ => None,
    }
}

/// Returns the matching opening delimiter for a closing delimiter.
///
/// Non-closing tokens return `None`.
#[must_use]
pub const fn matching_opener(close: TokenKind) -> Option<TokenKind> {
    match close {
        TokenKind::RightParen => Some(TokenKind::LeftParen),
        TokenKind::RightBrace => Some(TokenKind::LeftBrace),
        TokenKind::RightBracket => Some(TokenKind::LeftBracket),
        _ => None,
    }
}

/// Returns whether `kind` is a common parser recovery synchronization token.
#[must_use]
pub fn is_synchronization_token(kind: TokenKind) -> bool {
    is_in(kind, SYNCHRONIZATION)
}

/// Returns whether `kind` is a source-unit synchronization candidate.
#[must_use]
pub fn is_top_level_synchronization_token(kind: TokenKind) -> bool {
    is_in(kind, TOP_LEVEL_SYNCHRONIZATION)
}

// -----------------------------------------------------------------------------
// Dynamic sets for grammar extensions and specialized productions
// -----------------------------------------------------------------------------

/// A dynamically sized set of canonical token kinds.
///
/// Use this for production-specific FIRST/FOLLOW-like sets or extension
/// handlers that need to compose token categories. Core grammar sets above
/// remain the stable shared defaults.
///
/// `TokenSet` is deliberately independent of extension registration. An
/// extension contributes token kinds only after its registration and grammar
/// contracts have been validated by the owning registry.
///
/// The set has no configured cardinality ceiling. Its practical size is
/// bounded by available memory and the number of canonical token kinds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TokenSet {
    name: String,
    kinds: BTreeSet<TokenKind>,
}

impl TokenSet {
    /// Creates an empty named set.
    ///
    /// The name is diagnostic metadata and may be any string, including an
    /// empty one. Callers that require non-empty names should validate them
    /// at their configuration or extension-registration boundary.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kinds: BTreeSet::new(),
        }
    }

    /// Creates a set from an iterator of token kinds.
    #[must_use]
    pub fn from_kinds(
        name: impl Into<String>,
        kinds: impl IntoIterator<Item = TokenKind>,
    ) -> Self {
        Self {
            name: name.into(),
            kinds: kinds.into_iter().collect(),
        }
    }

    /// Creates a set from one of the shared static core sets.
    #[must_use]
    pub fn from_static(name: impl Into<String>, kinds: &[TokenKind]) -> Self {
        Self::from_kinds(name, kinds.iter().copied())
    }

    /// Returns the set's descriptive name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the set contains `kind`.
    #[must_use]
    pub fn contains(&self, kind: TokenKind) -> bool {
        self.kinds.contains(&kind)
    }

    /// Inserts `kind`, returning `true` only if it was not already present.
    pub fn insert(&mut self, kind: TokenKind) -> bool {
        self.kinds.insert(kind)
    }

    /// Removes `kind`, returning whether it was present.
    pub fn remove(&mut self, kind: TokenKind) -> bool {
        self.kinds.remove(&kind)
    }

    /// Adds all kinds from an iterator.
    ///
    /// Returns the number of newly inserted kinds.
    pub fn extend(&mut self, kinds: impl IntoIterator<Item = TokenKind>) -> usize {
        let mut inserted = 0;
        for kind in kinds {
            inserted += usize::from(self.kinds.insert(kind));
        }
        inserted
    }

    /// Adds all kinds from another set.
    ///
    /// The receiver keeps its own name.
    pub fn extend_set(&mut self, other: &Self) -> usize {
        self.extend(other.kinds.iter().copied())
    }

    /// Returns the number of distinct token kinds in this set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    /// Removes every token kind from this set.
    pub fn clear(&mut self) {
        self.kinds.clear();
    }

    /// Iterates over kinds in deterministic `TokenKind` ordering.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = TokenKind> + '_ {
        self.kinds.iter().copied()
    }

    /// Returns whether any kind in `candidates` belongs to this set.
    pub fn intersects(&self, candidates: impl IntoIterator<Item = TokenKind>) -> bool {
        candidates.into_iter().any(|kind| self.contains(kind))
    }

    /// Returns a deterministic snapshot of this set's kinds.
    #[must_use]
    pub fn to_vec(&self) -> Vec<TokenKind> {
        self.kinds.iter().copied().collect()
    }
}

impl FromIterator<TokenKind> for TokenSet {
    fn from_iter<T: IntoIterator<Item = TokenKind>>(iter: T) -> Self {
        Self::from_kinds("custom", iter)
    }
}

impl fmt::Display for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} token kinds)", self.name, self.kinds.len())
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_starts_include_common_literals_and_prefix_forms() {
        assert!(is_expression_start(TokenKind::Identifier));
        assert!(is_expression_start(TokenKind::IntegerLiteral));
        assert!(is_expression_start(TokenKind::StringLiteral));
        assert!(is_expression_start(TokenKind::LeftParen));
        assert!(is_expression_start(TokenKind::Minus));
        assert!(is_expression_start(TokenKind::KeywordIf));
        assert!(!is_expression_start(TokenKind::Eof));
        assert!(!is_expression_start(TokenKind::Semicolon));
    }

    #[test]
    fn declaration_and_item_starts_are_available_separately() {
        assert!(is_declaration_start(TokenKind::KeywordFn));
        assert!(is_declaration_start(TokenKind::KeywordStruct));
        assert!(is_item_start(TokenKind::KeywordQuantum));
        assert!(!is_declaration_start(TokenKind::Eof));
    }

    #[test]
    fn statement_start_includes_expression_statements() {
        assert!(is_statement_start(TokenKind::KeywordReturn));
        assert!(is_statement_start(TokenKind::Identifier));
        assert!(is_statement_start(TokenKind::IntegerLiteral));
        assert!(!is_statement_start(TokenKind::Eof));
    }

    #[test]
    fn type_and_pattern_sets_cover_shared_forms() {
        assert!(is_type_start(TokenKind::Identifier));
        assert!(is_type_start(TokenKind::LeftBracket));
        assert!(is_type_start(TokenKind::KeywordInt));

        assert!(is_pattern_start(TokenKind::Identifier));
        assert!(is_pattern_start(TokenKind::LeftBrace));
        assert!(is_pattern_start(TokenKind::KeywordTrue));
    }

    #[test]
    fn delimiters_map_in_both_directions() {
        assert_eq!(
            matching_closer(TokenKind::LeftParen),
            Some(TokenKind::RightParen)
        );
        assert_eq!(
            matching_closer(TokenKind::LeftBrace),
            Some(TokenKind::RightBrace)
        );
        assert_eq!(
            matching_closer(TokenKind::LeftBracket),
            Some(TokenKind::RightBracket)
        );

        assert_eq!(
            matching_opener(TokenKind::RightParen),
            Some(TokenKind::LeftParen)
        );
        assert_eq!(
            matching_opener(TokenKind::RightBrace),
            Some(TokenKind::LeftBrace)
        );
        assert_eq!(
            matching_opener(TokenKind::RightBracket),
            Some(TokenKind::LeftBracket)
        );

        assert_eq!(matching_closer(TokenKind::Identifier), None);
        assert_eq!(matching_opener(TokenKind::Eof), None);
    }

    #[test]
    fn synchronization_sets_include_eof_and_statement_boundaries() {
        assert!(is_synchronization_token(TokenKind::Semicolon));
        assert!(is_synchronization_token(TokenKind::RightBrace));
        assert!(is_synchronization_token(TokenKind::Eof));
        assert!(is_top_level_synchronization_token(TokenKind::KeywordFn));
        assert!(is_top_level_synchronization_token(TokenKind::Eof));
        assert!(!is_top_level_synchronization_token(TokenKind::IntegerLiteral));
    }

    #[test]
    fn dynamic_set_deduplicates_and_iterates_deterministically() {
        let mut set = TokenSet::new("test");

        assert!(set.insert(TokenKind::Identifier));
        assert!(!set.insert(TokenKind::Identifier));
        assert!(set.insert(TokenKind::Eof));
        assert_eq!(set.len(), 2);
        assert!(set.contains(TokenKind::Identifier));

        let snapshot = set.to_vec();
        assert_eq!(snapshot, set.to_vec());
        assert_eq!(snapshot.len(), 2);
    }

    #[test]
    fn dynamic_set_can_extend_from_static_and_dynamic_sets() {
        let mut set = TokenSet::from_static("expression", EXPRESSION_START);
        assert!(set.contains(TokenKind::Identifier));

        let mut extra = TokenSet::new("extension");
        extra.insert(TokenKind::KeywordLearn);
        extra.insert(TokenKind::Identifier);

        let added = set.extend_set(&extra);
        assert_eq!(added, 1);
        assert!(set.contains(TokenKind::KeywordLearn));
        assert!(set.contains(TokenKind::Identifier));
    }

    #[test]
    fn dynamic_set_supports_collection_and_clear() {
        let mut set: TokenSet = [
            TokenKind::Identifier,
            TokenKind::IntegerLiteral,
            TokenKind::Identifier,
        ]
        .into_iter()
        .collect();

        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
        assert_eq!(set.extend([TokenKind::Eof, TokenKind::Eof]), 1);

        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn core_sets_do_not_treat_invalid_input_as_a_grammar_start() {
        assert!(!is_expression_start(TokenKind::Invalid));
        assert!(!is_declaration_start(TokenKind::Invalid));
        assert!(!is_type_start(TokenKind::Invalid));
        assert!(!is_pattern_start(TokenKind::Invalid));
        assert!(!is_item_start(TokenKind::Invalid));
    }
}
