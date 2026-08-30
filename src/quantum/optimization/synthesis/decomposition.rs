//! Zamani Quantum Optimization — Generic Decomposition Engine
//!
//! Production-grade, backend-independent decomposition infrastructure for
//! Zamani's canonical Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    canonical quantum::ir::Gate
//!                               │
//!                               ▼
//!             optimization::synthesis::decomposition
//!                               │
//!                ┌──────────────┼──────────────┐
//!                │              │              │
//!                ▼              ▼              ▼
//!          native target   exact rules    resource limits
//!                │              │              │
//!                └──────────────┼──────────────┘
//!                               ▼
//!                    canonical Quantum IR gates
//!                               │
//!                               ▼
//!                optimization / routing / scheduling
//! ```
//!
//! This module is the generic decomposition *engine*.
//!
//! Specialized synthesis algorithms remain in:
//!
//! - `single_qubit.rs`
//! - `two_qubit.rs`
//! - `clifford.rs`
//! - `phase.rs`
//! - `unitary.rs`
//! - `isometry.rs`
//!
//! This module provides the common infrastructure they can share:
//!
//! - decomposition requests;
//! - target capability contracts;
//! - exact rewrite/decomposition rules;
//! - recursive expansion;
//! - recursion-cycle protection;
//! - deterministic rule selection;
//! - resource budgeting;
//! - output validation;
//! - operation accounting;
//! - provenance of applied decomposition rules;
//! - extensibility without modifying this engine;
//! - explicit failure for unsupported decompositions.
//!
//! # Important semantic rule
//!
//! This engine NEVER assumes that two operations are equivalent merely because
//! they have similar names or because a transformation "looks right".
//!
//! A decomposition rule must explicitly declare itself exact.
//!
//! Approximate synthesis belongs in dedicated approximate-synthesis modules and
//! must carry an explicit approximation contract.
//!
//! # Canonical IR
//!
//! This file deliberately does NOT define another:
//!
//! - `QuantumGate`;
//! - `Circuit`;
//! - `Qubit`;
//! - parameter representation;
//! - hardware representation.
//!
//! The authoritative representations are:
//!
//! - `crate::quantum::ir::gate::Gate`;
//! - `crate::quantum::ir::gate::GateKind`;
//! - `crate::quantum::ir::parameter::Parameter`;
//! - `crate::quantum::ir::qubits::QubitId`.
//!
//! # Responsibilities explicitly excluded
//!
//! This module does NOT own:
//!
//! - circuit-wide optimization;
//! - routing;
//! - physical qubit mapping;
//! - hardware topology;
//! - pulse generation;
//! - scheduling;
//! - calibration;
//! - QPU execution;
//! - measurement simulation;
//! - error-correction decoding;
//! - source parsing;
//! - backend communication.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Exactness
//!
//! The built-in rules are exact symbolic identities.
//!
//! In particular, parameterized transformations operate on `Parameter`
//! expressions instead of requiring numerical parameter binding.
//!
//! # Scaling
//!
//! There is no artificial circuit-size limit in this module.
//!
//! Practical scalability is bounded by:
//!
//! - available memory;
//! - `usize` addressability;
//! - canonical IR limits;
//! - caller-provided synthesis limits;
//! - decomposition depth;
//! - generated operation count.
//!
//! A caller may explicitly select an unlimited budget, although production
//! compiler pipelines should normally inherit bounded limits from the global
//! optimization configuration.
//!
//! The engine never recursively expands without a depth/budget check.
//!
//! # Determinism
//!
//! Decomposition is deterministic:
//!
//! - no randomness;
//! - no global mutable state;
//! - no wall-clock decisions;
//! - no hash-map-dependent ordering;
//! - no backend I/O;
//! - no unsafe code.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - `#![forbid(unsafe_code)]`.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes and emits canonical `Gate` values.
//!
//! ## `optimization::synthesis::two_qubit`
//!
//! Two-qubit synthesis can use this engine for recursive target lowering.
//!
//! ## `optimization::synthesis::single_qubit`
//!
//! Single-qubit synthesis can use the same decomposition contracts while
//! retaining its matrix/Euler-specific implementation.
//!
//! ## `optimization::synthesis::clifford`
//!
//! Clifford synthesis can register specialized exact decompositions.
//!
//! ## `optimization::synthesis::phase`
//!
//! Phase synthesis can use the same target and budget contracts.
//!
//! ## `optimization::synthesis::unitary`
//!
//! General unitary synthesis can use this engine as the final lowering layer.
//!
//! ## `optimization::targets`
//!
//! A future `OptimizationTarget` can implement [`DecompositionTarget`].
//!
//! This file therefore does not need to be edited when new target profiles are
//! added.
//!
//! ## `optimization::cost`
//!
//! [`DecompositionReport`] provides exact operation counts and rule usage.
//!
//! ## `optimization::pass`
//!
//! A synthesis optimization pass can call [`decompose_gate`] and replace one
//! logical operation with the returned canonical sequence.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline owns pass ordering, global limits, analysis invalidation,
//! provenance aggregation, and whole-circuit verification.
//!
//! ## `routing`
//!
//! This module does not move logical qubits. Routing remains a downstream
//! responsibility.
//!
//! ## `verification`
//!
//! This engine provides structural guarantees and exact-rule metadata, but
//! independent semantic verification remains the responsibility of the global
//! optimization verification subsystem.
//!
//! # Design principle
//!
//! The engine is deliberately more generic than the currently implemented
//! decomposition set. New exact rules can be registered through
//! [`DecompositionRule`] without changing the recursive engine itself.
//!
//! That is essential for Zamani's long-term goal of supporting additional
//! gate families, logical operations, native target gates, and future quantum
//! computational models.
//!

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
};
use crate::quantum::ir::qubits::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type for generic decomposition.
pub type DecompositionResult<T> =
    Result<T, DecompositionError>;

