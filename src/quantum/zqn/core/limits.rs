//! Zamani Quantum Noise (ZQN) — Declarative Resource Policy.
//!
//! # Ownership
//!
//! This module owns the declarative resource-policy boundary for ZQN.
//!
//! `ZqnLimits` answers:
//!
//! > "How much work/resource consumption is this particular ZQN execution
//! > context permitted to request?"
//!
//! It owns:
//!
//! - optional resource ceilings;
//! - explicit unlimited semantics;
//! - resource-dimension classification;
//! - limit validation;
//! - checked admission/preflight arithmetic;
//! - resource-request validation;
//! - deterministic limit diagnostics;
//! - policy composition helpers;
//! - serialization of the policy itself;
//! - schema identification for the policy;
//! - conversion between portable counts and machine-sized allocation counts.
//!
//! It does NOT own:
//!
//! - runtime resource accounting;
//! - actual memory allocation;
//! - allocator behavior;
//! - operating-system limits;
//! - process limits;
//! - hardware capacity;
//! - QPU discovery;
//! - QPU credentials;
//! - scheduling;
//! - routing;
//! - simulation;
//! - channel semantics;
//! - fault semantics;
//! - probability semantics;
//! - calibration;
//! - benchmarking;
//! - cancellation state;
//! - quantum-resource identity.
//!
//! # Fundamental architectural rule
//!
//! ZQN must not encode an artificial maximum number of qubits, operations,
//! faults, shots, tensor elements, bytes, or any other resource merely because
//! a finite default is convenient.
//!
//! Therefore this module uses:
//!
//! ```text
//! Some(limit) = explicitly bounded execution policy
//! None         = no ZQN-imposed ceiling for that dimension
//! ```
//!
//! `None` does NOT mean that the host machine, operating system, allocator,
//! compiler, runtime, QPU, simulator, or target has infinite capacity.
//!
//! It means only:
//!
//! > ZQN itself imposes no additional ceiling for this resource dimension.
//!
//! Actual availability remains determined by the runtime, target, memory
//! manager, operating system and physical resources.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no semantic constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_FAULTS
//! MAX_SHOTS
//! MAX_MEMORY
//! MAX_TENSOR_ELEMENTS
//! MAX_CORRELATED_RESOURCES
//! ```
//!
//! Such constants would eventually become accidental architectural ceilings.
//!
//! A ZQN computation may therefore describe any finite workload representable
//! by the available resources, subject only to an explicitly supplied
//! execution policy.
//!
//! # Resource policy versus runtime accounting
//!
//! The distinction is mandatory:
//!
//! ```text
//! ZqnLimits
//!     = admission policy
//!
//! Runtime resource manager
//!     = actual consumption/accounting
//!
//! Allocator
//!     = actual memory acquisition
//!
//! Hardware target
//!     = physical capability
//! ```
//!
//! `ZqnLimits` never pretends to know how much memory, CPU, GPU capacity,
//! device capacity or QPU capacity actually exists.
//!
//! # Integration contract
//!
//! `core::errors` may use this module's resource classification and values
//! when producing a canonical `ZqnError::LimitExceeded` diagnostic.
//!
//! `core::ids` remains independent from this module. ZQN resource policy does
//! not create a second `QubitId` or `PhysicalQubitId`.
//!
//! `core::context` should own an instance of `ZqnLimits` and pass it to
//! downstream ZQN operations.
//!
//! `channel`, `fault`, `noise`, `simulation`, `calibration`,
//! `characterization`, `propagation`, and `io` should use this module for
//! explicit preflight/admission checks instead of introducing their own
//! competing limit structures.
//!
//! `target` may combine ZQN policy with target capabilities, but target
//! capacity is not stored as a ZQN semantic limit.
//!
//! `integration::ir` should use the generic count-checking APIs rather than
//! depending on any particular IR representation.
//!
//! `integration::routing` and `integration::scheduling` may query the policy
//! before constructing potentially large work products.
//!
//! `integration::qec` may adapt QEC-specific limits into a ZQN execution
//! policy, but ZQN must not depend on QEC.
//!
//! `io` owns serialization transport. This file only defines the stable
//! serializable policy representation.
//!
//! # Canonical qubit identity
//!
//! This module intentionally does NOT define a qubit identifier.
//!
//! When a caller needs to validate a concrete quantum-resource identity, it
//! must use the canonical identity owned by:
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
//! A limit is about a resource *quantity*. It is not an identity type.
//!
//! Consequently, this low-level policy module does not need to depend on
//! `quantum::ir::qubit`, preventing an unnecessary dependency edge.
//!
//! Higher-level modules that possess canonical qubit IDs can use the generic
//! count APIs here.
//!
//! # Determinism
//!
//! Policy validation is deterministic.
//!
//! Given identical policy values and identical requested resource values, the
//! result is identical regardless of:
//!
//! - process address space;
//! - thread scheduling;
//! - execution order;
//! - host operating system;
//! - backend;
//! - quantum technology.
//!
//! This module owns no RNG and has no global mutable state.
//!
//! # Resource safety
//!
//! All derived-resource calculations use checked arithmetic.
//!
//! In particular, this module never computes a product such as:
//!
//! ```text
//! qubits * shots * operations
//! ```
//!
//! using unchecked arithmetic.
//!
//! Callers should use the checked APIs provided here before allocation or
//! execution.
//!
//! # No allocation requirement
//!
//! The policy representation itself is allocation-free.
//!
//! This is intentional because it may be consulted on failure paths or before
//! allocation is permitted.
//!
//! # Serialization
//!
//! `ZqnLimits` is serialized as an explicit, versioned policy object.
//!
//! `None` must remain distinguishable from `Some(0)`:
//!
//! ```text
//! None       = unlimited by ZQN policy
//! Some(0)    = invalid policy
//! Some(n>0)  = explicit ceiling
//! ```
//!
//! # Compatibility
//!
//! The schema version is owned here so that changing the semantic meaning of
//! this structure can be handled explicitly.
//!
//! The serialized representation must not depend on Rust struct layout.
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
//! - no `unsafe`.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. it contains no machine-size semantic ceiling;
//! 2. unlimited policy is representable;
//! 3. zero limits are rejected;
//! 4. checked arithmetic is used for derived quantities;
//! 5. policy validation is deterministic;
//! 6. the policy is independent of runtime accounting;
//! 7. no allocation is required for policy validation;
//! 8. no global mutable state exists;
//! 9. no RNG is used;
//! 10. no competing qubit identity exists;
//! 11. serialization is versioned;
//! 12. Rust 1.97.1 accepts the implementation;
//! 13. downstream modules can use it without changing this file merely because
//!     they are implemented later;
//! 14. adding a larger machine does not require changing this file;
//! 15. adding a new resource dimension is an explicit API/schema change rather
//!     than a hidden hard-coded ceiling.
//!
//! # Testing
//!
//! Tests at the bottom of this file verify:
//!
//! - unlimited semantics;
//! - bounded semantics;
//! - zero rejection;
//! - exact-boundary acceptance;
//! - over-bound rejection;
//! - checked addition;
//! - checked multiplication;
//! - checked accumulation;
//! - policy validation;
//! - schema validation;
//! - deterministic display;
//! - serde round-trip;
//! - absence of machine-size assumptions.
//!
//! Domain-specific integration tests belong in the corresponding ZQN
//! subsystem test directories.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use core::fmt;

