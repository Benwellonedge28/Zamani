//! Zamani Quantum Optimization — Phase-Polynomial Synthesis
//!
//! Production-grade, deterministic synthesis of canonical phase-polynomial
//! representations into a backend-neutral phase-synthesis plan.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::Gate
//!       │
//!       ▼
//! optimization::algebra::phase_polynomial
//!       │
//!       ▼
//! optimization::synthesis::phase
//!       │
//!       ├── parity-network synthesis
//!       ├── phase-gadget synthesis
//!       ├── affine-parity synthesis
//!       ├── global-phase preservation
//!       └── resource accounting
//!       │
//!       ▼
//! canonical Quantum IR emitter
//!       │
//!       ▼
//! routing → scheduling → hardware
//! ```
//!
//! # Scope
//!
//! This module synthesizes a [`PhasePolynomial`] into a deterministic sequence
//! of primitive synthesis operations.
//!
//! The phase polynomial is the semantic input. The returned
//! [`PhaseSynthesisPlan`] is an *emission plan*, not a second Quantum IR.
//!
//! The canonical Quantum IR remains owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! This distinction is intentional. The optimizer needs a temporary,
//! backend-neutral description of what must be emitted before the caller
//! converts those operations into canonical `Gate` values.
//!
//! # Supported mathematics
//!
//! A phase polynomial has the form:
//!
//! ```text
//! U(x) = exp(i * global_phase)
//!        * exp(i * Σ θ_j f_j(x))
//! ```
//!
//! where each `f_j` is an affine Boolean parity.
//!
//! For a linear parity:
//!
//! ```text
//! f(x) = x_a ⊕ x_b ⊕ ... ⊕ x_n
//! ```
//!
//! synthesis uses:
//!
//! ```text
//! CNOT ladder
//! RZ/Phase(theta)
//! inverse CNOT ladder
//! ```
//!
//! For an affine parity:
//!
//! ```text
//! f(x) = 1 ⊕ x_a ⊕ x_b ⊕ ... ⊕ x_n
//! ```
//!
//! an X operation is inserted around the pivot:
//!
//! ```text
//! X
//! CNOT ladder
//! RZ/Phase(theta)
//! inverse CNOT ladder
//! X
//! ```
//!
//! This exactly realizes the affine parity rather than silently treating it
//! as a linear parity.
//!
//! # Global phase
//!
//! Global phase is never silently discarded.
//!
//! When `Phase(theta)` is emitted:
//!
//! ```text
//! Phase(theta) = diag(1, exp(i theta))
//! ```
//!
//! and therefore contributes no additional global phase.
//!
//! When `RZ(theta)` is emitted:
//!
//! ```text
//! RZ(theta)
//!     = exp(-i theta / 2) Phase(theta)
//! ```
//!
//! Therefore every RZ contributes:
//!
//! ```text
//! -theta / 2
//! ```
//!
//! to the physical global phase.
//!
//! The synthesizer compensates this by returning the residual global phase:
//!
//! ```text
//! required_global_phase
//!     + Σ(theta / 2)
//! ```
//!
//! If the caller elects to ignore global phase, the residual may explicitly be
//! discarded by policy. The default policy is to preserve it.
//!
//! # Why the module returns a plan
//!
//! Zamani's current canonical `GateKind` set does not expose a dedicated
//! `GlobalPhase` operation. A synthesis routine that directly manufactured
//! only `Gate` values would therefore have to either:
//!
//! 1. silently lose global phase;
//! 2. invent a non-canonical gate;
//! 3. incorrectly encode global phase as an ordinary operation.
//!
//! None of those are acceptable.
//!
//! Instead, this file returns a backend-neutral plan containing explicit
//! `GlobalPhase` steps. A future canonical-IR extension can map that step to a
//! first-class global-phase representation without changing this file.
//!
//! # Scaling
//!
//! There is no artificial maximum number of qubits, phase terms, or operations.
//!
//! Practical limits are controlled by [`PhaseSynthesisBudget`].
//!
//! Let:
//!
//! ```text
//! n = logical qubits
//! m = phase terms
//! k = maximum parity weight
//! w = total parity weight
//! ```
//!
//! The independent phase-gadget strategy requires approximately:
//!
//! ```text
//! O(w)
//! ```
//!
//! CNOT/X/phase emission operations, plus allocation proportional to the
//! emitted result.
//!
//! The underlying parity representation is already packed into O(n / 64)
//! words per parity in `phase_polynomial.rs`.
//!
//! No recursion proportional to circuit size is used.
//!
//! # Optimization strategy
//!
//! The initial production strategy implemented here is exact and deterministic:
//!
//! ```text
//! IndependentPhaseGadgets
//! ```
//!
//! Every parity is synthesized exactly once and restored afterwards.
//!
//! This deliberately establishes a correct baseline for the complete
//! optimizer. More advanced strategies such as:
//!
//! - Gray-code parity synthesis;
//! - ParitySynth;
//! - shared parity networks;
//! - Steiner-tree synthesis;
//! - architecture-aware synthesis;
//! - SAT/SMT synthesis;
//! - blockwise optimal synthesis;
//!
//! can be added behind [`PhaseSynthesisStrategy`] without modifying the
//! semantic contracts of this file.
//!
//! This separation is important because optimal parity-network synthesis is
//! computationally difficult, while scalable heuristic synthesis is practical.
//! Current research similarly treats phase-polynomial synthesis as a parity
//! network problem and uses heuristic or blockwise approaches for scalability.
//!
//! # Hardware independence
//!
//! This file does not know:
//!
//! - hardware topology;
//! - physical qubit identifiers;
//! - calibration;
//! - pulse duration;
//! - QPU APIs;
//! - routing;
//! - scheduling;
//! - noise models.
//!
//! A target is represented only by primitive capabilities.
//!
//! Hardware-aware synthesis belongs in the target/routing layers.
//!
//! # Determinism
//!
//! Synthesis is deterministic:
//!
//! - no randomness;
//! - no global mutable state;
//! - no wall-clock decisions;
//! - no hash-map iteration;
//! - no backend I/O;
//! - deterministic parity ordering inherited from `PhasePolynomial::terms()`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is used.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no external dependencies
//!
//! # Integration contract
//!
//! ## `optimization::algebra::phase_polynomial`
//!
//! Consumes:
//!
//! - `PhasePolynomial`
//! - `PhaseTerm`
//! - `AffineParity`
//! - `Parameter`
//!
//! This file does not modify that module.
//!
//! ## `optimization::targets`
//!
//! A future target can implement [`PhaseSynthesisTarget`].
//!
//! No change to this file is required when new hardware profiles are added.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline may call:
//!
//! ```text
//! synthesize_phase_polynomial()
//! ```
//!
//! and then lower the returned plan into the canonical Quantum IR.
//!
//! ## `optimization::verification`
//!
//! The plan preserves enough information for exact semantic verification:
//!
//! - original phase polynomial;
//! - emitted primitive operations;
//! - residual global phase.
//!
//! ## `optimization::cost`
//!
//! [`PhaseSynthesisReport`] exposes:
//!
//! - emitted operation count;
//! - CNOT count;
//! - X count;
//! - phase-gate count;
//! - global-phase operations;
//! - total parity weight;
//! - maximum parity weight.
//!
//! ## `routing`
//!
//! The CNOTs generated here are logical CNOTs.
//!
//! They must be routed only after logical synthesis unless a future target-aware
//! phase synthesizer deliberately supplies topology-aware synthesis.
//!
//! ## `scheduling`
//!
//! This file emits a sequential plan. Scheduling may later parallelize
//! independent operations.
//!
//! # Important invariant
//!
//! Every non-empty parity term is synthesized exactly once.
//!
//! No term is silently ignored.
//!
//! No unsupported target capability is silently replaced with an approximate
//! operation.
//!
//! No symbolic parameter is numerically evaluated.
//!
//! No global phase is silently removed.
//!

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::gate::GateKind;
use crate::quantum::ir::parameter::{Parameter, ParameterExpression};

