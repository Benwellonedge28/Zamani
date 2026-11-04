
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI Security — Identity
//!
//! Path: `src/ai/security/identity.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Defines AI-domain identity references and security-principal references.
//!
//! This module establishes a consistent identity boundary for AI models,
//! agents, datasets, training runs, pipelines, artifacts and executions.
//!
//! It supports identity references across independent authorities and
//! execution environments without making local numeric identifiers
//! globally unique.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Canonical ownership remains with:
//!
//! - `src/ai/ids.rs`
//! - `src/ai/agents/definition.rs`
//! - `src/ai/provenance.rs`
//! - `src/frontend/semantic/domains/ai.rs`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/model/provenance.rs`
//! - `src/distributed/security/`
//!
//! Source-level identity syntax remains owned by:
//!
//! - `grammar/security/identifiers.g4`
//! - `grammar/security/security.g4`
//!
//! Authorization syntax remains owned by:
//!
//! - `grammar/security/authorization.g4`
//!
//! AI grammar composition remains owned by:
//!
//! - `grammar/ai/ai.g4`
//!
//! ## OWNS
//!
//! - AI entity identity references.
//! - AI security-principal references.
//! - Identity authority namespaces.
//! - External subject references.
//! - Identity category classification.
//! - Identity reference validation.
//! - Identity comparison semantics.
//!
//! ## DOES NOT OWN
//!
//! - Canonical identifier allocation.
//! - Source-language name resolution.
//! - AST identity.
//! - Authentication.
//! - Credential management.
//! - Cryptographic keys.
//! - Digital signatures.
//! - Trust evaluation.
//! - Authorization decisions.
//! - Permission grants.
//! - Capability issuance.
//! - Delegation enforcement.
//! - Identity-provider communication.
//! - Runtime execution identity allocation.
//! - Persistent identity storage.
//! - Distributed identity transport.
//! - Hardware identity discovery.
//!
//! ## IMPORTANT SECURITY INVARIANTS
//!
//! 1. Identity is not authentication.
//! 2. Identity is not trust.
//! 3. Identity is not authorization.
//! 4. Capability is not identity.
//! 5. Intelligence does not increase authority.
//! 6. An AI model cannot grant itself authority.
//! 7. An agent definition is not proof of an executing agent.
//! 8. A local numeric identifier is not globally unique.
//! 9. External identity references do not prove ownership.
//! 10. Simulation identities cannot acquire real execution authority
//!     merely through identity conversion.
//!
//! Authentication and authorization must be independently established
//! by the canonical security and runtime infrastructure.
//!
//! ## SCALABILITY
//!
//! No language-level or AI-specific limits are imposed on:
//!
//! - Models.
//! - Agents.
//! - Datasets.
//! - Training runs.
//! - Pipelines.
//! - Artifacts.
//! - Execution identities.
//! - Identity authorities.
//! - External subjects.
//!
//! No fixed machine, accelerator, memory, worker, node or device count
//! is embedded in this implementation.
//!
//! Identity values are bounded by their underlying canonical identifier
//! representation and available memory.
//!
//! Literal infinity is not representable by a finite computing system.
//!
//! ## POCO-REAF
//!
//! An identity reference describes a logical entity within an authority.
//!
//! It does not encode:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - operating system;
//! - physical device;
//! - quantum processor;
//! - hardware topology;
//! - deployment location.
//!
//! The same logical AI entity can be referenced independently of its
//! eventual execution target.
//!
//! ## INTEGRATION
//!
//! `ids.rs`
//!     -> canonical AI entity identifiers.
//!
//! `agents/definition.rs`
//!     -> AiAgentId.
//!
//! `provenance.rs`
//!     -> AI identity lineage references.
//!
//! `security/authorization.rs`
//!     -> principal references for authorization requests.
//!
//! `security/policies.rs`
//!     -> identity-based policy subjects.
//!
//! `security/audit.rs`
//!     -> identity references in audit records.
//!
//! `distributed/security/`
//!     -> distributed identity authentication and authorization.
//!
//! `runtime/`
//!     -> actual execution identity establishment.
//!
//! `frontend/semantic/domains/ai.rs`
//!     -> semantic validation and source identity adaptation.
//!
//! ## SERIALIZATION
//!
//! Identity references preserve their authority namespace and typed
//! subject representation.
//!
//! Validated string types use checked deserialization so that invalid
//! namespaces and external subject references cannot bypass constructors.
//!
//! ## FREEZE CRITERIA
//!
//! - Existing AI identifier types are reused.
//! - Entity identities and principal identities are distinct.
//! - Identity authority is explicit.
//! - External references are validated.
//! - Deserialization preserves invariants.
//! - Identity does not imply authentication.
//! - Identity does not imply authorization.
//! - No duplicate universal identity allocator exists.
//! - No unsafe Rust.
//! - Serialization round trips are tested.

