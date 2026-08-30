//! Zamani Quantum Optimization — Rotation Fusion
//!
//! Production-grade rotation-combination optimization over the canonical
//! Zamani Quantum IR.
//!
//! # Purpose
//!
//! This pass combines adjacent rotations acting on the same logical qubits:
//!
//! ```text
//! RX(a); RX(b)       -> RX(a + b)
//! RY(a); RY(b)       -> RY(a + b)
//! RZ(a); RZ(b)       -> RZ(a + b)
//! Phase(a); Phase(b) -> Phase(a + b)
//! U1(a); U1(b)      -> U1(a + b)
//!
//! CRX(a); CRX(b)    -> CRX(a + b)
//! CRY(a); CRY(b)    -> CRY(a + b)
//! CRZ(a); CRZ(b)    -> CRZ(a + b)
//! ```
//!
//! The transformation is performed only when the two operations have the
//! same canonical `GateKind`, the same ordered logical operands, and exactly
//! one parameter.
//!
//! # Architectural boundary
//!
//! ```text
//!                  quantum::ir::QuantumCircuit
//!                              │
//!                              ▼
//!                  local::rotation::RotationFusion
//!                              │
//!                              ▼
//!                  optimized QuantumCircuit
//! ```
//!
//! This module:
//!
//! - uses the canonical `crate::quantum::ir::Gate`;
//! - uses the canonical `crate::quantum::ir::Parameter`;
//! - uses the canonical `crate::quantum::ir::ParameterExpression`;
//! - mutates only through validated `QuantumCircuit` APIs;
//! - does not define another quantum IR;
//! - does not perform routing;
//! - does not perform scheduling;
//! - does not communicate with hardware;
//! - does not execute circuits;
//! - does not perform measurement;
//! - does not silently approximate symbolic parameters.
//!
//! # Important semantic rule
//!
//! This pass performs **exact symbolic/constant composition** only.
//!
//! It does not numerically approximate symbolic parameters and does not
//! introduce a tolerance-based equivalence decision. Floating-point
//! normalization is used only for finite concrete constants.
//!
//! # Parameter semantics
//!
//! The canonical IR represents parameters as:
//!
//! - `Parameter::Constant(f64)`;
//! - `Parameter::Symbol(String)`;
//! - `Parameter::Expression(...)`.
//!
//! Therefore this pass has three composition paths:
//!
//! 1. constant + constant:
//!    compute and normalize the finite numerical angle;
//!
//! 2. symbolic/expression + anything:
//!    construct an exact canonical `ParameterExpression::Add`;
//!
//! 3. mathematically zero constant:
//!    remove the resulting rotation entirely.
//!
//! Symbolic expressions are never evaluated without an explicit binding
//! environment.
//!
//! # Complexity
//!
//! The normal implementation is a single forward scan:
//!
//! - time: O(n);
//! - additional working memory: O(1) apart from replacement parameter
//!   allocation;
//! - no recursion over circuit size;
//! - no global state;
//! - no quadratic pair searching;
//! - no circuit-sized auxiliary dependency graph.
//!
//! This allows the pass to scale from tiny circuits to circuits limited only
//! by the canonical IR and optimizer resource policies.
//!
//! # Fixed-point behavior
//!
//! One left-to-right pass is sufficient for the local rule itself because
//! replacing two adjacent compatible rotations with one rotation cannot expose
//! a new compatible rotation to the left except through the newly created
//! operation. The implementation therefore performs a single linear sweep.
//!
//! The optimizer pipeline may repeat this pass when other passes create new
//! adjacent rotations.
//!
//! The pass is therefore safe to classify as fixed-point compatible.
//!
//! # Measurement/barrier/reset boundaries
//!
//! Rotation fusion never crosses:
//!
//! - measurements;
//! - barriers;
//! - resets;
//! - classical-control boundaries;
//! - unrelated operations.
//!
//! Only physically adjacent operations are combined.
//!
//! # Operand ordering
//!
//! For controlled rotations the ordered operand list is significant.
//!
//! ```text
//! CRZ(control, target, a)
//! CRZ(control, target, b)
//! ```
//!
//! may be combined.
//!
//! ```text
//! CRZ(control, target, a)
//! CRZ(target, control, b)
//! ```
//!
//! may NOT be combined merely because the same two qubits appear.
//!
//! This preserves the canonical IR's operand semantics.
//!
//! # Global phase
//!
//! `RX`, `RY`, `RZ`, `Phase`, `U1`, `CRX`, `CRY`, and `CRZ` are combined only
//! with the same canonical gate kind. No cross-family identity is introduced
//! here, because some alternative decompositions differ by global phase or
//! require gate-specific semantic reasoning.
//!
//! Cross-family identities belong to algebraic/rewrite passes.
//!
//! # Integration contract
//!
//! ## `quantum::ir::gate`
//!
//! Uses:
//!
//! - `Gate`;
//! - `GateKind`;
//! - canonical gate validation;
//! - canonical parameter arity.
//!
//! No optimizer-local gate representation is introduced.
//!
//! ## `quantum::ir::parameter`
//!
//! Uses:
//!
//! - `Parameter`;
//! - `ParameterExpression`.
//!
//! Symbolic expressions remain explicit and deterministic.
//!
//! ## `quantum::ir::circuit`
//!
//! Uses:
//!
//! - `QuantumCircuit::operations()`;
//! - `QuantumCircuit::replace()`;
//! - `QuantumCircuit::remove()`;
//! - `QuantumCircuit::validate()`.
//!
//! All canonical mutation therefore remains validated by the IR.
//!
//! ## `optimization::pass`
//!
//! Implements `OptimizationPass`.
//!
//! Metadata declares:
//!
//! - local rewrite;
//! - linear complexity;
//! - deterministic execution;
//! - semantic preservation;
//! - parameter transformation;
//! - operation fusion;
//! - operation replacement/removal;
//! - fixed-point safety.
//!
//! ## `optimization::context`
//!
//! The pass receives an invocation-scoped `OptimizationContext`.
//!
//! Resource accounting and pipeline lifecycle remain owned by the context and
//! pipeline. The pass does not create process-global state.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline may run this pass:
//!
//! - after canonical normalization;
//! - after parameter simplification;
//! - before/after cancellation;
//! - repeatedly when another pass creates new adjacent rotations.
//!
//! ## `optimization::analysis`
//!
//! This implementation intentionally does not require a dependency analysis
//! because it only combines physically adjacent operations.
//!
//! A future dependency-aware rotation pass may use `analysis::dependency` and
//! `analysis::commutation`, but that is a separate semantic extension.
//!
//! ## `optimization::cost`
//!
//! The pass improves the operation count whenever two compatible operations
//! become one and can remove an operation entirely when the resulting angle is
//! exactly zero within the canonical constant normalization policy.
//!
//! ## `optimization::statistics`
//!
//! The generic `PassOutcome` reports:
//!
//! - operations before;
//! - operations after;
//! - replacements;
//! - removals;
//! - rewrite count.
//!
//! Detailed aggregate statistics remain owned by the optimizer context/result
//! subsystem.
//!
//! ## `optimization::verification`
//!
//! The pass declares semantic preservation. The pipeline/verification layer
//! remains responsible for optional whole-circuit semantic verification.
//!
//! ## `optimization::targets`
//!
//! This pass is target-independent. Native-gate target decisions belong to
//! synthesis/decomposition and target-aware passes.
//!
//! # No future edits required for new rotation values
//!
//! New parameter values do not require this file to change.
//!
//! New rotation gate families should normally be introduced by extending the
//! canonical `GateKind` and then deliberately adding the new family here.
//! This is intentional: silently treating an unknown gate as a rotation would
//! be unsafe.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required by this optimization.
//!
//! # Determinism
//!
//! Given the same canonical circuit, this pass produces the same result.
//!
//! It does not use:
//!
//! - system time;
//! - process-global randomness;
//! - hash-map iteration order;
//! - environment variables;
//! - backend state.
//!
//! # Example
//!
//! ```text
//! RX(theta_1)
//! RX(theta_2)
//! ```
//!
//! becomes:
//!
//! ```text
//! RX(theta_1 + theta_2)
//! ```
//!
//! without requiring either parameter to be numerically bound.

