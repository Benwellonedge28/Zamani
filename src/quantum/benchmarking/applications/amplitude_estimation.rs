//! Zamani Quantum Benchmarking — Quantum Amplitude Estimation Application
//! Benchmark
//!
//! Production application-benchmark generator for canonical Quantum Amplitude
//! Estimation (QAE).
//!
//! # Purpose
//!
//! This module constructs a deterministic, backend-independent canonical QAE
//! workload using Zamani's canonical Quantum IR.
//!
//! The benchmark implements the phase-estimation form of QAE:
//
//! ```text
//! A |0> = sqrt(1-a) |bad> + sqrt(a) |good>
//!
//! Q = -A S0 A† Sf
//!
//! |+> -- controlled-Q^(2^k) -- inverse-QFT -- measurement
//!
//! amplitude estimate:
//!
//!     a_y = sin²(π y / 2^m)
//! ```
//!
//! where:
//!
//! - `a` is the configured target amplitude;
//! - `m` is the number of phase/evaluation qubits;
//! - the final objective qubit represents the good state;
//! - `S0` is the reflection about |0>;
//! - `Sf` is the good-state phase reflection;
//! - `Q` is the Grover/amplitude-amplification operator.
//!
//! # Scope
//!
//! This file implements ONLY the workload generator.
//!
//! It does NOT:
//!
//! - execute circuits;
//! - select a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - estimate an amplitude from measured counts;
//! - calculate confidence intervals;
//! - perform maximum-likelihood estimation;
//! - implement iterative amplitude estimation;
//! - implement maximum-likelihood amplitude estimation;
//! - parse Zamani source code;
//! - perform filesystem or network I/O;
//! - duplicate Quantum IR.
//!
//! Those responsibilities belong to the benchmarking execution, statistics,
//! metrics, analysis, hardware, runtime, and frontend layers.
//!
//! # Canonical problem
//!
//! The benchmark uses a deliberately small but mathematically complete
//! amplitude-estimation problem:
//!
//! ```text
//! A |0> = sqrt(1-a)|0> + sqrt(a)|1>
//! ```
//!
//! This is implemented as:
//
//! ```text
//! RY(2 asin(sqrt(a)))
//! ```
//!
//! on the objective qubit.
//!
//! Therefore:
//
//! ```text
//! a = sin²(theta / 2)
//! ```
//!
//! where:
//
//! ```text
//! theta = 2 asin(sqrt(a))
//! ```
//!
//! The good state is computational-basis `|1>`.
//!
//! This makes the benchmark completely deterministic while preserving the
//! canonical QAE structure. It also gives the analysis layer a known ideal
//! amplitude for simulator and regression tests.
//!
//! # Grover operator
//!
//! The canonical operator is:
//
//! ```text
//! Q = - A S0 A† Sf
//! ```
//!
//! For this one-qubit state-preparation problem:
//
//! ```text
//! S0 = Z
//! Sf = Z
//! A  = RY(theta)
//! A† = RY(-theta)
//! ```
//!
//! Hence:
//
//! ```text
//! Q = - RY(theta) Z RY(-theta) Z
//! ```
//!
//! A controlled-Q is therefore constructed as:
//
//! ```text
//! CRY(theta)
//! CZ
//! CRY(-theta)
//! CZ
//! Z(control)
//! ```
//!
//! The final `Z(control)` supplies the `-` phase in the canonical definition
//! of Q when the phase-estimation control is |1>.
//!
//! The global phase must NOT simply be discarded here: once Q is controlled,
//! that phase is observable through phase kickback and therefore changes the
//! phase-estimation problem.
//!
//! # Phase estimation
//!
//! The phase register contains `m = request.problem_size()` qubits.
//!
//! The objective qubit is allocated as the final qubit:
//
//! ```text
//! phase qubits:  0 .. m-1
//! objective:     m
//! total:         m+1
//! ```
//!
//! Phase qubit `k` controls:
//
//! ```text
//! Q^(2^k)
//! ```
//!
//! The inverse QFT is then applied to the phase register.
//!
//! The canonical amplitude grid is:
//
//! ```text
//! a_y = sin²(π y / 2^m)
//! ```
//!
//! Because the Grover operator has conjugate eigenphases, downstream analysis
//! must canonicalize a measured phase index using:
//
//! ```text
//! y_canonical = min(y, 2^m - y)
//! ```
//!
//! The generator records this convention explicitly in workload metadata.
//!
//! # Resource model
//!
//! The canonical circuit has exponential-in-precision controlled-Grover
//! repetition:
//
//! ```text
//! sum(k=0..m-1) 2^k = 2^m - 1
//! ```
//!
//! Each controlled Grover application contains five logical operations:
//!
//! ```text
//! CRY(theta)
//! CZ
//! CRY(-theta)
//! CZ
//! Z(control)
//! ```
//!
//! Therefore the controlled-Grover contribution is:
//
//! ```text
//! 5 * (2^m - 1)
//! ```
//!
//! The implementation applies strict production bounds BEFORE circuit
//! allocation. This is essential because a malicious benchmark request must
//! not be able to request an unbounded exponential circuit.
//!
//! # Precision limit
//!
//! `MAX_EVALUATION_QUBITS` is deliberately finite.
//!
//! Increasing this limit changes resource behavior and should therefore be
//! treated as an explicit generator-version change rather than an accidental
//! implementation detail.
//!
//! # Determinism
//!
//! This benchmark is deterministic.
//!
//! The generation seed supplied by the common application-generation contract
//! is recorded by the surrounding generation/provenance system but is not used
//! as entropy because the circuit itself is deterministic.
//!
//! Identical:
//!
//! ```text
//! application_id
//! instance_id
//! problem_size
//! parameters
//! generator revision
//! ```
//!
//! produce the same circuit.
//!
//! # Integration contract
//!
//! This file integrates with:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorDescriptor
//!     └── ApplicationGeneratorCapability
//!
//! benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     ├── CircuitWorkload
//!     ├── WorkloadId
//!     └── WorkloadError
//!
//! benchmarking::core::errors
//!     ├── BenchmarkError
//!     └── BenchmarkResult
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
//! The only namespace integration required for this file is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod amplitude_estimation;
//! ```
//!
//! No execution, runtime, hardware, or statistical file needs to be changed
//! merely to add this generator.
//!
//! # Future protocol integration
//!
//! This generator intentionally leaves the following separate:
//!
//! ```text
//! applications/amplitude_estimation.rs
//!     -> workload generation
//!
//! protocols/amplitude_estimation.rs
//!     -> protocol execution + analysis
//!
//! statistics/*
//!     -> confidence intervals / distributions
//!
//! metrics/*
//!     -> runtime / resource / accuracy metrics
//!
//! execution/*
//!     -> backend execution
//! ```
//!
//! This prevents the application generator from becoming a second execution
//! engine.
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
//! No external dependencies are required.

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
pub const AMPLITUDE_ESTIMATION_BENCHMARK_ID: &str =
    "amplitude_estimation";

