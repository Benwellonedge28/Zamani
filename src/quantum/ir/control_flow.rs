//! Zamani Quantum IR — Control-Flow Semantics
//!
//! Canonical, hardware-independent representation of structured quantum and
//! hybrid classical/quantum control flow.
//!
//! # Architectural role
//!
//! This module defines the semantic structure required for programs such as:
//!
//! ```text
//! measure(q0) -> c0
//!
//! if c0 == 1 {
//!     x(q1)
//! } else {
//!     h(q1)
//! }
//! ```
//!
//! and:
//!
//! ```text
//! while condition {
//!     quantum operations
//!     measurements
//!     classical decisions
//! }
//! ```
//!
//! It supports:
//!
//! - conditional execution;
//! - `if` / `else`;
//! - `while`;
//! - `do while`;
//! - counted/range `for` loops;
//! - iteration over logical qubit ranges;
//! - `repeat` loops;
//! - `break`;
//! - `continue`;
//! - `return`;
//! - nested control flow;
//! - classical predicates;
//! - deterministic structural validation;
//! - configurable control-flow resource limits;
//! - operation references through stable `OperationId`s;
//! - logical-qubit references through `quantum::ir::qubit::QubitId`;
//! - scalable logical namespaces;
//! - atomic mutation APIs;
//! - overflow-safe validation.
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - WHAT the control flow means;
//! - which classical values control execution;
//! - which IR operations belong to a branch or loop body;
//! - structured nesting;
//! - semantic loop domains;
//! - control-flow validation;
//! - control-flow depth accounting.
//!
//! This module does NOT own:
//!
//! - frontend parsing;
//! - source-language syntax;
//! - physical hardware;
//! - physical qubit topology;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - backend execution;
//! - simulator execution;
//! - measurement sampling;
//! - QPU communication;
//! - optimization algorithms;
//! - error-correction decoding.
//!
//! Those responsibilities belong to downstream compiler/backend subsystems.
//!
//! # Operation references
//!
//! `control_flow.rs` intentionally stores `OperationId` rather than defining
//! its own operation representation.
//!
//! This freezes the dependency boundary:
//!
//! ```text
//! control_flow.rs
//!       │
//!       └── OperationId
//!             │
//!             ▼
//!       operation.rs
//! ```
//!
//! The future `operation.rs` module can therefore represent gates,
//! measurements, resets, pulses, barriers, classical operations and other
//! operations without requiring this file to be rewritten.
//!
//! # Classical references
//!
//! The current repository exposes `ClassicalBitId` from `measurement.rs`.
//! This file intentionally consumes that canonical existing type rather than
//! defining a second incompatible classical-bit identity.
//!
//! When `classical.rs` is introduced, it should re-export the existing
//! `ClassicalBitId` rather than create another identity type.
//!
//! # Qubit references
//!
//! Logical qubits are always represented using:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module never uses `usize` as an untyped qubit reference.
//!
//! `usize` is used only for numeric loop bounds, collection lengths, and
//! resource accounting.
//!
//! # Scalability
//!
//! There is no architectural maximum number of:
//!
//! - logical qubits;
//! - control-flow nodes;
//! - operations;
//! - nested blocks;
//! - loop iterations;
//! - programs.
//!
//! Concrete resource policies are supplied explicitly through
//! `QuantumIrLimits`.
//!
//! Therefore the same semantic representation can describe:
//!
//! ```text
//! 1 qubit
//! 63 qubits
//! 64 qubits
//! 4096 qubits
//! 1,000,000 qubits
//! N finite qubits
//! ```
//!
//! subject only to explicit process, IR, compiler, deployment and hardware
//! resources.
//!
//! A resource limit is a safety policy, not a language architecture limit.
//!
//! # No unsafe code
//!
//! This file intentionally contains no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes that requirement compiler-enforced.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.

#![forbid(unsafe_code)]

use std::fmt;

use super::identity::OperationId;
use super::limits::QuantumIrLimits;
use super::measurement::ClassicalBitId;
use super::qubit::{QubitId, QubitRange};

// =============================================================================
// Result type
// =============================================================================

/// Result type for control-flow operations.
pub type ControlFlowResult<T> = Result<T, ControlFlowError>;

// =============================================================================
// Control-flow errors
// =============================================================================

/// Errors produced while constructing or validating control-flow IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowError {
    /// A required block is empty when a non-empty block is required.
    EmptyRequiredBlock {
        /// Semantic name of the block.
        block: &'static str,
    },

    /// A control-flow condition contains no predicate.
    MissingCondition,

    /// A condition contains no operands.
    EmptyCondition,

    /// A condition contains too many terms for the supplied policy.
    ConditionLimitExceeded {
        /// Number of condition terms requested.
        requested: usize,

        /// Maximum permitted terms.
        maximum: usize,
    },

    /// A classical-bit identifier is outside the supplied logical namespace.
    ClassicalBitOutOfRange {
        /// Invalid classical bit.
        bit: ClassicalBitId,

        /// Number of logical classical bits.
        num_classical_bits: usize,
    },

    /// A logical-qubit identifier is outside the supplied namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Number of logical qubits.
        num_qubits: usize,
    },

    /// A qubit range is outside the supplied logical namespace.
    QubitRangeOutOfRange {
        /// Start of the invalid range.
        start: usize,

        /// Exclusive end of the invalid range.
        end: usize,

        /// Number of logical qubits.
        num_qubits: usize,
    },

    /// A control-flow nesting depth exceeds the explicit IR policy.
    ControlFlowDepthExceeded {
        /// Required depth.
        requested: usize,

        /// Maximum permitted depth.
        maximum: usize,
    },

    /// A control-flow node count exceeds the explicit IR policy.
    NodeLimitExceeded {
        /// Requested number of nodes.
        requested: usize,

        /// Maximum permitted number of nodes.
        maximum: usize,
    },

    /// An arithmetic calculation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// A range uses a zero step.
    ZeroLoopStep,

    /// A loop has an invalid range configuration.
    InvalidLoopRange {
        /// Human-readable static reason.
        reason: &'static str,
    },

    /// A loop variable is invalid.
    InvalidLoopVariable,

    /// A control-flow construct cannot legally contain the requested
    /// structured transfer.
    InvalidTransfer {
        /// Transfer kind.
        transfer: ControlTransfer,
    },

    /// A `break` or `continue` was used outside a loop.
    TransferOutsideLoop {
        /// Transfer kind.
        transfer: ControlTransfer,
    },

    /// A return was used in a context where returns are not permitted.
    ReturnOutsideFunction,

    /// A block contains an operation reference that is numerically invalid.
    ///
    /// This is intentionally not an existence check because operation
    /// existence belongs to the enclosing program/operation table.
    InvalidOperationReference {
        /// Referenced operation identifier.
        operation: OperationId,
    },

    /// A control-flow node is structurally invalid.
    InvalidStructure {
        /// Static semantic reason.
        reason: &'static str,
    },

    /// A nested structure is invalid.
    Nested {
        /// The nested error.
        error: Box<ControlFlowError>,
    },
}

