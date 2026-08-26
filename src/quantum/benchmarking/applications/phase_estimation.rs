//! Zamani Quantum Benchmarking — Quantum Phase Estimation Application Benchmark
//!
//! Production application-benchmark generator for Quantum Phase Estimation
//! (QPE).
//!
//! # Purpose
//!
//! This module constructs a deterministic, backend-independent QPE application
//! workload using the canonical Zamani Quantum IR.
//!
//! The benchmark deliberately uses a fully specified one-qubit phase-unitary
//! instance rather than pretending that an application generator can accept an
//! arbitrary opaque unitary without a corresponding IR-level controlled-unitary
//! construction contract.
//!
//! The canonical benchmark unitary is:
//!
//! ```text
//! U(phi) = RZ(-4π phi)
//! ```
//!
//! with eigenstate:
//!
//! ```text
//! |0>
//! ```
//!
//! Because:
//!
//! ```text
//! RZ(theta)|0> = exp(-i theta / 2)|0>
//! ```
//!
//! substituting:
//!
//! ```text
//! theta = -4π phi
//! ```
//!
//! gives:
//!
//! ```text
//! U(phi)|0> = exp(2π i phi)|0>
//! ```
//!
//! Therefore the benchmark has the standard QPE eigenphase definition:
//!
//! ```text
//! U|ψ> = exp(2π i φ)|ψ>
//! ```
//!
//! and the expected phase is `φ ∈ [0, 1)`.
//!
//! # Circuit structure
//!
//! For `m` phase/evaluation qubits and one target eigenstate qubit:
//!
//! ```text
//! phase q[0] ──H────●────────────────────── IQFT ──M
//!                    │
//! phase q[1] ──H─────┼────●──────────────── IQFT ──M
//!                    │    │
//! ...                │    │
//! phase q[m] ──H─────┼────┼────●─────────── IQFT ──M
//!                    │    │    │
//! target q[m+1] ─────U────U²───U⁴────────── untouched
//! ```
//!
//! The controlled powers are:
//!
//! ```text
//! controlled-U^(2^k)
//! ```
//!
//! The inverse QFT is explicitly decomposed into canonical IR gates. Controlled
//! phase rotations are decomposed into RZ and CX operations because the
//! canonical Zamani Quantum IR currently has no dedicated controlled-phase
//! gate.
//!
//! # Benchmark boundary
//!
//! This module owns:
//!
//! - benchmark identity;
//! - benchmark-versioned generator semantics;
//! - parameter parsing;
//! - eigenphase validation;
//! - phase-register sizing;
//! - deterministic workload construction;
//! - QPE logical circuit construction;
//! - expected phase metadata;
//! - expected measurement metadata;
//! - logical resource metadata;
//! - reproducibility metadata attached to the workload;
//! - application-generator integration.
//!
//! This module does NOT own:
//!
//! - backend selection;
//! - physical routing;
//! - scheduling;
//! - hardware calibration;
//! - QPU communication;
//! - simulation;
//! - execution;
//! - statistical aggregation;
//! - confidence-interval calculation;
//! - benchmark result aggregation;
//! - hardware-specific decomposition.
//!
//! Those responsibilities remain in the surrounding benchmarking, compiler,
//! runtime, routing, scheduling, and hardware subsystems.
//!
//! # Architectural dependency direction
//!
//! ```text
//! ApplicationGenerationRequest
//!             │
//!             ▼
//! PhaseEstimationBenchmarkGenerator
//!             │
//!             ▼
//! ApplicationWorkload
//!             │
//!             ▼
//! CircuitWorkload
//!             │
//!             ▼
//! QuantumCircuit
//!             │
//!             ▼
//! compiler / optimization / routing / scheduling
//!             │
//!             ▼
//! runtime / hardware executor
//!             │
//!             ▼
//! observations
//!             │
//!             ▼
//! application analysis
//! ```
//!
//! The generator therefore never depends on execution.
//!
//! # Relationship with quantum::algorithms::phase_estimation
//!
//! The repository already contains a backend-independent QPE algorithm module.
//! That module owns the general QPE algorithm contract, including logical
//! controlled-power planning, inverse-QFT planning, execution requests and
//! phase decoding.
//!
//! This benchmark module is deliberately different:
//!
//! ```text
//! quantum::algorithms::phase_estimation
//!     = general QPE algorithm API
//!
//! quantum::benchmarking::applications::phase_estimation
//!     = reproducible benchmark workload
//! ```
//!
//! The benchmark provides a concrete unitary/eigenstate pair so the benchmark
//! can be generated without introducing a new opaque-unitary benchmark API.
//!
//! # Scientific semantics
//!
//! Quantum Phase Estimation estimates the phase `φ` in:
//!
//! ```text
//! U|ψ> = exp(2πiφ)|ψ>
//! ```
//!
//! With `m` evaluation qubits, the ideal binary-grid resolution is:
//!
//! ```text
//! Δφ = 1 / 2^m
//! ```
//!
//! If the phase is exactly representable on that grid, ideal QPE produces the
//! corresponding computational-basis value with probability one. For phases
//! that are not exactly representable, the output is probabilistic and the
//! analysis layer must measure phase-estimation error rather than requiring one
//! exact bit string.
//!
//! These semantics are consistent with the standard QPE formulation and with
//! the repository's existing `quantum::algorithms::phase_estimation` contract.
//!
//! # Reproducibility
//!
//! Generation is deterministic for identical:
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
//! No global RNG, system clock, process ID, pointer address, thread ID, or
//! external entropy source is consulted.
//!
//! # Security/resource model
//!
//! Benchmark requests can originate from the Zamani language, CI, serialized
//! benchmark definitions, or external tooling and are therefore treated as
//! untrusted.
//!
//! This implementation:
//!
//! - validates all identifiers through the canonical generator contract;
//! - validates every parameter before circuit allocation;
//! - rejects duplicate parameters;
//! - rejects unknown parameters;
//! - rejects non-finite eigenphases;
//! - rejects eigenphases outside `[0, 1)`;
//! - bounds the phase-register size;
//! - checks every arithmetic operation before allocation;
//! - checks every generated angle for finiteness;
//! - uses canonical Quantum IR validation for every gate;
//! - uses the canonical Quantum IR resource limits;
//! - does not enumerate an exponential truth table;
//! - does not execute caller-provided code;
//! - performs no filesystem/network I/O;
//! - does not select a backend;
//! - does not silently truncate user parameters.
//!
//! # Resource semantics
//!
//! For `m` phase qubits and one target qubit:
//!
//! ```text
//! logical qubits = m + 1
//! classical bits = m
//! controlled-U operations = m
//! ```
//!
//! The inverse QFT contributes:
//!
//! ```text
//! floor(m / 2) swaps
//! m Hadamards
//! m(m - 1) / 2 controlled-phase operations
//! ```
//!
//! Each controlled-phase operation is decomposed into five canonical IR gates:
//!
//! ```text
//! RZ(control, λ/2)
//! RZ(target,  λ/2)
//! CX(control, target)
//! RZ(target, -λ/2)
//! CX(control, target)
//! ```
//!
//! This decomposition differs from a dedicated controlled-phase primitive only
//! by a global phase and therefore preserves the logical operation for quantum
//! algorithm purposes.
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
//! No additional dependencies are required.
//!
//! # Integration contract
//!
//! This file intentionally integrates with the already-established contracts:
//!
//! ```text
//! crate::quantum::benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorDescriptor
//!     ├── circuit_application_descriptor
//!     └── make_application_workload
//!
//! crate::quantum::benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     ├── CircuitWorkload
//!     └── WorkloadId
//!
//! crate::quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── GateKind
//!     ├── Measurement
//!     ├── Parameter
//!     └── QubitId
//! ```
//!
//! The only namespace integration required later is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod phase_estimation;
//! ```
//!
//! This file does not need to be edited when that module is registered.
//!
//! # Recommended analysis contract
//!
//! The execution/analysis layer should consume the generated workload and
//! report at least:
//!
//! - estimated phase;
//! - absolute phase error;
//! - circular phase error;
//! - expected phase;
//! - binary-grid resolution;
//! - success within one-grid-cell;
//! - success within caller-selected tolerance;
//! - shot count;
//! - logical qubit count;
//! - circuit depth;
//! - gate count;
//! - two-qubit gate count;
//! - controlled-unitary count;
//! - inverse-QFT gate count;
//! - execution time;
//! - compilation time;
//! - routing overhead;
//! - backend provenance;
//! - calibration provenance;
//! - reproducibility fingerprint.
//!
//! This module does not calculate those execution metrics.
//!
//! # References
//!
//! The benchmark follows the standard QPE problem definition:
//!
//! ```text
//! U|ψ> = exp(2πiφ)|ψ>
//! ```
//!
//! and the standard controlled powers followed by inverse QFT.
//!
//! See:
//!
//! - IBM Quantum Learning, "The phase-estimation problem".
//! - IBM Quantum Learning, "Quantum phase estimation".
//! - IBM Quantum Learning, "Quantum Fourier transform".
//!
//! These references are external scientific references; the implementation
//! remains independent of any vendor SDK.

