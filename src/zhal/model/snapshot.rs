
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Observation Snapshot Model.
//!
//! Path: src/zhal/model/snapshot.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines immutable, versioned and reproducible snapshots of
//! observed physical hardware state.
//!
//! A snapshot represents a consistent, explicitly qualified
//! observation of hardware information at a particular observation
//! boundary.
//!
//! It can reference:
//!
//! - Hardware inventories.
//! - Physical devices.
//! - Physical resources.
//! - Hardware capabilities.
//! - Physical hardware topologies.
//! - Other observation snapshots.
//! - Provider observation records.
//!
//! The model supports:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - Accelerators.
//! - AI hardware.
//! - Embedded computing.
//! - Hybrid computing.
//! - Distributed hardware.
//! - Photonic computing.
//! - Neuromorphic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! New hardware technologies are represented through extensible
//! references rather than modifications to this module.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - HardwareSnapshot.
//! - SnapshotPayload.
//! - SnapshotRevision.
//! - SnapshotTimestamp.
//! - SnapshotConsistency.
//! - SnapshotProvenance.
//! - SnapshotReference.
//! - SnapshotReferenceKind.
//! - SnapshotFingerprint.
//! - SnapshotBuilder.
//! - Snapshot validation.
//! - Snapshot serialization.
//! - Snapshot fingerprint calculation.
//! - Snapshot integrity verification.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - Hardware discovery.
//! - Provider registration.
//! - Provider lifecycle.
//! - Hardware inventory management.
//! - Device communication.
//! - Device sessions.
//! - Resource allocation.
//! - Resource reservations.
//! - Resource leases.
//! - Resource accounting.
//! - Capability semantics.
//! - Capability feasibility.
//! - Hardware scheduling.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Runtime orchestration.
//! - Authentication.
//! - Authorization.
//! - Hardware execution.
//!
//! # Canonical repository authorities
//!
//! Hardware snapshot identity:
//!     crate::zhal::identity::HardwareSnapshotId
//!
//! Inventory identity:
//!     crate::zhal::identity::HardwareInventoryId
//!
//! Hardware device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Hardware resource identity:
//!     crate::zhal::identity::HardwareResourceIdentity
//!
//! Hardware topology identity:
//!     crate::zhal::identity::HardwareTopologyId
//!
//! Device descriptions:
//!     crate::zhal::model::device
//!
//! Resource descriptions and observations:
//!     crate::zhal::model::resource
//!
//! Physical topology:
//!     crate::zhal::model::topology
//!
//! Hardware metadata:
//!     crate::zhal::model::metadata
//!
//! Capability descriptions and snapshots:
//!     crate::stdlib::capability
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Runtime allocation and orchestration:
//!     crate::zur
//!
//! # Fundamental invariants
//!
//! Observation != availability guarantee.
//!
//! Availability != reservation.
//!
//! Reservation != allocation.
//!
//! Allocation != execution.
//!
//! Snapshot != live hardware state.
//!
//! Snapshot identity != snapshot fingerprint.
//!
//! Snapshot revision != schema version.
//!
//! Snapshot consistency != physical correctness.
//!
//! A fingerprint establishes content identity, not authenticity.
//!
//! # POCO-REAF
//!
//! Snapshots describe observed hardware without making hardware
//! identities part of the Zamani source-language contract.
//!
//! The same portable program can therefore consume different
//! compatible hardware realizations without changing its source.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Snapshot count.
//! - Device references.
//! - Resource references.
//! - Capability references.
//! - Related snapshots.
//! - Topology references.
//! - Provider-defined references.
//! - Identifier lengths.
//! - Revision lengths.
//! - Timestamp descriptions.
//!
//! Collections grow according to available resources and
//! representation constraints.
//!
//! Literal infinite memory or computation is not promised.
//!
//! # Determinism
//!
//! All collection fields use BTreeSet.
//!
//! Their iteration order is deterministic.
//!
//! Fingerprints are calculated from the canonical serialized
//! snapshot payload, excluding the fingerprint itself.
//!
//! # Security
//!
//! Snapshot references and provenance are descriptive.
//!
//! They are not credentials or authorization evidence.
//!
//! Never store secrets in snapshot references or provenance.
//!
//! # Initialization
//!
//! Constructing a snapshot does not access physical hardware.
//!
//! No global state, hardware discovery or resource allocation
//! occurs in this module.

use core::fmt;

use std::collections::BTreeSet;

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use sha2::{Digest, Sha256};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInventoryId,
    HardwareResourceIdentity,
    HardwareSnapshotId,
    HardwareTopologyId,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable module identity.
pub const COMPONENT: &str = "zhal.model.snapshot";

/// Snapshot schema identifier.
pub const SNAPSHOT_SCHEMA_ID: &str =
    "org.zamani.zhal.hardware-snapshot";

/// Current snapshot schema version.
///
/// This is independent of:
///
/// - Zamani language version.
/// - Cargo package version.
/// - ZHAL API version.
/// - Provider API version.
/// - Hardware firmware version.
/// - Physical hardware revision.
pub const SNAPSHOT_SCHEMA_VERSION: u16 = 1;

