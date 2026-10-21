
//! Zamani AArch64 native assembly backend.
//!
//! Target contract:
//! - Architecture: AArch64 / ARM64.
//! - Object format: ELF.
//! - Assembly syntax: GNU assembler, AArch64.
//! - Calling convention: AAPCS64, integer/pointer argument registers x0..x7.
//! - Stack: 16-byte aligned at public call boundaries.
//! - Frame pointer: x29; link register: x30.
//! - Scratch registers: x9, x10, x11, x12.
//!
//! This module emits assembly text only. It does not write files, execute
//! external tools, link executables, or select hardware.
//!
//! The implementation intentionally supports a documented scalar subset of
//! the current Zamani IR. Unsupported types and instructions return errors;
//! they are never replaced with comments, zero values, or no-op instructions.
//!
//! Integration:
//! - Uses the existing `crate::backend::{Backend, CodeGenError}` contract.
//! - Consumes `crate::ir_gen::{IrModule, IrFunction, IrInstruction, IrValue}`.
//! - Is selected for `CompilationTarget::Arm64` by the existing dispatcher.
//! - Does not own semantic analysis, IR verification, LLVM, quantum routing,
//!   HDL synthesis, linking, or runtime resource allocation.
//!
//! Safety: safe Rust only; no `unsafe`.
//!
//! Portability note:
//! This is an AArch64 ELF target, not a universal ARM target. Mach-O,
//! COFF/Windows, ILP32, variadic ABI details, and platform-specific symbol
//! conventions require separate target configurations.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use crate::backend::{Backend, CodeGenError};
use crate::ir_gen::{CmpOp, IrFunction, IrGlobal, IrInstruction, IrModule, IrType, IrValue};

const TARGET: &str = "aarch64-elf";
const ARGUMENT_REGISTERS: [&str; 8] =
    ["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"];
const SCRATCH_LEFT: &str = "x9";
const SCRATCH_RIGHT: &str = "x10";
const SCRATCH_EXTRA: &str = "x11";
const SCRATCH_ADDRESS: &str = "x12";

/// AArch64 ELF assembly emitter for the supported scalar IR subset.
#[derive(Debug, Clone, Copy, Default)]
pub struct Aarch64Backend;

impl Backend for Aarch64Backend {
    fn target_name(&self) -> &str {
        TARGET
    }

    fn file_extension(&self) -> &str {
        ".s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        validate_module(module)?;

        let mut output = String::new();
        writeln!(output, "// Zamani AArch64 ELF assembly").expect("writing to String");
        writeln!(output, "// Module: {}", safe_comment(&module.name))
            .expect("writing to String");
        writeln!(output, ".text").expect("writing to String");

        let mut external_symbols = HashSet::new();
        for function in &module.functions {
            if function.is_external {
                external_symbols.insert(function.name.as_str());
            }
        }

        let defined_symbols: HashSet<&str> = module
            .functions
            .iter()
            .filter(|function| !function.is_external)
            .map(|function| function.name.as_str())
            .collect();

        let mut emitted_externs = HashSet::new();
        for function in &module.functions {
            if function.is_external && !defined_symbols.contains(function.name.as_str()) {
                if emitted_externs.insert(function.name.as_str()) {
                    writeln!(output, ".extern {}", function.name)
                        .expect("writing to String");
                }
            }
        }

        if !module.functions.is_empty() {
            output.push('\n');
        }

        for function in &module.functions {
            if function.is_external {
                continue;
            }

            emit_function(function, &mut output)?;
            output.push('\n');
        }

        emit_read_only_strings(module, &mut output)?;
        emit_globals(&module.globals, &mut output)?;

        // ELF marks the object as not requiring an executable stack.
        writeln!(output, ".section .note.GNU-stack,\"\",%progbits")
            .expect("writing to String");

        Ok(output)
    }
}

fn error(message: impl Into<String>) -> CodeGenError {
    CodeGenError::new(message, TARGET)
}

fn validate_module(module: &IrModule) -> Result<(), CodeGenError> {
    validate_symbol(&module.name, "module")?;

    let mut symbols = HashSet::new();

    for function in &module.functions {
        validate_symbol(&function.name, "function")?;

        if !symbols.insert(function.name.as_str()) {
            return Err(error(format!(
                "duplicate function declaration or definition `{}`",
                function.name
            )));
        }

        validate_type(&function.return_type, "function return type")?;

        let mut parameter_names = HashSet::new();
        for (name, ty) in &function.params {
            validate_register_name(name)?;
            validate_type(ty, "function parameter")?;

            if !parameter_names.insert(name.as_str()) {
                return Err(error(format!(
                    "function `{}` has duplicate parameter `{name}`",
                    function.name
                )));
            }
        }

        if function.is_external {
            continue;
        }

        validate_function_body(function)?;
    }

    for global in &module.globals {
        validate_symbol(&global.name, "global")?;

        if !symbols.insert(global.name.as_str()) {
            return Err(error(format!(
                "duplicate global/function symbol `{}`",
                global.name
            )));
        }

        validate_global(global)?;
    }

    let mut string_names = HashSet::new();
    for (name, _) in &module.string_literals {
        validate_symbol(name, "string literal")?;

        if !symbols.insert(name.as_str()) || !string_names.insert(name.as_str()) {
            return Err(error(format!(
                "duplicate or conflicting string symbol `{name}`"
            )));
        }
    }

    Ok(())
}

