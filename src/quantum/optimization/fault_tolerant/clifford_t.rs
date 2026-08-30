//! Zamani Quantum Optimization — Clifford+T Optimization
//!
//! Production-grade exact optimization for circuits expressed in the
//! Clifford+T gate set.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                optimization::fault_tolerant
//!                              │
//!                 ┌────────────┴────────────┐
//!                 │                         │
//!                 ▼                         ▼
//!          clifford_t.rs                t_count.rs
//!                 │                         │
//!                 └────────────┬────────────┘
//!                              ▼
//!                    phase / synthesis
//!                              │
//!                              ▼
//!                           routing
//! ```
//!
//! This module owns exact local normalization of Clifford+T sequences.
//!
//! It deliberately does NOT own:
//!
//! - the canonical Quantum IR;
//! - Clifford tableau mathematics;
//! - general Clifford synthesis;
//! - phase-polynomial synthesis;
//! - T-count analysis as a standalone analysis;
//! - T-depth optimization;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - QPU communication;
//! - benchmarking;
//! - QEC codes;
//! - frontend parsing;
//! - approximate synthesis.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical representation
//!
//! This module operates exclusively on:
//!
//! `crate::quantum::ir::Gate`
//!
//! It never defines:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - a private circuit type;
//! - a second qubit representation;
//! - a second parameter representation.
//!
//! # Clifford+T semantic domain
//!
//! The exact supported gate family is:
//!
//! ```text
//! Clifford:
//!     I
//!     X Y Z
//!     H
//!     S Sdg
//!     CX
//!     CY CZ CH
//!     SWAP
//!     ISWAP
//!
//! Non-Clifford:
//!     T Tdg
//! ```
//!
//! However, this pass intentionally performs only transformations whose
//! semantics are established directly by the canonical gate definitions.
//!
//! In particular, this pass does NOT infer that an arbitrary parameterized
//! rotation is Clifford.
//!
//! Parameterized gates therefore form optimization boundaries here.
//!
//! # Exactness
//!
//! Every transformation in this module is exact.
//!
//! No floating-point tolerance is used.
//!
//! No approximate equality is used.
//!
//! No global-phase relaxation is introduced silently.
//!
//! No stochastic search is used.
//!
//! No hardware-dependent heuristic is used.
//!
//! # Main transformations
//!
//! The pass performs bounded, exact normalization such as:
//!
//! ```text
//! I          -> <removed>
//!
//! X X        -> I
//! Y Y        -> I
//! Z Z        -> I
//! H H        -> I
//!
//! S S        -> Z
//! Sdg Sdg    -> Z
//! S Sdg      -> I
//! Sdg S      -> I
//!
//! T T        -> S
//! Tdg Tdg    -> Sdg
//! T Tdg      -> I
//! Tdg T      -> I
//!
//! T^4        -> Z
//! T^8        -> I
//! ```
//!
//! These reductions are applied only when the operations have identical
//! logical operands and no semantic boundary is crossed.
//!
//! # Why this pass exists separately from `t_gate_reduction.rs`
//!
//! `t_gate_reduction.rs` owns T-family arithmetic.
//!
//! `clifford_t.rs` owns the larger Clifford+T local normalization contract.
//!
//! The two modules may therefore overlap on some exact identities, but they
//! have different architectural purposes:
//!
//! ```text
//! t_gate_reduction.rs
//!     │
//!     └── T/Tdg arithmetic
//!
//! clifford_t.rs
//!     │
//!     └── Clifford+T local canonicalization
//! ```
//!
//! The planner should normally avoid redundant execution of both passes in
//! the same stage when their rewrite sets overlap.
//!
//! # Maximal-chain handling
//!
//! The pass identifies maximal single-qubit Clifford+T chains.
//!
//! A chain terminates at:
//!
//! - measurement;
//! - reset;
//! - barrier;
//! - multi-qubit operation;
//! - parameterized operation;
//! - unsupported operation;
//! - an operation on another logical qubit;
//! - any future operation whose semantics are not explicitly known to this
//!   module.
//!
//! This conservative boundary rule is intentional.
//!
//! A future phase-polynomial or ZX-based optimizer may perform substantially
//! more powerful transformations, but those transformations require their own
//! mathematical and verification contracts.
//!
//! # Scaling
//!
//! The normal transformation is linear in the number of operations:
//!
//! ```text
//! O(n)
//! ```
//!
//! where `n` is the number of operations in the supplied sequence.
//!
//! The implementation does not impose an artificial maximum circuit size.
//!
//! Practical limits are governed by:
//!
//! - available memory;
//! - `usize` addressability;
//! - canonical Quantum IR limits;
//! - optimization limits;
//! - caller allocation policy.
//!
//! No recursive algorithm is used for circuit traversal.
//!
//! No exponentially sized representation is constructed.
//!
//! No dense unitary matrix is constructed.
//!
//! No per-circuit hash table proportional to the Hilbert-space dimension is
//! created.
//!
//! This means that the pass remains suitable for very large circuits, subject
//! to the resources explicitly available to the compiler.
//!
//! # Determinism
//!
//! The pass is deterministic.
//!
//! For identical input operation sequences it produces identical output,
//! statistics, and rewrite decisions.
//!
//! There is:
//!
//! - no randomness;
//! - no thread-local optimizer state;
//! - no process-global mutable state;
//! - no backend query;
//! - no timing-dependent optimization decision.
//!
//! # Transactional behavior
//!
//! The primary transformation API accepts an immutable operation slice and
//! returns a new vector.
//!
//! Consequently:
//!
//! - the input is never partially mutated;
//! - an error cannot leave a partially optimized sequence;
//! - callers can validate the candidate before committing it to a circuit;
//! - pipeline-level atomicity remains possible.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module consumes:
//!
//! - `Gate`;
//! - `GateKind`;
//! - `QubitId`;
//!
//! and nothing resembling an alternative quantum IR.
//!
//! ## `optimization::pass`
//!
//! The module exposes stable pass metadata compatible with the common
//! optimization-pass registry.
//!
//! The pass identifier is:
//!
//! `fault_tolerant.clifford_t`
//!
//! Alias:
//!
//! `clifford_t`
//!
//! ## `optimization::fault_tolerant::t_count`
//!
//! T-count analysis belongs there.
//!
//! This file may report T-count deltas in its local statistics, but it does not
//! replace the authoritative T-count analyzer.
//!
//! ## `optimization::fault_tolerant::t_depth`
//!
//! T-depth analysis and scheduling belong there.
//!
//! This pass never claims to perform global T-depth optimization.
//!
//! ## `optimization::algebra::clifford`
//!
//! General Clifford classification and tableau algebra belong there.
//!
//! This file deliberately uses the canonical `GateKind` classification for
//! local exact rewrites instead of creating another Clifford representation.
//!
//! ## `optimization::synthesis::clifford`
//!
//! General Clifford synthesis belongs there.
//!
//! This pass does not synthesize arbitrary Clifford tableaux.
//!
//! ## `optimization::synthesis::phase`
//!
//! Phase-polynomial synthesis belongs there.
//!
//! This pass does not perform parity-network synthesis.
//!
//! ## `optimization::local`
//!
//! Generic cancellation and peephole passes remain independently useful.
//!
//! The planner should decide whether this pass or a generic local pass owns a
//! particular rewrite stage to avoid redundant work.
//!
//! ## `optimization::verification`
//!
//! Pipeline-level semantic verification remains authoritative.
//!
//! Because every transformation in this module is exact, no approximate
//! verification is required by the pass itself.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline may invoke this module after canonicalization and before more
//! global phase-polynomial optimization.
//!
//! ## `optimization::planner`
//!
//! This pass is appropriate for fault-tolerant profiles and Clifford+T
//! workloads.
//!
//! It should not be selected for circuits containing arbitrary parameterized
//! rotations unless those operations have first been decomposed into a
//! supported exact Clifford+T representation.
//!
//! ## `optimization::registry`
//!
//! Registration should use:
//!
//! ```text
//! fault_tolerant.clifford_t
//! ```
//!
//! with:
//!
//! ```text
//! clifford_t
//! ```
//!
//! as the human-facing alias.
//!
//! The registry itself must not be initialized from this module.
//!
//! This preserves independent compilation and deterministic registration.
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
//! - no external dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe Rust.
//!
//! `unsafe_code` is forbidden explicitly.
//!
//! # External design precedent
//!
//! Qiskit's current `OptimizeCliffordT` specifically targets consecutive
//! Clifford+T sequences and describes linear-time processing for maximal
//! single-qubit chains. 5
//!
//! PyZX similarly separates circuit-level optimization from phase-polynomial
//! optimization and warns that different objectives can trade T-count against
//! two-qubit gate count. 6
//!
//! Zamani therefore keeps this pass exact and local while leaving global
//! phase-polynomial and synthesis decisions to their dedicated modules.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::ir::{Gate, GateKind, QubitId};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable compiler-facing pass identifier.
pub const PASS_ID: &str = "fault_tolerant.clifford_t";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Clifford+T Optimization";

