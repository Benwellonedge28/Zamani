//! Zamani Quantum Benchmarking — Quantum Fourier Transform Application Benchmark
//!
//! Production application-benchmark generator for the Quantum Fourier
//! Transform (QFT).
//!
//! # Purpose
//!
//! This module constructs a deterministic, backend-independent QFT benchmark
//! workload using Zamani's canonical Quantum IR and the existing application
//! benchmarking contract.
//!
//! It deliberately does NOT:
//!
//! - execute circuits;
//! - select a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - calculate fidelity;
//! - calculate runtime metrics;
//! - perform tomography;
//! - depend on a simulator;
//! - implement hardware-specific gates;
//! - duplicate Quantum IR;
//! - parse Zamani source code;
//! - perform filesystem or network I/O.
//!
//! Those responsibilities belong to the existing execution, hardware,
//! compiler, metrics, analysis, and frontend layers.
//!
//! # Architectural position
//!
//! ```text
//! ApplicationGenerationRequest
//!              │
//!              ▼
//!       QftBenchmarkGenerator
//!              │
//!       ┌──────┴─────────┐
//!       ▼                ▼
//! QFT description   QuantumCircuit
//!       │                │
//!       └───────┬────────┘
//!               ▼
//!       ApplicationWorkload
//!               │
//!               ▼
//!        BenchmarkExperiment
//!               │
//!               ▼
//!         BenchmarkExecutor
//!               │
//!               ▼
//!          Observations
//!               │
//!               ▼
//!        Application analysis
//! ```
//!
//! # Mathematical definition
//!
//! For `N = 2^n`, the QFT is:
//!
//! ```text
//! QFT_N |x> = 1/sqrt(N) *
//!          sum(y = 0 .. N-1)
//!          exp(2πi x y / N) |y>
//! ```
//!
//! The generated exact QFT uses the standard decomposition:
//!
//! ```text
//! H(q_i)
//! controlled-P(π / 2^(j-i)) (q_j, q_i)
//! ```
//!
//! followed by a bit-reversal permutation implemented with SWAP gates when
//! `swaps = true`.
//!
//! The Quantum IR does not currently expose a native controlled-phase gate.
//! Therefore a controlled phase is synthesized from the existing `RZ` and
//! `CX` gates:
//!
//! ```text
//! RZ(θ/2) on control
//! RZ(θ/2) on target
//! CX(control, target)
//! RZ(-θ/2) on target
//! CX(control, target)
//! ```
//!
//! This decomposition implements the controlled-phase operation up to a
//! physically irrelevant global phase. Global phase is not represented by the
//! current logical Quantum IR, and therefore this is the correct IR-level
//! representation of the QFT operation.
//!
//! # Approximation
//!
//! `approximation_degree` is an explicit benchmark parameter.
//!
//! It removes exactly that many of the smallest controlled-phase operations,
//! using a deterministic ordering. The smallest rotations are those with the
//! largest qubit separation. Ties are resolved deterministically by
//! `(control, target)` ordering.
//!
//! `approximation_degree = 0` is the exact QFT decomposition.
//!
//! Approximation is never silently enabled.
//!
//! # Input state
//!
//! The benchmark supports computational-basis input states through the
//! `input_basis` parameter.
//!
//! `input_basis = 0` is the default.
//!
//! For a basis input |x>, the ideal QFT output has uniform computational-basis
//! probabilities. The generator records this expected property in workload
//! metadata for downstream application analysis.
//!
//! The generator does not itself determine whether an executed result matches
//! the ideal distribution.
//!
//! # Output permutation
//!
//! With:
//!
//! ```text
//! swaps = true
//! ```
//!
//! the generated circuit implements the conventional QFT including the final
//! bit-reversal permutation.
//!
//! With:
//!
//! ```text
//! swaps = false
//! ```
//!
//! the circuit omits the physical SWAP layer. This is intentionally exposed as
//! `qft_with_reversal` semantics rather than being silently presented as the
//! conventional QFT.
//!
//! A downstream classical analysis layer may compensate for this permutation
//! when the application permits it.
//!
//! # Parameters
//!
//! Supported application parameters are:
//!
//! ```text
//! inverse = true | false
//! swaps = true | false
//! approximation_degree = non-negative integer
//! input_basis = non-negative integer
//! ```
//!
//! Defaults:
//!
//! ```text
//! inverse = false
//! swaps = true
//! approximation_degree = 0
//! input_basis = 0
//! ```
//!
//! Unknown parameters are rejected.
//! Duplicate parameters are rejected.
//! Malformed values are rejected.
//!
//! # Resource safety
//!
//! QFT has O(n²) controlled-phase structure. This implementation deliberately
//! bounds the application benchmark at `MAX_QFT_QUBITS` before constructing
//! any quadratic data structure.
//!
//! The implementation also:
//!
//! - uses checked arithmetic for resource counts;
//! - validates every parameter before circuit allocation;
//! - validates every generated gate through `Gate::new`;
//! - validates every circuit mutation through `QuantumCircuit::push`;
//! - performs final whole-circuit validation;
//! - never constructs a 2^n truth table;
//! - never allocates a state vector;
//! - never executes the workload;
//! - never uses global randomness;
//! - never uses system time as generation entropy.
//!
//! # Reproducibility
//!
//! QFT generation is deterministic.
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
//! produce semantically identical QFT workloads.
//!
//! The QFT itself does not require randomness, so the seed is recorded only
//! through the common generation contract and provenance system.
//!
//! # Complexity
//!
//! Let `n` be the number of QFT qubits.
//!
//! Exact controlled-phase count:
//!
//! ```text
//! n(n-1)/2
//! ```
//!
//! Each controlled phase is decomposed into:
//!
//! ```text
//! 3 RZ + 2 CX
//! ```
//!
//! Therefore exact logical gate count is:
//!
//! ```text
//! n                         Hadamards
//! + 5 * n(n-1)/2            controlled-phase decomposition
//! + floor(n/2)              final swaps when enabled
//! + popcount(input_basis)   input preparation
//! + n                       measurements
//! ```
//!
//! This is computed with checked arithmetic before the circuit is allocated.
//!
//! # Integration contract
//!
//! This file integrates with the existing repository contracts:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorDescriptor
//!     └── ApplicationGeneratorCapability
//!
//! benchmarking::core::workload
//!     ├── ApplicationWorkload
//!     ├── ApplicationParameter
//!     ├── CircuitWorkload
//!     └── WorkloadId
//!
//! quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── GateKind
//!     ├── Parameter
//!     ├── Measurement
//!     ├── QubitId
//!     └── ClassicalBitId
//! ```
//!
//! The only namespace integration required after this file exists is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod qft;
//! ```
//!
//! No existing file needs to be modified to change QFT generation semantics.
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
//! No external crates are required.