use std::f64::consts::PI;

use super::super::core::errors::{
    BenchmarkError,
    BenchmarkResult,
};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    CircuitWorkload,
    WorkloadError,
};
use super::super::generators::application::{
    circuit_application_descriptor,
    ApplicationBenchmarkGenerator,
    ApplicationGenerationRequest,
    ApplicationGeneratorDescriptor,
};

use crate::quantum::ir::{
    ClassicalBitId,
    Gate,
    GateKind,
    Measurement,
    Parameter,
    QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable application identifier.
pub const APPLICATION_ID: &str = "phase_estimation";

/// Stable generator identifier.
pub const GENERATOR_ID: &str =
    "phase_estimation_generator";

/// Human-readable benchmark name.
pub const BENCHMARK_NAME: &str =
    "Quantum Phase Estimation";

/// Generator semantic version.
pub const GENERATOR_VERSION: &str = "1.0.0";

/// Generator semantic revision.
///
/// Increment this when deterministic circuit semantics change.
pub const GENERATOR_REVISION: u32 = 1;

/// Stable source identifier embedded in generated Quantum IR metadata.
pub const CIRCUIT_SOURCE: &str =
    "zamani.quantum.benchmarking.applications.phase_estimation";

// =============================================================================
// Parameters
// =============================================================================

/// Optional parameter selecting the eigenphase.
///
/// Value must be a finite decimal representation in `[0, 1)`.
pub const EIGENPHASE_PARAMETER: &str = "eigenphase";

/// Optional parameter controlling inverse-QFT bit reversal.
///
/// Accepted values: `true` or `false`.
///
/// `true` is the canonical QPE measurement ordering used by this benchmark.
pub const BIT_REVERSAL_PARAMETER: &str = "bit_reversal";

/// Optional parameter documenting the benchmark unitary.
///
/// Only `phase` is currently supported.
pub const UNITARY_PARAMETER: &str = "unitary";

/// Canonical supported unitary kind.
pub const PHASE_UNITARY_KIND: &str = "phase";

/// Default canonical eigenphase.
///
/// `1/6` is exactly representable as a simple, nontrivial QPE test phase while
/// still producing a non-degenerate probability distribution for small
/// evaluation registers.
pub const DEFAULT_EIGENPHASE: f64 = 1.0 / 6.0;

/// Maximum phase-register size supported by this concrete benchmark.
///
/// The general QPE algorithm also has a u64 measurement representation, but
/// this application benchmark intentionally imposes a lower operational bound
/// so that generated circuits and angle magnitudes remain practical.
pub const MAX_PHASE_QUBITS: usize = 32;

/// Minimum supported phase-register size.
pub const MIN_PHASE_QUBITS: usize = 1;

/// Number of target/eigenstate qubits in this canonical benchmark.
pub const TARGET_QUBITS: usize = 1;

/// Number of classical measurement bits.
pub const CLASSICAL_BITS_PER_PHASE_QUBIT: usize = 1;

// =============================================================================
// Strongly typed benchmark description
// =============================================================================

/// Canonical phase-estimation benchmark instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseEstimationInstance {
    /// Number of evaluation/phase qubits.
    phase_qubits: usize,

    /// Eigenphase in `[0, 1)`.
    eigenphase: f64,

    /// Whether inverse-QFT bit reversal is included.
    bit_reversal: bool,
}

