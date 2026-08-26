//! Zamani Quantum Benchmarking — Random Circuit Generation
//!
//! Production random logical-circuit generation for the Zamani quantum
//! benchmarking subsystem.
//!
//! # Purpose
//!
//! This module generates reproducible randomized logical quantum circuits for
//! benchmark families such as:
//!
//! - random circuit sampling;
//! - cross-entropy benchmarking;
//! - volumetric random-circuit benchmarks;
//! - Quantum Volume support;
//! - direct randomized benchmarking;
//! - randomized compilation experiments;
//! - mirror-circuit preparation;
//! - scalable random-circuit regression workloads;
//! - simulator stress workloads;
//! - benchmark fixtures.
//!
//! The generator produces canonical `quantum::ir::QuantumCircuit` values and,
//! when requested, wraps them in the benchmarking `BenchmarkCircuit` contract.
//!
//! # Architectural boundary
//!
//! ```text
//! RandomCircuitConfig
//!        │
//!        ▼
//! generators::random
//!        │
//!        ▼
//! generators::random_circuits
//!        │
//!        ├──► QuantumCircuit
//!        │
//!        └──► BenchmarkCircuit
//!                │
//!                ├── execution
//!                ├── statistics
//!                ├── metrics
//!                └── protocols
//! ```
//!
//! This module deliberately does NOT:
//!
//! - execute circuits;
//! - select a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform optimization;
//! - perform calibration;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - calculate XEB;
//! - calculate Quantum Volume;
//! - calculate fidelity;
//! - calculate benchmark results;
//! - depend on a simulator;
//! - depend on a particular quantum technology.
//!
//! Those concerns belong to downstream subsystems.
//!
//! # Logical versus physical topology
//!
//! Random circuits are generated in the canonical logical Quantum IR.
//!
//! A random two-qubit operation may therefore initially target any pair of
//! logical qubits. This is intentional. Backend routing must later translate
//! the logical workload to physical topology.
//!
//! The generator MUST NOT inspect hardware topology or perform routing.
//!
//! This preserves the dependency direction:
//!
//! ```text
//! random_circuits
//!       │
//!       ▼
//! Quantum IR
//!       │
//!       ▼
//! routing / scheduling / hardware
//! ```
//!
//! # Reproducibility
//!
//! Randomness is provided exclusively by `generators::random::RandomStream`.
//!
//! For a fixed:
//!
//! - `RandomCircuitConfig`;
//! - benchmark seed;
//! - random algorithm version;
//! - generator revision;
//! - Quantum IR implementation;
//!
//! the generated logical circuit is reproducible.
//!
//! No:
//!
//! - global RNG;
//! - `thread_rng()`;
//! - system clock;
//! - filesystem;
//! - environment variable;
//! - network;
//! - hardware metadata;
//! - unordered collection iteration;
//!
//! is used to determine circuit structure.
//!
//! # Domain separation
//!
//! Each generated circuit receives an independent child stream derived from
//! the root stream and circuit sequence index.
//!
//! This means circuit `0`, circuit `1`, circuit `2`, ... do not consume a
//! shared mutable random stream. This is important for deterministic parallel
//! benchmark execution and replay.
//!
//! # Layer semantics
//!
//! A generated circuit consists of logical layers.
//!
//! Single-qubit layers contain at most one operation per logical qubit.
//!
//! Two-qubit layers first create a deterministic random permutation of the
//! logical qubits and then pair adjacent elements. Consequently:
//!
//! - no logical qubit is used twice in the same layer;
//! - every two-qubit gate has two distinct operands;
//! - the pairing is independent of physical topology;
//! - the generated layer is a valid disjoint two-qubit layer.
//!
//! If the number of qubits is odd, the unpaired logical qubit receives a
//! single-qubit operation so that the layer can still exercise the complete
//! logical register.
//!
//! # Gate sets
//!
//! The generator supports both fixed and continuously-parameterized logical
//! gate families.
//!
//! Single-qubit gates:
//!
//! - I
//! - X
//! - Y
//! - Z
//! - H
//! - S
//! - Sdg
//! - T
//! - Tdg
//! - V
//! - Vdg
//! - RX
//! - RY
//! - RZ
//! - Phase
//! - U1
//! - U2
//! - U3
//!
//! Two-qubit gates:
//!
//! - CX
//! - CY
//! - CZ
//! - CH
//! - SWAP
//! - ISWAP
//! - ECR
//! - CRX
//! - CRY
//! - CRZ
//!
//! Parameterized gates receive explicit finite random parameters in radians.
//! No backend-specific angle conventions are introduced here.
//!
//! # Security/resource safety
//!
//! Every public generation entry point validates:
//!
//! - qubit count;
//! - depth;
//! - estimated gate count;
//! - estimated two-qubit gate count;
//! - configured gate-set non-emptiness;
//! - benchmark identifier bounds;
//! - experiment identifier bounds;
//! - case identifier bounds;
//! - probability/percentage configuration;
//! - benchmark limits;
//! - arithmetic overflow.
//!
//! No unbounded allocation is performed before resource validation.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! - `core::circuit`;
//! - `core::limits`;
//! - `generators::random`;
//! - canonical `quantum::ir`.
//!
//! It does NOT depend on:
//!
//! - execution;
//! - statistics;
//! - metrics;
//! - protocols;
//! - hardware;
//! - runtime;
//! - frontend;
//! - algorithms;
//! - routing;
//! - scheduling;
//! - optimization.
//!
//! Future protocol modules should consume `RandomCircuitGenerator` rather than
//! implementing their own random-circuit construction logic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! No additional dependencies are required beyond dependencies already used
//! by the Zamani benchmarking random subsystem.

use std::fmt;

use crate::quantum::benchmarking::core::circuit::{
    BenchmarkCircuit,
    BenchmarkCircuitDescriptor,
    BenchmarkCircuitError,
    BenchmarkCircuitGeneration,
    BenchmarkCircuitRole,
};

use crate::quantum::benchmarking::core::limits::{
    BenchmarkLimits,
    LimitError,
};

use crate::quantum::benchmarking::generators::random::{
    BenchmarkSeed,
    RandomError,
    RandomStream,
    RANDOM_ALGORITHM_ID,
};

