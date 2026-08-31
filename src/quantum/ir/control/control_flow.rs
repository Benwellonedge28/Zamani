//! Zamani Quantum IR — Structured Control Flow
//!
//! Canonical, target-independent representation of structured control flow
//! for hybrid classical/quantum programs.
//!
//! # Architectural role
//!
//! This module represents the semantic meaning of control flow.
//!
//! It supports:
//!
//! - `if` / `else`;
//! - `while`;
//! - `do while`;
//! - counted/range loops;
//! - iteration over logical-qubit ranges;
//! - repeat loops;
//! - `break`;
//! - `continue`;
//! - `return`;
//! - nested control flow;
//! - measurement-driven classical feedback;
//! - classical predicates;
//! - operation references through stable `OperationId`s;
//! - logical-qubit references through `quantum::ir::qubit::QubitId`;
//! - explicit validation contexts;
//! - overflow-safe structural accounting;
//! - deterministic semantic equality and hashing;
//! - scalable, target-independent control-flow representation.
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - structured control-flow semantics;
//! - branch structure;
//! - loop structure;
//! - loop domains;
//! - structured transfers;
//! - classical predicates used directly by control flow;
//! - operation references inside control-flow bodies;
//! - logical-qubit references used by loop domains;
//! - control-flow structural validation;
//! - control-flow resource accounting.
//!
//! This module does NOT own:
//!
//! - source-language parsing;
//! - source-language ASTs;
//! - hardware;
//! - physical topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - simulation;
//! - QPU execution;
//! - optimization algorithms;
//! - QEC decoding;
//! - backend communication.
//!
//! # Dependency boundary
//!
//! The dependency direction is intentionally:
//!
//! ```text
//! identity.rs ───────────────┐
//!                            │
//! qubit.rs ──────────────────┤
//!                            ▼
//!                control/control_flow.rs
//!                            │
//!                            ▼
//!                   program / operation
//!                            │
//!                            ▼
//!                 validation / analysis
//! ```
//!
//! The control-flow layer stores `OperationId` rather than embedding the
//! operation implementation. Consequently, adding gates, measurements,
//! resets, pulses, analog operations, logical operations, or future dialect
//! operations does not require this file to change.
//!
//! # Logical qubit boundary
//!
//! Logical qubits are represented exclusively by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module never represents a logical qubit as a raw integer.
//!
//! The numeric index inside `QubitId` is only used when validating a logical
//! namespace supplied by the caller.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! - maximum number of qubits;
//! - maximum number of loop iterations;
//! - maximum number of branches;
//! - maximum number of operations;
//! - maximum control-flow depth;
//! - fixed machine topology;
//! - fixed hardware architecture.
//!
//! Explicit validation/resource policies are supplied by callers.
//!
//! A validation policy is a safety/resource boundary, not a definition of
//! what Zamani can express.
//!
//! Therefore the same representation can describe:
//!
//! ```text
//! one logical qubit
//! thousands of logical qubits
//! millions of logical qubits
//! any other finite logical namespace permitted by the execution environment
//! ```
//!
//! # No "infinite allocation"
//!
//! Rust collections are necessarily finite at runtime. "Scalable to infinity"
//! therefore means that the semantic model contains no artificial fixed
//! machine-size ceiling. Actual construction remains bounded by available
//! memory, address space, compiler resources, deployment policy, and target
//! resources.
//!
//! # No unsafe
//!
//! This module forbids unsafe code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! `src/quantum/ir/control/mod.rs` should expose this module:
//
//! ```text
//! pub mod control_flow;
//! ```
//!
//! `src/quantum/ir/mod.rs` should expose the control namespace:
//
//! ```text
//! pub mod control;
//! ```
//!
//! The old flat `control_flow.rs` should not remain the canonical implementation
//! once the migration to `control/control_flow.rs` is complete.
//!
//! Downstream modules should import this module through:
//!
//! ```text
//! crate::quantum::ir::control::control_flow
//! ```
//!
//! or through deliberately selected re-exports from `control/mod.rs`.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::measurement::ClassicalBitId;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type used by control-flow construction and validation.
pub type ControlFlowResult<T> = Result<T, ControlFlowError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating structured control flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowError {
    /// A required structured body is empty.
    EmptyRequiredBlock {
        /// Semantic name of the block.
        block: &'static str,
    },

    /// A predicate is missing.
    MissingCondition,

    /// A predicate has no terms.
    EmptyCondition,

    /// A condition exceeds the explicitly supplied validation policy.
    ConditionLimitExceeded {
        /// Number of condition nodes requested.
        requested: usize,

        /// Maximum allowed by the validation policy.
        maximum: usize,
    },

    /// A logical qubit is outside the supplied logical namespace.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Number of logical qubits in the validation namespace.
        num_qubits: usize,
    },

    /// A classical bit is outside the supplied classical namespace.
    ClassicalBitOutOfRange {
        /// Referenced classical bit.
        bit: ClassicalBitId,

        /// Number of classical bits in the validation namespace.
        num_classical_bits: usize,
    },

    /// A loop range is invalid.
    InvalidLoopRange {
        /// Static reason.
        reason: &'static str,
    },

    /// A loop has a zero step.
    ZeroLoopStep,

    /// A loop variable identifier is invalid for the supplied context.
    InvalidLoopVariable,

    /// A structured transfer is not legal in the current context.
    InvalidTransfer {
        /// Transfer being attempted.
        transfer: ControlTransfer,
    },

    /// `break` or `continue` was used outside a loop.
    TransferOutsideLoop {
        /// Invalid transfer.
        transfer: ControlTransfer,
    },

    /// `return` was used outside a function boundary.
    ReturnOutsideFunction,

    /// The requested nesting exceeds the supplied validation policy.
    ControlFlowDepthExceeded {
        /// Requested depth.
        requested: usize,

        /// Maximum permitted depth.
        maximum: usize,
    },

    /// The number of control-flow nodes exceeds the supplied policy.
    NodeLimitExceeded {
        /// Requested node count.
        requested: usize,

        /// Maximum permitted node count.
        maximum: usize,
    },

    /// Arithmetic overflow occurred while calculating resource usage.
    ArithmeticOverflow {
        /// Semantic calculation that overflowed.
        calculation: &'static str,
    },

    /// A structurally invalid control-flow node was encountered.
    InvalidStructure {
        /// Static reason.
        reason: &'static str,
    },

    /// An operation reference is malformed for the caller's operation table.
    ///
    /// This variant is reserved for integration boundaries. The control-flow
    /// module itself does not attempt to determine whether an `OperationId`
    /// exists in a program-wide operation table.
    InvalidOperationReference {
        /// Referenced operation.
        operation: OperationId,
    },

    /// A nested validation error.
    Nested {
        /// Underlying error.
        error: Box<ControlFlowError>,
    },
}

