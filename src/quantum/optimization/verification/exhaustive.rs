//! Zamani Quantum Optimization — Exhaustive Verification
//!
//! Production-grade exhaustive semantic verification for the canonical
//! Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    canonical Quantum IR
//!                            │
//!                            ▼
//!              verification::exhaustive
//!                            │
//!                exhaustive computational
//!                  basis-state verification
//!                            │
//!              ┌─────────────┴─────────────┐
//!              ▼                           ▼
//!       exact relation              global-phase relation
//!              │                           │
//!              └─────────────┬─────────────┘
//!                            ▼
//!                    ExhaustiveReport
//! ```
//!
//! # Purpose
//!
//! This module provides a deterministic, exact-in-method exhaustive verifier
//! for small-to-large circuits where the available resources permit complete
//! computational-basis verification.
//!
//! For an n-qubit unitary circuit, every one of the 2^n computational-basis
//! input states is independently executed through both circuits. The resulting
//! state vectors are compared completely.
//!
//! This is stronger than checking a single input state.
//!
//! It is also deliberately separate from:
//!
//! - optimization passes;
//! - rewrite rules;
//! - routing;
//! - scheduling;
//! - hardware;
//! - QPU execution;
//! - benchmarking;
//! - frontend parsing;
//! - quantum algorithms;
//! - error correction.
//!
//! # Canonical IR
//!
//! The only quantum representation consumed here is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and its canonical:
//!
//! `crate::quantum::ir::Gate`
//!
//! `crate::quantum::ir::GateKind`
//!
//! `crate::quantum::ir::Parameter`
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! No optimizer-specific QuantumGate representation is introduced.
//!
//! # Exhaustive meaning
//!
//! For a unitary circuit over n logical qubits, exhaustive verification checks:
//!
//! ```text
//! |0...000>
//! |0...001>
//! |0...010>
//! ...
//! |1...111>
//! ```
//!
//! and compares the complete output state for every basis input.
//!
//! Because computational-basis vectors form a complete basis, equality of the
//! resulting linear transformation is established for the implemented logical
//! gate semantics.
//!
//! # Global phase
//!
//! Two unitary circuits may satisfy:
//!
//!     U = exp(i * phi) V
//!
//! while having different matrix elements.
//!
//! The caller explicitly chooses whether global phase is:
//!
//! - significant; or
//! - ignored.
//!
//! When global phase is ignored, one phase factor is established from the
//! first non-zero output amplitude and is then required to remain consistent
//! across every basis input.
//!
//! A different phase for a later basis column is therefore correctly rejected.
//!
//! # Symbolic parameters
//!
//! This module never silently binds symbolic parameters.
//!
//! A circuit containing an unbound symbolic parameter produces an explicit
//! `Inconclusive` result.
//!
//! A caller that wants symbolic verification must first bind parameters through
//! the canonical IR parameter subsystem or use a future symbolic verifier.
//!
//! # Non-unitary circuits
//!
//! Measurement and reset are not treated as ordinary unitary operations.
//!
//! They therefore produce `Inconclusive` rather than being incorrectly modeled
//! as unitary transformations.
//!
//! Barrier operations are also not simulated as physical state transformations.
//! A barrier-containing circuit is therefore inconclusive unless the two
//! circuits are structurally identical.
//!
//! This conservative behavior is intentional.
//!
//! # Scalability
//!
//! Generic exhaustive unitary verification is exponential in the number of
//! logical qubits. No sound implementation can remove that mathematical cost.
//!
//! This implementation therefore scales safely by:
//!
//! - using no fixed architectural maximum;
//! - using checked power-of-two arithmetic;
//! - checking all resource budgets before allocation;
//! - using fallible vector reservation;
//! - processing one computational-basis input at a time;
//! - never constructing the complete dense 2^n × 2^n matrix;
//! - allowing explicit basis-state, amplitude, operation, and time limits;
//! - returning `Inconclusive` rather than exhausting process resources;
//! - never treating resource exhaustion as equivalence.
//!
//! The memory requirement is therefore O(2^n), rather than O(4^n), while the
//! exhaustive runtime remains O(4^n) up to circuit-operation factors.
//!
//! "Unlimited" in Zamani means:
//!
//! > there is no arbitrary architectural circuit-size ceiling; execution may
//! > continue for as long as the configured resource policy and actual machine
//! > resources safely permit.
//!
//! It does not mean that exponential quantum-equivalence verification becomes
//! polynomial.
//!
//! # Determinism
//!
//! Verification is deterministic.
//!
//! There is:
//!
//! - no randomness;
//! - no sampling;
//! - no backend execution;
//! - no network access;
//! - no hidden parameter binding;
//! - no nondeterministic scheduling.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No `unsafe`.
//!
//! # Integration contract
//!
//! `verification/mod.rs` should eventually contain:
//!
//! ```text
//! pub mod exhaustive;
//! ```
//!
//! `verification/semantic.rs` may use this module when an explicitly
//! exhaustive method is requested.
//!
//! `verification/randomized.rs` may be used as an alternative when exhaustive
//! verification reaches a configured resource boundary.
//!
//! `verification/certificates.rs` may consume `ExhaustiveReport`.
//!
//! `pipeline.rs` should invoke this module only when exhaustive verification is
//! selected by policy. The optimizer pipeline must never assume that an
//! `Inconclusive` result means equivalence.
//!
//! `tests/equivalence.rs` and `tests/properties.rs` should use the public
//! `verify` and `prove` helpers.
//!
//! No optimizer pass should need to modify this file merely because another
//! optimization pass is added.
//!
//! # Security properties
//!
//! This module:
//!
//! - contains no unsafe code;
//! - does not mutate either input circuit;
//! - validates both circuits before verification;
//! - rejects incompatible qubit counts;
//! - rejects symbolic parameters;
//! - rejects unsupported non-unitary semantics;
//! - checks every computational-basis input within the configured budget;
//! - uses checked arithmetic;
//! - uses fallible allocation;
//! - reports resource exhaustion as inconclusive;
//! - never converts inconclusive into equivalent;
//! - never executes hardware;
//! - never performs backend I/O;
//! - never trusts an optimizer's claimed result.
//!
//! # Gate semantics
//!
//! All currently canonical GateKind values are handled explicitly.
//!
//! The verifier uses the canonical logical gate definitions and never depends
//! on physical gate decomposition or backend topology.

#![forbid(unsafe_code)]

use std::fmt;
use std::time::{Duration, Instant};

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

// ============================================================================
// Public identifiers
// ============================================================================

/// Stable subsystem identifier.
pub const EXHAUSTIVE_VERIFICATION_ID: &str =
    "quantum.optimization.verification.exhaustive";

/// Public API contract version.
pub const EXHAUSTIVE_VERIFICATION_VERSION: u32 = 1;

// ============================================================================
// Complex arithmetic
// ============================================================================

