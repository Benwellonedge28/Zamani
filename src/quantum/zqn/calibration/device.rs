//! Zamani Quantum Noise (ZQN) — Device Calibration Scope and Resource Index.
//!
//! # Ownership
//!
//! This module owns the target-independent relationship between:
//!
//! - a ZQN `CalibrationId`;
//! - an abstract calibration scope/resource;
//! - a target/device reference;
//! - the physical quantum resources to which that calibration applies.
//!
//! In other words, this file answers:
//!
//! > "Which calibration identity applies to which resource of this target?"
//!
//! It does NOT own the actual measured calibration values.
//!
//! # Does not own
//!
//! This module deliberately does not own:
//!
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - hardware topology;
//! - provider APIs;
//! - credentials;
//! - QPU transport;
//! - calibration experiments;
//! - calibration measurements;
//! - numerical calibration parameters;
//! - calibration interpolation;
//! - calibration drift estimation;
//! - calibration snapshot persistence;
//! - routing;
//! - scheduling;
//! - execution;
//! - benchmarking;
//! - noise channels;
//! - noise models;
//! - QEC;
//! - global registries;
//! - global mutable state.
//!
//! The existing `quantum::hardware::calibration` subsystem owns authoritative
//! hardware calibration state/snapshots. ZQN consumes that evidence through
//! integration boundaries. This file must therefore remain independent of the
//! concrete hardware calibration implementation.
//!
//! # Why this layer exists
//!
//! A calibration value without a precise resource scope is ambiguous.
//!
//! For example:
//!
//! ```text
//! calibration #17
//! ```
//!
//! is insufficient to determine whether the calibration applies to:
//!
//! - the whole device;
//! - physical qubit 7;
//! - a pair of physical qubits;
//! - an instruction;
//! - a custom physical resource.
//!
//! `CalibrationScope` supplies that missing semantic boundary.
//!
//! # Canonical quantum-resource identity
//!
//! ZQN MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! for physical-qubit identity.
//!
//! This file intentionally does NOT define another `QubitId`,
//! `PhysicalQubitId`, integer wrapper, or hardware-specific qubit identity.
//!
//! The repository's ZQN identity layer already establishes that quantum
//! resource identity belongs to `quantum::ir::qubit`, while ZQN owns only
//! identities for ZQN-domain objects such as `CalibrationId`.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum number of:
//!
//! - devices;
//! - physical qubits;
//! - calibration records;
//! - calibration scopes;
//! - instructions;
//! - resources;
//! - calibration generations.
//!
//! A `usize` collection length is used only because Rust collections require a
//! host-platform index type. It is not a quantum-machine-size declaration.
//!
//! Resource limits are represented separately by `DeviceCalibrationLimits`.
//!
//! `None` in a resource limit means that this module itself imposes no limit.
//! Callers processing untrusted or externally supplied data should provide an
//! explicit policy.
//!
//! # Resource safety
//!
//! This module never silently allocates based on an attacker-controlled count.
//!
//! APIs that insert individual records perform ordinary allocation only for
//! the requested record. Bulk construction can be protected with
//! `DeviceCalibrationLimits` before insertion.
//!
//! The implementation:
//!
//! - performs checked collection-size arithmetic;
//! - supports explicit limits;
//! - validates string sizes;
//! - validates instruction/resource arity;
//! - rejects duplicate scope registrations;
//! - does not use global caches;
//! - does not use hidden registries;
//! - does not use `unsafe`;
//! - does not use hidden randomness.
//!
//! # Determinism
//!
//! Deterministic ordering is provided by `BTreeMap` and `BTreeSet`.
//!
//! Iteration therefore does not depend on hash-map iteration order.
//!
//! The module does not use:
//!
//! - random numbers;
//! - wall-clock time;
//! - thread identity;
//! - process identity;
//! - memory addresses;
//! - global mutable state.
//!
//! The calibration identity itself is supplied by the owning calibration/
//! provenance layer. This module does not invent identity values.
//!
//! # Calibration identity versus calibration state
//!
//! A `CalibrationId` identifies ZQN calibration evidence.
//!
//! `CalibrationScope` identifies where that evidence applies.
//!
//! The actual calibration values remain elsewhere.
//!
//! Conceptually:
//!
//! ```text
//! hardware/provider adapter
//!          |
//!          v
//! CalibrationSnapshot
//!          |
//!          | owns measured values/evidence
//!          v
//!      CalibrationId
//!          |
//!          v
//! ZQN DeviceCalibration
//!          |
//!          +---- scope -> PhysicalQubitId
//!          |
//!          +---- scope -> instruction/resource
//! ```
//!
//! This prevents ZQN from creating a second hardware-calibration authority.
//!
//! # Integration
//!
//! ```text
//! quantum::hardware::calibration
//!             |
//!             | calibration evidence
//!             v
//!      CalibrationSnapshot
//!             |
//!             | CalibrationId / provenance binding
//!             v
//! ZQN calibration/device.rs
//!             |
//!       +-----+------+----------------+
//!       |            |                |
//!       v            v                v
//!     noise       routing         scheduling
//!       |            |                |
//!       +------------+----------------+
//!                    |
//!                    v
//!                execution
//! ```
//!
//! The integration boundary is intentionally identifier/scope based. This
//! keeps ZQN independent from provider-specific calibration structures.
//!
//! # Hardware integration
//!
//! Provider adapters must NOT call this module to retrieve credentials or
//! communicate with hardware.
//!
//! Instead they construct/obtain authoritative calibration state and expose a
//! stable `CalibrationId`. An integration layer then associates that ID with a
//! `CalibrationScope`.
//!
//! # Routing integration
//!
//! Routing may query an exact physical-qubit scope:
//!
//! ```text
//! DeviceCalibration::calibration_for(
//!     &CalibrationScope::PhysicalQubit(q),
//! )
//! ```
//!
//! Routing remains responsible for routing decisions. This file only answers
//! which calibration evidence is associated with a resource.
//!
//! # Scheduling integration
//!
//! Scheduling may query instruction or resource scopes. This module does not
//! decide whether an operation should be scheduled.
//!
//! # ZQN noise integration
//!
//! `noise/*` can use the calibration identity to obtain calibrated parameters
//! through the appropriate calibration parameter/snapshot integration layer.
//!
//! This file does not depend on `noise/*`, preventing a circular dependency.
//!
//! # QEC integration
//!
//! QEC may use physical-qubit scopes when constructing physical error models.
//! QEC remains responsible for syndrome processing, decoding and correction.
//!
//! # Serialization
//!
//! This module intentionally does not establish a wire format.
//!
//! The canonical ZQN `io/*` subsystem is responsible for serialization,
//! schema versions, canonical encoding and compatibility migration.
//!
//! The in-memory ordering is deterministic, so an IO implementation can
//! serialize the contents canonically without depending on hash-map ordering.
//!
//! # Security
//!
//! Device references and custom resource names are identifiers, not
//! authentication credentials.
//!
//! They MUST NOT be interpreted as:
//!
//! - API keys;
//! - passwords;
//! - access tokens;
//! - provider credentials;
//! - authorization capabilities.
//!
//! This module does not perform authorization.
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
//! - no `unsafe`.
//!
//! # Testing contract
//!
//! Tests owned by this file should verify:
//!
//! 1. canonical `PhysicalQubitId` is used;
//! 2. empty identifiers are rejected;
//! 3. malformed identifiers are rejected;
//! 4. duplicate scopes are rejected;
//! 5. replacement is explicit;
//! 6. arbitrary physical-qubit IDs work;
//! 7. non-contiguous physical-qubit IDs work;
//! 8. multi-qubit scopes are deterministic;
//! 9. resource limits are policy-driven;
//! 10. no semantic machine-size ceiling exists;
//! 11. merge conflicts are deterministic;
//! 12. iteration is deterministic;
//! 13. no global state is required;
//! 14. no unsafe code is present;
//! 15. Rust 1.97/1.97.1 compilation succeeds.
//!
//! # Important distinction
//!
//! `DeviceCalibration` is NOT a hardware device object.
//!
//! It is a ZQN calibration-resource index for a target.
//!
//! Hardware identity, topology and lifecycle remain owned by the hardware
//! subsystem.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{btree_map, BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::qubit::PhysicalQubitId;
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::CalibrationId;

