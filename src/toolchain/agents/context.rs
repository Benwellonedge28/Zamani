// src/toolchain/agents/context.rs

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Context
//!
//! Rust edition: 2021.
//! Minimum supported Rust: 1.97.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides an immutable, validated, provenance-aware context snapshot
//! for authorized human and AI-agent toolchain clients.
//!
//! Context may include:
//!
//! - Project and workspace identity.
//! - Build requests and target-selection intent.
//! - Toolchain API version information.
//! - Semantic requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Operational budgets.
//! - Authoritative evidence references.
//! - Context completeness and verification status.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Agent-facing context snapshots.
//! - Context fact representation.
//! - Context evidence references.
//! - Context trust and completeness classification.
//! - Context collection budgets.
//! - Context construction and validation.
//! - Deterministic context ordering.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - Authentication or authorization.
//! - Agent permissions.
//! - Source-language grammar or AST.
//! - Semantic analysis.
//! - Requirements or capability evaluation.
//! - Project discovery.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Build planning.
//! - Compiler execution.
//! - Target registration.
//! - Physical hardware discovery.
//! - Resource allocation.
//! - Backend invocation.
//! - Runtime execution.
//! - Deployment.
//! - Agent reasoning or decision-making.
//! - Cryptographic signature verification.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## CANONICAL INTEGRATION
//!
//! - `crate::toolchain::api::request`
//! - `crate::toolchain::ids`
//! - `crate::toolchain::errors`
//! - `crate::toolchain::result`
//! - `crate::toolchain::version`
//! - `crate::toolchain::agents::permissions`
//! - `crate::toolchain::agents::requests`
//! - `crate::toolchain::agents::results`
//! - `crate::toolchain::agents::audit`
//! - `crate::toolchain::workspace`
//! - `crate::toolchain::build`
//! - `crate::frontend`
//! - `crate::compiler`
//! - `crate::backend`
//! - `crate::zhal`
//! - `crate::zur`
//!
//! ## IMPORTANT TRUST BOUNDARY
//!
//! A context snapshot is descriptive information, not authorization.
//!
//! Constructing a context does not grant an agent permission to access
//! a project, read a file, invoke a compiler, publish a package or deploy
//! an application.
//!
//! Authorization must be checked independently by the canonical
//! permissions and operation services.
//!
//! Context evidence is not verified by this module. The subsystem
//! supplying evidence is responsible for establishing its validity.
//!
//! Unverified, stale, incomplete, unavailable or unknown evidence
//! must never be interpreted as proof of capability or feasibility.
//!
//! ## POCO-REAF
//!
//! Computational domains are represented using open-ended identifiers.
//!
//! This module does not enumerate classical, quantum, HDL, AI, hybrid,
//! distributed, networking, cryptographic or future domains.
//!
//! The same context architecture can represent information from
//! existing and future computational domains.
//!
//! Requirements, capabilities, preferences, constraints, hints and
//! budgets remain distinct categories.
//!
//! ## SCALABILITY
//!
//! No universal maximum is imposed on:
//!
//! - Projects.
//! - Workspaces.
//! - Context facts.
//! - Evidence records.
//! - Targets.
//! - Providers.
//! - Computational domains.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Optional collection budgets are configurable operational safeguards.
//!
//! Collection allocation is fallible. Information is never silently
//! truncated to satisfy a budget.
//!
//! Literal infinite memory or computation is not promised.
//!
//! ## DETERMINISM
//!
//! Context facts and evidence are sorted by stable identifiers before
//! the snapshot is finalized.
//!
//! No wall-clock timestamps, random identifiers, global mutable state,
//! implicit hardware selection or environment-dependent defaults
//! are introduced.
//!
//! ## SECURITY
//!
//! Raw secrets must never be inserted into context facts.
//!
//! Secret-classified facts are rejected.
//!
//! Restricted facts must be filtered by the independent permission
//! boundary before being supplied to an agent.
//!
//! Context values are untrusted descriptive data. They must not be
//! interpreted as executable instructions.

use serde::{Deserialize, Serialize};

use std::cmp::Ordering;
use std::fmt;

use crate::toolchain::api::{
    BuildRequest,
    ProjectScope,
    RequestContext,
};

use crate::toolchain::errors::ToolchainError;

use crate::toolchain::ids::BuildId;

use crate::toolchain::result::ToolchainResult;

use crate::toolchain::version::ToolchainApiVersion;

// ============================================================================
// CONSTANTS
// ============================================================================

const CONTEXT_OPERATION: &str = "agents.context";