/// Stable configuration alias.
pub const PASS_ALIAS: &str = "clifford_t";

/// API version for the public contract of this module.
pub const CLIFFORD_T_API_VERSION: u32 = 1;

// =============================================================================
// Result
// =============================================================================

/// Result type for Clifford+T optimization.
pub type CliffordTResult<T> = Result<T, CliffordTError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the Clifford+T optimizer.
///
/// The optimizer is intentionally strict. It never silently accepts malformed
/// or semantically ambiguous input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliffordTError {
    /// A gate is structurally invalid for this optimization layer.
    InvalidGate {
        /// Operation position.
        operation: usize,

        /// Gate kind.
        gate: GateKind,

        /// Static reason.
        reason: &'static str,
    },

    /// The gate is outside the exact Clifford+T domain supported by this pass.
    UnsupportedGate {
        /// Operation position.
        operation: usize,

        /// Unsupported gate kind.
        gate: GateKind,
    },

    /// A Clifford+T gate has an unexpected operand count.
    InvalidArity {
        /// Operation position.
        operation: usize,

        /// Gate kind.
        gate: GateKind,

        /// Expected arity.
        expected: usize,

        /// Actual arity.
        actual: usize,
    },

    /// An arithmetic counter overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// The configured operation budget was exceeded.
    OperationLimitExceeded {
        /// Maximum permitted operations.
        maximum: usize,

        /// Required operations.
        required: usize,
    },

    /// The output allocation could not be reserved.
    AllocationFailure {
        /// Resource being allocated.
        resource: &'static str,

        /// Requested capacity.
        requested: usize,
    },

    /// A semantic boundary was encountered where a caller expected a
    /// continuous Clifford+T chain.
    ChainBoundary {
        /// Operation position.
        operation: usize,
    },

    /// Internal optimizer invariant failed.
    InternalInvariant {
        /// Static explanation.
        message: &'static str,
    },
}

