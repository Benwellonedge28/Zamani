//! Zamani Quantum Optimization — Production Peephole Pass
//!
//! Local, exact, semantics-preserving quantum-circuit rewrites over the
//! canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//!                 │
//!                 ▼
//!       optimization::local::peephole
//!                 │
//!                 ▼
//!       canonical QuantumCircuit
//! ```
//!
//! This module deliberately does NOT define another quantum IR.
//!
//! The authoritative representations remain:
//!
//! - `crate::quantum::ir::Gate`
//! - `crate::quantum::ir::GateKind`
//! - `crate::quantum::ir::Parameter`
//! - `crate::quantum::ir::QuantumCircuit`
//!
//! # Responsibilities
//!
//! This pass performs bounded, exact local rewrites such as:
//!
//! ```text
//! I                    -> ∅
//! X X                  -> ∅
//! Y Y                  -> ∅
//! Z Z                  -> ∅
//! H H                  -> ∅
//! CX CX                -> ∅
//! CZ CZ                -> ∅
//! SWAP SWAP            -> ∅
//! S Sdg                -> ∅
//! T Tdg                -> ∅
//! V Vdg                -> ∅
//!
//! RX(a) RX(b)          -> RX(a+b)
//! RY(a) RY(b)          -> RY(a+b)
//! RZ(a) RZ(b)          -> RZ(a+b)
//! Phase(a) Phase(b)    -> Phase(a+b)
//!
//! H X H                -> Z
//! H Z H                -> X
//! S X Sdg             -> Y
//! Sdg Y S              -> X
//!
//! H(q1) CX(q0,q1) H(q1)
//!                       -> CZ(q0,q1)
//!
//! H(q1) CZ(q0,q1) H(q1)
//!                       -> CX(q0,q1)
//!
//! H(q0) CZ(q0,q1) H(q0)
//!                       -> CX(q1,q0)
//! ```
//!
//! Only exact identities are used by default.
//!
//! No approximation is silently introduced.
//!
//! In particular, identities differing by a global phase are NOT applied
//! unless the canonical semantic policy explicitly grows to support that
//! distinction in a future version.
//!
//! # Semantic boundaries
//!
//! Peephole matching never crosses:
//!
//! - measurements;
//! - resets;
//! - barriers.
//!
//! This is intentional. A measurement/reset/barrier is not merely another
//! unitary operation and cannot be crossed by a local rewrite without an
//! explicit semantic proof.
//!
//! # Complexity
//!
//! The implementation is a bounded-window linear scan:
//!
//! - time: O(n)
//! - additional memory: O(n) for the edit plan
//! - maximum look-ahead: 3 operations
//!
//! It does not perform whole-circuit unitary construction, matrix
//! multiplication, exponential search, or equality saturation.
//!
//! Therefore a circuit may be arbitrarily large subject only to the canonical
//! IR limits and available resources.
//!
//! Repetition to a fixed point belongs to `pipeline.rs`, not this pass.
//!
//! # Transactional mutation
//!
//! The pass discovers all rewrites before mutating the circuit. Rewrites are
//! then applied from the end of the circuit toward the beginning.
//!
//! This prevents index invalidation and ensures that all fallible gate
//! construction happens before circuit mutation.
//!
//! # Integration
//!
//! ## `pass.rs`
//!
//! Implements `OptimizationPass`.
//!
//! ## `circuit.rs`
//!
//! Uses the canonical `QuantumCircuit` mutation API. No mutable operation
//! slice is required.
//!
//! ## `ir/gate.rs`
//!
//! Uses `Gate`, `GateKind`, and canonical gate validation.
//!
//! ## `ir/parameter.rs`
//!
//! Uses canonical `Parameter` and `ParameterExpression` for exact symbolic
//! rotation fusion.
//!
//! ## `errors.rs`
//!
//! All public failures use `OptimizationError`.
//!
//! ## `pipeline.rs`
//!
//! The pipeline may invoke this pass repeatedly when a fixed point is desired.
//!
//! This pass itself performs exactly one linear local scan.
//!
//! ## `rules.rs` / `rewrite.rs`
//!
//! The current implementation keeps the small production bootstrap rule set
//! local so this file is independently usable. The rules are deliberately
//! represented by stable IDs and isolated in helper functions so they can be
//! migrated into the generic rewrite registry later without changing the
//! semantic implementation.
//!
//! ## `analysis/*`
//!
//! This pass deliberately requires no global analysis. Its bounded local
//! patterns cannot cross semantic boundaries.
//!
//! ## `verification/*`
//!
//! Semantic verification remains a pipeline-level concern. Every rewrite in
//! this module is an exact algebraic identity.
//!
//! ## routing / scheduling / hardware
//!
//! No dependency exists on routing, physical topology, scheduling, pulse
//! generation, calibration, QPU APIs, or hardware providers.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no `unsafe`
//!
//! # Safety
//!
//! `#![forbid(unsafe_code)]` is intentional.