use std::collections::BTreeSet;
use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    CircuitWorkload,
    WorkloadError,
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
    parameter::Parameter,
    qubit::QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable benchmark identifier.
pub const QFT_BENCHMARK_ID: &str = "qft";

/// Stable application identifier.
pub const QFT_APPLICATION_ID: &str = "qft";

/// Generator implementation version.
pub const QFT_GENERATOR_VERSION: &str = "1.0.0";

/// Reproducibility revision of the generator.
pub const QFT_GENERATOR_REVISION: u32 = 1;

/// Human-readable application name.
pub const QFT_NAME: &str = "Quantum Fourier Transform";

/// Generator schema version.
pub const QFT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Resource limits
// =============================================================================

/// Maximum number of QFT logical qubits generated by this application
/// benchmark.
///
/// The bound is deliberately conservative because the circuit contains
/// quadratic controlled-phase structure.
pub const MAX_QFT_QUBITS: usize = 64;

/// Maximum approximation degree.
///
/// There can be at most n(n-1)/2 controlled-phase operations and n is bounded
/// by MAX_QFT_QUBITS.
pub const MAX_APPROXIMATION_DEGREE: usize =
    (MAX_QFT_QUBITS * (MAX_QFT_QUBITS - 1)) / 2;

/// Maximum textual length of an application parameter value.
pub const MAX_QFT_PARAMETER_VALUE_BYTES: usize = 128;

// =============================================================================
// QFT configuration
// =============================================================================

/// Strongly typed QFT configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QftConfiguration {
    /// Number of logical QFT qubits.
    pub qubits: usize,

    /// Whether to construct the inverse QFT.
    pub inverse: bool,

    /// Whether to include the final bit-reversal SWAP layer.
    pub swaps: bool,

    /// Number of smallest controlled-phase operations to omit.
    pub approximation_degree: usize,

    /// Computational-basis input state encoded as an unsigned integer.
    pub input_basis: u64,
}

impl QftConfiguration {
    /// Creates the exact conventional forward QFT configuration.
    pub fn new(qubits: usize) -> BenchmarkResult<Self> {
        let configuration = Self {
            qubits,
            inverse: false,
            swaps: true,
            approximation_degree: 0,
            input_basis: 0,
        };

        configuration.validate()?;

        Ok(configuration)
    }

    /// Validates the complete QFT configuration.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.qubits == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "QFT requires at least one logical qubit",
            ));
        }

        if self.qubits > MAX_QFT_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "QFT problem size exceeds the production QFT resource limit",
            ));
        }

        if self.approximation_degree > MAX_APPROXIMATION_DEGREE {
            return Err(invalid_configuration(
                "approximation_degree",
                "approximation degree exceeds the maximum QFT controlled-phase count",
            ));
        }

        let controlled_phase_count = exact_controlled_phase_count(self.qubits)?;

        if self.approximation_degree > controlled_phase_count {
            return Err(invalid_configuration(
                "approximation_degree",
                "approximation degree cannot exceed the number of controlled-phase operations",
            ));
        }

        if self.qubits < 64 {
            let maximum_basis = 1u64
                .checked_shl(self.qubits as u32)
                .ok_or_else(|| numerical_overflow("QFT basis-state range"))?;

            if self.input_basis >= maximum_basis {
                return Err(invalid_configuration(
                    "input_basis",
                    "input basis state is outside the QFT computational basis",
                ));
            }
        }

        Ok(())
    }

    /// Returns the exact number of controlled-phase instances before
    /// approximation.
    pub fn exact_controlled_phase_count(&self) -> BenchmarkResult<usize> {
        exact_controlled_phase_count(self.qubits)
    }

    /// Returns the number of controlled-phase instances retained after
    /// approximation.
    pub fn retained_controlled_phase_count(&self) -> BenchmarkResult<usize> {
        self.exact_controlled_phase_count()?
            .checked_sub(self.approximation_degree)
            .ok_or_else(|| numerical_overflow("retained QFT controlled-phase count"))
    }

    /// Returns the number of omitted controlled-phase instances.
    #[must_use]
    pub const fn omitted_controlled_phase_count(&self) -> usize {
        self.approximation_degree
    }

    /// Returns the number of input-preparation X gates.
    #[must_use]
    pub fn input_preparation_x_count(&self) -> usize {
        self.input_basis.count_ones() as usize
    }

    /// Returns the number of final SWAP operations.
    #[must_use]
    pub const fn swap_count(&self) -> usize {
        if self.swaps {
            self.qubits / 2
        } else {
            0
        }
    }

    /// Returns the logical operation count of the generated benchmark.
    pub fn logical_operation_count(&self) -> BenchmarkResult<usize> {
        let hadamards = self.qubits;

        let controlled_phases = self
            .retained_controlled_phase_count()?
            .checked_mul(5)
            .ok_or_else(|| numerical_overflow("QFT controlled-phase decomposition"))?;

        let input_x = self.input_preparation_x_count();
        let swaps = self.swap_count();
        let measurements = self.qubits;

        hadamards
            .checked_add(controlled_phases)
            .and_then(|value| value.checked_add(input_x))
            .and_then(|value| value.checked_add(swaps))
            .and_then(|value| value.checked_add(measurements))
            .ok_or_else(|| numerical_overflow("QFT logical operation count"))
    }

    /// Returns the number of logical two-qubit gates.
    pub fn logical_two_qubit_gate_count(&self) -> BenchmarkResult<usize> {
        let controlled_phases = self
            .retained_controlled_phase_count()?
            .checked_mul(2)
            .ok_or_else(|| numerical_overflow("QFT two-qubit gate count"))?;

        controlled_phases
            .checked_add(self.swap_count())
            .ok_or_else(|| numerical_overflow("QFT total two-qubit gate count"))
    }

    /// Returns the maximum basis value representable by the configuration.
    ///
    /// For 64 qubits the maximum value is `u64::MAX`.
    pub fn basis_dimension(&self) -> BenchmarkResult<u64> {
        if self.qubits == 64 {
            Ok(u64::MAX)
        } else {
            1u64
                .checked_shl(self.qubits as u32)
                .ok_or_else(|| numerical_overflow("QFT basis dimension"))
        }
    }

    /// Returns whether this is the conventional exact QFT.
    #[must_use]
    pub const fn is_exact_conventional_qft(&self) -> bool {
        !self.inverse
            && self.swaps
            && self.approximation_degree == 0
    }
}

