
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Security — Isolation Requirements
//!
//! Path: `src/ai/security/isolation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, declarative isolation requirements for AI entities
//! and AI execution contexts.
//!
//! This module describes the isolation properties that a valid execution
//! realization must preserve. It does not implement the mechanisms that
//! provide those properties.
//!
//! ## Architectural ownership
//!
//! Existing authorities remain:
//!
//! - `ai::security::identity`:
//!   AI entity and execution identity references.
//!
//! - `ai::security::authorization`:
//!   AI authorization integration.
//!
//! - `ai::security::policies`:
//!   AI security policy attachments.
//!
//! - `ai::agents::policies`:
//!   Agent policy references and attachments.
//!
//! - `ai::requirements`:
//!   AI computational requirements.
//!
//! - `ai::capabilities`:
//!   AI capability integration.
//!
//! - `frontend::semantic`:
//!   Source-level semantic resolution and validation.
//!
//! - `distributed::security`:
//!   Authentication, authorization and policy enforcement.
//!
//! - `runtime`:
//!   Actual execution lifecycle and enforcement integration.
//!
//! - `grammar/security/sandbox.g4`:
//!   Source-level sandbox syntax.
//!
//! - `grammar/security/security-constraints.g4`:
//!   Security-specific constraint syntax.
//!
//! - `grammar/ai/ai.g4`:
//!   AI grammar composition.
//!
//! ## Owns
//!
//! - AI isolation boundary identifiers.
//! - AI isolation control identifiers.
//! - AI isolation subject references.
//! - Isolation requirement declarations.
//! - Isolation requirement dispositions.
//! - Isolation boundary metadata.
//! - Deterministic isolation boundary collections.
//! - Structural validation.
//! - Invariant-preserving deserialization.
//!
//! ## Does not own
//!
//! - Authentication.
//! - Trust establishment.
//! - Authorization decisions.
//! - Permission grants.
//! - Policy evaluation.
//! - Policy precedence.
//! - Policy conflict resolution.
//! - Policy storage or distribution.
//! - Operating-system process isolation.
//! - Container creation.
//! - Virtual-machine creation.
//! - Memory protection.
//! - Filesystem isolation.
//! - Network filtering.
//! - Hardware enclaves.
//! - Device isolation.
//! - Resource allocation.
//! - Scheduling.
//! - Hardware selection.
//! - Runtime enforcement.
//!
//! ## Security invariants
//!
//! 1. An isolation declaration is not an enforcement mechanism.
//! 2. An isolation requirement does not grant authorization.
//! 3. An identity reference does not establish authentication.
//! 4. A capability declaration does not prove isolation.
//! 5. A preferred isolation property is not a mandatory guarantee.
//! 6. A prohibited property must not be silently treated as optional.
//! 7. An unsupported mandatory requirement must be reported downstream.
//! 8. An unresolved requirement must not be considered satisfied.
//! 9. An unavailable enforcement provider must not imply successful isolation.
//! 10. A runtime must verify isolation at the actual enforcement boundary.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no artificial semantic maximum on:
//!
//! - isolation boundaries;
//! - isolation requirements;
//! - AI entities;
//! - execution contexts;
//! - isolation controls;
//! - metadata entries.
//!
//! Collections grow according to available memory and representation limits.
//!
//! Operational budgets belong to the appropriate compiler, runtime,
//! deployment or security provider. They must not become language-level
//! semantic ceilings.
//!
//! Isolation controls are open-world symbolic identifiers. New isolation
//! mechanisms can be represented without changing this module.
//!
//! No physical hardware, operating system, vendor, processor, accelerator,
//! memory capacity, worker count or network topology is assumed.
//!
//! Literal infinite physical capacity is not promised.
//!
//! ## Integration
//!
//! `security/identity.rs`
//!     -> AI entity and execution references.
//!
//! `security/policies.rs`
//!     -> policy attachments remain independently represented.
//!
//! `security/authorization.rs`
//!     -> protected operations require canonical authorization.
//!
//! `ai/requirements.rs`
//!     -> isolation requirements participate in AI requirement analysis.
//!
//! `frontend/semantic/domains/ai.rs`
//!     -> resolves source-level isolation declarations.
//!
//! `grammar/security/sandbox.g4`
//!     -> supplies sandbox source syntax.
//!
//! `grammar/security/security-constraints.g4`
//!     -> supplies security constraint syntax.
//!
//! `distributed/security/`
//!     -> supplies canonical security enforcement.
//!
//! `runtime/`
//!     -> verifies and enforces the realized isolation boundary.
//!
//! ## Semantic boundary
//!
//! This module preserves declarations. It does not decide whether two
//! requirements conflict, whether a target can satisfy a requirement,
//! or which isolation provider should be selected.
//!
//! Those conclusions require the canonical semantic, capability,
//! policy and execution-feasibility authorities.
//!
//! ## Serialization
//!
//! Deserialization validates identifiers, metadata and collection keys.
//! Invalid serialized data cannot bypass the public constructors.
//!
//! ## Concurrency
//!
//! This module has no global mutable state, background workers or hidden
//! synchronization.
//!
//! Collections are owned by their callers and can be shared using the
//! synchronization mechanisms selected by the application.
//!
//! ## Freeze criteria
//!
//! - Existing AI identity references are reused.
//! - Isolation controls remain open-world.
//! - Requirement dispositions remain explicit.
//! - No enforcement engine is duplicated.
//! - Deserialization preserves invariants.
//! - Collections are deterministic.
//! - No arbitrary semantic ceilings are introduced.
//! - No unsafe Rust is used.

