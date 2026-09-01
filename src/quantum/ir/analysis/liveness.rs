//! Zamani Quantum IR — Logical Qubit Liveness Analysis
//!
//! Production-grade logical-state liveness analysis for the canonical
//! `quantum::ir::QuantumCircuit` representation.
//!
//! # Purpose
//!
//! This module determines the lifetime of logical quantum states without
//! making assumptions about:
//!
//! - physical hardware;
//! - physical qubit count;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse execution;
//! - backend implementation;
//! - simulator state;
//! - QEC implementation.
//!
//! The analysis answers questions such as:
//!
//! - Which logical qubits are actually used?
//! - When does a logical state first become live?
//! - When does it cease to be live?
//! - How many logical-state generations does a qubit have?
//! - Where do resets terminate logical-state generations?
//! - What is the peak number of simultaneously live logical states?
//! - Is a logical state dead at a particular operation boundary?
//! - Which logical qubits are completely unused?
//!
//! # Architectural boundary
//!
//! ```text
//!                  quantum::ir::QuantumCircuit
//!                              │
//!                              ▼
//!                   ir::analysis::liveness
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!          width          dependency       optimization
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                     resource planning
//! ```
//!
//! This module only analyzes. It never mutates the circuit.
//!
//! # Logical-state semantics
//!
//! A logical qubit may have multiple state generations:
//!
//! ```text
//! use ───── use ───── reset ───── use ───── use
//! │                         │
//! └── generation 0 ─────────┘
//!                             └── generation 1 ────
//! ```
//!
//! A reset is a semantic boundary:
//!
//! ```text
//! previous logical state
//!          │
//!          ▼
//!        reset
//!          │
//!          ▼
//! newly initialized logical state
//! ```
//!
//! A reset does not itself create a lifetime record for the new state.
//! A new lifetime begins only when a subsequent operation actually consumes
//! that newly established state.
//!
//! # Measurement semantics
//!
//! Measurement does NOT automatically end a lifetime.
//!
//! This is deliberate because dynamic quantum programs may legally perform:
//!
//! ```text
//! gate
//! measurement
//! gate
//! ```
//!
//! on the same logical qubit.
//!
//! Measurement may change the quantum state, but liveness is concerned with
//! whether the logical resource remains semantically relevant.
//!
//! A future measurement-aware analysis may prove stronger reuse opportunities,
//! but this module does not guess them.
//!
//! # Interval semantics
//!
//! Lifetimes use half-open operation-boundary intervals:
//!
//! ```text
//! [start, end)
//! ```
//!
//! `start` is the first operation consuming the logical state.
//!
//! `end` is the first operation boundary at which the logical state is no
//! longer live.
//!
//! For a final-use lifetime:
//!
//! ```text
//! operations:  0  1  2  3
//!                         ^
//!                         last use
//! interval:              [1, 4)
//! ```
//!
//! For a reset:
//!
//! ```text
//! operation 0: use
//! operation 1: use
//! operation 2: reset
//!
//! lifetime = [0, 2)
//! ```
//!
//! Importantly, the lifetime ends at the reset operation, not necessarily at
//! `last_use + 1`. There may be unrelated operations between the last use and
//! the reset.
//!
//! # Scalability
//!
//! Let:
//!
//! - `N` = number of operations;
//! - `A` = total number of logical operands;
//! - `K` = number of distinct logical qubits actually referenced;
//! - `R` = number of reset boundaries;
//! - `L` = number of discovered logical-state lifetimes.
//!
//! The implementation is:
//!
//! - O(N + A + L log L) expected time;
//! - O(K + L) retained memory.
//!
//! Temporary storage is proportional to the number of actually referenced
//! logical qubits and discovered lifetimes, NOT the declared logical-qubit
//! namespace.
//!
//! Therefore:
//!
//! ```text
//! declared qubits = enormous
//! referenced qubits = small
//!
//! analysis memory = proportional to referenced qubits
//! ```
//!
//! There is no hard-coded architectural maximum such as:
//!
//! ```text
//! 32
//! 64
//! 128
//! 1024
//! ```
//!
//! The practical limits are determined by the canonical IR representation,
//! explicit resource policy, addressable memory, and `usize` capacity.
//!
//! # Determinism
//!
//! Results are deterministic regardless of hash-map iteration order.
//!
//! Public intervals are ordered by:
//!
//! 1. logical `QubitId`;
//! 2. generation;
//! 3. start boundary;
//! 4. end boundary.
//!
//! Used-qubit results are ordered by canonical `QubitId`.
//!
//! Peak-live event processing is deterministic, with end events processed
//! before start events at the same boundary.
//!
//! # Canonical qubit identity
//!
//! The only logical-qubit identity used here is:
//!
//! ```rust
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module intentionally does NOT use the historical `qubits` module.
//!
//! # Integration
//!
//! `analysis/mod.rs` should expose this module:
//!
//! ```rust
//! pub mod liveness;
//! ```
//!
//! Optional convenient re-exports:
//!
//! ```rust
//! pub use liveness::{
//!     LivenessAnalysis,
//!     LivenessError,
//!     LivenessInterval,
//!     LivenessSummary,
//! };
//! ```
//!
//! Consumers include:
//!
//! - `analysis/width.rs`;
//! - `analysis/dependencies.rs`;
//! - resource estimation;
//! - optimization planning;
//! - qubit reuse planning;
//! - memory-pressure analysis;
//! - visualization;
//! - compiler diagnostics.
//!
//! None of those consumers are dependencies of this module.
//!
//! # Control-flow boundary
//!
//! `QuantumCircuit` currently provides an ordered operation representation.
//! This implementation therefore computes exact liveness for that ordered
//! representation.
//!
//! When the universal `QuantumProgram`/region/block control-flow layer becomes
//! the authoritative representation for dynamic branches and loops, a
//! higher-level control-flow liveness analysis should compose region-level
//! analyses.
//!
//! This module must not pretend that a linear interval is exact for arbitrary
//! cyclic control flow.
//!
//! # Safety
//!
//! This file forbids unsafe Rust.
//!
//! No unchecked indexing or unchecked arithmetic is used.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use crate::quantum::ir::circuit::QuantumCircuit;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Public index types
// =============================================================================