#![forbid(unsafe_code)]

use std::f64::consts::PI;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
};
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationResult,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassChange,
    PassComplexity,
    PassDeterminism,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable public identifier for this pass.
///
/// This identifier is intentionally independent from the Rust type name so
/// provenance and serialized optimizer configurations remain stable.
pub const PASS_ID: &str = "local.rotation_fusion";

/// Human-readable pass name.
pub const PASS_NAME: &str = "Rotation Fusion";

/// Maximum meaningful angle period for ordinary single-parameter rotations.
///
/// This is 2π because:
///
///     R(axis, theta + 2π) = R(axis, theta)
///
/// for the canonical rotation families handled here.
///
/// The pass does not apply this identity to arbitrary gate families.
const TWO_PI: f64 = 2.0 * PI;

/// Numerical zero threshold used only after finite constant arithmetic.
///
/// This is deliberately small and is NOT used to compare symbolic
/// expressions.
const CONSTANT_ZERO_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Statistics
// =============================================================================

/// Detailed statistics for one rotation-fusion invocation.
///
/// `PassOutcome` remains the stable optimizer-level result. This structure is
/// intentionally pass-local so that future global statistics changes do not
/// require changing the transformation algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RotationFusionStatistics {
    /// Number of compatible rotation pairs examined.
    pub candidate_pairs: u64,

    /// Number of successful parameter fusions.
    pub fused_rotations: u64,

    /// Number of rotations removed because the resulting angle was zero.
    pub zero_rotations_removed: u64,

    /// Number of constant-parameter fusions.
    pub constant_fusions: u64,

    /// Number of symbolic/expression fusions.
    pub symbolic_fusions: u64,

    /// Number of operations replaced by a fused rotation.
    pub operations_replaced: u64,

    /// Number of operations removed completely.
    pub operations_removed: u64,
}