// =============================================================================
// Strongly typed workload description
// =============================================================================

/// Strongly typed description of one QFT application benchmark instance.
///
/// The universal `ApplicationWorkload` remains the canonical workload
/// representation. This structure exists so Rust callers do not have to parse
/// textual `ApplicationParameter` values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QftWorkloadDescription {
    /// Number of logical QFT qubits.
    pub qubits: usize,

    /// Whether the inverse QFT is generated.
    pub inverse: bool,

    /// Whether the final reversal SWAP layer is generated.
    pub swaps: bool,

    /// Number of omitted controlled-phase operations.
    pub approximation_degree: usize,

    /// Number of exact controlled-phase operations.
    pub exact_controlled_phase_count: usize,

    /// Number of retained controlled-phase operations.
    pub retained_controlled_phase_count: usize,

    /// Computational-basis input value.
    pub input_basis: u64,

    /// Number of logical operations.
    pub logical_operation_count: usize,

    /// Number of logical two-qubit operations.
    pub logical_two_qubit_gate_count: usize,

    /// Number of input-preparation X gates.
    pub input_preparation_x_count: usize,

    /// Number of Hadamard gates.
    pub hadamard_count: usize,

    /// Number of SWAP operations.
    pub swap_count: usize,

    /// Number of measurement operations.
    pub measurement_count: usize,

    /// Expected ideal output distribution.
    ///
    /// For computational-basis input to a QFT, the ideal output probabilities
    /// are uniform over the complete computational basis.
    pub expected_output_distribution: &'static str,

    /// Stable output permutation identifier.
    pub output_permutation: &'static str,
}