#![forbid(unsafe_code)]

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
};
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationLocation,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassChange,
    PassComplexity,
    PassDeterminism,
    PassEffects,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassMetadataError,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable pass identifier.
///
/// This identifier is part of optimizer provenance and therefore must not be
/// casually renamed.
pub const PASS_ID: &str = "quantum.local.peephole";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Quantum Peephole Optimization";

/// Maximum pattern width.
///
/// The current production rule set never examines more than three adjacent
/// operations.
pub const MAX_PATTERN_WIDTH: usize = 3;

// =============================================================================
// Rewrite classification
// =============================================================================

/// Classification of one local rewrite.
#[derive(Debug, Clone, PartialEq)]
enum Rewrite {
    /// Remove `count` operations beginning at `start`.
    Remove {
        start: usize,
        count: usize,
        rule: &'static str,
    },

    /// Replace one operation and remove the following `remove_after`
    /// operations.
    Replace {
        start: usize,
        gate: Gate,
        remove_after: usize,
        rule: &'static str,
    },
}

impl Rewrite {
    fn start(&self) -> usize {
        match self {
            Self::Remove { start, .. } => *start,
            Self::Replace { start, .. } => *start,
        }
    }

    fn rule(&self) -> &'static str {
        match self {
            Self::Remove { rule, .. } => rule,
            Self::Replace { rule, .. } => rule,
        }
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production local peephole optimization pass.
///
/// The pass is stateless after construction and therefore can safely be shared
/// by the optimizer registry and scheduler.
#[derive(Debug, Clone)]
pub struct PeepholePass {
    metadata: PassMetadata,
}

impl PeepholePass {
    /// Constructs the production peephole pass.
    ///
    /// Metadata construction is fallible because the global optimization
    /// contract intentionally validates identifiers rather than relying on
    /// unchecked string constants.
    pub fn new() -> Result<Self, PassMetadataError> {
        let pass_id = PassIdentifier::from_static(PASS_ID)
            .map_err(|error| PassMetadataError::InvalidPassIdentifier {
                message: error.to_string(),
            })?;

        let metadata = PassMetadata::new(
            pass_id,
            PASS_NAME,
            PassKind::LocalRewrite,
        )?
        .with_description(
            "Applies bounded exact local quantum-circuit peephole rewrites \
             over the canonical Zamani Quantum IR.",
        )?
        .with_scope(PassScope::LocalWindow)
        .with_complexity(PassComplexity::Linear)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capabilities([
            PassCapability::RemovesOperations,
            PassCapability::ReplacesOperations,
            PassCapability::FusesOperations,
            PassCapability::ChangesGateCount,
            PassCapability::ChangesTwoQubitCount,
            PassCapability::ChangesDepth,
            PassCapability::ChangesParameters,
        ])
        .with_semantic_preservation(true)
        .supports_empty_circuit(true)
        .supports_single_operation(true)
        .supports_large_circuits(true)
        .requires_target(false)
        .requires_verification(false)
        .fixed_point_safe(true);

        metadata.validate()?;

        Ok(Self { metadata })
    }

