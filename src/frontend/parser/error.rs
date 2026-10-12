
#![forbid(unsafe_code)]

//! Structured parser errors for the Zamani frontend.
//!
//! File: `src/frontend/parser/error.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define the canonical, domain-neutral representation of parser failures.
//! Errors preserve machine-readable classifications and source locations
//! without coupling grammar productions to presentation, recovery policy,
//! semantic analysis, or a particular computing domain.
//!
//! # Owns
//!
//! - Parser error kinds and their stable textual identifiers.
//! - Recoverable versus fatal error classification.
//! - Optional canonical source spans.
//! - Unexpected-token and unexpected-EOF information.
//! - Parser resource-budget failures.
//! - Extension-handler failures.
//! - Optional grammar-context information.
//! - Human-readable formatting and standard error integration.
//!
//! # Does not own
//!
//! - Lexical scanning or token construction.
//! - Source-coordinate calculation.
//! - Diagnostic collection, sorting, or rendering.
//! - Parser cursor movement or synchronization.
//! - AST construction or semantic validation.
//! - Type, effect, capability, contract, or policy evaluation.
//! - Hardware discovery, resource negotiation, or execution.
//!
//! # Integration contract
//!
//! - `super::super::lexer::span::Span` is the canonical lexer source span.
//! - `super::limits::ParserLimitViolation` is the canonical parser-budget
//!   failure representation.
//! - `token_cursor.rs` supplies found-token information to constructors.
//! - `diagnostics.rs` converts parser errors into user-facing diagnostics.
//! - `recovery.rs` uses the classification to decide whether recovery is
//!   possible; this module itself never advances the cursor.
//! - `result.rs` and `session.rs` retain parser errors as appropriate.
//! - `api.rs` exposes errors through the approved public parser API.
//!
//! Error values do not own tokens, source files, AST nodes, or parser
//! sessions. Callers may attach an optional source span and contextual
//! descriptions when that information is available.
//!
//! # Scalability and resource safety
//!
//! This module imposes no language-level ceiling on source size, declarations,
//! qubits, devices, register sizes, tensor rank, or computational scale.
//! Finite resource budgets are represented by `ParserLimitViolation`.
//! Collections in errors can grow only as permitted by the caller's resource
//! policy; applications parsing untrusted input should configure parser
//! budgets and bound diagnostic retention.
//!
//! # Compatibility
//!
//! Error kinds are non-exhaustive. Downstream code should use the provided
//! accessors and include a fallback branch when matching error kinds.
//! `stable_code()` provides a stable identifier for tooling. Human-readable
//! messages are not a serialization format and may evolve.
//!
//! This module does not silently reinterpret lexical errors as parser errors.
//! The integration layer should preserve the original lexical failure when
//! it can do so, or wrap it explicitly as `LexicalFailure`.

use core::fmt;
use std::error::Error;

use super::limits::ParserLimitViolation;
use super::super::lexer::span::Span;

/// Machine-readable category of a parser error.
///
/// Variants describe syntax-processing failures rather than semantic
/// failures or limitations of a particular execution target.
///
/// This enum is non-exhaustive so new parser capabilities can be added
/// without requiring every downstream consumer to be updated immediately.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// The next token does not satisfy the active grammar production.
    UnexpectedToken {
        /// Human-readable spelling or token label, if available.
        found: Option<String>,
        /// Tokens or grammar symbols accepted at this position.
        expected: Vec<String>,
    },

    /// The input ended before a required grammar construct was complete.
    UnexpectedEof {
        /// Tokens or grammar symbols accepted at the end of input.
        expected: Vec<String>,
    },

    /// A lexical error prevented the parser from interpreting a token.
    ///
    /// Prefer retaining the original lexer diagnostic separately when the
    /// frontend API supports it. This variant is for boundaries that must
    /// communicate the failure through the parser error contract.
    LexicalFailure {
        /// Explanation supplied by the lexer integration layer.
        message: String,
    },

    /// A token stream violated a parser input invariant.
    ///
    /// Examples include a missing EOF token when the API requires one,
    /// tokens with inconsistent ordering, or an impossible cursor state.
    InvalidTokenStream {
        /// Description of the violated input invariant.
        message: String,
    },

    /// A parser resource budget rejected an operation.
    ResourceLimit(ParserLimitViolation),

    /// A registered grammar extension rejected or could not parse input.
    ExtensionFailure {
        /// Stable extension identifier where available.
        extension_id: String,
        /// Extension-provided explanation.
        message: String,
    },

    /// Input remained after a production that requires complete consumption.
    TrailingInput {
        /// Label or spelling of the first unconsumed token, if available.
        found: Option<String>,
    },

    /// A required language feature is disabled by the active configuration.
    FeatureDisabled {
        /// Stable feature identifier.
        feature_id: String,
    },

    /// A configuration or grammar registration conflict prevented parsing.
    ConfigurationFailure {
        /// Explanation of the configuration problem.
        message: String,
    },

    /// An internal parser invariant was violated.
    ///
    /// This represents an implementation defect or an invalid internal
    /// integration state, not an ordinary syntax error.
    InternalInvariant {
        /// Explanation suitable for logs and debugging.
        message: String,
    },
}

