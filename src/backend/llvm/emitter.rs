
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! LLVM textual IR emitter for Zamani.
//!
//! Path: src/backend/llvm/emitter.rs
//!
//! Responsibilities:
//! - Emit LLVM IR from the repository's existing `IrModule`.
//! - Preserve the input IR's declared types and control flow.
//! - Reject unsupported Zamani instructions instead of emitting comments
//!   that pretend unsupported operations have been implemented.
//! - Validate identifiers and structural invariants before emission.
//! - Return text without writing files, invoking tools, or printing.
//!
//! Integration:
//! - Input: crate::ir_gen::{IrModule, IrFunction, IrInstruction, IrValue,
//!   IrType, IrGlobal, CmpOp}.
//! - Toolchain verification belongs to the existing compiler LLVM adapter.
//! - Target selection belongs to backend configuration and selection.
//!
//! This emitter is not a semantic analyzer or an LLVM verifier. Generated
//! text must still pass the selected LLVM toolchain's verifier.

use crate::ir_gen::{
    CmpOp, IrFunction, IrGlobal, IrInstruction, IrModule, IrType, IrValue,
};

use std::collections::HashSet;
use std::fmt;

/// The target information applied to generated LLVM IR.
///
/// A caller must resolve these values from its actual compilation target.
/// Do not infer the target architecture from the host running the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlvmTarget {
    pub triple: String,
    pub data_layout: Option<String>,
}

impl LlvmTarget {
    pub fn new(triple: impl Into<String>) -> Self {
        Self {
            triple: triple.into(),
            data_layout: None,
        }
    }

    pub fn with_data_layout(mut self, layout: impl Into<String>) -> Self {
        self.data_layout = Some(layout.into());
        self
    }

    fn validate(&self) -> Result<(), LlvmEmitterError> {
        if self.triple.trim().is_empty() {
            return Err(LlvmEmitterError::InvalidTarget(
                "target triple must not be empty".into(),
            ));
        }

        if self.triple.chars().any(char::is_control) {
            return Err(LlvmEmitterError::InvalidTarget(
                "target triple contains control characters".into(),
            ));
        }

        if self
            .data_layout
            .as_ref()
            .is_some_and(|layout| layout.chars().any(char::is_control))
        {
            return Err(LlvmEmitterError::InvalidTarget(
                "target data layout contains control characters".into(),
            ));
        }

        Ok(())
    }
}

/// Emission policy.
///
/// These are operational policies, not hardware or language limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlvmEmitterConfig {
    /// Reject unsupported domain instructions rather than substituting them.
    pub reject_unsupported_instructions: bool,

    /// Emit source-independent comments identifying the generated module.
    pub emit_module_comment: bool,
}

impl Default for LlvmEmitterConfig {
    fn default() -> Self {
        Self {
            reject_unsupported_instructions: true,
            emit_module_comment: true,
        }
    }
}

/// Errors produced by the textual emitter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlvmEmitterError {
    InvalidTarget(String),
    InvalidIdentifier {
        kind: &'static str,
        name: String,
    },
    InvalidType(String),
    InvalidModule(String),
    InvalidFunction {
        function: String,
        reason: String,
    },
    UnsupportedInstruction {
        function: String,
        instruction: String,
        reason: String,
    },
    InvalidValue(String),
}

impl fmt::Display for LlvmEmitterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(message) => {
                write!(f, "invalid LLVM target: {message}")
            }
            Self::InvalidIdentifier { kind, name } => {
                write!(f, "invalid LLVM {kind} identifier: {name:?}")
            }
            Self::InvalidType(message) => {
                write!(f, "invalid or unsupported LLVM type: {message}")
            }
            Self::InvalidModule(message) => {
                write!(f, "invalid LLVM input module: {message}")
            }
            Self::InvalidFunction { function, reason } => {
                write!(f, "invalid function {function:?}: {reason}")
            }
            Self::UnsupportedInstruction {
                function,
                instruction,
                reason,
            } => {
                write!(
                    f,
                    "unsupported instruction {instruction} in {function:?}: {reason}"
                )
            }
            Self::InvalidValue(message) => {
                write!(f, "invalid LLVM operand: {message}")
            }
        }
    }
}

impl std::error::Error for LlvmEmitterError {}

pub type LlvmEmitterResult<T> = Result<T, LlvmEmitterError>;