// ============================================================================
// Constants
// ============================================================================

/// Logical namespace used by this module for diagnostic messages.
///
/// This is a semantic namespace, not a hardware/provider identifier.
pub const CALIBRATION_DEVICE_COMPONENT: &str =
    "zqn.calibration.device";

/// Default policy used only for maximum UTF-8 byte length of a device
/// reference.
///
/// This is a safety policy, not a machine-size limit.
pub const DEFAULT_DEVICE_REFERENCE_MAX_BYTES: usize = 4096;

/// Default policy used only for maximum UTF-8 byte length of a provider
/// reference.
pub const DEFAULT_PROVIDER_REFERENCE_MAX_BYTES: usize = 4096;

/// Default policy used only for maximum UTF-8 byte length of a resource
/// namespace.
pub const DEFAULT_RESOURCE_NAMESPACE_MAX_BYTES: usize = 4096;

/// Default policy used only for maximum UTF-8 byte length of a resource key.
pub const DEFAULT_RESOURCE_KEY_MAX_BYTES: usize = 4096;

/// Default policy used only for maximum UTF-8 byte length of an instruction
/// name.
pub const DEFAULT_INSTRUCTION_NAME_MAX_BYTES: usize = 4096;

// ============================================================================
// Calibration scope
// ============================================================================