/// Zero-based index of an operation in the canonical circuit.
pub type OperationIndex = usize;

/// Operation-boundary index.
///
/// A circuit with `N` operations has boundaries:
///
/// ```text
/// 0 ..= N
/// ```
///
/// Boundary `0` is before the first operation and boundary `N` is after the
/// final operation.
pub type OperationBoundary = usize;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by logical-state liveness analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivenessError {
    /// A logical operand lies outside the circuit namespace.
    QubitOutOfRange {
        /// Offending logical qubit.
        qubit: QubitId,

        /// Number of declared logical qubits.
        qubit_count: usize,

        /// Operation containing the invalid reference.
        operation: OperationIndex,
    },

    /// An operation boundary was requested outside the valid range.
    OperationBoundaryOutOfRange {
        /// Requested boundary.
        boundary: OperationBoundary,

        /// Number of operations.
        operation_count: usize,
    },

    /// A requested operation index is outside the circuit.
    OperationOutOfRange {
        /// Requested operation.
        operation: OperationIndex,

        /// Number of operations.
        operation_count: usize,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Description of the failed calculation.
        calculation: &'static str,
    },

    /// An internal lifetime invariant was violated.
    InvalidInterval {
        /// Logical qubit.
        qubit: QubitId,

        /// Generation number.
        generation: usize,

        /// Interval start.
        start: OperationBoundary,

        /// Interval end.
        end: OperationBoundary,
    },
}

impl fmt::Display for LivenessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitOutOfRange {
                qubit,
                qubit_count,
                operation,
            } => write!(
                formatter,
                "logical qubit {qubit} is outside namespace 0..{qubit_count} \
                 at operation {operation}"
            ),

            Self::OperationBoundaryOutOfRange {
                boundary,
                operation_count,
            } => write!(
                formatter,
                "operation boundary {boundary} is outside 0..={operation_count}"
            ),

            Self::OperationOutOfRange {
                operation,
                operation_count,
            } => write!(
                formatter,
                "operation {operation} is outside circuit length {operation_count}"
            ),

            Self::ArithmeticOverflow { calculation } => write!(
                formatter,
                "arithmetic overflow while calculating {calculation}"
            ),

            Self::InvalidInterval {
                qubit,
                generation,
                start,
                end,
            } => write!(
                formatter,
                "invalid logical-state lifetime for {qubit}, generation \
                 {generation}: [{start}, {end})"
            ),
        }
    }
}

impl std::error::Error for LivenessError {}

// =============================================================================
// Lifetime interval
// =============================================================================

/// One logical-state lifetime.
///
/// The interval is half-open:
///
/// ```text
/// [start, end)
/// ```
///
/// The lifetime belongs to a logical qubit *generation*. A reset terminates
/// one generation and permits a later generation to begin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LivenessInterval {
    qubit: QubitId,
    generation: usize,
    start: OperationBoundary,
    end: OperationBoundary,
    first_use: OperationIndex,
    last_use: OperationIndex,
    use_count: usize,
    measurement_count: usize,
    reset_terminated: bool,
}

