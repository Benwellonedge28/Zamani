//! Zamani Quantum Optimization — Two-Qubit Optimization
//!
//! Production logical two-qubit optimization for Zamani's canonical Quantum
//! IR.
//!
//! # Architectural boundary
//!
//! This module owns logical two-qubit optimization only.
//!
//! It does NOT own:
//!
//! - hardware topology;
//! - logical-to-physical routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - backend execution;
//! - QPU communication;
//! - benchmarking;
//! - QEC semantics;
//! - frontend parsing.
//!
//! The intended compiler direction is:
//!
//! ```text
//! Zamani source / frontend / algorithms
//!                  |
//!                  v
//!             quantum::ir
//!                  |
//!                  v
//!       logical optimization
//!                  |
//!                  v
//!              routing
//!                  |
//!                  v
//!             scheduling
//!                  |
//!                  v
//!          hardware / runtime
//! ```
//!
//! # Canonical IR
//!
//! This pass operates exclusively on `quantum::ir::Gate`.
//!
//! It deliberately does NOT introduce a second `QuantumGate` representation.
//! The existing optimization prototypes use local representations; this
//! production pass is the migration target for the future optimizer framework.
//!
//! # Transformations
//!
//! The pass performs only exact local transformations:
//!
//! - CX CX -> I
//! - CY CY -> I
//! - CZ CZ -> I
//! - CH CH -> I
//! - SWAP SWAP -> I
//! - CRX(a) CRX(b) -> CRX(a+b)
//! - CRY(a) CRY(b) -> CRY(a+b)
//! - CRZ(a) CRZ(b) -> CRZ(a+b)
//! - CR*(a) CR*(-a) -> I
//! - CR*(a) CR*(b) where a+b == 0 mod 2π -> I
//!
//! Operand order is significant.
//!
//! The pass intentionally does NOT assume:
//!
//! - ISWAP is self-inverse;
//! - ECR is self-inverse;
//! - gates may commute merely because they touch the same qubits;
//! - symbolic expressions may be numerically approximated;
//! - global phase may be discarded.
//!
//! Those transformations belong in dedicated algebraic, commutation, phase
//! polynomial, or synthesis passes where their semantic contracts can be
//! explicitly represented and verified.
//!
//! # Scalability
//!
//! The optimizer uses a deterministic stack rewrite:
//!
//! - time: O(n);
//! - additional operation storage: O(n);
//! - no recursive circuit traversal;
//! - no quadratic pair search;
//! - no fixed circuit-size ceiling;
//! - no global mutable state;
//! - no random state;
//! - no backend I/O;
//! - no unsafe code.
//!
//! Therefore tiny circuits and very large circuits are handled using the same
//! algorithm. Actual maximum size is determined by available memory and by
//! higher-level optimizer resource limits.
//!
//! # Integration contract
//!
//! Future `optimization::pass` can adapt this module through
//! [`TwoQubitOptimizationPass`].
//!
//! Future `optimization::pipeline` should call [`TwoQubitOptimizer::optimize`].
//!
//! Future `optimization::statistics` can aggregate
//! [`TwoQubitOptimizationStats`].
//!
//! Future `optimization::targets` should influence whether this pass is
//! selected, but must not be imported by this module.
//!
//! Future `optimization::verification` should verify the input/output circuit
//! pair independently of this pass.
//!
//! This separation means those future files can be implemented without
//! modifying the transformation implementation here.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - no nightly features
//! - no unsafe code

use std::f64::consts::PI;
use std::fmt;

use crate::quantum::ir::{Gate, GateKind, Parameter, QubitId};

/// Default tolerance used only for recognizing an already-normalized
/// floating-point angle as zero.
///
/// This is NOT an approximate circuit-equivalence tolerance.
pub const DEFAULT_ANGLE_TOLERANCE: f64 = 1.0e-12;

/// Largest finite angle for which this pass performs floating-point modulo
/// normalization.
///
/// Refusing to reduce extremely large angles is safer than silently losing
/// significant low-order bits.
pub const MAX_REDUCIBLE_ANGLE: f64 = 9.0e15;

/// Result type for this optimization pass.
pub type TwoQubitResult<T> = Result<T, TwoQubitOptimizationError>;

