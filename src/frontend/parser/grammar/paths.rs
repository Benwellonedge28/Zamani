
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Domain-neutral logical-path grammar for the Zamani frontend.
//!
//! Normative grammar: `grammar/core/paths.g4`.
//! Identifier authority: `grammar/core/names.g4`.
//! Lexical authority: `src/frontend/lexer/`.
//!
//! # Ownership
//!
//! This module owns the parser-level representation and productions for:
//! - logical paths;
//! - absolute and relative paths;
//! - current-scope and parent-scope components;
//! - path suffixes;
//! - path lists;
//! - wildcard path patterns.
//!
//! It does not own qualified-name syntax (`a::b`), identifier
//! tokenization, path normalization, filesystem or URL interpretation,
//! module resolution, symbol resolution, authorization, resource discovery,
//! or hardware placement.
//!
//! # Integration
//!
//! - `TokenCursor` validates and traverses the canonical lexer token stream.
//! - `ParserBudget` accounts for token consumption.
//! - `ParseError` represents syntax and resource-budget failures.
//! - `grammar/core/paths.g4` defines the normative productions.
//! - `grammar/core/names.g4` owns identifiers and qualified names.
//! - The AST adapter or a future canonical AST path node consumes the
//!   lossless path representation provided here.
//!
//! # Resource and safety policy
//!
//! This module has no fixed limit on path depth or path-list length.
//! Every consumed token is charged to the caller's parser budget.
//! Vector growth uses fallible reservation and reports allocation-capacity
//! failures through the parser error contract.
//!
//! Rust and the host process still have finite address space and resources;
//! "unbounded" means no arbitrary language-level or hardware-capacity ceiling.
//!
//! No path is normalized or resolved during parsing.

use core::fmt;

use super::super::error::{ParseError, ParseErrorClass, ParseErrorKind};
use super::super::limits::{ParserBudget, ParserLimitKind};
use super::super::token_cursor::TokenCursor;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;

/// A syntactic component of a logical path.
///
/// `Named` preserves the exact identifier spelling. Whether that spelling
/// resolves to a module, resource, capability, declaration, or other entity
/// is a later semantic concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathComponent {
    /// A normal identifier path component.
    Named {
        /// Original identifier spelling, preserved exactly.
        name: String,
        /// Source span of the identifier token.
        span: Span,
    },

    /// Current-scope component: `.`.
    Current {
        /// Source span of the dot token.
        span: Span,
    },

    /// Parent-scope component: `..`.
    Parent {
        /// Source span of the range token used as `..`.
        span: Span,
    },
}

impl PathComponent {
    /// Return the source span of this component.
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Named { span, .. }
            | Self::Current { span }
            | Self::Parent { span } => *span,
        }
    }
}

/// A parsed logical path.
///
/// The path is structural syntax only. `absolute` indicates whether the
/// path starts with `/`; it does not imply an operating-system filesystem
/// root. Components remain in source order and are not normalized.
///
/// Examples:
///
/// - `alpha` → relative path with one named component;
/// - `alpha/beta` → relative path with two named components;
/// - `/alpha/beta` → absolute logical path;
/// - `./alpha` → current-scope component followed by a named component;
/// - `../alpha` → parent-scope component followed by a named component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalPath {
    /// Whether the path begins with the absolute-path marker `/`.
    pub absolute: bool,

    /// Ordered path components.
    pub components: Vec<PathComponent>,

    /// Span covering the path, excluding surrounding trivia.
    pub span: Span,
}

impl LogicalPath {
    /// Return `true` when the path contains no components.
    ///
    /// Valid `path` productions are non-empty. This helper is useful for
    /// validating values constructed by integrations or deserialization.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

/// A component of a path pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathPatternComponent {
    /// A normal path component.
    Path(PathComponent),

    /// Wildcard component: `*`.
    ///
    /// This is a pattern marker only. It does not enumerate or discover
    /// anything during parsing.
    Wildcard {
        /// Source span of the wildcard token.
        span: Span,
    },
}

impl PathPatternComponent {
    /// Return the source span of this pattern component.
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Path(component) => component.span(),
            Self::Wildcard { span } => *span,
        }
    }
}

/// A parsed logical path pattern.
///
/// Pattern matching and expansion belong to later tooling or semantic
/// layers. A wildcard does not imply that any particular resource exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalPathPattern {
    /// Whether the pattern begins with `/`.
    pub absolute: bool,

    /// Ordered pattern components.
    pub components: Vec<PathPatternComponent>,

    /// Span covering the pattern.
    pub span: Span,
}

/// A parsed non-empty list of logical paths.
///
/// The parser preserves each path independently and does not assign meaning
/// to the list's eventual consumer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalPathList {
    /// Paths in source order.
    pub paths: Vec<LogicalPath>,

    /// Span covering the complete list.
    pub span: Span,
}