impl RotationFusionStatistics {
    /// Returns true if this invocation changed the circuit.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.fused_rotations != 0
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production rotation-fusion optimization pass.
///
/// The pass is stateless. All invocation-specific state is local to `run`.
#[derive(Debug, Clone)]
pub struct RotationFusion {
    metadata: PassMetadata,
}

impl RotationFusion {
    /// Creates a production rotation-fusion pass.
    ///
    /// Metadata is constructed once and stored in the pass, so the pipeline
    /// does not need to reconstruct it on every invocation.
    pub fn new() -> Result<Self, OptimizationError> {
        let metadata = build_metadata()?;

        Ok(Self { metadata })
    }

    /// Returns the pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Executes the transformation without requiring the pass trait.
    ///
    /// This helper is useful for direct unit/integration testing.
    ///
    /// It intentionally uses the same canonical circuit and context contract
    /// as `OptimizationPass::run`.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        self.run_impl(circuit, context)
    }

    /// Performs the actual single-pass transformation.
    fn run_impl(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        circuit
            .validate()
            .map_err(|error| {
                optimization_error(
                    PASS_ID,
                    "input circuit validation failed",
                    error.to_string(),
                )
            })?;

        let operations_before = checked_u64(circuit.len())?;

        if circuit.len() < 2 {
            return Ok(
                PassOutcome::unchanged(
                    operations_before,
                    operations_before,
                )
                .with_iterations(1)
                .with_message(
                    "rotation fusion requires at least two operations",
                ),
            );
        }

        let mut statistics =
            RotationFusionStatistics::default();

        // ---------------------------------------------------------------------
        // Important implementation detail
        // ---------------------------------------------------------------------
        //
        // We use a forward index over the canonical circuit.
        //
        // When a compatible pair is found:
        //
        //     i       i + 1
        //     └── A ──┴── B ──┘
        //
        // it becomes:
        //
        //     i
        //     └── A+B ──┘
        //
        // Therefore the next index remains `i + 1`.
        //
        // This avoids rescanning the whole circuit and keeps the pass O(n).
        //
        // A successful replacement is validated atomically by
        // `QuantumCircuit::replace`.
        // ---------------------------------------------------------------------

        let mut index = 0usize;

        while index + 1 < circuit.len() {
            // Context resource/deadline checks are intentionally performed
            // periodically rather than on every operation. The interval keeps
            // overhead low on very large circuits while still cooperating with
            // optimizer-level cancellation/deadline policies.
            if index & 0x3ff == 0 {
                context
                    .check_limits()
                    .map_err(|error| {
                        optimization_error(
                            PASS_ID,
                            "optimizer resource check failed",
                            error.to_string(),
                        )
                    })?;
            }

            let first = circuit
                .get(index)
                .ok_or_else(|| {
                    optimization_error(
                        PASS_ID,
                        "operation disappeared during rotation scan",
                        format!(
                            "operation index {index} is no longer valid"
                        ),
                    )
                })?;

            let second = circuit
                .get(index + 1)
                .ok_or_else(|| {
                    optimization_error(
                        PASS_ID,
                        "operation disappeared during rotation scan",
                        format!(
                            "operation index {} is no longer valid",
                            index + 1
                        ),
                    )
                })?;

            let Some(combined) =
                combine_adjacent_rotations(first, second)?
            else {
                index += 1;
                continue;
            };

            statistics.candidate_pairs =
                checked_increment(
                    statistics.candidate_pairs,
                    "rotation candidate pairs",
                )?;

            match combined {
                CombinedRotation::Replace(gate) => {
                    circuit
                        .replace(index, gate)
                        .map_err(|error| {
                            optimization_error(
                                PASS_ID,
                                "failed to replace fused rotation",
                                error.to_string(),
                            )
                        })?;

                    circuit
                        .remove(index + 1)
                        .map_err(|error| {
                            optimization_error(
                                PASS_ID,
                                "failed to remove fused rotation",
                                error.to_string(),
                            )
                        })?;

                    statistics.fused_rotations =
                        checked_increment(
                            statistics.fused_rotations,
                            "fused rotations",
                        )?;

                    statistics.operations_replaced =
                        checked_increment(
                            statistics.operations_replaced,
                            "replacement operations",
                        )?;

                    index = index.saturating_sub(1);
                }

                CombinedRotation::Remove => {
                    // Remove both operations. The resulting operation is
                    // exactly the identity.
                    circuit
                        .remove(index + 1)
                        .map_err(|error| {
                            optimization_error(
                                PASS_ID,
                                "failed to remove second zero rotation",
                                error.to_string(),
                            )
                        })?;

                    circuit
                        .remove(index)
                        .map_err(|error| {
                            optimization_error(
                                PASS_ID,
                                "failed to remove first zero rotation",
                                error.to_string(),
                            )
                        })?;

                    statistics.fused_rotations =
                        checked_increment(
                            statistics.fused_rotations,
                            "fused rotations",
                        )?;

                    statistics.zero_rotations_removed =
                        checked_increment(
                            statistics.zero_rotations_removed,
                            "zero rotations removed",
                        )?;

                    statistics.operations_removed =
                        statistics
                            .operations_removed
                            .checked_add(2)
                            .ok_or_else(|| {
                                optimization_error(
                                    PASS_ID,
                                    "rotation removal counter overflow",
                                    "u64 counter overflow".to_string(),
                                )
                            })?;

                    index = index.saturating_sub(1);
                }
            }
        }

        // The canonical circuit mutation APIs already validate each mutation,
        // but whole-circuit validation remains mandatory at a pass boundary.
        circuit
            .validate()
            .map_err(|error| {
                optimization_error(
                    PASS_ID,
                    "optimized circuit validation failed",
                    error.to_string(),
                )
            })?;

        let operations_after =
            checked_u64(circuit.len())?;

        let change = if statistics.changed() {
            PassChange::Changed
        } else {
            PassChange::Unchanged
        };

        let message = if statistics.changed() {
            format!(
                "fused {} rotation pair(s), removed {} operation(s)",
                statistics.fused_rotations,
                statistics.operations_removed,
            )
        } else {
            "no adjacent compatible rotations found".to_string()
        };

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_change(change)
            .with_operations_removed(
                statistics.operations_removed,
            )
            .with_operations_replaced(
                statistics.operations_replaced,
            )
            .with_rewrites(
                statistics.fused_rotations,
            )
            .with_iterations(1)
            .with_message(message),
        )
    }
}

