//! ZQN characterization protocol contracts.
//!
//! This module defines the backend-independent scientific contract for
//! quantum-noise characterization experiments.
//!
//! # Mission
//!
//! A characterization protocol answers:
//!
//! > "What experiment must be performed, what observations are required,
//! >    and what scientifically valid information can be inferred from those
//! >    observations?"
//!
//! This module deliberately does NOT implement:
//!
//! - quantum-circuit generation;
//! - quantum IR;
//! - hardware communication;
//! - simulator execution;
//! - random-number generation;
//! - statistical regression;
//! - confidence-interval mathematics;
//! - channel reconstruction;
//! - calibration storage;
//! - result persistence;
//! - provider/vendor APIs;
//! - QEC;
//! - routing;
//! - scheduling.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source / application
//!          │
//!          ▼
//!      quantum::ir
//!          │
//!          ▼
//!   ZQN characterization
//!          │
//!     ┌────┼──────────────┐
//!     ▼    ▼              ▼
//! protocol generator   requirements
//!     │
//!     ▼
//! execution contract
//!     │
//! ┌───┼───────────────┐
//! ▼   ▼               ▼
//! QPU simulator    emulator
//!     │
//!     ▼
//! observations
//!     │
//!     ▼
//! estimator / statistics
//!     │
//!     ▼
//! characterization result
//!     │
//!     ▼
//! ZQN noise / calibration
//! ```
//!
//! # Important ownership rule
//!
//! A protocol describes an experiment. It does not execute it.
//!
//! A protocol must therefore be usable with:
//!
//! - a real QPU;
//! - a CPU simulator;
//! - a GPU simulator;
//! - a distributed simulator;
//! - an emulator;
//! - a future quantum architecture.
//!
//! # Scalability
//!
//! There is deliberately no semantic maximum for:
//!
//! - number of characterized resources;
//! - number of repetitions;
//! - sequence length;
//! - number of protocol stages;
//! - number of observations;
//! - number of experiment instances;
//! - number of resources in a characterization scope.
//!
//! Resource limits are represented explicitly through
//! [`CharacterizationLimits`] and are supplied by the caller/execution
//! environment.
//!
//! A limit is a resource-governance decision, not a statement about the
//! capabilities of quantum computing.
//!
//! # Determinism
//!
//! Characterization protocols may require randomized experiment generation,
//! but this module does not own randomness.
//!
//! Randomness must be supplied through an explicit [`RandomnessContract`].
//! Implementations must never use a hidden global RNG.
//!
//! # Canonical quantum identities
//!
//! This module intentionally does not redefine `QubitId` or
//! `PhysicalQubitId`.
//!
//! When a concrete characterization implementation needs to identify a
//! quantum resource, it must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! or the appropriate canonical IR resource identity for non-qubit
//! modalities.
//!
//! This keeps ZQN from creating a second quantum identity system.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no `unsafe`.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! A future concrete protocol should implement [`CharacterizationProtocol`].
//!
//! ```text
//! CharacterizationProtocol
//!          │
//!          ├── descriptor()
//!          ├── validate()
//!          ├── requirements()
//!          ├── plan()
//!          └── analyze()
//! ```
//!
//! The generator/execution/statistics layers consume those contracts without
//! needing to know the concrete protocol's implementation.
//!
//! # Versioning
//!
//! Protocol identifiers and semantic versions are data owned by individual
//! protocol implementations. This file only defines the stable contract
//! needed to consume them.
//!
//! # Security
//!
//! Implementations must treat protocol configuration as potentially
//! untrusted input. In particular, callers must not be able to trigger
//! unbounded allocation or execution merely by supplying extremely large
//! experiment parameters.
//!
//! Implementations must validate resource requirements before materializing
//! workloads.

use core::fmt;

/// Stable result type for this module.
///
/// The concrete ZQN error hierarchy can wrap or convert this error without
/// requiring this module to depend on the rest of ZQN's implementation
/// hierarchy.
pub type ProtocolResult<T> = Result<T, ProtocolError>;