impl ParseErrorKind {
    /// Return the stable machine-readable error code.
    ///
    /// Codes are intended for diagnostics, tests, and tooling. They are not
    /// numeric IDs and do not encode severity.
    #[must_use]
    pub const fn stable_code(&self) -> &'static str {
        match self {
            Self::UnexpectedToken { .. } => "parser.unexpected_token",
            Self::UnexpectedEof { .. } => "parser.unexpected_eof",
            Self::LexicalFailure { .. } => "parser.lexical_failure",
            Self::InvalidTokenStream { .. } => "parser.invalid_token_stream",
            Self::ResourceLimit(_) => "parser.resource_limit",
            Self::ExtensionFailure { .. } => "parser.extension_failure",
            Self::TrailingInput { .. } => "parser.trailing_input",
            Self::FeatureDisabled { .. } => "parser.feature_disabled",
            Self::ConfigurationFailure { .. } => "parser.configuration_failure",
            Self::InternalInvariant { .. } => "parser.internal_invariant",
        }
    }

    /// Whether the failure is an implementation or integration failure.
    ///
    /// This is useful for distinguishing user-correctable syntax errors
    /// from failures that normally warrant developer investigation.
    #[must_use]
    pub const fn is_internal_failure(&self) -> bool {
        matches!(
            self,
            Self::InvalidTokenStream { .. }
                | Self::ConfigurationFailure { .. }
                | Self::InternalInvariant { .. }
        )
    }

    /// Whether the parser may reasonably attempt local syntax recovery.
    ///
    /// This is guidance, not a recovery algorithm. The recovery subsystem
    /// must still check cursor progress, synchronization points, and budgets.
    #[must_use]
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::UnexpectedToken { .. }
                | Self::UnexpectedEof { .. }
                | Self::TrailingInput { .. }
                | Self::FeatureDisabled { .. }
                | Self::ExtensionFailure { .. }
        )
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken { found, expected } => {
                formatter.write_str("unexpected token")?;

                if let Some(found) = found {
                    write!(formatter, " '{found}'")?;
                }

                write_expected(formatter, expected)
            }

            Self::UnexpectedEof { expected } => {
                formatter.write_str("unexpected end of input")?;
                write_expected(formatter, expected)
            }

            Self::LexicalFailure { message } => {
                write!(formatter, "lexical failure: {message}")
            }

            Self::InvalidTokenStream { message } => {
                write!(formatter, "invalid token stream: {message}")
            }

            Self::ResourceLimit(violation) => {
                write!(formatter, "{violation}")
            }

            Self::ExtensionFailure {
                extension_id,
                message,
            } => {
                write!(
                    formatter,
                    "grammar extension '{extension_id}' failed: {message}"
                )
            }

            Self::TrailingInput { found } => {
                formatter.write_str("unexpected trailing input")?;

                if let Some(found) = found {
                    write!(formatter, ": '{found}'")?;
                }

                Ok(())
            }

            Self::FeatureDisabled { feature_id } => {
                write!(
                    formatter,
                    "language feature '{feature_id}' is disabled"
                )
            }

            Self::ConfigurationFailure { message } => {
                write!(formatter, "parser configuration failure: {message}")
            }

            Self::InternalInvariant { message } => {
                write!(formatter, "parser invariant violated: {message}")
            }
        }
    }
}

/// Append a deterministic expected-symbol description.
fn write_expected(
    formatter: &mut fmt::Formatter<'_>,
    expected: &[String],
) -> fmt::Result {
    match expected {
        [] => Ok(()),
        [only] => write!(formatter, "; expected {only}"),
        many => {
            formatter.write_str("; expected one of ")?;

            for (index, symbol) in many.iter().enumerate() {
                if index > 0 {
                    formatter.write_str(", ")?;
                }

                formatter.write_str(symbol)?;
            }

            Ok(())
        }
    }
}

/// Classification of a parser error for API and recovery consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseErrorClass {
    /// A syntax problem that may be reported and recovered from.
    Recoverable,

    /// Parsing cannot safely continue with the current parser state.
    Fatal,

    /// An implementation or integration failure requiring investigation.
    Internal,
}

impl fmt::Display for ParseErrorClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recoverable => formatter.write_str("recoverable"),
            Self::Fatal => formatter.write_str("fatal"),
            Self::Internal => formatter.write_str("internal"),
        }
    }
}

