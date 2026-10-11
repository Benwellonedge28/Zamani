#![forbid(unsafe_code)]

//! Compatibility regression tests for the Zamani lexer migration.
//!
//! File: src/frontend/lexer/tests/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Protect the compatibility boundary between:
//!
//! - the existing `crate::lexer::Lexer` and `TokenType` API;
//! - the canonical `crate::frontend::lexer::TokenKind` vocabulary;
//! - the normative lexical contract in `grammar/specification/lexical.md`.
//!
//! # Integration contract
//!
//! This is a test module, not a second lexer or a production compatibility
//! adapter. It deliberately does not add conversion logic to production code.
//!
//! A production adapter, when introduced, should live in its own module and
//! map legacy token identities to canonical identities explicitly. These
//! tests can then exercise that adapter directly without changing the
//! compatibility cases or making the test suite responsible for conversion.
//!
//! # Scope
//!
//! These tests cover representative stable token identities, reserved words,
//! ordinary domain-specific identifiers, operators, literals, and the
//! separation between lexical identity and domain semantics.
//!
//! They do not claim that every legacy token already has a verified mapping.
//! An unmapped legacy token is reported as an explicit compatibility gap.
//!
//! # Scalability and safety
//!
//! No target-specific capacities or fixed hardware limits are introduced.
//! Tests use small examples to validate contracts, not to define language
//! limits. Production resource budgets belong to the lexer configuration.
//!
//! The tests use safe Rust only.

use crate::frontend::lexer::token_kind::TokenKind;
use crate::lexer::{Lexer, TokenType};
use crate::source_map::FileId;

/// Lex one token from a small standalone source fragment.
///
/// This helper intentionally exercises the existing public legacy API.
/// It does not replace the canonical scanner or prescribe production
/// tokenization implementation details.
fn legacy_token(source: &str) -> TokenType {
    let mut lexer = Lexer::from_str(FileId::new(1), source);
    lexer.next_token().token_type
}

