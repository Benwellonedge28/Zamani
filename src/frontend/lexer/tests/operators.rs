"src/frontend/lexer/tests/operators.rs"

#![forbid(unsafe_code)]

//! Production conformance tests for Zamani operator recognition.
//!
//! File: src/frontend/lexer/tests/operators.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Normative references:
//! - grammar/lexer/operators.g4
//! - grammar/lexer/operators.md
//! - grammar/lexer/punctuation.g4
//! - grammar/spec/lexical.md
//! - grammar/specification/lexical.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//!
//! Implementation under test:
//! - src/frontend/lexer/operator_registry.rs
//! - src/frontend/lexer/token_kind.rs
//! - src/frontend/lexer/token_metadata.rs
//!
//! Integration:
//! Register this module in src/frontend/lexer/tests/mod.rs:
//!
//!     mod operators;
//!
//! These tests exercise the production operator registry and canonical token
//! kinds. They deliberately do not implement a second operator scanner.
//!
//! Architectural invariants:
//! - Operators are recognized independently of their eventual semantics.
//! - The longest registered operator prefix wins.
//! - Registration order does not affect recognition.
//! - Duplicate spellings are rejected.
//! - Failed batch registration leaves the registry unchanged.
//! - Operator enumeration is deterministic.
//! - Token identities remain distinct from operator spellings.
//! - Unicode operator extensions use UTF-8 byte lengths.
//! - Punctuation cannot be registered as an operator using a punctuation kind.
//! - Resource limits are not encoded as language-level operator limits.
//! - No hardware-specific, quantum-specific, or target-specific assumptions.
//! - Safe Rust only.

use super::super::operator_registry::{
    OperatorDefinition,
    OperatorMatch,
    OperatorRegistry,
    OperatorRegistryError,
};
use super::super::token_kind::{TokenCategory, TokenKind};

/// Construct a valid operator definition for a test.
fn definition(spelling: &str, kind: TokenKind) -> OperatorDefinition {
    OperatorDefinition::new(spelling, kind)
        .unwrap_or_else(|error| {
            panic!(
                "operator definition {spelling:?} with kind {kind:?} \
                 should be valid: {error}"
            )
        })
}

/// Construct a registry containing the operator spellings currently
/// representable by the canonical Rust TokenKind model.
///
/// This is intentionally not a replacement for the normative grammar
/// catalog. The grammar contains additional spellings whose canonical token
/// identities must be added and integrated before those spellings can be
/// registered by the production implementation.
fn supported_operator_registry() -> OperatorRegistry {
    let definitions = [
        ("..=", TokenKind::RangeInclusive),
        ("..", TokenKind::Range),
        ("->", TokenKind::ThinArrow),
        ("=>", TokenKind::FatArrow),
        ("==", TokenKind::Equals),
        ("!=", TokenKind::NotEquals),
        ("<=", TokenKind::LessThanOrEqual),
        (">=", TokenKind::GreaterThanOrEqual),
        ("&&", TokenKind::LogicalAnd),
        ("||", TokenKind::LogicalOr),
        ("<<", TokenKind::LeftShift),
        (">>", TokenKind::RightShift),
        ("+=", TokenKind::PlusAssign),
        ("-=", TokenKind::MinusAssign),
        ("*=", TokenKind::StarAssign),
        ("/=", TokenKind::SlashAssign),
        ("%=", TokenKind::ModuloAssign),
        ("+", TokenKind::Plus),
        ("-", TokenKind::Minus),
        ("*", TokenKind::Star),
        ("/", TokenKind::Slash),
        ("%", TokenKind::Modulo),
        ("=", TokenKind::Assign),
        ("&", TokenKind::BitAnd),
        ("|", TokenKind::BitOr),
        ("^", TokenKind::BitXor),
        ("~", TokenKind::BitNot),
        ("<", TokenKind::LessThan),
        (">", TokenKind::GreaterThan),
        ("!", TokenKind::LogicalNot),
    ];

    let mut registry = OperatorRegistry::new();

    registry
        .register_batch(
            definitions
                .into_iter()
                .map(|(spelling, kind)| definition(spelling, kind)),
        )
        .expect("supported operator definitions should register");

    registry
        .validate()
        .expect("supported operator registry should be internally consistent");

    registry
}

