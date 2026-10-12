
#![forbid(unsafe_code)]

//! Zamani frontend — data-domain syntax boundary.
//!
//! Intended path:
//!     src/frontend/parser/domains/data.rs
//!
//! # Purpose
//!
//! Identify and preserve data-domain constructs in the canonical lexer stream
//! so the shared parser can dispatch them to the grammar productions defined
//! by `grammar/data/*.g4`.
//!
//! # Ownership
//!
//! This module owns:
//! - data-construct classification at the parser boundary;
//! - loss-preserving token-range identification;
//! - iterative delimiter validation for a construct region;
//! - structured boundary-scanning diagnostics;
//! - caller-configurable safety budgets for this scan.
//!
//! This module does not own:
//! - lexical analysis or token definitions;
//! - the authoritative Zamani grammar;
//! - expression, declaration, or type parsing;
//! - canonical AST definitions or AST construction;
//! - schema validation or data execution;
//! - SQL, JSON, XML, or other embedded-language parsing;
//! - tensor computation, data loading, or target selection;
//! - semantic analysis, resource allocation, or IR lowering.
//!
//! # Grammar authority
//!
//! `grammar/data/data.g4` owns the composition boundary for data syntax.
//! Specialized grammar files own their respective productions, including
//! collections, datasets, records, schemas, tables, streams, queries,
//! pipelines, and transformations.
//!
//! This Rust module is an adapter, not a replacement for those productions.
//!
//! # Scalability
//!
//! There are no hardware-capacity or language-level collection-size limits.
//! The delimiter stack grows according to the input and available memory.
//! Callers may supply resource budgets appropriate to their compilation
//! environment. Exhausting a configured budget is reported as an error.
//!
//! # Integration contract
//!
//! The shared parser should:
//! 1. pass a data-only token region to `parse_data_region`;
//! 2. inspect `DataConstruct::kind` and `token_range`;
//! 3. dispatch each range to the matching shared grammar production;
//! 4. convert the resulting syntax into canonical AST nodes through
//!    `ast_adapter.rs`;
//! 5. retain source spans and propagate diagnostics.
//!
//! The token ranges refer to the exact input slice supplied to the function.
//! They are not global source offsets or AST node identifiers.

use std::ops::Range;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

/// Recognized data-language construct families.
///
/// `Extension` deliberately permits an open vocabulary. Adding a new
/// registered data construct does not require adding a new enum variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DataConstructKind {
    Data,
    Record,
    Schema,
    Collection,
    Dataset,
    Table,
    Stream,
    Graph,
    Query,
    Pipeline,
    Transformation,
    Serialization,
    Deserialization,
    Extension(String),
}

impl DataConstructKind {
    /// Classify a source spelling using the built-in data grammar vocabulary.
    ///
    /// Matching is ASCII case-insensitive for compatibility with keyword
    /// spellings. The original spelling is always preserved in the token.
    #[must_use]
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword.to_ascii_lowercase().as_str() {
            "data" => Some(Self::Data),
            "record" => Some(Self::Record),
            "schema" => Some(Self::Schema),
            "collection" => Some(Self::Collection),
            "dataset" => Some(Self::Dataset),
            "table" => Some(Self::Table),
            "stream" => Some(Self::Stream),
            "graph" => Some(Self::Graph),
            "query" => Some(Self::Query),
            "pipeline" => Some(Self::Pipeline),
            "transform" | "transformation" => Some(Self::Transformation),
            "serialize" | "serialization" => Some(Self::Serialization),
            "deserialize" | "deserialization" => Some(Self::Deserialization),
            _ => None,
        }
    }
}

/// Caller-configurable limits for this boundary scan.
///
/// `None` means this module does not impose that particular budget. In a
/// production parser, the shared `ParserLimits` should normally supply the
/// values so budgets are coordinated across the whole parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataScanConfig {
    /// Maximum number of tokens to examine in one data region.
    pub max_tokens: Option<usize>,

    /// Maximum simultaneous open delimiters.
    pub max_nesting: Option<usize>,

    /// Additional construct keywords registered by the caller.
    ///
    /// These names extend recognition only. A registered name does not
    /// automatically define the syntax or semantics of that construct.
    pub extension_keywords: Vec<String>,
}

impl Default for DataScanConfig {
    fn default() -> Self {
        Self {
            max_tokens: None,
            max_nesting: None,
            extension_keywords: Vec::new(),
        }
    }
}