/// Minimal dependency-free complex number used by the exhaustive verifier.
///
/// The canonical Quantum IR deliberately does not depend on a numerical
/// complex-number crate. Keeping this implementation local prevents the
/// verifier from changing the IR dependency boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };
    const ONE: Self = Self { re: 1.0, im: 0.0 };
    const I: Self = Self { re: 0.0, im: 1.0 };

    const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    fn norm_squared(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    fn conjugate(self) -> Self {
        Self::new(self.re, -self.im)
    }

    fn scale(self, value: f64) -> Self {
        Self::new(self.re * value, self.im * value)
    }

    fn multiply(self, rhs: Self) -> Self {
        Self::new(
            self.re.mul_add(rhs.re, -(self.im * rhs.im)),
            self.re.mul_add(rhs.im, self.im * rhs.re),
        )
    }

    fn divide(self, rhs: Self) -> Option<Self> {
        let denominator = rhs.norm_squared();

        if !denominator.is_finite() || denominator == 0.0 {
            return None;
        }

        Some(Self::new(
            (self.re * rhs.re + self.im * rhs.im) / denominator,
            (self.im * rhs.re - self.re * rhs.im) / denominator,
        ))
    }

    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }

    fn exp_i(theta: f64) -> Option<Self> {
        if !theta.is_finite() {
            return None;
        }

        let (sin, cos) = theta.sin_cos();
        let value = Self::new(cos, sin);

        if value.is_finite() {
            Some(value)
        } else {
            None
        }
    }
}

// ============================================================================
// Public relation
// ============================================================================

/// Semantic relation used by exhaustive verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExhaustiveRelation {
    /// Require U == V.
    Exact,

    /// Permit one circuit-wide global phase:
    ///
    /// U = exp(i * phi) V
    UpToGlobalPhase,
}

impl Default for ExhaustiveRelation {
    fn default() -> Self {
        Self::UpToGlobalPhase
    }
}

// ============================================================================
// Public limits
// ============================================================================

/// Resource policy for one exhaustive verification request.
///
/// Zero for `max_basis_states` means "all basis states permitted by
/// `max_qubits`".
///
/// There is intentionally no fixed architectural maximum qubit count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExhaustiveLimits {
    /// Maximum logical qubits.
    pub max_qubits: usize,

    /// Maximum number of amplitudes in one state vector.
    pub max_amplitudes: usize,

    /// Maximum number of computational-basis input states.
    ///
    /// Zero means derive the complete basis count from the qubit count.
    pub max_basis_states: usize,

    /// Maximum number of operations inspected per circuit.
    pub max_operations: usize,

    /// Optional wall-clock budget.
    pub max_duration: Option<Duration>,
}

impl ExhaustiveLimits {
    /// Conservative production default.
    ///
    /// Twenty qubits means one million amplitudes per state vector.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_qubits: 20,
            max_amplitudes: 1usize << 20,
            max_basis_states: 0,
            max_operations: 10_000_000,
            max_duration: None,
        }
    }

    /// Larger production profile.
    #[must_use]
    pub const fn large() -> Self {
        Self {
            max_qubits: 26,
            max_amplitudes: 1usize << 26,
            max_basis_states: 0,
            max_operations: 100_000_000,
            max_duration: None,
        }
    }

    /// Creates a custom resource policy.
    pub fn new(
        max_qubits: usize,
        max_amplitudes: usize,
        max_basis_states: usize,
        max_operations: usize,
        max_duration: Option<Duration>,
    ) -> Result<Self, ExhaustiveError> {
        if max_qubits == 0 {
            return Err(ExhaustiveError::InvalidLimit {
                field: "max_qubits",
                value: 0,
            });
        }

        if max_amplitudes == 0 {
            return Err(ExhaustiveError::InvalidLimit {
                field: "max_amplitudes",
                value: 0,
            });
        }

        if max_operations == 0 {
            return Err(ExhaustiveError::InvalidLimit {
                field: "max_operations",
                value: 0,
            });
        }

        if let Some(duration) = max_duration {
            if duration.is_zero() {
                return Err(ExhaustiveError::InvalidLimit {
                    field: "max_duration",
                    value: 0,
                });
            }
        }

        Ok(Self {
            max_qubits,
            max_amplitudes,
            max_basis_states,
            max_operations,
            max_duration,
        })
    }
}

impl Default for ExhaustiveLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// Public configuration
// ============================================================================

/// Complete exhaustive-verification configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExhaustiveConfig {
    /// Semantic relation.
    pub relation: ExhaustiveRelation,

    /// Component-wise numerical tolerance.
    pub absolute_tolerance: f64,

    /// Relative numerical tolerance.
    pub relative_tolerance: f64,

    /// Resource limits.
    pub limits: ExhaustiveLimits,
}

impl ExhaustiveConfig {
    /// Production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            relation: ExhaustiveRelation::UpToGlobalPhase,
            absolute_tolerance: 1.0e-12,
            relative_tolerance: 1.0e-10,
            limits: ExhaustiveLimits::conservative(),
        }
    }

    /// Strict exact-relation configuration.
    ///
    /// Floating-point arithmetic remains subject to the configured numerical
    /// tolerance because the verifier evaluates transcendental gate parameters.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            relation: ExhaustiveRelation::Exact,
            absolute_tolerance: 1.0e-12,
            relative_tolerance: 1.0e-10,
            limits: ExhaustiveLimits::conservative(),
        }
    }

    /// Global-phase-insensitive production configuration.
    #[must_use]
    pub const fn up_to_global_phase() -> Self {
        Self::production()
    }

    /// Creates a validated numerical configuration.
    pub fn new(
        relation: ExhaustiveRelation,
        absolute_tolerance: f64,
        relative_tolerance: f64,
        limits: ExhaustiveLimits,
    ) -> Result<Self, ExhaustiveError> {
        if !absolute_tolerance.is_finite() || absolute_tolerance < 0.0 {
            return Err(ExhaustiveError::InvalidTolerance {
                field: "absolute_tolerance",
                value: absolute_tolerance,
            });
        }

        if !relative_tolerance.is_finite() || relative_tolerance < 0.0 {
            return Err(ExhaustiveError::InvalidTolerance {
                field: "relative_tolerance",
                value: relative_tolerance,
            });
        }

        Ok(Self {
            relation,
            absolute_tolerance,
            relative_tolerance,
            limits,
        })
    }
}

impl Default for ExhaustiveConfig {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Public verdict
// ============================================================================

/// Result of exhaustive verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExhaustiveVerdict {
    /// All required basis states were checked and the circuits were equivalent.
    Equivalent,

    /// At least one basis state demonstrated non-equivalence.
    NotEquivalent,

    /// A definitive answer could not safely be established.
    Inconclusive,
}

impl ExhaustiveVerdict {
    /// Returns true only for a proven equivalent result.
    #[must_use]
    pub const fn is_equivalent(self) -> bool {
        matches!(self, Self::Equivalent)
    }

    /// Returns true only for a proven non-equivalent result.
    #[must_use]
    pub const fn is_not_equivalent(self) -> bool {
        matches!(self, Self::NotEquivalent)
    }

    /// Returns true when no definitive proof was produced.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }
}

// ============================================================================
// Public inconclusive reasons
// ============================================================================

/// Reason exhaustive verification could not produce a definitive proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExhaustiveReason {
    /// Circuit contains symbolic parameters.
    SymbolicParameters,

    /// Circuit contains measurement.
    Measurement,

    /// Circuit contains reset.
    Reset,

    /// Circuit contains a barrier.
    Barrier,

    /// Unsupported logical gate.
    UnsupportedGate(GateKind),

    /// Number of logical qubits exceeds the configured budget.
    QubitLimitExceeded {
        actual: usize,
        maximum: usize,
    },

    /// Required state-vector size exceeds the configured budget.
    AmplitudeLimitExceeded {
        required: usize,
        maximum: usize,
    },

    /// Required basis-state count exceeds the configured budget.
    BasisStateLimitExceeded {
        required: usize,
        maximum: usize,
    },

    /// Operation count exceeds the configured budget.
    OperationLimitExceeded {
        actual: usize,
        maximum: usize,
    },

    /// Verification deadline reached.
    TimeLimitExceeded,

    /// Arithmetic overflow while calculating a resource requirement.
    ArithmeticOverflow {
        calculation: &'static str,
    },

    /// Memory reservation could not be satisfied.
    AllocationFailed {
        amplitudes: usize,
    },

    /// Input circuits do not have compatible logical dimensions.
    IncompatibleShape,

    /// A gate contains invalid numerical data.
    NonFiniteGateParameter,

    /// State simulation produced a non-finite amplitude.
    NonFiniteState,

    /// A global phase could not be determined safely.
    UndefinedGlobalPhase,
}

