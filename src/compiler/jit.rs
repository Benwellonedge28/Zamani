//! Zamani Compiler — Just-In-Time Execution Engine
//!
//! This module provides the execution boundary for Zamani IR.
//!
//! Important architectural rule:
//!
//! `JitEngine` does NOT claim to emit native machine code unless a real native
//! JIT backend is installed. The default implementation is a deterministic
//! IR execution engine. This keeps the compiler honest and gives tests,
//! tooling, the REPL, and future native JIT backends a stable execution API.
//!
//! Future native backends can implement the same execution contract without
//! changing callers of this module.
//!
//! Execution pipeline:
//!
//!     IrModule
//!        |
//!        v
//!     function lookup
//!        |
//!        v
//!     instruction execution
//!        |
//!        v
//!     return value
//!
//! The engine is deliberately bounded. Instruction and call limits prevent
//! accidental infinite loops from consuming the host indefinitely.

use crate::ir_gen::{
    CmpOp,
    IrFunction,
    IrInstruction,
    IrModule,
    IrRegister,
    IrType,
    IrValue,
};

use std::collections::HashMap;
use std::fmt;

// -----------------------------------------------------------------------------
// Public configuration
// -----------------------------------------------------------------------------

/// Configuration for JIT/IR execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitConfig {
    /// Optimization level requested by the caller.
    ///
    /// The execution engine itself does not perform speculative optimization.
    /// The value is retained so a future native JIT backend can use it.
    pub optimization_level: u32,

    /// Maximum number of instructions that may execute during one invocation.
    pub max_instructions: u64,

    /// Maximum call depth.
    pub max_call_depth: usize,

    /// Whether external functions are permitted.
    ///
    /// This is false by default because executing arbitrary external symbols
    /// is a security boundary and must never happen implicitly.
    pub allow_external_functions: bool,
}

impl Default for JitConfig {
    fn default() -> Self {
        Self {
            optimization_level: 0,
            max_instructions: 1_000_000,
            max_call_depth: 256,
            allow_external_functions: false,
        }
    }
}

impl JitConfig {
    /// Creates a conservative production configuration.
    pub fn production() -> Self {
        Self::default()
    }

