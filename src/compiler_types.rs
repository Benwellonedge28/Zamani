//! Zenith Compiler Internal Types
//!
//! This module defines core data structures used internally by the Zenith compiler
//! during phases like semantic analysis, type checking, and symbol management.
//! These types represent the compiler's understanding of the program's structure,
//! types, and rules.

use crate::ast::{Identifier, TypeExpr, Literal, Expression};
use crate::source_map::Span; // Corrected Span import
use std::collections::HashMap;

/// Represents a resolved type within the Zenith type system.
/// This is more detailed than AST's TypeExpr, containing resolved information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Primitive integer type (e.g., int, i32, u64)
    Int,
    /// Primitive floating-point type (e.g., float, f64)
    Float,
    /// Boolean type
    Bool,
    /// Character type
    Char,
    /// String type
    String,
    /// Unit type, for expressions that return no meaningful value
    Unit,
    /// Represents a single qubit
    Qubit,
    /// Represents a register of N qubits
    QubitArray(usize),
    /// Represents a type in a quantum superposition state
    Superposition(Box<Type>),
    /// Represents two or more entangled quantum types
    Entangled(Vec<Type>),
    /// The result of a quantum measurement
    QMeasured(Box<Type>), // Typically QMeasured<bool> or QMeasured<int>
    /// Represents a nano-particle or molecular agent
    NanoParticle,
    /// Represents an array/assembly of nano-particles
    NanoArray(usize),
    /// Multi-Timeline System slice type, allowing temporal branching
    MtsSlice(Box<Type>), // MtsSlice<T> implies T can exist in multiple timelines
    /// Sankofa History type, storing temporal data associated with a key
    History(Box<Type>), // History<T> for a specific key
    /// Type for representing the ancestral memory (immutable)
    Zamani,
    /// Type for representing the present/evolving knowledge (mutable)
    Sasa,
    /// Inter-Memory type, allowing interoperation between different language memories
    InterMemory(String, Box<Type>), // (Language ID, Inner Type)
    /// A user-defined struct type
    Struct(String, HashMap<String, Type>), // Name and its fields
    /// A user-defined enum type
    Enum(String, HashMap<String, Option<Type>>), // Name and its variants with optional payload types
    /// A function type: (param_types...) -> return_type
    Function(Vec<Type>, Box<Type>),
    /// A tuple type: (T1, T2, ...)
    Tuple(Vec<Type>),
    /// A type with a linear resource usage constraint
    Linear(Box<Type>),
    /// A type with an affine resource usage constraint
    Affine(Box<Type>),
    /// A type that is effectful, carrying information about potential side-effects
    Effectful(Box<Type>, Vec<Identifier>), // InnerType, list of effect names
    /// A dependent function type Π(name: BinderType) -> ReturnType
    Pi(String, Box<Type>, Box<Type>),
    /// A dependent pair type Σ(name: FirstType) x SecondType
    Sigma(String, Box<Type>, Box<Type>),
    /// A proof type, indicating a theorem has been proven
    Proof(Box<Type>), // Proof of what type assertion
    /// A type family or type class instance
    TypeFamily(String, Vec<Type>),
    /// Represents a quantum teleportation channel
    QuantumTeleportationChannel,
    /// Placeholder for a type that couldn't be resolved (error state)
    Error,
    /// Placeholder for a type that is currently unknown (e.g., during inference)
    Unknown,
}

/// Represents an entry in the symbol table, storing information about an identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub is_mutable: bool,
    pub span: Span,
    pub kind: SymbolKind, // e.g., variable, function, type alias
}

/// Distinguishes different kinds of symbols.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    TypeAlias,
    Effect,
    QuantumRegister,
    NanoAgent,
    MtsSlice,
    SankofaMemoryKey,
    // ... other kinds
}

/// Represents a scope in the program (e.g., function body, block).
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>, // Index into a vec of scopes
}

/// Represents a type constraint, used during type inference and checking.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Type equality: T1 = T2
    Equals(Type, Type),
    /// Subtyping: T1 <: T2
    Subtype(Type, Type),
    /// Has trait: T implements Trait
    Implements(Type, Identifier),
    /// Custom constraint for linear/affine usage (e.g., T must be consumable)
    LinearUsage(Type),
    /// Custom constraint for quantum state (e.g., T must be in superposition)
    QuantumState(Type, String),
    // ... other constraints
}

/// Represents the EVAS (Ethical, Verifiable, Accountable, Secure) policy for a given compilation context.
#[derive(Debug, Clone, PartialEq)]
pub struct EvasPolicy {
    /// If true, 'unsafe' blocks without explicit proofs are allowed.
    pub allow_unsafe_without_proof: bool,
    /// List of approved proof strings or proof IDs that justify 'unsafe' operations.
    pub approved_proofs: HashMap<String, bool>, // proof_string -> is_valid
    /// Determines how strictly resource linearity/affinity is enforced.
    pub strict_resource_management: bool,
    /// Controls the level of quantum entanglement tracking and validation.
    pub quantum_coherence_monitoring: bool,
    /// Defines acceptable behaviors for nano-agents (e.g., no self-replication without explicit permission).
    pub nano_agent_safety_protocols: Vec<String>,
    /// Rules for accessing and modifying Sankofa temporal memory (e.g., no unauthorized history rewrites).
    pub sankofa_memory_access_rules: Vec<String>,
    /// The overall ethical filter level (e.g., strict, permissive, default).
    pub ethical_filter_level: String,
}

impl Default for EvasPolicy {
    fn default() -> Self {
        EvasPolicy {
            allow_unsafe_without_proof: false,
            approved_proofs: HashMap::new(),
            strict_resource_management: true,
            quantum_coherence_monitoring: true,
            nano_agent_safety_protocols: vec!["no_uncontrolled_replication".to_string()],
            sankofa_memory_access_rules: vec!["no_unauthorized_history_overwrite".to_string()],
            ethical_filter_level: "strict".to_string(),
        }
    }
}
