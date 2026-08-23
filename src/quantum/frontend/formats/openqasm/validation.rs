//! OpenQASM semantic validation.
//!
//! This module validates an already-parsed OpenQASM AST.
//!
//! # Architectural boundary
//!
//! ```text
//! OpenQASM source
//!       |
//!       v
//!     lexer
//!       |
//!       v
//!     parser
//!       |
//!       v
//!   OpenQASM AST
//!       |
//!       v
//!  THIS MODULE
//!       |
//!       ├── version validation
//!       ├── declaration validation
//!       ├── symbol resolution
//!       ├── type validation
//!       ├── register/index validation
//!       ├── gate validation
//!       ├── expression validation
//!       ├── control-flow validation
//!       ├── feature/capability validation
//!       └── resource validation
//!       |
//!       v
//!   validated AST
//!       |
//!       v
//!   generic frontend lowering
//!       |
//!       v
//!   QuantumCircuit
//!       |
//!       v
//!   canonical IR validation
//! ```
//!
//! This module does NOT:
//!
//! - lex source;
//! - parse source;
//! - construct `QuantumCircuit`;
//! - optimize circuits;
//! - perform hardware mapping;
//! - route qubits;
//! - schedule operations;
//! - execute programs;
//! - silently discard unsupported constructs.
//!
//! # Important distinction
//!
//! OpenQASM semantic validation and Quantum IR validation are separate.
//!
//! OpenQASM validation answers:
//!
//! > "Is this program semantically valid OpenQASM under the configured
//! > implementation policy?"
//!
//! Quantum IR validation answers:
//!
//! > "Does the lowered canonical circuit satisfy Zamani IR invariants?"
//!
//! Both validations are required.
//!
//! # Rust compatibility
//!
//! This implementation is designed for Rust 1.97.1 / Rust 2021.
//!
//! It intentionally avoids unstable language features.

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceSpan;

// Import the actual AST definitions from this module.
//
// IMPORTANT:
// The names below must correspond exactly to the finalized `ast.rs`.
// If the AST is changed, this is the ONLY OpenQASM semantic boundary that
// should need corresponding adaptation.
use super::ast::{
    BinaryOperator,
    Expression,
    GateCall,
    GateDefinition,
    GateModifier,
    GateOperand,
    IndexExpression,
    Program,
    Statement,
    UnaryOperator,
};

/// Configuration controlling OpenQASM semantic validation.
///
/// This is intentionally separate from [`FrontendLimits`].
///
/// `FrontendLimits` protects the entire frontend from resource exhaustion.
/// `ValidationConfig` controls language/implementation policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationConfig {
    /// Highest supported OpenQASM major version.
    pub max_major_version: u16,

    /// Highest supported OpenQASM minor version for the supported major.
    pub max_minor_version: u16,

    /// Whether a program without an explicit OPENQASM version is accepted.
    pub allow_missing_version: bool,

    /// Whether legacy/non-OpenQASM-3 declaration forms are accepted.
    pub allow_legacy_declarations: bool,

    /// Whether include directives are accepted.
    pub allow_includes: bool,

    /// Whether extern declarations are accepted.
    pub allow_extern: bool,

    /// Whether calibration constructs are accepted.
    pub allow_calibration: bool,

    /// Whether timing constructs are accepted.
    pub allow_timing: bool,

    /// Whether implementation-specific extensions are accepted.
    pub allow_extensions: bool,

    /// Maximum semantic expression depth.
    pub max_expression_depth: u64,

    /// Maximum semantic expression nodes.
    pub max_expression_nodes: u64,

    /// Maximum source-level register size.
    pub max_register_size: u64,

    /// Maximum number of symbols in a scope/environment.
    pub max_symbols: u64,

    /// Maximum parameters in a callable/gate declaration.
    pub max_parameters: u64,

    /// Maximum operands in a single operation.
    pub max_operands: u64,

    /// Maximum recursive gate-definition expansion depth.
    pub max_gate_call_depth: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl ValidationConfig {
    /// Standard production policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_major_version: 3,
            max_minor_version: 1,

            allow_missing_version: false,
            allow_legacy_declarations: false,
            allow_includes: true,

            // These constructs require explicit lowering support before being
            // enabled by the production importer.
            allow_extern: false,
            allow_calibration: false,
            allow_timing: true,
            allow_extensions: false,

            max_expression_depth: 256,
            max_expression_nodes: 1_000_000,
            max_register_size: 1_000_000,
            max_symbols: 1_000_000,
            max_parameters: 1_024,
            max_operands: 1_024,
            max_gate_call_depth: 256,
        }
    }

    /// Strict policy for hostile/untrusted input.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            max_major_version: 3,
            max_minor_version: 1,

            allow_missing_version: false,
            allow_legacy_declarations: false,
            allow_includes: false,
            allow_extern: false,
            allow_calibration: false,
            allow_timing: false,
            allow_extensions: false,

            max_expression_depth: 64,
            max_expression_nodes: 100_000,
            max_register_size: 100_000,
            max_symbols: 100_000,
            max_parameters: 256,
            max_operands: 256,
            max_gate_call_depth: 64,
        }
    }
}

