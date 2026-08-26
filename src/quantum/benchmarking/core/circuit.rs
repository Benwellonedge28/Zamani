//! Zamani Quantum Benchmarking — Benchmark Circuit Contract
//!
//! This module defines the canonical circuit object consumed by the quantum
//! benchmarking subsystem.
//!
//! # Architectural boundary
//!
//! `BenchmarkCircuit` is a benchmarking view/container around Zamani's
//! canonical [`crate::quantum::ir::QuantumCircuit`].
//!
//! It deliberately does NOT:
//!
//! - generate random benchmark circuits;
//! - execute circuits;
//! - select a hardware backend;
//! - perform routing;
//! - perform scheduling;
//! - perform optimization;
//! - perform calibration;
//! - calculate benchmark statistics;
//! - calculate fidelity/error metrics;
//! - submit work to a QPU;
//! - contain simulator state;
//! - contain physical-qubit mappings;
//! - contain backend-specific metadata.
//!
//! Those responsibilities belong to the corresponding benchmarking,
//! compiler, runtime, and hardware subsystems.
//!
//! # Why this wrapper exists
//!
//! Benchmarking needs information that is broader than the logical circuit
//! itself, such as:
//!
//! - benchmark identity;
//! - experiment/case identity;
//! - deterministic generation seed;
//! - generation sequence;
//! - benchmark width/depth;
//! - logical circuit statistics;
//! - a stable circuit fingerprint;
//! - whether the circuit is expected to be exactly reproducible;
//! - whether an ideal reference distribution is required.
//!
//! Those concerns must not be added to `quantum::ir::QuantumCircuit`, because
//! the Quantum IR is the canonical representation of a logical quantum
//! program and must remain independent of benchmarking.
//!
//! Therefore the dependency direction is:
//!
//! ```text
//! quantum::ir::QuantumCircuit
//!          │
//!          ▼
//! benchmarking::core::BenchmarkCircuit
//!          │
//!          ├── generators
//!          ├── execution
//!          ├── statistics
//!          ├── metrics
//!          ├── protocols
//!          └── reporting
//! ```
//!
//! Never:
//!
//! ```text
//! quantum::ir → benchmarking
//! ```
//!
//! # Reproducibility
//!
//! A benchmark circuit can be reproduced when its source circuit, generation
//! metadata, and generator contract are reproduced.
//!
//! This module therefore provides a deterministic structural fingerprint.
//!
//! The fingerprint is deliberately:
//!
//! - deterministic;
//! - allocation-light;
//! - independent of hash-map iteration order;
//! - independent of process address space;
//! - independent of randomized `DefaultHasher` behavior;
//! - suitable for equality/regression/provenance checks.
//!
//! It is NOT intended to be a cryptographic hash. Cryptographic result
//! integrity belongs to the future provenance/result layer.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//!
//! # Integration contract
//!
//! This file depends only on the existing public Quantum IR API:
//!
//! - `crate::quantum::ir::QuantumCircuit`;
//! - `crate::quantum::ir::CircuitError`;
//! - `crate::quantum::ir::Gate`;
//! - `crate::quantum::ir::GateKind`;
//! - `crate::quantum::ir::CircuitStatistics`;
//! - `crate::quantum::ir::analyze`;
//! - `crate::quantum::ir::validate_circuit`.
//!
//! Future benchmarking files should depend on `BenchmarkCircuit`, not the
//! other way around.
//!
//! In particular:
//!
//! ```text
//! generators/*
//!      ↓
//! BenchmarkCircuit
//!      ↓
//! execution/*
//!      ↓
//! observation/result
//!      ↓
//! protocols/*
//! ```
//!
//! This allows circuit generation, execution, analysis, and statistical
//! processing to remain independently testable.

use std::fmt;

use crate::quantum::ir::{
    analyze,
    validate_circuit,
    CircuitError,
    CircuitId,
    CircuitStatistics,
    Gate,
    GateKind,
    QuantumCircuit,
};

// =============================================================================
// Constants
// =============================================================================

/// Current benchmark-circuit metadata schema version.
///
/// This is separate from the Quantum IR version because changing benchmark
/// metadata must not change the semantic version of the Quantum IR itself.
pub const BENCHMARK_CIRCUIT_SCHEMA_VERSION: u16 = 1;

/// Initial value for the deterministic structural fingerprint.
///
/// FNV-1a is used here only as a deterministic non-cryptographic fingerprint.
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

/// FNV-1a multiplication constant.
const FNV_PRIME: u64 = 0x100000001b3;

// =============================================================================
// Circuit identity
// =============================================================================

