//! Zamani Quantum Frontend — OpenQASM parser.
//!
//! This module converts the OpenQASM lexical token stream into the
//! OpenQASM-specific AST.
//!
//! Architectural boundary:
//!
//! ```text
//! OpenQASM source
//!      │
//!      ▼
//! lexer.rs
//!      │
//!      ▼
//! Token<'src>
//!      │
//!      ▼
//! parser.rs                 ← this module
//!      │
//!      ▼
//! OpenQASM AST
//!      │
//!      ▼
//! validation.rs
//!      │
//!      ▼
//! lowering.rs
//!      │
//!      ▼
//! Zamani Quantum IR
//! ```
//!
//! The parser deliberately does NOT:
//!
//! - construct QuantumCircuit;
//! - construct canonical IR Gate values;
//! - resolve OpenQASM symbols;
//! - resolve includes;
//! - access the filesystem;
//! - access the network;
//! - execute extern declarations;
//! - perform semantic type checking;
//! - perform gate capability validation;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - perform hardware mapping;
//! - execute a quantum program.
//!
//! The parser is responsible for syntax only.
//!
//! # Production requirements
//!
//! This parser is:
//!
//! - deterministic;
//! - bounded by explicit parser limits;
//! - panic-free for malformed input;
//! - independent of Quantum IR semantics;
//! - source-span preserving;
//! - non-I/O;
//! - non-executing;
//! - explicit about unsupported grammar;
//! - compatible with Rust 1.97.1;
//! - Rust 2021 compatible;
//! - dependency-free beyond the existing frontend modules.
//!
//! Semantic validation belongs in `validation.rs`.
//!
//! Include resolution belongs in `include.rs` / the importer boundary.
//!
//! Lowering belongs in `frontend/lowering.rs`.
//!
//! OpenQASM-specific syntax belongs in this module and `ast.rs`.
//!
//! # Important compatibility rule
//!
//! A parser error means that the input could not be represented as the
//! OpenQASM AST. It must never silently transform an unknown construct into a
//! different valid construct.
//!
//! For example, an unknown gate name is still parsed as a gate identifier.
//! Whether that gate exists, has the correct arity, or can be lowered is the
//! responsibility of semantic validation and lowering.
//!
//! Likewise, an unsupported language construct that can be represented by the
//! AST is parsed into its corresponding AST node. Constructs that cannot be
//! represented by the current AST are rejected explicitly rather than being
//! silently discarded.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! Rust 2021.
//! No nightly features.
//! No new dependencies.

use std::fmt;

use crate::quantum::frontend::core::source::{
    SourceId,
    SourceSpan,
};

use super::ast::{
    AliasDeclaration,
    Annotation,
    AnnotatedStatement,
    ArgumentDefinition,
    AssignmentOperator,
    AssignmentStatement,
    AssignmentValue,
    AstNode,
    BarrierStatement,
    BinaryOperator,
    BoxStatement,
    CalibrationGrammarStatement,
    ClassicalDeclaration,
    ControlStatement,
    ConstDeclaration,
    DefDefinition,
    DelayStatement,
    Designator,
    DurationLiteral,
    DurationUnit,
    ExternArgument,
    ExternDeclaration,
    Expression,
    ExpressionStatement,
    ForIterable,
    ForStatement,
    GateCall,
    GateDefinition,
    GateModifier,
    GateOperand,
    Identifier,
    IfStatement,
    IncludeStatement,
    IndexExpression,
    IntegerLiteral,
    IntegerRadix,
    IoDeclaration,
    MeasureAssignmentStatement,
    MeasureExpression,
    OldStyleDeclaration,
    OldStyleDeclarationKind,
    PhysicalQubit,
    PragmaStatement,
    Program,
    QuantumCallExpression,
    QuantumDeclaration,
    QuantumType,
    ResetStatement,
    ReturnSignature,
    ReturnStatement,
    ReturnValue,
    ScalarType,
    Scope,
    Statement,
    StatementOrScope,
    SwitchCase,
    SwitchStatement,
    TypeQualifier,
    TypeSpecifier,
    UnaryOperator,
    VersionDeclaration,
    WhileStatement,
};

use super::lexer::{
    LexError,
    OpenQasmLexer,
    Token,
    TokenKind,
};

// =============================================================================
// Parser configuration
// =============================================================================

/// Parser resource limits.
///
/// These limits are intentionally separate from lexer limits and from
/// `QuantumIrLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserLimits {
    /// Maximum number of tokens accepted by the parser.
    pub max_tokens: usize,

    /// Maximum number of statements in one lexical scope.
    pub max_statements_per_scope: usize,

    /// Maximum AST nodes created by one parse operation.
    pub max_ast_nodes: usize,

    /// Maximum syntactic nesting depth.
    pub max_nesting_depth: usize,

    /// Maximum expression nesting depth.
    pub max_expression_depth: usize,

    /// Maximum number of gate parameters.
    pub max_gate_parameters: usize,

    /// Maximum number of gate operands.
    pub max_gate_operands: usize,

    /// Maximum number of function arguments.
    pub max_arguments: usize,

    /// Maximum number of switch cases.
    pub max_switch_cases: usize,
}

impl Default for ParserLimits {
    fn default() -> Self {
        Self {
            max_tokens: 4_000_000,
            max_statements_per_scope: 1_000_000,
            max_ast_nodes: 4_000_000,
            max_nesting_depth: 4096,
            max_expression_depth: 4096,
            max_gate_parameters: 4096,
            max_gate_operands: 4096,
            max_arguments: 4096,
            max_switch_cases: 4096,
        }
    }
}

/// Parser configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserConfig {
    /// Source document ID attached to all generated AST spans.
    pub source_id: SourceId,

    /// Parser resource limits.
    pub limits: ParserLimits,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            source_id: SourceId::from_raw(0),
            limits: ParserLimits::default(),
        }
    }
}

// =============================================================================
// Parser errors
// =============================================================================

/// Parser result.
pub type ParserResult<T> = Result<T, ParseError>;

/// Stable parser error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseErrorKind {
    /// Token stream exceeded parser limits.
    TokenLimitExceeded,

    /// AST node limit exceeded.
    AstLimitExceeded,

    /// Statement limit exceeded.
    StatementLimitExceeded,

    /// Syntactic nesting limit exceeded.
    NestingLimitExceeded,

    /// Expression nesting limit exceeded.
    ExpressionDepthExceeded,

    /// Unexpected token.
    UnexpectedToken,

    /// Unexpected end of input.
    UnexpectedEof,

    /// Invalid version declaration.
    InvalidVersion,

    /// Invalid identifier position.
    ExpectedIdentifier,

    /// Invalid expression.
    InvalidExpression,

    /// Invalid type.
    InvalidType,

    /// Invalid designator.
    InvalidDesignator,

    /// Invalid gate operand.
    InvalidOperand,

    /// Invalid assignment.
    InvalidAssignment,

    /// Invalid statement.
    InvalidStatement,

    /// Unsupported syntax that cannot be represented by the current AST.
    UnsupportedSyntax,

    /// Numeric literal could not be represented by the AST literal model.
    InvalidLiteral,

    /// Internal source-span construction failed.
    InvalidSourceSpan,

    /// Lexer failure.
    Lexer,
}

impl ParseErrorKind {
    /// Stable machine-readable parser error code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::TokenLimitExceeded => "QASM-P001",
            Self::AstLimitExceeded => "QASM-P002",
            Self::StatementLimitExceeded => "QASM-P003",
            Self::NestingLimitExceeded => "QASM-P004",
            Self::ExpressionDepthExceeded => "QASM-P005",
            Self::UnexpectedToken => "QASM-P006",
            Self::UnexpectedEof => "QASM-P007",
            Self::InvalidVersion => "QASM-P008",
            Self::ExpectedIdentifier => "QASM-P009",
            Self::InvalidExpression => "QASM-P010",
            Self::InvalidType => "QASM-P011",
            Self::InvalidDesignator => "QASM-P012",
            Self::InvalidOperand => "QASM-P013",
            Self::InvalidAssignment => "QASM-P014",
            Self::InvalidStatement => "QASM-P015",
            Self::UnsupportedSyntax => "QASM-P016",
            Self::InvalidLiteral => "QASM-P017",
            Self::InvalidSourceSpan => "QASM-P018",
            Self::Lexer => "QASM-P019",
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// Structured parser error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<SourceSpan>,
    message: String,
}

