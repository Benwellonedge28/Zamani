
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Safe-Rust RISC-V 64-bit ELF assembly emitter for Zamani.
//!
//! Target contract:
//! - ISA: RV64I + M (integer base plus multiply/divide).
//! - ABI: LP64, with the standard integer calling convention.
//! - Object format: ELF.
//! - Assembly syntax: GNU assembler-compatible RISC-V assembly.
//! - Stack alignment: 16 bytes at function-call boundaries.
//! - Input: the existing Zamani IrModule representation.
//! - Output: assembly text only; no filesystem, process, or console side effects.
//!
//! Implemented IR subset:
//! - Integer and boolean constants/registers.
//! - Add, Sub, Mul, Div, Rem, Neg.
//! - And, Or, Xor, Shl, Shr, Not.
//! - Integer Cmp and conditional/unconditional branches.
//! - Assign, direct Call, Ret, Label, Comment, Unreachable.
//! - ZExt, SExt, Trunc.
//!
//! Unsupported operations fail closed. In particular, this emitter does not
//! claim to implement floating-point operations, memory IR, pointers other
//! than null constants, indirect calls, SSA Phi nodes, quantum operations,
//! HDL, Nano, Sankofa, or arbitrary structured types.
//!
//! Integration:
//! - Uses crate::backend::{Backend, CodeGenError}.
//! - Uses the existing crate::ir_gen IR types.
//! - Is selected by CodeGenerator for CompilationTarget::RiscV.
//! - Does not own IR verification, semantic analysis, target selection,
//!   linking, runtime execution, quantum routing, or HDL synthesis.
//!
//! Rust baseline: 1.97+; safe Rust only.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;

use crate::backend::{Backend, CodeGenError};
use crate::ir_gen::{
    CmpOp, IrFunction, IrInstruction, IrModule, IrType, IrValue,
};

const TARGET: &str = "riscv64-unknown-linux-gnu";
const ARG_REGS: [&str; 8] = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"];
const SCRATCH0: &str = "t0";
const SCRATCH1: &str = "t1";
const SCRATCH2: &str = "t2";
const ADDRESS: &str = "t6";

/// RV64IM LP64 ELF assembly backend.
///
/// The backend emits assembly, not an object file or executable. A configured
/// assembler/linker stage must validate and assemble the result before it is
/// treated as an executable artifact.
#[derive(Debug, Clone, Copy, Default)]
pub struct RiscVBackend;

impl Backend for RiscVBackend {
    fn target_name(&self) -> &str {
        TARGET
    }

    fn file_extension(&self) -> &str {
        ".s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        validate_module(module)?;

        let mut out = String::new();
        writeln!(out, "    .attribute arch, \"rv64im\"").map_err(fmt_error)?;
        writeln!(out, "    .text").map_err(fmt_error)?;
        writeln!(out, "    .option nopic").map_err(fmt_error)?;
        writeln!(out, "    # Zamani module: {}", safe_comment(&module.name))
            .map_err(fmt_error)?;

        for function in &module.functions {
            if function.is_external {
                continue;
            }
            emit_function(function, &mut out)?;
        }

        Ok(out)
    }
}

fn error(message: impl Into<String>) -> CodeGenError {
    CodeGenError::new(message, TARGET)
}

fn fmt_error(_: std::fmt::Error) -> CodeGenError {
    error("failed to format generated RISC-V assembly")
}

fn validate_module(module: &IrModule) -> Result<(), CodeGenError> {
    if !module.globals.is_empty() {
        return Err(error(
            "module globals are not supported by this RISC-V emitter yet",
        ));
    }
    if !module.string_literals.is_empty() {
        return Err(error(
            "string literals are not supported by this RISC-V emitter yet",
        ));
    }

    let mut symbols = HashSet::new();
    for function in &module.functions {
        validate_symbol(&function.name, "function")?;
        if !symbols.insert(function.name.as_str()) {
            return Err(error(format!(
                "duplicate function symbol `{}`",
                function.name
            )));
        }

        validate_type(&function.return_type, "function return type")?;
        for (name, ty) in &function.params {
            validate_register_name(name)?;
            validate_type(ty, "function parameter")?;
        }

        if function.is_external {
            continue;
        }
        validate_function(function)?;
    }

    Ok(())
}

