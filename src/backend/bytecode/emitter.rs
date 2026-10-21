#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Strict textual bytecode emission for Zamani's legacy IR.
//!
//! Path: `src/backend/bytecode/emitter.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Emit a deterministic, versioned textual representation of the subset of
//! legacy `IrModule` instructions implemented by this emitter.
//!
//! # Important format boundary
//!
//! This format is named `ZAMANI-MTS-TEXT` to distinguish it from a binary
//! bytecode format or a verified executable module. The repository does not
//! currently define a complete executable MTS instruction set or a matching
//! bytecode virtual machine.
//!
//! Therefore, this emitter:
//!
//! - emits an explicit format identifier and version;
//! - preserves module, function, parameter, return-type, and supported
//!   instruction information;
//! - escapes textual fields so user-provided strings cannot inject records;
//! - rejects unsupported instructions instead of silently dropping them;
//! - does not execute programs, perform I/O, or launch external tools;
//! - does not claim that generated text is executable bytecode.
//!
//! # Ownership
//!
//! `crate::ir_gen` owns the legacy IR types.
//! `crate::ir_verify` owns structural verification of that IR.
//! `crate::backend::pipeline` owns generic compilation orchestration.
//! The future bytecode runtime must own instruction decoding, verification,
//! execution, memory safety, and runtime semantics.
//!
//! This module must not introduce a competing IR, semantic analyzer, target
//! registry, or runtime.
//!
//! # Supported instruction subset
//!
//! Currently supported:
//!
//! - `Assign`
//! - `Add`, `Sub`, `Mul`
//! - `Label`, `Jump`, `CondJump`
//! - `Ret`
//! - `Comment`
//!
//! All other instructions are rejected explicitly.
//!
//! # Scalability
//!
//! This module defines no universal hardware-capacity limits. Output remains
//! subject to available memory, address-space limits, and caller-configured
//! compilation budgets. Capacity exhaustion must not produce partial output
//! reported as a successful artifact.

use std::error::Error;
use std::fmt;

use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType, IrValue};

/// The format identifier emitted by this module.
///
/// This is a textual interchange format identifier, not a claim of
/// compatibility with an existing executable bytecode runtime.
pub const BYTECODE_FORMAT_ID: &str = "ZAMANI-MTS-TEXT";

/// Version of the textual interchange format.
pub const BYTECODE_FORMAT_VERSION: u16 = 1;

/// Error returned when emission cannot preserve the supported IR semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeEmissionError {
    /// Module containing the rejected instruction, if applicable.
    pub module: String,

    /// Function containing the rejected instruction, if applicable.
    pub function: Option<String>,

    /// Zero-based instruction position, if applicable.
    pub instruction_index: Option<usize>,

    /// Stable machine-readable category.
    pub code: BytecodeEmissionErrorCode,

    /// Human-readable explanation.
    pub message: String,
}

/// Stable categories for textual-emission failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytecodeEmissionErrorCode {
    /// The input contains an unsupported IR instruction.
    UnsupportedInstruction,

    /// A required module, function, or register name is invalid.
    InvalidIdentifier,

    /// A type cannot be represented by this format version.
    UnsupportedType,

    /// A required value cannot be represented by this format version.
    UnsupportedValue,
}

impl fmt::Display for BytecodeEmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} in module `{}`",
            self.message, self.module
        )?;

        if let Some(function) = &self.function {
            write!(f, ", function `{function}`")?;
        }

        if let Some(index) = self.instruction_index {
            write!(f, ", instruction {index}")?;
        }

        Ok(())
    }
}

impl Error for BytecodeEmissionError {}

impl BytecodeEmissionError {
    fn new(
        module: &str,
        function: Option<&str>,
        instruction_index: Option<usize>,
        code: BytecodeEmissionErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            module: module.to_owned(),
            function: function.map(str::to_owned),
            instruction_index,
            code,
            message: message.into(),
        }
    }
}

/// Result of successfully emitting the supported textual format.
///
/// `text` is a textual interchange artifact. It is not a binary executable
/// and must not be passed to a runtime that expects a different format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeTextArtifact {
    /// Format identifier.
    pub format: &'static str,

    /// Format version.
    pub version: u16,

    /// Complete emitted text.
    pub text: String,

    /// Number of UTF-8 bytes in `text`.
    pub size_bytes: usize,

    /// Number of emitted instructions, excluding declarations and metadata.
    pub instruction_count: usize,
}

/// Stateless emitter for the supported legacy-IR subset.
#[derive(Debug, Default, Clone, Copy)]
pub struct BytecodeEmitter;