use crate::quantum::ir::{
    Gate,
    GateKind,
    Parameter,
    QubitId,
    QuantumCircuit,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable random-circuit generator algorithm identifier.
///
/// This identifier is part of benchmark provenance. Any semantically
/// incompatible change to the generation algorithm requires a new identifier.
pub const RANDOM_CIRCUIT_GENERATOR_ALGORITHM_ID: &str =
    "zamani.benchmarking.random-circuits.v1";

/// Current random-circuit generator revision.
///
/// This value is independent of the RNG algorithm version.
pub const RANDOM_CIRCUIT_GENERATOR_REVISION: u32 = 1;

/// Maximum UTF-8 byte length for benchmark identifiers accepted by this
/// generator.
///
/// The benchmarking circuit descriptor also validates identifiers, but
/// validating before descriptor construction avoids unnecessary allocation or
/// work at the boundary.
pub const MAX_IDENTIFIER_LENGTH: usize = 4_096;

/// Full angular period used for uniformly randomized gate parameters.
pub const TWO_PI: f64 = std::f64::consts::PI * 2.0;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by random-circuit generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RandomCircuitGeneratorError {
    /// The requested configuration is invalid.
    InvalidConfiguration {
        /// Configuration field that failed validation.
        field: &'static str,

        /// Static explanation.
        reason: &'static str,
    },

    /// A configured benchmark limit was exceeded.
    Limit(LimitError),

    /// Random-stream generation failed.
    Random(RandomError),

    /// Arithmetic required for resource estimation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        operation: &'static str,
    },

    /// A Quantum IR gate could not be constructed.
    GateConstruction {
        /// Gate construction error rendered from the canonical IR error.
        message: String,
    },

    /// A Quantum IR circuit could not be constructed.
    CircuitConstruction {
        /// Circuit construction error rendered from the canonical IR error.
        message: String,
    },

    /// Benchmark metadata could not be constructed.
    BenchmarkMetadata {
        /// Metadata construction error.
        message: String,
    },

    /// The generated circuit could not be wrapped as a benchmark circuit.
    BenchmarkCircuit {
        /// Benchmark-circuit construction error.
        message: String,
    },
}

impl fmt::Display for RandomCircuitGeneratorError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration {
                field,
                reason,
            } => {
                write!(
                    f,
                    "invalid random-circuit configuration `{field}`: {reason}"
                )
            }

            Self::Limit(error) => {
                write!(f, "random-circuit resource limit exceeded: {error}")
            }

            Self::Random(error) => {
                write!(f, "random-circuit random-stream error: {error}")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "arithmetic overflow while calculating {operation}"
                )
            }

            Self::GateConstruction { message } => {
                write!(
                    f,
                    "random-circuit gate construction failed: {message}"
                )
            }

            Self::CircuitConstruction { message } => {
                write!(
                    f,
                    "random-circuit Quantum IR construction failed: {message}"
                )
            }

            Self::BenchmarkMetadata { message } => {
                write!(
                    f,
                    "random-circuit benchmark metadata construction failed: {message}"
                )
            }

            Self::BenchmarkCircuit { message } => {
                write!(
                    f,
                    "random-circuit BenchmarkCircuit construction failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for RandomCircuitGeneratorError {}

impl From<LimitError> for RandomCircuitGeneratorError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

impl From<RandomError> for RandomCircuitGeneratorError {
    fn from(error: RandomError) -> Self {
        Self::Random(error)
    }
}

// =============================================================================
// Single-qubit gate set
// =============================================================================

/// Gate family from which a randomized single-qubit operation can be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomSingleQubitGate {
    /// Identity.
    I,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,

    /// Hadamard.
    H,

    /// S.
    S,

    /// S-dagger.
    Sdg,

    /// T.
    T,

    /// T-dagger.
    Tdg,

    /// V / square-root-X-style logical gate.
    V,

    /// V-dagger.
    Vdg,

    /// Random RX rotation.
    RX,

    /// Random RY rotation.
    RY,

    /// Random RZ rotation.
    RZ,

    /// Random Phase rotation.
    Phase,

    /// Random U1 rotation.
    U1,

    /// Random U2 rotation.
    U2,

    /// Random U3 rotation.
    U3,
}

impl RandomSingleQubitGate {
    /// Returns the corresponding canonical IR gate kind.
    #[must_use]
    pub const fn gate_kind(
        self,
    ) -> GateKind {
        match self {
            Self::I => GateKind::I,
            Self::X => GateKind::X,
            Self::Y => GateKind::Y,
            Self::Z => GateKind::Z,
            Self::H => GateKind::H,
            Self::S => GateKind::S,
            Self::Sdg => GateKind::Sdg,
            Self::T => GateKind::T,
            Self::Tdg => GateKind::Tdg,
            Self::V => GateKind::V,
            Self::Vdg => GateKind::Vdg,
            Self::RX => GateKind::RX,
            Self::RY => GateKind::RY,
            Self::RZ => GateKind::RZ,
            Self::Phase => GateKind::Phase,
            Self::U1 => GateKind::U1,
            Self::U2 => GateKind::U2,
            Self::U3 => GateKind::U3,
        }
    }

    /// Returns whether the gate requires random numerical parameters.
    #[must_use]
    pub const fn is_parameterized(
        self,
    ) -> bool {
        self.gate_kind().parameter_count() != 0
    }

    /// Returns the required parameter count.
    #[must_use]
    pub const fn parameter_count(
        self,
    ) -> usize {
        self.gate_kind().parameter_count()
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn id(
        self,
    ) -> &'static str {
        match self {
            Self::I => "i",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::H => "h",
            Self::S => "s",
            Self::Sdg => "sdg",
            Self::T => "t",
            Self::Tdg => "tdg",
            Self::V => "v",
            Self::Vdg => "vdg",
            Self::RX => "rx",
            Self::RY => "ry",
            Self::RZ => "rz",
            Self::Phase => "phase",
            Self::U1 => "u1",
            Self::U2 => "u2",
            Self::U3 => "u3",
        }
    }

    /// Returns all built-in single-qubit gate families.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::I,
            Self::X,
            Self::Y,
            Self::Z,
            Self::H,
            Self::S,
            Self::Sdg,
            Self::T,
            Self::Tdg,
            Self::V,
            Self::Vdg,
            Self::RX,
            Self::RY,
            Self::RZ,
            Self::Phase,
            Self::U1,
            Self::U2,
            Self::U3,
        ]
    }

    /// Returns the commonly useful universal random single-qubit gate set.
    ///
    /// This intentionally excludes identity and redundant basis-only
    /// operations from the default to avoid wasting circuit depth.
    #[must_use]
    pub fn default_set() -> Vec<Self> {
        vec![
            Self::H,
            Self::S,
            Self::Sdg,
            Self::T,
            Self::Tdg,
            Self::RX,
            Self::RY,
            Self::RZ,
        ]
    }
}

impl fmt::Display for RandomSingleQubitGate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Two-qubit gate set
// =============================================================================

/// Gate family from which a randomized two-qubit operation can be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomTwoQubitGate {
    /// Controlled-X.
    CX,

    /// Controlled-Y.
    CY,

    /// Controlled-Z.
    CZ,

    /// Controlled-Hadamard.
    CH,

    /// SWAP.
    SWAP,

    /// iSWAP.
    ISWAP,

    /// Echoed cross-resonance.
    ECR,

    /// Controlled RX with random angle.
    CRX,

    /// Controlled RY with random angle.
    CRY,

    /// Controlled RZ with random angle.
    CRZ,
}