impl QftWorkloadDescription {
    /// Creates a validated description.
    pub fn from_configuration(
        configuration: QftConfiguration,
    ) -> BenchmarkResult<Self> {
        configuration.validate()?;

        let exact_controlled_phase_count =
            configuration.exact_controlled_phase_count()?;

        let retained_controlled_phase_count =
            configuration.retained_controlled_phase_count()?;

        let logical_operation_count =
            configuration.logical_operation_count()?;

        let logical_two_qubit_gate_count =
            configuration.logical_two_qubit_gate_count()?;

        Ok(Self {
            qubits: configuration.qubits,
            inverse: configuration.inverse,
            swaps: configuration.swaps,
            approximation_degree: configuration.approximation_degree,
            exact_controlled_phase_count,
            retained_controlled_phase_count,
            input_basis: configuration.input_basis,
            logical_operation_count,
            logical_two_qubit_gate_count,
            input_preparation_x_count: configuration.input_preparation_x_count(),
            hadamard_count: configuration.qubits,
            swap_count: configuration.swap_count(),
            measurement_count: configuration.qubits,
            expected_output_distribution: "uniform",
            output_permutation: if configuration.swaps {
                "canonical"
            } else {
                "bit_reversed"
            },
        })
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production QFT application benchmark generator.
///
/// The generator is stateless and safe to share between benchmark jobs.
#[derive(Debug, Clone)]
pub struct QftBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl QftBenchmarkGenerator {
    /// Creates the canonical QFT benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = ApplicationGeneratorDescriptor::new(
            QFT_BENCHMARK_ID,
            QFT_APPLICATION_ID,
            QFT_GENERATOR_VERSION,
            "Production Quantum Fourier Transform application benchmark generator",
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

    /// Returns the generator descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Parses a common application-generation request into a QFT
    /// configuration.
    ///
    /// Unknown and duplicate application parameters are rejected.
    pub fn configuration_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QftConfiguration> {
        request.validate()?;

        if request.application_id() != QFT_APPLICATION_ID {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "qft.application_id".to_owned(),
                reason:
                    "QFT generator requires application_id `qft`"
                        .to_owned(),
            });
        }

        if request.problem_size() == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "QFT requires at least one logical qubit",
            ));
        }

        if request.problem_size() > MAX_QFT_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "QFT problem size exceeds the production QFT resource limit",
            ));
        }

        let mut inverse: Option<bool> = None;
        let mut swaps: Option<bool> = None;
        let mut approximation_degree: Option<usize> = None;
        let mut input_basis: Option<u64> = None;

        for parameter in request.parameters() {
            if parameter.value().len() > MAX_QFT_PARAMETER_VALUE_BYTES {
                return Err(invalid_configuration(
                    "application_parameter",
                    "QFT application parameter value is too large",
                ));
            }

            match parameter.name() {
                "inverse" => {
                    if inverse.is_some() {
                        return Err(invalid_configuration(
                            "inverse",
                            "duplicate inverse parameter",
                        ));
                    }

                    inverse = Some(parse_bool(
                        parameter.value(),
                        "inverse",
                    )?);
                }

                "swaps" => {
                    if swaps.is_some() {
                        return Err(invalid_configuration(
                            "swaps",
                            "duplicate swaps parameter",
                        ));
                    }

                    swaps = Some(parse_bool(
                        parameter.value(),
                        "swaps",
                    )?);
                }

                "approximation_degree" => {
                    if approximation_degree.is_some() {
                        return Err(invalid_configuration(
                            "approximation_degree",
                            "duplicate approximation_degree parameter",
                        ));
                    }

                    approximation_degree = Some(
                        parameter
                            .value()
                            .parse::<usize>()
                            .map_err(|_| {
                                invalid_configuration(
                                    "approximation_degree",
                                    "approximation_degree must be a non-negative integer",
                                )
                            })?,
                    );
                }

                "input_basis" => {
                    if input_basis.is_some() {
                        return Err(invalid_configuration(
                            "input_basis",
                            "duplicate input_basis parameter",
                        ));
                    }

                    input_basis = Some(
                        parameter
                            .value()
                            .parse::<u64>()
                            .map_err(|_| {
                                invalid_configuration(
                                    "input_basis",
                                    "input_basis must be an unsigned integer",
                                )
                            })?,
                    );
                }

                _ => {
                    return Err(invalid_configuration(
                        "application_parameter",
                        "unknown QFT application parameter",
                    ));
                }
            }
        }

        let configuration = QftConfiguration {
            qubits: request.problem_size(),
            inverse: inverse.unwrap_or(false),
            swaps: swaps.unwrap_or(true),
            approximation_degree: approximation_degree.unwrap_or(0),
            input_basis: input_basis.unwrap_or(0),
        };

        configuration.validate()?;

        Ok(configuration)
    }

    /// Returns a strongly typed resource description without constructing
    /// Quantum IR.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QftWorkloadDescription> {
        let configuration =
            self.configuration_from_request(request)?;

        QftWorkloadDescription::from_configuration(configuration)
    }

    /// Generates the canonical Quantum IR circuit.
    ///
    /// The circuit contains:
    ///
    /// - `qubits` logical qubits;
    /// - `qubits` classical measurement bits;
    /// - deterministic input preparation;
    /// - exact or approximate QFT;
    /// - optional final reversal;
    /// - computational-basis measurement.
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let configuration =
            self.configuration_from_request(request)?;

        let description =
            QftWorkloadDescription::from_configuration(configuration)?;

        let mut circuit = QuantumCircuit::new(
            configuration.qubits,
            configuration.qubits,
        )
        .map_err(|error| {
            circuit_error(
                "unable to construct QFT Quantum IR circuit",
                error,
            )
        })?;

        circuit
            .set_name(Some(format!(
                "qft_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QFT circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.qft"
                    .to_owned(),
            ))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QFT circuit source",
                    error,
                )
            })?;

        // ---------------------------------------------------------------------
        // Input-state preparation
        // ---------------------------------------------------------------------
        //
        // The default is |0...0>.
        //
        // For a requested basis state |x>, X is applied to every set bit.
        //
        // This is deterministic and does not require an exponentially sized
        // classical representation.

        append_input_basis_preparation(
            &mut circuit,
            configuration.qubits,
            configuration.input_basis,
        )?;

        // ---------------------------------------------------------------------
        // QFT / inverse QFT
        // ---------------------------------------------------------------------

        if configuration.inverse {
            append_inverse_qft(
                &mut circuit,
                &configuration,
            )?;
        } else {
            append_forward_qft(
                &mut circuit,
                &configuration,
            )?;
        }

        // ---------------------------------------------------------------------
        // Measurement
        // ---------------------------------------------------------------------
        //
        // Logical qubit i -> classical bit i.
        //
        // This one-to-one mapping keeps register interpretation independent of
        // backend-specific classical-bit ordering.

        for qubit in 0..configuration.qubits {
            circuit
                .push(measurement_gate(qubit, qubit)?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QFT measurement",
                        error,
                    )
                })?;
        }

        // ---------------------------------------------------------------------
        // Final canonical validation
        // ---------------------------------------------------------------------

        circuit
            .validate()
            .map_err(|error| {
                circuit_error(
                    "generated QFT circuit failed final validation",
                    error,
                )
            })?;

        // The description is intentionally constructed above even though the
        // circuit itself does not need it. This guarantees resource counting
        // and circuit construction share the same validated configuration.
        let _ = description;

        Ok(circuit)
    }

    /// Generates the complete canonical application workload.
    pub fn generate_application_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let configuration =
            self.configuration_from_request(request)?;

        let description =
            QftWorkloadDescription::from_configuration(configuration)?;

        let circuit = self.generate_circuit(request)?;

        let circuit_workload = CircuitWorkload::from_circuit(
            circuit,
            request.instance_id().clone(),
        )
        .map_err(|error| {
            workload_error(
                "unable to create QFT circuit workload",
                error,
            )
        })?;

        let mut workload = ApplicationWorkload::new(
            QFT_APPLICATION_ID,
            request.instance_id().clone(),
            request.problem_size(),
        )
        .map_err(|error| {
            workload_error(
                "unable to create QFT application workload",
                error,
            )
        })?
        .with_circuit(circuit_workload);

        // ---------------------------------------------------------------------
        // Canonical application metadata
        // ---------------------------------------------------------------------
        //
        // These fields are bounded through ApplicationParameter and therefore
        // can safely cross the generic benchmark/result serialization boundary.

        add_parameter(
            &mut workload,
            "application",
            QFT_APPLICATION_ID,
        )?;

        add_parameter(
            &mut workload,
            "schema_version",
            &QFT_SCHEMA_VERSION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            QFT_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &QFT_GENERATOR_REVISION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "qubits",
            &description.qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "inverse",
            if description.inverse {
                "true"
            } else {
                "false"
            },
        )?;

        add_parameter(
            &mut workload,
            "swaps",
            if description.swaps {
                "true"
            } else {
                "false"
            },
        )?;

        add_parameter(
            &mut workload,
            "approximation_degree",
            &description.approximation_degree.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "exact_controlled_phase_count",
            &description
                .exact_controlled_phase_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "retained_controlled_phase_count",
            &description
                .retained_controlled_phase_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "input_basis",
            &description.input_basis.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "input_preparation_x_count",
            &description
                .input_preparation_x_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "hadamard_count",
            &description.hadamard_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "swap_count",
            &description.swap_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "measurement_count",
            &description.measurement_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_operation_count",
            &description.logical_operation_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_two_qubit_gate_count",
            &description
                .logical_two_qubit_gate_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "expected_output_distribution",
            description.expected_output_distribution,
        )?;

        add_parameter(
            &mut workload,
            "output_permutation",
            description.output_permutation,
        )?;

        add_parameter(
            &mut workload,
            "phase_angle_convention",
            "pi_over_2_to_qubit_distance",
        )?;

        add_parameter(
            &mut workload,
            "controlled_phase_ir_decomposition",
            "rz_rz_cx_rz_cx",
        )?;

        Ok(workload)
    }
}

