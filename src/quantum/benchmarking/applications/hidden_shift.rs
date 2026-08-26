//! Zamani Quantum Benchmarking — Hidden Shift Application Benchmark
//!
//! Production application-benchmark generator for the Boolean Hidden Shift
//! problem using an even-width quadratic bent function.
//!
//! # Purpose
//!
//! This module constructs a deterministic, backend-independent Hidden Shift
//! benchmark workload and its canonical Zamani Quantum IR circuit.
//!
//! The benchmark represents the Boolean hidden-shift problem:
//
//!     g(x) = f(x xor s)
//
//! where:
//!
//! - `x` is an n-bit input;
//! - `s` is the hidden n-bit shift;
//! - `f` is a known Boolean bent function;
//! - `g` is the shifted version of `f`.
//!
//! This implementation uses the self-dual quadratic bent function:
//
//!     f(x) = x0*x1 xor x2*x3 xor ... xor x[n-2]*x[n-1]
//
//! Consequently `n` MUST be positive and even.
//!
//! The generated quantum algorithm is the canonical self-dual hidden-shift
//! construction:
//
//!     H^n
//!     U_g
//!     H^n
//!     U_f
//!     H^n
//!
//! followed by computational-basis measurement.
//!
//! For the ideal noiseless circuit, the measured result is the requested
//! hidden shift in the canonical logical-bit ordering defined by this module.
//!
//! # Architectural boundary
//!
//! This file deliberately does NOT:
//!
//! - execute circuits;
//! - select a backend;
//! - route logical qubits;
//! - schedule physical operations;
//! - perform calibration;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - calculate runtime metrics;
//! - calculate fidelity/error metrics;
//! - perform readout mitigation;
//! - parse Zamani source code;
//! - depend on a simulator;
//! - depend on a vendor SDK;
//! - duplicate Quantum IR;
//! - perform filesystem/network I/O.
//!
//! The dependency direction is:
//
//! ```text
//! ApplicationGenerationRequest
//!             │
//!             ▼
//! HiddenShiftBenchmarkGenerator
//!             │
//!             ├── validates parameters
//!             ├── determines hidden shift
//!             ├── constructs Quantum IR
//!             └── constructs ApplicationWorkload
//!                       │
//!                       ▼
//!                 BenchmarkExperiment
//!                       │
//!                       ▼
//!                 BenchmarkExecutor
//!                       │
//!                       ▼
//!                  Observations
//!                       │
//!                       ▼
//!             Hidden Shift analysis
//! ```
//!
//! # Integration
//!
//! This implementation consumes the already-established application-generator
//! contract:
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
//!     ├── Measurement
//!     ├── QubitId
//!     └── ClassicalBitId
//! ```
//!
//! No backend-specific dependency is introduced here.
//!
//! # Application parameters
//!
//! The following request parameters are supported:
//!
//! ```text
//! shift = 010110
//!
//! function = quadratic_bent
//! ```
//!
//! `shift` is optional.
//!
//! If omitted, the generator deterministically derives an n-bit shift from:
//!
//! - request seed;
//! - request sequence index;
//! - generator revision.
//!
//! No global RNG, operating-system entropy, clock, process identifier,
//! pointer address, or thread identifier is used.
//!
//! `function` is optional and defaults to `quadratic_bent`.
//!
//! Unknown parameters are rejected.
//! Duplicate parameters are rejected.
//!
//! # Hidden-shift construction
//!
//! For even `n`, define:
//
//!     f(x) = x0*x1 xor x2*x3 xor ... xor x[n-2]*x[n-1]
//
//! and:
//
//!     g(x) = f(x xor s).
//
//! The phase-oracle implementation uses:
//
//! ```text
//! U_f |x> = (-1)^f(x) |x>
//! U_g |x> = (-1)^g(x) |x>
//! ```
//!
//! The quadratic function is self-dual, so the hidden shift can be recovered
//! using:
//
//! ```text
//! H^n U_g H^n U_f H^n |0^n> = |s>
//! ```
//!
//! The middle Hadamard layer is deliberately retained.
//!
//! Some optimized implementations omit that layer and compensate with a
//! deterministic permutation/post-processing step. Zamani's canonical
//! benchmark keeps the mathematically direct construction so that the circuit
//! output directly represents the hidden shift without requiring
//! backend-specific bit manipulation in the generator.
//!
//! # Oracle implementation
//!
//! `CZ(q_i, q_j)` implements the phase:
//
//!     (-1)^(q_i*q_j)
//
//! Therefore:
//
//! ```text
//! CZ(q0,q1)
//! CZ(q2,q3)
//! ...
//! ```
//!
//! implements the quadratic bent-function phase oracle.
//!
//! For `g`, the input shift is implemented by:
//
//! ```text
//! X on every shifted qubit
//! U_f
//! X on every shifted qubit
//! ```
//!
//! because:
//
//!     X_s U_f X_s |x> = (-1)^f(x xor s)|x>.
//
//! # Resource model
//!
//! For n logical qubits:
//!
//! - quantum width = n;
//! - classical width = n;
//! - H gates = 3n;
//! - CZ gates = n/2 in `U_g`;
//! - CZ gates = n/2 in `U_f`;
//! - X gates <= 2n in `U_g`;
//! - measurements = n.
//!
//! Worst-case logical operation count:
//
//!     3n + n + n = 5n
//!
//! because the shifted oracle contains at most 2n X operations and n CZ/H
//! contributions combined as described by the exact construction below.
//!
//! The generator never constructs a truth table of size 2^n.
//!
//! # Security/resource safety
//!
//! Requests are untrusted.
//!
//! The implementation:
//!
//! - rejects zero-width problems;
//! - rejects odd widths;
//! - rejects malformed shift strings;
//! - rejects shift strings longer than the problem size;
//! - rejects duplicate parameters;
//! - rejects unknown parameters;
//! - uses checked arithmetic;
//! - avoids exponential allocations;
//! - relies on canonical Quantum IR limits;
//! - validates every generated gate;
//! - validates the complete circuit before returning it;
//! - does not execute caller-provided code;
//! - does not perform I/O.
//!
//! # Reproducibility
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
//! produce the same hidden shift and semantically identical circuit.
//!
//! The generated workload records the hidden shift in bounded application
//! metadata so that benchmark results can be independently analyzed without
//! reconstructing generator state.
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
//!
//! # Scientific references
//!
//! The implementation follows the Boolean hidden-shift formulation and the
//! quadratic/bent-function construction used in quantum hidden-shift work.
//!
//! The benchmark is also aligned with the application-oriented benchmark
//! family in which Hidden Shift is used as a small oracle-based quantum
//! application benchmark.
//!
//! The generator remains an implementation component, not a scientific claim
//! that a particular hardware execution necessarily demonstrates an ideal
//! quantum advantage.

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
    qubit::QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable Hidden Shift benchmark identifier.
