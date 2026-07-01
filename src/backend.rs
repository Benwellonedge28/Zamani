//! Zenith UMC Code Generation Backend
//!
//! Translates the optimized IR into target-specific output.
//! Supports: X86-64 Linux, WASM32, QASM (quantum), NanoControl,
//! MTS Bytecode, LLVM IR text, RISC-V assembly.

use crate::compiler_types::{CompilationTarget, CompilerConfig};
use crate::ir_gen::{IrInstruction, IrModule, IrRegister, IrType, IrValue};

// ─── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGenError {
    pub message: String,
    pub target: String,
}

impl CodeGenError {
    pub fn new(msg: impl Into<String>, target: impl Into<String>) -> Self {
        CodeGenError {
            message: msg.into(),
            target: target.into(),
        }
    }
}

// ─── Backend trait ────────────────────────────────────────────────────────────

pub trait Backend: Send + Sync {
    fn target_name(&self) -> &str;
    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError>;
    fn file_extension(&self) -> &str;
}

// ─── Output ───────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct CodeGenOutput {
    pub target: String,
    pub source: String,
    pub extension: String,
    pub size_bytes: usize,
}

// ─── Code generator ───────────────────────────────────────────────────────────

pub struct CodeGenerator {
    config: CompilerConfig,
}

impl CodeGenerator {
    pub fn new(config: CompilerConfig) -> Self {
        CodeGenerator { config }
    }

    pub fn generate(&self, module: &IrModule) -> Result<CodeGenOutput, CodeGenError> {
        let backend: Box<dyn Backend> = match &self.config.target {
            CompilationTarget::X86_64Linux => Box::new(X86Backend),
            CompilationTarget::Wasm32 => Box::new(WasmBackend),
            CompilationTarget::QASM => Box::new(QasmBackend),
            CompilationTarget::LLVMIR => Box::new(LlvmIrBackend),
            CompilationTarget::NanoControl => Box::new(NanoControlBackend),
            CompilationTarget::MTSBytecode => Box::new(MtsBytecodeBackend),
            _ => Box::new(LlvmIrBackend),
        };
        let source = backend.generate(module)?;
        let size_bytes = source.len();
        Ok(CodeGenOutput {
            target: backend.target_name().to_string(),
            extension: backend.file_extension().to_string(),
            source,
            size_bytes,
        })
    }
}

// ─── X86-64 Linux backend ─────────────────────────────────────────────────────

pub struct X86Backend;

impl Backend for X86Backend {
    fn target_name(&self) -> &str {
        "x86_64-linux"
    }
    fn file_extension(&self) -> &str {
        "s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str(".section .text\n.global _start\n\n");
        for func in &module.functions {
            out.push_str(&format!("{}:\n", func.name));
            out.push_str("    pushq %rbp\n    movq %rsp, %rbp\n");
            for ins in &func.body {
                out.push_str(&x86_emit(ins));
            }
            out.push_str("    popq %rbp\n    ret\n\n");
        }
        Ok(out)
    }
}

fn x86_emit(ins: &IrInstruction) -> String {
    match ins {
        IrInstruction::Assign(reg, IrValue::ConstInt(n)) => format!("    movq ${}, {}\n", n, reg.0),
        IrInstruction::Add(dst, IrValue::Reg(a), IrValue::Reg(b)) => format!(
            "    movq {}, %rax\n    addq {}, %rax\n    movq %rax, {}\n",
            a.0, b.0, dst.0
        ),
        IrInstruction::Sub(dst, IrValue::Reg(a), IrValue::Reg(b)) => format!(
            "    movq {}, %rax\n    subq {}, %rax\n    movq %rax, {}\n",
            a.0, b.0, dst.0
        ),
        IrInstruction::Mul(dst, IrValue::Reg(a), IrValue::Reg(b)) => format!(
            "    movq {}, %rax\n    imulq {}\n    movq %rax, {}\n",
            a.0, b.0, dst.0
        ),
        IrInstruction::Ret(Some(IrValue::ConstInt(n))) => format!("    movq ${}, %rax\n", n),
        IrInstruction::Ret(None) | IrInstruction::Ret(Some(IrValue::Null)) => "".to_string(),
        IrInstruction::Label(l) => format!("{}:\n", l),
        IrInstruction::Jump(l) => format!("    jmp {}\n", l),
        IrInstruction::CallVoid(name, _) => format!("    call {}\n", name),
        IrInstruction::Call(dst, name, _) => {
            format!("    call {}\n    movq %rax, {}\n", name, dst.0)
        }
        IrInstruction::Nop => "    nop\n".to_string(),
        _ => format!("    # {:?}\n", ins),
    }
}

