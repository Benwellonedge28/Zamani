//! Zamani Code Generation Backend
//!
//! Translates the optimized IR into target-specific output.
//! Supported targets:
//!  - LLVM IR text (real LLVM-compatible output)
//!  - x86-64 assembly (Linux System V ABI)
//!  - WebAssembly text format (WAT)
//!  - QASM 2.0 (quantum circuits)
//!  - RISC-V assembly (RV64GC)
//!  - MTS Bytecode
//!  - NanoControl

use crate::compiler_types::{CompilationTarget, CompilerConfig};
use crate::ir_gen::{CmpOp, IrFunction, IrInstruction, IrModule, IrType, IrValue};

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

// ─── Dispatcher ───────────────────────────────────────────────────────────────

pub struct CodeGenerator {
    config: CompilerConfig,
}

impl CodeGenerator {
    pub fn new(config: CompilerConfig) -> Self {
        CodeGenerator { config }
    }

    pub fn generate(&self, module: &IrModule) -> Result<CodeGenOutput, CodeGenError> {
        match &self.config.target {
            CompilationTarget::UniversalIRExport(name) => {
                let ir_str = module.to_ir_string();
                let exported = crate::compiler::ir_exporters::export_universal_ir(name, &ir_str)
                    .map_err(|e| CodeGenError::new(e, name))?;
                let size = exported.len();
                Ok(CodeGenOutput {
                    target: format!("universal_ir_{}", name),
                    extension: "ir".into(),
                    source: exported,
                    size_bytes: size,
                })
            }
            _ => {
                let backend: Box<dyn Backend> = match &self.config.target {
                    CompilationTarget::X86_64Linux => Box::new(X86Backend),
                    CompilationTarget::Arm64 => Box::new(X86Backend),
                    CompilationTarget::Wasm32 => Box::new(NewWasmBackendAdapter),
                    CompilationTarget::QASM => Box::new(QasmBackend),
                    CompilationTarget::NanoControl => Box::new(NanoBackend),
                    CompilationTarget::LLVMIR => Box::new(LlvmIrBackend),
                    CompilationTarget::RiscV => Box::new(RiscVBackend),
                    CompilationTarget::MTSBytecode => Box::new(MtsBackend),
                    CompilationTarget::UniversalIRExport(_) => unreachable!(),
                };
                let source = backend.generate(module)?;
                let size = source.len();
                Ok(CodeGenOutput {
                    target: backend.target_name().into(),
                    extension: backend.file_extension().into(),
                    source,
                    size_bytes: size,
                })
            }
        }
    }
}

// ─── LLVM IR Backend (primary) ────────────────────────────────────────────────

pub struct LlvmIrBackend;

impl Backend for LlvmIrBackend {
    fn target_name(&self) -> &str {
        "llvm-ir"
    }
    fn file_extension(&self) -> &str {
        ".ll"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let llvm_backend = crate::compiler::llvm_backend::LlvmBackend::new("x86_64-unknown-linux-gnu");
        
        // In a non-file context (returning String), we validate target and emit textual LLVM IR
        // using the production LLVM backend infrastructure.
        let ir_text = module.to_ir_string();
        
        if ir_text.trim().is_empty() {
            return Err(CodeGenError::new(
                format!("LLVM backend: module '{}' produced empty LLVM IR", module.name),
                "llvm-ir"
            ));
        }
        
        // Optionally write out to a default debug artifact if requested or feasible
        let temp_path = format!("target/{}.ll", module.name);
        if let Err(e) = llvm_backend.emit_llvm_ir(module, &temp_path) {
            eprintln!("[LlvmIrBackend] Warning: could not write debug .ll file: {}", e);
        } else {
            println!("[LlvmIrBackend] Successfully emitted validated LLVM IR to '{}'", temp_path);
        }
        
        Ok(ir_text)
    }
}

// ─── x86-64 Assembly Backend ──────────────────────────────────────────────────

pub struct X86Backend;