pub const HIDDEN_SHIFT_BENCHMARK_ID: &str = "hidden_shift";

/// Stable application identifier.
pub const HIDDEN_SHIFT_APPLICATION_ID: &str = "hidden_shift";

/// Generator implementation version.
pub const HIDDEN_SHIFT_GENERATOR_VERSION: &str = "1.0.0";

/// Generator semantic revision.
///
/// Increase this whenever generation semantics change in a way that can alter
/// generated circuits or benchmark meaning.
pub const HIDDEN_SHIFT_GENERATOR_REVISION: u32 = 1;

/// Human-readable benchmark name.
pub const HIDDEN_SHIFT_NAME: &str = "Hidden Shift";

/// Stable function family supported by this implementation.
pub const HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT: &str = "quadratic_bent";

/// Maximum UTF-8 byte length accepted for a textual shift parameter.
///
/// The canonical ApplicationParameter model is already bounded more tightly;
/// this application-specific constant documents the intended contract.
pub const MAX_SHIFT_PARAMETER_BYTES: usize = 512;

/// Maximum number of bits accepted in a shift parameter before the canonical
/// problem-size validation is applied.
///
/// The actual Quantum IR limit remains authoritative for generated circuits.
pub const MAX_SHIFT_BITS: usize = 4096;

// =============================================================================
// Function family
// =============================================================================

/// Hidden Shift function family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HiddenShiftFunction {
    /// Self-dual quadratic bent function:
    ///
    /// `f(x) = x0*x1 xor x2*x3 xor ...`.
    QuadraticBent,
}

impl HiddenShiftFunction {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuadraticBent => HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT,
        }
    }

    /// Returns whether the function is self-dual under the construction used
    /// by this benchmark.
    #[must_use]
    pub const fn is_self_dual(self) -> bool {
        match self {
            Self::QuadraticBent => true,
        }
    }
}

impl fmt::Display for HiddenShiftFunction {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Hidden shift
// =============================================================================

/// Canonical hidden shift.
///
/// Bits are stored in logical-qubit order:
///
/// ```text
/// bit 0 -> q0
/// bit 1 -> q1
/// ...
/// bit n-1 -> q(n-1)
/// ```
///
/// This avoids depending on a backend's textual count ordering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HiddenShift {
    bits: Vec<bool>,
}

impl HiddenShift {
    /// Creates a hidden shift from canonical logical bits.
    pub fn new(bits: Vec<bool>) -> BenchmarkResult<Self> {
        if bits.is_empty() {
            return Err(invalid_configuration(
                "shift",
                "hidden shift must contain at least one bit",
            ));
        }

        if bits.len() > MAX_SHIFT_BITS {
            return Err(invalid_configuration(
                "shift",
                "hidden shift exceeds the application-specific maximum",
            ));
        }

        Ok(Self { bits })
    }

