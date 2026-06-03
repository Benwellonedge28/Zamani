#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unreachable_code,
    unused_comparisons
)]

//! Zenith Backend — Comprehensive Integration Tests

use zenith_compiler::backend::{
    Backend, CodeGenerator, LlvmIrBackend, MtsBytecodeBackend, NanoControlBackend, QasmBackend,
    WasmBackend, X86Backend,
};
use zenith_compiler::compiler_types::{CompilationTarget, CompilerConfig, OptimizationLevel};
use zenith_compiler::ir_gen::{IrFunction, IrInstruction, IrModule, IrRegister, IrType, IrValue};

fn make_module(instructions: Vec<IrInstruction>) -> IrModule {
    let mut func = IrFunction::new("main", IrType::Unit);
    for ins in instructions {
        func.push(ins);
    }
    let mut m = IrModule::new();
    m.add_function(func);
    m
}

fn reg(name: &str) -> IrRegister {
    IrRegister(name.to_string())
}
fn ci(n: i64) -> IrValue {
    IrValue::ConstInt(n)
}

fn config_for(target: CompilationTarget) -> CompilerConfig {
    CompilerConfig {
        target,
        opt_level: OptimizationLevel::None,
        debug_info: false,
        verify: false,
        emit_ir: false,
        parallel: false,
    }
}

// ── X86-64 backend ────────────────────────────────────────────────────────────

#[test]
fn test_x86_backend_produces_output() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = X86Backend.generate(&m).unwrap();
    assert!(
        !out.is_empty(),
        "X86 backend should produce non-empty output"
    );
}

#[test]
fn test_x86_has_text_section() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = X86Backend.generate(&m).unwrap();
    assert!(out.contains(".section .text"), "Expected .text section");
}

#[test]
fn test_x86_has_global_start() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = X86Backend.generate(&m).unwrap();
    assert!(
        out.contains(".global _start") || out.contains("_start"),
        "Expected _start"
    );
}

#[test]
fn test_x86_integer_assign() {
    let m = make_module(vec![IrInstruction::Assign(reg("%r0"), ci(42))]);
    let out = X86Backend.generate(&m).unwrap();
    assert!(out.contains("42"), "Expected constant 42 in X86 output");
}

#[test]
fn test_x86_function_name_emitted() {
    let m = make_module(vec![IrInstruction::Nop]);
    let out = X86Backend.generate(&m).unwrap();
    assert!(out.contains("main"), "Expected function name in X86 output");
}

#[test]
fn test_x86_file_extension() {
    assert_eq!(X86Backend.file_extension(), "s");
}

// ── WASM backend ──────────────────────────────────────────────────────────────

#[test]
fn test_wasm_produces_module() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = WasmBackend.generate(&m).unwrap();
    assert!(out.contains("(module"), "Expected WASM module declaration");
}

#[test]
fn test_wasm_has_func() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = WasmBackend.generate(&m).unwrap();
    assert!(out.contains("(func"), "Expected WASM func declaration");
}

#[test]
fn test_wasm_integer_const() {
    let m = make_module(vec![IrInstruction::Assign(reg("%r0"), ci(99))]);
    let out = WasmBackend.generate(&m).unwrap();
    assert!(out.contains("99"), "Expected constant in WASM output");
}

#[test]
fn test_wasm_file_extension() {
    assert_eq!(WasmBackend.file_extension(), "wat");
}

// ── QASM backend ──────────────────────────────────────────────────────────────

#[test]
fn test_qasm_has_openqasm_header() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = QasmBackend.generate(&m).unwrap();
    assert!(out.contains("OPENQASM"), "Expected OPENQASM header");
}

#[test]
fn test_qasm_qubit_allocation() {
    let m = make_module(vec![IrInstruction::QAlloc(reg("%q0"), 4)]);
    let out = QasmBackend.generate(&m).unwrap();
    assert!(
        out.contains("qubit"),
        "Expected qubit declaration in QASM output"
    );
}

#[test]
fn test_qasm_gate_emission() {
    let m = make_module(vec![IrInstruction::QGate(
        "H".to_string(),
        vec![reg("%q0")],
    )]);
    let out = QasmBackend.generate(&m).unwrap();
    assert!(
        out.to_lowercase().contains("h "),
        "Expected H gate in QASM output"
    );
}

#[test]
fn test_qasm_measure_emission() {
    let m = make_module(vec![IrInstruction::QMeasure(reg("%c0"), reg("%q0"))]);
    let out = QasmBackend.generate(&m).unwrap();
    assert!(out.contains("measure"), "Expected measure in QASM output");
}

#[test]
fn test_qasm_file_extension() {
    assert_eq!(QasmBackend.file_extension(), "qasm");
}

// ── LLVM IR backend ───────────────────────────────────────────────────────────

