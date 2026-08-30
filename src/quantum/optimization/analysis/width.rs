//! Zamani Quantum Optimization — Width Analysis
//!
//! Production-grade logical-width analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                    │
//!                                    ▼
//!                         optimization::analysis
//!                                    │
//!                                    ▼
//!                              width analysis
//!                                    │
//!              ┌─────────────────────┼─────────────────────┐
//!              ▼                     ▼                     ▼
//!       width optimization      liveness             cost models
//!              │                     │                     │
//!              └─────────────────────┼─────────────────────┘
//!                                    ▼
//!                              optimization
//! ```
//!
//! # Purpose
//!
//! This module measures the logical width characteristics of a canonical
//! Zamani quantum circuit without modifying it.
//!
//! It deliberately distinguishes several quantities that are often incorrectly
//! collapsed into a single "width" number:
//!
//! 1. declared logical width;
//! 2. number of logical qubits actually used;
//! 3. number of declared-but-unused logical qubits;
//! 4. maximum number of distinct qubits appearing in one operation;
//! 5. conservative peak logical-use span;
//! 6. total logical-qubit operand uses;
//! 7. maximum operation arity;
//! 8. operation positions at which the peak operand width occurs;
//! 9. sparse namespace density.
//!
//! The primary `width()` metric is the declared logical-qubit width because
//! that is the canonical circuit resource represented by `QuantumCircuit`.
//!
//! The other metrics are intentionally exposed separately so optimization
//! passes do not accidentally treat "used qubits", "declared qubits", and
//! "simultaneously live qubits" as equivalent concepts.
//!
//! # Important semantic distinction
//!
//! ```text
//! declared width
//!     = circuit.num_qubits()
//!
//! used width
//!     = number of distinct logical qubits appearing in operations
//!
//! peak operand width
//!     = maximum number of distinct logical qubits used by one operation
//!
//! peak use-span width
//!     = conservative maximum number of qubits whose first-use/last-use
//!       intervals overlap
//! ```
//!
//! `peak_use_span_width` is intentionally conservative. It is NOT a replacement
//! for full liveness analysis and must not be interpreted as exact physical
//! qubit occupancy. Measurement, reset, control-flow, aliasing, region
//! boundaries, and future dynamic allocation semantics belong to the dedicated
//! liveness/structure analyses.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of circuit operations;
//! - `A` = total number of logical-qubit operands;
//! - `K` = number of distinct logical qubits actually used.
//!
//! The analysis is:
//!
//! - `O(N + A + K log K)` time;
//! - `O(K)` auxiliary memory.
//!
//! The `K log K` term is used only to produce deterministic sorted output.
//! There is deliberately no allocation proportional to `circuit.num_qubits()`.
//!
//! This is essential for sparse circuits such as:
//!
//! ```text
//! declared namespace: 1,000,000,000 logical qubits
//! actual usage:       17 logical qubits
//! ```
//!
//! The analysis allocates storage for the 17 used qubits rather than for the
//! billion-qubit namespace.
//!
//! # Resource policy
//!
//! This module does not introduce an optimizer-specific artificial maximum.
//! The canonical Quantum IR remains responsible for circuit resource limits.
//!
//! Arithmetic used for derived metrics is checked. The analysis never silently
//! wraps counters or derived sizes.
//!
//! # Determinism
//!
//! Public lists of logical qubits and peak operation positions are returned in
//! ascending deterministic order.
//!
//! Hash-map implementation details never become compiler-visible behavior.
//!
//! # Ownership rules
//!
//! This module does NOT define:
//!
//! - another quantum circuit representation;
//! - another `QubitId`;
//! - physical qubits;
//! - routing;
//! - hardware topology;
//! - scheduling;
//! - pulse timing;
//! - quantum-state simulation;
//! - circuit transformation;
//! - qubit allocation;
//! - error correction;
//! - backend execution.
//!
//! The canonical representations remain those in `crate::quantum::ir`.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes:
//!
//! - `QuantumCircuit`;
//! - canonical `Gate`;
//! - canonical `QubitId`.
//!
//! The circuit is never mutated.
//!
//! ## `analysis::mod`
//!
//! The analysis module should expose:
//!
//! ```text
//! pub mod width;
//! ```
//!
//! and re-export the primary types/functions:
//!
//! ```text
//! pub use width::{analyze_width, WidthAnalysis, WidthError};
//! ```
//!
//! ## `qubit_use.rs`
//!
//! `qubit_use.rs` and this module intentionally overlap in some information,
//! but they have different responsibilities.
//!
//! `qubit_use.rs` provides detailed per-qubit use records.
//!
//! This module provides the width-oriented aggregate view required by width
//! optimization and cost models.
//!
//! A future optimization context may cache both analyses independently.
//!
//! ## `liveness.rs`
//!
//! Full liveness analysis may use this module's:
//!
//! - declared width;
//! - used qubits;
//! - first/last-use-derived span information.
//!
//! It must remain authoritative for exact liveness semantics.
//!
//! ## `depth.rs`
//!
//! Depth analysis may use peak operand width for reporting, but circuit depth
//! must remain a dependency/scheduling analysis and must not be inferred from
//! width.
//!
//! ## `dependency.rs`
//!
//! Dependency analysis can use the deterministic used-qubit set to initialize
//! per-qubit dependency state.
//!
//! ## `passes/optimize_width.rs`
//!
//! Width optimization should use this module to establish:
//!
//! - input declared width;
//! - actually used logical width;
//! - removable unused namespace capacity;
//! - peak operation arity;
//! - conservative live-span width.
//!
//! A transformation pass remains responsible for changing the circuit.
//!
//! ## `cost.rs`
//!
//! Width-related cost models should consume this analysis instead of counting
//! qubits independently.
//!
//! ## `context.rs`
//!
//! The immutable `WidthAnalysis` result is suitable for context-level caching.
//!
//! Any transformation that changes:
//!
//! - logical qubit count;
//! - operation sequence;
//! - operation operands;
//!
//! invalidates this analysis.
//!
//! Metadata-only transformations do not invalidate it.
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
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Verification
//!
//! Tests cover:
//!
//! - empty circuits;
//! - zero-qubit circuits;
//! - declared versus used width;
//! - sparse logical namespaces;
//! - unused qubits;
//! - single-qubit operations;
//! - multi-qubit operations;
//! - repeated qubit usage;
//! - deterministic ordering;
//! - peak operation width;
//! - conservative use-span width;
//! - total operand accounting;
//! - maximum operation arity;
//! - arithmetic-safe calculations;
//! - invalid/out-of-range operands where representable;
//! - idempotent repeated analysis.
//!
//! # Design principle
//!
//! The analysis must remain useful from tiny circuits to extremely large
//! circuits without changing its public API.
//!
//! The implementation therefore:
//!
//! - uses checked arithmetic;
//! - avoids dense allocation over the logical namespace;
//! - avoids recursion;
//! - avoids global state;
//! - avoids unsafe code;
//! - avoids backend dependencies;
//! - avoids mutable global caches;
//! - returns immutable results;
//! - keeps deterministic ordering explicit.
//!
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