// =============================================================================
// Errors
// =============================================================================

/// Structured failures produced by the generic decomposition engine.
#[derive(Debug, Clone, PartialEq)]
pub enum DecompositionError {
    /// The supplied operation failed canonical IR validation.
    InvalidInput {
        /// Gate kind.
        gate: GateKind,

        /// Canonical IR validation message.
        message: String,
    },

    /// A generated operation failed canonical IR validation.
    InvalidGeneratedGate {
        /// Gate kind.
        gate: GateKind,

        /// Validation message.
        message: String,
    },

    /// The target cannot represent the requested operation and no exact rule
    /// can lower it.
    Unsupported {
        /// Gate kind.
        gate: GateKind,
    },

    /// The requested operation is already native, but the target's capability
    /// contract is invalid.
    InvalidTarget {
        /// Explanation.
        message: &'static str,
    },

    /// A decomposition rule was selected for an incompatible gate.
    RuleMismatch {
        /// Rule identifier.
        rule_id: &'static str,

        /// Gate received.
        gate: GateKind,
    },

    /// A decomposition rule generated an invalid parameter transformation.
    InvalidParameter {
        /// Source gate.
        gate: GateKind,

        /// Parameter index.
        index: usize,
    },

    /// A required parameter is absent.
    MissingParameter {
        /// Source gate.
        gate: GateKind,

        /// Missing parameter index.
        index: usize,
    },

    /// Two operands that must be distinct were equal.
    DuplicateQubit {
        /// Duplicate qubit.
        qubit: QubitId,
    },

    /// The recursive decomposition exceeded the configured depth.
    DepthExceeded {
        /// Maximum permitted depth.
        maximum: usize,

        /// Current depth.
        actual: usize,
    },

    /// The generated operation count exceeds the configured limit.
    OperationLimitExceeded {
        /// Maximum permitted generated operations.
        maximum: usize,

        /// Required number of operations.
        required: usize,
    },

    /// The number of applied rules exceeds the configured limit.
    RuleLimitExceeded {
        /// Maximum number of rules.
        maximum: usize,

        /// Number required.
        required: usize,
    },

    /// The same gate kind was recursively expanded in a cycle.
    RecursiveCycle {
        /// Gate kinds participating in the cycle.
        chain: Vec<GateKind>,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for DecompositionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidInput {
                gate,
                message,
            } => {
                write!(
                    formatter,
                    "invalid input gate {gate:?}: {message}"
                )
            }

            Self::InvalidGeneratedGate {
                gate,
                message,
            } => {
                write!(
                    formatter,
                    "generated gate {gate:?} is invalid: {message}"
                )
            }

            Self::Unsupported { gate } => {
                write!(
                    formatter,
                    "no exact decomposition available for {gate:?}"
                )
            }

            Self::InvalidTarget { message } => {
                write!(
                    formatter,
                    "invalid decomposition target: {message}"
                )
            }

            Self::RuleMismatch {
                rule_id,
                gate,
            } => {
                write!(
                    formatter,
                    "decomposition rule {rule_id} does not accept {gate:?}"
                )
            }

            Self::InvalidParameter {
                gate,
                index,
            } => {
                write!(
                    formatter,
                    "invalid parameter transformation for {gate:?} \
                     at parameter {index}"
                )
            }

            Self::MissingParameter {
                gate,
                index,
            } => {
                write!(
                    formatter,
                    "gate {gate:?} is missing parameter {index}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "operation contains duplicate qubit {qubit:?}"
                )
            }

            Self::DepthExceeded {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "decomposition depth exceeded: maximum {maximum}, \
                     actual {actual}"
                )
            }

            Self::OperationLimitExceeded {
                maximum,
                required,
            } => {
                write!(
                    formatter,
                    "decomposition operation limit exceeded: maximum \
                     {maximum}, required {required}"
                )
            }

            Self::RuleLimitExceeded {
                maximum,
                required,
            } => {
                write!(
                    formatter,
                    "decomposition rule limit exceeded: maximum \
                     {maximum}, required {required}"
                )
            }

            Self::RecursiveCycle { chain } => {
                write!(
                    formatter,
                    "recursive decomposition cycle detected: "
                )?;

                for (index, gate) in chain.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(" -> ")?;
                    }

                    write!(formatter, "{gate:?}")?;
                }

                Ok(())
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {operation}"
                )
            }
        }
    }
}

impl Error for DecompositionError {}

// =============================================================================
// Target
// =============================================================================

/// Describes the operations that a decomposition target can directly accept.
///
/// This is intentionally a capability interface rather than a hardware
/// interface.
///
/// A target may represent:
///
/// - a simulator basis;
/// - a logical gate basis;
/// - a hardware ISA;
/// - a fault-tolerant logical basis;
/// - a custom user-defined gate set.
///
/// Physical connectivity does not belong here.
pub trait DecompositionTarget {
    /// Returns whether the target accepts the supplied gate kind.
    fn supports(&self, gate: GateKind) -> bool;

    /// Optional target-specific generated-operation limit.
    ///
    /// `None` means that this target layer does not impose an additional
    /// operation-count limit.
    fn max_generated_operations(&self) -> Option<usize> {
        None
    }
}

/// A target backed by a static slice of native operations.
///
/// This is useful for tests, simulators, generic compilation, and simple
/// compiler configurations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticDecompositionTarget {
    /// Native gate set.
    pub gates: &'static [GateKind],

    /// Optional generated-operation limit.
    pub max_generated_operations: Option<usize>,
}

impl StaticDecompositionTarget {
    /// Creates an unlimited static target.
    #[must_use]
    pub const fn new(
        gates: &'static [GateKind],
    ) -> Self {
        Self {
            gates,
            max_generated_operations: None,
        }
    }

