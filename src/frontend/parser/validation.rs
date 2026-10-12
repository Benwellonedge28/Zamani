
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Parser-boundary validation for the Zamani frontend.
//!
//! # Responsibility
//!
//! This module validates properties that can be established at the parser
//! boundary:
//!
//! - token-stream ordering and EOF invariants;
//! - source-range bounds and UTF-8 boundaries when source text is available;
//! - delimiter pairing;
//! - invalid tokens;
//! - full-input consumption;
//! - delegation to the canonical AST structural validator.
//!
//! # Ownership boundaries
//!
//! This module does not:
//!
//! - tokenize source text;
//! - define another token, span, or AST representation;
//! - parse grammar productions;
//! - resolve identifiers or names;
//! - type-check expressions;
//! - authorize effects or capabilities;
//! - evaluate contracts;
//! - negotiate resources or select hardware;
//! - validate quantum-operation semantics;
//! - lower syntax into an intermediate representation.
//!
//! Lexical token identity comes from `frontend::lexer::token_kind`.
//! Token ranges come from `frontend::lexer::span`.
//! AST graph validation belongs to `frontend::ast::node::validation`.
//!
//! # Scalability
//!
//! There are no language-level limits on source size, token count, nesting,
//! qubit count, register width, device count, tensor rank, or computational
//! domain count.
//!
//! Optional diagnostic limits are caller-selected operational policies.
//! Delimiter checking is iterative rather than recursive. Actual processing
//! remains bounded by the available memory, address space, and execution
//! resources.
//!
//! # Integration contract
//!
//! `parser.rs` and `session.rs` may call `validate_token_stream` after
//! obtaining tokens from the canonical lexer.
//!
//! The parser should call `validate_full_consumption` when an entry point
//! requires a complete input rather than a valid prefix.
//!
//! AST construction code should call `validate_ast_fragment` or
//! `validate_program_ast` with the repository's canonical AST provider.
//!
//! The parser must not silently convert validation failures into successful
//! parses. Callers decide whether a report is fatal, recoverable, or suitable
//! for tooling.

use core::fmt;

use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

/// Classification of a parser-boundary validation finding.
///
/// Variants describe structural or input-integrity issues, not semantic
/// errors. Additions to this enum should preserve the meaning of existing
/// variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ValidationIssueKind {
    /// The token stream has no end-of-input token when one is required.
    MissingEof,

    /// The token stream contains more than one end-of-input token.
    MultipleEof,

    /// A non-EOF token appears after the first EOF token.
    TokensAfterEof,

    /// A token's start offset precedes the previous token's start offset.
    NonMonotonicTokenSpans,

    /// A token's end offset precedes its start offset.
    InvalidTokenSpan,

    /// A token's span extends past the supplied source text.
    SpanOutOfBounds,

    /// A token span endpoint is not a UTF-8 character boundary.
    InvalidUtf8Boundary,

    /// The lexer emitted a token marked as invalid.
    InvalidToken,

    /// A closing delimiter has no matching opening delimiter.
    UnexpectedClosingDelimiter,

    /// A closing delimiter does not match the most recent opening delimiter.
    MismatchedDelimiter,

    /// An opening delimiter has no matching closing delimiter.
    UnclosedDelimiter,

    /// The parser stopped before consuming all significant input.
    TrailingInput,

    /// The parser reported a cursor position beyond the token stream.
    InvalidConsumptionPosition,
}

impl fmt::Display for ValidationIssueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingEof => "missing end-of-input token",
            Self::MultipleEof => "multiple end-of-input tokens",
            Self::TokensAfterEof => "tokens found after end of input",
            Self::NonMonotonicTokenSpans => "token spans are not ordered",
            Self::InvalidTokenSpan => "invalid token source range",
            Self::SpanOutOfBounds => "token source range exceeds source length",
            Self::InvalidUtf8Boundary => {
                "token source range is not on a UTF-8 boundary"
            }
            Self::InvalidToken => "invalid lexical token",
            Self::UnexpectedClosingDelimiter => {
                "closing delimiter has no matching opener"
            }
            Self::MismatchedDelimiter => {
                "closing delimiter does not match its opener"
            }
            Self::UnclosedDelimiter => "opening delimiter is not closed",
            Self::TrailingInput => "input remains after the parsed construct",
            Self::InvalidConsumptionPosition => {
                "parser consumption position is outside the token stream"
            }
        };

        formatter.write_str(message)
    }
}