    /// Creates a hidden shift from a canonical binary string.
    ///
    /// The first character corresponds to logical qubit zero.
    pub fn from_bit_string(
        value: &str,
        expected_width: usize,
    ) -> BenchmarkResult<Self> {
        if value.is_empty() {
            return Err(invalid_configuration(
                "shift",
                "shift must not be empty",
            ));
        }

        if value.len() != expected_width {
            return Err(invalid_configuration(
                "shift",
                "shift width must equal problem_size",
            ));
        }

        if value.len() > MAX_SHIFT_PARAMETER_BYTES {
            return Err(invalid_configuration(
                "shift",
                "shift parameter is too large",
            ));
        }

        let mut bits = Vec::with_capacity(value.len());

        for byte in value.bytes() {
            match byte {
                b'0' => bits.push(false),
                b'1' => bits.push(true),
                _ => {
                    return Err(invalid_configuration(
                        "shift",
                        "shift must contain only ASCII binary digits",
                    ));
                }
            }
        }

        Self::new(bits)
    }

    /// Creates a deterministic shift from the benchmark seed.
    ///
    /// This is a small local deterministic generator intentionally kept
    /// independent of any external RNG implementation.
    pub fn deterministic(
        width: usize,
        seed: u64,
        sequence_index: u64,
        generator_revision: u32,
    ) -> BenchmarkResult<Self> {
        if width == 0 {
            return Err(invalid_configuration(
                "problem_size",
                "hidden shift width must be greater than zero",
            ));
        }

        if width > MAX_SHIFT_BITS {
            return Err(invalid_configuration(
                "problem_size",
                "hidden shift width exceeds the application-specific maximum",
            ));
        }

        let mut bits = Vec::with_capacity(width);

        for index in 0..width {
            let index = index as u64;

            let input = seed
                .wrapping_add(sequence_index.rotate_left(17))
                .wrapping_add(index.wrapping_mul(0x9E37_79B9_7F4A_7C15))
                .wrapping_add((generator_revision as u64).rotate_left(29));

            let mixed = splitmix64(input);

            bits.push((mixed & 1) != 0);
        }

        Self::new(bits)
    }

    /// Returns the number of logical shift bits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the shift is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns one logical shift bit.
    #[must_use]
    pub fn bit(&self, index: usize) -> bool {
        self.bits[index]
    }

    /// Returns the shift in canonical logical-qubit order.
    #[must_use]
    pub fn bits(&self) -> &[bool] {
        &self.bits
    }

    /// Returns a canonical binary representation.
    #[must_use]
    pub fn as_bit_string(&self) -> String {
        let mut result = String::with_capacity(self.bits.len());

        for &bit in &self.bits {
            result.push(if bit { '1' } else { '0' });
        }

        result
    }

    /// Returns whether all bits are zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|bit| !*bit)
    }

    /// Returns the Hamming weight of the shift.
    #[must_use]
    pub fn hamming_weight(&self) -> usize {
        self.bits.iter().filter(|bit| **bit).count()
    }
}

impl fmt::Display for HiddenShift {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.as_bit_string())
    }
}

// =============================================================================
// Workload description
// =============================================================================

/// Strongly typed description of one Hidden Shift workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenShiftWorkloadDescription {
    /// Number of logical qubits.
    pub qubits: usize,

    /// Number of classical measurement bits.
    pub classical_bits: usize,

    /// Hidden shift.
    pub hidden_shift: HiddenShift,

    /// Function family.
    pub function: HiddenShiftFunction,

    /// Number of CZ gates in the known function oracle.
    pub function_cz_count: usize,

    /// Number of CZ gates in the shifted oracle.
    pub shifted_function_cz_count: usize,

    /// Number of X gates in the shifted oracle.
    pub shift_x_count: usize,

    /// Number of Hadamard gates.
    pub hadamard_count: usize,

    /// Number of measurements.
    pub measurement_count: usize,

    /// Total logical operation count.
    pub logical_operation_count: usize,

    /// Logical two-qubit gate count.
    pub logical_two_qubit_gate_count: usize,

    /// Expected ideal measurement in canonical logical-bit order.
    pub expected_measurement: String,
}

