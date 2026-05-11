
//! Zenith Universal Meta-Compiler (UMC) Compiler Types
//!
//! This module defines the internal type system used by the Zenith compiler.
//! It represents all data types, function signatures, and other type-related
//! information gathered during semantic analysis. Zenith's type system is rich
//! and includes traditional classical types, quantum types, nano-agent types,
//! multi-timeline system (MTS) types, and Sankofa memory types.

use std::collections::HashMap;
use crate::ast::{Identifier, AccessModifier, MethodModifier}; // For OOP

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    Unit,
    Bool,
    Char,
    Int(IntWidth),
    Float(FloatWidth),
    String,

    // Compound types
    Array(Box<Type>, Option<usize>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>), // Parameters, Return Type
    Struct(Identifier, HashMap<String, Type>), // Name, Fields

    // Multi-paradigm types
    Qubit,
    QReg(usize), // Quantum Register (size)
    Superposition(Box<Type>), // e.g., Superposition<Qubit>
    Entangled(Box<Type>, Box<Type>), // e.g., Entangled<Qubit, Qubit>
    NanoAgent(Option<Identifier>), // Nano Agent (optional blueprint/class name)
    MtsSlice(Box<Type>), // Multi-Timeline Slice, parameterized by content type
    ZamaniFact(Box<Type>), // Sankofa Immutable Fact, parameterized by content type
    SasaKnowledge(Box<Type>), // Sankofa Evolving Knowledge, parameterized by content type
    History(Box<Type>, Box<crate::ast::Expression>), // Sankofa History type, param by content and timestamp expr
    ConsensusTrue(Box<Type>), // Sankofa type for provably true facts
    InterMemory(String, Box<Type>), // Inter-language memory access, param by language and content type

    // Dependent Types (conceptual)
    DependentPi(Identifier, Box<Type>, Box<Type>), // Π (binder : bindee_type) -> body_type
    DependentSigma(Identifier, Box<Type>, Box<Type>), // Σ (binder : bindee_type) x body_type
    Prop, // Type of propositions (for proofs)
    TypeUniverse(usize), // Type of types (e.g., Type@0, Type@1)

    // Linear & Affine Types
    Linear(Box<Type>), // Must be used exactly once
    Affine(Box<Type>), // Must be used at most once

    // Algebraic Effects
    Effect(Identifier), // Effect type (e.g., MyEffect)
    Effectful(Box<Type>, Vec<Identifier>), // Type that can perform certain effects

    // --- OOP Additions ---
    Class {
        name: Identifier,
        fields: HashMap<String, Type>,
        methods: HashMap<String, MethodType>,
        parent_class: Option<Box<Type>>,
        implemented_interfaces: Vec<Type>,
        is_abstract: bool, // Added for semantic analysis
    },
    Interface {
        name: Identifier,
        methods: HashMap<String, MethodType>,
        parent_interfaces: Vec<Type>, // Added for semantic analysis
    },
    Method(Vec<Type>, Box<Type>, AccessModifier, Option<MethodModifier>), // Parameters, Return Type, Access Modifier, Method Modifier

    // Special types for compiler internals/errors
    Unknown,
    Error,
}

impl Type {
    pub fn get_name(&self) -> Identifier { // Helper to get name for symbols
        match self {
            Type::Class { name, .. } => name.clone(),
            Type::Interface { name, .. } => name.clone(),
            _ => Identifier("Unknown".to_string(), Span::dummy()), // Placeholder
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntWidth {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatWidth {
    F32, F64,
}

// New struct for MethodType details for classes/interfaces
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodType {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
    pub access_modifier: AccessModifier,
    pub method_modifier: Option<MethodModifier>, // Virtual, Abstract, Override
    pub effects: Vec<Identifier>, // Effects that this method might perform
}