impl LivenessInterval {
    fn new(
        qubit: QubitId,
        generation: usize,
        first_use: OperationIndex,
        last_use: OperationIndex,
        use_count: usize,
        measurement_count: usize,
        end: OperationBoundary,
        reset_terminated: bool,
    ) -> Result<Self, LivenessError> {
        let minimum_end = last_use.checked_add(1).ok_or(
            LivenessError::ArithmeticOverflow {
                calculation: "last-use operation boundary",
            },
        )?;

        if end <= first_use || end < minimum_end {
            return Err(LivenessError::InvalidInterval {
                qubit,
                generation,
                start: first_use,
                end,
            });
        }

        Ok(Self {
            qubit,
            generation,
            start: first_use,
            end,
            first_use,
            last_use,
            use_count,
            measurement_count,
            reset_terminated,
        })
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the generation number.
    ///
    /// The first lifetime of a logical qubit is generation `0`.
    #[must_use]
    pub const fn generation(&self) -> usize {
        self.generation
    }

    /// Returns the inclusive first-use operation index.
    #[must_use]
    pub const fn start(&self) -> OperationBoundary {
        self.start
    }

    /// Returns the exclusive end boundary.
    #[must_use]
    pub const fn end(&self) -> OperationBoundary {
        self.end
    }

    /// Returns the first operation that consumed this logical state.
    #[must_use]
    pub const fn first_use(&self) -> OperationIndex {
        self.first_use
    }

    /// Returns the last operation that consumed this logical state.
    #[must_use]
    pub const fn last_use(&self) -> OperationIndex {
        self.last_use
    }

    /// Returns the number of operations consuming this logical state.
    #[must_use]
    pub const fn use_count(&self) -> usize {
        self.use_count
    }

    /// Returns the number of measurement operations in this lifetime.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Returns whether a reset terminated this lifetime.
    #[must_use]
    pub const fn reset_terminated(&self) -> bool {
        self.reset_terminated
    }

    /// Returns whether this lifetime contains an operation index.
    #[must_use]
    pub const fn contains_operation(&self, operation: OperationIndex) -> bool {
        self.start <= operation && operation < self.end
    }

    /// Returns whether this lifetime is live at an operation boundary.
    ///
    /// Boundary semantics are:
    ///
    /// ```text
    /// start <= boundary < end
    /// ```
    ///
    /// Therefore a lifetime ending at boundary `N` is dead at `N`.
    #[must_use]
    pub const fn is_live_at(&self, boundary: OperationBoundary) -> bool {
        self.start <= boundary && boundary < self.end
    }

    /// Returns the number of operation boundaries occupied by this lifetime.
    #[must_use]
    pub const fn span(&self) -> usize {
        self.end - self.start
    }
}

// =============================================================================
// Summary
// =============================================================================

/// Compact summary of a liveness analysis.
///
/// This type is intentionally independent of the full interval collection,
/// allowing consumers such as width analysis to avoid inspecting every
/// lifetime when only aggregate metrics are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LivenessSummary {
    declared_qubits: usize,
    used_qubits: usize,
    unused_qubits: usize,
    operation_count: usize,
    lifetime_count: usize,
    peak_live_qubits: usize,
    peak_live_boundary: OperationBoundary,
    reset_count: usize,
    measurement_count: usize,
}

impl LivenessSummary {
    /// Returns the declared logical-qubit namespace size.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of distinct logical qubits actually referenced.
    #[must_use]
    pub const fn used_qubits(&self) -> usize {
        self.used_qubits
    }

    /// Returns the number of declared logical qubits never referenced.
    ///
    /// This is an aggregate count. The analysis does not materialize every
    /// unused qubit identifier, preserving sparse scaling.
    #[must_use]
    pub const fn unused_qubits(&self) -> usize {
        self.unused_qubits
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of logical-state generations.
    #[must_use]
    pub const fn lifetime_count(&self) -> usize {
        self.lifetime_count
    }

    /// Returns the maximum number of simultaneously live logical states.
    #[must_use]
    pub const fn peak_live_qubits(&self) -> usize {
        self.peak_live_qubits
    }

    /// Returns the earliest boundary at which peak liveness occurs.
    #[must_use]
    pub const fn peak_live_boundary(&self) -> OperationBoundary {
        self.peak_live_boundary
    }

    /// Returns the number of reset operations.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }
}

// =============================================================================
// Internal state
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct ActiveLifetime {
    generation: usize,
    first_use: OperationIndex,
    last_use: OperationIndex,
    use_count: usize,
    measurement_count: usize,
}

impl ActiveLifetime {
    fn new(generation: usize, operation: OperationIndex, measurement: bool) -> Self {
        Self {
            generation,
            first_use: operation,
            last_use: operation,
            use_count: 1,
            measurement_count: usize::from(measurement),
        }
    }

