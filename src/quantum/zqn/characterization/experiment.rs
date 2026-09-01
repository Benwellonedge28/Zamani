//! Zamani Quantum Noise (ZQN) — Characterization Experiment Contract.
//!
//! # Purpose
//!
//! This module defines the canonical, backend-independent description of a
//! quantum-noise characterization experiment.
//!
//! A `CharacterizationExperiment` answers:
//!
//! > "What characterization work is being requested, against which canonical
//! > quantum resources/circuits, with what acquisition policy and
//! > reproducibility requirements?"
//!
//! It does NOT answer:
//!
//! - how the experiment is executed;
//! - how circuits are generated;
//! - how observations are statistically estimated;
//! - how a noise model is fitted;
//! - how a QPU is selected;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how calibration is changed;
//! - how a vendor API is called;
//! - how raw observations are stored;
//! - how a characterization result is serialized.
//!
//! Those responsibilities belong to the corresponding ZQN and surrounding
//! quantum subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir                         canonical semantics
//!      │
//!      ├───────────────────────────────┐
//!      │                               │
//!      ▼                               ▼
//! compiler transformations             ZQN
//!                                      │
//!                                      ▼
//!                         characterization::experiment
//!                                      │
//!                       experiment definition only
//!                                      │
//!                  ┌───────────────────┼───────────────────┐
//!                  ▼                   ▼                   ▼
//!               protocol            execution          observation
//!                  │                   │                   │
//!                  ▼                   ▼                   ▼
//!              estimator         hardware/runtime      analysis
//!                  │                                       │
//!                  └───────────────────┬───────────────────┘
//!                                      ▼
//!                               characterization result
//!                                      │
//!                                      ▼
//!                                calibration/noise
//! ```
//!
//! # Canonical quantum identity
//!
//! ZQN MUST use the canonical quantum-resource identity types owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module therefore does not define:
//!
//! ```text
//! ZqnQubitId
//! ZqnPhysicalQubitId
//! ```
//!
//! A logical target is represented by the canonical `QubitId`.
//!
//! A physical target is represented by the canonical `PhysicalQubitId`.
//!
//! This preserves the repository-wide rule that ZQN is not a second quantum
//! semantic IR.
//!
//! # Circuit identity
//!
//! Characterization experiments refer to canonical IR circuits through
//! `crate::quantum::ir::CircuitId`.
//!
//! This module does not embed a second circuit representation.
//!
//! The dependency is:
//!
//! ```text
//! QuantumCircuit
//!      │
//!      ▼
//! CircuitId
//!      │
//!      ▼
//! CharacterizationExperiment
//! ```
//!
//! The experiment therefore remains lightweight and can describe very large
//! computations without duplicating the entire circuit in the experiment
//! definition.
//!
//! # Write once, scale everywhere
//!
//! No semantic machine-size limit is encoded here.
//!
//! In particular, this module does not define:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_EXPERIMENTS
//! MAX_CIRCUITS
//! MAX_SHOTS
//! MAX_REPETITIONS
//! MAX_DEPTH
//! ```
//!
//! Counts use ordinary finite integer representations because an individual
//! execution must ultimately be representable by the host/runtime/storage
//! environment. Those integer types are representation boundaries, not claims
//! about a fixed maximum supported machine size.
//!
//! The architectural meaning of "infinity" is:
//!
//! > ZQN does not impose an artificial finite machine-size ceiling.
//!
//! Actual operational limits are supplied by:
//!
//! - runtime policy;
//! - `zqn::core::limits`;
//! - hardware capabilities;
//! - simulator capabilities;
//! - storage capacity;
//! - execution policy;
//! - user policy;
//! - security policy.
//!
//! # Program portability
//!
//! A characterization experiment must not contain:
//!
//! - vendor identifiers as semantic requirements;
//! - backend SDK handles;
//! - credentials;
//! - physical topology assumptions;
//! - fixed qubit counts;
//! - fixed gate sets;
//! - hard-coded device names;
//! - hard-coded simulator implementations.
//!
//! A target/backend may later determine whether the experiment is executable.
//!
//! The intended relationship is:
//!
//! ```text
//! CharacterizationExperiment
//!             │
//!             ▼
//! target::requirements / target::capabilities
//!             │
//!             ▼
//! compatibility validation
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//!     exact       explicit
//!   realization   approximation
//! ```
//!
//! An unsupported characterization requirement must never silently become a
//! different experiment.
//!
//! # Determinism
//!
//! This file owns the experiment's reproducibility *requirements*, not random
//! number generation.
//!
//! No global RNG is used.
//!
//! No wall-clock time is used to derive semantic identity.
//!
//! No process ID, memory address, thread ID, or hash-map iteration order is
//! used to define an experiment.
//!
//! If deterministic acquisition is requested, the caller supplies an explicit
//! seed and the downstream execution/sampling layer derives operation/resource
//! substreams from the experiment identity and execution context.
//!
//! # Provenance
//!
//! The experiment can carry references to provenance-relevant identities without
//! owning the provenance implementation.
//!
//! In particular, the experiment may identify:
//!
//! - characterization identity;
//! - experiment identity;
//! - circuit identity;
//! - calibration snapshot identity;
//! - target identity;
//! - protocol revision;
//! - requested source.
//!
//! Actual provenance construction belongs to `core::provenance` and the future
//! characterization result layer.
//!
//! # Separation from protocol.rs
//!
//! `experiment.rs` defines the *instance/request* boundary.
//!
//! `protocol.rs` should define the mathematical/procedural characterization
//! protocol.
//!
//! Therefore this file deliberately does not import `protocol.rs`.
//!
//! A future protocol can consume this stable contract:
//!
//! ```text
//! CharacterizationExperiment
//!          │
//!          ▼
//! CharacterizationProtocol
//!          │
//!          ▼
//! CharacterizationObservation
//! ```
//!
//! This prevents the experiment definition from being rewritten whenever a
//! new characterization protocol is introduced.
//!
//! # Separation from observation.rs
//!
//! Raw observations are execution output and therefore do not belong here.
//!
//! The dependency is:
//!
//! ```text
//! experiment.rs
//!      │
//!      ▼
//! execution
//!      │
//!      ▼
//! observation.rs
//! ```
//!
//! Never reverse that dependency.
//!
//! # Separation from estimator.rs
//!
//! Statistical estimation belongs to `estimator.rs`.
//!
//! `experiment.rs` may declare what kind of characterization information is
//! requested, but it must not perform fitting or estimation.
//!
//! # Separation from calibration
//!
//! An experiment may reference a calibration snapshot so that execution can be
//! reproducible against a known calibration state.
//!
//! It must not mutate calibration.
//!
//! Calibration ownership remains in:
//!
//! ```text
//! crate::quantum::zqn::calibration
//! ```
//!
//! # Separation from hardware
//!
//! No hardware API is imported here.
//!
//! A hardware adapter consumes the experiment after target compatibility
//! validation.
//!
//! # Error model
//!
//! Invalid experiment definitions return the canonical ZQN error model:
//!
//! ```text
//! crate::quantum::zqn::core::errors::ZqnError
//! crate::quantum::zqn::core::errors::ZqnResult
//! ```
//!
//! No competing characterization error hierarchy is introduced.
//!
//! # Resource safety
//!
//! This module does not allocate based on an attacker-controlled machine-size
//! declaration.
//!
//! Collections are caller-owned and ordinary Rust collections are used.
//!
//! Expensive work such as:
//!
//! - circuit materialization;
//! - observation storage;
//! - sampling;
//! - statistical estimation;
//! - tomography;
//! - fitting;
//!
//! belongs to downstream layers where `ZqnLimits` and cancellation policies can
//! be applied before expensive resources are consumed.
//!
//! # Serialization
//!
//! This file deliberately does not depend on a serialization framework.
//!
//! The future `zqn::io` layer may serialize this definition using a versioned
//! schema.
//!
//! Serialization must preserve:
//!
//! - experiment identity;
//! - characterization identity;
//! - circuit identity;
//! - resource scope;
//! - experiment kind;
//! - acquisition policy;
//! - reproducibility policy;
//! - calibration reference;
//! - protocol revision;
//! - metadata.
//!
//! Serialization must never reinterpret logical and physical qubit IDs as the
//! same identity domain.
//!
//! # Thread safety
//!
//! The types in this file are immutable value/configuration types once built.
//!
//! They contain no global state and no interior mutability.
//!
//! They are therefore suitable for concurrent use when placed inside
//! thread-safe surrounding containers.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! # Safety
//!
//! This file forbids unsafe Rust at compile time.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir
//!     │
//!     ├── CircuitId
//!     ├── QubitId
//!     └── PhysicalQubitId
//! ```
//!
//! ZQN core:
//!
//! ```text
//! core::ids
//! core::errors
//! ```
//!
//! Downstream:
//!
//! ```text
//! characterization::protocol
//! characterization::observation
//! characterization::estimator
//! calibration
//! simulation
//! target
//! integration::hardware
//! integration::runtime
//! benchmarking
//! ```
//!
//! No downstream implementation is required to construct the core experiment
//! contract.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. experiments have stable ZQN identity;
//! 2. canonical IR circuit/resource identities are used;
//! 3. no second quantum IR exists here;
//! 4. no execution implementation exists here;
//! 5. no statistical estimator exists here;
//! 6. no protocol implementation exists here;
//! 7. no hardware/vendor dependency exists here;
//! 8. acquisition policy is explicit;
//! 9. reproducibility policy is explicit;
//! 10. calibration references are explicit;
//! 11. resource scope is extensible;
//! 12. no fixed machine-size limit is encoded;
//! 13. invalid definitions are rejected deterministically;
//! 14. the canonical ZQN error model is used;
//! 15. the module is safe Rust;
//! 16. the API can be consumed by future protocol/observation/estimation files
//!     without modification to this file.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::CircuitId;
use crate::quantum::zqn::core::errors::{ZqnError, ZqnErrorCode, ZqnErrorKind, ZqnResult};
use crate::quantum::zqn::core::ids::{CalibrationId, CharacterizationId, ExperimentId};

