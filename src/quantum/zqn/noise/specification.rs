//! Zamani Quantum Noise (ZQN) — Declarative Noise Specification
//!
//! This module defines the canonical, backend-independent description of a
//! quantum-noise program.
//!
//! # Architectural role
//!
//! `specification.rs` answers:
//!
//! > "What noise semantics has the user requested or declared?"
//!
//! It does NOT answer:
//!
//! - how the noise is numerically simulated;
//! - how a channel is represented internally;
//! - how faults are sampled;
//! - how a QPU executes the noise;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how QEC decodes faults;
//! - how calibration is acquired;
//! - how a vendor backend is contacted.
//!
//! Those responsibilities belong to other ZQN and quantum subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source / frontend
//!          │
//!          ▼
//!     quantum::ir
//!          │
//!          ├───────────────────────────────┐
//!          │                               │
//!          ▼                               ▼
//!   canonical program              ZQN specification
//!                                          │
//!                                          ▼
//!                                   noise semantics
//!                                          │
//!                ┌─────────────────────────┼─────────────────────┐
//!                ▼                         ▼                     ▼
//!             routing                  scheduling               QEC
//!                │                         │                     │
//!                └─────────────────────────┼─────────────────────┘
//!                                          ▼
//!                                      execution
//! ```
//!
//! # Write once, scale everywhere
//!
//! A `NoiseSpecification` contains no semantic maximum for:
//!
//! - qubit count;
//! - operation count;
//! - circuit depth;
//! - number of noise rules;
//! - correlation-domain size;
//! - number of devices;
//! - number of quantum modes;
//! - number of calibration parameters;
//! - number of execution shots.
//!
//! Collections are therefore represented as ordinary dynamically sized Rust
//! collections and are constrained only by the explicit resource policy of the
//! surrounding execution/validation layer and the resources available to the
//! host system.
//!
//! "Infinity" in the Zamani architecture means:
//!
//! > no artificial finite machine-size ceiling is encoded into this semantic
//! > model.
//!
//! It does NOT claim that physical memory, storage, network bandwidth, runtime,
//! or a quantum processor is infinite.
//!
//! # Canonical quantum-resource identity
//!
//! Where a rule explicitly targets a logical or physical qubit, this module
//! uses the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! It does NOT define a second ZQN `QubitId`.
//!
//! This preserves the repository-wide rule that quantum-resource identity is
//! owned by the canonical Quantum IR.
//!
//! # Declarative design
//!
//! A specification describes requested semantics. It is not itself a noise
//! realization.
//!
//! Therefore:
//!
//! ```text
//! NoiseSpecification
//!        │
//!        ▼
//! validation
//!        │
//!        ▼
//! NoiseModel
//!        │
//!        ▼
//! NoiseApplication / channel / fault realization
//! ```
//!
//! This separation is intentional. The same specification can be validated,
//! compiled, simulated, routed, scheduled, benchmarked, or executed against
//! different targets without rewriting the user's quantum program.
//!
//! # Exact versus approximate semantics
//!
//! A specification can explicitly state whether its requested semantics must
//! be:
//!
//! - exact;
//! - approximate within a declared tolerance;
//! - bounded by a declared error bound;
//! - statistically estimated at a declared confidence level;
//! - otherwise rejected.
//!
//! Implementations MUST NOT silently replace an exact request with an
//! approximation.
//!
//! # Determinism
//!
//! This module contains no random-number generation.
//!
//! A specification is deterministic data.
//!
//! Stochastic realization belongs to the ZQN simulation/sampling layer and
//! must use an explicit reproducibility context rather than hidden global RNG
//! state.
//!
//! # Resource safety
//!
//! This module does not impose arbitrary global constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_NOISE_RULES
//! MAX_CORRELATIONS
//! MAX_OPERATIONS
//! ```
//!
//! Such limits are execution/resource-policy decisions.
//!
//! Validation code may apply explicit limits before materializing expensive
//! representations.
//!
//! # Serialization
//!
//! This module intentionally does not depend on a serialization crate.
//!
//! The structure is designed to be serializable by the future ZQN `io`
//! subsystem without making the semantic layer depend on that subsystem.
//!
//! Serialization MUST preserve:
//!
//! - semantic meaning;
//! - rule ordering where ordering is semantically significant;
//! - target/resource identity domain;
//! - parameter values;
//! - approximation policy;
//! - provenance references;
//! - extension information.
//!
//! Serialization MUST NOT silently discard unknown semantic fields.
//!
//! # Forward compatibility
//!
//! Extension points are represented explicitly rather than by accepting
//! arbitrary strings as semantic values.
//!
//! Consumers should reject unsupported extensions when they affect semantics,
//! unless an explicit compatibility policy allows them.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! Producers:
//!
//! - Zamani quantum frontend;
//! - canonical quantum IR lowering;
//! - user APIs;
//! - calibration/characterization importers;
//! - future ZQN schema deserializers.
//!
//! Consumers:
//!
//! - `noise::model`;
//! - `noise::application`;
//! - `noise::composition`;
//! - `noise::correlation`;
//! - `noise::temporal`;
//! - `noise::spatial`;
//! - `noise::crosstalk`;
//! - `simulation`;
//! - `propagation`;
//! - `target`;
//! - routing integration;
//! - scheduling integration;
//! - QEC integration;
//! - hardware integration;
//! - benchmarking integration.
//!
//! The specification module must remain independent of those consumers.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! 1. all declarative noise categories can be represented without vendor
//!    knowledge;
//! 2. no machine-size assumption exists in the semantic API;
//! 3. logical/physical qubit identities use the canonical IR types;
//! 4. exact/approximate semantics are explicit;
//! 5. validation can detect structurally invalid specifications;
//! 6. deterministic ordering is preserved;
//! 7. specifications can be consumed without coupling to an execution engine;
//! 8. no unsafe code is present;
//! 9. future noise implementations can consume this API without modifying its
//!    semantic foundations;
//! 10. the module remains valid independently of the implementation details of
//!     later ZQN files.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::NoiseModelId;