impl BytecodeEmitter {
    /// Construct an emitter.
    pub const fn new() -> Self {
        Self
    }

    /// Emit the supported portion of a legacy IR module.
    ///
    /// Emission is all-or-error: an unsupported instruction returns an error,
    /// and no partial artifact is returned.
    ///
    /// This method does not replace authoritative input IR verification.
    /// The caller must invoke `crate::ir_verify::verify_module` at the
    /// appropriate compilation boundary.
    pub fn emit(
        &self,
        module: &IrModule,
    ) -> Result<BytecodeTextArtifact, BytecodeEmissionError> {
        let mut output = String::new();
        let mut instruction_count = 0usize;

        output.push_str(BYTECODE_FORMAT_ID);
        output.push(' ');
        output.push_str(&BYTECODE_FORMAT_VERSION.to_string());
        output.push('\n');

        output.push_str("module ");
        output.push_str(&quoted(&module.name));
        output.push('\n');

        // Keep global data explicit. This format version does not yet define
        // global storage or initialization semantics, so do not omit it.
        if !module.globals.is_empty() {
            return Err(BytecodeEmissionError::new(
                &module.name,
                None,
                None,
                BytecodeEmissionErrorCode::UnsupportedValue,
                "global variables are not supported by textual format version 1",
            ));
        }

        if !module.string_literals.is_empty() {
            return Err(BytecodeEmissionError::new(
                &module.name,
                None,
                None,
                BytecodeEmissionErrorCode::UnsupportedValue,
                "string-literal storage is not defined by textual format version 1",
            ));
        }

        if !module.type_defs.is_empty() {
            return Err(BytecodeEmissionError::new(
                &module.name,
                None,
                None,
                BytecodeEmissionErrorCode::UnsupportedType,
                "named structure definitions are not supported by textual format version 1",
            ));
        }

        for function in &module.functions {
            self.emit_function(
                module,
                function,
                &mut output,
                &mut instruction_count,
            )?;
        }

        output.push_str("end_module\n");

        let size_bytes = output.len();

        Ok(BytecodeTextArtifact {
            format: BYTECODE_FORMAT_ID,
            version: BYTECODE_FORMAT_VERSION,
            text: output,
            size_bytes,
            instruction_count,
        })
    }

    fn emit_function(
        &self,
        module: &IrModule,
        function: &IrFunction,
        output: &mut String,
        instruction_count: &mut usize,
    ) -> Result<(), BytecodeEmissionError> {
        validate_name(&function.name).map_err(|message| {
            BytecodeEmissionError::new(
                &module.name,
                Some(&function.name),
                None,
                BytecodeEmissionErrorCode::InvalidIdentifier,
                message,
            )
        })?;

        if function.is_external {
            // External calls need an explicit import/linkage contract.
            // Do not emit a declaration that the runtime cannot resolve.
            return Err(BytecodeEmissionError::new(
                &module.name,
                Some(&function.name),
                None,
                BytecodeEmissionErrorCode::UnsupportedInstruction,
                "external functions require an import/linkage contract not defined by format version 1",
            ));
        }

        output.push_str("function ");
        output.push_str(&quoted(&function.name));
        output.push_str(" returns ");
        output.push_str(&quoted(&encode_type(&function.return_type)?));
        output.push_str(" params ");

        output.push('[');
        for (index, (name, ty)) in function.params.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }

            validate_name(name).map_err(|message| {
                BytecodeEmissionError::new(
                    &module.name,
                    Some(&function.name),
                    None,
                    BytecodeEmissionErrorCode::InvalidIdentifier,
                    message,
                )
            })?;

            output.push('(');
            output.push_str(&quoted(name));
            output.push_str(", ");
            output.push_str(&quoted(&encode_type(ty)?));
            output.push(')');
        }
        output.push_str("]\n");

        for (index, instruction) in function.body.iter().enumerate() {
            let encoded = encode_instruction(
                &module.name,
                &function.name,
                index,
                instruction,
            )?;

            if encoded.is_empty() {
                // Comments are retained as records and therefore never
                // disappear silently.
                continue;
            }

            output.push_str("  ");
            output.push_str(&encoded);
            output.push('\n');

            *instruction_count = instruction_count.checked_add(1).ok_or_else(|| {
                BytecodeEmissionError::new(
                    &module.name,
                    Some(&function.name),
                    Some(index),
                    BytecodeEmissionErrorCode::UnsupportedValue,
                    "instruction count overflow",
                )
            })?;
        }

        output.push_str("end_function\n");
        Ok(())
    }
}

