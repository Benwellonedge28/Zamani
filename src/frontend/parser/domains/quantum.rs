#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Quantum-domain syntax adapter for Zamani.
//!
//! Path: src/frontend/parser/domains/quantum.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: safe Rust only.
//!
//! # Authority
//!
//! The authoritative syntax contracts are:
//! - grammar/DESIGN.md
//! - grammar/spec/syntax.md
//! - grammar/spec/quantum.md
//! - grammar/quantum/quantum.g4
//! - grammar/quantum/operations.g4
//! - grammar/quantum/circuits.g4
//! - grammar/quantum/parameters.g4
//!
//! # Ownership
//!
//! This module owns quantum-domain token-stream recognition and an
//! intermediate, loss-preserving syntax representation.
//!
//! It does not own:
//! - lexical recognition or token spelling registries;
//! - the canonical frontend AST;
//! - expression/type semantic analysis;
//! - quantum operation definitions or gate catalogs;
//! - QubitId or physical-resource identity;
//! - capability resolution or resource allocation;
//! - quantum IR, routing, scheduling, QEC, or execution.
//!
//! # Extensibility
//!
//! Operation names, namespaces, vendor identifiers, parameter expressions,
//! and target expressions are preserved as syntax. This module does not
//! enumerate gates or encode machine-capacity assumptions.
//!
//! # Integration
//!
//! The shared parser should call `QuantumDomainParser::parse_next()` at a
//! quantum-domain extension point. The returned syntax tree must then be
//! converted to the canonical AST by the parser's AST adapter.
//!
//! This module deliberately does not construct AST nodes whose constructors
//! or invariants have not yet been established in the repository.

use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Public configuration
// -----------------------------------------------------------------------------

/// Resource budgets for parsing quantum syntax.
///
/// These are implementation-safety budgets, not language-level limits.
/// They do not limit qubits, operations, registers, circuit width, or target
/// hardware capacity.
///
/// The embedding parser should coordinate these values with its shared parser
/// configuration and resource-budget accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumParserLimits {
    /// Maximum recursive quantum-block nesting.
    pub max_nesting_depth: usize,

    /// Maximum number of tokens consumed by one quantum construct.
    pub max_tokens_per_construct: usize,

    /// Whether otherwise-unrecognized statements inside quantum blocks may
    /// be retained as opaque extension syntax.
    ///
    /// When false, unknown statements fail explicitly.
    pub allow_opaque_extension_statements: bool,
}

impl Default for QuantumParserLimits {
    fn default() -> Self {
        Self {
            max_nesting_depth: 512,
            max_tokens_per_construct: usize::MAX,
            allow_opaque_extension_statements: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Loss-preserving token and syntax representation
// -----------------------------------------------------------------------------

/// A token retained by the intermediate quantum syntax representation.
///
/// The original spelling, token identity, and canonical source span survive
/// conversion. This type is not another lexer token type: it is an owned
/// syntax snapshot used to bridge parser and AST integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumSyntaxToken {
    pub kind: TokenKind,
    pub spelling: String,
    pub span: Span,
}

impl QuantumSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// A source-level quantum construct recognized by this adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantumItem {
    /// A `quantum ...` declaration that is not a circuit declaration.
    Declaration(QuantumDeclaration),

    /// A reusable `circuit ...` or `quantum circuit ...` declaration.
    Circuit(QuantumCircuit),

    /// An `apply ...;` operation invocation.
    Operation(QuantumOperation),

    /// A measurement statement.
    Measurement(QuantumMeasurement),

    /// A reset statement.
    Reset(QuantumReset),

    /// A syntax fragment preserved for a registered extension handler.
    ///
    /// This variant is produced only when opaque extension syntax is enabled.
    Extension(QuantumExtensionSyntax),
}

/// A general quantum-domain declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumDeclaration {
    /// Tokens between the declaration introducer and its body/terminator.
    ///
    /// These tokens are intentionally not interpreted as a new type system.
    pub header: Vec<QuantumSyntaxToken>,

