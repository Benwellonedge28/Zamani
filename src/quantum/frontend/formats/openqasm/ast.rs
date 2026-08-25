//! OpenQASM 3.x abstract syntax tree.
//!
//! This module is the source-language representation of OpenQASM.
//!
//! Architectural boundary:
//!
//! ```text
//! OpenQASM source
//!       |
//!       v
//!     lexer
//!       |
//!       v
//!   OpenQASM AST              <- this module
//!       |
//!       v
//!   semantic validation
//!       |
//!       v
//!   frontend lowering
//!       |
//!       v
//!   canonical Quantum IR
//! ```
//!
//! # Responsibilities
//!
//! This module:
//!
//! - represents OpenQASM syntax;
//! - preserves source ordering;
//! - preserves source identifiers;
//! - preserves source spans;
//! - preserves symbolic expressions;
//! - preserves classical types;
//! - preserves quantum declarations;
//! - preserves gate definitions and modifiers;
//! - preserves subroutine definitions;
//! - preserves control flow;
//! - preserves timing constructs;
//! - preserves calibration constructs;
//! - preserves annotations and pragmas;
//! - preserves constructs that cannot currently be lowered to Quantum IR.
//!
//! This module does NOT:
//!
//! - resolve symbols;
//! - perform type checking;
//! - perform gate validation;
//! - perform include resolution;
//! - access the filesystem;
//! - access the network;
//! - execute code;
//! - execute calibration;
//! - construct QuantumCircuit;
//! - construct GateKind;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - perform hardware mapping.
//!
//! Those responsibilities belong to later frontend layers.
//!
//! # Source spans
//!
//! All syntactic nodes that can participate in diagnostics carry a
//! `SourceSpan` from the shared frontend source infrastructure.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! Rust 2021.
//! Stable Rust only.
//!
//! # OpenQASM compatibility
//!
//! The AST is intentionally capable of representing OpenQASM 3.x syntax.
//! Version-specific semantic restrictions belong in `validation.rs`.
//!
//! Official specification:
//! <https://openqasm.com/versions/3.1/>

use crate::quantum::frontend::core::source::SourceSpan;

use std::fmt;

// ============================================================================
// Core AST infrastructure
// ============================================================================

/// Result of an AST-local operation.
pub type AstResult<T> = Result<T, AstError>;

/// Structural AST errors.
///
/// These are not OpenQASM semantic errors. Semantic validation belongs in
/// `validation.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstError {
    MissingRequiredField {
        field: &'static str,
    },

    EmptyRequiredList {
        field: &'static str,
    },

    InvalidChildCount {
        node: &'static str,
        expected: &'static str,
        actual: usize,
    },

    InvalidSpan,

    InvalidVersion {
        major: u32,
        minor: u32,
    },

    InvalidIdentifier,

    NestingLimitExceeded {
        limit: usize,
    },
}

impl fmt::Display for AstError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

            Self::InvalidVersion { major, minor } => {
                write!(
                    f,
                    "unsupported OpenQASM AST version {}.{}",
                    major,
                    minor
                )
            }

            Self::InvalidIdentifier => {
                write!(f, "invalid OpenQASM identifier")
            }

            Self::NestingLimitExceeded { limit } => {
                write!(
                    f,
                    "AST nesting depth exceeds configured limit {}",
                    limit
                )
            }
        }
    }
}

impl std::error::Error for AstError {}

/// Trait implemented by AST nodes that have a source span.
pub trait Spanned {
    fn span(&self) -> SourceSpan;
}

/// Compatibility trait used by parser/tooling code that wants a common AST
/// node abstraction.
///
/// This trait deliberately exposes only source-location information.
pub trait AstNode: Spanned {
    fn node_kind(&self) -> AstNodeKind;
}

/// Stable AST node classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstNodeKind {
    Program,
    VersionDeclaration,
    Statement,
    Identifier,
    Literal,
    Expression,
    TypeSpecifier,
    QuantumType,
    ScalarType,
    GateDefinition,
    GateCall,
    Measurement,
    Declaration,
    ControlFlow,
    Annotation,
    Pragma,
}

// ============================================================================
// OpenQASM version
// ============================================================================

/// Supported OpenQASM major/minor language version.
///
/// The AST represents only versions that have an explicit semantic policy.
/// Future versions must not be silently mapped to the newest supported
/// version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpenQasmVersion {
    V3_0,
    V3_1,
}

impl OpenQasmVersion {
    #[must_use]
    pub const fn major(self) -> u8 {
        3
    }