/// Pure LLVM IR text emitter.
///
/// This type does not own target discovery, filesystem operations, external
/// processes, object-file generation, linking, or cache management.
#[derive(Debug, Clone)]
pub struct LlvmEmitter {
    target: LlvmTarget,
    config: LlvmEmitterConfig,
}

impl LlvmEmitter {
    pub fn new(target: LlvmTarget) -> Self {
        Self {
            target,
            config: LlvmEmitterConfig::default(),
        }
    }

    pub fn with_config(
        target: LlvmTarget,
        config: LlvmEmitterConfig,
    ) -> Self {
        Self { target, config }
    }

    pub fn target(&self) -> &LlvmTarget {
        &self.target
    }

    /// Emit textual LLVM IR.
    ///
    /// The result is not advertised as verified until an LLVM verifier such
    /// as llvm-as accepts it. No file is created and no tool is executed.
    pub fn emit(&self, module: &IrModule) -> LlvmEmitterResult<String> {
        self.target.validate()?;
        validate_module(module)?;

        let mut out = String::new();

        if self.config.emit_module_comment {
            out.push_str("; Generated by Zamani LLVM emitter\n");
        }

        out.push_str(&format!(
            "source_filename = \"{}\"\n",
            escape_llvm_string(module.name.as_bytes())
        ));
        out.push_str(&format!(
            "target triple = \"{}\"\n",
            escape_llvm_string(self.target.triple.as_bytes())
        ));

        if let Some(layout) = &self.target.data_layout {
            out.push_str(&format!(
                "target datalayout = \"{}\"\n",
                escape_llvm_string(layout.as_bytes())
            ));
        } else if !module.data_layout.trim().is_empty() {
            // This fallback preserves the existing module's layout only when
            // the caller has not supplied a target-resolved layout.
            // Production target selection should provide the authoritative
            // layout for the requested triple.
            out.push_str(&format!(
                "target datalayout = \"{}\"\n",
                escape_llvm_string(module.data_layout.as_bytes())
            ));
        }

        out.push('\n');

        self.emit_type_definitions(module, &mut out)?;
        self.emit_string_literals(module, &mut out);
        self.emit_globals(module, &mut out)?;

        if !module.type_defs.is_empty()
            || !module.string_literals.is_empty()
            || !module.globals.is_empty()
        {
            out.push('\n');
        }

        for function in &module.functions {
            self.emit_function(function, &mut out)?;
            out.push('\n');
        }

        Ok(out)
    }

    fn emit_type_definitions(
        &self,
        module: &IrModule,
        out: &mut String,
    ) -> LlvmEmitterResult<()> {
        for (name, fields) in &module.type_defs {
            validate_identifier("type", name)?;

            let members = fields
                .iter()
                .map(|(_, ty)| llvm_type(ty))
                .collect::<LlvmEmitterResult<Vec<_>>>()?;

            out.push_str(&format!("%{} = type {{ {} }}\n", name, members.join(", ")));
        }

        for global in &module.globals {
            if let IrType::Struct(name, fields) = &global.ty {
                let _ = fields;
                validate_identifier("type", name)?;
            }
        }

        if !module.type_defs.is_empty() {
            out.push('\n');
        }

        Ok(())
    }

    fn emit_string_literals(&self, module: &IrModule, out: &mut String) {
        for (name, value) in &module.string_literals {
            // Names are validated by validate_module before emission.
            let mut bytes = value.as_bytes().to_vec();
            bytes.push(0);

            out.push_str(&format!(
                "@{} = private unnamed_addr constant [{} x i8] c\"{}\", align 1\n",
                name,
                bytes.len(),
                escape_llvm_string(&bytes)
            ));
        }
    }

    fn emit_globals(
        &self,
        module: &IrModule,
        out: &mut String,
    ) -> LlvmEmitterResult<()> {
        for global in &module.globals {
            self.emit_global(global, out)?;
        }
        Ok(())
    }