/// Assert that an input prefix resolves to the expected spelling, token kind,
/// and UTF-8 byte length.
fn assert_match(
    registry: &OperatorRegistry,
    input: &str,
    expected_spelling: &str,
    expected_kind: TokenKind,
) {
    let matched = registry
        .match_prefix(input)
        .unwrap_or_else(|| panic!("expected an operator prefix in {input:?}"));

    assert_eq!(matched.spelling(), expected_spelling);
    assert_eq!(matched.kind(), expected_kind);
    assert_eq!(matched.byte_len(), expected_spelling.len());
}

// ---------------------------------------------------------------------------
// Canonical token contracts
// ---------------------------------------------------------------------------

#[test]
fn every_supported_operator_kind_is_classified_as_an_operator() {
    let registry = supported_operator_registry();

    for entry in registry.iter() {
        assert!(
            entry.kind().is_operator(),
            "registered spelling {:?} uses non-operator kind {:?}",
            entry.spelling(),
            entry.kind()
        );

        assert_eq!(
            entry.kind().category(),
            TokenCategory::Operator,
            "registered spelling {:?} has an inconsistent token category",
            entry.spelling()
        );
    }
}

#[test]
fn supported_operator_catalog_has_no_duplicate_spellings() {
    let registry = supported_operator_registry();
    let spellings: Vec<&str> =
        registry.iter().map(|entry| entry.spelling()).collect();

    let mut unique = spellings.clone();
    unique.sort_unstable();
    unique.dedup();

    assert_eq!(
        spellings.len(),
        unique.len(),
        "the operator catalog must not contain duplicate spellings"
    );

    assert_eq!(registry.len(), spellings.len());
}

#[test]
fn every_supported_operator_matches_its_exact_spelling() {
    let registry = supported_operator_registry();

    for entry in registry.iter() {
        let matched = registry
            .match_prefix(entry.spelling())
            .unwrap_or_else(|| {
                panic!(
                    "registered spelling {:?} must match itself",
                    entry.spelling()
                )
            });

        assert_eq!(matched.spelling(), entry.spelling());
        assert_eq!(matched.kind(), entry.kind());
        assert_eq!(matched.byte_len(), entry.spelling().len());
    }
}

#[test]
fn registry_lookup_returns_the_registered_definition() {
    let registry = supported_operator_registry();

    let plus = registry
        .get("+")
        .expect("plus should be registered");

    assert_eq!(plus.spelling(), "+");
    assert_eq!(plus.kind(), TokenKind::Plus);

    assert!(registry.contains_spelling("+"));
    assert!(!registry.contains_spelling("not_an_operator"));
}

#[test]
fn empty_registry_is_valid_and_contains_no_matches() {
    let registry = OperatorRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
    assert!(registry.iter().next().is_none());
    assert!(registry.match_prefix("+").is_none());

    registry
        .validate()
        .expect("an empty registry should be valid");
}

// ---------------------------------------------------------------------------
// Maximal munch / longest-prefix matching
// ---------------------------------------------------------------------------

#[test]
fn compound_comparison_operators_win_over_shorter_prefixes() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        ">=threshold",
        ">=",
        TokenKind::GreaterThanOrEqual,
    );

    assert_match(
        &registry,
        "<=threshold",
        "<=",
        TokenKind::LessThanOrEqual,
    );

    assert_match(
        &registry,
        "==value",
        "==",
        TokenKind::Equals,
    );

    assert_match(
        &registry,
        "!=value",
        "!=",
        TokenKind::NotEquals,
    );
}

#[test]
fn inclusive_range_wins_over_exclusive_range_and_dot_prefixes() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        "..=upper",
        "..=",
        TokenKind::RangeInclusive,
    );

    assert_match(
        &registry,
        "..upper",
        "..",
        TokenKind::Range,
    );

    assert_match(
        &registry,
        ".",
        "..",
        TokenKind::Range,
    );
}

