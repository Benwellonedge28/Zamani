//! Zamani Quantum Optimization — Two-Qubit Synthesis
//!
//! Production-grade logical two-qubit gate synthesis for Zamani's canonical
//! Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::Gate
//!                                │
//!                                ▼
//!                 optimization::synthesis::two_qubit
//!                                │
//!              ┌─────────────────┼──────────────────┐
//!              │                 │                  │
//!              ▼                 ▼                  ▼
//!        target capabilities   exact rules      resource budget
//!              │                 │                  │
//!              └─────────────────┼──────────────────┘
//!                                ▼
//!                         canonical Gate list
//!                                │
//!                                ▼
//!                         routing / scheduling
//! ```
//!
//! This module owns:
//!
//! - exact logical two-qubit gate decomposition;
//! - target-independent decomposition identities;
//! - target capability contracts;
//! - deterministic synthesis selection;
//! - synthesis resource accounting;
//! - bounded output generation;
//! - validation of generated gates.
//!
//! This module does NOT own:
//!
//! - hardware topology;
//! - physical qubit mapping;
//! - routing;
//! - pulse schedules;
//! - calibration;
//! - backend execution;
//! - QPU communication;
//! - circuit-wide optimization;
//! - matrix simulation;
//! - measurement;
//! - error correction;
//! - source parsing.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Canonical IR rule
//!
//! This file deliberately does not define another `QuantumGate`, circuit type,
//! qubit type, or parameter type.
//!
//! The authoritative representations remain:
//!
//! - `crate::quantum::ir::Gate`
//! - `crate::quantum::ir::GateKind`
//! - `crate::quantum::ir::Parameter`
//! - `crate::quantum::ir::QubitId`
//!
//! # Supported exact decompositions
//!
//! The following transformations are implemented exactly:
//!
//! ```text
//! CX    -> CX
//! CZ    -> CZ
//! CY    -> Sdg ; CX ; S
//! SWAP  -> CX ; CX ; CX
//! CRX   -> H ; CRZ ; H
//! CRY   -> RY(θ/2) ; CX ; RY(-θ/2) ; CX
//! CRZ   -> RZ(θ/2) ; CX ; RZ(-θ/2) ; CX
//! ```
//!
//! Direct native operations are always preferred when the target declares
//! them supported.
//!
//! `ISWAP`, `ECR`, and `CH` are deliberately not approximated or replaced by
//! an unverified identity. They are retained when native support exists and
//! otherwise return a structured `UnsupportedDecomposition` error.
//!
//! This is a safety property: an optimizer must fail explicitly rather than
//! silently emit a semantically different circuit.
//!
//! # Parameter preservation
//!
//! Controlled rotations support symbolic parameters.
//!
//! For example:
//!
//! ```text
//! CRZ(theta)
//! ```
//!
//! becomes:
//!
//! ```text
//! RZ(theta / 2)
//! CX
//! RZ(-theta / 2)
//! CX
//! ```
//!
//! No numerical binding is required.
//!
//! # Resource scaling
//!
//! There is deliberately no hard-coded circuit-size limit.
//!
//! Synthesis scales with the size of the supplied operation and its generated
//! replacement sequence. The caller controls the maximum generated operation
//! count through [`SynthesisBudget`].
//!
//! An unlimited budget is represented explicitly by `None`.
//!
//! This means practical scale is determined by:
//!
//! - available memory;
//! - `usize` addressability;
//! - canonical IR limits;
//! - caller-selected synthesis limits.
//!
//! # Determinism
//!
//! Synthesis is deterministic:
//!
//! - no randomness;
//! - no global state;
//! - no wall-clock decisions;
//! - no hash-map iteration;
//! - no backend I/O;
//! - no floating-point tolerance-based identity decisions.
//!
//! # Numerical policy
//!
//! This file does not use epsilon equality to prove equivalence.
//!
//! Parameter transformations are structural. Constant parameters are divided
//! exactly according to the IR's floating-point representation and symbolic
//! parameters are represented as canonical parameter expressions.
//!
//! # Global phase
//!
//! The exact decompositions in this file preserve the logical operation rather
//! than deliberately dropping global phase.
//!
//! This is especially important for controlled operations, where an apparently
//! global phase of an uncontrolled gate can become a relative phase when the
//! gate is controlled.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes the canonical IR directly.
//!
//! ## `optimization::targets`
//!
//! Future target implementations can implement [`TwoQubitSynthesisTarget`].
//!
//! This file does not need to be changed when new target implementations are
//! introduced.
//!
//! ## `optimization::cost`
//!
//! The returned [`SynthesisReport`] exposes exact generated operation counts.
//! The common optimization cost model can consume these values.
//!
//! ## `optimization::pass`
//!
//! A future synthesis optimization pass can invoke [`synthesize_gate`] for each
//! two-qubit operation without coupling this low-level algorithm to pipeline
//! state.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline remains responsible for:
//!
//! - pass ordering;
//! - global optimization limits;
//! - analysis invalidation;
//! - provenance;
//! - verification;
//! - circuit replacement.
//!
//! ## `routing`
//!
//! Routing must occur after logical synthesis unless the optimizer deliberately
//! performs a target-aware synthesis stage. This module itself never moves
//! logical qubits.
//!
//! ## `verification`
//!
//! Semantic verification remains outside this file. Every decomposition here is
//! an exact algebraic identity, but the global verification subsystem should
//! still be able to independently verify optimizer output.
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
//! No unsafe code is used or required.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{Parameter, ParameterExpression};
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result types
// =============================================================================