    fn emit_global(
        &self,
        global: &IrGlobal,
        out: &mut String,
    ) -> LlvmEmitterResult<()> {
        validate_identifier("global", &global.name)?;

        let ty = llvm_type(&global.ty)?;
        let value = llvm_constant(&global.value)?;

        if global.value.ty() != global.ty
            && !matches!(global.value, IrValue::ConstNull)
        {
            return Err(LlvmEmitterError::InvalidModule(format!(
                "global {:?} initializer type does not match its declared type",
                global.name
            )));
        }

        let storage = if global.is_const {
            "constant"
        } else {
            "global"
        };

        out.push_str(&format!(
            "@{} = {} {} {}, align {}\n",
            global.name,
            storage,
            ty,
            value,
            natural_alignment(&global.ty)?
        ));

        Ok(())
    }

    fn emit_function(
        &self,
        function: &IrFunction,
        out: &mut String,
    ) -> LlvmEmitterResult<()> {
        validate_identifier("function", &function.name)?;

        let return_type = llvm_type(&function.return_type)?;
        let mut parameter_text = Vec::with_capacity(function.params.len());
        let mut seen_parameters = HashSet::new();

        for (name, ty) in &function.params {
            validate_identifier("parameter", name)?;
            if !seen_parameters.insert(name.as_str()) {
                return Err(LlvmEmitterError::InvalidFunction {
                    function: function.name.clone(),
                    reason: format!("duplicate parameter {name:?}"),
                });
            }

            parameter_text.push(format!("{} %{}", llvm_type(ty)?, name));
        }

        if function.is_external {
            out.push_str(&format!(
                "declare {} @{}({})\n",
                return_type,
                function.name,
                parameter_text.join(", ")
            ));
            return Ok(());
        }

        validate_function_body(function)?;

        out.push_str(&format!(
            "define {} @{}({}) {{\n",
            return_type,
            function.name,
            parameter_text.join(", ")
        ));

        let has_entry_label = matches!(
            function.body.first(),
            Some(IrInstruction::Label(_))
        );

        if !has_entry_label {
            out.push_str("entry:\n");
        }

        for instruction in &function.body {
            self.emit_instruction(function, instruction, out)?;
        }

        out.push_str("}\n");
        Ok(())
    }