impl fmt::Display for ControlFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequiredBlock { block } => {
                write!(f, "control-flow block `{block}` must not be empty")
            }

            Self::MissingCondition => {
                f.write_str("control-flow construct requires a condition")
            }

            Self::EmptyCondition => {
                f.write_str("classical condition must not be empty")
            }

            Self::ConditionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "control-flow condition term limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => {
                write!(
                    f,
                    "classical bit {bit} is outside logical classical \
                     namespace of {num_classical_bits} bits"
                )
            }

            Self::QubitOutOfRange { qubit, num_qubits } => {
                write!(
                    f,
                    "logical qubit {qubit} is outside logical namespace \
                     of {num_qubits} qubits"
                )
            }

            Self::QubitRangeOutOfRange {
                start,
                end,
                num_qubits,
            } => {
                write!(
                    f,
                    "logical qubit range [{start}, {end}) is outside \
                     logical namespace of {num_qubits} qubits"
                )
            }

            Self::ControlFlowDepthExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "control-flow depth exceeded: requested {requested}, \
                     maximum {maximum}"
                )
            }

            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "control-flow node limit exceeded: requested {requested}, \
                     maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    f,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::ZeroLoopStep => {
                f.write_str("loop step must not be zero")
            }

            Self::InvalidLoopRange { reason } => {
                write!(f, "invalid loop range: {reason}")
            }

            Self::InvalidLoopVariable => {
                f.write_str("invalid loop variable")
            }

            Self::InvalidTransfer { transfer } => {
                write!(f, "invalid control transfer: {transfer}")
            }

            Self::TransferOutsideLoop { transfer } => {
                write!(
                    f,
                    "control transfer `{transfer}` is outside a loop"
                )
            }

            Self::ReturnOutsideFunction => {
                f.write_str("return is outside a function boundary")
            }

            Self::InvalidOperationReference { operation } => {
                write!(
                    f,
                    "invalid operation reference {operation}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(f, "invalid control-flow structure: {reason}")
            }

            Self::Nested { error } => {
                write!(f, "nested control-flow error: {error}")
            }
        }
    }
}

impl std::error::Error for ControlFlowError {}

// =============================================================================
// Control transfer
// =============================================================================

/// Structured control transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlTransfer {
    /// Exit the nearest enclosing loop.
    Break,

    /// Skip to the next iteration of the nearest enclosing loop.
    Continue,

    /// Return from the enclosing quantum/classical function.
    Return,
}

impl fmt::Display for ControlTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Break => f.write_str("break"),
            Self::Continue => f.write_str("continue"),
            Self::Return => f.write_str("return"),
        }
    }
}

// =============================================================================
// Classical condition
// =============================================================================

