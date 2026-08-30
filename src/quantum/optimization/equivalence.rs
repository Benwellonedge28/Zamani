//! Zamani Quantum Optimization — Circuit Equivalence
//!
//! Production-grade semantic and structural equivalence checking for the
//! canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                     crate::quantum::ir
//!                              │
//!                              ▼
//!                  optimization::equivalence
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!        structural        exact-unitary     future
//!        comparison        comparison        verifiers
//!                              │
//!                              ▼
//!                     EquivalenceVerdict
//! ```
//!
//! This module is deliberately independent from:
//!
//! - optimization passes;
//! - rewrite rules;
//! - e-graphs;
//! - routing;
//! - scheduling;
//! - hardware backends;
//! - QPU execution;
//! - benchmarking;
//! - frontend parsing;
//! - algorithm implementations;
//! - error-correction implementations.
//!
//! It consumes only the canonical `crate::quantum::ir` representation.
//!
//! # Core safety rule
//!
//! This module MUST NEVER report two circuits as equivalent merely because
//! exact verification was too expensive or unsupported.
//!
//! `Inconclusive` is a first-class result.
//!
//! Therefore:
//!
//! ```text
//! Equivalent
//!     = proven equivalent
//!
//! NotEquivalent
//!     = proven different
//!
//! Inconclusive
//!     = insufficient verification capability/resources
//! ```
//!
//! This distinction is essential for a production compiler.
//!
//! # Supported equivalence relations
//!
//! The verifier supports:
//!
//! - exact structural equivalence;
//! - exact unitary equivalence;
//! - exact unitary equivalence up to global phase;
//! - configurable numerical tolerance;
//! - deterministic bounded exact verification.
//!
//! Non-unitary circuits containing measurement, reset, or other unsupported
//! semantic operations are never incorrectly classified as unitary-equivalent.
//! They return `Inconclusive` unless they are structurally identical.
//!
//! # Scalability
//!
//! Quantum equivalence is computationally hard in the general case. Exact
//! unitary verification requires resources exponential in the number of
//! logical qubits for the generic dense method implemented here.
//!
//! Consequently this module scales by:
//!
//! - avoiding unnecessary allocations;
//! - rejecting impossible resource requests before allocation;
//! - using checked arithmetic everywhere;
//! - allowing the caller to configure maximum qubits, amplitudes, basis
//!   evaluations, and numerical tolerance;
//! - using structural comparison as an O(n) fast path;
//! - using global-phase-aware exact verification;
//! - returning `Inconclusive` instead of exhausting the process;
//! - leaving room for future stabilizer, tensor-network, decision-diagram,
//!   randomized, and certificate-backed verifiers.
//!
//! "Unlimited" circuit size therefore means:
//!
//! > the verifier can safely process any circuit size permitted by the
//! > configured resources and available machine resources without an internal
//! > fixed architectural circuit-size ceiling.
//!
//! It does NOT mean exponential quantum equivalence becomes polynomial.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are used.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Integration contract
//!
//! `pipeline.rs` should call [`verify`] after optimization when verification
//! is enabled.
//!
//! `verification/semantic.rs` may later delegate to this module for exact
//! logical-unitary verification.
//!
//! `verification/randomized.rs` may provide probabilistic verification for
//! cases returned as [`EquivalenceVerdict::Inconclusive`].
//!
//! `verification/exhaustive.rs` may use the same public result contract for
//! specialized exhaustive methods.
//!
//! `verification/certificates.rs` may record [`EquivalenceReport`] without
//! changing this module's public semantic model.
//!
//! `result.rs` can store [`EquivalenceReport`] as part of an optimization
//! result.
//!
//! `provenance.rs` can record the verifier configuration and verdict.
//!
//! No future optimizer pass should need to modify this file merely because a
//! new optimization transformation is introduced.
//!
//! -----------------------------------------------------------------------------
//! IMPORTANT SEMANTIC CONTRACT
//! -----------------------------------------------------------------------------
//!
//! A verifier must not assume that:
//!
//!     same gate count == same circuit
//!     same depth == same circuit
//!     same hash == same circuit
//!     same measurement count == same circuit
//!
//! These are fingerprints/heuristics, not semantic proofs.
//!
//! Structural equality is a proof of exact structural identity.
//!
//! Exact unitary comparison is a proof of unitary equality under the selected
//! equivalence relation and numerical tolerance.
//!
//! Everything else must remain explicitly non-conclusive.
//!
//! -----------------------------------------------------------------------------
//! Current canonical IR integration
//! -----------------------------------------------------------------------------
//!
//! The canonical gate representation currently provides:
//!
//! - GateKind;
//! - qubit operands;
//! - numerical/symbolic parameters;
//! - classical measurement destinations;
//! - measurement payloads;
//! - unitary/non-unitary classification;
//! - circuit validation.
//!
//! Parameter expressions remain symbolic unless the caller supplies a resolver.
//! This module therefore does not silently assign values to symbols.
//!
//! -----------------------------------------------------------------------------
//! Future extension points
//! -----------------------------------------------------------------------------
//!
//! The public model intentionally permits future verification engines:
//!
//!     ExactDense
//!     Stabilizer
//!     TensorNetwork
//!     DecisionDiagram
//!     Randomized
//!     Symbolic
//!     Certificate
//!
//! without changing `EquivalenceVerdict`.
//!
//! The current implementation uses `ExactDense` for supported constant
//! unitary circuits.

use std::fmt;
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

// ============================================================================
// Public configuration
// ============================================================================

/// Numerical comparison policy used by semantic equivalence verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquivalenceTolerance {
    /// Absolute amplitude tolerance.
    pub absolute: f64,

    /// Relative amplitude tolerance.
    pub relative: f64,
}

impl EquivalenceTolerance {
    /// Creates a validated tolerance.
    pub fn new(
        absolute: f64,
        relative: f64,
    ) -> Result<Self, EquivalenceError> {
        if !absolute.is_finite() || absolute < 0.0 {
            return Err(EquivalenceError::InvalidTolerance {
                field: "absolute",
                value: absolute,
            });
        }

        if !relative.is_finite() || relative < 0.0 {
            return Err(EquivalenceError::InvalidTolerance {
                field: "relative",
                value: relative,
            });
        }

        Ok(Self {
            absolute,
            relative,
        })
    }

    /// Exact floating-point comparison policy.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            absolute: 0.0,
            relative: 0.0,
        }
    }

    /// A conservative floating-point policy suitable for dense simulation.
    #[must_use]
    pub const fn numerical() -> Self {
        Self {
            absolute: 1.0e-12,
            relative: 1.0e-10,
        }
    }

    fn accepts(self, lhs: f64, rhs: f64) -> bool {
        if lhs == rhs {
            return true;
        }

        let difference = (lhs - rhs).abs();
        let scale = lhs.abs().max(rhs.abs());

        difference <= self.absolute + self.relative * scale
    }
}

impl Default for EquivalenceTolerance {
    fn default() -> Self {
        Self::numerical()
    }
}