/// Result type returned by two-qubit synthesis.
pub type TwoQubitSynthesisResult<T> = Result<T, TwoQubitSynthesisError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by two-qubit synthesis.
///
/// The error vocabulary is intentionally local to the synthesis algorithm.
/// The higher-level optimization error layer can wrap this type without this
/// file needing to depend on the eventual optimizer-wide error hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub enum TwoQubitSynthesisError {
    /// The supplied operation is not a two-qubit operation.
    NotTwoQubit {
        /// Operation kind received.
        gate: GateKind,

        /// Number of operands received.
        operands: usize,
    },

    /// The operation contains duplicate operands.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// The target does not support a required single-qubit operation.
    MissingSingleQubitCapability {
        /// Required operation.
        gate: GateKind,
    },

    /// The target does not support a required two-qubit operation.
    MissingTwoQubitCapability {
        /// Required operation.
        gate: GateKind,
    },

    /// No exact decomposition is currently provided for this operation.
    UnsupportedDecomposition {
        /// Gate that cannot be decomposed.
        gate: GateKind,
    },

    /// A required parameter is missing.
    MissingParameter {
        /// Gate requiring the parameter.
        gate: GateKind,

        /// Parameter index.
        index: usize,
    },

    /// A parameter transformation failed.
    InvalidParameter {
        /// Gate whose parameter was transformed.
        gate: GateKind,

        /// Parameter index.
        index: usize,
    },

    /// The synthesis budget would be exceeded.
    BudgetExceeded {
        /// Maximum generated operations.
        maximum: usize,

        /// Number required by the candidate.
        required: usize,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        operation: &'static str,
    },

    /// A generated gate failed canonical IR construction.
    InvalidGeneratedGate {
        /// Gate kind being generated.
        gate: GateKind,

        /// Human-readable construction error.
        message: String,
    },

    /// A supplied gate failed canonical validation.
    InvalidInputGate {
        /// Gate kind.
        gate: GateKind,

        /// Human-readable validation error.
        message: String,
    },

    /// A target capability contract is inconsistent.
    InvalidTarget {
        /// Static explanation.
        message: &'static str,
    },
}

impl fmt::Display for TwoQubitSynthesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotTwoQubit { gate, operands } => {
                write!(
                    f,
                    "gate {gate:?} is not a valid two-qubit operation: \
                     received {operands} operands"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "two-qubit operation contains duplicate logical qubit {qubit}"
                )
            }

            Self::MissingSingleQubitCapability { gate } => {
                write!(
                    f,
                    "target does not support required single-qubit gate {gate:?}"
                )
            }

            Self::MissingTwoQubitCapability { gate } => {
                write!(
                    f,
                    "target does not support required two-qubit gate {gate:?}"
                )
            }

            Self::UnsupportedDecomposition { gate } => {
                write!(
                    f,
                    "no exact two-qubit decomposition is registered for {gate:?}"
                )
            }

            Self::MissingParameter { gate, index } => {
                write!(
                    f,
                    "gate {gate:?} is missing required parameter {index}"
                )
            }

            Self::InvalidParameter { gate, index } => {
                write!(
                    f,
                    "parameter {index} of gate {gate:?} cannot be transformed"
                )
            }

            Self::BudgetExceeded {
                maximum,
                required,
            } => {
                write!(
                    f,
                    "two-qubit synthesis budget exceeded: \
                     maximum {maximum}, required {required}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "arithmetic overflow while calculating {operation}"
                )
            }

            Self::InvalidGeneratedGate { gate, message } => {
                write!(
                    f,
                    "generated gate {gate:?} failed canonical IR validation: {message}"
                )
            }

            Self::InvalidInputGate { gate, message } => {
                write!(
                    f,
                    "input gate {gate:?} failed canonical IR validation: {message}"
                )
            }

            Self::InvalidTarget { message } => {
                write!(f, "invalid two-qubit synthesis target: {message}")
            }
        }
    }
}

impl Error for TwoQubitSynthesisError {}

// =============================================================================
// Target capability contract
// =============================================================================

