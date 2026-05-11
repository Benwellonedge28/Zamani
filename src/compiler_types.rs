
//! Zenith Universal Meta-Compiler (UMC) Compiler Types
//!
//! This module defines fundamental data structures used throughout the Zenith compiler
//! pipeline, particularly for semantic analysis and IR generation. These types
//! represent the core concepts of Zenith's unified type system and symbol management.

use crate::ast::{TypeExpr, Identifier};
use crate::tokens::Span;
use std::collections::HashMap;

/// Represents a compiled type in Zenith's unified type system.
/// This encompasses classical, quantum, nano, dependent, linear, affine, and effectful types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // --- Primitive & Core Types ---
    Unit,         // The void/unit type
    Bool,         // Boolean type
    Int(IntWidth), // Integer with specified bit-width (e.g., I8, I16, I32, I64, I128)
    Float(FloatWidth), // Floating-point with specified precision (e.g., F32, F64)
    Char,         // Unicode scalar value
    String,       // Immutable string slice
    Pointer(Box<Type>), // Raw pointer type

    // --- Compound Types ---
    Array(Box<Type>, Option<usize>), // Array of elements with optional fixed size
    Tuple(Vec<Type>), // Fixed-size ordered collection of types
    Struct(Identifier, HashMap<String, Type>), // User-defined composite type
    Enum(Identifier, HashMap<String, Option<Type>>), // User-defined discriminated union
    Function(Vec<Type>, Box<Type>), // Function type: (param_types) -> return_type

    // --- Zenith-Specific Types ---
    // Quantum
    Qubit,        // Fundamental quantum bit
    QReg(usize),  // Quantum register (array of qubits) with size
    Superposition(Box<Type>), // Type representing a superposition state of another type
    Entangled(Box<Type>, Box<Type>), // Type representing two entangled types
    QMeasured(Box<Type>), // Type representing a classical value obtained from a quantum measurement

    // Nano-Agent
    NanoAgent(Option<Identifier>), // A nano-agent, optionally with a blueprint ID
    Atom(Box<Type>),      // A type viewed at the atomic level
    Molecule(Box<Type>),  // A type viewed at the molecular level

    // Multi-Timeline System (MTS)
    MtsSlice(Box<Type>), // A slice of state within an MTS timeline
    MtsTimeline(Box<Type>), // A full timeline

    // Linear/Affine Types
    Linear(Box<Type>),
    Affine(Box<Type>),

    // Dependent Types (Conceptual)
    DependentPi(Identifier, Box<Type>, Box<Type>), // (x: A) -> B(x)
    DependentSigma(Identifier, Box<Type>, Box<Type>), // Sigma(x: A) B(x)

    // Algebraic Effects
    Effect(Identifier),
    Effectful(Box<Type>, Vec<Identifier>),

    // Universe Types (for Metaprogramming/Type Theory)
    TypeUniverse(usize),
    Kind,
    Prop,

    // Sankofa Temporal Memory
    ZamaniFact(Box<Type>),
    SasaKnowledge(Box<Type>),
    History(Box<Type>, Box<Expression>), // History of a type over a temporal expression (e.g., years)
    ConsensusTrue(Box<Type>),
    InterMemory(Identifier, Box<Type>),

    // Error Type
    Error,
    Unknown,
}

/// Integer bit-widths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntWidth {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
}

/// Floating-point precisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatWidth {
    F32, F64,
}

/// Represents a symbol (variable, function, type, effect) in the compiler's symbol table.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub typ: Type,
    pub span: Span,
    pub is_mutable: bool,
    // Additional fields for context, e.g., scope, function parameters, global/local
}

impl Symbol {
    pub fn new(name: String, typ: Type, span: Span, is_mutable: bool) -> Self {
        Symbol { name, typ, span, is_mutable }
    }
}
