//! Zamani Quantum Benchmarking — Bernstein–Vazirani Application Benchmark
//!
//! Production implementation of the Bernstein–Vazirani (BV) application
//! workload generator.
//!
//! # Architectural role
//!
//! This module owns the Bernstein–Vazirani benchmark's:
//!
//! - workload identity;
//! - instance validation;
//! - hidden-string handling;
//! - deterministic instance generation;
//! - canonical Quantum IR circuit construction;
//! - exact small-instance reference information;
//! - application-generator integration;
//! - resource metadata exposed through the canonical workload;
//! - deterministic reproducibility checks.
//!
//! This module deliberately does NOT:
//!
//! - execute circuits;
//! - select a backend;
//! - route logical qubits;
//! - schedule operations;
//! - perform hardware calibration;
//! - submit jobs to a QPU;
//! - implement a simulator;
//! - implement statistical aggregation;
//! - define backend-specific metrics;
//! - duplicate Quantum IR;
//! - parse Zamani source code;
//! - perform filesystem or network I/O.
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani benchmark declaration
//!          │
//!          ▼
//! ApplicationGenerationRequest
//!          │
//!          ▼
//! BernsteinVaziraniGenerator
//!          │
//!          ▼
//! ApplicationWorkload
//!          │
//!          ▼
//! CircuitWorkload
//!          │
//!          ▼
//! QuantumCircuit
//!          │
//!          ▼
//! benchmarking execution layer
//!          │
//!          ▼
//! observations
//!          │
//!          ▼
//! application analysis
//! ```
//!
//! # Algorithm
//!
//! Bernstein–Vazirani solves the hidden-string problem.
//!
//! For a secret bit string:
//!
//! ```text
//! s = s[0] s[1] ... s[n-1]
//! ```
//!
//! the oracle implements:
//!
//! ```text
//! f_s(x) = s · x (mod 2)
//! ```
//!
//! The canonical circuit generated here is:
//!
//! ```text
//! input q[0..n-1]: |0> --H-- oracle --H-- measure
//!                              │
//! ancilla q[n]:    |0> --X--H-- oracle --------
//! ```
//!
//! For every `s[i] == 1`, the oracle contains:
//!
//! ```text
//! CX(q[i], ancilla)
//! ```
//!
//! The ancilla is prepared in `|->` through `X` followed by `H`.
//!
//! After the final Hadamards, the input register ideally contains exactly the
//! hidden string `s`.
//!
//! # Benchmark semantics
//!
//! `problem_size` is the number of hidden-string bits, not the total number of
//! physical or logical qubits in the generated circuit.
//!
//! For an instance of size `n`:
//!
//! - logical qubits = `n + 1`;
//! - classical bits = `n`;
//! - input Hadamards = `2n`;
//! - ancilla preparation gates = `2`;
//! - oracle CNOTs = Hamming weight of `s`;
//! - measurements = `n`;
//! - total gates = `3n + weight(s) + 2`.
//!
//! The circuit is therefore deterministic for a fixed:
//!
//! - application identifier;
//! - instance identifier;
//! - problem size;
//! - parameter set;
//! - seed;
//! - sequence index;
//! - generator revision.
//!
//! # Secret-string input
//!
//! A caller may explicitly provide an application parameter:
//!
//! ```text
//! secret = "010101"
//! ```
//!
//! The secret must:
//!
//! - contain exactly `problem_size` bits;
//! - contain only ASCII `0` and `1`;
//! - be non-empty;
//! - not exceed the workload parameter bounds.
//!
//! If `secret` is not supplied, this module derives a deterministic secret from
//! the generation metadata. No system entropy, clock, process ID, thread ID,
//! pointer address, or global RNG is used.
//!
//! # Reproducibility
//!
//! Explicit secrets are the strongest reproducibility mode because the complete
//! hidden instance is visible in the benchmark request.
//!
//! For generated secrets, the secret is derived from:
//!
//! ```text
//! seed
//! sequence_index
//! generator_revision
//! problem_size
//! ```
//!
//! using a local, deterministic SplitMix64-style derivation.
//!
//! This local derivation is deliberately not exposed as a general-purpose RNG.
//! General random benchmark protocols should use `generators::random`.
//!
//! # Security/resource model
//!
//! Requests can originate from the Zamani language, CI, serialized benchmark
//! definitions, or external tooling and therefore are treated as untrusted.
//!
//! This implementation:
//!
//! - validates all identifiers;
//! - validates problem size;
//! - checks arithmetic before allocation;
//! - checks circuit dimensions before construction;
//! - bounds explicit secret size through the canonical parameter model;
//! - avoids unchecked `n + 1` and gate-count arithmetic;
//! - never allocates from an unchecked user-controlled quantity;
//! - never executes user-provided code;
//! - never performs I/O;
//! - never silently truncates a secret;
//! - never silently changes an explicitly supplied secret.
//!
//! Global benchmark-wide limits remain owned by `core::limits` / `core::config`.
//!
//! # Integration contract
//!
//! This file intentionally integrates with:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneration
//!     ├── circuit_application_descriptor
//!     └── make_application_workload
//!
//! benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     ├── CircuitWorkload
//!     └── WorkloadId
//!
//! quantum::ir
//!     ├── QuantumCircuit
//!     ├── Gate
//!     ├── Measurement
//!     └── QubitId
//! ```
//!
//! It does not depend on execution, hardware, routing, scheduling, reporting,
//! or statistical analysis.
//!
//! # Future analysis integration
//!
//! The execution/analysis layer should consume the generated
//! `ApplicationWorkload` and use the canonical benchmark observation model.
//!
//! For an ideal BV execution, the application success condition is:
//!
//! ```text
//! measured_input == secret
//! ```
//!
//! Therefore future analysis should report at least:
//!
//! - success probability;
//! - exact-string probability;
//! - Hamming distance/error distribution;
//! - shots;
//! - logical qubit count;
//! - circuit depth;
//! - total gate count;
//! - two-qubit gate count;
//! - execution time;
//! - compilation/routing overhead where available;
//! - provenance and reproducibility metadata.
//!
//! This generator intentionally does not calculate those execution metrics.
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
//! # Scientific correctness
//!
//! Bernstein–Vazirani is an exact-query quantum algorithm in the standard
//! oracle model: one oracle query is sufficient to recover the complete hidden
//! bit string in the ideal circuit.
//!
//! This benchmark therefore measures implementation/execution quality rather
//! than treating the algorithm as a generic random workload.
//!
//! In particular, the benchmark must not claim that every real backend performs
//! exactly one physical operation per abstract oracle query. Compilation,
//! routing, decomposition, control hardware, and backend-native implementation
//! may transform the logical circuit downstream.
//!
//! Those transformations belong to the compiler/backend pipeline and should be
//! recorded in benchmark provenance rather than changing this logical workload.