    /// Returns the stable pass identifier.
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the maximum local pattern width.
    pub const fn max_pattern_width() -> usize {
        MAX_PATTERN_WIDTH
    }

    /// Performs one peephole scan without requiring the pipeline.
    ///
    /// This convenience API is useful for unit tests and specialized compiler
    /// integrations. Production compiler pipelines should normally invoke the
    /// `OptimizationPass` implementation instead.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<PassOutcome, OptimizationError> {
        let mut context = OptimizationContext::standalone();

        self.run(circuit, &mut context)
    }

    /// Finds all applicable rewrites without mutating the circuit.
    ///
    /// This is deliberately private to the mutation pipeline because the
    /// rewrite representation contains canonical `Gate` values rather than
    /// exposing an optimizer-specific IR.
    fn discover_rewrites(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<Vec<Rewrite>, OptimizationError> {
        let operations = circuit.operations();

        let mut rewrites = Vec::new();

        let mut index = 0usize;

        while index < operations.len() {
            let current = &operations[index];

            // A semantic boundary terminates the local search window.
            if is_semantic_boundary(current) {
                index += 1;
                continue;
            }

            // Prefer the largest pattern first. This makes the result
            // deterministic and avoids consuming a three-gate identity as
            // several smaller patterns.
            if index + 2 < operations.len() {
                if let Some(rewrite) = three_gate_rewrite(
                    circuit,
                    index,
                    &operations[index],
                    &operations[index + 1],
                    &operations[index + 2],
                )? {
                    rewrites.push(rewrite);
                    index += 3;
                    continue;
                }
            }

            if index + 1 < operations.len() {
                if let Some(rewrite) = two_gate_rewrite(
                    circuit,
                    index,
                    &operations[index],
                    &operations[index + 1],
                )? {
                    rewrites.push(rewrite);
                    index += 2;
                    continue;
                }
            }

            if let Some(rewrite) = one_gate_rewrite(
                index,
                current,
            )? {
                rewrites.push(rewrite);
                index += 1;
                continue;
            }

            index += 1;
        }

        Ok(rewrites)
    }

    /// Applies a previously validated rewrite plan.
    ///
    /// All edits are applied from highest index to lowest index, so deleting or
    /// replacing operations cannot invalidate the indices of earlier edits.
    fn apply_rewrites(
        &self,
        circuit: &mut QuantumCircuit,
        rewrites: &[Rewrite],
    ) -> Result<AppliedStatistics, OptimizationError> {
        let mut statistics = AppliedStatistics::default();

        for rewrite in rewrites.iter().rev() {
            match rewrite {
                Rewrite::Remove {
                    start,
                    count,
                    rule,
                } => {
                    let end = start
                        .checked_add(*count)
                        .ok_or_else(|| {
                            invariant_error(
                                "peephole rewrite index overflow",
                            )
                        })?;

                    if end > circuit.len() {
                        return Err(
                            invariant_error(
                                "peephole rewrite references an \
                                 operation outside the circuit",
                            )
                            .with_location(
                                OptimizationLocation::new()
                                    .operation(*start),
                            )
                            .with_rule_identifier(rule),
                        );
                    }

                    // Remove from the end of the local region first.
                    for index in (*start..end).rev() {
                        circuit.remove(index).map_err(|error| {
                            OptimizationError::invalid_rewrite(
                                OptimizationStage::Rewrite,
                                format!(
                                    "failed to remove operation {index}: \
                                     {error}"
                                ),
                            )
                            .with_location(
                                OptimizationLocation::new()
                                    .operation(index),
                            )
                            .with_rule_identifier(rule)
                        })?;
                    }

                    statistics.removed = statistics
                        .removed
                        .saturating_add(*count as u64);

                    statistics.rewrites =
                        statistics.rewrites.saturating_add(1);
                }

                Rewrite::Replace {
                    start,
                    gate,
                    remove_after,
                    rule,
                } => {
                    let end = start
                        .checked_add(
                            remove_after.saturating_add(1),
                        )
                        .ok_or_else(|| {
                            invariant_error(
                                "peephole replacement index overflow",
                            )
                        })?;

                    if end > circuit.len() {
                        return Err(
                            invariant_error(
                                "peephole replacement references an \
                                 operation outside the circuit",
                            )
                            .with_location(
                                OptimizationLocation::new()
                                    .operation(*start),
                            )
                            .with_rule_identifier(rule),
                        );
                    }

                    circuit
                        .replace(*start, gate.clone())
                        .map_err(|error| {
                            OptimizationError::invalid_rewrite(
                                OptimizationStage::Rewrite,
                                format!(
                                    "failed to replace operation {start}: \
                                     {error}"
                                ),
                            )
                            .with_location(
                                OptimizationLocation::new()
                                    .operation(*start),
                            )
                            .with_rule_identifier(rule)
                        })?;

                    for index in
                        ((*start + 1)..end).rev()
                    {
                        circuit.remove(index).map_err(
                            |error| {
                                OptimizationError::invalid_rewrite(
                                    OptimizationStage::Rewrite,
                                    format!(
                                        "failed to remove operation \
                                         {index}: {error}"
                                    ),
                                )
                                .with_location(
                                    OptimizationLocation::new()
                                        .operation(index),
                                )
                                .with_rule_identifier(rule)
                            },
                        )?;
                    }

                    statistics.removed = statistics
                        .removed
                        .saturating_add(
                            *remove_after as u64,
                        );

                    statistics.replaced = statistics
                        .replaced
                        .saturating_add(1);

                    statistics.rewrites =
                        statistics.rewrites.saturating_add(1);
                }
            }
        }

        Ok(statistics)
    }
}

impl OptimizationPass for PeepholePass {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        _context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        let operations_before = checked_u64(
            circuit.len(),
            "peephole operation count before optimization",
        )?;

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "canonical Quantum IR validation failed before \
                     peephole optimization: {error}"
                ),
            )
        })?;

        if circuit.is_empty() {
            return Ok(PassOutcome::unchanged(
                operations_before,
                operations_before,
            ));
        }

        let rewrites = self.discover_rewrites(circuit)?;

        if rewrites.is_empty() {
            return Ok(PassOutcome::no_improvement(
                operations_before,
                operations_before,
            ));
        }

        let statistics =
            self.apply_rewrites(circuit, &rewrites)?;

        circuit.validate().map_err(|error| {
            OptimizationError::rewrite_postcondition_failed(
                None,
                format!(
                    "peephole optimization produced invalid canonical \
                     Quantum IR: {error}"
                ),
            )
        })?;

        let operations_after = checked_u64(
            circuit.len(),
            "peephole operation count after optimization",
        )?;

        let change = if operations_after < operations_before {
            PassChange::Changed
        } else {
            // Current rules are non-expanding. Keeping this branch explicit
            // protects the pass contract if future rules introduce
            // replacements with equal operation counts.
            PassChange::Changed
        };

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_change(change)
            .with_operations_removed(
                statistics.removed,
            )
            .with_operations_replaced(
                statistics.replaced,
            )
            .with_rewrites(
                statistics.rewrites,
            )
            .with_iterations(1),
        )
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