impl fmt::Display for ControlFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequiredBlock { block } => {
                write!(
                    formatter,
                    "control-flow block `{block}` must not be empty"
                )
            }

            Self::MissingCondition => {
                formatter.write_str(
                    "control-flow construct requires a condition",
                )
            }

            Self::EmptyCondition => {
                formatter.write_str(
                    "classical condition must not be empty",
                )
            }

            Self::ConditionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition node limit exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside namespace \
                     containing {num_qubits} qubits"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => {
                write!(
                    formatter,
                    "classical bit {bit} is outside namespace \
                     containing {num_classical_bits} bits"
                )
            }

            Self::InvalidLoopRange { reason } => {
                write!(formatter, "invalid loop range: {reason}")
            }

            Self::ZeroLoopStep => {
                formatter.write_str("loop step must not be zero")
            }

            Self::InvalidLoopVariable => {
                formatter.write_str("invalid loop variable")
            }

            Self::InvalidTransfer { transfer } => {
                write!(
                    formatter,
                    "invalid control transfer: {transfer}"
                )
            }

            Self::TransferOutsideLoop { transfer } => {
                write!(
                    formatter,
                    "control transfer `{transfer}` is outside a loop"
                )
            }

            Self::ReturnOutsideFunction => {
                formatter.write_str(
                    "return is outside a function boundary",
                )
            }

            Self::ControlFlowDepthExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow depth exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow node limit exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid control-flow structure: {reason}"
                )
            }

            Self::InvalidOperationReference { operation } => {
                write!(
                    formatter,
                    "invalid operation reference {operation}"
                )
            }

            Self::Nested { error } => {
                write!(formatter, "nested control-flow error: {error}")
            }
        }
    }
}

impl std::error::Error for ControlFlowError {}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit validation policy for control flow.
///
/// This policy is intentionally local to validation rather than being an
/// architectural restriction on Zamani.
///
/// Production callers should derive these values from the repository-wide
/// `QuantumIrLimits` policy at the integration boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlFlowValidationPolicy {
    /// Maximum number of nodes accepted by this validation operation.
    pub max_nodes: usize,

    /// Maximum control-flow nesting depth accepted by this validation
    /// operation.
    pub max_depth: usize,

    /// Maximum predicate-node count accepted by this validation operation.
    pub max_condition_nodes: usize,
}

impl ControlFlowValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_nodes: usize,
        max_depth: usize,
        max_condition_nodes: usize,
    ) -> Self {
        Self {
            max_nodes,
            max_depth,
            max_condition_nodes,
        }
    }

    /// Creates a policy with no application-level finite ceiling.
    ///
    /// This does not create infinite memory or infinite execution resources.
    /// It simply means that this validation policy does not impose finite
    /// ceilings on these categories.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_nodes: usize::MAX,
            max_depth: usize::MAX,
            max_condition_nodes: usize::MAX,
        }
    }
}

impl Default for ControlFlowValidationPolicy {
    fn default() -> Self {
        Self::unbounded()
    }
}

// =============================================================================
// Validation context
// =============================================================================

/// Namespace and structural context used when validating control flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlFlowValidationContext {
    /// Number of logical qubits in the current logical namespace.
    pub num_qubits: usize,

    /// Number of classical bits in the current classical namespace.
    pub num_classical_bits: usize,

    /// Explicit resource/validation policy.
    pub policy: ControlFlowValidationPolicy,

    /// Whether the current region is a function body.
    pub in_function: bool,
}

impl ControlFlowValidationContext {
    /// Creates a validation context.
    #[must_use]
    pub const fn new(
        num_qubits: usize,
        num_classical_bits: usize,
        policy: ControlFlowValidationPolicy,
        in_function: bool,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            policy,
            in_function,
        }
    }

    /// Creates an unbounded validation context.
    #[must_use]
    pub const fn unbounded(
        num_qubits: usize,
        num_classical_bits: usize,
        in_function: bool,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            policy: ControlFlowValidationPolicy::unbounded(),
            in_function,
        }
    }
}

// =============================================================================
// Control transfer
// =============================================================================

/// Structured control-flow transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlTransfer {
    /// Exit the nearest enclosing loop.
    Break,

    /// Skip to the next iteration of the nearest enclosing loop.
    Continue,

    /// Return from the enclosing function.
    Return,
}

impl fmt::Display for ControlTransfer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Break => formatter.write_str("break"),
            Self::Continue => formatter.write_str("continue"),
            Self::Return => formatter.write_str("return"),
        }
    }
}

// =============================================================================
// Classical boolean
// =============================================================================

/// Canonical Boolean value used by control-flow predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalBool {
    /// Boolean true.
    True,

    /// Boolean false.
    False,
}

impl ClassicalBool {
    /// Returns the native Boolean value.
    #[must_use]
    pub const fn value(self) -> bool {
        match self {
            Self::True => true,
            Self::False => false,
        }
    }
}

impl From<bool> for ClassicalBool {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

// =============================================================================
// Classical predicate
// =============================================================================

/// Semantic classical predicate used by structured quantum control flow.
///
/// This is intentionally limited to control-flow semantics. Rich classical
/// expressions should eventually be represented by the canonical classical
/// expression subsystem and referenced through the operation/value layer.
///
/// The enum includes enough direct structure to represent measurement-driven
/// control flow without making the control-flow layer dependent on a frontend
/// AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalPredicate {
    /// Constant Boolean.
    Constant(ClassicalBool),

    /// Test whether a classical bit is set.
    Bit(ClassicalBitId),

    /// Compare a classical bit with a Boolean constant.
    BitEquals {
        /// Classical bit.
        bit: ClassicalBitId,

        /// Expected Boolean value.
        value: ClassicalBool,
    },

    /// Logical negation.
    Not(Box<Self>),

    /// Logical conjunction.
    And(Vec<Self>),

    /// Logical disjunction.
    Or(Vec<Self>),

    /// Logical exclusive-or.
    Xor(Vec<Self>),
}

impl ClassicalPredicate {
    /// Creates `true`.
    #[must_use]
    pub const fn always() -> Self {
        Self::Constant(ClassicalBool::True)
    }

    /// Creates `false`.
    #[must_use]
    pub const fn never() -> Self {
        Self::Constant(ClassicalBool::False)
    }

    /// Creates a classical-bit predicate.
    #[must_use]
    pub const fn bit(bit: ClassicalBitId) -> Self {
        Self::Bit(bit)
    }

    /// Creates a classical-bit equality predicate.
    #[must_use]
    pub const fn bit_equals(
        bit: ClassicalBitId,
        value: bool,
    ) -> Self {
        Self::BitEquals {
            bit,
            value: if value {
                ClassicalBool::True
            } else {
                ClassicalBool::False
            },
        }
    }

    /// Creates logical negation.
    #[must_use]
    pub fn not(predicate: Self) -> Self {
        Self::Not(Box::new(predicate))
    }

