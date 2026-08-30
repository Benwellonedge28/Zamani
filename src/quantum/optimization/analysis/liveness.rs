//! Zamani Quantum Optimization — Logical Qubit Liveness Analysis
//!
//! Production-grade logical-state liveness analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                   │
//!                                   ▼
//!                         analysis::liveness
//!                                   │
//!                 ┌─────────────────┼─────────────────┐
//!                 ▼                 ▼                 ▼
//!             dependency          width          optimization
//!                 │                 │                 │
//!                 └─────────────────┼─────────────────┘
//!                                   ▼
//!                              routing / reuse
//! ```
//!
//! This module answers logical-state lifetime questions such as:
//!
//! - When does a logical qubit first become live?
//! - When does its current logical state cease to be live?
//! - How many distinct logical lifetimes does a qubit have?
//! - Where do `reset` operations create new logical-state generations?
//! - What is the peak number of simultaneously live logical states?
//! - Which qubits are dead before the circuit starts?
//! - Which qubits are dead after their final use?
//! - Which qubits can potentially be considered for reuse?
//! - What are the lifetime intervals of each logical qubit?
//!
//! # Critical semantic distinction
//!
//! This module analyzes **logical-state liveness**, not physical hardware
//! occupancy.
//!
//! A qubit being live means that its current logical state may still matter to
//! subsequent quantum computation.
//!
//! This is deliberately different from:
//!
//! - physical qubit allocation;
//! - physical qubit topology;
//! - routing;
//! - pulse scheduling;
//! - hardware reset duration;
//! - calibration;
//! - QPU execution.
//!
//! Those concerns belong to downstream quantum subsystems.
//!
//! # Reset semantics
//!
//! `Reset` is treated as a logical-state lifetime boundary.
//!
//! Conceptually:
//!
//! ```text
//! old logical state
//!       │
//!       ▼
//!     reset
//!       │
//!       ▼
//! new |0> logical state
//! ```
//!
//! Therefore a reset:
//!
//! 1. terminates the preceding logical-state lifetime immediately before the
//!    reset operation;
//! 2. establishes a new logical state after the reset;
//! 3. does not automatically create a lifetime record if no later operation
//!    consumes the newly initialized state.
//!
//! This distinction is important for qubit reuse compilation.
//!
//! # Measurement semantics
//!
//! Measurement does **not** automatically terminate a lifetime.
//!
//! A measured qubit can legally participate in later operations in a dynamic
//! quantum circuit. Measurement collapses/observes the state, but the qubit
//! remains a valid logical resource until a semantic boundary such as reset or
//! final use.
//!
//! Therefore:
//!
//! ```text
//! gate → measure → gate
//! ```
//!
//! remains one logical lifetime.
//!
//! A later optimization pass may introduce a stronger measurement-aware
//! release policy when the surrounding program semantics prove that the
//! measured state and all dependent classical information are no longer
//! needed.
//!
//! That stronger analysis is intentionally NOT guessed here.
//!
//! # Lifetime representation
//!
//! A lifetime is represented as a half-open operation interval:
//!
//! ```text
//! [start, end)
//! ```
//!
//! where:
//!
//! - `start` is the first operation that consumes the logical state;
//! - `end` is the first operation boundary at which that logical state is no
//!   longer live.
//!
//! For a lifetime whose last use is operation `N`, the interval is:
//!
//! ```text
//! [start, N + 1)
//! ```
//!
//! A reset at operation `R` terminates the previous lifetime at `R`:
//!
//! ```text
//! [start, R)
//! ```
//!
//! If the qubit is subsequently used at operation `U`, the new lifetime is:
//!
//! ```text
//! [U, ...)
//! ```
//!
//! This representation makes interval overlap and peak-live analysis
//! unambiguous and avoids inclusive-endpoint off-by-one errors.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of circuit operations;
//! - `A` = total number of logical-qubit operands;
//! - `K` = number of distinct logical qubits used;
//! - `R` = number of reset operations;
//! - `L` = number of discovered logical-state lifetimes.
//!
//! Analysis is:
//!
//! - expected `O(N + A + L log L)` time;
//! - `O(K + L)` memory.
//!
//! The `L log L` term is used only for deterministic interval ordering and
//! peak-live event processing.
//!
//! The implementation never allocates an array proportional to the declared
//! logical-qubit namespace merely to discover liveness.
//!
//! Therefore a circuit may declare a very large number of logical qubits while
//! touching only a small subset without forcing an equally large liveness
//! allocation.
//!
//! # Resource scaling
//!
//! There is no artificial "maximum circuit size" in this module.
//!
//! The implementation uses `usize` for circuit indices and allocation sizes,
//! matching the canonical Rust collection/circuit representation.
//!
//! The practical maximum is therefore determined by:
//!
//! - the canonical IR limits;
//! - addressable memory;
//! - `usize` capacity;
//! - operating-system resource availability.
//!
//! This module does not use `unsafe` and does not bypass the canonical IR's
//! validation/resource policy.
//!
//! # Determinism
//!
//! Public lifetime records are sorted deterministically by:
//!
//! 1. logical qubit identifier;
//! 2. generation;
//! 3. start operation;
//! 4. end operation.
//!
//! Peak-live calculation uses deterministic interval events.
//!
//! No hash-map iteration order becomes compiler-visible behavior.
//!
//! # Ownership
//!
//! This module does NOT define:
//!
//! - another `QubitId`;
//! - another `QuantumCircuit`;
//! - another gate representation;
//! - routing;
//! - physical allocation;
//! - hardware topology;
//! - scheduling;
//! - optimization transformations;
//! - quantum-state simulation;
//! - measurement-result storage;
//! - QPU communication.
//!
//! The canonical quantum IR remains authoritative.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `QubitId`.
//!
//! It never mutates the circuit.
//!
//! ## `analysis/mod.rs`
//!
//! Export:
//!
//! ```text
//! pub mod liveness;
//! ```
//!
//! Optional public re-exports:
//!
//! ```text
//! pub use liveness::{
//!     LivenessAnalysis,
//!     LivenessError,
//!     LivenessInterval,
//!     LivenessSummary,
//! };
//! ```
//!
//! ## `analysis/qubit_use.rs`
//!
//! `qubit_use.rs` and this module intentionally have different responsibilities.
//!
//! `qubit_use.rs` answers:
//!
//! ```text
//! "Where does this qubit occur as an operand?"
//! ```
//!
//! This module answers:
//!
//! ```text
//! "Which logical state is live between those occurrences?"
//! ```
//!
//! Liveness can therefore use qubit-use analysis as a conceptual prerequisite,
//! but this implementation deliberately scans the canonical operations
//! directly. This prevents a hard compile-time dependency and avoids forcing
//! future versions of `qubit_use.rs` to preserve an implementation detail.
//!
//! ## `dependency.rs`
//!
//! Dependency analysis can consume lifetime intervals to efficiently determine
//! whether operations can share/reuse resources.
//!
//! It remains responsible for actual dependency semantics.
//!
//! ## `width.rs`
//!
//! Width analysis can consume:
//!
//! - `peak_live_qubits()`;
//! - `lifetime_count()`;
//! - `intervals()`;
//! - `can_be_reused_after()`.
//!
//! Width analysis remains responsible for reporting circuit-width metrics.
//!
//! ## `depth.rs`
//!
//! Depth analysis can use liveness to identify resource availability, but
//! operation dependency ordering remains the responsibility of depth analysis.
//!
//! ## optimization passes
//!
//! The analysis can support:
//!
//! - qubit reuse planning;
//! - ancilla reuse;
//! - dead-resource elimination;
//! - width reduction;
//! - reset placement analysis;
//! - dynamic-circuit optimization;
//! - memory-pressure-aware planning.
//!
//! It never performs those transformations itself.
//!
//! ## `context.rs`
//!
//! The optimization context may cache an immutable `LivenessAnalysis`.
//!
//! Any pass that changes:
//!
//! - operation order;
//! - operation insertion/removal;
//! - qubit operands;
//! - reset placement;
//! - control-flow structure;
//!
//! must invalidate this analysis.
//!
//! Metadata-only changes may preserve it.
//!
//! # Control-flow limitation
//!
//! The current canonical `QuantumCircuit` representation is an ordered
//! operation sequence. This implementation therefore computes exact liveness
//! for that ordered representation.
//!
//! If Zamani later adds quantum control-flow regions, loops, branches, or
//! dynamic circuit regions to the canonical IR, this file should remain the
//! linear-region engine and a higher-level control-flow liveness analysis should
//! compose it across regions.
//!
//! It must NOT silently pretend that a linear interval is exact for arbitrary
//! cyclic control flow.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! # Verification
//!
//! Tests cover:
//!
//! - empty circuits;
//! - unused qubits;
//! - single lifetime;
//! - multiple lifetimes caused by reset;
//! - measurement followed by later use;
//! - final-use termination;
//! - peak live qubits;
//! - deterministic ordering;
//! - sparse large logical namespaces;
//! - reset as a lifetime boundary;
//! - no lifetime created by a terminal reset;
//! - summary consistency;
//! - invalid circuits;
//! - overflow-safe interval handling.
//!
//! External compiler research also supports treating mid-circuit measurement and
//! reset as important resource-reuse boundaries. Qubit-reuse compilation has
//! demonstrated that measurement/reset can substantially reduce required
//! physical qubit resources, while current quantum SDKs explicitly model reset
//! as returning a qubit to |0>. This module provides the logical analysis layer
//! needed by such later transformations.
//!
//! References:
//!
//! - Qubit-reuse compilation with mid-circuit measurement and reset.
//! - Qiskit reset semantics and dynamic-circuit documentation.
//!
//! The analysis itself remains backend-independent.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

