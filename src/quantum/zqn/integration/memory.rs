//! Zamani Quantum Noise (ZQN) — Quantum Memory Integration
//!
//! This module defines the provider-neutral integration boundary between
//! `quantum::zqn` and `quantum::memory`.
//!
//! # Mission
//!
//! `integration::memory` answers:
//!
//! > How can ZQN noise semantics be safely presented to the quantum-memory
//! > subsystem without making either subsystem own the other?
//!
//! It does NOT implement:
//!
//! - quantum-state mathematics;
//! - state-vector simulation;
//! - density-matrix simulation;
//! - stabilizer simulation;
//! - tensor-network simulation;
//! - sparse simulation;
//! - channel mathematics;
//! - Kraus/Choi/Lindblad mathematics;
//! - probability distributions;
//! - memory allocation;
//! - memory pooling;
//! - GPU allocation;
//! - distributed memory;
//! - QPU communication;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - hardware-provider APIs;
//! - compiler parsing;
//! - canonical quantum IR semantics.
//!
//! Those responsibilities remain owned by their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                              v
//!                   canonical operation semantics
//!                              |
//!                  +-----------+-----------+
//!                  |                       |
//!                  v                       v
//!              ZQN noise               memory
//!                  |                       |
//!                  v                       v
//!        NoiseApplication              QuantumState
//!                  |                       |
//!                  +-----------+-----------+
//!                              |
//!                              v
//!                    integration::memory
//!                              |
//!                    +---------+---------+
//!                    |                   |
//!                    v                   v
//!             channel application   fault application
//!                    |                   |
//!                    +---------+---------+
//!                              |
//!                              v
//!                       execution layer
//! ```
//!
//! The integration layer is deliberately an adapter contract rather than a
//! concrete execution engine.
//!
//! # Critical ownership rule
//!
//! `quantum::memory` remains the owner of memory and state resources.
//!
//! `quantum::zqn` remains the owner of noise semantics.
//!
//! Therefore:
//!
//! ```text
//! ZQN
//!   |
//!   | describes noise
//!   v
//! memory integration
//!   |
//!   | requests state/channel interaction
//!   v
//! memory implementation
//! ```
//!
//! The reverse dependency is prohibited:
//!
//! ```text
//! ZQN -> concrete memory implementation
//! ZQN -> state-vector implementation
//! ZQN -> allocator
//! ZQN -> GPU API
//! ZQN -> vendor QPU API
//! ```
//!
//! # Canonical identities
//!
//! This file does not define a second qubit identity.
//!
//! Logical qubits MUST use:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! Physical qubits MUST use:
//!
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//!
//! Operation identities MUST use the canonical identity supplied by the
//! quantum IR.
//!
//! The integration boundary deliberately stores those identities in the
//! request types below rather than replacing them with `usize`.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum in this module for:
//!
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - memory resources;
//! - channel applications;
//! - fault applications;
//! - state representations;
//! - distributed partitions;
//! - machines;
//! - devices;
//! - execution nodes.
//!
//! In particular, this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_MEMORY
//! MAX_CHANNELS
//! MAX_OPERATIONS
//! ```
//!
//! Actual resource restrictions belong to the memory subsystem's resource
//! budgets/limits and the ZQN execution context.
//!
//! "Infinity" therefore means:
//!
//! > no artificial finite semantic ceiling imposed by this integration layer.
//!
//! It does not mean that a finite computer can materialize an infinite state.
//!
//! # Representation neutrality
//!
//! The integration boundary does not assume that the state is represented as:
//!
//! - a dense state vector;
//! - a density matrix;
//! - a stabilizer tableau;
//! - a sparse vector;
//! - a tensor network.
//!
//! It also does not assume that a physical QPU exposes any directly readable
//! state representation.
//!
//! This is consistent with the memory subsystem's provider-neutral state
//! contract: a QPU may expose no amplitudes and no state-vector cloning.
//!
//! # ZQN interaction model
//!
//! ZQN produces semantic effects such as:
//!
//! ```text
//! ideal operation
//!      |
//!      v
//! NoiseApplication
//!      |
//!      +--> channel semantics
//!      |
//!      +--> fault semantics
//!      |
//!      +--> uncertainty metadata
//!      |
//!      +--> provenance
//! ```
//!
//! This integration layer converts those semantics into an abstract memory
//! request. It does not itself evolve the state.
//!
//! # Determinism
//!
//! This module owns no RNG.
//!
//! It must never create a hidden global RNG and must never make stochastic
//! decisions implicitly.
//!
//! If ZQN has already produced a deterministic noise realization, this layer
//! preserves the identity of that realization.
//!
//! If sampling is required, sampling belongs to the ZQN simulation/execution
//! layer and must use an explicitly supplied deterministic execution context.
//!
//! # Error policy
//!
//! Integration failures are represented as `MemoryIntegrationError`.
//!
//! The type intentionally does not attempt to duplicate the entire ZQN or
//! memory error taxonomy. Higher layers may convert it into their canonical
//! subsystem error type.
//!
//! # Thread safety
//!
//! All data types in this module are value-oriented and contain no global
//! mutable state.
//!
//! The adapter traits require `Send + Sync` only where the actual implementer
//! elects to use them. The integration contract itself does not force a
//! particular concurrency strategy on a memory provider.
//!
//! # Security
//!
//! This boundary treats resource descriptions and requests as potentially
//! untrusted.
//!
//! It therefore:
//!
//! - validates identifiers;
//! - validates dimensions;
//! - rejects inconsistent mappings;
//! - uses checked arithmetic where arithmetic is necessary;
//! - never silently truncates identifiers;
//! - never silently changes representations;
//! - never allocates merely to validate a request;
//! - never exposes raw pointers;
//! - never accepts `unsafe` implementations through this module.
//!
//! # Integration contract
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! quantum::zqn          quantum::memory
//!      |                    |
//!      +---------+----------+
//!                |
//!                v
//!       zqn::integration::memory
//!                |
//!                v
//!          runtime/execution
//! ```
//!
//! A future memory representation can be added without modifying this file.
//!
//! A future ZQN channel representation can be added without modifying this
//! file.
//!
//! A future hardware provider can be added without modifying this file.
//!
//! # Integration with existing memory subsystem
//!
//! The existing `quantum::memory` subsystem owns:
//!
//! - memory resource identities;
//! - state representations;
//! - allocation;
//! - state lifecycle;
//! - views and slices;
//! - snapshots;
//! - persistence;
//! - host/device/distributed coherence;
//! - QPU resource contracts.
//!
//! This file consumes those concepts through a provider-neutral adapter
//! contract rather than depending on a particular implementation.
//!
//! # Integration with existing ZQN subsystem
//!
//! The existing ZQN application layer already models a `NoiseApplication`
//! as an immutable attachment of selected noise semantics to an operation or
//! resource scope.
//!
//! This file deliberately does not reconstruct that application.
//!
//! Instead, callers should:
//!
//! ```text
//! NoiseModel
//!      |
//!      v
//! NoiseApplication
//!      |
//!      v
//! MemoryNoiseRequest
//!      |
//!      v
//! MemoryNoiseAdapter
//!      |
//!      v
//! Quantum memory/state provider
//! ```
//!
//! # Integration with simulation
//!
//! ZQN simulation engines may use this module when the selected simulation
//! representation is owned by `quantum::memory`.
//!
//! The direction is:
//!
//! ```text
//! ZQN channel/fault
//!        |
//!        v
//! MemoryNoiseRequest
//!        |
//!        v
//! QuantumState adapter
//! ```
//!
//! The simulator remains responsible for numerical state evolution.
//!
//! # Integration with QEC
//!
//! QEC remains the owner of syndrome generation, decoding, correction and
//! logical error analysis.
//!
//! A QEC implementation may consume a `MemoryNoiseRequest` when physical
//! state/resource effects must be represented in memory.
//!
//! ZQN remains the source of the physical-noise semantics.
//!
//! # Integration with hardware
//!
//! Hardware adapters do not enter through this file as vendor APIs.
//!
//! A hardware provider may implement the memory adapter contract if it exposes
//! provider-neutral state/resource semantics.
//!
//! A real QPU may instead expose only an opaque resource and measurement
//! results. The contract therefore does not require direct state access.
//!
//! # Integration with routing
//!
//! Routing owns logical-to-physical placement.
//!
//! If routing supplies a logical-to-physical mapping, this module transports
//! that mapping without changing it.
//!
//! This module does not calculate routes.
//!
//! # Integration with scheduling
//!
//! Scheduling owns temporal ordering and timing.
//!
//! A scheduling layer may include operation identity and timing metadata in
//! `MemoryNoiseRequest` through the opaque integration metadata mechanism.
//!
//! This module does not determine execution order.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may observe memory/noise effects through higher-level
//! telemetry and execution results.
//!
//! This module must never depend on benchmark implementations.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `unsafe` is denied explicitly.
//!
//! # Future-proofing
//!
//! The integration request uses extensible enums and opaque identifiers rather
//! than machine-specific enumerations.
//!
//! New quantum technologies should therefore be able to use this boundary
//! without adding another fixed-size hardware model.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Result type used by the ZQN/memory integration boundary.
pub type MemoryIntegrationResult<T> = Result<T, MemoryIntegrationError>;