/// Errors produced while validating or constructing a characterization
/// protocol contract.
///
/// This is deliberately small and protocol-oriented. Concrete ZQN modules
/// may convert these errors into the repository's canonical ZQN error type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtocolError {
    /// The supplied protocol identifier is empty or otherwise invalid.
    InvalidProtocolId,

    /// A semantic protocol version is invalid.
    InvalidVersion,

    /// A configuration parameter is invalid.
    InvalidConfiguration,

    /// A requested resource count or workload size cannot be represented.
    InvalidResourceCount,

    /// A required capability is not available.
    UnsupportedCapability,

    /// The requested experiment exceeds caller-provided resource limits.
    ResourceLimitExceeded,

    /// The protocol cannot be represented by the selected execution model.
    IncompatibleExecutionModel,

    /// A protocol stage is invalid.
    InvalidStage,

    /// A protocol scope is invalid.
    InvalidScope,

    /// The requested statistical/observation semantics are insufficient.
    InsufficientObservations,

    /// A deterministic execution contract is incomplete.
    MissingDeterminismContract,

    /// The supplied protocol contract is internally inconsistent.
    InconsistentContract,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidProtocolId => "invalid characterization protocol identifier",
            Self::InvalidVersion => "invalid characterization protocol version",
            Self::InvalidConfiguration => "invalid characterization configuration",
            Self::InvalidResourceCount => "invalid characterization resource count",
            Self::UnsupportedCapability => {
                "required characterization capability is unsupported"
            }
            Self::ResourceLimitExceeded => {
                "characterization resource limit would be exceeded"
            }
            Self::IncompatibleExecutionModel => {
                "characterization protocol is incompatible with execution model"
            }
            Self::InvalidStage => "invalid characterization protocol stage",
            Self::InvalidScope => "invalid characterization scope",
            Self::InsufficientObservations => {
                "characterization requires more observations"
            }
            Self::MissingDeterminismContract => {
                "characterization requires an explicit determinism contract"
            }
            Self::InconsistentContract => {
                "characterization protocol contract is internally inconsistent"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ProtocolError {}

// ============================================================================
// Stable protocol identity
// ============================================================================

/// Stable semantic identity of a characterization protocol.
///
/// The identifier is intentionally an owned `String` rather than an enum.
///
/// This is essential for extensibility:
///
/// ```text
/// built-in protocol
///       │
///       ├── custom research protocol
///       ├── vendor-neutral protocol
///       ├── future quantum technology protocol
///       └── user-defined Zamani protocol
/// ```
///
/// None of those require editing this file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProtocolId(String);

impl ProtocolId {
    /// Creates a protocol identifier after validating it.
    pub fn new<S>(value: S) -> ProtocolResult<Self>
    where
        S: Into<String>,
    {
        let value = value.into();

        if value.is_empty() || !is_valid_identifier(&value) {
            return Err(ProtocolError::InvalidProtocolId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned representation.
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ProtocolId {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ProtocolId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validates a protocol identifier without imposing a fixed protocol
/// vocabulary.
///
/// Allowed characters:
///
/// - ASCII letters;
/// - ASCII digits;
/// - `_`;
/// - `-`;
/// - `.`.
///
/// The identifier must begin with an ASCII letter.
fn is_valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();

    match characters.next() {
        Some(first) if first.is_ascii_alphabetic() => {}
        _ => return false,
    }

    characters.all(|character| {
        character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.'
    })
}

// ============================================================================
// Semantic version
// ============================================================================

/// Protocol semantic version.
///
/// This deliberately avoids depending on an external semantic-version crate
/// for the core characterization contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProtocolVersion {
    /// Major semantic version.
    pub major: u32,

    /// Minor semantic version.
    pub minor: u32,

    /// Patch semantic version.
    pub patch: u32,
}

impl ProtocolVersion {
    /// Creates a semantic version.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the initial stable contract version.
    pub const fn contract() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// ============================================================================
// Scientific scope
// ============================================================================

/// Abstract scope of resources being characterized.
///
/// This is deliberately not tied to a particular quantum modality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterizationScope {
    /// One logical or physical resource.
    Single,

    /// A finite collection whose membership is explicitly supplied by the
    /// caller.
    Explicit,

    /// A generated collection selected by the execution environment.
    Generated,

    /// A complete target-defined scope.
    TargetDefined,

    /// A distributed scope spanning multiple execution domains.
    Distributed,
}

impl CharacterizationScope {
    /// Returns whether the scope can be constructed without a fixed
    /// compile-time resource count.
    #[inline]
    pub const fn is_dynamic(&self) -> bool {
        !matches!(self, Self::Single | Self::Explicit)
    }
}

// ============================================================================
// Experimental objective
// ============================================================================

/// Scientific objective of a characterization protocol.
///
/// This is descriptive rather than prescriptive. A concrete protocol may
/// define additional objective semantics outside this enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterizationObjective {
    /// Estimate a stochastic error process.
    NoiseModel,

    /// Estimate calibration parameters.
    Calibration,

    /// Estimate temporal coherence/decoherence properties.
    Coherence,

    /// Estimate gate/process behavior.
    Process,

    /// Estimate measurement/preparation behavior.
    SpAM,

    /// Estimate interactions between simultaneously active resources.
    Crosstalk,

    /// Estimate temporal stability or drift.
    Drift,

    /// Reconstruct a quantum state or process.
    Tomography,

    /// Estimate a protocol-specific quantity not covered above.
    Custom,
}

// ============================================================================
// Protocol descriptor
// ============================================================================

/// Immutable description of a characterization protocol.
///
/// A descriptor is metadata. It contains no execution state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolDescriptor {
    /// Stable protocol identifier.
    pub id: ProtocolId,

    /// Protocol semantic version.
    pub version: ProtocolVersion,

    /// Scientific objective.
    pub objective: CharacterizationObjective,

    /// Resource scope semantics.
    pub scope: CharacterizationScope,

    /// Human-readable protocol name.
    pub name: String,

    /// Human-readable description.
    pub description: String,
}

impl ProtocolDescriptor {
    /// Validates descriptor invariants.
    pub fn validate(&self) -> ProtocolResult<()> {
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err(ProtocolError::InvalidConfiguration);
        }

        if self.id.as_str().is_empty() {
            return Err(ProtocolError::InvalidProtocolId);
        }

        Ok(())
    }
}

// ============================================================================
// Capabilities
// ============================================================================

/// Capabilities required by a characterization protocol.
///
/// The values describe requirements rather than target capabilities.
///
/// A hardware/simulator subsystem compares this structure with its own
/// capability descriptor.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterizationRequirements {
    /// Requires quantum-resource addressing.
    pub resource_addressing: bool,

    /// Requires state preparation.
    pub state_preparation: bool,

    /// Requires measurement.
    pub measurement: bool,

    /// Requires repeated execution/shots.
    pub repeated_execution: bool,

    /// Requires mid-circuit measurement.
    pub mid_circuit_measurement: bool,

    /// Requires reset.
    pub reset: bool,

    /// Requires timing information.
    pub timing: bool,

    /// Requires calibration information.
    pub calibration: bool,

    /// Requires simultaneous resource activation.
    pub simultaneous_execution: bool,

    /// Requires access to ideal/reference probabilities.
    pub reference_probabilities: bool,

    /// Requires process/state information unavailable from ordinary
    /// measurement-only execution.
    pub process_access: bool,

    /// Requires dynamic circuits.
    pub dynamic_circuits: bool,
}

