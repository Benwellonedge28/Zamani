
//! Conceptual Tests: Lexer
//!
//! This module provides conceptual unit tests for the Zenith Lexer.
//! It verifies that the lexer correctly tokenizes various Zenith, Sankofa,
//! and Nimbus language constructs, including keywords, literals (classical,
//! quantum, nano, MTS), operators, identifiers, and comments.

use zenith_compiler::lexer::{Lexer, TokenType};
use zenith_compiler::source_map::{FileId, Span, BytePos};

// Helper function for creating a dummy span for tests
fn dummy_span() -> Span {
    Span::new(FileId::new(1), BytePos(0), BytePos(0), 1, 1)
}

#[test]
fn test_lexer_basic_tokens() {
    let source = "let x = 10 + 5;";
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert_eq!(tokens, vec![
        TokenType::KeywordLet,
        TokenType::Identifier,
        TokenType::Assign,
        TokenType::Integer,
        TokenType::Plus,
        TokenType::Integer,
        TokenType::Semicolon,
        TokenType::EOF,
    ]);
    assert!(lexer.get_errors().is_empty());
}

#[test]
fn test_lexer_zenith_keywords() {
    let source = "quantum circuit Q { linear affine handle effect }";
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert_eq!(tokens, vec![
        TokenType::KeywordQuantum,
        TokenType::KeywordCircuit,
        TokenType::Identifier, // Q
        TokenType::LBrace,
        TokenType::KeywordLinear,
        TokenType::KeywordAffine,
        TokenType::KeywordHandle,
        TokenType::KeywordEffect,
        TokenType::RBrace,
        TokenType::EOF,
    ]);
    assert!(lexer.get_errors().is_empty());
}

#[test]
fn test_lexer_sankofa_keywords() {
    let source = "remember recall learn wisdom zamani sasa ancestor";
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert_eq!(tokens, vec![
        TokenType::KeywordRemember,
        TokenType::KeywordRecall,
        TokenType::KeywordLearn,
        TokenType::KeywordWisdom,
        TokenType::KeywordZamani,
        TokenType::KeywordSasa,
        TokenType::KeywordAncestor,
        TokenType::EOF,
    ]);
    assert!(lexer.get_errors().is_empty());
}

#[test]
fn test_lexer_special_literals() {
    let source = r#"|0⟩ @atom(foo) mts[42] "hello" 'c' 1.23 Π(x:T)T Σ(x:T)T"#;
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert_eq!(tokens, vec![
        TokenType::QuantumLiteral,
        TokenType::NanoAnnotation,
        TokenType::MTSLiteral,
        TokenType::String,
        TokenType::Char,
        TokenType::Float,
        TokenType::PiSymbol,
        TokenType::LParen,
        TokenType::Identifier,
        TokenType::Colon,
        TokenType::Identifier,
        TokenType::RParen,
        TokenType::Identifier, // T
        TokenType::SigmaSymbol,
        TokenType::LParen,
        TokenType::Identifier,
        TokenType::Colon,
        TokenType::Identifier,
        TokenType::RParen,
        TokenType::Identifier, // T
        TokenType::EOF,
    ]);
    assert!(lexer.get_errors().is_empty());
}

#[test]
fn test_lexer_comments_and_whitespace() {
    let source = r#"
        // Single-line comment
        let x = /* Multi-line comment */ 10; // End comment
        /*
         * Another
         * Multi-line
         * Comment
         */
    "#;
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert_eq!(tokens, vec![
        TokenType::KeywordLet,
        TokenType::Identifier,
        TokenType::Assign,
        TokenType::Integer,
        TokenType::Semicolon,
        TokenType::EOF,
    ]);
    assert!(lexer.get_errors().is_empty());
}

#[test]
fn test_lexer_error_handling() {
    let source = "let x = `bad_char`;"; // ` is an illegal character
    let mut lexer = Lexer::new(FileId::new(1), source);
    let tokens: Vec<TokenType> = lexer.map(|t| t.token_type).collect();
    assert!(!lexer.get_errors().is_empty(), "Lexer should report errors for illegal characters.");
    assert_eq!(lexer.get_errors().len(), 1);
    assert!(tokens.contains(&TokenType::Illegal));
}