impl PhaseEstimationInstance {
    /// Creates a validated benchmark instance.
    pub fn new(
        phase_qubits: usize,
        eigenphase: f64,
        bit_reversal: bool,
    ) -> BenchmarkResult<Self> {
        validate_phase_qubits(phase_qubits)?;
        validate_eigenphase(eigenphase)?;

        Ok(Self {
            phase_qubits,
            eigenphase,
            bit_reversal,
        })
    }

    /// Returns the number of evaluation qubits.
    #[must_use]
    pub const fn phase_qubits(self) -> usize {
        self.phase_qubits
    }

    /// Returns the target/eigenstate qubit count.
    #[must_use]
    pub const fn target_qubits(self) -> usize {
        TARGET_QUBITS
    }

    /// Returns total logical qubits.
    pub fn total_qubits(self) -> BenchmarkResult<usize> {
        self.phase_qubits
            .checked_add(TARGET_QUBITS)
            .ok_or_else(|| numerical_overflow(
                "phase-estimation total logical qubits",
            ))
    }

    /// Returns the eigenphase.
    #[must_use]
    pub const fn eigenphase(self) -> f64 {
        self.eigenphase
    }

    /// Returns whether bit reversal is enabled.
    #[must_use]
    pub const fn bit_reversal(self) -> bool {
        self.bit_reversal
    }

    /// Returns the binary phase resolution.
    pub fn resolution(self) -> BenchmarkResult<f64> {
        phase_resolution(self.phase_qubits)
    }

    /// Returns the ideal measurement integer nearest to the phase.
    ///
    /// For exactly representable phases this is the exact binary encoding.
    /// For non-grid phases this is only an expected central outcome and must
    /// not be treated as the only valid hardware result.
    pub fn expected_measurement(self) -> BenchmarkResult<u64> {
        let dimension = phase_register_dimension(self.phase_qubits)?;
        let scaled = self.eigenphase * dimension as f64;

        if !scaled.is_finite() {
            return Err(BenchmarkError::NonFiniteValue {
                field: "expected QPE measurement",
                value: scaled,
            });
        }

        let rounded = (scaled + 0.5).floor();

        let value = if rounded >= dimension as f64 {
            dimension - 1
        } else {
            rounded as u64
        };

        Ok(value)
    }

    /// Returns the corresponding expected quantized phase.
    pub fn expected_quantized_phase(
        self,
    ) -> BenchmarkResult<f64> {
        let dimension = phase_register_dimension(self.phase_qubits)?;
        let measurement = self.expected_measurement()?;

        Ok(measurement as f64 / dimension as f64)
    }

    /// Returns the absolute quantization error of the nearest output bin.
    pub fn quantization_error(self) -> BenchmarkResult<f64> {
        Ok(
            (self.expected_quantized_phase()? - self.eigenphase)
                .abs(),
        )
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Quantum Phase Estimation application benchmark generator.
///
/// The generator is stateless and therefore safe to share through
/// `Arc<dyn ApplicationBenchmarkGenerator>`.
#[derive(Debug, Clone)]
pub struct PhaseEstimationBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl PhaseEstimationBenchmarkGenerator {
    /// Creates the canonical QPE application benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = circuit_application_descriptor(
            GENERATOR_ID,
            APPLICATION_ID,
            GENERATOR_VERSION,
            "Deterministic Quantum Phase Estimation application benchmark generator",
        )?;

        Ok(Self { descriptor })
    }

