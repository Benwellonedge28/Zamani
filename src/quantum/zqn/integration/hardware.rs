//! Zamani Quantum Noise (ZQN) — Hardware Integration Boundary
//!
//! Production-grade provider-neutral integration boundary between ZQN and the
//! Zamani Quantum hardware subsystem.
//!
//! # Ownership
//!
//! This file owns the ZQN-side contract for consuming hardware information.
//!
//! It owns:
//!
//! - immutable hardware integration snapshots;
//! - abstract hardware resource references;
//! - canonical logical/physical qubit association;
//! - hardware observation identity;
//! - hardware noise-observation envelopes;
//! - hardware-derived timing/noise context;
//! - explicit support/approximation status;
//! - hardware-to-ZQN integration diagnostics;
//! - deterministic resource ordering;
//! - validation of hardware information before ZQN consumption;
//! - provider-neutral conversion boundaries;
//! - capability-independent hardware observations;
//! - explicit provenance hooks;
//! - resource-safe streaming interfaces.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - provider APIs;
//! - provider SDKs;
//! - authentication;
//! - credentials;
//! - QPU handles;
//! - network connections;
//! - backend discovery;
//! - backend registries;
//! - hardware topology implementation;
//! - routing;
//! - scheduling;
//! - calibration acquisition;
//! - calibration storage;
//! - quantum-channel mathematics;
//! - noise-model mathematics;
//! - simulation;
//! - QEC decoding;
//! - benchmarking methodology;
//! - execution;
//! - canonical quantum IR;
//! - provider-specific identifiers as semantic types.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Program
//!                          |
//!                          v
//!                    quantum::ir
//!                          |
//!              +-----------+-----------+
//!              |                       |
//!              v                       v
//!             ZQN                 hardware subsystem
//!              |                       |
//!              |                provider adapters
//!              |                       |
//!              +-----------+-----------+
//!                          |
//!                          v
//!               THIS INTEGRATION BOUNDARY
//!                          |
//!             +------------+-------------+
//!             |            |             |
//!             v            v             v
//!          noise       calibration     target
//!             |            |             |
//!             +------------+-------------+
//!                          |
//!                          v
//!                    ZQN consumers
//!               routing / scheduling / QEC
//!               simulation / benchmarking
//! ```
//!
//! The critical dependency direction is:
//!
//! ```text
//! hardware facts
//!      |
//!      v
//! provider-neutral hardware contract
//!      |
//!      v
//! ZQN integration
//!      |
//!      v
//! ZQN domain semantics
//! ```
//!
//! ZQN MUST NOT call a provider adapter directly.
//!
//! # Canonical resource identity
//!
//! ZQN MUST use the repository's canonical quantum identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file therefore does not define another `QubitId` or
//! `PhysicalQubitId`.
//!
//! Logical and physical identity remain distinct:
//!
//! ```text
//! QubitId
//!     = semantic/logical resource
//!
//! PhysicalQubitId
//!     = target physical resource
//! ```
//!
//! For non-qubit quantum technologies, the integration boundary additionally
//! supports opaque provider-neutral resource keys. This avoids assuming that
//! every future quantum architecture is qubit-based.
//!
//! # Write once, scale everywhere
//!
//! This file imposes no semantic maximum on:
//!
//! - logical qubits;
//! - physical qubits;
//! - modes;
//! - resources;
//! - operations;
//! - observations;
//! - targets;
//! - execution nodes;
//! - links;
//! - hardware technologies;
//! - topology size.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_TARGETS
//! MAX_OBSERVATIONS
//! ```
//!
//! Resource limits, if required, belong to the caller/runtime policy.
//!
//! Collections in this module grow according to actual workload and available
//! resources.
//!
//! # Technology neutrality
//!
//! This boundary must support, without changing its semantic contract:
//!
//! - superconducting processors;
//! - trapped ions;
//! - neutral atoms;
//! - photonic processors;
//! - spin/semiconductor systems;
//! - topological systems;
//! - bosonic systems;
//! - continuous-variable systems;
//! - analog quantum processors;
//! - annealers;
//! - measurement-based systems;
//! - logical/fault-tolerant processors;
//! - distributed quantum systems;
//! - simulators;
//! - emulators;
//! - future quantum technologies.
//!
//! Consequently this file does not define a technology enum as the primary
//! integration abstraction.
//!
//! Hardware technology can be supplied as metadata or capability information.
//!
//! # Hardware versus ZQN semantics
//!
//! Hardware tells ZQN facts such as:
//!
//! ```text
//! resource exists
//! resource is available
//! operation has measured duration
//! calibration is valid during interval
//! noise observation was measured
//! target supports a capability
//! ```
//!
//! ZQN decides how those facts participate in:
//!
//! ```text
//! channel construction
//! fault generation
//! noise estimation
//! error propagation
//! simulation
//! routing cost
//! scheduling cost
//! ```
//!
//! This separation is mandatory.
//!
//! # Exactness
//!
//! Hardware information can be:
//!
//! ```text
//! Exact
//! Measured
//! Estimated
//! Approximate
//! Bounded
//! Statistical
//! Unavailable
//! ```
//!
//! The integration layer must preserve this distinction.
//!
//! It MUST NOT convert an estimate into an exact value.
//!
//! It MUST NOT silently replace unavailable information with zero.
//!
//! It MUST NOT silently turn approximate support into exact support.
//!
//! # Determinism
//!
//! This module:
//!
//! - performs no network I/O;
//! - reads no system clock;
//! - generates no random numbers;
//! - uses no global mutable state;
//! - does not depend on thread IDs;
//! - does not depend on memory addresses;
//! - provides deterministic ordering.
//!
//! Hardware observations are treated as input data.
//!
//! If two identical snapshots are supplied, deterministic validation and
//! iteration must produce identical results.
//!
//! # Parallelism
//!
//! Immutable integration snapshots may be shared across threads when their
//! contained types satisfy the standard Rust `Send`/`Sync` requirements.
//!
//! The integration layer must never make stochastic semantics depend on the
//! scheduling of worker threads.
//!
//! # Security
//!
//! Hardware integration data may originate from untrusted providers or remote
//! systems.
//!
//! This module therefore:
//!
//! - validates identifiers;
//! - rejects empty semantic identifiers where identity is required;
//! - rejects malformed resource associations;
//! - rejects contradictory resource mappings;
//! - rejects invalid numerical values;
//! - rejects negative quantities represented by signed values;
//! - uses checked arithmetic;
//! - does not contain credentials;
//! - does not contain authentication headers;
//! - does not execute provider data;
//! - does not perform network I/O.
//!
//! Provider adapters remain responsible for validating provider-specific
//! responses before constructing these value objects.
//!
//! # Serialization
//!
//! This module intentionally does not derive `Serialize`/`Deserialize`.
//!
//! Rust struct layout is not the ZQN wire format.
//!
//! The future:
//!
//! ```text
//! zqn::io
//! ```
//!
//! layer owns versioned serialization.
//!
//! The structures here are semantic value objects.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contract
//!
//! Hardware providers should implement [`HardwareObservationSource`].
//!
//! A provider adapter performs:
//!
//! ```text
//! provider API
//!      |
//!      v
//! provider adapter
//!      |
//!      v
//! HardwareObservationSource
//!      |
//!      v
//! HardwareSnapshot
//!      |
//!      v
//! ZQN
//! ```
//!
//! The adapter may internally use:
//!
//! ```text
//! quantum::hardware::backend
//! quantum::hardware::backend_trait
//! quantum::hardware::timing
//! quantum::hardware::topology
//! quantum::hardware::calibration
//! ```
//!
//! but this integration file does not require a concrete provider.
//!
//! This is intentional: adding a new hardware provider must not require
//! modifying this file.
//!
//! # Integration with canonical IR
//!
//! Canonical operations remain owned by `quantum::ir`.
//!
//! This file only associates canonical logical and physical resource identities
//! with hardware observations.
//!
//! The integration direction is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!             |
//!             v
//! HardwareResourceAssociation
//!             |
//!             v
//! PhysicalQubitId
//! ```
//!
//! No IR type is redefined here.
//!
//! # Integration with calibration
//!
//! Hardware calibration modules may create [`HardwareCalibrationReference`].
//!
//! The reference identifies the calibration snapshot used to produce an
//! observation without embedding calibration storage inside this file.
//!
//! ZQN calibration modules remain responsible for interpreting the calibration
//! data.
//!
//! # Integration with noise
//!
//! `HardwareNoiseObservation` provides measured or estimated information.
//!
//! It does not itself define a quantum channel.
//!
//! ZQN noise/channel modules consume the observation and construct the
//! appropriate semantic noise representation.
//!
//! # Integration with routing
//!
//! Routing may consume hardware resource associations and noise observations.
//!
//! Routing remains responsible for deciding placement.
//!
//! This file never chooses a route.
//!
//! # Integration with scheduling
//!
//! Scheduling may consume:
//!
//! - hardware timing observations;
//! - resource availability;
//! - calibration validity;
//! - operation duration;
//! - hardware noise observations.
//!
//! Scheduling remains responsible for ordering and time assignment.
//!
//! # Integration with QEC
//!
//! QEC may consume hardware-derived fault/noise observations.
//!
//! QEC remains responsible for:
//!
//! - syndrome generation;
//! - decoding;
//! - correction;
//! - logical error analysis.
//!
//! # Integration with simulation
//!
//! Simulation may consume hardware observations to construct a realistic noise
//! model.
//!
//! This module never evolves a quantum state.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume immutable hardware observations and provenance.
//!
//! This module never defines benchmark methodology.
//!
//! # Integration with runtime
//!
//! Runtime/adapters can materialize a `HardwareSnapshot` before execution.
//!
//! The snapshot may then be shared with ZQN consumers.
//!
//! This avoids repeated provider calls during semantic evaluation and prevents
//! network state from becoming hidden part of ZQN semantics.
//!
//! # Important architectural rule
//!
//! A hardware adapter MUST NOT force a provider-specific concept into ZQN.
//!
//! For example, this is forbidden:
//!
//! ```text
//! ZQN -> IBM API
//! ZQN -> IonQ SDK
//! ZQN -> Braket SDK
//! ZQN -> provider HTTP endpoint
//! ```
//!
//! Instead:
//!
//! ```text
//! provider adapter -> provider-neutral observation -> ZQN
//! ```
//!
//! # Definition of done
//!
//! This file is complete independently when:
//!
//! 1. provider-neutral resource identity is available;
//! 2. canonical QubitId/PhysicalQubitId are used;
//! 3. hardware observations preserve exactness;
//! 4. calibration references are explicit;
//! 5. resource associations are validated;
//! 6. deterministic iteration is guaranteed;
//! 7. no provider API is imported;
//! 8. no credentials are represented;
//! 9. no hardware execution occurs;
//! 10. no scheduling/routing algorithm is embedded;
//! 11. no semantic size ceiling exists;
//! 12. no unsafe code exists;
//! 13. invalid numerical values are rejected;
//! 14. integration is expressed through stable traits;
//! 15. future providers can implement the contract without editing this file.
//!
//! # No re-edit requirement
//!
//! Downstream modules should implement the traits defined here rather than
//! changing this file merely because a new provider, technology, scheduler,
//! simulator, or calibration implementation is added.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Duration;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable semantic schema identifier for the ZQN hardware integration boundary.
pub const ZQN_HARDWARE_INTEGRATION_SCHEMA_ID: &str =
    "zamani.quantum.zqn.integration.hardware";

