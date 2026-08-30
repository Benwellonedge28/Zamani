//! Zamani Quantum Optimization — Exact Identity Elimination
//!
//! Production-grade exact identity/no-op elimination for the canonical
//! Zamani Quantum IR.
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
//! optimization::local::identity
//!      │
//!      ▼
//! optimized quantum::ir::QuantumCircuit
//! ```
//!
//! This pass removes only operations whose semantics are exactly the identity
//! operation under the canonical logical Quantum IR semantics.
//!
//! # Responsibilities
//!
//! This pass owns:
//!
//! - removal of explicit `I` gates;
//! - removal of exactly-zero supported rotation operations;
//! - exact identity recognition for operations whose identity condition is
//!   represented directly and unambiguously by the canonical IR;
//! - deterministic, linear-time identity elimination;
//! - transactional/atomic canonical-circuit mutation;
//! - pass-level resource/cancellation cooperation;
//! - standardized optimization-pass accounting.
//!
//! This pass deliberately does NOT own:
//!
//! - adjacent inverse cancellation;
//! - non-local cancellation;
//! - commutation;
//! - symbolic algebra;
//! - phase-polynomial optimization;
//! - approximate identity detection;
//! - global-phase-relaxed optimization;
//! - target-specific optimization;
//! - synthesis;
//! - routing;
//! - scheduling;
//! - hardware interaction;
//! - QPU execution;
//! - measurement optimization;
//! - reset optimization;
//! - dead-code/light-cone elimination.
//!
//! Those responsibilities belong to other optimization layers.
//!
//! # Exact semantic policy
//!
//! This pass is intentionally stricter than an approximate identity-removal
//! pass.
//!
//! An operation is removed only when Zamani can establish exact identity
//! semantics from the operation kind and its canonical parameters.
//!
//! Examples:
//!
//! ```text
//! I(q0)              → <removed>
//! RX(0)(q0)          → <removed>
//! RY(0)(q0)          → <removed>
//! RZ(0)(q0)          → <removed>
//! Phase(0)(q0)       → <removed>
//! U1(0)(q0)          → <removed>
//! CRX(0)(q0,q1)      → <removed>
//! CRY(0)(q0,q1)      → <removed>
//! CRZ(0)(q0,q1)      → <removed>
//! ```
//!
//! The following are deliberately NOT removed here:
//!
//! ```text
//! T T†              // cancellation.rs
//! X X               // cancellation.rs
//! RZ(theta)RZ(-θ)   // cancellation.rs
//! RZ(a + -a)        // parameter/simplification.rs
//! U3(0,0,0)         // requires explicit canonical semantic contract
//! arbitrary U ≈ I   // approximate verification/optimization
//! global phase      // only if the configured equivalence policy permits it
//! ```
//!
//! # Why exact zero uses `== 0.0`
//!
//! Floating-point approximation is deliberately not used.
//!
//! A tolerance such as:
//!
//! ```text
//! abs(theta) < epsilon
//! ```
//!
//! changes the program's exact semantics. Approximate removal must therefore
//! be implemented by a separate pass with an explicit approximation policy,
//! target error model, and verification contract.
//!
//! `-0.0 == 0.0` is intentional: both represent the same mathematical zero
//! for these canonical zero-angle operations.
//!
//! NaN and infinity are never accepted as identity parameters.
//!
//! # Global phase
//!
//! This pass does not remove an operation merely because it is equivalent to
//! identity up to global phase.
//!
//! Exact logical semantics are the default optimization contract.
//!
//! A future global-phase-relaxed pass may use the optimization equivalence
//! subsystem when the compiler explicitly requests that semantic policy.
//!
//! # Measurements, barriers, and reset
//!
//! This pass never removes:
//!
//! - measurements;
//! - barriers;
//! - reset operations.
//!
//! They are not identity operations and may carry observable/compiler
//! semantics beyond their unitary action.
//!
//! # Classical destinations and metadata
//!
//! An operation is removable only if its canonical representation satisfies
//! the complete identity predicate.
//!
//! This prevents future IR extensions from accidentally being erased merely
//! because their gate kind resembles an identity operation.
//!
//! # Scaling
//!
//! The normal algorithm is O(n) in the number of circuit operations.
//!
//! It performs:
//!
//! 1. one read-only scan;
//! 2. one replacement-sequence construction;
//! 3. one canonical validation;
//! 4. one atomic commit.
//!
//! There is no artificial circuit-size limit in this file.
//!
//! Practical scalability is governed by:
//!
//! - available memory;
//! - `QuantumIrLimits`;
//! - `OptimizationLimits`;
//! - optimizer cancellation/deadline policy;
//! - the number of operations;
//! - the allocator/platform.
//!
//! The pass never repeatedly calls `QuantumCircuit::remove()` while scanning.
//! Repeated indexed removal can make a `Vec`-backed circuit quadratic.
//!
//! # Atomicity
//!
//! The original canonical circuit is not modified while identity recognition
//! is being performed.
//!
//! A candidate circuit is built independently. If:
//!
//! - cancellation is requested;
//! - a resource/accounting error occurs;
//! - an IR validation error occurs;
//! - a commit precondition fails;
//!
//! the original circuit remains unchanged.
//!
//! Only after the candidate has passed canonical validation is it assigned to
//! the caller's circuit.
//!
//! # Determinism
//!
//! The pass is deterministic.
//!
//! Given:
//!
//! - the same canonical IR;
//! - the same optimizer context;
//! - the same optimization configuration;
//! - the same resource policy;
//!
//! it produces exactly the same circuit and pass accounting.
//!
//! # Thread safety
//!
//! The pass contains immutable metadata only.
//!
//! Invocation-specific state belongs to `OptimizationContext`.
//!
//! No global mutable state is used.
//!
//! The pass does not spawn threads.
//!
//! Parallel scheduling remains the responsibility of
//! `optimization::scheduler`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! This file contains no unsafe operations.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! The pass consumes:
//!
//! - `Gate`;
//! - `GateKind`;
//! - `Parameter`;
//! - `QuantumCircuit`.
//!
//! No second gate or circuit representation is introduced.
//!
//! The canonical IR guarantees validated gate structure and provides atomic
//! `push`, `clear`, `remove`, `replace`, and circuit validation APIs.
//! 
//!
//! ## `optimization::pass`
//!
//! Implements `OptimizationPass`.
//!
//! Stable pass identifier:
//!
//! `local.identity`
//!
//! The pass declares itself as:
//!
//! - `PassKind::LocalRewrite`;
//! - linear complexity;
//! - deterministic;
//! - operation-removing;
//! - gate-count-changing.
//!
//! ## `optimization::context`
//!
//! Uses only the existing optimizer invocation context services required by
//! this pass:
//!
//! - `check_cancelled()`;
//! - `record_rewrite()`.
//!
//! The pass does not create a second cancellation, timeout, or rewrite-budget
//! system.
//!
//! ## `optimization::errors`
//!
//! All failures use the canonical `OptimizationError`.
//!
//! No `IdentityError` public error type is introduced.
//!
//! ## `optimization::circuit`
//!
//! The generic `CircuitEditor`/`CircuitEditPlan` is not required for the normal
//! implementation because identity elimination constructs a complete candidate
//! before commit.
//!
//! This avoids repeated indexed deletion and keeps the transformation O(n).
//!
//! ## `optimization::local::cancellation`
//!
//! `cancellation.rs` may also recognize explicit identities as an optimization
//! convenience, but the dedicated `local.identity` pass is the canonical
//! registry/planner entry for identity-only elimination.
//!
//! Future pipelines should normally avoid running both identity elimination
//! implementations redundantly in the same stage.
//!
//! The cancellation pass already operates on the canonical IR and uses a
//! linear replacement-vector strategy. 
//!
//! ## `optimization::parameter`
//!
//! Symbolic expressions such as:
//!
//! ```text
//! theta + (-theta)
//! a - a
//! 2*pi - 2*pi
//! ```
//!
//! are NOT simplified here.
//!
//! `parameter::simplification` and `parameter::constant_fold` own that work.
//!
//! After those passes produce an exact canonical zero, this pass can remove
//! the resulting zero-angle operation.
//!
//! ## `optimization::canonical`
//!
//! Canonical normalization may run before this pass.
//!
//! This pass nevertheless remains safe when invoked independently because it
//! validates the input circuit before transforming it.
//!
//! ## `optimization::verification`
//!
//! Exact identity elimination is mathematically semantics-preserving under the
//! canonical logical equivalence relation.
//!
//! Pipeline-level verification may still verify the complete optimized circuit.
//!
//! ## `optimization::registry`
//!
//! The registry should register this pass under:
//!
//! `local.identity`
//!
//! with alias:
//!
//! `identity`
//!
//! Registration belongs outside this file so this file remains independently
//! complete and does not depend on registry initialization order.
//!
//! ## `optimization::planner`
//!
//! The planner may select this pass for:
//!
//! - O1;
//! - O2;
//! - O3;
//! - normalization;
//! - generic simplification;
//! - gate-count optimization;
//! - depth optimization;
//! - two-qubit optimization;
//! - fault-tolerant optimization.
//!
//! Because it is linear and exact, it is suitable as a cheap early pass.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline invokes `run()`.
//!
//! The pipeline remains responsible for:
//!
//! - pass sequencing;
//! - fixed-point orchestration;
//! - verification policy;
//! - provenance;
//! - final optimization result construction.
//!
//! ## `optimization::statistics`
//!
//! `PassOutcome` provides standardized operation accounting.
//!
//! Detailed aggregate statistics remain owned by the common statistics layer.
//!
//! # Identity semantics supported by this file
//!
//! The canonical `GateKind` currently contains:
//!
//! - `I`;
//! - `RX`;
//! - `RY`;
//! - `RZ`;
//! - `Phase`;
//! - `U1`;
//! - `CRX`;
//! - `CRY`;
//! - `CRZ`;
//! - `U2`;
//! - `U3`;
//! - standard discrete gates;
//! - non-unitary operations.
//!
//! Only identity conditions that are unambiguous from the current IR are
//! recognized.
//!
//! In particular, this pass does NOT assume a particular mathematical
//! convention for `U2`/`U3` beyond what the canonical IR explicitly guarantees.
//!
//! This is intentional: an optimizer must not silently encode a gate convention
//! that belongs in the canonical IR specification.
//!
//! # Extension rule
//!
//! When a future `GateKind` acquires an intrinsic identity form, add it here
//! only if the canonical IR specification establishes the identity exactly.
//!
//! If a new identity requires symbolic algebra, global-phase reasoning,
//! target-specific semantics, or approximate equivalence, implement that logic
//! in the appropriate subsystem instead.
//!
//! # Production invariant
//!
//! This file must remain usable without requiring any future optimization file
//! to be edited.
//!
//! Its dependencies are exclusively:
//!
//! - stable canonical Quantum IR;
//! - stable optimization error/context/pass contracts.
//!
//! Future files integrate by consuming this pass, not by changing it.
//!
//! # External design precedent
//!
//! Identity/no-op elimination is a standard canonicalization transformation in
//! modern compiler infrastructure. MLIR explicitly lists identity/no-op
//! elimination among common canonicalization patterns. 5
//!
//! Quantum compiler stacks likewise distinguish exact inverse cancellation,
//! identity-equivalent removal, and more aggressive one-qubit optimization.
//! Qiskit's current transpiler documentation lists these as separate
//! optimization capabilities. 6