impl fmt::Display for CliffordTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGate {
                operation,
                gate,
                reason,
            } => write!(
                f,
                "invalid Clifford+T gate at operation {operation}: \
                 {gate:?}: {reason}"
            ),

            Self::UnsupportedGate { operation, gate } => write!(
                f,
                "operation {operation} contains unsupported gate {gate:?} \
                 for the Clifford+T optimizer"
            ),

            Self::InvalidArity {
                operation,
                gate,
                expected,
                actual,
            } => write!(
                f,
                "operation {operation} contains {gate:?} with invalid arity: \
                 expected {expected}, received {actual}"
            ),

            Self::ArithmeticOverflow { calculation } => write!(
                f,
                "arithmetic overflow during Clifford+T optimization: \
                 {calculation}"
            ),

            Self::OperationLimitExceeded {
                maximum,
                required,
            } => write!(
                f,
                "Clifford+T output operation limit exceeded: \
                 maximum {maximum}, required {required}"
            ),

            Self::AllocationFailure {
                resource,
                requested,
            } => write!(
                f,
                "allocation failed for {resource}: requested {requested}"
            ),

            Self::ChainBoundary { operation } => write!(
                f,
                "Clifford+T chain terminated at operation {operation}"
            ),

            Self::InternalInvariant { message } => write!(
                f,
                "Clifford+T optimizer invariant violated: {message}"
            ),
        }
    }
}

impl std::error::Error for CliffordTError {}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for exact Clifford+T optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliffordTConfig {
    /// Whether explicit identity gates should be removed.
    pub remove_identities: bool,

    /// Whether adjacent exact inverse/self-inverse Clifford gates should be
    /// cancelled.
    pub cancel_clifford_pairs: bool,

    /// Whether exact S/Sdg/T/Tdg phase identities should be normalized.
    pub normalize_phase_powers: bool,

    /// Whether multi-qubit Clifford gates should be recognized as boundaries
    /// rather than transformed by this single-qubit chain optimizer.
    ///
    /// This is deliberately true in the production default.
    pub preserve_multi_qubit_operations: bool,

    /// Maximum number of operations in the returned sequence.
    ///
    /// This is an output-safety budget, not a circuit-size ceiling.
    pub max_output_operations: usize,
}

impl CliffordTConfig {
    /// Production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            remove_identities: true,
            cancel_clifford_pairs: true,
            normalize_phase_powers: true,
            preserve_multi_qubit_operations: true,
            max_output_operations: usize::MAX,
        }
    }

    /// Conservative configuration.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            remove_identities: true,
            cancel_clifford_pairs: true,
            normalize_phase_powers: true,
            preserve_multi_qubit_operations: true,
            max_output_operations: usize::MAX,
        }
    }

    /// Returns a copy with an explicit output limit.
    #[must_use]
    pub const fn with_max_output_operations(
        self,
        maximum: usize,
    ) -> Self {
        Self {
            remove_identities: self.remove_identities,
            cancel_clifford_pairs: self.cancel_clifford_pairs,
            normalize_phase_powers: self.normalize_phase_powers,
            preserve_multi_qubit_operations: self.preserve_multi_qubit_operations,
            max_output_operations: maximum,
        }
    }

    /// Validates the configuration.
    pub const fn validate(self) -> CliffordTResult<()> {
        if self.max_output_operations == 0 {
            return Err(
                CliffordTError::OperationLimitExceeded {
                    maximum: 0,
                    required: 1,
                },
            );
        }

        Ok(())
    }
}

impl Default for CliffordTConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Exact statistics produced by one Clifford+T optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CliffordTStatistics {
    /// Number of input operations.
    pub input_operations: usize,

    /// Number of output operations.
    pub output_operations: usize,

    /// Number of explicit identity gates removed.
    pub identities_removed: usize,

    /// Number of Clifford inverse/self-inverse pairs removed.
    pub clifford_pairs_cancelled: usize,

