
//! Conceptual Test: Full Compiler Pipeline
//!
//! This module provides a high-level conceptual test for the entire Zenith
//! Universal Meta-Compiler (UMC) pipeline. It simulates the compilation
//! of a Zenith source code example through all stages (Lexer, Parser,
//! Semantic Analyzer, IR Generator, Optimizer, Backend) and verifies
//! that the process completes without unexpected errors and produces
//! conceptually valid outputs at each stage.

// Assuming the compiler components are accessible from the crate root
use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::SemanticAnalyzer;
use zenith_compiler::ir_gen::IrGenerator;
use zenith_compiler::optimizer::{UMC_Optimizer, CSE_Pass, DCE_Pass, QGateCancellationPass, NanoResourceOptimizer, MTSTimelineFusionPass, SankofaAccessOptimizer, ResourceManagementOptimizer};
use zenith_compiler::backend::{UMC_Backend, X86_64_Generator, QASM_Generator, NanoControlGenerator, MTS_RuntimeBytecode_Generator};
use zenith_compiler::source_map::FileId;

#[test]
fn test_full_compiler_pipeline_simple_zenith_program() {
    let source_code = r#"
        fn main() -> int {
            let x: int = 10;
            let y: int = add(x, 5);
            return y;
        }

        fn add(a: int, b: int) -> int {
            return a + b;
        }
    "#;

    let file_id = FileId::new(1);

    // 1. Lexical Analysis
    let lexer = Lexer::new(file_id, source_code);
    let tokens: Vec<_> = lexer.collect();
    assert!(lexer.get_errors().is_empty(), "Lexer should not have errors: {:?}", lexer.get_errors());
    assert!(!tokens.is_empty(), "Lexer should produce tokens.");

    // 2. Parsing
    let mut parser = Parser::new(Lexer::new(file_id, source_code));
    let program_ast = parser.parse_program();
    assert!(parser.get_errors().is_empty(), "Parser should not have errors: {:?}", parser.get_errors());
    assert!(!program_ast.statements.is_empty(), "Parser should produce AST statements.");

    // 3. Semantic Analysis
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&program_ast);
    assert!(semantic_result.is_ok(), "Semantic analyzer should not have errors: {:?}", semantic_result.unwrap_err());
    let symbol_table = semantic_analyzer.get_global_symbols().clone();
    assert!(!symbol_table.is_empty(), "Semantic analyzer should populate symbol table.");

    // 4. IR Generation
    let mut ir_generator = IrGenerator::new();
    let mut ir_code = ir_generator.generate_ir(&program_ast, &symbol_table)
        .expect("IR generation should succeed");
    assert!(!ir_code.is_empty(), "IR generator should produce instructions.");

    // 5. Optimization
    let mut optimizer = UMC_Optimizer::new();
    optimizer.add_pass(CSE_Pass);
    optimizer.add_pass(DCE_Pass);
    let metrics = optimizer.optimize(&mut ir_code)
        .expect("Optimizer should succeed");
    println!("Optimizer metrics: {:?}", metrics);
    // Assert some basic properties, e.g., instruction count might change after DCE
    assert!(ir_code.len() <= metrics.instruction_count_before, "Optimizer should not increase IR size for simple passes.");

    // 6. Backend Code Generation (Conceptual)
    let mut backend = UMC_Backend::new();
    backend.register_generator(X86_64_Generator);
    
    let x86_code = backend.generate(&ir_code, "x86_64")
        .expect("x86_64 backend should generate code");
    assert!(!x86_code.is_empty(), "x86_64 backend should produce bytes.");

    println!("Full compiler pipeline test passed for simple program.");
}

