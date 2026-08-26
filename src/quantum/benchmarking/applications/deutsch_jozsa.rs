//! Zamani Quantum Benchmarking — Deutsch–Jozsa Application Benchmark
//!
//! # Purpose
//!
//! Production application-benchmark generator for the Deutsch–Jozsa
//! algorithm.
//!
//! The generator constructs a canonical Zamani `QuantumCircuit` and wraps it
//! in the existing `ApplicationWorkload` model.
//!
//! It deliberately does NOT:
//!
//! - execute the circuit;
//! - select a backend;
//! - submit jobs;
//! - perform routing;
//! - perform scheduling;
//! - perform hardware calibration;
//! - perform statistical analysis;
//! - calculate runtime metrics;
//! - calculate fidelity/error metrics;
//! - communicate with hardware;
//! - perform filesystem/network I/O;
//! - depend on a simulator;
//! - implement backend-specific gates.
//!
//! Those responsibilities belong to the existing benchmarking execution,
//! hardware, runtime, metrics, and analysis layers.
//!
//! # Architectural position
//!
//! ```text
//! ApplicationGenerationRequest
//!          │
//!          ▼
//! DeutschJozsaBenchmarkGenerator
//!          │
//!          ├── validates promised oracle
//!          ├── constructs Quantum IR
//!          └── constructs ApplicationWorkload
//!                    │
//!                    ▼
//!              BenchmarkExperiment
//!                    │
//!                    ▼
//!              BenchmarkExecutor
//!                    │
//!                    ▼
//!              ObservationSet
//!                    │
//!                    ▼
//!              application analysis
//! ```
//!
//! # Deutsch–Jozsa problem
//!
//! The oracle represents a Boolean function:
//!
//! ```text
//! f : {0,1}^n -> {0,1}
//! ```
//!
//! The benchmark requires the oracle to be either:
//!
//! 1. constant — `f(x)` is identical for every input;
//! 2. balanced — exactly half of all inputs map to 0 and half map to 1.
//!
//! This implementation provides three concrete oracle forms:
//!
//! - `constant_zero`
//! - `constant_one`
//! - `balanced`
//!
//! The balanced implementation uses a non-empty parity function:
//!
//! ```text
//! f(x) = XOR(x_i for i in S)
//! ```
//!
//! for a non-empty subset `S` of input qubits.
//!
//! Any non-empty parity function is balanced because exactly half of all
//! possible input strings produce parity zero and half produce parity one.
//!
//! # Circuit
//!
//! For `n` input qubits the generated circuit contains:
//!
//! ```text
//! n input qubits
//! + 1 oracle/phase-kickback ancilla
//! + n classical output bits
//! ```
//!
//! Preparation:
//!
//! ```text
//! H on every input qubit
//! H on ancilla
//! X on ancilla
//! ```
//!
//! Oracle:
//!
//! ```text
//! constant_zero:
//!     identity
//!
//! constant_one:
//!     X on ancilla
//!
//! balanced:
//!     CX(input_i, ancilla) for every i in S
//! ```
//!
//! The ancilla preparation is arranged as:
//!
//! ```text
//! |0> --X--H-- Oracle --H--
//! ```
//!
//! which produces the usual `|->` phase-kickback state before the oracle.
//!
//! After the oracle:
//!
//! ```text
//! H on every input qubit
//! ```
//!
//! Finally, every input qubit is measured into a distinct classical bit.
//!
//! For a correct noiseless execution, the input measurement is expected to be
//! the all-zero bit string for both constant and balanced promised oracles.
//!
//! The analysis layer should use the measured input register to determine the
//! classification confidence. This generator does not perform that analysis.
//!
//! # Request parameters
//!
//! The canonical `ApplicationGenerationRequest` may contain these optional
//! parameters:
//!
//! ```text
//! oracle = constant_zero
//! oracle = constant_one
//! oracle = balanced
//!
//! balanced_qubits = 0
//! balanced_qubits = 0,2,5
//! ```
//!
//! `balanced_qubits` is required only when `oracle = balanced` if the caller
//! wants an explicit subset. If it is omitted, the generator deterministically
//! derives a single input qubit from the request seed and sequence index.
//!
//! Unknown parameters are rejected. Duplicate parameter names are rejected.
//!
//! This is intentional: benchmark configuration must be explicit and must not
//! silently ignore misspelled parameters.
//!
//! # Reproducibility
//!
//! Generation is deterministic.
//!
//! Identical:
//!
//! ```text
//! application_id
//! instance_id
//! problem_size
//! parameters
//! seed
//! sequence_index
//! generator_revision
//! ```
//!
//! produce semantically identical circuits.
//!
//! No global RNG, system clock, process ID, pointer address, thread ID, or
//! external entropy is consulted.
//!
//! # Security/resource model
//!
//! Requests are treated as untrusted input.
//!
//! The implementation:
//!
//! - rejects zero problem sizes;
//! - rejects arithmetic overflow;
//! - relies on canonical Quantum IR resource limits;
//! - validates every generated gate through `Gate::new`;
//! - validates every circuit insertion through `QuantumCircuit::push`;
//! - rejects malformed oracle parameters;
//! - rejects duplicate parameters;
//! - rejects invalid balanced-qubit indexes;
//! - rejects empty balanced subsets;
//! - rejects non-balanced oracle definitions;
//! - does not allocate a truth table exponentially in `n`;
//! - never enumerates all `2^n` classical inputs;
//! - never executes caller-provided code.
//!
//! The last point is important: a Deutsch–Jozsa benchmark must not construct an
//! exponential truth table merely to prove that a simple parity oracle is
//! balanced. The parity construction has a direct mathematical proof.
//!
//! # Complexity
//!
//! Circuit construction is linear in the number of input qubits plus the
//! number of balanced-oracle parity terms:
//!
//! ```text
//! O(n + |S|)
//! ```
//!
//! It does NOT construct the `2^n` truth table.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file integrates with the already-existing contracts:
//!
//! ```text
//! crate::quantum::benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorDescriptor
//!     └── ApplicationGeneratorCapability
//!
//! crate::quantum::benchmarking::core::workload
//!     ├── ApplicationWorkload
//!     ├── ApplicationParameter
//!     ├── CircuitWorkload
//!     └── WorkloadId
//!
//! crate::quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── GateKind
//!     ├── Measurement
//!     ├── QubitId
//!     └── ClassicalBitId
//! ```
//!
//! The generator does not require changes to those contracts.
//!
//! The only module-level integration required later is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!     pub mod deutsch_jozsa;
//! ```
//!
//! That is namespace registration only; this file's implementation does not
//! need to be edited when the module is registered.
//!
//! # Scientific references
//!
//! The benchmark follows the standard Deutsch–Jozsa promise problem:
//!
//! ```text
//! constant:  f(x) = c
//! balanced:  |{x : f(x)=0}| = |{x : f(x)=1}| = 2^(n-1)
//! ```
//!
//! The standard algorithm prepares the input register in superposition,
//! prepares an ancilla for phase kickback, applies the oracle, applies
//! Hadamards to the input register, and measures the input register.
//!
//! This implementation intentionally keeps the oracle construction explicit
//! rather than hiding it behind a vendor-specific oracle abstraction.

