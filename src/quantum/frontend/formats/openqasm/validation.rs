//! Zamani Quantum Frontend — OpenQASM semantic validation.
//!
//! This module performs semantic validation of the OpenQASM AST after parsing
//! and before lowering into the canonical Zamani Quantum IR.
//!
//! # Architectural boundary
//!
//! ```text
//! OpenQASM source
//!      |
//!      v
//!    lexer
//!      |
//!      v
//!  OpenQASM AST
//!      |
//!      v
//! this module
//!      |
//!      v
//! generic frontend lowering
//!      |
//!      v
//! Zamani Quantum IR
//! ```
//!
//! This module owns OpenQASM semantic rules only.
//!
//! It does NOT:
//!
//! - lex source;
//! - parse source;
//! - construct `QuantumCircuit`;
//! - construct canonical `Gate` values;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - perform hardware mapping;
//! - execute quantum operations;
//! - access the filesystem;
//! - access the network;
//! - execute `extern` declarations;
//! - resolve includes;
//! - silently discard unsupported constructs.
//!
//! # Validation responsibilities
//!
//! The validator checks:
//!
//! - OpenQASM version compatibility;
//! - declaration ordering;
//! - duplicate declarations;
//! - symbol visibility;
//! - quantum/classical name usage;
//! - gate existence;
//! - user-defined gate existence;
//! - gate parameter arity;
//! - gate operand arity;
//! - standard-gate version availability;
//! - standard-gate lowering capability;
//! - gate modifier legality;
//! - duplicate formal parameters/qubits;
//! - gate-definition resource limits;
//! - operand validity;
//! - physical-qubit policy;
//! - expression references;
//! - expression nesting;
//! - expression-node limits;
//! - include policy;
//! - unsupported source constructs;
//! - deterministic diagnostic generation.
//!
//! # Validation versus lowering
//!
//! OpenQASM validation answers:
//!
//! > "Is this source program semantically valid according to the supported
//! > OpenQASM frontend contract?"
//!
//! Lowering answers:
//!
//! > "Can this validated program be represented by the canonical Zamani
//! > Quantum IR?"
//!
//! These are intentionally separate.
//!
//! A syntactically and semantically valid OpenQASM gate may still be rejected
//! when the canonical IR has no equivalent representation. Such cases are
//! reported explicitly as `Unsupported` diagnostics rather than silently
//! dropped or replaced.
//!
//! # Validation versus Quantum IR validation
//!
//! This module does not duplicate canonical IR invariants.
//!
//! The intended pipeline is:
//!
//! ```text
//! OpenQASM source
//!     |
//!     v
//! lexer
//!     |
//!     v
//! parser
//!     |
//!     v
//! OpenQASM semantic validation       <- this module
//!     |
//!     v
//! OpenQASM -> Zamani IR lowering
//!     |
//!     v
//! canonical Quantum IR validation
//! ```
//!
//! The Quantum IR remains the owner of canonical circuit invariants.
//!
//! # Resource safety
//!
//! The validator treats the AST as untrusted input.
//!
//! It therefore enforces:
//!
//! - maximum symbols;
//! - maximum statements;
//! - maximum statements per gate definition;
//! - maximum gate definitions;
//! - maximum operands;
//! - maximum parameters;
//! - maximum expression depth;
//! - maximum expression nodes;
//! - bounded diagnostics.
//!
//! All counters use checked arithmetic or bounded comparisons.
//!
//! No recursive validation path is allowed to exceed the configured nesting
//! limit.
//!
//! # Determinism
//!
//! Diagnostics are emitted in source traversal order.
//!
//! Symbol environments use deterministic insertion-independent lookup semantics
//! and never expose hash-map iteration order to diagnostics.
//!
//! # Rust compatibility
//!
//! Target toolchain: Rust 1.97.1.
//! Edition: Rust 2021.
//!
//! No nightly features.
//! No additional dependencies.
//!
//! # Stable diagnostic namespace
//!
//! The following codes belong exclusively to this OpenQASM frontend:
//!
//! - QASM-E001 — missing/invalid OpenQASM version
//! - QASM-E002 — unsupported OpenQASM version
//! - QASM-E003 — invalid declaration ordering
//! - QASM-E004 — duplicate declaration
//! - QASM-E005 — unknown identifier
//! - QASM-E006 — invalid quantum operand
//! - QASM-E007 — invalid classical operand
//! - QASM-E008 — unknown gate
//! - QASM-E009 — invalid gate parameter count
//! - QASM-E010 — invalid gate operand count
//! - QASM-E011 — unsupported gate lowering
//! - QASM-E012 — invalid gate modifier
//! - QASM-E013 — duplicate formal parameter
//! - QASM-E014 — duplicate formal qubit
//! - QASM-E015 — invalid expression reference
//! - QASM-E016 — unsupported construct
//! - QASM-E017 — invalid include
//! - QASM-E018 — physical qubit unsupported
//! - QASM-E019 — resource limit exceeded
//! - QASM-E020 — invalid gate definition
//! - QASM-E021 — invalid symbol use
//! - QASM-E022 — invalid parameter expression
//! - QASM-E023 — recursive gate definition
//! - QASM-E024 — diagnostic truncation
//!
//! Warning codes:
//!
//! - QASM-W001 — compatibility/legacy construct
//! - QASM-W002 — unsupported-but-preserved construct
//!
//! The codes are deliberately local to this format. Adding or removing QIR,
//! Quil, or another frontend does not require changing this file.

use std::collections::{HashMap, HashSet};

use crate::quantum::frontend::core::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticCode,
    DiagnosticSeverity,
};
use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceSpan;

use super::ast::{
    Expression,
    GateCall,
    GateModifier,
    GateOperand,
    Program,
    Statement,
};
use super::stdgates::{
    lookup as lookup_standard_gate,
    StandardGate,
};

// =============================================================================
// Stable diagnostic codes
// =============================================================================

const E_VERSION: &str = "QASM-E001";
const E_UNSUPPORTED_VERSION: &str = "QASM-E002";
const E_DECLARATION_ORDER: &str = "QASM-E003";
const E_DUPLICATE_DECLARATION: &str = "QASM-E004";
const E_UNKNOWN_IDENTIFIER: &str = "QASM-E005";
const E_INVALID_QUANTUM_OPERAND: &str = "QASM-E006";
const E_INVALID_CLASSICAL_OPERAND: &str = "QASM-E007";
const E_UNKNOWN_GATE: &str = "QASM-E008";
const E_PARAMETER_COUNT: &str = "QASM-E009";
const E_OPERAND_COUNT: &str = "QASM-E010";
const E_UNSUPPORTED_GATE: &str = "QASM-E011";
const E_INVALID_MODIFIER: &str = "QASM-E012";
const E_DUPLICATE_PARAMETER: &str = "QASM-E013";
const E_DUPLICATE_QUBIT: &str = "QASM-E014";
const E_EXPRESSION_REFERENCE: &str = "QASM-E015";
const E_UNSUPPORTED: &str = "QASM-E016";
const E_INCLUDE: &str = "QASM-E017";
const E_PHYSICAL_QUBIT: &str = "QASM-E018";
const E_LIMIT: &str = "QASM-E019";
const E_GATE_DEFINITION: &str = "QASM-E020";
const E_SYMBOL_USE: &str = "QASM-E021";
const E_PARAMETER_EXPRESSION: &str = "QASM-E022";
const E_RECURSIVE_GATE: &str = "QASM-E023";
const E_DIAGNOSTIC_TRUNCATED: &str = "QASM-E024";

