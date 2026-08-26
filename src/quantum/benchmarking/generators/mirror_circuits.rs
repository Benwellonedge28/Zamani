//! Zamani Quantum Benchmarking — Mirror Circuit Generator
//!
//! Production-ready generation of logical mirror circuits for quantum
//! benchmarking and characterization.
//!
//! # Purpose
//!
//! A mirror circuit executes a circuit U followed by its exact logical
//! inverse U†:
//
//!     |ψ⟩ ── U ── U† ── M
//!
//! For an ideal noiseless backend, the complete circuit is therefore the
//! identity operation (up to global phase):
//!
//!     U† U = I
//!
//! Mirror circuits are useful for diagnosing:
//!
//! - gate errors;
//! - coherent errors;
//! - compilation errors;
//! - routing errors;
//! - depth-dependent degradation;
//! - correlated errors;
//! - calibration drift;
//! - backend/compiler regressions;
//! - circuit inversion correctness;
//! - end-to-end circuit fidelity.
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - mirror-circuit configuration;
//! - deterministic random generation;
//! - logical circuit-layer construction;
//! - operation validation;
//! - exact inversion of generated logical operations;
//! - mirror-circuit metadata;
//! - generator versioning;
//! - resource-limit checks local to generation.
//!
//! This module does NOT own:
//!
//! - Quantum IR construction;
//! - OpenQASM parsing;
//! - Zamani-language parsing;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - native-gate decomposition;
//! - backend selection;
//! - backend execution;
//! - calibration;
//! - statistical fitting;
//! - fidelity estimation;
//! - reporting.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Integration
//!
//! The intended architecture is:
//!
//! ```text
//! generators::random
//!        │
//!        ▼
//! generators::mirror_circuits
//!        │
//!        ├──► protocols::mirror
//!        │
//!        ├──► tests / reproducibility
//!        │
//!        ▼
//! core::circuit adapter
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ▼
//! optimization / routing / scheduling
//!        │
//!        ▼
//! runtime / hardware
//! ```
//!
//! The generator deliberately does not depend on `core::circuit` or
//! `quantum::ir`. This makes the generator independently testable and avoids
//! creating a circular dependency between generation and the canonical IR.
//!
//! A future adapter may translate [`MirrorCircuit`] into
//! `core::circuit::BenchmarkCircuit` / `quantum::ir::QuantumCircuit` without
//! requiring this generator to change.
//!
//! # Existing Clifford integration
//!
//! The existing `generators::clifford` module defines the logical primitive
//! Clifford operations:
//!
//! - `H`
//! - `S`
//! - `Sdg`
//!
//! This module reuses [`CliffordPrimitive`] rather than defining another
//! incompatible Clifford primitive vocabulary.
//!
//! The two-qubit operation used here is logical CX. It is intentionally not a
//! physical pulse-level instruction. Backend-specific decomposition remains
//! outside this file.
//!
//! # Scientific semantics
//!
//! A generated mirror experiment has the structure:
//!
//! ```text
//! forward layers
//!        │
//!        ▼
//!       U
//!        │
//!        ▼
//! inverse layers
//!        │
//!        ▼
//!       U†
//! ```
//!
//! The inverse is constructed from the generated forward circuit itself.
//! Therefore the generator never attempts to "guess" an inverse from a
//! circuit description after the fact.
//!
//! If the forward circuit is:
//!
//! ```text
//! L0 → L1 → L2
//! ```
//!
//! then the inverse is:
//!
//! ```text
//! L2† → L1† → L0†
//! ```
//!
//! Every operation is independently inverted:
//!
//! ```text
//! H   → H
//! S   → Sdg
//! Sdg → S
//! CX  → CX
//! ```
//!
//! Because operations within a generated layer act on disjoint qubits, their
//! order within a layer does not affect the logical inverse.
//!
//! # Randomness
//!
//! All randomized generation requires an explicit [`RngCore`].
//!
//! No global RNG is used.
//!
//! The generator uses rejection sampling for bounded random integers. It does
//! not use `random % bound`, which would introduce modulo bias unless the
//! source range is exactly divisible by the requested bound.
//!
//! Reproducibility is therefore controlled by the caller's RNG and complete
//! benchmark configuration.
//!
//! # Resource safety
//!
//! The generator validates:
//!
//! - non-zero qubit count;
//! - configured maximum qubit count;
//! - non-zero depth where required;
//! - maximum depth;
//! - maximum operations;
//! - arithmetic overflow;
//! - valid two-qubit operands;
//! - layer operation capacity;
//! - probability configuration.
//!
//! These checks are local safety checks. The authoritative experiment-wide
//! limits remain owned by `benchmarking::core::limits`.
//!
//! # Important distinction
//!
//! This generator does NOT claim that the generated circuit is uniformly
//! sampled from the full n-qubit Clifford group.
//!
//! It generates random logical Clifford layers according to the configured
//! sampling policy and then constructs their exact inverse.
//!
//! This distinction is important for scientific correctness. A mirror circuit
//! benchmark is not automatically a randomized-benchmarking experiment and
//! must not be interpreted as one.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Serialization
//!
//! This module deliberately does not require `serde` directly. The surrounding
//! benchmarking result/configuration layer can serialize generated benchmark
//! definitions through its canonical schema.
//!
//! This avoids creating a second serialization contract inside a generator.
//!
//! # Versioning
//!
//! Changing the logical generation semantics requires incrementing
//! [`MIRROR_GENERATOR_VERSION`]. The version is part of the reproducibility
//! contract and should be included in benchmark provenance by the protocol
//! layer.
//!
//! ```text
//! configuration
//! + RNG seed
//! + generator version
//! + protocol version
//!     │
//!     ▼
//! deterministic experiment identity
//! ```
//!
//! # No execution
//!
//! This file never:
//!
//! - submits a circuit;
//! - contacts a QPU;
//! - invokes a simulator;
//! - waits for hardware;
//! - reads calibration;
//! - performs routing;
//! - mutates a backend.
//!
//! It only generates the logical experiment.