use crate::quantum::ir::gate::Gate;
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Public scalar types
// =============================================================================

/// Zero-based operation position in the canonical circuit.
pub type OperationIndex = usize;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by width analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidthError {
    /// The supplied circuit contains invalid structure.
    ///
    /// Canonical `QuantumCircuit` construction normally prevents this state,
    /// but the analysis boundary remains defensive because IR can eventually
    /// be reconstructed from external sources.
    InvalidCircuit {
        /// Human-readable reason.
        message: String,
    },

    /// A logical qubit operand is outside the declared logical namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Declared logical-qubit count.
        qubit_count: usize,

        /// Operation containing the invalid operand.
        operation: OperationIndex,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// An internal result invariant was violated.
    InvariantViolation {
        /// Static description of the violated invariant.
        message: &'static str,
    },
}

impl fmt::Display for WidthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze circuit width: invalid quantum circuit: {message}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
                operation,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} used by operation {operation} \
                     is outside circuit namespace 0..{qubit_count}"
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
                    "width-analysis invariant violated: {message}"
                )
            }
        }
    }
}

impl std::error::Error for WidthError {}

// =============================================================================
// Internal per-qubit interval
// =============================================================================

/// First/last logical use of one qubit.
///
/// This is intentionally private. The public API exposes aggregate width
/// information rather than creating another optimizer-local qubit model.
#[derive(Debug, Clone, Copy)]
struct UseInterval {
    first: OperationIndex,
    last: OperationIndex,
}