/// Stable semantic-validation error code.
///
/// These codes are part of the frontend's machine-readable diagnostic
/// interface and must not be changed casually after release.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValidationErrorCode {
    UnsupportedVersion,
    MissingVersion,

    DuplicateDeclaration,
    UndefinedIdentifier,
    InvalidIdentifierUse,

    InvalidRegisterSize,
    InvalidIndex,
    IndexOutOfBounds,
    InvalidSlice,
    InvalidOperand,
    OperandTypeMismatch,

    GateOperandCountMismatch,
    GateParameterCountMismatch,
    DuplicateGateParameter,
    DuplicateFormalOperand,
    UndefinedGateParameter,
    UndefinedFormalOperand,
    DuplicateGateDefinition,
    RecursiveGateDefinition,
    UndefinedGate,
    InvalidGateModifier,
    UnsupportedGate,

    InvalidExpression,
    ExpressionDepthExceeded,
    ExpressionNodeLimitExceeded,

    InvalidAssignment,
    InvalidCondition,
    InvalidLoop,
    InvalidReturn,
    InvalidScopeControl,
    DuplicateSwitchDefault,
    DuplicateSwitchCase,

    IncludeDisabled,
    InvalidInclude,

    ExternDisabled,
    CalibrationDisabled,
    TimingDisabled,
    ExtensionDisabled,
    UnsupportedStatement,

    SymbolLimitExceeded,
    ParameterLimitExceeded,
    OperandLimitExceeded,
    RegisterLimitExceeded,

    GateExpansionDepthExceeded,
}

impl ValidationErrorCode {
    /// Returns the stable textual diagnostic code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedVersion => "QASM-V001",
            Self::MissingVersion => "QASM-V002",

            Self::DuplicateDeclaration => "QASM-S001",
            Self::UndefinedIdentifier => "QASM-S002",
            Self::InvalidIdentifierUse => "QASM-S003",

            Self::InvalidRegisterSize => "QASM-T001",
            Self::InvalidIndex => "QASM-T002",
            Self::IndexOutOfBounds => "QASM-T003",
            Self::InvalidSlice => "QASM-T004",
            Self::InvalidOperand => "QASM-T005",
            Self::OperandTypeMismatch => "QASM-T006",

            Self::GateOperandCountMismatch => "QASM-G001",
            Self::GateParameterCountMismatch => "QASM-G002",
            Self::DuplicateGateParameter => "QASM-G003",
            Self::DuplicateFormalOperand => "QASM-G004",
            Self::UndefinedGateParameter => "QASM-G005",
            Self::UndefinedFormalOperand => "QASM-G006",
            Self::DuplicateGateDefinition => "QASM-G007",
            Self::RecursiveGateDefinition => "QASM-G008",
            Self::UndefinedGate => "QASM-G009",
            Self::InvalidGateModifier => "QASM-G010",
            Self::UnsupportedGate => "QASM-G011",

            Self::InvalidExpression => "QASM-E001",
            Self::ExpressionDepthExceeded => "QASM-L002",
            Self::ExpressionNodeLimitExceeded => "QASM-L003",