impl CharacterizationRequirements {
    /// Returns whether two requirement sets can be merged without losing a
    /// requirement.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            resource_addressing: self.resource_addressing || other.resource_addressing,
            state_preparation: self.state_preparation || other.state_preparation,
            measurement: self.measurement || other.measurement,
            repeated_execution: self.repeated_execution || other.repeated_execution,
            mid_circuit_measurement: self.mid_circuit_measurement
                || other.mid_circuit_measurement,
            reset: self.reset || other.reset,
            timing: self.timing || other.timing,
            calibration: self.calibration || other.calibration,
            simultaneous_execution: self.simultaneous_execution
                || other.simultaneous_execution,
            reference_probabilities: self.reference_probabilities
                || other.reference_probabilities,
            process_access: self.process_access || other.process_access,
            dynamic_circuits: self.dynamic_circuits || other.dynamic_circuits,
        }
    }
}

// ============================================================================
// Resource governance
// ============================================================================

/// Caller-supplied resource limits.
///
/// These limits are deliberately optional.
///
/// `None` means that this layer imposes no additional limit; it does NOT mean
/// that the underlying system has infinite resources.
///
/// This distinction is essential for scaling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterizationLimits {
    /// Maximum number of materialized experiment instances.
    pub max_experiments: Option<u64>,

    /// Maximum number of repetitions per experiment.
    pub max_repetitions: Option<u64>,

    /// Maximum protocol depth/sequence length.
    pub max_sequence_length: Option<u64>,

    /// Maximum number of materialized observations.
    pub max_observations: Option<u64>,

    /// Maximum number of simultaneously materialized resources.
    pub max_materialized_resources: Option<u64>,

    /// Maximum estimated execution operations.
    pub max_execution_operations: Option<u64>,

    /// Maximum estimated memory consumption in bytes.
    pub max_memory_bytes: Option<u64>,
}