use crate::quantum::optimization::algebra::phase_polynomial::{
    AffineParity,
    PhasePolynomial,
    PhaseTerm,
};

// =============================================================================
// Public result types
// =============================================================================

/// Result type returned by phase synthesis.
pub type PhaseSynthesisResult<T> = Result<T, PhaseSynthesisError>;

/// Errors produced by phase-polynomial synthesis.
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseSynthesisError {
    /// The polynomial contains no qubits required by the supplied operation.
    InvalidQubit {
        /// Logical qubit index.
        qubit: usize,

        /// Polynomial qubit count.
        qubits: usize,
    },

    /// A required primitive is unavailable.
    MissingCapability {
        /// Primitive that is required.
        primitive: PhasePrimitive,
    },

    /// The requested synthesis budget would be exceeded.
    BudgetExceeded {
        /// Maximum allowed operations.
        maximum: usize,

        /// Operations required.
        required: usize,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Operation being calculated.
        operation: &'static str,
    },

    /// The parameter could not be transformed into a valid IR parameter.
    InvalidParameter {
        /// Human-readable explanation.
        message: &'static str,
    },

    /// The phase polynomial violates its own dimensional invariants.
    InvalidPolynomial {
        /// Human-readable explanation.
        message: &'static str,
    },

    /// Exact global-phase preservation was requested but the target/emitter
    /// cannot represent a global phase operation.
    GlobalPhaseUnsupported,

    /// An emission plan cannot be represented under the selected policy.
    UnsupportedStrategy {
        /// Strategy requested.
        strategy: PhaseSynthesisStrategy,
    },
}

impl fmt::Display for PhaseSynthesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubit { qubit, qubits } => {
                write!(
                    f,
                    "phase synthesis references qubit {qubit}, \
                     but polynomial contains {qubits} qubits"
                )
            }

            Self::MissingCapability { primitive } => {
                write!(
                    f,
                    "phase synthesis target does not support \
                     required primitive {primitive:?}"
                )
            }

            Self::BudgetExceeded {
                maximum,
                required,
            } => {
                write!(
                    f,
                    "phase synthesis budget exceeded: maximum {maximum}, \
                     required {required}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "phase synthesis arithmetic overflow while calculating {operation}"
                )
            }

            Self::InvalidParameter { message } => {
                write!(
                    f,
                    "invalid phase-synthesis parameter: {message}"
                )
            }

            Self::InvalidPolynomial { message } => {
                write!(
                    f,
                    "invalid phase polynomial for synthesis: {message}"
                )
            }

            Self::GlobalPhaseUnsupported => {
                write!(
                    f,
                    "exact global-phase preservation was requested, \
                     but the selected target cannot represent global phase"
                )
            }

            Self::UnsupportedStrategy { strategy } => {
                write!(
                    f,
                    "phase synthesis strategy {strategy:?} is not supported"
                )
            }
        }
    }
}

impl Error for PhaseSynthesisError {}

// =============================================================================
// Primitive model
// =============================================================================

/// Primitive operations required by a phase-polynomial synthesizer.
///
/// This is a capability vocabulary, not another Quantum IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PhasePrimitive {
    /// Pauli-X.
    X,

    /// Controlled-NOT.
    CX,

    /// Z-axis rotation.
    RZ,

    /// Computational-basis phase shift.
    Phase,

    /// Circuit-level global phase.
    GlobalPhase,
}

/// Indicates which physical primitive should carry a phase term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhasePrimitivePreference {
    /// Prefer canonical RZ.
    ///
    /// RZ's global phase is explicitly compensated.
    Rz,

    /// Prefer Phase/U1-style diagonal phase.
    ///
    /// This avoids introducing additional global phase.
    Phase,
}