use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;

use crate::ai::ids::{
    AiAgentId,
    AiArtifactId,
    AiDatasetId,
    AiModelId,
    AiPipelineId,
    AiTrainingRunId,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by AI identity operations.
pub type AiIdentityResult<T> = Result<T, AiIdentityError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by AI identity reference construction and validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiIdentityError {
    /// An identity authority is empty.
    EmptyAuthority,

    /// An authority contains invalid characters or whitespace.
    InvalidAuthority(String),

    /// An external subject identifier is empty.
    EmptySubject,

    /// An external subject identifier is malformed.
    InvalidSubject(String),

    /// A required identity component is missing.
    MissingIdentityComponent,

    /// An identity reference cannot be converted into the requested
    /// identity category.
    IncompatibleIdentityCategory,

    /// An identity reference does not represent an execution principal.
    NotAnExecutionPrincipal,
}

impl fmt::Display for AiIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAuthority => {
                f.write_str("identity authority cannot be empty")
            }

            Self::InvalidAuthority(value) => {
                write!(f, "invalid identity authority: {value}")
            }

            Self::EmptySubject => {
                f.write_str("identity subject cannot be empty")
            }

            Self::InvalidSubject(value) => {
                write!(f, "invalid external identity subject: {value}")
            }

            Self::MissingIdentityComponent => {
                f.write_str("identity reference has a missing component")
            }

            Self::IncompatibleIdentityCategory => {
                f.write_str("incompatible AI identity category")
            }

            Self::NotAnExecutionPrincipal => {
                f.write_str("identity is not an execution principal")
            }
        }
    }
}

impl std::error::Error for AiIdentityError {}

// ============================================================================
// VALIDATED STRING IDENTITIES
// ============================================================================

/// Validates an identity-related symbolic string.
///
/// The validation deliberately does not impose a universal maximum length.
///
/// Whitespace surrounding a value is rejected rather than silently
/// normalized, preserving exact identity semantics.
fn validate_identity_string(
    value: &str,
) -> AiIdentityResult<()> {
    if value.is_empty() {
        return Err(AiIdentityError::EmptySubject);
    }

    if value.trim() != value || value.chars().any(char::is_control) {
        return Err(AiIdentityError::InvalidSubject(
            value.to_owned(),
        ));
    }

    Ok(())
}

// ============================================================================
// IDENTITY AUTHORITY
// ============================================================================

/// Identifies the namespace or authority under which an identity
/// reference is interpreted.
///
/// An authority is a symbolic identity namespace.
///
/// It is not proof that the authority is trusted or authenticated.
///
/// Examples:
///
/// - `zamani::local`
/// - `organization::research`
/// - `deployment::production`
/// - `identity::provider`
/// - `future::authority`
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
    Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct IdentityAuthority(String);

impl IdentityAuthority {
    /// Creates a validated identity authority.
    pub fn new(
        value: impl Into<String>,
    ) -> AiIdentityResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AiIdentityError::EmptyAuthority);
        }

        if value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(
                AiIdentityError::InvalidAuthority(value),
            );
        }

        Ok(Self(value))
    }

    /// Returns the authority namespace.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the authority and returns its string representation.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for IdentityAuthority {
    type Error = AiIdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<IdentityAuthority> for String {
    fn from(value: IdentityAuthority) -> Self {
        value.0
    }
}