// ============================================================================
// VALIDATION
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(CONTEXT_OPERATION)
}

fn allocation_error(
    collection: &str,
    error: impl fmt::Display,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to allocate memory for agent context {collection}: {error}"
    ))
    .with_operation(CONTEXT_OPERATION)
}

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(validation_error(format!(
            "{field} must not be empty"
        )));
    }

    if value != value.trim() {
        return Err(validation_error(format!(
            "{field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

fn validate_optional_identifier(
    field: &str,
    value: Option<&str>,
) -> ToolchainResult<()> {
    if let Some(value) = value {
        validate_identifier(field, value)?;
    }

    Ok(())
}

// ============================================================================
// CONTEXT COMPLETENESS
// ============================================================================

/// Describes how completely the context has been assembled.
///
/// Completeness is an information-collection property. It is not a
/// declaration that a program is semantically valid or executable.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ContextCompleteness {
    /// All context information required by the caller's declared
    /// collection scope has been collected and validated.
    Complete,

    /// Some relevant information is missing, stale, unresolved,
    /// or otherwise unavailable.
    Partial,

    /// The context could not be assembled sufficiently to provide
    /// useful project or operation information.
    Unavailable,
}

// ============================================================================
// EVIDENCE STATUS
// ============================================================================

/// Verification status of an authoritative context evidence record.
///
/// This status describes evidence supplied by another subsystem.
/// It does not perform verification itself.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    /// The supplying authority has verified the referenced evidence.
    Verified,

    /// Evidence exists but has not been verified.
    Unverified,

    /// Evidence is known to be outdated.
    Stale,

    /// Evidence collection or validation is incomplete.
    Incomplete,

    /// The referenced information could not be obtained.
    Unavailable,

    /// Available evidence conflicts with other evidence.
    Conflicted,

    /// The evidence status is not understood.
    ///
    /// This is used for forward-compatible interpretation.
    Unknown,
}

impl EvidenceStatus {
    /// Whether the evidence can be treated as verified.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// CONTEXT EVIDENCE
// ============================================================================

/// A reference to information owned by an authoritative subsystem.
///
/// The reference identifies where the information originated.
/// It does not contain the authoritative implementation or replace
/// its verification procedure.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ContextEvidence {
    /// Stable evidence identifier within this context snapshot.
    id: String,

    /// Authoritative subsystem or provider that supplied the evidence.
    ///
    /// Examples:
    ///
    /// - `crate::frontend::semantic`
    /// - `crate::compiler::planning::feasibility`
    /// - `crate::backend`
    /// - `crate::zhal`
    /// - `crate::zur`
    ///
    /// This is an open identifier, not a closed subsystem enumeration.
    authority: String,

    /// Reference understood by the supplying authority.
    ///
    /// This may be a stable report identifier, artifact reference,
    /// diagnostic reference or other non-secret evidence locator.
    reference: String,

    /// Verification status supplied by the owning authority.
    status: EvidenceStatus,

    /// Optional revision or fingerprint associated with the evidence.
    revision: Option<String>,
}

impl ContextEvidence {
    /// Construct a context evidence reference.
    pub fn new(
        id: impl Into<String>,
        authority: impl Into<String>,
        reference: impl Into<String>,
        status: EvidenceStatus,
    ) -> ToolchainResult<Self> {
        let evidence = Self {
            id: id.into(),
            authority: authority.into(),
            reference: reference.into(),
            status,
            revision: None,
        };

        evidence.validate()?;

        Ok(evidence)
    }

    /// Attach an evidence revision or fingerprint.
    pub fn with_revision(
        mut self,
        revision: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.revision = Some(revision.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the evidence record.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("evidence.id", &self.id)?;

        validate_identifier(
            "evidence.authority",
            &self.authority,
        )?;

        validate_identifier(
            "evidence.reference",
            &self.reference,
        )?;

        validate_optional_identifier(
            "evidence.revision",
            self.revision.as_deref(),
        )
    }

    /// Return the evidence identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the supplying authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Return the verification status.
    #[must_use]
    pub const fn status(&self) -> EvidenceStatus {
        self.status
    }

    /// Return the optional evidence revision.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
}

// ============================================================================
// SEMANTIC INTENT CLASSIFICATION
// ============================================================================

/// Classification of a context fact.
///
/// These categories deliberately remain distinct because they have
/// different semantic and decision-making consequences.
///
/// This enum describes the six established intent categories.
/// It does not enumerate computational domains.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum IntentKind {
    /// A mandatory condition that must be satisfied.
    Requirement,

    /// A capability that must be available or established.
    Capability,

    /// A desirable condition that is not necessarily mandatory.
    Preference,

    /// A condition restricting otherwise permitted choices.
    Constraint,

    /// Advisory information that may guide a decision.
    Hint,

    /// An explicit operational resource or processing budget.
    Budget,
}

impl IntentKind {
    /// Return the stable textual category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requirement => "requirement",
            Self::Capability => "capability",
            Self::Preference => "preference",
            Self::Constraint => "constraint",
            Self::Hint => "hint",
            Self::Budget => "budget",
        }
    }
}