use super::clifford::CliffordPrimitive;
use rand::RngCore;
use std::fmt;

/// Stable semantic version of the mirror-circuit generator.
///
/// Increment this whenever changing:
///
/// - operation sampling semantics;
/// - layer generation semantics;
/// - inversion semantics;
/// - operation vocabulary;
/// - resource-accounting semantics;
///
/// Changes that only improve documentation or internal implementation without
/// changing generated experiment semantics do not require a version change.
pub const MIRROR_GENERATOR_VERSION: u16 = 1;

/// Stable identifier for the generator.
pub const MIRROR_GENERATOR_ID: &str = "zamani.quantum.benchmarking.mirror";

/// Default maximum number of qubits accepted by this generator.
///
/// This is a generator-local safety boundary, not a physical limitation.
pub const DEFAULT_MAX_QUBITS: usize = 4096;

/// Default maximum forward depth.
///
/// The complete mirror circuit has twice the forward depth.
pub const DEFAULT_MAX_FORWARD_DEPTH: usize = 1_000_000;

/// Default maximum number of operations in the complete mirror circuit.
pub const DEFAULT_MAX_TOTAL_OPERATIONS: usize = 10_000_000;

/// Default probability of attempting a two-qubit operation in a layer when
/// two-qubit operations are enabled.
pub const DEFAULT_TWO_QUBIT_PROBABILITY: f64 = 0.5;

/// Default probability that an eligible qubit participates in a two-qubit
/// operation after the layer decides to create one.
///
/// This is intentionally separate from [`DEFAULT_TWO_QUBIT_PROBABILITY`].
pub const DEFAULT_TWO_QUBIT_PAIR_PROBABILITY: f64 = 1.0;

/// Logical operation used by a mirror circuit.
///
/// These operations are intentionally logical rather than hardware-native.
///
/// The later lowering/compilation pipeline decides how these operations map to
/// the target backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MirrorOperation {
    /// A single-qubit Clifford primitive.
    SingleQubit {
        /// Logical target qubit.
        qubit: usize,

        /// Clifford primitive.
        primitive: CliffordPrimitive,
    },

    /// Logical controlled-X / CNOT operation.
    ///
    /// CX is self-inverse:
    ///
    ///     CX† = CX
    ///
    /// `control` and `target` must be distinct.
    Cx {
        /// Logical control qubit.
        control: usize,

        /// Logical target qubit.
        target: usize,
    },
}

impl MirrorOperation {
    /// Returns the number of qubits occupied by this operation.
    pub const fn arity(self) -> usize {
        match self {
            Self::SingleQubit { .. } => 1,
            Self::Cx { .. } => 2,
        }
    }

    /// Returns all logical qubits occupied by the operation.
    ///
    /// The returned array has a fixed maximum size of two. The second entry
    /// is `None` for a single-qubit operation.
    pub const fn qubits(self) -> ([usize; 2], usize) {
        match self {
            Self::SingleQubit { qubit, .. } => ([qubit, 0], 1),
            Self::Cx { control, target } => ([control, target], 2),
        }
    }

    /// Returns whether this operation is self-inverse.
    pub const fn is_self_inverse(self) -> bool {
        match self {
            Self::SingleQubit { primitive, .. } => primitive.is_self_inverse(),
            Self::Cx { .. } => true,
        }
    }

    /// Returns the exact logical inverse of this operation.
    pub const fn inverse(self) -> Self {
        match self {
            Self::SingleQubit { qubit, primitive } => Self::SingleQubit {
                qubit,
                primitive: primitive.inverse(),
            },

            Self::Cx { control, target } => Self::Cx { control, target },
        }
    }
}

/// A layer of logically parallel mirror operations.
///
/// Operations in one layer are guaranteed to act on pairwise-disjoint qubits.
///
/// The generator does not impose a physical scheduling policy. The layer is a
/// logical parallelism boundary that later scheduling may preserve or expand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorLayer {
    operations: Vec<MirrorOperation>,
}

impl MirrorLayer {
    /// Creates an empty layer.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Creates a layer from operations after validating that no qubit is used
    /// more than once.
    pub fn from_operations(
        operations: Vec<MirrorOperation>,
        qubit_count: usize,
    ) -> Result<Self, MirrorCircuitError> {
        validate_layer_operations(&operations, qubit_count)?;
        Ok(Self { operations })
    }

    /// Returns the operations in this layer.
    pub fn operations(&self) -> &[MirrorOperation] {
        &self.operations
    }

    /// Returns the number of operations in this layer.
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the layer contains no operations.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the exact inverse layer.
    ///
    /// Because all operations in a valid layer are disjoint, reversing the
    /// order within the layer is not required for logical correctness.
    /// We nevertheless reverse it to make the inverse operation formally
    /// correct for any future layer implementation that may carry ordering
    /// semantics.
    pub fn inverse(&self) -> Self {
        let operations = self
            .operations
            .iter()
            .rev()
            .map(|operation| operation.inverse())
            .collect();

        Self { operations }
    }
}

impl Default for MirrorLayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete logical mirror circuit.
///
/// The complete experiment consists of:
///
/// ```text
/// forward()
///     followed by
/// inverse()
/// ```
///
/// The forward circuit is stored separately so that its identity can be
/// hashed/fingerprinted by the protocol layer and so that tests can verify
/// inversion independently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorCircuit {
    qubit_count: usize,
    forward_layers: Vec<MirrorLayer>,
    inverse_layers: Vec<MirrorLayer>,
    generator_version: u16,
}