fn validate_function(function: &IrFunction) -> Result<(), CodeGenError> {
    let mut definitions = HashSet::<String>::new();
    let mut labels = HashSet::<String>::new();

    for (name, _) in &function.params {
        if !definitions.insert(name.clone()) {
            return Err(error(format!(
                "function `{}` has duplicate parameter `{name}`",
                function.name
            )));
        }
    }

    for instruction in &function.body {
        if let IrInstruction::Label(label) = instruction {
            validate_label(label)?;
            if !labels.insert(label.clone()) {
                return Err(error(format!(
                    "function `{}` defines label `{label}` more than once",
                    function.name
                )));
            }
        }

        if let Some((register, ty)) = definition(instruction) {
            validate_register_name(&register.0)?;
            validate_type(ty, "instruction result")?;
            if !definitions.insert(register.0.clone()) {
                return Err(error(format!(
                    "function `{}` defines register `{}` more than once; this emitter expects unique SSA-like register names",
                    function.name, register.0
                )));
            }
        }

        validate_instruction(instruction)?;
    }

    for instruction in &function.body {
        match instruction {
            IrInstruction::Jump(label) => require_label(&labels, label, &function.name)?,
            IrInstruction::CondJump(_, yes, no) => {
                require_label(&labels, yes, &function.name)?;
                require_label(&labels, no, &function.name)?;
            }
            _ => {}
        }
    }

    // This is a structural safety check, not a full control-flow proof.
    // The authoritative IR verifier remains responsible for dominance,
    // type consistency, reachability, and whole-function invariants.
    Ok(())
}

fn definition(instruction: &IrInstruction) -> Option<(&crate::ir_gen::IrRegister, &IrType)> {
    use IrInstruction::*;
    match instruction {
        Add(r, _, _) | Sub(r, _, _) | Mul(r, _, _) | Div(r, _, _)
        | Rem(r, _, _) | And(r, _, _) | Or(r, _, _) | Xor(r, _, _)
        | Shl(r, _, _) | Shr(r, _, _) | Cmp(r, _, _, _)
        | Assign(r, _) | Neg(r, _) | Not(r, _) | ZExt(r, _, _)
        | SExt(r, _, _) | Trunc(r, _, _) => Some((r, &r.1)),
        Call(Some(r), _, _) => Some((r, &r.1)),
        _ => None,
    }
}

fn validate_instruction(instruction: &IrInstruction) -> Result<(), CodeGenError> {
    use IrInstruction::*;

    match instruction {
        Add(r, a, b) | Sub(r, a, b) | Mul(r, a, b) | Div(r, a, b)
        | Rem(r, a, b) | And(r, a, b) | Or(r, a, b) | Xor(r, a, b)
        | Shl(r, a, b) | Shr(r, a, b) => {
            require_integer(&r.1, "arithmetic result")?;
            require_integer(&a.ty(), "arithmetic operand")?;
            require_integer(&b.ty(), "arithmetic operand")?;
            require_same_type(&a.ty(), &b.ty(), "binary operands")?;
            require_same_type(&r.1, &a.ty(), "binary result and operands")?;
            validate_value(a)?;
            validate_value(b)?;
        }
        Neg(r, value) => {
            require_integer(&r.1, "negation result")?;
            require_integer(&value.ty(), "negation operand")?;
            require_same_type(&r.1, &value.ty(), "negation result and operand")?;
            validate_value(value)?;
        }
        Not(r, value) => {
            require_integer(&r.1, "bitwise-not result")?;
            require_integer(&value.ty(), "bitwise-not operand")?;
            require_same_type(&r.1, &value.ty(), "bitwise-not result and operand")?;
            validate_value(value)?;
        }
        Cmp(r, op, a, b) => {
            if r.1 != IrType::Bool {
                return Err(error("comparison result must have boolean type"));
            }
            require_integer(&a.ty(), "comparison operand")?;
            require_integer(&b.ty(), "comparison operand")?;
            require_same_type(&a.ty(), &b.ty(), "comparison operands")?;
            if matches!(
                op,
                CmpOp::FEq | CmpOp::FNe | CmpOp::FLt | CmpOp::FLe | CmpOp::FGt | CmpOp::FGe
            ) {
                return Err(error(
                    "floating-point comparison is unsupported by the RV64IM emitter",
                ));
            }
            validate_value(a)?;
            validate_value(b)?;
        }
        Assign(r, value) => {
            require_same_type(&r.1, &value.ty(), "assignment")?;
            validate_value(value)?;
        }
        ZExt(r, value, target) | SExt(r, value, target) | Trunc(r, value, target) => {
            require_integer(&r.1, "conversion result")?;
            require_integer(&value.ty(), "conversion source")?;
            require_integer(target, "conversion target")?;
            require_same_type(&r.1, target, "conversion result and target")?;
            validate_value(value)?;
            match instruction {
                ZExt(_, _, _) if bit_width(&value.ty())? > bit_width(target)? => {
                    return Err(error("zext source is wider than its target"));
                }
                SExt(_, _, _) if bit_width(&value.ty())? > bit_width(target)? => {
                    return Err(error("sext source is wider than its target"));
                }
                Trunc(_, _, _) if bit_width(&value.ty())? < bit_width(target)? => {
                    return Err(error("trunc target is wider than its source"));
                }
                _ => {}
            }
        }
        Call(result, name, args) => {
            validate_symbol(name, "direct-call target")?;
            if let Some(register) = result {
                validate_type(&register.1, "call result")?;
                if register.1 == IrType::Void {
                    return Err(error("a call result register cannot have void type"));
                }
            }
            for arg in args {
                require_integer(&arg.ty(), "call argument")?;
                validate_value(arg)?;
            }
        }
        Ret(Some(value)) => {
            require_integer(&value.ty(), "return value")?;
            validate_value(value)?;
        }
        Ret(None) | Unreachable | Comment(_) => {}
        Label(label) => validate_label(label)?,
        Jump(label) => validate_label(label)?,
        CondJump(condition, yes, no) => {
            if condition.ty() != IrType::Bool {
                return Err(error(
                    "conditional branch condition must have boolean type",
                ));
            }
            validate_value(condition)?;
            validate_label(yes)?;
            validate_label(no)?;
        }
        // Deliberately rejected rather than silently approximated.
        Alloca(_, _)
        | Load(_, _)
        | Store(_, _)
        | GetElementPtr(_, _, _)
        | BitCast(_, _, _)
        | Phi(_, _)
        | FpExt(_, _, _)
        | FpTrunc(_, _, _)
        | SIToFP(_, _, _)
        | FPToSI(_, _, _)
        | CallIndirect(_, _, _)
        | QuantumGate(_, _, _)
        | NanoOp(_, _, _)
        | SankofaRecall(_, _)
        | SankofaRemember(_, _) => {
            return Err(error(format!(
                "unsupported IR instruction for RV64IM: {}",
                instruction_name(instruction)
            )));
        }
    }

    Ok(())
}