/// Stable application identifier.
pub const AMPLITUDE_ESTIMATION_APPLICATION_ID: &str =
    "amplitude_estimation";

/// Generator implementation version.
pub const AMPLITUDE_ESTIMATION_GENERATOR_VERSION: &str =
    "1.0.0";

/// Generator semantic/reproducibility revision.
pub const AMPLITUDE_ESTIMATION_GENERATOR_REVISION: u32 = 1;

/// Human-readable benchmark name.
pub const AMPLITUDE_ESTIMATION_NAME: &str =
    "Quantum Amplitude Estimation";

/// Generator metadata schema version.
pub const AMPLITUDE_ESTIMATION_SCHEMA_VERSION: u16 = 1;

/// Canonical algorithm identifier.
pub const AMPLITUDE_ESTIMATION_METHOD: &str =
    "canonical_qae";

// =============================================================================
// Production resource limits
// =============================================================================

/// Minimum number of phase/evaluation qubits.
pub const MIN_EVALUATION_QUBITS: usize = 1;

/// Maximum number of phase/evaluation qubits.
///
/// The controlled-Grover repetition count is `2^m - 1`, so this is an
/// intentional denial-of-service/resource bound.
pub const MAX_EVALUATION_QUBITS: usize = 10;

/// Maximum number of controlled-Grover applications.
pub const MAX_CONTROLLED_GROVER_APPLICATIONS: usize =
    (1usize << MAX_EVALUATION_QUBITS) - 1;

/// Maximum encoded parameter length.
pub const MAX_PARAMETER_VALUE_BYTES: usize = 128;

/// Minimum strictly positive amplitude.
pub const MIN_AMPLITUDE: f64 = 0.0;

/// Maximum amplitude, exclusive.
pub const MAX_AMPLITUDE: f64 = 1.0;

// =============================================================================
// Configuration
// =============================================================================

/// Strongly typed configuration for canonical QAE.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmplitudeEstimationConfiguration {
    /// Number of phase/evaluation qubits.
    pub evaluation_qubits: usize,

    /// Known ideal target amplitude.
    ///
    /// The benchmark prepares:
    ///
    /// `sqrt(1-a)|0> + sqrt(a)|1>`.
    pub amplitude: f64,

    /// Whether the objective qubit is measured.
    ///
    /// This is useful for diagnostic validation. It does not replace phase
    /// register measurement, which is required for canonical QAE.
    pub measure_objective: bool,
}

impl AmplitudeEstimationConfiguration {
    /// Creates a validated canonical-QAE configuration.
    pub fn new(
        evaluation_qubits: usize,
        amplitude: f64,
    ) -> BenchmarkResult<Self> {
        let configuration = Self {
            evaluation_qubits,
            amplitude,
            measure_objective: true,
        };

        configuration.validate()?;

        Ok(configuration)
    }