    #[must_use]
    pub const fn minor(self) -> u8 {
        match self {
            Self::V3_0 => 0,
            Self::V3_1 => 1,
        }
    }

    #[must_use]
    pub const fn source_text(self) -> &'static str {
        match self {
            Self::V3_0 => "OPENQASM 3.0;",
            Self::V3_1 => "OPENQASM 3.1;",
        }
    }

    #[must_use]
    pub const fn from_major_minor(
        major: u32,
        minor: u32,
    ) -> Option<Self> {
        match (major, minor) {
            (3, 0) => Some(Self::V3_0),
            (3, 1) => Some(Self::V3_1),
            _ => None,
        }
    }
}

impl fmt::Display for OpenQasmVersion {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}

/// `OPENQASM 3.x;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionDeclaration {
    pub span: SourceSpan,
    pub version: OpenQasmVersion,
}

impl VersionDeclaration {
    /// Constructor used by the parser.
    ///
    /// Keeping this constructor accepts numeric major/minor components and
    /// centralizes version conversion inside the AST contract.
    pub fn new(
        span: SourceSpan,
        major: u32,
        minor: u32,
    ) -> AstResult<Self> {
        let version =
            OpenQasmVersion::from_major_minor(major, minor)
                .ok_or(AstError::InvalidVersion {
                    major,
                    minor,
                })?;

        Ok(Self {
            span,
            version,
        })
    }

    #[must_use]
    pub const fn from_version(
        span: SourceSpan,
        version: OpenQasmVersion,
    ) -> Self {
        Self {
            span,
            version,
        }
    }

    #[must_use]
    pub const fn major(&self) -> u8 {
        self.version.major()
    }

    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.version.minor()
    }
}

impl Spanned for VersionDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for VersionDeclaration {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::VersionDeclaration
    }
}

// ============================================================================
// Program
// ============================================================================

/// Complete OpenQASM source program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub span: SourceSpan,
    pub version: Option<VersionDeclaration>,
    pub statements: Vec<Statement>,
}

impl Program {
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

    #[must_use]
    pub fn version(&self) -> Option<OpenQasmVersion> {
        self.version.as_ref().map(|v| v.version)
    }

    #[must_use]
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}

impl Spanned for Program {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for Program {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Program
    }
}

// ============================================================================
// Identifier
// ============================================================================

/// Source-level OpenQASM identifier.
///
/// Lexical validity is established by the lexer. Semantic restrictions such
/// as reserved names are established by validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    pub span: SourceSpan,
    pub name: String,
}

impl Identifier {
    #[must_use]
    pub fn new(
        span: SourceSpan,
        name: impl Into<String>,
    ) -> Self {
        Self {
            span,
            name: name.into(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl Spanned for Identifier {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for Identifier {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Identifier
    }
}

// ============================================================================
// Literals
// ============================================================================

/// Integer radix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerRadix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl IntegerRadix {
    #[must_use]
    pub const fn radix(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hexadecimal => 16,
        }
    }
}

/// Compatibility alias used by newer AST code.
pub type IntegerBase = IntegerRadix;

/// Integer literal.
///
/// The original spelling is retained. The AST must not prematurely convert
/// arbitrary literals to machine integers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub span: SourceSpan,
    pub raw: String,
    pub radix: IntegerRadix,
}

impl IntegerLiteral {
    #[must_use]
    pub fn new(
        span: SourceSpan,
        raw: impl Into<String>,
        radix: IntegerRadix,
    ) -> Self {
        Self {
            span,
            raw: raw.into(),
            radix,
        }
    }

    #[must_use]
    pub fn value_u128(&self) -> Option<u128> {
        let digits = match self.radix {
            IntegerRadix::Binary => self.raw.strip_prefix("0b"),
            IntegerRadix::Octal => self.raw.strip_prefix("0o"),
            IntegerRadix::Decimal => Some(self.raw.as_str()),
            IntegerRadix::Hexadecimal => {
                self.raw.strip_prefix("0x")
            }
        }?;

        u128::from_str_radix(
            digits.replace('_', "").as_str(),
            self.radix.radix(),
        )
        .ok()
    }
}

impl Spanned for IntegerLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for IntegerLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
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

impl AstNode for FloatLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
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

impl AstNode for ImaginaryLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
    }
}

/// String literal.
///
/// The AST preserves the decoded logical value and the source span. It does
/// not retain the original quote style because that is lexical presentation.
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

impl AstNode for StringLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
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

impl AstNode for BitStringLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
    }
}