// ============================================================================
// Fundamental scalar types
// ============================================================================

/// Non-negative finite probability.
///
/// This type deliberately does not depend on the future probability module.
/// It provides the minimum scalar required for declarative noise
/// specifications while keeping validation local to this file.
///
/// The internal representation is `f64` because the specification layer must
/// be able to interoperate with existing numerical APIs. Exact rational or
/// interval representations can be introduced by the probability subsystem
/// without changing the rule topology defined here.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NoiseProbability(f64);

impl NoiseProbability {
    /// Creates a probability after validating finiteness and bounds.
    pub fn new(value: f64) -> ZqnResult<Self> {
        if !value.is_finite() {
            return Err(invalid_spec(
                "noise probability must be finite",
            ));
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(invalid_spec(
                "noise probability must be within [0, 1]",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the underlying probability.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns one.
    #[must_use]
    pub const fn one() -> Self {
        Self(1.0)
    }
}

impl Eq for NoiseProbability {}

impl std::hash::Hash for NoiseProbability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for NoiseProbability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// ============================================================================
// Tolerances and approximation
// ============================================================================

/// Explicit numerical tolerance.
///
/// A tolerance is always finite and non-negative.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NoiseTolerance(f64);

impl NoiseTolerance {
    /// Creates a validated tolerance.
    pub fn new(value: f64) -> ZqnResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(invalid_spec(
                "noise tolerance must be finite and non-negative",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the tolerance.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Eq for NoiseTolerance {}

impl std::hash::Hash for NoiseTolerance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for NoiseTolerance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Declares the required semantic fidelity of a noise specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApproximationPolicy {
    /// The requested semantics must be represented without approximation.
    Exact,

    /// Approximation is allowed only when the declared tolerance is met.
    Approximate {
        /// Maximum permitted semantic/numerical deviation.
        tolerance: NoiseTolerance,
    },

    /// Approximation is allowed only when an explicit bound is provided.
    Bounded {
        /// Maximum permitted error bound.
        error_bound: NoiseTolerance,
    },

    /// The result is inherently statistical and must carry a confidence
    /// requirement.
    Statistical {
        /// Required confidence level in [0, 1].
        confidence: NoiseProbability,
    },

    /// The implementation may choose an appropriate representation, but it
    /// must expose the resulting approximation/error contract.
    Adaptive,
}

impl Eq for ApproximationPolicy {}

impl std::hash::Hash for ApproximationPolicy {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Exact | Self::Adaptive => {}
            Self::Approximate { tolerance } => tolerance.hash(state),
            Self::Bounded { error_bound } => error_bound.hash(state),
            Self::Statistical { confidence } => confidence.hash(state),
        }
    }
}

impl Default for ApproximationPolicy {
    fn default() -> Self {
        Self::Exact
    }
}

// ============================================================================
// Semantic scope
// ============================================================================

/// Describes the semantic domain to which a noise rule applies.
///
/// This is intentionally broader than qubits and gates so ZQN can represent
/// future quantum modalities such as qudits, modes, bosonic systems, analog
/// resources, distributed links, logical resources, and pulse-level systems.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoiseScope {
    /// Applies to every matching resource in the current execution context.
    Global,

    /// Applies to a logical qubit.
    LogicalQubit(QubitId),

    /// Applies to a physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Applies to a set of logical qubits.
    LogicalQubits(Vec<QubitId>),

    /// Applies to a set of physical qubits.
    PhysicalQubits(Vec<PhysicalQubitId>),

    /// Applies to a named logical resource.
    LogicalResource(String),

    /// Applies to a named physical resource.
    PhysicalResource(String),

    /// Applies to a resource identified by an externally owned stable name.
    Resource(String),

    /// Applies to all resources satisfying a declarative selector.
    Selector(ResourceSelector),

    /// Applies to a composite resource.
    Composite(Vec<NoiseScope>),
}

impl NoiseScope {
    /// Creates a logical-qubit scope.
    #[must_use]
    pub fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit scope.
    #[must_use]
    pub fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a logical-qubit collection scope.
    pub fn logical_qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::LogicalQubits(qubits.into_iter().collect())
    }

    /// Creates a physical-qubit collection scope.
    pub fn physical_qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        Self::PhysicalQubits(qubits.into_iter().collect())
    }

    /// Returns true when the scope contains no explicitly listed resources.
    #[must_use]
    pub fn is_explicitly_empty(&self) -> bool {
        matches!(
            self,
            Self::LogicalQubits(values) | Self::PhysicalQubits(values)
                if values.is_empty()
        )
    }
}

/// Declarative resource selection.
///
/// Selectors avoid forcing the specification to enumerate every resource of a
/// potentially enormous machine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceSelector {
    /// Select resources carrying a label.
    Label(String),

    /// Select resources belonging to a resource class.
    ResourceClass(String),

    /// Select resources belonging to a topology/domain.
    Domain(String),

    /// Select resources matching all supplied selectors.
    All(Vec<ResourceSelector>),

    /// Select resources matching any supplied selector.
    Any(Vec<ResourceSelector>),

    /// Exclude resources matching the nested selector.
    Not(Box<ResourceSelector>),
}

// ============================================================================
// Operation scope
// ============================================================================

/// Declarative operation category to which noise can attach.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationScope {
    /// Any operation.
    Any,

    /// A unitary operation.
    Unitary,

    /// A named gate or operation.
    Named(String),

    /// Preparation/initialization.
    Preparation,

    /// Reset.
    Reset,

    /// Measurement.
    Measurement,

    /// Idle/delay interval.
    Idle,

    /// Pulse/control operation.
    Pulse,

    /// Transport/shuttling/movement.
    Transport,

    /// Communication/link operation.
    Communication,

    /// Analog evolution.
    AnalogEvolution,

    /// Annealing evolution.
    Annealing,