fn validate_value(value: &IrValue) -> Result<(), CodeGenError> {
    match value {
        IrValue::Reg(register) => {
            validate_register_name(&register.0)?;
            require_integer(&register.1, "register value")
        }
        IrValue::ConstInt(_, ty) => require_integer(ty, "integer constant"),
        IrValue::ConstBool(_) => Ok(()),
        IrValue::ConstNull => Err(error(
            "null pointer values are not supported by the scalar RV64IM emitter",
        )),
        IrValue::ConstFloat(_, _) => Err(error(
            "floating-point constants are unsupported by the RV64IM emitter",
        )),
        IrValue::ConstStr(_) | IrValue::GlobalPtr(_, _) => Err(error(
            "string and global-address values are unsupported by the RV64IM emitter",
        )),
        IrValue::Void => Err(error("void is not a value in this instruction context")),
    }
}

fn validate_type(ty: &IrType, context: &str) -> Result<(), CodeGenError> {
    match ty {
        IrType::Void => Ok(()),
        IrType::Bool | IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64
        | IrType::U8 | IrType::U16 | IrType::U32 | IrType::U64 => Ok(()),
        IrType::I128 | IrType::U128 => Err(error(format!(
            "{context}: 128-bit integer operations are not supported by this RV64IM emitter"
        ))),
        IrType::F32 | IrType::F64 => Err(error(format!(
            "{context}: floating-point types require a separately configured RISC-V F/D profile"
        ))),
        IrType::Ptr(_) | IrType::Array(_, _) | IrType::Struct(_, _)
        | IrType::Function(_, _) | IrType::Quantum | IrType::Opaque(_) => Err(error(format!(
            "{context}: type `{}` is outside the supported RV64IM scalar subset",
            ty.ir_name()
        ))),
    }
}

fn require_integer(ty: &IrType, context: &str) -> Result<(), CodeGenError> {
    match ty {
        IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64
        | IrType::U8 | IrType::U16 | IrType::U32 | IrType::U64 => Ok(()),
        _ => Err(error(format!(
            "{context}: expected a supported integer type, got `{}`",
            ty.ir_name()
        ))),
    }
}

fn bit_width(ty: &IrType) -> Result<u32, CodeGenError> {
    match ty {
        IrType::I8 | IrType::U8 => Ok(8),
        IrType::I16 | IrType::U16 => Ok(16),
        IrType::I32 | IrType::U32 => Ok(32),
        IrType::I64 | IrType::U64 => Ok(64),
        _ => Err(error(format!("not a supported integer type: `{}`", ty.ir_name()))),
    }
}

fn is_signed(ty: &IrType) -> bool {
    matches!(ty, IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64)
}

fn require_same_type(
    left: &IrType,
    right: &IrType,
    context: &str,
) -> Result<(), CodeGenError> {
    if left == right {
        Ok(())
    } else {
        Err(error(format!(
            "{context}: type mismatch (`{}` versus `{}`)",
            left.ir_name(),
            right.ir_name()
        )))
    }
}

fn require_label(
    labels: &HashSet<String>,
    label: &str,
    function: &str,
) -> Result<(), CodeGenError> {
    if labels.contains(label) {
        Ok(())
    } else {
        Err(error(format!(
            "function `{function}` branches to undefined label `{label}`"
        )))
    }
}

fn validate_symbol(name: &str, context: &str) -> Result<(), CodeGenError> {
    if valid_identifier(name) {
        Ok(())
    } else {
        Err(error(format!("invalid {context} symbol `{name}`")))
    }
}

fn validate_register_name(name: &str) -> Result<(), CodeGenError> {
    if valid_identifier(name) && !name.starts_with(".L") {
        Ok(())
    } else {
        Err(error(format!("invalid IR register name `{name}`")))
    }
}