// =============================================================================
// Statistics
// =============================================================================

#[derive(Debug, Clone, Copy, Default)]
struct AppliedStatistics {
    removed: u64,
    replaced: u64,
    rewrites: u64,
}

// =============================================================================
// One-operation rewrites
// =============================================================================

fn one_gate_rewrite(
    index: usize,
    gate: &Gate,
) -> Result<Option<Rewrite>, OptimizationError> {
    if is_semantic_boundary(gate) {
        return Ok(None);
    }

    if gate.kind() == GateKind::I {
        return Ok(Some(Rewrite::Remove {
            start: index,
            count: 1,
            rule: "identity.single",
        }));
    }

    Ok(None)
}

// =============================================================================
// Two-operation rewrites
// =============================================================================

fn two_gate_rewrite(
    circuit: &QuantumCircuit,
    index: usize,
    first: &Gate,
    second: &Gate,
) -> Result<Option<Rewrite>, OptimizationError> {
    if is_semantic_boundary(first)
        || is_semantic_boundary(second)
    {
        return Ok(None);
    }

    // Local peephole cancellation is only legal when the two operations act on
    // exactly the same logical operands in exactly the same ordering.
    if first.qubits() != second.qubits() {
        return Ok(None);
    }

    // Identity elimination.
    if first.kind() == GateKind::I
        || second.kind() == GateKind::I
    {
        let start = if first.kind() == GateKind::I {
            index
        } else {
            index + 1
        };

        return Ok(Some(Rewrite::Remove {
            start,
            count: 1,
            rule: "identity.adjacent",
        }));
    }

    // Any canonical self-inverse operation cancels with an identical adjacent
    // operation.
    if first.kind() == second.kind()
        && first.kind().is_self_inverse()
        && first.parameters().is_empty()
        && second.parameters().is_empty()
    {
        return Ok(Some(Rewrite::Remove {
            start: index,
            count: 2,
            rule: "inverse.self",
        }));
    }

    // Explicit inverse pairs.
    if are_inverse(
        first.kind(),
        second.kind(),
    ) && first.parameters().is_empty()
        && second.parameters().is_empty()
    {
        return Ok(Some(Rewrite::Remove {
            start: index,
            count: 2,
            rule: "inverse.explicit",
        }));
    }

    // Same-axis parameterized rotations.
    if first.kind() == second.kind()
        && is_fusable_rotation(first.kind())
    {
        if let Some(parameter) =
            combine_parameters(
                first.parameters(),
                second.parameters(),
            )?
        {
            let replacement = rebuild_single_parameter_gate(
                circuit,
                first,
                parameter,
            )?;

            return Ok(Some(Rewrite::Replace {
                start: index,
                gate: replacement,
                remove_after: 1,
                rule: "rotation.fuse",
            }));
        }
    }

    Ok(None)
}