/// Describes the operations available to a two-qubit synthesis target.
///
/// This trait intentionally describes *capabilities*, not hardware topology.
///
/// A future `optimization::targets::target::OptimizationTarget` can implement
/// this trait without modifying this file.
///
/// The target must report capabilities for both single- and two-qubit gates
/// because a decomposition can replace one two-qubit gate with several
/// single-qubit gates plus an entangler.
pub trait TwoQubitSynthesisTarget {
    /// Returns whether the target accepts the supplied operation kind.
    fn supports(&self, gate: GateKind) -> bool;

    /// Returns the maximum number of generated operations, if bounded.
    ///
    /// `None` means the target imposes no additional synthesis-output limit.
    fn max_generated_operations(&self) -> Option<usize> {
        None
    }
}

/// A simple standard basis for tests, simulators, generic compilation, and
/// callers that do not yet have a full `OptimizationTarget`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoQubitBasis {
    /// CNOT/CX plus the single-qubit gates required by exact decompositions.
    Cnot,

    /// CZ plus the single-qubit gates required by exact decompositions.
    Cz,

    /// Native SWAP.
    Swap,

    /// Native iSWAP.
    ISwap,

    /// Native ECR.
    Ecr,

    /// A fully explicit native set.
    ///
    /// This variant allows callers to provide a static capability predicate
    /// through [`ExplicitTwoQubitBasis`].
    Explicit,
}

/// Explicit static capability set.
///
/// This is intentionally small and deterministic. Larger target descriptions
/// belong in `optimization::targets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplicitTwoQubitBasis {
    /// Native two-qubit gates.
    pub native_two_qubit: &'static [GateKind],

    /// Native single-qubit gates.
    pub native_single_qubit: &'static [GateKind],

    /// Optional output operation limit.
    pub max_generated_operations: Option<usize>,
}

impl TwoQubitSynthesisTarget for TwoQubitBasis {
    fn supports(&self, gate: GateKind) -> bool {
        match self {
            Self::Cnot => supports_cnot_basis(*gate),
            Self::Cz => supports_cz_basis(*gate),
            Self::Swap => supports_swap_basis(*gate),
            Self::ISwap => supports_iswap_basis(*gate),
            Self::Ecr => supports_ecr_basis(*gate),
            Self::Explicit => false,
        }
    }
}

impl TwoQubitSynthesisTarget for ExplicitTwoQubitBasis {
    fn supports(&self, gate: GateKind) -> bool {
        self.native_two_qubit
            .iter()
            .chain(self.native_single_qubit.iter())
            .any(|candidate| *candidate == gate)
    }

    fn max_generated_operations(&self) -> Option<usize> {
        self.max_generated_operations
    }
}

// =============================================================================
// Budget
// =============================================================================

/// Resource budget for one synthesis operation.
///
/// No hard-coded maximum exists in this module.
///
/// `None` means unlimited from the synthesis layer's perspective. The canonical
/// IR and higher-level optimization limits remain authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynthesisBudget {
    /// Maximum number of generated operations.
    ///
    /// `None` means unlimited.
    pub max_operations: Option<usize>,
}

impl SynthesisBudget {
    /// Creates an unlimited synthesis budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_operations: None,
        }
    }

    /// Creates a bounded synthesis budget.
    #[must_use]
    pub const fn bounded(max_operations: usize) -> Self {
        Self {
            max_operations: Some(max_operations),
        }
    }

    fn check(
        self,
        required: usize,
    ) -> TwoQubitSynthesisResult<()> {
        match self.max_operations {
            Some(maximum) if required > maximum => {
                Err(TwoQubitSynthesisError::BudgetExceeded {
                    maximum,
                    required,
                })
            }

            _ => Ok(()),
        }
    }
}

// =============================================================================
// Synthesis report
// =============================================================================

/// Exact accounting for one synthesis operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisReport {
    /// Original gate kind.
    pub input_gate: GateKind,

    /// Number of generated operations.
    pub generated_operations: usize,

    /// Number of generated two-qubit operations.
    pub generated_two_qubit_operations: usize,

    /// Number of generated single-qubit operations.
    pub generated_single_qubit_operations: usize,

    /// Whether the input gate was preserved unchanged.
    pub preserved_native: bool,
}

impl SynthesisReport {
    fn from_operations(
        input_gate: GateKind,
        operations: &[Gate],
        preserved_native: bool,
    ) -> Self {
        let mut two_qubit = 0usize;

        for operation in operations {
            if operation.qubits().len() == 2 {
                two_qubit += 1;
            }
        }

        Self {
            input_gate,
            generated_operations: operations.len(),
            generated_two_qubit_operations: two_qubit,
            generated_single_qubit_operations:
                operations.len().saturating_sub(two_qubit),
            preserved_native,
        }
    }
}

/// Result containing the generated gates and resource accounting.
#[derive(Debug, Clone, PartialEq)]
pub struct SynthesisResult {
    /// Generated canonical IR operations.
    pub operations: Vec<Gate>,

    /// Exact resource accounting.
    pub report: SynthesisReport,
}