/// Duration unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationUnit {
    Dt,
    S,
    Ms,
    Us,
    Ns,
    Ps,
    Fs,
}

impl DurationUnit {
    #[must_use]
    pub const fn source_text(self) -> &'static str {
        match self {
            Self::Dt => "dt",
            Self::S => "s",
            Self::Ms => "ms",
            Self::Us => "us",
            Self::Ns => "ns",
            Self::Ps => "ps",
            Self::Fs => "fs",
        }
    }
}

/// Duration literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationLiteral {
    pub span: SourceSpan,
    pub value: String,
    pub unit: DurationUnit,
}

impl Spanned for DurationLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for DurationLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
    }
}

/// Stretch literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StretchLiteral {
    pub span: SourceSpan,
    pub raw: String,
}

impl Spanned for StretchLiteral {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for StretchLiteral {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
    }
}

/// Literal expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool {
        span: SourceSpan,
        value: bool,
    },

    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Imaginary(ImaginaryLiteral),
    BitString(BitStringLiteral),
    String(StringLiteral),
    Duration(DurationLiteral),
    Stretch(StretchLiteral),
}

impl Spanned for Literal {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Bool { span, .. } => *span,
            Self::Integer(v) => v.span(),
            Self::Float(v) => v.span(),
            Self::Imaginary(v) => v.span(),
            Self::BitString(v) => v.span(),
            Self::String(v) => v.span(),
            Self::Duration(v) => v.span(),
            Self::Stretch(v) => v.span(),
        }
    }
}

impl AstNode for Literal {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Literal
    }
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitNot,
}

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

    BitAnd,
    BitOr,
    BitXor,

    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal {
        span: SourceSpan,
        value: Literal,
    },

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

    Cast {
        span: SourceSpan,
        target: TypeSpecifier,
        expression: Box<Expression>,
    },

    Index(IndexExpression),

    Slice {
        span: SourceSpan,
        expression: Box<Expression>,
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },

    FunctionCall {
        span: SourceSpan,
        function: Identifier,
        arguments: Vec<Expression>,
    },

    ArrayLiteral {
        span: SourceSpan,
        elements: Vec<Expression>,
    },

    DurationOf {
        span: SourceSpan,
        body: Box<StatementOrScope>,
    },

    QuantumCall(QuantumCallExpression),

    Parenthesized {
        span: SourceSpan,
        expression: Box<Expression>,
    },
}

impl Spanned for Expression {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Literal { span, .. } => *span,
            Self::Identifier(v) => v.span(),
            Self::Unary { span, .. } => *span,
            Self::Binary { span, .. } => *span,
            Self::Cast { span, .. } => *span,
            Self::Index(v) => v.span(),
            Self::Slice { span, .. } => *span,
            Self::FunctionCall { span, .. } => *span,
            Self::ArrayLiteral { span, .. } => *span,
            Self::DurationOf { span, .. } => *span,
            Self::QuantumCall(v) => v.span(),
            Self::Parenthesized { span, .. } => *span,
        }
    }
}

impl AstNode for Expression {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Expression
    }
}

// ============================================================================
// Indexing
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum IndexExpression {
    Single {
        span: SourceSpan,
        index: Box<Expression>,
    },

    Range {
        span: SourceSpan,
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },

    Set {
        span: SourceSpan,
        values: Vec<Expression>,
    },

    Concatenation {
        span: SourceSpan,
        values: Vec<Expression>,
    },
}

impl Spanned for IndexExpression {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Single { span, .. } => *span,
            Self::Range { span, .. } => *span,
            Self::Set { span, .. } => *span,
            Self::Concatenation { span, .. } => *span,
        }
    }
}

impl AstNode for IndexExpression {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Expression
    }
}

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarType {
    Bool,
    Bit,
    Int {
        width: u32,
    },
    UInt {
        width: u32,
    },
    Float {
        width: u32,
    },
    Angle {
        width: Option<u32>,
    },
    Complex {
        width: Option<u32>,
    },
    Duration,
    Stretch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuantumType {
    Qubit {
        size: Option<Expression>,
    },

    HardwareQubit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    Scalar(ScalarType),

    Quantum(QuantumType),

    Array {
        element: Box<TypeSpecifier>,
        dimensions: Vec<Expression>,
    },

    Void,
}

impl Spanned for TypeSpecifier {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Scalar(_) => {
                // Type spans are carried by the declaration using them.
                SourceSpan::default()
            }

            Self::Quantum(_) => {
                SourceSpan::default()
            }

            Self::Array { .. } => {
                SourceSpan::default()
            }

            Self::Void => {
                SourceSpan::default()
            }
        }
    }
}