/// Map the deliberately supported compatibility subset to canonical kinds.
///
/// This is a test oracle, not a production adapter. The wildcard is
/// intentional: adding a legacy token does not silently imply that its
/// canonical mapping has been reviewed.
fn canonical_kind_for_legacy(token: &TokenType) -> Option<TokenKind> {
    match token {
        TokenType::LParen => Some(TokenKind::LeftParen),
        TokenType::RParen => Some(TokenKind::RightParen),
        TokenType::LBrace => Some(TokenKind::LeftBrace),
        TokenType::RBrace => Some(TokenKind::RightBrace),
        TokenType::LBracket => Some(TokenKind::LeftBracket),
        TokenType::RBracket => Some(TokenKind::RightBracket),

        TokenType::Comma => Some(TokenKind::Comma),
        TokenType::Dot => Some(TokenKind::Dot),
        TokenType::Semicolon => Some(TokenKind::Semicolon),
        TokenType::Colon => Some(TokenKind::Colon),
        TokenType::DoubleColon => Some(TokenKind::DoubleColon),

        TokenType::Plus => Some(TokenKind::Plus),
        TokenType::Minus => Some(TokenKind::Minus),
        TokenType::Star => Some(TokenKind::Star),
        TokenType::Slash => Some(TokenKind::Slash),
        TokenType::Modulo => Some(TokenKind::Modulo),
        TokenType::Assign => Some(TokenKind::Assign),
        TokenType::PlusAssign => Some(TokenKind::PlusAssign),
        TokenType::MinusAssign => Some(TokenKind::MinusAssign),
        TokenType::StarAssign => Some(TokenKind::StarAssign),
        TokenType::SlashAssign => Some(TokenKind::SlashAssign),

        TokenType::Equals => Some(TokenKind::Equals),
        TokenType::NotEquals => Some(TokenKind::NotEquals),
        TokenType::LessThan => Some(TokenKind::LessThan),
        TokenType::GreaterThan => Some(TokenKind::GreaterThan),
        TokenType::LessThanEqual => Some(TokenKind::LessThanOrEqual),
        TokenType::GreaterThanEqual => Some(TokenKind::GreaterThanOrEqual),
        TokenType::LogicalAnd => Some(TokenKind::LogicalAnd),
        TokenType::LogicalOr => Some(TokenKind::LogicalOr),
        TokenType::BitAnd | TokenType::Ampersand => Some(TokenKind::BitAnd),
        TokenType::BitOr | TokenType::Pipe => Some(TokenKind::BitOr),
        TokenType::Caret => Some(TokenKind::BitXor),
        TokenType::Tilde => Some(TokenKind::BitNot),
        TokenType::LeftShift => Some(TokenKind::LeftShift),
        TokenType::RightShift => Some(TokenKind::RightShift),

        TokenType::ThinArrow | TokenType::Arrow => Some(TokenKind::ThinArrow),
        TokenType::FatArrow => Some(TokenKind::FatArrow),
        TokenType::DotDot => Some(TokenKind::Range),
        TokenType::DotDotEq => Some(TokenKind::RangeInclusive),
        TokenType::Question | TokenType::QuestionMark => Some(TokenKind::Question),
        TokenType::Hash => Some(TokenKind::Hash),
        TokenType::At => Some(TokenKind::At),
        TokenType::Exclamation | TokenType::Not => Some(TokenKind::LogicalNot),

        TokenType::Identifier => Some(TokenKind::Identifier),
        TokenType::Integer => Some(TokenKind::IntegerLiteral),
        TokenType::Float => Some(TokenKind::FloatLiteral),
        TokenType::String => Some(TokenKind::StringLiteral),
        TokenType::Char => Some(TokenKind::CharacterLiteral),
        TokenType::Boolean => Some(TokenKind::BooleanLiteral),
        TokenType::QuantumLiteral => Some(TokenKind::QuantumLiteral),
        TokenType::NanoAnnotation => Some(TokenKind::NanoAnnotation),
        TokenType::MTSLiteral => Some(TokenKind::MtsLiteral),

        TokenType::KeywordLet => Some(TokenKind::KeywordLet),
        TokenType::KeywordVar => Some(TokenKind::KeywordVar),
        TokenType::KeywordMut => Some(TokenKind::KeywordMut),
        TokenType::KeywordConst => Some(TokenKind::KeywordConst),
        TokenType::KeywordFn => Some(TokenKind::KeywordFn),
        TokenType::KeywordReturn => Some(TokenKind::KeywordReturn),
        TokenType::KeywordIf => Some(TokenKind::KeywordIf),
        TokenType::KeywordElse => Some(TokenKind::KeywordElse),
        TokenType::KeywordFor => Some(TokenKind::KeywordFor),
        TokenType::KeywordIn => Some(TokenKind::KeywordIn),
        TokenType::KeywordWhile => Some(TokenKind::KeywordWhile),
        TokenType::KeywordLoop => Some(TokenKind::KeywordLoop),
        TokenType::KeywordBreak => Some(TokenKind::KeywordBreak),
        TokenType::KeywordContinue => Some(TokenKind::KeywordContinue),
        TokenType::KeywordMatch => Some(TokenKind::KeywordMatch),
        TokenType::KeywordCase => Some(TokenKind::KeywordCase),
        TokenType::KeywordWhen => Some(TokenKind::KeywordWhen),
        TokenType::KeywordTrue => Some(TokenKind::KeywordTrue),
        TokenType::KeywordFalse => Some(TokenKind::KeywordFalse),
        TokenType::KeywordNil => Some(TokenKind::KeywordNil),

        TokenType::KeywordModule => Some(TokenKind::KeywordModule),
        TokenType::KeywordImport => Some(TokenKind::KeywordImport),
        TokenType::KeywordExport => Some(TokenKind::KeywordExport),
        TokenType::KeywordUse => Some(TokenKind::KeywordUse),
        TokenType::KeywordAs => Some(TokenKind::KeywordAs),
        TokenType::KeywordFrom => Some(TokenKind::KeywordFrom),
        TokenType::KeywordType => Some(TokenKind::KeywordType),
        TokenType::KeywordStruct => Some(TokenKind::KeywordStruct),
        TokenType::KeywordEnum => Some(TokenKind::KeywordEnum),
        TokenType::KeywordTrait => Some(TokenKind::KeywordTrait),
        TokenType::KeywordImpl => Some(TokenKind::KeywordImpl),
        TokenType::KeywordClass => Some(TokenKind::KeywordClass),
        TokenType::KeywordInterface => Some(TokenKind::KeywordInterface),
        TokenType::KeywordRecord => Some(TokenKind::KeywordRecord),
        TokenType::KeywordWhere => Some(TokenKind::KeywordWhere),

        TokenType::KeywordAsync => Some(TokenKind::KeywordAsync),
        TokenType::KeywordAwait => Some(TokenKind::KeywordAwait),
        TokenType::KeywordSpawn => Some(TokenKind::KeywordSpawn),
        TokenType::KeywordEffect => Some(TokenKind::KeywordEffect),
        TokenType::KeywordHandle => Some(TokenKind::KeywordHandle),
        TokenType::KeywordPerform => Some(TokenKind::KeywordPerform),
        TokenType::KeywordTry => Some(TokenKind::KeywordTry),
        TokenType::KeywordCatch => Some(TokenKind::KeywordCatch),
        TokenType::KeywordFinally => Some(TokenKind::KeywordFinally),
        TokenType::KeywordThrow => Some(TokenKind::KeywordThrow),
        TokenType::KeywordMove => Some(TokenKind::KeywordMove),
        TokenType::KeywordYield => Some(TokenKind::KeywordYield),

        TokenType::KeywordQuantum => Some(TokenKind::KeywordQuantum),
        TokenType::KeywordQubit => Some(TokenKind::KeywordQubit),
        TokenType::KeywordCircuit => Some(TokenKind::KeywordCircuit),
        TokenType::KeywordEntangle => Some(TokenKind::KeywordEntangle),
        TokenType::KeywordNoise => Some(TokenKind::KeywordNoise),
        TokenType::KeywordFidelity => Some(TokenKind::KeywordFidelity),
        TokenType::KeywordSurface => Some(TokenKind::KeywordSurface),
        TokenType::KeywordLogical => Some(TokenKind::KeywordLogical),
        TokenType::KeywordParity => Some(TokenKind::KeywordParity),

        TokenType::KeywordNano => Some(TokenKind::KeywordNano),
        TokenType::KeywordAgent => Some(TokenKind::KeywordAgent),
        TokenType::KeywordRemember => Some(TokenKind::KeywordRemember),
        TokenType::KeywordRecall => Some(TokenKind::KeywordRecall),
        TokenType::KeywordLearn => Some(TokenKind::KeywordLearn),
        TokenType::KeywordInfer => Some(TokenKind::KeywordInfer),
        TokenType::KeywordWisdom => Some(TokenKind::KeywordWisdom),
        TokenType::KeywordSasa => Some(TokenKind::KeywordSasa),
        TokenType::KeywordAncestor => Some(TokenKind::KeywordAncestor),

        TokenType::KeywordAffine => Some(TokenKind::KeywordAffine),
        TokenType::KeywordLinear => Some(TokenKind::KeywordLinear),
        TokenType::KeywordIs => Some(TokenKind::KeywordIs),
        TokenType::KeywordThen => Some(TokenKind::KeywordThen),
        TokenType::KeywordWith => Some(TokenKind::KeywordWith),

        TokenType::KeywordVoid => Some(TokenKind::KeywordVoid),
        TokenType::KeywordInt => Some(TokenKind::KeywordInt),
        TokenType::KeywordFloat => Some(TokenKind::KeywordFloat),
        TokenType::KeywordBool => Some(TokenKind::KeywordBool),
        TokenType::KeywordStr => Some(TokenKind::KeywordStr),
        TokenType::KeywordStringType => Some(TokenKind::KeywordStringType),
        TokenType::KeywordCharType => Some(TokenKind::KeywordCharType),
        TokenType::KeywordPrint => Some(TokenKind::KeywordPrint),
        TokenType::KeywordPrintln => Some(TokenKind::KeywordPrintln),
        TokenType::KeywordAssert => Some(TokenKind::KeywordAssert),
        TokenType::KeywordPanic => Some(TokenKind::KeywordPanic),
        TokenType::KeywordLen => Some(TokenKind::KeywordLen),
        TokenType::KeywordSizeof => Some(TokenKind::KeywordSizeof),

        TokenType::KeywordLanguage => Some(TokenKind::KeywordLanguage),
        TokenType::KeywordMts => Some(TokenKind::KeywordMts),
        TokenType::KeywordSigma => Some(TokenKind::KeywordSigma),
        TokenType::KeywordPi => Some(TokenKind::KeywordPi),
        TokenType::KeywordCode => Some(TokenKind::KeywordCode),
        TokenType::KeywordZamani => Some(TokenKind::KeywordZamani),
        TokenType::KeywordModel => Some(TokenKind::KeywordModel),
        TokenType::KeywordUnsafe => Some(TokenKind::KeywordUnsafe),

        // Tokens that have no canonical equivalent must not be guessed.
        // Their mapping needs an explicit compatibility decision.
        _ => None,
    }
}