fn validate_function_body(function: &IrFunction) -> Result<(), CodeGenError> {
    let mut labels = HashSet::new();
    let mut registers: HashMap<&str, &IrType> = HashMap::new();

    for (name, ty) in &function.params {
        registers.insert(name.as_str(), ty);
    }

    // First pass: collect definitions and labels. This allows forward branches
    // while still detecting duplicate labels and register definitions.
    for instruction in &function.body {
        if let IrInstruction::Label(label) = instruction {
            validate_label(label)?;

            if !labels.insert(label.as_str()) {
                return Err(error(format!(
                    "function `{}` has duplicate label `{label}`",
                    function.name
                )));
            }
        }

        if let Some((register, ty)) = instruction_definition(instruction) {
            validate_register_name(&register.0)?;
            validate_type(ty, "instruction result")?;

            if registers.insert(register.0.as_str(), ty).is_some() {
                return Err(error(format!(
                    "function `{}` redefines register `{}`",
                    function.name, register.0
                )));
            }
        }
    }

    // Second pass: validate every operand and branch target before emitting
    // any assembly for this function.
    for instruction in &function.body {
        validate_instruction(instruction, &registers, &labels, function)?;
    }

    Ok(())
}

fn instruction_definition(
    instruction: &IrInstruction,
) -> Option<(&crate::ir_gen::IrRegister, &IrType)> {
    use IrInstruction::*;

    match instruction {
        Add(r, _, _)
        | Sub(r, _, _)
        | Mul(r, _, _)
        | Div(r, _, _)
        | Rem(r, _, _)
        | Neg(r, _)
        | And(r, _, _)
        | Or(r, _, _)
        | Xor(r, _, _)
        | Shl(r, _, _)
        | Shr(r, _, _)
        | Not(r, _)
        | Cmp(r, _, _, _)
        | Assign(r, _)
        | CallIndirect(Some(r), _, _)
        | Call(Some(r), _, _) => Some((r, &r.1)),
        _ => None,
    }
}