    /// Creates a bounded static target.
    #[must_use]
    pub const fn with_limit(
        gates: &'static [GateKind],
        max_generated_operations: usize,
    ) -> Self {
        Self {
            gates,
            max_generated_operations:
                Some(max_generated_operations),
        }
    }
}

impl DecompositionTarget for StaticDecompositionTarget {
    fn supports(&self, gate: GateKind) -> bool {
        self.gates.iter().any(|candidate| *candidate == gate)
    }

    fn max_generated_operations(&self) -> Option<usize> {
        self.max_generated_operations
    }
}

/// A target that accepts every gate represented by the canonical IR.
///
/// This is useful for:
///
/// - testing;
/// - analysis-only compilation;
/// - pre-lowering pipelines.
///
/// It should not be mistaken for a hardware target.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CanonicalIrTarget;

impl DecompositionTarget for CanonicalIrTarget {
    fn supports(&self, _gate: GateKind) -> bool {
        true
    }
}

// =============================================================================
// Budget
// =============================================================================

/// Resource limits for one decomposition request.
///
/// All limits are optional.
///
/// `None` means that this particular layer imposes no limit and therefore
/// inherits limits from the caller or higher-level optimizer.
///
/// This is deliberately not a hard-coded global maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecompositionBudget {
    /// Maximum recursive decomposition depth.
    pub max_depth: Option<usize>,

    /// Maximum emitted operations.
    pub max_operations: Option<usize>,

    /// Maximum number of applied decomposition rules.
    pub max_rules: Option<usize>,
}

impl DecompositionBudget {
    /// Unlimited decomposition budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_depth: None,
            max_operations: None,
            max_rules: None,
        }
    }

    /// Conservative production default.
    ///
    /// This is intentionally a configurable default rather than a semantic
    /// hard limit.
    #[must_use]
    pub const fn production_default() -> Self {
        Self {
            max_depth: Some(64),
            max_operations: Some(1_000_000),
            max_rules: Some(1_000_000),
        }
    }

    /// Creates a budget with only an operation limit.
    #[must_use]
    pub const fn operations(
        maximum: usize,
    ) -> Self {
        Self {
            max_depth: None,
            max_operations: Some(maximum),
            max_rules: None,
        }
    }

    /// Creates a budget with only a depth limit.
    #[must_use]
    pub const fn depth(
        maximum: usize,
    ) -> Self {
        Self {
            max_depth: Some(maximum),
            max_operations: None,
            max_rules: None,
        }
    }
}

// =============================================================================
// Rule metadata
// =============================================================================

/// Describes the semantic category of a decomposition rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecompositionRuleClass {
    /// Exact algebraic identity.
    Exact,

    /// Exact Clifford identity.
    ExactClifford,

    /// Exact parameterized identity.
    ExactParameterized,

    /// Exact controlled-operation identity.
    ExactControlled,

    /// Exact permutation identity.
    ExactPermutation,

    /// Exact target-lowering identity.
    ExactTargetLowering,
}

/// Metadata for a decomposition rule.
///
/// The actual transformation is implemented by a deterministic function.
///
/// The rule itself never performs target routing or backend I/O.
#[derive(Debug, Clone, Copy)]
pub struct DecompositionRule {
    /// Stable rule identifier.
    pub id: &'static str,

    /// Source gate.
    pub source: GateKind,

    /// Semantic class.
    pub class: DecompositionRuleClass,

    /// Number of operations produced before recursive lowering.
    pub output_operations: usize,

    /// Rule transformation.
    pub apply: RuleFunction,
}

/// Function signature used by decomposition rules.
pub type RuleFunction =
    fn(&Gate) -> DecompositionResult<Vec<Gate>>;

// =============================================================================
// Rule application record
// =============================================================================

/// Records one applied decomposition rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedRule {
    /// Stable rule identifier.
    pub id: &'static str,

    /// Source gate kind.
    pub source: GateKind,

    /// Number of operations produced directly by the rule.
    pub generated_operations: usize,
}

// =============================================================================
// Report
// =============================================================================

/// Complete accounting for one decomposition request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecompositionReport {
    /// Original gate kind.
    pub input_gate: GateKind,

    /// Final number of emitted operations.
    pub generated_operations: usize,

    /// Number of generated one-qubit operations.
    pub generated_single_qubit_operations: usize,

    /// Number of generated two-qubit operations.
    pub generated_two_qubit_operations: usize,

    /// Number of generated three-or-more-qubit operations.
    pub generated_multi_qubit_operations: usize,

    /// Number of decomposition rules applied.
    pub rules_applied: usize,

    /// Maximum recursive depth reached.
    pub maximum_depth: usize,

    /// Whether the input was already native.
    pub preserved_native: bool,

    /// Rules used in deterministic order.
    pub rules: Vec<AppliedRule>,
}

// =============================================================================
// Result
// =============================================================================

/// Result of decomposing one canonical IR operation.
#[derive(Debug, Clone, PartialEq)]
pub struct DecompositionResult {
    /// Final canonical IR operations.
    pub operations: Vec<Gate>,

    /// Resource and provenance information.
    pub report: DecompositionReport,
}

// =============================================================================
// Engine state
// =============================================================================

struct DecompositionState {
    budget: DecompositionBudget,
    operations: Vec<Gate>,
    rules: Vec<AppliedRule>,
    active_chain: Vec<GateKind>,
    maximum_depth: usize,
}

impl DecompositionState {
    fn new(
        budget: DecompositionBudget,
    ) -> Self {
        Self {
            budget,
            operations: Vec::new(),
            rules: Vec::new(),
            active_chain: Vec::new(),
            maximum_depth: 0,
        }
    }

