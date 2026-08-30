//! Zamani Quantum Optimization — Normalization Pass.
//!
//! Production optimization-pass adapter for canonical Quantum IR
//! normalization.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                      │
//!                                      ▼
//!                    optimization::passes::normalize
//!                                      │
//!                                      ▼
//!                       optimization::canonical
//!                                      │
//!                                      ▼
//!                         canonical Quantum IR
//!                                      │
//!                    ┌─────────────────┴─────────────────┐
//!                    ▼                                   ▼
//!             local optimization                  algebra/synthesis
//! ```
//!
//! This file is intentionally a PASS adapter, not a second canonicalization
//! engine.
//!
//! The authoritative normalization implementation is:
//!
//! `crate::quantum::optimization::canonical`
//!
//! In particular:
//!
//! - `CanonicalizationPolicy` owns normalization policy;
//! - `canonicalize_circuit_with_policy` owns canonicalization semantics;
//! - `CanonicalizationResult` owns the canonicalized circuit and report.
//!
//! This pass owns:
//!
//! - the stable optimizer pass identifier;
//! - pass metadata;
//! - integration with `OptimizationPass`;
//! - optimizer resource/deadline cooperation;
//! - conversion from canonicalization failures into `OptimizationError`;
//! - atomic installation of the canonicalized circuit;
//! - pipeline-facing statistics.
//!
//! # Why this separation is mandatory
//!
//! The canonical Quantum IR is owned by `quantum::ir`. The optimization
//! subsystem must never create another circuit or gate representation.
//!
//! The canonical IR currently provides validated construction, immutable
//! operation access, atomic replacement/removal, circuit validation, resource
//! limits, identity/version preservation, and metadata preservation.
//!
//! This pass therefore never defines:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - another circuit type;
//! - another qubit type;
//! - another parameter type.
//!
//! # What normalization means
//!
//! This pass performs representation-level canonicalization only.
//!
//! It may perform transformations already defined by `optimization::canonical`,
//! including:
//!
//! - exact parameter-expression normalization;
//! - exact constant folding;
//! - signed-zero normalization;
//! - deterministic ordering of operands for mathematically symmetric gates;
//! - validation before and after canonicalization.
//!
//! It deliberately does NOT perform:
//!
//! - gate cancellation;
//! - inverse cancellation;
//! - commutation;
//! - rotation fusion;
//! - gate fusion;
//! - synthesis;
//! - routing;
//! - scheduling;
//! - hardware-specific decomposition;
//! - target-specific optimization;
//! - T-count optimization;
//! - T-depth optimization;
//! - approximate optimization;
//! - equality saturation;
//! - QPU/backend execution.
//!
//! Those responsibilities belong to later passes/subsystems.
//!
//! # Semantic contract
//!
//! The pass is semantically preserving under the canonical IR's exact
//! representation semantics.
//!
//! No numerical epsilon is introduced by this adapter.
//!
//! No random state is used.
//!
//! No backend state is consulted.
//!
//! No wall-clock state is consulted.
//!
//! No environment state is consulted.
//!
//! # Transactionality
//!
//! The canonicalizer first constructs and validates a complete replacement
//! circuit.
//!
//! This pass does NOT incrementally mutate the caller's circuit operation by
//! operation.
//!
//! Once canonicalization succeeds, the fully validated canonical circuit is
//! installed with `std::mem::swap`.
//!
//! Therefore:
//!
//! ```text
//! input circuit
//!      │
//!      ├── validation
//!      │
//!      ├── canonicalization
//!      │
//!      ├── output validation
//!      │
//!      └── only then ──► atomic installation
//! ```
//!
//! If canonicalization fails, the caller's circuit remains untouched.
//!
//! # Scaling
//!
//! The pass itself performs no circuit-sized secondary search.
//!
//! Its complexity is inherited from `optimization::canonical`:
//!
//! - approximately O(n + p), where `n` is operation count and `p` is the
//!   number of parameter-expression nodes;
//! - O(n + p) temporary output storage;
//! - no quadratic operation-pair scan;
//! - no e-graph;
//! - no global search;
//! - no recursion proportional to circuit size.
//!
//! Therefore the pass scales from tiny circuits to circuits limited by:
//!
//! - available memory;
//! - `QuantumIrLimits`;
//! - optimizer resource limits;
//! - the platform's address space;
//! - the configured compilation budget.
//!
//! "Infinite" circuit support is consequently interpreted as **no artificial
//! algorithmic circuit-size ceiling**. Physical resources and explicit
//! compiler limits remain authoritative.
//!
//! # Fixed-point behavior
//!
//! Canonicalization is idempotent by contract:
//!
//! ```text
//! normalize(normalize(C)) == normalize(C)
//! ```
//!
//! Therefore this pass is safe for fixed-point pipelines.
//!
//! A second invocation should normally report `Unchanged`.
//!
//! # Integration contract
//!
//! ## `optimization::canonical`
//!
//! This pass delegates all canonicalization semantics to that module.
//!
//! ## `optimization::pass`
//!
//! Implements `OptimizationPass`.
//!
//! Metadata declares:
//!
//! - normalization;
//! - circuit scope;
//! - linear complexity;
//! - deterministic execution;
//! - semantic preservation;
//! - parameter changes;
//! - operation replacement;
//! - large-circuit support;
//! - fixed-point safety.
//!
//! ## `optimization::context`
//!
//! The pass receives an invocation-scoped `OptimizationContext`.
//!
//! It does not create global optimizer state.
//!
//! Resource/deadline checks are delegated to the context.
//!
//! ## `optimization::pipeline`
//!
//! The intended default position is:
//!
//! ```text
//! validation
//!     ↓
//! normalize
//!     ↓
//! parameter simplification
//!     ↓
//! local optimization
//!     ↓
//! algebraic optimization
//!     ↓
//! synthesis
//! ```
//!
//! ## `optimization::planner`
//!
//! The planner can select this pass for O0/O1/O2/O3 and other profiles because
//! canonicalization is representation normalization rather than a hardware
//! optimization.
//!
//! ## `optimization::registry`
//!
//! Register the pass using `PASS_ID`.
//!
//! The registry should construct it through `Normalize::new()`.
//!
//! ## `optimization::statistics`
//!
//! The generic `PassOutcome` reports operation counts and rewrite/change count.
//! The underlying `CanonicalizationReport` remains available through the
//! direct normalization API.
//!
//! ## `optimization::provenance`
//!
//! `PASS_ID` is stable and should be recorded as the pass identifier.
//!
//! ## `optimization::verification`
//!
//! This pass declares semantic preservation. Whole-circuit verification remains
//! owned by the verification subsystem.
//!
//! ## `optimization::local` / `algebra` / `parameter` / `synthesis`
//!
//! These later passes consume the canonical representation produced here.
//!
//! They must not reimplement this pass's representation normalization.
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
//! - no `unsafe`.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe operations are required.