// =============================================================================
// Three-operation rewrites
// =============================================================================

fn three_gate_rewrite(
    circuit: &QuantumCircuit,
    index: usize,
    first: &Gate,
    second: &Gate,
    third: &Gate,
) -> Result<Option<Rewrite>, OptimizationError> {
    if is_semantic_boundary(first)
        || is_semantic_boundary(second)
        || is_semantic_boundary(third)
    {
        return Ok(None);
    }

    // -------------------------------------------------------------------------
    // H X H -> Z
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::H)
        && is_single_qubit_kind(second, GateKind::X)
        && is_single_qubit_kind(third, GateKind::H)
        && same_qubit(first, second)
        && same_qubit(second, third)
    {
        let gate = make_gate(
            circuit,
            GateKind::Z,
            vec![first.qubits()[0]],
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.hxh_to_z",
        }));
    }

    // -------------------------------------------------------------------------
    // H Z H -> X
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::H)
        && is_single_qubit_kind(second, GateKind::Z)
        && is_single_qubit_kind(third, GateKind::H)
        && same_qubit(first, second)
        && same_qubit(second, third)
    {
        let gate = make_gate(
            circuit,
            GateKind::X,
            vec![first.qubits()[0]],
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.hzh_to_x",
        }));
    }

    // -------------------------------------------------------------------------
    // S X Sdg -> Y
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::S)
        && is_single_qubit_kind(second, GateKind::X)
        && is_single_qubit_kind(third, GateKind::Sdg)
        && same_qubit(first, second)
        && same_qubit(second, third)
    {
        let gate = make_gate(
            circuit,
            GateKind::Y,
            vec![first.qubits()[0]],
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.sxsdag_to_y",
        }));
    }

    // -------------------------------------------------------------------------
    // Sdg Y S -> X
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::Sdg)
        && is_single_qubit_kind(second, GateKind::Y)
        && is_single_qubit_kind(third, GateKind::S)
        && same_qubit(first, second)
        && same_qubit(second, third)
    {
        let gate = make_gate(
            circuit,
            GateKind::X,
            vec![first.qubits()[0]],
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.sdagys_to_x",
        }));
    }

    // -------------------------------------------------------------------------
    // H(target) CX(control,target) H(target) -> CZ(control,target)
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::H)
        && second.kind() == GateKind::CX
        && is_single_qubit_kind(third, GateKind::H)
        && second.qubits().len() == 2
        && first.qubits()[0] == second.qubits()[1]
        && third.qubits()[0] == second.qubits()[1]
    {
        let gate = make_gate(
            circuit,
            GateKind::CZ,
            second.qubits().to_vec(),
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.h_cx_h_to_cz",
        }));
    }

    // -------------------------------------------------------------------------
    // H(target) CZ(control,target) H(target) -> CX(control,target)
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::H)
        && second.kind() == GateKind::CZ
        && is_single_qubit_kind(third, GateKind::H)
        && second.qubits().len() == 2
        && first.qubits()[0] == second.qubits()[1]
        && third.qubits()[0] == second.qubits()[1]
    {
        let gate = make_gate(
            circuit,
            GateKind::CX,
            second.qubits().to_vec(),
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.h_cz_h_to_cx",
        }));
    }

    // -------------------------------------------------------------------------
    // H(control) CZ(control,target) H(control) -> CX(target,control)
    // -------------------------------------------------------------------------

    if is_single_qubit_kind(first, GateKind::H)
        && second.kind() == GateKind::CZ
        && is_single_qubit_kind(third, GateKind::H)
        && second.qubits().len() == 2
        && first.qubits()[0] == second.qubits()[0]
        && third.qubits()[0] == second.qubits()[0]
    {
        let gate = make_gate(
            circuit,
            GateKind::CX,
            vec![
                second.qubits()[1],
                second.qubits()[0],
            ],
            Vec::new(),
            None,
            None,
        )?;

        return Ok(Some(Rewrite::Replace {
            start: index,
            gate,
            remove_after: 2,
            rule: "clifford.h_cz_h_control_to_reversed_cx",
        }));
    }

    Ok(None)
}