use serde::{Deserialize, Serialize};

/// Current serialized schema version for [`ZqnLimits`].
///
/// Increment this value only when the serialized structure or semantic meaning
/// changes in an incompatible way.
pub const ZQN_LIMITS_SCHEMA_VERSION: u32 = 1;

/// A portable resource count.
///
/// `u128` is intentionally used for policy/request arithmetic so that
/// intermediate calculations can be represented without prematurely reducing
/// them to the host's `usize`.
///
/// This is a mathematical/resource-policy quantity, not necessarily an
/// allocation size.
pub type ResourceCount = u128;

/// Optional resource ceiling.
///
/// ```text
/// None       -> no ZQN-imposed ceiling
/// Some(n>0)  -> explicitly bounded
/// Some(0)    -> invalid policy
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Limit(Option<ResourceCount>);

impl Limit {
    /// Creates an unlimited policy dimension.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self(None)
    }

    /// Creates an explicitly bounded policy dimension.
    ///
    /// Zero is rejected because a zero resource ceiling would make the
    /// corresponding execution dimension unusable and is almost always a
    /// configuration error.
    pub const fn bounded(
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        if maximum == 0 {
            return Err(LimitError::ZeroLimit {
                resource: LimitKind::Unknown,
            });
        }

        Ok(Self(Some(maximum)))
    }

    /// Creates a bounded limit while associating the resource dimension with
    /// any resulting error.
    pub const fn bounded_for(
        resource: LimitKind,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        if maximum == 0 {
            return Err(LimitError::ZeroLimit { resource });
        }

        Ok(Self(Some(maximum)))
    }

    /// Creates a limit directly from an optional value.
    ///
    /// This constructor is intended for deserialization and policy builders
    /// that already know the resource dimension.
    pub const fn from_option(
        resource: LimitKind,
        maximum: Option<ResourceCount>,
    ) -> Result<Self, LimitError> {
        match maximum {
            None => Ok(Self::unlimited()),
            Some(0) => Err(LimitError::ZeroLimit { resource }),
            Some(value) => Ok(Self(Some(value))),
        }
    }

    /// Returns true when this dimension has no ZQN-imposed ceiling.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.0.is_none()
    }

    /// Returns the configured ceiling.
    #[must_use]
    pub const fn maximum(self) -> Option<ResourceCount> {
        self.0
    }

    /// Returns true when the requested amount is permitted.
    #[must_use]
    pub const fn permits(
        self,
        requested: ResourceCount,
    ) -> bool {
        match self.0 {
            None => true,
            Some(maximum) => requested <= maximum,
        }
    }

    /// Validates this limit for a particular resource dimension.
    pub const fn validate(
        self,
        resource: LimitKind,
    ) -> Result<(), LimitError> {
        match self.0 {
            None => Ok(()),
            Some(0) => Err(LimitError::ZeroLimit { resource }),
            Some(_) => Ok(()),
        }
    }

    /// Checks a request against this limit.
    pub const fn check(
        self,
        resource: LimitKind,
        requested: ResourceCount,
    ) -> Result<(), LimitError> {
        match self.0 {
            None => Ok(()),
            Some(maximum) if requested <= maximum => Ok(()),
            Some(maximum) => Err(LimitError::Exceeded {
                resource,
                requested,
                maximum,
            }),
        }
    }

    /// Returns the smaller of two limits.
    ///
    /// Unlimited behaves as the mathematical infinity for this operation:
    ///
    /// ```text
    /// min(unlimited, bounded(n)) = bounded(n)
    /// min(unlimited, unlimited)   = unlimited
    /// ```
    #[must_use]
    pub const fn minimum(
        self,
        other: Self,
    ) -> Self {
        match (self.0, other.0) {
            (None, None) => Self::unlimited(),
            (None, Some(value)) => Self(Some(value)),
            (Some(value), None) => Self(Some(value)),
            (Some(left), Some(right)) => {
                if left <= right {
                    Self(Some(left))
                } else {
                    Self(Some(right))
                }
            }
        }
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl fmt::Display for Limit {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.0 {
            None => formatter.write_str("unlimited"),
            Some(value) => value.fmt(formatter),
        }
    }
}

/// Resource dimensions governed by [`ZqnLimits`].
///
/// This is a policy vocabulary, not a runtime-accounting vocabulary.
///
/// New resource dimensions should be added only when they represent a stable
/// cross-subsystem ZQN resource concept.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum LimitKind {
    /// Number of logical quantum resources.
    LogicalResources,

    /// Number of physical quantum resources.
    PhysicalResources,

    /// Number of classical resources associated with a workload.
    ClassicalResources,

    /// Number of quantum operations.
    Operations,

    /// Number of operations in one execution layer/depth unit.
    Depth,

    /// Number of noise/fault events.
    Faults,

    /// Number of resources participating in a single correlated-noise domain.
    CorrelatedResources,

    /// Number of entries in a discrete probability distribution.
    DistributionEntries,

    /// Number of stochastic samples/shots.
    Samples,

    /// Number of tensor elements.
    TensorElements,

    /// Number of matrix elements.
    MatrixElements,

    /// Number of channel operators.
    ChannelOperators,

    /// Number of model parameters.
    Parameters,

    /// Number of calibration entries.
    CalibrationEntries,

    /// Number of observations.
    Observations,

    /// Number of characterization experiments.
    Experiments,

    /// Number of queued or retained noise applications.
    NoiseApplications,

    /// Number of buffered events.
    BufferedEvents,

    /// Number of bytes.
    MemoryBytes,

    /// Number of serialized bytes.
    SerializedBytes,

    /// Number of worker tasks.
    ParallelTasks,

    /// Number of execution nodes.
    ExecutionNodes,

    /// Number of execution links.
    ExecutionLinks,

    /// Number of time steps.
    TimeSteps,

    /// Number of pulses.
    Pulses,

    /// Number of measurements.
    Measurements,

    /// Number of resets.
    Resets,

    /// Number of transport operations.
    TransportOperations,

    /// Number of resource dimensions in a composite request.
    CompositeResources,

    /// Number of verification operations.
    VerificationOperations,

    /// Unknown/custom resource dimension.
    ///
    /// This exists so low-level policy helpers can remain useful without
    /// forcing an unrelated subsystem to invent a ZQN-specific enum variant
    /// prematurely.
    Unknown,
}