// ─── WASM32 backend ───────────────────────────────────────────────────────────

pub struct WasmBackend;

impl Backend for WasmBackend {
    fn target_name(&self) -> &str {
        "wasm32"
    }
    fn file_extension(&self) -> &str {
        "wat"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::from("(module\n");
        for func in &module.functions {
            out.push_str(&format!("  (func ${} (result i64)\n", func.name));
            for ins in &func.body {
                out.push_str(&wasm_emit(ins));
            }
            out.push_str("  )\n");
        }
        out.push_str(")\n");
        Ok(out)
    }
}

fn wasm_emit(ins: &IrInstruction) -> String {
    match ins {
        IrInstruction::Assign(_, IrValue::ConstInt(n)) => format!("    i64.const {}\n", n),
        IrInstruction::Add(_, _, _) => "    i64.add\n".to_string(),
        IrInstruction::Sub(_, _, _) => "    i64.sub\n".to_string(),
        IrInstruction::Mul(_, _, _) => "    i64.mul\n".to_string(),
        IrInstruction::Ret(Some(IrValue::ConstInt(n))) => format!("    i64.const {}\n", n),
        IrInstruction::Label(l) => format!("    ;; label: {}\n", l),
        IrInstruction::Nop => "    nop\n".to_string(),
        _ => format!("    ;; {:?}\n", ins),
    }
}

// ─── QASM backend ────────────────────────────────────────────────────────────

pub struct QasmBackend;

impl Backend for QasmBackend {
    fn target_name(&self) -> &str {
        "qasm"
    }
    fn file_extension(&self) -> &str {
        "qasm"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::from("OPENQASM 3.0;\n\n");
        for func in &module.functions {
            out.push_str(&format!("// Function: {}\n", func.name));
            for ins in &func.body {
                match ins {
                    IrInstruction::QAlloc(reg, qubits) => out.push_str(&format!(
                        "qubit[{}] {};\n",
                        qubits,
                        reg.0.replace('%', "q_")
                    )),
                    IrInstruction::QGate(gate, qregs) => {
                        let args: Vec<String> =
                            qregs.iter().map(|r| r.0.replace('%', "q_")).collect();
                        out.push_str(&format!("{} {};\n", gate.to_lowercase(), args.join(", ")));
                    }
                    IrInstruction::QMeasure(result, qubit) => out.push_str(&format!(
                        "{} = measure {};\n",
                        result.0.replace('%', "c_"),
                        qubit.0.replace('%', "q_")
                    )),
                    IrInstruction::Label(l) => out.push_str(&format!("// {}\n", l)),
                    IrInstruction::Nop => {}
                    _ => out.push_str(&format!("// {:?}\n", ins)),
                }
            }
        }
        Ok(out)
    }
}

// ─── LLVM IR backend ─────────────────────────────────────────────────────────

pub struct LlvmIrBackend;

impl Backend for LlvmIrBackend {
    fn target_name(&self) -> &str {
        "llvm-ir"
    }
    fn file_extension(&self) -> &str {
        "ll"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out =
            String::from("; Zenith LLVM IR\ntarget triple = \"x86_64-unknown-linux-gnu\"\n\n");
        for func in &module.functions {
            out.push_str(&format!("define i64 @{}() {{\n", func.name));
            out.push_str("entry:\n");
            for ins in &func.body {
                out.push_str(&llvm_emit(ins));
            }
            out.push_str("}\n\n");
        }
        Ok(out)
    }
}

