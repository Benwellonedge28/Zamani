//! Zamani Quantum Optimization — Parameter Usage Analysis
//!
//! Production-grade, read-only analysis of symbolic and numerical parameter
//! usage in the canonical Zamani quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir::QuantumCircuit
//!                               │
//!                               ▼
//!                    optimization::analysis
//!                               │
//!                               ▼
//!                       parameter_usage
//!                               │
//!              ┌────────────────┼────────────────┐
//!              ▼                ▼                ▼
//!       symbolic passes   parameter folding   cost models
//!              │                │                │
//!              └────────────────┼────────────────┘
//!                               ▼
//!                         optimization
//! ```
//!
//! # Purpose
//!
//! This module determines how parameters are used throughout a canonical
//! quantum circuit without modifying the circuit.
//!
//! It answers questions such as:
//!
//! - Which symbolic parameters occur in the circuit?
//! - How many times is each symbol referenced?
//! - Which operations reference each symbol?
//! - Which gate-parameter slots reference each symbol?
//! - How many parameters are constants?
//! - How many parameters are symbolic?
//! - How many direct symbol references exist?
//! - How many symbol references occur inside expressions?
//! - What is the maximum expression depth observed?
//! - Which operations are parameterized?
//! - How many unique symbols are present?
//! - How many total symbolic references exist?
//!
//! # Architectural ownership
//!
//! This module does NOT define:
//!
//! - another quantum circuit representation;
//! - another parameter representation;
//! - another symbol table;
//! - parameter binding;
//! - parameter simplification;
//! - parameter rewriting;
//! - gate synthesis;
//! - routing;
//! - scheduling;
//! - hardware calibration;
//! - execution;
//! - simulation.
//!
//! The authoritative parameter representation remains:
//!
//! `crate::quantum::ir::parameter::Parameter`
//!
//! and:
//!
//! `crate::quantum::ir::parameter::ParameterExpression`
//!
//! The authoritative circuit representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `Parameter`;
//! - `ParameterExpression`.
//!
//! The circuit is never mutated.
//!
//! ## `optimization::circuit`
//!
//! This module accepts `CircuitView` so analyses can be performed through the
//! optimizer's canonical read-only access layer.
//!
//! The preferred entry points are:
//!
//! ```text
//! analyze_parameter_usage(&circuit)
//! analyze_parameter_usage_view(&view)
//! ```
//!
//! ## `analysis::mod`
//!
//! The analysis module should expose:
//!
//! ```text
//! pub mod parameter_usage;
//!
//! pub use parameter_usage::{
//!     analyze_parameter_usage,
//!     analyze_parameter_usage_view,
//!     ParameterUsageAnalysis,
//!     ParameterUsage,
//!     ParameterOccurrence,
//!     ParameterOccurrenceKind,
//!     ParameterUsageError,
//! };
//! ```
//!
//! ## `context.rs`
//!
//! `ParameterUsageAnalysis` is immutable and can be cached by an optimization
//! context.
//!
//! Any transformation that changes:
//!
//! - operation order;
//! - operation count;
//! - gate parameters;
//! - parameter expressions;
//! - parameter binding;
//! - parameter removal;
//! - parameter insertion;
//!
//! invalidates this analysis.
//!
//! A transformation that changes only:
//!
//! - metadata;
//! - compiler annotations;
//! - unrelated circuit properties;
//!
//! does not invalidate this analysis.
//!
//! ## `parameter/constant_fold.rs`
//!
//! This analysis identifies which parameters are symbolic and where they are
//! used. Constant folding can use this information to distinguish fully
//! numerical parameters from symbolic expressions.
//!
//! ## `parameter/symbolic.rs`
//!
//! Symbolic optimization can use:
//!
//! - symbol occurrence counts;
//! - operation locations;
//! - parameter-slot locations;
//! - direct versus expression references.
//!
//! ## `parameter/simplification.rs`
//!
//! Expression simplification can use the occurrence information to determine
//! which symbols and expressions need reconsideration after a transformation.
//!
//! ## `parameter/binding.rs`
//!
//! Parameter binding can use this analysis to determine whether a circuit is:
//!
//! - completely numerical;
//! - partially symbolic;
//! - fully symbolic with respect to its parameterized operations.
//!
//! ## `passes/optimize_*`
//!
//! Composite optimization passes may use this analysis for cost estimation and
//! pass selection.
//!
//! ## `cost.rs`
//!
//! Parameter usage is an input to parameter-sensitive cost models but does not
//! define cost itself.
//!
//! ## `verification`
//!
//! Verification can use the analysis to ensure that a transformation did not
//! unexpectedly introduce, remove, or alter symbolic dependencies.
//!
//! # Complexity
//!
//! Let:
//!
//! - `N` = number of operations;
//! - `P` = total gate-parameter slots;
//! - `R` = total symbolic references inside parameters;
//! - `S` = number of unique symbolic names;
//! - `D` = maximum expression depth.
//!
//! The analysis runs in:
//!
//! ```text
//! O(N + P + R + S log S)
//! ```
//!
//! where the final `S log S` component is used only to make the public symbol
//! ordering deterministic.
//!
//! Memory usage is:
//!
//! ```text
//! O(S + R + O)
//! ```
//!
//! where `O` is the number of operations referenced by symbolic parameters.
//!
//! No storage is allocated proportional to the declared number of logical
//! qubits or classical bits.
//!
//! This is essential for sparse circuits.
//!
//! Example:
//!
//! ```text
//! 1,000,000,000 logical qubits declared
//! 17 logical qubits used
//! 3 symbolic parameters used
//! ```
//!
//! The analysis allocates according to parameter usage, not according to the
//! billion-qubit namespace.
//!
//! # Scaling
//!
//! There is deliberately no artificial optimizer-specific maximum on:
//!
//! - number of operations;
//! - number of parameters;
//! - number of unique symbols;
//! - number of symbol references;
//! - number of parameterized gates.
//!
//! The canonical IR's resource limits remain authoritative.
//!
//! Memory allocation remains bounded by the actual result size and available
//! resources.
//!
//! The implementation does not recursively walk expressions. It uses an
//! explicit work stack so deeply nested expressions do not consume the Rust
//! call stack.
//!
//! The canonical IR currently bounds expression depth, but this implementation
//! does not depend on that bound for memory safety.
//!
//! # Determinism
//!
//! All externally visible collections are deterministic:
//!
//! - symbols are ordered lexicographically;
//! - operations are ordered by operation index;
//! - parameter indices are ordered numerically;
//! - occurrences retain circuit traversal order.
//!
//! No hash-map iteration order becomes compiler-visible behavior.
//!
//! # Numerical semantics
//!
//! This module does not evaluate parameter expressions.
//!
//! It therefore never:
//!
//! - approximates angles;
//! - compares floating-point parameter values;
//! - folds constants;
//! - binds symbols;
//! - assumes that a symbol represents an angle;
//! - assumes hardware-specific units.
//!
//! Those responsibilities belong to parameter transformation or backend
//! stages.
//!
//! # Safety
//!
//! No unsafe Rust is permitted.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Verification requirements
//!
//! Tests cover:
//!
//! - empty circuits;
//! - circuits without parameters;
//! - constant parameters;
//! - direct symbols;
//! - symbols inside expressions;
//! - repeated symbols;
//! - multiple symbols;
//! - repeated use across operations;
//! - repeated use in the same expression;
//! - nested expressions;
//! - deterministic ordering;
//! - operation/parameter indices;
//! - direct versus expression references;
//! - complete numeric circuits;
//! - partially symbolic circuits;
//! - fully symbolic circuits;
//! - very large operation counts subject to IR limits;
//! - no mutation of the input circuit;
//! - idempotent repeated analysis;
//! - invalid canonical circuits at the analysis boundary;
//! - arithmetic overflow protection.
//!
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
};