/// The resource to which a calibration identity applies.
///
/// This enum deliberately models *scope*, not calibration values.
///
/// # Design
///
/// The enum has explicit physical-qubit support because qubit identity is a
/// canonical IR concern. `NamedResource` exists so ZQN is not restricted to
/// qubit gate machines.
///
/// This permits future systems such as:
///
/// - bosonic modes;
//! - continuous-variable modes;
//! - photonic paths;
//! - transport channels;
//! - analog resources;
//! - distributed links;
//! - logical resources;
//! - future quantum modalities.
///
/// without requiring this module to define a new hardware-specific identity
/// system.
///
/// # Ordering
///
/// The enum derives `Ord` so `BTreeMap` can provide deterministic ordering.
///
/// Ordering is an implementation detail and MUST NOT be interpreted as
/// hardware priority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CalibrationScope {
    /// Calibration applying to the whole target/device.
    Device,

    /// Calibration applying to one physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Calibration applying to an explicitly defined set of physical qubits.
    ///
    /// The `BTreeSet` guarantees deterministic ordering and removes duplicate
    /// resource identities.
    PhysicalQubits(BTreeSet<PhysicalQubitId>),

    /// Calibration applying to an instruction and its physical operands.
    ///
    /// The instruction name is semantic and provider-independent. Provider
    /// adapters may map native instructions to this semantic name at the
    /// integration boundary.
    Instruction {
        /// Semantic instruction name.
        name: String,

        /// Physical resources participating in the instruction.
        qubits: Vec<PhysicalQubitId>,
    },

    /// Calibration applying to an arbitrary named resource.
    ///
    /// This is the escape hatch for technologies that cannot be expressed as
    /// qubit/instruction resources.
    NamedResource {
        /// Stable semantic namespace.
        namespace: String,

        /// Stable resource key within the namespace.
        key: String,
    },
}

impl CalibrationScope {
    /// Creates a whole-device calibration scope.
    #[must_use]
    pub const fn device() -> Self {
        Self::Device
    }

    /// Creates a single-physical-qubit calibration scope.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a multi-physical-qubit calibration scope.
    ///
    /// Duplicate qubits are removed deterministically.
    ///
    /// An empty set is rejected by [`Self::validate`].
    #[must_use]
    pub fn physical_qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        Self::PhysicalQubits(qubits.into_iter().collect())
    }

    /// Creates an instruction calibration scope.
    ///
    /// The instruction name is not interpreted by this module.
    ///
    /// An empty instruction name or empty operand list is rejected during
    /// validation.
    #[must_use]
    pub fn instruction<I>(
        name: impl Into<String>,
        qubits: I,
    ) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        Self::Instruction {
            name: name.into(),
            qubits: qubits.into_iter().collect(),
        }
    }

    /// Creates a named non-qubit resource scope.
    #[must_use]
    pub fn named_resource(
        namespace: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        Self::NamedResource {
            namespace: namespace.into(),
            key: key.into(),
        }
    }

    /// Returns the number of physical qubits explicitly named by this scope.
    ///
    /// For a whole-device or arbitrary named resource scope, this returns
    /// zero because no physical-qubit set is implied.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        match self {
            Self::Device | Self::NamedResource { .. } => 0,

            Self::PhysicalQubit(_) => 1,

            Self::PhysicalQubits(qubits) => qubits.len(),

            Self::Instruction { qubits, .. } => qubits.len(),
        }
    }

    /// Returns true if this scope explicitly references the supplied physical
    /// qubit.
    #[must_use]
    pub fn references_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        match self {
            Self::Device | Self::NamedResource { .. } => false,

            Self::PhysicalQubit(candidate) => *candidate == qubit,

            Self::PhysicalQubits(qubits) => qubits.contains(&qubit),

            Self::Instruction { qubits, .. } => {
                qubits.contains(&qubit)
            }
        }
    }

    /// Validates semantic scope invariants.
    ///
    /// This method does not validate whether a physical qubit actually exists
    /// on a target. Existence belongs to hardware capabilities/topology.
    pub fn validate(
        &self,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<()> {
        match self {
            Self::Device => Ok(()),

            Self::PhysicalQubit(_) => Ok(()),

            Self::PhysicalQubits(qubits) => {
                if qubits.is_empty() {
                    return Err(invalid_calibration(
                        ZqnErrorCode::InvalidCalibration,
                        "physical-qubit calibration scope cannot be empty",
                    ));
                }

                if let Some(maximum) =
                    limits.max_scope_physical_qubits
                {
                    if qubits.len() > maximum {
                        return Err(limit_exceeded(format!(
                            "physical-qubit scope contains {} resources; \
                             configured maximum is {maximum}",
                            qubits.len()
                        )));
                    }
                }

                Ok(())
            }

            Self::Instruction { name, qubits } => {
                validate_text(
                    "calibration instruction name",
                    name,
                    limits.max_instruction_name_bytes,
                )?;

                if qubits.is_empty() {
                    return Err(invalid_calibration(
                        ZqnErrorCode::InvalidCalibration,
                        "instruction calibration scope requires at least \
                         one physical resource",
                    ));
                }

                if let Some(maximum) =
                    limits.max_instruction_physical_qubits
                {
                    if qubits.len() > maximum {
                        return Err(limit_exceeded(format!(
                            "instruction calibration scope contains {} \
                             physical resources; configured maximum is \
                             {maximum}",
                            qubits.len()
                        )));
                    }
                }

                let mut seen = BTreeSet::new();

                for qubit in qubits {
                    if !seen.insert(*qubit) {
                        return Err(invalid_calibration(
                            ZqnErrorCode::DuplicateIdentifier,
                            format!(
                                "instruction calibration scope contains \
                                 physical qubit {qubit:?} more than once"
                            ),
                        ));
                    }
                }

                Ok(())
            }

            Self::NamedResource { namespace, key } => {
                validate_text(
                    "calibration resource namespace",
                    namespace,
                    limits.max_resource_namespace_bytes,
                )?;

                validate_text(
                    "calibration resource key",
                    key,
                    limits.max_resource_key_bytes,
                )?;

                Ok(())
            }
        }
    }

    /// Returns a deterministic human-readable scope description.
    ///
    /// This is diagnostic text, not a stable serialization format.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Device => "device".to_string(),

            Self::PhysicalQubit(qubit) => {
                format!("physical-qubit:{qubit:?}")
            }

            Self::PhysicalQubits(qubits) => {
                format!(
                    "physical-qubits:{}",
                    qubits.len()
                )
            }

            Self::Instruction { name, qubits } => {
                format!(
                    "instruction:{name}[{}]",
                    qubits.len()
                )
            }

            Self::NamedResource { namespace, key } => {
                format!("resource:{namespace}:{key}")
            }
        }
    }
}