fn validate_label(name: &str) -> Result<(), CodeGenError> {
    if valid_identifier(name) {
        Ok(())
    } else {
        Err(error(format!("invalid IR label `{name}`")))
    }
}

fn valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || matches!(first, '_' | '.' | '$'))
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '$'))
}

fn instruction_name(instruction: &IrInstruction) -> &'static str {
    use IrInstruction::*;
    match instruction {
        Alloca(_, _) => "alloca",
        Load(_, _) => "load",
        Store(_, _) => "store",
        Add(_, _, _) => "add",
        Sub(_, _, _) => "sub",
        Mul(_, _, _) => "mul",
        Div(_, _, _) => "div",
        Rem(_, _, _) => "rem",
        Neg(_, _) => "neg",
        And(_, _, _) => "and",
        Or(_, _, _) => "or",
        Xor(_, _, _) => "xor",
        Shl(_, _, _) => "shl",
        Shr(_, _, _) => "shr",
        Not(_, _) => "not",
        Cmp(_, _, _, _) => "cmp",
        Label(_) => "label",
        Jump(_) => "jump",
        CondJump(_, _, _) => "conditional branch",
        Ret(_) => "return",
        Unreachable => "unreachable",
        Call(_, _, _) => "direct call",
        CallIndirect(_, _, _) => "indirect call",
        GetElementPtr(_, _, _) => "getelementptr",
        BitCast(_, _, _) => "bitcast",
        Assign(_, _) => "assign",
        Phi(_, _) => "phi",
        ZExt(_, _, _) => "zext",
        SExt(_, _, _) => "sext",
        Trunc(_, _, _) => "trunc",
        FpExt(_, _, _) => "fpext",
        FpTrunc(_, _, _) => "fptrunc",
        SIToFP(_, _, _) => "sitofp",
        FPToSI(_, _, _) => "fptosi",
        QuantumGate(_, _, _) => "quantum gate",
        NanoOp(_, _, _) => "Nano operation",
        SankofaRecall(_, _) => "Sankofa recall",
        SankofaRemember(_, _) => "Sankofa remember",
        Comment(_) => "comment",
    }
}

#[derive(Debug)]
struct Frame {
    /// Each virtual register maps to a fixed-width spill slot.
    slots: BTreeMap<String, i64>,
    /// Total stack allocation, including the outgoing stack-argument area.
    frame_size: usize,
    /// Reserved outgoing argument bytes at the low end of the stack frame.
    outgoing_size: usize,
    /// Incoming parameters beyond a-registers are read from the caller frame.
    incoming_stack_params: HashMap<String, usize>,
}

impl Frame {
    fn build(function: &IrFunction) -> Result<Self, CodeGenError> {
        let mut names = BTreeSet::new();
        for (name, _) in &function.params {
            names.insert(name.clone());
        }
        for instruction in &function.body {
            if let Some((register, _)) = definition(instruction) {
                names.insert(register.0.clone());
            }
        }

        let max_outgoing = function
            .body
            .iter()
            .filter_map(|instruction| match instruction {
                IrInstruction::Call(_, _, args) => Some(args.len().saturating_sub(ARG_REGS.len())),
                _ => None,
            })
            .max()
            .unwrap_or(0);

        let outgoing_size = max_outgoing
            .checked_mul(8)
            .ok_or_else(|| error("outgoing call-argument area size overflow"))?;

        let mut slots = BTreeMap::new();
        for (index, name) in names.into_iter().enumerate() {
            let slot_bytes = index
                .checked_mul(8)
                .and_then(|v| v.checked_add(outgoing_size))
                .and_then(|v| v.checked_add(8))
                .ok_or_else(|| error("stack-frame slot offset overflow"))?;
            let offset = i64::try_from(slot_bytes)
                .map_err(|_| error("stack-frame slot offset exceeds supported address range"))?;
            slots.insert(name, -offset);
        }

        let slots_size = slots
            .len()
            .checked_mul(8)
            .ok_or_else(|| error("stack-frame size overflow"))?;
        let raw_size = outgoing_size
            .checked_add(slots_size)
            .and_then(|v| v.checked_add(16))
            .ok_or_else(|| error("stack-frame size overflow"))?;
        let frame_size = align_up(raw_size, 16)
            .ok_or_else(|| error("stack-frame alignment overflow"))?;

        let mut incoming_stack_params = HashMap::new();
        for (index, (name, _)) in function.params.iter().enumerate().skip(ARG_REGS.len()) {
            let offset = (index - ARG_REGS.len())
                .checked_mul(8)
                .ok_or_else(|| error("incoming argument offset overflow"))?;
            incoming_stack_params.insert(name.clone(), offset);
        }

        Ok(Self {
            slots,
            frame_size,
            outgoing_size,
            incoming_stack_params,
        })
    }

    fn slot(&self, name: &str) -> Result<i64, CodeGenError> {
        self.slots
            .get(name)
            .copied()
            .ok_or_else(|| error(format!("no stack slot allocated for register `{name}`")))
    }
}