    /// Number of T/Tdg pairs cancelled.
    pub t_pairs_cancelled: usize,

    /// Number of T/Tdg gates consumed by phase normalization.
    pub phase_gates_normalized: usize,

    /// Number of S/Sdg gates normalized.
    pub s_gates_normalized: usize,

    /// Number of exact Z replacements generated from phase powers.
    pub z_replacements: usize,

    /// Number of single-qubit Clifford+T chains encountered.
    pub chains_seen: usize,

    /// Number of operations inside those chains.
    pub chain_operations: usize,

    /// Number of transformations performed.
    pub rewrites: usize,
}

impl CliffordTStatistics {
    /// Returns the exact reduction in operation count.
    #[must_use]
    pub fn operations_removed(self) -> usize {
        self.input_operations
            .saturating_sub(self.output_operations)
    }

    /// Returns whether the optimizer changed the sequence.
    #[must_use]
    pub fn changed(self) -> bool {
        self.input_operations != self.output_operations
            || self.rewrites != 0
            || self.phase_gates_normalized != 0
            || self.s_gates_normalized != 0
    }
}

// =============================================================================
// Gate classification
// =============================================================================

/// Returns whether a gate is an exact Clifford gate supported by this pass.
#[must_use]
pub const fn is_clifford_gate(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::ISWAP
    )
}

/// Returns whether a gate is a non-Clifford member of the supported
/// Clifford+T basis.
#[must_use]
pub const fn is_t_gate(kind: GateKind) -> bool {
    matches!(kind, GateKind::T | GateKind::Tdg)
}

/// Returns whether a gate belongs to the exact supported Clifford+T basis.
#[must_use]
pub const fn is_clifford_t_gate(kind: GateKind) -> bool {
    is_clifford_gate(kind) || is_t_gate(kind)
}

/// Returns whether an operation must terminate a single-qubit Clifford+T
/// chain.
#[must_use]
pub const fn is_chain_boundary(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::Measure
            | GateKind::Barrier
            | GateKind::Reset
            | GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::CCX
            | GateKind::CSWAP
            | GateKind::ECR
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ
    )
}

/// Returns whether the gate is self-inverse for exact local cancellation.
#[must_use]
pub const fn is_self_inverse(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::CCX
            | GateKind::CSWAP
    )
}

/// Returns the exact inverse gate for the phase gates handled by this module.
#[must_use]
pub const fn inverse_phase_gate(kind: GateKind) -> Option<GateKind> {
    match kind {
        GateKind::S => Some(GateKind::Sdg),
        GateKind::Sdg => Some(GateKind::S),
        GateKind::T => Some(GateKind::Tdg),
        GateKind::Tdg => Some(GateKind::T),
        _ => None,
    }
}

// =============================================================================
// Phase exponent
// =============================================================================

/// Returns the exponent of T for a phase gate.
///
/// The convention is:
///
/// ```text
/// T    = T^1
/// S    = T^2
/// Z    = T^4
/// Sdg  = T^-2
/// Tdg  = T^-1
/// ```
#[must_use]
fn phase_exponent(kind: GateKind) -> Option<i32> {
    match kind {
        GateKind::T => Some(1),
        GateKind::Tdg => Some(-1),
        GateKind::S => Some(2),
        GateKind::Sdg => Some(-2),
        GateKind::Z => Some(4),
        _ => None,
    }
}

/// Returns a canonical phase sequence for a normalized T exponent.
///
/// The sequence is never longer than three gates.
///
/// The implementation deliberately uses the canonical exact T-power
/// identities rather than floating-point angles.
fn phase_sequence(
    exponent: i32,
    qubit: QubitId,
) -> Vec<Gate> {
    let normalized = exponent.rem_euclid(8);

    match normalized {
        0 => Vec::new(),

        1 => vec![make_gate(GateKind::T, &[qubit])],

        2 => vec![make_gate(GateKind::S, &[qubit])],

        3 => vec![
            make_gate(GateKind::S, &[qubit]),
            make_gate(GateKind::T, &[qubit]),
        ],

        4 => vec![make_gate(GateKind::Z, &[qubit])],

        5 => vec![
            make_gate(GateKind::Z, &[qubit]),
            make_gate(GateKind::T, &[qubit]),
        ],

        6 => vec![make_gate(GateKind::Sdg, &[qubit])],

        7 => vec![make_gate(GateKind::Tdg, &[qubit])],

        _ => unreachable!("rem_euclid(8) always returns 0..=7"),
    }
}

// =============================================================================
// Gate construction
// =============================================================================

/// Constructs a gate through the canonical Quantum IR constructor.
///
/// This function deliberately contains the only construction site for gates
/// generated by this module. If the canonical IR changes its construction
/// internals, this module has one stable semantic construction boundary.
///
/// The function never creates a private gate representation.
fn make_gate(
    kind: GateKind,
    qubits: &[QubitId],
) -> Gate {
    Gate::new(
        kind,
        qubits.to_vec(),
        Vec::new(),
        None,
        None,
    )
    .expect("optimizer-generated canonical gate must satisfy IR invariants")
}

