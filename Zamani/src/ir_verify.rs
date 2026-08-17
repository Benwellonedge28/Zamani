//! ZIR structural verifier.
//!
//! This module validates the invariants that later compiler stages rely on.
//! It deliberately focuses on structural/type correctness rather than trying
//! to prove whole-program semantic equivalence.
//!
//! Verification happens between IR generation and optimization/backend stages.

use crate::ir_gen::{
    IrFunction,
    IrInstruction,
    IrModule,
    IrRegister,
    IrType,
    IrValue,
};

use std::collections::{HashMap, HashSet};

/// A single ZIR verification failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrVerificationError {
    /// Function containing the error, if applicable.
    pub function: Option<String>,

    /// Instruction index containing the error, if applicable.
    pub instruction: Option<usize>,

    /// Human-readable explanation.
    pub message: String,
}

impl std::fmt::Display for IrVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.function, self.instruction) {
            (Some(function), Some(index)) => {
                write!(f, "{}:{}: {}", function, index, self.message)
            }

            (Some(function), None) => {
                write!(f, "{}: {}", function, self.message)
            }

            (None, Some(index)) => {
                write!(f, "instruction {}: {}", index, self.message)
            }

            (None, None) => {
                write!(f, "{}", self.message)
            }
        }
    }
}

impl std::error::Error for IrVerificationError {}