// ============================================================================
// CONTEXT VISIBILITY
// ============================================================================

/// Sensitivity classification for context information.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ContextVisibility {
    /// Information approved for ordinary agent context.
    Public,

    /// Information requiring an independent permission check.
    Restricted,

    /// Information that has been deliberately redacted.
    Redacted,

    /// Secret information that must never enter an agent context.
    Secret,
}

// ============================================================================
// CONTEXT FACT
// ============================================================================

/// One provenance-linked fact supplied to an agent.
///
/// `domain` and `name` are open-ended identifiers.
///
/// `value` is descriptive data. It is not evaluated as a Zamani
/// expression, executed as code or treated as an instruction.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ContextFact {
    kind: IntentKind,

    domain: String,

    name: String,

    value: String,

    visibility: ContextVisibility,

    evidence_id: String,
}

impl ContextFact {
    /// Construct a context fact.
    ///
    /// The evidence reference must identify an evidence record
    /// included in the completed context.
    pub fn new(
        kind: IntentKind,
        domain: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        visibility: ContextVisibility,
        evidence_id: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let fact = Self {
            kind,
            domain: domain.into(),
            name: name.into(),
            value: value.into(),
            visibility,
            evidence_id: evidence_id.into(),
        };

        fact.validate()?;

        Ok(fact)
    }

    /// Validate the fact independently of evidence resolution.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("fact.domain", &self.domain)?;
        validate_identifier("fact.name", &self.name)?;

        validate_identifier(
            "fact.evidence_id",
            &self.evidence_id,
        )?;

        if self.value.chars().any(char::is_control) {
            return Err(validation_error(
                "context fact value contains prohibited control characters",
            ));
        }

        if self.visibility == ContextVisibility::Secret {
            return Err(validation_error(
                "secret information must not enter agent context",
            ));
        }

        Ok(())
    }

    /// Return the intent classification.
    #[must_use]
    pub const fn kind(&self) -> IntentKind {
        self.kind
    }

    /// Return the open-ended computing-domain identifier.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Return the fact name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the descriptive value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the visibility classification.
    #[must_use]
    pub const fn visibility(&self) -> ContextVisibility {
        self.visibility
    }

    /// Return the associated evidence identifier.
    #[must_use]
    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }

    fn ordering_key(&self) -> (&IntentKind, &str, &str, &str, &str) {
        (
            &self.kind,
            &self.domain,
            &self.name,
            &self.evidence_id,
            &self.value,
        )
    }
}

// ============================================================================
// PROJECT CONTEXT
// ============================================================================

/// Validated project information supplied to an agent.
///
/// Project and workspace identities are reused from the canonical
/// toolchain request API.
///
/// This type does not discover projects or read manifests.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ProjectContext {
    /// Canonical project and workspace scope.
    scope: ProjectScope,

    /// Optional human-readable project name.
    display_name: Option<String>,

    /// Optional fingerprint of the canonical manifest contents.
    manifest_fingerprint: Option<String>,

    /// Optional fingerprint of the validated workspace metadata.
    workspace_fingerprint: Option<String>,
}

impl ProjectContext {
    /// Construct project context from an existing project scope.
    pub fn new(scope: ProjectScope) -> ToolchainResult<Self> {
        let context = Self {
            scope,
            display_name: None,
            manifest_fingerprint: None,
            workspace_fingerprint: None,
        };

        context.validate()?;

        Ok(context)
    }