// =============================================================================
// Experiment kind
// =============================================================================

/// Broad semantic purpose of a characterization experiment.
///
/// This enum intentionally describes *what is being characterized*, not how
/// the experiment is implemented.
///
/// Concrete protocols belong to `characterization::protocol`.
///
/// `Custom` allows future characterization domains to be represented without
/// requiring this foundational experiment contract to be changed merely because
/// a new quantum technology or protocol is introduced.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CharacterizationKind {
    /// Characterize a quantum operation or gate process.
    Operation,

    /// Characterize state preparation.
    StatePreparation,

    /// Characterize reset behavior.
    Reset,

    /// Characterize measurement/readout behavior.
    Measurement,

    /// Characterize idle/evolution behavior.
    Idle,

    /// Characterize transport/motion behavior.
    Transport,

    /// Characterize a general quantum channel/process.
    Channel,

    /// Characterize correlations between resources or operations.
    Correlation,

    /// Characterize temporal drift or time dependence.
    Temporal,

    /// Characterize crosstalk between concurrently active resources.
    Crosstalk,

    /// Characterize leakage/loss/erasure behavior.
    LeakageLossErasure,

    /// Characterize a complete physical or logical subsystem.
    System,

    /// Characterization domain supplied by an extension.
    ///
    /// The string is semantic extension metadata, not a vendor identifier.
    Custom(String),
}

impl CharacterizationKind {
    /// Returns the stable semantic name of the characterization kind.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Operation => "operation",
            Self::StatePreparation => "state_preparation",
            Self::Reset => "reset",
            Self::Measurement => "measurement",
            Self::Idle => "idle",
            Self::Transport => "transport",
            Self::Channel => "channel",
            Self::Correlation => "correlation",
            Self::Temporal => "temporal",
            Self::Crosstalk => "crosstalk",
            Self::LeakageLossErasure => "leakage_loss_erasure",
            Self::System => "system",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Creates an extension-defined characterization kind.
    ///
    /// The extension name must be non-empty and may not contain control
    /// characters.
    pub fn custom(value: impl Into<String>) -> ZqnResult<Self> {
        let value = value.into();
        validate_text_identifier("characterization kind", &value)?;
        Ok(Self::Custom(value))
    }
}