/// Error produced while validating or executing a ZQN/memory integration
/// request.
///
/// This is deliberately an integration error rather than a replacement for
/// the canonical memory or ZQN error hierarchies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryIntegrationError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Semantic name of the identifier.
        kind: &'static str,
    },

    /// A textual identifier exceeded the permitted integration-policy size.
    ///
    /// This is a validation-policy value, not a machine-size limit.
    IdentifierTooLong {
        /// Semantic name of the identifier.
        kind: &'static str,
    },

    /// A mapping contained two logical qubits referring to the same physical
    /// resource.
    DuplicatePhysicalQubit {
        /// Physical resource claimed more than once.
        physical: PhysicalQubitId,
    },

    /// A logical qubit was mapped inconsistently.
    InvalidLogicalMapping {
        /// Logical qubit involved in the invalid mapping.
        logical: QubitId,
    },

    /// A required operation identity was missing.
    MissingOperationIdentity,

    /// The memory provider does not support the requested operation.
    UnsupportedOperation {
        /// Stable operation name.
        operation: String,
    },

    /// The requested memory/state capability is unavailable.
    CapabilityUnavailable {
        /// Stable capability identifier.
        capability: String,
    },

    /// The request was cancelled before execution.
    Cancelled,

    /// The request exceeded a caller-supplied resource policy.
    ResourceLimitExceeded {
        /// Resource category.
        resource: String,
    },

    /// A provider rejected the request.
    ProviderRejected {
        /// Provider-neutral reason.
        reason: String,
    },

    /// The request was internally inconsistent.
    InconsistentRequest {
        /// Provider-neutral description.
        reason: String,
    },
}