/// Boolean literal used by classical predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// A classical predicate used to control quantum execution.
///
/// Predicates are semantic expressions. They do not describe how a backend
/// evaluates the condition.
///
/// Examples:
///
/// ```text
/// c0
/// !c0
/// c0 && c1
/// c0 || c1
/// c0 == true
/// c0 != false
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalPredicate {
    /// Constant boolean value.
    Constant(ClassicalBool),

    /// Test whether a classical bit is set.
    Bit(ClassicalBitId),

    /// Test whether a classical bit equals a Boolean value.
    BitEquals {
        /// Classical bit.
        bit: ClassicalBitId,

        /// Expected value.
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
    /// Creates a constant true predicate.
    #[must_use]
    pub const fn always() -> Self {
        Self::Constant(ClassicalBool::True)
    }

    /// Creates a constant false predicate.
    #[must_use]
    pub const fn never() -> Self {
        Self::Constant(ClassicalBool::False)
    }

    /// Creates a predicate testing whether a bit is set.
    #[must_use]
    pub const fn bit(bit: ClassicalBitId) -> Self {
        Self::Bit(bit)
    }

    /// Creates a predicate testing whether a bit equals a value.
    #[must_use]
    pub const fn bit_equals(
        bit: ClassicalBitId,
        value: bool,
    ) -> Self {
        Self::BitEquals {
            bit,
            value: ClassicalBool::from_bool_const(value),
        }
    }

    /// Creates a negated predicate.
    #[must_use]
    pub fn not(predicate: Self) -> Self {
        Self::Not(Box::new(predicate))
    }

    /// Creates an AND expression.
    ///
    /// The expression must contain at least one term.
    pub fn and(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::And(predicates))
    }

    /// Creates an OR expression.
    ///
    /// The expression must contain at least one term.
    pub fn or(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::Or(predicates))
    }

    /// Creates an XOR expression.
    ///
    /// The expression must contain at least one term.
    pub fn xor(
        predicates: Vec<Self>,
    ) -> ControlFlowResult<Self> {
        if predicates.is_empty() {
            return Err(ControlFlowError::EmptyCondition);
        }

        Ok(Self::Xor(predicates))
    }

    /// Validates the predicate structurally.
    pub fn validate(&self) -> ControlFlowResult<()> {
        self.validate_with_depth(0)
    }

    /// Validates the predicate against a classical namespace.
    pub fn validate_classical_bits(
        &self,
        num_classical_bits: usize,
    ) -> ControlFlowResult<()> {
        self.validate()?;

        let mut bits = Vec::new();
        self.collect_classical_bits(&mut bits)?;

        for bit in bits {
            if bit.index() >= num_classical_bits {
                return Err(
                    ControlFlowError::ClassicalBitOutOfRange {
                        bit,
                        num_classical_bits,
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns the number of leaf predicate terms.
    ///
    /// This is useful for resource accounting and validation.
    pub fn term_count(&self) -> ControlFlowResult<usize> {
        match self {
            Self::Constant(_) | Self::Bit(_) | Self::BitEquals { .. } => Ok(1),

            Self::Not(predicate) => predicate
                .term_count()?
                .checked_add(1)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "classical predicate term count",
                }),

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                let mut count = 1usize;

                for predicate in predicates {
                    count = count
                        .checked_add(predicate.term_count()?)
                        .ok_or(
                            ControlFlowError::ArithmeticOverflow {
                                calculation:
                                    "classical predicate term count",
                            },
                        )?;
                }

                Ok(count)
            }
        }
    }

    fn validate_with_depth(
        &self,
        depth: usize,
    ) -> ControlFlowResult<()> {
        // The recursive representation is owned and therefore cannot contain
        // an object cycle. The checked depth calculation prevents arithmetic
        // overflow even for adversarially generated IR.
        let _next_depth = depth
            .checked_add(1)
            .ok_or(ControlFlowError::ArithmeticOverflow {
                calculation: "classical predicate depth",
            })?;

        match self {
            Self::Constant(_) |
            Self::Bit(_) |
            Self::BitEquals { .. } => Ok(()),

            Self::Not(predicate) => {
                predicate.validate_with_depth(_next_depth)
            }

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                if predicates.is_empty() {
                    return Err(ControlFlowError::EmptyCondition);
                }

                for predicate in predicates {
                    predicate.validate_with_depth(_next_depth)?;
                }

                Ok(())
            }
        }
    }

    fn collect_classical_bits(
        &self,
        output: &mut Vec<ClassicalBitId>,
    ) -> ControlFlowResult<()> {
        match self {
            Self::Constant(_) => {}

            Self::Bit(bit) => output.push(*bit),

            Self::BitEquals { bit, .. } => {
                output.push(*bit);
            }

            Self::Not(predicate) => {
                predicate.collect_classical_bits(output)?;
            }

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                for predicate in predicates {
                    predicate.collect_classical_bits(output)?;
                }
            }
        }

        Ok(())
    }
}

impl ClassicalBool {
    const fn from_bool_const(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

// =============================================================================
// Loop variable
// =============================================================================

/// Identifier for a structured loop variable.
///
/// This is a semantic compiler identifier, not a source-language variable
/// name. Frontends can map source variables to this identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LoopVariable(usize);

impl LoopVariable {
    /// Creates a loop variable identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the numeric loop-variable identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for LoopVariable {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl fmt::Display for LoopVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{}", self.0)
    }
}

// =============================================================================
// Integer loop range
// =============================================================================

/// Semantic integer range for a counted loop.
///
/// The range is not materialized. This is important for very large programs.
///
/// Examples:
///
/// ```text
/// 0 .. 10
/// 0 ..= 10
/// 10 .. 0 step -1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerLoopRange {
    start: i128,
    end: i128,
    step: i128,
    inclusive_end: bool,
}

impl IntegerLoopRange {
    /// Creates an exclusive-end loop range.
    pub const fn exclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> ControlFlowResult<Self> {
        Self::new(start, end, step, false)
    }

    /// Creates an inclusive-end loop range.
    pub const fn inclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> ControlFlowResult<Self> {
        Self::new(start, end, step, true)
    }

    /// Creates a loop range with explicit endpoint semantics.
    pub const fn new(
        start: i128,
        end: i128,
        step: i128,
        inclusive_end: bool,
    ) -> ControlFlowResult<Self> {
        if step == 0 {
            return Err(ControlFlowError::ZeroLoopStep);
        }

        if step > 0 {
            if inclusive_end {
                if start > end {
                    return Err(
                        ControlFlowError::InvalidLoopRange {
                            reason:
                                "positive inclusive range must not \
                                 start above its end",
                        },
                    );
                }
            } else if start > end {
                return Err(
                    ControlFlowError::InvalidLoopRange {
                        reason:
                            "positive exclusive range must not \
                             start above its end",
                    },
                );
            }
        } else if inclusive_end {
            if start < end {
                return Err(
                    ControlFlowError::InvalidLoopRange {
                        reason:
                            "negative inclusive range must not \
                             start below its end",
                    },
                );
            }
        } else if start < end {
            return Err(
                ControlFlowError::InvalidLoopRange {
                    reason:
                        "negative exclusive range must not \
                         start below its end",
                },
            );
        }

        Ok(Self {
            start,
            end,
            step,
            inclusive_end,
        })
    }

    /// Returns the starting value.
    #[must_use]
    pub const fn start(self) -> i128 {
        self.start
    }

    /// Returns the endpoint.
    #[must_use]
    pub const fn end(self) -> i128 {
        self.end
    }

    /// Returns the step.
    #[must_use]
    pub const fn step(self) -> i128 {
        self.step
    }

    /// Returns whether the endpoint is inclusive.
    #[must_use]
    pub const fn is_inclusive(self) -> bool {
        self.inclusive_end
    }

    /// Returns whether the range is ascending.
    #[must_use]
    pub const fn is_ascending(self) -> bool {
        self.step > 0
    }

    /// Returns whether the range is descending.
    #[must_use]
    pub const fn is_descending(self) -> bool {
        self.step < 0
    }