#[test]
fn test_full_compiler_pipeline_zenith_multi_paradigm_program() {
    let source_code = r#"
        // Multi-paradigm Zenith program
        quantum circuit EntangleTwoQubits {
            let q1 = Qubit::new();
            let q2 = Qubit::new();
            q1.h();
            q1.cnot(q2);
            // Implicit return, or measurement as part of circuit
        }

        nano agent CleanUpCrew {
            let blueprint = "disassemble_waste";
            let components = ["gripper", "scanner"];
            let cleaner = NanoAgent::assemble(blueprint, components);
            cleaner.perform_action("scan_area");
        }

        remember historical_event = "First quantum entanglement achieved";
        let event_record = ZamaniFact::access("historical_event");

        fn main() -> int {
            EntangleTwoQubits();
            CleanUpCrew();
            if event_record.is_some() {
                stdlib::core::println("Retrieved historical event: " + event_record.unwrap().get_content::<String>());
            }
            return 0;
        }
    "#;

    let file_id = FileId::new(1);

    // 1. Lexical Analysis
    let lexer = Lexer::new(file_id, source_code);
    let tokens: Vec<_> = lexer.collect();
    assert!(lexer.get_errors().is_empty(), "Lexer should not have errors: {:?}", lexer.get_errors());

    // 2. Parsing
    let mut parser = Parser::new(Lexer::new(file_id, source_code));
    let program_ast = parser.parse_program();
    assert!(parser.get_errors().is_empty(), "Parser should not have errors: {:?}", parser.get_errors());

    // 3. Semantic Analysis
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&program_ast);
    assert!(semantic_result.is_ok(), "Semantic analyzer should not have errors: {:?}", semantic_result.unwrap_err());

    // 4. IR Generation
    let mut ir_generator = IrGenerator::new();
    let symbol_table = semantic_analyzer.get_global_symbols().clone();
    let mut ir_code = ir_generator.generate_ir(&program_ast, &symbol_table)
        .expect("IR generation should succeed");
    assert!(!ir_code.is_empty(), "IR generator should produce instructions.");

    // 5. Optimization (with specialized passes)
    let mut optimizer = UMC_Optimizer::new();
    optimizer.add_pass(QGateCancellationPass);
    optimizer.add_pass(NanoResourceOptimizer);
    let metrics = optimizer.optimize(&mut ir_code)
        .expect("Optimizer should succeed");
    println!("Optimizer metrics for multi-paradigm: {:?}", metrics);

    // 6. Backend Code Generation for multiple targets
    let mut backend = UMC_Backend::new();
    backend.register_generator(X86_64_Generator);
    backend.register_generator(QASM_Generator);
    backend.register_generator(NanoControlGenerator);

    let _ = backend.generate(&ir_code, "x86_64")
        .expect("x86_64 backend should generate code");
    let _ = backend.generate(&ir_code, "QASM")
        .expect("QASM backend should generate code");
    let _ = backend.generate(&ir_code, "NanoControl")
        .expect("NanoControl backend should generate code");
    
    println!("Full compiler pipeline test passed for multi-paradigm program.");
}

#[test]
fn test_compiler_pipeline_error_handling() {
    let source_code = r#"
        fn main() {
            let x: unknown_type = 10; // Semantic error: unknown type
            let y = 10 + "hello";    // Semantic error: type mismatch
            quantum circuit InvalidQubitOps {
                let q = Qubit::new();
                q.invalid_method(); // Semantic error: no such method
            }
        }
    "#;

    let file_id = FileId::new(1);

    // Lexing should still work
    let lexer = Lexer::new(file_id, source_code);
    let _tokens: Vec<_> = lexer.collect();
    assert!(lexer.get_errors().is_empty(), "Lexer should not have errors for valid tokens.");

    // Parsing should still work
    let mut parser = Parser::new(Lexer::new(file_id, source_code));
    let program_ast = parser.parse_program();
    assert!(parser.get_errors().is_empty(), "Parser should not have syntax errors here.");

    // Semantic Analysis should find errors
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&program_ast);
    assert!(semantic_result.is_err(), "Semantic analyzer should report errors.");
    let errors = semantic_result.unwrap_err();
    assert!(errors.len() >= 3, "Expected at least 3 semantic errors, got {}", errors.len());
    println!("Found expected semantic errors: {:?}", errors);

    // Subsequent stages might fail or receive empty IR if semantic analysis fails critically.
    // For this conceptual test, just ensure semantic errors are caught.
    println!("Compiler pipeline error handling test passed.");
}