impl CharacterizationLimits {
    /// Validates that every configured limit is positive.
    pub fn validate(&self) -> ProtocolResult<()> {
        for value in [
            self.max_experiments,
            self.max_repetitions,
            self.max_sequence_length,
            self.max_observations,
            self.max_materialized_resources,
            self.max_execution_operations,
            self.max_memory_bytes,
        ] {
            if matches!(value, Some(0)) {
                return Err(ProtocolError::InvalidConfiguration);
            }
        }

        Ok(())
    }

    /// Checks a bounded value against an optional limit.
    #[inline]
    pub fn permits(limit: Option<u64>, requested: u64) -> bool {
        match limit {
            Some(maximum) => requested <= maximum,
            None => true,
        }
    }
}

// ============================================================================
// Randomness / reproducibility
// ============================================================================

/// Explicit randomness provenance.
///
/// The protocol does not own an RNG. It declares whether randomized
/// generation is required and what reproducibility information must be
/// available.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RandomnessContract {
    /// Whether randomized experiment generation is required.
    pub required: bool,

    /// Stable domain used to derive deterministic substreams.
    pub domain: String,

    /// Whether replay must be possible.
    pub replayable: bool,
}

impl RandomnessContract {
    /// Creates a deterministic randomized protocol contract.
    pub fn deterministic<S>(domain: S) -> ProtocolResult<Self>
    where
        S: Into<String>,
    {
        let domain = domain.into();

        if domain.trim().is_empty() {
            return Err(ProtocolError::MissingDeterminismContract);
        }

        Ok(Self {
            required: true,
            domain,
            replayable: true,
        })
    }

    /// Creates a contract for a protocol that does not require randomness.
    pub fn deterministic_none() -> Self {
        Self {
            required: false,
            domain: String::new(),
            replayable: true,
        }
    }
}

// ============================================================================
// Workload model
// ============================================================================

/// Abstract experiment quantity.
///
/// Protocols express requested workload quantities using this structure
/// instead of forcing all protocols into a fixed number of shots/sequences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkloadQuantity {
    /// Number requested.
    pub count: u64,

    /// Semantic label, e.g. "sequence", "shot", "circuit", "setting".
    pub semantic: String,
}

impl WorkloadQuantity {
    /// Creates a workload quantity.
    pub fn new<S>(count: u64, semantic: S) -> ProtocolResult<Self>
    where
        S: Into<String>,
    {
        if count == 0 {
            return Err(ProtocolError::InvalidConfiguration);
        }

        let semantic = semantic.into();

        if semantic.trim().is_empty() {
            return Err(ProtocolError::InvalidConfiguration);
        }

        Ok(Self { count, semantic })
    }
}

// ============================================================================
// Experiment plan
// ============================================================================

/// Immutable high-level experiment plan.
///
/// This is intentionally not a circuit.
///
/// A downstream generator converts the plan into canonical
/// `quantum::ir` workloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExperimentPlan {
    /// Protocol identity.
    pub protocol: ProtocolDescriptor,

    /// Scientific scope.
    pub scope: CharacterizationScope,

    /// Required workload quantities.
    pub quantities: Vec<WorkloadQuantity>,

    /// Protocol requirements.
    pub requirements: CharacterizationRequirements,

    /// Resource limits to be honored by downstream materialization.
    pub limits: CharacterizationLimits,

    /// Randomness/reproducibility contract.
    pub randomness: RandomnessContract,
}

impl ExperimentPlan {
    /// Validates plan-level invariants without generating the workload.
    pub fn validate(&self) -> ProtocolResult<()> {
        self.protocol.validate()?;
        self.limits.validate()?;

        if self.quantities.is_empty() {
            return Err(ProtocolError::InvalidConfiguration);
        }

        if self.randomness.required && !self.randomness.replayable {
            return Err(ProtocolError::MissingDeterminismContract);
        }

        Ok(())
    }
}

// ============================================================================
// Observation requirements
// ============================================================================

