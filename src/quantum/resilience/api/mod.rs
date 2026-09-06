//! Zamani Quantum Resilience — public API boundary.
//!
//! Path: `src/quantum/resilience/api/mod.rs`
//!
//! This module is the composition boundary for the public resilience API.
//! It deliberately contains no orchestration, detection, diagnosis, planning,
//! adaptation, recovery, mitigation, verification, hardware, provider, I/O,
//! or persistence logic.
//!
//! ## Public API
//!
//! The stable API is intentionally small:
//!
//! - [`controller`] — high-level orchestration entry point;
//! - [`request`] — immutable caller requirements and execution scope;
//! - [`response`] — immutable lifecycle outcome envelope;
//! - [`context`] — explicitly injected execution/resilience capabilities.
//!
//! Higher-level callers should normally depend on the re-exported
//! `ResilienceController`, `ResilienceRequest`, `ResilienceResponse`, and
//! `ResilienceContext` contracts rather than importing implementation modules.
//!
//! ## Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir ───────────────────────────────┐
//!      │                                     │
//!      ▼                                     │
//! ResilienceRequest                          │
//!      │                                     │
//!      ▼                                     │
//! ResilienceController ◄── ResilienceContext│
//!      │                                     │
//!      ├── detection                         │
//!      ├── diagnosis                         │
//!      ├── policy                            │
//!      ├── planning                          │
//!      ├── adaptation                        │
//!      ├── recovery                          │
//!      ├── mitigation                        │
//!      └── verification                      │
//!      │                                     │
//!      ▼                                     │
//! ResilienceResponse                         │
//!      │                                     │
//!      ▼                                     │
//! runtime / history / telemetry / caller     │
//!                                            │
//! canonical quantum identities remain       │
//! owned by `quantum::ir::qubit` ─────────────┘
//! ```
//!
//! ## Dependency direction
//!
//! This parent module establishes namespace composition only. The intended
//! dependency direction is:
//!
//! ```text
//! api::request      ──► canonical quantum IR types
//! api::response     ──► canonical quantum IR types + API identity types
//! api::context      ──► resilience capability contracts
//! api::controller   ──► request + response + context + orchestration contracts
//! api::mod          ──► declares/re-exports the four modules only
//! ```
//!
//! The parent module must never become a dependency hub for concrete backend,
//! provider, QEC, routing, scheduling, compiler, optimization, filesystem,
//! network, or thread implementations.
//!
//! ## Canonical identity rule
//!
//! The API does not define a resilience-specific qubit identifier. Whenever a
//! public API needs a logical or physical qubit identity it must use the
//! canonical types owned by `crate::quantum::ir::qubit`:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! In particular, this module must never introduce aliases or replacement
//! structs such as `ResilienceQubitId`, `LogicalQubitId`, or
//! `PhysicalQubitId`.
//!
//! ## Write once, scale everywhere
//!
//! This boundary imposes no machine-size ceiling. It contains no constants or
//! assumptions for qubit counts, device counts, backend counts, operation
//! counts, incident counts, retry counts, fidelity thresholds, or topology
//! sizes.
//!
//! "Infinity" is therefore an architectural property: the API does not cap
//! the size of a valid finite computation. Actual limits are supplied by
//! discovered capabilities, explicit policy, runtime resources, and the
//! physical execution system.
//!
//! Collections in request/response contracts must remain dynamically sized and
//! immutable/shareable where appropriate; callers must not infer an artificial
//! maximum from an implementation detail.
//!
//! ## Determinism
//!
//! The API composition layer performs no I/O and reads no ambient state. It
//! does not access the clock, environment variables, process/thread identity,
//! random generators, global mutable state, or memory addresses.
//!
//! Determinism is therefore inherited from explicitly supplied request and
//! context values. If randomized execution is required, its seed/source must
//! be explicit in the request/context contract and included in provenance by
//! the owning subsystem.
//!
//! ## Security
//!
//! This module does not expose or own credentials, provider tokens, private
//! keys, authentication state, or authorization decisions. Those concerns
//! belong to the appropriate hardware/runtime/security boundary.
//!
//! The public controller must never use API composition as a route around
//! policy, verification, authorization, or provenance requirements.
//!
//! ## Failure semantics
//!
//! Public operations use the canonical resilience error contract. A completed
//! execution is not itself an accepted result: callers must inspect the
//! response decision and verification status supplied by the authoritative
//! verification subsystem.
//!
//! ## Integration contract
//!
//! The four API files are intentionally separated so each can be completed and
//! tested independently:
//!
//! `request.rs`
//! : Defines what the caller asks resilience to protect. It may use canonical
//!   `quantum::ir::qubit::QubitId` for logical scope but must not own placement.
//!
//! `context.rs`
//! : Defines the capabilities/dependencies injected into the controller. It
//!   connects resilience to detection, diagnosis, policy, planning, adaptation,
//!   recovery, mitigation, verification, execution, telemetry, and provenance
//!   without hard-coding concrete implementations here.
//!
//! `response.rs`
//! : Defines the immutable result envelope, including execution/verification
//!   status, decision, affected canonical resources, and artifact references.
//!
//! `controller.rs`
//! : Orchestrates the lifecycle using the preceding contracts. It must not
//!   duplicate subsystem algorithms or contain provider-specific branches.
//!
//! The API parent itself only composes these contracts and therefore should
//! not need modification when a detector, planner, backend adapter, QEC
//! implementation, mitigation strategy, or verifier is replaced.
//!
//! ## Stable namespace policy
//!
//! Module-qualified names remain the canonical compatibility surface:
//!
//! ```text
//! quantum::resilience::api::controller::...
//! quantum::resilience::api::request::...
//! quantum::resilience::api::response::...
//! quantum::resilience::api::context::...
//! ```
//!
//! The selected high-level types are also re-exported from this module for
//! ergonomic runtime integration. Wildcard re-exports are intentionally
//! avoided so future additions cannot create accidental name collisions.
//!
//! ## Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust only
//! - no nightly features
//! - no `unsafe`
//! - no unsafe FFI
//! - no hidden I/O
//! - no hidden concurrency
//! - no global mutable state
//!
//! `#![forbid(unsafe_code)]` is repeated at this module boundary so the API
//! cannot silently acquire unsafe code even if repository-wide lint settings
//! change later.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// API contract modules
// -----------------------------------------------------------------------------

