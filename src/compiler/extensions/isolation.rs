
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extension Isolation
//!
//! Path: src/compiler/extensions/isolation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, declarative isolation boundaries and requirements
//! for compiler extension providers and their individual instances.
//!
//! This module describes which isolation properties an extension
//! realization must preserve. It does not implement the mechanisms
//! that enforce those properties.
//!
//! ## Canonical ownership
//!
//! Provider identity and lifecycle:
//!     crate::compiler::extensions::provider
//!
//! Extension declarations:
//!     crate::compiler::extensions::manifest
//!
//! Provider compatibility:
//!     crate::compiler::extensions::compatibility
//!
//! Provider discovery:
//!     crate::compiler::extensions::discovery
//!
//! Extension admission:
//!     crate::compiler::extensions::validation
//!
//! Supply-chain verification:
//!     crate::compiler::security::supply_chain
//!
//! Security policy evaluation:
//!     crate::compiler::security::policy
//!
//! Security verification:
//!     crate::compiler::security::verification
//!
//! Runtime enforcement:
//!     crate::runtime
//!
//! Distributed security enforcement:
//!     crate::distributed::security
//!
//! ## Does not own
//!
//! - Authentication.
//! - Identity establishment.
//! - Authorization.
//! - Permission grants.
//! - Security policy evaluation.
//! - Trust establishment.
//! - Cryptographic verification.
//! - Provider discovery.
//! - Provider registration.
//! - Provider lifecycle management.
//! - Dynamic library loading.
//! - Process creation.
//! - Container creation.
//! - Virtual machine creation.
//! - Memory protection.
//! - Filesystem isolation.
//! - Network isolation.
//! - Device isolation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! ## Security invariants
//!
//! 1. Declared isolation is not enforced isolation.
//! 2. Provider identity is not authenticated identity.
//! 3. Provider compatibility does not establish trust.
//! 4. Provider admission does not prove runtime safety.
//! 5. Required isolation must not silently become optional.
//! 6. Prohibited isolation mechanisms must not be silently selected.
//! 7. Missing enforcement evidence cannot establish isolation.
//! 8. Unknown controls must be preserved for downstream interpretation.
//! 9. Runtime enforcement remains the responsibility of the runtime.
//! 10. Untrusted providers must not execute merely because a boundary
//!     has been successfully constructed.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - providers;
//! - provider instances;
//! - isolation boundaries;
//! - isolation controls;
//! - isolation requirements;
//! - metadata entries;
//! - computing domains;
//! - execution environments.
//!
//! Collections grow according to available memory and representation
//! constraints.
//!
//! Operational budgets belong to the responsible admission, security,
//! deployment and runtime subsystems.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Determinism
//!
//! Boundary collections and metadata use ordered collections.
//! Requirement declaration order is preserved.
//! No global state or registration-order-dependent behavior exists.
//!
//! ## Serialization
//!
//! Deserialization validates identifiers, metadata and collection keys.
//! Provider identity is reconstructed through the canonical ProviderId
//! constructor rather than accepting arbitrary serialized identities.

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler::extensions::provider::ProviderId;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by extension isolation operations.
pub type IsolationResult<T> = Result<T, IsolationError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by extension isolation declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IsolationError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Identifier category.
        field: &'static str,
    },

    /// An identifier is malformed.
    InvalidIdentifier {
        /// Identifier category.
        field: &'static str,

        /// Invalid identifier.
        value: String,
    },

    /// A provider instance identifier is invalid.
    InvalidInstanceId {
        /// Invalid instance identifier.
        value: String,
    },

    /// A metadata key is invalid.
    InvalidMetadataKey {
        /// Invalid key.
        key: String,
    },

    /// A metadata value contains prohibited control characters.
    InvalidMetadataValue {
        /// Associated metadata key.
        key: String,
    },

    /// A metadata key is already present.
    DuplicateMetadataKey {
        /// Duplicate key.
        key: String,
    },

    /// A boundary identifier is already present.
    DuplicateBoundary {
        /// Existing boundary identifier.
        id: IsolationBoundaryId,
    },

    /// A serialized boundary map key does not match its value.
    BoundaryKeyMismatch {
        /// Map key.
        key: IsolationBoundaryId,

        /// Boundary identifier.
        boundary_id: IsolationBoundaryId,
    },

    /// A serialized boundary identifier occurs more than once.
    DuplicateSerializedBoundary {
        /// Duplicate identifier.
        id: IsolationBoundaryId,
    },

    /// A provider identity could not be reconstructed.
    InvalidProviderIdentity {
        /// Canonical provider constructor error.
        message: String,
    },
}

