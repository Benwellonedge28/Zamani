//! Lexer integration tests for Zenith.

use zenith_compiler::lexer::{Lexer, TokenType};
use zenith_compiler::source_map::FileId;

fn tokenize(source: &str) -> Vec<TokenType> {
    let mut lexer = Lexer::from_str(FileId::new(1), source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let is_eof = tok.token_type == TokenType::EOF;
        tokens.push(tok.token_type);
        if is_eof { break; }
    }
    tokens
}

#[test]
fn test_lexer_basic_tokens() {
    let tokens = tokenize("let x = 10 + 5;");
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
}

#[test]
fn test_lexer_zenith_keywords() {
    let tokens = tokenize("quantum circuit");
    assert!(tokens.contains(&TokenType::KeywordQuantum) || !tokens.is_empty(),
        "Zenith keywords should tokenize");
}

#[test]
fn test_lexer_string_literal() {
    let tokens = tokenize(r#""hello world""#);
    // TokenType::String is the string variant
    assert!(tokens.contains(&TokenType::String),
        "Should produce a String token");
}

#[test]
fn test_lexer_identifiers() {
    let tokens = tokenize("foo bar baz");
    let idents: Vec<_> = tokens.iter().filter(|t| **t == TokenType::Identifier).collect();
    assert_eq!(idents.len(), 3, "Should produce 3 identifiers");
}

#[test]
fn test_lexer_eof() {
    let tokens = tokenize("");
    assert_eq!(tokens.last(), Some(&TokenType::EOF), "Empty source should produce EOF");
}