    /// Validates configuration values.
    pub fn validate(&self) -> Result<(), JitError> {
        if self.max_instructions == 0 {
            return Err(JitError::InvalidConfiguration(
                "max_instructions must be greater than zero".to_string(),
            ));
        }

        if self.max_call_depth == 0 {
            return Err(JitError::InvalidConfiguration(
                "max_call_depth must be greater than zero".to_string(),
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Runtime value
// -----------------------------------------------------------------------------

/// Runtime value used by the deterministic IR interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum JitValue {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Pointer(u64),
    String(String),
}

impl JitValue {
    fn as_int(&self) -> Result<i64, JitError> {
        match self {
            Self::Int(value) => Ok(*value),
            Self::Bool(value) => Ok(if *value { 1 } else { 0 }),
            other => Err(JitError::TypeMismatch {
                expected: "integer".to_string(),
                actual: format!("{other:?}"),
            }),
        }
    }

    fn as_float(&self) -> Result<f64, JitError> {
        match self {
            Self::Float(value) => Ok(*value),
            Self::Int(value) => Ok(*value as f64),
            other => Err(JitError::TypeMismatch {
                expected: "floating-point value".to_string(),
                actual: format!("{other:?}"),
            }),
        }
    }

    fn as_bool(&self) -> Result<bool, JitError> {
        match self {
            Self::Bool(value) => Ok(*value),
            Self::Int(value) => Ok(*value != 0),
            other => Err(JitError::TypeMismatch {
                expected: "boolean".to_string(),
                actual: format!("{other:?}"),
            }),
        }
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Structured JIT execution failure.
#[derive(Debug, Clone, PartialEq)]
pub enum JitError {
    InvalidConfiguration(String),

    EmptyModule,

    EntryPointNotFound(String),

    FunctionNotFound(String),

    ExternalFunction(String),

    InstructionLimitExceeded {
        limit: u64,
    },

    CallDepthExceeded {
        limit: usize,
    },

    InvalidInstructionPointer {
        function: String,
        instruction: usize,
    },

    MissingRegister(String),

    InvalidRegister(String),

    MissingLabel {
        function: String,
        label: String,
    },

    InvalidBranchCondition,

    InvalidReturn {
        function: String,
    },

    MissingReturnValue {
        function: String,
    },

    DivisionByZero,

    RemainderByZero,

    ArithmeticOverflow,

    InvalidShiftAmount(i64),

    InvalidConversion(String),

    UnsupportedInstruction(String),

    UnsupportedType(String),

    TypeMismatch {
        expected: String,
        actual: String,
    },

    ArgumentCount {
        function: String,
        expected: usize,
        actual: usize,
    },

    ParameterTypeMismatch {
        function: String,
        parameter: String,
    },
}

impl fmt::Display for JitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid JIT configuration: {message}")
            }

            Self::EmptyModule => {
                write!(formatter, "cannot execute an empty IR module")
            }

            Self::EntryPointNotFound(name) => {
                write!(formatter, "JIT entry point '{name}' was not found")
            }

            Self::FunctionNotFound(name) => {
                write!(formatter, "function '{name}' was not found")
            }

            Self::ExternalFunction(name) => {
                write!(
                    formatter,
                    "external function '{name}' cannot be executed by the safe IR JIT"
                )
            }

            Self::InstructionLimitExceeded { limit } => {
                write!(
                    formatter,
                    "JIT instruction execution limit of {limit} was exceeded"
                )
            }

            Self::CallDepthExceeded { limit } => {
                write!(formatter, "JIT call-depth limit of {limit} was exceeded")
            }

            Self::InvalidInstructionPointer {
                function,
                instruction,
            } => {
                write!(
                    formatter,
                    "invalid instruction pointer {instruction} in function '{function}'"
                )
            }

            Self::MissingRegister(register) => {
                write!(formatter, "register '%{register}' has no runtime value")
            }

            Self::InvalidRegister(register) => {
                write!(formatter, "invalid register '{register}'")
            }

            Self::MissingLabel { function, label } => {
                write!(
                    formatter,
                    "label '{label}' was not found in function '{function}'"
                )
            }

            Self::InvalidBranchCondition => {
                write!(formatter, "conditional branch requires a boolean/integer condition")
            }

            Self::InvalidReturn { function } => {
                write!(formatter, "invalid return in function '{function}'")
            }

            Self::MissingReturnValue { function } => {
                write!(
                    formatter,
                    "function '{function}' returned without the required value"
                )
            }

            Self::DivisionByZero => {
                write!(formatter, "integer division by zero")
            }

            Self::RemainderByZero => {
                write!(formatter, "integer remainder by zero")
            }

            Self::ArithmeticOverflow => {
                write!(formatter, "integer arithmetic overflow")
            }

            Self::InvalidShiftAmount(value) => {
                write!(formatter, "invalid shift amount {value}")
            }

            Self::InvalidConversion(message) => {
                write!(formatter, "invalid numeric conversion: {message}")
            }

            Self::UnsupportedInstruction(instruction) => {
                write!(formatter, "unsupported JIT instruction: {instruction}")
            }

            Self::UnsupportedType(ty) => {
                write!(formatter, "unsupported JIT type: {ty}")
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    formatter,
                    "JIT type mismatch: expected {expected}, got {actual}"
                )
            }

            Self::ArgumentCount {
                function,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "function '{function}' expects {expected} argument(s), got {actual}"
                )
            }

            Self::ParameterTypeMismatch {
                function,
                parameter,
            } => {
                write!(
                    formatter,
                    "argument type mismatch for parameter '{parameter}' in function '{function}'"
                )
            }
        }
    }
}

impl std::error::Error for JitError {}

// -----------------------------------------------------------------------------
// Execution statistics
// -----------------------------------------------------------------------------