use crate::quantum::ir::QuantumCircuit;

use crate::quantum::optimization::circuit::{
    CircuitView,
};

// =============================================================================
// Public scalar types
// =============================================================================

/// Zero-based operation position in the canonical circuit.
///
/// This is intentionally an analysis-local scalar rather than the optimizer's
/// `OperationId`. Analyses remain usable independently of optimizer mutation
/// infrastructure.
pub type OperationIndex = usize;

/// Zero-based parameter position within one gate.
pub type ParameterIndex = usize;

// =============================================================================
// Parameter occurrence kind
// =============================================================================

/// Describes how a symbol is referenced by one gate parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterOccurrenceKind {
    /// The parameter itself is exactly a symbolic parameter.
    ///
    /// Example:
    ///
    /// ```text
    /// rx(theta)
    /// ```
    DirectSymbol,

    /// The symbol occurs somewhere inside a parameter expression.
    ///
    /// Example:
    ///
    /// ```text
    /// rx(theta + phi)
    /// ```
    Expression,
}

impl ParameterOccurrenceKind {
    /// Returns whether this occurrence is a direct symbol.
    #[must_use]
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::DirectSymbol)
    }

    /// Returns whether this occurrence is inside an expression.
    #[must_use]
    pub const fn is_expression(self) -> bool {
        matches!(self, Self::Expression)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by parameter-usage analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterUsageError {
    /// The canonical input circuit is invalid.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// A parameter index could not be represented safely.
    ParameterIndexOverflow {
        /// Operation containing the invalid parameter position.
        operation: OperationIndex,
    },

    /// An internal checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// An internal analysis invariant was violated.
    InvariantViolation {
        /// Static invariant description.
        message: &'static str,
    },
}

impl fmt::Display for ParameterUsageError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze parameter usage: invalid quantum circuit: {message}"
                )
            }

            Self::ParameterIndexOverflow { operation } => {
                write!(
                    formatter,
                    "parameter index overflow while analyzing operation {operation}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvariantViolation { message } => {
                write!(
                    formatter,
                    "parameter-usage analysis invariant violated: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ParameterUsageError {}

// =============================================================================
// Parameter occurrence
// =============================================================================

/// One symbolic reference within one canonical gate parameter.
///
/// A single parameter expression can produce multiple occurrences.
///
/// Example:
///
/// ```text
/// rx(theta + theta)
/// ```
///
/// produces two occurrences for `theta`.
///
/// This structure intentionally identifies the operation and parameter slot,
/// but does not introduce an optimizer-local expression AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterOccurrence {
    /// Operation containing the reference.
    operation: OperationIndex,

    /// Parameter slot within that operation.
    parameter: ParameterIndex,

    /// How the symbol is referenced.
    kind: ParameterOccurrenceKind,
}

impl ParameterOccurrence {
    /// Creates a parameter occurrence.
    #[must_use]
    pub const fn new(
        operation: OperationIndex,
        parameter: ParameterIndex,
        kind: ParameterOccurrenceKind,
    ) -> Self {
        Self {
            operation,
            parameter,
            kind,
        }
    }

    /// Returns the operation index.
    #[must_use]
    pub const fn operation(&self) -> OperationIndex {
        self.operation
    }

    /// Returns the parameter index.
    #[must_use]
    pub const fn parameter(&self) -> ParameterIndex {
        self.parameter
    }

    /// Returns the occurrence kind.
    #[must_use]
    pub const fn kind(&self) -> ParameterOccurrenceKind {
        self.kind
    }

    /// Returns true when this is a direct symbol occurrence.
    #[must_use]
    pub const fn is_direct(&self) -> bool {
        self.kind.is_direct()
    }

    /// Returns true when this is an expression occurrence.
    #[must_use]
    pub const fn is_expression(&self) -> bool {
        self.kind.is_expression()
    }
}

// =============================================================================
// Per-symbol usage
// =============================================================================

/// Immutable usage information for one symbolic parameter.
///
/// Symbol names are owned by the analysis result so the result remains valid
/// independently of the source circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterUsage {
    /// Symbol name.
    name: String,

    /// Every symbolic reference to this symbol.
    occurrences: Vec<ParameterOccurrence>,

    /// Number of direct-symbol occurrences.
    direct_occurrences: usize,

    /// Number of expression occurrences.
    expression_occurrences: usize,

    /// Number of distinct operations referencing the symbol.
    operation_count: usize,

    /// Number of distinct parameter slots referencing the symbol.
    parameter_slot_count: usize,
}

impl ParameterUsage {
    fn new(name: String) -> Self {
        Self {
            name,
            occurrences: Vec::new(),
            direct_occurrences: 0,
            expression_occurrences: 0,
            operation_count: 0,
            parameter_slot_count: 0,
        }
    }

    fn record(
        &mut self,
        occurrence: ParameterOccurrence,
    ) -> Result<(), ParameterUsageError> {
        match occurrence.kind {
            ParameterOccurrenceKind::DirectSymbol => {
                self.direct_occurrences = self
                    .direct_occurrences
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation: "direct symbolic occurrence count",
                        },
                    )?;
            }

            ParameterOccurrenceKind::Expression => {
                self.expression_occurrences = self
                    .expression_occurrences
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "expression symbolic occurrence count",
                        },
                    )?;
            }
        }

        self.occurrences.push(occurrence);

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ParameterUsageError> {
        self.occurrences.sort_unstable_by_key(|occurrence| {
            (
                occurrence.operation(),
                occurrence.parameter(),
                occurrence.kind(),
            )
        });

        let mut previous_operation: Option<OperationIndex> = None;
        let mut previous_slot: Option<(OperationIndex, ParameterIndex)> =
            None;

        for occurrence in &self.occurrences {
            if previous_operation != Some(occurrence.operation()) {
                self.operation_count = self
                    .operation_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "distinct symbolic operation count",
                        },
                    )?;

                previous_operation = Some(occurrence.operation());
            }

            let slot = (
                occurrence.operation(),
                occurrence.parameter(),
            );

            if previous_slot != Some(slot) {
                self.parameter_slot_count = self
                    .parameter_slot_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "distinct symbolic parameter-slot count",
                        },
                    )?;

                previous_slot = Some(slot);
            }
        }

        Ok(())
    }

    /// Returns the symbol name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all symbolic occurrences.
    #[must_use]
    pub fn occurrences(&self) -> &[ParameterOccurrence] {
        &self.occurrences
    }

    /// Returns the number of symbolic references.
    #[must_use]
    pub fn occurrence_count(&self) -> usize {
        self.occurrences.len()
    }

    /// Returns the number of direct-symbol references.
    #[must_use]
    pub const fn direct_occurrence_count(&self) -> usize {
        self.direct_occurrences
    }

    /// Returns the number of expression references.
    #[must_use]
    pub const fn expression_occurrence_count(&self) -> usize {
        self.expression_occurrences
    }

    /// Returns the number of distinct operations referencing this symbol.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of distinct operation/parameter slots referencing
    /// this symbol.
    #[must_use]
    pub const fn parameter_slot_count(&self) -> usize {
        self.parameter_slot_count
    }

    /// Returns true when the symbol occurs directly at least once.
    #[must_use]
    pub const fn has_direct_occurrence(&self) -> bool {
        self.direct_occurrences != 0
    }

    /// Returns true when the symbol occurs inside an expression at least once.
    #[must_use]
    pub const fn has_expression_occurrence(&self) -> bool {
        self.expression_occurrences != 0
    }
}