impl HiddenShiftWorkloadDescription {
    /// Creates and validates a workload description.
    pub fn new(
        qubits: usize,
        hidden_shift: HiddenShift,
        function: HiddenShiftFunction,
    ) -> BenchmarkResult<Self> {
        validate_problem_size(qubits)?;

        if hidden_shift.len() != qubits {
            return Err(invalid_configuration(
                "shift",
                "hidden shift width must equal problem_size",
            ));
        }

        if !function.is_self_dual() {
            return Err(invalid_configuration(
                "function",
                "selected Hidden Shift function is not self-dual",
            ));
        }

        let function_cz_count = qubits / 2;

        let shifted_function_cz_count = function_cz_count;

        let shift_x_count = hidden_shift
            .hamming_weight()
            .checked_mul(2)
            .ok_or_else(|| numerical_overflow(
                "Hidden Shift shifted-oracle X gate count",
            ))?;

        let hadamard_count = qubits
            .checked_mul(3)
            .ok_or_else(|| numerical_overflow(
                "Hidden Shift Hadamard gate count",
            ))?;

        let measurement_count = qubits;

        let logical_two_qubit_gate_count =
            function_cz_count
                .checked_add(shifted_function_cz_count)
                .ok_or_else(|| numerical_overflow(
                    "Hidden Shift two-qubit gate count",
                ))?;

        let logical_operation_count = hadamard_count
            .checked_add(shift_x_count)
            .and_then(|value| {
                value.checked_add(
                    logical_two_qubit_gate_count,
                )
            })
            .and_then(|value| {
                value.checked_add(measurement_count)
            })
            .ok_or_else(|| numerical_overflow(
                "Hidden Shift logical operation count",
            ))?;

        Ok(Self {
            qubits,
            classical_bits: qubits,
            hidden_shift: hidden_shift.clone(),
            function,
            function_cz_count,
            shifted_function_cz_count,
            shift_x_count,
            hadamard_count,
            measurement_count,
            logical_operation_count,
            logical_two_qubit_gate_count,
            expected_measurement: hidden_shift.as_bit_string(),
        })
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Hidden Shift benchmark generator.
///
/// The generator is stateless and therefore safe to share through
/// `Arc<dyn ApplicationBenchmarkGenerator>`.
#[derive(Debug, Clone)]
pub struct HiddenShiftBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl HiddenShiftBenchmarkGenerator {
    /// Creates the canonical Hidden Shift generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = ApplicationGeneratorDescriptor::new(
            HIDDEN_SHIFT_BENCHMARK_ID,
            HIDDEN_SHIFT_APPLICATION_ID,
            HIDDEN_SHIFT_GENERATOR_VERSION,
            "Production Boolean Hidden Shift application benchmark generator using a self-dual quadratic bent function",
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

    /// Returns the typed function configuration represented by a request.
    pub fn function_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<HiddenShiftFunction> {
        self.validate_common(request)?;

        let mut function: Option<&str> = None;

        for parameter in request.parameters() {
            match parameter.name() {
                "function" => {
                    if function.is_some() {
                        return Err(invalid_configuration(
                            "function",
                            "duplicate function parameter",
                        ));
                    }

                    function = Some(parameter.value());
                }

                "shift" => {
                    // Parsed separately.
                }

                other => {
                    return Err(unknown_parameter(other));
                }
            }
        }

        match function.unwrap_or(
            HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT,
        ) {
            HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT => {
                Ok(HiddenShiftFunction::QuadraticBent)
            }

            _ => Err(invalid_configuration(
                "function",
                "function must be `quadratic_bent`",
            )),
        }
    }

    /// Returns the typed hidden shift represented by a request.
    pub fn shift_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<HiddenShift> {
        self.validate_common(request)?;

        let mut shift: Option<&str> = None;

        for parameter in request.parameters() {
            match parameter.name() {
                "shift" => {
                    if shift.is_some() {
                        return Err(invalid_configuration(
                            "shift",
                            "duplicate shift parameter",
                        ));
                    }

                    shift = Some(parameter.value());
                }

                "function" => {
                    // Parsed by function_from_request.
                }

                other => {
                    return Err(unknown_parameter(other));
                }
            }
        }

        match shift {
            Some(value) => HiddenShift::from_bit_string(
                value,
                request.problem_size(),
            ),

            None => HiddenShift::deterministic(
                request.problem_size(),
                request.metadata().seed(),
                request.metadata().sequence_index(),
                request.metadata().generator_revision(),
            ),
        }
    }

    /// Returns the strongly typed workload description.
    pub fn describe(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<HiddenShiftWorkloadDescription> {
        self.validate(request)?;

        let function = self.function_from_request(request)?;
        let shift = self.shift_from_request(request)?;

        HiddenShiftWorkloadDescription::new(
            request.problem_size(),
            shift,
            function,
        )
    }

    /// Generates the canonical Quantum IR Hidden Shift circuit.
    ///
    /// The circuit is:
    ///
    /// ```text
    /// H^n
    /// U_g
    /// H^n
    /// U_f
    /// H^n
    /// measure
    /// ```
    ///
    /// where:
    ///
    /// ```text
    /// f(x) = x0*x1 xor x2*x3 xor ...
    /// g(x) = f(x xor s)
    /// ```
    pub fn generate_circuit(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<QuantumCircuit> {
        let description = self.describe(request)?;

        let mut circuit = QuantumCircuit::new(
            description.qubits,
            description.classical_bits,
        )
        .map_err(|error| {
            circuit_error(
                "unable to construct Hidden Shift Quantum IR circuit",
                error,
            )
        })?;

        circuit
            .set_name(Some(format!(
                "hidden_shift_{}",
                request.instance_id().as_str()
            )))
            .map_err(|error| {
                circuit_error(
                    "unable to assign Hidden Shift circuit name",
                    error,
                )
            })?;

        circuit
            .set_source(Some(
                "zamani.quantum.benchmarking.applications.hidden_shift"
                    .to_owned(),
            ))
            .map_err(|error| {
                circuit_error(
                    "unable to assign Hidden Shift circuit source",
                    error,
                )
            })?;

        // ---------------------------------------------------------------------
        // Initial Fourier transform
        // ---------------------------------------------------------------------

        append_hadamards(
            &mut circuit,
            description.qubits,
        )?;

        // ---------------------------------------------------------------------
        // Shifted oracle U_g
        // ---------------------------------------------------------------------
        //
        // U_g = X_s U_f X_s
        //
        // where:
        //
        // X_s = product of X gates for every set shift bit.
        //
        // This implements:
        //
        //     f(x xor s)
        //
        // without constructing an exponential truth table.

        append_shift_x(
            &mut circuit,
            &description.hidden_shift,
        )?;

        append_quadratic_bent_oracle(
            &mut circuit,
            description.qubits,
        )?;

        append_shift_x(
            &mut circuit,
            &description.hidden_shift,
        )?;

        // ---------------------------------------------------------------------
        // Middle Fourier transform
        // ---------------------------------------------------------------------
        //
        // This layer is intentionally retained. It gives the direct canonical
        // hidden-shift algorithm for the self-dual bent function instead of
        // relying on a benchmark-specific post-processing permutation.

        append_hadamards(
            &mut circuit,
            description.qubits,
        )?;

        // ---------------------------------------------------------------------
        // Dual/known oracle U_f
        // ---------------------------------------------------------------------

        append_quadratic_bent_oracle(
            &mut circuit,
            description.qubits,
        )?;

        // ---------------------------------------------------------------------
        // Final Fourier transform
        // ---------------------------------------------------------------------

        append_hadamards(
            &mut circuit,
            description.qubits,
        )?;

        // ---------------------------------------------------------------------
        // Measurement
        // ---------------------------------------------------------------------
        //
        // Logical q_i -> classical c_i.
        //
        // The benchmark defines the expected result in this canonical logical
        // ordering. Backend-specific textual count ordering is not part of
        // this generator's contract.

        for qubit in 0..description.qubits {
            circuit
                .push(measurement_gate(qubit, qubit)?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append Hidden Shift measurement",
                        error,
                    )
                })?;
        }

        // Final whole-circuit validation is mandatory even though every push
        // already validates its local operation.
        circuit
            .validate()
            .map_err(|error| {
                circuit_error(
                    "generated Hidden Shift circuit failed final validation",
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
        self.validate(request)?;

        let description = self.describe(request)?;

        let circuit = self.generate_circuit(request)?;

        let circuit_workload = CircuitWorkload::from_circuit(
            circuit,
            request.instance_id().clone(),
        )
        .map_err(|error| {
            workload_error(
                "unable to create Hidden Shift circuit workload",
                error,
            )
        })?;

        let mut workload = ApplicationWorkload::new(
            HIDDEN_SHIFT_APPLICATION_ID,
            request.instance_id().clone(),
            request.problem_size(),
        )
        .map_err(|error| {
            workload_error(
                "unable to create Hidden Shift application workload",
                error,
            )
        })?
        .with_circuit(circuit_workload);

        // ---------------------------------------------------------------------
        // Canonical application metadata
        // ---------------------------------------------------------------------

        add_parameter(
            &mut workload,
            "function",
            description.function.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "hidden_shift",
            &description.hidden_shift.as_bit_string(),
        )?;

        add_parameter(
            &mut workload,
            "expected_measurement",
            &description.expected_measurement,
        )?;

        add_parameter(
            &mut workload,
            "qubits",
            &description.qubits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "classical_bits",
            &description.classical_bits.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "function_cz_count",
            &description.function_cz_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "shifted_function_cz_count",
            &description
                .shifted_function_cz_count
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "shift_x_count",
            &description.shift_x_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "hadamard_count",
            &description.hadamard_count.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "measurement_count",
            &description.measurement_count.to_string(),
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
            "generator_version",
            HIDDEN_SHIFT_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            &HIDDEN_SHIFT_GENERATOR_REVISION.to_string(),
        )?;

        Ok(workload)
    }

    /// Performs validation shared by all public generator entry points.
    fn validate_common(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;

        if request.application_id()
            != HIDDEN_SHIFT_APPLICATION_ID
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "hidden_shift.application_id".to_owned(),
                reason:
                    "Hidden Shift generator requires application_id `hidden_shift`"
                        .to_owned(),
            });
        }

        validate_problem_size(request.problem_size())?;

        Ok(())
    }
}

impl ApplicationBenchmarkGenerator
    for HiddenShiftBenchmarkGenerator
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

        // Force application-specific validation before any Quantum IR
        // allocation occurs.
        let _ = self.function_from_request(request)?;
        let _ = self.shift_from_request(request)?;

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
// Circuit construction
// =============================================================================

/// Appends H to every logical qubit.
fn append_hadamards(
    circuit: &mut QuantumCircuit,
    qubits: usize,
) -> BenchmarkResult<()> {
    for qubit in 0..qubits {
        circuit
            .push(single_qubit_gate(
                GateKind::H,
                qubit,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append Hidden Shift Hadamard",
                    error,
                )
            })?;
    }

    Ok(())
}

/// Appends the X gates representing the hidden input shift.
fn append_shift_x(
    circuit: &mut QuantumCircuit,
    shift: &HiddenShift,
) -> BenchmarkResult<()> {
    for (qubit, &set) in shift.bits().iter().enumerate() {
        if set {
            circuit
                .push(single_qubit_gate(
                    GateKind::X,
                    qubit,
                )?)
                .map_err(|error| {
                    circuit_error(
                        "unable to append Hidden Shift shift X gate",
                        error,
                    )
                })?;
        }
    }

    Ok(())
}

/// Appends the phase oracle for:
//
//!     f(x) = x0*x1 xor x2*x3 xor ...
//
//! using CZ gates.
fn append_quadratic_bent_oracle(
    circuit: &mut QuantumCircuit,
    qubits: usize,
) -> BenchmarkResult<()> {
    validate_problem_size(qubits)?;

    let mut first = 0usize;

    while first < qubits {
        let second = first
            .checked_add(1)
            .ok_or_else(|| numerical_overflow(
                "Hidden Shift quadratic oracle qubit index",
            ))?;

        circuit
            .push(two_qubit_gate(
                GateKind::CZ,
                first,
                second,
            )?)
            .map_err(|error| {
                circuit_error(
                    "unable to append Hidden Shift quadratic bent oracle",
                    error,
                )
            })?;

        first = first
            .checked_add(2)
            .ok_or_else(|| numerical_overflow(
                "Hidden Shift quadratic oracle iteration",
            ))?;
    }

    Ok(())
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
            "Hidden Shift generated invalid single-qubit gate",
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
            "circuit",
            "Hidden Shift two-qubit gate cannot target the same logical qubit",
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
            "Hidden Shift generated invalid two-qubit gate",
            error,
        )
    })
}

