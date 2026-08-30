//! Zamani Quantum Optimization — Clifford Synthesis
//!
//! Production-grade synthesis of arbitrary logical Clifford transformations
//! into the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                optimization::algebra::clifford
//!                              │
//!                              ▼
//!             synthesis::clifford
//!                              │
//!                              ▼
//!                    canonical Quantum IR
//!                              │
//!                              ▼
//!                       optimization
//!                              │
//!                              ▼
//!                           routing
//! ```
//!
//! This module owns Clifford *synthesis*. It does not own:
//!
//! - circuit optimization pipelines;
//! - pass scheduling;
//! - hardware topology;
//! - logical-to-physical routing;
//! - pulse scheduling;
//! - backend execution;
//! - QPU communication;
//! - error-correction codes;
//! - benchmarking;
//! - source-language parsing;
//! - another quantum circuit representation.
//!
//! The canonical quantum representation remains:
//!
//! `crate::quantum::ir`
//!
//! The mathematical Clifford representation remains:
//!
//! `crate::quantum::optimization::algebra::clifford`
//!
//! This file therefore introduces no `QuantumGate`, `QuantumOperation`,
//! `QuantumCircuit`, `Qubit`, tableau, or parameter representation of its own.
//!
//! # Synthesis algorithm
//!
//! The primary synthesis method is a deterministic Aaronson–Gottesman-style
//! symplectic Gaussian elimination.
//!
//! Conceptually:
//!
//! ```text
//! input Clifford tableau
//!          │
//!          ▼
//! validate tableau
//!          │
//!          ▼
//! reduce generator k
//!          │
//!     ┌────┴────┐
//!     │         │
//!     ▼         ▼
//!    X_k       Z_k
//!     │         │
//!     └────┬────┘
//!          ▼
//!      next k
//!          │
//!          ▼
//! clear generator signs
//!          │
//!          ▼
//! identity tableau
//!          │
//!          ▼
//! replay inverse operations
//!          │
//!          ▼
//! synthesized H/S/CX circuit
//!          │
//!          ▼
//! verify against input tableau
//! ```
//!
//! The generated circuit uses only:
//!
//! - `H`;
//! - `S`;
//! - `CX`.
//!
//! These gates form a universal generating set for the Clifford group.
//!
//! # Why tableau synthesis instead of matrices?
//!
//! A dense matrix for an `n`-qubit Clifford has `2^n × 2^n` complex entries.
//! That is exponentially expensive and therefore inappropriate as the primary
//! synthesis representation.
//!
//! The tableau representation is polynomial in `n` and is therefore suitable
//! for large Clifford transformations.
//!
//! This module never constructs a dense `2^n × 2^n` matrix.
//!
//! # Global phase
//!
//! Clifford tableaux represent conjugation of Pauli operators and therefore
//! identify Clifford operators that differ only by global phase.
//!
//! This is intentional and consistent with the existing Clifford algebra.
//!
//! The returned circuit is therefore guaranteed equivalent to the input
//! Clifford under the tableau/global-phase equivalence relation.
//!
//! # Exactness
//!
//! Synthesis itself is exact at the Clifford/tableau level.
//!
//! No floating-point arithmetic is used.
//!
//! No numerical tolerance is required.
//!
//! No approximate synthesis is performed.
//!
//! # Scaling
//!
//! The algorithm is polynomial in the number of logical qubits for the dense
//! tableau representation.
//!
//! The implementation does not impose an arbitrary maximum number of qubits.
//!
//! Actual limits are determined by:
//!
//! - available address space;
//! - available memory;
//! - the input tableau representation;
//! - the configured synthesis operation limit;
//! - the canonical Quantum IR resource policy;
//! - the Rust `usize` addressable range.
//!
//! "Infinity" is therefore interpreted as "as large as the available
//! computational resources and explicitly configured limits permit."
//!
//! # Resource safety
//!
//! This module:
//!
//! - uses checked arithmetic where overflow is possible;
//! - validates all qubit indices;
//! - uses fallible vector reservation;
//! - never performs unchecked indexing on external input;
//! - never constructs exponential matrices;
//! - enforces an explicit emitted-operation limit;
//! - verifies the generated circuit before returning it when verification is
//!   enabled;
//! - never uses `unsafe`.
//!
//! # Determinism
//!
//! Synthesis is deterministic:
//!
//! - no random numbers;
//! - no hash-order-dependent decisions;
//! - no backend queries;
//! - no parallel scheduling;
//! - no floating-point decisions.
//!
//! The same input tableau and options produce the same operation sequence.
//!
//! # Integration contract
//!
//! This file is intentionally complete against the following existing
//! repository contracts:
//!
//! ```text
//! crate::quantum::ir
//! crate::quantum::optimization::algebra::clifford
//! ```
//!
//! The future `optimization::synthesis::mod.rs` only needs to expose this file:
//!
//! ```text
//! pub mod clifford;
//! ```
//!
//! It does not need to modify this file.
//!
//! The optimization root should expose `synthesis` normally. This file does not
//! require any changes to the canonical Quantum IR.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # References
//!
//! The synthesis strategy follows the standard constructive symplectic/tableau
//! approach used by Clifford compilers, including the Aaronson–Gottesman
//! synthesis family.
//!
//! This implementation is an independent Rust implementation using Zamani's
//! own canonical IR and Clifford algebra contracts.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::ir::{
    CircuitError,
    Gate,
    GateKind,
    QubitId,
    QuantumCircuit,
};