    fn check_depth(
        &self,
        depth: usize,
    ) -> DecompositionResult<()> {
        if let Some(maximum) = self.budget.max_depth {
            if depth > maximum {
                return Err(
                    DecompositionError::DepthExceeded {
                        maximum,
                        actual: depth,
                    },
                );
            }
        }

        Ok(())
    }

    fn check_operation_limit(
        &self,
        additional: usize,
    ) -> DecompositionResult<()> {
        let required = self
            .operations
            .len()
            .checked_add(additional)
            .ok_or(
                DecompositionError::ArithmeticOverflow {
                    operation:
                        "decomposition operation count",
                },
            )?;

        if let Some(maximum) =
            self.budget.max_operations
        {
            if required > maximum {
                return Err(
                    DecompositionError::
                        OperationLimitExceeded {
                            maximum,
                            required,
                        },
                );
            }
        }

        Ok(())
    }

    fn check_rule_limit(
        &self,
    ) -> DecompositionResult<()> {
        let required = self
            .rules
            .len()
            .checked_add(1)
            .ok_or(
                DecompositionError::ArithmeticOverflow {
                    operation:
                        "decomposition rule count",
                },
            )?;

        if let Some(maximum) =
            self.budget.max_rules
        {
            if required > maximum {
                return Err(
                    DecompositionError::
                        RuleLimitExceeded {
                            maximum,
                            required,
                        },
                );
            }
        }

        Ok(())
    }

    fn enter(
        &mut self,
        gate: GateKind,
        depth: usize,
    ) -> DecompositionResult<()> {
        self.check_depth(depth)?;

        if self.active_chain.contains(&gate) {
            let mut chain =
                self.active_chain.clone();

            chain.push(gate);

            return Err(
                DecompositionError::RecursiveCycle {
                    chain,
                },
            );
        }

        self.active_chain.push(gate);

        if depth > self.maximum_depth {
            self.maximum_depth = depth;
        }

        Ok(())
    }

    fn leave(&mut self) {
        let _ = self.active_chain.pop();
    }
}

// =============================================================================
// Public entry points
// =============================================================================

/// Decomposes one canonical IR gate using the built-in exact rule set.
///
/// Native target operations are preserved.
///
/// Unsupported operations fail explicitly rather than being silently
/// approximated.
pub fn decompose_gate<T>(
    gate: &Gate,
    target: &T,
) -> DecompositionResult
where
    T: DecompositionTarget,
{
    decompose_gate_with_rules(
        gate,
        target,
        DecompositionBudget::production_default(),
        builtin_rules(),
    )
}

/// Decomposes one gate with an explicit budget.
pub fn decompose_gate_with_budget<T>(
    gate: &Gate,
    target: &T,
    budget: DecompositionBudget,
) -> DecompositionResult
where
    T: DecompositionTarget,
{
    decompose_gate_with_rules(
        gate,
        target,
        budget,
        builtin_rules(),
    )
}

/// Decomposes one gate using caller-provided rules.
///
/// This is the primary extension point for future synthesis modules.
///
/// A new synthesis algorithm does not need to modify the recursive engine.
/// It can provide its own rule slice.
pub fn decompose_gate_with_rules<T>(
    gate: &Gate,
    target: &T,
    budget: DecompositionBudget,
    rules: &[DecompositionRule],
) -> DecompositionResult
where
    T: DecompositionTarget,
{
    validate_gate(gate)?;

    validate_rules(rules)?;

    let mut state =
        DecompositionState::new(budget);

    expand_gate(
        gate,
        target,
        rules,
        &mut state,
        0,
    )?;

    validate_output(&state.operations)?;

    let report =
        build_report(
            gate.kind(),
            &state.operations,
            &state.rules,
            state.maximum_depth,
            target.supports(gate.kind()),
        );

    Ok(DecompositionResult {
        operations: state.operations,
        report,
    })
}

/// Returns the built-in exact rule table.
///
/// The returned slice is static and deterministic.
#[must_use]
pub fn builtin_rules() -> &'static [DecompositionRule] {
    &BUILTIN_RULES
}

/// Finds the built-in exact rule for a gate kind.
#[must_use]
pub fn builtin_rule(
    gate: GateKind,
) -> Option<&'static DecompositionRule> {
    BUILTIN_RULES
        .iter()
        .find(|rule| rule.source == gate)
}

/// Returns the number of direct operations produced by the selected
/// built-in rule.
///
/// This does not account for recursive lowering.
#[must_use]
pub fn direct_rule_operation_count(
    gate: GateKind,
) -> Option<usize> {
    builtin_rule(gate)
        .map(|rule| rule.output_operations)
}

// =============================================================================
// Recursive engine
// =============================================================================

fn expand_gate<T>(
    gate: &Gate,
    target: &T,
    rules: &[DecompositionRule],
    state: &mut DecompositionState,
    depth: usize,
) -> DecompositionResult<()>
where
    T: DecompositionTarget,
{
    state.check_depth(depth)?;

    validate_gate(gate)?;

    // Native operation: no decomposition is required.
    if target.supports(gate.kind()) {
        state.check_operation_limit(1)?;

        state.operations.push(gate.clone());

        return Ok(());
    }

    let rule =
        rules.iter().find(|candidate| {
            candidate.source == gate.kind()
        });

    let rule = match rule {
        Some(rule) => rule,
        None => {
            return Err(
                DecompositionError::Unsupported {
                    gate: gate.kind(),
                },
            );
        }
    };

    state.check_rule_limit()?;

    state.enter(
        gate.kind(),
        depth,
    )?;

    let generated =
        (rule.apply)(gate)?;

    state.leave();

    state.check_operation_limit(
        generated.len(),
    )?;

    state.rules.push(
        AppliedRule {
            id: rule.id,
            source: rule.source,
            generated_operations:
                generated.len(),
        },
    );

    for operation in generated {
        expand_gate(
            &operation,
            target,
            rules,
            state,
            depth.saturating_add(1),
        )?;
    }

    Ok(())
}