/// A recognized construct in the supplied token slice.
///
/// `token_range` is a half-open range into the exact slice supplied to
/// `parse_data_region`. `keyword_span` identifies the initial keyword;
/// `last_span` identifies the final token in the construct.
///
/// The full source text is not copied and the original tokens remain the
/// authoritative representation of the construct's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataConstruct {
    pub kind: DataConstructKind,
    pub keyword: String,
    pub token_range: Range<usize>,
    pub keyword_span: Span,
    pub last_span: Span,
}

/// Structured error category for data-region scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataScanErrorKind {
    UnexpectedToken,
    MismatchedDelimiter {
        expected: TokenKind,
        found: TokenKind,
    },
    UnterminatedDelimiter {
        expected: TokenKind,
    },
    MissingTerminator,
    NestingBudgetExceeded {
        budget: usize,
    },
    TokenBudgetExceeded {
        budget: usize,
    },
    InvalidToken,
    TokensAfterEof,
}

/// An error tied to the token that caused or revealed the problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataScanError {
    pub kind: DataScanErrorKind,
    pub token_index: usize,
    pub span: Span,
    pub message: String,
}

/// Result of scanning a data-only token region.
///
/// A report may contain partial constructs and errors. A caller must not
/// treat a report containing errors as a fully valid parse.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataScanReport {
    pub constructs: Vec<DataConstruct>,
    pub errors: Vec<DataScanError>,
    pub consumed_tokens: usize,
    pub reached_eof: bool,
}

impl DataScanReport {
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
}

#[derive(Debug, Clone, Copy)]
struct OpenDelimiter {
    opener: TokenKind,
    expected_closer: TokenKind,
    span: Span,
}