/// Return whether the current token can begin a logical path.
///
/// This is a recognition predicate only: it never consumes tokens.
///
/// `Range` is the canonical lexer representation of `..`; `Dot` represents
/// `.`. Slash-prefixed absolute paths use the ordinary `Slash` token.
#[must_use]
pub fn can_start_path(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Identifier
            | TokenKind::Slash
            | TokenKind::Dot
            | TokenKind::Range
    )
}

/// Return whether the current token can begin a path pattern.
#[must_use]
pub fn can_start_path_pattern(kind: TokenKind) -> bool {
    can_start_path(kind) || kind == TokenKind::Star
}

/// Parse one logical path according to `grammar/core/paths.g4`.
///
/// Supported forms include:
///
/// - `module`
/// - `module/child`
/// - `/system/core`
/// - `./module`
/// - `../module`
/// - `module/../other`
///
/// The function leaves the cursor at the first token after the path. It does
/// not require EOF because paths are embedded in larger grammar productions.
///
/// Each consumed token is charged to `budget`. The caller remains responsible
/// for applying any broader parse-session policy.
pub fn parse_path(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<LogicalPath, ParseError> {
    let first_span = cursor.span();
    let absolute = cursor.at(TokenKind::Slash);

    if absolute {
        consume_expected(cursor, budget, TokenKind::Slash, "/")?;
    }

    let mut components = Vec::new();

    if absolute {
        // The canonical absolutePath production requires one normal segment
        // immediately after its leading slash.
        let component = parse_named_component(cursor, budget)?;
        push_fallible(&mut components, component, cursor.span())?;
    } else {
        // Relative paths may begin with "." or "..", or with an identifier.
        let first = parse_relative_component(cursor, budget)?;
        push_fallible(&mut components, first, cursor.span())?;
    }

    while cursor.at(TokenKind::Slash) {
        consume_expected(cursor, budget, TokenKind::Slash, "/")?;

        // A slash must be followed by a component. In particular, `a/`,
        // `/a/`, and `a//b` are invalid paths.
        let component = parse_relative_component(cursor, budget)?;
        push_fallible(&mut components, component, cursor.span())?;
    }

    let end_span = components
        .last()
        .map(PathComponent::span)
        .unwrap_or(first_span);

    Ok(LogicalPath {
        absolute,
        components,
        span: combine_spans(first_span, end_span),
    })
}

/// Parse a non-empty comma-separated list of logical paths.
///
/// The caller owns any trailing-comma policy. This production follows the
/// canonical `pathList : path (COMMA path)*` rule and therefore does not
/// accept a trailing comma.
pub fn parse_path_list(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<LogicalPathList, ParseError> {
    let first_span = cursor.span();
    let first = parse_path(cursor, budget)?;
    let mut paths = Vec::new();
    push_fallible(&mut paths, first, cursor.span())?;

    while cursor.at(TokenKind::Comma) {
        consume_expected(cursor, budget, TokenKind::Comma, ",")?;

        if cursor.is_eof() {
            return Err(expected_error(
                cursor,
                "logical path after ','",
            ));
        }

        let path = parse_path(cursor, budget)?;
        push_fallible(&mut paths, path, cursor.span())?;
    }

    let end_span = paths
        .last()
        .map(|path| path.span)
        .unwrap_or(first_span);

    Ok(LogicalPathList {
        paths,
        span: combine_spans(first_span, end_span),
    })
}

/// Parse a path suffix.
///
/// A suffix begins with `/` and requires one or more named path components:
///
/// - `/child`
/// - `/child/grandchild`
///
/// Unlike a complete logical path, this production does not accept `.` or
/// `..` as suffix components because the normative `pathSuffix` rule uses
/// `pathSegment`, which is an identifier.
pub fn parse_path_suffix(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<LogicalPath, ParseError> {
    let first_span = cursor.span();

    consume_expected(cursor, budget, TokenKind::Slash, "/")?;

    let first = parse_named_component(cursor, budget)?;
    let mut components = Vec::new();
    push_fallible(&mut components, first, cursor.span())?;

    while cursor.at(TokenKind::Slash) {
        consume_expected(cursor, budget, TokenKind::Slash, "/")?;

        let component = parse_named_component(cursor, budget)?;
        push_fallible(&mut components, component, cursor.span())?;
    }

    let end_span = components
        .last()
        .map(PathComponent::span)
        .unwrap_or(first_span);

    Ok(LogicalPath {
        // A suffix is not independently an absolute path. Its leading slash
        // is relative to the path to which it will later be attached.
        absolute: false,
        components,
        span: combine_spans(first_span, end_span),
    })
}

/// Parse a logical path pattern.
///
/// Supported examples:
///
/// - `*`
/// - `module/*`
/// - `/system/*/capability`
/// - `../module/*`
///
/// The pattern grammar deliberately treats `*` as syntax. Pattern matching,
/// expansion, permissions and resource discovery happen downstream.
pub fn parse_path_pattern(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<LogicalPathPattern, ParseError> {
    let first_span = cursor.span();
    let absolute = cursor.at(TokenKind::Slash);

    if absolute {
        consume_expected(cursor, budget, TokenKind::Slash, "/")?;
    }

    let mut components = Vec::new();
    let first = parse_pattern_component(cursor, budget)?;
    push_fallible(&mut components, first, cursor.span())?;

    while cursor.at(TokenKind::Slash) {
        consume_expected(cursor, budget, TokenKind::Slash, "/")?;

        let component = parse_pattern_component(cursor, budget)?;
        push_fallible(&mut components, component, cursor.span())?;
    }

    let end_span = components
        .last()
        .map(PathPatternComponent::span)
        .unwrap_or(first_span);

    Ok(LogicalPathPattern {
        absolute,
        components,
        span: combine_spans(first_span, end_span),
    })
}

fn parse_pattern_component(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<PathPatternComponent, ParseError> {
    if cursor.at(TokenKind::Star) {
        let span = cursor.span();
        consume_expected(cursor, budget, TokenKind::Star, "*")?;
        return Ok(PathPatternComponent::Wildcard { span });
    }

    parse_relative_component(cursor, budget).map(PathPatternComponent::Path)
}

fn parse_relative_component(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<PathComponent, ParseError> {
    match cursor.peek().kind() {
        TokenKind::Identifier => parse_named_component(cursor, budget),

        TokenKind::Dot => {
            let span = cursor.span();
            consume_expected(cursor, budget, TokenKind::Dot, ".")?;
            Ok(PathComponent::Current { span })
        }

        TokenKind::Range => {
            let span = cursor.span();
            consume_expected(cursor, budget, TokenKind::Range, "..")?;
            Ok(PathComponent::Parent { span })
        }

        _ => Err(expected_error(
            cursor,
            "identifier, '.', or '..' path component",
        )),
    }
}

fn parse_named_component(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
) -> Result<PathComponent, ParseError> {
    if !cursor.at(TokenKind::Identifier) {
        return Err(expected_error(cursor, "identifier path component"));
    }

    let span = cursor.span();
    let name = cursor.peek().lexeme().to_owned();

    consume_expected(
        cursor,
        budget,
        TokenKind::Identifier,
        "identifier",
    )?;

    Ok(PathComponent::Named { name, span })
}

fn consume_expected(
    cursor: &mut TokenCursor<'_, '_>,
    budget: &mut ParserBudget,
    expected: TokenKind,
    expected_label: &'static str,
) -> Result<(), ParseError> {
    if cursor.at(expected) {
        cursor.advance_with_budget(budget)?;
        return Ok(());
    }

    Err(expected_error(cursor, expected_label))
}

fn expected_error(
    cursor: &TokenCursor<'_, '_>,
    expected: &'static str,
) -> ParseError {
    let token = cursor.peek();
    let span = cursor.span();

    if cursor.is_eof() {
        ParseError::unexpected_eof(span, [expected])
    } else {
        ParseError::unexpected_token(
            span,
            token.lexeme(),
            [expected],
        )
    }
}

fn push_fallible<T>(
    values: &mut Vec<T>,
    value: T,
    span: Span,
) -> Result<(), ParseError> {
    values.try_reserve(1).map_err(|error| {
        ParseError::new(
            ParseErrorKind::InternalInvariant {
                message: format!(
                    "unable to reserve storage for logical-path syntax: {error}"
                ),
            },
            ParseErrorClass::Fatal,
        )
        .with_span(span)
    })?;

    values.push(value);
    Ok(())
}

/// Combine two source spans when their offsets are ordered.
///
/// Lexer spans use the same source-coordinate contract, but their constructors
/// are intentionally not recreated here. If spans come from different source
/// identities or are not ordered, preserve the first span rather than
/// manufacturing an invalid range.
fn combine_spans(first: Span, last: Span) -> Span {
    if first.source() == last.source() && first.start() <= last.end() {
        match Span::new(first.source(), first.start(), last.end()) {
            Ok(span) => span,
            Err(_) => first,
        }
    } else {
        first
    }
}

// The grammar module is deliberately independent of AST builders. A later
// AST-adapter integration should convert LogicalPath and LogicalPathPattern
// into the canonical path representation when one is introduced. Until then,
// these lossless parser-level values preserve all path structure and spans.