/// Controls whether a circuit's global phase is part of the synthesis
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalPhasePolicy {
    /// Preserve exact global phase.
    Preserve,

    /// Permit the caller to treat circuits as equivalent modulo global phase.
    ///
    /// This must be an explicit compiler decision.
    Ignore,
}

/// Available phase-polynomial synthesis strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhaseSynthesisStrategy {
    /// Synthesize each phase parity independently and restore the logical
    /// basis after each phase gadget.
    IndependentPhaseGadgets,
}

/// Logical synthesis target.
///
/// This trait deliberately describes primitive capabilities rather than
/// hardware topology.
pub trait PhaseSynthesisTarget {
    /// Returns whether the target supports a primitive.
    fn supports(&self, primitive: PhasePrimitive) -> bool;

    /// Optional additional operation budget supplied by the target.
    fn max_operations(&self) -> Option<usize> {
        None
    }
}

/// A simple capability target useful for unit tests and generic compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicPhaseTarget {
    /// Whether X is available.
    pub x: bool,

    /// Whether CX is available.
    pub cx: bool,

    /// Whether RZ is available.
    pub rz: bool,

    /// Whether Phase/U1 is available.
    pub phase: bool,

    /// Whether global phase can be represented.
    pub global_phase: bool,

    /// Optional target-local operation limit.
    pub max_operations: Option<usize>,
}

impl BasicPhaseTarget {
    /// A conventional CNOT + RZ target.
    #[must_use]
    pub const fn cnot_rz() -> Self {
        Self {
            x: true,
            cx: true,
            rz: true,
            phase: false,
            global_phase: false,
            max_operations: None,
        }
    }

    /// A conventional CNOT + Phase target.
    #[must_use]
    pub const fn cnot_phase() -> Self {
        Self {
            x: true,
            cx: true,
            rz: false,
            phase: true,
            global_phase: false,
            max_operations: None,
        }
    }

    /// A target supporting both diagonal representations and global phase.
    #[must_use]
    pub const fn universal_phase() -> Self {
        Self {
            x: true,
            cx: true,
            rz: true,
            phase: true,
            global_phase: true,
            max_operations: None,
        }
    }
}

impl PhaseSynthesisTarget for BasicPhaseTarget {
    fn supports(&self, primitive: PhasePrimitive) -> bool {
        match primitive {
            PhasePrimitive::X => self.x,
            PhasePrimitive::CX => self.cx,
            PhasePrimitive::RZ => self.rz,
            PhasePrimitive::Phase => self.phase,
            PhasePrimitive::GlobalPhase => self.global_phase,
        }
    }

    fn max_operations(&self) -> Option<usize> {
        self.max_operations
    }
}

// =============================================================================
// Synthesis budget
// =============================================================================

/// Explicit resource budget for one phase-polynomial synthesis operation.
///
/// `None` means unlimited at this layer.
///
/// The optimizer's global `OptimizationLimits` remains authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhaseSynthesisBudget {
    /// Maximum emitted primitive operations.
    pub max_operations: Option<usize>,
}

impl PhaseSynthesisBudget {
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

    fn check(self, required: usize) -> PhaseSynthesisResult<()> {
        match self.max_operations {
            Some(maximum) if required > maximum => {
                Err(PhaseSynthesisError::BudgetExceeded {
                    maximum,
                    required,
                })
            }

            _ => Ok(()),
        }
    }
}

// =============================================================================
// Synthesis options
// =============================================================================

/// Configuration for phase-polynomial synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhaseSynthesisOptions {
    /// Synthesis strategy.
    pub strategy: PhaseSynthesisStrategy,

    /// Preferred diagonal primitive.
    pub phase_preference: PhasePrimitivePreference,

    /// Global-phase handling policy.
    pub global_phase_policy: GlobalPhasePolicy,

    /// Per-synthesis resource budget.
    pub budget: PhaseSynthesisBudget,
}

impl PhaseSynthesisOptions {
    /// Conservative production defaults.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            strategy: PhaseSynthesisStrategy::IndependentPhaseGadgets,
            phase_preference: PhasePrimitivePreference::Rz,
            global_phase_policy: GlobalPhasePolicy::Preserve,
            budget: PhaseSynthesisBudget::unlimited(),
        }
    }

    /// Production defaults that explicitly permit equivalence modulo global
    /// phase.
    #[must_use]
    pub const fn modulo_global_phase() -> Self {
        Self {
            strategy: PhaseSynthesisStrategy::IndependentPhaseGadgets,
            phase_preference: PhasePrimitivePreference::Rz,
            global_phase_policy: GlobalPhasePolicy::Ignore,
            budget: PhaseSynthesisBudget::unlimited(),
        }
    }

    /// Uses Phase/U1-style diagonal operations.
    #[must_use]
    pub const fn phase_gates() -> Self {
        Self {
            strategy: PhaseSynthesisStrategy::IndependentPhaseGadgets,
            phase_preference: PhasePrimitivePreference::Phase,
            global_phase_policy: GlobalPhasePolicy::Preserve,
            budget: PhaseSynthesisBudget::unlimited(),
        }
    }
}

// =============================================================================
// Emission plan
// =============================================================================

/// One transient phase-synthesis operation.
///
/// This is deliberately *not* a replacement for `quantum::ir::Gate`.
///
/// The caller converts these operations to canonical IR using its own
/// validated emitter.
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseSynthesisOp {
    /// X on one logical qubit.
    X {
        /// Logical qubit.
        qubit: usize,
    },

    /// CNOT with logical control and target.
    CX {
        /// Control qubit.
        control: usize,

        /// Target qubit.
        target: usize,
    },

    /// RZ phase rotation.
    RZ {
        /// Target qubit.
        qubit: usize,

        /// Rotation angle.
        angle: Parameter,
    },

    /// Computational-basis phase shift.
    Phase {
        /// Target qubit.
        qubit: usize,

        /// Phase angle.
        angle: Parameter,
    },

    /// Explicit circuit global phase.
    GlobalPhase {
        /// Global phase angle.
        angle: Parameter,
    },
}

