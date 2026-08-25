//! Zamani Quantum Frontend — OpenQASM importer.
//!
//! Production import boundary for OpenQASM 3.x.
//!
//! # Pipeline
//!
//! ```text
//! untrusted bytes
//!      |
//!      v
//! ImportInput
//!      |
//!      v
//! UTF-8 validation
//!      |
//!      v
//! OpenQASM lexer/parser
//!      |
//!      v
//! OpenQASM AST
//!      |
//!      v
//! OpenQASM semantic validation
//!      |
//!      v
//! deterministic lowering
//!      |
//!      v
//! canonical QuantumCircuit
//!      |
//!      v
//! ImportOutput::try_new()
//!      |
//!      v
//! canonical IR validation
//! ```
//!
//! # Security boundary
//!
//! This module never:
//!
//! - executes OpenQASM;
//! - executes `extern` declarations;
//! - executes calibration code;
//! - accesses the filesystem;
//! - accesses the network;
//! - spawns processes;
//! - accesses quantum hardware;
//! - performs hardware mapping;
//! - performs routing;
//! - performs scheduling;
//! - performs optimization;
//! - silently discards unsupported semantics.
//!
//! OpenQASM `include` is accepted only when it refers to the explicitly
//! supported standard-library include. Arbitrary include resolution belongs
//! to a higher-level resolver and is deliberately outside this importer.
//!
//! # Lowering policy
//!
//! The current canonical Quantum IR directly supports a subset of OpenQASM.
//! This importer therefore lowers only constructs for which the canonical IR
//! has an unambiguous representation.
//!
//! Unsupported constructs are rejected explicitly rather than being silently
//! erased.
//!
//! # Source locations
//!
//! Source locations are owned by the generic frontend `SourceMap` and the
//! OpenQASM AST. This file never creates a second source-location model.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / 1.97.1.
//! Rust 2021.
//! Stable Rust only.
//!
//! No additional dependencies.

use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;

use crate::quantum::frontend::core::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticCode,
    DiagnosticSeverity,
};
use crate::quantum::frontend::core::errors::{
    FrontendError,
    FrontendErrorCode,
    FrontendErrorKind,
    FrontendLimitViolation,
    FrontendResult,
};
use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::format::{
    FormatId,
    FormatVersion,
};
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportInput,
    ImportOutput,
    ImportResult,
};

use crate::quantum::ir::{
    Gate,
    GateKind,
    Measurement,
    Parameter,
    ParameterExpression,
    QuantumCircuit,
    QuantumIrLimits,
    QubitId,
};

use super::ast::{
    BinaryOperator,
    ClassicalDeclaration,
    ConstDeclaration,
    Designator,
    Expression,
    GateCall,
    GateOperand,
    IndexExpression,
    MeasureExpression,
    Program,
    QuantumDeclaration,
    QuantumType,
    ScalarType,
    Statement,
};
use super::parser::{
    OpenQasmParser,
    ParserConfig,
    ParserLimits,
};
use super::stdgates::{
    lookup as lookup_standard_gate,
    STANDARD_LIBRARY_INCLUDE,
};
use super::validation::{
    validate_program_with_config,
    ValidationConfig,
    ValidationError,
};

/// Production OpenQASM importer.
///
/// The importer contains no mutable execution state and can safely be shared
/// between threads.
#[derive(Clone, Debug)]
pub struct OpenQasmImporter {
    version: FormatVersion,
    validation: ValidationConfig,
}

impl Default for OpenQasmImporter {
    fn default() -> Self {
        Self::production()
    }
}