fn validate_instruction(
    instruction: &IrInstruction,
    registers: &HashMap<&str, &IrType>,
    labels: &HashSet<&str>,
    function: &IrFunction,
) -> Result<(), CodeGenError> {
    use IrInstruction::*;

    let check_value = |value: &IrValue| validate_value(value, registers);

    match instruction {
        Comment(_) => Ok(()),
        Label(_) => Ok(()),

        Assign(destination, value) => {
            validate_integer_type(&destination.1, "assignment destination")?;
            check_value(value)?;
            ensure_same_type(&destination.1, &value.ty(), "assignment")
        }

        Add(destination, left, right)
        | Sub(destination, left, right)
        | Mul(destination, left, right)
        | Div(destination, left, right)
        | Rem(destination, left, right)
        | And(destination, left, right)
        | Or(destination, left, right)
        | Xor(destination, left, right)
        | Shl(destination, left, right)
        | Shr(destination, left, right) => {
            validate_integer_type(&destination.1, "arithmetic result")?;
            check_value(left)?;
            check_value(right)?;
            ensure_same_type(&destination.1, &left.ty(), "left operand")?;
            ensure_same_type(&destination.1, &right.ty(), "right operand")
        }

        Neg(destination, value) | Not(destination, value) => {
            validate_integer_type(&destination.1, "unary result")?;
            check_value(value)?;
            ensure_same_type(&destination.1, &value.ty(), "unary operand")
        }

        Cmp(destination, operator, left, right) => {
            if destination.1 != IrType::Bool {
                return Err(error("comparison result must have Bool type"));
            }
            check_value(left)?;
            check_value(right)?;
            ensure_same_type(&left.ty(), &right.ty(), "comparison operands")?;

            if !is_integer_type(&left.ty()) {
                return Err(error(format!(
                    "AArch64 scalar emitter does not support comparison type {:?}",
                    left.ty()
                )));
            }

            match operator {
                CmpOp::Eq | CmpOp::Ne | CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => Ok(()),
                _ => Err(error(format!(
                    "floating-point comparison `{operator:?}` is unsupported by this emitter"
                ))),
            }
        }

        Jump(label) => require_label(label, labels, function),

        CondJump(condition, true_label, false_label) => {
            check_value(condition)?;
            if condition.ty() != IrType::Bool {
                return Err(error("conditional branch requires a Bool condition"));
            }
            require_label(true_label, labels, function)?;
            require_label(false_label, labels, function)
        }

        Ret(value) => match value {
            Some(value) => {
                check_value(value)?;
                ensure_same_type(&function.return_type, &value.ty(), "return value")
            }
            None if function.return_type == IrType::Void => Ok(()),
            None => Err(error(format!(
                "function `{}` returns {:?} but has a void return",
                function.name, function.return_type
            ))),
        },

        Call(destination, name, arguments) => {
            validate_symbol(name, "call target")?;

            if arguments.len() > ARGUMENT_REGISTERS.len() {
                return Err(error(format!(
                    "call to `{name}` has {} arguments; outgoing stack arguments are not yet supported",
                    arguments.len()
                )));
            }

            for argument in arguments {
                check_value(argument)?;
                validate_integer_type(&argument.ty(), "call argument")?;
            }

            if let Some(destination) = destination {
                validate_integer_type(&destination.1, "call result")?;
            }

            Ok(())
        }

        Unreachable => Ok(()),

        // These operations require memory layout, pointer provenance, floating
        // point, conversion, SSA-phi, or domain-specific lowering contracts.
        // Reject them rather than emit semantically incorrect instructions.
        Alloca(..)
        | Load(..)
        | Store(..)
        | CallIndirect(..)
        | GetElementPtr(..)
        | BitCast(..)
        | Phi(..)
        | ZExt(..)
        | SExt(..)
        | Trunc(..)
        | FpExt(..)
        | FpTrunc(..)
        | SIToFP(..)
        | FPToSI(..)
        | QuantumGate(..)
        | NanoOp(..)
        | SankofaRecall(..)
        | SankofaRemember(..) => Err(error(format!(
            "instruction `{}` is not supported by the AArch64 scalar emitter",
            instruction_name(instruction)
        ))),
    }
}

fn validate_value(
    value: &IrValue,
    registers: &HashMap<&str, &IrType>,
) -> Result<(), CodeGenError> {
    match value {
        IrValue::Reg(register) => {
            let Some(known_type) = registers.get(register.0.as_str()) else {
                return Err(error(format!(
                    "use of undefined IR register `{}`",
                    register.0
                )));
            };

            ensure_same_type(known_type, &register.1, "register reference")?;
            validate_integer_type(known_type, "register operand")
        }
        IrValue::ConstInt(_, ty) => validate_integer_type(ty, "integer constant"),
        IrValue::ConstBool(_) => Ok(()),
        IrValue::ConstNull => Err(error(
            "null pointer values are not supported by the scalar emitter",
        )),
        IrValue::ConstFloat(..) => Err(error(
            "floating-point constants are not supported by the scalar emitter",
        )),
        IrValue::ConstStr(_) | IrValue::GlobalPtr(..) => Err(error(
            "pointer and string operands require memory/address lowering",
        )),
        IrValue::Void => Err(error("void is not a valid scalar operand")),
    }
}

fn validate_type(ty: &IrType, context: &str) -> Result<(), CodeGenError> {
    if is_integer_type(ty) {
        Ok(())
    } else {
        Err(error(format!(
            "{context}: type {ty:?} is outside the supported scalar subset"
        )))
    }
}

fn validate_integer_type(ty: &IrType, context: &str) -> Result<(), CodeGenError> {
    validate_type(ty, context)
}

fn is_integer_type(ty: &IrType) -> bool {
    matches!(ty, IrType::Bool | IrType::I64 | IrType::U64)
}

fn ensure_same_type(
    expected: &IrType,
    actual: &IrType,
    context: &str,
) -> Result<(), CodeGenError> {
    if expected == actual {
        Ok(())
    } else {
        Err(error(format!(
            "{context}: type mismatch; expected {expected:?}, got {actual:?}"
        )))
    }
}

fn require_label(
    label: &str,
    labels: &HashSet<&str>,
    function: &IrFunction,
) -> Result<(), CodeGenError> {
    if labels.contains(label) {
        Ok(())
    } else {
        Err(error(format!(
            "function `{}` branches to undefined label `{label}`",
            function.name
        )))
    }
}

fn validate_global(global: &IrGlobal) -> Result<(), CodeGenError> {
    match (&global.ty, &global.value) {
        (IrType::I64 | IrType::U64, IrValue::ConstInt(_, value_type))
            if value_type == &global.ty =>
        {
            Ok(())
        }
        (IrType::Bool, IrValue::ConstBool(_)) => Ok(()),
        _ => Err(error(format!(
            "global `{}` has unsupported type/value combination ({:?}, {:?})",
            global.name, global.ty, global.value
        ))),
    }
}