fn encode_instruction(
    module: &str,
    function: &str,
    index: usize,
    instruction: &IrInstruction,
) -> Result<String, BytecodeEmissionError> {
    let unsupported = || {
        BytecodeEmissionError::new(
            module,
            Some(function),
            Some(index),
            BytecodeEmissionErrorCode::UnsupportedInstruction,
            format!(
                "instruction variant `{}` is not supported by textual format version 1",
                instruction_name(instruction),
            ),
        )
    };

    let result = match instruction {
        IrInstruction::Assign(destination, value) => {
            validate_register(module, function, index, &destination.0)?;
            format!(
                "assign {} {}",
                quoted(&destination.0),
                encode_value(module, function, index, value)?,
            )
        }

        IrInstruction::Add(destination, left, right) => {
            encode_binary(
                module,
                function,
                index,
                "add",
                destination,
                left,
                right,
            )?
        }

        IrInstruction::Sub(destination, left, right) => {
            encode_binary(
                module,
                function,
                index,
                "sub",
                destination,
                left,
                right,
            )?
        }

        IrInstruction::Mul(destination, left, right) => {
            encode_binary(
                module,
                function,
                index,
                "mul",
                destination,
                left,
                right,
            )?
        }

        IrInstruction::Label(label) => {
            validate_name(label).map_err(|message| {
                BytecodeEmissionError::new(
                    module,
                    Some(function),
                    Some(index),
                    BytecodeEmissionErrorCode::InvalidIdentifier,
                    message,
                )
            })?;
            format!("label {}", quoted(label))
        }

        IrInstruction::Jump(label) => {
            validate_name(label).map_err(|message| {
                BytecodeEmissionError::new(
                    module,
                    Some(function),
                    Some(index),
                    BytecodeEmissionErrorCode::InvalidIdentifier,
                    message,
                )
            })?;
            format!("jump {}", quoted(label))
        }

        IrInstruction::CondJump(condition, if_true, if_false) => {
            validate_name(if_true).map_err(|message| {
                BytecodeEmissionError::new(
                    module,
                    Some(function),
                    Some(index),
                    BytecodeEmissionErrorCode::InvalidIdentifier,
                    message,
                )
            })?;
            validate_name(if_false).map_err(|message| {
                BytecodeEmissionError::new(
                    module,
                    Some(function),
                    Some(index),
                    BytecodeEmissionErrorCode::InvalidIdentifier,
                    message,
                )
            })?;

            format!(
                "branch {} {} {}",
                encode_value(module, function, index, condition)?,
                quoted(if_true),
                quoted(if_false),
            )
        }

        IrInstruction::Ret(None) => "return_void".to_owned(),

        IrInstruction::Ret(Some(value)) => {
            format!(
                "return {}",
                encode_value(module, function, index, value)?,
            )
        }

        IrInstruction::Comment(comment) => {
            format!("comment {}", quoted(comment))
        }

        // Never turn an unsupported instruction into a comment, a no-op,
        // or an approximate operation. That would change program meaning.
        _ => return Err(unsupported()),
    };

    Ok(result)
}

fn encode_binary(
    module: &str,
    function: &str,
    index: usize,
    opcode: &str,
    destination: &crate::ir_gen::IrRegister,
    left: &IrValue,
    right: &IrValue,
) -> Result<String, BytecodeEmissionError> {
    validate_register(module, function, index, &destination.0)?;

    Ok(format!(
        "{opcode} {} {} {}",
        quoted(&destination.0),
        encode_value(module, function, index, left)?,
        encode_value(module, function, index, right)?,
    ))
}

fn encode_value(
    module: &str,
    function: &str,
    index: usize,
    value: &IrValue,
) -> Result<String, BytecodeEmissionError> {
    match value {
        IrValue::Reg(register) => {
            validate_register(module, function, index, &register.0)?;
            Ok(format!("reg {}", quoted(&register.0)))
        }

        IrValue::ConstInt(number, ty) => Ok(format!(
            "integer {} {}",
            quoted(&number.to_string()),
            quoted(&encode_type(ty)?),
        )),

        IrValue::ConstFloat(number, ty) if number.is_finite() => Ok(format!(
            "float {} {}",
            quoted(&number.to_string()),
            quoted(&encode_type(ty)?),
        )),

        IrValue::ConstFloat(_, _) => Err(BytecodeEmissionError::new(
            module,
            Some(function),
            Some(index),
            BytecodeEmissionErrorCode::UnsupportedValue,
            "non-finite floating-point constants are not supported by textual format version 1",
        )),

        IrValue::ConstBool(value) => Ok(format!("boolean {value}")),

        IrValue::ConstStr(value) => {
            Ok(format!("string {}", quoted(value)))
        }

        IrValue::ConstNull => Ok("null".to_owned()),

        IrValue::Void => Ok("void".to_owned()),

        // Global references require global-storage and relocation semantics.
        // Reject until those semantics are specified.
        IrValue::GlobalPtr(_, _) => Err(BytecodeEmissionError::new(
            module,
            Some(function),
            Some(index),
            BytecodeEmissionErrorCode::UnsupportedValue,
            "global pointers require a storage and relocation contract",
        )),
    }
}