fn align_up(value: usize, alignment: usize) -> Option<usize> {
    let remainder = value % alignment;
    if remainder == 0 {
        Some(value)
    } else {
        value.checked_add(alignment - remainder)
    }
}

fn emit_function(function: &IrFunction, output: &mut String) -> Result<(), CodeGenError> {
    let frame = Frame::build(function)?;

    writeln!(output, "\n    .globl {}", function.name).map_err(fmt_error)?;
    writeln!(output, "    .type {}, @function", function.name).map_err(fmt_error)?;
    writeln!(output, "{}:", function.name).map_err(fmt_error)?;

    emit_stack_adjust(output, false, frame.frame_size)?;
    emit_frame_store(output, "ra", -8)?;
    emit_frame_store(output, "s0", -16)?;
    emit_materialize_u64(output, "t6", frame.frame_size as u64);
    writeln!(output, "    add     s0, sp, t6").map_err(fmt_error)?;

    for (index, (name, _ty)) in function.params.iter().enumerate() {
        let slot = frame.slot(name)?;
        if index < ARG_REGS.len() {
            emit_frame_store(output, ARG_REGS[index], slot)?;
        } else {
            let incoming_offset = *frame
                .incoming_stack_params
                .get(name)
                .ok_or_else(|| error("missing incoming stack parameter metadata"))?;
            emit_load_base_offset(output, "t0", "s0", incoming_offset as i64)?;
            emit_frame_store(output, "t0", slot)?;
        }
    }

    for instruction in &function.body {
        emit_instruction(function, &frame, instruction, output)?;
    }

    writeln!(output, "    .size {}, .-{}", function.name, function.name)
        .map_err(fmt_error)?;
    Ok(())
}