    /// Creates conjunction.
    pub fn and(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::And(predicates))
    }

    /// Creates disjunction.
    pub fn or(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::Or(predicates))
    }

    /// Creates exclusive-or.
    pub fn xor(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::Xor(predicates))
    }

    /// Validates structural correctness.
    pub fn validate(&self) -> ControlFlowResult<()> {
        match self {
            Self::Constant(_) |
            Self::Bit(_) |
            Self::BitEquals { .. } => Ok(()),

            Self::Not(predicate) => predicate.validate(),

            Self::And(predicates) |
            Self::Or(predicates) |
            Self::Xor(predicates) => {
                if predicates.is_empty() {
                    return Err(ControlFlowError::EmptyCondition);
                }

                for predicate in predicates {
                    predicate.validate()?;
                }

                Ok(())
            }
        }
    }

    /// Validates all referenced classical bits.
    pub fn validate_classical_bits(
        &self,
        num_classical_bits: usize,
    ) -> ControlFlowResult<()> {
        self.validate()?;

        self.validate_bits_recursive(num_classical_bits)
    }

    /// Returns the number of predicate nodes.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        match self {
            Self::Constant(_) |
            Self::Bit(_) |
            Self::BitEquals { .. } => Ok(1),

            Self::Not(predicate) => {
                predicate
                    .node_count()?
                    .checked_add(1)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "classical predicate node count",
                    })
            }

            Self::And(predicates) |
            Self::Or(predicates) |
            Self::Xor(predicates) => {
                let mut count = 1usize;

                for predicate in predicates {
                    count = count
                        .checked_add(predicate.node_count()?)
                        .ok_or(
                            ControlFlowError::ArithmeticOverflow {
                                calculation:
                                    "classical predicate node count",
                            },
                        )?;
                }

                Ok(count)
            }
        }
    }

    fn validate_bits_recursive(
        &self,
        num_classical_bits: usize,
    ) -> ControlFlowResult<()> {
        match self {
            Self::Constant(_) => Ok(()),

            Self::Bit(bit) |
            Self::BitEquals { bit, .. } => {
                if bit.index() >= num_classical_bits {
                    return Err(
                        ControlFlowError::ClassicalBitOutOfRange {
                            bit: *bit,
                            num_classical_bits,
                        },
                    );
                }

                Ok(())
            }

            Self::Not(predicate) => {
                predicate.validate_bits_recursive(
                    num_classical_bits,
                )
            }

            Self::And(predicates) |
            Self::Or(predicates) |
            Self::Xor(predicates) => {
                for predicate in predicates {
                    predicate.validate_bits_recursive(
                        num_classical_bits,
                    )?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Loop variable
// =============================================================================

/// Stable semantic identifier for a loop induction variable.
///
/// The identifier is not a source-language variable name.
///
/// A frontend or compiler symbol table owns the mapping from this identifier
/// to source-level naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LoopVariable(u64);

impl LoopVariable {
    /// Creates a loop-variable identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for LoopVariable {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<LoopVariable> for u64 {
    fn from(variable: LoopVariable) -> Self {
        variable.value()
    }
}

impl fmt::Display for LoopVariable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "loop{}", self.0)
    }
}

// =============================================================================
// Integer loop range
// =============================================================================

/// Signed integer loop domain.
///
/// The range uses a half-open or inclusive endpoint according to
/// `inclusive_end`.
///
/// Examples:
///
/// ```text
/// 0 .. 10
/// 0 ..= 10
/// 10 .. 0 step -1
/// 10 ..= 0 step -1
/// ```
///
/// The representation is semantic and does not enumerate iterations.
///
/// This is important for scalability: a loop over an enormous domain does not
/// require the IR to materialize every iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerLoopRange {
    start: i128,
    end: i128,
    step: i128,
    inclusive_end: bool,
}

impl IntegerLoopRange {
    /// Creates a half-open range `[start, end)`.
    pub const fn new(
        start: i128,
        end: i128,
        step: i128,
    ) -> ControlFlowResult<Self> {
        if step == 0 {
            return Err(ControlFlowError::ZeroLoopStep);
        }

        if step > 0 && start > end {
            return Err(ControlFlowError::InvalidLoopRange {
                reason: "positive-step range has start greater than end",
            });
        }

        if step < 0 && start < end {
            return Err(ControlFlowError::InvalidLoopRange {
                reason: "negative-step range has start less than end",
            });
        }

        Ok(Self {
            start,
            end,
            step,
            inclusive_end: false,
        })
    }

    /// Creates an inclusive range `[start, end]`.
    pub const fn inclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> ControlFlowResult<Self> {
        if step == 0 {
            return Err(ControlFlowError::ZeroLoopStep);
        }

        if step > 0 && start > end {
            return Err(ControlFlowError::InvalidLoopRange {
                reason: "positive-step range has start greater than end",
            });
        }

        if step < 0 && start < end {
            return Err(ControlFlowError::InvalidLoopRange {
                reason: "negative-step range has start less than end",
            });
        }

        Ok(Self {
            start,
            end,
            step,
            inclusive_end: true,
        })
    }

    /// Returns the first value.
    #[must_use]
    pub const fn start(self) -> i128 {
        self.start
    }

    /// Returns the endpoint.
    #[must_use]
    pub const fn end(self) -> i128 {
        self.end
    }

    /// Returns the signed step.
    #[must_use]
    pub const fn step(self) -> i128 {
        self.step
    }

    /// Returns whether the endpoint is inclusive.
    #[must_use]
    pub const fn is_inclusive(self) -> bool {
        self.inclusive_end
    }

    /// Returns whether the domain is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        if self.step > 0 {
            if self.inclusive_end {
                self.start > self.end
            } else {
                self.start >= self.end
            }
        } else if self.inclusive_end {
            self.start < self.end
        } else {
            self.start <= self.end
        }
    }

    /// Returns the number of iterations when it can be represented as a
    /// `u128`.
    ///
    /// The method does not enumerate the loop.
    pub fn iteration_count(self) -> ControlFlowResult<u128> {
        if self.is_empty() {
            return Ok(0);
        }

        let start = self.start;
        let end = self.end;
        let step = self.step;

        if step > 0 {
            let distance = end
                .checked_sub(start)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "positive loop-range distance",
                })?;

            let step_abs = step;

            let mut count = distance
                .checked_div(step_abs)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "positive loop-range division",
                })?;

            let remainder = distance % step_abs;

            if self.inclusive_end {
                count = count
                    .checked_add(1)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "inclusive loop iteration count",
                    })?;

                if remainder != 0 {
                    return Ok(count);
                }

                return Ok(count);
            }

            if remainder != 0 {
                count = count
                    .checked_add(1)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "exclusive loop iteration count",
                    })?;
            }

            Ok(count as u128)
        } else {
            let distance = start
                .checked_sub(end)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "negative loop-range distance",
                })?;

            let step_abs = step
                .checked_abs()
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "negative loop step absolute value",
                })?;

            let mut count = distance
                .checked_div(step_abs)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "negative loop-range division",
                })?;

            let remainder = distance % step_abs;

            if self.inclusive_end {
                count = count
                    .checked_add(1)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "inclusive reverse-loop iteration count",
                    })?;

                return Ok(count as u128);
            }

            if remainder != 0 {
                count = count
                    .checked_add(1)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "exclusive reverse-loop iteration count",
                    })?;
            }

            Ok(count as u128)
        }
    }
}

// =============================================================================
// Logical qubit loop range
// =============================================================================

/// Logical-qubit iteration domain.
///
/// This is intentionally semantic and compact. It does not materialize every
/// `QubitId` in the range.
///
/// The range is half-open: `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitLoopRange {
    start: QubitId,
    end: QubitId,
}