impl fmt::Display for CharacterizationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Experiment target resources
// =============================================================================

/// Canonical quantum-resource scope for a characterization experiment.
///
/// The scope uses the canonical Quantum IR identity types directly.
///
/// It deliberately supports both logical and physical resources because
/// characterization can occur before or after physical realization.
///
/// No assumption is made about the number of resources in a scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CharacterizationResource {
    /// One canonical logical qubit.
    LogicalQubit(QubitId),

    /// Multiple canonical logical qubits.
    LogicalQubits(Vec<QubitId>),

    /// One canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Multiple canonical physical qubits.
    PhysicalQubits(Vec<PhysicalQubitId>),

    /// A canonical logical circuit.
    Circuit(CircuitId),

    /// An extension-defined resource reference.
    ///
    /// This is intentionally textual and opaque to ZQN. Hardware/resource
    /// resolution belongs to the target subsystem.
    Extension(String),
}

impl CharacterizationResource {
    /// Creates a logical-qubit resource reference.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit resource reference.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a logical-qubit set reference.
    ///
    /// Empty sets are rejected by `CharacterizationExperiment::validate`.
    #[must_use]
    pub fn logical_qubits(qubits: Vec<QubitId>) -> Self {
        Self::LogicalQubits(qubits)
    }

    /// Creates a physical-qubit set reference.
    ///
    /// Empty sets are rejected by `CharacterizationExperiment::validate`.
    #[must_use]
    pub fn physical_qubits(qubits: Vec<PhysicalQubitId>) -> Self {
        Self::PhysicalQubits(qubits)
    }

    /// Creates a circuit resource reference.
    #[must_use]
    pub const fn circuit(circuit: CircuitId) -> Self {
        Self::Circuit(circuit)
    }

    /// Creates an extension-defined resource reference.
    pub fn extension(value: impl Into<String>) -> ZqnResult<Self> {
        let value = value.into();
        validate_text_identifier("resource extension", &value)?;
        Ok(Self::Extension(value))
    }

    /// Returns the logical qubit when this is a single logical-qubit target.
    #[must_use]
    pub const fn as_logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(*qubit),
            _ => None,
        }
    }

    /// Returns the physical qubit when this is a single physical-qubit target.
    #[must_use]
    pub const fn as_physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(qubit) => Some(*qubit),
            _ => None,
        }
    }

    /// Returns true if the resource contains at least one concrete quantum
    /// resource.
    ///
    /// A circuit reference is also considered a concrete experiment resource.
    #[must_use]
    pub fn is_non_empty(&self) -> bool {
        match self {
            Self::LogicalQubit(_) => true,
            Self::PhysicalQubit(_) => true,
            Self::Circuit(_) => true,
            Self::LogicalQubits(qubits) => !qubits.is_empty(),
            Self::PhysicalQubits(qubits) => !qubits.is_empty(),
            Self::Extension(value) => !value.is_empty(),
        }
    }
}

// =============================================================================
// Acquisition policy
// =============================================================================

/// Statistical data-acquisition policy for a characterization experiment.
///
/// This type describes how much acquisition is requested. It does not execute
/// acquisition and does not perform statistical estimation.
///
/// A policy may be finite or convergence-driven.
///
/// `max_shots = None` means that the enclosing execution/resource policy is
/// responsible for deciding the operational ceiling.
///
/// It does NOT mean that a machine has infinite resources.
#[derive(Debug, Clone, PartialEq)]
pub enum AcquisitionPolicy {
    /// Perform exactly the requested number of shots.
    FixedShots(u64),

    /// Perform repeated acquisitions, where each repetition may consist of
    /// multiple shots.
    Repeated {
        /// Number of repetitions.
        repetitions: u64,

        /// Number of shots per repetition.
        shots_per_repetition: u64,
    },

    /// Continue acquisition until the downstream statistical stopping
    /// criterion is satisfied.
    ///
    /// This is a contract only; the estimator/execution layer owns convergence.
    UntilConverged {
        /// Required confidence level in `(0, 1)`.
        confidence: f64,

        /// Required absolute statistical tolerance, strictly greater than zero.
        absolute_tolerance: f64,

        /// Optional execution-policy ceiling.
        max_shots: Option<u64>,
    },

    /// Let the execution/runtime policy determine the acquisition amount.
    RuntimeControlled,
}

impl Default for AcquisitionPolicy {
    fn default() -> Self {
        Self::FixedShots(1)
    }
}

impl AcquisitionPolicy {
    /// Creates an exact fixed-shot policy.
    pub fn fixed_shots(shots: u64) -> ZqnResult<Self> {
        if shots == 0 {
            return Err(invalid_experiment(
                "fixed_shots must be greater than zero",
                "acquisition.fixed_shots",
            ));
        }

        Ok(Self::FixedShots(shots))
    }

    /// Creates a repeated acquisition policy.
    pub fn repeated(
        repetitions: u64,
        shots_per_repetition: u64,
    ) -> ZqnResult<Self> {
        if repetitions == 0 {
            return Err(invalid_experiment(
                "repetitions must be greater than zero",
                "acquisition.repetitions",
            ));
        }

        if shots_per_repetition == 0 {
            return Err(invalid_experiment(
                "shots_per_repetition must be greater than zero",
                "acquisition.shots_per_repetition",
            ));
        }

        Ok(Self::Repeated {
            repetitions,
            shots_per_repetition,
        })
    }