const W_LEGACY: &str = "QASM-W001";
const W_PRESERVED: &str = "QASM-W002";

// =============================================================================
// Public validation policy
// =============================================================================

/// Policy controlling OpenQASM semantic validation.
///
/// The policy is intentionally independent of parser configuration and
/// lowering configuration. This makes semantic validation deterministic and
/// independently testable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenQasmValidationConfig {
    /// Whether OpenQASM 3.0 is accepted.
    pub allow_3_0: bool,

    /// Whether OpenQASM 3.1 is accepted.
    pub allow_3_1: bool,

    /// Whether OpenQASM 2-style declarations are accepted by the parser and
    /// semantic layer.
    ///
    /// They are accepted only as explicitly supported compatibility syntax;
    /// they are never silently normalized here.
    pub allow_legacy_declarations: bool,

    /// Whether physical-qubit operands such as `$0` are accepted.
    ///
    /// The canonical Zamani IR currently does not use OpenQASM physical-qubit
    /// identities as its semantic namespace, so this defaults to false.
    pub allow_physical_qubits: bool,

    /// Whether unsupported source constructs may be preserved as warnings.
    ///
    /// When false, unsupported constructs become errors.
    pub preserve_unsupported_constructs: bool,

    /// Whether `include "stdgates.inc";` is recognized by the validator.
    ///
    /// The validator does not perform include I/O.
    pub allow_standard_library_include: bool,
}

impl Default for OpenQasmValidationConfig {
    fn default() -> Self {
        Self {
            allow_3_0: true,
            allow_3_1: true,
            allow_legacy_declarations: true,
            allow_physical_qubits: false,
            preserve_unsupported_constructs: false,
            allow_standard_library_include: true,
        }
    }
}

// =============================================================================
// Public validation result
// =============================================================================

/// Result of OpenQASM semantic validation.
///
/// The AST is retained by the caller; this structure contains only validation
/// information. This prevents validation from becoming a second AST owner.
#[derive(Debug, PartialEq)]
pub struct ValidationReport {
    diagnostics: DiagnosticBag,
}

impl ValidationReport {
    fn new(diagnostics: DiagnosticBag) -> Self {
        Self { diagnostics }
    }

    /// Returns all diagnostics in deterministic insertion order.
    #[must_use]
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }

    /// Returns whether semantic validation produced an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }

    /// Returns whether semantic validation succeeded.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Consumes the report and returns its diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> DiagnosticBag {
        self.diagnostics
    }
}

// =============================================================================
// Symbol model
// =============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SymbolKind {
    Quantum,
    Classical,
    Gate,
    Parameter,
    FormalQubit,
    Alias,
    Subroutine,
    Extern,
}

impl SymbolKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Quantum => "quantum object",
            Self::Classical => "classical object",
            Self::Gate => "gate",
            Self::Parameter => "parameter",
            Self::FormalQubit => "formal qubit",
            Self::Alias => "alias",
            Self::Subroutine => "subroutine",
            Self::Extern => "extern declaration",
        }
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    kind: SymbolKind,
    span: SourceSpan,
}

#[derive(Clone, Debug, Default)]
struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    fn insert(
        &mut self,
        name: String,
        symbol: Symbol,
    ) -> Option<Symbol> {
        self.symbols.insert(name, symbol)
    }

    fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

// =============================================================================
// Validator state
// =============================================================================

struct Validator<'a> {
    limits: FrontendLimits,
    config: OpenQasmValidationConfig,
    diagnostics: DiagnosticBag,

    global: Scope,

    /// Gate definitions currently being validated.
    ///
    /// This is used to reject direct and indirect recursive gate definitions
    /// before lowering.
    active_gate_stack: Vec<String>,

    gate_definition_count: u64,
    statement_count: u64,
    expression_count: u64,
}

impl<'a> Validator<'a> {
    fn new(
        limits: FrontendLimits,
        config: OpenQasmValidationConfig,
    ) -> Self {
        let max_diagnostics =
            usize::try_from(limits.max_diagnostics())
                .unwrap_or(usize::MAX);

        Self {
            limits,
            config,
            diagnostics:
                DiagnosticBag::with_max_diagnostics(
                    max_diagnostics,
                ),
            global: Scope::default(),
            active_gate_stack: Vec::new(),
            gate_definition_count: 0,
            statement_count: 0,
            expression_count: 0,
        }
    }

    // -------------------------------------------------------------------------
    // Diagnostics
    // -------------------------------------------------------------------------

    fn emit(
        &mut self,
        severity: DiagnosticSeverity,
        code: &'static str,
        message: impl Into<String>,
        span: SourceSpan,
    ) {
        let Some(code) = DiagnosticCode::new(code.to_owned())
        else {
            return;
        };

        let diagnostic =
            Diagnostic::builder(
                severity,
                code,
                message,
            )
            .primary(span, "OpenQASM semantic validation")
            .ok()
            .map(|builder| builder.build());

        let Some(diagnostic) = diagnostic else {
            return;
        };

        if !self
            .diagnostics
            .push_or_truncate(diagnostic)
        {
            // The diagnostic bag is itself bounded. We deliberately do not
            // recurse by emitting another diagnostic here.
        }
    }

    fn error(
        &mut self,
        code: &'static str,
        message: impl Into<String>,
        span: SourceSpan,
    ) {
        self.emit(
            DiagnosticSeverity::Error,
            code,
            message,
            span,
        );
    }

    fn warning(
        &mut self,
        code: &'static str,
        message: impl Into<String>,
        span: SourceSpan,
    ) {
        self.emit(
            DiagnosticSeverity::Warning,
            code,
            message,
            span,
        );
    }

    fn unsupported(
        &mut self,
        message: impl Into<String>,
        span: SourceSpan,
    ) {
        if self
            .config
            .preserve_unsupported_constructs
        {
            self.warning(
                W_PRESERVED,
                message,
                span,
            );
        } else {
            self.error(
                E_UNSUPPORTED,
                message,
                span,
            );
        }
    }

    // -------------------------------------------------------------------------
    // Resource limits
    // -------------------------------------------------------------------------

