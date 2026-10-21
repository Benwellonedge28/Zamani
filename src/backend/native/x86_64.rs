
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Native x86-64 assembly emitter for Zamani.
//!
//! Target contract:
//! - Architecture: x86-64.
//! - ABI: System V AMD64.
//! - Syntax: GNU assembler, AT&T operand order.
//! - Input: an `IrModule` produced by Zamani's IR pipeline.
//! - Output: assembly text only; this module does not write files or invoke tools.
//!
//! Correctness policy:
//! - Unsupported IR is an error, never a comment pretending to implement it.
//! - Symbols are validated before being written to assembly.
//! - Values receive stable stack slots before instruction emission.
//! - Floating-point, pointer, quantum, HDL, and unsupported memory operations
//!   are rejected rather than silently converted to integers.
//!
//! Integration:
//! This file expects the existing `crate::backend::{Backend, CodeGenError}`
//! contract and `crate::ir_gen` types. See the integration instructions below.

use std::collections::{BTreeMap, BTreeSet};

use crate::backend::{Backend, CodeGenError};
use crate::ir_gen::{CmpOp, IrFunction, IrInstruction, IrModule, IrType, IrValue};

/// Native x86-64 backend for the System V AMD64 ABI.
#[derive(Debug, Clone, Copy, Default)]
pub struct X86_64Backend;

const TARGET: &str = "x86_64-unknown-linux-gnu";
const ARGUMENT_REGISTERS: [&str; 6] =
    ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

impl Backend for X86_64Backend {
    fn target_name(&self) -> &str {
        TARGET
    }

    fn file_extension(&self) -> &str {
        ".s"
    }

    fn generate(&self, module: &IrModule) -> Result<String, CodeGenError> {
        emit_module(module)
    }
}

fn error(message: impl Into<String>) -> CodeGenError {
    CodeGenError::new(message, TARGET)
}

/// Emit a module without filesystem, process, or console side effects.
pub fn emit_module(module: &IrModule) -> Result<String, CodeGenError> {
    validate_module(module)?;

    let mut out = String::new();

    out.push_str("# Zamani x86-64 assembly\n");
    out.push_str("# ABI: System V AMD64; syntax: GNU AT&T\n");
    out.push_str(".text\n");

    for function in &module.functions {
        if function.is_external {
            continue;
        }

        emit_function(function, &mut out)?;
    }

    emit_globals(module, &mut out)?;

    out.push_str("\n.section .note.GNU-stack,\"\",@progbits\n");
    Ok(out)
}