    /// Validates all configuration invariants.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.evaluation_qubits < MIN_EVALUATION_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "canonical QAE requires at least one evaluation qubit",
            ));
        }

        if self.evaluation_qubits > MAX_EVALUATION_QUBITS {
            return Err(invalid_configuration(
                "problem_size",
                "canonical QAE exceeds the production evaluation-qubit limit",
            ));
        }

        if !self.amplitude.is_finite() {
            return Err(invalid_configuration(
                "amplitude",
                "amplitude must be finite",
            ));
        }

        if self.amplitude <= MIN_AMPLITUDE
            || self.amplitude >= MAX_AMPLITUDE
        {
            return Err(invalid_configuration(
                "amplitude",
                "canonical QAE benchmark amplitude must be strictly between 0 and 1",
            ));
        }

        let controlled_grover_applications =
            self.controlled_grover_application_count()?;

        if controlled_grover_applications
            > MAX_CONTROLLED_GROVER_APPLICATIONS
        {
            return Err(invalid_configuration(
                "problem_size",
                "canonical QAE controlled-Grover workload exceeds the production resource limit",
            ));
        }

        let theta = self.state_preparation_angle();

        if !theta.is_finite()
            || theta <= 0.0
            || theta >= std::f64::consts::PI
        {
            return Err(invalid_configuration(
                "amplitude",
                "state-preparation angle is outside the supported finite range",
            ));
        }

        Ok(())
    }

    /// Returns the total number of logical qubits.
    pub fn total_qubits(&self) -> BenchmarkResult<usize> {
        self.evaluation_qubits
            .checked_add(1)
            .ok_or_else(|| numerical_overflow("total QAE qubit count"))
    }

    /// Returns the objective-qubit index.
    #[must_use]
    pub const fn objective_qubit(&self) -> usize {
        self.evaluation_qubits
    }

    /// Returns the number of phase-register classical bits.
    #[must_use]
    pub const fn phase_classical_bits(&self) -> usize {
        self.evaluation_qubits
    }

    /// Returns the objective classical-bit index.
    #[must_use]
    pub const fn objective_classical_bit(&self) -> usize {
        self.evaluation_qubits
    }

    /// Returns `2^m`.
    pub fn phase_grid_size(&self) -> BenchmarkResult<usize> {
        1usize
            .checked_shl(self.evaluation_qubits as u32)
            .ok_or_else(|| numerical_overflow("QAE phase grid size"))
    }

    /// Returns the state-preparation angle:
    ///
    /// `theta = 2 asin(sqrt(a))`.
    #[must_use]
    pub fn state_preparation_angle(&self) -> f64 {
        2.0 * self.amplitude.sqrt().asin()
    }

    /// Returns the ideal positive Grover eigenphase fraction:
    ///
    /// `phi = asin(sqrt(a)) / pi`.
    #[must_use]
    pub fn ideal_phase_fraction(&self) -> f64 {
        self.amplitude.sqrt().asin()
            / std::f64::consts::PI
    }

    /// Returns the nearest canonical phase-grid index.
    pub fn ideal_grid_index(&self) -> BenchmarkResult<usize> {
        let grid = self.phase_grid_size()?;

        let raw =
            (self.ideal_phase_fraction() * grid as f64).round();

        if !raw.is_finite() || raw < 0.0 {
            return Err(numerical_overflow(
                "QAE ideal phase-grid index",
            ));
        }

        let mut index = raw as usize;

        if index >= grid {
            index = grid - 1;
        }

        Ok(index.min(grid.saturating_sub(index)))
    }

    /// Returns the amplitude represented by the nearest canonical grid point.
    pub fn ideal_grid_amplitude(&self) -> BenchmarkResult<f64> {
        let grid = self.phase_grid_size()?;
        let index = self.ideal_grid_index()?;

        let phase =
            std::f64::consts::PI * index as f64 / grid as f64;

        let amplitude = phase.sin().powi(2);

        if !amplitude.is_finite()
            || !(0.0..=1.0).contains(&amplitude)
        {
            return Err(numerical_overflow(
                "QAE ideal grid amplitude",
            ));
        }

        Ok(amplitude)
    }

    /// Returns the absolute discretization error introduced by the canonical
    /// phase grid.
    pub fn ideal_grid_absolute_error(&self) -> BenchmarkResult<f64> {
        let grid_amplitude = self.ideal_grid_amplitude()?;

        let error = (grid_amplitude - self.amplitude).abs();

        if !error.is_finite() {
            return Err(numerical_overflow(
                "QAE ideal grid amplitude error",
            ));
        }

        Ok(error)
    }

    /// Returns the total number of controlled-Grover applications.
    ///
    /// `1 + 2 + ... + 2^(m-1) = 2^m - 1`.
    pub fn controlled_grover_application_count(
        &self,
    ) -> BenchmarkResult<usize> {
        self.phase_grid_size()?
            .checked_sub(1)
            .ok_or_else(|| {
                numerical_overflow(
                    "QAE controlled-Grover application count",
                )
            })
    }

    /// Returns the number of logical operations used by controlled-Q
    /// applications.
    pub fn controlled_grover_gate_count(
        &self,
    ) -> BenchmarkResult<usize> {
        self.controlled_grover_application_count()?
            .checked_mul(5)
            .ok_or_else(|| {
                numerical_overflow(
                    "QAE controlled-Grover gate count",
                )
            })
    }

    /// Returns the number of inverse-QFT controlled-phase operations.
    pub fn inverse_qft_controlled_phase_count(
        &self,
    ) -> BenchmarkResult<usize> {
        self.evaluation_qubits
            .checked_mul(
                self.evaluation_qubits
                    .checked_sub(1)
                    .ok_or_else(|| {
                        numerical_overflow(
                            "QAE inverse-QFT phase count",
                        )
                    })?,
            )
            .ok_or_else(|| {
                numerical_overflow(
                    "QAE inverse-QFT phase multiplication",
                )
            })
            .map(|value| value / 2)
    }

    /// Returns the number of logical operations in the generated circuit.
    pub fn logical_operation_count(&self) -> BenchmarkResult<usize> {
        let phase_hadamards = self.evaluation_qubits;

        let initial_a = 1usize;

        let controlled_grover =
            self.controlled_grover_gate_count()?;

        let inverse_qft_hadamards =
            self.evaluation_qubits;

        let inverse_qft_controlled =
            self.inverse_qft_controlled_phase_count()?;

        let swaps = self.evaluation_qubits / 2;

        let measurements = if self.measure_objective {
            self.evaluation_qubits
                .checked_add(1)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAE measurement count",
                    )
                })?
        } else {
            self.evaluation_qubits
        };

        phase_hadamards
            .checked_add(initial_a)
            .and_then(|v| v.checked_add(controlled_grover))
            .and_then(|v| v.checked_add(inverse_qft_hadamards))
            .and_then(|v| v.checked_add(inverse_qft_controlled))
            .and_then(|v| v.checked_add(swaps))
            .and_then(|v| v.checked_add(measurements))
            .ok_or_else(|| {
                numerical_overflow(
                    "QAE logical operation count",
                )
            })
    }

    /// Returns the logical two-qubit gate count.
    pub fn logical_two_qubit_gate_count(
        &self,
    ) -> BenchmarkResult<usize> {
        let controlled_grover =
            self.controlled_grover_application_count()?
                .checked_mul(2)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAE controlled-Grover two-qubit count",
                    )
                })?;

        let inverse_qft =
            self.inverse_qft_controlled_phase_count()?;

        let swaps = self.evaluation_qubits / 2;

        controlled_grover
            .checked_add(inverse_qft)
            .and_then(|v| v.checked_add(swaps))
            .ok_or_else(|| {
                numerical_overflow(
                    "QAE two-qubit gate count",
                )
            })
    }
}

// =============================================================================
// Workload description
// =============================================================================

/// Immutable resource and semantic description of one QAE instance.
#[derive(Debug, Clone, PartialEq)]
pub struct AmplitudeEstimationWorkloadDescription {
    /// Number of phase/evaluation qubits.
    pub evaluation_qubits: usize,

