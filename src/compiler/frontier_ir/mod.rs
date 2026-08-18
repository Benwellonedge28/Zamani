//! Zamani Universal Meta-Compiler (UMC)
//!
//! This module owns the canonical production compilation pipeline.
//!
//! Production pipeline:
//!
//! ```text
//! source
//!   -> lexer
//!   -> parser
//!   -> semantic analysis
//!   -> ownership/borrow analysis
//!   -> IR generation
//!   -> optimization
//!   -> IR verification
//!   -> security inspection
//!   -> backend code generation
//!   -> output
//! ```
//!
//! The compiler is deliberately silent: it does not print progress messages
//! to stdout/stderr. Diagnostics are returned to the caller so that CLI,
//! LSP, IDE, library, and CI consumers can format them appropriately.
//!
//! The module keeps the existing public `compile()` entry point for
//! compatibility while making `compile_with_config()` the canonical API.

pub mod compilation_techniques;
pub mod ir_exporters;
pub mod wasm_backend;
pub mod wasm_cfg;
pub mod llvm_backend;
pub mod language_spec;
pub mod monomorphizer;
pub mod type_inference;
pub mod linker;
pub mod diagnostics;
pub mod macro_engine;
pub mod incremental;
pub mod parallel_build;
pub mod jit;
pub mod borrow_checker;
pub mod hardware_partitioner;
pub mod hybrid_pipeline;
pub mod hybrid_profiles;
pub mod ssbe;
pub mod sro;
pub mod instruction_fusion;
pub mod safety_guard;
pub mod fuzzing_harness;
pub mod audit_engine;
pub mod unique_ir_features;
pub mod oop_advanced;
pub mod optimization_strategies;
pub mod test_metadata;

/// Re-exports the front-end semantic-analysis types under a stable compiler
/// namespace.
pub mod frontend {
    pub use crate::semantic::{SemanticAnalyzer, TypeChecker};
}

use std::fmt;
use std::sync::Arc;

use crate::ast::Expression;
use crate::compiler_types::{CompilerConfig, OptimizationLevel};
use crate::ir::IrModule;

/// Structured compiler failure.
///
/// Keeping compilation failures typed internally makes the compiler usable
/// from CLI, LSP, IDE, library, and CI environments without parsing strings.
#[derive(Debug)]
pub enum CompilerError {
    Io {
        path: String,
        message: String,
    },
    Parse {
        message: String,
    },
    Semantic {
        message: String,
    },
    Borrow {
        message: String,
    },
    IrGeneration {
        message: String,
    },
    Optimization {
        message: String,
    },
    Verification {
        message: String,
    },
    Security {
        message: String,
    },
    CodeGeneration {
        message: String,
    },
    Configuration {
        message: String,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(formatter, "failed to read source `{path}`: {message}")
            }
            Self::Parse { message } => {
                write!(formatter, "parse error: {message}")
            }
            Self::Semantic { message } => {
                write!(formatter, "semantic error: {message}")
            }
            Self::Borrow { message } => {
                write!(formatter, "borrow error: {message}")
            }
            Self::IrGeneration { message } => {
                write!(formatter, "IR generation error: {message}")
            }
            Self::Optimization { message } => {
                write!(formatter, "optimization error: {message}")
            }
            Self::Verification { message } => {
                write!(formatter, "IR verification error: {message}")
            }
            Self::Security { message } => {
                write!(formatter, "security validation error: {message}")
            }
            Self::CodeGeneration { message } => {
                write!(formatter, "code generation error: {message}")
            }
            Self::Configuration { message } => {
                write!(formatter, "compiler configuration error: {message}")
            }
        }
    }
}

impl std::error::Error for CompilerError {}

/// Result of a successful compilation.
#[derive(Debug)]
pub struct CompilationOutput {
    /// Target-specific generated output.
    pub bytes: Vec<u8>,
    /// Target selected for the compilation.
    pub target: String,
    /// Number of bytes generated.
    pub size_bytes: usize,
}

/// Initializes compiler subsystems.
///
/// This function is retained for compatibility with existing callers.
///
/// Production compilation itself does not depend on this function having been
/// called first; compiler stages must be independently usable and testable.
pub fn initialize_compiler() {
    oop_advanced::init_oop_advanced();
    language_spec::init_language_spec();
    compilation_techniques::init_compilation_techniques();
    optimization_strategies::init_optimization_strategies();
    test_metadata::init_test_metadata();
}

/// Shuts down compiler subsystems.
///
/// This function is retained for compatibility with existing callers.
pub fn shutdown_compiler() {
    test_metadata::shutdown_test_metadata();
    optimization_strategies::shutdown_optimization_strategies();
    compilation_techniques::shutdown_compilation_techniques();
    language_spec::shutdown_language_spec();
    oop_advanced::shutdown_oop_advanced();
}

