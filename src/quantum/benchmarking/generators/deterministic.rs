//! Zamani Quantum Benchmarking — Deterministic Circuit Generation
//!
//! Production deterministic circuit generation for the Zamani quantum
//! benchmarking subsystem.
//!
//! # Purpose
//!
//! This module generates reproducible logical quantum circuits without using
//! process-global randomness, hardware information, backend SDKs, routing,
//! scheduling, optimization, calibration, or execution.
//!
//! It is the deterministic foundation for:
//!
//! - benchmark fixtures;
//! - regression tests;
//! - reproducibility tests;
//! - application benchmark fixtures;
//! - protocol development;
//! - simulator tests;
//! - benchmark debugging;
//! - deterministic reference workloads;
//! - future benchmark replay;
//! - CI-safe quantum workloads.
//!
//! # Architectural boundary
//!
//! ```text
//! DeterministicCircuitConfig
//!            │
//!            ▼
//!   deterministic generator
//!            │
//!            ▼
//!     QuantumCircuit
//!            │
//!            ▼
//!     BenchmarkCircuit
//!            │
//!       ┌────┴────┐
//!       ▼         ▼
//!   execution   analysis
//! ```
//!
//! This module deliberately does NOT:
//!
//! - execute circuits;
//! - select hardware;
//! - perform routing;
//! - perform scheduling;
//! - perform optimization;
//! - perform calibration;
//! - communicate with QPUs;
//! - generate physical-qubit mappings;
//! - perform statistical analysis;
//! - implement a benchmark protocol;
//! - depend on a simulator;
//! - depend on a particular quantum technology;
//! - generate benchmark results.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Determinism contract
//!
//! For a fixed:
//!
//! - configuration;
//! - generator algorithm version;
//! - Rust implementation;
//! - Quantum IR semantic version;
//!
//! this generator produces the same logical operation sequence.
//!
//! There is:
//!
//! - no hidden RNG;
//! - no system clock;
//! - no environment-variable input;
//! - no filesystem input;
//! - no network input;
//! - no process-global mutable state;
//! - no iteration over unordered collections;
//! - no dependence on hardware topology.
//!
//! The generation seed is retained as benchmark provenance metadata even
//! though this deterministic generator does not use randomness. This allows
//! the same benchmark-generation contract to carry a uniform seed field when
//! deterministic and randomized generators are compared or replayed.
//!
//! # Layer semantics
//!
//! `depth` means the requested number of logical layers.
//!
//! A single-qubit layer contains at most one operation per logical qubit.
//!
//! An entangling layer contains disjoint two-qubit operations. If the number
//! of qubits is odd, the final qubit is intentionally left idle for that
//! layer.
//!
//! This keeps the generated workload hardware-independent while preserving
//! an unambiguous logical-layer model.
//!
//! # Important distinction
//!
//! This module is a circuit generator, not a Quantum Volume generator.
//!
//! Quantum Volume-specific generation belongs in:
//!
//! `generators/qv.rs`
//!
//! Generic randomized generation belongs in:
//!
//! `generators/random.rs`
//!
//! Mirror circuits belong in:
//!
//! `generators/mirror_circuits.rs`
//!
//! Clifford sequence generation belongs in:
//!
//! `generators/clifford.rs`
//!
//! This separation prevents protocol-specific assumptions from leaking into
//! the deterministic foundation.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - `core::circuit`;
//! - `core::limits`;
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
//! Future generator modules can consume this module's public configuration
//! and helper types without changing this file's public contract.
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
//! No external dependencies are required.

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

use crate::quantum::ir::{
    Gate,
    GateKind,
    QuantumCircuit,
    QubitId,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable deterministic-generator algorithm identifier.
///
/// This identifier belongs in benchmark provenance. A semantically
/// incompatible change to the generation algorithm requires a new identifier.
pub const DETERMINISTIC_GENERATOR_ALGORITHM_ID: &str =
    "zamani.benchmarking.deterministic.v1";

/// Current deterministic-generator revision.
///
/// This is deliberately separate from the Quantum IR version.
pub const DETERMINISTIC_GENERATOR_REVISION: u32 = 1;

/// Maximum identifier length accepted by this generator.
///
/// Benchmark identifiers are already validated by `BenchmarkCircuit`, but
/// keeping a local bounded input protects allocation before descriptor
/// construction.
pub const MAX_IDENTIFIER_LENGTH: usize = 4_096;

// =============================================================================
// Deterministic single-qubit patterns
// =============================================================================

/// Deterministic single-qubit gate pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterministicSingleQubitGate {
    /// Identity.
    Identity,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,

    /// Hadamard.
    H,

    /// S gate.
    S,

    /// S-dagger gate.
    Sdg,

    /// T gate.
    T,

    /// T-dagger gate.
    Tdg,
}