/// Semantic version of this integration contract.
pub const ZQN_HARDWARE_INTEGRATION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced while validating provider-neutral hardware information.
///
/// This is deliberately local to the integration boundary. Domain-level ZQN
/// failures remain owned by `zqn::core::errors`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareIntegrationError {
    /// A required identifier is empty.
    EmptyIdentifier {
        /// Semantic field name.
        field: &'static str,
    },

    /// An identifier is too large for the requested integration operation.
    ///
    /// This is a caller-selected safety boundary, not a semantic machine-size
    /// limit.
    IdentifierTooLarge {
        /// Semantic field name.
        field: &'static str,

        /// Caller-selected maximum.
        maximum: usize,
    },

    /// A resource association is contradictory.
    ConflictingAssociation {
        /// Logical resource.
        logical: QubitId,

        /// Existing physical resource.
        existing: PhysicalQubitId,

        /// Newly supplied physical resource.
        requested: PhysicalQubitId,
    },

    /// A physical resource is already assigned to another logical resource in
    /// a mapping that requires injectivity.
    PhysicalResourceAlreadyAssigned {
        /// Physical resource.
        physical: PhysicalQubitId,

        /// Existing logical resource.
        existing: QubitId,

        /// Requested logical resource.
        requested: QubitId,
    },

    /// A physical resource was used where a valid resource was required.
    InvalidPhysicalResource {
        /// Resource index.
        index: u64,
    },

    /// A logical resource was used where a valid resource was required.
    InvalidLogicalResource {
        /// Resource index.
        index: u64,
    },

    /// A numerical observation is invalid.
    InvalidNumericValue {
        /// Semantic field name.
        field: &'static str,
    },

    /// A probability-like value is outside the closed unit interval.
    InvalidProbability {
        /// Semantic field name.
        field: &'static str,
    },

    /// An error bound is negative.
    NegativeErrorBound,

    /// A duration cannot be represented by the requested operation.
    DurationOverflow,

    /// An observation has inconsistent exactness metadata.
    InvalidExactness,

    /// A required hardware observation is unavailable.
    ObservationUnavailable {
        /// Semantic field.
        field: &'static str,
    },

    /// A provider-neutral snapshot is internally inconsistent.
    InvalidSnapshot {
        /// Explanation.
        reason: &'static str,
    },
}