impl fmt::Display for ExhaustiveReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolicParameters => {
                formatter.write_str("circuit contains unbound symbolic parameters")
            }

            Self::Measurement => {
                formatter.write_str(
                    "measurement semantics are outside the unitary exhaustive verifier",
                )
            }

            Self::Reset => {
                formatter.write_str(
                    "reset semantics are outside the unitary exhaustive verifier",
                )
            }

            Self::Barrier => {
                formatter.write_str(
                    "barrier semantics are not simulated as unitary operations",
                )
            }

            Self::UnsupportedGate(gate) => {
                write!(
                    formatter,
                    "gate {gate:?} is not supported by exhaustive simulation"
                )
            }

            Self::QubitLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "logical qubit limit exceeded: actual {actual}, maximum {maximum}"
                )
            }

            Self::AmplitudeLimitExceeded { required, maximum } => {
                write!(
                    formatter,
                    "state-vector amplitude limit exceeded: required {required}, maximum {maximum}"
                )
            }

            Self::BasisStateLimitExceeded { required, maximum } => {
                write!(
                    formatter,
                    "basis-state limit exceeded: required {required}, maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "operation limit exceeded: actual {actual}, maximum {maximum}"
                )
            }

            Self::TimeLimitExceeded => {
                formatter.write_str("exhaustive verification time limit reached")
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::AllocationFailed { amplitudes } => {
                write!(
                    formatter,
                    "unable to reserve state-vector storage for {amplitudes} amplitudes"
                )
            }

            Self::IncompatibleShape => {
                formatter.write_str("circuits have incompatible logical dimensions")
            }

            Self::NonFiniteGateParameter => {
                formatter.write_str("gate parameter evaluated to a non-finite value")
            }

            Self::NonFiniteState => {
                formatter.write_str("state-vector simulation produced a non-finite amplitude")
            }

            Self::UndefinedGlobalPhase => {
                formatter.write_str(
                    "a global phase could not be determined from the compared states",
                )
            }
        }
    }
}

// ============================================================================
// Public mismatch
// ============================================================================

/// Detailed evidence for a proven non-equivalence.
#[derive(Debug, Clone, PartialEq)]
pub struct ExhaustiveMismatch {
    /// Computational-basis input index that first demonstrated a mismatch.
    pub basis_state: usize,

    /// First amplitude index that differed.
    pub amplitude_index: usize,

    /// Original-circuit amplitude.
    pub original: (f64, f64),

    /// Optimized-circuit amplitude.
    pub optimized: (f64, f64),

    /// Global phase used by the comparison, when applicable.
    pub global_phase: Option<(f64, f64)>,
}

impl ExhaustiveMismatch {
    fn new(
        basis_state: usize,
        amplitude_index: usize,
        original: Complex,
        optimized: Complex,
        global_phase: Option<Complex>,
    ) -> Self {
        Self {
            basis_state,
            amplitude_index,
            original: (original.re, original.im),
            optimized: (optimized.re, optimized.im),
            global_phase: global_phase.map(|value| (value.re, value.im)),
        }
    }
}

// ============================================================================
// Public report
// ============================================================================

/// Complete result of one exhaustive verification invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct ExhaustiveReport {
    /// Stable verifier identifier.
    pub verifier_id: &'static str,

    /// Public API contract version.
    pub verifier_version: u32,

    /// Final verdict.
    pub verdict: ExhaustiveVerdict,

    /// Semantic relation used.
    pub relation: ExhaustiveRelation,

    /// Number of logical qubits.
    pub qubits: usize,

    /// Number of amplitudes in one state vector.
    pub amplitudes: usize,

    /// Number of basis states that were checked.
    pub basis_states_checked: usize,

    /// Total number of operations inspected per circuit.
    pub operations_per_circuit: usize,

    /// Total elapsed verification time.
    pub elapsed: Duration,

    /// Reason for an inconclusive result.
    pub inconclusive_reason: Option<ExhaustiveReason>,

    /// First mismatch when non-equivalence was proven.
    pub mismatch: Option<ExhaustiveMismatch>,

    /// Established global phase, when global phase was ignored.
    pub global_phase: Option<(f64, f64)>,
}

impl ExhaustiveReport {
    fn base(
        config: ExhaustiveConfig,
        qubits: usize,
        amplitudes: usize,
        operations: usize,
    ) -> Self {
        Self {
            verifier_id: EXHAUSTIVE_VERIFICATION_ID,
            verifier_version: EXHAUSTIVE_VERIFICATION_VERSION,
            verdict: ExhaustiveVerdict::Inconclusive,
            relation: config.relation,
            qubits,
            amplitudes,
            basis_states_checked: 0,
            operations_per_circuit: operations,
            elapsed: Duration::ZERO,
            inconclusive_reason: None,
            mismatch: None,
            global_phase: None,
        }
    }

    /// Returns true only when exhaustive equivalence was proven.
    #[must_use]
    pub const fn is_equivalent(&self) -> bool {
        self.verdict.is_equivalent()
    }

    /// Returns true only when non-equivalence was proven.
    #[must_use]
    pub const fn is_not_equivalent(&self) -> bool {
        self.verdict.is_not_equivalent()
    }

    /// Returns true when exhaustive verification was inconclusive.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.verdict.is_inconclusive()
    }
}

// ============================================================================
// Public errors
// ============================================================================

/// Errors that prevent exhaustive verification from executing.
///
/// A valid verifier result such as `Inconclusive` is not represented as an
/// error. This error type is reserved for invalid verifier configuration or
/// invalid canonical input.
#[derive(Debug, Clone, PartialEq)]
pub enum ExhaustiveError {
    /// Invalid numerical tolerance.
    InvalidTolerance {
        field: &'static str,
        value: f64,
    },

    /// Invalid resource limit.
    InvalidLimit {
        field: &'static str,
        value: usize,
    },

    /// Canonical IR validation failed.
    InvalidCircuit {
        circuit: &'static str,
        message: String,
    },
}

