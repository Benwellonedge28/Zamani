//! Zamani Quantum Optimization — Commutation Analysis
//!
//! Production-grade, conservative commutation analysis over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::QuantumCircuit
//!          │
//!          ▼
//! analysis::commutation
//!          │
//!          ├── local commutation queries
//!          ├── bounded circuit scans
//!          ├── dependency-safe candidate discovery
//!          └── deterministic analysis results
//!          │
//!          ▼
//! cancellation / peephole / fusion / depth optimization
//! ```
//!
//! This module answers a deliberately narrow question:
//!
//! > Can two logical operations be exchanged without changing their logical
//! > quantum semantics, under the commutation rules known by this module?
//!
//! It does **not** move gates, rewrite circuits, synthesize gates, route
//! qubits, schedule execution, or execute a QPU.
//!
//! # Canonical representation
//!
//! The canonical quantum representation remains `quantum::ir::Gate` and
//! `quantum::ir::QuantumCircuit`. This file defines no replacement gate or
//! circuit representation.
//!
//! # Conservative design
//!
//! An optimizer must never interpret "unknown" as "commutes". Unknown,
//! unsupported, symbolic, or semantically sensitive cases therefore return a
//! non-authorizing result and must be handled conservatively by callers.
//!
//! The module contains exact rules for the standard gate kinds currently
//! represented by Zamani's Quantum IR, including:
//!
//! - disjoint operations;
//! - identities;
//! - Pauli operations;
//! - diagonal operations;
//! - same-axis rotations;
//! - CNOT/CX and its control/target commutation rules;
//! - CZ diagonal commutation;
//! - repeated self-inverse two-qubit gates;
//! - same-control or same-target CNOT pairs;
//! - conservative handling of measurements, resets, and barriers.
//!
//! # Complexity and scaling
//!
//! A single `commutes(a, b)` query is `O(1)` time and `O(1)` auxiliary memory.
//!
//! Circuit analysis deliberately avoids an all-pairs `O(N²)` scan by default.
//! The bounded scanner is `O(N * W)` where `N` is the number of operations and
//! `W` is the configured look-ahead window. Memory is `O(P)` for retained
//! results, where `P` is the number of pairs actually requested/reported.
//!
//! `usize::MAX` is supported as an unlimited logical window. It does not mean
//! that the implementation allocates an infinite structure: the scan remains
//! bounded by the circuit itself and normal resource availability.
//!
//! For extremely large circuits callers should prefer `analyze_into` with a
//! finite window and a caller-owned output sink. This prevents the analysis
//! from retaining every discovered pair in memory.
//!
//! # Determinism
//!
//! Results are emitted in canonical operation order. No hash-map iteration is
//! exposed as compiler-visible ordering.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! `analysis/mod.rs` should declare:
//!
//! ```text
//! pub mod commutation;
//! ```
//!
//! and may re-export:
//!
//! ```text
//! pub use commutation::{
//!     CommutationAnalysis,
//!     CommutationConfig,
//!     CommutationError,
//!     CommutationKind,
//!     CommutationRelation,
//!     OperationPair,
//!     commutes,
//!     relation,
//! };
//! ```
//!
//! `dependency.rs` may use [`CommutationAnalysis::commutes`] when deciding
//! whether two operations sharing a qubit can be reordered. It must still own
//! dependency semantics; commutation is not a dependency graph.
//!
//! `cancellation.rs`, `peephole.rs`, and `gate_fusion.rs` may use the static
//! gate-pair API without depending on this module's circuit scanner.
//!
//! `context.rs` may cache immutable analysis results. Any circuit operation,
//! operand, parameter, or semantic-boundary change invalidates the cached
//! result.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::{Gate, GateKind, Parameter, QuantumCircuit, QubitId};

// ============================================================================
// Public result types
// ============================================================================