impl fmt::Display for MemoryIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(f, "{kind} identifier must not be empty")
            }
            Self::IdentifierTooLong { kind } => {
                write!(f, "{kind} identifier exceeds the integration policy")
            }
            Self::DuplicatePhysicalQubit { physical } => {
                write!(
                    f,
                    "physical qubit {} is mapped to more than one logical qubit",
                    physical.index()
                )
            }
            Self::InvalidLogicalMapping { logical } => {
                write!(
                    f,
                    "logical qubit {} has an invalid physical mapping",
                    logical.index()
                )
            }
            Self::MissingOperationIdentity => {
                f.write_str("an operation identity is required for this request")
            }
            Self::UnsupportedOperation { operation } => {
                write!(f, "memory provider does not support operation `{operation}`")
            }
            Self::CapabilityUnavailable { capability } => {
                write!(f, "required memory capability `{capability}` is unavailable")
            }
            Self::Cancelled => {
                f.write_str("memory integration request was cancelled")
            }
            Self::ResourceLimitExceeded { resource } => {
                write!(f, "memory integration resource policy exceeded: {resource}")
            }
            Self::ProviderRejected { reason } => {
                write!(f, "memory provider rejected the request: {reason}")
            }
            Self::InconsistentRequest { reason } => {
                write!(f, "inconsistent memory integration request: {reason}")
            }
        }
    }
}

impl std::error::Error for MemoryIntegrationError {}

/// Stable identifier for an abstract memory capability.
///
/// Capability names are deliberately extensible strings rather than a closed
/// enum. A new memory representation therefore does not require modifying this
/// integration layer.
///
/// Names must be non-empty and must not contain control characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemoryCapability(String);