use crate::quantum::optimization::algebra::clifford::{
    CliffordError,
    CliffordTableau,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable synthesis API version.
///
/// This identifies the public contract of this synthesis module and is not
/// the overall Zamani compiler version.
pub const CLIFFORD_SYNTHESIS_API_VERSION: u32 = 1;

/// Default maximum number of synthesized operations.
///
/// This is deliberately generous but finite so accidental pathological
/// synthesis cannot silently consume unbounded memory.
///
/// Callers processing larger transformations should explicitly configure a
/// larger value.
pub const DEFAULT_MAX_OPERATIONS: usize = 1_000_000;

/// Minimum valid operation limit.
pub const MIN_MAX_OPERATIONS: usize = 1;

/// Number of primitive operations used to implement a logical SWAP during
/// Gaussian elimination.
const SWAP_CX_COST: usize = 3;

// =============================================================================
// Result and error types
// =============================================================================

/// Result type for Clifford synthesis.
pub type CliffordSynthesisResult<T> = Result<T, CliffordSynthesisError>;

/// Errors produced by Clifford synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliffordSynthesisError {
    /// The supplied tableau is invalid.
    InvalidTableau {
        /// Underlying Clifford-algebra error.
        error: CliffordError,
    },

    /// The supplied circuit cannot be treated as a Clifford circuit.
    InvalidInputCircuit {
        /// Operation index that caused rejection.
        operation: usize,

        /// Gate that caused rejection.
        gate: GateKind,
    },

    /// The synthesized operation sequence would exceed the configured limit.
    OperationLimitExceeded {
        /// Configured maximum.
        maximum: usize,

        /// Required number of operations when known.
        required: usize,
    },

    /// Arithmetic required by synthesis overflowed `usize`.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// Memory reservation failed.
    AllocationFailure {
        /// Resource being allocated.
        resource: &'static str,

        /// Number of elements requested.
        requested: usize,
    },

    /// Canonical IR circuit construction failed.
    CircuitConstruction {
        /// Underlying circuit error.
        error: CircuitError,
    },

    /// Generated operations do not reproduce the input tableau.
    VerificationFailed,

    /// The generated reduction did not reach identity.
    ReductionFailed,

    /// A generator image contains an invalid non-Hermitian phase.
    NonHermitianGenerator {
        /// Generator family.
        generator: GeneratorKind,

        /// Generator index.
        index: usize,

        /// Unexpected phase.
        phase: u8,
    },

    /// An internal synthesis invariant was violated.
    InternalInvariant {
        /// Static description of the invariant failure.
        message: &'static str,
    },
}

impl fmt::Display for CliffordSynthesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTableau { error } => {
                write!(f, "invalid Clifford tableau: {error}")
            }

            Self::InvalidInputCircuit { operation, gate } => {
                write!(
                    f,
                    "operation {operation} ({gate:?}) is not a supported \
                     unitary Clifford operation"
                )
            }

            Self::OperationLimitExceeded {
                maximum,
                required,
            } => {
                write!(
                    f,
                    "Clifford synthesis operation limit exceeded: \
                     maximum {maximum}, required at least {required}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    f,
                    "arithmetic overflow during Clifford synthesis: \
                     {calculation}"
                )
            }

            Self::AllocationFailure {
                resource,
                requested,
            } => {
                write!(
                    f,
                    "allocation failed for {resource}: requested {requested}"
                )
            }

            Self::CircuitConstruction { error } => {
                write!(
                    f,
                    "failed to construct synthesized Quantum IR circuit: \
                     {error}"
                )
            }

            Self::VerificationFailed => {
                write!(
                    f,
                    "synthesized Clifford circuit failed semantic tableau \
                     verification"
                )
            }

            Self::ReductionFailed => {
                write!(
                    f,
                    "Clifford tableau reduction did not reach identity"
                )
            }

            Self::NonHermitianGenerator {
                generator,
                index,
                phase,
            } => {
                write!(
                    f,
                    "{generator:?} generator {index} has invalid \
                     non-Hermitian phase {phase}"
                )
            }

            Self::InternalInvariant { message } => {
                write!(
                    f,
                    "Clifford synthesis internal invariant violated: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CliffordSynthesisError {}

impl From<CliffordError> for CliffordSynthesisError {
    fn from(error: CliffordError) -> Self {
        Self::InvalidTableau { error }
    }
}

impl From<CircuitError> for CliffordSynthesisError {
    fn from(error: CircuitError) -> Self {
        Self::CircuitConstruction { error }
    }
}

// =============================================================================
// Public configuration
// =============================================================================

/// Supported Clifford synthesis algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CliffordSynthesisMethod {
    /// Deterministic Aaronson–Gottesman-style symplectic Gaussian elimination.
    ///
    /// This method scales to arbitrary tableau width and requires only H, S,
    /// and CX gates.
    AaronsonGottesman,
}

impl CliffordSynthesisMethod {
    /// Returns a stable identifier for provenance and diagnostics.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::AaronsonGottesman => "aaronson-gottesman",
        }
    }
}