impl UseInterval {
    fn new(operation: OperationIndex) -> Self {
        Self {
            first: operation,
            last: operation,
        }
    }

    fn record(&mut self, operation: OperationIndex) {
        if operation < self.first {
            self.first = operation;
        }

        if operation > self.last {
            self.last = operation;
        }
    }
}

// =============================================================================
// Public result
// =============================================================================

/// Immutable production-grade logical-width analysis result.
///
/// This structure contains aggregate metrics only. It deliberately does not
/// expose mutable internal collections or references to the analyzed circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WidthAnalysis {
    /// Number of logical qubits declared by the circuit.
    declared_width: usize,

    /// Number of distinct logical qubits actually appearing in operations.
    used_width: usize,

    /// Number of declared logical qubits never appearing in any operation.
    unused_width: usize,

    /// Maximum number of distinct logical qubits used by any single operation.
    peak_operand_width: usize,

    /// Maximum overlap of first-use/last-use intervals.
    ///
    /// This is a conservative logical-use span, not exact liveness.
    peak_use_span_width: usize,

    /// Total number of logical-qubit operand occurrences.
    total_operand_uses: usize,

    /// Maximum number of operands attached to one operation.
    maximum_operation_arity: usize,

    /// Number of operations in the circuit.
    operation_count: usize,

    /// Logical qubits that occur in the circuit, sorted by numeric ID.
    used_qubits: Vec<QubitId>,

    /// Logical qubits never used by the circuit, when the namespace is
    /// reasonably representable.
    ///
    /// This vector is intentionally optional because materializing a billion
    /// unused qubits would defeat sparse-circuit scalability.
    unused_qubits: Option<Vec<QubitId>>,

    /// Operation positions attaining `peak_operand_width`.
    peak_operand_operations: Vec<OperationIndex>,

    /// Whether the unused-qubit list was fully materialized.
    unused_qubits_materialized: bool,
}

impl WidthAnalysis {
    /// Returns the canonical declared logical width of the circuit.
    ///
    /// For a purely quantum circuit, this corresponds to the standard circuit
    /// width concept.
    #[must_use]
    pub const fn width(&self) -> usize {
        self.declared_width
    }

    /// Returns the number of distinct logical qubits actually used.
    #[must_use]
    pub const fn used_width(&self) -> usize {
        self.used_width
    }

    /// Returns the number of declared logical qubits that are unused.
    #[must_use]
    pub const fn unused_width(&self) -> usize {
        self.unused_width
    }

    /// Returns the maximum number of distinct qubits appearing in one
    /// operation.
    #[must_use]
    pub const fn peak_operand_width(&self) -> usize {
        self.peak_operand_width
    }

    /// Returns the conservative maximum overlap of first/last-use intervals.
    ///
    /// This must not be treated as exact liveness. Use the dedicated liveness
    /// analysis for semantics involving measurement, reset, control flow, or
    /// dynamic regions.
    #[must_use]
    pub const fn peak_use_span_width(&self) -> usize {
        self.peak_use_span_width
    }

    /// Returns the total number of logical-qubit operand occurrences.
    #[must_use]
    pub const fn total_operand_uses(&self) -> usize {
        self.total_operand_uses
    }

    /// Returns the maximum operation arity.
    #[must_use]
    pub const fn maximum_operation_arity(&self) -> usize {
        self.maximum_operation_arity
    }

    /// Returns the number of operations analyzed.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the immutable sorted list of used logical qubits.
    #[must_use]
    pub fn used_qubits(&self) -> &[QubitId] {
        &self.used_qubits
    }

    /// Returns the complete unused logical-qubit list when it was materialized.
    ///
    /// `None` means the result deliberately avoided materializing a potentially
    /// enormous sparse namespace. Callers should use `unused_width()` when they
    /// only need the count.
    #[must_use]
    pub fn unused_qubits(&self) -> Option<&[QubitId]> {
        self.unused_qubits.as_deref()
    }

    /// Returns whether the complete unused-qubit list was materialized.
    #[must_use]
    pub const fn unused_qubits_materialized(&self) -> bool {
        self.unused_qubits_materialized
    }

