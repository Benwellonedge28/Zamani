//! Zamani Quantum Resilience — Registry Boundary
//!
//! Path:
//!     src/quantum/resilience/registry/mod.rs
//!
//! This module is the stable composition boundary for all resilience
//! registries. It wires together detector, mitigation-strategy,
//! recovery-implementation, and backend-adapter registries without implementing
//! registry policy, quantum algorithms, hardware execution, or recovery logic.
//!
//! # Purpose
//!
//! The registry layer answers one question:
//!
//! > Which explicitly registered implementation is available under a stable,
//! > deterministic identity in this resilience execution context?
//!
//! It does NOT decide:
//!
//! - which implementation is best;
//! - which backend should be selected;
//! - whether an action is authorized;
//! - whether a recovery plan is feasible;
//! - whether a result is correct;
//! - how routing or scheduling is performed;
//! - how QEC is implemented;
//! - how mitigation mathematics is performed;
//! - how provider credentials are obtained;
//! - how dynamic plugins are loaded.
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! # Registry composition
//!
//! ```text
//! crate::quantum::resilience::registry
//! │
//! ├── detector.rs
//! │     └── DetectorRegistry
//! │
//! ├── strategy.rs
//! │     └── StrategyRegistry
//! │
//! ├── recovery.rs
//! │     └── RecoveryRegistry
//! │
//! └── backend.rs
//!       └── BackendRegistry
//! ```
//!
//! The four registries intentionally have different implementation contracts:
//!
//! ```text
//! DetectorRegistry
//!     detector-local mutable state
//!     Box<dyn DetectorObject>
//!
//! StrategyRegistry
//!     shareable strategy implementations
//!     Arc<dyn MitigationStrategy>
//!
//! RecoveryRegistry
//!     executable recovery implementations
//!     Box<dyn RecoveryImplementation>
//!
//! BackendRegistry
//!     shareable backend adapters
//!     Arc<dyn QuantumBackendAdapter>
//! ```
//!
//! `registry/mod.rs` does not homogenize these contracts. Doing so would weaken
//! the ownership, mutability, and concurrency guarantees established by their
//! owning subsystems.
//!
//! # Architectural ownership
//!
//! ```text
//! detection::detector
//!     owns detector semantics
//! registry::detector
//!     owns detector registration/discovery
//!
//! mitigation::strategy
//!     owns mitigation strategy semantics
//! registry::strategy
//!     owns strategy registration/discovery
//!
//! recovery::* / planning::action
//!     own recovery action semantics and implementations
//! registry::recovery
//!     owns recovery implementation registration/discovery
//!
//! hardware::backend_trait
//!     owns executable backend-adapter semantics
//! hardware::*
//!     owns target/capability/device semantics
//! registry::backend
//!     owns resilience-facing adapter registration/discovery
//! ```
//!
//! The registry layer therefore remains a catalogue boundary rather than a
//! second implementation of any quantum subsystem.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical subsystem contracts
//!         │
//!         ▼
//! registry implementations
//!         │
//!         ▼
//! resilience planners/controllers/executors
//!         │
//!         ▼
//! verification / telemetry / history
//! ```
//!
//! The registry boundary must not become a reverse dependency of canonical IR.
//!
//! In particular:
//!
//! ```text
//! quantum::ir              ─X─► resilience::registry
//! quantum::ir::qubit       ─X─► registry implementation state
//! hardware canonical model ─X─► registry policy
//! ZQN                      ─X─► registry storage
//! ```
//!
//! The registry consumes established contracts; it does not redefine them.
//!
//! # Canonical qubit identity
//!
//! This module deliberately does not define or re-export a qubit identifier.
//! Registry keys identify implementations, not quantum resources.
//!
//! Any concrete detector, strategy, recovery implementation, or backend
//! integration that needs logical or physical qubit identity must use the
//! canonical types owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular, new code must not introduce a resilience-local `QubitId`.
//!
//! `registry/mod.rs` remains qubit-agnostic so the same registry boundary works
//! for one qubit, a small QPU, a very large QPU, a logical fault-tolerant
//! machine, or a distributed heterogeneous quantum system.
//!
//! # Write once, scale everywhere
//!
//! No registry in this module imposes an artificial limit on:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - detectors;
//! - strategies;
//! - recovery implementations;
//! - adapter implementations;
//! - observations;
//! - execution width;
//! - topology size.
//!
//! The concrete registries use dynamically sized collections. Their effective
//! capacity is therefore determined by available memory/resources and explicit
//! application policy, not by a semantic machine-size constant.
//!
//! "Infinity" means that this module contributes no artificial finite quantum
//! machine-size ceiling. It does not mean physical memory, CPU time, provider
//! capacity, or execution budgets are infinite.
//!
//! # Determinism
//!
//! The concrete registries use ordered maps and stable implementation identity.
//! This module does not add nondeterministic ordering.
//!
//! Registry enumeration must never depend on:
//!
//! - hash-map iteration order;
//! - allocation addresses;
//! - thread scheduling;
//! - process identifiers;
//! - wall-clock time;
//! - environment variables;
//! - implicit random identifiers.
//!
//! A registry does not silently select the newest implementation. Version and
//! compatibility decisions remain explicit responsibilities of policy and
//! planning layers.
//!
//! # Explicit ownership and no global state
//!
//! These are ordinary owner-managed registry values. This module intentionally
//! creates no process-global singleton and no hidden mutable registry.
//!
//! There is no:
//!
//! ```text
//! static REGISTRY
//! OnceLock<...>
//! lazy_static!
//! thread_local!
//! ```
//!
//! A runtime, controller, simulator, test, or distributed worker creates and
//! owns the registry instance it needs. This provides isolation and makes the
//! registry suitable for deterministic replay and multi-context execution.
//!
//! # Concurrency
//!
//! Concurrency semantics remain owned by each registry implementation.
//!
//! - detector registry preserves the existing detector object's mutability;
//! - strategy registry uses `Arc` because its strategy contract is shareable;
//! - recovery registry preserves its mutable implementation contract;
//! - backend registry uses `Arc` because backend adapters are shareable.
//!
//! `registry/mod.rs` does not wrap these registries in a mutex or `RwLock`.
//!
//! Applications that need concurrent registry mutation should synchronize at
//! their ownership boundary. This avoids hidden global contention and permits
//! sharding independent registries across execution domains.
//!
//! # Selection is not registration
//!
//! A critical invariant is:
//!
//! ```text
//! registry     = discover
//! policy       = constrain / authorize
//! feasibility  = prove possible
//! planner      = choose
//! executor     = execute
//! verifier     = accept / reject
//! ```
//!
//! Consequently this module intentionally exposes no `select_best()` operation
//! and no provider-specific preference order.
//!
//! # Versioning
//!
//! Each registry owns its own stable schema identifier/version because the
//! observable contract of detector registration is not the same as that of
//! backend registration, mitigation strategy registration, or recovery
//! registration.
//!
//! This module deliberately does not invent a second registry-wide semantic
//! version for those child contracts.
//!
//! A composition-boundary version is provided below only to identify changes to
//! this module's public module tree and stable re-exports.
//!
//! # Serialization
//!
//! Executable trait objects are not serialized by this boundary.
//!
//! Serialization may persist stable metadata such as:
//!
//! - registry schema ID/version;
//! - implementation identity;
//! - implementation version;
//! - backend identity;
//! - strategy identity;
//! - recovery action identity.
//!
//! Reconstructing executable implementations must remain an explicit factory,
//! plugin, or dependency-injection operation controlled by the appropriate
//! security and compatibility boundary.
//!
//! # Plugin boundary
//!
//! This module is compatible with future plugin registration but is not itself
//! a dynamic-library loader.
//!
//! A plugin system must establish, outside this file:
//!
//! - authentication;
//! - authorization;
//! - signature/integrity verification;
//! - compatibility validation;
//! - sandboxing/isolation where required;
//! - lifecycle ownership;
//! - deterministic identity;
//! - capability restrictions.
//!
//! Once an implementation is trusted and constructed, it may be explicitly
//! registered through the appropriate registry API.
//!
//! # Error ownership
//!
//! Registry implementations use the canonical resilience error/result
//! contracts. This module intentionally defines no competing registry error
//! type.
//!
//! # Integration map
//!
//! ## Detector registry
//!
//! `registry::detector` consumes:
//!
//! ```text
//! crate::quantum::resilience::detection::detector
//! ```
//!
//! It is consumed by detector execution, diagnosis, telemetry, and resilience
//! orchestration layers.
//!
//! ## Strategy registry
//!
//! `registry::strategy` consumes:
//!
//! ```text
//! crate::quantum::resilience::mitigation::strategy
//! ```
//!
//! It is consumed by strategy selection, planning, mitigation execution,
//! verification, telemetry, and history.
//!
//! ## Recovery registry
//!
//! `registry::recovery` consumes:
//!
//! ```text
//! crate::quantum::resilience::planning::action
//! crate::quantum::resilience::recovery::recoverer::ActionOutcome
//! ```
//!
//! It is consumed by recovery orchestration, policy/feasibility integration,
//! verification, telemetry, and history.
//!
//! ## Backend registry
//!
//! `registry::backend` consumes:
//!
//! ```text
//! crate::quantum::hardware::backend_trait::QuantumBackendAdapter
//! ```
//!
//! It is consumed by backend discovery, adaptation, migration, planning,
//! feasibility, provenance, and execution integration.
//!
//! # Public API stability
//!
//! The preferred caller-facing paths are:
//!
//! ```text
//! crate::quantum::resilience::registry::DetectorRegistry
//! crate::quantum::resilience::registry::StrategyRegistry
//! crate::quantum::resilience::registry::RecoveryRegistry
//! crate::quantum::resilience::registry::BackendRegistry
//! ```
//!
//! Stable key/identity types are explicitly re-exported below where they form
//! part of the corresponding registry's public contract.
//!
//! Wildcard re-exports are deliberately avoided. Future implementation
//! details must not silently become public API.
//!
//! # Adding future registries
//!
//! A future registry should be added as a new child module when it represents a
//! genuinely distinct implementation family, for example:
//!
//! ```text
//! decoder.rs
//! checkpoint.rs
//! verifier.rs
//! telemetry.rs
//! ```
//!
//! It should not be added here merely because another module needs a lookup
//! table. Prefer the registry closest to the contract it indexes.
//!
//! Adding a new implementation to an existing registry must not require
//! changing this file.
//!
//! # Safety
//!
//! This module is a safe-Rust composition boundary.
//!
//! The resilience registry layer must not contain unsafe Rust, raw pointers,
//! unsafe FFI, hidden mutable globals, or implicit process-wide state.
//!
//! Rust compatibility target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only;
//! - no nightly features.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Child registry modules
// =============================================================================