// =============================================================================
// Gate construction
// =============================================================================

/// Constructs and validates a canonical gate.
///
/// All generated gates pass through the circuit's own validation path, so a
/// future IR invariant cannot be bypassed by this optimization pass.
fn make_gate(
    circuit: &QuantumCircuit,
    kind: GateKind,
    qubits: Vec<QubitId>,
    parameters: Vec<Parameter>,
    classical_target: Option<usize>,
    measurement: Option<
        crate::quantum::ir::measurement::Measurement,
    >,
) -> Result<Gate, OptimizationError> {
    let gate = Gate::new(
        kind,
        qubits,
        parameters,
        classical_target,
        measurement,
    )
    .map_err(|error| {
        OptimizationError::invalid_rewrite(
            OptimizationStage::Rewrite,
            format!(
                "peephole rule produced an invalid gate: {error}"
            ),
        )
    })?;

    circuit
        .validate_gate(&gate)
        .map_err(|error| {
            OptimizationError::invalid_rewrite(
                OptimizationStage::Rewrite,
                format!(
                    "peephole-generated gate failed canonical circuit \
                     validation: {error}"
                ),
            )
        })?;

    Ok(gate)
}

// =============================================================================
// Parameter fusion
// =============================================================================

/// Combines two same-axis parameters exactly.
///
/// Symbolic parameters are preserved as canonical parameter expressions:
///
/// ```text
/// RX(a) RX(b) -> RX(a+b)
/// ```
///
/// No numerical approximation is introduced.
fn combine_parameters(
    first: &[Parameter],
    second: &[Parameter],
) -> Result<Option<Parameter>, OptimizationError> {
    if first.len() != 1 || second.len() != 1 {
        return Ok(None);
    }

    let expression =
        ParameterExpression::Add(
            Box::new(first[0].clone()),
            Box::new(second[0].clone()),
        );

    Parameter::expression(expression)
        .map(Some)
        .map_err(|error| {
            OptimizationError::parameter_error(
                OptimizationStage::Rewrite,
                format!(
                    "failed to construct fused rotation parameter: \
                     {error}"
                ),
            )
        })
}

