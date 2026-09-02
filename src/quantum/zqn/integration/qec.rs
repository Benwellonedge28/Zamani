//! Zamani Quantum Noise (ZQN)
//! QEC Integration Boundary
//!
//! Path:
//!     src/quantum/zqn/integration/qec.rs
//!
//! # Purpose
//!
//! This module defines the stable boundary between the universal ZQN noise
//! subsystem and Zamani's quantum error-correction (QEC) subsystem.
//!
//! The fundamental architectural relationship is:
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                         ZQN semantics
//!                              |
//!                +-------------+-------------+
//!                |                           |
//!                v                           v
//!          noise/channel/fault          calibration
//!                |
//!                v
//!          integration::qec
//!                |
//!                v
//!        QEC fault projection
//!                |
//!       +--------+---------+
//!       |                  |
//!       v                  v
//! syndrome extraction    QEC simulation
//!       |                  |
//!       v                  v
//!    decoder             correction
//!       |                  |
//!       +--------+---------+
//!                |
//!                v
//!        logical-error analysis
//! ```
//!
//! This module is an adapter contract, not a QEC implementation.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the ZQN-to-QEC integration contract;
//! - canonical integration fault-event representation;
//! - fault-location representation at the integration boundary;
//! - deterministic execution context;
//! - integration resource policy;
//! - cancellation contract;
//! - fault-source traits;
//! - fault-sink traits;
//! - conversion/validation of ZQN fault events into QEC-facing events;
//! - deterministic event ordering;
//! - integration-local validation;
//! - integration error reporting;
//! - batch/streaming interfaces between ZQN and QEC;
//! - explicit approximation/execution contracts.
//!
//! This file does NOT own:
//!
//! - the canonical Quantum IR;
//! - canonical QubitId or PhysicalQubitId definitions;
//! - quantum channels;
//! - probability mathematics;
//! - universal ZQN noise-model definitions;
//! - QEC decoders;
//! - syndrome extraction algorithms;
//! - logical correction;
//! - Pauli-frame evolution;
//! - QEC code definitions;
//! - hardware APIs;
//! - hardware credentials;
//! - routing;
//! - scheduling;
//! - simulation engines;
//! - benchmark methodology;
//! - global RNG state;
//! - global calibration state;
//! - persistent storage.
//!
//! # Critical dependency rule
//!
//! The dependency direction is:
//!
//! ```text
//! ZQN
//!   |
//!   v
//! integration::qec
//!   |
//!   v
//! QEC
//! ```
//!
//! NOT:
//!
//! ```text
//! ZQN -> concrete QEC implementation
//! ```
//!
//! In particular, this file intentionally does not import the existing
//! `quantum::error_correction::noise` implementation.
//!
//! That existing module currently contains its own physical-noise/fault
//! implementation. The long-term migration is:
//!
//! ```text
//! CURRENT
//!
//! error_correction::noise
//!        |
//!        +-- probability
//!        +-- physical faults
//!        +-- correlated faults
//!        +-- deterministic sampling
//!
//! TARGET
//!
//! zqn
//!   |
//!   +-- probability
//!   +-- channels
//!   +-- faults
//!   +-- noise models
//!   +-- correlations
//!   +-- deterministic realization
//!          |
//!          v
//! integration::qec
//!          |
//!          v
//! error_correction
//! ```
//!
//! This avoids circular dependencies and permits the QEC subsystem to migrate
//! incrementally.
//!
//! # Canonical Quantum IR identities
//!
//! When a fault refers to a logical or physical qubit, this file uses the
//! canonical identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! It never defines another `QubitId` or `PhysicalQubitId`.
//!
//! Semantic operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! This prevents identity fragmentation across:
//!
//! ```text
//! frontend
//!     |
//!     v
//! quantum::ir
//!     |
//!     v
//! ZQN
//!     |
//!     v
//! QEC
//!     |
//!     v
//! hardware
//! ```
//!
//! # Scheduling integration
//!
//! QEC fault events may optionally carry a canonical scheduling interval.
//!
//! This is deliberately represented using:
//!
//! ```text
//! quantum::ir::timing::TimeInterval
//! ```
//!
//! The integration does not redefine time.
//!
//! Scheduling remains responsible for deciding when operations occur.
//!
//! ZQN/QEC integration only consumes the resulting temporal context.
//!
//! # Universal quantum-system principle
//!
//! QEC integration must not assume that every future fault is:
//!
//! ```text
//! qubit + Pauli error
//! ```
//!
//! The boundary therefore permits:
//!
//! - bit-flip faults;
//! - phase faults;
//! - arbitrary Pauli faults;
//! - leakage;
//! - erasure;
//! - loss;
//! - measurement faults;
//! - preparation faults;
//! - transport faults;
//! - correlated faults;
//! - subsystem faults;
//! - logical-resource faults;
//! - physical-resource faults;
//! - timing faults;
//! - composite faults;
//! - technology-specific fault classes;
//! - future fault classes.
//!
//! The actual QEC code determines which events it can interpret.
//!
//! # Fault versus noise
//!
//! ZQN noise is broader than a QEC fault.
//!
//! ```text
//! noise model
//!     |
//!     +-- quantum channel
//!     +-- coherent deviation
//!     +-- stochastic process
//!     +-- temporal process
//!     +-- spatial correlation
//!     +-- drift
//!     +-- leakage
//!     +-- loss
//!     +-- discrete fault realization
//! ```
//!
//! This integration boundary deals primarily with the last category.
//!
//! A continuous/noise-channel model must therefore only be converted into
//! fault events when the selected realization explicitly supports that
//! conversion.
//!
//! No silent conversion is permitted.
//!
//! # Approximation policy
//!
//! This module recognizes four explicit execution contracts:
//!
//! ```text
//! Exact
//! Approximate
//! Bounded
//! Statistical
//! ```
//!
//! An implementation must never silently turn an exact request into an
//! approximate or statistical result.
//!
//! # Scalability
//!
//! There is no semantic upper bound on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of faults;
//! - number of correlated resources;
//! - number of QEC rounds;
//! - number of operations;
//! - circuit depth;
//! - number of shots;
//! - number of distributed nodes;
//! - fault-stream duration.
//!
//! The integration layer grows only with data actually supplied to it.
//!
//! It must support:
//!
//! - tiny systems;
//! - large systems;
//! - sparse systems;
//! - distributed systems;
//! - streaming fault sources;
//! - bounded batches;
//! - lazy generation;
//! - parallel execution;
//! - deterministic execution;
//! - resource-constrained execution.
//!
//! "Infinity" means no artificial semantic machine-size ceiling. Actual
//! execution remains bounded by available memory, CPU, accelerator resources,
//! target resources, storage and explicit caller policies.
//!
//! # Resource safety
//!
//! No global maximum such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_CORRELATION_SIZE
//! MAX_QEC_ROUNDS
//! ```
//!
//! is defined here.
//!
//! Resource limits are runtime policy.
//!
//! A caller may configure:
//!
//! - maximum events;
//! - maximum bytes;
//! - maximum correlation cardinality;
//! - maximum processing work;
//! - maximum retained events.
//!
//! `None` means that this integration layer does not impose that particular
//! limit.
//!
//! This is deliberately different from the existing QEC implementation,
//! which currently contains concrete API-safety constants such as
//! `MAX_QUBIT_INDEX`, `MAX_CORRELATED_QUBITS`, and `MAX_FAULTS_PER_BATCH`.
//! Those existing constants should eventually be moved toward explicit
//! resource-policy ownership rather than becoming universal semantic limits.
//!
//! # Determinism
//!
//! This file contains no global random generator.
//!
//! A deterministic execution is identified by:
//!
//! ```text
//! master seed
//! + program identity
//! + model identity
//! + target identity
//! + calibration identity
//! + operation identity
//! + shot index
//! + event ordinal
//! ```
//!
//! The actual ZQN noise model owns random realization.
//!
//! This module only transports the deterministic execution context.
//!
//! Parallel processing must not require a different semantic result from
//! sequential processing when the same deterministic context and fault source
//! are used.
//!
//! # Thread safety
//!
//! No global mutable state is used.
//!
//! The integration contracts are ownership-based.
//!
//! Implementations may be `Send + Sync` where their underlying state permits
//! it, but this module does not force synchronization mechanisms onto callers.
//!
//! # Serialization
//!
//! This file does not define a wire format.
//!
//! Canonical ZQN serialization remains owned by the ZQN I/O subsystem.
//!
//! QEC serialization remains owned by the QEC subsystem.
//!
//! A serialized integration event should contain semantic fields only:
//!
//! - identity;
//! - location;
//! - operation;
//! - time;
//! - classification;
//! - correlation;
//! - realization;
//! - approximation/execution contract;
//! - provenance where available.
//!
//! It must not serialize:
//!
//! - pointers;
//! - allocator state;
//! - collection capacity;
//! - hash-map internals;
//! - thread identity;
//! - process identity;
//! - temporary caches.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` intentionally makes the no-unsafe requirement
//! compiler-enforced.
//!
//! -----------------------------------------------------------------------------
//! This file is an integration boundary, not a QEC implementation.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::timing::TimeInterval;