/// Deterministic registry for resilience detector implementations.
///
/// Owns detector registration, lookup, lifecycle/reset operations, and
/// deterministic detector execution dispatch.
///
/// Detection semantics remain in
/// `crate::quantum::resilience::detection::detector`.
pub mod detector;

/// Deterministic registry for mitigation strategy implementations.
///
/// Owns strategy registration and discovery.
///
/// Strategy evaluation, policy-aware selection, and execution remain in their
/// owning mitigation/planning layers.
pub mod strategy;

/// Deterministic registry for concrete recovery implementations.
///
/// Resolves declared recovery actions to implementations.
///
/// Policy, authorization, feasibility, orchestration, and result verification
/// remain outside the registry.
pub mod recovery;

/// Deterministic registry for executable quantum backend adapters.
///
/// Canonical backend semantics remain owned by `crate::quantum::hardware`.
/// This module provides only the resilience-facing adapter catalogue.
pub mod backend;

// =============================================================================
// Stable public re-exports
// =============================================================================
//
// Keep these explicit.
//
// Do not replace them with:
//     pub use detector::*;
//     pub use strategy::*;
//     pub use recovery::*;
//     pub use backend::*;
//
// Explicit exports prevent accidental public-API expansion when an internal
// registry implementation gains a helper type or implementation detail.

