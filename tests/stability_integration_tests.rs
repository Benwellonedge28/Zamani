//! Cross-Subsystem Stability Integration Tests for Zamani
//! Validates Quantum Noise Models, Cognitive Vetting, HDL Synthesis, and Knowledge Fabric.

use zamani::ast::*;
use zamani::source_map::Span;
use zamani::ai::cognitive_engine::CognitiveEngine;
use zamani::distributed::omni_exec::DistributedExecutor;
use zamani::quantum::stabilizer_scheduler::StabilizerScheduler;
use zamani::ir_gen::{IrFunction, IrModule};

#[test]
fn test_cognitive_alignment_vetting() {
    // Valid cognitive block
    let valid_stmts = vec![
        Statement::SankofaMemory(Span::default(), "safe_memory".to_string(), Expression::Literal(Literal::Integer(42, Span::default())))
    ];
    let result_valid = CognitiveEngine::verify_alignment("SafeNexus", &valid_stmts);
    assert!(result_valid.is_ok(), "Safe cognitive block should pass alignment vetting.");

    // Unsafe / Rogue cognitive block
    let rogue_stmts = vec![
        Statement::Unsafe(Span::default(), None, Expression::Literal(Literal::Integer(0, Span::default())))
    ];
    let result_rogue = CognitiveEngine::verify_alignment("RogueNexus", &rogue_stmts);
    assert!(result_rogue.is_err(), "Unsafe cognitive block must be rejected by alignment vetting.");
}

#[test]
fn test_hdl_synthesis_generation() {
    let stmts = vec![
        Statement::Let(Span::default(), "control_signal".to_string(), None, Expression::Literal(Literal::Integer(1, Span::default())))
    ];
    let verilog = DistributedExecutor::synthesize_from_ast("QpuController", &stmts);
    
    assert!(verilog.contains("module QpuController"), "Verilog should contain module declaration.");
    assert!(verilog.contains("control_signal"), "Verilog should contain output signal from let statement.");
    assert!(verilog.contains("always @(posedge clk or negedge rst_n)"), "Verilog should contain sequential logic.");
}

#[test]
fn test_quantum_stabilizer_scheduling() {
    let mut func = IrFunction::new("test_circuit".into());
    let scheduler = StabilizerScheduler::new("SurfacePatch7x7", 3);
    scheduler.schedule_rounds(&mut func, 2);

    let mut found_x = false;
    let mut found_z = false;

    for inst in &func.instructions {
        if let zamani::ir_gen::IrInstruction::Comment(c) = inst {
            if c.contains("X-Stabilizers") { found_x = true; }
            if c.contains("Z-Stabilizers") { found_z = true; }
        }
    }

    assert!(found_x, "Stabilizer scheduler must include X-stabilizer rounds.");
    assert!(found_z, "Stabilizer scheduler must include Z-stabilizer rounds.");
}