impl OptimizationPass for RotationFusion {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        self.run_impl(circuit, context)
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

// =============================================================================
// Metadata
// =============================================================================

fn build_metadata() -> Result<PassMetadata, OptimizationError> {
    let metadata = PassMetadata::new(
        PASS_ID,
        PASS_NAME,
        "Combines adjacent compatible parameterized rotations using exact "
            .to_string()
            + "canonical parameter arithmetic.",
        PassKind::LocalRewrite,
        PassScope::LocalWindow,
        PassComplexity::Linear,
        PassDeterminism::Deterministic,
    )
    .with_capabilities([
        PassCapability::ReplacesOperations,
        PassCapability::RemovesOperations,
        PassCapability::FusesOperations,
        PassCapability::ChangesParameters,
        PassCapability::ChangesGateCount,
    ])
    .with_semantic_preservation(true)
    .supports_empty_circuit(true)
    .supports_single_operation(true)
    .supports_large_circuits(true)
    .requires_target(false)
    .requires_verification(false)
    .fixed_point_safe(true);

    metadata.validate().map_err(|error| {
        optimization_error(
            PASS_ID,
            "invalid rotation-fusion pass metadata",
            error.to_string(),
        )
    })?;

    Ok(metadata)
}

// =============================================================================
// Combination model
// =============================================================================

enum CombinedRotation {
    /// Replace the first operation with the fused operation and remove the
    /// second operation.
    Replace(Gate),