impl Default for DeterministicSingleQubitGate {
    fn default() -> Self {
        Self::H
    }
}

impl DeterministicSingleQubitGate {
    /// Converts this deterministic pattern into its canonical IR gate kind.
    #[must_use]
    pub const fn gate_kind(self) -> GateKind {
        match self {
            Self::Identity => GateKind::I,
            Self::X => GateKind::X,
            Self::Y => GateKind::Y,
            Self::Z => GateKind::Z,
            Self::H => GateKind::H,
            Self::S => GateKind::S,
            Self::Sdg => GateKind::Sdg,
            Self::T => GateKind::T,
            Self::Tdg => GateKind::Tdg,
        }
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::H => "h",
            Self::S => "s",
            Self::Sdg => "sdg",
            Self::T => "t",
            Self::Tdg => "tdg",
        }
    }
}

impl fmt::Display for DeterministicSingleQubitGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Deterministic entangling patterns
// =============================================================================

/// Deterministic two-qubit gate pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterministicTwoQubitGate {
    /// Controlled-X.
    CX,

    /// Controlled-Z.
    CZ,

    /// SWAP.
    SWAP,

    /// iSWAP.
    ISWAP,
}

impl Default for DeterministicTwoQubitGate {
    fn default() -> Self {
        Self::CX
    }
}

impl DeterministicTwoQubitGate {
    /// Converts this deterministic pattern into its canonical IR gate kind.
    #[must_use]
    pub const fn gate_kind(self) -> GateKind {
        match self {
            Self::CX => GateKind::CX,
            Self::CZ => GateKind::CZ,
            Self::SWAP => GateKind::SWAP,
            Self::ISWAP => GateKind::ISWAP,
        }
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::CX => "cx",
            Self::CZ => "cz",
            Self::SWAP => "swap",
            Self::ISWAP => "iswap",
        }
    }
}

impl fmt::Display for DeterministicTwoQubitGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Layer strategy
// =============================================================================

/// Strategy used to construct logical layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterministicLayerStrategy {
    /// Every layer contains only the configured single-qubit operation.
    SingleQubit,

    /// Every layer contains disjoint two-qubit operations.
    ///
    /// If fewer than two qubits exist, the generator falls back to a
    /// single-qubit layer because a two-qubit layer cannot be represented.
    TwoQubit,

    /// Odd-numbered layers are single-qubit layers and even-numbered layers
    /// are two-qubit layers.
    Alternating,
}

impl Default for DeterministicLayerStrategy {
    fn default() -> Self {
        Self::Alternating
    }
}

impl DeterministicLayerStrategy {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SingleQubit => "single_qubit",
            Self::TwoQubit => "two_qubit",
            Self::Alternating => "alternating",
        }
    }
}

impl fmt::Display for DeterministicLayerStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Generator configuration
// =============================================================================

/// Complete configuration for deterministic circuit generation.
///
/// The configuration is intentionally independent of backend topology and
/// execution configuration.
///
/// `qubits` and `depth` describe the logical workload. Hardware compilation,
/// routing, and scheduling happen later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicCircuitConfig {
    /// Number of logical qubits.
    qubits: usize,

    /// Number of logical circuit layers.
    depth: usize,

    /// Single-qubit gate pattern.
    single_qubit_gate: DeterministicSingleQubitGate,

    /// Two-qubit gate pattern.
    two_qubit_gate: DeterministicTwoQubitGate,

    /// Logical layer construction strategy.
    layer_strategy: DeterministicLayerStrategy,

    /// Benchmark identifier.
    benchmark_id: String,

    /// Experiment identifier.
    experiment_id: String,

    /// Optional benchmark case identifier.
    case_id: Option<String>,

    /// Benchmark circuit role.
    role: BenchmarkCircuitRole,

    /// Generation seed retained for provenance/replay compatibility.
    ///
    /// The deterministic algorithm itself does not use this value to select
    /// operations.
    seed: u64,

    /// Whether downstream analysis requires an ideal reference distribution.
    requires_ideal_reference: bool,
}

impl DeterministicCircuitConfig {
    /// Creates a deterministic circuit configuration.
    ///
    /// The configuration uses:
    ///
    /// - Hadamard single-qubit gates;
    /// - controlled-X entangling gates;
    /// - alternating layers;
    /// - generic benchmark metadata;
    /// - seed `0`.
    pub fn new(
        qubits: usize,
        depth: usize,
    ) -> Result<Self, DeterministicGeneratorError> {
        Self::builder(qubits, depth)
            .build()
    }

