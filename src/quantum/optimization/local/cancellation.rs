//! Zamani Quantum Optimization — Local Cancellation Pass
//!
//! Production-grade local cancellation for the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir::QuantumCircuit
//!      │
//!      ▼
//! optimization::local::cancellation
//!      │
//!      ▼
//! optimized quantum::ir::QuantumCircuit
//! ```
//!
//! This pass deliberately operates on the canonical:
//!
//! - [`crate::quantum::ir::Gate`];
//! - [`crate::quantum::ir::GateKind`];
//! - [`crate::quantum::ir::Parameter`];
//! - [`crate::quantum::ir::QuantumCircuit`];
//!
//! It does NOT define another `QuantumGate`, `QuantumOperation`, or circuit
//! representation.
//!
//! # Responsibilities
//!
//! This pass performs only local, exact-semantic cancellation:
//!
//! - identity removal;
//! - adjacent self-inverse gate cancellation;
//! - adjacent explicit inverse-pair cancellation;
//! - exact inverse cancellation for selected parameterized rotations;
//! - exact zero-angle removal for selected one-angle operations;
//! - cascading cancellation exposed by previous cancellations.
//!
//! The implementation intentionally does NOT perform:
//!
//! - general commutation;
//! - non-local gate movement;
//! - arbitrary symbolic algebra;
//! - phase-polynomial optimization;
//! - synthesis;
//! - routing;
//! - scheduling;
//! - hardware interaction;
//! - approximate transformations;
//! - global-phase-relaxed transformations.
//!
//! Those responsibilities belong to other optimization layers.
//!
//! # Semantic conservatism
//!
//! Cancellation is exact. This implementation does not use floating-point
//! tolerances when deciding whether two parameterized operations are inverses.
//! Approximate equality can change program semantics and therefore belongs to
//! an explicitly approximate optimization/verification policy.
//!
//! In particular:
//!
//! - `RX(theta); RX(-theta)` cancels when the inverse relationship is
//!   structurally or numerically exact;
//! - `RX(2π)` is NOT removed merely because it is a global phase;
//! - `RZ(2π)` is NOT removed merely because a measurement-only equivalence
//!   might permit it;
//! - symbolic expressions are not algebraically transformed here unless their
//!   inverse relationship is structurally explicit.
//!
//! # Scaling
//!
//! The algorithm is O(n) in the number of circuit operations for the normal
//! straight-line circuit representation.
//!
//! It uses an optimizer-local `Vec<Gate>` containing at most one copy of the
//! circuit's operation sequence. The canonical circuit is mutated only after
//! the transformation has completed successfully.
//!
//! This avoids repeated indexed deletion, which can otherwise make a large
//! cancellation pass accidentally quadratic.
//!
//! There is no artificial circuit-size ceiling in this file. Actual scaling is
//! governed by:
//!
//! - available memory;
//! - canonical Quantum IR limits;
//! - optimization resource limits;
//! - context cancellation/deadline policy.
//!
//! # Determinism
//!
//! The pass is deterministic.
//!
//! Equal input circuits under the same canonical IR representation produce the
//! same output circuit.
//!
//! # Thread safety
//!
//! The pass contains immutable metadata only and implements the shared
//! `OptimizationPass` contract. Invocation-specific mutable state remains in
//! `OptimizationContext`.
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
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This file consumes the canonical IR directly. No duplicate gate model is
//! introduced.
//!
//! ## `optimization::pass`
//!
//! Implements [`OptimizationPass`] and declares the pass metadata:
//!
//! `local.cancellation`
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
//! All failures are returned through the canonical `OptimizationError`.
//! There is deliberately no `CancellationError`.
//!
//! ## `optimization::circuit`
//!
//! The generic transactional editor is not required for this particular
//! implementation because the pass constructs the complete replacement
//! operation sequence before touching the canonical circuit. This keeps the
//! transformation linear rather than repeatedly shifting a large operation
//! vector.
//!
//! ## `optimization::statistics`
//!
//! Detailed statistics remain owned by the common optimization statistics
//! system. This pass reports the standardized `PassOutcome`.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline decides whether this pass executes once or participates in a
//! larger optimization pipeline. This pass itself reaches its local fixed
//! point in one scan because cancellation is performed using a stack.
//!
//! ## `optimization::verification`
//!
//! Exact cancellation is mathematically semantics-preserving. Global pipeline
//! verification may nevertheless verify the complete optimized circuit.
//!
//! ## `optimization::local::rotation`
//!
//! General rotation fusion belongs there. This file only performs exact
//! parameterized inverse cancellation and exact zero-angle removal.
//!
//! ## `optimization::local::commutation`
//!
//! Cancellation across intervening commuting operations belongs there. This
//! pass intentionally requires the cancellable operations to become adjacent.
//!
//! # Important invariant
//!
//! The canonical circuit is never partially transformed because of an ordinary
//! optimizer failure. The replacement vector is completely constructed before
//! the circuit is cleared and rebuilt.
//!
//! `QuantumCircuit::clear()` followed by `push()` is safe here because every
//! element in the replacement vector originated from an already-valid
//! canonical circuit and the replacement sequence can never contain more
//! operations than the input circuit.
//!
//! No unsafe code is necessary.
//!
//! # Examples
//!
//! ```text
//! X q0; X q0
//!     ↓
//! <empty>
//!
//! H q0; H q0
//!     ↓
//! <empty>
//!
//! CX q0,q1; CX q0,q1
//!     ↓
//! <empty>
//!
//! S q0; Sdg q0
//!     ↓
//! <empty>
//!
//! T q0; Tdg q0
//!     ↓
//! <empty>
//!
//! H q0; X q0; X q0; H q0
//!     ↓
//! <empty>
//!
//! RX(θ) q0; RX(-θ) q0
//!     ↓
//! <empty>
//! ```
//!
//! Measurement, reset, and barrier operations are never crossed by this pass.
//!
//! # Security / robustness
//!
//! This pass does not trust optimizer-generated input blindly. The canonical
//! `QuantumCircuit` API is responsible for maintaining IR invariants, while
//! the pass validates cancellation conditions from the immutable operation
//! representation before making any change.