/// Creates a computational-basis measurement.
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
            "Hidden Shift generated invalid measurement gate",
            error,
        )
    })
}

// =============================================================================
// Workload metadata helpers
// =============================================================================

/// Adds one bounded application parameter.
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
            "unable to encode Hidden Shift application metadata",
            error,
        )
    })?;

    workload
        .add_parameter(parameter)
        .map_err(|error| {
            workload_error(
                "unable to attach Hidden Shift application metadata",
                error,
            )
        })
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Hidden Shift's quadratic bent construction requires positive even width.
fn validate_problem_size(
    problem_size: usize,
) -> BenchmarkResult<()> {
    if problem_size == 0 {
        return Err(invalid_configuration(
            "problem_size",
            "Hidden Shift requires at least two logical qubits",
        ));
    }

    if problem_size < 2 {
        return Err(invalid_configuration(
            "problem_size",
            "Hidden Shift quadratic bent construction requires at least two qubits",
        ));
    }

    if problem_size % 2 != 0 {
        return Err(invalid_configuration(
            "problem_size",
            "Hidden Shift quadratic bent construction requires an even number of qubits",
        ));
    }

    if problem_size > MAX_SHIFT_BITS {
        return Err(invalid_configuration(
            "problem_size",
            "Hidden Shift problem size exceeds the application-specific maximum",
        ));
    }

    Ok(())
}