    /// Starts construction of a deterministic configuration.
    pub fn builder(
        qubits: usize,
        depth: usize,
    ) -> DeterministicCircuitConfigBuilder {
        DeterministicCircuitConfigBuilder {
            qubits,
            depth,
            single_qubit_gate:
                DeterministicSingleQubitGate::default(),
            two_qubit_gate:
                DeterministicTwoQubitGate::default(),
            layer_strategy:
                DeterministicLayerStrategy::default(),
            benchmark_id:
                "deterministic".to_owned(),
            experiment_id:
                "deterministic".to_owned(),
            case_id: None,
            role:
                BenchmarkCircuitRole::Generic,
            seed: 0,
            requires_ideal_reference: false,
        }
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the requested logical depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the configured single-qubit gate.
    #[must_use]
    pub const fn single_qubit_gate(
        &self,
    ) -> DeterministicSingleQubitGate {
        self.single_qubit_gate
    }

    /// Returns the configured two-qubit gate.
    #[must_use]
    pub const fn two_qubit_gate(
        &self,
    ) -> DeterministicTwoQubitGate {
        self.two_qubit_gate
    }

    /// Returns the layer strategy.
    #[must_use]
    pub const fn layer_strategy(
        &self,
    ) -> DeterministicLayerStrategy {
        self.layer_strategy
    }

    /// Returns the benchmark identifier.
    #[must_use]
    pub fn benchmark_id(&self) -> &str {
        &self.benchmark_id
    }

    /// Returns the experiment identifier.
    #[must_use]
    pub fn experiment_id(&self) -> &str {
        &self.experiment_id
    }

    /// Returns the optional case identifier.
    #[must_use]
    pub fn case_id(&self) -> Option<&str> {
        self.case_id.as_deref()
    }

    /// Returns the circuit role.
    #[must_use]
    pub const fn role(&self) -> BenchmarkCircuitRole {
        self.role
    }

    /// Returns the generation seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns whether an ideal reference is required.
    #[must_use]
    pub const fn requires_ideal_reference(&self) -> bool {
        self.requires_ideal_reference
    }

    /// Validates this configuration against production limits.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), DeterministicGeneratorError> {
        limits
            .validate()
            .map_err(DeterministicGeneratorError::Limit)?;

        if self.qubits == 0 {
            return Err(
                DeterministicGeneratorError::InvalidConfiguration {
                    field: "qubits",
                    reason: "must be greater than zero",
                },
            );
        }

        if self.depth == 0 {
            return Err(
                DeterministicGeneratorError::InvalidConfiguration {
                    field: "depth",
                    reason: "must be greater than zero",
                },
            );
        }

        if self.benchmark_id.is_empty()
            || self.benchmark_id.len() > MAX_IDENTIFIER_LENGTH
        {
            return Err(
                DeterministicGeneratorError::InvalidConfiguration {
                    field: "benchmark_id",
                    reason:
                        "must be non-empty and within the identifier size limit",
                },
            );
        }

        if self.experiment_id.is_empty()
            || self.experiment_id.len() > MAX_IDENTIFIER_LENGTH
        {
            return Err(
                DeterministicGeneratorError::InvalidConfiguration {
                    field: "experiment_id",
                    reason:
                        "must be non-empty and within the identifier size limit",
                },
            );
        }

        if let Some(case_id) = &self.case_id {
            if case_id.is_empty()
                || case_id.len() > MAX_IDENTIFIER_LENGTH
            {
                return Err(
                    DeterministicGeneratorError::InvalidConfiguration {
                        field: "case_id",
                        reason:
                            "must be non-empty and within the identifier size limit",
                    },
                );
            }
        }

        limits
            .check_qubits(self.qubits)
            .map_err(DeterministicGeneratorError::Limit)?;

        limits
            .check_circuit_depth(self.depth)
            .map_err(DeterministicGeneratorError::Limit)?;

        let estimated_operations = self
            .qubits
            .checked_mul(self.depth)
            .ok_or(
                DeterministicGeneratorError::ArithmeticOverflow {
                    operation: "qubits × depth",
                },
            )?;

        limits
            .check_gate_count(estimated_operations)
            .map_err(DeterministicGeneratorError::Limit)?;

        Ok(())
    }
}

// =============================================================================
// Configuration builder
// =============================================================================

