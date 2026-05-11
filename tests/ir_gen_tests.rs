
//! Conceptual Tests: IR Generator
//!
//! This module provides conceptual unit tests for the Zenith IR Generator.
//! It verifies that the generator correctly translates various AST nodes
//! into the appropriate Universal Meta-Compiler (UMC) Intermediate Representation (IR)
//! instructions. Special emphasis is placed on Zenith-specific instructions for
//! quantum, nano, MTS, and Sankofa paradigms.

use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::SemanticAnalyzer;
use zenith_compiler::ir_gen::{IrGenerator, IrInstruction, IrValue, IrRegister, IrType};
use zenith_compiler::source_map::{FileId, Span, BytePos};
use zenith_compiler::compiler_types::{Symbol, Type, IntWidth};
use zenith_compiler::ast::Literal;
use std::collections::HashMap;

// Helper function for creating a FileId for tests
fn test_file_id() -> FileId { FileId::new(1) }
// Helper to create dummy Span for comparisons where exact span details don't matter
fn dummy_span() -> Span { Span::new(test_file_id(), BytePos(0), BytePos(0), 1, 1) }

fn generate_ir_and_check(input: &str) -> Result<Vec<IrInstruction>, Vec<String>> {
    let file_id = test_file_id();
    let lexer = Lexer::new(file_id, input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.get_errors().is_empty() {
        return Err(parser.get_errors().into_iter().map(|e| e.message).collect());
    }

    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(&program);
    if let Err(errors) = semantic_result {
        return Err(errors.into_iter().map(|e| e.message).collect());
    }
    let symbol_table = semantic_analyzer.get_global_symbols().clone();

    let mut ir_generator = IrGenerator::new();
    ir_generator.generate_ir(&program, &symbol_table)
        .map_err(|errors| errors.into_iter().map(|e| e.message).collect())
}

#[test]
fn test_ir_gen_basic_arithmetic() {
    let input = "fn main() { let x = 1 + 2; }";
    let ir = generate_ir_and_check(input).expect("IR generation for basic arithmetic should succeed");

    // Expect instructions like:
    // Label(func_main)
    // Alloc(R0, I32)
    // Add(R1, Literal(1), Literal(2))
    // Store(R1, R0)
    // Return(None)
    assert!(ir.len() >= 5); // Label, Alloc, Add, Store, Return
    assert!(ir.contains(&IrInstruction::Add(IrRegister(1), IrValue::Literal(Literal::Integer("1".to_string(), dummy_span())), IrValue::Literal(Literal::Integer("2".to_string(), dummy_span())))));
}

#[test]
fn test_ir_gen_quantum_circuit() {
    let input = r#"
        quantum circuit MyQuantumAlg {
            let q1 = Qubit::new();
            q1.h();
            q1.measure();
        }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for quantum circuit should succeed");

    // Expect instructions like:
    // Label(qcirc_MyQuantumAlg)
    // QGate(R_result, "H", [R_q1])
    // QMeasure(R_classical_result, R_q1)
    assert!(ir.contains(&IrInstruction::Label("qcirc_MyQuantumAlg".to_string())));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::QGate(_, g, _) if g == "H")));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::QMeasure(_, _))));
}

#[test]
fn test_ir_gen_nano_agent() {
    let input = r#"
        nano agent MyNano {
            let blueprint = "basic";
            NanoAgent::assemble(blueprint, ["sensor"]);
        }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for nano agent should succeed");

    // Expect instructions like:
    // Label(nano_MyNano)
    // NanoAssemble(R_agent, blueprint_val, [sensor_val])
    assert!(ir.contains(&IrInstruction::Label("nano_MyNano".to_string())));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::NanoAssemble(_, _, _))));
}

#[test]
fn test_ir_gen_mts_operations() {
    let input = r#"
        fn main() {
            let slice = MtsSlice::new(10);
            slice.store(20, 100);
            let val = slice.load(50);
        }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for MTS operations should succeed");

    // Expect instructions like:
    // MTSCreate(R_slice, Literal(10))
    // MTSStore(R_slice, Literal(20), Literal(100))
    // MTSLoad(R_val, R_slice, Literal(50))
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::MTSCreate(_, _))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::MTSStore(_, _, _))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::MTSLoad(_, _, _))));
}

#[test]
fn test_ir_gen_sankofa_memory() {
    let input = r#"
        remember my_fact = "data";
        fn main() {
            let data = ZamaniFact::access("my_fact");
            SasaKnowledge::update("my_sasa_knowledge", 100);
        }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for Sankofa memory should succeed");

    // Expect instructions like:
    // WriteHistory("my_fact", Literal("data"), "current_timestamp")
    // AccessZamani(R_data, "my_fact")
    // WriteHistory("my_sasa_knowledge", Literal(100), "current_timestamp")
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::WriteHistory(k, _, _) if k == "my_fact")));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::AccessZamani(_, k) if k == "my_fact")));
}

#[test]
fn test_ir_gen_effect_handling() {
    let input = r#"
        effect MyEffect;
        fn performer() { perform MyEffect("error"); }
        handle MyEffect { performer(); } with { |msg: String| { return 0; } }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for effect handling should succeed");

    // Expect instructions like:
    // Label(func_performer)
    // EffectOp(R_result, "MyEffect", [Literal("error")])
    // HandleEffect("MyEffect", body_label, handler_label)
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::EffectOp(_, eff_name, _) if eff_name == "MyEffect")));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Label(l) if l.starts_with("effect_handler_MyEffect"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::HandleEffect(eff_name, _, _) if eff_name == "MyEffect")));
}

#[test]
fn test_ir_gen_if_expression() {
    let input = "fn main() { let x = if true { 1 } else { 0 }; }";
    let ir = generate_ir_and_check(input).expect("IR generation for if expression should succeed");

    // Expect Branch, Label, Jump instructions for if/else structure
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Branch(_, _, _))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Label(l) if l.starts_with("if_then"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Label(l) if l.starts_with("if_else"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Jump(l) if l.starts_with("if_end"))));
}

#[test]
fn test_ir_gen_loops_and_control_flow() {
    let input = r#"
        fn main() {
            while true { break; }
            for i in [1, 2, 3] { continue; }
        }
    "#;
    let ir = generate_ir_and_check(input).expect("IR generation for loops should succeed");

    // Expect labels and jumps for while loop
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Label(l) if l.starts_with("while_loop_start"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Branch(_, _, _))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Jump(l) if l.starts_with("while_loop_end"))));

    // Expect labels and jumps for for loop, and iterator calls
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Label(l) if l.starts_with("for_loop_start"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Call(_, f, _) if f.contains("__get_iterator"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Call(_, f, _) if f.contains("__has_next"))));
    assert!(ir.iter().any(|instr| matches!(instr, IrInstruction::Call(_, f, _) if f.contains("__next"))));
}