#![forbid(unsafe_code)]

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
};
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
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

use crate::quantum::optimization::errors::PassIdentifier;

// =============================================================================
// Constants
// =============================================================================

/// Stable identifier for the cancellation pass.
///
/// This identifier is part of optimization provenance and should not be
/// changed casually.
pub const PASS_ID: &str = "local.cancellation";

/// Human-readable pass name.
pub const PASS_NAME: &str = "Local Gate Cancellation";

// =============================================================================
// Pass
// =============================================================================

/// Exact local gate-cancellation optimization pass.
///
/// The pass is stateless between invocations. All invocation-specific state
/// belongs to [`OptimizationContext`].
#[derive(Debug, Clone)]
pub struct CancellationPass {
    metadata: PassMetadata,
}

impl CancellationPass {
    /// Creates a production cancellation pass.
    ///
    /// The pass metadata is static and deterministic. Failure to construct the
    /// static identifier would indicate a programmer error rather than an
    /// input/runtime condition.
    #[must_use]
    pub fn new() -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect("local.cancellation has a valid static identifier");

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::LocalRewrite,
        )
        .expect("local cancellation metadata must be valid")
        .with_scope(PassScope::LocalWindow)
        .with_complexity(PassComplexity::Linear)
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::ChangesGateCount);

        Self { metadata }
    }

    /// Performs the complete local cancellation transformation.
    ///
    /// This method constructs the optimized operation sequence without
    /// mutating the canonical circuit.
    fn transform(
        &self,
        operations: &[Gate],
        context: &mut OptimizationContext,
    ) -> Result<Vec<Gate>, OptimizationError> {
        let mut output = Vec::with_capacity(operations.len());

        for gate in operations {
            context
                .check_cancelled()
                .map_err(|error| {
                    OptimizationError::internal(
                        OptimizationStage::LocalOptimization,
                        format!(
                            "local cancellation context check failed: {error}"
                        ),
                    )
                })?;

            // An exact identity operation is always removable.
            //
            // This is independent of neighboring operations and therefore
            // does not cross or alter semantic boundaries.
            if is_exact_identity_gate(gate) {
                context
                    .record_rewrite()
                    .map_err(|error| {
                        OptimizationError::internal(
                            OptimizationStage::LocalOptimization,
                            format!(
                                "failed to record identity removal: {error}"
                            ),
                        )
                    })?;

                continue;
            }

            // The stack-like output makes cascading cancellation naturally
            // reach a local fixed point in one O(n) traversal.
            if let Some(previous) = output.last() {
                if gates_cancel(previous, gate) {
                    output.pop();

                    context
                        .record_rewrite()
                        .map_err(|error| {
                            OptimizationError::internal(
                                OptimizationStage::LocalOptimization,
                                format!(
                                    "failed to record gate cancellation: {error}"
                                ),
                            )
                        })?;

                    continue;
                }
            }

            // Exact zero-angle operations are identities.
            //
            // This is deliberately checked after the neighboring cancellation
            // test so the ordinary stack semantics remain obvious.
            if is_exact_zero_rotation(gate) {
                context
                    .record_rewrite()
                    .map_err(|error| {
                        OptimizationError::internal(
                            OptimizationStage::LocalOptimization,
                            format!(
                                "failed to record zero-angle removal: {error}"
                            ),
                        )
                    })?;

                continue;
            }

            output.push(gate.clone());
        }

        Ok(output)
    }

    /// Returns whether two canonical gates cancel exactly.
    ///
    /// This is public because other local optimizer components and tests may
    /// need to query the exact cancellation relation without duplicating gate
    /// semantics.
    #[must_use]
    pub fn can_cancel(first: &Gate, second: &Gate) -> bool {
        gates_cancel(first, second)
    }

    /// Returns whether a canonical gate is an exact identity operation that
    /// this pass is permitted to remove.
    #[must_use]
    pub fn is_identity(gate: &Gate) -> bool {
        is_exact_identity_gate(gate) || is_exact_zero_rotation(gate)
    }
}