/// Logical role of a circuit inside a benchmark experiment.
///
/// The role is descriptive metadata. It does not affect execution semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkCircuitRole {
    /// Generic benchmark circuit.
    Generic,

    /// Reference circuit used as a control/baseline.
    Reference,

    /// Trial circuit whose result is being measured.
    Trial,

    /// Random circuit used by randomized-circuit protocols.
    Random,

    /// Mirror circuit used for mirror-circuit benchmarking.
    Mirror,

    /// Circuit belonging to an application workload.
    Application,

    /// Circuit used for characterization of a physical/logical operation.
    Characterization,

    /// Circuit used by a QEC experiment.
    ErrorCorrection,

    /// Circuit representing a logical operation/workload.
    Logical,
}

impl Default for BenchmarkCircuitRole {
    fn default() -> Self {
        Self::Generic
    }
}

impl fmt::Display for BenchmarkCircuitRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Generic => "generic",
            Self::Reference => "reference",
            Self::Trial => "trial",
            Self::Random => "random",
            Self::Mirror => "mirror",
            Self::Application => "application",
            Self::Characterization => "characterization",
            Self::ErrorCorrection => "error_correction",
            Self::Logical => "logical",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Generation metadata
// =============================================================================

/// Deterministic generation metadata attached to a benchmark circuit.
///
/// This object records *how the benchmark circuit was selected/generated* but
/// does not contain the generator implementation itself.
///
/// The generator implementation and its version belong to the future
/// provenance layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkCircuitGeneration {
    /// User/experiment supplied deterministic seed.
    seed: u64,

    /// Zero-based circuit position within the generated experiment.
    sequence_index: u64,

    /// Optional generator revision identifier.
    ///
    /// `0` means that the caller did not provide a generator revision.
    generator_revision: u32,
}

impl BenchmarkCircuitGeneration {
    /// Creates deterministic generation metadata.
    pub const fn new(
        seed: u64,
        sequence_index: u64,
        generator_revision: u32,
    ) -> Self {
        Self {
            seed,
            sequence_index,
            generator_revision,
        }
    }

    /// Returns the generation seed.
    #[must_use]
    pub const fn seed(self) -> u64 {
        self.seed
    }

    /// Returns the sequence position.
    #[must_use]
    pub const fn sequence_index(self) -> u64 {
        self.sequence_index
    }

    /// Returns the generator revision.
    #[must_use]
    pub const fn generator_revision(self) -> u32 {
        self.generator_revision
    }
}

impl Default for BenchmarkCircuitGeneration {
    fn default() -> Self {
        Self {
            seed: 0,
            sequence_index: 0,
            generator_revision: 0,
        }
    }
}

// =============================================================================
// Benchmark circuit descriptor
// =============================================================================

/// Immutable descriptive metadata for a benchmark circuit.
///
/// This metadata does not duplicate the complete circuit. The actual circuit
/// remains owned by `QuantumCircuit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkCircuitDescriptor {
    /// Benchmark protocol identifier.
    benchmark_id: String,

    /// Experiment identifier.
    experiment_id: String,

    /// Optional workload/case identifier.
    case_id: Option<String>,

    /// Circuit role.
    role: BenchmarkCircuitRole,

    /// Deterministic generation metadata.
    generation: BenchmarkCircuitGeneration,

    /// Whether this circuit is expected to have an independently computable
    /// ideal reference distribution.
    requires_ideal_reference: bool,
}

impl BenchmarkCircuitDescriptor {
    /// Creates a validated descriptor.
    pub fn new<B, E>(
        benchmark_id: B,
        experiment_id: E,
        role: BenchmarkCircuitRole,
        generation: BenchmarkCircuitGeneration,
    ) -> Result<Self, BenchmarkCircuitError>
    where
        B: Into<String>,
        E: Into<String>,
    {
        Self::with_case_id(
            benchmark_id,
            experiment_id,
            None,
            role,
            generation,
        )
    }

    /// Creates a descriptor with an optional workload/case identifier.
    pub fn with_case_id<B, E, C>(
        benchmark_id: B,
        experiment_id: E,
        case_id: Option<C>,
        role: BenchmarkCircuitRole,
        generation: BenchmarkCircuitGeneration,
    ) -> Result<Self, BenchmarkCircuitError>
    where
        B: Into<String>,
        E: Into<String>,
        C: Into<String>,
    {
        let benchmark_id = benchmark_id.into();
        let experiment_id = experiment_id.into();
        let case_id = case_id.map(Into::into);

        validate_identifier(
            "benchmark_id",
            &benchmark_id,
        )?;

        validate_identifier(
            "experiment_id",
            &experiment_id,
        )?;

        if let Some(case_id) = &case_id {
            validate_identifier("case_id", case_id)?;
        }

        Ok(Self {
            benchmark_id,
            experiment_id,
            case_id,
            role,
            generation,
            requires_ideal_reference: false,
        })
    }