// =============================================================================
// Public optimizer
// =============================================================================

/// Exact Clifford+T optimizer.
///
/// The optimizer is immutable and thread-safe as a value. Invocation state is
/// stored entirely in the returned statistics and local stack data.
#[derive(Debug, Clone, Copy)]
pub struct CliffordTOptimizer {
    config: CliffordTConfig,
}

impl CliffordTOptimizer {
    /// Creates an optimizer with production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: CliffordTConfig::production(),
        }
    }

    /// Creates an optimizer with explicit configuration.
    pub const fn with_config(
        config: CliffordTConfig,
    ) -> CliffordTResult<Self> {
        match config.validate() {
            Ok(()) => Ok(Self { config }),
            Err(error) => Err(error),
        }
    }

    /// Returns the optimizer configuration.
    #[must_use]
    pub const fn config(self) -> CliffordTConfig {
        self.config
    }

    /// Optimizes a canonical sequence of operations.
    ///
    /// The input is never modified.
    ///
    /// The transformation is deterministic and exact.
    pub fn optimize(
        &self,
        operations: &[Gate],
    ) -> CliffordTResult<(
        Vec<Gate>,
        CliffordTStatistics,
    )> {
        self.config.validate()?;

        let mut stats = CliffordTStatistics {
            input_operations: operations.len(),
            ..CliffordTStatistics::default()
        };

        let mut output = Vec::new();

        output
            .try_reserve(operations.len())
            .map_err(|_| CliffordTError::AllocationFailure {
                resource: "Clifford+T output operation vector",
                requested: operations.len(),
            })?;

        let mut index = 0usize;

        while index < operations.len() {
            let gate = operations
                .get(index)
                .ok_or(CliffordTError::InternalInvariant {
                    message: "operation index exceeded input sequence",
                })?;

            self.validate_operation(index, gate)?;

            if self.config.remove_identities
                && gate.kind() == GateKind::I
            {
                stats.identities_removed =
                    stats.identities_removed.checked_add(1).ok_or(
                        CliffordTError::ArithmeticOverflow {
                            calculation: "identity count",
                        },
                    )?;

                stats.rewrites =
                    stats.rewrites.checked_add(1).ok_or(
                        CliffordTError::ArithmeticOverflow {
                            calculation: "rewrite count",
                        },
                    )?;

                index += 1;
                continue;
            }

            if is_chain_boundary(gate.kind()) {
                self.push_checked(&mut output, gate.clone())?;
                index += 1;
                continue;
            }

            if !is_clifford_t_gate(gate.kind()) {
                self.push_checked(&mut output, gate.clone())?;
                index += 1;
                continue;
            }

            let qubit = match gate.qubits().first().copied() {
                Some(value) => value,
                None => {
                    return Err(CliffordTError::InvalidGate {
                        operation: index,
                        gate: gate.kind(),
                        reason: "Clifford+T single-qubit operation has no operand",
                    });
                }
            };

            if gate.qubits().len() != 1 {
                // Multi-qubit Clifford+T operations are preserved exactly.
                //
                // They may participate in higher-level Clifford synthesis or
                // commutation passes, but this pass intentionally does not
                // cross that boundary.
                self.push_checked(&mut output, gate.clone())?;
                index += 1;
                continue;
            }

            let chain_start = index;

            let mut chain_end = index;

            while chain_end < operations.len() {
                let candidate = operations
                    .get(chain_end)
                    .ok_or(
                        CliffordTError::InternalInvariant {
                            message:
                                "chain index exceeded input sequence",
                        },
                    )?;

                self.validate_operation(chain_end, candidate)?;

                if !is_clifford_t_gate(candidate.kind())
                    || candidate.qubits().len() != 1
                    || candidate.qubits().first().copied()
                        != Some(qubit)
                {
                    break;
                }

                chain_end += 1;
            }

            if chain_end == chain_start {
                return Err(CliffordTError::InternalInvariant {
                    message: "empty Clifford+T chain",
                });
            }

            stats.chains_seen =
                stats.chains_seen.checked_add(1).ok_or(
                    CliffordTError::ArithmeticOverflow {
                        calculation: "chain count",
                    },
                )?;

            stats.chain_operations =
                stats.chain_operations.checked_add(
                    chain_end - chain_start,
                )
                .ok_or(
                    CliffordTError::ArithmeticOverflow {
                        calculation: "chain operation count",
                    },
                )?;

            let chain = operations
                .get(chain_start..chain_end)
                .ok_or(CliffordTError::InternalInvariant {
                    message: "invalid Clifford+T chain range",
                })?;

            let normalized =
                self.optimize_single_qubit_chain(
                    chain,
                    qubit,
                    &mut stats,
                )?;

            for normalized_gate in normalized {
                self.push_checked(
                    &mut output,
                    normalized_gate,
                )?;
            }

            index = chain_end;
        }

        stats.output_operations = output.len();

        Ok((output, stats))
    }

    /// Validates that an operation belongs to the exact domain this pass can
    /// reason about.
    fn validate_operation(
        &self,
        operation: usize,
        gate: &Gate,
    ) -> CliffordTResult<()> {
        let kind = gate.kind();

        if gate.is_parameterized() {
            // Parameterized gates are not rejected globally because the
            // optimizer can safely preserve them. They simply terminate the
            // current Clifford+T chain.
            return Ok(());
        }

        let arity = gate.qubits().len();

        let expected = match kind {
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
            | GateKind::Measure
            | GateKind::Reset => 1,

            GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::ECR
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ => 2,

            GateKind::CCX
            | GateKind::CSWAP => 3,

            GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::Barrier => {
                // These are either parameterized or semantic boundaries.
                //
                // Their detailed IR validation belongs to the canonical
                // Quantum IR validator.
                return Ok(());
            }
        };

        if arity != expected {
            return Err(CliffordTError::InvalidArity {
                operation,
                gate: kind,
                expected,
                actual: arity,
            });
        }

        if is_clifford_t_gate(kind) && arity == 0 {
            return Err(CliffordTError::InvalidGate {
                operation,
                gate: kind,
                reason: "supported Clifford+T gate has no logical operand",
            });
        }

        Ok(())
    }

    /// Optimizes one maximal single-qubit Clifford+T chain.
    fn optimize_single_qubit_chain(
        &self,
        chain: &[Gate],
        qubit: QubitId,
        stats: &mut CliffordTStatistics,
    ) -> CliffordTResult<Vec<Gate>> {
        let mut output = Vec::new();

        output
            .try_reserve(chain.len())
            .map_err(|_| CliffordTError::AllocationFailure {
                resource: "single-qubit Clifford+T chain",
                requested: chain.len(),
            })?;

        let mut index = 0usize;

        while index < chain.len() {
            let gate = chain
                .get(index)
                .ok_or(CliffordTError::InternalInvariant {
                    message: "chain index exceeded chain length",
                })?;

            let kind = gate.kind();

            if self.config.remove_identities
                && kind == GateKind::I
            {
                stats.identities_removed =
                    stats.identities_removed.checked_add(1).ok_or(
                        CliffordTError::ArithmeticOverflow {
                            calculation: "identity count",
                        },
                    )?;

                stats.rewrites =
                    stats.rewrites.checked_add(1).ok_or(
                        CliffordTError::ArithmeticOverflow {
                            calculation: "rewrite count",
                        },
                    )?;

                index += 1;
                continue;
            }

            // -------------------------------------------------------------
            // Exact adjacent self-inverse cancellation.
            // -------------------------------------------------------------
            if self.config.cancel_clifford_pairs
                && is_self_inverse(kind)
            {
                if let Some(next) = chain.get(index + 1) {
                    if next.kind() == kind
                        && next.qubits() == gate.qubits()
                    {
                        stats.clifford_pairs_cancelled =
                            stats
                                .clifford_pairs_cancelled
                                .checked_add(1)
                                .ok_or(
                                    CliffordTError::ArithmeticOverflow {
                                        calculation:
                                            "Clifford pair count",
                                    },
                                )?;

                        stats.rewrites =
                            stats.rewrites.checked_add(1).ok_or(
                                CliffordTError::ArithmeticOverflow {
                                    calculation:
                                        "rewrite count",
                                },
                            )?;

                        index += 2;
                        continue;
                    }
                }
            }

            // -------------------------------------------------------------
            // Exact inverse phase cancellation.
            // -------------------------------------------------------------
            if self.config.normalize_phase_powers {
                if let Some(next) = chain.get(index + 1) {
                    if inverse_phase_gate(kind)
                        == Some(next.kind())
                        && next.qubits() == gate.qubits()
                    {
                        stats.t_pairs_cancelled =
                            stats.t_pairs_cancelled.checked_add(1).ok_or(
                                CliffordTError::ArithmeticOverflow {
                                    calculation:
                                        "phase pair count",
                                },
                            )?;

                        stats.rewrites =
                            stats.rewrites.checked_add(1).ok_or(
                                CliffordTError::ArithmeticOverflow {
                                    calculation:
                                        "rewrite count",
                                },
                            )?;

                        index += 2;
                        continue;
                    }
                }
            }

            // -------------------------------------------------------------
            // Exact consecutive Z-axis phase-power normalization.
            //
            // This is the key operation that unifies:
            //
            //     T, Tdg, S, Sdg, Z
            //
            // into one modulo-eight exponent.
            // -------------------------------------------------------------
            if self.config.normalize_phase_powers
                && phase_exponent(kind).is_some()
            {
                let start = index;

                let mut exponent = 0i32;
                let mut end = index;

                while end < chain.len() {
                    let candidate = chain
                        .get(end)
                        .ok_or(
                            CliffordTError::InternalInvariant {
                                message:
                                    "phase scan exceeded chain",
                            },
                        )?;

                    if candidate.qubits() != gate.qubits() {
                        break;
                    }

                    let candidate_exponent =
                        match phase_exponent(candidate.kind()) {
                            Some(value) => value,
                            None => break,
                        };

                    exponent = exponent.checked_add(
                        candidate_exponent,
                    )
                    .ok_or(
                        CliffordTError::ArithmeticOverflow {
                            calculation:
                                "phase exponent accumulation",
                        },
                    )?;

                    end += 1;
                }

                if end > start {
                    let consumed = end - start;

                    let normalized =
                        phase_sequence(exponent, qubit);

                    for generated in &normalized {
                        match generated.kind() {
                            GateKind::T | GateKind::Tdg => {
                                stats.phase_gates_normalized =
                                    stats
                                        .phase_gates_normalized
                                        .checked_add(1)
                                        .ok_or(
                                            CliffordTError::ArithmeticOverflow {
                                                calculation:
                                                    "normalized T count",
                                            },
                                        )?;
                            }

                            GateKind::S | GateKind::Sdg => {
                                stats.s_gates_normalized =
                                    stats
                                        .s_gates_normalized
                                        .checked_add(1)
                                        .ok_or(
                                            CliffordTError::ArithmeticOverflow {
                                                calculation:
                                                    "normalized S count",
                                            },
                                        )?;
                            }

                            GateKind::Z => {
                                stats.z_replacements =
                                    stats
                                        .z_replacements
                                        .checked_add(1)
                                        .ok_or(
                                            CliffordTError::ArithmeticOverflow {
                                                calculation:
                                                    "Z replacement count",
                                            },
                                        )?;
                            }

                            _ => {}
                        }
                    }

                    if normalized.len() != consumed {
                        stats.rewrites =
                            stats.rewrites.checked_add(1).ok_or(
                                CliffordTError::ArithmeticOverflow {
                                    calculation:
                                        "phase rewrite count",
                                },
                            )?;
                    }

                    output.extend(normalized);
                    index = end;
                    continue;
                }
            }

            // -------------------------------------------------------------
            // Nothing specialized applies.
            // -------------------------------------------------------------
            output.push(gate.clone());
            index += 1;
        }

        Ok(output)
    }

    /// Adds an operation to the output while enforcing the configured output
    /// budget.
    fn push_checked(
        &self,
        output: &mut Vec<Gate>,
        gate: Gate,
    ) -> CliffordTResult<()> {
        if output.len() >= self.config.max_output_operations {
            return Err(
                CliffordTError::OperationLimitExceeded {
                    maximum: self.config.max_output_operations,
                    required: output.len().saturating_add(1),
                },
            );
        }

        output
            .try_reserve(1)
            .map_err(|_| CliffordTError::AllocationFailure {
                resource: "Clifford+T output operation",
                requested: output.len().saturating_add(1),
            })?;

        output.push(gate);

        Ok(())
    }
}