/// Builder for [`DeterministicCircuitConfig`].
///
/// The builder allows all configuration to be finalized before any circuit
/// allocation occurs.
#[derive(Debug, Clone)]
pub struct DeterministicCircuitConfigBuilder {
    qubits: usize,
    depth: usize,
    single_qubit_gate: DeterministicSingleQubitGate,
    two_qubit_gate: DeterministicTwoQubitGate,
    layer_strategy: DeterministicLayerStrategy,
    benchmark_id: String,
    experiment_id: String,
    case_id: Option<String>,
    role: BenchmarkCircuitRole,
    seed: u64,
    requires_ideal_reference: bool,
}

impl DeterministicCircuitConfigBuilder {
    /// Sets the single-qubit gate.
    #[must_use]
    pub const fn single_qubit_gate(
        mut self,
        gate: DeterministicSingleQubitGate,
    ) -> Self {
        self.single_qubit_gate = gate;
        self
    }

    /// Sets the two-qubit gate.
    #[must_use]
    pub const fn two_qubit_gate(
        mut self,
        gate: DeterministicTwoQubitGate,
    ) -> Self {
        self.two_qubit_gate = gate;
        self
    }

    /// Sets the layer strategy.
    #[must_use]
    pub const fn layer_strategy(
        mut self,
        strategy: DeterministicLayerStrategy,
    ) -> Self {
        self.layer_strategy = strategy;
        self
    }

    /// Sets the benchmark identifier.
    pub fn benchmark_id(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.benchmark_id = value.into();
        self
    }

    /// Sets the experiment identifier.
    pub fn experiment_id(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.experiment_id = value.into();
        self
    }

    /// Sets an optional case identifier.
    pub fn case_id(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.case_id = Some(value.into());
        self
    }

    /// Sets the benchmark circuit role.
    #[must_use]
    pub const fn role(
        mut self,
        role: BenchmarkCircuitRole,
    ) -> Self {
        self.role = role;
        self
    }

    /// Sets the provenance/replay seed.
    #[must_use]
    pub const fn seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.seed = seed;
        self
    }

    /// Indicates that downstream analysis requires an ideal reference.
    #[must_use]
    pub const fn requires_ideal_reference(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_ideal_reference = required;
        self
    }

    /// Validates and creates the immutable configuration.
    pub fn build(
        self,
    ) -> Result<DeterministicCircuitConfig, DeterministicGeneratorError> {
        let config = DeterministicCircuitConfig {
            qubits: self.qubits,
            depth: self.depth,
            single_qubit_gate: self.single_qubit_gate,
            two_qubit_gate: self.two_qubit_gate,
            layer_strategy: self.layer_strategy,
            benchmark_id: self.benchmark_id,
            experiment_id: self.experiment_id,
            case_id: self.case_id,
            role: self.role,
            seed: self.seed,
            requires_ideal_reference:
                self.requires_ideal_reference,
        };

        config.validate(&BenchmarkLimits::production())?;

        Ok(config)
    }
}

// =============================================================================
// Generator errors
// =============================================================================

/// Errors produced by deterministic circuit generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeterministicGeneratorError {
    /// A configuration field is invalid.
    InvalidConfiguration {
        /// Configuration field.
        field: &'static str,

        /// Static explanation.
        reason: &'static str,
    },

    /// A production resource limit was exceeded.
    Limit(LimitError),

    /// An arithmetic operation required for safe generation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        operation: &'static str,
    },

    /// Circuit construction through canonical IR failed.
    CircuitConstruction(String),

    /// Benchmark-circuit wrapping failed.
    BenchmarkCircuit(String),

    /// The requested strategy cannot produce a valid operation for the
    /// configured circuit.
    UnsupportedConfiguration {
        /// Static explanation.
        reason: &'static str,
    },
}