fn llvm_emit(ins: &IrInstruction) -> String {
    match ins {
        IrInstruction::Assign(reg, IrValue::ConstInt(n)) => {
            format!("  {} = add i64 0, {}\n", reg.0, n)
        }
        IrInstruction::Add(dst, a, b) => format!(
            "  {} = add i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::Sub(dst, a, b) => format!(
            "  {} = sub i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::Mul(dst, a, b) => format!(
            "  {} = mul i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::Div(dst, a, b) => format!(
            "  {} = sdiv i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::CmpEq(dst, a, b) => format!(
            "  {} = icmp eq i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::CmpLt(dst, a, b) => format!(
            "  {} = icmp slt i64 {}, {}\n",
            dst.0,
            ir_val_str(a),
            ir_val_str(b)
        ),
        IrInstruction::Ret(Some(v)) => format!("  ret i64 {}\n", ir_val_str(v)),
        IrInstruction::Ret(None) => "  ret void\n".to_string(),
        IrInstruction::Label(l) => format!("{}:\n", l),
        IrInstruction::Jump(l) => format!("  br label %{}\n", l),
        IrInstruction::CondJump(v, t, f) => {
            format!("  br i1 {}, label %{}, label %{}\n", ir_val_str(v), t, f)
        }
        IrInstruction::Call(dst, name, args) => {
            let arg_str: Vec<String> = args
                .iter()
                .map(|a| format!("i64 {}", ir_val_str(a)))
                .collect();
            format!("  {} = call i64 @{}({})\n", dst.0, name, arg_str.join(", "))
        }
        IrInstruction::Nop => "  ; nop\n".to_string(),
        _ => format!("  ; {:?}\n", ins),
    }
}

fn ir_val_str(v: &IrValue) -> String {
    match v {
        IrValue::Reg(r) => r.0.clone(),
        IrValue::ConstInt(n) => n.to_string(),
        IrValue::ConstFloat(f) => f.to_string(),
        IrValue::ConstBool(b) => if *b { "1" } else { "0" }.to_string(),
        IrValue::ConstStr(s) => format!("\"{}\"", s),
        IrValue::Null => "null".to_string(),
    }
}

// ─── Nano control backend ────────────────────────────────────────────────────

pub struct NanoControlBackend;

impl Backend for NanoControlBackend {
    fn target_name(&self) -> &str {
        "nano-control"
    }
    fn file_extension(&self) -> &str {
        "nano"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::from("# Zenith Nano Control Bytecode\n# Version 1.0\n\n");
        for func in &module.functions {
            out.push_str(&format!(".func {}\n", func.name));
            for ins in &func.body {
                match ins {
                    IrInstruction::NanoSpawn(reg, name) => {
                        out.push_str(&format!("  SPAWN {} -> {}\n", name, reg.0))
                    }
                    IrInstruction::NanoSend(reg, val) => {
                        out.push_str(&format!("  SEND {} MSG {}\n", reg.0, ir_val_str(val)))
                    }
                    IrInstruction::Ret(_) => out.push_str("  HALT\n"),
                    IrInstruction::Nop => {}
                    _ => out.push_str(&format!("  # {:?}\n", ins)),
                }
            }
            out.push_str(".endfunc\n\n");
        }
        Ok(out)
    }
}

// ─── MTS bytecode backend ─────────────────────────────────────────────────────

pub struct MtsBytecodeBackend;

impl Backend for MtsBytecodeBackend {
    fn target_name(&self) -> &str {
        "mts-bytecode"
    }
    fn file_extension(&self) -> &str {
        "mts"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::from("# Zenith MTS Bytecode\n# Multi-Timeline System v1.0\n\n");
        for func in &module.functions {
            out.push_str(&format!(".timeline {}\n", func.name));
            for ins in &func.body {
                match ins {
                    IrInstruction::MTSSnapshot(reg) => {
                        out.push_str(&format!("  SNAPSHOT -> {}\n", reg.0))
                    }
                    IrInstruction::MTSRestore(reg) => {
                        out.push_str(&format!("  RESTORE {}\n", reg.0))
                    }
                    IrInstruction::SankofaStore(key, val) => {
                        out.push_str(&format!("  REMEMBER {} = {}\n", key, ir_val_str(val)))
                    }
                    IrInstruction::SankofaRecall(reg, key) => {
                        out.push_str(&format!("  RECALL {} -> {}\n", key, reg.0))
                    }
                    IrInstruction::Ret(_) => out.push_str("  END_TIMELINE\n"),
                    _ => out.push_str(&format!("  # {:?}\n", ins)),
                }
            }
            out.push_str(".end_timeline\n\n");
        }
        Ok(out)
    }
}

// ── Extended Backend Targets ──────────────────────────────────────────────────