impl MirrorCircuit {
    /// Constructs a validated mirror circuit from forward layers.
    ///
    /// The inverse is derived immediately from the supplied forward circuit.
    pub fn from_forward_layers(
        qubit_count: usize,
        forward_layers: Vec<MirrorLayer>,
    ) -> Result<Self, MirrorCircuitError> {
        validate_qubit_count(qubit_count, DEFAULT_MAX_QUBITS)?;

        let forward_depth = forward_layers.len();

        if forward_depth > DEFAULT_MAX_FORWARD_DEPTH {
            return Err(MirrorCircuitError::DepthExceeded {
                requested: forward_depth,
                maximum: DEFAULT_MAX_FORWARD_DEPTH,
            });
        }

        let forward_operations = count_layer_operations(&forward_layers)?;

        let doubled_operations = forward_operations
            .checked_mul(2)
            .ok_or(MirrorCircuitError::SizeOverflow)?;

        if doubled_operations > DEFAULT_MAX_TOTAL_OPERATIONS {
            return Err(MirrorCircuitError::OperationLimitExceeded {
                requested: doubled_operations,
                maximum: DEFAULT_MAX_TOTAL_OPERATIONS,
            });
        }

        for layer in &forward_layers {
            validate_layer_operations(&layer.operations, qubit_count)?;
        }

        let inverse_layers = forward_layers
            .iter()
            .rev()
            .map(MirrorLayer::inverse)
            .collect::<Vec<_>>();

        let circuit = Self {
            qubit_count,
            forward_layers,
            inverse_layers,
            generator_version: MIRROR_GENERATOR_VERSION,
        };

        circuit.validate_exact_inverse()?;

        Ok(circuit)
    }

    /// Returns the number of logical qubits.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the forward circuit depth.
    pub fn forward_depth(&self) -> usize {
        self.forward_layers.len()
    }

    /// Returns the complete mirror depth.
    ///
    /// No measurement or backend scheduling overhead is included.
    pub fn total_depth(&self) -> usize {
        self.forward_depth().saturating_mul(2)
    }

    /// Returns the number of operations in U.
    pub fn forward_operation_count(&self) -> usize {
        self.forward_layers
            .iter()
            .map(MirrorLayer::operation_count)
            .sum()
    }

    /// Returns the number of operations in U†.
    pub fn inverse_operation_count(&self) -> usize {
        self.inverse_layers
            .iter()
            .map(MirrorLayer::operation_count)
            .sum()
    }

    /// Returns the total number of logical operations.
    pub fn total_operation_count(&self) -> usize {
        self.forward_operation_count()
            .saturating_add(self.inverse_operation_count())
    }

    /// Returns the number of single-qubit operations in U.
    pub fn forward_single_qubit_operation_count(&self) -> usize {
        self.forward_layers
            .iter()
            .flat_map(|layer| layer.operations.iter())
            .filter(|operation| {
                matches!(operation, MirrorOperation::SingleQubit { .. })
            })
            .count()
    }

    /// Returns the number of CX operations in U.
    pub fn forward_cx_count(&self) -> usize {
        self.forward_layers
            .iter()
            .flat_map(|layer| layer.operations.iter())
            .filter(|operation| matches!(operation, MirrorOperation::Cx { .. }))
            .count()
    }

    /// Returns the forward layers.
    pub fn forward_layers(&self) -> &[MirrorLayer] {
        &self.forward_layers
    }

    /// Returns the inverse layers.
    pub fn inverse_layers(&self) -> &[MirrorLayer] {
        &self.inverse_layers
    }

    /// Returns the generator semantic version.
    pub const fn generator_version(&self) -> u16 {
        self.generator_version
    }

    /// Returns the generator identifier.
    pub const fn generator_id(&self) -> &'static str {
        MIRROR_GENERATOR_ID
    }

    /// Returns all operations in execution order.
    ///
    /// This produces:
    ///
    /// ```text
    /// U || U†
    /// ```
    ///
    /// without modifying the stored circuit.
    pub fn operations(&self) -> Vec<MirrorOperation> {
        let capacity = self.total_operation_count();
        let mut operations = Vec::with_capacity(capacity);

        for layer in &self.forward_layers {
            operations.extend_from_slice(&layer.operations);
        }

        for layer in &self.inverse_layers {
            operations.extend_from_slice(&layer.operations);
        }

        operations
    }

    /// Returns all layers in execution order.
    ///
    /// This produces the forward layers followed by inverse layers.
    pub fn layers(&self) -> Vec<&MirrorLayer> {
        let mut layers = Vec::with_capacity(self.forward_depth().saturating_mul(2));

        layers.extend(self.forward_layers.iter());
        layers.extend(self.inverse_layers.iter());

        layers
    }

    /// Validates that the stored inverse is exactly the inverse of the forward
    /// circuit.
    ///
    /// This is a structural check. It does not simulate the circuit.
    pub fn validate_exact_inverse(&self) -> Result<(), MirrorCircuitError> {
        if self.forward_layers.len() != self.inverse_layers.len() {
            return Err(MirrorCircuitError::InvalidInverse);
        }

        for (forward, inverse) in self
            .forward_layers
            .iter()
            .zip(self.inverse_layers.iter().rev())
        {
            if inverse.operations.len() != forward.operations.len() {
                return Err(MirrorCircuitError::InvalidInverse);
            }

            for (operation, inverse_operation) in
                forward.operations.iter().zip(inverse.operations.iter().rev())
            {
                if operation.inverse() != *inverse_operation {
                    return Err(MirrorCircuitError::InvalidInverse);
                }
            }
        }

        Ok(())
    }

    /// Returns whether the circuit contains no forward operations.
    pub fn is_identity(&self) -> bool {
        self.forward_operation_count() == 0
    }
}

/// Configuration for random mirror-circuit generation.
///
/// The configuration is intentionally independent of benchmark execution.
/// A protocol such as `protocols::mirror` can add shots, backend identity,
/// statistical settings and quality criteria in its own configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MirrorCircuitConfig {
    /// Number of logical qubits.
    pub qubit_count: usize,

    /// Number of forward layers.
    pub forward_depth: usize,

    /// Probability of selecting a layer containing a two-qubit operation.
    ///
    /// This probability is applied only when at least two qubits are
    /// available.
    pub two_qubit_probability: f64,

    /// Maximum qubit count accepted by this configuration.
    ///
    /// The global benchmark limit should normally be stricter or equal.
    pub max_qubits: usize,

    /// Maximum forward depth accepted by this configuration.
    pub max_forward_depth: usize,

    /// Maximum number of operations in the complete mirror circuit.
    pub max_total_operations: usize,
}

