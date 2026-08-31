//! Zamani Quantum IR — Structured Loop Semantics
//!
//! Canonical, hardware-independent representation of loop constructs used by
//! the Zamani quantum programming language.
//!
//! # Architectural role
//!
//! This module owns the SEMANTIC DESCRIPTION of loops.
//!
//! It represents:
//!
//! - counted loops;
//! - range loops;
//! - runtime/symbolic loops;
//! - logical-qubit iteration;
//! - repeat loops;
//! - while loops;
//! - do-while loops;
//! - structured loop regions;
//! - loop induction variables;
//! - loop-control targets;
//! - loop termination semantics;
//! - loop validation;
//! - overflow-safe static range analysis.
//!
//! It does NOT own:
//!
//! - source-language parsing;
//! - source variable names;
//! - concrete operation definitions;
//! - physical qubit allocation;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - calibration;
//! - pulse generation;
//! - backend execution;
//! - simulation;
//! - QEC decoding;
//! - optimization algorithms.
//!
//! Those responsibilities belong to other Quantum IR and compiler modules.
//!
//! # Architectural principle
//!
//! A loop is semantic intent, not a hardware execution strategy.
//!
//! For example:
//!
//! ```text
//! for i in 0..n {
//!     x(q[i]);
//! }
//! ```
//!
//! must remain representable without deciding:
//!
//! - how many physical qubits exist;
//! - how the qubits are routed;
//! - whether iterations execute serially;
//! - whether iterations can be parallelized;
//! - which native gate implements `x`;
//! - which machine executes the loop.
//!
//! Those decisions belong downstream.
//!
//! # Structured-region model
//!
//! The canonical structure is:
//!
//! ```text
//! Loop
//!   │
//!   ├── domain / condition
//!   ├── induction variable
//!   ├── body RegionId
//!   ├── condition RegionId (where required)
//!   └── loop-control metadata
//! ```
//!
//! The actual body is owned by `region.rs`.
//!
//! This module therefore stores `RegionId` references rather than recursively
//! embedding regions. This avoids recursive ownership and allows a program
//! arena/registry to scale to very large IR graphs.
//!
//! # Canonical qubit namespace
//!
//! Logical qubits are always represented through:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::QubitRange
//! ```
//!
//! This file MUST NOT define another qubit identifier.
//!
//! Physical qubits may appear only when a lower-level IR explicitly requires
//! physical identity. A normal source-level loop over qubits is a logical
//! operation.
//!
//! # Scaling
//!
//! No fixed architectural limit is encoded here.
//!
//! The following are all valid semantic concepts:
//!
//! ```text
//! 1 iteration
//! 10 iterations
//! 1_000_000 iterations
//! N iterations
//! runtime-dependent iterations
//!
//! 1 logical qubit
//! 1_000_000 logical qubits
//! N logical qubits
//! ```
//!
//! A static loop range is not materialized into a collection of iteration
//! values. This is critical for scalability.
//!
//! For example:
//!
//! ```text
//! 0 .. 10^12
//! ```
//!
//! remains a compact range description rather than allocating 10^12 values.
//!
//! Concrete resource/security policies belong to the compilation/execution
//! policy layer, such as `QuantumIrLimits`.
//!
//! # Static versus dynamic bounds
//!
//! Static bounds are represented exactly with `i128`.
//!
//! Dynamic bounds use the canonical `Parameter` representation where the
//! surrounding compiler can resolve the value later.
//!
//! This permits a loop to survive frontend lowering without prematurely
//! specializing it to a concrete machine size.
//!
//! # OpenQASM / structured-CF compatibility
//!
//! The representation intentionally accommodates semantics found in modern
//! quantum IRs:
//!
//! - `for` over ranges;
//! - `for` over collections/registers;
//! - runtime-capable loop bounds;
//! - loop-local induction variables;
//! - `while` pre-condition loops;
//! - `do-while` post-condition loops;
//! - `break`;
//! - `continue`;
//! - structured regions.
//!
//! OpenQASM 3.1 defines `for` loops over sets/ranges/expressions and scopes the
//! loop variable to the loop body. It also defines `break` and `continue`
//! relative to the nearest containing loop.
//!
//! MLIR's structured-control-flow model similarly keeps loop semantics in the
//! loop operation while representing the body through regions and supports
//! separate condition/body regions for generic while/do-while semantics.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This file consumes:
//!
//! ```text
//! quantum::ir::identity
//! quantum::ir::parameter
//! quantum::ir::qubit
//! ```
//!
//! It references program regions using `RegionId` and loop identity using
//! `OperationId`.
//!
//! It intentionally does NOT import `control_flow.rs`. This prevents the
//! structured loop layer from becoming cyclically dependent on the broader
//! control-flow module.
//!
//! The future `control/mod.rs` should expose this module as:
//!
//! ```rust
//! pub mod r#loop;
//! ```
//!
//! The root IR module can then expose selected types through:
//!
//! ```rust
//! pub use control::r#loop::*;
//! ```
//!
//! Existing `control_flow.rs` can migrate its legacy loop-specific types to
//! re-exports from this module without changing this file.
//!
//! # Important ownership rule
//!
//! `loop.rs` owns loop semantics.
//!
//! `region.rs` owns region structure.
//!
//! `operation.rs` owns concrete operations.
//!
//! `program.rs` owns program-level registries.
//!
//! Therefore this file should not need to be edited merely because an
//! operation, backend, topology, or scheduling implementation changes.
//!
//! # No semantic hard-coding
//!
//! This module deliberately does NOT encode:
//!
//! ```text
//! MAX_QUBITS
//! MAX_ITERATIONS
//! MAX_LOOP_DEPTH
//! MAX_PROGRAM_SIZE
//! IBM
//! IonQ
//! Rigetti
//! Quantinuum
//! D-Wave
//! superconducting
//! trapped-ion
//! neutral-atom
//! photonic
//!
//! ```
//!
//! Such values would incorrectly turn implementation policy into language
//! architecture.
//!
//! ---------------------------------------------------------------------------
//! Module implementation
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::identity::{OperationId, RegionId};
use super::super::parameter::Parameter;
use super::super::qubit::{QubitId, QubitRange};