    /// Hamiltonian evolution.
    HamiltonianEvolution,

    /// Measurement-based operation.
    MeasurementBased,

    /// Composite operation.
    Composite(Vec<OperationScope>),
}

// ============================================================================
// Temporal scope
// ============================================================================

/// Describes when a noise rule applies.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TemporalScope {
    /// Applies regardless of execution time.
    Always,

    /// Applies within a logical execution-time interval.
    Interval {
        /// Inclusive lower bound.
        start: TimeValue,

        /// Exclusive upper bound.
        end: TimeValue,
    },

    /// Applies at a specific logical execution time.
    At(TimeValue),

    /// Applies according to a named temporal phase.
    Phase(String),

    /// Applies according to a declarative temporal predicate.
    Predicate(TemporalPredicate),
}

/// Target-independent time value.
///
/// The unit is represented explicitly so the specification does not silently
/// assume nanoseconds, microseconds, clock cycles, or another backend-specific
/// unit.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeValue {
    /// Numeric magnitude.
    value: f64,

    /// Explicit unit.
    unit: TimeUnit,
}

impl TimeValue {
    /// Creates a finite, non-negative time value.
    pub fn new(value: f64, unit: TimeUnit) -> ZqnResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(invalid_spec(
                "time value must be finite and non-negative",
            ));
        }

        Ok(Self { value, unit })
    }

    /// Returns the numeric magnitude.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the unit.
    #[must_use]
    pub const fn unit(&self) -> TimeUnit {
        self.unit
    }
}

impl Eq for TimeValue {}

impl std::hash::Hash for TimeValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
        self.unit.hash(state);
    }
}

/// Explicit time unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnit {
    /// Seconds.
    Seconds,

    /// Milliseconds.
    Milliseconds,

    /// Microseconds.
    Microseconds,

    /// Nanoseconds.
    Nanoseconds,

    /// Picoseconds.
    Picoseconds,

    /// Femtoseconds.
    Femtoseconds,

    /// Backend-independent clock cycles.
    Cycles,
}

/// Declarative temporal predicate.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TemporalPredicate {
    /// Parameter drift condition.
    ParameterChanged(String),

    /// Calibration validity condition.
    CalibrationValid,

    /// Calibration invalidity condition.
    CalibrationInvalid,

    /// Named execution phase.
    Phase(String),

    /// User-defined semantic predicate.
    Custom(String),
}

// ============================================================================
// Noise mechanism
// ============================================================================

/// High-level noise mechanism.
///
/// The variants describe physical semantics rather than a particular
/// simulation representation.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseMechanism {
    /// Bit-flip-type stochastic noise.
    BitFlip {
        probability: NoiseProbability,
    },

    /// Phase-flip-type stochastic noise.
    PhaseFlip {
        probability: NoiseProbability,
    },

    /// Depolarizing noise.
    Depolarizing {
        probability: NoiseProbability,
    },

    /// Amplitude damping.
    AmplitudeDamping {
        probability: NoiseProbability,
    },

    /// Phase damping/dephasing.
    PhaseDamping {
        probability: NoiseProbability,
    },

    /// Thermal relaxation.
    ThermalRelaxation {
        relaxation_time: TimeValue,
        excitation_time: Option<TimeValue>,
    },

    /// Generic stochastic Pauli error.
    Pauli {
        /// Pauli terms represented symbolically.
        terms: Vec<PauliTerm>,
    },

    /// Generalized channel identified by an external channel/model reference.
    ChannelReference {
        channel: String,
    },

    /// Generic stochastic distribution.
    Stochastic {
        distribution: DistributionReference,
    },

    /// Continuous-time Lindblad-style dynamics.
    Lindblad {
        generator: GeneratorReference,
    },

    /// Leakage out of the computational subspace.
    Leakage {
        probability: NoiseProbability,
    },

    /// Erasure.
    Erasure {
        probability: NoiseProbability,
    },

    /// Loss.
    Loss {
        probability: NoiseProbability,
    },

    /// Coherent/control error.
    Coherent {
        parameter: String,
        magnitude: f64,
    },

    /// Readout assignment error.
    ReadoutAssignment {
        /// Mapping from observed symbolic outcome to error probability.
        outcomes: Vec<ReadoutError>,
    },

    /// State-preparation error.
    PreparationError {
        probability: NoiseProbability,
    },

    /// Crosstalk.
    Crosstalk {
        interaction: CrosstalkReference,
    },

    /// User-defined mechanism whose semantics are provided by an extension.
    Extension(NoiseExtension),
}

impl Eq for NoiseMechanism {}

impl std::hash::Hash for NoiseMechanism {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::BitFlip { probability }
            | Self::PhaseFlip { probability }
            | Self::Depolarizing { probability }
            | Self::AmplitudeDamping { probability }
            | Self::PhaseDamping { probability }
            | Self::Leakage { probability }
            | Self::Erasure { probability }
            | Self::Loss { probability }
            | Self::PreparationError { probability } => {
                probability.hash(state);
            }

            Self::ThermalRelaxation {
                relaxation_time,
                excitation_time,
            } => {
                relaxation_time.hash(state);

                excitation_time.hash(state);
            }

            Self::Pauli { terms } => {
                terms.hash(state);
            }

            Self::ChannelReference { channel } => {
                channel.hash(state);
            }

            Self::Stochastic { distribution } => {
                distribution.hash(state);
            }

            Self::Lindblad { generator } => {
                generator.hash(state);
            }

            Self::Coherent {
                parameter,
                magnitude,
            } => {
                parameter.hash(state);
                magnitude.to_bits().hash(state);
            }

            Self::ReadoutAssignment { outcomes } => {
                outcomes.hash(state);
            }

            Self::Crosstalk { interaction } => {
                interaction.hash(state);
            }

            Self::Extension(extension) => {
                extension.hash(state);
            }
        }
    }
}

// ============================================================================
// Pauli semantics
// ============================================================================