/// Errors produced by the two-qubit optimizer.
#[derive(Debug, Clone, PartialEq)]
pub enum TwoQubitOptimizationError {
    /// An operation classified as a two-qubit operation does not contain
    /// exactly two operands.
    UnsupportedArity {
        gate: GateKind,
        actual: usize,
    },

    /// A two-qubit operation contains the same logical qubit twice.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// A controlled rotation contains a non-finite angle.
    NonFiniteAngle {
        gate: GateKind,
        angle: f64,
    },

    /// A finite angle is too large for conservative floating-point reduction.
    AngleTooLarge {
        gate: GateKind,
        angle: f64,
    },

    /// The configured numerical tolerance is invalid.
    InvalidTolerance {
        value: f64,
    },

    /// An explicit local operation budget was exceeded.
    OperationBudgetExceeded {
        limit: usize,
        actual: usize,
    },

    /// A replacement could not be represented as valid canonical IR.
    InvalidReplacement {
        gate: GateKind,
        message: String,
    },
}

impl fmt::Display for TwoQubitOptimizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedArity { gate, actual } => {
                write!(
                    formatter,
                    "two-qubit optimizer received {gate:?} with \
                     {actual} operands"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "two-qubit operation contains duplicate qubit {qubit}"
                )
            }

            Self::NonFiniteAngle { gate, angle } => {
                write!(
                    formatter,
                    "two-qubit rotation {gate:?} contains \
                     non-finite angle {angle}"
                )
            }

            Self::AngleTooLarge { gate, angle } => {
                write!(
                    formatter,
                    "two-qubit rotation {gate:?} angle {angle} is too \
                     large for conservative floating-point normalization"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "invalid two-qubit angle tolerance {value}"
                )
            }

            Self::OperationBudgetExceeded { limit, actual } => {
                write!(
                    formatter,
                    "two-qubit optimizer operation budget exceeded: \
                     {actual} > {limit}"
                )
            }

            Self::InvalidReplacement { gate, message } => {
                write!(
                    formatter,
                    "failed to construct optimized {gate:?}: {message}"
                )
            }
        }
    }
}

impl std::error::Error for TwoQubitOptimizationError {}

/// Statistics produced by one invocation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TwoQubitOptimizationStats {
    /// Total number of operations inspected.
    pub operations_inspected: usize,

    /// Number of two-qubit operations inspected.
    pub two_qubit_operations_inspected: usize,

    /// Number of operations removed.
    pub gates_removed: usize,

    /// Number of exact cancellation pairs removed.
    pub pairs_cancelled: usize,

    /// Number of controlled rotations combined.
    pub rotations_combined: usize,

    /// Number of operations emitted.
    pub operations_emitted: usize,
}

impl TwoQubitOptimizationStats {
    /// Returns whether the pass changed the operation sequence.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.gates_removed != 0 || self.rotations_combined != 0
    }
}

/// Result of a two-qubit optimization invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoQubitOptimizationResult {
    /// Optimized canonical operations.
    pub operations: Vec<Gate>,

    /// Statistics describing the transformation.
    pub statistics: TwoQubitOptimizationStats,
}

impl TwoQubitOptimizationResult {
    /// Returns whether optimization changed the input.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.statistics.changed()
    }

    /// Consumes the result and returns the optimized operations.
    #[must_use]
    pub fn into_operations(self) -> Vec<Gate> {
        self.operations
    }
}

/// Configuration owned by this pass.
///
/// `max_operations = None` means that this pass imposes no local circuit-size
/// ceiling. Global limits belong to the future optimization context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoQubitOptimizationConfig {
    /// Optional local operation budget.
    pub max_operations: Option<usize>,

    /// Zero-detection tolerance for normalized finite angles.
    pub angle_tolerance: f64,
}

impl Default for TwoQubitOptimizationConfig {
    fn default() -> Self {
        Self {
            max_operations: None,
            angle_tolerance: DEFAULT_ANGLE_TOLERANCE,
        }
    }
}

impl TwoQubitOptimizationConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_operations: None,
            angle_tolerance: DEFAULT_ANGLE_TOLERANCE,
        }
    }

    /// Sets an optional local operation budget.
    #[must_use]
    pub const fn with_max_operations(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.max_operations = value;
        self
    }

    /// Sets the numerical zero tolerance.
    #[must_use]
    pub const fn with_angle_tolerance(
        mut self,
        value: f64,
    ) -> Self {
        self.angle_tolerance = value;
        self
    }
}