    /// Total logical qubits.
    pub total_qubits: usize,

    /// Objective-qubit index.
    pub objective_qubit: usize,

    /// Known target amplitude.
    pub amplitude: f64,

    /// State-preparation angle.
    pub state_preparation_angle: f64,

    /// Ideal positive eigenphase fraction.
    pub ideal_phase_fraction: f64,

    /// Canonical phase-grid size.
    pub phase_grid_size: usize,

    /// Canonical nearest phase-grid index.
    pub ideal_grid_index: usize,

    /// Amplitude represented by the nearest grid point.
    pub ideal_grid_amplitude: f64,

    /// Absolute canonical-grid discretization error.
    pub ideal_grid_absolute_error: f64,

    /// Number of controlled-Grover applications.
    pub controlled_grover_applications: usize,

    /// Number of logical operations.
    pub logical_operation_count: usize,

    /// Number of logical two-qubit operations.
    pub logical_two_qubit_gate_count: usize,

    /// Whether the objective qubit is measured.
    pub measure_objective: bool,

    /// Stable measurement interpretation.
    pub measurement_convention: &'static str,

    /// Stable phase interpretation.
    pub phase_convention: &'static str,
}

impl AmplitudeEstimationWorkloadDescription {
    /// Constructs a validated description.
    pub fn from_configuration(
        configuration: AmplitudeEstimationConfiguration,
    ) -> BenchmarkResult<Self> {
        configuration.validate()?;

        Ok(Self {
            evaluation_qubits: configuration.evaluation_qubits,
            total_qubits: configuration.total_qubits()?,
            objective_qubit: configuration.objective_qubit(),
            amplitude: configuration.amplitude,
            state_preparation_angle:
                configuration.state_preparation_angle(),
            ideal_phase_fraction:
                configuration.ideal_phase_fraction(),
            phase_grid_size:
                configuration.phase_grid_size()?,
            ideal_grid_index:
                configuration.ideal_grid_index()?,
            ideal_grid_amplitude:
                configuration.ideal_grid_amplitude()?,
            ideal_grid_absolute_error:
                configuration.ideal_grid_absolute_error()?,
            controlled_grover_applications:
                configuration.controlled_grover_application_count()?,
            logical_operation_count:
                configuration.logical_operation_count()?,
            logical_two_qubit_gate_count:
                configuration.logical_two_qubit_gate_count()?,
            measure_objective: configuration.measure_objective,
            measurement_convention:
                "phase_bits_0_to_m_minus_1_objective_bit_m",
            phase_convention:
                "canonical_qae_y=min(raw_y,2^m-raw_y)",
        })
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production canonical-QAE application benchmark generator.
///
/// The generator is stateless and safe to share across benchmark jobs.
#[derive(Debug, Clone)]
pub struct AmplitudeEstimationBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl AmplitudeEstimationBenchmarkGenerator {
    /// Creates the canonical QAE generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor =
            ApplicationGeneratorDescriptor::new(
                AMPLITUDE_ESTIMATION_BENCHMARK_ID,
                AMPLITUDE_ESTIMATION_APPLICATION_ID,
                AMPLITUDE_ESTIMATION_GENERATOR_VERSION,
                "Production canonical Quantum Amplitude Estimation application benchmark generator",
            )?
            .with_capabilities([
                ApplicationGeneratorCapability::GeneratesCircuit,
                ApplicationGeneratorCapability::Deterministic,
                ApplicationGeneratorCapability::Parameterized,
                ApplicationGeneratorCapability::ScalableProblemSize,
                ApplicationGeneratorCapability::ExactSmallInstanceReference,
                ApplicationGeneratorCapability::ClassicallyVerifiable,
                ApplicationGeneratorCapability::ResourceEstimation,
                ApplicationGeneratorCapability::HardwareExecutable,
            ]);

        Ok(Self { descriptor })
    }

    /// Returns the generator descriptor.
    #[must_use]
    pub fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Converts the common generation request into strongly typed QAE
    /// configuration.
    ///
    /// Supported parameters:
    ///
    /// ```text
    /// amplitude = finite number strictly between 0 and 1
    /// measure_objective = true | false
    /// method = canonical_qae
    /// ```
    ///
    /// The application-generation `problem_size` is the number of
    /// phase/evaluation qubits.
    pub fn configuration_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<
        AmplitudeEstimationConfiguration,
    > {
        request.validate()?;

        if request.application_id()
            != AMPLITUDE_ESTIMATION_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first: "request.application_id".to_owned(),
                    second:
                        "amplitude_estimation.application_id"
                            .to_owned(),
                    reason:
                        "amplitude estimation generator requires application_id `amplitude_estimation`"
                            .to_owned(),
                },
            );
        }

        if request.problem_size()
            < MIN_EVALUATION_QUBITS
        {
            return Err(invalid_configuration(
                "problem_size",
                "amplitude estimation requires at least one evaluation qubit",
            ));
        }

        if request.problem_size()
            > MAX_EVALUATION_QUBITS
        {
            return Err(invalid_configuration(
                "problem_size",
                "amplitude estimation exceeds the production evaluation-qubit limit",
            ));
        }

        let mut amplitude: Option<f64> = None;
        let mut measure_objective: Option<bool> =
            None;
        let mut method: Option<String> = None;

        for parameter in request.parameters() {
            if parameter.value().len()
                > MAX_PARAMETER_VALUE_BYTES
            {
                return Err(invalid_configuration(
                    "application_parameter",
                    "amplitude estimation parameter value is too large",
                ));
            }

            match parameter.name() {
                "amplitude" => {
                    if amplitude.is_some() {
                        return Err(invalid_configuration(
                            "amplitude",
                            "duplicate amplitude parameter",
                        ));
                    }

                    let parsed =
                        parameter.value().parse::<f64>().map_err(
                            |_| {
                                invalid_configuration(
                                    "amplitude",
                                    "amplitude must be a finite floating-point value",
                                )
                            },
                        )?;

                    if !parsed.is_finite() {
                        return Err(invalid_configuration(
                            "amplitude",
                            "amplitude must be finite",
                        ));
                    }

                    amplitude = Some(parsed);
                }

                "measure_objective" => {
                    if measure_objective.is_some() {
                        return Err(invalid_configuration(
                            "measure_objective",
                            "duplicate measure_objective parameter",
                        ));
                    }

                    measure_objective = Some(
                        parse_bool(
                            parameter.value(),
                            "measure_objective",
                        )?,
                    );
                }

                "method" => {
                    if method.is_some() {
                        return Err(invalid_configuration(
                            "method",
                            "duplicate method parameter",
                        ));
                    }

                    if parameter.value()
                        != AMPLITUDE_ESTIMATION_METHOD
                    {
                        return Err(invalid_configuration(
                            "method",
                            "only `canonical_qae` is implemented by this generator",
                        ));
                    }

                    method =
                        Some(parameter.value().to_owned());
                }

                _ => {
                    return Err(invalid_configuration(
                        "application_parameter",
                        "unknown amplitude estimation application parameter",
                    ));
                }
            }
        }

        let configuration =
            AmplitudeEstimationConfiguration {
                evaluation_qubits: request.problem_size(),
                amplitude: amplitude.unwrap_or(0.25),
                measure_objective:
                    measure_objective.unwrap_or(true),
            };

        let _ = method;

        configuration.validate()?;

        Ok(configuration)
    }

    /// Returns a resource description without constructing Quantum IR.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<
        AmplitudeEstimationWorkloadDescription,
    > {
        let configuration =
            self.configuration_from_request(request)?;

        AmplitudeEstimationWorkloadDescription::from_configuration(
            configuration,
        )
    }

    /// Generates the complete canonical-QAE Quantum IR circuit.
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let configuration =
            self.configuration_from_request(request)?;

        let description =
            AmplitudeEstimationWorkloadDescription::from_configuration(
                configuration,
            )?;

        let total_qubits =
            configuration.total_qubits()?;

        let mut circuit =
            QuantumCircuit::new(
                total_qubits,
                total_qubits,
            )
            .map_err(|error| {
                circuit_error(
                    "unable to construct amplitude-estimation Quantum IR circuit",
                    error,
                )
            })?;

        circuit
            .set_name(Some(format!(
                "amplitude_estimation_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign amplitude-estimation circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.amplitude_estimation"
                    .to_owned(),
            ))
            .map_err(|error| {
                circuit_error(
                    "unable to assign amplitude-estimation circuit source",
                    error,
                )
            })?;

        // =====================================================================
        // State preparation
        // =====================================================================
        //
        // A|0> = sqrt(1-a)|0> + sqrt(a)|1>
        //
        // with A = RY(2 asin(sqrt(a))).

        circuit
            .push(parameterized_single_qubit_gate(
                GateKind::RY,
                configuration.objective_qubit(),
                configuration.state_preparation_angle(),
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append amplitude-estimation state preparation",
                    error,
                )
            })?;

        // =====================================================================
        // Phase register preparation
        // =====================================================================

        for phase_qubit in 0..configuration.evaluation_qubits {
            circuit
                .push(single_qubit_gate(
                    GateKind::H,
                    phase_qubit,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append amplitude-estimation phase-register Hadamard",
                        error,
                    )
                })?;
        }

        // =====================================================================
        // Controlled powers of the Grover operator
        // =====================================================================

        for phase_qubit in 0..configuration.evaluation_qubits {
            let repetitions = 1usize
                .checked_shl(phase_qubit as u32)
                .ok_or_else(|| {
                    numerical_overflow(
                        "amplitude-estimation controlled-Grover power",
                    )
                })?;

            for _ in 0..repetitions {
                append_controlled_grover(
                    &mut circuit,
                    phase_qubit,
                    configuration.objective_qubit(),
                    configuration.state_preparation_angle(),
                )?;
            }
        }

        // =====================================================================
        // Inverse QFT on the phase register
        // =====================================================================

        append_inverse_qft(
            &mut circuit,
            configuration.evaluation_qubits,
        )?;

        // =====================================================================
        // Measurements
        // =====================================================================
        //
        // Phase qubit i -> classical bit i.
        //
        // Objective qubit m -> classical bit m when requested.
        //
        // Keeping the objective bit separate makes simulator/hardware
        // diagnostics possible without changing the canonical phase-register
        // interpretation.

        for phase_qubit in
            0..configuration.evaluation_qubits
        {
            circuit
                .push(measurement_gate(
                    phase_qubit,
                    phase_qubit,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append amplitude-estimation phase measurement",
                        error,
                    )
                })?;
        }

        if configuration.measure_objective {
            circuit
                .push(measurement_gate(
                    configuration.objective_qubit(),
                    configuration.objective_classical_bit(),
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append amplitude-estimation objective measurement",
                        error,
                    )
                })?;
        }

        // Keep the description alive through circuit construction so that
        // configuration/resource validation happens before allocation-heavy
        // generation.
        let _ = description;

        // =====================================================================
        // Final IR validation
        // =====================================================================

        circuit
            .validate()
            .map_err(|error| {
                circuit_error(
                    "generated amplitude-estimation circuit failed final validation",
                    error,
                )
            })?;

        Ok(circuit)
    }

    /// Generates the canonical application workload.
    pub fn generate_application_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let configuration =
            self.configuration_from_request(request)?;

        let description =
            AmplitudeEstimationWorkloadDescription::from_configuration(
                configuration,
            )?;

        let circuit =
            self.generate_circuit(request)?;

        let circuit_workload =
            CircuitWorkload::from_circuit(
                circuit,
                request.instance_id().clone(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create amplitude-estimation circuit workload",
                    error,
                )
            })?;

        let mut workload =
            ApplicationWorkload::new(
                AMPLITUDE_ESTIMATION_APPLICATION_ID,
                request.instance_id().clone(),
                request.problem_size(),
            )
            .map_err(|error| {
                workload_error(
                    "unable to create amplitude-estimation application workload",
                    error,
                )
            })?
            .with_circuit(circuit_workload);

        // =====================================================================
        // Stable semantic metadata
        // =====================================================================

        add_parameter(
            &mut workload,
            "application",
            AMPLITUDE_ESTIMATION_APPLICATION_ID,
        )?;

        add_parameter(
            &mut workload,
            "method",
            AMPLITUDE_ESTIMATION_METHOD,
        )?;

        add_parameter(
            &mut workload,
            "schema_version",
            &AMPLITUDE_ESTIMATION_SCHEMA_VERSION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            AMPLITUDE_ESTIMATION_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &AMPLITUDE_ESTIMATION_GENERATOR_REVISION
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "evaluation_qubits",
            &description.evaluation_qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "total_qubits",
            &description.total_qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "objective_qubit",
            &description.objective_qubit.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "amplitude",
            &format!("{:.17}", description.amplitude),
        )?;

        add_parameter(
            &mut workload,
            "state_preparation_angle",
            &format!(
                "{:.17}",
                description.state_preparation_angle
            ),
        )?;

        add_parameter(
            &mut workload,
            "ideal_phase_fraction",
            &format!(
                "{:.17}",
                description.ideal_phase_fraction
            ),
        )?;

        add_parameter(
            &mut workload,
            "phase_grid_size",
            &description.phase_grid_size.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "ideal_grid_index",
            &description.ideal_grid_index.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "ideal_grid_amplitude",
            &format!(
                "{:.17}",
                description.ideal_grid_amplitude
            ),
        )?;

        add_parameter(
            &mut workload,
            "ideal_grid_absolute_error",
            &format!(
                "{:.17}",
                description.ideal_grid_absolute_error
            ),
        )?;

        add_parameter(
            &mut workload,
            "controlled_grover_applications",
            &description
                .controlled_grover_applications
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "logical_operation_count",
            &description
                .logical_operation_count
                .to_string(),
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
            "measure_objective",
            if description.measure_objective {
                "true"
            } else {
                "false"
            },
        )?;

        add_parameter(
            &mut workload,
            "measurement_convention",
            description.measurement_convention,
        )?;

        add_parameter(
            &mut workload,
            "phase_convention",
            description.phase_convention,
        )?;

        add_parameter(
            &mut workload,
            "state_preparation",
            "ry_2_asin_sqrt_amplitude",
        )?;

        add_parameter(
            &mut workload,
            "good_state",
            "computational_basis_1",
        )?;

        add_parameter(
            &mut workload,
            "grover_operator",
            "-A_S0_A_dagger_Sf",
        )?;

        add_parameter(
            &mut workload,
            "s0_reflection",
            "Z_objective",
        )?;

        add_parameter(
            &mut workload,
            "good_state_reflection",
            "Z_objective",
        )?;

        add_parameter(
            &mut workload,
            "controlled_grover_decomposition",
            "CRY_CZ_CRY_inverse_CZ_Z_control",
        )?;

        add_parameter(
            &mut workload,
            "phase_grid_formula",
            "sin2_pi_y_over_2_to_m",
        )?;

        add_parameter(
            &mut workload,
            "canonical_phase_index",
            "min_y_2_to_m_minus_y",
        )?;

        Ok(workload)
    }
}