/// One machine-readable validation finding.
///
/// `token_index` is the zero-based index into the supplied token slice.
/// `start` and `end` are zero-based UTF-8 byte offsets when a token span is
/// available. These offsets are absent for stream-level findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// Classification of the finding.
    pub kind: ValidationIssueKind,

    /// Index of the relevant token, if applicable.
    pub token_index: Option<usize>,

    /// Start byte offset, if applicable.
    pub start: Option<usize>,

    /// End byte offset, if applicable.
    pub end: Option<usize>,

    /// Optional expected token or delimiter name.
    pub expected: Option<String>,

    /// Optional actual token or delimiter name.
    pub found: Option<String>,

    /// Human-readable explanation.
    pub message: String,
}

impl ValidationIssue {
    fn stream(kind: ValidationIssueKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            token_index: None,
            start: None,
            end: None,
            expected: None,
            found: None,
            message: message.into(),
        }
    }

    fn at_token(
        kind: ValidationIssueKind,
        token_index: usize,
        start: usize,
        end: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            token_index: Some(token_index),
            start: Some(start),
            end: Some(end),
            expected: None,
            found: None,
            message: message.into(),
        }
    }

    fn with_delimiters(
        mut self,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        self.expected = Some(expected.into());
        self.found = Some(found.into());
        self
    }
}

/// Policy for parser-boundary validation.
///
/// The default is intended for ordinary parser integration:
///
/// - require an EOF token;
/// - validate source bounds when source text is supplied;
/// - validate UTF-8 boundaries when source text is supplied;
/// - check token ordering;
/// - check delimiter balance;
/// - report invalid lexical tokens;
/// - do not impose a diagnostic-count limit.
///
/// `max_issues` bounds the number of stored findings, not the size of a
/// valid Zamani program. When the cap is reached, the report marks itself
/// as truncated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationPolicy {
    /// Require exactly one EOF token, positioned at the end.
    pub require_eof: bool,

    /// Require token start offsets to be nondecreasing.
    pub check_token_order: bool,

    /// Check token bounds against source text when available.
    pub check_source_bounds: bool,

    /// Check source-range endpoints against UTF-8 boundaries when source
    /// text is available.
    pub check_utf8_boundaries: bool,

    /// Check balanced parentheses, braces, and brackets.
    pub check_delimiters: bool,

    /// Report tokens classified as invalid by the lexer.
    pub report_invalid_tokens: bool,

    /// Optional maximum number of retained findings.
    ///
    /// `None` means no policy-level cap. A cap should be configured by the
    /// caller when processing untrusted input in a bounded service.
    pub max_issues: Option<usize>,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            require_eof: true,
            check_token_order: true,
            check_source_bounds: true,
            check_utf8_boundaries: true,
            check_delimiters: true,
            report_invalid_tokens: true,
            max_issues: None,
        }
    }
}

/// Result of parser-boundary validation.
///
/// `is_valid()` is false whenever at least one finding was retained.
/// `is_truncated()` indicates that additional findings may have been omitted
/// because the configured diagnostic budget was reached.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
    truncated: bool,
}

impl ValidationReport {
    /// Create an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all retained findings in discovery order.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Return the number of retained findings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.issues.len()
    }

    /// Return whether no findings were retained.
    ///
    /// An empty report with `is_truncated() == true` is not proof that the
    /// input was valid.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    /// Return whether validation retained no findings and did not truncate.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty() && !self.truncated
    }

    /// Return whether some findings may have been omitted.
    #[must_use]
    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Append a finding, respecting the caller's diagnostic budget.
    ///
    /// A zero budget retains no findings and marks the report as truncated
    /// when a finding is attempted.
    fn push(&mut self, issue: ValidationIssue, max_issues: Option<usize>) {
        if let Some(limit) = max_issues {
            if self.issues.len() >= limit {
                self.truncated = true;
                return;
            }
        }

        self.issues.push(issue);
    }
}

#[derive(Debug, Clone, Copy)]
struct OpeningDelimiter {
    kind: TokenKind,
    index: usize,
    start: usize,
    end: usize,
}

fn opening_delimiter(kind: TokenKind) -> Option<TokenKind> {
    match kind {
        TokenKind::LeftParen => Some(TokenKind::RightParen),
        TokenKind::LeftBrace => Some(TokenKind::RightBrace),
        TokenKind::LeftBracket => Some(TokenKind::RightBracket),
        _ => None,
    }
}

fn closing_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RightParen | TokenKind::RightBrace | TokenKind::RightBracket
    )
}

fn delimiter_name(kind: TokenKind) -> &'static str {
    match kind {
        TokenKind::LeftParen => "(",
        TokenKind::RightParen => ")",
        TokenKind::LeftBrace => "{",
        TokenKind::RightBrace => "}",
        TokenKind::LeftBracket => "[",
        TokenKind::RightBracket => "]",
        _ => "non-delimiter",
    }
}