    /// Sets whether an ideal reference distribution is required.
    #[must_use]
    pub const fn with_ideal_reference(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_ideal_reference = required;
        self
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

    /// Returns the optional workload/case identifier.
    #[must_use]
    pub fn case_id(&self) -> Option<&str> {
        self.case_id.as_deref()
    }

    /// Returns the circuit role.
    #[must_use]
    pub const fn role(&self) -> BenchmarkCircuitRole {
        self.role
    }

    /// Returns deterministic generation metadata.
    #[must_use]
    pub const fn generation(
        &self,
    ) -> BenchmarkCircuitGeneration {
        self.generation
    }

    /// Returns whether an ideal reference distribution is required.
    #[must_use]
    pub const fn requires_ideal_reference(&self) -> bool {
        self.requires_ideal_reference
    }
}

// =============================================================================
// Circuit fingerprint
// =============================================================================

/// Deterministic structural fingerprint of a benchmark circuit.
///
/// The fingerprint represents executable logical-circuit structure, not
/// benchmark descriptor metadata.
///
/// Consequently, changing a benchmark label does not change the circuit
/// fingerprint, while changing a gate, operand, parameter, measurement target,
/// or circuit namespace does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CircuitFingerprint {
    /// First independent 64-bit FNV-1a digest.
    primary: u64,

    /// Second independently mixed digest.
    secondary: u64,
}

impl CircuitFingerprint {
    /// Creates a fingerprint from its two 64-bit components.
    pub const fn new(
        primary: u64,
        secondary: u64,
    ) -> Self {
        Self {
            primary,
            secondary,
        }
    }

    /// Returns the primary digest.
    #[must_use]
    pub const fn primary(self) -> u64 {
        self.primary
    }

    /// Returns the secondary digest.
    #[must_use]
    pub const fn secondary(self) -> u64 {
        self.secondary
    }

    /// Returns the fingerprint as a fixed-width hexadecimal string.
    pub fn to_hex(self) -> String {
        format!(
            "{:016x}{:016x}",
            self.primary,
            self.secondary
        )
    }
}

impl fmt::Display for CircuitFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:016x}{:016x}",
            self.primary,
            self.secondary
        )
    }
}

// =============================================================================
// Circuit resources
// =============================================================================

/// Benchmark-relevant logical resource summary.
///
/// This is deliberately a snapshot. It does not replace
/// `quantum::ir::CircuitStatistics`.
///
/// The full IR statistics remain authoritative for general circuit analysis;
/// this compact structure provides the fields benchmark protocols most often
/// need without requiring every consumer to know the complete IR analysis
/// type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkCircuitResources {
    /// Declared logical qubits.
    qubits: usize,

    /// Declared logical classical bits.
    classical_bits: usize,

    /// Ordered operation count.
    operations: usize,

    /// Logical circuit depth.
    depth: usize,

    /// Number of one-qubit operations.
    single_qubit_gates: usize,

    /// Number of two-qubit operations.
    two_qubit_gates: usize,

    /// Number of operations with three or more operands.
    multi_qubit_gates: usize,

    /// Number of parameterized operations.
    parameterized_gates: usize,

    /// Number of measurement operations.
    measurements: usize,

    /// Number of reset operations.
    resets: usize,
}

impl BenchmarkCircuitResources {
    /// Constructs a resource snapshot from canonical IR statistics.
    fn from_statistics(
        statistics: &CircuitStatistics,
    ) -> Self {
        Self {
            qubits: statistics.qubits(),
            classical_bits: statistics.classical_bits(),
            operations: statistics.operation_count(),
            depth: statistics.depth(),
            single_qubit_gates: statistics.single_qubit_operations(),
            two_qubit_gates: statistics.two_qubit_operations(),
            multi_qubit_gates: statistics.multi_qubit_operations(),
            parameterized_gates: statistics.parameterized_operations(),
            measurements: statistics.measurement_count(),
            resets: statistics.reset_count(),
        }
    }

    /// Number of declared logical qubits.
    #[must_use]
    pub const fn qubits(self) -> usize {
        self.qubits
    }

    /// Number of declared logical classical bits.
    #[must_use]
    pub const fn classical_bits(self) -> usize {
        self.classical_bits
    }

    /// Number of logical operations.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }

    /// Logical circuit depth.
    #[must_use]
    pub const fn depth(self) -> usize {
        self.depth
    }

    /// Number of single-qubit gates.
    #[must_use]
    pub const fn single_qubit_gates(self) -> usize {
        self.single_qubit_gates
    }

    /// Number of two-qubit gates.
    #[must_use]
    pub const fn two_qubit_gates(self) -> usize {
        self.two_qubit_gates
    }

    /// Number of multi-qubit gates.
    #[must_use]
    pub const fn multi_qubit_gates(self) -> usize {
        self.multi_qubit_gates
    }

    /// Number of parameterized gates.
    #[must_use]
    pub const fn parameterized_gates(self) -> usize {
        self.parameterized_gates
    }

    /// Number of measurements.
    #[must_use]
    pub const fn measurements(self) -> usize {
        self.measurements
    }

    /// Number of resets.
    #[must_use]
    pub const fn resets(self) -> usize {
        self.resets
    }
}

// =============================================================================
// Benchmark circuit errors
// =============================================================================