    /// Creates a convergence-driven policy.
    pub fn until_converged(
        confidence: f64,
        absolute_tolerance: f64,
        max_shots: Option<u64>,
    ) -> ZqnResult<Self> {
        if !confidence.is_finite() || !(0.0 < confidence && confidence < 1.0) {
            return Err(invalid_experiment(
                "confidence must be finite and strictly between zero and one",
                "acquisition.confidence",
            ));
        }

        if !absolute_tolerance.is_finite() || absolute_tolerance <= 0.0 {
            return Err(invalid_experiment(
                "absolute_tolerance must be finite and greater than zero",
                "acquisition.absolute_tolerance",
            ));
        }

        if matches!(max_shots, Some(0)) {
            return Err(invalid_experiment(
                "max_shots must be greater than zero when supplied",
                "acquisition.max_shots",
            ));
        }

        Ok(Self::UntilConverged {
            confidence,
            absolute_tolerance,
            max_shots,
        })
    }

    /// Returns true when the policy specifies a finite fixed shot count.
    #[must_use]
    pub const fn is_fixed(&self) -> bool {
        matches!(self, Self::FixedShots(_))
    }

    /// Returns the exact total shot count when it can be determined without
    /// execution.
    ///
    /// Overflow is reported instead of wrapping.
    pub fn exact_total_shots(&self) -> ZqnResult<Option<u64>> {
        match self {
            Self::FixedShots(shots) => Ok(Some(*shots)),

            Self::Repeated {
                repetitions,
                shots_per_repetition,
            } => repetitions
                .checked_mul(*shots_per_repetition)
                .map(Some)
                .ok_or_else(|| {
                    invalid_experiment(
                        "repetitions multiplied by shots_per_repetition overflowed",
                        "acquisition",
                    )
                }),

            Self::UntilConverged { .. } | Self::RuntimeControlled => Ok(None),
        }
    }
}

// =============================================================================
// Reproducibility policy
// =============================================================================

/// Reproducibility requirements for a characterization experiment.
///
/// The policy is declarative. It does not generate random numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReproducibilityPolicy {
    /// Reproducibility is required.
    ///
    /// `seed` is the caller-supplied root seed for deterministic downstream
    /// sampling where sampling is applicable.
    Required {
        seed: u64,
    },

    /// Reproducibility is requested but the execution layer may report that it
    /// cannot provide it under the selected target/capabilities.
    Requested {
        seed: u64,
    },

    /// The experiment does not require deterministic stochastic replay.
    NotRequired,
}

impl Default for ReproducibilityPolicy {
    fn default() -> Self {
        Self::Requested { seed: 0 }
    }
}

impl ReproducibilityPolicy {
    /// Returns the caller-supplied root seed when one exists.
    #[must_use]
    pub const fn seed(&self) -> Option<u64> {
        match self {
            Self::Required { seed } | Self::Requested { seed } => Some(*seed),
            Self::NotRequired => None,
        }
    }

    /// Returns whether deterministic reproducibility is mandatory.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(self, Self::Required { .. })
    }
}

// =============================================================================
// Experiment metadata
// =============================================================================

/// Small, backend-independent experiment metadata.
///
/// Metadata is descriptive and must not silently alter characterization
/// semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CharacterizationMetadata {
    /// Human-readable experiment name.
    name: Option<String>,

    /// Human-readable description.
    description: Option<String>,

    /// Stable extension/application labels.
    labels: Vec<String>,
}

impl CharacterizationMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            description: None,
            labels: Vec::new(),
        }
    }

    /// Sets a human-readable name.
    pub fn with_name(mut self, name: impl Into<String>) -> ZqnResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(invalid_experiment(
                "metadata name must not be empty",
                "metadata.name",
            ));
        }

        if contains_control_character(&name) {
            return Err(invalid_experiment(
                "metadata name must not contain control characters",
                "metadata.name",
            ));
        }

        self.name = Some(name);
        Ok(self)
    }

    /// Sets a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> ZqnResult<Self> {
        let description = description.into();

        if contains_control_character(&description) {
            return Err(invalid_experiment(
                "metadata description must not contain control characters",
                "metadata.description",
            ));
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Adds a stable descriptive label.
    pub fn with_label(mut self, label: impl Into<String>) -> ZqnResult<Self> {
        let label = label.into();
        validate_text_identifier("metadata label", &label)?;
        self.labels.push(label);
        Ok(self)
    }

    /// Returns the optional name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns metadata labels.
    #[must_use]
    pub fn labels(&self) -> &[String] {
        &self.labels
    }
}

// =============================================================================
// Characterization experiment
// =============================================================================

/// Canonical backend-independent characterization experiment definition.
///
/// This is the principal public type of this module.
///
/// It is intentionally a definition rather than an execution object.
///
/// A caller constructs an experiment once and downstream layers decide how to
/// execute it on a compatible simulator, emulator, QPU, distributed target, or
/// future quantum technology.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterizationExperiment {
    /// Stable ZQN characterization identity.
    characterization_id: CharacterizationId,

    /// Stable ZQN experiment identity.
    experiment_id: ExperimentId,

    /// Broad characterization purpose.
    kind: CharacterizationKind,

    /// Canonical quantum resources under characterization.
    resources: Vec<CharacterizationResource>,

    /// Optional canonical circuit references.
    ///
    /// Circuits remain owned by the Quantum IR.
    circuits: Vec<CircuitId>,

    /// Acquisition policy.
    acquisition: AcquisitionPolicy,

    /// Reproducibility requirement.
    reproducibility: ReproducibilityPolicy,

    /// Optional calibration snapshot used as an execution reference.
    calibration_id: Option<CalibrationId>,

    /// Optional protocol revision.
    ///
    /// The protocol implementation remains owned by `protocol.rs`.
    protocol_revision: Option<u32>,

    /// Whether the experiment requires exact realization of its requested
    /// characterization semantics rather than an approximation.
    exact_semantics_required: bool,

    /// Descriptive metadata.
    metadata: CharacterizationMetadata,
}