// =============================================================================
// Result
// =============================================================================

/// Result type for ZQN/QEC integration operations.
pub type QecIntegrationResult<T> = Result<T, QecIntegrationError>;

// =============================================================================
// Execution contract
// =============================================================================

/// Accuracy/execution contract of a ZQN-to-QEC realization.
///
/// Implementations must never silently downgrade one contract to another.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionContract {
    /// The realization must be exact within the mathematical semantics of the
    /// selected representation.
    Exact,

    /// The realization is intentionally approximate.
    ///
    /// The tolerance is an application-defined non-negative finite value.
    Approximate {
        /// Declared approximation tolerance.
        tolerance: f64,
    },

    /// The realization is bounded by a declared error bound.
    Bounded {
        /// Declared upper error bound.
        error_bound: f64,
    },

    /// The realization is statistical with a declared confidence level.
    Statistical {
        /// Confidence in the interval/estimate.
        ///
        /// This must be finite and within `[0, 1]`.
        confidence: f64,
    },
}

impl ExecutionContract {
    /// Validates the numerical parameters of the contract.
    pub fn validate(self) -> QecIntegrationResult<()> {
        match self {
            Self::Exact => Ok(()),

            Self::Approximate { tolerance } => {
                validate_finite_non_negative(
                    tolerance,
                    "approximation tolerance",
                )
            }

            Self::Bounded { error_bound } => {
                validate_finite_non_negative(
                    error_bound,
                    "error bound",
                )
            }

            Self::Statistical { confidence } => {
                if !confidence.is_finite()
                    || !(0.0..=1.0).contains(&confidence)
                {
                    return Err(
                        QecIntegrationError::InvalidNumericalValue {
                            field: "confidence",
                            value: confidence,
                        },
                    );
                }

                Ok(())
            }
        }
    }
}