/// Errors produced by the benchmark-circuit boundary.
///
/// This type intentionally remains local to this file until the centralized
/// `benchmarking::core::errors` contract is introduced.
///
/// Future `core/errors.rs` can convert this type without changing the circuit
/// API:
///
/// ```text
/// BenchmarkCircuitError
///        ↓
/// BenchmarkingError
/// ```
///
/// The circuit API itself therefore does not need to be redesigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkCircuitError {
    /// Benchmark identifier is empty or invalid.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// The supplied benchmark circuit is structurally invalid.
    InvalidCircuit {
        /// Static validation reason.
        message: &'static str,
    },

    /// Canonical Quantum IR validation failed.
    IrValidation {
        /// String representation of the underlying IR error.
        message: String,
    },

    /// Canonical Quantum IR statistics could not be computed.
    AnalysisFailed {
        /// String representation of the analysis error.
        message: String,
    },

    /// A circuit generation sequence number overflowed.
    SequenceOverflow,

    /// The caller attempted to construct an inconsistent benchmark circuit.
    InconsistentMetadata {
        /// Static reason.
        message: &'static str,
    },
}

impl fmt::Display for BenchmarkCircuitError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(
                    f,
                    "benchmark circuit `{field}` must not be empty and must contain a valid identifier"
                )
            }

            Self::InvalidCircuit { message } => {
                write!(
                    f,
                    "invalid benchmark circuit: {message}"
                )
            }

            Self::IrValidation { message } => {
                write!(
                    f,
                    "canonical Quantum IR validation failed: {message}"
                )
            }

            Self::AnalysisFailed { message } => {
                write!(
                    f,
                    "canonical Quantum IR analysis failed: {message}"
                )
            }

            Self::SequenceOverflow => {
                f.write_str(
                    "benchmark circuit sequence index overflow",
                )
            }

            Self::InconsistentMetadata { message } => {
                write!(
                    f,
                    "inconsistent benchmark circuit metadata: {message}"
                )
            }
        }
    }
}

impl std::error::Error for BenchmarkCircuitError {}

impl From<CircuitError> for BenchmarkCircuitError {
    fn from(error: CircuitError) -> Self {
        Self::IrValidation {
            message: error.to_string(),
        }
    }
}

// =============================================================================
// Benchmark circuit
// =============================================================================

/// Canonical circuit container used by Zamani's benchmarking subsystem.
///
/// `BenchmarkCircuit` owns a validated logical `QuantumCircuit` plus
/// benchmark-specific descriptive metadata and deterministic derived
/// information.
///
/// The underlying `QuantumCircuit` remains the sole semantic source of truth.
///
/// No backend, physical mapping, calibration, execution result, or statistical
/// information is stored here.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkCircuit {
    /// Canonical logical quantum circuit.
    circuit: QuantumCircuit,

    /// Benchmark descriptor.
    descriptor: BenchmarkCircuitDescriptor,

    /// Deterministic structural fingerprint.
    fingerprint: CircuitFingerprint,

    /// Logical resource snapshot.
    resources: BenchmarkCircuitResources,
}

impl BenchmarkCircuit {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates a benchmark circuit from an already validated Quantum IR
    /// circuit.
    ///
    /// The circuit is validated again at the benchmarking boundary. This is
    /// intentional: a benchmark may receive IR reconstructed from
    /// deserialization, generated by another compiler stage, or supplied by
    /// external tooling.
    pub fn new(
        circuit: QuantumCircuit,
        descriptor: BenchmarkCircuitDescriptor,
    ) -> Result<Self, BenchmarkCircuitError> {
        validate_circuit(&circuit).map_err(|error| {
            BenchmarkCircuitError::IrValidation {
                message: error.to_string(),
            }
        })?;

        let statistics = analyze(&circuit).map_err(|error| {
            BenchmarkCircuitError::AnalysisFailed {
                message: error.to_string(),
            }
        })?;

        let resources =
            BenchmarkCircuitResources::from_statistics(
                &statistics,
            );

        let fingerprint =
            calculate_fingerprint(&circuit);

        Ok(Self {
            circuit,
            descriptor,
            fingerprint,
            resources,
        })
    }

    /// Creates a generic benchmark circuit.
    ///
    /// This convenience constructor is useful for tests, generic circuit
    /// benchmarks, and protocol generators that will assign richer metadata
    /// later.
    pub fn generic(
        circuit: QuantumCircuit,
        benchmark_id: impl Into<String>,
        experiment_id: impl Into<String>,
    ) -> Result<Self, BenchmarkCircuitError> {
        let descriptor =
            BenchmarkCircuitDescriptor::new(
                benchmark_id,
                experiment_id,
                BenchmarkCircuitRole::Generic,
                BenchmarkCircuitGeneration::default(),
            )?;

        Self::new(circuit, descriptor)
    }

    // -------------------------------------------------------------------------
    // Canonical circuit access
    // -------------------------------------------------------------------------