/// High-level resilience lifecycle orchestration boundary.
pub mod controller;

/// Explicit dependency/capability injection boundary for orchestration.
pub mod context;

/// Immutable caller request contract.
pub mod request;

/// Immutable resilience lifecycle response contract.
pub mod response;

// -----------------------------------------------------------------------------
// Stable ergonomic re-exports
// -----------------------------------------------------------------------------

// Re-export only the intentional public boundary. Internal implementation
// details remain reachable through their module-qualified namespaces but are
// not flattened into this namespace.
pub use controller::{
    ResilienceController,
    ResilienceControllerResult,
    ResilienceCycleId,
    ResilienceCycleSummary,
    ResilienceDecision,
    ResiliencePhase,
    RESILIENCE_CONTROLLER_SCHEMA_ID,
    RESILIENCE_CONTROLLER_SCHEMA_VERSION,
};

pub use context::ResilienceContext;

pub use request::{
    AdaptationPermissions,
    RecoveryPermissions,
    ResilienceExecutionMode,
    ResilienceRequest,
    ResilienceRequestId,
    ResilienceScope,
    ResourcePreference,
    SemanticGuarantee,
    RESILIENCE_REQUEST_SCHEMA_ID,
    RESILIENCE_REQUEST_SCHEMA_VERSION,
};

pub use response::{
    ArtifactId,
    DegradationStatus,
    ExecutionStatus,
    ResourceImpact,
    ResilienceActivitySummary,
    ResilienceResponse,
    VerificationStatus,
    RESILIENCE_RESPONSE_SCHEMA_ID,
    RESILIENCE_RESPONSE_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Compile-time API invariants
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_schema_identifiers_are_non_empty() {
        assert!(!RESILIENCE_CONTROLLER_SCHEMA_ID.is_empty());
        assert!(!RESILIENCE_REQUEST_SCHEMA_ID.is_empty());
        assert!(!RESILIENCE_RESPONSE_SCHEMA_ID.is_empty());
    }

    #[test]
    fn schema_versions_are_non_zero() {
        assert!(RESILIENCE_CONTROLLER_SCHEMA_VERSION > 0);
        assert!(RESILIENCE_REQUEST_SCHEMA_VERSION > 0);
        assert!(RESILIENCE_RESPONSE_SCHEMA_VERSION > 0);
    }

    #[test]
    fn lifecycle_names_are_stable() {
        assert_eq!(ResiliencePhase::Observe.as_str(), "observe");
        assert_eq!(ResiliencePhase::Detect.as_str(), "detect");
        assert_eq!(ResiliencePhase::Diagnose.as_str(), "diagnose");
        assert_eq!(ResiliencePhase::Policy.as_str(), "policy");
        assert_eq!(ResiliencePhase::Plan.as_str(), "plan");
        assert_eq!(ResiliencePhase::Adapt.as_str(), "adapt");
        assert_eq!(ResiliencePhase::Recover.as_str(), "recover");
        assert_eq!(ResiliencePhase::Mitigate.as_str(), "mitigate");
        assert_eq!(ResiliencePhase::Verify.as_str(), "verify");
        assert_eq!(ResiliencePhase::Decide.as_str(), "decide");
    }

    #[test]
    fn decisions_have_explicit_acceptance_semantics() {
        assert!(ResilienceDecision::Accept.is_accepted());
        assert!(ResilienceDecision::DegradedAccept.is_accepted());
        assert!(!ResilienceDecision::Repeat.is_accepted());
        assert!(!ResilienceDecision::Escalate.is_accepted());
        assert!(!ResilienceDecision::Reject.is_accepted());
    }
}