fn validate_finite_non_negative(
    value: f64,
    field: &'static str,
) -> QecIntegrationResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(
            QecIntegrationError::InvalidNumericalValue {
                field,
                value,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Resource policy
// =============================================================================

/// Explicit resource policy for the ZQN/QEC integration boundary.
///
/// All limits are optional.
///
/// No field is a semantic limit on quantum-system size.
///
/// They are only execution-safety policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecIntegrationLimits {
    /// Maximum number of events a materializing consumer may retain.
    pub max_events: Option<u128>,

    /// Maximum number of bytes a materializing consumer may retain.
    pub max_bytes: Option<u128>,

    /// Maximum number of resources in one correlation domain.
    pub max_correlated_resources: Option<u128>,

    /// Maximum processing steps permitted by the integration consumer.
    pub max_work: Option<u128>,
}

impl Default for QecIntegrationLimits {
    fn default() -> Self {
        Self {
            max_events: None,
            max_bytes: None,
            max_correlated_resources: None,
            max_work: None,
        }
    }
}

impl QecIntegrationLimits {
    /// Creates an unrestricted integration policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_events: None,
            max_bytes: None,
            max_correlated_resources: None,
            max_work: None,
        }
    }

    /// Validates that configured limits are internally valid.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        true
    }

    /// Checks an event-count request.
    pub fn check_events(
        &self,
        requested: u128,
    ) -> QecIntegrationResult<()> {
        if let Some(maximum) = self.max_events {
            if requested > maximum {
                return Err(
                    QecIntegrationError::ResourceLimitExceeded {
                        resource: ResourceKind::Events,
                        requested,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks a byte-count request.
    pub fn check_bytes(
        &self,
        requested: u128,
    ) -> QecIntegrationResult<()> {
        if let Some(maximum) = self.max_bytes {
            if requested > maximum {
                return Err(
                    QecIntegrationError::ResourceLimitExceeded {
                        resource: ResourceKind::Bytes,
                        requested,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks a correlation-domain size.
    pub fn check_correlated_resources(
        &self,
        requested: u128,
    ) -> QecIntegrationResult<()> {
        if let Some(maximum) = self.max_correlated_resources {
            if requested > maximum {
                return Err(
                    QecIntegrationError::ResourceLimitExceeded {
                        resource: ResourceKind::CorrelatedResources,
                        requested,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks a work estimate.
    pub fn check_work(
        &self,
        requested: u128,
    ) -> QecIntegrationResult<()> {
        if let Some(maximum) = self.max_work {
            if requested > maximum {
                return Err(
                    QecIntegrationError::ResourceLimitExceeded {
                        resource: ResourceKind::Work,
                        requested,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Cooperative cancellation contract.
///
/// ZQN and QEC implementations may provide their own cancellation primitive
/// by implementing this trait.
///
/// The integration layer never owns a global cancellation token.
pub trait Cancellation: Send + Sync {
    /// Returns whether execution has been cancelled.
    fn is_cancelled(&self) -> bool;
}

/// A cancellation implementation that is never cancelled.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancelled;

impl Cancellation for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// Deterministic execution context
// =============================================================================

/// Deterministic execution context shared by ZQN and QEC.
///
/// The fields are identifiers/seeds, not collection indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QecExecutionContext {
    /// Caller-supplied master seed.
    master_seed: u64,

    /// Program/execution identity.
    program_identity: u64,

    /// Noise-model identity.
    noise_model_identity: u64,

    /// Target identity.
    target_identity: u64,

    /// Calibration identity.
    calibration_identity: u64,

    /// Shot number.
    shot_index: u128,
}

impl QecExecutionContext {
    /// Creates an execution context.
    #[must_use]
    pub const fn new(
        master_seed: u64,
        program_identity: u64,
        noise_model_identity: u64,
        target_identity: u64,
        calibration_identity: u64,
        shot_index: u128,
    ) -> Self {
        Self {
            master_seed,
            program_identity,
            noise_model_identity,
            target_identity,
            calibration_identity,
            shot_index,
        }
    }

    /// Returns the master seed.
    #[must_use]
    pub const fn master_seed(self) -> u64 {
        self.master_seed
    }

    /// Returns the program identity.
    #[must_use]
    pub const fn program_identity(self) -> u64 {
        self.program_identity
    }

    /// Returns the noise-model identity.
    #[must_use]
    pub const fn noise_model_identity(self) -> u64 {
        self.noise_model_identity
    }

    /// Returns the target identity.
    #[must_use]
    pub const fn target_identity(self) -> u64 {
        self.target_identity
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn calibration_identity(self) -> u64 {
        self.calibration_identity
    }

    /// Returns the shot index.
    #[must_use]
    pub const fn shot_index(self) -> u128 {
        self.shot_index
    }

    /// Derives a deterministic event key.
    ///
    /// This is deliberately not a random-number generator.
    ///
    /// The value is intended to provide a stable identity for deterministic
    /// partitioning, ordering, or caller-owned seed derivation.
    #[must_use]
    pub fn event_key(
        self,
        operation_id: Option<OperationId>,
        event_ordinal: u128,
    ) -> u64 {
        let operation = operation_id
            .map(|id| id.value())
            .unwrap_or(0);

        let mut value = self.master_seed;

        value = mix_u64(value, self.program_identity);
        value = mix_u64(value, self.noise_model_identity);
        value = mix_u64(value, self.target_identity);
        value = mix_u64(value, self.calibration_identity);
        value = mix_u64(value, self.shot_index as u64);
        value = mix_u64(value, (self.shot_index >> 64) as u64);
        value = mix_u64(value, operation);
        value = mix_u64(value, event_ordinal as u64);
        value = mix_u64(value, (event_ordinal >> 64) as u64);

        value
    }
}

/// Deterministic 64-bit mixing function.
///
/// This is not intended as cryptography and must not be used as a security
/// primitive.
fn mix_u64(left: u64, right: u64) -> u64 {
    let mut x = left ^ right.wrapping_add(0x9E37_79B9_7F4A_7C15);

    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;

    x
}

// =============================================================================
// Fault location
// =============================================================================

/// Canonical integration-level location of a QEC-relevant fault.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QecFaultLocation {
    /// Fault associated with a logical qubit.
    LogicalQubit(QubitId),

    /// Fault associated with a physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Fault associated with a semantic operation.
    Operation(OperationId),

    /// Fault associated with an operation and a logical qubit.
    OperationLogicalQubit {
        /// Semantic operation.
        operation: OperationId,

        /// Logical qubit.
        qubit: QubitId,
    },

    /// Fault associated with an operation and a physical qubit.
    OperationPhysicalQubit {
        /// Semantic operation.
        operation: OperationId,

        /// Physical qubit.
        qubit: PhysicalQubitId,
    },

    /// Fault associated with an arbitrary integration resource.
    ///
    /// The namespace belongs to the broader IR/resource subsystem.
    Resource(u64),

    /// Composite fault over multiple locations.
    ///
    /// The collection is required to be canonical and duplicate-free.
    Composite(Vec<Self>),
}

impl QecFaultLocation {
    /// Creates a canonical composite location.
    ///
    /// Empty composites are rejected.
    pub fn composite(
        locations: impl IntoIterator<Item = Self>,
    ) -> QecIntegrationResult<Self> {
        let mut locations: Vec<Self> =
            locations.into_iter().collect();

        if locations.is_empty() {
            return Err(
                QecIntegrationError::EmptyCompositeLocation,
            );
        }

        locations.sort();

        for pair in locations.windows(2) {
            if pair[0] == pair[1] {
                return Err(
                    QecIntegrationError::DuplicateFaultLocation,
                );
            }
        }

        Ok(Self::Composite(locations))
    }

    /// Returns the number of primitive locations represented by this location.
    ///
    /// This is a structural count, not a machine-size limit.
    #[must_use]
    pub fn cardinality(&self) -> u128 {
        match self {
            Self::Composite(locations) => locations
                .iter()
                .map(Self::cardinality)
                .fold(0_u128, |total, value| {
                    total.saturating_add(value)
                }),

            _ => 1,
        }
    }

    /// Returns whether the location is associated with a physical qubit.
    #[must_use]
    pub fn contains_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        match self {
            Self::PhysicalQubit(candidate) => *candidate == qubit,

            Self::OperationPhysicalQubit {
                qubit: candidate,
                ..
            } => *candidate == qubit,

            Self::Composite(locations) => locations
                .iter()
                .any(|location| {
                    location.contains_physical_qubit(qubit)
                }),

            _ => false,
        }
    }

    /// Returns whether the location is associated with a logical qubit.
    #[must_use]
    pub fn contains_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        match self {
            Self::LogicalQubit(candidate) => *candidate == qubit,

            Self::OperationLogicalQubit {
                qubit: candidate,
                ..
            } => *candidate == qubit,

            Self::Composite(locations) => locations
                .iter()
                .any(|location| {
                    location.contains_logical_qubit(qubit)
                }),

            _ => false,
        }
    }
}

// =============================================================================
// Fault classification
// =============================================================================

/// Broad, technology-independent classification of a QEC-relevant fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecFaultKind {
    /// State-preparation error.
    Preparation,

    /// Gate/operation error.
    Operation,

    /// Measurement error.
    Measurement,

    /// Reset error.
    Reset,

    /// Idle/decoherence event.
    Idle,

    /// Bit-flip-like fault.
    BitFlip,

    /// Phase-flip-like fault.
    PhaseFlip,

    /// General Pauli fault.
    Pauli,

    /// Leakage outside the intended computational subspace.
    Leakage,

    /// Erasure with a known/identified location.
    Erasure,

    /// Physical loss.
    Loss,

    /// Transport/movement fault.
    Transport,

    /// Correlated fault with more than one affected resource.
    Correlated,

    /// Coherent/control-system deviation represented at the QEC boundary.
    Coherent,

    /// Generic fault not covered by the predefined categories.
    Other,
}

// =============================================================================
// Fault event
// =============================================================================

/// A deterministic, QEC-facing realization of ZQN noise.
#[derive(Debug, Clone, PartialEq)]
pub struct QecFaultEvent {
    /// Stable event identity within the integration stream.
    event_id: u128,

    /// Semantic operation associated with the event, when applicable.
    operation_id: Option<OperationId>,

    /// Fault location.
    location: QecFaultLocation,

    /// Fault classification.
    kind: QecFaultKind,

    /// Temporal interval associated with the event, when known.
    interval: Option<TimeInterval>,

    /// Deterministic event ordinal supplied by the source.
    ordinal: u128,

    /// Explicit execution contract.
    contract: ExecutionContract,

    /// Optional model-specific scalar weight.
    ///
    /// This is intentionally not interpreted as a universal probability.
    ///
    /// Probability semantics belong to the ZQN probability subsystem.
    weight: Option<f64>,
}

impl QecFaultEvent {
    /// Creates a fault event.
    pub fn new(
        event_id: u128,
        operation_id: Option<OperationId>,
        location: QecFaultLocation,
        kind: QecFaultKind,
        interval: Option<TimeInterval>,
        ordinal: u128,
        contract: ExecutionContract,
        weight: Option<f64>,
    ) -> QecIntegrationResult<Self> {
        contract.validate()?;

        if let Some(weight) = weight {
            if !weight.is_finite() || weight < 0.0 {
                return Err(
                    QecIntegrationError::InvalidNumericalValue {
                        field: "fault weight",
                        value: weight,
                    },
                );
            }
        }

        let event = Self {
            event_id,
            operation_id,
            location,
            kind,
            interval,
            ordinal,
            contract,
            weight,
        };

        event.validate()?;

        Ok(event)
    }

    /// Returns the event identity.
    #[must_use]
    pub const fn event_id(&self) -> u128 {
        self.event_id
    }

    /// Returns the associated operation identity.
    #[must_use]
    pub const fn operation_id(
        &self,
    ) -> Option<OperationId> {
        self.operation_id
    }

    /// Returns the fault location.
    #[must_use]
    pub fn location(&self) -> &QecFaultLocation {
        &self.location
    }

    /// Returns the fault classification.
    #[must_use]
    pub const fn kind(&self) -> QecFaultKind {
        self.kind
    }

    /// Returns the associated time interval.
    #[must_use]
    pub const fn interval(
        &self,
    ) -> Option<TimeInterval> {
        self.interval
    }

    /// Returns the deterministic event ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> u128 {
        self.ordinal
    }

    /// Returns the execution contract.
    #[must_use]
    pub const fn contract(
        &self,
    ) -> ExecutionContract {
        self.contract
    }

    /// Returns the optional model-specific weight.
    #[must_use]
    pub const fn weight(&self) -> Option<f64> {
        self.weight
    }

    /// Validates event invariants.
    pub fn validate(&self) -> QecIntegrationResult<()> {
        self.contract.validate()?;

        if let Some(weight) = self.weight {
            if !weight.is_finite() || weight < 0.0 {
                return Err(
                    QecIntegrationError::InvalidNumericalValue {
                        field: "fault weight",
                        value: weight,
                    },
                );
            }
        }

        if self.location.cardinality() == 0 {
            return Err(
                QecIntegrationError::EmptyCompositeLocation,
            );
        }

        Ok(())
    }

    /// Returns a deterministic sort key.
    ///
    /// This ordering is semantic and independent of insertion order.
    #[must_use]
    pub fn ordering_key(
        &self,
    ) -> FaultOrderingKey {
        FaultOrderingKey {
            start: self.interval.map(|interval| {
                interval.start()
            }),
            end: self.interval.map(|interval| {
                interval.end()
            }),
            operation_id: self.operation_id,
            event_id: self.event_id,
            ordinal: self.ordinal,
        }
    }
}

/// Deterministic fault-event ordering key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultOrderingKey {
    /// Optional start time.
    pub start: Option<crate::quantum::ir::timing::TimePoint>,

    /// Optional end time.
    pub end: Option<crate::quantum::ir::timing::TimePoint>,

    /// Optional operation identity.
    pub operation_id: Option<OperationId>,

    /// Stable event identity.
    pub event_id: u128,

    /// Source ordinal.
    pub ordinal: u128,
}

impl Ord for FaultOrderingKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
            .then_with(|| {
                self.operation_id
                    .cmp(&other.operation_id)
            })
            .then_with(|| self.event_id.cmp(&other.event_id))
            .then_with(|| self.ordinal.cmp(&other.ordinal))
    }
}

impl PartialOrd for FaultOrderingKey {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Fault source
// =============================================================================

/// Streaming source of QEC-relevant fault realizations.
///
/// A ZQN noise model can implement this contract directly or through an
/// adapter owned by the ZQN noise subsystem.
///
/// The source does not require materializing all faults in memory.
pub trait QecFaultSource {
    /// Returns the next fault event.
    ///
    /// `None` means the source is exhausted.
    fn next_fault(
        &mut self,
    ) -> QecIntegrationResult<Option<QecFaultEvent>>;
}

// =============================================================================
// Fault sink
// =============================================================================

/// Consumer of QEC-facing fault events.
///
/// QEC implementations can implement this trait without requiring ZQN to know
/// about their internal decoder, code, syndrome or correction structures.
pub trait QecFaultSink {
    /// Accepts one validated fault event.
    fn accept_fault(
        &mut self,
        event: QecFaultEvent,
    ) -> QecIntegrationResult<()>;

    /// Signals the end of a fault stream.
    fn finish(
        &mut self,
    ) -> QecIntegrationResult<()> {
        Ok(())
    }
}

// =============================================================================
// Integration context
// =============================================================================

/// Context controlling one ZQN-to-QEC integration execution.
pub struct QecIntegrationContext<'a> {
    /// Deterministic execution identity.
    execution: QecExecutionContext,

    /// Resource policy.
    limits: QecIntegrationLimits,

    /// Cancellation provider.
    cancellation: &'a dyn Cancellation,

    /// Required execution contract.
    contract: ExecutionContract,
}

impl<'a> QecIntegrationContext<'a> {
    /// Creates an integration context.
    pub fn new(
        execution: QecExecutionContext,
        limits: QecIntegrationLimits,
        cancellation: &'a dyn Cancellation,
        contract: ExecutionContract,
    ) -> QecIntegrationResult<Self> {
        contract.validate()?;

        if !limits.is_valid() {
            return Err(
                QecIntegrationError::InvalidLimits,
            );
        }

        Ok(Self {
            execution,
            limits,
            cancellation,
            contract,
        })
    }

    /// Returns deterministic execution identity.
    #[must_use]
    pub const fn execution(
        &self,
    ) -> QecExecutionContext {
        self.execution
    }

    /// Returns resource limits.
    #[must_use]
    pub const fn limits(
        &self,
    ) -> QecIntegrationLimits {
        self.limits
    }

    /// Returns the required execution contract.
    #[must_use]
    pub const fn contract(
        &self,
    ) -> ExecutionContract {
        self.contract
    }

    /// Checks cancellation.
    pub fn checkpoint(
        &self,
    ) -> QecIntegrationResult<()> {
        if self.cancellation.is_cancelled() {
            return Err(
                QecIntegrationError::Cancelled,
            );
        }

        Ok(())
    }

    /// Checks that a produced event satisfies the required execution
    /// contract.
    pub fn validate_event_contract(
        &self,
        event: &QecFaultEvent,
    ) -> QecIntegrationResult<()> {
        event.validate()?;

        match (self.contract, event.contract) {
            (ExecutionContract::Exact, ExecutionContract::Exact) => {
                Ok(())
            }

            (
                ExecutionContract::Approximate { .. },
                ExecutionContract::Exact,
            ) => Ok(()),

            (
                ExecutionContract::Bounded { .. },
                ExecutionContract::Exact,
            ) => Ok(()),

            (
                ExecutionContract::Statistical { .. },
                ExecutionContract::Exact,
            ) => Ok(()),

            (required, provided) => {
                if execution_contract_is_compatible(
                    required,
                    provided,
                ) {
                    Ok(())
                } else {
                    Err(
                        QecIntegrationError::ExecutionContractMismatch {
                            required,
                            provided,
                        },
                    )
                }
            }
        }
    }
}

fn execution_contract_is_compatible(
    required: ExecutionContract,
    provided: ExecutionContract,
) -> bool {
    match (required, provided) {
        (ExecutionContract::Exact, ExecutionContract::Exact) => true,

        (
            ExecutionContract::Approximate {
                tolerance: required,
            },
            ExecutionContract::Approximate {
                tolerance: provided,
            },
        ) => provided <= required,

        (
            ExecutionContract::Bounded {
                error_bound: required,
            },
            ExecutionContract::Bounded {
                error_bound: provided,
            },
        ) => provided <= required,

        (
            ExecutionContract::Statistical {
                confidence: required,
            },
            ExecutionContract::Statistical {
                confidence: provided,
            },
        ) => provided >= required,

        _ => false,
    }
}

// =============================================================================
// Fault stream driver
// =============================================================================

/// Deterministic streaming adapter from a ZQN fault source to a QEC sink.
pub struct QecFaultBridge;

impl QecFaultBridge {
    /// Streams faults from `source` into `sink`.
    ///
    /// Events are validated and delivered one at a time.
    ///
    /// This method does not require materializing the entire fault set.
    ///
    /// Event order is validated as deterministic source order. A source that
    /// produces non-monotonic event identities is rejected rather than
    /// silently reordered, because silently reordering a stochastic source can
    /// change its semantics.
    pub fn stream<S, T>(
        source: &mut S,
        sink: &mut T,
        context: &QecIntegrationContext<'_>,
    ) -> QecIntegrationResult<u128>
    where
        S: QecFaultSource,
        T: QecFaultSink,
    {
        let mut count = 0_u128;
        let mut previous_key: Option<FaultOrderingKey> = None;

        loop {
            context.checkpoint()?;

            let event = match source.next_fault()? {
                Some(event) => event,
                None => break,
            };

            context.validate_event_contract(&event)?;

            let key = event.ordering_key();

            if let Some(previous) = previous_key {
                if key < previous {
                    return Err(
                        QecIntegrationError::NonDeterministicOrder,
                    );
                }
            }

            previous_key = Some(key);

            count = count.checked_add(1).ok_or(
                QecIntegrationError::ArithmeticOverflow,
            )?;

            context.limits().check_events(count)?;

            sink.accept_fault(event)?;
        }

        sink.finish()?;

        Ok(count)
    }

    /// Collects a bounded number of events from a source.
    ///
    /// This is intentionally an explicit materialization operation.
    ///
    /// Callers handling large or unbounded systems should prefer `stream`.
    pub fn collect_bounded<S>(
        source: &mut S,
        context: &QecIntegrationContext<'_>,
        requested: u128,
    ) -> QecIntegrationResult<Vec<QecFaultEvent>>
    where
        S: QecFaultSource,
    {
        context.checkpoint()?;
        context.limits().check_events(requested)?;

        let capacity = usize::try_from(requested)
            .map_err(|_| {
                QecIntegrationError::CollectionSizeOverflow {
                    requested,
                }
            })?;

        let mut events = Vec::with_capacity(capacity);

        let mut previous_key: Option<FaultOrderingKey> = None;

        while events.len() < capacity {
            context.checkpoint()?;

            let event = match source.next_fault()? {
                Some(event) => event,
                None => break,
            };

            context.validate_event_contract(&event)?;

            let key = event.ordering_key();

            if let Some(previous) = previous_key {
                if key < previous {
                    return Err(
                        QecIntegrationError::NonDeterministicOrder,
                    );
                }
            }

            previous_key = Some(key);
            events.push(event);
        }

        Ok(events)
    }
}

// =============================================================================
// Correlation validation
// =============================================================================

/// Validates that all physical resources represented by an event are unique.
pub fn validate_event_resource_uniqueness(
    event: &QecFaultEvent,
) -> QecIntegrationResult<()> {
    let mut physical = BTreeSet::new();
    let mut logical = BTreeSet::new();

    collect_resource_ids(
        event.location(),
        &mut physical,
        &mut logical,
    )?;

    Ok(())
}

fn collect_resource_ids(
    location: &QecFaultLocation,
    physical: &mut BTreeSet<PhysicalQubitId>,
    logical: &mut BTreeSet<QubitId>,
) -> QecIntegrationResult<()> {
    match location {
        QecFaultLocation::PhysicalQubit(id) => {
            if !physical.insert(*id) {
                return Err(
                    QecIntegrationError::DuplicateFaultLocation,
                );
            }
        }

        QecFaultLocation::LogicalQubit(id) => {
            if !logical.insert(*id) {
                return Err(
                    QecIntegrationError::DuplicateFaultLocation,
                );
            }
        }

        QecFaultLocation::OperationPhysicalQubit {
            qubit,
            ..
        } => {
            if !physical.insert(*qubit) {
                return Err(
                    QecIntegrationError::DuplicateFaultLocation,
                );
            }
        }

        QecFaultLocation::OperationLogicalQubit {
            qubit,
            ..
        } => {
            if !logical.insert(*qubit) {
                return Err(
                    QecIntegrationError::DuplicateFaultLocation,
                );
            }
        }

        QecFaultLocation::Composite(locations) => {
            for child in locations {
                collect_resource_ids(
                    child,
                    physical,
                    logical,
                )?;
            }
        }

        QecFaultLocation::Operation(_)
        | QecFaultLocation::Resource(_) => {}
    }

    Ok(())
}

// =============================================================================
// QEC integration errors
// =============================================================================

/// Errors produced by the ZQN/QEC integration boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum QecIntegrationError {
    /// A numerical input was invalid.
    InvalidNumericalValue {
        /// Field name.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// An execution contract was invalid or unsupported.
    ExecutionContractMismatch {
        /// Required contract.
        required: ExecutionContract,

        /// Provided contract.
        provided: ExecutionContract,
    },

    /// A configured resource policy was invalid.
    InvalidLimits,

    /// An execution cancellation was requested.
    Cancelled,

    /// A resource policy was exceeded.
    ResourceLimitExceeded {
        /// Resource category.
        resource: ResourceKind,

        /// Requested amount.
        requested: u128,

        /// Configured maximum.
        maximum: u128,
    },

    /// Event arithmetic overflowed.
    ArithmeticOverflow,

    /// A collection size could not be represented by the platform collection
    /// type.
    CollectionSizeOverflow {
        /// Requested collection size.
        requested: u128,
    },

    /// A composite fault location was empty.
    EmptyCompositeLocation,

    /// A composite fault contained the same location more than once.
    DuplicateFaultLocation,

    /// A source emitted events in non-deterministic order.
    NonDeterministicOrder,

    /// A source returned an invalid fault event.
    InvalidFaultEvent,

    /// The requested operation is not supported by the selected adapter.
    Unsupported {
        /// Human-readable description.
        message: String,
    },
}

impl fmt::Display for QecIntegrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNumericalValue {
                field,
                value,
            } => write!(
                formatter,
                "invalid numerical value for {field}: {value}"
            ),

            Self::ExecutionContractMismatch {
                required,
                provided,
            } => write!(
                formatter,
                "QEC execution contract mismatch: required {required:?}, provided {provided:?}"
            ),

            Self::InvalidLimits => {
                formatter.write_str(
                    "invalid QEC integration resource limits",
                )
            }

            Self::Cancelled => {
                formatter.write_str(
                    "ZQN/QEC integration cancelled",
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                formatter,
                "QEC integration resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
            ),

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "QEC integration arithmetic overflow",
                )
            }

            Self::CollectionSizeOverflow {
                requested,
            } => write!(
                formatter,
                "requested collection size {requested} cannot be represented by the platform collection type"
            ),

            Self::EmptyCompositeLocation => {
                formatter.write_str(
                    "composite fault location cannot be empty",
                )
            }

            Self::DuplicateFaultLocation => {
                formatter.write_str(
                    "fault location contains duplicate resources",
                )
            }

            Self::NonDeterministicOrder => {
                formatter.write_str(
                    "fault source produced non-deterministic event ordering",
                )
            }

            Self::InvalidFaultEvent => {
                formatter.write_str(
                    "fault source produced an invalid fault event",
                )
            }

            Self::Unsupported { message } => {
                write!(
                    formatter,
                    "unsupported ZQN/QEC integration operation: {message}"
                )
            }
        }
    }
}

impl std::error::Error for QecIntegrationError {}

/// Resource categories used by integration resource policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Fault/event count.
    Events,

    /// Retained memory.
    Bytes,

    /// Number of correlated resources.
    CorrelatedResources,

    /// Abstract processing work.
    Work,
}

impl fmt::Display for ResourceKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Events => formatter.write_str("events"),
            Self::Bytes => formatter.write_str("bytes"),
            Self::CorrelatedResources => {
                formatter.write_str("correlated-resources")
            }
            Self::Work => formatter.write_str("work"),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct VecFaultSource {
        events: Vec<QecFaultEvent>,
        position: usize,
    }

    impl VecFaultSource {
        fn new(
            events: Vec<QecFaultEvent>,
        ) -> Self {
            Self {
                events,
                position: 0,
            }
        }
    }

    impl QecFaultSource for VecFaultSource {
        fn next_fault(
            &mut self,
        ) -> QecIntegrationResult<Option<QecFaultEvent>> {
            if self.position >= self.events.len() {
                return Ok(None);
            }

            let event = self.events[self.position].clone();
            self.position += 1;

            Ok(Some(event))
        }
    }

    #[derive(Default)]
    struct VecFaultSink {
        events: Vec<QecFaultEvent>,
    }

    impl QecFaultSink for VecFaultSink {
        fn accept_fault(
            &mut self,
            event: QecFaultEvent,
        ) -> QecIntegrationResult<()> {
            self.events.push(event);
            Ok(())
        }
    }

    fn context() -> QecIntegrationContext<'static> {
        QecIntegrationContext::new(
            QecExecutionContext::new(
                42,
                1,
                2,
                3,
                4,
                0,
            ),
            QecIntegrationLimits::unrestricted(),
            &NeverCancelled,
            ExecutionContract::Exact,
        )
        .expect("valid context")
    }

    fn event(
        event_id: u128,
        ordinal: u128,
    ) -> QecFaultEvent {
        QecFaultEvent::new(
            event_id,
            None,
            QecFaultLocation::PhysicalQubit(
                PhysicalQubitId::new(0),
            ),
            QecFaultKind::Pauli,
            None,
            ordinal,
            ExecutionContract::Exact,
            None,
        )
        .expect("valid event")
    }

    #[test]
    fn exact_contract_is_valid() {
        assert!(
            ExecutionContract::Exact.validate().is_ok()
        );
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let result = ExecutionContract::Approximate {
            tolerance: f64::NAN,
        }
        .validate();

        assert!(result.is_err());
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let result = ExecutionContract::Statistical {
            confidence: 2.0,
        }
        .validate();

        assert!(result.is_err());
    }

    #[test]
    fn composite_locations_are_canonical() {
        let composite =
            QecFaultLocation::composite([
                QecFaultLocation::PhysicalQubit(
                    PhysicalQubitId::new(2),
                ),
                QecFaultLocation::PhysicalQubit(
                    PhysicalQubitId::new(1),
                ),
            ])
            .expect("valid composite");

        assert_eq!(composite.cardinality(), 2);
    }

    #[test]
    fn duplicate_composite_location_is_rejected() {
        let result =
            QecFaultLocation::composite([
                QecFaultLocation::PhysicalQubit(
                    PhysicalQubitId::new(1),
                ),
                QecFaultLocation::PhysicalQubit(
                    PhysicalQubitId::new(1),
                ),
            ]);

        assert!(matches!(
            result,
            Err(QecIntegrationError::DuplicateFaultLocation)
        ));
    }

    #[test]
    fn deterministic_event_key_is_stable() {
        let execution =
            QecExecutionContext::new(
                123,
                1,
                2,
                3,
                4,
                5,
            );

        let first = execution.event_key(
            Some(OperationId::new(7)),
            9,
        );

        let second = execution.event_key(
            Some(OperationId::new(7)),
            9,
        );

        assert_eq!(first, second);
    }

    #[test]
    fn stream_is_incremental() {
        let mut source =
            VecFaultSource::new(vec![
                event(1, 0),
                event(2, 1),
                event(3, 2),
            ]);

        let mut sink =
            VecFaultSink::default();

        let count =
            QecFaultBridge::stream(
                &mut source,
                &mut sink,
                &context(),
            )
            .expect("stream succeeds");

        assert_eq!(count, 3);
        assert_eq!(sink.events.len(), 3);
    }

    #[test]
    fn non_monotonic_source_is_rejected() {
        let mut source =
            VecFaultSource::new(vec![
                event(2, 1),
                event(1, 0),
            ]);

        let mut sink =
            VecFaultSink::default();

        let result =
            QecFaultBridge::stream(
                &mut source,
                &mut sink,
                &context(),
            );

        assert!(matches!(
            result,
            Err(
                QecIntegrationError::NonDeterministicOrder
            )
        ));
    }

    #[test]
    fn event_limit_is_enforced() {
        let limits = QecIntegrationLimits {
            max_events: Some(1),
            ..QecIntegrationLimits::unrestricted()
        };

        let cancellation = NeverCancelled;

        let context =
            QecIntegrationContext::new(
                QecExecutionContext::new(
                    1, 1, 1, 1, 1, 0,
                ),
                limits,
                &cancellation,
                ExecutionContract::Exact,
            )
            .expect("valid context");

        let mut source =
            VecFaultSource::new(vec![
                event(1, 0),
                event(2, 1),
            ]);

        let mut sink =
            VecFaultSink::default();

        let result =
            QecFaultBridge::stream(
                &mut source,
                &mut sink,
                &context,
            );

        assert!(matches!(
            result,
            Err(
                QecIntegrationError::ResourceLimitExceeded {
                    resource: ResourceKind::Events,
                    ..
                }
            )
        ));
    }

    #[test]
    fn cancellation_is_observed() {
        struct Cancelled;

        impl Cancellation for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let cancellation = Cancelled;

        let context =
            QecIntegrationContext::new(
                QecExecutionContext::new(
                    1, 1, 1, 1, 1, 0,
                ),
                QecIntegrationLimits::unrestricted(),
                &cancellation,
                ExecutionContract::Exact,
            )
            .expect("valid context");

        let result = context.checkpoint();

        assert!(matches!(
            result,
            Err(QecIntegrationError::Cancelled)
        ));
    }
}