// -----------------------------------------------------------------------------
// Detector registry
// -----------------------------------------------------------------------------

pub use self::detector::{
    DetectorKey,
    DetectorRegistrationResult,
    DetectorRegistry,
    DetectorUnregistrationResult,
    DETECTOR_REGISTRY_SCHEMA_ID,
    DETECTOR_REGISTRY_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Strategy registry
// -----------------------------------------------------------------------------

pub use self::strategy::{
    StrategyKey,
    StrategyRegistry,
    STRATEGY_REGISTRY_SCHEMA_ID,
    STRATEGY_REGISTRY_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Recovery registry
// -----------------------------------------------------------------------------

pub use self::recovery::{
    RecoveryImplementation,
    RecoveryImplementationId,
    RecoveryRegistration,
    RecoveryRegistry,
    RecoveryRegistryKey,
    RECOVERY_REGISTRY_SCHEMA_ID,
    RECOVERY_REGISTRY_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Backend registry
// -----------------------------------------------------------------------------

pub use self::backend::{
    BackendRegistration,
    BackendRegistry,
    BackendRegistryKey,
    BACKEND_REGISTRY_SCHEMA_ID,
    BACKEND_REGISTRY_SCHEMA_VERSION,
};

// =============================================================================
// Registry-family metadata
// =============================================================================

/// Stable identifier for the resilience registry family.
///
/// This identifies the composition boundary rather than any individual
/// detector, strategy, recovery implementation, or backend adapter.
pub const REGISTRY_FAMILY_ID: &str =
    "zamani.quantum.resilience.registry";

/// Semantic version of the registry-family composition boundary.
///
/// This changes only when the public composition contract of this file changes,
/// such as changing child-module exposure or stable re-exports.
///
/// Individual registry schema versions remain owned by their respective child
/// modules.
pub const REGISTRY_FAMILY_VERSION: u16 = 1;

/// Returns the stable identifier of this registry family.
#[must_use]
pub const fn registry_family_id() -> &'static str {
    REGISTRY_FAMILY_ID
}

/// Returns the composition-boundary version.
#[must_use]
pub const fn registry_family_version() -> u16 {
    REGISTRY_FAMILY_VERSION
}