/// A structured parser error.
///
/// `span` is optional because some failures occur before a usable source
/// position is available, such as malformed parser configuration. When a
/// token or source position is available, callers should attach its canonical
/// lexer span.
///
/// `contexts` records the grammar contexts that contributed to the error.
/// Context strings are descriptive only; consumers should use `kind()` and
/// `stable_code()` for programmatic decisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Span>,
    class: ParseErrorClass,
    contexts: Vec<String>,
}

impl ParseError {
    /// Construct a parser error with no source span or grammar context.
    ///
    /// Use a specialized constructor where possible so classification remains
    /// consistent across grammar modules.
    #[must_use]
    pub fn new(kind: ParseErrorKind, class: ParseErrorClass) -> Self {
        Self {
            kind,
            span: None,
            class,
            contexts: Vec::new(),
        }
    }

    /// Construct an unexpected-token error.
    ///
    /// Expected symbols are sorted and deduplicated to make diagnostics
    /// deterministic regardless of grammar registration order.
    #[must_use]
    pub fn unexpected_token(
        span: Span,
        found: impl Into<String>,
        expected: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedToken {
                found: Some(found.into()),
                expected: normalize_expected(expected),
            },
            ParseErrorClass::Recoverable,
        )
        .with_span(span)
    }

    /// Construct an unexpected-token error when its spelling is unavailable.
    #[must_use]
    pub fn unexpected_token_kind(
        span: Span,
        expected: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedToken {
                found: None,
                expected: normalize_expected(expected),
            },
            ParseErrorClass::Recoverable,
        )
        .with_span(span)
    }

    /// Construct an unexpected-end-of-input error.
    #[must_use]
    pub fn unexpected_eof(
        span: Span,
        expected: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedEof {
                expected: normalize_expected(expected),
            },
            ParseErrorClass::Recoverable,
        )
        .with_span(span)
    }

    /// Construct a trailing-input error.
    #[must_use]
    pub fn trailing_input(span: Span, found: Option<String>) -> Self {
        Self::new(
            ParseErrorKind::TrailingInput { found },
            ParseErrorClass::Recoverable,
        )
        .with_span(span)
    }

    /// Construct a parser error from a resource-budget violation.
    #[must_use]
    pub fn resource_limit(
        violation: ParserLimitViolation,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind: ParseErrorKind::ResourceLimit(violation),
            span,
            class: ParseErrorClass::Fatal,
            contexts: Vec::new(),
        }
    }

    /// Construct an error reported by a registered grammar extension.
    #[must_use]
    pub fn extension_failure(
        extension_id: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind: ParseErrorKind::ExtensionFailure {
                extension_id: extension_id.into(),
                message: message.into(),
            },
            span,
            class: ParseErrorClass::Recoverable,
            contexts: Vec::new(),
        }
    }

    /// Construct an internal parser invariant error.
    #[must_use]
    pub fn internal_invariant(
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind: ParseErrorKind::InternalInvariant {
                message: message.into(),
            },
            span,
            class: ParseErrorClass::Internal,
            contexts: Vec::new(),
        }
    }

    /// Attach a source span.
    #[must_use]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Add a grammar-context description.
    ///
    /// Contexts are retained in insertion order, from the innermost context
    /// to the outermost context if callers attach them while unwinding.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.contexts.push(context.into());
        self
    }

    /// Replace the error classification.
    ///
    /// Intended for the parser orchestration layer when it has stronger
    /// knowledge about whether the current parser state remains valid.
    #[must_use]
    pub fn with_class(mut self, class: ParseErrorClass) -> Self {
        self.class = class;
        self
    }

    /// Return the structured error kind.
    #[must_use]
    pub const fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    /// Return the stable machine-readable error code.
    #[must_use]
    pub const fn stable_code(&self) -> &'static str {
        self.kind.stable_code()
    }

    /// Return the error classification.
    #[must_use]
    pub const fn class(&self) -> ParseErrorClass {
        self.class
    }

    /// Return the source span, if one is available.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    /// Return grammar-context descriptions.
    #[must_use]
    pub fn contexts(&self) -> &[String] {
        &self.contexts
    }

    /// Return whether this error is classified as recoverable.
    #[must_use]
    pub const fn is_recoverable(&self) -> bool {
        matches!(self.class, ParseErrorClass::Recoverable)
    }

    /// Return whether this error is fatal.
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        matches!(self.class, ParseErrorClass::Fatal)
    }

    /// Return whether this error indicates an internal failure.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(self.class, ParseErrorClass::Internal)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.stable_code(), self.kind)?;

        if let Some(span) = self.span {
            write!(formatter, " at {span}")?;
        }

        if !self.contexts.is_empty() {
            formatter.write_str(" (while parsing ")?;

            for (index, context) in self.contexts.iter().rev().enumerate() {
                if index > 0 {
                    formatter.write_str(" > ")?;
                }

                formatter.write_str(context)?;
            }

            formatter.write_str(")")?;
        }

        Ok(())
    }
}