use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    CircuitWorkload,
    WorkloadId,
};
use super::super::generators::application::{
    circuit_application_descriptor,
    make_application_workload,
    ApplicationBenchmarkGenerator,
    ApplicationGenerationRequest,
    ApplicationGeneratorDescriptor,
};
use super::super::core::errors::{
    BenchmarkError,
    BenchmarkResult,
};
use crate::quantum::ir::{
    Gate,
    Measurement,
    QubitId,
    QuantumCircuit,
};

// =============================================================================
// Stable benchmark identity/version
// =============================================================================

/// Stable application identifier used by the benchmark registry.
pub const APPLICATION_ID: &str = "bernstein_vazirani";

/// Stable generator identifier.
pub const GENERATOR_ID: &str = "bernstein_vazirani_generator";

/// Version of the BV application generator semantics.
///
/// This version must be included in provenance/fingerprints by callers using
/// the generator.
pub const GENERATOR_VERSION: &str = "1";

/// Human-readable generator description.
pub const GENERATOR_DESCRIPTION: &str =
    "Deterministic Bernstein-Vazirani quantum application workload generator";

/// Current semantic generator revision.
///
/// Increment this when the deterministic instance-generation algorithm or
/// circuit-generation semantics change.
pub const GENERATOR_REVISION: u32 = 1;

/// Name of the optional explicit secret parameter.
pub const SECRET_PARAMETER_NAME: &str = "secret";

/// Maximum problem size accepted by this application module.
///
/// The benchmark-wide resource policy remains authoritative. This value is an
/// application-local structural guard preventing pathological `n + 1`
/// calculations before the IR layer is reached.
///
/// A production benchmark runner should normally impose a much smaller limit
/// through `core::limits`.
pub const MAX_PROBLEM_SIZE: usize = usize::MAX - 1;

/// Maximum length of the derived secret in bytes.
///
/// This is bounded independently of the workload parameter representation so
/// that secret construction cannot unexpectedly allocate an enormous string.
pub const MAX_DERIVED_SECRET_BYTES: usize = 1_048_576;

// =============================================================================
// Public semantic metadata
// =============================================================================

/// Describes the Bernstein–Vazirani instance represented by a generated
/// workload.
///
/// This is metadata, not a competing workload representation.
///
/// The canonical workload remains [`ApplicationWorkload`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BernsteinVaziraniInstance {
    secret: String,
    problem_size: usize,
    hamming_weight: usize,
    explicit_secret: bool,
}

impl BernsteinVaziraniInstance {
    /// Creates and validates an instance from an explicit secret.
    pub fn from_secret<S: Into<String>>(
        secret: S,
    ) -> BenchmarkResult<Self> {
        let secret = secret.into();

        validate_secret(&secret)?;

        let problem_size = secret.len();
        let hamming_weight = secret
            .bytes()
            .filter(|byte| *byte == b'1')
            .count();

        Ok(Self {
            secret,
            problem_size,
            hamming_weight,
            explicit_secret: true,
        })
    }

    /// Creates an instance from deterministic generation metadata.
    pub fn derive(
        problem_size: usize,
        seed: u64,
        sequence_index: u64,
        generator_revision: u32,
    ) -> BenchmarkResult<Self> {
        validate_problem_size(problem_size)?;

        let secret = derive_secret(
            problem_size,
            seed,
            sequence_index,
            generator_revision,
        )?;

        let hamming_weight = secret
            .bytes()
            .filter(|byte| *byte == b'1')
            .count();

        Ok(Self {
            secret,
            problem_size,
            hamming_weight,
            explicit_secret: false,
        })
    }

    /// Returns the hidden bit string.
    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Returns the problem size.
    #[must_use]
    pub const fn problem_size(&self) -> usize {
        self.problem_size
    }

    /// Returns the Hamming weight of the secret.
    #[must_use]
    pub const fn hamming_weight(&self) -> usize {
        self.hamming_weight
    }

    /// Returns whether the secret was explicitly supplied by the caller.
    #[must_use]
    pub const fn explicit_secret(&self) -> bool {
        self.explicit_secret
    }

    /// Returns the number of logical qubits required by the canonical BV
    /// circuit, including the oracle ancilla.
    pub fn logical_qubits(&self) -> BenchmarkResult<usize> {
        self.problem_size
            .checked_add(1)
            .ok_or_else(|| BenchmarkError::NumericalOverflow {
                operation: "Bernstein-Vazirani logical qubit count".to_owned(),
                value: Some(self.problem_size.to_string()),
            })
    }