impl ParseError {
    fn new(
        kind: ParseErrorKind,
        span: Option<SourceSpan>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Returns the parser error kind.
    #[must_use]
    pub const fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    /// Returns the stable parser error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Returns the source span when available.
    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(
                formatter,
                "{} at {}: {}",
                self.code(),
                span,
                self.message
            ),

            None => write!(
                formatter,
                "{}: {}",
                self.code(),
                self.message
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(error: LexError) -> Self {
        Self::new(
            ParseErrorKind::Lexer,
            None,
            error.to_string(),
        )
    }
}

// =============================================================================
// Public parser
// =============================================================================

/// Production OpenQASM parser.
///
/// The parser owns the token vector but does not own source text. Token
/// lexemes remain borrowed from the original source.
pub struct OpenQasmParser<'src> {
    tokens: Vec<Token<'src>>,
    config: ParserConfig,
    position: usize,
    ast_nodes: usize,
    nesting_depth: usize,
    expression_depth: usize,
}

impl<'src> OpenQasmParser<'src> {
    /// Lexes and parses OpenQASM source using default production limits.
    pub fn parse(
        source: &'src str,
        config: ParserConfig,
    ) -> ParserResult<Program> {
        let lexer = OpenQasmLexer::new(source)?;
        let tokens = lexer.tokenize()?;

        Self::from_tokens(tokens, config)?.parse_program()
    }