/// Production logical two-qubit optimizer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoQubitOptimizer {
    config: TwoQubitOptimizationConfig,
}

impl Default for TwoQubitOptimizer {
    fn default() -> Self {
        Self {
            config: TwoQubitOptimizationConfig::new(),
        }
    }
}

impl TwoQubitOptimizer {
    /// Creates the default optimizer.
    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }

    /// Creates an optimizer after validating its configuration.
    pub fn try_new(
        config: TwoQubitOptimizationConfig,
    ) -> TwoQubitResult<Self> {
        validate_config(&config)?;
        Ok(Self { config })
    }

    /// Returns the optimizer configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> &TwoQubitOptimizationConfig {
        &self.config
    }

    /// Optimizes a canonical ordered operation sequence.
    ///
    /// Non-two-qubit operations are preserved exactly.
    ///
    /// The stack algorithm naturally reaches a local fixed point in one pass:
    /// when a pair cancels, the preceding operation becomes the new stack
    /// boundary and can immediately participate in the next rewrite.
    pub fn optimize(
        &self,
        operations: &[Gate],
    ) -> TwoQubitResult<TwoQubitOptimizationResult> {
        validate_config(&self.config)?;

        if let Some(limit) = self.config.max_operations {
            if operations.len() > limit {
                return Err(
                    TwoQubitOptimizationError::OperationBudgetExceeded {
                        limit,
                        actual: operations.len(),
                    },
                );
            }
        }

        let mut output = Vec::with_capacity(operations.len());

        let mut statistics =
            TwoQubitOptimizationStats::default();

        for operation in operations {
            statistics.operations_inspected =
                statistics.operations_inspected.saturating_add(1);

            if is_two_qubit_gate(operation.kind()) {
                validate_two_qubit_gate(operation)?;

                statistics.two_qubit_operations_inspected =
                    statistics
                        .two_qubit_operations_inspected
                        .saturating_add(1);
            }

            /*
             * Only the immediate predecessor is considered.
             *
             * This is a critical semantic restriction. A generic optimizer
             * must never move operations through one another without a
             * dependency/commutation proof.
             */
            if let Some(previous) = output.last().cloned() {
                if let Some(rewrite) = try_combine(
                    &previous,
                    operation,
                    self.config.angle_tolerance,
                )? {
                    match rewrite {
                        LocalRewrite::Cancel => {
                            output.pop();

                            statistics.gates_removed =
                                statistics
                                    .gates_removed
                                    .saturating_add(2);

                            statistics.pairs_cancelled =
                                statistics
                                    .pairs_cancelled
                                    .saturating_add(1);

                            continue;
                        }

                        LocalRewrite::Replace(replacement) => {
                            output.pop();
                            output.push(replacement);

                            statistics.gates_removed =
                                statistics
                                    .gates_removed
                                    .saturating_add(1);

                            statistics.rotations_combined =
                                statistics
                                    .rotations_combined
                                    .saturating_add(1);

                            continue;
                        }
                    }
                }
            }

            output.push(operation.clone());
        }

        statistics.operations_emitted = output.len();

        Ok(TwoQubitOptimizationResult {
            operations: output,
            statistics,
        })
    }

    /// Optimizes and returns only the resulting operation sequence.
    pub fn optimize_operations(
        &self,
        operations: &[Gate],
    ) -> TwoQubitResult<Vec<Gate>> {
        Ok(self.optimize(operations)?.operations)
    }

    /// Determines whether the two operations have a local exact rewrite.
    pub fn can_combine(
        &self,
        first: &Gate,
        second: &Gate,
    ) -> TwoQubitResult<bool> {
        if !is_two_qubit_gate(first.kind())
            || !is_two_qubit_gate(second.kind())
        {
            return Ok(false);
        }

        validate_two_qubit_gate(first)?;
        validate_two_qubit_gate(second)?;

        Ok(
            try_combine(
                first,
                second,
                self.config.angle_tolerance,
            )?
            .is_some(),
        )
    }
}