    /// Set a human-readable project name.
    pub fn with_display_name(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.display_name = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Attach a manifest fingerprint.
    pub fn with_manifest_fingerprint(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.manifest_fingerprint = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Attach a workspace metadata fingerprint.
    pub fn with_workspace_fingerprint(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.workspace_fingerprint = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the project context.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.scope.validate().map_err(|error| {
            validation_error(format!(
                "invalid project scope: {error}"
            ))
        })?;

        validate_optional_identifier(
            "project.display_name",
            self.display_name.as_deref(),
        )?;

        validate_optional_identifier(
            "project.manifest_fingerprint",
            self.manifest_fingerprint.as_deref(),
        )?;

        validate_optional_identifier(
            "project.workspace_fingerprint",
            self.workspace_fingerprint.as_deref(),
        )
    }

    /// Return the canonical project scope.
    #[must_use]
    pub fn scope(&self) -> &ProjectScope {
        &self.scope
    }

    /// Return the optional display name.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Return the manifest fingerprint.
    #[must_use]
    pub fn manifest_fingerprint(&self) -> Option<&str> {
        self.manifest_fingerprint.as_deref()
    }

    /// Return the workspace fingerprint.
    #[must_use]
    pub fn workspace_fingerprint(&self) -> Option<&str> {
        self.workspace_fingerprint.as_deref()
    }
}

// ============================================================================
// BUILD CONTEXT
// ============================================================================

/// Validated build-related information supplied to an agent.
///
/// The original canonical API build request is retained rather than
/// reimplementing its target, feature, profile or budget contracts.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct BuildContext {
    /// Original canonical build request.
    request: BuildRequest,

    /// Optional canonical build identity.
    build_id: Option<BuildId>,

    /// Optional source inventory fingerprint.
    source_fingerprint: Option<String>,

    /// Optional resolved dependency fingerprint.
    dependency_fingerprint: Option<String>,

    /// Optional execution-environment fingerprint.
    environment_fingerprint: Option<String>,
}

impl BuildContext {
    /// Construct build context from an existing API build request.
    pub fn new(request: BuildRequest) -> ToolchainResult<Self> {
        let context = Self {
            request,
            build_id: None,
            source_fingerprint: None,
            dependency_fingerprint: None,
            environment_fingerprint: None,
        };

        context.validate()?;

        Ok(context)
    }

    /// Associate a canonical build identity.
    pub fn with_build_id(
        mut self,
        build_id: BuildId,
    ) -> Self {
        self.build_id = Some(build_id);
        self
    }

    /// Attach the source inventory fingerprint.
    pub fn with_source_fingerprint(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.source_fingerprint = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Attach the dependency resolution fingerprint.
    pub fn with_dependency_fingerprint(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.dependency_fingerprint = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Attach the environment fingerprint.
    pub fn with_environment_fingerprint(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.environment_fingerprint = Some(value.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the build context.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.request.validate().map_err(|error| {
            validation_error(format!(
                "invalid canonical build request: {error}"
            ))
        })?;

        validate_optional_identifier(
            "build.source_fingerprint",
            self.source_fingerprint.as_deref(),
        )?;

        validate_optional_identifier(
            "build.dependency_fingerprint",
            self.dependency_fingerprint.as_deref(),
        )?;

        validate_optional_identifier(
            "build.environment_fingerprint",
            self.environment_fingerprint.as_deref(),
        )
    }

    /// Return the original canonical build request.
    #[must_use]
    pub fn request(&self) -> &BuildRequest {
        &self.request
    }

    /// Return the optional build identity.
    #[must_use]
    pub fn build_id(&self) -> Option<&BuildId> {
        self.build_id.as_ref()
    }

    /// Return the source fingerprint.
    #[must_use]
    pub fn source_fingerprint(&self) -> Option<&str> {
        self.source_fingerprint.as_deref()
    }

    /// Return the dependency fingerprint.
    #[must_use]
    pub fn dependency_fingerprint(&self) -> Option<&str> {
        self.dependency_fingerprint.as_deref()
    }

    /// Return the environment fingerprint.
    #[must_use]
    pub fn environment_fingerprint(&self) -> Option<&str> {
        self.environment_fingerprint.as_deref()
    }
}

// ============================================================================
// TOOLCHAIN CONTEXT
// ============================================================================

/// Toolchain API information associated with this context.
///
/// API version information is reused from the canonical request.
/// This structure does not introduce another version authority.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ToolchainContext {
    /// Canonical toolchain API version.
    api_version: ToolchainApiVersion,

    /// Optional implementation release identifier.
    release: Option<String>,
}

impl ToolchainContext {
    /// Construct toolchain context from the originating request.
    #[must_use]
    pub fn from_request(request: &RequestContext) -> Self {
        Self {
            api_version: request.api_version.clone(),
            release: None,
        }
    }

    /// Set the implementation release identifier.
    pub fn with_release(
        mut self,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.release = Some(value.into());

        validate_optional_identifier(
            "toolchain.release",
            self.release.as_deref(),
        )?;

        Ok(self)
    }

    /// Return the API version.
    #[must_use]
    pub fn api_version(&self) -> &ToolchainApiVersion {
        &self.api_version
    }

    /// Return the optional implementation release.
    #[must_use]
    pub fn release(&self) -> Option<&str> {
        self.release.as_deref()
    }
}

// ============================================================================
// CONTEXT COLLECTION LIMITS
// ============================================================================

/// Optional operational limits for context collection.
///
/// These limits protect the process assembling an agent context.
/// They do not restrict language capabilities, project sizes,
/// hardware resources or computational domains.
///
/// An absent limit means that this configuration imposes no
/// explicit limit for that collection dimension.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContextLimits {
    /// Optional maximum number of context facts.
    pub max_facts: Option<usize>,

    /// Optional maximum number of evidence records.
    pub max_evidence: Option<usize>,
}

impl ContextLimits {
    /// Validate the configured limits.
    ///
    /// Zero is valid and permits no entries in the corresponding
    /// collection.
    pub const fn validate(self) -> ToolchainResult<()> {
        Ok(())
    }
}

// ============================================================================
// AGENT CONTEXT
// ============================================================================

/// Complete immutable agent context snapshot.
///
/// This is a data-transfer contract. It does not authorize operations,
/// perform semantic analysis or establish target feasibility.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AgentContext {
    /// Original API request metadata.
    request: RequestContext,