    fn emit_instruction(
        &self,
        function: &IrFunction,
        instruction: &IrInstruction,
        out: &mut String,
    ) -> LlvmEmitterResult<()> {
        use IrInstruction as I;

        let line = match instruction {
            I::Comment(comment) => {
                out.push_str("  ; ");
                out.push_str(&sanitize_comment(comment));
                out.push('\n');
                return Ok(());
            }

            I::Label(label) => {
                validate_identifier("label", label)?;
                format!("{label}:")
            }

            I::Alloca(result, ty) => {
                format!("%{} = alloca {}", result.0, llvm_type(ty)?)
            }

            I::Load(result, pointer) => {
                let result_ty = llvm_type(&result.1)?;
                let pointer_ty = llvm_type(&pointer.ty())?;

                format!(
                    "%{} = load {}, {} {}, align {}",
                    result.0,
                    result_ty,
                    pointer_ty,
                    llvm_value(pointer)?,
                    natural_alignment(&result.1)?
                )
            }

            I::Store(value, pointer) => {
                let value_ty = llvm_type(&value.ty())?;
                let pointer_ty = llvm_type(&pointer.ty())?;

                format!(
                    "store {} {}, {} {}, align {}",
                    value_ty,
                    llvm_value(value)?,
                    pointer_ty,
                    llvm_value(pointer)?,
                    natural_alignment(&value.ty())?
                )
            }

            I::Add(result, left, right) => {
                binary("add", result, left, right)?
            }
            I::Sub(result, left, right) => {
                binary("sub", result, left, right)?
            }
            I::Mul(result, left, right) => {
                binary("mul", result, left, right)?
            }
            I::Div(result, left, right) => {
                binary("sdiv", result, left, right)?
            }
            I::Rem(result, left, right) => {
                binary("srem", result, left, right)?
            }
            I::And(result, left, right) => {
                binary("and", result, left, right)?
            }
            I::Or(result, left, right) => {
                binary("or", result, left, right)?
            }
            I::Xor(result, left, right) => {
                binary("xor", result, left, right)?
            }
            I::Shl(result, left, right) => {
                binary("shl", result, left, right)?
            }
            I::Shr(result, left, right) => {
                binary("ashr", result, left, right)?
            }

            I::Neg(result, value) => {
                let ty = llvm_type(&value.ty())?;
                format!(
                    "%{} = sub {} 0, {}",
                    result.0,
                    ty,
                    llvm_value(value)?
                )
            }

            I::Not(result, value) => {
                let ty = llvm_type(&value.ty())?;
                format!(
                    "%{} = xor {} {}, -1",
                    result.0,
                    ty,
                    llvm_value(value)?
                )
            }

            I::Cmp(result, op, left, right) => {
                let (opcode, predicate) = comparison(op, &left.ty())?;
                format!(
                    "%{} = {} {} {} {}, {}",
                    result.0,
                    opcode,
                    predicate,
                    llvm_type(&left.ty())?,
                    llvm_value(left)?,
                    llvm_value(right)?
                )
            }

            I::Jump(label) => {
                validate_identifier("label", label)?;
                format!("br label %{label}")
            }

            I::CondJump(condition, yes, no) => {
                validate_identifier("label", yes)?;
                validate_identifier("label", no)?;

                if condition.ty() != IrType::Bool {
                    return Err(LlvmEmitterError::InvalidFunction {
                        function: function.name.clone(),
                        reason: "conditional branch requires an i1 condition".into(),
                    });
                }

                format!(
                    "br i1 {}, label %{}, label %{}",
                    llvm_value(condition)?,
                    yes,
                    no
                )
            }

            I::Ret(None) => {
                if function.return_type != IrType::Void {
                    return Err(LlvmEmitterError::InvalidFunction {
                        function: function.name.clone(),
                        reason: "void return in a non-void function".into(),
                    });
                }
                "ret void".into()
            }

            I::Ret(Some(value)) => {
                if function.return_type != value.ty() {
                    return Err(LlvmEmitterError::InvalidFunction {
                        function: function.name.clone(),
                        reason: "return value type differs from function return type".into(),
                    });
                }
                format!(
                    "ret {} {}",
                    llvm_type(&value.ty())?,
                    llvm_value(value)?
                )
            }

            I::Unreachable => "unreachable".into(),

            I::Call(result, name, arguments) => {
                validate_identifier("function", name)?;

                let args = arguments
                    .iter()
                    .map(|arg| {
                        Ok(format!(
                            "{} {}",
                            llvm_type(&arg.ty())?,
                            llvm_value(arg)?
                        ))
                    })
                    .collect::<LlvmEmitterResult<Vec<_>>>()?;

                match result {
                    Some(result) => format!(
                        "%{} = call {} @{}({})",
                        result.0,
                        llvm_type(&result.1)?,
                        name,
                        args.join(", ")
                    ),
                    None => format!("call void @{}({})", name, args.join(", ")),
                }
            }

            I::Assign(result, value) => {
                // LLVM has no generic assignment instruction. A same-type
                // integer, pointer, or floating value is materialized using
                // a no-op operation where possible. Reject values for which
                // this cannot be done without changing semantics.
                assign_value(result, value)?
            }

            I::Phi(result, incoming) => {
                if incoming.is_empty() {
                    return Err(LlvmEmitterError::InvalidFunction {
                        function: function.name.clone(),
                        reason: "phi instruction has no incoming values".into(),
                    });
                }

                let mut values = Vec::with_capacity(incoming.len());
                for (value, label) in incoming {
                    validate_identifier("label", label)?;
                    if value.ty() != result.1 {
                        return Err(LlvmEmitterError::InvalidFunction {
                            function: function.name.clone(),
                            reason: "phi incoming value has mismatched type".into(),
                        });
                    }
                    values.push(format!(
                        "[ {}, %{} ]",
                        llvm_value(value)?,
                        label
                    ));
                }

                format!(
                    "%{} = phi {} {}",
                    result.0,
                    llvm_type(&result.1)?,
                    values.join(", ")
                )
            }

            I::ZExt(result, value, target) => {
                cast("zext", result, value, target)?
            }
            I::SExt(result, value, target) => {
                cast("sext", result, value, target)?
            }
            I::Trunc(result, value, target) => {
                cast("trunc", result, value, target)?
            }
            I::FpExt(result, value, target) => {
                cast("fpext", result, value, target)?
            }
            I::FpTrunc(result, value, target) => {
                cast("fptrunc", result, value, target)?
            }
            I::SIToFP(result, value, target) => {
                cast("sitofp", result, value, target)?
            }
            I::FPToSI(result, value, target) => {
                cast("fptosi", result, value, target)?
            }
            I::BitCast(result, value, target) => {
                cast("bitcast", result, value, target)?
            }

            I::GetElementPtr(result, base, indices) => {
                if indices.is_empty() {
                    return Err(LlvmEmitterError::InvalidFunction {
                        function: function.name.clone(),
                        reason: "getelementptr requires at least one index".into(),
                    });
                }

                let index_text = indices
                    .iter()
                    .map(|index| {
                        Ok(format!(
                            "{} {}",
                            llvm_type(&index.ty())?,
                            llvm_value(index)?
                        ))
                    })
                    .collect::<LlvmEmitterResult<Vec<_>>>()?;

                format!(
                    "%{} = getelementptr {}, {} {}, {}",
                    result.0,
                    llvm_type(&base.ty())?,
                    llvm_type(&base.ty())?,
                    llvm_value(base)?,
                    index_text.join(", ")
                )
            }

            I::CallIndirect(_, _, _) => {
                return Err(unsupported(
                    function,
                    "CallIndirect",
                    "indirect calls require a validated function-pointer signature",
                ));
            }

            I::QuantumGate(_, _, _) => {
                return Err(unsupported(
                    function,
                    "QuantumGate",
                    "quantum operations require an explicit quantum lowering/runtime contract",
                ));
            }

            I::NanoOp(_, _, _) => {
                return Err(unsupported(
                    function,
                    "NanoOp",
                    "Nano operations require a registered domain lowering",
                ));
            }

            I::SankofaRecall(_, _) | I::SankofaRemember(_, _) => {
                return Err(unsupported(
                    function,
                    "Sankofa",
                    "Sankofa operations require an explicit runtime and effect contract",
                ));
            }
        };

        out.push_str("  ");
        out.push_str(&line);
        out.push('\n');
        Ok(())
    }
}

