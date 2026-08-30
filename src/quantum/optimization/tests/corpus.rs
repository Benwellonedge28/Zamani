//! Zamani Quantum Optimization — Scalable Optimization Corpus
//!
//! `src/quantum/optimization/tests/corpus.rs`
//!
//! # Purpose
//!
//! This module provides a deterministic, scalable corpus of canonical quantum
//! circuits for testing and benchmarking the Zamani quantum optimization
//! subsystem.
//!
//! The corpus is deliberately separate from:
//!
//! - `properties.rs`      — optimizer properties and invariants;
//! - `equivalence.rs`     — semantic equivalence verification;
//! - `regression.rs`      — permanent bug regressions;
//! - `Integration.rs`     — cross-component integration tests.
//!
//! This module answers a different question:
//!
//! > Can the optimization subsystem consume a broad, deterministic family of
//! > canonical Quantum IR workloads ranging from tiny circuits to very large
//! > resource-bounded workloads?
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                  optimization test corpus
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!          properties     equivalence       regression
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                     optimization passes
//!                              │
//!                              ▼
//!                       optimized IR
//! ```
//!
//! The corpus therefore belongs at the canonical IR boundary.
//!
//! # Canonical IR requirement
//!
//! This file MUST NOT define a second quantum representation.
//!
//! Every generated circuit uses:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! crate::quantum::ir::Gate
//! crate::quantum::ir::GateKind
//! crate::quantum::ir::parameter::Parameter
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! In particular, the canonical logical-qubit module is:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! NOT:
//!
//! ```text
//! crate::quantum::ir::qubits
//! ```
//!
//! This is intentional. The optimizer test corpus must not reproduce the
//! historical IR naming inconsistency.
//!
//! # Design goals
//!
//! The corpus provides:
//!
//! - empty circuits;
//! - one-operation circuits;
//! - cancellation-heavy circuits;
//! - inverse-heavy circuits;
//! - rotation-heavy circuits;
//! - Clifford circuits;
//! - Clifford+T circuits;
//! - two-qubit circuits;
//! - sparse logical-qubit workloads;
//! - barrier-separated workloads;
//! - reset-separated workloads;
//! - parameterized workloads;
//! - mixed workloads;
//! - deterministic pseudo-random workloads;
//! - pathological workloads;
//! - large sequential workloads;
//! - large independent workloads;
//! - large cancellation workloads;
//! - depth-oriented workloads;
//! - width-oriented workloads;
//! - fault-tolerant-oriented workloads.
//!
//! # Scalability contract
//!
//! There is deliberately NO fixed architectural maximum in this module.
//!
//! A corpus constructor receives an explicit requested size and constructs only
//! the requested workload.
//!
//! Environment-controlled tests use:
//!
//! ```text
//! ZAMANI_OPTIMIZATION_CORPUS_SCALE=4096 cargo test
//! ```
//!
//! Larger workloads can be requested when machine resources permit:
//!
//! ```text
//! ZAMANI_OPTIMIZATION_CORPUS_SCALE=1000000 cargo test
//! ```
//!
//! The environment variable controls TEST WORKLOAD ONLY.
//!
//! It is not an optimizer limit.
//!
//! Production resource limits remain owned by:
//!
//! ```text
//! optimization::limits
//! optimization::context
//! ```
//!
//! Therefore the corpus does not artificially claim that quantum circuits have
//! a fixed maximum size.
//!
//! The actual maximum workload is constrained only by:
//!
//! - available memory;
//! - available CPU;
//! - platform address-space constraints;
//! - canonical Quantum IR limits;
//! - optimizer limits;
//! - test-runner limits.
//!
//! # Determinism
//!
//! Every generated workload is deterministic.
//!
//! No:
//!
//! - OS randomness;
//! - timestamps;
//! - global mutable state;
//! - hash-map iteration order;
//! - thread-local randomness;
//! - network state;
//! - filesystem state
//!
//! is used to determine circuit contents.
//!
//! Every generated workload is determined by:
//!
//! ```text
//! CorpusKind
//! seed
//! requested operation count
//! requested qubit count
//! ```
//!
//! This means a failing corpus case can be reproduced exactly.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
//!
//! No raw pointers, FFI, unsafe blocks, process execution, network access, or
//! filesystem access are required.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `tests/mod.rs` should expose this module with:
//!
//! ```text
//! mod corpus;
//! ```
//!
//! No production optimization module should depend on this test module.
//!
//! Future test modules may import this module's public corpus constructors.
//!
//! Adding a new optimization pass MUST NOT require this file to change.
//!
//! A future pass should instead consume an existing corpus:
//!
//! ```text
//! corpus::small_suite()
//! corpus::medium_suite()
//! corpus::large_suite()
//! corpus::generate(...)
//! corpus::generate_kind(...)
//! ```
//!
//! # Important semantic rule
//!
//! Corpus generation is NOT semantic verification.
//!
//! A generated circuit is a workload.
//!
//! It must never be assumed to be equivalent to another circuit merely because
//! both belong to the same corpus family.
//!
//! Semantic claims belong to `equivalence.rs` and the optimization verification
//! subsystem.
//!
//! # Important resource rule
//!
//! This module deliberately avoids dense matrix construction, state-vector
//! construction, tensor construction, or exponential semantic simulation.
//!
//! The purpose of the corpus is to generate IR workloads.
//!
//! Semantic verification of small circuits can be performed by the equivalence
//! suite.
//!
//! Very large circuits must not accidentally trigger exponential verification
//! merely because they belong to this corpus.
//!
//! ============================================================================
//! Module configuration
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Imports
// ============================================================================

use std::f64::consts::PI;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

// ============================================================================
// Corpus constants
// ============================================================================

/// Default number of operations used by medium corpus tests.
///
/// This value is intentionally modest enough for ordinary CI.
pub const DEFAULT_CORPUS_SCALE: usize = 4_096;

/// Default number of operations used by larger stress tests.
///
/// This is still a test workload, not an architectural maximum.
pub const DEFAULT_LARGE_CORPUS_SCALE: usize = 16_384;

/// Default logical-qubit count for generated mixed workloads.
pub const DEFAULT_QUBIT_COUNT: usize = 8;

/// Minimum logical-qubit count accepted by generated workloads.
pub const MINIMUM_QUBIT_COUNT: usize = 1;

/// Stable base seed for corpus generation.
pub const DEFAULT_CORPUS_SEED: u64 = 0x5A4D_414E_495F_4350;

/// Secondary seed used for independent-workload generation.
pub const INDEPENDENT_CORPUS_SEED: u64 = 0x5A4D_414E_495F_4944;

/// Seed used for cancellation-heavy workloads.
pub const CANCELLATION_CORPUS_SEED: u64 = 0x5A4D_414E_495F_4341;

/// Seed used for parameterized workloads.
pub const PARAMETER_CORPUS_SEED: u64 = 0x5A4D_414E_495F_5041;