use std::collections::BTreeSet;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    CircuitWorkload,
    WorkloadError,
    WorkloadId,
};
use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
    ApplicationGenerationRequest,
};

use crate::quantum::ir::{
    gate::{Gate, GateKind},
    measurement::{ClassicalBitId, Measurement},
    qubit::QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable application benchmark identifier.
pub const DEUTSCH_JOZSA_BENCHMARK_ID: &str = "deutsch_jozsa";

/// Stable application identifier.
///
/// This is deliberately the same as the benchmark identifier because this
/// implementation represents the canonical Deutsch–Jozsa application.
pub const DEUTSCH_JOZSA_APPLICATION_ID: &str = "deutsch_jozsa";

/// Generator implementation version.
///
/// This is independent from the global benchmarking contract version.
pub const DEUTSCH_JOZSA_GENERATOR_VERSION: &str = "1.0.0";

/// Generator revision used in reproducibility metadata.
pub const DEUTSCH_JOZSA_GENERATOR_REVISION: u32 = 1;

/// Human-readable benchmark name.
pub const DEUTSCH_JOZSA_NAME: &str = "Deutsch–Jozsa";

/// Maximum number of explicit balanced-qubit entries accepted from the
/// bounded `ApplicationParameter` representation.
///
/// The canonical Quantum IR resource policy remains authoritative for actual
/// circuit size. This limit only prevents a pathological textual parameter
/// from becoming an unnecessarily large parser input.
pub const MAX_BALANCED_QUBITS_PARAMETER_ENTRIES: usize = 256;

/// Maximum number of bytes accepted for the complete `balanced_qubits`
/// parameter before parsing.
///
/// `ApplicationParameter` already imposes its own smaller general bound; this
/// constant makes the application-specific contract explicit.
pub const MAX_BALANCED_QUBITS_PARAMETER_BYTES: usize = 512;

// =============================================================================
// Classification
// =============================================================================

/// Mathematical classification of a Deutsch–Jozsa oracle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeutschJozsaClassification {
    /// `f(x)` is constant for every input.
    Constant,

    /// `f(x)` is balanced.
    Balanced,
}

impl DeutschJozsaClassification {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constant => "constant",
            Self::Balanced => "balanced",
        }
    }
}

impl std::fmt::Display for DeutschJozsaClassification {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Oracle
// =============================================================================

/// Concrete Deutsch–Jozsa oracle definition.
///
/// The oracle is represented semantically first. Circuit construction is
/// performed only after validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeutschJozsaOracle {
    /// Constant-zero function:
    ///
    /// `f(x) = 0`.
    ConstantZero,

    /// Constant-one function:
    ///
    /// `f(x) = 1`.
    ConstantOne,

    /// Balanced parity function:
    ///
    /// `f(x) = XOR(x_i for i in qubits)`.
    ///
    /// The subset must be non-empty.
    BalancedParity {
        /// Input-qubit indexes participating in the parity.
        qubits: Vec<usize>,
    },
}

impl DeutschJozsaOracle {
    /// Returns the mathematical classification.
    #[must_use]
    pub const fn classification(&self) -> DeutschJozsaClassification {
        match self {
            Self::ConstantZero | Self::ConstantOne => {
                DeutschJozsaClassification::Constant
            }
            Self::BalancedParity { .. } => {
                DeutschJozsaClassification::Balanced
            }
        }
    }

