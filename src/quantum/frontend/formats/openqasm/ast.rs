//! OpenQASM 3.x abstract syntax tree.
//!
//! This module contains the source-language representation of OpenQASM.
//!
//! # Architectural boundary
//!
//! ```text
//! OpenQASM source
//!       │
//!       ▼
//!     lexer
//!       │
//!       ▼
//! OpenQASM AST  ← this module
//!       │
//!       ▼
//! OpenQASM semantic validation
//!       │
//!       ▼
//! OpenQASM lowering
//!       │
//!       ▼
//! quantum::ir
//! ```
//!
//! This AST intentionally does NOT contain:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `QubitId`;
//! - `Parameter` from the canonical quantum IR;
//! - backend topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - hardware calibration semantics;
//! - QIR/Quil/other-format representations.
//!
//! Those concerns belong to later layers.
//!
//! # Design goals
//!
//! The AST is designed to:
//!
//! - represent OpenQASM syntax without prematurely applying IR semantics;
//! - preserve source-level names and indexing;
//! - preserve source ordering;
//! - preserve constructs that may be parsed but cannot yet be lowered;
//! - support OpenQASM 3.0 and 3.1 version-aware validation;
//! - support Unicode identifiers;
//! - support symbolic expressions;
//! - support classical and quantum declarations;
//! - support user-defined gates;
//! - support subroutines;
//! - support control flow;
//! - support switch/case/default;
//! - support timing constructs;
//! - support annotations and pragmas;
//! - provide enough structure for diagnostics;
//! - avoid hidden filesystem/network access;
//! - avoid execution during parsing;
//! - remain independent from future frontend formats.
//!
//! # Source spans
//!
//! Every syntactic node carries a [`SourceSpan`].
//!
//! `SourceSpan` is deliberately imported from the shared frontend source
//! infrastructure.  The OpenQASM frontend must not define a competing span
//! representation.
//!
//! # Semantic ownership
//!
//! This AST describes what the source says.
//!
//! `validation.rs` determines whether that source is semantically valid
//! OpenQASM.
//!
//! `lowering.rs` determines whether a validated construct can be represented
//! by the canonical Zamani Quantum IR.
//!
//! The AST therefore does not reject a syntactically valid OpenQASM construct
//! merely because the current Zamani IR cannot represent it.
//!
//! # Rust compatibility
//!
//! Target compiler: Rust 1.97.1.
//!
//! This file intentionally uses only stable Rust language/library facilities.
//!
//! # OpenQASM compatibility
//!
//! The model follows the official OpenQASM 3.x language structure.  OpenQASM
//! 3.1 is the current target version. Version-specific semantic restrictions
//! belong in `validation.rs`, not scattered throughout this AST.
//!
//! Official specification:
//! <https://openqasm.com/versions/3.1/>

use crate::quantum::frontend::core::source::SourceSpan;

// -----------------------------------------------------------------------------
// Basic AST infrastructure
// -----------------------------------------------------------------------------

/// Result of an AST structural validation operation.
///
/// Semantic validation is intentionally performed by `validation.rs`.
/// This result exists for local AST invariants only.
pub type AstResult<T> = Result<T, AstError>;

/// Errors representing malformed AST structure rather than invalid OpenQASM
/// semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstError {
    /// A required AST field was absent.
    MissingRequiredField {
        field: &'static str,
    },

    /// A list that must not be empty was empty.
    EmptyRequiredList {
        field: &'static str,
    },

    /// A syntactic structure contained an invalid number of children.
    InvalidChildCount {
        node: &'static str,
        expected: &'static str,
        actual: usize,
    },

    /// A source span was invalid.
    InvalidSpan,

    /// An AST nesting level exceeded the supplied structural limit.
    NestingLimitExceeded {
        limit: usize,
    },
}

impl std::fmt::Display for AstError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::MissingRequiredField { field } => {
                write!(f, "missing required AST field `{field}`")
            }

            Self::EmptyRequiredList { field } => {
                write!(f, "AST field `{field}` must not be empty")
            }

            Self::InvalidChildCount {
                node,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "AST node `{node}` requires {expected} children, got {actual}"
                )
            }

            Self::InvalidSpan => {
                write!(f, "AST node contains an invalid source span")
            }

            Self::NestingLimitExceeded { limit } => {
                write!(
                    f,
                    "AST nesting depth exceeds configured limit {limit}"
                )
            }
        }
    }
}

impl std::error::Error for AstError {}

/// Every top-level OpenQASM AST node that participates in diagnostics should
/// expose its source span.
pub trait Spanned {
    /// Returns the complete source span occupied by this node.
    fn span(&self) -> SourceSpan;
}

/// OpenQASM language version.
///
/// Version is represented explicitly because semantic and standard-library
/// availability can depend on the selected language version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpenQasmVersion {
    /// OpenQASM 3.0.
    V3_0,

    /// OpenQASM 3.1.
    V3_1,
}