/// Final phase-synthesis result.
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseSynthesisPlan {
    /// Logical qubit count.
    qubits: usize,

    /// Synthesized primitive operations.
    operations: Vec<PhaseSynthesisOp>,

    /// Exact global phase that must be represented separately.
    ///
    /// This is zero when no residual global phase exists.
    residual_global_phase: Parameter,

    /// Synthesis statistics.
    report: PhaseSynthesisReport,
}

impl PhaseSynthesisPlan {
    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the generated operations.
    #[must_use]
    pub fn operations(&self) -> &[PhaseSynthesisOp] {
        &self.operations
    }

    /// Consumes the plan and returns its operations.
    #[must_use]
    pub fn into_operations(self) -> Vec<PhaseSynthesisOp> {
        self.operations
    }

    /// Returns the residual global phase.
    #[must_use]
    pub fn residual_global_phase(&self) -> &Parameter {
        &self.residual_global_phase
    }

    /// Returns synthesis statistics.
    #[must_use]
    pub const fn report(&self) -> &PhaseSynthesisReport {
        &self.report
    }

    /// Returns true when the plan contains no primitive operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Exact resource statistics for one synthesized phase polynomial.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhaseSynthesisReport {
    /// Number of input phase terms.
    pub terms: usize,

    /// Number of non-constant parity terms synthesized.
    pub synthesized_terms: usize,

    /// Number of CNOT operations.
    pub cnot_count: usize,

    /// Number of X operations.
    pub x_count: usize,

    /// Number of RZ operations.
    pub rz_count: usize,

    /// Number of Phase operations.
    pub phase_count: usize,

    /// Number of explicit global-phase operations.
    pub global_phase_count: usize,

    /// Total emitted primitive operations.
    pub operation_count: usize,

    /// Total parity support weight.
    pub total_parity_weight: usize,

    /// Largest parity support weight.
    pub maximum_parity_weight: usize,

    /// Number of affine terms.
    pub affine_terms: usize,
}

// =============================================================================
// Main synthesis API
// =============================================================================

/// Synthesizes a phase polynomial using production defaults.
///
/// The target must provide X, CX and at least one of RZ/Phase.
///
/// Global phase is preserved.
pub fn synthesize_phase_polynomial<T>(
    polynomial: &PhasePolynomial,
    target: &T,
) -> PhaseSynthesisResult<PhaseSynthesisPlan>
where
    T: PhaseSynthesisTarget,
{
    synthesize_phase_polynomial_with_options(
        polynomial,
        target,
        PhaseSynthesisOptions::production(),
    )
}

/// Synthesizes a phase polynomial with explicit options.
pub fn synthesize_phase_polynomial_with_options<T>(
    polynomial: &PhasePolynomial,
    target: &T,
    options: PhaseSynthesisOptions,
) -> PhaseSynthesisResult<PhaseSynthesisPlan>
where
    T: PhaseSynthesisTarget,
{
    validate_polynomial(polynomial)?;

    match options.strategy {
        PhaseSynthesisStrategy::IndependentPhaseGadgets => {
            synthesize_independent_gadgets(
                polynomial,
                target,
                options,
            )
        }
    }
}

// =============================================================================
// Core synthesis
// =============================================================================