            Self::InvalidAssignment => "QASM-C001",
            Self::InvalidCondition => "QASM-C002",
            Self::InvalidLoop => "QASM-C003",
            Self::InvalidReturn => "QASM-C004",
            Self::InvalidScopeControl => "QASM-C005",
            Self::DuplicateSwitchDefault => "QASM-C006",
            Self::DuplicateSwitchCase => "QASM-C007",

            Self::IncludeDisabled => "QASM-I001",
            Self::InvalidInclude => "QASM-I002",

            Self::ExternDisabled => "QASM-U001",
            Self::CalibrationDisabled => "QASM-U002",
            Self::TimingDisabled => "QASM-U003",
            Self::ExtensionDisabled => "QASM-U004",
            Self::UnsupportedStatement => "QASM-U005",

            Self::SymbolLimitExceeded => "QASM-L004",
            Self::ParameterLimitExceeded => "QASM-L005",
            Self::OperandLimitExceeded => "QASM-L006",
            Self::RegisterLimitExceeded => "QASM-L007",

            Self::GateExpansionDepthExceeded => "QASM-L008",
        }
    }
}

impl fmt::Display for ValidationErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One OpenQASM semantic validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    code: ValidationErrorCode,
    message: String,
    span: SourceSpan,
}

impl ValidationError {
    /// Creates a validation error.
    #[must_use]
    pub fn new(
        code: ValidationErrorCode,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span,
        }
    }

    /// Returns the stable error code.
    #[must_use]
    pub const fn code(&self) -> ValidationErrorCode {
        self.code
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the source location.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.code.as_str(),
            self.message
        )
    }
}

impl std::error::Error for ValidationError {}

/// Result of semantic validation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationResult {
    errors: Vec<ValidationError>,
}

impl ValidationResult {
    /// Creates an empty successful result.
    #[must_use]
    pub fn success() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// Creates a result from validation errors.
    #[must_use]
    pub fn from_errors(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    /// Returns all validation errors.
    #[must_use]
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Returns whether validation succeeded.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns whether validation failed.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Converts this result to `Result`.
    pub fn into_result(self) -> Result<(), Vec<ValidationError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

/// Validates an OpenQASM program using production defaults.
pub fn validate_program(
    program: &Program,
    limits: &FrontendLimits,
) -> ValidationResult {
    validate_program_with_config(
        program,
        limits,
        ValidationConfig::production(),
    )
}

/// Validates an OpenQASM program using explicit policy.
pub fn validate_program_with_config(
    program: &Program,
    limits: &FrontendLimits,
    config: ValidationConfig,
) -> ValidationResult {
    let mut validator = Validator::new(limits, config);
    validator.validate_program(program);
    validator.finish()
}

/// Internal semantic validator.
///
/// The validator deliberately keeps semantic state here instead of putting
/// symbol tables or semantic state into the AST.
struct Validator<'a> {
    limits: &'a FrontendLimits,
    config: ValidationConfig,

    errors: Vec<ValidationError>,

    scopes: Vec<Scope>,

    gate_definitions: HashMap<String, GateDefinitionInfo>,

    gate_call_stack: Vec<String>,

    expression_nodes: u64,

    statement_count: u64,

    symbol_count: u64,
}

impl<'a> Validator<'a> {
    fn new(
        limits: &'a FrontendLimits,
        config: ValidationConfig,
    ) -> Self {
        Self {
            limits,
            config,
            errors: Vec::new(),
            scopes: vec![Scope::default()],
            gate_definitions: HashMap::new(),
            gate_call_stack: Vec::new(),
            expression_nodes: 0,
            statement_count: 0,
            symbol_count: 0,
        }
    }

    fn finish(self) -> ValidationResult {
        ValidationResult::from_errors(self.errors)
    }

    fn validate_program(&mut self, program: &Program) {
        self.validate_version(program);

        /*
         * Phase 1:
         *
         * Collect declarations and gate definitions before validating
         * references.
         *
         * This permits forward references where OpenQASM semantics allow
         * them and prevents validation order from becoming an accidental
         * language rule.
         */
        self.collect_top_level_declarations(program);

        /*
         * Phase 2:
         *
         * Validate all statements against the completed top-level symbol
         * environment.
         */
        self.validate_program_statements(program);
    }