    /// Returns the number of iterations when it can be calculated safely.
    ///
    /// This method never materializes the range.
    pub fn iteration_count(self) -> ControlFlowResult<u128> {
        if self.step == 0 {
            return Err(ControlFlowError::ZeroLoopStep);
        }

        if self.step > 0 {
            if self.start > self.end
                || (!self.inclusive_end && self.start == self.end)
            {
                return Ok(0);
            }

            let distance = self
                .end
                .checked_sub(self.start)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "ascending loop-range distance",
                })?;

            let distance = u128::try_from(distance).map_err(|_| {
                ControlFlowError::ArithmeticOverflow {
                    calculation: "ascending loop-range distance conversion",
                }
            })?;

            let step = u128::try_from(self.step).map_err(|_| {
                ControlFlowError::ArithmeticOverflow {
                    calculation: "ascending loop-range step conversion",
                }
            })?;

            if self.inclusive_end {
                distance
                    .checked_div(step)
                    .and_then(|v| v.checked_add(1))
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "ascending inclusive loop iteration count",
                    })
            } else {
                Ok(distance.div_ceil(step))
            }
        } else {
            if self.start < self.end
                || (!self.inclusive_end && self.start == self.end)
            {
                return Ok(0);
            }

            let distance = self
                .start
                .checked_sub(self.end)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "descending loop-range distance",
                })?;

            let distance = u128::try_from(distance).map_err(|_| {
                ControlFlowError::ArithmeticOverflow {
                    calculation:
                        "descending loop-range distance conversion",
                }
            })?;

            let step_abs = self.step.checked_abs().ok_or(
                ControlFlowError::ArithmeticOverflow {
                    calculation: "descending loop-range step absolute value",
                },
            )?;

            let step = u128::try_from(step_abs).map_err(|_| {
                ControlFlowError::ArithmeticOverflow {
                    calculation:
                        "descending loop-range step conversion",
                }
            })?;

            if self.inclusive_end {
                distance
                    .checked_div(step)
                    .and_then(|v| v.checked_add(1))
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation:
                            "descending inclusive loop iteration count",
                    })
            } else {
                Ok(distance.div_ceil(step))
            }
        }
    }
}

// =============================================================================
// Loop domain
// =============================================================================

/// Domain over which a `for` construct iterates.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoopDomain {
    /// Iterate over an integer range.
    Integer(IntegerLoopRange),

    /// Iterate over a logical qubit range.
    ///
    /// The range is represented symbolically and therefore does not require
    /// materializing every qubit identifier.
    Qubits(QubitRange),
}

impl LoopDomain {
    /// Validates the loop domain.
    pub fn validate(&self) -> ControlFlowResult<()> {
        match self {
            Self::Integer(range) => {
                let _ = range.iteration_count()?;
                Ok(())
            }

            Self::Qubits(range) => {
                if range.start() > range.end() {
                    return Err(
                        ControlFlowError::InvalidLoopRange {
                            reason: "qubit range start exceeds end",
                        },
                    );
                }

                Ok(())
            }
        }
    }

    /// Returns a symbolic iteration count where it is representable.
    pub fn iteration_count(&self) -> ControlFlowResult<u128> {
        match self {
            Self::Integer(range) => range.iteration_count(),

            Self::Qubits(range) => {
                let count = range
                    .end()
                    .checked_sub(range.start())
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "qubit loop iteration count",
                    })?;

                u128::try_from(count).map_err(|_| {
                    ControlFlowError::ArithmeticOverflow {
                        calculation:
                            "qubit loop iteration count conversion",
                    }
                })
            }
        }
    }
}

// =============================================================================
// Repeat policy
// =============================================================================

/// Iteration policy for a repeat loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepeatCount {
    /// Repeat exactly a fixed number of times.
    Exact(u128),

    /// Repeat indefinitely until control flow exits the loop.
    Unbounded,
}

impl RepeatCount {
    /// Returns the fixed iteration count if bounded.
    #[must_use]
    pub const fn exact(self) -> Option<u128> {
        match self {
            Self::Exact(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Returns whether this represents unbounded repetition.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }
}

// =============================================================================
// Block
// =============================================================================

/// Structured control-flow block.
///
/// A block owns an ordered sequence of control-flow nodes.
///
/// The block deliberately does not contain source-language syntax.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlFlowBlock {
    nodes: Vec<ControlFlowNode>,
}

impl ControlFlowBlock {
    /// Creates an empty block.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// Creates a block with pre-existing nodes.
    pub fn from_nodes(
        nodes: Vec<ControlFlowNode>,
    ) -> ControlFlowResult<Self> {
        let block = Self { nodes };
        block.validate()?;
        Ok(block)
    }

    /// Returns the number of direct nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the block contains no direct nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns an immutable view of direct nodes.
    #[must_use]
    pub fn nodes(&self) -> &[ControlFlowNode] {
        &self.nodes
    }

    /// Appends a node.
    ///
    /// Validation of the complete tree is available through `validate`.
    pub fn push(
        &mut self,
        node: ControlFlowNode,
    ) -> ControlFlowResult<()> {
        node.validate()?;
        self.nodes.push(node);
        Ok(())
    }

    /// Removes a node by index.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Option<ControlFlowNode> {
        if index < self.nodes.len() {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    /// Clears the block.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Returns an iterator over direct nodes.
    pub fn iter(&self) -> std::slice::Iter<'_, ControlFlowNode> {
        self.nodes.iter()
    }

    /// Validates the block structurally.
    pub fn validate(&self) -> ControlFlowResult<()> {
        for node in &self.nodes {
            node.validate()?;
        }

        Ok(())
    }

    /// Validates the block under a complete IR resource policy.
    pub fn validate_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> ControlFlowResult<()> {
        self.validate_with_limits_at_depth(limits, 0)
    }