impl MemoryCapability {
    /// Creates a validated capability identifier.
    pub fn new(value: impl Into<String>) -> MemoryIntegrationResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(MemoryIntegrationError::EmptyIdentifier {
                kind: "memory capability",
            });
        }

        if value.len() > 256 {
            return Err(MemoryIntegrationError::IdentifierTooLong {
                kind: "memory capability",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(MemoryIntegrationError::InconsistentRequest {
                reason: "memory capability contains a control character".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the capability identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MemoryCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable identifier for an abstract memory resource.
///
/// Qubit resources use canonical IR identities through [`MemoryResourceRef`].
/// Other future quantum resources use an opaque namespace/id pair.
///
/// This permits the same integration layer to support qubits, qudits, modes,
/// oscillators, photonic modes, links, logical resources and future resource
/// kinds without introducing a fixed global hardware taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemoryResourceRef {
    /// Logical qubit owned by the canonical quantum IR.
    LogicalQubit(QubitId),

    /// Physical qubit owned by the canonical quantum IR.
    PhysicalQubit(PhysicalQubitId),

    /// Extensible resource identity.
    Opaque {
        /// Namespace defining the resource kind.
        namespace: String,

        /// Stable provider-neutral resource identifier.
        id: String,
    },
}

impl MemoryResourceRef {
    /// Creates an opaque resource reference.
    pub fn opaque(
        namespace: impl Into<String>,
        id: impl Into<String>,
    ) -> MemoryIntegrationResult<Self> {
        let namespace = namespace.into();
        let id = id.into();

        validate_identifier(&namespace, "memory resource namespace")?;
        validate_identifier(&id, "memory resource identifier")?;

        Ok(Self::Opaque { namespace, id })
    }

    /// Returns whether the reference identifies a logical qubit.
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether the reference identifies a physical qubit.
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

/// Logical-to-physical placement supplied by routing/hardware.
///
/// This structure represents a mapping at one particular execution point. It
/// does not calculate the mapping.
///
/// A physical qubit may not simultaneously represent two logical qubits within
/// one mapping snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LogicalPhysicalMapping {
    mappings: BTreeMap<QubitId, PhysicalQubitId>,
}

impl LogicalPhysicalMapping {
    /// Creates an empty mapping.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mappings: BTreeMap::new(),
        }
    }

    /// Creates and validates a mapping from an iterator.
    ///
    /// The iterator may contain an arbitrary number of mappings. No semantic
    /// maximum is imposed.
    pub fn from_iter<I>(items: I) -> MemoryIntegrationResult<Self>
    where
        I: IntoIterator<Item = (QubitId, PhysicalQubitId)>,
    {
        let mut mapping = Self::new();

        for (logical, physical) in items {
            mapping.insert(logical, physical)?;
        }

        Ok(mapping)
    }

    /// Inserts a logical-to-physical mapping.
    ///
    /// Replacing an existing mapping for the same logical qubit is permitted
    /// because callers may be constructing a new snapshot. A physical qubit
    /// may not be assigned to another logical qubit in the same snapshot.
    pub fn insert(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> MemoryIntegrationResult<()> {
        if !logical.is_valid() {
            return Err(MemoryIntegrationError::InvalidLogicalMapping { logical });
        }

        if !physical.is_valid() {
            return Err(MemoryIntegrationError::InconsistentRequest {
                reason: format!(
                    "physical qubit {} is not a valid canonical physical qubit",
                    physical.index()
                ),
            });
        }

        if let Some(previous) = self.mappings.get(&logical) {
            if *previous == physical {
                return Ok(());
            }
        }

        if self
            .mappings
            .iter()
            .any(|(existing_logical, existing_physical)| {
                *existing_logical != logical && *existing_physical == physical
            })
        {
            return Err(MemoryIntegrationError::DuplicatePhysicalQubit { physical });
        }

        self.mappings.insert(logical, physical);
        Ok(())
    }

    /// Returns the physical qubit assigned to a logical qubit.
    #[must_use]
    pub fn physical_for(&self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.mappings.get(&logical).copied()
    }

    /// Returns the number of mapped logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Returns the mappings in deterministic logical-qubit order.
    pub fn iter(&self) -> impl Iterator<Item = (&QubitId, &PhysicalQubitId)> {
        self.mappings.iter()
    }

    /// Validates the complete mapping.
    pub fn validate(&self) -> MemoryIntegrationResult<()> {
        let mut physical_to_logical = BTreeMap::<PhysicalQubitId, QubitId>::new();

        for (logical, physical) in &self.mappings {
            if !logical.is_valid() {
                return Err(MemoryIntegrationError::InvalidLogicalMapping {
                    logical: *logical,
                });
            }

            if !physical.is_valid() {
                return Err(MemoryIntegrationError::InconsistentRequest {
                    reason: format!(
                        "physical qubit {} is invalid",
                        physical.index()
                    ),
                });
            }

            if physical_to_logical.insert(*physical, *logical).is_some() {
                return Err(MemoryIntegrationError::DuplicatePhysicalQubit {
                    physical: *physical,
                });
            }
        }

        Ok(())
    }
}

/// Execution scope for a memory/noise interaction.
///
/// The scope deliberately separates logical and physical identity. Routing
/// determines placement; memory integration transports that information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryExecutionScope {
    /// Logical resources participating in the operation.
    logical_resources: Vec<QubitId>,

    /// Optional logical-to-physical mapping.
    mapping: LogicalPhysicalMapping,
}

impl MemoryExecutionScope {
    /// Creates a scope from logical resources.
    ///
    /// The resource collection is caller-owned. No fixed arity is assumed.
    pub fn new<I>(logical_resources: I) -> MemoryIntegrationResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let logical_resources = logical_resources.into_iter().collect::<Vec<_>>();

        if logical_resources
            .iter()
            .any(|qubit| !qubit.is_valid())
        {
            let invalid = logical_resources
                .iter()
                .copied()
                .find(|qubit| !qubit.is_valid())
                .unwrap_or_else(|| QubitId::new(0));

            return Err(MemoryIntegrationError::InvalidLogicalMapping {
                logical: invalid,
            });
        }

        Ok(Self {
            logical_resources,
            mapping: LogicalPhysicalMapping::new(),
        })
    }

    /// Adds/sets the physical mapping for the scope.
    pub fn with_mapping(
        mut self,
        mapping: LogicalPhysicalMapping,
    ) -> MemoryIntegrationResult<Self> {
        mapping.validate()?;
        self.mapping = mapping;
        Ok(self)
    }

    /// Returns the logical resources.
    pub fn logical_resources(&self) -> &[QubitId] {
        &self.logical_resources
    }

    /// Returns the logical-to-physical mapping.
    #[must_use]
    pub const fn mapping(&self) -> &LogicalPhysicalMapping {
        &self.mapping
    }

    /// Returns the physical resource for a logical qubit when mapped.
    #[must_use]
    pub fn physical_for(&self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.mapping.physical_for(logical)
    }

    /// Validates the complete execution scope.
    pub fn validate(&self) -> MemoryIntegrationResult<()> {
        let mut seen = BTreeMap::<QubitId, ()>::new();

        for logical in &self.logical_resources {
            if !logical.is_valid() {
                return Err(MemoryIntegrationError::InvalidLogicalMapping {
                    logical: *logical,
                });
            }

            if seen.insert(*logical, ()).is_some() {
                return Err(MemoryIntegrationError::InconsistentRequest {
                    reason: format!(
                        "logical qubit {} appears more than once in an execution scope",
                        logical.index()
                    ),
                });
            }
        }

        self.mapping.validate()
    }
}

/// Abstract kind of state interaction requested from memory.
///
/// This is intentionally semantic rather than representation-specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemoryInteractionKind {
    /// Apply a quantum channel.
    Channel,

    /// Apply a discrete fault realization.
    Fault,

    /// Attach/read noise metadata without modifying state.
    Annotation,

    /// Query whether an interaction is supported.
    CapabilityQuery,
}