impl OpenQasmImporter {
    /// Creates the production OpenQASM 3.1 importer.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            version: FormatVersion::new(3, 1, 0),
            validation: ValidationConfig::production(),
        }
    }

    /// Creates an importer with an explicit OpenQASM version and validation
    /// policy.
    #[must_use]
    pub const fn new(
        version: FormatVersion,
        validation: ValidationConfig,
    ) -> Self {
        Self {
            version,
            validation,
        }
    }

    /// Returns the configured OpenQASM version.
    #[must_use]
    pub const fn configured_version(&self) -> FormatVersion {
        self.version
    }

    /// Returns the validation policy.
    #[must_use]
    pub const fn validation_config(&self) -> ValidationConfig {
        self.validation
    }

    /// Parses and validates an already UTF-8-decoded OpenQASM source.
    ///
    /// This helper deliberately accepts an `ImportInput` rather than
    /// manufacturing a `SourceMap`. The generic import contract requires the
    /// source map to contain the exact source represented by `source_id`.
    pub fn import_utf8(
        &self,
        input: ImportInput,
    ) -> ImportResult {
        self.import(input)
    }

    fn parse(
        &self,
        source: &str,
        source_id: crate::quantum::frontend::core::source::SourceId,
        limits: &FrontendLimits,
    ) -> FrontendResult<Program> {
        let parser_limits =
            parser_limits_from_frontend(limits)?;

        let parser_config = ParserConfig {
            source_id,
            limits: parser_limits,
        };

        OpenQasmParser::parse(
            source,
            parser_config,
        )
        .map_err(|error| {
            FrontendError::with_code(
                FrontendErrorKind::Syntax,
                FrontendErrorCode::new(error.code()),
                error.to_string(),
            )
            .context("format", "OpenQASM")
            .context("stage", "parser")
        })
    }

    fn check_version(
        &self,
        program: &Program,
    ) -> FrontendResult<()> {
        let version =
            program.version().ok_or_else(|| {
                FrontendError::with_code(
                    FrontendErrorKind::Semantic,
                    FrontendErrorCode::new("QASM-V002"),
                    "OpenQASM version declaration is required",
                )
                .context("format", "OpenQASM")
            })?;

        let source_version =
            FormatVersion::major_minor(
                u32::from(version.major()),
                u32::from(version.minor()),
            );

        if source_version.major() != 3 {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Unsupported,
                    FrontendErrorCode::new("QASM-V001"),
                    format!(
                        "OpenQASM major version {} is not supported; \
                         this importer supports OpenQASM 3.x",
                        version.major()
                    ),
                )
                .context("format", "OpenQASM")
                .context(
                    "requested_version",
                    source_version.to_string(),
                )
                .context(
                    "supported_version",
                    self.version.to_string(),
                ),
            );
        }

        if source_version.is_newer_than(self.version) {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Unsupported,
                    FrontendErrorCode::new("QASM-V001"),
                    format!(
                        "OpenQASM version {}.{} is newer than \
                         the configured importer version {}",
                        version.major(),
                        version.minor(),
                        self.version,
                    ),
                )
                .context("format", "OpenQASM")
                .context(
                    "requested_version",
                    source_version.to_string(),
                )
                .context(
                    "supported_version",
                    self.version.to_string(),
                ),
            );
        }

        Ok(())
    }

    fn validate(
        &self,
        program: &Program,
        limits: &FrontendLimits,
    ) -> FrontendResult<DiagnosticBag> {
        let result =
            validate_program_with_config(
                program,
                limits,
                self.validation,
            );

        let max_diagnostics =
            usize_from_u64(
                limits.max_diagnostics(),
                "max_diagnostics",
            )?;

        let mut diagnostics =
            DiagnosticBag::with_max_diagnostics(
                max_diagnostics,
            );

        if result.is_valid() {
            return Ok(diagnostics);
        }

        /*
         * Validation errors are fatal. We still construct the complete
         * bounded diagnostic bag first so callers get deterministic,
         * structured information rather than only the first textual error.
         */
        for error in result.errors() {
            push_validation_diagnostic(
                &mut diagnostics,
                error,
            )?;
        }

        let first =
            result.errors().first().ok_or_else(|| {
                FrontendError::internal(
                    "OpenQASM validator reported an invalid result \
                     without a semantic error",
                )
            })?;

        Err(
            FrontendError::with_code(
                FrontendErrorKind::Semantic,
                FrontendErrorCode::new(
                    first.code().as_str(),
                ),
                first.message(),
            )
            .context("format", "OpenQASM")
            .context(
                "stage",
                "semantic-validation",
            )
            .context(
                "diagnostic_count",
                result.errors().len().to_string(),
            ),
        )
    }

    fn lower(
        &self,
        program: &Program,
        limits: &FrontendLimits,
    ) -> FrontendResult<QuantumCircuit> {
        OpenQasmLowerer::new(
            program,
            limits,
        )?
        .lower()
    }
}

impl FormatImporter for OpenQasmImporter {
    fn format(&self) -> FormatId {
        FormatId::new("openqasm")
            .expect(
                "the built-in OpenQASM format identifier \
                 must satisfy FormatId invariants",
            )
    }

    fn version(&self) -> FormatVersion {
        self.version
    }

    fn import(
        &self,
        input: ImportInput,
    ) -> ImportResult {
        let limits = input.config().limits();

        /*
         * OpenQASM source is UTF-8. Decode before invoking the parser so the
         * parser never receives malformed text and cannot accidentally
         * interpret replacement characters as source.
         */
        let source =
            std::str::from_utf8(input.source())
                .map_err(|error| {
                    FrontendError::with_code(
                        FrontendErrorKind::Lexical,
                        FrontendErrorCode::new(
                            "QASM-P020",
                        ),
                        format!(
                            "OpenQASM source must be valid UTF-8: {error}"
                        ),
                    )
                    .context("format", "OpenQASM")
                    .context(
                        "stage",
                        "source-decoding",
                    )
                })?;

        let program =
            self.parse(
                source,
                input.source_id(),
                limits,
            )?;

        self.check_version(&program)?;

        /*
         * Validation must complete before lowering. This prevents malformed
         * semantic constructs from reaching canonical IR construction.
         */
        let diagnostics =
            self.validate(
                &program,
                limits,
            )?;

        let circuit =
            self.lower(
                &program,
                limits,
            )?;

        /*
         * ImportOutput::try_new is the final generic frontend boundary. It
         * validates the canonical IR again and therefore protects the public
         * success invariant even if lowering later changes.
         */
        ImportOutput::try_new(
            circuit,
            self.format(),
            self.version(),
            diagnostics,
        )
    }
}

// =============================================================================
// Lowering
// =============================================================================

struct OpenQasmLowerer<'a> {
    program: &'a Program,
    frontend_limits: &'a FrontendLimits,

    quantum_registers:
        BTreeMap<String, RegisterRange>,

    classical_registers:
        BTreeMap<String, RegisterRange>,

    constants:
        HashMap<String, f64>,

    next_qubit: usize,
    next_classical_bit: usize,

    operations: Vec<Gate>,
}

#[derive(Clone, Copy, Debug)]
struct RegisterRange {
    base: usize,
    len: usize,
}

impl RegisterRange {
    fn checked_end(
        self,
    ) -> FrontendResult<usize> {
        self.base
            .checked_add(self.len)
            .ok_or_else(|| {
                FrontendError::internal(
                    "OpenQASM register offset overflowed",
                )
            })
    }

    fn index(
        self,
        index: usize,
    ) -> FrontendResult<usize> {
        if index >= self.len {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new(
                        "QASM-L001",
                    ),
                    format!(
                        "register index {} is outside \
                         register length {}",
                        index,
                        self.len,
                    ),
                )
                .context("stage", "lowering"),
            );
        }

        self.base
            .checked_add(index)
            .ok_or_else(|| {
                FrontendError::internal(
                    "OpenQASM register index overflowed",
                )
            })
    }
}