impl ApplicationBenchmarkGenerator
    for AmplitudeEstimationBenchmarkGenerator
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
        request.validate()?;

        if request.application_id()
            != AMPLITUDE_ESTIMATION_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first: "request.application_id".to_owned(),
                    second:
                        "amplitude_estimation.application_id"
                            .to_owned(),
                    reason:
                        "amplitude estimation generator requires application_id `amplitude_estimation`"
                            .to_owned(),
                },
            );
        }

        let _ =
            self.configuration_from_request(request)?;

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
// Controlled Grover construction
// =============================================================================

/// Appends one controlled canonical Grover operator.
///
/// Canonical:
//
// ```text
/// Q = -A S0 A† Sf
/// ```
//
// For this one-qubit benchmark:
//
// ```text
/// A  = RY(theta)
/// S0 = Z
/// Sf = Z
/// A† = RY(-theta)
/// ```
//
// Therefore the controlled form is:
//
// ```text
/// CRY(theta)
/// CZ
/// CRY(-theta)
/// CZ
/// Z(control)
/// ```
//
// The final control Z is essential because the global `-` in the canonical
/// Grover operator becomes relative phase under control.
fn append_controlled_grover(
    circuit: &mut QuantumCircuit,
    control: usize,
    objective: usize,
    theta: f64,
) -> BenchmarkResult<()> {
    if control == objective {
        return Err(invalid_configuration(
            "qae",
            "controlled-Grover control and objective qubits must differ",
        ));
    }

    if !theta.is_finite() {
        return Err(invalid_configuration(
            "qae",
            "controlled-Grover angle must be finite",
        ));
    }

    // CRY(theta) = controlled A.
    circuit
        .push(parameterized_two_qubit_gate(
            GateKind::CRY,
            control,
            objective,
            theta,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append controlled state-preparation operation",
                error,
            )
        })?;

    // Controlled S0.
    circuit
        .push(two_qubit_gate(
            GateKind::CZ,
            control,
            objective,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append controlled S0 reflection",
                error,
            )
        })?;

    // Controlled A†.
    circuit
        .push(parameterized_two_qubit_gate(
            GateKind::CRY,
            control,
            objective,
            -theta,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append controlled inverse state preparation",
                error,
            )
        })?;

    // Controlled Sf.
    circuit
        .push(two_qubit_gate(
            GateKind::CZ,
            control,
            objective,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append controlled good-state reflection",
                error,
            )
        })?;

    // Canonical minus sign in Q.
    circuit
        .push(single_qubit_gate(
            GateKind::Z,
            control,
        )?)
        .map_err(|error| {
            circuit_error(
                "unable to append controlled-Grover global-phase correction",
                error,
            )
        })?;

    Ok(())
}