/// Validate a lexer token stream before or after parsing.
///
/// The function reuses the canonical lexer `Token` and `TokenKind` types.
/// It does not tokenize source text or construct an AST.
///
/// When `source` is `Some`, span bounds and UTF-8 boundaries are checked
/// against that source. When it is `None`, only token-local ordering and
/// delimiter properties can be checked.
///
/// The validator is iterative. Its delimiter stack grows with the nesting
/// depth actually present in the input; it does not impose a fixed nesting
/// ceiling.
///
/// # EOF contract
///
/// If EOF is required, the stream must contain exactly one EOF token and it
/// must be the last token. A parser-facing token stream should not append
/// significant tokens after EOF.
pub fn validate_token_stream(
    tokens: &[Token<'_>],
    source: Option<&str>,
    policy: &ValidationPolicy,
) -> ValidationReport {
    let mut report = ValidationReport::new();
    let mut eof_index: Option<usize> = None;
    let mut previous_start: Option<usize> = None;
    let mut delimiters: Vec<OpeningDelimiter> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let span = token.span();
        let start = span.start_byte();
        let end = span.end_byte();

        if end < start {
            report.push(
                ValidationIssue::at_token(
                    ValidationIssueKind::InvalidTokenSpan,
                    index,
                    start,
                    end,
                    "token end offset precedes its start offset",
                ),
                policy.max_issues,
            );
        }

        if policy.check_token_order {
            if let Some(previous) = previous_start {
                if start < previous {
                    report.push(
                        ValidationIssue::at_token(
                            ValidationIssueKind::NonMonotonicTokenSpans,
                            index,
                            start,
                            end,
                            format!(
                                "token starts at byte {start}, before the previous token start at byte {previous}"
                            ),
                        ),
                        policy.max_issues,
                    );
                }
            }
            previous_start = Some(start);
        }

        if let Some(source_text) = source {
            if policy.check_source_bounds && end > source_text.len() {
                report.push(
                    ValidationIssue::at_token(
                        ValidationIssueKind::SpanOutOfBounds,
                        index,
                        start,
                        end,
                        format!(
                            "token range {start}..{end} exceeds source length {}",
                            source_text.len()
                        ),
                    ),
                    policy.max_issues,
                );
            } else if policy.check_utf8_boundaries
                && end <= source_text.len()
                && ( !source_text.is_char_boundary(start)
                    || !source_text.is_char_boundary(end))
            {
                report.push(
                    ValidationIssue::at_token(
                        ValidationIssueKind::InvalidUtf8Boundary,
                        index,
                        start,
                        end,
                        "token range endpoint is not a UTF-8 character boundary",
                    ),
                    policy.max_issues,
                );
            }
        }

        if policy.report_invalid_tokens && token.kind().is_invalid() {
            report.push(
                ValidationIssue::at_token(
                    ValidationIssueKind::InvalidToken,
                    index,
                    start,
                    end,
                    "lexer classified this token as invalid",
                ),
                policy.max_issues,
            );
        }

        if token.is_eof() {
            if let Some(first_eof) = eof_index {
                report.push(
                    ValidationIssue::at_token(
                        ValidationIssueKind::MultipleEof,
                        index,
                        start,
                        end,
                        format!(
                            "EOF token at index {index} follows EOF at index {first_eof}"
                        ),
                    ),
                    policy.max_issues,
                );
            } else {
                eof_index = Some(index);
            }
        } else if eof_index.is_some() {
            report.push(
                ValidationIssue::at_token(
                    ValidationIssueKind::TokensAfterEof,
                    index,
                    start,
                    end,
                    "non-EOF token appears after the first EOF token",
                ),
                policy.max_issues,
            );
        }

        if policy.check_delimiters {
            if let Some(expected_closer) = opening_delimiter(token.kind()) {
                delimiters.push(OpeningDelimiter {
                    kind: token.kind(),
                    index,
                    start,
                    end,
                });

                // The expected closer is intentionally derived from the
                // token kind. No hardware or domain-specific token list is
                // involved.
                let _ = expected_closer;
            } else if closing_delimiter(token.kind()) {
                match delimiters.pop() {
                    None => {
                        report.push(
                            ValidationIssue::at_token(
                                ValidationIssueKind::UnexpectedClosingDelimiter,
                                index,
                                start,
                                end,
                                format!(
                                    "closing delimiter '{}' has no opening delimiter",
                                    delimiter_name(token.kind())
                                ),
                            ),
                            policy.max_issues,
                        );
                    }
                    Some(opening) => {
                        let expected = opening_delimiter(opening.kind);
                        if expected != Some(token.kind()) {
                            report.push(
                                ValidationIssue::at_token(
                                    ValidationIssueKind::MismatchedDelimiter,
                                    index,
                                    start,
                                    end,
                                    format!(
                                        "expected '{}' to close delimiter '{}' opened at token index {}",
                                        expected.map(delimiter_name).unwrap_or("?"),
                                        delimiter_name(opening.kind),
                                        opening.index
                                    ),
                                )
                                .with_delimiters(
                                    expected.map(delimiter_name).unwrap_or("?"),
                                    delimiter_name(token.kind()),
                                ),
                                policy.max_issues,
                            );
                        }
                    }
                }
            }
        }
    }

    if policy.require_eof && eof_index.is_none() {
        report.push(
            ValidationIssue::stream(
                ValidationIssueKind::MissingEof,
                "the token stream does not contain an EOF token",
            ),
            policy.max_issues,
        );
    }

    if policy.require_eof {
        if let Some(index) = eof_index {
            if index + 1 != tokens.len() {
                report.push(
                    ValidationIssue::stream(
                        ValidationIssueKind::TokensAfterEof,
                        format!(
                            "EOF at token index {index} is not the final token"
                        ),
                    ),
                    policy.max_issues,
                );
            }
        }
    }

    if policy.check_delimiters {
        for opening in delimiters {
            report.push(
                ValidationIssue::at_token(
                    ValidationIssueKind::UnclosedDelimiter,
                    opening.index,
                    opening.start,
                    opening.end,
                    format!(
                        "opening delimiter '{}' has no matching closer",
                        delimiter_name(opening.kind)
                    ),
                ),
                policy.max_issues,
            );
        }
    }

    report
}

