
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution
//!
//! Path: `src/zur/execution/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! This module is the public composition root for ZUR execution.
//!
//! It exposes the independently implemented execution components that
//! validate artifacts, load executable artifacts, manage execution
//! sessions and instances, dispatch validated requests, coordinate
//! execution, handle cancellation, and report actual execution outcomes.
//!
//! This module composes those components. It does not implement another
//! execution engine or introduce another execution data model.
//!
//! ## Authoritative responsibilities
//!
//! The child modules retain their individual ownership:
//!
//! - `artifact`: runtime validation of canonical compiler artifacts.
//! - `loader`: loading validated artifacts through explicit adapters.
//! - `session`: session lifecycle, cancellation and retained references.
//! - `instance`: ownership and observation of accepted loaded executions.
//! - `result`: normalized execution outcomes and recovery reporting.
//! - `cancellation`: cooperative cancellation coordination.
//! - `dispatch`: validated hand-off to an explicit execution authority.
//! - `executor`: execution submission, observation, monitoring and collection.
//!
//! ## Existing architectural authorities
//!
//! This module integrates with, but does not replace:
//!
//! - `zur::contracts::execution`:
//!   execution requests, handles, observations, lifecycle states,
//!   cancellation dispositions and the `ExecutionEngine` contract.
//!
//! - `zur::contracts::abi`:
//!   runtime ABI and entry-point contracts.
//!
//! - `zur::contracts::provider`:
//!   provider integration contracts.
//!
//! - `zur::contracts::service`:
//!   optional runtime service contracts.
//!
//! - `zur::error`:
//!   canonical `ZurError` and `ZurResult`.
//!
//! - `zur::ids`:
//!   runtime-owned execution and session identities.
//!
//! - `zur::version`:
//!   runtime ABI and service-contract compatibility.
//!
//! - `zur::core`:
//!   runtime context and service lifecycle infrastructure.
//!
//! - `stdlib::context`:
//!   operation contexts and cooperative cancellation.
//!
//! - `compiler::artifacts` and `backend`:
//!   canonical compiler artifacts, manifests and artifact metadata.
//!
//! - `compiler`, `hybrid`, `distributed`, `quantum`, HDL and AI:
//!   their respective planning, representation and domain authorities.
//!
//! ## Ownership boundaries
//!
//! This module does not:
//!
//! - Parse execution grammar.
//! - Interpret source-language semantics or AST nodes.
//! - Define requirements, capabilities, preferences, constraints or hints.
//! - Construct or optimize compiler execution plans.
//! - Define a competing intermediate representation.
//! - Construct compiler artifacts or artifact manifests.
//! - Register or discover physical devices.
//! - Select hardware or backend implementations.
//! - Perform resource feasibility analysis or resource reservation.
//! - Implement provider-specific execution.
//! - Implement hybrid or distributed planning.
//! - Implement domain-specific quantum, HDL, AI or classical computation.
//! - Claim execution success based only on submission or cancellation request.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Execution invariants
//!
//! 1. Artifact validation is distinct from artifact loading.
//! 2. Artifact loading is distinct from execution submission.
//! 3. Submission is distinct from execution completion.
//! 4. A cancellation request is distinct from confirmed cancellation.
//! 5. Provider registration is distinct from provider availability.
//! 6. Capability evidence is distinct from resource reservation.
//! 7. A partial result is never promoted to full success.
//! 8. An incomplete or unknown outcome is never promoted to success.
//! 9. Resource ownership remains with the component that acquired it.
//! 10. All execution outcomes must be grounded in authoritative observations.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Execution count.
//! - Session count.
//! - Artifact count.
//! - Result references.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Tasks.
//! - Distributed participants.
//! - Resource quantities.
//!
//! The child modules and their providers operate subject to actual
//! address-space, memory, storage, platform, policy and physical-resource
//! availability. Literal infinite physical computation is not promised.
//!
//! New computational domains integrate through the existing provider,
//! artifact, ABI and execution contracts. Adding a domain must not require
//! changing this composition root unless a genuinely new shared execution
//! contract is introduced.
//!
//! ## Grammar boundary
//!
//! `grammar/execution/` defines execution-language constructs and intent.
//! It does not define this Rust runtime implementation.
//!
//! The frontend and semantic authorities validate source intent; compiler
//! and domain authorities produce compatible executable artifacts and
//! validated plans; ZUR consumes those artifacts and requests through
//! the contracts exposed by this module.
//!
//! No grammar parser, AST dependency or grammar-specific domain enum
//! belongs in this module.
//!
//! ## Safety
//!
//! Safe Rust only.
//! No `unsafe`.
//! No raw-pointer execution API.
//! No global mutable execution state.
//! No fabricated execution success.