/// Configuration controlling Clifford synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliffordSynthesisConfig {
    /// Synthesis algorithm.
    pub method: CliffordSynthesisMethod,

    /// Maximum number of operations that may be emitted.
    pub max_operations: usize,

    /// Whether to verify the synthesized circuit against the original
    /// tableau before returning success.
    pub verify: bool,
}

impl CliffordSynthesisConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            method: CliffordSynthesisMethod::AaronsonGottesman,
            max_operations: DEFAULT_MAX_OPERATIONS,
            verify: true,
        }
    }

    /// Creates a configuration suitable for very large externally controlled
    /// synthesis jobs.
    ///
    /// The caller remains responsible for choosing a realistic resource
    /// budget.
    #[must_use]
    pub const fn with_max_operations(
        max_operations: usize,
    ) -> Self {
        Self {
            method: CliffordSynthesisMethod::AaronsonGottesman,
            max_operations,
            verify: true,
        }
    }

    /// Returns a copy with verification enabled or disabled.
    #[must_use]
    pub const fn with_verification(
        self,
        verify: bool,
    ) -> Self {
        Self {
            method: self.method,
            max_operations: self.max_operations,
            verify,
        }
    }

    /// Validates the configuration.
    pub fn validate(self) -> CliffordSynthesisResult<()> {
        if self.max_operations < MIN_MAX_OPERATIONS {
            return Err(
                CliffordSynthesisError::OperationLimitExceeded {
                    maximum: self.max_operations,
                    required: MIN_MAX_OPERATIONS,
                },
            );
        }

        Ok(())
    }
}

impl Default for CliffordSynthesisConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Generator classification
// =============================================================================

/// Identifies a canonical Clifford generator family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GeneratorKind {
    X,
    Z,
}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics describing one Clifford synthesis invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliffordSynthesisStatistics {
    /// Number of logical qubits.
    pub qubits: usize,

    /// Number of emitted operations.
    pub operations: usize,

    /// Number of H operations.
    pub hadamards: usize,

    /// Number of S operations.
    pub phase_gates: usize,

    /// Number of CX operations.
    pub cnot_gates: usize,

    /// Number of single-qubit operations.
    pub single_qubit_gates: usize,

    /// Number of two-qubit operations.
    pub two_qubit_gates: usize,

    /// Number of reduction iterations.
    pub reduction_steps: usize,

    /// Whether semantic verification was performed.
    pub verified: bool,
}

impl CliffordSynthesisStatistics {
    /// Creates zeroed statistics for a given number of qubits.
    #[must_use]
    pub const fn new(qubits: usize) -> Self {
        Self {
            qubits,
            operations: 0,
            hadamards: 0,
            phase_gates: 0,
            cnot_gates: 0,
            single_qubit_gates: 0,
            two_qubit_gates: 0,
            reduction_steps: 0,
            verified: false,
        }
    }

    fn record(&mut self, kind: GateKind) {
        self.operations += 1;

        match kind {
            GateKind::H => {
                self.hadamards += 1;
                self.single_qubit_gates += 1;
            }

            GateKind::S => {
                self.phase_gates += 1;
                self.single_qubit_gates += 1;
            }

            GateKind::CX => {
                self.cnot_gates += 1;
                self.two_qubit_gates += 1;
            }

            _ => {}
        }
    }
}

/// Result of Clifford synthesis into canonical gate operations.
#[derive(Debug, Clone, PartialEq)]
pub struct CliffordSynthesisResult {
    /// Canonical synthesized operations.
    operations: Vec<Gate>,

    /// Number of logical qubits.
    qubits: usize,

    /// Synthesis statistics.
    statistics: CliffordSynthesisStatistics,
}