impl Default for MirrorCircuitConfig {
    fn default() -> Self {
        Self {
            qubit_count: 1,
            forward_depth: 1,
            two_qubit_probability: DEFAULT_TWO_QUBIT_PROBABILITY,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_forward_depth: DEFAULT_MAX_FORWARD_DEPTH,
            max_total_operations: DEFAULT_MAX_TOTAL_OPERATIONS,
        }
    }
}

impl MirrorCircuitConfig {
    /// Validates this configuration without consuming or using an RNG.
    pub fn validate(&self) -> Result<(), MirrorCircuitError> {
        if self.qubit_count == 0 {
            return Err(MirrorCircuitError::InvalidQubitCount);
        }

        if self.max_qubits == 0 {
            return Err(MirrorCircuitError::InvalidConfiguration {
                reason: "max_qubits must be greater than zero",
            });
        }

        if self.qubit_count > self.max_qubits {
            return Err(MirrorCircuitError::QubitLimitExceeded {
                requested: self.qubit_count,
                maximum: self.max_qubits,
            });
        }

        if self.forward_depth > self.max_forward_depth {
            return Err(MirrorCircuitError::DepthExceeded {
                requested: self.forward_depth,
                maximum: self.max_forward_depth,
            });
        }

        if self.max_total_operations == 0 && self.forward_depth > 0 {
            return Err(MirrorCircuitError::InvalidConfiguration {
                reason: "max_total_operations must be greater than zero for a non-empty circuit",
            });
        }

        if !self.two_qubit_probability.is_finite()
            || !(0.0..=1.0).contains(&self.two_qubit_probability)
        {
            return Err(MirrorCircuitError::InvalidProbability {
                probability: self.two_qubit_probability,
            });
        }

        Ok(())
    }
}

/// Errors produced by mirror-circuit generation and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum MirrorCircuitError {
    /// Zero qubits were requested.
    InvalidQubitCount,

    /// The requested number of qubits exceeds the configured limit.
    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// The requested depth exceeds the configured limit.
    DepthExceeded {
        requested: usize,
        maximum: usize,
    },

    /// The requested operation count exceeds the configured limit.
    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// An arithmetic calculation overflowed.
    SizeOverflow,

    /// A qubit index is outside the circuit width.
    InvalidQubitIndex {
        qubit: usize,
        qubit_count: usize,
    },

    /// A two-qubit operation targets the same qubit twice.
    DuplicateQubit {
        qubit: usize,
    },

    /// A layer contains overlapping operations.
    OverlappingLayerOperations {
        qubit: usize,
    },

    /// A probability is NaN, infinite, below zero, or above one.
    InvalidProbability {
        probability: f64,
    },

    /// A configuration contains an invalid parameter.
    InvalidConfiguration {
        reason: &'static str,
    },

    /// The generated inverse is structurally inconsistent with U.
    InvalidInverse,

    /// A requested random selection had an impossible bound.
    InvalidRandomBound,

    /// The operation set could not generate a valid requested layer.
    GenerationFailure {
        reason: &'static str,
    },
}

impl fmt::Display for MirrorCircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount => {
                write!(f, "mirror circuits require at least one qubit")
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "mirror circuit qubit count {requested} exceeds maximum {maximum}"
                )
            }

            Self::DepthExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "mirror circuit forward depth {requested} exceeds maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "mirror circuit operation count {requested} exceeds maximum {maximum}"
                )
            }

            Self::SizeOverflow => {
                write!(f, "mirror circuit size calculation overflowed")
            }

            Self::InvalidQubitIndex {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "qubit index {qubit} is outside circuit width {qubit_count}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "two-qubit operation cannot target qubit {qubit} twice"
                )
            }

            Self::OverlappingLayerOperations { qubit } => {
                write!(
                    f,
                    "layer contains multiple operations using qubit {qubit}"
                )
            }

            Self::InvalidProbability { probability } => {
                write!(
                    f,
                    "invalid two-qubit probability {probability}; expected a finite value in [0, 1]"
                )
            }

            Self::InvalidConfiguration { reason } => {
                write!(f, "invalid mirror-circuit configuration: {reason}")
            }

            Self::InvalidInverse => {
                write!(f, "mirror inverse does not exactly invert the forward circuit")
            }

            Self::InvalidRandomBound => {
                write!(f, "random selection bound is invalid")
            }

            Self::GenerationFailure { reason } => {
                write!(f, "mirror-circuit generation failed: {reason}")
            }
        }
    }
}

impl std::error::Error for MirrorCircuitError {}

/// Generates a random mirror circuit using an explicit random source.
///
/// The generated forward circuit contains logical Clifford primitives and,
/// when enabled by the configured probability, logical CX operations.
///
/// The returned circuit always contains the exact inverse of the generated
/// forward circuit.
///
/// # Reproducibility
///
/// For reproducible experiments, provide a deterministic RNG seeded from the
/// benchmark's canonical reproducibility seed. The RNG implementation itself
/// is part of the caller's reproducibility contract.
pub fn generate_mirror_circuit<R: RngCore + ?Sized>(
    rng: &mut R,
    config: MirrorCircuitConfig,
) -> Result<MirrorCircuit, MirrorCircuitError> {
    config.validate()?;

    let mut forward_layers = Vec::with_capacity(config.forward_depth);

    for _ in 0..config.forward_depth {
        let layer = generate_random_layer(
            rng,
            config.qubit_count,
            config.two_qubit_probability,
        )?;

        let current_operations = count_layer_operations(&forward_layers)?;
        let next_operations = current_operations
            .checked_add(layer.operation_count())
            .ok_or(MirrorCircuitError::SizeOverflow)?;

        let doubled = next_operations
            .checked_mul(2)
            .ok_or(MirrorCircuitError::SizeOverflow)?;

        if doubled > config.max_total_operations {
            return Err(MirrorCircuitError::OperationLimitExceeded {
                requested: doubled,
                maximum: config.max_total_operations,
            });
        }

        forward_layers.push(layer);
    }

    let circuit = MirrorCircuit::from_forward_layers(
        config.qubit_count,
        forward_layers,
    )?;

    if circuit.total_operation_count() > config.max_total_operations {
        return Err(MirrorCircuitError::OperationLimitExceeded {
            requested: circuit.total_operation_count(),
            maximum: config.max_total_operations,
        });
    }

    Ok(circuit)
}