    /// Creates a parser from an already-tokenized OpenQASM source.
    ///
    /// This is useful for tooling and tests and preserves the same parser
    /// semantics as [`Self::parse`].
    pub fn from_tokens(
        tokens: Vec<Token<'src>>,
        config: ParserConfig,
    ) -> ParserResult<Self> {
        if tokens.len() > config.limits.max_tokens {
            return Err(ParseError::new(
                ParseErrorKind::TokenLimitExceeded,
                None,
                "token stream exceeds configured parser limit",
            ));
        }

        if tokens.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedEof,
                None,
                "token stream is empty",
            ));
        }

        if !tokens.last().is_some_and(|token| token.is_eof()) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedEof,
                None,
                "token stream does not contain an EOF token",
            ));
        }

        Ok(Self {
            tokens,
            config,
            position: 0,
            ast_nodes: 0,
            nesting_depth: 0,
            expression_depth: 0,
        })
    }

    /// Parses the complete token stream.
    pub fn parse_program(
        mut self,
    ) -> ParserResult<Program> {
        let start = self.current_span();

        let version = self.parse_optional_version()?;

        let mut statements = Vec::new();

        while !self.at(TokenKind::Eof) {
            if statements.len()
                >= self.config.limits.max_statements_per_scope
            {
                return Err(self.error(
                    ParseErrorKind::StatementLimitExceeded,
                    "program contains too many statements",
                ));
            }

            statements.push(self.parse_statement()?);
        }

        let end = self.current_span();

        let span = self.join_spans(start, end)?;

        Ok(Program::new(
            span,
            version,
            statements,
        ))
    }

    // =========================================================================
    // Program header
    // =========================================================================

    fn parse_optional_version(
        &mut self,
    ) -> ParserResult<Option<VersionDeclaration>> {
        if !self.at(TokenKind::KwOpenQasm) {
            return Ok(None);
        }

        let start = self.bump()?;

        let version_token = self.current();

        let raw = version_token.lexeme();

        let (major, minor) = parse_version_literal(raw)
            .ok_or_else(|| {
                self.error_at(
                    ParseErrorKind::InvalidVersion,
                    version_token,
                    "OpenQASM version must be a numeric major.minor value",
                )
            })?;

        let end = self.bump()?;

        self.expect(
            TokenKind::Semicolon,
            "expected `;` after OPENQASM version",
        )?;

        Ok(Some(VersionDeclaration::new(
            self.join_spans(
                start.span(),
                end.span(),
            )?,
            major,
            minor,
        )))
    }

    // =========================================================================
    // Statements
    // =========================================================================

    fn parse_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        match self.current().kind() {
            TokenKind::KwInclude => {
                self.parse_include()
            }

            TokenKind::KwDefcalGrammar => {
                self.parse_calibration_grammar()
            }

            TokenKind::KwQubit | TokenKind::KwQreg => {
                self.parse_quantum_declaration()
            }

            TokenKind::KwBit
            | TokenKind::KwBool
            | TokenKind::KwInt
            | TokenKind::KwUInt
            | TokenKind::KwFloat
            | TokenKind::KwAngle
            | TokenKind::KwComplex
            | TokenKind::KwDuration
            | TokenKind::KwStretch
            | TokenKind::KwConst
            | TokenKind::KwInput
            | TokenKind::KwOutput
            | TokenKind::KwReadonly => {
                self.parse_classical_or_io_declaration()
            }

            TokenKind::KwGate => {
                self.parse_gate_definition()
            }

            TokenKind::KwDef => {
                self.parse_def_definition()
            }

            TokenKind::KwExtern => {
                self.parse_extern_declaration()
            }

            TokenKind::KwMeasure => {
                self.parse_measure_statement()
            }

            TokenKind::KwReset => {
                self.parse_reset()
            }

            TokenKind::KwBarrier => {
                self.parse_barrier()
            }

            TokenKind::KwDelay => {
                self.parse_delay()
            }

            TokenKind::KwBox => {
                self.parse_box()
            }

            TokenKind::KwIf => {
                self.parse_if()
            }

            TokenKind::KwFor => {
                self.parse_for()
            }

            TokenKind::KwWhile => {
                self.parse_while()
            }

            TokenKind::KwReturn => {
                self.parse_return()
            }

            TokenKind::At => {
                self.parse_annotated_statement()
            }

            TokenKind::Hash => {
                self.parse_pragma()
            }

            TokenKind::KwBreak => {
                let token = self.bump()?;
                let semi = self.expect(
                    TokenKind::Semicolon,
                    "expected `;` after break",
                )?;

                Ok(Statement::Break(
                    ControlStatement::new(
                        self.join_spans(
                            token.span(),
                            semi.span(),
                        )?,
                    ),
                ))
            }

            TokenKind::KwContinue => {
                let token = self.bump()?;
                let semi = self.expect(
                    TokenKind::Semicolon,
                    "expected `;` after continue",
                )?;

                Ok(Statement::Continue(
                    ControlStatement::new(
                        self.join_spans(
                            token.span(),
                            semi.span(),
                        )?,
                    ),
                ))
            }

            TokenKind::Identifier
            | TokenKind::HardwareQubit => {
                self.parse_identifier_leading_statement()
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidStatement,
                "token cannot begin an OpenQASM statement",
            )),
        }
    }

    fn parse_include(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let path = self.expect(
            TokenKind::StringLiteral,
            "expected include string",
        )?;

        let decoded = decode_string_literal(
            path.lexeme(),
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::InvalidLiteral,
                path,
                "invalid OpenQASM string literal",
            )
        })?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after include",
        )?;

        Ok(Statement::Include(
            IncludeStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                decoded,
            ),
        ))
    }

    fn parse_calibration_grammar(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let name = self.expect(
            TokenKind::StringLiteral,
            "expected calibration grammar string",
        )?;

        let decoded = decode_string_literal(
            name.lexeme(),
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::InvalidLiteral,
                name,
                "invalid calibration grammar string",
            )
        })?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after calibration grammar",
        )?;

        Ok(Statement::CalibrationGrammar(
            CalibrationGrammarStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                decoded,
            ),
        ))
    }

    fn parse_quantum_declaration(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current().span();

        let ty = match self.current().kind() {
            TokenKind::KwQubit => {
                self.bump()?;

                let size = if self.consume(TokenKind::LBracket)? {
                    let value = self.parse_expression()?;
                    self.expect(
                        TokenKind::RBracket,
                        "expected `]` after qubit size",
                    )?;
                    Some(value)
                } else {
                    None
                };

                QuantumType::Qubit(size)
            }

            TokenKind::KwQreg => {
                self.bump()?;

                let size = if self.consume(TokenKind::LBracket)? {
                    let value = self.parse_expression()?;
                    self.expect(
                        TokenKind::RBracket,
                        "expected `]` after qreg size",
                    )?;
                    Some(value)
                } else {
                    None
                };

                QuantumType::QReg(size)
            }

            _ => {
                return Err(self.error(
                    ParseErrorKind::InvalidType,
                    "expected quantum type",
                ));
            }
        };

        let name = self.parse_identifier()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after quantum declaration",
        )?;

        Ok(Statement::QuantumDeclaration(
            QuantumDeclaration::new(
                self.join_spans(start, end.span())?,
                ty,
                name,
            ),
        ))
    }

    fn parse_classical_or_io_declaration(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current().span();

        let mut qualifier = None;

        if let Some(value) = self.parse_type_qualifier() {
            qualifier = Some(value);
        }

        let ty = self.parse_scalar_type()?;

        let name = self.parse_identifier()?;

        let initializer =
            if self.consume(TokenKind::Equal)? {
                Some(self.parse_expression()?)
            } else {
                None
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after declaration",
        )?;

        if matches!(
            qualifier,
            Some(
                TypeQualifier::Input
                    | TypeQualifier::Output
                    | TypeQualifier::Readonly
            )
        ) {
            Ok(Statement::IoDeclaration(
                IoDeclaration::new(
                    self.join_spans(
                        start,
                        end.span(),
                    )?,
                    qualifier.unwrap_or(
                        TypeQualifier::Mutable,
                    ),
                    ty,
                    name,
                    initializer,
                ),
            ))
        } else if matches!(
            qualifier,
            Some(TypeQualifier::Const)
        ) {
            let initializer =
                initializer.ok_or_else(|| {
                    self.error(
                        ParseErrorKind::InvalidStatement,
                        "`const` declaration requires an initializer",
                    )
                })?;

            Ok(Statement::ConstDeclaration(
                ConstDeclaration::new(
                    self.join_spans(
                        start,
                        end.span(),
                    )?,
                    ty,
                    name,
                    initializer,
                ),
            ))
        } else {
            Ok(Statement::ClassicalDeclaration(
                ClassicalDeclaration::new(
                    self.join_spans(
                        start,
                        end.span(),
                    )?,
                    qualifier,
                    ty,
                    name,
                    initializer,
                ),
            ))
        }
    }

    fn parse_type_qualifier(
        &mut self,
    ) -> Option<TypeQualifier> {
        let value = match self.current().kind() {
            TokenKind::KwConst => TypeQualifier::Const,
            TokenKind::KwInput => TypeQualifier::Input,
            TokenKind::KwOutput => TypeQualifier::Output,
            TokenKind::KwReadonly => TypeQualifier::Readonly,
            _ => return None,
        };

        let _ = self.bump();
        Some(value)
    }

    fn parse_scalar_type(
        &mut self,
    ) -> ParserResult<ScalarType> {
        let ty = match self.current().kind() {
            TokenKind::KwBool => {
                self.bump()?;
                ScalarType::Bool
            }

            TokenKind::KwBit => {
                self.bump()?;
                ScalarType::Bit(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwInt => {
                self.bump()?;
                ScalarType::Int(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwUInt => {
                self.bump()?;
                ScalarType::UInt(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwFloat => {
                self.bump()?;
                ScalarType::Float(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwAngle => {
                self.bump()?;
                ScalarType::Angle(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwComplex => {
                self.bump()?;
                ScalarType::Complex(
                    self.parse_optional_size()?,
                )
            }

            TokenKind::KwDuration => {
                self.bump()?;
                ScalarType::Duration
            }

            TokenKind::KwStretch => {
                self.bump()?;
                ScalarType::Stretch
            }

            _ => {
                return Err(self.error(
                    ParseErrorKind::InvalidType,
                    "expected OpenQASM classical type",
                ));
            }
        };

        Ok(ty)
    }

    fn parse_optional_size(
        &mut self,
    ) -> ParserResult<Option<Expression>> {
        if !self.consume(TokenKind::LBracket)? {
            return Ok(None);
        }

        let value = self.parse_expression()?;

        self.expect(
            TokenKind::RBracket,
            "expected `]` after type size",
        )?;

        Ok(Some(value))
    }

    // =========================================================================
    // Gate definitions
    // =========================================================================

    fn parse_gate_definition(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let name = self.parse_identifier()?;

        let parameters =
            if self.consume(TokenKind::LParen)? {
                let values = self.parse_identifier_list(
                    TokenKind::RParen,
                )?;
                values
            } else {
                Vec::new()
            };

        let mut qubits = Vec::new();

        loop {
            let identifier = self.parse_identifier()?;
            qubits.push(identifier);

            if !self.consume(TokenKind::Comma)? {
                break;
            }
        }

        self.enter_nesting(start.span())?;

        self.expect(
            TokenKind::LBrace,
            "expected `{` before gate body",
        )?;

        let body = self.parse_statement_block()?;

        self.leave_nesting();

        let end = self.previous_span();

        Ok(Statement::GateDefinition(
            GateDefinition::new(
                self.join_spans(
                    start.span(),
                    end,
                )?,
                name,
                parameters,
                qubits,
                body,
            ),
        ))
    }

    fn parse_identifier_list(
        &mut self,
        terminator: TokenKind,
    ) -> ParserResult<Vec<Identifier>> {
        let mut result = Vec::new();

        if self.at(terminator) {
            self.bump()?;
            return Ok(result);
        }

        loop {
            if result.len()
                >= self.config.limits.max_arguments
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many identifiers",
                ));
            }

            result.push(
                self.parse_identifier()?,
            );

            if !self.consume(TokenKind::Comma)? {
                break;
            }
        }

        self.expect(
            terminator,
            "expected list terminator",
        )?;

        Ok(result)
    }

    // =========================================================================
    // Subroutines
    // =========================================================================

    fn parse_def_definition(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let name = self.parse_identifier()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after subroutine name",
        )?;

        let arguments =
            self.parse_argument_definitions()?;

        let return_type =
            if self.consume(TokenKind::Arrow)? {
                Some(ReturnSignature::new(
                    self.current_span(),
                    self.parse_type_specifier()?,
                ))
            } else {
                None
            };

        self.enter_nesting(start.span())?;

        let body_start = self.expect(
            TokenKind::LBrace,
            "expected `{` before subroutine body",
        )?;

        let body_statements =
            self.parse_statement_block()?;

        self.leave_nesting();

        let body = Scope::new(
            self.join_spans(
                body_start.span(),
                self.previous_span(),
            )?,
            body_statements,
        );

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        Ok(Statement::DefDefinition(
            DefDefinition::new(
                span,
                name,
                arguments,
                return_type,
                body,
            ),
        ))
    }

    fn parse_argument_definitions(
        &mut self,
    ) -> ParserResult<Vec<ArgumentDefinition>> {
        let mut result = Vec::new();

        if self.consume(TokenKind::RParen)? {
            return Ok(result);
        }

        loop {
            if result.len()
                >= self.config.limits.max_arguments
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many subroutine arguments",
                ));
            }

            let start = self.current_span();

            let qualifier =
                self.parse_type_qualifier();

            let ty = self.parse_type_specifier()?;

            let name = self.parse_identifier()?;

            result.push(
                ArgumentDefinition::new(
                    self.join_spans(
                        start,
                        name.span(),
                    )?,
                    qualifier,
                    ty,
                    name,
                ),
            );

            if !self.consume(TokenKind::Comma)? {
                break;
            }
        }

        self.expect(
            TokenKind::RParen,
            "expected `)` after subroutine arguments",
        )?;

        Ok(result)
    }

    fn parse_type_specifier(
        &mut self,
    ) -> ParserResult<TypeSpecifier> {
        match self.current().kind() {
            TokenKind::KwQubit
            | TokenKind::KwQreg => {
                let ty =
                    self.parse_quantum_type()?;
                Ok(TypeSpecifier::Quantum(ty))
            }

            _ => Ok(
                TypeSpecifier::Classical(
                    self.parse_scalar_type()?,
                ),
            ),
        }
    }

    fn parse_quantum_type(
        &mut self,
    ) -> ParserResult<QuantumType> {
        match self.current().kind() {
            TokenKind::KwQubit => {
                self.bump()?;

                Ok(QuantumType::Qubit(
                    self.parse_optional_size()?,
                ))
            }

            TokenKind::KwQreg => {
                self.bump()?;

                Ok(QuantumType::QReg(
                    self.parse_optional_size()?,
                ))
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidType,
                "expected quantum type",
            )),
        }
    }

    // =========================================================================
    // Extern
    // =========================================================================

    fn parse_extern_declaration(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let name = self.parse_identifier()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after extern name",
        )?;

        let mut arguments = Vec::new();

        if !self.at(TokenKind::RParen) {
            loop {
                if arguments.len()
                    >= self.config.limits.max_arguments
                {
                    return Err(self.error(
                        ParseErrorKind::AstLimitExceeded,
                        "too many extern arguments",
                    ));
                }

                let arg_start =
                    self.current_span();

                let ty =
                    self.parse_type_specifier()?;

                arguments.push(
                    ExternArgument::new(
                        self.join_spans(
                            arg_start,
                            self.previous_span(),
                        )?,
                        ty,
                    ),
                );

                if !self.consume(TokenKind::Comma)? {
                    break;
                }
            }
        }

        self.expect(
            TokenKind::RParen,
            "expected `)` after extern arguments",
        )?;

        let return_type =
            if self.consume(TokenKind::Arrow)? {
                Some(ReturnSignature::new(
                    self.current_span(),
                    self.parse_type_specifier()?,
                ))
            } else {
                None
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after extern declaration",
        )?;

        Ok(Statement::ExternDeclaration(
            ExternDeclaration::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                name,
                arguments,
                return_type,
            ),
        ))
    }

    // =========================================================================
    // Quantum operations
    // =========================================================================

    fn parse_measure_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operand =
            self.parse_designator()?;

        let source =
            MeasureExpression::new(
                self.join_spans(
                    start.span(),
                    operand.span(),
                )?,
                operand,
            );

        if self.consume(TokenKind::Arrow)? {
            let destination =
                self.parse_designator()?;

            let end = self.expect(
                TokenKind::Semicolon,
                "expected `;` after measurement assignment",
            )?;

            Ok(
                Statement::MeasureAssignment(
                    MeasureAssignmentStatement::new(
                        self.join_spans(
                            start.span(),
                            end.span(),
                        )?,
                        source,
                        destination,
                    ),
                ),
            )
        } else {
            let end = self.expect(
                TokenKind::Semicolon,
                "expected `;` after measurement",
            )?;

            Ok(Statement::Expression(
                ExpressionStatement::new(
                    self.join_spans(
                        start.span(),
                        end.span(),
                    )?,
                    Expression::Measure(source),
                ),
            ))
        }
    }

    fn parse_reset(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operands =
            self.parse_operand_list()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after reset",
        )?;

        Ok(Statement::Reset(
            ResetStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                operands,
            ),
        ))
    }

    fn parse_barrier(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operands =
            self.parse_operand_list()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after barrier",
        )?;

        Ok(Statement::Barrier(
            BarrierStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                operands,
            ),
        ))
    }

    fn parse_delay(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let duration =
            self.parse_expression()?;

        let operands =
            self.parse_operand_list()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after delay",
        )?;

        Ok(Statement::Delay(
            DelayStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                duration,
                operands,
            ),
        ))
    }

    fn parse_box(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let designator =
            if self.consume(TokenKind::LBracket)? {
                let expression =
                    self.parse_expression()?;

                self.expect(
                    TokenKind::RBracket,
                    "expected `]` after box duration",
                )?;

                Some(expression)
            } else {
                None
            };

        self.enter_nesting(start.span())?;

        self.expect(
            TokenKind::LBrace,
            "expected `{` after box",
        )?;

        let body =
            self.parse_statement_block()?;

        self.leave_nesting();

        let end = self.previous_span();

        Ok(Statement::Box(
            BoxStatement::new(
                self.join_spans(
                    start.span(),
                    end,
                )?,
                designator,
                body,
            ),
        ))
    }

    // =========================================================================
    // Control flow
    // =========================================================================

    fn parse_if(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after if",
        )?;

        let condition =
            self.parse_expression()?;

        self.expect(
            TokenKind::RParen,
            "expected `)` after if condition",
        )?;

        self.enter_nesting(start.span())?;

        let then_body =
            self.parse_statement_or_scope()?;

        let else_body =
            if self.consume(TokenKind::KwElse)? {
                Some(
                    self.parse_statement_or_scope()?,
                )
            } else {
                None
            };

        self.leave_nesting();

        let end = self.previous_span();

        Ok(Statement::If(
            IfStatement::new(
                self.join_spans(
                    start.span(),
                    end,
                )?,
                condition,
                then_body,
                else_body,
            ),
        ))
    }

    fn parse_for(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let variable_type =
            self.parse_scalar_type()?;

        let variable =
            self.parse_identifier()?;

        self.expect(
            TokenKind::KwIn,
            "expected `in` in for loop",
        )?;

        let iterable =
            self.parse_for_iterable()?;

        self.enter_nesting(start.span())?;

        let body =
            self.parse_statement_or_scope()?;

        self.leave_nesting();

        let end = self.previous_span();

        Ok(Statement::For(
            ForStatement::new(
                self.join_spans(
                    start.span(),
                    end,
                )?,
                variable_type,
                variable,
                iterable,
                body,
            ),
        ))
    }

    fn parse_for_iterable(
        &mut self,
    ) -> ParserResult<ForIterable> {
        let first =
            self.parse_expression()?;

        if !self.consume(TokenKind::Colon)? {
            return Ok(
                ForIterable::Expression(first)
            );
        }

        let second =
            self.parse_expression()?;

        if self.consume(TokenKind::Colon)? {
            let third =
                self.parse_expression()?;

            Ok(ForIterable::Range {
                start: first,
                step: Some(second),
                stop: third,
            })
        } else {
            Ok(ForIterable::Range {
                start: first,
                step: None,
                stop: second,
            })
        }
    }

    fn parse_while(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after while",
        )?;

        let condition =
            self.parse_expression()?;

        self.expect(
            TokenKind::RParen,
            "expected `)` after while condition",
        )?;

        self.enter_nesting(start.span())?;

        let body =
            self.parse_statement_or_scope()?;

        self.leave_nesting();

        let end = self.previous_span();

        Ok(Statement::While(
            WhileStatement::new(
                self.join_spans(
                    start.span(),
                    end,
                )?,
                condition,
                body,
            ),
        ))
    }

    fn parse_return(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        if self.consume(TokenKind::Semicolon)? {
            return Ok(Statement::Return(
                ReturnStatement::new(
                    self.join_spans(
                        start.span(),
                        self.previous_span(),
                    )?,
                    None,
                ),
            ));
        }

        let expression =
            self.parse_expression()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after return",
        )?;

        Ok(Statement::Return(
            ReturnStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                Some(
                    ReturnValue::Expression(
                        expression,
                    ),
                ),
            ),
        ))
    }

    fn parse_statement_or_scope(
        &mut self,
    ) -> ParserResult<StatementOrScope> {
        if self.at(TokenKind::LBrace) {
            let start = self.bump()?;

            let body =
                self.parse_statement_block()?;

            let end =
                self.previous_span();

            Ok(
                StatementOrScope::Scope(
                    Scope::new(
                        self.join_spans(
                            start.span(),
                            end,
                        )?,
                        body,
                    ),
                ),
            )
        } else {
            Ok(
                StatementOrScope::Statement(
                    Box::new(
                        self.parse_statement()?,
                    ),
                ),
            )
        }
    }

    fn parse_statement_block(
        &mut self,
    ) -> ParserResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                return Err(self.error(
                    ParseErrorKind::UnexpectedEof,
                    "unterminated `{ ... }` block",
                ));
            }

            if statements.len()
                >= self.config.limits.max_statements_per_scope
            {
                return Err(self.error(
                    ParseErrorKind::StatementLimitExceeded,
                    "scope contains too many statements",
                ));
            }

            statements.push(
                self.parse_statement()?,
            );
        }

        self.expect(
            TokenKind::RBrace,
            "expected `}`",
        )?;

        Ok(statements)
    }

    // =========================================================================
    // Annotations / pragmas
    // =========================================================================

    fn parse_annotated_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current_span();
        let mut annotations = Vec::new();

        while self.at(TokenKind::At) {
            let at = self.bump()?;

            let keyword =
                self.parse_identifier()?;

            let payload =
                if self.at(TokenKind::Identifier)
                    || self.at(TokenKind::StringLiteral)
                {
                    Some(
                        self.bump()?
                            .lexeme()
                            .to_owned(),
                    )
                } else {
                    None
                };

            annotations.push(
                Annotation::new(
                    self.join_spans(
                        at.span(),
                        keyword.span(),
                    )?,
                    keyword,
                    payload,
                ),
            );
        }

        let statement =
            self.parse_statement()?;

        let end = statement.span();

        Ok(Statement::Annotated(
            AnnotatedStatement::new(
                self.join_spans(
                    start,
                    end,
                )?,
                annotations,
                statement,
            ),
        ))
    }

    fn parse_pragma(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let mut payload = String::new();

        while !self.at(TokenKind::Semicolon)
            && !self.at(TokenKind::Eof)
        {
            if !payload.is_empty() {
                payload.push(' ');
            }

            payload.push_str(
                self.bump()?.lexeme(),
            );
        }

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after pragma",
        )?;

        Ok(Statement::Pragma(
            PragmaStatement::new(
                self.join_spans(
                    start.span(),
                    end.span(),
                )?,
                payload,
            ),
        ))
    }

    // =========================================================================
    // Identifier-leading statements
    // =========================================================================

    fn parse_identifier_leading_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let save = self.position;

        let modifiers =
            self.parse_gate_modifiers()?;

        if self.at(TokenKind::Identifier) {
            let name =
                self.parse_identifier()?;

            if self.at(TokenKind::LParen)
                || self.is_operand_start()
            {
                let parameters =
                    self.parse_optional_expression_list()?;

                let operands =
                    self.parse_operand_list()?;

                let end = self.expect(
                    TokenKind::Semicolon,
                    "expected `;` after gate invocation",
                )?;

                return Ok(Statement::GateCall(
                    GateCall::new(
                        self.join_spans(
                            self.tokens[save].span(),
                            end.span(),
                        )?,
                        modifiers,
                        name,
                        parameters,
                        operands,
                    ),
                ));
            }

            self.position = save;
            return self.parse_assignment_or_expression_statement();
        }

        self.position = save;

        self.parse_assignment_or_expression_statement()
    }

    fn parse_gate_modifiers(
        &mut self,
    ) -> ParserResult<Vec<GateModifier>> {
        let mut modifiers = Vec::new();

        loop {
            let modifier = match self.current().lexeme() {
                "ctrl" if self.at(TokenKind::Identifier) => {
                    self.bump()?;
                    GateModifier::Ctrl
                }

                "negctrl"
                    if self.at(TokenKind::Identifier) =>
                {
                    self.bump()?;
                    GateModifier::NegCtrl
                }

                "inv" if self.at(TokenKind::Identifier) => {
                    self.bump()?;
                    GateModifier::Inv
                }

                _ => break,
            };

            modifiers.push(modifier);
        }

        Ok(modifiers)
    }

    fn parse_assignment_or_expression_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current_span();

        let expression =
            self.parse_expression()?;

        if let Expression::Designator(target) =
            &expression
        {
            if let Some(operator) =
                self.assignment_operator()
            {
                let _ = self.bump()?;

                let value =
                    self.parse_expression()?;

                let end = self.expect(
                    TokenKind::Semicolon,
                    "expected `;` after assignment",
                )?;

                return Ok(Statement::Assignment(
                    AssignmentStatement::new(
                        self.join_spans(
                            start,
                            end.span(),
                        )?,
                        target.clone(),
                        operator,
                        AssignmentValue::Expression(
                            value,
                        ),
                    ),
                ));
            }
        }

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after expression",
        )?;

        Ok(Statement::Expression(
            ExpressionStatement::new(
                self.join_spans(
                    start,
                    end.span(),
                )?,
                expression,
            ),
        ))
    }

    fn assignment_operator(
        &self,
    ) -> Option<AssignmentOperator> {
        match self.current().kind() {
            TokenKind::Equal =>
                Some(AssignmentOperator::Assign),

            TokenKind::PlusEqual =>
                Some(AssignmentOperator::AddAssign),

            TokenKind::MinusEqual =>
                Some(AssignmentOperator::SubtractAssign),

            TokenKind::StarEqual =>
                Some(AssignmentOperator::MultiplyAssign),

            TokenKind::SlashEqual =>
                Some(AssignmentOperator::DivideAssign),

            TokenKind::PercentEqual =>
                Some(AssignmentOperator::RemainderAssign),

            TokenKind::BitAnd =>
                Some(AssignmentOperator::BitAndAssign),

            TokenKind::BitOr =>
                Some(AssignmentOperator::BitOrAssign),

            TokenKind::BitXor =>
                Some(AssignmentOperator::BitXorAssign),

            TokenKind::ShiftLeft =>
                Some(AssignmentOperator::ShiftLeftAssign),

            TokenKind::ShiftRight =>
                Some(AssignmentOperator::ShiftRightAssign),

            _ => None,
        }
    }

    // =========================================================================
    // Operands
    // =========================================================================

    fn parse_operand_list(
        &mut self,
    ) -> ParserResult<Vec<GateOperand>> {
        let mut result = Vec::new();

        if !self.is_operand_start() {
            return Ok(result);
        }

        loop {
            if result.len()
                >= self.config.limits.max_gate_operands
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many gate operands",
                ));
            }

            result.push(
                self.parse_gate_operand()?,
            );

            if !self.consume(TokenKind::Comma)? {
                break;
            }
        }

        Ok(result)
    }

    fn is_operand_start(&self) -> bool {
        matches!(
            self.current().kind(),
            TokenKind::Identifier
                | TokenKind::HardwareQubit
        )
    }

    fn parse_gate_operand(
        &mut self,
    ) -> ParserResult<GateOperand> {
        if self.at(TokenKind::HardwareQubit) {
            let token = self.bump()?;

            let index =
                token.lexeme()
                    .strip_prefix('$')
                    .and_then(|value| {
                        value.parse::<u64>().ok()
                    })
                    .ok_or_else(|| {
                        self.error_at(
                            ParseErrorKind::InvalidOperand,
                            token,
                            "invalid physical qubit index",
                        )
                    })?;

            return Ok(
                GateOperand::Physical(
                    PhysicalQubit::new(
                        self.source_span(token.span())?,
                        index,
                    ),
                ),
            );
        }

        let designator =
            self.parse_designator()?;

        Ok(
            GateOperand::Designator(
                designator,
            ),
        )
    }

    fn parse_designator(
        &mut self,
    ) -> ParserResult<Designator> {
        let name =
            self.parse_identifier()?;

        let index =
            if self.consume(TokenKind::LBracket)? {
                let value =
                    self.parse_index_expression()?;

                self.expect(
                    TokenKind::RBracket,
                    "expected `]` after designator",
                )?;

                Some(value)
            } else {
                None
            };

        let span = if let Some(ref index) = index {
            self.join_spans(
                name.span(),
                self.previous_span(),
            )?
        } else {
            name.span()
        };

        Ok(
            Designator::new(
                span,
                name,
                index,
            ),
        )
    }

    fn parse_index_expression(
        &mut self,
    ) -> ParserResult<IndexExpression> {
        let first =
            self.parse_expression()?;

        if self.consume(TokenKind::Colon)? {
            let second =
                if self.at(TokenKind::Colon)
                    || self.at(TokenKind::RBracket)
                {
                    None
                } else {
                    Some(
                        self.parse_expression()?
                    )
                };

            if self.consume(TokenKind::Colon)? {
                let third =
                    if self.at(TokenKind::RBracket) {
                        None
                    } else {
                        Some(
                            self.parse_expression()?
                        )
                    };

                return Ok(
                    IndexExpression::Range {
                        start: Some(first),
                        step: second,
                        stop: third,
                    },
                );
            }

            return Ok(
                IndexExpression::Slice {
                    start: Some(first),
                    stop: second,
                },
            );
        }

        Ok(IndexExpression::Index(first))
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    fn parse_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(
        &mut self,
        minimum_precedence: u8,
    ) -> ParserResult<Expression> {
        self.enter_expression()?;

        let mut left =
            self.parse_unary_expression()?;

        loop {
            let Some((operator, precedence)) =
                self.binary_operator()
            else {
                break;
            };

            if precedence < minimum_precedence {
                break;
            }

            self.bump()?;

            let next_minimum =
                if matches!(
                    operator,
                    BinaryOperator::Power
                ) {
                    precedence
                } else {
                    precedence + 1
                };

            let right =
                self.parse_binary_expression(
                    next_minimum,
                )?;

            let span =
                self.join_spans(
                    left.span(),
                    right.span(),
                )?;

            left = Expression::Binary {
                node: AstNode::new(span),
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        self.leave_expression();

        Ok(left)
    }

    fn binary_operator(
        &self,
    ) -> Option<(BinaryOperator, u8)> {
        let value = match self.current().kind() {
            TokenKind::LogicalOr =>
                (BinaryOperator::LogicalOr, 1),

            TokenKind::LogicalAnd =>
                (BinaryOperator::LogicalAnd, 2),

            TokenKind::BitOr =>
                (BinaryOperator::BitOr, 3),

            TokenKind::BitXor =>
                (BinaryOperator::BitXor, 4),

            TokenKind::BitAnd =>
                (BinaryOperator::BitAnd, 5),

            TokenKind::EqualEqual =>
                (BinaryOperator::Equal, 6),

            TokenKind::NotEqual =>
                (BinaryOperator::NotEqual, 6),

            TokenKind::Less =>
                (BinaryOperator::Less, 7),

            TokenKind::LessEqual =>
                (BinaryOperator::LessEqual, 7),

            TokenKind::Greater =>
                (BinaryOperator::Greater, 7),

            TokenKind::GreaterEqual =>
                (BinaryOperator::GreaterEqual, 7),

            TokenKind::ShiftLeft =>
                (BinaryOperator::ShiftLeft, 8),

            TokenKind::ShiftRight =>
                (BinaryOperator::ShiftRight, 8),

            TokenKind::Plus =>
                (BinaryOperator::Add, 9),

            TokenKind::Minus =>
                (BinaryOperator::Subtract, 9),

            TokenKind::Star =>
                (BinaryOperator::Multiply, 10),

            TokenKind::Slash =>
                (BinaryOperator::Divide, 10),

            TokenKind::Percent =>
                (BinaryOperator::Remainder, 10),

            TokenKind::Power =>
                (BinaryOperator::Power, 11),

            _ => return None,
        };

        Some(value)
    }

    fn parse_unary_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        let operator =
            match self.current().kind() {
                TokenKind::Plus =>
                    Some(UnaryOperator::Plus),

                TokenKind::Minus =>
                    Some(UnaryOperator::Minus),

                TokenKind::LogicalNot =>
                    Some(UnaryOperator::LogicalNot),

                TokenKind::BitNot =>
                    Some(UnaryOperator::BitNot),

                _ => None,
            };

        if let Some(operator) = operator {
            let start = self.bump()?;

            let operand =
                self.parse_unary_expression()?;

            let span =
                self.join_spans(
                    start.span(),
                    operand.span(),
                )?;

            return Ok(
                Expression::Unary {
                    node: AstNode::new(span),
                    operator,
                    operand: Box::new(
                        operand,
                    ),
                },
            );
        }

        self.parse_primary_expression()
    }

    fn parse_primary_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        match self.current().kind() {
            TokenKind::IntegerLiteral => {
                let token = self.bump()?;

                let radix =
                    integer_radix(
                        token.lexeme(),
                    );

                Ok(
                    Expression::IntegerLiteral {
                        node: AstNode::new(
                            self.source_span(
                                token.span(),
                            )?,
                        ),
                        value:
                            IntegerLiteral::new(
                                normalize_integer_literal(
                                    token.lexeme(),
                                ),
                                radix,
                            ),
                    },
                )
            }

            TokenKind::FloatLiteral => {
                let token = self.bump()?;

                Ok(
                    Expression::FloatLiteral {
                        node: AstNode::new(
                            self.source_span(
                                token.span(),
                            )?,
                        ),
                        value:
                            super::ast::FloatLiteral::new(
                                normalize_numeric_literal(
                                    token.lexeme(),
                                ),
                            ),
                    },
                )
            }

            TokenKind::DurationLiteral => {
                let token = self.bump()?;

                let (raw_value, unit) =
                    split_duration_literal(
                        token.lexeme(),
                    )
                    .ok_or_else(|| {
                        self.error_at(
                            ParseErrorKind::InvalidLiteral,
                            token,
                            "invalid duration literal",
                        )
                    })?;

                let value =
                    if raw_value.contains('.')
                        || raw_value.contains('e')
                        || raw_value.contains('E')
                    {
                        Expression::FloatLiteral {
                            node: AstNode::new(
                                self.source_span(
                                    token.span(),
                                )?,
                            ),
                            value:
                                super::ast::FloatLiteral::new(
                                    raw_value.to_owned(),
                                ),
                        }
                    } else {
                        Expression::IntegerLiteral {
                            node: AstNode::new(
                                self.source_span(
                                    token.span(),
                                )?,
                            ),
                            value:
                                IntegerLiteral::new(
                                    raw_value.to_owned(),
                                    IntegerRadix::Decimal,
                                ),
                        }
                    };

                Ok(
                    Expression::DurationLiteral {
                        node: AstNode::new(
                            self.source_span(
                                token.span(),
                            )?,
                        ),
                        value:
                            DurationLiteral::new(
                                value,
                                unit,
                            ),
                    },
                )
            }

            TokenKind::KwTrue
            | TokenKind::KwFalse => {
                let token = self.bump()?;

                Ok(
                    Expression::BoolLiteral {
                        node: AstNode::new(
                            self.source_span(
                                token.span(),
                            )?,
                        ),
                        value:
                            token.kind()
                                == TokenKind::KwTrue,
                    },
                )
            }

            TokenKind::Identifier => {
                let designator =
                    self.parse_designator()?;

                if self.consume(
                    TokenKind::LParen,
                )? {
                    let arguments =
                        self.parse_expression_list(
                            TokenKind::RParen,
                        )?;

                    Ok(
                        Expression::FunctionCall {
                            node: AstNode::new(
                                self.join_spans(
                                    designator.span(),
                                    self.previous_span(),
                                )?,
                            ),
                            name: designator
                                .name()
                                .clone(),
                            arguments,
                        },
                    )
                } else {
                    Ok(
                        Expression::Designator(
                            designator,
                        ),
                    )
                }
            }

            TokenKind::LParen => {
                let start = self.bump()?;

                let expression =
                    self.parse_expression()?;

                let end = self.expect(
                    TokenKind::RParen,
                    "expected `)`",
                )?;

                Ok(
                    Expression::Parenthesized {
                        node: AstNode::new(
                            self.join_spans(
                                start.span(),
                                end.span(),
                            )?,
                        ),
                        expression:
                            Box::new(expression),
                    },
                )
            }

            TokenKind::KwMeasure => {
                let start = self.bump()?;

                let operand =
                    self.parse_designator()?;

                Ok(Expression::Measure(
                    MeasureExpression::new(
                        self.join_spans(
                            start.span(),
                            operand.span(),
                        )?,
                        operand,
                    ),
                ))
            }

            TokenKind::KwPi => {
                let token = self.bump()?;

                Ok(
                    Expression::Identifier(
                        Identifier::new(
                            self.source_span(
                                token.span(),
                            )?,
                            token.lexeme()
                                .to_owned(),
                        )
                        .ok_or_else(|| {
                            self.error_at(
                                ParseErrorKind::ExpectedIdentifier,
                                token,
                                "invalid pi identifier",
                            )
                        })?,
                    ),
                )
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidExpression,
                "expected an OpenQASM expression",
            )),
        }
    }

    fn parse_optional_expression_list(
        &mut self,
    ) -> ParserResult<Vec<Expression>> {
        if !self.consume(TokenKind::LParen)? {
            return Ok(Vec::new());
        }

        self.parse_expression_list(
            TokenKind::RParen,
        )
    }

    fn parse_expression_list(
        &mut self,
        terminator: TokenKind,
    ) -> ParserResult<Vec<Expression>> {
        let mut result = Vec::new();

        if self.consume(terminator)? {
            return Ok(result);
        }

        loop {
            if result.len()
                >= self.config.limits.max_gate_parameters
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many expressions in list",
                ));
            }

            result.push(
                self.parse_expression()?,
            );

            if !self.consume(TokenKind::Comma)? {
                break;
            }
        }

        self.expect(
            terminator,
            "expected expression-list terminator",
        )?;

        Ok(result)
    }

    // =========================================================================
    // Identifiers and tokens
    // =========================================================================

    fn parse_identifier(
        &mut self,
    ) -> ParserResult<Identifier> {
        let token = self.current();

        if !token.kind().is_identifier_like()
            || token.kind()
                == TokenKind::HardwareQubit
        {
            return Err(self.error_at(
                ParseErrorKind::ExpectedIdentifier,
                token,
                "expected an OpenQASM identifier",
            ));
        }

        let token = self.bump()?;

        Identifier::new(
            self.source_span(token.span())?,
            token.lexeme().to_owned(),
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::ExpectedIdentifier,
                token,
                "identifier cannot be empty",
            )
        })
    }

    fn current(
        &self,
    ) -> Token<'src> {
        self.tokens[
            self.position
                .min(self.tokens.len() - 1)
        ]
    }

    fn current_span(
        &self,
    ) -> SourceSpan {
        self.source_span(
            self.current().span(),
        )
        .unwrap_or_else(|_| {
            SourceSpan::point(
                self.config.source_id,
                0,
            )
        })
    }

    fn previous_span(
        &self,
    ) -> SourceSpan {
        if self.position == 0 {
            self.current_span()
        } else {
            self.source_span(
                self.tokens[
                    self.position - 1
                ]
                .span(),
            )
            .unwrap_or_else(|_| {
                self.current_span()
            })
        }
    }

    fn at(
        &self,
        kind: TokenKind,
    ) -> bool {
        self.current().kind() == kind
    }

    fn consume(
        &mut self,
        kind: TokenKind,
    ) -> ParserResult<bool> {
        if self.at(kind) {
            self.bump()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn expect(
        &mut self,
        kind: TokenKind,
        message: &str,
    ) -> ParserResult<Token<'src>> {
        if self.at(kind) {
            return self.bump();
        }

        Err(self.error(
            ParseErrorKind::UnexpectedToken,
            message,
        ))
    }

    fn bump(
        &mut self,
    ) -> ParserResult<Token<'src>> {
        let token = self.current();

        if token.is_eof() {
            return Err(self.error_at(
                ParseErrorKind::UnexpectedEof,
                token,
                "unexpected end of OpenQASM input",
            ));
        }

        self.position =
            self.position.saturating_add(1);

        Ok(token)
    }

    fn source_span(
        &self,
        span: super::lexer::Span,
    ) -> ParserResult<SourceSpan> {
        SourceSpan::new(
            self.config.source_id,
            span.start,
            span.end,
        )
        .ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InvalidSourceSpan,
                None,
                "lexer produced an invalid source span",
            )
        })
    }

    fn join_spans(
        &self,
        start: SourceSpan,
        end: SourceSpan,
    ) -> ParserResult<SourceSpan> {
        start.join(end).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InvalidSourceSpan,
                Some(start),
                "AST node spans belong to different source documents",
            )
        })
    }

    fn error(
        &self,
        kind: ParseErrorKind,
        message: &str,
    ) -> ParseError {
        ParseError::new(
            kind,
            Some(self.current_span()),
            message,
        )
    }

    fn error_at(
        &self,
        kind: ParseErrorKind,
        token: Token<'src>,
        message: &str,
    ) -> ParseError {
        ParseError::new(
            kind,
            self.source_span(token.span()).ok(),
            message,
        )
    }

    // =========================================================================
    // Resource accounting
    // =========================================================================

    fn enter_nesting(
        &mut self,
        span: SourceSpan,
    ) -> ParserResult<()> {
        if self.nesting_depth
            >= self.config.limits.max_nesting_depth
        {
            return Err(ParseError::new(
                ParseErrorKind::NestingLimitExceeded,
                Some(span),
                "OpenQASM nesting depth exceeds configured limit",
            ));
        }

        self.nesting_depth += 1;

        Ok(())
    }

    fn leave_nesting(
        &mut self,
    ) {
        self.nesting_depth =
            self.nesting_depth.saturating_sub(1);
    }

    fn enter_expression(
        &mut self,
    ) -> ParserResult<()> {
        if self.expression_depth
            >= self.config.limits.max_expression_depth
        {
            return Err(ParseError::new(
                ParseErrorKind::ExpressionDepthExceeded,
                Some(self.current_span()),
                "expression depth exceeds configured limit",
            ));
        }

        self.expression_depth += 1;

        Ok(())
    }

    fn leave_expression(
        &mut self,
    ) {
        self.expression_depth =
            self.expression_depth.saturating_sub(1);
    }
}