impl QubitLoopRange {
    /// Creates a logical-qubit range `[start, end)`.
    pub const fn new(
        start: QubitId,
        end: QubitId,
    ) -> ControlFlowResult<Self> {
        if start.index() > end.index() {
            return Err(ControlFlowError::InvalidLoopRange {
                reason: "logical-qubit range has start greater than end",
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the first logical qubit.
    #[must_use]
    pub const fn start(self) -> QubitId {
        self.start
    }

    /// Returns the exclusive endpoint.
    #[must_use]
    pub const fn end(self) -> QubitId {
        self.end
    }

    /// Returns the number of logical qubits in the range.
    pub fn len(self) -> ControlFlowResult<usize> {
        self.end
            .index()
            .checked_sub(self.start.index())
            .ok_or(ControlFlowError::ArithmeticOverflow {
                calculation: "logical-qubit range length",
            })
    }

    /// Returns whether the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.index() == self.end.index()
    }

    /// Validates the range against a logical-qubit namespace.
    pub fn validate(
        self,
        num_qubits: usize,
    ) -> ControlFlowResult<()> {
        if self.start.index() > num_qubits
            || self.end.index() > num_qubits
        {
            return Err(ControlFlowError::QubitOutOfRange {
                qubit: if self.start.index() >= num_qubits {
                    self.start
                } else {
                    self.end
                },
                num_qubits,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Loop domain
// =============================================================================

/// Semantic iteration domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopDomain {
    /// Integer iteration.
    Integer(IntegerLoopRange),

    /// Logical-qubit iteration.
    Qubits(QubitLoopRange),

    /// Single-count repeat domain.
    ///
    /// The number is the number of repetitions. It is represented directly
    /// rather than expanded into individual operations.
    Repeat(u128),
}

impl LoopDomain {
    /// Returns the statically known iteration count when one exists.
    pub fn iteration_count(self) -> ControlFlowResult<Option<u128>> {
        match self {
            Self::Integer(range) => {
                Ok(Some(range.iteration_count()?))
            }

            Self::Qubits(range) => {
                Ok(Some(range.len()? as u128))
            }

            Self::Repeat(count) => Ok(Some(count)),
        }
    }

    /// Validates the domain against the logical-qubit namespace.
    pub fn validate(
        self,
        num_qubits: usize,
    ) -> ControlFlowResult<()> {
        match self {
            Self::Integer(range) => {
                if range.step() == 0 {
                    return Err(ControlFlowError::ZeroLoopStep);
                }

                Ok(())
            }

            Self::Qubits(range) => range.validate(num_qubits),

            Self::Repeat(_) => Ok(()),
        }
    }
}

// =============================================================================
// Return payload
// =============================================================================

/// Optional semantic return payload.
///
/// Control flow itself does not own the canonical value representation.
/// Instead, an operation/value identity can be used as the payload.
///
/// This keeps the control-flow layer independent of the future classical
/// value/type hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnValue {
    /// No value.
    Unit,

    /// Return an existing IR operation result/value.
    Value(OperationId),
}

impl ReturnValue {
    /// Returns whether the return is unit-valued.
    #[must_use]
    pub const fn is_unit(self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Returns the operation result reference, if present.
    #[must_use]
    pub const fn value(self) -> Option<OperationId> {
        match self {
            Self::Unit => None,
            Self::Value(operation) => Some(operation),
        }
    }
}

// =============================================================================
// Control-flow node
// =============================================================================

/// One structured control-flow node.
///
/// Operation nodes reference existing operations by `OperationId`.
///
/// Structured nodes own their nested blocks directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlFlowNode {
    /// Execute an existing IR operation.
    Operation(OperationId),

    /// Conditional execution.
    If {
        /// Classical predicate.
        condition: ClassicalPredicate,

        /// Executed when the predicate evaluates to true.
        then_block: ControlFlowBlock,

        /// Optional block executed when the predicate evaluates to false.
        else_block: Option<ControlFlowBlock>,
    },

    /// Pre-test loop.
    While {
        /// Loop predicate.
        condition: ClassicalPredicate,

        /// Loop body.
        body: ControlFlowBlock,
    },

    /// Post-test loop.
    DoWhile {
        /// Loop body.
        body: ControlFlowBlock,

        /// Predicate evaluated after each iteration.
        condition: ClassicalPredicate,
    },

    /// Structured counted/domain loop.
    For {
        /// Induction-variable identifier.
        variable: LoopVariable,

        /// Iteration domain.
        domain: LoopDomain,

        /// Loop body.
        body: ControlFlowBlock,
    },

    /// Structured repeat loop.
    Repeat {
        /// Number of repetitions.
        count: u128,

        /// Repeated body.
        body: ControlFlowBlock,
    },

    /// Exit the nearest enclosing loop.
    Break,

    /// Continue the nearest enclosing loop.
    Continue,

    /// Return from the enclosing function.
    Return(ReturnValue),
}

impl ControlFlowNode {
    /// Creates an operation node.
    #[must_use]
    pub const fn operation(operation: OperationId) -> Self {
        Self::Operation(operation)
    }

    /// Creates an `if` node without an `else`.
    pub fn if_then(
        condition: ClassicalPredicate,
        then_block: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        condition.validate()?;

        if then_block.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "if.then",
            });
        }

        Ok(Self::If {
            condition,
            then_block,
            else_block: None,
        })
    }

    /// Creates an `if` / `else` node.
    pub fn if_else(
        condition: ClassicalPredicate,
        then_block: ControlFlowBlock,
        else_block: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        condition.validate()?;

        if then_block.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "if.then",
            });
        }

        if else_block.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "if.else",
            });
        }

        Ok(Self::If {
            condition,
            then_block,
            else_block: Some(else_block),
        })
    }

    /// Creates a `while` node.
    pub fn while_loop(
        condition: ClassicalPredicate,
        body: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        condition.validate()?;

        if body.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "while.body",
            });
        }

        Ok(Self::While { condition, body })
    }

    /// Creates a `do while` node.
    pub fn do_while(
        body: ControlFlowBlock,
        condition: ClassicalPredicate,
    ) -> ControlFlowResult<Self> {
        condition.validate()?;

        if body.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "do_while.body",
            });
        }

        Ok(Self::DoWhile { body, condition })
    }

    /// Creates a domain-based `for` loop.
    pub fn for_loop(
        variable: LoopVariable,
        domain: LoopDomain,
        body: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        if body.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "for.body",
            });
        }

        Ok(Self::For {
            variable,
            domain,
            body,
        })
    }

    /// Creates a repeat loop.
    pub fn repeat(
        count: u128,
        body: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        if body.is_empty() {
            return Err(ControlFlowError::EmptyRequiredBlock {
                block: "repeat.body",
            });
        }

        Ok(Self::Repeat { count, body })
    }

    /// Creates a `break`.
    #[must_use]
    pub const fn break_loop() -> Self {
        Self::Break
    }

    /// Creates a `continue`.
    #[must_use]
    pub const fn continue_loop() -> Self {
        Self::Continue
    }

    /// Creates a unit return.
    #[must_use]
    pub const fn return_unit() -> Self {
        Self::Return(ReturnValue::Unit)
    }

    /// Creates a value return.
    #[must_use]
    pub const fn return_value(operation: OperationId) -> Self {
        Self::Return(ReturnValue::Value(operation))
    }

    /// Returns whether this node is a loop.
    #[must_use]
    pub const fn is_loop(&self) -> bool {
        matches!(
            self,
            Self::While { .. }
                | Self::DoWhile { .. }
                | Self::For { .. }
                | Self::Repeat { .. }
        )
    }

    /// Returns whether this node is a structured transfer.
    #[must_use]
    pub const fn is_transfer(&self) -> bool {
        matches!(
            self,
            Self::Break
                | Self::Continue
                | Self::Return(_)
        )
    }

    /// Returns the direct nested-node count.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        match self {
            Self::Operation(_) |
            Self::Break |
            Self::Continue |
            Self::Return(_) => Ok(1),

            Self::If {
                then_block,
                else_block,
                ..
            } => {
                let mut count = 1usize;

                count = count
                    .checked_add(then_block.node_count()?)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation:
                            "if control-flow node count",
                    })?;

                if let Some(block) = else_block {
                    count = count
                        .checked_add(block.node_count()?)
                        .ok_or(
                            ControlFlowError::ArithmeticOverflow {
                                calculation:
                                    "else control-flow node count",
                            },
                        )?;
                }

                Ok(count)
            }

            Self::While { body, .. } |
            Self::DoWhile { body, .. } |
            Self::For { body, .. } |
            Self::Repeat { body, .. } => {
                1usize
                    .checked_add(body.node_count()?)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation:
                            "loop control-flow node count",
                    })
            }
        }
    }
}