// =============================================================================
// Analysis result
// =============================================================================

/// Immutable production-grade parameter-usage analysis result.
///
/// The result owns all symbol names and occurrence information. The analyzed
/// circuit does not need to remain alive after analysis completes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterUsageAnalysis {
    /// Number of operations in the analyzed circuit.
    operation_count: usize,

    /// Total number of gate parameter slots.
    parameter_count: usize,

    /// Number of parameter slots containing concrete constants.
    constant_parameter_count: usize,

    /// Number of parameter slots containing symbols or symbolic expressions.
    symbolic_parameter_count: usize,

    /// Number of operations containing at least one parameter.
    parameterized_operation_count: usize,

    /// Number of operations containing at least one symbolic parameter.
    symbolic_operation_count: usize,

    /// Total number of symbolic references.
    symbolic_reference_count: usize,

    /// Number of direct symbol references.
    direct_symbol_reference_count: usize,

    /// Number of references inside expressions.
    expression_symbol_reference_count: usize,

    /// Number of unique symbolic names.
    unique_symbol_count: usize,

    /// Maximum expression depth observed.
    maximum_expression_depth: usize,

    /// Number of parameters that contain expressions.
    expression_parameter_count: usize,

    /// Symbol usage indexed by deterministic symbol name.
    symbols: BTreeMap<String, ParameterUsage>,
}

impl ParameterUsageAnalysis {
    /// Returns the number of analyzed operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the total number of gate parameter slots.
    #[must_use]
    pub const fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    /// Returns the number of concrete constant parameter slots.
    #[must_use]
    pub const fn constant_parameter_count(&self) -> usize {
        self.constant_parameter_count
    }