#[test]
fn arrows_win_over_their_single_character_prefixes() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        "->result",
        "->",
        TokenKind::ThinArrow,
    );

    assert_match(
        &registry,
        "=>result",
        "=>",
        TokenKind::FatArrow,
    );
}

#[test]
fn logical_operators_win_over_single_character_prefixes() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        "&&condition",
        "&&",
        TokenKind::LogicalAnd,
    );

    assert_match(
        &registry,
        "||condition",
        "||",
        TokenKind::LogicalOr,
    );
}

#[test]
fn shift_operators_win_over_comparison_prefixes() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        "<<value",
        "<<",
        TokenKind::LeftShift,
    );

    assert_match(
        &registry,
        ">>value",
        ">>",
        TokenKind::RightShift,
    );
}

#[test]
fn compound_assignments_win_over_single_character_operators() {
    let registry = supported_operator_registry();

    for (spelling, kind) in [
        ("+=", TokenKind::PlusAssign),
        ("-=", TokenKind::MinusAssign),
        ("*=", TokenKind::StarAssign),
        ("/=", TokenKind::SlashAssign),
        ("%=", TokenKind::ModuloAssign),
    ] {
        assert_match(&registry, spelling, spelling, kind);
    }
}

#[test]
fn shorter_operator_is_selected_when_no_longer_registered_match_exists() {
    let registry = supported_operator_registry();

    assert_match(&registry, "+value", "+", TokenKind::Plus);
    assert_match(&registry, "-value", "-", TokenKind::Minus);
    assert_match(&registry, "*value", "*", TokenKind::Star);
    assert_match(&registry, "/value", "/", TokenKind::Slash);
    assert_match(&registry, "%value", "%", TokenKind::Modulo);

    assert_match(&registry, "<value", "<", TokenKind::LessThan);
    assert_match(&registry, ">value", ">", TokenKind::GreaterThan);
    assert_match(&registry, "=value", "=", TokenKind::Assign);
    assert_match(&registry, "!value", "!", TokenKind::LogicalNot);
}

#[test]
fn matching_does_not_require_the_entire_input_to_be_an_operator() {
    let registry = supported_operator_registry();

    assert_match(
        &registry,
        "+operand",
        "+",
        TokenKind::Plus,
    );

    assert_match(
        &registry,
        ">=operand",
        ">=",
        TokenKind::GreaterThanOrEqual,
    );

    assert_match(
        &registry,
        "..=end_of_range",
        "..=",
        TokenKind::RangeInclusive,
    );
}

#[test]
fn matching_returns_none_when_no_registered_prefix_exists() {
    let registry = supported_operator_registry();

    for input in [
        "",
        "identifier",
        "λ",
        "word +",
        "word >= value",
    ] {
        assert!(
            registry.match_prefix(input).is_none(),
            "unexpected operator match for {input:?}"
        );
    }
}

#[test]
fn prefix_matching_is_deterministic_across_repeated_calls() {
    let registry = supported_operator_registry();

    let inputs = [
        ">=",
        ">>",
        "..=",
        "..",
        "->",
        "=>",
        "&&",
        "||",
        "+=",
        "-",
        "!",
        "unknown",
    ];

    for input in inputs {
        let first = registry
            .match_prefix(input)
            .map(|matched| {
                (
                    matched.spelling().to_owned(),
                    matched.kind(),
                    matched.byte_len(),
                )
            });

        for _ in 0..8 {
            let repeated = registry
                .match_prefix(input)
                .map(|matched| {
                    (
                        matched.spelling().to_owned(),
                        matched.kind(),
                        matched.byte_len(),
                    )
                });

            assert_eq!(repeated, first, "unstable match for {input:?}");
        }
    }
}

// ---------------------------------------------------------------------------
// Registration order and deterministic enumeration
// ---------------------------------------------------------------------------