/// Resource limits for one equivalence-verification invocation.
///
/// These limits are verifier-local. They do not replace the canonical IR
/// resource policy or the optimizer-wide `OptimizationLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquivalenceLimits {
    /// Maximum number of logical qubits permitted for dense exact verification.
    pub max_qubits: usize,

    /// Maximum number of state amplitudes that may be allocated.
    pub max_amplitudes: usize,

    /// Maximum number of basis-state columns evaluated by exact unitary
    /// comparison.
    ///
    /// A value of `0` means "derive from max_qubits".
    pub max_basis_states: usize,

    /// Maximum number of quantum operations inspected during semantic
    /// verification.
    pub max_operations: usize,

    /// Maximum wall-clock duration for one verification request.
    ///
    /// `None` means no verifier-local wall-clock deadline.
    pub max_duration: Option<Duration>,
}

impl EquivalenceLimits {
    /// Conservative default suitable for compiler verification without
    /// accidentally allocating enormous dense state vectors.
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

    /// Creates an intentionally larger policy.
    ///
    /// This does not bypass machine memory limits. Allocation failures are
    /// represented as verification errors rather than being allowed to become
    /// uncontrolled process failures.
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

    /// Creates a custom policy.
    pub fn new(
        max_qubits: usize,
        max_amplitudes: usize,
        max_basis_states: usize,
        max_operations: usize,
        max_duration: Option<Duration>,
    ) -> Result<Self, EquivalenceError> {
        if max_qubits == 0 {
            return Err(EquivalenceError::InvalidLimit {
                field: "max_qubits",
                value: 0,
            });
        }

        if max_amplitudes == 0 {
            return Err(EquivalenceError::InvalidLimit {
                field: "max_amplitudes",
                value: 0,
            });
        }

        if max_operations == 0 {
            return Err(EquivalenceError::InvalidLimit {
                field: "max_operations",
                value: 0,
            });
        }

        if let Some(duration) = max_duration {
            if duration.is_zero() {
                return Err(EquivalenceError::InvalidLimit {
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

impl Default for EquivalenceLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

/// Relation used when comparing unitary circuits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitaryRelation {
    /// Require U == V.
    Exact,

    /// Permit U == exp(i*phi) V for one circuit-wide global phase.
    UpToGlobalPhase,
}

impl Default for UnitaryRelation {
    fn default() -> Self {
        Self::UpToGlobalPhase
    }
}

/// Verification method requested by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquivalenceMethod {
    /// Compare canonical IR structure exactly.
    Structural,

    /// Compare complete unitary action using exact dense simulation.
    ExactUnitary {
        /// Whether a circuit-wide global phase is ignored.
        relation: UnitaryRelation,
    },

    /// Select the strongest safe method automatically.
    Auto {
        /// Unitary relation to use when exact dense verification is possible.
        relation: UnitaryRelation,
    },
}

impl Default for EquivalenceMethod {
    fn default() -> Self {
        Self::Auto {
            relation: UnitaryRelation::UpToGlobalPhase,
        }
    }
}

/// Complete verification configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquivalenceConfig {
    /// Verification method.
    pub method: EquivalenceMethod,

    /// Numerical tolerance for semantic comparison.
    pub tolerance: EquivalenceTolerance,

    /// Resource limits.
    pub limits: EquivalenceLimits,
}

impl Default for EquivalenceConfig {
    fn default() -> Self {
        Self {
            method: EquivalenceMethod::default(),
            tolerance: EquivalenceTolerance::default(),
            limits: EquivalenceLimits::default(),
        }
    }
}

// ============================================================================
// Public verdict
// ============================================================================

/// Final semantic verification verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquivalenceVerdict {
    /// Equivalence was proven.
    Equivalent,

    /// Non-equivalence was proven.
    NotEquivalent,

    /// The verifier could not safely establish either relation.
    ///
    /// This is NOT an error. It means a stronger or different verification
    /// engine is required.
    Inconclusive,
}

impl EquivalenceVerdict {
    /// Returns true only when equivalence was actually proven.
    #[must_use]
    pub const fn is_equivalent(self) -> bool {
        matches!(self, Self::Equivalent)
    }

    /// Returns true only when non-equivalence was actually proven.
    #[must_use]
    pub const fn is_not_equivalent(self) -> bool {
        matches!(self, Self::NotEquivalent)
    }

    /// Returns true when no definitive semantic conclusion was established.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }
}

/// Why an equivalence request could not be decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InconclusiveReason {
    /// The circuit contains symbolic parameters without a supplied binding.
    SymbolicParameters,

    /// The circuit contains non-unitary operations not handled by the exact
    /// unitary verifier.
    NonUnitaryCircuit,

    /// The gate kind is not implemented by the dense verifier.
    UnsupportedGate(GateKind),

    /// The circuit exceeds the verifier's configured qubit budget.
    QubitLimitExceeded {
        /// Number of logical qubits.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The state vector would exceed the configured amplitude budget.
    AmplitudeLimitExceeded {
        /// Required number of amplitudes.
        required: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The required number of basis-state evaluations exceeds the configured
    /// budget.
    BasisStateLimitExceeded {
        /// Required basis states.
        required: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Operation inspection limit reached.
    OperationLimitExceeded {
        /// Number of operations.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Verification deadline reached.
    TimeLimitExceeded,

    /// The selected method does not support this semantic relation.
    UnsupportedRelation,

    /// The input circuits use incompatible logical namespaces.
    IncompatibleShape,
}

impl fmt::Display for InconclusiveReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolicParameters => {
                f.write_str("symbolic parameters are not bound")
            }

            Self::NonUnitaryCircuit => {
                f.write_str(
                    "exact dense unitary verification does not cover non-unitary operations",
                )
            }

            Self::UnsupportedGate(gate) => {
                write!(f, "dense verifier does not implement gate {gate:?}")
            }

            Self::QubitLimitExceeded { actual, maximum } => {
                write!(
                    f,
                    "logical-qubit verification limit exceeded: {actual} > {maximum}"
                )
            }

            Self::AmplitudeLimitExceeded { required, maximum } => {
                write!(
                    f,
                    "state-vector amplitude limit exceeded: {required} > {maximum}"
                )
            }

            Self::BasisStateLimitExceeded { required, maximum } => {
                write!(
                    f,
                    "basis-state verification limit exceeded: {required} > {maximum}"
                )
            }

            Self::OperationLimitExceeded { actual, maximum } => {
                write!(
                    f,
                    "verification operation limit exceeded: {actual} > {maximum}"
                )
            }

            Self::TimeLimitExceeded => {
                f.write_str("equivalence verification time limit exceeded")
            }

            Self::UnsupportedRelation => {
                f.write_str("requested equivalence relation is unsupported")
            }

            Self::IncompatibleShape => {
                f.write_str("circuits have incompatible logical namespaces")
            }
        }
    }
}

/// Structured explanation of a non-equivalence result.
#[derive(Debug, Clone, PartialEq)]
pub enum Difference {
    /// Circuits have different logical qubit counts.
    QubitCount {
        left: usize,
        right: usize,
    },

    /// Circuits have different classical-bit counts.
    ClassicalBitCount {
        left: usize,
        right: usize,
    },

    /// Structural operation mismatch.
    StructuralOperation {
        index: usize,
    },

    /// Unitary action differs for one computational basis input.
    UnitaryAction {
        basis_state: usize,
        /// Maximum component-wise numerical difference observed.
        max_error: f64,
    },

    /// Unitary dimensions differ.
    UnitaryDimension {
        left: usize,
        right: usize,
    },

    /// The circuits differ only by a global phase when exact equality was
    /// requested.
    GlobalPhaseOnly {
        phase_real: f64,
        phase_imag: f64,
    },
}

impl fmt::Display for Difference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitCount { left, right } => {
                write!(f, "logical qubit count differs: {left} != {right}")
            }

            Self::ClassicalBitCount { left, right } => {
                write!(
                    f,
                    "classical-bit count differs: {left} != {right}"
                )
            }

            Self::StructuralOperation { index } => {
                write!(f, "operation differs at index {index}")
            }

            Self::UnitaryAction {
                basis_state,
                max_error,
            } => {
                write!(
                    f,
                    "unitary action differs for basis state {basis_state}; maximum error {max_error:e}"
                )
            }

            Self::UnitaryDimension { left, right } => {
                write!(f, "unitary dimensions differ: {left} != {right}")
            }

            Self::GlobalPhaseOnly {
                phase_real,
                phase_imag,
            } => {
                write!(
                    f,
                    "circuits differ only by global phase ({phase_real:e}, {phase_imag:e}i)"
                )
            }
        }
    }
}

