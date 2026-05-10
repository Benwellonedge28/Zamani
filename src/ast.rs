//! Zenith Abstract Syntax Tree (AST)
//!
//! This module defines the Abstract Syntax Tree (AST) for the Zenith programming language.
//! The AST is the output of the parsing phase and represents the hierarchical structure
//! of the source code, independent of its textual representation. It serves as the input
//! for subsequent compilation phases like semantic analysis and IR generation.

use crate::tokens::{Token, TokenType}; // Keep TokenType for pattern matching within AST nodes
use crate::source_map::Span;
use std::collections::HashMap;

/// Represents the entire Zenith program as a sequence of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span, // Span covering the entire program
}

/// Represents a single statement in the Zenith language.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// let <name>: <TypeExpr> = <expr>;
    Let(Span, String, Option<TypeExpr>, Expression),
    /// return <expr>;
    Return(Span, Expression),
    /// <expr>; (expression used as a statement, its value is discarded)
    Expression(Expression),
    /// fn <name>(<params>): <ReturnType> <body>
    Function(
        Span,
        String, // Function name
        Vec<Parameter>, // Parameters
        Option<TypeExpr>, // Optional return type annotation
        Box<Expression>, // Function body (typically a block expression)
    ),
    /// quantum_circuit <name> <body> (a block of quantum operations)
    QuantumCircuit(Span, String, Box<Expression>),
    /// nano_agent <name> <body> (a block defining nano-agent behavior/blueprint)
    NanoAgent(Span, String, Box<Expression>),
    /// remember <name> = <expr>; (Sankofa memory write)
    SankofaMemory(Span, String, Expression),
    /// type <name> = <type_expression>; (Custom type definition)
    TypeDeclaration(Span, String, TypeExpr),
    /// effect <name>; (Algebraic effect declaration)
    EffectDeclaration(Span, String),
    /// language <name> grammar <grammar_expr>; (Meta-compilation/language extension)
    LanguageDeclaration(Span, String, Expression),
    /// while <condition> <block>
    While(Span, Box<Expression>, Box<Expression>),
    /// for <iterator_var> in <iterable> <block>
    For(Span, Identifier, Box<Expression>, Box<Expression>),
    /// break;
    Break(Span),
    /// continue;
    Continue(Span),
    /// match <expression> { <case> => <body_expr>, ... }
    Match(Span, Box<Expression>, Vec<MatchCase>),
    /// unsafe!(evas: "proof") { ... } (an unsafe block that requires an EVAS proof)
    Unsafe(Span, Option<String>, Box<Expression>), // Option<String> for the proof string
}

/// Represents a parameter in a function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub span: Span,
    pub name: String,
    pub param_type: TypeExpr,
    pub is_linear: bool, // Added for linear types
    pub is_affine: bool, // Added for affine types
}

/// Represents a single case in a match expression.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub span: Span,
    pub pattern: Expression, // The pattern to match against
    pub body: Expression,    // The expression to execute if the pattern matches
}

/// Represents an expression in the Zenith language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// <literal>
    Literal(Literal),
    /// <identifier>
    Identifier(Identifier),
    /// <prefix_operator><expression> (e.g., !true, -1)
    Prefix(Span, TokenType, Box<Expression>),
    /// <expression><infix_operator><expression> (e.g., 1 + 2, a == b)
    Infix(Span, Box<Expression>, TokenType, Box<Expression>),
    /// if <condition> <then_block> else <else_block>
    If(Span, Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    /// { <statement>; <statement>; ... <expression> }
    Block(Span, Vec<Statement>),
    /// <function_name>(<arg1>, <arg2>, ...)
    Call(Span, Box<Expression>, Vec<Expression>),
    /// <array>[<index>]
    Index(Span, Box<Expression>, Box<Expression>),
    /// <object>.<member>
    MemberAccess(Span, Box<Expression>, Identifier),
    /// Used for quantum gate applications, etc. (e.g., H(q0), CNOT(q0, q1))
    QuantumGateApplication(Span, String, Vec<Expression>),
    /// Used for nano-agent specific actions (e.g., move_to(agent, target_coords))
    NanoAction(Span, String, Vec<Expression>),
    /// Used for MTS-specific operations (e.g., timeline_slice.load(timestamp))
    MtsOperation(Span, String, Vec<Expression>),
    /// Used for performing algebraic effects (e.g., perform Read(addr))
    PerformEffect(Span, String, Vec<Expression>),
}

/// Represents a literal value in the Zenith language.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(String, Span),
    Float(String, Span),
    String(String, Span),
    Char(String, Span),
    Boolean(bool, Span),
    Quantum(String, Span), // e.g., |0>, |+>
    MTS(String, Span),     // e.g., mts[5]
}

/// Represents an identifier in the Zenith language.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(
    pub String, // The identifier name
    pub Span,
);

/// Represents a type expression in the Zenith language.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Base(Identifier), // e.g., `int`, `Qubit`, `MyStruct`
    Array(Box<TypeExpr>, Option<String>), // e.g., `[int]`, `QReg[N]` (where N is a literal string)
    FunctionType(Vec<TypeExpr>, Box<TypeExpr>), // e.g., `fn(int, bool) -> float`
    Tuple(Vec<TypeExpr>), // e.g., `(int, bool)`
    Generic(Identifier, Vec<TypeExpr>), // e.g., `List<T>`, `Superposition<Qubit>`
    Linear(Box<TypeExpr>), // e.g., `linear Qubit`
    Affine(Box<TypeExpr>), // e.g., `affine Handle`
    Effectful(Box<TypeExpr>, Vec<Identifier>), // e.g., `int with effects {Read, Write}`
    // Dependent types, e.g., `int where N > 0` (conceptual, parser/semantic would handle the 'where' clause)
    Dependent(Box<TypeExpr>, Box<Expression>), // e.g., `Vec<T> where T: Copy` or `int where (value > 0)`
    // Pi types for dependent function types: Π(x: A) → B
    PiType(String, Box<TypeExpr>, Box<TypeExpr>), // name, binder_type, return_type (conceptual)
    // Sigma types for dependent pair types: Σ(x: A) × B
    SigmaType(String, Box<TypeExpr>, Box<TypeExpr>), // name, first_type, second_type (conceptual)
    // Proof types for formal verification
    Proof(Box<TypeExpr>, Box<Expression>), // Type of thing being proven, the proof expression itself
    // Type families / Type classes
    TypeFamily(Identifier, Vec<TypeExpr>), // e.g., `Iterator<Item=T>`
    // Special Zenith types
    QuantumReg(Box<TypeExpr>, String), // Explicit QReg[N] where the type is Qubit, and String is N
    Superposition(Box<TypeExpr>), // Superposition<Qubit>
    Entangled(Box<TypeExpr>, Box<TypeExpr>), // Entangled<Qubit, Qubit>
    QMeasured(Box<TypeExpr>), // QMeasured<bool>
    NanoAgentType(Box<TypeExpr>), // NanoAgent<Blueprint>
    ArchaeveType(Box<TypeExpr>), // Archaeve<Data>
    MtsSlice(Box<TypeExpr>, Option<String>), // MtsSlice<Data, N>
    HistoryType(Box<TypeExpr>, Option<String>), // History<Data, years>
    ConsensusTrueType(Box<TypeExpr>), // ConsensusTrue<Proposal>
    InterMemoryType(Box<TypeExpr>, Box<TypeExpr>), // InterMemory<LangId, Data>

    // Placeholders for error recovery or unparsed types
    Error(Span),
}