impl CharacterizationExperiment {
    /// Creates a new characterization experiment.
    ///
    /// Validation is performed immediately so callers do not construct an
    /// invalid experiment and discover the problem much later during execution.
    pub fn new(
        characterization_id: CharacterizationId,
        experiment_id: ExperimentId,
        kind: CharacterizationKind,
        resources: Vec<CharacterizationResource>,
    ) -> ZqnResult<Self> {
        if resources.is_empty() {
            return Err(invalid_experiment(
                "at least one characterization resource is required",
                "resources",
            ));
        }

        for resource in &resources {
            if !resource.is_non_empty() {
                return Err(invalid_experiment(
                    "characterization resource must not be empty",
                    "resources",
                ));
            }
        }

        Ok(Self {
            characterization_id,
            experiment_id,
            kind,
            resources,
            circuits: Vec::new(),
            acquisition: AcquisitionPolicy::default(),
            reproducibility: ReproducibilityPolicy::default(),
            calibration_id: None,
            protocol_revision: None,
            exact_semantics_required: true,
            metadata: CharacterizationMetadata::new(),
        })
    }

    /// Adds a canonical IR circuit reference.
    ///
    /// The circuit remains owned by the Quantum IR.
    #[must_use]
    pub fn with_circuit(mut self, circuit: CircuitId) -> Self {
        self.circuits.push(circuit);
        self
    }

    /// Adds multiple canonical IR circuit references.
    #[must_use]
    pub fn with_circuits(
        mut self,
        circuits: impl IntoIterator<Item = CircuitId>,
    ) -> Self {
        self.circuits.extend(circuits);
        self
    }

    /// Replaces the acquisition policy.
    pub fn with_acquisition(
        mut self,
        acquisition: AcquisitionPolicy,
    ) -> ZqnResult<Self> {
        validate_acquisition_policy(&acquisition)?;
        self.acquisition = acquisition;
        Ok(self)
    }