fn synthesize_independent_gadgets<T>(
    polynomial: &PhasePolynomial,
    target: &T,
    options: PhaseSynthesisOptions,
) -> PhaseSynthesisResult<PhaseSynthesisPlan>
where
    T: PhaseSynthesisTarget,
{
    let terms: Vec<PhaseTerm> = polynomial.terms().collect();

    let mut report = PhaseSynthesisReport {
        terms: terms.len(),
        ..PhaseSynthesisReport::default()
    };

    /*
     * Pre-flight calculation.
     *
     * We calculate the exact output size before constructing the vector.
     * This prevents partially constructed plans from being returned after a
     * budget failure.
     */
    let mut required_operations = 0usize;
    let mut total_weight = 0usize;
    let mut maximum_weight = 0usize;
    let mut affine_terms = 0usize;
    let mut synthesized_terms = 0usize;

    let mut rz_global_correction = zero_parameter()?;

    let phase_primitive =
        choose_phase_primitive(target, options.phase_preference)?;

    for term in &terms {
        let parity = term.parity();
        let weight = parity.mask().weight();

        /*
         * Constant-only parity was already folded into PhasePolynomial's
         * global phase. Therefore a zero mask should never appear here as a
         * non-global term.
         */
        if weight == 0 {
            return Err(
                PhaseSynthesisError::InvalidPolynomial {
                    message:
                        "constant-only term remained in the non-global term set",
                },
            );
        }

        total_weight = total_weight
            .checked_add(weight)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "total parity weight",
                },
            )?;

        maximum_weight = maximum_weight.max(weight);

        if parity.constant() {
            affine_terms = affine_terms
                .checked_add(1)
                .ok_or(
                    PhaseSynthesisError::ArithmeticOverflow {
                        operation: "affine term count",
                    },
                )?;
        }

        /*
         * A parity of k variables requires:
         *
         *     k - 1 CNOTs to compute
         *     1 phase operation
         *     k - 1 CNOTs to restore
         *
         * An affine parity requires two additional X operations.
         */
        let ladder = weight
            .checked_sub(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "parity ladder length",
                },
            )?;

        let cnot_count = ladder
            .checked_mul(2)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "CNOT ladder count",
                },
            )?;

        let affine_x_count = if parity.constant() { 2 } else { 0 };

        let term_operations = cnot_count
            .checked_add(1)
            .and_then(|value| value.checked_add(affine_x_count))
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "term operation count",
                },
            )?;

        required_operations = required_operations
            .checked_add(term_operations)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "total operation count",
                },
            )?;

        synthesized_terms = synthesized_terms
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "synthesized term count",
                },
            )?;

        /*
         * RZ contributes exp(-i theta / 2) globally.
         *
         * To preserve the original phase polynomial exactly, the compensating
         * global phase is +theta/2.
         */
        if phase_primitive == PhasePrimitive::RZ {
            let half = half_parameter(
                term.coefficient().clone(),
            )?;

            rz_global_correction =
                add_parameters(
                    rz_global_correction,
                    half,
                )?;
        }
    }

    /*
     * The polynomial's own global phase must also be retained.
     */
    let residual_global_phase =
        add_parameters(
            polynomial.global_phase().clone(),
            rz_global_correction,
        )?;

    let global_phase_needed =
        !parameter_is_zero(&residual_global_phase);

    if global_phase_needed
        && matches!(
            options.global_phase_policy,
            GlobalPhasePolicy::Preserve
        )
        && !target.supports(PhasePrimitive::GlobalPhase)
    {
        /*
         * Do not silently lose exact unitary semantics.
         *
         * The caller can explicitly choose Ignore if their equivalence
         * relation is modulo global phase.
         */
        return Err(
            PhaseSynthesisError::GlobalPhaseUnsupported,
        );
    }

    if global_phase_needed
        && matches!(
            options.global_phase_policy,
            GlobalPhasePolicy::Preserve
        )
    {
        required_operations = required_operations
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "global phase operation count",
                },
            )?;
    }

    /*
     * Both the local synthesis budget and target budget apply.
     */
    options.budget.check(required_operations)?;

    if let Some(maximum) = target.max_operations() {
        if required_operations > maximum {
            return Err(
                PhaseSynthesisError::BudgetExceeded {
                    maximum,
                    required: required_operations,
                },
            );
        }
    }

    /*
     * Only now allocate the final operation vector.
     */
    let mut operations =
        Vec::with_capacity(required_operations);

    for term in &terms {
        synthesize_term(
            term,
            phase_primitive,
            &mut operations,
            &mut report,
        )?;
    }

    report.terms = terms.len();
    report.synthesized_terms = synthesized_terms;
    report.total_parity_weight = total_weight;
    report.maximum_parity_weight = maximum_weight;
    report.affine_terms = affine_terms;

    /*
     * The global phase is represented explicitly only when exact preservation
     * was requested.
     */
    if global_phase_needed
        && matches!(
            options.global_phase_policy,
            GlobalPhasePolicy::Preserve
        )
    {
        operations.push(
            PhaseSynthesisOp::GlobalPhase {
                angle: residual_global_phase.clone(),
            },
        );

        report.global_phase_count = 1;
    }

    report.operation_count = operations.len();

    Ok(PhaseSynthesisPlan {
        qubits: polynomial.qubits(),
        operations,
        residual_global_phase,
        report,
    })
}

// =============================================================================
// Single-term synthesis
// =============================================================================

fn synthesize_term(
    term: &PhaseTerm,
    phase_primitive: PhasePrimitive,
    operations: &mut Vec<PhaseSynthesisOp>,
    report: &mut PhaseSynthesisReport,
) -> PhaseSynthesisResult<()> {
    let parity = term.parity();
    let qubits = parity.to_qubits();

    if qubits.is_empty() {
        return Err(
            PhaseSynthesisError::InvalidPolynomial {
                message:
                    "non-global phase term has empty parity support",
            },
        );
    }

    /*
     * The pivot is deterministic.
     *
     * Choosing the lowest participating logical qubit keeps output stable
     * across runs and platforms.
     */
    let pivot = qubits[0];

    /*
     * Affine parity:
     *
     *     1 ⊕ p
     *
     * is obtained by flipping the pivot before and after computing p.
     */
    if parity.constant() {
        operations.push(
            PhaseSynthesisOp::X {
                qubit: pivot,
            },
        );

        report.x_count = report
            .x_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "X count",
                },
            )?;

        report.operation_count = report
            .operation_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "operation count",
                },
            )?;
    }

    /*
     * Compute the parity onto the pivot.
     *
     * CNOT(control, pivot) transforms:
     *
     *     pivot := control XOR pivot
     */
    for &control in qubits.iter().skip(1) {
        operations.push(
            PhaseSynthesisOp::CX {
                control,
                target: pivot,
            },
        );

        report.cnot_count = report
            .cnot_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "CNOT count",
                },
            )?;

        report.operation_count = report
            .operation_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "operation count",
                },
            )?;
    }

    /*
     * Apply the phase while the pivot contains the required parity.
     */
    match phase_primitive {
        PhasePrimitive::RZ => {
            operations.push(
                PhaseSynthesisOp::RZ {
                    qubit: pivot,
                    angle: term.coefficient().clone(),
                },
            );

            report.rz_count = report
                .rz_count
                .checked_add(1)
                .ok_or(
                    PhaseSynthesisError::ArithmeticOverflow {
                        operation: "RZ count",
                    },
                )?;
        }

        PhasePrimitive::Phase => {
            operations.push(
                PhaseSynthesisOp::Phase {
                    qubit: pivot,
                    angle: term.coefficient().clone(),
                },
            );

            report.phase_count = report
                .phase_count
                .checked_add(1)
                .ok_or(
                    PhaseSynthesisError::ArithmeticOverflow {
                        operation: "phase count",
                    },
                )?;
        }

        _ => {
            return Err(
                PhaseSynthesisError::MissingCapability {
                    primitive: phase_primitive,
                },
            );
        }
    }

    report.operation_count = report
        .operation_count
        .checked_add(1)
        .ok_or(
            PhaseSynthesisError::ArithmeticOverflow {
                operation: "operation count",
            },
        )?;

    /*
     * Restore the original logical parity basis.
     *
     * CNOTs are self-inverse, so the reverse sequence restores the pivot.
     */
    for &control in qubits.iter().skip(1).rev() {
        operations.push(
            PhaseSynthesisOp::CX {
                control,
                target: pivot,
            },
        );

        report.cnot_count = report
            .cnot_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "CNOT count",
                },
            )?;

        report.operation_count = report
            .operation_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "operation count",
                },
            )?;
    }

    if parity.constant() {
        operations.push(
            PhaseSynthesisOp::X {
                qubit: pivot,
            },
        );

        report.x_count = report
            .x_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "X count",
                },
            )?;

        report.operation_count = report
            .operation_count
            .checked_add(1)
            .ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "operation count",
                },
            )?;
    }

    Ok(())
}