/// Assert that one legacy token has the intended canonical identity.
fn assert_compatibility(source: &str, expected: TokenKind) {
    let legacy = legacy_token(source);
    let actual = canonical_kind_for_legacy(&legacy);

    assert_eq!(
        actual,
        Some(expected),
        "legacy token {legacy:?} for source {source:?} must map to {expected:?}"
    );
}

#[test]
fn legacy_core_keywords_keep_their_canonical_identity() {
    let cases = [
        ("let", TokenKind::KeywordLet),
        ("var", TokenKind::KeywordVar),
        ("mut", TokenKind::KeywordMut),
        ("const", TokenKind::KeywordConst),
        ("fn", TokenKind::KeywordFn),
        ("return", TokenKind::KeywordReturn),
        ("if", TokenKind::KeywordIf),
        ("else", TokenKind::KeywordElse),
        ("while", TokenKind::KeywordWhile),
        ("match", TokenKind::KeywordMatch),
        ("true", TokenKind::KeywordTrue),
        ("false", TokenKind::KeywordFalse),
    ];

    for (source, expected) in cases {
        assert_compatibility(source, expected);
    }
}

#[test]
fn legacy_module_and_type_keywords_have_canonical_kinds() {
    let cases = [
        ("module", TokenKind::KeywordModule),
        ("import", TokenKind::KeywordImport),
        ("export", TokenKind::KeywordExport),
        ("use", TokenKind::KeywordUse),
        ("type", TokenKind::KeywordType),
        ("struct", TokenKind::KeywordStruct),
        ("enum", TokenKind::KeywordEnum),
        ("trait", TokenKind::KeywordTrait),
        ("impl", TokenKind::KeywordImpl),
        ("class", TokenKind::KeywordClass),
        ("interface", TokenKind::KeywordInterface),
        ("record", TokenKind::KeywordRecord),
        ("where", TokenKind::KeywordWhere),
    ];

    for (source, expected) in cases {
        assert_compatibility(source, expected);
    }
}

