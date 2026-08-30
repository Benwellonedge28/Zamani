//! Zamani Quantum Optimization — Fault-Tolerant T-Gate Reduction
//!
//! Production-grade exact local reduction of contiguous Clifford+T phase
//! powers for the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::fault_tolerant
//!                              │
//!                ┌─────────────┼─────────────┐
//!                │             │             │
//!                ▼             ▼             ▼
//!           T reduction   T-count       T-depth
//!                │          optimization   optimization
//!                │             │             │
//!                └─────────────┼─────────────┘
//!                              ▼
//!                       optimized Quantum IR
//! ```
//!
//! # Purpose
//!
//! This pass performs exact, local, deterministic reduction of contiguous
//! `T`/`Tdg` operations acting on the same logical qubit.
//!
//! It recognizes the exact Clifford+T identities:
//!
//! ```text
//! T^0  = I
//! T^1  = T
//! T^2  = S
//! T^3  = S T
//! T^4  = Z
//! T^5  = Z T
//! T^6  = Sdg
//! T^7  = Tdg
//! T^8  = I
//! ```
//!
//! Negative powers are handled modulo eight:
//!
//! ```text
//! T^-1 = Tdg
//! T^-2 = Sdg
//! T^-4 = Z
//! ```
//!
//! This gives exact reduction for arbitrary finite contiguous runs without
//! introducing a floating-point approximation or an artificial circuit-size
//! ceiling.
//!
//! # What this pass DOES
//!
//! - uses the canonical `quantum::ir::Gate`;
//! - uses the canonical `quantum::ir::QuantumCircuit`;
//! - recognizes `T` and `Tdg`;
//! - accumulates contiguous T exponents modulo eight;
//! - combines `T` and `Tdg` in either order;
//! - removes exact `T^8` cycles;
//! - converts `T^2` to `S`;
//! - converts `T^4` to `Z`;
//! - converts `T^6` to `Sdg`;
//! - converts `T^7` to `Tdg`;
//! - represents `T^3` as `S; T`;
//! - represents `T^5` as `Z; T`;
//! - preserves exact unitary semantics;
//! - preserves logical qubit identity;
//! - never crosses another operation;
//! - reaches its local fixed point in one linear scan;
//! - cooperates with `OptimizationContext` cancellation/rewrite limits;
//! - validates the input and output canonical IR;
//! - integrates with the common `OptimizationPass` contract;
//! - remains deterministic;
//! - remains safe Rust;
//! - scales with available memory and configured IR/optimization limits.
//!
//! # What this pass DOES NOT do
//!
//! This pass intentionally does NOT attempt to solve global T-count
//! minimization.
//!
//! It does not:
//!
//! - commute T gates through Clifford gates;
//! - move T gates between circuit locations;
//! - construct phase polynomials;
//! - perform TODD optimization;
//! - perform Reed–Muller decoding;
//! - perform ZX-calculus optimization;
//! - optimize arbitrary Pauli rotations;
//! - optimize T-depth globally;
//! - introduce ancillas;
//! - route qubits;
//! - schedule operations;
//! - access hardware;
//! - access a QPU;
//! - perform approximate synthesis;
//! - ignore global phase unless explicitly allowed by another subsystem.
//!
//! Those responsibilities belong to other optimization passes.
//!
//! In particular, stronger global T-count optimization belongs to the future
//! `algebra::phase_polynomial` / `fault_tolerant::t_count` layers. Published
//! work shows that T-count optimization is substantially harder than local
//! power reduction, including NP-hard formulations of general T-count
//! optimization. The architecture therefore deliberately keeps this pass
//! bounded and predictable.
//!
//! # Semantic policy
//!
//! All transformations in this file are exact.
//!
//! No floating-point tolerance is used.
//!
//! No approximation is used.
//!
//! No global-phase relaxation is used.
//!
//! Every replacement represents exactly the same unitary operation on the
//! affected logical qubit.
//!
//! # Important boundary rule
//!
//! A T run is reduced only while all gates satisfy:
//!
//! 1. the gate is `T` or `Tdg`;
//! 2. the gate has no parameters;
//! 3. the gate has exactly one logical qubit;
//! 4. the logical qubit is identical to the run's logical qubit.
//!
//! The pass does NOT skip an intervening operation.
//!
//! For example:
//!
//! ```text
//! T q0
//! H q0
//! T q0
//! ```
//!
//! is not changed by this pass.
//!
//! Likewise:
//!
//! ```text
//! T q0
//! X q1
//! T q0
//! ```
//!
//! is not merged here even though those operations may commute. Commutation
//! belongs to the dedicated commutation optimizer.
//!
//! This separation makes the pass locally provable and deterministic.
//!
//! # Scaling
//!
//! Let `n` be the number of circuit operations.
//!
//! Time complexity:
//!
//! ```text
//! O(n)
//! ```
//!
//! Additional optimizer-owned operation storage:
//!
//! ```text
//! O(n)
//! ```
//!
//! No indexed deletion is performed during transformation. This avoids the
//! accidental O(n²) behavior that can result from repeatedly removing entries
//! from the middle of a `Vec`.
//!
//! There is no hard-coded maximum circuit size in this file.
//!
//! Practical maximum size is governed by:
//!
//! - canonical Quantum IR limits;
//! - optimization limits;
//! - available memory;
//! - cancellation/deadline policy;
//! - Rust allocation limits.
//!
//! # Determinism
//!
//! The pass is deterministic.
//!
//! Given the same validated canonical circuit and the same optimizer policy,
//! the resulting circuit is identical.
//!
//! No random number generator is used.
//!
//! # Transactional mutation
//!
//! The pass first constructs the complete replacement sequence.
//!
//! The canonical circuit is modified only after:
//!
//! - input validation succeeds;
//! - transformation succeeds;
//! - cancellation/resource checks succeed.
//!
//! Therefore ordinary transformation failures cannot leave the circuit
//! partially transformed.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Uses:
//!
//! - `Gate`;
//! - `GateKind`;
//! - `QuantumCircuit`;
//! - `QubitId`.
//!
//! No second gate or circuit representation is introduced.
//!
//! ## `optimization::pass`
//!
//! Implements:
//!
//! `OptimizationPass`
//!
//! with stable identifier:
//!
//! `fault_tolerant.t_gate_reduction`
//!
//! ## `optimization::context`
//!
//! Uses:
//!
//! - `check_cancelled()`;
//! - `record_rewrite()`.
//!
//! The pass does not create a second limit system.
//!
//! ## `optimization::errors`
//!
//! All failures are represented through the canonical `OptimizationError`.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline may run this pass:
//!
//! - once;
//! - repeatedly;
//! - as part of a fixed-point pipeline.
//!
//! The pass itself is locally fixed-point complete because each contiguous
//! T-run is reduced directly to its canonical modulo-eight representation.
//!
//! Running the pass again therefore produces no further change.
//!
//! ## `optimization::fault_tolerant::t_count`
//!
//! This pass provides a cheap first-stage reduction before more expensive
//! global T-count optimization.
//!
//! ## `optimization::fault_tolerant::t_depth`
//!
//! This pass may reduce the number of T gates before T-depth analysis.
//! It does not itself perform T-depth scheduling.
//!
//! ## `optimization::algebra::phase_polynomial`
//!
//! This pass is deliberately compatible with phase-polynomial optimization.
//! Local T powers can be eliminated before phase-polynomial extraction, or the
//! planner can place phase-polynomial optimization before/after this pass
//! according to the selected profile.
//!
//! ## `optimization::verification`
//!
//! Exact identities used here are mathematically semantics-preserving.
//! Pipeline-level semantic verification may still verify the complete
//! optimized circuit.
//!
//! ## `optimization::registry`
//!
//! The registry should register this implementation under:
//!
//! `fault_tolerant.t_gate_reduction`
//!
//! ## `optimization::profile`
//!
//! The production profile already defines the same stable pass identifier.
//!
//! ## `optimization::planner`
//!
//! The planner may select this pass whenever the circuit contains T/Tdg
//! operations or when the fault-tolerant optimization profile requests it.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required or permitted by this module.