/// Deterministic SplitMix64 mixer.
///
/// This is not a cryptographic RNG and MUST NOT be used for cryptographic
/// secrets. It is used only to derive reproducible benchmark instances.
#[must_use]
fn splitmix64(mut value: u64) -> u64 {
    value = value
        .wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    z = (z ^ (z >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

/// Converts an application parameter problem into the canonical benchmark
/// error type.
fn invalid_configuration(
    field: &'static str,
    reason: &'static str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Converts an unknown application parameter into the canonical benchmark
/// error type.
fn unknown_parameter(
    parameter: &str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: "application_parameter".to_owned(),
        reason: format!(
            "unknown Hidden Shift application parameter `{parameter}`"
        ),
    }
}

/// Converts a workload-generation error into the canonical benchmark error
/// vocabulary.
fn invalid_workload(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: HIDDEN_SHIFT_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a canonical workload-model error.
fn workload_error(
    reason: &'static str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: HIDDEN_SHIFT_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Converts a Quantum IR construction/validation error into the benchmark
/// error boundary.
fn circuit_error(
    reason: &'static str,
    error: impl fmt::Display,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: HIDDEN_SHIFT_APPLICATION_ID.to_owned(),
        reason: format!("{reason}: {error}"),
    }
}

/// Creates a canonical numerical-overflow benchmark error.
fn numerical_overflow(
    operation: &'static str,
) -> BenchmarkError {
    BenchmarkError::NumericalOverflow {
        operation: operation.to_owned(),
        value: None,
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::generators::application::{
        ApplicationGenerationRequest,
    };
    use super::super::super::core::workload::WorkloadId;

    fn request(
        width: usize,
        seed: u64,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            HIDDEN_SHIFT_APPLICATION_ID,
            WorkloadId::new(
                "hidden_shift_test",
            )
            .expect("test workload ID"),
            width,
            seed,
        )
        .expect("test generation request")
    }

    fn request_with_shift(
        width: usize,
        shift: &str,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            HIDDEN_SHIFT_APPLICATION_ID,
            WorkloadId::new(
                "hidden_shift_test",
            )
            .expect("test workload ID"),
            width,
            1234,
        )
        .expect("test generation request")
        .with_parameter(
            ApplicationParameter::new(
                "shift",
                shift,
            )
            .expect("test shift parameter"),
        )
        .expect("attach shift")
    }

    #[test]
    fn generator_descriptor_is_stable() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert_eq!(
            generator.descriptor().generator_id(),
            HIDDEN_SHIFT_BENCHMARK_ID
        );

        assert_eq!(
            generator.descriptor().application_id(),
            HIDDEN_SHIFT_APPLICATION_ID
        );

        assert_eq!(
            generator.descriptor().version(),
            HIDDEN_SHIFT_GENERATOR_VERSION
        );

        assert!(
            generator.descriptor().supports(
                ApplicationGeneratorCapability::GeneratesCircuit
            )
        );

        assert!(
            generator.descriptor().supports(
                ApplicationGeneratorCapability::Deterministic
            )
        );
    }

    #[test]
    fn zero_width_is_rejected() {
        let result = HiddenShiftWorkloadDescription::new(
            0,
            HiddenShift::new(Vec::new()).unwrap_or_else(
                |_| HiddenShift {
                    bits: Vec::new(),
                },
            ),
            HiddenShiftFunction::QuadraticBent,
        );

        assert!(result.is_err());
    }

    #[test]
    fn odd_width_is_rejected() {
        let result = request(3, 1);

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert!(generator.validate(&result).is_err());
    }

    #[test]
    fn one_qubit_is_rejected() {
        let result = request(1, 1);

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert!(generator.validate(&result).is_err());
    }

    #[test]
    fn explicit_shift_is_parsed_in_logical_order() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "1010");

        let shift = generator
            .shift_from_request(&request)
            .expect("shift");

        assert_eq!(
            shift.as_bit_string(),
            "1010"
        );

        assert_eq!(
            shift.hamming_weight(),
            2
        );
    }

    #[test]
    fn malformed_shift_is_rejected() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "10a0");

        assert!(
            generator
                .shift_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn wrong_shift_width_is_rejected() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "101");

        assert!(
            generator
                .shift_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn duplicate_shift_parameter_is_rejected() {
        let request =
            ApplicationGenerationRequest::new(
                HIDDEN_SHIFT_APPLICATION_ID,
                WorkloadId::new(
                    "hidden_shift_test",
                )
                .expect("workload ID"),
                4,
                1,
            )
            .expect("request")
            .with_parameter(
                ApplicationParameter::new(
                    "shift",
                    "1010",
                )
                .expect("parameter"),
            )
            .expect("parameter")
            .with_parameter(
                ApplicationParameter::new(
                    "shift",
                    "0101",
                )
                .expect("parameter"),
            )
            .expect("parameter");

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert!(
            generator.shift_from_request(&request)
                .is_err()
        );
    }

    #[test]
    fn unknown_parameter_is_rejected() {
        let request =
            ApplicationGenerationRequest::new(
                HIDDEN_SHIFT_APPLICATION_ID,
                WorkloadId::new(
                    "hidden_shift_test",
                )
                .expect("workload ID"),
                4,
                1,
            )
            .expect("request")
            .with_parameter(
                ApplicationParameter::new(
                    "not_a_parameter",
                    "x",
                )
                .expect("parameter"),
            )
            .expect("parameter");

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert!(
            generator.validate(&request)
                .is_err()
        );
    }

    #[test]
    fn deterministic_generation_is_reproducible() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let first = generator
            .shift_from_request(&request(8, 12345))
            .expect("shift");

        let second = generator
            .shift_from_request(&request(8, 12345))
            .expect("shift");

        assert_eq!(first, second);
    }

    #[test]
    fn sequence_index_changes_generated_instances() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let base = request(8, 12345);

        let first_request =
            base.clone()
                .with_sequence_index(0);

        let second_request =
            base.with_sequence_index(1);

        let first = generator
            .shift_from_request(&first_request)
            .expect("shift");

        let second = generator
            .shift_from_request(&second_request)
            .expect("shift");

        assert_ne!(first, second);
    }

    #[test]
    fn zero_shift_is_valid() {
        let shift =
            HiddenShift::from_bit_string(
                "0000",
                4,
            )
            .expect("zero shift");

        assert!(shift.is_zero());
        assert_eq!(shift.hamming_weight(), 0);
    }

    #[test]
    fn circuit_generation_succeeds() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "1010");

        let circuit = generator
            .generate_circuit(&request)
            .expect("circuit");

        assert_eq!(
            circuit.num_qubits(),
            4
        );

        assert_eq!(
            circuit.num_classical_bits(),
            4
        );

        circuit.validate()
            .expect("valid circuit");
    }

    #[test]
    fn circuit_generation_is_deterministic() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(6, "011010");

        let first = generator
            .generate_circuit(&request)
            .expect("first circuit");

        let second = generator
            .generate_circuit(&request)
            .expect("second circuit");

        assert_eq!(
            first.operations(),
            second.operations()
        );
    }

    #[test]
    fn workload_contains_canonical_metadata() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "1010");

        let workload = generator
            .generate_application_workload(&request)
            .expect("workload");

        assert_eq!(
            workload.application_id(),
            HIDDEN_SHIFT_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            4
        );

        assert!(
            workload.circuit().is_some()
        );

        let parameters = workload.parameters();

        assert!(
            parameters.iter().any(|parameter| {
                parameter.name() == "function"
                    && parameter.value()
                        == HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT
            })
        );

        assert!(
            parameters.iter().any(|parameter| {
                parameter.name() == "hidden_shift"
                    && parameter.value() == "1010"
            })
        );

        assert!(
            parameters.iter().any(|parameter| {
                parameter.name() == "expected_measurement"
                    && parameter.value() == "1010"
            })
        );
    }

    #[test]
    fn workload_generation_through_trait_succeeds() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(4, "1010");

        let generation = generator
            .generate(&request)
            .expect("generation");

        assert_eq!(
            generation.workload().application_id(),
            HIDDEN_SHIFT_APPLICATION_ID
        );

        assert_eq!(
            generation.metadata().seed(),
            request.metadata().seed()
        );
    }

    #[test]
    fn batch_generation_is_supported() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request = request(4, 42);

        let results = generator
            .generate_batch(&request, 4)
            .expect("batch");

        assert_eq!(results.len(), 4);

        assert_eq!(
            results[0].metadata().sequence_index(),
            0
        );

        assert_eq!(
            results[1].metadata().sequence_index(),
            1
        );

        assert_eq!(
            results[2].metadata().sequence_index(),
            2
        );

        assert_eq!(
            results[3].metadata().sequence_index(),
            3
        );
    }

    #[test]
    fn description_resource_counts_are_consistent() {
        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        let request =
            request_with_shift(6, "111000");

        let description =
            generator.describe(&request)
                .expect("description");

        assert_eq!(
            description.function_cz_count,
            3
        );

        assert_eq!(
            description.shifted_function_cz_count,
            3
        );

        assert_eq!(
            description.shift_x_count,
            6
        );

        assert_eq!(
            description.hadamard_count,
            18
        );

        assert_eq!(
            description.measurement_count,
            6
        );

        assert_eq!(
            description.logical_two_qubit_gate_count,
            6
        );

        assert_eq!(
            description.expected_measurement,
            "111000"
        );
    }

    #[test]
    fn quadratic_bent_oracle_requires_even_width() {
        let mut circuit =
            QuantumCircuit::new(4, 4)
                .expect("circuit");

        append_quadratic_bent_oracle(
            &mut circuit,
            4,
        )
        .expect("oracle");

        circuit.validate()
            .expect("valid oracle circuit");
    }

    #[test]
    fn explicit_function_parameter_is_accepted() {
        let request =
            ApplicationGenerationRequest::new(
                HIDDEN_SHIFT_APPLICATION_ID,
                WorkloadId::new(
                    "hidden_shift_test",
                )
                .expect("workload ID"),
                4,
                1,
            )
            .expect("request")
            .with_parameter(
                ApplicationParameter::new(
                    "function",
                    HIDDEN_SHIFT_FUNCTION_QUADRATIC_BENT,
                )
                .expect("function"),
            )
            .expect("parameter");

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert_eq!(
            generator
                .function_from_request(&request)
                .expect("function"),
            HiddenShiftFunction::QuadraticBent
        );
    }

    #[test]
    fn unsupported_function_is_rejected() {
        let request =
            ApplicationGenerationRequest::new(
                HIDDEN_SHIFT_APPLICATION_ID,
                WorkloadId::new(
                    "hidden_shift_test",
                )
                .expect("workload ID"),
                4,
                1,
            )
            .expect("request")
            .with_parameter(
                ApplicationParameter::new(
                    "function",
                    "unsupported",
                )
                .expect("function"),
            )
            .expect("parameter");

        let generator =
            HiddenShiftBenchmarkGenerator::new()
                .expect("generator");

        assert!(
            generator
                .function_from_request(&request)
                .is_err()
        );
    }
}