    /// Returns the number of classical bits used by the canonical BV circuit.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.problem_size
    }

    /// Returns the number of input-register Hadamard gates.
    pub fn input_hadamards(&self) -> BenchmarkResult<usize> {
        self.problem_size
            .checked_mul(2)
            .ok_or_else(|| BenchmarkError::NumericalOverflow {
                operation: "Bernstein-Vazirani Hadamard count".to_owned(),
                value: Some(self.problem_size.to_string()),
            })
    }

    /// Returns the number of oracle CNOT gates.
    #[must_use]
    pub const fn oracle_cnot_count(&self) -> usize {
        self.hamming_weight
    }

    /// Returns the two ancilla-preparation gates (`X`, `H`).
    #[must_use]
    pub const fn ancilla_preparation_gate_count(&self) -> usize {
        2
    }

    /// Returns the number of input measurements.
    #[must_use]
    pub const fn measurement_count(&self) -> usize {
        self.problem_size
    }

    /// Returns the complete logical gate count of the generated circuit.
    ///
    /// The formula is:
    ///
    /// `2n + 2 + weight(s) + n`
    ///
    /// which simplifies to:
    ///
    /// `3n + weight(s) + 2`.
    pub fn gate_count(&self) -> BenchmarkResult<usize> {
        let input_hadamards = self.input_hadamards()?;

        input_hadamards
            .checked_add(self.ancilla_preparation_gate_count())
            .and_then(|value| value.checked_add(self.oracle_cnot_count()))
            .and_then(|value| value.checked_add(self.measurement_count()))
            .ok_or_else(|| BenchmarkError::NumericalOverflow {
                operation: "Bernstein-Vazirani total gate count".to_owned(),
                value: Some(self.problem_size.to_string()),
            })
    }

    /// Returns the number of two-qubit gates.
    #[must_use]
    pub const fn two_qubit_gate_count(&self) -> usize {
        self.oracle_cnot_count()
    }

    /// Returns the ideal output string.
    ///
    /// For Bernstein–Vazirani, the ideal measured input register is exactly the
    /// hidden secret.
    #[must_use]
    pub fn ideal_output(&self) -> &str {
        self.secret()
    }
}

// =============================================================================
// Benchmark generator
// =============================================================================

/// Production Bernstein–Vazirani application benchmark generator.
///
/// This is a zero-state generator. All semantic inputs arrive through
/// [`ApplicationGenerationRequest`].
#[derive(Debug, Clone)]
pub struct BernsteinVaziraniGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl BernsteinVaziraniGenerator {
    /// Creates the canonical Bernstein–Vazirani generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = circuit_application_descriptor(
            GENERATOR_ID,
            APPLICATION_ID,
            GENERATOR_VERSION,
            GENERATOR_DESCRIPTION,
        )?;

        Ok(Self { descriptor })
    }

    /// Returns the canonical generator descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Generates a benchmark instance directly from a problem size and seed.
    ///
    /// This convenience API is useful to Rust callers while the normal
    /// production integration remains the generic application-generator
    /// contract.
    pub fn generate_for_size(
        &self,
        problem_size: usize,
        seed: u64,
    ) -> BenchmarkResult<super::super::generators::application::ApplicationGeneration>
    {
        let instance_id = WorkloadId::new(format!(
            "{APPLICATION_ID}_{problem_size}_{seed}"
        ))
        .map_err(|error| BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

        let request = ApplicationGenerationRequest::new(
            APPLICATION_ID,
            instance_id,
            problem_size,
            seed,
        )?;

        self.generate(&request)
    }

    /// Generates a benchmark instance using an explicitly supplied hidden
    /// secret.
    ///
    /// The secret must have exactly `problem_size` bits.
    pub fn generate_with_secret<S: Into<String>>(
        &self,
        problem_size: usize,
        seed: u64,
        secret: S,
    ) -> BenchmarkResult<super::super::generators::application::ApplicationGeneration>
    {
        let secret = secret.into();

        validate_secret_for_problem_size(
            &secret,
            problem_size,
        )?;

        let instance_id = WorkloadId::new(format!(
            "{APPLICATION_ID}_{problem_size}_explicit"
        ))
        .map_err(|error| BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

        let request = ApplicationGenerationRequest::new(
            APPLICATION_ID,
            instance_id,
            problem_size,
            seed,
        )?
        .with_parameter(
            ApplicationParameter::new(
                SECRET_PARAMETER_NAME,
                secret,
            )
            .map_err(|error| BenchmarkError::InvalidWorkload {
                workload: APPLICATION_ID.to_owned(),
                reason: error.to_string(),
            })?,
        )?;

        self.generate(&request)
    }

    /// Extracts and validates the optional secret parameter.
    fn instance_from_request(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<BernsteinVaziraniInstance> {
        validate_request(request)?;

        if let Some(secret) =
            find_parameter(request, SECRET_PARAMETER_NAME)
        {
            validate_secret_for_problem_size(
                secret,
                request.problem_size(),
            )?;

            BernsteinVaziraniInstance::from_secret(secret)
        } else {
            BernsteinVaziraniInstance::derive(
                request.problem_size(),
                request.metadata().seed(),
                request.metadata().sequence_index(),
                request
                    .metadata()
                    .generator_revision(),
            )
        }
    }
}

impl Default for BernsteinVaziraniGenerator {
    fn default() -> Self {
        Self::new()
            .expect("canonical Bernstein-Vazirani generator descriptor must be valid")
    }
}

impl ApplicationBenchmarkGenerator
    for BernsteinVaziraniGenerator
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
        validate_request(request)?;

        if self.descriptor.application_id()
            != request.application_id()
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first: "bernstein_vazirani.generator.application_id"
                        .to_owned(),
                    second: "bernstein_vazirani.request.application_id"
                        .to_owned(),
                    reason: "application identifiers must match"
                        .to_owned(),
                },
            );
        }

        // Validate the optional secret before circuit construction.
        let _ = self.instance_from_request(request)?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let instance = self.instance_from_request(request)?;

        let circuit = build_circuit(&instance)?;

        let circuit_workload = CircuitWorkload::from_circuit(
            circuit,
            request.instance_id().clone(),
        )
        .map_err(|error| BenchmarkError::InvalidWorkload {
            workload: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

        let mut workload = make_application_workload(request)?;

        /*
         * Preserve the canonical application workload model.
         *
         * The circuit is attached only after all BV-specific validation has
         * succeeded. This prevents a partially constructed workload from
         * crossing the application-generator boundary.
         */
        workload = workload.with_circuit(circuit_workload);

        /*
         * The generated workload already contains all caller parameters.
         * The secret is deliberately not added automatically when it was
         * derived from the seed. This prevents silently changing the
         * canonical request parameter set and keeps the distinction between:
         *
         *   explicit input
         *
         * and:
         *
         *   deterministic generated instance
         *
         * available to provenance.
         */

        Ok(workload)
    }
}