impl ApplicationBenchmarkGenerator for QftBenchmarkGenerator {
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        // Validate the common application-generation contract before any
        // application-specific allocation.
        request.validate()?;

        if request.application_id() != QFT_APPLICATION_ID {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "qft.application_id".to_owned(),
                reason:
                    "QFT generator requires application_id `qft`"
                        .to_owned(),
            });
        }

        // Parse and validate every application-specific parameter.
        //
        // This intentionally occurs before QuantumCircuit allocation.
        let _ = self.configuration_from_request(request)?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.generate_application_workload(request)
    }
}

// =============================================================================
// QFT construction
// =============================================================================

/// Appends the requested computational-basis input preparation.
fn append_input_basis_preparation(
    circuit: &mut QuantumCircuit,
    qubits: usize,
    input_basis: u64,
) -> BenchmarkResult<()> {
    for qubit in 0..qubits {
        let set = ((input_basis >> qubit) & 1u64) != 0;

        if !set {
            continue;
        }

        circuit
            .push(single_qubit_gate(
                GateKind::X,
                qubit,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append QFT input-state preparation",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends the conventional forward QFT.
fn append_forward_qft(
    circuit: &mut QuantumCircuit,
    configuration: &QftConfiguration,
) -> BenchmarkResult<()> {
    let qubits = configuration.qubits;

    // Build the deterministic ordering of controlled-phase operations that
    // would occur in the exact QFT.
    //
    // Each pair is represented as:
    //
    // (control = higher index, target = lower index)
    //
    // and the phase is:
    //
    // π / 2^(control - target)
    //
    // The smallest angles are the largest separations.
    let omitted = omitted_phase_pairs(
        qubits,
        configuration.approximation_degree,
    )?;

    for target in 0..qubits {
        circuit
            .push(single_qubit_gate(
                GateKind::H,
                target,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append QFT Hadamard",
                    error,
                )
            })?;

        for control in (target + 1)..qubits {
            if omitted.contains(&(control, target)) {
                continue;
            }

            let distance = control
                .checked_sub(target)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QFT controlled-phase qubit distance",
                    )
                })?;

            let angle = phase_angle(distance)?;

            append_controlled_phase(
                circuit,
                control,
                target,
                angle,
            )?;
        }
    }

    if configuration.swaps {
        append_bit_reversal_swaps(
            circuit,
            qubits,
        )?;
    }

    Ok(())
}

