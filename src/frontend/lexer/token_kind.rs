//! Canonical token identities for the Zamani programming language.
//!
//! File: src/frontend/lexer/token_kind.rs
//! Rust: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only; no unsafe code.
//!
//! # Purpose
//!
//! Define stable, domain-neutral identities for lexical tokens.
//!
//! # Responsibilities
//!
//! This module owns:
//! - canonical token identities;
//! - broad lexical categories;
//! - category and predicate helpers;
//! - stable diagnostic labels.
//!
//! This module does not own:
//! - source scanning or UTF-8 traversal;
//! - keyword and operator spellings;
//! - source spans or source storage;
//! - lexical configuration or resource budgets;
//! - diagnostics and error recovery;
//! - dialect registration;
//! - AST, semantic analysis, quantum semantics, or IR.
//!
//! # Integration contract
//!
//! - `token.rs` stores `TokenKind` in each token.
//! - `token_metadata.rs` derives metadata from `TokenKind`.
//! - `keyword_registry.rs` maps reserved spellings to keyword kinds.
//! - `operator_registry.rs` maps operator spellings to operator kinds.
//! - `dialect.rs` controls approved dialect-specific classification.
//! - `scanner.rs` constructs tokens using these identities.
//! - `diagnostics.rs` and `recovery.rs` report and recover from errors.
//! - The parser consumes the resulting token stream.
//! - Semantic analysis and canonical IR interpret program meaning.
//!
//! # Extensibility and scalability
//!
//! Quantum operation names, hardware vendor names, device names,
//! capability names, and dialect-defined names are identifiers unless
//! the normative language specification explicitly reserves their spelling.
//!
//! This module imposes no language-level limit on source size, token count,
//! identifier length, integer width, qubit count, processor count, memory,
//! tensor rank, device count, or distributed topology.
//!
//! Implementation resource budgets belong in `config.rs` and `limits.rs`.
//!
//! # Compatibility
//!
//! The existing `src/lexer.rs` and `src/parser.rs` use a legacy `TokenType`
//! enum. This module defines the canonical model for the new frontend.
//! Migrate those consumers through an explicit compatibility mapping;
//! do not maintain two competing token vocabularies indefinitely.

use core::fmt;