    /// Returns the canonical generator descriptor.
    #[must_use]
    pub fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Parses a generation request into a strongly typed benchmark instance.
    ///
    /// Unknown parameters and duplicate parameters are rejected.
    pub fn instance_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<PhaseEstimationInstance> {
        self.validate_common(request)?;

        let mut eigenphase = None;
        let mut bit_reversal = true;
        let mut unitary_kind = None;

        for parameter in request.parameters() {
            match parameter.name() {
                EIGENPHASE_PARAMETER => {
                    if eigenphase.is_some() {
                        return Err(invalid_configuration(
                            EIGENPHASE_PARAMETER,
                            "duplicate eigenphase parameter",
                        ));
                    }

                    eigenphase = Some(parse_eigenphase(
                        parameter.value(),
                    )?);
                }

                BIT_REVERSAL_PARAMETER => {
                    if parameter.value() != "true"
                        && parameter.value() != "false"
                    {
                        return Err(invalid_configuration(
                            BIT_REVERSAL_PARAMETER,
                            "bit_reversal must be `true` or `false`",
                        ));
                    }

                    if bit_reversal != true {
                        return Err(invalid_configuration(
                            BIT_REVERSAL_PARAMETER,
                            "duplicate bit_reversal parameter",
                        ));
                    }

                    bit_reversal =
                        parameter.value() == "true";
                }

                UNITARY_PARAMETER => {
                    if unitary_kind.is_some() {
                        return Err(invalid_configuration(
                            UNITARY_PARAMETER,
                            "duplicate unitary parameter",
                        ));
                    }

                    unitary_kind =
                        Some(parameter.value().to_owned());
                }

                other => {
                    return Err(invalid_configuration(
                        "application_parameter",
                        match other {
                            "" => {
                                "application parameter name must not be empty"
                            }
                            _ => {
                                "unknown Phase Estimation application parameter"
                            }
                        },
                    ));
                }
            }
        }

        let eigenphase =
            eigenphase.unwrap_or(DEFAULT_EIGENPHASE);

        if let Some(unitary_kind) = unitary_kind {
            if unitary_kind != PHASE_UNITARY_KIND {
                return Err(invalid_configuration(
                    UNITARY_PARAMETER,
                    "only the canonical `phase` unitary is supported",
                ));
            }
        }

        PhaseEstimationInstance::new(
            request.problem_size(),
            eigenphase,
            bit_reversal,
        )
    }

    /// Returns a strongly typed description without allocating a Quantum IR
    /// circuit.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<PhaseEstimationInstance> {
        self.instance_from_request(request)
    }

    /// Generates the complete canonical Quantum IR QPE circuit.
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let instance =
            self.instance_from_request(request)?;

        let total_qubits =
            instance.total_qubits()?;

        let mut circuit =
            QuantumCircuit::new(
                total_qubits,
                instance.phase_qubits(),
            )
            .map_err(|error| {
                circuit_error(
                    "unable to construct QPE Quantum IR circuit",
                    error,
                )
            })?;

        circuit
            .set_name(Some(format!(
                "phase_estimation_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QPE circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(CIRCUIT_SOURCE.to_owned()))
            .map_err(|error| {
                circuit_error(
                    "unable to assign QPE circuit source",
                    error,
                )
            })?;

        let target =
            instance.phase_qubits();

        // ---------------------------------------------------------------------
        // State preparation
        // ---------------------------------------------------------------------
        //
        // The target begins in |0>, which is the exact eigenstate of the
        // canonical RZ-based benchmark unitary. No target preparation gate is
        // therefore necessary.
        //
        // Evaluation register:
        //
        //     |0> --H--
        //
        // Target:
        //
        //     |0>
        //
        for qubit in 0..instance.phase_qubits() {
            circuit
                .push(single_qubit_gate(
                    GateKind::H,
                    qubit,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QPE evaluation Hadamard",
                        error,
                    )
                })?;
        }

        // ---------------------------------------------------------------------
        // Controlled powers U^(2^k)
        // ---------------------------------------------------------------------
        //
        // U(phi) = RZ(-4πphi)
        //
        // On |0>:
        //
        //     U(phi)|0> = exp(2πi phi)|0>
        //
        // Therefore controlled-U^(2^k) can be represented exactly on the
        // benchmark eigenstate by:
        //
        //     CRZ(-4πphi * 2^k)
        //
        // because CRZ(theta)|0_target> contributes exp(-i theta / 2).
        for control in 0..instance.phase_qubits() {
            let power =
                checked_power_of_two(control)?;

            let angle =
                -4.0
                    * PI
                    * instance.eigenphase()
                    * power as f64;

            validate_angle(
                angle,
                "controlled-U QPE angle",
            )?;

            circuit
                .push(two_qubit_parameterized_gate(
                    GateKind::CRZ,
                    control,
                    target,
                    angle,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QPE controlled-unitary power",
                        error,
                    )
                })?;
        }

        // ---------------------------------------------------------------------
        // Inverse QFT
        // ---------------------------------------------------------------------
        //
        // The inverse QFT is applied to the evaluation register.
        apply_inverse_qft(
            &mut circuit,
            instance.phase_qubits(),
            instance.bit_reversal(),
        )?;

        // ---------------------------------------------------------------------
        // Measurement
        // ---------------------------------------------------------------------
        //
        // Evaluation q[k] -> classical c[k].
        //
        // The target eigenstate qubit is not measured because it is not part of
        // the phase estimate.
        for qubit in 0..instance.phase_qubits() {
            circuit
                .push(measurement_gate(
                    qubit,
                    qubit,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QPE phase measurement",
                        error,
                    )
                })?;
        }

        circuit
            .validate()
            .map_err(|error| {
                circuit_error(
                    "generated QPE circuit failed final Quantum IR validation",
                    error,
                )
            })?;

        Ok(circuit)
    }

    /// Generates the complete canonical application workload.
    pub fn generate_application_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let instance =
            self.instance_from_request(request)?;

        let circuit =
            self.generate_circuit(request)?;

        let circuit_workload =
            CircuitWorkload::from_circuit(
                circuit,
                request.instance_id().clone(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create QPE circuit workload",
                    error,
                )
            })?;

        let mut workload =
            ApplicationWorkload::new(
                APPLICATION_ID,
                request.instance_id().clone(),
                request.problem_size(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create QPE application workload",
                    error,
                )
            })?
            .with_circuit(circuit_workload);

        // ---------------------------------------------------------------------
        // Canonical application metadata
        // ---------------------------------------------------------------------
        //
        // The universal ApplicationWorkload intentionally stores application
        // parameters as bounded strings. The strongly typed
        // PhaseEstimationInstance remains available through `describe()`.
        add_parameter(
            &mut workload,
            "benchmark_name",
            BENCHMARK_NAME,
        )?;

        add_parameter(
            &mut workload,
            "unitary",
            PHASE_UNITARY_KIND,
        )?;

        add_parameter(
            &mut workload,
            "eigenphase",
            &format!("{:.17}", instance.eigenphase()),
        )?;

        add_parameter(
            &mut workload,
            "phase_qubits",
            &instance.phase_qubits().to_string(),
        )?;

        add_parameter(
            &mut workload,
            "target_qubits",
            &instance.target_qubits().to_string(),
        )?;

        add_parameter(
            &mut workload,
            "total_qubits",
            &instance.total_qubits()?.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "resolution",
            &format!(
                "{:.17}",
                instance.resolution()?
            ),
        )?;

        add_parameter(
            &mut workload,
            "expected_measurement",
            &instance
                .expected_measurement()?
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "expected_quantized_phase",
            &format!(
                "{:.17}",
                instance.expected_quantized_phase()?
            ),
        )?;

        add_parameter(
            &mut workload,
            "quantization_error",
            &format!(
                "{:.17}",
                instance.quantization_error()?
            ),
        )?;

        add_parameter(
            &mut workload,
            "bit_reversal",
            if instance.bit_reversal() {
                "true"
            } else {
                "false"
            },
        )?;

        add_parameter(
            &mut workload,
            "generator_id",
            GENERATOR_ID,
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &GENERATOR_REVISION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "sequence_index",
            &request
                .metadata()
                .sequence_index()
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "seed",
            &request
                .metadata()
                .seed()
                .to_string(),
        )?;

        Ok(workload)
    }

    fn validate_common(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;

        if request.application_id()
            != APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first: "request.application_id"
                        .to_owned(),
                    second: "phase_estimation.application_id"
                        .to_owned(),
                    reason:
                        "Phase Estimation generator requires application_id `phase_estimation`"
                            .to_owned(),
                },
            );
        }

        validate_phase_qubits(
            request.problem_size(),
        )?;

        Ok(())
    }
}