    /// Sets the reproducibility policy.
    #[must_use]
    pub const fn with_reproducibility(
        mut self,
        reproducibility: ReproducibilityPolicy,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Pins the experiment to a calibration snapshot.
    ///
    /// The calibration subsystem remains the owner of the actual snapshot.
    #[must_use]
    pub const fn with_calibration(
        mut self,
        calibration_id: CalibrationId,
    ) -> Self {
        self.calibration_id = Some(calibration_id);
        self
    }

    /// Associates the experiment with a protocol revision.
    ///
    /// The revision is metadata identifying the protocol contract to be used
    /// downstream. It does not implement or validate the protocol itself.
    #[must_use]
    pub const fn with_protocol_revision(
        mut self,
        revision: u32,
    ) -> Self {
        self.protocol_revision = Some(revision);
        self
    }

    /// Controls whether an approximate characterization realization is allowed.
    ///
    /// When `true`, the target/execution layer must preserve the exact
    /// semantics requested by this experiment.
    ///
    /// When `false`, a downstream compatibility layer may select an explicitly
    /// declared approximation, subject to its own error/uncertainty contract.
    #[must_use]
    pub const fn with_exact_semantics_required(
        mut self,
        required: bool,
    ) -> Self {
        self.exact_semantics_required = required;
        self
    }

    /// Attaches descriptive metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: CharacterizationMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns the characterization identity.
    #[must_use]
    pub const fn characterization_id(&self) -> CharacterizationId {
        self.characterization_id
    }

    /// Returns the experiment identity.
    #[must_use]
    pub const fn experiment_id(&self) -> ExperimentId {
        self.experiment_id
    }

    /// Returns the characterization kind.
    #[must_use]
    pub fn kind(&self) -> &CharacterizationKind {
        &self.kind
    }

    /// Returns the experiment resources.
    #[must_use]
    pub fn resources(&self) -> &[CharacterizationResource] {
        &self.resources
    }

    /// Returns the referenced canonical IR circuits.
    #[must_use]
    pub fn circuits(&self) -> &[CircuitId] {
        &self.circuits
    }

    /// Returns the acquisition policy.
    #[must_use]
    pub const fn acquisition(&self) -> &AcquisitionPolicy {
        &self.acquisition
    }

    /// Returns the reproducibility policy.
    #[must_use]
    pub const fn reproducibility(&self) -> &ReproducibilityPolicy {
        &self.reproducibility
    }

    /// Returns the optional calibration identity.
    #[must_use]
    pub const fn calibration_id(&self) -> Option<CalibrationId> {
        self.calibration_id
    }

    /// Returns the optional protocol revision.
    #[must_use]
    pub const fn protocol_revision(&self) -> Option<u32> {
        self.protocol_revision
    }

    /// Returns whether exact characterization semantics are mandatory.
    #[must_use]
    pub const fn exact_semantics_required(&self) -> bool {
        self.exact_semantics_required
    }

    /// Returns experiment metadata.
    #[must_use]
    pub const fn metadata(&self) -> &CharacterizationMetadata {
        &self.metadata
    }

    /// Validates the complete experiment contract.
    ///
    /// This method performs structural validation only. It deliberately does
    /// not perform target capability checks because target capability belongs
    /// to `zqn::target`.
    pub fn validate(&self) -> ZqnResult<()> {
        validate_resources(&self.resources)?;
        validate_acquisition_policy(&self.acquisition)?;
        validate_reproducibility_policy(&self.reproducibility)?;

        if let Some(revision) = self.protocol_revision {
            // Zero is reserved for "not specified" by convention. A supplied
            // protocol revision must therefore be non-zero.
            if revision == 0 {
                return Err(invalid_experiment(
                    "protocol_revision must be greater than zero when supplied",
                    "protocol_revision",
                ));
            }
        }

        validate_metadata(&self.metadata)?;

        Ok(())
    }

    /// Returns the exact requested shot count when the acquisition policy
    /// determines one statically.
    ///
    /// `None` means that execution/convergence policy determines the count.
    pub fn exact_requested_shots(&self) -> ZqnResult<Option<u64>> {
        self.acquisition.exact_total_shots()
    }

    /// Returns true if this experiment references at least one canonical
    /// circuit.
    #[must_use]
    pub fn has_circuits(&self) -> bool {
        !self.circuits.is_empty()
    }

    /// Returns the number of resource references.
    ///
    /// This is descriptive and is not a machine-size limit.
    #[must_use]
    pub fn resource_reference_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of circuit references.
    ///
    /// This is descriptive and is not a machine-size limit.
    #[must_use]
    pub fn circuit_reference_count(&self) -> usize {
        self.circuits.len()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_resources(
    resources: &[CharacterizationResource],
) -> ZqnResult<()> {
    if resources.is_empty() {
        return Err(invalid_experiment(
            "at least one characterization resource is required",
            "resources",
        ));
    }

    for resource in resources {
        match resource {
            CharacterizationResource::LogicalQubit(_) => {}

            CharacterizationResource::PhysicalQubit(_) => {}

            CharacterizationResource::Circuit(_) => {}

            CharacterizationResource::LogicalQubits(qubits) => {
                if qubits.is_empty() {
                    return Err(invalid_experiment(
                        "logical-qubit resource collection must not be empty",
                        "resources.logical_qubits",
                    ));
                }

                ensure_unique_logical_qubits(qubits)?;
            }

            CharacterizationResource::PhysicalQubits(qubits) => {
                if qubits.is_empty() {
                    return Err(invalid_experiment(
                        "physical-qubit resource collection must not be empty",
                        "resources.physical_qubits",
                    ));
                }

                ensure_unique_physical_qubits(qubits)?;
            }

            CharacterizationResource::Extension(value) => {
                validate_text_identifier("resource extension", value)?;
            }
        }
    }

    Ok(())
}

fn ensure_unique_logical_qubits(
    qubits: &[QubitId],
) -> ZqnResult<()> {
    // O(n²) validation is intentionally kept local to construction/validation.
    //
    // This avoids requiring a particular hash implementation or allocation
    // strategy in this foundational file. Large-scale callers can avoid
    // duplicate validation by constructing canonical, already-normalized
    // resource sets in their owning layer.
    //
    // The algorithm does not impose a semantic upper bound on the number of
    // qubits; runtime/resource policy governs operational scale.
    for (index, left) in qubits.iter().enumerate() {
        if qubits[index + 1..].iter().any(|right| right == left) {
            return Err(invalid_experiment(
                "logical-qubit resource collection contains a duplicate identity",
                "resources.logical_qubits",
            ));
        }
    }

    Ok(())
}

fn ensure_unique_physical_qubits(
    qubits: &[PhysicalQubitId],
) -> ZqnResult<()> {
    for (index, left) in qubits.iter().enumerate() {
        if qubits[index + 1..].iter().any(|right| right == left) {
            return Err(invalid_experiment(
                "physical-qubit resource collection contains a duplicate identity",
                "resources.physical_qubits",
            ));
        }
    }

    Ok(())
}

fn validate_acquisition_policy(
    policy: &AcquisitionPolicy,
) -> ZqnResult<()> {
    match policy {
        AcquisitionPolicy::FixedShots(shots) => {
            if *shots == 0 {
                return Err(invalid_experiment(
                    "fixed shot count must be greater than zero",
                    "acquisition.fixed_shots",
                ));
            }
        }

        AcquisitionPolicy::Repeated {
            repetitions,
            shots_per_repetition,
        } => {
            if *repetitions == 0 {
                return Err(invalid_experiment(
                    "repetitions must be greater than zero",
                    "acquisition.repetitions",
                ));
            }

            if *shots_per_repetition == 0 {
                return Err(invalid_experiment(
                    "shots_per_repetition must be greater than zero",
                    "acquisition.shots_per_repetition",
                ));
            }

            repetitions.checked_mul(*shots_per_repetition).ok_or_else(|| {
                invalid_experiment(
                    "requested repeated acquisition count overflows u64",
                    "acquisition",
                )
            })?;
        }

        AcquisitionPolicy::UntilConverged {
            confidence,
            absolute_tolerance,
            max_shots,
        } => {
            if !confidence.is_finite()
                || !(0.0 < *confidence && *confidence < 1.0)
            {
                return Err(invalid_experiment(
                    "confidence must be finite and strictly between zero and one",
                    "acquisition.confidence",
                ));
            }

            if !absolute_tolerance.is_finite()
                || *absolute_tolerance <= 0.0
            {
                return Err(invalid_experiment(
                    "absolute_tolerance must be finite and greater than zero",
                    "acquisition.absolute_tolerance",
                ));
            }

            if matches!(max_shots, Some(0)) {
                return Err(invalid_experiment(
                    "max_shots must be greater than zero when supplied",
                    "acquisition.max_shots",
                ));
            }
        }

        AcquisitionPolicy::RuntimeControlled => {}
    }

    Ok(())
}

fn validate_reproducibility_policy(
    policy: &ReproducibilityPolicy,
) -> ZqnResult<()> {
    // All currently representable seeds are valid by construction.
    //
    // Keeping validation as an explicit function establishes a stable
    // extension point if future reproducibility policies introduce additional
    // invariants.
    match policy {
        ReproducibilityPolicy::Required { .. }
        | ReproducibilityPolicy::Requested { .. }
        | ReproducibilityPolicy::NotRequired => Ok(()),
    }
}

fn validate_metadata(
    metadata: &CharacterizationMetadata,
) -> ZqnResult<()> {
    if let Some(name) = &metadata.name {
        if name.is_empty() {
            return Err(invalid_experiment(
                "metadata name must not be empty",
                "metadata.name",
            ));
        }

        if contains_control_character(name) {
            return Err(invalid_experiment(
                "metadata name must not contain control characters",
                "metadata.name",
            ));
        }
    }

    if let Some(description) = &metadata.description {
        if contains_control_character(description) {
            return Err(invalid_experiment(
                "metadata description must not contain control characters",
                "metadata.description",
            ));
        }
    }

    for label in &metadata.labels {
        validate_text_identifier("metadata label", label)?;
    }

    Ok(())
}

fn validate_text_identifier(
    field: &str,
    value: &str,
) -> ZqnResult<()> {
    if value.is_empty() {
        return Err(invalid_experiment(
            &format!("{field} must not be empty"),
            field,
        ));
    }

    if contains_control_character(value) {
        return Err(invalid_experiment(
            &format!("{field} must not contain control characters"),
            field,
        ));
    }

    Ok(())
}

fn contains_control_character(value: &str) -> bool {
    value.chars().any(char::is_control)
}

fn invalid_experiment(
    message: &str,
    field: &str,
) -> ZqnError {
    // `ZqnError` is the canonical ZQN error boundary. The exact constructor
    // contract is centralized there so characterization does not create a
    // competing error hierarchy.
    //
    // The implementation below intentionally uses the public constructor
    // contract expected from core/errors.rs.
    ZqnError::new(
        ZqnErrorKind::Characterization,
        ZqnErrorCode::InvalidCharacterizationExperiment,
        message,
    )
    .with_context(field)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Test helpers
    // ------------------------------------------------------------------------

    fn characterization_id() -> CharacterizationId {
        CharacterizationId::new(1)
    }

    fn experiment_id() -> ExperimentId {
        ExperimentId::new(1)
    }

    fn logical_qubit(value: usize) -> QubitId {
        QubitId::new(value)
    }

    fn physical_qubit(value: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(value)
    }

    fn circuit_id(value: usize) -> CircuitId {
        CircuitId::new(value)
    }

    fn experiment() -> CharacterizationExperiment {
        CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::LogicalQubit(
                logical_qubit(0),
            )],
        )
        .expect("test experiment should construct")
    }

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn constructs_valid_experiment() {
        let experiment = experiment();

        assert_eq!(
            experiment.characterization_id(),
            characterization_id()
        );
        assert_eq!(experiment.experiment_id(), experiment_id());
        assert_eq!(
            experiment.kind(),
            &CharacterizationKind::Operation
        );
        assert_eq!(experiment.resource_reference_count(), 1);
        assert!(!experiment.has_circuits());
    }

    #[test]
    fn rejects_empty_resource_scope() {
        let result = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_logical_resource_collection() {
        let result = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::LogicalQubits(
                Vec::new(),
            )],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_physical_resource_collection() {
        let result = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::PhysicalQubits(
                Vec::new(),
            )],
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Canonical qubit identity
    // ------------------------------------------------------------------------

    #[test]
    fn uses_canonical_logical_qubit_identity() {
        let qubit = logical_qubit(7);

        let resource = CharacterizationResource::logical_qubit(qubit);

        assert_eq!(resource.as_logical_qubit(), Some(qubit));
    }

    #[test]
    fn uses_canonical_physical_qubit_identity() {
        let qubit = physical_qubit(11);

        let resource = CharacterizationResource::physical_qubit(qubit);

        assert_eq!(resource.as_physical_qubit(), Some(qubit));
    }

    #[test]
    fn accepts_multiple_distinct_logical_qubits() {
        let experiment = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::logical_qubits(vec![
                logical_qubit(0),
                logical_qubit(1),
                logical_qubit(2),
                logical_qubit(3),
            ])],
        )
        .expect("distinct logical qubits should be accepted");

        assert_eq!(experiment.validate().is_ok(), true);
    }