    fn validate_with_limits_at_depth(
        &self,
        limits: &QuantumIrLimits,
        depth: usize,
    ) -> ControlFlowResult<()> {
        for node in &self.nodes {
            node.validate_with_limits_at_depth(
                limits,
                depth,
            )?;
        }

        Ok(())
    }

    /// Counts direct and nested nodes without materializing anything.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        let mut count = 0usize;

        for node in &self.nodes {
            count = count
                .checked_add(node.node_count()?)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "control-flow node count",
                })?;
        }

        Ok(count)
    }

    /// Calculates the maximum structured control-flow nesting depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.nodes
            .iter()
            .map(ControlFlowNode::depth)
            .max()
            .unwrap_or(0)
    }

    /// Returns whether this block contains a loop transfer.
    #[must_use]
    pub fn contains_loop_transfer(&self) -> bool {
        self.nodes
            .iter()
            .any(ControlFlowNode::contains_loop_transfer)
    }
}

impl Default for ControlFlowBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ControlFlowBlock {
    type Item = ControlFlowNode;
    type IntoIter = std::vec::IntoIter<ControlFlowNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<'a> IntoIterator for &'a ControlFlowBlock {
    type Item = &'a ControlFlowNode;
    type IntoIter = std::slice::Iter<'a, ControlFlowNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter()
    }
}

// =============================================================================
// Control-flow node
// =============================================================================

/// One structured control-flow node.
///
/// Operation references are intentionally represented by `OperationId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlFlowNode {
    /// Execute an existing IR operation.
    Operation(OperationId),

    /// Conditional branch.
    If {
        /// Branch predicate.
        condition: ClassicalPredicate,

        /// Then branch.
        then_block: ControlFlowBlock,

        /// Optional else branch.
        else_block: Option<ControlFlowBlock>,
    },

    /// Pre-condition loop.
    While {
        /// Loop predicate.
        condition: ClassicalPredicate,

        /// Loop body.
        body: ControlFlowBlock,
    },

    /// Post-condition loop.
    DoWhile {
        /// Loop body.
        body: ControlFlowBlock,

        /// Loop predicate evaluated after the body.
        condition: ClassicalPredicate,
    },

    /// Counted/range loop.
    For {
        /// Loop variable.
        variable: LoopVariable,

        /// Iteration domain.
        domain: LoopDomain,

        /// Loop body.
        body: ControlFlowBlock,
    },

    /// Repeat loop.
    Repeat {
        /// Number of repetitions.
        count: RepeatCount,

        /// Loop body.
        body: ControlFlowBlock,
    },

    /// Exit the nearest enclosing loop.
    Break,

    /// Continue the nearest enclosing loop.
    Continue,

    /// Return from the enclosing function.
    Return,
}

impl ControlFlowNode {
    /// Creates an operation node.
    #[must_use]
    pub const fn operation(operation: OperationId) -> Self {
        Self::Operation(operation)
    }

    /// Creates an `if` node.
    #[must_use]
    pub fn if_then(
        condition: ClassicalPredicate,
        then_block: ControlFlowBlock,
    ) -> Self {
        Self::If {
            condition,
            then_block,
            else_block: None,
        }
    }

    /// Creates an `if/else` node.
    #[must_use]
    pub fn if_then_else(
        condition: ClassicalPredicate,
        then_block: ControlFlowBlock,
        else_block: ControlFlowBlock,
    ) -> Self {
        Self::If {
            condition,
            then_block,
            else_block: Some(else_block),
        }
    }

    /// Creates a `while` node.
    #[must_use]
    pub fn while_loop(
        condition: ClassicalPredicate,
        body: ControlFlowBlock,
    ) -> Self {
        Self::While {
            condition,
            body,
        }
    }

    /// Creates a `do while` node.
    #[must_use]
    pub fn do_while(
        body: ControlFlowBlock,
        condition: ClassicalPredicate,
    ) -> Self {
        Self::DoWhile {
            body,
            condition,
        }
    }

    /// Creates an integer/range `for` loop.
    #[must_use]
    pub fn for_loop(
        variable: LoopVariable,
        domain: IntegerLoopRange,
        body: ControlFlowBlock,
    ) -> Self {
        Self::For {
            variable,
            domain: LoopDomain::Integer(domain),
            body,
        }
    }

    /// Creates a logical-qubit iteration loop.
    #[must_use]
    pub fn for_each_qubit(
        variable: LoopVariable,
        range: QubitRange,
        body: ControlFlowBlock,
    ) -> Self {
        Self::For {
            variable,
            domain: LoopDomain::Qubits(range),
            body,
        }
    }

    /// Creates a repeat loop.
    #[must_use]
    pub fn repeat(
        count: RepeatCount,
        body: ControlFlowBlock,
    ) -> Self {
        Self::Repeat { count, body }
    }

    /// Creates a break.
    #[must_use]
    pub const fn break_loop() -> Self {
        Self::Break
    }

    /// Creates a continue.
    #[must_use]
    pub const fn continue_loop() -> Self {
        Self::Continue
    }

    /// Creates a return.
    #[must_use]
    pub const fn return_from_function() -> Self {
        Self::Return
    }

