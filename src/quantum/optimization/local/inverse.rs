//! Zamani Quantum Optimization — Inverse Cancellation
//!
//! Production-grade local inverse cancellation over Zamani's canonical
//! Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                     crate::quantum::ir::Gate
//!                                │
//!                                ▼
//!                    optimization::operation
//!                                │
//!                                ▼
//!                  local::inverse cancellation
//!                                │
//!                                ▼
//!                    canonical Quantum IR
//! ```
//!
//! This module performs one specific transformation:
//!
//!     U ; U†  -> identity
//!
//! and, where the canonical IR proves the relationship:
//!
//!     U ; U    -> identity
//!
//! for self-inverse operations.
//!
//! Parameterized inverse pairs are recognized only when their inverse
//! relationship can be proven exactly from the canonical parameter structure.
//!
//! # Important architectural rule
//!
//! This file MUST NOT define another quantum gate or circuit representation.
//!
//! The authoritative representations remain:
//!
//! - `crate::quantum::ir::Gate`
//! - `crate::quantum::ir::QuantumCircuit`
//!
//! Optimization-layer semantic classification is provided by:
//!
//! - `crate::quantum::optimization::operation::OperationView`
//! - `crate::quantum::optimization::operation::InverseKind`
//!
//! # What this pass does
//!
//! This pass:
//!
//! - removes adjacent fixed inverse pairs;
//! - removes adjacent self-inverse pairs;
//! - removes adjacent parameterized inverse pairs when exact inversion is
//!   provable;
//! - preserves logical qubit ordering;
//! - preserves all semantic boundaries;
//! - validates supplied gates before optimization;
//! - produces deterministic output;
//! - scales linearly with circuit length;
//! - uses no artificial circuit-size limit;
//! - performs no gate movement;
//! - performs no approximate floating-point equivalence;
//! - performs no backend-specific transformation;
//! - performs no routing;
//! - performs no scheduling;
//! - performs no hardware communication;
//! - performs no execution.
//!
//! # What this pass deliberately does NOT do
//!
//! It does not:
//!
//! - commute gates;
//! - move operations through other operations;
//! - cancel non-adjacent gates;
//! - synthesize U2/U3 inverses;
//! - infer arbitrary matrix equivalence;
//! - use numerical tolerances to prove equality;
//! - cross measurement boundaries;
//! - cross reset boundaries;
//! - cross barriers;
//! - optimize classical control flow;
//! - perform global circuit equivalence checking.
//!
//! Those responsibilities belong to other optimizer layers.
//!
//! # Why adjacent cancellation is sufficient for this pass
//!
//! Consider:
//!
//!     A ; B ; B† ; A†
//!
//! A stack-style left-to-right scan first removes:
//!
//!     B ; B†
//!
//! leaving:
//!
//!     A ; A†
//!
//! which is then removed.
//!
//! Therefore the algorithm reaches the complete fixed point for adjacent
//! inverse cancellation in one linear scan. No repeated full-circuit passes
//! are necessary.
//!
//! # Complexity
//!
//! For `n` operations:
//!
//! - time: O(n);
//! - additional retained output storage: O(n);
//! - comparisons per operation: bounded by the operation's parameter count;
//! - no recursion over circuit size;
//! - no quadratic search;
//! - no e-graph;
//! - no matrix construction.
//!
//! Parameter-expression comparison is bounded by the canonical IR's parameter
//! expression depth limit.
//!
//! # Determinism
//!
//! Given the same canonical input circuit, this module always produces the
//! same output and statistics.
//!
//! There is no randomness, global mutable state, wall-clock dependence, or
//! hash-map iteration involved.
//!
//! # Floating-point semantics
//!
//! This module does NOT use epsilon-based equality.
//!
//! For constants, an inverse relationship is accepted only when:
//!
//!     a + b == 0.0
//!
//! exactly in the represented IEEE-754 values.
//!
//! This is deliberate. A compiler transformation must not turn an approximate
//! numerical coincidence into an exact semantic claim.
//!
//! # Symbolic parameters
//!
//! The following are safe:
//!
//!     RX(theta) ; RX(-theta)
//!     RZ(theta) ; RZ(-theta)
//!
//! when the negation relationship is represented explicitly in the canonical
//! parameter expression.
//!
//! The pass does not attempt arbitrary algebra such as proving:
//!
//!     a + b = 0
//!
//! from unrelated symbolic expressions. That belongs to the parameter
//! simplification/algebra subsystem.
//!
//! # Integration contract
//!
//! ## `quantum::ir::gate`
//!
//! Consumes canonical `Gate` and `GateKind`.
//!
//! ## `quantum::ir::parameter`
//!
//! Uses canonical `Parameter` and `ParameterExpression` without creating a
//! second parameter representation.
//!
//! ## `optimization::operation`
//!
//! Uses `OperationView` and `InverseKind` for semantic classification.
//!
//! ## `optimization::circuit`
//!
//! This module does not bypass the transactional circuit editor. It produces
//! an optimized operation sequence. A higher-level pass can place the result
//! into a `CircuitEditPlan` / `CircuitEditor` transaction.
//!
//! ## `optimization::pass`
//!
//! The pass is intentionally implementable as an optimization operation
//! without requiring this low-level algorithm to own pipeline state.
//!
//! The pipeline remains responsible for:
//!
//! - pass sequencing;
//! - global limits;
//! - analysis invalidation;
//! - verification;
//! - provenance;
//! - statistics aggregation.
//!
//! ## `local::cancellation`
//!
//! `cancellation.rs` may perform broader local algebraic cancellation.
//! This module is the specialized inverse-pair primitive.
//!
//! It must not introduce a competing gate representation.
//!
//! ## `local::commutation`
//!
//! Non-adjacent inverse cancellation requiring movement belongs there.
//!
//! ## `verification`
//!
//! Semantic equivalence verification remains outside this file.
//!
//! # Resource scaling
//!
//! This implementation deliberately does not contain a hard-coded maximum
//! circuit size.
//!
//! The maximum practical circuit size is therefore determined by:
//!
//! - available memory;
//! - `usize` addressability;
//! - the caller's optimizer resource policy;
//! - the canonical IR's own configured limits.
//!
//! The pass itself remains O(n).
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{Parameter, ParameterExpression};