    /// Returns the canonical Quantum IR circuit.
    ///
    /// The returned circuit is immutable. Benchmarking consumers must not
    /// mutate the underlying circuit after the benchmark object is created.
    #[must_use]
    pub fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Consumes the benchmark circuit and returns the canonical Quantum IR.
    ///
    /// Benchmark metadata and derived values are discarded because they are
    /// benchmarking-layer concerns.
    #[must_use]
    pub fn into_circuit(self) -> QuantumCircuit {
        self.circuit
    }

    /// Returns the stable logical circuit identity supplied by the IR.
    #[must_use]
    pub const fn circuit_id(&self) -> CircuitId {
        self.circuit.id()
    }

    /// Returns the underlying IR version.
    #[must_use]
    pub const fn ir_version(
        &self,
    ) -> crate::quantum::ir::IrVersion {
        self.circuit.version()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.circuit.num_qubits()
    }

    /// Returns the number of logical classical bits.
    #[must_use]
    pub const fn num_classical_bits(
        &self,
    ) -> usize {
        self.circuit.num_classical_bits()
    }

    /// Returns the ordered immutable gate sequence.
    #[must_use]
    pub fn operations(&self) -> &[Gate] {
        self.circuit.operations()
    }

    /// Returns one gate by sequence index.
    #[must_use]
    pub fn operation(
        &self,
        index: usize,
    ) -> Option<&Gate> {
        self.circuit.get(index)
    }