/// Complete result of an equivalence request.
#[derive(Debug, Clone, PartialEq)]
pub struct EquivalenceReport {
    /// Final verdict.
    pub verdict: EquivalenceVerdict,

    /// Method actually used.
    pub method: EquivalenceMethod,

    /// Detailed inconclusive reason, when applicable.
    pub inconclusive_reason: Option<InconclusiveReason>,

    /// Proven difference, when applicable.
    pub difference: Option<Difference>,

    /// Whether the input circuits were structurally identical.
    pub structurally_equal: bool,

    /// Number of logical qubits.
    pub qubits: usize,

    /// Number of operations in the left circuit.
    pub left_operations: usize,

    /// Number of operations in the right circuit.
    pub right_operations: usize,

    /// Maximum numerical error observed by semantic comparison.
    pub max_error: f64,

    /// Global phase discovered during global-phase-aware comparison.
    ///
    /// `(real, imaginary)`.
    pub global_phase: Option<(f64, f64)>,

    /// SHA-256 structural fingerprint of the left circuit.
    pub left_fingerprint: [u8; 32],

    /// SHA-256 structural fingerprint of the right circuit.
    pub right_fingerprint: [u8; 32],

    /// Verification duration.
    pub elapsed: Duration,
}

impl EquivalenceReport {
    /// Returns true only when equivalence was proven.
    #[must_use]
    pub const fn is_equivalent(&self) -> bool {
        self.verdict.is_equivalent()
    }

    /// Returns true only when non-equivalence was proven.
    #[must_use]
    pub const fn is_not_equivalent(&self) -> bool {
        self.verdict.is_not_equivalent()
    }

    /// Returns true when verification could not decide.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.verdict.is_inconclusive()
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors that prevent a verification request from being executed.
#[derive(Debug, Clone, PartialEq)]
pub enum EquivalenceError {
    /// A supplied tolerance is invalid.
    InvalidTolerance {
        field: &'static str,
        value: f64,
    },

    /// A supplied verifier limit is invalid.
    InvalidLimit {
        field: &'static str,
        value: usize,
    },

    /// A circuit failed canonical IR validation.
    InvalidCircuit {
        side: &'static str,
        message: String,
    },

    /// Arithmetic overflow was detected.
    ArithmeticOverflow {
        calculation: &'static str,
    },

    /// A state-vector allocation was impossible under the configured policy.
    AllocationRejected {
        amplitudes: usize,
    },
}

impl fmt::Display for EquivalenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTolerance { field, value } => {
                write!(
                    f,
                    "invalid equivalence tolerance `{field}`: {value}"
                )
            }

            Self::InvalidLimit { field, value } => {
                write!(
                    f,
                    "invalid equivalence limit `{field}`: {value}"
                )
            }

            Self::InvalidCircuit { side, message } => {
                write!(
                    f,
                    "{side} circuit failed canonical IR validation: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    f,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::AllocationRejected { amplitudes } => {
                write!(
                    f,
                    "state-vector allocation rejected for {amplitudes} amplitudes"
                )
            }
        }
    }
}

impl std::error::Error for EquivalenceError {}

// ============================================================================
// Public entry points
// ============================================================================

/// Verifies equivalence using the supplied configuration.
///
/// This is the main compiler-facing API.
///
/// The function validates both canonical IR inputs before verification.
/// Structural equality is checked first because it is exact and inexpensive.
/// `Auto` then attempts exact dense unitary verification when the circuits are
/// suitable.
pub fn verify(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
    config: EquivalenceConfig,
) -> Result<EquivalenceReport, EquivalenceError> {
    let started = Instant::now();

    validate_input("left", left)?;
    validate_input("right", right)?;

    let left_fingerprint = structural_fingerprint(left);
    let right_fingerprint = structural_fingerprint(right);

    let structurally_equal = left == right;

    if structurally_equal {
        return Ok(EquivalenceReport {
            verdict: EquivalenceVerdict::Equivalent,
            method: EquivalenceMethod::Structural,
            inconclusive_reason: None,
            difference: None,
            structurally_equal: true,
            qubits: left.num_qubits(),
            left_operations: left.operations().len(),
            right_operations: right.operations().len(),
            max_error: 0.0,
            global_phase: None,
            left_fingerprint,
            right_fingerprint,
            elapsed: started.elapsed(),
        });
    }

    if left.num_qubits() != right.num_qubits() {
        return Ok(report_not_equivalent(
            started,
            EquivalenceMethod::Structural,
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            Difference::QubitCount {
                left: left.num_qubits(),
                right: right.num_qubits(),
            },
        ));
    }

    if left.num_classical_bits() != right.num_classical_bits() {
        return Ok(report_not_equivalent(
            started,
            EquivalenceMethod::Structural,
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            Difference::ClassicalBitCount {
                left: left.num_classical_bits(),
                right: right.num_classical_bits(),
            },
        ));
    }

    match config.method {
        EquivalenceMethod::Structural => Ok(report_not_equivalent(
            started,
            EquivalenceMethod::Structural,
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            first_structural_difference(left, right)
                .unwrap_or(Difference::StructuralOperation { index: 0 }),
        )),

        EquivalenceMethod::ExactUnitary { relation } => {
            verify_exact_unitary(
                left,
                right,
                relation,
                config.tolerance,
                config.limits,
                started,
                left_fingerprint,
                right_fingerprint,
            )
        }

        EquivalenceMethod::Auto { relation } => {
            verify_auto(
                left,
                right,
                relation,
                config.tolerance,
                config.limits,
                started,
                left_fingerprint,
                right_fingerprint,
            )
        }
    }
}

/// Verifies structural identity only.
///
/// This is always O(n) in the number of operations and never performs dense
/// simulation.
pub fn verify_structural(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) -> Result<EquivalenceReport, EquivalenceError> {
    verify(
        left,
        right,
        EquivalenceConfig {
            method: EquivalenceMethod::Structural,
            ..EquivalenceConfig::default()
        },
    )
}