impl RandomTwoQubitGate {
    /// Returns the canonical IR gate kind.
    #[must_use]
    pub const fn gate_kind(
        self,
    ) -> GateKind {
        match self {
            Self::CX => GateKind::CX,
            Self::CY => GateKind::CY,
            Self::CZ => GateKind::CZ,
            Self::CH => GateKind::CH,
            Self::SWAP => GateKind::SWAP,
            Self::ISWAP => GateKind::ISWAP,
            Self::ECR => GateKind::ECR,
            Self::CRX => GateKind::CRX,
            Self::CRY => GateKind::CRY,
            Self::CRZ => GateKind::CRZ,
        }
    }

    /// Returns whether the gate requires a random numerical parameter.
    #[must_use]
    pub const fn is_parameterized(
        self,
    ) -> bool {
        self.gate_kind().parameter_count() != 0
    }

    /// Returns a stable identifier.
    #[must_use]
    pub const fn id(
        self,
    ) -> &'static str {
        match self {
            Self::CX => "cx",
            Self::CY => "cy",
            Self::CZ => "cz",
            Self::CH => "ch",
            Self::SWAP => "swap",
            Self::ISWAP => "iswap",
            Self::ECR => "ecr",
            Self::CRX => "crx",
            Self::CRY => "cry",
            Self::CRZ => "crz",
        }
    }

    /// Returns all built-in two-qubit gate families.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::CX,
            Self::CY,
            Self::CZ,
            Self::CH,
            Self::SWAP,
            Self::ISWAP,
            Self::ECR,
            Self::CRX,
            Self::CRY,
            Self::CRZ,
        ]
    }

    /// Returns the default entangling set.
    ///
    /// The default deliberately emphasizes commonly useful fixed entangling
    /// operations while allowing protocol-specific callers to opt into
    /// parameterized controlled rotations.
    #[must_use]
    pub fn default_set() -> Vec<Self> {
        vec![
            Self::CX,
            Self::CZ,
            Self::ISWAP,
            Self::ECR,
        ]
    }
}

impl fmt::Display for RandomTwoQubitGate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Layer strategy
// =============================================================================

/// Controls how random circuit layers are selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomLayerStrategy {
    /// Every layer contains only single-qubit operations.
    SingleQubit,

    /// Every layer is an entangling layer.
    ///
    /// If the circuit contains fewer than two logical qubits, a single-qubit
    /// layer is generated because a two-qubit operation is impossible.
    TwoQubit,

    /// Alternate between single-qubit and two-qubit layers.
    ///
    /// Layer zero is single-qubit; layer one is two-qubit.
    Alternating,

    /// Select a two-qubit layer independently for each layer using the
    /// configured percentage.
    Mixed {
        /// Percentage in `[0, 100]` at which an eligible layer becomes a
        /// two-qubit layer.
        two_qubit_layer_percent: u8,
    },
}

impl Default for RandomLayerStrategy {
    fn default() -> Self {
        Self::Alternating
    }
}

impl RandomLayerStrategy {
    /// Validates the strategy.
    pub fn validate(
        self,
    ) -> Result<(), RandomCircuitGeneratorError> {
        match self {
            Self::Mixed {
                two_qubit_layer_percent,
            } if two_qubit_layer_percent > 100 => {
                Err(
                    RandomCircuitGeneratorError::InvalidConfiguration {
                        field: "two_qubit_layer_percent",
                        reason: "must be between 0 and 100",
                    },
                )
            }

            _ => Ok(()),
        }
    }

    /// Returns a stable identifier.
    #[must_use]
    pub const fn id(
        self,
    ) -> &'static str {
        match self {
            Self::SingleQubit => "single_qubit",
            Self::TwoQubit => "two_qubit",
            Self::Alternating => "alternating",
            Self::Mixed { .. } => "mixed",
        }
    }

    /// Returns whether the strategy can request a two-qubit layer.
    #[must_use]
    pub const fn can_use_two_qubit_layers(
        self,
    ) -> bool {
        match self {
            Self::SingleQubit => false,
            Self::TwoQubit
            | Self::Alternating
            | Self::Mixed { .. } => true,
        }
    }
}

impl fmt::Display for RandomLayerStrategy {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Mixed {
                two_qubit_layer_percent,
            } => {
                write!(
                    f,
                    "mixed({two_qubit_layer_percent}%)"
                )
            }

            _ => f.write_str(self.id()),
        }
    }
}

// =============================================================================
// Pairing strategy
// =============================================================================

/// Controls how logical qubits are paired in an entangling layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomPairingStrategy {
    /// Shuffle the logical-qubit permutation for every layer, then pair
    /// adjacent elements.
    RandomDisjoint,

    /// Pair adjacent logical qubits without shuffling.
    ///
    /// This is useful for deterministic comparison of the random gate choices
    /// while holding pairing structure fixed.
    Adjacent,
}

impl Default for RandomPairingStrategy {
    fn default() -> Self {
        Self::RandomDisjoint
    }
}

impl RandomPairingStrategy {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn id(
        self,
    ) -> &'static str {
        match self {
            Self::RandomDisjoint => "random_disjoint",
            Self::Adjacent => "adjacent",
        }
    }
}

impl fmt::Display for RandomPairingStrategy {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete configuration for production random-circuit generation.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomCircuitConfig {
    /// Number of logical qubits.
    qubits: usize,

    /// Number of logical layers.
    depth: usize,

    /// Candidate single-qubit gate families.
    single_qubit_gates: Vec<RandomSingleQubitGate>,

    /// Candidate two-qubit gate families.
    two_qubit_gates: Vec<RandomTwoQubitGate>,

    /// Layer selection strategy.
    layer_strategy: RandomLayerStrategy,

    /// Two-qubit pairing strategy.
    pairing_strategy: RandomPairingStrategy,

    /// Benchmark identifier.
    benchmark_id: String,

    /// Experiment identifier.
    experiment_id: String,

    /// Optional benchmark case identifier.
    case_id: Option<String>,

    /// Circuit role in the benchmark.
    role: BenchmarkCircuitRole,

    /// Root benchmark seed.
    seed: BenchmarkSeed,

    /// Whether downstream analysis requires an independently computed ideal
    /// reference distribution.
    requires_ideal_reference: bool,
}

impl RandomCircuitConfig {
    /// Creates a production-default configuration.
    pub fn new(
        qubits: usize,
        depth: usize,
    ) -> Result<Self, RandomCircuitGeneratorError> {
        Self::builder(qubits, depth).build()
    }