impl OpenQasmVersion {
    /// Returns the canonical numeric major version.
    #[must_use]
    pub const fn major(self) -> u8 {
        3
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u8 {
        match self {
            Self::V3_0 => 0,
            Self::V3_1 => 1,
        }
    }

    /// Returns the canonical source spelling used by the exporter.
    #[must_use]
    pub const fn source_text(self) -> &'static str {
        match self {
            Self::V3_0 => "OPENQASM 3.0;",
            Self::V3_1 => "OPENQASM 3.1;",
        }
    }
}

impl std::fmt::Display for OpenQasmVersion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}

// -----------------------------------------------------------------------------
// Program
// -----------------------------------------------------------------------------

/// Complete OpenQASM source program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Source span covering the complete program.
    pub span: SourceSpan,

    /// Language version declared by the source.
    pub version: Option<VersionDeclaration>,

    /// Ordered top-level statements.
    ///
    /// Source ordering is significant and must never be replaced by a map.
    pub statements: Vec<Statement>,

    /// Optional source-level trailing annotations/comments that the parser
    /// explicitly chooses to preserve.
    ///
    /// Comments are normally lexer trivia and need not enter the AST.  This
    /// field is intentionally absent; comments should not become semantic
    /// program data unless a future formatter explicitly requires a trivia
    /// AST.
}

impl Spanned for Program {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl Program {
    /// Creates a program.
    #[must_use]
    pub fn new(
        span: SourceSpan,
        version: Option<VersionDeclaration>,
        statements: Vec<Statement>,
    ) -> Self {
        Self {
            span,
            version,
            statements,
        }
    }

    /// Returns the program's declared version.
    #[must_use]
    pub fn version(&self) -> Option<OpenQasmVersion> {
        self.version.as_ref().map(|version| version.version)
    }
}

/// The OpenQASM version declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionDeclaration {
    /// Complete source span.
    pub span: SourceSpan,

    /// Declared language version.
    pub version: OpenQasmVersion,
}

impl Spanned for VersionDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Statements
// -----------------------------------------------------------------------------

/// A top-level or block-level OpenQASM statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Include another OpenQASM source.
    Include(IncludeStatement),

    /// Import a calibration grammar.
    DefcalGrammar(DefcalGrammarStatement),

    /// Quantum declaration.
    QuantumDeclaration(QuantumDeclaration),

    /// Classical declaration.
    ClassicalDeclaration(ClassicalDeclaration),

    /// Alias declaration.
    AliasDeclaration(AliasDeclaration),

    /// User-defined gate.
    GateDefinition(GateDefinition),

    /// User-defined subroutine.
    SubroutineDefinition(SubroutineDefinition),

    /// External function declaration.
    ExternDeclaration(ExternDeclaration),

    /// Gate invocation.
    GateCall(GateCall),

    /// Measurement.
    Measurement(MeasurementStatement),

    /// Reset.
    Reset(ResetStatement),

    /// Barrier.
    Barrier(BarrierStatement),

    /// Delay.
    Delay(DelayStatement),

    /// Boxed timing block.
    Box(BoxStatement),

    /// Assignment.
    Assignment(AssignmentStatement),

    /// Variable declaration with optional initializer.
    VariableDeclaration(VariableDeclaration),

    /// Classical expression used as a statement.
    Expression(ExpressionStatement),

    /// If/else control flow.
    If(IfStatement),

    /// For loop.
    For(ForStatement),

    /// While loop.
    While(WhileStatement),

    /// Switch/case/default control flow.
    Switch(SwitchStatement),

    /// Return from a subroutine.
    Return(ReturnStatement),

    /// Break from a loop.
    Break(BreakStatement),

    /// Continue a loop.
    Continue(ContinueStatement),

    /// `let` binding.
    Let(LetStatement),

    /// Inline calibration definition.
    Defcal(DefcalDefinition),

    /// Inline calibration execution block.
    Calibration(CalibrationStatement),

    /// OpenPulse/calibration statement.
    ///
    /// The exact pulse grammar is deliberately represented separately so that
    /// the normal OpenQASM AST does not become coupled to backend pulse
    /// semantics.
    Pulse(PulseStatement),

    /// OpenQASM pragma.
    Pragma(PragmaStatement),

    /// OpenQASM annotation.
    Annotation(AnnotationStatement),

    /// Explicit empty statement.
    Empty(EmptyStatement),
}

