//! Zenith Abstract Syntax Tree
//!
//! Full AST covering every language construct: expressions, statements,
//! types, patterns, items, attributes.  Every node carries a `Span` so
//! the compiler can emit precise diagnostics.

use crate::lexer::TokenType;
use crate::source_map::Span;

// ─── Program ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Program {
    pub fn new(statements: Vec<Statement>, span: Span) -> Self {
        Program { statements, span }
    }
}

// ─── Statements ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Variable bindings
    Let(Span, String, Option<TypeExpr>, Expression),
    Const(Span, String, Option<TypeExpr>, Expression),
    // Control flow
    Return(Span, Expression),
    Break(Span),
    Continue(Span),
    While(Span, Expression, Expression),
    For(Span, Identifier, Expression, Expression),
    Match(Span, Expression, Vec<MatchCase>),
    // Definitions
    Function(Span, String, Vec<Parameter>, Option<TypeExpr>, Expression),
    Struct(Span, Identifier, Vec<TypeParameter>, Vec<StructField>),
    Enum(Span, Identifier, Vec<TypeParameter>, Vec<EnumVariant>),
    Trait(Span, Identifier, Vec<TypeParameter>, Vec<TraitItem>),
    Impl(Span, Option<Identifier>, TypeExpr, Vec<ImplItem>), // impl [Trait for] Type
    TypeAlias(Span, Identifier, Vec<TypeParameter>, TypeExpr),
    TypeDeclaration(Span, String, TypeExpr),
    Module(Span, String, Vec<Statement>),
    Import(Span, Vec<String>),
    Use(Span, UsePath),
    Class(Span, Identifier, Vec<Identifier>, Vec<ClassMember>),
    Interface(Span, Identifier, Vec<Identifier>, Vec<InterfaceMember>),
    // Zenith-native
    QuantumCircuit(Span, String, Expression),
    NanoAgent(Span, String, Expression),
    SankofaMemory(Span, String, Expression),
    EffectDeclaration(Span, Identifier),
    LanguageDeclaration(Span, String, String),
    // Safety & correctness
    Unsafe(Span, Option<Identifier>, Expression),
    Handle(Span, Identifier, Expression, Expression),
    // Expression statement
    Expression(Expression),
    // Wisdom (meta-declaration)
    Wisdom(Span, String, Expression),
}