    /// Starts a configuration builder.
    #[must_use]
    pub fn builder(
        qubits: usize,
        depth: usize,
    ) -> RandomCircuitConfigBuilder {
        RandomCircuitConfigBuilder {
            qubits,
            depth,
            single_qubit_gates:
                RandomSingleQubitGate::default_set(),
            two_qubit_gates:
                RandomTwoQubitGate::default_set(),
            layer_strategy:
                RandomLayerStrategy::default(),
            pairing_strategy:
                RandomPairingStrategy::default(),
            benchmark_id:
                "random_circuit".to_owned(),
            experiment_id:
                "random_circuit".to_owned(),
            case_id: None,
            role: BenchmarkCircuitRole::Random,
            seed: BenchmarkSeed::from_u64(0),
            requires_ideal_reference: false,
        }
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubits(
        &self,
    ) -> usize {
        self.qubits
    }

    /// Returns the logical depth.
    #[must_use]
    pub const fn depth(
        &self,
    ) -> usize {
        self.depth
    }

    /// Returns the configured single-qubit gate set.
    #[must_use]
    pub fn single_qubit_gates(
        &self,
    ) -> &[RandomSingleQubitGate] {
        &self.single_qubit_gates
    }

    /// Returns the configured two-qubit gate set.
    #[must_use]
    pub fn two_qubit_gates(
        &self,
    ) -> &[RandomTwoQubitGate] {
        &self.two_qubit_gates
    }

    /// Returns the layer strategy.
    #[must_use]
    pub const fn layer_strategy(
        &self,
    ) -> RandomLayerStrategy {
        self.layer_strategy
    }

    /// Returns the pairing strategy.
    #[must_use]
    pub const fn pairing_strategy(
        &self,
    ) -> RandomPairingStrategy {
        self.pairing_strategy
    }

    /// Returns the benchmark identifier.
    #[must_use]
    pub fn benchmark_id(
        &self,
    ) -> &str {
        &self.benchmark_id
    }

    /// Returns the experiment identifier.
    #[must_use]
    pub fn experiment_id(
        &self,
    ) -> &str {
        &self.experiment_id
    }

    /// Returns the optional case identifier.
    #[must_use]
    pub fn case_id(
        &self,
    ) -> Option<&str> {
        self.case_id.as_deref()
    }

    /// Returns the circuit role.
    #[must_use]
    pub const fn role(
        &self,
    ) -> BenchmarkCircuitRole {
        self.role
    }

    /// Returns the root benchmark seed.
    #[must_use]
    pub const fn seed(
        &self,
    ) -> BenchmarkSeed {
        self.seed
    }

    /// Returns whether an ideal reference distribution is required.
    #[must_use]
    pub const fn requires_ideal_reference(
        &self,
    ) -> bool {
        self.requires_ideal_reference
    }

    /// Validates the configuration against explicit production limits.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), RandomCircuitGeneratorError> {
        limits
            .validate()
            .map_err(RandomCircuitGeneratorError::Limit)?;

        if self.qubits == 0 {
            return Err(
                RandomCircuitGeneratorError::InvalidConfiguration {
                    field: "qubits",
                    reason: "must be greater than zero",
                },
            );
        }

        if self.depth == 0 {
            return Err(
                RandomCircuitGeneratorError::InvalidConfiguration {
                    field: "depth",
                    reason: "must be greater than zero",
                },
            );
        }

        limits.check_qubits(self.qubits)?;

        limits.check_circuit_depth(self.depth)?;

        if self.single_qubit_gates.is_empty() {
            return Err(
                RandomCircuitGeneratorError::InvalidConfiguration {
                    field: "single_qubit_gates",
                    reason: "must contain at least one gate family",
                },
            );
        }

        if self.two_qubit_gates.is_empty()
            && self.layer_strategy.can_use_two_qubit_layers()
            && self.qubits >= 2
        {
            return Err(
                RandomCircuitGeneratorError::InvalidConfiguration {
                    field: "two_qubit_gates",
                    reason:
                        "must contain at least one gate family when two-qubit layers are possible",
                },
            );
        }

        self.layer_strategy.validate()?;

        validate_identifier(
            "benchmark_id",
            &self.benchmark_id,
        )?;

        validate_identifier(
            "experiment_id",
            &self.experiment_id,
        )?;

        if let Some(case_id) = &self.case_id {
            validate_identifier(
                "case_id",
                case_id,
            )?;
        }

        // The generated circuit can never contain more than `qubits` gates
        // per logical layer because every qubit participates in at most one
        // operation in a layer.
        let estimated_max_operations =
            self.qubits
                .checked_mul(self.depth)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation: "qubits × depth",
                    },
                )?;

        limits.check_gate_count(
            estimated_max_operations,
        )?;

        // The maximum number of disjoint two-qubit operations per layer is
        // floor(qubits / 2).
        let max_two_qubit_per_layer =
            self.qubits / 2;

        let estimated_max_two_qubit =
            max_two_qubit_per_layer
                .checked_mul(self.depth)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation:
                            "floor(qubits / 2) × depth",
                    },
                )?;

        limits.check_two_qubit_gates(
            estimated_max_two_qubit,
        )?;

        Ok(())
    }
}

// =============================================================================
// Configuration builder
// =============================================================================

/// Builder for [`RandomCircuitConfig`].
#[derive(Debug, Clone)]
pub struct RandomCircuitConfigBuilder {
    qubits: usize,
    depth: usize,
    single_qubit_gates:
        Vec<RandomSingleQubitGate>,
    two_qubit_gates:
        Vec<RandomTwoQubitGate>,
    layer_strategy:
        RandomLayerStrategy,
    pairing_strategy:
        RandomPairingStrategy,
    benchmark_id: String,
    experiment_id: String,
    case_id: Option<String>,
    role: BenchmarkCircuitRole,
    seed: BenchmarkSeed,
    requires_ideal_reference: bool,
}

impl RandomCircuitConfigBuilder {
    /// Replaces the single-qubit gate set.
    #[must_use]
    pub fn single_qubit_gates(
        mut self,
        gates: Vec<RandomSingleQubitGate>,
    ) -> Self {
        self.single_qubit_gates = gates;
        self
    }

    /// Replaces the two-qubit gate set.
    #[must_use]
    pub fn two_qubit_gates(
        mut self,
        gates: Vec<RandomTwoQubitGate>,
    ) -> Self {
        self.two_qubit_gates = gates;
        self
    }

    /// Sets the layer strategy.
    #[must_use]
    pub fn layer_strategy(
        mut self,
        strategy: RandomLayerStrategy,
    ) -> Self {
        self.layer_strategy = strategy;
        self
    }

    /// Sets the logical-qubit pairing strategy.
    #[must_use]
    pub fn pairing_strategy(
        mut self,
        strategy: RandomPairingStrategy,
    ) -> Self {
        self.pairing_strategy = strategy;
        self
    }

    /// Sets the benchmark identifier.
    #[must_use]
    pub fn benchmark_id<S>(
        mut self,
        value: S,
    ) -> Self
    where
        S: Into<String>,
    {
        self.benchmark_id = value.into();
        self
    }

    /// Sets the experiment identifier.
    #[must_use]
    pub fn experiment_id<S>(
        mut self,
        value: S,
    ) -> Self
    where
        S: Into<String>,
    {
        self.experiment_id = value.into();
        self
    }