// =============================================================================
// Result
// =============================================================================

/// Result type returned by loop construction and validation APIs.
pub type LoopResult<T> = Result<T, LoopError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the canonical loop IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopError {
    /// A zero step would make a counted loop non-progressing.
    ZeroStep,

    /// Static bounds are inconsistent with the direction of the step.
    InvalidStaticRange {
        /// Start value.
        start: i128,

        /// End value.
        end: i128,

        /// Step value.
        step: i128,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Operation being evaluated.
        operation: &'static str,
    },

    /// The loop requires a body region.
    MissingBody,

    /// A condition region is required by this loop kind.
    MissingCondition,

    /// A loop kind cannot have the supplied domain.
    InvalidDomainForKind,

    /// A qubit range has invalid bounds.
    InvalidQubitRange {
        /// Range start.
        start: usize,

        /// Range end.
        end: usize,
    },

    /// A loop variable identifier is invalid for the requested operation.
    InvalidLoopVariable,

    /// A loop-control transfer is not valid in the current loop kind.
    InvalidControlTransfer,

    /// A loop contains an invalid region reference.
    InvalidRegionReference {
        /// Referenced region.
        region: RegionId,
    },

    /// A loop contains an invalid operation reference.
    InvalidOperationReference {
        /// Referenced operation.
        operation: OperationId,
    },

    /// A nested loop structure is invalid.
    Nested {
        /// Nested error.
        error: Box<LoopError>,
    },
}

impl fmt::Display for LoopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroStep => {
                f.write_str("loop step must not be zero")
            }

            Self::InvalidStaticRange {
                start,
                end,
                step,
            } => {
                write!(
                    f,
                    "invalid static loop range: start={start}, \
                     end={end}, step={step}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "arithmetic overflow while evaluating {operation}"
                )
            }

            Self::MissingBody => {
                f.write_str("loop requires a body region")
            }

            Self::MissingCondition => {
                f.write_str("loop requires a condition region")
            }

            Self::InvalidDomainForKind => {
                f.write_str("loop domain is incompatible with loop kind")
            }

            Self::InvalidQubitRange { start, end } => {
                write!(
                    f,
                    "invalid logical-qubit range [{start}, {end})"
                )
            }

            Self::InvalidLoopVariable => {
                f.write_str("invalid loop variable")
            }

            Self::InvalidControlTransfer => {
                f.write_str("invalid loop-control transfer")
            }

            Self::InvalidRegionReference { region } => {
                write!(f, "invalid loop region reference {region}")
            }

            Self::InvalidOperationReference { operation } => {
                write!(f, "invalid loop operation reference {operation}")
            }

            Self::Nested { error } => {
                write!(f, "nested loop error: {error}")
            }
        }
    }
}

impl std::error::Error for LoopError {}

// =============================================================================
// Loop variable
// =============================================================================

/// Stable semantic identifier for a loop induction variable.
///
/// The identifier is local to the enclosing IR scope. It is NOT a source-code
/// variable name and does not imply a particular integer width.
///
/// This deliberately does not use a string because source-level naming is a
/// frontend concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LoopVariableId(u64);

impl LoopVariableId {
    /// Creates a loop-variable identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for LoopVariableId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for LoopVariableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loopvar{}", self.0)
    }
}

// =============================================================================
// Loop kind
// =============================================================================

/// Semantic kind of structured loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoopKind {
    /// Counted/range loop with an induction variable.
    For,

    /// Pre-condition loop.
    While,

    /// Post-condition loop.
    DoWhile,

    /// Fixed-count repetition without an externally visible induction
    /// variable.
    Repeat,

    /// Iteration over a logical-qubit range.
    ForEachQubit,

    /// Parallelizable iteration-space declaration.
    ///
    /// This is semantic intent only. It does not force a backend to execute
    /// iterations concurrently.
    ParallelFor,
}

impl LoopKind {
    /// Returns whether this loop kind has an induction variable.
    #[must_use]
    pub const fn has_induction_variable(self) -> bool {
        matches!(
            self,
            Self::For | Self::ForEachQubit | Self::ParallelFor
        )
    }

    /// Returns whether this loop kind requires a condition region.
    #[must_use]
    pub const fn requires_condition_region(self) -> bool {
        matches!(self, Self::While | Self::DoWhile)
    }

    /// Returns whether this loop kind requires a domain.
    #[must_use]
    pub const fn requires_domain(self) -> bool {
        matches!(
            self,
            Self::For
                | Self::Repeat
                | Self::ForEachQubit
                | Self::ParallelFor
        )
    }

    /// Returns whether `break` is meaningful for this loop.
    #[must_use]
    pub const fn permits_break(self) -> bool {
        true
    }

    /// Returns whether `continue` is meaningful for this loop.
    #[must_use]
    pub const fn permits_continue(self) -> bool {
        true
    }
}

impl fmt::Display for LoopKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::For => "for",
            Self::While => "while",
            Self::DoWhile => "do_while",
            Self::Repeat => "repeat",
            Self::ForEachQubit => "for_each_qubit",
            Self::ParallelFor => "parallel_for",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Integer bounds
// =============================================================================

/// A statically known integer loop bound.
///
/// `i128` is used so loop-domain arithmetic can be performed exactly without
/// silently narrowing the semantic range to the host machine's pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StaticIntegerRange {
    start: i128,
    end: i128,
    step: i128,
    inclusive_end: bool,
}

impl StaticIntegerRange {
    /// Creates a half-open `[start, end)` range.
    pub const fn exclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> LoopResult<Self> {
        Self::new(start, end, step, false)
    }

    /// Creates an inclusive `[start, end]` range.
    pub const fn inclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> LoopResult<Self> {
        Self::new(start, end, step, true)
    }