impl fmt::Display for MemoryInteractionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Channel => "channel",
            Self::Fault => "fault",
            Self::Annotation => "annotation",
            Self::CapabilityQuery => "capability_query",
        };

        f.write_str(value)
    }
}

/// Opaque ZQN semantic identity.
///
/// The integration layer does not interpret the internals of a channel or
/// fault. It only transports the stable identity and declared semantic kind.
///
/// This avoids coupling memory to Kraus, Choi, Pauli-transfer, Lindblad or
/// other future ZQN representations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoiseSemanticRef {
    /// Stable ZQN namespace.
    namespace: String,

    /// Stable semantic identity.
    id: String,

    /// Semantic interaction kind.
    kind: MemoryInteractionKind,
}

impl NoiseSemanticRef {
    /// Creates a validated semantic reference.
    pub fn new(
        namespace: impl Into<String>,
        id: impl Into<String>,
        kind: MemoryInteractionKind,
    ) -> MemoryIntegrationResult<Self> {
        let namespace = namespace.into();
        let id = id.into();

        validate_identifier(&namespace, "noise namespace")?;
        validate_identifier(&id, "noise semantic identifier")?;

        Ok(Self {
            namespace,
            id,
            kind,
        })
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the semantic identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the semantic interaction kind.
    #[must_use]
    pub const fn kind(&self) -> MemoryInteractionKind {
        self.kind
    }
}

/// Optional deterministic operation identity.
///
/// The integration layer intentionally stores the canonical IR operation
/// identity as an opaque `u64` only at the boundary where this file cannot
/// rely on the complete constructor/API of the repository's operation identity
/// type.
///
/// The value is never interpreted as a qubit index or machine size.
///
/// Callers should obtain it from `quantum::ir::identity::OperationId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationIdentity(u64);

impl OperationIdentity {
    /// Creates an integration identity from the canonical operation value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable operation value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Immutable request describing one ZQN-to-memory interaction.
///
/// The request contains semantics and identity, but never a direct memory
/// pointer, allocator reference or backend object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryNoiseRequest {
    /// Operation identity when the noise is operation-scoped.
    operation: Option<OperationIdentity>,

    /// Resources affected by the interaction.
    scope: MemoryExecutionScope,

    /// Selected ZQN semantic effect.
    noise: NoiseSemanticRef,

    /// Optional calibration identity.
    ///
    /// This remains opaque so memory does not depend on ZQN calibration
    /// implementation details.
    calibration_id: Option<String>,
}

impl MemoryNoiseRequest {
    /// Creates a new request.
    pub fn new(
        operation: Option<OperationIdentity>,
        scope: MemoryExecutionScope,
        noise: NoiseSemanticRef,
    ) -> MemoryIntegrationResult<Self> {
        scope.validate()?;

        if matches!(
            noise.kind(),
            MemoryInteractionKind::Channel | MemoryInteractionKind::Fault
        ) && operation.is_none()
        {
            return Err(MemoryIntegrationError::MissingOperationIdentity);
        }

        Ok(Self {
            operation,
            scope,
            noise,
            calibration_id: None,
        })
    }