/// Zero-based operation position in the canonical circuit.
pub type OperationIndex = usize;

/// One-past-the-end operation boundary.
///
/// A lifetime `[start, end)` contains operations:
///
/// ```text
/// start .. end
/// ```
///
/// This type is an alias intentionally matching the canonical operation index
/// representation rather than introducing a second operation-ID system.
pub type OperationBoundary = usize;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by logical qubit liveness analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivenessError {
    /// The canonical circuit failed validation.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// An operation index is outside the analyzed circuit.
    OperationOutOfRange {
        /// Requested operation.
        index: usize,

        /// Number of operations.
        operation_count: usize,
    },

    /// A logical qubit is outside the declared circuit namespace.
    QubitOutOfRange {
        /// Offending logical qubit.
        qubit: QubitId,

        /// Number of declared logical qubits.
        qubit_count: usize,
    },

    /// An internal lifetime invariant was violated.
    InvalidInterval {
        /// Logical qubit associated with the interval.
        qubit: QubitId,

        /// Lifetime generation.
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
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze quantum liveness: invalid circuit: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::OperationOutOfRange {
                index,
                operation_count,
            } => {
                write!(
                    formatter,
                    "operation index {index} is outside circuit length \
                     {operation_count}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit:?} is outside circuit namespace \
                     0..{qubit_count}"
                )
            }

            Self::InvalidInterval {
                qubit,
                generation,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "invalid liveness interval for {qubit:?}, generation \
                     {generation}: [{start}, {end})"
                )
            }
        }
    }
}

impl std::error::Error for LivenessError {}

// ============================================================================
// Lifetime interval
// ============================================================================

