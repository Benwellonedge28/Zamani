//! Zenith Omniversal Language Compiler — Core Library
//!
//! Exposes the full Zenith compiler pipeline:
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
#[cfg(feature = "full")]
pub mod nimbus;
#[cfg(feature = "full")]
pub mod nimbus_os;
pub mod optimizer;
pub mod parser;
#[cfg(feature = "full")]
pub mod runtime;
pub mod semantic;
pub mod source_map;
#[cfg(feature = "full")]
pub mod stdlib;
#[cfg(feature = "full")]
pub mod toolchain;
#[cfg(feature = "full")]
pub mod zenith_project_config;

/// Initialise the Zenith Universal Trinity Runtime.
pub fn initialize_runtime() {
    eprintln!("[Zenith] Universal Trinity Runtime initialised.");
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