    #[test]
    fn accepts_multiple_distinct_physical_qubits() {
        let experiment = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::physical_qubits(vec![
                physical_qubit(0),
                physical_qubit(1),
                physical_qubit(2),
                physical_qubit(3),
            ])],
        )
        .expect("distinct physical qubits should be accepted");

        assert_eq!(experiment.validate().is_ok(), true);
    }

    #[test]
    fn rejects_duplicate_logical_qubits() {
        let experiment = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::logical_qubits(vec![
                logical_qubit(1),
                logical_qubit(1),
            ])],
        )
        .expect("construction itself is allowed before full validation");

        assert!(experiment.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_physical_qubits() {
        let experiment = CharacterizationExperiment::new(
            characterization_id(),
            experiment_id(),
            CharacterizationKind::Operation,
            vec![CharacterizationResource::physical_qubits(vec![
                physical_qubit(4),
                physical_qubit(4),
            ])],
        )
        .expect("construction itself is allowed before full validation");

        assert!(experiment.validate().is_err());
    }

    // ------------------------------------------------------------------------
    // Circuit integration
    // ------------------------------------------------------------------------

    #[test]
    fn references_canonical_ir_circuit_without_embedding_it() {
        let circuit = circuit_id(42);

        let experiment = experiment().with_circuit(circuit);

        assert_eq!(experiment.circuits(), &[circuit]);
        assert!(experiment.has_circuits());
        assert_eq!(experiment.circuit_reference_count(), 1);
    }

    #[test]
    fn supports_multiple_circuit_references() {
        let experiment = experiment().with_circuits([
            circuit_id(1),
            circuit_id(2),
            circuit_id(3),
        ]);

        assert_eq!(experiment.circuit_reference_count(), 3);
    }

    // ------------------------------------------------------------------------
    // Acquisition
    // ------------------------------------------------------------------------

    #[test]
    fn fixed_shot_policy_requires_positive_shots() {
        assert!(AcquisitionPolicy::fixed_shots(0).is_err());
        assert!(AcquisitionPolicy::fixed_shots(1).is_ok());
    }

    #[test]
    fn repeated_policy_calculates_exact_total() {
        let policy =
            AcquisitionPolicy::repeated(10, 100)
                .expect("valid repeated policy");

        assert_eq!(
            policy.exact_total_shots().expect("calculation"),
            Some(1_000)
        );
    }

    #[test]
    fn repeated_policy_rejects_overflow() {
        let policy = AcquisitionPolicy::Repeated {
            repetitions: u64::MAX,
            shots_per_repetition: 2,
        };

        assert!(policy.exact_total_shots().is_err());
    }

    #[test]
    fn convergence_policy_validates_confidence() {
        assert!(
            AcquisitionPolicy::until_converged(
                0.95,
                0.01,
                None,
            )
            .is_ok()
        );

        assert!(
            AcquisitionPolicy::until_converged(
                1.0,
                0.01,
                None,
            )
            .is_err()
        );

        assert!(
            AcquisitionPolicy::until_converged(
                0.0,
                0.01,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn convergence_policy_validates_tolerance() {
        assert!(
            AcquisitionPolicy::until_converged(
                0.95,
                0.01,
                None,
            )
            .is_ok()
        );

        assert!(
            AcquisitionPolicy::until_converged(
                0.95,
                0.0,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn experiment_accepts_acquisition_policy() {
        let experiment = experiment()
            .with_acquisition(
                AcquisitionPolicy::fixed_shots(10)
                    .expect("valid policy"),
            )
            .expect("experiment should accept policy");

        assert_eq!(
            experiment
                .exact_requested_shots()
                .expect("shot calculation"),
            Some(10)
        );
    }

    // ------------------------------------------------------------------------
    // Reproducibility
    // ------------------------------------------------------------------------

    #[test]
    fn reproducibility_policy_preserves_seed() {
        let policy =
            ReproducibilityPolicy::Required { seed: 12345 };

        assert_eq!(policy.seed(), Some(12345));
        assert!(policy.is_required());
    }

    #[test]
    fn reproducibility_not_required_has_no_seed() {
        let policy = ReproducibilityPolicy::NotRequired;

        assert_eq!(policy.seed(), None);
        assert!(!policy.is_required());
    }

    #[test]
    fn experiment_preserves_reproducibility_contract() {
        let experiment = experiment().with_reproducibility(
            ReproducibilityPolicy::Required { seed: 99 },
        );

        assert_eq!(
            experiment.reproducibility().seed(),
            Some(99)
        );
        assert!(experiment.reproducibility().is_required());
    }

    // ------------------------------------------------------------------------
    // Exact semantics
    // ------------------------------------------------------------------------

    #[test]
    fn exact_semantics_are_required_by_default() {
        let experiment = experiment();

        assert!(experiment.exact_semantics_required());
    }

    #[test]
    fn exact_semantics_requirement_can_be_explicitly_relaxed() {
        let experiment =
            experiment().with_exact_semantics_required(false);

        assert!(!experiment.exact_semantics_required());
    }

    // ------------------------------------------------------------------------
    // Calibration/protocol integration
    // ------------------------------------------------------------------------

    #[test]
    fn calibration_reference_is_optional() {
        assert_eq!(experiment().calibration_id(), None);
    }

    #[test]
    fn calibration_reference_can_be_attached() {
        let calibration_id = CalibrationId::new(5);

        let experiment =
            experiment().with_calibration(calibration_id);

        assert_eq!(
            experiment.calibration_id(),
            Some(calibration_id)
        );
    }

    #[test]
    fn protocol_revision_is_optional() {
        assert_eq!(experiment().protocol_revision(), None);
    }

    #[test]
    fn protocol_revision_can_be_attached() {
        let experiment =
            experiment().with_protocol_revision(1);

        assert_eq!(
            experiment.protocol_revision(),
            Some(1)
        );
    }

    #[test]
    fn protocol_revision_zero_is_rejected() {
        let experiment =
            experiment().with_protocol_revision(0);

        assert!(experiment.validate().is_err());
    }

    // ------------------------------------------------------------------------
    // Metadata
    // ------------------------------------------------------------------------

    #[test]
    fn metadata_can_be_attached() {
        let metadata = CharacterizationMetadata::new()
            .with_name("single-operation-characterization")
            .expect("valid name")
            .with_description("Characterize one operation")
            .expect("valid description")
            .with_label("production")
            .expect("valid label");

        let experiment =
            experiment().with_metadata(metadata);

        assert_eq!(
            experiment.metadata().name(),
            Some("single-operation-characterization")
        );

        assert_eq!(
            experiment.metadata().description(),
            Some("Characterize one operation")
        );

        assert_eq!(
            experiment.metadata().labels(),
            &["production".to_string()]
        );
    }

    #[test]
    fn metadata_rejects_empty_name() {
        assert!(
            CharacterizationMetadata::new()
                .with_name("")
                .is_err()
        );
    }

    #[test]
    fn metadata_rejects_control_characters_in_name() {
        assert!(
            CharacterizationMetadata::new()
                .with_name("invalid\nname")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Extension kind/resource
    // ------------------------------------------------------------------------

    #[test]
    fn custom_characterization_kind_is_supported() {
        let kind =
            CharacterizationKind::custom("future_process");

        assert!(kind.is_ok());
        assert_eq!(
            kind.expect("valid custom kind").as_str(),
            "future_process"
        );
    }

    #[test]
    fn custom_characterization_kind_rejects_empty_value() {
        assert!(CharacterizationKind::custom("").is_err());
    }

    #[test]
    fn extension_resource_is_supported() {
        let resource =
            CharacterizationResource::extension(
                "future_quantum_resource",
            )
            .expect("valid extension");

        assert!(resource.is_non_empty());
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    #[test]
    fn complete_valid_experiment_validates() {
        let experiment = experiment()
            .with_circuit(circuit_id(1))
            .with_acquisition(
                AcquisitionPolicy::fixed_shots(100)
                    .expect("valid acquisition"),
            )
            .expect("acquisition accepted")
            .with_reproducibility(
                ReproducibilityPolicy::Required { seed: 42 },
            )
            .with_protocol_revision(1);

        assert!(experiment.validate().is_ok());
    }

    #[test]
    fn runtime_controlled_acquisition_has_no_static_shot_count() {
        let experiment = experiment()
            .with_acquisition(
                AcquisitionPolicy::RuntimeControlled,
            )
            .expect("runtime policy should be valid");

        assert_eq!(
            experiment
                .exact_requested_shots()
                .expect("query should succeed"),
            None
        );
    }

    // ------------------------------------------------------------------------
    // Clone/equality determinism
    // ------------------------------------------------------------------------

    #[test]
    fn cloning_preserves_experiment_definition() {
        let original = experiment()
            .with_circuit(circuit_id(7))
            .with_reproducibility(
                ReproducibilityPolicy::Required { seed: 123 },
            );

        let clone = original.clone();

        assert_eq!(original, clone);
    }

    #[test]
    fn experiment_definition_has_no_execution_state() {
        let experiment = experiment();

        // This test intentionally documents the architectural property:
        // cloning/equality only compares the declarative experiment contract.
        //
        // Execution handles, QPU sessions, RNG objects, threads, sockets,
        // simulator state, and observations are absent from this type.
        assert_eq!(
            experiment.resource_reference_count(),
            1
        );
    }
}