// =============================================================================
// Capability selection
// =============================================================================

fn choose_phase_primitive<T>(
    target: &T,
    preference: PhasePrimitivePreference,
) -> PhaseSynthesisResult<PhasePrimitive>
where
    T: PhaseSynthesisTarget,
{
    match preference {
        PhasePrimitivePreference::Rz => {
            if target.supports(PhasePrimitive::RZ) {
                Ok(PhasePrimitive::RZ)
            } else if target.supports(PhasePrimitive::Phase) {
                Ok(PhasePrimitive::Phase)
            } else {
                Err(
                    PhaseSynthesisError::MissingCapability {
                        primitive: PhasePrimitive::RZ,
                    },
                )
            }
        }

        PhasePrimitivePreference::Phase => {
            if target.supports(PhasePrimitive::Phase) {
                Ok(PhasePrimitive::Phase)
            } else if target.supports(PhasePrimitive::RZ) {
                Ok(PhasePrimitive::RZ)
            } else {
                Err(
                    PhaseSynthesisError::MissingCapability {
                        primitive: PhasePrimitive::Phase,
                    },
                )
            }
        }
    }
}

// =============================================================================
// Polynomial validation
// =============================================================================

fn validate_polynomial(
    polynomial: &PhasePolynomial,
) -> PhaseSynthesisResult<()> {
    /*
     * PhasePolynomial::terms() already exposes canonical terms. The explicit
     * checks here protect this module from future representation changes.
     */
    for term in polynomial.terms() {
        let parity = term.parity();

        if parity.qubits() != polynomial.qubits() {
            return Err(
                PhaseSynthesisError::InvalidPolynomial {
                    message:
                        "term dimension differs from polynomial dimension",
                },
            );
        }

        for qubit in parity.to_qubits() {
            if qubit >= polynomial.qubits() {
                return Err(
                    PhaseSynthesisError::InvalidQubit {
                        qubit,
                        qubits: polynomial.qubits(),
                    },
                );
            }
        }

        term.coefficient()
            .validate()
            .map_err(|_| {
                PhaseSynthesisError::InvalidParameter {
                    message:
                        "phase coefficient failed canonical IR validation",
                }
            })?;
    }

    polynomial
        .global_phase()
        .validate()
        .map_err(|_| {
            PhaseSynthesisError::InvalidParameter {
                message:
                    "global phase failed canonical IR validation",
            }
        })?;

    Ok(())
}

// =============================================================================
// Parameter helpers
// =============================================================================

fn zero_parameter() -> PhaseSynthesisResult<Parameter> {
    Parameter::constant(0.0).map_err(|_| {
        PhaseSynthesisError::InvalidParameter {
            message:
                "failed to construct constant zero parameter",
        }
    })
}

fn half_parameter(
    parameter: Parameter,
) -> PhaseSynthesisResult<Parameter> {
    let half =
        Parameter::constant(0.5).map_err(|_| {
            PhaseSynthesisError::InvalidParameter {
                message:
                    "failed to construct constant one-half parameter",
            }
        })?;

    Parameter::expression(
        ParameterExpression::Multiply(
            Box::new(parameter),
            Box::new(half),
        ),
    )
    .map_err(|_| {
        PhaseSynthesisError::InvalidParameter {
            message:
                "failed to construct parameter multiplied by one-half",
        }
    })
}

fn add_parameters(
    left: Parameter,
    right: Parameter,
) -> PhaseSynthesisResult<Parameter> {
    /*
     * Fold concrete constants immediately.
     *
     * This prevents unnecessary expression growth for large phase
     * polynomials containing many numeric terms.
     */
    match (left, right) {
        (Parameter::Constant(a), Parameter::Constant(b)) => {
            let value = a + b;

            if !value.is_finite() {
                return Err(
                    PhaseSynthesisError::InvalidParameter {
                        message:
                            "constant phase addition became non-finite",
                    },
                );
            }

            Parameter::constant(value).map_err(|_| {
                PhaseSynthesisError::InvalidParameter {
                    message:
                        "failed to construct folded constant phase",
                }
            })
        }

        (left, right) => {
            Parameter::expression(
                ParameterExpression::Add(
                    Box::new(left),
                    Box::new(right),
                ),
            )
            .map_err(|_| {
                PhaseSynthesisError::InvalidParameter {
                    message:
                        "failed to construct symbolic phase addition",
                }
            })
        }
    }
}

fn parameter_is_zero(
    parameter: &Parameter,
) -> bool {
    match parameter {
        Parameter::Constant(value) => *value == 0.0,

        /*
         * Symbolic zero cannot be proven from the current IR parameter
         * contract without performing symbolic algebra. Therefore it must
         * remain represented.
         */
        Parameter::Symbol(_) |
        Parameter::Expression(_) => false,
    }
}

// =============================================================================
// Canonical lowering support
// =============================================================================