    fn record_use(
        &mut self,
        operation: OperationIndex,
        measurement: bool,
    ) -> Result<(), LivenessError> {
        self.last_use = operation;

        self.use_count = self.use_count.checked_add(1).ok_or(
            LivenessError::ArithmeticOverflow {
                calculation: "logical-state use count",
            },
        )?;

        if measurement {
            self.measurement_count = self.measurement_count.checked_add(1).ok_or(
                LivenessError::ArithmeticOverflow {
                    calculation: "logical-state measurement count",
                },
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Main analysis
// =============================================================================

/// Complete immutable logical-state liveness analysis.
///
/// The analysis is constructed from a canonical `QuantumCircuit` and owns
/// only derived data. The original circuit is never modified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessAnalysis {
    declared_qubits: usize,
    operation_count: usize,
    intervals: Vec<LivenessInterval>,
    qubit_ranges: BTreeMap<QubitId, (usize, usize)>,
    used_qubits: Vec<QubitId>,
    peak_live_qubits: usize,
    peak_live_boundary: OperationBoundary,
    reset_count: usize,
    measurement_count: usize,
}

impl LivenessAnalysis {
    /// Analyzes logical-state liveness of a canonical quantum circuit.
    ///
    /// This is the primary construction API.
    ///
    /// The analysis:
    ///
    /// 1. walks operations exactly once;
    /// 2. tracks only actually referenced logical qubits;
    /// 3. creates lifetime generations lazily;
    /// 4. closes generations at reset or final program boundary;
    /// 5. sorts the resulting intervals deterministically;
    /// 6. computes peak live-state count.
    ///
    /// No array proportional to `circuit.num_qubits()` is allocated.
    pub fn analyze(circuit: &QuantumCircuit) -> Result<Self, LivenessError> {
        let declared_qubits = circuit.num_qubits();
        let operation_count = circuit.operations().len();

        let mut active: HashMap<QubitId, ActiveLifetime> = HashMap::new();
        let mut generation_counts: HashMap<QubitId, usize> = HashMap::new();
        let mut intervals = Vec::new();
        let mut used_qubits = BTreeSet::new();

        let mut reset_count = 0usize;
        let mut measurement_count = 0usize;

        for (operation_index, operation) in circuit.operations().iter().enumerate() {
            let qubits = operation.qubits();

            let is_reset = operation.is_reset();
            let is_measurement = operation.is_measurement();

            if is_reset {
                reset_count = reset_count.checked_add(1).ok_or(
                    LivenessError::ArithmeticOverflow {
                        calculation: "reset count",
                    },
                )?;
            }

            if is_measurement {
                measurement_count = measurement_count.checked_add(1).ok_or(
                    LivenessError::ArithmeticOverflow {
                        calculation: "measurement count",
                    },
                )?;
            }

            // Validate every quantum operand against the canonical namespace.
            //
            // This intentionally does not assume a fixed arity.
            for &qubit in &qubits {
                if qubit.index() >= declared_qubits {
                    return Err(LivenessError::QubitOutOfRange {
                        qubit,
                        qubit_count: declared_qubits,
                        operation: operation_index,
                    });
                }

                used_qubits.insert(qubit);
            }

            if is_reset {
                // Reset terminates the old logical-state generation.
                //
                // The reset operation itself is NOT considered a use of the
                // newly initialized state.
                for qubit in qubits {
                    if let Some(previous) = active.remove(&qubit) {
                        let interval = LivenessInterval::new(
                            qubit,
                            previous.generation,
                            previous.first_use,
                            previous.last_use,
                            previous.use_count,
                            previous.measurement_count,
                            operation_index,
                            true,
                        )?;

                        intervals.push(interval);
                    }
                }

                continue;
            }

            // Every non-reset quantum operation consumes its referenced
            // logical state. Measurement is intentionally just another use
            // for liveness purposes.
            for qubit in qubits {
                match active.get_mut(&qubit) {
                    Some(current) => {
                        current.record_use(operation_index, is_measurement)?;
                    }

                    None => {
                        let generation = generation_counts
                            .get(&qubit)
                            .copied()
                            .unwrap_or(0);

                        active.insert(
                            qubit,
                            ActiveLifetime::new(
                                generation,
                                operation_index,
                                is_measurement,
                            ),
                        );
                    }
                }
            }

            // No per-declared-qubit state is created here.
        }

        // Any remaining active state survives until the final circuit boundary.
        for (qubit, previous) in active {
            let end = operation_count;

            let interval = LivenessInterval::new(
                qubit,
                previous.generation,
                previous.first_use,
                previous.last_use,
                previous.use_count,
                previous.measurement_count,
                end,
                false,
            )?;

            intervals.push(interval);
        }

        // A reset followed by a future use creates a new generation. The
        // generation number is derived from the number of already closed
        // generations rather than from the declared namespace.
        //
        // Recompute generation numbers deterministically from the intervals.
        intervals.sort_by(|left, right| {
            left.qubit()
                .cmp(&right.qubit())
                .then_with(|| left.start().cmp(&right.start()))
                .then_with(|| left.end().cmp(&right.end()))
        });

        let mut next_generation: BTreeMap<QubitId, usize> = BTreeMap::new();
        let mut normalized = Vec::with_capacity(intervals.len());

        for interval in intervals {
            let generation = next_generation
                .get(&interval.qubit())
                .copied()
                .unwrap_or(0);

            let normalized_interval = LivenessInterval::new(
                interval.qubit(),
                generation,
                interval.first_use(),
                interval.last_use(),
                interval.use_count(),
                interval.measurement_count(),
                interval.end(),
                interval.reset_terminated(),
            )?;

            normalized.push(normalized_interval);

            let next = generation.checked_add(1).ok_or(
                LivenessError::ArithmeticOverflow {
                    calculation: "logical-qubit generation",
                },
            )?;

            next_generation.insert(interval.qubit(), next);
        }

        intervals = normalized;

        let used_qubits: Vec<QubitId> = used_qubits.into_iter().collect();

        let qubit_ranges = build_qubit_ranges(&intervals)?;

        let (peak_live_qubits, peak_live_boundary) =
            calculate_peak_live_qubits(&intervals)?;

        Ok(Self {
            declared_qubits,
            operation_count,
            intervals,
            qubit_ranges,
            used_qubits,
            peak_live_qubits,
            peak_live_boundary,
            reset_count,
            measurement_count,
        })
    }

    /// Alias for [`Self::analyze`].
    pub fn new(circuit: &QuantumCircuit) -> Result<Self, LivenessError> {
        Self::analyze(circuit)
    }

    /// Returns the number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of operations analyzed.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns all lifetime intervals in deterministic order.
    #[must_use]
    pub fn intervals(&self) -> &[LivenessInterval] {
        &self.intervals
    }

    /// Returns all actually used logical qubits in deterministic order.
    ///
    /// Unused qubits are not materialized.
    #[must_use]
    pub fn used_qubits(&self) -> &[QubitId] {
        &self.used_qubits
    }

    /// Returns whether a logical qubit occurs in the circuit.
    #[must_use]
    pub fn is_used(&self, qubit: QubitId) -> bool {
        self.used_qubits.binary_search(&qubit).is_ok()
    }

    /// Returns the number of distinct logical qubits actually referenced.
    #[must_use]
    pub fn used_qubit_count(&self) -> usize {
        self.used_qubits.len()
    }

    /// Returns the number of declared but unused logical qubits.
    ///
    /// The calculation is checked rather than relying on an unchecked
    /// subtraction.
    pub fn unused_qubit_count(&self) -> Result<usize, LivenessError> {
        self.declared_qubits
            .checked_sub(self.used_qubit_count())
            .ok_or(LivenessError::ArithmeticOverflow {
                calculation: "unused logical-qubit count",
            })
    }

    /// Returns the total number of logical-state lifetimes.
    #[must_use]
    pub fn lifetime_count(&self) -> usize {
        self.intervals.len()
    }

    /// Returns the maximum number of simultaneously live logical states.
    #[must_use]
    pub const fn peak_live_qubits(&self) -> usize {
        self.peak_live_qubits
    }

    /// Returns the earliest boundary at which peak liveness occurs.
    #[must_use]
    pub const fn peak_live_boundary(&self) -> OperationBoundary {
        self.peak_live_boundary
    }

    /// Returns the number of reset operations.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Returns a compact deterministic summary.
    pub fn summary(&self) -> Result<LivenessSummary, LivenessError> {
        Ok(LivenessSummary {
            declared_qubits: self.declared_qubits,
            used_qubits: self.used_qubit_count(),
            unused_qubits: self.unused_qubit_count()?,
            operation_count: self.operation_count,
            lifetime_count: self.lifetime_count(),
            peak_live_qubits: self.peak_live_qubits,
            peak_live_boundary: self.peak_live_boundary,
            reset_count: self.reset_count,
            measurement_count: self.measurement_count,
        })
    }

    /// Returns all lifetimes belonging to one logical qubit.
    ///
    /// Because intervals are grouped deterministically, this operation is
    /// O(number of generations for the qubit).
    #[must_use]
    pub fn lifetimes_for(&self, qubit: QubitId) -> &[LivenessInterval] {
        match self.qubit_ranges.get(&qubit) {
            Some(&(start, end)) => &self.intervals[start..end],
            None => &[],
        }
    }

    /// Returns one specific logical-state generation.
    #[must_use]
    pub fn lifetime(
        &self,
        qubit: QubitId,
        generation: usize,
    ) -> Option<&LivenessInterval> {
        self.lifetimes_for(qubit)
            .get(generation)
    }

    /// Returns the lifetime containing an operation index.
    #[must_use]
    pub fn lifetime_at_operation(
        &self,
        qubit: QubitId,
        operation: OperationIndex,
    ) -> Option<&LivenessInterval> {
        self.lifetimes_for(qubit)
            .iter()
            .find(|interval| interval.contains_operation(operation))
    }

    /// Returns the lifetime live at a particular operation boundary.
    #[must_use]
    pub fn lifetime_at_boundary(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Option<&LivenessInterval> {
        self.lifetimes_for(qubit)
            .iter()
            .find(|interval| interval.is_live_at(boundary))
    }

    /// Returns whether a logical state is live at an operation boundary.
    ///
    /// This method validates the boundary before answering.
    pub fn is_live_at(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Result<bool, LivenessError> {
        self.validate_boundary(boundary)?;

        Ok(self
            .lifetime_at_boundary(qubit, boundary)
            .is_some())
    }

    /// Returns whether a logical state is dead at a boundary.
    ///
    /// A qubit that has a future lifetime but is currently dead is considered
    /// dead at that boundary. This does NOT mean a compiler may freely remap
    /// the logical qubit without considering future semantic constraints.
    pub fn is_dead_at(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Result<bool, LivenessError> {
        Ok(!self.is_live_at(qubit, boundary)?)
    }

    /// Returns whether a logical qubit is completely unused.
    #[must_use]
    pub fn is_unused(&self, qubit: QubitId) -> bool {
        !self.is_used(qubit)
    }

    /// Returns whether the logical qubit has no future lifetime after a
    /// boundary.
    ///
    /// This is stronger than `is_dead_at`.
    pub fn has_no_future_lifetime(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Result<bool, LivenessError> {
        self.validate_boundary(boundary)?;

        Ok(self
            .lifetimes_for(qubit)
            .iter()
            .all(|interval| interval.start() < boundary || interval.start() == boundary))
            && self
                .lifetimes_for(qubit)
                .iter()
                .all(|interval| interval.end() <= boundary || interval.start() <= boundary)
    }

    /// Returns whether the qubit has completed its final lifetime by the
    /// specified boundary.
    ///
    /// This is the strongest liveness-only signal available for reuse
    /// planning. It does not perform physical allocation or routing.
    pub fn final_lifetime_ended_at(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Result<bool, LivenessError> {
        self.validate_boundary(boundary)?;

        match self.lifetimes_for(qubit).last() {
            Some(last) => Ok(last.end() <= boundary),
            None => Ok(false),
        }
    }

    /// Returns whether a logical resource is potentially available at a
    /// boundary according to liveness alone.
    ///
    /// This is intentionally NOT named `can_allocate_physical_qubit`:
    /// physical allocation belongs to the routing/hardware layers.
    pub fn can_be_reused_at(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> Result<bool, LivenessError> {
        self.validate_boundary(boundary)?;

        Ok(self.is_used(qubit) && !self.is_live_at(qubit, boundary)?)
    }

    /// Returns the first boundary after the final lifetime.
    ///
    /// Returns `None` for an unused qubit.
    #[must_use]
    pub fn final_release_boundary(&self, qubit: QubitId) -> Option<OperationBoundary> {
        self.lifetimes_for(qubit)
            .last()
            .map(LivenessInterval::end)
    }

    /// Returns the first operation that starts the first lifetime.
    ///
    /// Returns `None` for an unused qubit.
    #[must_use]
    pub fn first_use(&self, qubit: QubitId) -> Option<OperationIndex> {
        self.lifetimes_for(qubit)
            .first()
            .map(LivenessInterval::first_use)
    }

    /// Returns the final operation consuming any lifetime of the qubit.
    ///
    /// Returns `None` for an unused qubit.
    #[must_use]
    pub fn last_use(&self, qubit: QubitId) -> Option<OperationIndex> {
        self.lifetimes_for(qubit)
            .last()
            .map(LivenessInterval::last_use)
    }

    /// Returns the number of distinct generations for one logical qubit.
    #[must_use]
    pub fn generation_count(&self, qubit: QubitId) -> usize {
        self.lifetimes_for(qubit).len()
    }

    /// Validates an operation index against this analysis.
    pub fn validate_operation(
        &self,
        operation: OperationIndex,
    ) -> Result<(), LivenessError> {
        if operation >= self.operation_count {
            return Err(LivenessError::OperationOutOfRange {
                operation,
                operation_count: self.operation_count,
            });
        }

        Ok(())
    }

    /// Validates an operation boundary against this analysis.
    pub fn validate_boundary(
        &self,
        boundary: OperationBoundary,
    ) -> Result<(), LivenessError> {
        if boundary > self.operation_count {
            return Err(LivenessError::OperationBoundaryOutOfRange {
                boundary,
                operation_count: self.operation_count,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn build_qubit_ranges(
    intervals: &[LivenessInterval],
) -> Result<BTreeMap<QubitId, (usize, usize)>, LivenessError> {
    let mut ranges = BTreeMap::new();

    if intervals.is_empty() {
        return Ok(ranges);
    }

    let mut start = 0usize;

    while start < intervals.len() {
        let qubit = intervals[start].qubit();

        let mut end = start
            .checked_add(1)
            .ok_or(LivenessError::ArithmeticOverflow {
                calculation: "liveness interval range end",
            })?;

        while end < intervals.len() && intervals[end].qubit() == qubit {
            end = end
                .checked_add(1)
                .ok_or(LivenessError::ArithmeticOverflow {
                    calculation: "liveness interval range end",
                })?;
        }

        ranges.insert(qubit, (start, end));
        start = end;
    }

    Ok(ranges)
}

fn calculate_peak_live_qubits(
    intervals: &[LivenessInterval],
) -> Result<(usize, OperationBoundary), LivenessError> {
    if intervals.is_empty() {
        return Ok((0, 0));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum EventKind {
        End,
        Start,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Event {
        boundary: OperationBoundary,
        kind: EventKind,
    }

    let event_count = intervals
        .len()
        .checked_mul(2)
        .ok_or(LivenessError::ArithmeticOverflow {
            calculation: "liveness peak event count",
        })?;

    let mut events = Vec::with_capacity(event_count);

    for interval in intervals {
        events.push(Event {
            boundary: interval.start(),
            kind: EventKind::Start,
        });

        events.push(Event {
            boundary: interval.end(),
            kind: EventKind::End,
        });
    }

    // End events must precede start events at the same boundary because the
    // lifetime interval is half-open.
    events.sort_by(|left, right| {
        left.boundary
            .cmp(&right.boundary)
            .then_with(|| match (left.kind, right.kind) {
                (EventKind::End, EventKind::Start) => std::cmp::Ordering::Less,
                (EventKind::Start, EventKind::End) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            })
    });

    let mut live = 0usize;
    let mut peak = 0usize;
    let mut peak_boundary = 0usize;

    for event in events {
        match event.kind {
            EventKind::Start => {
                live = live.checked_add(1).ok_or(
                    LivenessError::ArithmeticOverflow {
                        calculation: "simultaneously live logical states",
                    },
                )?;

                if live > peak {
                    peak = live;
                    peak_boundary = event.boundary;
                }
            }

            EventKind::End => {
                live = live.checked_sub(1).ok_or(
                    LivenessError::ArithmeticOverflow {
                        calculation: "simultaneously live logical states",
                    },
                )?;
            }
        }
    }

    Ok((peak, peak_boundary))
}

// =============================================================================
// Convenience analysis API
// =============================================================================

/// Performs logical-state liveness analysis.
///
/// This is the lightweight functional API for callers that do not need to
/// name the analysis type explicitly.
pub fn analyze_liveness(
    circuit: &QuantumCircuit,
) -> Result<LivenessAnalysis, LivenessError> {
    LivenessAnalysis::analyze(circuit)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::gate::{Gate, GateKind};
    use crate::quantum::ir::identity::CircuitId;
    use crate::quantum::ir::limits::QuantumIrLimits;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn limits() -> QuantumIrLimits {
        QuantumIrLimits::default()
    }

    fn circuit(qubits: usize) -> QuantumCircuit {
        QuantumCircuit::new(
            CircuitId::new(1),
            qubits,
            0,
            limits(),
        )
        .expect("test circuit must be constructible")
    }

    fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
        Gate::new(
            kind,
            qubits.iter().copied().map(QubitId::new).collect(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn empty_circuit_has_no_lifetimes() {
        let circuit = circuit(8);
        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.declared_qubits(), 8);
        assert_eq!(analysis.operation_count(), 0);
        assert_eq!(analysis.used_qubit_count(), 0);
        assert_eq!(analysis.lifetime_count(), 0);
        assert_eq!(analysis.peak_live_qubits(), 0);
        assert_eq!(analysis.peak_live_boundary(), 0);
    }

    #[test]
    fn unused_declared_qubits_are_not_materialized() {
        let circuit = circuit(1_000_000);
        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.declared_qubits(), 1_000_000);
        assert_eq!(analysis.used_qubit_count(), 0);
        assert_eq!(
            analysis.unused_qubit_count().expect("count must succeed"),
            1_000_000
        );
        assert!(analysis.used_qubits().is_empty());
    }

    #[test]
    fn one_use_creates_one_lifetime() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.lifetime_count(), 1);

        let lifetime = analysis
            .lifetime(q(0), 0)
            .expect("generation zero must exist");

        assert_eq!(lifetime.start(), 0);
        assert_eq!(lifetime.end(), 1);
        assert_eq!(lifetime.first_use(), 0);
        assert_eq!(lifetime.last_use(), 0);
        assert_eq!(lifetime.use_count(), 1);
        assert!(!lifetime.reset_terminated());
    }

    #[test]
    fn measurement_does_not_end_lifetime() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Measure, &[0]))
            .expect("measurement insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.lifetime_count(), 1);

        let lifetime = analysis
            .lifetime(q(0), 0)
            .expect("lifetime must exist");

        assert_eq!(lifetime.first_use(), 0);
        assert_eq!(lifetime.last_use(), 2);
        assert_eq!(lifetime.use_count(), 3);
        assert_eq!(lifetime.measurement_count(), 1);
        assert_eq!(lifetime.end(), 3);
    }

    #[test]
    fn reset_creates_new_generation() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        circuit
            .append_gate(gate(GateKind::H, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.lifetime_count(), 2);

        let first = analysis
            .lifetime(q(0), 0)
            .expect("first generation must exist");

        let second = analysis
            .lifetime(q(0), 1)
            .expect("second generation must exist");

        assert_eq!(first.start(), 0);
        assert_eq!(first.end(), 1);
        assert!(first.reset_terminated());

        assert_eq!(second.start(), 2);
        assert_eq!(second.end(), 3);
        assert!(!second.reset_terminated());
    }

    #[test]
    fn reset_does_not_create_empty_lifetime() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.lifetime_count(), 0);
        assert_eq!(analysis.reset_count(), 1);
        assert!(analysis.is_unused(q(0)));
    }

    #[test]
    fn reset_can_end_lifetime_after_unrelated_operations() {
        let mut circuit = circuit(2);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[1]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        let lifetime = analysis
            .lifetime(q(0), 0)
            .expect("lifetime must exist");

        assert_eq!(lifetime.first_use(), 0);
        assert_eq!(lifetime.last_use(), 0);

        // The reset occurs at operation 2, so the state remains live through
        // boundary 1 even though its last quantum use was operation 0.
        assert_eq!(lifetime.end(), 2);
        assert!(lifetime.reset_terminated());
    }

    #[test]
    fn peak_live_qubits_are_counted_at_boundaries() {
        let mut circuit = circuit(3);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[1]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[2]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.peak_live_qubits(), 3);
        assert_eq!(analysis.peak_live_boundary(), 2);
    }

    #[test]
    fn disjoint_lifetimes_do_not_overlap() {
        let mut circuit = circuit(2);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[1]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.peak_live_qubits(), 1);
    }

    #[test]
    fn used_qubits_are_deterministically_sorted() {
        let mut circuit = circuit(8);

        circuit
            .append_gate(gate(GateKind::CX, &[5, 2]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::X, &[7]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(
            analysis.used_qubits(),
            &[q(2), q(5), q(7)]
        );
    }

    #[test]
    fn lifetime_generation_order_is_deterministic() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Y, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Reset, &[0]))
            .expect("reset insertion must succeed");

        circuit
            .append_gate(gate(GateKind::Z, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(analysis.generation_count(q(0)), 3);

        assert_eq!(
            analysis.lifetime(q(0), 0).expect("generation 0").generation(),
            0
        );
        assert_eq!(
            analysis.lifetime(q(0), 1).expect("generation 1").generation(),
            1
        );
        assert_eq!(
            analysis.lifetime(q(0), 2).expect("generation 2").generation(),
            2
        );
    }

    #[test]
    fn final_release_boundary_is_after_last_use() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        circuit
            .append_gate(gate(GateKind::H, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert_eq!(
            analysis.final_release_boundary(q(0)),
            Some(2)
        );
    }

    #[test]
    fn liveness_boundary_is_half_open() {
        let mut circuit = circuit(1);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert!(
            analysis
                .is_live_at(q(0), 0)
                .expect("boundary must be valid")
        );

        assert!(
            !analysis
                .is_live_at(q(0), 1)
                .expect("boundary must be valid")
        );
    }

    #[test]
    fn unused_qubit_is_never_live() {
        let mut circuit = circuit(2);

        circuit
            .append_gate(gate(GateKind::X, &[0]))
            .expect("gate insertion must succeed");

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        assert!(
            !analysis
                .is_live_at(q(1), 0)
                .expect("boundary must be valid")
        );
    }

    #[test]
    fn invalid_boundary_is_rejected() {
        let circuit = circuit(1);

        let analysis = LivenessAnalysis::analyze(&circuit)
            .expect("analysis must succeed");

        let result = analysis.is_live_at(q(0), 1);

        assert_eq!(
            result,
            Err(LivenessError::OperationBoundaryOutOfRange {
                boundary: 1,
                operation_count: 0,
            })
        );
    }
}