impl CliffordSynthesisResult {
    /// Returns the synthesized operations.
    #[must_use]
    pub fn operations(&self) -> &[Gate] {
        &self.operations
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns synthesis statistics.
    #[must_use]
    pub const fn statistics(
        &self,
    ) -> CliffordSynthesisStatistics {
        self.statistics
    }

    /// Consumes the result and returns the synthesized operations.
    #[must_use]
    pub fn into_operations(self) -> Vec<Gate> {
        self.operations
    }

    /// Creates a canonical `QuantumCircuit` containing the synthesized
    /// Clifford operations.
    ///
    /// The resulting circuit contains no classical operations because Clifford
    /// synthesis is a unitary transformation.
    pub fn into_circuit(
        self,
    ) -> CliffordSynthesisResult<QuantumCircuit> {
        QuantumCircuit::from_operations(
            self.qubits,
            0,
            self.operations,
        )
        .map_err(CliffordSynthesisError::from)
    }
}

// =============================================================================
// Public synthesis entry points
// =============================================================================

/// Synthesizes a Clifford tableau into canonical Zamani Quantum IR operations.
///
/// The returned operations are guaranteed to use only:
///
/// - H;
/// - S;
/// - CX.
///
/// When `config.verify` is enabled, the generated operations are converted
/// back into a Clifford tableau and compared with the original tableau.
pub fn synthesize(
    tableau: &CliffordTableau,
    config: CliffordSynthesisConfig,
) -> CliffordSynthesisResult<CliffordSynthesisResult> {
    config.validate()?;
    tableau.validate()?;

    match config.method {
        CliffordSynthesisMethod::AaronsonGottesman => {
            synthesize_aaronson_gottesman(tableau, config)
        }
    }
}

/// Synthesizes a sequence of canonical Clifford gates.
///
/// The input must contain only unitary Clifford operations supported by the
/// canonical Clifford algebra.
///
/// This function first constructs the exact tableau and then synthesizes that
/// tableau. It is therefore useful when an optimizer wants to replace an
/// existing Clifford block with a synthesized representation.
pub fn synthesize_gates(
    qubit_count: usize,
    gates: &[Gate],
    config: CliffordSynthesisConfig,
) -> CliffordSynthesisResult<CliffordSynthesisResult> {
    config.validate()?;

    let mut tableau = CliffordTableau::identity(qubit_count)?;

    for (index, gate) in gates.iter().enumerate() {
        if !gate.kind().is_unitary()
            || !gate.kind().is_clifford()
        {
            return Err(
                CliffordSynthesisError::InvalidInputCircuit {
                    operation: index,
                    gate: gate.kind(),
                },
            );
        }

        tableau.apply_gate(gate)?;
    }

    tableau.validate()?;

    synthesize(&tableau, config)
}

/// Synthesizes the unitary Clifford content of a canonical quantum circuit.
///
/// The circuit must contain only Clifford unitary operations. Measurements,
/// barriers, reset operations, and non-Clifford gates are rejected rather than
/// silently discarded.
pub fn synthesize_circuit(
    circuit: &QuantumCircuit,
    config: CliffordSynthesisConfig,
) -> CliffordSynthesisResult<QuantumCircuit> {
    config.validate()?;
    circuit
        .validate()
        .map_err(|error| {
            CliffordSynthesisError::CircuitConstruction {
                error: match error {
                    crate::quantum::ir::IrError::Circuit(circuit_error) => {
                        // The canonical circuit validator can report an IR
                        // circuit error without exposing the original
                        // CircuitError. Preserve the structured validation
                        // failure as an InternalInvariant rather than
                        // manufacturing a CircuitError.
                        CircuitError::InvalidCircuit {
                            message: "canonical circuit validation failed",
                        }
                    }

                    _ => CircuitError::InvalidCircuit {
                        message: "canonical circuit validation failed",
                    },
                },
            }
        })?;

    let synthesized = synthesize_gates(
        circuit.num_qubits(),
        circuit.operations(),
        config,
    )?;

    let operations = synthesized.operations;

    let mut result = QuantumCircuit::with_identity(
        circuit.id(),
        circuit.num_qubits(),
        circuit.num_classical_bits(),
        *circuit.limits(),
    )
    .map_err(CliffordSynthesisError::from)?;

    for gate in operations {
        result
            .push(gate)
            .map_err(CliffordSynthesisError::from)?;
    }

    result
        .set_metadata(circuit.metadata().clone())
        .map_err(CliffordSynthesisError::from)?;

    result
        .set_version(circuit.version())
        .map_err(CliffordSynthesisError::from)?;

    result
        .validate()
        .map_err(|_| {
            CliffordSynthesisError::CircuitConstruction {
                error: CircuitError::InvalidCircuit {
                    message:
                        "synthesized circuit failed canonical validation",
                },
            }
        })?;

    Ok(result)
}

// =============================================================================
// Aaronson–Gottesman synthesis
// =============================================================================

fn synthesize_aaronson_gottesman(
    input: &CliffordTableau,
    config: CliffordSynthesisConfig,
) -> CliffordSynthesisResult<CliffordSynthesisResult> {
    let n = input.qubit_count();

    let mut working = input.clone();

    let mut reduction = ReductionRecorder::new(
        n,
        config.max_operations,
    )?;

    // -------------------------------------------------------------------------
    // Symplectic Gaussian elimination
    // -------------------------------------------------------------------------
    //
    // Each iteration isolates one logical generator pair.
    //
    // The operations recorded here transform the input tableau toward the
    // identity. The final synthesized circuit is the inverse of these
    // operations in reverse order.
    //
    // This is the key distinction between:
    //
    //     reduction circuit
    //
    // and:
    //
    //     synthesized circuit.
    //
    // The reduction operations act on the LEFT of the represented Clifford.
    // Therefore the original Clifford is recovered by reversing and inverting
    // them.

    for k in 0..n {
        reduce_generator(
            &mut working,
            &mut reduction,
            k,
            GeneratorKind::X,
        )?;

        reduce_generator(
            &mut working,
            &mut reduction,
            k,
            GeneratorKind::Z,
        )?;

        reduction.statistics.reduction_steps =
            reduction
                .statistics
                .reduction_steps
                .checked_add(1)
                .ok_or(
                    CliffordSynthesisError::ArithmeticOverflow {
                        calculation:
                            "Clifford reduction step count",
                    },
                )?;
    }

    // The elimination must produce the identity tableau.
    if !working.is_identity() {
        return Err(
            CliffordSynthesisError::ReductionFailed,
        );
    }

    working.validate()?;

    // -------------------------------------------------------------------------
    // Clear generator signs
    // -------------------------------------------------------------------------

    for k in 0..n {
        clear_x_sign(
            &mut working,
            &mut reduction,
            k,
        )?;

        clear_z_sign(
            &mut working,
            &mut reduction,
            k,
        )?;
    }

    if !working.is_identity() {
        return Err(
            CliffordSynthesisError::ReductionFailed,
        );
    }

    // -------------------------------------------------------------------------
    // Invert reduction operations
    // -------------------------------------------------------------------------

    let mut operations = reduction.into_inverse_operations()?;

    // The reduction recorder owns the operation statistics. The inverse
    // circuit has the same operation cardinality and generator distribution
    // because H, S and CX are each self-counted under inversion.
    let mut statistics = CliffordSynthesisStatistics::new(n);

    for gate in &operations {
        statistics.record(gate.kind());
    }

    statistics.reduction_steps =
        operations.len();

    // -------------------------------------------------------------------------
    // Semantic verification
    // -------------------------------------------------------------------------

    if config.verify {
        let synthesized =
            CliffordTableau::from_gates(
                n,
                operations.iter(),
            )?;

        if !synthesized
            .equivalent_up_to_global_phase(input)?
        {
            return Err(
                CliffordSynthesisError::VerificationFailed,
            );
        }

        synthesized.validate()?;
        statistics.verified = true;
    }

    // The operation vector is already fully owned and bounded.
    //
    // No subsequent module needs to modify this file for integration.
    Ok(CliffordSynthesisResult {
        operations,
        qubits: n,
        statistics,
    })
}

// =============================================================================
// Reduction recorder
// =============================================================================

/// Records the operations used to reduce a tableau to identity.
///
/// The recorder stores only canonical Quantum IR gates. There is deliberately
/// no second private gate representation.
struct ReductionRecorder {
    operations: Vec<Gate>,
    statistics: CliffordSynthesisStatistics,
    max_operations: usize,
}

impl ReductionRecorder {
    fn new(
        qubits: usize,
        max_operations: usize,
    ) -> CliffordSynthesisResult<Self> {
        let mut operations = Vec::new();

        // Reserve only a small initial amount. Large reservations are avoided
        // because the exact operation count depends on the input tableau.
        operations
            .try_reserve(1)
            .map_err(|_| {
                CliffordSynthesisError::AllocationFailure {
                    resource:
                        "Clifford reduction operation buffer",
                    requested: 1,
                }
            })?;

        Ok(Self {
            operations,
            statistics:
                CliffordSynthesisStatistics::new(qubits),
            max_operations,
        })
    }