impl Default for CancellationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for CancellationPass {
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
                    OptimizationStage::LocalOptimization,
                    format!(
                        "local cancellation cannot start: {error}"
                    ),
                )
            })?;

        let operations_before = circuit.len();

        if operations_before == 0 {
            return Ok(PassOutcome::unchanged(0, 0));
        }

        // The canonical circuit has already been constructed through validated
        // mutation APIs, but a pass must not assume externally reconstructed IR
        // is valid forever.
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "local cancellation received invalid Quantum IR: {error}"
                ),
            )
        })?;

        // IMPORTANT:
        //
        // Do not repeatedly call circuit.remove(index). Indexed removal shifts
        // the tail of a Vec and can make a large cancellation workload
        // quadratic.
        //
        // Instead, perform one linear transformation into an optimizer-local
        // vector and mutate the canonical circuit only after the complete
        // transformation succeeds.
        let optimized =
            self.transform(circuit.operations(), context)?;

        let operations_after = optimized.len();

        if operations_after == operations_before {
            return Ok(PassOutcome::unchanged(
                usize_to_u64(
                    operations_before,
                    "operation count before cancellation",
                )?,
                usize_to_u64(
                    operations_after,
                    "operation count after cancellation",
                )?,
            ));
        }

        context
            .check_cancelled()
            .map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::LocalOptimization,
                    format!(
                        "local cancellation was cancelled before commit: {error}"
                    ),
                )
            })?;

        // Every optimized operation originated from the validated input
        // circuit. The output can never contain more operations than the input.
        //
        // Therefore the canonical circuit's operation-count limit cannot be
        // exceeded by this transformation.
        //
        // clear() + push() is used instead of repeated indexed removal to keep
        // the commit O(n).
        circuit.clear();

        for gate in optimized {
            circuit.push(gate).map_err(|error| {
                // This should be unreachable for a valid canonical circuit and
                // a deletion-only transformation. Returning an error is still
                // preferable to silently accepting an inconsistent IR.
                OptimizationError::internal(
                    OptimizationStage::LocalOptimization,
                    format!(
                        "failed to commit locally cancelled gate sequence: {error}"
                    ),
                )
            })?;
        }

        // The canonical mutation API validates each inserted operation. Perform
        // one complete postcondition validation as the final semantic boundary.
        circuit.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::LocalOptimization,
                format!(
                    "local cancellation produced invalid Quantum IR: {error}"
                ),
            )
        })?;

        Ok(PassOutcome::changed(
            usize_to_u64(
                operations_before,
                "operation count before cancellation",
            )?,
            usize_to_u64(
                operations_after,
                "operation count after cancellation",
            )?,
        ))
    }
}