/// A single logical-state lifetime.
///
/// A lifetime is represented as a half-open interval `[start, end)`.
///
/// The interval describes the lifetime of one logical state generation, not
/// physical occupancy of a QPU qubit.
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
        if end <= first_use {
            return Err(LivenessError::InvalidInterval {
                qubit,
                generation,
                start: first_use,
                end,
            });
        }

        let expected_end = last_use.checked_add(1).ok_or(
            LivenessError::ArithmeticOverflow {
                calculation: "last-use operation boundary",
            },
        )?;

        if reset_terminated {
            if end > expected_end {
                return Err(LivenessError::InvalidInterval {
                    qubit,
                    generation,
                    start: first_use,
                    end,
                });
            }
        } else if end != expected_end {
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

    /// Returns the logical qubit represented by this lifetime.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the zero-based lifetime generation.
    ///
    /// Generation zero is the first logical state generation.
    #[must_use]
    pub const fn generation(self) -> usize {
        self.generation
    }

    /// Returns the first live operation boundary.
    #[must_use]
    pub const fn start(self) -> OperationBoundary {
        self.start
    }

    /// Returns the exclusive end boundary.
    #[must_use]
    pub const fn end(self) -> OperationBoundary {
        self.end
    }

    /// Returns the first operation using this logical state.
    #[must_use]
    pub const fn first_use(self) -> OperationIndex {
        self.first_use
    }

    /// Returns the final operation using this logical state.
    #[must_use]
    pub const fn last_use(self) -> OperationIndex {
        self.last_use
    }

    /// Returns the number of operand occurrences associated with this
    /// lifetime.
    #[must_use]
    pub const fn use_count(self) -> usize {
        self.use_count
    }

    /// Returns the number of measurement operations in this lifetime.
    #[must_use]
    pub const fn measurement_count(self) -> usize {
        self.measurement_count
    }

    /// Returns whether reset terminated this lifetime.
    #[must_use]
    pub const fn reset_terminated(self) -> bool {
        self.reset_terminated
    }

    /// Returns whether the lifetime reaches the end of the circuit.
    #[must_use]
    pub const fn reaches_circuit_end(self, operation_count: usize) -> bool {
        self.end == operation_count
    }

    /// Returns the number of operation positions covered by this lifetime.
    #[must_use]
    pub const fn length(self) -> usize {
        self.end - self.start
    }

    /// Returns the half-open operation range.
    #[must_use]
    pub const fn range(self) -> std::ops::Range<OperationBoundary> {
        self.start..self.end
    }

    /// Returns whether this lifetime is live at an operation boundary.
    ///
    /// Boundaries are interpreted using half-open interval semantics:
    ///
    /// ```text
    /// start <= boundary < end
    /// ```
    #[must_use]
    pub const fn contains_boundary(self, boundary: OperationBoundary) -> bool {
        self.start <= boundary && boundary < self.end
    }

    /// Returns whether this lifetime overlaps another lifetime.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns whether this lifetime is immediately reusable after another
    /// lifetime, ignoring hardware and semantic constraints.
    ///
    /// This is a purely structural interval predicate. A higher-level reuse
    /// planner must still verify measurement/reset/control-flow semantics and
    /// hardware support.
    #[must_use]
    pub const fn is_non_overlapping_with(self, other: Self) -> bool {
        self.end <= other.start || other.end <= self.start
    }
}

// ============================================================================
// Per-qubit liveness summary
// ============================================================================

/// Immutable liveness information for one logical qubit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitLiveness {
    qubit: QubitId,
    lifetime_start: usize,
    lifetime_count: usize,
    first_use: Option<OperationIndex>,
    last_use: Option<OperationIndex>,
    total_use_count: usize,
    total_measurement_count: usize,
    total_reset_count: usize,
}