impl AstNode for TypeSpecifier {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::TypeSpecifier
    }
}

/// Qualifiers such as `const`, `input`, `output`, and `readonly`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeQualifier {
    Const,
    Input,
    Output,
    Readonly,
}

// ============================================================================
// Designators
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Designator {
    pub span: SourceSpan,
    pub expression: Expression,
}

impl Spanned for Designator {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Quantum operands
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalQubit {
    Identifier {
        span: SourceSpan,
        index: u64,
    },
}

impl Spanned for PhysicalQubit {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Identifier { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GateOperand {
    Identifier(Identifier),

    Indexed {
        span: SourceSpan,
        identifier: Identifier,
        index: IndexExpression,
    },

    Physical(PhysicalQubit),

    Concatenation {
        span: SourceSpan,
        operands: Vec<GateOperand>,
    },
}

impl Spanned for GateOperand {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Identifier(v) => v.span(),
            Self::Indexed { span, .. } => *span,
            Self::Physical(v) => v.span(),
            Self::Concatenation { span, .. } => *span,
        }
    }
}

// ============================================================================
// Gate modifiers
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum GateModifier {
    Ctrl {
        span: SourceSpan,
        count: Option<Expression>,
    },

    NegCtrl {
        span: SourceSpan,
        count: Option<Expression>,
    },

    Inv {
        span: SourceSpan,
    },

    Pow {
        span: SourceSpan,
        exponent: Expression,
    },
}

impl Spanned for GateModifier {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Ctrl { span, .. } => *span,
            Self::NegCtrl { span, .. } => *span,
            Self::Inv { span } => *span,
            Self::Pow { span, .. } => *span,
        }
    }
}

// ============================================================================
// Gate invocation
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct GateCall {
    pub span: SourceSpan,
    pub modifiers: Vec<GateModifier>,
    pub name: Identifier,
    pub parameters: Vec<Expression>,
    pub operands: Vec<GateOperand>,
}

impl Spanned for GateCall {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Quantum declarations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub quantum_type: QuantumType,
    pub designator: Option<Designator>,
}

impl Spanned for QuantumDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for QuantumDeclaration {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Declaration
    }
}

// ============================================================================
// Classical declarations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalDeclaration {
    pub span: SourceSpan,
    pub qualifiers: Vec<TypeQualifier>,
    pub type_specifier: TypeSpecifier,
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

impl Spanned for ClassicalDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl AstNode for ClassicalDeclaration {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Declaration
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    pub span: SourceSpan,
    pub type_specifier: TypeSpecifier,
    pub name: Identifier,
    pub initializer: Expression,
}

impl Spanned for ConstDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub span: SourceSpan,
    pub declaration: ClassicalDeclaration,
}

impl Spanned for VariableDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// I/O declarations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct IoDeclaration {
    pub span: SourceSpan,
    pub qualifier: TypeQualifier,
    pub type_specifier: TypeSpecifier,
    pub name: Identifier,
    pub initializer: Option<Expression>,
}

impl Spanned for IoDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Legacy OpenQASM 2-style declarations
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OldStyleDeclarationKind {
    Qreg,
    Creg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OldStyleDeclaration {
    pub span: SourceSpan,
    pub kind: OldStyleDeclarationKind,
    pub name: Identifier,
    pub size: Expression,
}

impl Spanned for OldStyleDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Alias
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct AliasDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub operands: Vec<GateOperand>,
}

impl Spanned for AliasDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Gate definitions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub type_specifier: Option<TypeSpecifier>,
}

impl Spanned for ArgumentDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GateDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub parameters: Vec<ArgumentDefinition>,
    pub qubits: Vec<ArgumentDefinition>,
    pub body: Vec<Statement>,
}

impl Spanned for GateDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Subroutines
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnSignature {
    pub span: SourceSpan,
    pub type_specifier: Option<TypeSpecifier>,
}

impl Spanned for ReturnSignature {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub arguments: Vec<ArgumentDefinition>,
    pub return_signature: Option<ReturnSignature>,
    pub body: StatementOrScope,
}

impl Spanned for SubroutineDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Extern
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ExternArgument {
    pub span: SourceSpan,
    pub type_specifier: TypeSpecifier,
}

