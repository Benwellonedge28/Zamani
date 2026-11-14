
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Security
//!
//! Path: `src/zur/security/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for ZUR's runtime security
//! enforcement boundaries.
//!
//! It exposes the existing security subsystems through one stable,
//! domain-neutral module path:
//!
//! - [`permissions`]: runtime permission enforcement integrated with
//!   the canonical authentication and authorization authorities.
//! - [`policy`]: runtime enforcement of supplied security policies,
//!   including fail-closed authorization and mandatory audit checks.
//! - [`isolation`]: provider-neutral establishment and verification
//!   of execution-isolation sessions.
//!
//! Security enforcement is independent of the computational domain.
//! It can protect operations involving classical, quantum, HDL,
//! AI/tensor, accelerator, hybrid, distributed, networking, and
//! future computational domains.
//!
//! ## Architectural ownership
//!
//! This module owns only the composition and public exposure of
//! ZUR security facilities.
//!
//! It does not own:
//!
//! - Security grammar or source-language syntax.
//! - AST construction or semantic security analysis.
//! - Authentication or principal identity.
//! - Trust establishment or credential storage.
//! - Policy definition, persistence, or evaluation algorithms.
//! - Cryptographic algorithms or key management.
//! - Artifact manifests or compiler artifact formats.
//! - Capability discovery or capability evidence.
//! - Hardware discovery, selection, or allocation.
//! - Process, container, VM, enclave, or operating-system isolation
//!   implementations.
//! - Domain-specific intermediate representations.
//! - Execution scheduling or physical execution.
//! - Audit storage or distributed security protocols.
//!
//! These responsibilities remain with their existing repository
//! authorities.
//!
//! ## Canonical integration authorities
//!
//! Source-level security intent is owned by:
//!
//! - `grammar/security/`
//! - `grammar/security/README.md`
//! - `grammar/security/authorization.g4`
//! - `grammar/security/permissions.g4`
//! - `grammar/security/policies.g4`
//! - `grammar/security/audit.g4`
//! - `grammar/security/security-constraints.g4`
//!
//! The frontend semantic layer resolves and validates that intent.
//!
//! Runtime permission and authorization integration delegates to:
//!
//! - `crate::distributed::security`
//! - `crate::stdlib::security`
//!
//! Runtime operational errors use:
//!
//! - `crate::zur::error`
//!
//! Runtime security policy configuration uses:
//!
//! - `crate::zur::policy`
//!
//! Artifact and generated-code integrity must integrate with the
//! canonical backend/compiler artifact authorities when its dedicated
//! ZUR implementation is added.
//!
//! ## Security boundaries
//!
//! The security subsystems are deliberately separate:
//!
//! 1. Permission enforcement answers whether an authenticated subject
//!    is authorized to perform a requested operation.
//! 2. Policy enforcement validates the canonical authorization result,
//!    its obligations, and required audit confirmation.
//! 3. Isolation establishes and verifies an actual provider-managed
//!    execution-isolation session.
//! 4. Artifact integrity verification must establish whether an
//!    artifact or generated-code object satisfies its integrity and
//!    provenance requirements. It is not implemented by this module.
//!
//! These controls are complementary. One must not be treated as a
//! substitute for another.
//!
//! In particular:
//!
//! - Authentication does not imply authorization.
//! - A permission declaration does not grant permission.
//! - A capability does not grant permission.
//! - A policy permit does not prove that isolation is active.
//! - A registered isolation provider does not prove that isolation
//!   has been established.
//! - An artifact manifest does not, by itself, prove artifact integrity.
//! - An authorization receipt does not replace provider-side security.
//!
//! ## Fail-closed requirements
//!
//! Protected operations must not proceed when required security
//! evidence is absent, invalid, stale, contradictory, or indeterminate.
//!
//! Denial, missing authorization, unresolved obligations, failed
//! mandatory audit, invalid provider evidence, failed isolation,
//! and failed integrity verification must remain failures.
//!
//! No module in this composition may turn an unsupported operation,
//! unavailable provider, or failed security check into success.
//!
//! ## Integration with execution
//!
//! Execution dispatch, JIT, generated-code execution, hybrid execution,
//! distributed execution, and domain providers must invoke the relevant
//! security boundaries immediately before protected operations.
//!
//! The required enforcement sequence depends on the operation, but
//! must include all applicable controls:
//!
//! - Validate the operation and its trusted execution context.
//! - Enforce the applicable authorization policy.
//! - Establish and verify required isolation.
//! - Verify artifact or generated-code integrity where applicable.
//! - Execute only after all mandatory checks succeed.
//! - Preserve audit and execution outcomes.
//! - Release security and isolation resources through their owning
//!   lifecycle contracts.
//!
//! Authorization receipts must not be treated as permanent grants.
//! Providers must enforce their own applicable security boundaries.
//!
//! ## Runtime optionality
//!
//! This module does not initialize security services, create global
//! state, discover providers, or start background tasks.
//!
//! Security facilities are constructed and injected by the ZUR
//! composition root when the selected execution configuration needs
//! them.
//!
//! Runtime-free AOT programs must not acquire a dependency on this
//! module merely because the Zamani language supports security syntax.
//!
//! ## Open-world scalability
//!
//! Security identifiers and provider integrations remain extensible.
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Principals.
//! - Permissions.
//! - Policies.
//! - Actions.
//! - Resources.
//! - Isolation controls.
//! - Security providers.
//! - Devices.
//! - Execution sessions.
//! - Computational domains.
//!
//! Practical capacity is determined by available resources, provider
//! implementations, platform constraints, and explicit operational
//! budgets.
//!
//! Literal infinite computation on finite physical resources is not
//! promised.
//!
//! New computational domains must reuse these security boundaries
//! rather than requiring changes to this composition root.
//!
//! ## Concurrency and safety
//!
//! This module introduces:
//!
//! - No `unsafe` code.
//! - No global mutable state.
//! - No implicit singleton.
//! - No hidden provider registry.
//! - No implicit allow-all policy.
//! - No authorization-result cache.
//! - No hardware-specific assumptions.
//! - No arbitrary resource ceilings.
//!
//! Thread-safety and lifecycle guarantees remain those of the
//! individual injected services and providers.
//!
//! ## Module integration
//!
//! The parent `crate::zur` module must declare:
//!
//! ```ignore
//! pub mod security;
//! ```
//!
//! This file exposes only implementation files that currently exist.
//! When a dedicated `code_integrity.rs` implementation is completed,
//! it must be added here as a separate module and public API.
//!
//! This composition root must not declare missing or placeholder
//! modules merely to suggest that a security feature is implemented.