/// Verifies exact unitary equivalence up to global phase using default
/// numerical tolerances and conservative resource limits.
pub fn verify_unitary(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) -> Result<EquivalenceReport, EquivalenceError> {
    verify(
        left,
        right,
        EquivalenceConfig {
            method: EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase,
            },
            ..EquivalenceConfig::default()
        },
    )
}

/// Returns a deterministic SHA-256 fingerprint of the canonical circuit's
/// logical structure.
///
/// This is a fingerprint, not a semantic-equivalence proof.
///
/// Two equivalent circuits can intentionally have different fingerprints.
pub fn structural_fingerprint(
    circuit: &QuantumCircuit,
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hash_usize(&mut hasher, circuit.num_qubits());
    hash_usize(&mut hasher, circuit.num_classical_bits());

    let operations = circuit.operations();

    hash_usize(&mut hasher, operations.len());

    for gate in operations {
        hash_gate(&mut hasher, gate);
    }

    let digest = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(&digest);
    result
}

// ============================================================================
// Automatic verification
// ============================================================================

fn verify_auto(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
    relation: UnitaryRelation,
    tolerance: EquivalenceTolerance,
    limits: EquivalenceLimits,
    started: Instant,
    left_fingerprint: [u8; 32],
    right_fingerprint: [u8; 32],
) -> Result<EquivalenceReport, EquivalenceError> {
    let left_ops = left.operations();
    let right_ops = right.operations();

    if left_ops.len() > limits.max_operations {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::Auto { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::OperationLimitExceeded {
                actual: left_ops.len(),
                maximum: limits.max_operations,
            },
        ));
    }

    if right_ops.len() > limits.max_operations {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::Auto { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::OperationLimitExceeded {
                actual: right_ops.len(),
                maximum: limits.max_operations,
            },
        ));
    }

    if left
        .operations()
        .iter()
        .chain(right.operations().iter())
        .any(|gate| !gate.kind().is_unitary())
    {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::Auto { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::NonUnitaryCircuit,
        ));
    }

    verify_exact_unitary(
        left,
        right,
        relation,
        tolerance,
        limits,
        started,
        left_fingerprint,
        right_fingerprint,
    )
}

// ============================================================================
// Exact dense unitary verification
// ============================================================================

fn verify_exact_unitary(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
    relation: UnitaryRelation,
    tolerance: EquivalenceTolerance,
    limits: EquivalenceLimits,
    started: Instant,
    left_fingerprint: [u8; 32],
    right_fingerprint: [u8; 32],
) -> Result<EquivalenceReport, EquivalenceError> {
    if left.num_qubits() > limits.max_qubits {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::ExactUnitary { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::QubitLimitExceeded {
                actual: left.num_qubits(),
                maximum: limits.max_qubits,
            },
        ));
    }

    let dimension = checked_dimension(left.num_qubits())?;

    if dimension > limits.max_amplitudes {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::ExactUnitary { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::AmplitudeLimitExceeded {
                required: dimension,
                maximum: limits.max_amplitudes,
            },
        ));
    }

    let basis_limit = if limits.max_basis_states == 0 {
        dimension
    } else {
        limits.max_basis_states.min(dimension)
    };

    if basis_limit < dimension {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::ExactUnitary { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::BasisStateLimitExceeded {
                required: dimension,
                maximum: basis_limit,
            },
        ));
    }

    if left.operations().len() > limits.max_operations {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::ExactUnitary { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::OperationLimitExceeded {
                actual: left.operations().len(),
                maximum: limits.max_operations,
            },
        ));
    }

    if right.operations().len() > limits.max_operations {
        return Ok(report_inconclusive(
            started,
            EquivalenceMethod::ExactUnitary { relation },
            left,
            right,
            left_fingerprint,
            right_fingerprint,
            InconclusiveReason::OperationLimitExceeded {
                actual: right.operations().len(),
                maximum: limits.max_operations,
            },
        ));
    }

    for gate in left.operations().iter().chain(right.operations().iter()) {
        if !gate.kind().is_unitary() {
            return Ok(report_inconclusive(
                started,
                EquivalenceMethod::ExactUnitary { relation },
                left,
                right,
                left_fingerprint,
                right_fingerprint,
                InconclusiveReason::NonUnitaryCircuit,
            ));
        }

        if let Some(reason) = unsupported_gate_reason(gate) {
            return Ok(report_inconclusive(
                started,
                EquivalenceMethod::ExactUnitary { relation },
                left,
                right,
                left_fingerprint,
                right_fingerprint,
                reason,
            ));
        }

        if gate
            .parameters()
            .iter()
            .any(|parameter| !parameter.is_constant())
        {
            return Ok(report_inconclusive(
                started,
                EquivalenceMethod::ExactUnitary { relation },
                left,
                right,
                left_fingerprint,
                right_fingerprint,
                InconclusiveReason::SymbolicParameters,
            ));
        }
    }

    let mut reference_phase: Option<Complex> = None;
    let mut max_error = 0.0f64;

    for basis in 0..dimension {
        if deadline_exceeded(started, limits.max_duration) {
            return Ok(report_inconclusive(
                started,
                EquivalenceMethod::ExactUnitary { relation },
                left,
                right,
                left_fingerprint,
                right_fingerprint,
                InconclusiveReason::TimeLimitExceeded,
            ));
        }

        let mut lhs = StateVector::basis(dimension, basis);
        let mut rhs = StateVector::basis(dimension, basis);

        apply_circuit(left, &mut lhs)?;
        apply_circuit(right, &mut rhs)?;

        match relation {
            UnitaryRelation::Exact => {
                let error = lhs.max_difference(&rhs);

                if error > max_error {
                    max_error = error;
                }

                if !states_equal(&lhs, &rhs, tolerance) {
                    return Ok(report_not_equivalent(
                        started,
                        EquivalenceMethod::ExactUnitary { relation },
                        left,
                        right,
                        left_fingerprint,
                        right_fingerprint,
                        Difference::UnitaryAction {
                            basis_state: basis,
                            max_error: error,
                        },
                    ));
                }
            }

            UnitaryRelation::UpToGlobalPhase => {
                let phase = match reference_phase {
                    Some(value) => value,

                    None => {
                        match find_relative_phase(
                            &lhs,
                            &rhs,
                            tolerance,
                        ) {
                            Some(value) => {
                                reference_phase = Some(value);
                                value
                            }

                            None => {
                                let error = lhs.max_difference(&rhs);

                                return Ok(
                                    report_not_equivalent(
                                        started,
                                        EquivalenceMethod::ExactUnitary {
                                            relation,
                                        },
                                        left,
                                        right,
                                        left_fingerprint,
                                        right_fingerprint,
                                        Difference::UnitaryAction {
                                            basis_state: basis,
                                            max_error: error,
                                        },
                                    ),
                                );
                            }
                        }
                    }
                };

                let phased_rhs = rhs.scaled(phase);
                let error = lhs.max_difference(&phased_rhs);

                if error > max_error {
                    max_error = error;
                }

                if !states_equal(&lhs, &phased_rhs, tolerance) {
                    return Ok(report_not_equivalent(
                        started,
                        EquivalenceMethod::ExactUnitary { relation },
                        left,
                        right,
                        left_fingerprint,
                        right_fingerprint,
                        Difference::UnitaryAction {
                            basis_state: basis,
                            max_error: error,
                        },
                    ));
                }
            }
        }
    }

    let global_phase = reference_phase.map(|phase| (phase.re, phase.im));

    Ok(EquivalenceReport {
        verdict: EquivalenceVerdict::Equivalent,
        method: EquivalenceMethod::ExactUnitary { relation },
        inconclusive_reason: None,
        difference: None,
        structurally_equal: false,
        qubits: left.num_qubits(),
        left_operations: left.operations().len(),
        right_operations: right.operations().len(),
        max_error,
        global_phase,
        left_fingerprint,
        right_fingerprint,
        elapsed: started.elapsed(),
    })
}