    /// Returns a stable oracle identifier.
    #[must_use]
    pub const fn kind_id(&self) -> &'static str {
        match self {
            Self::ConstantZero => "constant_zero",
            Self::ConstantOne => "constant_one",
            Self::BalancedParity { .. } => "balanced",
        }
    }

    /// Returns whether the oracle is mathematically balanced.
    ///
    /// Every non-empty parity function is balanced.
    #[must_use]
    pub fn is_balanced(&self) -> bool {
        matches!(self, Self::BalancedParity { qubits } if !qubits.is_empty())
    }

    /// Validates the oracle against an input-register size.
    pub fn validate(
        &self,
        input_qubits: usize,
    ) -> BenchmarkResult<()> {
        if input_qubits == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "Deutsch–Jozsa requires at least one input qubit",
            ));
        }

        match self {
            Self::ConstantZero | Self::ConstantOne => Ok(()),

            Self::BalancedParity { qubits } => {
                if qubits.is_empty() {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "a balanced parity oracle requires at least one input qubit",
                    ));
                }

                if qubits.len() > MAX_BALANCED_QUBITS_PARAMETER_ENTRIES {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "too many balanced parity qubits",
                    ));
                }

                let mut seen = BTreeSet::new();

                for &qubit in qubits {
                    if qubit >= input_qubits {
                        return Err(invalid_configuration(
                            "balanced_qubits",
                            "balanced oracle qubit index is outside the input register",
                        ));
                    }

                    if !seen.insert(qubit) {
                        return Err(invalid_configuration(
                            "balanced_qubits",
                            "balanced oracle contains a duplicate qubit index",
                        ));
                    }
                }

                Ok(())
            }
        }
    }

    /// Returns the number of oracle CX operations.
    #[must_use]
    pub fn oracle_gate_count(&self) -> usize {
        match self {
            Self::ConstantZero => 0,
            Self::ConstantOne => 1,
            Self::BalancedParity { qubits } => qubits.len(),
        }
    }

    /// Returns a deterministic textual representation of the oracle subset.
    pub fn balanced_qubits_string(&self) -> Option<String> {
        match self {
            Self::BalancedParity { qubits } => {
                let mut output = String::new();

                for (index, qubit) in qubits.iter().enumerate() {
                    if index != 0 {
                        output.push(',');
                    }

                    output.push_str(&qubit.to_string());
                }

                Some(output)
            }

            Self::ConstantZero | Self::ConstantOne => None,
        }
    }
}

// =============================================================================
// Benchmark workload description
// =============================================================================

/// Immutable application-specific description of one generated Deutsch–Jozsa
/// workload.
///
/// The canonical `ApplicationWorkload` remains the semantic workload object.
/// This structure provides strongly typed application metadata to callers
/// that need to inspect the generated benchmark without parsing textual
/// `ApplicationParameter` values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeutschJozsaWorkloadDescription {
    /// Number of input qubits.
    pub input_qubits: usize,

    /// Number of oracle ancilla qubits.
    pub ancilla_qubits: usize,

    /// Number of measured classical bits.
    pub classical_bits: usize,

    /// Oracle definition.
    pub oracle: DeutschJozsaOracle,

    /// Expected algorithm classification.
    pub expected_classification: DeutschJozsaClassification,

    /// Expected input-register measurement.
    ///
    /// A string containing exactly `input_qubits` zero characters.
    pub expected_measurement: String,

    /// Number of logical quantum operations in the generated circuit.
    pub logical_operation_count: usize,

    /// Number of logical two-qubit operations.
    pub logical_two_qubit_gate_count: usize,
}

impl DeutschJozsaWorkloadDescription {
    /// Builds a validated workload description.
    pub fn new(
        input_qubits: usize,
        oracle: DeutschJozsaOracle,
    ) -> BenchmarkResult<Self> {
        oracle.validate(input_qubits)?;

        let ancilla_qubits = 1usize;
        let classical_bits = input_qubits;

        let preparation_and_final_hadamards = input_qubits
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| numerical_overflow("Deutsch–Jozsa Hadamard count"))?;

        let measurements = input_qubits;

        let logical_operation_count = preparation_and_final_hadamards
            .checked_add(oracle.oracle_gate_count())
            .and_then(|value| value.checked_add(measurements))
            .ok_or_else(|| numerical_overflow(
                "Deutsch–Jozsa logical operation count",
            ))?;

        let logical_two_qubit_gate_count = match &oracle {
            DeutschJozsaOracle::BalancedParity { qubits } => qubits.len(),
            DeutschJozsaOracle::ConstantZero
            | DeutschJozsaOracle::ConstantOne => 0,
        };