impl fmt::Display for DeterministicGeneratorError {
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
                    "invalid deterministic generator configuration \
                     `{field}`: {reason}"
                )
            }

            Self::Limit(error) => {
                write!(
                    f,
                    "deterministic generator resource limit: {error}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "deterministic circuit generation arithmetic \
                     overflow: {operation}"
                )
            }

            Self::CircuitConstruction(message) => {
                write!(
                    f,
                    "deterministic circuit construction failed: {message}"
                )
            }

            Self::BenchmarkCircuit(message) => {
                write!(
                    f,
                    "deterministic benchmark-circuit construction \
                     failed: {message}"
                )
            }

            Self::UnsupportedConfiguration { reason } => {
                write!(
                    f,
                    "unsupported deterministic generator \
                     configuration: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for DeterministicGeneratorError {}

impl From<LimitError> for DeterministicGeneratorError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

impl From<BenchmarkCircuitError> for DeterministicGeneratorError {
    fn from(error: BenchmarkCircuitError) -> Self {
        Self::BenchmarkCircuit(error.to_string())
    }
}

// =============================================================================
// Generation statistics
// =============================================================================

/// Exact generation summary.
///
/// This is calculated from the generated circuit rather than estimated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicGenerationSummary {
    /// Logical qubits.
    pub qubits: usize,

    /// Requested logical depth.
    pub requested_depth: usize,

    /// Number of generated operations.
    pub operations: usize,

    /// Number of generated single-qubit operations.
    pub single_qubit_operations: usize,

    /// Number of generated two-qubit operations.
    pub two_qubit_operations: usize,

    /// Number of logical layers generated.
    pub layers: usize,
}

impl DeterministicGenerationSummary {
    /// Returns the number of generated operations.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }

    /// Returns whether the generated workload contains entangling gates.
    #[must_use]
    pub const fn contains_two_qubit_operations(self) -> bool {
        self.two_qubit_operations != 0
    }
}

// =============================================================================
// Deterministic generator
// =============================================================================

/// Production deterministic circuit generator.
///
/// The generator is immutable and side-effect free.
///
/// It does not contain mutable random state. The same configuration produces
/// the same logical circuit.
#[derive(Debug, Clone, Copy, Default)]
pub struct DeterministicCircuitGenerator;

impl DeterministicCircuitGenerator {
    /// Creates a deterministic generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates a canonical logical Quantum IR circuit.
    ///
    /// This method does not wrap the circuit in benchmarking metadata.
    pub fn generate_circuit(
        &self,
        config: &DeterministicCircuitConfig,
    ) -> Result<QuantumCircuit, DeterministicGeneratorError> {
        self.generate_circuit_with_limits(
            config,
            BenchmarkLimits::production(),
        )
    }

    /// Generates a canonical logical Quantum IR circuit under explicit
    /// benchmark resource limits.
    pub fn generate_circuit_with_limits(
        &self,
        config: &DeterministicCircuitConfig,
        limits: BenchmarkLimits,
    ) -> Result<QuantumCircuit, DeterministicGeneratorError> {
        config.validate(&limits)?;

        let estimated_operations = config
            .qubits
            .checked_mul(config.depth)
            .ok_or(
                DeterministicGeneratorError::ArithmeticOverflow {
                    operation: "qubits × depth",
                },
            )?;

        limits
            .check_gate_count(estimated_operations)
            .map_err(DeterministicGeneratorError::Limit)?;

        let mut operations =
            Vec::with_capacity(estimated_operations);

        for layer_index in 0..config.depth {
            self.generate_layer(
                config,
                layer_index,
                &mut operations,
                &limits,
            )?;
        }

        limits
            .check_gate_count(operations.len())
            .map_err(DeterministicGeneratorError::Limit)?;

        QuantumCircuit::from_operations(
            config.qubits,
            0,
            operations,
        )
        .map_err(|error| {
            DeterministicGeneratorError::CircuitConstruction(
                error.to_string(),
            )
        })
    }

    /// Generates a complete benchmarking-layer circuit.
    ///
    /// The returned object is the type downstream execution and analysis
    /// modules should consume.
    pub fn generate(
        &self,
        config: &DeterministicCircuitConfig,
    ) -> Result<BenchmarkCircuit, DeterministicGeneratorError> {
        self.generate_with_limits(
            config,
            BenchmarkLimits::production(),
        )
    }

    /// Generates a complete benchmarking-layer circuit using an explicit
    /// resource policy.
    pub fn generate_with_limits(
        &self,
        config: &DeterministicCircuitConfig,
        limits: BenchmarkLimits,
    ) -> Result<BenchmarkCircuit, DeterministicGeneratorError> {
        config.validate(&limits)?;

        let circuit =
            self.generate_circuit_with_limits(config, limits)?;

        let generation =
            BenchmarkCircuitGeneration::new(
                config.seed,
                0,
                DETERMINISTIC_GENERATOR_REVISION,
            );

        let descriptor =
            BenchmarkCircuitDescriptor::with_case_id(
                config.benchmark_id.clone(),
                config.experiment_id.clone(),
                config.case_id.clone(),
                config.role,
                generation,
            )?
            .with_ideal_reference(
                config.requires_ideal_reference,
            );

        BenchmarkCircuit::new(circuit, descriptor)
            .map_err(DeterministicGeneratorError::from)
    }

    /// Generates a deterministic circuit and returns its exact generation
    /// summary.
    pub fn generate_with_summary(
        &self,
        config: &DeterministicCircuitConfig,
    ) -> Result<
        (
            BenchmarkCircuit,
            DeterministicGenerationSummary,
        ),
        DeterministicGeneratorError,
    > {
        let benchmark_circuit = self.generate(config)?;

        let resources = benchmark_circuit.resources();

        let summary = DeterministicGenerationSummary {
            qubits: resources.qubits(),
            requested_depth: config.depth,
            operations: resources.operations(),
            single_qubit_operations:
                resources.single_qubit_gates(),
            two_qubit_operations:
                resources.two_qubit_gates(),
            layers: config.depth,
        };

        Ok((benchmark_circuit, summary))
    }

    // -------------------------------------------------------------------------
    // Layer generation
    // -------------------------------------------------------------------------

    fn generate_layer(
        &self,
        config: &DeterministicCircuitConfig,
        layer_index: usize,
        operations: &mut Vec<Gate>,
        limits: &BenchmarkLimits,
    ) -> Result<(), DeterministicGeneratorError> {
        let use_two_qubit_layer =
            match config.layer_strategy {
                DeterministicLayerStrategy::SingleQubit => false,

                DeterministicLayerStrategy::TwoQubit => {
                    config.qubits >= 2
                }

                DeterministicLayerStrategy::Alternating => {
                    config.qubits >= 2
                        && layer_index % 2 == 1
                }
            };

        if use_two_qubit_layer {
            self.generate_two_qubit_layer(
                config,
                operations,
                limits,
            )
        } else {
            self.generate_single_qubit_layer(
                config,
                operations,
                limits,
            )
        }
    }

    fn generate_single_qubit_layer(
        &self,
        config: &DeterministicCircuitConfig,
        operations: &mut Vec<Gate>,
        limits: &BenchmarkLimits,
    ) -> Result<(), DeterministicGeneratorError> {
        let gate_kind =
            config.single_qubit_gate.gate_kind();

        limits
            .check_gate_count(
                operations.len().saturating_add(
                    config.qubits,
                ),
            )
            .map_err(DeterministicGeneratorError::Limit)?;

        for qubit_index in 0..config.qubits {
            let gate = Gate::new(
                gate_kind,
                vec![QubitId::new(qubit_index)],
                Vec::new(),
                None,
                None,
            )
            .map_err(|error| {
                DeterministicGeneratorError::CircuitConstruction(
                    error.to_string(),
                )
            })?;

            operations.push(gate);
        }

        Ok(())
    }

    fn generate_two_qubit_layer(
        &self,
        config: &DeterministicCircuitConfig,
        operations: &mut Vec<Gate>,
        limits: &BenchmarkLimits,
    ) -> Result<(), DeterministicGeneratorError> {
        let pair_count = config.qubits / 2;

        limits
            .check_gate_count(
                operations.len().saturating_add(pair_count),
            )
            .map_err(DeterministicGeneratorError::Limit)?;

        let gate_kind =
            config.two_qubit_gate.gate_kind();

        for pair_index in 0..pair_count {
            let first = pair_index
                .checked_mul(2)
                .ok_or(
                    DeterministicGeneratorError::ArithmeticOverflow {
                        operation:
                            "pair index × 2",
                    },
                )?;

            let second = first.checked_add(1).ok_or(
                DeterministicGeneratorError::ArithmeticOverflow {
                    operation:
                        "pair first index + 1",
                },
            )?;

            let gate = Gate::new(
                gate_kind,
                vec![
                    QubitId::new(first),
                    QubitId::new(second),
                ],
                Vec::new(),
                None,
                None,
            )
            .map_err(|error| {
                DeterministicGeneratorError::CircuitConstruction(
                    error.to_string(),
                )
            })?;

            operations.push(gate);
        }

        Ok(())
    }
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Generates a deterministic logical Quantum IR circuit.
///
/// This is the simplest entry point for compiler/test code that does not need
/// a benchmarking metadata wrapper.
pub fn generate_deterministic_circuit(
    qubits: usize,
    depth: usize,
) -> Result<QuantumCircuit, DeterministicGeneratorError> {
    let config =
        DeterministicCircuitConfig::new(qubits, depth)?;

    DeterministicCircuitGenerator::new()
        .generate_circuit(&config)
}

/// Generates a deterministic benchmarking circuit using the default
/// configuration.
pub fn generate_deterministic_benchmark_circuit(
    qubits: usize,
    depth: usize,
) -> Result<BenchmarkCircuit, DeterministicGeneratorError> {
    let config =
        DeterministicCircuitConfig::new(qubits, depth)?;

    DeterministicCircuitGenerator::new()
        .generate(&config)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn config(
        qubits: usize,
        depth: usize,
    ) -> DeterministicCircuitConfig {
        DeterministicCircuitConfig::builder(
            qubits,
            depth,
        )
        .benchmark_id("test")
        .experiment_id("deterministic")
        .build()
        .expect("test configuration must be valid")
    }

    #[test]
    fn identical_configurations_produce_identical_circuits() {
        let generator =
            DeterministicCircuitGenerator::new();

        let first = generator
            .generate_circuit(&config(4, 6))
            .expect("first generation must succeed");

        let second = generator
            .generate_circuit(&config(4, 6))
            .expect("second generation must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn identical_configurations_produce_identical_benchmark_circuits() {
        let generator =
            DeterministicCircuitGenerator::new();

        let first = generator
            .generate(&config(4, 6))
            .expect("first generation must succeed");

        let second = generator
            .generate(&config(4, 6))
            .expect("second generation must succeed");

        assert_eq!(first, second);

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn different_depths_produce_different_circuits() {
        let generator =
            DeterministicCircuitGenerator::new();

        let first = generator
            .generate_circuit(&config(4, 4))
            .expect("first generation must succeed");

        let second = generator
            .generate_circuit(&config(4, 5))
            .expect("second generation must succeed");

        assert_ne!(first, second);
    }

    #[test]
    fn different_widths_produce_different_circuits() {
        let generator =
            DeterministicCircuitGenerator::new();

        let first = generator
            .generate_circuit(&config(4, 4))
            .expect("first generation must succeed");

        let second = generator
            .generate_circuit(&config(5, 4))
            .expect("second generation must succeed");

        assert_ne!(first, second);
    }

    #[test]
    fn generated_circuit_is_canonically_valid() {
        let generator =
            DeterministicCircuitGenerator::new();

        let circuit = generator
            .generate_circuit(&config(8, 10))
            .expect("generation must succeed");

        assert!(
            crate::quantum::ir::validate_circuit(
                &circuit
            )
            .is_ok()
        );
    }

    #[test]
    fn single_qubit_strategy_generates_expected_operations() {
        let configuration =
            DeterministicCircuitConfig::builder(4, 3)
                .single_qubit_gate(
                    DeterministicSingleQubitGate::X,
                )
                .layer_strategy(
                    DeterministicLayerStrategy::SingleQubit,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        let statistics =
            crate::quantum::ir::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            statistics.qubits(),
            4
        );

        assert_eq!(
            statistics.operation_count(),
            12
        );

        assert_eq!(
            statistics.single_qubit_operations(),
            12
        );

        assert_eq!(
            statistics.two_qubit_operations(),
            0
        );
    }

    #[test]
    fn two_qubit_strategy_generates_disjoint_pairs() {
        let configuration =
            DeterministicCircuitConfig::builder(6, 4)
                .two_qubit_gate(
                    DeterministicTwoQubitGate::CZ,
                )
                .layer_strategy(
                    DeterministicLayerStrategy::TwoQubit,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        let statistics =
            crate::quantum::ir::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            statistics.operation_count(),
            12
        );

        assert_eq!(
            statistics.two_qubit_operations(),
            12
        );

        assert_eq!(
            statistics.single_qubit_operations(),
            0
        );
    }

    #[test]
    fn odd_qubit_count_leaves_one_qubit_idle_in_entangling_layers() {
        let configuration =
            DeterministicCircuitConfig::builder(5, 2)
                .layer_strategy(
                    DeterministicLayerStrategy::TwoQubit,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        let statistics =
            crate::quantum::ir::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            statistics.two_qubit_operations(),
            4
        );

        assert_eq!(
            statistics.operation_count(),
            4
        );
    }

    #[test]
    fn one_qubit_two_qubit_strategy_falls_back_to_single_qubit() {
        let configuration =
            DeterministicCircuitConfig::builder(1, 4)
                .layer_strategy(
                    DeterministicLayerStrategy::TwoQubit,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        let statistics =
            crate::quantum::ir::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            statistics.operation_count(),
            4
        );

        assert_eq!(
            statistics.single_qubit_operations(),
            4
        );
    }

    #[test]
    fn alternating_strategy_contains_both_gate_classes() {
        let configuration =
            DeterministicCircuitConfig::builder(4, 4)
                .layer_strategy(
                    DeterministicLayerStrategy::Alternating,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        let statistics =
            crate::quantum::ir::analyze(&circuit)
                .expect("analysis must succeed");

        assert_eq!(
            statistics.single_qubit_operations(),
            8
        );

        assert_eq!(
            statistics.two_qubit_operations(),
            4
        );
    }

    #[test]
    fn seed_is_metadata_and_does_not_change_deterministic_structure() {
        let first =
            DeterministicCircuitConfig::builder(4, 4)
                .seed(1)
                .build()
                .expect("configuration must be valid");

        let second =
            DeterministicCircuitConfig::builder(4, 4)
                .seed(2)
                .build()
                .expect("configuration must be valid");

        let generator =
            DeterministicCircuitGenerator::new();

        let first_circuit = generator
            .generate_circuit(&first)
            .expect("generation must succeed");

        let second_circuit = generator
            .generate_circuit(&second)
            .expect("generation must succeed");

        assert_eq!(
            first_circuit,
            second_circuit
        );
    }

    #[test]
    fn seed_is_preserved_in_benchmark_metadata() {
        let configuration =
            DeterministicCircuitConfig::builder(4, 4)
                .seed(42)
                .build()
                .expect("configuration must be valid");

        let benchmark =
            DeterministicCircuitGenerator::new()
                .generate(&configuration)
                .expect("generation must succeed");

        assert_eq!(
            benchmark.generation().seed(),
            42
        );

        assert_eq!(
            benchmark.generation().generator_revision(),
            DETERMINISTIC_GENERATOR_REVISION
        );
    }

    #[test]
    fn ideal_reference_requirement_is_preserved() {
        let configuration =
            DeterministicCircuitConfig::builder(3, 3)
                .requires_ideal_reference(true)
                .build()
                .expect("configuration must be valid");

        let benchmark =
            DeterministicCircuitGenerator::new()
                .generate(&configuration)
                .expect("generation must succeed");

        assert!(
            benchmark
                .descriptor()
                .requires_ideal_reference()
        );
    }

    #[test]
    fn configured_gate_patterns_are_respected() {
        let configuration =
            DeterministicCircuitConfig::builder(3, 2)
                .single_qubit_gate(
                    DeterministicSingleQubitGate::Z,
                )
                .layer_strategy(
                    DeterministicLayerStrategy::SingleQubit,
                )
                .build()
                .expect("configuration must be valid");

        let circuit =
            DeterministicCircuitGenerator::new()
                .generate_circuit(&configuration)
                .expect("generation must succeed");

        for operation in circuit.operations() {
            assert_eq!(
                operation.kind(),
                GateKind::Z
            );
        }
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let result =
            DeterministicCircuitConfig::new(0, 1);

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn zero_depth_is_rejected() {
        let result =
            DeterministicCircuitConfig::new(1, 0);

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn excessive_workload_is_rejected_before_generation() {
        let limits =
            BenchmarkLimits::production()
                .with_max_qubits(4)
                .with_max_circuit_depth(4)
                .with_max_gate_count(8);

        let configuration =
            DeterministicCircuitConfig::builder(4, 4)
                .build()
                .expect("production configuration must initially be valid");

        let result =
            DeterministicCircuitGenerator::new()
                .generate_circuit_with_limits(
                    &configuration,
                    limits,
                );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn summary_matches_generated_resources() {
        let configuration =
            DeterministicCircuitConfig::builder(6, 5)
                .build()
                .expect("configuration must be valid");

        let generator =
            DeterministicCircuitGenerator::new();

        let (benchmark, summary) =
            generator
                .generate_with_summary(&configuration)
                .expect("generation must succeed");

        let resources =
            benchmark.resources();

        assert_eq!(
            summary.qubits,
            resources.qubits()
        );

        assert_eq!(
            summary.operations,
            resources.operations()
        );

        assert_eq!(
            summary.single_qubit_operations,
            resources.single_qubit_gates()
        );

        assert_eq!(
            summary.two_qubit_operations,
            resources.two_qubit_gates()
        );

        assert_eq!(
            summary.requested_depth,
            configuration.depth()
        );
    }

    #[test]
    fn generated_benchmark_has_generation_metadata() {
        let configuration =
            DeterministicCircuitConfig::builder(4, 4)
                .benchmark_id("deterministic_test")
                .experiment_id("experiment_001")
                .case_id("case_001")
                .seed(1234)
                .role(
                    BenchmarkCircuitRole::Trial,
                )
                .build()
                .expect("configuration must be valid");

        let benchmark =
            DeterministicCircuitGenerator::new()
                .generate(&configuration)
                .expect("generation must succeed");

        assert_eq!(
            benchmark.benchmark_id(),
            "deterministic_test"
        );

        assert_eq!(
            benchmark.experiment_id(),
            "experiment_001"
        );

        assert_eq!(
            benchmark.case_id(),
            Some("case_001")
        );

        assert_eq!(
            benchmark.role(),
            BenchmarkCircuitRole::Trial
        );

        assert_eq!(
            benchmark.generation().seed(),
            1234
        );
    }
}