impl Error for ParseError {}

/// Normalize expected grammar symbols for deterministic error output.
fn normalize_expected(
    expected: impl IntoIterator<Item = impl Into<String>>,
) -> Vec<String> {
    let mut normalized: Vec<String> = expected.into_iter().map(Into::into).collect();
    normalized.sort_unstable();
    normalized.dedup();
    normalized
}

/// Convert a parser error into a concise message without discarding its
/// structured kind, span, or classification.
///
/// The diagnostic subsystem should use the structured accessors directly
/// rather than treating this string as a machine-readable representation.
impl From<ParseError> for String {
    fn from(error: ParseError) -> Self {
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn unexpected_token_has_stable_code_and_sorted_expectations() {
        let error = ParseError::unexpected_token(
            test_span(),
            "}",
            ["expression", "identifier", "expression"],
        );

        assert_eq!(error.stable_code(), "parser.unexpected_token");
        assert_eq!(error.class(), ParseErrorClass::Recoverable);

        match error.kind() {
            ParseErrorKind::UnexpectedToken { found, expected } => {
                assert_eq!(found.as_deref(), Some("}"));
                assert_eq!(
                    expected,
                    &vec!["expression".to_owned(), "identifier".to_owned()]
                );
            }
            other => panic!("unexpected error kind: {other:?}"),
        }
    }

    #[test]
    fn unexpected_eof_preserves_expected_symbols() {
        let error = ParseError::unexpected_eof(test_span(), ["}", ")"]);

        assert_eq!(error.stable_code(), "parser.unexpected_eof");
        assert!(error.is_recoverable());

        match error.kind() {
            ParseErrorKind::UnexpectedEof { expected } => {
                assert_eq!(expected, &vec![")".to_owned(), "}".to_owned()]);
            }
            other => panic!("unexpected error kind: {other:?}"),
        }
    }

    #[test]
    fn source_span_and_context_are_preserved() {
        let error = ParseError::unexpected_token(test_span(), ")", ["expression"])
            .with_context("function body")
            .with_context("expression");

        assert_eq!(error.span(), Some(test_span()));
        assert_eq!(
            error.contexts(),
            &["function body".to_owned(), "expression".to_owned()]
        );

        let rendered = error.to_string();
        assert!(rendered.contains("parser.unexpected_token"));
        assert!(rendered.contains("while parsing expression > function body"));
    }

    #[test]
    fn internal_failure_is_not_misclassified_as_syntax_recovery() {
        let error =
            ParseError::internal_invariant("cursor moved outside token stream", None);

        assert!(error.is_internal());
        assert!(!error.is_recoverable());
        assert_eq!(error.stable_code(), "parser.internal_invariant");
    }

    #[test]
    fn trailing_input_has_a_distinct_error_code() {
        let error =
            ParseError::trailing_input(test_span(), Some("extra".to_owned()));

        assert_eq!(error.stable_code(), "parser.trailing_input");
        assert!(error.is_recoverable());
        assert!(error.to_string().contains("extra"));
    }

    #[test]
    fn error_kind_codes_are_independent_of_display_text() {
        let cases = [
            (
                ParseErrorKind::LexicalFailure {
                    message: "bad escape".to_owned(),
                },
                "parser.lexical_failure",
            ),
            (
                ParseErrorKind::InvalidTokenStream {
                    message: "missing EOF".to_owned(),
                },
                "parser.invalid_token_stream",
            ),
            (
                ParseErrorKind::ExtensionFailure {
                    extension_id: "example.syntax".to_owned(),
                    message: "unsupported construct".to_owned(),
                },
                "parser.extension_failure",
            ),
            (
                ParseErrorKind::FeatureDisabled {
                    feature_id: "example.feature".to_owned(),
                },
                "parser.feature_disabled",
            ),
            (
                ParseErrorKind::ConfigurationFailure {
                    message: "conflicting registration".to_owned(),
                },
                "parser.configuration_failure",
            ),
        ];

        for (kind, expected_code) in cases {
            assert_eq!(kind.stable_code(), expected_code);
        }
    }

    #[test]
    fn error_context_is_optional() {
        let error = ParseError::new(
            ParseErrorKind::ConfigurationFailure {
                message: "invalid parser profile".to_owned(),
            },
            ParseErrorClass::Internal,
        );

        assert!(error.span().is_none());
        assert!(error.contexts().is_empty());
        assert!(error.is_internal());
    }
}