#[test]
fn test_llvm_has_target_triple() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = LlvmIrBackend.generate(&m).unwrap();
    assert!(
        out.contains("target triple"),
        "Expected target triple in LLVM IR"
    );
}

#[test]
fn test_llvm_has_define() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let out = LlvmIrBackend.generate(&m).unwrap();
    assert!(out.contains("define"), "Expected define in LLVM IR");
}

#[test]
fn test_llvm_add_instruction() {
    let m = make_module(vec![IrInstruction::Add(
        reg("%r0"),
        IrValue::Reg(reg("%a")),
        IrValue::Reg(reg("%b")),
    )]);
    let out = LlvmIrBackend.generate(&m).unwrap();
    assert!(out.contains("add i64"), "Expected add i64 in LLVM IR");
}

#[test]
fn test_llvm_ret_instruction() {
    let m = make_module(vec![IrInstruction::Ret(Some(ci(0)))]);
    let out = LlvmIrBackend.generate(&m).unwrap();
    assert!(out.contains("ret"), "Expected ret in LLVM IR");
}

#[test]
fn test_llvm_file_extension() {
    assert_eq!(LlvmIrBackend.file_extension(), "ll");
}

// ── Nano control backend ──────────────────────────────────────────────────────

#[test]
fn test_nano_has_func_declaration() {
    let m = make_module(vec![IrInstruction::NanoSpawn(
        reg("%n0"),
        "Scout".to_string(),
    )]);
    let out = NanoControlBackend.generate(&m).unwrap();
    assert!(out.contains(".func"), "Expected .func in nano output");
}

#[test]
fn test_nano_spawn_emitted() {
    let m = make_module(vec![IrInstruction::NanoSpawn(
        reg("%n0"),
        "Recon".to_string(),
    )]);
    let out = NanoControlBackend.generate(&m).unwrap();
    assert!(out.contains("SPAWN"), "Expected SPAWN in nano output");
    assert!(out.contains("Recon"), "Expected agent name in nano output");
}

#[test]
fn test_nano_file_extension() {
    assert_eq!(NanoControlBackend.file_extension(), "nano");
}

// ── MTS bytecode backend ──────────────────────────────────────────────────────

#[test]
fn test_mts_has_timeline_declaration() {
    let m = make_module(vec![IrInstruction::MTSSnapshot(reg("%t0"))]);
    let out = MtsBytecodeBackend.generate(&m).unwrap();
    assert!(
        out.contains(".timeline"),
        "Expected .timeline in MTS output"
    );
}

#[test]
fn test_mts_snapshot_emitted() {
    let m = make_module(vec![IrInstruction::MTSSnapshot(reg("%t0"))]);
    let out = MtsBytecodeBackend.generate(&m).unwrap();
    assert!(out.contains("SNAPSHOT"), "Expected SNAPSHOT in MTS output");
}

#[test]
fn test_mts_sankofa_remember() {
    let m = make_module(vec![IrInstruction::SankofaStore("key".to_string(), ci(7))]);
    let out = MtsBytecodeBackend.generate(&m).unwrap();
    assert!(out.contains("REMEMBER"), "Expected REMEMBER in MTS output");
}

#[test]
fn test_mts_sankofa_recall() {
    let m = make_module(vec![IrInstruction::SankofaRecall(
        reg("%r0"),
        "key".to_string(),
    )]);
    let out = MtsBytecodeBackend.generate(&m).unwrap();
    assert!(out.contains("RECALL"), "Expected RECALL in MTS output");
}

#[test]
fn test_mts_file_extension() {
    assert_eq!(MtsBytecodeBackend.file_extension(), "mts");
}

// ── CodeGenerator dispatch ────────────────────────────────────────────────────

#[test]
fn test_codegen_x86_dispatch() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let gen = CodeGenerator::new(config_for(CompilationTarget::X86_64Linux));
    let out = gen.generate(&m).unwrap();
    assert_eq!(out.target, "x86_64-linux");
    assert_eq!(out.extension, "s");
}

#[test]
fn test_codegen_wasm_dispatch() {
    let m = make_module(vec![IrInstruction::Ret(None)]);
    let gen = CodeGenerator::new(config_for(CompilationTarget::Wasm32));
    let out = gen.generate(&m).unwrap();
    assert_eq!(out.target, "wasm32");
}

#[test]
fn test_codegen_output_size_nonzero() {
    let m = make_module(vec![
        IrInstruction::Assign(reg("%r0"), ci(1)),
        IrInstruction::Ret(Some(ci(0))),
    ]);
    let gen = CodeGenerator::new(config_for(CompilationTarget::LLVMIR));
    let out = gen.generate(&m).unwrap();
    assert!(out.size_bytes > 0, "Output should be non-empty");
}