#[test]
fn registration_order_does_not_change_maximal_munch() {
    let mut shorter_first = OperatorRegistry::new();

    shorter_first
        .register_batch([
            definition(">", TokenKind::GreaterThan),
            definition(">=", TokenKind::GreaterThanOrEqual),
            definition(".", TokenKind::Plus),
            definition("..", TokenKind::Range),
            definition("..=", TokenKind::RangeInclusive),
        ])
        .expect("first registration order should be valid");

    let mut longer_first = OperatorRegistry::new();

    longer_first
        .register_batch([
            definition("..=", TokenKind::RangeInclusive),
            definition("..", TokenKind::Range),
            definition(".", TokenKind::Plus),
            definition(">=", TokenKind::GreaterThanOrEqual),
            definition(">", TokenKind::GreaterThan),
        ])
        .expect("second registration order should be valid");

    for input in [">=", ">value", "..=", "..", "."] {
        let first = shorter_first
            .match_prefix(input)
            .map(|matched| (matched.spelling(), matched.kind()));

        let second = longer_first
            .match_prefix(input)
            .map(|matched| (matched.spelling(), matched.kind()));

        assert_eq!(
            first, second,
            "registration order changed the result for {input:?}"
        );
    }
}

#[test]
fn iteration_uses_deterministic_spelling_order() {
    let registry = supported_operator_registry();

    let spellings: Vec<&str> =
        registry.iter().map(|entry| entry.spelling()).collect();

    let mut sorted = spellings.clone();
    sorted.sort_unstable();

    assert_eq!(
        spellings, sorted,
        "operator enumeration must use deterministic spelling order"
    );
}

#[test]
fn validation_succeeds_after_all_supported_operators_are_registered() {
    let registry = supported_operator_registry();

    registry
        .validate()
        .expect("registry and trie should remain consistent");
}

// ---------------------------------------------------------------------------
// Invalid definitions and duplicate registration
// ---------------------------------------------------------------------------

#[test]
fn rejects_empty_operator_spellings() {
    assert!(matches!(
        OperatorDefinition::new("", TokenKind::Plus),
        Err(OperatorRegistryError::EmptySpelling)
    ));
}

#[test]
fn rejects_whitespace_inside_operator_spellings() {
    assert!(matches!(
        OperatorDefinition::new("+ =", TokenKind::Plus),
        Err(OperatorRegistryError::WhitespaceInSpelling { .. })
    ));

    assert!(matches!(
        OperatorDefinition::new(" +", TokenKind::Plus),
        Err(OperatorRegistryError::WhitespaceInSpelling { .. })
    ));
}

#[test]
fn rejects_control_characters_inside_operator_spellings() {
    assert!(matches!(
        OperatorDefinition::new("+\n", TokenKind::Plus),
        Err(OperatorRegistryError::ControlCharacterInSpelling { .. })
    ));

    assert!(matches!(
        OperatorDefinition::new("\u{0000}", TokenKind::Plus),
        Err(OperatorRegistryError::ControlCharacterInSpelling { .. })
    ));
}

#[test]
fn rejects_token_kinds_that_are_not_operators() {
    for kind in [
        TokenKind::Identifier,
        TokenKind::KeywordFn,
        TokenKind::IntegerLiteral,
        TokenKind::LeftParen,
        TokenKind::Dot,
        TokenKind::Eof,
        TokenKind::Invalid,
    ] {
        assert!(
            !kind.is_operator(),
            "test kind {kind:?} should not be an operator"
        );

        assert!(
            matches!(
                OperatorDefinition::new("custom", kind),
                Err(OperatorRegistryError::NonOperatorTokenKind { .. })
            ),
            "registry accepted non-operator token kind {kind:?}"
        );
    }
}

#[test]
fn rejects_duplicate_spellings_without_changing_existing_entry() {
    let mut registry = supported_operator_registry();
    let original_len = registry.len();

    let error = registry
        .register(definition("+", TokenKind::Plus))
        .expect_err("duplicate spelling should be rejected");

    assert!(matches!(
        error,
        OperatorRegistryError::DuplicateSpelling { .. }
    ));

    assert_eq!(registry.len(), original_len);
    assert_eq!(
        registry.get("+").map(OperatorDefinition::kind),
        Some(TokenKind::Plus)
    );

    registry
        .validate()
        .expect("registry should remain valid after rejected registration");
}