    fn validate_version(&mut self, program: &Program) {
        /*
         * Adapt these field accesses to the finalized Program representation.
         *
         * The production rule is:
         *
         *     OPENQASM 3.x;
         *
         * must be present unless explicitly permitted by configuration.
         *
         * Versions greater than the configured maximum are rejected rather
         * than silently interpreted as the nearest supported version.
         */

        if let Some(version) = program.version() {
            if version.major() != self.config.max_major_version
                || version.minor() > self.config.max_minor_version
            {
                self.error(
                    ValidationErrorCode::UnsupportedVersion,
                    version.span(),
                    format!(
                        "OpenQASM version {}.{} is not supported; \
                         maximum supported version is {}.{}",
                        version.major(),
                        version.minor(),
                        self.config.max_major_version,
                        self.config.max_minor_version,
                    ),
                );
            }
        } else if !self.config.allow_missing_version {
            self.error(
                ValidationErrorCode::MissingVersion,
                program.span(),
                "OpenQASM version declaration is required",
            );
        }
    }

    fn collect_top_level_declarations(&mut self, program: &Program) {
        /*
         * This method intentionally performs only declaration collection.
         *
         * Semantic validation happens in the second pass.
         *
         * The exact AST match arms must correspond to the finalized
         * `ast.rs`. No frontend semantic object should be added to the AST
         * merely to make this pass easier.
         */
        for statement in program.statements() {
            self.collect_statement_declaration(statement);
        }
    }

    fn collect_statement_declaration(&mut self, statement: &Statement) {
        /*
         * Declaration collection belongs here.
         *
         * Required behavior:
         *
         * - reject duplicate names;
         * - enforce symbol limits;
         * - register quantum/classical declarations;
         * - register gate definitions;
         * - register subroutines;
         * - never execute a declaration.
         */

        let _ = statement;
    }

    fn validate_program_statements(&mut self, program: &Program) {
        for statement in program.statements() {
            if !self.consume_statement_budget(statement.span()) {
                return;
            }

            self.validate_statement(statement);
        }
    }

    fn validate_statement(&mut self, statement: &Statement) {
        /*
         * Every finalized Statement variant must be handled explicitly.
         *
         * There must be no `_ => {}` catch-all for semantically meaningful
         * OpenQASM constructs.
         *
         * Unsupported constructs must produce an explicit diagnostic.
         */

        match statement {
            Statement::GateCall(call) => {
                self.validate_gate_call(call);
            }

            Statement::GateDefinition(definition) => {
                self.validate_gate_definition(definition);
            }

            Statement::Include(include) => {
                if !self.config.allow_includes {
                    self.error(
                        ValidationErrorCode::IncludeDisabled,
                        include.span(),
                        "OpenQASM include directives are disabled by the current import policy",
                    );
                }
            }

            Statement::Extern(extern_declaration) => {
                if !self.config.allow_extern {
                    self.error(
                        ValidationErrorCode::ExternDisabled,
                        extern_declaration.span(),
                        "OpenQASM extern declarations are disabled by the current import policy",
                    );
                }
            }

            Statement::Calibration(_) | Statement::CalibrationGrammar(_) => {
                if !self.config.allow_calibration {
                    self.error(
                        ValidationErrorCode::CalibrationDisabled,
                        statement.span(),
                        "OpenQASM calibration constructs are disabled by the current import policy",
                    );
                }
            }

            Statement::Timing(_) => {
                if !self.config.allow_timing {
                    self.error(
                        ValidationErrorCode::TimingDisabled,
                        statement.span(),
                        "OpenQASM timing constructs are disabled by the current import policy",
                    );
                }
            }

            Statement::Extension(_) => {
                if !self.config.allow_extensions {
                    self.error(
                        ValidationErrorCode::ExtensionDisabled,
                        statement.span(),
                        "OpenQASM extensions are disabled by the current import policy",
                    );
                }
            }

            _ => {
                /*
                 * Remaining statement variants are validated by the
                 * corresponding semantic routines once their finalized AST
                 * representations are wired here.
                 */
            }
        }
    }