impl Spanned for ExternArgument {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternDeclaration {
    pub span: SourceSpan,
    pub name: Identifier,
    pub arguments: Vec<ExternArgument>,
    pub return_signature: Option<ReturnSignature>,
}

impl Spanned for ExternDeclaration {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Measurement
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct MeasureExpression {
    pub span: SourceSpan,
    pub operand: GateOperand,
}

impl Spanned for MeasureExpression {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementStatement {
    pub span: SourceSpan,
    pub expression: MeasureExpression,
    pub destination: Option<GateOperand>,
}

impl Spanned for MeasurementStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasureAssignmentStatement {
    pub span: SourceSpan,
    pub destination: GateOperand,
    pub measurement: MeasureExpression,
}

impl Spanned for MeasureAssignmentStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Quantum operations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ResetStatement {
    pub span: SourceSpan,
    pub operand: GateOperand,
}

impl Spanned for ResetStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BarrierStatement {
    pub span: SourceSpan,
    pub operands: Vec<GateOperand>,
}

impl Spanned for BarrierStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DelayStatement {
    pub span: SourceSpan,
    pub duration: Expression,
    pub operands: Vec<GateOperand>,
}

impl Spanned for DelayStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Assignment
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentValue {
    Expression(Expression),
    Measurement(MeasureExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    pub span: SourceSpan,
    pub target: Expression,
    pub operator: AssignmentOperator,
    pub value: AssignmentValue,
}

impl Spanned for AssignmentStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

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

// ============================================================================
// Control flow
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum StatementOrScope {
    Statement(Statement),

    Scope(Scope),
}

impl Spanned for StatementOrScope {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Statement(v) => v.span(),
            Self::Scope(v) => v.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub span: SourceSpan,
    pub statements: Vec<Statement>,
}

impl Spanned for Scope {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlStatement {
    pub span: SourceSpan,
    pub condition: Expression,
    pub body: StatementOrScope,
}

impl Spanned for ControlStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub span: SourceSpan,
    pub condition: Expression,
    pub then_branch: StatementOrScope,
    pub else_branch: Option<StatementOrScope>,
}

impl Spanned for IfStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForIterable {
    Range {
        span: SourceSpan,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
    },

    Set {
        span: SourceSpan,
        values: Vec<Expression>,
    },

    Expression(Expression),
}

impl Spanned for ForIterable {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Range { span, .. } => *span,
            Self::Set { span, .. } => *span,
            Self::Expression(v) => v.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub span: SourceSpan,
    pub variable: Identifier,
    pub iterable: ForIterable,
    pub body: StatementOrScope,
}

impl Spanned for ForStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub span: SourceSpan,
    pub condition: Expression,
    pub body: StatementOrScope,
}

impl Spanned for WhileStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub span: SourceSpan,
    pub labels: Vec<Expression>,
    pub body: Vec<Statement>,
}

impl Spanned for SwitchCase {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
    pub span: SourceSpan,
    pub expression: Expression,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Scope>,
}

impl Spanned for SwitchStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnValue {
    Expression(Expression),
    Measurement(MeasureExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub span: SourceSpan,
    pub value: Option<ReturnValue>,
}

impl Spanned for ReturnStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakStatement {
    pub span: SourceSpan,
}

impl Spanned for BreakStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueStatement {
    pub span: SourceSpan,
}

impl Spanned for ContinueStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

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

// ============================================================================
// Timing
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct BoxStatement {
    pub span: SourceSpan,
    pub duration: Option<Expression>,
    pub body: Vec<Statement>,
}

impl Spanned for BoxStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Include
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeStatement {
    pub span: SourceSpan,
    pub path: String,
}

impl Spanned for IncludeStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Calibration
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationGrammarStatement {
    pub span: SourceSpan,
    pub name: String,
}

impl Spanned for CalibrationGrammarStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Compatibility name used by the current parser.
pub type DefcalGrammarStatement = CalibrationGrammarStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct DefcalDefinition {
    pub span: SourceSpan,
    pub name: Identifier,
    pub parameters: Vec<ArgumentDefinition>,
    pub operands: Vec<GateOperand>,
    pub body: Vec<Statement>,
}

impl Spanned for DefcalDefinition {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationStatement {
    pub span: SourceSpan,
    pub body: String,
}

impl Spanned for CalibrationStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PulseStatement {
    pub span: SourceSpan,
    pub body: String,
}

impl Spanned for PulseStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Annotations and pragmas
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub span: SourceSpan,
    pub text: String,
}

impl Spanned for Annotation {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedStatement {
    pub span: SourceSpan,
    pub annotations: Vec<Annotation>,
    pub statement: Box<Statement>,
}

impl Spanned for AnnotatedStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationStatement {
    pub span: SourceSpan,
    pub annotation: Annotation,
}

impl Spanned for AnnotationStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PragmaStatement {
    pub span: SourceSpan,
    pub text: String,
}

impl Spanned for PragmaStatement {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Quantum call expressions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCallExpression {
    pub span: SourceSpan,
    pub name: Identifier,
    pub parameters: Vec<Expression>,
    pub operands: Vec<GateOperand>,
}

impl Spanned for QuantumCallExpression {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// ============================================================================
// Statement enum
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Include(IncludeStatement),