        Ok(Self {
            input_qubits,
            ancilla_qubits,
            classical_bits,
            oracle: oracle.clone(),
            expected_classification: oracle.classification(),
            expected_measurement: "0".repeat(input_qubits),
            logical_operation_count,
            logical_two_qubit_gate_count,
        })
    }

    /// Returns the total logical qubit count.
    #[must_use]
    pub fn total_qubits(&self) -> usize {
        self.input_qubits + self.ancilla_qubits
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Deutsch–Jozsa application benchmark generator.
///
/// The generator itself is stateless and therefore safe to share through
/// `Arc<dyn ApplicationBenchmarkGenerator>`.
#[derive(Debug, Clone)]
pub struct DeutschJozsaBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl DeutschJozsaBenchmarkGenerator {
    /// Creates the canonical Deutsch–Jozsa generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = ApplicationGeneratorDescriptor::new(
            DEUTSCH_JOZSA_BENCHMARK_ID,
            DEUTSCH_JOZSA_APPLICATION_ID,
            DEUTSCH_JOZSA_GENERATOR_VERSION,
            "Production Deutsch–Jozsa application benchmark generator",
        )?
        .with_capabilities([
            ApplicationGeneratorCapability::GeneratesCircuit,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::BatchGeneration,
            ApplicationGeneratorCapability::ScalableProblemSize,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::ExactSmallInstanceReference,
            ApplicationGeneratorCapability::ClassicallyVerifiable,
            ApplicationGeneratorCapability::ResourceEstimation,
            ApplicationGeneratorCapability::HardwareExecutable,
        ]);

        Ok(Self { descriptor })
    }

    /// Parses an application generation request into a typed oracle.
    ///
    /// Unknown parameters and duplicate parameters are rejected.
    pub fn oracle_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<DeutschJozsaOracle> {
        self.validate(request)?;

        let mut oracle_kind: Option<String> = None;
        let mut balanced_qubits: Option<String> = None;

        for parameter in request.parameters() {
            match parameter.name() {
                "oracle" => {
                    if oracle_kind.is_some() {
                        return Err(invalid_configuration(
                            "oracle",
                            "duplicate oracle parameter",
                        ));
                    }

                    oracle_kind = Some(parameter.value().to_owned());
                }

                "balanced_qubits" => {
                    if balanced_qubits.is_some() {
                        return Err(invalid_configuration(
                            "balanced_qubits",
                            "duplicate balanced_qubits parameter",
                        ));
                    }

                    if parameter.value().len()
                        > MAX_BALANCED_QUBITS_PARAMETER_BYTES
                    {
                        return Err(invalid_configuration(
                            "balanced_qubits",
                            "balanced_qubits parameter is too large",
                        ));
                    }

                    balanced_qubits = Some(parameter.value().to_owned());
                }

                other => {
                    return Err(invalid_configuration(
                        "application_parameter",
                        match other {
                            "" => "application parameter name must not be empty",
                            _ => "unknown Deutsch–Jozsa application parameter",
                        },
                    ));
                }
            }
        }

        let oracle_kind = oracle_kind.unwrap_or_else(|| "balanced".to_owned());

        let oracle = match oracle_kind.as_str() {
            "constant_zero" => {
                if balanced_qubits.is_some() {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "balanced_qubits is only valid for a balanced oracle",
                    ));
                }

                DeutschJozsaOracle::ConstantZero
            }

            "constant_one" => {
                if balanced_qubits.is_some() {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "balanced_qubits is only valid for a balanced oracle",
                    ));
                }

                DeutschJozsaOracle::ConstantOne
            }

            "balanced" => {
                let qubits = match balanced_qubits {
                    Some(value) => parse_balanced_qubits(
                        &value,
                        request.problem_size(),
                    )?,

                    None => {
                        // Deterministic default:
                        //
                        // seed + sequence_index selects one input qubit.
                        //
                        // A single-input parity function is balanced and
                        // scales to arbitrary problem sizes without requiring
                        // an exponentially large truth table or a fixed-width
                        // integer mask.
                        let index = request
                            .metadata()
                            .seed()
                            .wrapping_add(
                                request.metadata().sequence_index(),
                            )
                            % request.problem_size();

                        vec![index]
                    }
                };

                DeutschJozsaOracle::BalancedParity { qubits }
            }

            _ => {
                return Err(invalid_configuration(
                    "oracle",
                    "oracle must be constant_zero, constant_one, or balanced",
                ));
            }
        };

        oracle.validate(request.problem_size())?;

        Ok(oracle)
    }

    /// Generates a typed workload description without constructing the
    /// canonical Quantum IR.
    ///
    /// This is useful for:
    ///
    /// - validation;
    /// - resource estimation;
    /// - testing;
    /// - reporting;
    /// - benchmark planning.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<DeutschJozsaWorkloadDescription> {
        let oracle = self.oracle_from_request(request)?;

        DeutschJozsaWorkloadDescription::new(
            request.problem_size(),
            oracle,
        )
    }

    /// Generates the canonical Quantum IR circuit for a request.
    ///
    /// The returned circuit has:
    ///
    /// ```text
    /// problem_size + 1
    /// ```
    ///
    /// logical qubits and:
    ///
    /// ```text
    /// problem_size
    /// ```
    ///
    /// logical classical bits.
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let description = self.describe(request)?;

        let total_qubits = description
            .input_qubits
            .checked_add(description.ancilla_qubits)
            .ok_or_else(|| numerical_overflow(
                "Deutsch–Jozsa total qubit count",
            ))?;

        let mut circuit = QuantumCircuit::new(
            total_qubits,
            description.classical_bits,
        )
        .map_err(|error| circuit_error(
            "unable to construct Deutsch–Jozsa Quantum IR circuit",
            error,
        ))?;

        circuit
            .set_name(Some(format!(
                "deutsch_jozsa_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| circuit_error(
                "unable to assign Deutsch–Jozsa circuit name",
                error,
            ))?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.deutsch_jozsa"
                    .to_owned(),
            ))
            .map_err(|error| circuit_error(
                "unable to assign Deutsch–Jozsa circuit source",
                error,
            ))?;

        let ancilla = description.input_qubits;

        // ---------------------------------------------------------------------
        // State preparation
        // ---------------------------------------------------------------------
        //
        // Input register:
        //
        //     |0> --H--
        //
        // Ancilla:
        //
        //     |0> --X--H--
        //
        // This creates |-> on the ancilla for phase kickback.
        //
        // The ordering is deterministic: q0, q1, ..., q(n-1), ancilla.

        for input in 0..description.input_qubits {
            circuit.push(single_qubit_gate(
                GateKind::H,
                input,
            )?)
            .map_err(|error| circuit_error(
                "unable to append Deutsch–Jozsa input Hadamard",
                error,
            ))?;
        }

        circuit
            .push(single_qubit_gate(GateKind::X, ancilla)?)
            .map_err(|error| circuit_error(
                "unable to append Deutsch–Jozsa ancilla X gate",
                error,
            ))?;

        circuit
            .push(single_qubit_gate(GateKind::H, ancilla)?)
            .map_err(|error| circuit_error(
                "unable to append Deutsch–Jozsa ancilla Hadamard",
                error,
            ))?;

        // ---------------------------------------------------------------------
        // Oracle
        // ---------------------------------------------------------------------

        match &description.oracle {
            DeutschJozsaOracle::ConstantZero => {
                // Identity oracle: no logical operation is necessary.
            }

            DeutschJozsaOracle::ConstantOne => {
                // Applying X to |-> produces the required global phase flip.
                circuit
                    .push(single_qubit_gate(GateKind::X, ancilla)?)
                    .map_err(|error| circuit_error(
                        "unable to append Deutsch–Jozsa constant-one oracle",
                        error,
                    ))?;
            }

            DeutschJozsaOracle::BalancedParity { qubits } => {
                for &input in qubits {
                    circuit
                        .push(two_qubit_gate(
                            GateKind::CX,
                            input,
                            ancilla,
                        )?)
                        .map_err(|error| circuit_error(
                            "unable to append Deutsch–Jozsa balanced oracle",
                            error,
                        ))?;
                }
            }
        }

        // ---------------------------------------------------------------------
        // Interference / classification stage
        // ---------------------------------------------------------------------

        for input in 0..description.input_qubits {
            circuit
                .push(single_qubit_gate(
                    GateKind::H,
                    input,
                )?)
                .map_err(|error| circuit_error(
                    "unable to append Deutsch–Jozsa final Hadamard",
                    error,
                ))?;
        }

        // ---------------------------------------------------------------------
        // Measurement
        // ---------------------------------------------------------------------
        //
        // The oracle ancilla is intentionally not measured. The algorithm's
        // classification is determined entirely from the input register.
        //
        // Logical input q_i -> classical bit c_i.
        //
        // This gives a one-to-one deterministic logical measurement mapping
        // and avoids backend-dependent register conventions inside the
        // generator.

        for input in 0..description.input_qubits {
            circuit
                .push(measurement_gate(input, input)?)
                .map_err(|error| circuit_error(
                    "unable to append Deutsch–Jozsa measurement",
                    error,
                ))?;
        }

        // The circuit has already passed per-operation validation through
        // `push`. The final validation is nevertheless useful because this
        // boundary may eventually receive transformations from external
        // generation layers.
        circuit
            .validate()
            .map_err(|error| circuit_error(
                "generated Deutsch–Jozsa circuit failed final validation",
                error,
            ))?;

        Ok(circuit)
    }

    /// Generates a complete canonical application workload.
    ///
    /// This is the direct application-specific construction API used by the
    /// `ApplicationBenchmarkGenerator` implementation below.
    pub fn generate_application_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let oracle = self.oracle_from_request(request)?;
        let description = DeutschJozsaWorkloadDescription::new(
            request.problem_size(),
            oracle.clone(),
        )?;

        let circuit = self.generate_circuit(request)?;

        let circuit_workload = CircuitWorkload::from_circuit(
            circuit,
            request.instance_id().clone(),
        )
        .map_err(|error| workload_error(
            "unable to create Deutsch–Jozsa circuit workload",
            error,
        ))?;

        let mut workload = ApplicationWorkload::new(
            DEUTSCH_JOZSA_APPLICATION_ID,
            request.instance_id().clone(),
            request.problem_size(),
        )
        .map_err(|error| workload_error(
            "unable to create Deutsch–Jozsa application workload",
            error,
        ))?
        .with_circuit(circuit_workload);

        // ---------------------------------------------------------------------
        // Canonical application metadata
        // ---------------------------------------------------------------------
        //
        // The universal workload model intentionally does not know about
        // Deutsch–Jozsa-specific fields. We therefore encode the application
        // contract as bounded ApplicationParameter values.
        //
        // The strongly typed `DeutschJozsaWorkloadDescription` remains
        // available to Rust callers that need structured access.

        add_parameter(
            &mut workload,
            "oracle_kind",
            oracle.kind_id(),
        )?;

        add_parameter(
            &mut workload,
            "expected_classification",
            description.expected_classification.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "expected_measurement",
            &description.expected_measurement,
        )?;

        add_parameter(
            &mut workload,
            "input_qubits",
            &description.input_qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "ancilla_qubits",
            &description.ancilla_qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_operation_count",
            &description.logical_operation_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_two_qubit_gate_count",
            &description.logical_two_qubit_gate_count.to_string(),
        )?;

        if let Some(qubits) = oracle.balanced_qubits_string() {
            add_parameter(
                &mut workload,
                "balanced_qubits",
                &qubits,
            )?;
        }

        add_parameter(
            &mut workload,
            "generator_version",
            DEUTSCH_JOZSA_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &DEUTSCH_JOZSA_GENERATOR_REVISION.to_string(),
        )?;

        Ok(workload)
    }
}