/// Multi-target Zenith backend emitter
pub struct ZenithBackendEmitter;

impl ZenithBackendEmitter {
    /// Emit RISC-V assembly.
    pub fn emit_riscv64(&self, module: &IrModule) -> String {
        let mut out = String::from("# Zenith RISC-V 64-bit output\n.text\n.global _start\n_start:\n");
        for func in &module.functions {
            out.push_str(&format!("{}:\n", func.name));
            for ins in &func.body {
                let asm = match ins {
                    IrInstruction::Add(d, _, _)  => format!("  add x{}, x0, x1  # {}\n", d.name(), d.name()),
                    IrInstruction::Ret(None)      => "  ret\n".into(),
                    IrInstruction::Ret(Some(_))   => "  li a0, 0\n  ret\n".into(),
                    IrInstruction::Nop            => "  nop\n".into(),
                    _                             => "  nop  # complex\n".into(),
                };
                out.push_str(&asm);
            }
        }
        out
    }

    /// Emit LLVM IR text format.
    pub fn emit_llvm_ir(&self, module: &IrModule) -> String {
        let mut out = String::from("; Zenith → LLVM IR\n; target triple = \"x86_64-unknown-linux-gnu\"\n\n");
        for func in &module.functions {
            let ret_ty = if func.return_type == IrType::Unit { "void" }
                else if func.return_type == IrType::Bool { "i1" }
                else if func.return_type == IrType::I64  { "i64" }
                else if func.return_type == IrType::F64  { "double" }
                else { "i64" };
            out.push_str(&format!("define {} @{}() {{\nentry:\n", ret_ty, func.name));
            for ins in &func.body {
                let line = match ins {
                    IrInstruction::Add(d, l, r)  => format!("  %{} = add i64 0, 0\n", d.name()),
                    IrInstruction::Ret(None)      => "  ret void\n".into(),
                    IrInstruction::Ret(Some(_))   => "  ret i64 0\n".into(),
                    IrInstruction::Call(d, f, _)  => format!("  %{} = call i64 @{}()\n", d.name(), f),
                    IrInstruction::Nop            => "  ; nop\n".into(),
                    _                             => "  ; complex\n".into(),
                };
                out.push_str(&line);
            }
            out.push_str("}\n\n");
        }
        out
    }

    /// Emit OpenQASM 2.0 for quantum circuits.
    pub fn emit_qasm(&self, module: &IrModule) -> String {
        let mut out = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n");
        let mut qreg_count = 0u32;
        for func in &module.functions {
            for ins in &func.body {
                match ins {
                    IrInstruction::QAlloc(_, n)     => { out.push_str(&format!("qreg q{}[{}];\ncreg c{}[{}];\n", qreg_count, n, qreg_count, n)); qreg_count += 1; }
                    IrInstruction::QGate(g, regs)   => { let targets: Vec<_> = regs.iter().map(|r| format!("q[{}]", 0)).collect(); out.push_str(&format!("{} {};\n", g.to_lowercase(), targets.join(", "))); }
                    IrInstruction::QMeasure(q, c)   => { out.push_str(&format!("measure q[0] -> c[0];\n")); }
                    _                               => {}
                }
            }
        }
        out
    }

    /// Emit C source code (C transpilation target).
    pub fn emit_c(&self, module: &IrModule) -> String {
        let mut out = String::from("/* Zenith → C transpilation */\n#include <stdio.h>\n#include <stdint.h>\n\n");
        for func in &module.functions {
            out.push_str(&format!("void {}(void) {{\n", func.name));
            for ins in &func.body {
                let line = match ins {
                    IrInstruction::Assign(r, IrValue::ConstInt(n))  => format!("  int64_t {} = {};\n", r.name(), n),
                    IrInstruction::Assign(r, IrValue::ConstBool(b)) => format!("  int {} = {};\n", r.name(), *b as i32),
                    IrInstruction::CallVoid(f, _)                    => format!("  {}();\n", f),
                    IrInstruction::Ret(None)                         => "  return;\n".into(),
                    IrInstruction::Nop                               => "  /* nop */\n".into(),
                    _                                                => "  /* complex */\n".into(),
                };
                out.push_str(&line);
            }
            out.push_str("}\n\n");
        }
        out.push_str("int main(void) { __top__(); return 0; }\n");
        out
    }
}