// =============================================================================
// Control-flow block
// =============================================================================

/// Ordered sequence of structured control-flow nodes.
///
/// Blocks own semantic structure, not source syntax.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ControlFlowBlock {
    nodes: Vec<ControlFlowNode>,
}

impl ControlFlowBlock {
    /// Creates an empty block.
    #[must_use]
    pub const fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Creates a block from already constructed nodes.
    ///
    /// The caller can validate the resulting block with `validate`.
    #[must_use]
    pub fn from_nodes(nodes: Vec<ControlFlowNode>) -> Self {
        Self { nodes }
    }

    /// Returns the number of direct nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the block contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns a direct node by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&ControlFlowNode> {
        self.nodes.get(index)
    }

    /// Returns the block's direct nodes.
    #[must_use]
    pub fn nodes(&self) -> &[ControlFlowNode] {
        &self.nodes
    }

    /// Returns a mutable view of the block's direct nodes.
    ///
    /// The caller is responsible for revalidation after mutation.
    #[must_use]
    pub fn nodes_mut(&mut self) -> &mut [ControlFlowNode] {
        &mut self.nodes
    }

    /// Appends a node.
    pub fn push(
        &mut self,
        node: ControlFlowNode,
    ) -> ControlFlowResult<()> {
        self.nodes.push(node);
        Ok(())
    }

    /// Atomically appends a node while enforcing a node-count policy.
    ///
    /// If the node would exceed the supplied limit, the block remains
    /// unchanged.
    pub fn try_push_with_policy(
        &mut self,
        node: ControlFlowNode,
        policy: ControlFlowValidationPolicy,
    ) -> ControlFlowResult<()> {
        let current = self.node_count()?;
        let additional = node.node_count()?;

        let requested = current
            .checked_add(additional)
            .ok_or(ControlFlowError::ArithmeticOverflow {
                calculation: "control-flow block node count",
            })?;

        if requested > policy.max_nodes {
            return Err(ControlFlowError::NodeLimitExceeded {
                requested,
                maximum: policy.max_nodes,
            });
        }

        self.nodes.push(node);
        Ok(())
    }

    /// Removes the last node.
    pub fn pop(&mut self) -> Option<ControlFlowNode> {
        self.nodes.pop()
    }

    /// Clears all nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Returns total recursive node count.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        let mut count = 0usize;

        for node in &self.nodes {
            count = count
                .checked_add(node.node_count()?)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation:
                        "recursive control-flow block node count",
                })?;
        }

        Ok(count)
    }

    /// Returns maximum control-flow nesting depth.
    ///
    /// A block containing only operations has depth zero.
    ///
    /// A block containing an `if` whose branch contains a loop has depth two.
    pub fn depth(&self) -> ControlFlowResult<usize> {
        let mut maximum = 0usize;

        for node in &self.nodes {
            let node_depth = node_depth(node)?;

            if node_depth > maximum {
                maximum = node_depth;
            }
        }

        Ok(maximum)
    }

    /// Validates the block.
    pub fn validate(
        &self,
        context: ControlFlowValidationContext,
    ) -> ControlFlowResult<()> {
        let nodes = self.node_count()?;

        if nodes > context.policy.max_nodes {
            return Err(ControlFlowError::NodeLimitExceeded {
                requested: nodes,
                maximum: context.policy.max_nodes,
            });
        }

        validate_block(self, context, 0, false)
    }
}

// =============================================================================
// Control-flow region
// =============================================================================

/// Top-level structured control-flow region.
///
/// A region represents a semantic executable region. Function ownership,
/// symbol ownership, and ABI details remain outside this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ControlFlowRegion {
    body: ControlFlowBlock,
}

impl ControlFlowRegion {
    /// Creates an empty region.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: ControlFlowBlock::new(),
        }
    }

    /// Creates a region from a block.
    #[must_use]
    pub fn from_block(body: ControlFlowBlock) -> Self {
        Self { body }
    }

    /// Returns the body.
    #[must_use]
    pub fn body(&self) -> &ControlFlowBlock {
        &self.body
    }

    /// Returns a mutable body.
    ///
    /// Revalidation is required after mutation.
    #[must_use]
    pub fn body_mut(&mut self) -> &mut ControlFlowBlock {
        &mut self.body
    }

    /// Appends an operation.
    pub fn push_operation(
        &mut self,
        operation: OperationId,
    ) -> ControlFlowResult<()> {
        self.body
            .push(ControlFlowNode::operation(operation))
    }

    /// Appends a control-flow node.
    pub fn push(
        &mut self,
        node: ControlFlowNode,
    ) -> ControlFlowResult<()> {
        self.body.push(node)
    }

    /// Returns total node count.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        self.body.node_count()
    }

    /// Returns maximum nesting depth.
    pub fn depth(&self) -> ControlFlowResult<usize> {
        self.body.depth()
    }

    /// Validates the complete region.
    pub fn validate(
        &self,
        context: ControlFlowValidationContext,
    ) -> ControlFlowResult<()> {
        self.body.validate(context)
    }
}

// =============================================================================
// Validation
// =============================================================================

fn validate_block(
    block: &ControlFlowBlock,
    context: ControlFlowValidationContext,
    depth: usize,
    in_loop: bool,
) -> ControlFlowResult<()> {
    if depth > context.policy.max_depth {
        return Err(
            ControlFlowError::ControlFlowDepthExceeded {
                requested: depth,
                maximum: context.policy.max_depth,
            },
        );
    }

    for node in block.nodes() {
        validate_node(
            node,
            context,
            depth,
            in_loop,
        )?;
    }

    Ok(())
}