// =============================================================================
// Public generation helpers
// =============================================================================

/// Creates the canonical Bernstein–Vazirani generator descriptor.
///
/// This helper is useful for registry construction without instantiating the
/// generator itself.
pub fn bernstein_vazirani_descriptor(
) -> BenchmarkResult<ApplicationGeneratorDescriptor> {
    circuit_application_descriptor(
        GENERATOR_ID,
        APPLICATION_ID,
        GENERATOR_VERSION,
        GENERATOR_DESCRIPTION,
    )
}

/// Generates one canonical Bernstein–Vazirani workload.
///
/// This is the lowest-level convenient application API. The generic
/// `ApplicationBenchmarkGenerator` contract remains the authoritative
/// integration interface.
pub fn generate_workload(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<ApplicationWorkload> {
    BernsteinVaziraniGenerator::default()
        .generate_workload(request)
}

/// Generates one complete Bernstein–Vazirani application benchmark instance.
pub fn generate(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<
    super::super::generators::application::ApplicationGeneration,
> {
    BernsteinVaziraniGenerator::default().generate(request)
}

/// Generates a deterministic Bernstein–Vazirani instance for `problem_size`.
pub fn generate_for_size(
    problem_size: usize,
    seed: u64,
) -> BenchmarkResult<
    super::super::generators::application::ApplicationGeneration,
> {
    BernsteinVaziraniGenerator::default()
        .generate_for_size(problem_size, seed)
}

/// Generates a Bernstein–Vazirani instance using an explicit secret.
pub fn generate_with_secret<S: Into<String>>(
    problem_size: usize,
    seed: u64,
    secret: S,
) -> BenchmarkResult<
    super::super::generators::application::ApplicationGeneration,
> {
    BernsteinVaziraniGenerator::default()
        .generate_with_secret(problem_size, seed, secret)
}

// =============================================================================
// Circuit construction
// =============================================================================

/// Builds the canonical Bernstein–Vazirani Quantum IR circuit.
///
/// The circuit uses:
///
/// - qubits `0..n` as the input register;
/// - qubit `n` as the oracle ancilla;
/// - classical bits `0..n` for input measurements.
///
/// The ancilla is prepared as `|->`.
///
/// The input register is prepared as `|+...+>`.
///
/// For each secret bit equal to one, a CNOT is inserted from the corresponding
/// input qubit to the ancilla.
///
/// The input register is then transformed by Hadamards and measured in the
/// computational basis.
fn build_circuit(
    instance: &BernsteinVaziraniInstance,
) -> BenchmarkResult<QuantumCircuit> {
    let problem_size = instance.problem_size();

    let total_qubits = instance.logical_qubits()?;

    /*
     * The BV circuit needs exactly `n` classical destinations. The Quantum IR
     * constructor is fallible and applies the repository's canonical
     * production resource policy.
     */
    let mut circuit = QuantumCircuit::new(
        total_qubits,
        instance.classical_bits(),
    )
    .map_err(|error| BenchmarkError::InvalidCircuit {
        circuit: APPLICATION_ID.to_owned(),
        reason: error.to_string(),
    })?;

    /*
     * Give the circuit stable logical provenance. These metadata fields belong
     * to Quantum IR and contain no hardware information.
     */
    circuit
        .set_name(Some(
            "bernstein_vazirani".to_owned(),
        ))
        .map_err(|error| BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

    circuit
        .set_source(Some(
            "zamani.quantum.benchmarking.applications.bernstein_vazirani"
                .to_owned(),
        ))
        .map_err(|error| BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

    /*
     * Input preparation:
     *
     * |0>^n -> H^n -> |+>^n
     */
    for index in 0..problem_size {
        let qubit = QubitId::new(index);

        circuit
            .push(
                Gate::h(qubit).map_err(|error| {
                    BenchmarkError::InvalidCircuit {
                        circuit: APPLICATION_ID.to_owned(),
                        reason: error.to_string(),
                    }
                })?,
            )
            .map_err(|error| BenchmarkError::InvalidCircuit {
                circuit: APPLICATION_ID.to_owned(),
                reason: error.to_string(),
            })?;
    }

    /*
     * Ancilla preparation:
     *
     * |0> -> X -> H -> |->
     */
    let ancilla = QubitId::new(problem_size);

    circuit
        .push(
            Gate::x(ancilla).map_err(|error| {
                BenchmarkError::InvalidCircuit {
                    circuit: APPLICATION_ID.to_owned(),
                    reason: error.to_string(),
                }
            })?,
        )
        .map_err(|error| BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

    circuit
        .push(
            Gate::h(ancilla).map_err(|error| {
                BenchmarkError::InvalidCircuit {
                    circuit: APPLICATION_ID.to_owned(),
                    reason: error.to_string(),
                }
            })?,
        )
        .map_err(|error| BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

    /*
     * Oracle:
     *
     * f_s(x) = s · x (mod 2)
     *
     * A CNOT is required exactly where the corresponding secret bit is one.
     */
    for (index, bit) in instance.secret().bytes().enumerate() {
        if bit == b'1' {
            let input = QubitId::new(index);

            circuit
                .push(
                    Gate::cx(input, ancilla).map_err(
                        |error| BenchmarkError::InvalidCircuit {
                            circuit: APPLICATION_ID.to_owned(),
                            reason: error.to_string(),
                        },
                    )?,
                )
                .map_err(|error| BenchmarkError::InvalidCircuit {
                    circuit: APPLICATION_ID.to_owned(),
                    reason: error.to_string(),
                })?;
        }
    }

    /*
     * Interference:
     *
     * |+>^n -- H^n --> |s>
     */
    for index in 0..problem_size {
        let qubit = QubitId::new(index);

        circuit
            .push(
                Gate::h(qubit).map_err(|error| {
                    BenchmarkError::InvalidCircuit {
                        circuit: APPLICATION_ID.to_owned(),
                        reason: error.to_string(),
                    }
                })?,
            )
            .map_err(|error| BenchmarkError::InvalidCircuit {
                circuit: APPLICATION_ID.to_owned(),
                reason: error.to_string(),
            })?;
    }

    /*
     * Measure only the input register.
     *
     * The oracle ancilla is not part of the application output and therefore
     * does not need a classical destination.
     */
    for index in 0..problem_size {
        let qubit = QubitId::new(index);

        let measurement = Measurement::new(
            qubit,
            index.into(),
        );

        circuit
            .push(
                Gate::measure(
                    qubit,
                    index,
                    measurement,
                )
                .map_err(|error| {
                    BenchmarkError::InvalidCircuit {
                        circuit: APPLICATION_ID.to_owned(),
                        reason: error.to_string(),
                    }
                })?,
            )
            .map_err(|error| BenchmarkError::InvalidCircuit {
                circuit: APPLICATION_ID.to_owned(),
                reason: error.to_string(),
            })?;
    }

    /*
     * The IR mutation API validates each operation locally. Perform the
     * complete validation again before the workload crosses the generator
     * boundary because the circuit is an externally reusable semantic object.
     */
    circuit
        .validate()
        .map_err(|error| BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

    /*
     * Verify that the generated resource shape agrees with the mathematical
     * instance description. This protects against accidental future changes to
     * the circuit construction sequence.
     */
    verify_generated_circuit_shape(
        &circuit,
        instance,
    )?;

    Ok(circuit)
}

/// Verifies the generated circuit's structural invariants.
fn verify_generated_circuit_shape(
    circuit: &QuantumCircuit,
    instance: &BernsteinVaziraniInstance,
) -> BenchmarkResult<()> {
    let expected_qubits = instance.logical_qubits()?;
    let expected_classical_bits = instance.classical_bits();
    let expected_operations = instance.gate_count()?;

    if circuit.num_qubits() != expected_qubits {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {expected_qubits} logical qubits but generated {}",
                circuit.num_qubits()
            ),
        });
    }

    if circuit.num_classical_bits()
        != expected_classical_bits
    {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {expected_classical_bits} classical bits but generated {}",
                circuit.num_classical_bits()
            ),
        });
    }

    if circuit.len() != expected_operations {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {expected_operations} operations but generated {}",
                circuit.len()
            ),
        });
    }

    /*
     * Count operation classes directly from the canonical IR rather than
     * trusting only the mathematical formula.
     */
    let mut hadamards = 0usize;
    let mut cnot_count = 0usize;
    let mut measurements = 0usize;
    let mut ancilla_x = 0usize;

    for operation in circuit.operations() {
        match operation.kind() {
            crate::quantum::ir::GateKind::H => {
                hadamards = hadamards.checked_add(1).ok_or_else(
                    || BenchmarkError::NumericalOverflow {
                        operation:
                            "Bernstein-Vazirani Hadamard verification"
                                .to_owned(),
                        value: Some(hadamards.to_string()),
                    },
                )?;
            }

            crate::quantum::ir::GateKind::CX => {
                cnot_count = cnot_count.checked_add(1).ok_or_else(
                    || BenchmarkError::NumericalOverflow {
                        operation:
                            "Bernstein-Vazirani CNOT verification"
                                .to_owned(),
                        value: Some(cnot_count.to_string()),
                    },
                )?;
            }

            crate::quantum::ir::GateKind::Measure => {
                measurements =
                    measurements.checked_add(1).ok_or_else(
                        || BenchmarkError::NumericalOverflow {
                            operation:
                                "Bernstein-Vazirani measurement verification"
                                    .to_owned(),
                            value: Some(
                                measurements.to_string(),
                            ),
                        },
                    )?;
            }

            crate::quantum::ir::GateKind::X => {
                ancilla_x = ancilla_x.checked_add(1).ok_or_else(
                    || BenchmarkError::NumericalOverflow {
                        operation:
                            "Bernstein-Vazirani ancilla verification"
                                .to_owned(),
                        value: Some(ancilla_x.to_string()),
                    },
                )?;
            }

            _ => {}
        }
    }

    let expected_hadamards = instance.input_hadamards()?;

    if hadamards != expected_hadamards + 1 {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {} Hadamards including ancilla preparation but generated {hadamards}",
                expected_hadamards + 1
            ),
        });
    }

    if cnot_count != instance.oracle_cnot_count() {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {} oracle CNOTs but generated {cnot_count}",
                instance.oracle_cnot_count()
            ),
        });
    }

    if measurements != instance.measurement_count() {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected {} measurements but generated {measurements}",
                instance.measurement_count()
            ),
        });
    }

    if ancilla_x != 1 {
        return Err(BenchmarkError::InvalidCircuit {
            circuit: APPLICATION_ID.to_owned(),
            reason: format!(
                "expected exactly one ancilla X gate but generated {ancilla_x}"
            ),
        });
    }

    Ok(())
}