impl LimitKind {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalResources => "logical_resources",
            Self::PhysicalResources => "physical_resources",
            Self::ClassicalResources => "classical_resources",
            Self::Operations => "operations",
            Self::Depth => "depth",
            Self::Faults => "faults",
            Self::CorrelatedResources => "correlated_resources",
            Self::DistributionEntries => "distribution_entries",
            Self::Samples => "samples",
            Self::TensorElements => "tensor_elements",
            Self::MatrixElements => "matrix_elements",
            Self::ChannelOperators => "channel_operators",
            Self::Parameters => "parameters",
            Self::CalibrationEntries => "calibration_entries",
            Self::Observations => "observations",
            Self::Experiments => "experiments",
            Self::NoiseApplications => "noise_applications",
            Self::BufferedEvents => "buffered_events",
            Self::MemoryBytes => "memory_bytes",
            Self::SerializedBytes => "serialized_bytes",
            Self::ParallelTasks => "parallel_tasks",
            Self::ExecutionNodes => "execution_nodes",
            Self::ExecutionLinks => "execution_links",
            Self::TimeSteps => "time_steps",
            Self::Pulses => "pulses",
            Self::Measurements => "measurements",
            Self::Resets => "resets",
            Self::TransportOperations => "transport_operations",
            Self::CompositeResources => "composite_resources",
            Self::VerificationOperations => "verification_operations",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for LimitKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced by ZQN resource-policy validation and admission checks.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum LimitError {
    /// A policy dimension was explicitly configured as zero.
    ZeroLimit {
        resource: LimitKind,
    },

    /// A requested amount exceeds a configured policy ceiling.
    Exceeded {
        resource: LimitKind,
        requested: ResourceCount,
        maximum: ResourceCount,
    },

    /// A checked resource calculation overflowed `u128`.
    ArithmeticOverflow {
        resource: LimitKind,
        operation: ArithmeticOperation,
    },

    /// A requested host allocation cannot be represented as `usize`.
    HostSizeOverflow {
        resource: LimitKind,
        requested: ResourceCount,
    },

    /// The policy schema is not supported.
    UnsupportedSchema {
        found: u32,
        expected: u32,
    },

    /// A policy relationship is invalid.
    Inconsistent {
        resource: LimitKind,
        related_resource: LimitKind,
        reason: &'static str,
    },

    /// A requested limit value is invalid for the selected resource.
    InvalidValue {
        resource: LimitKind,
        value: ResourceCount,
        reason: &'static str,
    },
}

impl fmt::Display for LimitError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroLimit { resource } => write!(
                formatter,
                "ZQN resource limit for {resource} must be greater than zero"
            ),

            Self::Exceeded {
                resource,
                requested,
                maximum,
            } => write!(
                formatter,
                "ZQN resource limit exceeded for {resource}: \
                 requested {requested}, maximum {maximum}"
            ),

            Self::ArithmeticOverflow {
                resource,
                operation,
            } => write!(
                formatter,
                "ZQN resource arithmetic overflow for {resource} \
                 during {operation}"
            ),

            Self::HostSizeOverflow {
                resource,
                requested,
            } => write!(
                formatter,
                "ZQN {resource} request {requested} cannot be represented \
                 by the host allocation type"
            ),

            Self::UnsupportedSchema {
                found,
                expected,
            } => write!(
                formatter,
                "unsupported ZQN limits schema {found}; expected {expected}"
            ),

            Self::Inconsistent {
                resource,
                related_resource,
                reason,
            } => write!(
                formatter,
                "inconsistent ZQN limits for {resource} and \
                 {related_resource}: {reason}"
            ),

            Self::InvalidValue {
                resource,
                value,
                reason,
            } => write!(
                formatter,
                "invalid ZQN limit value {value} for {resource}: {reason}"
            ),
        }
    }
}

impl std::error::Error for LimitError {}

/// Checked resource-arithmetic operation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum ArithmeticOperation {
    /// Addition.
    Addition,

    /// Multiplication.
    Multiplication,

    /// Accumulation.
    Accumulation,

    /// Conversion to host allocation size.
    HostSizeConversion,
}

impl fmt::Display for ArithmeticOperation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Addition => "addition",
            Self::Multiplication => "multiplication",
            Self::Accumulation => "accumulation",
            Self::HostSizeConversion => "host-size conversion",
        };

        formatter.write_str(value)
    }
}

/// Canonical declarative ZQN resource policy.
///
/// Every field is optional by design.
///
/// The default policy is therefore unlimited from ZQN's perspective.
///
/// This is deliberate. ZQN must not embed arbitrary finite ceilings that
/// become barriers to future quantum systems.
///
/// Concrete deployments should normally create a bounded policy at their
/// execution boundary, using the resources actually available to that
/// execution.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ZqnLimits {
    /// Serialized schema version.
    pub schema_version: u32,

    /// Maximum logical resources.
    pub logical_resources: Limit,

    /// Maximum physical resources.
    pub physical_resources: Limit,

    /// Maximum classical resources.
    pub classical_resources: Limit,

    /// Maximum quantum operations.
    pub operations: Limit,

    /// Maximum execution depth.
    pub depth: Limit,

    /// Maximum generated fault/noise events.
    pub faults: Limit,

    /// Maximum resources in one correlated-noise domain.
    pub correlated_resources: Limit,

    /// Maximum discrete distribution entries.
    pub distribution_entries: Limit,

    /// Maximum stochastic samples/shots.
    pub samples: Limit,

    /// Maximum tensor elements.
    pub tensor_elements: Limit,

    /// Maximum matrix elements.
    pub matrix_elements: Limit,

    /// Maximum channel operators.
    pub channel_operators: Limit,

    /// Maximum model parameters.
    pub parameters: Limit,

    /// Maximum calibration entries.
    pub calibration_entries: Limit,

    /// Maximum observations.
    pub observations: Limit,

    /// Maximum characterization experiments.
    pub experiments: Limit,

    /// Maximum materialized noise applications.
    pub noise_applications: Limit,

    /// Maximum buffered events.
    pub buffered_events: Limit,

    /// Maximum memory that the ZQN operation may request.
    pub memory_bytes: Limit,

    /// Maximum serialized representation size.
    pub serialized_bytes: Limit,

    /// Maximum parallel tasks requested by the ZQN operation.
    pub parallel_tasks: Limit,

    /// Maximum execution nodes.
    pub execution_nodes: Limit,

    /// Maximum execution links.
    pub execution_links: Limit,

    /// Maximum modeled time steps.
    pub time_steps: Limit,

    /// Maximum pulse records.
    pub pulses: Limit,

    /// Maximum measurement records.
    pub measurements: Limit,

    /// Maximum reset records.
    pub resets: Limit,

    /// Maximum transport operations.
    pub transport_operations: Limit,

    /// Maximum resource dimensions in a composite request.
    pub composite_resources: Limit,

    /// Maximum mathematical verification operations.
    pub verification_operations: Limit,
}

impl Default for ZqnLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl ZqnLimits {
    /// Creates a completely unlimited ZQN policy.
    ///
    /// "Unlimited" means only that ZQN adds no finite ceiling.
    ///
    /// It does not bypass:
    ///
    /// - runtime limits;
    /// - memory limits;
    /// - target capabilities;
    /// - operating-system limits;
    /// - physical hardware capacity.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            schema_version: ZQN_LIMITS_SCHEMA_VERSION,