impl Spanned for Statement {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Include(value) => value.span(),
            Self::DefcalGrammar(value) => value.span(),
            Self::QuantumDeclaration(value) => value.span(),
            Self::ClassicalDeclaration(value) => value.span(),
            Self::AliasDeclaration(value) => value.span(),
            Self::GateDefinition(value) => value.span(),
            Self::SubroutineDefinition(value) => value.span(),
            Self::ExternDeclaration(value) => value.span(),
            Self::GateCall(value) => value.span(),
            Self::Measurement(value) => value.span(),
            Self::Reset(value) => value.span(),
            Self::Barrier(value) => value.span(),
            Self::Delay(value) => value.span(),
            Self::Box(value) => value.span(),
            Self::Assignment(value) => value.span(),
            Self::VariableDeclaration(value) => value.span(),
            Self::Expression(value) => value.span(),
            Self::If(value) => value.span(),
            Self::For(value) => value.span(),
            Self::While(value) => value.span(),
            Self::Switch(value) => value.span(),
            Self::Return(value) => value.span(),
            Self::Break(value) => value.span(),
            Self::Continue(value) => value.span(),
            Self::Let(value) => value.span(),
            Self::Defcal(value) => value.span(),
            Self::Calibration(value) => value.span(),
            Self::Pulse(value) => value.span(),
            Self::Pragma(value) => value.span(),
            Self::Annotation(value) => value.span(),
            Self::Empty(value) => value.span(),
        }
    }
}

// -----------------------------------------------------------------------------
// Identifiers and literals
// -----------------------------------------------------------------------------

/// OpenQASM identifier.
///
/// The parser/validator must enforce the OpenQASM Unicode identifier rules.
/// The AST deliberately does not reduce identifiers to ASCII.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    /// Complete source span.
    pub span: SourceSpan,

    /// Source spelling.
    pub name: String,
}

impl Spanned for Identifier {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl Identifier {
    /// Creates an identifier without performing semantic validation.
    ///
    /// Lexical validation belongs to the lexer; reserved-name validation
    /// belongs to semantic validation.
    #[must_use]
    pub fn new(
        span: SourceSpan,
        name: String,
    ) -> Self {
        Self { span, name }
    }

    /// Returns the source spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// A source-level literal.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Boolean literal.
    Bool {
        span: SourceSpan,
        value: bool,
    },

    /// Integer literal preserving its original spelling/base.
    Integer(IntegerLiteral),

    /// Floating-point literal preserving source spelling.
    Float(FloatLiteral),

    /// Imaginary literal.
    Imaginary(ImaginaryLiteral),

    /// Bit-string literal.
    BitString(BitStringLiteral),

    /// String literal.
    String(StringLiteral),

    /// Duration literal.
    Duration(DurationLiteral),

    /// Stretch literal.
    Stretch(StretchLiteral),
}

impl Spanned for Literal {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Bool { span, .. } => *span,
            Self::Integer(value) => value.span(),
            Self::Float(value) => value.span(),
            Self::Imaginary(value) => value.span(),
            Self::BitString(value) => value.span(),
            Self::String(value) => value.span(),
            Self::Duration(value) => value.span(),
            Self::Stretch(value) => value.span(),
        }
    }
}

/// Integer literal base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

/// Integer literal.
///
/// The original spelling is preserved because an AST is a source
/// representation, not a canonical numerical representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub span: SourceSpan,
    pub raw: String,
    pub base: IntegerBase,
}

impl Spanned for IntegerLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Floating-point literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

impl Spanned for FloatLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Imaginary literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImaginaryLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

impl Spanned for ImaginaryLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Bit-string literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitStringLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

impl Spanned for BitStringLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// String literal.
///
/// The parser should store the decoded value and preserve the raw spelling
/// only if the lexer/parser contract explicitly requires it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub span: SourceSpan,
    pub value: String,
}

impl Spanned for StringLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// OpenQASM duration literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

/// OpenQASM stretch literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StretchLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

impl Spanned for DurationLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl Spanned for StretchLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Include/directive statements
// -----------------------------------------------------------------------------

/// `include "file";`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeStatement {
    pub span: SourceSpan,
    pub path: StringLiteral,
}

impl Spanned for IncludeStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// `defcalgrammar "name";`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefcalGrammarStatement {
    pub span: SourceSpan,
    pub path: StringLiteral,
}

impl Spanned for DefcalGrammarStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// `pragma ...`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PragmaStatement {
    pub span: SourceSpan,

    /// Text following the pragma directive.
    pub body: String,
}

impl Spanned for PragmaStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// `@annotation ...`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationStatement {
    pub span: SourceSpan,

    /// Annotation keyword without the leading `@`.
    pub name: Identifier,

    /// Raw annotation arguments/text.
    pub arguments: String,
}

impl Spanned for AnnotationStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Quantum declarations
// -----------------------------------------------------------------------------

/// Quantum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub size: Option<Expression>,
    pub input: bool,
    pub output: bool,
}