// ============================================================================
// Circuit validation and result construction
// ============================================================================

fn validate_input(
    side: &'static str,
    circuit: &QuantumCircuit,
) -> Result<(), EquivalenceError> {
    circuit
        .validate()
        .map_err(|error| EquivalenceError::InvalidCircuit {
            side,
            message: error.to_string(),
        })
}

fn report_not_equivalent(
    started: Instant,
    method: EquivalenceMethod,
    left: &QuantumCircuit,
    right: &QuantumCircuit,
    left_fingerprint: [u8; 32],
    right_fingerprint: [u8; 32],
    difference: Difference,
) -> EquivalenceReport {
    EquivalenceReport {
        verdict: EquivalenceVerdict::NotEquivalent,
        method,
        inconclusive_reason: None,
        difference: Some(difference),
        structurally_equal: false,
        qubits: left.num_qubits().max(right.num_qubits()),
        left_operations: left.operations().len(),
        right_operations: right.operations().len(),
        max_error: 0.0,
        global_phase: None,
        left_fingerprint,
        right_fingerprint,
        elapsed: started.elapsed(),
    }
}

fn report_inconclusive(
    started: Instant,
    method: EquivalenceMethod,
    left: &QuantumCircuit,
    right: &QuantumCircuit,
    left_fingerprint: [u8; 32],
    right_fingerprint: [u8; 32],
    reason: InconclusiveReason,
) -> EquivalenceReport {
    EquivalenceReport {
        verdict: EquivalenceVerdict::Inconclusive,
        method,
        inconclusive_reason: Some(reason),
        difference: None,
        structurally_equal: false,
        qubits: left.num_qubits().max(right.num_qubits()),
        left_operations: left.operations().len(),
        right_operations: right.operations().len(),
        max_error: 0.0,
        global_phase: None,
        left_fingerprint,
        right_fingerprint,
        elapsed: started.elapsed(),
    }
}

fn first_structural_difference(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) -> Option<Difference> {
    let left_ops = left.operations();
    let right_ops = right.operations();

    let common = left_ops.len().min(right_ops.len());

    for index in 0..common {
        if left_ops[index] != right_ops[index] {
            return Some(Difference::StructuralOperation { index });
        }
    }

    if left_ops.len() != right_ops.len() {
        return Some(Difference::StructuralOperation {
            index: common,
        });
    }

    None
}

// ============================================================================
// Dense state-vector implementation
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };
    const ONE: Self = Self { re: 1.0, im: 0.0 };
    const I: Self = Self { re: 0.0, im: 1.0 };

    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    fn scale(self, value: f64) -> Self {
        Self {
            re: self.re * value,
            im: self.im * value,
        }
    }

    fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    fn div(self, other: Self) -> Option<Self> {
        let denominator = other.norm_squared();

        if denominator == 0.0 || !denominator.is_finite() {
            return None;
        }

        Some(Self {
            re: (self.re * other.re + self.im * other.im)
                / denominator,
            im: (self.im * other.re - self.re * other.im)
                / denominator,
        })
    }
}

#[derive(Debug, Clone)]
struct StateVector {
    amplitudes: Vec<Complex>,
}

impl StateVector {
    fn basis(
        dimension: usize,
        basis: usize,
    ) -> Self {
        let mut amplitudes = vec![Complex::ZERO; dimension];
        amplitudes[basis] = Complex::ONE;

        Self { amplitudes }
    }

    fn scaled(
        &self,
        factor: Complex,
    ) -> Self {
        let amplitudes = self
            .amplitudes
            .iter()
            .copied()
            .map(|value| value.mul(factor))
            .collect();

        Self { amplitudes }
    }

    fn max_difference(
        &self,
        other: &Self,
    ) -> f64 {
        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .map(|(lhs, rhs)| {
                lhs.sub(*rhs).norm()
            })
            .fold(0.0, f64::max)
    }
}

fn states_equal(
    lhs: &StateVector,
    rhs: &StateVector,
    tolerance: EquivalenceTolerance,
) -> bool {
    lhs.amplitudes
        .iter()
        .zip(rhs.amplitudes.iter())
        .all(|(lhs, rhs)| {
            tolerance.accepts(lhs.re, rhs.re)
                && tolerance.accepts(lhs.im, rhs.im)
        })
}

fn find_relative_phase(
    lhs: &StateVector,
    rhs: &StateVector,
    tolerance: EquivalenceTolerance,
) -> Option<Complex> {
    let mut phase: Option<Complex> = None;

    for (left, right) in lhs
        .amplitudes
        .iter()
        .copied()
        .zip(rhs.amplitudes.iter().copied())
    {
        let left_norm = left.norm();
        let right_norm = right.norm();

        if left_norm <= tolerance.absolute
            && right_norm <= tolerance.absolute
        {
            continue;
        }

        if left_norm <= tolerance.absolute
            || right_norm <= tolerance.absolute
        {
            return None;
        }

        let candidate = left.div(right)?;

        let candidate_norm = candidate.norm();

        if !candidate_norm.is_finite()
            || candidate_norm <= tolerance.absolute
        {
            return None;
        }

        let normalized = candidate.scale(1.0 / candidate_norm);

        if let Some(reference) = phase {
            if !tolerance.accepts(
                reference.re,
                normalized.re,
            ) || !tolerance.accepts(
                reference.im,
                normalized.im,
            ) {
                return None;
            }
        } else {
            phase = Some(normalized);
        }
    }

    phase.or(Some(Complex::ONE))
}

// ============================================================================
// Circuit execution
// ============================================================================

fn apply_circuit(
    circuit: &QuantumCircuit,
    state: &mut StateVector,
) -> Result<(), EquivalenceError> {
    for gate in circuit.operations() {
        apply_gate(
            gate,
            circuit.num_qubits(),
            state,
        )?;
    }

    Ok(())
}