/// Describes what the execution layer must preserve from an experiment.
///
/// This prevents protocol implementations from depending on undocumented
/// backend behavior.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObservationRequirements {
    /// Preserve raw measurement counts/results.
    pub raw_measurements: bool,

    /// Preserve per-shot observations where supported.
    pub per_shot_observations: bool,

    /// Preserve execution timing.
    pub timing: bool,

    /// Preserve resource identities.
    pub resource_identity: bool,

    /// Preserve experiment identity.
    pub experiment_identity: bool,

    /// Preserve calibration identity.
    pub calibration_identity: bool,

    /// Preserve backend/target identity.
    pub target_identity: bool,

    /// Preserve deterministic randomness provenance.
    pub randomness_provenance: bool,
}

// ============================================================================
// Characterization observation
// ============================================================================

/// Opaque observation identity.
///
/// Observations are normally produced by the execution subsystem. The
/// protocol contract only requires stable identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObservationId(pub u128);

/// Lightweight protocol observation metadata.
///
/// The raw quantum data remains owned by the observation subsystem. This
/// structure prevents ZQN protocols from inventing a second observation
/// storage system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationMetadata {
    /// Stable observation identity.
    pub id: ObservationId,

    /// Protocol identity.
    pub protocol: ProtocolId,

    /// Experiment index within the protocol execution.
    pub experiment_index: u64,

    /// Whether the observation is complete.
    pub complete: bool,
}

impl ObservationMetadata {
    /// Validates observation metadata.
    pub fn validate(&self) -> ProtocolResult<()> {
        if self.protocol.as_str().is_empty() {
            return Err(ProtocolError::InvalidProtocolId);
        }

        Ok(())
    }
}

// ============================================================================
// Analysis contract
// ============================================================================

/// Describes what a characterization protocol is expected to infer.
///
/// The estimator itself belongs to a downstream characterization estimator
/// module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalysisContract {
    /// Human-readable quantity being estimated.
    pub quantity: String,

    /// Whether uncertainty must accompany the estimate.
    pub requires_uncertainty: bool,

    /// Whether raw observations must remain available for independent
    /// re-analysis.
    pub requires_raw_observations: bool,

    /// Whether the result can be represented as a finite scalar.
    ///
    /// `false` is important for future protocols producing distributions,
    /// channels, tensors, curves, or other structured results.
    pub scalar_result: bool,
}

impl AnalysisContract {
    /// Validates the analysis contract.
    pub fn validate(&self) -> ProtocolResult<()> {
        if self.quantity.trim().is_empty() {
            return Err(ProtocolError::InvalidConfiguration);
        }

        Ok(())
    }
}

// ============================================================================
// Protocol trait
// ============================================================================

/// Backend-independent ZQN characterization protocol.
///
/// This trait defines the scientific lifecycle but does not prescribe how
/// circuits, pulses, analog controls, or other workloads are represented.
///
/// Implementations should remain independent of:
///
/// - hardware providers;
/// - simulator implementations;
/// - RNG implementations;
/// - statistical libraries;
/// - serialization formats.
///
/// # Lifecycle
///
/// ```text
/// descriptor
///     │
///     ▼
/// requirements
///     │
///     ▼
/// validate configuration
///     │
///     ▼
/// plan experiment
///     │
///     ▼
/// generator
///     │
///     ▼
/// execution
///     │
///     ▼
/// observations
///     │
///     ▼
/// estimator
/// ```
///
/// The trait intentionally returns metadata/contracts rather than owning
/// execution.
pub trait CharacterizationProtocol {
    /// Returns immutable protocol metadata.
    fn descriptor(&self) -> ProtocolDescriptor;

    /// Returns execution capabilities required by the protocol.
    fn requirements(&self) -> CharacterizationRequirements;

    /// Returns required observation semantics.
    fn observation_requirements(&self) -> ObservationRequirements;

    /// Returns the expected scientific analysis contract.
    fn analysis_contract(&self) -> AnalysisContract;

    /// Returns the randomness/reproducibility requirements.
    fn randomness_contract(&self) -> RandomnessContract;

    /// Validates a protocol configuration against caller-supplied resource
    /// limits.
    ///
    /// This method must not allocate an experiment workload.
    fn validate(
        &self,
        limits: &CharacterizationLimits,
    ) -> ProtocolResult<()>;

    /// Produces an abstract experiment plan.
    ///
    /// The plan is not a circuit and must not contain vendor-specific
    /// execution instructions.
    fn plan(
        &self,
        limits: &CharacterizationLimits,
    ) -> ProtocolResult<ExperimentPlan>;

    /// Returns a stable description of the protocol's analysis semantics.
    ///
    /// The actual estimator remains outside this contract.
    fn analysis(&self) -> AnalysisContract {
        self.analysis_contract()
    }
}