// =============================================================================
// Exact cancellation semantics
// =============================================================================

/// Determines whether two canonical gates are exact inverses under the
/// semantics supported by this pass.
///
/// The relation is deliberately conservative:
///
/// - operands must match in exact canonical order;
/// - measurements/barriers/resets are never treated as unitary cancellation
///   operations;
/// - only gate kinds with known exact inverse semantics are accepted;
/// - parameterized inverses require exact numerical or structural negation.
///
/// No approximate floating-point comparison is performed.
fn gates_cancel(
    first: &Gate,
    second: &Gate,
) -> bool {
    // Non-unitary operations are semantic boundaries and are never canceled
    // against another operation by this pass.
    if !first.is_unitary()
        || !second.is_unitary()
        || first.is_measurement()
        || second.is_measurement()
        || first.is_barrier()
        || second.is_barrier()
        || first.is_reset()
        || second.is_reset()
    {
        return false;
    }

    // Operand order is semantically significant for the canonical IR.
    //
    // This intentionally does not assume symmetry for gates such as CX.
    if first.qubits() != second.qubits() {
        return false;
    }

    match (first.kind(), second.kind()) {
        // Explicit inverse pairs.
        (GateKind::S, GateKind::Sdg)
        | (GateKind::Sdg, GateKind::S)
        | (GateKind::T, GateKind::Tdg)
        | (GateKind::Tdg, GateKind::T)
        | (GateKind::V, GateKind::Vdg)
        | (GateKind::Vdg, GateKind::V) => {
            first.parameters().is_empty()
                && second.parameters().is_empty()
        }

        // Self-inverse gates.
        (first_kind, second_kind)
            if first_kind == second_kind
                && first_kind.is_self_inverse() =>
        {
            first.parameters().is_empty()
                && second.parameters().is_empty()
        }

        // Parameterized one-angle gates whose inverse is obtained by negating
        // the angle.
        (GateKind::RX, GateKind::RX)
        | (GateKind::RY, GateKind::RY)
        | (GateKind::RZ, GateKind::RZ)
        | (GateKind::Phase, GateKind::Phase)
        | (GateKind::U1, GateKind::U1)
        | (GateKind::CRX, GateKind::CRX)
        | (GateKind::CRY, GateKind::CRY)
        | (GateKind::CRZ, GateKind::CRZ) => {
            one_parameter_inverse(
                first.parameters(),
                second.parameters(),
            )
        }

        _ => false,
    }
}

/// Checks the exact inverse relation for a one-parameter operation.
fn one_parameter_inverse(
    first: &[Parameter],
    second: &[Parameter],
) -> bool {
    if first.len() != 1 || second.len() != 1 {
        return false;
    }

    parameters_are_exact_negatives(
        &first[0],
        &second[0],
    )
}