#[test]
fn quantum_vocabulary_does_not_require_operation_names_to_be_keywords() {
    assert_compatibility("quantum", TokenKind::KeywordQuantum);
    assert_compatibility("qubit", TokenKind::KeywordQubit);
    assert_compatibility("circuit", TokenKind::KeywordCircuit);

    // Operation names and vendor extensions should normally remain names.
    // The lexer must not need a new keyword for every future operation.
    assert_compatibility("H", TokenKind::Identifier);
    assert_compatibility("custom_gate", TokenKind::Identifier);
    assert_compatibility("vendor_operation", TokenKind::Identifier);
}

#[test]
fn classical_and_hdl_names_remain_ordinary_identifiers() {
    let names = [
        "value",
        "data_width",
        "register_bank",
        "pipeline_stage",
        "signal_out",
        "clock_domain",
        "tensor_rank",
        "network_endpoint",
        "device_capability",
    ];

    for name in names {
        assert_compatibility(name, TokenKind::Identifier);
    }
}

#[test]
fn representative_literals_keep_their_canonical_kinds() {
    assert_compatibility("42", TokenKind::IntegerLiteral);
    assert_compatibility("3.25", TokenKind::FloatLiteral);
    assert_compatibility("\"zamani\"", TokenKind::StringLiteral);
    assert_compatibility("'x'", TokenKind::CharacterLiteral);
}

#[test]
fn longest_match_operators_keep_their_canonical_kinds() {
    let cases = [
        ("==", TokenKind::Equals),
        ("!=", TokenKind::NotEquals),
        ("<=", TokenKind::LessThanOrEqual),
        (">=", TokenKind::GreaterThanOrEqual),
        ("&&", TokenKind::LogicalAnd),
        ("||", TokenKind::LogicalOr),
        ("<<", TokenKind::LeftShift),
        (">>", TokenKind::RightShift),
        ("->", TokenKind::ThinArrow),
        ("=>", TokenKind::FatArrow),
        ("..", TokenKind::Range),
        ("..=", TokenKind::RangeInclusive),
        ("::", TokenKind::DoubleColon),
        ("+=", TokenKind::PlusAssign),
        ("-=", TokenKind::MinusAssign),
        ("*=", TokenKind::StarAssign),
        ("/=", TokenKind::SlashAssign),
    ];

    for (source, expected) in cases {
        assert_compatibility(source, expected);
    }
}