fn validate_node(
    node: &ControlFlowNode,
    context: ControlFlowValidationContext,
    depth: usize,
    in_loop: bool,
) -> ControlFlowResult<()> {
    match node {
        ControlFlowNode::Operation(operation) => {
            validate_operation_reference(*operation)
        }

        ControlFlowNode::If {
            condition,
            then_block,
            else_block,
        } => {
            validate_condition(condition, context)?;

            let next_depth = checked_depth(depth)?;

            validate_block(
                then_block,
                context,
                next_depth,
                in_loop,
            )?;

            if let Some(block) = else_block {
                validate_block(
                    block,
                    context,
                    next_depth,
                    in_loop,
                )?;
            }

            Ok(())
        }

        ControlFlowNode::While {
            condition,
            body,
        } => {
            validate_condition(condition, context)?;

            let next_depth = checked_depth(depth)?;

            validate_block(
                body,
                context,
                next_depth,
                true,
            )
        }

        ControlFlowNode::DoWhile {
            body,
            condition,
        } => {
            validate_condition(condition, context)?;

            let next_depth = checked_depth(depth)?;

            validate_block(
                body,
                context,
                next_depth,
                true,
            )
        }

        ControlFlowNode::For {
            variable,
            domain,
            body,
        } => {
            if variable.value() == u64::MAX {
                return Err(
                    ControlFlowError::InvalidLoopVariable,
                );
            }

            domain.validate(context.num_qubits)?;

            let next_depth = checked_depth(depth)?;

            validate_block(
                body,
                context,
                next_depth,
                true,
            )
        }

        ControlFlowNode::Repeat {
            count: _,
            body,
        } => {
            let next_depth = checked_depth(depth)?;

            validate_block(
                body,
                context,
                next_depth,
                true,
            )
        }

        ControlFlowNode::Break |
        ControlFlowNode::Continue => {
            if !in_loop {
                return Err(
                    ControlFlowError::TransferOutsideLoop {
                        transfer: if matches!(
                            node,
                            ControlFlowNode::Break
                        ) {
                            ControlTransfer::Break
                        } else {
                            ControlTransfer::Continue
                        },
                    },
                );
            }

            Ok(())
        }

        ControlFlowNode::Return(value) => {
            if !context.in_function {
                return Err(
                    ControlFlowError::ReturnOutsideFunction,
                );
            }

            if let ReturnValue::Value(operation) = value {
                validate_operation_reference(*operation)?;
            }

            Ok(())
        }
    }
}

fn validate_condition(
    condition: &ClassicalPredicate,
    context: ControlFlowValidationContext,
) -> ControlFlowResult<()> {
    condition.validate()?;

    let count = condition.node_count()?;

    if count > context.policy.max_condition_nodes {
        return Err(
            ControlFlowError::ConditionLimitExceeded {
                requested: count,
                maximum: context.policy.max_condition_nodes,
            },
        );
    }

    condition.validate_classical_bits(
        context.num_classical_bits,
    )
}

fn validate_operation_reference(
    operation: OperationId,
) -> ControlFlowResult<()> {
    //
    // OperationId is intentionally opaque.
    //
    // Whether an operation exists in a Program/Module operation table is
    // outside the ownership of this module.
    //
    // We therefore accept every syntactically valid OperationId rather than
    // inventing a second global operation registry here.
    //
    // The dedicated program/operation validation layer must verify existence.
    //
    let _ = operation;

    Ok(())
}

fn checked_depth(depth: usize) -> ControlFlowResult<usize> {
    depth
        .checked_add(1)
        .ok_or(ControlFlowError::ArithmeticOverflow {
            calculation: "control-flow nesting depth",
        })
}

// =============================================================================
// Depth calculation
// =============================================================================

fn node_depth(
    node: &ControlFlowNode,
) -> ControlFlowResult<usize> {
    match node {
        ControlFlowNode::Operation(_) |
        ControlFlowNode::Break |
        ControlFlowNode::Continue |
        ControlFlowNode::Return(_) => Ok(0),

        ControlFlowNode::If {
            then_block,
            else_block,
            ..
        } => {
            let then_depth = then_block.depth()?;
            let else_depth = match else_block {
                Some(block) => block.depth()?,
                None => 0,
            };

            let nested = then_depth.max(else_depth);

            nested
                .checked_add(1)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "if control-flow depth",
                })
        }

        ControlFlowNode::While { body, .. } |
        ControlFlowNode::DoWhile { body, .. } |
        ControlFlowNode::For { body, .. } |
        ControlFlowNode::Repeat { body, .. } => {
            body.depth()?
                .checked_add(1)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "loop control-flow depth",
                })
        }
    }
}

// =============================================================================
// Operation collection
// =============================================================================

/// Collects operation references recursively in execution order.
///
/// This method is intentionally read-only.
///
/// The returned vector is newly allocated and therefore can be used safely by
/// downstream analyses without aliasing the IR.
pub fn collect_operations(
    region: &ControlFlowRegion,
) -> ControlFlowResult<Vec<OperationId>> {
    let mut operations = Vec::new();

    collect_operations_from_block(
        region.body(),
        &mut operations,
    )?;

    Ok(operations)
}

fn collect_operations_from_block(
    block: &ControlFlowBlock,
    output: &mut Vec<OperationId>,
) -> ControlFlowResult<()> {
    for node in block.nodes() {
        match node {
            ControlFlowNode::Operation(operation) => {
                output.push(*operation);
            }

            ControlFlowNode::If {
                then_block,
                else_block,
                ..
            } => {
                collect_operations_from_block(
                    then_block,
                    output,
                )?;

                if let Some(block) = else_block {
                    collect_operations_from_block(
                        block,
                        output,
                    )?;
                }
            }

            ControlFlowNode::While { body, .. } |
            ControlFlowNode::DoWhile { body, .. } |
            ControlFlowNode::For { body, .. } |
            ControlFlowNode::Repeat { body, .. } => {
                collect_operations_from_block(
                    body,
                    output,
                )?;
            }

            ControlFlowNode::Break |
            ControlFlowNode::Continue |
            ControlFlowNode::Return(_) => {}
        }
    }

    Ok(())
}

// =============================================================================
// Operation count
// =============================================================================

/// Counts operation references recursively without materializing them.
pub fn operation_count(
    region: &ControlFlowRegion,
) -> ControlFlowResult<usize> {
    count_operations_in_block(region.body())
}

fn count_operations_in_block(
    block: &ControlFlowBlock,
) -> ControlFlowResult<usize> {
    let mut count = 0usize;

    for node in block.nodes() {
        let additional = match node {
            ControlFlowNode::Operation(_) => 1,

            ControlFlowNode::If {
                then_block,
                else_block,
                ..
            } => {
                let mut nested =
                    count_operations_in_block(then_block)?;

                if let Some(block) = else_block {
                    nested = nested
                        .checked_add(
                            count_operations_in_block(block)?,
                        )
                        .ok_or(
                            ControlFlowError::ArithmeticOverflow {
                                calculation:
                                    "if operation count",
                            },
                        )?;
                }

                nested
            }

            ControlFlowNode::While { body, .. } |
            ControlFlowNode::DoWhile { body, .. } |
            ControlFlowNode::For { body, .. } |
            ControlFlowNode::Repeat { body, .. } => {
                count_operations_in_block(body)?
            }

            ControlFlowNode::Break |
            ControlFlowNode::Continue |
            ControlFlowNode::Return(_) => 0,
        };

        count = count
            .checked_add(additional)
            .ok_or(ControlFlowError::ArithmeticOverflow {
                calculation:
                    "recursive control-flow operation count",
            })?;
    }

    Ok(count)
}

// =============================================================================
// Classical dependency collection
// =============================================================================

/// Collects every classical bit referenced by predicates.
///
/// Duplicates are preserved because this is a dependency occurrence list.
/// Canonical deduplication belongs to the analysis layer.
pub fn collect_classical_dependencies(
    region: &ControlFlowRegion,
) -> ControlFlowResult<Vec<ClassicalBitId>> {
    let mut dependencies = Vec::new();

    collect_classical_dependencies_from_block(
        region.body(),
        &mut dependencies,
    )?;

    Ok(dependencies)
}