    /// Attaches an opaque calibration identity.
    pub fn with_calibration_id(
        mut self,
        calibration_id: impl Into<String>,
    ) -> MemoryIntegrationResult<Self> {
        let calibration_id = calibration_id.into();

        validate_identifier(&calibration_id, "calibration identifier")?;

        self.calibration_id = Some(calibration_id);
        Ok(self)
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationIdentity> {
        self.operation
    }

    /// Returns the execution scope.
    #[must_use]
    pub const fn scope(&self) -> &MemoryExecutionScope {
        &self.scope
    }

    /// Returns the selected noise semantic reference.
    #[must_use]
    pub const fn noise(&self) -> &NoiseSemanticRef {
        &self.noise
    }

    /// Returns the optional calibration identity.
    pub fn calibration_id(&self) -> Option<&str> {
        self.calibration_id.as_deref()
    }

    /// Performs complete request validation.
    pub fn validate(&self) -> MemoryIntegrationResult<()> {
        self.scope.validate()?;

        if matches!(
            self.noise.kind(),
            MemoryInteractionKind::Channel | MemoryInteractionKind::Fault
        ) && self.operation.is_none()
        {
            return Err(MemoryIntegrationError::MissingOperationIdentity);
        }

        Ok(())
    }
}

/// Capability description supplied by a memory implementation.
///
/// Capability names are extensible, deterministic and provider-neutral.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryCapabilities {
    capabilities: Vec<MemoryCapability>,
}

impl MemoryCapabilities {
    /// Creates an empty capability set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// Creates a capability set from an iterator.
    pub fn from_iter<I>(items: I) -> MemoryIntegrationResult<Self>
    where
        I: IntoIterator<Item = MemoryCapability>,
    {
        let mut capabilities = Self::new();

        for capability in items {
            capabilities.insert(capability);
        }

        Ok(capabilities)
    }

    /// Adds a capability.
    ///
    /// Duplicate capability names are ignored.
    pub fn insert(&mut self, capability: MemoryCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
            self.capabilities.sort();
        }
    }

    /// Returns whether the capability is supported.
    #[must_use]
    pub fn supports(&self, capability: &MemoryCapability) -> bool {
        self.capabilities.binary_search(capability).is_ok()
    }

    /// Returns capabilities in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &MemoryCapability> {
        self.capabilities.iter()
    }

    /// Returns the number of capabilities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether no capabilities are declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
}

/// Abstract result of a memory interaction.
///
/// A memory implementation can return an opaque result identifier when the
/// underlying representation cannot be inspected, which is essential for real
/// QPUs and provider-managed states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryInteractionResult {
    /// Stable result identifier.
    result_id: String,

    /// Whether the state was actually modified.
    state_modified: bool,

    /// Whether the result is directly inspectable by this integration layer.
    inspectable: bool,
}

impl MemoryInteractionResult {
    /// Creates a result.
    pub fn new(
        result_id: impl Into<String>,
        state_modified: bool,
        inspectable: bool,
    ) -> MemoryIntegrationResult<Self> {
        let result_id = result_id.into();
        validate_identifier(&result_id, "memory interaction result")?;

        Ok(Self {
            result_id,
            state_modified,
            inspectable,
        })
    }

    /// Returns the result identifier.
    pub fn result_id(&self) -> &str {
        &self.result_id
    }

    /// Returns whether state was modified.
    #[must_use]
    pub const fn state_modified(&self) -> bool {
        self.state_modified
    }

    /// Returns whether this integration result can be inspected directly.
    #[must_use]
    pub const fn inspectable(&self) -> bool {
        self.inspectable
    }
}

/// Provider-neutral memory adapter.
///
/// A concrete memory implementation implements this trait.
///
/// The trait intentionally contains no state-vector-specific method.
///
/// A dense simulator, tensor-network simulator, stabilizer engine, GPU
/// implementation, distributed memory implementation or QPU-backed memory
/// provider can all implement the same boundary.
pub trait MemoryNoiseAdapter {
    /// Validates the requested interaction against the provider's current
    /// capabilities.
    fn validate_noise_request(
        &self,
        request: &MemoryNoiseRequest,
    ) -> MemoryIntegrationResult<()>;