/// Adapter contract for the future generic optimization pass registry.
///
/// `optimization::pass` can later wrap this trait in the repository-wide
/// `OptimizationPass` abstraction without modifying the actual transformation
/// implementation.
pub trait TwoQubitOptimizationPass {
    /// Runs the pass over canonical operations.
    fn run(
        &self,
        operations: &[Gate],
    ) -> TwoQubitResult<TwoQubitOptimizationResult>;
}

impl TwoQubitOptimizationPass for TwoQubitOptimizer {
    fn run(
        &self,
        operations: &[Gate],
    ) -> TwoQubitResult<TwoQubitOptimizationResult> {
        self.optimize(operations)
    }
}

enum LocalRewrite {
    Cancel,
    Replace(Gate),
}

/// Returns whether the operation is a two-qubit operation owned by this pass.
#[must_use]
const fn is_two_qubit_gate(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::ECR
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
    )
}

/// Exact self-inverse two-qubit operations.
///
/// ISWAP is deliberately excluded because:
///
/// `ISWAP * ISWAP = SWAP`
///
/// rather than identity.
///
/// ECR is deliberately excluded because its exact phase/decomposition
/// semantics should be handled by a dedicated algebraic/native-gate pass.
#[must_use]
const fn is_self_inverse(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
    )
}

/// Parameterized controlled rotations handled by this pass.
#[must_use]
const fn is_controlled_rotation(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::CRX | GateKind::CRY | GateKind::CRZ
    )
}

/// Operand order must match exactly.
///
/// For controlled operations:
///
/// `CX(control, target)`
///
/// is not interchangeable with:
///
/// `CX(target, control)`.
#[must_use]
fn same_ordered_qubits(
    first: &Gate,
    second: &Gate,
) -> bool {
    first.qubits() == second.qubits()
}

/// Attempts one exact local transformation.
fn try_combine(
    first: &Gate,
    second: &Gate,
    tolerance: f64,
) -> TwoQubitResult<Option<LocalRewrite>> {
    if !is_two_qubit_gate(first.kind())
        || !is_two_qubit_gate(second.kind())
    {
        return Ok(None);
    }

    if !same_ordered_qubits(first, second) {
        return Ok(None);
    }

    /*
     * Exact self-inverse cancellation.
     */
    if first.kind() == second.kind()
        && is_self_inverse(first.kind())
    {
        return Ok(Some(LocalRewrite::Cancel));
    }

    /*
     * Same controlled rotation family:
     *
     * CRX(a) CRX(b) -> CRX(a+b)
     *
     * and similarly for CRY / CRZ.
     */
    if first.kind() == second.kind()
        && is_controlled_rotation(first.kind())
    {
        return combine_controlled_rotations(
            first,
            second,
            tolerance,
        );
    }

    Ok(None)
}

/// Combines two adjacent equal controlled rotations when both parameters are
/// finite constants.
///
/// Symbolic parameters are deliberately left unchanged. The future symbolic
/// parameter optimizer can perform expression-aware transformations using the
/// canonical parameter algebra.
fn combine_controlled_rotations(
    first: &Gate,
    second: &Gate,
    tolerance: f64,
) -> TwoQubitResult<Option<LocalRewrite>> {
    let Some(first_angle) = constant_angle(first) else {
        return Ok(None);
    };

    let Some(second_angle) = constant_angle(second) else {
        return Ok(None);
    };

    validate_angle(first.kind(), first_angle)?;
    validate_angle(second.kind(), second_angle)?;

    let combined = first_angle + second_angle;

    if !combined.is_finite() {
        return Err(
            TwoQubitOptimizationError::NonFiniteAngle {
                gate: first.kind(),
                angle: combined,
            },
        );
    }

    if combined.abs() > MAX_REDUCIBLE_ANGLE {
        return Err(
            TwoQubitOptimizationError::AngleTooLarge {
                gate: first.kind(),
                angle: combined,
            },
        );
    }

    let normalized = normalize_angle(combined);

    /*
     * The combined operation is identity.
     */
    if normalized.abs() <= tolerance {
        return Ok(Some(LocalRewrite::Cancel));
    }

    let parameter =
        Parameter::constant(normalized).map_err(|error| {
            TwoQubitOptimizationError::InvalidReplacement {
                gate: first.kind(),
                message: error.to_string(),
            }
        })?;

    /*
     * Construct through the canonical IR constructor.
     *
     * No unchecked construction and no direct mutation of private Gate state
     * is performed.
     */
    let replacement = Gate::new(
        first.kind(),
        first.qubits().to_vec(),
        vec![parameter],
        first.classical_target(),
        first.measurement().cloned(),
    )
    .map_err(|error| {
        TwoQubitOptimizationError::InvalidReplacement {
            gate: first.kind(),
            message: error.to_string(),
        }
    })?;

    Ok(Some(LocalRewrite::Replace(replacement)))
}