impl<'a> OpenQasmLowerer<'a> {
    fn new(
        program: &'a Program,
        frontend_limits: &'a FrontendLimits,
    ) -> FrontendResult<Self> {
        Ok(Self {
            program,
            frontend_limits,

            quantum_registers: BTreeMap::new(),
            classical_registers: BTreeMap::new(),

            constants: HashMap::new(),

            next_qubit: 0,
            next_classical_bit: 0,

            operations: Vec::new(),
        })
    }

    fn lower(
        mut self,
    ) -> FrontendResult<QuantumCircuit> {
        self.collect_declarations()?;
        self.lower_statements()?;

        let ir_limits =
            QuantumIrLimits::production();

        if self.next_qubit
            > ir_limits.max_qubits()
        {
            return Err(
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_qubits",
                        self.next_qubit,
                        ir_limits.max_qubits(),
                    ),
                )
                .context("format", "OpenQASM"),
            );
        }

        if self.next_classical_bit
            > ir_limits.max_classical_bits()
        {
            return Err(
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_classical_bits",
                        self.next_classical_bit,
                        ir_limits.max_classical_bits(),
                    ),
                )
                .context("format", "OpenQASM"),
            );
        }

        QuantumCircuit::from_operations_with_limits(
            self.next_qubit,
            self.next_classical_bit,
            self.operations,
            ir_limits,
        )
        .map_err(|error| {
            FrontendError::lowering(format!(
                "canonical Quantum IR rejected \
                 OpenQASM import: {error}"
            ))
            .context("format", "OpenQASM")
            .context("stage", "lowering")
        })
    }

    fn collect_declarations(
        &mut self,
    ) -> FrontendResult<()> {
        for statement in self.program.statements() {
            match statement {
                Statement::QuantumDeclaration(
                    declaration,
                ) => {
                    self.collect_quantum_declaration(
                        declaration,
                    )?;
                }

                Statement::ClassicalDeclaration(
                    declaration,
                ) => {
                    self.collect_classical_declaration(
                        declaration,
                    )?;
                }

                Statement::ConstDeclaration(
                    declaration,
                ) => {
                    self.collect_constant(
                        declaration,
                    )?;
                }

                Statement::Include(include) => {
                    if include.path()
                        != STANDARD_LIBRARY_INCLUDE
                    {
                        return Err(
                            FrontendError::unsupported(
                                format!(
                                    "OpenQASM include `{}` \
                                     requires an explicit \
                                     include resolver",
                                    include.path(),
                                ),
                            )
                            .context(
                                "format",
                                "OpenQASM",
                            )
                            .context(
                                "stage",
                                "lowering",
                            ),
                        );
                    }
                }

                /*
                 * Declarations handled in later phases or intentionally
                 * unsupported constructs are not lowered here.
                 */
                _ => {}
            }
        }

        Ok(())
    }

    fn collect_quantum_declaration(
        &mut self,
        declaration: &QuantumDeclaration,
    ) -> FrontendResult<()> {
        let length =
            match declaration.ty() {
                QuantumType::Qubit(size)
                | QuantumType::QReg(size) => {
                    match size {
                        Some(expression) => {
                            self.eval_size_expression(
                                expression,
                            )?
                        }
                        None => 1,
                    }
                }
            };

        self.check_register_size(length)?;

        let base =
            self.next_qubit;

        let end =
            base.checked_add(length)
                .ok_or_else(|| {
                    FrontendError::internal(
                        "logical qubit register \
                         allocation overflowed",
                    )
                })?;

        self.next_qubit = end;

        self.quantum_registers.insert(
            declaration.name().as_str().to_owned(),
            RegisterRange {
                base,
                len: length,
            },
        );

        Ok(())
    }

    fn collect_classical_declaration(
        &mut self,
        declaration: &ClassicalDeclaration,
    ) -> FrontendResult<()> {
        match declaration.ty() {
            ScalarType::Bit(size) => {
                let length =
                    match size {
                        Some(expression) => {
                            self.eval_size_expression(
                                expression,
                            )?
                        }
                        None => 1,
                    };

                self.check_register_size(length)?;

                let base =
                    self.next_classical_bit;

                let end =
                    base.checked_add(length)
                        .ok_or_else(|| {
                            FrontendError::internal(
                                "classical register \
                                 allocation overflowed",
                            )
                        })?;

                self.next_classical_bit =
                    end;

                self.classical_registers
                    .insert(
                        declaration
                            .name()
                            .as_str()
                            .to_owned(),
                        RegisterRange {
                            base,
                            len: length,
                        },
                    );
            }

            _ => {
                return Err(
                    FrontendError::unsupported(
                        format!(
                            "classical type `{}` has no \
                             canonical Quantum IR \
                             declaration representation",
                            format!(
                                "{:?}",
                                declaration.ty()
                            ),
                        ),
                    )
                    .context(
                        "format",
                        "OpenQASM",
                    )
                    .context(
                        "stage",
                        "lowering",
                    ),
                );
            }
        }

        Ok(())
    }

    fn collect_constant(
        &mut self,
        declaration: &ConstDeclaration,
    ) -> FrontendResult<()> {
        let value =
            self.eval_numeric_expression(
                declaration.initializer(),
            )?;

        self.constants.insert(
            declaration
                .name()
                .as_str()
                .to_owned(),
            value,
        );

        Ok(())
    }

    fn lower_statements(
        &mut self,
    ) -> FrontendResult<()> {
        for statement in self.program.statements() {
            match statement {
                Statement::Include(_) => {}

                Statement::ConstDeclaration(_) => {}

                Statement::QuantumDeclaration(_) => {}

                Statement::ClassicalDeclaration(_) => {}

                Statement::GateCall(call) => {
                    self.lower_gate_call(call)?;
                }

                Statement::MeasureAssignment(
                    assignment,
                ) => {
                    self.lower_measurement_assignment(
                        assignment.source(),
                        assignment.destination(),
                    )?;
                }

                Statement::Reset(reset) => {
                    self.lower_reset(
                        reset.operands(),
                    )?;
                }

                Statement::Barrier(barrier) => {
                    self.lower_barrier(
                        barrier.operands(),
                    )?;
                }

                statement => {
                    return Err(
                        FrontendError::unsupported(
                            format!(
                                "OpenQASM statement `{}` \
                                 is not representable by \
                                 the current canonical \
                                 Quantum IR",
                                statement_name(
                                    statement
                                ),
                            ),
                        )
                        .context(
                            "format",
                            "OpenQASM",
                        )
                        .context(
                            "stage",
                            "lowering",
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    fn lower_gate_call(
        &mut self,
        call: &GateCall,
    ) -> FrontendResult<()> {
        if !call.modifiers().is_empty() {
            return Err(
                FrontendError::unsupported(
                    format!(
                        "gate modifiers on `{}` are not \
                         currently representable by the \
                         canonical Quantum IR",
                        call.name(),
                    ),
                )
                .context("format", "OpenQASM")
                .context("stage", "lowering"),
            );
        }

        let standard_gate =
            lookup_standard_gate(
                call.name().as_str(),
            )
            .ok_or_else(|| {
                FrontendError::unsupported(
                    format!(
                        "OpenQASM gate `{}` is not available \
                         in the standard-gate catalogue",
                        call.name(),
                    ),
                )
                .context("format", "OpenQASM")
                .context("stage", "lowering")
            })?;

        let kind =
            standard_gate.gate_kind().ok_or_else(
                || {
                    FrontendError::unsupported(
                        format!(
                            "OpenQASM gate `{}` has no direct \
                             canonical Quantum IR representation",
                            call.name(),
                        ),
                    )
                    .context(
                        "format",
                        "OpenQASM",
                    )
                    .context(
                        "stage",
                        "lowering",
                    )
                },
            )?;

        let parameters =
            self.lower_parameters(
                call.parameters(),
            )?;

        let operands =
            self.lower_gate_operands(
                call.operands(),
                standard_gate.qubit_count(),
            )?;

        for expanded in operands {
            let gate =
                Gate::new(
                    kind,
                    expanded,
                    parameters.clone(),
                    None,
                    None,
                )
                .map_err(|error| {
                    FrontendError::lowering(
                        format!(
                            "failed to construct canonical \
                             gate for OpenQASM `{}`: {error}",
                            call.name(),
                        ),
                    )
                })?;

            self.push_gate(gate)?;
        }

        Ok(())
    }

    fn lower_parameters(
        &self,
        expressions: &[Expression],
    ) -> FrontendResult<Vec<Parameter>> {
        let mut parameters =
            Vec::with_capacity(
                expressions.len(),
            );

        for expression in expressions {
            parameters.push(
                self.lower_parameter(
                    expression
                )?,
            );
        }

        Ok(parameters)
    }

    fn lower_parameter(
        &self,
        expression: &Expression,
    ) -> FrontendResult<Parameter> {
        if let Ok(value) =
            self.eval_numeric_expression(
                expression
            )
        {
            return Parameter::constant(value)
                .map_err(|error| {
                    FrontendError::lowering(
                        format!(
                            "invalid OpenQASM numeric \
                             parameter: {error}"
                        ),
                    )
                });
        }

        self.expression_to_parameter(
            expression
        )
    }

    fn expression_to_parameter(
        &self,
        expression: &Expression,
    ) -> FrontendResult<Parameter> {
        match expression {
            Expression::BoolLiteral { .. } => {
                Err(FrontendError::unsupported(
                    "boolean expression cannot be used \
                     as a quantum gate parameter",
                ))
            }

            Expression::IntegerLiteral {
                value,
                ..
            } => {
                let numeric =
                    parse_integer_literal(
                        value
                    )?;

                Parameter::constant(
                    numeric
                )
                .map_err(|error| {
                    FrontendError::lowering(
                        error.to_string()
                    )
                })
            }

            Expression::FloatLiteral {
                value,
                ..
            } => {
                let numeric =
                    value.raw()
                        .parse::<f64>()
                        .map_err(|error| {
                            FrontendError::lowering(
                                format!(
                                    "invalid OpenQASM \
                                     floating-point parameter: \
                                     {error}"
                                ),
                            )
                        })?;

                checked_finite(numeric)?;

                Parameter::constant(
                    numeric
                )
                .map_err(|error| {
                    FrontendError::lowering(
                        error.to_string()
                    )
                })
            }

            Expression::Identifier(
                identifier,
            ) => {
                if let Some(value) =
                    self.constants.get(
                        identifier.as_str()
                    )
                {
                    return Parameter::constant(
                        *value
                    )
                    .map_err(|error| {
                        FrontendError::lowering(
                            error.to_string()
                        )
                    });
                }

                match identifier.as_str() {
                    "pi" => {
                        Parameter::constant(
                            std::f64::consts::PI
                        )
                        .map_err(|error| {
                            FrontendError::lowering(
                                error.to_string()
                            )
                        })
                    }

                    "tau" => {
                        Parameter::constant(
                            std::f64::consts::TAU
                        )
                        .map_err(|error| {
                            FrontendError::lowering(
                                error.to_string()
                            )
                        })
                    }

                    _ => {
                        Parameter::symbol(
                            identifier.as_str()
                        )
                        .map_err(|error| {
                            FrontendError::lowering(
                                error.to_string()
                            )
                        })
                    }
                }
            }

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                let value =
                    self.expression_to_parameter(
                        operand
                    )?;

                match operator {
                    crate::quantum::frontend::formats::openqasm::ast::UnaryOperator::Plus => {
                        Ok(value)
                    }

                    crate::quantum::frontend::formats::openqasm::ast::UnaryOperator::Minus => {
                        Parameter::expression(
                            ParameterExpression::Negate(
                                Box::new(value)
                            )
                        )
                        .map_err(|error| {
                            FrontendError::lowering(
                                error.to_string()
                            )
                        })
                    }

                    _ => Err(
                        FrontendError::unsupported(
                            "logical or bitwise unary \
                             operator cannot be used as a \
                             canonical quantum parameter",
                        )
                    ),
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let lhs =
                    self.expression_to_parameter(
                        left
                    )?;

                let rhs =
                    self.expression_to_parameter(
                        right
                    )?;

                let expression =
                    match operator {
                        BinaryOperator::Add =>
                            ParameterExpression::Add(
                                Box::new(lhs),
                                Box::new(rhs),
                            ),

                        BinaryOperator::Subtract =>
                            ParameterExpression::Subtract(
                                Box::new(lhs),
                                Box::new(rhs),
                            ),

                        BinaryOperator::Multiply =>
                            ParameterExpression::Multiply(
                                Box::new(lhs),
                                Box::new(rhs),
                            ),

                        BinaryOperator::Divide =>
                            ParameterExpression::Divide(
                                Box::new(lhs),
                                Box::new(rhs),
                            ),

                        _ => {
                            return Err(
                                FrontendError::unsupported(
                                    "comparison, logical, bitwise, \
                                     or unsupported power expression \
                                     cannot be represented as a \
                                     canonical quantum parameter",
                                )
                            );
                        }
                    };

                Parameter::expression(
                    expression
                )
                .map_err(|error| {
                    FrontendError::lowering(
                        error.to_string()
                    )
                })
            }

            Expression::Parenthesized {
                expression,
                ..
            } => {
                self.expression_to_parameter(
                    expression
                )
            }

            Expression::FunctionCall {
                name,
                ..
            } => {
                Err(
                    FrontendError::unsupported(
                        format!(
                            "OpenQASM parameter function `{}` \
                             is not currently representable by \
                             the canonical Quantum IR parameter \
                             expression",
                            name,
                        ),
                    )
                )
            }

            _ => Err(
                FrontendError::unsupported(
                    "OpenQASM expression cannot be represented \
                     as a canonical quantum parameter",
                )
            ),
        }
    }

    fn lower_gate_operands(
        &self,
        operands: &[GateOperand],
        expected: usize,
    ) -> FrontendResult<Vec<Vec<QubitId>>> {
        if operands.len() != expected {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new(
                        "QASM-G001",
                    ),
                    format!(
                        "gate requires {} operands, \
                         received {}",
                        expected,
                        operands.len(),
                    ),
                )
                .context("stage", "lowering"),
            );
        }

        let mut expanded =
            Vec::with_capacity(
                expected
            );

        for operand in operands {
            expanded.push(
                self.expand_quantum_operand(
                    operand
                )?
            );
        }

        let broadcast_len =
            expanded
                .iter()
                .map(Vec::len)
                .max()
                .unwrap_or(0);

        if broadcast_len == 0 {
            return Err(
                FrontendError::lowering(
                    "OpenQASM gate operand expansion \
                     produced no qubits",
                )
            );
        }

        for values in &expanded {
            if values.len() != 1
                && values.len() != broadcast_len
            {
                return Err(
                    FrontendError::with_code(
                        FrontendErrorKind::Lowering,
                        FrontendErrorCode::new(
                            "QASM-Q004",
                        ),
                        "OpenQASM register operands have \
                         incompatible broadcast lengths",
                    )
                );
            }
        }

        let mut result =
            Vec::with_capacity(
                broadcast_len
            );

        for index in 0..broadcast_len {
            let mut gate_operands =
                Vec::with_capacity(
                    expected
                );

            for values in &expanded {
                gate_operands.push(
                    if values.len() == 1 {
                        values[0]
                    } else {
                        values[index]
                    }
                );
            }

            result.push(gate_operands);
        }

        Ok(result)
    }

    fn expand_quantum_operand(
        &self,
        operand: &GateOperand,
    ) -> FrontendResult<Vec<QubitId>> {
        match operand {
            GateOperand::Physical(
                physical
            ) => {
                Err(
                    FrontendError::unsupported(
                        format!(
                            "physical qubit ${} cannot be lowered \
                             into hardware-independent logical Quantum IR",
                            physical.index(),
                        )
                    )
                )
            }

            GateOperand::Alias(alias) => {
                Err(
                    FrontendError::unsupported(
                        format!(
                            "OpenQASM alias `{}` is not currently \
                             representable by the canonical Quantum IR",
                            alias,
                        )
                    )
                )
            }

            GateOperand::Designator(
                designator
            ) => {
                self.expand_quantum_designator(
                    designator
                )
            }
        }
    }

    fn expand_quantum_designator(
        &self,
        designator: &Designator,
    ) -> FrontendResult<Vec<QubitId>> {
        let register =
            self.quantum_registers
                .get(
                    designator.name().as_str()
                )
                .ok_or_else(|| {
                    FrontendError::with_code(
                        FrontendErrorKind::Lowering,
                        FrontendErrorCode::new(
                            "QASM-Q001",
                        ),
                        format!(
                            "quantum register `{}` is not declared",
                            designator.name(),
                        ),
                    )
                })?;

        match designator.index() {
            None => {
                let end =
                    register.checked_end()?;

                let mut result =
                    Vec::with_capacity(
                        register.len
                    );

                for index in
                    register.base..end
                {
                    result.push(
                        QubitId::new(index)
                    );
                }

                Ok(result)
            }

            Some(
                IndexExpression::Index(
                    expression
                )
            ) => {
                let index =
                    self.eval_index_expression(
                        expression
                    )?;

                Ok(vec![
                    QubitId::new(
                        register.index(index)?
                    )
                ])
            }

            Some(_) => {
                Err(
                    FrontendError::unsupported(
                        "OpenQASM slices, ranges, index sets, \
                         and register concatenation are not currently \
                         representable at the canonical logical-qubit boundary",
                    )
                )
            }
        }
    }

    fn lower_measurement_assignment(
        &mut self,
        source: &MeasureExpression,
        destination: &Designator,
    ) -> FrontendResult<()> {
        let source_qubits =
            self.expand_quantum_designator(
                source.operand()
            )?;

        let destination_bits =
            self.expand_classical_designator(
                destination
            )?;

        if source_qubits.len()
            != destination_bits.len()
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new(
                        "QASM-M003",
                    ),
                    "measurement source and destination \
                     registers must have equal length",
                )
            );
        }

        for (
            qubit,
            classical_bit,
        ) in source_qubits
            .into_iter()
            .zip(destination_bits)
        {
            let measurement =
                Measurement::new(
                    qubit,
                    classical_bit,
                );

            let gate =
                Gate::new(
                    GateKind::Measure,
                    vec![qubit],
                    Vec::new(),
                    Some(classical_bit),
                    Some(measurement),
                )
                .map_err(|error| {
                    FrontendError::lowering(
                        format!(
                            "failed to construct canonical \
                             measurement: {error}"
                        ),
                    )
                })?;

            self.push_gate(gate)?;
        }

        Ok(())
    }

    fn expand_classical_designator(
        &self,
        designator: &Designator,
    ) -> FrontendResult<Vec<usize>> {
        let register =
            self.classical_registers
                .get(
                    designator.name().as_str()
                )
                .ok_or_else(|| {
                    FrontendError::with_code(
                        FrontendErrorKind::Lowering,
                        FrontendErrorCode::new(
                            "QASM-M003",
                        ),
                        format!(
                            "classical register `{}` is not declared",
                            designator.name(),
                        ),
                    )
                })?;

        match designator.index() {
            None => {
                let end =
                    register.checked_end()?;

                Ok(
                    (register.base..end)
                        .collect()
                )
            }

            Some(
                IndexExpression::Index(
                    expression
                )
            ) => {
                let index =
                    self.eval_index_expression(
                        expression
                    )?;

                Ok(vec![
                    register.index(index)?
                ])
            }

            Some(_) => {
                Err(
                    FrontendError::unsupported(
                        "OpenQASM classical slices, ranges, \
                         index sets, and concatenation are not currently \
                         representable by the canonical measurement boundary",
                    )
                )
            }
        }
    }

    fn lower_reset(
        &mut self,
        operands: &[GateOperand],
    ) -> FrontendResult<()> {
        for operand in operands {
            let qubits =
                self.expand_quantum_operand(
                    operand
                )?;

            for qubit in qubits {
                let gate =
                    Gate::new(
                        GateKind::Reset,
                        vec![qubit],
                        Vec::new(),
                        None,
                        None,
                    )
                    .map_err(|error| {
                        FrontendError::lowering(
                            format!(
                                "failed to construct canonical \
                                 reset: {error}"
                            ),
                        )
                    })?;

                self.push_gate(gate)?;
            }
        }

        Ok(())
    }

    fn lower_barrier(
        &mut self,
        operands: &[GateOperand],
    ) -> FrontendResult<()> {
        let mut qubits =
            Vec::new();

        for operand in operands {
            qubits.extend(
                self.expand_quantum_operand(
                    operand
                )?
            );
        }

        if qubits.is_empty() {
            return Err(
                FrontendError::lowering(
                    "OpenQASM barrier contains no logical qubits",
                )
            );
        }

        let gate =
            Gate::new(
                GateKind::Barrier,
                qubits,
                Vec::new(),
                None,
                None,
            )
            .map_err(|error| {
                FrontendError::lowering(
                    format!(
                        "failed to construct canonical barrier: {error}"
                    ),
                )
            })?;

        self.push_gate(gate)
    }

    fn push_gate(
        &mut self,
        gate: Gate,
    ) -> FrontendResult<()> {
        let next =
            self.operations
                .len()
                .checked_add(1)
                .ok_or_else(|| {
                    FrontendError::internal(
                        "OpenQASM operation count overflowed",
                    )
                })?;

        let maximum =
            usize_from_u64(
                self.frontend_limits
                    .max_gate_operations(),
                "max_gate_operations",
            )?;

        if next > maximum {
            return Err(
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_gate_operations",
                        next,
                        maximum,
                    ),
                )
                .context("format", "OpenQASM")
            );
        }

        self.operations.push(gate);

        Ok(())
    }

    fn check_register_size(
        &self,
        size: usize,
    ) -> FrontendResult<()> {
        let maximum =
            usize_from_u64(
                self.frontend_limits
                    .max_register_size(),
                "max_register_size",
            )?;

        if size > maximum {
            return Err(
                FrontendError::limit_exceeded(
                    FrontendLimitViolation::new(
                        "max_register_size",
                        size,
                        maximum,
                    ),
                )
                .context("format", "OpenQASM")
            );
        }

        Ok(())
    }

    fn eval_size_expression(
        &self,
        expression: &Expression,
    ) -> FrontendResult<usize> {
        let value =
            self.eval_numeric_expression(
                expression
            )?;

        if !value.is_finite()
            || value < 0.0
            || value.fract() != 0.0
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new(
                        "QASM-T001",
                    ),
                    "OpenQASM register size must be \
                     a finite non-negative integer",
                )
            );
        }

        if value
            > usize::MAX as f64
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new(
                        "QASM-T001",
                    ),
                    "OpenQASM register size exceeds \
                     the host index range",
                )
            );
        }

        Ok(value as usize)
    }

    fn eval_index_expression(
        &self,
        expression: &Expression,
    ) -> FrontendResult<usize> {
        self.eval_size_expression(
            expression
        )
    }

    fn eval_numeric_expression(
        &self,
        expression: &Expression,
    ) -> FrontendResult<f64> {
        match expression {
            Expression::IntegerLiteral {
                value,
                ..
            } => {
                parse_integer_literal(value)
            }

            Expression::FloatLiteral {
                value,
                ..
            } => {
                let number =
                    value.raw()
                        .parse::<f64>()
                        .map_err(|error| {
                            FrontendError::with_code(
                                FrontendErrorKind::Lowering,
                                FrontendErrorCode::new(
                                    "QASM-E005",
                                ),
                                format!(
                                    "invalid OpenQASM numeric \
                                     literal: {error}"
                                ),
                            )
                        })?;

                checked_finite(number)
            }

            Expression::Identifier(
                identifier
            ) => {
                if let Some(value) =
                    self.constants.get(
                        identifier.as_str()
                    )
                {
                    return Ok(*value);
                }

                match identifier.as_str() {
                    "pi" => {
                        Ok(std::f64::consts::PI)
                    }

                    "tau" => {
                        Ok(std::f64::consts::TAU)
                    }

                    _ => Err(
                        FrontendError::with_code(
                            FrontendErrorKind::Lowering,
                            FrontendErrorCode::new(
                                "QASM-E001",
                            ),
                            format!(
                                "OpenQASM identifier `{}` \
                                 is not a compile-time numeric constant",
                                identifier,
                            ),
                        )
                    )
                }
            }

            Expression::Unary {
                operator,
                operand,
                ..
            } => {
                let value =
                    self.eval_numeric_expression(
                        operand
                    )?;

                match operator {
                    crate::quantum::frontend::formats::openqasm::ast::UnaryOperator::Plus => {
                        Ok(value)
                    }

                    crate::quantum::frontend::formats::openqasm::ast::UnaryOperator::Minus => {
                        checked_finite(-value)
                    }

                    _ => {
                        Err(
                            FrontendError::unsupported(
                                "logical or bitwise unary operation \
                                 is not a numeric compile-time expression",
                            )
                        )
                    }
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let lhs =
                    self.eval_numeric_expression(
                        left
                    )?;

                let rhs =
                    self.eval_numeric_expression(
                        right
                    )?;

                let value =
                    match operator {
                        BinaryOperator::Add =>
                            lhs + rhs,

                        BinaryOperator::Subtract =>
                            lhs - rhs,

                        BinaryOperator::Multiply =>
                            lhs * rhs,

                        BinaryOperator::Divide => {
                            if rhs == 0.0 {
                                return Err(
                                    FrontendError::with_code(
                                        FrontendErrorKind::Lowering,
                                        FrontendErrorCode::new(
                                            "QASM-E006",
                                        ),
                                        "division by zero in \
                                         OpenQASM constant expression",
                                    )
                                );
                            }

                            lhs / rhs
                        }

                        BinaryOperator::Power =>
                            lhs.powf(rhs),

                        _ => {
                            return Err(
                                FrontendError::unsupported(
                                    "comparison, logical, or bitwise \
                                     expression is not a numeric \
                                     compile-time expression",
                                )
                            );
                        }
                    };

                checked_finite(value)
            }

            Expression::Parenthesized {
                expression,
                ..
            } => {
                self.eval_numeric_expression(
                    expression
                )
            }

            _ => {
                Err(
                    FrontendError::unsupported(
                        "OpenQASM expression cannot be evaluated \
                         as a compile-time numeric value",
                    )
                )
            }
        }
    }
}