/// Determines whether two canonical parameters are exact negatives.
///
/// The function deliberately avoids numerical tolerances.
///
/// Supported forms:
///
/// ```text
/// Constant(a), Constant(-a)
/// Symbol(x), Negate(Symbol(x))
/// Negate(Symbol(x)), Symbol(x)
/// Negate(expr), expr
/// expr, Negate(expr)
/// ```
///
/// More complicated algebraic equivalence belongs to the parameter
/// simplification/algebra subsystem.
fn parameters_are_exact_negatives(
    first: &Parameter,
    second: &Parameter,
) -> bool {
    match (first, second) {
        (Parameter::Constant(a), Parameter::Constant(b)) => {
            a.is_finite()
                && b.is_finite()
                && *a + *b == 0.0
        }

        (
            Parameter::Symbol(first_symbol),
            Parameter::Expression(second_expression),
        ) => {
            matches!(
                second_expression.as_ref(),
                ParameterExpression::Negate(inner)
                    if inner.as_ref()
                        == &Parameter::Symbol(first_symbol.clone())
            )
        }

        (
            Parameter::Expression(first_expression),
            Parameter::Symbol(second_symbol),
        ) => {
            matches!(
                first_expression.as_ref(),
                ParameterExpression::Negate(inner)
                    if inner.as_ref()
                        == &Parameter::Symbol(second_symbol.clone())
            )
        }

        (
            Parameter::Expression(first_expression),
            second_parameter,
        ) => {
            matches!(
                first_expression.as_ref(),
                ParameterExpression::Negate(inner)
                    if inner.as_ref() == second_parameter
            )
        }

        (
            first_parameter,
            Parameter::Expression(second_expression),
        ) => {
            matches!(
                second_expression.as_ref(),
                ParameterExpression::Negate(inner)
                    if inner.as_ref() == first_parameter
            )
        }

        _ => false,
    }
}

// =============================================================================
// Identity recognition
// =============================================================================

/// Returns true when a gate is an unconditional exact identity.
fn is_exact_identity_gate(gate: &Gate) -> bool {
    gate.kind() == GateKind::I
        && gate.parameters().is_empty()
        && gate.classical_target().is_none()
        && gate.measurement().is_none()
}

/// Returns true when a supported one-angle gate has exactly zero numerical
/// angle.
///
/// This is exact and therefore does not rely on a floating-point tolerance.
///
/// We intentionally do not classify arbitrary symbolic expressions that happen
/// to evaluate to zero as identities. Symbolic simplification belongs to the
/// parameter optimization subsystem.
fn is_exact_zero_rotation(gate: &Gate) -> bool {
    let kind = gate.kind();

    if !matches!(
        kind,
        GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
    ) {
        return false;
    }

    if gate.parameters().len() != 1 {
        return false;
    }

    matches!(
        gate.parameters()[0],
        Parameter::Constant(value)
            if value.is_finite() && value == 0.0
    )
}

// =============================================================================
// Integer conversion
// =============================================================================