#![forbid(unsafe_code)]

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{OptimizationError, OptimizationStage};
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
// Stable pass metadata
// =============================================================================

/// Stable machine-readable pass identifier.
///
/// This identifier becomes part of optimizer provenance and configuration.
pub const PASS_ID: &str = "local.identity";

/// Human-readable pass name.
pub const PASS_NAME: &str = "Exact Identity Elimination";

/// Stable alias used by configuration/front-end layers.
pub const PASS_ALIAS: &str = "identity";

// =============================================================================
// Pass
// =============================================================================

/// Exact identity/no-op elimination pass.
///
/// The pass is stateless between optimizer invocations.
#[derive(Debug, Clone)]
pub struct IdentityPass {
    metadata: PassMetadata,
}

impl IdentityPass {
    /// Constructs the production identity-elimination pass.
    ///
    /// Construction cannot fail for a statically defined valid pass ID and
    /// metadata configuration. A programmer error therefore results in a
    /// descriptive panic rather than contaminating the runtime API with an
    /// impossible construction failure.
    #[must_use]
    pub fn new() -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect("local.identity has a valid static identifier");

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::LocalRewrite,
        )
        .expect("identity pass metadata must be valid")
        .with_scope(PassScope::Operation)
        .with_complexity(PassComplexity::Linear)
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::ChangesGateCount);

        Self { metadata }
    }

    /// Returns whether `gate` is an exact identity operation handled by this
    /// pass.
    ///
    /// This function is public so canonicalization, tests, planners, and other
    /// local optimization components can query the exact identity predicate
    /// without duplicating it.
    #[must_use]
    pub fn is_identity(gate: &Gate) -> bool {
        is_exact_identity_gate(gate)
    }

    /// Returns the number of identity operations in a canonical operation
    /// sequence.
    ///
    /// This is a read-only O(n) helper and does not allocate.
    #[must_use]
    pub fn count_identities(operations: &[Gate]) -> usize {
        operations
            .iter()
            .filter(|gate| Self::is_identity(gate))
            .count()
    }

    /// Builds the identity-free operation sequence.
    ///
    /// The input is never mutated.
    ///
    /// The returned vector contains only clones of operations that were already
    /// present in the validated canonical circuit.
    fn transform(
        &self,
        operations: &[Gate],
        context: &mut OptimizationContext,
    ) -> Result<Vec<Gate>, OptimizationError> {
        // Count first so the replacement vector does not over-allocate for
        // circuits containing a significant number of identities.
        //
        // This is still O(n), and avoids a second capacity expansion when the
        // identity density is high.
        let identity_count = Self::count_identities(operations);

        if identity_count == 0 {
            return Ok(operations.to_vec());
        }

        let output_capacity = operations
            .len()
            .checked_sub(identity_count)
            .ok_or_else(|| {
                OptimizationError::internal(
                    OptimizationStage::LocalOptimization,
                    "identity count exceeded operation count",
                )
            })?;

        let mut output = Vec::with_capacity(output_capacity);

        for gate in operations {
            context
                .check_cancelled()
                .map_err(|error| {
                    OptimizationError::internal(
                        OptimizationStage::LocalOptimization,
                        format!(
                            "identity elimination was cancelled: {error}"
                        ),
                    )
                })?;

            if Self::is_identity(gate) {
                context
                    .record_rewrite()
                    .map_err(|error| {
                        OptimizationError::internal(
                            OptimizationStage::LocalOptimization,
                            format!(
                                "failed to record identity elimination: {error}"
                            ),
                        )
                    })?;

                continue;
            }

            output.push(gate.clone());
        }

        debug_assert_eq!(
            output.len(),
            operations.len() - identity_count
        );

        Ok(output)
    }
}