fn validate_module(module: &IrModule) -> Result<(), CodeGenError> {
    validate_symbol(&module.name, "module")?;

    let mut symbols = BTreeSet::new();

    for function in &module.functions {
        validate_symbol(&function.name, "function")?;

        if !symbols.insert(function.name.as_str()) {
            return Err(error(format!(
                "duplicate function symbol `{}`",
                function.name
            )));
        }

        if function.return_type != IrType::Void {
            require_integer_type(&function.return_type, "function return type")?;
        }

        // This initial implementation supports integer-like scalar arguments.
        for (name, ty) in &function.params {
            validate_symbol(name, "parameter")?;
            require_integer_type(ty, "function parameter type")?;
        }
    }

    for global in &module.globals {
        validate_symbol(&global.name, "global")?;

        if !symbols.insert(global.name.as_str()) {
            return Err(error(format!(
                "duplicate global/function symbol `{}`",
                global.name
            )));
        }

        require_integer_type(&global.ty, "global type")?;
        validate_integer_constant(&global.value)?;
    }

    for (name, _) in &module.string_literals {
        validate_symbol(name, "string literal")?;

        if !symbols.insert(name.as_str()) {
            return Err(error(format!(
                "duplicate symbol `{name}`"
            )));
        }
    }

    if !module.type_defs.is_empty() {
        return Err(error(
            "native emitter does not yet support aggregate type definitions",
        ));
    }

    // External functions can be called, but every call signature must remain
    // compatible with the scalar ABI subset supported by this emitter.
    for function in &module.functions {
        for instruction in &function.body {
            match instruction {
                IrInstruction::Call(_, name, args) => {
                    validate_symbol(name, "call target")?;

                    if args.len() > ARGUMENT_REGISTERS.len() {
                        return Err(error(format!(
                            "call to `{name}` has {} arguments; stack-passed \
                             outgoing arguments are not implemented yet",
                            args.len()
                        )));
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn validate_symbol(name: &str, kind: &str) -> Result<(), CodeGenError> {
    let mut chars = name.chars();

    let valid_start = chars
        .next()
        .map(|c| c.is_ascii_alphabetic() || matches!(c, '_' | '.' | '$'))
        .unwrap_or(false);

    let valid_rest = chars.all(|c| {
        c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '$')
    });

    if !valid_start || !valid_rest {
        return Err(error(format!(
            "invalid {kind} symbol `{name}` for GNU assembler output"
        )));
    }

    Ok(())
}

fn require_integer_type(ty: &IrType, context: &str) -> Result<(), CodeGenError> {
    match ty {
        IrType::Bool
        | IrType::I8
        | IrType::I16
        | IrType::I32
        | IrType::I64
        | IrType::I128
        | IrType::U8
        | IrType::U16
        | IrType::U32
        | IrType::U64
        | IrType::U128 => Ok(()),
        _ => Err(error(format!(
            "unsupported {context}: {ty:?}; this emitter currently \
             supports integer-like scalar values only"
        ))),
    }
}

fn validate_integer_constant(value: &IrValue) -> Result<(), CodeGenError> {
    match value {
        IrValue::ConstInt(_, ty) => require_integer_type(ty, "integer constant type"),
        IrValue::ConstBool(_) => Ok(()),
        _ => Err(error(format!(
            "unsupported global initializer: {value:?}"
        ))),
    }
}

#[derive(Debug)]
struct Frame {
    /// SSA-like IR register name -> negative `%rbp` offset.
    slots: BTreeMap<String, i64>,
    /// Total frame size, aligned to the ABI stack boundary.
    size: u64,
}

impl Frame {
    fn build(function: &IrFunction) -> Result<Self, CodeGenError> {
        let mut names = BTreeSet::new();

        for (name, ty) in &function.params {
            validate_symbol(name, "parameter")?;
            require_integer_type(ty, "parameter type")?;

            if !names.insert(name.clone()) {
                return Err(error(format!(
                    "duplicate parameter `{name}` in function `{}`",
                    function.name
                )));
            }
        }

        for instruction in &function.body {
            if let Some((register, ty)) = result_register(instruction) {
                validate_symbol(&register.0, "IR register")?;
                require_integer_type(ty, "IR result type")?;

                if !names.insert(register.0.clone()) {
                    return Err(error(format!(
                        "register `{}` is defined more than once in `{}`",
                        register.0, function.name
                    )));
                }
            }
        }

        let mut slots = BTreeMap::new();
        let mut next_offset: u64 = 8;

        for name in names {
            let offset = i64::try_from(next_offset)
                .map_err(|_| error("stack-frame offset exceeds supported addressing"))?;

            slots.insert(name, -offset);

            next_offset = next_offset
                .checked_add(8)
                .ok_or_else(|| error("stack-frame size overflow"))?;
        }

        // The prologue pushes %rbp, leaving %rsp 16-byte aligned on entry to
        // the body. Round the local allocation up to preserve call alignment.
        let raw_size = next_offset.saturating_sub(8);
        let size = raw_size
            .checked_add(15)
            .ok_or_else(|| error("stack-frame size overflow"))?
            / 16
            * 16;

        Ok(Self { slots, size })
    }

    fn slot(&self, name: &str) -> Result<String, CodeGenError> {
        self.slots
            .get(name)
            .map(|offset| format!("{offset}(%rbp)"))
            .ok_or_else(|| error(format!("use of undefined IR register `{name}`")))
    }
}

fn result_register(
    instruction: &IrInstruction,
) -> Option<(&crate::ir_gen::IrRegister, &IrType)> {
    use IrInstruction::*;

    match instruction {
        Assign(r, _) | Add(r, _, _) | Sub(r, _, _) | Mul(r, _, _)
        | Div(r, _, _) | Rem(r, _, _) | Neg(r, _) | And(r, _, _)
        | Or(r, _, _) | Xor(r, _, _) | Shl(r, _, _) | Shr(r, _, _)
        | Not(r, _) | Cmp(r, _, _, _) => Some((r, &r.1)),
        Call(Some(r), _, _) => Some((r, &r.1)),
        _ => None,
    }
}

fn emit_function(function: &IrFunction, out: &mut String) -> Result<(), CodeGenError> {
    let frame = Frame::build(function)?;

    // This subset uses only integer/pointer-free scalar slots. Incoming stack
    // arguments are read at positive offsets from %rbp.
    out.push_str(&format!(
        "\n.globl {}\n.type {}, @function\n{}:\n",
        function.name, function.name, function.name
    ));
    out.push_str("    pushq %rbp\n");
    out.push_str("    movq %rsp, %rbp\n");

    if frame.size != 0 {
        out.push_str(&format!("    subq ${}, %rsp\n", frame.size));
    }

    for (index, (name, _)) in function.params.iter().enumerate() {
        let dst = frame.slot(name)?;

        if let Some(register) = ARGUMENT_REGISTERS.get(index) {
            out.push_str(&format!("    movq {register}, {dst}\n"));
        } else {
            // At function entry, the first stack argument is at 16(%rbp).
            let stack_index = index - ARGUMENT_REGISTERS.len();
            let incoming_offset = 16_u64
                .checked_add(
                    u64::try_from(stack_index)
                        .map_err(|_| error("argument index conversion failed"))?
                        .checked_mul(8)
                        .ok_or_else(|| error("incoming argument offset overflow"))?,
                )
                .ok_or_else(|| error("incoming argument offset overflow"))?;

            out.push_str(&format!(
                "    movq {}(%rbp), %rax\n    movq %rax, {}\n",
                incoming_offset, dst
            ));
        }
    }

    for instruction in &function.body {
        emit_instruction(instruction, &frame, out)?;
    }

    // Reaching the end of a non-void function without a return is invalid.
    // Do not synthesize a return value that would change program semantics.
    if !matches!(
        function.body.last(),
        Some(IrInstruction::Ret(_)) | Some(IrInstruction::Unreachable)
    ) {
        return Err(error(format!(
            "function `{}` has no explicit terminal instruction; \
             control-flow verification is required before native emission",
            function.name
        )));
    }

    out.push_str(&format!(
        ".size {}, .-{}\n",
        function.name, function.name
    ));

    Ok(())
}

fn emit_instruction(
    instruction: &IrInstruction,
    frame: &Frame,
    out: &mut String,
) -> Result<(), CodeGenError> {
    use IrInstruction::*;

    match instruction {
        Comment(comment) => {
            // Emit one comment line per input line to prevent line injection.
            for line in comment.lines() {
                out.push_str("    # ");
                out.push_str(&line.replace('\r', " "));
                out.push('\n');
            }
        }

        Label(label) => {
            validate_symbol(label, "basic-block label")?;
            out.push_str(&format!(".L_zamani_{label}:\n"));
        }

        Assign(result, value) => {
            load_integer(value, frame, "%rax", out)?;
            out.push_str(&format!("    movq %rax, {}\n", frame.slot(&result.0)?));
        }

        Add(result, left, right)
        | Sub(result, left, right)
        | Mul(result, left, right)
        | And(result, left, right)
        | Or(result, left, right)
        | Xor(result, left, right)
        | Shl(result, left, right)
        | Shr(result, left, right) => {
            load_integer(left, frame, "%rax", out)?;
            load_integer(right, frame, "%r10", out)?;

            let opcode = match instruction {
                Add(..) => "addq",
                Sub(..) => "subq",
                Mul(..) => "imulq",
                And(..) => "andq",
                Or(..) => "orq",
                Xor(..) => "xorq",
                Shl(..) => "salq",
                Shr(..) => "sarq",
                _ => unreachable!(),
            };

            out.push_str(&format!("    {opcode} %r10, %rax\n"));
            out.push_str(&format!("    movq %rax, {}\n", frame.slot(&result.0)?));
        }

        Div(result, left, right) | Rem(result, left, right) => {
            load_integer(left, frame, "%rax", out)?;
            load_integer(right, frame, "%r10", out)?;

            out.push_str("    cqto\n");
            out.push_str("    idivq %r10\n");

            let source = if matches!(instruction, Div(..)) {
                "%rax"
            } else {
                "%rdx"
            };

            out.push_str(&format!(
                "    movq {source}, {}\n",
                frame.slot(&result.0)?
            ));
        }

        Neg(result, value) => {
            load_integer(value, frame, "%rax", out)?;
            out.push_str("    negq %rax\n");
            out.push_str(&format!("    movq %rax, {}\n", frame.slot(&result.0)?));
        }

        Not(result, value) => {
            load_integer(value, frame, "%rax", out)?;
            out.push_str("    notq %rax\n");
            out.push_str(&format!("    movq %rax, {}\n", frame.slot(&result.0)?));
        }

        Cmp(result, op, left, right) => {
            if matches!(
                op,
                CmpOp::FEq | CmpOp::FNe | CmpOp::FLt | CmpOp::FLe
                    | CmpOp::FGt | CmpOp::FGe
            ) {
                return Err(error("floating-point comparisons are not supported by this emitter"));
            }

            load_integer(left, frame, "%rax", out)?;
            load_integer(right, frame, "%r10", out)?;
            out.push_str("    cmpq %r10, %rax\n");

            let setcc = match op {
                CmpOp::Eq => "sete",
                CmpOp::Ne => "setne",
                CmpOp::Lt => "setl",
                CmpOp::Le => "setle",
                CmpOp::Gt => "setg",
                CmpOp::Ge => "setge",
                _ => return Err(error("unsupported comparison predicate")),
            };

            out.push_str(&format!("    {setcc} %al\n"));
            out.push_str("    movzbq %al, %rax\n");
            out.push_str(&format!("    movq %rax, {}\n", frame.slot(&result.0)?));
        }

        Jump(label) => {
            validate_symbol(label, "jump label")?;
            out.push_str(&format!("    jmp .L_zamani_{label}\n"));
        }

        CondJump(condition, if_true, if_false) => {
            validate_symbol(if_true, "true branch label")?;
            validate_symbol(if_false, "false branch label")?;
            load_integer(condition, frame, "%rax", out)?;
            out.push_str("    testq %rax, %rax\n");
            out.push_str(&format!("    jne .L_zamani_{if_true}\n"));
            out.push_str(&format!("    jmp .L_zamani_{if_false}\n"));
        }

        Ret(value) => {
            if let Some(value) = value {
                load_integer(value, frame, "%rax", out)?;
            } else {
                out.push_str("    xorl %eax, %eax\n");
            }

            out.push_str("    movq %rbp, %rsp\n");
            out.push_str("    popq %rbp\n");
            out.push_str("    ret\n");
        }

        Unreachable => {
            out.push_str("    ud2\n");
        }

        Call(destination, name, arguments) => {
            validate_symbol(name, "call target")?;

            if arguments.len() > ARGUMENT_REGISTERS.len() {
                return Err(error(format!(
                    "call to `{name}` exceeds the implemented register-argument ABI subset"
                )));
            }

            // Evaluate arguments to stable stack locations first. Loading them
            // into argument registers afterward avoids register clobbering
            // when multiple arguments are sourced from registers.
            for (index, argument) in arguments.iter().enumerate() {
                let scratch = format!("%r{}", 11 + index);
                load_integer(argument, frame, &scratch, out)?;
            }

            for (index, _) in arguments.iter().enumerate() {
                let scratch = format!("%r{}", 11 + index);
                out.push_str(&format!(
                    "    movq {scratch}, {}\n",
                    ARGUMENT_REGISTERS[index]
                ));
            }

            out.push_str(&format!("    call {name}\n"));

            if let Some(destination) = destination {
                out.push_str(&format!(
                    "    movq %rax, {}\n",
                    frame.slot(&destination.0)?
                ));
            }
        }

        // Explicitly reject every operation whose lowering is not yet
        // implemented and verified. In particular, do not turn memory,
        // conversion, indirect-call, quantum, or hardware instructions into
        // comments or no-ops.
        unsupported => {
            return Err(error(format!(
                "unsupported x86-64 IR instruction: {unsupported:?}"
            )));
        }
    }

    Ok(())
}

fn load_integer(
    value: &IrValue,
    frame: &Frame,
    register: &str,
    out: &mut String,
) -> Result<(), CodeGenError> {
    match value {
        IrValue::Reg(ir_register) => {
            require_integer_type(&ir_register.1, "register operand type")?;
            out.push_str(&format!(
                "    movq {}, {register}\n",
                frame.slot(&ir_register.0)?
            ));
        }

        IrValue::ConstInt(value, ty) => {
            require_integer_type(ty, "integer operand type")?;
            out.push_str(&format!("    movabsq ${value}, {register}\n"));
        }

        IrValue::ConstBool(value) => {
            out.push_str(&format!(
                "    movq ${}, {register}\n",
                u8::from(*value)
            ));
        }

        unsupported => {
            return Err(error(format!(
                "unsupported x86-64 scalar operand: {unsupported:?}"
            )));
        }
    }

    Ok(())
}

fn emit_globals(module: &IrModule, out: &mut String) -> Result<(), CodeGenError> {
    if module.globals.is_empty() && module.string_literals.is_empty() {
        return Ok(());
    }

    out.push_str("\n.data\n");

    for global in &module.globals {
        validate_symbol(&global.name, "global")?;

        out.push_str(&format!(
            ".globl {}\n.type {}, @object\n{}:\n",
            global.name, global.name, global.name
        ));

        match &global.value {
            IrValue::ConstInt(value, ty) => {
                require_integer_type(ty, "global initializer type")?;
                out.push_str(&format!("    .quad {value}\n"));
            }

            IrValue::ConstBool(value) => {
                out.push_str(&format!("    .byte {}\n", u8::from(*value)));
            }

            unsupported => {
                return Err(error(format!(
                    "unsupported global initializer: {unsupported:?}"
                )));
            }
        }

        out.push_str(&format!(
            ".size {}, .-{}\n",
            global.name, global.name
        ));
    }

    for (name, contents) in &module.string_literals {
        validate_symbol(name, "string literal")?;

        out.push_str(&format!(
            "\n.globl {name}\n.type {name}, @object\n{name}:\n    .byte "
        ));

        // Emit the UTF-8 bytes as numeric directives. This avoids assembler
        // quoting, escaping, and source-line injection problems.
        for (index, byte) in contents.as_bytes().iter().enumerate() {
            if index != 0 {
                out.push_str(", ");
            }
            out.push_str(&byte.to_string());
        }

        if !contents.is_empty() {
            out.push_str(", ");
        }

        out.push_str("0\n");
        out.push_str(&format!(".size {name}, .-{name}\n"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrFunction, IrModule, IrRegister};

    #[test]
    fn emits_integer_addition_and_return() {
        let mut module = IrModule::new("arithmetic");
        let mut function = IrFunction::new(
            "add_values",
            vec![
                ("left".to_string(), IrType::I64),
                ("right".to_string(), IrType::I64),
            ],
            IrType::I64,
        );

        let result = IrRegister::new("sum", IrType::I64);

        function.push(IrInstruction::Add(
            result.clone(),
            IrValue::Reg(IrRegister::new("left", IrType::I64)),
            IrValue::Reg(IrRegister::new("right", IrType::I64)),
        ));
        function.push(IrInstruction::Ret(Some(IrValue::Reg(result))));
        module.add_function(function);

        let assembly = emit_module(&module).expect("valid supported module");

        assert!(assembly.contains("addq %r10, %rax"));
        assert!(assembly.contains("ret"));
        assert!(assembly.contains(".note.GNU-stack"));
    }

    #[test]
    fn rejects_unsupported_instructions_instead_of_ignoring_them() {
        let mut module = IrModule::new("unsupported");
        let mut function =
            IrFunction::new("work", vec![], IrType::Void);

        function.push(IrInstruction::QuantumGate(
            IrRegister::new("result", IrType::Quantum),
            "h".to_string(),
            vec![],
        ));
        function.push(IrInstruction::Ret(None));
        module.add_function(function);

        assert!(emit_module(&module).is_err());
    }

    #[test]
    fn rejects_invalid_symbols() {
        let mut module = IrModule::new("invalid");
        module.add_function(IrFunction::new(
            "name\n.globl injected",
            vec![],
            IrType::Void,
        ));

        assert!(emit_module(&module).is_err());
    }

    #[test]
    fn does_not_claim_to_support_floating_point() {
        let mut module = IrModule::new("float");
        module.add_function(IrFunction::new(
            "compute",
            vec![("x".to_string(), IrType::F64)],
            IrType::F64,
        ));

        assert!(emit_module(&module).is_err());
    }
}