use super::super::operation::{
    inverse_kind,
    InverseKind,
    OperationView,
};

// =============================================================================
// Public identifiers
// =============================================================================

/// Stable identifier for this optimization algorithm.
///
/// This string is intentionally independent of the registry implementation.
/// Registry code may use it as the canonical pass name.
pub const PASS_ID: &str = "quantum.local.inverse_cancellation";

/// Stable human-readable name.
pub const PASS_NAME: &str = "Inverse Cancellation";

/// Algorithm version.
///
/// This is an implementation/provenance identifier, not the compiler version.
pub const ALGORITHM_VERSION: &str = "1";

/// Maximum number of parameters inspected for a single gate.
///
/// The canonical IR already enforces gate parameter arity. This constant is
/// therefore not a circuit-size limit; it exists only as a defensive guard
/// against a future gate kind accidentally exposing an unexpectedly large
/// parameter vector.
pub const MAX_PARAMETERS_PER_GATE: usize = 64;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by inverse cancellation.
///
/// The canonical IR normally guarantees that a `Gate` is valid. This error
/// type exists because optimization code must remain robust when handed gates
/// from deserialization, tests, future IR adapters, or other compiler stages.
#[derive(Debug, Clone, PartialEq)]
pub enum InverseCancellationError {
    /// A supplied gate failed canonical IR validation.
    InvalidGate {
        /// Operation index within the supplied sequence.
        index: usize,

        /// Canonical IR validation failure.
        message: String,
    },

    /// A gate exposed an unexpectedly large parameter list.
    ///
    /// This is a defensive programming invariant rather than a normal
    /// production path.
    ParameterArityExceeded {
        /// Operation index.
        index: usize,

        /// Actual number of parameters.
        actual: usize,

        /// Maximum supported by this pass.
        maximum: usize,
    },

    /// The supplied circuit length cannot be represented in the resulting
    /// vector capacity calculation.
    CapacityOverflow {
        /// Original operation count.
        input_operations: usize,
    },
}

impl fmt::Display for InverseCancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGate { index, message } => {
                write!(
                    formatter,
                    "inverse cancellation received invalid gate at \
                     operation {index}: {message}"
                )
            }

            Self::ParameterArityExceeded {
                index,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "inverse cancellation parameter arity exceeded at \
                     operation {index}: actual {actual}, maximum {maximum}"
                )
            }

            Self::CapacityOverflow {
                input_operations,
            } => {
                write!(
                    formatter,
                    "cannot allocate optimizer output for {input_operations} \
                     input operations because the capacity calculation \
                     overflowed"
                )
            }
        }
    }
}