fn emit_instruction(
    function: &IrFunction,
    frame: &Frame,
    instruction: &IrInstruction,
    output: &mut String,
) -> Result<(), CodeGenError> {
    use IrInstruction::*;

    match instruction {
        Add(r, a, b) | Sub(r, a, b) | Mul(r, a, b) | Div(r, a, b)
        | Rem(r, a, b) | And(r, a, b) | Or(r, a, b) | Xor(r, a, b)
        | Shl(r, a, b) | Shr(r, a, b) => {
            emit_value(a, frame, SCRATCH0, output)?;
            emit_value(b, frame, SCRATCH1, output)?;
            let mnemonic = match instruction {
                Add(_, _, _) => "add",
                Sub(_, _, _) => "sub",
                Mul(_, _, _) => "mul",
                Div(_, _, _) if is_signed(&a.ty()) => "div",
                Div(_, _, _) => "divu",
                Rem(_, _, _) if is_signed(&a.ty()) => "rem",
                Rem(_, _, _) => "remu",
                And(_, _, _) => "and",
                Or(_, _, _) => "or",
                Xor(_, _, _) => "xor",
                Shl(_, _, _) => "sll",
                Shr(_, _, _) if is_signed(&a.ty()) => "sra",
                Shr(_, _, _) => "srl",
                _ => unreachable!(),
            };
            writeln!(output, "    {mnemonic:<7} t2, t0, t1").map_err(fmt_error)?;
            emit_normalize(output, SCRATCH2, &r.1)?;
            emit_frame_store(output, SCRATCH2, frame.slot(&r.0)?)?;
        }
        Neg(r, value) => {
            emit_value(value, frame, SCRATCH0, output)?;
            writeln!(output, "    neg     t2, t0").map_err(fmt_error)?;
            emit_normalize(output, SCRATCH2, &r.1)?;
            emit_frame_store(output, SCRATCH2, frame.slot(&r.0)?)?;
        }
        Not(r, value) => {
            emit_value(value, frame, SCRATCH0, output)?;
            writeln!(output, "    not     t2, t0").map_err(fmt_error)?;
            emit_normalize(output, SCRATCH2, &r.1)?;
            emit_frame_store(output, SCRATCH2, frame.slot(&r.0)?)?;
        }
        Cmp(r, op, a, b) => {
            emit_value(a, frame, SCRATCH0, output)?;
            emit_value(b, frame, SCRATCH1, output)?;
            match op {
                CmpOp::Eq => {
                    writeln!(output, "    sub     t2, t0, t1").map_err(fmt_error)?;
                    writeln!(output, "    seqz    t2, t2").map_err(fmt_error)?;
                }
                CmpOp::Ne => {
                    writeln!(output, "    sub     t2, t0, t1").map_err(fmt_error)?;
                    writeln!(output, "    snez    t2, t2").map_err(fmt_error)?;
                }
                CmpOp::Lt => {
                    let op = if is_signed(&a.ty()) { "slt" } else { "sltu" };
                    writeln!(output, "    {op:<7} t2, t0, t1").map_err(fmt_error)?;
                }
                CmpOp::Gt => {
                    let op = if is_signed(&a.ty()) { "slt" } else { "sltu" };
                    writeln!(output, "    {op:<7} t2, t1, t0").map_err(fmt_error)?;
                }
                CmpOp::Le => {
                    let op = if is_signed(&a.ty()) { "slt" } else { "sltu" };
                    writeln!(output, "    {op:<7} t2, t1, t0").map_err(fmt_error)?;
                    writeln!(output, "    xori    t2, t2, 1").map_err(fmt_error)?;
                }
                CmpOp::Ge => {
                    let op = if is_signed(&a.ty()) { "slt" } else { "sltu" };
                    writeln!(output, "    {op:<7} t2, t0, t1").map_err(fmt_error)?;
                    writeln!(output, "    xori    t2, t2, 1").map_err(fmt_error)?;
                }
                CmpOp::FEq | CmpOp::FNe | CmpOp::FLt | CmpOp::FLe | CmpOp::FGt | CmpOp::FGe => {
                    return Err(error("floating-point comparison is unsupported"));
                }
            }
            emit_frame_store(output, SCRATCH2, frame.slot(&r.0)?)?;
        }
        Assign(r, value) => {
            emit_value(value, frame, SCRATCH0, output)?;
            emit_normalize(output, SCRATCH0, &r.1)?;
            emit_frame_store(output, SCRATCH0, frame.slot(&r.0)?)?;
        }
        ZExt(r, value, target) => {
            emit_value(value, frame, SCRATCH0, output)?;
            emit_zero_extend(output, SCRATCH0, &value.ty(), target)?;
            emit_frame_store(output, SCRATCH0, frame.slot(&r.0)?)?;
        }
        SExt(r, value, target) => {
            emit_value(value, frame, SCRATCH0, output)?;
            emit_sign_extend(output, SCRATCH0, &value.ty(), target)?;
            emit_frame_store(output, SCRATCH0, frame.slot(&r.0)?)?;
        }
        Trunc(r, value, target) => {
            emit_value(value, frame, SCRATCH0, output)?;
            emit_normalize(output, SCRATCH0, target)?;
            emit_frame_store(output, SCRATCH0, frame.slot(&r.0)?)?;
        }
        Call(result, name, args) => {
            // Values are materialized one at a time. Each value is loaded from
            // its stable spill slot, so argument-register setup cannot destroy
            // another argument's source.
            for (index, argument) in args.iter().enumerate() {
                emit_value(argument, frame, SCRATCH0, output)?;
                if index < ARG_REGS.len() {
                    writeln!(output, "    mv      {}, t0", ARG_REGS[index])
                        .map_err(fmt_error)?;
                } else {
                    let stack_offset = (index - ARG_REGS.len())
                        .checked_mul(8)
                        .ok_or_else(|| error("outgoing argument offset overflow"))?;
                    emit_store_base_offset(
                        output,
                        "t0",
                        "sp",
                        i64::try_from(stack_offset)
                            .map_err(|_| error("outgoing argument offset too large"))?,
                    )?;
                }
            }
            writeln!(output, "    call    {name}").map_err(fmt_error)?;
            if let Some(register) = result {
                emit_normalize(output, "a0", &register.1)?;
                emit_frame_store(output, "a0", frame.slot(&register.0)?)?;
            }
        }
        Label(label) => {
            writeln!(output, "{}:", local_label(&function.name, label))
                .map_err(fmt_error)?;
        }
        Jump(label) => {
            writeln!(output, "    j       {}", local_label(&function.name, label))
                .map_err(fmt_error)?;
        }
        CondJump(condition, yes, no) => {
            emit_value(condition, frame, SCRATCH0, output)?;
            writeln!(output, "    bnez    t0, {}", local_label(&function.name, yes))
                .map_err(fmt_error)?;
            writeln!(output, "    j       {}", local_label(&function.name, no))
                .map_err(fmt_error)?;
        }
        Ret(value) => {
            if let Some(value) = value {
                emit_value(value, frame, "a0", output)?;
                emit_normalize(output, "a0", &value.ty())?;
            } else {
                writeln!(output, "    li      a0, 0").map_err(fmt_error)?;
            }
            emit_epilogue(output)?;
        }
        Unreachable => {
            writeln!(output, "    unimp").map_err(fmt_error)?;
        }
        Comment(comment) => {
            writeln!(output, "    # {}", safe_comment(comment)).map_err(fmt_error)?;
        }
        Alloca(_, _) | Load(_, _) | Store(_, _) | GetElementPtr(_, _, _)
        | BitCast(_, _, _) | Phi(_, _) | FpExt(_, _, _) | FpTrunc(_, _, _)
        | SIToFP(_, _, _) | FPToSI(_, _, _) | CallIndirect(_, _, _)
        | QuantumGate(_, _, _) | NanoOp(_, _, _) | SankofaRecall(_, _)
        | SankofaRemember(_, _) => {
            return Err(error(format!(
                "unsupported IR instruction reached emission: {}",
                instruction_name(instruction)
            )));
        }
    }

    Ok(())
}