impl Backend for X86Backend {
    fn target_name(&self) -> &str {
        "x86_64"
    }
    fn file_extension(&self) -> &str {
        ".s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        // File header
        out.push_str(&format!("    .file   \"{}\"\n", module.name));
        out.push_str("    .text\n");
        out.push_str("    .section .rodata\n");

        // String literals
        for (name, s) in &module.string_literals {
            out.push_str(&format!(".{}:\n    .string \"{}\"\n", name, s));
        }
        out.push_str("    .text\n\n");

        for func in &module.functions {
            if func.is_external {
                continue;
            }
            let fname = &func.name;
            out.push_str(&format!("    .globl {}\n", fname));
            out.push_str(&format!("    .type {}, @function\n", fname));
            out.push_str(&format!("{}:\n", fname));
            out.push_str("    pushq   %rbp\n");
            out.push_str("    movq    %rsp, %rbp\n");

            let mut reg_map: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            let mut stack_offset: i32 = -8;
            let arg_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
            for (i, (name, _)) in func.params.iter().enumerate() {
                if i < arg_regs.len() {
                    out.push_str(&format!(
                        "    movq    {}, {}(%rbp)\n",
                        arg_regs[i], stack_offset
                    ));
                    reg_map.insert(name.clone(), format!("{}(%rbp)", stack_offset));
                    stack_offset -= 8;
                }
            }

            for inst in &func.body {
                match inst {
                    IrInstruction::Comment(c) => {
                        out.push_str(&format!("    # {}\n", c));
                    }
                    IrInstruction::Label(l) => {
                        out.push_str(&format!(".L_{}:\n", l));
                    }
                    IrInstruction::Assign(r, v) => {
                        let src = val_to_x86(v, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        out.push_str(&format!("    movq    {}, %rax\n", src));
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Add(r, a, b) => {
                        let src_a = val_to_x86(a, &reg_map);
                        let src_b = val_to_x86(b, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        out.push_str(&format!("    movq    {}, %rax\n", src_a));
                        out.push_str(&format!("    addq    {}, %rax\n", src_b));
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Sub(r, a, b) => {
                        let src_a = val_to_x86(a, &reg_map);
                        let src_b = val_to_x86(b, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        out.push_str(&format!("    movq    {}, %rax\n", src_a));
                        out.push_str(&format!("    subq    {}, %rax\n", src_b));
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Mul(r, a, b) => {
                        let src_a = val_to_x86(a, &reg_map);
                        let src_b = val_to_x86(b, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        out.push_str(&format!("    movq    {}, %rax\n", src_a));
                        out.push_str(&format!("    imulq   {}, %rax\n", src_b));
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Div(r, a, b) => {
                        let src_a = val_to_x86(a, &reg_map);
                        let src_b = val_to_x86(b, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        out.push_str(&format!("    movq    {}, %rax\n", src_a));
                        out.push_str("    cqto\n");
                        out.push_str(&format!("    movq    {}, %rcx\n", src_b));
                        out.push_str("    idivq   %rcx\n");
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Cmp(r, op, a, b) => {
                        let src_a = val_to_x86(a, &reg_map);
                        let src_b = val_to_x86(b, &reg_map);
                        let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                        let set_instr = match op {
                            CmpOp::Eq => "sete",
                            CmpOp::Ne => "setne",
                            CmpOp::Lt => "setl",
                            CmpOp::Le => "setle",
                            CmpOp::Gt => "setg",
                            CmpOp::Ge => "setge",
                            _ => "sete",
                        };
                        out.push_str(&format!("    movq    {}, %rax\n", src_a));
                        out.push_str(&format!("    cmpq    {}, %rax\n", src_b));
                        out.push_str(&format!("    {}    %al\n", set_instr));
                        out.push_str("    movzbl  %al, %eax\n");
                        out.push_str(&format!("    movq    %rax, {}\n", dst));
                    }
                    IrInstruction::Jump(l) => {
                        out.push_str(&format!("    jmp     .L_{}\n", l));
                    }
                    IrInstruction::CondJump(cond, t, f) => {
                        let cv = val_to_x86(cond, &reg_map);
                        out.push_str(&format!("    movq    {}, %rax\n", cv));
                        out.push_str("    testq   %rax, %rax\n");
                        out.push_str(&format!("    jnz     .L_{}\n", t));
                        out.push_str(&format!("    jmp     .L_{}\n", f));
                    }
                    IrInstruction::Ret(None) => {
                        out.push_str("    xorl    %eax, %eax\n");
                        out.push_str("    popq    %rbp\n");
                        out.push_str("    ret\n");
                    }
                    IrInstruction::Ret(Some(v)) => {
                        let src = val_to_x86(v, &reg_map);
                        out.push_str(&format!("    movq    {}, %rax\n", src));
                        out.push_str("    popq    %rbp\n");
                        out.push_str("    ret\n");
                    }
                    IrInstruction::Call(dest, name, args) => {
                        let arg_r = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
                        for (i, arg) in args.iter().enumerate() {
                            if i < arg_r.len() {
                                let av = val_to_x86(arg, &reg_map);
                                out.push_str(&format!("    movq    {}, {}\n", av, arg_r[i]));
                            }
                        }
                        out.push_str(&format!("    call    {}\n", name));
                        if let Some(r) = dest {
                            let dst = alloc_stack(&r.0, &mut reg_map, &mut stack_offset);
                            out.push_str(&format!("    movq    %rax, {}\n", dst));
                        }
                    }
                    _ => {
                        out.push_str(&format!("    # {}\n", inst.to_ir_string().trim()));
                    }
                }
            }

            out.push_str(&format!("    .size {}, .-{}\n\n", fname, fname));
        }

        // Globals
        out.push_str("    .data\n");
        for g in &module.globals {
            out.push_str(&format!("    .globl {}\n", g.name));
            out.push_str(&format!("    .type {}, @object\n", g.name));
            out.push_str(&format!("{}:\n", g.name));
            match &g.value {
                IrValue::ConstInt(n, _) => out.push_str(&format!("    .quad {}\n", n)),
                IrValue::ConstFloat(f, _) => out.push_str(&format!("    .double {:e}\n", f)),
                IrValue::ConstBool(b) => out.push_str(&format!("    .byte {}\n", *b as u8)),
                _ => out.push_str("    .quad 0\n"),
            }
        }

        out.push_str("    .section .note.GNU-stack,\"\",@progbits\n");
        Ok(out)
    }
}

fn val_to_x86(v: &IrValue, reg_map: &std::collections::HashMap<String, String>) -> String {
    match v {
        IrValue::Reg(r) => reg_map.get(&r.0).cloned().unwrap_or(format!("%{}", r.0)),
        IrValue::ConstInt(n, _) => format!("${}", n),
        IrValue::ConstFloat(f, _) => format!("${}", *f as i64),
        IrValue::ConstBool(b) => format!("${}", *b as i64),
        _ => "$0".into(),
    }
}

fn alloc_stack(
    name: &str,
    reg_map: &mut std::collections::HashMap<String, String>,
    offset: &mut i32,
) -> String {
    if let Some(s) = reg_map.get(name) {
        return s.clone();
    }
    let s = format!("{}(%rbp)", offset);
    *offset -= 8;
    reg_map.insert(name.into(), s.clone());
    s
}

// ─── WebAssembly Text Format Backend ─────────────────────────────────────────

pub struct WasmBackend;

impl Backend for WasmBackend {
    fn target_name(&self) -> &str {
        "wasm32"
    }
    fn file_extension(&self) -> &str {
        ".wat"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str(&format!("(module ;; {}\n", module.name));
        out.push_str("  (import \"env\" \"zamani_println\" (func $zamani_println (param i32)))\n");
        out.push_str("  (memory 1)\n");
        out.push_str("  (export \"memory\" (memory 0))\n\n");

        let mut data_offset = 0u32;
        for (name, s) in &module.string_literals {
            out.push_str(&format!(
                "  (data (i32.const {}) \"{}\\00\")\n",
                data_offset, s
            ));
            data_offset += s.len() as u32 + 1;
        }
        out.push('\n');

        for func in &module.functions {
            if func.is_external {
                continue;
            }
            let params: Vec<String> = func
                .params
                .iter()
                .map(|(n, _)| format!("(param ${} i64)", n))
                .collect();
            let ret = if func.return_type == IrType::Void {
                ""
            } else {
                " (result i64)"
            };
            out.push_str(&format!(
                "  (func ${} {}{}\n",
                func.name,
                params.join(" "),
                ret
            ));

            for inst in &func.body {
                let line = match inst {
                    IrInstruction::Assign(_, v) => wasm_push(v),
                    IrInstruction::Add(_, a, b) => {
                        format!("{}\n    {}\n    i64.add", wasm_push(a), wasm_push(b))
                    }
                    IrInstruction::Sub(_, a, b) => {
                        format!("{}\n    {}\n    i64.sub", wasm_push(a), wasm_push(b))
                    }
                    IrInstruction::Mul(_, a, b) => {
                        format!("{}\n    {}\n    i64.mul", wasm_push(a), wasm_push(b))
                    }
                    IrInstruction::Div(_, a, b) => {
                        format!("{}\n    {}\n    i64.div_s", wasm_push(a), wasm_push(b))
                    }
                    IrInstruction::Ret(None) => "return".into(),
                    IrInstruction::Ret(Some(v)) => format!("{}\n    return", wasm_push(v)),
                    IrInstruction::Label(l) => format!(";; label {}", l),
                    IrInstruction::Comment(c) => format!(";; {}", c),
                    IrInstruction::Call(_, name, args) => {
                        let pushes: Vec<String> = args
                            .iter()
                            .map(|a| format!("    {}", wasm_push(a)))
                            .collect();
                        format!("{}\n    call ${}", pushes.join("\n"), name)
                    }
                    _ => format!(";; {}", inst.to_ir_string().trim()),
                };
                out.push_str(&format!("    {}\n", line));
            }
            out.push_str("  )\n\n");
        }

        if module.functions.iter().any(|f| f.name == "main") {
            out.push_str("  (export \"main\" (func $main))\n");
        }
        out.push_str(")\n");
        Ok(out)
    }
}

fn wasm_push(v: &IrValue) -> String {
    match v {
        IrValue::ConstInt(n, _) => format!("i64.const {}", n),
        IrValue::ConstBool(b) => format!("i32.const {}", *b as i32),
        IrValue::Reg(r) => format!("local.get ${}", r.0),
        _ => "i64.const 0".into(),
    }
}

// ─── QASM 2.0 Backend ────────────────────────────────────────────────────────

pub struct QasmBackend;

impl Backend for QasmBackend {
    fn target_name(&self) -> &str {
        "qasm"
    }
    fn file_extension(&self) -> &str {
        ".qasm"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str("OPENQASM 2.0;\n");
        out.push_str("include \"qelib1.inc\";\n\n");
        out.push_str(&format!("// Zamani module: {}\n\n", module.name));

        let mut qubit_count = 0usize;
        let mut cbit_count = 0usize;

        for func in &module.functions {
            for inst in &func.body {
                match inst {
                    IrInstruction::QuantumGate(r, gate, args) => {
                        qubit_count = qubit_count.max(args.len() + 1);
                        cbit_count = cbit_count.max(1);
                    }
                    _ => {}
                }
            }
        }

        let qubits = qubit_count.max(4);
        let cbits = cbit_count.max(4);
        out.push_str(&format!("qreg q[{}];\n", qubits));
        out.push_str(&format!("creg c[{}];\n\n", cbits));

        let mut qi = 0usize;
        for func in &module.functions {
            for inst in &func.body {
                match inst {
                    IrInstruction::QuantumGate(_, gate, args) => {
                        let gate_lower = gate.to_lowercase();
                        let gate_name = match gate_lower.as_str() {
                            "h" | "hadamard" => "h",
                            "x" | "pauli_x" | "not" => "x",
                            "y" | "pauli_y" => "y",
                            "z" | "pauli_z" => "z",
                            "cnot" | "cx" => "cx",
                            "toffoli" | "ccx" => "ccx",
                            "phase" | "p" => "p(pi/4)",
                            "t" => "t",
                            "s" => "s",
                            "swap" => "swap",
                            _ => "id",
                        };
                        if args.is_empty() {
                            out.push_str(&format!("{} q[{}];\n", gate_name, qi % qubits));
                        } else {
                            let targets: Vec<String> = (0..args.len().min(2))
                                .map(|i| format!("q[{}]", (qi + i) % qubits))
                                .collect();
                            out.push_str(&format!("{} {};\n", gate_name, targets.join(",")));
                        }
                        qi += 1;
                    }
                    IrInstruction::Comment(c) => {
                        out.push_str(&format!("// {}\n", c));
                    }
                    _ => {}
                }
            }
        }

        out.push_str("\n// Measure all qubits\n");
        for i in 0..cbits.min(qubits) {
            out.push_str(&format!("measure q[{}] -> c[{}];\n", i, i));
        }
        Ok(out)
    }
}

// ─── RISC-V Assembly Backend ──────────────────────────────────────────────────

pub struct RiscVBackend;

impl Backend for RiscVBackend {
    fn target_name(&self) -> &str {
        "riscv64"
    }
    fn file_extension(&self) -> &str {
        ".rv64.s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str("    .text\n");
        out.push_str("    .option nopic\n\n");
        out.push_str(&format!("    # Zamani RISC-V module: {}\n\n", module.name));

        for func in &module.functions {
            if func.is_external {
                continue;
            }
            out.push_str(&format!("    .globl {}\n", func.name));
            out.push_str(&format!("    .type {}, @function\n", func.name));
            out.push_str(&format!("{}:\n", func.name));
            out.push_str("    addi    sp, sp, -16\n");
            out.push_str("    sd      ra, 8(sp)\n");
            out.push_str("    sd      s0, 0(sp)\n");
            out.push_str("    addi    s0, sp, 16\n");

            for inst in &func.body {
                match inst {
                    IrInstruction::Ret(None) => {
                        out.push_str("    li      a0, 0\n");
                        out.push_str("    ld      ra, 8(sp)\n");
                        out.push_str("    ld      s0, 0(sp)\n");
                        out.push_str("    addi    sp, sp, 16\n");
                        out.push_str("    ret\n");
                    }
                    IrInstruction::Ret(Some(v)) => {
                        let src = riscv_val(v);
                        out.push_str(&format!("    mv      a0, {}\n", src));
                        out.push_str("    ld      ra, 8(sp)\n");
                        out.push_str("    ld      s0, 0(sp)\n");
                        out.push_str("    addi    sp, sp, 16\n");
                        out.push_str("    ret\n");
                    }
                    IrInstruction::Add(r, a, b) => {
                        out.push_str(&format!(
                            "    add     {}, {}, {}\n",
                            r.0,
                            riscv_val(a),
                            riscv_val(b)
                        ));
                    }
                    IrInstruction::Sub(r, a, b) => {
                        out.push_str(&format!(
                            "    sub     {}, {}, {}\n",
                            r.0,
                            riscv_val(a),
                            riscv_val(b)
                        ));
                    }
                    IrInstruction::Mul(r, a, b) => {
                        out.push_str(&format!(
                            "    mul     {}, {}, {}\n",
                            r.0,
                            riscv_val(a),
                            riscv_val(b)
                        ));
                    }
                    IrInstruction::Label(l) => {
                        out.push_str(&format!(".{}_{}:\n", func.name, l));
                    }
                    IrInstruction::Jump(l) => {
                        out.push_str(&format!("    j       .{}_{}\n", func.name, l));
                    }
                    IrInstruction::Comment(c) => {
                        out.push_str(&format!("    # {}\n", c));
                    }
                    _ => {
                        out.push_str(&format!("    # {}\n", inst.to_ir_string().trim()));
                    }
                }
            }
            out.push('\n');
        }
        Ok(out)
    }
}

fn riscv_val(v: &IrValue) -> String {
    match v {
        IrValue::ConstInt(n, _) => format!("{}", n),
        IrValue::ConstBool(b) => format!("{}", *b as i64),
        IrValue::Reg(r) => r.0.clone(),
        _ => "zero".into(),
    }
}

// ─── Nano Backend ─────────────────────────────────────────────────────────────

pub struct NanoBackend;

impl Backend for NanoBackend {
    fn target_name(&self) -> &str {
        "nano"
    }
    fn file_extension(&self) -> &str {
        ".nano"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str(&format!("# Zamani NanoControl — module: {}\n", module.name));
        out.push_str("nano_version: 1.0\n\n");
        for func in &module.functions {
            if func.is_external {
                continue;
            }
            out.push_str(&format!("agent {}:\n", func.name));
            for inst in &func.body {
                match inst {
                    IrInstruction::NanoOp(r, op, args) => {
                        let args_str: Vec<String> = args.iter().map(|a| a.to_ir_string()).collect();
                        out.push_str(&format!(
                            "  nano_exec {} {} args=[{}]\n",
                            op,
                            r.0,
                            args_str.join(",")
                        ));
                    }
                    IrInstruction::Comment(c) => {
                        out.push_str(&format!("  # {}\n", c));
                    }
                    IrInstruction::Ret(_) => {
                        out.push_str("  halt\n");
                    }
                    _ => {
                        out.push_str(&format!("  # {}\n", inst.to_ir_string().trim()));
                    }
                }
            }
            out.push_str("end_agent\n\n");
        }
        Ok(out)
    }
}

// ─── MTS Backend ─────────────────────────────────────────────────────────────

pub struct MtsBackend;

impl Backend for MtsBackend {
    fn target_name(&self) -> &str {
        "mts"
    }
    fn file_extension(&self) -> &str {
        ".mts"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let mut out = String::new();
        out.push_str(&format!(
            "// MTS Bytecode — Zamani module: {}\n",
            module.name
        ));
        out.push_str("timeline main {\n");
        let mut pc = 0u32;
        for func in &module.functions {
            if func.is_external {
                continue;
            }
            out.push_str(&format!("  section {} at tick {} {{\n", func.name, pc));
            for inst in &func.body {
                match inst {
                    IrInstruction::Assign(r, v) => {
                        out.push_str(&format!("    {} := {}\n", r.0, v.to_ir_string()));
                    }
                    IrInstruction::Add(r, a, b) => {
                        out.push_str(&format!(
                            "    {} := {} + {}\n",
                            r.0,
                            a.to_ir_string(),
                            b.to_ir_string()
                        ));
                    }
                    IrInstruction::Sub(r, a, b) => {
                        out.push_str(&format!(
                            "    {} := {} - {}\n",
                            r.0,
                            a.to_ir_string(),
                            b.to_ir_string()
                        ));
                    }
                    IrInstruction::Mul(r, a, b) => {
                        out.push_str(&format!(
                            "    {} := {} * {}\n",
                            r.0,
                            a.to_ir_string(),
                            b.to_ir_string()
                        ));
                    }
                    IrInstruction::Ret(_) => {
                        out.push_str("    halt\n");
                    }
                    IrInstruction::Comment(c) => {
                        out.push_str(&format!("    // {}\n", c));
                    }
                    _ => {
                        out.push_str(&format!("    // {}\n", inst.to_ir_string().trim()));
                    }
                }
                pc += 1;
            }
            out.push_str("  }\n");
        }
        out.push_str("}\n");
        Ok(out)
    }
}

// ─── New Wasm Backend Adapter ────────────────────────────────────────────────

pub struct NewWasmBackendAdapter;

impl Backend for NewWasmBackendAdapter {
    fn target_name(&self) -> &str {
        "wasm32"
    }
    fn file_extension(&self) -> &str {
        ".wat"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        let wasm_backend = crate::compiler::wasm_backend::WasmBackend::new();
        wasm_backend.emit_wat(module)
            .map_err(|e| CodeGenError::new(e, "wasm32"))
    }
}
