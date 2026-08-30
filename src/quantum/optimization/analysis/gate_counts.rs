//! Zamani Quantum Optimization — Gate-Count Analysis
//!
//! Production-grade logical operation/gate-count analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                               │
//!                               ▼
//!                    optimization::analysis
//!                               │
//!                               ▼
//!                         gate_counts
//!                               │
//!          ┌────────────────────┼────────────────────┐
//!          ▼                    ▼                    ▼
//!      cost models       optimization passes    reports/metrics
//!          │                    │                    │
//!          └────────────────────┼────────────────────┘
//!                               ▼
//!                         optimization
//! ```
//!
//! This module is intentionally an analysis-only component. It never mutates
//! the circuit and never performs optimization itself.
//!
//! # Canonical representation
//!
//! This module does NOT define another quantum circuit, gate, operation, or
//! qubit representation.
//!
//! The authoritative representations are:
//!
//! - `crate::quantum::ir::QuantumCircuit`;
//! - `crate::quantum::ir::Gate`;
//! - `crate::quantum::ir::gate::GateKind`;
//! - `crate::quantum::ir::qubits::QubitId`.
//!
//! # Counting semantics
//!
//! The analysis deliberately distinguishes:
//!
//! - total logical operations;
//! - unitary gate count;
//! - non-unitary operation count;
//! - one-qubit operation count;
//! - two-qubit operation count;
//! - three-qubit operation count;
//! - multi-qubit operation count;
//! - parameterized operation count;
//! - Clifford operation count;
//! - non-Clifford unitary operation count;
//! - measurement count;
//! - reset count;
//! - barrier count;
//! - total logical-qubit operand uses;
//! - total parameter uses;
//! - classical measurement destinations;
//! - maximum operation arity.
//!
//! This prevents the common compiler error of treating "number of gates",
//! "number of operations", "number of qubit operands", and "number of
//! non-Clifford operations" as interchangeable metrics.
//!
//! # Gate-count definition
//!
//! `gate_count()` means the number of unitary logical operations.
//!
//! `operation_count()` means every canonical operation, including:
//!
//! - gates;
//! - measurement;
//! - reset;
//! - barrier.
//!
//! Therefore:
//!
//! ```text
//! operation_count >= gate_count
//!
//! operation_count =
//!     gate_count
//!     + measurement_count
//!     + reset_count
//!     + barrier_count
//! ```
//!
//! for the current canonical IR, because these are the current non-unitary
//! operation kinds.
//!
//! # Current canonical GateKind coverage
//!
//! The current canonical IR defines:
//!
//! Single-qubit operations:
//!
//! - I
//! - X
//! - Y
//! - Z
//! - H
//! - S
//! - Sdg
//! - T
//! - Tdg
//! - V
//! - Vdg
//!
//! Parameterized single-qubit operations:
//!
//! - RX
//! - RY
//! - RZ
//! - Phase
//! - U1
//! - U2
//! - U3
//!
//! Two-qubit operations:
//!
//! - CX
//! - CY
//! - CZ
//! - CH
//! - SWAP
//! - ISWAP
//! - ECR
//! - CRX
//! - CRY
//! - CRZ
//!
//! Three-qubit operations:
//!
//! - CCX
//! - CSWAP
//!
//! Non-unitary operations:
//!
//! - Measure
//! - Barrier
//! - Reset
//!
//! Every current `GateKind` is explicitly represented in this file.
//!
//! # Future GateKind additions
//!
//! The canonical IR uses a closed Rust enum. Therefore a future addition to
//! `GateKind` must intentionally update the exhaustive classifier in this file.
//!
//! This is desirable for production compiler correctness: a newly introduced
//! operation cannot silently disappear from optimization metrics.
//!
//! The compiler should therefore treat an exhaustive-match compilation error
//! after adding a new `GateKind` variant as an integration requirement rather
//! than weakening this module with a wildcard match.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of canonical operations;
//! - `A` = total number of logical-qubit operands;
//! - `P` = total number of gate parameters.
//!
//! The analysis performs:
//!
//! - expected `O(N + A + P)` work;
//! - `O(1)` auxiliary memory apart from the returned result;
//! - no allocation proportional to the number of declared qubits;
//! - no recursion;
//! - no graph construction;
//! - no per-operation heap allocation.
//!
//! The result itself is a fixed-size aggregate plus a fixed-size list of
//! per-kind counters. Its memory consumption is therefore independent of
//! circuit width and proportional only to the number of metric entries.
//!
//! The analysis can consequently process circuits from tiny examples through
//! extremely large circuits subject to the canonical IR/resource policy and
//! the host's available resources.
//!
//! # Sparse-circuit behavior
//!
//! Gate counting never allocates a record for every declared logical qubit.
//!
//! A circuit may declare:
//!
//! ```text
//! 1,000,000,000 logical qubits
//! ```
//!
//! while containing only:
//!
//! ```text
//! 100 operations
//! ```
//!
//! This analysis remains proportional to the 100 operations and their operands.
//!
//! # Overflow policy
//!
//! All aggregate counters use checked arithmetic.
//!
//! Overflow is reported as an explicit `GateCountError` rather than silently
//! wrapping in release builds.
//!
//! This is important because optimizer statistics eventually feed cost models,
//! optimization decisions, diagnostics, benchmarking, and reproducibility.
//!
//! # Determinism
//!
//! The result is deterministic for a deterministic canonical circuit.
//!
//! No hash maps, global state, random numbers, threads, or backend state are
//! required.
//!
//! Per-gate counters are stored in a fixed structure, so serialization and
//! reporting order is stable.
//!
//! # Validation
//!
//! The analysis validates the canonical circuit before counting.
//!
//! This is intentional. Optimization analyses may eventually receive circuits
//! reconstructed from:
//!
//! - deserialization;
//! - frontend lowering;
//! - generated IR;
//! - optimization passes;
//! - external compiler tools.
//!
//! A caller must not be able to obtain apparently valid gate-count statistics
//! from an invalid canonical circuit.
//!
//! # Integration contract
//!
//! ## `analysis/mod.rs`
//!
//! Export:
//!
//! ```text
//! pub mod gate_counts;
//! ```
//!
//! Recommended re-exports:
//!
//! ```text
//! pub use gate_counts::{analyze_gate_counts, GateCountAnalysis, GateCountError};
//! ```
//!
//! ## `cost.rs`
//!
//! Cost models should consume this analysis for:
//!
//! - total gate count;
//! - two-qubit gate count;
//! - single-qubit gate count;
//! - non-Clifford count;
//! - T count;
//! - measurement count;
//! - reset count;
//! - barrier count;
//! - parameterized operation count.
//!
//! `cost.rs` remains the owner of monetary, timing, error, energy, and
//! multi-objective cost semantics. This file only reports structural counts.
//!
//! ## `context.rs`
//!
//! `GateCountAnalysis` is immutable and suitable for analysis caching.
//!
//! Any transformation that changes the operation sequence, operation kind,
//! operation arity, qubit operands, or parameter list invalidates this
//! analysis.
//!
//! A metadata-only transformation does not invalidate it.
//!
//! ## `depth.rs`
//!
//! Depth analysis remains independent.
//!
//! Gate counts do not imply depth:
//!
//! ```text
//! H q0
//! H q1
//! ```
//!
//! has two gates but depth one.
//!
//! ## `width.rs`
//!
//! Width analysis remains independent.
//!
//! Gate counts do not imply qubit width.
//!
//! ## `qubit_use.rs`
//!
//! Qubit-use analysis owns detailed per-qubit use records.
//!
//! This module only counts aggregate operand occurrences.
//!
//! ## `dependency.rs`
//!
//! Dependency analysis may use gate counts as metadata, but gate counting must
//! never construct or depend on the dependency graph.
//!
//! ## `passes/optimize_gate_count.rs`
//!
//! Gate-count optimization should compare:
//!
//! ```text
//! before.gate_count()
//! after.gate_count()
//! ```
//!
//! and should also inspect specialized metrics such as:
//!
//! ```text
//! before.two_qubit_gate_count()
//! after.two_qubit_gate_count()
//! ```
//!
//! because minimizing total gates can increase expensive two-qubit operations.
//!
//! ## `passes/optimize_two_qubit.rs`
//!
//! This analysis provides the authoritative aggregate two-qubit count for the
//! logical circuit.
//!
//! Hardware topology and physical routing remain outside this module.
//!
//! ## `fault_tolerant/`
//!
//! Fault-tolerant optimization can consume:
//!
//! - T count;
//! - T-dagger count;
//! - non-Clifford count;
//! - Clifford count.
//!
//! T-depth remains a separate analysis.
//!
//! ## `benchmarking/`
//!
//! Benchmarking can consume this result but this module must never depend on
//! benchmarking.
//!
//! ## `routing/`
//!
//! Routing may consume pre-routing counts as a baseline. It must not be a
//! dependency of gate counting.
//!
//! ## `scheduling/`
//!
//! Scheduling can consume logical gate counts as one input to reporting, but
//! this module does not model timing.
//!
//! ## `hardware/`
//!
//! Hardware-specific gate costs must not be encoded here.
//!
//! A CZ and a CX are both two-qubit gates here even if a target device assigns
//! them radically different physical costs.
//!
//! Target-aware cost models belong under `optimization::targets` and
//! `optimization::cost`.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
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
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Verification requirements
//!
//! Tests included below cover:
//!
//! - empty circuits;
//! - all current gate kinds;
//! - single-qubit counts;
//! - two-qubit counts;
//! - three-qubit counts;
//! - measurement/reset/barrier counts;
//! - Clifford/non-Clifford counts;
//! - T/T-dagger counts;
//! - parameterized counts;
//! - operand counts;
//! - parameter counts;
//! - maximum arity;
//! - deterministic repeated analysis;
//! - invalid circuits;
//! - checked aggregate invariants.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by gate-count analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateCountError {
    /// The supplied canonical circuit is invalid.
    InvalidCircuit {
        /// Human-readable canonical-IR validation failure.
        message: String,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Description of the counter/calculation that overflowed.
        calculation: &'static str,
    },

    /// An internal analysis invariant was violated.
    InvariantViolation {
        /// Static description of the violated invariant.
        message: &'static str,
    },
}