#![forbid(unsafe_code)]

use std::mem;

use crate::quantum::ir::QuantumCircuit;

use super::super::canonical::{
    canonicalize_circuit_with_policy,
    CanonicalizationPolicy,
    CanonicalizationReport,
    CanonicalizationResult,
};

use super::super::context::OptimizationContext;

use super::super::errors::OptimizationError;

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
// Stable identifiers
// =============================================================================

/// Stable machine-readable identifier for the normalization pass.
///
/// This identifier is part of optimizer provenance and should not be changed
/// merely because the Rust type or file organization changes.
pub const PASS_ID: &str = "passes.normalize";

/// Human-readable pass name.
pub const PASS_NAME: &str = "Quantum IR Normalization";

/// Pass contract version.
///
/// This is the version of this pass adapter's public behavior, not the Quantum
/// IR schema version and not the canonicalizer implementation version.
pub const PASS_VERSION: u32 = 1;

// =============================================================================
// Normalization pass
// =============================================================================

/// Production canonical Quantum IR normalization pass.
///
/// The pass contains no mutable invocation state. The configured
/// `CanonicalizationPolicy` is immutable and can therefore safely be shared
/// by the pass registry/scheduler.
#[derive(Debug, Clone)]
pub struct Normalize {
    metadata: PassMetadata,
    policy: CanonicalizationPolicy,
}

impl Normalize {
    /// Creates the production normalization pass.
    ///
    /// The production canonicalization policy is deliberately conservative and
    /// representation-focused.
    pub fn new() -> Result<Self, OptimizationError> {
        Self::with_policy(CanonicalizationPolicy::production())
    }