    /// Validates local structural invariants.
    pub fn validate(&self) -> ControlFlowResult<()> {
        match self {
            Self::Operation(operation) => {
                Self::validate_operation_id(*operation)
            }

            Self::If {
                condition,
                then_block,
                else_block,
            } => {
                condition.validate()?;

                if then_block.is_empty() {
                    return Err(
                        ControlFlowError::EmptyRequiredBlock {
                            block: "if/then",
                        },
                    );
                }

                then_block.validate()?;

                if let Some(block) = else_block {
                    if block.is_empty() {
                        return Err(
                            ControlFlowError::EmptyRequiredBlock {
                                block: "if/else",
                            },
                        );
                    }

                    block.validate()?;
                }

                Ok(())
            }

            Self::While {
                condition,
                body,
            } => {
                condition.validate()?;

                if body.is_empty() {
                    return Err(
                        ControlFlowError::EmptyRequiredBlock {
                            block: "while",
                        },
                    );
                }

                body.validate()
            }

            Self::DoWhile {
                body,
                condition,
            } => {
                condition.validate()?;

                if body.is_empty() {
                    return Err(
                        ControlFlowError::EmptyRequiredBlock {
                            block: "do/while",
                        },
                    );
                }

                body.validate()
            }

            Self::For {
                variable,
                domain,
                body,
            } => {
                Self::validate_loop_variable(*variable)?;
                domain.validate()?;

                if body.is_empty() {
                    return Err(
                        ControlFlowError::EmptyRequiredBlock {
                            block: "for",
                        },
                    );
                }

                body.validate()
            }

            Self::Repeat {
                count,
                body,
            } => {
                if matches!(count, RepeatCount::Exact(0)) {
                    // An exact zero-repeat body is semantically valid and is
                    // intentionally retained rather than optimized away.
                    //
                    // This preserves source/IR semantics for later passes.
                }

                if body.is_empty() {
                    return Err(
                        ControlFlowError::EmptyRequiredBlock {
                            block: "repeat",
                        },
                    );
                }

                body.validate()
            }

            Self::Break |
            Self::Continue |
            Self::Return => Ok(()),
        }
    }

    /// Validates the node against an explicit IR resource policy.
    pub fn validate_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> ControlFlowResult<()> {
        self.validate_with_limits_at_depth(limits, 0)
    }

    fn validate_with_limits_at_depth(
        &self,
        limits: &QuantumIrLimits,
        depth: usize,
    ) -> ControlFlowResult<()> {
        self.validate_local_with_limits(limits)?;

        let next_depth = match self {
            Self::Operation(_) |
            Self::Break |
            Self::Continue |
            Self::Return => depth,

            Self::If { .. } |
            Self::While { .. } |
            Self::DoWhile { .. } |
            Self::For { .. } |
            Self::Repeat { .. } => depth
                .checked_add(1)
                .ok_or(ControlFlowError::ArithmeticOverflow {
                    calculation: "control-flow nesting depth",
                })?,
        };

        if next_depth > limits.max_control_flow_depth {
            return Err(
                ControlFlowError::ControlFlowDepthExceeded {
                    requested: next_depth,
                    maximum: limits.max_control_flow_depth,
                },
            );
        }

        match self {
            Self::If {
                then_block,
                else_block,
                ..
            } => {
                then_block.validate_with_limits_at_depth(
                    limits,
                    next_depth,
                )?;

                if let Some(block) = else_block {
                    block.validate_with_limits_at_depth(
                        limits,
                        next_depth,
                    )?;
                }
            }

            Self::While { body, .. }
            | Self::DoWhile { body, .. }
            | Self::For { body, .. }
            | Self::Repeat { body, .. } => {
                body.validate_with_limits_at_depth(
                    limits,
                    next_depth,
                )?;
            }

            Self::Operation(_) |
            Self::Break |
            Self::Continue |
            Self::Return => {}
        }

        Ok(())
    }

    fn validate_local_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> ControlFlowResult<()> {
        match self {
            Self::If {
                condition,
                ..
            }
            | Self::While {
                condition,
                ..
            }
            | Self::DoWhile {
                condition,
                ..
            } => {
                let terms = condition.term_count()?;

                // The current limits contract exposes the overall operand
                // policy. Conditions are semantic operands, so the
                // per-operation operand policy is the appropriate safety
                // boundary until a dedicated condition-term limit exists.
                if terms > limits.max_operands {
                    return Err(
                        ControlFlowError::ConditionLimitExceeded {
                            requested: terms,
                            maximum: limits.max_operands,
                        },
                    );
                }
            }

            Self::Operation(_) |
            Self::For { .. } |
            Self::Repeat { .. } |
            Self::Break |
            Self::Continue |
            Self::Return => {}
        }

        Ok(())
    }

    /// Returns the number of nodes in this subtree.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        let mut count = 1usize;

        match self {
            Self::Operation(_) |
            Self::Break |
            Self::Continue |
            Self::Return => {}

            Self::If {
                then_block,
                else_block,
                ..
            } => {
                count = count
                    .checked_add(then_block.node_count()?)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "if node count",
                    })?;