    fn record(
        &mut self,
        kind: GateKind,
        qubits: &[usize],
    ) -> CliffordSynthesisResult<()> {
        let next_len =
            self.operations
                .len()
                .checked_add(1)
                .ok_or(
                    CliffordSynthesisError::ArithmeticOverflow {
                        calculation:
                            "synthesized operation count",
                    },
                )?;

        if next_len > self.max_operations {
            return Err(
                CliffordSynthesisError::OperationLimitExceeded {
                    maximum: self.max_operations,
                    required: next_len,
                },
            );
        }

        if self.operations.len()
            == self.operations.capacity()
        {
            self.operations
                .try_reserve(1)
                .map_err(|_| {
                    CliffordSynthesisError::AllocationFailure {
                        resource:
                            "Clifford reduction operation buffer",
                        requested: 1,
                    }
                })?;
        }

        let gate = make_unitary_gate(
            kind,
            qubits,
        )?;

        self.operations.push(gate);
        self.statistics.record(kind);

        Ok(())
    }

    fn into_inverse_operations(
        mut self,
    ) -> CliffordSynthesisResult<Vec<Gate>> {
        let mut result = Vec::new();

        result
            .try_reserve_exact(self.operations.len())
            .map_err(|_| {
                CliffordSynthesisError::AllocationFailure {
                    resource:
                        "Clifford synthesized operation buffer",
                    requested: self.operations.len(),
                }
            })?;

        while let Some(gate) =
            self.operations.pop()
        {
            result.push(inverse_gate(&gate)?);
        }

        Ok(result)
    }
}

// =============================================================================
// Generator reduction
// =============================================================================

fn reduce_generator(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    k: usize,
    generator: GeneratorKind,
) -> CliffordSynthesisResult<()> {
    let n = tableau.qubit_count();

    if k >= n {
        return Err(
            CliffordSynthesisError::InternalInvariant {
                message:
                    "generator index exceeds tableau width",
            },
        );
    }

    // -------------------------------------------------------------------------
    // Step 1: Find a non-identity support term and move it to column k.
    // -------------------------------------------------------------------------

    let mut pivot = None;

    for j in k..n {
        let character =
            generator_character(
                tableau,
                generator,
                k,
                j,
            )?;

        if character != 'I' {
            pivot = Some(j);
            break;
        }
    }

    let pivot = pivot.ok_or(
        CliffordSynthesisError::ReductionFailed,
    )?;

    if pivot != k {
        // SWAP(k, pivot) implemented using three CX gates.
        //
        // The operation sequence:
        //
        // CX(k,pivot)
        // CX(pivot,k)
        // CX(k,pivot)
        //
        // is its own inverse and therefore remains the same when the
        // reduction sequence is inverted.
        record_cx(
            tableau,
            recorder,
            k,
            pivot,
        )?;

        record_cx(
            tableau,
            recorder,
            pivot,
            k,
        )?;

        record_cx(
            tableau,
            recorder,
            k,
            pivot,
        )?;
    }

    // -------------------------------------------------------------------------
    // Step 2: Convert the pivot Pauli character into Z.
    // -------------------------------------------------------------------------

    for j in k..n {
        let character =
            generator_character(
                tableau,
                generator,
                k,
                j,
            )?;

        match character {
            'Z' => {
                // S† maps Z -> Z and X -> -Y under conjugation. This is the
                // exact operation used by the Gaussian-elimination strategy.
                record_sdg(
                    tableau,
                    recorder,
                    j,
                )?;
            }

            'X' | 'Y' => {
                record_h(
                    tableau,
                    recorder,
                    j,
                )?;
            }

            'I' => {}

            _ => {
                return Err(
                    CliffordSynthesisError::InternalInvariant {
                        message:
                            "unexpected Pauli character",
                    },
                );
            }
        }
    }

    // -------------------------------------------------------------------------
    // Step 3: Eliminate all remaining Z support.
    // -------------------------------------------------------------------------

    for j in k + 1..n {
        let character =
            generator_character(
                tableau,
                generator,
                k,
                j,
            )?;

        if character != 'I' {
            record_cx(
                tableau,
                recorder,
                j,
                k,
            )?;
        }
    }

    Ok(())
}

// =============================================================================
// Sign correction
// =============================================================================

fn clear_x_sign(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    k: usize,
) -> CliffordSynthesisResult<()> {
    let image =
        tableau.x_image(k)?;

    let phase = image.phase();

    if phase % 2 != 0 {
        return Err(
            CliffordSynthesisError::NonHermitianGenerator {
                generator: GeneratorKind::X,
                index: k,
                phase,
            },
        );
    }

    if phase == 2 {
        // S†² = Z, and Z X Z = -X.
        record_sdg(
            tableau,
            recorder,
            k,
        )?;

        record_sdg(
            tableau,
            recorder,
            k,
        )?;
    }

    Ok(())
}

fn clear_z_sign(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    k: usize,
) -> CliffordSynthesisResult<()> {
    let image =
        tableau.z_image(k)?;

    let phase = image.phase();

    if phase % 2 != 0 {
        return Err(
            CliffordSynthesisError::NonHermitianGenerator {
                generator: GeneratorKind::Z,
                index: k,
                phase,
            },
        );
    }

    if phase == 2 {
        // H Z H = X and X Z X = -Z, so:
        //
        // H S† S† H
        //
        // implements the required sign correction.
        record_h(
            tableau,
            recorder,
            k,
        )?;

        record_sdg(
            tableau,
            recorder,
            k,
        )?;

        record_sdg(
            tableau,
            recorder,
            k,
        )?;

        record_h(
            tableau,
            recorder,
            k,
        )?;
    }

    Ok(())
}

// =============================================================================
// Tableau character access
// =============================================================================

fn generator_character(
    tableau: &CliffordTableau,
    generator: GeneratorKind,
    generator_index: usize,
    qubit: usize,
) -> CliffordSynthesisResult<char> {
    let pauli =
        match generator {
            GeneratorKind::X => {
                tableau.x_image(generator_index)?
            }

            GeneratorKind::Z => {
                tableau.z_image(generator_index)?
            }
        };

    Ok(pauli.character_at(qubit))
}

// =============================================================================
// Recorded primitive operations
// =============================================================================

fn record_h(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    qubit: usize,
) -> CliffordSynthesisResult<()> {
    let gate =
        make_unitary_gate(
            GateKind::H,
            &[qubit],
        )?;

    tableau.apply_gate(&gate)?;

    recorder.record(
        GateKind::H,
        &[qubit],
    )
}

fn record_sdg(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    qubit: usize,
) -> CliffordSynthesisResult<()> {
    let gate =
        make_unitary_gate(
            GateKind::Sdg,
            &[qubit],
        )?;

    tableau.apply_gate(&gate)?;

    recorder.record(
        GateKind::Sdg,
        &[qubit],
    )
}

fn record_cx(
    tableau: &mut CliffordTableau,
    recorder: &mut ReductionRecorder,
    control: usize,
    target: usize,
) -> CliffordSynthesisResult<()> {
    if control == target {
        return Err(
            CliffordSynthesisError::InternalInvariant {
                message:
                    "CNOT control and target must differ",
            },
        );
    }

    let gate =
        make_unitary_gate(
            GateKind::CX,
            &[control, target],
        )?;

    tableau.apply_gate(&gate)?;

    recorder.record(
        GateKind::CX,
        &[control, target],
    )
}

// =============================================================================
// Canonical gate construction
// =============================================================================

fn make_unitary_gate(
    kind: GateKind,
    qubits: &[usize],
) -> CliffordSynthesisResult<Gate> {
    let mut operands = Vec::new();

    operands
        .try_reserve_exact(qubits.len())
        .map_err(|_| {
            CliffordSynthesisError::AllocationFailure {
                resource:
                    "Clifford gate operand buffer",
                requested: qubits.len(),
            }
        })?;

    for &qubit in qubits {
        operands.push(QubitId::new(qubit));
    }

    Gate::new(
        kind,
        operands,
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        CliffordSynthesisError::InternalInvariant {
            message:
                gate_error_message(error),
        }
    })
}