// =============================================================================
// Public synthesis entry points
// =============================================================================

/// Synthesizes one canonical two-qubit gate against a target capability set.
///
/// Native gates are always preserved. Otherwise an exact registered
/// decomposition is selected.
///
/// No approximate decomposition is performed.
pub fn synthesize_gate<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<SynthesisResult>
where
    T: TwoQubitSynthesisTarget,
{
    synthesize_gate_with_budget(
        gate,
        target,
        SynthesisBudget::unlimited(),
    )
}

/// Synthesizes one canonical two-qubit gate with an explicit resource budget.
pub fn synthesize_gate_with_budget<T>(
    gate: &Gate,
    target: &T,
    budget: SynthesisBudget,
) -> TwoQubitSynthesisResult<SynthesisResult>
where
    T: TwoQubitSynthesisTarget,
{
    validate_two_qubit_gate(gate)?;

    validate_target_for_gate(gate.kind(), target)?;

    if target.supports(gate.kind()) {
        let operations = vec![gate.clone()];

        let target_limit = target.max_generated_operations();

        if let Some(limit) = target_limit {
            if operations.len() > limit {
                return Err(
                    TwoQubitSynthesisError::BudgetExceeded {
                        maximum: limit,
                        required: operations.len(),
                    },
                );
            }
        }

        budget.check(operations.len())?;

        return Ok(SynthesisResult {
            report: SynthesisReport::from_operations(
                gate.kind(),
                &operations,
                true,
            ),
            operations,
        });
    }

    let operations = decompose_exact(gate, target)?;

    if let Some(limit) = target.max_generated_operations() {
        if operations.len() > limit {
            return Err(
                TwoQubitSynthesisError::BudgetExceeded {
                    maximum: limit,
                    required: operations.len(),
                },
            );
        }
    }

    budget.check(operations.len())?;

    validate_generated_operations(&operations)?;

    Ok(SynthesisResult {
        report: SynthesisReport::from_operations(
            gate.kind(),
            &operations,
            false,
        ),
        operations,
    })
}

/// Returns the exact number of operations generated by the decomposition
/// without allocating the final operation vector.
///
/// This is useful to a planner or cost model before committing a rewrite.
///
/// Symbolic parameters do not affect the count.
pub fn decomposition_operation_count(
    gate: GateKind,
) -> Option<usize> {
    match gate {
        GateKind::CX
        | GateKind::CZ => Some(1),

        GateKind::CY => Some(3),

        GateKind::SWAP => Some(3),

        GateKind::CRX => Some(6),

        GateKind::CRY
        | GateKind::CRZ => Some(4),

        GateKind::CH
        | GateKind::ISWAP
        | GateKind::ECR => None,

        _ => None,
    }
}

// =============================================================================
// Exact decomposition engine
// =============================================================================

fn decompose_exact<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    match gate.kind() {
        GateKind::CX => {
            decompose_cx(gate, target)
        }

        GateKind::CZ => {
            decompose_cz(gate, target)
        }

        GateKind::CY => {
            decompose_cy(gate, target)
        }

        GateKind::SWAP => {
            decompose_swap(gate, target)
        }

        GateKind::CRX => {
            decompose_crx(gate, target)
        }

        GateKind::CRY => {
            decompose_cry(gate, target)
        }

        GateKind::CRZ => {
            decompose_crz(gate, target)
        }

        GateKind::CH
        | GateKind::ISWAP
        | GateKind::ECR => {
            Err(
                TwoQubitSynthesisError::UnsupportedDecomposition {
                    gate: gate.kind(),
                },
            )
        }

        _ => {
            Err(
                TwoQubitSynthesisError::NotTwoQubit {
                    gate: gate.kind(),
                    operands: gate.qubits().len(),
                },
            )
        }
    }
}

// =============================================================================
// CX
// =============================================================================

fn decompose_cx<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CX) {
        return Ok(vec![gate.clone()]);
    }

    // H(target) ; CZ(control,target) ; H(target)
    if target.supports(GateKind::CZ)
        && target.supports(GateKind::H)
    {
        let control = gate.qubits()[0];
        let target_qubit = gate.qubits()[1];

        return Ok(vec![
            single(GateKind::H, target_qubit, &[])?,
            two(GateKind::CZ, control, target_qubit, &[])?,
            single(GateKind::H, target_qubit, &[])?,
        ]);
    }

    Err(
        TwoQubitSynthesisError::MissingTwoQubitCapability {
            gate: GateKind::CX,
        },
    )
}

// =============================================================================
// CZ
// =============================================================================