impl Default for IdentityPass {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationPass for IdentityPass {
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
                        "identity elimination cannot start: {error}"
                    ),
                )
            })?;

        let operations_before = circuit.len();

        // Empty circuits are a valid no-op.
        if operations_before == 0 {
            return Ok(PassOutcome::unchanged(0, 0));
        }

        // Do not trust a reconstructed/deserialized circuit merely because it
        // has the canonical Rust type. Validate the semantic IR boundary before
        // transforming it.
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "identity elimination received invalid Quantum IR: {error}"
                ),
            )
        })?;

        // Fast path: determine whether any identity exists before allocating a
        // replacement sequence.
        let identity_count = Self::count_identities(circuit.operations());

        if identity_count == 0 {
            return Ok(PassOutcome::unchanged(
                usize_to_u64(
                    operations_before,
                    "operation count before identity elimination",
                )?,
                usize_to_u64(
                    operations_before,
                    "operation count after identity elimination",
                )?,
            ));
        }

        // Construct the candidate before touching the original circuit.
        //
        // `transform()` performs cancellation/deadline checks while building the
        // candidate. Any error therefore leaves the original circuit unchanged.
        let optimized = self.transform(
            circuit.operations(),
            context,
        )?;

        let operations_after = optimized.len();

        debug_assert!(
            operations_after < operations_before
        );

        // A second cancellation/deadline boundary is intentionally checked
        // immediately before the commit.
        context
            .check_cancelled()
            .map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::LocalOptimization,
                    format!(
                        "identity elimination was cancelled before commit: {error}"
                    ),
                )
            })?;

        // Build an independent candidate QuantumCircuit.
        //
        // Cloning the circuit preserves:
        //
        // - circuit identity;
        // - IR version;
        // - logical qubit namespace;
        // - classical namespace;
        // - IR resource policy;
        // - metadata.
        //
        // We then replace only the operation sequence.
        //
        // The original circuit remains untouched until the candidate has been
        // completely validated.
        let mut candidate = circuit.clone();

        candidate.clear();

        for gate in optimized {
            candidate.push(gate).map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::LocalOptimization,
                    format!(
                        "failed to construct identity-elimination candidate: {error}"
                    ),
                )
            })?;
        }

        // Final canonical semantic/structural validation is the commit
        // precondition.
        candidate.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::LocalOptimization,
                format!(
                    "identity elimination produced invalid Quantum IR: {error}"
                ),
            )
        })?;

        // The candidate is now fully validated. Assignment is infallible and
        // therefore provides the atomic commit boundary.
        *circuit = candidate;

        Ok(PassOutcome::changed(
            usize_to_u64(
                operations_before,
                "operation count before identity elimination",
            )?,
            usize_to_u64(
                operations_after,
                "operation count after identity elimination",
            )?,
        )
        .with_operations_removed(
            usize_to_u64(
                identity_count,
                "identity operation count",
            )?,
        )
        .with_rewrites(
            usize_to_u64(
                identity_count,
                "identity rewrite count",
            )?,
        ))
    }
}

