//! Zamani Quantum Resilience — Adaptation subsystem.
//!
//! Path:
//!     `src/quantum/resilience/adaptation/mod.rs`
//!
//! This module is the stable composition boundary for resilience adaptation.
//! It exposes the concrete adaptation contracts without duplicating ownership
//! of quantum IR, qubit identity, routing, scheduling, compilation,
//! optimization, QEC, hardware discovery, or backend execution.
//!
//! # Architectural role
//!
//! ```text
//!                         quantum::resilience
//!                                  │
//!                                  ▼
//!                         planning::action
//!                                  │
//!                                  ▼
//!                     adaptation::adapter
//!                                  │
//!             ┌────────────────────┼────────────────────┐
//!             │                    │                    │
//!             ▼                    ▼                    ▼
//!        remapping             rerouting          rescheduling
//!             │                    │                    │
//!             └────────────────────┼────────────────────┘
//!                                  │
//!                    ┌─────────────┼─────────────┐
//!                    ▼             ▼             ▼
//!              recompilation  reoptimization  qec_adaptation
//!                    │             │             │
//!                    └─────────────┼─────────────┘
//!                                  ▼
//!                         backend_selection
//! ```
//!
//! The adaptation subsystem converts an already-authorized resilience action
//! into a request for an authoritative quantum subsystem. It does not become
//! a second implementation of those subsystems.
//!
//! # Ownership boundaries
//!
//! The following ownership rules are normative:
//!
//! * Canonical quantum semantics are owned by `crate::quantum::ir`.
//! * Canonical qubit identity is owned by
//!   `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}` where applicable.
//! * Logical/physical mappings are owned by the canonical IR resource mapping
//!   layer; resilience adaptation consumes them rather than defining another
//!   mapping ontology.
//! * Routing algorithms are owned by `crate::quantum::routing`.
//! * Scheduling algorithms are owned by `crate::quantum::scheduling`.
//! * Compilation is owned by the compiler/IR pipeline.
//! * Optimization is owned by `crate::quantum::optimization`.
//! * QEC algorithms are owned by the QEC subsystem.
//! * Hardware capabilities, topology, calibration, execution and provider
//!   integration are owned by the hardware HAL.
//! * Resilience planning owns the decision to request an adaptation; this
//!   module owns only the adaptation contracts and implementations.
//!
//! Consequently, this module must never define a competing `QubitId`,
//! `PhysicalQubitId`, topology model, scheduler, router, compiler, optimizer,
//! decoder, provider client, or hardware limit.
//!
//! # Write once, scale everywhere
//!
//! No module in this boundary introduces an architectural upper bound on:
//!
//! * logical qubits;
//! * physical qubits;
//! * circuit operations;
//! * circuit depth;
//! * devices;
//! * backends;
//! * topology size;
//! * distributed execution resources.
//!
//! Concrete bounds come only from discovered capabilities, execution resources,
//! caller policy, safety constraints and operating-system/process limits.
//! "Infinity" therefore means that this subsystem adds no artificial finite
//! machine-size ceiling; every actual execution is bounded by the resources
//! that exist for that execution.
//!
//! Sparse/dynamic resource identity must be preserved. Code that deals with
//! qubit identities must use the canonical `quantum::ir::qubit` types rather
//! than indexing by an assumed machine size.
//!
//! # Transactional lifecycle
//!
//! Adaptation implementations follow the lifecycle defined by `adapter.rs`:
//!
//! ```text
//! request
//!   │
//!   ▼
//! preflight
//!   │
//!   ▼
//! prepare ─────────► immutable candidate
//!   │                       │
//!   │                       ▼
//!   │                    commit
//!   │                       │
//!   │                       ▼
//!   └──────────────────► verify
//! ```
//!
//! A prepared candidate is never equivalent to a committed transformation.
//! This distinction is required for stale-state detection, semantic
//! verification, rollback, deterministic replay and distributed execution.
//!
//! # Determinism
//!
//! Composition order and public exports are static and deterministic. Runtime
//! adapter selection remains the responsibility of the adapter/registry
//! contracts. This module contains no hidden mutable state and no provider-
//! specific selection logic.
//!
//! # Security
//!
//! This module is not an authorization boundary. Policy, capability,
//! feasibility and security authorization must be established before a
//! mutating adaptation is committed. Concrete adapters must not silently
//! override those decisions.
//!
//! # Rust compatibility
//!
//! * Rust 1.97 / 1.97.1
//! * Rust 2021
//! * stable Rust only
//! * no nightly features
//! * safe Rust only
//! * no `unsafe`
//!
//! # Integration contract
//!
//! `quantum::resilience::adaptation::adapter` is the common contract. Concrete
//! modules implement or consume that contract and integrate with their
//! authoritative subsystem through explicit dependency injection or existing
//! repository interfaces. Adding a new adaptation implementation requires
//! adding its module declaration here; it does not require changing the
//! canonical quantum IR or inventing a new resilience-wide qubit type.
//!
//! The module declarations intentionally do not glob-re-export every concrete
//! type. Several concrete adaptation implementations have domain-local value
//! objects with similar names. Keeping them namespaced prevents accidental
//! type collisions and preserves clear ownership. The common adapter contract
//! is re-exported explicitly for ergonomic use by planning/recovery integration.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

