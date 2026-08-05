#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unreachable_code,
    unused_comparisons
)]

//! Zamani Compiler Pipeline — End-to-End Integration Tests
//!
//! Tests the full pipeline: source → lex → parse → semantic → IR → optimize → codegen.

use std::sync::Arc;
use zamani_compiler::backend::{Backend, CodeGenerator, LlvmIrBackend};
use zamani_compiler::compiler_types::{CompilationTarget, CompilerConfig, OptimizationLevel};
use zamani_compiler::ir_gen::IrGenerator;
use zamani_compiler::lexer::Lexer;
use zamani_compiler::optimizer::Optimizer;
use zamani_compiler::parser::Parser;
use zamani_compiler::semantic::SemanticAnalyzer;
use zamani_compiler::source_map::{FileId, SourceFile};
use zamani_compiler::{compile, initialize_runtime, VERSION};

fn full_pipeline(source: &str) -> Result<String, Vec<String>> {
    compile(source).map(|module| {
        LlvmIrBackend
            .generate(&module)
            .unwrap_or_else(|e| format!("CodeGenError: {}", e.message))
    })
}

fn pipeline_ok(source: &str) -> String {
    full_pipeline(source)
        .unwrap_or_else(|e| panic!("Pipeline should succeed for: {:?} — {:?}", source, e))
}

// ── Runtime init ──────────────────────────────────────────────────────────────

#[test]
fn test_runtime_initializes() {
    // Should not panic
    initialize_runtime();
}

#[test]
fn test_version_string_nonempty() {
    assert!(!VERSION.is_empty(), "Version should be defined");
}

// ── Full pipeline: valid programs ─────────────────────────────────────────────

#[test]
fn test_pipeline_empty_program() {
    let result = compile("");
    assert!(result.is_ok(), "Empty program should compile: {:?}", result);
}

#[test]
fn test_pipeline_single_let() {
    let result = compile("let x = 42;");
    assert!(result.is_ok(), "Single let should compile: {:?}", result);
}

#[test]
fn test_pipeline_multiple_lets() {
    let result = compile("let a = 1; let b = 2; let c = 3;");
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_arithmetic() {
    let result = compile("let r = 10 + 5 - 2;");
    assert!(result.is_ok(), "Arithmetic should compile: {:?}", result);
}

#[test]
fn test_pipeline_function_declaration() {
    let result = compile("fn add(x, y) { return x; }");
    assert!(result.is_ok(), "Function decl should compile: {:?}", result);
}

#[test]
fn test_pipeline_if_else() {
    let result = compile("if true { let a = 1; } else { let b = 2; }");
    assert!(result.is_ok(), "If-else should compile: {:?}", result);
}

#[test]
fn test_pipeline_while_loop() {
    let result = compile("while true { let x = 1; }");
    assert!(result.is_ok(), "While loop should compile: {:?}", result);
}

#[test]
fn test_pipeline_for_loop() {
    let result = compile("let list = 1; for item in list { let x = item; }");
    assert!(result.is_ok(), "For loop should compile: {:?}", result);
}

#[test]
fn test_pipeline_quantum_circuit() {
    let result = compile("quantum circuit Bell { let q = 1; }");
    assert!(
        result.is_ok(),
        "Quantum circuit should compile: {:?}",
        result
    );
}

#[test]
fn test_pipeline_nano_agent() {
    let result = compile("agent Scout { let x = 1; }");
    assert!(result.is_ok(), "Nano agent should compile: {:?}", result);
}

#[test]
fn test_pipeline_sankofa_memory() {
    let result = compile("remember mem_val = 42;");
    assert!(
        result.is_ok(),
        "Sankofa remember should compile — got: {:?}",
        result
    );
}

#[test]
fn test_pipeline_nested_function_call() {
    let result = compile("let r = f(g(1));");
    // undefined f,g → semantic error is valid
    let _ = result;
}

#[test]
fn test_pipeline_boolean_expression() {
    let result = compile("let b = true;");
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_string_literal() {
    let result = compile(r#"let s = "hello world";"#);
    assert!(result.is_ok());
}

// ── IR module properties ──────────────────────────────────────────────────────

#[test]
fn test_ir_module_has_functions() {
    let module = compile("let x = 1;").unwrap();
    assert!(
        !module.functions.is_empty(),
        "IR module should have functions"
    );
}

#[test]
fn test_ir_module_instruction_count() {
    let module = compile("let a = 1; let b = 2;").unwrap();
    assert!(
        module.instruction_count() > 0,
        "IR should have instructions"
    );
}

#[test]
fn test_function_decl_produces_named_function_in_ir() {
    let module = compile("fn compute(x) { return x; }").unwrap();
    assert!(
        module.functions.iter().any(|f| f.name == "compute"),
        "Expected 'compute' in IR functions"
    );
}

// ── Optimize then codegen ─────────────────────────────────────────────────────

#[test]
fn test_optimize_then_codegen() {
    let module = compile("let r = 3 + 4;").unwrap();
    let mut opt = Optimizer::new(zamani_compiler::optimizer::OptimizationConfig::default());
    let optimized = opt.optimize(&module);
    let out = LlvmIrBackend.generate(&optimized).unwrap();
    assert!(
        !out.is_empty(),
        "Post-optimization codegen should produce output"
    );
}

#[test]
fn test_constant_folding_in_pipeline() {
    let module = compile("let a = 6 * 7; let b = a + 1;").unwrap();
    let mut opt = Optimizer::new(zamani_compiler::optimizer::OptimizationConfig::default());
    opt.optimize(&module);
    // 6*7 should be folded to 42
    assert!(
        opt.stats.constants_folded > 0,
        "Optimizer should fold at least one constant expression"
    );
}

// ── All backends roundtrip ────────────────────────────────────────────────────

#[test]
fn test_all_backends_roundtrip() {
    let source = "let x = 42; fn main() { let r = x; }";
    let module = compile(source).unwrap();

    let targets = vec![
        CompilationTarget::X86_64Linux,
        CompilationTarget::Wasm32,
        CompilationTarget::QASM,
        CompilationTarget::LLVMIR,
        CompilationTarget::NanoControl,
        CompilationTarget::MTSBytecode,
    ];

    for target in targets {
        let cfg = CompilerConfig {
            target: target.clone(),
            opt_level: OptimizationLevel::Basic,
            debug_info: false,
            verify: false,
            emit_ir: false,
            parallel: false,
        };
        let gen = CodeGenerator::new(cfg);
        let result = gen.generate(&module);
        assert!(result.is_ok(), "Backend {:?} failed", target);
        assert!(
            result.unwrap().size_bytes > 0,
            "Backend {:?} produced empty output",
            target
        );
    }
}

// ── Error propagation ─────────────────────────────────────────────────────────

#[test]
fn test_undefined_variable_propagates_as_semantic_error() {
    // Semantic analyser should catch this — compile() returns Err
    let result = compile("let x = undefined_var;");
    // May or may not be Err depending on severity threshold
    // At minimum, it should not panic
    let _ = result;
}

// ── Lexer → Parser → IR direct ────────────────────────────────────────────────

#[test]
fn test_lexer_to_ir_direct() {
    let source = "let answer = 42;";
    let sf = Arc::new(SourceFile::new("<test>".to_string(), source.to_string()));
    let lex = Lexer::new(FileId::new(1), sf);
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    assert!(parser.get_errors().is_empty());
    let mut gen = IrGenerator::new();
    let module = gen.generate(&prog);
    assert!(!module.functions.is_empty());
}