impl ApplicationBenchmarkGenerator for DeutschJozsaBenchmarkGenerator {
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        if request.application_id()
            != DEUTSCH_JOZSA_APPLICATION_ID
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "deutsch_jozsa.application_id".to_owned(),
                reason: "Deutsch–Jozsa generator requires application_id \
                          `deutsch_jozsa`"
                    .to_owned(),
            });
        }

        if request.problem_size() == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "Deutsch–Jozsa requires at least one input qubit",
            ));
        }

        // Validate the common application-generator contract.
        request.validate()?;

        // Force validation of the application-specific parameter vocabulary
        // before any Quantum IR allocation occurs.
        self.oracle_from_request_unchecked_common(request)?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.generate_application_workload(request)
    }
}

impl DeutschJozsaBenchmarkGenerator {
    /// Validates the application-specific request without recursively calling
    /// the public `validate()` implementation.
    ///
    /// This method exists to keep validation order explicit:
    ///
    /// ```text
    /// common request validation
    ///       ↓
    /// application parameter validation
    ///       ↓
    /// oracle validation
    ///       ↓
    /// circuit allocation
    /// ```
    fn oracle_from_request_unchecked_common(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        let _ = self.oracle_from_request_after_common_validation(request)?;
        Ok(())
    }

    /// Internal oracle parser that assumes the common generator request has
    /// already been validated.
    fn oracle_from_request_after_common_validation(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<DeutschJozsaOracle> {
        let mut oracle_kind: Option<&str> = None;
        let mut balanced_qubits: Option<&str> = None;

        for parameter in request.parameters() {
            match parameter.name() {
                "oracle" => {
                    if oracle_kind.is_some() {
                        return Err(invalid_configuration(
                            "oracle",
                            "duplicate oracle parameter",
                        ));
                    }

                    oracle_kind = Some(parameter.value());
                }

                "balanced_qubits" => {
                    if balanced_qubits.is_some() {
                        return Err(invalid_configuration(
                            "balanced_qubits",
                            "duplicate balanced_qubits parameter",
                        ));
                    }

                    balanced_qubits = Some(parameter.value());
                }

                _ => {
                    return Err(invalid_configuration(
                        "application_parameter",
                        "unknown Deutsch–Jozsa application parameter",
                    ));
                }
            }
        }

        let kind = oracle_kind.unwrap_or("balanced");

        let oracle = match kind {
            "constant_zero" => {
                if balanced_qubits.is_some() {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "balanced_qubits is only valid for a balanced oracle",
                    ));
                }

                DeutschJozsaOracle::ConstantZero
            }

            "constant_one" => {
                if balanced_qubits.is_some() {
                    return Err(invalid_configuration(
                        "balanced_qubits",
                        "balanced_qubits is only valid for a balanced oracle",
                    ));
                }

                DeutschJozsaOracle::ConstantOne
            }

            "balanced" => {
                let qubits = match balanced_qubits {
                    Some(value) => {
                        parse_balanced_qubits(
                            value,
                            request.problem_size(),
                        )?
                    }

                    None => {
                        let index = request
                            .metadata()
                            .seed()
                            .wrapping_add(
                                request.metadata().sequence_index(),
                            )
                            % request.problem_size();

                        vec![index]
                    }
                };

                DeutschJozsaOracle::BalancedParity { qubits }
            }

            _ => {
                return Err(invalid_configuration(
                    "oracle",
                    "oracle must be constant_zero, constant_one, or balanced",
                ));
            }
        };