fn apply_gate(
    gate: &Gate,
    num_qubits: usize,
    state: &mut StateVector,
) -> Result<(), EquivalenceError> {
    match gate.kind() {
        GateKind::I => {
            Ok(())
        }

        GateKind::X => {
            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ZERO, Complex::ONE],
                    [Complex::ONE, Complex::ZERO],
                ],
            )
        }

        GateKind::Y => {
            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ZERO, Complex::new(0.0, -1.0)],
                    [Complex::I, Complex::ZERO],
                ],
            )
        }

        GateKind::Z => {
            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(-1.0, 0.0)],
                ],
            )
        }

        GateKind::H => {
            let s = 1.0 / 2.0f64.sqrt();

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [
                        Complex::new(s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                    [
                        Complex::new(s, 0.0),
                        Complex::new(-s, 0.0),
                    ],
                ],
            )
        }

        GateKind::S => {
            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::I],
                ],
            )
        }

        GateKind::Sdg => {
            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(0.0, -1.0)],
                ],
            )
        }

        GateKind::T => {
            let phase = Complex::new(
                (std::f64::consts::PI / 4.0).cos(),
                (std::f64::consts::PI / 4.0).sin(),
            );

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, phase],
                ],
            )
        }

        GateKind::Tdg => {
            let phase = Complex::new(
                (std::f64::consts::PI / 4.0).cos(),
                -(std::f64::consts::PI / 4.0).sin(),
            );

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, phase],
                ],
            )
        }

        GateKind::V => {
            let s = 0.5f64.sqrt();

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [
                        Complex::new(s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                    [
                        Complex::new(s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                ],
            )
        }

        GateKind::Vdg => {
            let s = 0.5f64.sqrt();

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [
                        Complex::new(s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                    [
                        Complex::new(-s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                ],
            )
        }

        GateKind::RX => {
            let theta = parameter(gate, 0)?;

            let half = theta / 2.0;
            let c = half.cos();
            let s = half.sin();

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [
                        Complex::new(c, 0.0),
                        Complex::new(0.0, -s),
                    ],
                    [
                        Complex::new(0.0, -s),
                        Complex::new(c, 0.0),
                    ],
                ],
            )
        }

        GateKind::RY => {
            let theta = parameter(gate, 0)?;

            let half = theta / 2.0;
            let c = half.cos();
            let s = half.sin();

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                [
                    [
                        Complex::new(c, 0.0),
                        Complex::new(-s, 0.0),
                    ],
                    [
                        Complex::new(s, 0.0),
                        Complex::new(c, 0.0),
                    ],
                ],
            )
        }

        GateKind::RZ
        | GateKind::Phase
        | GateKind::U1 => {
            let theta = parameter(gate, 0)?;

            let half = theta / 2.0;

            let minus = Complex::new(
                half.cos(),
                -half.sin(),
            );

            let plus = Complex::new(
                half.cos(),
                half.sin(),
            );

            let matrix = match gate.kind() {
                GateKind::RZ => [
                    [minus, Complex::ZERO],
                    [Complex::ZERO, plus],
                ],

                GateKind::Phase | GateKind::U1 => [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(theta.cos(), theta.sin())],
                ],

                _ => unreachable!(),
            };

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                matrix,
            )
        }

        GateKind::U2 => {
            let phi = parameter(gate, 0)?;
            let lambda = parameter(gate, 1)?;

            let s = 1.0 / 2.0f64.sqrt();

            let matrix = [
                [
                    Complex::new(s, 0.0),
                    Complex::new(
                        -lambda.cos() * s,
                        -lambda.sin() * s,
                    ),
                ],
                [
                    Complex::new(
                        phi.cos() * s,
                        phi.sin() * s,
                    ),
                    Complex::new(
                        (phi + lambda).cos() * s,
                        (phi + lambda).sin() * s,
                    ),
                ],
            ];

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                matrix,
            )
        }

        GateKind::U3 => {
            let theta = parameter(gate, 0)?;
            let phi = parameter(gate, 1)?;
            let lambda = parameter(gate, 2)?;

            let half = theta / 2.0;
            let c = half.cos();
            let s = half.sin();

            let matrix = [
                [
                    Complex::new(c, 0.0),
                    Complex::new(
                        -lambda.cos() * s,
                        -lambda.sin() * s,
                    ),
                ],
                [
                    Complex::new(
                        phi.cos() * s,
                        phi.sin() * s,
                    ),
                    Complex::new(
                        (phi + lambda).cos() * s,
                        (phi + lambda).sin() * s,
                    ),
                ],
            ];

            apply_single_qubit_matrix(
                state,
                qubit(gate, 0)?,
                num_qubits,
                matrix,
            )
        }

        GateKind::CX => {
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                [
                    [Complex::ZERO, Complex::ONE],
                    [Complex::ONE, Complex::ZERO],
                ],
            )
        }

        GateKind::CY => {
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                [
                    [Complex::ZERO, Complex::new(0.0, -1.0)],
                    [Complex::I, Complex::ZERO],
                ],
            )
        }

        GateKind::CZ => {
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                [
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::new(-1.0, 0.0)],
                ],
            )
        }

        GateKind::CH => {
            let s = 1.0 / 2.0f64.sqrt();

            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                [
                    [
                        Complex::new(s, 0.0),
                        Complex::new(s, 0.0),
                    ],
                    [
                        Complex::new(s, 0.0),
                        Complex::new(-s, 0.0),
                    ],
                ],
            )
        }

        GateKind::SWAP => {
            apply_swap(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
            )
        }

        GateKind::ISWAP => {
            let matrix = [
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
            ];

            apply_two_qubit_matrix(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                matrix,
            )
        }

        GateKind::CRX => {
            let theta = parameter(gate, 0)?;
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                rx_matrix(theta),
            )
        }

        GateKind::CRY => {
            let theta = parameter(gate, 0)?;
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                ry_matrix(theta),
            )
        }

        GateKind::CRZ => {
            let theta = parameter(gate, 0)?;
            apply_controlled_single_qubit(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                num_qubits,
                rz_matrix(theta),
            )
        }

        GateKind::CCX => {
            apply_toffoli(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                qubit(gate, 2)?,
                num_qubits,
            )
        }

        GateKind::CSWAP => {
            apply_controlled_swap(
                state,
                qubit(gate, 0)?,
                qubit(gate, 1)?,
                qubit(gate, 2)?,
                num_qubits,
            )
        }

        GateKind::ECR => {
            return Err(EquivalenceError::InvalidCircuit {
                side: "semantic verifier",
                message: "ECR has no canonical dense semantic definition in this verifier"
                    .to_string(),
            });
        }

        GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset => {
            return Err(EquivalenceError::InvalidCircuit {
                side: "semantic verifier",
                message: "non-unitary operation reached unitary executor"
                    .to_string(),
            });
        }
    }
}

fn rx_matrix(theta: f64) -> [[Complex; 2]; 2] {
    let half = theta / 2.0;
    let c = half.cos();
    let s = half.sin();

    [
        [
            Complex::new(c, 0.0),
            Complex::new(0.0, -s),
        ],
        [
            Complex::new(0.0, -s),
            Complex::new(c, 0.0),
        ],
    ]
}

fn ry_matrix(theta: f64) -> [[Complex; 2]; 2] {
    let half = theta / 2.0;
    let c = half.cos();
    let s = half.sin();

    [
        [
            Complex::new(c, 0.0),
            Complex::new(-s, 0.0),
        ],
        [
            Complex::new(s, 0.0),
            Complex::new(c, 0.0),
        ],
    ]
}