/// Compiles a source file using the production-default configuration.
///
/// This is the compatibility API used by existing callers.
pub fn compile(source_file_path: &str) -> Result<Vec<u8>, String> {
    compile_with_config(source_file_path, production_config())
        .map(|output| output.bytes)
        .map_err(|error| error.to_string())
}

/// Compiles a source file using an explicit configuration.
///
/// This is the canonical compiler entry point.
pub fn compile_with_config(
    source_file_path: &str,
    config: CompilerConfig,
) -> Result<CompilationOutput, CompilerError> {
    validate_config(&config)?;

    let source_code = std::fs::read_to_string(source_file_path).map_err(|error| {
        CompilerError::Io {
            path: source_file_path.to_string(),
            message: error.to_string(),
        }
    })?;

    compile_source(&source_code, source_file_path, config)
}

/// Compiles source text without requiring a temporary source file.
///
/// This is the preferred API for LSPs, tests, editors, REPLs, and embedded
/// compiler consumers.
pub fn compile_source(
    source_code: &str,
    source_file_path: &str,
    config: CompilerConfig,
) -> Result<CompilationOutput, CompilerError> {
    validate_config(&config)?;

    if source_code.is_empty() {
        return Err(CompilerError::Parse {
            message: "source file is empty".to_string(),
        });
    }

    // ---------------------------------------------------------------------
    // 1. Lexing + parsing
    // ---------------------------------------------------------------------

    let file_id = crate::source_map::FileId::new(0);

    let source_file = Arc::new(crate::source_map::SourceFile::new(
        source_file_path.into(),
        source_code.to_string(),
    ));

    let lexer = crate::lexer::Lexer::new(file_id, source_file);
    let mut parser = crate::parser::Parser::new(lexer);

    let program = parser.parse_program();

    let parser_errors = parser.get_errors();

    if !parser_errors.is_empty() {
        return Err(CompilerError::Parse {
            message: format_parser_errors(parser_errors),
        });
    }

    // ---------------------------------------------------------------------
    // 2. Semantic analysis
    // ---------------------------------------------------------------------

    let mut analyzer = crate::semantic::SemanticAnalyzer::new();

    let semantic_errors = analyzer.analyze(&program);

    if !semantic_errors.is_empty() {
        return Err(CompilerError::Semantic {
            message: format_semantic_errors(semantic_errors),
        });
    }

    // ---------------------------------------------------------------------
    // 3. Ownership / borrow analysis
    // ---------------------------------------------------------------------
    //
    // The current AST contains language constructs beyond the ownership
    // primitives understood by the standalone BorrowChecker. We therefore
    // initialize the checker here as part of the canonical pipeline without
    // guessing at AST variants. Ownership-sensitive semantic lowering should
    // call its explicit declare/borrow/move APIs.

    let mut borrow_checker = borrow_checker::BorrowChecker::new();

    check_program_ownership(&program, &mut borrow_checker)?;

    // ---------------------------------------------------------------------
    // 4. IR generation
    // ---------------------------------------------------------------------

    let mut ir_generator = crate::ir_gen::IrGenerator::new();

    let raw_ir_module = ir_generator.generate(&program);

    // ---------------------------------------------------------------------
    // 5. Optimization
    // ---------------------------------------------------------------------

    let optimization_level = optimizer_level(&config.opt_level);

    let optimized_ir = if optimization_level == 0 {
        raw_ir_module
    } else {
        let mut optimizer =
            crate::optimizer::Optimizer::with_level(optimization_level);

        optimizer.optimize(&raw_ir_module)
    };

    // ---------------------------------------------------------------------
    // 6. Mandatory IR verification
    // ---------------------------------------------------------------------

    verify_ir(&optimized_ir)?;

    // ---------------------------------------------------------------------
    // 7. Security inspection
    // ---------------------------------------------------------------------

    inspect_security(&optimized_ir)?;

    // ---------------------------------------------------------------------
    // 8. Backend code generation
    // ---------------------------------------------------------------------

    let code_generator =
        crate::backend::CodeGenerator::new(config.clone());

    let generated = code_generator
        .generate(&optimized_ir)
        .map_err(|error| CompilerError::CodeGeneration {
            message: format!("{:?}: {}", error.target, error.message),
        })?;

    // ---------------------------------------------------------------------
    // 9. Post-generation validation
    // ---------------------------------------------------------------------
    //
    // We cannot blindly reinterpret target output as Zamani IR, so the final
    // stage validates the generated artifact at the backend boundary where
    // possible. Backend-specific validation belongs to each backend.

    if generated.source.is_empty() {
        return Err(CompilerError::CodeGeneration {
            message: "backend produced an empty compilation artifact".to_string(),
        });
    }

    Ok(CompilationOutput {
        bytes: generated.source.into_bytes(),
        target: generated.target,
        size_bytes: generated.size_bytes,
    })
}