    /// Creates a normalization pass using an explicit canonicalization policy.
    ///
    /// This is the extension point for compiler profiles that need stricter
    /// validation-only normalization or another already-supported canonical
    /// representation policy.
    ///
    /// The policy does not introduce a second normalization implementation.
    pub fn with_policy(
        policy: CanonicalizationPolicy,
    ) -> Result<Self, OptimizationError> {
        let metadata = build_metadata()?;

        Ok(Self {
            metadata,
            policy,
        })
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the pass contract version.
    #[must_use]
    pub const fn pass_version() -> u32 {
        PASS_VERSION
    }

    /// Returns the canonicalization policy used by this pass.
    #[must_use]
    pub const fn policy(&self) -> CanonicalizationPolicy {
        self.policy
    }

    /// Returns the canonicalization implementation version used by the
    /// underlying canonicalizer.
    ///
    /// This is exposed so provenance/reporting layers can distinguish a pass
    /// adapter version from the canonicalizer contract version.
    #[must_use]
    pub const fn canonicalization_version() -> u32 {
        super::super::canonical::CANONICALIZATION_VERSION
    }

    /// Canonicalizes a circuit without mutating it.
    ///
    /// This direct API is useful for:
    ///
    /// - unit tests;
    /// - verification;
    /// - compiler tooling;
    /// - diagnostics;
    /// - preview/dry-run optimization;
    /// - future incremental compilation infrastructure.
    pub fn normalize_circuit(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<CanonicalizationResult, OptimizationError> {
        canonicalize_circuit_with_policy(
            circuit,
            self.policy,
        )
        .map_err(|error| {
            canonicalization_error(
                "canonicalization failed",
                error.to_string(),
            )
        })
    }

    /// Returns the canonicalization report without installing the result.
    ///
    /// This is a convenience API for diagnostics and benchmarking.
    pub fn inspect(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<CanonicalizationReport, OptimizationError> {
        Ok(self.normalize_circuit(circuit)?.report())
    }

    /// Executes normalization against the canonical circuit.
    ///
    /// The caller's circuit is modified only after a complete canonicalized
    /// replacement has been successfully constructed and validated.
    pub fn normalize(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        self.run_impl(circuit, context)
    }

    // -------------------------------------------------------------------------
    // Implementation
    // -------------------------------------------------------------------------

    fn run_impl(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        // ---------------------------------------------------------------------
        // Input validation
        // ---------------------------------------------------------------------

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                super::super::errors::OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })?;

        // ---------------------------------------------------------------------
        // Resource/deadline cooperation
        // ---------------------------------------------------------------------
        //
        // The pass itself does not invent optimizer-wide limits. The context
        // owns those policies.
        //
        // Canonicalization is linear, so one initial check and one final check
        // avoid imposing a per-operation context call overhead on enormous
        // circuits while still respecting invocation-level cancellation and
        // resource policy.

        context.check_limits().map_err(|error| {
            canonicalization_error(
                "optimizer resource check failed",
                error.to_string(),
            )
        })?;

        let operations_before = checked_u64(circuit.len())?;

        // Empty and single-operation circuits are valid inputs and require no
        // special canonicalization shortcut. Running the canonicalizer keeps
        // validation and policy behavior identical for every circuit size.
        let result = self.normalize_circuit(circuit)?;

        let report = result.report();

        let operations_after = checked_u64(result.circuit().len())?;

        // Canonicalization is representation-only. It must not change the
        // number of operations.
        //
        // Treat a violation as an optimizer invariant failure rather than
        // silently accepting an unexpected transformation.
        if operations_before != operations_after {
            return Err(
                OptimizationError::pass_failure(
                    PASS_ID,
                    format!(
                        "canonicalization changed operation count: \
                         before={operations_before}, \
                         after={operations_after}"
                    ),
                ),
            );
        }

        // The canonicalizer has already validated the output. Perform the
        // final optimizer-level resource check before installation.
        context.check_limits().map_err(|error| {
            canonicalization_error(
                "final optimizer resource check failed",
                error.to_string(),
            )
        })?;

        // ---------------------------------------------------------------------
        // Atomic installation
        // ---------------------------------------------------------------------
        //
        // `CanonicalizationResult` owns the fully validated replacement
        // circuit. Swap only after every previous stage has succeeded.
        //
        // This is stronger than replacing operations one-by-one because a
        // failure during canonicalization can never leave a partially
        // normalized caller-owned circuit.
        let mut canonical_circuit = result.into_circuit();

        mem::swap(circuit, &mut canonical_circuit);

        // `canonical_circuit` now owns the previous circuit and is immediately
        // dropped. No caller-visible partially normalized state exists.
        drop(canonical_circuit);

        let changed = !report.is_unchanged();

        let change = if changed {
            PassChange::Changed
        } else {
            PassChange::Unchanged
        };

        let changed_operations =
            checked_u64(report.operations_changed())?;

        let message = if changed {
            format!(
                "canonicalized {changed_operations} operation(s); \
                 parameter_expressions={}, \
                 constant_folds={}, \
                 signed_zero={}, \
                 symmetric_operands={}",
                report.parameter_expressions_normalized(),
                report.constants_folded(),
                report.signed_zero_normalized(),
                report.symmetric_operands_normalized(),
            )
        } else {
            "circuit was already canonical".to_string()
        };

        Ok(
            PassOutcome::unchanged(
                operations_before,
                operations_after,
            )
            .with_change(change)
            .with_operations_replaced(
                changed_operations,
            )
            .with_rewrites(
                changed_operations,
            )
            .with_iterations(1)
            .with_message(message),
        )
    }
}

// =============================================================================
// OptimizationPass implementation
// =============================================================================

impl OptimizationPass for Normalize {
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
        // Canonicalization is idempotent. A pipeline may safely stop once this
        // pass reports no change.
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
        "Canonicalizes the logical Quantum IR representation without \
         performing target-dependent or cost-driven optimization.",
        PassKind::Normalization,
        PassScope::Circuit,
        PassComplexity::Linear,
        PassDeterminism::Deterministic,
    )
    .with_capabilities([
        PassCapability::ReplacesOperations,
        PassCapability::ChangesParameters,
    ])
    .with_semantic_preservation(true)
    .supports_empty_circuit(true)
    .supports_single_operation(true)
    .supports_large_circuits(true)
    .requires_target(false)
    .requires_verification(false)
    .fixed_point_safe(true);