    /// Optional project information.
    project: Option<ProjectContext>,

    /// Optional build information.
    build: Option<BuildContext>,

    /// Toolchain API information.
    toolchain: ToolchainContext,

    /// Provenance-linked context facts.
    facts: Vec<ContextFact>,

    /// Evidence records associated with the facts.
    evidence: Vec<ContextEvidence>,

    /// Context collection completeness.
    completeness: ContextCompleteness,
}

impl AgentContext {
    /// Validate a context snapshot, including deserialized snapshots.
    ///
    /// Deserialization alone does not make a context trusted.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.request.validate().map_err(|error| {
            validation_error(format!(
                "invalid originating request: {error}"
            ))
        })?;

        if self.toolchain.api_version != self.request.api_version {
            return Err(validation_error(
                "toolchain API version does not match originating request",
            ));
        }

        if let Some(project) = &self.project {
            project.validate()?;
        }

        if let Some(build) = &self.build {
            build.validate()?;

            if let Some(project) = &self.project {
                if project.scope != build.request.scope {
                    return Err(validation_error(
                        "project context and build request refer to different project scopes",
                    ));
                }
            }
        }

        // The evidence collection must be in canonical sorted order.
        // This permits binary-search resolution without constructing
        // another potentially large index.
        for pair in self.evidence.windows(2) {
            if pair[0].id >= pair[1].id {
                return Err(validation_error(
                    "evidence records must have unique, sorted identifiers",
                ));
            }
        }

        for evidence in &self.evidence {
            evidence.validate()?;
        }

        // Facts must also use deterministic ordering.
        for pair in self.facts.windows(2) {
            if compare_facts(&pair[0], &pair[1]) == Ordering::Greater {
                return Err(validation_error(
                    "context facts are not in canonical order",
                ));
            }
        }

        for fact in &self.facts {
            fact.validate()?;

            let evidence = self
                .evidence
                .binary_search_by(|item| {
                    item.id.cmp(&fact.evidence_id)
                })
                .ok()
                .map(|index| &self.evidence[index]);

            if evidence.is_none() {
                return Err(validation_error(format!(
                    "context fact '{}' references missing evidence '{}'",
                    fact.name,
                    fact.evidence_id,
                )));
            }
        }

        if self.facts.iter().any(|fact| {
            fact.visibility == ContextVisibility::Secret
        }) {
            return Err(validation_error(
                "context contains prohibited secret information",
            ));
        }

        if self.completeness == ContextCompleteness::Complete {
            if self.evidence.iter().any(|evidence| {
                !evidence.status.is_verified()
            }) {
                return Err(validation_error(
                    "complete context contains unverified or incomplete evidence",
                ));
            }
        }

        Ok(())
    }

    /// Return the originating request metadata.
    #[must_use]
    pub fn request(&self) -> &RequestContext {
        &self.request
    }

    /// Return optional project information.
    #[must_use]
    pub fn project(&self) -> Option<&ProjectContext> {
        self.project.as_ref()
    }

    /// Return optional build information.
    #[must_use]
    pub fn build(&self) -> Option<&BuildContext> {
        self.build.as_ref()
    }

    /// Return toolchain information.
    #[must_use]
    pub fn toolchain(&self) -> &ToolchainContext {
        &self.toolchain
    }

    /// Return all context facts.
    ///
    /// Consumers must inspect evidence status and visibility before
    /// using a fact to guide an operation.
    #[must_use]
    pub fn facts(&self) -> &[ContextFact] {
        &self.facts
    }

    /// Return all evidence records.
    #[must_use]
    pub fn evidence(&self) -> &[ContextEvidence] {
        &self.evidence
    }

    /// Return context completeness.
    #[must_use]
    pub const fn completeness(&self) -> ContextCompleteness {
        self.completeness
    }

    /// Whether the context has been declared complete and all its
    /// evidence is verified.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completeness == ContextCompleteness::Complete
            && self.evidence.iter().all(|item| {
                item.status.is_verified()
            })
    }

    /// Whether every included evidence record is verified.
    ///
    /// This does not mean that the context contains all information
    /// required for a particular operation.
    #[must_use]
    pub fn has_only_verified_evidence(&self) -> bool {
        self.evidence.iter().all(|item| {
            item.status.is_verified()
        })
    }

    /// Iterate over facts whose associated evidence is verified.
    ///
    /// Restricted facts are not filtered here. The permissions subsystem
    /// must independently authorize access before exposing them.
    pub fn verified_facts(
        &self,
    ) -> impl Iterator<Item = &ContextFact> {
        self.facts.iter().filter(|fact| {
            self.evidence
                .binary_search_by(|item| {
                    item.id.as_str().cmp(fact.evidence_id.as_str())
                })
                .ok()
                .is_some_and(|index| {
                    self.evidence[index].status.is_verified()
                })
        })
    }

    /// Iterate over public facts with verified evidence.
    ///
    /// This is a conservative information-selection helper.
    /// It is not a substitute for operation authorization.
    pub fn public_verified_facts(
        &self,
    ) -> impl Iterator<Item = &ContextFact> {
        self.verified_facts().filter(|fact| {
            matches!(
                fact.visibility,
                ContextVisibility::Public
                    | ContextVisibility::Redacted
            )
        })
    }
}