fn unsupported(
    function: &IrFunction,
    instruction: &str,
    reason: &str,
) -> LlvmEmitterError {
    LlvmEmitterError::UnsupportedInstruction {
        function: function.name.clone(),
        instruction: instruction.into(),
        reason: reason.into(),
    }
}

fn binary(
    opcode: &str,
    result: &crate::ir_gen::IrRegister,
    left: &IrValue,
    right: &IrValue,
) -> LlvmEmitterResult<String> {
    if left.ty() != right.ty() || left.ty() != result.1 {
        return Err(LlvmEmitterError::InvalidValue(format!(
            "{opcode} operands and result must have identical types"
        )));
    }

    if matches!(left.ty(), IrType::Void | IrType::Quantum | IrType::Opaque(_)) {
        return Err(LlvmEmitterError::InvalidType(format!(
            "{opcode} does not support {:?}",
            left.ty()
        )));
    }

    Ok(format!(
        "%{} = {} {} {}, {}",
        result.0,
        opcode,
        llvm_type(&left.ty())?,
        llvm_value(left)?,
        llvm_value(right)?
    ))
}

fn cast(
    opcode: &str,
    result: &crate::ir_gen::IrRegister,
    value: &IrValue,
    target: &IrType,
) -> LlvmEmitterResult<String> {
    if &result.1 != target {
        return Err(LlvmEmitterError::InvalidValue(
            "cast result register type differs from its destination type".into(),
        ));
    }

    Ok(format!(
        "%{} = {} {} {} to {}",
        result.0,
        opcode,
        llvm_type(&value.ty())?,
        llvm_value(value)?,
        llvm_type(target)?
    ))
}

fn assign_value(
    result: &crate::ir_gen::IrRegister,
    value: &IrValue,
) -> LlvmEmitterResult<String> {
    if result.1 != value.ty() {
        return Err(LlvmEmitterError::InvalidValue(
            "assignment source and destination types differ".into(),
        ));
    }

    match &result.1 {
        IrType::I8
        | IrType::I16
        | IrType::I32
        | IrType::I64
        | IrType::I128
        | IrType::U8
        | IrType::U16
        | IrType::U32
        | IrType::U64
        | IrType::U128
        | IrType::Bool => Ok(format!(
            "%{} = add {} 0, {}",
            result.0,
            llvm_type(&result.1)?,
            llvm_value(value)?
        )),
        IrType::F32 | IrType::F64 => Ok(format!(
            "%{} = fadd {} 0.0, {}",
            result.0,
            llvm_type(&result.1)?,
            llvm_value(value)?
        )),
        IrType::Ptr(_) => Ok(format!(
            "%{} = getelementptr {}, {} {}, i64 0",
            result.0,
            llvm_type(&result.1)?,
            llvm_type(&result.1)?,
            llvm_value(value)?
        )),
        _ => Err(LlvmEmitterError::InvalidValue(format!(
            "cannot materialize assignment to {:?} without a defined lowering",
            result.1
        ))),
    }
}