use serde::de;
use serde::{Deserialize, Deserializer, Serialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::security::identity::{
    AiExecutionRef,
    AiIdentityRef,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by AI isolation operations.
pub type AiIsolationResult<T> = Result<T, AiIsolationError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by AI isolation declarations and collections.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiIsolationError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Identifier category.
        field: &'static str,
    },

    /// An identifier is malformed.
    InvalidIdentifier {
        /// Identifier category.
        field: &'static str,

        /// Invalid value.
        value: String,
    },

    /// A boundary identifier is already present.
    DuplicateBoundary {
        /// Existing boundary identifier.
        id: AiIsolationBoundaryId,
    },

    /// A serialized collection key differs from its boundary identifier.
    BoundaryKeyMismatch {
        /// Map key.
        key: AiIsolationBoundaryId,

        /// Boundary's internal identifier.
        boundary_id: AiIsolationBoundaryId,
    },

    /// A metadata key is already present.
    DuplicateMetadataKey {
        /// Duplicate metadata key.
        key: String,
    },

    /// A metadata key is invalid.
    InvalidMetadataKey {
        /// Invalid key.
        key: String,
    },

    /// A metadata value contains invalid control characters.
    InvalidMetadataValue {
        /// Associated metadata key.
        key: String,
    },
}

impl fmt::Display for AiIsolationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(formatter, "invalid {field}: {value:?}")
            }

            Self::DuplicateBoundary { id } => {
                write!(formatter, "duplicate isolation boundary: {id}")
            }

            Self::BoundaryKeyMismatch { key, boundary_id } => {
                write!(
                    formatter,
                    "isolation boundary map key {key} does not match boundary ID {boundary_id}"
                )
            }

            Self::DuplicateMetadataKey { key } => {
                write!(formatter, "duplicate isolation metadata key: {key}")
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
        }
    }
}