// =============================================================================
// Limit conversion
// =============================================================================

fn parser_limits_from_frontend(
    limits: &FrontendLimits,
) -> FrontendResult<ParserLimits> {
    Ok(ParserLimits {
        max_tokens: usize_from_u64(
            limits.max_tokens(),
            "max_tokens",
        )?,

        max_statements_per_scope:
            usize_from_u64(
                limits.max_statements_per_block(),
                "max_statements_per_block",
            )?,

        max_ast_nodes:
            usize_from_u64(
                limits.max_ast_nodes(),
                "max_ast_nodes",
            )?,

        max_nesting_depth:
            usize_from_u64(
                limits.max_nesting_depth(),
                "max_nesting_depth",
            )?,

        max_expression_depth:
            usize_from_u64(
                limits.max_expression_depth(),
                "max_expression_depth",
            )?,

        max_gate_parameters:
            usize_from_u64(
                limits.max_parameters(),
                "max_parameters",
            )?,

        max_gate_operands:
            usize_from_u64(
                limits.max_operands(),
                "max_operands",
            )?,

        max_arguments:
            usize_from_u64(
                limits.max_parameters(),
                "max_parameters",
            )?,

        max_switch_cases:
            usize_from_u64(
                limits.max_operands(),
                "max_operands",
            )?,
    })
}