            logical_resources: Limit::unlimited(),
            physical_resources: Limit::unlimited(),
            classical_resources: Limit::unlimited(),
            operations: Limit::unlimited(),
            depth: Limit::unlimited(),
            faults: Limit::unlimited(),
            correlated_resources: Limit::unlimited(),
            distribution_entries: Limit::unlimited(),
            samples: Limit::unlimited(),
            tensor_elements: Limit::unlimited(),
            matrix_elements: Limit::unlimited(),
            channel_operators: Limit::unlimited(),
            parameters: Limit::unlimited(),
            calibration_entries: Limit::unlimited(),
            observations: Limit::unlimited(),
            experiments: Limit::unlimited(),
            noise_applications: Limit::unlimited(),
            buffered_events: Limit::unlimited(),
            memory_bytes: Limit::unlimited(),
            serialized_bytes: Limit::unlimited(),
            parallel_tasks: Limit::unlimited(),
            execution_nodes: Limit::unlimited(),
            execution_links: Limit::unlimited(),
            time_steps: Limit::unlimited(),
            pulses: Limit::unlimited(),
            measurements: Limit::unlimited(),
            resets: Limit::unlimited(),
            transport_operations: Limit::unlimited(),
            composite_resources: Limit::unlimited(),
            verification_operations: Limit::unlimited(),
        }
    }

    /// Alias for [`Self::unlimited`].
    #[must_use]
    pub const fn new() -> Self {
        Self::unlimited()
    }

    /// Validates the entire policy.
    ///
    /// No finite resource value is required. Unlimited dimensions are valid.
    pub fn validate(&self) -> Result<(), LimitError> {
        if self.schema_version != ZQN_LIMITS_SCHEMA_VERSION {
            return Err(LimitError::UnsupportedSchema {
                found: self.schema_version,
                expected: ZQN_LIMITS_SCHEMA_VERSION,
            });
        }

        self.logical_resources
            .validate(LimitKind::LogicalResources)?;

        self.physical_resources
            .validate(LimitKind::PhysicalResources)?;

        self.classical_resources
            .validate(LimitKind::ClassicalResources)?;

        self.operations
            .validate(LimitKind::Operations)?;

        self.depth
            .validate(LimitKind::Depth)?;

        self.faults
            .validate(LimitKind::Faults)?;

        self.correlated_resources
            .validate(LimitKind::CorrelatedResources)?;

        self.distribution_entries
            .validate(LimitKind::DistributionEntries)?;

        self.samples
            .validate(LimitKind::Samples)?;

        self.tensor_elements
            .validate(LimitKind::TensorElements)?;

        self.matrix_elements
            .validate(LimitKind::MatrixElements)?;

        self.channel_operators
            .validate(LimitKind::ChannelOperators)?;

        self.parameters
            .validate(LimitKind::Parameters)?;

        self.calibration_entries
            .validate(LimitKind::CalibrationEntries)?;

        self.observations
            .validate(LimitKind::Observations)?;

        self.experiments
            .validate(LimitKind::Experiments)?;

        self.noise_applications
            .validate(LimitKind::NoiseApplications)?;

        self.buffered_events
            .validate(LimitKind::BufferedEvents)?;

        self.memory_bytes
            .validate(LimitKind::MemoryBytes)?;

        self.serialized_bytes
            .validate(LimitKind::SerializedBytes)?;

        self.parallel_tasks
            .validate(LimitKind::ParallelTasks)?;

        self.execution_nodes
            .validate(LimitKind::ExecutionNodes)?;

        self.execution_links
            .validate(LimitKind::ExecutionLinks)?;

        self.time_steps
            .validate(LimitKind::TimeSteps)?;

        self.pulses
            .validate(LimitKind::Pulses)?;

        self.measurements
            .validate(LimitKind::Measurements)?;

        self.resets
            .validate(LimitKind::Resets)?;

        self.transport_operations
            .validate(LimitKind::TransportOperations)?;

        self.composite_resources
            .validate(LimitKind::CompositeResources)?;

        self.verification_operations
            .validate(LimitKind::VerificationOperations)?;

        Ok(())
    }

    /// Returns the configured limit for a resource dimension.
    #[must_use]
    pub const fn limit(
        &self,
        resource: LimitKind,
    ) -> Limit {
        match resource {
            LimitKind::LogicalResources => self.logical_resources,
            LimitKind::PhysicalResources => self.physical_resources,
            LimitKind::ClassicalResources => self.classical_resources,
            LimitKind::Operations => self.operations,
            LimitKind::Depth => self.depth,
            LimitKind::Faults => self.faults,
            LimitKind::CorrelatedResources => self.correlated_resources,
            LimitKind::DistributionEntries => self.distribution_entries,
            LimitKind::Samples => self.samples,
            LimitKind::TensorElements => self.tensor_elements,
            LimitKind::MatrixElements => self.matrix_elements,
            LimitKind::ChannelOperators => self.channel_operators,
            LimitKind::Parameters => self.parameters,
            LimitKind::CalibrationEntries => self.calibration_entries,
            LimitKind::Observations => self.observations,
            LimitKind::Experiments => self.experiments,
            LimitKind::NoiseApplications => self.noise_applications,
            LimitKind::BufferedEvents => self.buffered_events,
            LimitKind::MemoryBytes => self.memory_bytes,
            LimitKind::SerializedBytes => self.serialized_bytes,
            LimitKind::ParallelTasks => self.parallel_tasks,
            LimitKind::ExecutionNodes => self.execution_nodes,
            LimitKind::ExecutionLinks => self.execution_links,
            LimitKind::TimeSteps => self.time_steps,
            LimitKind::Pulses => self.pulses,
            LimitKind::Measurements => self.measurements,
            LimitKind::Resets => self.resets,
            LimitKind::TransportOperations => self.transport_operations,
            LimitKind::CompositeResources => self.composite_resources,
            LimitKind::VerificationOperations => self.verification_operations,
            LimitKind::Unknown => Limit::unlimited(),
        }
    }

    /// Checks one requested resource amount.
    pub const fn check(
        &self,
        resource: LimitKind,
        requested: ResourceCount,
    ) -> Result<(), LimitError> {
        self.limit(resource).check(resource, requested)
    }

    /// Checks whether a request is permitted without returning an error.
    #[must_use]
    pub const fn permits(
        &self,
        resource: LimitKind,
        requested: ResourceCount,
    ) -> bool {
        self.limit(resource).permits(requested)
    }

    /// Validates a resource count represented by `usize`.
    pub const fn check_usize(
        &self,
        resource: LimitKind,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check(resource, requested as ResourceCount)
    }

    /// Validates a resource count represented by `u64`.
    pub const fn check_u64(
        &self,
        resource: LimitKind,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check(resource, requested as ResourceCount)
    }

    /// Validates a resource count represented by `u128`.
    pub const fn check_u128(
        &self,
        resource: LimitKind,
        requested: u128,
    ) -> Result<(), LimitError> {
        self.check(resource, requested)
    }

    /// Checks an allocation-sized value.
    ///
    /// This method deliberately performs two separate checks:
    ///
    /// 1. ZQN policy admission;
    /// 2. host `usize` representability.
    ///
    /// Passing the first does not guarantee that allocation will succeed.
    pub fn check_allocation(
        &self,
        resource: LimitKind,
        requested: ResourceCount,
    ) -> Result<usize, LimitError> {
        self.check(resource, requested)?;

        usize::try_from(requested).map_err(|_| {
            LimitError::HostSizeOverflow {
                resource,
                requested,
            }
        })
    }

    /// Checks a derived resource represented by `a + b`.
    pub const fn check_add(
        &self,
        resource: LimitKind,
        a: ResourceCount,
        b: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        let total = match a.checked_add(b) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Addition,
                    },
                )
            }
        };

        self.check(resource, total)?;
        Ok(total)
    }

    /// Checks a derived resource represented by `a * b`.
    pub const fn check_mul(
        &self,
        resource: LimitKind,
        a: ResourceCount,
        b: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        let total = match a.checked_mul(b) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        self.check(resource, total)?;
        Ok(total)
    }

    /// Checks a product of three resource dimensions.
    ///
    /// This is useful for expressions such as:
    ///
    /// ```text
    /// resources × samples × operations
    /// ```
    ///
    /// The multiplication is checked before the result reaches the policy
    /// comparison.
    pub const fn check_mul3(
        &self,
        resource: LimitKind,
        a: ResourceCount,
        b: ResourceCount,
        c: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        let first = match a.checked_mul(b) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        let total = match first.checked_mul(c) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        self.check(resource, total)?;
        Ok(total)
    }

    /// Checks a product of four resource dimensions.
    pub const fn check_mul4(
        &self,
        resource: LimitKind,
        a: ResourceCount,
        b: ResourceCount,
        c: ResourceCount,
        d: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        let first = match a.checked_mul(b) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        let second = match first.checked_mul(c) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        let total = match second.checked_mul(d) {
            Some(value) => value,
            None => {
                return Err(
                    LimitError::ArithmeticOverflow {
                        resource,
                        operation: ArithmeticOperation::Multiplication,
                    },
                )
            }
        };

        self.check(resource, total)?;
        Ok(total)
    }

    /// Checks a sequence of resource amounts using checked accumulation.
    ///
    /// The iterator is consumed incrementally so callers do not need to
    /// construct an additional collection merely for preflight.
    pub fn check_sum<I>(
        &self,
        resource: LimitKind,
        values: I,
    ) -> Result<ResourceCount, LimitError>
    where
        I: IntoIterator<Item = ResourceCount>,
    {
        let mut total = 0_u128;

        for value in values {
            total = match total.checked_add(value) {
                Some(next) => next,
                None => {
                    return Err(
                        LimitError::ArithmeticOverflow {
                            resource,
                            operation: ArithmeticOperation::Accumulation,
                        },
                    )
                }
            };

            if let Some(maximum) =
                self.limit(resource).maximum()
            {
                if total > maximum {
                    return Err(
                        LimitError::Exceeded {
                            resource,
                            requested: total,
                            maximum,
                        },
                    );
                }
            }
        }

        Ok(total)
    }

    /// Returns the effective limit after applying a stricter external policy.
    ///
    /// This is useful when multiple policy layers exist:
    ///
    /// ```text
    /// global policy
    ///      +
    /// deployment policy
    ///      +
    /// job policy
    ///      +
    /// operation policy
    ///      ↓
    /// effective policy
    /// ```
    ///
    /// The operation is monotonic: a stricter policy can only reduce
    /// permissions, never expand them.
    #[must_use]
    pub const fn intersect(
        &self,
        other: &Self,
    ) -> Self {
        Self {
            schema_version: ZQN_LIMITS_SCHEMA_VERSION,

            logical_resources: self
                .logical_resources
                .minimum(other.logical_resources),

            physical_resources: self
                .physical_resources
                .minimum(other.physical_resources),

            classical_resources: self
                .classical_resources
                .minimum(other.classical_resources),

            operations: self
                .operations
                .minimum(other.operations),

            depth: self
                .depth
                .minimum(other.depth),

            faults: self
                .faults
                .minimum(other.faults),

            correlated_resources: self
                .correlated_resources
                .minimum(other.correlated_resources),

            distribution_entries: self
                .distribution_entries
                .minimum(other.distribution_entries),

            samples: self
                .samples
                .minimum(other.samples),

            tensor_elements: self
                .tensor_elements
                .minimum(other.tensor_elements),

            matrix_elements: self
                .matrix_elements
                .minimum(other.matrix_elements),

            channel_operators: self
                .channel_operators
                .minimum(other.channel_operators),

            parameters: self
                .parameters
                .minimum(other.parameters),

            calibration_entries: self
                .calibration_entries
                .minimum(other.calibration_entries),

            observations: self
                .observations
                .minimum(other.observations),

            experiments: self
                .experiments
                .minimum(other.experiments),

            noise_applications: self
                .noise_applications
                .minimum(other.noise_applications),

            buffered_events: self
                .buffered_events
                .minimum(other.buffered_events),

            memory_bytes: self
                .memory_bytes
                .minimum(other.memory_bytes),

            serialized_bytes: self
                .serialized_bytes
                .minimum(other.serialized_bytes),

            parallel_tasks: self
                .parallel_tasks
                .minimum(other.parallel_tasks),

            execution_nodes: self
                .execution_nodes
                .minimum(other.execution_nodes),

            execution_links: self
                .execution_links
                .minimum(other.execution_links),

            time_steps: self
                .time_steps
                .minimum(other.time_steps),

            pulses: self
                .pulses
                .minimum(other.pulses),

            measurements: self
                .measurements
                .minimum(other.measurements),

            resets: self
                .resets
                .minimum(other.resets),

            transport_operations: self
                .transport_operations
                .minimum(other.transport_operations),

            composite_resources: self
                .composite_resources
                .minimum(other.composite_resources),

            verification_operations: self
                .verification_operations
                .minimum(other.verification_operations),
        }
    }

    /// Returns the effective limit for one resource after applying another
    /// limit.
    #[must_use]
    pub const fn effective_limit(
        &self,
        resource: LimitKind,
        external: Limit,
    ) -> Limit {
        self.limit(resource).minimum(external)
    }

    /// Validates this policy against another policy's schema version.
    ///
    /// Policy intersection itself is schema-stable because both policies are
    /// represented by the current structure, but this explicit check prevents
    /// accidentally treating an unsupported deserialized policy as valid.
    pub fn validate_compatible(
        &self,
        other: &Self,
    ) -> Result<(), LimitError> {
        self.validate()?;
        other.validate()?;

        Ok(())
    }
}

