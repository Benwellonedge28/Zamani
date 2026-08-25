//! Zamani Quantum Frontend — OpenQASM parser.
//!
//! Production syntax parser for OpenQASM 3.x.
//!
//! Architectural boundary:
//!
//! ```text
//! source
//!   │
//!   ▼
//! lexer.rs
//!   │
//!   ▼
//! Token<'src>
//!   │
//!   ▼
//! parser.rs
//!   │
//!   ▼
//! OpenQASM AST
//!   │
//!   ▼
//! validation.rs
//!   │
//!   ▼
//! importer/lowering
//!   │
//!   ▼
//! Quantum IR
//! ```
//!
//! This module performs syntax parsing only.
//!
//! It MUST NOT:
//!
//! - resolve symbols;
//! - perform semantic type checking;
//! - resolve includes;
//! - access the filesystem;
//! - access the network;
//! - execute `extern` declarations;
//! - execute calibration code;
//! - construct Quantum IR;
//! - optimize;
//! - route;
//! - schedule;
//! - map to hardware;
//! - execute a quantum program.
//!
//! The parser is deliberately format-local. The generic frontend contract
//! remains in `frontend/format.rs`, `frontend/importer.rs`,
//! `frontend/exporter.rs`, and `frontend/lowering.rs`.
//!
//! Rust:
//!
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no new dependencies
//!
//! OpenQASM authority:
//!
//! https://openqasm.com/versions/3.1/grammar/index.html
//!
//! Important:
//!
//! Syntactic acceptance is NOT semantic acceptance. The official OpenQASM
//! grammar explicitly leaves semantic analysis to compiler implementations.
//! Therefore this parser must preserve syntactically valid constructs for
//! `validation.rs` rather than rejecting them merely because they cannot yet
//! be lowered to Quantum IR.

use std::fmt;

use crate::quantum::frontend::core::source::{
    SourceId,
    SourceSpan,
};

use super::ast::{
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
    ConstDeclaration,
    ControlStatement,
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
    PhysicalQubit,
    PragmaStatement,
    Program,
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

/// Maximum recursion depth that the handwritten recursive parser is allowed
/// to approach.
///
/// This is deliberately lower than an arbitrary attacker-controlled value.
/// A parser limit is not useful if the Rust call stack overflows before the
/// parser's logical limit is reached.
const SAFE_MAX_RECURSION_DEPTH: usize = 256;

/// Parser resource limits.
///
/// These limits apply to parser-owned allocations and recursive structures.
/// The lexer must enforce its own lexical limits before this layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserLimits {
    /// Maximum number of tokens accepted.
    pub max_tokens: usize,

    /// Maximum statements in a single scope.
    pub max_statements_per_scope: usize,

    /// Maximum AST nodes created.
    pub max_ast_nodes: usize,

    /// Maximum syntactic nesting depth.
    pub max_nesting_depth: usize,

    /// Maximum expression nesting depth.
    pub max_expression_depth: usize,

    /// Maximum gate parameters.
    pub max_gate_parameters: usize,

    /// Maximum gate operands.
    pub max_gate_operands: usize,

    /// Maximum argument definitions.
    pub max_arguments: usize,

    /// Maximum switch cases.
    pub max_switch_cases: usize,

    /// Maximum decoded string size.
    pub max_decoded_string_bytes: usize,
}

impl Default for ParserLimits {
    fn default() -> Self {
        Self {
            max_tokens: 4_000_000,
            max_statements_per_scope: 1_000_000,
            max_ast_nodes: 4_000_000,
            max_nesting_depth: 256,
            max_expression_depth: 256,
            max_gate_parameters: 4096,
            max_gate_operands: 4096,
            max_arguments: 4096,
            max_switch_cases: 4096,
            max_decoded_string_bytes: 1_048_576,
        }
    }
}

impl ParserLimits {
    /// Validates parser configuration before parsing.
    ///
    /// This prevents callers from accidentally configuring a recursion limit
    /// beyond what the handwritten parser can safely enforce.
    pub fn validate(self) -> Result<Self, ParseError> {
        if self.max_tokens == 0 {
            return Err(ParseError::configuration(
                "max_tokens must be greater than zero",
            ));
        }

        if self.max_ast_nodes == 0 {
            return Err(ParseError::configuration(
                "max_ast_nodes must be greater than zero",
            ));
        }

        if self.max_statements_per_scope == 0 {
            return Err(ParseError::configuration(
                "max_statements_per_scope must be greater than zero",
            ));
        }

        if self.max_nesting_depth == 0
            || self.max_nesting_depth > SAFE_MAX_RECURSION_DEPTH
        {
            return Err(ParseError::configuration(
                "max_nesting_depth must be between 1 and 256",
            ));
        }

        if self.max_expression_depth == 0
            || self.max_expression_depth > SAFE_MAX_RECURSION_DEPTH
        {
            return Err(ParseError::configuration(
                "max_expression_depth must be between 1 and 256",
            ));
        }

        if self.max_gate_parameters == 0
            || self.max_gate_operands == 0
            || self.max_arguments == 0
            || self.max_switch_cases == 0
        {
            return Err(ParseError::configuration(
                "collection limits must be greater than zero",
            ));
        }

        Ok(self)
    }
}

/// Parser configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserConfig {
    /// Source document identity.
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

/// Parser result.
pub type ParserResult<T> = Result<T, ParseError>;

/// Stable parser error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseErrorKind {
    Configuration,
    TokenLimitExceeded,
    AstLimitExceeded,
    StatementLimitExceeded,
    NestingLimitExceeded,
    ExpressionDepthExceeded,
    UnexpectedToken,
    UnexpectedEof,
    InvalidVersion,
    ExpectedIdentifier,
    InvalidExpression,
    InvalidType,
    InvalidDesignator,
    InvalidOperand,
    InvalidAssignment,
    InvalidStatement,
    InvalidLiteral,
    InvalidSourceSpan,
    InvalidTokenStream,
    Lexer,
}

impl ParseErrorKind {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Configuration => "QASM-P000",
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
            Self::InvalidLiteral => "QASM-P016",
            Self::InvalidSourceSpan => "QASM-P017",
            Self::InvalidTokenStream => "QASM-P018",
            Self::Lexer => "QASM-P019",
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// Structured parser failure.
///
/// This remains independent of the generic frontend diagnostic renderer.
/// `core/diagnostics.rs` is responsible for presentation and aggregation.
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

    fn configuration(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::Configuration, None, message)
    }

    #[must_use]
    pub const fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => {
                write!(f, "{} at {}: {}", self.code(), span, self.message)
            }
            None => write!(f, "{}: {}", self.code(), self.message),
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