// ─── Items within definitions ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Identifier,
    pub typ: TypeExpr,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub fields: EnumVariantKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<TypeExpr>),
    Struct(Vec<StructField>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitItem {
    pub name: Identifier,
    pub kind: TraitItemKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitItemKind {
    Method {
        params: Vec<Parameter>,
        ret: Option<TypeExpr>,
        default_body: Option<Expression>,
    },
    AssociatedType(Option<TypeExpr>),
    Const(TypeExpr, Option<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplItem {
    pub name: Identifier,
    pub kind: ImplItemKind,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImplItemKind {
    Method {
        params: Vec<Parameter>,
        ret: Option<TypeExpr>,
        body: Expression,
    },
    AssociatedType(TypeExpr),
    Const(TypeExpr, Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub name: Identifier,
    pub bounds: Vec<TypeBound>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeBound {
    Trait(Identifier),
    Lifetime(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsePath {
    pub segments: Vec<String>,
    pub kind: UseKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseKind {
    Single,
    Glob,
    Named(Vec<String>),
}

// ─── Match ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
    pub span: Span,
}

// ─── Patterns ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard(Span),
    Identifier(Identifier),
    Literal(Literal),
    Tuple(Span, Vec<Pattern>),
    Struct(Span, Identifier, Vec<(String, Pattern)>),
    Enum(Span, Identifier, Vec<Pattern>),
    Or(Span, Vec<Pattern>),
    Range(Span, Box<Pattern>, Box<Pattern>),
    Ref(Span, Box<Pattern>),
}

// ─── Parameters ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub typ: Option<TypeExpr>,
    pub default: Option<Expression>,
    pub is_self: bool,
    pub is_mutable: bool,
}

impl Parameter {
    pub fn simple(name: Identifier, typ: TypeExpr) -> Self {
        Parameter {
            name,
            typ: Some(typ),
            default: None,
            is_self: false,
            is_mutable: false,
        }
    }
}

// ─── Expressions ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Primitives
    Identifier(Identifier),
    Literal(Literal),
    // Operators
    Prefix(Span, TokenType, Box<Expression>),
    Infix(Span, Box<Expression>, TokenType, Box<Expression>),
    // Control flow expressions
    If(
        Span,
        Box<Expression>,
        Box<Expression>,
        Option<Box<Expression>>,
    ),
    Block(Span, Vec<Statement>),
    Match(Span, Box<Expression>, Vec<MatchCase>),
    Loop(Span, Box<Expression>),
    // Functions & closures
    Call(Span, Box<Expression>, Vec<Expression>),
    Lambda(Span, Vec<Parameter>, Box<Expression>),
    // Data
    Array(Span, Vec<Expression>),
    Tuple(Span, Vec<Expression>),
    Struct(Span, Identifier, Vec<(String, Expression)>), // struct literal
    Index(Span, Box<Expression>, Box<Expression>),
    Range(Span, Box<Expression>, Box<Expression>, bool), // inclusive flag
    // Access
    MemberAccess(Span, Box<Expression>, Identifier),
    MethodCall(Span, Box<Expression>, Identifier, Vec<Expression>),
    // Type operations
    Cast(Span, Box<Expression>, TypeExpr),
    TypeAscription(Span, Box<Expression>, TypeExpr),
    // Assignment
    Assign(Span, Box<Expression>, Box<Expression>),
    CompoundAssign(Span, Box<Expression>, TokenType, Box<Expression>),
    // Error handling
    Try(Span, Box<Expression>), // expr?
    TryCatch(Span, Box<Expression>, Vec<CatchArm>),
    // Async
    Await(Span, Box<Expression>),
    Async(Span, Box<Expression>),
    Spawn(Span, Box<Expression>),
    // Object creation
    New(Span, Identifier, Vec<Expression>),
    // Zenith-native
    QuantumOp(Span, String, Vec<Expression>),
    NanoOp(Span, String, Vec<Expression>),
    Recall(Span, Box<Expression>),
    Remember(Span, String, Box<Expression>),
    Learn(Span, Box<Expression>),
    Perform(Span, Box<Expression>),
    Zamani(Span, Box<Expression>),
    Sasa(Span, Box<Expression>),
    // Meta
    Macro(Span, String, Vec<Expression>),
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
            | Expression::Match(s, _, _)
            | Expression::Loop(s, _)
            | Expression::Call(s, _, _)
            | Expression::Lambda(s, _, _)
            | Expression::Array(s, _)
            | Expression::Tuple(s, _)
            | Expression::Struct(s, _, _)
            | Expression::Index(s, _, _)
            | Expression::Range(s, _, _, _)
            | Expression::MemberAccess(s, _, _)
            | Expression::MethodCall(s, _, _, _)
            | Expression::Cast(s, _, _)
            | Expression::TypeAscription(s, _, _)
            | Expression::Assign(s, _, _)
            | Expression::CompoundAssign(s, _, _, _)
            | Expression::Try(s, _)
            | Expression::TryCatch(s, _, _)
            | Expression::Await(s, _)
            | Expression::Async(s, _)
            | Expression::Spawn(s, _)
            | Expression::New(s, _, _)
            | Expression::QuantumOp(s, _, _)
            | Expression::NanoOp(s, _, _)
            | Expression::Recall(s, _)
            | Expression::Remember(s, _, _)
            | Expression::Learn(s, _)
            | Expression::Perform(s, _)
            | Expression::Zamani(s, _)
            | Expression::Sasa(s, _)
            | Expression::Macro(s, _, _) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchArm {
    pub error_type: Option<TypeExpr>,
    pub binding: Option<Identifier>,
    pub body: Expression,
    pub span: Span,
}

// ─── Literals ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    Char(char, Span),
    Null(Span),
    Unit(Span),
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
            | Literal::Unit(s)
            | Literal::Quantum(_, s)
            | Literal::Nano(_, s)
            | Literal::MTS(_, s) => s,
        }
    }
}

// ─── Identifiers ─────────────────────────────────────────────────────────────

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

// ─── Type Expressions ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    // Named types
    Identifier(Identifier),
    // Parameterized: Vec<T>, HashMap<K,V>
    Generic(Box<TypeExpr>, Vec<TypeExpr>),
    // Structural
    Tuple(Vec<TypeExpr>),
    Array(Box<TypeExpr>),
    Slice(Box<TypeExpr>),
    // Functions
    Function(Vec<TypeExpr>, Box<TypeExpr>),
    // References & pointers
    Reference(bool, Box<TypeExpr>), // mutable flag
    Pointer(bool, Box<TypeExpr>),   // mutable flag
    // Special
    Optional(Box<TypeExpr>),              // T?
    Result(Box<TypeExpr>, Box<TypeExpr>), // Result<T,E>
    Never,
    Unit,
    SelfType,
    // Zenith-specific
    Quantum(Box<TypeExpr>),
    Linear(Box<TypeExpr>),
    Affine(Box<TypeExpr>),
    Temporal(Box<TypeExpr>),
}

impl TypeExpr {
    pub fn name(&self) -> String {
        match self {
            TypeExpr::Identifier(id) => id.0.clone(),
            TypeExpr::Generic(base, args) => {
                let base_name = base.name();
                let args_str: Vec<String> = args.iter().map(|a| a.name()).collect();
                format!("{}<{}>", base_name, args_str.join(", "))
            }
            TypeExpr::Tuple(ts) => {
                let inner: Vec<String> = ts.iter().map(|t| t.name()).collect();
                format!("({})", inner.join(", "))
            }
            TypeExpr::Array(t) => format!("[{}]", t.name()),
            TypeExpr::Slice(t) => format!("&[{}]", t.name()),
            TypeExpr::Function(params, ret) => {
                let ps: Vec<String> = params.iter().map(|p| p.name()).collect();
                format!("fn({}) -> {}", ps.join(", "), ret.name())
            }
            TypeExpr::Reference(m, t) => {
                if *m {
                    format!("&mut {}", t.name())
                } else {
                    format!("&{}", t.name())
                }
            }
            TypeExpr::Pointer(m, t) => {
                if *m {
                    format!("*mut {}", t.name())
                } else {
                    format!("*const {}", t.name())
                }
            }
            TypeExpr::Optional(t) => format!("{}?", t.name()),
            TypeExpr::Result(ok, err) => format!("Result<{},{}>", ok.name(), err.name()),
            TypeExpr::Never => "!".into(),
            TypeExpr::Unit => "()".into(),
            TypeExpr::SelfType => "Self".into(),
            TypeExpr::Quantum(t) => format!("Quantum<{}>", t.name()),
            TypeExpr::Linear(t) => format!("Linear<{}>", t.name()),
            TypeExpr::Affine(t) => format!("Affine<{}>", t.name()),
            TypeExpr::Temporal(t) => format!("Temporal<{}>", t.name()),
        }
    }
}

// ─── Types (resolved) ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntWidth {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatWidth {
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Never,
    Bool,
    Char,
    Str,
    String,
    Int(IntWidth),
    UInt(IntWidth),
    Float(FloatWidth),
    Array(Box<Type>, Option<usize>),
    Slice(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Reference(bool, Box<Type>),
    Pointer(bool, Box<Type>),
    Named(String),
    Generic(String, Vec<Type>),
    Optional(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Quantum,
    Linear(Box<Type>),
    Affine(Box<Type>),
    Unknown,
}

impl Type {
    pub fn get_name(&self) -> String {
        match self {
            Type::Unit => "()".into(),
            Type::Never => "!".into(),
            Type::Bool => "Bool".into(),
            Type::Char => "Char".into(),
            Type::Str => "str".into(),
            Type::String => "String".into(),
            Type::Int(w) => match w {
                IntWidth::I8 => "i8",
                IntWidth::I16 => "i16",
                IntWidth::I32 => "i32",
                IntWidth::I64 => "i64",
                IntWidth::I128 => "i128",
                IntWidth::ISize => "isize",
            }
            .into(),
            Type::UInt(w) => match w {
                IntWidth::I8 => "u8",
                IntWidth::I16 => "u16",
                IntWidth::I32 => "u32",
                IntWidth::I64 => "u64",
                IntWidth::I128 => "u128",
                IntWidth::ISize => "usize",
            }
            .into(),
            Type::Float(w) => match w {
                FloatWidth::F32 => "f32",
                FloatWidth::F64 => "f64",
            }
            .into(),
            Type::Array(t, sz) => match sz {
                Some(n) => format!("[{}; {}]", t.get_name(), n),
                None => format!("[{}]", t.get_name()),
            },
            Type::Slice(t) => format!("&[{}]", t.get_name()),
            Type::Tuple(ts) => {
                let inner: Vec<String> = ts.iter().map(|t| t.get_name()).collect();
                format!("({})", inner.join(", "))
            }
            Type::Function(params, ret) => {
                let ps: Vec<String> = params.iter().map(|p| p.get_name()).collect();
                format!("fn({}) -> {}", ps.join(", "), ret.get_name())
            }
            Type::Reference(m, t) => {
                if *m {
                    format!("&mut {}", t.get_name())
                } else {
                    format!("&{}", t.get_name())
                }
            }
            Type::Pointer(m, t) => {
                if *m {
                    format!("*mut {}", t.get_name())
                } else {
                    format!("*const {}", t.get_name())
                }
            }
            Type::Named(n) => n.clone(),
            Type::Generic(n, args) => {
                let as_: Vec<String> = args.iter().map(|a| a.get_name()).collect();
                format!("{}<{}>", n, as_.join(", "))
            }
            Type::Optional(t) => format!("{}?", t.get_name()),
            Type::Result(ok, e) => format!("Result<{},{}>", ok.get_name(), e.get_name()),
            Type::Quantum => "Quantum".into(),
            Type::Linear(t) => format!("Linear<{}>", t.get_name()),
            Type::Affine(t) => format!("Affine<{}>", t.get_name()),
            Type::Unknown => "<unknown>".into(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_) | Type::Float(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Type::Float(_))
    }
}

// ─── Class / Interface (OOP layer) ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field {
        name: Identifier,
        typ: TypeExpr,
        visibility: Visibility,
        default: Option<Expression>,
    },
    Method {
        name: Identifier,
        params: Vec<Parameter>,
        ret: Option<TypeExpr>,
        body: Expression,
        visibility: Visibility,
        is_static: bool,
        is_virtual: bool,
    },
    Constructor {
        params: Vec<Parameter>,
        body: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceMember {
    Method {
        name: Identifier,
        params: Vec<Parameter>,
        ret: Option<TypeExpr>,
        default_body: Option<Expression>,
    },
    Property {
        name: Identifier,
        typ: TypeExpr,
    },
}

// ─── Extra AST nodes (used by spec modules) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ProveAttribute {
    pub invariants: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvariantBlock {
    pub conditions: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostCondition {
    pub expr: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EthicalAttribute {
    pub rules: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SovereignEntityDecl {
    pub name: Identifier,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParadigmBlock {
    pub paradigm: String,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActorSpawn {
    pub actor_type: Identifier,
    pub args: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextOfExpr {
    pub target: Box<Expression>,
    pub context: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryOmniState {
    pub query: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaTransformDirective {
    pub name: String,
    pub args: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LanguageDialectDecl {
    pub name: String,
    pub version: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WisdomDecl {
    pub name: Identifier,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsensusExpr {
    pub proposal: Box<Expression>,
    pub validators: Vec<Expression>,
    pub span: Span,
}