impl Error for AiIsolationError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate an open-world symbolic identifier.
///
/// Identifiers must be non-empty, have no surrounding whitespace,
/// and contain no whitespace or control characters.
///
/// No arbitrary maximum length is imposed.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> AiIsolationResult<()> {
    if value.is_empty() {
        return Err(AiIsolationError::EmptyIdentifier { field });
    }

    if value.trim() != value
        || value.chars().any(|character| {
            character.is_whitespace() || character.is_control()
        })
    {
        return Err(AiIsolationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// ISOLATION BOUNDARY ID
// ============================================================================

/// Stable symbolic identity for an AI isolation boundary.
///
/// The identifier is local to the owning semantic or runtime authority.
/// It is not a globally unique identity and does not establish trust.
///
/// The namespace is open-world.
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
pub struct AiIsolationBoundaryId(String);

impl AiIsolationBoundaryId {
    /// Construct a validated isolation boundary identifier.
    pub fn new(value: impl Into<String>) -> AiIsolationResult<Self> {
        let value = value.into();

        validate_identifier("isolation boundary identifier", &value)?;

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string representation.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for AiIsolationBoundaryId {
    type Error = AiIsolationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AiIsolationBoundaryId> for String {
    fn from(value: AiIsolationBoundaryId) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for AiIsolationBoundaryId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for AiIsolationBoundaryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ISOLATION CONTROL ID
// ============================================================================

/// Open-world identifier for an isolation control or property.
///
/// Examples include:
///
/// - `process.address_space`
/// - `memory.domain`
/// - `dataset.access`
/// - `model.state`
/// - `agent.state`
/// - `execution.context`
/// - `network.external_access`
/// - `accelerator.shared_access`
/// - `tenant.boundary`
///
/// These examples are descriptive identifiers, not a closed catalogue.
///
/// The actual meaning and enforceability of a control are defined by
/// the authoritative semantic and enforcement providers.
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
pub struct AiIsolationControlId(String);

impl AiIsolationControlId {
    /// Construct a validated isolation control identifier.
    pub fn new(value: impl Into<String>) -> AiIsolationResult<Self> {
        let value = value.into();

        validate_identifier("isolation control identifier", &value)?;

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string representation.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for AiIsolationControlId {
    type Error = AiIsolationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AiIsolationControlId> for String {
    fn from(value: AiIsolationControlId) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for AiIsolationControlId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for AiIsolationControlId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ISOLATION SUBJECT
// ============================================================================

/// An AI entity or execution context to which an isolation boundary applies.
///
/// Entity identity and execution identity are deliberately distinct.
///
/// An entity reference identifies a logical AI resource. An execution
/// reference identifies an execution subject within an authority.
///
/// Neither reference proves authentication, ownership or authorization.
#[derive(
    Clone,
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
pub enum AiIsolationSubject {
    /// An AI model, agent, dataset, training run, pipeline or artifact.
    Entity(AiIdentityRef),

    /// An execution context identified by an external authority.
    Execution(AiExecutionRef),
}

// ============================================================================
// ISOLATION REQUIREMENT DISPOSITION
// ============================================================================

/// The normative strength of an isolation declaration.
///
/// These values describe source-level intent. They are not enforcement
/// results and do not establish that a target can satisfy the declaration.
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
pub enum AiIsolationDisposition {
    /// The realization must provide the declared isolation property.
    Required,

    /// The realization should prefer the declared isolation property
    /// when it can do so without violating mandatory requirements.
    Preferred,

    /// The realization must not use the declared isolation property
    /// or mechanism.
    Prohibited,
}

// ============================================================================
// ISOLATION REQUIREMENT
// ============================================================================

/// One declarative isolation requirement.
///
/// The control identifier is open-world. Its detailed semantics are
/// resolved by the canonical semantic and security infrastructure.
///
/// Requirements are preserved in declaration order. This module does
/// not resolve conflicts, precedence or inheritance.
///
/// A repeated control is intentionally preserved so that the semantic
/// layer can diagnose conflicting or repeated source declarations.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AiIsolationRequirement {
    control: AiIsolationControlId,

    disposition: AiIsolationDisposition,
}

impl AiIsolationRequirement {
    /// Construct an isolation requirement.
    pub fn new(
        control: AiIsolationControlId,
        disposition: AiIsolationDisposition,
    ) -> Self {
        Self {
            control,
            disposition,
        }
    }

    /// Return the isolation control.
    pub fn control(&self) -> &AiIsolationControlId {
        &self.control
    }

    /// Return the declared disposition.
    pub const fn disposition(&self) -> AiIsolationDisposition {
        self.disposition
    }
}

// ============================================================================
// METADATA VALIDATION
// ============================================================================

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> AiIsolationResult<()> {
    for (key, value) in metadata {
        if key.is_empty()
            || key.trim() != key
            || key.chars().any(|character| {
                character.is_whitespace() || character.is_control()
            })
        {
            return Err(AiIsolationError::InvalidMetadataKey {
                key: key.clone(),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(AiIsolationError::InvalidMetadataValue {
                key: key.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// ISOLATION BOUNDARY
// ============================================================================

/// A declarative isolation boundary for an AI entity or execution context.
///
/// This type describes requirements only. It does not create a process,
/// container, virtual machine, memory domain, security principal or
/// enforcement provider.
///
/// Requirement order is preserved because source declaration order
/// can be useful for diagnostics and provenance. No precedence is implied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AiIsolationBoundary {
    id: AiIsolationBoundaryId,

    subject: AiIsolationSubject,

    requirements: Vec<AiIsolationRequirement>,

    metadata: BTreeMap<String, String>,
}

impl AiIsolationBoundary {
    /// Construct an empty isolation boundary.
    pub fn new(
        id: AiIsolationBoundaryId,
        subject: AiIsolationSubject,
    ) -> Self {
        Self {
            id,
            subject,
            requirements: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Return the boundary identifier.
    pub fn id(&self) -> &AiIsolationBoundaryId {
        &self.id
    }

    /// Return the subject protected by this boundary.
    pub fn subject(&self) -> &AiIsolationSubject {
        &self.subject
    }

    /// Return the isolation requirements in declaration order.
    pub fn requirements(&self) -> &[AiIsolationRequirement] {
        &self.requirements
    }

    /// Return descriptive metadata.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Append an isolation requirement.
    ///
    /// Repeated controls are preserved. Conflict detection and semantic
    /// interpretation belong to the frontend semantic authority.
    pub fn add_requirement(
        &mut self,
        requirement: AiIsolationRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Insert descriptive metadata.
    ///
    /// Duplicate keys are rejected instead of silently replacing
    /// previously supplied metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiIsolationResult<()> {
        let key = key.into();
        let value = value.into();

        let mut candidate = self.metadata.clone();

        if candidate.contains_key(&key) {
            return Err(AiIsolationError::DuplicateMetadataKey {
                key,
            });
        }

        candidate.insert(key.clone(), value.clone());

        validate_metadata(&candidate)?;

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Validate this boundary's structural invariants.
    pub fn validate(&self) -> AiIsolationResult<()> {
        validate_metadata(&self.metadata)?;

        for (key, value) in &self.metadata {
            if key.is_empty() {
                return Err(AiIsolationError::InvalidMetadataKey {
                    key: key.clone(),
                });
            }

            if value.chars().any(char::is_control) {
                return Err(AiIsolationError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct AiIsolationBoundaryRepr {
    id: AiIsolationBoundaryId,

    subject: AiIsolationSubject,

    #[serde(default)]
    requirements: Vec<AiIsolationRequirement>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl<'de> Deserialize<'de> for AiIsolationBoundary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = AiIsolationBoundaryRepr::deserialize(deserializer)?;

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

/// Deterministic collection of AI isolation boundaries.
///
/// The collection is keyed by canonical boundary identifiers.
///
/// It owns no global state and performs no runtime enforcement.
///
/// Insertion rejects duplicate boundary identities. It does not silently
/// replace an existing boundary.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct AiIsolationPlan {
    boundaries: BTreeMap<AiIsolationBoundaryId, AiIsolationBoundary>,
}

impl AiIsolationPlan {
    /// Construct an empty isolation plan.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a boundary.
    ///
    /// Duplicate identifiers are rejected.
    pub fn insert(
        &mut self,
        boundary: AiIsolationBoundary,
    ) -> AiIsolationResult<()> {
        boundary.validate()?;

        let id = boundary.id.clone();

        if self.boundaries.contains_key(&id) {
            return Err(AiIsolationError::DuplicateBoundary { id });
        }

        self.boundaries.insert(id, boundary);

        Ok(())
    }

    /// Get a boundary by identifier.
    pub fn get(
        &self,
        id: &AiIsolationBoundaryId,
    ) -> Option<&AiIsolationBoundary> {
        self.boundaries.get(id)
    }

    /// Remove a boundary by identifier.
    pub fn remove(
        &mut self,
        id: &AiIsolationBoundaryId,
    ) -> Option<AiIsolationBoundary> {
        self.boundaries.remove(id)
    }

    /// Iterate over boundaries in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AiIsolationBoundaryId, &AiIsolationBoundary)> {
        self.boundaries.iter()
    }

    /// Return the number of boundaries.
    pub fn len(&self) -> usize {
        self.boundaries.len()
    }

    /// Return whether the plan contains no boundaries.
    pub fn is_empty(&self) -> bool {
        self.boundaries.is_empty()
    }

    /// Validate all boundaries and collection-key invariants.
    pub fn validate(&self) -> AiIsolationResult<()> {
        for (key, boundary) in &self.boundaries {
            if key != boundary.id() {
                return Err(AiIsolationError::BoundaryKeyMismatch {
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
struct AiIsolationPlanRepr {
    #[serde(default)]
    boundaries: BTreeMap<AiIsolationBoundaryId, AiIsolationBoundary>,
}

impl<'de> Deserialize<'de> for AiIsolationPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = AiIsolationPlanRepr::deserialize(deserializer)?;

        let plan = Self {
            boundaries: repr.boundaries,
        };

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

    fn boundary_id(value: &str) -> AiIsolationBoundaryId {
        AiIsolationBoundaryId::new(value)
            .expect("valid boundary identifier")
    }

    fn control(value: &str) -> AiIsolationControlId {
        AiIsolationControlId::new(value)
            .expect("valid control identifier")
    }

    fn boundary(value: &str) -> AiIsolationBoundary {
        use crate::ai::security::identity::{
            AiEntityRef,
            IdentityAuthority,
        };

        use crate::ai::ids::AiModelId;

        let authority = IdentityAuthority::new("zamani::test")
            .expect("valid authority");

        let model_id = AiModelId::new(1)
            .expect("valid model ID");

        let identity = AiIdentityRef::new(
            authority,
            AiEntityRef::Model(model_id),
        );

        AiIsolationBoundary::new(
            boundary_id(value),
            AiIsolationSubject::Entity(identity),
        )
    }

    #[test]
    fn boundary_identifiers_reject_invalid_values() {
        assert!(AiIsolationBoundaryId::new("").is_err());
        assert!(AiIsolationBoundaryId::new(" leading").is_err());
        assert!(AiIsolationBoundaryId::new("trailing ").is_err());
        assert!(AiIsolationBoundaryId::new("contains space").is_err());
        assert!(AiIsolationBoundaryId::new("invalid\nname").is_err());

        assert!(
            AiIsolationBoundaryId::new(
                "ai::security::model_boundary"
            )
            .is_ok()
        );
    }

    #[test]
    fn isolation_control_identifiers_are_open_world() {
        assert!(
            AiIsolationControlId::new(
                "future::isolation::mechanism"
            )
            .is_ok()
        );
    }

    #[test]
    fn boundary_preserves_explicit_requirement_dispositions() {
        let mut boundary = boundary("boundary::model");

        boundary.add_requirement(
            AiIsolationRequirement::new(
                control("memory.domain"),
                AiIsolationDisposition::Required,
            ),
        );

        boundary.add_requirement(
            AiIsolationRequirement::new(
                control("network.external_access"),
                AiIsolationDisposition::Prohibited,
            ),
        );

        boundary.add_requirement(
            AiIsolationRequirement::new(
                control("accelerator.shared_access"),
                AiIsolationDisposition::Preferred,
            ),
        );

        assert_eq!(boundary.requirements().len(), 3);

        assert_eq!(
            boundary.requirements()[0].disposition(),
            AiIsolationDisposition::Required,
        );

        assert_eq!(
            boundary.requirements()[1].disposition(),
            AiIsolationDisposition::Prohibited,
        );

        assert_eq!(
            boundary.requirements()[2].disposition(),
            AiIsolationDisposition::Preferred,
        );
    }

    #[test]
    fn repeated_controls_are_preserved_for_semantic_analysis() {
        let mut boundary = boundary("boundary::repeated");

        boundary.add_requirement(
            AiIsolationRequirement::new(
                control("memory.domain"),
                AiIsolationDisposition::Required,
            ),
        );

        boundary.add_requirement(
            AiIsolationRequirement::new(
                control("memory.domain"),
                AiIsolationDisposition::Prohibited,
            ),
        );

        assert_eq!(boundary.requirements().len(), 2);
    }

    #[test]
    fn metadata_rejects_control_characters() {
        let mut boundary = boundary("boundary::metadata");

        assert!(
            boundary.add_metadata(
                "description",
                "invalid\nvalue",
            ).is_err()
        );
    }

    #[test]
    fn metadata_rejects_duplicate_keys() {
        let mut boundary = boundary("boundary::metadata");

        boundary.add_metadata("owner", "team-a")
            .expect("first metadata value");

        assert!(
            boundary.add_metadata("owner", "team-b").is_err()
        );

        assert_eq!(
            boundary.metadata().get("owner").map(String::as_str),
            Some("team-a"),
        );
    }

    #[test]
    fn plan_rejects_duplicate_boundary_ids() {
        let mut plan = AiIsolationPlan::new();

        plan.insert(boundary("boundary::duplicate"))
            .expect("first boundary");

        assert!(
            plan.insert(boundary("boundary::duplicate")).is_err()
        );
    }

    #[test]
    fn plan_iteration_is_deterministic() {
        let mut plan = AiIsolationPlan::new();

        plan.insert(boundary("boundary::z"))
            .expect("boundary z");

        plan.insert(boundary("boundary::a"))
            .expect("boundary a");

        plan.insert(boundary("boundary::m"))
            .expect("boundary m");

        let ids: Vec<&str> = plan
            .iter()
            .map(|(id, _)| id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "boundary::a",
                "boundary::m",
                "boundary::z",
            ],
        );
    }

    #[test]
    fn empty_plan_is_valid() {
        let plan = AiIsolationPlan::new();

        assert!(plan.is_empty());
        assert_eq!(plan.len(), 0);
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn plan_supports_dynamically_sized_collections() {
        let mut plan = AiIsolationPlan::new();

        for index in 0..1_000 {
            plan.insert(
                boundary(&format!("boundary::{index}"))
            )
            .expect("unique boundary");
        }

        assert_eq!(plan.len(), 1_000);
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn removal_does_not_affect_other_boundaries() {
        let mut plan = AiIsolationPlan::new();

        plan.insert(boundary("boundary::one"))
            .expect("boundary one");

        plan.insert(boundary("boundary::two"))
            .expect("boundary two");

        let removed = plan.remove(&boundary_id("boundary::one"));

        assert!(removed.is_some());
        assert_eq!(plan.len(), 1);
        assert!(plan.get(&boundary_id("boundary::two")).is_some());
    }
}