fn decompose_cz<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CZ) {
        return Ok(vec![gate.clone()]);
    }

    // H(target) ; CX(control,target) ; H(target)
    if target.supports(GateKind::CX)
        && target.supports(GateKind::H)
    {
        let control = gate.qubits()[0];
        let target_qubit = gate.qubits()[1];

        return Ok(vec![
            single(GateKind::H, target_qubit, &[])?,
            two(GateKind::CX, control, target_qubit, &[])?,
            single(GateKind::H, target_qubit, &[])?,
        ]);
    }

    Err(
        TwoQubitSynthesisError::MissingTwoQubitCapability {
            gate: GateKind::CZ,
        },
    )
}

// =============================================================================
// CY
// =============================================================================

fn decompose_cy<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CY) {
        return Ok(vec![gate.clone()]);
    }

    // Sdg(target) ; CX(control,target) ; S(target)
    //
    // Because:
    //
    //     S X S† = Y
    //
    // the resulting controlled operation is exactly CY.
    if target.supports(GateKind::CX)
        && target.supports(GateKind::S)
        && target.supports(GateKind::Sdg)
    {
        let control = gate.qubits()[0];
        let target_qubit = gate.qubits()[1];

        return Ok(vec![
            single(
                GateKind::Sdg,
                target_qubit,
                &[],
            )?,
            two(
                GateKind::CX,
                control,
                target_qubit,
                &[],
            )?,
            single(
                GateKind::S,
                target_qubit,
                &[],
            )?,
        ]);
    }

    Err(
        TwoQubitSynthesisError::MissingTwoQubitCapability {
            gate: GateKind::CY,
        },
    )
}

// =============================================================================
// SWAP
// =============================================================================

fn decompose_swap<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::SWAP) {
        return Ok(vec![gate.clone()]);
    }

    // SWAP(a,b) =
    //
    // CX(a,b)
    // CX(b,a)
    // CX(a,b)
    //
    // This is exact and preserves the logical operand identities.
    if target.supports(GateKind::CX) {
        let a = gate.qubits()[0];
        let b = gate.qubits()[1];

        return Ok(vec![
            two(GateKind::CX, a, b, &[])?,
            two(GateKind::CX, b, a, &[])?,
            two(GateKind::CX, a, b, &[])?,
        ]);
    }

    if target.supports(GateKind::CZ)
        && target.supports(GateKind::H)
    {
        // SWAP through CZ is possible, but using it directly here would make
        // this implementation larger and generally less useful than the
        // standard CNOT construction.
        //
        // Explicitly refuse rather than silently selecting a worse
        // decomposition. A future cost-aware planner can choose it.
        return Err(
            TwoQubitSynthesisError::MissingTwoQubitCapability {
                gate: GateKind::CX,
            },
        );
    }

    Err(
        TwoQubitSynthesisError::MissingTwoQubitCapability {
            gate: GateKind::CX,
        },
    )
}

// =============================================================================
// CRY
// =============================================================================

fn decompose_cry<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CRY) {
        return Ok(vec![gate.clone()]);
    }

    let theta = parameter_at(
        gate,
        0,
    )?;

    if !target.supports(GateKind::CX) {
        return Err(
            TwoQubitSynthesisError::MissingTwoQubitCapability {
                gate: GateKind::CX,
            },
        );
    }

    if !target.supports(GateKind::RY) {
        return Err(
            TwoQubitSynthesisError::MissingSingleQubitCapability {
                gate: GateKind::RY,
            },
        );
    }

    let half = scale_parameter(
        theta,
        0.5,
        gate.kind(),
        0,
    )?;

    let negative_half = negate_parameter(
        half.clone(),
        gate.kind(),
        0,
    )?;

    let control = gate.qubits()[0];
    let target_qubit = gate.qubits()[1];

    Ok(vec![
        single_parameter(
            GateKind::RY,
            target_qubit,
            half,
        )?,
        two(
            GateKind::CX,
            control,
            target_qubit,
            &[],
        )?,
        single_parameter(
            GateKind::RY,
            target_qubit,
            negative_half,
        )?,
        two(
            GateKind::CX,
            control,
            target_qubit,
            &[],
        )?,
    ])
}

// =============================================================================
// CRZ
// =============================================================================

fn decompose_crz<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CRZ) {
        return Ok(vec![gate.clone()]);
    }

    let theta = parameter_at(
        gate,
        0,
    )?;

    if !target.supports(GateKind::CX) {
        return Err(
            TwoQubitSynthesisError::MissingTwoQubitCapability {
                gate: GateKind::CX,
            },
        );
    }

    if !target.supports(GateKind::RZ) {
        return Err(
            TwoQubitSynthesisError::MissingSingleQubitCapability {
                gate: GateKind::RZ,
            },
        );
    }

    let half = scale_parameter(
        theta,
        0.5,
        gate.kind(),
        0,
    )?;

    let negative_half = negate_parameter(
        half.clone(),
        gate.kind(),
        0,
    )?;

    let control = gate.qubits()[0];
    let target_qubit = gate.qubits()[1];

    Ok(vec![
        single_parameter(
            GateKind::RZ,
            target_qubit,
            half,
        )?,
        two(
            GateKind::CX,
            control,
            target_qubit,
            &[],
        )?,
        single_parameter(
            GateKind::RZ,
            target_qubit,
            negative_half,
        )?,
        two(
            GateKind::CX,
            control,
            target_qubit,
            &[],
        )?,
    ])
}