/// Fingerprint algorithm identifier.
pub const SNAPSHOT_FINGERPRINT_ALGORITHM: &str =
    "sha256";

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of snapshot validation errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SnapshotErrorKind {
    /// A required field is empty.
    EmptyField,

    /// A field contains invalid whitespace.
    InvalidWhitespace,

    /// A field contains control characters.
    ControlCharacter,

    /// A snapshot revision is invalid.
    InvalidRevision,

    /// An observation timestamp is invalid.
    InvalidTimestamp,

    /// A clock-domain description is invalid.
    InvalidClockDomain,

    /// A provenance description is invalid.
    InvalidProvenance,

    /// A snapshot reference is invalid.
    InvalidReference,

    /// A duplicate reference was supplied.
    DuplicateReference,

    /// A snapshot references itself.
    SelfReference,

    /// The serialized schema is unsupported.
    UnsupportedSchema,

    /// The fingerprint algorithm is unsupported.
    UnsupportedFingerprintAlgorithm,

    /// The serialized fingerprint is invalid.
    InvalidFingerprint,

    /// The fingerprint does not match the payload.
    FingerprintMismatch,

    /// The snapshot is internally inconsistent.
    InconsistentSnapshot,

    /// Canonical serialization failed.
    SerializationFailure,
}

/// Structured snapshot error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotError {
    kind: SnapshotErrorKind,
    field: &'static str,
    message: String,
}

impl SnapshotError {
    fn new(
        kind: SnapshotErrorKind,
        field: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field,
            message: message.into(),
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> SnapshotErrorKind {
        self.kind
    }

    /// Returns the affected field.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SnapshotError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}: {}",
            COMPONENT,
            self.field,
            self.message
        )
    }
}

impl std::error::Error for SnapshotError {}