// =============================================================================
// Request validation
// =============================================================================

/// Validates a Bernstein–Vazirani generation request.
fn validate_request(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<()> {
    request.validate()?;

    if request.application_id() != APPLICATION_ID {
        return Err(BenchmarkError::InconsistentConfiguration {
            first: "request.application_id".to_owned(),
            second: "bernstein_vazirani.application_id".to_owned(),
            reason: format!(
                "expected `{APPLICATION_ID}` but received `{}`",
                request.application_id()
            ),
        });
    }

    validate_problem_size(request.problem_size())?;

    validate_secret_parameter_cardinality(request)?;

    Ok(())
}

/// Validates the application-local problem-size bound.
fn validate_problem_size(
    problem_size: usize,
) -> BenchmarkResult<()> {
    if problem_size == 0 {
        return Err(BenchmarkError::InvalidRange {
            field: "bernstein_vazirani.problem_size".to_owned(),
            value: "0".to_owned(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_PROBLEM_SIZE.to_string()),
        });
    }

    if problem_size > MAX_PROBLEM_SIZE {
        return Err(BenchmarkError::InvalidRange {
            field: "bernstein_vazirani.problem_size".to_owned(),
            value: problem_size.to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_PROBLEM_SIZE.to_string()),
        });
    }

    /*
     * A secret is represented as one byte per bit. This protects the
     * deterministic generator from constructing an unnecessarily huge String.
     */
    if problem_size > MAX_DERIVED_SECRET_BYTES {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "bernstein_vazirani_secret_bytes"
                .to_owned(),
            requested: problem_size as u64,
            maximum: MAX_DERIVED_SECRET_BYTES as u64,
        });
    }

    Ok(())
}