// =============================================================================
// CRX
// =============================================================================

fn decompose_crx<T>(
    gate: &Gate,
    target: &T,
) -> TwoQubitSynthesisResult<Vec<Gate>>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(GateKind::CRX) {
        return Ok(vec![gate.clone()]);
    }

    // H RZ(theta) H = RX(theta).
    //
    // Therefore:
    //
    // CRX(theta)
    // =
    // H(target)
    // CRZ(theta)
    // H(target)
    //
    // The inner CRZ is recursively synthesized against the same target.
    if !target.supports(GateKind::H) {
        return Err(
            TwoQubitSynthesisError::MissingSingleQubitCapability {
                gate: GateKind::H,
            },
        );
    }

    let crz = Gate::new(
        GateKind::CRZ,
        gate.qubits().to_vec(),
        gate.parameters().to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        TwoQubitSynthesisError::InvalidGeneratedGate {
            gate: GateKind::CRZ,
            message: error.to_string(),
        }
    })?;

    let middle = decompose_crz(
        &crz,
        target,
    )?;

    let target_qubit = gate.qubits()[1];

    let mut operations = Vec::new();

    operations.push(single(
        GateKind::H,
        target_qubit,
        &[],
    )?);

    operations.extend(middle);

    operations.push(single(
        GateKind::H,
        target_qubit,
        &[],
    )?);

    Ok(operations)
}

// =============================================================================
// Validation
// =============================================================================

fn validate_two_qubit_gate(
    gate: &Gate,
) -> TwoQubitSynthesisResult<()> {
    if gate.qubits().len() != 2 {
        return Err(
            TwoQubitSynthesisError::NotTwoQubit {
                gate: gate.kind(),
                operands: gate.qubits().len(),
            },
        );
    }

    let first = gate.qubits()[0];
    let second = gate.qubits()[1];

    if first == second {
        return Err(
            TwoQubitSynthesisError::DuplicateQubit {
                qubit: first,
            },
        );
    }

    gate.validate().map_err(|error| {
        TwoQubitSynthesisError::InvalidInputGate {
            gate: gate.kind(),
            message: error.to_string(),
        }
    })?;

    Ok(())
}

fn validate_generated_operations(
    operations: &[Gate],
) -> TwoQubitSynthesisResult<()> {
    for operation in operations {
        operation.validate().map_err(|error| {
            TwoQubitSynthesisError::InvalidGeneratedGate {
                gate: operation.kind(),
                message: error.to_string(),
            }
        })?;
    }

    Ok(())
}

fn validate_target_for_gate<T>(
    gate: GateKind,
    target: &T,
) -> TwoQubitSynthesisResult<()>
where
    T: TwoQubitSynthesisTarget,
{
    if target.supports(gate) {
        return Ok(());
    }

    Ok(())
}

// =============================================================================
// Gate constructors
// =============================================================================

fn single(
    kind: GateKind,
    qubit: QubitId,
    parameters: &[Parameter],
) -> TwoQubitSynthesisResult<Gate> {
    Gate::new(
        kind,
        vec![qubit],
        parameters.to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        TwoQubitSynthesisError::InvalidGeneratedGate {
            gate: kind,
            message: error.to_string(),
        }
    })
}

fn single_parameter(
    kind: GateKind,
    qubit: QubitId,
    parameter: Parameter,
) -> TwoQubitSynthesisResult<Gate> {
    single(
        kind,
        qubit,
        &[parameter],
    )
}