impl ApplicationBenchmarkGenerator
    for PhaseEstimationBenchmarkGenerator
{
    fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        self.validate_common(request)?;

        // Force complete application-parameter validation before any circuit
        // allocation occurs.
        let _ = self.instance_from_request(request)?;

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
// Circuit construction helpers
// =============================================================================

/// Creates a validated parameter-free single-qubit gate.
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
        BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: format!(
                "invalid QPE single-qubit gate: {error}"
            ),
        }
    })
}

/// Creates a validated parameter-free two-qubit gate.
fn two_qubit_gate(
    kind: GateKind,
    control: usize,
    target: usize,
) -> BenchmarkResult<Gate> {
    Gate::new(
        kind,
        vec![
            QubitId::new(control),
            QubitId::new(target),
        ],
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: format!(
                "invalid QPE two-qubit gate: {error}"
            ),
        }
    })
}

/// Creates a validated parameterized two-qubit gate.
fn two_qubit_parameterized_gate(
    kind: GateKind,
    control: usize,
    target: usize,
    angle: f64,
) -> BenchmarkResult<Gate> {
    let parameter =
        Parameter::constant(angle)
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload: APPLICATION_ID.to_owned(),
                    reason: format!(
                        "invalid QPE gate parameter: {error}"
                    ),
                }
            })?;

    Gate::new(
        kind,
        vec![
            QubitId::new(control),
            QubitId::new(target),
        ],
        vec![parameter],
        None,
        None,
    )
    .map_err(|error| {
        BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: format!(
                "invalid QPE parameterized gate: {error}"
            ),
        }
    })
}

/// Creates a validated parameterized single-qubit RZ gate.
fn rz_gate(
    qubit: usize,
    angle: f64,
) -> BenchmarkResult<Gate> {
    validate_angle(angle, "QPE RZ angle")?;

    let parameter =
        Parameter::constant(angle)
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload: APPLICATION_ID.to_owned(),
                    reason: format!(
                        "invalid QPE RZ parameter: {error}"
                    ),
                }
            })?;

    Gate::new(
        GateKind::RZ,
        vec![QubitId::new(qubit)],
        vec![parameter],
        None,
        None,
    )
    .map_err(|error| {
        BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: format!(
                "invalid QPE RZ gate: {error}"
            ),
        }
    })
}

/// Creates a validated computational-basis measurement.
fn measurement_gate(
    qubit: usize,
    classical_bit: usize,
) -> BenchmarkResult<Gate> {
    let measurement =
        Measurement::new(
            QubitId::new(qubit),
            ClassicalBitId::new(classical_bit),
        );

    Gate::new(
        GateKind::Measure,
        vec![QubitId::new(qubit)],
        Vec::new(),
        Some(classical_bit),
        Some(measurement),
    )
    .map_err(|error| {
        BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: format!(
                "invalid QPE measurement gate: {error}"
            ),
        }
    })
}