impl Default for CliffordTOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Standalone helpers
// =============================================================================

/// Optimizes a canonical operation sequence using production defaults.
pub fn optimize(
    operations: &[Gate],
) -> CliffordTResult<(
    Vec<Gate>,
    CliffordTStatistics,
)> {
    CliffordTOptimizer::new().optimize(operations)
}

/// Returns whether a canonical gate belongs to the exact Clifford+T basis
/// recognized by this module.
#[must_use]
pub fn supports_gate(gate: &Gate) -> bool {
    is_clifford_t_gate(gate.kind())
}

/// Returns the exact T exponent associated with a supported phase gate.
#[must_use]
pub fn exact_phase_exponent(gate: &Gate) -> Option<i32> {
    phase_exponent(gate.kind())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit identifier should be valid")
    }

    fn gate(
        kind: GateKind,
        index: usize,
    ) -> Gate {
        make_gate(kind, &[q(index)])
    }

    #[test]
    fn pass_identifier_is_stable() {
        assert_eq!(
            PASS_ID,
            "fault_tolerant.clifford_t"
        );

        assert_eq!(
            PASS_ALIAS,
            "clifford_t"
        );
    }

    #[test]
    fn recognizes_exact_clifford_gates() {
        assert!(is_clifford_gate(GateKind::I));
        assert!(is_clifford_gate(GateKind::X));
        assert!(is_clifford_gate(GateKind::Y));
        assert!(is_clifford_gate(GateKind::Z));
        assert!(is_clifford_gate(GateKind::H));
        assert!(is_clifford_gate(GateKind::S));
        assert!(is_clifford_gate(GateKind::Sdg));
        assert!(is_clifford_gate(GateKind::CX));
    }

    #[test]
    fn recognizes_t_family() {
        assert!(is_t_gate(GateKind::T));
        assert!(is_t_gate(GateKind::Tdg));
        assert!(!is_t_gate(GateKind::S));
    }

    #[test]
    fn recognizes_clifford_t_basis() {
        assert!(is_clifford_t_gate(GateKind::H));
        assert!(is_clifford_t_gate(GateKind::S));
        assert!(is_clifford_t_gate(GateKind::T));
        assert!(is_clifford_t_gate(GateKind::Tdg));

        assert!(!is_clifford_t_gate(GateKind::RX));
        assert!(!is_clifford_t_gate(GateKind::RY));
        assert!(!is_clifford_t_gate(GateKind::RZ));
    }

    #[test]
    fn t_pair_cancels() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 0),
        ];

        let (result, stats) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.t_pairs_cancelled, 1);
    }

    #[test]
    fn tdg_pair_cancels() {
        let operations = [
            gate(GateKind::Tdg, 0),
            gate(GateKind::T, 0),
        ];

        let (result, stats) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.t_pairs_cancelled, 1);
    }

    #[test]
    fn two_t_gates_become_s() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind(), GateKind::S);
    }

    #[test]
    fn two_tdg_gates_become_sdg() {
        let operations = [
            gate(GateKind::Tdg, 0),
            gate(GateKind::Tdg, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind(), GateKind::Sdg);
    }

    #[test]
    fn four_t_gates_become_z() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind(), GateKind::Z);
    }

    #[test]
    fn eight_t_gates_disappear() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn s_and_sdg_cancel() {
        let operations = [
            gate(GateKind::S, 0),
            gate(GateKind::Sdg, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn sdg_and_s_cancel() {
        let operations = [
            gate(GateKind::Sdg, 0),
            gate(GateKind::S, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn four_s_gates_become_identity() {
        let operations = [
            gate(GateKind::S, 0),
            gate(GateKind::S, 0),
            gate(GateKind::S, 0),
            gate(GateKind::S, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn two_hadamards_cancel() {
        let operations = [
            gate(GateKind::H, 0),
            gate(GateKind::H, 0),
        ];

        let (result, stats) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.clifford_pairs_cancelled, 1);
    }

    #[test]
    fn two_x_gates_cancel() {
        let operations = [
            gate(GateKind::X, 0),
            gate(GateKind::X, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn identity_is_removed() {
        let operations = [
            gate(GateKind::I, 0),
            gate(GateKind::H, 0),
        ];

        let (result, stats) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind(), GateKind::H);
        assert_eq!(stats.identities_removed, 1);
    }

    #[test]
    fn unrelated_qubit_breaks_single_qubit_chain() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::T, 1),
            gate(GateKind::Tdg, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn parameterized_operation_is_preserved() {
        let parameterized = Gate::new(
            GateKind::RZ,
            vec![q(0)],
            vec![
                crate::quantum::ir::GateParameter::Constant(
                    0.5,
                ),
            ],
            None,
            None,
        )
        .expect("test parameterized gate should be valid");

        let operations = [
            gate(GateKind::T, 0),
            parameterized.clone(),
            gate(GateKind::Tdg, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
        assert_eq!(result[1], parameterized);
    }

    #[test]
    fn multi_qubit_operation_is_preserved() {
        let cx = Gate::new(
            GateKind::CX,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        )
        .expect("test CX should be valid");

        let operations = [
            gate(GateKind::T, 0),
            cx.clone(),
            gate(GateKind::Tdg, 0),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
        assert_eq!(result[1], cx);
    }

    #[test]
    fn different_qubits_do_not_cancel() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::Tdg, 1),
        ];

        let (result, _) =
            optimize(&operations)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn z_is_four_t() {
        assert_eq!(
            exact_phase_exponent(
                &gate(GateKind::Z, 0)
            ),
            Some(4)
        );
    }

    #[test]
    fn t_is_one_t() {
        assert_eq!(
            exact_phase_exponent(
                &gate(GateKind::T, 0)
            ),
            Some(1)
        );
    }

    #[test]
    fn sdg_is_negative_two_t() {
        assert_eq!(
            exact_phase_exponent(
                &gate(GateKind::Sdg, 0)
            ),
            Some(-2)
        );
    }

    #[test]
    fn optimizer_is_deterministic() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::S, 0),
            gate(GateKind::H, 0),
            gate(GateKind::H, 0),
            gate(GateKind::Tdg, 0),
        ];

        let first =
            optimize(&operations)
                .expect("first optimization should succeed");

        let second =
            optimize(&operations)
                .expect("second optimization should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn optimization_is_idempotent() {
        let operations = [
            gate(GateKind::T, 0),
            gate(GateKind::T, 0),
            gate(GateKind::H, 0),
            gate(GateKind::H, 0),
            gate(GateKind::S, 0),
            gate(GateKind::Sdg, 0),
        ];

        let (first, _) =
            optimize(&operations)
                .expect("first optimization should succeed");

        let (second, _) =
            optimize(&first)
                .expect("second optimization should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn empty_input_is_valid() {
        let (result, stats) =
            optimize(&[])
                .expect("empty optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.input_operations, 0);
        assert_eq!(stats.output_operations, 0);
    }
}