                if let Some(block) = else_block {
                    count = count
                        .checked_add(block.node_count()?)
                        .ok_or(
                            ControlFlowError::ArithmeticOverflow {
                                calculation: "else node count",
                            },
                        )?;
                }
            }

            Self::While { body, .. }
            | Self::DoWhile { body, .. }
            | Self::For { body, .. }
            | Self::Repeat { body, .. } => {
                count = count
                    .checked_add(body.node_count()?)
                    .ok_or(ControlFlowError::ArithmeticOverflow {
                        calculation: "nested control-flow node count",
                    })?;
            }
        }

        Ok(count)
    }

    /// Returns the maximum nested control-flow depth of this node.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::Operation(_) |
            Self::Break |
            Self::Continue |
            Self::Return => 0,

            Self::If {
                then_block,
                else_block,
                ..
            } => {
                let child_depth = then_block.depth();

                let else_depth = else_block
                    .as_ref()
                    .map(ControlFlowBlock::depth)
                    .unwrap_or(0);

                1usize
                    .saturating_add(child_depth.max(else_depth))
            }

            Self::While { body, .. }
            | Self::DoWhile { body, .. }
            | Self::For { body, .. }
            | Self::Repeat { body, .. } => {
                1usize.saturating_add(body.depth())
            }
        }
    }

    /// Returns whether this node contains a loop transfer.
    #[must_use]
    pub fn contains_loop_transfer(&self) -> bool {
        match self {
            Self::Break |
            Self::Continue => true,

            Self::If {
                then_block,
                else_block,
                ..
            } => {
                then_block.contains_loop_transfer()
                    || else_block
                        .as_ref()
                        .map(ControlFlowBlock::contains_loop_transfer)
                        .unwrap_or(false)
            }

            Self::While { body, .. }
            | Self::DoWhile { body, .. }
            | Self::For { body, .. }
            | Self::Repeat { body, .. } => {
                body.contains_loop_transfer()
            }

            Self::Operation(_) |
            Self::Return => false,
        }
    }

    /// Validates whether this node can appear in the supplied structured
    /// context.
    pub fn validate_transfers(
        &self,
        loop_depth: usize,
        function_depth: usize,
    ) -> ControlFlowResult<()> {
        match self {
            Self::Break |
            Self::Continue => {
                if loop_depth == 0 {
                    let transfer = match self {
                        Self::Break => ControlTransfer::Break,
                        Self::Continue => ControlTransfer::Continue,
                        _ => unreachable!(),
                    };

                    return Err(
                        ControlFlowError::TransferOutsideLoop {
                            transfer,
                        },
                    );
                }
            }

            Self::Return => {
                if function_depth == 0 {
                    return Err(
                        ControlFlowError::ReturnOutsideFunction,
                    );
                }
            }

            Self::If {
                then_block,
                else_block,
                ..
            } => {
                for node in then_block.nodes() {
                    node.validate_transfers(
                        loop_depth,
                        function_depth,
                    )?;
                }

                if let Some(block) = else_block {
                    for node in block.nodes() {
                        node.validate_transfers(
                            loop_depth,
                            function_depth,
                        )?;
                    }
                }
            }

            Self::While { body, .. }
            | Self::DoWhile { body, .. }
            | Self::For { body, .. }
            | Self::Repeat { body, .. } => {
                for node in body.nodes() {
                    node.validate_transfers(
                        loop_depth.saturating_add(1),
                        function_depth,
                    )?;
                }
            }

            Self::Operation(_) => {}
        }

        Ok(())
    }

    fn validate_operation_id(
        operation: OperationId,
    ) -> ControlFlowResult<()> {
        // OperationId is intentionally opaque. Existence is checked by the
        // enclosing program/circuit operation table. This module therefore
        // only records the reference.
        //
        // The explicit function exists so this boundary remains stable when
        // operation.rs becomes the canonical operation registry.
        let _ = operation;
        Ok(())
    }

    fn validate_loop_variable(
        _variable: LoopVariable,
    ) -> ControlFlowResult<()> {
        Ok(())
    }
}

// =============================================================================
// Control-flow region
// =============================================================================

/// Top-level structured control-flow region.
///
/// A region is the unit consumed by a future `QuantumProgram`/`Region`
/// representation.
///
/// It deliberately contains no source-language function name, ABI, backend
/// information, or hardware information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlFlowRegion {
    body: ControlFlowBlock,
}

impl ControlFlowRegion {
    /// Creates an empty region.
    #[must_use]
    pub fn new() -> Self {
        Self {
            body: ControlFlowBlock::new(),
        }
    }

    /// Creates a region from a complete body.
    pub fn from_block(
        body: ControlFlowBlock,
    ) -> ControlFlowResult<Self> {
        body.validate()?;

        Ok(Self { body })
    }

    /// Returns the body.
    #[must_use]
    pub fn body(&self) -> &ControlFlowBlock {
        &self.body
    }

    /// Returns a mutable body.
    ///
    /// Mutation remains local to this region. Call `validate()` before
    /// crossing a compiler-stage boundary.
    #[must_use]
    pub fn body_mut(&mut self) -> &mut ControlFlowBlock {
        &mut self.body
    }

    /// Appends a node to the region.
    pub fn push(
        &mut self,
        node: ControlFlowNode,
    ) -> ControlFlowResult<()> {
        self.body.push(node)
    }

    /// Validates the region structurally.
    pub fn validate(&self) -> ControlFlowResult<()> {
        self.body.validate()
    }

    /// Validates the region under an explicit IR resource policy.
    pub fn validate_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> ControlFlowResult<()> {
        self.body.validate_with_limits(limits)
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> ControlFlowResult<usize> {
        self.body.node_count()
    }

    /// Returns maximum control-flow depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.body.depth()
    }

    /// Validates structured transfers.
    ///
    /// `function_depth` is supplied by the enclosing program/function model.
    pub fn validate_transfers(
        &self,
        function_depth: usize,
    ) -> ControlFlowResult<()> {
        for node in self.body.nodes() {
            node.validate_transfers(0, function_depth)?;
        }

        Ok(())
    }
}

impl Default for ControlFlowRegion {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Convenience aliases
// =============================================================================

/// Compatibility alias for callers that use `ControlFlow` as the region name.
pub type ControlFlow = ControlFlowRegion;

/// Compatibility alias for callers that use `Block`.
pub type Block = ControlFlowBlock;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(index: usize) -> OperationId {
        OperationId::new(index)
    }

    #[test]
    fn classical_predicate_bit_is_valid() {
        let predicate =
            ClassicalPredicate::bit(ClassicalBitId::new(3));

        assert!(predicate.validate().is_ok());
        assert!(
            predicate
                .validate_classical_bits(4)
                .is_ok()
        );
    }