        oracle.validate(request.problem_size())?;

        Ok(oracle)
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Parses a bounded comma-separated balanced parity qubit list.
fn parse_balanced_qubits(
    value: &str,
    input_qubits: usize,
) -> BenchmarkResult<Vec<usize>> {
    if value.trim().is_empty() {
        return Err(invalid_configuration(
            "balanced_qubits",
            "balanced_qubits must not be empty",
        ));
    }

    if value.len() > MAX_BALANCED_QUBITS_PARAMETER_BYTES {
        return Err(invalid_configuration(
            "balanced_qubits",
            "balanced_qubits parameter is too large",
        ));
    }

    let mut result = Vec::new();
    let mut seen = BTreeSet::new();

    for raw in value.split(',') {
        let token = raw.trim();

        if token.is_empty() {
            return Err(invalid_configuration(
                "balanced_qubits",
                "balanced_qubits contains an empty entry",
            ));
        }

        let index = token.parse::<usize>().map_err(|_| {
            invalid_configuration(
                "balanced_qubits",
                "balanced_qubits contains a non-numeric qubit index",
            )
        })?;

        if index >= input_qubits {
            return Err(invalid_configuration(
                "balanced_qubits",
                "balanced_qubits contains an out-of-range qubit index",
            ));
        }

        if !seen.insert(index) {
            return Err(invalid_configuration(
                "balanced_qubits",
                "balanced_qubits contains a duplicate qubit index",
            ));
        }

        result.push(index);

        if result.len() > MAX_BALANCED_QUBITS_PARAMETER_ENTRIES {
            return Err(invalid_configuration(
                "balanced_qubits",
                "too many balanced qubit entries",
            ));
        }
    }

    if result.is_empty() {
        return Err(invalid_configuration(
            "balanced_qubits",
            "balanced_qubits must contain at least one qubit",
        ));
    }

    Ok(result)
}

/// Creates a one-qubit gate with no parameters.
fn single_qubit_gate(
    kind: GateKind,
    qubit: usize,
) -> BenchmarkResult<Gate> {
    Gate::new(
        kind,
        vec![QubitId::new(qubit)],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "Deutsch–Jozsa generated invalid single-qubit gate",
            error,
        )
    })
}

/// Creates a two-qubit gate with no parameters.
fn two_qubit_gate(
    kind: GateKind,
    first: usize,
    second: usize,
) -> BenchmarkResult<Gate> {
    if first == second {
        return Err(invalid_configuration(
            "oracle",
            "Deutsch–Jozsa two-qubit gate cannot target the same logical qubit",
        ));
    }

    Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
        ],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "Deutsch–Jozsa generated invalid two-qubit gate",
            error,
        )
    })
}

/// Creates a computational-basis measurement gate.
fn measurement_gate(
    qubit: usize,
    classical_bit: usize,
) -> BenchmarkResult<Gate> {
    Gate::new(
        GateKind::Measure,
        vec![QubitId::new(qubit)],
        Vec::new(),
        Some(classical_bit),
        Some(Measurement::new(
            QubitId::new(qubit),
            ClassicalBitId::new(classical_bit),
        )),
    )
    .map_err(|error| {
        invalid_workload(
            "Deutsch–Jozsa generated invalid measurement gate",
            error,
        )
    })
}

/// Adds an application parameter through the canonical workload API.
fn add_parameter(
    workload: &mut ApplicationWorkload,
    name: &str,
    value: &str,
) -> BenchmarkResult<()> {
    let parameter = ApplicationParameter::new(
        name,
        value,
    )
    .map_err(|error| {
        workload_error(
            "unable to encode Deutsch–Jozsa application metadata",
            error,
        )
    })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to attach Deutsch–Jozsa application metadata",
                error,
            )
        })
}