fn validate_symbol(name: &str, context: &str) -> Result<(), CodeGenError> {
    if valid_assembler_identifier(name) {
        Ok(())
    } else {
        Err(error(format!(
            "invalid {context} symbol `{name}`; expected an ASCII assembler identifier"
        )))
    }
}

fn validate_register_name(name: &str) -> Result<(), CodeGenError> {
    if valid_assembler_identifier(name) {
        Ok(())
    } else {
        Err(error(format!("invalid IR register name `{name}`")))
    }
}

fn validate_label(name: &str) -> Result<(), CodeGenError> {
    if valid_assembler_identifier(name) {
        Ok(())
    } else {
        Err(error(format!("invalid IR label `{name}`")))
    }
}

fn valid_assembler_identifier(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some(c) if c == '_' || c == '.' || c.is_ascii_alphabetic())
        && chars.all(|c| c == '_' || c == '.' || c.is_ascii_alphanumeric())
}

fn instruction_name(instruction: &IrInstruction) -> &'static str {
    use IrInstruction::*;

    match instruction {
        Alloca(..) => "alloca",
        Load(..) => "load",
        Store(..) => "store",
        Add(..) => "add",
        Sub(..) => "sub",
        Mul(..) => "mul",
        Div(..) => "div",
        Rem(..) => "rem",
        Neg(..) => "neg",
        And(..) => "and",
        Or(..) => "or",
        Xor(..) => "xor",
        Shl(..) => "shl",
        Shr(..) => "shr",
        Not(..) => "not",
        Cmp(..) => "cmp",
        Label(..) => "label",
        Jump(..) => "jump",
        CondJump(..) => "conditional jump",
        Ret(..) => "return",
        Unreachable => "unreachable",
        Call(..) => "call",
        CallIndirect(..) => "indirect call",
        GetElementPtr(..) => "getelementptr",
        BitCast(..) => "bitcast",
        Assign(..) => "assign",
        Phi(..) => "phi",
        ZExt(..) => "zext",
        SExt(..) => "sext",
        Trunc(..) => "trunc",
        FpExt(..) => "fpext",
        FpTrunc(..) => "fptrunc",
        SIToFP(..) => "sitofp",
        FPToSI(..) => "fptosi",
        QuantumGate(..) => "quantum gate",
        NanoOp(..) => "Nano operation",
        SankofaRecall(..) => "Sankofa recall",
        SankofaRemember(..) => "Sankofa remember",
        Comment(..) => "comment",
    }
}

fn emit_function(function: &IrFunction, output: &mut String) -> Result<(), CodeGenError> {
    let frame = Frame::for_function(function)?;

    writeln!(output, ".p2align 2").expect("writing to String");
    writeln!(output, ".globl {}", function.name).expect("writing to String");
    writeln!(output, ".type {}, %function", function.name).expect("writing to String");
    writeln!(output, "{}:", function.name).expect("writing to String");

    // Save FP/LR in a 16-byte pair, then allocate a 16-byte-aligned local frame.
    writeln!(output, "    stp x29, x30, [sp, #-16]!").expect("writing to String");
    writeln!(output, "    mov x29, sp").expect("writing to String");

    if frame.size != 0 {
        emit_stack_adjust(output, false, frame.size);
    }

    // Parameters are copied to stable frame slots. This makes values survive
    // calls, which may clobber caller-saved registers.
    for (index, (name, _)) in function.params.iter().enumerate() {
        let offset = frame.offset(name)?;
        if index < ARGUMENT_REGISTERS.len() {
            writeln!(output, "    str {}, [x29, #{}]", ARGUMENT_REGISTERS[index], offset)
                .expect("writing to String");
        } else {
            // AAPCS64 stack arguments begin at the caller's SP. Since x29
            // points at the saved FP/LR pair, the first stack argument is
            // at x29 + 16.
            let incoming_offset = 16usize
                .checked_add((index - ARGUMENT_REGISTERS.len()).checked_mul(8).ok_or_else(
                    || error("incoming stack-argument offset overflow"),
                )?)
                .ok_or_else(|| error("incoming stack-argument offset overflow"))?;

            if incoming_offset <= 32760 {
                writeln!(
                    output,
                    "    ldr x9, [x29, #{}]",
                    incoming_offset
                )
                .expect("writing to String");
            } else {
                writeln!(output, "    movz x9, #{}", incoming_offset & 0xffff)
                    .expect("writing to String");
                writeln!(output, "    movk x9, #{}, lsl #16", (incoming_offset >> 16) & 0xffff)
                    .expect("writing to String");
                writeln!(output, "    add x9, x29, x9").expect("writing to String");
                writeln!(output, "    ldr x9, [x9]").expect("writing to String");
            }

            emit_store_frame_offset(output, "x9", offset);
        }
    }

    for instruction in &function.body {
        emit_instruction(instruction, &frame, function, output)?;
    }

    // A function body without an explicit return is rejected rather than
    // inventing a return value or falling through into unrelated code.
    if !function
        .body
        .iter()
        .any(|instruction| matches!(instruction, IrInstruction::Ret(_) | IrInstruction::Unreachable))
    {
        return Err(error(format!(
            "function `{}` has no explicit return or unreachable terminator",
            function.name
        )));
    }

    writeln!(output, ".size {}, .-{}", function.name, function.name)
        .expect("writing to String");
    Ok(())
}