// ============================================================================
// EXECUTION COMPONENTS
// ============================================================================

/// Runtime artifact validation against canonical compiler artifacts,
/// manifests, ABI compatibility and required runtime services.
pub mod artifact;

/// Cooperative cancellation coordination for execution operations,
/// sessions and nested runtime work.
pub mod cancellation;

/// Validated dispatch hand-off to an explicitly supplied execution
/// authority.
pub mod dispatch;

/// Submission, observation, monitoring, cancellation and collection
/// through the canonical execution-engine contract.
pub mod executor;

/// Ownership and observation of an accepted, loaded execution.
pub mod instance;

/// Loading of previously validated executable artifacts through
/// explicit artifact-format adapters.
pub mod loader;

/// Normalized execution outcomes, partial results, failures and
/// recovery-status reporting.
pub mod result;

/// Isolated execution-session lifecycle, cancellation and retained
/// service/resource references.
pub mod session;

// ============================================================================
// ARTIFACT VALIDATION API
// ============================================================================

pub use artifact::{
    contains_artifact,
    resolve_artifact_dependencies,
    validate_executable_artifact,
    ArtifactLoadPolicy,
    ArtifactPayloadResolver,
    ArtifactValidationReport,
    RuntimeServiceResolver,
    VerifiedArtifact,
    ARTIFACT_VALIDATION_CONTRACT_ID,
    ARTIFACT_VALIDATION_CONTRACT_MAJOR,
    ARTIFACT_VALIDATION_CONTRACT_MINOR,
    ARTIFACT_VALIDATION_CONTRACT_PATCH,
};

// ============================================================================
// CANCELLATION API
// ============================================================================

pub use cancellation::{
    CancellationRequestReport,
    ExecutionCancellationController,
    ExecutionCancellationFuture,
};

// ============================================================================
// DISPATCH API
// ============================================================================

pub use dispatch::{
    DispatchCoordinator,
    DispatchPlanAuthority,
    DispatchPlanEvidence,
    DispatchReceipt,
    ExecutionAuthority,
    ExecutionAuthorityId,
    COMPONENT as DISPATCH_COMPONENT,
    DISPATCH_CONTRACT_ID,
    DISPATCH_CONTRACT_MAJOR,
    DISPATCH_CONTRACT_MINOR,
    DISPATCH_CONTRACT_PATCH,
};

// ============================================================================
// EXECUTOR API
// ============================================================================

pub use executor::{
    ExecutionExecutor,
    ExecutionOutcome,
    ExecutionWaiter,
};

// ============================================================================
// INSTANCE API
// ============================================================================

pub use instance::{
    ExecutionInstance,
    ExecutionInstanceSnapshot,
};

// ============================================================================
// LOADER API
// ============================================================================

pub use loader::{
    ArtifactFormatAdapter,
    ArtifactLoadRequest,
    ArtifactLoader,
    LoadedArtifact,
    LoadedArtifactHandle,
};

// ============================================================================
// RESULT API
// ============================================================================

pub use result::{
    ExecutionResult,
    ExecutionResultStatus,
    RecoveryReport,
    RecoveryStatus,
};

// ============================================================================
// SESSION API
// ============================================================================

pub use session::{
    DiagnosticLevel,
    ExecutionSession,
    ResourceAccountingReference,
    ServiceLeaseReference,
    SessionCancellation,
    SessionDiagnostic,
    SessionSnapshot,
    SessionStatus,
};