/// Appends the inverse QFT.
///
/// The inverse is the adjoint of the forward QFT:
///
/// 1. undo final bit reversal;
/// 2. walk the forward construction in reverse order;
/// 3. conjugate every controlled phase.
///
/// SWAP is self-inverse, so the reversal layer is unchanged.
fn append_inverse_qft(
    circuit: &mut QuantumCircuit,
    configuration: &QftConfiguration,
) -> BenchmarkResult<()> {
    let qubits = configuration.qubits;

    if configuration.swaps {
        append_bit_reversal_swaps(
            circuit,
            qubits,
        )?;
    }

    let omitted = omitted_phase_pairs(
        qubits,
        configuration.approximation_degree,
    )?;

    for target in (0..qubits).rev() {
        for control in (0..target).rev() {
            if omitted.contains(&(target, control)) {
                continue;
            }

            let distance = target
                .checked_sub(control)
                .ok_or_else(|| {
                    numerical_overflow(
                        "inverse QFT controlled-phase qubit distance",
                    )
                })?;

            let angle = phase_angle(distance)?
                .checked_neg()
                .ok_or_else(|| {
                    numerical_overflow(
                        "inverse QFT controlled-phase angle",
                    )
                })?;

            append_controlled_phase(
                circuit,
                target,
                control,
                angle,
            )?;
        }

        circuit
            .push(single_qubit_gate(
                GateKind::H,
                target,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append inverse QFT Hadamard",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends the QFT output bit-reversal permutation.
fn append_bit_reversal_swaps(
    circuit: &mut QuantumCircuit,
    qubits: usize,
) -> BenchmarkResult<()> {
    for index in 0..(qubits / 2) {
        let opposite = qubits
            .checked_sub(1)
            .and_then(|value| value.checked_sub(index))
            .ok_or_else(|| {
                numerical_overflow(
                    "QFT bit-reversal index",
                )
            })?;

        if index == opposite {
            return Err(invalid_configuration(
                "swaps",
                "QFT bit-reversal attempted to swap a qubit with itself",
            ));
        }

        circuit
            .push(two_qubit_gate(
                GateKind::SWAP,
                index,
                opposite,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append QFT bit-reversal SWAP",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends a logical controlled-phase operation.
///
/// The current Zamani Quantum IR has `CRZ` but not a native `CP` gate.
/// Therefore the decomposition uses:
///
/// ```text
/// RZ(θ/2) control
/// RZ(θ/2) target
/// CX(control,target)
/// RZ(-θ/2) target
/// CX(control,target)
/// ```
///
/// The resulting unitary differs from CP(θ) only by a global phase, which is
/// physically unobservable and is not represented by the current logical IR.
fn append_controlled_phase(
    circuit: &mut QuantumCircuit,
    control: usize,
    target: usize,
    angle: f64,
) -> BenchmarkResult<()> {
    if control == target {
        return Err(invalid_configuration(
            "qft",
            "controlled-phase control and target must differ",
        ));
    }

    let half_angle = angle / 2.0;

    circuit
        .push(parameterized_single_qubit_gate(
            GateKind::RZ,
            control,
            half_angle,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QFT controlled-phase control RZ",
                error,
            )
        })?;

    circuit
        .push(parameterized_single_qubit_gate(
            GateKind::RZ,
            target,
            half_angle,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QFT controlled-phase target RZ",
                error,
            )
        })?;

    circuit
        .push(two_qubit_gate(
            GateKind::CX,
            control,
            target,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QFT controlled-phase CX",
                error,
            )
        })?;

    circuit
        .push(parameterized_single_qubit_gate(
            GateKind::RZ,
            target,
            -half_angle,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QFT controlled-phase correction",
                error,
            )
        })?;

    circuit
        .push(two_qubit_gate(
            GateKind::CX,
            control,
            target,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QFT controlled-phase final CX",
                error,
            )
        })?;

    Ok(())
}

// =============================================================================
// Approximation
// =============================================================================

/// Returns the controlled-phase pairs omitted by `approximation_degree`.
///
/// The smallest phases correspond to the largest qubit separations.
///
/// Ordering is deterministic:
///
/// 1. descending distance;
/// 2. ascending control;
/// 3. ascending target.
///
/// This makes approximation reproducible without a floating-point threshold.
fn omitted_phase_pairs(
    qubits: usize,
    approximation_degree: usize,
) -> BenchmarkResult<BTreeSet<(usize, usize)>> {
    let total = exact_controlled_phase_count(qubits)?;

    if approximation_degree > total {
        return Err(invalid_configuration(
            "approximation_degree",
            "approximation degree exceeds the QFT controlled-phase count",
        ));
    }

    if approximation_degree == 0 {
        return Ok(BTreeSet::new());
    }

    let mut pairs = Vec::with_capacity(total);

    for target in 0..qubits {
        for control in (target + 1)..qubits {
            pairs.push((control, target));
        }
    }

    pairs.sort_by(|left, right| {
        let left_distance =
            left.0.saturating_sub(left.1);

        let right_distance =
            right.0.saturating_sub(right.1);

        right_distance
            .cmp(&left_distance)
            .then_with(|| left.0.cmp(&right.0))
            .then_with(|| left.1.cmp(&right.1))
    });

    Ok(pairs
        .into_iter()
        .take(approximation_degree)
        .collect())
}

// =============================================================================
// Resource calculations
// =============================================================================

/// Returns n(n-1)/2 with checked arithmetic.
fn exact_controlled_phase_count(
    qubits: usize,
) -> BenchmarkResult<usize> {
    if qubits == 0 {
        return Ok(0);
    }

    let n_minus_one = qubits
        .checked_sub(1)
        .ok_or_else(|| {
            numerical_overflow(
                "QFT controlled-phase count",
            )
        })?;

    let product = qubits
        .checked_mul(n_minus_one)
        .ok_or_else(|| {
            numerical_overflow(
                "QFT controlled-phase count multiplication",
            )
        })?;

    Ok(product / 2)
}

/// Returns the exact QFT phase angle for a qubit separation.
///
/// ```text
/// θ = π / 2^distance
/// ```
fn phase_angle(
    distance: usize,
) -> BenchmarkResult<f64> {
    if distance == 0 {
        return Err(invalid_configuration(
            "qft",
            "controlled-phase distance must be greater than zero",
        ));
    }

    if distance > 63 {
        return Err(invalid_configuration(
            "qft",
            "controlled-phase distance exceeds supported numeric range",
        ));
    }

    let denominator = 2f64.powi(distance as i32);
    let angle = std::f64::consts::PI / denominator;

    if !angle.is_finite() || angle <= 0.0 {
        return Err(numerical_overflow(
            "QFT controlled-phase angle",
        ));
    }

    Ok(angle)
}

// =============================================================================
// IR gate helpers
// =============================================================================

/// Creates an unparameterized single-qubit gate.
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
            "QFT generated invalid single-qubit gate",
            error,
        )
    })
}