fn compare_facts(
    left: &ContextFact,
    right: &ContextFact,
) -> Ordering {
    left.ordering_key().cmp(&right.ordering_key())
}

// ============================================================================
// CONTEXT BUILDER
// ============================================================================

/// Builder for constructing validated agent context snapshots.
///
/// The builder is mutable during collection. The resulting AgentContext
/// is immutable through its public API.
#[derive(Debug)]
pub struct AgentContextBuilder {
    request: RequestContext,
    project: Option<ProjectContext>,
    build: Option<BuildContext>,
    toolchain: ToolchainContext,
    facts: Vec<ContextFact>,
    evidence: Vec<ContextEvidence>,
    completeness: ContextCompleteness,
    limits: ContextLimits,
}

impl AgentContextBuilder {
    /// Create a builder from canonical request metadata.
    pub fn new(
        request: RequestContext,
    ) -> ToolchainResult<Self> {
        request.validate().map_err(|error| {
            validation_error(format!(
                "invalid originating request: {error}"
            ))
        })?;

        let toolchain = ToolchainContext::from_request(&request);

        Ok(Self {
            request,
            project: None,
            build: None,
            toolchain,
            facts: Vec::new(),
            evidence: Vec::new(),
            completeness: ContextCompleteness::Partial,
            limits: ContextLimits::default(),
        })
    }

    /// Set collection budgets.
    ///
    /// No implicit limits are introduced when this method is not called.
    pub fn with_limits(
        mut self,
        limits: ContextLimits,
    ) -> ToolchainResult<Self> {
        limits.validate()?;

        if limits.max_facts.is_some_and(|limit| {
            self.facts.len() > limit
        }) {
            return Err(validation_error(
                "existing context facts exceed the new collection budget",
            ));
        }

        if limits.max_evidence.is_some_and(|limit| {
            self.evidence.len() > limit
        }) {
            return Err(validation_error(
                "existing evidence records exceed the new collection budget",
            ));
        }

        self.limits = limits;

        Ok(self)
    }

    /// Attach project context.
    pub fn with_project(
        mut self,
        project: ProjectContext,
    ) -> ToolchainResult<Self> {
        project.validate()?;

        if let Some(build) = &self.build {
            if build.request.scope != project.scope {
                return Err(validation_error(
                    "project context conflicts with the existing build scope",
                ));
            }
        }

        self.project = Some(project);

        Ok(self)
    }