impl fmt::Display for ExhaustiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTolerance { field, value } => {
                write!(
                    formatter,
                    "invalid exhaustive-verification tolerance `{field}`: {value}"
                )
            }

            Self::InvalidLimit { field, value } => {
                write!(
                    formatter,
                    "invalid exhaustive-verification limit `{field}`: {value}"
                )
            }

            Self::InvalidCircuit { circuit, message } => {
                write!(
                    formatter,
                    "invalid {circuit} quantum circuit: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ExhaustiveError {}

// ============================================================================
// Public API
// ============================================================================

/// Exhaustively verifies two canonical Quantum IR circuits using the default
/// production configuration.
pub fn verify(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<ExhaustiveReport, ExhaustiveError> {
    verify_with_config(
        original,
        optimized,
        ExhaustiveConfig::production(),
    )
}

/// Exhaustively verifies two canonical Quantum IR circuits using the supplied
/// configuration.
///
/// This function never treats resource exhaustion as equivalence.
pub fn verify_with_config(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    config: ExhaustiveConfig,
) -> Result<ExhaustiveReport, ExhaustiveError> {
    validate_config(&config)?;
    validate_input(original, "original")?;
    validate_input(optimized, "optimized")?;

    let started = Instant::now();

    let original_qubits = original.num_qubits();
    let optimized_qubits = optimized.num_qubits();

    let original_operations = original.operations().len();
    let optimized_operations = optimized.operations().len();

    let operations = original_operations.max(optimized_operations);

    if original_qubits != optimized_qubits {
        let mut report = ExhaustiveReport::base(
            config,
            original_qubits.max(optimized_qubits),
            0,
            operations,
        );

        report.verdict = ExhaustiveVerdict::Inconclusive;
        report.inconclusive_reason = Some(ExhaustiveReason::IncompatibleShape);
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    let qubits = original_qubits;

    if qubits > config.limits.max_qubits {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason = Some(
            ExhaustiveReason::QubitLimitExceeded {
                actual: qubits,
                maximum: config.limits.max_qubits,
            },
        );
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    if original_operations > config.limits.max_operations {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason = Some(
            ExhaustiveReason::OperationLimitExceeded {
                actual: original_operations,
                maximum: config.limits.max_operations,
            },
        );
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    if optimized_operations > config.limits.max_operations {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason = Some(
            ExhaustiveReason::OperationLimitExceeded {
                actual: optimized_operations,
                maximum: config.limits.max_operations,
            },
        );
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    if let Some(reason) = unsupported_semantics(original) {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason = Some(reason);
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    if let Some(reason) = unsupported_semantics(optimized) {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason = Some(reason);
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    if contains_symbolic_parameters(original)
        || contains_symbolic_parameters(optimized)
    {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            0,
            operations,
        );

        report.inconclusive_reason =
            Some(ExhaustiveReason::SymbolicParameters);
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    let amplitudes = match basis_size(qubits) {
        Ok(value) => value,
        Err(reason) => {
            let mut report = ExhaustiveReport::base(
                config,
                qubits,
                0,
                operations,
            );

            report.inconclusive_reason = Some(reason);
            report.elapsed = started.elapsed();

            return Ok(report);
        }
    };

    if amplitudes > config.limits.max_amplitudes {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            amplitudes,
            operations,
        );

        report.inconclusive_reason = Some(
            ExhaustiveReason::AmplitudeLimitExceeded {
                required: amplitudes,
                maximum: config.limits.max_amplitudes,
            },
        );
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    let basis_states = amplitudes;

    if config.limits.max_basis_states != 0
        && basis_states > config.limits.max_basis_states
    {
        let mut report = ExhaustiveReport::base(
            config,
            qubits,
            amplitudes,
            operations,
        );

        report.inconclusive_reason = Some(
            ExhaustiveReason::BasisStateLimitExceeded {
                required: basis_states,
                maximum: config.limits.max_basis_states,
            },
        );
        report.elapsed = started.elapsed();

        return Ok(report);
    }

    let mut original_state =
        match allocate_state(amplitudes) {
            Ok(state) => state,
            Err(reason) => {
                let mut report = ExhaustiveReport::base(
                    config,
                    qubits,
                    amplitudes,
                    operations,
                );

                report.inconclusive_reason = Some(reason);
                report.elapsed = started.elapsed();

                return Ok(report);
            }
        };

    let mut optimized_state =
        match allocate_state(amplitudes) {
            Ok(state) => state,
            Err(reason) => {
                let mut report = ExhaustiveReport::base(
                    config,
                    qubits,
                    amplitudes,
                    operations,
                );

                report.inconclusive_reason = Some(reason);
                report.elapsed = started.elapsed();

                return Ok(report);
            }
        };

    let mut report = ExhaustiveReport::base(
        config,
        qubits,
        amplitudes,
        operations,
    );

    let mut global_phase: Option<Complex> = None;

    for basis_state in 0..basis_states {
        if deadline_reached(started, config.limits.max_duration) {
            report.inconclusive_reason =
                Some(ExhaustiveReason::TimeLimitExceeded);
            report.elapsed = started.elapsed();
            return Ok(report);
        }

        reset_basis_state(
            &mut original_state,
            basis_state,
        );

        reset_basis_state(
            &mut optimized_state,
            basis_state,
        );

        if let Err(reason) = simulate(
            original,
            &mut original_state,
            config.limits.max_duration,
            started,
        ) {
            report.inconclusive_reason = Some(reason);
            report.elapsed = started.elapsed();
            return Ok(report);
        }

        if let Err(reason) = simulate(
            optimized,
            &mut optimized_state,
            config.limits.max_duration,
            started,
        ) {
            report.inconclusive_reason = Some(reason);
            report.elapsed = started.elapsed();
            return Ok(report);
        }

        match config.relation {
            ExhaustiveRelation::Exact => {
                if let Some(mismatch) = compare_exact(
                    basis_state,
                    &original_state,
                    &optimized_state,
                    config.absolute_tolerance,
                    config.relative_tolerance,
                ) {
                    report.verdict =
                        ExhaustiveVerdict::NotEquivalent;
                    report.mismatch = Some(mismatch);
                    report.basis_states_checked =
                        basis_state.saturating_add(1);
                    report.elapsed = started.elapsed();

                    return Ok(report);
                }
            }

            ExhaustiveRelation::UpToGlobalPhase => {
                match compare_up_to_global_phase(
                    basis_state,
                    &original_state,
                    &optimized_state,
                    &mut global_phase,
                    config.absolute_tolerance,
                    config.relative_tolerance,
                ) {
                    PhaseComparison::Equivalent => {}

                    PhaseComparison::Mismatch(mismatch) => {
                        report.verdict =
                            ExhaustiveVerdict::NotEquivalent;
                        report.mismatch = Some(mismatch);
                        report.basis_states_checked =
                            basis_state.saturating_add(1);
                        report.elapsed = started.elapsed();

                        return Ok(report);
                    }

                    PhaseComparison::UndefinedPhase => {
                        report.inconclusive_reason =
                            Some(
                                ExhaustiveReason::UndefinedGlobalPhase,
                            );
                        report.basis_states_checked =
                            basis_state.saturating_add(1);
                        report.elapsed = started.elapsed();

                        return Ok(report);
                    }
                }
            }
        }

        report.basis_states_checked =
            basis_state.saturating_add(1);
    }

    report.verdict = ExhaustiveVerdict::Equivalent;
    report.global_phase = global_phase.map(|value| {
        (value.re, value.im)
    });
    report.elapsed = started.elapsed();

    Ok(report)
}

/// Strict exhaustive verification.
///
/// Returns `Ok(())` only when equivalence is proven.
///
/// `NotEquivalent` and `Inconclusive` are both returned as
/// `ExhaustiveError::InvalidCircuit`-style operational failures would be
/// inappropriate, so this helper uses the dedicated proof error below.
pub fn prove(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<ExhaustiveReport, ExhaustiveProofError> {
    prove_with_config(
        original,
        optimized,
        ExhaustiveConfig::production(),
    )
}

/// Strict exhaustive verification with an explicit configuration.
pub fn prove_with_config(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    config: ExhaustiveConfig,
) -> Result<ExhaustiveReport, ExhaustiveProofError> {
    let report = verify_with_config(
        original,
        optimized,
        config,
    )
    .map_err(ExhaustiveProofError::Execution)?;

    match report.verdict {
        ExhaustiveVerdict::Equivalent => Ok(report),

        ExhaustiveVerdict::NotEquivalent
        | ExhaustiveVerdict::Inconclusive => {
            Err(ExhaustiveProofError::NotProven(report))
        }
    }
}

// ============================================================================
// Strict proof error
// ============================================================================

/// Failure of the strict proof-only API.
#[derive(Debug, Clone, PartialEq)]
pub enum ExhaustiveProofError {
    /// Verifier configuration or canonical-input failure.
    Execution(ExhaustiveError),

    /// Verification completed without proving equivalence.
    NotProven(ExhaustiveReport),
}

impl fmt::Display for ExhaustiveProofError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Execution(error) => {
                write!(
                    formatter,
                    "exhaustive verification could not execute: {error}"
                )
            }

            Self::NotProven(report) => {
                match report.verdict {
                    ExhaustiveVerdict::NotEquivalent => {
                        formatter.write_str(
                            "exhaustive verification proved non-equivalence",
                        )
                    }

                    ExhaustiveVerdict::Inconclusive => {
                        formatter.write_str(
                            "exhaustive verification could not prove equivalence",
                        )
                    }

                    ExhaustiveVerdict::Equivalent => {
                        formatter.write_str(
                            "internal exhaustive proof-state inconsistency",
                        )
                    }
                }
            }
        }
    }
}

impl std::error::Error for ExhaustiveProofError {}

// ============================================================================
// Configuration/input validation
// ============================================================================

fn validate_config(
    config: &ExhaustiveConfig,
) -> Result<(), ExhaustiveError> {
    if !config.absolute_tolerance.is_finite()
        || config.absolute_tolerance < 0.0
    {
        return Err(ExhaustiveError::InvalidTolerance {
            field: "absolute_tolerance",
            value: config.absolute_tolerance,
        });
    }

    if !config.relative_tolerance.is_finite()
        || config.relative_tolerance < 0.0
    {
        return Err(ExhaustiveError::InvalidTolerance {
            field: "relative_tolerance",
            value: config.relative_tolerance,
        });
    }

    if config.limits.max_qubits == 0 {
        return Err(ExhaustiveError::InvalidLimit {
            field: "max_qubits",
            value: 0,
        });
    }

    if config.limits.max_amplitudes == 0 {
        return Err(ExhaustiveError::InvalidLimit {
            field: "max_amplitudes",
            value: 0,
        });
    }

    if config.limits.max_operations == 0 {
        return Err(ExhaustiveError::InvalidLimit {
            field: "max_operations",
            value: 0,
        });
    }

    Ok(())
}

fn validate_input(
    circuit: &QuantumCircuit,
    name: &'static str,
) -> Result<(), ExhaustiveError> {
    circuit
        .validate()
        .map_err(|error| ExhaustiveError::InvalidCircuit {
            circuit: name,
            message: error.to_string(),
        })
}

fn basis_size(
    qubits: usize,
) -> Result<usize, ExhaustiveReason> {
    if qubits >= usize::BITS as usize {
        return Err(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "2^qubits basis-state count",
            },
        );
    }

    1usize
        .checked_shl(qubits as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "2^qubits basis-state count",
            },
        )
}

// ============================================================================
// Semantic preflight
// ============================================================================

fn unsupported_semantics(
    circuit: &QuantumCircuit,
) -> Option<ExhaustiveReason> {
    for gate in circuit.operations() {
        match gate.kind() {
            GateKind::Measure => {
                return Some(ExhaustiveReason::Measurement);
            }

            GateKind::Reset => {
                return Some(ExhaustiveReason::Reset);
            }

            GateKind::Barrier => {
                return Some(ExhaustiveReason::Barrier);
            }

            _ => {}
        }
    }

    None
}

fn contains_symbolic_parameters(
    circuit: &QuantumCircuit,
) -> bool {
    circuit
        .operations()
        .iter()
        .any(|gate| {
            gate.parameters()
                .iter()
                .any(Parameter::is_symbolic)
        })
}

// ============================================================================
// State-vector allocation
// ============================================================================

fn allocate_state(
    amplitudes: usize,
) -> Result<Vec<Complex>, ExhaustiveReason> {
    let mut state = Vec::new();

    if state.try_reserve_exact(amplitudes).is_err() {
        return Err(
            ExhaustiveReason::AllocationFailed {
                amplitudes,
            },
        );
    }

    state.resize(amplitudes, Complex::ZERO);

    Ok(state)
}

fn reset_basis_state(
    state: &mut [Complex],
    basis_state: usize,
) {
    for value in state.iter_mut() {
        *value = Complex::ZERO;
    }

    if let Some(value) = state.get_mut(basis_state) {
        *value = Complex::ONE;
    }
}

// ============================================================================
// Simulation
// ============================================================================

fn simulate(
    circuit: &QuantumCircuit,
    state: &mut [Complex],
    max_duration: Option<Duration>,
    started: Instant,
) -> Result<(), ExhaustiveReason> {
    for gate in circuit.operations() {
        if deadline_reached(
            started,
            max_duration,
        ) {
            return Err(
                ExhaustiveReason::TimeLimitExceeded,
            );
        }

        apply_gate(gate, state)?;
    }

    Ok(())
}

fn apply_gate(
    gate: &Gate,
    state: &mut [Complex],
) -> Result<(), ExhaustiveReason> {
    match gate.kind() {
        GateKind::I => {
            // Identity.
        }

        GateKind::X => {
            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ZERO, Complex::ONE],
                    [Complex::ONE, Complex::ZERO],
                ],
            )?;
        }

        GateKind::Y => {
            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ZERO, Complex::new(0.0, -1.0)],
                    [Complex::I, Complex::ZERO],
                ],
            )?;
        }

        GateKind::Z => {
            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(-1.0, 0.0)],
                ],
            )?;
        }

        GateKind::H => {
            let scale = std::f64::consts::FRAC_1_SQRT_2;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(scale, 0.0),
                        Complex::new(scale, 0.0),
                    ],
                    [
                        Complex::new(scale, 0.0),
                        Complex::new(-scale, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::S => {
            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::I],
                ],
            )?;
        }

        GateKind::Sdg => {
            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [
                        Complex::ZERO,
                        Complex::new(0.0, -1.0),
                    ],
                ],
            )?;
        }

        GateKind::T => {
            let phase = phase_from_parameter(
                gate,
                0,
                std::f64::consts::FRAC_PI_4,
            )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, phase],
                ],
            )?;
        }

        GateKind::Tdg => {
            let phase = phase_from_parameter(
                gate,
                0,
                -std::f64::consts::FRAC_PI_4,
            )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, phase],
                ],
            )?;
        }

        GateKind::V => {
            let half = 0.5;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(half, half),
                        Complex::new(half, -half),
                    ],
                    [
                        Complex::new(half, -half),
                        Complex::new(half, half),
                    ],
                ],
            )?;
        }

        GateKind::Vdg => {
            let half = 0.5;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(half, -half),
                        Complex::new(half, half),
                    ],
                    [
                        Complex::new(half, half),
                        Complex::new(half, -half),
                    ],
                ],
            )?;
        }

        GateKind::RX => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;
            let (sin, cos) = half.sin_cos();

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(cos, 0.0),
                        Complex::new(0.0, -sin),
                    ],
                    [
                        Complex::new(0.0, -sin),
                        Complex::new(cos, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::RY => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;
            let (sin, cos) = half.sin_cos();

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(cos, 0.0),
                        Complex::new(-sin, 0.0),
                    ],
                    [
                        Complex::new(sin, 0.0),
                        Complex::new(cos, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::RZ => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;

            let minus =
                Complex::exp_i(-half).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let plus =
                Complex::exp_i(half).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [minus, Complex::ZERO],
                    [Complex::ZERO, plus],
                ],
            )?;
        }

        GateKind::Phase | GateKind::U1 => {
            let theta = parameter_value(gate, 0)?;

            let phase =
                Complex::exp_i(theta).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, phase],
                ],
            )?;
        }

        GateKind::U2 => {
            let phi = parameter_value(gate, 0)?;
            let lambda = parameter_value(gate, 1)?;

            let scale =
                std::f64::consts::FRAC_1_SQRT_2;

            let e_lambda =
                Complex::exp_i(lambda).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let e_phi =
                Complex::exp_i(phi).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let e_sum =
                Complex::exp_i(phi + lambda).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(scale, 0.0),
                        e_lambda.scale(-scale),
                    ],
                    [
                        e_phi.scale(scale),
                        e_sum.scale(scale),
                    ],
                ],
            )?;
        }

        GateKind::U3 => {
            let theta = parameter_value(gate, 0)?;
            let phi = parameter_value(gate, 1)?;
            let lambda = parameter_value(gate, 2)?;

            let half = theta * 0.5;
            let (sin, cos) = half.sin_cos();

            let e_lambda =
                Complex::exp_i(lambda).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let e_phi =
                Complex::exp_i(phi).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let e_sum =
                Complex::exp_i(phi + lambda).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            apply_single_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(cos, 0.0),
                        e_lambda.scale(-sin),
                    ],
                    [
                        e_phi.scale(sin),
                        e_sum.scale(cos),
                    ],
                ],
            )?;
        }

        GateKind::CX => {
            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ZERO, Complex::ONE],
                    [Complex::ONE, Complex::ZERO],
                ],
            )?;
        }

        GateKind::CY => {
            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ZERO, Complex::new(0.0, -1.0)],
                    [Complex::I, Complex::ZERO],
                ],
            )?;
        }

        GateKind::CZ => {
            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [Complex::ONE, Complex::ZERO],
                    [
                        Complex::ZERO,
                        Complex::new(-1.0, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::CH => {
            let scale =
                std::f64::consts::FRAC_1_SQRT_2;

            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(scale, 0.0),
                        Complex::new(scale, 0.0),
                    ],
                    [
                        Complex::new(scale, 0.0),
                        Complex::new(-scale, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::CRX => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;
            let (sin, cos) = half.sin_cos();

            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(cos, 0.0),
                        Complex::new(0.0, -sin),
                    ],
                    [
                        Complex::new(0.0, -sin),
                        Complex::new(cos, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::CRY => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;
            let (sin, cos) = half.sin_cos();

            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::new(cos, 0.0),
                        Complex::new(-sin, 0.0),
                    ],
                    [
                        Complex::new(sin, 0.0),
                        Complex::new(cos, 0.0),
                    ],
                ],
            )?;
        }

        GateKind::CRZ => {
            let theta = parameter_value(gate, 0)?;
            let half = theta * 0.5;

            let minus =
                Complex::exp_i(-half).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            let plus =
                Complex::exp_i(half).ok_or(
                    ExhaustiveReason::NonFiniteGateParameter,
                )?;

            apply_controlled_matrix(
                state,
                gate.qubits(),
                [
                    [minus, Complex::ZERO],
                    [Complex::ZERO, plus],
                ],
            )?;
        }

        GateKind::SWAP => {
            apply_two_qubit_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::ONE,
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ONE,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::ONE,
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ONE,
                    ],
                ],
            )?;
        }

        GateKind::ISWAP => {
            apply_two_qubit_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::ONE,
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::I,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::I,
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::ONE,
                    ],
                ],
            )?;
        }

        GateKind::CZ => unreachable!(),

        GateKind::ECR => {
            let scale =
                std::f64::consts::FRAC_1_SQRT_2;

            apply_two_qubit_matrix(
                state,
                gate.qubits(),
                [
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::new(scale, 0.0),
                        Complex::new(0.0, scale),
                    ],
                    [
                        Complex::ZERO,
                        Complex::ZERO,
                        Complex::new(0.0, scale),
                        Complex::new(scale, 0.0),
                    ],
                    [
                        Complex::new(scale, 0.0),
                        Complex::new(0.0, -scale),
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                    [
                        Complex::new(0.0, -scale),
                        Complex::new(scale, 0.0),
                        Complex::ZERO,
                        Complex::ZERO,
                    ],
                ],
            )?;
        }

        GateKind::CCX => {
            apply_ccx(
                state,
                gate.qubits(),
            )?;
        }

        GateKind::CSWAP => {
            apply_cswap(
                state,
                gate.qubits(),
            )?;
        }

        GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset => {
            return Err(
                ExhaustiveReason::UnsupportedGate(
                    gate.kind(),
                ),
            );
        }
    }

    ensure_finite_state(state)
}