/// Generates a deterministic mirror circuit from an explicit list of forward
/// layers.
///
/// This is useful for:
///
/// - golden tests;
/// - regression fixtures;
/// - reproducibility tests;
/// - protocol unit tests;
/// - externally generated logical workloads.
///
/// No random source is required.
pub fn mirror_from_layers(
    qubit_count: usize,
    forward_layers: Vec<MirrorLayer>,
) -> Result<MirrorCircuit, MirrorCircuitError> {
    MirrorCircuit::from_forward_layers(qubit_count, forward_layers)
}

/// Generates a deterministic mirror circuit from a flat forward operation
/// sequence.
///
/// Each operation becomes its own logical layer.
///
/// This function intentionally does not perform scheduling or parallelization.
pub fn mirror_from_operations(
    qubit_count: usize,
    operations: Vec<MirrorOperation>,
) -> Result<MirrorCircuit, MirrorCircuitError> {
    let mut layers = Vec::with_capacity(operations.len());

    for operation in operations {
        layers.push(MirrorLayer::from_operations(
            vec![operation],
            qubit_count,
        )?);
    }

    mirror_from_layers(qubit_count, layers)
}

/// Creates a one-qubit identity mirror circuit.
///
/// This is useful as a baseline fixture and for validating protocol behavior
/// when the forward circuit is empty.
pub fn identity_mirror_circuit(
    qubit_count: usize,
) -> Result<MirrorCircuit, MirrorCircuitError> {
    validate_qubit_count(qubit_count, DEFAULT_MAX_QUBITS)?;

    MirrorCircuit::from_forward_layers(qubit_count, Vec::new())
}

/// Generates one random logical Clifford primitive.
///
/// The distribution is uniform over:
///
/// - H
/// - S
/// - Sdg
///
/// This is a logical primitive distribution and is not a claim of uniform
/// sampling over the full Clifford group.
fn random_clifford_primitive<R: RngCore + ?Sized>(
    rng: &mut R,
) -> CliffordPrimitive {
    match uniform_index(rng, 3) {
        0 => CliffordPrimitive::H,
        1 => CliffordPrimitive::S,
        _ => CliffordPrimitive::Sdg,
    }
}

/// Generates one random valid logical layer.
///
/// The layer is constructed so that operations never overlap.
///
/// If a two-qubit operation is selected, two distinct qubits are selected and
/// a CX operation is inserted.
///
/// All remaining qubits receive independent single-qubit Clifford primitives.
///
/// Consequently every layer contains either:
///
/// - only single-qubit operations; or
/// - exactly one CX plus single-qubit operations on the remaining qubits.
///
/// This conservative structure is deliberate. It provides a clear
/// backend-independent parallelism model while avoiding ambiguous scheduling
/// semantics.
fn generate_random_layer<R: RngCore + ?Sized>(
    rng: &mut R,
    qubit_count: usize,
    two_qubit_probability: f64,
) -> Result<MirrorLayer, MirrorCircuitError> {
    validate_qubit_count(qubit_count, DEFAULT_MAX_QUBITS)?;

    if !two_qubit_probability.is_finite()
        || !(0.0..=1.0).contains(&two_qubit_probability)
    {
        return Err(MirrorCircuitError::InvalidProbability {
            probability: two_qubit_probability,
        });
    }

    let mut operations = Vec::with_capacity(qubit_count);

    if qubit_count >= 2
        && random_probability(rng) < two_qubit_probability
    {
        let control = uniform_index(rng, qubit_count);
        let mut target = uniform_index(rng, qubit_count - 1);

        // Map [0, qubit_count - 2] onto every qubit except `control`.
        if target >= control {
            target += 1;
        }

        operations.push(MirrorOperation::Cx { control, target });

        for qubit in 0..qubit_count {
            if qubit != control && qubit != target {
                operations.push(MirrorOperation::SingleQubit {
                    qubit,
                    primitive: random_clifford_primitive(rng),
                });
            }
        }
    } else {
        for qubit in 0..qubit_count {
            operations.push(MirrorOperation::SingleQubit {
                qubit,
                primitive: random_clifford_primitive(rng),
            });
        }
    }

    MirrorLayer::from_operations(operations, qubit_count)
}

/// Validates a layer.
///
/// The function enforces the fundamental logical-layer invariant:
///
/// > No qubit may participate in more than one operation in the same layer.
fn validate_layer_operations(
    operations: &[MirrorOperation],
    qubit_count: usize,
) -> Result<(), MirrorCircuitError> {
    validate_qubit_count(qubit_count, DEFAULT_MAX_QUBITS)?;

    let mut occupied = vec![false; qubit_count];

    for operation in operations {
        let (qubits, arity) = operation.qubits();

        for qubit in qubits.into_iter().take(arity) {
            validate_qubit_index(qubit, qubit_count)?;

            if occupied[qubit] {
                return Err(MirrorCircuitError::OverlappingLayerOperations {
                    qubit,
                });
            }

            occupied[qubit] = true;
        }

        if let MirrorOperation::Cx { control, target } = *operation {
            if control == target {
                return Err(MirrorCircuitError::DuplicateQubit {
                    qubit: control,
                });
            }
        }
    }

    Ok(())
}

/// Counts operations without allowing the sum to wrap.
fn count_layer_operations(
    layers: &[MirrorLayer],
) -> Result<usize, MirrorCircuitError> {
    let mut count = 0usize;

    for layer in layers {
        count = count
            .checked_add(layer.operation_count())
            .ok_or(MirrorCircuitError::SizeOverflow)?;
    }

    Ok(count)
}