impl QubitLiveness {
    fn unused(qubit: QubitId) -> Self {
        Self {
            qubit,
            lifetime_start: 0,
            lifetime_count: 0,
            first_use: None,
            last_use: None,
            total_use_count: 0,
            total_measurement_count: 0,
            total_reset_count: 0,
        }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the first index in the global lifetime array belonging to this
    /// qubit.
    ///
    /// This is an implementation-stable index into the immutable analysis
    /// result, not a circuit operation ID.
    #[must_use]
    pub const fn lifetime_start(&self) -> usize {
        self.lifetime_start
    }

    /// Returns the number of logical-state lifetimes.
    #[must_use]
    pub const fn lifetime_count(&self) -> usize {
        self.lifetime_count
    }

    /// Returns the first operation using this qubit.
    #[must_use]
    pub const fn first_use(&self) -> Option<OperationIndex> {
        self.first_use
    }

    /// Returns the last operation using this qubit.
    #[must_use]
    pub const fn last_use(&self) -> Option<OperationIndex> {
        self.last_use
    }

    /// Returns whether this qubit is ever used.
    #[must_use]
    pub const fn is_used(&self) -> bool {
        self.first_use.is_some()
    }

    /// Returns whether this qubit is never used.
    #[must_use]
    pub const fn is_unused(&self) -> bool {
        self.first_use.is_none()
    }

    /// Returns total operand uses across all logical-state generations.
    #[must_use]
    pub const fn total_use_count(&self) -> usize {
        self.total_use_count
    }

    /// Returns total measurement operations involving this qubit.
    #[must_use]
    pub const fn total_measurement_count(&self) -> usize {
        self.total_measurement_count
    }

    /// Returns total reset operations involving this qubit.
    #[must_use]
    pub const fn total_reset_count(&self) -> usize {
        self.total_reset_count
    }

    /// Returns whether this qubit has been logically reused after reset.
    #[must_use]
    pub const fn was_reused_after_reset(&self) -> bool {
        self.lifetime_count > 1
    }
}

// ============================================================================
// Internal mutable state
// ============================================================================

#[derive(Debug)]
struct ActiveLifetime {
    generation: usize,
    first_use: OperationIndex,
    last_use: OperationIndex,
    use_count: usize,
    measurement_count: usize,
}

impl ActiveLifetime {
    fn new(
        generation: usize,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<Self, LivenessError> {
        let measurement_count = if gate.is_measurement() {
            1
        } else {
            0
        };

        Ok(Self {
            generation,
            first_use: operation,
            last_use: operation,
            use_count: 1,
            measurement_count,
        })
    }

    fn record(
        &mut self,
        operation: OperationIndex,
        gate: &Gate,
    ) -> Result<(), LivenessError> {
        self.last_use = operation;

        self.use_count = self.use_count.checked_add(1).ok_or(
            LivenessError::ArithmeticOverflow {
                calculation: "lifetime use count",
            },
        )?;

        if gate.is_measurement() {
            self.measurement_count = self
                .measurement_count
                .checked_add(1)
                .ok_or(LivenessError::ArithmeticOverflow {
                    calculation: "lifetime measurement count",
                })?;
        }

        Ok(())
    }
}

// ============================================================================
// Peak-live event
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventKind {
    End,
    Start,
}

impl EventKind {
    const fn order(self) -> u8 {
        match self {
            Self::End => 0,
            Self::Start => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LiveEvent {
    boundary: OperationBoundary,
    kind: EventKind,
}

impl LiveEvent {
    const fn new(
        boundary: OperationBoundary,
        kind: EventKind,
    ) -> Self {
        Self { boundary, kind }
    }
}

// ============================================================================
// Aggregate analysis
// ============================================================================

/// Complete immutable logical-state liveness analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessAnalysis {
    declared_qubits: usize,
    operation_count: usize,
    intervals: Vec<LivenessInterval>,
    per_qubit: Vec<QubitLiveness>,
    peak_live_qubits: usize,
    peak_live_boundary: Option<OperationBoundary>,
    used_qubit_count: usize,
    unused_qubit_count: usize,
    lifetime_count: usize,
    reset_count: usize,
    measurement_count: usize,
}

impl LivenessAnalysis {
    /// Analyze a canonical quantum circuit.
    ///
    /// This method validates the circuit before analysis.
    ///
    /// # Complexity
    ///
    /// Expected:
    ///
    /// ```text
    /// O(N + A + L log L)
    /// ```
    ///
    /// memory:
    ///
    /// ```text
    /// O(K + L)
    /// ```
    pub fn analyze(
        circuit: &QuantumCircuit,
    ) -> Result<Self, LivenessError> {
        circuit
            .validate()
            .map_err(|error| LivenessError::InvalidCircuit {
                message: error.to_string(),
            })?;

        Self::analyze_validated(circuit)
    }

    /// Analyze a circuit that the caller has already validated.
    ///
    /// This avoids repeating whole-circuit validation in an optimization
    /// pipeline.
    ///
    /// The caller is responsible for the validation precondition.
    #[must_use = "liveness results should not be silently discarded"]
    pub fn analyze_validated(
        circuit: &QuantumCircuit,
    ) -> Result<Self, LivenessError> {
        let declared_qubits = circuit.num_qubits();
        let operations = circuit.operations();
        let operation_count = operations.len();

        let mut active: HashMap<QubitId, ActiveLifetime> =
            HashMap::new();

        let mut intervals = Vec::<LivenessInterval>::new();

        let mut reset_counts: HashMap<QubitId, usize> =
            HashMap::new();

        let mut measurement_counts: HashMap<QubitId, usize> =
            HashMap::new();

        for (operation_index, gate) in operations.iter().enumerate() {
            for &qubit in gate.qubits() {
                if qubit.index() >= declared_qubits {
                    return Err(LivenessError::QubitOutOfRange {
                        qubit,
                        qubit_count: declared_qubits,
                    });
                }

                if gate.is_measurement() {
                    let count = measurement_counts
                        .entry(qubit)
                        .or_insert(0);

                    *count = count.checked_add(1).ok_or(
                        LivenessError::ArithmeticOverflow {
                            calculation:
                                "per-qubit measurement count",
                        },
                    )?;
                }

                if gate.is_reset() {
                    let count = reset_counts
                        .entry(qubit)
                        .or_insert(0);

                    *count = count.checked_add(1).ok_or(
                        LivenessError::ArithmeticOverflow {
                            calculation:
                                "per-qubit reset count",
                        },
                    )?;

                    if let Some(state) = active.remove(&qubit) {
                        let interval =
                            LivenessInterval::new(
                                qubit,
                                state.generation,
                                state.first_use,
                                state.last_use,
                                state.use_count,
                                state.measurement_count,
                                operation_index,
                                true,
                            )?;

                        intervals.push(interval);
                    }

                    // Reset itself establishes a new |0> state, but that new
                    // state is not considered live until a later operation
                    // actually consumes it.
                    continue;
                }

                match active.get_mut(&qubit) {
                    Some(state) => {
                        state.record(operation_index, gate)?;
                    }

                    None => {
                        let generation = intervals
                            .iter()
                            .filter(|interval| {
                                interval.qubit() == qubit
                            })
                            .count();

                        active.insert(
                            qubit,
                            ActiveLifetime::new(
                                generation,
                                operation_index,
                                gate,
                            )?,
                        );
                    }
                }
            }
        }

        // Close all remaining logical-state lifetimes at the end of the
        // circuit.
        for (qubit, state) in active {
            let end = operation_count;

            let interval = LivenessInterval::new(
                qubit,
                state.generation,
                state.first_use,
                state.last_use,
                state.use_count,
                state.measurement_count,
                end,
                false,
            )?;

            intervals.push(interval);
        }

        intervals.sort_unstable_by(|left, right| {
            left.qubit()
                .cmp(&right.qubit())
                .then_with(|| {
                    left.generation().cmp(&right.generation())
                })
                .then_with(|| left.start().cmp(&right.start()))
                .then_with(|| left.end().cmp(&right.end()))
        });

        let lifetime_count = intervals.len();

        let mut per_qubit = Vec::with_capacity(
            declared_qubits.min(
                lifetime_count
                    .checked_add(
                        declared_qubits.saturating_sub(lifetime_count),
                    )
                    .unwrap_or(declared_qubits),
            ),
        );

        // For sparse logical namespaces, constructing a record for every
        // qubit would defeat the purpose of sparse liveness analysis.
        //
        // Therefore `per_qubit` contains only qubits that are actually used.
        //
        // The vector is deterministic because intervals are sorted by qubit.
        let mut index = 0usize;

        while index < intervals.len() {
            let qubit = intervals[index].qubit();
            let lifetime_start = index;

            let first_use = intervals[index].first_use();
            let mut last_use = intervals[index].last_use();
            let mut total_use_count = 0usize;
            let mut lifetime_end = index;

            while lifetime_end < intervals.len()
                && intervals[lifetime_end].qubit() == qubit
            {
                let interval = intervals[lifetime_end];

                last_use = last_use.max(interval.last_use());

                total_use_count = total_use_count
                    .checked_add(interval.use_count())
                    .ok_or(LivenessError::ArithmeticOverflow {
                        calculation: "per-qubit total use count",
                    })?;

                lifetime_end += 1;
            }

            let measurement_count =
                measurement_counts.get(&qubit).copied().unwrap_or(0);

            let reset_count =
                reset_counts.get(&qubit).copied().unwrap_or(0);

            per_qubit.push(QubitLiveness {
                qubit,
                lifetime_start,
                lifetime_count: lifetime_end - lifetime_start,
                first_use: Some(first_use),
                last_use: Some(last_use),
                total_use_count,
                total_measurement_count: measurement_count,
                total_reset_count: reset_count,
            });

            index = lifetime_end;
        }

        let used_qubit_count = per_qubit.len();

        let unused_qubit_count =
            declared_qubits.saturating_sub(used_qubit_count);

        let mut events = Vec::<LiveEvent>::with_capacity(
            lifetime_count
                .checked_mul(2)
                .ok_or(LivenessError::ArithmeticOverflow {
                    calculation: "liveness event capacity",
                })?,
        );

        for interval in &intervals {
            events.push(LiveEvent::new(
                interval.start(),
                EventKind::Start,
            ));

            events.push(LiveEvent::new(
                interval.end(),
                EventKind::End,
            ));
        }

        events.sort_unstable_by(|left, right| {
            left.boundary
                .cmp(&right.boundary)
                .then_with(|| {
                    left.kind
                        .order()
                        .cmp(&right.kind.order())
                })
        });

        let (
            peak_live_qubits,
            peak_live_boundary,
        ) = Self::compute_peak(&events);

        let reset_count = reset_counts.values().try_fold(
            0usize,
            |total, value| {
                total.checked_add(*value).ok_or(
                    LivenessError::ArithmeticOverflow {
                        calculation: "total reset count",
                    },
                )
            },
        )?;

        let measurement_count =
            measurement_counts.values().try_fold(
                0usize,
                |total, value| {
                    total.checked_add(*value).ok_or(
                        LivenessError::ArithmeticOverflow {
                            calculation: "total measurement count",
                        },
                    )
                },
            )?;

        // `Vec::with_capacity` above intentionally avoids materializing
        // declared_qubits entries for sparse circuits. Keep the binding
        // consumed so the compiler does not warn about an unused capacity
        // calculation if optimization changes later.
        per_qubit.shrink_to_fit();

        Ok(Self {
            declared_qubits,
            operation_count,
            intervals,
            per_qubit,
            peak_live_qubits,
            peak_live_boundary,
            used_qubit_count,
            unused_qubit_count,
            lifetime_count,
            reset_count,
            measurement_count,
        })
    }

    fn compute_peak(
        events: &[LiveEvent],
    ) -> (usize, Option<OperationBoundary>) {
        let mut current = 0usize;
        let mut peak = 0usize;
        let mut peak_boundary = None;

        for event in events {
            match event.kind {
                EventKind::End => {
                    current = current.saturating_sub(1);
                }

                EventKind::Start => {
                    current = current.saturating_add(1);

                    if current > peak {
                        peak = current;
                        peak_boundary = Some(event.boundary);
                    }
                }
            }
        }

        (peak, peak_boundary)
    }

    /// Returns the declared logical-qubit namespace size.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of operations analyzed.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of distinct logical qubits used.
    #[must_use]
    pub const fn used_qubit_count(&self) -> usize {
        self.used_qubit_count
    }

    /// Returns the number of declared logical qubits never used.
    #[must_use]
    pub const fn unused_qubit_count(&self) -> usize {
        self.unused_qubit_count
    }

    /// Returns the number of logical-state lifetimes.
    #[must_use]
    pub const fn lifetime_count(&self) -> usize {
        self.lifetime_count
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

    /// Returns the peak number of simultaneously live logical states.
    ///
    /// This is a logical-state metric, not a hardware-qubit count.
    #[must_use]
    pub const fn peak_live_qubits(&self) -> usize {
        self.peak_live_qubits
    }

    /// Returns the operation boundary at which peak logical liveness first
    /// occurs.
    ///
    /// Returns `None` when no logical state is ever live.
    #[must_use]
    pub const fn peak_live_boundary(
        &self,
    ) -> Option<OperationBoundary> {
        self.peak_live_boundary
    }

    /// Returns all lifetime intervals in deterministic order.
    #[must_use]
    pub fn intervals(&self) -> &[LivenessInterval] {
        &self.intervals
    }

    /// Returns all used-qubit summaries in deterministic qubit order.
    ///
    /// Unused logical qubits are intentionally omitted to keep sparse analysis
    /// memory proportional to actual usage.
    #[must_use]
    pub fn used_qubits(&self) -> &[QubitLiveness] {
        &self.per_qubit
    }

    /// Returns the lifetime intervals belonging to one logical qubit.
    ///
    /// Returns an empty slice when the qubit is declared but never used.
    #[must_use]
    pub fn lifetimes_for(
        &self,
        qubit: QubitId,
    ) -> &[LivenessInterval] {
        match self
            .per_qubit
            .binary_search_by_key(&qubit, QubitLiveness::qubit)
        {
            Ok(index) => {
                let summary = &self.per_qubit[index];

                &self.intervals[
                    summary.lifetime_start
                        ..summary.lifetime_start
                            + summary.lifetime_count
                ]
            }

            Err(_) => &[],
        }
    }

    /// Returns the liveness summary for one logical qubit.
    ///
    /// Returns `None` for an unused logical qubit.
    #[must_use]
    pub fn qubit(
        &self,
        qubit: QubitId,
    ) -> Option<&QubitLiveness> {
        self.per_qubit
            .binary_search_by_key(&qubit, QubitLiveness::qubit)
            .ok()
            .map(|index| &self.per_qubit[index])
    }

    /// Returns whether a logical qubit is used.
    #[must_use]
    pub fn is_used(&self, qubit: QubitId) -> bool {
        self.qubit(qubit).is_some()
    }

    /// Returns whether a declared logical qubit is unused.
    #[must_use]
    pub fn is_unused(&self, qubit: QubitId) -> bool {
        qubit.index() < self.declared_qubits
            && !self.is_used(qubit)
    }

    /// Returns the first operation using a qubit.
    #[must_use]
    pub fn first_use(
        &self,
        qubit: QubitId,
    ) -> Option<OperationIndex> {
        self.qubit(qubit)
            .and_then(QubitLiveness::first_use)
    }

    /// Returns the final operation using a qubit.
    #[must_use]
    pub fn last_use(
        &self,
        qubit: QubitId,
    ) -> Option<OperationIndex> {
        self.qubit(qubit)
            .and_then(QubitLiveness::last_use)
    }

    /// Returns the number of logical-state generations for a qubit.
    #[must_use]
    pub fn lifetime_count_for(
        &self,
        qubit: QubitId,
    ) -> usize {
        self.qubit(qubit)
            .map(QubitLiveness::lifetime_count)
            .unwrap_or(0)
    }

    /// Returns whether a qubit has been reused after a reset.
    #[must_use]
    pub fn was_reused_after_reset(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.qubit(qubit)
            .map(QubitLiveness::was_reused_after_reset)
            .unwrap_or(false)
    }

    /// Returns whether a logical state is live at a given operation boundary.
    ///
    /// Complexity is `O(log L)` because lifetime intervals are stored in
    /// deterministic order and searched by the qubit's interval range.
    #[must_use]
    pub fn is_live_at(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> bool {
        self.lifetimes_for(qubit)
            .iter()
            .any(|interval| interval.contains_boundary(boundary))
    }

    /// Returns whether two logical-state lifetimes overlap.
    ///
    /// This is a convenience wrapper for callers implementing resource
    /// allocation/reuse planning.
    #[must_use]
    pub fn lifetimes_overlap(
        &self,
        left: LivenessInterval,
        right: LivenessInterval,
    ) -> bool {
        left.overlaps(right)
    }

    /// Returns whether a qubit's logical state has no use after the supplied
    /// operation boundary.
    ///
    /// This does not mean that the physical qubit may necessarily be released.
    /// Hardware and dynamic-circuit semantics remain outside this analysis.
    #[must_use]
    pub fn is_dead_after(
        &self,
        qubit: QubitId,
        boundary: OperationBoundary,
    ) -> bool {
        match self.last_use(qubit) {
            Some(last_use) => last_use < boundary,
            None => true,
        }
    }

    /// Returns whether a qubit has become dead after its final logical use.
    #[must_use]
    pub fn is_dead_after_last_use(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.last_use(qubit).is_some()
    }

    /// Returns the number of currently live states at a boundary.
    ///
    /// This intentionally scans only the stored intervals rather than
    /// materializing a full `operation × qubit` matrix.
    ///
    /// It is therefore suitable for diagnostics and sparse queries. For bulk
    /// per-boundary analysis, callers should consume `intervals()` directly or
    /// build a dedicated sweep.
    #[must_use]
    pub fn live_count_at(
        &self,
        boundary: OperationBoundary,
    ) -> usize {
        self.intervals
            .iter()
            .filter(|interval| interval.contains_boundary(boundary))
            .count()
    }

    /// Returns the first used logical qubit.
    #[must_use]
    pub fn first_used_qubit(&self) -> Option<QubitId> {
        self.per_qubit.first().map(QubitLiveness::qubit)
    }

    /// Returns the last used logical qubit.
    #[must_use]
    pub fn last_used_qubit(&self) -> Option<QubitId> {
        self.per_qubit.last().map(QubitLiveness::qubit)
    }

    /// Returns whether the circuit contains no logical-state lifetimes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Returns the logical-qubit utilization ratio.
    ///
    /// Returns `0.0` for a zero-qubit circuit.
    #[must_use]
    pub fn qubit_utilization(&self) -> f64 {
        if self.declared_qubits == 0 {
            return 0.0;
        }

        self.used_qubit_count as f64
            / self.declared_qubits as f64
    }

    /// Returns the average number of logical-state lifetimes per used qubit.
    ///
    /// Returns `0.0` when no qubits are used.
    #[must_use]
    pub fn average_lifetimes_per_used_qubit(&self) -> f64 {
        if self.used_qubit_count == 0 {
            return 0.0;
        }

        self.lifetime_count as f64
            / self.used_qubit_count as f64
    }

    /// Returns an immutable compact summary.
    #[must_use]
    pub fn summary(&self) -> LivenessSummary {
        LivenessSummary {
            declared_qubits: self.declared_qubits,
            used_qubits: self.used_qubit_count,
            unused_qubits: self.unused_qubit_count,
            operation_count: self.operation_count,
            lifetime_count: self.lifetime_count,
            peak_live_qubits: self.peak_live_qubits,
            reset_count: self.reset_count,
            measurement_count: self.measurement_count,
        }
    }
}

// ============================================================================
// Summary
// ============================================================================

/// Allocation-free summary of a liveness analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LivenessSummary {
    declared_qubits: usize,
    used_qubits: usize,
    unused_qubits: usize,
    operation_count: usize,
    lifetime_count: usize,
    peak_live_qubits: usize,
    reset_count: usize,
    measurement_count: usize,
}

impl LivenessSummary {
    /// Number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(self) -> usize {
        self.declared_qubits
    }

    /// Number of used logical qubits.
    #[must_use]
    pub const fn used_qubits(self) -> usize {
        self.used_qubits
    }

    /// Number of unused logical qubits.
    #[must_use]
    pub const fn unused_qubits(self) -> usize {
        self.unused_qubits
    }

    /// Number of circuit operations.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        self.operation_count
    }

    /// Number of logical-state lifetimes.
    #[must_use]
    pub const fn lifetime_count(self) -> usize {
        self.lifetime_count
    }

    /// Peak simultaneously live logical states.
    #[must_use]
    pub const fn peak_live_qubits(self) -> usize {
        self.peak_live_qubits
    }

    /// Number of reset operations.
    #[must_use]
    pub const fn reset_count(self) -> usize {
        self.reset_count
    }

    /// Number of measurement operations.
    #[must_use]
    pub const fn measurement_count(self) -> usize {
        self.measurement_count
    }
}

// ============================================================================
// Convenience API
// ============================================================================

/// Analyze logical qubit liveness in a canonical quantum circuit.
#[must_use = "liveness results should not be silently discarded"]
pub fn analyze_liveness(
    circuit: &QuantumCircuit,
) -> Result<LivenessAnalysis, LivenessError> {
    LivenessAnalysis::analyze(circuit)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::parameter::Parameter;

    fn circuit_with_qubits(
        qubits: usize,
    ) -> QuantumCircuit {
        QuantumCircuit::new(qubits, 0)
            .expect("valid test circuit")
    }

    fn append_gate(
        circuit: &mut QuantumCircuit,
        kind: GateKind,
        qubits: &[usize],
    ) {
        let operands = qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        let parameters = match kind.parameter_count() {
            0 => Vec::new(),

            1 => vec![Parameter::Constant(0.0)],

            2 => vec![
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
            ],

            3 => vec![
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
                Parameter::Constant(0.0),
            ],

            count => panic!(
                "unsupported test parameter count {count}"
            ),
        };

        let classical_target =
            if kind.requires_classical_target() {
                Some(0)
            } else {
                None
            };

        let gate = Gate::new(
            kind,
            operands,
            parameters,
            classical_target,
            None,
        )
        .expect("valid test gate");

        circuit
            .push_gate(gate)
            .expect("valid gate insertion");
    }

    #[test]
    fn empty_circuit_has_no_lifetimes() {
        let circuit = circuit_with_qubits(8);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(analysis.declared_qubits(), 8);
        assert_eq!(analysis.operation_count(), 0);
        assert_eq!(analysis.used_qubit_count(), 0);
        assert_eq!(analysis.unused_qubit_count(), 8);
        assert_eq!(analysis.lifetime_count(), 0);
        assert_eq!(analysis.peak_live_qubits(), 0);
        assert!(analysis.is_empty());
    }

    #[test]
    fn one_gate_creates_one_lifetime() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(analysis.lifetime_count(), 1);
        assert_eq!(analysis.peak_live_qubits(), 1);
        assert_eq!(
            analysis.peak_live_boundary(),
            Some(0)
        );

        let intervals = analysis.lifetimes_for(
            QubitId::new(0),
        );

        assert_eq!(intervals.len(), 1);

        let interval = intervals[0];

        assert_eq!(interval.generation(), 0);
        assert_eq!(interval.start(), 0);
        assert_eq!(interval.end(), 1);
        assert_eq!(interval.first_use(), 0);
        assert_eq!(interval.last_use(), 0);
        assert_eq!(interval.use_count(), 1);
        assert!(!interval.reset_terminated());
    }