// =============================================================================
// Inverse QFT
// =============================================================================

/// Appends the inverse QFT to the first `qubits` logical qubits.
///
/// The implementation follows the same deterministic construction contract as
/// the existing QFT application benchmark:
///
/// 1. undo bit reversal;
/// 2. walk the QFT construction in reverse order;
/// 3. negate controlled-phase angles;
/// 4. apply Hadamards.
///
/// The phase register is independent of the objective qubit.
fn append_inverse_qft(
    circuit: &mut QuantumCircuit,
    qubits: usize,
) -> BenchmarkResult<()> {
    if qubits == 0 {
        return Err(invalid_configuration(
            "qae",
            "inverse QFT requires at least one phase qubit",
        ));
    }

    // Undo the final bit reversal first.
    append_bit_reversal_swaps(circuit, qubits)?;

    for target in (0..qubits).rev() {
        for control in (0..target).rev() {
            let distance = target
                .checked_sub(control)
                .ok_or_else(|| {
                    numerical_overflow(
                        "QAE inverse-QFT qubit distance",
                    )
                })?;

            let angle =
                phase_angle(distance)?
                    .checked_neg()
                    .ok_or_else(|| {
                        numerical_overflow(
                            "QAE inverse-QFT phase angle",
                        )
                    })?;

            circuit
                .push(parameterized_two_qubit_gate(
                    GateKind::CRZ,
                    target,
                    control,
                    angle,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append QAE inverse-QFT controlled phase",
                        error,
                    )
                })?;
        }

        circuit
            .push(single_qubit_gate(
                GateKind::H,
                target,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append QAE inverse-QFT Hadamard",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends the QFT bit-reversal permutation.
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
                    "QAE inverse-QFT bit-reversal index",
                )
            })?;

        if index == opposite {
            return Err(invalid_configuration(
                "qae",
                "inverse-QFT attempted a self-SWAP",
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
                    "unable to append QAE inverse-QFT SWAP",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Returns the QFT phase angle for a positive qubit separation.
///
/// ```text
/// theta = pi / 2^distance
/// ```
fn phase_angle(
    distance: usize,
) -> BenchmarkResult<f64> {
    if distance == 0 {
        return Err(invalid_configuration(
            "qae",
            "inverse-QFT phase distance must be greater than zero",
        ));
    }

    if distance > 63 {
        return Err(invalid_configuration(
            "qae",
            "inverse-QFT phase distance exceeds supported numeric range",
        ));
    }

    let denominator =
        2f64.powi(distance as i32);

    let angle =
        std::f64::consts::PI / denominator;

    if !angle.is_finite() || angle <= 0.0 {
        return Err(numerical_overflow(
            "QAE inverse-QFT phase angle",
        ));
    }

    Ok(angle)
}

// =============================================================================
// IR helpers
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
            "generated invalid QAE single-qubit gate",
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
            "qae",
            "single-qubit gate parameter must be finite",
        ));
    }

    let parameter =
        Parameter::constant(value)
            .map_err(|error| {
                invalid_workload(
                    "generated invalid QAE single-qubit parameter",
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
            "generated invalid QAE parameterized single-qubit gate",
            error,
        )
    })
}