/// Symbolic Pauli term.
///
/// The term is intentionally not tied to a fixed qubit count.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauliTerm {
    /// Symbolic Pauli operator.
    pub operator: PauliOperator,

    /// Probability associated with this term.
    pub probability: NoiseProbability,

    /// Optional resource scope.
    pub scope: Option<NoiseScope>,
}

/// Pauli operator symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PauliOperator {
    /// Identity.
    I,

    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

// ============================================================================
// Generic external references
// ============================================================================

/// Reference to a future distribution implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DistributionReference {
    /// Stable external/reference name.
    pub name: String,
}

/// Reference to a generator representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratorReference {
    /// Stable generator/model name.
    pub name: String,
}

/// Reference to a crosstalk interaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrosstalkReference {
    /// Stable interaction/model reference.
    pub name: String,
}

/// Readout error declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadoutError {
    /// Logical/physical result label.
    pub outcome: String,

    /// Probability that the reported result is incorrect according to the
    /// declared model.
    pub probability: NoiseProbability,
}

impl Eq for ReadoutError {}

impl std::hash::Hash for ReadoutError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.outcome.hash(state);
        self.probability.hash(state);
    }
}

// ============================================================================
// Extension model
// ============================================================================

/// Explicit extension mechanism for future noise technologies.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseExtension {
    /// Extension namespace.
    pub namespace: String,

    /// Extension semantic type.
    pub kind: String,

    /// Version of the extension semantics.
    pub version: String,

    /// Opaque canonical payload.
    ///
    /// The semantic layer does not interpret this field.
    pub payload: String,
}

impl NoiseExtension {
    /// Creates an extension after validating its identity fields.
    pub fn new<N, K, V, P>(
        namespace: N,
        kind: K,
        version: V,
        payload: P,
    ) -> ZqnResult<Self>
    where
        N: Into<String>,
        K: Into<String>,
        V: Into<String>,
        P: Into<String>,
    {
        let namespace = namespace.into();
        let kind = kind.into();
        let version = version.into();
        let payload = payload.into();

        if namespace.trim().is_empty() {
            return Err(invalid_spec("noise extension namespace cannot be empty"));
        }

        if kind.trim().is_empty() {
            return Err(invalid_spec("noise extension kind cannot be empty"));
        }

        if version.trim().is_empty() {
            return Err(invalid_spec("noise extension version cannot be empty"));
        }

        Ok(Self {
            namespace,
            kind,
            version,
            payload,
        })
    }
}

// ============================================================================
// Correlation
// ============================================================================

/// Declarative correlation specification.
///
/// The number of correlated resources is never fixed by this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorrelationSpec {
    /// No correlation beyond independent application.
    Independent,

    /// Fully correlated mechanism.
    FullyCorrelated,

    /// Named correlation model.
    Model(String),

    /// Explicit resource correlation.
    Resources(Vec<NoiseScope>),

    /// Spatial correlation.
    Spatial {
        model: String,
    },

    /// Temporal correlation.
    Temporal {
        model: String,
    },

    /// Space-time correlation.
    Spatiotemporal {
        model: String,
    },

    /// User-defined correlation extension.
    Extension(NoiseExtension),
}

// ============================================================================
// Rule condition
// ============================================================================

/// Condition under which a rule is active.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoiseCondition {
    /// Rule always applies.
    Always,

    /// Rule applies to a specific temporal scope.
    Temporal(TemporalScope),

    /// Rule applies only when a named calibration parameter has a condition.
    Calibration(String),

    /// Rule applies only for a named execution context.
    ExecutionContext(String),

    /// Rule applies when another rule/model is active.
    ModelActive(NoiseModelId),

    /// All conditions must hold.
    All(Vec<NoiseCondition>),

    /// At least one condition must hold.
    Any(Vec<NoiseCondition>),

    /// Nested condition must not hold.
    Not(Box<NoiseCondition>),
}

// ============================================================================
// Noise rule
// ============================================================================

/// One declarative noise rule.
///
/// A rule combines:
///
/// - where the noise applies;
/// - which operations it affects;
/// - when it applies;
/// - what physical mechanism is requested;
/// - how resources are correlated;
/// - what approximation semantics are allowed.
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseRule {
    /// Stable rule name within the specification.
    pub name: String,

    /// Resource scope.
    pub scope: NoiseScope,

    /// Operation scope.
    pub operation: OperationScope,

    /// Temporal scope.
    pub temporal: TemporalScope,

    /// Optional additional condition.
    pub condition: NoiseCondition,

    /// Requested physical mechanism.
    pub mechanism: NoiseMechanism,

    /// Correlation semantics.
    pub correlation: CorrelationSpec,

    /// Required semantic fidelity.
    pub approximation: ApproximationPolicy,

    /// Optional priority for deterministic rule resolution.
    ///
    /// Priority is semantic ordering, not machine scheduling priority.
    pub priority: i64,

    /// Whether another matching rule may also apply.
    pub composable: bool,

    /// Human-readable description.
    pub description: Option<String>,
}

impl Eq for NoiseRule {}

impl std::hash::Hash for NoiseRule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.scope.hash(state);
        self.operation.hash(state);
        self.temporal.hash(state);
        self.condition.hash(state);
        self.mechanism.hash(state);
        self.correlation.hash(state);
        self.approximation.hash(state);
        self.priority.hash(state);
        self.composable.hash(state);
        self.description.hash(state);
    }
}