/// Validate that a parser entry point consumed the complete token stream.
///
/// `consumed_position` is the next token index after the tokens consumed by
/// the parser. For a stream containing ordinary tokens followed by EOF, a
/// parser may stop with the cursor positioned at EOF; the EOF token itself
/// need not be consumed.
///
/// This function does not require the token stream to be lexically valid.
/// Call `validate_token_stream` separately when stream validation is needed.
pub fn validate_full_consumption(
    tokens: &[Token<'_>],
    consumed_position: usize,
    max_issues: Option<usize>,
) -> ValidationReport {
    let mut report = ValidationReport::new();

    if consumed_position > tokens.len() {
        report.push(
            ValidationIssue::stream(
                ValidationIssueKind::InvalidConsumptionPosition,
                format!(
                    "parser cursor position {consumed_position} exceeds token count {}",
                    tokens.len()
                ),
            ),
            max_issues,
        );
        return report;
    }

    if let Some((index, token)) = tokens
        .iter()
        .enumerate()
        .skip(consumed_position)
        .find(|(_, token)| !token.is_eof() && !token.kind().is_trivia())
    {
        let span = token.span();

        report.push(
            ValidationIssue::at_token(
                ValidationIssueKind::TrailingInput,
                index,
                span.start_byte(),
                span.end_byte(),
                format!(
                    "unconsumed significant token '{}' remains at index {index}",
                    token.kind()
                ),
            ),
            max_issues,
        );
    }

    report
}

/// Validate an AST fragment through the canonical AST validation facade.
///
/// This function deliberately delegates traversal, identity checks, cycle
/// detection, child-reference validation, and configured structural policy
/// to the existing AST subsystem. It does not implement a second AST walker.
///
/// The provider must implement the repository's `AstWalkProvider` contract.
pub fn validate_ast_fragment<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
) -> Result<
    crate::frontend::ast::node::validation::StructuralValidationStatistics,
    crate::frontend::ast::node::validation::StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: fmt::Display,
{
    crate::frontend::ast::node::validation::validate_fragment(provider, root)
}