/// Builder for [`ZqnLimits`].
///
/// The builder exists so callers can construct an explicit bounded execution
/// policy without modifying the policy structure itself.
///
/// Every unspecified dimension remains unlimited.
#[derive(Clone, Copy, Debug, Default)]
pub struct ZqnLimitsBuilder {
    limits: ZqnLimits,
}

impl ZqnLimitsBuilder {
    /// Creates a new builder with every dimension unlimited.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: ZqnLimits::unlimited(),
        }
    }

    /// Sets logical-resource limit.
    pub const fn logical_resources(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.logical_resources =
            Limit::bounded_for(
                LimitKind::LogicalResources,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets physical-resource limit.
    pub const fn physical_resources(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.physical_resources =
            Limit::bounded_for(
                LimitKind::PhysicalResources,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets classical-resource limit.
    pub const fn classical_resources(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.classical_resources =
            Limit::bounded_for(
                LimitKind::ClassicalResources,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets operation limit.
    pub const fn operations(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.operations =
            Limit::bounded_for(
                LimitKind::Operations,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets depth limit.
    pub const fn depth(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.depth =
            Limit::bounded_for(
                LimitKind::Depth,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets fault limit.
    pub const fn faults(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.faults =
            Limit::bounded_for(
                LimitKind::Faults,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets correlated-resource limit.
    pub const fn correlated_resources(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.correlated_resources =
            Limit::bounded_for(
                LimitKind::CorrelatedResources,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets distribution-entry limit.
    pub const fn distribution_entries(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.distribution_entries =
            Limit::bounded_for(
                LimitKind::DistributionEntries,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets sample/shot limit.
    pub const fn samples(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.samples =
            Limit::bounded_for(
                LimitKind::Samples,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets tensor-element limit.
    pub const fn tensor_elements(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.tensor_elements =
            Limit::bounded_for(
                LimitKind::TensorElements,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets matrix-element limit.
    pub const fn matrix_elements(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.matrix_elements =
            Limit::bounded_for(
                LimitKind::MatrixElements,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets channel-operator limit.
    pub const fn channel_operators(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.channel_operators =
            Limit::bounded_for(
                LimitKind::ChannelOperators,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets parameter limit.
    pub const fn parameters(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.parameters =
            Limit::bounded_for(
                LimitKind::Parameters,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets calibration-entry limit.
    pub const fn calibration_entries(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.calibration_entries =
            Limit::bounded_for(
                LimitKind::CalibrationEntries,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets observation limit.
    pub const fn observations(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.observations =
            Limit::bounded_for(
                LimitKind::Observations,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets experiment limit.
    pub const fn experiments(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.experiments =
            Limit::bounded_for(
                LimitKind::Experiments,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets materialized noise-application limit.
    pub const fn noise_applications(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.noise_applications =
            Limit::bounded_for(
                LimitKind::NoiseApplications,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets buffered-event limit.
    pub const fn buffered_events(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.buffered_events =
            Limit::bounded_for(
                LimitKind::BufferedEvents,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets memory-byte limit.
    pub const fn memory_bytes(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.memory_bytes =
            Limit::bounded_for(
                LimitKind::MemoryBytes,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets serialized-byte limit.
    pub const fn serialized_bytes(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.serialized_bytes =
            Limit::bounded_for(
                LimitKind::SerializedBytes,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets parallel-task limit.
    pub const fn parallel_tasks(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.parallel_tasks =
            Limit::bounded_for(
                LimitKind::ParallelTasks,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets execution-node limit.
    pub const fn execution_nodes(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.execution_nodes =
            Limit::bounded_for(
                LimitKind::ExecutionNodes,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets execution-link limit.
    pub const fn execution_links(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.execution_links =
            Limit::bounded_for(
                LimitKind::ExecutionLinks,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets time-step limit.
    pub const fn time_steps(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.time_steps =
            Limit::bounded_for(
                LimitKind::TimeSteps,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets pulse limit.
    pub const fn pulses(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.pulses =
            Limit::bounded_for(
                LimitKind::Pulses,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets measurement limit.
    pub const fn measurements(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.measurements =
            Limit::bounded_for(
                LimitKind::Measurements,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets reset limit.
    pub const fn resets(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.resets =
            Limit::bounded_for(
                LimitKind::Resets,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets transport-operation limit.
    pub const fn transport_operations(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.transport_operations =
            Limit::bounded_for(
                LimitKind::TransportOperations,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets composite-resource limit.
    pub const fn composite_resources(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.composite_resources =
            Limit::bounded_for(
                LimitKind::CompositeResources,
                maximum,
            )?;

        Ok(self)
    }

    /// Sets verification-operation limit.
    pub const fn verification_operations(
        mut self,
        maximum: ResourceCount,
    ) -> Result<Self, LimitError> {
        self.limits.verification_operations =
            Limit::bounded_for(
                LimitKind::VerificationOperations,
                maximum,
            )?;

        Ok(self)
    }

    /// Replaces one resource dimension.
    ///
    /// `None` means unlimited.
    pub const fn with_limit(
        mut self,
        resource: LimitKind,
        maximum: Option<ResourceCount>,
    ) -> Result<Self, LimitError> {
        let limit =
            Limit::from_option(resource, maximum)?;

        match resource {
            LimitKind::LogicalResources => {
                self.limits.logical_resources = limit;
            }

            LimitKind::PhysicalResources => {
                self.limits.physical_resources = limit;
            }

            LimitKind::ClassicalResources => {
                self.limits.classical_resources = limit;
            }

            LimitKind::Operations => {
                self.limits.operations = limit;
            }

            LimitKind::Depth => {
                self.limits.depth = limit;
            }

            LimitKind::Faults => {
                self.limits.faults = limit;
            }

            LimitKind::CorrelatedResources => {
                self.limits.correlated_resources = limit;
            }

            LimitKind::DistributionEntries => {
                self.limits.distribution_entries = limit;
            }

            LimitKind::Samples => {
                self.limits.samples = limit;
            }

            LimitKind::TensorElements => {
                self.limits.tensor_elements = limit;
            }

            LimitKind::MatrixElements => {
                self.limits.matrix_elements = limit;
            }

            LimitKind::ChannelOperators => {
                self.limits.channel_operators = limit;
            }

            LimitKind::Parameters => {
                self.limits.parameters = limit;
            }

            LimitKind::CalibrationEntries => {
                self.limits.calibration_entries = limit;
            }

            LimitKind::Observations => {
                self.limits.observations = limit;
            }

            LimitKind::Experiments => {
                self.limits.experiments = limit;
            }

            LimitKind::NoiseApplications => {
                self.limits.noise_applications = limit;
            }

            LimitKind::BufferedEvents => {
                self.limits.buffered_events = limit;
            }

            LimitKind::MemoryBytes => {
                self.limits.memory_bytes = limit;
            }

            LimitKind::SerializedBytes => {
                self.limits.serialized_bytes = limit;
            }

            LimitKind::ParallelTasks => {
                self.limits.parallel_tasks = limit;
            }

            LimitKind::ExecutionNodes => {
                self.limits.execution_nodes = limit;
            }

            LimitKind::ExecutionLinks => {
                self.limits.execution_links = limit;
            }

            LimitKind::TimeSteps => {
                self.limits.time_steps = limit;
            }

            LimitKind::Pulses => {
                self.limits.pulses = limit;
            }

            LimitKind::Measurements => {
                self.limits.measurements = limit;
            }

            LimitKind::Resets => {
                self.limits.resets = limit;
            }

            LimitKind::TransportOperations => {
                self.limits.transport_operations = limit;
            }

            LimitKind::CompositeResources => {
                self.limits.composite_resources = limit;
            }

            LimitKind::VerificationOperations => {
                self.limits.verification_operations = limit;
            }

            LimitKind::Unknown => {
                return Err(
                    LimitError::InvalidValue {
                        resource,
                        value: maximum.unwrap_or(0),
                        reason:
                            "unknown resources cannot be configured \
                             through the canonical policy",
                    },
                );
            }
        }

        Ok(self)
    }

    /// Finishes the builder after validating the resulting policy.
    pub fn build(self) -> Result<ZqnLimits, LimitError> {
        self.limits.validate()?;
        Ok(self.limits)
    }
}

/// A compact resource request used by generic admission paths.
///
/// This deliberately does not contain quantum-specific identities.
///
/// It allows higher-level systems to preflight multiple dimensions without
/// making this module depend on IR, hardware, routing or simulation types.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ResourceRequest {
    /// Resource dimension.
    pub resource: LimitKind,

    /// Requested quantity.
    pub amount: ResourceCount,
}

impl ResourceRequest {
    /// Creates a resource request.
    #[must_use]
    pub const fn new(
        resource: LimitKind,
        amount: ResourceCount,
    ) -> Self {
        Self {
            resource,
            amount,
        }
    }

    /// Validates the request against a policy.
    pub const fn check(
        self,
        limits: &ZqnLimits,
    ) -> Result<(), LimitError> {
        limits.check(
            self.resource,
            self.amount,
        )
    }
}

/// Checks multiple resource requests.
///
/// Requests are evaluated incrementally. No additional collection is
/// allocated.
pub fn check_requests<I>(
    limits: &ZqnLimits,
    requests: I,
) -> Result<(), LimitError>
where
    I: IntoIterator<Item = ResourceRequest>,
{
    limits.validate()?;

    for request in requests {
        request.check(limits)?;
    }

    Ok(())
}

/// Checks whether a resource count can be represented by the host allocation
/// type after passing the ZQN policy.
pub fn check_allocation_size(
    limits: &ZqnLimits,
    resource: LimitKind,
    requested: ResourceCount,
) -> Result<usize, LimitError> {
    limits.check_allocation(
        resource,
        requested,
    )
}

/// Creates an explicitly bounded limit.
///
/// This helper is convenient for callers that want a named resource in the
/// resulting diagnostic.
pub const fn bounded(
    resource: LimitKind,
    maximum: ResourceCount,
) -> Result<Limit, LimitError> {
    Limit::bounded_for(
        resource,
        maximum,
    )
}

/// Creates an unlimited limit.
#[must_use]
pub const fn unlimited() -> Limit {
    Limit::unlimited()
}

/// Returns the minimum of two limits.
///
/// Unlimited behaves as mathematical infinity.
#[must_use]
pub const fn minimum(
    left: Limit,
    right: Limit,
) -> Limit {
    left.minimum(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_unlimited() {
        let limits = ZqnLimits::default();

        assert_eq!(
            limits.schema_version,
            ZQN_LIMITS_SCHEMA_VERSION
        );

        assert!(limits
            .logical_resources
            .is_unlimited());

        assert!(limits
            .physical_resources
            .is_unlimited());

        assert!(limits.operations.is_unlimited());
        assert!(limits.faults.is_unlimited());
        assert!(limits.samples.is_unlimited());
        assert!(limits.memory_bytes.is_unlimited());

        assert!(limits.validate().is_ok());
    }

    #[test]
    fn zero_limit_is_rejected() {
        let result = Limit::bounded_for(
            LimitKind::Operations,
            0,
        );

        assert_eq!(
            result,
            Err(LimitError::ZeroLimit {
                resource: LimitKind::Operations,
            })
        );
    }

    #[test]
    fn unlimited_permits_any_u128_value() {
        let limit = Limit::unlimited();

        assert!(limit.permits(0));
        assert!(limit.permits(1));
        assert!(limit.permits(u128::MAX));
    }

    #[test]
    fn bounded_limit_accepts_boundary() {
        let limit = Limit::bounded_for(
            LimitKind::Operations,
            100,
        )
        .expect("positive limit must be valid");

        assert!(limit.permits(100));
        assert!(!limit.permits(101));
    }

    #[test]
    fn bounded_limit_rejects_excess() {
        let limit = Limit::bounded_for(
            LimitKind::Operations,
            100,
        )
        .expect("positive limit must be valid");

        let result = limit.check(
            LimitKind::Operations,
            101,
        );

        assert_eq!(
            result,
            Err(LimitError::Exceeded {
                resource: LimitKind::Operations,
                requested: 101,
                maximum: 100,
            })
        );
    }

    #[test]
    fn policy_check_uses_selected_dimension() {
        let limits = ZqnLimitsBuilder::new()
            .operations(10)
            .expect("valid operation limit")
            .faults(20)
            .expect("valid fault limit")
            .build()
            .expect("valid policy");

        assert!(limits
            .check(
                LimitKind::Operations,
                10
            )
            .is_ok());

        assert!(limits
            .check(
                LimitKind::Faults,
                20
            )
            .is_ok());

        assert!(limits
            .check(
                LimitKind::Operations,
                11
            )
            .is_err());
    }

    #[test]
    fn checked_add_rejects_u128_overflow() {
        let limits = ZqnLimits::unlimited();

        let result = limits.check_add(
            LimitKind::Operations,
            u128::MAX,
            1,
        );

        assert_eq!(
            result,
            Err(LimitError::ArithmeticOverflow {
                resource: LimitKind::Operations,
                operation: ArithmeticOperation::Addition,
            })
        );
    }

    #[test]
    fn checked_multiplication_rejects_u128_overflow() {
        let limits = ZqnLimits::unlimited();

        let result = limits.check_mul(
            LimitKind::TensorElements,
            u128::MAX,
            2,
        );

        assert_eq!(
            result,
            Err(LimitError::ArithmeticOverflow {
                resource: LimitKind::TensorElements,
                operation: ArithmeticOperation::Multiplication,
            })
        );
    }

    #[test]
    fn checked_product_respects_policy() {
        let limits = ZqnLimitsBuilder::new()
            .tensor_elements(100)
            .expect("valid tensor limit")
            .build()
            .expect("valid policy");

        let result = limits.check_mul(
            LimitKind::TensorElements,
            10,
            10,
        );

        assert_eq!(result, Ok(100));

        let result = limits.check_mul(
            LimitKind::TensorElements,
            10,
            11,
        );

        assert_eq!(
            result,
            Err(LimitError::Exceeded {
                resource: LimitKind::TensorElements,
                requested: 110,
                maximum: 100,
            })
        );
    }

    #[test]
    fn checked_sum_stops_at_policy_boundary() {
        let limits = ZqnLimitsBuilder::new()
            .faults(100)
            .expect("valid fault limit")
            .build()
            .expect("valid policy");

        let values = [20_u128, 30, 40, 11];

        let result = limits.check_sum(
            LimitKind::Faults,
            values,
        );

        assert_eq!(
            result,
            Err(LimitError::Exceeded {
                resource: LimitKind::Faults,
                requested: 101,
                maximum: 100,
            })
        );
    }

    #[test]
    fn checked_sum_detects_u128_overflow() {
        let limits = ZqnLimits::unlimited();

        let values = [
            u128::MAX,
            1_u128,
        ];

        let result = limits.check_sum(
            LimitKind::Operations,
            values,
        );

        assert_eq!(
            result,
            Err(LimitError::ArithmeticOverflow {
                resource: LimitKind::Operations,
                operation: ArithmeticOperation::Accumulation,
            })
        );
    }

    #[test]
    fn host_size_conversion_is_checked() {
        let limits = ZqnLimits::unlimited();

        let requested =
            usize::MAX as ResourceCount + 1;

        let result =
            limits.check_allocation(
                LimitKind::MemoryBytes,
                requested,
            );

        if usize::MAX < u128::MAX as usize {
            assert!(matches!(
                result,
                Err(
                    LimitError::HostSizeOverflow {
                        resource:
                            LimitKind::MemoryBytes,
                        ..
                    }
                )
            ));
        } else {
            assert_eq!(
                result,
                Err(
                    LimitError::HostSizeOverflow {
                        resource:
                            LimitKind::MemoryBytes,
                        ..
                    }
                )
            );
        }
    }

    #[test]
    fn policy_intersection_never_expands_permissions() {
        let left = ZqnLimitsBuilder::new()
            .operations(100)
            .expect("valid limit")
            .samples(1_000)
            .expect("valid limit")
            .build()
            .expect("valid policy");

        let right = ZqnLimitsBuilder::new()
            .operations(50)
            .expect("valid limit")
            .build()
            .expect("valid policy");

        let effective =
            left.intersect(&right);

        assert_eq!(
            effective
                .operations
                .maximum(),
            Some(50)
        );

        assert_eq!(
            effective
                .samples
                .maximum(),
            Some(1_000)
        );
    }

    #[test]
    fn unlimited_is_identity_for_intersection() {
        let bounded_policy =
            ZqnLimitsBuilder::new()
                .operations(100)
                .expect("valid limit")
                .build()
                .expect("valid policy");

        let unlimited_policy =
            ZqnLimits::unlimited();

        let effective =
            bounded_policy
                .intersect(&unlimited_policy);

        assert_eq!(
            effective.operations.maximum(),
            Some(100)
        );

        assert!(effective
            .faults
            .is_unlimited());
    }

    #[test]
    fn builder_supports_explicit_unlimited_dimension() {
        let limits = ZqnLimitsBuilder::new()
            .operations(100)
            .expect("valid limit")
            .with_limit(
                LimitKind::Samples,
                None,
            )
            .expect("unlimited is valid")
            .build()
            .expect("valid policy");

        assert_eq!(
            limits.operations.maximum(),
            Some(100)
        );

        assert!(limits.samples.is_unlimited());
    }

    #[test]
    fn builder_rejects_zero() {
        let result =
            ZqnLimitsBuilder::new()
                .operations(0);

        assert_eq!(
            result,
            Err(LimitError::ZeroLimit {
                resource: LimitKind::Operations,
            })
        );
    }

    #[test]
    fn unknown_resource_is_not_configurable() {
        let result =
            ZqnLimitsBuilder::new()
                .with_limit(
                    LimitKind::Unknown,
                    Some(10),
                );

        assert!(matches!(
            result,
            Err(
                LimitError::InvalidValue {
                    resource:
                        LimitKind::Unknown,
                    ..
                }
            )
        ));
    }

    #[test]
    fn resource_request_is_deterministic() {
        let limits = ZqnLimitsBuilder::new()
            .faults(10)
            .expect("valid limit")
            .build()
            .expect("valid policy");

        let request =
            ResourceRequest::new(
                LimitKind::Faults,
                10,
            );

        assert!(request.check(&limits).is_ok());
        assert!(request.check(&limits).is_ok());
    }

    #[test]
    fn multiple_requests_are_checked_without_allocation() {
        let limits = ZqnLimitsBuilder::new()
            .operations(100)
            .expect("valid limit")
            .faults(200)
            .expect("valid limit")
            .build()
            .expect("valid policy");

        let requests = [
            ResourceRequest::new(
                LimitKind::Operations,
                100,
            ),
            ResourceRequest::new(
                LimitKind::Faults,
                200,
            ),
        ];

        assert!(
            check_requests(
                &limits,
                requests
            )
            .is_ok()
        );
    }

    #[test]
    fn schema_mismatch_is_rejected() {
        let mut limits =
            ZqnLimits::unlimited();

        limits.schema_version =
            ZQN_LIMITS_SCHEMA_VERSION + 1;

        let result = limits.validate();

        assert_eq!(
            result,
            Err(
                LimitError::UnsupportedSchema {
                    found:
                        ZQN_LIMITS_SCHEMA_VERSION
                            + 1,
                    expected:
                        ZQN_LIMITS_SCHEMA_VERSION,
                }
            )
        );
    }

    #[test]
    fn display_is_stable() {
        assert_eq!(
            LimitKind::PhysicalResources
                .to_string(),
            "physical_resources"
        );

        assert_eq!(
            LimitKind::TensorElements
                .to_string(),
            "tensor_elements"
        );

        assert_eq!(
            Limit::unlimited().to_string(),
            "unlimited"
        );

        assert_eq!(
            Limit::bounded_for(
                LimitKind::Operations,
                42,
            )
            .expect("valid")
            .to_string(),
            "42"
        );
    }

    #[test]
    fn serde_round_trip_preserves_policy() {
        let original =
            ZqnLimitsBuilder::new()
                .logical_resources(1_000)
                .expect("valid limit")
                .physical_resources(2_000)
                .expect("valid limit")
                .operations(50_000)
                .expect("valid limit")
                .faults(100_000)
                .expect("valid limit")
                .samples(1_000_000)
                .expect("valid limit")
                .memory_bytes(
                    1024 * 1024,
                )
                .expect("valid limit")
                .build()
                .expect("valid policy");

        let encoded =
            serde_json::to_string(&original)
                .expect("serialization must succeed");

        let decoded =
            serde_json::from_str::<ZqnLimits>(
                &encoded,
            )
            .expect(
                "deserialization must succeed",
            );

        assert_eq!(
            original,
            decoded
        );
    }

    #[test]
    fn huge_finite_values_are_valid_policy_values() {
        let maximum =
            u128::MAX;

        let limits =
            ZqnLimitsBuilder::new()
                .operations(maximum)
                .expect("u128::MAX is a valid positive policy")
                .faults(maximum)
                .expect("u128::MAX is a valid positive policy")
                .build()
                .expect("policy is valid");

        assert_eq!(
            limits.operations.maximum(),
            Some(maximum)
        );

        assert!(
            limits
                .check(
                    LimitKind::Operations,
                    maximum
                )
                .is_ok()
        );
    }

    #[test]
    fn no_finite_default_machine_size_exists() {
        let limits =
            ZqnLimits::default();

        assert!(
            limits
                .physical_resources
                .is_unlimited()
        );

        assert!(
            limits
                .logical_resources
                .is_unlimited()
        );

        assert!(
            limits
                .operations
                .is_unlimited()
        );

        assert!(
            limits
                .faults
                .is_unlimited()
        );

        assert!(
            limits
                .samples
                .is_unlimited()
        );

        assert!(
            limits
                .tensor_elements
                .is_unlimited()
        );
    }
}