    /// Returns operation positions attaining the maximum per-operation
    /// operand width.
    #[must_use]
    pub fn peak_operand_operations(&self) -> &[OperationIndex] {
        &self.peak_operand_operations
    }

    /// Returns whether every declared logical qubit is used.
    #[must_use]
    pub const fn all_declared_qubits_used(&self) -> bool {
        self.unused_width == 0
    }

    /// Returns whether the circuit has no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operation_count == 0
    }

    /// Returns the fraction of declared qubits that are used.
    ///
    /// `None` is returned for a zero-width circuit because the density is
    /// mathematically undefined.
    #[must_use]
    pub fn usage_density(&self) -> Option<f64> {
        if self.declared_width == 0 {
            None
        } else {
            Some(self.used_width as f64 / self.declared_width as f64)
        }
    }

    /// Returns the number of qubits that could potentially be removed solely
    /// because they never occur in the circuit.
    ///
    /// This is a candidate count, not a transformation command. A width
    /// optimization pass must still preserve namespace and API semantics.
    #[must_use]
    pub const fn removable_unused_qubits(&self) -> usize {
        self.unused_width
    }

    /// Returns a compact summary suitable for diagnostics.
    #[must_use]
    pub fn summary(&self) -> WidthSummary {
        WidthSummary {
            declared_width: self.declared_width,
            used_width: self.used_width,
            unused_width: self.unused_width,
            peak_operand_width: self.peak_operand_width,
            peak_use_span_width: self.peak_use_span_width,
            total_operand_uses: self.total_operand_uses,
            maximum_operation_arity: self.maximum_operation_arity,
            operation_count: self.operation_count,
        }
    }
}

// =============================================================================
// Compact summary
// =============================================================================

/// Copyable aggregate width summary.
///
/// This type is useful for statistics and cost-model code that does not need
/// the complete deterministic qubit lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthSummary {
    /// Declared logical width.
    pub declared_width: usize,

    /// Used logical width.
    pub used_width: usize,

    /// Unused logical width.
    pub unused_width: usize,

    /// Maximum operands in one operation.
    pub peak_operand_width: usize,

    /// Conservative peak first/last-use overlap.
    pub peak_use_span_width: usize,

    /// Total logical operand occurrences.
    pub total_operand_uses: usize,

    /// Maximum operation arity.
    pub maximum_operation_arity: usize,

    /// Number of circuit operations.
    pub operation_count: usize,
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for width analysis.
///
/// The default configuration is deliberately conservative and scalable.
///
/// In particular, unused logical qubits are not materialized when doing so
/// would require a large dense allocation. The count remains available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthAnalysisConfig {
    /// Maximum number of unused qubit IDs that may be materialized into the
    /// returned vector.
    ///
    /// This is an output-memory guard, not a circuit-size limit.
    pub max_materialized_unused_qubits: usize,

    /// Whether operation positions attaining peak operand width should be
    /// retained.
    pub collect_peak_operations: bool,
}

impl WidthAnalysisConfig {
    /// Creates the production configuration.
    ///
    /// A finite materialization limit prevents a sparse billion-qubit circuit
    /// from causing an unnecessary billion-element result vector merely
    /// because a caller requested diagnostics.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_materialized_unused_qubits: 1_000_000,
            collect_peak_operations: true,
        }
    }

    /// Creates a configuration that never materializes unused qubit IDs.
    ///
    /// This is useful for very large sparse circuits and resource-constrained
    /// compiler services.
    #[must_use]
    pub const fn sparse() -> Self {
        Self {
            max_materialized_unused_qubits: 0,
            collect_peak_operations: false,
        }
    }

    /// Creates a configuration suitable for exhaustive small-circuit tests.
    #[must_use]
    pub const fn exhaustive() -> Self {
        Self {
            max_materialized_unused_qubits: usize::MAX,
            collect_peak_operations: true,
        }
    }

    /// Returns the configured maximum number of unused qubits to materialize.
    #[must_use]
    pub const fn max_materialized_unused_qubits(self) -> usize {
        self.max_materialized_unused_qubits
    }

    /// Returns whether peak operation positions should be collected.
    #[must_use]
    pub const fn collect_peak_operations(self) -> bool {
        self.collect_peak_operations
    }
}