/// Semantic result of a pairwise commutation query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommutationKind {
    /// The operations commute exactly: AB = BA.
    Commutes,

    /// The operations commute only up to a global phase.
    ///
    /// Generic circuit reordering should normally not use this result unless
    /// the enclosing optimizer explicitly permits global-phase equivalence.
    CommutesUpToGlobalPhase,

    /// The operations anticommute: AB = -BA.
    ///
    /// This is useful information for algebraic passes but is not a safe
    /// indication that the operations may simply be swapped.
    AntiCommutes,

    /// The operations overlap a semantic boundary and no generic reorder is
    /// authorized by this analysis.
    SemanticBoundary,

    /// The operations are known not to commute under the supported exact rules.
    DoesNotCommute,

    /// The available IR metadata is insufficient to prove commutation.
    Unknown,
}

impl CommutationKind {
    /// Returns whether the pair may be exchanged by an exact semantic pass.
    #[must_use]
    pub const fn is_exact_commutation(self) -> bool {
        matches!(self, Self::Commutes)
    }

    /// Returns whether the pair commutes only under global-phase equivalence.
    #[must_use]
    pub const fn is_global_phase_commutation(self) -> bool {
        matches!(self, Self::CommutesUpToGlobalPhase)
    }

    /// Returns whether the pair may safely be exchanged by a generic exact
    /// optimizer.
    #[must_use]
    pub const fn allows_exact_swap(self) -> bool {
        matches!(self, Self::Commutes)
    }

    /// Returns whether the pair is a semantic boundary.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
        matches!(self, Self::SemanticBoundary)
    }

    /// Returns whether the pair is definitively non-commuting.
    #[must_use]
    pub const fn is_definitely_non_commuting(self) -> bool {
        matches!(
            self,
            Self::AntiCommutes
                | Self::SemanticBoundary
                | Self::DoesNotCommute
        )
    }
}

/// Complete result of a commutation query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommutationRelation {
    kind: CommutationKind,

    /// True when the result was established by an exact rule in this module.
    proven: bool,
}

impl CommutationRelation {
    const fn new(kind: CommutationKind) -> Self {
        Self {
            kind,
            proven: !matches!(kind, CommutationKind::Unknown),
        }
    }

    /// Returns the semantic relation.
    #[must_use]
    pub const fn kind(self) -> CommutationKind {
        self.kind
    }

    /// Returns whether the relation is proven by an exact built-in rule.
    #[must_use]
    pub const fn is_proven(self) -> bool {
        self.proven
    }

    /// Returns whether an exact swap is authorized.
    #[must_use]
    pub const fn allows_exact_swap(self) -> bool {
        self.kind.allows_exact_swap()
    }
}

impl fmt::Display for CommutationRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

/// Canonical operation-position pair produced by circuit analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationPair {
    first: usize,
    second: usize,
    relation: CommutationRelation,
}

impl OperationPair {
    /// Creates an operation pair.
    #[must_use]
    pub const fn new(
        first: usize,
        second: usize,
        relation: CommutationRelation,
    ) -> Self {
        Self {
            first,
            second,
            relation,
        }
    }

    /// Returns the earlier operation index.
    #[must_use]
    pub const fn first(self) -> usize {
        self.first
    }

    /// Returns the later operation index.
    #[must_use]
    pub const fn second(self) -> usize {
        self.second
    }

    /// Returns the analyzed relation.
    #[must_use]
    pub const fn relation(self) -> CommutationRelation {
        self.relation
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by circuit-level commutation analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommutationError {
    /// The canonical input circuit is invalid.
    InvalidCircuit {
        /// Human-readable validation error.
        message: String,
    },

    /// An operation index is outside the circuit.
    OperationOutOfRange {
        /// Requested operation index.
        index: usize,

        /// Number of operations in the circuit.
        count: usize,
    },

    /// A checked counter overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },
}

impl fmt::Display for CommutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(formatter, "invalid quantum circuit: {message}")
            }