impl FromStr for IdentityAuthority {
    type Err = AiIdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl fmt::Display for IdentityAuthority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// EXTERNAL SUBJECT IDENTIFIER
// ============================================================================

/// Opaque external subject identifier.
///
/// This is useful when an execution identity or external principal
/// is owned by another subsystem.
///
/// It deliberately does not interpret the external identifier.
///
/// Authentication and validation against the external authority
/// must happen elsewhere.
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
#[serde(try_from = "String", into = "String")]
pub struct ExternalSubjectId(String);

impl ExternalSubjectId {
    /// Creates a validated external subject identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> AiIdentityResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AiIdentityError::EmptySubject);
        }

        validate_identity_string(&value)?;

        Ok(Self(value))
    }

    /// Returns the opaque external subject identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for ExternalSubjectId {
    type Error = AiIdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ExternalSubjectId> for String {
    fn from(value: ExternalSubjectId) -> Self {
        value.0
    }
}

impl FromStr for ExternalSubjectId {
    type Err = AiIdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl fmt::Display for ExternalSubjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// AI ENTITY CATEGORY
// ============================================================================

/// Categories of AI entities represented by this subsystem.
///
/// This is a classification of entities already owned by their respective
/// AI modules, not a closed catalogue of all possible AI concepts.
///
/// New entity categories can be introduced through the canonical AI
/// identifier architecture without changing unrelated security systems.
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
pub enum AiIdentityCategory {
    Model,
    Agent,
    Dataset,
    TrainingRun,
    Pipeline,
    Artifact,
}

// ============================================================================
// AI ENTITY REFERENCE
// ============================================================================

/// A strongly typed reference to an AI entity.
///
/// The underlying identity types remain owned by `src/ai/ids.rs`.
///
/// These references are not security principals by default.
///
/// For example, a model definition is an AI entity but is not automatically
/// an authenticated actor capable of requesting privileged operations.
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
pub enum AiEntityRef {
    Model(AiModelId),

    Agent(AiAgentId),

    Dataset(AiDatasetId),

    TrainingRun(AiTrainingRunId),

    Pipeline(AiPipelineId),

    Artifact(AiArtifactId),
}

impl AiEntityRef {
    /// Returns the category of this entity.
    pub const fn category(&self) -> AiIdentityCategory {
        match self {
            Self::Model(_) => AiIdentityCategory::Model,

            Self::Agent(_) => AiIdentityCategory::Agent,

            Self::Dataset(_) => AiIdentityCategory::Dataset,

            Self::TrainingRun(_) => AiIdentityCategory::TrainingRun,

            Self::Pipeline(_) => AiIdentityCategory::Pipeline,

            Self::Artifact(_) => AiIdentityCategory::Artifact,
        }
    }

    /// Returns the underlying numeric identifier.
    ///
    /// This is an opaque local identifier.
    ///
    /// It is not globally unique without its authority namespace.
    pub const fn local_id(&self) -> u128 {
        match self {
            Self::Model(id) => id.get(),

            Self::Agent(id) => id.get(),

            Self::Dataset(id) => id.get(),

            Self::TrainingRun(id) => id.get(),

            Self::Pipeline(id) => id.get(),

            Self::Artifact(id) => id.get(),
        }
    }
}

// ============================================================================
// AI IDENTITY REFERENCE
// ============================================================================

/// Complete AI entity identity reference.
///
/// The authority namespace prevents independent local identifier
/// allocators from being incorrectly treated as globally unique.
///
/// Equality includes both:
///
/// - authority;
/// - typed entity reference.
///
/// Thus, identical local numeric IDs belonging to different authorities
/// are distinct identities.
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
pub struct AiIdentityRef {
    authority: IdentityAuthority,

    entity: AiEntityRef,
}

impl AiIdentityRef {
    /// Creates an AI identity reference.
    pub fn new(
        authority: IdentityAuthority,
        entity: AiEntityRef,
    ) -> Self {
        Self {
            authority,
            entity,
        }
    }

    /// Returns the identity authority.
    pub fn authority(&self) -> &IdentityAuthority {
        &self.authority
    }

    /// Returns the referenced AI entity.
    pub fn entity(&self) -> &AiEntityRef {
        &self.entity
    }

    /// Returns the entity category.
    pub fn category(&self) -> AiIdentityCategory {
        self.entity.category()
    }