// =============================================================================
// Exact identity semantics
// =============================================================================

/// Returns whether a gate is an intrinsic exact identity operation.
///
/// IMPORTANT:
///
/// This function is intentionally conservative.
///
/// It does not attempt:
///
/// - symbolic simplification;
/// - matrix construction;
/// - numerical unitary comparison;
/// - global-phase comparison;
/// - target-dependent equivalence;
/// - approximate equivalence.
#[must_use]
fn is_exact_identity_gate(gate: &Gate) -> bool {
    // Explicit identity gate.
    //
    // Requiring the complete canonical payload to be empty prevents a future
    // extension from accidentally being removed merely because its kind is I.
    if gate.kind() == GateKind::I {
        return gate.parameters().is_empty()
            && gate.classical_target().is_none()
            && gate.measurement().is_none();
    }

    // All remaining supported intrinsic identities are exactly-zero
    // parameterized rotations.
    //
    // We deliberately do not include U2/U3 here because their exact identity
    // conditions depend on the canonical gate convention and, in some
    // representations, global phase.
    match gate.kind() {
        GateKind::RX
        | GateKind::RY
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRX
        | GateKind::CRY
        | GateKind::CRZ => {
            gate.parameters().len() == 1
                && gate.classical_target().is_none()
                && gate.measurement().is_none()
                && is_exact_zero_parameter(
                    &gate.parameters()[0],
                )
        }

        _ => false,
    }
}