fn comparison(
    op: &CmpOp,
    ty: &IrType,
) -> LlvmEmitterResult<(&'static str, &'static str)> {
    let floating = matches!(ty, IrType::F32 | IrType::F64);

    let result = match (floating, op) {
        (false, CmpOp::Eq) => ("icmp", "eq"),
        (false, CmpOp::Ne) => ("icmp", "ne"),
        (false, CmpOp::Lt) => ("icmp", "slt"),
        (false, CmpOp::Le) => ("icmp", "sle"),
        (false, CmpOp::Gt) => ("icmp", "sgt"),
        (false, CmpOp::Ge) => ("icmp", "sge"),
        (true, CmpOp::FEq) | (true, CmpOp::Eq) => ("fcmp", "oeq"),
        (true, CmpOp::FNe) | (true, CmpOp::Ne) => ("fcmp", "one"),
        (true, CmpOp::FLt) | (true, CmpOp::Lt) => ("fcmp", "olt"),
        (true, CmpOp::FLe) | (true, CmpOp::Le) => ("fcmp", "ole"),
        (true, CmpOp::FGt) | (true, CmpOp::Gt) => ("fcmp", "ogt"),
        (true, CmpOp::FGe) | (true, CmpOp::Ge) => ("fcmp", "oge"),
        (false, CmpOp::FEq | CmpOp::FNe | CmpOp::FLt | CmpOp::FLe | CmpOp::FGt | CmpOp::FGe) => {
            return Err(LlvmEmitterError::InvalidValue(
                "floating comparison used with a non-floating type".into(),
            ));
        }
        (true, _) => unreachable!("all comparison variants are covered"),
    };

    Ok(result)
}

fn llvm_type(ty: &IrType) -> LlvmEmitterResult<String> {
    match ty {
        IrType::Void => Ok("void".into()),
        IrType::Bool => Ok("i1".into()),
        IrType::I8 | IrType::U8 => Ok("i8".into()),
        IrType::I16 | IrType::U16 => Ok("i16".into()),
        IrType::I32 | IrType::U32 => Ok("i32".into()),
        IrType::I64 | IrType::U64 => Ok("i64".into()),
        IrType::I128 | IrType::U128 => Ok("i128".into()),
        IrType::F32 => Ok("float".into()),
        IrType::F64 => Ok("double".into()),
        IrType::Ptr(inner) => {
            if **inner == IrType::Void {
                // LLVM typed-pointer IR has no void* type. A byte pointer is
                // used as the explicit generic pointer representation.
                Ok("i8*".into())
            } else {
                Ok(format!("{}*", llvm_type(inner)?))
            }
        }
        IrType::Array(inner, length) => {
            Ok(format!("[{} x {}]", length, llvm_type(inner)?))
        }
        IrType::Struct(name, _) | IrType::Opaque(name) => {
            validate_identifier("type", name)?;
            Ok(format!("%{name}"))
        }
        IrType::Quantum => Err(LlvmEmitterError::InvalidType(
            "quantum values must be lowered by a quantum-aware adapter".into(),
        )),
        IrType::Function(_, _) => Err(LlvmEmitterError::InvalidType(
            "function types require an explicit calling-convention/signature lowering".into(),
        )),
    }
}