/// Converts a configuration problem into the canonical benchmark error type.
fn invalid_configuration(
    field: &'static str,
    reason: &'static str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Converts a workload-generation problem into the canonical benchmark error
/// vocabulary.
fn invalid_workload(
    reason: &'static str,
    error: impl std::fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: DEUTSCH_JOZSA_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a canonical workload-model error.
fn workload_error(
    reason: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: DEUTSCH_JOZSA_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a Quantum IR construction error into the benchmark error boundary.
fn circuit_error(
    reason: &'static str,
    error: impl std::fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: DEUTSCH_JOZSA_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Creates a canonical numerical-overflow error.
fn numerical_overflow(
    operation: &'static str,
) -> BenchmarkError {
    BenchmarkError::NumericalOverflow {
        operation: operation.to_owned(),
        value: None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        problem_size: usize,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            DEUTSCH_JOZSA_APPLICATION_ID,
            WorkloadId::new("instance_0")
                .expect("test workload ID must be valid"),
            problem_size,
            42,
        )
        .expect("test request must be valid")
        .with_generator_revision(
            DEUTSCH_JOZSA_GENERATOR_REVISION,
        )
    }

    fn request_with_parameters(
        problem_size: usize,
        parameters: Vec<(&str, &str)>,
    ) -> ApplicationGenerationRequest {
        let mut request = request(problem_size);

        for (name, value) in parameters {
            request = request
                .with_parameter(
                    ApplicationParameter::new(
                        name,
                        value,
                    )
                    .expect("test parameter must be valid"),
                )
                .expect("test parameter must be accepted");
        }

        request
    }

    #[test]
    fn default_generator_descriptor_is_stable() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        assert_eq!(
            generator.descriptor().generator_id(),
            DEUTSCH_JOZSA_BENCHMARK_ID
        );

        assert_eq!(
            generator.descriptor().application_id(),
            DEUTSCH_JOZSA_APPLICATION_ID
        );

        assert_eq!(
            generator.descriptor().version(),
            DEUTSCH_JOZSA_GENERATOR_VERSION
        );

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::Deterministic
                )
        );

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::ClassicallyVerifiable
                )
        );
    }

    #[test]
    fn default_oracle_is_balanced() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request(4);

        let oracle = generator
            .oracle_from_request(&request)
            .expect("default oracle must be valid");

        assert_eq!(
            oracle.classification(),
            DeutschJozsaClassification::Balanced
        );

        assert!(oracle.is_balanced());

        match oracle {
            DeutschJozsaOracle::BalancedParity { qubits } => {
                assert_eq!(qubits.len(), 1);
                assert!(qubits[0] < 4);
            }

            _ => panic!("default oracle must be balanced"),
        }
    }

    #[test]
    fn constant_zero_oracle_is_supported() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            3,
            vec![("oracle", "constant_zero")],
        );

        let oracle = generator
            .oracle_from_request(&request)
            .expect("constant-zero oracle must be valid");

        assert_eq!(
            oracle,
            DeutschJozsaOracle::ConstantZero
        );

        assert_eq!(
            oracle.classification(),
            DeutschJozsaClassification::Constant
        );
    }

    #[test]
    fn constant_one_oracle_is_supported() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            3,
            vec![("oracle", "constant_one")],
        );

        let oracle = generator
            .oracle_from_request(&request)
            .expect("constant-one oracle must be valid");

        assert_eq!(
            oracle,
            DeutschJozsaOracle::ConstantOne
        );

        assert_eq!(
            oracle.classification(),
            DeutschJozsaClassification::Constant
        );
    }

    #[test]
    fn explicit_balanced_oracle_is_validated() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            6,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "0,2,5"),
            ],
        );

        let oracle = generator
            .oracle_from_request(&request)
            .expect("balanced oracle must be valid");

        assert_eq!(
            oracle,
            DeutschJozsaOracle::BalancedParity {
                qubits: vec![0, 2, 5],
            }
        );

        assert!(oracle.is_balanced());
    }

    #[test]
    fn balanced_oracle_rejects_empty_subset() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            4,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", ""),
            ],
        );

        assert!(
            generator
                .oracle_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn balanced_oracle_rejects_out_of_range_qubit() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            4,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "0,4"),
            ],
        );

        assert!(
            generator
                .oracle_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn balanced_oracle_rejects_duplicate_qubits() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            4,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "0,2,2"),
            ],
        );

        assert!(
            generator
                .oracle_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn unknown_application_parameter_is_rejected() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            4,
            vec![("oracl", "balanced")],
        );

        assert!(
            generator
                .generate_application_workload(&request)
                .is_err()
        );
    }

    #[test]
    fn duplicate_oracle_parameter_is_rejected() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            4,
            vec![
                ("oracle", "balanced"),
                ("oracle", "constant_zero"),
            ],
        );

        assert!(
            generator
                .generate_application_workload(&request)
                .is_err()
        );
    }

    #[test]
    fn balanced_circuit_has_expected_structure() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            3,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "0,2"),
            ],
        );

        let circuit = generator
            .generate_circuit(&request)
            .expect("balanced circuit must generate");

        // 3 input qubits + 1 ancilla.
        assert_eq!(circuit.num_qubits(), 4);

        // One classical result per input qubit.
        assert_eq!(circuit.num_classical_bits(), 3);

        // Preparation:
        //   3 H input + X ancilla + H ancilla = 5
        //
        // Oracle:
        //   2 CX = 2
        //
        // Final H:
        //   3
        //
        // Measurements:
        //   3
        //
        // Total = 13.
        assert_eq!(circuit.len(), 13);

        let two_qubit_count = circuit
            .operations()
            .iter()
            .filter(|gate| {
                gate.kind() == GateKind::CX
            })
            .count();

        assert_eq!(two_qubit_count, 2);

        let measurement_count = circuit
            .operations()
            .iter()
            .filter(|gate| {
                gate.kind() == GateKind::Measure
            })
            .count();

        assert_eq!(measurement_count, 3);

        assert!(
            circuit
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn constant_zero_has_no_oracle_gate() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            3,
            vec![("oracle", "constant_zero")],
        );

        let circuit = generator
            .generate_circuit(&request)
            .expect("constant-zero circuit must generate");

        assert_eq!(
            circuit
                .operations()
                .iter()
                .filter(|gate| {
                    gate.kind() == GateKind::CX
                        || (
                            gate.kind() == GateKind::X
                                && gate.qubit()
                                    .map(|qubit| {
                                        qubit.index()
                                            == 3
                                    })
                                    .unwrap_or(false)
                        )
                })
                .count(),
            1
        );
    }

    #[test]
    fn constant_one_has_ancilla_oracle_x() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            2,
            vec![("oracle", "constant_one")],
        );

        let circuit = generator
            .generate_circuit(&request)
            .expect("constant-one circuit must generate");

        let ancilla_x_count = circuit
            .operations()
            .iter()
            .filter(|gate| {
                gate.kind() == GateKind::X
                    && gate
                        .qubit()
                        .map(|qubit| qubit.index() == 2)
                        .unwrap_or(false)
            })
            .count();

        // One X prepares |1>; one X implements constant-one oracle.
        assert_eq!(ancilla_x_count, 2);
    }

    #[test]
    fn measurements_map_input_qubits_to_matching_classical_bits() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request(4);

        let circuit = generator
            .generate_circuit(&request)
            .expect("circuit must generate");

        let measurements: Vec<_> = circuit
            .operations()
            .iter()
            .filter(|gate| {
                gate.kind() == GateKind::Measure
            })
            .collect();

        assert_eq!(measurements.len(), 4);

        for (index, gate) in measurements.iter().enumerate() {
            assert_eq!(
                gate.qubits()[0].index(),
                index
            );

            assert_eq!(
                gate.classical_target(),
                Some(index)
            );

            assert_eq!(
                gate.measurement()
                    .expect("measurement payload must exist")
                    .qubit()
                    .index(),
                index
            );

            assert_eq!(
                gate.measurement()
                    .expect("measurement payload must exist")
                    .classical_bit()
                    .index(),
                index
            );
        }
    }

    #[test]
    fn workload_contains_canonical_application_metadata() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let request = request_with_parameters(
            3,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "0,2"),
            ],
        );

        let workload = generator
            .generate_application_workload(&request)
            .expect("workload must generate");

        assert_eq!(
            workload.application_id(),
            DEUTSCH_JOZSA_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            3
        );

        assert!(
            workload.circuit().is_some()
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "oracle_kind"
                        && parameter.value()
                            == "balanced"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "expected_classification"
                        && parameter.value()
                            == "balanced"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "expected_measurement"
                        && parameter.value()
                            == "000"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "balanced_qubits"
                        && parameter.value()
                            == "0,2"
                })
        );
    }

    #[test]
    fn generation_is_reproducible_for_identical_requests() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let first = request_with_parameters(
            5,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "1,4"),
            ],
        );

        let second = request_with_parameters(
            5,
            vec![
                ("oracle", "balanced"),
                ("balanced_qubits", "1,4"),
            ],
        );

        let first_circuit = generator
            .generate_circuit(&first)
            .expect("first circuit must generate");

        let second_circuit = generator
            .generate_circuit(&second)
            .expect("second circuit must generate");

        assert_eq!(
            first_circuit,
            second_circuit
        );
    }

    #[test]
    fn different_sequence_indices_can_select_different_default_balanced_qubits() {
        let generator =
            DeutschJozsaBenchmarkGenerator::new()
                .expect("generator must construct");

        let first = request(4);

        let second = request(4)
            .with_sequence_index(1);

        let first_oracle = generator
            .oracle_from_request(&first)
            .expect("first oracle must generate");

        let second_oracle = generator
            .oracle_from_request(&second)
            .expect("second oracle must generate");

        assert_ne!(
            first_oracle,
            second_oracle
        );
    }

    #[test]
    fn description_has_linear_resource_counts() {
        let oracle =
            DeutschJozsaOracle::BalancedParity {
                qubits: vec![0, 2, 4],
            };

        let description =
            DeutschJozsaWorkloadDescription::new(
                5,
                oracle,
            )
            .expect("description must generate");

        assert_eq!(
            description.total_qubits(),
            6
        );

        assert_eq!(
            description.classical_bits,
            5
        );

        // 2n + 1 Hadamards/X preparation
        // + |S| oracle CX
        // + n final H
        // + n measurements.
        //
        // 2*5 + 1 + 3 + 5 + 5 = 24.
        assert_eq!(
            description.logical_operation_count,
            24
        );

        assert_eq!(
            description.logical_two_qubit_gate_count,
            3
        );
    }

    #[test]
    fn zero_problem_size_is_rejected() {
        let result =
            ApplicationGenerationRequest::new(
                DEUTSCH_JOZSA_APPLICATION_ID,
                WorkloadId::new("instance_0")
                    .expect("test workload ID"),
                0,
                42,
            );

        assert!(result.is_err());
    }

    #[test]
    fn balanced_oracle_is_never_constructed_from_an_empty_set() {
        let oracle =
            DeutschJozsaOracle::BalancedParity {
                qubits: Vec::new(),
            };

        assert!(
            oracle.validate(4).is_err()
        );

        assert!(
            !oracle.is_balanced()
        );
    }
}