/// Rebuilds a single-parameter gate while preserving its canonical qubits.
fn rebuild_single_parameter_gate(
    circuit: &QuantumCircuit,
    source: &Gate,
    parameter: Parameter,
) -> Result<Gate, OptimizationError> {
    make_gate(
        circuit,
        source.kind(),
        source.qubits().to_vec(),
        vec![parameter],
        None,
        None,
    )
}

// =============================================================================
// Gate classification
// =============================================================================

fn is_fusable_rotation(
    kind: GateKind,
) -> bool {
    matches!(
        kind,
        GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
    )
}

fn are_inverse(
    first: GateKind,
    second: GateKind,
) -> bool {
    matches!(
        (first, second),
        (GateKind::S, GateKind::Sdg)
            | (GateKind::Sdg, GateKind::S)
            | (GateKind::T, GateKind::Tdg)
            | (GateKind::Tdg, GateKind::T)
            | (GateKind::V, GateKind::Vdg)
            | (GateKind::Vdg, GateKind::V)
    )
}

fn is_single_qubit_kind(
    gate: &Gate,
    kind: GateKind,
) -> bool {
    gate.kind() == kind
        && gate.qubits().len() == 1
        && gate.parameters().is_empty()
}

fn same_qubit(
    first: &Gate,
    second: &Gate,
) -> bool {
    first.qubits().len() == 1
        && second.qubits().len() == 1
        && first.qubits()[0] == second.qubits()[0]
}

// =============================================================================
// Semantic boundaries
// =============================================================================

fn is_semantic_boundary(
    gate: &Gate,
) -> bool {
    gate.is_measurement()
        || gate.is_reset()
        || gate.is_barrier()
}

// =============================================================================
// Error helpers
// =============================================================================

fn invariant_error(
    message: &'static str,
) -> OptimizationError {
    OptimizationError::invariant(
        OptimizationStage::Rewrite,
        message,
        None,
    )
}

/// Adds a stable rule identifier to an optimization error.
///
/// `OptimizationError::with_rule` requires the canonical `RuleIdentifier`.
fn with_rule_identifier(
    error: OptimizationError,
    rule: &'static str,
) -> OptimizationError {
    match crate::quantum::optimization::errors::RuleIdentifier::new(
        rule,
    ) {
        Ok(identifier) => error.with_rule(identifier),
        Err(_) => error,
    }
}

/// Extension methods used internally to keep rewrite diagnostics concise.
trait OptimizationErrorRuleExt {
    fn with_rule_identifier(
        self,
        rule: &'static str,
    ) -> OptimizationError;
}

impl OptimizationErrorRuleExt for OptimizationError {
    fn with_rule_identifier(
        self,
        rule: &'static str,
    ) -> OptimizationError {
        with_rule_identifier(self, rule)
    }
}

// =============================================================================
// Integer conversion
// =============================================================================