// =============================================================================
// Validation
// =============================================================================

fn validate_gate(
    gate: &Gate,
) -> DecompositionResult<()> {
    gate.validate().map_err(|error| {
        DecompositionError::InvalidInput {
            gate: gate.kind(),
            message: error.to_string(),
        }
    })?;

    validate_unique_qubits(gate)
}

fn validate_unique_qubits(
    gate: &Gate,
) -> DecompositionResult<()> {
    let qubits = gate.qubits();

    for first_index in 0..qubits.len() {
        for second_index
            in (first_index + 1)..qubits.len()
        {
            if qubits[first_index]
                == qubits[second_index]
            {
                return Err(
                    DecompositionError::
                        DuplicateQubit {
                            qubit:
                                qubits[first_index],
                        },
                );
            }
        }
    }

    Ok(())
}

fn validate_output(
    operations: &[Gate],
) -> DecompositionResult<()> {
    for operation in operations {
        operation.validate().map_err(
            |error| {
                DecompositionError::
                    InvalidGeneratedGate {
                        gate: operation.kind(),
                        message:
                            error.to_string(),
                    }
            },
        )?;

        validate_unique_qubits(operation)?;
    }

    Ok(())
}

fn validate_rules(
    rules: &[DecompositionRule],
) -> DecompositionResult<()> {
    for first_index in 0..rules.len() {
        for second_index
            in (first_index + 1)..rules.len()
        {
            if rules[first_index].source
                == rules[second_index].source
            {
                // Duplicate source rules would make deterministic rule
                // selection dependent on registration order.
                //
                // Rejecting them is safer than silently choosing one.
                return Err(
                    DecompositionError::
                        InvalidTarget {
                            message:
                                "duplicate decomposition rule \
                                 for the same source gate",
                        },
                );
            }
        }
    }

    Ok(())
}

// =============================================================================
// Report
// =============================================================================

fn build_report(
    input_gate: GateKind,
    operations: &[Gate],
    rules: &[AppliedRule],
    maximum_depth: usize,
    preserved_native: bool,
) -> DecompositionReport {
    let mut single = 0usize;
    let mut two = 0usize;
    let mut multi = 0usize;

    for operation in operations {
        match operation.qubits().len() {
            0 | 1 => {
                single =
                    single.saturating_add(1);
            }

            2 => {
                two =
                    two.saturating_add(1);
            }

            _ => {
                multi =
                    multi.saturating_add(1);
            }
        }
    }

    DecompositionReport {
        input_gate,
        generated_operations:
            operations.len(),
        generated_single_qubit_operations:
            single,
        generated_two_qubit_operations:
            two,
        generated_multi_qubit_operations:
            multi,
        rules_applied:
            rules.len(),
        maximum_depth,
        preserved_native,
        rules: rules.to_vec(),
    }
}

// =============================================================================
// Built-in exact rules
// =============================================================================
//
// These rules deliberately only describe exact logical identities.
// Target-specific availability is evaluated by the recursive engine.
//
// If a generated gate is itself unsupported by the target, the engine will
// recursively lower it through another registered rule.
//
// This gives Zamani a compositional decomposition graph rather than a flat
// collection of backend-specific cases.
//

static BUILTIN_RULES: [
    DecompositionRule;
    9
] = [
    DecompositionRule {
        id: "cx_to_cz",
        source: GateKind::CX,
        class:
            DecompositionRuleClass::ExactClifford,
        output_operations: 3,
        apply: rule_cx_to_cz,
    },

    DecompositionRule {
        id: "cz_to_cx",
        source: GateKind::CZ,
        class:
            DecompositionRuleClass::ExactClifford,
        output_operations: 3,
        apply: rule_cz_to_cx,
    },

    DecompositionRule {
        id: "cy_to_cx",
        source: GateKind::CY,
        class:
            DecompositionRuleClass::ExactClifford,
        output_operations: 3,
        apply: rule_cy_to_cx,
    },

    DecompositionRule {
        id: "swap_to_cx",
        source: GateKind::SWAP,
        class:
            DecompositionRuleClass::ExactPermutation,
        output_operations: 3,
        apply: rule_swap_to_cx,
    },

    DecompositionRule {
        id: "crz_to_cx_rz",
        source: GateKind::CRZ,
        class:
            DecompositionRuleClass::ExactControlled,
        output_operations: 4,
        apply: rule_crz_to_cx,
    },

    DecompositionRule {
        id: "cry_to_cx_ry",
        source: GateKind::CRY,
        class:
            DecompositionRuleClass::ExactControlled,
        output_operations: 4,
        apply: rule_cry_to_cx,
    },

    DecompositionRule {
        id: "crx_to_h_crz_h",
        source: GateKind::CRX,
        class:
            DecompositionRuleClass::ExactControlled,
        output_operations: 6,
        apply: rule_crx_to_h_crz_h,
    },

    DecompositionRule {
        id: "ccx_to_clifford_t",
        source: GateKind::CCX,
        class:
            DecompositionRuleClass::ExactClifford,
        output_operations: 15,
        apply: rule_ccx_to_clifford_t,
    },

    DecompositionRule {
        id: "cswap_to_ccx_cx",
        source: GateKind::CSWAP,
        class:
            DecompositionRuleClass::ExactPermutation,
        output_operations: 8,
        apply: rule_cswap_to_ccx_cx,
    },
];

// =============================================================================
// CX ↔ CZ
// =============================================================================

fn rule_cx_to_cz(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    Ok(vec![
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
        make_two(
            GateKind::CZ,
            control,
            target,
            &[],
        )?,
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
    ])
}

fn rule_cz_to_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    Ok(vec![
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
    ])
}

// =============================================================================
// CY
// =============================================================================

