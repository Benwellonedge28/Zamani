//! Zamani Omniversal Language Compiler — Core Library
#![allow(static_mut_refs)]
//!
//! Exposes the full Zamani compiler pipeline:
//! source_map → lexer → AST → parser → semantic → ir_gen → optimizer → backend

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::all)]

pub mod ast;
pub mod backend;
#[cfg(feature = "full")]
pub mod compiler;
pub mod compiler_types;
#[cfg(feature = "full")]
pub mod core_lang_primitives;
pub mod error_reporting;
#[cfg(feature = "full")]
pub mod hdl;
pub mod ir_gen;
pub mod lexer;
pub mod nano; // Nano runtime primitives
#[cfg(feature = "full")]
pub mod nimbus;
#[cfg(feature = "full")]
pub mod nimbus_os;
pub mod omega_trinity_libs_161_200; // Omega Trinity extended libs 161-200
pub mod optimizer;
pub mod parser;
pub mod quantum; // Quantum primitives
#[cfg(feature = "full")]
pub mod runtime;
pub mod semantic;
pub mod source_map;
#[cfg(feature = "full")]
pub mod stdlib;
#[cfg(any(feature = "full", feature = "lsp", feature = "buildsystem"))]
pub mod toolchain;
#[cfg(any(feature = "full", feature = "buildsystem"))]
pub mod zamani_project_config;

/// Initialise the Zamani Universal Trinity Runtime.
pub fn initialize_runtime() {
    eprintln!("[Zamani] Universal Trinity Runtime initialised.");
}

/// Version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run the full compile pipeline on a source string.
/// Returns Ok(ir) or Err(diagnostics).
pub fn compile(source: &str) -> Result<ir_gen::IrModule, Vec<String>> {
    use source_map::{FileId, SourceFile};
    use std::sync::Arc;

    let file_id = FileId::new(1);
    let sf = Arc::new(SourceFile::new("<stdin>".to_string(), source.to_string()));
    let lex = lexer::Lexer::new(file_id, sf);

    let mut parser = parser::Parser::new(lex);
    let program = parser.parse_program();
    let parse_errors = parser.get_errors().clone();
    if !parse_errors.is_empty() {
        return Err(parse_errors
            .iter()
            .map(|e| format!("ParseError: {}", e.message))
            .collect());
    }

    let mut sem = semantic::SemanticAnalyzer::new();
    let sem_errors = sem.analyze(&program);
    if !sem_errors.is_empty() {
        return Err(sem_errors
            .iter()
            .map(|e| format!("SemanticError: {}", e.message))
            .collect());
    }

    let mut ir_gen = ir_gen::IrGenerator::new();
    let module = ir_gen.generate(&program);
    Ok(module)
}