    /// Sets an optional case identifier.
    #[must_use]
    pub fn case_id<S>(
        mut self,
        value: S,
    ) -> Self
    where
        S: Into<String>,
    {
        self.case_id = Some(value.into());
        self
    }

    /// Clears the case identifier.
    #[must_use]
    pub fn without_case_id(
        mut self,
    ) -> Self {
        self.case_id = None;
        self
    }

    /// Sets the benchmark circuit role.
    #[must_use]
    pub fn role(
        mut self,
        role: BenchmarkCircuitRole,
    ) -> Self {
        self.role = role;
        self
    }

    /// Sets the root benchmark seed.
    #[must_use]
    pub fn seed(
        mut self,
        seed: BenchmarkSeed,
    ) -> Self {
        self.seed = seed;
        self
    }

    /// Sets a root seed from a 64-bit value.
    #[must_use]
    pub fn seed_u64(
        mut self,
        seed: u64,
    ) -> Self {
        self.seed = BenchmarkSeed::from_u64(seed);
        self
    }

    /// Marks the circuit as requiring an ideal reference distribution.
    #[must_use]
    pub fn requires_ideal_reference(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_ideal_reference = required;
        self
    }

    /// Builds and validates the configuration.
    pub fn build(
        self,
    ) -> Result<RandomCircuitConfig, RandomCircuitGeneratorError> {
        let config = RandomCircuitConfig {
            qubits: self.qubits,
            depth: self.depth,
            single_qubit_gates:
                self.single_qubit_gates,
            two_qubit_gates:
                self.two_qubit_gates,
            layer_strategy:
                self.layer_strategy,
            pairing_strategy:
                self.pairing_strategy,
            benchmark_id:
                self.benchmark_id,
            experiment_id:
                self.experiment_id,
            case_id:
                self.case_id,
            role: self.role,
            seed: self.seed,
            requires_ideal_reference:
                self.requires_ideal_reference,
        };

        config.validate(
            &BenchmarkLimits::production(),
        )?;

        Ok(config)
    }
}

// =============================================================================
// Generation summary
// =============================================================================

/// Exact resource summary returned for a generated random circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomCircuitGenerationSummary {
    /// Logical width.
    pub qubits: usize,

    /// Requested logical depth.
    pub requested_depth: usize,

    /// Actual logical operation count.
    pub operations: usize,

    /// Actual single-qubit operation count.
    pub single_qubit_operations: usize,

    /// Actual two-qubit operation count.
    pub two_qubit_operations: usize,

    /// Actual multi-qubit operation count.
    pub multi_qubit_operations: usize,

    /// Number of generated layers.
    pub layers: usize,

    /// Root benchmark seed.
    pub seed: BenchmarkSeed,
}

impl RandomCircuitGenerationSummary {
    /// Returns whether the generated circuit contains entangling operations.
    #[must_use]
    pub const fn contains_two_qubit_operations(
        self,
    ) -> bool {
        self.two_qubit_operations != 0
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production random logical-circuit generator.
///
/// The generator itself contains no mutable random state. Each generation
/// request creates explicit deterministic streams from the configuration seed.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomCircuitGenerator;

impl RandomCircuitGenerator {
    /// Creates a new random-circuit generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates a canonical logical Quantum IR circuit using production
    /// benchmark limits.
    pub fn generate_circuit(
        &self,
        config: &RandomCircuitConfig,
    ) -> Result<QuantumCircuit, RandomCircuitGeneratorError> {
        self.generate_circuit_with_limits(
            config,
            BenchmarkLimits::production(),
        )
    }

    /// Generates a canonical logical Quantum IR circuit under an explicit
    /// benchmark resource policy.
    pub fn generate_circuit_with_limits(
        &self,
        config: &RandomCircuitConfig,
        limits: BenchmarkLimits,
    ) -> Result<QuantumCircuit, RandomCircuitGeneratorError> {
        config.validate(&limits)?;

        let estimated_operations =
            config
                .qubits
                .checked_mul(config.depth)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation: "qubits × depth",
                    },
                )?;

        limits.check_gate_count(
            estimated_operations,
        )?;

        let mut operations =
            Vec::with_capacity(estimated_operations);

        let root_stream =
            RandomStream::from_seed(config.seed);

        for layer_index in 0..config.depth {
            let layer_stream =
                root_stream.fork(
                    layer_index as u64,
                );

            self.generate_layer(
                config,
                layer_stream,
                &mut operations,
                limits,
                layer_index,
            )?;
        }

        limits.check_gate_count(
            operations.len(),
        )?;

        QuantumCircuit::from_operations(
            config.qubits,
            0,
            operations,
        )
        .map_err(|error| {
            RandomCircuitGeneratorError::CircuitConstruction {
                message: error.to_string(),
            }
        })
    }

    /// Generates a complete benchmarking-layer circuit.
    pub fn generate(
        &self,
        config: &RandomCircuitConfig,
    ) -> Result<BenchmarkCircuit, RandomCircuitGeneratorError> {
        self.generate_with_limits(
            config,
            BenchmarkLimits::production(),
        )
    }

    /// Generates a complete benchmarking-layer circuit under explicit
    /// resource limits.
    pub fn generate_with_limits(
        &self,
        config: &RandomCircuitConfig,
        limits: BenchmarkLimits,
    ) -> Result<BenchmarkCircuit, RandomCircuitGeneratorError> {
        config.validate(&limits)?;

        let circuit =
            self.generate_circuit_with_limits(
                config,
                limits,
            )?;

        let generation =
            BenchmarkCircuitGeneration::new(
                seed_to_u64(config.seed),
                0,
                RANDOM_CIRCUIT_GENERATOR_REVISION,
            );

        let descriptor =
            BenchmarkCircuitDescriptor::with_case_id(
                config.benchmark_id.clone(),
                config.experiment_id.clone(),
                config.case_id.clone(),
                config.role,
                generation,
            )
            .map_err(|error| {
                RandomCircuitGeneratorError::BenchmarkMetadata {
                    message: error.to_string(),
                }
            })?
            .with_ideal_reference(
                config.requires_ideal_reference,
            );

        BenchmarkCircuit::new(
            circuit,
            descriptor,
        )
        .map_err(|error| {
            RandomCircuitGeneratorError::BenchmarkCircuit {
                message: error.to_string(),
            }
        })
    }

    /// Generates a random benchmark circuit and returns its exact resource
    /// summary.
    pub fn generate_with_summary(
        &self,
        config: &RandomCircuitConfig,
    ) -> Result<
        (
            BenchmarkCircuit,
            RandomCircuitGenerationSummary,
        ),
        RandomCircuitGeneratorError,
    > {
        let benchmark_circuit =
            self.generate(config)?;

        let resources =
            benchmark_circuit.resources();

        let summary =
            RandomCircuitGenerationSummary {
                qubits: resources.qubits(),
                requested_depth: config.depth,
                operations: resources.operations(),
                single_qubit_operations:
                    resources.single_qubit_gates(),
                two_qubit_operations:
                    resources.two_qubit_gates(),
                multi_qubit_operations:
                    resources.multi_qubit_gates(),
                layers: config.depth,
                seed: config.seed,
            };

        Ok((
            benchmark_circuit,
            summary,
        ))
    }