    /// Returns the underlying local identifier.
    pub fn local_id(&self) -> u128 {
        self.entity.local_id()
    }
}

// ============================================================================
// EXECUTION IDENTITY
// ============================================================================

/// An execution identity reference.
///
/// Actual execution identities are established by runtime infrastructure.
///
/// This type only identifies an execution subject within an external
/// authority namespace.
///
/// It cannot authenticate an execution or prove that an execution exists.
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
pub struct AiExecutionRef {
    authority: IdentityAuthority,

    subject: ExternalSubjectId,
}

impl AiExecutionRef {
    /// Creates an execution identity reference.
    pub fn new(
        authority: IdentityAuthority,
        subject: ExternalSubjectId,
    ) -> Self {
        Self {
            authority,
            subject,
        }
    }

    /// Returns the authority.
    pub fn authority(&self) -> &IdentityAuthority {
        &self.authority
    }

    /// Returns the execution subject identifier.
    pub fn subject(&self) -> &ExternalSubjectId {
        &self.subject
    }
}

// ============================================================================
// SECURITY PRINCIPAL SUBJECT
// ============================================================================

/// AI-domain subjects that may participate in security decisions.
///
/// Only categories that can meaningfully represent an actor are exposed
/// as AI principal subjects.
///
/// A dataset, model artifact or model definition is not automatically
/// a principal.
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
pub enum AiPrincipalSubject {
    Agent(AiAgentId),

    Execution(ExternalSubjectId),

    External(ExternalSubjectId),
}

// ============================================================================
// AI SECURITY PRINCIPAL REFERENCE
// ============================================================================

/// An AI-domain security-principal reference.
///
/// This type identifies a subject that may request an operation.
///
/// It does not establish:
///
/// - authentication;
/// - trust;
/// - authorization;
/// - permissions;
/// - capabilities;
/// - delegation authority.
///
/// The canonical security system must establish those properties
/// independently.
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
pub struct AiPrincipalRef {
    authority: IdentityAuthority,

    subject: AiPrincipalSubject,
}

impl AiPrincipalRef {
    /// Creates a principal reference for an AI agent.
    pub fn agent(
        authority: IdentityAuthority,
        agent: AiAgentId,
    ) -> Self {
        Self {
            authority,
            subject: AiPrincipalSubject::Agent(agent),
        }
    }

    /// Creates a principal reference for an execution.
    pub fn execution(
        execution: AiExecutionRef,
    ) -> Self {
        Self {
            authority: execution.authority,
            subject: AiPrincipalSubject::Execution(
                execution.subject,
            ),
        }
    }

    /// Creates a reference to an externally managed principal.
    pub fn external(
        authority: IdentityAuthority,
        subject: ExternalSubjectId,
    ) -> Self {
        Self {
            authority,
            subject: AiPrincipalSubject::External(subject),
        }
    }

    /// Returns the principal authority.
    pub fn authority(&self) -> &IdentityAuthority {
        &self.authority
    }

    /// Returns the principal subject.
    pub fn subject(&self) -> &AiPrincipalSubject {
        &self.subject
    }

    /// Returns whether the principal represents an execution.
    pub fn is_execution(&self) -> bool {
        matches!(
            self.subject,
            AiPrincipalSubject::Execution(_)
        )
    }

    /// Returns whether the principal represents an AI agent.
    pub fn is_agent(&self) -> bool {
        matches!(
            self.subject,
            AiPrincipalSubject::Agent(_)
        )
    }
}

// ============================================================================
// IDENTITY RELATIONSHIP
// ============================================================================

/// A reference to an AI identity together with its associated principal.
///
/// These concepts remain separate.
///
/// A model may be owned by an agent without becoming that agent.
///
/// An execution may operate on a model without becoming that model.
///
/// The relationship itself does not grant authority.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AiIdentityAssociation {
    entity: AiIdentityRef,

    principal: AiPrincipalRef,
}

impl AiIdentityAssociation {
    /// Creates an explicit identity association.
    pub fn new(
        entity: AiIdentityRef,
        principal: AiPrincipalRef,
    ) -> Self {
        Self {
            entity,
            principal,
        }
    }

    /// Returns the AI entity.
    pub fn entity(&self) -> &AiIdentityRef {
        &self.entity
    }

    /// Returns the associated principal.
    pub fn principal(&self) -> &AiPrincipalRef {
        &self.principal
    }
}

// ============================================================================
// IDENTITY REFERENCE COMPARISON
// ============================================================================