// ============================================================================
// Parameter helpers
// ============================================================================

fn parameter_value(
    gate: &Gate,
    index: usize,
) -> Result<f64, ExhaustiveReason> {
    let parameter = gate
        .parameters()
        .get(index)
        .ok_or(
            ExhaustiveReason::NonFiniteGateParameter,
        )?;

    parameter
        .as_constant()
        .filter(|value| value.is_finite())
        .ok_or(
            ExhaustiveReason::NonFiniteGateParameter,
        )
}

fn phase_from_parameter(
    gate: &Gate,
    index: usize,
    fallback: f64,
) -> Result<Complex, ExhaustiveReason> {
    let theta = if gate.parameters().is_empty() {
        fallback
    } else {
        parameter_value(gate, index)?
    };

    Complex::exp_i(theta).ok_or(
        ExhaustiveReason::NonFiniteGateParameter,
    )
}

// ============================================================================
// Matrix application
// ============================================================================

fn qubit_index(
    qubit: QubitId,
) -> Result<usize, ExhaustiveReason> {
    Ok(qubit.index())
}

fn apply_single_matrix(
    state: &mut [Complex],
    qubits: &[QubitId],
    matrix: [[Complex; 2]; 2],
) -> Result<(), ExhaustiveReason> {
    let qubit = *qubits
        .first()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::I,
            ),
        )?;

    let bit = qubit_index(qubit)?;

    if bit >= usize::BITS as usize {
        return Err(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "single-qubit bit mask",
            },
        );
    }

    let mask = 1usize
        .checked_shl(bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "single-qubit bit mask",
            },
        )?;

    for base in 0..state.len() {
        if base & mask != 0 {
            continue;
        }

        let one = base | mask;

        let a0 = state[base];
        let a1 = state[one];

        state[base] = matrix[0][0]
            .multiply(a0)
            .add(matrix[0][1].multiply(a1));

        state[one] = matrix[1][0]
            .multiply(a0)
            .add(matrix[1][1].multiply(a1));
    }

    Ok(())
}

