//! Zenith UMC Compiler Types
//!
//! Shared types used across compiler phases.

use crate::source_map::Span;

/// A symbol identifier: name + source span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String, pub Span);

/// Access visibility modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
}

/// Method behaviour modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodModifier {
    Override,
    Virtual,
    Abstract,
    Static,
}

/// Output target for code generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationTarget {
    X86_64Linux,
    Wasm32,
    QASM,
    NanoControl,
    MTSBytecode,
}

/// Optimisation level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    UltraAGI,
}

/// Overall compiler configuration.
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub target:     CompilationTarget,
    pub opt_level:  OptimizationLevel,
    pub debug_info: bool,
    pub verify:     bool,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        CompilerConfig {
            target:     CompilationTarget::X86_64Linux,
            opt_level:  OptimizationLevel::Basic,
            debug_info: true,
            verify:     false,
        }
    }
}