/// Creates a parameterized single-qubit gate.
fn parameterized_single_qubit_gate(
    kind: GateKind,
    qubit: usize,
    value: f64,
) -> BenchmarkResult<Gate> {
    if !value.is_finite() {
        return Err(invalid_configuration(
            "qft",
            "QFT gate parameter must be finite",
        ));
    }

    let parameter =
        Parameter::constant(value).map_err(|error| {
            invalid_workload(
                "QFT generated invalid gate parameter",
                error,
            )
        })?;

    Gate::new(
        kind,
        vec![QubitId::new(qubit)],
        vec![parameter],
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "QFT generated invalid parameterized gate",
            error,
        )
    })
}

/// Creates a two-qubit gate.
fn two_qubit_gate(
    kind: GateKind,
    first: usize,
    second: usize,
) -> BenchmarkResult<Gate> {
    if first == second {
        return Err(invalid_configuration(
            "qft",
            "QFT two-qubit gates cannot target the same logical qubit",
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
            "QFT generated invalid two-qubit gate",
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
            "QFT generated invalid measurement gate",
            error,
        )
    })
}

// =============================================================================
// Application metadata helpers
// =============================================================================

/// Adds one bounded application parameter.
fn add_parameter(
    workload: &mut ApplicationWorkload,
    name: &str,
    value: &str,
) -> BenchmarkResult<()> {
    let parameter =
        ApplicationParameter::new(name, value)
            .map_err(|error| {
                workload_error(
                    "unable to encode QFT application metadata",
                    error,
                )
            })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to attach QFT application metadata",
                error,
            )
        })
}

// =============================================================================
// Parsing helpers
// =============================================================================

/// Parses a strict boolean.
fn parse_bool(
    value: &str,
    field: &'static str,
) -> BenchmarkResult<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(invalid_configuration(
            field,
            "boolean parameter must be exactly `true` or `false`",
        )),
    }
}

// =============================================================================
// Error conversion helpers
// =============================================================================