// ============================================================================
// Device calibration limits
// ============================================================================

/// Explicit resource policy for a `DeviceCalibration`.
///
/// These limits are **not** architectural limits.
///
/// `None` means that this module imposes no limit for that dimension.
///
/// This separation is fundamental to the Zamani scalability contract:
///
/// ```text
/// semantic capacity
///       !=
/// resource policy
/// ```
///
/// A production deployment processing untrusted calibration data should
/// configure finite limits appropriate to the available memory/CPU/storage.
///
/// A deployment with trusted input and sufficient resources can use
/// `DeviceCalibrationLimits::unlimited()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCalibrationLimits {
    /// Maximum device-reference UTF-8 byte length.
    pub max_device_reference_bytes: Option<usize>,

    /// Maximum provider-reference UTF-8 byte length.
    pub max_provider_reference_bytes: Option<usize>,

    /// Maximum number of calibration records.
    pub max_records: Option<usize>,

    /// Maximum number of distinct physical qubits referenced by one index.
    pub max_physical_qubits: Option<usize>,

    /// Maximum number of physical qubits in one `PhysicalQubits` scope.
    pub max_scope_physical_qubits: Option<usize>,

    /// Maximum number of physical qubits in one instruction scope.
    pub max_instruction_physical_qubits: Option<usize>,

    /// Maximum instruction-name UTF-8 byte length.
    pub max_instruction_name_bytes: Option<usize>,

    /// Maximum custom-resource namespace UTF-8 byte length.
    pub max_resource_namespace_bytes: Option<usize>,

    /// Maximum custom-resource key UTF-8 byte length.
    pub max_resource_key_bytes: Option<usize>,
}

impl DeviceCalibrationLimits {
    /// Creates a policy imposing no ZQN-specific finite resource limits.
    ///
    /// This does not make the host machine infinite. It only means this
    /// module will not introduce an artificial semantic ceiling.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_device_reference_bytes: None,
            max_provider_reference_bytes: None,
            max_records: None,
            max_physical_qubits: None,
            max_scope_physical_qubits: None,
            max_instruction_physical_qubits: None,
            max_instruction_name_bytes: None,
            max_resource_namespace_bytes: None,
            max_resource_key_bytes: None,
        }
    }

    /// Creates a conservative policy suitable as a starting point for
    /// untrusted data.
    ///
    /// These values are operational safety defaults, not machine-size
    /// assumptions.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_device_reference_bytes:
                Some(DEFAULT_DEVICE_REFERENCE_MAX_BYTES),

            max_provider_reference_bytes:
                Some(DEFAULT_PROVIDER_REFERENCE_MAX_BYTES),

            max_records: None,

            max_physical_qubits: None,

            max_scope_physical_qubits: None,

            max_instruction_physical_qubits: None,

            max_instruction_name_bytes:
                Some(DEFAULT_INSTRUCTION_NAME_MAX_BYTES),

            max_resource_namespace_bytes:
                Some(DEFAULT_RESOURCE_NAMESPACE_MAX_BYTES),

            max_resource_key_bytes:
                Some(DEFAULT_RESOURCE_KEY_MAX_BYTES),
        }
    }

    /// Returns the number of records allowed by the policy.
    #[must_use]
    pub fn allows_record_count(
        &self,
        count: usize,
    ) -> bool {
        match self.max_records {
            Some(maximum) => count <= maximum,
            None => true,
        }
    }

    /// Returns the number of physical qubits allowed by the policy.
    #[must_use]
    pub fn allows_physical_qubit_count(
        &self,
        count: usize,
    ) -> bool {
        match self.max_physical_qubits {
            Some(maximum) => count <= maximum,
            None => true,
        }
    }
}

impl Default for DeviceCalibrationLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// Calibration record
// ============================================================================

/// A binding between a calibration identity and a resource scope.
///
/// This is intentionally small.
///
/// It does not contain calibration values.
///
/// The corresponding calibration evidence is owned by the calibration
/// snapshot/state subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationBinding {
    /// ZQN calibration identity.
    calibration: CalibrationId,

    /// Resource scope covered by this calibration identity.
    scope: CalibrationScope,
}