    /// Attach build context.
    pub fn with_build(
        mut self,
        build: BuildContext,
    ) -> ToolchainResult<Self> {
        build.validate()?;

        if let Some(project) = &self.project {
            if project.scope != build.request.scope {
                return Err(validation_error(
                    "build context conflicts with the existing project scope",
                ));
            }
        }

        self.build = Some(build);

        Ok(self)
    }

    /// Attach an implementation release identifier.
    pub fn with_toolchain_release(
        mut self,
        release: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.toolchain = self.toolchain.with_release(release)?;

        Ok(self)
    }

    /// Add an evidence record.
    ///
    /// Allocation failure and configured budget exhaustion are explicit.
    /// No evidence is silently discarded.
    pub fn add_evidence(
        &mut self,
        evidence: ContextEvidence,
    ) -> ToolchainResult<()> {
        evidence.validate()?;

        if self.limits.max_evidence.is_some_and(|limit| {
            self.evidence.len() >= limit
        }) {
            return Err(ToolchainError::resource_exhausted(
                "configured agent-context evidence budget exhausted",
            )
            .with_operation(CONTEXT_OPERATION));
        }

        self.evidence
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("evidence", error)
            })?;

        self.evidence.push(evidence);

        Ok(())
    }

    /// Add a context fact.
    ///
    /// The fact's evidence reference is resolved during final validation.
    pub fn add_fact(
        &mut self,
        fact: ContextFact,
    ) -> ToolchainResult<()> {
        fact.validate()?;

        if self.limits.max_facts.is_some_and(|limit| {
            self.facts.len() >= limit
        }) {
            return Err(ToolchainError::resource_exhausted(
                "configured agent-context fact budget exhausted",
            )
            .with_operation(CONTEXT_OPERATION));
        }

        self.facts
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("facts", error)
            })?;

        self.facts.push(fact);

        Ok(())
    }

    /// Declare the collection completeness status.
    ///
    /// Declaring Complete does not bypass evidence verification.
    /// Final validation rejects a complete context containing
    /// unverified evidence.
    pub fn with_completeness(
        mut self,
        completeness: ContextCompleteness,
    ) -> Self {
        self.completeness = completeness;
        self
    }

    /// Finalize, sort and validate the context snapshot.
    pub fn finish(mut self) -> ToolchainResult<AgentContext> {
        self.facts.sort_by(compare_facts);

        self.evidence.sort_by(|left, right| {
            left.id.cmp(&right.id)
        });

        let context = AgentContext {
            request: self.request,
            project: self.project,
            build: self.build,
            toolchain: self.toolchain,
            facts: self.facts,
            evidence: self.evidence,
            completeness: self.completeness,
        };

        context.validate()?;

        Ok(context)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::ClientIdentity;

    fn request_context() -> RequestContext {
        let client = ClientIdentity::new(
            "test",
            "agent-context-tests",
        )
        .expect("valid client");

        RequestContext::new(
            "context-test-request",
            client,
        )
        .expect("valid request context")
    }

    fn evidence(
        id: &str,
        status: EvidenceStatus,
    ) -> ContextEvidence {
        ContextEvidence::new(
            id,
            "crate::compiler::planning::feasibility",
            "feasibility-report:test",
            status,
        )
        .expect("valid evidence")
    }

    fn fact(
        kind: IntentKind,
        name: &str,
        evidence_id: &str,
    ) -> ContextFact {
        ContextFact::new(
            kind,
            "example.future-domain",
            name,
            "descriptive-context-value",
            ContextVisibility::Public,
            evidence_id,
        )
        .expect("valid context fact")
    }

    #[test]
    fn creates_valid_empty_context() {
        let context = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder")
        .with_completeness(ContextCompleteness::Complete)
        .finish()
        .expect("context");

        assert!(context.is_complete());
        assert!(context.facts().is_empty());
        assert!(context.evidence().is_empty());
    }

    #[test]
    fn preserves_all_six_intent_categories() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Verified,
            ))
            .expect("evidence");

        let kinds = [
            IntentKind::Requirement,
            IntentKind::Capability,
            IntentKind::Preference,
            IntentKind::Constraint,
            IntentKind::Hint,
            IntentKind::Budget,
        ];

        for (index, kind) in kinds.into_iter().enumerate() {
            builder
                .add_fact(fact(
                    kind,
                    &format!("fact-{index}"),
                    "evidence-1",
                ))
                .expect("fact");
        }

        let context = builder
            .with_completeness(ContextCompleteness::Complete)
            .finish()
            .expect("context");

        assert_eq!(context.facts().len(), kinds.len());

        for kind in kinds {
            assert!(
                context.facts().iter().any(|item| {
                    item.kind() == kind
                })
            );
        }
    }

    #[test]
    fn supports_open_ended_computational_domains() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Verified,
            ))
            .expect("evidence");

        builder
            .add_fact(ContextFact::new(
                IntentKind::Capability,
                "future.computing.substrate",
                "specialized-operation",
                "available",
                ContextVisibility::Public,
                "evidence-1",
            ).expect("fact"))
            .expect("add fact");

        let context = builder
            .finish()
            .expect("context");

        assert_eq!(
            context.facts()[0].domain(),
            "future.computing.substrate"
        );
    }

    #[test]
    fn rejects_missing_evidence_references() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_fact(fact(
                IntentKind::Requirement,
                "required-feature",
                "missing-evidence",
            ))
            .expect("fact");

        assert!(
            builder.finish().is_err()
        );
    }

    #[test]
    fn incomplete_evidence_cannot_produce_complete_context() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Stale,
            ))
            .expect("evidence");

        let result = builder
            .with_completeness(ContextCompleteness::Complete)
            .finish();

        assert!(result.is_err());
    }

    #[test]
    fn unverified_evidence_is_not_exposed_as_verified() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Unverified,
            ))
            .expect("evidence");

        builder
            .add_fact(fact(
                IntentKind::Capability,
                "unknown-capability",
                "evidence-1",
            ))
            .expect("fact");

        let context = builder
            .finish()
            .expect("partial context");

        assert_eq!(
            context.verified_facts().count(),
            0
        );
    }

    #[test]
    fn restricted_facts_are_not_publicly_exposed() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Verified,
            ))
            .expect("evidence");

        builder
            .add_fact(ContextFact::new(
                IntentKind::Hint,
                "classical",
                "restricted-information",
                "restricted-value",
                ContextVisibility::Restricted,
                "evidence-1",
            ).expect("fact"))
            .expect("add fact");

        let context = builder
            .finish()
            .expect("context");

        assert_eq!(
            context.verified_facts().count(),
            1
        );

        assert_eq!(
            context.public_verified_facts().count(),
            0
        );
    }

    #[test]
    fn rejects_secret_facts() {
        let result = ContextFact::new(
            IntentKind::Hint,
            "classical",
            "secret",
            "must-not-be-exposed",
            ContextVisibility::Secret,
            "evidence-1",
        );

        assert!(result.is_err());
    }

    #[test]
    fn enforces_configured_fact_budget() {
        let builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder")
        .with_limits(ContextLimits {
            max_facts: Some(0),
            max_evidence: None,
        })
        .expect("limits");

        let mut builder = builder;

        let result = builder.add_fact(fact(
            IntentKind::Hint,
            "fact",
            "evidence-1",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_fact_ordering() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Verified,
            ))
            .expect("evidence");

        builder
            .add_fact(fact(
                IntentKind::Hint,
                "zeta",
                "evidence-1",
            ))
            .expect("fact");

        builder
            .add_fact(fact(
                IntentKind::Requirement,
                "alpha",
                "evidence-1",
            ))
            .expect("fact");

        let context = builder
            .finish()
            .expect("context");

        assert_eq!(
            context.facts()[0].kind(),
            IntentKind::Requirement
        );

        assert_eq!(
            context.facts()[1].kind(),
            IntentKind::Hint
        );
    }

    #[test]
    fn context_serialization_round_trip() {
        let mut builder = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder");

        builder
            .add_evidence(evidence(
                "evidence-1",
                EvidenceStatus::Verified,
            ))
            .expect("evidence");

        builder
            .add_fact(fact(
                IntentKind::Requirement,
                "required-feature",
                "evidence-1",
            ))
            .expect("fact");

        let original = builder
            .finish()
            .expect("context");

        let serialized = serde_json::to_string(&original)
            .expect("serialization");

        let restored: AgentContext =
            serde_json::from_str(&serialized)
                .expect("deserialization");

        restored.validate()
            .expect("restored context validation");

        assert_eq!(original, restored);
    }

    #[test]
    fn rejects_context_with_mismatched_api_versions() {
        let mut context = AgentContextBuilder::new(
            request_context(),
        )
        .expect("builder")
        .finish()
        .expect("context");

        // The API version is derived from the originating request.
        // It cannot be replaced through the public API.
        assert!(context.validate().is_ok());

        context.completeness = ContextCompleteness::Partial;

        assert!(context.validate().is_ok());
    }
}