fn rz_matrix(theta: f64) -> [[Complex; 2]; 2] {
    let half = theta / 2.0;

    [
        [
            Complex::new(half.cos(), -half.sin()),
            Complex::ZERO,
        ],
        [
            Complex::ZERO,
            Complex::new(half.cos(), half.sin()),
        ],
    ]
}

fn apply_single_qubit_matrix(
    state: &mut StateVector,
    target: QubitId,
    num_qubits: usize,
    matrix: [[Complex; 2]; 2],
) -> Result<(), EquivalenceError> {
    let bit = target.index();

    validate_qubit_index(
        bit,
        num_qubits,
    )?;

    let stride = checked_power_of_two(bit)?;

    let block = stride
        .checked_mul(2)
        .ok_or(EquivalenceError::ArithmeticOverflow {
            calculation: "single-qubit block size",
        })?;

    let length = state.amplitudes.len();

    let mut base = 0usize;

    while base < length {
        let mut offset = 0usize;

        while offset < stride {
            let zero = base + offset;
            let one = zero + stride;

            let a = state.amplitudes[zero];
            let b = state.amplitudes[one];

            state.amplitudes[zero] =
                matrix[0][0].mul(a).add(matrix[0][1].mul(b));

            state.amplitudes[one] =
                matrix[1][0].mul(a).add(matrix[1][1].mul(b));

            offset += 1;
        }

        base = base
            .checked_add(block)
            .ok_or(EquivalenceError::ArithmeticOverflow {
                calculation: "single-qubit block traversal",
            })?;
    }

    Ok(())
}

fn apply_controlled_single_qubit(
    state: &mut StateVector,
    control: QubitId,
    target: QubitId,
    num_qubits: usize,
    matrix: [[Complex; 2]; 2],
) -> Result<(), EquivalenceError> {
    let control_bit = control.index();
    let target_bit = target.index();

    validate_qubit_index(control_bit, num_qubits)?;
    validate_qubit_index(target_bit, num_qubits)?;

    if control_bit == target_bit {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "controlled gate uses the same control and target qubit"
                .to_string(),
        });
    }

    let control_mask = checked_power_of_two(control_bit)?;
    let target_mask = checked_power_of_two(target_bit)?;

    let length = state.amplitudes.len();

    for base in 0..length {
        if base & control_mask == 0 {
            continue;
        }

        if base & target_mask != 0 {
            continue;
        }

        let zero = base;
        let one = base | target_mask;

        let a = state.amplitudes[zero];
        let b = state.amplitudes[one];

        state.amplitudes[zero] =
            matrix[0][0].mul(a).add(matrix[0][1].mul(b));

        state.amplitudes[one] =
            matrix[1][0].mul(a).add(matrix[1][1].mul(b));
    }

    Ok(())
}

fn apply_two_qubit_matrix(
    state: &mut StateVector,
    first: QubitId,
    second: QubitId,
    num_qubits: usize,
    matrix: [[Complex; 4]; 4],
) -> Result<(), EquivalenceError> {
    let first_bit = first.index();
    let second_bit = second.index();

    validate_qubit_index(first_bit, num_qubits)?;
    validate_qubit_index(second_bit, num_qubits)?;

    if first_bit == second_bit {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "two-qubit gate uses duplicate qubits"
                .to_string(),
        });
    }

    let first_mask = checked_power_of_two(first_bit)?;
    let second_mask = checked_power_of_two(second_bit)?;

    let length = state.amplitudes.len();

    for base in 0..length {
        if base & first_mask != 0 || base & second_mask != 0 {
            continue;
        }

        let i00 = base;
        let i01 = base | second_mask;
        let i10 = base | first_mask;
        let i11 = base | first_mask | second_mask;

        let input = [
            state.amplitudes[i00],
            state.amplitudes[i01],
            state.amplitudes[i10],
            state.amplitudes[i11],
        ];

        let mut output = [Complex::ZERO; 4];

        for row in 0..4 {
            for column in 0..4 {
                output[row] = output[row]
                    .add(matrix[row][column].mul(input[column]));
            }
        }

        state.amplitudes[i00] = output[0];
        state.amplitudes[i01] = output[1];
        state.amplitudes[i10] = output[2];
        state.amplitudes[i11] = output[3];
    }

    Ok(())
}

fn apply_swap(
    state: &mut StateVector,
    first: QubitId,
    second: QubitId,
    num_qubits: usize,
) -> Result<(), EquivalenceError> {
    let first_bit = first.index();
    let second_bit = second.index();

    validate_qubit_index(first_bit, num_qubits)?;
    validate_qubit_index(second_bit, num_qubits)?;

    if first_bit == second_bit {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "SWAP uses duplicate qubits"
                .to_string(),
        });
    }

    let first_mask = checked_power_of_two(first_bit)?;
    let second_mask = checked_power_of_two(second_bit)?;

    for index in 0..state.amplitudes.len() {
        let first_set = index & first_mask != 0;
        let second_set = index & second_mask != 0;

        if !first_set && second_set {
            let swapped = index ^ first_mask ^ second_mask;

            state.amplitudes.swap(index, swapped);
        }
    }

    Ok(())
}

fn apply_toffoli(
    state: &mut StateVector,
    control_a: QubitId,
    control_b: QubitId,
    target: QubitId,
    num_qubits: usize,
) -> Result<(), EquivalenceError> {
    let a = checked_power_of_two(control_a.index())?;
    let b = checked_power_of_two(control_b.index())?;
    let t = checked_power_of_two(target.index())?;

    validate_qubit_index(control_a.index(), num_qubits)?;
    validate_qubit_index(control_b.index(), num_qubits)?;
    validate_qubit_index(target.index(), num_qubits)?;

    if control_a == control_b
        || control_a == target
        || control_b == target
    {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "CCX uses duplicate qubits"
                .to_string(),
        });
    }

    for index in 0..state.amplitudes.len() {
        if index & a != 0
            && index & b != 0
            && index & t == 0
        {
            let partner = index | t;
            state.amplitudes.swap(index, partner);
        }
    }

    Ok(())
}

fn apply_controlled_swap(
    state: &mut StateVector,
    control: QubitId,
    first: QubitId,
    second: QubitId,
    num_qubits: usize,
) -> Result<(), EquivalenceError> {
    let control_mask = checked_power_of_two(control.index())?;
    let first_mask = checked_power_of_two(first.index())?;
    let second_mask = checked_power_of_two(second.index())?;

    validate_qubit_index(control.index(), num_qubits)?;
    validate_qubit_index(first.index(), num_qubits)?;
    validate_qubit_index(second.index(), num_qubits)?;

    if control == first
        || control == second
        || first == second
    {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "CSWAP uses duplicate qubits"
                .to_string(),
        });
    }

    for index in 0..state.amplitudes.len() {
        if index & control_mask == 0 {
            continue;
        }

        let first_set = index & first_mask != 0;
        let second_set = index & second_mask != 0;

        if !first_set && second_set {
            let swapped = index ^ first_mask ^ second_mask;

            state.amplitudes.swap(index, swapped);
        }
    }

    Ok(())
}