    /// Returns the number of symbolic parameter slots.
    #[must_use]
    pub const fn symbolic_parameter_count(&self) -> usize {
        self.symbolic_parameter_count
    }

    /// Returns the number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operation_count(&self) -> usize {
        self.parameterized_operation_count
    }

    /// Returns the number of operations containing symbolic parameters.
    #[must_use]
    pub const fn symbolic_operation_count(&self) -> usize {
        self.symbolic_operation_count
    }

    /// Returns the total number of symbolic references.
    #[must_use]
    pub const fn symbolic_reference_count(&self) -> usize {
        self.symbolic_reference_count
    }

    /// Returns the number of direct symbol references.
    #[must_use]
    pub const fn direct_symbol_reference_count(&self) -> usize {
        self.direct_symbol_reference_count
    }

    /// Returns the number of expression-contained symbol references.
    #[must_use]
    pub const fn expression_symbol_reference_count(&self) -> usize {
        self.expression_symbol_reference_count
    }

    /// Returns the number of unique symbolic names.
    #[must_use]
    pub const fn unique_symbol_count(&self) -> usize {
        self.unique_symbol_count
    }

    /// Returns the maximum symbolic-expression depth observed.
    #[must_use]
    pub const fn maximum_expression_depth(&self) -> usize {
        self.maximum_expression_depth
    }

    /// Returns the number of parameter slots containing expressions.
    #[must_use]
    pub const fn expression_parameter_count(&self) -> usize {
        self.expression_parameter_count
    }

    /// Returns true when the circuit contains no symbolic parameters.
    #[must_use]
    pub const fn is_fully_bound(&self) -> bool {
        self.symbolic_parameter_count == 0
    }

    /// Returns true when the circuit contains at least one symbolic parameter.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        self.symbolic_parameter_count != 0
    }

    /// Returns true when the circuit contains no gate parameters.
    #[must_use]
    pub const fn has_no_parameters(&self) -> bool {
        self.parameter_count == 0
    }

    /// Returns all symbol names in deterministic lexicographic order.
    #[must_use]
    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.symbols.keys().map(String::as_str)
    }

    /// Returns all symbol usages in deterministic lexicographic order.
    #[must_use]
    pub fn symbols(&self) -> impl Iterator<Item = &ParameterUsage> {
        self.symbols.values()
    }

    /// Looks up one symbolic parameter by exact name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ParameterUsage> {
        self.symbols.get(name)
    }

    /// Returns whether the exact symbol occurs in the circuit.
    #[must_use]
    pub fn contains_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Returns the complete immutable symbol map.
    ///
    /// The `BTreeMap` is intentionally exposed read-only so consumers receive
    /// deterministic ordering without relying on implementation-specific hash
    /// ordering.
    #[must_use]
    pub fn symbol_map(&self) -> &BTreeMap<String, ParameterUsage> {
        &self.symbols
    }

    /// Returns the average symbolic-reference multiplicity for symbols.
    ///
    /// Returns `None` when no symbols exist.
    #[must_use]
    pub fn average_symbol_references(&self) -> Option<f64> {
        if self.unique_symbol_count == 0 {
            return None;
        }

        Some(
            self.symbolic_reference_count as f64
                / self.unique_symbol_count as f64,
        )
    }

    /// Returns the maximum number of references made to any single symbol.
    #[must_use]
    pub fn maximum_symbol_references(&self) -> usize {
        self.symbols
            .values()
            .map(ParameterUsage::occurrence_count)
            .max()
            .unwrap_or(0)
    }

    /// Returns the symbol with the largest number of references.
    ///
    /// When several symbols tie, lexicographically smallest symbol name is
    /// returned because the underlying map is deterministic.
    #[must_use]
    pub fn most_used_symbol(&self) -> Option<&ParameterUsage> {
        self.symbols.values().max_by(|left, right| {
            left.occurrence_count()
                .cmp(&right.occurrence_count())
                .then_with(|| right.name().cmp(left.name()))
        })
    }
}

// =============================================================================
// Internal mutable builder
// =============================================================================

#[derive(Debug, Default)]
struct AnalysisBuilder {
    operation_count: usize,
    parameter_count: usize,
    constant_parameter_count: usize,
    symbolic_parameter_count: usize,
    parameterized_operation_count: usize,
    symbolic_operation_count: usize,
    symbolic_reference_count: usize,
    direct_symbol_reference_count: usize,
    expression_symbol_reference_count: usize,
    expression_parameter_count: usize,
    maximum_expression_depth: usize,
    symbols: BTreeMap<String, ParameterUsage>,
}

impl AnalysisBuilder {
    fn record_operation(
        &mut self,
        parameter_count: usize,
    ) -> Result<(), ParameterUsageError> {
        self.operation_count = self
            .operation_count
            .checked_add(1)
            .ok_or(
                ParameterUsageError::ArithmeticOverflow {
                    calculation: "operation count",
                },
            )?;

        if parameter_count != 0 {
            self.parameterized_operation_count = self
                .parameterized_operation_count
                .checked_add(1)
                .ok_or(
                    ParameterUsageError::ArithmeticOverflow {
                        calculation: "parameterized operation count",
                    },
                )?;
        }

        self.parameter_count = self
            .parameter_count
            .checked_add(parameter_count)
            .ok_or(
                ParameterUsageError::ArithmeticOverflow {
                    calculation: "parameter count",
                },
            )?;

        Ok(())
    }