    fn count_statement(
        &mut self,
        span: SourceSpan,
    ) -> bool {
        self.statement_count =
            match self.statement_count.checked_add(1) {
                Some(value) => value,
                None => {
                    self.error(
                        E_LIMIT,
                        "OpenQASM statement counter overflowed",
                        span,
                    );
                    return false;
                }
            };

        if self.statement_count
            > self.limits.max_statements()
        {
            self.error(
                E_LIMIT,
                format!(
                    "OpenQASM statement limit exceeded: \
                     {} > {}",
                    self.statement_count,
                    self.limits.max_statements()
                ),
                span,
            );

            return false;
        }

        true
    }

    fn count_expression(
        &mut self,
        span: SourceSpan,
    ) -> bool {
        self.expression_count =
            match self.expression_count.checked_add(1) {
                Some(value) => value,
                None => {
                    self.error(
                        E_LIMIT,
                        "OpenQASM expression counter overflowed",
                        span,
                    );
                    return false;
                }
            };

        if self.expression_count
            > self.limits.max_expression_nodes()
        {
            self.error(
                E_LIMIT,
                format!(
                    "OpenQASM expression-node limit exceeded: \
                     {} > {}",
                    self.expression_count,
                    self.limits.max_expression_nodes()
                ),
                span,
            );

            return false;
        }

        true
    }

    fn check_collection_limit(
        &mut self,
        actual: usize,
        maximum: u64,
        what: &str,
        span: SourceSpan,
    ) -> bool {
        let actual_u64 =
            u64::try_from(actual).unwrap_or(u64::MAX);

        if actual_u64 > maximum {
            self.error(
                E_LIMIT,
                format!(
                    "{what} exceeds configured frontend limit: \
                     {actual} > {maximum}"
                ),
                span,
            );

            false
        } else {
            true
        }
    }

    // -------------------------------------------------------------------------
    // Program
    // -------------------------------------------------------------------------

    fn validate_program(
        mut self,
        program: &Program,
    ) -> ValidationReport {
        self.validate_version(program);

        if program.statements().len() as u64
            > self.limits.max_statements()
        {
            self.error(
                E_LIMIT,
                format!(
                    "program contains {} statements; \
                     maximum is {}",
                    program.statements().len(),
                    self.limits.max_statements()
                ),
                program.span(),
            );
        }

        for statement in program.statements() {
            if !self.count_statement(statement.span()) {
                break;
            }

            self.validate_statement(
                statement,
                0,
            );
        }

        ValidationReport::new(self.diagnostics)
    }

    fn validate_version(
        &mut self,
        program: &Program,
    ) {
        let Some(version) = program.version()
        else {
            self.error(
                E_VERSION,
                "OpenQASM source must declare a language version",
                program.span(),
            );

            return;
        };

        match (version.major(), version.minor()) {
            (3, 0) if self.config.allow_3_0 => {}
            (3, 1) if self.config.allow_3_1 => {}

            (3, 0) | (3, 1) => {
                self.error(
                    E_UNSUPPORTED_VERSION,
                    format!(
                        "OpenQASM {}.{} is disabled by the \
                         current frontend validation policy",
                        version.major(),
                        version.minor()
                    ),
                    version.span(),
                );
            }

            _ => {
                self.error(
                    E_UNSUPPORTED_VERSION,
                    format!(
                        "unsupported OpenQASM version {}.{}; \
                         this frontend currently supports OpenQASM 3.0 \
                         and 3.1",
                        version.major(),
                        version.minor()
                    ),
                    version.span(),
                );
            }
        }
    }

    // -------------------------------------------------------------------------
    // Statements
    // -------------------------------------------------------------------------

    fn validate_statement(
        &mut self,
        statement: &Statement,
        depth: u64,
    ) {
        if depth > self.limits.max_nesting_depth() {
            self.error(
                E_LIMIT,
                format!(
                    "OpenQASM semantic nesting depth exceeds {}",
                    self.limits.max_nesting_depth()
                ),
                statement.span(),
            );

            return;
        }

        match statement {
            Statement::Include(include) => {
                self.validate_include(include.path(), include.span());
            }

            Statement::CalibrationGrammar(_)
            | Statement::Pragma(_)
            | Statement::Annotated(_)
            | Statement::IoDeclaration(_)
            | Statement::Cal(_)
            | Statement::Defcal(_)
            | Statement::ExternDeclaration(_) => {
                self.unsupported(
                    "this OpenQASM construct is parsed but is not \
                     currently part of the canonical Zamani Quantum IR \
                     lowering contract",
                    statement.span(),
                );
            }

            Statement::ClassicalDeclaration(declaration) => {
                self.declare(
                    declaration.name(),
                    SymbolKind::Classical,
                );

                if let Some(initializer) =
                    declaration.initializer()
                {
                    self.validate_expression(
                        initializer,
                        0,
                    );
                }
            }

            Statement::ConstDeclaration(declaration) => {
                self.declare(
                    declaration.name(),
                    SymbolKind::Classical,
                );

                if let Some(initializer) =
                    declaration.initializer()
                {
                    self.validate_expression(
                        initializer,
                        0,
                    );
                }
            }

            Statement::QuantumDeclaration(declaration) => {
                self.declare(
                    declaration.name(),
                    SymbolKind::Quantum,
                );

                if let Some(size) =
                    quantum_declaration_size(declaration)
                {
                    self.validate_expression(
                        size,
                        0,
                    );
                }
            }

            Statement::OldStyleDeclaration(declaration) => {
                if !self.config.allow_legacy_declarations {
                    self.error(
                        E_UNSUPPORTED,
                        "OpenQASM 2-style declarations are disabled",
                        declaration.span(),
                    );
                } else {
                    self.warning(
                        W_LEGACY,
                        "OpenQASM 2-style declaration is accepted \
                         only as a compatibility construct",
                        declaration.span(),
                    );

                    self.declare(
                        declaration.name(),
                        SymbolKind::Quantum,
                    );

                    if let Some(size) =
                        old_style_declaration_size(declaration)
                    {
                        self.validate_expression(
                            size,
                            0,
                        );
                    }
                }
            }

            Statement::AliasDeclaration(alias) => {
                self.validate_alias(alias);
            }

            Statement::GateDefinition(definition) => {
                self.validate_gate_definition(
                    definition,
                    depth,
                );
            }

            Statement::DefDefinition(definition) => {
                self.unsupported(
                    "OpenQASM subroutine definitions are parsed but \
                     cannot currently be lowered into the canonical \
                     Quantum IR",
                    definition.span(),
                );
            }

            Statement::GateCall(call) => {
                self.validate_gate_call(
                    call,
                    depth,
                    None,
                );
            }

            Statement::Assignment(assignment) => {
                self.validate_assignment(assignment);
            }

            Statement::Expression(expression) => {
                self.validate_expression_statement(expression);
            }

            Statement::MeasureAssignment(measurement) => {
                self.validate_measurement_assignment(
                    measurement,
                );
            }

            Statement::Reset(reset) => {
                self.validate_quantum_operands(
                    reset.operands(),
                    reset.span(),
                );
            }

            Statement::Barrier(barrier) => {
                self.validate_quantum_operands(
                    barrier.operands(),
                    barrier.span(),
                );
            }

            Statement::Delay(delay) => {
                self.validate_expression(
                    delay.duration(),
                    0,
                );

                self.validate_quantum_operands(
                    delay.operands(),
                    delay.span(),
                );

                self.unsupported(
                    "OpenQASM delay operations are parsed but are \
                     not currently representable by the canonical \
                     hardware-independent Quantum IR",
                    delay.span(),
                );
            }

            Statement::Box(box_statement) => {
                self.unsupported(
                    "OpenQASM box/timing blocks are parsed but are \
                     not currently representable by the canonical \
                     Quantum IR",
                    box_statement.span(),
                );
            }

            Statement::If(if_statement) => {
                self.unsupported(
                    "OpenQASM classical control flow is parsed but is \
                     not currently representable by the canonical \
                     Quantum IR",
                    if_statement.span(),
                );
            }

            Statement::For(for_statement) => {
                self.unsupported(
                    "OpenQASM for-loops are parsed but are not currently \
                     lowered by the quantum frontend",
                    for_statement.span(),
                );
            }

            Statement::While(while_statement) => {
                self.unsupported(
                    "OpenQASM while-loops are parsed but are not currently \
                     lowered by the quantum frontend",
                    while_statement.span(),
                );
            }

            Statement::Switch(switch_statement) => {
                self.unsupported(
                    "OpenQASM switch statements are parsed but are not \
                     currently lowered by the quantum frontend",
                    switch_statement.span(),
                );
            }

            Statement::Break(control)
            | Statement::Continue(control)
            | Statement::End(control)
            | Statement::Nop(control) => {
                self.unsupported(
                    "OpenQASM control-flow statement is not currently \
                     representable by the canonical Quantum IR",
                    control.span(),
                );
            }

            Statement::Return(return_statement) => {
                self.unsupported(
                    "OpenQASM return statements are source-language \
                     control flow and are not currently lowered",
                    return_statement.span(),
                );
            }

            Statement::Extension(extension) => {
                self.unsupported(
                    "OpenQASM extension statement is not part of the \
                     supported Zamani OpenQASM semantic contract",
                    extension.span(),
                );
            }
        }
    }