#![forbid(unsafe_code)]

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassExecutionResult,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable machine-readable pass identifier.
///
/// This identifier is referenced by optimization profiles, planners,
/// registries, provenance, diagnostics, and configuration files.
pub const PASS_ID: &str = "fault_tolerant.t_gate_reduction";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Fault-Tolerant T-Gate Reduction";

/// Stable pass implementation version.
///
/// Increment when the transformation semantics change in a way that can affect
/// reproducibility/provenance.
pub const PASS_SCHEMA_VERSION: u32 = 1;

/// The Clifford+T phase group has period eight.
///
/// `T^8 = I`.
pub const T_PHASE_PERIOD: i8 = 8;

// =============================================================================
// Pass
// =============================================================================

/// Exact local T/Tdg power-reduction pass.
///
/// The pass is stateless between invocations. All mutable invocation state is
/// supplied through [`OptimizationContext`].
#[derive(Debug, Clone)]
pub struct TGateReductionPass {
    metadata: PassMetadata,
}

impl TGateReductionPass {
    /// Constructs the production T-gate reduction pass.
    ///
    /// Metadata construction is deterministic and uses a compile-time stable
    /// identifier. Failure to construct the identifier is therefore a
    /// programmer error rather than a runtime/input error.
    #[must_use]
    pub fn new() -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect("fault_tolerant.t_gate_reduction is a valid static identifier");

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::FaultTolerant,
        )
        .expect("T-gate reduction pass metadata must be valid")
        .with_description(
            "Exact local reduction of contiguous T/Tdg phase powers modulo eight.",
        )
        .with_scope(PassScope::LocalWindow)
        .with_complexity(PassComplexity::Linear)
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::ReplacesOperations)
        .with_capability(PassCapability::ChangesGateCount)
        .with_capability(PassCapability::ChangesFaultTolerantCost);

        Self { metadata }
    }

    /// Returns the exact local reduction of a contiguous T/Tdg run.
    ///
    /// The returned gates all operate on `qubit`.
    ///
    /// The result is canonical with respect to T-power reduction:
    ///
    /// ```text
    /// exponent 0 -> []
    /// exponent 1 -> [T]
    /// exponent 2 -> [S]
    /// exponent 3 -> [S, T]
    /// exponent 4 -> [Z]
    /// exponent 5 -> [Z, T]
    /// exponent 6 -> [Sdg]
    /// exponent 7 -> [Tdg]
    /// ```
    ///
    /// This helper does not inspect or mutate a circuit.
    pub fn reduce_exponent(
        qubit: QubitId,
        exponent: i8,
    ) -> Result<Vec<Gate>, OptimizationError> {
        let normalized = normalize_exponent(exponent);

        match normalized {
            0 => Ok(Vec::new()),

            1 => Ok(vec![new_gate(GateKind::T, qubit)?]),

            2 => Ok(vec![new_gate(GateKind::S, qubit)?]),

            3 => Ok(vec![
                new_gate(GateKind::S, qubit)?,
                new_gate(GateKind::T, qubit)?,
            ]),

            4 => Ok(vec![new_gate(GateKind::Z, qubit)?]),

            5 => Ok(vec![
                new_gate(GateKind::Z, qubit)?,
                new_gate(GateKind::T, qubit)?,
            ]),

            6 => Ok(vec![new_gate(GateKind::Sdg, qubit)?]),

            7 => Ok(vec![new_gate(GateKind::Tdg, qubit)?]),

            // normalize_exponent() guarantees 0..=7.
            _ => Err(OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                "internal T exponent normalization produced an invalid value",
            )),
        }
    }

    /// Returns the modulo-eight exponent represented by one T-family gate.
    ///
    /// `T` contributes `+1`; `Tdg` contributes `-1`.
    #[must_use]
    pub const fn gate_exponent(gate: &Gate) -> Option<i8> {
        match gate.kind() {
            GateKind::T
                if gate.parameters().is_empty()
                    && gate.qubits().len() == 1 =>
            {
                Some(1)
            }

            GateKind::Tdg
                if gate.parameters().is_empty()
                    && gate.qubits().len() == 1 =>
            {
                Some(-1)
            }

            _ => None,
        }
    }

    /// Returns whether `gate` can participate in a T-power run.
    #[must_use]
    pub fn is_t_gate(gate: &Gate) -> bool {
        Self::gate_exponent(gate).is_some()
    }

    /// Reduces one complete contiguous T/Tdg run.
    ///
    /// The input must contain only T-family gates operating on the same
    /// logical qubit. The function validates that precondition and returns a
    /// structured optimizer error if it is violated.
    pub fn reduce_run(
        gates: &[Gate],
    ) -> Result<Vec<Gate>, OptimizationError> {
        if gates.is_empty() {
            return Ok(Vec::new());
        }

        let first_qubit = gates
            .first()
            .and_then(Gate::qubit)
            .ok_or_else(|| {
                OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run has no logical qubit",
                )
            })?;

        let mut exponent = 0i8;

        for gate in gates {
            if gate.qubits().len() != 1 {
                return Err(OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run contains an operation with invalid arity",
                ));
            }

            if gate.qubit() != Some(first_qubit) {
                return Err(OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run contains multiple logical qubits",
                ));
            }

            let contribution = Self::gate_exponent(gate).ok_or_else(|| {
                OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run contains a non-T operation",
                )
            })?;

            exponent = add_mod_eight(exponent, contribution);
        }

        Self::reduce_exponent(first_qubit, exponent)
    }

    /// Returns whether two T-family gates can be combined into the same local
    /// phase run.
    ///
    /// This deliberately requires adjacency and the same logical qubit.
    /// Commutation across intervening gates belongs to another pass.
    #[must_use]
    pub fn can_combine(first: &Gate, second: &Gate) -> bool {
        match (
            Self::gate_exponent(first),
            Self::gate_exponent(second),
        ) {
            (Some(_), Some(_)) => first.qubits() == second.qubits(),
            _ => false,
        }
    }

    /// Returns the normalized modulo-eight exponent represented by a slice of
    /// contiguous T/Tdg operations.
    pub fn run_exponent(
        gates: &[Gate],
    ) -> Result<i8, OptimizationError> {
        if gates.is_empty() {
            return Ok(0);
        }

        let first_qubit = gates
            .first()
            .and_then(Gate::qubit)
            .ok_or_else(|| {
                OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run has no logical qubit",
                )
            })?;

        let mut exponent = 0i8;

        for gate in gates {
            if gate.qubits().len() != 1
                || gate.qubit() != Some(first_qubit)
            {
                return Err(OptimizationError::invalid_input(
                    OptimizationStage::FaultTolerantOptimization,
                    "T-gate run contains incompatible logical operands",
                ));
            }

            let contribution =
                Self::gate_exponent(gate).ok_or_else(|| {
                    OptimizationError::invalid_input(
                        OptimizationStage::FaultTolerantOptimization,
                        "T-gate run contains a non-T operation",
                    )
                })?;

            exponent = add_mod_eight(exponent, contribution);
        }

        Ok(normalize_exponent(exponent))
    }

    /// Performs the complete linear transformation without mutating the
    /// canonical circuit.
    fn transform(
        &self,
        operations: &[Gate],
        context: &mut OptimizationContext,
    ) -> Result<Vec<Gate>, OptimizationError> {
        let mut output = Vec::with_capacity(operations.len());

        let mut index = 0usize;

        while index < operations.len() {
            context
                .check_cancelled()
                .map_err(|error| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        format!(
                            "T-gate reduction cancellation check failed: {error}"
                        ),
                    )
                })?;

            let current = &operations[index];

            // Non-T operation: preserve it exactly and move on.
            if !Self::is_t_gate(current) {
                output.push(current.clone());
                index += 1;
                continue;
            }

            let qubit = current
                .qubit()
                .ok_or_else(|| {
                    OptimizationError::invalid_input(
                        OptimizationStage::FaultTolerantOptimization,
                        "T/Tdg gate has no logical qubit",
                    )
                })?;

            // Find the maximal contiguous run of T/Tdg gates on this same
            // logical qubit.
            //
            // We intentionally stop at EVERY non-T operation. We do not
            // commute through other operations here.
            let run_start = index;
            let mut run_end = index;

            while run_end < operations.len() {
                let candidate = &operations[run_end];

                if !Self::is_t_gate(candidate)
                    || candidate.qubit() != Some(qubit)
                {
                    break;
                }

                run_end += 1;
            }

            let run = &operations[run_start..run_end];

            let reduced = Self::reduce_run(run)?;

            // A run containing one T-family operation is already canonical.
            // Avoid recording a rewrite unnecessarily.
            let changed = reduced.len() != run.len()
                || !same_gate_sequence(run, &reduced);

            if changed {
                context
                    .record_rewrite()
                    .map_err(|error| {
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            format!(
                                "failed to record T-gate reduction rewrite: {error}"
                            ),
                        )
                    })?;
            }

            output.extend(reduced);

            index = run_end;
        }

        Ok(output)
    }

    /// Returns the number of T-family gates in a canonical operation slice.
    #[must_use]
    pub fn count_t_gates(operations: &[Gate]) -> usize {
        operations
            .iter()
            .filter(|gate| Self::is_t_gate(gate))
            .count()
    }

    /// Returns the number of contiguous T/Tdg runs in a canonical operation
    /// slice.
    ///
    /// This is useful for diagnostics and future T-depth preprocessing.
    #[must_use]
    pub fn count_t_runs(operations: &[Gate]) -> usize {
        let mut runs = 0usize;
        let mut previous: Option<QubitId> = None;

        for gate in operations {
            match (Self::gate_exponent(gate), gate.qubit()) {
                (Some(_), Some(qubit)) if previous == Some(qubit) => {}

                (Some(_), Some(qubit)) => {
                    runs += 1;
                    previous = Some(qubit);
                }

                _ => {
                    previous = None;
                }
            }
        }

        runs
    }
}