/// Returns the production compiler defaults.
///
/// Verification is intentionally always enabled by the production pipeline.
pub fn production_config() -> CompilerConfig {
    CompilerConfig {
        target: crate::compiler_types::CompilationTarget::X86_64Linux,
        opt_level: OptimizationLevel::Basic,
        debug_info: true,
        verify: true,
        emit_ir: false,
        parallel: false,
    }
}

fn validate_config(config: &CompilerConfig) -> Result<(), CompilerError> {
    // UltraAGI optimization is deliberately not accepted as a production
    // compiler mode until every transformation in that pipeline has an
    // independently verified semantic-preservation contract.
    if matches!(config.opt_level, OptimizationLevel::UltraAGI) {
        return Err(CompilerError::Configuration {
            message:
                "UltraAGI optimization is experimental and cannot be used by the production compiler"
                    .to_string(),
        });
    }

    Ok(())
}

fn optimizer_level(level: &OptimizationLevel) -> u8 {
    match level {
        OptimizationLevel::None => 0,
        OptimizationLevel::Basic => 1,
        OptimizationLevel::Aggressive => 2,
        OptimizationLevel::UltraAGI => 3,
    }
}

fn verify_ir(module: &IrModule) -> Result<(), CompilerError> {
    crate::ir_verify::verify_module(module).map_err(|errors| {
        let message = errors
            .into_iter()
            .map(|error| format!("IR Verification Error: {error}"))
            .collect::<Vec<_>>()
            .join("\n");

        CompilerError::Verification { message }
    })
}

fn inspect_security(module: &IrModule) -> Result<(), CompilerError> {
    let mut security_context =
        safety_guard::GlobalSecurityContext::new();

    let guard = safety_guard::SafetyGuard::new("Zamani-Compiler-Core");

    let function_names = module
        .functions
        .iter()
        .map(|function| function.name.clone())
        .collect::<Vec<_>>();

    guard
        .inspect_with_context(&function_names, &mut security_context)
        .map_err(|error| CompilerError::Security {
            message: error.to_string(),
        })
}

/// Performs the ownership-analysis integration point.
///
/// The standalone BorrowChecker intentionally does not guess AST variants.
/// This function therefore establishes a clean boundary for the semantic
/// ownership pass. Once the semantic analyzer exposes ownership operations,
/// they should be lowered here.
fn check_program_ownership(
    _program: &crate::ast::Program,
    _checker: &mut borrow_checker::BorrowChecker,
) -> Result<(), CompilerError> {
    Ok(())
}

fn format_parser_errors<T: fmt::Debug>(errors: Vec<T>) -> String {
    errors
        .into_iter()
        .map(|error| format!("{error:?}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_semantic_errors<T: fmt::Debug>(errors: Vec<T>) -> String {
    errors
        .into_iter()
        .map(|error| format!("{error:?}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_configuration_enables_verification() {
        let config = production_config();

        assert!(config.verify);
        assert_eq!(config.target, crate::compiler_types::CompilationTarget::X86_64Linux);
        assert_eq!(config.opt_level, OptimizationLevel::Basic);
    }

    #[test]
    fn production_configuration_does_not_use_experimental_optimization() {
        let config = production_config();

        assert_ne!(config.opt_level, OptimizationLevel::UltraAGI);
    }

    #[test]
    fn ultra_agi_optimization_is_rejected() {
        let config = CompilerConfig {
            opt_level: OptimizationLevel::UltraAGI,
            ..production_config()
        };

        let error = validate_config(&config)
            .expect_err("experimental optimization must be rejected");

        assert!(matches!(error, CompilerError::Configuration { .. }));
    }

    #[test]
    fn optimizer_levels_are_deterministic() {
        assert_eq!(optimizer_level(&OptimizationLevel::None), 0);
        assert_eq!(optimizer_level(&OptimizationLevel::Basic), 1);
        assert_eq!(optimizer_level(&OptimizationLevel::Aggressive), 2);
        assert_eq!(optimizer_level(&OptimizationLevel::UltraAGI), 3);
    }

    #[test]
    fn compiler_error_display_is_stable() {
        let error = CompilerError::Io {
            path: "example.zm".to_string(),
            message: "permission denied".to_string(),
        };

        assert_eq!(
            error.to_string(),
            "failed to read source `example.zm`: permission denied"
        );
    }
}