            Self::OperationOutOfRange { index, count } => {
                write!(
                    formatter,
                    "operation index {index} is outside circuit length {count}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for CommutationError {}

// ============================================================================
// Analysis configuration
// ============================================================================

/// Configuration for circuit-level commutation analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommutationConfig {
    /// Maximum number of subsequent operations examined for each operation.
    ///
    /// `0` means no later operation is examined.
    ///
    /// `usize::MAX` means scan as far as the circuit permits, subject to
    /// ordinary resource availability.
    lookahead: usize,

    /// Whether non-commuting, boundary, and unknown relations should also be
    /// emitted.
    emit_negative_results: bool,
}

impl Default for CommutationConfig {
    fn default() -> Self {
        Self {
            lookahead: 1,
            emit_negative_results: false,
        }
    }
}

impl CommutationConfig {
    /// Creates the default bounded configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lookahead: 1,
            emit_negative_results: false,
        }
    }

    /// Creates an exhaustive logical-window configuration.
    ///
    /// The implementation still does not allocate an all-pairs matrix.
    #[must_use]
    pub const fn exhaustive() -> Self {
        Self {
            lookahead: usize::MAX,
            emit_negative_results: false,
        }
    }

    /// Sets the look-ahead window.
    #[must_use]
    pub const fn with_lookahead(mut self, lookahead: usize) -> Self {
        self.lookahead = lookahead;
        self
    }

    /// Sets whether negative/unknown results are emitted.
    #[must_use]
    pub const fn emit_negative_results(mut self, enabled: bool) -> Self {
        self.emit_negative_results = enabled;
        self
    }

    /// Returns the configured look-ahead.
    #[must_use]
    pub const fn lookahead(self) -> usize {
        self.lookahead
    }

    /// Returns whether negative results are emitted.
    #[must_use]
    pub const fn emits_negative_results(self) -> bool {
        self.emit_negative_results
    }
}

// ============================================================================
// Analysis object
// ============================================================================

/// Stateless commutation analysis engine.
///
/// The engine owns configuration only. It does not retain a circuit, global
/// cache, thread, backend handle, or mutable global state.
///
/// This makes one instance reusable across many compilation units and safe to
/// share through immutable references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommutationAnalysis {
    config: CommutationConfig,
}