/// Canonical identity of a lexical token.
///
/// A token kind describes lexical recognition, not semantic meaning.
/// Spelling, source location, and semantic interpretation are maintained
/// by their respective layers.
///
/// This enum is non-exhaustive so downstream crates can prepare for
/// future additions without assuming that the vocabulary is permanently
/// closed. Code matching on this enum outside its defining crate must
/// include a wildcard arm.
///
/// Do not use implicit enum discriminants as serialized token IDs.
/// Any external ID format must be explicitly defined and versioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TokenKind {
    // ------------------------------------------------------------------
    // General lexical categories
    // ------------------------------------------------------------------

    /// An identifier not classified as a reserved keyword.
    Identifier,

    /// An integer literal.
    IntegerLiteral,

    /// A floating-point or fractional numeric literal.
    FloatLiteral,

    /// A string literal.
    StringLiteral,

    /// A character literal.
    CharacterLiteral,

    /// A boolean literal, when recognized as a literal by the lexer.
    BooleanLiteral,

    /// A quantum-state or quantum-domain literal.
    ///
    /// This identifies syntax only; it does not model a quantum state,
    /// operation, register, or hardware resource.
    QuantumLiteral,

    /// A duration literal, when supported by the active language version.
    DurationLiteral,

    /// A size or quantity literal, when supported by the grammar.
    SizeLiteral,

    /// A hardware/resource-oriented literal, when specified by the grammar.
    HardwareLiteral,

    /// An annotation construct.
    Annotation,

    /// A nano annotation construct.
    NanoAnnotation,

    /// An MTS literal, when supported by the active language version.
    MtsLiteral,

    /// A line comment retained in the token stream.
    LineComment,

    /// A block comment retained in the token stream.
    BlockComment,

    /// Whitespace retained in the token stream.
    Whitespace,

    /// A newline retained as a distinct token.
    Newline,

    /// Invalid or unrecognized lexical input.
    ///
    /// Detailed diagnostics belong in `diagnostics.rs`.
    /// Resource exhaustion must be reported separately.
    Invalid,

    /// Synthetic end-of-input token.
    Eof,

    // ------------------------------------------------------------------
    // Delimiters
    // ------------------------------------------------------------------

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // ------------------------------------------------------------------
    // Punctuation
    // ------------------------------------------------------------------

    Comma,
    Dot,
    Semicolon,
    Colon,
    DoubleColon,
    At,
    Hash,
    Dollar,
    Question,
    Tilde,

    // ------------------------------------------------------------------
    // Arithmetic operators
    // ------------------------------------------------------------------

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    // ------------------------------------------------------------------
    // Assignment operators
    // ------------------------------------------------------------------

    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    ModuloAssign,

    // ------------------------------------------------------------------
    // Equality and comparison operators
    // ------------------------------------------------------------------

    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // ------------------------------------------------------------------
    // Boolean, bitwise, and shift operators
    // ------------------------------------------------------------------

    LogicalAnd,
    LogicalOr,
    LogicalNot,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    LeftShift,
    RightShift,

    // ------------------------------------------------------------------
    // Ranges and arrows
    // ------------------------------------------------------------------

    /// Exclusive range operator, such as `..`.
    Range,

    /// Inclusive range operator, such as `..=`.
    RangeInclusive,

    /// Thin arrow, such as `->`.
    ThinArrow,

    /// Fat arrow, such as `=>`.
    FatArrow,

    // ------------------------------------------------------------------
    // Core language keywords
    // ------------------------------------------------------------------

    KeywordLet,
    KeywordVar,
    KeywordMut,
    KeywordConst,
    KeywordFn,
    KeywordReturn,

    KeywordIf,
    KeywordElse,
    KeywordFor,
    KeywordIn,
    KeywordWhile,
    KeywordLoop,
    KeywordBreak,
    KeywordContinue,

    KeywordMatch,
    KeywordCase,
    KeywordWhen,

    KeywordTrue,
    KeywordFalse,
    KeywordNil,

    KeywordAnd,
    KeywordOr,
    KeywordNot,

    // ------------------------------------------------------------------
    // Modules, declarations, and types
    // ------------------------------------------------------------------

    KeywordModule,
    KeywordImport,
    KeywordExport,
    KeywordUse,
    KeywordAs,
    KeywordFrom,

    KeywordType,
    KeywordStruct,
    KeywordEnum,
    KeywordTrait,
    KeywordImpl,
    KeywordClass,
    KeywordInterface,
    KeywordRecord,
    KeywordWhere,

    KeywordPublic,
    KeywordPrivate,
    KeywordProtected,
    KeywordStatic,
    KeywordOverride,
    KeywordVirtual,
    KeywordAbstract,
    KeywordExtends,
    KeywordImplements,

    KeywordThis,
    KeywordSelf,
    KeywordSuper,
    KeywordNew,

    // ------------------------------------------------------------------
    // Concurrency, effects, and control flow
    // ------------------------------------------------------------------

    KeywordAsync,
    KeywordAwait,
    KeywordSpawn,

    KeywordEffect,
    KeywordHandle,
    KeywordPerform,

    KeywordTry,
    KeywordCatch,
    KeywordFinally,
    KeywordThrow,

    KeywordMove,
    KeywordYield,

    // ------------------------------------------------------------------
    // Quantum and hybrid constructs
    // ------------------------------------------------------------------

    KeywordQuantum,
    KeywordQubit,
    KeywordCircuit,

    KeywordEntangle,
    KeywordNoise,
    KeywordFidelity,
    KeywordSurface,

    KeywordLogical,
    KeywordParity,

    // ------------------------------------------------------------------
    // Nano, agent, and knowledge constructs
    // ------------------------------------------------------------------

    KeywordNano,
    KeywordAgent,

    KeywordRemember,
    KeywordRecall,
    KeywordLearn,
    KeywordInfer,
    KeywordWisdom,
    KeywordSasa,
    KeywordAncestor,

    // ------------------------------------------------------------------
    // Ownership and type-system constructs
    // ------------------------------------------------------------------

    KeywordAffine,
    KeywordLinear,

    KeywordIs,
    KeywordThen,
    KeywordWith,

    // ------------------------------------------------------------------
    // Built-in types and utilities
    // ------------------------------------------------------------------

    KeywordVoid,
    KeywordInt,
    KeywordFloat,
    KeywordBool,
    KeywordStr,
    KeywordStringType,
    KeywordCharType,

    KeywordPrint,
    KeywordPrintln,
    KeywordAssert,
    KeywordPanic,
    KeywordLen,
    KeywordSizeof,

    // ------------------------------------------------------------------
    // Language and compatibility declarations
    // ------------------------------------------------------------------

    KeywordLanguage,
    KeywordMts,
    KeywordSigma,
    KeywordPi,
    KeywordCode,
    KeywordZamani,
    KeywordModel,
    KeywordUnsafe,

    // ------------------------------------------------------------------
    // Extended system vocabulary retained from the legacy lexer
    // ------------------------------------------------------------------
    //
    // These variants preserve existing vocabulary during migration.
    // Their existence does not require every spelling to be a reserved
    // word in every language version or dialect.
    //
    // Application-specific constructs should normally be provided by
    // libraries, dialects, capabilities, policies, or ordinary identifiers.

    KeywordOmniversal,
    KeywordSimulate,
    KeywordSynthesize,
    KeywordDeploy,
    KeywordAlignment,
    KeywordContainment,
    KeywordTrust,
    KeywordKnowledge,
    KeywordGenerative,
    KeywordSovereignty,
    KeywordGoal,
    KeywordBionano,
    KeywordReality,
    KeywordNlp,
    KeywordSystem,

    KeywordAsi,
    KeywordAesi,
    KeywordAsesi,
    KeywordAdmin,
    KeywordPayment,
    KeywordGateway,
    KeywordGraphics,
    KeywordVideo,
    KeywordAdjust,
    KeywordVersioning,
    KeywordCopyright,
    KeywordNotice,
    KeywordLegal,
    KeywordAction,
    KeywordTailor,
    KeywordBusiness,
}