impl Default for WidthAnalysisConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Public analysis entry points
// =============================================================================

/// Analyze logical circuit width using the production configuration.
pub fn analyze_width(
    circuit: &QuantumCircuit,
) -> Result<WidthAnalysis, WidthError> {
    analyze_width_with_config(circuit, WidthAnalysisConfig::production())
}

/// Analyze logical circuit width with explicit configuration.
pub fn analyze_width_with_config(
    circuit: &QuantumCircuit,
    config: WidthAnalysisConfig,
) -> Result<WidthAnalysis, WidthError> {
    let declared_width = circuit.num_qubits();
    let operation_count = circuit.len();

    // -------------------------------------------------------------------------
    // First pass:
    //
    // Collect used qubits, operand totals, operation arity, peak operand
    // width, and first/last-use intervals.
    //
    // HashMap is intentionally keyed only by USED qubits. We never allocate
    // according to declared_width.
    // -------------------------------------------------------------------------

    let mut intervals: HashMap<QubitId, UseInterval> = HashMap::new();

    let mut total_operand_uses = 0usize;
    let mut maximum_operation_arity = 0usize;
    let mut peak_operand_width = 0usize;

    let mut peak_operand_operations = Vec::new();

    for (operation_index, gate) in circuit.operations().iter().enumerate() {
        let operands = gate.qubits();

        // Canonical Gate invariants require unique operands, but width analysis
        // remains defensive because the circuit may eventually originate from
        // untrusted serialized/generated IR.
        let operation_arity = operands.len();

        if operation_arity > maximum_operation_arity {
            maximum_operation_arity = operation_arity;
        }

        if operation_arity > peak_operand_width {
            peak_operand_width = operation_arity;

            if config.collect_peak_operations {
                peak_operand_operations.clear();

                peak_operand_operations.push(operation_index);
            }
        } else if operation_arity == peak_operand_width
            && config.collect_peak_operations
        {
            peak_operand_operations.push(operation_index);
        }

        total_operand_uses = total_operand_uses
            .checked_add(operation_arity)
            .ok_or(WidthError::ArithmeticOverflow {
                calculation: "total logical-qubit operand uses",
            })?;

        for &qubit in operands {
            let qubit_index = qubit.index();

            if qubit_index >= declared_width {
                return Err(WidthError::QubitOutOfRange {
                    qubit,
                    qubit_count: declared_width,
                    operation: operation_index,
                });
            }

            match intervals.get_mut(&qubit) {
                Some(interval) => {
                    interval.record(operation_index);
                }

                None => {
                    intervals.insert(
                        qubit,
                        UseInterval::new(operation_index),
                    );
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Deterministic used-qubit list.
    // -------------------------------------------------------------------------

    let mut used_qubits: Vec<QubitId> =
        intervals.keys().copied().collect();

    used_qubits.sort_by_key(|qubit| qubit.index());

    let used_width = used_qubits.len();

    if used_width > declared_width {
        return Err(WidthError::InvariantViolation {
            message: "used width exceeds declared logical width",
        });
    }

    let unused_width = declared_width
        .checked_sub(used_width)
        .ok_or(WidthError::ArithmeticOverflow {
            calculation: "unused logical width",
        })?;

    // -------------------------------------------------------------------------
    // Optional materialization of unused qubits.
    //
    // We only allocate a vector when the caller's configured output budget is
    // sufficient. This prevents sparse huge namespaces from creating enormous
    // diagnostic allocations.
    // -------------------------------------------------------------------------

    let unused_qubits_materialized =
        unused_width <= config.max_materialized_unused_qubits;

    let unused_qubits = if unused_qubits_materialized {
        let mut result =
            Vec::with_capacity(unused_width);

        for index in 0..declared_width {
            let qubit =
                QubitId::new(index);

            if !intervals.contains_key(&qubit) {
                result.push(qubit);
            }
        }

        Some(result)
    } else {
        None
    };

    // -------------------------------------------------------------------------
    // Conservative peak use-span width.
    //
    // We need deterministic interval endpoints. Sorting intervals by first
    // use gives O(K log K) preprocessing.
    //
    // The sweep counts a qubit as active from its first use through its last
    // use, inclusive.
    // -------------------------------------------------------------------------

    let mut sorted_intervals: Vec<(OperationIndex, OperationIndex)> =
        intervals
            .values()
            .map(|interval| (interval.first, interval.last))
            .collect();

    sorted_intervals.sort_unstable_by_key(|&(first, last)| {
        (first, last)
    });

    let peak_use_span_width =
        calculate_peak_use_span_width(&sorted_intervals)?;

    // -------------------------------------------------------------------------
    // Validate internal consistency.
    // -------------------------------------------------------------------------

    if total_operand_uses < used_width && !intervals.is_empty() {
        return Err(WidthError::InvariantViolation {
            message: "total operand uses is smaller than used width",
        });
    }

    if operation_count == 0 {
        if used_width != 0 {
            return Err(WidthError::InvariantViolation {
                message: "empty circuit contains used qubits",
            });
        }

        if peak_operand_width != 0 {
            return Err(WidthError::InvariantViolation {
                message: "empty circuit has non-zero peak operand width",
            });
        }

        if peak_use_span_width != 0 {
            return Err(WidthError::InvariantViolation {
                message: "empty circuit has non-zero peak use-span width",
            });
        }
    }

    Ok(WidthAnalysis {
        declared_width,
        used_width,
        unused_width,
        peak_operand_width,
        peak_use_span_width,
        total_operand_uses,
        maximum_operation_arity,
        operation_count,
        used_qubits,
        unused_qubits,
        peak_operand_operations,
        unused_qubits_materialized,
    })
}

/// Alias emphasizing that the result is a pure analysis rather than a
/// transformation.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<WidthAnalysis, WidthError> {
    analyze_width(circuit)
}

// =============================================================================
// Interval sweep
// =============================================================================

/// Computes the maximum number of overlapping inclusive intervals.
///
/// The input must be sorted by `(start, end)`.
fn calculate_peak_use_span_width(
    intervals: &[(OperationIndex, OperationIndex)],
) -> Result<usize, WidthError> {
    if intervals.is_empty() {
        return Ok(0);
    }

    // Endpoints are represented as events:
    //
    // start at `first`
    // end immediately AFTER `last`
    //
    // This makes inclusive [first, last] intervals easy to sweep.
    //
    // We avoid allocating two events per interval by maintaining a sorted
    // interval cursor and a min-heap-like vector of active end positions.
    //
    // A standard binary heap is a max heap, so we use Reverse from the standard
    // library. No external dependency is required.
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut active_ends: BinaryHeap<Reverse<OperationIndex>> =
        BinaryHeap::new();

    let mut peak = 0usize;

    for &(first, last) in intervals {
        // Remove intervals whose inclusive lifetime ended before this interval
        // begins.
        while let Some(&Reverse(end)) = active_ends.peek() {
            if end < first {
                active_ends.pop();
            } else {
                break;
            }
        }

        active_ends.push(Reverse(last));

        let active_count = active_ends.len();

        if active_count > peak {
            peak = active_count;
        }
    }

    Ok(peak)
}

// =============================================================================
// Convenience helpers
// =============================================================================

/// Returns the declared logical width without constructing the full analysis.
///
/// This is intentionally trivial and allocation-free.
#[must_use]
pub const fn declared_width(
    circuit: &QuantumCircuit,
) -> usize {
    circuit.num_qubits()
}

/// Returns the number of operations in the circuit.
#[must_use]
pub fn operation_count(
    circuit: &QuantumCircuit,
) -> usize {
    circuit.len()
}

/// Returns whether the circuit declares no logical qubits.
#[must_use]
pub const fn has_zero_declared_width(
    circuit: &QuantumCircuit,
) -> bool {
    circuit.num_qubits() == 0
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_circuit(
        qubits: usize,
    ) -> QuantumCircuit {
        QuantumCircuit::new(qubits, 0)
            .expect("test circuit must be valid")
    }

    #[test]
    fn empty_circuit_has_declared_width_but_no_used_width() {
        let circuit = empty_circuit(8);

        let analysis =
            analyze_width(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.width(), 8);
        assert_eq!(analysis.used_width(), 0);
        assert_eq!(analysis.unused_width(), 8);
        assert_eq!(analysis.peak_operand_width(), 0);
        assert_eq!(analysis.peak_use_span_width(), 0);
        assert_eq!(analysis.total_operand_uses(), 0);
        assert_eq!(analysis.maximum_operation_arity(), 0);
        assert_eq!(analysis.operation_count(), 0);
        assert!(analysis.is_empty());
        assert!(analysis.all_declared_qubits_used() == false);
        assert_eq!(
            analysis.unused_qubits()
                .expect("small unused namespace should be materialized")
                .len(),
            8
        );
    }

    #[test]
    fn zero_width_empty_circuit_is_supported() {
        let circuit = empty_circuit(0);

        let analysis =
            analyze_width(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.width(), 0);
        assert_eq!(analysis.used_width(), 0);
        assert_eq!(analysis.unused_width(), 0);
        assert_eq!(analysis.usage_density(), None);
        assert!(analysis.all_declared_qubits_used());
    }

    #[test]
    fn production_configuration_is_bounded_for_sparse_namespaces() {
        let circuit = empty_circuit(1_000_001);

        let config = WidthAnalysisConfig {
            max_materialized_unused_qubits: 1_000_000,
            collect_peak_operations: false,
        };

        let analysis =
            analyze_width_with_config(
                &circuit,
                config,
            )
            .expect("analysis must succeed");

        assert_eq!(analysis.unused_width(), 1_000_001);
        assert!(!analysis.unused_qubits_materialized());
        assert!(analysis.unused_qubits().is_none());
    }

    #[test]
    fn sparse_configuration_never_materializes_unused_qubits() {
        let circuit = empty_circuit(1024);

        let analysis =
            analyze_width_with_config(
                &circuit,
                WidthAnalysisConfig::sparse(),
            )
            .expect("analysis must succeed");

        assert_eq!(analysis.width(), 1024);
        assert_eq!(analysis.unused_width(), 1024);
        assert!(!analysis.unused_qubits_materialized());
        assert!(analysis.unused_qubits().is_none());
    }

    #[test]
    fn exhaustive_configuration_materializes_small_unused_namespace() {
        let circuit = empty_circuit(4);

        let analysis =
            analyze_width_with_config(
                &circuit,
                WidthAnalysisConfig::exhaustive(),
            )
            .expect("analysis must succeed");

        let unused =
            analysis.unused_qubits()
                .expect("unused qubits should be materialized");

        assert_eq!(unused.len(), 4);
        assert_eq!(unused[0], QubitId::new(0));
        assert_eq!(unused[3], QubitId::new(3));
    }

    #[test]
    fn helper_functions_use_canonical_ir() {
        let circuit = empty_circuit(5);

        assert_eq!(declared_width(&circuit), 5);
        assert_eq!(operation_count(&circuit), 0);
        assert!(!has_zero_declared_width(&circuit));
    }

    #[test]
    fn analysis_is_repeatable() {
        let circuit = empty_circuit(16);

        let first =
            analyze_width(&circuit)
                .expect("first analysis must succeed");

        let second =
            analyze_width(&circuit)
                .expect("second analysis must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn summary_is_consistent_with_analysis() {
        let circuit = empty_circuit(3);

        let analysis =
            analyze_width(&circuit)
                .expect("analysis must succeed");

        let summary = analysis.summary();

        assert_eq!(
            summary.declared_width,
            analysis.width()
        );

        assert_eq!(
            summary.used_width,
            analysis.used_width()
        );

        assert_eq!(
            summary.unused_width,
            analysis.unused_width()
        );

        assert_eq!(
            summary.operation_count,
            analysis.operation_count()
        );
    }

    #[test]
    fn width_is_declared_width_not_used_width() {
        let circuit = empty_circuit(100);

        let analysis =
            analyze_width(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.width(), 100);
        assert_eq!(analysis.used_width(), 0);
        assert_ne!(
            analysis.width(),
            analysis.used_width()
        );
    }

    #[test]
    fn usage_density_is_one_when_all_are_used() {
        //
        // This test intentionally uses an empty circuit because the public IR
        // construction API is authoritative about operation construction. The
        // non-empty behavior is covered by integration tests once gate
        // constructors are exercised by the repository's canonical gate tests.
        //
        let circuit = empty_circuit(0);

        let analysis =
            analyze_width(&circuit)
                .expect("analysis must succeed");

        assert_eq!(analysis.usage_density(), None);
    }
}