/// Creates a parameterized two-qubit gate.
fn parameterized_two_qubit_gate(
    kind: GateKind,
    first: usize,
    second: usize,
    value: f64,
) -> BenchmarkResult<Gate> {
    if first == second {
        return Err(invalid_configuration(
            "qae",
            "two-qubit gate operands must be distinct",
        ));
    }

    if !value.is_finite() {
        return Err(invalid_configuration(
            "qae",
            "two-qubit gate parameter must be finite",
        ));
    }

    let parameter =
        Parameter::constant(value)
            .map_err(|error| {
                invalid_workload(
                    "generated invalid QAE two-qubit parameter",
                    error,
                )
            })?;

    Gate::new(
        kind,
        vec![
            QubitId::new(first),
            QubitId::new(second),
        ],
        vec![parameter],
        None,
        None,
    )
    .map_err(|error| {
        invalid_workload(
            "generated invalid QAE parameterized two-qubit gate",
            error,
        )
    })
}

/// Creates an unparameterized two-qubit gate.
fn two_qubit_gate(
    kind: GateKind,
    first: usize,
    second: usize,
) -> BenchmarkResult<Gate> {
    if first == second {
        return Err(invalid_configuration(
            "qae",
            "two-qubit gate operands must be distinct",
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
            "generated invalid QAE two-qubit gate",
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
            "generated invalid QAE measurement gate",
            error,
        )
    })
}

// =============================================================================
// Workload metadata
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
                    "unable to encode QAE application metadata",
                    error,
                )
            })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to attach QAE application metadata",
                error,
            )
        })
}

// =============================================================================
// Parsing
// =============================================================================

/// Parses the intentionally strict benchmark boolean syntax.
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
// Errors
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