/// Production OpenQASM parser.
pub struct OpenQasmParser<'src> {
    tokens: Vec<Token<'src>>,
    config: ParserConfig,
    position: usize,

    /// Number of AST nodes charged to the current parse.
    ast_nodes: usize,

    /// Current grammar nesting.
    nesting_depth: usize,

    /// Current expression nesting.
    expression_depth: usize,
}

impl<'src> OpenQasmParser<'src> {
    /// Lex and parse a source document.
    pub fn parse(
        source: &'src str,
        config: ParserConfig,
    ) -> ParserResult<Program> {
        config.limits.validate()?;

        let lexer = OpenQasmLexer::new(source)?;
        let tokens = lexer.tokenize()?;

        Self::from_tokens(tokens, config)?.parse_program()
    }

    /// Construct a parser from an existing token stream.
    pub fn from_tokens(
        tokens: Vec<Token<'src>>,
        config: ParserConfig,
    ) -> ParserResult<Self> {
        config.limits.validate()?;

        if tokens.len() > config.limits.max_tokens {
            return Err(ParseError::new(
                ParseErrorKind::TokenLimitExceeded,
                None,
                "token stream exceeds parser token limit",
            ));
        }

        if tokens.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::InvalidTokenStream,
                None,
                "parser received an empty token stream",
            ));
        }

        if !tokens
            .last()
            .map(Token::is_eof)
            .unwrap_or(false)
        {
            return Err(ParseError::new(
                ParseErrorKind::InvalidTokenStream,
                None,
                "parser token stream must end with EOF",
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

    /// Parse the entire program.
    pub fn parse_program(mut self) -> ParserResult<Program> {
        let start = self.current_span();

        /*
         * The OpenQASM reference grammar permits:
         *
         *     version?
         *     statementOrScope*
         *     EOF
         *
         * Semantic validation is responsible for enforcing the version's
         * uniqueness and position rules.
         */
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

        self.charge_nodes(1, Some(start))?;

        Ok(Program::new(
            self.join_spans(start, end)?,
            version,
            statements,
        ))
    }

    // -------------------------------------------------------------------------
    // Version
    // -------------------------------------------------------------------------

    fn parse_optional_version(
        &mut self,
    ) -> ParserResult<Option<VersionDeclaration>> {
        if !self.at(TokenKind::KwOpenQasm) {
            return Ok(None);
        }

        let start = self.bump()?;
        let version = self.current();

        /*
         * The lexer should normally provide a dedicated VersionSpecifier
         * token. The parser deliberately validates the lexical payload too,
         * because this protects the parser when supplied with externally
         * constructed token streams.
         */
        let (major, minor) =
            parse_version_literal(version.lexeme())
                .ok_or_else(|| {
                    self.error_at(
                        ParseErrorKind::InvalidVersion,
                        version,
                        "invalid OpenQASM version specifier",
                    )
                })?;

        let version_end = self.bump()?;

        self.expect(
            TokenKind::Semicolon,
            "expected `;` after OPENQASM version",
        )?;

        let span = self.join_spans(
            start.span(),
            version_end.span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Some(VersionDeclaration::new(
            self.source_span(start.span())?,
            major,
            minor,
        )))
    }

    // -------------------------------------------------------------------------
    // Statements
    // -------------------------------------------------------------------------

    fn parse_statement(&mut self) -> ParserResult<Statement> {
        let statement = match self.current().kind() {
            TokenKind::KwInclude => self.parse_include(),

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
            | TokenKind::KwReadonly
            | TokenKind::KwMutable => {
                self.parse_classical_or_io_declaration()
            }

            TokenKind::KwGate => self.parse_gate_definition(),

            TokenKind::KwDef => self.parse_def_definition(),

            TokenKind::KwExtern => self.parse_extern_declaration(),

            TokenKind::KwMeasure => self.parse_measure_statement(),

            TokenKind::KwReset => self.parse_reset(),

            TokenKind::KwBarrier => self.parse_barrier(),

            TokenKind::KwDelay => self.parse_delay(),

            TokenKind::KwBox => self.parse_box(),

            TokenKind::KwIf => self.parse_if(),

            TokenKind::KwFor => self.parse_for(),

            TokenKind::KwWhile => self.parse_while(),

            TokenKind::KwSwitch => self.parse_switch(),

            TokenKind::KwReturn => self.parse_return(),

            TokenKind::KwBreak => self.parse_break(),

            TokenKind::KwContinue => self.parse_continue(),

            TokenKind::KwEnd => self.parse_end(),

            TokenKind::KwLet => self.parse_alias(),

            TokenKind::KwCal => self.parse_cal(),

            TokenKind::KwDefcal => self.parse_defcal(),

            TokenKind::At => self.parse_annotated_statement(),

            TokenKind::Hash => self.parse_pragma(),

            TokenKind::Identifier
            | TokenKind::HardwareQubit
            | TokenKind::KwGphase
            | TokenKind::KwMeasure => {
                self.parse_identifier_leading_statement()
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidStatement,
                "token cannot begin an OpenQASM statement",
            )),
        }?;

        Ok(statement)
    }

    fn parse_break(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after break",
        )?;

        self.charge_nodes(1, Some(self.source_span(start.span())?))?;

        Ok(Statement::Break(ControlStatement::new(
            self.join_spans(start.span(), end.span())?,
        )))
    }

    fn parse_continue(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after continue",
        )?;

        self.charge_nodes(1, Some(self.source_span(start.span())?))?;

        Ok(Statement::Continue(ControlStatement::new(
            self.join_spans(start.span(), end.span())?,
        )))
    }

    fn parse_end(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after end",
        )?;

        self.charge_nodes(1, Some(self.source_span(start.span())?))?;

        Ok(Statement::End(ControlStatement::new(
            self.join_spans(start.span(), end.span())?,
        )))
    }

    // -------------------------------------------------------------------------
    // Include
    // -------------------------------------------------------------------------

    fn parse_include(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let path = self.expect(
            TokenKind::StringLiteral,
            "expected include string literal",
        )?;

        let decoded = decode_string_literal(
            path.lexeme(),
            self.config.limits.max_decoded_string_bytes,
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::InvalidLiteral,
                path,
                "invalid include string literal",
            )
        })?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after include",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Include(IncludeStatement::new(
            span,
            decoded,
        )))
    }

    fn parse_calibration_grammar(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let grammar = self.expect(
            TokenKind::StringLiteral,
            "expected calibration grammar string",
        )?;

        let decoded = decode_string_literal(
            grammar.lexeme(),
            self.config.limits.max_decoded_string_bytes,
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::InvalidLiteral,
                grammar,
                "invalid calibration grammar string",
            )
        })?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after defcalgrammar",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::CalibrationGrammar(
            CalibrationGrammarStatement::new(span, decoded),
        ))
    }

    // -------------------------------------------------------------------------
    // Quantum declarations
    // -------------------------------------------------------------------------

    fn parse_quantum_declaration(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current().span();

        let quantum_type = self.parse_quantum_type()?;
        let name = self.parse_identifier()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after quantum declaration",
        )?;

        let span = self.join_spans(start, end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::QuantumDeclaration(
            QuantumDeclaration::new(
                span,
                quantum_type,
                name,
            ),
        ))
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

    // -------------------------------------------------------------------------
    // Classical declarations
    // -------------------------------------------------------------------------

    fn parse_classical_or_io_declaration(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current().span();

        let qualifier = self.parse_type_qualifier();

        let scalar_type = self.parse_scalar_type()?;
        let name = self.parse_identifier()?;

        let initializer = if self.consume(TokenKind::Equal)? {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after declaration",
        )?;

        let span = self.join_spans(start, end.span())?;

        self.charge_nodes(1, Some(span))?;

        match qualifier {
            Some(TypeQualifier::Input) |
            Some(TypeQualifier::Output) => {
                if initializer.is_some() {
                    return Err(self.error(
                        ParseErrorKind::InvalidStatement,
                        "input/output declarations cannot have initializers",
                    ));
                }

                Ok(Statement::IoDeclaration(
                    IoDeclaration::new(
                        span,
                        qualifier.unwrap_or(TypeQualifier::Input),
                        scalar_type,
                        name,
                        None,
                    ),
                ))
            }

            Some(TypeQualifier::Const) => {
                let initializer = initializer.ok_or_else(|| {
                    self.error(
                        ParseErrorKind::InvalidStatement,
                        "const declarations require an initializer",
                    )
                })?;

                Ok(Statement::ConstDeclaration(
                    ConstDeclaration::new(
                        span,
                        scalar_type,
                        name,
                        initializer,
                    ),
                ))
            }

            _ => Ok(Statement::ClassicalDeclaration(
                ClassicalDeclaration::new(
                    span,
                    qualifier,
                    scalar_type,
                    name,
                    initializer,
                ),
            )),
        }
    }

    fn parse_type_qualifier(
        &mut self,
    ) -> Option<TypeQualifier> {
        match self.current().kind() {
            TokenKind::KwConst => {
                let _ = self.bump();
                Some(TypeQualifier::Const)
            }

            TokenKind::KwInput => {
                let _ = self.bump();
                Some(TypeQualifier::Input)
            }

            TokenKind::KwOutput => {
                let _ = self.bump();
                Some(TypeQualifier::Output)
            }

            TokenKind::KwReadonly => {
                let _ = self.bump();
                Some(TypeQualifier::Readonly)
            }

            TokenKind::KwMutable => {
                let _ = self.bump();
                Some(TypeQualifier::Mutable)
            }

            _ => None,
        }
    }

    fn parse_scalar_type(
        &mut self,
    ) -> ParserResult<ScalarType> {
        match self.current().kind() {
            TokenKind::KwBool => {
                self.bump()?;
                Ok(ScalarType::Bool)
            }

            TokenKind::KwBit => {
                self.bump()?;
                Ok(ScalarType::Bit(self.parse_optional_size()?))
            }

            TokenKind::KwInt => {
                self.bump()?;
                Ok(ScalarType::Int(self.parse_optional_size()?))
            }

            TokenKind::KwUInt => {
                self.bump()?;
                Ok(ScalarType::UInt(self.parse_optional_size()?))
            }

            TokenKind::KwFloat => {
                self.bump()?;
                Ok(ScalarType::Float(self.parse_optional_size()?))
            }

            TokenKind::KwAngle => {
                self.bump()?;
                Ok(ScalarType::Angle(self.parse_optional_size()?))
            }

            TokenKind::KwComplex => {
                self.bump()?;
                Ok(ScalarType::Complex(self.parse_optional_size()?))
            }

            TokenKind::KwDuration => {
                self.bump()?;
                Ok(ScalarType::Duration)
            }

            TokenKind::KwStretch => {
                self.bump()?;
                Ok(ScalarType::Stretch)
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidType,
                "expected OpenQASM scalar type",
            )),
        }
    }

    fn parse_optional_size(
        &mut self,
    ) -> ParserResult<Option<Expression>> {
        if !self.consume(TokenKind::LBracket)? {
            return Ok(None);
        }

        let expression = self.parse_expression()?;

        self.expect(
            TokenKind::RBracket,
            "expected `]` after type designator",
        )?;

        Ok(Some(expression))
    }

    // -------------------------------------------------------------------------
    // Gate definitions
    // -------------------------------------------------------------------------

    fn parse_gate_definition(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;
        let name = self.parse_identifier()?;

        let parameters =
            if self.consume(TokenKind::LParen)? {
                self.parse_identifier_list(TokenKind::RParen)?
            } else {
                Vec::new()
            };

        let qubits =
            self.parse_identifier_list(TokenKind::LBrace)?;

        self.enter_nesting(start.span())?;

        self.expect(
            TokenKind::LBrace,
            "expected `{` before gate body",
        )?;

        let body = self.parse_statement_block()?;

        self.expect_closing_nesting();

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::GateDefinition(
            GateDefinition::new(
                span,
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
        let mut values = Vec::new();

        if self.at(terminator) {
            self.bump()?;
            return Ok(values);
        }

        loop {
            if values.len()
                >= self.config.limits.max_arguments
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "identifier list exceeds configured limit",
                ));
            }

            values.push(self.parse_identifier()?);

            if !self.consume(TokenKind::Comma)? {
                break;
            }

            // OpenQASM allows trailing commas in list productions.
            if self.at(terminator) {
                break;
            }
        }

        self.expect(
            terminator,
            "expected list terminator",
        )?;

        Ok(values)
    }

    // -------------------------------------------------------------------------
    // Subroutines
    // -------------------------------------------------------------------------

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
                let type_start = self.current_span();
                let ty = self.parse_scalar_type()?;

                Some(ReturnSignature::new(
                    type_start,
                    ty,
                ))
            } else {
                None
            };

        self.enter_nesting(start.span())?;

        let opening =
            self.expect(
                TokenKind::LBrace,
                "expected `{` before subroutine body",
            )?;

        let body_statements =
            self.parse_statement_block()?;

        self.expect_closing_nesting();

        let body = Scope::new(
            self.join_spans(
                opening.span(),
                self.previous_span(),
            )?,
            body_statements,
        );

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

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
        let mut values = Vec::new();

        if self.consume(TokenKind::RParen)? {
            return Ok(values);
        }

        loop {
            if values.len()
                >= self.config.limits.max_arguments
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many subroutine arguments",
                ));
            }

            let start = self.current_span();

            let qualifier = self.parse_type_qualifier();
            let ty = self.parse_type_specifier()?;
            let name = self.parse_identifier()?;

            values.push(
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

            if self.at(TokenKind::RParen) {
                break;
            }
        }

        self.expect(
            TokenKind::RParen,
            "expected `)` after argument list",
        )?;

        Ok(values)
    }

    fn parse_type_specifier(
        &mut self,
    ) -> ParserResult<TypeSpecifier> {
        match self.current().kind() {
            TokenKind::KwQubit
            | TokenKind::KwQreg => {
                Ok(TypeSpecifier::Quantum(
                    self.parse_quantum_type()?,
                ))
            }

            _ => Ok(TypeSpecifier::Classical(
                self.parse_scalar_type()?,
            )),
        }
    }

    // -------------------------------------------------------------------------
    // Extern
    // -------------------------------------------------------------------------

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

                let arg_start = self.current_span();
                let ty = self.parse_type_specifier()?;

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

                if self.at(TokenKind::RParen) {
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
                let type_start = self.current_span();
                let ty = self.parse_scalar_type()?;

                Some(ReturnSignature::new(
                    type_start,
                    ty,
                ))
            } else {
                None
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after extern declaration",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::ExternDeclaration(
            ExternDeclaration::new(
                span,
                name,
                arguments,
                return_type,
            ),
        ))
    }

    // -------------------------------------------------------------------------
    // Quantum operations
    // -------------------------------------------------------------------------

    fn parse_measure_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operand = self.parse_designator()?;

        let source = MeasureExpression::new(
            self.join_spans(
                start.span(),
                operand.span(),
            )?,
            operand,
        );

        let destination =
            if self.consume(TokenKind::Arrow)? {
                Some(self.parse_designator()?)
            } else {
                None
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after measurement",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        match destination {
            Some(destination) => {
                Ok(Statement::MeasureAssignment(
                    MeasureAssignmentStatement::new(
                        span,
                        source,
                        destination,
                    ),
                ))
            }

            None => {
                Ok(Statement::Expression(
                    ExpressionStatement::new(
                        span,
                        Expression::Measure(source),
                    ),
                ))
            }
        }
    }

    fn parse_reset(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operand = self.parse_gate_operand()?;

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after reset",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Reset(ResetStatement::new(
            span,
            vec![operand],
        )))
    }

    fn parse_barrier(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let operands =
            if self.is_operand_start() {
                self.parse_operand_list()?
            } else {
                Vec::new()
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after barrier",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Barrier(
            BarrierStatement::new(span, operands),
        ))
    }

    fn parse_delay(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let duration = self.parse_designator()?;

        let operands =
            if self.is_operand_start() {
                self.parse_operand_list()?
            } else {
                Vec::new()
            };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after delay",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Delay(DelayStatement::new(
            span,
            Expression::Designator(duration),
            operands,
        )))
    }

    fn parse_box(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.bump()?;

        let designator =
            if self.at(TokenKind::LBracket) {
                Some(self.parse_designator_expression()?)
            } else {
                None
            };

        self.enter_nesting(start.span())?;

        self.expect(
            TokenKind::LBrace,
            "expected `{` after box",
        )?;

        let body = self.parse_statement_block()?;

        self.expect_closing_nesting();

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Box(BoxStatement::new(
            span,
            designator,
            body,
        )))
    }

    // -------------------------------------------------------------------------
    // Control flow
    // -------------------------------------------------------------------------

    fn parse_if(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after if",
        )?;

        let condition = self.parse_expression()?;

        self.expect(
            TokenKind::RParen,
            "expected `)` after if condition",
        )?;

        self.enter_nesting(start.span())?;

        let then_body =
            self.parse_statement_or_scope()?;

        let else_body =
            if self.consume(TokenKind::KwElse)? {
                Some(self.parse_statement_or_scope()?)
            } else {
                None
            };

        self.expect_closing_nesting();

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::If(IfStatement::new(
            span,
            condition,
            then_body,
            else_body,
        )))
    }

    fn parse_for(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let variable_type = self.parse_scalar_type()?;
        let variable = self.parse_identifier()?;

        self.expect(
            TokenKind::KwIn,
            "expected `in` in for statement",
        )?;

        let iterable = self.parse_for_iterable()?;

        self.enter_nesting(start.span())?;

        let body = self.parse_statement_or_scope()?;

        self.expect_closing_nesting();

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::For(ForStatement::new(
            span,
            variable_type,
            variable,
            iterable,
            body,
        )))
    }

    fn parse_for_iterable(
        &mut self,
    ) -> ParserResult<ForIterable> {
        let first = self.parse_expression()?;

        if !self.consume(TokenKind::Colon)? {
            return Ok(ForIterable::Expression(first));
        }

        let second = self.parse_expression()?;

        if self.consume(TokenKind::Colon)? {
            let third = self.parse_expression()?;

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

    fn parse_while(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after while",
        )?;

        let condition = self.parse_expression()?;

        self.expect(
            TokenKind::RParen,
            "expected `)` after while condition",
        )?;

        self.enter_nesting(start.span())?;

        let body = self.parse_statement_or_scope()?;

        self.expect_closing_nesting();

        let span = self.join_spans(
            start.span(),
            self.previous_span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::While(WhileStatement::new(
            span,
            condition,
            body,
        )))
    }

    fn parse_switch(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        self.expect(
            TokenKind::LParen,
            "expected `(` after switch",
        )?;

        let expression = self.parse_expression()?;

        self.expect(
            TokenKind::RParen,
            "expected `)` after switch expression",
        )?;

        self.enter_nesting(start.span())?;

        self.expect(
            TokenKind::LBrace,
            "expected `{` before switch cases",
        )?;

        let mut cases = Vec::new();

        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                self.expect_closing_nesting();
                return Err(self.error(
                    ParseErrorKind::UnexpectedEof,
                    "unterminated switch statement",
                ));
            }

            if cases.len()
                >= self.config.limits.max_switch_cases
            {
                self.expect_closing_nesting();
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "switch contains too many cases",
                ));
            }

            let case = if self.consume(TokenKind::KwCase)? {
                let expressions =
                    self.parse_expression_list_until_scope()?;

                self.enter_nesting(start.span())?;

                let opening = self.expect(
                    TokenKind::LBrace,
                    "expected `{` after switch case",
                )?;

                let body =
                    self.parse_statement_block()?;

                self.expect_closing_nesting();

                SwitchCase::Case {
                    span: self.join_spans(
                        opening.span(),
                        self.previous_span(),
                    )?,
                    expressions,
                    body,
                }
            } else if self.consume(TokenKind::KwDefault)? {
                self.enter_nesting(start.span())?;

                let opening = self.expect(
                    TokenKind::LBrace,
                    "expected `{` after default",
                )?;

                let body =
                    self.parse_statement_block()?;

                self.expect_closing_nesting();

                SwitchCase::Default {
                    span: self.join_spans(
                        opening.span(),
                        self.previous_span(),
                    )?,
                    body,
                }
            } else {
                self.expect_closing_nesting();
                return Err(self.error(
                    ParseErrorKind::UnexpectedToken,
                    "expected `case` or `default` in switch",
                ));
            };

            cases.push(case);
        }

        let end = self.expect(
            TokenKind::RBrace,
            "expected `}` after switch cases",
        )?;

        self.expect_closing_nesting();

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Switch(SwitchStatement::new(
            span,
            expression,
            cases,
        )))
    }

    fn parse_return(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let value = if self.at(TokenKind::Semicolon) {
            None
        } else if self.at(TokenKind::KwMeasure) {
            let measure = self.parse_measure_expression()?;
            Some(ReturnValue::Measure(measure))
        } else {
            Some(ReturnValue::Expression(
                self.parse_expression()?,
            ))
        };

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after return",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Return(
            ReturnStatement::new(span, value),
        ))
    }

    fn parse_statement_or_scope(
        &mut self,
    ) -> ParserResult<StatementOrScope> {
        if self.at(TokenKind::LBrace) {
            let start = self.bump()?;

            self.enter_nesting(start.span())?;

            let body = self.parse_statement_block()?;

            self.expect_closing_nesting();

            let span = self.join_spans(
                start.span(),
                self.previous_span(),
            )?;

            self.charge_nodes(1, Some(span))?;

            Ok(StatementOrScope::Scope(
                Scope::new(span, body),
            ))
        } else {
            Ok(StatementOrScope::Statement(
                Box::new(self.parse_statement()?),
            ))
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
                    "unterminated `{ ... }` scope",
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

            statements.push(self.parse_statement()?);
        }

        self.expect(
            TokenKind::RBrace,
            "expected `}`",
        )?;

        Ok(statements)
    }

    // -------------------------------------------------------------------------
    // Annotation / pragma
    // -------------------------------------------------------------------------

    fn parse_annotated_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current_span();
        let mut annotations = Vec::new();

        /*
         * The lexer already recognizes the annotation keyword as one lexical
         * construct. Do not treat arbitrary `@` occurrences as annotations.
         */
        while self.at(TokenKind::At) {
            let at = self.bump()?;
            let keyword = self.parse_identifier()?;

            let payload =
                if self.at(TokenKind::Identifier)
                    || self.at(TokenKind::StringLiteral)
                {
                    Some(self.bump()?.lexeme().to_owned())
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

        let statement = self.parse_statement()?;

        let span = self.join_spans(
            start,
            statement.span(),
        )?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Annotated(
            AnnotatedStatement::new(
                span,
                annotations,
                statement,
            ),
        ))
    }

    fn parse_pragma(&mut self) -> ParserResult<Statement> {
        let start = self.bump()?;

        let mut payload = String::new();

        while !self.at(TokenKind::Semicolon)
            && !self.at(TokenKind::Eof)
        {
            let fragment = self.bump()?.lexeme();

            if !payload.is_empty() {
                payload.push(' ');
            }

            payload.push_str(fragment);
        }

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after pragma",
        )?;

        let span = self.join_spans(start.span(), end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Pragma(
            PragmaStatement::new(span, payload),
        ))
    }

    // -------------------------------------------------------------------------
    // Gate calls / assignment / expressions
    // -------------------------------------------------------------------------

    fn parse_identifier_leading_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let save = self.position;

        let modifiers = self.parse_gate_modifiers()?;

        /*
         * gphase is a gate keyword, not an ordinary Identifier.
         */
        if self.at(TokenKind::KwGphase) {
            let start = self.current_span();
            let name = self.parse_identifier_from_keyword()?;

            let parameters =
                self.parse_optional_expression_list()?;

            let operands =
                if self.is_operand_start() {
                    self.parse_operand_list()?
                } else {
                    Vec::new()
                };

            let end = self.expect(
                TokenKind::Semicolon,
                "expected `;` after gphase",
            )?;

            let span = self.join_spans(
                start,
                end.span(),
            )?;

            self.charge_nodes(1, Some(span))?;

            return Ok(Statement::GateCall(
                GateCall::new(
                    span,
                    modifiers,
                    name,
                    parameters,
                    operands,
                ),
            ));
        }

        if !self.at(TokenKind::Identifier) {
            self.position = save;

            return self.parse_assignment_or_expression_statement();
        }

        let name = self.parse_identifier()?;

        /*
         * A gate call is syntactically distinguished from an expression
         * statement by the presence of a gate operand.
         */
        let parameters =
            self.parse_optional_expression_list()?;

        if self.is_operand_start() {
            let operands = self.parse_operand_list()?;

            let end = self.expect(
                TokenKind::Semicolon,
                "expected `;` after gate invocation",
            )?;

            let span = self.join_spans(
                self.tokens[save].span(),
                end.span(),
            )?;

            self.charge_nodes(1, Some(span))?;

            return Ok(Statement::GateCall(
                GateCall::new(
                    span,
                    modifiers,
                    name,
                    parameters,
                    operands,
                ),
            ));
        }

        /*
         * No operand means this was not a gate invocation. Restore the token
         * cursor and parse it using the expression grammar.
         */
        self.position = save;

        self.parse_assignment_or_expression_statement()
    }

    fn parse_gate_modifiers(
        &mut self,
    ) -> ParserResult<Vec<GateModifier>> {
        let mut modifiers = Vec::new();

        loop {
            let modifier = match self.current().kind() {
                TokenKind::KwInv => {
                    self.bump()?;
                    GateModifier::Inv
                }

                TokenKind::KwPow => {
                    self.bump()?;

                    self.expect(
                        TokenKind::LParen,
                        "expected `(` after pow",
                    )?;

                    let exponent =
                        self.parse_expression()?;

                    self.expect(
                        TokenKind::RParen,
                        "expected `)` after pow expression",
                    )?;

                    GateModifier::Pow(exponent)
                }

                TokenKind::KwCtrl
                | TokenKind::KwNegctrl => {
                    let negative =
                        self.at(TokenKind::KwNegctrl);

                    self.bump()?;

                    let count =
                        if self.consume(TokenKind::LParen)? {
                            let expression =
                                self.parse_expression()?;

                            self.expect(
                                TokenKind::RParen,
                                "expected `)` after control count",
                            )?;

                            Some(expression)
                        } else {
                            None
                        };

                    if negative {
                        GateModifier::NegCtrl(count)
                    } else {
                        GateModifier::Ctrl(count)
                    }
                }

                _ => break,
            };

            self.expect(
                TokenKind::At,
                "expected `@` after gate modifier",
            )?;

            modifiers.push(modifier);

            if modifiers.len()
                >= self.config.limits.max_gate_parameters
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many gate modifiers",
                ));
            }
        }

        Ok(modifiers)
    }

    fn parse_assignment_or_expression_statement(
        &mut self,
    ) -> ParserResult<Statement> {
        let start = self.current_span();

        let expression = self.parse_expression()?;

        if let Expression::Designator(target) = &expression {
            if let Some(operator) = self.assignment_operator() {
                self.bump()?;

                let value =
                    if self.at(TokenKind::KwMeasure) {
                        AssignmentValue::Measure(
                            self.parse_measure_expression()?,
                        )
                    } else {
                        AssignmentValue::Expression(
                            self.parse_expression()?,
                        )
                    };

                let end = self.expect(
                    TokenKind::Semicolon,
                    "expected `;` after assignment",
                )?;

                let span =
                    self.join_spans(start, end.span())?;

                self.charge_nodes(1, Some(span))?;

                return Ok(Statement::Assignment(
                    AssignmentStatement::new(
                        span,
                        target.clone(),
                        operator,
                        value,
                    ),
                ));
            }
        }

        let end = self.expect(
            TokenKind::Semicolon,
            "expected `;` after expression",
        )?;

        let span = self.join_spans(start, end.span())?;

        self.charge_nodes(1, Some(span))?;

        Ok(Statement::Expression(
            ExpressionStatement::new(
                span,
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

            TokenKind::BitAndEqual =>
                Some(AssignmentOperator::BitAndAssign),

            TokenKind::BitOrEqual =>
                Some(AssignmentOperator::BitOrAssign),

            TokenKind::BitXorEqual =>
                Some(AssignmentOperator::BitXorAssign),

            TokenKind::ShiftLeftEqual =>
                Some(AssignmentOperator::ShiftLeftAssign),

            TokenKind::ShiftRightEqual =>
                Some(AssignmentOperator::ShiftRightAssign),

            TokenKind::PowerEqual =>
                Some(AssignmentOperator::PowerAssign),

            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Operands
    // -------------------------------------------------------------------------

    fn is_operand_start(&self) -> bool {
        matches!(
            self.current().kind(),
            TokenKind::Identifier
                | TokenKind::HardwareQubit
        )
    }

    fn parse_operand_list(
        &mut self,
    ) -> ParserResult<Vec<GateOperand>> {
        let mut operands = Vec::new();

        if !self.is_operand_start() {
            return Ok(operands);
        }

        loop {
            if operands.len()
                >= self.config.limits.max_gate_operands
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "too many gate operands",
                ));
            }

            operands.push(self.parse_gate_operand()?);

            if !self.consume(TokenKind::Comma)? {
                break;
            }

            if !self.is_operand_start() {
                return Err(self.error(
                    ParseErrorKind::InvalidOperand,
                    "expected gate operand after comma",
                ));
            }
        }

        Ok(operands)
    }

    fn parse_gate_operand(
        &mut self,
    ) -> ParserResult<GateOperand> {
        if self.at(TokenKind::HardwareQubit) {
            let token = self.bump()?;

            let raw = token.lexeme();

            let index = raw
                .strip_prefix('$')
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(|| {
                    self.error_at(
                        ParseErrorKind::InvalidOperand,
                        token,
                        "invalid physical qubit",
                    )
                })?;

            return Ok(GateOperand::Physical(
                PhysicalQubit::new(
                    self.source_span(token.span())?,
                    index,
                ),
            ));
        }

        Ok(GateOperand::Designator(
            self.parse_designator()?,
        ))
    }

    fn parse_designator(
        &mut self,
    ) -> ParserResult<Designator> {
        let name = self.parse_identifier()?;

        let index =
            if self.at(TokenKind::LBracket) {
                Some(self.parse_designator_expression()?)
            } else {
                None
            };

        let span = match &index {
            Some(index) => self.join_spans(
                name.span(),
                self.previous_span(),
            )?,
            None => name.span(),
        };

        Ok(Designator::new(
            span,
            name,
            index.map(IndexExpression::Index),
        ))
    }

    fn parse_designator_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        self.expect(
            TokenKind::LBracket,
            "expected `[`",
        )?;

        let expression = self.parse_expression()?;

        self.expect(
            TokenKind::RBracket,
            "expected `]`",
        )?;

        Ok(expression)
    }

    // -------------------------------------------------------------------------
    // Expressions
    // -------------------------------------------------------------------------

    fn parse_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        self.parse_binary_expression(1)
    }

    fn parse_binary_expression(
        &mut self,
        minimum_precedence: u8,
    ) -> ParserResult<Expression> {
        self.enter_expression()?;

        let mut left = self.parse_unary_expression()?;

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
                if matches!(operator, BinaryOperator::Power) {
                    precedence
                } else {
                    precedence + 1
                };

            let right =
                self.parse_binary_expression(next_minimum)?;

            let span = self.join_spans(
                left.span(),
                right.span(),
            )?;

            self.charge_nodes(1, Some(span))?;

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
        match self.current().kind() {
            TokenKind::LogicalOr =>
                Some((BinaryOperator::LogicalOr, 1)),

            TokenKind::LogicalAnd =>
                Some((BinaryOperator::LogicalAnd, 2)),

            TokenKind::BitOr =>
                Some((BinaryOperator::BitOr, 3)),

            TokenKind::BitXor =>
                Some((BinaryOperator::BitXor, 4)),

            TokenKind::BitAnd =>
                Some((BinaryOperator::BitAnd, 5)),

            TokenKind::EqualEqual =>
                Some((BinaryOperator::Equal, 6)),

            TokenKind::NotEqual =>
                Some((BinaryOperator::NotEqual, 6)),

            TokenKind::Less =>
                Some((BinaryOperator::Less, 7)),

            TokenKind::LessEqual =>
                Some((BinaryOperator::LessEqual, 7)),

            TokenKind::Greater =>
                Some((BinaryOperator::Greater, 7)),

            TokenKind::GreaterEqual =>
                Some((BinaryOperator::GreaterEqual, 7)),

            TokenKind::ShiftLeft =>
                Some((BinaryOperator::ShiftLeft, 8)),

            TokenKind::ShiftRight =>
                Some((BinaryOperator::ShiftRight, 8)),

            TokenKind::Plus =>
                Some((BinaryOperator::Add, 9)),

            TokenKind::Minus =>
                Some((BinaryOperator::Subtract, 9)),

            TokenKind::Star =>
                Some((BinaryOperator::Multiply, 10)),

            TokenKind::Slash =>
                Some((BinaryOperator::Divide, 10)),

            TokenKind::Percent =>
                Some((BinaryOperator::Remainder, 10)),

            TokenKind::Power =>
                Some((BinaryOperator::Power, 11)),

            _ => None,
        }
    }

    fn parse_unary_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        let operator = match self.current().kind() {
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
            let operand = self.parse_unary_expression()?;

            let span = self.join_spans(
                start.span(),
                operand.span(),
            )?;

            self.charge_nodes(1, Some(span))?;

            return Ok(Expression::Unary {
                node: AstNode::new(span),
                operator,
                operand: Box::new(operand),
            });
        }

        self.parse_primary_expression()
    }

    fn parse_primary_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        match self.current().kind() {
            TokenKind::IntegerLiteral => {
                let token = self.bump()?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::IntegerLiteral {
                    node: AstNode::new(span),
                    value: IntegerLiteral::new(
                        normalize_integer_literal(
                            token.lexeme(),
                        ),
                        integer_radix(
                            token.lexeme(),
                        ),
                    ),
                })
            }

            TokenKind::FloatLiteral => {
                let token = self.bump()?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::FloatLiteral {
                    node: AstNode::new(span),
                    value: super::ast::FloatLiteral::new(
                        normalize_numeric_literal(
                            token.lexeme(),
                        ),
                    ),
                })
            }

            TokenKind::ImaginaryLiteral => {
                let token = self.bump()?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::ImaginaryLiteral {
                    node: AstNode::new(span),
                    value: normalize_numeric_literal(
                        token.lexeme(),
                    ),
                })
            }

            TokenKind::BitstringLiteral => {
                let token = self.bump()?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::BitstringLiteral {
                    node: AstNode::new(span),
                    value: token.lexeme().to_owned(),
                })
            }

            TokenKind::DurationLiteral => {
                self.parse_duration_expression()
            }

            TokenKind::BooleanLiteral
            | TokenKind::KwTrue
            | TokenKind::KwFalse => {
                let token = self.bump()?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::BoolLiteral {
                    node: AstNode::new(span),
                    value: matches!(
                        token.lexeme(),
                        "true"
                    ),
                })
            }

            TokenKind::HardwareQubit => {
                let token = self.bump()?;

                let index = token
                    .lexeme()
                    .strip_prefix('$')
                    .and_then(|value| value.parse::<u64>().ok())
                    .ok_or_else(|| {
                        self.error_at(
                            ParseErrorKind::InvalidLiteral,
                            token,
                            "invalid hardware qubit literal",
                        )
                    })?;

                let span =
                    self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::HardwareQubit(
                    PhysicalQubit::new(span, index),
                ))
            }

            TokenKind::Identifier => {
                self.parse_identifier_expression()
            }

            TokenKind::KwPi => {
                let token = self.bump()?;
                let span = self.source_span(token.span())?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::Identifier(
                    Identifier::new(
                        span,
                        token.lexeme().to_owned(),
                    )
                    .ok_or_else(|| {
                        self.error_at(
                            ParseErrorKind::ExpectedIdentifier,
                            token,
                            "invalid pi identifier",
                        )
                    })?,
                ))
            }

            TokenKind::LParen => {
                let start = self.bump()?;
                let expression = self.parse_expression()?;

                let end = self.expect(
                    TokenKind::RParen,
                    "expected `)`",
                )?;

                let span = self.join_spans(
                    start.span(),
                    end.span(),
                )?;

                self.charge_nodes(1, Some(span))?;

                Ok(Expression::Parenthesized {
                    node: AstNode::new(span),
                    expression: Box::new(expression),
                })
            }

            TokenKind::KwMeasure => {
                let measure = self.parse_measure_expression()?;

                Ok(Expression::Measure(measure))
            }

            _ => Err(self.error(
                ParseErrorKind::InvalidExpression,
                "expected OpenQASM expression",
            )),
        }
    }

    fn parse_identifier_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        let designator = self.parse_designator()?;

        if self.consume(TokenKind::LParen)? {
            let arguments =
                self.parse_expression_list(TokenKind::RParen)?;

            let span = self.join_spans(
                designator.span(),
                self.previous_span(),
            )?;

            self.charge_nodes(1, Some(span))?;

            Ok(Expression::FunctionCall {
                node: AstNode::new(span),
                name: designator.name().clone(),
                arguments,
            })
        } else {
            Ok(Expression::Designator(designator))
        }
    }

    fn parse_measure_expression(
        &mut self,
    ) -> ParserResult<MeasureExpression> {
        let start = self.expect(
            TokenKind::KwMeasure,
            "expected `measure`",
        )?;

        let operand = self.parse_gate_operand()?;

        let designator = match operand {
            GateOperand::Designator(value) => value,

            GateOperand::Physical(_) => {
                return Err(self.error_at(
                    ParseErrorKind::InvalidOperand,
                    start,
                    "measurement requires an indexed identifier or quantum identifier",
                ));
            }
        };

        let span = self.join_spans(
            start.span(),
            designator.span(),
        )?;

        Ok(MeasureExpression::new(
            span,
            designator,
        ))
    }

    fn parse_optional_expression_list(
        &mut self,
    ) -> ParserResult<Vec<Expression>> {
        if !self.consume(TokenKind::LParen)? {
            return Ok(Vec::new());
        }

        self.parse_expression_list(TokenKind::RParen)
    }

    fn parse_expression_list(
        &mut self,
        terminator: TokenKind,
    ) -> ParserResult<Vec<Expression>> {
        let mut values = Vec::new();

        if self.consume(terminator)? {
            return Ok(values);
        }

        loop {
            if values.len()
                >= self.config.limits.max_gate_parameters
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "expression list exceeds configured limit",
                ));
            }

            values.push(self.parse_expression()?);

            if !self.consume(TokenKind::Comma)? {
                break;
            }

            if self.at(terminator) {
                break;
            }
        }

        self.expect(
            terminator,
            "expected expression-list terminator",
        )?;

        Ok(values)
    }

    fn parse_expression_list_until_scope(
        &mut self,
    ) -> ParserResult<Vec<Expression>> {
        let mut values = Vec::new();

        loop {
            if values.len()
                >= self.config.limits.max_gate_parameters
            {
                return Err(self.error(
                    ParseErrorKind::AstLimitExceeded,
                    "switch case expression list exceeds limit",
                ));
            }

            values.push(self.parse_expression()?);

            if !self.consume(TokenKind::Comma)? {
                break;
            }

            if self.at(TokenKind::LBrace) {
                break;
            }
        }

        Ok(values)
    }

    fn parse_duration_expression(
        &mut self,
    ) -> ParserResult<Expression> {
        let token = self.bump()?;

        let (raw, unit) =
            split_duration_literal(token.lexeme())
                .ok_or_else(|| {
                    self.error_at(
                        ParseErrorKind::InvalidLiteral,
                        token,
                        "invalid duration literal",
                    )
                })?;

        let span =
            self.source_span(token.span())?;

        let numeric =
            if raw.contains('.')
                || raw.contains('e')
                || raw.contains('E')
            {
                Expression::FloatLiteral {
                    node: AstNode::new(span),
                    value: super::ast::FloatLiteral::new(
                        raw.to_owned(),
                    ),
                }
            } else {
                Expression::IntegerLiteral {
                    node: AstNode::new(span),
                    value: IntegerLiteral::new(
                        raw.replace('_', ""),
                        IntegerRadix::Decimal,
                    ),
                }
            };

        self.charge_nodes(2, Some(span))?;

        Ok(Expression::DurationLiteral {
            node: AstNode::new(span),
            value: DurationLiteral::new(
                numeric,
                unit,
            ),
        })
    }

    // -------------------------------------------------------------------------
    // Identifier/token infrastructure
    // -------------------------------------------------------------------------

    fn parse_identifier(
        &mut self,
    ) -> ParserResult<Identifier> {
        let token = self.current();

        if !token.kind().is_identifier_like()
            || token.kind() == TokenKind::HardwareQubit
        {
            return Err(self.error_at(
                ParseErrorKind::ExpectedIdentifier,
                token,
                "expected OpenQASM identifier",
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

    fn parse_identifier_from_keyword(
        &mut self,
    ) -> ParserResult<Identifier> {
        let token = self.bump()?;

        Identifier::new(
            self.source_span(token.span())?,
            token.lexeme().to_owned(),
        )
        .ok_or_else(|| {
            self.error_at(
                ParseErrorKind::ExpectedIdentifier,
                token,
                "invalid keyword-backed identifier",
            )
        })
    }

    #[inline]
    fn current(&self) -> Token<'src> {
        self.tokens[
            self.position.min(
                self.tokens.len().saturating_sub(1)
            )
        ]
    }

    fn current_span(&self) -> SourceSpan {
        self.source_span(self.current().span())
            .unwrap_or_else(|_| {
                SourceSpan::point(
                    self.config.source_id,
                    0,
                )
            })
    }

    fn previous_span(&self) -> SourceSpan {
        if self.position == 0 {
            return self.current_span();
        }

        self.source_span(
            self.tokens[self.position - 1].span(),
        )
        .unwrap_or_else(|_| self.current_span())
    }

    #[inline]
    fn at(&self, kind: TokenKind) -> bool {
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

    fn bump(&mut self) -> ParserResult<Token<'src>> {
        let token = self.current();

        if token.is_eof() {
            return Err(self.error_at(
                ParseErrorKind::UnexpectedEof,
                token,
                "unexpected end of OpenQASM input",
            ));
        }

        let next = self.position.saturating_add(1);

        if next >= self.tokens.len() {
            return Err(self.error_at(
                ParseErrorKind::InvalidTokenStream,
                token,
                "parser attempted to advance beyond token stream",
            ));
        }

        self.position = next;

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
                "AST node spans belong to different sources",
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

    // -------------------------------------------------------------------------
    // Resource accounting
    // -------------------------------------------------------------------------

    fn charge_nodes(
        &mut self,
        count: usize,
        span: Option<SourceSpan>,
    ) -> ParserResult<()> {
        let next = self
            .ast_nodes
            .checked_add(count)
            .ok_or_else(|| {
                ParseError::new(
                    ParseErrorKind::AstLimitExceeded,
                    span,
                    "AST node accounting overflow",
                )
            })?;

        if next > self.config.limits.max_ast_nodes {
            return Err(ParseError::new(
                ParseErrorKind::AstLimitExceeded,
                span,
                "AST node limit exceeded",
            ));
        }

        self.ast_nodes = next;

        Ok(())
    }

    fn enter_nesting(
        &mut self,
        span: super::lexer::Span,
    ) -> ParserResult<()> {
        if self.nesting_depth
            >= self.config.limits.max_nesting_depth
        {
            return Err(ParseError::new(
                ParseErrorKind::NestingLimitExceeded,
                self.source_span(span).ok(),
                "OpenQASM nesting depth exceeds configured limit",
            ));
        }

        self.nesting_depth += 1;

        Ok(())
    }

    fn expect_closing_nesting(&mut self) {
        self.nesting_depth =
            self.nesting_depth.saturating_sub(1);
    }

    fn enter_expression(&mut self) -> ParserResult<()> {
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

    fn leave_expression(&mut self) {
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
    let mut parts = value.split('.');

    let major =
        parts.next()?.parse::<u16>().ok()?;

    let minor =
        match parts.next() {
            Some(value) => value.parse::<u16>().ok()?,
            None => 0,
        };

    if parts.next().is_some() {
        return None;
    }

    Some((major, minor))
}

fn integer_radix(
    value: &str,
) -> IntegerRadix {
    if value.starts_with("0x")
        || value.starts_with("0X")
    {
        IntegerRadix::Hexadecimal
    } else if value.starts_with("0b")
        || value.starts_with("0B")
    {
        IntegerRadix::Binary
    } else if value.starts_with("0o")
        || value.starts_with("0O")
    {
        IntegerRadix::Octal
    } else {
        IntegerRadix::Decimal
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
    /*
     * OpenQASM accepts SI units as well as backend cycles (`dt`).
     *
     * The official grammar also permits the microsecond Unicode spelling.
     */
    let suffixes = [
        ("dt", DurationUnit::Cycles),
        ("ns", DurationUnit::Nanoseconds),
        ("us", DurationUnit::Microseconds),
        ("µs", DurationUnit::Microseconds),
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
    max_bytes: usize,
) -> Option<String> {
    if value.len() < 2 {
        return None;
    }

    let quote = value.as_bytes()[0];

    if quote != b'"'
        && quote != b'\''
    {
        return None;
    }

    if value.as_bytes()[value.len() - 1] != quote {
        return None;
    }

    let body =
        &value[1..value.len() - 1];

    let mut result =
        String::with_capacity(
            body.len().min(max_bytes),
        );

    let mut chars = body.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            if result.len()
                .checked_add(ch.len_utf8())?
                > max_bytes
            {
                return None;
            }

            result.push(ch);
            continue;
        }

        let escaped = chars.next()?;

        let decoded = match escaped {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            '0' => '\0',

            'x' => {
                let a = chars.next()?;
                let b = chars.next()?;

                let byte =
                    u8::from_str_radix(
                        &format!("{a}{b}"),
                        16,
                    )
                    .ok()?;

                char::from(byte)
            }

            'u' => {
                if chars.next()? != '{' {
                    return None;
                }

                let mut digits =
                    String::new();

                loop {
                    let digit = chars.next()?;

                    if digit == '}' {
                        break;
                    }

                    if !digit.is_ascii_hexdigit()
                        || digits.len() >= 6
                    {
                        return None;
                    }

                    digits.push(digit);
                }

                let codepoint =
                    u32::from_str_radix(
                        &digits,
                        16,
                    )
                    .ok()?;

                char::from_u32(codepoint)?
            }

            _ => return None,
        };

        if result
            .len()
            .checked_add(decoded.len_utf8())?
            > max_bytes
        {
            return None;
        }

        result.push(decoded);
    }

    Some(result)
}