/// Converts a platform `usize` counter into the common optimizer `u64`
/// statistics representation.
///
/// The checked conversion prevents silent truncation on platforms where
/// `usize` could theoretically exceed `u64`.
fn usize_to_u64(
    value: usize,
    what: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::internal(
            OptimizationStage::LocalOptimization,
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

    use crate::quantum::ir::gate::Gate;
    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::qubits::QubitId;
    use crate::quantum::ir::QuantumCircuit;
    use crate::quantum::optimization::config::OptimizationConfig;
    use crate::quantum::optimization::context::OptimizationContext;
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

    fn circuit_with(
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(8, 8)
                .expect("test circuit should construct");

        for gate in gates {
            circuit
                .push(gate)
                .expect("test gate should be accepted");
        }

        circuit
    }

    fn gate(
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
        .expect("test gate should be valid")
    }

    // -------------------------------------------------------------------------
    // Exact self-inverse gates
    // -------------------------------------------------------------------------

    #[test]
    fn cancels_x_pair() {
        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            Gate::x(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_h_pair() {
        let mut circuit = circuit_with(vec![
            Gate::h(q(0)).unwrap(),
            Gate::h(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_cx_pair() {
        let mut circuit = circuit_with(vec![
            Gate::cx(q(0), q(1)).unwrap(),
            Gate::cx(q(0), q(1)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_cz_pair() {
        let mut circuit = circuit_with(vec![
            Gate::cz(q(0), q(1)).unwrap(),
            Gate::cz(q(0), q(1)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_swap_pair() {
        let mut circuit = circuit_with(vec![
            Gate::swap(q(0), q(1)).unwrap(),
            Gate::swap(q(0), q(1)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    // -------------------------------------------------------------------------
    // Explicit inverse pairs
    // -------------------------------------------------------------------------

    #[test]
    fn cancels_s_sdg_pair() {
        let mut circuit = circuit_with(vec![
            Gate::s(q(0)).unwrap(),
            Gate::sdg(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_sdg_s_pair() {
        let mut circuit = circuit_with(vec![
            Gate::sdg(q(0)).unwrap(),
            Gate::s(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn cancels_t_tdg_pair() {
        let mut circuit = circuit_with(vec![
            Gate::tdg(q(0)).unwrap(),
            Gate::t(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    // -------------------------------------------------------------------------
    // Parameterized exact inverse pairs
    // -------------------------------------------------------------------------

    #[test]
    fn cancels_constant_rx_inverse_pair() {
        let first = gate(
            GateKind::RX,
            vec![q(0)],
            vec![Parameter::Constant(0.25)],
        );

        let second = gate(
            GateKind::RX,
            vec![q(0)],
            vec![Parameter::Constant(-0.25)],
        );

        assert!(CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    #[test]
    fn cancels_constant_ry_inverse_pair() {
        let first = gate(
            GateKind::RY,
            vec![q(0)],
            vec![Parameter::Constant(0.25)],
        );

        let second = gate(
            GateKind::RY,
            vec![q(0)],
            vec![Parameter::Constant(-0.25)],
        );

        assert!(CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    #[test]
    fn cancels_constant_rz_inverse_pair() {
        let first = gate(
            GateKind::RZ,
            vec![q(0)],
            vec![Parameter::Constant(0.25)],
        );

        let second = gate(
            GateKind::RZ,
            vec![q(0)],
            vec![Parameter::Constant(-0.25)],
        );

        assert!(CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    #[test]
    fn cancels_symbolic_negation_pair() {
        let theta = Parameter::symbol("theta")
            .expect("symbol should be valid");

        let neg_theta = Parameter::expression(
            ParameterExpression::Negate(
                Box::new(theta.clone()),
            ),
        )
        .expect("negated symbol should be valid");

        let first = gate(
            GateKind::RX,
            vec![q(0)],
            vec![theta],
        );

        let second = gate(
            GateKind::RX,
            vec![q(0)],
            vec![neg_theta],
        );

        assert!(CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    // -------------------------------------------------------------------------
    // Exact zero-angle identities
    // -------------------------------------------------------------------------

    #[test]
    fn removes_zero_rx() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RX,
                vec![q(0)],
                vec![Parameter::Constant(0.0)],
            ),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_rz() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RZ,
                vec![q(0)],
                vec![Parameter::Constant(-0.0)],
            ),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    // -------------------------------------------------------------------------
    // Identity
    // -------------------------------------------------------------------------

    #[test]
    fn removes_identity_gate() {
        let mut circuit = circuit_with(vec![
            Gate::identity(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    // -------------------------------------------------------------------------
    // Cascading cancellation
    // -------------------------------------------------------------------------

    #[test]
    fn reaches_local_fixed_point_in_one_scan() {
        // H X X H
        //
        // X X cancels first.
        // The resulting H H then cancels.
        //
        // A naive one-pass adjacent algorithm can miss the second
        // cancellation. The stack-based implementation does not.
        let mut circuit = circuit_with(vec![
            Gate::h(q(0)).unwrap(),
            Gate::x(q(0)).unwrap(),
            Gate::x(q(0)).unwrap(),
            Gate::h(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn odd_number_of_self_inverse_gates_leaves_one() {
        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            Gate::x(q(0)).unwrap(),
            Gate::x(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.operations()[0].kind(),
            GateKind::X
        );
    }

    // -------------------------------------------------------------------------
    // Semantic boundaries
    // -------------------------------------------------------------------------

    #[test]
    fn does_not_cancel_across_measurement() {
        let measurement = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement should be valid");

        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            measurement,
            Gate::x(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert_eq!(circuit.len(), 3);
    }

    #[test]
    fn does_not_cancel_across_barrier() {
        let barrier = Gate::new(
            GateKind::Barrier,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("barrier should be valid");

        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            barrier,
            Gate::x(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert_eq!(circuit.len(), 3);
    }

    #[test]
    fn does_not_cancel_across_reset() {
        let reset = Gate::new(
            GateKind::Reset,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("reset should be valid");

        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            reset,
            Gate::x(q(0)).unwrap(),
        ]);

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("cancellation should succeed");

        assert_eq!(circuit.len(), 3);
    }

    // -------------------------------------------------------------------------
    // Operand safety
    // -------------------------------------------------------------------------

    #[test]
    fn does_not_cancel_same_gate_on_different_qubits() {
        let first = Gate::x(q(0)).unwrap();
        let second = Gate::x(q(1)).unwrap();

        assert!(!CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    #[test]
    fn does_not_reverse_controlled_gate_operands() {
        let first =
            Gate::cx(q(0), q(1)).unwrap();

        let second =
            Gate::cx(q(1), q(0)).unwrap();

        assert!(!CancellationPass::can_cancel(
            &first,
            &second
        ));
    }

    // -------------------------------------------------------------------------
    // Global phase conservatism
    // -------------------------------------------------------------------------

    #[test]
    fn does_not_remove_rx_two_pi_as_identity() {
        let gate = gate(
            GateKind::RX,
            vec![q(0)],
            vec![Parameter::Constant(
                std::f64::consts::TAU,
            )],
        );

        assert!(!CancellationPass::is_identity(&gate));
    }

    #[test]
    fn does_not_remove_rz_two_pi_as_identity() {
        let gate = gate(
            GateKind::RZ,
            vec![q(0)],
            vec![Parameter::Constant(
                std::f64::consts::TAU,
            )],
        );

        assert!(!CancellationPass::is_identity(&gate));
    }

    // -------------------------------------------------------------------------
    // Pass metadata
    // -------------------------------------------------------------------------

    #[test]
    fn metadata_is_stable() {
        let pass = CancellationPass::new();

        assert_eq!(
            pass.metadata().id().as_str(),
            PASS_ID
        );

        assert_eq!(
            pass.metadata().name(),
            PASS_NAME
        );

        assert_eq!(
            pass.metadata().kind(),
            PassKind::LocalRewrite
        );

        assert_eq!(
            pass.metadata().scope(),
            PassScope::LocalWindow
        );

        assert_eq!(
            pass.metadata().complexity(),
            PassComplexity::Linear
        );
    }

    // -------------------------------------------------------------------------
    // No-op behavior
    // -------------------------------------------------------------------------

    #[test]
    fn unchanged_circuit_is_not_rewritten() {
        let mut circuit = circuit_with(vec![
            Gate::x(q(0)).unwrap(),
            Gate::h(q(1)).unwrap(),
        ]);

        let before =
            circuit.operations().to_vec();

        let outcome =
            CancellationPass::new()
                .run(
                    &mut circuit,
                    &mut context(),
                )
                .expect("cancellation should succeed");

        assert!(!outcome.changed());
        assert_eq!(
            circuit.operations(),
            before.as_slice()
        );
    }

    // -------------------------------------------------------------------------
    // Large linear workload
    // -------------------------------------------------------------------------

    #[test]
    fn handles_large_linear_cancellation_workload() {
        let count = 10_000usize;

        let mut circuit =
            QuantumCircuit::new(1, 1)
                .expect("large test circuit should construct");

        for _ in 0..count {
            circuit
                .push(
                    Gate::x(q(0))
                        .expect("x gate should construct"),
                )
                .expect("gate should be inserted");
        }

        CancellationPass::new()
            .run(&mut circuit, &mut context())
            .expect("large cancellation should succeed");

        assert_eq!(
            circuit.len(),
            0
        );
    }
}