#[test]
fn single_character_delimiters_preserve_identity() {
    let cases = [
        ("(", TokenKind::LeftParen),
        (")", TokenKind::RightParen),
        ("{", TokenKind::LeftBrace),
        ("}", TokenKind::RightBrace),
        ("[", TokenKind::LeftBracket),
        ("]", TokenKind::RightBracket),
        (",", TokenKind::Comma),
        (".", TokenKind::Dot),
        (";", TokenKind::Semicolon),
        (":", TokenKind::Colon),
        ("@", TokenKind::At),
        ("#", TokenKind::Hash),
        ("?", TokenKind::Question),
        ("~", TokenKind::BitNot),
    ];

    for (source, expected) in cases {
        assert_compatibility(source, expected);
    }
}

#[test]
fn legacy_keyword_classification_matches_canonical_categories() {
    let cases = [
        ("let", TokenKind::KeywordLet),
        ("quantum", TokenKind::KeywordQuantum),
        ("qubit", TokenKind::KeywordQubit),
        ("learn", TokenKind::KeywordLearn),
        ("infer", TokenKind::KeywordInfer),
        ("int", TokenKind::KeywordInt),
    ];

    for (source, expected) in cases {
        assert_compatibility(source, expected);
        assert!(expected.is_keyword(), "{source:?} must be a keyword");
    }
}

#[test]
fn domain_names_are_not_misclassified_as_keywords() {
    let names = [
        "H",
        "X",
        "Y",
        "Z",
        "CNOT",
        "SWAP",
        "custom_gate",
        "vendor",
        "vendor_operation",
        "gpu0",
        "qpu0",
        "fpga0",
        "tensor_compute",
        "network_route",
    ];

    for name in names {
        let legacy = legacy_token(name);
        assert_eq!(
            legacy,
            TokenType::Identifier,
            "domain or hardware name {name:?} should remain an identifier"
        );

        assert_eq!(
            canonical_kind_for_legacy(&legacy),
            Some(TokenKind::Identifier),
            "domain or hardware name {name:?} must have the canonical identifier kind"
        );
    }
}

#[test]
fn canonical_categories_are_domain_neutral() {
    // TokenKind classifies lexical form, not which compiler backend will
    // eventually process the program.
    assert_eq!(
        TokenKind::Identifier.category(),
        crate::frontend::lexer::token_kind::TokenCategory::Identifier
    );
    assert_eq!(
        TokenKind::IntegerLiteral.category(),
        crate::frontend::lexer::token_kind::TokenCategory::Literal
    );
    assert_eq!(
        TokenKind::QuantumLiteral.category(),
        crate::frontend::lexer::token_kind::TokenCategory::Literal
    );
    assert_eq!(
        TokenKind::LeftShift.category(),
        crate::frontend::lexer::token_kind::TokenCategory::Operator
    );
}

#[test]
fn compatibility_mapping_does_not_invent_mappings_for_unreviewed_tokens() {
    // Legacy-only identities must be reviewed explicitly instead of being
    // silently reinterpreted as a different canonical token.
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::EOF),
        None,
        "EOF needs an explicit adapter rule to canonical TokenKind::Eof"
    );
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::Illegal),
        None,
        "Illegal needs an explicit adapter rule to canonical TokenKind::Invalid"
    );
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::SigmaSymbol),
        None,
        "SigmaSymbol needs a documented canonical representation"
    );
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::PiSymbol),
        None,
        "PiSymbol needs a documented canonical representation"
    );
}

#[test]
fn legacy_aliases_are_documented_as_compatibility_cases() {
    // These spellings have legacy identities that differ from the
    // canonical enum's naming. Keep them explicit until the adapter is
    // implemented and its behavior can be tested directly.
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::Arrow),
        Some(TokenKind::ThinArrow)
    );
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::Exclamation),
        Some(TokenKind::LogicalNot)
    );
    assert_eq!(
        canonical_kind_for_legacy(&TokenType::QuestionMark),
        Some(TokenKind::Question)
    );
}

#[test]
fn canonical_token_vocabulary_supports_extended_computing_domains() {
    // These assertions protect representative canonical identities.
    // They do not imply that every spelling is a reserved keyword.
    let classical = TokenKind::Identifier;
    let quantum = TokenKind::QuantumLiteral;
    let literal = TokenKind::IntegerLiteral;
    let operator = TokenKind::LeftShift;

    assert_eq!(classical, TokenKind::Identifier);
    assert_eq!(quantum, TokenKind::QuantumLiteral);
    assert_eq!(literal, TokenKind::IntegerLiteral);
    assert_eq!(operator, TokenKind::LeftShift);
}