impl Default for TGateReductionPass {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for TGateReductionPass {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult {
        context
            .check_cancelled()
            .map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "T-gate reduction cannot start: {error}"
                    ),
                )
            })?;

        let operations_before = circuit.len();

        if operations_before == 0 {
            return Ok(PassOutcome::unchanged(0, 0));
        }

        // Validate before transformation.
        //
        // This is particularly important for fault-tolerant optimization
        // because silently interpreting malformed gate data as a phase gate
        // could invalidate resource accounting.
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "T-gate reduction received invalid Quantum IR: {error}"
                ),
            )
        })?;

        // Transform into a separate vector.
        //
        // This guarantees that ordinary transformation failures cannot leave
        // the canonical circuit partially modified.
        let optimized =
            self.transform(circuit.operations(), context)?;

        let operations_after = optimized.len();

        if operations_before == operations_after
            && same_gate_sequence(circuit.operations(), &optimized)
        {
            return Ok(PassOutcome::unchanged(
                usize_to_u64(
                    operations_before,
                    "operation count before T-gate reduction",
                )?,
                usize_to_u64(
                    operations_after,
                    "operation count after T-gate reduction",
                )?,
            ));
        }

        context
            .check_cancelled()
            .map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "T-gate reduction was cancelled before commit: {error}"
                    ),
                )
            })?;

        // This pass is deletion/replacement-only. It never creates more
        // operations than the original T run because every normalized exponent
        // representation has at most two gates and:
        //
        //   3 -> 2
        //   5 -> 2
        //
        // while all other reduced forms contain <= original run size.
        //
        // For a one-gate run, the output is identical.
        //
        // The canonical circuit's existing resource policy therefore remains
        // sufficient.
        circuit.clear();

        for gate in optimized {
            circuit.push(gate).map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "failed to commit T-gate reduction result: {error}"
                    ),
                )
            })?;
        }

        // Final canonical IR validation.
        circuit.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "T-gate reduction produced invalid Quantum IR: {error}"
                ),
            )
        })?;

        Ok(PassOutcome::changed(
            usize_to_u64(
                operations_before,
                "operation count before T-gate reduction",
            )?,
            usize_to_u64(
                operations_after,
                "operation count after T-gate reduction",
            )?,
        ))
    }
}