    // -------------------------------------------------------------------------
    // Layer generation
    // -------------------------------------------------------------------------

    fn generate_layer(
        &self,
        config: &RandomCircuitConfig,
        mut stream: RandomStream,
        operations: &mut Vec<Gate>,
        limits: BenchmarkLimits,
        layer_index: usize,
    ) -> Result<(), RandomCircuitGeneratorError> {
        let use_two_qubit_layer =
            self.select_layer_kind(
                config,
                &mut stream,
                layer_index,
            )?;

        if use_two_qubit_layer
            && config.qubits >= 2
        {
            self.generate_two_qubit_layer(
                config,
                &mut stream,
                operations,
                limits,
            )
        } else {
            self.generate_single_qubit_layer(
                config,
                &mut stream,
                operations,
                limits,
            )
        }
    }

    fn select_layer_kind(
        &self,
        config: &RandomCircuitConfig,
        stream: &mut RandomStream,
        layer_index: usize,
    ) -> Result<bool, RandomCircuitGeneratorError> {
        match config.layer_strategy {
            RandomLayerStrategy::SingleQubit => {
                Ok(false)
            }

            RandomLayerStrategy::TwoQubit => {
                Ok(config.qubits >= 2)
            }

            RandomLayerStrategy::Alternating => {
                Ok(
                    config.qubits >= 2
                        && layer_index % 2 == 1,
                )
            }

            RandomLayerStrategy::Mixed {
                two_qubit_layer_percent,
            } => {
                if config.qubits < 2 {
                    return Ok(false);
                }

                if two_qubit_layer_percent == 0 {
                    return Ok(false);
                }

                if two_qubit_layer_percent == 100 {
                    return Ok(true);
                }

                let draw =
                    stream.range_u64(0, 100)?;

                Ok(
                    draw
                        < two_qubit_layer_percent as u64,
                )
            }
        }
    }

    fn generate_single_qubit_layer(
        &self,
        config: &RandomCircuitConfig,
        stream: &mut RandomStream,
        operations: &mut Vec<Gate>,
        limits: BenchmarkLimits,
    ) -> Result<(), RandomCircuitGeneratorError> {
        let additional =
            config.qubits;

        let new_length =
            operations
                .len()
                .checked_add(additional)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation:
                            "existing operations + single-qubit layer",
                    },
                )?;

        limits.check_gate_count(
            new_length,
        )?;

        for qubit_index in 0..config.qubits {
            let gate_family =
                select_single_qubit_gate(
                    stream,
                    &config.single_qubit_gates,
                )?;

            let parameters =
                self.random_parameters_for_single_qubit_gate(
                    gate_family,
                    stream,
                )?;

            let gate =
                Gate::new(
                    gate_family.gate_kind(),
                    vec![
                        QubitId::new(
                            qubit_index,
                        ),
                    ],
                    parameters,
                    None,
                    None,
                )
                .map_err(|error| {
                    RandomCircuitGeneratorError::GateConstruction {
                        message: error.to_string(),
                    }
                })?;

            operations.push(gate);
        }

        Ok(())
    }

    fn generate_two_qubit_layer(
        &self,
        config: &RandomCircuitConfig,
        stream: &mut RandomStream,
        operations: &mut Vec<Gate>,
        limits: BenchmarkLimits,
    ) -> Result<(), RandomCircuitGeneratorError> {
        let mut permutation =
            match config.pairing_strategy {
                RandomPairingStrategy::RandomDisjoint => {
                    stream.permutation(config.qubits)?
                }

                RandomPairingStrategy::Adjacent => {
                    (0..config.qubits).collect()
                }
            };

        let pair_count =
            config.qubits / 2;

        let unpaired_count =
            config.qubits % 2;

        let required_operations =
            pair_count
                .checked_add(unpaired_count)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation:
                            "two-qubit layer operation count",
                    },
                )?;

        let new_length =
            operations
                .len()
                .checked_add(required_operations)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation:
                            "existing operations + two-qubit layer",
                    },
                )?;

        limits.check_gate_count(
            new_length,
        )?;

        let existing_two_qubit =
            count_two_qubit_operations(
                operations,
            );

        let new_two_qubit =
            existing_two_qubit
                .checked_add(pair_count)
                .ok_or(
                    RandomCircuitGeneratorError::ArithmeticOverflow {
                        operation:
                            "existing two-qubit operations + layer",
                    },
                )?;

        limits.check_two_qubit_gates(
            new_two_qubit,
        )?;

        for pair_index in 0..pair_count {
            let first_index =
                pair_index
                    .checked_mul(2)
                    .ok_or(
                        RandomCircuitGeneratorError::ArithmeticOverflow {
                            operation:
                                "pair index × 2",
                        },
                    )?;

            let second_index =
                first_index
                    .checked_add(1)
                    .ok_or(
                        RandomCircuitGeneratorError::ArithmeticOverflow {
                            operation:
                                "pair first index + 1",
                        },
                    )?;

            let first =
                permutation[first_index];

            let second =
                permutation[second_index];

            let gate_family =
                select_two_qubit_gate(
                    stream,
                    &config.two_qubit_gates,
                )?;

            let parameters =
                self.random_parameters_for_two_qubit_gate(
                    gate_family,
                    stream,
                )?;

            let gate =
                Gate::new(
                    gate_family.gate_kind(),
                    vec![
                        QubitId::new(first),
                        QubitId::new(second),
                    ],
                    parameters,
                    None,
                    None,
                )
                .map_err(|error| {
                    RandomCircuitGeneratorError::GateConstruction {
                        message: error.to_string(),
                    }
                })?;

            operations.push(gate);
        }

        // An odd-width circuit has one unpaired logical qubit. Rather than
        // leaving it completely idle, exercise it with one single-qubit
        // operation. This preserves the layer's logical width without ever
        // violating disjointness.
        if unpaired_count != 0 {
            let unpaired =
                permutation
                    .pop()
                    .ok_or(
                        RandomCircuitGeneratorError::InvalidConfiguration {
                            field: "qubits",
                            reason:
                                "internal pairing state unexpectedly became empty",
                        },
                    )?;

            let gate_family =
                select_single_qubit_gate(
                    stream,
                    &config.single_qubit_gates,
                )?;

            let parameters =
                self.random_parameters_for_single_qubit_gate(
                    gate_family,
                    stream,
                )?;

            let gate =
                Gate::new(
                    gate_family.gate_kind(),
                    vec![
                        QubitId::new(unpaired),
                    ],
                    parameters,
                    None,
                    None,
                )
                .map_err(|error| {
                    RandomCircuitGeneratorError::GateConstruction {
                        message: error.to_string(),
                    }
                })?;

            operations.push(gate);
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Parameter generation
    // -------------------------------------------------------------------------

    fn random_parameters_for_single_qubit_gate(
        &self,
        gate: RandomSingleQubitGate,
        stream: &mut RandomStream,
    ) -> Result<Vec<Parameter>, RandomCircuitGeneratorError> {
        match gate {
            RandomSingleQubitGate::RX
            | RandomSingleQubitGate::RY
            | RandomSingleQubitGate::RZ
            | RandomSingleQubitGate::Phase
            | RandomSingleQubitGate::U1 => {
                Ok(
                    vec![
                        random_parameter(stream)?,
                    ],
                )
            }

            RandomSingleQubitGate::U2 => {
                Ok(
                    vec![
                        random_parameter(stream)?,
                        random_parameter(stream)?,
                    ],
                )
            }

            RandomSingleQubitGate::U3 => {
                Ok(
                    vec![
                        random_parameter(stream)?,
                        random_parameter(stream)?,
                        random_parameter(stream)?,
                    ],
                )
            }

            _ => Ok(Vec::new()),
        }
    }

    fn random_parameters_for_two_qubit_gate(
        &self,
        gate: RandomTwoQubitGate,
        stream: &mut RandomStream,
    ) -> Result<Vec<Parameter>, RandomCircuitGeneratorError> {
        match gate {
            RandomTwoQubitGate::CRX
            | RandomTwoQubitGate::CRY
            | RandomTwoQubitGate::CRZ => {
                Ok(
                    vec![
                        random_parameter(stream)?,
                    ],
                )
            }

            _ => Ok(Vec::new()),
        }
    }
}