/// Validates a qubit count against a caller-provided maximum.
fn validate_qubit_count(
    qubit_count: usize,
    maximum: usize,
) -> Result<(), MirrorCircuitError> {
    if qubit_count == 0 {
        return Err(MirrorCircuitError::InvalidQubitCount);
    }

    if qubit_count > maximum {
        return Err(MirrorCircuitError::QubitLimitExceeded {
            requested: qubit_count,
            maximum,
        });
    }

    Ok(())
}

/// Validates an individual qubit index.
fn validate_qubit_index(
    qubit: usize,
    qubit_count: usize,
) -> Result<(), MirrorCircuitError> {
    if qubit >= qubit_count {
        return Err(MirrorCircuitError::InvalidQubitIndex {
            qubit,
            qubit_count,
        });
    }

    Ok(())
}

/// Generates a uniformly distributed integer in [0, bound).
///
/// This implementation uses rejection sampling rather than modulo reduction
/// so that the result is not biased when `bound` does not divide the RNG's
/// native 32-bit range.
///
/// The function uses the high-level 32-bit output of `RngCore` because the
/// benchmark generator only requires bounded indices.
fn uniform_index<R: RngCore + ?Sized>(
    rng: &mut R,
    bound: usize,
) -> usize {
    debug_assert!(bound > 0);

    if bound == 1 {
        return 0;
    }

    let bound_u32 = match u32::try_from(bound) {
        Ok(value) => value,
        Err(_) => {
            // `bound` can be larger than u32::MAX on 64-bit platforms.
            // Fall back to a rejection sampler using the full usize width.
            return uniform_index_usize(rng, bound);
        }
    };

    let range = u32::MAX - (u32::MAX % bound_u32);

    loop {
        let value = rng.next_u32();

        if value < range {
            return (value % bound_u32) as usize;
        }
    }
}

/// Full-width bounded random integer for platforms where `usize` is wider
/// than 32 bits and the requested bound exceeds u32::MAX.
///
/// This function uses rejection sampling over the full `usize` range.
fn uniform_index_usize<R: RngCore + ?Sized>(
    rng: &mut R,
    bound: usize,
) -> usize {
    debug_assert!(bound > 0);

    if bound == 1 {
        return 0;
    }

    #[cfg(target_pointer_width = "64")]
    {
        let bound_u64 = bound as u64;
        let range = u64::MAX - (u64::MAX % bound_u64);

        loop {
            let value =
                ((rng.next_u64() as u64) << 32) | rng.next_u32() as u64;

            if value < range {
                return (value % bound_u64) as usize;
            }
        }
    }

    #[cfg(target_pointer_width = "32")]
    {
        let bound_u32 = bound as u32;
        let range = u32::MAX - (u32::MAX % bound_u32);

        loop {
            let value = rng.next_u32();

            if value < range {
                return (value % bound_u32) as usize;
            }
        }
    }
}