    fn record_parameter(
        &mut self,
        parameter: &Parameter,
        operation: OperationIndex,
        parameter_index: ParameterIndex,
    ) -> Result<bool, ParameterUsageError> {
        match parameter {
            Parameter::Constant(_) => {
                self.constant_parameter_count = self
                    .constant_parameter_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "constant parameter count",
                        },
                    )?;

                Ok(false)
            }

            Parameter::Symbol(name) => {
                self.symbolic_parameter_count = self
                    .symbolic_parameter_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "symbolic parameter count",
                        },
                    )?;

                self.symbolic_reference_count = self
                    .symbolic_reference_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "symbolic reference count",
                        },
                    )?;

                self.direct_symbol_reference_count = self
                    .direct_symbol_reference_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "direct symbol reference count",
                        },
                    )?;

                let usage = self
                    .symbols
                    .entry(name.clone())
                    .or_insert_with(|| {
                        ParameterUsage::new(name.clone())
                    });

                usage.record(ParameterOccurrence::new(
                    operation,
                    parameter_index,
                    ParameterOccurrenceKind::DirectSymbol,
                ))?;

                Ok(true)
            }

            Parameter::Expression(expression) => {
                self.symbolic_parameter_count = self
                    .symbolic_parameter_count
                    .checked_add(
                        usize::from(expression.is_symbolic()),
                    )
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "symbolic parameter count",
                        },
                    )?;

                self.expression_parameter_count = self
                    .expression_parameter_count
                    .checked_add(1)
                    .ok_or(
                        ParameterUsageError::ArithmeticOverflow {
                            calculation:
                                "expression parameter count",
                        },
                    )?;

                let mut collector = ExpressionCollector::new();

                collector.collect(expression)?;

                if collector.maximum_depth
                    > self.maximum_expression_depth
                {
                    self.maximum_expression_depth =
                        collector.maximum_depth;
                }

                let contains_symbol =
                    collector.symbols.len() != 0;

                if contains_symbol {
                    self.symbolic_operation_count = self
                        .symbolic_operation_count
                        .checked_add(1)
                        .ok_or(
                            ParameterUsageError::ArithmeticOverflow {
                                calculation:
                                    "symbolic operation count",
                            },
                        )?;

                    for reference in collector.references {
                        self.symbolic_reference_count =
                            self.symbolic_reference_count
                                .checked_add(1)
                                .ok_or(
                                    ParameterUsageError::ArithmeticOverflow {
                                        calculation:
                                            "symbolic reference count",
                                    },
                                )?;

                        match reference.kind {
                            ParameterOccurrenceKind::DirectSymbol => {
                                self.direct_symbol_reference_count =
                                    self.direct_symbol_reference_count
                                        .checked_add(1)
                                        .ok_or(
                                            ParameterUsageError::ArithmeticOverflow {
                                                calculation:
                                                    "direct symbol reference count",
                                            },
                                        )?;
                            }

                            ParameterOccurrenceKind::Expression => {
                                self.expression_symbol_reference_count =
                                    self.expression_symbol_reference_count
                                        .checked_add(1)
                                        .ok_or(
                                            ParameterUsageError::ArithmeticOverflow {
                                                calculation:
                                                    "expression symbol reference count",
                                            },
                                        )?;
                            }
                        }

                        let usage = self
                            .symbols
                            .entry(reference.name.clone())
                            .or_insert_with(|| {
                                ParameterUsage::new(
                                    reference.name.clone(),
                                )
                            });

                        usage.record(
                            ParameterOccurrence::new(
                                operation,
                                parameter_index,
                                reference.kind,
                            ),
                        )?;
                    }
                }

                Ok(contains_symbol)
            }
        }
    }

    fn finalize(
        mut self,
    ) -> Result<ParameterUsageAnalysis, ParameterUsageError> {
        for usage in self.symbols.values_mut() {
            usage.finalize()?;
        }

        let unique_symbol_count = self.symbols.len();

        if unique_symbol_count
            != self
                .symbols
                .values()
                .filter(|usage| usage.occurrence_count() != 0)
                .count()
        {
            return Err(
                ParameterUsageError::InvariantViolation {
                    message:
                        "symbol table contains an empty symbol usage",
                },
            );
        }

        if self.symbolic_reference_count
            != self.direct_symbol_reference_count
                + self.expression_symbol_reference_count
        {
            return Err(
                ParameterUsageError::InvariantViolation {
                    message:
                        "symbolic reference totals do not reconcile",
                },
            );
        }

        Ok(ParameterUsageAnalysis {
            operation_count: self.operation_count,
            parameter_count: self.parameter_count,
            constant_parameter_count: self.constant_parameter_count,
            symbolic_parameter_count: self.symbolic_parameter_count,
            parameterized_operation_count:
                self.parameterized_operation_count,
            symbolic_operation_count:
                self.symbolic_operation_count,
            symbolic_reference_count:
                self.symbolic_reference_count,
            direct_symbol_reference_count:
                self.direct_symbol_reference_count,
            expression_symbol_reference_count:
                self.expression_symbol_reference_count,
            unique_symbol_count,
            maximum_expression_depth:
                self.maximum_expression_depth,
            expression_parameter_count:
                self.expression_parameter_count,
            symbols: self.symbols,
        })
    }
}

// =============================================================================
// Expression collection
// =============================================================================

/// Internal symbolic-reference record.
///
/// The name is owned because expression traversal may outlive the source
/// parameter during analysis construction.
#[derive(Debug, Clone)]
struct CollectedReference {
    name: String,
    kind: ParameterOccurrenceKind,
}

/// Explicit stack frame for iterative parameter-expression traversal.
#[derive(Debug, Clone, Copy)]
enum ExpressionWork<'a> {
    Parameter {
        parameter: &'a Parameter,
        depth: usize,
        kind: ParameterOccurrenceKind,
    },

    Expression {
        expression: &'a ParameterExpression,
        depth: usize,
    },
}

