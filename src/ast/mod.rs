//! Zenith UMC Abstract Syntax Tree
//!
//! All AST node types for the Zenith multi-paradigm compiler.

use crate::lexer::TokenType;
use crate::source_map::Span;

pub use crate::compiler_types::{AccessModifier, FloatWidth, IntWidth, MethodModifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

impl Default for Program {
    fn default() -> Self {
        Program::new()
    }
}

/// Source type annotation (before semantic resolution).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeExpr {
    Identifier(Identifier),
    Array(Box<TypeExpr>),
    Tuple(Vec<TypeExpr>),
    Generic(Box<TypeExpr>, Vec<TypeExpr>),
    Function(Vec<TypeExpr>, Box<TypeExpr>),
    Reference(Box<TypeExpr>, bool), // bool = mutable
    Linear(Box<TypeExpr>),
    Affine(Box<TypeExpr>),
    Effectful(Box<TypeExpr>, Vec<Identifier>),
    Quantum(Box<TypeExpr>),
    Nano(Box<TypeExpr>),
    MTS(Box<TypeExpr>),
    Sankofa(Box<TypeExpr>),
    DependentPi(Box<Identifier>, Box<TypeExpr>, Box<TypeExpr>),
    DependentSigma(Box<Identifier>, Box<TypeExpr>, Box<TypeExpr>),
    Universe(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Span, String, Option<TypeExpr>, Expression),
    Return(Span, Expression),
    Expression(Expression),
    Function(Span, String, Vec<Parameter>, Option<TypeExpr>, Expression),
    QuantumCircuit(Span, String, Expression),
    NanoAgent(Span, String, Expression),
    SankofaMemory(Span, String, Expression),
    TypeDeclaration(Span, String, TypeExpr),
    EffectDeclaration(Span, Identifier),
    LanguageDeclaration(Span, String, String),
    While(Span, Expression, Expression),
    For(Span, Identifier, Expression, Expression),
    Break(Span),
    Continue(Span),
    Match(Span, Expression, Vec<MatchCase>),
    Unsafe(Span, Option<Identifier>, Expression),
    Handle(Span, Identifier, Expression, Expression),
    Class(Span, Identifier, Vec<Identifier>, Vec<ClassMember>),
    Interface(Span, Identifier, Vec<Identifier>, Vec<InterfaceMember>),
    Import(Span, Vec<String>),
    Module(Span, String, Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Expression,
    pub body: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub typ: Option<TypeExpr>,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Span, TokenType, Box<Expression>),
    Infix(Span, Box<Expression>, TokenType, Box<Expression>),
    If(
        Span,
        Box<Expression>,
        Box<Expression>,
        Option<Box<Expression>>,
    ),
    Block(Span, Vec<Statement>),
    Call(Span, Box<Expression>, Vec<Expression>),
    Index(Span, Box<Expression>, Box<Expression>),
    MemberAccess(Span, Box<Expression>, Identifier),
    Lambda(Span, Vec<Parameter>, Box<Expression>),
    Assign(Span, Box<Expression>, Box<Expression>),
    Tuple(Span, Vec<Expression>),
    Array(Span, Vec<Expression>),
    Range(Span, Box<Expression>, Box<Expression>, bool), // inclusive
    Cast(Span, Box<Expression>, TypeExpr),
    Await(Span, Box<Expression>),
    Spawn(Span, Box<Expression>),
    QuantumOp(Span, String, Vec<Expression>),
    NanoOp(Span, String, Vec<Expression>),
    Recall(Span, Box<Expression>),           // Sankofa recall
    Remember(Span, String, Box<Expression>), // Sankofa store
    New(Span, Identifier, Vec<Expression>),
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Identifier(id) => id.span(),
            Expression::Literal(lit) => lit.span(),
            Expression::Prefix(s, _, _)
            | Expression::Infix(s, _, _, _)
            | Expression::If(s, _, _, _)
            | Expression::Block(s, _)
            | Expression::Call(s, _, _)
            | Expression::Index(s, _, _)
            | Expression::MemberAccess(s, _, _)
            | Expression::Lambda(s, _, _)
            | Expression::Assign(s, _, _)
            | Expression::Tuple(s, _)
            | Expression::Array(s, _)
            | Expression::Range(s, _, _, _)
            | Expression::Cast(s, _, _)
            | Expression::Await(s, _)
            | Expression::Spawn(s, _)
            | Expression::QuantumOp(s, _, _)
            | Expression::NanoOp(s, _, _)
            | Expression::Recall(s, _)
            | Expression::Remember(s, _, _)
            | Expression::New(s, _, _) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    Char(char, Span),
    Null(Span),
    Quantum(String, Span),
    Nano(String, Span),
    MTS(String, Span),
}

impl Literal {
    pub fn span(&self) -> &Span {
        match self {
            Literal::Integer(_, s)
            | Literal::Float(_, s)
            | Literal::String(_, s)
            | Literal::Boolean(_, s)
            | Literal::Char(_, s)
            | Literal::Null(s)
            | Literal::Quantum(_, s)
            | Literal::Nano(_, s)
            | Literal::MTS(_, s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String, pub Span);

impl Identifier {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Identifier(name.into(), span)
    }
    pub fn name(&self) -> &str {
        &self.0
    }
    pub fn span(&self) -> &Span {
        &self.1
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field(
        Span,
        AccessModifier,
        Identifier,
        TypeExpr,
        Option<Expression>,
    ),
    Method(
        Span,
        AccessModifier,
        Option<MethodModifier>,
        Identifier,
        Vec<Parameter>,
        Option<TypeExpr>,
        Expression,
        Vec<Identifier>, // effects
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceMember {
    Method(Span, Identifier, Vec<Parameter>, Option<TypeExpr>),
    DefaultMethod(
        Span,
        Identifier,
        Vec<Parameter>,
        Option<TypeExpr>,
        Expression,
    ),
}