impl NoiseRule {
    /// Creates a rule with safe semantic defaults.
    pub fn new<N>(
        name: N,
        scope: NoiseScope,
        operation: OperationScope,
        mechanism: NoiseMechanism,
    ) -> ZqnResult<Self>
    where
        N: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(invalid_spec("noise rule name cannot be empty"));
        }

        Ok(Self {
            name,
            scope,
            operation,
            temporal: TemporalScope::Always,
            condition: NoiseCondition::Always,
            mechanism,
            correlation: CorrelationSpec::Independent,
            approximation: ApproximationPolicy::Exact,
            priority: 0,
            composable: true,
            description: None,
        })
    }

    /// Sets the temporal scope.
    #[must_use]
    pub fn with_temporal(mut self, temporal: TemporalScope) -> Self {
        self.temporal = temporal;
        self
    }

    /// Sets the condition.
    #[must_use]
    pub fn with_condition(mut self, condition: NoiseCondition) -> Self {
        self.condition = condition;
        self
    }

    /// Sets correlation semantics.
    #[must_use]
    pub fn with_correlation(mut self, correlation: CorrelationSpec) -> Self {
        self.correlation = correlation;
        self
    }

    /// Sets the approximation policy.
    #[must_use]
    pub fn with_approximation(mut self, approximation: ApproximationPolicy) -> Self {
        self.approximation = approximation;
        self
    }

    /// Sets deterministic rule priority.
    #[must_use]
    pub fn with_priority(mut self, priority: i64) -> Self {
        self.priority = priority;
        self
    }

    /// Controls whether this rule composes with other matching rules.
    #[must_use]
    pub fn with_composability(mut self, composable: bool) -> Self {
        self.composable = composable;
        self
    }

    /// Adds a human-readable description.
    #[must_use]
    pub fn with_description<D>(mut self, description: D) -> Self
    where
        D: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    /// Performs local structural validation.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.name.trim().is_empty() {
            return Err(invalid_spec("noise rule name cannot be empty"));
        }

        if self.scope.is_explicitly_empty() {
            return Err(invalid_spec(
                "noise rule cannot target an explicitly empty resource set",
            ));
        }

        validate_mechanism(&self.mechanism)?;

        validate_correlation(&self.correlation)?;

        validate_condition(&self.condition)?;

        validate_temporal(&self.temporal)?;

        Ok(())
    }
}

// ============================================================================
// Specification metadata
// ============================================================================

/// Declarative metadata associated with a noise specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NoiseSpecificationMetadata {
    /// Stable human-readable name.
    pub name: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Semantic author/source identifier.
    pub source: Option<String>,

    /// Optional semantic version of the specification.
    pub version: Option<String>,

    /// Arbitrary stable labels.
    pub labels: Vec<(String, String)>,

    /// Optional provenance reference.
    pub provenance: Option<String>,
}