#[derive(Debug)]
struct Frame {
    offsets: HashMap<String, usize>,
    size: usize,
}

impl Frame {
    fn for_function(function: &IrFunction) -> Result<Self, CodeGenError> {
        let mut names = Vec::<String>::new();

        for (name, _) in &function.params {
            if !names.iter().any(|existing| existing == name) {
                names.push(name.clone());
            }
        }

        for instruction in &function.body {
            if let Some((register, _)) = instruction_definition(instruction) {
                if !names.iter().any(|existing| existing == &register.0) {
                    names.push(register.0.clone());
                }
            }
        }

        let mut offsets = HashMap::new();
        for (index, name) in names.iter().enumerate() {
            let slot = index
                .checked_add(1)
                .and_then(|n| n.checked_mul(8))
                .ok_or_else(|| error("stack frame size overflow"))?;
            offsets.insert(name.clone(), slot);
        }

        let unaligned = names
            .len()
            .checked_mul(8)
            .ok_or_else(|| error("stack frame size overflow"))?;
        let size = align_up(unaligned, 16)
            .ok_or_else(|| error("stack frame alignment overflow"))?;

        Ok(Self { offsets, size })
    }

    fn offset(&self, name: &str) -> Result<usize, CodeGenError> {
        self.offsets
            .get(name)
            .copied()
            .ok_or_else(|| error(format!("no stack slot allocated for `{name}`")))
    }

    fn slot(&self, name: &str) -> Result<String, CodeGenError> {
        let offset = self.offset(name)?;
        if offset <= 256 {
            Ok(format!("[x29, #-{offset}]"))
        } else {
            Err(error(format!(
                "frame slot for `{name}` exceeds direct addressing range; large-frame addressing is required"
            )))
        }
    }
}

fn align_up(value: usize, alignment: usize) -> Option<usize> {
    if alignment == 0 || !alignment.is_power_of_two() {
        return None;
    }
    value.checked_add(alignment - 1).map(|n| n & !(alignment - 1))
}

fn emit_stack_adjust(output: &mut String, add: bool, bytes: usize) {
    let operation = if add { "add" } else { "sub" };

    // AArch64 immediate arithmetic supports a 12-bit immediate, optionally
    // shifted by 12. Emit a sequence rather than imposing a universal frame
    // size ceiling.
    let mut remaining = bytes;
    while remaining != 0 {
        let chunk = if remaining >= 4096 && remaining % 4096 == 0 {
            let shifted = (remaining / 4096).min(4095);
            shifted * 4096
        } else {
            remaining.min(4095)
        };

        if chunk >= 4096 && chunk % 4096 == 0 {
            writeln!(
                output,
                "    {operation} sp, sp, #{}, lsl #12",
                chunk / 4096
            )
            .expect("writing to String");
        } else {
            writeln!(output, "    {operation} sp, sp, #{chunk}")
                .expect("writing to String");
        }

        remaining -= chunk;
    }
}

fn emit_store_frame_offset(output: &mut String, register: &str, offset: usize) {
    if offset <= 256 {
        writeln!(output, "    str {register}, [x29, #-{offset}]")
            .expect("writing to String");
    } else {
        emit_materialize_u64(output, SCRATCH_ADDRESS, offset as u64);
        writeln!(output, "    sub {SCRATCH_ADDRESS}, x29, {SCRATCH_ADDRESS}")
            .expect("writing to String");
        writeln!(output, "    str {register}, [{SCRATCH_ADDRESS}]")
            .expect("writing to String");
    }
}

fn emit_load_frame_offset(output: &mut String, register: &str, offset: usize) {
    if offset <= 256 {
        writeln!(output, "    ldr {register}, [x29, #-{offset}]")
            .expect("writing to String");
    } else {
        emit_materialize_u64(output, SCRATCH_ADDRESS, offset as u64);
        writeln!(output, "    sub {SCRATCH_ADDRESS}, x29, {SCRATCH_ADDRESS}")
            .expect("writing to String");
        writeln!(output, "    ldr {register}, [{SCRATCH_ADDRESS}]")
            .expect("writing to String");
    }
}