// ============================================================================
// Protocol validation helpers
// ============================================================================

/// Validates a protocol contract as a whole.
///
/// This function is useful for registry/discovery code and allows protocol
/// implementations to share invariant checks without coupling the registry
/// to concrete protocol types.
pub fn validate_protocol<P>(protocol: &P) -> ProtocolResult<()>
where
    P: CharacterizationProtocol + ?Sized,
{
    let descriptor = protocol.descriptor();
    descriptor.validate()?;

    let requirements = protocol.requirements();

    if requirements.process_access && requirements.measurement {
        // Process access and measurement are not inherently contradictory;
        // this branch intentionally does nothing. It exists as a semantic
        // reminder that capability negotiation must not reject valid hybrid
        // protocols merely because they require both.
    }

    let observations = protocol.observation_requirements();

    if observations.experiment_identity && !observations.resource_identity {
        // Experiment identity can be valid without resource identity for
        // target-defined aggregate characterization. No rejection is made.
    }

    protocol.analysis_contract().validate()?;

    let randomness = protocol.randomness_contract();

    if randomness.required && !randomness.replayable {
        return Err(ProtocolError::MissingDeterminismContract);
    }

    Ok(())
}

// ============================================================================
// Capability matching
// ============================================================================

/// Target/execution capabilities relevant to characterization.
///
/// This is intentionally separate from protocol requirements.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterizationCapabilities {
    /// Can address individual quantum resources.
    pub resource_addressing: bool,

    /// Can prepare states.
    pub state_preparation: bool,

    /// Can measure.
    pub measurement: bool,

    /// Supports repeated execution.
    pub repeated_execution: bool,

    /// Supports mid-circuit measurement.
    pub mid_circuit_measurement: bool,

    /// Supports reset.
    pub reset: bool,

    /// Provides timing.
    pub timing: bool,

    /// Provides calibration.
    pub calibration: bool,

    /// Supports simultaneous execution.
    pub simultaneous_execution: bool,

    /// Provides reference probabilities.
    pub reference_probabilities: bool,

    /// Provides process access.
    pub process_access: bool,

    /// Supports dynamic circuits.
    pub dynamic_circuits: bool,
}

impl CharacterizationCapabilities {
    /// Returns whether these capabilities satisfy the supplied requirements.
    pub fn satisfies(
        &self,
        requirements: &CharacterizationRequirements,
    ) -> bool {
        (!requirements.resource_addressing || self.resource_addressing)
            && (!requirements.state_preparation || self.state_preparation)
            && (!requirements.measurement || self.measurement)
            && (!requirements.repeated_execution || self.repeated_execution)
            && (!requirements.mid_circuit_measurement
                || self.mid_circuit_measurement)
            && (!requirements.reset || self.reset)
            && (!requirements.timing || self.timing)
            && (!requirements.calibration || self.calibration)
            && (!requirements.simultaneous_execution
                || self.simultaneous_execution)
            && (!requirements.reference_probabilities
                || self.reference_probabilities)
            && (!requirements.process_access || self.process_access)
            && (!requirements.dynamic_circuits || self.dynamic_circuits)
    }
}

// ============================================================================
// Protocol compatibility
// ============================================================================

/// Validates whether an execution target can satisfy a protocol.
pub fn validate_capabilities<P>(
    protocol: &P,
    capabilities: &CharacterizationCapabilities,
) -> ProtocolResult<()>
where
    P: CharacterizationProtocol + ?Sized,
{
    if !capabilities.satisfies(&protocol.requirements()) {
        return Err(ProtocolError::UnsupportedCapability);
    }

    Ok(())
}

// ============================================================================
// Workload validation
// ============================================================================

/// Validates a requested workload quantity against limits.
///
/// This helper performs checked arithmetic and never allocates.
pub fn validate_workload_quantity(
    quantity: &WorkloadQuantity,
    limits: &CharacterizationLimits,
) -> ProtocolResult<()> {
    if quantity.count == 0 {
        return Err(ProtocolError::InvalidConfiguration);
    }

    if !CharacterizationLimits::permits(
        limits.max_experiments,
        quantity.count,
    ) {
        return Err(ProtocolError::ResourceLimitExceeded);
    }

    Ok(())
}