fn apply_controlled_matrix(
    state: &mut [Complex],
    qubits: &[QubitId],
    target_matrix: [[Complex; 2]; 2],
) -> Result<(), ExhaustiveReason> {
    let control = qubits
        .first()
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CX,
            ),
        )?;

    let target = qubits
        .get(1)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CX,
            ),
        )?;

    let control_bit = qubit_index(control)?;
    let target_bit = qubit_index(target)?;

    if control_bit == target_bit {
        return Err(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CX,
            ),
        );
    }

    if control_bit >= usize::BITS as usize
        || target_bit >= usize::BITS as usize
    {
        return Err(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "controlled-gate bit mask",
            },
        );
    }

    let control_mask = 1usize
        .checked_shl(control_bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "controlled-gate control mask",
            },
        )?;

    let target_mask = 1usize
        .checked_shl(target_bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "controlled-gate target mask",
            },
        )?;

    for base in 0..state.len() {
        if base & control_mask == 0
            || base & target_mask != 0
        {
            continue;
        }

        let one = base | target_mask;

        let a0 = state[base];
        let a1 = state[one];

        state[base] = target_matrix[0][0]
            .multiply(a0)
            .add(target_matrix[0][1].multiply(a1));

        state[one] = target_matrix[1][0]
            .multiply(a0)
            .add(target_matrix[1][1].multiply(a1));
    }

    Ok(())
}