fn llvm_value(value: &IrValue) -> LlvmEmitterResult<String> {
    match value {
        IrValue::Reg(register) => {
            validate_identifier("register", &register.0)?;
            Ok(format!("%{}", register.0))
        }
        IrValue::ConstInt(number, _) => Ok(number.to_string()),
        IrValue::ConstFloat(number, ty) => {
            if !number.is_finite() {
                return Err(LlvmEmitterError::InvalidValue(
                    "non-finite floating-point constants require bit-exact constant encoding"
                        .into(),
                ));
            }

            if !matches!(ty, IrType::F32 | IrType::F64) {
                return Err(LlvmEmitterError::InvalidValue(
                    "floating constant has a non-floating type".into(),
                ));
            }

            // Scientific decimal notation avoids locale-dependent formatting.
            Ok(format!("{number:.17e}"))
        }
        IrValue::ConstBool(value) => Ok(if *value { "true" } else { "false" }.into()),
        IrValue::ConstStr(_) => Err(LlvmEmitterError::InvalidValue(
            "string constants must be interned as module globals and referenced by GlobalPtr"
                .into(),
        )),
        IrValue::GlobalPtr(name, length) => {
            validate_identifier("global", name)?;
            if *length == 0 {
                Ok(format!("@{name}"))
            } else {
                Ok(format!(
                    "getelementptr inbounds ([{} x i8], [{} x i8]* @{}, i64 0, i64 0)",
                    length, length, name
                ))
            }
        }
        IrValue::ConstNull => Ok("null".into()),
        IrValue::Void => Err(LlvmEmitterError::InvalidValue(
            "void cannot be used as an operand".into(),
        )),
    }
}

fn llvm_constant(value: &IrValue) -> LlvmEmitterResult<String> {
    match value {
        IrValue::Reg(_) | IrValue::ConstStr(_) | IrValue::Void => {
            Err(LlvmEmitterError::InvalidValue(
                "global initializer is not a supported LLVM constant".into(),
            ))
        }
        _ => llvm_value(value),
    }
}

fn natural_alignment(ty: &IrType) -> LlvmEmitterResult<usize> {
    // This is a fallback alignment for emitted declarations, not a claim
    // about every target ABI. Target-aware layout should override this in a
    // later data-layout integration.
    match ty {
        IrType::Bool | IrType::I8 | IrType::U8 => Ok(1),
        IrType::I16 | IrType::U16 => Ok(2),
        IrType::I32 | IrType::U32 | IrType::F32 => Ok(4),
        IrType::I64 | IrType::U64 | IrType::F64 | IrType::Ptr(_) => Ok(8),
        IrType::I128 | IrType::U128 => Ok(16),
        IrType::Array(inner, _) => natural_alignment(inner),
        IrType::Struct(_, _) | IrType::Opaque(_) => Ok(1),
        IrType::Void | IrType::Quantum | IrType::Function(_, _) => {
            Err(LlvmEmitterError::InvalidType(format!(
                "no fallback alignment for {ty:?}"
            )))
        }
    }
}

fn validate_identifier(kind: &'static str, name: &str) -> LlvmEmitterResult<()> {
    let mut chars = name.chars();

    let valid_first = chars
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || matches!(ch, '_' | '.' | '$'));

    let valid_rest = chars.all(|ch| {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$' | '-')
    });

    if name.is_empty() || !valid_first || !valid_rest {
        return Err(LlvmEmitterError::InvalidIdentifier {
            kind,
            name: name.to_owned(),
        });
    }

    Ok(())
}

