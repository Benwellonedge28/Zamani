
//! Zenith Universal Meta-Compiler (UMC) - Main Entry Point
//!
//! This module serves as the main entry point and orchestrator for the
//! Zenith Universal Meta-Compiler. It ties together all the conceptual
//! phases of the compiler: lexical analysis, parsing, semantic analysis,
//! IR generation, optimization, and backend code generation.

use std::collections::HashMap;

use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::ir_gen::{IrGenerator, IrInstruction};
use crate::optimizer::{UMC_Optimizer, CSE_Pass, DCE_Pass, QGateCancellationPass, NanoResourceOptimizer, MTSTimelineFusionPass, SankofaAccessOptimizer, ResourceManagementOptimizer};
use crate::backend::{UMC_Backend, X86_64_Generator, QASM_Generator, NanoControlGenerator, MTS_RuntimeBytecode_Generator};
use crate::runtime;
use crate::stdlib;
use crate::toolchain;
use crate::source_map::FileId;
use crate::error_reporting::{CompilerError, Severity};

fn main() -> Result<(), String> {
    println!("-----------------------------------------------------");
    println!(" Zenith Universal Meta-Compiler (UMC) - Conceptual Run");
    println!("-----------------------------------------------------");

    // 1. Initialize Runtime, Standard Library, and Toolchain (Conceptual)
    runtime::init_runtime();
    stdlib::initialize_stdlib();
    toolchain::init_toolchain_integration();
    println!("
--- Compiler Pipeline Start ---
");

    // 2. Conceptual Zenith Source Code
    let source_code = r#"
        // A simple Zenith program demonstrating unified paradigms.
        fn add(a: int, b: int) -> int {
            let result = a + b;
            return result;
        }

        quantum circuit MyQuantumAlgorithm {
            let q = QReg[2];
            q.get_mut(0).h();
            q.get_mut(0).cnot(q.get_mut(1));
            let classical_result = q.get_mut(0).measure();
        }

        nano agent BasicHarvester {
            let my_agent = NanoAgent::assemble("blueprint", ["sensor"]);
            my_agent.perform_action("harvest");
        }

        remember my_fact = "Zenith is universal";
        
        fn main() -> int {
            let x = add(5, 3);
            MyQuantumAlgorithm();
            BasicHarvester();
            return 0;
        }
    "#;

    let mut compiler_errors: Vec<CompilerError> = Vec::new();
    let file_id = FileId::new(1);

    // 3. Lexical Analysis
    println!("Lexing source code...");
    let lexer = Lexer::new(file_id, source_code);
    let _tokens: Vec<_> = lexer.clone().collect(); // Consume copy to check for errors
    if !lexer.get_errors().is_empty() {
        compiler_errors.extend(lexer.get_errors().iter().cloned().map(CompilerError::Lexer));
    }

    // 4. Parsing
    println!("Parsing tokens into AST...");
    let mut parser = Parser::new(Lexer::new(file_id, source_code));
    let program_ast = parser.parse_program();
    if !parser.get_errors().is_empty() {
        compiler_errors.extend(parser.get_errors().iter().cloned().map(CompilerError::Parser));
    }

    // 5. Semantic Analysis
    println!("Performing semantic analysis...");
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&program_ast);
    if let Err(errors) = semantic_result {
        compiler_errors.extend(errors.into_iter().map(CompilerError::Semantic));
    }

    // 6. IR Generation
    println!("Generating Universal Meta-Compiler IR...");
    let mut ir_generator = IrGenerator::new();
    let symbol_table_snapshot = semantic_analyzer.get_global_symbols().clone();
    let mut ir_code = match ir_generator.generate_ir(&program_ast, &symbol_table_snapshot) {
        Ok(code) => code,
        Err(errors) => {
            compiler_errors.extend(errors.into_iter().map(CompilerError::IrGen));
            Vec::new() // Placeholder, won't be used if errors exist
        }
    };

    // --- Critical Error Check Before Optimization ---
    if !compiler_errors.is_empty() {
        eprintln!("
Compilation failed with {} errors:
", compiler_errors.len());
        for err in compiler_errors {
            eprintln!("{}", err.report(source_code));
        }
        return Err("Compiler pipeline failed due to semantic or earlier errors.".to_string());
    }

    // 7. Optimization
    println!("Optimizing UMC IR...");
    let mut optimizer = UMC_Optimizer::new();
    optimizer.add_pass(CSE_Pass);
    optimizer.add_pass(DCE_Pass);
    optimizer.add_pass(QGateCancellationPass);
    optimizer.add_pass(NanoResourceOptimizer);
    optimizer.add_pass(MTSTimelineFusionPass);
    optimizer.add_pass(SankofaAccessOptimizer);
    optimizer.add_pass(ResourceManagementOptimizer);
    
    let metrics = optimizer.optimize(&mut ir_code)
        .map_err(|e| {
            for err in e { eprintln!("{}", CompilerError::Optimizer(err).report(source_code)); }
            "Optimization failed.".to_string()
        })?;
    println!("Optimization complete. Changes: {}, Instructions (before/after): {}/{}", metrics.total_changes_made, metrics.instruction_count_before, metrics.instruction_count_after);

    // 8. Backend Code Generation
    println!("Generating target-specific code...");
    let mut backend = UMC_Backend::new();
    backend.register_generator(X86_64_Generator);
    backend.register_generator(QASM_Generator);
    backend.register_generator(NanoControlGenerator);
    backend.register_generator(MTS_RuntimeBytecode_Generator);

    let targets = ["x86_64", "QASM", "NanoControl", "MTS_Bytecode"];
    for target in targets {
        match backend.generate(&ir_code, target) {
            Ok(code) => println!("  Generated {} bytes of {} code.", code.len(), target),
            Err(errors) => {
                for err in errors { eprintln!("{}", CompilerError::Backend(err).report(source_code)); }
            }
        }
    }

    println!("
--- Compiler Pipeline End ---
");

    // 9. Shutdown Runtime, Standard Library, and Toolchain (Conceptual)
    runtime::shutdown_runtime();
    stdlib::shutdown_stdlib();
    toolchain::shutdown_toolchain_integration();
    println!("
Zenith UMC Conceptual Run Complete.
");

    Ok(())
}