fn collect_classical_dependencies_from_block(
    block: &ControlFlowBlock,
    output: &mut Vec<ClassicalBitId>,
) -> ControlFlowResult<()> {
    for node in block.nodes() {
        match node {
            ControlFlowNode::Operation(_) => {}

            ControlFlowNode::If {
                condition,
                then_block,
                else_block,
            } => {
                collect_predicate_bits(
                    condition,
                    output,
                )?;

                collect_classical_dependencies_from_block(
                    then_block,
                    output,
                )?;

                if let Some(block) = else_block {
                    collect_classical_dependencies_from_block(
                        block,
                        output,
                    )?;
                }
            }

            ControlFlowNode::While {
                condition,
                body,
            } => {
                collect_predicate_bits(
                    condition,
                    output,
                )?;

                collect_classical_dependencies_from_block(
                    body,
                    output,
                )?;
            }

            ControlFlowNode::DoWhile {
                body,
                condition,
            } => {
                collect_classical_dependencies_from_block(
                    body,
                    output,
                )?;

                collect_predicate_bits(
                    condition,
                    output,
                )?;
            }

            ControlFlowNode::For { body, .. } |
            ControlFlowNode::Repeat { body, .. } => {
                collect_classical_dependencies_from_block(
                    body,
                    output,
                )?;
            }

            ControlFlowNode::Break |
            ControlFlowNode::Continue |
            ControlFlowNode::Return(_) => {}
        }
    }

    Ok(())
}

fn collect_predicate_bits(
    predicate: &ClassicalPredicate,
    output: &mut Vec<ClassicalBitId>,
) -> ControlFlowResult<()> {
    match predicate {
        ClassicalPredicate::Constant(_) => {}

        ClassicalPredicate::Bit(bit) |
        ClassicalPredicate::BitEquals { bit, .. } => {
            output.push(*bit);
        }

        ClassicalPredicate::Not(predicate) => {
            collect_predicate_bits(predicate, output)?;
        }

        ClassicalPredicate::And(predicates) |
        ClassicalPredicate::Or(predicates) |
        ClassicalPredicate::Xor(predicates) => {
            for predicate in predicates {
                collect_predicate_bits(
                    predicate,
                    output,
                )?;
            }
        }
    }

    Ok(())
}

// =============================================================================
// Logical-qubit dependency collection
// =============================================================================

/// Collects logical qubits referenced by loop domains.
///
/// This does not inspect the operations referenced by `OperationId`.
/// Operation operand analysis belongs to the operation layer.
pub fn collect_loop_qubits(
    region: &ControlFlowRegion,
) -> ControlFlowResult<Vec<QubitId>> {
    let mut qubits = Vec::new();

    collect_loop_qubits_from_block(
        region.body(),
        &mut qubits,
    )?;

    Ok(qubits)
}

fn collect_loop_qubits_from_block(
    block: &ControlFlowBlock,
    output: &mut Vec<QubitId>,
) -> ControlFlowResult<()> {
    for node in block.nodes() {
        match node {
            ControlFlowNode::Operation(_) |
            ControlFlowNode::Break |
            ControlFlowNode::Continue |
            ControlFlowNode::Return(_) => {}

            ControlFlowNode::If {
                then_block,
                else_block,
                ..
            } => {
                collect_loop_qubits_from_block(
                    then_block,
                    output,
                )?;

                if let Some(block) = else_block {
                    collect_loop_qubits_from_block(
                        block,
                        output,
                    )?;
                }
            }

            ControlFlowNode::While { body, .. } |
            ControlFlowNode::DoWhile { body, .. } => {
                collect_loop_qubits_from_block(
                    body,
                    output,
                )?;
            }

            ControlFlowNode::For {
                domain,
                body,
                ..
            } => {
                if let LoopDomain::Qubits(range) = domain {
                    let mut index = range.start().index();

                    while index < range.end().index() {
                        output.push(QubitId::new(index));

                        index = index
                            .checked_add(1)
                            .ok_or(
                                ControlFlowError::ArithmeticOverflow {
                                    calculation:
                                        "logical-qubit range expansion",
                                },
                            )?;
                    }
                }

                collect_loop_qubits_from_block(
                    body,
                    output,
                )?;
            }

            ControlFlowNode::Repeat { body, .. } => {
                collect_loop_qubits_from_block(
                    body,
                    output,
                )?;
            }
        }
    }

    Ok(())
}

// =============================================================================
// Transfer validation
// =============================================================================