    /// Parsed block, when the declaration uses a braced body.
    pub body: Option<Vec<QuantumItem>>,

    pub span: Span,
}

/// A reusable quantum circuit declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumCircuit {
    /// Circuit name, generic syntax, parameters, constraints and other header
    /// syntax, preserved without resolving identifiers.
    pub header: Vec<QuantumSyntaxToken>,

    /// Quantum-domain items contained in the circuit.
    pub body: Vec<QuantumItem>,

    pub span: Span,
}

/// A quantum operation invocation.
///
/// `designator` is the token sequence before the first top-level parenthesized
/// group. `argument_groups` preserves every top-level parenthesized group in
/// source order. `trailing` preserves any remaining syntax.
///
/// The groups are intentionally not prematurely classified as parameter
/// arguments or targets. For example, controlled-operation modifiers can
/// contain their own operation-designator groups. The canonical grammar and
/// AST adapter retain responsibility for interpreting those structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumOperation {
    pub designator: Vec<QuantumSyntaxToken>,
    pub argument_groups: Vec<Vec<QuantumSyntaxToken>>,
    pub trailing: Vec<QuantumSyntaxToken>,

    /// Complete invocation payload, excluding `apply` and the terminator.
    ///
    /// This guarantees that future grammar refinements can recover syntax
    /// that is not yet represented by the convenience fields above.
    pub payload: Vec<QuantumSyntaxToken>,

    pub span: Span,
}

/// A measurement statement.
///
/// Measurement target and result syntax remain token sequences until the
/// canonical grammar/AST adapter can apply the normative expression rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumMeasurement {
    pub payload: Vec<QuantumSyntaxToken>,
    pub span: Span,
}

/// A reset statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumReset {
    pub payload: Vec<QuantumSyntaxToken>,
    pub span: Span,
}

/// An opaque, explicitly permitted extension statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumExtensionSyntax {
    pub tokens: Vec<QuantumSyntaxToken>,
    pub span: Span,
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Machine-readable classification of quantum syntax failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum QuantumParseErrorKind {
    UnexpectedToken,
    MissingDeclarationName,
    MissingCircuitBody,
    MissingBlockTerminator,
    MissingStatementTerminator,
    EmptyOperation,
    EmptyMeasurement,
    EmptyReset,
    UnbalancedDelimiter,
    NestingBudgetExceeded,
    ConstructTokenBudgetExceeded,
    UnsupportedStatement,
    InvalidTokenStream,
}

/// A structured syntax error.
///
/// Human-readable messages are explanatory only. Callers should use `kind`
/// for programmatic handling and `span` for source highlighting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumParseError {
    pub kind: QuantumParseErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub expected: Option<String>,
    pub found: Option<String>,
}

impl QuantumParseError {
    fn new(
        kind: QuantumParseErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            expected: None,
            found: None,
        }
    }

    fn expected(
        kind: QuantumParseErrorKind,
        expected: impl Into<String>,
        found: Option<String>,
        span: Option<Span>,
    ) -> Self {
        let expected = expected.into();
        let message = match found.as_deref() {
            Some(actual) => format!("expected {expected}, found {actual}"),
            None => format!("expected {expected}, found end of input"),
        };

        Self {
            kind,
            message,
            span,
            expected: Some(expected),
            found,
        }
    }
}

impl fmt::Display for QuantumParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for QuantumParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental quantum-domain parser over the canonical lexer token stream.
///
/// The parser borrows tokens and does not re-tokenize source text. Its cursor
/// and recursion state belong to this invocation; no global mutable state is
/// used.
///
/// A parser instance is intended to be owned by one parser session. The
/// embedding parser should own cross-domain checkpoints, diagnostics,
/// compatibility policy and global resource-budget accounting.
pub struct QuantumDomainParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    limits: QuantumParserLimits,
    construct_start: usize,
}