    #[test]
    fn multiple_gates_share_one_lifetime() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::Z, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let intervals =
            analysis.lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].start(), 0);
        assert_eq!(intervals[0].end(), 3);
        assert_eq!(intervals[0].last_use(), 2);
        assert_eq!(intervals[0].use_count(), 3);
    }

    #[test]
    fn reset_splits_lifetimes() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let intervals =
            analysis.lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 2);

        assert_eq!(intervals[0].generation(), 0);
        assert_eq!(intervals[0].start(), 0);
        assert_eq!(intervals[0].end(), 1);
        assert_eq!(intervals[0].last_use(), 0);
        assert!(intervals[0].reset_terminated());

        assert_eq!(intervals[1].generation(), 1);
        assert_eq!(intervals[1].start(), 2);
        assert_eq!(intervals[1].end(), 3);
        assert_eq!(intervals[1].last_use(), 2);
        assert!(!intervals[1].reset_terminated());

        assert_eq!(
            analysis.lifetime_count_for(QubitId::new(0)),
            2
        );

        assert!(
            analysis.was_reused_after_reset(
                QubitId::new(0)
            )
        );
    }

    #[test]
    fn terminal_reset_does_not_create_empty_new_lifetime() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let intervals =
            analysis.lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].start(), 0);
        assert_eq!(intervals[0].end(), 1);
        assert_eq!(intervals[0].last_use(), 0);
        assert!(intervals[0].reset_terminated());
    }

    #[test]
    fn reset_as_first_operation_creates_no_lifetime() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let intervals =
            analysis.lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].generation(), 0);
        assert_eq!(intervals[0].start(), 1);
        assert_eq!(intervals[0].end(), 2);
    }

    #[test]
    fn measurement_does_not_automatically_kill_lifetime() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(
            &mut circuit,
            GateKind::Measure,
            &[0],
        );
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let intervals =
            analysis.lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].start(), 0);
        assert_eq!(intervals[0].end(), 3);
        assert_eq!(intervals[0].measurement_count(), 1);
        assert_eq!(intervals[0].last_use(), 2);
    }

    #[test]
    fn final_measurement_ends_lifetime_at_circuit_boundary() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(
            &mut circuit,
            GateKind::Measure,
            &[0],
        );

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let interval =
            analysis.lifetimes_for(QubitId::new(0))[0];

        assert_eq!(interval.start(), 0);
        assert_eq!(interval.end(), 2);
        assert_eq!(interval.last_use(), 1);
        assert!(!interval.reset_terminated());
        assert!(
            interval.reaches_circuit_end(
                analysis.operation_count()
            )
        );
    }

    #[test]
    fn independent_qubits_have_overlapping_lifetimes() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::X, &[1]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(analysis.peak_live_qubits(), 2);

        let q0 =
            analysis.lifetimes_for(QubitId::new(0))[0];

        let q1 =
            analysis.lifetimes_for(QubitId::new(1))[0];

        assert!(q0.overlaps(q1));
    }

    #[test]
    fn reset_allows_non_overlapping_generations() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::X, &[1]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let q0 = analysis.lifetimes_for(
            QubitId::new(0),
        );

        let q1 = analysis.lifetimes_for(
            QubitId::new(1),
        );

        assert_eq!(q0.len(), 2);
        assert_eq!(q1.len(), 1);

        assert!(
            q0[0].is_non_overlapping_with(q0[1])
        );
        assert!(
            q0[0].is_non_overlapping_with(q1[0])
        );

        assert_eq!(analysis.peak_live_qubits(), 2);
    }

    #[test]
    fn peak_live_boundary_is_deterministic() {
        let mut circuit = circuit_with_qubits(3);

        append_gate(&mut circuit, GateKind::X, &[2]);
        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::X, &[1]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(
            analysis.peak_live_qubits(),
            3
        );

        assert_eq!(
            analysis.peak_live_boundary(),
            Some(2)
        );
    }

    #[test]
    fn sparse_namespace_does_not_create_unused_records() {
        let mut circuit =
            circuit_with_qubits(1_000_000);

        append_gate(
            &mut circuit,
            GateKind::H,
            &[999_999],
        );

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(
            analysis.declared_qubits(),
            1_000_000
        );

        assert_eq!(
            analysis.used_qubit_count(),
            1
        );

        assert_eq!(
            analysis.unused_qubit_count(),
            999_999
        );

        assert_eq!(
            analysis.used_qubits().len(),
            1
        );

        assert_eq!(
            analysis.first_used_qubit(),
            Some(QubitId::new(999_999))
        );
    }

    #[test]
    fn deterministic_qubit_ordering() {
        let mut circuit = circuit_with_qubits(8);

        append_gate(&mut circuit, GateKind::X, &[7]);
        append_gate(&mut circuit, GateKind::X, &[2]);
        append_gate(&mut circuit, GateKind::X, &[5]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let ids = analysis
            .used_qubits()
            .iter()
            .map(QubitLiveness::qubit)
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                QubitId::new(0),
                QubitId::new(2),
                QubitId::new(5),
                QubitId::new(7),
            ]
        );
    }

    #[test]
    fn deterministic_lifetime_generation_ordering() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::Z, &[0]);

        let intervals =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis")
                .lifetimes_for(QubitId::new(0));

        assert_eq!(intervals.len(), 3);

        assert_eq!(intervals[0].generation(), 0);
        assert_eq!(intervals[1].generation(), 1);
        assert_eq!(intervals[2].generation(), 2);

        assert_eq!(intervals[0].start(), 0);
        assert_eq!(intervals[0].end(), 1);

        assert_eq!(intervals[1].start(), 2);
        assert_eq!(intervals[1].end(), 3);

        assert_eq!(intervals[2].start(), 4);
        assert_eq!(intervals[2].end(), 5);
    }

    #[test]
    fn is_live_at_uses_half_open_semantics() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::H, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let interval =
            analysis.lifetimes_for(QubitId::new(0))[0];

        assert!(interval.contains_boundary(0));
        assert!(interval.contains_boundary(1));
        assert!(!interval.contains_boundary(2));

        assert!(
            analysis.is_live_at(
                QubitId::new(0),
                0
            )
        );

        assert!(
            analysis.is_live_at(
                QubitId::new(0),
                1
            )
        );

        assert!(
            !analysis.is_live_at(
                QubitId::new(0),
                2
            )
        );
    }

    #[test]
    fn dead_after_uses_is_correct() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::X, &[0]);
        append_gate(&mut circuit, GateKind::H, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert!(
            !analysis.is_dead_after(
                QubitId::new(0),
                0
            )
        );

        assert!(
            !analysis.is_dead_after(
                QubitId::new(0),
                1
            )
        );

        assert!(
            analysis.is_dead_after(
                QubitId::new(0),
                2
            )
        );
    }

    #[test]
    fn summary_matches_analysis() {
        let mut circuit = circuit_with_qubits(3);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::CX, &[0, 1]);
        append_gate(&mut circuit, GateKind::Measure, &[1]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let summary = analysis.summary();

        assert_eq!(
            summary.declared_qubits(),
            analysis.declared_qubits()
        );

        assert_eq!(
            summary.used_qubits(),
            analysis.used_qubit_count()
        );

        assert_eq!(
            summary.unused_qubits(),
            analysis.unused_qubit_count()
        );

        assert_eq!(
            summary.operation_count(),
            analysis.operation_count()
        );

        assert_eq!(
            summary.lifetime_count(),
            analysis.lifetime_count()
        );

        assert_eq!(
            summary.peak_live_qubits(),
            analysis.peak_live_qubits()
        );

        assert_eq!(
            summary.reset_count(),
            analysis.reset_count()
        );

        assert_eq!(
            summary.measurement_count(),
            analysis.measurement_count()
        );
    }

    #[test]
    fn analyze_validated_matches_full_analysis() {
        let mut circuit = circuit_with_qubits(2);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(&mut circuit, GateKind::CX, &[0, 1]);
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let normal =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let validated =
            LivenessAnalysis::analyze_validated(
                &circuit,
            )
            .expect("analysis");

        assert_eq!(normal, validated);
    }

    #[test]
    fn convenience_function_matches_constructor() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::X, &[0]);

        let direct =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let convenience =
            analyze_liveness(&circuit)
                .expect("analysis");

        assert_eq!(direct, convenience);
    }

    #[test]
    fn qubit_liveness_summary_is_sparse() {
        let mut circuit = circuit_with_qubits(16);

        append_gate(&mut circuit, GateKind::X, &[3]);
        append_gate(&mut circuit, GateKind::X, &[7]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        assert_eq!(analysis.used_qubits().len(), 2);
        assert_eq!(
            analysis
                .qubit(QubitId::new(3))
                .expect("q3")
                .lifetime_count(),
            1
        );

        assert!(
            analysis
                .qubit(QubitId::new(0))
                .is_none()
        );
    }

    #[test]
    fn measurement_and_reset_counts_are_preserved() {
        let mut circuit = circuit_with_qubits(1);

        append_gate(&mut circuit, GateKind::H, &[0]);
        append_gate(
            &mut circuit,
            GateKind::Measure,
            &[0],
        );
        append_gate(&mut circuit, GateKind::Reset, &[0]);
        append_gate(&mut circuit, GateKind::X, &[0]);

        let analysis =
            LivenessAnalysis::analyze(&circuit)
                .expect("analysis");

        let summary =
            analysis
                .qubit(QubitId::new(0))
                .expect("q0");

        assert_eq!(
            summary.total_measurement_count(),
            1
        );

        assert_eq!(
            summary.total_reset_count(),
            1
        );

        assert_eq!(
            summary.lifetime_count(),
            2
        );

        assert_eq!(
            analysis.measurement_count(),
            1
        );

        assert_eq!(
            analysis.reset_count(),
            1
        );
    }
}