/// Result type for snapshot operations.
pub type SnapshotResult<T> = Result<T, SnapshotError>;

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_description(
    value: &str,
    field: &'static str,
) -> SnapshotResult<()> {
    if value.trim().is_empty() {
        return Err(SnapshotError::new(
            SnapshotErrorKind::EmptyField,
            field,
            "value must not be empty or whitespace-only",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(SnapshotError::new(
            SnapshotErrorKind::ControlCharacter,
            field,
            "control characters are not permitted",
        ));
    }

    Ok(())
}

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> SnapshotResult<()> {
    validate_description(value, field)?;

    if value.trim() != value {
        return Err(SnapshotError::new(
            SnapshotErrorKind::InvalidWhitespace,
            field,
            "surrounding whitespace is not permitted",
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(SnapshotError::new(
            SnapshotErrorKind::InvalidWhitespace,
            field,
            "whitespace is not permitted in an identifier",
        ));
    }

    Ok(())
}

// ============================================================================
// SNAPSHOT REVISION
// ============================================================================

/// Opaque revision of a hardware observation snapshot.
///
/// A revision identifies an observation generation.
///
/// It is independent of the snapshot schema version.
///
/// Revision values are provider- or inventory-owner-defined.
///
/// They are not forced into u64, u128 or another machine-sized
/// integer representation.
///
/// This avoids introducing arbitrary revision-number ceilings.
///
/// Revisions are opaque identities, not automatically ordered
/// timestamps.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct SnapshotRevision(String);

impl SnapshotRevision {
    /// Constructs a validated snapshot revision.
    ///
    /// # Errors
    ///
    /// Returns an error if the revision is empty or invalid.
    pub fn new(
        value: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "revision",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact revision identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SnapshotRevision {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SnapshotRevision {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(D::Error::custom)
    }
}

// ============================================================================
// OBSERVATION TIMESTAMP
// ============================================================================

/// Provider-supplied observation timestamp.
///
/// The timestamp is retained as an opaque description because
/// providers may use different clock and timestamp authorities.
///
/// Examples:
///
/// - An RFC 3339 UTC timestamp.
/// - A provider-defined timestamp.
/// - A hardware-clock observation.
/// - A remote observation timestamp.
///
/// Consumers requiring timestamp arithmetic must explicitly
/// interpret the timestamp using the appropriate time authority.
///
/// The clock-domain identity must be preserved.
///
/// No assumption is made that timestamps from different clocks
/// are directly comparable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SnapshotTimestamp {
    value: String,
    clock_domain: String,
}

impl SnapshotTimestamp {
    /// Creates a validated observation timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error for empty or invalid fields.
    pub fn new(
        value: impl Into<String>,
        clock_domain: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let value = value.into();
        let clock_domain = clock_domain.into();

        validate_description(
            &value,
            "timestamp.value",
        )?;

        validate_identifier(
            &clock_domain,
            "timestamp.clock_domain",
        )?;

        Ok(Self {
            value,
            clock_domain,
        })
    }

    /// Returns the provider-supplied timestamp.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the timestamp clock-domain identity.
    #[must_use]
    pub fn clock_domain(&self) -> &str {
        &self.clock_domain
    }
}

impl<'de> Deserialize<'de> for SnapshotTimestamp {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            value: String,
            clock_domain: String,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.value,
            wire.clock_domain,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// OBSERVATION CONSISTENCY
// ============================================================================

/// Describes the consistency and completeness of an observation.
///
/// These values qualify the information collected by a provider.
/// They do not establish physical correctness.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotConsistency {
    /// The provider reports that the declared observation scope
    /// was completely collected under its consistency contract.
    Complete,

    /// The observation contains only part of the requested scope.
    Partial,

    /// The observation uses best-effort collection semantics.
    BestEffort,

    /// The consistency of the observation has not been established.
    #[default]
    Unknown,
}

// ============================================================================
// PROVENANCE
// ============================================================================

/// Describes the origin of a hardware snapshot.
///
/// Provenance records where an observation came from.
///
/// It does not authenticate the provider or establish that
/// the reported information is correct.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SnapshotProvenance {
    source: String,
    provider_reference: Option<String>,
    evidence_reference: Option<String>,
    collection_method: Option<String>,
}

impl SnapshotProvenance {
    /// Creates snapshot provenance.
    ///
    /// `source` identifies the observation source.
    ///
    /// Optional references describe external provider and evidence
    /// records without taking ownership of those records.
    ///
    /// # Errors
    ///
    /// Returns an error when a supplied reference is invalid.
    pub fn new(
        source: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let source = source.into();

        validate_identifier(
            &source,
            "provenance.source",
        )?;

        Ok(Self {
            source,
            provider_reference: None,
            evidence_reference: None,
            collection_method: None,
        })
    }

    /// Associates a provider reference.
    pub fn with_provider_reference(
        mut self,
        reference: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let reference = reference.into();

        validate_identifier(
            &reference,
            "provenance.provider_reference",
        )?;

        self.provider_reference = Some(reference);

        Ok(self)
    }

    /// Associates an external evidence reference.
    pub fn with_evidence_reference(
        mut self,
        reference: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let reference = reference.into();

        validate_identifier(
            &reference,
            "provenance.evidence_reference",
        )?;

        self.evidence_reference = Some(reference);

        Ok(self)
    }

    /// Records the collection method.
    pub fn with_collection_method(
        mut self,
        method: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let method = method.into();

        validate_description(
            &method,
            "provenance.collection_method",
        )?;

        self.collection_method = Some(method);

        Ok(self)
    }

    /// Returns the observation source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the optional provider reference.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }

    /// Returns the optional evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Returns the collection method.
    #[must_use]
    pub fn collection_method(&self) -> Option<&str> {
        self.collection_method.as_deref()
    }
}

impl<'de> Deserialize<'de> for SnapshotProvenance {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            source: String,
            provider_reference: Option<String>,
            evidence_reference: Option<String>,
            collection_method: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let mut value = Self::new(wire.source)
            .map_err(D::Error::custom)?;

        if let Some(reference) = wire.provider_reference {
            value = value
                .with_provider_reference(reference)
                .map_err(D::Error::custom)?;
        }

        if let Some(reference) = wire.evidence_reference {
            value = value
                .with_evidence_reference(reference)
                .map_err(D::Error::custom)?;
        }

        if let Some(method) = wire.collection_method {
            value = value
                .with_collection_method(method)
                .map_err(D::Error::custom)?;
        }

        Ok(value)
    }
}

// ============================================================================
// EXTENSIBLE SNAPSHOT REFERENCES
// ============================================================================

/// Open-world classification of an external snapshot reference.
///
/// These identifiers are extensible strings rather than a closed
/// enumeration of hardware technologies.
///
/// This permits future hardware and observation systems to be
/// referenced without changing the snapshot model.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct SnapshotReferenceKind(String);

impl SnapshotReferenceKind {
    /// Creates a validated reference-kind identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> SnapshotResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "reference.kind",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact reference-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SnapshotReferenceKind {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(D::Error::custom)
    }
}

/// Standard reference-kind identifiers.
///
/// These are conveniences, not a closed registry.
pub mod reference_kind {
    /// Hardware inventory reference.
    pub const INVENTORY: &str = "hardware.inventory";

    /// Hardware device reference.
    pub const DEVICE: &str = "hardware.device";

    /// Hardware resource observation reference.
    pub const RESOURCE: &str = "hardware.resource";

    /// Capability snapshot reference.
    pub const CAPABILITY: &str = "hardware.capability";

    /// Physical topology reference.
    pub const TOPOLOGY: &str = "hardware.topology";

    /// Provider observation reference.
    pub const PROVIDER: &str = "hardware.provider";

    /// External hardware observation reference.
    pub const OBSERVATION: &str = "hardware.observation";
}

/// Reference to an immutable external observation.
///
/// References identify information owned by another subsystem.
///
/// They do not embed the referenced object or establish that it
/// is still available.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct SnapshotReference {
    kind: SnapshotReferenceKind,
    identity: String,
    revision: Option<String>,
}

impl SnapshotReference {
    /// Constructs an external snapshot reference.
    pub fn new(
        kind: SnapshotReferenceKind,
        identity: impl Into<String>,
        revision: Option<String>,
    ) -> SnapshotResult<Self> {
        let identity = identity.into();

        validate_identifier(
            &identity,
            "reference.identity",
        )?;

        if let Some(revision) = &revision {
            validate_identifier(
                revision,
                "reference.revision",
            )?;
        }

        Ok(Self {
            kind,
            identity,
            revision,
        })
    }

    /// Returns the reference classification.
    #[must_use]
    pub fn kind(&self) -> &SnapshotReferenceKind {
        &self.kind
    }

    /// Returns the external identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the optional external revision.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
}

impl<'de> Deserialize<'de> for SnapshotReference {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: SnapshotReferenceKind,
            identity: String,
            revision: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.kind,
            wire.identity,
            wire.revision,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// CAPABILITY REFERENCES
// ============================================================================

/// Reference to a capability observation.
///
/// Capability identity remains owned by:
///
///     crate::stdlib::capability::CapabilityId
///
/// The snapshot retains its portable textual identity and references
/// the canonical external capability observation.
///
/// It does not create another capability semantic model.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct SnapshotCapabilityReference {
    identity: String,
    observation_reference: Option<String>,
}

impl SnapshotCapabilityReference {
    /// Constructs a capability reference.
    pub fn new(
        identity: impl Into<String>,
        observation_reference: Option<String>,
    ) -> SnapshotResult<Self> {
        let identity = identity.into();

        validate_identifier(
            &identity,
            "capability.identity",
        )?;

        if let Some(reference) = &observation_reference {
            validate_identifier(
                reference,
                "capability.observation_reference",
            )?;
        }

        Ok(Self {
            identity,
            observation_reference,
        })
    }

    /// Returns the canonical textual capability identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the external observation reference.
    #[must_use]
    pub fn observation_reference(&self) -> Option<&str> {
        self.observation_reference.as_deref()
    }
}

impl<'de> Deserialize<'de> for SnapshotCapabilityReference {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            identity: String,
            observation_reference: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.identity,
            wire.observation_reference,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// CANONICAL SNAPSHOT PAYLOAD
// ============================================================================

/// Immutable snapshot payload.
///
/// All collections use deterministic ordering.
///
/// The payload contains references rather than competing copies
/// of the device, resource, capability and topology authorities.
///
/// This separation prevents snapshots from becoming duplicate
/// hardware-model registries.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPayload {
    schema_id: String,
    schema_version: u16,

    id: HardwareSnapshotId,
    inventory_id: HardwareInventoryId,
    revision: SnapshotRevision,

    observed_at: SnapshotTimestamp,

    consistency: SnapshotConsistency,

    provenance: SnapshotProvenance,

    devices: BTreeSet<HardwareDeviceIdentity>,

    resources: BTreeSet<HardwareResourceIdentity>,

    capabilities: BTreeSet<SnapshotCapabilityReference>,

    topology: Option<HardwareTopologyId>,

    related_snapshots: BTreeSet<HardwareSnapshotId>,

    references: BTreeSet<SnapshotReference>,
}

impl SnapshotPayload {
    /// Returns the schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the snapshot identity.
    #[must_use]
    pub fn id(&self) -> &HardwareSnapshotId {
        &self.id
    }

    /// Returns the owning inventory identity.
    #[must_use]
    pub fn inventory_id(&self) -> &HardwareInventoryId {
        &self.inventory_id
    }

    /// Returns the observation revision.
    #[must_use]
    pub fn revision(&self) -> &SnapshotRevision {
        &self.revision
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub fn observed_at(&self) -> &SnapshotTimestamp {
        &self.observed_at
    }

    /// Returns the consistency classification.
    #[must_use]
    pub const fn consistency(&self) -> SnapshotConsistency {
        self.consistency
    }

    /// Returns the snapshot provenance.
    #[must_use]
    pub fn provenance(&self) -> &SnapshotProvenance {
        &self.provenance
    }

    /// Returns the observed device identities.
    pub fn devices(
        &self,
    ) -> impl Iterator<Item = &HardwareDeviceIdentity> {
        self.devices.iter()
    }

    /// Returns the observed resource identities.
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = &HardwareResourceIdentity> {
        self.resources.iter()
    }

    /// Returns the referenced capability observations.
    pub fn capabilities(
        &self,
    ) -> impl Iterator<Item = &SnapshotCapabilityReference> {
        self.capabilities.iter()
    }

    /// Returns the physical topology identity.
    #[must_use]
    pub fn topology(&self) -> Option<&HardwareTopologyId> {
        self.topology.as_ref()
    }

    /// Returns related observation snapshot identities.
    pub fn related_snapshots(
        &self,
    ) -> impl Iterator<Item = &HardwareSnapshotId> {
        self.related_snapshots.iter()
    }

    /// Returns external observation references.
    pub fn references(
        &self,
    ) -> impl Iterator<Item = &SnapshotReference> {
        self.references.iter()
    }

    /// Returns whether the snapshot contains device references.
    #[must_use]
    pub fn has_devices(&self) -> bool {
        !self.devices.is_empty()
    }

    /// Returns whether the snapshot contains resource references.
    #[must_use]
    pub fn has_resources(&self) -> bool {
        !self.resources.is_empty()
    }

    /// Returns whether the snapshot contains capability references.
    #[must_use]
    pub fn has_capabilities(&self) -> bool {
        !self.capabilities.is_empty()
    }

    /// Returns the number of device references.
    #[must_use]
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Returns the number of resource references.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of capability references.
    #[must_use]
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns the number of external references.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.references.len()
    }

    /// Validates the snapshot payload.
    pub fn validate(&self) -> SnapshotResult<()> {
        if self.schema_id != SNAPSHOT_SCHEMA_ID {
            return Err(SnapshotError::new(
                SnapshotErrorKind::UnsupportedSchema,
                "schema_id",
                "unsupported snapshot schema identifier",
            ));
        }

        if self.schema_version != SNAPSHOT_SCHEMA_VERSION {
            return Err(SnapshotError::new(
                SnapshotErrorKind::UnsupportedSchema,
                "schema_version",
                "unsupported snapshot schema version",
            ));
        }

        if self.related_snapshots.contains(&self.id) {
            return Err(SnapshotError::new(
                SnapshotErrorKind::SelfReference,
                "related_snapshots",
                "a snapshot cannot reference itself",
            ));
        }

        validate_identifier(
            self.revision.as_str(),
            "revision",
        )?;

        validate_description(
            self.observed_at.value(),
            "timestamp.value",
        )?;

        validate_identifier(
            self.observed_at.clock_domain(),
            "timestamp.clock_domain",
        )?;

        validate_identifier(
            self.provenance.source(),
            "provenance.source",
        )?;

        Ok(())
    }
}

// ============================================================================
// SNAPSHOT FINGERPRINT
// ============================================================================

/// Deterministic SHA-256 fingerprint of a snapshot payload.
///
/// A fingerprint identifies serialized content.
///
/// It does not establish authenticity, authorization, provider trust
/// or physical correctness.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SnapshotFingerprint {
    algorithm: String,
    digest: String,
}