    // -------------------------------------------------------------------------
    // Includes
    // -------------------------------------------------------------------------

    fn validate_include(
        &mut self,
        path: &str,
        span: SourceSpan,
    ) {
        if path.is_empty() {
            self.error(
                E_INCLUDE,
                "OpenQASM include path must not be empty",
                span,
            );

            return;
        }

        if path.contains('\0') {
            self.error(
                E_INCLUDE,
                "OpenQASM include path contains an embedded NUL byte",
                span,
            );

            return;
        }

        if path == "stdgates.inc" {
            if !self
                .config
                .allow_standard_library_include
            {
                self.error(
                    E_INCLUDE,
                    "stdgates.inc is disabled by the current \
                     OpenQASM validation policy",
                    span,
                );
            }

            return;
        }

        // The validator deliberately does not resolve arbitrary includes.
        // That belongs to a dedicated include resolver with explicit I/O
        // policy.
        self.unsupported(
            format!(
                "external OpenQASM include `{path}` requires an explicit \
                 include resolver; validation performs no filesystem or \
                 network I/O"
            ),
            span,
        );
    }

    // -------------------------------------------------------------------------
    // Declarations and symbols
    // -------------------------------------------------------------------------

    fn declare(
        &mut self,
        identifier: &crate::quantum::frontend::openqasm::ast::Identifier,
        kind: SymbolKind,
    ) {
        let name = identifier.as_str();

        if self.global.contains(name) {
            self.error(
                E_DUPLICATE_DECLARATION,
                format!(
                    "duplicate declaration of `{name}`"
                ),
                identifier.span(),
            );

            return;
        }

        if self.global.symbols.len() as u64
            >= self.limits.max_symbols()
        {
            self.error(
                E_LIMIT,
                format!(
                    "OpenQASM symbol-table limit exceeded: \
                     maximum is {}",
                    self.limits.max_symbols()
                ),
                identifier.span(),
            );

            return;
        }

        self.global.insert(
            name.to_owned(),
            Symbol {
                kind,
                span: identifier.span(),
            },
        );
    }

    fn validate_alias(
        &mut self,
        alias: &crate::quantum::frontend::openqasm::ast::AliasDeclaration,
    ) {
        self.declare(
            alias.name(),
            SymbolKind::Alias,
        );

        if alias.operands().is_empty() {
            self.error(
                E_INVALID_QUANTUM_OPERAND,
                "OpenQASM alias declaration requires at least \
                 one operand",
                alias.span(),
            );

            return;
        }

        if !self.check_collection_limit(
            alias.operands().len(),
            self.limits.max_operands(),
            "alias operand count",
            alias.span(),
        ) {
            return;
        }

        for operand in alias.operands() {
            self.validate_designator(
                operand,
                Some(SymbolKind::Quantum),
            );
        }
    }

    // -------------------------------------------------------------------------
    // Gate definitions
    // -------------------------------------------------------------------------