/// Verify an entire ZIR module.
///
/// Returns `Ok(())` when all structural invariants hold.
///
/// Returns every discovered error rather than stopping at the first error.
pub fn verify_module(
    module: &IrModule,
) -> Result<(), Vec<IrVerificationError>> {
    let mut errors = Vec::new();

    let mut function_names = HashSet::new();
    let mut global_names = HashSet::new();

    // ---------------------------------------------------------------------
    // Globals
    // ---------------------------------------------------------------------

    for global in &module.globals {
        if !global_names.insert(global.name.clone()) {
            errors.push(error(
                None,
                None,
                format!("duplicate global `{}`", global.name),
            ));
        }
    }

    // ---------------------------------------------------------------------
    // Functions
    // ---------------------------------------------------------------------

    for function in &module.functions {
        if !function_names.insert(function.name.clone()) {
            errors.push(error(
                None,
                None,
                format!("duplicate function `{}`", function.name),
            ));
        }

        verify_function(function, &mut errors);
    }

    // ---------------------------------------------------------------------
    // Symbol collisions
    // ---------------------------------------------------------------------

    for (name, _) in &module.string_literals {
        if function_names.contains(name) || global_names.contains(name) {
            errors.push(error(
                None,
                None,
                format!(
                    "string literal `{}` collides with another module symbol",
                    name
                ),
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Verify a single IR function.
fn verify_function(
    function: &IrFunction,
    errors: &mut Vec<IrVerificationError>,
) {
    let mut definitions: HashMap<String, IrType> = HashMap::new();

    let mut labels: HashSet<String> = HashSet::new();

    let mut label_positions: HashMap<String, usize> = HashMap::new();

    let mut predecessors: HashMap<String, usize> = HashMap::new();

    // ---------------------------------------------------------------------
    // Parameters
    // ---------------------------------------------------------------------

    for (name, ty) in &function.params {
        if definitions.insert(name.clone(), ty.clone()).is_some() {
            errors.push(error(
                Some(function.name.clone()),
                None,
                format!("duplicate parameter `%{}`", name),
            ));
        }
    }

    // ---------------------------------------------------------------------
    // First pass: collect labels
    // ---------------------------------------------------------------------

    for (index, instruction) in function.body.iter().enumerate() {
        if let IrInstruction::Label(label) = instruction {
            if !labels.insert(label.clone()) {
                errors.push(error(
                    Some(function.name.clone()),
                    Some(index),
                    format!("duplicate label `{}`", label),
                ));
            } else {
                label_positions.insert(label.clone(), index);
            }
        }
    }

    // ---------------------------------------------------------------------
    // Second pass: instructions
    // ---------------------------------------------------------------------

    for (index, instruction) in function.body.iter().enumerate() {
        match instruction {
            IrInstruction::Alloca(register, _) => {
                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::Load(register, pointer) => {
                verify_value(
                    pointer,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::Store(value, pointer) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                verify_value(
                    pointer,
                    &definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::Add(register, a, b)
            | IrInstruction::Sub(register, a, b)
            | IrInstruction::Mul(register, a, b)
            | IrInstruction::Div(register, a, b)
            | IrInstruction::Rem(register, a, b)
            | IrInstruction::And(register, a, b)
            | IrInstruction::Or(register, a, b)
            | IrInstruction::Xor(register, a, b)
            | IrInstruction::Shl(register, a, b)
            | IrInstruction::Shr(register, a, b) => {
                verify_value(
                    a,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                verify_value(
                    b,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                if a.ty() != b.ty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "binary operands have different types",
                    ));
                }

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != a.ty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "binary result type does not match operand type",
                    ));
                }
            }

            IrInstruction::Neg(register, value)
            | IrInstruction::Not(register, value) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != value.ty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "unary result type does not match operand type",
                    ));
                }
            }

            IrInstruction::Cmp(register, _, a, b) => {
                verify_value(
                    a,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                verify_value(
                    b,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                if a.ty() != b.ty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "comparison operands have different types",
                    ));
                }

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != IrType::Bool {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "comparison result must have type bool",
                    ));
                }
            }

            IrInstruction::Label(_) => {}

            IrInstruction::Jump(label) => {
                verify_label(
                    label,
                    &label_positions,
                    function,
                    index,
                    errors,
                );

                *predecessors
                    .entry(label.clone())
                    .or_insert(0) += 1;
            }

            IrInstruction::CondJump(condition, true_label, false_label) => {
                verify_value(
                    condition,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                if condition.ty() != IrType::Bool {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "conditional branch requires bool condition",
                    ));
                }

                verify_label(
                    true_label,
                    &label_positions,
                    function,
                    index,
                    errors,
                );

                verify_label(
                    false_label,
                    &label_positions,
                    function,
                    index,
                    errors,
                );

                *predecessors
                    .entry(true_label.clone())
                    .or_insert(0) += 1;

                *predecessors
                    .entry(false_label.clone())
                    .or_insert(0) += 1;
            }

            IrInstruction::Ret(value) => {
                match value {
                    Some(value) => {
                        verify_value(
                            value,
                            &definitions,
                            function,
                            index,
                            errors,
                        );

                        if value.ty() != function.return_type {
                            errors.push(error(
                                Some(function.name.clone()),
                                Some(index),
                                "return value type does not match function return type",
                            ));
                        }
                    }

                    None => {
                        if function.return_type != IrType::Void {
                            errors.push(error(
                                Some(function.name.clone()),
                                Some(index),
                                "void return in non-void function",
                            ));
                        }
                    }
                }
            }

            IrInstruction::Unreachable => {}

            IrInstruction::Call(output, _, arguments) => {
                for argument in arguments {
                    verify_value(
                        argument,
                        &definitions,
                        function,
                        index,
                        errors,
                    );
                }

                if let Some(register) = output {
                    define_register(
                        register,
                        &mut definitions,
                        function,
                        index,
                        errors,
                    );
                }
            }

            IrInstruction::CallIndirect(output, callee, arguments) => {
                verify_value(
                    callee,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                for argument in arguments {
                    verify_value(
                        argument,
                        &definitions,
                        function,
                        index,
                        errors,
                    );
                }

                if let Some(register) = output {
                    define_register(
                        register,
                        &mut definitions,
                        function,
                        index,
                        errors,
                    );
                }
            }

            IrInstruction::Assign(register, value) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != value.ty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "assignment type does not match destination type",
                    ));
                }
            }

            IrInstruction::Phi(register, incoming) => {
                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if incoming.is_empty() {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "phi node must have at least one incoming value",
                    ));
                }

                for (value, label) in incoming {
                    verify_value(
                        value,
                        &definitions,
                        function,
                        index,
                        errors,
                    );

                    verify_label(
                        label,
                        &label_positions,
                        function,
                        index,
                        errors,
                    );

                    if value.ty() != register.1 {
                        errors.push(error(
                            Some(function.name.clone()),
                            Some(index),
                            "phi incoming value type does not match phi type",
                        ));
                    }
                }
            }

            IrInstruction::ZExt(register, value, ty)
            | IrInstruction::SExt(register, value, ty)
            | IrInstruction::Trunc(register, value, ty)
            | IrInstruction::FpExt(register, value, ty)
            | IrInstruction::FpTrunc(register, value, ty)
            | IrInstruction::SIToFP(register, value, ty)
            | IrInstruction::FPToSI(register, value, ty) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != *ty {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "conversion result type does not match destination type",
                    ));
                }
            }

            IrInstruction::BitCast(register, value, ty) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );

                if register.1 != *ty {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        "bitcast destination type does not match register type",
                    ));
                }
            }

            IrInstruction::GetElementPtr(register, base, indices) => {
                verify_value(
                    base,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                for index_value in indices {
                    verify_value(
                        index_value,
                        &definitions,
                        function,
                        index,
                        errors,
                    );
                }

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::QuantumGate(register, _, arguments)
            | IrInstruction::NanoOp(register, _, arguments) => {
                for argument in arguments {
                    verify_value(
                        argument,
                        &definitions,
                        function,
                        index,
                        errors,
                    );
                }

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::SankofaRecall(register, domain) => {
                verify_value(
                    domain,
                    &definitions,
                    function,
                    index,
                    errors,
                );

                define_register(
                    register,
                    &mut definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::SankofaRemember(_, value) => {
                verify_value(
                    value,
                    &definitions,
                    function,
                    index,
                    errors,
                );
            }

            IrInstruction::Comment(_) => {}
        }
    }

    // ---------------------------------------------------------------------
    // Function termination
    // ---------------------------------------------------------------------

    if !function.is_external {
        if function.body.is_empty() {
            errors.push(error(
                Some(function.name.clone()),
                None,
                "non-external function has an empty body",
            ));
        } else if let Some(last) = function.body.last() {
            if !matches!(
                last,
                IrInstruction::Ret(_)
                    | IrInstruction::Unreachable
                    | IrInstruction::Jump(_)
                    | IrInstruction::CondJump(_, _, _)
            ) {
                errors.push(error(
                    Some(function.name.clone()),
                    Some(function.body.len() - 1),
                    "function body falls through without a terminator",
                ));
            }
        }
    }

    // ---------------------------------------------------------------------
    // Phi predecessor validation
    // ---------------------------------------------------------------------

    for (index, instruction) in function.body.iter().enumerate() {
        if let IrInstruction::Phi(_, incoming) = instruction {
            for (_, label) in incoming {
                if predecessors.get(label).copied().unwrap_or(0) == 0 {
                    errors.push(error(
                        Some(function.name.clone()),
                        Some(index),
                        format!(
                            "phi incoming label `{}` has no recorded predecessor",
                            label
                        ),
                    ));
                }
            }
        }
    }
}

/// Register definition validation.
fn define_register(
    register: &IrRegister,
    definitions: &mut HashMap<String, IrType>,
    function: &IrFunction,
    index: usize,
    errors: &mut Vec<IrVerificationError>,
) {
    if definitions
        .insert(register.0.clone(), register.1.clone())
        .is_some()
    {
        errors.push(error(
            Some(function.name.clone()),
            Some(index),
            format!(
                "register `%{}` is defined more than once",
                register.0
            ),
        ));
    }
}

/// Validate an IR value.
fn verify_value(
    value: &IrValue,
    definitions: &HashMap<String, IrType>,
    function: &IrFunction,
    index: usize,
    errors: &mut Vec<IrVerificationError>,
) {
    if let IrValue::Reg(register) = value {
        match definitions.get(&register.0) {
            Some(ty) if ty != &register.1 => {
                errors.push(error(
                    Some(function.name.clone()),
                    Some(index),
                    format!(
                        "register `%{}` used with inconsistent type",
                        register.0
                    ),
                ));
            }

            Some(_) => {}

            None => {
                errors.push(error(
                    Some(function.name.clone()),
                    Some(index),
                    format!(
                        "use of undefined register `%{}`",
                        register.0
                    ),
                ));
            }
        }
    }
}

/// Validate a branch target.
fn verify_label(
    label: &str,
    labels: &HashMap<String, usize>,
    function: &IrFunction,
    index: usize,
    errors: &mut Vec<IrVerificationError>,
) {
    if !labels.contains_key(label) {
        errors.push(error(
            Some(function.name.clone()),
            Some(index),
            format!("branch references undefined label `{}`", label),
        ));
    }
}

/// Construct a verification error.
fn error(
    function: Option<String>,
    instruction: Option<usize>,
    message: impl Into<String>,
) -> IrVerificationError {
    IrVerificationError {
        function,
        instruction,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_valid_function() {
        let mut module = IrModule::new("test");

        let mut function =
            IrFunction::new("main", vec![], IrType::I32);

        function.push(IrInstruction::Ret(Some(
            IrValue::ConstInt(0, IrType::I32),
        )));

        module.add_function(function);

        assert!(verify_module(&module).is_ok());
    }

    #[test]
    fn rejects_undefined_register() {
        let mut module = IrModule::new("test");

        let mut function =
            IrFunction::new("main", vec![], IrType::I32);

        let register =
            IrRegister::new("missing", IrType::I32);

        function.push(IrInstruction::Ret(Some(
            IrValue::Reg(register),
        )));

        module.add_function(function);

        assert!(verify_module(&module).is_err());
    }

    #[test]
    fn rejects_bad_branch_target() {
        let mut module = IrModule::new("test");

        let mut function =
            IrFunction::new("main", vec![], IrType::Void);

        function.push(
            IrInstruction::Jump("missing".into())
        );

        module.add_function(function);

        assert!(verify_module(&module).is_err());
    }
}