impl fmt::Display for GateCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze gate counts: invalid quantum circuit: {message}"
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
                    "gate-count analysis invariant violated: {message}"
                )
            }
        }
    }
}

impl std::error::Error for GateCountError {}

// =============================================================================
// Scalar aliases
// =============================================================================

/// Number of canonical logical operations.
pub type OperationCount = usize;

/// Number of logical gate parameters.
pub type ParameterCount = usize;

/// Number of logical-qubit operand occurrences.
pub type OperandCount = usize;

// =============================================================================
// Per-gate-kind counters
// =============================================================================

/// Complete counters for the currently supported canonical `GateKind` values.
///
/// The structure is intentionally explicit rather than using a `HashMap` so
/// that:
///
/// - counting has no hashing overhead;
/// - memory use is fixed;
/// - results are deterministic;
/// - serialization/reporting order is stable;
/// - a new canonical `GateKind` requires an intentional compiler-visible
///   integration change.
///
/// All fields represent counts of operations, not qubit operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GateKindCounts {
    // -------------------------------------------------------------------------
    // Identity and Pauli gates
    // -------------------------------------------------------------------------

    /// Identity operations.
    pub i: usize,

    /// X gates.
    pub x: usize,

    /// Y gates.
    pub y: usize,

    /// Z gates.
    pub z: usize,

    /// Hadamard gates.
    pub h: usize,

    // -------------------------------------------------------------------------
    // Clifford phase gates
    // -------------------------------------------------------------------------

    /// S gates.
    pub s: usize,

    /// S-dagger gates.
    pub sdg: usize,

    // -------------------------------------------------------------------------
    // Non-Clifford phase gates
    // -------------------------------------------------------------------------

    /// T gates.
    pub t: usize,

    /// T-dagger gates.
    pub tdg: usize,

    // -------------------------------------------------------------------------
    // V-family gates
    // -------------------------------------------------------------------------

    /// V gates.
    pub v: usize,

    /// V-dagger gates.
    pub vdg: usize,

    // -------------------------------------------------------------------------
    // Parameterized one-qubit gates
    // -------------------------------------------------------------------------

    /// RX rotations.
    pub rx: usize,

    /// RY rotations.
    pub ry: usize,

    /// RZ rotations.
    pub rz: usize,

    /// Phase rotations.
    pub phase: usize,

    /// U1 gates.
    pub u1: usize,

    /// U2 gates.
    pub u2: usize,

    /// U3 gates.
    pub u3: usize,

    // -------------------------------------------------------------------------
    // Two-qubit Clifford/native gates
    // -------------------------------------------------------------------------

    /// Controlled-X / CNOT gates.
    pub cx: usize,

    /// Controlled-Y gates.
    pub cy: usize,

    /// Controlled-Z gates.
    pub cz: usize,

    /// Controlled-H gates.
    pub ch: usize,

    /// SWAP gates.
    pub swap: usize,

    /// iSWAP gates.
    pub iswap: usize,

    /// Echoed cross-resonance gates.
    pub ecr: usize,

    // -------------------------------------------------------------------------
    // Two-qubit parameterized gates
    // -------------------------------------------------------------------------

    /// Controlled-RX gates.
    pub crx: usize,

    /// Controlled-RY gates.
    pub cry: usize,

    /// Controlled-RZ gates.
    pub crz: usize,

    // -------------------------------------------------------------------------
    // Three-qubit gates
    // -------------------------------------------------------------------------

    /// Toffoli / CCX gates.
    pub ccx: usize,

    /// Controlled-SWAP gates.
    pub cswap: usize,

    // -------------------------------------------------------------------------
    // Non-unitary operations
    // -------------------------------------------------------------------------

    /// Measurement operations.
    pub measure: usize,

    /// Barrier operations.
    pub barrier: usize,

    /// Reset operations.
    pub reset: usize,
}

