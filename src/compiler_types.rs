//! Zamani UMC Compiler Types
//! Shared types used across all compiler phases.

use crate::source_map::Span;

/// Symbol identifier: name + source span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String, pub Span);

impl Identifier {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodModifier {
    Override,
    Virtual,
    Abstract,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntWidth {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatWidth {
    F32,
    F64,
    F128,
}

/// Fully-resolved Zamani type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Unit,
    Bool,
    Int(IntWidth),
    Float(FloatWidth),
    Char,
    Str,
    String,
    Array(Box<Type>, Option<usize>),
    Slice(Box<Type>),
    Tuple(Vec<Type>),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Ref(Box<Type>),
    MutRef(Box<Type>),
    Owned(Box<Type>),
    Linear(Box<Type>),
    Affine(Box<Type>),
    Generic(std::string::String, Vec<Type>),
    TypeVar(std::string::String),
    Class(std::string::String, Vec<(std::string::String, Box<Type>)>),
    Interface(std::string::String, Vec<MethodType>),
    Trait(std::string::String, Vec<MethodType>),
    Quantum(Box<Type>),
    Nano(Box<Type>),
    MTS(Box<Type>),
    Sankofa(Box<Type>),
    Effect(std::string::String),
    Effectful(Box<Type>, Vec<std::string::String>),
    DependentPi(Box<Type>, Box<Type>),
    DependentSigma(Box<Type>, Box<Type>),
    Universe(usize),
    Unknown,
    Error,
}

impl Type {
    pub fn get_name(&self) -> std::string::String {
        match self {
            Type::Unit => "unit".into(),
            Type::Bool => "bool".into(),
            Type::Int(w) => format!("{:?}", w).to_lowercase(),
            Type::Float(w) => format!("{:?}", w).to_lowercase(),
            Type::Char => "char".into(),
            Type::Str => "str".into(),
            Type::String => "String".into(),
            Type::TypeVar(n) | Type::Effect(n) => n.clone(),
            Type::Generic(n, args) if args.is_empty() => n.clone(),
            Type::Generic(n, args) => format!(
                "{}<{}>",
                n,
                args.iter()
                    .map(|a| a.get_name())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Class(n, _) | Type::Interface(n, _) | Type::Trait(n, _) => n.clone(),
            Type::Quantum(t) => format!("quantum<{}>", t.get_name()),
            Type::Nano(t) => format!("nano<{}>", t.get_name()),
            Type::MTS(t) => format!("mts<{}>", t.get_name()),
            Type::Sankofa(t) => format!("sankofa<{}>", t.get_name()),
            Type::Universe(n) => format!("Type{}", n),
            Type::Unknown => "<unknown>".into(),
            Type::Error => "<error>".into(),
            _ => format!("{:?}", self),
        }
    }
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::Float(_))
    }
    pub fn is_bool(&self) -> bool {
        *self == Type::Bool
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Type::Error | Type::Unknown)
    }
    pub fn is_quantum(&self) -> bool {
        matches!(self, Type::Quantum(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodType {
    pub name: std::string::String,
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationTarget {
    X86_64Linux,
    Arm64,
    Wasm32,
    QASM,
    NanoControl,
    MTSBytecode,
    LLVMIR,
    RiscV,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    UltraAGI,
}

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub target: CompilationTarget,
    pub opt_level: OptimizationLevel,
    pub debug_info: bool,
    pub verify: bool,
    pub emit_ir: bool,
    pub parallel: bool,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        CompilerConfig {
            target: CompilationTarget::X86_64Linux,
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: false,
            emit_ir: false,
            parallel: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: std::string::String,
    pub span: Option<Span>,
    pub notes: Vec<std::string::String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<std::string::String>, span: Option<Span>) -> Self {
        Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            notes: vec![],
        }
    }
    pub fn warning(message: impl Into<std::string::String>, span: Option<Span>) -> Self {
        Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
            notes: vec![],
        }
    }
    pub fn with_note(mut self, note: impl Into<std::string::String>) -> Self {
        self.notes.push(note.into());
        self
    }
}