    metadata.validate().map_err(|error| {
        OptimizationError::pass_failure(
            PASS_ID,
            format!(
                "invalid normalization pass metadata: {error}"
            ),
        )
    })?;

    Ok(metadata)
}

// =============================================================================
// Error helpers
// =============================================================================

/// Converts a canonicalization failure into the optimizer-wide error contract.
///
/// The canonicalizer intentionally owns its own local error vocabulary. This
/// adapter prevents that implementation detail from leaking through the
/// optimizer pass interface.
fn canonicalization_error(
    operation: &str,
    detail: String,
) -> OptimizationError {
    OptimizationError::pass_failure(
        PASS_ID,
        format!("{operation}: {detail}"),
    )
}

/// Converts a platform-sized operation count into the optimizer's standardized
/// u64 statistics representation.
///
/// The conversion is checked rather than truncated.
fn checked_u64(
    value: usize,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::pass_failure(
            PASS_ID,
            format!(
                "operation counter conversion overflow: \
                 cannot represent {value} as u64"
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

    use crate::quantum::ir::{
        Gate,
        GateKind,
        Parameter,
        QubitId,
        QuantumCircuit,
    };

    fn single_qubit_circuit(
        gate: Gate,
    ) -> QuantumCircuit {
        QuantumCircuit::from_operations(
            1,
            0,
            vec![gate],
        )
        .expect("test circuit must be valid")
    }

    #[test]
    fn metadata_is_valid() {
        let pass =
            Normalize::new()
                .expect("normalization metadata must be valid");

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
            PassKind::Normalization
        );

        assert_eq!(
            pass.metadata().complexity(),
            PassComplexity::Linear
        );

        assert_eq!(
            pass.metadata().determinism(),
            PassDeterminism::Deterministic
        );

        assert!(
            pass.metadata().semantic_preserving()
        );

        assert!(
            pass.metadata().supports_empty_circuit()
        );

        assert!(
            pass.metadata().supports_large_circuits()
        );

        assert!(
            pass.metadata().fixed_point_safe()
        );
    }

    #[test]
    fn production_policy_is_default() {
        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        assert_eq!(
            pass.policy(),
            CanonicalizationPolicy::production()
        );
    }

    #[test]
    fn empty_circuit_is_canonical() {
        let circuit =
            QuantumCircuit::new(0, 0)
                .expect("empty circuit must be valid");

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let result = pass
            .normalize_circuit(&circuit)
            .expect("empty circuit must normalize");

        assert!(
            result.report().is_unchanged()
        );

        assert_eq!(
            result.circuit().len(),
            0
        );
    }

    #[test]
    fn single_operation_remains_single_operation() {
        let gate = Gate::new(
            GateKind::X,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("X gate must be valid");

        let circuit =
            single_qubit_circuit(gate);

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let result = pass
            .normalize_circuit(&circuit)
            .expect("circuit must normalize");

        assert_eq!(
            result.circuit().len(),
            1
        );

        assert!(
            result.report().is_unchanged()
        );
    }

    #[test]
    fn signed_zero_is_canonicalized() {
        let gate = Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            vec![Parameter::constant(
                -0.0,
            )
            .expect("zero parameter must be valid")],
            None,
            None,
        )
        .expect("RX gate must be valid");

        let circuit =
            single_qubit_circuit(gate);

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let result = pass
            .normalize_circuit(&circuit)
            .expect("circuit must normalize");

        assert_eq!(
            result.circuit().len(),
            1
        );

        assert_eq!(
            result
                .circuit()
                .operations()[0]
                .parameters()
                .len(),
            1
        );

        match result
            .circuit()
            .operations()[0]
            .parameters()[0]
        {
            Parameter::Constant(value) => {
                assert_eq!(value, 0.0);
                assert!(!value.is_sign_negative());
            }
            _ => panic!(
                "normalized constant must remain a constant"
            ),
        }

        assert_eq!(
            result
                .report()
                .signed_zero_normalized(),
            1
        );
    }

    #[test]
    fn normalization_is_idempotent() {
        let gate = Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            vec![Parameter::constant(
                -0.0,
            )
            .expect("zero parameter must be valid")],
            None,
            None,
        )
        .expect("RX gate must be valid");

        let circuit =
            single_qubit_circuit(gate);

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let first =
            pass.normalize_circuit(&circuit)
                .expect("first normalization must succeed");

        let second =
            pass.normalize_circuit(
                first.circuit(),
            )
            .expect("second normalization must succeed");

        assert!(
            second.report().is_unchanged()
        );

        assert_eq!(
            first.circuit(),
            second.circuit()
        );
    }

    #[test]
    fn normalization_does_not_change_operation_count() {
        let gates = vec![
            Gate::new(
                GateKind::X,
                vec![QubitId::new(0)],
                Vec::new(),
                None,
                None,
            )
            .expect("X gate must be valid"),
            Gate::new(
                GateKind::H,
                vec![QubitId::new(1)],
                Vec::new(),
                None,
                None,
            )
            .expect("H gate must be valid"),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                gates,
            )
            .expect("test circuit must be valid");

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let result =
            pass.normalize_circuit(&circuit)
                .expect("normalization must succeed");

        assert_eq!(
            result.circuit().len(),
            circuit.len()
        );
    }

    #[test]
    fn symmetric_gate_operands_can_be_canonicalized() {
        let gate = Gate::new(
            GateKind::CZ,
            vec![
                QubitId::new(1),
                QubitId::new(0),
            ],
            Vec::new(),
            None,
            None,
        )
        .expect("CZ gate must be valid");

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![gate],
            )
            .expect("test circuit must be valid");

        let pass =
            Normalize::new()
                .expect("normalization pass must construct");

        let result =
            pass.normalize_circuit(&circuit)
                .expect("normalization must succeed");

        assert_eq!(
            result
                .circuit()
                .operations()[0]
                .qubits(),
            &[
                QubitId::new(0),
                QubitId::new(1),
            ]
        );

        assert_eq!(
            result
                .report()
                .symmetric_operands_normalized(),
            1
        );
    }

    #[test]
    fn validation_only_policy_preserves_representation() {
        let gate = Gate::new(
            GateKind::CZ,
            vec![
                QubitId::new(1),
                QubitId::new(0),
            ],
            Vec::new(),
            None,
            None,
        )
        .expect("CZ gate must be valid");

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![gate],
            )
            .expect("test circuit must be valid");

        let pass =
            Normalize::with_policy(
                CanonicalizationPolicy::validation_only(),
            )
            .expect("validation-only pass must construct");

        let result =
            pass.normalize_circuit(&circuit)
                .expect("validation-only normalization must succeed");

        assert!(
            result.report().is_unchanged()
        );

        assert_eq!(
            result.circuit(),
            &circuit
        );
    }

    #[test]
    fn canonicalizer_version_is_exposed_separately() {
        assert_eq!(
            Normalize::pass_version(),
            PASS_VERSION
        );

        assert_eq!(
            Normalize::canonicalization_version(),
            super::super::super::canonical::CANONICALIZATION_VERSION
        );
    }
}