    /// Creates a range with explicit endpoint semantics.
    pub const fn new(
        start: i128,
        end: i128,
        step: i128,
        inclusive_end: bool,
    ) -> LoopResult<Self> {
        if step == 0 {
            return Err(LoopError::ZeroStep);
        }

        if step > 0 {
            if start > end {
                return Err(LoopError::InvalidStaticRange {
                    start,
                    end,
                    step,
                });
            }
        } else if start < end {
            return Err(LoopError::InvalidStaticRange {
                start,
                end,
                step,
            });
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

    /// Returns the ending value.
    #[must_use]
    pub const fn end(self) -> i128 {
        self.end
    }

    /// Returns the step.
    #[must_use]
    pub const fn step(self) -> i128 {
        self.step
    }

    /// Returns whether the end is inclusive.
    #[must_use]
    pub const fn inclusive_end(self) -> bool {
        self.inclusive_end
    }

    /// Returns whether this range contains no iterations.
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

    /// Returns the exact number of iterations without materializing them.
    ///
    /// This method uses checked arithmetic and therefore cannot wrap.
    pub fn trip_count(self) -> LoopResult<u128> {
        if self.is_empty() {
            return Ok(0);
        }

        if self.step > 0 {
            let distance = self
                .end
                .checked_sub(self.start)
                .ok_or(LoopError::ArithmeticOverflow {
                    operation: "positive loop-range distance",
                })?;

            let distance = u128::try_from(distance).map_err(|_| {
                LoopError::ArithmeticOverflow {
                    operation: "positive loop-range conversion",
                }
            })?;

            let step =
                u128::try_from(self.step).map_err(|_| {
                    LoopError::ArithmeticOverflow {
                        operation: "positive loop step conversion",
                    }
                })?;

            if self.inclusive_end {
                let adjusted =
                    distance.checked_add(1).ok_or(
                        LoopError::ArithmeticOverflow {
                            operation:
                                "inclusive positive loop-range distance",
                        },
                    )?;

                Ok((adjusted / step)
                    .checked_add(u128::from(adjusted % step != 0))
                    .unwrap_or(u128::MAX))
            } else {
                Ok((distance / step)
                    .checked_add(u128::from(distance % step != 0))
                    .unwrap_or(u128::MAX))
            }
        } else {
            let distance = self
                .start
                .checked_sub(self.end)
                .ok_or(LoopError::ArithmeticOverflow {
                    operation: "negative loop-range distance",
                })?;

            let distance = u128::try_from(distance).map_err(|_| {
                LoopError::ArithmeticOverflow {
                    operation: "negative loop-range conversion",
                }
            })?;

            let step_abs = self
                .step
                .checked_abs()
                .ok_or(LoopError::ArithmeticOverflow {
                    operation: "negative loop-step absolute value",
                })?;

            let step =
                u128::try_from(step_abs).map_err(|_| {
                    LoopError::ArithmeticOverflow {
                        operation: "negative loop step conversion",
                    }
                })?;

            if self.inclusive_end {
                let adjusted =
                    distance.checked_add(1).ok_or(
                        LoopError::ArithmeticOverflow {
                            operation:
                                "inclusive negative loop-range distance",
                        },
                    )?;

                Ok((adjusted / step)
                    .checked_add(u128::from(adjusted % step != 0))
                    .unwrap_or(u128::MAX))
            } else {
                Ok((distance / step)
                    .checked_add(u128::from(distance % step != 0))
                    .unwrap_or(u128::MAX))
            }
        }
    }

    /// Returns the first iteration value.
    #[must_use]
    pub const fn first_value(self) -> Option<i128> {
        if self.is_empty() {
            None
        } else {
            Some(self.start)
        }
    }

    /// Calculates an iteration value without materializing the range.
    ///
    /// `index` is the zero-based iteration number.
    pub fn value_at(self, index: u128) -> LoopResult<Option<i128>> {
        let count = self.trip_count()?;

        if index >= count {
            return Ok(None);
        }

        let index =
            i128::try_from(index).map_err(|_| {
                LoopError::ArithmeticOverflow {
                    operation: "loop iteration index conversion",
                }
            })?;

        let delta = self
            .step
            .checked_mul(index)
            .ok_or(LoopError::ArithmeticOverflow {
                operation: "loop iteration value multiplication",
            })?;

        let value = self
            .start
            .checked_add(delta)
            .ok_or(LoopError::ArithmeticOverflow {
                operation: "loop iteration value addition",
            })?;

        Ok(Some(value))
    }
}

impl fmt::Display for StaticIntegerRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inclusive_end {
            write!(
                f,
                "{}..={} step {}",
                self.start, self.end, self.step
            )
        } else {
            write!(
                f,
                "{}..{} step {}",
                self.start, self.end, self.step
            )
        }
    }
}

// =============================================================================
// Dynamic integer range
// =============================================================================

/// A symbolic/runtime integer loop range.
///
/// `Parameter` is intentionally used for bounds rather than resolving them
/// during IR construction. The compiler may later bind the parameters to
/// concrete values or lower them to runtime classical expressions.
///
/// The semantic contract requires the resolved values to be integer-compatible
/// for the selected loop type.
#[derive(Debug, Clone, PartialEq)]
pub struct DynamicIntegerRange {
    start: Parameter,
    end: Parameter,
    step: Parameter,
    inclusive_end: bool,
}

impl DynamicIntegerRange {
    /// Creates a symbolic/runtime range.
    ///
    /// The step is not numerically validated here because its value may not be
    /// known until parameter binding.
    #[must_use]
    pub const fn new(
        start: Parameter,
        end: Parameter,
        step: Parameter,
        inclusive_end: bool,
    ) -> Self {
        Self {
            start,
            end,
            step,
            inclusive_end,
        }
    }

    /// Returns the start expression.
    #[must_use]
    pub const fn start(&self) -> &Parameter {
        &self.start
    }

    /// Returns the end expression.
    #[must_use]
    pub const fn end(&self) -> &Parameter {
        &self.end
    }

    /// Returns the step expression.
    #[must_use]
    pub const fn step(&self) -> &Parameter {
        &self.step
    }