/// Applies the inverse QFT to the first `qubit_count` qubits.
///
/// The implementation includes the final bit-reversal when requested.
///
/// The controlled-phase operation is decomposed as:
///
/// ```text
/// RZ(control, λ/2)
/// RZ(target,  λ/2)
/// CX(control, target)
/// RZ(target, -λ/2)
/// CX(control, target)
/// ```
///
/// which implements controlled-phase up to a global phase.
fn apply_inverse_qft(
    circuit: &mut QuantumCircuit,
    qubit_count: usize,
    bit_reversal: bool,
) -> BenchmarkResult<()> {
    if qubit_count == 0 {
        return Err(
            BenchmarkError::InvalidRange {
                field: "qpe.inverse_qft_qubit_count"
                    .to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    MAX_PHASE_QUBITS.to_string(),
                ),
            },
        );
    }

    if qubit_count > MAX_PHASE_QUBITS {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "qpe_inverse_qft_qubits"
                        .to_owned(),
                requested: qubit_count as u64,
                maximum: MAX_PHASE_QUBITS as u64,
            },
        );
    }

    // QFT inverse convention:
    //
    // First reverse the register if the benchmark requests the canonical
    // computational-bit ordering.
    if bit_reversal {
        for left in 0..(qubit_count / 2) {
            let right =
                qubit_count - left - 1;

            circuit
                .push(two_qubit_gate(
                    GateKind::SWAP,
                    left,
                    right,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QPE inverse-QFT bit-reversal SWAP",
                        error,
                    )
                })?;
        }
    }

    // Inverse QFT.
    //
    // For each target j:
    //
    //     controlled-phase(-π / 2^(j-k))
    //
    // followed by H(j).
    for target in 0..qubit_count {
        for control in 0..target {
            let exponent =
                target - control;

            let denominator =
                checked_power_of_two(exponent)?;

            let angle =
                -PI / denominator as f64;

            append_controlled_phase(
                circuit,
                control,
                target,
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
                    "unable to append QPE inverse-QFT Hadamard",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends a logical controlled-phase operation using canonical IR gates.
///
/// Up to a global phase:
///
/// ```text
/// CP(λ)
/// = RZ(control, λ/2)
///   RZ(target, λ/2)
///   CX(control,target)
///   RZ(target,-λ/2)
///   CX(control,target)
/// ```
fn append_controlled_phase(
    circuit: &mut QuantumCircuit,
    control: usize,
    target: usize,
    angle: f64,
) -> BenchmarkResult<()> {
    validate_angle(
        angle,
        "QPE controlled-phase angle",
    )?;

    circuit
        .push(rz_gate(
            control,
            angle / 2.0,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QPE controlled-phase control RZ",
                error,
            )
        })?;

    circuit
        .push(rz_gate(
            target,
            angle / 2.0,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QPE controlled-phase target RZ",
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
                "unable to append QPE controlled-phase first CX",
                error,
            )
        })?;

    circuit
        .push(rz_gate(
            target,
            -angle / 2.0,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append QPE controlled-phase inverse RZ",
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
                "unable to append QPE controlled-phase second CX",
                error,
            )
        })?;

    Ok(())
}

// =============================================================================
// Validation and arithmetic helpers
// =============================================================================

/// Validates the phase-register size.
fn validate_phase_qubits(
    phase_qubits: usize,
) -> BenchmarkResult<()> {
    if phase_qubits < MIN_PHASE_QUBITS {
        return Err(
            BenchmarkError::InvalidRange {
                field:
                    "phase_estimation.phase_qubits"
                        .to_owned(),
                value: phase_qubits.to_string(),
                minimum: Some(
                    MIN_PHASE_QUBITS.to_string(),
                ),
                maximum: Some(
                    MAX_PHASE_QUBITS.to_string(),
                ),
            },
        );
    }

    if phase_qubits > MAX_PHASE_QUBITS {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "phase_estimation.phase_qubits"
                        .to_owned(),
                requested: phase_qubits as u64,
                maximum: MAX_PHASE_QUBITS as u64,
            },
        );
    }

    Ok(())
}

/// Validates an eigenphase.
///
/// Phase `0` is valid. Phase `1` is excluded because phases are periodic
/// modulo one.
fn validate_eigenphase(
    eigenphase: f64,
) -> BenchmarkResult<()> {
    if !eigenphase.is_finite() {
        return Err(
            BenchmarkError::NonFiniteValue {
                field: "phase_estimation.eigenphase",
                value: eigenphase,
            },
        );
    }

    if !(0.0..1.0).contains(&eigenphase) {
        return Err(
            BenchmarkError::InvalidRange {
                field:
                    "phase_estimation.eigenphase"
                        .to_owned(),
                value: eigenphase.to_string(),
                minimum: Some("0".to_owned()),
                maximum: Some(
                    "(1, exclusive)".to_owned(),
                ),
            },
        );
    }

    Ok(())
}

/// Parses a bounded textual eigenphase.
fn parse_eigenphase(
    value: &str,
) -> BenchmarkResult<f64> {
    if value.is_empty() {
        return Err(invalid_configuration(
            EIGENPHASE_PARAMETER,
            "eigenphase must not be empty",
        ));
    }

    let parsed =
        value.parse::<f64>().map_err(|_| {
            invalid_configuration(
                EIGENPHASE_PARAMETER,
                "eigenphase must be a finite decimal floating-point value",
            )
        })?;

    validate_eigenphase(parsed)?;

    Ok(parsed)
}

/// Validates an IR gate angle.
fn validate_angle(
    angle: f64,
    field: &'static str,
) -> BenchmarkResult<()> {
    if !angle.is_finite() {
        return Err(
            BenchmarkError::NonFiniteValue {
                field,
                value: angle,
            },
        );
    }

    Ok(())
}

/// Returns 2^qubits as a checked `u64`.
fn phase_register_dimension(
    qubits: usize,
) -> BenchmarkResult<u64> {
    validate_phase_qubits(qubits)?;

    if qubits >= 64 {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "qpe_phase_register_dimension"
                        .to_owned(),
                requested: qubits as u64,
                maximum: 63,
            },
        );
    }

    1_u64
        .checked_shl(qubits as u32)
        .ok_or_else(|| {
            numerical_overflow(
                "QPE phase-register dimension",
            )
        })
}