/// Broad lexical classification of a token kind.
///
/// This classification is independent of source spelling, dialect,
/// source location, and semantic interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenCategory {
    Identifier,
    Literal,
    Annotation,
    Comment,
    Whitespace,
    Delimiter,
    Punctuation,
    Operator,
    Keyword,
    EndOfInput,
    Invalid,
}

impl TokenKind {
    /// Returns the broad lexical category of this kind.
    #[must_use]
    pub const fn category(self) -> TokenCategory {
        use TokenCategory as C;
        use TokenKind as K;

        match self {
            K::Identifier => C::Identifier,

            K::IntegerLiteral
            | K::FloatLiteral
            | K::StringLiteral
            | K::CharacterLiteral
            | K::BooleanLiteral
            | K::QuantumLiteral
            | K::DurationLiteral
            | K::SizeLiteral
            | K::HardwareLiteral
            | K::MtsLiteral => C::Literal,

            K::Annotation | K::NanoAnnotation => C::Annotation,

            K::LineComment | K::BlockComment => C::Comment,

            K::Whitespace | K::Newline => C::Whitespace,

            K::LeftParen
            | K::RightParen
            | K::LeftBrace
            | K::RightBrace
            | K::LeftBracket
            | K::RightBracket => C::Delimiter,

            K::Comma
            | K::Dot
            | K::Semicolon
            | K::Colon
            | K::DoubleColon
            | K::At
            | K::Hash
            | K::Dollar
            | K::Question
            | K::Tilde => C::Punctuation,

            K::Plus
            | K::Minus
            | K::Star
            | K::Slash
            | K::Modulo
            | K::Assign
            | K::PlusAssign
            | K::MinusAssign
            | K::StarAssign
            | K::SlashAssign
            | K::ModuloAssign
            | K::Equals
            | K::NotEquals
            | K::LessThan
            | K::LessThanOrEqual
            | K::GreaterThan
            | K::GreaterThanOrEqual
            | K::LogicalAnd
            | K::LogicalOr
            | K::LogicalNot
            | K::BitAnd
            | K::BitOr
            | K::BitXor
            | K::BitNot
            | K::LeftShift
            | K::RightShift
            | K::Range
            | K::RangeInclusive
            | K::ThinArrow
            | K::FatArrow => C::Operator,

            K::Eof => C::EndOfInput,
            K::Invalid => C::Invalid,

            _ => C::Keyword,
        }
    }