fn encode_type(ty: &IrType) -> Result<String, BytecodeEmissionError> {
    match ty {
        IrType::Void => Ok("void".to_owned()),
        IrType::Bool => Ok("bool".to_owned()),
        IrType::I8 => Ok("i8".to_owned()),
        IrType::I16 => Ok("i16".to_owned()),
        IrType::I32 => Ok("i32".to_owned()),
        IrType::I64 => Ok("i64".to_owned()),
        IrType::I128 => Ok("i128".to_owned()),
        IrType::U8 => Ok("u8".to_owned()),
        IrType::U16 => Ok("u16".to_owned()),
        IrType::U32 => Ok("u32".to_owned()),
        IrType::U64 => Ok("u64".to_owned()),
        IrType::U128 => Ok("u128".to_owned()),
        IrType::F32 => Ok("f32".to_owned()),
        IrType::F64 => Ok("f64".to_owned()),

        // Pointers, arrays, structures, function types, quantum types, and
        // opaque types need layout and runtime semantics that this format
        // version does not define.
        _ => Err(BytecodeEmissionError::new(
            "<type>",
            None,
            None,
            BytecodeEmissionErrorCode::UnsupportedType,
            format!(
                "type `{}` is not supported by textual format version 1",
                ty.ir_name(),
            ),
        )),
    }
}

fn validate_register(
    module: &str,
    function: &str,
    index: usize,
    register: &str,
) -> Result<(), BytecodeEmissionError> {
    validate_name(register).map_err(|message| {
        BytecodeEmissionError::new(
            module,
            Some(function),
            Some(index),
            BytecodeEmissionErrorCode::InvalidIdentifier,
            message,
        )
    })
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("identifier must not be empty".to_owned());
    }

    let mut characters = name.chars();

    let first = characters
        .next()
        .ok_or_else(|| "identifier must not be empty".to_owned())?;

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(format!("invalid identifier `{name}`"));
    }

    if !characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '.' | '$' | '-')
    }) {
        return Err(format!("invalid identifier `{name}`"));
    }

    Ok(())
}

/// Quote a field using deterministic escaping.
///
/// The returned field cannot inject a new instruction or declaration because
/// line breaks, quotes, backslashes, and control characters are escaped.
fn quoted(value: &str) -> String {
    let mut output = String::with_capacity(value.len().saturating_add(2));
    output.push('"');

    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                use fmt::Write;
                let _ = write!(output, "\\u{{{:x}}}", character as u32);
            }
            character => output.push(character),
        }
    }

    output.push('"');
    output
}