// =============================================================================
// Mathematical helpers
// =============================================================================

/// Normalizes any small signed exponent into `[0, 7]`.
///
/// The function uses only integer arithmetic and therefore has no floating
/// point or precision concerns.
#[must_use]
pub const fn normalize_exponent(exponent: i8) -> i8 {
    let remainder = exponent % T_PHASE_PERIOD;

    if remainder < 0 {
        remainder + T_PHASE_PERIOD
    } else {
        remainder
    }
}

/// Adds two T exponents modulo eight.
///
/// The values are intentionally kept in the small `i8` domain so a circuit
/// containing an arbitrarily large number of T gates cannot overflow a running
/// exponent counter.
#[must_use]
pub const fn add_mod_eight(left: i8, right: i8) -> i8 {
    normalize_exponent(
        normalize_exponent(left) + normalize_exponent(right),
    )
}

/// Creates one parameter-free canonical gate on one logical qubit.
///
/// All gates emitted by this pass are standard canonical IR gates.
fn new_gate(
    kind: GateKind,
    qubit: QubitId,
) -> Result<Gate, OptimizationError> {
    Gate::new(
        kind,
        vec![qubit],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        OptimizationError::internal(
            OptimizationStage::FaultTolerantOptimization,
            format!(
                "failed to construct canonical {kind:?} gate during T reduction: {error}"
            ),
        )
    })
}