impl std::error::Error for InverseCancellationError {}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics produced by [`InverseCancellationPass`].
///
/// These counters are local to this pass. The global optimization statistics
/// subsystem may aggregate them into `OptimizationStatistics`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InverseCancellationStatistics {
    /// Number of input operations inspected.
    pub operations_inspected: usize,

    /// Number of adjacent inverse/self-inverse pairs removed.
    pub pairs_cancelled: usize,

    /// Number of operations removed as a consequence of cancellation.
    pub operations_removed: usize,

    /// Number of fixed inverse-name pairs cancelled.
    pub fixed_inverse_pairs: usize,

    /// Number of self-inverse pairs cancelled.
    pub self_inverse_pairs: usize,

    /// Number of parameterized inverse pairs cancelled.
    pub parameterized_inverse_pairs: usize,

    /// Number of candidate adjacent pairs considered.
    pub candidate_pairs_examined: usize,

    /// Returns true when the pass changed the operation sequence.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.operations_removed != 0
    }

    /// Returns the number of operations in the output if the supplied input
    /// operation count is valid.
    ///
    /// Returns `None` only if subtraction would underflow, which cannot happen
    /// for statistics produced by this pass.
    #[must_use]
    pub const fn output_operations(
        self,
        input_operations: usize,
    ) -> Option<usize> {
        input_operations.checked_sub(self.operations_removed)
    }
}

// =============================================================================
// Cancellation reason
// =============================================================================

/// Why a pair of operations was considered exact inverses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationReason {
    /// The first and second operations are both self-inverse.
    SelfInverse,

    /// The operations have canonical fixed inverse kinds such as S/Sdg or
    /// T/Tdg.
    FixedInverse,

    /// The operations are parameterized inverses and their parameters are
    /// exactly negations.
    ParameterizedInverse,
}

impl CancellationReason {
    /// Returns a stable identifier useful for provenance and diagnostics.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfInverse => "self_inverse",
            Self::FixedInverse => "fixed_inverse",
            Self::ParameterizedInverse => "parameterized_inverse",
        }
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// Immutable description of one adjacent inverse-cancellation candidate.
///
/// This type does not own or mutate the gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InverseCandidate {
    /// Index of the first operation.
    pub first_index: usize,

    /// Index of the second operation.
    pub second_index: usize,

    /// Why the operations cancel.
    pub reason: CancellationReason,
}

impl InverseCandidate {
    /// Returns the number of operations removed by this candidate.
    #[must_use]
    pub const fn operations_removed(self) -> usize {
        2
    }
}

// =============================================================================
// Pass configuration
// =============================================================================

/// Configuration for inverse cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InverseCancellationConfig {
    /// Whether self-inverse pairs should be cancelled.
    pub cancel_self_inverse: bool,

    /// Whether fixed named inverse pairs should be cancelled.
    pub cancel_fixed_inverse: bool,

    /// Whether parameterized inverse pairs should be cancelled.
    pub cancel_parameterized_inverse: bool,

    /// Whether the pass validates every input gate.
    ///
    /// This should normally remain enabled at public API boundaries.
    ///
    /// A trusted compiler pipeline may disable repeated validation after a
    /// previous stage has already established the canonical IR invariant.
    pub validate_input: bool,
}

impl Default for InverseCancellationConfig {
    fn default() -> Self {
        Self {
            cancel_self_inverse: true,
            cancel_fixed_inverse: true,
            cancel_parameterized_inverse: true,
            validate_input: true,
        }
    }
}

impl InverseCancellationConfig {
    /// Returns the production default configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            cancel_self_inverse: true,
            cancel_fixed_inverse: true,
            cancel_parameterized_inverse: true,
            validate_input: true,
        }
    }

    /// Returns a configuration suitable for trusted pipelines that already
    /// validated the canonical IR.
    #[must_use]
    pub const fn trusted_ir() -> Self {
        Self {
            cancel_self_inverse: true,
            cancel_fixed_inverse: true,
            cancel_parameterized_inverse: true,
            validate_input: false,
        }
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production inverse-cancellation optimizer.
///
/// The algorithm is a stack-style single scan:
///
/// 1. inspect the next operation;
/// 2. compare it with the last retained operation;
/// 3. if they are exact inverses, remove the retained operation;
/// 4. otherwise retain the new operation.
///
/// This means nested adjacent cancellations naturally collapse without
/// repeated whole-circuit scans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InverseCancellationPass {
    config: InverseCancellationConfig,
}