impl fmt::Display for HardwareIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "hardware integration identifier '{field}' is empty")
            }

            Self::IdentifierTooLarge { field, maximum } => {
                write!(
                    formatter,
                    "hardware integration identifier '{field}' exceeds caller limit {maximum}"
                )
            }

            Self::ConflictingAssociation {
                logical,
                existing,
                requested,
            } => {
                write!(
                    formatter,
                    "logical resource {} is already associated with physical \
                     resource {}; requested {}",
                    logical.index(),
                    existing.index(),
                    requested.index()
                )
            }

            Self::PhysicalResourceAlreadyAssigned {
                physical,
                existing,
                requested,
            } => {
                write!(
                    formatter,
                    "physical resource {} is assigned to logical resource {}, \
                     cannot also assign it to logical resource {}",
                    physical.index(),
                    existing.index(),
                    requested.index()
                )
            }

            Self::InvalidPhysicalResource { index } => {
                write!(formatter, "invalid physical resource index {index}")
            }

            Self::InvalidLogicalResource { index } => {
                write!(formatter, "invalid logical resource index {index}")
            }

            Self::InvalidNumericValue { field } => {
                write!(formatter, "invalid numerical value for '{field}'")
            }

            Self::InvalidProbability { field } => {
                write!(
                    formatter,
                    "probability-like value for '{field}' must be in [0, 1]"
                )
            }

            Self::NegativeErrorBound => {
                formatter.write_str("error bound cannot be negative")
            }

            Self::DurationOverflow => {
                formatter.write_str("duration conversion overflow")
            }

            Self::InvalidExactness => {
                formatter.write_str("invalid observation exactness metadata")
            }

            Self::ObservationUnavailable { field } => {
                write!(formatter, "required hardware observation '{field}' is unavailable")
            }

            Self::InvalidSnapshot { reason } => {
                write!(formatter, "invalid hardware integration snapshot: {reason}")
            }
        }
    }
}

impl std::error::Error for HardwareIntegrationError {}

/// Result type for this integration boundary.
pub type HardwareIntegrationResult<T> = Result<T, HardwareIntegrationError>;

// =============================================================================
// Observation exactness
// =============================================================================

/// Scientific status of a hardware-derived observation.
///
/// The distinction is mandatory because measured/estimated/approximate values
/// must never silently become exact semantic facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObservationQuality {
    /// The value is an exact consequence of the supplied hardware contract.
    Exact,

    /// The value was directly measured.
    Measured,

    /// The value was inferred from measurements or a validated model.
    Estimated,

    /// The value is an explicitly bounded approximation.
    Approximate,

    /// The value represents an explicit statistical estimate.
    Statistical,

    /// No value is available.
    Unavailable,
}

impl ObservationQuality {
    /// Returns whether the observation contains usable information.
    #[must_use]
    pub const fn is_available(self) -> bool {
        !matches!(self, Self::Unavailable)
    }

    /// Returns whether the observation is exact.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the observation is statistical.
    #[must_use]
    pub const fn is_statistical(self) -> bool {
        matches!(self, Self::Statistical)
    }

    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Measured => "measured",
            Self::Estimated => "estimated",
            Self::Approximate => "approximate",
            Self::Statistical => "statistical",
            Self::Unavailable => "unavailable",
        }
    }
}

impl fmt::Display for ObservationQuality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource identity
// =============================================================================

/// Provider-neutral quantum hardware resource.
///
/// Canonical logical and physical qubit IDs are preserved where applicable.
///
/// The opaque variant allows the same integration boundary to represent
/// non-qubit resources such as:
///
/// - bosonic modes;
/// - optical paths;
/// - resonators;
/// - transport channels;
/// - control resources;
/// - distributed links;
/// - future quantum resource types.
///
/// The opaque identifier is not a provider API object. It is merely a stable
/// semantic resource key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Technology-neutral opaque resource identifier.
    Opaque(String),
}