/// Generates a uniform floating-point value in [0, 1).
///
/// The implementation uses the upper 53 random bits and scales them into the
/// representable unit interval.
///
/// This avoids using floating-point modulo or platform-specific distribution
/// behavior and is deterministic for a given RNG stream.
fn random_probability<R: RngCore + ?Sized>(
    rng: &mut R,
) -> f64 {
    const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);

    let value = rng.next_u64() >> 11;

    (value as f64) * SCALE
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn seeded_rng(seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed)
    }

    #[test]
    fn generator_version_is_stable_and_nonzero() {
        assert_eq!(MIRROR_GENERATOR_VERSION, 1);
        assert!(!MIRROR_GENERATOR_ID.is_empty());
    }

    #[test]
    fn default_configuration_is_valid() {
        assert!(MirrorCircuitConfig::default().validate().is_ok());
    }

    #[test]
    fn zero_qubits_are_rejected() {
        let config = MirrorCircuitConfig {
            qubit_count: 0,
            ..MirrorCircuitConfig::default()
        };

        assert_eq!(
            config.validate(),
            Err(MirrorCircuitError::InvalidQubitCount)
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let config = MirrorCircuitConfig {
            two_qubit_probability: 1.5,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn negative_probability_is_rejected() {
        let config = MirrorCircuitConfig {
            two_qubit_probability: -0.1,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn nan_probability_is_rejected() {
        let config = MirrorCircuitConfig {
            two_qubit_probability: f64::NAN,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn infinity_probability_is_rejected() {
        let config = MirrorCircuitConfig {
            two_qubit_probability: f64::INFINITY,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn single_qubit_operation_inverse_is_correct() {
        let h = MirrorOperation::SingleQubit {
            qubit: 0,
            primitive: CliffordPrimitive::H,
        };

        let s = MirrorOperation::SingleQubit {
            qubit: 0,
            primitive: CliffordPrimitive::S,
        };

        let sdg = MirrorOperation::SingleQubit {
            qubit: 0,
            primitive: CliffordPrimitive::Sdg,
        };

        assert_eq!(h.inverse(), h);
        assert_eq!(
            s.inverse(),
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::Sdg
            }
        );
        assert_eq!(
            sdg.inverse(),
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::S
            }
        );
    }

    #[test]
    fn cx_is_self_inverse() {
        let cx = MirrorOperation::Cx {
            control: 0,
            target: 1,
        };

        assert!(cx.is_self_inverse());
        assert_eq!(cx.inverse(), cx);
    }

    #[test]
    fn overlapping_layer_is_rejected() {
        let operations = vec![
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::H,
            },
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::S,
            },
        ];

        let result = MirrorLayer::from_operations(operations, 1);

        assert!(matches!(
            result,
            Err(MirrorCircuitError::OverlappingLayerOperations {
                qubit: 0
            })
        ));
    }

    #[test]
    fn duplicate_cx_operands_are_rejected() {
        let operations = vec![MirrorOperation::Cx {
            control: 0,
            target: 0,
        }];

        let result = MirrorLayer::from_operations(operations, 1);

        assert_eq!(
            result,
            Err(MirrorCircuitError::DuplicateQubit { qubit: 0 })
        );
    }

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let operations = vec![MirrorOperation::SingleQubit {
            qubit: 2,
            primitive: CliffordPrimitive::H,
        }];

        let result = MirrorLayer::from_operations(operations, 2);

        assert_eq!(
            result,
            Err(MirrorCircuitError::InvalidQubitIndex {
                qubit: 2,
                qubit_count: 2
            })
        );
    }

    #[test]
    fn generated_single_qubit_mirror_has_exact_inverse() {
        let mut rng = seeded_rng(42);

        let config = MirrorCircuitConfig {
            qubit_count: 1,
            forward_depth: 32,
            two_qubit_probability: 1.0,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(circuit.forward_depth(), 32);
        assert_eq!(circuit.total_depth(), 64);
        assert!(circuit.validate_exact_inverse().is_ok());
        assert_eq!(
            circuit.forward_operation_count(),
            circuit.inverse_operation_count()
        );
    }

    #[test]
    fn generated_multi_qubit_mirror_has_exact_inverse() {
        let mut rng = seeded_rng(12345);

        let config = MirrorCircuitConfig {
            qubit_count: 8,
            forward_depth: 50,
            two_qubit_probability: 0.75,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(circuit.qubit_count(), 8);
        assert_eq!(
            circuit.forward_operation_count(),
            circuit.inverse_operation_count()
        );
        assert!(circuit.validate_exact_inverse().is_ok());
    }

    #[test]
    fn two_qubit_probability_zero_generates_no_cx() {
        let mut rng = seeded_rng(7);

        let config = MirrorCircuitConfig {
            qubit_count: 16,
            forward_depth: 10,
            two_qubit_probability: 0.0,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(circuit.forward_cx_count(), 0);
        assert_eq!(
            circuit.forward_single_qubit_operation_count(),
            16 * 10
        );
    }

    #[test]
    fn two_qubit_probability_one_generates_cx_for_multi_qubit_layers() {
        let mut rng = seeded_rng(7);

        let config = MirrorCircuitConfig {
            qubit_count: 8,
            forward_depth: 10,
            two_qubit_probability: 1.0,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(circuit.forward_cx_count(), 10);
        assert_eq!(
            circuit.forward_single_qubit_operation_count(),
            6 * 10
        );
    }

    #[test]
    fn one_qubit_configuration_never_generates_cx() {
        let mut rng = seeded_rng(999);

        let config = MirrorCircuitConfig {
            qubit_count: 1,
            forward_depth: 100,
            two_qubit_probability: 1.0,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(circuit.forward_cx_count(), 0);
        assert_eq!(
            circuit.forward_single_qubit_operation_count(),
            100
        );
    }

    #[test]
    fn deterministic_seed_reproduces_exact_circuit() {
        let config = MirrorCircuitConfig {
            qubit_count: 12,
            forward_depth: 20,
            two_qubit_probability: 0.5,
            ..MirrorCircuitConfig::default()
        };

        let mut rng_a = seeded_rng(123456789);
        let mut rng_b = seeded_rng(123456789);

        let circuit_a =
            generate_mirror_circuit(&mut rng_a, config).expect("generation");

        let circuit_b =
            generate_mirror_circuit(&mut rng_b, config).expect("generation");

        assert_eq!(circuit_a, circuit_b);
    }

    #[test]
    fn different_seeds_can_produce_different_circuits() {
        let config = MirrorCircuitConfig {
            qubit_count: 8,
            forward_depth: 20,
            two_qubit_probability: 0.5,
            ..MirrorCircuitConfig::default()
        };

        let mut rng_a = seeded_rng(1);
        let mut rng_b = seeded_rng(2);

        let circuit_a =
            generate_mirror_circuit(&mut rng_a, config).expect("generation");

        let circuit_b =
            generate_mirror_circuit(&mut rng_b, config).expect("generation");

        assert_ne!(circuit_a, circuit_b);
    }

    #[test]
    fn flat_operation_constructor_creates_exact_mirror() {
        let operations = vec![
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::H,
            },
            MirrorOperation::Cx {
                control: 0,
                target: 1,
            },
            MirrorOperation::SingleQubit {
                qubit: 1,
                primitive: CliffordPrimitive::S,
            },
        ];

        let circuit =
            mirror_from_operations(2, operations).expect("construction");

        assert_eq!(circuit.forward_depth(), 3);
        assert_eq!(circuit.total_depth(), 6);
        assert_eq!(circuit.forward_operation_count(), 3);
        assert_eq!(circuit.inverse_operation_count(), 3);
        assert!(circuit.validate_exact_inverse().is_ok());
    }

    #[test]
    fn identity_mirror_is_empty() {
        let circuit =
            identity_mirror_circuit(4).expect("identity construction");

        assert!(circuit.is_identity());
        assert_eq!(circuit.forward_depth(), 0);
        assert_eq!(circuit.total_depth(), 0);
        assert_eq!(circuit.total_operation_count(), 0);
        assert!(circuit.validate_exact_inverse().is_ok());
    }

    #[test]
    fn inverse_layers_are_reverse_of_forward_layers() {
        let layer_a = MirrorLayer::from_operations(
            vec![MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::H,
            }],
            1,
        )
        .expect("layer");

        let layer_b = MirrorLayer::from_operations(
            vec![MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::S,
            }],
            1,
        )
        .expect("layer");

        let circuit =
            mirror_from_layers(1, vec![layer_a.clone(), layer_b.clone()])
                .expect("circuit");

        assert_eq!(
            circuit.inverse_layers()[0],
            layer_b.inverse()
        );

        assert_eq!(
            circuit.inverse_layers()[1],
            layer_a.inverse()
        );
    }

    #[test]
    fn generated_layers_have_no_overlapping_qubits() {
        let mut rng = seeded_rng(456);

        let config = MirrorCircuitConfig {
            qubit_count: 32,
            forward_depth: 100,
            two_qubit_probability: 1.0,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        for layer in circuit.forward_layers() {
            let mut used = vec![false; circuit.qubit_count()];

            for operation in layer.operations() {
                let (qubits, arity) = operation.qubits();

                for qubit in qubits.into_iter().take(arity) {
                    assert!(!used[qubit]);
                    used[qubit] = true;
                }
            }
        }
    }

    #[test]
    fn operation_count_is_symmetric() {
        let mut rng = seeded_rng(777);

        let config = MirrorCircuitConfig {
            qubit_count: 20,
            forward_depth: 25,
            two_qubit_probability: 0.4,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        assert_eq!(
            circuit.forward_operation_count(),
            circuit.inverse_operation_count()
        );

        assert_eq!(
            circuit.total_operation_count(),
            circuit.forward_operation_count() * 2
        );
    }

    #[test]
    fn total_depth_is_twice_forward_depth() {
        let mut rng = seeded_rng(8);

        for depth in [0usize, 1, 2, 10, 100] {
            let config = MirrorCircuitConfig {
                qubit_count: 4,
                forward_depth: depth,
                ..MirrorCircuitConfig::default()
            };

            let circuit =
                generate_mirror_circuit(&mut rng, config).expect("generation");

            assert_eq!(circuit.total_depth(), depth * 2);
        }
    }

    #[test]
    fn inverse_of_inverse_operation_is_original() {
        let operations = [
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::H,
            },
            MirrorOperation::SingleQubit {
                qubit: 1,
                primitive: CliffordPrimitive::S,
            },
            MirrorOperation::SingleQubit {
                qubit: 2,
                primitive: CliffordPrimitive::Sdg,
            },
            MirrorOperation::Cx {
                control: 0,
                target: 2,
            },
        ];

        for operation in operations {
            assert_eq!(operation.inverse().inverse(), operation);
        }
    }

    #[test]
    fn inverse_layer_preserves_operation_count() {
        let layer = MirrorLayer::from_operations(
            vec![
                MirrorOperation::SingleQubit {
                    qubit: 0,
                    primitive: CliffordPrimitive::H,
                },
                MirrorOperation::SingleQubit {
                    qubit: 1,
                    primitive: CliffordPrimitive::S,
                },
            ],
            2,
        )
        .expect("layer");

        assert_eq!(
            layer.operation_count(),
            layer.inverse().operation_count()
        );
    }

    #[test]
    fn max_qubit_limit_is_enforced() {
        let config = MirrorCircuitConfig {
            qubit_count: 9,
            max_qubits: 8,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::QubitLimitExceeded {
                requested: 9,
                maximum: 8
            })
        ));
    }

    #[test]
    fn max_depth_limit_is_enforced() {
        let config = MirrorCircuitConfig {
            forward_depth: 100,
            max_forward_depth: 10,
            ..MirrorCircuitConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(MirrorCircuitError::DepthExceeded {
                requested: 100,
                maximum: 10
            })
        ));
    }

    #[test]
    fn total_operation_limit_is_enforced_during_generation() {
        let mut rng = seeded_rng(42);

        let config = MirrorCircuitConfig {
            qubit_count: 10,
            forward_depth: 10,
            two_qubit_probability: 0.0,
            max_total_operations: 100,
            ..MirrorCircuitConfig::default()
        };

        // 10 qubits × 10 layers = 100 forward operations,
        // therefore 200 operations in the complete mirror circuit.
        let result = generate_mirror_circuit(&mut rng, config);

        assert!(matches!(
            result,
            Err(MirrorCircuitError::OperationLimitExceeded {
                requested: 200,
                maximum: 100
            })
        ));
    }

    #[test]
    fn layer_constructor_accepts_disjoint_operations() {
        let layer = MirrorLayer::from_operations(
            vec![
                MirrorOperation::SingleQubit {
                    qubit: 0,
                    primitive: CliffordPrimitive::H,
                },
                MirrorOperation::SingleQubit {
                    qubit: 1,
                    primitive: CliffordPrimitive::S,
                },
                MirrorOperation::Cx {
                    control: 2,
                    target: 3,
                },
            ],
            4,
        );

        assert!(layer.is_ok());
    }

    #[test]
    fn flat_operations_are_each_separate_layer() {
        let operations = vec![
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::H,
            },
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::S,
            },
        ];

        let circuit =
            mirror_from_operations(1, operations).expect("construction");

        assert_eq!(circuit.forward_depth(), 2);
        assert_eq!(circuit.forward_operation_count(), 2);
    }

    #[test]
    fn all_generated_operations_have_valid_qubits() {
        let mut rng = seeded_rng(321);

        let config = MirrorCircuitConfig {
            qubit_count: 64,
            forward_depth: 20,
            two_qubit_probability: 0.5,
            ..MirrorCircuitConfig::default()
        };

        let circuit =
            generate_mirror_circuit(&mut rng, config).expect("generation");

        for operation in circuit.operations() {
            let (qubits, arity) = operation.qubits();

            for qubit in qubits.into_iter().take(arity) {
                assert!(qubit < circuit.qubit_count());
            }
        }
    }

    #[test]
    fn no_randomness_is_consumed_by_identity_construction() {
        let circuit =
            identity_mirror_circuit(2).expect("identity construction");

        assert_eq!(circuit.forward_operation_count(), 0);
        assert_eq!(circuit.inverse_operation_count(), 0);
    }

    #[test]
    fn exact_inverse_validation_is_independent_of_simulation() {
        let operations = vec![
            MirrorOperation::SingleQubit {
                qubit: 0,
                primitive: CliffordPrimitive::S,
            },
            MirrorOperation::Cx {
                control: 0,
                target: 1,
            },
            MirrorOperation::SingleQubit {
                qubit: 1,
                primitive: CliffordPrimitive::H,
            },
        ];

        let circuit =
            mirror_from_operations(2, operations).expect("construction");

        // This test deliberately verifies the structural contract only.
        // No simulator is required.
        assert!(circuit.validate_exact_inverse().is_ok());
    }
}