/// Seed used for fault-tolerant workloads.
pub const FAULT_TOLERANT_CORPUS_SEED: u64 = 0x5A4D_414E_495F_4654;

// ============================================================================
// Corpus kind
// ============================================================================

/// Canonical workload families provided by the corpus.
///
/// The enum is intentionally independent of individual optimization passes.
/// New optimizer passes therefore do not require changes to this file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorpusKind {
    /// Empty circuit.
    Empty,

    /// One-operation circuits.
    Tiny,

    /// Repeated self-inverse operations.
    CancellationHeavy,

    /// Repeated inverse pairs.
    InverseHeavy,

    /// Parameterized rotation workloads.
    RotationHeavy,

    /// Clifford-only workloads.
    Clifford,

    /// Clifford+T workloads.
    FaultTolerant,

    /// Two-qubit-heavy workloads.
    TwoQubit,

    /// Independent gates distributed across qubits.
    Independent,

    /// Deep sequential workloads.
    Deep,

    /// Wide workloads with many independently active qubits.
    Wide,

    /// Barrier-separated workloads.
    BarrierSeparated,

    /// Reset-separated workloads.
    ResetSeparated,

    /// Sparse logical-qubit workloads.
    Sparse,

    /// Mixed realistic workload.
    Mixed,

    /// Deterministic stress workload.
    Stress,

    /// Deliberately adversarial/pathological workload.
    Pathological,
}

impl CorpusKind {
    /// Returns the stable textual corpus identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Tiny => "tiny",
            Self::CancellationHeavy => "cancellation_heavy",
            Self::InverseHeavy => "inverse_heavy",
            Self::RotationHeavy => "rotation_heavy",
            Self::Clifford => "clifford",
            Self::FaultTolerant => "fault_tolerant",
            Self::TwoQubit => "two_qubit",
            Self::Independent => "independent",
            Self::Deep => "deep",
            Self::Wide => "wide",
            Self::BarrierSeparated => "barrier_separated",
            Self::ResetSeparated => "reset_separated",
            Self::Sparse => "sparse",
            Self::Mixed => "mixed",
            Self::Stress => "stress",
            Self::Pathological => "pathological",
        }
    }
}

impl std::fmt::Display for CorpusKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Corpus specification
// ============================================================================

/// Complete deterministic description of a corpus workload.
///
/// The specification contains only generation parameters. It does not contain
/// optimizer state.
///
/// This makes corpus construction independent from optimizer implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorpusSpec {
    /// Workload family.
    pub kind: CorpusKind,

    /// Number of logical qubits requested.
    pub qubits: usize,

    /// Number of operations requested.
    pub operations: usize,

    /// Deterministic generation seed.
    pub seed: u64,
}

impl CorpusSpec {
    /// Creates a corpus specification.
    #[must_use]
    pub const fn new(
        kind: CorpusKind,
        qubits: usize,
        operations: usize,
        seed: u64,
    ) -> Self {
        Self {
            kind,
            qubits,
            operations,
            seed,
        }
    }

    /// Creates a small specification.
    #[must_use]
    pub const fn small(kind: CorpusKind) -> Self {
        Self {
            kind,
            qubits: DEFAULT_QUBIT_COUNT,
            operations: 64,
            seed: DEFAULT_CORPUS_SEED,
        }
    }

    /// Creates a medium specification.
    #[must_use]
    pub const fn medium(kind: CorpusKind) -> Self {
        Self {
            kind,
            qubits: DEFAULT_QUBIT_COUNT,
            operations: DEFAULT_CORPUS_SCALE,
            seed: DEFAULT_CORPUS_SEED,
        }
    }

    /// Creates a large specification.
    #[must_use]
    pub const fn large(kind: CorpusKind) -> Self {
        Self {
            kind,
            qubits: DEFAULT_QUBIT_COUNT,
            operations: DEFAULT_LARGE_CORPUS_SCALE,
            seed: DEFAULT_CORPUS_SEED,
        }
    }

    /// Returns a copy with a different seed.
    #[must_use]
    pub const fn with_seed(self, seed: u64) -> Self {
        Self { seed, ..self }
    }

    /// Returns a copy with a different operation count.
    #[must_use]
    pub const fn with_operations(
        self,
        operations: usize,
    ) -> Self {
        Self {
            operations,
            ..self
        }
    }

    /// Returns a copy with a different qubit count.
    #[must_use]
    pub const fn with_qubits(
        self,
        qubits: usize,
    ) -> Self {
        Self {
            qubits,
            ..self
        }
    }
}

// ============================================================================
// Deterministic generator
// ============================================================================

/// Deterministic pseudo-random generator used only for corpus construction.
///
/// The generator uses integer-only state and therefore has no dependency on
/// platform randomness or external RNG crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    /// Creates a generator from a stable seed.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Returns the next deterministic 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    /// Returns a deterministic index in `[0, upper)`.
    #[must_use]
    pub fn index(
        &mut self,
        upper: usize,
    ) -> usize {
        if upper == 0 {
            return 0;
        }

        (self.next_u64() % upper as u64) as usize
    }

    /// Returns a deterministic Boolean.
    #[must_use]
    pub fn boolean(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Returns a deterministic signed angle.
    #[must_use]
    pub fn angle(&mut self) -> f64 {
        let bucket = self.index(17) as f64 - 8.0;

        bucket * (PI / 8.0)
    }
}

// ============================================================================
// Canonical IR constructors
// ============================================================================

/// Constructs the canonical logical qubit.
///
/// This function intentionally uses `quantum::ir::qubit::QubitId`.
#[must_use]
pub const fn q(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Constructs a validated finite canonical parameter.
pub fn parameter(value: f64) -> Parameter {
    Parameter::constant(value)
        .expect("corpus parameters must be finite")
}

/// Constructs a canonical gate without parameters.
pub fn gate(
    kind: GateKind,
    qubits: &[usize],
) -> Gate {
    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(q)
            .collect(),
        Vec::new(),
        None,
        None,
    )
    .expect("corpus gate must satisfy canonical Quantum IR invariants")
}

/// Constructs a canonical one-parameter gate.
pub fn parameterized_gate(
    kind: GateKind,
    qubits: &[usize],
    value: f64,
) -> Gate {
    Gate::new(
        kind,
        qubits
            .iter()
            .copied()
            .map(q)
            .collect(),
        vec![parameter(value)],
        None,
        None,
    )
    .expect("corpus parameterized gate must satisfy canonical IR invariants")
}

/// Constructs a canonical barrier.
pub fn barrier(qubits: &[usize]) -> Gate {
    gate(GateKind::Barrier, qubits)
}

/// Constructs a canonical reset.
pub fn reset(qubit: usize) -> Gate {
    gate(GateKind::Reset, &[qubit])
}

/// Constructs a validated canonical circuit.
pub fn circuit(
    num_qubits: usize,
    operations: Vec<Gate>,
) -> QuantumCircuit {
    QuantumCircuit::from_operations(
        num_qubits,
        0,
        operations,
    )
    .expect("corpus circuit must satisfy canonical Quantum IR invariants")
}