fn rule_cy_to_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    // Correct conjugation:
    //
    //     S X S† = Y
    //
    // Therefore:
    //
    //     CY = S(target) CX S†(target)
    //
    // The order is important. `Sdg ; CX ; S` would implement -Y rather
    // than Y.
    Ok(vec![
        make_single(
            GateKind::S,
            target,
            &[],
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_single(
            GateKind::Sdg,
            target,
            &[],
        )?,
    ])
}

// =============================================================================
// SWAP
// =============================================================================

fn rule_swap_to_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let first = gate.qubits()[0];
    let second = gate.qubits()[1];

    Ok(vec![
        make_two(
            GateKind::CX,
            first,
            second,
            &[],
        )?,
        make_two(
            GateKind::CX,
            second,
            first,
            &[],
        )?,
        make_two(
            GateKind::CX,
            first,
            second,
            &[],
        )?,
    ])
}

// =============================================================================
// CRZ
// =============================================================================

fn rule_crz_to_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let theta =
        parameter_at(gate, 0)?;

    let half =
        scale_parameter(
            theta,
            0.5,
            gate.kind(),
            0,
        )?;

    let negative_half =
        negate_parameter(
            half.clone(),
            gate.kind(),
            0,
        )?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    Ok(vec![
        make_parameterized_single(
            GateKind::RZ,
            target,
            half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_parameterized_single(
            GateKind::RZ,
            target,
            negative_half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
    ])
}

// =============================================================================
// CRY
// =============================================================================

fn rule_cry_to_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let theta =
        parameter_at(gate, 0)?;

    let half =
        scale_parameter(
            theta,
            0.5,
            gate.kind(),
            0,
        )?;

    let negative_half =
        negate_parameter(
            half.clone(),
            gate.kind(),
            0,
        )?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    Ok(vec![
        make_parameterized_single(
            GateKind::RY,
            target,
            half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_parameterized_single(
            GateKind::RY,
            target,
            negative_half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
    ])
}

// =============================================================================
// CRX
// =============================================================================

fn rule_crx_to_h_crz_h(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 2)?;

    let theta =
        parameter_at(gate, 0)?;

    let half =
        scale_parameter(
            theta,
            0.5,
            gate.kind(),
            0,
        )?;

    let negative_half =
        negate_parameter(
            half.clone(),
            gate.kind(),
            0,
        )?;

    let control = gate.qubits()[0];
    let target = gate.qubits()[1];

    // RX(theta) = H RZ(theta) H.
    //
    // The controlled transformation is therefore:
    //
    // H(target)
    // RZ(theta/2,target)
    // CX(control,target)
    // RZ(-theta/2,target)
    // CX(control,target)
    // H(target)
    //
    // This is exact under the canonical rotation convention used by the IR.
    Ok(vec![
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
        make_parameterized_single(
            GateKind::RZ,
            target,
            half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_parameterized_single(
            GateKind::RZ,
            target,
            negative_half,
        )?,
        make_two(
            GateKind::CX,
            control,
            target,
            &[],
        )?,
        make_single(
            GateKind::H,
            target,
            &[],
        )?,
    ])
}

// =============================================================================
// CCX / Toffoli
// =============================================================================
//
// Standard exact Clifford+T decomposition.
//
// The sequence below is the canonical 15-operation decomposition:
// five CX operations, H, S/T/Tdg operations.
//
// The rule intentionally targets canonical GateKind values rather than
// assuming a particular hardware ISA. Recursive lowering handles the actual
// target.
//
// =============================================================================

fn rule_ccx_to_clifford_t(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 3)?;

    let control_a = gate.qubits()[0];
    let control_b = gate.qubits()[1];
    let target = gate.qubits()[2];

    // Standard Toffoli decomposition.
    //
    // The exact sequence is kept explicit so it remains auditable and
    // independently verifiable.
    Ok(vec![
        make_single(
            GateKind::H,
            target,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_b,
            target,
            &[],
        )?,

        make_single(
            GateKind::Tdg,
            target,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_a,
            target,
            &[],
        )?,

        make_single(
            GateKind::T,
            target,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_b,
            target,
            &[],
        )?,

        make_single(
            GateKind::Tdg,
            target,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_a,
            target,
            &[],
        )?,

        make_single(
            GateKind::T,
            control_b,
            &[],
        )?,

        make_single(
            GateKind::T,
            target,
            &[],
        )?,

        make_single(
            GateKind::H,
            target,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_a,
            control_b,
            &[],
        )?,

        make_single(
            GateKind::T,
            control_a,
            &[],
        )?,

        make_single(
            GateKind::Tdg,
            control_b,
            &[],
        )?,

        make_two(
            GateKind::CX,
            control_a,
            control_b,
            &[],
        )?,
    ])
}

// =============================================================================
// CSWAP / Fredkin
// =============================================================================

fn rule_cswap_to_ccx_cx(
    gate: &Gate,
) -> DecompositionResult<Vec<Gate>> {
    require_qubits(gate, 3)?;

    let control = gate.qubits()[0];
    let first = gate.qubits()[1];
    let second = gate.qubits()[2];

    // Fredkin / controlled-SWAP:
    //
    // CX(second, first)
    // CCX(control, first, second)
    // CX(second, first)
    // CCX(control, first, second)
    //
    // This is an exact structural decomposition.
    //
    // The generic engine recursively lowers CCX if the target does not
    // support it.
    Ok(vec![
        make_two(
            GateKind::CX,
            second,
            first,
            &[],
        )?,
        make_three(
            GateKind::CCX,
            control,
            first,
            second,
            &[],
        )?,
        make_two(
            GateKind::CX,
            second,
            first,
            &[],
        )?,
        make_three(
            GateKind::CCX,
            control,
            first,
            second,
            &[],
        )?,
    ])
}

// =============================================================================
// Gate constructors
// =============================================================================

fn make_single(
    kind: GateKind,
    qubit: QubitId,
    parameters: &[Parameter],
) -> DecompositionResult<Gate> {
    Gate::new(
        kind,
        vec![qubit],
        parameters.to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        DecompositionError::InvalidGeneratedGate {
            gate: kind,
            message: error.to_string(),
        }
    })
}