    DefcalGrammar(CalibrationGrammarStatement),

    QuantumDeclaration(QuantumDeclaration),

    ClassicalDeclaration(ClassicalDeclaration),

    ConstDeclaration(ConstDeclaration),

    IoDeclaration(IoDeclaration),

    OldStyleDeclaration(OldStyleDeclaration),

    AliasDeclaration(AliasDeclaration),

    GateDefinition(GateDefinition),

    SubroutineDefinition(SubroutineDefinition),

    ExternDeclaration(ExternDeclaration),

    GateCall(GateCall),

    Measurement(MeasurementStatement),

    MeasureAssignment(MeasureAssignmentStatement),

    Reset(ResetStatement),

    Barrier(BarrierStatement),

    Delay(DelayStatement),

    Box(BoxStatement),

    Assignment(AssignmentStatement),

    VariableDeclaration(VariableDeclaration),

    Expression(ExpressionStatement),

    If(IfStatement),

    For(ForStatement),

    While(WhileStatement),

    Switch(SwitchStatement),

    Return(ReturnStatement),

    Break(BreakStatement),

    Continue(ContinueStatement),

    Let(LetStatement),

    Defcal(DefcalDefinition),

    Calibration(CalibrationStatement),

    Pulse(PulseStatement),

    Pragma(PragmaStatement),

    Annotation(AnnotationStatement),
}

impl Spanned for Statement {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Include(v) => v.span(),
            Self::DefcalGrammar(v) => v.span(),
            Self::QuantumDeclaration(v) => v.span(),
            Self::ClassicalDeclaration(v) => v.span(),
            Self::ConstDeclaration(v) => v.span(),
            Self::IoDeclaration(v) => v.span(),
            Self::OldStyleDeclaration(v) => v.span(),
            Self::AliasDeclaration(v) => v.span(),
            Self::GateDefinition(v) => v.span(),
            Self::SubroutineDefinition(v) => v.span(),
            Self::ExternDeclaration(v) => v.span(),
            Self::GateCall(v) => v.span(),
            Self::Measurement(v) => v.span(),
            Self::MeasureAssignment(v) => v.span(),
            Self::Reset(v) => v.span(),
            Self::Barrier(v) => v.span(),
            Self::Delay(v) => v.span(),
            Self::Box(v) => v.span(),
            Self::Assignment(v) => v.span(),
            Self::VariableDeclaration(v) => v.span(),
            Self::Expression(v) => v.span(),
            Self::If(v) => v.span(),
            Self::For(v) => v.span(),
            Self::While(v) => v.span(),
            Self::Switch(v) => v.span(),
            Self::Return(v) => v.span(),
            Self::Break(v) => v.span(),
            Self::Continue(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Defcal(v) => v.span(),
            Self::Calibration(v) => v.span(),
            Self::Pulse(v) => v.span(),
            Self::Pragma(v) => v.span(),
            Self::Annotation(v) => v.span(),
        }
    }
}

impl AstNode for Statement {
    fn node_kind(&self) -> AstNodeKind {
        AstNodeKind::Statement
    }
}

// ============================================================================
// Compatibility helpers
// ============================================================================

/// Convenience constructor for a source-level boolean literal.
#[must_use]
pub fn boolean_literal(
    span: SourceSpan,
    value: bool,
) -> Literal {
    Literal::Bool {
        span,
        value,
    }
}

/// Convenience constructor for an identifier expression.
#[must_use]
pub fn identifier_expression(
    identifier: Identifier,
) -> Expression {
    Expression::Identifier(identifier)
}

/// Convenience constructor for an integer expression.
#[must_use]
pub fn integer_expression(
    literal: IntegerLiteral,
) -> Expression {
    Expression::Literal {
        span: literal.span,
        value: Literal::Integer(literal),
    }
}