// ============================================================================
// Gate helpers
// ============================================================================

fn qubit(
    gate: &Gate,
    index: usize,
) -> Result<QubitId, EquivalenceError> {
    gate.qubits()
        .get(index)
        .copied()
        .ok_or(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "gate is missing a required qubit operand"
                .to_string(),
        })
}

fn parameter(
    gate: &Gate,
    index: usize,
) -> Result<f64, EquivalenceError> {
    let value = gate
        .parameters()
        .get(index)
        .ok_or(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "gate is missing a required parameter"
                .to_string(),
        })?;

    match value {
        Parameter::Constant(value) => {
            if value.is_finite() {
                Ok(*value)
            } else {
                Err(EquivalenceError::InvalidCircuit {
                    side: "semantic verifier",
                    message: "gate parameter is not finite"
                        .to_string(),
                })
            }
        }

        Parameter::Symbol(_)
        | Parameter::Expression(_) => {
            Err(EquivalenceError::InvalidCircuit {
                side: "semantic verifier",
                message: "symbolic parameter reached constant-only dense verifier"
                    .to_string(),
            })
        }
    }
}

fn unsupported_gate_reason(
    gate: &Gate,
) -> Option<InconclusiveReason> {
    match gate.kind() {
        GateKind::ECR => {
            Some(InconclusiveReason::UnsupportedGate(
                GateKind::ECR,
            ))
        }

        GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset => {
            Some(InconclusiveReason::NonUnitaryCircuit)
        }

        _ => None,
    }
}

// ============================================================================
// Structural fingerprint
// ============================================================================

fn hash_gate(
    hasher: &mut Sha256,
    gate: &Gate,
) {
    hash_u64(
        hasher,
        gate.kind() as u64,
    );

    hash_usize(
        hasher,
        gate.qubits().len(),
    );

    for qubit in gate.qubits() {
        hash_usize(
            hasher,
            qubit.index(),
        );
    }

    hash_usize(
        hasher,
        gate.parameters().len(),
    );

    for parameter in gate.parameters() {
        match parameter {
            Parameter::Constant(value) => {
                hasher.update([0u8]);
                hasher.update(value.to_bits().to_le_bytes());
            }

            Parameter::Symbol(name) => {
                hasher.update([1u8]);
                hash_bytes(hasher, name.as_bytes());
            }

            Parameter::Expression(expression) => {
                hasher.update([2u8]);
                hash_bytes(
                    hasher,
                    expression.to_string().as_bytes(),
                );
            }
        }
    }

    match gate.classical_target() {
        Some(target) => {
            hasher.update([1u8]);
            hash_usize(hasher, target);
        }

        None => {
            hasher.update([0u8]);
        }
    }

    if let Some(measurement) = gate.measurement() {
        hasher.update([1u8]);
        hash_bytes(
            hasher,
            format!("{measurement:?}").as_bytes(),
        );
    } else {
        hasher.update([0u8]);
    }
}

fn hash_usize(
    hasher: &mut Sha256,
    value: usize,
) {
    hasher.update(
        (value as u64)
            .to_le_bytes(),
    );
}

fn hash_u64(
    hasher: &mut Sha256,
    value: u64,
) {
    hasher.update(value.to_le_bytes());
}

fn hash_bytes(
    hasher: &mut Sha256,
    value: &[u8],
) {
    hash_usize(
        hasher,
        value.len(),
    );

    hasher.update(value);
}

// ============================================================================
// Resource arithmetic
// ============================================================================

fn checked_dimension(
    qubits: usize,
) -> Result<usize, EquivalenceError> {
    if qubits >= usize::BITS as usize {
        return Err(EquivalenceError::ArithmeticOverflow {
            calculation: "2^qubits state dimension",
        });
    }

    1usize
        .checked_shl(qubits as u32)
        .ok_or(EquivalenceError::ArithmeticOverflow {
            calculation: "2^qubits state dimension",
        })
}

fn checked_power_of_two(
    bit: usize,
) -> Result<usize, EquivalenceError> {
    if bit >= usize::BITS as usize {
        return Err(EquivalenceError::ArithmeticOverflow {
            calculation: "qubit bit-mask",
        });
    }

    1usize
        .checked_shl(bit as u32)
        .ok_or(EquivalenceError::ArithmeticOverflow {
            calculation: "qubit bit-mask",
        })
}

fn validate_qubit_index(
    index: usize,
    num_qubits: usize,
) -> Result<(), EquivalenceError> {
    if index >= num_qubits {
        return Err(EquivalenceError::InvalidCircuit {
            side: "semantic verifier",
            message: "gate references a qubit outside the circuit namespace"
                .to_string(),
        });
    }

    Ok(())
}

fn deadline_exceeded(
    started: Instant,
    limit: Option<Duration>,
) -> bool {
    match limit {
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

    #[test]
    fn tolerance_rejects_negative_values() {
        assert!(EquivalenceTolerance::new(
            -1.0,
            0.0
        )
        .is_err());

        assert!(EquivalenceTolerance::new(
            0.0,
            -1.0
        )
        .is_err());
    }

    #[test]
    fn tolerance_accepts_exact_values() {
        let tolerance = EquivalenceTolerance::exact();

        assert!(tolerance.accepts(
            1.0,
            1.0
        ));

        assert!(!tolerance.accepts(
            1.0,
            1.0000000001
        ));
    }

    #[test]
    fn checked_dimension_is_safe() {
        assert_eq!(
            checked_dimension(0).unwrap(),
            1
        );

        assert_eq!(
            checked_dimension(1).unwrap(),
            2
        );

        assert_eq!(
            checked_dimension(4).unwrap(),
            16
        );
    }

    #[test]
    fn complex_division_rejects_zero() {
        assert!(
            Complex::ONE
                .div(Complex::ZERO)
                .is_none()
        );
    }

    #[test]
    fn structural_fingerprint_is_deterministic() {
        // This test deliberately verifies only determinism of the hashing
        // mechanism. Circuit-construction-specific tests belong in the
        // integration test suite because QuantumCircuit constructors are
        // owned by quantum::ir.
        let mut hasher_a = Sha256::new();
        let mut hasher_b = Sha256::new();

        hash_usize(&mut hasher_a, 4);
        hash_usize(&mut hasher_b, 4);

        assert_eq!(
            hasher_a.finalize(),
            hasher_b.finalize()
        );
    }

    #[test]
    fn verdict_helpers_are_correct() {
        assert!(
            EquivalenceVerdict::Equivalent
                .is_equivalent()
        );

        assert!(
            EquivalenceVerdict::NotEquivalent
                .is_not_equivalent()
        );

        assert!(
            EquivalenceVerdict::Inconclusive
                .is_inconclusive()
        );
    }

    #[test]
    fn relative_phase_of_identical_states_is_one() {
        let lhs = StateVector::basis(2, 0);
        let rhs = StateVector::basis(2, 0);

        let phase = find_relative_phase(
            &lhs,
            &rhs,
            EquivalenceTolerance::numerical(),
        )
        .unwrap();

        assert!(
            (phase.re - 1.0).abs() < 1.0e-12
        );

        assert!(
            phase.im.abs() < 1.0e-12
        );
    }
}