    /// Applies an already-selected ZQN semantic interaction.
    ///
    /// The adapter owns the actual state evolution or provider interaction.
    ///
    /// This method must not reinterpret an unsupported channel as a different
    /// channel or silently approximate it.
    fn apply_noise(
        &mut self,
        request: &MemoryNoiseRequest,
    ) -> MemoryIntegrationResult<MemoryInteractionResult>;

    /// Returns the capabilities exposed by this memory implementation.
    fn capabilities(&self) -> MemoryCapabilities;
}

/// Read-only capability provider.
///
/// This smaller trait is useful for routing, scheduling, validation and
/// planning stages that must not mutate quantum memory.
pub trait MemoryCapabilityProvider {
    /// Returns current memory capabilities.
    fn memory_capabilities(&self) -> MemoryCapabilities;
}

/// A provider-independent reference to a memory resource.
///
/// This deliberately contains no address and no raw pointer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemoryResourceBinding {
    /// Resource exposed to ZQN.
    resource: MemoryResourceRef,

    /// Optional opaque backend resource identifier.
    backend_id: Option<String>,
}

impl MemoryResourceBinding {
    /// Creates a binding for a canonical resource.
    #[must_use]
    pub fn new(resource: MemoryResourceRef) -> Self {
        Self {
            resource,
            backend_id: None,
        }
    }

    /// Attaches an opaque provider resource identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> MemoryIntegrationResult<Self> {
        let backend_id = backend_id.into();
        validate_identifier(&backend_id, "backend memory identifier")?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Returns the canonical resource.
    #[must_use]
    pub const fn resource(&self) -> &MemoryResourceRef {
        &self.resource
    }

    /// Returns the opaque provider identifier.
    pub fn backend_id(&self) -> Option<&str> {
        self.backend_id.as_deref()
    }
}

/// Collection of resource bindings.
///
/// The vector preserves caller ordering because ordering can be semantically
/// relevant to some operations. Consumers that require canonical ordering
/// should sort by `MemoryResourceBinding`, whose ordering is deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryResourceBindings {
    bindings: Vec<MemoryResourceBinding>,
}

impl MemoryResourceBindings {
    /// Creates an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    /// Creates bindings from an iterator.
    pub fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = MemoryResourceBinding>,
    {
        Self {
            bindings: items.into_iter().collect(),
        }
    }