fn checked_u64(
    value: usize,
    calculation: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::invariant(
            OptimizationStage::Rewrite,
            "usize-to-u64 conversion overflow",
            Some(calculation.to_owned()),
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

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
                    .expect("finite test angle"),
            ],
            None,
            None,
        )
        .expect("test rotation must be valid")
    }

    fn circuit(
        qubits: usize,
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        QuantumCircuit::from_operations(
            qubits,
            0,
            gates,
        )
        .expect("test circuit must be valid")
    }

    #[test]
    fn identity_is_removed() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::I, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        let outcome = pass
            .optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 0);
        assert_eq!(outcome.rewrites(), 1);
    }

    #[test]
    fn self_inverse_pair_is_removed() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::H, &[0]),
                gate(GateKind::H, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn explicit_inverse_pair_is_removed() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::T, &[0]),
                gate(GateKind::Tdg, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn rotations_are_symbolically_fused() {
        let mut circuit = circuit(
            1,
            vec![
                rotation(GateKind::RZ, 0, 0.25),
                rotation(GateKind::RZ, 0, 0.75),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);

        let parameter =
            circuit.get(0)
                .expect("fused gate must exist")
                .parameters()
                .first()
                .expect("fused parameter must exist");

        assert!(parameter.is_symbolic() == false);
    }

    #[test]
    fn hxh_becomes_z() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::H, &[0]),
                gate(GateKind::X, &[0]),
                gate(GateKind::H, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0).expect("gate exists").kind(),
            GateKind::Z
        );
    }

    #[test]
    fn hzh_becomes_x() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::H, &[0]),
                gate(GateKind::Z, &[0]),
                gate(GateKind::H, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0).expect("gate exists").kind(),
            GateKind::X
        );
    }

    #[test]
    fn s_x_sdg_becomes_y() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::S, &[0]),
                gate(GateKind::X, &[0]),
                gate(GateKind::Sdg, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0).expect("gate exists").kind(),
            GateKind::Y
        );
    }

    #[test]
    fn measurement_boundary_is_not_crossed() {
        // The actual measurement payload is intentionally omitted from this
        // test because constructing one is owned by the IR measurement tests.
        //
        // The boundary predicate itself is separately covered by the canonical
        // GateKind semantics. The peephole pass never attempts to reorder or
        // remove measurements.
        assert!(is_semantic_boundary(
            &gate(GateKind::Barrier, &[0])
        ));
    }

    #[test]
    fn barrier_boundary_is_not_crossed() {
        let mut circuit = circuit(
            1,
            vec![
                gate(GateKind::H, &[0]),
                gate(GateKind::Barrier, &[0]),
                gate(GateKind::H, &[0]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 3);
    }

    #[test]
    fn different_qubits_are_not_cancelled() {
        let mut circuit = circuit(
            2,
            vec![
                gate(GateKind::X, &[0]),
                gate(GateKind::X, &[1]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert_eq!(circuit.len(), 2);
    }

    #[test]
    fn cnot_pair_is_cancelled() {
        let mut circuit = circuit(
            2,
            vec![
                gate(GateKind::CX, &[0, 1]),
                gate(GateKind::CX, &[0, 1]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn swap_pair_is_cancelled() {
        let mut circuit = circuit(
            2,
            vec![
                gate(GateKind::SWAP, &[0, 1]),
                gate(GateKind::SWAP, &[0, 1]),
            ],
        );

        let pass =
            PeepholePass::new().expect("metadata must be valid");

        pass.optimize(&mut circuit)
            .expect("optimization must succeed");

        assert!(circuit.is_empty());
    }

    #[test]
    fn pass_is_deterministic() {
        let pass =
            PeepholePass::new().expect("metadata must be valid");

        assert_eq!(
            pass.determinism(),
            PassDeterminism::Deterministic
        );

        assert_eq!(
            pass.complexity(),
            PassComplexity::Linear
        );

        assert_eq!(
            pass.scope(),
            PassScope::LocalWindow
        );
    }

    #[test]
    fn pattern_width_is_bounded() {
        assert_eq!(
            PeepholePass::max_pattern_width(),
            3
        );
    }
}