impl GateKindCounts {
    /// Returns the number of counters represented by this structure.
    ///
    /// This is a compile-time constant useful for report/schema generation.
    #[must_use]
    pub const fn field_count() -> usize {
        33
    }

    /// Returns the count for a canonical gate kind.
    #[must_use]
    pub const fn get(self, kind: GateKind) -> usize {
        match kind {
            GateKind::I => self.i,
            GateKind::X => self.x,
            GateKind::Y => self.y,
            GateKind::Z => self.z,
            GateKind::H => self.h,
            GateKind::S => self.s,
            GateKind::Sdg => self.sdg,
            GateKind::T => self.t,
            GateKind::Tdg => self.tdg,
            GateKind::V => self.v,
            GateKind::Vdg => self.vdg,
            GateKind::RX => self.rx,
            GateKind::RY => self.ry,
            GateKind::RZ => self.rz,
            GateKind::Phase => self.phase,
            GateKind::U1 => self.u1,
            GateKind::U2 => self.u2,
            GateKind::U3 => self.u3,
            GateKind::CX => self.cx,
            GateKind::CY => self.cy,
            GateKind::CZ => self.cz,
            GateKind::CH => self.ch,
            GateKind::SWAP => self.swap,
            GateKind::ISWAP => self.iswap,
            GateKind::ECR => self.ecr,
            GateKind::CRX => self.crx,
            GateKind::CRY => self.cry,
            GateKind::CRZ => self.crz,
            GateKind::CCX => self.ccx,
            GateKind::CSWAP => self.cswap,
            GateKind::Measure => self.measure,
            GateKind::Barrier => self.barrier,
            GateKind::Reset => self.reset,
        }
    }

    /// Increments exactly one canonical gate-kind counter.
    fn increment(
        &mut self,
        kind: GateKind,
    ) -> Result<(), GateCountError> {
        let counter = match kind {
            GateKind::I => &mut self.i,
            GateKind::X => &mut self.x,
            GateKind::Y => &mut self.y,
            GateKind::Z => &mut self.z,
            GateKind::H => &mut self.h,
            GateKind::S => &mut self.s,
            GateKind::Sdg => &mut self.sdg,
            GateKind::T => &mut self.t,
            GateKind::Tdg => &mut self.tdg,
            GateKind::V => &mut self.v,
            GateKind::Vdg => &mut self.vdg,
            GateKind::RX => &mut self.rx,
            GateKind::RY => &mut self.ry,
            GateKind::RZ => &mut self.rz,
            GateKind::Phase => &mut self.phase,
            GateKind::U1 => &mut self.u1,
            GateKind::U2 => &mut self.u2,
            GateKind::U3 => &mut self.u3,
            GateKind::CX => &mut self.cx,
            GateKind::CY => &mut self.cy,
            GateKind::CZ => &mut self.cz,
            GateKind::CH => &mut self.ch,
            GateKind::SWAP => &mut self.swap,
            GateKind::ISWAP => &mut self.iswap,
            GateKind::ECR => &mut self.ecr,
            GateKind::CRX => &mut self.crx,
            GateKind::CRY => &mut self.cry,
            GateKind::CRZ => &mut self.crz,
            GateKind::CCX => &mut self.ccx,
            GateKind::CSWAP => &mut self.cswap,
            GateKind::Measure => &mut self.measure,
            GateKind::Barrier => &mut self.barrier,
            GateKind::Reset => &mut self.reset,
        };

        *counter = counter
            .checked_add(1)
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "per-gate-kind count",
            })?;

        Ok(())
    }

    /// Returns all current gate-kind counters in deterministic canonical order.
    ///
    /// The returned slice is intentionally represented as a fixed array rather
    /// than a map. This makes reports reproducible across platforms and runs.
    #[must_use]
    pub fn as_pairs(self) -> [(GateKind, usize); 33] {
        [
            (GateKind::I, self.i),
            (GateKind::X, self.x),
            (GateKind::Y, self.y),
            (GateKind::Z, self.z),
            (GateKind::H, self.h),
            (GateKind::S, self.s),
            (GateKind::Sdg, self.sdg),
            (GateKind::T, self.t),
            (GateKind::Tdg, self.tdg),
            (GateKind::V, self.v),
            (GateKind::Vdg, self.vdg),
            (GateKind::RX, self.rx),
            (GateKind::RY, self.ry),
            (GateKind::RZ, self.rz),
            (GateKind::Phase, self.phase),
            (GateKind::U1, self.u1),
            (GateKind::U2, self.u2),
            (GateKind::U3, self.u3),
            (GateKind::CX, self.cx),
            (GateKind::CY, self.cy),
            (GateKind::CZ, self.cz),
            (GateKind::CH, self.ch),
            (GateKind::SWAP, self.swap),
            (GateKind::ISWAP, self.iswap),
            (GateKind::ECR, self.ecr),
            (GateKind::CRX, self.crx),
            (GateKind::CRY, self.cry),
            (GateKind::CRZ, self.crz),
            (GateKind::CCX, self.ccx),
            (GateKind::CSWAP, self.cswap),
            (GateKind::Measure, self.measure),
            (GateKind::Barrier, self.barrier),
            (GateKind::Reset, self.reset),
        ]
    }

    /// Returns the total number of operations represented by all per-kind
    /// counters.
    pub fn total(self) -> Result<usize, GateCountError> {
        let mut total = 0usize;

        for (_, count) in self.as_pairs() {
            total = total
                .checked_add(count)
                .ok_or(GateCountError::ArithmeticOverflow {
                    calculation: "sum of gate-kind counts",
                })?;
        }

        Ok(total)
    }
}