impl SnapshotFingerprint {
    fn calculate(
        payload: &SnapshotPayload,
    ) -> SnapshotResult<Self> {
        let encoded = serde_json::to_vec(payload)
            .map_err(|error| {
                SnapshotError::new(
                    SnapshotErrorKind::SerializationFailure,
                    "fingerprint",
                    error.to_string(),
                )
            })?;

        let digest = Sha256::digest(&encoded);

        Ok(Self {
            algorithm:
                SNAPSHOT_FINGERPRINT_ALGORITHM.to_owned(),
            digest: format!("{digest:x}"),
        })
    }

    /// Returns the fingerprint algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Returns the lowercase hexadecimal digest.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }

    /// Returns the canonical fingerprint representation.
    #[must_use]
    pub fn as_str(&self) -> String {
        format!("{}:{}", self.algorithm, self.digest)
    }

    /// Validates the fingerprint representation.
    pub fn validate(&self) -> SnapshotResult<()> {
        if self.algorithm != SNAPSHOT_FINGERPRINT_ALGORITHM {
            return Err(SnapshotError::new(
                SnapshotErrorKind::UnsupportedFingerprintAlgorithm,
                "fingerprint.algorithm",
                "unsupported fingerprint algorithm",
            ));
        }

        if self.digest.len() != 64
            || !self.digest.bytes().all(|byte| {
                byte.is_ascii_digit()
                    || (b'a'..=b'f').contains(&byte)
            })
        {
            return Err(SnapshotError::new(
                SnapshotErrorKind::InvalidFingerprint,
                "fingerprint.digest",
                "invalid SHA-256 hexadecimal digest",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// IMMUTABLE HARDWARE SNAPSHOT
// ============================================================================

/// Immutable, versioned hardware observation snapshot.
///
/// A snapshot represents observed hardware information and its
/// associated provenance at a defined observation boundary.
///
/// Once constructed, its payload cannot be modified through this API.
///
/// Updating physical observations requires creating a new snapshot.
///
/// A snapshot does not reserve, allocate, lock or execute hardware.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareSnapshot {
    payload: SnapshotPayload,
    fingerprint: SnapshotFingerprint,
}

impl HardwareSnapshot {
    /// Returns a new snapshot builder.
    #[must_use]
    pub fn builder(
        id: HardwareSnapshotId,
        inventory_id: HardwareInventoryId,
        revision: SnapshotRevision,
        observed_at: SnapshotTimestamp,
        provenance: SnapshotProvenance,
    ) -> SnapshotBuilder {
        SnapshotBuilder::new(
            id,
            inventory_id,
            revision,
            observed_at,
            provenance,
        )
    }

    /// Returns the immutable snapshot payload.
    #[must_use]
    pub fn payload(&self) -> &SnapshotPayload {
        &self.payload
    }

    /// Returns the snapshot identity.
    #[must_use]
    pub fn id(&self) -> &HardwareSnapshotId {
        self.payload.id()
    }

    /// Returns the inventory identity.
    #[must_use]
    pub fn inventory_id(&self) -> &HardwareInventoryId {
        self.payload.inventory_id()
    }

    /// Returns the snapshot revision.
    #[must_use]
    pub fn revision(&self) -> &SnapshotRevision {
        self.payload.revision()
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub fn observed_at(&self) -> &SnapshotTimestamp {
        self.payload.observed_at()
    }

    /// Returns the observation consistency.
    #[must_use]
    pub fn consistency(&self) -> SnapshotConsistency {
        self.payload.consistency()
    }

    /// Returns the observation provenance.
    #[must_use]
    pub fn provenance(&self) -> &SnapshotProvenance {
        self.payload.provenance()
    }

    /// Returns the deterministic content fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &SnapshotFingerprint {
        &self.fingerprint
    }

    /// Recalculates and verifies the snapshot fingerprint.
    ///
    /// This is useful after deserialization or when validating
    /// externally persisted snapshot records.
    pub fn verify_integrity(&self) -> SnapshotResult<bool> {
        self.payload.validate()?;
        self.fingerprint.validate()?;

        let calculated =
            SnapshotFingerprint::calculate(&self.payload)?;

        Ok(calculated == self.fingerprint)
    }

    /// Returns the device references.
    pub fn devices(
        &self,
    ) -> impl Iterator<Item = &HardwareDeviceIdentity> {
        self.payload.devices()
    }

    /// Returns the resource references.
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = &HardwareResourceIdentity> {
        self.payload.resources()
    }

    /// Returns the capability references.
    pub fn capabilities(
        &self,
    ) -> impl Iterator<Item = &SnapshotCapabilityReference> {
        self.payload.capabilities()
    }

    /// Returns the physical topology reference.
    #[must_use]
    pub fn topology(&self) -> Option<&HardwareTopologyId> {
        self.payload.topology()
    }

    /// Returns whether the snapshot explicitly declares complete
    /// observation consistency.
    ///
    /// This does not independently prove physical completeness.
    #[must_use]
    pub fn is_declared_complete(&self) -> bool {
        self.consistency() == SnapshotConsistency::Complete
    }

    /// Returns whether the snapshot explicitly declares partial
    /// observation consistency.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        self.consistency() == SnapshotConsistency::Partial
    }
}

// ============================================================================
// VALIDATED DESERIALIZATION
// ============================================================================

impl<'de> Deserialize<'de> for HardwareSnapshot {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireSnapshot {
            payload: SnapshotPayload,
            fingerprint: SnapshotFingerprint,
        }

        let wire = WireSnapshot::deserialize(deserializer)?;

        wire.payload
            .validate()
            .map_err(D::Error::custom)?;

        wire.fingerprint
            .validate()
            .map_err(D::Error::custom)?;

        let calculated =
            SnapshotFingerprint::calculate(&wire.payload)
                .map_err(D::Error::custom)?;

        if calculated != wire.fingerprint {
            return Err(D::Error::custom(
                "snapshot fingerprint does not match payload",
            ));
        }

        Ok(Self {
            payload: wire.payload,
            fingerprint: wire.fingerprint,
        })
    }
}

// ============================================================================
// SNAPSHOT BUILDER
// ============================================================================

/// Construction helper for immutable hardware snapshots.
///
/// The builder accumulates observation references and validates
/// the resulting payload before constructing the snapshot.
///
/// It does not perform discovery, allocation or hardware access.
#[derive(Clone, Debug)]
pub struct SnapshotBuilder {
    id: HardwareSnapshotId,
    inventory_id: HardwareInventoryId,
    revision: SnapshotRevision,
    observed_at: SnapshotTimestamp,
    consistency: SnapshotConsistency,
    provenance: SnapshotProvenance,
    devices: BTreeSet<HardwareDeviceIdentity>,
    resources: BTreeSet<HardwareResourceIdentity>,
    capabilities: BTreeSet<SnapshotCapabilityReference>,
    topology: Option<HardwareTopologyId>,
    related_snapshots: BTreeSet<HardwareSnapshotId>,
    references: BTreeSet<SnapshotReference>,
}

impl SnapshotBuilder {
    /// Creates a new snapshot builder.
    #[must_use]
    pub fn new(
        id: HardwareSnapshotId,
        inventory_id: HardwareInventoryId,
        revision: SnapshotRevision,
        observed_at: SnapshotTimestamp,
        provenance: SnapshotProvenance,
    ) -> Self {
        Self {
            id,
            inventory_id,
            revision,
            observed_at,
            consistency: SnapshotConsistency::Unknown,
            provenance,
            devices: BTreeSet::new(),
            resources: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            topology: None,
            related_snapshots: BTreeSet::new(),
            references: BTreeSet::new(),
        }
    }

    /// Sets the observation consistency.
    #[must_use]
    pub fn with_consistency(
        mut self,
        consistency: SnapshotConsistency,
    ) -> Self {
        self.consistency = consistency;
        self
    }

    /// Adds a device observation reference.
    ///
    /// Duplicate device identities are naturally deduplicated.
    #[must_use]
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> Self {
        self.devices.insert(device);
        self
    }

    /// Adds multiple device references.
    #[must_use]
    pub fn with_devices<I>(
        mut self,
        devices: I,
    ) -> Self
    where
        I: IntoIterator<Item = HardwareDeviceIdentity>,
    {
        self.devices.extend(devices);
        self
    }

    /// Adds a physical resource observation reference.
    #[must_use]
    pub fn with_resource(
        mut self,
        resource: HardwareResourceIdentity,
    ) -> Self {
        self.resources.insert(resource);
        self
    }

    /// Adds multiple resource references.
    #[must_use]
    pub fn with_resources<I>(
        mut self,
        resources: I,
    ) -> Self
    where
        I: IntoIterator<Item = HardwareResourceIdentity>,
    {
        self.resources.extend(resources);
        self
    }

    /// Adds a capability observation reference.
    ///
    /// Capability identity is supplied using its canonical
    /// standard-library textual representation.
    pub fn with_capability(
        mut self,
        capability: SnapshotCapabilityReference,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds multiple capability references.
    #[must_use]
    pub fn with_capabilities<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = SnapshotCapabilityReference>,
    {
        self.capabilities.extend(capabilities);
        self
    }

    /// Associates a physical topology observation.
    #[must_use]
    pub fn with_topology(
        mut self,
        topology: HardwareTopologyId,
    ) -> Self {
        self.topology = Some(topology);
        self
    }

    /// Adds a related snapshot reference.
    ///
    /// Self-reference is rejected during construction.
    #[must_use]
    pub fn with_related_snapshot(
        mut self,
        snapshot: HardwareSnapshotId,
    ) -> Self {
        self.related_snapshots.insert(snapshot);
        self
    }

    /// Adds an external observation reference.
    #[must_use]
    pub fn with_reference(
        mut self,
        reference: SnapshotReference,
    ) -> Self {
        self.references.insert(reference);
        self
    }

    /// Adds multiple external observation references.
    #[must_use]
    pub fn with_references<I>(
        mut self,
        references: I,
    ) -> Self
    where
        I: IntoIterator<Item = SnapshotReference>,
    {
        self.references.extend(references);
        self
    }

    /// Builds and validates an immutable hardware snapshot.
    ///
    /// The fingerprint is calculated only after payload validation.
    ///
    /// # Errors
    ///
    /// Returns a structured snapshot error when validation or
    /// canonical serialization fails.
    pub fn build(self) -> SnapshotResult<HardwareSnapshot> {
        let payload = SnapshotPayload {
            schema_id: SNAPSHOT_SCHEMA_ID.to_owned(),
            schema_version: SNAPSHOT_SCHEMA_VERSION,
            id: self.id,
            inventory_id: self.inventory_id,
            revision: self.revision,
            observed_at: self.observed_at,
            consistency: self.consistency,
            provenance: self.provenance,
            devices: self.devices,
            resources: self.resources,
            capabilities: self.capabilities,
            topology: self.topology,
            related_snapshots: self.related_snapshots,
            references: self.references,
        };

        payload.validate()?;

        let fingerprint =
            SnapshotFingerprint::calculate(&payload)?;

        Ok(HardwareSnapshot {
            payload,
            fingerprint,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot_id(value: &str) -> HardwareSnapshotId {
        HardwareSnapshotId::new(value).unwrap()
    }

    fn inventory_id() -> HardwareInventoryId {
        HardwareInventoryId::new("inventory.primary").unwrap()
    }

    fn revision() -> SnapshotRevision {
        SnapshotRevision::new("revision.1").unwrap()
    }

    fn timestamp() -> SnapshotTimestamp {
        SnapshotTimestamp::new(
            "2026-11-15T10:00:00Z",
            "clock.utc",
        )
        .unwrap()
    }

    fn provenance() -> SnapshotProvenance {
        SnapshotProvenance::new("provider.observation")
            .unwrap()
            .with_provider_reference("provider.example")
            .unwrap()
            .with_collection_method("provider.discovery")
            .unwrap()
    }

    fn empty_snapshot() -> HardwareSnapshot {
        HardwareSnapshot::builder(
            snapshot_id("snapshot.primary"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .build()
        .unwrap()
    }

    #[test]
    fn creates_valid_empty_observation_snapshot() {
        let snapshot = empty_snapshot();

        assert_eq!(
            snapshot.id().as_str(),
            "snapshot.primary",
        );

        assert_eq!(
            snapshot.inventory_id().as_str(),
            "inventory.primary",
        );

        assert_eq!(
            snapshot.consistency(),
            SnapshotConsistency::Unknown,
        );
    }

    #[test]
    fn snapshots_are_immutable_through_public_api() {
        let snapshot = empty_snapshot();

        assert_eq!(
            snapshot.payload().revision().as_str(),
            "revision.1",
        );
    }

    #[test]
    fn snapshot_fingerprint_is_deterministic() {
        let first = empty_snapshot();
        let second = empty_snapshot();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn snapshot_fingerprint_changes_with_content() {
        let first = empty_snapshot();

        let second = HardwareSnapshot::builder(
            snapshot_id("snapshot.primary"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_consistency(SnapshotConsistency::Partial)
        .build()
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn snapshot_integrity_verifies() {
        let snapshot = empty_snapshot();

        assert!(
            snapshot.verify_integrity().unwrap()
        );
    }

    #[test]
    fn snapshot_serialization_round_trips() {
        let original = empty_snapshot();

        let encoded =
            serde_json::to_string(&original).unwrap();

        let restored: HardwareSnapshot =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, restored);
        assert!(restored.verify_integrity().unwrap());
    }

    #[test]
    fn snapshot_deserialization_rejects_modified_payload() {
        let snapshot = empty_snapshot();

        let mut encoded =
            serde_json::to_value(&snapshot).unwrap();

        encoded["payload"]["revision"] =
            serde_json::Value::String(
                "revision.changed".to_owned(),
            );

        assert!(
            serde_json::from_value::<HardwareSnapshot>(
                encoded
            ).is_err()
        );
    }

    #[test]
    fn snapshot_rejects_self_reference() {
        let id = snapshot_id("snapshot.self");

        let result = HardwareSnapshot::builder(
            id.clone(),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_related_snapshot(id)
        .build();

        assert_eq!(
            result.unwrap_err().kind(),
            SnapshotErrorKind::SelfReference,
        );
    }

    #[test]
    fn partial_observations_are_supported() {
        let snapshot = HardwareSnapshot::builder(
            snapshot_id("snapshot.partial"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_consistency(SnapshotConsistency::Partial)
        .build()
        .unwrap();

        assert!(snapshot.is_partial());
        assert!(!snapshot.is_declared_complete());
    }

    #[test]
    fn complete_observations_are_distinguishable() {
        let snapshot = HardwareSnapshot::builder(
            snapshot_id("snapshot.complete"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_consistency(SnapshotConsistency::Complete)
        .build()
        .unwrap();

        assert!(snapshot.is_declared_complete());
    }

    #[test]
    fn capability_references_are_extensible() {
        let capability =
            SnapshotCapabilityReference::new(
                "future.photonic.computation",
                Some("observation.capability.1".to_owned()),
            )
            .unwrap();

        let snapshot = HardwareSnapshot::builder(
            snapshot_id("snapshot.capability"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_capability(capability)
        .build()
        .unwrap();

        assert_eq!(
            snapshot.payload().capability_count(),
            1,
        );
    }

    #[test]
    fn arbitrary_future_reference_kinds_are_supported() {
        let reference = SnapshotReference::new(
            SnapshotReferenceKind::new(
                "future.molecular.hardware",
            )
            .unwrap(),
            "molecular.observation.1",
            Some("revision.1".to_owned()),
        )
        .unwrap();

        let snapshot = HardwareSnapshot::builder(
            snapshot_id("snapshot.future"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .with_reference(reference)
        .build()
        .unwrap();

        assert_eq!(
            snapshot.payload().reference_count(),
            1,
        );
    }

    #[test]
    fn timestamp_clock_domain_is_preserved() {
        let value = SnapshotTimestamp::new(
            "provider-clock-value",
            "device.clock.domain",
        )
        .unwrap();

        assert_eq!(
            value.clock_domain(),
            "device.clock.domain",
        );
    }

    #[test]
    fn invalid_revision_is_rejected() {
        assert!(
            SnapshotRevision::new("").is_err()
        );

        assert!(
            SnapshotRevision::new("invalid revision").is_err()
        );
    }

    #[test]
    fn invalid_timestamp_is_rejected() {
        assert!(
            SnapshotTimestamp::new(
                "",
                "clock.utc",
            )
            .is_err()
        );

        assert!(
            SnapshotTimestamp::new(
                "timestamp",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_provenance_is_rejected() {
        assert!(
            SnapshotProvenance::new("").is_err()
        );
    }

    #[test]
    fn fingerprint_algorithm_is_explicit() {
        let snapshot = empty_snapshot();

        assert_eq!(
            snapshot.fingerprint().algorithm(),
            "sha256",
        );

        assert_eq!(
            snapshot.fingerprint().digest().len(),
            64,
        );
    }

    #[test]
    fn duplicate_device_references_are_deduplicated() {
        let snapshot = HardwareSnapshot::builder(
            snapshot_id("snapshot.devices"),
            inventory_id(),
            revision(),
            timestamp(),
            provenance(),
        )
        .build()
        .unwrap();

        assert_eq!(
            snapshot.payload().device_count(),
            0,
        );
    }

    #[test]
    fn empty_observations_are_not_automatically_complete() {
        let snapshot = empty_snapshot();

        assert_eq!(
            snapshot.consistency(),
            SnapshotConsistency::Unknown,
        );
    }
}