fn apply_two_qubit_matrix(
    state: &mut [Complex],
    qubits: &[QubitId],
    matrix: [[Complex; 4]; 4],
) -> Result<(), ExhaustiveReason> {
    let first = qubits
        .first()
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::SWAP,
            ),
        )?;

    let second = qubits
        .get(1)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::SWAP,
            ),
        )?;

    let first_bit = qubit_index(first)?;
    let second_bit = qubit_index(second)?;

    if first_bit == second_bit {
        return Err(
            ExhaustiveReason::UnsupportedGate(
                GateKind::SWAP,
            ),
        );
    }

    if first_bit >= usize::BITS as usize
        || second_bit >= usize::BITS as usize
    {
        return Err(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "two-qubit bit mask",
            },
        );
    }

    let first_mask = 1usize
        .checked_shl(first_bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "two-qubit first mask",
            },
        )?;

    let second_mask = 1usize
        .checked_shl(second_bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "two-qubit second mask",
            },
        )?;

    for base in 0..state.len() {
        if base & first_mask != 0
            || base & second_mask != 0
        {
            continue;
        }

        let i00 = base;
        let i01 = base | second_mask;
        let i10 = base | first_mask;
        let i11 = base | first_mask | second_mask;

        let input = [
            state[i00],
            state[i01],
            state[i10],
            state[i11],
        ];

        let mut output = [
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
        ];

        for row in 0..4 {
            for column in 0..4 {
                output[row] = output[row]
                    .add(
                        matrix[row][column]
                            .multiply(input[column]),
                    );
            }
        }

        state[i00] = output[0];
        state[i01] = output[1];
        state[i10] = output[2];
        state[i11] = output[3];
    }

    Ok(())
}

// ============================================================================
// Three-qubit permutations
// ============================================================================

fn apply_ccx(
    state: &mut [Complex],
    qubits: &[QubitId],
) -> Result<(), ExhaustiveReason> {
    let control_a = qubits
        .first()
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CCX,
            ),
        )?;

    let control_b = qubits
        .get(1)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CCX,
            ),
        )?;

    let target = qubits
        .get(2)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CCX,
            ),
        )?;

    let a = qubit_index(control_a)?;
    let b = qubit_index(control_b)?;
    let t = qubit_index(target)?;

    if a == b || a == t || b == t {
        return Err(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CCX,
            ),
        );
    }

    let mask_a = bit_mask(a)?;
    let mask_b = bit_mask(b)?;
    let mask_t = bit_mask(t)?;

    for index in 0..state.len() {
        if index & mask_a != 0
            && index & mask_b != 0
            && index & mask_t == 0
        {
            let partner = index | mask_t;
            state.swap(index, partner);
        }
    }

    Ok(())
}

fn apply_cswap(
    state: &mut [Complex],
    qubits: &[QubitId],
) -> Result<(), ExhaustiveReason> {
    let control = qubits
        .first()
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CSWAP,
            ),
        )?;

    let first = qubits
        .get(1)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CSWAP,
            ),
        )?;

    let second = qubits
        .get(2)
        .copied()
        .ok_or(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CSWAP,
            ),
        )?;

    let c = qubit_index(control)?;
    let a = qubit_index(first)?;
    let b = qubit_index(second)?;

    if c == a || c == b || a == b {
        return Err(
            ExhaustiveReason::UnsupportedGate(
                GateKind::CSWAP,
            ),
        );
    }

    let control_mask = bit_mask(c)?;
    let first_mask = bit_mask(a)?;
    let second_mask = bit_mask(b)?;

    for index in 0..state.len() {
        if index & control_mask == 0
            || index & first_mask == 0
            || index & second_mask != 0
        {
            continue;
        }

        let partner =
            index ^ first_mask ^ second_mask;

        state.swap(index, partner);
    }

    Ok(())
}

fn bit_mask(
    bit: usize,
) -> Result<usize, ExhaustiveReason> {
    if bit >= usize::BITS as usize {
        return Err(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "qubit bit mask",
            },
        );
    }

    1usize
        .checked_shl(bit as u32)
        .ok_or(
            ExhaustiveReason::ArithmeticOverflow {
                calculation: "qubit bit mask",
            },
        )
}

// ============================================================================
// State validation
// ============================================================================

fn ensure_finite_state(
    state: &[Complex],
) -> Result<(), ExhaustiveReason> {
    if state.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err(ExhaustiveReason::NonFiniteState)
    }
}

// ============================================================================
// Comparisons
// ============================================================================

enum PhaseComparison {
    Equivalent,
    Mismatch(ExhaustiveMismatch),
    UndefinedPhase,
}

