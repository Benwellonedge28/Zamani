
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Security Subsystem
//!
//! Path: `src/ai/security/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the canonical composition root for AI-domain security.
//!
//! It integrates AI security identity, authorization, policy attachments,
//! isolation requirements, artifact integrity and security auditing.
//!
//! It provides a stable public interface through which the AI subsystem,
//! compiler, semantic infrastructure, distributed execution infrastructure
//! and runtime can access AI security functionality.
//!
//! # Architectural position
//!
//! ```text
//!                 ZAMANI SOURCE
//!                       |
//!                       v
//!                CANONICAL FRONTEND
//!                       |
//!                       v
//!               AI SEMANTIC ANALYSIS
//!                       |
//!                       v
//!                AI SECURITY INTENT
//!                       |
//!          +------------+------------+
//!          |            |            |
//!          v            v            v
//!       Identity     Policies    Isolation
//!          |            |            |
//!          +------------+------------+
//!                       |
//!                       v
//!                AI AUTHORIZATION
//!                       |
//!                       v
//!             DISTRIBUTED SECURITY
//!                       |
//!                       v
//!              RUNTIME ENFORCEMENT
//!                       |
//!          +------------+------------+
//!          |                         |
//!          v                         v
//!     Artifact Integrity        Security Audit
//!          |                         |
//!          v                         v
//!    Backend Artifact          Canonical Audit Sink
//!
//! ```
//!
//! # Module ownership
//!
//! This composition root owns:
//!
//! - Security module registration.
//! - Public AI security exports.
//! - The stable AI security import boundary.
//! - Security subsystem ownership documentation.
//! - Cross-module architectural integration contracts.
//!
//! It does not own:
//!
//! - Authentication.
//! - Cryptographic identity.
//! - Credential management.
//! - Cryptographic algorithms.
//! - Key management.
//! - Trust establishment.
//! - Generic policy evaluation.
//! - Permission storage.
//! - Authorization enforcement.
//! - Capability discovery.
//! - Resource allocation.
//! - Hardware discovery.
//! - Process isolation.
//! - Operating-system sandbox enforcement.
//! - Container or virtual-machine creation.
//! - Distributed transport.
//! - Scheduling.
//! - Model execution.
//! - General logging infrastructure.
//! - Persistent audit storage.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! # Canonical module responsibilities
//!
//! ## identity
//!
//! Owns AI entity identity references, execution identity references,
//! principal references and identity authority namespaces.
//!
//! Identity is not authentication, trust or authorization.
//!
//! ## authorization
//!
//! Integrates AI operations with canonical distributed authentication,
//! authorization and policy enforcement.
//!
//! Protected AI operations must fail closed.
//!
//! ## policies
//!
//! Owns AI-specific policy attachments and policy scopes.
//!
//! Policy declarations and evaluation remain with the canonical
//! policy infrastructure.
//!
//! ## isolation
//!
//! Describes portable AI isolation boundaries and requirements.
//!
//! Actual isolation enforcement belongs to runtime and security providers.
//!
//! ## integrity
//!
//! Coordinates AI artifact integrity verification through the canonical
//! backend artifact infrastructure.
//!
//! It does not implement another hashing or cryptographic subsystem.
//!
//! ## audit
//!
//! Owns AI-specific security audit events, event correlation,
//! provenance references and explicit audit persistence acknowledgements.
//!
//! General telemetry, tracing and persistent storage remain external.
//!
//! # Dependency ownership
//!
//! The following repository modules remain authoritative:
//!
//! - `crate::ai::ids`
//! - `crate::ai::agents::policies`
//! - `crate::ai::provenance`
//! - `crate::ai::capabilities`
//! - `crate::ai::requirements`
//! - `crate::frontend::semantic`
//! - `crate::distributed::security`
//! - `crate::backend::artifact`
//! - `crate::runtime`
//!
//! This module composes those facilities without replacing them.
//!
//! # Grammar integration
//!
//! Source-level security intent remains owned by:
//!
//! - `grammar/security/security.g4`
//! - `grammar/security/authorization.g4`
//! - `grammar/security/audit.g4`
//! - `grammar/security/sandbox.g4`
//! - `grammar/security/security-constraints.g4`
//! - `grammar/security/identifiers.g4`
//! - `grammar/security/policies.g4`
//! - `grammar/ai/ai.g4`
//!
//! This Rust module does not parse grammar productions or create AST nodes.
//!
//! The canonical integration pathway is:
//!
//! Grammar
//!     -> Canonical frontend AST
//!     -> Semantic validation
//!     -> AI security requirements
//!     -> Security policy and capability analysis
//!     -> Distributed security
//!     -> Runtime enforcement
//!
//! # POCO-REAF
//!
//! AI security descriptions must remain independent of physical execution
//! environments.
//!
//! This subsystem introduces no fixed maximum for:
//!
//! - AI identities.
//! - AI principals.
//! - Models.
//! - Agents.
//! - Datasets.
//! - Training runs.
//! - Pipelines.
//! - Artifacts.
//! - Policies.
//! - Isolation boundaries.
//! - Authorization requests.
//! - Audit events.
//! - Execution targets.
//! - Distributed participants.
//!
//! Actual capacity is determined by available memory, storage,
//! representation limits and explicitly configured operational budgets.
//!
//! Operational budgets must not become language-level semantic ceilings.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! # Security invariants
//!
//! 1. Identity never implies authentication.
//! 2. Authentication never automatically implies authorization.
//! 3. Capability never automatically implies permission.
//! 4. Intelligence never increases authority.
//! 5. An AI model cannot authorize itself.
//! 6. Policy attachment never grants permission.
//! 7. Isolation declarations never establish actual isolation.
//! 8. Integrity verification never establishes authenticity.
//! 9. Missing security providers never imply successful enforcement.
//! 10. Indeterminate authorization never becomes permission.
//! 11. Failed mandatory security obligations prevent execution.
//! 12. Failed audit persistence remains an explicit failure.
//! 13. Security enforcement occurs at the actual protected boundary.
//! 14. Security failures must not be silently discarded.
//!
//! # Integration contract
//!
//! `crate::ai::security::identity`
//!     -> AI identities and principal references.
//!
//! `crate::ai::security::authorization`
//!     -> AI authorization integration.
//!
//! `crate::ai::security::policies`
//!     -> AI policy attachments.
//!
//! `crate::ai::security::isolation`
//!     -> Portable isolation requirements.
//!
//! `crate::ai::security::integrity`
//!     -> Canonical artifact verification.
//!
//! `crate::ai::security::audit`
//!     -> AI security audit recording.
//!
//! `crate::distributed::security`
//!     -> Authentication and authorization enforcement.
//!
//! `crate::backend::artifact`
//!     -> Artifact content integrity.
//!
//! `crate::frontend::semantic`
//!     -> Source-level semantic security validation.
//!
//! `crate::runtime`
//!     -> Enforcement at protected execution boundaries.
//!
//! # Concurrency
//!
//! This composition root creates no global mutable state, hidden task
//! queues, background workers or implicit synchronization.
//!
//! Concurrency and lifecycle management belong to the services that
//! implement the corresponding security responsibilities.
//!
//! # Versioning
//!
//! Public interfaces are exposed through their owning modules.
//!
//! Changes to those interfaces must preserve their documented ownership,
//! security invariants and compatibility contracts.