impl Default for InverseCancellationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl InverseCancellationPass {
    /// Creates a production inverse-cancellation pass.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: InverseCancellationConfig::production(),
        }
    }

    /// Creates a pass with an explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: InverseCancellationConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the pass configuration.
    #[must_use]
    pub const fn config(self) -> InverseCancellationConfig {
        self.config
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub const fn id() -> &'static str {
        PASS_ID
    }

    /// Returns the stable pass name.
    #[must_use]
    pub const fn name() -> &'static str {
        PASS_NAME
    }

    /// Optimizes an operation slice.
    ///
    /// This is the primary low-level API.
    ///
    /// The returned vector contains canonical `Gate` values cloned from the
    /// input except for cancelled operations, which are omitted.
    ///
    /// No canonical IR object is mutated.
    pub fn optimize(
        &self,
        operations: &[Gate],
    ) -> Result<
        (
            Vec<Gate>,
            InverseCancellationStatistics,
        ),
        InverseCancellationError,
    > {
        if self.config.validate_input {
            self.validate_operations(operations)?;
        } else {
            self.validate_parameter_arity(operations)?;
        }

        let mut output = Vec::with_capacity(
            operations
                .len()
                .checked_add(0)
                .ok_or(
                    InverseCancellationError::CapacityOverflow {
                        input_operations: operations.len(),
                    },
                )?,
        );

        let mut statistics = InverseCancellationStatistics {
            operations_inspected: operations.len(),
            ..InverseCancellationStatistics::default()
        };

        for (index, operation) in operations.iter().enumerate() {
            if let Some(previous) = output.last() {
                statistics.candidate_pairs_examined += 1;

                if let Some(reason) = self.classify_pair(
                    previous,
                    operation,
                ) {
                    output.pop();

                    statistics.pairs_cancelled += 1;
                    statistics.operations_removed += 2;

                    match reason {
                        CancellationReason::SelfInverse => {
                            statistics.self_inverse_pairs += 1;
                        }

                        CancellationReason::FixedInverse => {
                            statistics.fixed_inverse_pairs += 1;
                        }

                        CancellationReason::ParameterizedInverse => {
                            statistics.parameterized_inverse_pairs += 1;
                        }
                    }

                    continue;
                }
            }

            // `index` is intentionally consumed by the loop binding because
            // keeping the loop index available makes diagnostics/debugging
            // straightforward if this implementation is extended later.
            let _ = index;

            output.push(operation.clone());
        }

        debug_assert_eq!(
            output.len() + statistics.operations_removed,
            operations.len()
        );

        Ok((output, statistics))
    }

    /// Performs optimization using the trusted-IR configuration.
    ///
    /// This avoids repeating canonical gate validation when the caller has
    /// already validated the complete circuit in the same compiler stage.
    ///
    /// This is still safe Rust; "trusted" refers only to the caller's
    /// established IR invariant.
    pub fn optimize_validated(
        &self,
        operations: &[Gate],
    ) -> Result<
        (
            Vec<Gate>,
            InverseCancellationStatistics,
        ),
        InverseCancellationError,
    > {
        let trusted = Self::with_config(
            InverseCancellationConfig::trusted_ir(),
        );

        trusted.optimize(operations)
    }

    /// Returns whether two adjacent gates are exact inverse candidates.
    ///
    /// This function does not perform any mutation.
    #[must_use]
    pub fn is_inverse_pair(
        &self,
        first: &Gate,
        second: &Gate,
    ) -> bool {
        self.classify_pair(first, second).is_some()
    }

    /// Classifies an adjacent pair without allocating.
    ///
    /// The returned reason is sufficient for statistics and provenance.
    #[must_use]
    pub fn classify_pair(
        &self,
        first: &Gate,
        second: &Gate,
    ) -> Option<CancellationReason> {
        // Both gates must be unitary.
        //
        // This excludes measurement, reset and barrier operations even if a
        // future IR version adds inverse-like metadata to them.
        if !first.is_unitary() || !second.is_unitary() {
            return None;
        }

        // The logical operand sequence is part of the operation semantics.
        //
        // For example:
        //
        // CX(q0,q1)
        //
        // is not interchangeable with:
        //
        // CX(q1,q0)
        //
        // even though both have the same gate kind.
        if first.qubits() != second.qubits() {
            return None;
        }

        // Canonical unitary operations must not carry classical destinations.
        // Keep this check explicit so the optimizer remains conservative if
        // the IR is extended in the future.
        if first.classical_target().is_some()
            || second.classical_target().is_some()
        {
            return None;
        }

        let first_view = OperationView::new(first);
        let second_view = OperationView::new(second);

        let first_inverse = inverse_kind(first);
        let second_inverse = inverse_kind(second);

        // ---------------------------------------------------------------------
        // Self-inverse operations
        // ---------------------------------------------------------------------

        if self.config.cancel_self_inverse
            && first_view.is_self_inverse()
            && second_view.is_self_inverse()
            && first.kind() == second.kind()
            && first.parameters().is_empty()
            && second.parameters().is_empty()
        {
            return Some(CancellationReason::SelfInverse);
        }

        // ---------------------------------------------------------------------
        // Fixed inverse operations
        // ---------------------------------------------------------------------

        if self.config.cancel_fixed_inverse {
            if fixed_inverse_pair(
                first.kind(),
                second.kind(),
                first.parameters(),
                second.parameters(),
                first_inverse,
                second_inverse,
            ) {
                return Some(CancellationReason::FixedInverse);
            }
        }

        // ---------------------------------------------------------------------
        // Parameterized inverse operations
        // ---------------------------------------------------------------------

        if self.config.cancel_parameterized_inverse
            && parameterized_inverse_pair(
                first,
                second,
                first_inverse,
                second_inverse,
            )
        {
            return Some(CancellationReason::ParameterizedInverse);
        }

        None
    }

    // =========================================================================
    // Validation
    // =========================================================================

    fn validate_operations(
        &self,
        operations: &[Gate],
    ) -> Result<(), InverseCancellationError> {
        for (index, operation) in operations.iter().enumerate() {
            operation
                .validate()
                .map_err(|error| {
                    InverseCancellationError::InvalidGate {
                        index,
                        message: error.to_string(),
                    }
                })?;

            if operation.parameters().len()
                > MAX_PARAMETERS_PER_GATE
            {
                return Err(
                    InverseCancellationError::ParameterArityExceeded {
                        index,
                        actual: operation.parameters().len(),
                        maximum: MAX_PARAMETERS_PER_GATE,
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_parameter_arity(
        &self,
        operations: &[Gate],
    ) -> Result<(), InverseCancellationError> {
        for (index, operation) in operations.iter().enumerate() {
            if operation.parameters().len()
                > MAX_PARAMETERS_PER_GATE
            {
                return Err(
                    InverseCancellationError::ParameterArityExceeded {
                        index,
                        actual: operation.parameters().len(),
                        maximum: MAX_PARAMETERS_PER_GATE,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Fixed inverse relationships
// =============================================================================

/// Checks canonical fixed inverse relationships.
///
/// Fixed inverse gates have no parameters. Parameterized gates are deliberately
/// excluded so that this function cannot accidentally treat a parameterized
/// gate as a fixed inverse pair.
fn fixed_inverse_pair(
    first_kind: GateKind,
    second_kind: GateKind,
    first_parameters: &[Parameter],
    second_parameters: &[Parameter],
    first_inverse: InverseKind,
    second_inverse: InverseKind,
) -> bool {
    if !first_parameters.is_empty()
        || !second_parameters.is_empty()
    {
        return false;
    }

    match (first_inverse, second_inverse) {
        (InverseKind::Fixed(a), InverseKind::Fixed(b)) => {
            a == second_kind
                && b == first_kind
                && first_kind != second_kind
        }

        (InverseKind::Fixed(a), InverseKind::SelfInverse) => {
            a == second_kind && first_kind != second_kind
        }

        (InverseKind::SelfInverse, InverseKind::Fixed(b)) => {
            b == first_kind && first_kind != second_kind
        }

        _ => false,
    }
}

// =============================================================================
// Parameterized inverse relationships
// =============================================================================

/// Determines whether two same-kind parameterized gates are exact inverses.
///
/// The operation descriptor declares `NegateParameters` for:
///
/// - RX
/// - RY
/// - RZ
/// - Phase
/// - U1
/// - CRX
/// - CRY
/// - CRZ
///
/// This function additionally requires exact parameter negation.
///
/// U2/U3 are intentionally not handled here because `operation.rs` classifies
/// their inverse as `General`. Their inverse requires parameter algebra and
/// must be handled by synthesis/algebra rather than guessed locally.
fn parameterized_inverse_pair(
    first: &Gate,
    second: &Gate,
    first_inverse: InverseKind,
    second_inverse: InverseKind,
) -> bool {
    if first.kind() != second.kind() {
        return false;
    }

    if !first.kind().is_parameterized()
        || !second.kind().is_parameterized()
    {
        return false;
    }

    if !matches!(
        first_inverse,
        InverseKind::NegateParameters
    ) {
        return false;
    }

    if !matches!(
        second_inverse,
        InverseKind::NegateParameters
    ) {
        return false;
    }

    let first_parameters = first.parameters();
    let second_parameters = second.parameters();

    if first_parameters.len() != second_parameters.len()
        || first_parameters.is_empty()
    {
        return false;
    }

    first_parameters
        .iter()
        .zip(second_parameters.iter())
        .all(|(first, second)| {
            parameters_are_exact_negations(first, second)
        })
}

// =============================================================================
// Exact parameter inversion
// =============================================================================

/// Returns true when two canonical parameters are exact mathematical
/// negatives according to their canonical representation.
///
/// This function is deliberately conservative.
///
/// It recognizes:
///
///     c ; -c
///
/// for concrete constants.
///
/// It also recognizes explicit symbolic negation:
///
///     theta ; -theta
///
/// and structurally identical expression negation:
///
///     expr ; -(expr)
///
/// It does not attempt algebraic normalization such as:
///
///     -(a + b) == -a - b
///
/// unless the canonical parameter representation already makes that identity
/// explicit.
///
/// Such normalization belongs to `parameter::simplification`.
fn parameters_are_exact_negations(
    first: &Parameter,
    second: &Parameter,
) -> bool {
    // -------------------------------------------------------------------------
    // Concrete constants
    // -------------------------------------------------------------------------
    //
    // Exact zero-sum comparison is intentional. No epsilon is used.
    if let (Some(first_value), Some(second_value)) = (
        first.as_constant(),
        second.as_constant(),
    ) {
        return first_value + second_value == 0.0;
    }

    // -------------------------------------------------------------------------
    // Explicit expression negation
    // -------------------------------------------------------------------------
    //
    // first == -(second)
    if let Parameter::Expression(expression) = second {
        if let ParameterExpression::Negate(inner) =
            expression.as_ref()
        {
            if inner.as_ref() == first {
                return true;
            }
        }
    }

    // second == -(first)
    if let Parameter::Expression(expression) = first {
        if let ParameterExpression::Negate(inner) =
            expression.as_ref()
        {
            if inner.as_ref() == second {
                return true;
            }
        }
    }

    // -------------------------------------------------------------------------
    // Identical explicit double-negation forms
    // -------------------------------------------------------------------------
    //
    // This is already covered by the structural comparison above. Keeping
    // this section conceptually explicit makes the intended semantics clear:
    //
    //     -(x) is the exact inverse of x.
    //
    // No simplification is attempted here.

    false
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Optimizes a canonical operation slice using the production configuration.
///
/// This is the simplest API for callers that do not need a persistent pass
/// object.
pub fn optimize(
    operations: &[Gate],
) -> Result<
    (
        Vec<Gate>,
        InverseCancellationStatistics,
    ),
    InverseCancellationError,
> {
    InverseCancellationPass::new().optimize(operations)
}

/// Returns whether two canonical gates are exact adjacent inverse candidates
/// under the production rules.
///
/// This function performs no mutation.
#[must_use]
pub fn is_inverse_pair(
    first: &Gate,
    second: &Gate,
) -> bool {
    InverseCancellationPass::new()
        .is_inverse_pair(first, second)
}

/// Classifies an adjacent inverse pair under the production rules.
///
/// Returns `None` when the operations are not exact adjacent inverses.
#[must_use]
pub fn classify_inverse_pair(
    first: &Gate,
    second: &Gate,
) -> Option<CancellationReason> {
    InverseCancellationPass::new()
        .classify_pair(first, second)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::PI;

    use crate::quantum::ir::gate::Gate;
    use crate::quantum::ir::parameter::{
        Parameter,
        ParameterExpression,
    };
    use crate::quantum::ir::qubits::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn assert_optimized_len(
        operations: &[Gate],
        expected: usize,
    ) {
        let (optimized, _) = optimize(operations)
            .expect("inverse optimization should succeed");

        assert_eq!(optimized.len(), expected);
    }

    // =========================================================================
    // Self-inverse gates
    // =========================================================================

    #[test]
    fn cancels_x_x() {
        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            Gate::x(q(0)).expect("valid X"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_y_y() {
        let circuit = vec![
            Gate::y(q(0)).expect("valid Y"),
            Gate::y(q(0)).expect("valid Y"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_z_z() {
        let circuit = vec![
            Gate::z(q(0)).expect("valid Z"),
            Gate::z(q(0)).expect("valid Z"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_h_h() {
        let circuit = vec![
            Gate::h(q(0)).expect("valid H"),
            Gate::h(q(0)).expect("valid H"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_cx_cx() {
        let circuit = vec![
            Gate::cx(q(0), q(1)).expect("valid CX"),
            Gate::cx(q(0), q(1)).expect("valid CX"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_cy_cy() {
        let circuit = vec![
            Gate::cy(q(0), q(1)).expect("valid CY"),
            Gate::cy(q(0), q(1)).expect("valid CY"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_cz_cz() {
        let circuit = vec![
            Gate::cz(q(0), q(1)).expect("valid CZ"),
            Gate::cz(q(0), q(1)).expect("valid CZ"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_ch_ch() {
        let circuit = vec![
            Gate::ch(q(0), q(1)).expect("valid CH"),
            Gate::ch(q(0), q(1)).expect("valid CH"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_swap_swap() {
        let circuit = vec![
            Gate::swap(q(0), q(1)).expect("valid SWAP"),
            Gate::swap(q(0), q(1)).expect("valid SWAP"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_ccx_ccx() {
        let circuit = vec![
            Gate::ccx(q(0), q(1), q(2))
                .expect("valid CCX"),
            Gate::ccx(q(0), q(1), q(2))
                .expect("valid CCX"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_cswap_cswap() {
        let circuit = vec![
            Gate::cswap(q(0), q(1), q(2))
                .expect("valid CSWAP"),
            Gate::cswap(q(0), q(1), q(2))
                .expect("valid CSWAP"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    // =========================================================================
    // Fixed inverse pairs
    // =========================================================================

    #[test]
    fn cancels_s_sdg() {
        let circuit = vec![
            Gate::s(q(0)).expect("valid S"),
            Gate::sdg(q(0)).expect("valid Sdg"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_sdg_s() {
        let circuit = vec![
            Gate::sdg(q(0)).expect("valid Sdg"),
            Gate::s(q(0)).expect("valid S"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_t_tdg() {
        let circuit = vec![
            Gate::t(q(0)).expect("valid T"),
            Gate::tdg(q(0)).expect("valid Tdg"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_tdg_t() {
        let circuit = vec![
            Gate::tdg(q(0)).expect("valid Tdg"),
            Gate::t(q(0)).expect("valid T"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_v_vdg() {
        let circuit = vec![
            Gate::v(q(0)).expect("valid V"),
            Gate::vdg(q(0)).expect("valid Vdg"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_vdg_v() {
        let circuit = vec![
            Gate::vdg(q(0)).expect("valid Vdg"),
            Gate::v(q(0)).expect("valid V"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    // =========================================================================
    // Parameterized inverse pairs
    // =========================================================================

    #[test]
    fn cancels_rx_theta_rx_negative_theta() {
        let circuit = vec![
            Gate::rx(q(0), PI / 4.0)
                .expect("valid RX"),
            Gate::rx(q(0), -PI / 4.0)
                .expect("valid RX"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_ry_theta_ry_negative_theta() {
        let circuit = vec![
            Gate::ry(q(0), PI / 3.0)
                .expect("valid RY"),
            Gate::ry(q(0), -PI / 3.0)
                .expect("valid RY"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_rz_theta_rz_negative_theta() {
        let circuit = vec![
            Gate::rz(q(0), PI / 5.0)
                .expect("valid RZ"),
            Gate::rz(q(0), -PI / 5.0)
                .expect("valid RZ"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_phase_theta_phase_negative_theta() {
        let circuit = vec![
            Gate::phase(q(0), PI / 7.0)
                .expect("valid phase"),
            Gate::phase(q(0), -PI / 7.0)
                .expect("valid phase"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    #[test]
    fn cancels_u1_theta_u1_negative_theta() {
        let circuit = vec![
            Gate::u1(q(0), PI / 9.0)
                .expect("valid U1"),
            Gate::u1(q(0), -PI / 9.0)
                .expect("valid U1"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    // =========================================================================
    // Symbolic parameters
    // =========================================================================

    #[test]
    fn cancels_symbolic_explicit_negation() {
        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let negative_theta =
            Parameter::expression(
                ParameterExpression::Negate(
                    Box::new(theta.clone()),
                ),
            )
            .expect("valid negation");

        let first = Gate::parameterized(
            GateKind::RX,
            vec![q(0)],
            vec![theta],
        )
        .expect("valid symbolic RX");

        let second = Gate::parameterized(
            GateKind::RX,
            vec![q(0)],
            vec![negative_theta],
        )
        .expect("valid symbolic RX");

        assert!(is_inverse_pair(&first, &second));
    }

    #[test]
    fn does_not_cancel_unrelated_symbols() {
        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let phi =
            Parameter::symbol("phi")
                .expect("valid symbol");

        let first = Gate::parameterized(
            GateKind::RX,
            vec![q(0)],
            vec![theta],
        )
        .expect("valid symbolic RX");

        let second = Gate::parameterized(
            GateKind::RX,
            vec![q(0)],
            vec![phi],
        )
        .expect("valid symbolic RX");

        assert!(!is_inverse_pair(&first, &second));
    }

    // =========================================================================
    // Qubit ordering and independence
    // =========================================================================

    #[test]
    fn does_not_cancel_different_qubits() {
        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            Gate::x(q(1)).expect("valid X"),
        ];

        assert_optimized_len(&circuit, 2);
    }

    #[test]
    fn does_not_cancel_reversed_two_qubit_operands() {
        let circuit = vec![
            Gate::cx(q(0), q(1)).expect("valid CX"),
            Gate::cx(q(1), q(0)).expect("valid CX"),
        ];

        assert_optimized_len(&circuit, 2);
    }

    #[test]
    fn cancels_same_two_qubit_operand_order() {
        let circuit = vec![
            Gate::cx(q(0), q(1)).expect("valid CX"),
            Gate::cx(q(0), q(1)).expect("valid CX"),
        ];

        assert_optimized_len(&circuit, 0);
    }

    // =========================================================================
    // Semantic boundaries
    // =========================================================================

    #[test]
    fn does_not_cancel_across_measurement() {
        let measure = Gate::measure(
            q(0),
            0,
        )
        .expect("valid measurement");

        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            measure,
            Gate::x(q(0)).expect("valid X"),
        ];

        assert_optimized_len(&circuit, 3);
    }

    #[test]
    fn does_not_cancel_across_reset() {
        let reset =
            Gate::reset(q(0)).expect("valid reset");

        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            reset,
            Gate::x(q(0)).expect("valid X"),
        ];

        assert_optimized_len(&circuit, 3);
    }

    #[test]
    fn does_not_cancel_across_barrier() {
        let barrier =
            Gate::barrier(vec![q(0)])
                .expect("valid barrier");

        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            barrier,
            Gate::x(q(0)).expect("valid X"),
        ];

        assert_optimized_len(&circuit, 3);
    }

    // =========================================================================
    // Nested cancellation / fixed point
    // =========================================================================

    #[test]
    fn one_scan_reaches_nested_fixed_point() {
        let circuit = vec![
            Gate::h(q(0)).expect("valid H"),
            Gate::x(q(0)).expect("valid X"),
            Gate::x(q(0)).expect("valid X"),
            Gate::h(q(0)).expect("valid H"),
        ];

        let (optimized, statistics) =
            optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(statistics.pairs_cancelled, 2);
        assert_eq!(statistics.operations_removed, 4);
    }

    #[test]
    fn long_repeated_self_inverse_sequence_is_linear() {
        let mut circuit = Vec::with_capacity(10_000);

        for _ in 0..5_000 {
            circuit.push(
                Gate::x(q(0)).expect("valid X"),
            );
            circuit.push(
                Gate::x(q(0)).expect("valid X"),
            );
        }

        let (optimized, statistics) =
            optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(statistics.pairs_cancelled, 5_000);
        assert_eq!(statistics.operations_removed, 10_000);
    }

    // =========================================================================
    // Non-inverse gates
    // =========================================================================

    #[test]
    fn does_not_cancel_non_inverse_pair() {
        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            Gate::z(q(0)).expect("valid Z"),
        ];

        assert_optimized_len(&circuit, 2);
    }

    #[test]
    fn does_not_cancel_iswap_pair() {
        let circuit = vec![
            Gate::iswap(q(0), q(1))
                .expect("valid iSWAP"),
            Gate::iswap(q(0), q(1))
                .expect("valid iSWAP"),
        ];

        assert_optimized_len(&circuit, 2);
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    #[test]
    fn can_disable_self_inverse_cancellation() {
        let config = InverseCancellationConfig {
            cancel_self_inverse: false,
            ..InverseCancellationConfig::production()
        };

        let pass =
            InverseCancellationPass::with_config(config);

        let circuit = vec![
            Gate::x(q(0)).expect("valid X"),
            Gate::x(q(0)).expect("valid X"),
        ];

        let (optimized, _) =
            pass.optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 2);
    }

    #[test]
    fn can_disable_fixed_inverse_cancellation() {
        let config = InverseCancellationConfig {
            cancel_fixed_inverse: false,
            ..InverseCancellationConfig::production()
        };

        let pass =
            InverseCancellationPass::with_config(config);

        let circuit = vec![
            Gate::s(q(0)).expect("valid S"),
            Gate::sdg(q(0)).expect("valid Sdg"),
        ];

        let (optimized, _) =
            pass.optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 2);
    }

    #[test]
    fn can_disable_parameterized_inverse_cancellation() {
        let config = InverseCancellationConfig {
            cancel_parameterized_inverse: false,
            ..InverseCancellationConfig::production()
        };

        let pass =
            InverseCancellationPass::with_config(config);

        let circuit = vec![
            Gate::rx(q(0), PI / 4.0)
                .expect("valid RX"),
            Gate::rx(q(0), -PI / 4.0)
                .expect("valid RX"),
        ];

        let (optimized, _) =
            pass.optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 2);
    }

    // =========================================================================
    // Exactness
    // =========================================================================

    #[test]
    fn does_not_use_approximate_parameter_equality() {
        let circuit = vec![
            Gate::rx(q(0), 1.0)
                .expect("valid RX"),
            Gate::rx(q(0), -1.0 + 1.0e-15)
                .expect("valid RX"),
        ];

        assert_optimized_len(&circuit, 2);
    }

    // =========================================================================
    // Validation
    // =========================================================================

    #[test]
    fn production_path_validates_input() {
        let valid = Gate::x(q(0))
            .expect("valid gate");

        let pass = InverseCancellationPass::new();

        let result = pass.optimize(&[valid]);

        assert!(result.is_ok());
    }
}