impl NoiseSpecificationMetadata {
    /// Validates metadata without imposing a naming convention.
    pub fn validate(&self) -> ZqnResult<()> {
        for (key, value) in &self.labels {
            if key.trim().is_empty() {
                return Err(invalid_spec("noise specification label key cannot be empty"));
            }

            if value.trim().is_empty() {
                return Err(invalid_spec(
                    "noise specification label value cannot be empty",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Noise specification
// ============================================================================

/// Complete declarative ZQN noise specification.
///
/// This is the primary type defined by this module.
///
/// It is intentionally independent from:
///
/// - simulation;
/// - channel implementation;
/// - QEC;
/// - hardware;
/// - routing;
/// - scheduling;
/// - serialization.
///
/// It can therefore be finalized before those downstream subsystems are
/// implemented.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseSpecification {
    /// Optional model identity.
///
/// `None` means the specification is anonymous until registered by the
/// owning subsystem.
    pub model_id: Option<NoiseModelId>,

    /// Schema/semantic version selected by the owner of the ZQN versioning
    /// subsystem.
///
/// This remains a string here to keep this foundational file independent from
/// version.rs.
    pub schema_version: String,

    /// Metadata.
    pub metadata: NoiseSpecificationMetadata,

    /// Global approximation policy.
    ///
    /// Individual rules may override this policy.
    pub approximation: ApproximationPolicy,

    /// Ordered noise rules.
///
/// Rule order is retained intentionally because deterministic rule resolution
/// may depend on declared order when priorities are equal.
    pub rules: Vec<NoiseRule>,

    /// Global correlation policy.
    pub correlation: CorrelationSpec,

    /// Explicit semantic assumptions.
    pub assumptions: Vec<String>,

    /// Named extension declarations.
    pub extensions: Vec<NoiseExtension>,
}

impl NoiseSpecification {
    /// Creates an empty specification.
    ///
    /// An empty specification is valid and semantically means:
    ///
    /// > no additional noise is requested by this specification.
    ///
    /// It does not mean that a physical target is noiseless.
    ///
    /// Hardware-observed noise may still exist outside this specification.
    #[must_use]
    pub fn new() -> Self {
        Self {
            model_id: None,
            schema_version: String::from("1"),
            metadata: NoiseSpecificationMetadata::default(),
            approximation: ApproximationPolicy::Exact,
            rules: Vec::new(),
            correlation: CorrelationSpec::Independent,
            assumptions: Vec::new(),
            extensions: Vec::new(),
        }
    }

    /// Creates a specification with a caller-provided schema version.
    pub fn with_schema_version<V>(version: V) -> ZqnResult<Self>
    where
        V: Into<String>,
    {
        let version = version.into();

        if version.trim().is_empty() {
            return Err(invalid_spec(
                "noise specification schema version cannot be empty",
            ));
        }

        Ok(Self {
            schema_version: version,
            ..Self::new()
        })
    }

    /// Sets the model identity.
    #[must_use]
    pub fn with_model_id(mut self, model_id: NoiseModelId) -> Self {
        self.model_id = Some(model_id);
        self
    }

    /// Sets metadata.
    #[must_use]
    pub fn with_metadata(mut self, metadata: NoiseSpecificationMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Sets the global approximation policy.
    #[must_use]
    pub fn with_approximation(mut self, approximation: ApproximationPolicy) -> Self {
        self.approximation = approximation;
        self
    }

    /// Sets global correlation semantics.
    #[must_use]
    pub fn with_correlation(mut self, correlation: CorrelationSpec) -> Self {
        self.correlation = correlation;
        self
    }

    /// Adds one rule while preserving declaration order.
    pub fn add_rule(&mut self, rule: NoiseRule) -> ZqnResult<()> {
        rule.validate()?;
        self.rules.push(rule);
        Ok(())
    }

    /// Adds an extension.
    pub fn add_extension(&mut self, extension: NoiseExtension) -> ZqnResult<()> {
        if self
            .extensions
            .iter()
            .any(|existing| {
                existing.namespace == extension.namespace
                    && existing.kind == extension.kind
            })
        {
            return Err(invalid_spec(
                "duplicate noise extension namespace/kind",
            ));
        }

        self.extensions.push(extension);
        Ok(())
    }

    /// Adds a semantic assumption.
    pub fn add_assumption<A>(&mut self, assumption: A) -> ZqnResult<()>
    where
        A: Into<String>,
    {
        let assumption = assumption.into();

        if assumption.trim().is_empty() {
            return Err(invalid_spec(
                "noise specification assumption cannot be empty",
            ));
        }

        self.assumptions.push(assumption);
        Ok(())
    }

    /// Returns the number of explicitly declared rules.
    ///
    /// This is an observation, not a semantic limit.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Returns true when no explicit rules are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Performs complete structural validation that is possible without a
    /// target, calibration snapshot, or execution context.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.schema_version.trim().is_empty() {
            return Err(invalid_spec(
                "noise specification schema version cannot be empty",
            ));
        }

        self.metadata.validate()?;

        validate_correlation(&self.correlation)?;

        let mut names = std::collections::HashSet::with_capacity(self.rules.len());

        for rule in &self.rules {
            rule.validate()?;

            if !names.insert(rule.name.as_str()) {
                return Err(invalid_spec(
                    "noise specification contains duplicate rule names",
                ));
            }
        }

        for extension in &self.extensions {
            if extension.namespace.trim().is_empty()
                || extension.kind.trim().is_empty()
                || extension.version.trim().is_empty()
            {
                return Err(invalid_spec(
                    "noise specification contains an invalid extension",
                ));
            }
        }

        Ok(())
    }

    /// Returns rules sorted into deterministic evaluation order without
    /// mutating the declaration order.
    ///
    /// Ordering is:
    ///
    /// 1. descending semantic priority;
    /// 2. declaration order as the stable tie-breaker.
    ///
    /// The returned vector owns cloned rules so callers cannot accidentally
    /// mutate the specification while resolving it.
    #[must_use]
    pub fn rules_in_evaluation_order(&self) -> Vec<NoiseRule> {
        let mut indexed = self
            .rules
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<_>>();

        indexed.sort_by(|left, right| {
            right
                .1
                .priority
                .cmp(&left.1.priority)
                .then_with(|| left.0.cmp(&right.0))
        });

        indexed
            .into_iter()
            .map(|(_, rule)| rule)
            .collect()
    }

    /// Finds a rule by its stable declaration name.
    #[must_use]
    pub fn rule(&self, name: &str) -> Option<&NoiseRule> {
        self.rules.iter().find(|rule| rule.name == name)
    }
}

impl Default for NoiseSpecification {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_mechanism(mechanism: &NoiseMechanism) -> ZqnResult<()> {
    match mechanism {
        NoiseMechanism::BitFlip { probability }
        | NoiseMechanism::PhaseFlip { probability }
        | NoiseMechanism::Depolarizing { probability }
        | NoiseMechanism::AmplitudeDamping { probability }
        | NoiseMechanism::PhaseDamping { probability }
        | NoiseMechanism::Leakage { probability }
        | NoiseMechanism::Erasure { probability }
        | NoiseMechanism::Loss { probability }
        | NoiseMechanism::PreparationError { probability } => {
            if !probability.value().is_finite() {
                return Err(invalid_spec(
                    "noise mechanism probability must be finite",
                ));
            }
        }

        NoiseMechanism::ThermalRelaxation {
            relaxation_time,
            excitation_time,
        } => {
            validate_time_value(relaxation_time)?;

            if let Some(time) = excitation_time {
                validate_time_value(time)?;
            }
        }

        NoiseMechanism::Pauli { terms } => {
            for term in terms {
                if !term.probability.value().is_finite() {
                    return Err(invalid_spec(
                        "Pauli noise probability must be finite",
                    ));
                }
            }
        }

        NoiseMechanism::ChannelReference { channel } => {
            if channel.trim().is_empty() {
                return Err(invalid_spec(
                    "channel reference cannot be empty",
                ));
            }
        }

        NoiseMechanism::Stochastic { distribution } => {
            if distribution.name.trim().is_empty() {
                return Err(invalid_spec(
                    "distribution reference cannot be empty",
                ));
            }
        }

        NoiseMechanism::Lindblad { generator } => {
            if generator.name.trim().is_empty() {
                return Err(invalid_spec(
                    "generator reference cannot be empty",
                ));
            }
        }

        NoiseMechanism::Coherent {
            parameter,
            magnitude,
        } => {
            if parameter.trim().is_empty() {
                return Err(invalid_spec(
                    "coherent-error parameter cannot be empty",
                ));
            }

            if !magnitude.is_finite() {
                return Err(invalid_spec(
                    "coherent-error magnitude must be finite",
                ));
            }
        }

        NoiseMechanism::ReadoutAssignment { outcomes } => {
            for outcome in outcomes {
                if outcome.outcome.trim().is_empty() {
                    return Err(invalid_spec(
                        "readout outcome cannot be empty",
                    ));
                }
            }
        }

        NoiseMechanism::Crosstalk { interaction } => {
            if interaction.name.trim().is_empty() {
                return Err(invalid_spec(
                    "crosstalk interaction reference cannot be empty",
                ));
            }
        }

        NoiseMechanism::Extension(extension) => {
            if extension.namespace.trim().is_empty()
                || extension.kind.trim().is_empty()
                || extension.version.trim().is_empty()
            {
                return Err(invalid_spec(
                    "noise extension identity cannot be empty",
                ));
            }
        }
    }

    Ok(())
}

fn validate_correlation(correlation: &CorrelationSpec) -> ZqnResult<()> {
    match correlation {
        CorrelationSpec::Independent
        | CorrelationSpec::FullyCorrelated => Ok(()),

        CorrelationSpec::Model(name)
        | CorrelationSpec::Spatial { model: name }
        | CorrelationSpec::Temporal { model: name }
        | CorrelationSpec::Spatiotemporal { model: name } => {
            if name.trim().is_empty() {
                Err(invalid_spec(
                    "correlation model name cannot be empty",
                ))
            } else {
                Ok(())
            }
        }

        CorrelationSpec::Resources(resources) => {
            if resources.is_empty() {
                return Err(invalid_spec(
                    "correlation resource collection cannot be empty",
                ));
            }

            if resources.iter().any(NoiseScope::is_explicitly_empty) {
                return Err(invalid_spec(
                    "correlation contains an explicitly empty resource scope",
                ));
            }

            Ok(())
        }

        CorrelationSpec::Extension(extension) => {
            if extension.namespace.trim().is_empty()
                || extension.kind.trim().is_empty()
                || extension.version.trim().is_empty()
            {
                Err(invalid_spec(
                    "correlation extension identity cannot be empty",
                ))
            } else {
                Ok(())
            }
        }
    }
}

fn validate_condition(condition: &NoiseCondition) -> ZqnResult<()> {
    match condition {
        NoiseCondition::Always => Ok(()),

        NoiseCondition::Temporal(temporal) => {
            validate_temporal(temporal)
        }

        NoiseCondition::Calibration(value)
        | NoiseCondition::ExecutionContext(value) => {
            if value.trim().is_empty() {
                Err(invalid_spec(
                    "noise condition reference cannot be empty",
                ))
            } else {
                Ok(())
            }
        }

        NoiseCondition::ModelActive(_) => Ok(()),

        NoiseCondition::All(values)
        | NoiseCondition::Any(values) => {
            if values.is_empty() {
                return Err(invalid_spec(
                    "compound noise condition cannot be empty",
                ));
            }

            for value in values {
                validate_condition(value)?;
            }

            Ok(())
        }

        NoiseCondition::Not(value) => validate_condition(value),
    }
}

fn validate_temporal(temporal: &TemporalScope) -> ZqnResult<()> {
    match temporal {
        TemporalScope::Always => Ok(()),

        TemporalScope::Interval { start, end } => {
            validate_time_value(start)?;
            validate_time_value(end)?;

            if start.unit != end.unit {
                return Err(invalid_spec(
                    "temporal interval endpoints must use the same unit",
                ));
            }

            if start.value >= end.value {
                return Err(invalid_spec(
                    "temporal interval start must precede its end",
                ));
            }

            Ok(())
        }

        TemporalScope::At(time) => validate_time_value(time),

        TemporalScope::Phase(phase) => {
            if phase.trim().is_empty() {
                Err(invalid_spec(
                    "temporal phase cannot be empty",
                ))
            } else {
                Ok(())
            }
        }

        TemporalScope::Predicate(predicate) => {
            match predicate {
                TemporalPredicate::ParameterChanged(value)
                | TemporalPredicate::Phase(value)
                | TemporalPredicate::Custom(value) => {
                    if value.trim().is_empty() {
                        return Err(invalid_spec(
                            "temporal predicate value cannot be empty",
                        ));
                    }
                }

                TemporalPredicate::CalibrationValid
                | TemporalPredicate::CalibrationInvalid => {}
            }

            Ok(())
        }
    }
}

fn validate_time_value(time: &TimeValue) -> ZqnResult<()> {
    if !time.value.is_finite() || time.value < 0.0 {
        return Err(invalid_spec(
            "time value must be finite and non-negative",
        ));
    }

    Ok(())
}

// ============================================================================
// Error boundary
// ============================================================================

fn invalid_spec(message: &str) -> ZqnError {
    ZqnError::invalid_noise_specification(message)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_accepts_valid_values() {
        assert_eq!(
            NoiseProbability::new(0.0)
                .expect("zero probability should be valid")
                .value(),
            0.0
        );

        assert_eq!(
            NoiseProbability::new(1.0)
                .expect("unit probability should be valid")
                .value(),
            1.0
        );
    }

    #[test]
    fn probability_rejects_invalid_values() {
        assert!(NoiseProbability::new(-0.1).is_err());
        assert!(NoiseProbability::new(1.1).is_err());
        assert!(NoiseProbability::new(f64::NAN).is_err());
        assert!(NoiseProbability::new(f64::INFINITY).is_err());
    }

    #[test]
    fn tolerance_rejects_invalid_values() {
        assert!(NoiseTolerance::new(-1.0).is_err());
        assert!(NoiseTolerance::new(f64::NAN).is_err());
        assert!(NoiseTolerance::new(f64::INFINITY).is_err());
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_ir_identity() {
        let qubit = QubitId::new(7);
        let scope = NoiseScope::logical_qubit(qubit);

        assert_eq!(scope, NoiseScope::LogicalQubit(qubit));
    }

    #[test]
    fn physical_qubit_scope_uses_canonical_ir_identity() {
        let qubit = PhysicalQubitId::new(11);
        let scope = NoiseScope::physical_qubit(qubit);

        assert_eq!(scope, NoiseScope::PhysicalQubit(qubit));
    }

    #[test]
    fn empty_explicit_scope_is_rejected() {
        let rule = NoiseRule::new(
            "empty",
            NoiseScope::LogicalQubits(Vec::new()),
            OperationScope::Any,
            NoiseMechanism::BitFlip {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("rule construction itself should succeed");

        assert!(rule.validate().is_err());
    }

    #[test]
    fn rule_requires_non_empty_name() {
        let result = NoiseRule::new(
            "",
            NoiseScope::Global,
            OperationScope::Any,
            NoiseMechanism::BitFlip {
                probability: NoiseProbability::zero(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rule_can_be_composed_without_fixed_arity() {
        let scope = NoiseScope::logical_qubits(
            (0_u64..1024_u64).map(QubitId::new),
        );

        let rule = NoiseRule::new(
            "large-domain",
            scope,
            OperationScope::Any,
            NoiseMechanism::Depolarizing {
                probability: NoiseProbability::new(0.001)
                    .expect("valid probability"),
            },
        )
        .expect("rule should be constructible");

        assert!(rule.validate().is_ok());
    }

    #[test]
    fn empty_specification_is_valid() {
        let specification = NoiseSpecification::new();

        assert!(specification.validate().is_ok());
        assert!(specification.is_empty());
    }

    #[test]
    fn duplicate_rule_names_are_rejected() {
        let mut specification = NoiseSpecification::new();

        let first = NoiseRule::new(
            "same",
            NoiseScope::Global,
            OperationScope::Any,
            NoiseMechanism::BitFlip {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule");

        let second = NoiseRule::new(
            "same",
            NoiseScope::Global,
            OperationScope::Measurement,
            NoiseMechanism::PhaseFlip {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule");

        specification
            .add_rule(first)
            .expect("first rule should be accepted");

        specification
            .add_rule(second)
            .expect("duplicate is rejected at validation time, not insertion time");

        assert!(specification.validate().is_err());
    }

    #[test]
    fn evaluation_order_is_deterministic() {
        let mut specification = NoiseSpecification::new();

        let low = NoiseRule::new(
            "low",
            NoiseScope::Global,
            OperationScope::Any,
            NoiseMechanism::BitFlip {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule")
        .with_priority(1);

        let high = NoiseRule::new(
            "high",
            NoiseScope::Global,
            OperationScope::Any,
            NoiseMechanism::PhaseFlip {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule")
        .with_priority(10);

        specification
            .add_rule(low)
            .expect("low rule should be accepted");

        specification
            .add_rule(high)
            .expect("high rule should be accepted");

        let ordered = specification.rules_in_evaluation_order();

        assert_eq!(ordered[0].name, "high");
        assert_eq!(ordered[1].name, "low");
    }

    #[test]
    fn temporal_interval_requires_ordered_endpoints() {
        let start =
            TimeValue::new(10.0, TimeUnit::Nanoseconds)
                .expect("valid time");

        let end =
            TimeValue::new(5.0, TimeUnit::Nanoseconds)
                .expect("valid time");

        let result = NoiseRule::new(
            "invalid-interval",
            NoiseScope::Global,
            OperationScope::Idle,
            NoiseMechanism::PhaseDamping {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule")
        .with_temporal(TemporalScope::Interval { start, end });

        assert!(result.validate().is_err());
    }

    #[test]
    fn temporal_interval_requires_matching_units() {
        let start =
            TimeValue::new(1.0, TimeUnit::Nanoseconds)
                .expect("valid time");

        let end =
            TimeValue::new(2.0, TimeUnit::Microseconds)
                .expect("valid time");

        let result = NoiseRule::new(
            "invalid-units",
            NoiseScope::Global,
            OperationScope::Idle,
            NoiseMechanism::PhaseDamping {
                probability: NoiseProbability::zero(),
            },
        )
        .expect("valid rule")
        .with_temporal(TemporalScope::Interval { start, end });

        assert!(result.validate().is_err());
    }

    #[test]
    fn extension_requires_identity() {
        assert!(
            NoiseExtension::new("", "kind", "1", "payload").is_err()
        );

        assert!(
            NoiseExtension::new("namespace", "", "1", "payload").is_err()
        );

        assert!(
            NoiseExtension::new("namespace", "kind", "", "payload").is_err()
        );
    }

    #[test]
    fn specification_can_represent_large_rule_sets() {
        let mut specification = NoiseSpecification::new();

        for index in 0_u64..4096_u64 {
            let rule = NoiseRule::new(
                format!("rule-{index}"),
                NoiseScope::LogicalQubit(QubitId::new(index)),
                OperationScope::Any,
                NoiseMechanism::Depolarizing {
                    probability: NoiseProbability::new(0.001)
                        .expect("valid probability"),
                },
            )
            .expect("rule should be valid");

            specification
                .add_rule(rule)
                .expect("rule should be accepted");
        }

        assert_eq!(specification.rule_count(), 4096);
        assert!(specification.validate().is_ok());
    }

    #[test]
    fn deterministic_rule_order_preserves_declaration_order_on_ties() {
        let mut specification = NoiseSpecification::new();

        for name in ["first", "second", "third"] {
            let rule = NoiseRule::new(
                name,
                NoiseScope::Global,
                OperationScope::Any,
                NoiseMechanism::BitFlip {
                    probability: NoiseProbability::zero(),
                },
            )
            .expect("valid rule");

            specification
                .add_rule(rule)
                .expect("rule should be accepted");
        }

        let ordered = specification.rules_in_evaluation_order();

        assert_eq!(ordered[0].name, "first");
        assert_eq!(ordered[1].name, "second");
        assert_eq!(ordered[2].name, "third");
    }

    #[test]
    fn readout_errors_are_validated() {
        let mechanism = NoiseMechanism::ReadoutAssignment {
            outcomes: vec![ReadoutError {
                outcome: String::from("0"),
                probability: NoiseProbability::new(0.02)
                    .expect("valid probability"),
            }],
        };

        let rule = NoiseRule::new(
            "readout",
            NoiseScope::Global,
            OperationScope::Measurement,
            mechanism,
        )
        .expect("valid rule");

        assert!(rule.validate().is_ok());
    }

    #[test]
    fn exact_semantics_are_default() {
        assert_eq!(
            ApproximationPolicy::default(),
            ApproximationPolicy::Exact
        );
    }

    #[test]
    fn empty_noise_specification_does_not_mean_physical_noiselessness() {
        let specification = NoiseSpecification::new();

        assert!(specification.rules.is_empty());
        assert!(specification.validate().is_ok());
    }
}