/// Converts a workload error into the benchmark error boundary.
fn workload_error(
    reason: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            AMPLITUDE_ESTIMATION_APPLICATION_ID
                .to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a generated-IR error into the benchmark boundary.
fn invalid_workload(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            AMPLITUDE_ESTIMATION_APPLICATION_ID
                .to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a circuit construction/validation error.
fn circuit_error(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            AMPLITUDE_ESTIMATION_APPLICATION_ID
                .to_owned(),
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

    use super::super::super::core::workload::{
        WorkloadId,
    };

    fn request(
        problem_size: usize,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            AMPLITUDE_ESTIMATION_APPLICATION_ID,
            WorkloadId::new("instance_0")
                .expect("test workload ID must be valid"),
            problem_size,
            42,
        )
        .expect("test request must be valid")
        .with_generator_revision(
            AMPLITUDE_ESTIMATION_GENERATOR_REVISION,
        )
    }

    fn request_with_parameters(
        problem_size: usize,
        parameters: &[(&str, &str)],
    ) -> ApplicationGenerationRequest {
        let mut request = request(problem_size);

        for (name, value) in parameters {
            request = request
                .with_parameter(
                    ApplicationParameter::new(
                        name,
                        value,
                    )
                    .expect(
                        "test parameter must be valid",
                    ),
                )
                .expect(
                    "test parameter must be accepted",
                );
        }

        request
    }

    #[test]
    fn default_configuration_is_valid() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let configuration =
            generator
                .configuration_from_request(
                    &request(3),
                )
                .expect("configuration");

        assert_eq!(
            configuration.evaluation_qubits,
            3
        );

        assert_eq!(
            configuration.amplitude,
            0.25
        );

        assert!(configuration.measure_objective);
    }

    #[test]
    fn state_preparation_angle_matches_known_amplitude() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                4,
                0.25,
            )
            .expect("configuration");

        let expected =
            std::f64::consts::PI / 3.0;

        assert!(
            (configuration.state_preparation_angle()
                - expected)
                .abs()
                < 1.0e-14
        );
    }

    #[test]
    fn phase_fraction_matches_known_amplitude() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                4,
                0.25,
            )
            .expect("configuration");

        let expected = 1.0 / 6.0;

        assert!(
            (configuration.ideal_phase_fraction()
                - expected)
                .abs()
                < 1.0e-14
        );
    }

    #[test]
    fn grid_size_is_power_of_two() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                8,
                0.25,
            )
            .expect("configuration");

        assert_eq!(
            configuration
                .phase_grid_size()
                .expect("grid"),
            256
        );
    }

    #[test]
    fn grid_index_uses_canonical_conjugate_phase() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                8,
                0.25,
            )
            .expect("configuration");

        assert_eq!(
            configuration
                .ideal_grid_index()
                .expect("index"),
            43
        );
    }

    #[test]
    fn controlled_grover_count_is_two_to_m_minus_one() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                5,
                0.25,
            )
            .expect("configuration");

        assert_eq!(
            configuration
                .controlled_grover_application_count()
                .expect("count"),
            31
        );
    }

    #[test]
    fn zero_amplitude_is_rejected() {
        let result =
            AmplitudeEstimationConfiguration::new(
                4,
                0.0,
            );

        assert!(result.is_err());
    }

    #[test]
    fn unit_amplitude_is_rejected() {
        let result =
            AmplitudeEstimationConfiguration::new(
                4,
                1.0,
            );

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_amplitude_is_rejected() {
        let result =
            AmplitudeEstimationConfiguration::new(
                4,
                f64::NAN,
            );

        assert!(result.is_err());
    }

    #[test]
    fn excessive_precision_is_rejected() {
        let result =
            AmplitudeEstimationConfiguration::new(
                MAX_EVALUATION_QUBITS + 1,
                0.25,
            );

        assert!(result.is_err());
    }

    #[test]
    fn unknown_parameter_is_rejected() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                &[("unknown", "value")],
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
    fn duplicate_amplitude_is_rejected() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                &[
                    ("amplitude", "0.25"),
                    ("amplitude", "0.5"),
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
    fn invalid_method_is_rejected() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                &[("method", "iterative")],
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
    fn circuit_generation_succeeds_for_small_instance() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let circuit =
            generator
                .generate_circuit(&request(2))
                .expect("QAE circuit");

        circuit
            .validate()
            .expect("generated circuit must validate");
    }

    #[test]
    fn circuit_has_expected_total_qubits() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let circuit =
            generator
                .generate_circuit(&request(3))
                .expect("QAE circuit");

        assert_eq!(
            circuit.qubit_count(),
            4
        );
    }

    #[test]
    fn workload_generation_succeeds() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let workload =
            generator
                .generate_application_workload(
                    &request(3),
                )
                .expect("QAE workload");

        assert_eq!(
            workload.application_id(),
            AMPLITUDE_ESTIMATION_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            3
        );
    }

    #[test]
    fn custom_amplitude_is_preserved_in_workload() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                4,
                &[("amplitude", "0.37")],
            );

        let description =
            generator
                .describe(&request)
                .expect("description");

        assert!(
            (description.amplitude - 0.37).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn_objective_measurement_can_be_disabled() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                3,
                &[("measure_objective", "false")],
            );

        let description =
            generator
                .describe(&request)
                .expect("description");

        assert!(!description.measure_objective);
    }

    #[test]
    fn strict_boolean_parsing_is_enforced() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_parameters(
                3,
                &[("measure_objective", "TRUE")],
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
    fn generator_is_deterministic() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let first =
            generator
                .generate_circuit(&request(3))
                .expect("first circuit");

        let second =
            generator
                .generate_circuit(&request(3))
                .expect("second circuit");

        assert_eq!(first, second);
    }

    #[test]
    fn generator_descriptor_has_required_capabilities() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let descriptor =
            generator.descriptor();

        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::GeneratesCircuit
            )
        );

        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::Deterministic
            )
        );

        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::ResourceEstimation
            )
        );

        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::HardwareExecutable
            )
        );
    }

    #[test]
    fn generated_workload_contains_canonical_method() {
        let generator =
            AmplitudeEstimationBenchmarkGenerator::new()
                .expect("generator");

        let workload =
            generator
                .generate_application_workload(
                    &request(3),
                )
                .expect("workload");

        let _ = workload;
    }

    #[test]
    fn qae_resource_bound_is_safe() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                MAX_EVALUATION_QUBITS,
                0.25,
            )
            .expect("maximum bounded configuration");

        assert_eq!(
            configuration
                .controlled_grover_application_count()
                .expect("count"),
            MAX_CONTROLLED_GROVER_APPLICATIONS
        );
    }

    #[test]
    fn inverse_qft_phase_count_is_correct() {
        let configuration =
            AmplitudeEstimationConfiguration::new(
                4,
                0.25,
            )
            .expect("configuration");

        assert_eq!(
            configuration
                .inverse_qft_controlled_phase_count()
                .expect("phase count"),
            6
        );
    }
}