/// Returns a finite constant angle when the operation contains exactly one
/// constant parameter.
fn constant_angle(gate: &Gate) -> Option<f64> {
    if gate.parameters().len() != 1 {
        return None;
    }

    match &gate.parameters()[0] {
        Parameter::Constant(value) => Some(*value),
        Parameter::Symbol(_) | Parameter::Expression(_) => None,
    }
}

/// Validates the local structure required by this pass.
fn validate_two_qubit_gate(
    gate: &Gate,
) -> TwoQubitResult<()> {
    if gate.qubits().len() != 2 {
        return Err(
            TwoQubitOptimizationError::UnsupportedArity {
                gate: gate.kind(),
                actual: gate.qubits().len(),
            },
        );
    }

    if gate.qubits()[0] == gate.qubits()[1] {
        return Err(
            TwoQubitOptimizationError::DuplicateQubit {
                qubit: gate.qubits()[0],
            },
        );
    }

    if is_controlled_rotation(gate.kind()) {
        if let Some(angle) = constant_angle(gate) {
            validate_angle(gate.kind(), angle)?;
        }
    }

    Ok(())
}

/// Validates a rotation angle before numerical manipulation.
fn validate_angle(
    kind: GateKind,
    angle: f64,
) -> TwoQubitResult<()> {
    if !angle.is_finite() {
        return Err(
            TwoQubitOptimizationError::NonFiniteAngle {
                gate: kind,
                angle,
            },
        );
    }

    if angle.abs() > MAX_REDUCIBLE_ANGLE {
        return Err(
            TwoQubitOptimizationError::AngleTooLarge {
                gate: kind,
                angle,
            },
        );
    }

    Ok(())
}

/// Validates configuration.
fn validate_config(
    config: &TwoQubitOptimizationConfig,
) -> TwoQubitResult<()> {
    if !config.angle_tolerance.is_finite()
        || config.angle_tolerance < 0.0
    {
        return Err(
            TwoQubitOptimizationError::InvalidTolerance {
                value: config.angle_tolerance,
            },
        );
    }

    Ok(())
}