fn emit_value(
    value: &IrValue,
    frame: &Frame,
    destination: &str,
    output: &mut String,
) -> Result<(), CodeGenError> {
    match value {
        IrValue::Reg(register) => {
            emit_frame_load(output, destination, frame.slot(&register.0)?)
        }
        IrValue::ConstInt(value, _) => emit_materialize_i64(output, destination, *value),
        IrValue::ConstBool(value) => {
            writeln!(output, "    li      {destination}, {}", i32::from(*value))
                .map_err(fmt_error)
        }
        _ => Err(error(format!(
            "unsupported value reached emission: `{}`",
            value.to_ir_string()
        ))),
    }
}

fn emit_normalize(
    output: &mut String,
    register: &str,
    ty: &IrType,
) -> Result<(), CodeGenError> {
    match ty {
        IrType::I8 => {
            writeln!(output, "    slli    {register}, {register}, 56").map_err(fmt_error)?;
            writeln!(output, "    srai    {register}, {register}, 56").map_err(fmt_error)?;
        }
        IrType::I16 => {
            writeln!(output, "    slli    {register}, {register}, 48").map_err(fmt_error)?;
            writeln!(output, "    srai    {register}, {register}, 48").map_err(fmt_error)?;
        }
        IrType::I32 => {
            writeln!(output, "    addiw   {register}, {register}, 0").map_err(fmt_error)?;
        }
        IrType::U8 => {
            writeln!(output, "    andi    {register}, {register}, 255").map_err(fmt_error)?;
        }
        IrType::U16 => {
            writeln!(output, "    slli    {register}, {register}, 48").map_err(fmt_error)?;
            writeln!(output, "    srli    {register}, {register}, 48").map_err(fmt_error)?;
        }
        IrType::U32 => {
            writeln!(output, "    slli    {register}, {register}, 32").map_err(fmt_error)?;
            writeln!(output, "    srli    {register}, {register}, 32").map_err(fmt_error)?;
        }
        IrType::I64 | IrType::U64 | IrType::Bool => {}
        _ => return Err(error(format!("cannot normalize unsupported type `{}`", ty.ir_name()))),
    }
    Ok(())
}

fn emit_zero_extend(
    output: &mut String,
    register: &str,
    source: &IrType,
    target: &IrType,
) -> Result<(), CodeGenError> {
    let source_width = bit_width(source)?;
    let target_width = bit_width(target)?;
    if source_width > target_width {
        return Err(error("invalid zero extension: source is wider than target"));
    }
    match source_width {
        8 => writeln!(output, "    andi    {register}, {register}, 255").map_err(fmt_error)?,
        16 => {
            writeln!(output, "    slli    {register}, {register}, 48").map_err(fmt_error)?;
            writeln!(output, "    srli    {register}, {register}, 48").map_err(fmt_error)?;
        }
        32 => {
            writeln!(output, "    slli    {register}, {register}, 32").map_err(fmt_error)?;
            writeln!(output, "    srli    {register}, {register}, 32").map_err(fmt_error)?;
        }
        64 => {}
        _ => return Err(error("unsupported zero-extension source width")),
    }
    emit_normalize(output, register, target)
}

fn emit_sign_extend(
    output: &mut String,
    register: &str,
    source: &IrType,
    target: &IrType,
) -> Result<(), CodeGenError> {
    let source_width = bit_width(source)?;
    let target_width = bit_width(target)?;
    if source_width > target_width {
        return Err(error("invalid sign extension: source is wider than target"));
    }
    match source_width {
        8 => {
            writeln!(output, "    slli    {register}, {register}, 56").map_err(fmt_error)?;
            writeln!(output, "    srai    {register}, {register}, 56").map_err(fmt_error)?;
        }
        16 => {
            writeln!(output, "    slli    {register}, {register}, 48").map_err(fmt_error)?;
            writeln!(output, "    srai    {register}, {register}, 48").map_err(fmt_error)?;
        }
        32 => {
            writeln!(output, "    addiw   {register}, {register}, 0").map_err(fmt_error)?;
        }
        64 => {}
        _ => return Err(error("unsupported sign-extension source width")),
    }
    emit_normalize(output, register, target)
}

fn emit_materialize_i64(
    output: &mut String,
    register: &str,
    value: i64,
) -> Result<(), CodeGenError> {
    writeln!(output, "    li      {register}, {value}").map_err(fmt_error)
}

fn emit_materialize_u64(
    output: &mut String,
    register: &str,
    value: u64,
) -> Result<(), CodeGenError> {
    writeln!(output, "    li      {register}, {value}").map_err(fmt_error)
}

fn emit_frame_load(
    output: &mut String,
    register: &str,
    offset: i64,
) -> Result<(), CodeGenError> {
    emit_load_base_offset(output, register, "s0", offset)
}

fn emit_frame_store(
    output: &mut String,
    register: &str,
    offset: i64,
) -> Result<(), CodeGenError> {
    emit_store_base_offset(output, register, "s0", offset)
}