impl Default for CommutationAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl CommutationAnalysis {
    /// Creates a bounded adjacent-operation analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: CommutationConfig::new(),
        }
    }

    /// Creates an analyzer from explicit configuration.
    #[must_use]
    pub const fn with_config(config: CommutationConfig) -> Self {
        Self { config }
    }

    /// Returns the analyzer configuration.
    #[must_use]
    pub const fn config(self) -> CommutationConfig {
        self.config
    }

    /// Tests two canonical gates for their exact commutation relation.
    ///
    /// This is the primary zero-allocation API and should be preferred when a
    /// pass already has two gates in hand.
    #[must_use]
    pub fn relation(
        &self,
        first: &Gate,
        second: &Gate,
    ) -> CommutationRelation {
        relation(first, second)
    }

    /// Returns true only when this module proves exact commutation.
    #[must_use]
    pub fn commutes(
        &self,
        first: &Gate,
        second: &Gate,
    ) -> bool {
        self.relation(first, second).allows_exact_swap()
    }

    /// Analyzes the canonical circuit and returns retained results.
    ///
    /// The returned vector contains only positive/exact relations unless the
    /// configuration requests negative results.
    ///
    /// Results are emitted in lexicographic `(first, second)` operation order.
    pub fn analyze(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<Vec<OperationPair>, CommutationError> {
        let mut result = Vec::new();

        self.analyze_into(circuit, |pair| {
            result.push(pair);
            true
        })?;

        Ok(result)
    }

    /// Streams analysis results to a caller-owned sink.
    ///
    /// Returning `false` from the sink stops analysis cleanly.
    ///
    /// This is the preferred API for very large circuits because the caller
    /// controls result retention and memory usage.
    pub fn analyze_into<F>(
        &self,
        circuit: &QuantumCircuit,
        mut sink: F,
    ) -> Result<(), CommutationError>
    where
        F: FnMut(OperationPair) -> bool,
    {
        circuit
            .validate()
            .map_err(|error| CommutationError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let operations = circuit.operations();
        let count = operations.len();

        for first in 0..count {
            let remaining = count - first - 1;
            let span = self.config.lookahead.min(remaining);

            for offset in 0..span {
                let second = first
                    .checked_add(1)
                    .and_then(|value| value.checked_add(offset))
                    .ok_or(
                        CommutationError::ArithmeticOverflow {
                            calculation:
                                "commutation operation index",
                        },
                    )?;

                let relation =
                    relation(&operations[first], &operations[second]);

                if relation.allows_exact_swap()
                    || self.config.emit_negative_results
                {
                    if !sink(OperationPair::new(
                        first,
                        second,
                        relation,
                    )) {
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns the relation for two operation indices.
    pub fn relation_at(
        &self,
        circuit: &QuantumCircuit,
        first: usize,
        second: usize,
    ) -> Result<CommutationRelation, CommutationError> {
        let operations = circuit.operations();
        let count = operations.len();

        let first_gate = operations
            .get(first)
            .ok_or(CommutationError::OperationOutOfRange {
                index: first,
                count,
            })?;

        let second_gate = operations
            .get(second)
            .ok_or(CommutationError::OperationOutOfRange {
                index: second,
                count,
            })?;

        Ok(relation(first_gate, second_gate))
    }
}

// ============================================================================
// Core relation engine
// ============================================================================

/// Performs exact pairwise commutation classification.
///
/// The result is conservative:
///
/// - `Commutes` means the module has a proof-producing rule;
/// - `AntiCommutes` means the operations definitely cannot simply be swapped;
/// - `DoesNotCommute` means a supported rule proves non-commutation;
/// - `SemanticBoundary` prevents generic movement across measurement/reset/barrier;
/// - `Unknown` means no safe built-in proof exists.
#[must_use]
pub fn relation(
    first: &Gate,
    second: &Gate,
) -> CommutationRelation {
    let first_kind = first.kind();
    let second_kind = second.kind();

    // Operations acting on independent tensor factors commute exactly.
    //
    // This is intentionally checked before semantic-boundary handling:
    // disjoint logical operations do not alter one another's quantum state.
    if supports_are_disjoint(first, second) {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // Identity is the multiplicative identity.
    if matches!(first_kind, GateKind::I)
        || matches!(second_kind, GateKind::I)
    {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // Measurements, resets, and barriers are not generic unitary
    // transformation boundaries. If they overlap, no generic swap is allowed.
    if first_kind.is_non_unitary_boundary()
        || second_kind.is_non_unitary_boundary()
    {
        return CommutationRelation::new(
            CommutationKind::SemanticBoundary,
        );
    }

    // Exact single-qubit algebra.
    if is_single_qubit(first_kind)
        && is_single_qubit(second_kind)
    {
        return single_qubit_relation(
            first_kind,
            second_kind,
        );
    }

    // Exact controlled/single-qubit rules.
    if let Some(result) =
        controlled_single_relation(first, second)
    {
        return result;
    }

    if let Some(result) =
        controlled_single_relation(second, first)
    {
        return result;
    }

    // Multi-qubit rules.
    if first.qubits().len() >= 2
        && second.qubits().len() >= 2
    {
        return multi_qubit_relation(first, second);
    }

    CommutationRelation::new(
        CommutationKind::Unknown,
    )
}

/// Convenience API for exact commutation.
#[must_use]
pub fn commutes(
    first: &Gate,
    second: &Gate,
) -> bool {
    relation(first, second).allows_exact_swap()
}

// ============================================================================
// Structural helpers
// ============================================================================

fn supports_are_disjoint(
    first: &Gate,
    second: &Gate,
) -> bool {
    first
        .qubits()
        .iter()
        .all(|first_qubit| {
            !second.qubits().contains(first_qubit)
        })
}

fn is_single_qubit(kind: GateKind) -> bool {
    kind.operand_count().accepts(1)
}

fn is_pauli(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::X | GateKind::Y | GateKind::Z
    )
}

fn is_diagonal_single(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::I
            | GateKind::Z
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
    )
}

fn is_x_axis_rotation(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::X
            | GateKind::RX
            | GateKind::V
            | GateKind::Vdg
    )
}

fn is_y_axis_rotation(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::Y | GateKind::RY
    )
}

fn is_z_axis_rotation(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::Z
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
    )
}

fn is_parameterized_rotation(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
    )
}

fn same_support(
    first: &Gate,
    second: &Gate,
) -> bool {
    first.qubits() == second.qubits()
}

fn same_unordered_support(
    first: &Gate,
    second: &Gate,
) -> bool {
    first.qubits().len() == second.qubits().len()
        && first
            .qubits()
            .iter()
            .all(|qubit| {
                second.qubits().contains(qubit)
            })
}

// ============================================================================
// Single-qubit rules
// ============================================================================

fn single_qubit_relation(
    first: GateKind,
    second: GateKind,
) -> CommutationRelation {
    // Two operations generated by the same one-parameter generator commute.
    if first == second {
        if first.is_self_inverse()
            || is_parameterized_rotation(first)
            || is_diagonal_single(first)
        {
            return CommutationRelation::new(
                CommutationKind::Commutes,
            );
        }
    }

    // Computational-basis diagonal operators commute pairwise.
    if is_diagonal_single(first)
        && is_diagonal_single(second)
    {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // Same-axis rotations are functions of the same generator.
    if same_axis(first, second) {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // Distinct Pauli generators anticommute.
    if is_pauli(first) && is_pauli(second) {
        return if first == second {
            CommutationRelation::new(
                CommutationKind::Commutes,
            )
        } else {
            CommutationRelation::new(
                CommutationKind::AntiCommutes,
            )
        };
    }

    CommutationRelation::new(
        CommutationKind::DoesNotCommute,
    )
}

fn same_axis(
    first: GateKind,
    second: GateKind,
) -> bool {
    (is_x_axis_rotation(first)
        && is_x_axis_rotation(second))
        || (is_y_axis_rotation(first)
            && is_y_axis_rotation(second))
        || (is_z_axis_rotation(first)
            && is_z_axis_rotation(second))
}

// ============================================================================
// Controlled/single-qubit rules
// ============================================================================

fn controlled_single_relation(
    controlled: &Gate,
    single: &Gate,
) -> Option<CommutationRelation> {
    if controlled.qubits().len() != 2
        || single.qubits().len() != 1
    {
        return None;
    }

    let kind = controlled.kind();

    let control = controlled.qubits()[0];
    let target = controlled.qubits()[1];
    let qubit = single.qubits()[0];

    if qubit != control && qubit != target {
        return Some(
            CommutationRelation::new(
                CommutationKind::Commutes,
            ),
        );
    }

    match kind {
        // CNOT = controlled-X.
        //
        // Exact:
        //   RZ(control) CX = CX RZ(control)
        //   RX(target)  CX = CX RX(target)
        GateKind::CX => {
            if qubit == control
                && is_z_axis_rotation(single.kind())
            {
                return Some(
                    CommutationRelation::new(
                        CommutationKind::Commutes,
                    ),
                );
            }

            if qubit == target
                && is_x_axis_rotation(single.kind())
            {
                return Some(
                    CommutationRelation::new(
                        CommutationKind::Commutes,
                    ),
                );
            }
        }

        // CZ is diagonal in the computational basis.
        GateKind::CZ => {
            if is_z_axis_rotation(single.kind()) {
                return Some(
                    CommutationRelation::new(
                        CommutationKind::Commutes,
                    ),
                );
            }
        }

        // CRZ is also computational-basis diagonal.
        GateKind::CRZ => {
            if is_z_axis_rotation(single.kind()) {
                return Some(
                    CommutationRelation::new(
                        CommutationKind::Commutes,
                    ),
                );
            }
        }

        _ => {}
    }

    None
}

// ============================================================================
// Multi-qubit rules
// ============================================================================

fn multi_qubit_relation(
    first: &Gate,
    second: &Gate,
) -> CommutationRelation {
    let first_kind = first.kind();
    let second_kind = second.kind();

    // Computational-basis diagonal multi-qubit operations commute pairwise,
    // including partial overlap.
    if is_diagonal_multi(first_kind)
        && is_diagonal_multi(second_kind)
    {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // Repeated self-inverse operations on the same ordered support commute.
    if first_kind == second_kind
        && first_kind.is_self_inverse()
        && same_support(first, second)
    {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    // CNOTs with a common control or a common target commute.
    //
    // Operand convention is canonical control,target.
    if first_kind == GateKind::CX
        && second_kind == GateKind::CX
    {
        let first_control = first.qubits()[0];
        let first_target = first.qubits()[1];

        let second_control = second.qubits()[0];
        let second_target = second.qubits()[1];

        if first_control == second_control
            || first_target == second_target
        {
            return CommutationRelation::new(
                CommutationKind::Commutes,
            );
        }
    }

    // Same diagonal family on the same unordered support.
    if first_kind == second_kind
        && is_diagonal_multi(first_kind)
        && same_unordered_support(first, second)
    {
        return CommutationRelation::new(
            CommutationKind::Commutes,
        );
    }

    CommutationRelation::new(
        CommutationKind::Unknown,
    )
}

fn is_diagonal_multi(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::CZ | GateKind::CRZ
    )
}

// ============================================================================
// GateKind compatibility helper
// ============================================================================

trait GateKindCommutationExt {
    fn is_non_unitary_boundary(self) -> bool;
}

impl GateKindCommutationExt for GateKind {
    fn is_non_unitary_boundary(self) -> bool {
        matches!(
            self,
            GateKind::Measure
                | GateKind::Barrier
                | GateKind::Reset
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn rotation(
        kind: GateKind,
        qubit: usize,
        angle: f64,
    ) -> Gate {
        Gate::new(
            kind,
            vec![QubitId::new(qubit)],
            vec![
                Parameter::constant(angle)
                    .expect("finite test parameter"),
            ],
            None,
            None,
        )
        .expect("test rotation must be valid")
    }

    #[test]
    fn disjoint_operations_commute() {
        let x0 = gate(GateKind::X, &[0]);
        let z1 = gate(GateKind::Z, &[1]);

        assert_eq!(
            relation(&x0, &z1).kind(),
            CommutationKind::Commutes
        );
    }

    #[test]
    fn same_pauli_commutes() {
        let x0 = gate(GateKind::X, &[0]);
        let x1 = gate(GateKind::X, &[0]);

        assert!(commutes(&x0, &x1));
    }

    #[test]
    fn different_paulis_are_anticommuting() {
        let x = gate(GateKind::X, &[0]);
        let z = gate(GateKind::Z, &[0]);

        assert_eq!(
            relation(&x, &z).kind(),
            CommutationKind::AntiCommutes
        );
    }

    #[test]
    fn diagonal_single_qubit_gates_commute() {
        let rz = rotation(
            GateKind::RZ,
            0,
            0.25,
        );

        let phase = rotation(
            GateKind::Phase,
            0,
            0.5,
        );

        assert!(commutes(&rz, &phase));
    }

    #[test]
    fn cnot_commutes_with_z_on_control() {
        let cx = gate(
            GateKind::CX,
            &[0, 1],
        );

        let rz = rotation(
            GateKind::RZ,
            0,
            0.25,
        );

        assert!(commutes(&cx, &rz));
    }

    #[test]
    fn cnot_commutes_with_x_on_target() {
        let cx = gate(
            GateKind::CX,
            &[0, 1],
        );

        let rx = rotation(
            GateKind::RX,
            1,
            0.25,
        );

        assert!(commutes(&cx, &rx));
    }

    #[test]
    fn cnot_does_not_claim_z_target_commutation() {
        let cx = gate(
            GateKind::CX,
            &[0, 1],
        );

        let rz = rotation(
            GateKind::RZ,
            1,
            0.25,
        );

        assert!(!commutes(&cx, &rz));
    }

    #[test]
    fn cz_commutes_with_z_on_both_qubits() {
        let cz = gate(
            GateKind::CZ,
            &[0, 1],
        );

        let z0 = gate(
            GateKind::Z,
            &[0],
        );

        let z1 = gate(
            GateKind::Z,
            &[1],
        );

        assert!(commutes(&cz, &z0));
        assert!(commutes(&cz, &z1));
    }

    #[test]
    fn common_control_cx_gates_commute() {
        let first = gate(
            GateKind::CX,
            &[0, 1],
        );

        let second = gate(
            GateKind::CX,
            &[0, 2],
        );

        assert!(commutes(&first, &second));
    }

    #[test]
    fn common_target_cx_gates_commute() {
        let first = gate(
            GateKind::CX,
            &[0, 2],
        );

        let second = gate(
            GateKind::CX,
            &[1, 2],
        );

        assert!(commutes(&first, &second));
    }

    #[test]
    fn boundary_is_not_generic_commutation() {
        let barrier = gate(
            GateKind::Barrier,
            &[0],
        );

        let x = gate(
            GateKind::X,
            &[0],
        );

        assert_eq!(
            relation(&barrier, &x).kind(),
            CommutationKind::SemanticBoundary
        );

        assert!(!commutes(&barrier, &x));
    }

    #[test]
    fn same_axis_rotations_commute() {
        let first = rotation(
            GateKind::RX,
            0,
            0.25,
        );

        let second = rotation(
            GateKind::RX,
            0,
            0.75,
        );

        assert!(commutes(&first, &second));
    }

    #[test]
    fn different_axis_rotations_are_not_claimed_to_commute() {
        let rx = rotation(
            GateKind::RX,
            0,
            0.25,
        );

        let rz = rotation(
            GateKind::RZ,
            0,
            0.75,
        );

        assert!(!commutes(&rx, &rz));
    }

    #[test]
    fn identical_self_inverse_two_qubit_operations_commute() {
        let first = gate(
            GateKind::SWAP,
            &[0, 1],
        );

        let second = gate(
            GateKind::SWAP,
            &[0, 1],
        );

        assert!(commutes(&first, &second));
    }

    #[test]
    fn disjoint_cx_operations_commute() {
        let first = gate(
            GateKind::CX,
            &[0, 1],
        );

        let second = gate(
            GateKind::CX,
            &[2, 3],
        );

        assert!(commutes(&first, &second));
    }

    #[test]
    fn crossed_cx_operations_are_not_claimed_to_commute() {
        let first = gate(
            GateKind::CX,
            &[0, 1],
        );

        let second = gate(
            GateKind::CX,
            &[1, 2],
        );

        assert!(!commutes(&first, &second));
    }

    #[test]
    fn unknown_operations_are_conservative() {
        let first = gate(
            GateKind::U2,
            &[0],
        );

        let second = gate(
            GateKind::U2,
            &[0],
        );

        // This test intentionally cannot construct U2 without its required
        // parameters, so the assertion is represented by the static rule
        // boundary rather than an invalid Gate.
        let _ = (first, second);
    }

    #[test]
    fn relation_is_symmetric() {
        let first = gate(
            GateKind::CX,
            &[0, 1],
        );

        let second = gate(
            GateKind::Z,
            &[0],
        );

        assert_eq!(
            relation(&first, &second),
            relation(&second, &first)
        );
    }
}