/// Compares two AI identity references.
///
/// Local numeric IDs alone are never sufficient to establish identity.
///
/// Both the authority and the typed entity category must match.
pub fn same_identity(
    left: &AiIdentityRef,
    right: &AiIdentityRef,
) -> bool {
    left == right
}

/// Compares two security-principal references.
///
/// Authority namespaces are included in the comparison.
pub fn same_principal(
    left: &AiPrincipalRef,
    right: &AiPrincipalRef,
) -> bool {
    left == right
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn authority(value: &str) -> IdentityAuthority {
        IdentityAuthority::new(value)
            .expect("valid identity authority")
    }

    #[test]
    fn authority_rejects_empty_values() {
        assert_eq!(
            IdentityAuthority::new(""),
            Err(AiIdentityError::EmptyAuthority)
        );
    }

    #[test]
    fn authority_rejects_surrounding_whitespace() {
        assert!(IdentityAuthority::new(" authority").is_err());

        assert!(IdentityAuthority::new("authority ").is_err());
    }

    #[test]
    fn authority_rejects_control_characters() {
        assert!(
            IdentityAuthority::new("authority\nname").is_err()
        );
    }

    #[test]
    fn external_subject_rejects_empty_values() {
        assert_eq!(
            ExternalSubjectId::new(""),
            Err(AiIdentityError::EmptySubject)
        );
    }

    #[test]
    fn external_subject_rejects_surrounding_whitespace() {
        assert!(ExternalSubjectId::new(" subject").is_err());

        assert!(ExternalSubjectId::new("subject ").is_err());
    }

    #[test]
    fn valid_external_subject_is_preserved() {
        let subject = ExternalSubjectId::new(
            "runtime::execution::session-123",
        )
        .expect("valid external subject");

        assert_eq!(
            subject.as_str(),
            "runtime::execution::session-123"
        );
    }

    #[test]
    fn model_identity_preserves_canonical_identifier() {
        let id = AiModelId::new(42)
            .expect("valid model identifier");

        let identity = AiIdentityRef::new(
            authority("zamani::local"),
            AiEntityRef::Model(id),
        );

        assert_eq!(
            identity.category(),
            AiIdentityCategory::Model
        );

        assert_eq!(identity.local_id(), 42);
    }

    #[test]
    fn agent_identity_uses_canonical_agent_identifier() {
        let id = AiAgentId::new(19)
            .expect("valid agent identifier");

        let identity = AiIdentityRef::new(
            authority("zamani::agents"),
            AiEntityRef::Agent(id),
        );

        assert_eq!(
            identity.category(),
            AiIdentityCategory::Agent
        );
    }

    #[test]
    fn different_authorities_produce_different_identities() {
        let id = AiModelId::new(7)
            .expect("valid model identifier");

        let first = AiIdentityRef::new(
            authority("organization::one"),
            AiEntityRef::Model(id),
        );

        let second = AiIdentityRef::new(
            authority("organization::two"),
            AiEntityRef::Model(id),
        );

        assert_ne!(first, second);

        assert!(!same_identity(&first, &second));
    }

    #[test]
    fn different_entity_categories_are_not_interchangeable() {
        let model_id = AiModelId::new(7)
            .expect("valid model identifier");

        let dataset_id = AiDatasetId::new(7)
            .expect("valid dataset identifier");

        let model = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Model(model_id),
        );

        let dataset = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Dataset(dataset_id),
        );

        assert_ne!(model, dataset);
    }

    #[test]
    fn identical_identity_references_compare_equal() {
        let id = AiModelId::new(100)
            .expect("valid identifier");

        let first = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Model(id),
        );

        let second = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Model(id),
        );

        assert!(same_identity(&first, &second));
    }

    #[test]
    fn execution_identity_is_separate_from_agent_identity() {
        let agent = AiPrincipalRef::agent(
            authority("zamani"),
            AiAgentId::new(10).expect("valid agent"),
        );

        let execution = AiPrincipalRef::execution(
            AiExecutionRef::new(
                authority("runtime"),
                ExternalSubjectId::new("execution-10")
                    .expect("valid execution"),
            ),
        );

        assert!(agent.is_agent());
        assert!(!agent.is_execution());

        assert!(execution.is_execution());
        assert!(!execution.is_agent());

        assert_ne!(agent, execution);
    }

    #[test]
    fn principal_identity_includes_authority() {
        let subject = ExternalSubjectId::new("principal-1")
            .expect("valid subject");

        let first = AiPrincipalRef::external(
            authority("provider::one"),
            subject.clone(),
        );

        let second = AiPrincipalRef::external(
            authority("provider::two"),
            subject,
        );

        assert_ne!(first, second);
    }

    #[test]
    fn identity_association_does_not_merge_entity_and_principal() {
        let entity = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Model(
                AiModelId::new(5).expect("valid model"),
            ),
        );

        let principal = AiPrincipalRef::agent(
            authority("zamani"),
            AiAgentId::new(8).expect("valid agent"),
        );

        let association = AiIdentityAssociation::new(
            entity.clone(),
            principal.clone(),
        );

        assert_eq!(association.entity(), &entity);

        assert_eq!(association.principal(), &principal);

        assert_ne!(
            association.entity().local_id(),
            match association.principal().subject() {
                AiPrincipalSubject::Agent(id) => id.get(),
                _ => 0,
            }
        );
    }

    #[test]
    fn identity_authority_serde_round_trip() {
        let original = authority("organization::research");

        let encoded = serde_json::to_string(&original)
            .expect("serialize authority");

        let decoded: IdentityAuthority =
            serde_json::from_str(&encoded)
                .expect("deserialize authority");

        assert_eq!(original, decoded);
    }

    #[test]
    fn identity_authority_serde_rejects_invalid_values() {
        assert!(
            serde_json::from_str::<IdentityAuthority>("\"\"")
                .is_err()
        );

        assert!(
            serde_json::from_str::<IdentityAuthority>(
                "\" invalid\""
            )
            .is_err()
        );
    }

    #[test]
    fn external_subject_serde_round_trip() {
        let original = ExternalSubjectId::new(
            "distributed::execution::worker",
        )
        .expect("valid subject");

        let encoded = serde_json::to_string(&original)
            .expect("serialize subject");

        let decoded: ExternalSubjectId =
            serde_json::from_str(&encoded)
                .expect("deserialize subject");

        assert_eq!(original, decoded);
    }

    #[test]
    fn external_subject_serde_rejects_empty_values() {
        assert!(
            serde_json::from_str::<ExternalSubjectId>("\"\"")
                .is_err()
        );
    }

    #[test]
    fn complete_ai_identity_serde_round_trip() {
        let original = AiIdentityRef::new(
            authority("zamani::models"),
            AiEntityRef::Model(
                AiModelId::new(123)
                    .expect("valid model"),
            ),
        );

        let encoded = serde_json::to_string(&original)
            .expect("serialize identity");

        let decoded: AiIdentityRef =
            serde_json::from_str(&encoded)
                .expect("deserialize identity");

        assert_eq!(original, decoded);
    }

    #[test]
    fn principal_serde_round_trip() {
        let original = AiPrincipalRef::agent(
            authority("zamani::agents"),
            AiAgentId::new(17)
                .expect("valid agent"),
        );

        let encoded = serde_json::to_string(&original)
            .expect("serialize principal");

        let decoded: AiPrincipalRef =
            serde_json::from_str(&encoded)
                .expect("deserialize principal");

        assert_eq!(original, decoded);
    }

    #[test]
    fn maximum_supported_local_identifier_is_preserved() {
        let id = AiModelId::new(u128::MAX)
            .expect("valid nonzero identifier");

        let identity = AiIdentityRef::new(
            authority("zamani"),
            AiEntityRef::Model(id),
        );

        assert_eq!(identity.local_id(), u128::MAX);
    }

    #[test]
    fn same_principal_requires_matching_authority_and_subject() {
        let subject = ExternalSubjectId::new("agent-execution")
            .expect("valid subject");

        let first = AiPrincipalRef::external(
            authority("authority-a"),
            subject.clone(),
        );

        let second = AiPrincipalRef::external(
            authority("authority-a"),
            subject.clone(),
        );

        let third = AiPrincipalRef::external(
            authority("authority-b"),
            subject,
        );

        assert!(same_principal(&first, &second));

        assert!(!same_principal(&first, &third));
    }
}