fn emit_instruction(
    instruction: &IrInstruction,
    frame: &Frame,
    function: &IrFunction,
    output: &mut String,
) -> Result<(), CodeGenError> {
    use IrInstruction::*;

    match instruction {
        Comment(comment) => {
            writeln!(output, "    // {}", safe_comment(comment)).expect("writing to String");
        }

        Label(label) => {
            writeln!(output, "{}:", local_label(&function.name, label))
                .expect("writing to String");
        }

        Assign(destination, value) => {
            emit_value(value, SCRATCH_LEFT, frame, output)?;
            emit_store_frame_offset(output, SCRATCH_LEFT, frame.offset(&destination.0)?);
        }

        Add(destination, left, right)
        | Sub(destination, left, right)
        | Mul(destination, left, right)
        | Div(destination, left, right)
        | Rem(destination, left, right)
        | And(destination, left, right)
        | Or(destination, left, right)
        | Xor(destination, left, right)
        | Shl(destination, left, right)
        | Shr(destination, left, right) => {
            emit_value(left, SCRATCH_LEFT, frame, output)?;
            emit_value(right, SCRATCH_RIGHT, frame, output)?;

            let operation = match instruction {
                Add(..) => "add",
                Sub(..) => "sub",
                Mul(..) => "mul",
                Div(..) if destination.1 == IrType::U64 => "udiv",
                Div(..) => "sdiv",
                Rem(..) => {
                    if destination.1 == IrType::U64 {
                        "udiv"
                    } else {
                        "sdiv"
                    }
                }
                And(..) => "and",
                Or(..) => "orr",
                Xor(..) => "eor",
                Shl(..) => "lsl",
                Shr(..) => {
                    if destination.1 == IrType::U64 {
                        "lsr"
                    } else {
                        "asr"
                    }
                }
                _ => unreachable!("matched arithmetic instruction"),
            };

            if matches!(instruction, Rem(..)) {
                writeln!(
                    output,
                    "    {operation} {SCRATCH_EXTRA}, {SCRATCH_LEFT}, {SCRATCH_RIGHT}"
                )
                .expect("writing to String");
                writeln!(
                    output,
                    "    msub {SCRATCH_LEFT}, {SCRATCH_EXTRA}, {SCRATCH_RIGHT}, {SCRATCH_LEFT}"
                )
                .expect("writing to String");
            } else {
                writeln!(
                    output,
                    "    {operation} {SCRATCH_LEFT}, {SCRATCH_LEFT}, {SCRATCH_RIGHT}"
                )
                .expect("writing to String");
            }

            emit_store_frame_offset(
                output,
                SCRATCH_LEFT,
                frame.offset(&destination.0)?,
            );
        }

        Neg(destination, value) => {
            emit_value(value, SCRATCH_LEFT, frame, output)?;
            writeln!(output, "    neg {SCRATCH_LEFT}, {SCRATCH_LEFT}")
                .expect("writing to String");
            emit_store_frame_offset(output, SCRATCH_LEFT, frame.offset(&destination.0)?);
        }

        Not(destination, value) => {
            emit_value(value, SCRATCH_LEFT, frame, output)?;
            writeln!(output, "    mvn {SCRATCH_LEFT}, {SCRATCH_LEFT}")
                .expect("writing to String");
            emit_store_frame_offset(output, SCRATCH_LEFT, frame.offset(&destination.0)?);
        }

        Cmp(destination, operator, left, right) => {
            emit_value(left, SCRATCH_LEFT, frame, output)?;
            emit_value(right, SCRATCH_RIGHT, frame, output)?;
            writeln!(output, "    cmp {SCRATCH_LEFT}, {SCRATCH_RIGHT}")
                .expect("writing to String");

            let condition = match operator {
                CmpOp::Eq => "eq",
                CmpOp::Ne => "ne",
                CmpOp::Lt if left.ty() == IrType::U64 => "lo",
                CmpOp::Le if left.ty() == IrType::U64 => "ls",
                CmpOp::Gt if left.ty() == IrType::U64 => "hi",
                CmpOp::Ge if left.ty() == IrType::U64 => "hs",
                CmpOp::Lt => "lt",
                CmpOp::Le => "le",
                CmpOp::Gt => "gt",
                CmpOp::Ge => "ge",
                _ => return Err(error("unsupported comparison operator")),
            };

            writeln!(output, "    cset w9, {condition}").expect("writing to String");
            emit_store_frame_offset(output, SCRATCH_LEFT, frame.offset(&destination.0)?);
        }

        Jump(label) => {
            writeln!(output, "    b {}", local_label(&function.name, label))
                .expect("writing to String");
        }

        CondJump(condition, true_label, false_label) => {
            emit_value(condition, SCRATCH_LEFT, frame, output)?;
            writeln!(output, "    cbnz {SCRATCH_LEFT}, {}", local_label(&function.name, true_label))
                .expect("writing to String");
            writeln!(output, "    b {}", local_label(&function.name, false_label))
                .expect("writing to String");
        }

        Ret(value) => {
            if let Some(value) = value {
                emit_value(value, "x0", frame, output)?;
            } else {
                writeln!(output, "    mov x0, xzr").expect("writing to String");
            }

            if frame.size != 0 {
                emit_stack_adjust(output, true, frame.size);
            }
            writeln!(output, "    ldp x29, x30, [sp], #16")
                .expect("writing to String");
            writeln!(output, "    ret").expect("writing to String");
        }

        Call(destination, name, arguments) => {
            for (index, argument) in arguments.iter().enumerate() {
                emit_value(argument, ARGUMENT_REGISTERS[index], frame, output)?;
            }

            writeln!(output, "    bl {name}").expect("writing to String");

            if let Some(destination) = destination {
                emit_store_frame_offset(
                    output,
                    "x0",
                    frame.offset(&destination.0)?,
                );
            }
        }

        Unreachable => {
            writeln!(output, "    brk #0").expect("writing to String");
        }

        _ => {
            return Err(error(format!(
                "internal error: unsupported instruction `{}` reached emission",
                instruction_name(instruction)
            )));
        }
    }

    Ok(())
}