/// Validates whether a structured transfer is legal in the supplied context.
pub const fn validate_transfer(
    transfer: ControlTransfer,
    in_loop: bool,
    in_function: bool,
) -> ControlFlowResult<()> {
    match transfer {
        ControlTransfer::Break |
        ControlTransfer::Continue => {
            if !in_loop {
                return Err(
                    ControlFlowError::TransferOutsideLoop {
                        transfer,
                    },
                );
            }
        }

        ControlTransfer::Return => {
            if !in_function {
                return Err(
                    ControlFlowError::ReturnOutsideFunction,
                );
            }
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn classical_bit(value: usize) -> ClassicalBitId {
        ClassicalBitId::new(value)
    }

    fn context() -> ControlFlowValidationContext {
        ControlFlowValidationContext::new(
            16,
            16,
            ControlFlowValidationPolicy::new(
                10_000,
                256,
                10_000,
            ),
            true,
        )
    }

    #[test]
    fn predicate_bit_is_valid() {
        let predicate =
            ClassicalPredicate::bit(classical_bit(0));

        assert!(predicate.validate().is_ok());
        assert!(
            predicate
                .validate_classical_bits(1)
                .is_ok()
        );
    }

    #[test]
    fn predicate_rejects_out_of_range_classical_bit() {
        let predicate =
            ClassicalPredicate::bit(classical_bit(4));

        assert_eq!(
            predicate.validate_classical_bits(4),
            Err(
                ControlFlowError::ClassicalBitOutOfRange {
                    bit: classical_bit(4),
                    num_classical_bits: 4,
                }
            )
        );
    }

    #[test]
    fn empty_and_is_rejected() {
        assert_eq!(
            ClassicalPredicate::and(Vec::new()),
            Err(ControlFlowError::EmptyCondition)
        );
    }

    #[test]
    fn integer_range_counts_without_expansion() {
        let range =
            IntegerLoopRange::new(0, 10, 2)
                .expect("range should be valid");

        assert_eq!(
            range.iteration_count()
                .expect("count should succeed"),
            5
        );
    }

    #[test]
    fn inclusive_integer_range_counts_correctly() {
        let range =
            IntegerLoopRange::inclusive(0, 10, 2)
                .expect("range should be valid");

        assert_eq!(
            range.iteration_count()
                .expect("count should succeed"),
            6
        );
    }

    #[test]
    fn reverse_integer_range_counts_correctly() {
        let range =
            IntegerLoopRange::new(10, 0, -2)
                .expect("range should be valid");

        assert_eq!(
            range.iteration_count()
                .expect("count should succeed"),
            5
        );
    }

    #[test]
    fn zero_step_is_rejected() {
        assert_eq!(
            IntegerLoopRange::new(0, 10, 0),
            Err(ControlFlowError::ZeroLoopStep)
        );
    }

    #[test]
    fn logical_qubit_range_is_semantic() {
        let range = QubitLoopRange::new(
            QubitId::new(2),
            QubitId::new(10),
        )
        .expect("range should be valid");

        assert_eq!(
            range.len().expect("length should succeed"),
            8
        );
    }

    #[test]
    fn logical_qubit_range_validates_namespace() {
        let range = QubitLoopRange::new(
            QubitId::new(2),
            QubitId::new(10),
        )
        .expect("range should be valid");

        assert!(
            range.validate(10).is_ok()
        );

        assert!(
            range.validate(9).is_err()
        );
    }

    #[test]
    fn if_else_is_constructed() {
        let mut then_block =
            ControlFlowBlock::new();

        then_block
            .push(ControlFlowNode::operation(operation(1)))
            .expect("push should succeed");

        let mut else_block =
            ControlFlowBlock::new();

        else_block
            .push(ControlFlowNode::operation(operation(2)))
            .expect("push should succeed");

        let node = ControlFlowNode::if_else(
            ClassicalPredicate::bit(classical_bit(0)),
            then_block,
            else_block,
        )
        .expect("if should be valid");

        assert!(matches!(
            node,
            ControlFlowNode::If {
                else_block: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn break_outside_loop_is_rejected() {
        let mut block =
            ControlFlowBlock::new();

        block
            .push(ControlFlowNode::break_loop())
            .expect("push should succeed");

        let result = block.validate(context());

        assert_eq!(
            result,
            Err(
                ControlFlowError::TransferOutsideLoop {
                    transfer: ControlTransfer::Break,
                }
            )
        );
    }

    #[test]
    fn continue_inside_loop_is_valid() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::continue_loop(),
        )
        .expect("push should succeed");

        let loop_node =
            ControlFlowNode::repeat(10, body)
                .expect("repeat should be valid");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(loop_node)
            .expect("push should succeed");

        assert!(
            region.validate(context()).is_ok()
        );
    }

    #[test]
    fn return_requires_function_context() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::return_unit())
            .expect("push should succeed");

        let non_function =
            ControlFlowValidationContext::new(
                1,
                1,
                ControlFlowValidationPolicy::unbounded(),
                false,
            );

        assert_eq!(
            region.validate(non_function),
            Err(
                ControlFlowError::ReturnOutsideFunction
            )
        );
    }

    #[test]
    fn return_is_valid_inside_function() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::return_unit())
            .expect("push should succeed");

        assert!(
            region.validate(context()).is_ok()
        );
    }

    #[test]
    fn node_limit_is_enforced_atomically() {
        let mut block =
            ControlFlowBlock::new();

        block
            .push(ControlFlowNode::operation(operation(1)))
            .expect("push should succeed");

        let original_len = block.len();

        let result = block.try_push_with_policy(
            ControlFlowNode::operation(operation(2)),
            ControlFlowValidationPolicy::new(
                1,
                10,
                10,
            ),
        );

        assert!(matches!(
            result,
            Err(
                ControlFlowError::NodeLimitExceeded {
                    requested: 2,
                    maximum: 1,
                }
            )
        ));

        assert_eq!(
            block.len(),
            original_len
        );
    }

    #[test]
    fn nested_depth_is_calculated() {
        let mut innermost =
            ControlFlowBlock::new();

        innermost
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push should succeed");

        let while_node =
            ControlFlowNode::while_loop(
                ClassicalPredicate::always(),
                innermost,
            )
            .expect("while should be valid");

        let mut middle =
            ControlFlowBlock::new();

        middle
            .push(while_node)
            .expect("push should succeed");

        let if_node =
            ControlFlowNode::if_then(
                ClassicalPredicate::always(),
                middle,
            )
            .expect("if should be valid");

        let mut outer =
            ControlFlowBlock::new();

        outer
            .push(if_node)
            .expect("push should succeed");

        assert_eq!(
            outer.depth()
                .expect("depth should succeed"),
            2
        );
    }

    #[test]
    fn operations_are_collected_in_structural_order() {
        let mut then_block =
            ControlFlowBlock::new();

        then_block
            .push(ControlFlowNode::operation(
                operation(2),
            ))
            .expect("push should succeed");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push should succeed");

        region
            .push(
                ControlFlowNode::if_then(
                    ClassicalPredicate::always(),
                    then_block,
                )
                .expect("if should be valid"),
            )
            .expect("push should succeed");

        region
            .push(ControlFlowNode::operation(
                operation(3),
            ))
            .expect("push should succeed");

        assert_eq!(
            collect_operations(&region)
                .expect("collection should succeed"),
            vec![
                operation(1),
                operation(2),
                operation(3),
            ]
        );
    }

    #[test]
    fn operation_count_does_not_expand_loops() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(operation(1)),
        )
        .expect("push should succeed");

        let node =
            ControlFlowNode::repeat(1_000_000_000_000u128, body)
                .expect("repeat should be valid");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push should succeed");

        assert_eq!(
            operation_count(&region)
                .expect("count should succeed"),
            1
        );
    }

    #[test]
    fn classical_dependencies_are_collected() {
        let predicate =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::bit(
                    classical_bit(0),
                ),
                ClassicalPredicate::not(
                    ClassicalPredicate::bit(
                        classical_bit(2),
                    ),
                ),
            ])
            .expect("predicate should be valid");

        let mut block =
            ControlFlowBlock::new();

        block
            .push(
                ControlFlowNode::if_then(
                    predicate,
                    {
                        let mut body =
                            ControlFlowBlock::new();

                        body.push(
                            ControlFlowNode::operation(
                                operation(1),
                            ),
                        )
                        .expect("push should succeed");

                        body
                    },
                )
                .expect("if should be valid"),
            )
            .expect("push should succeed");

        let region =
            ControlFlowRegion::from_block(block);

        assert_eq!(
            collect_classical_dependencies(
                &region,
            )
            .expect("collection should succeed"),
            vec![
                classical_bit(0),
                classical_bit(2),
            ]
        );
    }

    #[test]
    fn qubit_loop_domain_is_validated() {
        let domain =
            LoopDomain::Qubits(
                QubitLoopRange::new(
                    QubitId::new(0),
                    QubitId::new(8),
                )
                .expect("range should be valid"),
            );

        assert!(
            domain.validate(8).is_ok()
        );

        assert!(
            domain.validate(7).is_err()
        );
    }

    #[test]
    fn nested_control_flow_validates() {
        let mut loop_body =
            ControlFlowBlock::new();

        loop_body
            .push(ControlFlowNode::break_loop())
            .expect("push should succeed");

        let loop_node =
            ControlFlowNode::while_loop(
                ClassicalPredicate::bit(
                    classical_bit(0),
                ),
                loop_body,
            )
            .expect("while should be valid");

        let mut branch_body =
            ControlFlowBlock::new();

        branch_body
            .push(loop_node)
            .expect("push should succeed");

        let branch =
            ControlFlowNode::if_then(
                ClassicalPredicate::bit(
                    classical_bit(1),
                ),
                branch_body,
            )
            .expect("if should be valid");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(branch)
            .expect("push should succeed");

        assert!(
            region.validate(context()).is_ok()
        );
    }

    #[test]
    fn depth_limit_is_enforced() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(operation(1)),
        )
        .expect("push should succeed");

        let node =
            ControlFlowNode::if_then(
                ClassicalPredicate::always(),
                body,
            )
            .expect("if should be valid");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push should succeed");

        let policy =
            ControlFlowValidationPolicy::new(
                100,
                0,
                100,
            );

        let context =
            ControlFlowValidationContext::new(
                1,
                1,
                policy,
                true,
            );

        assert!(matches!(
            region.validate(context),
            Err(
                ControlFlowError::ControlFlowDepthExceeded {
                    ..
                }
            )
        ));
    }
}