    /// Appends a binding.
    pub fn push(&mut self, binding: MemoryResourceBinding) {
        self.bindings.push(binding);
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Returns bindings in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &MemoryResourceBinding> {
        self.bindings.iter()
    }

    /// Returns a deterministic sorted copy.
    #[must_use]
    pub fn sorted(&self) -> Vec<MemoryResourceBinding> {
        let mut result = self.bindings.clone();
        result.sort();
        result
    }
}

/// Validates an integration identifier.
///
/// This is deliberately a conservative syntactic validation policy and is not
/// a semantic machine-size restriction.
fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> MemoryIntegrationResult<()> {
    if value.is_empty() {
        return Err(MemoryIntegrationError::EmptyIdentifier { kind });
    }

    if value.len() > 256 {
        return Err(MemoryIntegrationError::IdentifierTooLong { kind });
    }

    if value.chars().any(char::is_control) {
        return Err(MemoryIntegrationError::InconsistentRequest {
            reason: format!("{kind} contains a control character"),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_physical_mapping_is_deterministic() {
        let mut mapping = LogicalPhysicalMapping::new();

        mapping
            .insert(QubitId::new(2), PhysicalQubitId::new(7))
            .expect("mapping should be valid");

        mapping
            .insert(QubitId::new(0), PhysicalQubitId::new(3))
            .expect("mapping should be valid");

        assert_eq!(
            mapping.physical_for(QubitId::new(0)),
            Some(PhysicalQubitId::new(3))
        );

        assert_eq!(
            mapping.physical_for(QubitId::new(2)),
            Some(PhysicalQubitId::new(7))
        );

        let values: Vec<(u64, u64)> = mapping
            .iter()
            .map(|(logical, physical)| (logical.index(), physical.index()))
            .collect();

        assert_eq!(values, vec![(0, 3), (2, 7)]);
    }

    #[test]
    fn duplicate_physical_mapping_is_rejected() {
        let mut mapping = LogicalPhysicalMapping::new();

        mapping
            .insert(QubitId::new(0), PhysicalQubitId::new(4))
            .expect("first mapping should be valid");

        let result =
            mapping.insert(QubitId::new(1), PhysicalQubitId::new(4));

        assert!(matches!(
            result,
            Err(MemoryIntegrationError::DuplicatePhysicalQubit { .. })
        ));
    }

    #[test]
    fn same_logical_mapping_is_idempotent() {
        let mut mapping = LogicalPhysicalMapping::new();

        let logical = QubitId::new(0);
        let physical = PhysicalQubitId::new(5);

        mapping
            .insert(logical, physical)
            .expect("first mapping should be valid");

        mapping
            .insert(logical, physical)
            .expect("identical mapping should remain valid");

        assert_eq!(mapping.len(), 1);
    }

    #[test]
    fn resource_namespace_and_id_are_validated() {
        assert!(MemoryResourceRef::opaque("mode", "m0").is_ok());

        assert!(MemoryResourceRef::opaque("", "m0").is_err());

        assert!(MemoryResourceRef::opaque("mode", "").is_err());
    }

    #[test]
    fn noise_semantic_reference_is_extensible() {
        let reference = NoiseSemanticRef::new(
            "zqn.channel",
            "example-channel",
            MemoryInteractionKind::Channel,
        )
        .expect("reference should be valid");

        assert_eq!(reference.namespace(), "zqn.channel");
        assert_eq!(reference.id(), "example-channel");
        assert_eq!(reference.kind(), MemoryInteractionKind::Channel);
    }

    #[test]
    fn channel_request_requires_operation_identity() {
        let scope =
            MemoryExecutionScope::new([QubitId::new(0)]).expect("scope");

        let noise = NoiseSemanticRef::new(
            "zqn.channel",
            "example",
            MemoryInteractionKind::Channel,
        )
        .expect("noise reference");

        let request = MemoryNoiseRequest::new(None, scope, noise);

        assert!(matches!(
            request,
            Err(MemoryIntegrationError::MissingOperationIdentity)
        ));
    }

    #[test]
    fn annotation_request_can_be_operation_independent() {
        let scope =
            MemoryExecutionScope::new([QubitId::new(0)]).expect("scope");

        let noise = NoiseSemanticRef::new(
            "zqn.annotation",
            "example",
            MemoryInteractionKind::Annotation,
        )
        .expect("noise reference");

        let request =
            MemoryNoiseRequest::new(None, scope, noise);

        assert!(request.is_ok());
    }

    #[test]
    fn capabilities_are_sorted_and_deduplicated() {
        let first =
            MemoryCapability::new("zqn.channel").expect("capability");

        let second =
            MemoryCapability::new("state.observable").expect("capability");

        let mut capabilities = MemoryCapabilities::new();

        capabilities.insert(first.clone());
        capabilities.insert(second.clone());
        capabilities.insert(first.clone());

        let names: Vec<&str> =
            capabilities.iter().map(MemoryCapability::as_str).collect();

        assert_eq!(
            names,
            vec!["state.observable", "zqn.channel"]
        );

        assert_eq!(capabilities.len(), 2);
    }

    #[test]
    fn operation_identity_is_opaque_and_stable() {
        let identity = OperationIdentity::new(42);

        assert_eq!(identity.value(), 42);
    }

    #[test]
    fn resource_bindings_do_not_expose_addresses() {
        let binding = MemoryResourceBinding::new(
            MemoryResourceRef::LogicalQubit(QubitId::new(3)),
        );

        assert_eq!(
            binding.resource(),
            &MemoryResourceRef::LogicalQubit(QubitId::new(3))
        );

        assert!(binding.backend_id().is_none());
    }

    #[test]
    fn empty_mapping_is_valid() {
        let mapping = LogicalPhysicalMapping::new();

        assert!(mapping.validate().is_ok());
        assert!(mapping.is_empty());
    }

    #[test]
    fn arbitrary_generated_mapping_has_no_architectural_qubit_limit() {
        let mut mapping = LogicalPhysicalMapping::new();

        // This test deliberately derives its size from the test input rather
        // than asserting a production maximum.
        for index in 0_u64..256_u64 {
            mapping
                .insert(
                    QubitId::new(index),
                    PhysicalQubitId::new(index),
                )
                .expect("generated mapping should be valid");
        }

        assert_eq!(mapping.len(), 256);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn scope_rejects_duplicate_logical_resources() {
        let result = MemoryExecutionScope::new([
            QubitId::new(1),
            QubitId::new(1),
        ]);

        let scope = result.expect("construction itself permits validation");

        assert!(matches!(
            scope.validate(),
            Err(MemoryIntegrationError::InconsistentRequest { .. })
        ));
    }

    #[test]
    fn request_validation_is_repeatable() {
        let scope =
            MemoryExecutionScope::new([QubitId::new(0)]).expect("scope");

        let noise = NoiseSemanticRef::new(
            "zqn.channel",
            "channel",
            MemoryInteractionKind::Channel,
        )
        .expect("noise");

        let request = MemoryNoiseRequest::new(
            Some(OperationIdentity::new(17)),
            scope,
            noise,
        )
        .expect("request");

        assert!(request.validate().is_ok());
        assert!(request.validate().is_ok());
    }
}