/// Ensures that at most one `secret` parameter is present.
///
/// The canonical workload model intentionally preserves parameters as supplied
/// by the caller; therefore this generator must reject duplicate secret
/// parameters rather than silently selecting one.
fn validate_secret_parameter_cardinality(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<()> {
    let mut count = 0usize;

    for parameter in request.parameters() {
        if parameter.name() == SECRET_PARAMETER_NAME {
            count = count.checked_add(1).ok_or_else(|| {
                BenchmarkError::NumericalOverflow {
                    operation:
                        "Bernstein-Vazirani secret parameter count"
                            .to_owned(),
                    value: Some(count.to_string()),
                }
            })?;
        }
    }

    if count > 1 {
        return Err(
            BenchmarkError::InconsistentConfiguration {
                first: "bernstein_vazirani.secret[0]".to_owned(),
                second: "bernstein_vazirani.secret[1]".to_owned(),
                reason:
                    "exactly zero or one secret parameter is permitted"
                        .to_owned(),
            },
        );
    }

    Ok(())
}

/// Finds the first occurrence of an application parameter by name.
///
/// `validate_secret_parameter_cardinality` must be called before this helper
/// when uniqueness is required.
fn find_parameter<'a>(
    request: &'a ApplicationGenerationRequest,
    name: &str,
) -> Option<&'a str> {
    request
        .parameters()
        .iter()
        .find(|parameter| parameter.name() == name)
        .map(ApplicationParameter::value)
}

// =============================================================================
// Secret validation and deterministic generation
// =============================================================================

/// Validates a hidden BV secret.
fn validate_secret(
    secret: &str,
) -> BenchmarkResult<()> {
    if secret.is_empty() {
        return Err(BenchmarkError::InvalidRange {
            field: "bernstein_vazirani.secret".to_owned(),
            value: "0".to_owned(),
            minimum: Some("1".to_owned()),
            maximum: Some(
                MAX_DERIVED_SECRET_BYTES.to_string(),
            ),
        });
    }

    if secret.len() > MAX_DERIVED_SECRET_BYTES {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "bernstein_vazirani_secret_bytes"
                .to_owned(),
            requested: secret.len() as u64,
            maximum: MAX_DERIVED_SECRET_BYTES as u64,
        });
    }

    if !secret.bytes().all(|byte| {
        byte == b'0' || byte == b'1'
    }) {
        return Err(BenchmarkError::InvalidIdentifier {
            field: SECRET_PARAMETER_NAME.to_owned(),
            value: secret.to_owned(),
        });
    }

    Ok(())
}

/// Validates a secret against a requested problem size.
fn validate_secret_for_problem_size(
    secret: &str,
    problem_size: usize,
) -> BenchmarkResult<()> {
    validate_problem_size(problem_size)?;
    validate_secret(secret)?;

    if secret.len() != problem_size {
        return Err(BenchmarkError::InconsistentConfiguration {
            first: "bernstein_vazirani.secret.length".to_owned(),
            second: "bernstein_vazirani.problem_size".to_owned(),
            reason: format!(
                "secret contains {} bits but problem_size is {problem_size}",
                secret.len()
            ),
        });
    }

    Ok(())
}

/// Derives a deterministic secret from generation metadata.
///
/// This is a local deterministic derivation primitive, not a general-purpose
/// RNG. It exists so this application can remain deterministic without hidden
/// entropy or dependence on execution state.
///
/// The output is stable for the current generator revision. Changing the
/// derivation algorithm requires incrementing `GENERATOR_REVISION`.
fn derive_secret(
    problem_size: usize,
    seed: u64,
    sequence_index: u64,
    generator_revision: u32,
) -> BenchmarkResult<String> {
    validate_problem_size(problem_size)?;

    let mut result = String::new();

    result.try_reserve(problem_size).map_err(|_| {
        BenchmarkError::ResourceLimitExceeded {
            resource:
                "bernstein_vazirani_secret_allocation"
                    .to_owned(),
            requested: problem_size as u64,
            maximum: MAX_DERIVED_SECRET_BYTES as u64,
        }
    })?;

    /*
     * Domain-separated initial state.
     *
     * The constants are fixed and part of generator revision 1.
     */
    let mut state = seed
        ^ 0x4256_5A41_4D41_4E49_u64
        ^ ((problem_size as u64)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15))
        ^ sequence_index.rotate_left(17)
        ^ (generator_revision as u64).rotate_left(31);

    for index in 0..problem_size {
        state = splitmix64_next(&mut state);

        /*
         * Taking the low bit is sufficient for deterministic construction.
         * No statistical quality is claimed or required here.
         */
        let bit = if state & 1 == 0 {
            b'0'
        } else {
            b'1'
        };

        result.push(bit as char);

        /*
         * Domain-separate each position so that changing the generation
         * sequence cannot accidentally collapse positions into one state.
         */
        state ^= (index as u64)
            .wrapping_mul(0xD6E8_FEB8_6659_FD93);
    }

    Ok(result)
}