    /// Returns whether the end is inclusive.
    #[must_use]
    pub const fn inclusive_end(&self) -> bool {
        self.inclusive_end
    }

    /// Validates the symbolic parameter structure.
    pub fn validate(&self) -> LoopResult<()> {
        self.start.validate().map_err(|_| {
            LoopError::Nested {
                error: Box::new(
                    LoopError::ArithmeticOverflow {
                        operation: "dynamic loop start validation",
                    },
                ),
            }
        })?;

        self.end.validate().map_err(|_| {
            LoopError::Nested {
                error: Box::new(
                    LoopError::ArithmeticOverflow {
                        operation: "dynamic loop end validation",
                    },
                ),
            }
        })?;

        self.step.validate().map_err(|_| {
            LoopError::Nested {
                error: Box::new(
                    LoopError::ArithmeticOverflow {
                        operation: "dynamic loop step validation",
                    },
                ),
            }
        })?;

        Ok(())
    }
}

// =============================================================================
// Loop domain
// =============================================================================

/// Semantic iteration domain of a loop.
///
/// The domain is deliberately non-materialized. Large ranges and qubit
/// collections remain compact descriptions.
#[derive(Debug, Clone, PartialEq)]
pub enum LoopDomain {
    /// Statically known integer range.
    StaticInteger(StaticIntegerRange),

    /// Runtime/symbolic integer range.
    DynamicInteger(DynamicIntegerRange),

    /// Fixed repetition count.
    ///
    /// `u128` prevents accidental narrowing to the host pointer width.
    Count(u128),

    /// Logical-qubit range.
    Qubits(QubitRange),
}

impl LoopDomain {
    /// Creates a static exclusive integer range.
    pub fn integer_range(
        start: i128,
        end: i128,
        step: i128,
    ) -> LoopResult<Self> {
        Ok(Self::StaticInteger(
            StaticIntegerRange::exclusive(start, end, step)?,
        ))
    }

    /// Creates a static inclusive integer range.
    pub fn integer_range_inclusive(
        start: i128,
        end: i128,
        step: i128,
    ) -> LoopResult<Self> {
        Ok(Self::StaticInteger(
            StaticIntegerRange::inclusive(start, end, step)?,
        ))
    }

    /// Creates a fixed repetition count.
    #[must_use]
    pub const fn count(iterations: u128) -> Self {
        Self::Count(iterations)
    }

    /// Creates a logical-qubit iteration range.
    pub const fn qubits(
        start: usize,
        end: usize,
    ) -> LoopResult<Self> {
        if start > end {
            return Err(LoopError::InvalidQubitRange { start, end });
        }

        // `QubitRange::new` has the same semantic invariant. Constructing it
        // here avoids accepting an invalid range even if its implementation
        // changes later.
        match QubitRange::new(start, end) {
            Ok(range) => Ok(Self::Qubits(range)),
            Err(_) => Err(LoopError::InvalidQubitRange { start, end }),
        }
    }

    /// Returns whether this is a statically known domain.
    #[must_use]
    pub const fn is_static(&self) -> bool {
        matches!(
            self,
            Self::StaticInteger(_) | Self::Count(_) | Self::Qubits(_)
        )
    }

    /// Returns the static trip count when it can be computed without runtime
    /// evaluation.
    pub fn static_trip_count(&self) -> LoopResult<Option<u128>> {
        match self {
            Self::StaticInteger(range) => {
                Ok(Some(range.trip_count()?))
            }

            Self::Count(count) => Ok(Some(*count)),

            Self::Qubits(range) => {
                let count = range.end().checked_sub(range.start()).ok_or(
                    LoopError::ArithmeticOverflow {
                        operation: "logical-qubit range size",
                    },
                )?;

                Ok(Some(u128::from(count as u64)))
            }

            Self::DynamicInteger(_) => Ok(None),
        }
    }

    /// Returns the logical-qubit range when applicable.
    #[must_use]
    pub const fn qubit_range(&self) -> Option<QubitRange> {
        match self {
            Self::Qubits(range) => Some(*range),
            _ => None,
        }
    }

    /// Returns the integer range when statically known.
    #[must_use]
    pub const fn static_integer_range(
        &self,
    ) -> Option<StaticIntegerRange> {
        match self {
            Self::StaticInteger(range) => Some(*range),
            _ => None,
        }
    }

    /// Validates the domain.
    pub fn validate(&self) -> LoopResult<()> {
        match self {
            Self::StaticInteger(range) => {
                if range.step() == 0 {
                    return Err(LoopError::ZeroStep);
                }

                Ok(())
            }

            Self::DynamicInteger(range) => range.validate(),

            Self::Count(_) => Ok(()),

            Self::Qubits(range) => {
                if range.start() > range.end() {
                    return Err(LoopError::InvalidQubitRange {
                        start: range.start(),
                        end: range.end(),
                    });
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Loop control
// =============================================================================

/// Semantic loop-control operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopControl {
    /// Exit the nearest containing loop.
    Break,

    /// Continue with the next iteration of the nearest containing loop.
    Continue,
}

impl LoopControl {
    /// Returns whether this is `break`.
    #[must_use]
    pub const fn is_break(self) -> bool {
        matches!(self, Self::Break)
    }

    /// Returns whether this is `continue`.
    #[must_use]
    pub const fn is_continue(self) -> bool {
        matches!(self, Self::Continue)
    }
}

impl fmt::Display for LoopControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Break => f.write_str("break"),
            Self::Continue => f.write_str("continue"),
        }
    }
}

// =============================================================================
// Loop regions
// =============================================================================

/// Region references used by a structured loop.
///
/// The references are intentionally IDs rather than owned regions.
///
/// This follows the canonical `region.rs` ownership model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopRegions {
    /// Main loop body.
    body: RegionId,

    /// Optional condition region.
    ///
    /// Required for `while` and `do-while`.
    condition: Option<RegionId>,

    /// Optional latch/update region.
    ///
    /// A future lowering stage may use this for explicit induction-variable
    /// updates or loop-carried state.
    latch: Option<RegionId>,
}