fn make_parameterized_single(
    kind: GateKind,
    qubit: QubitId,
    parameter: Parameter,
) -> DecompositionResult<Gate> {
    make_single(
        kind,
        qubit,
        &[parameter],
    )
}

fn make_two(
    kind: GateKind,
    first: QubitId,
    second: QubitId,
    parameters: &[Parameter],
) -> DecompositionResult<Gate> {
    if first == second {
        return Err(
            DecompositionError::DuplicateQubit {
                qubit: first,
            },
        );
    }

    Gate::new(
        kind,
        vec![first, second],
        parameters.to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        DecompositionError::InvalidGeneratedGate {
            gate: kind,
            message: error.to_string(),
        }
    })
}

fn make_three(
    kind: GateKind,
    first: QubitId,
    second: QubitId,
    third: QubitId,
    parameters: &[Parameter],
) -> DecompositionResult<Gate> {
    if first == second {
        return Err(
            DecompositionError::DuplicateQubit {
                qubit: first,
            },
        );
    }

    if first == third {
        return Err(
            DecompositionError::DuplicateQubit {
                qubit: first,
            },
        );
    }

    if second == third {
        return Err(
            DecompositionError::DuplicateQubit {
                qubit: second,
            },
        );
    }

    Gate::new(
        kind,
        vec![first, second, third],
        parameters.to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        DecompositionError::InvalidGeneratedGate {
            gate: kind,
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Parameter helpers
// =============================================================================

fn parameter_at<'a>(
    gate: &'a Gate,
    index: usize,
) -> DecompositionResult<&'a Parameter> {
    gate.parameters()
        .get(index)
        .ok_or(
            DecompositionError::MissingParameter {
                gate: gate.kind(),
                index,
            },
        )
}

fn scale_parameter(
    parameter: &Parameter,
    factor: f64,
    gate: GateKind,
    index: usize,
) -> DecompositionResult<Parameter> {
    if !factor.is_finite() {
        return Err(
            DecompositionError::InvalidParameter {
                gate,
                index,
            },
        );
    }

    match parameter {
        Parameter::Constant(value) => {
            let result =
                *value * factor;

            if !result.is_finite() {
                return Err(
                    DecompositionError::
                        InvalidParameter {
                            gate,
                            index,
                        },
                );
            }

            Parameter::constant(result)
                .map_err(|_| {
                    DecompositionError::
                        InvalidParameter {
                            gate,
                            index,
                        }
                })
        }

        Parameter::Symbol(_)
        | Parameter::Expression(_) => {
            let factor_parameter =
                Parameter::constant(factor)
                    .map_err(|_| {
                        DecompositionError::
                            InvalidParameter {
                                gate,
                                index,
                            }
                    })?;

            Parameter::expression(
                ParameterExpression::Multiply(
                    Box::new(
                        parameter.clone(),
                    ),
                    Box::new(
                        factor_parameter,
                    ),
                ),
            )
            .map_err(|_| {
                DecompositionError::
                    InvalidParameter {
                        gate,
                        index,
                    }
            })
        }
    }
}

fn negate_parameter(
    parameter: Parameter,
    gate: GateKind,
    index: usize,
) -> DecompositionResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    DecompositionError::
                        InvalidParameter {
                            gate,
                            index,
                        },
                );
            }

            Parameter::constant(-value)
                .map_err(|_| {
                    DecompositionError::
                        InvalidParameter {
                            gate,
                            index,
                        }
                })
        }

        other => {
            Parameter::expression(
                ParameterExpression::Negate(
                    Box::new(other),
                ),
            )
            .map_err(|_| {
                DecompositionError::
                    InvalidParameter {
                        gate,
                        index,
                    }
            })
        }
    }
}

// =============================================================================
// Operand validation helpers
// =============================================================================