impl Spanned for QuantumDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Physical qubit reference such as `$0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalQubit {
    pub span: SourceSpan,
    pub index: u64,
}

impl Spanned for PhysicalQubit {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Quantum operand.
///
/// This is still a source-level operand. It is deliberately not a canonical
/// IR qubit identifier.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumOperand {
    /// A named quantum object.
    Identifier(Identifier),

    /// A single indexed element.
    Indexed(IndexExpression),

    /// A slice/range.
    Slice(SliceExpression),

    /// A physical hardware qubit.
    Physical(PhysicalQubit),

    /// A concatenation or expanded register expression.
    Concatenation {
        span: SourceSpan,
        operands: Vec<QuantumOperand>,
    },
}

impl Spanned for QuantumOperand {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Identifier(value) => value.span(),
            Self::Indexed(value) => value.span(),
            Self::Slice(value) => value.span(),
            Self::Physical(value) => value.span(),
            Self::Concatenation { span, .. } => *span,
        }
    }
}

/// Generic operand usable by gate/subroutine calls.
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Quantum(QuantumOperand),
    Classical(Expression),
}

impl Spanned for Argument {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Quantum(value) => value.span(),
            Self::Classical(value) => value.span(),
        }
    }
}

// -----------------------------------------------------------------------------
// Classical types
// -----------------------------------------------------------------------------

/// OpenQASM type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// `bool`
    Bool,

    /// `bit`
    Bit {
        width: Option<u64>,
    },

    /// Signed integer.
    Int {
        width: u64,
    },

    /// Unsigned integer.
    Uint {
        width: u64,
    },

    /// Floating-point value.
    Float {
        width: u64,
    },

    /// Angle.
    Angle {
        width: u64,
    },

    /// Complex value.
    Complex {
        width: u64,
    },

    /// Duration.
    Duration,

    /// Stretch.
    Stretch,

    /// Void return type.
    Void,

    /// Array type.
    Array {
        element: Box<Type>,
        dimensions: Vec<Expression>,
    },

    /// Named/implementation-defined type.
    Named(Identifier),
}

impl Type {
    /// Returns true for a quantum-free classical type.
    #[must_use]
    pub const fn is_classical(&self) -> bool {
        true
    }
}

// -----------------------------------------------------------------------------
// Classical declarations
// -----------------------------------------------------------------------------

/// Classical declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalDeclaration {
    pub span: SourceSpan,
    pub ty: Type,
    pub name: Identifier,
    pub initializer: Option<Expression>,
    pub input: bool,
    pub output: bool,
    pub readonly: bool,
    pub mutable: bool,
    pub constant: bool,
}

impl Spanned for ClassicalDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// General variable declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub span: SourceSpan,
    pub ty: Type,
    pub name: Identifier,
    pub initializer: Option<Expression>,
    pub input: bool,
    pub output: bool,
    pub readonly: bool,
    pub mutable: bool,
    pub constant: bool,
}

impl Spanned for VariableDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// `let` binding.
#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub span: SourceSpan,
    pub name: Identifier,
    pub value: Expression,
}

impl Spanned for LetStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Alias declaration.
///
/// The exact semantic restrictions are checked by `validation.rs`.
#[derive(Debug, Clone, PartialEq)]
pub struct AliasDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub value: AliasExpression,
}

impl Spanned for AliasDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Alias source expression.
#[derive(Debug, Clone, PartialEq)]
pub enum AliasExpression {
    Operand(QuantumOperand),

    Concatenation {
        span: SourceSpan,
        operands: Vec<QuantumOperand>,
    },

    Slice {
        span: SourceSpan,
        operand: Box<QuantumOperand>,
        index: Box<IndexExpression>,
    },
}

impl Spanned for AliasExpression {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Operand(value) => value.span(),
            Self::Concatenation { span, .. } => *span,
            Self::Slice { span, .. } => *span,
        }
    }
}

// -----------------------------------------------------------------------------
// Indexing and slices
// -----------------------------------------------------------------------------

/// Index expression.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub span: SourceSpan,
    pub target: Identifier,
    pub index: Expression,
}

impl Spanned for IndexExpression {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Slice expression.
///
/// `start`, `stop`, and `step` are optional according to the OpenQASM slicing
/// grammar supported by the parser.
#[derive(Debug, Clone, PartialEq)]
pub struct SliceExpression {
    pub span: SourceSpan,
    pub target: Identifier,
    pub start: Option<Expression>,
    pub stop: Option<Expression>,
    pub step: Option<Expression>,
}

impl Spanned for SliceExpression {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Expressions
// -----------------------------------------------------------------------------

/// OpenQASM expression.
///
/// Expressions remain source-level and are not lowered to the canonical IR
/// `ParameterExpression` in this module.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),

    Identifier(Identifier),

    Unary {
        span: SourceSpan,
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    Binary {
        span: SourceSpan,
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Call {
        span: SourceSpan,
        function: Identifier,
        arguments: Vec<Expression>,
    },

    Cast {
        span: SourceSpan,
        target: Type,
        expression: Box<Expression>,
    },

    Index {
        span: SourceSpan,
        target: Box<Expression>,
        index: Box<Expression>,
    },

    Slice {
        span: SourceSpan,
        target: Box<Expression>,
        start: Option<Box<Expression>>,
        stop: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },

    ArrayLiteral {
        span: SourceSpan,
        values: Vec<Expression>,
    },

    Bitstring {
        span: SourceSpan,
        value: BitStringLiteral,
    },

    DurationOf {
        span: SourceSpan,
        body: Box<Statement>,
    },

    FunctionReference {
        span: SourceSpan,
        name: Identifier,
    },

    Parenthesized {
        span: SourceSpan,
        expression: Box<Expression>,
    },
}

impl Spanned for Expression {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Literal(value) => value.span(),
            Self::Identifier(value) => value.span(),

            Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::Call { span, .. }
            | Self::Cast { span, .. }
            | Self::Index { span, .. }
            | Self::Slice { span, .. }
            | Self::ArrayLiteral { span, .. }
            | Self::Bitstring { span, .. }
            | Self::DurationOf { span, .. }
            | Self::FunctionReference { span, .. }
            | Self::Parenthesized { span, .. } => *span,
        }
    }
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    Equal,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LogicalAnd,
    LogicalOr,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    ShiftLeft,
    ShiftRight,
}

// -----------------------------------------------------------------------------
// Gate definitions
// -----------------------------------------------------------------------------

/// User-defined gate.
#[derive(Debug, Clone, PartialEq)]
pub struct GateDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub parameters: Vec<GateParameterDeclaration>,
    pub qubits: Vec<GateQubitDeclaration>,
    pub body: Vec<GateBodyStatement>,
}

impl Spanned for GateDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Gate parameter declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct GateParameterDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
}

impl Spanned for GateParameterDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Gate-local qubit declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct GateQubitDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
}

impl Spanned for GateQubitDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Statements permitted in a gate body.
#[derive(Debug, Clone, PartialEq)]
pub enum GateBodyStatement {
    GateCall(GateCall),

    Barrier(BarrierStatement),

    Delay(DelayStatement),

    Box(BoxStatement),

    If(IfStatement),

    For(ForStatement),

    While(WhileStatement),

    Switch(SwitchStatement),

    Annotation(AnnotationStatement),
}

impl Spanned for GateBodyStatement {
    fn span(&self) -> SourceSpan {
        match self {
            Self::GateCall(value) => value.span(),
            Self::Barrier(value) => value.span(),
            Self::Delay(value) => value.span(),
            Self::Box(value) => value.span(),
            Self::If(value) => value.span(),
            Self::For(value) => value.span(),
            Self::While(value) => value.span(),
            Self::Switch(value) => value.span(),
            Self::Annotation(value) => value.span(),
        }
    }
}

// -----------------------------------------------------------------------------
// Gate calls
// -----------------------------------------------------------------------------

/// Gate invocation.
///
/// This is the critical source-level structure used by the future importer.
/// It preserves the actual operand list rather than assuming qubit positions.
#[derive(Debug, Clone, PartialEq)]
pub struct GateCall {
    pub span: SourceSpan,

    /// Optional modifiers such as `ctrl`, `negctrl`, `inv`, and `pow`.
    pub modifiers: Vec<GateModifier>,

    /// Gate identifier.
    pub name: Identifier,

    /// Gate parameter expressions.
    pub parameters: Vec<Expression>,

    /// Quantum operands in source order.
    pub operands: Vec<QuantumOperand>,
}

impl Spanned for GateCall {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Gate modifier.
#[derive(Debug, Clone, PartialEq)]
pub enum GateModifier {
    /// `ctrl`
    Ctrl {
        span: SourceSpan,
        count: Option<Expression>,
    },

    /// `negctrl`
    NegCtrl {
        span: SourceSpan,
        count: Option<Expression>,
    },

    /// `inv`
    Inverse {
        span: SourceSpan,
    },

    /// `pow(...)`
    Power {
        span: SourceSpan,
        exponent: Expression,
    },
}

impl Spanned for GateModifier {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Ctrl { span, .. }
            | Self::NegCtrl { span, .. }
            | Self::Inverse { span }
            | Self::Power { span, .. } => *span,
        }
    }
}

// -----------------------------------------------------------------------------
// Measurement/reset/barrier
// -----------------------------------------------------------------------------

/// Measurement statement.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementStatement {
    pub span: SourceSpan,

    /// Quantum source being measured.
    pub source: QuantumOperand,

    /// Optional classical destination.
    ///
    /// Keeping this optional is important because validation must distinguish
    /// source-language forms from semantic validity.
    pub destination: Option<Expression>,
}

impl Spanned for MeasurementStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Reset statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ResetStatement {
    pub span: SourceSpan,
    pub target: QuantumOperand,
}

impl Spanned for ResetStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Barrier statement.
#[derive(Debug, Clone, PartialEq)]
pub struct BarrierStatement {
    pub span: SourceSpan,
    pub operands: Vec<QuantumOperand>,
}

impl Spanned for BarrierStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Timing
// -----------------------------------------------------------------------------

/// Delay statement.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayStatement {
    pub span: SourceSpan,

    /// Duration expression.
    pub duration: Expression,

    /// Quantum targets.
    pub targets: Vec<QuantumOperand>,
}

impl Spanned for DelayStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Boxed timing construct.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxStatement {
    pub span: SourceSpan,

    /// Optional duration bound.
    pub duration: Option<Expression>,

    /// Body.
    pub body: Vec<Statement>,
}

impl Spanned for BoxStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Assignments and expressions
// -----------------------------------------------------------------------------

/// Assignment operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    BitNotAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    ModuloAssign,
    PowerAssign,
}

/// Assignment statement.
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    pub span: SourceSpan,
    pub target: Expression,
    pub operator: AssignmentOperator,
    pub value: Expression,
}

impl Spanned for AssignmentStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub span: SourceSpan,
    pub expression: Expression,
}

impl Spanned for ExpressionStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Control flow
// -----------------------------------------------------------------------------

/// General statement block.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub span: SourceSpan,
    pub statements: Vec<Statement>,
}

impl Spanned for Block {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// If statement.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub span: SourceSpan,
    pub condition: Expression,
    pub then_body: Block,
    pub else_body: Option<Block>,
}

impl Spanned for IfStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// For-loop variable.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopVariable {
    pub span: SourceSpan,
    pub name: Identifier,
    pub ty: Option<Type>,
}

impl Spanned for LoopVariable {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// For loop.
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub span: SourceSpan,
    pub variable: LoopVariable,
    pub iterable: Expression,
    pub body: Block,
}

impl Spanned for ForStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// While loop.
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub span: SourceSpan,
    pub condition: Expression,
    pub body: Block,
}

impl Spanned for WhileStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Switch statement.
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
    pub span: SourceSpan,
    pub expression: Expression,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Block>,
}

impl Spanned for SwitchStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Switch case.
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub span: SourceSpan,
    pub values: Vec<Expression>,
    pub body: Block,
}

impl Spanned for SwitchCase {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Return statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub span: SourceSpan,
    pub value: Option<Expression>,
}

impl Spanned for ReturnStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Break statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BreakStatement {
    pub span: SourceSpan,
}

impl Spanned for BreakStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Continue statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinueStatement {
    pub span: SourceSpan,
}

impl Spanned for ContinueStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Empty statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyStatement {
    pub span: SourceSpan,
}

impl Spanned for EmptyStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Subroutines
// -----------------------------------------------------------------------------

/// User-defined subroutine.
#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub return_type: Option<Type>,
    pub parameters: Vec<SubroutineParameter>,
    pub body: Block,
}

impl Spanned for SubroutineDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Subroutine parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineParameter {
    pub span: SourceSpan,
    pub ty: Type,
    pub name: Identifier,
    pub input: bool,
    pub output: bool,
    pub readonly: bool,
    pub mutable: bool,
}

impl Spanned for SubroutineParameter {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// External function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ExternDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub return_type: Type,
    pub parameters: Vec<ExternParameter>,
}

impl Spanned for ExternDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// External function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct ExternParameter {
    pub span: SourceSpan,
    pub ty: Type,
    pub name: Option<Identifier>,
}

impl Spanned for ExternParameter {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Defcal / calibration
// -----------------------------------------------------------------------------

/// Inline calibration definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DefcalDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub parameters: Vec<Expression>,
    pub qubits: Vec<QuantumOperand>,
    pub return_type: Option<Type>,
    pub body: CalibrationBody,
}

impl Spanned for DefcalDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Calibration statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationStatement {
    pub span: SourceSpan,
    pub body: CalibrationBody,
}

impl Spanned for CalibrationStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Calibration body.
///
/// Raw text is intentionally retained at this boundary. The parser can later
/// expose a richer OpenPulse AST without changing ordinary OpenQASM nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationBody {
    pub span: SourceSpan,
    pub source: String,
}

impl Spanned for CalibrationBody {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Pulse-level statement.
///
/// This is intentionally opaque to the ordinary circuit importer. A later
/// OpenPulse-specific layer can lower supported constructs explicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseStatement {
    pub span: SourceSpan,
    pub source: String,
}

impl Spanned for PulseStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// Return / call helpers
// -----------------------------------------------------------------------------

/// A generic source-level callable invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub span: SourceSpan,
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

impl Spanned for CallExpression {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// -----------------------------------------------------------------------------
// AST utilities
// -----------------------------------------------------------------------------

impl Program {
    /// Walk all top-level statements in source order.
    ///
    /// This does not perform semantic validation.
    pub fn visit_statements<F>(
        &self,
        mut visitor: F,
    )
    where
        F: FnMut(&Statement),
    {
        for statement in &self.statements {
            visitor(statement);
        }
    }

    /// Returns the number of top-level statements.
    #[must_use]
    pub fn statement_count(&self) -> usize {
        self.statements.len()
    }
}

impl GateCall {
    /// Returns true when the call has no explicit modifiers.
    #[must_use]
    pub fn is_unmodified(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Returns the number of quantum operands.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Returns the number of gate parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }
}

impl GateDefinition {
    /// Returns the number of formal quantum operands.
    #[must_use]
    pub fn qubit_parameter_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of formal scalar parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }
}

impl MeasurementStatement {
    /// Returns true when the measurement has an explicit classical target.
    #[must_use]
    pub fn has_destination(&self) -> bool {
        self.destination.is_some()
    }
}

impl BarrierStatement {
    /// Returns the number of barrier operands.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }
}

impl DelayStatement {
    /// Returns the number of delayed quantum targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }
}

impl Block {
    /// Returns true when the block has no statements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Structural validation
// -----------------------------------------------------------------------------

impl Program {
    /// Performs only structural validation.
    ///
    /// This deliberately does NOT check:
    ///
    /// - undefined identifiers;
    /// - duplicate names;
    /// - scope;
    /// - type compatibility;
    /// - gate arity;
    /// - standard-library availability;
    /// - OpenQASM version rules;
    /// - IR representability.
    ///
    /// Those belong to `validation.rs`.
    pub fn validate_structure(
        &self,
        max_depth: usize,
    ) -> AstResult<()> {
        for statement in &self.statements {
            validate_statement_depth(statement, 0, max_depth)?;
        }

        Ok(())
    }
}

fn validate_statement_depth(
    statement: &Statement,
    depth: usize,
    max_depth: usize,
) -> AstResult<()> {
    if depth > max_depth {
        return Err(
            AstError::NestingLimitExceeded {
                limit: max_depth,
            },
        );
    }

    match statement {
        Statement::If(value) => {
            validate_block_depth(
                &value.then_body,
                depth + 1,
                max_depth,
            )?;

            if let Some(body) = &value.else_body {
                validate_block_depth(
                    body,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        Statement::For(value) => {
            validate_block_depth(
                &value.body,
                depth + 1,
                max_depth,
            )?;
        }

        Statement::While(value) => {
            validate_block_depth(
                &value.body,
                depth + 1,
                max_depth,
            )?;
        }

        Statement::Switch(value) => {
            for case in &value.cases {
                validate_block_depth(
                    &case.body,
                    depth + 1,
                    max_depth,
                )?;
            }

            if let Some(body) = &value.default {
                validate_block_depth(
                    body,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        Statement::Box(value) => {
            for child in &value.body {
                validate_statement_depth(
                    child,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        Statement::GateDefinition(value) => {
            for child in &value.body {
                validate_gate_body_depth(
                    child,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        Statement::SubroutineDefinition(value) => {
            validate_block_depth(
                &value.body,
                depth + 1,
                max_depth,
            )?;
        }

        _ => {}
    }

    Ok(())
}

fn validate_block_depth(
    block: &Block,
    depth: usize,
    max_depth: usize,
) -> AstResult<()> {
    if depth > max_depth {
        return Err(
            AstError::NestingLimitExceeded {
                limit: max_depth,
            },
        );
    }

    for statement in &block.statements {
        validate_statement_depth(
            statement,
            depth + 1,
            max_depth,
        )?;
    }

    Ok(())
}

fn validate_gate_body_depth(
    statement: &GateBodyStatement,
    depth: usize,
    max_depth: usize,
) -> AstResult<()> {
    if depth > max_depth {
        return Err(
            AstError::NestingLimitExceeded {
                limit: max_depth,
            },
        );
    }

    match statement {
        GateBodyStatement::If(value) => {
            validate_block_depth(
                &value.then_body,
                depth + 1,
                max_depth,
            )?;

            if let Some(body) = &value.else_body {
                validate_block_depth(
                    body,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        GateBodyStatement::For(value) => {
            validate_block_depth(
                &value.body,
                depth + 1,
                max_depth,
            )?;
        }

        GateBodyStatement::While(value) => {
            validate_block_depth(
                &value.body,
                depth + 1,
                max_depth,
            )?;
        }

        GateBodyStatement::Switch(value) => {
            for case in &value.cases {
                validate_block_depth(
                    &case.body,
                    depth + 1,
                    max_depth,
                )?;
            }

            if let Some(body) = &value.default {
                validate_block_depth(
                    body,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        GateBodyStatement::Box(value) => {
            for child in &value.body {
                validate_statement_depth(
                    child,
                    depth + 1,
                    max_depth,
                )?;
            }
        }

        GateBodyStatement::GateCall(_)
        | GateBodyStatement::Barrier(_)
        | GateBodyStatement::Delay(_)
        | GateBodyStatement::Annotation(_) => {}
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Source-independent classification helpers
// -----------------------------------------------------------------------------

impl Statement {
    /// Returns true for operations that directly apply to quantum operands.
    #[must_use]
    pub const fn is_quantum_operation(&self) -> bool {
        matches!(
            self,
            Self::GateCall(_)
                | Self::Measurement(_)
                | Self::Reset(_)
                | Self::Barrier(_)
                | Self::Delay(_)
                | Self::Box(_)
        )
    }

    /// Returns true when the statement defines a named symbol.
    #[must_use]
    pub fn defines_symbol(&self) -> bool {
        matches!(
            self,
            Self::QuantumDeclaration(_)
                | Self::ClassicalDeclaration(_)
                | Self::VariableDeclaration(_)
                | Self::AliasDeclaration(_)
                | Self::GateDefinition(_)
                | Self::SubroutineDefinition(_)
                | Self::ExternDeclaration(_)
                | Self::Defcal(_)
        )
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> SourceSpan {
        SourceSpan::new(
            0,
            0,
            0,
        )
    }

    #[test]
    fn version_is_explicit() {
        assert_eq!(
            OpenQasmVersion::V3_0.to_string(),
            "3.0"
        );

        assert_eq!(
            OpenQasmVersion::V3_1.to_string(),
            "3.1"
        );
    }

    #[test]
    fn version_source_text_is_stable() {
        assert_eq!(
            OpenQasmVersion::V3_0.source_text(),
            "OPENQASM 3.0;"
        );

        assert_eq!(
            OpenQasmVersion::V3_1.source_text(),
            "OPENQASM 3.1;"
        );
    }

    #[test]
    fn gate_call_preserves_operand_order() {
        let call = GateCall {
            span: span(),
            modifiers: Vec::new(),
            name: Identifier::new(
                span(),
                "cx".to_owned(),
            ),
            parameters: Vec::new(),
            operands: vec![
                QuantumOperand::Identifier(
                    Identifier::new(
                        span(),
                        "q".to_owned(),
                    ),
                ),
                QuantumOperand::Indexed(
                    IndexExpression {
                        span: span(),
                        target: Identifier::new(
                            span(),
                            "q".to_owned(),
                        ),
                        index: Expression::Literal(
                            Literal::Integer(
                                IntegerLiteral {
                                    span: span(),
                                    raw: "1".to_owned(),
                                    base: IntegerBase::Decimal,
                                },
                            ),
                        ),
                    },
                ),
            ],
        };

        assert_eq!(call.operand_count(), 2);
        assert_eq!(
            call.parameters.len(),
            0
        );
    }

    #[test]
    fn measurement_does_not_force_a_destination() {
        let measurement = MeasurementStatement {
            span: span(),
            source: QuantumOperand::Identifier(
                Identifier::new(
                    span(),
                    "q".to_owned(),
                ),
            ),
            destination: None,
        };

        assert!(!measurement.has_destination());
    }

    #[test]
    fn program_preserves_statement_order() {
        let program = Program::new(
            span(),
            Some(VersionDeclaration {
                span: span(),
                version: OpenQasmVersion::V3_1,
            }),
            vec![
                Statement::Empty(
                    EmptyStatement {
                        span: span(),
                    },
                ),
                Statement::Empty(
                    EmptyStatement {
                        span: span(),
                    },
                ),
            ],
        );

        assert_eq!(
            program.statement_count(),
            2
        );
    }

    #[test]
    fn nested_blocks_are_checked_against_depth() {
        let program = Program::new(
            span(),
            None,
            vec![
                Statement::If(
                    IfStatement {
                        span: span(),
                        condition:
                            Expression::Literal(
                                Literal::Bool {
                                    span: span(),
                                    value: true,
                                },
                            ),
                        then_body: Block {
                            span: span(),
                            statements: vec![],
                        },
                        else_body: None,
                    },
                ),
            ],
        );

        assert!(
            program
                .validate_structure(4)
                .is_ok()
        );
    }

    #[test]
    fn statement_classification_is_source_level() {
        let statement = Statement::GateCall(
            GateCall {
                span: span(),
                modifiers: Vec::new(),
                name: Identifier::new(
                    span(),
                    "h".to_owned(),
                ),
                parameters: Vec::new(),
                operands: vec![],
            },
        );

        assert!(
            statement.is_quantum_operation()
        );
    }
}