// =============================================================================
// Literal helpers
// =============================================================================

fn parse_version_literal(
    value: &str,
) -> Option<(u16, u16)> {
    let value = value.replace('_', "");

    let mut pieces =
        value.split('.');

    let major =
        pieces.next()?.parse::<u16>().ok()?;

    let minor =
        pieces.next()?.parse::<u16>().ok()?;

    if pieces.next().is_some() {
        return None;
    }

    Some((major, minor))
}

fn integer_radix(
    value: &str,
) -> IntegerRadix {
    match value {
        value
            if value.starts_with("0x")
                || value.starts_with("0X") =>
        {
            IntegerRadix::Hexadecimal
        }

        value
            if value.starts_with("0b")
                || value.starts_with("0B") =>
        {
            IntegerRadix::Binary
        }

        value
            if value.starts_with("0o")
                || value.starts_with("0O") =>
        {
            IntegerRadix::Octal
        }

        _ => IntegerRadix::Decimal,
    }
}

fn normalize_integer_literal(
    value: &str,
) -> String {
    value.replace('_', "")
}

fn normalize_numeric_literal(
    value: &str,
) -> String {
    value.replace('_', "")
}

fn split_duration_literal(
    value: &str,
) -> Option<(&str, DurationUnit)> {
    let suffixes = [
        ("dt", DurationUnit::Cycles),
        ("ns", DurationUnit::Nanoseconds),
        ("us", DurationUnit::Microseconds),
        ("ms", DurationUnit::Milliseconds),
        ("s", DurationUnit::Seconds),
    ];

    for (suffix, unit) in suffixes {
        if let Some(number) =
            value.strip_suffix(suffix)
        {
            if !number.is_empty() {
                return Some((number, unit));
            }
        }
    }

    None
}