/// Normalizes a finite angle into approximately `[-π, π]`.
#[must_use]
fn normalize_angle(angle: f64) -> f64 {
    let mut value = angle % (2.0 * PI);

    if value > PI {
        value -= 2.0 * PI;
    } else if value < -PI {
        value += 2.0 * PI;
    }

    /*
     * Normalize negative zero so the canonical IR never receives `-0.0`
     * from this pass.
     */
    if value == -0.0 {
        0.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn two_qubit_gate(kind: GateKind) -> Gate {
        Gate::new(
            kind,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        )
        .expect("valid two-qubit test gate")
    }

    fn controlled_rotation(
        kind: GateKind,
        angle: f64,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(0), q(1)],
            vec![
                Parameter::constant(angle)
                    .expect("finite test angle"),
            ],
            None,
            None,
        )
        .expect("valid controlled rotation")
    }

    #[test]
    fn cancels_cx_pair() {
        let optimizer = TwoQubitOptimizer::new();

        let result = optimizer
            .optimize(&[
                two_qubit_gate(GateKind::CX),
                two_qubit_gate(GateKind::CX),
            ])
            .expect("optimization succeeds");

        assert!(result.operations.is_empty());
        assert_eq!(result.statistics.pairs_cancelled, 1);
        assert_eq!(result.statistics.gates_removed, 2);
    }

    #[test]
    fn cancels_all_supported_self_inverse_pairs() {
        for kind in [
            GateKind::CX,
            GateKind::CY,
            GateKind::CZ,
            GateKind::CH,
            GateKind::SWAP,
        ] {
            let result = TwoQubitOptimizer::new()
                .optimize(&[
                    two_qubit_gate(kind),
                    two_qubit_gate(kind),
                ])
                .expect("optimization succeeds");

            assert!(
                result.operations.is_empty(),
                "{kind:?} did not cancel"
            );
        }
    }

    #[test]
    fn does_not_cancel_reversed_control_target_order() {
        let first = Gate::new(
            GateKind::CX,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        )
        .expect("valid gate");

        let second = Gate::new(
            GateKind::CX,
            vec![q(1), q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("valid gate");

        let result = TwoQubitOptimizer::new()
            .optimize(&[
                first.clone(),
                second.clone(),
            ])
            .expect("optimization succeeds");

        assert_eq!(
            result.operations,
            vec![first, second]
        );
    }

    #[test]
    fn does_not_cancel_iswap_pair() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                two_qubit_gate(GateKind::ISWAP),
                two_qubit_gate(GateKind::ISWAP),
            ])
            .expect("optimization succeeds");

        assert_eq!(result.operations.len(), 2);
        assert_eq!(
            result.statistics.pairs_cancelled,
            0
        );
    }

    #[test]
    fn does_not_cancel_ecr_pair_without_explicit_rule() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                two_qubit_gate(GateKind::ECR),
                two_qubit_gate(GateKind::ECR),
            ])
            .expect("optimization succeeds");

        assert_eq!(result.operations.len(), 2);
        assert_eq!(
            result.statistics.pairs_cancelled,
            0
        );
    }

    #[test]
    fn combines_controlled_rz_constants() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                controlled_rotation(
                    GateKind::CRZ,
                    0.25,
                ),
                controlled_rotation(
                    GateKind::CRZ,
                    0.75,
                ),
            ])
            .expect("optimization succeeds");

        assert_eq!(result.operations.len(), 1);
        assert_eq!(
            result.statistics.rotations_combined,
            1
        );
        assert_eq!(
            constant_angle(&result.operations[0]),
            Some(1.0)
        );
    }

    #[test]
    fn combines_each_controlled_rotation_family() {
        for kind in [
            GateKind::CRX,
            GateKind::CRY,
            GateKind::CRZ,
        ] {
            let result = TwoQubitOptimizer::new()
                .optimize(&[
                    controlled_rotation(kind, 0.5),
                    controlled_rotation(kind, 0.5),
                ])
                .expect("optimization succeeds");

            assert_eq!(result.operations.len(), 1);
            assert_eq!(
                constant_angle(&result.operations[0]),
                Some(1.0)
            );
        }
    }

    #[test]
    fn removes_inverse_controlled_rotation() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                controlled_rotation(
                    GateKind::CRX,
                    1.0,
                ),
                controlled_rotation(
                    GateKind::CRX,
                    -1.0,
                ),
            ])
            .expect("optimization succeeds");

        assert!(result.operations.is_empty());
        assert_eq!(
            result.statistics.pairs_cancelled,
            1
        );
    }

    #[test]
    fn removes_full_two_pi_controlled_rotation() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                controlled_rotation(
                    GateKind::CRZ,
                    PI,
                ),
                controlled_rotation(
                    GateKind::CRZ,
                    PI,
                ),
            ])
            .expect("optimization succeeds");

        assert!(result.operations.is_empty());
    }

    #[test]
    fn preserves_symbolic_rotation() {
        let symbolic = Gate::new(
            GateKind::CRZ,
            vec![q(0), q(1)],
            vec![
                Parameter::symbol("theta")
                    .expect("valid symbol"),
            ],
            None,
            None,
        )
        .expect("valid symbolic gate");

        let result = TwoQubitOptimizer::new()
            .optimize(&[
                symbolic.clone(),
                symbolic.clone(),
            ])
            .expect("optimization succeeds");

        assert_eq!(
            result.operations,
            vec![symbolic.clone(), symbolic]
        );
    }

    #[test]
    fn preserves_non_two_qubit_operations() {
        let h = Gate::new(
            GateKind::H,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("valid single-qubit gate");

        let result = TwoQubitOptimizer::new()
            .optimize(std::slice::from_ref(&h))
            .expect("optimization succeeds");

        assert_eq!(result.operations, vec![h]);
    }

    #[test]
    fn does_not_cross_measurement() {
        let first =
            two_qubit_gate(GateKind::CX);

        let measurement = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("valid measurement");

        let second =
            two_qubit_gate(GateKind::CX);

        let result = TwoQubitOptimizer::new()
            .optimize(&[
                first.clone(),
                measurement.clone(),
                second.clone(),
            ])
            .expect("optimization succeeds");

        assert_eq!(
            result.operations,
            vec![first, measurement, second]
        );
    }

    #[test]
    fn does_not_cross_unrelated_operation() {
        let first =
            two_qubit_gate(GateKind::CX);

        let middle =
            two_qubit_gate(GateKind::CZ);

        let second =
            two_qubit_gate(GateKind::CX);

        let result = TwoQubitOptimizer::new()
            .optimize(&[
                first.clone(),
                middle.clone(),
                second.clone(),
            ])
            .expect("optimization succeeds");

        assert_eq!(
            result.operations,
            vec![first, middle, second]
        );
    }

    #[test]
    fn stack_rewrite_reaches_fixed_point_in_one_pass() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                two_qubit_gate(GateKind::CX),
                two_qubit_gate(GateKind::CX),
                two_qubit_gate(GateKind::CZ),
                two_qubit_gate(GateKind::CZ),
            ])
            .expect("optimization succeeds");

        assert!(result.operations.is_empty());
        assert_eq!(
            result.statistics.pairs_cancelled,
            2
        );
    }

    #[test]
    fn optimizer_is_idempotent() {
        let input = vec![
            controlled_rotation(
                GateKind::CRZ,
                0.5,
            ),
            controlled_rotation(
                GateKind::CRZ,
                0.5,
            ),
            two_qubit_gate(GateKind::CX),
            two_qubit_gate(GateKind::CX),
        ];

        let optimizer =
            TwoQubitOptimizer::new();

        let first = optimizer
            .optimize(&input)
            .expect("first optimization succeeds");

        let second = optimizer
            .optimize(&first.operations)
            .expect("second optimization succeeds");

        assert_eq!(
            first.operations,
            second.operations
        );
    }

    #[test]
    fn rejects_invalid_tolerance() {
        let config =
            TwoQubitOptimizationConfig::new()
                .with_angle_tolerance(f64::NAN);

        assert!(
            TwoQubitOptimizer::try_new(config)
                .is_err()
        );
    }

    #[test]
    fn rejects_negative_tolerance() {
        let config =
            TwoQubitOptimizationConfig::new()
                .with_angle_tolerance(-1.0);

        assert!(
            TwoQubitOptimizer::try_new(config)
                .is_err()
        );
    }

    #[test]
    fn enforces_optional_operation_budget() {
        let config =
            TwoQubitOptimizationConfig::new()
                .with_max_operations(Some(1));

        let optimizer =
            TwoQubitOptimizer::try_new(config)
                .expect("configuration is valid");

        let input = vec![
            two_qubit_gate(GateKind::CX),
            two_qubit_gate(GateKind::CX),
        ];

        assert!(matches!(
            optimizer.optimize(&input),
            Err(
                TwoQubitOptimizationError::OperationBudgetExceeded {
                    limit: 1,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn can_combine_reports_exact_local_rules() {
        let optimizer =
            TwoQubitOptimizer::new();

        let cx =
            two_qubit_gate(GateKind::CX);

        assert!(
            optimizer
                .can_combine(&cx, &cx)
                .expect("query succeeds")
        );
    }

    #[test]
    fn can_combine_rejects_non_two_qubit_operations() {
        let optimizer =
            TwoQubitOptimizer::new();

        let h = Gate::new(
            GateKind::H,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("valid gate");

        assert!(
            !optimizer
                .can_combine(&h, &h)
                .expect("query succeeds")
        );
    }

    #[test]
    fn statistics_are_consistent() {
        let result = TwoQubitOptimizer::new()
            .optimize(&[
                two_qubit_gate(GateKind::CX),
                two_qubit_gate(GateKind::CX),
                two_qubit_gate(GateKind::CZ),
            ])
            .expect("optimization succeeds");

        assert_eq!(
            result.statistics.operations_inspected,
            3
        );

        assert_eq!(
            result.statistics.two_qubit_operations_inspected,
            3
        );

        assert_eq!(
            result.statistics.gates_removed,
            2
        );

        assert_eq!(
            result.statistics.operations_emitted,
            result.operations.len()
        );
    }
}