//! Zamani Universal Meta-Compiler (UMC) Compiler Stages and Control
//!
//! This module orchestrates the various stages of the Zamani UMC, from
//! frontend parsing to backend code generation. It defines the overall
//! compiler pipeline and integrates advanced features like optimization,
//! formal verification, and multi-paradigm support.

pub mod compilation_techniques; // For Hybrid Compilation Strategies
pub mod ir_exporters;
pub mod wasm_backend;
pub mod wasm_cfg;
pub mod llvm_backend;
pub mod language_spec; // Zamani Language Specification modules
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
pub mod oop_advanced; // Advanced OOP Features
pub mod optimization_strategies;
pub mod test_metadata; // Compiler test metadata helpers // For managing and applying diverse optimization passes

/// Re-exports the front-end (lexing/parsing/semantic-analysis) stage types
/// under a single conventional path for language-spec modules to depend on.
pub mod frontend {
    pub use crate::semantic::{SemanticAnalyzer, TypeChecker};
}

/// Initializes the entire Zamani UMC compiler pipeline.
pub fn initialize_compiler() {
    println!("Initializing Zamani UMC Compiler...");
    oop_advanced::init_oop_advanced();
    language_spec::init_language_spec();
    self::compilation_techniques::init_compilation_techniques();
    optimization_strategies::init_optimization_strategies(); // Initialize Optimization Strategies module
    test_metadata::init_test_metadata();
    println!("Zamani UMC Compiler initialized.");
}

/// Shuts down the entire Zamani UMC compiler pipeline.
pub fn shutdown_compiler() {
    println!("Shutting down Zamani UMC Compiler...");
    test_metadata::shutdown_test_metadata();
    optimization_strategies::shutdown_optimization_strategies(); // Shutdown Optimization Strategies module
    self::compilation_techniques::shutdown_compilation_techniques();
    language_spec::shutdown_language_spec();
    oop_advanced::shutdown_oop_advanced();
    println!("Zamani UMC Compiler shut down.");
}

/// Triggers a full compilation process for a given Zamani source file.
pub fn compile(source_file_path: &str) -> Result<Vec<u8>, String> {
    println!("Compiling '{}' using Zamani UMC pipeline.", source_file_path);
    
    // 1. Read source file
    let source_code = std::fs::read_to_string(source_file_path)
        .map_err(|e| format!("Failed to read source file '{}': {}", source_file_path, e))?;

    // 2. Lexing & Parsing (with Macro Expansion pass)
    let file_id = crate::source_map::FileId::new(0);
    let source_file = std::sync::Arc::new(crate::source_map::SourceFile::new(source_file_path.into(), source_code));
    let lexer = crate::lexer::Lexer::new(file_id, source_file);
    let mut parser = crate::parser::Parser::new(lexer);
    let mut program = parser.parse_program();

    // 2.5. Macro Engine Pass (Optimized / Asynchronous)

    let errors = parser.get_errors();
    if !errors.is_empty() {
        let mut err_msg = String::new();
        for err in errors {
            err_msg.push_str(&format!("Parse Error: {:?}\n", err));
        }
        return Err(err_msg);
    }

    // 3. Semantic Analysis
    let mut analyzer = crate::semantic::SemanticAnalyzer::new();
    let semantic_errors = analyzer.analyze(&program);
    if !semantic_errors.is_empty() {
        let mut err_msg = String::new();
        for err in semantic_errors {
            err_msg.push_str(&format!("Semantic Error: {:?}\n", err));
        }
        return Err(err_msg);
    }

    // 4. IR Generation
    let mut ir_gen = crate::ir_gen::IrGenerator::new();
    let raw_ir_module = ir_gen.generate(&program);

    // 4.2. Optimization Pass
    let mut optimizer = crate::optimizer::Optimizer::with_level(2);
    let ir_module = optimizer.optimize(&raw_ir_module);

    // 4.5. IR Verification & Safety Guard Inspection
    if let Err(errors) = crate::ir_verify::verify_module(&ir_module) {
        let mut err_msg = String::new();
        for err in errors {
            err_msg.push_str(&format!("IR Verification Error: {}\n", err));
        }
        return Err(err_msg);
    }

    let mut security_ctx = crate::compiler::safety_guard::GlobalSecurityContext::new();
    let safety_guard = crate::compiler::safety_guard::SafetyGuard::new("Zamani-Compiler-Core");
    let funcs_as_strings: Vec<String> = ir_module.functions.iter().map(|f| f.name.clone()).collect();
    if let Err(e) = safety_guard.inspect_with_context(&funcs_as_strings, &mut security_ctx) {
        return Err(format!("Safety Guard Interception: {}", e));
    }

    // 5. Backend Code Generation
    let config = crate::compiler_types::CompilerConfig::default();
    let code_gen = crate::backend::CodeGenerator::new(config);
    let output = code_gen.generate(&ir_module)
        .map_err(|e| format!("Code generation error ({:?}): {}", e.target, e.message))?;

    println!("Successfully compiled to target '{}' ({} bytes)", output.target, output.size_bytes);
    Ok(output.source.into_bytes())
}
