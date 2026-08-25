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
//!   THIS MODULE
//!       |
//!       ├── version validation
//!       ├── declaration validation
//!       ├── scope validation
//!       ├── symbol resolution
//!       ├── type validation
//!       ├── register/index validation
//!       ├── gate validation
//!       ├── modifier validation
//!       ├── expression validation
//!       ├── control-flow validation
//!       ├── measurement validation
//!       ├── include validation
//!       ├── feature-policy validation
//!       └── resource validation
//!       |
//!       v
//!   semantically validated AST
//!       |
//!       v
//!   generic frontend lowering
//!       |
//!       v
//!   QuantumCircuit
//! ```
//!
//! # Production invariants
//!
//! 1. No filesystem access.
//! 2. No network access.
//! 3. No process execution.
//! 4. No QPU/hardware access.
//! 5. No calibration execution.
//! 6. No silent deletion of unsupported syntax.
//! 7. Every unbounded semantic operation is subject to FrontendLimits.
//! 8. Declaration-before-use is enforced.
//! 9. OpenQASM scope visibility is enforced.
//! 10. The selected standard-library version is the program version.
//! 11. Validation never constructs canonical Quantum IR.
//! 12. Validation errors are deterministic and bounded.
//!
//! Rust compatibility: Rust 1.97.1 / Rust 2021.

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceSpan;

use super::ast::{
    AssignmentOperator,
    AssignmentValue,
    BinaryOperator,
    Designator,
    Expression,
    ForIterable,
    GateCall,
    GateDefinition,
    GateModifier,
    GateOperand,
    IndexExpression,
    IntegerRadix,
    OldStyleDeclarationKind,
    OpenQasmVersion,
    Program,
    QuantumType,
    ReturnValue,
    ScalarType,
    Statement,
    StatementOrScope,
    SwitchCase,
    TypeSpecifier,
    UnaryOperator,
};

use super::stdgates::{
    lookup as lookup_standard_gate,
    StandardGate,
    STANDARD_LIBRARY_INCLUDE,
};

// ============================================================================
// Configuration
// ============================================================================

/// Configuration controlling OpenQASM semantic acceptance.
///
/// This is deliberately separate from [`FrontendLimits`].
///
/// `FrontendLimits` answers:
///
/// > How much work may the frontend perform?
///
/// `ValidationConfig` answers:
///
/// > Which language features may this frontend accept?
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationConfig {
    pub max_major_version: u16,
    pub max_minor_version: u16,

    pub allow_missing_version: bool,
    pub allow_legacy_declarations: bool,
    pub allow_includes: bool,

    pub allow_extern: bool,
    pub allow_calibration: bool,
    pub allow_timing: bool,
    pub allow_extensions: bool,
    pub allow_physical_qubits: bool,

    pub allow_pragmas: bool,
    pub allow_annotations: bool,

    /// Permit non-constant quantum-register indexes.
    ///
    /// OpenQASM permits implementations to impose a stricter rule here.
    /// Zamani production mode defaults to false because the canonical
    /// logical-quantum frontend is intended to remain statically bounded.
    pub allow_runtime_quantum_index: bool,

    pub max_expression_depth: u64,
    pub max_expression_nodes: u64,
    pub max_register_size: u64,
    pub max_symbols: u64,
    pub max_parameters: u64,
    pub max_operands: u64,
    pub max_gate_call_depth: u64,
    pub max_statements: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl ValidationConfig {
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_major_version: 3,
            max_minor_version: 1,

            allow_missing_version: false,
            allow_legacy_declarations: false,
            allow_includes: true,

            allow_extern: false,
            allow_calibration: false,
            allow_timing: true,
            allow_extensions: false,
            allow_physical_qubits: false,

            allow_pragmas: true,
            allow_annotations: true,

            allow_runtime_quantum_index: false,

            max_expression_depth: 256,
            max_expression_nodes: 1_000_000,
            max_register_size: 1_000_000,
            max_symbols: 1_000_000,
            max_parameters: 1_024,
            max_operands: 1_024,
            max_gate_call_depth: 256,
            max_statements: 2_000_000,
        }
    }

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
            allow_physical_qubits: false,

            allow_pragmas: false,
            allow_annotations: false,

            allow_runtime_quantum_index: false,

            max_expression_depth: 64,
            max_expression_nodes: 100_000,
            max_register_size: 100_000,
            max_symbols: 100_000,
            max_parameters: 256,
            max_operands: 256,
            max_gate_call_depth: 64,
            max_statements: 250_000,
        }
    }
}

// ============================================================================
// Error codes
// ============================================================================

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValidationErrorCode {
    UnsupportedVersion,
    MissingVersion,

    DuplicateDeclaration,
    UndefinedIdentifier,
    InvalidIdentifierUse,
    InvalidScope,
    InvalidGlobalDeclaration,

    InvalidRegisterSize,
    InvalidTypeSize,
    InvalidIndex,
    IndexOutOfBounds,
    InvalidSlice,
    EmptyIndexSet,
    DuplicateIndex,
    IndexTypeMismatch,

    InvalidOperand,
    OperandTypeMismatch,
    PhysicalQubitDisabled,
    RuntimeQuantumIndexDisabled,
    RegisterBroadcastMismatch,
    DuplicateQuantumOperand,

    GateOperandCountMismatch,
    GateParameterCountMismatch,
    DuplicateGateParameter,
    DuplicateFormalOperand,
    UndefinedGateParameter,
    UndefinedFormalOperand,
    DuplicateGateDefinition,
    RecursiveGateDefinition,
    GateForwardReference,
    UndefinedGate,
    UnsupportedGate,
    InvalidGateModifier,
    InvalidModifierCount,
    ModifierParameterNotConstant,

    InvalidExpression,
    ExpressionDepthExceeded,
    ExpressionNodeLimitExceeded,
    NonFiniteLiteral,
    InvalidNumericLiteral,
    DivisionByZero,

    InvalidAssignment,
    AssignmentTypeMismatch,
    InvalidAssignmentTarget,
    InvalidCondition,
    InvalidLoop,
    InvalidLoopVariable,
    InvalidReturn,
    ReturnOutsideSubroutine,
    InvalidScopeControl,
    BreakOutsideLoop,
    ContinueOutsideLoop,

    InvalidMeasurement,
    InvalidMeasurementSource,
    InvalidMeasurementDestination,

    IncludeDisabled,
    IncludeOutOfScope,
    InvalidInclude,
    StandardLibraryUnavailable,
    DuplicateInclude,

    ExternDisabled,
    CalibrationDisabled,
    TimingDisabled,
    ExtensionDisabled,
    PragmaDisabled,
    AnnotationDisabled,
    UnsupportedStatement,

    SymbolLimitExceeded,
    ParameterLimitExceeded,
    OperandLimitExceeded,
    RegisterLimitExceeded,
    StatementLimitExceeded,
    GateExpansionDepthExceeded,

    InvalidGateBody,
    InvalidGateBodyDeclaration,
    IndexedGateFormalOperand,
    GateBodyClassicalOperation,

    InvalidFunctionScope,
    InvalidGateScope,
}

impl ValidationErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedVersion => "QASM-V001",
            Self::MissingVersion => "QASM-V002",

            Self::DuplicateDeclaration => "QASM-S001",
            Self::UndefinedIdentifier => "QASM-S002",
            Self::InvalidIdentifierUse => "QASM-S003",
            Self::InvalidScope => "QASM-S004",
            Self::InvalidGlobalDeclaration => "QASM-S005",

            Self::InvalidRegisterSize => "QASM-T001",
            Self::InvalidTypeSize => "QASM-T002",
            Self::InvalidIndex => "QASM-T003",
            Self::IndexOutOfBounds => "QASM-T004",
            Self::InvalidSlice => "QASM-T005",
            Self::EmptyIndexSet => "QASM-T006",
            Self::DuplicateIndex => "QASM-T007",
            Self::IndexTypeMismatch => "QASM-T008",

            Self::InvalidOperand => "QASM-Q001",
            Self::OperandTypeMismatch => "QASM-Q002",
            Self::PhysicalQubitDisabled => "QASM-Q003",
            Self::RuntimeQuantumIndexDisabled => "QASM-Q004",
            Self::RegisterBroadcastMismatch => "QASM-Q005",
            Self::DuplicateQuantumOperand => "QASM-Q006",

            Self::GateOperandCountMismatch => "QASM-G001",
            Self::GateParameterCountMismatch => "QASM-G002",
            Self::DuplicateGateParameter => "QASM-G003",
            Self::DuplicateFormalOperand => "QASM-G004",
            Self::UndefinedGateParameter => "QASM-G005",
            Self::UndefinedFormalOperand => "QASM-G006",
            Self::DuplicateGateDefinition => "QASM-G007",
            Self::RecursiveGateDefinition => "QASM-G008",
            Self::GateForwardReference => "QASM-G009",
            Self::UndefinedGate => "QASM-G010",
            Self::UnsupportedGate => "QASM-G011",
            Self::InvalidGateModifier => "QASM-G012",
            Self::InvalidModifierCount => "QASM-G013",
            Self::ModifierParameterNotConstant => "QASM-G014",

            Self::InvalidExpression => "QASM-E001",
            Self::ExpressionDepthExceeded => "QASM-E002",
            Self::ExpressionNodeLimitExceeded => "QASM-E003",
            Self::NonFiniteLiteral => "QASM-E004",
            Self::InvalidNumericLiteral => "QASM-E005",
            Self::DivisionByZero => "QASM-E006",

            Self::InvalidAssignment => "QASM-C001",
            Self::AssignmentTypeMismatch => "QASM-C002",
            Self::InvalidAssignmentTarget => "QASM-C003",
            Self::InvalidCondition => "QASM-C004",
            Self::InvalidLoop => "QASM-C005",
            Self::InvalidLoopVariable => "QASM-C006",
            Self::InvalidReturn => "QASM-C007",
            Self::ReturnOutsideSubroutine => "QASM-C008",
            Self::InvalidScopeControl => "QASM-C009",
            Self::BreakOutsideLoop => "QASM-C010",
            Self::ContinueOutsideLoop => "QASM-C011",

            Self::InvalidMeasurement => "QASM-M001",
            Self::InvalidMeasurementSource => "QASM-M002",
            Self::InvalidMeasurementDestination => "QASM-M003",

            Self::IncludeDisabled => "QASM-I001",
            Self::IncludeOutOfScope => "QASM-I002",
            Self::InvalidInclude => "QASM-I003",
            Self::StandardLibraryUnavailable => "QASM-I004",
            Self::DuplicateInclude => "QASM-I005",

            Self::ExternDisabled => "QASM-U001",
            Self::CalibrationDisabled => "QASM-U002",
            Self::TimingDisabled => "QASM-U003",
            Self::ExtensionDisabled => "QASM-U004",
            Self::PragmaDisabled => "QASM-U005",
            Self::AnnotationDisabled => "QASM-U006",
            Self::UnsupportedStatement => "QASM-U007",

            Self::SymbolLimitExceeded => "QASM-L001",
            Self::ParameterLimitExceeded => "QASM-L002",
            Self::OperandLimitExceeded => "QASM-L003",
            Self::RegisterLimitExceeded => "QASM-L004",
            Self::StatementLimitExceeded => "QASM-L005",
            Self::GateExpansionDepthExceeded => "QASM-L006",

            Self::InvalidGateBody => "QASM-B001",
            Self::InvalidGateBodyDeclaration => "QASM-B002",
            Self::IndexedGateFormalOperand => "QASM-B003",
            Self::GateBodyClassicalOperation => "QASM-B004",

            Self::InvalidFunctionScope => "QASM-F001",
            Self::InvalidGateScope => "QASM-F002",
        }
    }
}

impl fmt::Display for ValidationErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Result
// ============================================================================

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    code: ValidationErrorCode,
    message: String,
    span: SourceSpan,
}

impl ValidationError {
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