impl CalibrationBinding {
    /// Creates a calibration binding.
    pub fn new(
        calibration: CalibrationId,
        scope: CalibrationScope,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<Self> {
        scope.validate(limits)?;

        Ok(Self {
            calibration,
            scope,
        })
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn calibration(&self) -> CalibrationId {
        self.calibration
    }

    /// Returns the resource scope.
    #[must_use]
    pub fn scope(&self) -> &CalibrationScope {
        &self.scope
    }

    /// Returns true when this binding explicitly references a physical qubit.
    #[must_use]
    pub fn references_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.scope.references_physical_qubit(qubit)
    }
}

// ============================================================================
// Device calibration index
// ============================================================================

/// Target-independent index of calibration identities by device resource.
///
/// # Important
///
/// `DeviceCalibration` does not represent the hardware device itself.
///
/// It is a calibration applicability index.
///
/// The actual hardware calibration state remains owned by the hardware
/// calibration subsystem.
///
/// # Example
///
/// ```text
/// DeviceCalibration
///     device_ref = "target-A"
///
///     Device
///         -> CalibrationId(1)
///
///     PhysicalQubit(PhysicalQubitId::new(7))
///         -> CalibrationId(42)
///
///     Instruction("cx", [q7, q9])
///         -> CalibrationId(81)
/// ```
///
/// This design allows the same ZQN abstraction to represent:
///
/// - small processors;
/// - large processors;
/// - distributed processors;
/// - simulators;
/// - emulators;
/// - logical devices;
/// - non-qubit resources.
///
/// No machine-size-specific branch is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCalibration {
    /// Stable external reference for the target/device.
    ///
    /// This is an identifier only. It is not a credential.
    device_reference: String,

    /// Optional provider/adapter reference.
    ///
    /// This is deliberately opaque to ZQN.
    provider_reference: Option<String>,

    /// Monotonic revision supplied by the owning integration layer.
    ///
    /// Zero is allowed and means that no revision policy was supplied.
    revision: u64,

    /// Calibration bindings indexed by exact resource scope.
    bindings: BTreeMap<CalibrationScope, CalibrationId>,
}

impl DeviceCalibration {
    /// Creates an empty device-calibration index.
    ///
    /// No hardware existence check occurs here.
    pub fn new(
        device_reference: impl Into<String>,
    ) -> ZqnResult<Self> {
        let limits = DeviceCalibrationLimits::conservative();

        Self::with_limits(
            device_reference,
            limits,
        )
    }

    /// Creates an empty device-calibration index using an explicit resource
    /// policy.
    pub fn with_limits(
        device_reference: impl Into<String>,
        limits: DeviceCalibrationLimits,
    ) -> ZqnResult<Self> {
        let device_reference = device_reference.into();

        validate_text(
            "device reference",
            &device_reference,
            limits.max_device_reference_bytes,
        )?;

        Ok(Self {
            device_reference,
            provider_reference: None,
            revision: 0,
            bindings: BTreeMap::new(),
        })
    }

    /// Returns the device reference.
    #[must_use]
    pub fn device_reference(&self) -> &str {
        &self.device_reference
    }