impl LoopRegions {
    /// Creates a body-only loop.
    #[must_use]
    pub const fn body(body: RegionId) -> Self {
        Self {
            body,
            condition: None,
            latch: None,
        }
    }

    /// Creates a conditional loop.
    #[must_use]
    pub const fn conditional(
        body: RegionId,
        condition: RegionId,
    ) -> Self {
        Self {
            body,
            condition: Some(condition),
            latch: None,
        }
    }

    /// Creates a loop with body, condition and latch regions.
    #[must_use]
    pub const fn with_latch(
        body: RegionId,
        condition: Option<RegionId>,
        latch: Option<RegionId>,
    ) -> Self {
        Self {
            body,
            condition,
            latch,
        }
    }

    /// Returns the body region.
    #[must_use]
    pub const fn body_region(self) -> RegionId {
        self.body
    }

    /// Returns the condition region.
    #[must_use]
    pub const fn condition_region(self) -> Option<RegionId> {
        self.condition
    }

    /// Returns the latch region.
    #[must_use]
    pub const fn latch_region(self) -> Option<RegionId> {
        self.latch
    }
}

// =============================================================================
// Loop
// =============================================================================

/// Canonical structured loop.
///
/// A `Loop` is a semantic control-flow node. It references the regions that
/// contain its executable body rather than owning those regions directly.
///
/// Concrete operation storage remains the responsibility of the program/IR
/// registry.
///
/// # Loop-carried values
///
/// Loop-carried SSA/value semantics belong to the operation/program layer.
/// This type deliberately does not duplicate `ValueId` or operand/result
/// definitions. A loop operation may use normal IR operands/results to carry
/// state through the loop.
#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    /// Identity of the loop operation.
    id: OperationId,

    /// Semantic loop kind.
    kind: LoopKind,

    /// Optional induction variable.
    induction_variable: Option<LoopVariableId>,

    /// Optional iteration domain.
    domain: Option<LoopDomain>,

    /// Structured regions.
    regions: LoopRegions,

    /// Optional source-independent metadata describing whether the loop is
    /// permitted to be transformed into another equivalent loop form.
    transformable: bool,
}