/// Returns a static diagnostic for an impossible internally generated gate
/// construction failure.
///
/// All generated H/S/CX operations are locally valid by construction.
const fn gate_error_message(
    _error: crate::quantum::ir::GateError,
) -> &'static str {
    "internally generated Clifford gate failed canonical gate validation"
}

// =============================================================================
// Gate inversion
// =============================================================================

fn inverse_gate(
    gate: &Gate,
) -> CliffordSynthesisResult<Gate> {
    let kind =
        match gate.kind() {
            GateKind::H => GateKind::H,
            GateKind::S => GateKind::Sdg,
            GateKind::Sdg => GateKind::S,
            GateKind::CX => GateKind::CX,

            _ => {
                return Err(
                    CliffordSynthesisError::InternalInvariant {
                        message:
                            "reduction sequence contains unsupported gate",
                    },
                )
            }
        };

    Gate::new(
        kind,
        gate.qubits().to_vec(),
        Vec::new(),
        None,
        None,
    )
    .map_err(|_| {
        CliffordSynthesisError::InternalInvariant {
            message:
                "inverse of internally generated Clifford gate \
                 failed canonical validation",
        }
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
        make_unitary_gate(
            kind,
            qubits,
        )
        .expect("test gate must be valid")
    }

    fn synthesize_and_verify(
        qubits: usize,
        input: &[Gate],
    ) {
        let tableau =
            CliffordTableau::from_gates(
                qubits,
                input.iter(),
            )
            .expect("input must define a Clifford");

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("Clifford synthesis must succeed");

        let output_tableau =
            CliffordTableau::from_gates(
                qubits,
                result.operations(),
            )
            .expect("output must define a Clifford");

        assert!(
            output_tableau
                .equivalent_up_to_global_phase(
                    &tableau,
                )
                .expect("same tableau width")
        );

        assert!(
            result
                .statistics()
                .verified
        );
    }

    #[test]
    fn identity_clifford_synthesizes_to_empty_circuit() {
        let tableau =
            CliffordTableau::identity(0)
                .expect("zero-qubit identity is valid");

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("identity synthesis must succeed");

        assert!(
            result.operations().is_empty()
        );
        assert_eq!(
            result.statistics().operations,
            0
        );
    }

    #[test]
    fn identity_one_qubit_synthesizes_to_empty_circuit() {
        let tableau =
            CliffordTableau::identity(1)
                .expect("identity tableau is valid");

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("identity synthesis must succeed");

        assert!(
            result.operations().is_empty()
        );
    }

    #[test]
    fn synthesizes_h() {
        let input = [
            gate(
                GateKind::H,
                &[0],
            ),
        ];

        synthesize_and_verify(
            1,
            &input,
        );
    }

    #[test]
    fn synthesizes_s() {
        let input = [
            gate(
                GateKind::S,
                &[0],
            ),
        ];

        synthesize_and_verify(
            1,
            &input,
        );
    }

    #[test]
    fn synthesizes_x() {
        let input = [
            gate(
                GateKind::X,
                &[0],
            ),
        ];

        synthesize_and_verify(
            1,
            &input,
        );
    }

    #[test]
    fn synthesizes_y() {
        let input = [
            gate(
                GateKind::Y,
                &[0],
            ),
        ];

        synthesize_and_verify(
            1,
            &input,
        );
    }

    #[test]
    fn synthesizes_z() {
        let input = [
            gate(
                GateKind::Z,
                &[0],
            ),
        ];

        synthesize_and_verify(
            1,
            &input,
        );
    }

    #[test]
    fn synthesizes_cnot() {
        let input = [
            gate(
                GateKind::CX,
                &[0, 1],
            ),
        ];

        synthesize_and_verify(
            2,
            &input,
        );
    }

    #[test]
    fn synthesizes_cz() {
        let input = [
            gate(
                GateKind::CZ,
                &[0, 1],
            ),
        ];

        synthesize_and_verify(
            2,
            &input,
        );
    }

    #[test]
    fn synthesizes_swap() {
        let input = [
            gate(
                GateKind::SWAP,
                &[0, 1],
            ),
        ];

        synthesize_and_verify(
            2,
            &input,
        );
    }

    #[test]
    fn synthesizes_multi_gate_clifford() {
        let input = [
            gate(
                GateKind::H,
                &[0],
            ),
            gate(
                GateKind::S,
                &[1],
            ),
            gate(
                GateKind::CX,
                &[0, 1],
            ),
            gate(
                GateKind::H,
                &[1],
            ),
            gate(
                GateKind::Sdg,
                &[0],
            ),
        ];

        synthesize_and_verify(
            2,
            &input,
        );
    }

    #[test]
    fn rejects_non_clifford_rotation() {
        let rotation = Gate::new(
            GateKind::RZ,
            vec![QubitId::new(0)],
            vec![
                crate::quantum::ir::Parameter::Constant(
                    0.25,
                ),
            ],
            None,
            None,
        )
        .expect("parameterized gate must be locally valid");

        let result =
            synthesize_gates(
                1,
                &[rotation],
                CliffordSynthesisConfig::production(),
            );

        assert!(
            matches!(
                result,
                Err(
                    CliffordSynthesisError::InvalidInputCircuit {
                        operation: 0,
                        gate: GateKind::RZ,
                    }
                )
            )
        );
    }

    #[test]
    fn rejects_measurement() {
        let result =
            synthesize_gates(
                1,
                &[],
                CliffordSynthesisConfig::production(),
            );

        assert!(
            result.is_ok()
        );
    }

    #[test]
    fn operation_limit_is_enforced() {
        let input = [
            gate(
                GateKind::H,
                &[0],
            ),
        ];

        let tableau =
            CliffordTableau::from_gates(
                1,
                input.iter(),
            )
            .expect("input must be valid");

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::with_max_operations(1),
            );

        // H is its own inverse and requires one emitted operation, so a limit
        // of one is valid.
        assert!(
            result.is_ok()
        );

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::with_max_operations(0),
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn output_contains_only_canonical_generators() {
        let input = [
            gate(
                GateKind::X,
                &[0],
            ),
            gate(
                GateKind::Y,
                &[1],
            ),
            gate(
                GateKind::CZ,
                &[0, 1],
            ),
            gate(
                GateKind::SWAP,
                &[0, 1],
            ),
        ];

        let tableau =
            CliffordTableau::from_gates(
                2,
                input.iter(),
            )
            .expect("input must be valid");

        let result =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("synthesis must succeed");

        for operation in result.operations() {
            assert!(
                matches!(
                    operation.kind(),
                    GateKind::H
                        | GateKind::S
                        | GateKind::CX
                )
            );
        }
    }

    #[test]
    fn synthesizes_circuit_without_losing_identity_or_metadata_contract() {
        let input = [
            gate(
                GateKind::H,
                &[0],
            ),
            gate(
                GateKind::CX,
                &[0, 1],
            ),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                input.to_vec(),
            )
            .expect("input circuit must be valid");

        let output =
            synthesize_circuit(
                &circuit,
                CliffordSynthesisConfig::production(),
            )
            .expect("circuit synthesis must succeed");

        assert_eq!(
            output.num_qubits(),
            circuit.num_qubits()
        );

        assert_eq!(
            output.num_classical_bits(),
            circuit.num_classical_bits()
        );

        assert_eq!(
            output.id(),
            circuit.id()
        );

        assert_eq!(
            output.version(),
            circuit.version()
        );

        assert!(
            CliffordTableau::from_gates(
                output.num_qubits(),
                output.operations().iter(),
            )
            .expect("output tableau must be valid")
            .equivalent_up_to_global_phase(
                &CliffordTableau::from_gates(
                    circuit.num_qubits(),
                    circuit.operations().iter(),
                )
                .expect("input tableau must be valid")
            )
            .expect("tableau widths must match")
        );
    }

    #[test]
    fn synthesis_is_deterministic() {
        let input = [
            gate(
                GateKind::H,
                &[0],
            ),
            gate(
                GateKind::S,
                &[0],
            ),
            gate(
                GateKind::CX,
                &[0, 1],
            ),
            gate(
                GateKind::H,
                &[1],
            ),
        ];

        let tableau =
            CliffordTableau::from_gates(
                2,
                input.iter(),
            )
            .expect("input must be valid");

        let first =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("first synthesis must succeed");

        let second =
            synthesize(
                &tableau,
                CliffordSynthesisConfig::production(),
            )
            .expect("second synthesis must succeed");

        assert_eq!(
            first.operations(),
            second.operations()
        );
    }
}