/// Creates a canonical invalid-configuration error.
fn invalid_configuration(
    field: &'static str,
    reason: &'static str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Converts a workload-generation failure into the canonical benchmark error.
fn invalid_workload(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: QFT_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a canonical workload-model error.
fn workload_error(
    reason: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: QFT_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts Quantum IR construction/validation errors into the benchmark
/// boundary.
fn circuit_error(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: QFT_APPLICATION_ID.to_owned(),
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

    use super::super::super::core::workload::WorkloadId;

    fn request(
        problem_size: usize,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            QFT_APPLICATION_ID,
            WorkloadId::new("instance_0")
                .expect("test workload ID must be valid"),
            problem_size,
            42,
        )
        .expect("test request must be valid")
        .with_generator_revision(
            QFT_GENERATOR_REVISION,
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
    fn exact_controlled_phase_count_is_correct() {
        assert_eq!(
            exact_controlled_phase_count(1)
                .expect("count"),
            0
        );

        assert_eq!(
            exact_controlled_phase_count(2)
                .expect("count"),
            1
        );

        assert_eq!(
            exact_controlled_phase_count(3)
                .expect("count"),
            3
        );

        assert_eq!(
            exact_controlled_phase_count(4)
                .expect("count"),
            6
        );
    }

    #[test]
    fn phase_angles_follow_qft_convention() {
        let pi = std::f64::consts::PI;

        let angle_1 =
            phase_angle(1).expect("angle");

        let angle_2 =
            phase_angle(2).expect("angle");

        assert!(
            (angle_1 - pi / 2.0).abs() < 1.0e-14
        );

        assert!(
            (angle_2 - pi / 4.0).abs() < 1.0e-14
        );
    }

    #[test]
    fn default_configuration_is_exact_conventional_qft() {
        let configuration =
            QftConfiguration::new(4)
                .expect("configuration");

        assert!(
            configuration
                .is_exact_conventional_qft()
        );

        assert_eq!(
            configuration.approximation_degree,
            0
        );

        assert!(configuration.swaps);
        assert!(!configuration.inverse);
    }

    #[test]
    fn request_defaults_are_deterministic() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let configuration =
            generator
                .configuration_from_request(
                    &request(4),
                )
                .expect("configuration");

        assert_eq!(configuration.qubits, 4);
        assert!(!configuration.inverse);
        assert!(configuration.swaps);
        assert_eq!(
            configuration.approximation_degree,
            0
        );
        assert_eq!(
            configuration.input_basis,
            0
        );
    }

    #[test]
    fn parameters_are_parsed() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                5,
                vec![
                    ("inverse", "true"),
                    ("swaps", "false"),
                    ("approximation_degree", "2"),
                    ("input_basis", "7"),
                ],
            );

        let configuration =
            generator
                .configuration_from_request(
                    &request,
                )
                .expect("configuration");

        assert!(configuration.inverse);
        assert!(!configuration.swaps);
        assert_eq!(
            configuration.approximation_degree,
            2
        );
        assert_eq!(
            configuration.input_basis,
            7
        );
    }

    #[test]
    fn unknown_parameter_is_rejected() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                3,
                vec![("unknown", "true")],
            );

        assert!(
            generator
                .configuration_from_request(
                    &request,
                )
                .is_err()
        );
    }

    #[test]
    fn duplicate_parameter_is_rejected() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                3,
                vec![
                    ("inverse", "true"),
                    ("inverse", "false"),
                ],
            );

        assert!(
            generator
                .configuration_from_request(
                    &request,
                )
                .is_err()
        );
    }

    #[test]
    fn invalid_basis_state_is_rejected() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                3,
                vec![("input_basis", "8")],
            );

        assert!(
            generator
                .configuration_from_request(
                    &request,
                )
                .is_err()
        );
    }

    #[test]
    fn qft_one_qubit_is_hadamard_plus_measurement() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let circuit =
            generator
                .generate_circuit(
                    &request(1),
                )
                .expect("circuit");

        assert_eq!(
            circuit.num_qubits(),
            1
        );

        assert_eq!(
            circuit.num_classical_bits(),
            1
        );

        assert_eq!(
            circuit.operations().len(),
            2
        );

        assert_eq!(
            circuit.operations()[0]
                .kind(),
            GateKind::H
        );

        assert_eq!(
            circuit.operations()[1]
                .kind(),
            GateKind::Measure
        );
    }

    #[test]
    fn qft_two_qubit_exact_resource_count_is_correct() {
        let configuration =
            QftConfiguration::new(2)
                .expect("configuration");

        // H0
        // 5-gate controlled phase
        // H1
        // SWAP
        // two measurements
        assert_eq!(
            configuration
                .logical_operation_count()
                .expect("count"),
            10
        );

        assert_eq!(
            configuration
                .logical_two_qubit_gate_count()
                .expect("count"),
            3
        );
    }

    #[test]
    fn qft_circuit_is_valid_for_small_sizes() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        for qubits in 1..=6 {
            let circuit =
                generator
                    .generate_circuit(
                        &request(qubits),
                    )
                    .expect("QFT circuit");

            circuit
                .validate()
                .expect("generated QFT must validate");
        }
    }

    #[test]
    fn inverse_qft_is_generated() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                vec![("inverse", "true")],
            );

        let circuit =
            generator
                .generate_circuit(&request)
                .expect("inverse QFT circuit");

        circuit
            .validate()
            .expect("inverse QFT must validate");

        assert!(
            circuit
                .operations()
                .iter()
                .any(|operation| {
                    operation.kind() == GateKind::RZ
                })
        );
    }

    #[test]
    fn approximate_qft_reduces_phase_count() {
        let configuration =
            QftConfiguration {
                qubits: 5,
                inverse: false,
                swaps: true,
                approximation_degree: 3,
                input_basis: 0,
            };

        configuration
            .validate()
            .expect("configuration");

        assert_eq!(
            configuration
                .exact_controlled_phase_count()
                .expect("count"),
            10
        );

        assert_eq!(
            configuration
                .retained_controlled_phase_count()
                .expect("count"),
            7
        );
    }

    #[test]
    fn omitted_phase_pairs_are_deterministic() {
        let first =
            omitted_phase_pairs(4, 2)
                .expect("pairs");

        let second =
            omitted_phase_pairs(4, 2)
                .expect("pairs");

        assert_eq!(first, second);

        assert!(
            first.contains(&(0, 3))
        );

        assert!(
            first.contains(&(0, 2))
        );
    }

    #[test]
    fn swaps_can_be_disabled_explicitly() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                vec![("swaps", "false")],
            );

        let description =
            generator
                .describe(&request)
                .expect("description");

        assert_eq!(
            description.output_permutation,
            "bit_reversed"
        );

        assert_eq!(
            description.swap_count,
            0
        );
    }

    #[test]
    fn basis_preparation_is_deterministic() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                vec![("input_basis", "5")],
            );

        let first =
            generator
                .generate_circuit(&request)
                .expect("first circuit");

        let second =
            generator
                .generate_circuit(&request)
                .expect("second circuit");

        assert_eq!(
            first.operations(),
            second.operations()
        );
    }

    #[test]
    fn application_workload_contains_canonical_metadata() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let workload =
            generator
                .generate_application_workload(
                    &request(3),
                )
                .expect("workload");

        assert_eq!(
            workload.application_id(),
            QFT_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            3
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "logical_operation_count"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "expected_output_distribution"
                        && parameter.value()
                            == "uniform"
                })
        );
    }

    #[test]
    fn generator_descriptor_has_required_capabilities() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::GeneratesCircuit
                )
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

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::HardwareExecutable
                )
        );
    }

    #[test]
    fn qft_resource_limit_is_enforced_before_generation() {
        let generator =
            QftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request(MAX_QFT_QUBITS + 1);

        assert!(
            generator
                .configuration_from_request(
                    &request,
                )
                .is_err()
        );
    }

    #[test]
    fn approximation_degree_cannot_exceed_phase_count() {
        let configuration =
            QftConfiguration {
                qubits: 4,
                inverse: false,
                swaps: true,
                approximation_degree: 7,
                input_basis: 0,
            };

        assert!(
            configuration.validate().is_err()
        );
    }

    #[test]
    fn non_finite_gate_parameters_are_never_generated() {
        let angle =
            phase_angle(1)
                .expect("phase angle");

        assert!(angle.is_finite());
        assert!(angle > 0.0);
    }
}