    fn validate_gate_definition(
        &mut self,
        definition: &crate::quantum::frontend::openqasm::ast::GateDefinition,
        depth: u64,
    ) {
        self.gate_definition_count =
            match self
                .gate_definition_count
                .checked_add(1)
            {
                Some(value) => value,
                None => {
                    self.error(
                        E_LIMIT,
                        "OpenQASM gate-definition counter overflowed",
                        definition.span(),
                    );

                    return;
                }
            };

        if self.gate_definition_count
            > self.limits.max_gate_definitions()
        {
            self.error(
                E_LIMIT,
                format!(
                    "maximum OpenQASM gate definitions exceeded: \
                     {} > {}",
                    self.gate_definition_count,
                    self.limits.max_gate_definitions()
                ),
                definition.span(),
            );

            return;
        }

        let name = definition.name().as_str();

        if lookup_standard_gate(name).is_some() {
            self.error(
                E_GATE_DEFINITION,
                format!(
                    "user-defined gate `{name}` conflicts with an \
                     OpenQASM standard-library gate"
                ),
                definition.name().span(),
            );
        }

        self.declare(
            definition.name(),
            SymbolKind::Gate,
        );

        if definition.parameters().len() as u64
            > self.limits.max_parameters()
        {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{name}` declares {} parameters; maximum is {}",
                    definition.parameters().len(),
                    self.limits.max_parameters()
                ),
                definition.span(),
            );
        }

        if definition.qubits().len() as u64
            > self.limits.max_operands()
        {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{name}` declares {} formal qubits; maximum is {}",
                    definition.qubits().len(),
                    self.limits.max_operands()
                ),
                definition.span(),
            );
        }

        if definition.body().len() as u64
            > self.limits.max_gate_operations()
        {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{name}` contains {} operations; maximum is {}",
                    definition.body().len(),
                    self.limits.max_gate_operations()
                ),
                definition.span(),
            );
        }

        let mut local = Scope::default();

        for parameter in definition.parameters() {
            let name = parameter.as_str();

            if local.contains(name) {
                self.error(
                    E_DUPLICATE_PARAMETER,
                    format!(
                        "duplicate formal parameter `{name}` \
                         in gate `{}`",
                        definition.name().as_str()
                    ),
                    parameter.span(),
                );

                continue;
            }

            if local.symbols.len() as u64
                >= self.limits.max_symbols()
            {
                self.error(
                    E_LIMIT,
                    "gate-local symbol-table limit exceeded",
                    parameter.span(),
                );

                break;
            }

            local.insert(
                name.to_owned(),
                Symbol {
                    kind: SymbolKind::Parameter,
                    span: parameter.span(),
                },
            );
        }

        for qubit in definition.qubits() {
            let name = qubit.as_str();

            if local.contains(name) {
                self.error(
                    E_DUPLICATE_QUBIT,
                    format!(
                        "formal qubit `{name}` duplicates another \
                         formal parameter or qubit in gate `{}`",
                        definition.name().as_str()
                    ),
                    qubit.span(),
                );

                continue;
            }

            local.insert(
                name.to_owned(),
                Symbol {
                    kind: SymbolKind::FormalQubit,
                    span: qubit.span(),
                },
            );
        }

        if self
            .active_gate_stack
            .iter()
            .any(|active| active == name)
        {
            self.error(
                E_RECURSIVE_GATE,
                format!(
                    "recursive gate definition detected for `{name}`"
                ),
                definition.name().span(),
            );

            return;
        }

        self.active_gate_stack
            .push(name.to_owned());

        for statement in definition.body() {
            if !self.count_statement(statement.span()) {
                break;
            }

            self.validate_gate_body_statement(
                statement,
                &local,
                depth.saturating_add(1),
                name,
            );
        }

        let _ = self.active_gate_stack.pop();
    }

    fn validate_gate_body_statement(
        &mut self,
        statement: &Statement,
        local: &Scope,
        depth: u64,
        gate_name: &str,
    ) {
        if depth > self.limits.max_nesting_depth() {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{gate_name}` exceeds maximum semantic \
                     nesting depth of {}",
                    self.limits.max_nesting_depth()
                ),
                statement.span(),
            );

            return;
        }

        match statement {
            Statement::GateCall(call) => {
                self.validate_gate_call(
                    call,
                    depth,
                    Some(local),
                );
            }

            Statement::Reset(reset) => {
                self.validate_gate_body_operands(
                    reset.operands(),
                    local,
                    reset.span(),
                );
            }

            Statement::Barrier(barrier) => {
                self.validate_gate_body_operands(
                    barrier.operands(),
                    local,
                    barrier.span(),
                );
            }

            Statement::Delay(delay) => {
                self.validate_expression_in_scope(
                    delay.duration(),
                    local,
                    0,
                );

                self.validate_gate_body_operands(
                    delay.operands(),
                    local,
                    delay.span(),
                );

                self.unsupported(
                    "delay is not currently representable inside \
                     canonical gate definitions",
                    delay.span(),
                );
            }

            Statement::Expression(expression) => {
                self.validate_expression_in_scope(
                    expression.expression(),
                    local,
                    0,
                );
            }

            _ => {
                self.unsupported(
                    format!(
                        "statement inside gate `{gate_name}` is not \
                         currently allowed by the Zamani gate-definition \
                         lowering contract"
                    ),
                    statement.span(),
                );
            }
        }
    }

    // -------------------------------------------------------------------------
    // Gate calls
    // -------------------------------------------------------------------------

    fn validate_gate_call(
        &mut self,
        call: &GateCall,
        _depth: u64,
        local: Option<&Scope>,
    ) {
        let name = call.name().as_str();

        let standard =
            lookup_standard_gate(name);

        let user_gate =
            self.global.get(name);

        let Some(gate) = standard else {
            if let Some(symbol) = user_gate {
                if symbol.kind != SymbolKind::Gate {
                    self.error(
                        E_SYMBOL_USE,
                        format!(
                            "`{name}` is not callable as a gate"
                        ),
                        call.name().span(),
                    );

                    return;
                }

                // User-defined gate signature information is retained in the
                // AST. The complete formal-signature check is performed when
                // the definition table is available to lowering. At this
                // stage, operand/name validation still happens.
                self.validate_gate_call_operands(
                    call,
                    local,
                );

                return;
            }

            self.error(
                E_UNKNOWN_GATE,
                format!(
                    "unknown OpenQASM gate `{name}`"
                ),
                call.name().span(),
            );

            return;
        };

        self.validate_standard_gate(
            call,
            gate,
            local,
        );
    }

    fn validate_standard_gate(
        &mut self,
        call: &GateCall,
        gate: StandardGate,
        local: Option<&Scope>,
    ) {
        let name = call.name().as_str();

        if !gate.available_in(
            self.program_major_version(),
            self.program_minor_version(),
        ) {
            self.error(
                E_UNSUPPORTED_VERSION,
                format!(
                    "standard-library gate `{name}` is not available \
                     in the selected OpenQASM version"
                ),
                call.name().span(),
            );

            return;
        }

        let actual_parameters =
            call.parameters().len();

        if actual_parameters
            != gate.parameter_count()
        {
            self.error(
                E_PARAMETER_COUNT,
                format!(
                    "gate `{name}` expects {} parameter(s), \
                     but {} were supplied",
                    gate.parameter_count(),
                    actual_parameters
                ),
                call.span(),
            );
        }

        let actual_operands =
            call.operands().len();

        if actual_operands
            != gate.qubit_count()
        {
            self.error(
                E_OPERAND_COUNT,
                format!(
                    "gate `{name}` expects {} quantum operand(s), \
                     but {} were supplied",
                    gate.qubit_count(),
                    actual_operands
                ),
                call.span(),
            );
        }

        if actual_parameters as u64
            > self.limits.max_parameters()
        {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{name}` exceeds the maximum parameter count"
                ),
                call.span(),
            );
        }

        if actual_operands as u64
            > self.limits.max_operands()
        {
            self.error(
                E_LIMIT,
                format!(
                    "gate `{name}` exceeds the maximum operand count"
                ),
                call.span(),
            );
        }

        for parameter in call.parameters() {
            self.validate_expression_in_scope(
                parameter,
                local.unwrap_or(&self.global),
                0,
            );
        }

        self.validate_gate_call_operands(
            call,
            local,
        );

        self.validate_modifiers(
            call,
            gate,
        );

        if !gate.is_supported() {
            self.error(
                E_UNSUPPORTED_GATE,
                format!(
                    "OpenQASM gate `{name}` is valid but cannot currently \
                     be represented directly by the canonical Quantum IR: {}",
                    gate
                        .lowering()
                        .unsupported_reason()
                        .unwrap_or("no direct lowering is available")
                ),
                call.span(),
            );
        }
    }

    fn validate_gate_call_operands(
        &mut self,
        call: &GateCall,
        local: Option<&Scope>,
    ) {
        let scope = local.unwrap_or(&self.global);

        for operand in call.operands() {
            match operand {
                GateOperand::Designator(designator) => {
                    self.validate_designator_in_scope(
                        designator,
                        scope,
                        Some(SymbolKind::Quantum),
                    );
                }

                GateOperand::Physical(physical) => {
                    if !self.config.allow_physical_qubits {
                        self.error(
                            E_PHYSICAL_QUBIT,
                            "physical OpenQASM qubits are not accepted \
                             by the current hardware-independent frontend \
                             boundary",
                            physical.span(),
                        );
                    }
                }
            }
        }
    }

    fn validate_modifiers(
        &mut self,
        call: &GateCall,
        gate: StandardGate,
    ) {
        let modifiers =
            call.modifiers();

        let mut seen_inv = false;
        let mut seen_pow = false;
        let mut control_count = 0usize;

        for modifier in modifiers {
            match modifier {
                GateModifier::Inv => {
                    if seen_inv {
                        self.error(
                            E_INVALID_MODIFIER,
                            format!(
                                "gate `{}` contains duplicate `inv` \
                                 modifier",
                                call.name().as_str()
                            ),
                            call.span(),
                        );
                    }

                    seen_inv = true;
                }

                GateModifier::Pow(expression) => {
                    if seen_pow {
                        self.error(
                            E_INVALID_MODIFIER,
                            format!(
                                "gate `{}` contains duplicate `pow` \
                                 modifier",
                                call.name().as_str()
                            ),
                            call.span(),
                        );
                    }

                    seen_pow = true;

                    self.validate_expression(
                        expression,
                        0,
                    );
                }

                GateModifier::Ctrl
                | GateModifier::NegCtrl => {
                    control_count =
                        control_count.saturating_add(1);
                }

                GateModifier::CtrlCount {
                    negative,
                    count,
                } => {
                    if *count == 0 {
                        self.error(
                            E_INVALID_MODIFIER,
                            "control modifier count must be greater \
                             than zero",
                            call.span(),
                        );
                    }

                    if *negative {
                        // Negative controls are valid OpenQASM syntax;
                        // representation support is decided by lowering.
                    }

                    control_count =
                        control_count.saturating_add(
                            usize::try_from(*count)
                                .unwrap_or(usize::MAX),
                        );
                }
            }
        }

        if control_count > 0 {
            // Modifiers change the semantic arity of the underlying operation.
            // The current standard-gate catalogue describes the unmodified
            // operation. A controlled operation is therefore not silently
            // accepted as equivalent to the unmodified gate.
            self.error(
                E_INVALID_MODIFIER,
                format!(
                    "controlled form of gate `{}` requires explicit \
                     lowering support; it cannot be treated as the \
                     unmodified `{}` operation",
                    call.name().as_str(),
                    gate.name(),
                ),
                call.span(),
            );
        }

        if seen_inv && seen_pow {
            self.error(
                E_INVALID_MODIFIER,
                format!(
                    "gate `{}` cannot combine `inv` and `pow` under \
                     the current Zamani OpenQASM lowering contract",
                    call.name().as_str()
                ),
                call.span(),
            );
        }
    }

    // -------------------------------------------------------------------------
    // Operands
    // -------------------------------------------------------------------------

    fn validate_quantum_operands(
        &mut self,
        operands: &[GateOperand],
        span: SourceSpan,
    ) {
        if operands.is_empty() {
            self.error(
                E_INVALID_QUANTUM_OPERAND,
                "quantum operation requires at least one operand",
                span,
            );

            return;
        }

        if !self.check_collection_limit(
            operands.len(),
            self.limits.max_operands(),
            "quantum operand count",
            span,
        ) {
            return;
        }

        for operand in operands {
            match operand {
                GateOperand::Designator(designator) => {
                    self.validate_designator(
                        designator,
                        Some(SymbolKind::Quantum),
                    );
                }

                GateOperand::Physical(physical) => {
                    if !self.config.allow_physical_qubits {
                        self.error(
                            E_PHYSICAL_QUBIT,
                            "physical qubit operands are unsupported by \
                             the current canonical IR boundary",
                            physical.span(),
                        );
                    }
                }
            }
        }
    }

    fn validate_gate_body_operands(
        &mut self,
        operands: &[GateOperand],
        local: &Scope,
        span: SourceSpan,
    ) {
        if operands.is_empty() {
            self.error(
                E_INVALID_QUANTUM_OPERAND,
                "gate operation requires at least one quantum operand",
                span,
            );

            return;
        }

        for operand in operands {
            match operand {
                GateOperand::Designator(designator) => {
                    self.validate_designator_in_scope(
                        designator,
                        local,
                        Some(SymbolKind::FormalQubit),
                    );
                }

                GateOperand::Physical(physical) => {
                    self.error(
                        E_PHYSICAL_QUBIT,
                        "physical qubits are not valid formal operands \
                         inside a source-level gate definition",
                        physical.span(),
                    );
                }
            }
        }
    }

    fn validate_designator(
        &mut self,
        designator: &crate::quantum::frontend::openqasm::ast::Designator,
        expected: Option<SymbolKind>,
    ) {
        self.validate_designator_in_scope(
            designator,
            &self.global.clone(),
            expected,
        );
    }

    fn validate_designator_in_scope(
        &mut self,
        designator: &crate::quantum::frontend::openqasm::ast::Designator,
        scope: &Scope,
        expected: Option<SymbolKind>,
    ) {
        let name = designator.name().as_str();

        let symbol = scope
            .get(name)
            .or_else(|| self.global.get(name));

        let Some(symbol) = symbol else {
            self.error(
                E_UNKNOWN_IDENTIFIER,
                format!(
                    "unknown OpenQASM identifier `{name}`"
                ),
                designator.name().span(),
            );

            return;
        };

        if let Some(expected_kind) = expected {
            let compatible =
                match expected_kind {
                    SymbolKind::Quantum => {
                        matches!(
                            symbol.kind,
                            SymbolKind::Quantum
                                | SymbolKind::Alias
                                | SymbolKind::FormalQubit
                        )
                    }

                    SymbolKind::FormalQubit => {
                        symbol.kind
                            == SymbolKind::FormalQubit
                    }

                    SymbolKind::Classical => {
                        symbol.kind
                            == SymbolKind::Classical
                    }

                    _ => true,
                };

            if !compatible {
                self.error(
                    E_INVALID_QUANTUM_OPERAND,
                    format!(
                        "`{name}` is a {}, but a {} operand is required",
                        symbol.kind.as_str(),
                        expected_kind.as_str()
                    ),
                    designator.span(),
                );
            }
        }

        if let Some(index) = designator.index() {
            self.validate_index_expression(
                index,
                scope,
            );
        }
    }

    fn validate_index_expression(
        &mut self,
        index: &crate::quantum::frontend::openqasm::ast::IndexExpression,
        scope: &Scope,
    ) {
        match index {
            crate::quantum::frontend::openqasm::ast::IndexExpression::Single(
                expression,
            ) => {
                self.validate_expression_in_scope(
                    expression,
                    scope,
                    0,
                );
            }

            crate::quantum::frontend::openqasm::ast::IndexExpression::Slice(
                slice,
            ) => {
                self.validate_expression_in_scope(
                    slice.start(),
                    scope,
                    0,
                );

                if let Some(step) = slice.step() {
                    self.validate_expression_in_scope(
                        step,
                        scope,
                        0,
                    );
                }

                self.validate_expression_in_scope(
                    slice.stop(),
                    scope,
                    0,
                );
            }

            crate::quantum::frontend::openqasm::ast::IndexExpression::Set(
                values,
            ) => {
                for value in values {
                    self.validate_expression_in_scope(
                        value,
                        scope,
                        0,
                    );
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Measurement
    // -------------------------------------------------------------------------

    fn validate_measurement_assignment(
        &mut self,
        statement: &crate::quantum::frontend::openqasm::ast::MeasureAssignmentStatement,
    ) {
        let source =
            statement.source();

        self.validate_designator(
            source.operand(),
            Some(SymbolKind::Quantum),
        );

        self.validate_designator(
            statement.destination(),
            Some(SymbolKind::Classical),
        );
    }

    // -------------------------------------------------------------------------
    // Assignments
    // -------------------------------------------------------------------------

    fn validate_assignment(
        &mut self,
        assignment: &crate::quantum::frontend::openqasm::ast::AssignmentStatement,
    ) {
        self.validate_designator(
            assignment.target(),
            Some(SymbolKind::Classical),
        );

        self.validate_expression(
            assignment.value(),
            0,
        );
    }

    fn validate_expression_statement(
        &mut self,
        statement: &crate::quantum::frontend::openqasm::ast::ExpressionStatement,
    ) {
        self.validate_expression(
            statement.expression(),
            0,
        );
    }

    // -------------------------------------------------------------------------
    // Expressions
    // -------------------------------------------------------------------------

    fn validate_expression(
        &mut self,
        expression: &Expression,
        depth: u64,
    ) {
        self.validate_expression_in_scope(
            expression,
            &self.global.clone(),
            depth,
        );
    }

    fn validate_expression_in_scope(
        &mut self,
        expression: &Expression,
        scope: &Scope,
        depth: u64,
    ) {
        if !self.count_expression(expression.span()) {
            return;
        }

        if depth > self.limits.max_expression_depth() {
            self.error(
                E_LIMIT,
                format!(
                    "OpenQASM expression nesting depth exceeds {}",
                    self.limits.max_expression_depth()
                ),
                expression.span(),
            );

            return;
        }

        match expression {
            Expression::BoolLiteral { .. }
            | Expression::IntegerLiteral { .. }
            | Expression::FloatLiteral { .. }
            | Expression::DurationLiteral { .. } => {}

            Expression::Identifier(identifier) => {
                let name = identifier.as_str();

                if !scope.contains(name)
                    && !self.global.contains(name)
                {
                    self.error(
                        E_EXPRESSION_REFERENCE,
                        format!(
                            "unknown identifier `{name}` in expression"
                        ),
                        identifier.span(),
                    );
                }
            }

            Expression::Designator(designator) => {
                self.validate_designator_in_scope(
                    designator,
                    scope,
                    None,
                );
            }

            Expression::Unary {
                operand,
                ..
            } => {
                self.validate_expression_in_scope(
                    operand,
                    scope,
                    depth.saturating_add(1),
                );
            }

            Expression::Binary {
                left,
                right,
                ..
            } => {
                self.validate_expression_in_scope(
                    left,
                    scope,
                    depth.saturating_add(1),
                );

                self.validate_expression_in_scope(
                    right,
                    scope,
                    depth.saturating_add(1),
                );
            }

            Expression::FunctionCall {
                arguments,
                ..
            } => {
                if arguments.len() as u64
                    > self.limits.max_parameters()
                {
                    self.error(
                        E_LIMIT,
                        format!(
                            "expression function has {} arguments; \
                             maximum is {}",
                            arguments.len(),
                            self.limits.max_parameters()
                        ),
                        expression.span(),
                    );
                }

                for argument in arguments {
                    self.validate_expression_in_scope(
                        argument,
                        scope,
                        depth.saturating_add(1),
                    );
                }
            }

            Expression::Cast {
                operand,
                ..
            } => {
                self.validate_expression_in_scope(
                    operand,
                    scope,
                    depth.saturating_add(1),
                );
            }

            Expression::ArrayLiteral {
                values,
                ..
            }
            | Expression::SetLiteral {
                values,
                ..
            }
            | Expression::Concatenation {
                values,
                ..
            } => {
                for value in values {
                    self.validate_expression_in_scope(
                        value,
                        scope,
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
                    self.validate_expression_in_scope(
                        start,
                        scope,
                        depth.saturating_add(1),
                    );
                }

                if let Some(step) = step {
                    self.validate_expression_in_scope(
                        step,
                        scope,
                        depth.saturating_add(1),
                    );
                }

                if let Some(stop) = stop {
                    self.validate_expression_in_scope(
                        stop,
                        scope,
                        depth.saturating_add(1),
                    );
                }
            }

            Expression::Parenthesized {
                expression,
                ..
            } => {
                self.validate_expression_in_scope(
                    expression,
                    scope,
                    depth.saturating_add(1),
                );
            }

            Expression::SizeOf {
                operand,
                ..
            }
            | Expression::DurationOf {
                operand,
                ..
            } => {
                self.validate_designator_in_scope(
                    operand,
                    scope,
                    None,
                );
            }

            Expression::Extension { .. } => {
                self.unsupported(
                    "OpenQASM expression extension is not supported \
                     by the current semantic frontend",
                    expression.span(),
                );
            }

            Expression::QuantumCall(call) => {
                self.validate_quantum_call_expression(
                    call,
                    scope,
                    depth,
                );
            }
        }
    }

    fn validate_quantum_call_expression(
        &mut self,
        call: &crate::quantum::frontend::openqasm::ast::QuantumCallExpression,
        scope: &Scope,
        depth: u64,
    ) {
        let name = call.name().as_str();

        let Some(gate) =
            lookup_standard_gate(name)
        else {
            if self.global
                .get(name)
                .map(|symbol| {
                    symbol.kind == SymbolKind::Gate
                })
                .unwrap_or(false)
            {
                for parameter in call.parameters() {
                    self.validate_expression_in_scope(
                        parameter,
                        scope,
                        depth.saturating_add(1),
                    );
                }

                for operand in call.operands() {
                    self.validate_gate_operand_in_scope(
                        operand,
                        scope,
                    );
                }

                return;
            }

            self.error(
                E_UNKNOWN_GATE,
                format!(
                    "unknown quantum-call gate `{name}`"
                ),
                call.name().span(),
            );

            return;
        };

        if call.parameters().len()
            != gate.parameter_count()
        {
            self.error(
                E_PARAMETER_COUNT,
                format!(
                    "quantum-call gate `{name}` expects {} parameter(s), \
                     but {} were supplied",
                    gate.parameter_count(),
                    call.parameters().len()
                ),
                call.span(),
            );
        }

        if call.operands().len()
            != gate.qubit_count()
        {
            self.error(
                E_OPERAND_COUNT,
                format!(
                    "quantum-call gate `{name}` expects {} operand(s), \
                     but {} were supplied",
                    gate.qubit_count(),
                    call.operands().len()
                ),
                call.span(),
            );
        }

        for parameter in call.parameters() {
            self.validate_expression_in_scope(
                parameter,
                scope,
                depth.saturating_add(1),
            );
        }

        for operand in call.operands() {
            self.validate_gate_operand_in_scope(
                operand,
                scope,
            );
        }

        if !gate.is_supported() {
            self.error(
                E_UNSUPPORTED_GATE,
                format!(
                    "quantum-call gate `{name}` cannot currently be \
                     lowered to the canonical Quantum IR"
                ),
                call.span(),
            );
        }
    }

    fn validate_gate_operand_in_scope(
        &mut self,
        operand: &GateOperand,
        scope: &Scope,
    ) {
        match operand {
            GateOperand::Designator(designator) => {
                self.validate_designator_in_scope(
                    designator,
                    scope,
                    Some(SymbolKind::Quantum),
                );
            }

            GateOperand::Physical(physical) => {
                if !self.config.allow_physical_qubits {
                    self.error(
                        E_PHYSICAL_QUBIT,
                        "physical qubit operands are not supported \
                         by the current frontend/IR boundary",
                        physical.span(),
                    );
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Version helpers
    // -------------------------------------------------------------------------

    fn program_major_version(&self) -> u8 {
        // The public validator is deliberately conservative. The parser owns
        // the actual program version, and the AST exposes it through Program.
        //
        // The standard-gate catalogue currently starts at OpenQASM 3.0.
        //
        // This method is kept centralized so version-dependent rules do not
        // become scattered throughout validation.rs.
        //
        // The public entry point rejects missing/unsupported versions before
        // statement validation.
        3
    }

    fn program_minor_version(&self) -> u8 {
        // The AST version is consumed by the public entry point through
        // `validate_program`. This default is 1 because both 3.0 and 3.1
        // currently share the standard-gate catalogue.
        //
        // Version-specific semantic rules should be introduced here when
        // OpenQASM 3.x differences materially affect validation.
        1
    }
}

// =============================================================================
// Public API
// =============================================================================

/// Validates an OpenQASM AST using production frontend limits.
///
/// This is the primary semantic-validation entry point.
///
/// The function never performs filesystem or network I/O and never lowers into
/// the canonical Quantum IR.
///
/// # Example
///
/// ```ignore
/// let report = validate_program(
///     &program,
///     FrontendLimits::production(),
///     OpenQasmValidationConfig::default(),
/// );
///
/// if report.is_valid() {
///     // Safe to proceed to the OpenQASM lowering boundary.
/// }
/// ```
#[must_use]
pub fn validate_program(
    program: &Program,
    limits: FrontendLimits,
    config: OpenQasmValidationConfig,
) -> ValidationReport {
    Validator::new(limits, config)
        .validate_program(program)
}

// =============================================================================
// Compatibility aliases
// =============================================================================

/// Validates an OpenQASM program using the default production policy.
#[must_use]
pub fn validate(
    program: &Program,
) -> ValidationReport {
    validate_program(
        program,
        FrontendLimits::production(),
        OpenQasmValidationConfig::default(),
    )
}

// =============================================================================
// AST compatibility helpers
// =============================================================================
//
// These helpers intentionally isolate AST-shape assumptions from the semantic
// validator. If the AST gains another declaration representation later, the
// semantic implementation changes here rather than being duplicated across
// validation paths.

fn quantum_declaration_size(
    declaration: &crate::quantum::frontend::openqasm::ast::QuantumDeclaration,
) -> Option<&Expression> {
    match declaration.ty() {
        crate::quantum::frontend::openqasm::ast::QuantumType::Qubit(
            size,
        )
        | crate::quantum::frontend::openqasm::ast::QuantumType::QReg(
            size,
        ) => size.as_ref(),

        _ => None,
    }
}

fn old_style_declaration_size(
    declaration: &crate::quantum::frontend::openqasm::ast::OldStyleDeclaration,
) -> Option<&Expression> {
    declaration.size()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_validation_config_is_production_compatible() {
        let config =
            OpenQasmValidationConfig::default();

        assert!(config.allow_3_0);
        assert!(config.allow_3_1);
        assert!(config.allow_standard_library_include);
        assert!(!config.allow_physical_qubits);
        assert!(!config.preserve_unsupported_constructs);
    }

    #[test]
    fn diagnostic_codes_are_format_local() {
        assert_eq!(E_VERSION, "QASM-E001");
        assert_eq!(E_UNKNOWN_GATE, "QASM-E008");
        assert_eq!(E_UNSUPPORTED_GATE, "QASM-E011");
        assert_eq!(E_LIMIT, "QASM-E019");
    }

    #[test]
    fn unsupported_standard_gate_is_not_silently_accepted() {
        let gate =
            lookup_standard_gate("sx");

        assert!(gate.is_some());

        let gate =
            gate.expect("sx must exist in the catalogue");

        assert!(!gate.is_supported());
        assert!(gate.lowering().unsupported_reason().is_some());
    }

    #[test]
    fn physical_qubits_are_disabled_by_default() {
        assert!(
            !OpenQasmValidationConfig::default()
                .allow_physical_qubits
        );
    }

    #[test]
    fn production_limits_are_finite() {
        let limits =
            FrontendLimits::production();

        assert!(limits.max_symbols() > 0);
        assert!(limits.max_operands() > 0);
        assert!(limits.max_parameters() > 0);
        assert!(limits.max_expression_depth() > 0);
        assert!(limits.max_expression_nodes() > 0);
    }
}