/// Returns whether two gate slices are structurally identical.
///
/// This comparison is intentionally exact. No numerical tolerance is used.
#[must_use]
fn same_gate_sequence(
    left: &[Gate],
    right: &[Gate],
) -> bool {
    left == right
}

/// Converts a platform `usize` counter to the common optimizer `u64`
/// representation without silent truncation.
fn usize_to_u64(
    value: usize,
    what: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::internal(
            OptimizationStage::FaultTolerantOptimization,
            format!(
                "{what} cannot be represented by the optimizer's u64 counter"
            ),
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::QuantumCircuit;
    use crate::quantum::optimization::config::OptimizationConfig;
    use crate::quantum::optimization::limits::OptimizationLimits;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit identifier should be valid")
    }

    fn context() -> OptimizationContext {
        OptimizationContext::new(
            OptimizationConfig::default(),
            OptimizationLimits::production(),
        )
        .expect("production optimization context should construct")
    }

    fn gate(
        kind: GateKind,
        qubit: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(qubit)],
            Vec::<Parameter>::new(),
            None,
            None,
        )
        .expect("test gate should be valid")
    }

    fn circuit_with(
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(8, 8)
                .expect("test circuit should construct");

        for operation in gates {
            circuit
                .push(operation)
                .expect("test gate should be accepted");
        }

        circuit
    }

    fn kinds(
        gates: &[Gate],
    ) -> Vec<GateKind> {
        gates.iter().map(Gate::kind).collect()
    }

    #[test]
    fn metadata_has_stable_identifier() {
        let pass = TGateReductionPass::new();

        assert_eq!(
            pass.metadata().id().as_str(),
            PASS_ID
        );
    }

    #[test]
    fn exponent_normalization_is_modulo_eight() {
        assert_eq!(normalize_exponent(0), 0);
        assert_eq!(normalize_exponent(1), 1);
        assert_eq!(normalize_exponent(7), 7);
        assert_eq!(normalize_exponent(8), 0);
        assert_eq!(normalize_exponent(9), 1);
        assert_eq!(normalize_exponent(-1), 7);
        assert_eq!(normalize_exponent(-2), 6);
        assert_eq!(normalize_exponent(-8), 0);
        assert_eq!(normalize_exponent(-9), 7);
    }

    #[test]
    fn t_gate_has_positive_exponent() {
        let gate = gate(GateKind::T, 0);

        assert_eq!(
            TGateReductionPass::gate_exponent(&gate),
            Some(1)
        );
    }

    #[test]
    fn tdg_gate_has_negative_exponent() {
        let gate = gate(GateKind::Tdg, 0);

        assert_eq!(
            TGateReductionPass::gate_exponent(&gate),
            Some(-1)
        );
    }

    #[test]
    fn ordinary_gate_is_not_t_gate() {
        let gate = gate(GateKind::H, 0);

        assert_eq!(
            TGateReductionPass::gate_exponent(&gate),
            None
        );
    }

    #[test]
    fn t_squared_becomes_s() {
        let result = TGateReductionPass::reduce_exponent(q(0), 2)
            .expect("T^2 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::S]
        );
    }

    #[test]
    fn t_cubed_becomes_s_then_t() {
        let result = TGateReductionPass::reduce_exponent(q(0), 3)
            .expect("T^3 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::S, GateKind::T]
        );
    }

    #[test]
    fn t_four_becomes_z() {
        let result = TGateReductionPass::reduce_exponent(q(0), 4)
            .expect("T^4 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Z]
        );
    }

    #[test]
    fn t_five_becomes_z_then_t() {
        let result = TGateReductionPass::reduce_exponent(q(0), 5)
            .expect("T^5 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Z, GateKind::T]
        );
    }

    #[test]
    fn t_six_becomes_sdg() {
        let result = TGateReductionPass::reduce_exponent(q(0), 6)
            .expect("T^6 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Sdg]
        );
    }

    #[test]
    fn t_seven_becomes_tdg() {
        let result = TGateReductionPass::reduce_exponent(q(0), 7)
            .expect("T^7 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Tdg]
        );
    }

    #[test]
    fn t_eight_becomes_identity_sequence() {
        let result = TGateReductionPass::reduce_exponent(q(0), 8)
            .expect("T^8 should reduce");

        assert!(result.is_empty());
    }

    #[test]
    fn negative_one_becomes_tdg() {
        let result = TGateReductionPass::reduce_exponent(q(0), -1)
            .expect("T^-1 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Tdg]
        );
    }

    #[test]
    fn negative_two_becomes_sdg() {
        let result = TGateReductionPass::reduce_exponent(q(0), -2)
            .expect("T^-2 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Sdg]
        );
    }

    #[test]
    fn negative_four_becomes_z() {
        let result = TGateReductionPass::reduce_exponent(q(0), -4)
            .expect("T^-4 should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::Z]
        );
    }

    #[test]
    fn mixed_t_and_tdg_cancel() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
        ];

        let result = TGateReductionPass::reduce_run(&gates)
            .expect("T Tdg should cancel");

        assert!(result.is_empty());
    }

    #[test]
    fn mixed_run_reduces_modulo_eight() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 0),
        ];

        // Net exponent = 1 + 1 - 1 + 1 = 2.
        let result = TGateReductionPass::reduce_run(&gates)
            .expect("mixed T run should reduce");

        assert_eq!(
            kinds(&result),
            vec![GateKind::S]
        );
    }

    #[test]
    fn different_qubits_do_not_combine() {
        let first = gate(GateKind::T, 0);
        let second = gate(GateKind::T, 1);

        assert!(
            !TGateReductionPass::can_combine(
                &first,
                &second
            )
        );
    }

    #[test]
    fn same_qubit_t_family_gates_combine() {
        let first = gate(GateKind::T, 0);
        let second = gate(GateKind::Tdg, 0);

        assert!(
            TGateReductionPass::can_combine(
                &first,
                &second
            )
        );
    }

    #[test]
    fn t_run_count_is_correct() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::H, 0),
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 1),
        ];

        assert_eq!(
            TGateReductionPass::count_t_gates(&gates),
            5
        );

        assert_eq!(
            TGateReductionPass::count_t_runs(&gates),
            3
        );
    }

    #[test]
    fn pass_reduces_eight_t_gates_to_empty() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("T^8 optimization should succeed");

        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn pass_reduces_three_t_gates() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("T^3 optimization should succeed");

        assert_eq!(
            kinds(circuit.operations()),
            vec![GateKind::S, GateKind::T]
        );
    }

    #[test]
    fn pass_reduces_five_t_gates() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("T^5 optimization should succeed");

        assert_eq!(
            kinds(circuit.operations()),
            vec![GateKind::Z, GateKind::T]
        );
    }

    #[test]
    fn pass_reduces_t_tdg() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("T Tdg optimization should succeed");

        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn pass_reduces_tdg_t() {
        let gates = vec![
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 0),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("Tdg T optimization should succeed");

        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn pass_does_not_cross_hadamard() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::H, 0),
            gate(GateKind::T, 0),
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn pass_does_not_cross_measurement() {
        let t0 = gate(GateKind::T, 0);

        let measurement = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement should be valid");

        let gates = vec![
            t0.clone(),
            measurement.clone(),
            t0,
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn pass_does_not_cross_reset() {
        let t0 = gate(GateKind::T, 0);

        let reset = Gate::new(
            GateKind::Reset,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("reset should be valid");

        let gates = vec![
            t0.clone(),
            reset.clone(),
            t0,
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn pass_does_not_cross_barrier() {
        let t0 = gate(GateKind::T, 0);

        let barrier = Gate::new(
            GateKind::Barrier,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("barrier should be valid");

        let gates = vec![
            t0.clone(),
            barrier.clone(),
            t0,
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn pass_does_not_combine_different_qubits() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 1),
            gate(GateKind::Tdg, 0),
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn pass_is_idempotent() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 1),
            gate(GateKind::Tdg, 1),
        ];

        let mut circuit = circuit_with(gates);
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("first optimization should succeed");

        let first_result =
            circuit.operations().to_vec();

        pass.run(&mut circuit, &mut context)
            .expect("second optimization should succeed");

        assert_eq!(
            circuit.operations(),
            first_result.as_slice()
        );
    }

    #[test]
    fn pass_preserves_non_t_operations() {
        let gates = vec![
            gate(GateKind::H, 0),
            gate(GateKind::X, 1),
            gate(GateKind::CX, 0),
        ];

        let mut circuit = circuit_with(gates.clone());
        let mut context = context();
        let pass = TGateReductionPass::new();

        pass.run(&mut circuit, &mut context)
            .expect("optimization should succeed");

        assert_eq!(
            circuit.operations(),
            gates.as_slice()
        );
    }

    #[test]
    fn run_exponent_matches_reduction() {
        let gates = vec![
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 0),
        ];

        assert_eq!(
            TGateReductionPass::run_exponent(&gates)
                .expect("run exponent should succeed"),
            2
        );
    }

    #[test]
fn very_large_run_does_not_accumulate_unbounded_exponent() {
    let mut exponent = 0i8;

    for _ in 0..1_000_000usize {
        exponent = add_mod_eight(exponent, 1);
    }

    assert_eq!(exponent, 1_000_000usize.rem_euclid(8) as i8);
}