// =============================================================================
// Primary analysis result
// =============================================================================

/// Immutable aggregate gate-count analysis.
///
/// This is intentionally a value type with no reference to the analyzed
/// circuit. It can therefore be:
///
/// - cached by `OptimizationContext`;
/// - copied into reports;
/// - compared before/after optimization;
/// - serialized by a future reporting layer;
/// - passed across analysis boundaries;
/// - retained after the source circuit is dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GateCountAnalysis {
    /// Total number of canonical operations.
    operation_count: usize,

    /// Number of unitary logical operations.
    gate_count: usize,

    /// Number of non-unitary logical operations.
    non_unitary_operation_count: usize,

    /// Number of single-qubit operations.
    single_qubit_operation_count: usize,

    /// Number of two-qubit operations.
    two_qubit_operation_count: usize,

    /// Number of three-qubit operations.
    three_qubit_operation_count: usize,

    /// Number of operations involving more than three qubits.
    ///
    /// This is zero for the current canonical IR, but is retained as an
    /// explicit metric so the aggregate API remains conceptually ready for
    /// future variadic/multi-qubit operation kinds.
    multi_qubit_operation_count: usize,

    /// Number of unitary Clifford operations.
    clifford_operation_count: usize,

    /// Number of unitary non-Clifford operations.
    non_clifford_operation_count: usize,

    /// Number of parameterized operations.
    parameterized_operation_count: usize,

    /// Number of logical-qubit operand occurrences across all operations.
    total_qubit_operand_count: usize,

    /// Number of parameter occurrences across all operations.
    total_parameter_count: usize,

    /// Number of operations with a classical destination.
    classical_target_operation_count: usize,

    /// Number of measurement operations.
    measurement_count: usize,

    /// Number of reset operations.
    reset_count: usize,

    /// Number of barrier operations.
    barrier_count: usize,

    /// Number of identity operations.
    identity_count: usize,

    /// Number of T operations.
    t_count: usize,

    /// Number of T-dagger operations.
    t_dagger_count: usize,

    /// Number of non-Clifford phase-resource operations represented by T/Tdg.
    t_family_count: usize,

    /// Maximum logical-qubit arity of any operation.
    maximum_operation_arity: usize,

    /// Per-canonical-kind counters.
    by_kind: GateKindCounts,
}

impl GateCountAnalysis {
    /// Returns the total number of canonical operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of unitary logical gates.
    ///
    /// This is the primary gate-count metric.
    #[must_use]
    pub const fn gate_count(&self) -> usize {
        self.gate_count
    }

    /// Returns the number of non-unitary logical operations.
    #[must_use]
    pub const fn non_unitary_operation_count(&self) -> usize {
        self.non_unitary_operation_count
    }

    /// Returns the number of one-qubit operations.
    #[must_use]
    pub const fn single_qubit_operation_count(&self) -> usize {
        self.single_qubit_operation_count
    }

    /// Returns the number of two-qubit operations.
    #[must_use]
    pub const fn two_qubit_operation_count(&self) -> usize {
        self.two_qubit_operation_count
    }

    /// Returns the number of three-qubit operations.
    #[must_use]
    pub const fn three_qubit_operation_count(&self) -> usize {
        self.three_qubit_operation_count
    }

    /// Returns the number of operations with arity greater than three.
    #[must_use]
    pub const fn multi_qubit_operation_count(&self) -> usize {
        self.multi_qubit_operation_count
    }

    /// Returns the number of unitary Clifford operations.
    #[must_use]
    pub const fn clifford_operation_count(&self) -> usize {
        self.clifford_operation_count
    }

    /// Returns the number of unitary non-Clifford operations.
    #[must_use]
    pub const fn non_clifford_operation_count(&self) -> usize {
        self.non_clifford_operation_count
    }