fn usize_from_u64(
    value: u64,
    field: &'static str,
) -> FrontendResult<usize> {
    usize::try_from(value)
        .map_err(|_| {
            FrontendError::internal(
                format!(
                    "frontend limit `{field}` cannot be represented \
                     by this target's usize",
                ),
            )
        })
}

// =============================================================================
// Numeric conversion
// =============================================================================

fn checked_finite(
    value: f64,
) -> FrontendResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(
            FrontendError::with_code(
                FrontendErrorKind::Lowering,
                FrontendErrorCode::new(
                    "QASM-E004",
                ),
                "OpenQASM numeric expression evaluated \
                 to a non-finite value",
            )
        )
    }
}

fn parse_integer_literal(
    literal:
        &crate::quantum::frontend::formats::openqasm::ast::IntegerLiteral,
) -> FrontendResult<f64> {
    let raw = literal.raw();

    let value =
        match literal.radix() {
            crate::quantum::frontend::formats::openqasm::ast::IntegerRadix::Decimal => {
                u128::from_str_radix(
                    raw,
                    10
                )
            }

            crate::quantum::frontend::formats::openqasm::ast::IntegerRadix::Binary => {
                u128::from_str_radix(
                    raw.trim_start_matches(
                        "0b"
                    ),
                    2
                )
            }

            crate::quantum::frontend::formats::openqasm::ast::IntegerRadix::Octal => {
                u128::from_str_radix(
                    raw.trim_start_matches(
                        "0o"
                    ),
                    8
                )
            }

            crate::quantum::frontend::formats::openqasm::ast::IntegerRadix::Hexadecimal => {
                u128::from_str_radix(
                    raw.trim_start_matches(
                        "0x"
                    ),
                    16
                )
            }
        }
        .map_err(|error| {
            FrontendError::with_code(
                FrontendErrorKind::Lowering,
                FrontendErrorCode::new(
                    "QASM-E005",
                ),
                format!(
                    "invalid OpenQASM integer literal `{raw}`: {error}"
                ),
            )
        })?;

    /*
     * Do not allow a large integer to silently wrap when converted to f64.
     * The canonical parameter representation currently uses f64, so values
     * beyond the exactly representable integer range are rejected rather than
     * changing their mathematical meaning during lowering.
     */
    if value > (1_u128 << 53) {
        return Err(
            FrontendError::with_code(
                FrontendErrorKind::Lowering,
                FrontendErrorCode::new(
                    "QASM-E007",
                ),
                "OpenQASM integer literal is too large \
                 to be represented exactly by the current \
                 canonical floating-point parameter representation",
            )
        );
    }

    checked_finite(
        value as f64
    )
}