    /// Remove both operations because their combined angle is exactly zero
    /// under the finite-constant normalization policy.
    Remove,
}

/// Combines two adjacent gates when the operation is mathematically valid.
///
/// No circuit mutation occurs here.
fn combine_adjacent_rotations(
    first: &Gate,
    second: &Gate,
) -> Result<Option<CombinedRotation>, OptimizationError> {
    if first.kind() != second.kind() {
        return Ok(None);
    }

    if !is_supported_rotation_kind(first.kind()) {
        return Ok(None);
    }

    if first.qubits() != second.qubits() {
        return Ok(None);
    }

    if first.classical_target() != second.classical_target() {
        return Ok(None);
    }

    if first.measurement().is_some()
        || second.measurement().is_some()
    {
        return Ok(None);
    }

    if first.parameters().len() != 1
        || second.parameters().len() != 1
    {
        return Err(optimization_error(
            PASS_ID,
            "rotation gate has invalid parameter arity",
            format!(
                "gate kind {:?} requires exactly one parameter for "
                    + "rotation fusion",
                first.kind(),
            ),
        ));
    }

    let left = &first.parameters()[0];
    let right = &second.parameters()[0];

    let combined =
        combine_parameters(left, right)?;

    match combined {
        CombinedParameter::Zero => {
            Ok(Some(CombinedRotation::Remove))
        }

        CombinedParameter::Parameter(parameter) => {
            let gate = Gate::new(
                first.kind(),
                first.qubits().to_vec(),
                vec![parameter],
                None,
                None,
            )
            .map_err(|error| {
                optimization_error(
                    PASS_ID,
                    "failed to construct fused rotation",
                    error.to_string(),
                )
            })?;

            Ok(Some(CombinedRotation::Replace(gate)))
        }
    }
}

// =============================================================================
// Parameter composition
// =============================================================================

enum CombinedParameter {
    Zero,
    Parameter(Parameter),
}

/// Combines two exact canonical parameters.
///
/// Constant values are normalized numerically. Symbolic values are represented
/// by an explicit canonical expression.
///
/// This function deliberately does not attempt aggressive symbolic algebra.
/// That responsibility belongs to `parameter::simplification`.
fn combine_parameters(
    left: &Parameter,
    right: &Parameter,
) -> Result<CombinedParameter, OptimizationError> {
    match (left, right) {
        (
            Parameter::Constant(left_value),
            Parameter::Constant(right_value),
        ) => {
            let sum =
                left_value
                    .checked_add(*right_value)
                    .ok_or_else(|| {
                        optimization_error(
                            PASS_ID,
                            "rotation parameter arithmetic overflow",
                            "finite floating-point addition produced "
                                .to_string()
                                + "a non-finite value",
                        )
                    })?;

            if !sum.is_finite() {
                return Err(optimization_error(
                    PASS_ID,
                    "rotation parameter became non-finite",
                    format!("sum={sum:?}"),
                ));
            }

            let normalized =
                normalize_constant_angle(sum);

            if is_effectively_zero(normalized) {
                return Ok(CombinedParameter::Zero);
            }

            let parameter =
                Parameter::constant(normalized)
                    .map_err(|error| {
                        optimization_error(
                            PASS_ID,
                            "failed to construct normalized rotation parameter",
                            error.to_string(),
                        )
                    })?;

            Ok(CombinedParameter::Parameter(parameter))
        }

        _ => {
            // Preserve symbolic information exactly.
            //
            // No evaluation is attempted. If `left` or `right` contains
            // symbols, the expression remains symbolic until a later explicit
            // parameter-binding stage.
            let expression =
                ParameterExpression::Add(
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                );

            let parameter =
                Parameter::expression(expression)
                    .map_err(|error| {
                        optimization_error(
                            PASS_ID,
                            "failed to construct symbolic rotation expression",
                            error.to_string(),
                        )
                    })?;

            Ok(CombinedParameter::Parameter(parameter))
        }
    }
}

// =============================================================================
// Rotation classification
// =============================================================================

/// Returns true for exactly the one-parameter rotation families currently
/// defined by the canonical Zamani IR.
///
/// The explicit match is intentional. Unknown future gate kinds must not be
/// silently optimized.
const fn is_supported_rotation_kind(
    kind: GateKind,
) -> bool {
    matches!(
        kind,
        GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
    )
}

// =============================================================================
// Constant normalization
// =============================================================================

/// Normalizes a finite rotation angle into approximately `[-π, π]`.
///
/// The operation is deterministic and does not allocate.
///
/// This is applied only to concrete constants.
fn normalize_constant_angle(
    angle: f64,
) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let mut normalized =
        angle % TWO_PI;