impl HardwareResource {
    /// Creates a validated opaque resource.
    pub fn opaque(value: impl Into<String>) -> HardwareIntegrationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HardwareIntegrationError::EmptyIdentifier {
                field: "resource",
            });
        }

        Ok(Self::Opaque(value))
    }

    /// Returns the canonical logical qubit, if this resource is one.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            Self::PhysicalQubit(_) | Self::Opaque(_) => None,
        }
    }

    /// Returns the canonical physical qubit, if this resource is one.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            Self::LogicalQubit(_) | Self::Opaque(_) => None,
        }
    }

    /// Returns true when this resource is an opaque non-qubit resource.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }

    /// Validates the resource identity.
    pub fn validate(&self) -> HardwareIntegrationResult<()> {
        match self {
            Self::LogicalQubit(id) => {
                if !id.is_valid() {
                    return Err(HardwareIntegrationError::InvalidLogicalResource {
                        index: id.index(),
                    });
                }
            }

            Self::PhysicalQubit(id) => {
                if !id.is_valid() {
                    return Err(HardwareIntegrationError::InvalidPhysicalResource {
                        index: id.index(),
                    });
                }
            }

            Self::Opaque(value) => {
                if value.trim().is_empty() {
                    return Err(HardwareIntegrationError::EmptyIdentifier {
                        field: "resource",
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Logical / physical association
// =============================================================================

/// Association between a canonical logical qubit and a canonical physical
/// qubit.
///
/// This is a value object. It does not perform routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitAssociation {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitAssociation {
    /// Creates a validated logical-to-physical association.
    pub fn new(
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> HardwareIntegrationResult<Self> {
        if !logical.is_valid() {
            return Err(HardwareIntegrationError::InvalidLogicalResource {
                index: logical.index(),
            });
        }

        if !physical.is_valid() {
            return Err(HardwareIntegrationError::InvalidPhysicalResource {
                index: physical.index(),
            });
        }

        Ok(Self { logical, physical })
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(self) -> PhysicalQubitId {
        self.physical
    }
}

/// Deterministic logical-to-physical association map.
///
/// The map itself does not decide placement. It only records a placement
/// already selected by routing or execution infrastructure.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QubitAssociations {
    logical_to_physical: BTreeMap<QubitId, PhysicalQubitId>,
    physical_to_logical: BTreeMap<PhysicalQubitId, QubitId>,
}

impl QubitAssociations {
    /// Creates an empty association map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of logical-to-physical associations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns true when there are no associations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Inserts an association while enforcing one-to-one physical assignment.
    ///
    /// This is intentionally a mapping invariant rather than a machine-size
    /// limit.
    pub fn insert(
        &mut self,
        association: QubitAssociation,
    ) -> HardwareIntegrationResult<bool> {
        let logical = association.logical();
        let physical = association.physical();

        if let Some(existing) = self.logical_to_physical.get(&logical) {
            if *existing == physical {
                return Ok(false);
            }

            return Err(HardwareIntegrationError::ConflictingAssociation {
                logical,
                existing: *existing,
                requested: physical,
            });
        }

        if let Some(existing) = self.physical_to_logical.get(&physical) {
            if *existing != logical {
                return Err(
                    HardwareIntegrationError::PhysicalResourceAlreadyAssigned {
                        physical,
                        existing: *existing,
                        requested: logical,
                    },
                );
            }
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(true)
    }

    /// Returns the physical resource associated with a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Returns the logical resource associated with a physical qubit.
    #[must_use]
    pub fn logical_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.physical_to_logical.get(&physical).copied()
    }

    /// Iterates associations in deterministic logical-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = QubitAssociation> + '_ {
        self.logical_to_physical
            .iter()
            .map(|(logical, physical)| QubitAssociation {
                logical: *logical,
                physical: *physical,
            })
    }

    /// Returns all logical qubits in deterministic order.
    pub fn logical_qubits(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.logical_to_physical.keys().copied()
    }

    /// Returns all physical qubits in deterministic order.
    pub fn physical_qubits(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.physical_to_logical.keys().copied()
    }

    /// Validates all associations.
    pub fn validate(&self) -> HardwareIntegrationResult<()> {
        if self.logical_to_physical.len() != self.physical_to_logical.len() {
            return Err(HardwareIntegrationError::InvalidSnapshot {
                reason: "logical and physical association indexes disagree",
            });
        }

        for (logical, physical) in &self.logical_to_physical {
            if !logical.is_valid() {
                return Err(HardwareIntegrationError::InvalidLogicalResource {
                    index: logical.index(),
                });
            }

            if !physical.is_valid() {
                return Err(HardwareIntegrationError::InvalidPhysicalResource {
                    index: physical.index(),
                });
            }

            if self.physical_to_logical.get(physical) != Some(logical) {
                return Err(HardwareIntegrationError::InvalidSnapshot {
                    reason: "association indexes are not reciprocal",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Hardware calibration reference
// =============================================================================

/// Reference to a calibration snapshot used to produce a hardware observation.
///
/// The actual calibration object remains owned by the ZQN calibration subsystem
/// or hardware calibration subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HardwareCalibrationReference {
    snapshot_id: String,
    source: Option<String>,
}

impl HardwareCalibrationReference {
    /// Creates a calibration reference.
    pub fn new(
        snapshot_id: impl Into<String>,
    ) -> HardwareIntegrationResult<Self> {
        let snapshot_id = snapshot_id.into();

        validate_required_identifier("snapshot_id", &snapshot_id)?;

        Ok(Self {
            snapshot_id,
            source: None,
        })
    }

    /// Adds an optional source identifier.
    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();

        if !source.trim().is_empty() {
            self.source = Some(source);
        }

        self
    }

    /// Returns the calibration snapshot identifier.
    #[must_use]
    pub fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    /// Returns the optional source.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
}

// =============================================================================
// Hardware timing observation
// =============================================================================

/// Provider-neutral timing observation.
///
/// `std::time::Duration` is used here only as an integration convenience for
/// hardware observations. Canonical quantum semantic timing remains owned by
/// `quantum::ir::timing`.
///
/// Providers that expose sub-nanosecond or exact hardware units should preserve
/// those exact values in their own timing representation and convert only when
/// the conversion is lossless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareTimingObservation {
    duration: Duration,
    quality: ObservationQuality,
}

impl HardwareTimingObservation {
    /// Creates a timing observation.
    #[must_use]
    pub const fn new(
        duration: Duration,
        quality: ObservationQuality,
    ) -> Self {
        Self { duration, quality }
    }

    /// Returns the observed duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.duration
    }

    /// Returns observation quality.
    #[must_use]
    pub const fn quality(self) -> ObservationQuality {
        self.quality
    }

    /// Returns true when timing is available.
    #[must_use]
    pub const fn is_available(self) -> bool {
        self.quality.is_available()
    }
}

// =============================================================================
// Noise observation
// =============================================================================

/// A scalar hardware-derived noise observation.
///
/// This is intentionally not a quantum channel.
///
/// The observation can represent quantities such as:
///
/// - measured error probability;
/// - assignment error;
/// - leakage probability;
/// - loss probability;
/// - failure probability;
/// - bounded error estimate.
///
/// Interpretation belongs to the consuming ZQN domain module.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HardwareNoiseObservation {
    value: f64,
    quality: ObservationQuality,
    absolute_error: Option<f64>,
}

impl HardwareNoiseObservation {
    /// Creates a validated noise observation.
    ///
    /// The value must be finite and within `[0, 1]`.
    pub fn probability(
        value: f64,
        quality: ObservationQuality,
    ) -> HardwareIntegrationResult<Self> {
        validate_probability("noise_observation", value)?;

        if matches!(quality, ObservationQuality::Unavailable) {
            return Err(HardwareIntegrationError::InvalidExactness);
        }

        Ok(Self {
            value,
            quality,
            absolute_error: None,
        })
    }

    /// Adds an explicit non-negative absolute uncertainty.
    pub fn with_absolute_error(
        mut self,
        error: f64,
    ) -> HardwareIntegrationResult<Self> {
        validate_finite("absolute_error", error)?;

        if error < 0.0 {
            return Err(HardwareIntegrationError::NegativeErrorBound);
        }

        self.absolute_error = Some(error);
        Ok(self)
    }

    /// Returns the observed value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns observation quality.
    #[must_use]
    pub const fn quality(self) -> ObservationQuality {
        self.quality
    }

    /// Returns the optional absolute error.
    #[must_use]
    pub const fn absolute_error(self) -> Option<f64> {
        self.absolute_error
    }
}

// =============================================================================
// Resource noise observation
// =============================================================================

/// Noise observations associated with one hardware resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceNoiseObservation {
    resource: HardwareResource,
    gate_error: Option<HardwareNoiseObservation>,
    preparation_error: Option<HardwareNoiseObservation>,
    measurement_error: Option<HardwareNoiseObservation>,
    reset_error: Option<HardwareNoiseObservation>,
    idle_error: Option<HardwareNoiseObservation>,
    leakage: Option<HardwareNoiseObservation>,
    loss: Option<HardwareNoiseObservation>,
    timing: Option<HardwareTimingObservation>,
}

impl ResourceNoiseObservation {
    /// Creates an observation for a resource.
    pub fn new(
        resource: HardwareResource,
    ) -> HardwareIntegrationResult<Self> {
        resource.validate()?;

        Ok(Self {
            resource,
            gate_error: None,
            preparation_error: None,
            measurement_error: None,
            reset_error: None,
            idle_error: None,
            leakage: None,
            loss: None,
            timing: None,
        })
    }

    /// Returns the resource.
    #[must_use]
    pub fn resource(&self) -> &HardwareResource {
        &self.resource
    }

    /// Sets gate error.
    #[must_use]
    pub fn with_gate_error(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.gate_error = Some(observation);
        self
    }

    /// Sets preparation error.
    #[must_use]
    pub fn with_preparation_error(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.preparation_error = Some(observation);
        self
    }

    /// Sets measurement error.
    #[must_use]
    pub fn with_measurement_error(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.measurement_error = Some(observation);
        self
    }

    /// Sets reset error.
    #[must_use]
    pub fn with_reset_error(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.reset_error = Some(observation);
        self
    }

    /// Sets idle error.
    #[must_use]
    pub fn with_idle_error(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.idle_error = Some(observation);
        self
    }

    /// Sets leakage probability.
    #[must_use]
    pub fn with_leakage(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.leakage = Some(observation);
        self
    }

    /// Sets loss probability.
    #[must_use]
    pub fn with_loss(
        mut self,
        observation: HardwareNoiseObservation,
    ) -> Self {
        self.loss = Some(observation);
        self
    }

    /// Sets timing information.
    #[must_use]
    pub fn with_timing(
        mut self,
        observation: HardwareTimingObservation,
    ) -> Self {
        self.timing = Some(observation);
        self
    }

    /// Returns gate-error observation.
    #[must_use]
    pub const fn gate_error(&self) -> Option<HardwareNoiseObservation> {
        self.gate_error
    }

    /// Returns preparation-error observation.
    #[must_use]
    pub const fn preparation_error(
        &self,
    ) -> Option<HardwareNoiseObservation> {
        self.preparation_error
    }

    /// Returns measurement-error observation.
    #[must_use]
    pub const fn measurement_error(
        &self,
    ) -> Option<HardwareNoiseObservation> {
        self.measurement_error
    }

    /// Returns reset-error observation.
    #[must_use]
    pub const fn reset_error(&self) -> Option<HardwareNoiseObservation> {
        self.reset_error
    }

    /// Returns idle-error observation.
    #[must_use]
    pub const fn idle_error(&self) -> Option<HardwareNoiseObservation> {
        self.idle_error
    }

    /// Returns leakage observation.
    #[must_use]
    pub const fn leakage(&self) -> Option<HardwareNoiseObservation> {
        self.leakage
    }

    /// Returns loss observation.
    #[must_use]
    pub const fn loss(&self) -> Option<HardwareNoiseObservation> {
        self.loss
    }

    /// Returns timing observation.
    #[must_use]
    pub const fn timing(&self) -> Option<HardwareTimingObservation> {
        self.timing
    }
}

// =============================================================================
// Hardware operation observation
// =============================================================================

/// Provider-neutral observation associated with an executable operation.
///
/// The operation identity is intentionally a string rather than an enum so
/// that adding a new native operation does not require modifying ZQN.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareOperationObservation {
    operation_id: String,
    resources: BTreeSet<HardwareResource>,
    duration: Option<HardwareTimingObservation>,
    error: Option<HardwareNoiseObservation>,
    calibration: Option<HardwareCalibrationReference>,
}

impl HardwareOperationObservation {
    /// Creates an operation observation.
    pub fn new(
        operation_id: impl Into<String>,
    ) -> HardwareIntegrationResult<Self> {
        let operation_id = operation_id.into();

        validate_required_identifier("operation_id", &operation_id)?;

        Ok(Self {
            operation_id,
            resources: BTreeSet::new(),
            duration: None,
            error: None,
            calibration: None,
        })
    }

    /// Adds a resource used by the operation.
    pub fn add_resource(
        &mut self,
        resource: HardwareResource,
    ) -> HardwareIntegrationResult<bool> {
        resource.validate()?;
        Ok(self.resources.insert(resource))
    }

    /// Adds many resources from an iterator.
    ///
    /// The iterator is consumed incrementally; this method does not impose a
    /// semantic maximum on operation arity.
    pub fn add_resources<I>(
        &mut self,
        resources: I,
    ) -> HardwareIntegrationResult<usize>
    where
        I: IntoIterator<Item = HardwareResource>,
    {
        let mut inserted = 0usize;

        for resource in resources {
            if self.add_resource(resource)? {
                inserted = inserted
                    .checked_add(1)
                    .ok_or(HardwareIntegrationError::InvalidSnapshot {
                        reason: "resource count overflow",
                    })?;
            }
        }

        Ok(inserted)
    }

    /// Sets duration.
    #[must_use]
    pub fn with_duration(
        mut self,
        duration: HardwareTimingObservation,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets operation error.
    #[must_use]
    pub fn with_error(
        mut self,
        error: HardwareNoiseObservation,
    ) -> Self {
        self.error = Some(error);
        self
    }

    /// Associates calibration.
    #[must_use]
    pub fn with_calibration(
        mut self,
        calibration: HardwareCalibrationReference,
    ) -> Self {
        self.calibration = Some(calibration);
        self
    }

    /// Returns operation identifier.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Returns resources in deterministic order.
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = &HardwareResource> {
        self.resources.iter()
    }

    /// Returns operation duration.
    #[must_use]
    pub const fn duration(
        &self,
    ) -> Option<HardwareTimingObservation> {
        self.duration
    }

    /// Returns operation error.
    #[must_use]
    pub const fn error(
        &self,
    ) -> Option<HardwareNoiseObservation> {
        self.error
    }

    /// Returns calibration reference.
    #[must_use]
    pub fn calibration(
        &self,
    ) -> Option<&HardwareCalibrationReference> {
        self.calibration.as_ref()
    }

    /// Validates the observation.
    pub fn validate(&self) -> HardwareIntegrationResult<()> {
        validate_required_identifier("operation_id", &self.operation_id)?;

        for resource in &self.resources {
            resource.validate()?;
        }

        if let Some(error) = self.error {
            validate_probability("operation_error", error.value)?;

            if let Some(bound) = error.absolute_error {
                validate_finite("absolute_error", bound)?;

                if bound < 0.0 {
                    return Err(HardwareIntegrationError::NegativeErrorBound);
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Target identity
// =============================================================================

/// Provider-neutral target identity.
///
/// This is metadata, not a provider API handle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HardwareTargetIdentity {
    target_id: String,
    target_revision: Option<String>,
}

impl HardwareTargetIdentity {
    /// Creates a target identity.
    pub fn new(
        target_id: impl Into<String>,
    ) -> HardwareIntegrationResult<Self> {
        let target_id = target_id.into();

        validate_required_identifier("target_id", &target_id)?;

        Ok(Self {
            target_id,
            target_revision: None,
        })
    }

    /// Adds an optional revision.
    #[must_use]
    pub fn with_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        let revision = revision.into();

        if !revision.trim().is_empty() {
            self.target_revision = Some(revision);
        }

        self
    }

    /// Returns target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Returns target revision.
    #[must_use]
    pub fn target_revision(&self) -> Option<&str> {
        self.target_revision.as_deref()
    }
}

// =============================================================================
// Hardware snapshot
// =============================================================================

/// Immutable provider-neutral hardware snapshot.
///
/// A snapshot is a point-in-time semantic input to ZQN.
///
/// It MUST NOT perform live hardware queries.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareSnapshot {
    target: HardwareTargetIdentity,
    associations: QubitAssociations,
    resources: BTreeSet<HardwareResource>,
    operations: BTreeMap<String, HardwareOperationObservation>,
    resource_noise: BTreeMap<HardwareResource, ResourceNoiseObservation>,
    calibrations: BTreeSet<HardwareCalibrationReference>,
}

impl HardwareSnapshot {
    /// Creates an empty hardware snapshot.
    pub fn new(
        target: HardwareTargetIdentity,
    ) -> Self {
        Self {
            target,
            associations: QubitAssociations::new(),
            resources: BTreeSet::new(),
            operations: BTreeMap::new(),
            resource_noise: BTreeMap::new(),
            calibrations: BTreeSet::new(),
        }
    }

    /// Returns target identity.
    #[must_use]
    pub fn target(&self) -> &HardwareTargetIdentity {
        &self.target
    }

    /// Adds a hardware resource.
    pub fn add_resource(
        &mut self,
        resource: HardwareResource,
    ) -> HardwareIntegrationResult<bool> {
        resource.validate()?;
        Ok(self.resources.insert(resource))
    }

    /// Adds many resources incrementally.
    pub fn add_resources<I>(
        &mut self,
        resources: I,
    ) -> HardwareIntegrationResult<usize>
    where
        I: IntoIterator<Item = HardwareResource>,
    {
        let mut inserted = 0usize;

        for resource in resources {
            if self.add_resource(resource)? {
                inserted = inserted
                    .checked_add(1)
                    .ok_or(HardwareIntegrationError::InvalidSnapshot {
                        reason: "resource count overflow",
                    })?;
            }
        }

        Ok(inserted)
    }

    /// Adds a logical-to-physical association.
    pub fn add_association(
        &mut self,
        association: QubitAssociation,
    ) -> HardwareIntegrationResult<bool> {
        self.associations.insert(association)?;

        self.resources
            .insert(HardwareResource::LogicalQubit(association.logical()));
        self.resources
            .insert(HardwareResource::PhysicalQubit(association.physical()));

        Ok(true)
    }

    /// Adds an operation observation.
    pub fn add_operation(
        &mut self,
        operation: HardwareOperationObservation,
    ) -> HardwareIntegrationResult<bool> {
        operation.validate()?;

        let operation_id = operation.operation_id().to_owned();

        for resource in operation.resources() {
            self.resources.insert(resource.clone());
        }

        if let Some(calibration) = operation.calibration() {
            self.calibrations.insert(calibration.clone());
        }

        if self.operations.contains_key(&operation_id) {
            return Ok(false);
        }

        self.operations.insert(operation_id, operation);

        Ok(true)
    }

    /// Adds resource-level noise information.
    pub fn add_resource_noise(
        &mut self,
        observation: ResourceNoiseObservation,
    ) -> HardwareIntegrationResult<bool> {
        observation.resource().validate()?;

        if self
            .resource_noise
            .contains_key(observation.resource())
        {
            return Ok(false);
        }

        self.resources
            .insert(observation.resource().clone());

        self.resource_noise.insert(
            observation.resource().clone(),
            observation,
        );

        Ok(true)
    }

    /// Adds a calibration reference.
    pub fn add_calibration(
        &mut self,
        calibration: HardwareCalibrationReference,
    ) -> HardwareIntegrationResult<bool> {
        validate_required_identifier(
            "snapshot_id",
            calibration.snapshot_id(),
        )?;

        Ok(self.calibrations.insert(calibration))
    }

    /// Returns logical/physical associations.
    #[must_use]
    pub fn associations(&self) -> &QubitAssociations {
        &self.associations
    }

    /// Returns all resources in deterministic order.
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = &HardwareResource> {
        self.resources.iter()
    }

    /// Returns the number of resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns an operation observation.
    #[must_use]
    pub fn operation(
        &self,
        operation_id: &str,
    ) -> Option<&HardwareOperationObservation> {
        self.operations.get(operation_id)
    }

    /// Iterates operation observations deterministically.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = &HardwareOperationObservation> {
        self.operations.values()
    }

    /// Returns resource-level noise observations.
    #[must_use]
    pub fn resource_noise(
        &self,
        resource: &HardwareResource,
    ) -> Option<&ResourceNoiseObservation> {
        self.resource_noise.get(resource)
    }

    /// Iterates resource-level observations deterministically.
    pub fn resource_noise_observations(
        &self,
    ) -> impl Iterator<Item = &ResourceNoiseObservation> {
        self.resource_noise.values()
    }

    /// Iterates calibration references deterministically.
    pub fn calibrations(
        &self,
    ) -> impl Iterator<Item = &HardwareCalibrationReference> {
        self.calibrations.iter()
    }

    /// Validates the complete snapshot.
    pub fn validate(&self) -> HardwareIntegrationResult<()> {
        self.associations.validate()?;

        for resource in &self.resources {
            resource.validate()?;
        }

        for operation in self.operations.values() {
            operation.validate()?;
        }

        for observation in self.resource_noise.values() {
            observation.resource().validate()?;

            validate_optional_noise(
                observation.gate_error(),
                "gate_error",
            )?;
            validate_optional_noise(
                observation.preparation_error(),
                "preparation_error",
            )?;
            validate_optional_noise(
                observation.measurement_error(),
                "measurement_error",
            )?;
            validate_optional_noise(
                observation.reset_error(),
                "reset_error",
            )?;
            validate_optional_noise(
                observation.idle_error(),
                "idle_error",
            )?;
            validate_optional_noise(
                observation.leakage(),
                "leakage",
            )?;
            validate_optional_noise(
                observation.loss(),
                "loss",
            )?;
        }

        for calibration in &self.calibrations {
            validate_required_identifier(
                "snapshot_id",
                calibration.snapshot_id(),
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Hardware observation source
// =============================================================================

/// Stable provider-neutral source contract.
///
/// A hardware adapter implements this trait to expose a snapshot to ZQN.
///
/// The trait intentionally contains no provider-specific methods.
///
/// It is object-safe, so registries can hold:
///
/// ```text
/// Box<dyn HardwareObservationSource>
/// Arc<dyn HardwareObservationSource>
/// ```
///
/// The source is not required to own the snapshot permanently. A provider may
/// construct one on demand, cache it, or expose an immutable pre-fetched
/// snapshot.
pub trait HardwareObservationSource {
    /// Returns the provider-neutral target identity.
    fn target_identity(&self) -> HardwareIntegrationResult<HardwareTargetIdentity>;

    /// Returns an immutable snapshot of currently known hardware facts.
    ///
    /// Implementations MUST NOT expose credentials through the snapshot.
    fn snapshot(&self) -> HardwareIntegrationResult<HardwareSnapshot>;

    /// Returns true when the source can provide a fresh snapshot without
    /// violating its own contract.
    ///
    /// This does not cause a refresh.
    fn supports_refresh(&self) -> bool {
        false
    }
}

/// Convenience adapter around an immutable snapshot.
///
/// This is useful for simulators, emulators, tests, replay systems, and
/// hardware adapters that already have a normalized snapshot.
#[derive(Debug, Clone)]
pub struct StaticHardwareObservationSource {
    snapshot: HardwareSnapshot,
}

impl StaticHardwareObservationSource {
    /// Creates a source from a validated snapshot.
    pub fn new(
        snapshot: HardwareSnapshot,
    ) -> HardwareIntegrationResult<Self> {
        snapshot.validate()?;

        Ok(Self { snapshot })
    }

    /// Returns the contained snapshot.
    #[must_use]
    pub fn snapshot_ref(&self) -> &HardwareSnapshot {
        &self.snapshot
    }
}

impl HardwareObservationSource for StaticHardwareObservationSource {
    fn target_identity(
        &self,
    ) -> HardwareIntegrationResult<HardwareTargetIdentity> {
        Ok(self.snapshot.target().clone())
    }

    fn snapshot(
        &self,
    ) -> HardwareIntegrationResult<HardwareSnapshot> {
        Ok(self.snapshot.clone())
    }
}

// =============================================================================
// Integration policy
// =============================================================================

/// Policy controlling how hardware observations may be consumed.
///
/// This policy does not change the observation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareObservationPolicy {
    /// Accept only exact observations.
    ExactOnly,

    /// Accept exact or directly measured observations.
    ExactOrMeasured,

    /// Accept measured and estimated observations.
    AllowEstimated,

    /// Accept explicitly approximate observations.
    AllowApproximate,

    /// Accept all available observations, preserving their quality metadata.
    AnyAvailable,
}

impl Default for HardwareObservationPolicy {
    fn default() -> Self {
        Self::ExactOrMeasured
    }
}

impl HardwareObservationPolicy {
    /// Returns whether the policy accepts a particular quality.
    #[must_use]
    pub const fn accepts(
        self,
        quality: ObservationQuality,
    ) -> bool {
        match self {
            Self::ExactOnly => matches!(quality, ObservationQuality::Exact),

            Self::ExactOrMeasured => matches!(
                quality,
                ObservationQuality::Exact
                    | ObservationQuality::Measured
            ),

            Self::AllowEstimated => matches!(
                quality,
                ObservationQuality::Exact
                    | ObservationQuality::Measured
                    | ObservationQuality::Estimated
                    | ObservationQuality::Statistical
            ),

            Self::AllowApproximate => matches!(
                quality,
                ObservationQuality::Exact
                    | ObservationQuality::Measured
                    | ObservationQuality::Estimated
                    | ObservationQuality::Approximate
                    | ObservationQuality::Statistical
            ),

            Self::AnyAvailable => quality.is_available(),
        }
    }
}

// =============================================================================
// Integration view
// =============================================================================

/// Immutable view consumed by ZQN domain modules.
///
/// The view prevents consumers from modifying the original hardware snapshot.
///
/// It also makes the observation-acceptance policy explicit.
#[derive(Debug, Clone)]
pub struct HardwareIntegrationView {
    snapshot: HardwareSnapshot,
    policy: HardwareObservationPolicy,
}

impl HardwareIntegrationView {
    /// Creates a validated integration view.
    pub fn new(
        snapshot: HardwareSnapshot,
        policy: HardwareObservationPolicy,
    ) -> HardwareIntegrationResult<Self> {
        snapshot.validate()?;

        Ok(Self { snapshot, policy })
    }

    /// Returns the observation policy.
    #[must_use]
    pub const fn policy(&self) -> HardwareObservationPolicy {
        self.policy
    }

    /// Returns the immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &HardwareSnapshot {
        &self.snapshot
    }

    /// Returns whether an observation quality is accepted.
    #[must_use]
    pub const fn accepts_quality(
        &self,
        quality: ObservationQuality,
    ) -> bool {
        self.policy.accepts(quality)
    }

    /// Returns an accepted operation observation.
    pub fn operation(
        &self,
        operation_id: &str,
    ) -> HardwareIntegrationResult<Option<&HardwareOperationObservation>> {
        let observation = self.snapshot.operation(operation_id);

        if let Some(observation) = observation {
            if let Some(error) = observation.error() {
                if !self.accepts_quality(error.quality()) {
                    return Err(
                        HardwareIntegrationError::InvalidExactness,
                    );
                }
            }

            if let Some(duration) = observation.duration() {
                if !self.accepts_quality(duration.quality()) {
                    return Err(
                        HardwareIntegrationError::InvalidExactness,
                    );
                }
            }
        }

        Ok(observation)
    }

    /// Returns an accepted resource noise observation.
    pub fn resource_noise(
        &self,
        resource: &HardwareResource,
    ) -> HardwareIntegrationResult<
        Option<&ResourceNoiseObservation>,
    > {
        let observation = self.snapshot.resource_noise(resource);

        if let Some(observation) = observation {
            let observations = [
                observation.gate_error(),
                observation.preparation_error(),
                observation.measurement_error(),
                observation.reset_error(),
                observation.idle_error(),
                observation.leakage(),
                observation.loss(),
            ];

            for value in observations.into_iter().flatten() {
                if !self.accepts_quality(value.quality()) {
                    return Err(
                        HardwareIntegrationError::InvalidExactness,
                    );
                }
            }

            if let Some(timing) = observation.timing() {
                if !self.accepts_quality(timing.quality()) {
                    return Err(
                        HardwareIntegrationError::InvalidExactness,
                    );
                }
            }
        }

        Ok(observation)
    }
}

// =============================================================================
// Source validation
// =============================================================================

/// Validates and snapshots a hardware source.
///
/// This is the principal entry point for hardware adapters entering ZQN.
pub fn import_hardware_source(
    source: &dyn HardwareObservationSource,
) -> HardwareIntegrationResult<HardwareSnapshot> {
    let target = source.target_identity()?;
    let snapshot = source.snapshot()?;

    if snapshot.target() != &target {
        return Err(HardwareIntegrationError::InvalidSnapshot {
            reason: "source target identity differs from snapshot target",
        });
    }

    snapshot.validate()?;

    Ok(snapshot)
}

/// Imports a source under an explicit observation policy.
pub fn import_hardware_view(
    source: &dyn HardwareObservationSource,
    policy: HardwareObservationPolicy,
) -> HardwareIntegrationResult<HardwareIntegrationView> {
    let snapshot = import_hardware_source(source)?;

    HardwareIntegrationView::new(snapshot, policy)
}

// =============================================================================
// Numeric validation helpers
// =============================================================================

fn validate_required_identifier(
    field: &'static str,
    value: &str,
) -> HardwareIntegrationResult<()> {
    if value.trim().is_empty() {
        return Err(HardwareIntegrationError::EmptyIdentifier { field });
    }

    Ok(())
}

fn validate_finite(
    field: &'static str,
    value: f64,
) -> HardwareIntegrationResult<()> {
    if !value.is_finite() {
        return Err(HardwareIntegrationError::InvalidNumericValue {
            field,
        });
    }

    Ok(())
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> HardwareIntegrationResult<()> {
    validate_finite(field, value)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(HardwareIntegrationError::InvalidProbability {
            field,
        });
    }

    Ok(())
}

fn validate_optional_noise(
    value: Option<HardwareNoiseObservation>,
    field: &'static str,
) -> HardwareIntegrationResult<()> {
    if let Some(value) = value {
        validate_probability(field, value.value)?;

        if let Some(error) = value.absolute_error {
            validate_finite("absolute_error", error)?;

            if error < 0.0 {
                return Err(HardwareIntegrationError::NegativeErrorBound);
            }
        }

        if !value.quality.is_available() {
            return Err(HardwareIntegrationError::InvalidExactness);
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_qubit_ids_are_used() {
        let logical = QubitId::new(0);
        let physical = PhysicalQubitId::new(7);

        let association =
            QubitAssociation::new(logical, physical)
                .expect("valid canonical IDs");

        assert_eq!(association.logical(), logical);
        assert_eq!(association.physical(), physical);
    }

    #[test]
    fn associations_are_deterministic_and_bijective() {
        let mut associations = QubitAssociations::new();

        let first = QubitAssociation::new(
            QubitId::new(0),
            PhysicalQubitId::new(3),
        )
        .expect("valid association");

        let second = QubitAssociation::new(
            QubitId::new(1),
            PhysicalQubitId::new(7),
        )
        .expect("valid association");

        assert!(associations.insert(first).expect("insert"));
        assert!(associations.insert(second).expect("insert"));

        assert_eq!(
            associations.physical_for(QubitId::new(0)),
            Some(PhysicalQubitId::new(3))
        );

        assert_eq!(
            associations.logical_for(PhysicalQubitId::new(7)),
            Some(QubitId::new(1))
        );

        assert!(associations.validate().is_ok());
    }

    #[test]
    fn conflicting_logical_mapping_is_rejected() {
        let mut associations = QubitAssociations::new();

        let first = QubitAssociation::new(
            QubitId::new(0),
            PhysicalQubitId::new(3),
        )
        .expect("valid association");

        let second = QubitAssociation::new(
            QubitId::new(0),
            PhysicalQubitId::new(4),
        )
        .expect("valid association");

        associations.insert(first).expect("first insert");

        let result = associations.insert(second);

        assert!(matches!(
            result,
            Err(
                HardwareIntegrationError::ConflictingAssociation { .. }
            )
        ));
    }

    #[test]
    fn physical_resource_cannot_be_assigned_twice() {
        let mut associations = QubitAssociations::new();

        let first = QubitAssociation::new(
            QubitId::new(0),
            PhysicalQubitId::new(3),
        )
        .expect("valid association");

        let second = QubitAssociation::new(
            QubitId::new(1),
            PhysicalQubitId::new(3),
        )
        .expect("valid association");

        associations.insert(first).expect("first insert");

        let result = associations.insert(second);

        assert!(matches!(
            result,
            Err(
                HardwareIntegrationError::PhysicalResourceAlreadyAssigned {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(matches!(
            HardwareNoiseObservation::probability(
                1.1,
                ObservationQuality::Measured
            ),
            Err(HardwareIntegrationError::InvalidProbability { .. })
        ));

        assert!(matches!(
            HardwareNoiseObservation::probability(
                f64::NAN,
                ObservationQuality::Measured
            ),
            Err(HardwareIntegrationError::InvalidNumericValue { .. })
        ));

        assert!(matches!(
            HardwareNoiseObservation::probability(
                f64::INFINITY,
                ObservationQuality::Measured
            ),
            Err(HardwareIntegrationError::InvalidNumericValue { .. })
        ));
    }

    #[test]
    fn negative_uncertainty_is_rejected() {
        let observation =
            HardwareNoiseObservation::probability(
                0.01,
                ObservationQuality::Measured,
            )
            .expect("valid observation");

        assert!(matches!(
            observation.with_absolute_error(-0.1),
            Err(HardwareIntegrationError::NegativeErrorBound)
        ));
    }

    #[test]
    fn unavailable_quality_is_not_accepted_as_probability() {
        let result =
            HardwareNoiseObservation::probability(
                0.1,
                ObservationQuality::Unavailable,
            );

        assert!(matches!(
            result,
            Err(HardwareIntegrationError::InvalidExactness)
        ));
    }

    #[test]
    fn opaque_resources_support_non_qubit_architectures() {
        let mode =
            HardwareResource::opaque("bosonic-mode:0")
                .expect("valid resource");

        assert!(mode.is_opaque());
        assert!(mode.validate().is_ok());
    }

    #[test]
    fn empty_opaque_resource_is_rejected() {
        let result = HardwareResource::opaque(" ");

        assert!(matches!(
            result,
            Err(HardwareIntegrationError::EmptyIdentifier { .. })
        ));
    }

    #[test]
    fn observation_policy_preserves_exactness() {
        assert!(
            HardwareObservationPolicy::ExactOnly
                .accepts(ObservationQuality::Exact)
        );

        assert!(
            !HardwareObservationPolicy::ExactOnly
                .accepts(ObservationQuality::Measured)
        );

        assert!(
            HardwareObservationPolicy::ExactOrMeasured
                .accepts(ObservationQuality::Measured)
        );

        assert!(
            !HardwareObservationPolicy::ExactOrMeasured
                .accepts(ObservationQuality::Estimated)
        );

        assert!(
            HardwareObservationPolicy::AllowEstimated
                .accepts(ObservationQuality::Estimated)
        );

        assert!(
            HardwareObservationPolicy::AllowApproximate
                .accepts(ObservationQuality::Approximate)
        );
    }

    #[test]
    fn snapshot_is_deterministic() {
        let target =
            HardwareTargetIdentity::new("test-target")
                .expect("valid target");

        let mut snapshot = HardwareSnapshot::new(target);

        snapshot
            .add_resource(HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(8),
            ))
            .expect("resource");

        snapshot
            .add_resource(HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(2),
            ))
            .expect("resource");

        let resources = snapshot
            .resources()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            resources,
            vec![
                HardwareResource::PhysicalQubit(
                    PhysicalQubitId::new(2)
                ),
                HardwareResource::PhysicalQubit(
                    PhysicalQubitId::new(8)
                ),
            ]
        );
    }

    #[test]
    fn operation_observation_is_provider_neutral() {
        let mut operation =
            HardwareOperationObservation::new("native_operation")
                .expect("valid operation");

        operation
            .add_resource(HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(0),
            ))
            .expect("resource");

        operation
            .add_resource(HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(1),
            ))
            .expect("resource");

        operation.duration = Some(
            HardwareTimingObservation::new(
                Duration::from_nanos(10),
                ObservationQuality::Measured,
            ),
        );

        operation.error = Some(
            HardwareNoiseObservation::probability(
                0.001,
                ObservationQuality::Measured,
            )
            .expect("valid error"),
        );

        assert!(operation.validate().is_ok());
    }

    #[test]
    fn static_source_round_trips_snapshot() {
        let target =
            HardwareTargetIdentity::new("simulator")
                .expect("valid target");

        let snapshot = HardwareSnapshot::new(target);

        let source =
            StaticHardwareObservationSource::new(snapshot.clone())
                .expect("valid source");

        let imported =
            import_hardware_source(&source)
                .expect("source import");

        assert_eq!(imported, snapshot);
    }

    #[test]
    fn source_view_enforces_quality_policy() {
        let target =
            HardwareTargetIdentity::new("test")
                .expect("valid target");

        let mut snapshot =
            HardwareSnapshot::new(target);

        let observation =
            ResourceNoiseObservation::new(
                HardwareResource::PhysicalQubit(
                    PhysicalQubitId::new(0),
                ),
            )
            .expect("valid resource")
            .with_gate_error(
                HardwareNoiseObservation::probability(
                    0.01,
                    ObservationQuality::Estimated,
                )
                .expect("valid observation"),
            );

        snapshot
            .add_resource_noise(observation)
            .expect("add observation");

        let source =
            StaticHardwareObservationSource::new(snapshot)
                .expect("source");

        let view =
            import_hardware_view(
                &source,
                HardwareObservationPolicy::ExactOrMeasured,
            )
            .expect("view");

        let result = view.resource_noise(
            &HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(0),
            ),
        );

        assert!(matches!(
            result,
            Err(HardwareIntegrationError::InvalidExactness)
        ));
    }

    #[test]
    fn source_view_accepts_explicit_estimates_when_policy_allows() {
        let target =
            HardwareTargetIdentity::new("test")
                .expect("valid target");

        let mut snapshot =
            HardwareSnapshot::new(target);

        let observation =
            ResourceNoiseObservation::new(
                HardwareResource::PhysicalQubit(
                    PhysicalQubitId::new(0),
                ),
            )
            .expect("valid resource")
            .with_gate_error(
                HardwareNoiseObservation::probability(
                    0.01,
                    ObservationQuality::Estimated,
                )
                .expect("valid observation"),
            );

        snapshot
            .add_resource_noise(observation)
            .expect("add observation");

        let source =
            StaticHardwareObservationSource::new(snapshot)
                .expect("source");

        let view =
            import_hardware_view(
                &source,
                HardwareObservationPolicy::AllowEstimated,
            )
            .expect("view");

        let result = view.resource_noise(
            &HardwareResource::PhysicalQubit(
                PhysicalQubitId::new(0),
            ),
        );

        assert!(result.is_ok());
        assert!(result.expect("result").is_some());
    }

    #[test]
    fn large_resource_counts_have_no_semantic_ceiling() {
        let target =
            HardwareTargetIdentity::new("scalable")
                .expect("valid target");

        let mut snapshot =
            HardwareSnapshot::new(target);

        for index in 0_u64..4096 {
            snapshot
                .add_resource(
                    HardwareResource::PhysicalQubit(
                        PhysicalQubitId::new(index),
                    ),
                )
                .expect("resource");

            // The test intentionally generates resources rather than relying
            // on a hard-coded architectural maximum.
        }

        assert_eq!(snapshot.resource_count(), 4096);
        assert!(snapshot.validate().is_ok());
    }
}