// ============================================================================
// SECURITY MODULES
// ============================================================================

// AI identity references and security principals.

pub mod identity;

// AI authorization integration.

pub mod authorization;

// AI policy attachments and scopes.

pub mod policies;

// Declarative AI isolation requirements.

pub mod isolation;

// AI artifact integrity verification.

pub mod integrity;

// AI security audit records and persistence integration.

pub mod audit;

// ============================================================================
// PUBLIC IDENTITY API
// ============================================================================

pub use identity::{
    AiEntityRef,
    AiExecutionRef,
    AiIdentityAssociation,
    AiIdentityCategory,
    AiIdentityError,
    AiIdentityRef,
    AiIdentityResult,
    AiPrincipalRef,
    AiPrincipalSubject,
    ExternalSubjectId,
    IdentityAuthority,
};

// ============================================================================
// PUBLIC AUTHORIZATION API
// ============================================================================

pub use authorization::{
    AiAuthorizationAction,
    AiAuthorizationRequest,
    AiAuthorizationResult,
    AiAuthorizationService,
};

// ============================================================================
// PUBLIC POLICY API
// ============================================================================

pub use policies::{
    AiSecurityPolicyBinding,
    AiSecurityPolicyBindingId,
    AiSecurityPolicyError,
    AiSecurityPolicyReference,
    AiSecurityPolicyResult,
    AiSecurityPolicyScope,
    AiSecurityPolicySet,
    AiSecurityPolicySubject,
    AiSecurityPolicyTarget,
};

// ============================================================================
// PUBLIC ISOLATION API
// ============================================================================

pub use isolation::{
    AiIsolationBoundary,
    AiIsolationBoundaryId,
    AiIsolationControlId,
    AiIsolationDisposition,
    AiIsolationError,
    AiIsolationPlan,
    AiIsolationRequirement,
    AiIsolationResult,
    AiIsolationSubject,
};

// ============================================================================
// PUBLIC INTEGRITY API
// ============================================================================

pub use integrity::{
    refresh_artifact_digest,
    verify_artifact,
    AiArtifactIntegrityProvider,
    AiContentDigest,
    AiIntegrityError,
    AiIntegrityEvidence,
    AiIntegrityPolicy,
    AiIntegrityResult,
    AiIntegrityService,
    BackendArtifactIntegrityProvider,
    BACKEND_SHA256_ALGORITHM,
};

// ============================================================================
// PUBLIC AUDIT API
// ============================================================================

pub use audit::{
    AiAuditCategory,
    AiAuditDecision,
    AiAuditDurability,
    AiAuditError,
    AiAuditEvent,
    AiAuditEventId,
    AiAuditFieldName,
    AiAuditFieldValue,
    AiAuditOutcome,
    AiAuditReceipt,
    AiAuditResult,
    AiAuditService,
    AiAuditSink,
    AiAuditSourceRef,
    AiAuditTimestamp,
    AI_AUDIT_SCHEMA_VERSION,
};