    /// Returns whether this kind is a keyword.
    #[must_use]
    pub const fn is_keyword(self) -> bool {
        matches!(self.category(), TokenCategory::Keyword)
    }

    /// Returns whether this kind is a literal.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(self.category(), TokenCategory::Literal)
    }

    /// Returns whether this kind is an operator.
    #[must_use]
    pub const fn is_operator(self) -> bool {
        matches!(self.category(), TokenCategory::Operator)
    }

    /// Returns whether this kind is a delimiter.
    #[must_use]
    pub const fn is_delimiter(self) -> bool {
        matches!(self.category(), TokenCategory::Delimiter)
    }

    /// Returns whether this kind is end of input.
    #[must_use]
    pub const fn is_eof(self) -> bool {
        matches!(self, Self::Eof)
    }

    /// Returns whether this kind represents invalid lexical input.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns whether this kind represents trivia.
    ///
    /// Whether trivia is retained or skipped is controlled by the scanner
    /// and lexical configuration.
    #[must_use]
    pub const fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace
                | Self::Newline
                | Self::LineComment
                | Self::BlockComment
        )
    }

    /// Returns a stable diagnostic label.
    ///
    /// This is not the token's source spelling. Source spelling belongs
    /// to the original source text or the appropriate registry.
    #[must_use]
    pub const fn diagnostic_name(self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::IntegerLiteral => "integer literal",
            Self::FloatLiteral => "floating-point literal",
            Self::StringLiteral => "string literal",
            Self::CharacterLiteral => "character literal",
            Self::BooleanLiteral => "boolean literal",
            Self::QuantumLiteral => "quantum literal",
            Self::DurationLiteral => "duration literal",
            Self::SizeLiteral => "size literal",
            Self::HardwareLiteral => "hardware literal",
            Self::Annotation => "annotation",
            Self::NanoAnnotation => "nano annotation",
            Self::MtsLiteral => "MTS literal",
            Self::LineComment => "line comment",
            Self::BlockComment => "block comment",
            Self::Whitespace => "whitespace",
            Self::Newline => "newline",
            Self::Invalid => "invalid token",
            Self::Eof => "end of input",
            _ => self.variant_name(),
        }
    }

    /// Returns the canonical Rust variant name as a diagnostic label.
    ///
    /// This method is exhaustive within this crate and does not infer
    /// source spelling from an enum variant.
    #[must_use]
    pub const fn variant_name(self) -> &'static str {
        match self {
            Self::Identifier => "Identifier",
            Self::IntegerLiteral => "IntegerLiteral",
            Self::FloatLiteral => "FloatLiteral",
            Self::StringLiteral => "StringLiteral",
            Self::CharacterLiteral => "CharacterLiteral",
            Self::BooleanLiteral => "BooleanLiteral",
            Self::QuantumLiteral => "QuantumLiteral",
            Self::DurationLiteral => "DurationLiteral",
            Self::SizeLiteral => "SizeLiteral",
            Self::HardwareLiteral => "HardwareLiteral",
            Self::Annotation => "Annotation",
            Self::NanoAnnotation => "NanoAnnotation",
            Self::MtsLiteral => "MtsLiteral",
            Self::LineComment => "LineComment",
            Self::BlockComment => "BlockComment",
            Self::Whitespace => "Whitespace",
            Self::Newline => "Newline",
            Self::Invalid => "Invalid",
            Self::Eof => "Eof",

            Self::LeftParen => "LeftParen",
            Self::RightParen => "RightParen",
            Self::LeftBrace => "LeftBrace",
            Self::RightBrace => "RightBrace",
            Self::LeftBracket => "LeftBracket",
            Self::RightBracket => "RightBracket",

            Self::Comma => "Comma",
            Self::Dot => "Dot",
            Self::Semicolon => "Semicolon",
            Self::Colon => "Colon",
            Self::DoubleColon => "DoubleColon",
            Self::At => "At",
            Self::Hash => "Hash",
            Self::Dollar => "Dollar",
            Self::Question => "Question",
            Self::Tilde => "Tilde",

            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Modulo => "Modulo",

            Self::Assign => "Assign",
            Self::PlusAssign => "PlusAssign",
            Self::MinusAssign => "MinusAssign",
            Self::StarAssign => "StarAssign",
            Self::SlashAssign => "SlashAssign",
            Self::ModuloAssign => "ModuloAssign",

            Self::Equals => "Equals",
            Self::NotEquals => "NotEquals",
            Self::LessThan => "LessThan",
            Self::LessThanOrEqual => "LessThanOrEqual",
            Self::GreaterThan => "GreaterThan",
            Self::GreaterThanOrEqual => "GreaterThanOrEqual",

            Self::LogicalAnd => "LogicalAnd",
            Self::LogicalOr => "LogicalOr",
            Self::LogicalNot => "LogicalNot",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::BitXor => "BitXor",
            Self::BitNot => "BitNot",
            Self::LeftShift => "LeftShift",
            Self::RightShift => "RightShift",

            Self::Range => "Range",
            Self::RangeInclusive => "RangeInclusive",
            Self::ThinArrow => "ThinArrow",
            Self::FatArrow => "FatArrow",

            Self::KeywordLet => "KeywordLet",
            Self::KeywordVar => "KeywordVar",
            Self::KeywordMut => "KeywordMut",
            Self::KeywordConst => "KeywordConst",
            Self::KeywordFn => "KeywordFn",
            Self::KeywordReturn => "KeywordReturn",
            Self::KeywordIf => "KeywordIf",
            Self::KeywordElse => "KeywordElse",
            Self::KeywordFor => "KeywordFor",
            Self::KeywordIn => "KeywordIn",
            Self::KeywordWhile => "KeywordWhile",
            Self::KeywordLoop => "KeywordLoop",
            Self::KeywordBreak => "KeywordBreak",
            Self::KeywordContinue => "KeywordContinue",
            Self::KeywordMatch => "KeywordMatch",
            Self::KeywordCase => "KeywordCase",
            Self::KeywordWhen => "KeywordWhen",
            Self::KeywordTrue => "KeywordTrue",
            Self::KeywordFalse => "KeywordFalse",
            Self::KeywordNil => "KeywordNil",
            Self::KeywordAnd => "KeywordAnd",
            Self::KeywordOr => "KeywordOr",
            Self::KeywordNot => "KeywordNot",

            Self::KeywordModule => "KeywordModule",
            Self::KeywordImport => "KeywordImport",
            Self::KeywordExport => "KeywordExport",
            Self::KeywordUse => "KeywordUse",
            Self::KeywordAs => "KeywordAs",
            Self::KeywordFrom => "KeywordFrom",
            Self::KeywordType => "KeywordType",
            Self::KeywordStruct => "KeywordStruct",
            Self::KeywordEnum => "KeywordEnum",
            Self::KeywordTrait => "KeywordTrait",
            Self::KeywordImpl => "KeywordImpl",
            Self::KeywordClass => "KeywordClass",
            Self::KeywordInterface => "KeywordInterface",
            Self::KeywordRecord => "KeywordRecord",
            Self::KeywordWhere => "KeywordWhere",
            Self::KeywordPublic => "KeywordPublic",
            Self::KeywordPrivate => "KeywordPrivate",
            Self::KeywordProtected => "KeywordProtected",
            Self::KeywordStatic => "KeywordStatic",
            Self::KeywordOverride => "KeywordOverride",
            Self::KeywordVirtual => "KeywordVirtual",
            Self::KeywordAbstract => "KeywordAbstract",
            Self::KeywordExtends => "KeywordExtends",
            Self::KeywordImplements => "KeywordImplements",
            Self::KeywordThis => "KeywordThis",
            Self::KeywordSelf => "KeywordSelf",
            Self::KeywordSuper => "KeywordSuper",
            Self::KeywordNew => "KeywordNew",

            Self::KeywordAsync => "KeywordAsync",
            Self::KeywordAwait => "KeywordAwait",
            Self::KeywordSpawn => "KeywordSpawn",
            Self::KeywordEffect => "KeywordEffect",
            Self::KeywordHandle => "KeywordHandle",
            Self::KeywordPerform => "KeywordPerform",
            Self::KeywordTry => "KeywordTry",
            Self::KeywordCatch => "KeywordCatch",
            Self::KeywordFinally => "KeywordFinally",
            Self::KeywordThrow => "KeywordThrow",
            Self::KeywordMove => "KeywordMove",
            Self::KeywordYield => "KeywordYield",

            Self::KeywordQuantum => "KeywordQuantum",
            Self::KeywordQubit => "KeywordQubit",
            Self::KeywordCircuit => "KeywordCircuit",
            Self::KeywordEntangle => "KeywordEntangle",
            Self::KeywordNoise => "KeywordNoise",
            Self::KeywordFidelity => "KeywordFidelity",
            Self::KeywordSurface => "KeywordSurface",
            Self::KeywordLogical => "KeywordLogical",
            Self::KeywordParity => "KeywordParity",

            Self::KeywordNano => "KeywordNano",
            Self::KeywordAgent => "KeywordAgent",
            Self::KeywordRemember => "KeywordRemember",
            Self::KeywordRecall => "KeywordRecall",
            Self::KeywordLearn => "KeywordLearn",
            Self::KeywordInfer => "KeywordInfer",
            Self::KeywordWisdom => "KeywordWisdom",
            Self::KeywordSasa => "KeywordSasa",
            Self::KeywordAncestor => "KeywordAncestor",

            Self::KeywordAffine => "KeywordAffine",
            Self::KeywordLinear => "KeywordLinear",
            Self::KeywordIs => "KeywordIs",
            Self::KeywordThen => "KeywordThen",
            Self::KeywordWith => "KeywordWith",

            Self::KeywordVoid => "KeywordVoid",
            Self::KeywordInt => "KeywordInt",
            Self::KeywordFloat => "KeywordFloat",
            Self::KeywordBool => "KeywordBool",
            Self::KeywordStr => "KeywordStr",
            Self::KeywordStringType => "KeywordStringType",
            Self::KeywordCharType => "KeywordCharType",
            Self::KeywordPrint => "KeywordPrint",
            Self::KeywordPrintln => "KeywordPrintln",
            Self::KeywordAssert => "KeywordAssert",
            Self::KeywordPanic => "KeywordPanic",
            Self::KeywordLen => "KeywordLen",
            Self::KeywordSizeof => "KeywordSizeof",

            Self::KeywordLanguage => "KeywordLanguage",
            Self::KeywordMts => "KeywordMts",
            Self::KeywordSigma => "KeywordSigma",
            Self::KeywordPi => "KeywordPi",
            Self::KeywordCode => "KeywordCode",
            Self::KeywordZamani => "KeywordZamani",
            Self::KeywordModel => "KeywordModel",
            Self::KeywordUnsafe => "KeywordUnsafe",

            Self::KeywordOmniversal => "KeywordOmniversal",
            Self::KeywordSimulate => "KeywordSimulate",
            Self::KeywordSynthesize => "KeywordSynthesize",
            Self::KeywordDeploy => "KeywordDeploy",
            Self::KeywordAlignment => "KeywordAlignment",
            Self::KeywordContainment => "KeywordContainment",
            Self::KeywordTrust => "KeywordTrust",
            Self::KeywordKnowledge => "KeywordKnowledge",
            Self::KeywordGenerative => "KeywordGenerative",
            Self::KeywordSovereignty => "KeywordSovereignty",
            Self::KeywordGoal => "KeywordGoal",
            Self::KeywordBionano => "KeywordBionano",
            Self::KeywordReality => "KeywordReality",
            Self::KeywordNlp => "KeywordNlp",
            Self::KeywordSystem => "KeywordSystem",

            Self::KeywordAsi => "KeywordAsi",
            Self::KeywordAesi => "KeywordAesi",
            Self::KeywordAsesi => "KeywordAsesi",
            Self::KeywordAdmin => "KeywordAdmin",
            Self::KeywordPayment => "KeywordPayment",
            Self::KeywordGateway => "KeywordGateway",
            Self::KeywordGraphics => "KeywordGraphics",
            Self::KeywordVideo => "KeywordVideo",
            Self::KeywordAdjust => "KeywordAdjust",
            Self::KeywordVersioning => "KeywordVersioning",
            Self::KeywordCopyright => "KeywordCopyright",
            Self::KeywordNotice => "KeywordNotice",
            Self::KeywordLegal => "KeywordLegal",
            Self::KeywordAction => "KeywordAction",
            Self::KeywordTailor => "KeywordTailor",
            Self::KeywordBusiness => "KeywordBusiness",
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.diagnostic_name())
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenCategory, TokenKind};
    use std::collections::HashSet;

    #[test]
    fn general_categories_are_correct() {
        assert_eq!(
            TokenKind::Identifier.category(),
            TokenCategory::Identifier
        );
        assert_eq!(
            TokenKind::IntegerLiteral.category(),
            TokenCategory::Literal
        );
        assert_eq!(
            TokenKind::QuantumLiteral.category(),
            TokenCategory::Literal
        );
        assert_eq!(
            TokenKind::KeywordQuantum.category(),
            TokenCategory::Keyword
        );
        assert_eq!(
            TokenKind::LeftParen.category(),
            TokenCategory::Delimiter
        );
        assert_eq!(
            TokenKind::Plus.category(),
            TokenCategory::Operator
        );
        assert_eq!(
            TokenKind::Eof.category(),
            TokenCategory::EndOfInput
        );
        assert_eq!(
            TokenKind::Invalid.category(),
            TokenCategory::Invalid
        );
    }

    #[test]
    fn classification_predicates_are_consistent() {
        assert!(TokenKind::KeywordFn.is_keyword());
        assert!(TokenKind::StringLiteral.is_literal());
        assert!(TokenKind::ThinArrow.is_operator());
        assert!(TokenKind::LeftBrace.is_delimiter());
        assert!(TokenKind::Eof.is_eof());
        assert!(TokenKind::Invalid.is_invalid());

        assert!(!TokenKind::Identifier.is_keyword());
        assert!(!TokenKind::Identifier.is_literal());
        assert!(!TokenKind::Plus.is_keyword());
    }

    #[test]
    fn trivia_classification_is_consistent() {
        assert!(TokenKind::Whitespace.is_trivia());
        assert!(TokenKind::Newline.is_trivia());
        assert!(TokenKind::LineComment.is_trivia());
        assert!(TokenKind::BlockComment.is_trivia());

        assert!(!TokenKind::Identifier.is_trivia());
        assert!(!TokenKind::Eof.is_trivia());
    }

    #[test]
    fn diagnostic_names_are_nonempty() {
        assert_eq!(TokenKind::Identifier.diagnostic_name(), "identifier");
        assert_eq!(TokenKind::Eof.diagnostic_name(), "end of input");
        assert_eq!(TokenKind::Invalid.diagnostic_name(), "invalid token");

        assert!(!TokenKind::KeywordQuantum.diagnostic_name().is_empty());
        assert!(!TokenKind::ThinArrow.variant_name().is_empty());
    }

    #[test]
    fn token_kinds_are_hashable() {
        let mut kinds = HashSet::new();

        kinds.insert(TokenKind::Identifier);
        kinds.insert(TokenKind::QuantumLiteral);
        kinds.insert(TokenKind::Identifier);

        assert_eq!(kinds.len(), 2);
    }

    #[test]
    fn extensible_quantum_operations_use_identifiers() {
        // A new operation name does not require a new TokenKind variant.
        // The scanner recognizes its spelling as Identifier unless the
        // active, approved language configuration reserves that spelling.
        let operation_kind = TokenKind::Identifier;

        assert_eq!(operation_kind.category(), TokenCategory::Identifier);
    }

    #[test]
    fn equivalent_operator_spellings_have_one_canonical_identity() {
        // Spellings are resolved by operator_registry.rs. This enum does
        // not provide duplicate identities for the same operator.
        assert_eq!(TokenKind::ThinArrow, TokenKind::ThinArrow);
        assert_eq!(TokenKind::Question, TokenKind::Question);
    }
}