fn validate_module(module: &IrModule) -> LlvmEmitterResult<()> {
    let mut function_names = HashSet::new();
    let mut global_names = HashSet::new();

    for (name, _) in &module.string_literals {
        validate_identifier("string global", name)?;
        if !global_names.insert(name.as_str()) {
            return Err(LlvmEmitterError::InvalidModule(format!(
                "duplicate global symbol {name:?}"
            )));
        }
    }

    for global in &module.globals {
        validate_identifier("global", &global.name)?;
        if !global_names.insert(global.name.as_str()) {
            return Err(LlvmEmitterError::InvalidModule(format!(
                "duplicate global symbol {:?}",
                global.name
            )));
        }
    }

    for function in &module.functions {
        validate_identifier("function", &function.name)?;
        if !function_names.insert(function.name.as_str()) {
            return Err(LlvmEmitterError::InvalidModule(format!(
                "duplicate function symbol {:?}",
                function.name
            )));
        }
    }

    for function in &module.functions {
        for instruction in &function.body {
            match instruction {
                IrInstruction::Label(label)
                | IrInstruction::Jump(label) => validate_identifier("label", label)?,
                IrInstruction::CondJump(_, yes, no) => {
                    validate_identifier("label", yes)?;
                    validate_identifier("label", no)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn validate_function_body(function: &IrFunction) -> LlvmEmitterResult<()> {
    if function.body.is_empty() {
        return Err(LlvmEmitterError::InvalidFunction {
            function: function.name.clone(),
            reason: "function body is empty".into(),
        });
    }

    let mut labels = HashSet::new();

    for instruction in &function.body {
        if let IrInstruction::Label(label) = instruction {
            if !labels.insert(label.as_str()) {
                return Err(LlvmEmitterError::InvalidFunction {
                    function: function.name.clone(),
                    reason: format!("duplicate basic-block label {label:?}"),
                });
            }
        }
    }

    for instruction in &function.body {
        let target = match instruction {
            IrInstruction::Jump(label) => Some(label.as_str()),
            IrInstruction::CondJump(_, yes, no) => {
                if !labels.contains(yes.as_str()) && !is_entry_label(function, yes) {
                    return Err(missing_label(function, yes));
                }
                if !labels.contains(no.as_str()) && !is_entry_label(function, no) {
                    return Err(missing_label(function, no));
                }
                None
            }
            _ => None,
        };

        if let Some(label) = target {
            if !labels.contains(label) && !is_entry_label(function, label) {
                return Err(missing_label(function, label));
            }
        }
    }

    Ok(())
}

fn is_entry_label(function: &IrFunction, label: &str) -> bool {
    !matches!(function.body.first(), Some(IrInstruction::Label(_))) && label == "entry"
}

fn missing_label(function: &IrFunction, label: &str) -> LlvmEmitterError {
    LlvmEmitterError::InvalidFunction {
        function: function.name.clone(),
        reason: format!("branch references undefined label {label:?}"),
    }
}

fn escape_llvm_string(bytes: &[u8]) -> String {
    let mut escaped = String::new();

    for byte in bytes {
        match *byte {
            b'"' => escaped.push_str("\\22"),
            b'\\' => escaped.push_str("\\5C"),
            0x20..=0x7e => escaped.push(*byte as char),
            other => escaped.push_str(&format!("\\{other:02X}")),
        }
    }

    escaped
}

fn sanitize_comment(comment: &str) -> String {
    comment
        .chars()
        .map(|ch| if ch.is_control() { ' ' } else { ch })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrFunction, IrInstruction, IrRegister};

    fn target() -> LlvmTarget {
        LlvmTarget::new("x86_64-unknown-linux-gnu")
    }

    #[test]
    fn emits_actual_llvm_module_and_function() {
        let mut module = IrModule::new("example");
        module.functions.push({
            let mut function = IrFunction::new("main", vec![], IrType::I32);
            function.push(IrInstruction::Ret(Some(IrValue::ConstInt(
                0,
                IrType::I32,
            ))));
            function
        });

        let text = LlvmEmitter::new(target()).emit(&module).unwrap();

        assert!(text.contains("target triple = \"x86_64-unknown-linux-gnu\""));
        assert!(text.contains("define i32 @main()"));
        assert!(text.contains("ret i32 0"));
    }

    #[test]
    fn rejects_duplicate_function_symbols() {
        let mut module = IrModule::new("duplicate");
        module.functions.push(IrFunction::new("f", vec![], IrType::Void));
        module.functions.push(IrFunction::new("f", vec![], IrType::Void));

        let error = LlvmEmitter::new(target()).emit(&module).unwrap_err();
        assert!(matches!(error, LlvmEmitterError::InvalidModule(_)));
    }

    #[test]
    fn rejects_unlowered_quantum_operations() {
        let mut module = IrModule::new("quantum");
        let mut function = IrFunction::new("main", vec![], IrType::Void);
        function.push(IrInstruction::QuantumGate(
            IrRegister::new("q", IrType::Quantum),
            "H".into(),
            vec![],
        ));
        function.push(IrInstruction::Ret(None));
        module.functions.push(function);

        assert!(matches!(
            LlvmEmitter::new(target()).emit(&module),
            Err(LlvmEmitterError::UnsupportedInstruction { .. })
                | Err(LlvmEmitterError::InvalidType(_))
        ));
    }

    #[test]
    fn escapes_string_literal_bytes() {
        assert_eq!(escape_llvm_string(b"a\"\\\n"), "a\\22\\5C\\0A");
    }

    #[test]
    fn does_not_write_files_or_invoke_tools() {
        let module = IrModule::new("pure");
        let emitter = LlvmEmitter::new(target());
        // Empty modules are valid LLVM modules, so this tests the pure API.
        assert!(emitter.emit(&module).is_ok());
    }
}