/// Runtime statistics produced by one JIT invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JitStatistics {
    pub instructions_executed: u64,
    pub function_calls: u64,
    pub max_call_depth: usize,
}

impl JitStatistics {
    pub fn instructions_executed(&self) -> u64 {
        self.instructions_executed
    }

    pub fn function_calls(&self) -> u64 {
        self.function_calls
    }

    pub fn max_call_depth(&self) -> usize {
        self.max_call_depth
    }
}

// -----------------------------------------------------------------------------
// Execution result
// -----------------------------------------------------------------------------

/// Result of one JIT invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct JitExecutionResult {
    pub value: JitValue,
    pub statistics: JitStatistics,
}

impl JitExecutionResult {
    pub fn value(&self) -> &JitValue {
        &self.value
    }

    pub fn statistics(&self) -> JitStatistics {
        self.statistics
    }
}

// -----------------------------------------------------------------------------
// JIT engine
// -----------------------------------------------------------------------------

/// Deterministic Zamani IR execution engine.
///
/// This is the safe baseline execution implementation for `jit.rs`.
///
/// It does not execute arbitrary native memory and does not dynamically load
/// symbols. This makes it suitable for tests, REPL execution, compiler
/// validation, and sandboxed tooling.
#[derive(Debug, Clone)]
pub struct JitEngine {
    pub optimization_level: u32,
    pub config: JitConfig,
}

impl JitEngine {
    /// Creates an engine using conservative production defaults.
    pub fn new(opt_level: u32) -> Self {
        let mut config = JitConfig::production();
        config.optimization_level = opt_level;

        Self {
            optimization_level: opt_level,
            config,
        }
    }

    /// Creates an engine from explicit configuration.
    pub fn with_config(config: JitConfig) -> Result<Self, JitError> {
        config.validate()?;

        Ok(Self {
            optimization_level: config.optimization_level,
            config,
        })
    }

    /// Executes the requested entry point and returns only its value.
    ///
    /// This method preserves the public API used by the original JIT module.
    pub fn execute(
        &self,
        module: &IrModule,
        entry_point: &str,
    ) -> Result<i64, String> {
        self.execute_result(module, entry_point, &[])
            .map(|result| match result.value {
                JitValue::Int(value) => value,
                JitValue::Bool(value) => {
                    if value {
                        1
                    } else {
                        0
                    }
                }
                JitValue::Void => 0,
                other => {
                    // The legacy API returns i64. Do not silently reinterpret
                    // floating-point, pointer, or string values.
                    return Err(format!(
                        "JIT entry point '{entry_point}' returned unsupported value {other:?}"
                    ));
                }
            })
            .map_err(|error| error.to_string())
    }

    /// Executes an entry point with explicit arguments.
    pub fn execute_with_args(
        &self,
        module: &IrModule,
        entry_point: &str,
        args: &[JitValue],
    ) -> Result<JitExecutionResult, JitError> {
        self.execute_result(module, entry_point, args)
    }

    /// Executes an entry point with explicit arguments.
    pub fn execute_result(
        &self,
        module: &IrModule,
        entry_point: &str,
        args: &[JitValue],
    ) -> Result<JitExecutionResult, JitError> {
        self.config.validate()?;

        if module.functions.is_empty() {
            return Err(JitError::EmptyModule);
        }

        let function_index = self.find_function(module, entry_point)?;

        let mut state = ExecutionState {
            module,
            config: &self.config,
            statistics: JitStatistics::default(),
        };

        let value = state.execute_function(function_index, args, 0)?;

        Ok(JitExecutionResult {
            value,
            statistics: state.statistics,
        })
    }

    /// Returns whether the module contains a function with the requested name.
    pub fn contains_function(
        &self,
        module: &IrModule,
        name: &str,
    ) -> bool {
        module.functions.iter().any(|function| function.name == name)
    }

    /// Returns the number of functions in an IR module.
    pub fn function_count(&self, module: &IrModule) -> usize {
        module.functions.len()
    }