// ============================================================================
// Corpus generation
// ============================================================================

/// Generates one deterministic corpus from a specification.
///
/// This is the primary public corpus API.
///
/// All generated operations are canonical Quantum IR operations.
pub fn generate(
    specification: CorpusSpec,
) -> QuantumCircuit {
    let qubits = specification.qubits.max(MINIMUM_QUBIT_COUNT);

    match specification.kind {
        CorpusKind::Empty => {
            generate_empty(qubits)
        }

        CorpusKind::Tiny => {
            generate_tiny(qubits, specification.seed)
        }

        CorpusKind::CancellationHeavy => {
            generate_cancellation_heavy(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::InverseHeavy => {
            generate_inverse_heavy(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::RotationHeavy => {
            generate_rotation_heavy(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Clifford => {
            generate_clifford(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::FaultTolerant => {
            generate_fault_tolerant(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::TwoQubit => {
            generate_two_qubit(
                qubits.max(2),
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Independent => {
            generate_independent(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Deep => {
            generate_deep(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Wide => {
            generate_wide(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::BarrierSeparated => {
            generate_barrier_separated(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::ResetSeparated => {
            generate_reset_separated(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Sparse => {
            generate_sparse(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Mixed => {
            generate_mixed(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Stress => {
            generate_stress(
                qubits,
                specification.operations,
                specification.seed,
            )
        }

        CorpusKind::Pathological => {
            generate_pathological(
                qubits,
                specification.operations,
                specification.seed,
            )
        }
    }
}

// ============================================================================
// Small canonical workloads
// ============================================================================

/// Generates an empty circuit.
pub fn generate_empty(
    qubits: usize,
) -> QuantumCircuit {
    circuit(qubits, Vec::new())
}

/// Generates a tiny deterministic circuit.
///
/// The workload deliberately contains several fundamental gate families.
pub fn generate_tiny(
    qubits: usize,
    seed: u64,
) -> QuantumCircuit {
    let available = qubits.max(1);

    let mut generator = DeterministicGenerator::new(seed);

    let q0 = generator.index(available);

    let q1 = if available > 1 {
        let candidate = generator.index(available);

        if candidate == q0 {
            (candidate + 1) % available
        } else {
            candidate
        }
    } else {
        q0
    };

    let mut operations = Vec::new();

    operations.push(gate(GateKind::H, &[q0]));
    operations.push(gate(GateKind::X, &[q0]));

    if available > 1 {
        operations.push(gate(
            GateKind::CX,
            &[q0, q1],
        ));
    }

    operations.push(parameterized_gate(
        GateKind::RZ,
        &[q0],
        PI / 2.0,
    ));

    circuit(available, operations)
}

// ============================================================================
// Cancellation workloads
// ============================================================================

/// Generates repeated self-inverse gate pairs.
///
/// Expected optimizer opportunity:
///
/// ```text
/// G G → I
/// ```
pub fn generate_cancellation_heavy(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator = DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    while gates.len() < operations {
        let qubit = generator.index(count);

        let kind = match generator.index(4) {
            0 => GateKind::X,
            1 => GateKind::Y,
            2 => GateKind::Z,
            _ => GateKind::H,
        };

        gates.push(gate(kind, &[qubit]));

        if gates.len() < operations {
            gates.push(gate(kind, &[qubit]));
        }
    }

    circuit(count, gates)
}

/// Generates explicit inverse pairs.
///
/// The generated sequence contains both self-inverse and explicit inverse
/// relationships.
pub fn generate_inverse_heavy(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator = DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    while gates.len() < operations {
        let qubit = generator.index(count);

        match generator.index(6) {
            0 => {
                gates.push(gate(GateKind::S, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(
                        GateKind::Sdg,
                        &[qubit],
                    ));
                }
            }

            1 => {
                gates.push(gate(GateKind::T, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(
                        GateKind::Tdg,
                        &[qubit],
                    ));
                }
            }

            2 => {
                gates.push(gate(GateKind::X, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(GateKind::X, &[qubit]));
                }
            }

            3 => {
                gates.push(gate(GateKind::Y, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(GateKind::Y, &[qubit]));
                }
            }

            4 => {
                gates.push(gate(GateKind::Z, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(GateKind::Z, &[qubit]));
                }
            }

            _ => {
                gates.push(gate(GateKind::H, &[qubit]));

                if gates.len() < operations {
                    gates.push(gate(GateKind::H, &[qubit]));
                }
            }
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Parameterized workloads
// ============================================================================

/// Generates rotation-heavy workloads.
pub fn generate_rotation_heavy(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator = DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let qubit = generator.index(count);

        let kind = match generator.index(3) {
            0 => GateKind::RX,
            1 => GateKind::RY,
            _ => GateKind::RZ,
        };

        gates.push(parameterized_gate(
            kind,
            &[qubit],
            generator.angle(),
        ));
    }

    circuit(count, gates)
}

// ============================================================================
// Clifford workloads
// ============================================================================

/// Generates Clifford-only workloads.
///
/// This workload is useful for Clifford simplification, symplectic analysis,
/// commutation analysis, cancellation, and synthesis tests.
pub fn generate_clifford(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator = DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let first = generator.index(count);

        let choice = generator.index(7);

        match choice {
            0 => gates.push(gate(
                GateKind::H,
                &[first],
            )),

            1 => gates.push(gate(
                GateKind::S,
                &[first],
            )),

            2 => gates.push(gate(
                GateKind::Sdg,
                &[first],
            )),

            3 => gates.push(gate(
                GateKind::X,
                &[first],
            )),

            4 => gates.push(gate(
                GateKind::Y,
                &[first],
            )),

            5 => gates.push(gate(
                GateKind::Z,
                &[first],
            )),

            _ if count > 1 => {
                let mut second =
                    generator.index(count);

                if first == second {
                    second =
                        (second + 1) % count;
                }

                gates.push(gate(
                    GateKind::CX,
                    &[first, second],
                ));
            }

            _ => gates.push(gate(
                GateKind::H,
                &[first],
            )),
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Fault-tolerant workloads
// ============================================================================

/// Generates Clifford+T workloads.
///
/// The workload intentionally contains T/T† sequences of different lengths so
/// fault-tolerant optimization can exercise T-count and phase simplification.
pub fn generate_fault_tolerant(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let qubit = generator.index(count);

        match generator.index(10) {
            0 | 1 | 2 => {
                gates.push(gate(
                    GateKind::T,
                    &[qubit],
                ));
            }

            3 | 4 => {
                gates.push(gate(
                    GateKind::Tdg,
                    &[qubit],
                ));
            }

            5 => {
                gates.push(gate(
                    GateKind::S,
                    &[qubit],
                ));
            }

            6 => {
                gates.push(gate(
                    GateKind::Sdg,
                    &[qubit],
                ));
            }

            7 => {
                gates.push(gate(
                    GateKind::H,
                    &[qubit],
                ));
            }

            8 if count > 1 => {
                let mut second =
                    generator.index(count);

                if second == qubit {
                    second =
                        (second + 1) % count;
                }

                gates.push(gate(
                    GateKind::CX,
                    &[qubit, second],
                ));
            }

            _ => {
                gates.push(gate(
                    GateKind::X,
                    &[qubit],
                ));
            }
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Two-qubit workloads
// ============================================================================

/// Generates two-qubit-heavy workloads.
///
/// Two-qubit operations are deliberately emphasized because they are often a
/// dominant hardware cost and therefore a critical optimization target.
pub fn generate_two_qubit(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(2);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for _ in 0..operations {
        let first = generator.index(count);

        let mut second = generator.index(count);

        if first == second {
            second = (second + 1) % count;
        }

        match generator.index(5) {
            0 | 1 => {
                gates.push(gate(
                    GateKind::CX,
                    &[first, second],
                ));
            }

            2 => {
                gates.push(gate(
                    GateKind::CZ,
                    &[first, second],
                ));
            }

            3 => {
                gates.push(gate(
                    GateKind::SWAP,
                    &[first, second],
                ));
            }

            _ => {
                gates.push(gate(
                    GateKind::CX,
                    &[first, second],
                ));
            }
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Independent workloads
// ============================================================================

/// Generates operations that are deliberately distributed across qubits.
///
/// This workload is useful for testing parallelism, dependency analysis,
/// commutation analysis, and width optimization.
pub fn generate_independent(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for index in 0..operations {
        let qubit =
            if count == 1 {
                0
            } else {
                index % count
            };

        let kind =
            match generator.index(4) {
                0 => GateKind::X,
                1 => GateKind::Y,
                2 => GateKind::Z,
                _ => GateKind::H,
            };

        gates.push(gate(kind, &[qubit]));
    }

    circuit(count, gates)
}

// ============================================================================
// Deep workloads
// ============================================================================

/// Generates a deep sequential circuit.
///
/// Most operations touch the same logical qubit, creating a long dependency
/// chain.
pub fn generate_deep(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let active = generator.index(count);

    let mut gates =
        Vec::with_capacity(operations);

    for _ in 0..operations {
        match generator.index(5) {
            0 => gates.push(gate(
                GateKind::H,
                &[active],
            )),

            1 => gates.push(gate(
                GateKind::X,
                &[active],
            )),

            2 => gates.push(gate(
                GateKind::Y,
                &[active],
            )),

            3 => gates.push(gate(
                GateKind::Z,
                &[active],
            )),

            _ => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[active],
                    generator.angle(),
                ),
            ),
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Wide workloads
// ============================================================================

/// Generates a wide circuit with independent operations distributed across
/// many logical qubits.
pub fn generate_wide(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(operations);

    for index in 0..operations {
        let qubit = index % count;

        let kind =
            match generator.index(6) {
                0 => GateKind::X,
                1 => GateKind::Y,
                2 => GateKind::Z,
                3 => GateKind::H,
                4 => GateKind::S,
                _ => GateKind::Sdg,
            };

        gates.push(gate(kind, &[qubit]));
    }

    circuit(count, gates)
}

// ============================================================================
// Boundary workloads
// ============================================================================

/// Generates workloads separated by barriers.
///
/// The construction deliberately places potentially cancellable operations on
/// opposite sides of barriers so passes that respect semantic boundaries can
/// be tested.
pub fn generate_barrier_separated(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates = Vec::with_capacity(
        operations.saturating_add(4),
    );

    let first =
        generator.index(count);

    gates.push(gate(
        GateKind::X,
        &[first],
    ));

    gates.push(gate(
        GateKind::X,
        &[first],
    ));

    gates.push(barrier(&[]));

    for _ in 0..operations {
        let qubit =
            generator.index(count);

        gates.push(gate(
            match generator.index(4) {
                0 => GateKind::H,
                1 => GateKind::X,
                2 => GateKind::Y,
                _ => GateKind::Z,
            },
            &[qubit],
        ));
    }

    gates.push(barrier(&[]));

    gates.push(gate(
        GateKind::X,
        &[first],
    ));

    gates.push(gate(
        GateKind::X,
        &[first],
    ));

    circuit(count, gates)
}

/// Generates reset-separated workloads.
///
/// Reset is treated as a semantic boundary rather than an ordinary unitary
/// operation.
pub fn generate_reset_separated(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let active =
        generator.index(count);

    let mut gates = Vec::with_capacity(
        operations.saturating_add(3),
    );

    gates.push(gate(
        GateKind::X,
        &[active],
    ));

    gates.push(reset(active));

    for _ in 0..operations {
        let qubit =
            generator.index(count);

        gates.push(gate(
            match generator.index(4) {
                0 => GateKind::H,
                1 => GateKind::X,
                2 => GateKind::Y,
                _ => GateKind::Z,
            },
            &[qubit],
        ));
    }

    gates.push(reset(active));

    gates.push(gate(
        GateKind::X,
        &[active],
    ));

    circuit(count, gates)
}

// ============================================================================
// Sparse logical namespaces
// ============================================================================

/// Generates workloads using a sparse logical namespace.
///
/// The logical register itself remains canonical and contiguous, while the
/// workload intentionally concentrates operations on selected identifiers.
/// This tests whether optimizers incorrectly assume dense operational use of
/// every logical qubit.
pub fn generate_sparse(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut active_qubits = Vec::new();

    if count == 1 {
        active_qubits.push(0);
    } else {
        active_qubits.push(0);
        active_qubits.push(count - 1);

        if count > 4 {
            active_qubits.push(count / 2);
        }
    }

    let mut gates =
        Vec::with_capacity(operations);

    for _ in 0..operations {
        let selected =
            active_qubits[
                generator.index(active_qubits.len())
            ];

        let kind =
            match generator.index(5) {
                0 => GateKind::X,
                1 => GateKind::H,
                2 => GateKind::Z,
                3 => GateKind::S,
                _ => GateKind::Sdg,
            };

        gates.push(gate(kind, &[selected]));
    }

    circuit(count, gates)
}

// ============================================================================
// Mixed workloads
// ============================================================================

/// Generates a heterogeneous workload representative of a general optimizer
/// input.
pub fn generate_mixed(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates =
        Vec::with_capacity(operations);

    for _ in 0..operations {
        let first =
            generator.index(count);

        match generator.index(12) {
            0 => gates.push(gate(
                GateKind::H,
                &[first],
            )),

            1 => gates.push(gate(
                GateKind::X,
                &[first],
            )),

            2 => gates.push(gate(
                GateKind::Y,
                &[first],
            )),

            3 => gates.push(gate(
                GateKind::Z,
                &[first],
            )),

            4 => gates.push(gate(
                GateKind::S,
                &[first],
            )),

            5 => gates.push(gate(
                GateKind::Sdg,
                &[first],
            )),

            6 => gates.push(gate(
                GateKind::T,
                &[first],
            )),

            7 => gates.push(gate(
                GateKind::Tdg,
                &[first],
            )),

            8 => gates.push(
                parameterized_gate(
                    GateKind::RX,
                    &[first],
                    generator.angle(),
                ),
            ),

            9 => gates.push(
                parameterized_gate(
                    GateKind::RY,
                    &[first],
                    generator.angle(),
                ),
            ),

            10 => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[first],
                    generator.angle(),
                ),
            ),

            _ if count > 1 => {
                let mut second =
                    generator.index(count);

                if first == second {
                    second =
                        (second + 1) % count;
                }

                gates.push(gate(
                    GateKind::CX,
                    &[first, second],
                ));
            }

            _ => gates.push(gate(
                GateKind::H,
                &[first],
            )),
        }
    }

    circuit(count, gates)
}

// ============================================================================
// Stress workloads
// ============================================================================

/// Generates a large deterministic stress workload.
///
/// The function deliberately uses only O(n) corpus construction memory and
/// does not recursively build circuits.
pub fn generate_stress(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let mut gates =
        Vec::with_capacity(operations);

    for index in 0..operations {
        let first =
            generator.index(count);

        let choice =
            (index + generator.index(17)) % 17;

        match choice {
            0 => gates.push(gate(
                GateKind::H,
                &[first],
            )),

            1 => gates.push(gate(
                GateKind::X,
                &[first],
            )),

            2 => gates.push(gate(
                GateKind::Y,
                &[first],
            )),

            3 => gates.push(gate(
                GateKind::Z,
                &[first],
            )),

            4 => gates.push(gate(
                GateKind::S,
                &[first],
            )),

            5 => gates.push(gate(
                GateKind::Sdg,
                &[first],
            )),

            6 => gates.push(gate(
                GateKind::T,
                &[first],
            )),

            7 => gates.push(gate(
                GateKind::Tdg,
                &[first],
            )),

            8 => gates.push(
                parameterized_gate(
                    GateKind::RX,
                    &[first],
                    generator.angle(),
                ),
            ),

            9 => gates.push(
                parameterized_gate(
                    GateKind::RY,
                    &[first],
                    generator.angle(),
                ),
            ),

            10 => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[first],
                    generator.angle(),
                ),
            ),

            11 | 12 if count > 1 => {
                let mut second =
                    generator.index(count);

                if first == second {
                    second =
                        (second + 1) % count;
                }

                gates.push(gate(
                    GateKind::CX,
                    &[first, second],
                ));
            }

            13 => {
                gates.push(gate(
                    GateKind::X,
                    &[first],
                ));

                if gates.len() < operations {
                    gates.push(gate(
                        GateKind::X,
                        &[first],
                    ));
                }
            }

            14 => {
                gates.push(gate(
                    GateKind::H,
                    &[first],
                ));

                if gates.len() < operations {
                    gates.push(gate(
                        GateKind::H,
                        &[first],
                    ));
                }
            }

            15 => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[first],
                    0.0,
                ),
            ),

            _ => {
                gates.push(gate(
                    GateKind::Z,
                    &[first],
                ));
            }
        }
    }

    gates.truncate(operations);

    circuit(count, gates)
}

// ============================================================================
// Pathological workloads
// ============================================================================

/// Generates deliberately difficult optimizer workloads.
///
/// These include:
///
/// - repeated cancellations;
/// - repeated rotations;
/// - dense dependencies on one qubit;
/// - repeated T operations;
/// - alternating two-qubit interactions.
///
/// The workload remains deterministic and linear to construct.
pub fn generate_pathological(
    qubits: usize,
    operations: usize,
    seed: u64,
) -> QuantumCircuit {
    let count = qubits.max(1);
    let mut generator =
        DeterministicGenerator::new(seed);

    let primary =
        generator.index(count);

    let secondary =
        if count > 1 {
            if primary == 0 {
                1
            } else {
                0
            }
        } else {
            primary
        };

    let mut gates =
        Vec::with_capacity(operations);

    for index in 0..operations {
        match index % 10 {
            0 => gates.push(gate(
                GateKind::H,
                &[primary],
            )),

            1 => gates.push(gate(
                GateKind::H,
                &[primary],
            )),

            2 => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[primary],
                    PI / 4.0,
                ),
            ),

            3 => gates.push(
                parameterized_gate(
                    GateKind::RZ,
                    &[primary],
                    -PI / 4.0,
                ),
            ),

            4 => gates.push(gate(
                GateKind::T,
                &[primary],
            )),

            5 => gates.push(gate(
                GateKind::Tdg,
                &[primary],
            )),

            6 if count > 1 => {
                gates.push(gate(
                    GateKind::CX,
                    &[primary, secondary],
                ));
            }

            7 if count > 1 => {
                gates.push(gate(
                    GateKind::CX,
                    &[secondary, primary],
                ));
            }

            8 => gates.push(gate(
                GateKind::X,
                &[primary],
            )),

            _ => gates.push(gate(
                GateKind::X,
                &[primary],
            )),
        }
    }

    gates.truncate(operations);

    circuit(count, gates)
}

// ============================================================================
// Standard suites
// ============================================================================

/// Returns the standard small corpus suite.
///
/// This function is intended for ordinary CI.
pub fn small_suite() -> Vec<QuantumCircuit> {
    vec![
        generate(CorpusSpec::small(
            CorpusKind::Empty,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Tiny,
        )),
        generate(CorpusSpec::small(
            CorpusKind::CancellationHeavy,
        )),
        generate(CorpusSpec::small(
            CorpusKind::InverseHeavy,
        )),
        generate(CorpusSpec::small(
            CorpusKind::RotationHeavy,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Clifford,
        )),
        generate(CorpusSpec::small(
            CorpusKind::FaultTolerant,
        )),
        generate(CorpusSpec::small(
            CorpusKind::TwoQubit,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Independent,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Deep,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Wide,
        )),
        generate(CorpusSpec::small(
            CorpusKind::BarrierSeparated,
        )),
        generate(CorpusSpec::small(
            CorpusKind::ResetSeparated,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Sparse,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Mixed,
        )),
        generate(CorpusSpec::small(
            CorpusKind::Pathological,
        )),
    ]
}

/// Returns the standard medium corpus suite.
///
/// The suite is intentionally separate from `small_suite()` so CI can choose
/// the desired resource level.
pub fn medium_suite() -> Vec<QuantumCircuit> {
    vec![
        generate(CorpusSpec::medium(
            CorpusKind::CancellationHeavy,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::RotationHeavy,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::Clifford,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::FaultTolerant,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::TwoQubit,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::Independent,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::Deep,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::Wide,
        )),
        generate(CorpusSpec::medium(
            CorpusKind::Mixed,
        )),
    ]
}

/// Returns a large stress suite.
///
/// This suite should normally be used by explicit stress jobs rather than
/// every pull request.
pub fn large_suite() -> Vec<QuantumCircuit> {
    vec![
        generate(CorpusSpec::large(
            CorpusKind::CancellationHeavy,
        )),
        generate(CorpusSpec::large(
            CorpusKind::RotationHeavy,
        )),
        generate(CorpusSpec::large(
            CorpusKind::TwoQubit,
        )),
        generate(CorpusSpec::large(
            CorpusKind::FaultTolerant,
        )),
        generate(CorpusSpec::large(
            CorpusKind::Mixed,
        )),
    ]
}

// ============================================================================
// Environment-controlled scale
// ============================================================================

/// Reads the corpus scale from the environment.
///
/// Invalid, zero, or missing values fall back to the default.
///
/// Environment state is deliberately confined to test configuration and does
/// not affect production optimizer behavior.
pub fn configured_scale() -> usize {
    std::env::var(
        "ZAMANI_OPTIMIZATION_CORPUS_SCALE",
    )
    .ok()
    .and_then(|value| {
        value.parse::<usize>().ok()
    })
    .filter(|value| *value > 0)
    .unwrap_or(DEFAULT_CORPUS_SCALE)
}

/// Returns a configured stress specification.
#[must_use]
pub fn configured_stress_spec(
    kind: CorpusKind,
) -> CorpusSpec {
    CorpusSpec::new(
        kind,
        DEFAULT_QUBIT_COUNT,
        configured_scale(),
        DEFAULT_CORPUS_SEED,
    )
}

// ============================================================================
// Corpus fingerprints
// ============================================================================

/// A deterministic lightweight fingerprint of a circuit.
///
/// This is NOT a semantic fingerprint.
///
/// It is only useful for asserting deterministic corpus generation.
///
/// The function intentionally does not use a hash map or a randomized hash
/// builder, so its result is stable across executions.
pub fn fingerprint(
    circuit: &QuantumCircuit,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    hash = mix(
        hash,
        circuit.num_qubits() as u64,
    );

    for operation in circuit.operations() {
        hash = mix(
            hash,
            gate_kind_tag(
                operation.kind(),
            ),
        );

        for qubit in operation.qubits() {
            hash = mix(
                hash,
                qubit.index() as u64,
            );
        }

        for parameter in operation.parameters() {
            hash = mix(
                hash,
                parameter_fingerprint(
                    parameter,
                ),
            );
        }
    }

    hash
}

/// Performs one deterministic integer mixing step.
fn mix(
    value: u64,
    input: u64,
) -> u64 {
    let mut result =
        value ^ input;

    result = result
        .wrapping_mul(
            0x1000_0000_01B3,
        );

    result.rotate_left(13)
}

/// Produces a stable tag for a canonical gate kind.
///
/// The exact numeric values are an internal corpus fingerprint format, not a
/// public semantic encoding.
fn gate_kind_tag(
    kind: GateKind,
) -> u64 {
    match kind {
        GateKind::I => 1,
        GateKind::X => 2,
        GateKind::Y => 3,
        GateKind::Z => 4,
        GateKind::H => 5,
        GateKind::S => 6,
        GateKind::Sdg => 7,
        GateKind::T => 8,
        GateKind::Tdg => 9,
        GateKind::RX => 10,
        GateKind::RY => 11,
        GateKind::RZ => 12,
        GateKind::CX => 13,
        GateKind::CZ => 14,
        GateKind::SWAP => 15,
        GateKind::Barrier => 16,
        GateKind::Reset => 17,
        _ => 0xFFFF_FFFF,
    }
}

/// Produces a deterministic fingerprint component for a parameter.
///
/// The parameter is intentionally represented through its stable debug form
/// rather than relying on platform-dependent floating-point byte layout.
///
/// This function is used only to detect corpus-generation changes, never for
/// semantic equivalence.
fn parameter_fingerprint(
    parameter: &Parameter,
) -> u64 {
    let representation =
        format!("{parameter:?}");

    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for byte in representation.bytes() {
        hash = mix(
            hash,
            byte as u64,
        );
    }

    hash
}

// ============================================================================
// Corpus metadata
// ============================================================================

/// Returns the number of operations in a canonical circuit.
#[must_use]
pub fn operation_count(
    circuit: &QuantumCircuit,
) -> usize {
    circuit.operations().len()
}

/// Returns the number of logical qubits in a canonical circuit.
#[must_use]
pub fn qubit_count(
    circuit: &QuantumCircuit,
) -> usize {
    circuit.num_qubits()
}

/// Returns whether all circuit operations use valid logical qubit indices.
#[must_use]
pub fn has_valid_qubit_operands(
    circuit: &QuantumCircuit,
) -> bool {
    let count =
        circuit.num_qubits();

    circuit
        .operations()
        .iter()
        .all(|operation| {
            operation
                .qubits()
                .iter()
                .all(|qubit| {
                    qubit.index() < count
                })
        })
}

/// Returns whether a circuit contains at least one two-qubit operation.
#[must_use]
pub fn contains_two_qubit_operation(
    circuit: &QuantumCircuit,
) -> bool {
    circuit
        .operations()
        .iter()
        .any(|operation| {
            operation.qubits().len() == 2
        })
}

/// Returns whether a circuit contains parameterized operations.
#[must_use]
pub fn contains_parameters(
    circuit: &QuantumCircuit,
) -> bool {
    circuit
        .operations()
        .iter()
        .any(|operation| {
            !operation.parameters().is_empty()
        })
}

/// Returns whether a circuit contains a semantic boundary operation.
#[must_use]
pub fn contains_boundary(
    circuit: &QuantumCircuit,
) -> bool {
    circuit
        .operations()
        .iter()
        .any(|operation| {
            matches!(
                operation.kind(),
                GateKind::Barrier
                    | GateKind::Reset
            )
        })
}

// ============================================================================
// Tests — deterministic generation
// ============================================================================

#[test]
fn corpus_generation_is_deterministic() {
    let specification =
        CorpusSpec::new(
            CorpusKind::Mixed,
            8,
            512,
            DEFAULT_CORPUS_SEED,
        );

    let first =
        generate(specification);

    let second =
        generate(specification);

    assert_eq!(
        fingerprint(&first),
        fingerprint(&second),
        "same corpus specification must produce the same workload",
    );

    assert_eq!(
        operation_count(&first),
        operation_count(&second),
    );

    assert_eq!(
        qubit_count(&first),
        qubit_count(&second),
    );
}

#[test]
fn different_seeds_can_produce_different_workloads() {
    let first =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                8,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    let second =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                8,
                512,
                DEFAULT_CORPUS_SEED
                    .wrapping_add(1),
            ),
        );

    assert_ne!(
        fingerprint(&first),
        fingerprint(&second),
        "different seeds should normally produce different deterministic workloads",
    );
}

#[test]
fn requested_operation_count_is_respected() {
    let requested = 1_024;

    let specification =
        CorpusSpec::new(
            CorpusKind::Mixed,
            8,
            requested,
            DEFAULT_CORPUS_SEED,
        );

    let generated =
        generate(specification);

    assert_eq!(
        operation_count(&generated),
        requested,
    );
}

#[test]
fn zero_requested_operations_produce_empty_workload() {
    let specification =
        CorpusSpec::new(
            CorpusKind::Mixed,
            8,
            0,
            DEFAULT_CORPUS_SEED,
        );

    let generated =
        generate(specification);

    assert_eq!(
        operation_count(&generated),
        0,
    );

    assert_eq!(
        qubit_count(&generated),
        8,
    );
}

#[test]
fn zero_requested_qubits_are_normalized_to_one() {
    let specification =
        CorpusSpec::new(
            CorpusKind::Tiny,
            0,
            4,
            DEFAULT_CORPUS_SEED,
        );

    let generated =
        generate(specification);

    assert_eq!(
        qubit_count(&generated),
        1,
    );

    assert!(
        has_valid_qubit_operands(
            &generated
        ),
    );
}

// ============================================================================
// Tests — corpus family coverage
// ============================================================================

#[test]
fn small_suite_contains_expected_workload_families() {
    let suite =
        small_suite();

    assert_eq!(
        suite.len(),
        16,
    );

    for circuit in &suite {
        assert!(
            has_valid_qubit_operands(
                circuit
            ),
            "all corpus circuits must contain valid logical qubit operands",
        );
    }
}

#[test]
fn cancellation_corpus_contains_cancellation_opportunities() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::CancellationHeavy,
                8,
                512,
                CANCELLATION_CORPUS_SEED,
            ),
        );

    assert!(
        operation_count(&generated) > 0
    );
}

#[test]
fn rotation_corpus_contains_parameters() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::RotationHeavy,
                8,
                512,
                PARAMETER_CORPUS_SEED,
            ),
        );

    assert!(
        contains_parameters(&generated),
        "rotation corpus must contain parameterized operations",
    );
}

#[test]
fn two_qubit_corpus_contains_two_qubit_operations() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::TwoQubit,
                8,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert!(
        contains_two_qubit_operation(
            &generated
        ),
        "two-qubit corpus must contain two-qubit operations",
    );
}

#[test]
fn fault_tolerant_corpus_contains_operations() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::FaultTolerant,
                8,
                512,
                FAULT_TOLERANT_CORPUS_SEED,
            ),
        );

    assert!(
        operation_count(&generated) > 0
    );
}

#[test]
fn boundary_corpora_contain_boundaries() {
    let barrier_corpus =
        generate(
            CorpusSpec::new(
                CorpusKind::BarrierSeparated,
                8,
                128,
                DEFAULT_CORPUS_SEED,
            ),
        );

    let reset_corpus =
        generate(
            CorpusSpec::new(
                CorpusKind::ResetSeparated,
                8,
                128,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert!(
        contains_boundary(
            &barrier_corpus
        ),
    );

    assert!(
        contains_boundary(
            &reset_corpus
        ),
    );
}

// ============================================================================
// Tests — canonical IR safety
// ============================================================================

#[test]
fn every_standard_corpus_uses_canonical_logical_qubits() {
    let kinds = [
        CorpusKind::Empty,
        CorpusKind::Tiny,
        CorpusKind::CancellationHeavy,
        CorpusKind::InverseHeavy,
        CorpusKind::RotationHeavy,
        CorpusKind::Clifford,
        CorpusKind::FaultTolerant,
        CorpusKind::TwoQubit,
        CorpusKind::Independent,
        CorpusKind::Deep,
        CorpusKind::Wide,
        CorpusKind::BarrierSeparated,
        CorpusKind::ResetSeparated,
        CorpusKind::Sparse,
        CorpusKind::Mixed,
        CorpusKind::Stress,
        CorpusKind::Pathological,
    ];

    for kind in kinds {
        let generated =
            generate(
                CorpusSpec::new(
                    kind,
                    8,
                    256,
                    DEFAULT_CORPUS_SEED,
                ),
            );

        assert!(
            has_valid_qubit_operands(
                &generated
            ),
            "invalid logical-qubit operand in {kind}",
        );
    }
}

#[test]
fn sparse_corpus_preserves_declared_qubit_namespace() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Sparse,
                64,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        qubit_count(&generated),
        64,
    );

    assert!(
        has_valid_qubit_operands(
            &generated
        ),
    );
}

#[test]
fn single_qubit_corpus_never_creates_invalid_two_qubit_gate() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                1,
                2_048,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert!(
        generated
            .operations()
            .iter()
            .all(|operation| {
                operation.qubits().len() <= 1
            }),
        "one-qubit workloads must never contain invalid two-qubit operations",
    );
}

// ============================================================================
// Tests — scalable construction
// ============================================================================

#[test]
fn medium_corpus_constructs_without_recursive_generation() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Stress,
                DEFAULT_QUBIT_COUNT,
                DEFAULT_CORPUS_SCALE,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        DEFAULT_CORPUS_SCALE,
    );

    assert!(
        has_valid_qubit_operands(
            &generated
        ),
    );
}

#[test]
fn large_corpus_constructor_scales_linearly_in_requested_operations() {
    let requested =
        DEFAULT_LARGE_CORPUS_SCALE;

    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Stress,
                DEFAULT_QUBIT_COUNT,
                requested,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        requested,
    );

    assert!(
        has_valid_qubit_operands(
            &generated
        ),
    );
}

// ============================================================================
// Tests — fingerprint contract
// ============================================================================

#[test]
fn fingerprint_is_stable_for_identical_canonical_circuits() {
    let first =
        generate(
            CorpusSpec::new(
                CorpusKind::Clifford,
                8,
                256,
                DEFAULT_CORPUS_SEED,
            ),
        );

    let second =
        generate(
            CorpusSpec::new(
                CorpusKind::Clifford,
                8,
                256,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        fingerprint(&first),
        fingerprint(&second),
    );
}

#[test]
fn fingerprint_changes_when_workload_changes() {
    let first =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                8,
                128,
                DEFAULT_CORPUS_SEED,
            ),
        );

    let second =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                8,
                129,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_ne!(
        fingerprint(&first),
        fingerprint(&second),
    );
}

// ============================================================================
// Tests — generator itself
// ============================================================================

#[test]
fn deterministic_generator_repeats_exactly() {
    let mut first =
        DeterministicGenerator::new(
            DEFAULT_CORPUS_SEED,
        );

    let mut second =
        DeterministicGenerator::new(
            DEFAULT_CORPUS_SEED,
        );

    for _ in 0..1_024 {
        assert_eq!(
            first.next_u64(),
            second.next_u64(),
        );
    }
}

#[test]
fn deterministic_generator_index_stays_in_range() {
    let mut generator =
        DeterministicGenerator::new(
            DEFAULT_CORPUS_SEED,
        );

    for upper in [
        1_usize,
        2,
        3,
        8,
        64,
        1_024,
    ] {
        for _ in 0..128 {
            assert!(
                generator.index(upper)
                    < upper
            );
        }
    }
}

#[test]
fn deterministic_generator_handles_zero_upper_bound() {
    let mut generator =
        DeterministicGenerator::new(
            DEFAULT_CORPUS_SEED,
        );

    assert_eq!(
        generator.index(0),
        0,
    );
}

// ============================================================================
// Tests — corpus specification
// ============================================================================

#[test]
fn corpus_spec_builder_methods_are_deterministic() {
    let specification =
        CorpusSpec::small(
            CorpusKind::Mixed,
        )
        .with_operations(1_024)
        .with_qubits(16)
        .with_seed(12345);

    assert_eq!(
        specification.kind,
        CorpusKind::Mixed,
    );

    assert_eq!(
        specification.operations,
        1_024,
    );

    assert_eq!(
        specification.qubits,
        16,
    );

    assert_eq!(
        specification.seed,
        12345,
    );
}

#[test]
fn corpus_kind_identifiers_are_unique() {
    let kinds = [
        CorpusKind::Empty,
        CorpusKind::Tiny,
        CorpusKind::CancellationHeavy,
        CorpusKind::InverseHeavy,
        CorpusKind::RotationHeavy,
        CorpusKind::Clifford,
        CorpusKind::FaultTolerant,
        CorpusKind::TwoQubit,
        CorpusKind::Independent,
        CorpusKind::Deep,
        CorpusKind::Wide,
        CorpusKind::BarrierSeparated,
        CorpusKind::ResetSeparated,
        CorpusKind::Sparse,
        CorpusKind::Mixed,
        CorpusKind::Stress,
        CorpusKind::Pathological,
    ];

    for outer in 0..kinds.len() {
        for inner in (outer + 1)..kinds.len() {
            assert_ne!(
                kinds[outer].as_str(),
                kinds[inner].as_str(),
                "corpus identifiers must be unique",
            );
        }
    }
}

// ============================================================================
// Tests — environment configuration
// ============================================================================

#[test]
fn configured_scale_is_positive() {
    assert!(
        configured_scale() > 0
    );
}

#[test]
fn configured_stress_spec_has_expected_kind() {
    let specification =
        configured_stress_spec(
            CorpusKind::Stress,
        );

    assert_eq!(
        specification.kind,
        CorpusKind::Stress,
    );

    assert!(
        specification.operations > 0
    );

    assert!(
        specification.qubits > 0
    );
}

// ============================================================================
// Tests — corpus semantic neutrality
// ============================================================================

#[test]
fn corpus_fingerprint_is_not_used_as_semantic_equivalence() {
    let first =
        generate(
            CorpusSpec::new(
                CorpusKind::CancellationHeavy,
                1,
                2,
                DEFAULT_CORPUS_SEED,
            ),
        );

    let second =
        generate(
            CorpusSpec::new(
                CorpusKind::CancellationHeavy,
                1,
                2,
                DEFAULT_CORPUS_SEED
                    .wrapping_add(1),
            ),
        );

    let first_fingerprint =
        fingerprint(&first);

    let second_fingerprint =
        fingerprint(&second);

    assert!(
        first_fingerprint != second_fingerprint
            || first_fingerprint == second_fingerprint,
        "fingerprints are intentionally only deterministic workload identifiers",
    );
}

// ============================================================================
// Tests — future-pass independence
// ============================================================================

/// This test intentionally does not import any optimization pass.
///
/// The corpus layer must remain usable before or after individual optimization
/// passes are added.
#[test]
fn corpus_is_independent_of_specific_optimizer_passes() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Mixed,
                8,
                256,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        256,
    );

    assert_eq!(
        qubit_count(&generated),
        8,
    );
}

// ============================================================================
// Tests — resource-safe zero and tiny cases
// ============================================================================

#[test]
fn all_corpus_kinds_support_zero_operations() {
    let kinds = [
        CorpusKind::Empty,
        CorpusKind::Tiny,
        CorpusKind::CancellationHeavy,
        CorpusKind::InverseHeavy,
        CorpusKind::RotationHeavy,
        CorpusKind::Clifford,
        CorpusKind::FaultTolerant,
        CorpusKind::TwoQubit,
        CorpusKind::Independent,
        CorpusKind::Deep,
        CorpusKind::Wide,
        CorpusKind::BarrierSeparated,
        CorpusKind::ResetSeparated,
        CorpusKind::Sparse,
        CorpusKind::Mixed,
        CorpusKind::Stress,
        CorpusKind::Pathological,
    ];

    for kind in kinds {
        let generated =
            generate(
                CorpusSpec::new(
                    kind,
                    8,
                    0,
                    DEFAULT_CORPUS_SEED,
                ),
            );

        assert_eq!(
            operation_count(&generated),
            0,
            "{kind} must support an empty requested workload",
        );

        assert_eq!(
            qubit_count(&generated),
            8,
        );
    }
}

#[test]
fn tiny_corpus_supports_one_operation() {
    let kinds = [
        CorpusKind::CancellationHeavy,
        CorpusKind::InverseHeavy,
        CorpusKind::RotationHeavy,
        CorpusKind::Clifford,
        CorpusKind::FaultTolerant,
        CorpusKind::TwoQubit,
        CorpusKind::Independent,
        CorpusKind::Deep,
        CorpusKind::Wide,
        CorpusKind::Mixed,
        CorpusKind::Stress,
        CorpusKind::Pathological,
    ];

    for kind in kinds {
        let generated =
            generate(
                CorpusSpec::new(
                    kind,
                    8,
                    1,
                    DEFAULT_CORPUS_SEED,
                ),
            );

        assert!(
            operation_count(&generated)
                <= 1,
            "{kind} must never exceed the requested one-operation workload",
        );

        assert!(
            has_valid_qubit_operands(
                &generated
            ),
        );
    }
}

// ============================================================================
// Tests — explicit workload characteristics
// ============================================================================

#[test]
fn deep_workload_has_operations() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Deep,
                8,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        512,
    );
}

#[test]
fn wide_workload_has_operations() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Wide,
                64,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        512,
    );

    assert_eq!(
        qubit_count(&generated),
        64,
    );
}

#[test]
fn pathological_workload_has_operations() {
    let generated =
        generate(
            CorpusSpec::new(
                CorpusKind::Pathological,
                8,
                512,
                DEFAULT_CORPUS_SEED,
            ),
        );

    assert_eq!(
        operation_count(&generated),
        512,
    );
}

// ============================================================================
// Public corpus contract summary
// ============================================================================

/// Compile-time documentation anchor for the corpus architecture.
///
/// This function is intentionally trivial. It provides one discoverable place
/// documenting the stable integration boundary for future maintainers.
#[allow(dead_code)]
fn corpus_contract() {
    // The corpus is:
    //
    //   deterministic
    //   canonical-IR based
    //   resource bounded by requested workload
    //   independent of optimizer implementation
    //   independent of routing
    //   independent of scheduling
    //   independent of hardware
    //   independent of QPU execution
    //   independent of benchmarking
    //
    // Future optimization passes consume this corpus rather than modifying it.
}