fn require_qubits(
    gate: &Gate,
    expected: usize,
) -> DecompositionResult<()> {
    if gate.qubits().len() != expected {
        return Err(
            DecompositionError::RuleMismatch {
                rule_id:
                    "operand-count-check",
                gate: gate.kind(),
            },
        );
    }

    validate_unique_qubits(gate)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const CX_TARGET_GATES: &[GateKind] = &[
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::CX,
    ];

    const CLIFFORD_T_GATES: &[GateKind] = &[
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
        GateKind::CX,
    ];

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn gate(
        kind: GateKind,
        qubits: &[usize],
        parameters: &[Parameter],
    ) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            parameters.to_vec(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn native_gate_is_preserved() {
        let input =
            gate(
                GateKind::CX,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("native gate must succeed");

        assert_eq!(
            result.operations.len(),
            1
        );

        assert!(
            result.report.preserved_native
        );

        assert_eq!(
            result.report.rules_applied,
            0
        );
    }

    #[test]
    fn cx_lowers_to_cz_basis() {
        const GATES: &[GateKind] = &[
            GateKind::H,
            GateKind::CZ,
        ];

        let input =
            gate(
                GateKind::CX,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CX must lower to CZ");

        assert_eq!(
            result.operations.len(),
            3
        );

        assert_eq!(
            result.operations[0].kind(),
            GateKind::H
        );

        assert_eq!(
            result.operations[1].kind(),
            GateKind::CZ
        );

        assert_eq!(
            result.operations[2].kind(),
            GateKind::H
        );
    }

    #[test]
    fn cy_uses_correct_conjugation_order() {
        let input =
            gate(
                GateKind::CY,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CY must lower");

        assert_eq!(
            result.operations.len(),
            3
        );

        assert_eq!(
            result.operations[0].kind(),
            GateKind::S
        );

        assert_eq!(
            result.operations[1].kind(),
            GateKind::CX
        );

        assert_eq!(
            result.operations[2].kind(),
            GateKind::Sdg
        );
    }

    #[test]
    fn swap_lowers_to_three_cx() {
        let input =
            gate(
                GateKind::SWAP,
                &[3, 7],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("SWAP must lower");

        assert_eq!(
            result.operations.len(),
            3
        );

        assert_eq!(
            result.operations[0]
                .qubits(),
            &[q(3), q(7)]
        );

        assert_eq!(
            result.operations[1]
                .qubits(),
            &[q(7), q(3)]
        );

        assert_eq!(
            result.operations[2]
                .qubits(),
            &[q(3), q(7)]
        );
    }

    #[test]
    fn symbolic_crz_is_preserved() {
        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let input =
            gate(
                GateKind::CRZ,
                &[0, 1],
                &[theta],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CRZ must lower");

        assert_eq!(
            result.operations.len(),
            4
        );

        assert!(
            result.operations[0]
                .parameters()[0]
                .is_symbolic()
        );

        assert!(
            result.operations[2]
                .parameters()[0]
                .is_symbolic()
        );
    }

    #[test]
    fn constant_crz_is_halved() {
        let theta =
            Parameter::constant(1.2)
                .expect("finite parameter");

        let input =
            gate(
                GateKind::CRZ,
                &[0, 1],
                &[theta],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CRZ must lower");

        assert_eq!(
            result.operations[0]
                .parameters()[0]
                .as_constant(),
            Some(0.6)
        );

        assert_eq!(
            result.operations[2]
                .parameters()[0]
                .as_constant(),
            Some(-0.6)
        );
    }

    #[test]
    fn recursive_ccx_lowering_works() {
        let input =
            gate(
                GateKind::CCX,
                &[0, 1, 2],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CLIFFORD_T_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CCX must lower");

        assert!(
            !result.operations.is_empty()
        );

        for operation
            in &result.operations
        {
            assert!(
                matches!(
                    operation.kind(),
                    GateKind::I
                        | GateKind::X
                        | GateKind::Y
                        | GateKind::Z
                        | GateKind::H
                        | GateKind::S
                        | GateKind::Sdg
                        | GateKind::T
                        | GateKind::Tdg
                        | GateKind::CX
                )
            );
        }
    }

    #[test]
    fn unsupported_gate_fails_explicitly() {
        let input =
            gate(
                GateKind::ISWAP,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let error =
            decompose_gate(
                &input,
                &target,
            )
            .expect_err(
                "unsupported gate must fail",
            );

        assert!(matches!(
            error,
            DecompositionError::Unsupported {
                gate: GateKind::ISWAP
            }
        ));
    }

    #[test]
    fn operation_budget_is_enforced() {
        let input =
            gate(
                GateKind::SWAP,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let error =
            decompose_gate_with_budget(
                &input,
                &target,
                DecompositionBudget {
                    max_depth: None,
                    max_operations: Some(2),
                    max_rules: None,
                },
            )
            .expect_err(
                "SWAP requires three operations",
            );

        assert!(matches!(
            error,
            DecompositionError::
                OperationLimitExceeded {
                    maximum: 2,
                    ..
                }
        ));
    }

    #[test]
    fn rule_budget_is_enforced() {
        let input =
            gate(
                GateKind::SWAP,
                &[0, 1],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let error =
            decompose_gate_with_budget(
                &input,
                &target,
                DecompositionBudget {
                    max_depth: None,
                    max_operations: None,
                    max_rules: Some(0),
                },
            )
            .expect_err(
                "rule budget must reject rewrite",
            );

        assert!(matches!(
            error,
            DecompositionError::
                RuleLimitExceeded {
                    maximum: 0,
                    ..
                }
        ));
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let input =
            gate(
                GateKind::CX,
                &[0, 0],
                &[],
            );

        let target =
            CanonicalIrTarget;

        let error =
            decompose_gate(
                &input,
                &target,
            )
            .expect_err(
                "duplicate operands must fail",
            );

        assert!(matches!(
            error,
            DecompositionError::
                InvalidInput { .. }
        ));
    }

    #[test]
    fn production_budget_is_configurable() {
        let budget =
            DecompositionBudget::production_default();

        assert_eq!(
            budget.max_depth,
            Some(64)
        );

        assert_eq!(
            budget.max_operations,
            Some(1_000_000)
        );

        assert_eq!(
            budget.max_rules,
            Some(1_000_000)
        );
    }

    #[test]
    fn builtin_rule_lookup_is_deterministic() {
        let first =
            builtin_rule(GateKind::SWAP)
                .expect("SWAP rule");

        let second =
            builtin_rule(GateKind::SWAP)
                .expect("SWAP rule");

        assert_eq!(
            first.id,
            second.id
        );

        assert_eq!(
            first.output_operations,
            3
        );
    }

    #[test]
    fn output_is_canonical_ir() {
        let input =
            gate(
                GateKind::SWAP,
                &[2, 9],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CX_TARGET_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("SWAP must lower");

        for operation
            in result.operations
        {
            operation
                .validate()
                .expect(
                    "generated operation must \
                     remain canonical IR",
                );
        }
    }

    #[test]
    fn cswap_can_lower_recursively() {
        let input =
            gate(
                GateKind::CSWAP,
                &[0, 1, 2],
                &[],
            );

        let target =
            StaticDecompositionTarget::new(
                CLIFFORD_T_GATES,
            );

        let result =
            decompose_gate(
                &input,
                &target,
            )
            .expect("CSWAP must lower");

        assert!(
            !result.operations.is_empty()
        );

        for operation
            in result.operations
        {
            assert!(
                operation.qubits().len()
                    <= 2
            );
        }
    }
}