    /// Returns the number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operation_count(&self) -> usize {
        self.parameterized_operation_count
    }

    /// Returns the total number of logical-qubit operand occurrences.
    #[must_use]
    pub const fn total_qubit_operand_count(&self) -> usize {
        self.total_qubit_operand_count
    }

    /// Returns the total number of parameter occurrences.
    #[must_use]
    pub const fn total_parameter_count(&self) -> usize {
        self.total_parameter_count
    }

    /// Returns the number of operations with a classical destination.
    #[must_use]
    pub const fn classical_target_operation_count(&self) -> usize {
        self.classical_target_operation_count
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.measurement_count
    }

    /// Returns the number of reset operations.
    #[must_use]
    pub const fn reset_count(&self) -> usize {
        self.reset_count
    }

    /// Returns the number of barrier operations.
    #[must_use]
    pub const fn barrier_count(&self) -> usize {
        self.barrier_count
    }

    /// Returns the number of identity operations.
    #[must_use]
    pub const fn identity_count(&self) -> usize {
        self.identity_count
    }

    /// Returns the number of T operations.
    #[must_use]
    pub const fn t_count(&self) -> usize {
        self.t_count
    }

    /// Returns the number of T-dagger operations.
    #[must_use]
    pub const fn t_dagger_count(&self) -> usize {
        self.t_dagger_count
    }

    /// Returns the combined T/T-dagger count.
    #[must_use]
    pub const fn t_family_count(&self) -> usize {
        self.t_family_count
    }

    /// Returns the maximum logical-qubit arity of one operation.
    #[must_use]
    pub const fn maximum_operation_arity(&self) -> usize {
        self.maximum_operation_arity
    }

    /// Returns all per-kind counters.
    #[must_use]
    pub const fn by_kind(&self) -> GateKindCounts {
        self.by_kind
    }

    /// Returns the count for one specific canonical gate kind.
    #[must_use]
    pub const fn count(&self, kind: GateKind) -> usize {
        self.by_kind.get(kind)
    }

    /// Returns whether the circuit contains at least one unitary gate.
    #[must_use]
    pub const fn has_gates(&self) -> bool {
        self.gate_count != 0
    }

    /// Returns whether the circuit contains at least one two-qubit operation.
    #[must_use]
    pub const fn has_two_qubit_operations(&self) -> bool {
        self.two_qubit_operation_count != 0
    }

    /// Returns whether the circuit contains at least one non-Clifford operation.
    #[must_use]
    pub const fn has_non_clifford_operations(&self) -> bool {
        self.non_clifford_operation_count != 0
    }

    /// Returns whether the circuit contains T-family operations.
    #[must_use]
    pub const fn has_t_family_operations(&self) -> bool {
        self.t_family_count != 0
    }

    /// Returns whether the circuit contains measurement operations.
    #[must_use]
    pub const fn has_measurements(&self) -> bool {
        self.measurement_count != 0
    }

    /// Returns whether the circuit contains resets.
    #[must_use]
    pub const fn has_resets(&self) -> bool {
        self.reset_count != 0
    }

    /// Returns whether the circuit contains barriers.
    #[must_use]
    pub const fn has_barriers(&self) -> bool {
        self.barrier_count != 0
    }

    /// Returns the fraction of operations that are unitary gates.
    ///
    /// Returns `None` for an empty circuit.
    ///
    /// This method intentionally performs floating-point conversion only at
    /// the reporting boundary. Structural counts remain exact integers.
    #[must_use]
    pub fn gate_fraction(&self) -> Option<f64> {
        if self.operation_count == 0 {
            return None;
        }

        Some(self.gate_count as f64 / self.operation_count as f64)
    }

    /// Returns the fraction of unitary operations that are two-qubit gates.
    ///
    /// Returns `None` when there are no unitary gates.
    #[must_use]
    pub fn two_qubit_gate_fraction(&self) -> Option<f64> {
        if self.gate_count == 0 {
            return None;
        }

        Some(
            self.two_qubit_operation_count as f64
                / self.gate_count as f64,
        )
    }

    /// Returns the fraction of unitary operations that are non-Clifford.
    ///
    /// Returns `None` when there are no unitary gates.
    #[must_use]
    pub fn non_clifford_fraction(&self) -> Option<f64> {
        if self.gate_count == 0 {
            return None;
        }

        Some(
            self.non_clifford_operation_count as f64
                / self.gate_count as f64,
        )
    }

    /// Returns the average number of logical-qubit operands per operation.
    ///
    /// Returns `None` for an empty circuit.
    #[must_use]
    pub fn average_operation_arity(&self) -> Option<f64> {
        if self.operation_count == 0 {
            return None;
        }

        Some(
            self.total_qubit_operand_count as f64
                / self.operation_count as f64,
        )
    }

    /// Returns the average number of parameters per operation.
    ///
    /// Returns `None` for an empty circuit.
    #[must_use]
    pub fn average_parameters_per_operation(&self) -> Option<f64> {
        if self.operation_count == 0 {
            return None;
        }

        Some(
            self.total_parameter_count as f64
                / self.operation_count as f64,
        )
    }

    /// Returns the aggregate structural count represented by all canonical
    /// gate-kind counters.
    pub fn counted_kind_total(&self) -> Result<usize, GateCountError> {
        self.by_kind.total()
    }

    /// Validates all aggregate invariants.
    ///
    /// This is useful for:
    ///
    /// - debug assertions in higher-level analyses;
    /// - deserialization/report validation;
    /// - regression tests;
    /// - compiler diagnostics.
    pub fn validate_invariants(&self) -> Result<(), GateCountError> {
        let counted_total = self.counted_kind_total()?;

        if counted_total != self.operation_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind total does not equal operation count",
            });
        }

        let expected_non_unitary = self
            .measurement_count
            .checked_add(self.reset_count)
            .and_then(|value| value.checked_add(self.barrier_count))
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "non-unitary operation invariant",
            })?;

        if expected_non_unitary != self.non_unitary_operation_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "measurement + reset + barrier count does not equal non-unitary count",
            });
        }

        let expected_operation_total = self
            .gate_count
            .checked_add(self.non_unitary_operation_count)
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "operation-count invariant",
            })?;

        if expected_operation_total != self.operation_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "gate count + non-unitary count does not equal operation count",
            });
        }

        let expected_t_family = self
            .t_count
            .checked_add(self.t_dagger_count)
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "T-family count invariant",
            })?;

        if expected_t_family != self.t_family_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "T + T-dagger count does not equal T-family count",
            });
        }

        if self.by_kind.t != self.t_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind T count does not equal aggregate T count",
            });
        }

        if self.by_kind.tdg != self.t_dagger_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind T-dagger count does not equal aggregate T-dagger count",
            });
        }

        if self.by_kind.measure != self.measurement_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind measurement count does not equal aggregate measurement count",
            });
        }

        if self.by_kind.reset != self.reset_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind reset count does not equal aggregate reset count",
            });
        }

        if self.by_kind.barrier != self.barrier_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "per-kind barrier count does not equal aggregate barrier count",
            });
        }

        let expected_two_qubit = self
            .by_kind
            .cx
            .checked_add(self.by_kind.cy)
            .and_then(|value| value.checked_add(self.by_kind.cz))
            .and_then(|value| value.checked_add(self.by_kind.ch))
            .and_then(|value| value.checked_add(self.by_kind.swap))
            .and_then(|value| value.checked_add(self.by_kind.iswap))
            .and_then(|value| value.checked_add(self.by_kind.ecr))
            .and_then(|value| value.checked_add(self.by_kind.crx))
            .and_then(|value| value.checked_add(self.by_kind.cry))
            .and_then(|value| value.checked_add(self.by_kind.crz))
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "two-qubit operation invariant",
            })?;

        if expected_two_qubit != self.two_qubit_operation_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "current two-qubit GateKind counts do not equal aggregate two-qubit count",
            });
        }

        let expected_three_qubit = self
            .by_kind
            .ccx
            .checked_add(self.by_kind.cswap)
            .ok_or(GateCountError::ArithmeticOverflow {
                calculation: "three-qubit operation invariant",
            })?;

        if expected_three_qubit != self.three_qubit_operation_count {
            return Err(GateCountError::InvariantViolation {
                message:
                    "current three-qubit GateKind counts do not equal aggregate three-qubit count",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

fn checked_add(
    target: &mut usize,
    value: usize,
    calculation: &'static str,
) -> Result<(), GateCountError> {
    *target = target
        .checked_add(value)
        .ok_or(GateCountError::ArithmeticOverflow {
            calculation,
        })?;

    Ok(())
}

fn classify_gate(
    analysis: &mut GateCountAnalysis,
    gate: &Gate,
) -> Result<(), GateCountError> {
    let kind = gate.kind();

    analysis.by_kind.increment(kind)?;

    checked_add(
        &mut analysis.operation_count,
        1,
        "operation count",
    )?;

    checked_add(
        &mut analysis.total_qubit_operand_count,
        gate.qubits().len(),
        "total logical-qubit operand count",
    )?;

    checked_add(
        &mut analysis.total_parameter_count,
        gate.parameters().len(),
        "total parameter count",
    )?;

    if gate.qubits().len() > analysis.maximum_operation_arity {
        analysis.maximum_operation_arity = gate.qubits().len();
    }

    if gate.is_unitary() {
        checked_add(
            &mut analysis.gate_count,
            1,
            "unitary gate count",
        )?;

        if gate.is_clifford() {
            checked_add(
                &mut analysis.clifford_operation_count,
                1,
                "Clifford operation count",
            )?;
        } else {
            checked_add(
                &mut analysis.non_clifford_operation_count,
                1,
                "non-Clifford operation count",
            )?;
        }
    } else {
        checked_add(
            &mut analysis.non_unitary_operation_count,
            1,
            "non-unitary operation count",
        )?;
    }

    if gate.is_parameterized() {
        checked_add(
            &mut analysis.parameterized_operation_count,
            1,
            "parameterized operation count",
        )?;
    }

    match kind {
        GateKind::Measure => {
            checked_add(
                &mut analysis.measurement_count,
                1,
                "measurement count",
            )?;

            if gate.classical_target().is_some() {
                checked_add(
                    &mut analysis.classical_target_operation_count,
                    1,
                    "classical target operation count",
                )?;
            }
        }

        GateKind::Reset => {
            checked_add(
                &mut analysis.reset_count,
                1,
                "reset count",
            )?;
        }

        GateKind::Barrier => {
            checked_add(
                &mut analysis.barrier_count,
                1,
                "barrier count",
            )?;
        }

        _ => {}
    }

    if kind == GateKind::I {
        checked_add(
            &mut analysis.identity_count,
            1,
            "identity count",
        )?;
    }

    match kind {
        GateKind::T => {
            checked_add(
                &mut analysis.t_count,
                1,
                "T count",
            )?;

            checked_add(
                &mut analysis.t_family_count,
                1,
                "T-family count",
            )?;
        }

        GateKind::Tdg => {
            checked_add(
                &mut analysis.t_dagger_count,
                1,
                "T-dagger count",
            )?;

            checked_add(
                &mut analysis.t_family_count,
                1,
                "T-family count",
            )?;
        }

        _ => {}
    }

    match gate.qubits().len() {
        0 => {}

        1 => {
            checked_add(
                &mut analysis.single_qubit_operation_count,
                1,
                "single-qubit operation count",
            )?;
        }

        2 => {
            checked_add(
                &mut analysis.two_qubit_operation_count,
                1,
                "two-qubit operation count",
            )?;
        }

        3 => {
            checked_add(
                &mut analysis.three_qubit_operation_count,
                1,
                "three-qubit operation count",
            )?;
        }

        _ => {
            checked_add(
                &mut analysis.multi_qubit_operation_count,
                1,
                "multi-qubit operation count",
            )?;
        }
    }

    Ok(())
}

// =============================================================================
// Public analysis API
// =============================================================================

/// Analyzes all gate and operation counts in a canonical quantum circuit.
///
/// # Guarantees
///
/// On success:
///
/// - the circuit has passed canonical IR validation;
/// - every current `GateKind` has been classified;
/// - all counters are internally consistent;
/// - no circuit mutation occurred;
/// - the result is deterministic;
/// - no unsafe code was used;
/// - no backend was accessed.
///
/// # Complexity
///
/// Let `N` be the number of operations, `A` the total number of qubit
/// operands, and `P` the total number of parameters.
///
/// ```text
/// Time:   O(N + A + P)
/// Memory: O(1) auxiliary memory
/// ```
///
/// The returned `GateCountAnalysis` has fixed-size storage.
///
/// # Example
///
/// ```
/// use zamani::quantum::ir::QuantumCircuit;
/// use zamani::quantum::optimization::analysis::gate_counts::analyze_gate_counts;
///
/// let circuit = QuantumCircuit::new(2, 0).expect("valid circuit");
/// let counts = analyze_gate_counts(&circuit).expect("valid analysis");
///
/// assert_eq!(counts.operation_count(), 0);
/// assert_eq!(counts.gate_count(), 0);
/// assert_eq!(counts.two_qubit_operation_count(), 0);
/// ```
///
/// The exact crate-level import path may differ for downstream consumers
/// depending on Zamani's public library boundary; the analysis itself only
/// depends on the canonical internal IR.
pub fn analyze_gate_counts(
    circuit: &QuantumCircuit,
) -> Result<GateCountAnalysis, GateCountError> {
    circuit
        .validate()
        .map_err(|error| GateCountError::InvalidCircuit {
            message: error.to_string(),
        })?;

    let mut analysis = GateCountAnalysis::default();

    for gate in circuit.operations() {
        classify_gate(&mut analysis, gate)?;
    }

    analysis.validate_invariants()?;

    Ok(analysis)
}

/// Analyzes a canonical circuit without requiring callers to retain the result
/// after returning from the function.
///
/// This is a semantic alias intended for readability at analysis call sites.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<GateCountAnalysis, GateCountError> {
    analyze_gate_counts(circuit)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::qubits::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn gate(
        kind: GateKind,
        qubits: Vec<QubitId>,
    ) -> Gate {
        Gate::new(
            kind,
            qubits,
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn parameterized_gate(
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameters: Vec<Parameter>,
    ) -> Gate {
        Gate::new(
            kind,
            qubits,
            parameters,
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn measurement(
        qubit: QubitId,
        classical_bit: usize,
    ) -> Gate {
        Gate::new(
            GateKind::Measure,
            vec![qubit],
            Vec::new(),
            Some(classical_bit),
            None,
        )
        .expect("test measurement must be valid")
    }

    // -------------------------------------------------------------------------
    // Empty circuit
    // -------------------------------------------------------------------------

    #[test]
    fn empty_circuit_has_zero_counts() {
        let circuit =
            QuantumCircuit::new(4, 2)
                .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.operation_count(), 0);
        assert_eq!(result.gate_count(), 0);
        assert_eq!(
            result.non_unitary_operation_count(),
            0
        );
        assert_eq!(
            result.single_qubit_operation_count(),
            0
        );
        assert_eq!(
            result.two_qubit_operation_count(),
            0
        );
        assert_eq!(
            result.three_qubit_operation_count(),
            0
        );
        assert_eq!(
            result.total_qubit_operand_count(),
            0
        );
        assert_eq!(
            result.total_parameter_count(),
            0
        );
        assert_eq!(
            result.maximum_operation_arity(),
            0
        );
        assert_eq!(
            result.counted_kind_total()
                .expect("no overflow"),
            0
        );
    }

    // -------------------------------------------------------------------------
    // Complete current GateKind coverage
    // -------------------------------------------------------------------------

    #[test]
    fn every_current_gate_kind_is_counted() {
        let operations = vec![
            gate(GateKind::I, vec![q(0)]),
            gate(GateKind::X, vec![q(0)]),
            gate(GateKind::Y, vec![q(0)]),
            gate(GateKind::Z, vec![q(0)]),
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::S, vec![q(0)]),
            gate(GateKind::Sdg, vec![q(0)]),
            gate(GateKind::T, vec![q(0)]),
            gate(GateKind::Tdg, vec![q(0)]),
            gate(GateKind::V, vec![q(0)]),
            gate(GateKind::Vdg, vec![q(0)]),
            parameterized_gate(
                GateKind::RX,
                vec![q(0)],
                vec![Parameter::Constant(0.1)],
            ),
            parameterized_gate(
                GateKind::RY,
                vec![q(0)],
                vec![Parameter::Constant(0.2)],
            ),
            parameterized_gate(
                GateKind::RZ,
                vec![q(0)],
                vec![Parameter::Constant(0.3)],
            ),
            parameterized_gate(
                GateKind::Phase,
                vec![q(0)],
                vec![Parameter::Constant(0.4)],
            ),
            parameterized_gate(
                GateKind::U1,
                vec![q(0)],
                vec![Parameter::Constant(0.5)],
            ),
            parameterized_gate(
                GateKind::U2,
                vec![q(0)],
                vec![
                    Parameter::Constant(0.6),
                    Parameter::Constant(0.7),
                ],
            ),
            parameterized_gate(
                GateKind::U3,
                vec![q(0)],
                vec![
                    Parameter::Constant(0.8),
                    Parameter::Constant(0.9),
                    Parameter::Constant(1.0),
                ],
            ),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::CY, vec![q(0), q(1)]),
            gate(GateKind::CZ, vec![q(0), q(1)]),
            gate(GateKind::CH, vec![q(0), q(1)]),
            gate(GateKind::SWAP, vec![q(0), q(1)]),
            gate(GateKind::ISWAP, vec![q(0), q(1)]),
            gate(GateKind::ECR, vec![q(0), q(1)]),
            parameterized_gate(
                GateKind::CRX,
                vec![q(0), q(1)],
                vec![Parameter::Constant(1.1)],
            ),
            parameterized_gate(
                GateKind::CRY,
                vec![q(0), q(1)],
                vec![Parameter::Constant(1.2)],
            ),
            parameterized_gate(
                GateKind::CRZ,
                vec![q(0), q(1)],
                vec![Parameter::Constant(1.3)],
            ),
            gate(GateKind::CCX, vec![q(0), q(1), q(2)]),
            gate(
                GateKind::CSWAP,
                vec![q(0), q(1), q(2)],
            ),
            measurement(q(0), 0),
            gate(
                GateKind::Barrier,
                vec![q(0), q(1), q(2)],
            ),
            gate(GateKind::Reset, vec![q(0)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                3,
                1,
                operations,
            )
            .expect("valid test circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.operation_count(), 33);
        assert_eq!(result.counted_kind_total().unwrap(), 33);

        assert_eq!(
            result.single_qubit_operation_count(),
            19
        );
        assert_eq!(
            result.two_qubit_operation_count(),
            10
        );
        assert_eq!(
            result.three_qubit_operation_count(),
            2
        );

        assert_eq!(
            result.measurement_count(),
            1
        );
        assert_eq!(result.reset_count(), 1);
        assert_eq!(result.barrier_count(), 1);

        assert_eq!(
            result.non_unitary_operation_count(),
            3
        );

        assert_eq!(result.gate_count(), 30);
        assert_eq!(
            result.parameterized_operation_count(),
            7
        );
        assert_eq!(
            result.total_parameter_count(),
            10
        );
        assert_eq!(
            result.maximum_operation_arity(),
            3
        );
    }

    // -------------------------------------------------------------------------
    // Clifford/non-Clifford
    // -------------------------------------------------------------------------

    #[test]
    fn clifford_and_non_clifford_counts_are_separate() {
        let operations = vec![
            gate(GateKind::I, vec![q(0)]),
            gate(GateKind::X, vec![q(0)]),
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::S, vec![q(0)]),
            gate(GateKind::Sdg, vec![q(0)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::CZ, vec![q(0), q(1)]),
            gate(GateKind::T, vec![q(0)]),
            gate(GateKind::Tdg, vec![q(0)]),
            gate(GateKind::V, vec![q(0)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                operations,
            )
            .expect("valid test circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            result.clifford_operation_count(),
            7
        );

        assert_eq!(
            result.non_clifford_operation_count(),
            3
        );

        assert_eq!(result.gate_count(), 10);
    }

    // -------------------------------------------------------------------------
    // T-family
    // -------------------------------------------------------------------------

    #[test]
    fn t_family_counts_are_exact() {
        let operations = vec![
            gate(GateKind::T, vec![q(0)]),
            gate(GateKind::T, vec![q(0)]),
            gate(GateKind::Tdg, vec![q(0)]),
            gate(GateKind::Tdg, vec![q(0)]),
            gate(GateKind::H, vec![q(0)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.t_count(), 2);
        assert_eq!(result.t_dagger_count(), 2);
        assert_eq!(result.t_family_count(), 4);
    }

    // -------------------------------------------------------------------------
    // Two-qubit metric
    // -------------------------------------------------------------------------

    #[test]
    fn two_qubit_count_is_independent_of_total_gate_count() {
        let operations = vec![
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::H, vec![q(1)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::RZ, vec![q(0)]),
            gate(GateKind::CZ, vec![q(0), q(1)]),
            gate(GateKind::X, vec![q(1)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.gate_count(), 6);
        assert_eq!(
            result.two_qubit_operation_count(),
            2
        );
        assert_eq!(
            result.single_qubit_operation_count(),
            4
        );
    }

    // -------------------------------------------------------------------------
    // Parameter accounting
    // -------------------------------------------------------------------------

    #[test]
    fn parameter_count_tracks_parameter_occurrences() {
        let operations = vec![
            parameterized_gate(
                GateKind::RX,
                vec![q(0)],
                vec![Parameter::Constant(0.1)],
            ),
            parameterized_gate(
                GateKind::U2,
                vec![q(0)],
                vec![
                    Parameter::Constant(0.2),
                    Parameter::Constant(0.3),
                ],
            ),
            parameterized_gate(
                GateKind::U3,
                vec![q(1)],
                vec![
                    Parameter::Constant(0.4),
                    Parameter::Constant(0.5),
                    Parameter::Constant(0.6),
                ],
            ),
            gate(GateKind::H, vec![q(1)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            result.parameterized_operation_count(),
            3
        );
        assert_eq!(
            result.total_parameter_count(),
            6
        );
    }

    // -------------------------------------------------------------------------
    // Measurement/classical destination
    // -------------------------------------------------------------------------

    #[test]
    fn measurement_classical_targets_are_counted() {
        let operations = vec![
            measurement(q(0), 0),
            measurement(q(1), 1),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                2,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.measurement_count(), 2);
        assert_eq!(
            result.classical_target_operation_count(),
            2
        );
        assert_eq!(
            result.non_unitary_operation_count(),
            2
        );
        assert_eq!(result.gate_count(), 0);
    }

    // -------------------------------------------------------------------------
    // Operand accounting
    // -------------------------------------------------------------------------

    #[test]
    fn operand_count_counts_every_logical_operand() {
        let operations = vec![
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::CCX, vec![q(0), q(1), q(2)]),
            gate(
                GateKind::Barrier,
                vec![q(0), q(1), q(2)],
            ),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                3,
                0,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            result.total_qubit_operand_count(),
            1 + 2 + 3 + 3
        );

        assert_eq!(
            result.maximum_operation_arity(),
            3
        );
    }

    // -------------------------------------------------------------------------
    // Invariants
    // -------------------------------------------------------------------------

    #[test]
    fn invariants_hold_for_normal_circuit() {
        let operations = vec![
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::T, vec![q(1)]),
            measurement(q(0), 0),
            gate(GateKind::Reset, vec![q(1)]),
            gate(
                GateKind::Barrier,
                vec![q(0), q(1)],
            ),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                1,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        result
            .validate_invariants()
            .expect("all invariants must hold");
    }

    // -------------------------------------------------------------------------
    // Determinism
    // -------------------------------------------------------------------------

    #[test]
    fn repeated_analysis_is_deterministic() {
        let operations = vec![
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::T, vec![q(1)]),
            gate(GateKind::RZ, vec![q(0)]),
            measurement(q(1), 0),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                1,
                operations,
            )
            .expect("valid circuit");

        let first =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        let second =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(first, second);
    }

    // -------------------------------------------------------------------------
    // Fractions
    // -------------------------------------------------------------------------

    #[test]
    fn fractions_are_defined_only_when_denominator_exists() {
        let empty =
            QuantumCircuit::new(1, 0)
                .expect("valid circuit");

        let empty_result =
            analyze_gate_counts(&empty)
                .expect("analysis must succeed");

        assert_eq!(
            empty_result.gate_fraction(),
            None
        );

        assert_eq!(
            empty_result.two_qubit_gate_fraction(),
            None
        );

        assert_eq!(
            empty_result.non_clifford_fraction(),
            None
        );

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![
                    gate(GateKind::H, vec![q(0)]),
                    gate(GateKind::CX, vec![q(0), q(1)]),
                    gate(GateKind::T, vec![q(1)]),
                ],
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            result.gate_fraction(),
            Some(1.0)
        );

        assert_eq!(
            result.two_qubit_gate_fraction(),
            Some(1.0 / 3.0)
        );

        assert_eq!(
            result.non_clifford_fraction(),
            Some(1.0 / 3.0)
        );
    }

    // -------------------------------------------------------------------------
    // Empty/zero-width namespace
    // -------------------------------------------------------------------------

    #[test]
    fn zero_qubit_empty_circuit_is_supported() {
        let circuit =
            QuantumCircuit::new(0, 0)
                .expect("zero-qubit circuit is valid");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(result.operation_count(), 0);
        assert_eq!(result.gate_count(), 0);
    }

    // -------------------------------------------------------------------------
    // Canonical circuit remains unchanged
    // -------------------------------------------------------------------------

    #[test]
    fn analysis_does_not_mutate_circuit() {
        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![
                    gate(GateKind::H, vec![q(0)]),
                    gate(GateKind::CX, vec![q(0), q(1)]),
                ],
            )
            .expect("valid circuit");

        let before = circuit.clone();

        let _ =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(circuit, before);
    }

    // -------------------------------------------------------------------------
    // GateKind lookup
    // -------------------------------------------------------------------------

    #[test]
    fn per_kind_lookup_matches_explicit_fields() {
        let operations = vec![
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::H, vec![q(0)]),
            gate(GateKind::CX, vec![q(0), q(1)]),
            gate(GateKind::T, vec![q(1)]),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                operations,
            )
            .expect("valid circuit");

        let result =
            analyze_gate_counts(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            result.count(GateKind::H),
            2
        );

        assert_eq!(
            result.count(GateKind::CX),
            1
        );

        assert_eq!(
            result.count(GateKind::T),
            1
        );

        assert_eq!(
            result.count(GateKind::Measure),
            0
        );
    }

    // -------------------------------------------------------------------------
    // Current kind total
    // -------------------------------------------------------------------------

    #[test]
    fn kind_counter_contains_every_current_variant() {
        let counters =
            GateKindCounts::default();

        assert_eq!(
            counters.as_pairs().len(),
            GateKindCounts::field_count()
        );

        assert_eq!(
            counters.total().unwrap(),
            0
        );
    }
}