fn two(
    kind: GateKind,
    first: QubitId,
    second: QubitId,
    parameters: &[Parameter],
) -> TwoQubitSynthesisResult<Gate> {
    if first == second {
        return Err(
            TwoQubitSynthesisError::DuplicateQubit {
                qubit: first,
            },
        );
    }

    Gate::new(
        kind,
        vec![first, second],
        parameters.to_vec(),
        None,
        None,
    )
    .map_err(|error| {
        TwoQubitSynthesisError::InvalidGeneratedGate {
            gate: kind,
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Parameter helpers
// =============================================================================

fn parameter_at<'a>(
    gate: &'a Gate,
    index: usize,
) -> TwoQubitSynthesisResult<&'a Parameter> {
    gate.parameters()
        .get(index)
        .ok_or(
            TwoQubitSynthesisError::MissingParameter {
                gate: gate.kind(),
                index,
            },
        )
}

fn scale_parameter(
    parameter: &Parameter,
    factor: f64,
    gate: GateKind,
    index: usize,
) -> TwoQubitSynthesisResult<Parameter> {
    if !factor.is_finite() {
        return Err(
            TwoQubitSynthesisError::InvalidParameter {
                gate,
                index,
            },
        );
    }

    match parameter {
        Parameter::Constant(value) => {
            let result = *value * factor;

            if !result.is_finite() {
                return Err(
                    TwoQubitSynthesisError::InvalidParameter {
                        gate,
                        index,
                    },
                );
            }

            Parameter::constant(result)
                .map_err(|_| {
                    TwoQubitSynthesisError::InvalidParameter {
                        gate,
                        index,
                    }
                })
        }

        Parameter::Symbol(_)
        | Parameter::Expression(_) => {
            let factor_parameter =
                Parameter::constant(factor)
                    .map_err(|_| {
                        TwoQubitSynthesisError::InvalidParameter {
                            gate,
                            index,
                        }
                    })?;

            Parameter::expression(
                ParameterExpression::Multiply(
                    Box::new(parameter.clone()),
                    Box::new(factor_parameter),
                ),
            )
            .map_err(|_| {
                TwoQubitSynthesisError::InvalidParameter {
                    gate,
                    index,
                }
            })
        }
    }
}

fn negate_parameter(
    parameter: Parameter,
    gate: GateKind,
    index: usize,
) -> TwoQubitSynthesisResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    TwoQubitSynthesisError::InvalidParameter {
                        gate,
                        index,
                    },
                );
            }

            Parameter::constant(-value)
                .map_err(|_| {
                    TwoQubitSynthesisError::InvalidParameter {
                        gate,
                        index,
                    }
                })
        }

        other => {
            Parameter::expression(
                ParameterExpression::Negate(
                    Box::new(other),
                ),
            )
            .map_err(|_| {
                TwoQubitSynthesisError::InvalidParameter {
                    gate,
                    index,
                }
            })
        }
    }
}

// =============================================================================
// Standard capability sets
// =============================================================================

fn supports_cnot_basis(
    gate: GateKind,
) -> bool {
    matches!(
        gate,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::V
            | GateKind::Vdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::CX
    )
}

fn supports_cz_basis(
    gate: GateKind,
) -> bool {
    matches!(
        gate,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::V
            | GateKind::Vdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::CZ
    )
}

fn supports_swap_basis(
    gate: GateKind,
) -> bool {
    supports_cnot_basis(gate)
        || matches!(gate, GateKind::SWAP)
}

fn supports_iswap_basis(
    gate: GateKind,
) -> bool {
    supports_cnot_basis(gate)
        || matches!(gate, GateKind::ISWAP)
}