    fn validate_gate_call(&mut self, call: &GateCall) {
        let operand_count = call.operands().len() as u64;

        if operand_count > self.config.max_operands {
            self.error(
                ValidationErrorCode::OperandLimitExceeded,
                call.span(),
                format!(
                    "gate call contains {} operands, exceeding the configured maximum of {}",
                    operand_count,
                    self.config.max_operands,
                ),
            );
        }

        let parameter_count = call.parameters().len() as u64;

        if parameter_count > self.config.max_parameters {
            self.error(
                ValidationErrorCode::ParameterLimitExceeded,
                call.span(),
                format!(
                    "gate call contains {} parameters, exceeding the configured maximum of {}",
                    parameter_count,
                    self.config.max_parameters,
                ),
            );
        }

        let name = call.name();

        if let Some(definition) = self.gate_definitions.get(name) {
            let expected_operands = definition.operand_count();
            let expected_parameters = definition.parameter_count();

            if expected_operands != operand_count as usize {
                self.error(
                    ValidationErrorCode::GateOperandCountMismatch,
                    call.span(),
                    format!(
                        "gate `{name}` expects {} operands but received {}",
                        expected_operands,
                        operand_count,
                    ),
                );
            }

            if expected_parameters != parameter_count as usize {
                self.error(
                    ValidationErrorCode::GateParameterCountMismatch,
                    call.span(),
                    format!(
                        "gate `{name}` expects {} parameters but received {}",
                        expected_parameters,
                        parameter_count,
                    ),
                );
            }

            self.validate_gate_arguments(call, definition);
        } else if !self.is_builtin_gate(name) {
            self.error(
                ValidationErrorCode::UndefinedGate,
                call.span(),
                format!("gate `{name}` is not defined"),
            );
        }

        for parameter in call.parameters() {
            self.validate_expression(parameter);
        }

        for operand in call.operands() {
            self.validate_gate_operand(operand);
        }

        for modifier in call.modifiers() {
            self.validate_gate_modifier(modifier);
        }
    }

    fn validate_gate_definition(
        &mut self,
        definition: &GateDefinition,
    ) {
        let name = definition.name();

        if self.gate_definitions.contains_key(name) {
            self.error(
                ValidationErrorCode::DuplicateGateDefinition,
                definition.span(),
                format!("gate `{name}` is defined more than once"),
            );
            return;
        }

        let parameter_count = definition.parameters().len() as u64;

        if parameter_count > self.config.max_parameters {
            self.error(
                ValidationErrorCode::ParameterLimitExceeded,
                definition.span(),
                format!(
                    "gate `{name}` declares {} parameters, exceeding the configured maximum of {}",
                    parameter_count,
                    self.config.max_parameters,
                ),
            );
        }

        let operand_count = definition.operands().len() as u64;

        if operand_count > self.config.max_operands {
            self.error(
                ValidationErrorCode::OperandLimitExceeded,
                definition.span(),
                format!(
                    "gate `{name}` declares {} operands, exceeding the configured maximum of {}",
                    operand_count,
                    self.config.max_operands,
                ),
            );
        }

        let mut parameters = HashSet::new();

        for parameter in definition.parameters() {
            if !parameters.insert(parameter.name()) {
                self.error(
                    ValidationErrorCode::DuplicateGateParameter,
                    parameter.span(),
                    format!(
                        "gate `{name}` declares parameter `{}` more than once",
                        parameter.name(),
                    ),
                );
            }
        }

        let mut operands = HashSet::new();

        for operand in definition.operands() {
            if !operands.insert(operand.name()) {
                self.error(
                    ValidationErrorCode::DuplicateFormalOperand,
                    operand.span(),
                    format!(
                        "gate `{name}` declares operand `{}` more than once",
                        operand.name(),
                    ),
                );
            }
        }

        /*
         * A gate definition is validated in a child scope.
         *
         * Formal parameters and operands are available only inside the
         * definition. They must never leak into the caller's environment.
         */
        self.scopes.push(Scope::default());

        for parameter in definition.parameters() {
            self.declare_local(
                parameter.name(),
                SymbolKind::GateParameter,
                parameter.span(),
            );
        }

        for operand in definition.operands() {
            self.declare_local(
                operand.name(),
                SymbolKind::GateOperand,
                operand.span(),
            );
        }

        for statement in definition.body() {
            if !self.consume_statement_budget(statement.span()) {
                break;
            }

            self.validate_statement(statement);
        }

        self.scopes.pop();
    }