// =============================================================================
// Diagnostics
// =============================================================================

fn push_validation_diagnostic(
    diagnostics: &mut DiagnosticBag,
    error: &ValidationError,
) -> FrontendResult<()> {
    let code =
        DiagnosticCode::new(
            error.code()
                .as_str()
                .to_owned(),
        )
        .ok_or_else(|| {
            FrontendError::internal(
                "OpenQASM validation produced \
                 an invalid diagnostic code",
            )
        })?;

    let diagnostic =
        Diagnostic::new(
            DiagnosticSeverity::Error,
            code,
            error.message(),
        );

    let diagnostic =
        diagnostic
            .primary_label(
                error.span(),
                "OpenQASM semantic error",
            )
            .map_err(|error| {
                FrontendError::internal(
                    format!(
                        "failed to construct OpenQASM \
                         diagnostic: {error}"
                    ),
                )
            })?
            .build();

    diagnostics
        .push(diagnostic)
        .map_err(|error| {
            FrontendError::with_code(
                FrontendErrorKind::Diagnostic,
                FrontendErrorCode::new(
                    "QASM-D001",
                ),
                format!(
                    "unable to retain OpenQASM diagnostic: {error}"
                ),
            )
        })?;

    Ok(())
}

// =============================================================================
// Statement names
// =============================================================================