fn instruction_name(instruction: &IrInstruction) -> &'static str {
    match instruction {
        IrInstruction::Alloca(..) => "Alloca",
        IrInstruction::Load(..) => "Load",
        IrInstruction::Store(..) => "Store",
        IrInstruction::Add(..) => "Add",
        IrInstruction::Sub(..) => "Sub",
        IrInstruction::Mul(..) => "Mul",
        IrInstruction::Div(..) => "Div",
        IrInstruction::Rem(..) => "Rem",
        IrInstruction::Neg(..) => "Neg",
        IrInstruction::And(..) => "And",
        IrInstruction::Or(..) => "Or",
        IrInstruction::Xor(..) => "Xor",
        IrInstruction::Shl(..) => "Shl",
        IrInstruction::Shr(..) => "Shr",
        IrInstruction::Not(..) => "Not",
        IrInstruction::Cmp(..) => "Cmp",
        IrInstruction::Label(..) => "Label",
        IrInstruction::Jump(..) => "Jump",
        IrInstruction::CondJump(..) => "CondJump",
        IrInstruction::Ret(..) => "Ret",
        IrInstruction::Unreachable => "Unreachable",
        IrInstruction::Call(..) => "Call",
        IrInstruction::CallIndirect(..) => "CallIndirect",
        IrInstruction::GetElementPtr(..) => "GetElementPtr",
        IrInstruction::BitCast(..) => "BitCast",
        IrInstruction::Assign(..) => "Assign",
        IrInstruction::Phi(..) => "Phi",
        IrInstruction::ZExt(..) => "ZExt",
        IrInstruction::SExt(..) => "SExt",
        IrInstruction::Trunc(..) => "Trunc",
        IrInstruction::FpExt(..) => "FpExt",
        IrInstruction::FpTrunc(..) => "FpTrunc",
        IrInstruction::SIToFP(..) => "SIToFP",
        IrInstruction::FPToSI(..) => "FPToSI",
        IrInstruction::QuantumGate(..) => "QuantumGate",
        IrInstruction::NanoOp(..) => "NanoOp",
        IrInstruction::SankofaRecall(..) => "SankofaRecall",
        IrInstruction::SankofaRemember(..) => "SankofaRemember",
        IrInstruction::Comment(..) => "Comment",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrFunction, IrInstruction, IrRegister};

    #[test]
    fn emits_versioned_header_and_deterministic_text() {
        let mut module = IrModule::new("test_module");
        let mut function =
            IrFunction::new("main", vec![], IrType::I32);

        function.push(IrInstruction::Assign(
            IrRegister::new("answer", IrType::I32),
            IrValue::ConstInt(42, IrType::I32),
        ));
        function.push(IrInstruction::Ret(Some(IrValue::Reg(
            IrRegister::new("answer", IrType::I32),
        ))));
        module.add_function(function);

        let emitter = BytecodeEmitter::new();
        let first = emitter.emit(&module).expect("emission should succeed");
        let second = emitter.emit(&module).expect("emission should succeed");

        assert_eq!(first, second);
        assert_eq!(first.format, BYTECODE_FORMAT_ID);
        assert_eq!(first.version, BYTECODE_FORMAT_VERSION);
        assert_eq!(first.size_bytes, first.text.len());
        assert_eq!(first.instruction_count, 2);
        assert!(first.text.contains("ZAMANI-MTS-TEXT 1"));
        assert!(first.text.contains("assign \"answer\""));
        assert!(first.text.contains("return reg \"answer\""));
    }

    #[test]
    fn escapes_newlines_in_comments_and_strings() {
        let mut module = IrModule::new("test_module");
        let mut function =
            IrFunction::new("main", vec![], IrType::Void);

        function.push(IrInstruction::Comment(
            "first line\nsecond line".to_owned(),
        ));
        function.push(IrInstruction::Assign(
            IrRegister::new("message", IrType::I32),
            IrValue::ConstStr("text\nwith newline".to_owned()),
        ));

        module.add_function(function);

        let artifact = BytecodeEmitter::new()
            .emit(&module)
            .expect("supported fields should be escaped");

        assert!(artifact.text.contains(r#"comment "first line\nsecond line""#));
        assert!(artifact.text.contains(r#""text\nwith newline""#));
    }

    #[test]
    fn rejects_unsupported_instructions_instead_of_dropping_them() {
        let mut module = IrModule::new("test_module");
        let mut function =
            IrFunction::new("main", vec![], IrType::Void);

        function.push(IrInstruction::Unreachable);
        module.add_function(function);

        let error = BytecodeEmitter::new()
            .emit(&module)
            .expect_err("unsupported instructions must fail");

        assert_eq!(
            error.code,
            BytecodeEmissionErrorCode::UnsupportedInstruction
        );
        assert_eq!(error.instruction_index, Some(0));
    }

    #[test]
    fn rejects_external_functions_until_imports_are_defined() {
        let mut module = IrModule::new("test_module");
        let mut function =
            IrFunction::new("external_fn", vec![], IrType::Void);

        function.is_external = true;
        module.add_function(function);

        assert_eq!(
            BytecodeEmitter::new().emit(&module).unwrap_err().code,
            BytecodeEmissionErrorCode::UnsupportedInstruction
        );
    }

    #[test]
    fn rejects_record_injection_through_identifiers() {
        let mut module = IrModule::new("test_module");
        let mut function =
            IrFunction::new("main", vec![], IrType::Void);

        function.push(IrInstruction::Label(
            "valid\ninjected".to_owned(),
        ));
        module.add_function(function);

        assert_eq!(
            BytecodeEmitter::new().emit(&module).unwrap_err().code,
            BytecodeEmissionErrorCode::InvalidIdentifier
        );
    }
}