impl<'tokens, 'src> QuantumDomainParser<'tokens, 'src> {
    /// Construct a quantum parser over a canonical token slice.
    ///
    /// Tokens may include trivia. Trivia is skipped using the canonical token
    /// channel, while source spans and spellings remain unchanged.
    #[must_use]
    pub fn new(
        tokens: &'tokens [Token<'src>],
        limits: QuantumParserLimits,
    ) -> Self {
        Self {
            tokens,
            index: 0,
            limits,
            construct_start: 0,
        }
    }

    /// Return the current token index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Set the token index for explicit integration with a shared parser.
    ///
    /// This operation is checked. It never accepts an index beyond the token
    /// slice and never modifies the underlying token stream.
    pub fn set_position(&mut self, position: usize) -> Result<(), QuantumParseError> {
        if position > self.tokens.len() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::InvalidTokenStream,
                "parser position lies beyond the token stream",
                None,
            ));
        }

        self.index = position;
        Ok(())
    }

    /// Parse the next quantum construct.
    ///
    /// Returns `Ok(None)` when the current significant token does not begin a
    /// construct owned by this module. This allows the main parser to retain
    /// ownership of ordinary classical, HDL, and other language syntax.
    ///
    /// This method parses exactly one construct. It does not require EOF,
    /// because it is designed for use as a sub-parser inside a shared parser.
    pub fn parse_next(&mut self) -> Result<Option<QuantumItem>, QuantumParseError> {
        self.skip_trivia();

        let Some(token) = self.current() else {
            return Ok(None);
        };

        if token.kind() == TokenKind::Eof {
            return Ok(None);
        }

        let word = token.lexeme();

        if word == "quantum" || token.kind() == TokenKind::KeywordQuantum {
            self.construct_start = self.index;
            self.bump();

            if self.current_is_word("circuit") {
                self.bump();
                return self.parse_circuit_after_introducer().map(Some);
            }

            return self.parse_declaration_after_introducer().map(Some);
        }

        if word == "circuit" || token.kind() == TokenKind::KeywordCircuit {
            self.construct_start = self.index;
            self.bump();
            return self.parse_circuit_after_introducer().map(Some);
        }

        if word == "apply" {
            self.construct_start = self.index;
            return self.parse_operation().map(Some);
        }

        if word == "measure" || word == "measurement" {
            self.construct_start = self.index;
            return self.parse_measurement().map(Some);
        }

        if word == "reset" {
            self.construct_start = self.index;
            return self.parse_reset().map(Some);
        }

        Ok(None)
    }

    /// Parse a contiguous sequence of quantum constructs until EOF or a
    /// closing block delimiter.
    ///
    /// This helper is useful for testing or for a parser layer that has
    /// already established a quantum-only region. It intentionally does not
    /// accept arbitrary ordinary-language statements unless the explicit
    /// extension policy permits them.
    pub fn parse_region(&mut self) -> Result<Vec<QuantumItem>, QuantumParseError> {
        let mut items = Vec::new();

        loop {
            self.skip_trivia();

            if self.at_eof() || self.current_is_kind(TokenKind::RightBrace) {
                break;
            }

            let before = self.index;

            if let Some(item) = self.parse_next()? {
                items.push(item);
                continue;
            }

            if self.limits.allow_opaque_extension_statements {
                items.push(QuantumItem::Extension(
                    self.parse_opaque_extension_statement()?,
                ));
                continue;
            }

            return Err(self.unexpected_current(
                QuantumParseErrorKind::UnsupportedStatement,
                "a supported quantum statement or an explicitly enabled extension",
            )
            .unwrap_or_else(|| {
                QuantumParseError::new(
                    QuantumParseErrorKind::UnexpectedToken,
                    "unexpected end of quantum region",
                    None,
                )
            }));

            // Progress is guaranteed by successful parse methods or by an
            // error. `before` is retained as an invariant marker for future
            // recovery integration.
            #[allow(unreachable_code)]
            if self.index <= before {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::InvalidTokenStream,
                    "parser made no progress",
                    self.current_span(),
                ));
            }
        }

        Ok(items)
    }

    // -------------------------------------------------------------------------
    // Declarations and circuits
    // -------------------------------------------------------------------------

    fn parse_declaration_after_introducer(
        &mut self,
    ) -> Result<QuantumItem, QuantumParseError> {
        let header = self.collect_header()?;

        if header.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::MissingDeclarationName,
                "expected quantum declaration syntax after `quantum`",
                self.current_span(),
            ));
        }

        let body = if self.current_is_kind(TokenKind::LeftBrace) {
            self.bump();
            Some(self.parse_block_items(1)?)
        } else {
            self.expect_terminator()?;
            None
        };

        let span = self.span_from(self.construct_start, self.index);

        Ok(QuantumItem::Declaration(QuantumDeclaration {
            header,
            body,
            span,
        }))
    }

    fn parse_circuit_after_introducer(
        &mut self,
    ) -> Result<QuantumItem, QuantumParseError> {
        let header = self.collect_header()?;

        if header.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::MissingDeclarationName,
                "expected circuit name and optional signature",
                self.current_span(),
            ));
        }

        if !self.current_is_kind(TokenKind::LeftBrace) {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::MissingCircuitBody,
                "expected a braced circuit body",
                self.current_span(),
            ));
        }

        self.bump();
        let body = self.parse_block_items(1)?;
        let span = self.span_from(self.construct_start, self.index);

        Ok(QuantumItem::Circuit(QuantumCircuit {
            header,
            body,
            span,
        }))
    }

    /// Consume declaration-header syntax up to a top-level body or terminator.
    ///
    /// Parenthesized and bracketed groups are preserved intact. A closing
    /// brace is not consumed: it belongs to the enclosing block.
    fn collect_header(&mut self) -> Result<Vec<QuantumSyntaxToken>, QuantumParseError> {
        let mut output = Vec::new();
        let mut parens = 0usize;
        let mut brackets = 0usize;
        let start = self.index;

        loop {
            self.check_construct_budget(start)?;

            let Some(token) = self.current() else {
                break;
            };

            if token.kind() == TokenKind::Eof {
                break;
            }

            if parens == 0
                && brackets == 0
                && (token.kind() == TokenKind::LeftBrace
                    || token.kind() == TokenKind::Semicolon)
            {
                break;
            }

            if token.kind() == TokenKind::RightBrace && parens == 0 && brackets == 0 {
                break;
            }

            match token.kind() {
                TokenKind::LeftParen => parens = parens.saturating_add(1),
                TokenKind::RightParen => {
                    if parens == 0 {
                        return Err(self.unbalanced(token));
                    }
                    parens -= 1;
                }
                TokenKind::LeftBracket => brackets = brackets.saturating_add(1),
                TokenKind::RightBracket => {
                    if brackets == 0 {
                        return Err(self.unbalanced(token));
                    }
                    brackets -= 1;
                }
                _ => {}
            }

            output.push(QuantumSyntaxToken::from_token(token));
            self.bump();
        }

        if parens != 0 || brackets != 0 {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::UnbalancedDelimiter,
                "unclosed delimiter in quantum declaration header",
                self.span_from(start, self.index),
            ));
        }

        Ok(output)
    }

    fn parse_block_items(
        &mut self,
        depth: usize,
    ) -> Result<Vec<QuantumItem>, QuantumParseError> {
        if depth > self.limits.max_nesting_depth {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::NestingBudgetExceeded,
                "quantum block nesting budget exhausted",
                self.current_span(),
            ));
        }

        let mut items = Vec::new();

        loop {
            self.skip_trivia();

            if self.at_eof() {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::MissingBlockTerminator,
                    "expected `}` before end of input",
                    self.current_span(),
                ));
            }

            if self.current_is_kind(TokenKind::RightBrace) {
                self.bump();
                return Ok(items);
            }

            let before = self.index;

            // Recognized nested quantum constructs are parsed recursively.
            if self.current_is_word("quantum") {
                self.construct_start = self.index;
                self.bump();

                let nested = if self.current_is_word("circuit") {
                    self.bump();
                    self.parse_circuit_after_introducer()?
                } else {
                    self.parse_declaration_after_introducer()?
                };

                items.push(nested);
            } else if self.current_is_word("circuit") {
                self.construct_start = self.index;
                self.bump();
                items.push(self.parse_circuit_after_introducer()?);
            } else if self.current_is_word("apply") {
                self.construct_start = self.index;
                items.push(self.parse_operation()?);
            } else if self.current_is_word("measure")
                || self.current_is_word("measurement")
            {
                self.construct_start = self.index;
                items.push(self.parse_measurement()?);
            } else if self.current_is_word("reset") {
                self.construct_start = self.index;
                items.push(self.parse_reset()?);
            } else if self.limits.allow_opaque_extension_statements {
                items.push(QuantumItem::Extension(
                    self.parse_opaque_extension_statement()?,
                ));
            } else {
                return Err(self.unexpected_current(
                    QuantumParseErrorKind::UnsupportedStatement,
                    "a supported quantum construct",
                )
                .unwrap_or_else(|| {
                    QuantumParseError::new(
                        QuantumParseErrorKind::UnexpectedToken,
                        "unexpected end of quantum block",
                        self.current_span(),
                    )
                }));
            }

            if self.index <= before {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::InvalidTokenStream,
                    "quantum block parser made no progress",
                    self.current_span(),
                ));
            }
        }
    }

    // -------------------------------------------------------------------------
    // Operations, measurement, and reset
    // -------------------------------------------------------------------------

    fn parse_operation(&mut self) -> Result<QuantumItem, QuantumParseError> {
        let start = self.index;
        self.expect_word("apply")?;

        let payload = self.collect_statement_payload()?;

        if payload.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::EmptyOperation,
                "`apply` requires an operation invocation",
                self.span_from(start, self.index),
            ));
        }

        let (designator, argument_groups, trailing) =
            Self::split_operation_payload(&payload);

        if designator.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::EmptyOperation,
                "quantum operation is missing its designator",
                self.span_from(start, self.index),
            ));
        }

        let span = self.span_from(start, self.index);

        Ok(QuantumItem::Operation(QuantumOperation {
            designator,
            argument_groups,
            trailing,
            payload,
            span,
        }))
    }

    fn parse_measurement(&mut self) -> Result<QuantumItem, QuantumParseError> {
        let start = self.index;
        self.bump();

        let payload = self.collect_statement_payload()?;

        if payload.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::EmptyMeasurement,
                "measurement requires an operand or target",
                self.span_from(start, self.index),
            ));
        }

        let span = self.span_from(start, self.index);

        Ok(QuantumItem::Measurement(QuantumMeasurement {
            payload,
            span,
        }))
    }

    fn parse_reset(&mut self) -> Result<QuantumItem, QuantumParseError> {
        let start = self.index;
        self.bump();

        let payload = self.collect_statement_payload()?;

        if payload.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::EmptyReset,
                "reset requires a target expression",
                self.span_from(start, self.index),
            ));
        }

        let span = self.span_from(start, self.index);

        Ok(QuantumItem::Reset(QuantumReset { payload, span }))
    }

    /// Collect one semicolon-terminated statement payload.
    ///
    /// Parentheses, brackets, and nested braces protect embedded semicolons.
    /// The terminating semicolon is consumed but is not included in payload.
    fn collect_statement_payload(
        &mut self,
    ) -> Result<Vec<QuantumSyntaxToken>, QuantumParseError> {
        let start = self.index;
        let mut output = Vec::new();
        let mut parens = 0usize;
        let mut brackets = 0usize;
        let mut braces = 0usize;

        loop {
            self.check_construct_budget(start)?;

            let Some(token) = self.current() else {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::MissingStatementTerminator,
                    "expected `;` before end of input",
                    self.current_span(),
                ));
            };

            if token.kind() == TokenKind::Eof {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::MissingStatementTerminator,
                    "expected `;` before end of input",
                    Some(*token.span()),
                ));
            }

            if token.kind() == TokenKind::RightBrace
                && parens == 0
                && brackets == 0
                && braces == 0
            {
                return Err(QuantumParseError::new(
                    QuantumParseErrorKind::MissingStatementTerminator,
                    "expected `;` before the end of the enclosing block",
                    Some(*token.span()),
                ));
            }

            if token.kind() == TokenKind::Semicolon
                && parens == 0
                && brackets == 0
                && braces == 0
            {
                self.bump();
                return Ok(output);
            }

            match token.kind() {
                TokenKind::LeftParen => parens = parens.saturating_add(1),
                TokenKind::RightParen => {
                    if parens == 0 {
                        return Err(self.unbalanced(token));
                    }
                    parens -= 1;
                }
                TokenKind::LeftBracket => brackets = brackets.saturating_add(1),
                TokenKind::RightBracket => {
                    if brackets == 0 {
                        return Err(self.unbalanced(token));
                    }
                    brackets -= 1;
                }
                TokenKind::LeftBrace => braces = braces.saturating_add(1),
                TokenKind::RightBrace => {
                    if braces == 0 {
                        return Err(self.unbalanced(token));
                    }
                    braces -= 1;
                }
                _ => {}
            }

            output.push(QuantumSyntaxToken::from_token(token));
            self.bump();
        }
    }

    /// Separate a payload into a leading designator, top-level parenthesized
    /// groups, and remaining tokens. Nested groups remain in their parent
    /// group; no operation-name catalog is consulted.
    fn split_operation_payload(
        payload: &[QuantumSyntaxToken],
    ) -> (
        Vec<QuantumSyntaxToken>,
        Vec<Vec<QuantumSyntaxToken>>,
        Vec<QuantumSyntaxToken>,
    ) {
        let Some(first_group) = payload
            .iter()
            .position(|token| token.kind == TokenKind::LeftParen)
        else {
            return (payload.to_vec(), Vec::new(), Vec::new());
        };

        let designator = payload[..first_group].to_vec();
        let mut groups = Vec::new();
        let mut trailing = Vec::new();
        let mut index = first_group;

        while index < payload.len() {
            if payload[index].kind != TokenKind::LeftParen {
                trailing.push(payload[index].clone());
                index += 1;
                continue;
            }

            let group_start = index + 1;
            let mut depth = 1usize;
            index += 1;

            while index < payload.len() && depth > 0 {
                match payload[index].kind {
                    TokenKind::LeftParen => depth = depth.saturating_add(1),
                    TokenKind::RightParen => depth -= 1,
                    _ => {}
                }
                index += 1;
            }

            if depth == 0 {
                groups.push(payload[group_start..index - 1].to_vec());
            } else {
                // Normally rejected by collect_statement_payload. Retaining
                // this fallback keeps this helper total for direct testing.
                groups.push(payload[group_start..].to_vec());
                break;
            }
        }

        (designator, groups, trailing)
    }

    // -------------------------------------------------------------------------
    // Explicit extension handling
    // -------------------------------------------------------------------------

    fn parse_opaque_extension_statement(
        &mut self,
    ) -> Result<QuantumExtensionSyntax, QuantumParseError> {
        let start = self.index;
        let tokens = self.collect_statement_payload()?;
        let span = self.span_from(start, self.index);

        if tokens.is_empty() {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::UnsupportedStatement,
                "empty extension statement",
                span,
            ));
        }

        Ok(QuantumExtensionSyntax { tokens, span })
    }

    // -------------------------------------------------------------------------
    // Cursor and shared-token helpers
    // -------------------------------------------------------------------------

    fn current(&self) -> Option<&Token<'src>> {
        self.tokens.get(self.index)
    }

    fn bump(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
        self.skip_trivia();
    }

    fn skip_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.index) {
            if token.is_trivia_channel()
                || matches!(
                    token.kind(),
                    TokenKind::Whitespace
                        | TokenKind::Newline
                        | TokenKind::LineComment
                        | TokenKind::BlockComment
                )
            {
                self.index += 1;
            } else {
                break;
            }
        }
    }

    fn at_eof(&self) -> bool {
        self.current()
            .is_none_or(|token| token.kind() == TokenKind::Eof)
    }

    fn current_is_kind(&self, kind: TokenKind) -> bool {
        self.current()
            .is_some_and(|token| token.kind() == kind)
    }

    fn current_is_word(&self, word: &str) -> bool {
        self.current()
            .is_some_and(|token| token.lexeme() == word)
    }

    fn expect_word(&mut self, word: &str) -> Result<(), QuantumParseError> {
        if self.current_is_word(word) {
            self.bump();
            Ok(())
        } else {
            Err(self.unexpected_current(
                QuantumParseErrorKind::UnexpectedToken,
                word,
            )
            .unwrap_or_else(|| {
                QuantumParseError::expected(
                    QuantumParseErrorKind::UnexpectedToken,
                    word,
                    None,
                    self.current_span(),
                )
            }))
        }
    }

    fn expect_terminator(&mut self) -> Result<(), QuantumParseError> {
        if self.current_is_kind(TokenKind::Semicolon) {
            self.bump();
            return Ok(());
        }

        Err(self.unexpected_current(
            QuantumParseErrorKind::UnexpectedToken,
            "`;` or a braced body",
        )
        .unwrap_or_else(|| {
            QuantumParseError::expected(
                QuantumParseErrorKind::UnexpectedToken,
                "`;` or a braced body",
                None,
                self.current_span(),
            )
        }))
    }

    fn unexpected_current(
        &self,
        kind: QuantumParseErrorKind,
        expected: &str,
    ) -> Option<QuantumParseError> {
        let token = self.current()?;

        Some(QuantumParseError::expected(
            kind,
            expected,
            Some(token.lexeme().to_owned()),
            Some(*token.span()),
        ))
    }

    fn unbalanced(&self, token: &Token<'_>) -> QuantumParseError {
        QuantumParseError::new(
            QuantumParseErrorKind::UnbalancedDelimiter,
            format!("unexpected closing delimiter `{}`", token.lexeme()),
            Some(*token.span()),
        )
    }

    fn current_span(&self) -> Option<Span> {
        self.current().map(|token| *token.span())
    }

    fn span_from(&self, start: usize, end: usize) -> Option<Span> {
        let first = self.tokens.get(start)?;
        let last = self.tokens.get(end.saturating_sub(1))?;

        first.span().cover(*last.span()).ok().or(Some(*first.span()))
    }

    fn check_construct_budget(&self, start: usize) -> Result<(), QuantumParseError> {
        let consumed = self.index.saturating_sub(start);

        if consumed > self.limits.max_tokens_per_construct {
            return Err(QuantumParseError::new(
                QuantumParseErrorKind::ConstructTokenBudgetExceeded,
                "quantum construct token budget exhausted",
                self.current_span(),
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Integration notes
// -----------------------------------------------------------------------------

// The shared parser should own the following responsibilities:
//
// 1. Call parse_next() only after the main parser has reached a quantum
//    extension point, or use its `None` result to continue ordinary parsing.
//
// 2. Convert QuantumItem and its child structures into the canonical AST.
//    Do not expose this intermediate representation as a competing canonical
//    AST. The adapter must preserve source spans and every retained token.
//
// 3. Coordinate QuantumParserLimits with the shared parser's resource-budget
//    accounting. The per-construct token budget must not replace the overall
//    source, token, diagnostic, or recovery budgets.
//
// 4. Use the authoritative ANTLR grammar to validate and refine operation
//    modifier, parameter, target, measurement, reset, declaration and circuit
//    syntax. This adapter deliberately does not replace the ANTLR grammar.
//
// 5. Keep semantic checks downstream: name resolution, operation signatures,
//    effect checking, measurement permissions, resource requirements,
//    capabilities, target feasibility, and lowering.
//
// 6. Add tests under src/frontend/parser/tests/ for malformed delimiters,
//    missing semicolons, nested circuits, operation modifiers, arbitrary
//    qualified operation names, Unicode spans, token budgets, and repeated
//    parse determinism.