fn emit_load_base_offset(
    output: &mut String,
    register: &str,
    base: &str,
    offset: i64,
) -> Result<(), CodeGenError> {
    if (-2048..=2047).contains(&offset) {
        writeln!(output, "    ld      {register}, {offset}({base})").map_err(fmt_error)
    } else {
        emit_materialize_i64(output, ADDRESS, offset)?;
        writeln!(output, "    add     {ADDRESS}, {base}, {ADDRESS}").map_err(fmt_error)?;
        writeln!(output, "    ld      {register}, 0({ADDRESS})").map_err(fmt_error)
    }
}

fn emit_store_base_offset(
    output: &mut String,
    register: &str,
    base: &str,
    offset: i64,
) -> Result<(), CodeGenError> {
    if (-2048..=2047).contains(&offset) {
        writeln!(output, "    sd      {register}, {offset}({base})").map_err(fmt_error)
    } else {
        emit_materialize_i64(output, ADDRESS, offset)?;
        writeln!(output, "    add     {ADDRESS}, {base}, {ADDRESS}").map_err(fmt_error)?;
        writeln!(output, "    sd      {register}, 0({ADDRESS})").map_err(fmt_error)
    }
}

fn emit_stack_adjust(
    output: &mut String,
    restore: bool,
    bytes: usize,
) -> Result<(), CodeGenError> {
    let signed = i64::try_from(bytes).map_err(|_| error("stack adjustment is too large"))?;
    if signed <= 2048 {
        if restore {
            writeln!(output, "    addi    sp, sp, {signed}").map_err(fmt_error)
        } else {
            writeln!(output, "    addi    sp, sp, -{signed}").map_err(fmt_error)
        }
    } else {
        emit_materialize_i64(output, ADDRESS, signed)?;
        if restore {
            writeln!(output, "    add     sp, sp, {ADDRESS}").map_err(fmt_error)
        } else {
            writeln!(output, "    sub     sp, sp, {ADDRESS}").map_err(fmt_error)
        }
    }
}

fn emit_epilogue(output: &mut String) -> Result<(), CodeGenError> {
    emit_frame_load(output, "ra", -8)?;
    emit_frame_load(output, "t6", -16)?;
    writeln!(output, "    mv      sp, s0").map_err(fmt_error)?;
    writeln!(output, "    mv      s0, t6").map_err(fmt_error)?;
    writeln!(output, "    ret").map_err(fmt_error)
}

fn local_label(function: &str, label: &str) -> String {
    format!(".L{}_{}", function, label)
}

fn safe_comment(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch == '\n' || ch == '\r' { ' ' } else { ch })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrRegister};

    fn reg(name: &str, ty: IrType) -> IrRegister {
        IrRegister::new(name, ty)
    }

    fn module_with(function: IrFunction) -> IrModule {
        let mut module = IrModule::new("riscv_test");
        module.add_function(function);
        module
    }

    #[test]
    fn reports_the_target_contract() {
        let backend = RiscVBackend;
        assert_eq!(backend.target_name(), TARGET);
        assert_eq!(backend.file_extension(), ".s");
    }

    #[test]
    fn emits_integer_arithmetic_and_return() {
        let mut function = IrFunction::new(
            "add_pair",
            vec![("left".into(), IrType::I64), ("right".into(), IrType::I64)],
            IrType::I64,
        );
        function.push(IrInstruction::Add(
            reg("sum", IrType::I64),
            IrValue::Reg(reg("left", IrType::I64)),
            IrValue::Reg(reg("right", IrType::I64)),
        ));
        function.push(IrInstruction::Ret(Some(IrValue::Reg(reg(
            "sum",
            IrType::I64,
        )))));

        let assembly = RiscVBackend.generate(&module_with(function)).unwrap();
        assert!(assembly.contains(".attribute arch, \"rv64im\""));
        assert!(assembly.contains("add     t2, t0, t1"));
        assert!(assembly.contains("ret"));
    }

    #[test]
    fn rejects_unsupported_memory_instead_of_emitting_comments() {
        let mut function = IrFunction::new("bad_memory", vec![], IrType::Void);
        function.push(IrInstruction::Alloca(
            reg("ptr", IrType::Ptr(Box::new(IrType::I64))),
            IrType::I64,
        ));

        let result = RiscVBackend.generate(&module_with(function));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_undefined_branch_targets() {
        let mut function = IrFunction::new("bad_branch", vec![], IrType::Void);
        function.push(IrInstruction::Jump("missing".into()));

        let result = RiscVBackend.generate(&module_with(function));
        assert!(result.is_err());
    }

    #[test]
    fn supports_more_than_eight_arguments_without_a_fixed_argument_limit() {
        let parameters = (0..10)
            .map(|index| (format!("p{index}"), IrType::I64))
            .collect();
        let function = IrFunction::new("many_args", parameters, IrType::Void);
        let assembly = RiscVBackend.generate(&module_with(function)).unwrap();
        assert!(assembly.contains("0(s0)"));
        assert!(assembly.contains("8(s0)"));
    }

    #[test]
    fn rejects_floating_point_types_explicitly() {
        let function = IrFunction::new("float_fn", vec![], IrType::F64);
        assert!(RiscVBackend.generate(&module_with(function)).is_err());
    }
}