fn emit_value(
    value: &IrValue,
    destination: &str,
    frame: &Frame,
    output: &mut String,
) -> Result<(), CodeGenError> {
    match value {
        IrValue::Reg(register) => {
            emit_load_frame_offset(output, destination, frame.offset(&register.0)?);
        }

        IrValue::ConstInt(number, _) => {
            emit_materialize_u64(output, destination, *number as u64);
        }

        IrValue::ConstBool(value) => {
            writeln!(
                output,
                "    mov {destination}, #{}",
                if *value { 1 } else { 0 }
            )
            .expect("writing to String");
        }

        _ => {
            return Err(error(format!(
                "cannot lower value `{}` in AArch64 scalar emitter",
                value.to_ir_string()
            )));
        }
    }

    Ok(())
}

fn emit_materialize_u64(output: &mut String, register: &str, value: u64) {
    // MOVZ/MOVK construct any 64-bit bit pattern using 16-bit chunks.
    let chunks = [
        (value & 0xffff) as u16,
        ((value >> 16) & 0xffff) as u16,
        ((value >> 32) & 0xffff) as u16,
        ((value >> 48) & 0xffff) as u16,
    ];

    let first = chunks.iter().position(|chunk| *chunk != 0).unwrap_or(0);
    writeln!(
        output,
        "    movz {register}, #{}, lsl #{}",
        chunks[first],
        first * 16
    )
    .expect("writing to String");

    for (index, chunk) in chunks.iter().enumerate() {
        if index != first && *chunk != 0 {
            writeln!(
                output,
                "    movk {register}, #{chunk}, lsl #{}",
                index * 16
            )
            .expect("writing to String");
        }
    }
}

fn local_label(function: &str, label: &str) -> String {
    format!(".Lzamani_{function}_{label}")
}

fn safe_comment(comment: &str) -> String {
    comment
        .chars()
        .map(|character| match character {
            '\n' | '\r' => ' ',
            character if character.is_control() => ' ',
            character => character,
        })
        .collect()
}

fn emit_read_only_strings(
    module: &IrModule,
    output: &mut String,
) -> Result<(), CodeGenError> {
    if module.string_literals.is_empty() {
        return Ok(());
    }

    writeln!(output, ".section .rodata").expect("writing to String");

    for (name, string) in &module.string_literals {
        validate_symbol(name, "string literal")?;
        writeln!(output, ".p2align 0").expect("writing to String");
        writeln!(output, "{}:", name).expect("writing to String");
        write_escaped_bytes(output, string.as_bytes());
        writeln!(output, "    .byte 0").expect("writing to String");
    }

    Ok(())
}