    if normalized > PI {
        normalized -= TWO_PI;
    } else if normalized < -PI {
        normalized += TWO_PI;
    }

    // Collapse numerical noise around zero. This is intentionally much smaller
    // than the tolerance normally used by approximate numerical algorithms.
    if normalized.abs()
        <= CONSTANT_ZERO_TOLERANCE
    {
        0.0
    } else {
        normalized
    }
}

/// Returns whether a finite constant is treated as zero by this pass.
const fn is_effectively_zero(
    value: f64,
) -> bool {
    value.abs() <= CONSTANT_ZERO_TOLERANCE
}

// =============================================================================
// Error helpers
// =============================================================================

fn optimization_error(
    pass_id: &str,
    operation: &str,
    detail: String,
) -> OptimizationError {
    OptimizationError::pass_failure(
        pass_id,
        format!("{operation}: {detail}"),
    )
}

fn checked_u64(
    value: usize,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        optimization_error(
            PASS_ID,
            "counter conversion overflow",
            format!("cannot represent {value} as u64"),
        )
    })
}

fn checked_increment(
    value: u64,
    resource: &'static str,
) -> Result<u64, OptimizationError> {
    value.checked_add(1).ok_or_else(|| {
        optimization_error(
            PASS_ID,
            "statistics counter overflow",
            resource.to_string(),
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubits::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn circuit(
        operations: Vec<Gate>,
    ) -> QuantumCircuit {
        QuantumCircuit::from_operations(
            4,
            0,
            operations,
        )
        .expect("test circuit must be valid")
    }

    fn context()
        -> OptimizationContext
    {
        OptimizationContext::default_for_testing()
            .expect("test optimization context must be valid")
    }

    #[test]
    fn fuses_rx_constants() {
        let mut circuit = circuit(vec![
            Gate::rx(q(0), 0.25)
                .expect("rx must be valid"),
            Gate::rx(q(0), 0.5)
                .expect("rx must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        let outcome =
            pass.optimize(
                &mut circuit,
                &mut context(),
            )
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert!(outcome.changed());

        let gate = circuit
            .get(0)
            .expect("fused gate must exist");

        assert_eq!(gate.kind(), GateKind::RX);
        assert_eq!(
            gate.parameters()[0].as_constant(),
            Some(0.75),
        );
    }

    #[test]
    fn fuses_ry_constants() {
        let mut circuit = circuit(vec![
            Gate::ry(q(0), 0.25)
                .expect("ry must be valid"),
            Gate::ry(q(0), 0.75)
                .expect("ry must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit
                .get(0)
                .expect("gate")
                .parameters()[0]
                .as_constant(),
            Some(1.0),
        );
    }

    #[test]
    fn fuses_rz_constants_across_period() {
        let mut circuit = circuit(vec![
            Gate::rz(q(0), PI)
                .expect("rz must be valid"),
            Gate::rz(q(0), PI)
                .expect("rz must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_rotation() {
        let mut circuit = circuit(vec![
            Gate::rz(q(0), 1.25)
                .expect("rz must be valid"),
            Gate::rz(q(0), -1.25)
                .expect("rz must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn fuses_phase_gates() {
        let mut circuit = circuit(vec![
            Gate::phase(q(0), 0.2)
                .expect("phase must be valid"),
            Gate::phase(q(0), 0.3)
                .expect("phase must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit
                .get(0)
                .expect("gate")
                .parameters()[0]
                .as_constant(),
            Some(0.5),
        );
    }

    #[test]
    fn fuses_controlled_rotations_only_when_operand_order_matches() {
        let first =
            Gate::crz(q(0), q(1), 0.2)
                .expect("crz must be valid");

        let second =
            Gate::crz(q(0), q(1), 0.3)
                .expect("crz must be valid");

        let mut circuit =
            circuit(vec![first, second]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit
                .get(0)
                .expect("gate")
                .parameters()[0]
                .as_constant(),
            Some(0.5),
        );
    }

    #[test]
    fn does_not_fuse_controlled_rotations_with_reversed_operands() {
        let first =
            Gate::crz(q(0), q(1), 0.2)
                .expect("crz must be valid");

        let second =
            Gate::crz(q(1), q(0), 0.3)
                .expect("crz must be valid");

        let mut circuit =
            circuit(vec![first, second]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 2);
    }

    #[test]
    fn does_not_cross_non_rotation_operation() {
        let mut circuit = circuit(vec![
            Gate::rx(q(0), 0.2)
                .expect("rx must be valid"),
            Gate::x(q(0))
                .expect("x must be valid"),
            Gate::rx(q(0), 0.3)
                .expect("rx must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 3);
    }

    #[test]
    fn does_not_fuse_different_rotation_families() {
        let mut circuit = circuit(vec![
            Gate::rx(q(0), 0.2)
                .expect("rx must be valid"),
            Gate::ry(q(0), 0.3)
                .expect("ry must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 2);
    }

    #[test]
    fn preserves_symbolic_parameters() {
        let theta =
            Parameter::symbol("theta")
                .expect("symbol must be valid");

        let phi =
            Parameter::symbol("phi")
                .expect("symbol must be valid");

        let first =
            Gate::new(
                GateKind::RX,
                vec![q(0)],
                vec![theta.clone()],
                None,
                None,
            )
            .expect("symbolic rx must be valid");

        let second =
            Gate::new(
                GateKind::RX,
                vec![q(0)],
                vec![phi.clone()],
                None,
                None,
            )
            .expect("symbolic rx must be valid");

        let mut circuit =
            circuit(vec![first, second]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut circuit,
            &mut context(),
        )
        .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert!(
            matches!(
                circuit
                    .get(0)
                    .expect("fused gate")
                    .parameters()[0],
                Parameter::Expression(_)
            )
        );
    }

    #[test]
    fn preserves_single_operation_circuit() {
        let mut circuit = circuit(vec![
            Gate::rx(q(0), 0.2)
                .expect("rx must be valid"),
        ]);

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        let outcome =
            pass.optimize(
                &mut circuit,
                &mut context(),
            )
            .expect("optimization must succeed");

        assert!(!outcome.changed());
        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn preserves_empty_circuit() {
        let mut circuit =
            circuit(Vec::new());

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        let outcome =
            pass.optimize(
                &mut circuit,
                &mut context(),
            )
            .expect("optimization must succeed");

        assert!(!outcome.changed());
        assert!(circuit.is_empty());
    }

    #[test]
    fn normalization_is_periodic() {
        let normalized =
            normalize_constant_angle(
                TWO_PI + 0.25,
            );

        assert!(
            (normalized - 0.25).abs()
                <= CONSTANT_ZERO_TOLERANCE,
        );
    }

    #[test]
    fn symbolic_expression_is_not_eagerly_evaluated() {
        let theta =
            Parameter::symbol("theta")
                .expect("symbol must be valid");

        let phi =
            Parameter::symbol("phi")
                .expect("symbol must be valid");

        let result =
            combine_parameters(
                &theta,
                &phi,
            )
            .expect("symbolic composition must succeed");

        assert!(
            matches!(
                result,
                CombinedParameter::Parameter(
                    Parameter::Expression(_)
                )
            )
        );
    }

    #[test]
    fn pass_is_deterministic() {
        let mut first =
            circuit(vec![
                Gate::rx(q(0), 0.2)
                    .expect("rx must be valid"),
                Gate::rx(q(0), 0.3)
                    .expect("rx must be valid"),
            ]);

        let mut second =
            first.clone();

        let pass =
            RotationFusion::new()
                .expect("metadata must be valid");

        pass.optimize(
            &mut first,
            &mut context(),
        )
        .expect("first optimization must succeed");

        pass.optimize(
            &mut second,
            &mut context(),
        )
        .expect("second optimization must succeed");

        assert_eq!(first, second);
    }
}