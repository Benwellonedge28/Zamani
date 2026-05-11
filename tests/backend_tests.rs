
//! Conceptual Tests: Backend Code Generation
//!
//! This module provides conceptual unit tests for the Zenith Backend Code Generators.
//! It verifies that the various backend implementations correctly translate
//! Universal Meta-Compiler (UMC) Intermediate Representation (IR) into
//! target-specific code, such as x86_64 assembly, QASM for quantum circuits,
//! NanoControl sequences for nano-agents, and MTS Runtime Bytecode.

use zenith_compiler::ir_gen::{IrInstruction, IrValue, IrRegister, IrType};
use zenith_compiler::backend::{UMC_Backend, X86_64_Generator, QASM_Generator, NanoControlGenerator, MTS_RuntimeBytecode_Generator};
use zenith_compiler::ast::Literal;
use zenith_compiler::source_map::{FileId, Span, BytePos};
use std::collections::HashMap;

// Helper for dummy Span
fn dummy_span() -> Span { Span::new(FileId::new(1), BytePos(0), BytePos(0), 1, 1) }

// Helper for dummy IrRegister
fn dummy_reg(id: usize) -> IrRegister { IrRegister(id) }

#[test]
fn test_backend_x86_64_generation() {
    let ir_code = vec![
        IrInstruction::Label("func_main".to_string()),
        IrInstruction::Alloc(dummy_reg(0), IrType::I32),
        IrInstruction::Add(dummy_reg(1), IrValue::Literal(Literal::Integer("10".to_string(), dummy_span())), IrValue::Literal(Literal::Integer("20".to_string(), dummy_span()))),
        IrInstruction::Store(IrValue::Register(dummy_reg(1)), IrValue::Register(dummy_reg(0))),
        IrInstruction::Return(Some(IrValue::Register(dummy_reg(0)))),
    ];

    let mut backend = UMC_Backend::new();
    backend.register_generator(X86_64_Generator);
    let code = backend.generate(&ir_code, "x86_64").expect("x86_64 generation should succeed");
    
    let assembly = String::from_utf8(code).expect("Should be valid UTF-8");
    println!("Generated x86_64:\n{}", assembly);

    assert!(assembly.contains(".section .text"));
    assert!(assembly.contains("_start:"));
    assert!(assembly.contains("; Add instruction:"));
    assert!(assembly.contains("syscall"));
}

#[test]
fn test_backend_qasm_generation() {
    let ir_code = vec![
        IrInstruction::QAlloc(dummy_reg(0), IrValue::Literal(Literal::Integer("2".to_string(), dummy_span()))), // Allocate 2 qubits
        IrInstruction::QGate(dummy_reg(0), "H".to_string(), vec![IrValue::Register(dummy_reg(0))]), // H on q[0]
        IrInstruction::QGate(dummy_reg(0), "CNOT".to_string(), vec![IrValue::Register(dummy_reg(0)), IrValue::Register(dummy_reg(1))]), // CNOT(q[0], q[1])
        IrInstruction::QMeasure(dummy_reg(2), IrValue::Register(dummy_reg(0))), // Measure q[0] to c[0]
        IrInstruction::QMeasure(dummy_reg(3), IrValue::Register(dummy_reg(1))), // Measure q[1] to c[1]
    ];

    let mut backend = UMC_Backend::new();
    backend.register_generator(QASM_Generator);
    let code = backend.generate(&ir_code, "QASM").expect("QASM generation should succeed");
    
    let qasm = String::from_utf8(code).expect("Should be valid UTF-8");
    println!("Generated QASM:\n{}", qasm);

    assert!(qasm.contains("OPENQASM 2.0;"));
    assert!(qasm.contains("qreg q[2];")); // Based on QAlloc(size=2)
    assert!(qasm.contains("creg c[2];")); // Based on 2 QMeasure
    assert!(qasm.contains("h_gate q[0];"));
    assert!(qasm.contains("cnot_gate q[0];")); // Simplified, actual CNOT syntax is different
    assert!(qasm.contains("measure q[0] -> c["));
}

#[test]
fn test_backend_nano_control_generation() {
    let ir_code = vec![
        IrInstruction::NanoAssemble(dummy_reg(0), IrValue::Literal(Literal::String("basic_unit".to_string(), dummy_span())), vec![]),
        IrInstruction::NanoCommunicate(IrValue::Register(dummy_reg(0)), IrValue::Register(dummy_reg(1)), IrValue::Literal(Literal::String("hello".to_string(), dummy_span()))),
        IrInstruction::NanoReplicate(dummy_reg(2), IrValue::Register(dummy_reg(0))),
    ];

    let mut backend = UMC_Backend::new();
    backend.register_generator(NanoControlGenerator);
    let code = backend.generate(&ir_code, "NanoControl").expect("NanoControl generation should succeed");
    
    let nano_control = String::from_utf8(code).expect("Should be valid UTF-8");
    println!("Generated NanoControl:\n{}", nano_control);

    assert!(nano_control.contains("START_NANO_AGENT_PROGRAM"));
    assert!(nano_control.contains("ASSEMBLE_AGENT"));
    assert!(nano_control.contains("AGENT_COMMUNICATE"));
    assert!(nano_control.contains("REPLICATE_AGENT"));
}

#[test]
fn test_backend_mts_bytecode_generation() {
    let ir_code = vec![
        IrInstruction::MTSCreate(dummy_reg(0), IrValue::Literal(Literal::Integer("0".to_string(), dummy_span()))),
        IrInstruction::MTSStore(IrValue::Register(dummy_reg(0)), IrValue::Literal(Literal::Integer("100".to_string(), dummy_span())), IrValue::Literal(Literal::Integer("50".to_string(), dummy_span()))),
        IrInstruction::MTSLoad(dummy_reg(1), IrValue::Register(dummy_reg(0)), IrValue::Literal(Literal::Integer("25".to_string(), dummy_span()))),
    ];

    let mut backend = UMC_Backend::new();
    backend.register_generator(MTS_RuntimeBytecode_Generator);
    let code = backend.generate(&ir_code, "MTS_Bytecode").expect("MTS Bytecode generation should succeed");
    
    let mts_bytecode = String::from_utf8(code).expect("Should be valid UTF-8");
    println!("Generated MTS Bytecode:\n{}", mts_bytecode);

    assert!(mts_bytecode.contains("MTS_PROGRAM_START"));
    assert!(mts_bytecode.contains("CREATE_TIMELINE_SLICE"));
    assert!(mts_bytecode.contains("STORE_TIMELINE_STATE"));
    assert!(mts_bytecode.contains("LOAD_TIMELINE_STATE"));
}

#[test]
fn test_backend_unsupported_target_error() {
    let ir_code = vec![IrInstruction::NoOp];
    let mut backend = UMC_Backend::new();
    // Do not register any generators
    let result = backend.generate(&ir_code, "unsupported_target");
    
    assert!(result.is_err(), "Generating code for an unsupported target should fail.");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("No code generator registered for target"));
}