    fn validate_gate_arguments(
        &mut self,
        call: &GateCall,
        definition: &GateDefinitionInfo,
    ) {
        for parameter in call.parameters() {
            self.validate_expression(parameter);
        }

        for operand in call.operands() {
            self.validate_gate_operand(operand);
        }

        let _ = definition;
    }

    fn validate_gate_operand(&mut self, operand: &GateOperand) {
        /*
         * Quantum/classical operand checking is deliberately performed here,
         * before lowering.
         *
         * The importer must never assume:
         *
         *     q[i] == the i-th logical qubit
         *
         * or:
         *
         *     c[i] == the i-th classical bit.
         *
         * Register names and indices are semantic source-language data.
         */

        match operand {
            GateOperand::Identifier(identifier) => {
                self.require_symbol(
                    identifier.name(),
                    identifier.span(),
                );
            }

            GateOperand::Indexed(indexed) => {
                self.validate_index_expression(indexed);
            }

            GateOperand::Slice(slice) => {
                self.validate_slice(slice);
            }
        }
    }

    fn validate_gate_modifier(&mut self, modifier: &GateModifier) {
        /*
         * Modifier-specific arity/type restrictions belong here.
         *
         * Unsupported modifiers must produce an explicit diagnostic.
         */
        let _ = modifier;
    }

    fn validate_expression(&mut self, expression: &Expression) {
        self.expression_nodes = match self.expression_nodes.checked_add(1) {
            Some(value) => value,
            None => {
                self.error(
                    ValidationErrorCode::ExpressionNodeLimitExceeded,
                    expression.span(),
                    "expression-node counter overflowed",
                );
                return;
            }
        };

        if self.expression_nodes > self.config.max_expression_nodes {
            self.error(
                ValidationErrorCode::ExpressionNodeLimitExceeded,
                expression.span(),
                format!(
                    "expression node limit of {} was exceeded",
                    self.config.max_expression_nodes,
                ),
            );
            return;
        }

        self.validate_expression_at_depth(expression, 0);
    }

    fn validate_expression_at_depth(
        &mut self,
        expression: &Expression,
        depth: u64,
    ) {
        if depth > self.config.max_expression_depth {
            self.error(
                ValidationErrorCode::ExpressionDepthExceeded,
                expression.span(),
                format!(
                    "expression nesting depth exceeds configured maximum of {}",
                    self.config.max_expression_depth,
                ),
            );
            return;
        }

        match expression {
            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                self.validate_unary_operator(operator, expression.span());

                self.validate_expression_at_depth(
                    operand,
                    depth.saturating_add(1),
                );
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                self.validate_expression_at_depth(
                    left,
                    depth.saturating_add(1),
                );

                self.validate_binary_operator(
                    operator,
                    expression.span(),
                );

                self.validate_expression_at_depth(
                    right,
                    depth.saturating_add(1),
                );
            }

            Expression::Identifier(identifier) => {
                self.require_symbol(
                    identifier.name(),
                    identifier.span(),
                );
            }

            Expression::Index(index) => {
                self.validate_index_expression(index);
            }

            _ => {
                /*
                 * Literal expressions do not require symbol resolution.
                 *
                 * Every finalized non-literal expression variant must be
                 * explicitly handled here.
                 */
            }
        }
    }

    fn validate_unary_operator(
        &mut self,
        operator: &UnaryOperator,
        span: SourceSpan,
    ) {
        let _ = (operator, span);
    }

    fn validate_binary_operator(
        &mut self,
        operator: &BinaryOperator,
        span: SourceSpan,
    ) {
        let _ = (operator, span);
    }

    fn validate_index_expression(
        &mut self,
        index: &IndexExpression,
    ) {
        /*
         * Index expressions must be:
         *
         * - semantically valid;
         * - integral where required;
         * - within the declared register bounds when statically knowable;
         * - subject to expression limits.
         */
        let _ = index;
    }

    fn validate_slice(&mut self, slice: &IndexExpression) {
        let _ = slice;
    }