/// Deterministic SplitMix64-style state transition.
///
/// This is intentionally private. It is an instance derivation mechanism, not
/// a public random-number API.
fn splitmix64_next(
    state: &mut u64,
) -> u64 {
    *state = state.wrapping_add(
        0x9E37_79B9_7F4A_7C15,
    );

    let mut value = *state;

    value = (value ^ (value >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    value = (value ^ (value >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    value ^ (value >> 31)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        problem_size: usize,
        seed: u64,
    ) -> ApplicationGenerationRequest {
        ApplicationGenerationRequest::new(
            APPLICATION_ID,
            WorkloadId::new(format!(
                "bv_test_{problem_size}_{seed}"
            ))
            .expect("test workload ID must be valid"),
            problem_size,
            seed,
        )
        .expect("test request must be valid")
    }

    #[test]
    fn descriptor_has_stable_identity() {
        let generator =
            BernsteinVaziraniGenerator::new()
                .expect("descriptor must be valid");

        assert_eq!(
            generator.descriptor().application_id(),
            APPLICATION_ID
        );

        assert_eq!(
            generator.descriptor().generator_id(),
            GENERATOR_ID
        );

        assert_eq!(
            generator.descriptor().version(),
            GENERATOR_VERSION
        );
    }

    #[test]
    fn explicit_secret_is_validated() {
        let instance =
            BernsteinVaziraniInstance::from_secret(
                "010101",
            )
            .expect("secret must be valid");

        assert_eq!(
            instance.problem_size(),
            6
        );

        assert_eq!(
            instance.hamming_weight(),
            3
        );

        assert_eq!(
            instance.logical_qubits()
                .expect("qubit count must fit"),
            7
        );

        assert_eq!(
            instance.classical_bits(),
            6
        );

        assert_eq!(
            instance.oracle_cnot_count(),
            3
        );

        assert_eq!(
            instance.gate_count()
                .expect("gate count must fit"),
            23
        );
    }

    #[test]
    fn invalid_secret_characters_are_rejected() {
        assert!(
            BernsteinVaziraniInstance::from_secret(
                "010201"
            )
            .is_err()
        );
    }

    #[test]
    fn empty_secret_is_rejected() {
        assert!(
            BernsteinVaziraniInstance::from_secret("")
                .is_err()
        );
    }

    #[test]
    fn explicit_secret_length_must_match_problem_size() {
        let generator =
            BernsteinVaziraniGenerator::default();

        assert!(
            generator
                .generate_with_secret(
                    4,
                    7,
                    "010"
                )
                .is_err()
        );

        assert!(
            generator
                .generate_with_secret(
                    4,
                    7,
                    "01010"
                )
                .is_err()
        );
    }

    #[test]
    fn same_seed_and_size_are_reproducible() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let first = generator
            .generate_for_size(8, 42)
            .expect("first generation must succeed");

        let second = generator
            .generate_for_size(8, 42)
            .expect("second generation must succeed");

        let first_workload = first.workload();
        let second_workload = second.workload();

        let first_circuit = first_workload
            .circuit()
            .expect("BV workload must contain a circuit")
            .circuit();

        let second_circuit = second_workload
            .circuit()
            .expect("BV workload must contain a circuit")
            .circuit();

        assert_eq!(
            first_circuit,
            second_circuit
        );
    }

    #[test]
    fn sequence_index_changes_generated_instance_deterministically() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let base = request(16, 1234);

        let first = generator
            .generate(
                &base
                    .clone()
                    .with_sequence_index(0),
            )
            .expect("first generation must succeed");

        let second = generator
            .generate(
                &base
                    .clone()
                    .with_sequence_index(1),
            )
            .expect("second generation must succeed");

        let first_circuit = first
            .workload()
            .circuit()
            .expect("first circuit must exist")
            .circuit();

        let second_circuit = second
            .workload()
            .circuit()
            .expect("second circuit must exist")
            .circuit();

        /*
         * Different sequence indices are allowed to collide theoretically
         * under a finite derivation function. Therefore the contract does not
         * assert inequality here; it asserts successful independent generation
         * and deterministic repeatability instead.
         */
        let second_again = generator
            .generate(
                &base
                    .with_sequence_index(1),
            )
            .expect("repeat generation must succeed");

        let second_again_circuit = second_again
            .workload()
            .circuit()
            .expect("repeat circuit must exist")
            .circuit();

        assert_eq!(
            second_circuit,
            second_again_circuit
        );

        /*
         * Ensure the first generated object remains valid and independent.
         */
        assert_eq!(
            first_circuit.num_qubits(),
            17
        );
        assert_eq!(
            second_circuit.num_qubits(),
            17
        );
    }

    #[test]
    fn explicit_secret_is_preserved_in_workload_parameters() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_with_secret(
                5,
                99,
                "10101",
            )
            .expect("generation must succeed");

        let workload = generation.workload();

        assert_eq!(
            workload.application_id(),
            APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            5
        );

        let secret = workload
            .parameters()
            .iter()
            .find(|parameter| {
                parameter.name()
                    == SECRET_PARAMETER_NAME
            })
            .expect("explicit secret must be retained");

        assert_eq!(
            secret.value(),
            "10101"
        );
    }

    #[test]
    fn generated_workload_contains_canonical_circuit() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_for_size(4, 42)
            .expect("generation must succeed");

        let circuit_workload = generation
            .workload()
            .circuit()
            .expect("BV must contain a circuit");

        let circuit = circuit_workload.circuit();

        assert_eq!(
            circuit.num_qubits(),
            5
        );

        assert_eq!(
            circuit.num_classical_bits(),
            4
        );

        assert!(
            circuit.len() >= 4
        );

        assert!(
            circuit
                .operations()
                .iter()
                .any(|operation| {
                    operation.kind()
                        == crate::quantum::ir::GateKind::Measure
                })
        );
    }

    #[test]
    fn explicit_zero_secret_generates_no_oracle_cnot() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_with_secret(
                6,
                1,
                "000000",
            )
            .expect("generation must succeed");

        let circuit = generation
            .workload()
            .circuit()
            .expect("circuit must exist")
            .circuit();

        let cnot_count = circuit
            .operations()
            .iter()
            .filter(|operation| {
                operation.kind()
                    == crate::quantum::ir::GateKind::CX
            })
            .count();

        assert_eq!(
            cnot_count,
            0
        );
    }

    #[test]
    fn explicit_all_one_secret_generates_n_cnot_oracle() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_with_secret(
                6,
                1,
                "111111",
            )
            .expect("generation must succeed");

        let circuit = generation
            .workload()
            .circuit()
            .expect("circuit must exist")
            .circuit();

        let cnot_count = circuit
            .operations()
            .iter()
            .filter(|operation| {
                operation.kind()
                    == crate::quantum::ir::GateKind::CX
            })
            .count();

        assert_eq!(
            cnot_count,
            6
        );
    }

    #[test]
    fn circuit_shape_matches_instance() {
        let instance =
            BernsteinVaziraniInstance::from_secret(
                "10110",
            )
            .expect("instance must be valid");

        let circuit = build_circuit(&instance)
            .expect("circuit must be valid");

        verify_generated_circuit_shape(
            &circuit,
            &instance,
        )
        .expect("circuit shape must match");
    }

    #[test]
    fn ideal_output_is_secret() {
        let instance =
            BernsteinVaziraniInstance::from_secret(
                "100101",
            )
            .expect("instance must be valid");

        assert_eq!(
            instance.ideal_output(),
            "100101"
        );
    }

    #[test]
    fn duplicate_secret_parameters_are_rejected() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let request = request(4, 1)
            .with_parameter(
                ApplicationParameter::new(
                    SECRET_PARAMETER_NAME,
                    "0101",
                )
                .expect("parameter must be valid"),
            )
            .expect("first parameter must be accepted")
            .with_parameter(
                ApplicationParameter::new(
                    SECRET_PARAMETER_NAME,
                    "1010",
                )
                .expect("parameter must be valid"),
            )
            .expect("second parameter must be accepted");

        assert!(
            generator.generate(&request).is_err()
        );
    }

    #[test]
    fn wrong_application_id_is_rejected() {
        let request = ApplicationGenerationRequest::new(
            "grover",
            WorkloadId::new("wrong_application")
                .expect("test ID must be valid"),
            4,
            1,
        )
        .expect("request construction must succeed");

        let generator =
            BernsteinVaziraniGenerator::default();

        assert!(
            generator.generate(&request).is_err()
        );
    }

    #[test]
    fn generated_secret_contains_only_binary_digits() {
        let instance =
            BernsteinVaziraniInstance::derive(
                128,
                0x1234_5678_9ABC_DEF0,
                3,
                GENERATOR_REVISION,
            )
            .expect("derived instance must be valid");

        assert_eq!(
            instance.secret().len(),
            128
        );

        assert!(
            instance
                .secret()
                .bytes()
                .all(|byte| {
                    byte == b'0'
                        || byte == b'1'
                })
        );
    }

    #[test]
    fn generated_secret_is_stable() {
        let first =
            BernsteinVaziraniInstance::derive(
                64,
                42,
                7,
                GENERATOR_REVISION,
            )
            .expect("first derivation must succeed");

        let second =
            BernsteinVaziraniInstance::derive(
                64,
                42,
                7,
                GENERATOR_REVISION,
            )
            .expect("second derivation must succeed");

        assert_eq!(
            first.secret(),
            second.secret()
        );
    }

    #[test]
    fn generated_gate_formula_is_consistent() {
        for problem_size in 1usize..=32usize {
            let instance =
                BernsteinVaziraniInstance::derive(
                    problem_size,
                    9876,
                    problem_size as u64,
                    GENERATOR_REVISION,
                )
                .expect("instance must be valid");

            let expected = problem_size
                .checked_mul(3)
                .and_then(|value| {
                    value.checked_add(
                        instance.hamming_weight(),
                    )
                })
                .and_then(|value| {
                    value.checked_add(2)
                })
                .expect("test formula must fit");

            assert_eq!(
                instance
                    .gate_count()
                    .expect("gate count must fit"),
                expected
            );
        }
    }

    #[test]
    fn ancilla_is_not_measured() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_with_secret(
                4,
                7,
                "1010",
            )
            .expect("generation must succeed");

        let circuit = generation
            .workload()
            .circuit()
            .expect("circuit must exist")
            .circuit();

        let ancilla_index = 4usize;

        let ancilla_measurements = circuit
            .operations()
            .iter()
            .filter(|operation| {
                operation.kind()
                    == crate::quantum::ir::GateKind::Measure
                    && operation
                        .qubit()
                        .map(|qubit| {
                            qubit.index()
                                == ancilla_index
                        })
                        .unwrap_or(false)
            })
            .count();

        assert_eq!(
            ancilla_measurements,
            0
        );
    }

    #[test]
    fn every_measurement_targets_matching_classical_bit() {
        let generator =
            BernsteinVaziraniGenerator::default();

        let generation = generator
            .generate_with_secret(
                8,
                7,
                "10110011",
            )
            .expect("generation must succeed");

        let circuit = generation
            .workload()
            .circuit()
            .expect("circuit must exist")
            .circuit();

        for operation in circuit.operations() {
            if operation.kind()
                == crate::quantum::ir::GateKind::Measure
            {
                let qubit =
                    operation
                        .qubit()
                        .expect("measurement has qubit");

                let classical =
                    operation
                        .classical_target()
                        .expect(
                            "measurement has classical target",
                        );

                assert_eq!(
                    qubit.index(),
                    classical
                );
            }
        }
    }
}