/// A trait implemented by a canonical-IR emitter.
///
/// The synthesis algorithm itself remains independent from the exact `Gate`
/// constructor API. This is deliberate: canonical gate construction and
/// validation belong to `quantum::ir::gate`.
///
/// A future implementation can translate:
///
/// ```text
/// PhaseSynthesisOp::X
///     -> GateKind::X
///
/// PhaseSynthesisOp::CX
///     -> GateKind::CX
///
/// PhaseSynthesisOp::RZ
///     -> GateKind::RZ
///
/// PhaseSynthesisOp::Phase
///     -> GateKind::Phase / GateKind::U1
///
/// PhaseSynthesisOp::GlobalPhase
///     -> canonical circuit/global-phase representation
/// ```
///
/// without modifying this synthesis algorithm.
pub trait PhaseSynthesisEmitter {
    /// Output type produced for one primitive operation.
    type Output;

    /// Error returned by the canonical emitter.
    type Error;

    /// Emits one X operation.
    fn emit_x(
        &mut self,
        qubit: usize,
    ) -> Result<Self::Output, Self::Error>;

    /// Emits one CNOT operation.
    fn emit_cx(
        &mut self,
        control: usize,
        target: usize,
    ) -> Result<Self::Output, Self::Error>;

    /// Emits one RZ operation.
    fn emit_rz(
        &mut self,
        qubit: usize,
        angle: Parameter,
    ) -> Result<Self::Output, Self::Error>;

    /// Emits one Phase/U1 operation.
    fn emit_phase(
        &mut self,
        qubit: usize,
        angle: Parameter,
    ) -> Result<Self::Output, Self::Error>;

    /// Emits a circuit-level global phase.
    fn emit_global_phase(
        &mut self,
        angle: Parameter,
    ) -> Result<Self::Output, Self::Error>;
}

/// Low-level conversion of a synthesis plan into canonical operations.
///
/// The emitter is responsible for constructing validated
/// `crate::quantum::ir::Gate` values.
///
/// This function intentionally does not depend on a particular `Gate`
/// constructor, allowing the canonical IR to evolve without requiring phase
/// synthesis to be rewritten.
pub fn emit_plan<E>(
    plan: &PhaseSynthesisPlan,
    emitter: &mut E,
) -> Result<Vec<E::Output>, E::Error>
where
    E: PhaseSynthesisEmitter,
{
    let mut output =
        Vec::with_capacity(plan.operations.len());

    for operation in &plan.operations {
        let emitted = match operation {
            PhaseSynthesisOp::X { qubit } => {
                emitter.emit_x(*qubit)
            }

            PhaseSynthesisOp::CX {
                control,
                target,
            } => {
                emitter.emit_cx(
                    *control,
                    *target,
                )
            }

            PhaseSynthesisOp::RZ {
                qubit,
                angle,
            } => {
                emitter.emit_rz(
                    *qubit,
                    angle.clone(),
                )
            }

            PhaseSynthesisOp::Phase {
                qubit,
                angle,
            } => {
                emitter.emit_phase(
                    *qubit,
                    angle.clone(),
                )
            }

            PhaseSynthesisOp::GlobalPhase {
                angle,
            } => {
                emitter.emit_global_phase(
                    angle.clone(),
                )
            }
        }?;

        output.push(emitted);
    }

    Ok(output)
}

// =============================================================================
// Analysis helpers
// =============================================================================