// =============================================================================
// Selection helpers
// =============================================================================

fn select_single_qubit_gate(
    stream: &mut RandomStream,
    gates: &[RandomSingleQubitGate],
) -> Result<RandomSingleQubitGate, RandomCircuitGeneratorError> {
    stream
        .choose(gates)
        .copied()
        .map_err(RandomCircuitGeneratorError::Random)
}

fn select_two_qubit_gate(
    stream: &mut RandomStream,
    gates: &[RandomTwoQubitGate],
) -> Result<RandomTwoQubitGate, RandomCircuitGeneratorError> {
    stream
        .choose(gates)
        .copied()
        .map_err(RandomCircuitGeneratorError::Random)
}

fn random_parameter(
    stream: &mut RandomStream,
) -> Result<Parameter, RandomCircuitGeneratorError> {
    let angle =
        stream.next_f64()? * TWO_PI;

    Parameter::constant(angle).map_err(|error| {
        RandomCircuitGeneratorError::GateConstruction {
            message: error.to_string(),
        }
    })
}

fn count_two_qubit_operations(
    operations: &[Gate],
) -> usize {
    operations
        .iter()
        .filter(|operation| {
            operation.qubits().len() == 2
        })
        .count()
}

fn seed_to_u64(
    seed: BenchmarkSeed,
) -> u64 {
    let words = seed.words();

    // The BenchmarkCircuitGeneration contract stores a u64 seed. The complete
    // 256-bit seed remains the authoritative generator identity in the
    // configuration/provenance layer; this value is a stable compact
    // representation for the current benchmark-circuit metadata contract.
    words[0]
        ^ words[1].rotate_left(13)
        ^ words[2].rotate_left(29)
        ^ words[3].rotate_left(47)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), RandomCircuitGeneratorError> {
    if value.is_empty() {
        return Err(
            RandomCircuitGeneratorError::InvalidConfiguration {
                field,
                reason: "must not be empty",
            },
        );
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(
            RandomCircuitGeneratorError::InvalidConfiguration {
                field,
                reason:
                    "exceeds the maximum identifier length",
            },
        );
    }

    Ok(())
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Generates a random logical Quantum IR circuit using production defaults.
pub fn generate_random_circuit(
    qubits: usize,
    depth: usize,
    seed: BenchmarkSeed,
) -> Result<QuantumCircuit, RandomCircuitGeneratorError> {
    let config =
        RandomCircuitConfig::builder(
            qubits,
            depth,
        )
        .seed(seed)
        .build()?;

    RandomCircuitGenerator::new()
        .generate_circuit(&config)
}

/// Generates a random benchmarking circuit using production defaults.
pub fn generate_random_benchmark_circuit(
    qubits: usize,
    depth: usize,
    seed: BenchmarkSeed,
) -> Result<BenchmarkCircuit, RandomCircuitGeneratorError> {
    let config =
        RandomCircuitConfig::builder(
            qubits,
            depth,
        )
        .seed(seed)
        .requires_ideal_reference(true)
        .build()?;

    RandomCircuitGenerator::new()
        .generate(&config)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(
        qubits: usize,
        depth: usize,
        seed: u64,
    ) -> RandomCircuitConfig {
        RandomCircuitConfig::builder(
            qubits,
            depth,
        )
        .benchmark_id("test.random")
        .experiment_id("random-circuits")
        .seed_u64(seed)
        .build()
        .expect("test configuration must be valid")
    }

    #[test]
    fn identical_seed_and_configuration_are_reproducible() {
        let generator =
            RandomCircuitGenerator::new();

        let config =
            test_config(8, 12, 42);

        let first =
            generator
                .generate_circuit(&config)
                .expect("first generation must succeed");

        let second =
            generator
                .generate_circuit(&config)
                .expect("second generation must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn different_seeds_produce_different_circuits() {
        let generator =
            RandomCircuitGenerator::new();

        let first_config =
            test_config(8, 12, 42);

        let second_config =
            test_config(8, 12, 43);

        let first =
            generator
                .generate_circuit(&first_config)
                .expect("first generation must succeed");

        let second =
            generator
                .generate_circuit(&second_config)
                .expect("second generation must succeed");

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn generated_circuit_is_canonically_valid() {
        let generator =
            RandomCircuitGenerator::new();

        let config =
            test_config(12, 20, 1234);

        let circuit =
            generator
                .generate_circuit(&config)
                .expect("generation must succeed");

        assert!(
            crate::quantum::ir::validate_circuit(
                &circuit
            )
            .is_ok()
        );
    }

    #[test]
    fn generated_benchmark_circuit_is_reproducible() {
        let generator =
            RandomCircuitGenerator::new();

        let config =
            test_config(6, 10, 99);

        let first =
            generator
                .generate(&config)
                .expect("first generation must succeed");

        let second =
            generator
                .generate(&config)
                .expect("second generation must succeed");

        assert_eq!(
            first,
            second
        );

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn single_qubit_strategy_contains_only_single_qubit_operations() {
        let config =
            RandomCircuitConfig::builder(
                8,
                6,
            )
            .layer_strategy(
                RandomLayerStrategy::SingleQubit,
            )
            .seed_u64(7)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        assert!(
            circuit
                .operations()
                .iter()
                .all(|gate| gate.qubits().len() == 1)
        );
    }

    #[test]
    fn two_qubit_layers_are_disjoint() {
        let config =
            RandomCircuitConfig::builder(
                9,
                8,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .seed_u64(123)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        for gate in circuit.operations() {
            assert!(
                gate.qubits().len() == 1
                    || gate.qubits().len() == 2
            );
        }
    }

    #[test]
    fn odd_width_keeps_unpaired_qubit_active() {
        let config =
            RandomCircuitConfig::builder(
                5,
                4,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .pairing_strategy(
                RandomPairingStrategy::Adjacent,
            )
            .seed_u64(10)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        let operations =
            circuit.operations();

        // Each two-qubit layer contains two 2Q operations and one 1Q
        // operation for the odd-width remainder.
        assert_eq!(
            operations.len(),
            12
        );
    }

    #[test]
    fn parameterized_gate_sets_generate_finite_parameters() {
        let config =
            RandomCircuitConfig::builder(
                4,
                6,
            )
            .single_qubit_gates(vec![
                RandomSingleQubitGate::RX,
                RandomSingleQubitGate::RY,
                RandomSingleQubitGate::RZ,
                RandomSingleQubitGate::U2,
                RandomSingleQubitGate::U3,
            ])
            .two_qubit_gates(vec![
                RandomTwoQubitGate::CRX,
                RandomTwoQubitGate::CRY,
                RandomTwoQubitGate::CRZ,
            ])
            .layer_strategy(
                RandomLayerStrategy::Alternating,
            )
            .seed_u64(123456)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        for gate in circuit.operations() {
            for parameter in gate.parameters() {
                if let Parameter::Constant(value) =
                    parameter
                {
                    assert!(value.is_finite());
                    assert!(
                        *value >= 0.0
                    );
                    assert!(
                        *value < TWO_PI
                    );
                }
            }
        }
    }

    #[test]
    fn mixed_zero_percent_never_selects_two_qubit_layers() {
        let config =
            RandomCircuitConfig::builder(
                8,
                20,
            )
            .layer_strategy(
                RandomLayerStrategy::Mixed {
                    two_qubit_layer_percent: 0,
                },
            )
            .seed_u64(55)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        assert!(
            circuit
                .operations()
                .iter()
                .all(|gate| gate.qubits().len() == 1)
        );
    }

    #[test]
    fn mixed_one_hundred_percent_selects_two_qubit_layers_when_possible() {
        let config =
            RandomCircuitConfig::builder(
                8,
                20,
            )
            .layer_strategy(
                RandomLayerStrategy::Mixed {
                    two_qubit_layer_percent: 100,
                },
            )
            .seed_u64(55)
            .build()
            .expect("configuration must be valid");

        let circuit =
            RandomCircuitGenerator::new()
                .generate_circuit(&config)
                .expect("generation must succeed");

        assert!(
            circuit
                .operations()
                .iter()
                .any(|gate| gate.qubits().len() == 2)
        );
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let result =
            RandomCircuitConfig::builder(
                0,
                4,
            )
            .build();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn zero_depth_is_rejected() {
        let result =
            RandomCircuitConfig::builder(
                4,
                0,
            )
            .build();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn empty_single_qubit_gate_set_is_rejected() {
        let result =
            RandomCircuitConfig::builder(
                4,
                4,
            )
            .single_qubit_gates(Vec::new())
            .build();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn empty_two_qubit_gate_set_is_rejected_when_needed() {
        let result =
            RandomCircuitConfig::builder(
                4,
                4,
            )
            .two_qubit_gates(Vec::new())
            .build();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn adjacent_pairing_is_seed_independent_for_pair_structure() {
        let first_config =
            RandomCircuitConfig::builder(
                6,
                2,
            )
            .pairing_strategy(
                RandomPairingStrategy::Adjacent,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .seed_u64(1)
            .build()
            .expect("configuration must be valid");

        let second_config =
            RandomCircuitConfig::builder(
                6,
                2,
            )
            .pairing_strategy(
                RandomPairingStrategy::Adjacent,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .seed_u64(2)
            .build()
            .expect("configuration must be valid");

        let generator =
            RandomCircuitGenerator::new();

        let first =
            generator
                .generate_circuit(&first_config)
                .expect("generation must succeed");

        let second =
            generator
                .generate_circuit(&second_config)
                .expect("generation must succeed");

        let first_pairs =
            first
                .operations()
                .iter()
                .filter_map(|gate| {
                    if gate.qubits().len() == 2 {
                        Some((
                            gate.qubits()[0],
                            gate.qubits()[1],
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

        let second_pairs =
            second
                .operations()
                .iter()
                .filter_map(|gate| {
                    if gate.qubits().len() == 2 {
                        Some((
                            gate.qubits()[0],
                            gate.qubits()[1],
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

        assert_eq!(
            first_pairs,
            second_pairs
        );
    }

    #[test]
    fn random_disjoint_pairing_can_change_pair_structure() {
        let first_config =
            RandomCircuitConfig::builder(
                8,
                4,
            )
            .pairing_strategy(
                RandomPairingStrategy::RandomDisjoint,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .seed_u64(1)
            .build()
            .expect("configuration must be valid");

        let second_config =
            RandomCircuitConfig::builder(
                8,
                4,
            )
            .pairing_strategy(
                RandomPairingStrategy::RandomDisjoint,
            )
            .layer_strategy(
                RandomLayerStrategy::TwoQubit,
            )
            .seed_u64(2)
            .build()
            .expect("configuration must be valid");

        let generator =
            RandomCircuitGenerator::new();

        let first =
            generator
                .generate_circuit(&first_config)
                .expect("generation must succeed");

        let second =
            generator
                .generate_circuit(&second_config)
                .expect("generation must succeed");

        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn default_seed_is_reproducible() {
        let first =
            generate_random_circuit(
                4,
                4,
                BenchmarkSeed::from_u64(0),
            )
            .expect("generation must succeed");

        let second =
            generate_random_circuit(
                4,
                4,
                BenchmarkSeed::from_u64(0),
            )
            .expect("generation must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn production_limits_are_checked_before_generation() {
        let limits =
            BenchmarkLimits {
                max_qubits: 2,
                ..BenchmarkLimits::production()
            };

        let config =
            RandomCircuitConfig::builder(
                3,
                10,
            )
            .build()
            .expect("configuration is valid against production defaults");

        let result =
            RandomCircuitGenerator::new()
                .generate_circuit_with_limits(
                    &config,
                    limits,
                );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn generator_metadata_identifiers_are_stable() {
        assert_eq!(
            RANDOM_CIRCUIT_GENERATOR_REVISION,
            1
        );

        assert_eq!(
            RANDOM_CIRCUIT_GENERATOR_ALGORITHM_ID,
            "zamani.benchmarking.random-circuits.v1"
        );

        assert_eq!(
            RANDOM_ALGORITHM_ID,
            "splitmix64-zamani-v1"
        );
    }
}