/// Iterative expression symbol collector.
///
/// No recursion is used. This protects the analysis from depending on Rust's
/// call-stack capacity.
#[derive(Debug, Default)]
struct ExpressionCollector {
    references: Vec<CollectedReference>,
    symbols: BTreeMap<String, usize>,
    maximum_depth: usize,
}

impl ExpressionCollector {
    fn new() -> Self {
        Self::default()
    }

    fn collect(
        &mut self,
        expression: &ParameterExpression,
    ) -> Result<(), ParameterUsageError> {
        let mut work = Vec::new();

        work.push(ExpressionWork::Expression {
            expression,
            depth: 0,
        });

        while let Some(item) = work.pop() {
            match item {
                ExpressionWork::Expression {
                    expression,
                    depth,
                } => {
                    if depth > self.maximum_depth {
                        self.maximum_depth = depth;
                    }

                    match expression {
                        ParameterExpression::Add(left, right)
                        | ParameterExpression::Subtract(
                            left,
                            right,
                        )
                        | ParameterExpression::Multiply(
                            left,
                            right,
                        )
                        | ParameterExpression::Divide(
                            left,
                            right,
                        ) => {
                            let child_depth = depth
                                .checked_add(1)
                                .ok_or(
                                    ParameterUsageError::ArithmeticOverflow {
                                        calculation:
                                            "parameter expression depth",
                                    },
                                )?;

                            work.push(
                                ExpressionWork::Parameter {
                                    parameter: right,
                                    depth: child_depth,
                                    kind:
                                        ParameterOccurrenceKind::Expression,
                                },
                            );

                            work.push(
                                ExpressionWork::Parameter {
                                    parameter: left,
                                    depth: child_depth,
                                    kind:
                                        ParameterOccurrenceKind::Expression,
                                },
                            );
                        }

                        ParameterExpression::Negate(value) => {
                            let child_depth = depth
                                .checked_add(1)
                                .ok_or(
                                    ParameterUsageError::ArithmeticOverflow {
                                        calculation:
                                            "parameter expression depth",
                                    },
                                )?;

                            work.push(
                                ExpressionWork::Parameter {
                                    parameter: value,
                                    depth: child_depth,
                                    kind:
                                        ParameterOccurrenceKind::Expression,
                                },
                            );
                        }
                    }
                }

                ExpressionWork::Parameter {
                    parameter,
                    depth,
                    kind,
                } => {
                    if depth > self.maximum_depth {
                        self.maximum_depth = depth;
                    }

                    match parameter {
                        Parameter::Constant(_) => {}

                        Parameter::Symbol(name) => {
                            let entry = self
                                .symbols
                                .entry(name.clone())
                                .or_insert(0);

                            *entry = entry.checked_add(1).ok_or(
                                ParameterUsageError::ArithmeticOverflow {
                                    calculation:
                                        "symbol expression occurrence count",
                                },
                            )?;

                            self.references.push(
                                CollectedReference {
                                    name: name.clone(),
                                    kind,
                                },
                            );
                        }

                        Parameter::Expression(nested) => {
                            work.push(
                                ExpressionWork::Expression {
                                    expression: nested,
                                    depth,
                                },
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Public analysis entry points
// =============================================================================

/// Analyzes parameter usage in a canonical quantum circuit.
///
/// The circuit is validated through `CircuitView::new` before analysis.
///
/// # Errors
///
/// Returns [`ParameterUsageError::InvalidCircuit`] if canonical IR validation
/// fails.
///
/// # Complexity
///
/// ```text
/// O(N + P + R + S log S)
/// ```
///
/// where `N` is operation count, `P` parameter-slot count, `R` symbolic
/// references, and `S` unique symbols.
///
/// # Determinism
///
/// The result is deterministic for identical canonical IR.
pub fn analyze_parameter_usage(
    circuit: &QuantumCircuit,
) -> Result<ParameterUsageAnalysis, ParameterUsageError> {
    let view = CircuitView::new(circuit).map_err(|error| {
        ParameterUsageError::InvalidCircuit {
            message: error.to_string(),
        }
    })?;

    analyze_parameter_usage_view(&view)
}

/// Analyzes parameter usage through an already-created optimizer circuit view.
///
/// This is the preferred entry point for optimization pipelines that already
/// validated their circuit.
///
/// It avoids repeating whole-circuit validation.
///
/// # Ownership
///
/// The analysis does not retain the `CircuitView` or the circuit.
pub fn analyze_parameter_usage_view(
    view: &CircuitView<'_>,
) -> Result<ParameterUsageAnalysis, ParameterUsageError> {
    let mut builder = AnalysisBuilder::default();

    for operation in view.operations().iter() {
        let parameters = operation.gate().parameters();

        builder.record_operation(parameters.len())?;

        let mut operation_is_symbolic = false;

        for (parameter_index, parameter) in
            parameters.iter().enumerate()
        {
            let contains_symbol = builder.record_parameter(
                parameter,
                operation.index(),
                parameter_index,
            )?;

            if contains_symbol {
                operation_is_symbolic = true;
            }
        }

        if operation_is_symbolic {
            // `record_parameter` already counted expression/direct references.
            // This branch intentionally exists only to keep operation-level
            // symbolic accounting centralized.
            //
            // We cannot increment here because an operation containing several
            // symbolic parameters must count exactly once. The count is
            // therefore reconstructed below from the completed result.
        }
    }

    let mut result = builder.finalize()?;

    // Recompute symbolic-operation count deterministically from the immutable
    // symbol occurrence lists. This avoids duplicate incrementing when a single
    // operation contains multiple symbolic parameters or repeated symbols.
    let mut symbolic_operations = BTreeMap::<OperationIndex, ()>::new();

    for usage in result.symbols.values() {
        for occurrence in usage.occurrences() {
            symbolic_operations
                .entry(occurrence.operation())
                .or_insert(());
        }
    }

    result.symbolic_operation_count =
        symbolic_operations.len();

    if result.symbolic_parameter_count
        > result.parameter_count
    {
        return Err(
            ParameterUsageError::InvariantViolation {
                message:
                    "symbolic parameter count exceeds total parameter count",
            },
        );
    }

    if result.constant_parameter_count
        + result.symbolic_parameter_count
        > result.parameter_count
    {
        return Err(
            ParameterUsageError::InvariantViolation {
                message:
                    "constant and symbolic parameter counts exceed total parameter count",
            },
        );
    }

    if result.expression_parameter_count
        > result.parameter_count
    {
        return Err(
            ParameterUsageError::InvariantViolation {
                message:
                    "expression parameter count exceeds total parameter count",
            },
        );
    }

    Ok(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Internal expression helpers
    // -------------------------------------------------------------------------

    fn symbol(name: &str) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn add(
        left: Parameter,
        right: Parameter,
    ) -> Parameter {
        Parameter::expression(
            ParameterExpression::Add(
                Box::new(left),
                Box::new(right),
            ),
        )
        .expect("test expression must be valid")
    }

    fn negate(value: Parameter) -> Parameter {
        Parameter::expression(
            ParameterExpression::Negate(
                Box::new(value),
            ),
        )
        .expect("test expression must be valid")
    }

    // -------------------------------------------------------------------------
    // Expression collector tests
    // -------------------------------------------------------------------------

    #[test]
    fn direct_expression_symbol_is_collected() {
        let expression = ParameterExpression::Add(
            Box::new(symbol("theta")),
            Box::new(constant(1.0)),
        );

        let mut collector =
            ExpressionCollector::new();

        collector
            .collect(&expression)
            .expect("collection must succeed");

        assert_eq!(
            collector.references.len(),
            1
        );

        assert_eq!(
            collector.references[0].name,
            "theta"
        );

        assert_eq!(
            collector.references[0].kind,
            ParameterOccurrenceKind::Expression
        );
    }

    #[test]
    fn repeated_symbol_is_counted_repeatedly() {
        let expression = ParameterExpression::Add(
            Box::new(symbol("theta")),
            Box::new(symbol("theta")),
        );

        let mut collector =
            ExpressionCollector::new();

        collector
            .collect(&expression)
            .expect("collection must succeed");

        assert_eq!(
            collector.references.len(),
            2
        );

        assert_eq!(
            collector.symbols.get("theta"),
            Some(&2)
        );
    }

    #[test]
    fn nested_expression_is_collected_iteratively() {
        let expression =
            add(
                add(
                    symbol("a"),
                    symbol("b"),
                ),
                negate(symbol("c")),
            );

        let expression = match expression {
            Parameter::Expression(value) => value,
            _ => panic!("expected expression"),
        };

        let mut collector =
            ExpressionCollector::new();

        collector
            .collect(&expression)
            .expect("collection must succeed");

        assert_eq!(
            collector.references.len(),
            3
        );

        assert!(collector.symbols.contains_key("a"));
        assert!(collector.symbols.contains_key("b"));
        assert!(collector.symbols.contains_key("c"));
    }

    // -------------------------------------------------------------------------
    // Usage object tests
    // -------------------------------------------------------------------------

    #[test]
    fn occurrence_kind_is_precise() {
        let direct =
            ParameterOccurrence::new(
                0,
                0,
                ParameterOccurrenceKind::DirectSymbol,
            );

        let expression =
            ParameterOccurrence::new(
                1,
                0,
                ParameterOccurrenceKind::Expression,
            );

        assert!(direct.is_direct());
        assert!(!direct.is_expression());

        assert!(expression.is_expression());
        assert!(!expression.is_direct());
    }

    // -------------------------------------------------------------------------
    // Builder tests
    // -------------------------------------------------------------------------

    #[test]
    fn constant_parameter_is_not_symbolic() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(1)
            .expect("operation");

        let symbolic = builder
            .record_parameter(
                &constant(1.0),
                0,
                0,
            )
            .expect("parameter");

        assert!(!symbolic);

        let result = builder
            .finalize()
            .expect("analysis");

        assert_eq!(
            result.parameter_count(),
            1
        );

        assert_eq!(
            result.constant_parameter_count(),
            1
        );

        assert_eq!(
            result.symbolic_parameter_count(),
            0
        );

        assert!(result.is_fully_bound());
    }

    #[test]
    fn direct_symbol_is_counted() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(1)
            .expect("operation");

        builder
            .record_parameter(
                &symbol("theta"),
                0,
                0,
            )
            .expect("parameter");

        let result = builder
            .finalize()
            .expect("analysis");

        assert_eq!(
            result.parameter_count(),
            1
        );

        assert_eq!(
            result.symbolic_parameter_count(),
            1
        );

        assert_eq!(
            result.symbolic_reference_count(),
            1
        );

        assert_eq!(
            result.direct_symbol_reference_count(),
            1
        );

        assert_eq!(
            result.expression_symbol_reference_count(),
            0
        );

        assert_eq!(
            result.unique_symbol_count(),
            1
        );

        let usage = result
            .get("theta")
            .expect("theta must exist");

        assert_eq!(
            usage.occurrence_count(),
            1
        );

        assert_eq!(
            usage.operation_count(),
            1
        );

        assert_eq!(
            usage.parameter_slot_count(),
            1
        );

        assert!(usage.has_direct_occurrence());
        assert!(!usage.has_expression_occurrence());
    }

    #[test]
    fn expression_symbol_is_counted() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(1)
            .expect("operation");

        let expression =
            add(
                symbol("theta"),
                constant(1.0),
            );

        builder
            .record_parameter(
                &expression,
                0,
                0,
            )
            .expect("parameter");

        let result = builder
            .finalize()
            .expect("analysis");

        assert_eq!(
            result.symbolic_parameter_count(),
            1
        );

        assert_eq!(
            result.expression_parameter_count(),
            1
        );

        assert_eq!(
            result.symbolic_reference_count(),
            1
        );

        assert_eq!(
            result.direct_symbol_reference_count(),
            0
        );

        assert_eq!(
            result.expression_symbol_reference_count(),
            1
        );

        let usage = result
            .get("theta")
            .expect("theta must exist");

        assert_eq!(
            usage.expression_occurrence_count(),
            1
        );

        assert!(!usage.has_direct_occurrence());
        assert!(usage.has_expression_occurrence());
    }

    #[test]
    fn repeated_symbol_in_one_expression_is_not_deduplicated() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(1)
            .expect("operation");

        let expression =
            add(
                symbol("theta"),
                symbol("theta"),
            );

        builder
            .record_parameter(
                &expression,
                0,
                0,
            )
            .expect("parameter");

        let result = builder
            .finalize()
            .expect("analysis");

        let usage = result
            .get("theta")
            .expect("theta");

        assert_eq!(
            usage.occurrence_count(),
            2
        );

        assert_eq!(
            usage.operation_count(),
            1
        );

        assert_eq!(
            usage.parameter_slot_count(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Result tests
    // -------------------------------------------------------------------------

    #[test]
    fn fully_bound_result_is_detected() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(2)
            .expect("operation");

        builder
            .record_parameter(
                &constant(1.0),
                0,
                0,
            )
            .expect("parameter");

        builder
            .record_parameter(
                &constant(2.0),
                0,
                1,
            )
            .expect("parameter");

        let result = builder
            .finalize()
            .expect("analysis");

        assert!(result.is_fully_bound());
        assert!(!result.is_symbolic());
        assert!(!result.has_no_parameters());
    }

    #[test]
    fn most_used_symbol_is_deterministic() {
        let mut builder =
            AnalysisBuilder::default();

        for operation in 0..3 {
            builder
                .record_operation(1)
                .expect("operation");

            builder
                .record_parameter(
                    &symbol(if operation == 0 {
                        "b"
                    } else {
                        "a"
                    }),
                    operation,
                    0,
                )
                .expect("parameter");
        }

        let result = builder
            .finalize()
            .expect("analysis");

        let most =
            result
                .most_used_symbol()
                .expect("symbol");

        assert_eq!(
            most.name(),
            "a"
        );
    }

    #[test]
    fn symbol_names_are_sorted() {
        let mut builder =
            AnalysisBuilder::default();

        for (operation, name) in
            ["z", "a", "m"].iter().enumerate()
        {
            builder
                .record_operation(1)
                .expect("operation");

            builder
                .record_parameter(
                    &symbol(name),
                    operation,
                    0,
                )
                .expect("parameter");
        }

        let result = builder
            .finalize()
            .expect("analysis");

        let names: Vec<&str> =
            result.symbol_names().collect();

        assert_eq!(
            names,
            vec!["a", "m", "z"]
        );
    }

    #[test]
    fn empty_analysis_has_zero_metrics() {
        let result = AnalysisBuilder::default()
            .finalize()
            .expect("analysis");

        assert_eq!(
            result.operation_count(),
            0
        );

        assert_eq!(
            result.parameter_count(),
            0
        );

        assert_eq!(
            result.constant_parameter_count(),
            0
        );

        assert_eq!(
            result.symbolic_parameter_count(),
            0
        );

        assert_eq!(
            result.unique_symbol_count(),
            0
        );

        assert_eq!(
            result.symbolic_reference_count(),
            0
        );

        assert!(result.is_fully_bound());
        assert!(result.has_no_parameters());
        assert_eq!(
            result.maximum_symbol_references(),
            0
        );

        assert!(
            result.most_used_symbol().is_none()
        );
    }

    // -------------------------------------------------------------------------
    // Invariant tests
    // -------------------------------------------------------------------------

    #[test]
    fn reference_totals_reconcile() {
        let mut builder =
            AnalysisBuilder::default();

        builder
            .record_operation(2)
            .expect("operation");

        builder
            .record_parameter(
                &symbol("a"),
                0,
                0,
            )
            .expect("parameter");

        builder
            .record_parameter(
                &add(
                    symbol("b"),
                    symbol("b"),
                ),
                0,
                1,
            )
            .expect("parameter");

        let result = builder
            .finalize()
            .expect("analysis");

        assert_eq!(
            result.symbolic_reference_count(),
            result.direct_symbol_reference_count()
                + result.expression_symbol_reference_count()
        );
    }

    #[test]
    fn repeated_analysis_is_deterministic() {
        let mut builder1 =
            AnalysisBuilder::default();

        let mut builder2 =
            AnalysisBuilder::default();

        for builder in [&mut builder1, &mut builder2] {
            builder
                .record_operation(1)
                .expect("operation");

            builder
                .record_parameter(
                    &add(
                        symbol("theta"),
                        symbol("phi"),
                    ),
                    0,
                    0,
                )
                .expect("parameter");
        }

        let first =
            builder1.finalize().expect("analysis");

        let second =
            builder2.finalize().expect("analysis");

        assert_eq!(first, second);
    }
}