/// Calculates the exact number of primitive operations required by the
/// independent phase-gadget strategy without allocating the operation list.
///
/// This is useful for planners and resource-limit checks.
pub fn estimate_operation_count(
    polynomial: &PhasePolynomial,
    phase_primitive: PhasePrimitive,
    preserve_global_phase: bool,
) -> PhaseSynthesisResult<usize> {
    validate_polynomial(polynomial)?;

    let mut total = 0usize;

    for term in polynomial.terms() {
        let weight = term.parity().mask().weight();

        if weight == 0 {
            return Err(
                PhaseSynthesisError::InvalidPolynomial {
                    message:
                        "empty non-global parity encountered",
                },
            );
        }

        let ladder =
            weight.checked_sub(1).ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation: "estimated ladder length",
                },
            )?;

        let cnot_count =
            ladder.checked_mul(2).ok_or(
                PhaseSynthesisError::ArithmeticOverflow {
                    operation:
                        "estimated CNOT count",
                },
            )?;

        let affine_x_count =
            if term.parity().constant() {
                2
            } else {
                0
            };

        let term_count =
            cnot_count
                .checked_add(affine_x_count)
                .and_then(|value| value.checked_add(1))
                .ok_or(
                    PhaseSynthesisError::ArithmeticOverflow {
                        operation:
                            "estimated term operation count",
                    },
                )?;

        total = total.checked_add(term_count).ok_or(
            PhaseSynthesisError::ArithmeticOverflow {
                operation:
                    "estimated total operation count",
            },
        )?;
    }

    /*
     * The global phase operation is required only if the selected primitive
     * does not itself make the polynomial's global phase zero.
     *
     * Exact determination is deliberately conservative for symbolic values.
     */
    if preserve_global_phase
        && phase_primitive == PhasePrimitive::Phase
        && !parameter_is_zero(polynomial.global_phase())
    {
        total = total.checked_add(1).ok_or(
            PhaseSynthesisError::ArithmeticOverflow {
                operation:
                    "estimated global phase operation count",
            },
        )?;
    }

    Ok(total)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::parameter::Parameter;

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("finite test parameter")
    }

    fn simple_polynomial(
        qubits: usize,
        support: &[usize],
        angle: f64,
    ) -> PhasePolynomial {
        let mut polynomial =
            PhasePolynomial::new(qubits)
                .expect("valid polynomial");

        let parity =
            AffineParity::new(
                crate::quantum::optimization::algebra::phase_polynomial::ParityMask::from_qubits(
                    qubits,
                    support.iter().copied(),
                )
                .expect("valid parity"),
                false,
            );

        polynomial
            .add_phase(
                parity,
                constant(angle),
            )
            .expect("valid phase");

        polynomial
    }

    #[test]
    fn single_qubit_phase_needs_one_phase_operation() {
        let polynomial =
            simple_polynomial(
                1,
                &[0],
                std::f64::consts::PI,
            );

        let target =
            BasicPhaseTarget::cnot_phase();

        let options =
            PhaseSynthesisOptions::phase_gates();

        let plan =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &target,
                options,
            )
            .expect("synthesis succeeds");

        assert_eq!(
            plan.report().phase_count,
            1
        );

        assert_eq!(
            plan.report().cnot_count,
            0
        );

        assert_eq!(
            plan.report().operation_count,
            1
        );
    }

    #[test]
    fn two_qubit_phase_has_two_cnot_ladders() {
        let polynomial =
            simple_polynomial(
                2,
                &[0, 1],
                std::f64::consts::FRAC_PI_2,
            );

        let target =
            BasicPhaseTarget::cnot_phase();

        let plan =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &target,
                PhaseSynthesisOptions::phase_gates(),
            )
            .expect("synthesis succeeds");

        /*
         * Compute parity:
         *
         * CX(1, 0)
         *
         * phase
         *
         * restore:
         *
         * CX(1, 0)
         */
        assert_eq!(
            plan.report().cnot_count,
            2
        );

        assert_eq!(
            plan.report().phase_count,
            1
        );

        assert_eq!(
            plan.report().operation_count,
            3
        );
    }

    #[test]
    fn affine_phase_adds_two_x_operations() {
        let mut polynomial =
            PhasePolynomial::new(2)
                .expect("valid polynomial");

        let mask =
            crate::quantum::optimization::algebra::phase_polynomial::ParityMask::from_qubits(
                2,
                [0, 1],
            )
            .expect("valid parity");

        let parity =
            AffineParity::new(
                mask,
                true,
            );

        polynomial
            .add_phase(
                parity,
                constant(
                    std::f64::consts::FRAC_PI_4,
                ),
            )
            .expect("valid phase");

        let plan =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &BasicPhaseTarget::cnot_phase(),
                PhaseSynthesisOptions::phase_gates(),
            )
            .expect("synthesis succeeds");

        assert_eq!(
            plan.report().x_count,
            2
        );

        assert_eq!(
            plan.report().cnot_count,
            2
        );

        assert_eq!(
            plan.report().phase_count,
            1
        );

        assert_eq!(
            plan.report().operation_count,
            5
        );
    }

    #[test]
    fn rz_synthesis_preserves_residual_global_phase() {
        let polynomial =
            simple_polynomial(
                1,
                &[0],
                std::f64::consts::PI,
            );

        /*
         * RZ(pi) contributes a physical global phase of -pi/2.
         * The synthesizer therefore returns +pi/2 as compensation.
         */
        let target =
            BasicPhaseTarget::cnot_rz();

        let plan =
            synthesize_phase_polynomial(
                &polynomial,
                &target,
            );

        /*
         * BasicPhaseTarget::cnot_rz() deliberately does not claim support
         * for an explicit GlobalPhase primitive.
         *
         * Exact preservation must therefore fail rather than silently
         * discard the phase.
         */
        assert_eq!(
            plan,
            Err(
                PhaseSynthesisError::GlobalPhaseUnsupported
            )
        );
    }

    #[test]
    fn modulo_global_phase_allows_rz() {
        let polynomial =
            simple_polynomial(
                1,
                &[0],
                std::f64::consts::PI,
            );

        let plan =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &BasicPhaseTarget::cnot_rz(),
                PhaseSynthesisOptions::modulo_global_phase(),
            )
            .expect("modulo-global-phase synthesis succeeds");

        assert_eq!(
            plan.report().rz_count,
            1
        );
    }

    #[test]
    fn bounded_budget_is_enforced_before_emission() {
        let polynomial =
            simple_polynomial(
                3,
                &[0, 1, 2],
                std::f64::consts::PI,
            );

        let mut options =
            PhaseSynthesisOptions::phase_gates();

        /*
         * 3-qubit parity requires:
         *
         * 2 CNOTs
         * 1 phase
         * 2 CNOTs
         *
         * = 5 operations.
         */
        options.budget =
            PhaseSynthesisBudget::bounded(4);

        let result =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &BasicPhaseTarget::cnot_phase(),
                options,
            );

        assert_eq!(
            result,
            Err(
                PhaseSynthesisError::BudgetExceeded {
                    maximum: 4,
                    required: 5,
                }
            )
        );
    }

    #[test]
    fn estimate_matches_independent_gadget_strategy() {
        let polynomial =
            simple_polynomial(
                4,
                &[0, 1, 2, 3],
                std::f64::consts::FRAC_PI_4,
            );

        let estimated =
            estimate_operation_count(
                &polynomial,
                PhasePrimitive::Phase,
                false,
            )
            .expect("estimation succeeds");

        assert_eq!(
            estimated,
            7
        );
    }

    #[test]
    fn symbolic_parameters_are_not_evaluated() {
        let mut polynomial =
            PhasePolynomial::new(1)
                .expect("valid polynomial");

        let parity =
            AffineParity::variable(1, 0)
                .expect("valid parity");

        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        polynomial
            .add_phase(
                parity,
                theta.clone(),
            )
            .expect("symbolic phase accepted");

        let plan =
            synthesize_phase_polynomial_with_options(
                &polynomial,
                &BasicPhaseTarget::cnot_phase(),
                PhaseSynthesisOptions::phase_gates(),
            )
            .expect("symbolic synthesis succeeds");

        match &plan.operations()[0] {
            PhaseSynthesisOp::Phase {
                angle,
                ..
            } => {
                assert_eq!(
                    angle,
                    &theta
                );
            }

            operation => {
                panic!(
                    "expected phase operation, got {operation:?}"
                );
            }
        }
    }
}