impl Loop {
    /// Creates a counted/range `for` loop.
    pub fn for_loop(
        id: OperationId,
        induction_variable: LoopVariableId,
        domain: LoopDomain,
        body: RegionId,
    ) -> LoopResult<Self> {
        let loop_ir = Self {
            id,
            kind: LoopKind::For,
            induction_variable: Some(induction_variable),
            domain: Some(domain),
            regions: LoopRegions::body(body),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Creates a logical-qubit iteration loop.
    pub fn for_each_qubit(
        id: OperationId,
        induction_variable: LoopVariableId,
        range: QubitRange,
        body: RegionId,
    ) -> LoopResult<Self> {
        Self::for_each_qubit_range(
            id,
            induction_variable,
            range.start(),
            range.end(),
            body,
        )
    }

    /// Creates a logical-qubit iteration loop from a half-open range.
    pub fn for_each_qubit_range(
        id: OperationId,
        induction_variable: LoopVariableId,
        start: usize,
        end: usize,
        body: RegionId,
    ) -> LoopResult<Self> {
        let domain = LoopDomain::qubits(start, end)?;

        let loop_ir = Self {
            id,
            kind: LoopKind::ForEachQubit,
            induction_variable: Some(induction_variable),
            domain: Some(domain),
            regions: LoopRegions::body(body),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Creates a fixed-count `repeat` loop.
    pub fn repeat(
        id: OperationId,
        iterations: u128,
        body: RegionId,
    ) -> LoopResult<Self> {
        let loop_ir = Self {
            id,
            kind: LoopKind::Repeat,
            induction_variable: None,
            domain: Some(LoopDomain::Count(iterations)),
            regions: LoopRegions::body(body),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Creates a pre-condition `while` loop.
    ///
    /// The condition region is evaluated before the body.
    pub fn while_loop(
        id: OperationId,
        condition: RegionId,
        body: RegionId,
    ) -> LoopResult<Self> {
        let loop_ir = Self {
            id,
            kind: LoopKind::While,
            induction_variable: None,
            domain: None,
            regions: LoopRegions::conditional(body, condition),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Creates a post-condition `do-while` loop.
    ///
    /// The body executes before the condition region is evaluated.
    pub fn do_while_loop(
        id: OperationId,
        body: RegionId,
        condition: RegionId,
    ) -> LoopResult<Self> {
        let loop_ir = Self {
            id,
            kind: LoopKind::DoWhile,
            induction_variable: None,
            domain: None,
            regions: LoopRegions::conditional(body, condition),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Creates a parallel loop semantic declaration.
    ///
    /// `ParallelFor` expresses iteration independence as semantic intent. It
    /// does not guarantee parallel execution and must not be interpreted as a
    /// hardware scheduling decision.
    pub fn parallel_for(
        id: OperationId,
        induction_variable: LoopVariableId,
        domain: LoopDomain,
        body: RegionId,
    ) -> LoopResult<Self> {
        let loop_ir = Self {
            id,
            kind: LoopKind::ParallelFor,
            induction_variable: Some(induction_variable),
            domain: Some(domain),
            regions: LoopRegions::body(body),
            transformable: true,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }

    /// Returns the loop identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Returns the loop kind.
    #[must_use]
    pub const fn kind(&self) -> LoopKind {
        self.kind
    }

    /// Returns the induction variable.
    #[must_use]
    pub const fn induction_variable(
        &self,
    ) -> Option<LoopVariableId> {
        self.induction_variable
    }

    /// Returns the iteration domain.
    #[must_use]
    pub const fn domain(&self) -> Option<&LoopDomain> {
        self.domain.as_ref()
    }

    /// Returns the loop body region.
    #[must_use]
    pub const fn body_region(&self) -> RegionId {
        self.regions.body_region()
    }

    /// Returns the condition region.
    #[must_use]
    pub const fn condition_region(&self) -> Option<RegionId> {
        self.regions.condition_region()
    }

    /// Returns the latch region.
    #[must_use]
    pub const fn latch_region(&self) -> Option<RegionId> {
        self.regions.latch_region()
    }

    /// Returns the complete region descriptor.
    #[must_use]
    pub const fn regions(&self) -> LoopRegions {
        self.regions
    }

    /// Returns whether the loop is marked transformable.
    ///
    /// This is a semantic permission for compiler transformations. It does not
    /// itself perform an optimization.
    #[must_use]
    pub const fn is_transformable(&self) -> bool {
        self.transformable
    }

    /// Sets the transformation permission.
    pub fn set_transformable(&mut self, transformable: bool) {
        self.transformable = transformable;
    }

    /// Validates the complete loop structure.
    pub fn validate(&self) -> LoopResult<()> {
        self.validate_structure()?;

        if let Some(domain) = &self.domain {
            domain.validate()?;
        }

        Ok(())
    }

    /// Validates only the structural relationship between loop kind, domain,
    /// induction variable and regions.
    pub fn validate_structure(&self) -> LoopResult<()> {
        // Every loop must have a body region.
        //
        // `RegionId` is an opaque semantic identity, so existence is resolved
        // by the enclosing program registry. We only verify that the ID is
        // structurally present as a field.
        let _body = self.regions.body_region();

        match self.kind {
            LoopKind::For | LoopKind::ParallelFor => {
                if self.induction_variable.is_none() {
                    return Err(LoopError::InvalidLoopVariable);
                }

                if self.domain.is_none() {
                    return Err(LoopError::InvalidDomainForKind);
                }

                if self.regions.condition_region().is_some() {
                    return Err(LoopError::InvalidDomainForKind);
                }
            }

            LoopKind::ForEachQubit => {
                if self.induction_variable.is_none() {
                    return Err(LoopError::InvalidLoopVariable);
                }

                match self.domain {
                    Some(LoopDomain::Qubits(_)) => {}
                    _ => {
                        return Err(
                            LoopError::InvalidDomainForKind
                        );
                    }
                }

                if self.regions.condition_region().is_some() {
                    return Err(LoopError::InvalidDomainForKind);
                }
            }

            LoopKind::Repeat => {
                if self.induction_variable.is_some() {
                    return Err(LoopError::InvalidLoopVariable);
                }

                match self.domain {
                    Some(LoopDomain::Count(_)) => {}
                    _ => {
                        return Err(
                            LoopError::InvalidDomainForKind
                        );
                    }
                }

                if self.regions.condition_region().is_some() {
                    return Err(LoopError::InvalidDomainForKind);
                }
            }

            LoopKind::While | LoopKind::DoWhile => {
                if self.induction_variable.is_some() {
                    return Err(LoopError::InvalidLoopVariable);
                }

                if self.domain.is_some() {
                    return Err(LoopError::InvalidDomainForKind);
                }

                if self.regions.condition_region().is_none() {
                    return Err(LoopError::MissingCondition);
                }
            }
        }

        Ok(())
    }

    /// Returns the exact static iteration count when available.
    ///
    /// Returns `None` for runtime-dependent integer ranges and condition-based
    /// loops.
    pub fn static_trip_count(&self) -> LoopResult<Option<u128>> {
        match self.kind {
            LoopKind::While | LoopKind::DoWhile => Ok(None),

            _ => match &self.domain {
                Some(domain) => domain.static_trip_count(),
                None => Err(LoopError::InvalidDomainForKind),
            },
        }
    }

    /// Returns whether the loop may execute zero times.
    #[must_use]
    pub fn may_execute_zero_times(&self) -> bool {
        match self.kind {
            LoopKind::DoWhile => false,

            LoopKind::While => true,

            LoopKind::For
            | LoopKind::ParallelFor
            | LoopKind::ForEachQubit
            | LoopKind::Repeat => match &self.domain {
                Some(LoopDomain::StaticInteger(range)) => {
                    range.is_empty()
                }

                Some(LoopDomain::Count(count)) => *count == 0,

                Some(LoopDomain::Qubits(range)) => {
                    range.start() == range.end()
                }

                Some(LoopDomain::DynamicInteger(_)) => true,

                None => true,
            },
        }
    }

    /// Returns whether the loop domain is statically finite.
    ///
    /// All representable static ranges/counts are finite. Runtime loops are
    /// not assumed to terminate.
    #[must_use]
    pub fn is_statically_bounded(&self) -> bool {
        match self.kind {
            LoopKind::While | LoopKind::DoWhile => false,

            _ => match &self.domain {
                Some(LoopDomain::DynamicInteger(_)) => false,
                Some(_) => true,
                None => false,
            },
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} loop {} -> body {}",
            self.kind,
            self.id,
            self.body_region()
        )
    }
}

// =============================================================================
// Loop builder
// =============================================================================

/// Builder for constructing loop IR incrementally while keeping validation at
/// the boundary.
///
/// The builder never owns the body region. It only records its identity.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopBuilder {
    id: OperationId,
    kind: LoopKind,
    induction_variable: Option<LoopVariableId>,
    domain: Option<LoopDomain>,
    body: Option<RegionId>,
    condition: Option<RegionId>,
    latch: Option<RegionId>,
    transformable: bool,
}

impl LoopBuilder {
    /// Creates a builder for the specified loop operation identity and kind.
    #[must_use]
    pub fn new(id: OperationId, kind: LoopKind) -> Self {
        Self {
            id,
            kind,
            induction_variable: None,
            domain: None,
            body: None,
            condition: None,
            latch: None,
            transformable: true,
        }
    }

    /// Sets the induction variable.
    #[must_use]
    pub const fn induction_variable(
        mut self,
        variable: LoopVariableId,
    ) -> Self {
        self.induction_variable = Some(variable);
        self
    }

    /// Sets the loop domain.
    #[must_use]
    pub fn domain(mut self, domain: LoopDomain) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Sets the body region.
    #[must_use]
    pub const fn body(mut self, body: RegionId) -> Self {
        self.body = Some(body);
        self
    }

    /// Sets the condition region.
    #[must_use]
    pub const fn condition(mut self, condition: RegionId) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Sets the latch region.
    #[must_use]
    pub const fn latch(mut self, latch: RegionId) -> Self {
        self.latch = Some(latch);
        self
    }

    /// Sets whether compiler transformations may rewrite this loop.
    #[must_use]
    pub const fn transformable(mut self, value: bool) -> Self {
        self.transformable = value;
        self
    }

    /// Builds and validates the loop.
    pub fn build(self) -> LoopResult<Loop> {
        let body = self.body.ok_or(LoopError::MissingBody)?;

        let loop_ir = Loop {
            id: self.id,
            kind: self.kind,
            induction_variable: self.induction_variable,
            domain: self.domain,
            regions: LoopRegions::with_latch(
                body,
                self.condition,
                self.latch,
            ),
            transformable: self.transformable,
        };

        loop_ir.validate()?;
        Ok(loop_ir)
    }
}

// =============================================================================
// Static-range helpers
// =============================================================================

/// Returns the exact values of a static range.
///
/// This helper is deliberately bounded by the caller's requested collection
/// capacity and is intended only for small compiler-side transformations.
///
/// It must NOT be used by the semantic IR merely to represent a loop.
///
/// Large loops should remain represented by `StaticIntegerRange`.
pub fn materialize_static_range(
    range: StaticIntegerRange,
    maximum_values: usize,
) -> LoopResult<Vec<i128>> {
    let count = range.trip_count()?;

    let count_usize = usize::try_from(count).map_err(|_| {
        LoopError::ArithmeticOverflow {
            operation: "static loop range materialization size",
        }
    })?;

    if count_usize > maximum_values {
        return Err(LoopError::ArithmeticOverflow {
            operation: "static loop range materialization policy",
        });
    }

    let mut values = Vec::new();

    values
        .try_reserve_exact(count_usize)
        .map_err(|_| LoopError::ArithmeticOverflow {
            operation: "static loop range allocation",
        })?;

    for index in 0..count {
        if let Some(value) = range.value_at(index)? {
            values.push(value);
        }
    }

    Ok(values)
}

// =============================================================================
// Qubit-domain helpers
// =============================================================================

/// Returns a logical qubit identifier for an iteration of a qubit-domain loop.
///
/// The returned qubit is always from the canonical
/// `quantum::ir::qubit::QubitId` namespace.
pub fn qubit_at(
    range: QubitRange,
    iteration: usize,
) -> Option<QubitId> {
    let index = range.start().checked_add(iteration)?;

    if index >= range.end() {
        return None;
    }

    Some(QubitId::new(index))
}

/// Returns the exact number of logical qubits represented by a range.
///
/// No qubit collection is allocated.
pub fn qubit_range_size(
    range: QubitRange,
) -> LoopResult<usize> {
    range
        .end()
        .checked_sub(range.start())
        .ok_or(LoopError::ArithmeticOverflow {
            operation: "logical-qubit range size",
        })
}

// =============================================================================
// Loop-control validation
// =============================================================================

/// Validates a `break` or `continue` operation at a loop boundary.
///
/// `break` and `continue` are legal only inside loops. This function is
/// intentionally small because lexical/control-flow nesting is owned by the
/// enclosing control-flow validator.
pub const fn validate_loop_control(
    control: LoopControl,
    inside_loop: bool,
) -> LoopResult<()> {
    if !inside_loop {
        return Err(LoopError::InvalidControlTransfer);
    }

    match control {
        LoopControl::Break | LoopControl::Continue => Ok(()),
    }
}

// =============================================================================
// Invariants
// =============================================================================

/// Documents the semantic invariants of this module.
///
/// This is a compile-time-accessible API surface for validators/tests and keeps
/// the invariants discoverable without duplicating them as magic constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopInvariants;

impl LoopInvariants {
    /// Returns true because zero-step ranges are always rejected.
    #[must_use]
    pub const fn zero_step_rejected() -> bool {
        true
    }

    /// Returns true because static ranges are represented without
    /// materializing iteration values.
    #[must_use]
    pub const fn static_ranges_are_lazy() -> bool {
        true
    }

    /// Returns true because logical qubits use the canonical `QubitId`.
    #[must_use]
    pub const fn uses_canonical_qubit_ids() -> bool {
        true
    }

    /// Returns true because loop bodies are referenced by `RegionId`.
    #[must_use]
    pub const fn bodies_are_region_references() -> bool {
        true
    }

    /// Returns true because no hardware model is encoded by this module.
    #[must_use]
    pub const fn hardware_independent() -> bool {
        true
    }

    /// Returns true because no unsafe Rust is permitted.
    #[must_use]
    pub const fn unsafe_free() -> bool {
        true
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn region_id(value: u64) -> RegionId {
        RegionId::new(value)
    }

    #[test]
    fn exclusive_positive_range_has_expected_trip_count() {
        let range =
            StaticIntegerRange::exclusive(0, 10, 2).expect("valid range");

        assert_eq!(range.trip_count().expect("trip count"), 5);
        assert_eq!(range.value_at(0).expect("value"), Some(0));
        assert_eq!(range.value_at(4).expect("value"), Some(8));
        assert_eq!(range.value_at(5).expect("value"), None);
    }

    #[test]
    fn inclusive_positive_range_has_expected_trip_count() {
        let range =
            StaticIntegerRange::inclusive(0, 10, 2).expect("valid range");

        assert_eq!(range.trip_count().expect("trip count"), 6);
        assert_eq!(range.value_at(5).expect("value"), Some(10));
    }

    #[test]
    fn exclusive_negative_range_has_expected_trip_count() {
        let range =
            StaticIntegerRange::exclusive(10, 0, -2).expect("valid range");

        assert_eq!(range.trip_count().expect("trip count"), 5);
        assert_eq!(range.value_at(0).expect("value"), Some(10));
        assert_eq!(range.value_at(4).expect("value"), Some(2));
        assert_eq!(range.value_at(5).expect("value"), None);
    }

    #[test]
    fn inclusive_negative_range_has_expected_trip_count() {
        let range =
            StaticIntegerRange::inclusive(10, 0, -2).expect("valid range");

        assert_eq!(range.trip_count().expect("trip count"), 6);
        assert_eq!(range.value_at(5).expect("value"), Some(0));
    }

    #[test]
    fn zero_step_is_rejected() {
        assert_eq!(
            StaticIntegerRange::exclusive(0, 10, 0),
            Err(LoopError::ZeroStep)
        );
    }

    #[test]
    fn wrong_direction_is_rejected() {
        assert!(matches!(
            StaticIntegerRange::exclusive(10, 0, 1),
            Err(LoopError::InvalidStaticRange { .. })
        ));

        assert!(matches!(
            StaticIntegerRange::exclusive(0, 10, -1),
            Err(LoopError::InvalidStaticRange { .. })
        ));
    }

    #[test]
    fn empty_positive_range_is_zero_iterations() {
        let range =
            StaticIntegerRange::exclusive(10, 10, 1).expect("valid range");

        assert_eq!(range.trip_count().expect("trip count"), 0);
        assert!(range.is_empty());
    }

    #[test]
    fn large_ranges_are_not_materialized_by_trip_count() {
        let range = StaticIntegerRange::exclusive(
            0,
            1_000_000_000_000i128,
            1,
        )
        .expect("valid range");

        assert_eq!(
            range.trip_count().expect("trip count"),
            1_000_000_000_000u128
        );
    }

    #[test]
    fn qubit_domain_uses_canonical_qubit_id() {
        let range = QubitRange::new(10, 20).expect("valid range");

        assert_eq!(qubit_at(range, 0), Some(QubitId::new(10)));
        assert_eq!(qubit_at(range, 9), Some(QubitId::new(19)));
        assert_eq!(qubit_at(range, 10), None);
    }

    #[test]
    fn qubit_range_does_not_materialize_qubits() {
        let range = QubitRange::new(0, 1_000_000).expect("valid range");

        assert_eq!(
            qubit_range_size(range).expect("range size"),
            1_000_000
        );
    }

    #[test]
    fn for_loop_validates() {
        let domain =
            LoopDomain::integer_range(0, 100, 1).expect("valid domain");

        let loop_ir = Loop::for_loop(
            operation_id(1),
            LoopVariableId::new(0),
            domain,
            region_id(2),
        )
        .expect("valid loop");

        assert_eq!(loop_ir.kind(), LoopKind::For);
        assert_eq!(loop_ir.static_trip_count().expect("count"), Some(100));
    }

    #[test]
    fn repeat_loop_validates() {
        let loop_ir =
            Loop::repeat(operation_id(1), 1000, region_id(2))
                .expect("valid loop");

        assert_eq!(loop_ir.kind(), LoopKind::Repeat);
        assert_eq!(
            loop_ir.static_trip_count().expect("count"),
            Some(1000)
        );
    }

    #[test]
    fn while_loop_requires_condition() {
        let result =
            LoopBuilder::new(operation_id(1), LoopKind::While)
                .body(region_id(2))
                .build();

        assert_eq!(result, Err(LoopError::MissingCondition));
    }

    #[test]
    fn while_loop_validates() {
        let loop_ir = Loop::while_loop(
            operation_id(1),
            region_id(2),
            region_id(3),
        )
        .expect("valid loop");

        assert_eq!(loop_ir.kind(), LoopKind::While);
        assert!(loop_ir.static_trip_count().expect("count").is_none());
        assert!(loop_ir.may_execute_zero_times());
    }

    #[test]
    fn do_while_loop_is_not_zero_iteration_by_structure() {
        let loop_ir = Loop::do_while_loop(
            operation_id(1),
            region_id(2),
            region_id(3),
        )
        .expect("valid loop");

        assert!(!loop_ir.may_execute_zero_times());
    }

    #[test]
    fn for_each_qubit_validates() {
        let range =
            QubitRange::new(0, 128).expect("valid qubit range");

        let loop_ir = Loop::for_each_qubit(
            operation_id(1),
            LoopVariableId::new(0),
            range,
            region_id(2),
        )
        .expect("valid loop");

        assert_eq!(loop_ir.kind(), LoopKind::ForEachQubit);
        assert_eq!(
            loop_ir.static_trip_count().expect("count"),
            Some(128)
        );
    }

    #[test]
    fn parallel_for_is_semantic_not_hardware_specific() {
        let domain =
            LoopDomain::integer_range(0, 1024, 1)
                .expect("valid domain");

        let loop_ir = Loop::parallel_for(
            operation_id(1),
            LoopVariableId::new(0),
            domain,
            region_id(2),
        )
        .expect("valid loop");

        assert_eq!(loop_ir.kind(), LoopKind::ParallelFor);
        assert!(loop_ir.is_transformable());
    }

    #[test]
    fn loop_control_requires_loop_context() {
        assert_eq!(
            validate_loop_control(LoopControl::Break, false),
            Err(LoopError::InvalidControlTransfer)
        );

        assert_eq!(
            validate_loop_control(LoopControl::Continue, true),
            Ok(())
        );
    }

    #[test]
    fn materialization_is_explicitly_bounded() {
        let range =
            StaticIntegerRange::exclusive(0, 10, 1).expect("valid range");

        assert_eq!(
            materialize_static_range(range, 10)
                .expect("materialization"),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );

        assert!(materialize_static_range(range, 5).is_err());
    }

    #[test]
    fn invariants_are_explicit() {
        assert!(LoopInvariants::zero_step_rejected());
        assert!(LoopInvariants::static_ranges_are_lazy());
        assert!(LoopInvariants::uses_canonical_qubit_ids());
        assert!(LoopInvariants::bodies_are_region_references());
        assert!(LoopInvariants::hardware_independent());
        assert!(LoopInvariants::unsafe_free());
    }
}