#[test]
fn failed_batch_registration_is_atomic() {
    let mut registry = supported_operator_registry();
    let original_len = registry.len();

    let result = registry.register_batch([
        definition("custom_operator", TokenKind::Plus),
        definition("+", TokenKind::Plus),
    ]);

    assert!(
        matches!(
            result,
            Err(OperatorRegistryError::DuplicateSpelling { .. })
        ),
        "batch containing a duplicate must fail"
    );

    assert_eq!(registry.len(), original_len);
    assert!(!registry.contains_spelling("custom_operator"));
    assert!(registry.contains_spelling("+"));

    registry
        .validate()
        .expect("failed batch must leave registry consistent");
}

#[test]
fn failed_batch_does_not_partially_register_new_prefixes() {
    let mut registry = OperatorRegistry::new();

    registry
        .register(definition(">", TokenKind::GreaterThan))
        .expect("initial definition should register");

    let result = registry.register_batch([
        definition(">=", TokenKind::GreaterThanOrEqual),
        definition(">", TokenKind::GreaterThan),
    ]);

    assert!(matches!(
        result,
        Err(OperatorRegistryError::DuplicateSpelling { .. })
    ));

    assert_eq!(registry.len(), 1);
    assert!(!registry.contains_spelling(">="));

    assert_match(
        &registry,
        ">=value",
        ">",
        TokenKind::GreaterThan,
    );

    registry
        .validate()
        .expect("registry should remain consistent after atomic failure");
}

#[test]
fn rejects_empty_descriptions() {
    let result = definition("+", TokenKind::Plus)
        .with_description("   ");

    assert!(matches!(
        result,
        Err(OperatorRegistryError::EmptyDescription { .. })
    ));
}

#[test]
fn accepts_non_empty_operator_descriptions() {
    let operator = definition("+", TokenKind::Plus)
        .with_description("Addition operator")
        .expect("non-empty description should be accepted");

    assert_eq!(operator.description(), Some("Addition operator"));
    assert_eq!(operator.spelling(), "+");
    assert_eq!(operator.kind(), TokenKind::Plus);
}

// ---------------------------------------------------------------------------
// Extensibility and UTF-8 correctness
// ---------------------------------------------------------------------------

#[test]
fn registry_can_accept_an_extension_using_an_existing_operator_kind() {
    let mut registry = supported_operator_registry();
    let original_len = registry.len();

    registry
        .register(
            definition("⊕", TokenKind::BitXor)
                .with_description("Example extension spelling")
                .expect("description should be valid")
        )
        .expect("extension should register against an existing operator kind");

    assert_eq!(registry.len(), original_len + 1);

    assert_match(
        &registry,
        "⊕value",
        "⊕",
        TokenKind::BitXor,
    );
}

#[test]
fn unicode_operator_match_length_is_measured_in_utf8_bytes() {
    let mut registry = OperatorRegistry::new();

    registry
        .register(definition("⊕", TokenKind::BitXor))
        .expect("Unicode operator spelling should register");

    let matched = registry
        .match_prefix("⊕operand")
        .expect("Unicode operator should match");

    assert_eq!(matched.spelling(), "⊕");
    assert_eq!(matched.kind(), TokenKind::BitXor);
    assert_eq!(matched.byte_len(), "⊕".len());
    assert_eq!(matched.byte_len(), 3);
}

#[test]
fn longer_extension_spelling_wins_over_its_registered_prefix() {
    let mut registry = OperatorRegistry::new();

    registry
        .register_batch([
            definition("⊕", TokenKind::BitXor),
            definition("⊕=", TokenKind::PlusAssign),
        ])
        .expect("extension spellings should register");

    assert_match(
        &registry,
        "⊕=value",
        "⊕=",
        TokenKind::PlusAssign,
    );

    assert_match(
        &registry,
        "⊕value",
        "⊕",
        TokenKind::BitXor,
    );
}

// ---------------------------------------------------------------------------
// Punctuation ownership and cross-module conformance
// ---------------------------------------------------------------------------

