
//! Zenith Universal Meta-Compiler (UMC) Abstract Syntax Tree (AST)
//!
//! This module defines the data structures that represent the Abstract Syntax Tree (AST)
//! for Zenith programs. The AST is the output of the parsing phase and serves as the
//! primary input for subsequent compilation stages like semantic analysis, IR generation,
//! and optimization.

use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// Represents a type as it appears in the source code (before semantic resolution)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeExpr {
    Identifier(Identifier), // e.g., "int", "MyClass", "List"
    Array(Box<TypeExpr>),   // e.g., "List<int>" (simplified for now)
    // Extend for generic types (e.g., List<T>) later
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Span, String, Option<TypeExpr>, Expression), // let x: Type = expr;
    Return(Span, Expression),                           // return expr;
    Expression(Expression),                             // expr;
    Function(Span, String, Vec<Parameter>, Option<TypeExpr>, Expression), // fn name(params) -> return_type { body }
    QuantumCircuit(Span, String, Expression), // quantum circuit Name { body }
    NanoAgent(Span, String, Expression),      // nano agent Name { body }
    SankofaMemory(Span, String, Expression),  // remember name = expr;
    TypeDeclaration(Span, String, TypeExpr),  // type MyType = OtherType;
    EffectDeclaration(Span, Identifier),      // effect MyEffect;
    LanguageDeclaration(Span, String, String), // language "paradigm" "version";
    While(Span, Expression, Expression),      // while (cond) { body }
    For(Span, Identifier, Expression, Expression), // for item in iterable { body }
    Break(Span),
    Continue(Span),
    Match(Span, Expression, Vec<MatchCase>),  // match expr { case pattern -> body }
    Unsafe(Span, Option<Identifier>, Expression), // unsafe!(evas:proof_id) { body }
    Handle(Span, Identifier, Expression, Expression), // handle Effect { body } with { handler }
    
    // --- OOP Additions ---
    Class(Span, Identifier, Vec<Identifier>, Vec<ClassMember>), // Name, Parent Classes/Interfaces, Members
    Interface(Span, Identifier, Vec<Identifier>, Vec<InterfaceMember>), // Name, Parent Interfaces, Members
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Expression,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub typ: Option<TypeExpr>, // Type annotation for parameter
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Span, TokenType, Box<Expression>),
    Infix(Span, Box<Expression>, TokenType, Box<Expression>),
    If(Span, Box<Expression>, Box<Expression>, Option<Box<Expression>>), // if (cond) { then } else { else }
    Block(Span, Vec<Statement>), // { statements... }
    Call(Span, Box<Expression>, Vec<Expression>), // func(args...)
    Index(Span, Box<Expression>, Box<Expression>), // array[index]
    MemberAccess(Span, Box<Expression>, Identifier), // object.member

    // --- OOP Additions ---
    NewInstance(Span, Identifier, Vec<Expression>), // Class Name, Constructor Arguments
    MethodCall(Span, Box<Expression>, Identifier, Vec<Expression>), // Object, Method Name, Arguments
    FieldAccess(Span, Box<Expression>, Identifier), // Object, Field Name
    This(Span), // 'this' keyword
    Super(Span), // 'super' keyword
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(
    pub String,
    pub Span,
);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(String, Span),
    Float(String, Span),
    String(String, Span),
    Boolean(bool, Span),
    Char(char, Span),
    Quantum(String, Span), // e.g. |0⟩
    Nano(String, Span),    // e.g. @atom(blueprint_id)
    MTS(String, Span),     // e.g. mts[timestamp]
}

// --- OOP Additions ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field(Span, AccessModifier, Identifier, TypeExpr, Option<Expression>), // Modifier, Name, Type, Initializer
    Method(Span, AccessModifier, Option<MethodModifier>, Identifier, Vec<Parameter>, Option<TypeExpr>, Expression, Vec<Identifier>), // Modifier, Name, Params, Return Type, Body, Effects
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceMember {
    MethodSignature(Span, Identifier, Vec<Parameter>, Option<TypeExpr>, Vec<Identifier>), // Name, Params, Return Type, Effects (no body)
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodModifier {
    Override,
    Virtual,
    Abstract,
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Identifier(i) => i.1.clone(),
            Expression::Literal(l) => match l {
                Literal::Integer(_, s) => s.clone(),
                Literal::Float(_, s) => s.clone(),
                Literal::String(_, s) => s.clone(),
                Literal::Boolean(_, s) => s.clone(),
                Literal::Char(_, s) => s.clone(),
                Literal::Quantum(_, s) => s.clone(),
                Literal::Nano(_, s) => s.clone(),
                Literal::MTS(_, s) => s.clone(),
            },
            Expression::Prefix(s, _, _) => s.clone(),
            Expression::Infix(s, _, _, _) => s.clone(),
            Expression::If(s, _, _, _) => s.clone(),
            Expression::Block(s, _) => s.clone(),
            Expression::Call(s, _, _) => s.clone(),
            Expression::Index(s, _, _) => s.clone(),
            Expression::MemberAccess(s, _, _) => s.clone(),
            Expression::NewInstance(s, _, _) => s.clone(),
            Expression::MethodCall(s, _, _, _) => s.clone(),
            Expression::FieldAccess(s, _, _) => s.clone(),
            Expression::This(s) => s.clone(),
            Expression::Super(s) => s.clone(),
        }
    }
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Let(s, _, _, _) => s.clone(),
            Statement::Return(s, _) => s.clone(),
            Statement::Expression(e) => e.span(),
            Statement::Function(s, _, _, _, _) => s.clone(),
            Statement::QuantumCircuit(s, _, _) => s.clone(),
            Statement::NanoAgent(s, _, _) => s.clone(),
            Statement::SankofaMemory(s, _, _) => s.clone(),
            Statement::TypeDeclaration(s, _, _) => s.clone(),
            Statement::EffectDeclaration(s, _) => s.clone(),
            Statement::LanguageDeclaration(s, _, _) => s.clone(),
            Statement::While(s, _, _) => s.clone(),
            Statement::For(s, _, _, _) => s.clone(),
            Statement::Break(s) => s.clone(),
            Statement::Continue(s) => s.clone(),
            Statement::Match(s, _, _) => s.clone(),
            Statement::Unsafe(s, _, _) => s.clone(),
            Statement::Handle(s, _, _, _) => s.clone(),
            Statement::Class(s, _, _, _) => s.clone(),
            Statement::Interface(s, _, _, _) => s.clone(),
        }
    }
}