    /// Assigns an optional opaque provider reference.
    ///
    /// The provider reference is metadata only.
    ///
    /// It must not contain credentials.
    pub fn with_provider_reference(
        mut self,
        provider_reference: impl Into<String>,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<Self> {
        let provider_reference =
            provider_reference.into();

        validate_text(
            "provider reference",
            &provider_reference,
            limits.max_provider_reference_bytes,
        )?;

        self.provider_reference =
            Some(provider_reference);

        Ok(self)
    }

    /// Sets the calibration-index revision.
    ///
    /// Revision zero is valid.
    ///
    /// This method does not derive the revision from wall-clock time.
    #[must_use]
    pub const fn with_revision(
        mut self,
        revision: u64,
    ) -> Self {
        self.revision = revision;
        self
    }

    /// Returns the current calibration-index revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the optional provider reference.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }

    /// Returns the number of exact calibration bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns true when no calibration bindings exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Inserts a new calibration binding.
    ///
    /// Duplicate scopes are rejected rather than silently overwritten.
    ///
    /// Use [`Self::replace`] when replacement is intentional.
    pub fn insert(
        &mut self,
        scope: CalibrationScope,
        calibration: CalibrationId,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<()> {
        scope.validate(limits)?;

        let prospective_len =
            self.bindings
                .len()
                .checked_add(1)
                .ok_or_else(|| {
                    limit_exceeded(
                        "calibration record count overflow",
                    )
                })?;

        if !limits.allows_record_count(
            prospective_len,
        ) {
            return Err(limit_exceeded(format!(
                "adding calibration binding would create \
                 {prospective_len} records, exceeding the \
                 configured maximum"
            )));
        }

        if self.bindings.contains_key(&scope) {
            return Err(invalid_calibration(
                ZqnErrorCode::DuplicateIdentifier,
                format!(
                    "calibration scope `{}` is already bound",
                    scope.describe()
                ),
            ));
        }

        self.bindings.insert(
            scope,
            calibration,
        );

        Ok(())
    }

    /// Replaces the calibration identity for an existing scope.
    ///
    /// This method requires that the scope already exist.
    ///
    /// It therefore prevents accidental creation when a caller intended a
    /// replacement.
    pub fn replace(
        &mut self,
        scope: CalibrationScope,
        calibration: CalibrationId,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<CalibrationId> {
        scope.validate(limits)?;

        match self.bindings.get_mut(&scope) {
            Some(existing) => {
                let previous = *existing;
                *existing = calibration;
                Ok(previous)
            }

            None => Err(invalid_calibration(
                ZqnErrorCode::UnknownResource,
                format!(
                    "cannot replace missing calibration scope `{}`",
                    scope.describe()
                ),
            )),
        }
    }

    /// Inserts a binding or replaces an existing binding explicitly.
    pub fn insert_or_replace(
        &mut self,
        scope: CalibrationScope,
        calibration: CalibrationId,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<Option<CalibrationId>> {
        scope.validate(limits)?;

        if let Some(existing) =
            self.bindings.get_mut(&scope)
        {
            let previous = *existing;
            *existing = calibration;
            return Ok(Some(previous));
        }

        self.insert(
            scope,
            calibration,
            limits,
        )?;

        Ok(None)
    }

    /// Removes an exact calibration scope.
    ///
    /// No fallback resolution is performed.
    pub fn remove(
        &mut self,
        scope: &CalibrationScope,
    ) -> Option<CalibrationId> {
        self.bindings.remove(scope)
    }

    /// Looks up a calibration identity for an exact scope.
    ///
    /// This method intentionally does not perform implicit fallback.
    ///
    /// For example, a device-wide calibration does not automatically become
    /// the calibration for every physical qubit.
    ///
    /// Such fallback would be a semantic decision and belongs to the
    /// calibration-resolution policy layer.
    #[must_use]
    pub fn calibration_for(
        &self,
        scope: &CalibrationScope,
    ) -> Option<CalibrationId> {
        self.bindings.get(scope).copied()
    }

    /// Returns all bindings that explicitly reference a physical qubit.
    ///
    /// The returned iterator is deterministic.
    pub fn calibrations_for_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> impl Iterator<
        Item = (&CalibrationScope, &CalibrationId),
    > {
        self.bindings.iter().filter(
            move |(scope, _)| {
                scope.references_physical_qubit(qubit)
            },
        )
    }

    /// Returns all bindings.
    ///
    /// Iteration is deterministic because the underlying map is a
    /// `BTreeMap`.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&CalibrationScope, &CalibrationId),
    > {
        self.bindings.iter()
    }

    /// Returns all bindings as an exact iterator.
    ///
    /// This is equivalent to [`Self::iter`] and is provided as a semantic
    /// convenience for integration code.
    pub fn bindings(
        &self,
    ) -> impl Iterator<
        Item = (&CalibrationScope, &CalibrationId),
    > {
        self.bindings.iter()
    }

    /// Returns the calibration IDs associated with a physical qubit.
    ///
    /// The result is deterministic and contains no duplicate calibration IDs.
    pub fn calibration_ids_for_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> Vec<CalibrationId> {
        let mut result = BTreeSet::new();

        for (_, calibration) in
            self.calibrations_for_physical_qubit(qubit)
        {
            result.insert(*calibration);
        }

        result.into_iter().collect()
    }

    /// Returns all physical qubits explicitly referenced by this index.
    ///
    /// Device-wide and arbitrary named scopes do not imply any particular
    /// physical qubits.
    #[must_use]
    pub fn physical_qubits(
        &self,
    ) -> BTreeSet<PhysicalQubitId> {
        let mut result = BTreeSet::new();

        for scope in self.bindings.keys() {
            match scope {
                CalibrationScope::Device
                | CalibrationScope::NamedResource {
                    ..
                } => {}

                CalibrationScope::PhysicalQubit(
                    qubit,
                ) => {
                    result.insert(*qubit);
                }

                CalibrationScope::PhysicalQubits(
                    qubits,
                ) => {
                    result.extend(qubits.iter().copied());
                }

                CalibrationScope::Instruction {
                    qubits,
                    ..
                } => {
                    result.extend(qubits.iter().copied());
                }
            }
        }

        result
    }

    /// Validates the entire calibration index using an explicit policy.
    ///
    /// This validates the ZQN index itself. It does not prove that the
    /// referenced device or physical qubits exist.
    pub fn validate(
        &self,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<()> {
        validate_text(
            "device reference",
            &self.device_reference,
            limits.max_device_reference_bytes,
        )?;

        if let Some(provider) =
            &self.provider_reference
        {
            validate_text(
                "provider reference",
                provider,
                limits.max_provider_reference_bytes,
            )?;
        }

        if !limits.allows_record_count(
            self.bindings.len(),
        ) {
            return Err(limit_exceeded(format!(
                "device calibration contains {} \
                 records, exceeding the configured maximum",
                self.bindings.len()
            )));
        }

        for scope in self.bindings.keys() {
            scope.validate(limits)?;
        }

        let physical_qubit_count =
            self.physical_qubits().len();

        if !limits.allows_physical_qubit_count(
            physical_qubit_count,
        ) {
            return Err(limit_exceeded(format!(
                "device calibration references {} \
                 distinct physical qubits, exceeding \
                 the configured maximum",
                physical_qubit_count
            )));
        }

        Ok(())
    }

    /// Returns a new validated index containing bindings from both inputs.
    ///
    /// Conflicting scopes are rejected.
    ///
    /// The original indexes remain unchanged.
    pub fn merged(
        &self,
        other: &Self,
        limits: &DeviceCalibrationLimits,
    ) -> ZqnResult<Self> {
        if self.device_reference
            != other.device_reference
        {
            return Err(ZqnError::new(
                ZqnErrorKind::Calibration,
                ZqnErrorCode::CalibrationResourceMismatch,
                "cannot merge calibration indexes belonging to \
                 different device references",
            ));
        }

        if let (
            Some(left),
            Some(right),
        ) = (
            self.provider_reference.as_deref(),
            other.provider_reference.as_deref(),
        ) {
            if left != right {
                return Err(ZqnError::new(
                    ZqnErrorKind::Calibration,
                    ZqnErrorCode::CalibrationResourceMismatch,
                    "cannot merge calibration indexes with \
                     different provider references",
                ));
            }
        }

        let mut merged =
            Self::with_limits(
                self.device_reference.clone(),
                limits.clone(),
            )?;

        merged.provider_reference =
            self.provider_reference
                .clone()
                .or_else(|| {
                    other.provider_reference.clone()
                });

        merged.revision =
            self.revision.max(other.revision);

        for (
            scope,
            calibration,
        ) in &self.bindings
        {
            merged.insert(
                scope.clone(),
                *calibration,
                limits,
            )?;
        }

        for (
            scope,
            calibration,
        ) in &other.bindings
        {
            if let Some(existing) =
                merged.bindings.get(scope)
            {
                if existing != calibration {
                    return Err(
                        invalid_calibration(
                            ZqnErrorCode::DuplicateIdentifier,
                            format!(
                                "conflicting calibration identities \
                                 for scope `{}`",
                                scope.describe()
                            ),
                        ),
                    );
                }

                continue;
            }

            merged.insert(
                scope.clone(),
                *calibration,
                limits,
            )?;
        }

        merged.validate(limits)?;

        Ok(merged)
    }

    /// Returns an immutable map view for internal ZQN integration.
    ///
    /// The returned map cannot mutate the index.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> &BTreeMap<CalibrationScope, CalibrationId> {
        &self.bindings
    }
}

impl<'a>
    IntoIterator
    for &'a DeviceCalibration
{
    type Item =
        (
            &'a CalibrationScope,
            &'a CalibrationId,
        );

    type IntoIter =
        btree_map::Iter<
            'a,
            CalibrationScope,
            CalibrationId,
        >;

    fn into_iter(self) -> Self::IntoIter {
        self.bindings.iter()
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Validates a textual identifier under an optional byte-size policy.
fn validate_text(
    field: &'static str,
    value: &str,
    maximum: Option<usize>,
) -> ZqnResult<()> {
    if value.is_empty() {
        return Err(invalid_calibration(
            ZqnErrorCode::InvalidIdentifier,
            format!(
                "{field} cannot be empty"
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_calibration(
            ZqnErrorCode::InvalidIdentifier,
            format!(
                "{field} contains control characters"
            ),
        ));
    }

    if let Some(maximum) = maximum {
        if value.len() > maximum {
            return Err(limit_exceeded(format!(
                "{field} is {} UTF-8 bytes, \
                 exceeding the configured maximum \
                 of {maximum}",
                value.len()
            )));
        }
    }

    Ok(())
}

/// Constructs a canonical ZQN calibration error.
fn invalid_calibration(
    code: ZqnErrorCode,
    message: impl Into<String>,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Calibration,
        code,
        message.into(),
    )
}

/// Constructs a canonical ZQN resource-limit error.
fn limit_exceeded(
    message: impl Into<String>,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Limits,
        ZqnErrorCode::LimitExceeded,
        message.into(),
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> DeviceCalibrationLimits {
        DeviceCalibrationLimits {
            max_device_reference_bytes: Some(128),
            max_provider_reference_bytes: Some(128),
            max_records: Some(16),
            max_physical_qubits: Some(32),
            max_scope_physical_qubits: Some(8),
            max_instruction_physical_qubits: Some(8),
            max_instruction_name_bytes: Some(128),
            max_resource_namespace_bytes: Some(128),
            max_resource_key_bytes: Some(128),
        }
    }

    fn calibration_id(
        value: u64,
    ) -> CalibrationId {
        CalibrationId::new(value)
    }

    #[test]
    fn creates_empty_device_calibration() {
        let device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("valid device reference");

        assert_eq!(
            device.device_reference(),
            "target-A"
        );

        assert_eq!(device.len(), 0);
        assert!(device.is_empty());
    }

    #[test]
    fn rejects_empty_device_reference() {
        let result =
            DeviceCalibration::with_limits(
                "",
                limits(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_non_contiguous_physical_qubit_ids() {
        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("device");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(17),
                ),
                calibration_id(1),
                &limits(),
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(3),
                ),
                calibration_id(2),
                &limits(),
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(42),
                ),
                calibration_id(3),
                &limits(),
            )
            .expect("insert");

        assert_eq!(
            device.physical_qubits().len(),
            3
        );
    }

    #[test]
    fn uses_canonical_physical_qubit_identity() {
        let qubit =
            PhysicalQubitId::new(11);

        let scope =
            CalibrationScope::physical_qubit(
                qubit,
            );

        assert!(scope.references_physical_qubit(
            qubit
        ));
    }

    #[test]
    fn rejects_duplicate_scope_insertion() {
        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("device");

        let scope =
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            );

        device
            .insert(
                scope.clone(),
                calibration_id(1),
                &limits(),
            )
            .expect("first insert");

        let result = device.insert(
            scope,
            calibration_id(2),
            &limits(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn replacement_is_explicit() {
        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("device");

        let scope =
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            );

        device
            .insert(
                scope.clone(),
                calibration_id(1),
                &limits(),
            )
            .expect("insert");

        let previous = device
            .replace(
                scope.clone(),
                calibration_id(2),
                &limits(),
            )
            .expect("replace");

        assert_eq!(
            previous,
            calibration_id(1)
        );

        assert_eq!(
            device.calibration_for(&scope),
            Some(calibration_id(2))
        );
    }

    #[test]
    fn instruction_scope_rejects_duplicate_qubits() {
        let scope =
            CalibrationScope::instruction(
                "cx",
                [
                    PhysicalQubitId::new(1),
                    PhysicalQubitId::new(1),
                ],
            );

        let result =
            scope.validate(&limits());

        assert!(result.is_err());
    }

    #[test]
    fn physical_qubit_set_is_canonicalized() {
        let scope =
            CalibrationScope::physical_qubits(
                [
                    PhysicalQubitId::new(42),
                    PhysicalQubitId::new(3),
                    PhysicalQubitId::new(42),
                    PhysicalQubitId::new(17),
                ],
            );

        assert_eq!(
            scope.physical_qubit_count(),
            3
        );
    }

    #[test]
    fn exact_lookup_does_not_apply_device_fallback() {
        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("device");

        device
            .insert(
                CalibrationScope::Device,
                calibration_id(1),
                &limits(),
            )
            .expect("insert");

        let qubit_scope =
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(7),
            );

        assert_eq!(
            device.calibration_for(
                &qubit_scope
            ),
            None
        );
    }

    #[test]
    fn finds_all_calibrations_for_a_qubit() {
        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                limits(),
            )
            .expect("device");

        let q7 =
            PhysicalQubitId::new(7);
        let q9 =
            PhysicalQubitId::new(9);

        device
            .insert(
                CalibrationScope::physical_qubit(
                    q7,
                ),
                calibration_id(1),
                &limits(),
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::physical_qubits(
                    [q7, q9],
                ),
                calibration_id(2),
                &limits(),
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::instruction(
                    "cx",
                    [q7, q9],
                ),
                calibration_id(3),
                &limits(),
            )
            .expect("insert");

        let ids =
            device.calibration_ids_for_physical_qubit(
                q7,
            );

        assert_eq!(
            ids,
            vec![
                calibration_id(1),
                calibration_id(2),
                calibration_id(3),
            ]
        );
    }

    #[test]
    fn named_resources_support_non_qubit_technologies() {
        let scope =
            CalibrationScope::named_resource(
                "bosonic.mode",
                "mode-7",
            );

        scope
            .validate(&limits())
            .expect("valid resource");

        assert_eq!(
            scope.physical_qubit_count(),
            0
        );
    }

    #[test]
    fn merge_accepts_identical_bindings() {
        let policy = limits();

        let mut left =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        left.insert(
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            calibration_id(1),
            &policy,
        )
        .expect("insert");

        let mut right =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        right.insert(
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            calibration_id(1),
            &policy,
        )
        .expect("insert");

        let merged =
            left.merged(
                &right,
                &policy,
            )
            .expect("merge");

        assert_eq!(
            merged.len(),
            1
        );
    }

    #[test]
    fn merge_rejects_conflicting_bindings() {
        let policy = limits();

        let mut left =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        left.insert(
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            calibration_id(1),
            &policy,
        )
        .expect("insert");

        let mut right =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        right.insert(
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            calibration_id(2),
            &policy,
        )
        .expect("insert");

        let result =
            left.merged(
                &right,
                &policy,
            );

        assert!(result.is_err());
    }

    #[test]
    fn resource_limits_are_policy_not_semantic_limits() {
        let policy =
            DeviceCalibrationLimits {
                max_device_reference_bytes:
                    Some(128),
                max_provider_reference_bytes:
                    Some(128),
                max_records: Some(1),
                max_physical_qubits: None,
                max_scope_physical_qubits:
                    None,
                max_instruction_physical_qubits:
                    None,
                max_instruction_name_bytes:
                    Some(128),
                max_resource_namespace_bytes:
                    Some(128),
                max_resource_key_bytes:
                    Some(128),
            };

        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(
                        usize::MAX,
                    ),
                ),
                calibration_id(1),
                &policy,
            )
            .expect(
                "the ID itself is not a semantic \
                 machine-size limit",
            );
    }

    #[test]
    fn iteration_is_deterministic() {
        let policy = limits();

        let mut device =
            DeviceCalibration::with_limits(
                "target-A",
                policy.clone(),
            )
            .expect("device");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(42),
                ),
                calibration_id(42),
                &policy,
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(3),
                ),
                calibration_id(3),
                &policy,
            )
            .expect("insert");

        device
            .insert(
                CalibrationScope::physical_qubit(
                    PhysicalQubitId::new(17),
                ),
                calibration_id(17),
                &policy,
            )
            .expect("insert");

        let first: Vec<_> =
            device
                .iter()
                .map(|(scope, calibration)| {
                    (
                        scope.clone(),
                        *calibration,
                    )
                })
                .collect();

        let second: Vec<_> =
            device
                .iter()
                .map(|(scope, calibration)| {
                    (
                        scope.clone(),
                        *calibration,
                    )
                })
                .collect();

        assert_eq!(
            first,
            second
        );
    }
}