#[test]
fn punctuation_kinds_cannot_be_registered_as_operators() {
    // The operator grammar assigns operator ownership to several spellings
    // whose current TokenKind mapping is inconsistent. Until those mappings
    // are corrected, the registry must reject punctuation kinds rather than
    // silently treating them as operators.
    for (spelling, kind) in [
        ("::", TokenKind::DoubleColon),
        (".", TokenKind::Dot),
        ("?", TokenKind::Question),
        ("~", TokenKind::Tilde),
    ] {
        assert!(
            !kind.is_operator(),
            "punctuation kind {kind:?} unexpectedly classified as operator"
        );

        assert!(
            matches!(
                OperatorDefinition::new(spelling, kind),
                Err(OperatorRegistryError::NonOperatorTokenKind { .. })
            ),
            "punctuation spelling {spelling:?} must not bypass token-kind validation"
        );
    }
}

#[test]
fn operator_tokens_are_not_keywords_or_delimiters() {
    for kind in [
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Assign,
        TokenKind::Equals,
        TokenKind::LogicalAnd,
        TokenKind::Range,
        TokenKind::RangeInclusive,
        TokenKind::ThinArrow,
        TokenKind::FatArrow,
    ] {
        assert!(kind.is_operator());
        assert!(!kind.is_keyword());
        assert!(!kind.is_delimiter());
        assert!(!kind.is_eof());
        assert!(!kind.is_invalid());
    }
}

// ---------------------------------------------------------------------------
// Scalability and resource-policy boundaries
// ---------------------------------------------------------------------------

#[test]
fn registry_does_not_impose_a_fixed_operator_count() {
    let mut registry = OperatorRegistry::new();

    // Generate distinct spellings from a finite test alphabet. The test
    // verifies dynamic registration without introducing a production limit.
    let mut registered = Vec::new();

    for first in 'a'..='z' {
        for second in 'a'..='z' {
            let spelling = format!("{first}{second}");
            registered.push(spelling);
        }
    }

    registry
        .register_batch(
            registered
                .iter()
                .map(|spelling| definition(spelling, TokenKind::Plus)),
        )
        .expect("distinct extension spellings should register");

    assert_eq!(registry.len(), registered.len());

    for spelling in &registered {
        let matched = registry
            .match_prefix(spelling)
            .expect("every registered spelling should match");

        assert_eq!(matched.spelling(), spelling);
        assert_eq!(matched.kind(), TokenKind::Plus);
    }

    registry
        .validate()
        .expect("large test registry should remain consistent");
}

#[test]
fn long_operator_spellings_are_not_rejected_by_an_arbitrary_language_limit() {
    let mut registry = OperatorRegistry::new();
    let spelling = format!("~{}", "x".repeat(4096));

    registry
        .register(definition(&spelling, TokenKind::BitNot))
        .expect("valid long spelling should not hit a hard-coded language limit");

    let matched = registry
        .match_prefix(&spelling)
        .expect("long registered spelling should match");

    assert_eq!(matched.spelling(), spelling);
    assert_eq!(matched.byte_len(), spelling.len());
}

#[test]
fn registration_and_matching_remain_consistent_after_many_extensions() {
    let mut registry = supported_operator_registry();

    let spellings = [
        "%%",
        "%%%",
        "&&&",
        "||||",
        "^^^",
        "!!",
        "!!!",
    ];

    for spelling in spellings {
        registry
            .register(definition(spelling, TokenKind::BitXor))
            .unwrap_or_else(|error| {
                panic!("extension {spelling:?} should register: {error}")
            });
    }

    registry
        .validate()
        .expect("registry must remain consistent after extension registration");

    assert_match(&registry, "%%%%", "%%%", TokenKind::BitXor);
    assert_match(&registry, "&&&value", "&&&", TokenKind::BitXor);
    assert_match(&registry, "||||value", "||||", TokenKind::BitXor);
    assert_match(&registry, "^^^^value", "^^^", TokenKind::BitXor);
    assert_match(&registry, "!!!value", "!!!", TokenKind::BitXor);
}

/// Compile-time type assertion that the matching API returns the expected
/// borrowed-match type. This also guards against accidentally changing the
/// API to an unrelated owned representation.
#[allow(dead_code)]
fn operator_match_api_contract<'a>(
    registry: &'a OperatorRegistry,
    input: &str,
) -> Option<OperatorMatch<'a>> {
    registry.match_prefix(input)
}