fn supports_ecr_basis(
    gate: GateKind,
) -> bool {
    supports_cnot_basis(gate)
        || matches!(gate, GateKind::ECR)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn make_gate(
        kind: GateKind,
        qubits: &[usize],
        parameters: &[Parameter],
    ) -> Gate {
        Gate::new(
            kind,
            qubits.iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            parameters.to_vec(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn native_cx_is_preserved() {
        let gate = make_gate(
            GateKind::CX,
            &[0, 1],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CX must synthesize");

        assert!(result.report.preserved_native);
        assert_eq!(
            result.operations.len(),
            1
        );
        assert_eq!(
            result.operations[0].kind(),
            GateKind::CX
        );
    }

    #[test]
    fn cz_is_converted_to_cx_basis() {
        let gate = make_gate(
            GateKind::CZ,
            &[0, 1],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CZ must synthesize");

        assert!(!result.report.preserved_native);
        assert_eq!(
            result.operations.len(),
            3
        );
        assert_eq!(
            result.operations[0].kind(),
            GateKind::H
        );
        assert_eq!(
            result.operations[1].kind(),
            GateKind::CX
        );
        assert_eq!(
            result.operations[2].kind(),
            GateKind::H
        );
    }

    #[test]
    fn cx_is_converted_to_cz_basis() {
        let gate = make_gate(
            GateKind::CX,
            &[0, 1],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cz,
            )
            .expect("CX must synthesize");

        assert_eq!(
            result.operations.len(),
            3
        );
        assert_eq!(
            result.operations[0].kind(),
            GateKind::H
        );
        assert_eq!(
            result.operations[1].kind(),
            GateKind::CZ
        );
        assert_eq!(
            result.operations[2].kind(),
            GateKind::H
        );
    }

    #[test]
    fn swap_uses_three_cx_operations() {
        let gate = make_gate(
            GateKind::SWAP,
            &[0, 1],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("SWAP must synthesize");

        assert_eq!(
            result.operations.len(),
            3
        );

        assert_eq!(
            result.operations[0].qubits(),
            &[q(0), q(1)]
        );

        assert_eq!(
            result.operations[1].qubits(),
            &[q(1), q(0)]
        );

        assert_eq!(
            result.operations[2].qubits(),
            &[q(0), q(1)]
        );
    }

    #[test]
    fn cy_uses_sdg_cx_s() {
        let gate = make_gate(
            GateKind::CY,
            &[0, 1],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CY must synthesize");

        assert_eq!(
            result.operations.len(),
            3
        );

        assert_eq!(
            result.operations[0].kind(),
            GateKind::Sdg
        );
        assert_eq!(
            result.operations[1].kind(),
            GateKind::CX
        );
        assert_eq!(
            result.operations[2].kind(),
            GateKind::S
        );
    }

    #[test]
    fn cry_constant_parameter_is_halved() {
        let theta =
            Parameter::constant(1.2)
                .expect("finite parameter");

        let gate = make_gate(
            GateKind::CRY,
            &[0, 1],
            &[theta],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CRY must synthesize");

        assert_eq!(
            result.operations.len(),
            4
        );

        assert_eq!(
            result.operations[0]
                .parameters()[0]
                .as_constant(),
            Some(0.6)
        );

        assert_eq!(
            result.operations[2]
                .parameters()[0]
                .as_constant(),
            Some(-0.6)
        );
    }

    #[test]
    fn crz_symbolic_parameter_is_preserved() {
        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let gate = make_gate(
            GateKind::CRZ,
            &[0, 1],
            &[theta],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CRZ must synthesize");

        assert_eq!(
            result.operations.len(),
            4
        );

        assert!(
            result.operations[0]
                .parameters()[0]
                .is_symbolic()
        );

        assert!(
            result.operations[2]
                .parameters()[0]
                .is_symbolic()
        );
    }

    #[test]
    fn crx_uses_h_crz_h() {
        let theta =
            Parameter::constant(
                std::f64::consts::PI / 3.0,
            )
            .expect("finite parameter");

        let gate = make_gate(
            GateKind::CRX,
            &[0, 1],
            &[theta],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("CRX must synthesize");

        assert_eq!(
            result.operations.len(),
            6
        );

        assert_eq!(
            result.operations[0].kind(),
            GateKind::H
        );

        assert_eq!(
            result.operations[1].kind(),
            GateKind::RZ
        );

        assert_eq!(
            result.operations[2].kind(),
            GateKind::CX
        );

        assert_eq!(
            result.operations[3].kind(),
            GateKind::RZ
        );

        assert_eq!(
            result.operations[4].kind(),
            GateKind::CX
        );

        assert_eq!(
            result.operations[5].kind(),
            GateKind::H
        );
    }

    #[test]
    fn budget_is_enforced_before_result_is_returned() {
        let gate = make_gate(
            GateKind::SWAP,
            &[0, 1],
            &[],
        );

        let error =
            synthesize_gate_with_budget(
                &gate,
                &TwoQubitBasis::Cnot,
                SynthesisBudget::bounded(2),
            )
            .expect_err(
                "three operations must exceed budget two",
            );

        assert!(matches!(
            error,
            TwoQubitSynthesisError::BudgetExceeded {
                maximum: 2,
                required: 3
            }
        ));
    }

    #[test]
    fn unsupported_gate_fails_explicitly() {
        let gate = make_gate(
            GateKind::ISWAP,
            &[0, 1],
            &[],
        );

        let error =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect_err(
                "ISWAP must not be silently approximated",
            );

        assert!(matches!(
            error,
            TwoQubitSynthesisError::UnsupportedDecomposition {
                gate: GateKind::ISWAP
            }
        ));
    }

    #[test]
    fn duplicate_operands_are_rejected() {
        let gate = make_gate(
            GateKind::CX,
            &[0, 0],
            &[],
        );

        let error =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect_err(
                "duplicate operands must be rejected",
            );

        assert!(matches!(
            error,
            TwoQubitSynthesisError::DuplicateQubit {
                qubit: QubitId(_)
            }
        ));
    }

    #[test]
    fn decomposition_counts_are_deterministic() {
        assert_eq!(
            decomposition_operation_count(
                GateKind::CX
            ),
            Some(1)
        );

        assert_eq!(
            decomposition_operation_count(
                GateKind::CY
            ),
            Some(3)
        );

        assert_eq!(
            decomposition_operation_count(
                GateKind::SWAP
            ),
            Some(3)
        );

        assert_eq!(
            decomposition_operation_count(
                GateKind::CRY
            ),
            Some(4)
        );

        assert_eq!(
            decomposition_operation_count(
                GateKind::CRZ
            ),
            Some(4)
        );

        assert_eq!(
            decomposition_operation_count(
                GateKind::ISWAP
            ),
            None
        );
    }

    #[test]
    fn generated_operations_remain_canonical_ir() {
        let gate = make_gate(
            GateKind::SWAP,
            &[2, 7],
            &[],
        );

        let result =
            synthesize_gate(
                &gate,
                &TwoQubitBasis::Cnot,
            )
            .expect("SWAP must synthesize");

        validate_generated_operations(
            &result.operations,
        )
        .expect(
            "every generated operation must remain valid canonical IR",
        );
    }
}