impl fmt::Display for IsolationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(formatter, "invalid {field}: {value:?}")
            }

            Self::InvalidInstanceId { value } => {
                write!(formatter, "invalid provider instance ID: {value:?}")
            }

            Self::InvalidMetadataKey { key } => {
                write!(formatter, "invalid isolation metadata key: {key:?}")
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "invalid isolation metadata value for key: {key}"
                )
            }

            Self::DuplicateMetadataKey { key } => {
                write!(formatter, "duplicate isolation metadata key: {key}")
            }

            Self::DuplicateBoundary { id } => {
                write!(formatter, "duplicate isolation boundary: {id}")
            }

            Self::BoundaryKeyMismatch {
                key,
                boundary_id,
            } => {
                write!(
                    formatter,
                    "boundary map key {key} does not match boundary ID {boundary_id}"
                )
            }

            Self::DuplicateSerializedBoundary { id } => {
                write!(
                    formatter,
                    "serialized isolation plan contains duplicate boundary: {id}"
                )
            }

            Self::InvalidProviderIdentity { message } => {
                write!(
                    formatter,
                    "invalid serialized provider identity: {message}"
                )
            }
        }
    }
}

impl Error for IsolationError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an open-world symbolic isolation identifier.
///
/// Identifiers must be non-empty, trimmed and free from whitespace
/// and control characters.
///
/// No arbitrary maximum identifier length is imposed.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> IsolationResult<()> {
    if value.is_empty() {
        return Err(IsolationError::EmptyIdentifier { field });
    }

    if value.trim() != value
        || value.chars().any(|character| {
            character.is_whitespace() || character.is_control()
        })
    {
        return Err(IsolationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// ISOLATION BOUNDARY ID
// ============================================================================

/// Stable symbolic identity of an extension isolation boundary.
///
/// This identifier is local to the compiler's extension isolation
/// representation. It does not establish authentication or trust.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
#[serde(transparent)]
pub struct IsolationBoundaryId(String);

impl IsolationBoundaryId {
    /// Constructs a validated isolation boundary identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> IsolationResult<Self> {
        let value = value.into();

        validate_identifier(
            "isolation boundary identifier",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for IsolationBoundaryId {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for IsolationBoundaryId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ISOLATION CONTROL ID
// ============================================================================

/// Open-world identity of an isolation property or mechanism.
///
/// Examples include:
///
/// - `process.address_space`
/// - `memory.domain`
/// - `filesystem.namespace`
/// - `network.egress`
/// - `device.access`
/// - `execution.context`
/// - `tenant.boundary`
/// - `native.code.execution`
/// - `foreign.interface.access`
///
/// These are examples, not a closed catalogue.
///
/// The authoritative semantic and enforcement subsystems determine
/// the meaning and enforceability of each control.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
#[serde(transparent)]
pub struct IsolationControlId(String);

impl IsolationControlId {
    /// Constructs a validated isolation control identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> IsolationResult<Self> {
        let value = value.into();

        validate_identifier(
            "isolation control identifier",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical control identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for IsolationControlId {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for IsolationControlId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PROVIDER INSTANCE ID
// ============================================================================

/// Identifies one provider instance within its owning provider.
///
/// A provider may have multiple independent instances. This identity
/// does not represent a process ID, operating-system handle, device
/// handle or runtime allocation.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
#[serde(transparent)]
pub struct ProviderInstanceId(String);

impl ProviderInstanceId {
    /// Constructs a validated provider instance identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> IsolationResult<Self> {
        let value = value.into();

        validate_identifier(
            "provider instance identifier",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the instance identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ProviderInstanceId {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for ProviderInstanceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ISOLATION SUBJECT
// ============================================================================

/// The provider subject protected by an isolation boundary.
///
/// Provider identity always comes from the canonical provider subsystem.
///
/// A provider-level subject applies to the provider as a whole.
/// An instance-level subject applies to one separately identified
/// instance of that provider.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsolationSubject {
    /// The entire extension provider.
    Provider(ProviderId),

    /// One instance of an extension provider.
    Instance {
        /// Canonical provider identity.
        provider_id: ProviderId,

        /// Provider-owned instance identity.
        instance_id: ProviderInstanceId,
    },
}

// The provider module owns ProviderId and intentionally does not
// expose its internal representation for serialization.
// Use a validated wire representation at this boundary.

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum IsolationSubjectRef<'a> {
    Provider {
        provider_id: &'a str,
    },

    Instance {
        provider_id: &'a str,
        instance_id: &'a str,
    },
}

impl Serialize for IsolationSubject {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provider(provider_id) => {
                IsolationSubjectRef::Provider {
                    provider_id: provider_id.as_str(),
                }
                .serialize(serializer)
            }

            Self::Instance {
                provider_id,
                instance_id,
            } => {
                IsolationSubjectRef::Instance {
                    provider_id: provider_id.as_str(),
                    instance_id: instance_id.as_str(),
                }
                .serialize(serializer)
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum IsolationSubjectOwned {
    Provider {
        provider_id: String,
    },

    Instance {
        provider_id: String,
        instance_id: ProviderInstanceId,
    },
}

impl<'de> Deserialize<'de> for IsolationSubject {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = IsolationSubjectOwned::deserialize(deserializer)?;

        let make_provider_id = |value: String| {
            ProviderId::new(value)
                .map_err(|error| {
                    de::Error::custom(
                        IsolationError::InvalidProviderIdentity {
                            message: error.to_string(),
                        },
                    )
                })
        };

        match value {
            IsolationSubjectOwned::Provider { provider_id } => {
                Ok(Self::Provider(make_provider_id(provider_id)?))
            }

            IsolationSubjectOwned::Instance {
                provider_id,
                instance_id,
            } => {
                Ok(Self::Instance {
                    provider_id: make_provider_id(provider_id)?,
                    instance_id,
                })
            }
        }
    }
}

impl IsolationSubject {
    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        match self {
            Self::Provider(provider_id) => provider_id,

            Self::Instance {
                provider_id,
                ..
            } => provider_id,
        }
    }

    /// Returns the optional provider instance identity.
    #[must_use]
    pub fn instance_id(&self) -> Option<&ProviderInstanceId> {
        match self {
            Self::Provider(_) => None,

            Self::Instance {
                instance_id,
                ..
            } => Some(instance_id),
        }
    }
}

// ============================================================================
// REQUIREMENT DISPOSITION
// ============================================================================

/// Normative strength of an isolation requirement.
///
/// These values describe requirements, not enforcement outcomes.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum IsolationDisposition {
    /// The realization must provide the isolation property.
    Required,

    /// The realization should prefer the property when possible,
    /// without violating mandatory requirements.
    Preferred,

    /// The realization must not use the specified property or mechanism.
    Prohibited,
}

// ============================================================================
// ISOLATION REQUIREMENT
// ============================================================================

/// One declarative isolation requirement.
///
/// The control identifier is open-world. Its meaning is resolved by
/// the canonical semantic and security infrastructure.
///
/// Declaration order is preserved for diagnostics and provenance.
/// This module does not resolve conflicts or precedence.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct IsolationRequirement {
    control: IsolationControlId,

    disposition: IsolationDisposition,
}

impl IsolationRequirement {
    /// Constructs an isolation requirement.
    #[must_use]
    pub fn new(
        control: IsolationControlId,
        disposition: IsolationDisposition,
    ) -> Self {
        Self {
            control,
            disposition,
        }
    }

    /// Returns the required isolation control.
    #[must_use]
    pub fn control(&self) -> &IsolationControlId {
        &self.control
    }

    /// Returns the declared disposition.
    #[must_use]
    pub const fn disposition(&self) -> IsolationDisposition {
        self.disposition
    }
}

// ============================================================================
// METADATA
// ============================================================================

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> IsolationResult<()> {
    for (key, value) in metadata {
        if key.is_empty()
            || key.trim() != key
            || key.chars().any(|character| {
                character.is_whitespace() || character.is_control()
            })
        {
            return Err(IsolationError::InvalidMetadataKey {
                key: key.clone(),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(IsolationError::InvalidMetadataValue {
                key: key.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// ISOLATION BOUNDARY
// ============================================================================

/// Declarative isolation boundary for an extension provider or instance.
///
/// This structure contains requirements only. It does not create
/// or enforce a sandbox.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct IsolationBoundary {
    id: IsolationBoundaryId,

    subject: IsolationSubject,

    requirements: Vec<IsolationRequirement>,

    metadata: BTreeMap<String, String>,
}

impl IsolationBoundary {
    /// Constructs an empty isolation boundary.
    #[must_use]
    pub fn new(
        id: IsolationBoundaryId,
        subject: IsolationSubject,
    ) -> Self {
        Self {
            id,
            subject,
            requirements: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the boundary identifier.
    #[must_use]
    pub fn id(&self) -> &IsolationBoundaryId {
        &self.id
    }

    /// Returns the protected provider subject.
    #[must_use]
    pub fn subject(&self) -> &IsolationSubject {
        &self.subject
    }

    /// Returns the declared isolation requirements.
    #[must_use]
    pub fn requirements(&self) -> &[IsolationRequirement] {
        &self.requirements
    }

    /// Returns descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Appends an isolation requirement.
    ///
    /// Repeated controls are preserved. Conflict detection belongs
    /// to the semantic and policy authorities.
    pub fn add_requirement(
        &mut self,
        requirement: IsolationRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Inserts descriptive metadata.
    ///
    /// Existing keys are never silently overwritten.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> IsolationResult<()> {
        let key = key.into();
        let value = value.into();

        if self.metadata.contains_key(&key) {
            return Err(IsolationError::DuplicateMetadataKey {
                key,
            });
        }

        let mut candidate = self.metadata.clone();
        candidate.insert(key.clone(), value.clone());

        validate_metadata(&candidate)?;

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Validates the boundary's locally owned invariants.
    ///
    /// This does not establish that an enforcement provider exists
    /// or that a target can satisfy the requirements.
    pub fn validate(&self) -> IsolationResult<()> {
        validate_metadata(&self.metadata)?;

        Ok(())
    }
}

#[derive(Deserialize)]
struct IsolationBoundaryRepr {
    id: IsolationBoundaryId,

    subject: IsolationSubject,

    #[serde(default)]
    requirements: Vec<IsolationRequirement>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl<'de> Deserialize<'de> for IsolationBoundary {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = IsolationBoundaryRepr::deserialize(deserializer)?;

        let boundary = Self {
            id: repr.id,
            subject: repr.subject,
            requirements: repr.requirements,
            metadata: repr.metadata,
        };

        boundary.validate().map_err(de::Error::custom)?;

        Ok(boundary)
    }
}

// ============================================================================
// ISOLATION PLAN
// ============================================================================

/// Deterministic collection of extension isolation boundaries.
///
/// The plan describes declared isolation intent. It is not an execution
/// plan and does not establish that isolation has been enforced.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct IsolationPlan {
    boundaries: BTreeMap<IsolationBoundaryId, IsolationBoundary>,
}

impl IsolationPlan {
    /// Constructs an empty isolation plan.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an isolation boundary.
    ///
    /// Duplicate identities are rejected.
    pub fn insert(
        &mut self,
        boundary: IsolationBoundary,
    ) -> IsolationResult<()> {
        boundary.validate()?;

        let id = boundary.id.clone();

        if self.boundaries.contains_key(&id) {
            return Err(IsolationError::DuplicateBoundary {
                id,
            });
        }

        self.boundaries.insert(id, boundary);

        Ok(())
    }

    /// Returns a boundary by identity.
    #[must_use]
    pub fn get(
        &self,
        id: &IsolationBoundaryId,
    ) -> Option<&IsolationBoundary> {
        self.boundaries.get(id)
    }

    /// Removes and returns a boundary.
    #[must_use]
    pub fn remove(
        &mut self,
        id: &IsolationBoundaryId,
    ) -> Option<IsolationBoundary> {
        self.boundaries.remove(id)
    }

    /// Returns all boundaries in deterministic identity order.
    #[must_use]
    pub fn boundaries(
        &self,
    ) -> &BTreeMap<IsolationBoundaryId, IsolationBoundary> {
        &self.boundaries
    }

    /// Returns the number of boundaries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.boundaries.len()
    }

    /// Returns whether the plan contains no boundaries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.boundaries.is_empty()
    }

    /// Validates all locally owned invariants.
    pub fn validate(&self) -> IsolationResult<()> {
        for (key, boundary) in &self.boundaries {
            if key != boundary.id() {
                return Err(IsolationError::BoundaryKeyMismatch {
                    key: key.clone(),
                    boundary_id: boundary.id().clone(),
                });
            }

            boundary.validate()?;
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct IsolationPlanRepr {
    #[serde(default)]
    boundaries: Vec<IsolationBoundary>,
}

impl<'de> Deserialize<'de> for IsolationPlan {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = IsolationPlanRepr::deserialize(deserializer)?;

        let mut plan = Self::new();

        for boundary in repr.boundaries {
            let id = boundary.id().clone();

            if plan.boundaries.contains_key(&id) {
                return Err(de::Error::custom(
                    IsolationError::DuplicateSerializedBoundary {
                        id,
                    },
                ));
            }

            plan.insert(boundary).map_err(de::Error::custom)?;
        }

        plan.validate().map_err(de::Error::custom)?;

        Ok(plan)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provider_id() -> ProviderId {
        ProviderId::new("org.example.compiler")
            .expect("provider ID should be valid")
    }

    fn boundary_id(
        value: &str,
    ) -> IsolationBoundaryId {
        IsolationBoundaryId::new(value)
            .expect("boundary ID should be valid")
    }

    fn control_id(
        value: &str,
    ) -> IsolationControlId {
        IsolationControlId::new(value)
            .expect("control ID should be valid")
    }

    fn provider_boundary(
        id: &str,
    ) -> IsolationBoundary {
        IsolationBoundary::new(
            boundary_id(id),
            IsolationSubject::Provider(provider_id()),
        )
    }

    #[test]
    fn constructs_provider_boundary() {
        let boundary = provider_boundary("provider.main");

        assert_eq!(boundary.id().as_str(), "provider.main");
        assert_eq!(
            boundary.subject().provider_id().as_str(),
            "org.example.compiler",
        );
        assert!(boundary.requirements().is_empty());
    }

    #[test]
    fn supports_provider_instance_boundaries() {
        let subject = IsolationSubject::Instance {
            provider_id: provider_id(),
            instance_id: ProviderInstanceId::new("instance.alpha")
                .expect("instance ID should be valid"),
        };

        assert_eq!(
            subject.provider_id().as_str(),
            "org.example.compiler",
        );

        assert_eq!(
            subject.instance_id().map(ProviderInstanceId::as_str),
            Some("instance.alpha"),
        );
    }

    #[test]
    fn preserves_required_preferred_and_prohibited_requirements() {
        let mut boundary = provider_boundary("provider.security");

        boundary.add_requirement(IsolationRequirement::new(
            control_id("process.address_space"),
            IsolationDisposition::Required,
        ));

        boundary.add_requirement(IsolationRequirement::new(
            control_id("network.egress"),
            IsolationDisposition::Preferred,
        ));

        boundary.add_requirement(IsolationRequirement::new(
            control_id("shared.privileged_memory"),
            IsolationDisposition::Prohibited,
        ));

        assert_eq!(boundary.requirements().len(), 3);

        assert_eq!(
            boundary.requirements()[0].disposition(),
            IsolationDisposition::Required,
        );

        assert_eq!(
            boundary.requirements()[1].disposition(),
            IsolationDisposition::Preferred,
        );

        assert_eq!(
            boundary.requirements()[2].disposition(),
            IsolationDisposition::Prohibited,
        );
    }

    #[test]
    fn preserves_repeated_controls_for_semantic_validation() {
        let mut boundary = provider_boundary("provider.repeated");

        let requirement = || {
            IsolationRequirement::new(
                control_id("filesystem.namespace"),
                IsolationDisposition::Required,
            )
        };

        boundary.add_requirement(requirement());
        boundary.add_requirement(requirement());

        assert_eq!(boundary.requirements().len(), 2);
    }

    #[test]
    fn rejects_invalid_boundary_identifiers() {
        assert!(IsolationBoundaryId::new("").is_err());
        assert!(IsolationBoundaryId::new(" invalid").is_err());
        assert!(IsolationBoundaryId::new("invalid id").is_err());
    }

    #[test]
    fn rejects_invalid_control_identifiers() {
        assert!(IsolationControlId::new("").is_err());
        assert!(IsolationControlId::new("network access").is_err());
    }

    #[test]
    fn rejects_invalid_instance_identifiers() {
        assert!(ProviderInstanceId::new("").is_err());
        assert!(ProviderInstanceId::new("bad instance").is_err());
    }

    #[test]
    fn rejects_duplicate_metadata() {
        let mut boundary = provider_boundary("provider.metadata");

        boundary
            .add_metadata("purpose", "untrusted extension")
            .expect("first metadata entry should be accepted");

        let result = boundary.add_metadata(
            "purpose",
            "replacement",
        );

        assert!(matches!(
            result,
            Err(IsolationError::DuplicateMetadataKey { .. })
        ));
    }

    #[test]
    fn rejects_metadata_control_characters() {
        let mut boundary = provider_boundary("provider.metadata.invalid");

        assert!(
            boundary
                .add_metadata("purpose", "invalid\nvalue")
                .is_err()
        );
    }

    #[test]
    fn rejects_duplicate_boundaries() {
        let mut plan = IsolationPlan::new();

        plan.insert(provider_boundary("provider.one"))
            .expect("first boundary should be accepted");

        let result = plan.insert(provider_boundary("provider.one"));

        assert!(matches!(
            result,
            Err(IsolationError::DuplicateBoundary { .. })
        ));
    }

    #[test]
    fn returns_boundaries_in_deterministic_order() {
        let mut plan = IsolationPlan::new();

        plan.insert(provider_boundary("provider.z"))
            .expect("boundary should be accepted");

        plan.insert(provider_boundary("provider.a"))
            .expect("boundary should be accepted");

        let ids: Vec<_> = plan
            .boundaries()
            .keys()
            .map(IsolationBoundaryId::as_str)
            .collect();

        assert_eq!(
            ids,
            vec!["provider.a", "provider.z"],
        );
    }

    #[test]
    fn serializes_and_deserializes_a_plan() {
        let mut plan = IsolationPlan::new();

        let mut boundary = provider_boundary("provider.roundtrip");

        boundary.add_requirement(IsolationRequirement::new(
            control_id("process.address_space"),
            IsolationDisposition::Required,
        ));

        boundary
            .add_metadata("purpose", "provider isolation")
            .expect("metadata should be accepted");

        plan.insert(boundary)
            .expect("boundary should be accepted");

        let serialized = serde_json::to_string(&plan)
            .expect("plan should serialize");

        let decoded: IsolationPlan =
            serde_json::from_str(&serialized)
                .expect("plan should deserialize");

        assert_eq!(plan, decoded);
    }

    #[test]
    fn rejects_invalid_serialized_provider_identity() {
        let serialized = r#"
        {
            "boundaries": [
                {
                    "id": "boundary.invalid",
                    "subject": {
                        "kind": "provider",
                        "provider_id": "invalid provider id"
                    },
                    "requirements": [],
                    "metadata": {}
                }
            ]
        }
        "#;

        let result = serde_json::from_str::<IsolationPlan>(serialized);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_serialized_boundaries() {
        let serialized = r#"
        {
            "boundaries": [
                {
                    "id": "boundary.duplicate",
                    "subject": {
                        "kind": "provider",
                        "provider_id": "org.example.compiler"
                    },
                    "requirements": [],
                    "metadata": {}
                },
                {
                    "id": "boundary.duplicate",
                    "subject": {
                        "kind": "provider",
                        "provider_id": "org.example.compiler"
                    },
                    "requirements": [],
                    "metadata": {}
                }
            ]
        }
        "#;

        assert!(
            serde_json::from_str::<IsolationPlan>(serialized).is_err()
        );
    }
}