    #[test]
    fn classical_predicate_rejects_out_of_range_bit() {
        let predicate =
            ClassicalPredicate::bit(ClassicalBitId::new(4));

        let result =
            predicate.validate_classical_bits(4);

        assert!(matches!(
            result,
            Err(
                ControlFlowError::ClassicalBitOutOfRange {
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_boolean_expression_is_rejected() {
        assert!(
            ClassicalPredicate::and(Vec::new()).is_err()
        );

        assert!(
            ClassicalPredicate::or(Vec::new()).is_err()
        );

        assert!(
            ClassicalPredicate::xor(Vec::new()).is_err()
        );
    }

    #[test]
    fn integer_loop_range_counts_without_materializing() {
        let range =
            IntegerLoopRange::exclusive(0, 1_000_000, 1)
                .expect("range should be valid");

        assert_eq!(
            range
                .iteration_count()
                .expect("count should succeed"),
            1_000_000
        );
    }

    #[test]
    fn descending_loop_range_is_supported() {
        let range =
            IntegerLoopRange::inclusive(10, 0, -2)
                .expect("range should be valid");

        assert_eq!(
            range
                .iteration_count()
                .expect("count should succeed"),
            6
        );
    }

    #[test]
    fn zero_loop_step_is_rejected() {
        let result =
            IntegerLoopRange::exclusive(0, 10, 0);

        assert!(matches!(
            result,
            Err(ControlFlowError::ZeroLoopStep)
        ));
    }

    #[test]
    fn operation_node_is_valid() {
        let node =
            ControlFlowNode::operation(operation(0));

        assert!(node.validate().is_ok());
        assert_eq!(
            node.node_count().expect("count should succeed"),
            1
        );
    }

    #[test]
    fn if_else_structure_is_valid() {
        let mut then_block =
            ControlFlowBlock::new();

        then_block
            .push(ControlFlowNode::operation(operation(0)))
            .expect("then operation should be valid");

        let mut else_block =
            ControlFlowBlock::new();

        else_block
            .push(ControlFlowNode::operation(operation(1)))
            .expect("else operation should be valid");

        let node = ControlFlowNode::if_then_else(
            ClassicalPredicate::bit(
                ClassicalBitId::new(0),
            ),
            then_block,
            else_block,
        );

        assert!(node.validate().is_ok());
        assert_eq!(
            node.node_count().expect("count should succeed"),
            3
        );
        assert_eq!(node.depth(), 1);
    }

    #[test]
    fn empty_then_block_is_rejected() {
        let node = ControlFlowNode::if_then(
            ClassicalPredicate::always(),
            ControlFlowBlock::new(),
        );

        assert!(matches!(
            node.validate(),
            Err(
                ControlFlowError::EmptyRequiredBlock {
                    block: "if/then"
                }
            )
        ));
    }

    #[test]
    fn nested_control_flow_depth_is_calculated() {
        let mut inner =
            ControlFlowBlock::new();

        inner
            .push(ControlFlowNode::operation(operation(0)))
            .expect("operation should be valid");

        let inner_if = ControlFlowNode::if_then(
            ClassicalPredicate::always(),
            inner,
        );

        let mut outer =
            ControlFlowBlock::new();

        outer
            .push(inner_if)
            .expect("inner if should be valid");

        let outer_while = ControlFlowNode::while_loop(
            ClassicalPredicate::always(),
            outer,
        );

        assert_eq!(outer_while.depth(), 2);
    }

    #[test]
    fn qubit_iteration_uses_canonical_qubit_identity() {
        let range =
            QubitRange::new(0, 1_000_000)
                .expect("qubit range should be valid");

        let node = ControlFlowNode::for_each_qubit(
            LoopVariable::new(0),
            range,
            {
                let mut body =
                    ControlFlowBlock::new();

                body.push(
                    ControlFlowNode::operation(
                        operation(0),
                    ),
                )
                .expect("operation should be valid");

                body
            },
        );

        assert!(node.validate().is_ok());
        assert_eq!(
            node.node_count().expect("count should succeed"),
            2
        );
    }

    #[test]
    fn loop_transfer_requires_loop_context() {
        let node =
            ControlFlowNode::break_loop();

        assert!(matches!(
            node.validate_transfers(0, 0),
            Err(
                ControlFlowError::TransferOutsideLoop {
                    transfer: ControlTransfer::Break
                }
            )
        ));
    }

    #[test]
    fn loop_transfer_is_valid_inside_loop() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::continue_loop(),
        )
        .expect("continue should be structurally valid");

        let loop_node = ControlFlowNode::while_loop(
            ClassicalPredicate::always(),
            body,
        );

        assert!(
            loop_node
                .validate_transfers(0, 0)
                .is_ok()
        );
    }

    #[test]
    fn return_requires_function_context() {
        let node =
            ControlFlowNode::return_from_function();

        assert!(matches!(
            node.validate_transfers(0, 0),
            Err(
                ControlFlowError::ReturnOutsideFunction
            )
        ));

        assert!(
            node.validate_transfers(0, 1).is_ok()
        );
    }

    #[test]
    fn bounded_repeat_is_valid() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(operation(0)),
        )
        .expect("operation should be valid");

        let node = ControlFlowNode::repeat(
            RepeatCount::Exact(1_000_000_000_000u128),
            body,
        );

        assert!(node.validate().is_ok());
    }

    #[test]
    fn unbounded_repeat_is_representable() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::break_loop(),
        )
        .expect("break should be structurally valid");

        let node = ControlFlowNode::repeat(
            RepeatCount::Unbounded,
            body,
        );

        assert!(node.validate().is_ok());
    }

    #[test]
    fn condition_term_limit_is_checked() {
        let condition =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::bit(
                    ClassicalBitId::new(0),
                ),
                ClassicalPredicate::bit(
                    ClassicalBitId::new(1),
                ),
                ClassicalPredicate::bit(
                    ClassicalBitId::new(2),
                ),
            ])
            .expect("condition should be valid");

        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(operation(0)),
        )
        .expect("operation should be valid");

        let node =
            ControlFlowNode::if_then(condition, body);

        // The production policy's operand limit is expected to be greater
        // than this condition, so the node itself should validate.
        let limits = QuantumIrLimits::production();

        assert!(
            node.validate_with_limits(&limits).is_ok()
        );
    }

    #[test]
    fn region_can_be_constructed_and_validated() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(0),
            ))
            .expect("operation should be valid");

        assert!(region.validate().is_ok());
        assert_eq!(
            region.node_count().expect("count should succeed"),
            1
        );
    }

    #[test]
    fn very_large_qubit_range_is_symbolic() {
        let range =
            QubitRange::new(0, usize::MAX)
                .expect("range should be representable");

        let domain = LoopDomain::Qubits(range);

        let count =
            domain.iteration_count()
                .expect("usize count should convert to u128");

        assert_eq!(
            count,
            usize::MAX as u128
        );
    }
}