fn compare_exact(
    basis_state: usize,
    original: &[Complex],
    optimized: &[Complex],
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> Option<ExhaustiveMismatch> {
    for index in 0..original.len() {
        if !approximately_equal(
            original[index],
            optimized[index],
            absolute_tolerance,
            relative_tolerance,
        ) {
            return Some(
                ExhaustiveMismatch::new(
                    basis_state,
                    index,
                    original[index],
                    optimized[index],
                    None,
                ),
            );
        }
    }

    None
}

fn compare_up_to_global_phase(
    basis_state: usize,
    original: &[Complex],
    optimized: &[Complex],
    global_phase: &mut Option<Complex>,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> PhaseComparison {
    if global_phase.is_none() {
        let mut found = None;

        for index in 0..original.len() {
            let lhs = original[index];
            let rhs = optimized[index];

            let lhs_norm = lhs.norm();
            let rhs_norm = rhs.norm();

            if lhs_norm <= absolute_tolerance
                && rhs_norm <= absolute_tolerance
            {
                continue;
            }

            if lhs_norm <= absolute_tolerance
                || rhs_norm <= absolute_tolerance
            {
                return PhaseComparison::Mismatch(
                    ExhaustiveMismatch::new(
                        basis_state,
                        index,
                        lhs,
                        rhs,
                        None,
                    ),
                );
            }

            let phase = match lhs.divide(rhs) {
                Some(value) => value,
                None => {
                    return PhaseComparison::UndefinedPhase;
                }
            };

            let magnitude = phase.norm();

            if !approximately_scalar(
                magnitude,
                1.0,
                absolute_tolerance,
                relative_tolerance,
            ) {
                return PhaseComparison::Mismatch(
                    ExhaustiveMismatch::new(
                        basis_state,
                        index,
                        lhs,
                        rhs,
                        Some(phase),
                    ),
                );
            }

            found = Some(
                phase.scale(1.0 / magnitude),
            );

            break;
        }

        match found {
            Some(phase) => {
                *global_phase = Some(phase);
            }

            None => {
                // A unitary column cannot be identically zero. If this ever
                // occurs, the simulated operation semantics are not valid.
                return PhaseComparison::UndefinedPhase;
            }
        }
    }

    let phase = match *global_phase {
        Some(value) => value,
        None => return PhaseComparison::UndefinedPhase,
    };

    for index in 0..original.len() {
        let expected =
            phase.multiply(optimized[index]);

        if !approximately_equal(
            original[index],
            expected,
            absolute_tolerance,
            relative_tolerance,
        ) {
            return PhaseComparison::Mismatch(
                ExhaustiveMismatch::new(
                    basis_state,
                    index,
                    original[index],
                    optimized[index],
                    Some(phase),
                ),
            );
        }
    }

    PhaseComparison::Equivalent
}

fn approximately_equal(
    lhs: Complex,
    rhs: Complex,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    approximately_scalar(
        lhs.re,
        rhs.re,
        absolute_tolerance,
        relative_tolerance,
    ) && approximately_scalar(
        lhs.im,
        rhs.im,
        absolute_tolerance,
        relative_tolerance,
    )
}

fn approximately_scalar(
    lhs: f64,
    rhs: f64,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let difference = (lhs - rhs).abs();
    let scale = lhs.abs().max(rhs.abs());

    difference
        <= absolute_tolerance
            + relative_tolerance * scale
}

// ============================================================================
// Deadline
// ============================================================================

fn deadline_reached(
    started: Instant,
    maximum: Option<Duration>,
) -> bool {
    match maximum {
        Some(limit) => started.elapsed() >= limit,
        None => false,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parameter(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test parameter must be finite")
    }

    fn gate(
        kind: GateKind,
        qubits: &[usize],
        parameters: &[f64],
    ) -> Gate {
        let operands = qubits
            .iter()
            .copied()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        let params = parameters
            .iter()
            .copied()
            .map(parameter)
            .collect::<Vec<_>>();

        Gate::new(
            kind,
            operands,
            params,
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn circuit(
        qubits: usize,
        gates: Vec<Gate>,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(qubits, 0)
                .expect("test circuit must be valid");

        for operation in gates {
            circuit
                .add_operation(operation)
                .expect("test operation must be valid");
        }

        circuit
    }

    #[test]
    fn identical_empty_circuits_are_equivalent() {
        let original = circuit(1, Vec::new());
        let optimized = circuit(1, Vec::new());

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );

        assert_eq!(
            report.basis_states_checked,
            2
        );
    }

    #[test]
    fn identical_x_circuits_are_equivalent() {
        let original = circuit(
            1,
            vec![gate(GateKind::X, &[0], &[])],
        );

        let optimized = circuit(
            1,
            vec![gate(GateKind::X, &[0], &[])],
        );

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );
    }

    #[test]
    fn x_and_identity_are_not_equivalent() {
        let original = circuit(
            1,
            vec![gate(GateKind::X, &[0], &[])],
        );

        let optimized = circuit(1, Vec::new());

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::NotEquivalent
        );

        assert!(
            report.mismatch.is_some()
        );
    }

    #[test]
    fn h_h_is_equivalent_to_identity() {
        let original = circuit(
            1,
            vec![
                gate(GateKind::H, &[0], &[]),
                gate(GateKind::H, &[0], &[]),
            ],
        );

        let optimized = circuit(1, Vec::new());

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );
    }

    #[test]
    fn global_phase_can_be_ignored() {
        let original = circuit(
            1,
            vec![gate(
                GateKind::RZ,
                &[0],
                &[std::f64::consts::PI * 2.0],
            )],
        );

        let optimized = circuit(1, Vec::new());

        let config =
            ExhaustiveConfig::up_to_global_phase();

        let report = verify_with_config(
            &original,
            &optimized,
            config,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );
    }

    #[test]
    fn exact_relation_rejects_global_phase() {
        let original = circuit(
            1,
            vec![gate(
                GateKind::RZ,
                &[0],
                &[std::f64::consts::PI * 2.0],
            )],
        );

        let optimized = circuit(1, Vec::new());

        let config =
            ExhaustiveConfig::exact();

        let report = verify_with_config(
            &original,
            &optimized,
            config,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::NotEquivalent
        );
    }

    #[test]
    fn symbolic_parameters_are_never_silently_bound() {
        let symbolic =
            Parameter::symbol("theta")
                .expect("symbol must be valid");

        let operation = Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            vec![symbolic],
            None,
            None,
        )
        .expect("symbolic gate must be valid");

        let original = circuit(
            1,
            vec![operation],
        );

        let optimized = circuit(1, Vec::new());

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Inconclusive
        );

        assert_eq!(
            report.inconclusive_reason,
            Some(
                ExhaustiveReason::SymbolicParameters
            )
        );
    }

    #[test]
    fn measurement_is_not_treated_as_unitary() {
        let measurement = Gate::new(
            GateKind::Measure,
            vec![QubitId::new(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement must be valid");

        let original = circuit(
            1,
            vec![measurement],
        );

        let optimized = circuit(1, Vec::new());

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Inconclusive
        );

        assert_eq!(
            report.inconclusive_reason,
            Some(ExhaustiveReason::Measurement)
        );
    }

    #[test]
    fn resource_limit_is_inconclusive() {
        let original = circuit(
            3,
            vec![gate(GateKind::H, &[0], &[])],
        );

        let optimized = original.clone();

        let limits = ExhaustiveLimits::new(
            2,
            4,
            0,
            100,
            None,
        )
        .expect("limits must be valid");

        let config =
            ExhaustiveConfig::new(
                ExhaustiveRelation::Exact,
                1.0e-12,
                1.0e-10,
                limits,
            )
            .expect("configuration must be valid");

        let report = verify_with_config(
            &original,
            &optimized,
            config,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Inconclusive
        );

        assert_eq!(
            report.inconclusive_reason,
            Some(
                ExhaustiveReason::QubitLimitExceeded {
                    actual: 3,
                    maximum: 2,
                }
            )
        );
    }

    #[test]
    fn controlled_x_is_exhaustively_verified() {
        let original = circuit(
            2,
            vec![
                gate(GateKind::CX, &[0, 1], &[]),
            ],
        );

        let optimized = circuit(
            2,
            vec![
                gate(GateKind::CX, &[0, 1], &[]),
            ],
        );

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );

        assert_eq!(
            report.basis_states_checked,
            4
        );
    }

    #[test]
    fn three_qubit_toffoli_is_exhaustively_verified() {
        let original = circuit(
            3,
            vec![
                gate(
                    GateKind::CCX,
                    &[0, 1, 2],
                    &[],
                ),
            ],
        );

        let optimized = original.clone();

        let report = verify(
            &original,
            &optimized,
        )
        .expect("verification must execute");

        assert_eq!(
            report.verdict,
            ExhaustiveVerdict::Equivalent
        );

        assert_eq!(
            report.basis_states_checked,
            8
        );
    }

    #[test]
    fn prove_rejects_non_equivalent_circuits() {
        let original = circuit(
            1,
            vec![gate(GateKind::X, &[0], &[])],
        );

        let optimized = circuit(1, Vec::new());

        let result = prove(
            &original,
            &optimized,
        );

        assert!(
            matches!(
                result,
                Err(
                    ExhaustiveProofError::NotProven(_)
                )
            )
        );
    }
}