/// Validate an AST root that is required to represent a complete program.
///
/// This is separate from fragment validation because parser recovery,
/// incremental compilation, generated nodes, and syntax transformations can
/// legitimately produce AST fragments that are not program roots.
pub fn validate_program_ast<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
) -> Result<
    crate::frontend::ast::node::validation::StructuralValidationStatistics,
    crate::frontend::ast::node::validation::StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: fmt::Display,
{
    crate::frontend::ast::node::validation::validate_program(provider, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::lexer::cursor::CursorPosition;
    use crate::frontend::lexer::span::Span;

    fn token(kind: TokenKind, lexeme: &str, start: usize, end: usize) -> Token<'_> {
        let start_position = CursorPosition::new(start, 1, start.saturating_add(1));
        let end_position = CursorPosition::new(end, 1, end.saturating_add(1));
        let span = Span::try_new(start_position, end_position)
            .expect("test token span should be valid");

        Token::borrowed(kind, lexeme, span)
    }

    #[test]
    fn empty_stream_requires_eof_by_default() {
        let report = validate_token_stream(&[], Some(""), &ValidationPolicy::default());

        assert!(!report.is_valid());
        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::MissingEof));
    }

    #[test]
    fn eof_at_end_is_valid_for_stream_integrity() {
        let tokens = [token(TokenKind::Eof, "", 0, 0)];
        let report =
            validate_token_stream(&tokens, Some(""), &ValidationPolicy::default());

        assert!(report.is_valid(), "{:?}", report.issues());
    }

    #[test]
    fn multiple_eof_tokens_are_reported() {
        let tokens = [
            token(TokenKind::Eof, "", 0, 0),
            token(TokenKind::Eof, "", 0, 0),
        ];

        let report =
            validate_token_stream(&tokens, Some(""), &ValidationPolicy::default());

        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::MultipleEof));
    }

    #[test]
    fn mismatched_delimiters_are_reported() {
        let tokens = [
            token(TokenKind::LeftParen, "(", 0, 1),
            token(TokenKind::RightBrace, "}", 1, 2),
            token(TokenKind::Eof, "", 2, 2),
        ];

        let report = validate_token_stream(
            &tokens,
            Some("()}"),
            &ValidationPolicy::default(),
        );

        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::MismatchedDelimiter));
    }

    #[test]
    fn unclosed_delimiters_are_reported() {
        let tokens = [
            token(TokenKind::LeftBracket, "[", 0, 1),
            token(TokenKind::Eof, "", 1, 1),
        ];

        let report = validate_token_stream(
            &tokens,
            Some("["),
            &ValidationPolicy::default(),
        );

        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::UnclosedDelimiter));
    }

    #[test]
    fn unexpected_closing_delimiter_is_reported() {
        let tokens = [
            token(TokenKind::RightParen, ")", 0, 1),
            token(TokenKind::Eof, "", 1, 1),
        ];

        let report = validate_token_stream(
            &tokens,
            Some(")"),
            &ValidationPolicy::default(),
        );

        assert!(report.issues().iter().any(|issue| {
            issue.kind == ValidationIssueKind::UnexpectedClosingDelimiter
        }));
    }

    #[test]
    fn source_bounds_are_checked_when_source_is_available() {
        let tokens = [
            token(TokenKind::Identifier, "name", 0, 4),
            token(TokenKind::Eof, "", 4, 4),
        ];

        let report = validate_token_stream(
            &tokens,
            Some("abc"),
            &ValidationPolicy::default(),
        );

        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::SpanOutOfBounds));
    }

    #[test]
    fn diagnostic_budget_marks_report_as_truncated() {
        let tokens = [
            token(TokenKind::RightBrace, "}", 0, 1),
            token(TokenKind::RightBracket, "]", 1, 2),
            token(TokenKind::Eof, "", 2, 2),
        ];

        let policy = ValidationPolicy {
            max_issues: Some(1),
            ..ValidationPolicy::default()
        };

        let report = validate_token_stream(&tokens, Some("}]"), &policy);

        assert_eq!(report.len(), 1);
        assert!(report.is_truncated());
        assert!(!report.is_valid());
    }

    #[test]
    fn complete_consumption_allows_cursor_at_eof() {
        let tokens = [
            token(TokenKind::Identifier, "name", 0, 4),
            token(TokenKind::Eof, "", 4, 4),
        ];

        let report = validate_full_consumption(&tokens, 1, None);

        assert!(report.is_valid());
    }

    #[test]
    fn incomplete_consumption_reports_trailing_input() {
        let tokens = [
            token(TokenKind::Identifier, "first", 0, 5),
            token(TokenKind::Identifier, "second", 6, 12),
            token(TokenKind::Eof, "", 12, 12),
        ];

        let report = validate_full_consumption(&tokens, 1, None);

        assert!(report
            .issues()
            .iter()
            .any(|issue| issue.kind == ValidationIssueKind::TrailingInput));
    }

    #[test]
    fn consumption_position_past_end_is_reported() {
        let tokens = [token(TokenKind::Eof, "", 0, 0)];

        let report = validate_full_consumption(&tokens, 2, None);

        assert_eq!(
            report.issues()[0].kind,
            ValidationIssueKind::InvalidConsumptionPosition
        );
    }
}