// ============================================================================
// Adaptation contracts and implementations
// ============================================================================

/// Provider-independent adaptation adapter contract, transactional lifecycle,
/// request/result types, adapter identity/version/capabilities and common
/// validation helpers.
pub mod adapter;

/// Logical-to-physical remapping adaptation.
pub mod remapping;

/// Topology-aware rerouting adaptation.
pub mod rerouting;

/// Schedule reconstruction adaptation.
pub mod rescheduling;

/// Recompilation adaptation for changed execution targets/capabilities.
pub mod recompilation;

/// Reoptimization adaptation for changed execution conditions.
pub mod reoptimization;

/// QEC configuration adaptation.
pub mod qec_adaptation;

/// Backend/device migration and target-selection adaptation.
pub mod backend_selection;

// ============================================================================
// Stable public contract re-exports
// ============================================================================

// Keep the common adapter contract directly available as:
//
// `quantum::resilience::adaptation::{...}`
//
// Do not glob-re-export concrete modules: their domain-local types intentionally
// remain namespaced and may legitimately share conceptual names.
pub use adapter::{
    adaptation_failed,
    semantic_incompatibility,
    stale_request,
    unsupported_operation,
    AdaptationAdapter,
    AdaptationAdapterHandle,
    AdaptationAdapterSet,
    AdaptationCandidate,
    AdaptationCapabilities,
    AdaptationPhase,
    AdaptationRequest,
    AdaptationResult,
    AdaptationStatus,
    AdapterId,
    AdapterOperation,
    AdapterVersion,
    ExecutionGeneration,
    SemanticRevision,
    ADAPTATION_ADAPTER_SCHEMA_ID,
    ADAPTATION_ADAPTER_SCHEMA_VERSION,
};

// ============================================================================
// Integration metadata
// ============================================================================

/// Stable schema identifier for the adaptation composition boundary.
pub const ADAPTATION_MODULE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation";

/// Stable schema version for the adaptation composition boundary.
///
/// This version belongs to the module composition contract and is independent
/// from the individual adapter contract version in `adapter.rs`.
pub const ADAPTATION_MODULE_SCHEMA_VERSION: u16 = 1;

/// Returns the stable schema identity of the adaptation composition boundary.
///
/// This function is intentionally:
///
/// * allocation-free;
/// * deterministic;
/// * side-effect-free;
/// * independent of hardware size;
/// * independent of backend/provider;
/// * independent of runtime global state.
#[must_use]
pub const fn schema_identity() -> (&'static str, u16) {
    (
        ADAPTATION_MODULE_SCHEMA_ID,
        ADAPTATION_MODULE_SCHEMA_VERSION,
    )
}

// ============================================================================
// Integration invariants
// ============================================================================
//
// The following rules are deliberately documented at the composition boundary
// so future implementations do not accidentally create competing abstractions:
//
// 1. planning/action.rs owns RecoveryAction and ActionKind.
//
// 2. adapter.rs owns AdaptationAdapter, AdaptationRequest,
//    AdaptationCandidate, AdaptationResult and adapter lifecycle semantics.
//
// 3. remapping.rs owns remapping-specific validation and transformation
//    contracts. Canonical qubit identity remains:
//        crate::quantum::ir::qubit::QubitId
//        crate::quantum::ir::qubit::PhysicalQubitId
//
// 4. rerouting.rs requests routing behavior; it does not become a routing
//    algorithm.
//
// 5. rescheduling.rs requests schedule reconstruction; it does not become a
//    second scheduler.
//
// 6. recompilation.rs requests compilation; it does not become a compiler.
//
// 7. reoptimization.rs requests canonical optimization; it does not duplicate
//    optimization passes.
//
// 8. qec_adaptation.rs requests QEC configuration changes; it does not
//    implement QEC decoders/codes itself.
//
// 9. backend_selection.rs consumes backend/device capability abstractions and
//    does not hard-code providers or device sizes.
//
// 10. No module here may assume that a machine has a particular number of
//     qubits, couplings, execution slots, devices, or backends.
//
// 11. No module here may manufacture a replacement QubitId abstraction.
//
// 12. No module here may bypass policy, feasibility, authorization,
//     verification, provenance, or execution-state checks.
//
// 13. A successful preparation is not a successful commit.
//
// 14. A successful commit is not automatically a semantically accepted
//     computation. Verification remains downstream and mandatory.
//
// 15. Concrete adapters must remain replaceable without changing the canonical
//     quantum program.
//
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            schema_identity(),
            (
                "zamani.quantum.resilience.adaptation",
                1,
            )
        );
    }

    #[test]
    fn common_adapter_contract_is_reexported() {
        assert_eq!(
            ADAPTATION_ADAPTER_SCHEMA_ID,
            "zamani.quantum.resilience.adaptation.adapter"
        );

        assert_eq!(ADAPTATION_ADAPTER_SCHEMA_VERSION, 1);
    }

    #[test]
    fn module_schema_is_independent_from_adapter_schema() {
        assert_ne!(
            ADAPTATION_MODULE_SCHEMA_ID,
            ADAPTATION_ADAPTER_SCHEMA_ID
        );
    }
}