fn emit_globals(globals: &[IrGlobal], output: &mut String) -> Result<(), CodeGenError> {
    if globals.is_empty() {
        return Ok(());
    }

    writeln!(output, ".data").expect("writing to String");

    for global in globals {
        validate_symbol(&global.name, "global")?;
        writeln!(output, ".p2align 3").expect("writing to String");
        writeln!(output, ".globl {}", global.name).expect("writing to String");
        writeln!(output, ".type {}, %object", global.name).expect("writing to String");
        writeln!(output, "{}:", global.name).expect("writing to String");

        match (&global.ty, &global.value) {
            (IrType::I64 | IrType::U64, IrValue::ConstInt(value, _)) => {
                writeln!(output, "    .xword {value}").expect("writing to String");
            }
            (IrType::Bool, IrValue::ConstBool(value)) => {
                writeln!(output, "    .byte {}", u8::from(*value))
                    .expect("writing to String");
            }
            _ => {
                return Err(error(format!(
                    "unsupported initializer for global `{}`",
                    global.name
                )));
            }
        }

        let size = match global.ty {
            IrType::Bool => 1,
            IrType::I64 | IrType::U64 => 8,
            _ => return Err(error("unsupported global type")),
        };

        writeln!(output, ".size {}, {size}", global.name).expect("writing to String");
    }

    Ok(())
}

fn write_escaped_bytes(output: &mut String, bytes: &[u8]) {
    // Emit decimal bytes rather than assembler string escapes so quotes,
    // backslashes, Unicode UTF-8, and control bytes cannot inject assembly.
    for chunk in bytes.chunks(16) {
        output.push_str("    .byte ");
        for (index, byte) in chunk.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }
            write!(output, "{byte}").expect("writing to String");
        }
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrRegister};

    fn register(name: &str, ty: IrType) -> IrRegister {
        IrRegister(name.to_owned(), ty)
    }

    fn simple_module() -> IrModule {
        let mut module = IrModule::new("aarch64_test");
        let mut function = IrFunction::new(
            "sum",
            vec![
                ("left".to_owned(), IrType::I64),
                ("right".to_owned(), IrType::I64),
            ],
            IrType::I64,
        );

        let result = register("result", IrType::I64);
        function.push(IrInstruction::Add(
            result.clone(),
            IrValue::Reg(register("left", IrType::I64)),
            IrValue::Reg(register("right", IrType::I64)),
        ));
        function.push(IrInstruction::Ret(Some(IrValue::Reg(result))));
        module.add_function(function);
        module
    }

    #[test]
    fn emits_aarch64_not_x86_64() {
        let assembly = Aarch64Backend.generate(&simple_module()).unwrap();
        assert!(assembly.contains("stp x29, x30"));
        assert!(assembly.contains("add x9, x9, x10"));
        assert!(assembly.contains("ret"));
        assert!(!assembly.contains("%rax"));
        assert!(!assembly.contains("pushq"));
    }

    #[test]
    fn reports_correct_target_contract() {
        let backend = Aarch64Backend;
        assert_eq!(backend.target_name(), "aarch64-elf");
        assert_eq!(backend.file_extension(), ".s");
    }

    #[test]
    fn rejects_unsupported_float_values() {
        let mut module = IrModule::new("float_test");
        let mut function = IrFunction::new("f", vec![], IrType::I64);
        function.push(IrInstruction::Ret(Some(IrValue::ConstFloat(
            1.0,
            IrType::F64,
        ))));
        module.add_function(function);

        assert!(Aarch64Backend.generate(&module).is_err());
    }

    #[test]
    fn rejects_undefined_branch_targets() {
        let mut module = IrModule::new("branch_test");
        let mut function = IrFunction::new("f", vec![], IrType::Void);
        function.push(IrInstruction::Jump("missing".to_owned()));
        function.push(IrInstruction::Ret(None));
        module.add_function(function);

        assert!(Aarch64Backend.generate(&module).is_err());
    }

    #[test]
    fn rejects_duplicate_symbols() {
        let mut module = IrModule::new("duplicate_test");
        module.add_function(IrFunction::new("same", vec![], IrType::Void));
        module.add_function(IrFunction::new("same", vec![], IrType::Void));

        assert!(Aarch64Backend.generate(&module).is_err());
    }

    #[test]
    fn emits_large_integer_constants_without_truncation() {
        let mut module = IrModule::new("constant_test");
        let mut function = IrFunction::new("constant", vec![], IrType::I64);
        function.push(IrInstruction::Ret(Some(IrValue::ConstInt(
            i64::MIN,
            IrType::I64,
        ))));
        module.add_function(function);

        let assembly = Aarch64Backend.generate(&module).unwrap();
        assert!(assembly.contains("movz x0"));
        assert!(assembly.contains("movk x0"));
    }

    #[test]
    fn escapes_string_literals_as_bytes() {
        let mut module = IrModule::new("string_test");
        module
            .string_literals
            .push(("text".to_owned(), "quote: \" slash: \\\n".to_owned()));

        let assembly = Aarch64Backend.generate(&module).unwrap();
        assert!(assembly.contains(".section .rodata"));
        assert!(assembly.contains(".byte"));
        assert!(!assembly.contains(".asciz"));
    }
}