    #[must_use]
    pub const fn code(&self) -> ValidationErrorCode {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ValidationError {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationResult {
    errors: Vec<ValidationError>,
}

impl ValidationResult {
    #[must_use]
    pub fn success() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_errors(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    #[must_use]
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    #[must_use]
    pub fn is_invalid(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn into_result(self) -> Result<(), Vec<ValidationError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

// ============================================================================
// Semantic type system
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
enum SemanticType {
    Unknown,

    Bool,
    Bit(Option<u64>),
    Int(Option<u64>),
    UInt(Option<u64>),
    Float(Option<u64>),
    Angle(Option<u64>),
    Complex(Option<u64>),
    Duration,
    Stretch,

    Quantum {
        size: Option<u64>,
    },

    Array {
        element: Box<SemanticType>,
        dimensions: Vec<u64>,
    },

    GateParameter,
    GateOperand,

    Subroutine,
    Extern,

    Alias {
        quantum: bool,
        size: Option<u64>,
    },
}

impl SemanticType {
    fn is_quantum(&self) -> bool {
        matches!(
            self,
            Self::Quantum { .. }
                | Self::GateOperand
                | Self::Alias {
                    quantum: true,
                    ..
                }
        )
    }

    fn is_classical(&self) -> bool {
        !self.is_quantum()
            && !matches!(
                self,
                Self::Unknown
                    | Self::Subroutine
                    | Self::Extern
            )
    }

    fn quantum_size(&self) -> Option<u64> {
        match self {
            Self::Quantum { size } => *size,

            Self::GateOperand => Some(1),

            Self::Alias {
                quantum: true,
                size,
            } => *size,

            _ => None,
        }
    }

    fn classical_width(&self) -> Option<u64> {
        match self {
            Self::Bool => Some(1),

            Self::Bit(size)
            | Self::Int(size)
            | Self::UInt(size)
            | Self::Float(size)
            | Self::Angle(size)
            | Self::Complex(size) => *size,

            Self::Alias {
                quantum: false,
                size,
            } => *size,

            _ => None,
        }
    }
}

// ============================================================================
// Scope model
// ============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScopeKind {
    Global,
    Block,
    Gate,
    Subroutine,
}

#[derive(Clone, Debug)]
struct Symbol {
    ty: SemanticType,
    span: SourceSpan,
    constant: bool,
    scope: ScopeKind,
}

#[derive(Clone, Debug)]
struct Scope {
    kind: ScopeKind,
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    fn new(kind: ScopeKind) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct GateSignature {
    parameter_count: usize,
    operand_count: usize,
    span: SourceSpan,
}

// ============================================================================
// Validator
// ============================================================================

struct Validator<'a> {
    limits: &'a FrontendLimits,
    config: ValidationConfig,

    errors: Vec<ValidationError>,

    scopes: Vec<Scope>,

    gates: HashMap<String, GateSignature>,
    subroutines: HashMap<String, GateSignature>,

    gate_stack: Vec<String>,

    includes: HashSet<String>,

    /// Actual source-program version.
    ///
    /// This MUST NOT be replaced with `config.max_minor_version`.
    program_version: Option<OpenQasmVersion>,

    stdgates_available: bool,

    expression_nodes: u64,
    statement_count: u64,
    symbol_count: u64,

    loop_depth: u64,
    subroutine_depth: u64,

    in_gate_body: bool,
    in_subroutine_body: bool,
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

            scopes: vec![Scope::new(ScopeKind::Global)],

            gates: HashMap::new(),
            subroutines: HashMap::new(),

            gate_stack: Vec::new(),

            includes: HashSet::new(),

            program_version: None,
            stdgates_available: false,

            expression_nodes: 0,
            statement_count: 0,
            symbol_count: 0,

            loop_depth: 0,
            subroutine_depth: 0,

            in_gate_body: false,
            in_subroutine_body: false,
        }
    }

    fn finish(self) -> ValidationResult {
        ValidationResult::from_errors(self.errors)
    }

    // ------------------------------------------------------------------------
    // Program
    // ------------------------------------------------------------------------

    fn validate_program(&mut self, program: &Program) {
        self.program_version = program.version();

        self.validate_version(program);

        for statement in program.statements() {
            if !self.consume_statement_budget(statement.span()) {
                break;
            }

            self.validate_statement(statement);
        }
    }

    fn validate_version(&mut self, program: &Program) {
        match program.version() {
            Some(version) => {
                let supported =
                    version.major() as u16 == self.config.max_major_version
                        && version.minor() as u16 <= self.config.max_minor_version;

                if !supported {
                    self.error(
                        ValidationErrorCode::UnsupportedVersion,
                        version.span(),
                        format!(
                            "OpenQASM version {} is not supported; \
                             maximum supported version is {}.{}",
                            version,
                            self.config.max_major_version,
                            self.config.max_minor_version
                        ),
                    );
                }
            }

            None if !self.config.allow_missing_version => {
                self.error(
                    ValidationErrorCode::MissingVersion,
                    program.span(),
                    "OpenQASM version declaration is required \
                     by the configured production policy",
                );
            }

            None => {}
        }
    }

    // ------------------------------------------------------------------------
    // Statement dispatcher
    // ------------------------------------------------------------------------

    fn validate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Include(value) => {
                self.validate_include(
                    value.path(),
                    value.span(),
                );
            }

            Statement::CalibrationGrammar(value) => {
                if !self.config.allow_calibration {
                    self.error(
                        ValidationErrorCode::CalibrationDisabled,
                        value.span(),
                        "calibration grammar declarations are disabled \
                         by the current frontend policy",
                    );
                }
            }

            Statement::Pragma(value) => {
                if !self.config.allow_pragmas {
                    self.error(
                        ValidationErrorCode::PragmaDisabled,
                        value.span(),
                        "pragma statements are disabled",
                    );
                }
            }

            Statement::Annotated(value) => {
                if !self.config.allow_annotations {
                    self.error(
                        ValidationErrorCode::AnnotationDisabled,
                        value.span(),
                        "annotations are disabled",
                    );
                    return;
                }

                for annotation in value.annotations() {
                    if annotation.keyword().as_str().is_empty() {
                        self.error(
                            ValidationErrorCode::InvalidIdentifierUse,
                            annotation.span(),
                            "annotation keyword cannot be empty",
                        );
                    }
                }

                self.validate_statement(value.statement());
            }

            Statement::ClassicalDeclaration(value) => {
                if self.in_gate_body {
                    self.error(
                        ValidationErrorCode::InvalidGateBodyDeclaration,
                        value.span(),
                        "classical declarations are not permitted \
                         inside a gate definition",
                    );
                    return;
                }

                let ty = self.semantic_scalar_type(value.ty());

                self.validate_scalar_type(
                    value.ty(),
                    value.span(),
                );

                if let Some(initializer) = value.initializer() {
                    self.validate_expression(initializer);

                    self.validate_initializer_type(
                        &ty,
                        initializer,
                        value.span(),
                    );
                }

                self.declare(
                    value.name().as_str(),
                    ty,
                    value.name().span(),
                    false,
                );
            }

            Statement::ConstDeclaration(value) => {
                if self.in_gate_body {
                    self.error(
                        ValidationErrorCode::InvalidGateBodyDeclaration,
                        value.span(),
                        "constant declarations are not permitted \
                         inside a gate definition",
                    );
                    return;
                }

                let ty = self.semantic_scalar_type(value.ty());

                self.validate_scalar_type(
                    value.ty(),
                    value.span(),
                );

                self.validate_expression(
                    value.initializer(),
                );

                self.declare(
                    value.name().as_str(),
                    ty,
                    value.name().span(),
                    true,
                );
            }

            Statement::QuantumDeclaration(value) => {
                if !matches!(
                    self.current_scope_kind(),
                    ScopeKind::Global
                ) {
                    self.error(
                        ValidationErrorCode::InvalidGlobalDeclaration,
                        value.span(),
                        "qubit declarations are only permitted \
                         in global scope",
                    );
                    return;
                }

                self.validate_quantum_declaration(
                    value.ty(),
                    value.name().as_str(),
                    value.span(),
                    value.name().span(),
                );
            }

            Statement::OldStyleDeclaration(value) => {
                if !self.config.allow_legacy_declarations {
                    self.error(
                        ValidationErrorCode::UnsupportedStatement,
                        value.span(),
                        "legacy qreg/creg declarations are disabled",
                    );
                    return;
                }

                let size = match value.size() {
                    Some(expression) => {
                        self.validate_expression(expression);
                        self.const_u64(expression)
                    }

                    None => Some(1),
                };

                let Some(size) = size else {
                    self.error(
                        ValidationErrorCode::InvalidRegisterSize,
                        value.span(),
                        "legacy register size must be a compile-time \
                         non-negative integer",
                    );
                    return;
                };

                if size == 0 {
                    self.error(
                        ValidationErrorCode::InvalidRegisterSize,
                        value.span(),
                        "register size must be positive",
                    );
                    return;
                }

                self.check_register_size(size, value.span());

                let ty = match value.kind() {
                    OldStyleDeclarationKind::QReg => {
                        SemanticType::Quantum {
                            size: Some(size),
                        }
                    }

                    OldStyleDeclarationKind::CReg => {
                        SemanticType::Bit(Some(size))
                    }
                };

                self.declare(
                    value.name().as_str(),
                    ty,
                    value.name().span(),
                    false,
                );
            }

            Statement::AliasDeclaration(value) => {
                self.validate_alias_declaration(value);
            }

            Statement::IoDeclaration(value) => {
                if self.in_gate_body {
                    self.error(
                        ValidationErrorCode::InvalidGateBodyDeclaration,
                        value.span(),
                        "I/O declarations are not permitted \
                         inside a gate definition",
                    );
                    return;
                }

                self.validate_scalar_type(
                    value.ty(),
                    value.span(),
                );

                if value.initializer().is_some() {
                    self.error(
                        ValidationErrorCode::InvalidAssignment,
                        value.span(),
                        "input/output declarations cannot contain \
                         an initializer in this AST form",
                    );
                }

                self.declare(
                    value.name().as_str(),
                    self.semantic_scalar_type(value.ty()),
                    value.name().span(),
                    false,
                );
            }

            Statement::GateDefinition(value) => {
                if self.current_scope_kind() != ScopeKind::Global {
                    self.error(
                        ValidationErrorCode::InvalidGateScope,
                        value.span(),
                        "gate definitions are only permitted at global scope",
                    );
                    return;
                }

                self.validate_gate_definition(value);
            }

            Statement::DefDefinition(value) => {
                if self.current_scope_kind() != ScopeKind::Global {
                    self.error(
                        ValidationErrorCode::InvalidFunctionScope,
                        value.span(),
                        "subroutine definitions are only permitted \
                         at global scope",
                    );
                    return;
                }

                self.validate_def_definition(value);
            }

            Statement::ExternDeclaration(value) => {
                if !self.config.allow_extern {
                    self.error(
                        ValidationErrorCode::ExternDisabled,
                        value.span(),
                        "extern declarations are disabled",
                    );
                }

                if self.current_scope_kind() != ScopeKind::Global {
                    self.error(
                        ValidationErrorCode::InvalidScope,
                        value.span(),
                        "extern declarations are only permitted \
                         at global scope",
                    );
                }

                for argument in value.arguments() {
                    self.validate_type_specifier(
                        argument.ty(),
                        argument.span(),
                    );
                }

                if let Some(return_type) = value.return_type() {
                    self.validate_type_specifier(
                        return_type.ty(),
                        return_type.span(),
                    );
                }

                self.declare(
                    value.name().as_str(),
                    SemanticType::Extern,
                    value.name().span(),
                    true,
                );
            }

            Statement::GateCall(value) => {
                self.validate_gate_call(value);
            }

            Statement::Assignment(value) => {
                self.validate_assignment(
                    value.target(),
                    value.operator(),
                    value.value(),
                    value.span(),
                );
            }

            Statement::Expression(value) => {
                self.validate_expression(
                    value.expression(),
                );
            }

            Statement::MeasureAssignment(value) => {
                self.validate_measurement_assignment(
                    value.source().operand(),
                    value.destination(),
                    value.span(),
                );
            }

            Statement::Reset(value) => {
                self.validate_reset(
                    value.operands(),
                    value.span(),
                );
            }

            Statement::Barrier(value) => {
                self.validate_operands(
                    value.operands(),
                    value.span(),
                );
            }

            Statement::Delay(value) => {
                if !self.config.allow_timing {
                    self.error(
                        ValidationErrorCode::TimingDisabled,
                        value.span(),
                        "delay statements are disabled",
                    );
                }

                self.validate_expression(
                    value.duration(),
                );

                self.validate_operands(
                    value.operands(),
                    value.span(),
                );
            }

            Statement::Box(value) => {
                if !self.config.allow_timing {
                    self.error(
                        ValidationErrorCode::TimingDisabled,
                        value.span(),
                        "box statements are disabled",
                    );
                }

                if let Some(designator) = value.designator() {
                    self.validate_expression(designator);
                }

                self.with_block_scope(|validator| {
                    for statement in value.body() {
                        if !validator.consume_statement_budget(
                            statement.span(),
                        ) {
                            break;
                        }

                        validator.validate_statement(statement);
                    }
                });
            }

            Statement::If(value) => {
                self.validate_expression(
                    value.condition(),
                );

                self.validate_condition(
                    value.condition(),
                );

                self.validate_statement_or_scope(
                    value.then_body(),
                );

                if let Some(body) = value.else_body() {
                    self.validate_statement_or_scope(body);
                }
            }

            Statement::For(value) => {
                self.validate_scalar_type(
                    &value.variable_type(),
                    value.span(),
                );

                self.validate_for_iterable(
                    value.iterable(),
                );

                self.with_block_scope(|validator| {
                    let ty = validator.semantic_scalar_type(
                        value.variable_type(),
                    );

                    validator.declare(
                        value.variable().as_str(),
                        ty,
                        value.variable().span(),
                        false,
                    );

                    validator.loop_depth =
                        validator.loop_depth.saturating_add(1);

                    validator.validate_statement_or_scope(
                        value.body(),
                    );

                    validator.loop_depth =
                        validator.loop_depth.saturating_sub(1);
                });
            }

            Statement::While(value) => {
                self.validate_expression(
                    value.condition(),
                );

                self.validate_condition(
                    value.condition(),
                );

                self.with_block_scope(|validator| {
                    validator.loop_depth =
                        validator.loop_depth.saturating_add(1);

                    validator.validate_statement_or_scope(
                        value.body(),
                    );

                    validator.loop_depth =
                        validator.loop_depth.saturating_sub(1);
                });
            }

            Statement::Switch(value) => {
                self.validate_expression(
                    value.expression(),
                );

                let mut default_seen = false;
                let mut case_keys = HashSet::<String>::new();

                for case in value.cases() {
                    match case {
                        SwitchCase::Case {
                            expressions,
                            body,
                        } => {
                            for expression in expressions {
                                self.validate_expression(expression);

                                if let Some(key) =
                                    self.constant_expression_key(expression)
                                {
                                    if !case_keys.insert(key) {
                                        self.error(
                                            ValidationErrorCode::DuplicateIndex,
                                            expression.span(),
                                            "duplicate switch case value",
                                        );
                                    }
                                }
                            }

                            self.with_block_scope(|validator| {
                                for statement in body {
                                    if !validator
                                        .consume_statement_budget(
                                            statement.span(),
                                        )
                                    {
                                        break;
                                    }

                                    validator.validate_statement(statement);
                                }
                            });
                        }

                        SwitchCase::Default { body } => {
                            if default_seen {
                                self.error(
                                    ValidationErrorCode::InvalidCondition,
                                    value.span(),
                                    "switch may contain at most one default case",
                                );
                            }

                            default_seen = true;

                            self.with_block_scope(|validator| {
                                for statement in body {
                                    if !validator
                                        .consume_statement_budget(
                                            statement.span(),
                                        )
                                    {
                                        break;
                                    }

                                    validator.validate_statement(statement);
                                }
                            });
                        }
                    }
                }
            }

            Statement::Break(value) => {
                if self.loop_depth == 0 {
                    self.error(
                        ValidationErrorCode::BreakOutsideLoop,
                        value.span(),
                        "`break` is only valid inside a loop",
                    );
                }
            }

            Statement::Continue(value) => {
                if self.loop_depth == 0 {
                    self.error(
                        ValidationErrorCode::ContinueOutsideLoop,
                        value.span(),
                        "`continue` is only valid inside a loop",
                    );
                }
            }

            Statement::End(value) => {
                self.error(
                    ValidationErrorCode::UnsupportedStatement,
                    value.span(),
                    "`end` is not supported by the current logical frontend",
                );
            }

            Statement::Return(value) => {
                if !self.in_subroutine_body {
                    self.error(
                        ValidationErrorCode::ReturnOutsideSubroutine,
                        value.span(),
                        "`return` is only valid inside a subroutine",
                    );
                }

                if let Some(return_value) = value.value() {
                    match return_value {
                        ReturnValue::Expression(expression) => {
                            self.validate_expression(expression);
                        }

                        ReturnValue::Measure(measure) => {
                            self.validate_designator(
                                measure.operand(),
                                true,
                            );
                        }

                        ReturnValue::QuantumCall(call) => {
                            self.validate_quantum_call(call);
                        }
                    }
                }
            }

            Statement::Cal(value) => {
                if !self.config.allow_calibration {
                    self.error(
                        ValidationErrorCode::CalibrationDisabled,
                        value.span(),
                        "inline calibration blocks are disabled",
                    );
                }
            }

            Statement::Defcal(value) => {
                if !self.config.allow_calibration {
                    self.error(
                        ValidationErrorCode::CalibrationDisabled,
                        value.span(),
                        "defcal statements are disabled",
                    );
                }

                self.validate_operands(
                    value.operands(),
                    value.span(),
                );

                for argument in value.arguments() {
                    self.validate_type_specifier(
                        argument.ty(),
                        argument.span(),
                    );
                }

                if let Some(return_type) = value.return_type() {
                    self.validate_type_specifier(
                        return_type.ty(),
                        return_type.span(),
                    );
                }
            }

            Statement::Nop(value) => {
                self.error(
                    ValidationErrorCode::UnsupportedStatement,
                    value.span(),
                    "`nop` is not part of the supported logical IR subset",
                );
            }

            Statement::Extension(value) => {
                if !self.config.allow_extensions {
                    self.error(
                        ValidationErrorCode::ExtensionDisabled,
                        value.span(),
                        format!(
                            "OpenQASM extension `{}` is disabled",
                            value.name()
                        ),
                    );
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Include handling
    // ------------------------------------------------------------------------

    fn validate_include(
        &mut self,
        path: &str,
        span: SourceSpan,
    ) {
        if self.current_scope_kind() != ScopeKind::Global {
            self.error(
                ValidationErrorCode::IncludeOutOfScope,
                span,
                "include directives are only valid at global scope",
            );
            return;
        }

        if !self.config.allow_includes {
            self.error(
                ValidationErrorCode::IncludeDisabled,
                span,
                "include directives are disabled",
            );
            return;
        }

        if path.is_empty() || path.as_bytes().contains(&0) {
            self.error(
                ValidationErrorCode::InvalidInclude,
                span,
                "include path must be non-empty and must not contain NUL",
            );
            return;
        }

        if !self.includes.insert(path.to_owned()) {
            self.error(
                ValidationErrorCode::DuplicateInclude,
                span,
                format!("include `{path}` has already been processed"),
            );
            return;
        }

        if path == STANDARD_LIBRARY_INCLUDE {
            self.stdgates_available = true;
        }

        // Deliberately no filesystem/network operation here.
        //
        // Actual include resolution belongs to the importer/resolver layer.
    }

    // ------------------------------------------------------------------------
    // Alias
    // ------------------------------------------------------------------------

    fn validate_alias_declaration(
        &mut self,
        value: &super::ast::AliasDeclaration,
    ) {
        let mut quantum: Option<bool> = None;
        let mut total_size: Option<u64> = Some(0);

        if value.operands().is_empty() {
            self.error(
                ValidationErrorCode::InvalidOperand,
                value.span(),
                "alias must contain at least one operand",
            );
        }

        for operand in value.operands() {
            let symbol = self.resolve(
                operand.name().as_str(),
            );

            match symbol {
                Some(symbol) if symbol.ty.is_quantum() => {
                    if quantum == Some(false) {
                        self.error(
                            ValidationErrorCode::OperandTypeMismatch,
                            operand.span(),
                            "an alias cannot combine classical and quantum operands",
                        );
                    }

                    quantum = Some(true);

                    total_size = self.add_sizes(
                        total_size,
                        self.designator_size(operand),
                    );
                }

                Some(symbol) if symbol.ty.is_classical() => {
                    if quantum == Some(true) {
                        self.error(
                            ValidationErrorCode::OperandTypeMismatch,
                            operand.span(),
                            "an alias cannot combine quantum and classical operands",
                        );
                    }

                    quantum = Some(false);

                    total_size = self.add_sizes(
                        total_size,
                        self.designator_size(operand),
                    );
                }

                Some(_) => {
                    self.error(
                        ValidationErrorCode::InvalidIdentifierUse,
                        operand.span(),
                        format!(
                            "`{}` is not aliasable",
                            operand.name().as_str()
                        ),
                    );
                }

                None => {
                    self.error(
                        ValidationErrorCode::UndefinedIdentifier,
                        operand.name().span(),
                        format!(
                            "identifier `{}` is not defined",
                            operand.name().as_str()
                        ),
                    );
                }
            }

            self.validate_designator(
                operand,
                quantum != Some(false),
            );
        }

        self.declare(
            value.name().as_str(),
            SemanticType::Alias {
                quantum: quantum.unwrap_or(false),
                size: total_size,
            },
            value.name().span(),
            false,
        );
    }

    // ------------------------------------------------------------------------
    // Declarations/types
    // ------------------------------------------------------------------------

    fn validate_quantum_declaration(
        &mut self,
        ty: &QuantumType,
        name: &str,
        span: SourceSpan,
        name_span: SourceSpan,
    ) {
        let size = match ty {
            QuantumType::Qubit(size)
            | QuantumType::QReg(size) => match size {
                Some(expression) => {
                    self.validate_expression(expression);
                    self.const_u64(expression)
                }

                None => Some(1),
            },
        };

        let Some(size) = size else {
            self.error(
                ValidationErrorCode::InvalidRegisterSize,
                span,
                "quantum register size must be a compile-time positive integer",
            );
            return;
        };

        if size == 0 {
            self.error(
                ValidationErrorCode::InvalidRegisterSize,
                span,
                "quantum register size must be positive",
            );
            return;
        }

        self.check_register_size(size, span);

        self.declare(
            name,
            SemanticType::Quantum {
                size: Some(size),
            },
            name_span,
            false,
        );
    }

    fn validate_scalar_type(
        &mut self,
        ty: &ScalarType,
        span: SourceSpan,
    ) {
        match ty {
            ScalarType::Bool
            | ScalarType::Duration
            | ScalarType::Stretch
            | ScalarType::Void => {}

            ScalarType::Bit(size)
            | ScalarType::Int(size)
            | ScalarType::UInt(size)
            | ScalarType::Float(size)
            | ScalarType::Angle(size)
            | ScalarType::Complex(size) => {
                if let Some(expression) = size {
                    self.validate_expression(expression);

                    match self.const_u64(expression) {
                        Some(value) if value > 0 => {
                            self.check_register_size(
                                value,
                                expression.span(),
                            );
                        }

                        _ => {
                            self.error(
                                ValidationErrorCode::InvalidTypeSize,
                                expression.span(),
                                "type width must be a compile-time positive integer",
                            );
                        }
                    }
                }
            }

            ScalarType::Array {
                element,
                dimensions,
            } => {
                self.validate_scalar_type(
                    element,
                    span,
                );

                if dimensions.is_empty() {
                    self.error(
                        ValidationErrorCode::InvalidTypeSize,
                        span,
                        "array type must contain at least one dimension",
                    );
                }

                if dimensions.len() > 7 {
                    self.error(
                        ValidationErrorCode::InvalidTypeSize,
                        span,
                        "OpenQASM arrays may contain at most seven dimensions",
                    );
                }

                for dimension in dimensions {
                    self.validate_expression(dimension);

                    match self.const_u64(dimension) {
                        Some(value) if value > 0 => {
                            self.check_register_size(
                                value,
                                dimension.span(),
                            );
                        }

                        _ => {
                            self.error(
                                ValidationErrorCode::InvalidTypeSize,
                                dimension.span(),
                                "array dimensions must be compile-time positive integers",
                            );
                        }
                    }
                }
            }
        }
    }

    fn validate_type_specifier(
        &mut self,
        ty: &TypeSpecifier,
        span: SourceSpan,
    ) {
        match ty {
            TypeSpecifier::Classical(value) => {
                self.validate_scalar_type(
                    value,
                    span,
                );
            }

            TypeSpecifier::Quantum(value) => {
                self.validate_quantum_type(
                    value,
                    span,
                );
            }
        }
    }

    fn validate_quantum_type(
        &mut self,
        ty: &QuantumType,
        span: SourceSpan,
    ) {
        match ty {
            QuantumType::Qubit(size)
            | QuantumType::QReg(size) => {
                if let Some(expression) = size {
                    self.validate_expression(expression);

                    match self.const_u64(expression) {
                        Some(value) if value > 0 => {
                            self.check_register_size(
                                value,
                                expression.span(),
                            );
                        }

                        _ => {
                            self.error(
                                ValidationErrorCode::InvalidRegisterSize,
                                expression.span(),
                                "quantum register size must be a compile-time positive integer",
                            );
                        }
                    }
                }
            }
        }
    }

    fn semantic_scalar_type(
        &self,
        ty: &ScalarType,
    ) -> SemanticType {
        match ty {
            ScalarType::Bool => SemanticType::Bool,

            ScalarType::Bit(size) => {
                SemanticType::Bit(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::Int(size) => {
                SemanticType::Int(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::UInt(size) => {
                SemanticType::UInt(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::Float(size) => {
                SemanticType::Float(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::Angle(size) => {
                SemanticType::Angle(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::Complex(size) => {
                SemanticType::Complex(
                    size.as_ref()
                        .and_then(|value| self.const_u64(value)),
                )
            }

            ScalarType::Duration => SemanticType::Duration,

            ScalarType::Stretch => SemanticType::Stretch,

            ScalarType::Void => SemanticType::Unknown,

            ScalarType::Array {
                element,
                dimensions,
            } => SemanticType::Array {
                element: Box::new(
                    self.semantic_scalar_type(element),
                ),
                dimensions: dimensions
                    .iter()
                    .filter_map(|value| {
                        self.const_u64(value)
                    })
                    .collect(),
            },
        }
    }

    // ------------------------------------------------------------------------
    // Gates
    // ------------------------------------------------------------------------

    fn validate_gate_definition(
        &mut self,
        definition: &GateDefinition,
    ) {
        let name = definition.name().as_str();

        if self.gates.contains_key(name)
            || self.subroutines.contains_key(name)
            || self.resolve(name).is_some()
            || self.is_language_builtin_gate(name)
            || (
                self.stdgates_available
                    && lookup_standard_gate(name).is_some()
            )
        {
            self.error(
                ValidationErrorCode::DuplicateGateDefinition,
                definition.name().span(),
                format!(
                    "gate `{name}` conflicts with an existing declaration or built-in"
                ),
            );
            return;
        }

        let parameter_count =
            definition.parameters().len();

        let operand_count =
            definition.qubits().len();

        if parameter_count as u64
            > self.effective_max_parameters()
        {
            self.error(
                ValidationErrorCode::ParameterLimitExceeded,
                definition.span(),
                "gate parameter count exceeds configured limit",
            );
        }

        if operand_count as u64
            > self.effective_max_operands()
        {
            self.error(
                ValidationErrorCode::OperandLimitExceeded,
                definition.span(),
                "gate operand count exceeds configured limit",
            );
        }

        let mut parameters = HashSet::<&str>::new();

        for parameter in definition.parameters() {
            if !parameters.insert(parameter.as_str()) {
                self.error(
                    ValidationErrorCode::DuplicateGateParameter,
                    parameter.span(),
                    format!(
                        "gate parameter `{}` is declared more than once",
                        parameter.as_str()
                    ),
                );
            }
        }

        let mut qubits = HashSet::<&str>::new();

        for qubit in definition.qubits() {
            if !qubits.insert(qubit.as_str()) {
                self.error(
                    ValidationErrorCode::DuplicateFormalOperand,
                    qubit.span(),
                    format!(
                        "gate operand `{}` is declared more than once",
                        qubit.as_str()
                    ),
                );
            }
        }

        if self.gate_stack.len() as u64
            >= self.config.max_gate_call_depth
        {
            self.error(
                ValidationErrorCode::GateExpansionDepthExceeded,
                definition.span(),
                "gate-definition validation depth exceeds configured maximum",
            );
            return;
        }

        // Declaration-before-use requires the signature to become available
        // before validating the body, but only after conflict checking.
        self.gates.insert(
            name.to_owned(),
            GateSignature {
                parameter_count,
                operand_count,
                span: definition.span(),
            },
        );

        let previous_gate_body =
            self.in_gate_body;

        self.in_gate_body = true;
        self.gate_stack.push(name.to_owned());

        self.scopes.push(
            Scope::new(ScopeKind::Gate),
        );

        for parameter in definition.parameters() {
            self.declare(
                parameter.as_str(),
                SemanticType::GateParameter,
                parameter.span(),
                true,
            );
        }

        for qubit in definition.qubits() {
            self.declare(
                qubit.as_str(),
                SemanticType::GateOperand,
                qubit.span(),
                false,
            );
        }

        for statement in definition.body() {
            if !self.consume_statement_budget(
                statement.span(),
            ) {
                break;
            }

            if !self.is_valid_gate_body_statement(statement) {
                self.error(
                    ValidationErrorCode::InvalidGateBody,
                    statement.span(),
                    "statement is not permitted inside a gate definition",
                );
                continue;
            }

            self.validate_statement(statement);
        }

        self.scopes.pop();
        self.gate_stack.pop();

        self.in_gate_body = previous_gate_body;
    }

    fn is_valid_gate_body_statement(
        &self,
        statement: &Statement,
    ) -> bool {
        matches!(
            statement,
            Statement::GateCall(_)
                | Statement::If(_)
                | Statement::For(_)
                | Statement::While(_)
                | Statement::Box(_)
                | Statement::Barrier(_)
                | Statement::Delay(_)
                | Statement::Expression(_)
                | Statement::Annotated(_)
                | Statement::Pragma(_)
                | Statement::Reset(_)
        )
    }

    fn validate_gate_call(
        &mut self,
        call: &GateCall,
    ) {
        let name = call.name().as_str();

        let signature =
            if let Some(signature) = self.gates.get(name) {
                if self.gate_stack.iter().any(
                    |active| active == name,
                ) {
                    self.error(
                        ValidationErrorCode::RecursiveGateDefinition,
                        call.name().span(),
                        format!(
                            "gate `{name}` recursively invokes itself"
                        ),
                    );
                }

                Some(signature.clone())
            } else if let Some(signature) =
                self.language_builtin_signature(name)
            {
                Some(signature)
            } else if self.stdgates_available {
                self.standard_gate_signature(name)
            } else {
                None
            };

        let Some(signature) = signature else {
            if self.is_known_standard_gate(name) {
                self.error(
                    ValidationErrorCode::StandardLibraryUnavailable,
                    call.name().span(),
                    format!(
                        "standard-library gate `{name}` requires \
                         `include \"{STANDARD_LIBRARY_INCLUDE}\";`"
                    ),
                );
            } else {
                self.error(
                    ValidationErrorCode::UndefinedGate,
                    call.name().span(),
                    format!(
                        "gate `{name}` is not defined before this use"
                    ),
                );
            }

            self.validate_gate_arguments_without_signature(
                call,
            );
            return;
        };

        let control_count =
            self.control_modifier_count(
                call.modifiers(),
            );

        let expected_operands =
            signature.operand_count
                .saturating_add(control_count);

        if call.operands().len()
            != expected_operands
        {
            self.error(
                ValidationErrorCode::GateOperandCountMismatch,
                call.span(),
                format!(
                    "gate `{name}` expects {} quantum operands, \
                     received {}",
                    expected_operands,
                    call.operands().len()
                ),
            );
        }

        if call.parameters().len()
            != signature.parameter_count
        {
            self.error(
                ValidationErrorCode::GateParameterCountMismatch,
                call.span(),
                format!(
                    "gate `{name}` expects {} parameters, \
                     received {}",
                    signature.parameter_count,
                    call.parameters().len()
                ),
            );
        }

        if call.operands().len() as u64
            > self.effective_max_operands()
        {
            self.error(
                ValidationErrorCode::OperandLimitExceeded,
                call.span(),
                "gate call exceeds configured operand limit",
            );
        }

        if call.parameters().len() as u64
            > self.effective_max_parameters()
        {
            self.error(
                ValidationErrorCode::ParameterLimitExceeded,
                call.span(),
                "gate call exceeds configured parameter limit",
            );
        }

        for parameter in call.parameters() {
            self.validate_expression(parameter);
        }

        self.validate_operands(
            call.operands(),
            call.span(),
        );

        self.validate_gate_modifiers(
            call.modifiers(),
            call.span(),
        );

        self.validate_broadcasting(
            call.operands(),
            call.span(),
        );
    }

    fn validate_gate_arguments_without_signature(
        &mut self,
        call: &GateCall,
    ) {
        for parameter in call.parameters() {
            self.validate_expression(parameter);
        }

        self.validate_operands(
            call.operands(),
            call.span(),
        );

        self.validate_gate_modifiers(
            call.modifiers(),
            call.span(),
        );
    }

    fn language_builtin_signature(
        &self,
        name: &str,
    ) -> Option<GateSignature> {
        match name {
            "U" => Some(
                GateSignature {
                    parameter_count: 3,
                    operand_count: 1,
                    span: dummy_span(),
                },
            ),

            "gphase" => Some(
                GateSignature {
                    parameter_count: 1,
                    operand_count: 0,
                    span: dummy_span(),
                },
            ),

            _ => None,
        }
    }

    fn standard_gate_signature(
        &self,
        name: &str,
    ) -> Option<GateSignature> {
        let entry = lookup_standard_gate(name)?;

        let version =
            self.program_version.unwrap_or(
                OpenQasmVersion::V3_1,
            );

        if !entry.available_in(
            version.major(),
            version.minor(),
        ) {
            return None;
        }

        Some(
            GateSignature {
                parameter_count:
                    entry.parameter_count(),
                operand_count:
                    entry.qubit_count(),
                span: dummy_span(),
            },
        )
    }

    fn current_version(&self) -> OpenQasmVersion {
        self.program_version.unwrap_or(
            OpenQasmVersion::V3_1,
        )
    }

    fn is_language_builtin_gate(
        &self,
        name: &str,
    ) -> bool {
        matches!(
            name,
            "U" | "gphase"
        )
    }

    fn is_known_standard_gate(
        &self,
        name: &str,
    ) -> bool {
        lookup_standard_gate(name).is_some()
    }

    fn validate_gate_modifiers(
        &mut self,
        modifiers: &[GateModifier],
        span: SourceSpan,
    ) {
        let mut controls = 0u64;
        let mut inverse = 0u64;

        for modifier in modifiers {
            match modifier {
                GateModifier::Ctrl => {
                    controls = controls.saturating_add(1);
                }

                GateModifier::NegCtrl => {
                    controls = controls.saturating_add(1);
                }

                GateModifier::Inv => {
                    inverse = inverse.saturating_add(1);
                }

                GateModifier::Pow(expression) => {
                    self.validate_expression(expression);

                    // OpenQASM 3.1 permits a positive integer or floating-point
                    // exponent and explicitly permits it to be non-constant.
                    //
                    // Therefore do NOT require const_u64().
                    let value =
                        self.const_f64(expression);

                    if let Some(value) = value {
                        if !value.is_finite() || value <= 0.0 {
                            self.error(
                                ValidationErrorCode::InvalidGateModifier,
                                expression.span(),
                                "pow exponent must be positive and finite",
                            );
                        }
                    }
                }

                GateModifier::CtrlCount {
                    count,
                    ..
                } => {
                    let count_value =
                        match count {
                            Some(expression) => {
                                self.validate_expression(expression);
                                self.const_u64(expression)
                            }

                            None => Some(1),
                        };

                    match count_value {
                        Some(value) if value > 0 => {
                            controls =
                                controls.saturating_add(value);
                        }

                        _ => {
                            self.error(
                                ValidationErrorCode::InvalidModifierCount,
                                span,
                                "control count must be a positive compile-time integer",
                            );
                        }
                    }
                }
            }
        }

        if inverse > 1 {
            self.error(
                ValidationErrorCode::InvalidGateModifier,
                span,
                "multiple inverse modifiers are not permitted",
            );
        }

        if controls >
            self.effective_max_operands()
        {
            self.error(
                ValidationErrorCode::OperandLimitExceeded,
                span,
                "gate control modifiers exceed operand limit",
            );
        }
    }

    fn control_modifier_count(
        &self,
        modifiers: &[GateModifier],
    ) -> usize {
        modifiers
            .iter()
            .map(|modifier| match modifier {
                GateModifier::Ctrl
                | GateModifier::NegCtrl => 1,

                GateModifier::CtrlCount {
                    count,
                    ..
                } => count
                    .as_ref()
                    .and_then(|expression| {
                        self.const_u64(expression)
                    })
                    .unwrap_or(1) as usize,

                GateModifier::Inv
                | GateModifier::Pow(_) => 0,
            })
            .sum()
    }

    // ------------------------------------------------------------------------
    // Operands
    // ------------------------------------------------------------------------

    fn validate_operands(
        &mut self,
        operands: &[GateOperand],
        span: SourceSpan,
    ) {
        let mut identities =
            HashSet::<String>::new();

        for operand in operands {
            match operand {
                GateOperand::Designator(designator) => {
                    let symbol = self.resolve(
                        designator.name().as_str(),
                    );

                    match symbol {
                        Some(symbol) if symbol.ty.is_quantum() => {
                            self.validate_designator(
                                designator,
                                true,
                            );

                            let identity =
                                self.designator_identity(
                                    designator,
                                );

                            if !identities.insert(identity) {
                                self.error(
                                    ValidationErrorCode::DuplicateQuantumOperand,
                                    designator.span(),
                                    "the same quantum operand is used more than once",
                                );
                            }
                        }

                        Some(symbol) => {
                            self.error(
                                ValidationErrorCode::OperandTypeMismatch,
                                designator.span(),
                                format!(
                                    "operand `{}` has type {:?}, \
                                     but a quantum operand is required",
                                    designator.name().as_str(),
                                    symbol.ty
                                ),
                            );
                        }

                        None => {
                            self.error(
                                ValidationErrorCode::UndefinedIdentifier,
                                designator.name().span(),
                                format!(
                                    "quantum operand `{}` is not defined",
                                    designator.name().as_str()
                                ),
                            );
                        }
                    }
                }

                GateOperand::Physical(physical) => {
                    if !self.config.allow_physical_qubits {
                        self.error(
                            ValidationErrorCode::PhysicalQubitDisabled,
                            physical.span(),
                            format!(
                                "physical qubit `${}` is disabled \
                                 by the logical frontend policy",
                                physical.index()
                            ),
                        );
                    }
                }

                GateOperand::Alias(identifier) => {
                    match self.resolve(
                        identifier.as_str(),
                    ) {
                        Some(symbol) if symbol.ty.is_quantum() => {}

                        Some(_) => {
                            self.error(
                                ValidationErrorCode::OperandTypeMismatch,
                                identifier.span(),
                                format!(
                                    "alias `{}` is not quantum",
                                    identifier.as_str()
                                ),
                            );
                        }

                        None => {
                            self.error(
                                ValidationErrorCode::UndefinedIdentifier,
                                identifier.span(),
                                format!(
                                    "alias `{}` is not defined",
                                    identifier.as_str()
                                ),
                            );
                        }
                    }
                }
            }
        }

        let _ = span;
    }

    fn validate_broadcasting(
        &mut self,
        operands: &[GateOperand],
        span: SourceSpan,
    ) {
        let mut expected: Option<u64> = None;

        for operand in operands {
            let Some(size) =
                self.operand_broadcast_size(operand)
            else {
                continue;
            };

            if size <= 1 {
                continue;
            }

            match expected {
                None => expected = Some(size),

                Some(existing)
                    if existing != size =>
                {
                    self.error(
                        ValidationErrorCode::RegisterBroadcastMismatch,
                        span,
                        format!(
                            "broadcasted quantum registers must have \
                             identical sizes; found {existing} and {size}"
                        ),
                    );
                }

                _ => {}
            }
        }
    }

    fn operand_broadcast_size(
        &self,
        operand: &GateOperand,
    ) -> Option<u64> {
        match operand {
            GateOperand::Designator(designator) => {
                let symbol =
                    self.resolve(
                        designator.name().as_str(),
                    )?;

                if designator.index().is_some() {
                    return Some(1);
                }

                symbol.ty.quantum_size()
            }

            GateOperand::Alias(identifier) => {
                self.resolve(
                    identifier.as_str(),
                )
                .and_then(
                    |symbol| symbol.ty.quantum_size(),
                )
            }

            GateOperand::Physical(_) => Some(1),
        }
    }

    // ------------------------------------------------------------------------
    // Designators/index sets
    // ------------------------------------------------------------------------

    fn validate_designator(
        &mut self,
        designator: &Designator,
        quantum_expected: bool,
    ) {
        let Some(symbol) =
            self.resolve(
                designator.name().as_str(),
            )
        else {
            self.error(
                ValidationErrorCode::UndefinedIdentifier,
                designator.name().span(),
                format!(
                    "identifier `{}` is not defined",
                    designator.name().as_str()
                ),
            );
            return;
        };

        if quantum_expected && !symbol.ty.is_quantum() {
            self.error(
                ValidationErrorCode::OperandTypeMismatch,
                designator.span(),
                "a quantum object is required",
            );
        }

        if !quantum_expected && symbol.ty.is_quantum() {
            self.error(
                ValidationErrorCode::OperandTypeMismatch,
                designator.span(),
                "a classical object is required",
            );
        }

        if let Some(index) =
            designator.index()
        {
            if self.in_gate_body
                && matches!(
                    symbol.ty,
                    SemanticType::GateOperand
                )
            {
                self.error(
                    ValidationErrorCode::IndexedGateFormalOperand,
                    index_span(index),
                    "formal gate operands cannot be indexed \
                     inside a gate definition",
                );
            }

            if symbol.ty.is_quantum()
                && !self.config.allow_runtime_quantum_index
                && !self.index_is_compile_time_constant(index)
            {
                self.error(
                    ValidationErrorCode::RuntimeQuantumIndexDisabled,
                    index_span(index),
                    "runtime quantum-register indexing is disabled \
                     by the production frontend policy",
                );
            }

            self.validate_index_expression(
                index,
                symbol.ty.quantum_size()
                    .or_else(|| symbol.ty.classical_width()),
                symbol.ty.is_quantum(),
            );
        }
    }

    fn index_is_compile_time_constant(
        &self,
        index: &IndexExpression,
    ) -> bool {
        match index {
            IndexExpression::Index(expression) => {
                self.const_i128(expression).is_some()
            }

            IndexExpression::Slice {
                start,
                stop,
            } => start
                .as_ref()
                .map(|value| self.const_i128(value).is_some())
                .unwrap_or(true)
                && stop
                    .as_ref()
                    .map(|value| self.const_i128(value).is_some())
                    .unwrap_or(true),

            IndexExpression::Range {
                start,
                step,
                stop,
            } => start
                .as_ref()
                .map(|value| self.const_i128(value).is_some())
                .unwrap_or(true)
                && step
                    .as_ref()
                    .map(|value| self.const_i128(value).is_some())
                    .unwrap_or(true)
                && stop
                    .as_ref()
                    .map(|value| self.const_i128(value).is_some())
                    .unwrap_or(true),

            IndexExpression::Set(values) => values
                .iter()
                .all(|value| self.const_i128(value).is_some()),

            IndexExpression::Concatenation(_) => true,
        }
    }

    fn validate_index_expression(
        &mut self,
        index: &IndexExpression,
        known_size: Option<u64>,
        quantum: bool,
    ) {
        match index {
            IndexExpression::Index(expression) => {
                self.validate_expression(expression);

                if self.const_i128(expression).is_none() {
                    if quantum && !self.config.allow_runtime_quantum_index {
                        self.error(
                            ValidationErrorCode::RuntimeQuantumIndexDisabled,
                            expression.span(),
                            "quantum index must be compile-time constant",
                        );
                    }

                    return;
                }

                if let (Some(index), Some(size)) =
                    (
                        self.const_i128(expression),
                        known_size,
                    )
                {
                    if !self.index_in_bounds(
                        index,
                        size,
                    ) {
                        self.error(
                            ValidationErrorCode::IndexOutOfBounds,
                            expression.span(),
                            format!(
                                "index {index} is outside valid range \
                                 for a collection of size {size}"
                            ),
                        );
                    }
                }
            }

            IndexExpression::Slice {
                start,
                stop,
            } => {
                if let Some(start) = start {
                    self.validate_expression(start);
                }

                if let Some(stop) = stop {
                    self.validate_expression(stop);
                }

                self.validate_slice_bounds(
                    start.as_ref(),
                    None,
                    stop.as_ref(),
                    known_size,
                );
            }

            IndexExpression::Range {
                start,
                step,
                stop,
            } => {
                if let Some(start) = start {
                    self.validate_expression(start);
                }

                if let Some(step) = step {
                    self.validate_expression(step);

                    if let Some(value) =
                        self.const_i128(step)
                    {
                        if value == 0 {
                            self.error(
                                ValidationErrorCode::InvalidSlice,
                                step.span(),
                                "range step cannot be zero",
                            );
                        }
                    }
                }

                if let Some(stop) = stop {
                    self.validate_expression(stop);
                }

                self.validate_slice_bounds(
                    start.as_ref(),
                    step.as_ref(),
                    stop.as_ref(),
                    known_size,
                );
            }

            IndexExpression::Set(values) => {
                if values.is_empty() {
                    self.error(
                        ValidationErrorCode::EmptyIndexSet,
                        index_span(index),
                        "index set cannot be empty",
                    );
                    return;
                }

                let mut seen =
                    HashSet::<i128>::new();

                for value in values {
                    self.validate_expression(value);

                    let Some(index) =
                        self.const_i128(value)
                    else {
                        continue;
                    };

                    if !seen.insert(index) {
                        self.error(
                            ValidationErrorCode::DuplicateIndex,
                            value.span(),
                            format!(
                                "index {index} occurs more than once"
                            ),
                        );
                    }

                    if let Some(size) =
                        known_size
                    {
                        if !self.index_in_bounds(
                            index,
                            size,
                        ) {
                            self.error(
                                ValidationErrorCode::IndexOutOfBounds,
                                value.span(),
                                format!(
                                    "index {index} is outside valid range \
                                     for a collection of size {size}"
                                ),
                            );
                        }
                    }
                }
            }

            IndexExpression::Concatenation(
                designators,
            ) => {
                if designators.is_empty() {
                    self.error(
                        ValidationErrorCode::InvalidSlice,
                        index_span(index),
                        "register concatenation cannot be empty",
                    );
                    return;
                }

                for designator in designators {
                    self.validate_designator(
                        designator,
                        quantum,
                    );
                }
            }
        }
    }

    fn index_in_bounds(
        &self,
        index: i128,
        size: u64,
    ) -> bool {
        if size == 0 {
            return false;
        }

        let size_i =
            size as i128;

        // OpenQASM arrays and register slices support negative indices.
        let normalized =
            if index < 0 {
                size_i.checked_add(index)
            } else {
                Some(index)
            };

        normalized
            .map(|value| {
                value >= 0
                    && value < size_i
            })
            .unwrap_or(false)
    }

    fn validate_slice_bounds(
        &mut self,
        start: Option<&Expression>,
        step: Option<&Expression>,
        stop: Option<&Expression>,
        known_size: Option<u64>,
    ) {
        let Some(size) = known_size else {
            return;
        };

        let start_value =
            start.and_then(
                |expression| self.const_i128(expression),
            );

        let stop_value =
            stop.and_then(
                |expression| self.const_i128(expression),
            );

        let step_value =
            step
                .and_then(
                    |expression| {
                        self.const_i128(expression)
                    },
                )
                .unwrap_or(1);

        if step_value == 0 {
            return;
        }

        let size_i =
            size as i128;

        if let Some(value) = start_value {
            if value < -size_i
                || value >= size_i
            {
                self.error(
                    ValidationErrorCode::IndexOutOfBounds,
                    start.unwrap().span(),
                    format!(
                        "slice start {value} is outside \
                         valid bounds for size {size}"
                    ),
                );
            }
        }

        if let Some(value) = stop_value {
            if value < -size_i
                || value >= size_i
            {
                self.error(
                    ValidationErrorCode::IndexOutOfBounds,
                    stop.unwrap().span(),
                    format!(
                        "slice stop {value} is outside \
                         valid bounds for size {size}"
                    ),
                );
            }
        }

        // A descending range is legal. The old validator incorrectly
        // rejected start > stop regardless of step sign.
        if let (Some(start), Some(stop)) =
            (start_value, stop_value)
        {
            if step_value > 0 && start > stop {
                // OpenQASM semantics make this an empty range rather than
                // an invalid range.
            }

            if step_value < 0 && start < stop {
                // Likewise: empty range, not a semantic error.
            }
        }
    }

    fn designator_identity(
        &self,
        designator: &Designator,
    ) -> String {
        match designator.index() {
            Some(index) => format!(
                "{}:{index:?}",
                designator.name().as_str()
            ),

            None => designator
                .name()
                .as_str()
                .to_owned(),
        }
    }

    fn designator_size(
        &self,
        designator: &Designator,
    ) -> Option<u64> {
        let symbol =
            self.resolve(
                designator.name().as_str(),
            )?;

        match designator.index() {
            None => symbol.ty
                .quantum_size()
                .or_else(
                    || symbol.ty.classical_width(),
                ),

            Some(index) => {
                self.index_set_cardinality(
                    index,
                    symbol.ty.quantum_size()
                        .or_else(
                            || symbol.ty.classical_width(),
                        ),
                )
            }
        }
    }

    fn index_set_cardinality(
        &self,
        index: &IndexExpression,
        known_size: Option<u64>,
    ) -> Option<u64> {
        match index {
            IndexExpression::Index(_) => Some(1),

            IndexExpression::Set(values) => {
                Some(values.len() as u64)
            }

            IndexExpression::Concatenation(values) => {
                let mut total = 0u64;

                for designator in values {
                    total = total.checked_add(
                        self.designator_size(designator)?,
                    )?;
                }

                Some(total)
            }

            IndexExpression::Slice {
                start,
                stop,
            } => {
                let size = known_size?;

                let start =
                    start.and_then(
                        |value| self.const_i128(value),
                    )
                    .unwrap_or(0);

                let stop =
                    stop.and_then(
                        |value| self.const_i128(value),
                    )
                    .unwrap_or(
                        size as i128 - 1,
                    );

                self.range_cardinality(
                    start,
                    1,
                    stop,
                    size,
                )
            }

            IndexExpression::Range {
                start,
                step,
                stop,
            } => {
                let size = known_size?;

                let start =
                    start.and_then(
                        |value| self.const_i128(value),
                    )
                    .unwrap_or(0);

                let step =
                    step.and_then(
                        |value| self.const_i128(value),
                    )
                    .unwrap_or(1);

                let stop =
                    stop.and_then(
                        |value| self.const_i128(value),
                    )
                    .unwrap_or(
                        size as i128 - 1,
                    );

                self.range_cardinality(
                    start,
                    step,
                    stop,
                    size,
                )
            }
        }
    }

    fn range_cardinality(
        &self,
        start: i128,
        step: i128,
        stop: i128,
        size: u64,
    ) -> Option<u64> {
        if step == 0 || size == 0 {
            return None;
        }

        let size_i =
            size as i128;

        let normalize = |value: i128| {
            if value < 0 {
                size_i.checked_add(value)
            } else {
                Some(value)
            }
        };

        let start =
            normalize(start)?;

        let stop =
            normalize(stop)?;

        if step > 0 {
            if start > stop {
                return Some(0);
            }

            let distance =
                stop.checked_sub(start)?;

            let count =
                distance.checked_div(step)?;

            u64::try_from(
                count.checked_add(1)?,
            )
            .ok()
        } else {
            if start < stop {
                return Some(0);
            }

            let distance =
                start.checked_sub(stop)?;

            let count =
                distance.checked_div(
                    step.checked_neg()?,
                )?;

            u64::try_from(
                count.checked_add(1)?,
            )
            .ok()
        }
    }

    // ------------------------------------------------------------------------
    // Measurement
    // ------------------------------------------------------------------------

    fn validate_measurement_assignment(
        &mut self,
        source: &Designator,
        destination: &Designator,
        span: SourceSpan,
    ) {
        self.validate_designator(
            source,
            true,
        );

        self.validate_designator(
            destination,
            false,
        );

        let source_size =
            self.designator_size(source);

        let destination_size =
            self.designator_size(destination);

        if let (
            Some(source_size),
            Some(destination_size),
        ) = (
            source_size,
            destination_size,
        ) {
            if source_size != destination_size {
                self.error(
                    ValidationErrorCode::InvalidMeasurementDestination,
                    span,
                    format!(
                        "measurement produces {source_size} bit(s), \
                         but destination contains {destination_size}"
                    ),
                );
            }
        }
    }

    fn validate_measurement_expression(
        &mut self,
        operand: &Designator,
    ) {
        self.validate_designator(
            operand,
            true,
        );
    }

    fn validate_reset(
        &mut self,
        operands: &[GateOperand],
        span: SourceSpan,
    ) {
        if operands.is_empty() {
            self.error(
                ValidationErrorCode::InvalidOperand,
                span,
                "reset requires at least one quantum operand",
            );
        }

        self.validate_operands(
            operands,
            span,
        );
    }

    // ------------------------------------------------------------------------
    // Assignment
    // ------------------------------------------------------------------------

    fn validate_assignment(
        &mut self,
        target: &Designator,
        operator: AssignmentOperator,
        value: &AssignmentValue,
        span: SourceSpan,
    ) {
        if self.in_gate_body {
            self.error(
                ValidationErrorCode::GateBodyClassicalOperation,
                span,
                "classical assignment is not permitted inside a gate",
            );
        }

        self.validate_designator(
            target,
            false,
        );

        match value {
            AssignmentValue::Expression(expression) => {
                self.validate_expression(expression);

                let target_type =
                    self.designator_type(target);

                let value_type =
                    self.expression_type(expression);

                if !self.types_compatible(
                    &target_type,
                    &value_type,
                ) {
                    self.error(
                        ValidationErrorCode::AssignmentTypeMismatch,
                        span,
                        format!(
                            "assignment type mismatch: target is {:?}, \
                             value is {:?}",
                            target_type,
                            value_type
                        ),
                    );
                }
            }

            AssignmentValue::Measure(measurement) => {
                self.validate_measurement_expression(
                    measurement.operand(),
                );

                if !self.designator_is_classical(target) {
                    self.error(
                        ValidationErrorCode::InvalidAssignmentTarget,
                        target.span(),
                        "measurement destination must be classical",
                    );
                }
            }

            AssignmentValue::QuantumCall(call) => {
                self.validate_quantum_call(call);

                self.error(
                    ValidationErrorCode::InvalidAssignment,
                    span,
                    "quantum-call assignment is not representable \
                     by the current logical Quantum IR",
                );
            }
        }

        if !matches!(
            operator,
            AssignmentOperator::Assign
        ) && !self.designator_is_mutable(target) {
            self.error(
                ValidationErrorCode::InvalidAssignmentTarget,
                target.span(),
                "compound assignment requires a mutable target",
            );
        }
    }

    fn designator_is_classical(
        &self,
        designator: &Designator,
    ) -> bool {
        self.resolve(
            designator.name().as_str(),
        )
        .map(|symbol| symbol.ty.is_classical())
        .unwrap_or(false)
    }

    fn designator_is_mutable(
        &self,
        designator: &Designator,
    ) -> bool {
        self.resolve(
            designator.name().as_str(),
        )
        .map(|symbol| !symbol.constant)
        .unwrap_or(false)
    }

    // ------------------------------------------------------------------------
    // Control flow
    // ------------------------------------------------------------------------

    fn validate_condition(
        &mut self,
        expression: &Expression,
    ) {
        let ty =
            self.expression_type(expression);

        if !matches!(
            ty,
            SemanticType::Bool
                | SemanticType::Bit(_)
                | SemanticType::UInt(_)
                | SemanticType::Int(_)
        ) {
            self.error(
                ValidationErrorCode::InvalidCondition,
                expression.span(),
                format!(
                    "condition must be boolean or integral; found {:?}",
                    ty
                ),
            );
        }
    }

    fn validate_for_iterable(
        &mut self,
        iterable: &ForIterable,
    ) {
        match iterable {
            ForIterable::Expression(expression) => {
                self.validate_expression(expression);
            }

            ForIterable::Range {
                start,
                step,
                stop,
            } => {
                self.validate_expression(start);

                if let Some(step) = step {
                    self.validate_expression(step);

                    if self.const_i128(step) == Some(0) {
                        self.error(
                            ValidationErrorCode::InvalidLoop,
                            step.span(),
                            "for-loop range step cannot be zero",
                        );
                    }
                }

                self.validate_expression(stop);
            }

            ForIterable::Set(values) => {
                if values.is_empty() {
                    self.error(
                        ValidationErrorCode::InvalidLoop,
                        dummy_span(),
                        "for-loop set cannot be empty",
                    );
                }

                for value in values {
                    self.validate_expression(value);
                }
            }
        }
    }

    fn validate_statement_or_scope(
        &mut self,
        body: &StatementOrScope,
    ) {
        match body {
            StatementOrScope::Statement(statement) => {
                self.validate_statement(statement);
            }

            StatementOrScope::Scope(scope) => {
                self.with_block_scope(|validator| {
                    for statement in scope.statements() {
                        if !validator.consume_statement_budget(
                            statement.span(),
                        ) {
                            break;
                        }

                        validator.validate_statement(statement);
                    }
                });
            }
        }
    }

    // ------------------------------------------------------------------------
    // Expressions
    // ------------------------------------------------------------------------

    fn validate_expression(
        &mut self,
        expression: &Expression,
    ) {
        if self.expression_nodes
            >= self.effective_max_expression_nodes()
        {
            self.error(
                ValidationErrorCode::ExpressionNodeLimitExceeded,
                expression.span(),
                "semantic expression-node limit exceeded",
            );
            return;
        }

        self.expression_nodes =
            self.expression_nodes.saturating_add(1);

        self.validate_expression_at_depth(
            expression,
            0,
        );
    }

    fn validate_expression_at_depth(
        &mut self,
        expression: &Expression,
        depth: u64,
    ) {
        if depth >
            self.effective_max_expression_depth()
        {
            self.error(
                ValidationErrorCode::ExpressionDepthExceeded,
                expression.span(),
                "semantic expression depth exceeds configured limit",
            );
            return;
        }

        match expression {
            Expression::BoolLiteral { .. } => {}

            Expression::IntegerLiteral { value, .. } => {
                if self.parse_integer_literal(
                    value.raw(),
                    value.radix(),
                ).is_none() {
                    self.error(
                        ValidationErrorCode::InvalidNumericLiteral,
                        expression.span(),
                        "integer literal cannot be represented",
                    );
                }
            }

            Expression::FloatLiteral { value, .. } => {
                match value.raw().parse::<f64>() {
                    Ok(value) if value.is_finite() => {}

                    _ => {
                        self.error(
                            ValidationErrorCode::NonFiniteLiteral,
                            expression.span(),
                            "floating-point literal must be finite",
                        );
                    }
                }
            }

            Expression::DurationLiteral { value, .. } => {
                self.validate_expression(
                    value.value(),
                );
            }

            Expression::Identifier(identifier) => {
                if identifier.as_str() != "pi" {
                    self.require_symbol(
                        identifier.as_str(),
                        identifier.span(),
                    );
                }
            }

            Expression::Designator(designator) => {
                self.validate_designator(
                    designator,
                    false,
                );
            }

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                self.validate_unary_operator(
                    *operator,
                    expression.span(),
                );

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
                    *operator,
                    expression.span(),
                );

                self.validate_expression_at_depth(
                    right,
                    depth.saturating_add(1),
                );

                if matches!(
                    operator,
                    BinaryOperator::Divide
                        | BinaryOperator::Remainder
                ) {
                    if self.const_f64(right) == Some(0.0) {
                        self.error(
                            ValidationErrorCode::DivisionByZero,
                            right.span(),
                            "division by zero is not valid",
                        );
                    }
                }
            }

            Expression::FunctionCall {
                name,
                arguments,
                ..
            } => {
                self.validate_function_call(
                    name.as_str(),
                    arguments,
                    expression.span(),
                );
            }

            Expression::Cast {
                target,
                operand,
                ..
            } => {
                self.validate_scalar_type(
                    target,
                    expression.span(),
                );

                self.validate_expression_at_depth(
                    operand,
                    depth.saturating_add(1),
                );
            }

            Expression::Measure(measurement) => {
                self.validate_measurement_expression(
                    measurement.operand(),
                );
            }

            Expression::QuantumCall(call) => {
                self.validate_quantum_call(call);
            }

            Expression::ArrayLiteral { values, .. }
            | Expression::SetLiteral { values, .. }
            | Expression::Concatenation { values, .. } => {
                for value in values {
                    self.validate_expression_at_depth(
                        value,
                        depth.saturating_add(1),
                    );
                }
            }

            Expression::Range {
                start,
                step,
                stop,
                ..
            } => {
                if let Some(start) = start {
                    self.validate_expression_at_depth(
                        start,
                        depth.saturating_add(1),
                    );
                }

                if let Some(step) = step {
                    self.validate_expression_at_depth(
                        step,
                        depth.saturating_add(1),
                    );
                }

                if let Some(stop) = stop {
                    self.validate_expression_at_depth(
                        stop,
                        depth.saturating_add(1),
                    );
                }
            }

            Expression::Parenthesized {
                expression,
                ..
            } => {
                self.validate_expression_at_depth(
                    expression,
                    depth.saturating_add(1),
                );
            }

            Expression::SizeOf {
                operand,
                ..
            } => {
                self.validate_designator(
                    operand,
                    false,
                );
            }

            Expression::DurationOf {
                body,
                ..
            } => {
                if !self.config.allow_timing {
                    self.error(
                        ValidationErrorCode::TimingDisabled,
                        expression.span(),
                        "durationof is disabled",
                    );
                }

                self.with_block_scope(|validator| {
                    for statement in body {
                        if !validator.consume_statement_budget(
                            statement.span(),
                        ) {
                            break;
                        }

                        validator.validate_statement(statement);
                    }
                });
            }

            Expression::Extension {
                name,
                arguments,
                ..
            } => {
                if !self.config.allow_extensions {
                    self.error(
                        ValidationErrorCode::ExtensionDisabled,
                        expression.span(),
                        format!(
                            "expression extension `{}` is disabled",
                            name.as_str()
                        ),
                    );
                }

                for argument in arguments {
                    self.validate_expression_at_depth(
                        argument,
                        depth.saturating_add(1),
                    );
                }
            }
        }
    }

    fn validate_unary_operator(
        &mut self,
        _operator: UnaryOperator,
        _span: SourceSpan,
    ) {
    }

    fn validate_binary_operator(
        &mut self,
        _operator: BinaryOperator,
        _span: SourceSpan,
    ) {
    }

    fn validate_function_call(
        &mut self,
        name: &str,
        arguments: &[Expression],
        span: SourceSpan,
    ) {
        for argument in arguments {
            self.validate_expression(argument);
        }

        let known =
            matches!(
                name,
                "arccos"
                    | "arcsin"
                    | "arctan"
                    | "ceil"
                    | "ceiling"
                    | "cos"
                    | "exp"
                    | "floor"
                    | "log"
                    | "ln"
                    | "mod"
                    | "popcount"
                    | "pow"
                    | "rotl"
                    | "rotr"
                    | "sin"
                    | "sqrt"
                    | "tan"
            );

        if !known
            && self.resolve(name).is_none()
            && !self.config.allow_extensions
        {
            self.error(
                ValidationErrorCode::UndefinedIdentifier,
                span,
                format!(
                    "function `{name}` is not defined"
                ),
            );
        }
    }

    fn validate_quantum_call(
        &mut self,
        call: &super::ast::QuantumCallExpression,
    ) {
        for parameter in call.parameters() {
            self.validate_expression(parameter);
        }

        self.validate_operands(
            call.operands(),
            call.span(),
        );

        self.validate_gate_modifiers(
            call.modifiers(),
            call.span(),
        );

        self.validate_broadcasting(
            call.operands(),
            call.span(),
        );

        let name =
            call.name().as_str();

        if self.gates.contains_key(name)
            || self.is_language_builtin_gate(name)
            || (
                self.stdgates_available
                    && self.is_known_standard_gate(name)
            )
        {
            return;
        }

        self.error(
            ValidationErrorCode::UndefinedGate,
            call.name().span(),
            format!(
                "quantum call `{name}` is not defined"
            ),
        );
    }

    // ------------------------------------------------------------------------
    // Expression typing
    // ------------------------------------------------------------------------

    fn expression_type(
        &self,
        expression: &Expression,
    ) -> SemanticType {
        match expression {
            Expression::BoolLiteral { .. } => {
                SemanticType::Bool
            }

            Expression::IntegerLiteral { .. } => {
                SemanticType::Int(None)
            }

            Expression::FloatLiteral { .. } => {
                SemanticType::Float(None)
            }

            Expression::DurationLiteral { .. } => {
                SemanticType::Duration
            }

            Expression::Identifier(identifier) => {
                if identifier.as_str() == "pi" {
                    SemanticType::Float(None)
                } else {
                    self.resolve(
                        identifier.as_str(),
                    )
                    .map(|symbol| symbol.ty.clone())
                    .unwrap_or(
                        SemanticType::Unknown,
                    )
                }
            }

            Expression::Designator(designator) => {
                self.designator_type(designator)
            }

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                match operator {
                    UnaryOperator::LogicalNot => {
                        SemanticType::Bool
                    }

                    _ => self.expression_type(operand),
                }
            }

            Expression::Binary {
                operator,
                left,
                right,
                ..
            } => {
                match operator {
                    BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::LogicalAnd
                    | BinaryOperator::LogicalOr => {
                        SemanticType::Bool
                    }

                    _ => self.promote_numeric_types(
                        self.expression_type(left),
                        self.expression_type(right),
                    ),
                }
            }

            Expression::FunctionCall { name, .. } => {
                match name.as_str() {
                    "popcount" => SemanticType::UInt(None),

                    "rotl" | "rotr" => SemanticType::Unknown,

                    _ => SemanticType::Float(None),
                }
            }

            Expression::Cast { target, .. } => {
                self.semantic_scalar_type(target)
            }

            Expression::Measure(_) => {
                SemanticType::Bit(Some(1))
            }

            Expression::QuantumCall(_) => {
                SemanticType::Unknown
            }

            Expression::ArrayLiteral { .. }
            | Expression::SetLiteral { .. }
            | Expression::Range { .. }
            | Expression::Concatenation { .. } => {
                SemanticType::Unknown
            }

            Expression::Parenthesized {
                expression,
                ..
            } => self.expression_type(expression),

            Expression::SizeOf { .. } => {
                SemanticType::UInt(None)
            }

            Expression::DurationOf { .. } => {
                SemanticType::Duration
            }

            Expression::Extension { .. } => {
                SemanticType::Unknown
            }
        }
    }

    fn designator_type(
        &self,
        designator: &Designator,
    ) -> SemanticType {
        let Some(symbol) =
            self.resolve(
                designator.name().as_str(),
            )
        else {
            return SemanticType::Unknown;
        };

        let Some(index) =
            designator.index()
        else {
            return symbol.ty.clone();
        };

        match &symbol.ty {
            SemanticType::Quantum { .. }
            | SemanticType::GateOperand => {
                SemanticType::Quantum {
                    size: self.index_set_cardinality(
                        index,
                        symbol.ty.quantum_size(),
                    ),
                }
            }

            SemanticType::Bit(_)
            | SemanticType::Int(_)
            | SemanticType::UInt(_)
            | SemanticType::Angle(_) => {
                SemanticType::Bit(
                    self.index_set_cardinality(
                        index,
                        symbol.ty.classical_width(),
                    ),
                )
            }

            SemanticType::Array { .. } => {
                symbol.ty.clone()
            }

            _ => SemanticType::Unknown,
        }
    }

    fn promote_numeric_types(
        &self,
        left: SemanticType,
        right: SemanticType,
    ) -> SemanticType {
        match (&left, &right) {
            (SemanticType::Complex(_), _)
            | (_, SemanticType::Complex(_)) => {
                SemanticType::Complex(None)
            }

            (SemanticType::Float(_), _)
            | (_, SemanticType::Float(_)) => {
                SemanticType::Float(None)
            }

            (SemanticType::Angle(_), _)
            | (_, SemanticType::Angle(_)) => {
                SemanticType::Angle(None)
            }

            (SemanticType::UInt(_), _)
            | (_, SemanticType::UInt(_)) => {
                SemanticType::UInt(None)
            }

            (SemanticType::Int(_), _)
            | (_, SemanticType::Int(_)) => {
                SemanticType::Int(None)
            }

            _ => SemanticType::Unknown,
        }
    }

    fn types_compatible(
        &self,
        left: &SemanticType,
        right: &SemanticType,
    ) -> bool {
        if matches!(
            left,
            SemanticType::Unknown
        ) || matches!(
            right,
            SemanticType::Unknown
        ) {
            return true;
        }

        if left == right {
            return true;
        }

        matches!(
            (left, right),
            (
                SemanticType::Bool,
                SemanticType::Bit(Some(1))
            )
            | (
                SemanticType::Bit(Some(1)),
                SemanticType::Bool
            )
            | (
                SemanticType::Int(_),
                SemanticType::UInt(_)
            )
            | (
                SemanticType::UInt(_),
                SemanticType::Int(_)
            )
            | (
                SemanticType::Float(_),
                SemanticType::Int(_)
            )
            | (
                SemanticType::Float(_),
                SemanticType::UInt(_)
            )
            | (
                SemanticType::Float(_),
                SemanticType::Angle(_)
            )
            | (
                SemanticType::Angle(_),
                SemanticType::Float(_)
            )
        )
    }

    fn validate_initializer_type(
        &mut self,
        target: &SemanticType,
        expression: &Expression,
        span: SourceSpan,
    ) {
        let source =
            self.expression_type(expression);

        if !self.types_compatible(
            target,
            &source,
        ) {
            self.error(
                ValidationErrorCode::AssignmentTypeMismatch,
                span,
                format!(
                    "initializer type {:?} is incompatible \
                     with declared type {:?}",
                    source,
                    target
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // Symbol/scoping
    // ------------------------------------------------------------------------

    fn current_scope_kind(
        &self,
    ) -> ScopeKind {
        self.scopes
            .last()
            .map(|scope| scope.kind)
            .unwrap_or(ScopeKind::Global)
    }

    fn declare(
        &mut self,
        name: &str,
        ty: SemanticType,
        span: SourceSpan,
        constant: bool,
    ) {
        if self.symbol_count
            >= self.effective_max_symbols()
        {
            self.error(
                ValidationErrorCode::SymbolLimitExceeded,
                span,
                "frontend symbol limit exceeded",
            );
            return;
        }

        let scope_kind =
            self.current_scope_kind();

        if scope_kind == ScopeKind::Global {
            // OpenQASM global declarations cannot be redeclared.
            if self.resolve(name).is_some() {
                self.error(
                    ValidationErrorCode::DuplicateDeclaration,
                    span,
                    format!(
                        "identifier `{name}` is already declared"
                    ),
                );
                return;
            }
        } else if let Some(scope) =
            self.scopes.last()
        {
            if scope.symbols.contains_key(name) {
                self.error(
                    ValidationErrorCode::DuplicateDeclaration,
                    span,
                    format!(
                        "identifier `{name}` is declared more than once \
                         in this scope"
                    ),
                );
                return;
            }
        }

        let Some(scope) =
            self.scopes.last_mut()
        else {
            self.error(
                ValidationErrorCode::InvalidScope,
                span,
                "validator has no active scope",
            );
            return;
        };

        scope.symbols.insert(
            name.to_owned(),
            Symbol {
                ty,
                span,
                constant,
                scope: scope_kind,
            },
        );

        self.symbol_count =
            self.symbol_count.saturating_add(1);
    }

    fn resolve(
        &self,
        name: &str,
    ) -> Option<&Symbol> {
        let current =
            self.current_scope_kind();

        for scope in self.scopes.iter().rev() {
            if let Some(symbol) =
                scope.symbols.get(name)
            {
                match current {
                    ScopeKind::Gate
                    | ScopeKind::Subroutine => {
                        if scope.kind == ScopeKind::Global {
                            if symbol.constant
                                || symbol.ty.is_quantum()
                                || matches!(
                                    symbol.ty,
                                    SemanticType::Subroutine
                                        | SemanticType::Extern
                                )
                            {
                                return Some(symbol);
                            }

                            continue;
                        }

                        if scope.kind == ScopeKind::Gate
                            || scope.kind == ScopeKind::Subroutine
                        {
                            return Some(symbol);
                        }

                        continue;
                    }

                    ScopeKind::Global
                    | ScopeKind::Block => {
                        return Some(symbol);
                    }
                }
            }
        }

        // Built-in constant.
        if name == "pi" {
            return None;
        }

        None
    }

    fn require_symbol(
        &mut self,
        name: &str,
        span: SourceSpan,
    ) {
        if name == "pi" {
            return;
        }

        if self.resolve(name).is_none() {
            self.error(
                ValidationErrorCode::UndefinedIdentifier,
                span,
                format!(
                    "identifier `{name}` is not defined"
                ),
            );
        }
    }

    fn with_block_scope<F>(
        &mut self,
        function: F,
    )
    where
        F: FnOnce(&mut Self),
    {
        self.scopes.push(
            Scope::new(ScopeKind::Block),
        );

        function(self);

        self.scopes.pop();
    }

    // ------------------------------------------------------------------------
    // Constants
    // ------------------------------------------------------------------------

    fn const_u64(
        &self,
        expression: &Expression,
    ) -> Option<u64> {
        let value =
            self.const_i128(expression)?;

        u64::try_from(value).ok()
    }

    fn const_i128(
        &self,
        expression: &Expression,
    ) -> Option<i128> {
        match expression {
            Expression::IntegerLiteral {
                value,
                ..
            } => self.parse_integer_literal(
                value.raw(),
                value.radix(),
            ),

            Expression::Parenthesized {
                expression,
                ..
            } => self.const_i128(expression),

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                let value =
                    self.const_i128(operand)?;

                match operator {
                    UnaryOperator::Plus =>
                        Some(value),

                    UnaryOperator::Minus =>
                        value.checked_neg(),

                    _ => None,
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let left =
                    self.const_i128(left)?;

                let right =
                    self.const_i128(right)?;

                match operator {
                    BinaryOperator::Add =>
                        left.checked_add(right),

                    BinaryOperator::Subtract =>
                        left.checked_sub(right),

                    BinaryOperator::Multiply =>
                        left.checked_mul(right),

                    BinaryOperator::Divide => {
                        if right == 0 {
                            None
                        } else {
                            left.checked_div(right)
                        }
                    }

                    BinaryOperator::Remainder => {
                        if right == 0 {
                            None
                        } else {
                            left.checked_rem(right)
                        }
                    }

                    BinaryOperator::Power => {
                        if right < 0
                            || right > 127
                        {
                            return None;
                        }

                        let mut result = 1i128;
                        let mut exponent =
                            right as u32;
                        let mut base = left;

                        while exponent > 0 {
                            if exponent & 1 == 1 {
                                result =
                                    result.checked_mul(base)?;
                            }

                            exponent >>= 1;

                            if exponent > 0 {
                                base =
                                    base.checked_mul(base)?;
                            }
                        }

                        Some(result)
                    }

                    _ => None,
                }
            }

            Expression::Identifier(identifier)
                if identifier.as_str() == "pi" =>
            {
                None
            }

            _ => None,
        }
    }

    fn const_f64(
        &self,
        expression: &Expression,
    ) -> Option<f64> {
        match expression {
            Expression::IntegerLiteral {
                value,
                ..
            } => self
                .parse_integer_literal(
                    value.raw(),
                    value.radix(),
                )
                .map(|value| value as f64),

            Expression::FloatLiteral {
                value,
                ..
            } => value
                .raw()
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite()),

            Expression::Parenthesized {
                expression,
                ..
            } => self.const_f64(expression),

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                let value =
                    self.const_f64(operand)?;

                match operator {
                    UnaryOperator::Plus =>
                        Some(value),

                    UnaryOperator::Minus =>
                        Some(-value),

                    _ => None,
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let left =
                    self.const_f64(left)?;

                let right =
                    self.const_f64(right)?;

                let result =
                    match operator {
                        BinaryOperator::Add =>
                            left + right,

                        BinaryOperator::Subtract =>
                            left - right,

                        BinaryOperator::Multiply =>
                            left * right,

                        BinaryOperator::Divide => {
                            if right == 0.0 {
                                return None;
                            }

                            left / right
                        }

                        _ => return None,
                    };

                result
                    .is_finite()
                    .then_some(result)
            }

            _ => None,
        }
    }

    fn parse_integer_literal(
        &self,
        raw: &str,
        radix: IntegerRadix,
    ) -> Option<i128> {
        let normalized =
            raw.replace('_', "");

        let (digits, base) =
            match radix {
                IntegerRadix::Decimal => {
                    (
                        normalized.as_str(),
                        10,
                    )
                }

                IntegerRadix::Binary => {
                    (
                        normalized
                            .strip_prefix("0b")
                            .or_else(
                                || normalized.strip_prefix("0B"),
                            )?,
                        2,
                    )
                }

                IntegerRadix::Octal => {
                    (
                        normalized
                            .strip_prefix("0o")
                            .or_else(
                                || normalized.strip_prefix("0O"),
                            )?,
                        8,
                    )
                }

                IntegerRadix::Hexadecimal => {
                    (
                        normalized
                            .strip_prefix("0x")
                            .or_else(
                                || normalized.strip_prefix("0X"),
                            )?,
                        16,
                    )
                }
            };

        i128::from_str_radix(
            digits,
            base,
        )
        .ok()
    }

    fn constant_expression_key(
        &self,
        expression: &Expression,
    ) -> Option<String> {
        if let Some(value) =
            self.const_i128(expression)
        {
            return Some(
                format!("i:{value}"),
            );
        }

        self.const_f64(expression)
            .map(|value| {
                format!("f:{value:.17e}")
            })
    }

    // ------------------------------------------------------------------------
    // Limits
    // ------------------------------------------------------------------------

    fn consume_statement_budget(
        &mut self,
        span: SourceSpan,
    ) -> bool {
        if self.statement_count
            >= self.effective_max_statements()
        {
            self.error(
                ValidationErrorCode::StatementLimitExceeded,
                span,
                "semantic statement limit exceeded",
            );
            return false;
        }

        self.statement_count =
            self.statement_count.saturating_add(1);

        true
    }

    fn check_register_size(
        &mut self,
        size: u64,
        span: SourceSpan,
    ) {
        if size >
            self.effective_max_register_size()
        {
            self.error(
                ValidationErrorCode::RegisterLimitExceeded,
                span,
                format!(
                    "register size {size} exceeds configured maximum {}",
                    self.effective_max_register_size()
                ),
            );
        }
    }

    fn effective_max_expression_depth(
        &self,
    ) -> u64 {
        self.config
            .max_expression_depth
            .min(
                self.limits.max_expression_depth(),
            )
    }

    fn effective_max_expression_nodes(
        &self,
    ) -> u64 {
        self.config
            .max_expression_nodes
            .min(
                self.limits.max_expression_nodes(),
            )
    }

    fn effective_max_register_size(
        &self,
    ) -> u64 {
        self.config
            .max_register_size
            .min(
                self.limits.max_register_size(),
            )
    }

    fn effective_max_symbols(
        &self,
    ) -> u64 {
        self.config
            .max_symbols
            .min(
                self.limits.max_symbols(),
            )
    }

    fn effective_max_parameters(
        &self,
    ) -> u64 {
        self.config
            .max_parameters
            .min(
                self.limits.max_parameters(),
            )
    }

    fn effective_max_operands(
        &self,
    ) -> u64 {
        self.config
            .max_operands
            .min(
                self.limits.max_operands(),
            )
    }

    fn effective_max_statements(
        &self,
    ) -> u64 {
        self.config
            .max_statements
            .min(
                self.limits.max_statements(),
            )
    }

    fn add_sizes(
        &mut self,
        left: Option<u64>,
        right: Option<u64>,
    ) -> Option<u64> {
        match (left, right) {
            (Some(left), Some(right)) => {
                let value =
                    left.checked_add(right)?;

                if value >
                    self.effective_max_register_size()
                {
                    None
                } else {
                    Some(value)
                }
            }

            _ => None,
        }
    }

    // ------------------------------------------------------------------------
    // Errors
    // ------------------------------------------------------------------------

    fn error(
        &mut self,
        code: ValidationErrorCode,
        span: SourceSpan,
        message: impl Into<String>,
    ) {
        if self.errors.len() as u64
            >= self.limits.max_diagnostics()
        {
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

// ============================================================================
// Public API
// ============================================================================

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

pub fn validate_program_with_config(
    program: &Program,
    limits: &FrontendLimits,
    config: ValidationConfig,
) -> ValidationResult {
    let mut validator =
        Validator::new(
            limits,
            config,
        );

    validator.validate_program(program);

    validator.finish()
}

// ============================================================================
// Helpers
// ============================================================================

fn index_span(
    index: &IndexExpression,
) -> SourceSpan {
    match index {
        IndexExpression::Index(expression) =>
            expression.span(),

        IndexExpression::Slice {
            start,
            stop,
        } =>
            start
                .as_ref()
                .map(|value| value.span())
                .or_else(|| {
                    stop
                        .as_ref()
                        .map(|value| value.span())
                })
                .unwrap_or_else(dummy_span),

        IndexExpression::Range {
            start,
            step,
            stop,
        } =>
            start
                .as_ref()
                .map(|value| value.span())
                .or_else(|| {
                    step
                        .as_ref()
                        .map(|value| value.span())
                })
                .or_else(|| {
                    stop
                        .as_ref()
                        .map(|value| value.span())
                })
                .unwrap_or_else(dummy_span),

        IndexExpression::Set(values) =>
            values
                .first()
                .map(|value| value.span())
                .unwrap_or_else(dummy_span),

        IndexExpression::Concatenation(values) =>
            values
                .first()
                .map(|value| value.span())
                .unwrap_or_else(dummy_span),
    }
}

fn dummy_span() -> SourceSpan {
    SourceSpan::point(
        crate::quantum::frontend::core::source::SourceId::from_raw(0),
        0,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_targets_openqasm_31() {
        let config =
            ValidationConfig::production();

        assert_eq!(
            config.max_major_version,
            3
        );

        assert_eq!(
            config.max_minor_version,
            1
        );

        assert!(
            !config.allow_runtime_quantum_index
        );
    }

    #[test]
    fn strict_policy_has_no_external_execution_features() {
        let config =
            ValidationConfig::strict();

        assert!(
            !config.allow_includes
        );

        assert!(
            !config.allow_extern
        );

        assert!(
            !config.allow_calibration
        );

        assert!(
            !config.allow_physical_qubits
        );

        assert!(
            !config.allow_extensions
        );
    }

    #[test]
    fn stable_error_codes() {
        assert_eq!(
            ValidationErrorCode::UndefinedGate
                .as_str(),
            "QASM-G010"
        );

        assert_eq!(
            ValidationErrorCode::IndexOutOfBounds
                .as_str(),
            "QASM-T004"
        );

        assert_eq!(
            ValidationErrorCode::MissingVersion
                .as_str(),
            "QASM-V002"
        );
    }

    #[test]
    fn empty_result_is_success() {
        let result =
            ValidationResult::success();

        assert!(
            result.is_valid()
        );

        assert!(
            result.errors().is_empty()
        );
    }

    #[test]
    fn decimal_integer_is_parsed() {
        let validator =
            Validator::new(
                &FrontendLimits::production(),
                ValidationConfig::production(),
            );

        assert_eq!(
            validator.parse_integer_literal(
                "42",
                IntegerRadix::Decimal,
            ),
            Some(42)
        );
    }

    #[test]
    fn binary_integer_is_parsed() {
        let validator =
            Validator::new(
                &FrontendLimits::production(),
                ValidationConfig::production(),
            );

        assert_eq!(
            validator.parse_integer_literal(
                "0b101010",
                IntegerRadix::Binary,
            ),
            Some(42)
        );
    }

    #[test]
    fn octal_integer_is_parsed() {
        let validator =
            Validator::new(
                &FrontendLimits::production(),
                ValidationConfig::production(),
            );

        assert_eq!(
            validator.parse_integer_literal(
                "0o52",
                IntegerRadix::Octal,
            ),
            Some(42)
        );
    }

    #[test]
    fn hexadecimal_integer_is_parsed() {
        let validator =
            Validator::new(
                &FrontendLimits::production(),
                ValidationConfig::production(),
            );

        assert_eq!(
            validator.parse_integer_literal(
                "0x2a",
                IntegerRadix::Hexadecimal,
            ),
            Some(42)
        );
    }

    #[test]
    fn integer_overflow_is_rejected() {
        let validator =
            Validator::new(
                &FrontendLimits::production(),
                ValidationConfig::production(),
            );

        assert!(
            validator
                .parse_integer_literal(
                    "170141183460469231731687303715884105727",
                    IntegerRadix::Decimal,
                )
                .is_some()
        );

        assert!(
            validator
                .parse_integer_literal(
                    "170141183460469231731687303715884105728",
                    IntegerRadix::Decimal,
                )
                .is_none()
        );
    }
}