    fn find_function(
        &self,
        module: &IrModule,
        name: &str,
    ) -> Result<usize, JitError> {
        module
            .functions
            .iter()
            .position(|function| function.name == name)
            .ok_or_else(|| JitError::EntryPointNotFound(name.to_string()))
    }
}

// -----------------------------------------------------------------------------
// Internal execution state
// -----------------------------------------------------------------------------

struct ExecutionState<'a> {
    module: &'a IrModule,
    config: &'a JitConfig,
    statistics: JitStatistics,
}

impl<'a> ExecutionState<'a> {
    fn execute_function(
        &mut self,
        function_index: usize,
        args: &[JitValue],
        call_depth: usize,
    ) -> Result<JitValue, JitError> {
        if call_depth >= self.config.max_call_depth {
            return Err(JitError::CallDepthExceeded {
                limit: self.config.max_call_depth,
            });
        }

        let function = self
            .module
            .functions
            .get(function_index)
            .ok_or_else(|| {
                JitError::FunctionNotFound(function_index.to_string())
            })?;

        if function.is_external {
            if !self.config.allow_external_functions {
                return Err(JitError::ExternalFunction(
                    function.name.clone(),
                ));
            }

            return Err(JitError::ExternalFunction(
                function.name.clone(),
            ));
        }

        if function.params.len() != args.len() {
            return Err(JitError::ArgumentCount {
                function: function.name.clone(),
                expected: function.params.len(),
                actual: args.len(),
            });
        }

        let function_name = function.name.clone();

        let mut registers: HashMap<String, JitValue> = HashMap::new();

        for ((parameter_name, parameter_type), value) in
            function.params.iter().zip(args.iter())
        {
            validate_runtime_type(value, parameter_type).map_err(|_| {
                JitError::ParameterTypeMismatch {
                    function: function_name.clone(),
                    parameter: parameter_name.clone(),
                }
            })?;

            registers.insert(parameter_name.clone(), value.clone());
        }

        let labels = build_label_table(function);

        let mut instruction_pointer = 0usize;
        let mut previous_label: Option<String> = None;

        loop {
            if instruction_pointer >= function.body.len() {
                return Err(JitError::InvalidInstructionPointer {
                    function: function_name.clone(),
                    instruction: instruction_pointer,
                });
            }

            self.consume_instruction()?;

            let instruction = function
                .body
                .get(instruction_pointer)
                .ok_or_else(|| JitError::InvalidInstructionPointer {
                    function: function_name.clone(),
                    instruction: instruction_pointer,
                })?;

            match instruction {
                IrInstruction::Comment(_) => {
                    instruction_pointer += 1;
                }

                IrInstruction::Label(label) => {
                    previous_label = Some(label.clone());
                    instruction_pointer += 1;
                }

                IrInstruction::Assign(register, value) => {
                    let runtime = self.resolve_value(value, &registers)?;
                    registers.insert(register.0.clone(), runtime);
                    instruction_pointer += 1;
                }

                IrInstruction::Add(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| a.checked_add(b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Sub(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| a.checked_sub(b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Mul(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| a.checked_mul(b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Div(register, lhs, rhs) => {
                    let a = self.resolve_value(lhs, &registers)?.as_int()?;
                    let b = self.resolve_value(rhs, &registers)?.as_int()?;

                    if b == 0 {
                        return Err(JitError::DivisionByZero);
                    }

                    let value = a
                        .checked_div(b)
                        .ok_or(JitError::ArithmeticOverflow)?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Rem(register, lhs, rhs) => {
                    let a = self.resolve_value(lhs, &registers)?.as_int()?;
                    let b = self.resolve_value(rhs, &registers)?.as_int()?;

                    if b == 0 {
                        return Err(JitError::RemainderByZero);
                    }

                    let value = a
                        .checked_rem(b)
                        .ok_or(JitError::ArithmeticOverflow)?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Neg(register, value) => {
                    let value = self
                        .resolve_value(value, &registers)?
                        .as_int()?;

                    let value = value
                        .checked_neg()
                        .ok_or(JitError::ArithmeticOverflow)?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::And(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| Some(a & b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Or(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| Some(a | b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Xor(register, lhs, rhs) => {
                    let value = self.integer_binary(
                        lhs,
                        rhs,
                        &registers,
                        |a, b| Some(a ^ b),
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Shl(register, lhs, rhs) => {
                    let value = self.shift(
                        lhs,
                        rhs,
                        &registers,
                        false,
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Shr(register, lhs, rhs) => {
                    let value = self.shift(
                        lhs,
                        rhs,
                        &registers,
                        true,
                    )?;

                    registers.insert(register.0.clone(), JitValue::Int(value));
                    instruction_pointer += 1;
                }

                IrInstruction::Not(register, value) => {
                    let value = self
                        .resolve_value(value, &registers)?
                        .as_int()?;

                    registers.insert(register.0.clone(), JitValue::Int(!value));
                    instruction_pointer += 1;
                }

                IrInstruction::Cmp(register, op, lhs, rhs) => {
                    let result =
                        self.compare(op, lhs, rhs, &registers)?;

                    registers.insert(
                        register.0.clone(),
                        JitValue::Bool(result),
                    );

                    instruction_pointer += 1;
                }

                IrInstruction::Jump(label) => {
                    instruction_pointer = *labels.get(label).ok_or_else(|| {
                        JitError::MissingLabel {
                            function: function_name.clone(),
                            label: label.clone(),
                        }
                    })?;
                }

                IrInstruction::CondJump(condition, true_label, false_label) => {
                    let condition =
                        self.resolve_value(condition, &registers)?;

                    let condition = condition.as_bool().map_err(|_| {
                        JitError::InvalidBranchCondition
                    })?;

                    let target = if condition {
                        true_label
                    } else {
                        false_label
                    };

                    instruction_pointer = *labels.get(target).ok_or_else(|| {
                        JitError::MissingLabel {
                            function: function_name.clone(),
                            label: target.clone(),
                        }
                    })?;
                }

                IrInstruction::Ret(value) => {
                    let result = match value {
                        Some(value) => {
                            let runtime =
                                self.resolve_value(value, &registers)?;

                            validate_runtime_type(
                                &runtime,
                                &function.return_type,
                            )?;

                            runtime
                        }
                        None => {
                            if function.return_type != IrType::Void {
                                return Err(
                                    JitError::MissingReturnValue {
                                        function: function_name.clone(),
                                    },
                                );
                            }

                            JitValue::Void
                        }
                    };

                    return Ok(result);
                }

                IrInstruction::Unreachable => {
                    return Err(JitError::UnsupportedInstruction(
                        "reached unreachable instruction".to_string(),
                    ));
                }

                IrInstruction::Call(
                    destination,
                    callee,
                    arguments,
                ) => {
                    let callee_index = self
                        .module
                        .functions
                        .iter()
                        .position(|function| function.name == *callee)
                        .ok_or_else(|| {
                            JitError::FunctionNotFound(callee.clone())
                        })?;

                    let values = arguments
                        .iter()
                        .map(|argument| {
                            self.resolve_value(argument, &registers)
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    self.statistics.function_calls += 1;

                    let result = self.execute_function(
                        callee_index,
                        &values,
                        call_depth + 1,
                    )?;

                    if let Some(destination) = destination {
                        registers.insert(destination.0.clone(), result);
                    }

                    instruction_pointer += 1;
                }

                IrInstruction::CallIndirect(_, _, _) => {
                    return Err(JitError::UnsupportedInstruction(
                        "indirect calls are not enabled by the safe JIT"
                            .to_string(),
                    ));
                }

                IrInstruction::Alloca(_, _)
                | IrInstruction::Load(_, _)
                | IrInstruction::Store(_, _)
                | IrInstruction::GetElementPtr(_, _, _)
                | IrInstruction::BitCast(_, _, _)
                | IrInstruction::Phi(_, _)
                | IrInstruction::ZExt(_, _, _)
                | IrInstruction::SExt(_, _, _)
                | IrInstruction::Trunc(_, _, _)
                | IrInstruction::FpExt(_, _, _)
                | IrInstruction::FpTrunc(_, _, _)
                | IrInstruction::SIToFP(_, _, _)
                | IrInstruction::FPToSI(_, _, _)
                | IrInstruction::QuantumGate(_, _, _)
                | IrInstruction::NanoOp(_, _, _)
                | IrInstruction::SankofaRecall(_, _)
                | IrInstruction::SankofaRemember(_, _)
                => {
                    return Err(JitError::UnsupportedInstruction(
                        format!("{instruction:?}"),
                    ));
                }
            }

            let _ = &previous_label;
        }
    }

    fn consume_instruction(&mut self) -> Result<(), JitError> {
        if self.statistics.instructions_executed
            >= self.config.max_instructions
        {
            return Err(JitError::InstructionLimitExceeded {
                limit: self.config.max_instructions,
            });
        }

        self.statistics.instructions_executed += 1;

        Ok(())
    }

    fn resolve_value(
        &self,
        value: &IrValue,
        registers: &HashMap<String, JitValue>,
    ) -> Result<JitValue, JitError> {
        match value {
            IrValue::Reg(register) => registers
                .get(&register.0)
                .cloned()
                .ok_or_else(|| {
                    JitError::MissingRegister(register.0.clone())
                }),

            IrValue::ConstInt(value, _) => Ok(JitValue::Int(*value)),

            IrValue::ConstFloat(value, _) => Ok(JitValue::Float(*value)),

            IrValue::ConstBool(value) => Ok(JitValue::Bool(*value)),

            IrValue::ConstStr(value) => {
                Ok(JitValue::String(value.clone()))
            }

            IrValue::GlobalPtr(_, _) => {
                Err(JitError::UnsupportedInstruction(
                    "global pointers are not dereferenced by the safe baseline JIT"
                        .to_string(),
                ))
            }

            IrValue::ConstNull => Ok(JitValue::Pointer(0)),

            IrValue::Void => Ok(JitValue::Void),
        }
    }

    fn integer_binary<F>(
        &self,
        lhs: &IrValue,
        rhs: &IrValue,
        registers: &HashMap<String, JitValue>,
        operation: F,
    ) -> Result<i64, JitError>
    where
        F: FnOnce(i64, i64) -> Option<i64>,
    {
        let lhs = self.resolve_value(lhs, registers)?.as_int()?;
        let rhs = self.resolve_value(rhs, registers)?.as_int()?;

        operation(lhs, rhs).ok_or(JitError::ArithmeticOverflow)
    }

    fn shift(
        &self,
        lhs: &IrValue,
        rhs: &IrValue,
        registers: &HashMap<String, JitValue>,
        arithmetic_right: bool,
    ) -> Result<i64, JitError> {
        let value = self.resolve_value(lhs, registers)?.as_int()?;
        let amount = self.resolve_value(rhs, registers)?.as_int()?;

        if !(0..64).contains(&amount) {
            return Err(JitError::InvalidShiftAmount(amount));
        }

        let amount = amount as u32;

        if arithmetic_right {
            Ok(value >> amount)
        } else {
            Ok(value << amount)
        }
    }

    fn compare(
        &self,
        op: &CmpOp,
        lhs: &IrValue,
        rhs: &IrValue,
        registers: &HashMap<String, JitValue>,
    ) -> Result<bool, JitError> {
        let lhs = self.resolve_value(lhs, registers)?;
        let rhs = self.resolve_value(rhs, registers)?;

        match op {
            CmpOp::Eq => Ok(lhs == rhs),
            CmpOp::Ne => Ok(lhs != rhs),

            CmpOp::Lt => Ok(lhs.as_int()? < rhs.as_int()?),
            CmpOp::Le => Ok(lhs.as_int()? <= rhs.as_int()?),
            CmpOp::Gt => Ok(lhs.as_int()? > rhs.as_int()?),
            CmpOp::Ge => Ok(lhs.as_int()? >= rhs.as_int()?),

            CmpOp::FLt => Ok(lhs.as_float()? < rhs.as_float()?),
            CmpOp::FLe => Ok(lhs.as_float()? <= rhs.as_float()?),
            CmpOp::FGt => Ok(lhs.as_float()? > rhs.as_float()?),
            CmpOp::FGe => Ok(lhs.as_float()? >= rhs.as_float()?),
            CmpOp::FEq => Ok(lhs.as_float()? == rhs.as_float()?),
            CmpOp::FNe => Ok(lhs.as_float()? != rhs.as_float()?),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn build_label_table(function: &IrFunction) -> HashMap<String, usize> {
    let mut labels = HashMap::new();

    for (index, instruction) in function.body.iter().enumerate() {
        if let IrInstruction::Label(label) = instruction {
            labels.entry(label.clone()).or_insert(index);
        }
    }

    labels
}

fn validate_runtime_type(
    value: &JitValue,
    ty: &IrType,
) -> Result<(), JitError> {
    let valid = match (value, ty) {
        (JitValue::Void, IrType::Void) => true,

        (JitValue::Bool(_), IrType::Bool) => true,

        (JitValue::Int(_), IrType::I8)
        | (JitValue::Int(_), IrType::I16)
        | (JitValue::Int(_), IrType::I32)
        | (JitValue::Int(_), IrType::I64)
        | (JitValue::Int(_), IrType::I128)
        | (JitValue::Int(_), IrType::U8)
        | (JitValue::Int(_), IrType::U16)
        | (JitValue::Int(_), IrType::U32)
        | (JitValue::Int(_), IrType::U64)
        | (JitValue::Int(_), IrType::U128) => true,

        (JitValue::Float(_), IrType::F32)
        | (JitValue::Float(_), IrType::F64) => true,

        (JitValue::Pointer(_), IrType::Ptr(_)) => true,

        (JitValue::String(_), IrType::Ptr(_)) => true,

        (_, IrType::Opaque(_))
        | (_, IrType::Array(_, _))
        | (_, IrType::Struct(_, _))
        | (_, IrType::Function(_, _))
        | (_, IrType::Quantum) => false,

        _ => false,
    };

    if valid {
        Ok(())
    } else {
        Err(JitError::TypeMismatch {
            expected: format!("{ty:?}"),
            actual: format!("{value:?}"),
        })
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn module_with_function(function: IrFunction) -> IrModule {
        let mut module = IrModule::new("jit_test");
        module.add_function(function);
        module
    }

    fn function_returning(value: i64) -> IrFunction {
        IrFunction::new(
            "main",
            Vec::new(),
            IrType::I64,
            vec![IrInstruction::Ret(Some(IrValue::ConstInt(
                value,
                IrType::I64,
            )))],
            false,
        )
    }

    #[test]
    fn engine_does_not_return_fake_42() {
        let module = module_with_function(function_returning(123));

        let engine = JitEngine::new(0);

        let result = engine
            .execute(&module, "main")
            .expect("execution should succeed");

        assert_eq!(result, 123);
    }

    #[test]
    fn missing_entry_point_is_rejected() {
        let module = module_with_function(function_returning(1));

        let engine = JitEngine::new(0);

        let result = engine.execute(&module, "missing");

        assert!(result.is_err());
    }

    #[test]
    fn integer_arithmetic_is_executed() {
        let register = IrRegister::new("result", IrType::I64);

        let function = IrFunction::new(
            "main",
            Vec::new(),
            IrType::I64,
            vec![
                IrInstruction::Add(
                    register.clone(),
                    IrValue::ConstInt(20, IrType::I64),
                    IrValue::ConstInt(22, IrType::I64),
                ),
                IrInstruction::Ret(Some(IrValue::Reg(register))),
            ],
            false,
        );

        let module = module_with_function(function);
        let engine = JitEngine::new(0);

        assert_eq!(engine.execute(&module, "main").unwrap(), 42);
    }

    #[test]
    fn function_calls_work() {
        let register = IrRegister::new("result", IrType::I64);

        let add_one = IrFunction::new(
            "add_one",
            vec![("x".to_string(), IrType::I64)],
            IrType::I64,
            vec![IrInstruction::Add(
                register.clone(),
                IrValue::Reg(IrRegister::new("x", IrType::I64)),
                IrValue::ConstInt(1, IrType::I64),
            ),
            IrInstruction::Ret(Some(IrValue::Reg(register)))],
            false,
        );

        let main = IrFunction::new(
            "main",
            Vec::new(),
            IrType::I64,
            vec![
                IrInstruction::Call(
                    Some(register.clone()),
                    "add_one".to_string(),
                    vec![IrValue::ConstInt(41, IrType::I64)],
                ),
                IrInstruction::Ret(Some(IrValue::Reg(register))),
            ],
            false,
        );

        let mut module = IrModule::new("call_test");
        module.add_function(add_one);
        module.add_function(main);

        let engine = JitEngine::new(0);

        assert_eq!(engine.execute(&module, "main").unwrap(), 42);
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let register = IrRegister::new("result", IrType::I64);

        let function = IrFunction::new(
            "main",
            Vec::new(),
            IrType::I64,
            vec![
                IrInstruction::Div(
                    register.clone(),
                    IrValue::ConstInt(1, IrType::I64),
                    IrValue::ConstInt(0, IrType::I64),
                ),
                IrInstruction::Ret(Some(IrValue::Reg(register))),
            ],
            false,
        );

        let module = module_with_function(function);
        let engine = JitEngine::new(0);

        assert!(matches!(
            engine.execute_result(&module, "main", &[]),
            Err(JitError::DivisionByZero)
        ));
    }

    #[test]
    fn instruction_limit_stops_infinite_loop() {
        let function = IrFunction::new(
            "main",
            Vec::new(),
            IrType::I64,
            vec![
                IrInstruction::Label("loop".to_string()),
                IrInstruction::Jump("loop".to_string()),
            ],
            false,
        );

        let module = module_with_function(function);

        let config = JitConfig {
            max_instructions: 10,
            ..JitConfig::default()
        };

        let engine =
            JitEngine::with_config(config).expect("configuration is valid");

        let result = engine.execute_result(&module, "main", &[]);

        assert!(matches!(
            result,
            Err(JitError::InstructionLimitExceeded { limit: 10 })
        ));
    }

    #[test]
    fn external_functions_are_blocked_by_default() {
        let external = IrFunction::new(
            "external",
            Vec::new(),
            IrType::I64,
            Vec::new(),
            true,
        );

        let module = module_with_function(external);

        let engine = JitEngine::new(0);

        let result = engine.execute_result(&module, "external", &[]);

        assert!(matches!(
            result,
            Err(JitError::ExternalFunction(_))
        ));
    }

    #[test]
    fn comparisons_produce_boolean_values() {
        let register = IrRegister::new("cmp", IrType::Bool);

        let function = IrFunction::new(
            "main",
            Vec::new(),
            IrType::Bool,
            vec![
                IrInstruction::Cmp(
                    register.clone(),
                    CmpOp::Gt,
                    IrValue::ConstInt(10, IrType::I64),
                    IrValue::ConstInt(5, IrType::I64),
                ),
                IrInstruction::Ret(Some(IrValue::Reg(register))),
            ],
            false,
        );

        let module = module_with_function(function);
        let engine = JitEngine::new(0);

        let result = engine
            .execute_result(&module, "main", &[])
            .expect("comparison should execute");

        assert_eq!(result.value, JitValue::Bool(true));
    }

    #[test]
    fn configuration_rejects_zero_instruction_limit() {
        let config = JitConfig {
            max_instructions: 0,
            ..JitConfig::default()
        };

        assert!(JitEngine::with_config(config).is_err());
    }
}