/// Returns whether a canonical parameter is exactly numerical zero.
///
/// Symbolic expressions are deliberately rejected here.
///
/// For example:
///
/// ```text
/// Symbol("x") + -Symbol("x")
/// ```
///
/// may be mathematically zero, but proving that belongs to the symbolic
/// parameter subsystem.
#[must_use]
fn is_exact_zero_parameter(parameter: &Parameter) -> bool {
    match parameter {
        Parameter::Constant(value) => {
            value.is_finite() && *value == 0.0
        }

        Parameter::Symbol(_) | Parameter::Expression(_) => false,
    }
}

// =============================================================================
// Counter conversion
// =============================================================================

/// Converts a platform `usize` counter into the optimizer's common `u64`
/// accounting representation.
///
/// The conversion is checked so statistics never silently wrap or truncate.
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

    fn qubit(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit ID must be valid")
    }

    fn parameter(value: f64) -> Parameter {
        Parameter::Constant(value)
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
        .expect("test gate must be valid")
    }

    fn circuit_with(
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::new(4, 4)
            .expect("test circuit must construct");

        for gate in gates {
            circuit
                .push(gate)
                .expect("test gate must be accepted");
        }

        circuit
    }

    fn context() -> OptimizationContext {
        OptimizationContext::new(
            OptimizationConfig::default(),
            OptimizationLimits::production(),
        )
        .expect("production optimization context must construct")
    }

    // -------------------------------------------------------------------------
    // Metadata
    // -------------------------------------------------------------------------

    #[test]
    fn metadata_is_stable() {
        let pass = IdentityPass::new();

        assert_eq!(
            pass.metadata().id_str(),
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
            PassScope::Operation
        );

        assert_eq!(
            pass.metadata().complexity(),
            PassComplexity::Linear
        );

        assert!(
            pass.metadata()
                .has_capability(
                    PassCapability::RemovesOperations
                )
        );

        assert!(
            pass.metadata()
                .has_capability(
                    PassCapability::ChangesGateCount
                )
        );

        pass.validate()
            .expect("identity metadata must be valid");
    }

    // -------------------------------------------------------------------------
    // Explicit identity
    // -------------------------------------------------------------------------

    #[test]
    fn removes_explicit_identity() {
        let circuit = circuit_with(vec![
            gate(
                GateKind::I,
                vec![qubit(0)],
                Vec::new(),
            ),
        ]);

        let mut circuit = circuit;
        let pass = IdentityPass::new();

        let outcome = pass
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert!(circuit.is_empty());
        assert!(outcome.changed());
        assert_eq!(outcome.operations_before(), 1);
        assert_eq!(outcome.operations_after(), 0);
        assert_eq!(outcome.operations_removed(), 1);
        assert_eq!(outcome.rewrites(), 1);
    }

    // -------------------------------------------------------------------------
    // Zero rotations
    // -------------------------------------------------------------------------

    #[test]
    fn removes_zero_rx() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RX,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("RX(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_ry() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RY,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("RY(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_rz() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("RZ(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_phase() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::Phase,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("Phase(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_u1() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::U1,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("U1(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_crx() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::CRX,
                vec![qubit(0), qubit(1)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("CRX(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_cry() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::CRY,
                vec![qubit(0), qubit(1)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("CRY(0) must be removable");

        assert!(circuit.is_empty());
    }

    #[test]
    fn removes_zero_crz() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::CRZ,
                vec![qubit(0), qubit(1)],
                vec![parameter(0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("CRZ(0) must be removable");

        assert!(circuit.is_empty());
    }

    // -------------------------------------------------------------------------
    // Exactness
    // -------------------------------------------------------------------------

    #[test]
    fn does_not_remove_nonzero_rotation() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(1.0e-30)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn does_not_remove_negative_nonzero_rotation() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RX,
                vec![qubit(0)],
                vec![parameter(-1.0e-30)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn removes_negative_zero() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(-0.0)],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("negative zero is exact zero");

        assert!(circuit.is_empty());
    }

    #[test]
    fn does_not_remove_symbolic_zero_like_expression() {
        let symbolic = Parameter::Symbol(
            "theta".to_string(),
        );

        let mut circuit = circuit_with(vec![
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![symbolic],
            ),
        ]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("symbolic operation must be accepted");

        assert_eq!(circuit.len(), 1);
    }

    // -------------------------------------------------------------------------
    // Boundaries
    // -------------------------------------------------------------------------

    #[test]
    fn does_not_remove_measurement() {
        let measurement = Gate::measure(
            qubit(0),
            0,
        )
        .expect("measurement must be valid");

        let mut circuit =
            circuit_with(vec![measurement]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 1);
        assert!(circuit.first().unwrap().is_measurement());
    }

    #[test]
    fn does_not_remove_reset() {
        let reset = Gate::reset(qubit(0))
            .expect("reset must be valid");

        let mut circuit =
            circuit_with(vec![reset]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 1);
        assert!(circuit.first().unwrap().is_reset());
    }

    #[test]
    fn does_not_remove_barrier() {
        let barrier = Gate::barrier(
            vec![qubit(0)],
        )
        .expect("barrier must be valid");

        let mut circuit =
            circuit_with(vec![barrier]);

        IdentityPass::new()
            .run(&mut circuit, &mut context())
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 1);
        assert!(circuit.first().unwrap().is_barrier());
    }

    // -------------------------------------------------------------------------
    // Mixed circuit
    // -------------------------------------------------------------------------

    #[test]
    fn removes_only_intrinsic_identities() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::I,
                vec![qubit(0)],
                Vec::new(),
            ),
            gate(
                GateKind::H,
                vec![qubit(0)],
                Vec::new(),
            ),
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
            gate(
                GateKind::X,
                vec![qubit(1)],
                Vec::new(),
            ),
        ]);

        let outcome = IdentityPass::new()
            .run(
                &mut circuit,
                &mut context(),
            )
            .expect("identity elimination must succeed");

        assert_eq!(circuit.len(), 2);
        assert_eq!(
            circuit.get(0).unwrap().kind(),
            GateKind::H
        );
        assert_eq!(
            circuit.get(1).unwrap().kind(),
            GateKind::X
        );

        assert_eq!(
            outcome.operations_removed(),
            2
        );
    }

    // -------------------------------------------------------------------------
    // Idempotence
    // -------------------------------------------------------------------------

    #[test]
    fn second_run_is_unchanged() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::I,
                vec![qubit(0)],
                Vec::new(),
            ),
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
            gate(
                GateKind::H,
                vec![qubit(0)],
                Vec::new(),
            ),
        ]);

        let pass = IdentityPass::new();

        pass.run(
            &mut circuit,
            &mut context(),
        )
        .expect("first run must succeed");

        let second = pass
            .run(
                &mut circuit,
                &mut context(),
            )
            .expect("second run must succeed");

        assert_eq!(
            second.change(),
            super::super::super::pass::PassChange::Unchanged
        );

        assert_eq!(
            circuit.len(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Predicate
    // -------------------------------------------------------------------------

    #[test]
    fn identity_predicate_is_exact() {
        let identity = gate(
            GateKind::I,
            vec![qubit(0)],
            Vec::new(),
        );

        let non_identity = gate(
            GateKind::X,
            vec![qubit(0)],
            Vec::new(),
        );

        let zero_rotation = gate(
            GateKind::RZ,
            vec![qubit(0)],
            vec![parameter(0.0)],
        );

        let nonzero_rotation = gate(
            GateKind::RZ,
            vec![qubit(0)],
            vec![parameter(0.1)],
        );

        assert!(
            IdentityPass::is_identity(
                &identity
            )
        );

        assert!(
            !IdentityPass::is_identity(
                &non_identity
            )
        );

        assert!(
            IdentityPass::is_identity(
                &zero_rotation
            )
        );

        assert!(
            !IdentityPass::is_identity(
                &nonzero_rotation
            )
        );
    }

    #[test]
    fn count_identities_is_linear_and_exact() {
        let operations = vec![
            gate(
                GateKind::I,
                vec![qubit(0)],
                Vec::new(),
            ),
            gate(
                GateKind::X,
                vec![qubit(0)],
                Vec::new(),
            ),
            gate(
                GateKind::RZ,
                vec![qubit(0)],
                vec![parameter(0.0)],
            ),
            gate(
                GateKind::H,
                vec![qubit(0)],
                Vec::new(),
            ),
        ];

        assert_eq!(
            IdentityPass::count_identities(
                &operations
            ),
            2
        );
    }

    // -------------------------------------------------------------------------
    // Circuit preservation
    // -------------------------------------------------------------------------

    #[test]
    fn preserves_circuit_identity_metadata_and_limits() {
        let mut circuit = circuit_with(vec![
            gate(
                GateKind::I,
                vec![qubit(0)],
                Vec::new(),
            ),
        ]);

        let original_id = circuit.id();
        let original_version = circuit.version();
        let original_limits = *circuit.limits();
        let original_metadata = circuit.metadata().clone();

        IdentityPass::new()
            .run(
                &mut circuit,
                &mut context(),
            )
            .expect("identity elimination must succeed");

        assert_eq!(
            circuit.id(),
            original_id
        );

        assert_eq!(
            circuit.version(),
            original_version
        );

        assert_eq!(
            circuit.limits(),
            &original_limits
        );

        assert_eq!(
            circuit.metadata(),
            &original_metadata
        );
    }

    // -------------------------------------------------------------------------
    // Mixed large input
    // -------------------------------------------------------------------------

    #[test]
    fn handles_large_identity_heavy_sequences() {
        let mut operations = Vec::with_capacity(10_000);

        for index in 0..10_000usize {
            if index % 2 == 0 {
                operations.push(
                    gate(
                        GateKind::I,
                        vec![qubit(0)],
                        Vec::new(),
                    ),
                );
            } else {
                operations.push(
                    gate(
                        GateKind::X,
                        vec![qubit(0)],
                        Vec::new(),
                    ),
                );
            }
        }

        let mut circuit =
            circuit_with(operations);

        let outcome = IdentityPass::new()
            .run(
                &mut circuit,
                &mut context(),
            )
            .expect("large identity workload must succeed");

        assert_eq!(
            circuit.len(),
            5_000
        );

        assert_eq!(
            outcome.operations_removed(),
            5_000
        );

        assert_eq!(
            outcome.operations_before(),
            10_000
        );

        assert_eq!(
            outcome.operations_after(),
            5_000
        );
    }
}