/// Safely computes a product of workload dimensions.
///
/// This helper is useful when concrete protocols need to estimate total
/// experiment counts without risking integer overflow.
///
/// No fixed machine-size limit is introduced.
pub fn checked_workload_product(values: &[u64]) -> ProtocolResult<u64> {
    let mut product = 1_u64;

    for &value in values {
        if value == 0 {
            return Err(ProtocolError::InvalidConfiguration);
        }

        product = product
            .checked_mul(value)
            .ok_or(ProtocolError::InvalidResourceCount)?;
    }

    Ok(product)
}

// ============================================================================
// Protocol registry-neutral catalog abstraction
// ============================================================================

/// Metadata needed by a future protocol registry.
///
/// The registry remains outside this file.
///
/// This type is deliberately data-only so a registry can discover protocols
/// without depending on implementation details.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolCatalogEntry {
    /// Protocol descriptor.
    pub descriptor: ProtocolDescriptor,

    /// Required execution capabilities.
    pub requirements: CharacterizationRequirements,

    /// Observation contract.
    pub observations: ObservationRequirements,

    /// Analysis contract.
    pub analysis: AnalysisContract,
}

impl ProtocolCatalogEntry {
    /// Builds a registry-neutral catalog entry.
    pub fn from_protocol<P>(protocol: &P) -> ProtocolResult<Self>
    where
        P: CharacterizationProtocol + ?Sized,
    {
        validate_protocol(protocol)?;

        Ok(Self {
            descriptor: protocol.descriptor(),
            requirements: protocol.requirements(),
            observations: protocol.observation_requirements(),
            analysis: protocol.analysis_contract(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleProtocol;

    impl CharacterizationProtocol for ExampleProtocol {
        fn descriptor(&self) -> ProtocolDescriptor {
            ProtocolDescriptor {
                id: ProtocolId::new("example.noise").expect("valid test ID"),
                version: ProtocolVersion::new(1, 0, 0),
                objective: CharacterizationObjective::NoiseModel,
                scope: CharacterizationScope::Generated,
                name: String::from("Example Noise Characterization"),
                description: String::from(
                    "Test protocol demonstrating the ZQN contract.",
                ),
            }
        }

        fn requirements(&self) -> CharacterizationRequirements {
            CharacterizationRequirements {
                resource_addressing: true,
                state_preparation: true,
                measurement: true,
                repeated_execution: true,
                ..CharacterizationRequirements::default()
            }
        }

        fn observation_requirements(&self) -> ObservationRequirements {
            ObservationRequirements {
                raw_measurements: true,
                experiment_identity: true,
                resource_identity: true,
                randomness_provenance: true,
                ..ObservationRequirements::default()
            }
        }

        fn analysis_contract(&self) -> AnalysisContract {
            AnalysisContract {
                quantity: String::from("noise model"),
                requires_uncertainty: true,
                requires_raw_observations: true,
                scalar_result: false,
            }
        }

        fn randomness_contract(&self) -> RandomnessContract {
            RandomnessContract::deterministic("zqn.example")
                .expect("valid deterministic domain")
        }

        fn validate(
            &self,
            limits: &CharacterizationLimits,
        ) -> ProtocolResult<()> {
            limits.validate()
        }

        fn plan(
            &self,
            limits: &CharacterizationLimits,
        ) -> ProtocolResult<ExperimentPlan> {
            self.validate(limits)?;

            let descriptor = self.descriptor();
            let requirements = self.requirements();

            let quantities = vec![
                WorkloadQuantity::new(1, "experiment")?,
            ];

            let plan = ExperimentPlan {
                protocol: descriptor,
                scope: CharacterizationScope::Generated,
                quantities,
                requirements,
                limits: limits.clone(),
                randomness: self.randomness_contract(),
            };

            plan.validate()?;

            Ok(plan)
        }
    }

    #[test]
    fn protocol_identifier_is_extensible() {
        let id = ProtocolId::new("future.quantum.protocol")
            .expect("identifier should be accepted");

        assert_eq!(id.as_str(), "future.quantum.protocol");
    }

    #[test]
    fn invalid_protocol_identifier_is_rejected() {
        assert_eq!(
            ProtocolId::new(""),
            Err(ProtocolError::InvalidProtocolId)
        );

        assert_eq!(
            ProtocolId::new("123-invalid"),
            Err(ProtocolError::InvalidProtocolId)
        );
    }

    #[test]
    fn protocol_version_is_stable() {
        assert_eq!(
            ProtocolVersion::contract(),
            ProtocolVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn zero_limits_are_rejected() {
        let limits = CharacterizationLimits {
            max_experiments: Some(0),
            ..CharacterizationLimits::default()
        };

        assert_eq!(
            limits.validate(),
            Err(ProtocolError::InvalidConfiguration)
        );
    }

    #[test]
    fn absent_limits_do_not_create_artificial_capacity_limits() {
        assert!(CharacterizationLimits::permits(None, u64::MAX));
    }

    #[test]
    fn bounded_limits_are_enforced() {
        assert!(CharacterizationLimits::permits(Some(10), 10));
        assert!(!CharacterizationLimits::permits(Some(10), 11));
    }

    #[test]
    fn workload_product_uses_checked_arithmetic() {
        assert_eq!(
            checked_workload_product(&[2, 3, 4]).expect("valid product"),
            24
        );
    }

    #[test]
    fn workload_overflow_is_rejected() {
        assert_eq!(
            checked_workload_product(&[u64::MAX, 2]),
            Err(ProtocolError::InvalidResourceCount)
        );
    }

    #[test]
    fn capabilities_are_negotiated_explicitly() {
        let protocol = ExampleProtocol;

        let capabilities = CharacterizationCapabilities {
            resource_addressing: true,
            state_preparation: true,
            measurement: true,
            repeated_execution: true,
            ..CharacterizationCapabilities::default()
        };

        assert!(
            validate_capabilities(&protocol, &capabilities).is_ok()
        );
    }

    #[test]
    fn missing_capability_is_rejected() {
        let protocol = ExampleProtocol;

        let capabilities = CharacterizationCapabilities::default();

        assert_eq!(
            validate_capabilities(&protocol, &capabilities),
            Err(ProtocolError::UnsupportedCapability)
        );
    }

    #[test]
    fn protocol_contract_is_valid() {
        let protocol = ExampleProtocol;

        assert!(validate_protocol(&protocol).is_ok());
    }

    #[test]
    fn plan_is_backend_neutral() {
        let protocol = ExampleProtocol;

        let limits = CharacterizationLimits::default();

        let plan = protocol.plan(&limits).expect("valid plan");

        assert_eq!(
            plan.protocol.id.as_str(),
            "example.noise"
        );
    }

    #[test]
    fn catalog_entry_is_registry_neutral() {
        let protocol = ExampleProtocol;

        let entry =
            ProtocolCatalogEntry::from_protocol(&protocol)
                .expect("valid catalog entry");

        assert_eq!(
            entry.descriptor.id.as_str(),
            "example.noise"
        );
    }

    #[test]
    fn randomized_protocol_requires_replayability() {
        let contract = RandomnessContract {
            required: true,
            domain: String::from("test"),
            replayable: false,
        };

        let protocol = ExampleProtocol;

        struct NonReplayableProtocol(RandomnessContract);

        impl CharacterizationProtocol for NonReplayableProtocol {
            fn descriptor(&self) -> ProtocolDescriptor {
                protocol_descriptor("nonreplayable", "test")
            }

            fn requirements(&self) -> CharacterizationRequirements {
                CharacterizationRequirements::default()
            }

            fn observation_requirements(&self) -> ObservationRequirements {
                ObservationRequirements::default()
            }

            fn analysis_contract(&self) -> AnalysisContract {
                AnalysisContract {
                    quantity: String::from("test"),
                    requires_uncertainty: false,
                    requires_raw_observations: true,
                    scalar_result: true,
                }
            }

            fn randomness_contract(&self) -> RandomnessContract {
                contract.clone()
            }

            fn validate(
                &self,
                _limits: &CharacterizationLimits,
            ) -> ProtocolResult<()> {
                Ok(())
            }

            fn plan(
                &self,
                _limits: &CharacterizationLimits,
            ) -> ProtocolResult<ExperimentPlan> {
                Err(ProtocolError::MissingDeterminismContract)
            }
        }

        assert_eq!(
            validate_protocol(&NonReplayableProtocol(contract)),
            Err(ProtocolError::MissingDeterminismContract)
        );
    }

    fn protocol_descriptor(
        id: &str,
        description: &str,
    ) -> ProtocolDescriptor {
        ProtocolDescriptor {
            id: ProtocolId::new(id).expect("valid test ID"),
            version: ProtocolVersion::contract(),
            objective: CharacterizationObjective::Custom,
            scope: CharacterizationScope::Generated,
            name: String::from("Test"),
            description: String::from(description),
        }
    }
}