/// Returns `1 / 2^qubits`.
fn phase_resolution(
    qubits: usize,
) -> BenchmarkResult<f64> {
    let dimension =
        phase_register_dimension(qubits)?;

    let resolution =
        1.0 / dimension as f64;

    if !resolution.is_finite()
        || resolution <= 0.0
    {
        return Err(
            BenchmarkError::NumericalInstability {
                operation:
                    "QPE phase resolution"
                        .to_owned(),
            },
        );
    }

    Ok(resolution)
}

/// Returns `2^index` as a checked `u64`.
fn checked_power_of_two(
    index: usize,
) -> BenchmarkResult<u64> {
    if index >= 64 {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "qpe_controlled_power_exponent"
                        .to_owned(),
                requested: index as u64,
                maximum: 63,
            },
        );
    }

    1_u64
        .checked_shl(index as u32)
        .ok_or_else(|| {
            numerical_overflow(
                "QPE controlled-unitary exponent",
            )
        })
}

/// Creates the canonical numerical-overflow benchmark error.
fn numerical_overflow(
    operation: &'static str,
) -> BenchmarkError {
    BenchmarkError::NumericalOverflow {
        operation: operation.to_owned(),
        value: None,
    }
}

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

/// Converts a workload error into the canonical benchmark error hierarchy.
fn workload_error(
    context: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: APPLICATION_ID.to_owned(),
        reason: format!("{context}: {error}"),
    }
}

/// Converts a Quantum IR construction error into the canonical benchmark error
/// hierarchy.
fn circuit_error<E>(
    context: &'static str,
    error: E,
) -> BenchmarkError
where
    E: std::fmt::Display,
{
    BenchmarkError::InvalidWorkload {
        workload: APPLICATION_ID.to_owned(),
        reason: format!("{context}: {error}"),
    }
}