fn statement_name(
    statement: &Statement,
) -> &'static str {
    match statement {
        Statement::Include(_) =>
            "include",

        Statement::CalibrationGrammar(_) =>
            "defcalgrammar",

        Statement::Pragma(_) =>
            "pragma",

        Statement::Annotated(_) =>
            "annotation",

        Statement::ClassicalDeclaration(_) =>
            "classical declaration",

        Statement::ConstDeclaration(_) =>
            "const declaration",

        Statement::QuantumDeclaration(_) =>
            "quantum declaration",

        Statement::OldStyleDeclaration(_) =>
            "legacy declaration",

        Statement::AliasDeclaration(_) =>
            "alias declaration",

        Statement::IoDeclaration(_) =>
            "I/O declaration",

        Statement::GateDefinition(_) =>
            "gate definition",

        Statement::DefDefinition(_) =>
            "subroutine definition",

        Statement::ExternDeclaration(_) =>
            "extern declaration",

        Statement::GateCall(_) =>
            "gate call",

        Statement::Assignment(_) =>
            "assignment",

        Statement::Expression(_) =>
            "expression",

        Statement::MeasureAssignment(_) =>
            "measurement assignment",

        Statement::Reset(_) =>
            "reset",

        Statement::Barrier(_) =>
            "barrier",

        Statement::Delay(_) =>
            "delay",

        Statement::Box(_) =>
            "box",

        Statement::If(_) =>
            "if",

        Statement::For(_) =>
            "for",

        Statement::While(_) =>
            "while",

        Statement::Switch(_) =>
            "switch",

        Statement::Break(_) =>
            "break",

        Statement::Continue(_) =>
            "continue",

        Statement::End(_) =>
            "end",

        Statement::Return(_) =>
            "return",

        Statement::Cal(_) =>
            "cal",

        Statement::Defcal(_) =>
            "defcal",

        Statement::Nop(_) =>
            "nop",

        Statement::Extension(_) =>
            "extension",
    }
}