// ============================================================================
// SECURITY SUBSYSTEMS
// ============================================================================

/// Runtime permission enforcement.
///
/// Integrates ZUR protected operations with the canonical distributed
/// authentication, authorization, and policy-enforcement services.
///
/// This module does not own authentication, identity, policy evaluation,
/// or grant persistence.
pub mod permissions;

/// Provider-neutral execution-isolation contracts and enforcement.
///
/// Establishes and verifies provider-managed isolation sessions.
/// It does not implement operating-system, container, virtual-machine,
/// enclave, or hardware isolation itself.
pub mod isolation;

/// Runtime security policy enforcement.
///
/// Delegates policy evaluation to the canonical authorization service
/// and validates the authorization result, outstanding obligations,
/// and mandatory audit confirmation before protected execution.
pub mod policy;

// ============================================================================
// PUBLIC PERMISSION API
// ============================================================================

/// Open-ended runtime permission identifier.
pub use permissions::PermissionIdentifier;

/// Borrowed evidence of permission for the duration of a protected
/// operation.
pub use permissions::PermissionPermit;

/// ZUR adapter for authorize-before-execute operations.
pub use permissions::PermissionAuthorizer;

/// Result type used by the canonical distributed security integration.
pub use permissions::PermissionResult;

// ============================================================================
// PUBLIC ISOLATION API
// ============================================================================

/// Isolation operation result.
pub use isolation::IsolationResult;

/// Isolation error classification.
pub use isolation::IsolationErrorKind;

/// Structured isolation error.
pub use isolation::IsolationError;

/// Open-ended isolation control, subject, provider, and session identity.
pub use isolation::IsolationIdentifier;

/// Required, preferred, or prohibited isolation directive.
pub use isolation::IsolationDisposition;

/// One requested isolation control and its disposition.
pub use isolation::IsolationDirective;

/// Validated isolation request.
pub use isolation::IsolationRequest;

/// Provider-reported state of an isolation control.
pub use isolation::IsolationControlState;

/// Evidence reported by an isolation provider.
pub use isolation::IsolationEvidence;

/// Provider-managed isolation session contract.
pub use isolation::IsolationSession;

/// Isolation provider contract.
pub use isolation::IsolationProvider;

/// Report of verified isolation requirements and unmet preferences.
pub use isolation::IsolationReport;

/// Owned isolation session lease with explicit cleanup.
pub use isolation::IsolationLease;

/// Manager for an explicitly injected isolation provider.
pub use isolation::IsolationManager;

// ============================================================================
// PUBLIC POLICY ENFORCEMENT API
// ============================================================================

/// Classification of runtime security enforcement failures.
pub use policy::SecurityEnforcementError;

/// Runtime enforcement adapter for the canonical security policy
/// and authorization services.
pub use policy::SecurityPolicyEnforcer;