    /// Returns the number of logical operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.circuit.len()
    }

    /// Returns whether the circuit contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.circuit.is_empty()
    }

    // -------------------------------------------------------------------------
    // Benchmark metadata
    // -------------------------------------------------------------------------

    /// Returns the complete benchmark descriptor.
    #[must_use]
    pub fn descriptor(
        &self,
    ) -> &BenchmarkCircuitDescriptor {
        &self.descriptor
    }

    /// Returns the benchmark protocol identifier.
    #[must_use]
    pub fn benchmark_id(&self) -> &str {
        self.descriptor.benchmark_id()
    }

    /// Returns the experiment identifier.
    #[must_use]
    pub fn experiment_id(&self) -> &str {
        self.descriptor.experiment_id()
    }

    /// Returns the optional case identifier.
    #[must_use]
    pub fn case_id(&self) -> Option<&str> {
        self.descriptor.case_id()
    }

    /// Returns the circuit role.
    #[must_use]
    pub const fn role(
        &self,
    ) -> BenchmarkCircuitRole {
        self.descriptor.role()
    }

    /// Returns deterministic generation metadata.
    #[must_use]
    pub const fn generation(
        &self,
    ) -> BenchmarkCircuitGeneration {
        self.descriptor.generation()
    }

    /// Returns whether the benchmark requires an ideal reference.
    #[must_use]
    pub const fn requires_ideal_reference(
        &self,
    ) -> bool {
        self.descriptor.requires_ideal_reference()
    }

    // -------------------------------------------------------------------------
    // Derived resources
    // -------------------------------------------------------------------------

    /// Returns the logical resource snapshot.
    ///
    /// These values are derived from canonical Quantum IR analysis and do not
    /// describe physical execution.
    #[must_use]
    pub const fn resources(
        &self,
    ) -> BenchmarkCircuitResources {
        self.resources
    }

    /// Returns logical circuit depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.resources.depth()
    }

    /// Returns the number of one-qubit gates.
    #[must_use]
    pub const fn single_qubit_gate_count(
        &self,
    ) -> usize {
        self.resources.single_qubit_gates()
    }

    /// Returns the number of two-qubit gates.
    #[must_use]
    pub const fn two_qubit_gate_count(
        &self,
    ) -> usize {
        self.resources.two_qubit_gates()
    }

    /// Returns the number of gates containing three or more operands.
    #[must_use]
    pub const fn multi_qubit_gate_count(
        &self,
    ) -> usize {
        self.resources.multi_qubit_gates()
    }

    /// Returns the number of parameterized gates.
    #[must_use]
    pub const fn parameterized_gate_count(
        &self,
    ) -> usize {
        self.resources.parameterized_gates()
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub const fn measurement_count(
        &self,
    ) -> usize {
        self.resources.measurements()
    }

    /// Returns the number of reset operations.
    #[must_use]
    pub const fn reset_count(
        &self,
    ) -> usize {
        self.resources.resets()
    }

    // -------------------------------------------------------------------------
    // Fingerprinting
    // -------------------------------------------------------------------------

    /// Returns the deterministic structural fingerprint.
    #[must_use]
    pub const fn fingerprint(
        &self,
    ) -> CircuitFingerprint {
        self.fingerprint
    }

    /// Returns the fingerprint as a fixed-width hexadecimal value.
    #[must_use]
    pub fn fingerprint_hex(&self) -> String {
        self.fingerprint.to_hex()
    }

    /// Returns true if two benchmark circuits have identical executable
    /// logical-circuit structure.
    ///
    /// Benchmark descriptor metadata is deliberately ignored.
    #[must_use]
    pub fn same_structure(
        &self,
        other: &Self,
    ) -> bool {
        self.fingerprint == other.fingerprint
            && circuits_structurally_equal(
                &self.circuit,
                &other.circuit,
            )
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Revalidates the canonical Quantum IR.
    ///
    /// This method exists because benchmark circuits may eventually be
    /// reconstructed from serialized benchmark data.
    pub fn validate(
        &self,
    ) -> Result<(), BenchmarkCircuitError> {
        validate_circuit(&self.circuit).map_err(
            |error| BenchmarkCircuitError::IrValidation {
                message: error.to_string(),
            },
        )
    }

    /// Returns a fresh canonical statistics calculation.
    ///
    /// The cached benchmark resources remain available for normal operation,
    /// while this method permits explicit validation/debugging of the cache.
    pub fn recompute_statistics(
        &self,
    ) -> Result<CircuitStatistics, BenchmarkCircuitError> {
        analyze(&self.circuit).map_err(|error| {
            BenchmarkCircuitError::AnalysisFailed {
                message: error.to_string(),
            }
        })
    }

    /// Verifies that the derived fingerprint and resource snapshot still agree
    /// with the underlying immutable circuit.
    ///
    /// This is primarily useful at serialization/replay boundaries.
    pub fn verify_integrity(
        &self,
    ) -> Result<(), BenchmarkCircuitError> {
        self.validate()?;

        let statistics =
            self.recompute_statistics()?;

        let expected_resources =
            BenchmarkCircuitResources::from_statistics(
                &statistics,
            );

        if expected_resources != self.resources {
            return Err(
                BenchmarkCircuitError::InconsistentMetadata {
                    message:
                        "cached resource summary does not match canonical IR",
                },
            );
        }

        let expected_fingerprint =
            calculate_fingerprint(&self.circuit);

        if expected_fingerprint != self.fingerprint {
            return Err(
                BenchmarkCircuitError::InconsistentMetadata {
                    message:
                        "cached circuit fingerprint does not match canonical IR",
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates a benchmark identifier.
///
/// Identifiers are intentionally stricter than arbitrary human-readable
/// strings because these values become registry keys, experiment identifiers,
/// report fields, and reproducibility inputs.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), BenchmarkCircuitError> {
    if value.is_empty() {
        return Err(
            BenchmarkCircuitError::InvalidIdentifier {
                field,
            },
        );
    }

    if value.len() > 256 {
        return Err(
            BenchmarkCircuitError::InvalidIdentifier {
                field,
            },
        );
    }

    let mut chars = value.chars();

    let first = match chars.next() {
        Some(character) => character,
        None => {
            return Err(
                BenchmarkCircuitError::InvalidIdentifier {
                    field,
                },
            );
        }
    };

    if !(first.is_ascii_alphanumeric() || first == '_') {
        return Err(
            BenchmarkCircuitError::InvalidIdentifier {
                field,
            },
        );
    }

    for character in chars {
        if !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.')
        {
            return Err(
                BenchmarkCircuitError::InvalidIdentifier {
                    field,
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Fingerprint implementation
// =============================================================================

/// Deterministic FNV-1a hasher used only for circuit fingerprints.
#[derive(Debug, Clone, Copy)]
struct FingerprintHasher {
    primary: u64,
    secondary: u64,
}

impl FingerprintHasher {
    fn new() -> Self {
        Self {
            primary: FNV_OFFSET_BASIS,
            secondary:
                FNV_OFFSET_BASIS
                    ^ 0x9e3779b97f4a7c15,
        }
    }

    fn write_u8(
        &mut self,
        value: u8,
    ) {
        self.primary ^= value as u64;
        self.primary =
            self.primary.wrapping_mul(FNV_PRIME);

        self.secondary ^=
            (value as u64)
                .wrapping_add(0x9e);
        self.secondary = self
            .secondary
            .rotate_left(5)
            .wrapping_mul(FNV_PRIME);
    }

    fn write_u16(
        &mut self,
        value: u16,
    ) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    fn write_u32(
        &mut self,
        value: u32,
    ) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    fn write_u64(
        &mut self,
        value: u64,
    ) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    fn write_usize(
        &mut self,
        value: usize,
    ) {
        self.write_u64(value as u64);
    }

    fn write_bool(
        &mut self,
        value: bool,
    ) {
        self.write_u8(if value { 1 } else { 0 });
    }

    fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) {
        self.write_usize(bytes.len());

        for &byte in bytes {
            self.write_u8(byte);
        }
    }

    fn write_str(
        &mut self,
        value: &str,
    ) {
        self.write_bytes(value.as_bytes());
    }

    fn finish(
        self,
    ) -> CircuitFingerprint {
        CircuitFingerprint::new(
            self.primary,
            self.secondary,
        )
    }
}

/// Calculates the executable structural fingerprint of a Quantum IR circuit.
fn calculate_fingerprint(
    circuit: &QuantumCircuit,
) -> CircuitFingerprint {
    let mut hasher =
        FingerprintHasher::new();

    // Version/schema boundary.
    hasher.write_u16(
        crate::quantum::benchmarking::core::circuit::BENCHMARK_CIRCUIT_SCHEMA_VERSION,
    );

    // Quantum IR identity that affects semantic interpretation.
    write_debug_value(
        &mut hasher,
        &circuit.version(),
    );

    // Namespace sizes affect the circuit's semantic environment.
    hasher.write_usize(
        circuit.num_qubits(),
    );

    hasher.write_usize(
        circuit.num_classical_bits(),
    );

    // Operation order is semantically significant.
    hasher.write_usize(
        circuit.operations().len(),
    );

    for gate in circuit.operations() {
        write_gate_fingerprint(
            &mut hasher,
            gate,
        );
    }

    hasher.finish()
}

/// Writes the complete semantically relevant gate structure.
///
/// `GateKind`, operands, parameters, classical targets, and measurement
/// payloads are included. This ensures that changing a measurement basis or
/// parameter cannot accidentally preserve the same fingerprint.
fn write_gate_fingerprint(
    hasher: &mut FingerprintHasher,
    gate: &Gate,
) {
    write_debug_value(
        hasher,
        &gate.kind(),
    );

    hasher.write_usize(
        gate.qubits().len(),
    );

    for qubit in gate.qubits() {
        hasher.write_usize(
            qubit.index(),
        );
    }

    hasher.write_usize(
        gate.parameters().len(),
    );

    for parameter in gate.parameters() {
        write_debug_value(
            hasher,
            parameter,
        );
    }

    match gate.classical_target() {
        Some(target) => {
            hasher.write_bool(true);
            hasher.write_usize(target);
        }

        None => {
            hasher.write_bool(false);
        }
    }

    match gate.measurement() {
        Some(measurement) => {
            hasher.write_bool(true);
            write_debug_value(
                hasher,
                measurement,
            );
        }

        None => {
            hasher.write_bool(false);
        }
    }
}

/// Adds a stable textual representation of a value to the fingerprint.
///
/// This is used only for IR enum/parameter payloads whose public API currently
/// does not expose a canonical byte serialization.
///
/// The benchmark circuit fingerprint is therefore a deterministic
/// implementation-level fingerprint, not a cryptographic wire-format hash.
/// The benchmark provenance layer must record the Zamani/IR schema version
/// alongside it.
fn write_debug_value<T>(
    hasher: &mut FingerprintHasher,
    value: &T,
) where
    T: fmt::Debug,
{
    let representation =
        format!("{value:?}");

    hasher.write_str(
        &representation,
    );
}

/// Performs an explicit structural comparison in addition to fingerprint
/// comparison.
///
/// This prevents a fingerprint collision from being interpreted as circuit
/// equality.
fn circuits_structurally_equal(
    left: &QuantumCircuit,
    right: &QuantumCircuit,
) -> bool {
    if left.version() != right.version() {
        return false;
    }

    if left.num_qubits()
        != right.num_qubits()
    {
        return false;
    }

    if left.num_classical_bits()
        != right.num_classical_bits()
    {
        return false;
    }

    let left_operations =
        left.operations();

    let right_operations =
        right.operations();

    if left_operations.len()
        != right_operations.len()
    {
        return false;
    }

    left_operations
        .iter()
        .zip(right_operations.iter())
        .all(|(left_gate, right_gate)| {
            format!("{left_gate:?}")
                == format!("{right_gate:?}")
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        QubitId,
    };

    fn bell_circuit() -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .expect("valid test circuit");

        circuit
            .push(
                Gate::h(QubitId::new(0))
                    .expect("valid H gate"),
            )
            .expect("valid H insertion");

        circuit
            .push(
                Gate::cx(
                    QubitId::new(0),
                    QubitId::new(1),
                )
                .expect("valid CX gate"),
            )
            .expect("valid CX insertion");

        circuit
    }

    #[test]
    fn descriptor_rejects_empty_benchmark_id() {
        let result =
            BenchmarkCircuitDescriptor::new(
                "",
                "experiment",
                BenchmarkCircuitRole::Generic,
                BenchmarkCircuitGeneration::default(),
            );

        assert!(matches!(
            result,
            Err(
                BenchmarkCircuitError::InvalidIdentifier {
                    field: "benchmark_id"
                }
            )
        ));
    }

    #[test]
    fn descriptor_rejects_invalid_identifier_characters() {
        let result =
            BenchmarkCircuitDescriptor::new(
                "quantum volume",
                "experiment",
                BenchmarkCircuitRole::Generic,
                BenchmarkCircuitGeneration::default(),
            );

        assert!(matches!(
            result,
            Err(
                BenchmarkCircuitError::InvalidIdentifier {
                    field: "benchmark_id"
                }
            )
        ));
    }

    #[test]
    fn descriptor_accepts_production_identifier() {
        let descriptor =
            BenchmarkCircuitDescriptor::new(
                "quantum_volume",
                "experiment-001",
                BenchmarkCircuitRole::Random,
                BenchmarkCircuitGeneration::new(
                    42,
                    0,
                    1,
                ),
            )
            .expect("descriptor must be valid");

        assert_eq!(
            descriptor.benchmark_id(),
            "quantum_volume"
        );

        assert_eq!(
            descriptor.experiment_id(),
            "experiment-001"
        );

        assert_eq!(
            descriptor.role(),
            BenchmarkCircuitRole::Random
        );

        assert_eq!(
            descriptor.generation().seed(),
            42
        );
    }

    #[test]
    fn benchmark_circuit_uses_canonical_ir() {
        let circuit =
            bell_circuit();

        let descriptor =
            BenchmarkCircuitDescriptor::new(
                "test",
                "experiment",
                BenchmarkCircuitRole::Trial,
                BenchmarkCircuitGeneration::default(),
            )
            .expect("valid descriptor");

        let benchmark_circuit =
            BenchmarkCircuit::new(
                circuit,
                descriptor,
            )
            .expect("valid benchmark circuit");

        assert_eq!(
            benchmark_circuit.num_qubits(),
            2
        );

        assert_eq!(
            benchmark_circuit.operation_count(),
            2
        );

        assert_eq!(
            benchmark_circuit.two_qubit_gate_count(),
            1
        );

        assert_eq!(
            benchmark_circuit.single_qubit_gate_count(),
            1
        );

        assert_eq!(
            benchmark_circuit.depth(),
            2
        );
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let first =
            BenchmarkCircuit::generic(
                bell_circuit(),
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        let second =
            BenchmarkCircuit::generic(
                bell_circuit(),
                "different_name",
                "different_experiment",
            )
            .expect("valid benchmark circuit");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );

        assert!(
            first.same_structure(&second)
        );
    }

    #[test]
    fn different_gate_structure_changes_fingerprint() {
        let first =
            BenchmarkCircuit::generic(
                bell_circuit(),
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        let mut modified =
            QuantumCircuit::new(2, 2)
                .expect("valid circuit");

        modified
            .push(
                Gate::x(QubitId::new(0))
                    .expect("valid X gate"),
            )
            .expect("valid insertion");

        modified
            .push(
                Gate::cx(
                    QubitId::new(0),
                    QubitId::new(1),
                )
                .expect("valid CX gate"),
            )
            .expect("valid insertion");

        let second =
            BenchmarkCircuit::generic(
                modified,
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );

        assert!(
            !first.same_structure(&second)
        );
    }

    #[test]
    fn generation_metadata_is_preserved() {
        let generation =
            BenchmarkCircuitGeneration::new(
                0x1234,
                17,
                3,
            );

        let descriptor =
            BenchmarkCircuitDescriptor::new(
                "rb",
                "exp",
                BenchmarkCircuitRole::Random,
                generation,
            )
            .expect("valid descriptor");

        let circuit =
            BenchmarkCircuit::new(
                bell_circuit(),
                descriptor,
            )
            .expect("valid benchmark circuit");

        assert_eq!(
            circuit.generation().seed(),
            0x1234
        );

        assert_eq!(
            circuit.generation().sequence_index(),
            17
        );

        assert_eq!(
            circuit.generation().generator_revision(),
            3
        );
    }

    #[test]
    fn ideal_reference_requirement_is_preserved() {
        let descriptor =
            BenchmarkCircuitDescriptor::new(
                "xeb",
                "exp",
                BenchmarkCircuitRole::Random,
                BenchmarkCircuitGeneration::default(),
            )
            .expect("valid descriptor")
            .with_ideal_reference(true);

        let circuit =
            BenchmarkCircuit::new(
                bell_circuit(),
                descriptor,
            )
            .expect("valid benchmark circuit");

        assert!(
            circuit.requires_ideal_reference()
        );
    }

    #[test]
    fn integrity_verification_succeeds() {
        let circuit =
            BenchmarkCircuit::generic(
                bell_circuit(),
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        circuit
            .verify_integrity()
            .expect("integrity must hold");
    }

    #[test]
    fn into_circuit_returns_canonical_ir() {
        let original =
            bell_circuit();

        let expected_operations =
            original.operations().len();

        let benchmark =
            BenchmarkCircuit::generic(
                original,
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        let recovered =
            benchmark.into_circuit();

        assert_eq!(
            recovered.operations().len(),
            expected_operations
        );

        assert_eq!(
            recovered.num_qubits(),
            2
        );
    }

    #[test]
    fn fingerprint_hex_has_fixed_length() {
        let circuit =
            BenchmarkCircuit::generic(
                bell_circuit(),
                "test",
                "experiment",
            )
            .expect("valid benchmark circuit");

        assert_eq!(
            circuit.fingerprint_hex().len(),
            32
        );
    }
}