/// Adds a bounded canonical application parameter.
fn add_parameter(
    workload: &mut ApplicationWorkload,
    name: &str,
    value: &str,
) -> BenchmarkResult<()> {
    let parameter =
        ApplicationParameter::new(
            name.to_owned(),
            value.to_owned(),
        )
        .map_err(|error| {
            workload_error(
                "unable to create QPE application parameter",
                error,
            )
        })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to add QPE application parameter",
                error,
            )
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::generators::application::{
        ApplicationGenerationMetadata,
    };
    use super::super::super::core::workload::WorkloadId;

    fn request(
        phase_qubits: usize,
        parameters: Vec<ApplicationParameter>,
    ) -> ApplicationGenerationRequest {
        let metadata =
            ApplicationGenerationMetadata::new(
                42,
                0,
                GENERATOR_REVISION,
            );

        ApplicationGenerationRequest::with_metadata(
            APPLICATION_ID,
            WorkloadId::new("phase_instance")
                .expect("test workload ID must be valid"),
            phase_qubits,
            parameters,
            metadata,
        )
        .expect("test request must be valid")
    }

    fn parameter(
        name: &str,
        value: &str,
    ) -> ApplicationParameter {
        ApplicationParameter::new(
            name.to_owned(),
            value.to_owned(),
        )
        .expect("test parameter must be valid")
    }

    #[test]
    fn generator_descriptor_is_stable() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .expect("generator must be valid");

        assert_eq!(
            generator.descriptor().generator_id(),
            GENERATOR_ID
        );

        assert_eq!(
            generator.descriptor().application_id(),
            APPLICATION_ID
        );

        assert_eq!(
            generator.descriptor().version(),
            GENERATOR_VERSION
        );
    }

    #[test]
    fn generator_rejects_wrong_application_id() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let metadata =
            ApplicationGenerationMetadata::new(
                1,
                0,
                GENERATOR_REVISION,
            );

        let request =
            ApplicationGenerationRequest::with_metadata(
                "other_application",
                WorkloadId::new(
                    "phase_instance",
                )
                .unwrap(),
                4,
                Vec::new(),
                metadata,
            )
            .unwrap();

        assert!(
            generator.generate(&request).is_err()
        );
    }

    #[test]
    fn default_instance_is_valid() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let generated =
            generator.generate(
                &request(8, Vec::new()),
            )
            .unwrap();

        assert_eq!(
            generated.workload().application_id(),
            APPLICATION_ID
        );

        assert_eq!(
            generated.workload().problem_size(),
            8
        );

        assert!(
            generated.workload().circuit().is_some()
        );
    }

    #[test]
    fn explicit_eigenphase_is_parsed() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let instance =
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0.25",
                        ),
                    ],
                ))
                .unwrap();

        assert_eq!(
            instance.eigenphase(),
            0.25
        );

        assert_eq!(
            instance.expected_measurement()
                .unwrap(),
            4
        );
    }

    #[test]
    fn phase_zero_is_valid() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let instance =
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0",
                        ),
                    ],
                ))
                .unwrap();

        assert_eq!(
            instance.eigenphase(),
            0.0
        );

        assert_eq!(
            instance.expected_measurement()
                .unwrap(),
            0
        );
    }

    #[test]
    fn phase_one_is_rejected() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "1",
                        ),
                    ],
                ))
                .is_err()
        );
    }

    #[test]
    fn non_finite_phase_is_rejected() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "NaN",
                        ),
                    ],
                ))
                .is_err()
        );

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "inf",
                        ),
                    ],
                ))
                .is_err()
        );
    }

    #[test]
    fn unknown_parameter_is_rejected() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            "unknown",
                            "1",
                        ),
                    ],
                ))
                .is_err()
        );
    }

    #[test]
    fn duplicate_eigenphase_is_rejected() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0.25",
                        ),
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0.5",
                        ),
                    ],
                ))
                .is_err()
        );
    }

    #[test]
    fn unsupported_unitary_is_rejected() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    4,
                    vec![
                        parameter(
                            UNITARY_PARAMETER,
                            "qft",
                        ),
                    ],
                ))
                .is_err()
        );
    }

    #[test]
    fn phase_qubit_limits_are_enforced() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        assert!(
            generator
                .describe(&request(
                    MAX_PHASE_QUBITS + 1,
                    Vec::new(),
                ))
                .is_err()
        );

        assert!(
            generator
                .describe(&request(
                    0,
                    Vec::new(),
                ))
                .is_err()
        );
    }

    #[test]
    fn expected_measurement_for_exact_binary_phase_is_exact() {
        let instance =
            PhaseEstimationInstance::new(
                8,
                0.25,
                true,
            )
            .unwrap();

        assert_eq!(
            instance.expected_measurement()
                .unwrap(),
            64
        );

        assert_eq!(
            instance
                .expected_quantized_phase()
                .unwrap(),
            0.25
        );

        assert_eq!(
            instance
                .quantization_error()
                .unwrap(),
            0.0
        );
    }

    #[test]
    fn resolution_is_binary_grid_resolution() {
        let instance =
            PhaseEstimationInstance::new(
                8,
                0.25,
                true,
            )
            .unwrap();

        assert!(
            (instance.resolution().unwrap()
                - 1.0 / 256.0)
                .abs()
                < 1e-15
        );
    }

    #[test]
    fn controlled_power_exponent_is_deterministic() {
        assert_eq!(
            checked_power_of_two(0).unwrap(),
            1
        );

        assert_eq!(
            checked_power_of_two(5).unwrap(),
            32
        );
    }

    #[test]
    fn controlled_power_overflow_is_rejected() {
        assert!(
            checked_power_of_two(64).is_err()
        );
    }

    #[test]
    fn generated_circuit_has_expected_dimensions() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let circuit =
            generator
                .generate_circuit(
                    &request(4, Vec::new()),
                )
                .unwrap();

        assert_eq!(
            circuit.num_qubits(),
            5
        );

        assert_eq!(
            circuit.num_classical_bits(),
            4
        );
    }

    #[test]
    fn generated_circuit_is_valid() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let circuit =
            generator
                .generate_circuit(
                    &request(4, vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0.25",
                        ),
                    ]),
                )
                .unwrap();

        circuit.validate().unwrap();
    }

    #[test]
    fn generated_circuit_contains_measurements() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let circuit =
            generator
                .generate_circuit(
                    &request(3, Vec::new()),
                )
                .unwrap();

        let measurements =
            circuit
                .operations()
                .iter()
                .filter(|operation| {
                    operation.kind()
                        == GateKind::Measure
                })
                .count();

        assert_eq!(measurements, 3);
    }

    #[test]
    fn generated_circuit_has_one_controlled_unitary_per_phase_qubit() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let circuit =
            generator
                .generate_circuit(
                    &request(5, Vec::new()),
                )
                .unwrap();

        let controlled_powers =
            circuit
                .operations()
                .iter()
                .filter(|operation| {
                    operation.kind()
                        == GateKind::CRZ
                })
                .count();

        assert_eq!(
            controlled_powers,
            5
        );
    }

    #[test]
    fn inverse_qft_is_present() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let circuit =
            generator
                .generate_circuit(
                    &request(4, Vec::new()),
                )
                .unwrap();

        let hadamards =
            circuit
                .operations()
                .iter()
                .filter(|operation| {
                    operation.kind()
                        == GateKind::H
                })
                .count();

        // Four initial evaluation H gates plus four inverse-QFT H gates.
        assert_eq!(
            hadamards,
            8
        );
    }

    #[test]
    fn generated_workload_contains_reproducibility_metadata() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let generated =
            generator
                .generate(
                    &request(6, vec![
                        parameter(
                            EIGENPHASE_PARAMETER,
                            "0.25",
                        ),
                    ]),
                )
                .unwrap();

        let names = generated
            .workload()
            .parameters()
            .iter()
            .map(|parameter| parameter.name())
            .collect::<Vec<_>>();

        assert!(
            names.contains(&"generator_revision")
        );

        assert!(
            names.contains(&"sequence_index")
        );

        assert!(
            names.contains(&"seed")
        );
    }

    #[test]
    fn identical_requests_generate_identical_circuits() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let request_a =
            request(
                5,
                vec![
                    parameter(
                        EIGENPHASE_PARAMETER,
                        "0.25",
                    ),
                ],
            );

        let request_b =
            request(
                5,
                vec![
                    parameter(
                        EIGENPHASE_PARAMETER,
                        "0.25",
                    ),
                ],
            );

        let circuit_a =
            generator
                .generate_circuit(&request_a)
                .unwrap();

        let circuit_b =
            generator
                .generate_circuit(&request_b)
                .unwrap();

        assert_eq!(
            circuit_a,
            circuit_b
        );
    }

    #[test]
    fn different_sequence_indices_are_recorded() {
        let generator =
            PhaseEstimationBenchmarkGenerator::new()
                .unwrap();

        let metadata =
            ApplicationGenerationMetadata::new(
                42,
                7,
                GENERATOR_REVISION,
            );

        let request =
            ApplicationGenerationRequest::with_metadata(
                APPLICATION_ID,
                WorkloadId::new(
                    "phase_instance_7",
                )
                .unwrap(),
                5,
                Vec::new(),
                metadata,
            )
            .unwrap();

        let generated =
            generator
                .generate(&request)
                .unwrap();

        assert_eq!(
            generated
                .metadata()
                .seed(),
            42
        );

        assert_eq!(
            generated
                .metadata()
                .sequence_index(),
            7
        );
    }
}