/// Scan a token slice that is expected to contain only data-domain constructs.
///
/// This function deliberately does not tokenize source text. The caller must
/// pass tokens from Zamani's canonical lexer, in source order.
///
/// Trivia-channel tokens are ignored for classification but remain inside
/// each construct's token range. The function stops at the first EOF token.
///
/// # Recovery
///
/// On an unexpected top-level token, scanning advances by one token and
/// continues. On a malformed construct, the scanner attempts to resume at a
/// later construct boundary. The original token slice is never modified.
///
/// # Budgets
///
/// `config.max_tokens` and `config.max_nesting` are optional safeguards, not
/// language-level limits. They should be coordinated with the shared parser's
/// resource-budget policy.
pub fn parse_data_region(
    tokens: &[Token<'_>],
    config: &DataScanConfig,
) -> DataScanReport {
    let mut report = DataScanReport::default();
    let extensions = config
        .extension_keywords
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .collect::<Vec<_>>();

    let mut cursor = 0usize;

    while cursor < tokens.len() {
        if let Some(budget) = config.max_tokens {
            if cursor >= budget {
                report.errors.push(make_error(
                    tokens,
                    cursor,
                    DataScanErrorKind::TokenBudgetExceeded { budget },
                    "data-region token budget exhausted",
                ));
                report.consumed_tokens = cursor;
                return report;
            }
        }

        let token = &tokens[cursor];

        if token.kind() == TokenKind::Eof {
            report.reached_eof = true;
            report.consumed_tokens = cursor + 1;

            if tokens[cursor + 1..]
                .iter()
                .any(|following| following.kind() != TokenKind::Eof)
            {
                report.errors.push(make_error(
                    tokens,
                    cursor + 1,
                    DataScanErrorKind::TokensAfterEof,
                    "non-EOF tokens occur after end of input",
                ));
            }

            return report;
        }

        if token.channel() != TokenChannel::Default {
            cursor += 1;
            continue;
        }

        if token.kind() == TokenKind::Invalid {
            report.errors.push(make_error(
                tokens,
                cursor,
                DataScanErrorKind::InvalidToken,
                "invalid lexical token in data region",
            ));
            cursor += 1;
            continue;
        }

        let Some(kind) = classify_token(token, &extensions) else {
            report.errors.push(make_error(
                tokens,
                cursor,
                DataScanErrorKind::UnexpectedToken,
                "expected a data-domain construct",
            ));
            cursor += 1;
            continue;
        };

        let start = cursor;
        match scan_construct(tokens, start, config) {
            Ok(end_exclusive) => {
                let last_index = end_exclusive.saturating_sub(1);
                let construct = DataConstruct {
                    kind,
                    keyword: token.lexeme().to_owned(),
                    token_range: start..end_exclusive,
                    keyword_span: *token.span(),
                    last_span: *tokens[last_index].span(),
                };

                report.constructs.push(construct);
                cursor = end_exclusive;
            }
            Err(error) => {
                let recovery_start = cursor;
                report.errors.push(error);

                // Recovery must always advance. Prefer a synchronization
                // boundary at a top-level semicolon or closing brace.
                cursor = recover_construct(tokens, recovery_start);

                if cursor <= recovery_start {
                    cursor = recovery_start.saturating_add(1);
                }
            }
        }
    }

    report.consumed_tokens = cursor;
    report
}

fn classify_token(
    token: &Token<'_>,
    extension_keywords: &[String],
) -> Option<DataConstructKind> {
    if let Some(kind) = DataConstructKind::from_keyword(token.lexeme()) {
        return Some(kind);
    }

    let normalized = token.lexeme().to_ascii_lowercase();
    if extension_keywords.iter().any(|name| name == &normalized) {
        return Some(DataConstructKind::Extension(normalized));
    }

    None
}

fn scan_construct(
    tokens: &[Token<'_>],
    start: usize,
    config: &DataScanConfig,
) -> Result<usize, DataScanError> {
    let keyword = tokens.get(start).ok_or_else(|| {
        make_error(
            tokens,
            start,
            DataScanErrorKind::UnexpectedToken,
            "construct start is outside the token slice",
        )
    })?;

    let mut stack: Vec<OpenDelimiter> = Vec::new();
    let mut saw_block_body = false;
    let mut cursor = start + 1;

    while cursor < tokens.len() {
        if let Some(budget) = config.max_tokens {
            if cursor >= budget {
                return Err(make_error(
                    tokens,
                    cursor,
                    DataScanErrorKind::TokenBudgetExceeded { budget },
                    "data construct exceeded the token budget",
                ));
            }
        }

        let token = &tokens[cursor];

        if token.kind() == TokenKind::Eof {
            if let Some(open) = stack.last() {
                return Err(make_error(
                    tokens,
                    cursor,
                    DataScanErrorKind::UnterminatedDelimiter {
                        expected: open.expected_closer,
                    },
                    "end of input reached before closing a delimiter",
                ));
            }

            return Err(make_error(
                tokens,
                cursor,
                DataScanErrorKind::MissingTerminator,
                "data construct is missing a semicolon or a closed block",
            ));
        }

        if token.channel() != TokenChannel::Default {
            cursor += 1;
            continue;
        }

        match token.kind() {
            TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => {
                let expected_closer = matching_closer(token.kind()).expect(
                    "opening delimiter must have a matching closing delimiter",
                );

                if token.kind() == TokenKind::LeftBrace && stack.is_empty() {
                    saw_block_body = true;
                }

                if let Some(budget) = config.max_nesting {
                    if stack.len() >= budget {
                        return Err(make_error(
                            tokens,
                            cursor,
                            DataScanErrorKind::NestingBudgetExceeded { budget },
                            "data construct exceeded the configured nesting budget",
                        ));
                    }
                }

                stack.push(OpenDelimiter {
                    opener: token.kind(),
                    expected_closer,
                    span: *token.span(),
                });
            }

            TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::RightBrace => {
                let Some(open) = stack.pop() else {
                    return Err(make_error(
                        tokens,
                        cursor,
                        DataScanErrorKind::MismatchedDelimiter {
                            expected: TokenKind::Eof,
                            found: token.kind(),
                        },
                        "closing delimiter has no matching opener",
                    ));
                };

                if open.expected_closer != token.kind() {
                    return Err(DataScanError {
                        kind: DataScanErrorKind::MismatchedDelimiter {
                            expected: open.expected_closer,
                            found: token.kind(),
                        },
                        token_index: cursor,
                        span: *token.span(),
                        message: format!(
                            "closing delimiter {:?} does not match opener {:?} at byte {}",
                            token.kind(),
                            open.opener,
                            open.span.start().byte_offset(),
                        ),
                    });
                }

                if stack.is_empty() && saw_block_body {
                    let mut end = cursor + 1;

                    // Permit an optional semicolon after a declaration block.
                    if tokens
                        .get(end)
                        .is_some_and(|next| next.kind() == TokenKind::Semicolon)
                    {
                        end += 1;
                    }

                    return Ok(end);
                }
            }

            TokenKind::Semicolon if stack.is_empty() => return Ok(cursor + 1),

            _ => {}
        }

        cursor += 1;
    }

    Err(DataScanError {
        kind: DataScanErrorKind::MissingTerminator,
        token_index: tokens.len(),
        span: *tokens
            .last()
            .map(Token::span)
            .unwrap_or(keyword.span()),
        message: "data construct reached the end of the token slice without a terminator"
            .to_owned(),
    })
}

fn matching_closer(kind: TokenKind) -> Option<TokenKind> {
    match kind {
        TokenKind::LeftParen => Some(TokenKind::RightParen),
        TokenKind::LeftBracket => Some(TokenKind::RightBracket),
        TokenKind::LeftBrace => Some(TokenKind::RightBrace),
        _ => None,
    }
}

fn recover_construct(tokens: &[Token<'_>], start: usize) -> usize {
    let mut stack: Vec<TokenKind> = Vec::new();
    let mut cursor = start.saturating_add(1);

    while cursor < tokens.len() {
        let token = &tokens[cursor];

        if token.kind() == TokenKind::Eof {
            return cursor;
        }

        if token.channel() != TokenChannel::Default {
            cursor += 1;
            continue;
        }

        match token.kind() {
            TokenKind::LeftParen => stack.push(TokenKind::RightParen),
            TokenKind::LeftBracket => stack.push(TokenKind::RightBracket),
            TokenKind::LeftBrace => stack.push(TokenKind::RightBrace),

            TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::RightBrace => {
                if stack.last().copied() == Some(token.kind()) {
                    stack.pop();
                } else if stack.is_empty() {
                    return cursor + 1;
                }
            }

            TokenKind::Semicolon if stack.is_empty() => return cursor + 1,

            _ => {}
        }

        cursor += 1;
    }

    cursor
}

fn make_error(
    tokens: &[Token<'_>],
    index: usize,
    kind: DataScanErrorKind,
    message: &str,
) -> DataScanError {
    let span = tokens
        .get(index)
        .or_else(|| tokens.last())
        .map(Token::span)
        .copied()
        .unwrap_or_else(|| {
            // Empty token slices have no source position to preserve.
            // Callers should normally supply the lexer's synthetic EOF token.
            // This fallback is only used for defensive handling of invalid API
            // calls; no fabricated source coordinate is introduced.
            Span::default()
        });

    DataScanError {
        kind,
        token_index: index,
        span,
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_existing_data_construct_families() {
        assert_eq!(
            DataConstructKind::from_keyword("record"),
            Some(DataConstructKind::Record)
        );
        assert_eq!(
            DataConstructKind::from_keyword("SCHEMA"),
            Some(DataConstructKind::Schema)
        );
        assert_eq!(
            DataConstructKind::from_keyword("collection"),
            Some(DataConstructKind::Collection)
        );
        assert_eq!(
            DataConstructKind::from_keyword("dataset"),
            Some(DataConstructKind::Dataset)
        );
        assert_eq!(
            DataConstructKind::from_keyword("table"),
            Some(DataConstructKind::Table)
        );
    }

    #[test]
    fn does_not_claim_unknown_constructs_without_registration() {
        assert_eq!(DataConstructKind::from_keyword("future_store"), None);
    }

    #[test]
    fn allows_open_extension_vocabulary() {
        let extensions = vec!["future_store".to_owned()];
        let token = Token::borrowed(
            TokenKind::Identifier,
            "future_store",
            Span::default(),
        );

        assert_eq!(
            classify_token(&token, &extensions),
            Some(DataConstructKind::Extension("future_store".to_owned()))
        );
    }

    #[test]
    fn delimiter_mapping_is_symmetric() {
        assert_eq!(
            matching_closer(TokenKind::LeftParen),
            Some(TokenKind::RightParen)
        );
        assert_eq!(
            matching_closer(TokenKind::LeftBracket),
            Some(TokenKind::RightBracket)
        );
        assert_eq!(
            matching_closer(TokenKind::LeftBrace),
            Some(TokenKind::RightBrace)
        );
        assert_eq!(matching_closer(TokenKind::Identifier), None);
    }
}