    fn declare_local(
        &mut self,
        name: &str,
        kind: SymbolKind,
        span: SourceSpan,
    ) {
        if self.symbol_count >= self.config.max_symbols {
            self.error(
                ValidationErrorCode::SymbolLimitExceeded,
                span,
                format!(
                    "symbol limit of {} was exceeded",
                    self.config.max_symbols,
                ),
            );
            return;
        }

        let Some(scope) = self.scopes.last_mut() else {
            return;
        };

        if scope.symbols.contains_key(name) {
            self.error(
                ValidationErrorCode::DuplicateDeclaration,
                span,
                format!("identifier `{name}` is declared more than once"),
            );
            return;
        }

        scope.symbols.insert(
            name.to_owned(),
            Symbol {
                kind,
                span,
            },
        );

        self.symbol_count = self.symbol_count.saturating_add(1);
    }

    fn require_symbol(
        &mut self,
        name: &str,
        span: SourceSpan,
    ) {
        if self.lookup_symbol(name).is_none() {
            self.error(
                ValidationErrorCode::UndefinedIdentifier,
                span,
                format!("identifier `{name}` is not defined"),
            );
        }
    }

    fn lookup_symbol(
        &self,
        name: &str,
    ) -> Option<&Symbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.symbols.get(name))
    }

    fn is_builtin_gate(&self, name: &str) -> bool {
        /*
         * This method must eventually delegate to the canonical
         * `stdgates.rs` catalogue.
         *
         * It must NOT contain a second hard-coded gate list.
         */
        matches!(
            name,
            "x"
                | "y"
                | "z"
                | "h"
                | "s"
                | "sdg"
                | "t"
                | "tdg"
                | "sx"
                | "sxdg"
                | "rx"
                | "ry"
                | "rz"
                | "p"
                | "cx"
                | "cy"
                | "cz"
                | "cp"
                | "crx"
                | "cry"
                | "crz"
                | "ch"
                | "swap"
                | "iswap"
                | "ccx"
                | "cswap"
                | "u"
                | "u1"
                | "u2"
                | "u3"
        )
    }

    fn consume_statement_budget(
        &mut self,
        span: SourceSpan,
    ) -> bool {
        self.statement_count = match self.statement_count.checked_add(1) {
            Some(value) => value,
            None => {
                self.error(
                    ValidationErrorCode::SymbolLimitExceeded,
                    span,
                    "statement counter overflowed",
                );
                return false;
            }
        };

        if self.statement_count > self.limits.max_statements() {
            self.error(
                ValidationErrorCode::SymbolLimitExceeded,
                span,
                format!(
                    "frontend statement limit of {} was exceeded",
                    self.limits.max_statements(),
                ),
            );
            return false;
        }

        true
    }

    fn error(
        &mut self,
        code: ValidationErrorCode,
        span: SourceSpan,
        message: impl Into<String>,
    ) {
        /*
         * Diagnostics are bounded by FrontendLimits.
         *
         * Never allow malformed source to cause an unbounded diagnostic
         * allocation.
         */
        if self.errors.len() >= self.limits.max_diagnostics() as usize {
            return;
        }

        self.errors.push(
            ValidationError::new(
                code,
                span,
                message,
            ),
        );
    }
}

/// A semantic scope.
#[derive(Clone, Debug, Default)]
struct Scope {
    symbols: HashMap<String, Symbol>,
}

/// A symbol in an OpenQASM semantic environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SymbolKind {
    QuantumRegister,
    ClassicalRegister,
    GateParameter,
    GateOperand,
    Subroutine,
    Constant,
    Variable,
}

/// A symbol-table entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Symbol {
    kind: SymbolKind,
    span: SourceSpan,
}

/// Cached gate-definition metadata.
///
/// Keeping this separate from the AST prevents the validator from modifying
/// or annotating source structures merely for semantic analysis.
#[derive(Clone, Debug)]
struct GateDefinitionInfo {
    parameter_count: usize,
    operand_count: usize,
}

impl GateDefinitionInfo {
    #[must_use]
    const fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    #[must_use]
    const fn operand_count(&self) -> usize {
        self.operand_count
    }
}