fn decode_string_literal(
    value: &str,
) -> Option<String> {
    if value.len() < 2
        || !value.starts_with('"')
        || !value.ends_with('"')
    {
        return None;
    }

    let body =
        &value[1..value.len() - 1];

    let mut result = String::with_capacity(
        body.len(),
    );

    let mut chars = body.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            result.push(ch);
            continue;
        }

        let escaped = chars.next()?;

        match escaped {
            'n' => result.push('\n'),
            'r' => result.push('\r'),
            't' => result.push('\t'),
            '\\' => result.push('\\'),
            '"' => result.push('"'),
            '\'' => result.push('\''),
            '0' => result.push('\0'),

            'x' => {
                let a = chars.next()?;
                let b = chars.next()?;

                let value = u8::from_str_radix(
                    &format!("{a}{b}"),
                    16,
                )
                .ok()?;

                result.push(
                    char::from(value),
                );
            }

            'u' => {
                if chars.next()? != '{' {
                    return None;
                }

                let mut digits =
                    String::new();

                loop {
                    let ch = chars.next()?;

                    if ch == '}' {
                        break;
                    }

                    if !ch.is_ascii_hexdigit()
                        || digits.len() >= 6
                    {
                        return None;
                    }

                    digits.push(ch);
                }

                let codepoint =
                    u32::from_str_radix(
                        &digits,
                        16,
                    )
                    .ok()?;

                result.push(
                    char::from_u32(
                        codepoint,
                    )?,
                );
            }

            _ => return None,
        }
    }

    Some(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> ParserConfig {
        ParserConfig {
            source_id: SourceId::from_raw(7),
            limits: ParserLimits::default(),
        }
    }

    #[test]
    fn parses_version() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.1;",
                config(),
            )
            .expect("version must parse");

        let version =
            program
                .version()
                .expect("version expected");

        assert_eq!(version.major(), 3);
        assert_eq!(version.minor(), 1);
    }

    #[test]
    fn parses_qubit_declaration() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit[4] q;",
                config(),
            )
            .expect("qubit declaration must parse");

        assert_eq!(
            program.statements().len(),
            1
        );

        match &program.statements()[0] {
            Statement::QuantumDeclaration(
                declaration,
            ) => {
                assert_eq!(
                    declaration.name().as_str(),
                    "q"
                );
            }

            _ => panic!(
                "expected quantum declaration"
            ),
        }
    }

    #[test]
    fn parses_parameterized_gate() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit[2] q; rx(pi/2) q[0];",
                config(),
            )
            .expect("gate must parse");

        let statement =
            program
                .statements()
                .iter()
                .find(|statement| {
                    matches!(
                        statement,
                        Statement::GateCall(_)
                    )
                })
                .expect("gate call expected");

        match statement {
            Statement::GateCall(call) => {
                assert_eq!(
                    call.name().as_str(),
                    "rx"
                );
                assert_eq!(
                    call.parameters().len(),
                    1
                );
                assert_eq!(
                    call.operands().len(),
                    1
                );
            }

            _ => unreachable!(),
        }
    }

    #[test]
    fn preserves_register_operands() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit[4] q; h q;",
                config(),
            )
            .expect("gate must parse");

        let statement =
            program
                .statements()
                .iter()
                .find(|statement| {
                    matches!(
                        statement,
                        Statement::GateCall(_)
                    )
                })
                .expect("gate call expected");

        match statement {
            Statement::GateCall(call) => {
                assert_eq!(
                    call.operands().len(),
                    1
                );

                match &call.operands()[0] {
                    GateOperand::Designator(
                        designator,
                    ) => {
                        assert_eq!(
                            designator
                                .name()
                                .as_str(),
                            "q"
                        );

                        assert!(
                            designator.index().is_none()
                        );
                    }

                    _ => panic!(
                        "expected logical designator"
                    ),
                }
            }

            _ => unreachable!(),
        }
    }

    #[test]
    fn preserves_measurement_mapping() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit[2] q; bit[2] c; measure q[1] -> c[0];",
                config(),
            )
            .expect("measurement must parse");

        let statement =
            program
                .statements()
                .iter()
                .find(|statement| {
                    matches!(
                        statement,
                        Statement::MeasureAssignment(_)
                    )
                })
                .expect(
                    "measurement assignment expected"
                );

        match statement {
            Statement::MeasureAssignment(
                measurement,
            ) => {
                assert_eq!(
                    measurement
                        .source()
                        .operand()
                        .name()
                        .as_str(),
                    "q"
                );

                assert_eq!(
                    measurement
                        .destination()
                        .name()
                        .as_str(),
                    "c"
                );
            }

            _ => unreachable!(),
        }
    }

    #[test]
    fn does_not_insert_measurements() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit[2] q; h q[0];",
                config(),
            )
            .expect("program must parse");

        assert_eq!(
            program
                .statements()
                .iter()
                .filter(|statement| {
                    matches!(
                        statement,
                        Statement::MeasureAssignment(_)
                    )
                })
                .count(),
            0
        );
    }

    #[test]
    fn parses_physical_qubits() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; h $17;",
                config(),
            )
            .expect("physical qubit must parse");

        let statement =
            program.statements()
                .iter()
                .find(|statement| {
                    matches!(
                        statement,
                        Statement::GateCall(_)
                    )
                })
                .expect("gate expected");

        match statement {
            Statement::GateCall(call) => {
                match &call.operands()[0] {
                    GateOperand::Physical(
                        qubit,
                    ) => {
                        assert_eq!(
                            qubit.index(),
                            17
                        );
                    }

                    _ => panic!(
                        "expected physical qubit"
                    ),
                }
            }

            _ => unreachable!(),
        }
    }

    #[test]
    fn parses_nested_if_scope() {
        let source = concat!(
            "OPENQASM 3.0;",
            "bit c;",
            "qubit q;",
            "if (c) {",
            "  reset q;",
            "}"
        );

        let program =
            OpenQasmParser::parse(
                source,
                config(),
            )
            .expect("if must parse");

        assert!(
            program.statements()
                .iter()
                .any(|statement| {
                    matches!(
                        statement,
                        Statement::If(_)
                    )
                })
        );
    }

    #[test]
    fn rejects_unterminated_block() {
        let source =
            "OPENQASM 3.0; if (true) { reset q;";

        let error =
            OpenQasmParser::parse(
                source,
                config(),
            )
            .expect_err(
                "unterminated block must fail"
            );

        assert_eq!(
            error.kind(),
            ParseErrorKind::UnexpectedEof
        );
    }

    #[test]
    fn rejects_invalid_version() {
        let error =
            OpenQasmParser::parse(
                "OPENQASM nope;",
                config(),
            )
            .expect_err(
                "invalid version must fail"
            );

        assert_eq!(
            error.kind(),
            ParseErrorKind::InvalidVersion
        );
    }

    #[test]
    fn string_decoding_is_deterministic() {
        assert_eq!(
            decode_string_literal(
                "\"hello\\\\nworld\""
            )
            .expect("string"),
            "hello\nworld"
        );
    }

    #[test]
    fn duration_suffixes_are_preserved() {
        let result =
            split_duration_literal(
                "10ns"
            )
            .expect("duration");

        assert_eq!(
            result.0,
            "10"
        );

        assert_eq!(
            result.1,
            DurationUnit::Nanoseconds
        );
    }

    #[test]
    fn parser_is_deterministic() {
        let source =
            "OPENQASM 3.0; qubit[2] q; h q[0]; cx q[0], q[1];";

        let a =
            OpenQasmParser::parse(
                source,
                config(),
            )
            .expect("first parse");

        let b =
            OpenQasmParser::parse(
                source,
                config(),
            )
            .expect("second parse");

        assert_eq!(
            a,
            b
        );
    }

    #[test]
    fn parser_does_not_construct_ir() {
        let program =
            OpenQasmParser::parse(
                "OPENQASM 3.0; qubit q; h q;",
                config(),
            )
            .expect("program");

        assert_eq!(
            program.statements().len(),
            2
        );
    }
}