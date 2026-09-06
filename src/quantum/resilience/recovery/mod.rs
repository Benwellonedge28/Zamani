//! Zamani Quantum Resilience — Recovery Subsystem
//!
//! Path:
//!     src/quantum/resilience/recovery/mod.rs
//!
//! Purpose:
//!     Defines the public module boundary for provider-independent quantum
//!     recovery mechanisms.
//
//! Architectural role:
//!
//!     detection
//!         |
//!         v
//!     diagnosis
//!         |
//!         v
//!     policy
//!         |
//!         v
//!     planning
//!         |
//!         v
//!     recovery
//!         |
//!         +--> retry
//!         +--> restart
//!         +--> resume
//!         +--> rollback
//!         +--> checkpoint recovery
//!         +--> orchestration
//!         |
//!         v
//!     verification
//!
//! This module is the recovery namespace only. It does not own fault detection,
//! diagnosis, policy, planning, routing, scheduling, compilation, optimization,
//! QEC, mitigation, hardware drivers, backend SDKs, persistence, or telemetry
//! export.
//!
//! -----------------------------------------------------------------------------
//! DESIGN CONTRACT
//! -----------------------------------------------------------------------------
//!
//! Recovery is deliberately separated into:
//!
//! - declarative planning;
//! - recovery orchestration;
//! - concrete recovery mechanisms;
//! - verification;
//! - external state and execution boundaries.
//!
//! A recovery mechanism MUST NOT silently invent policy.
//!
//! A recovery mechanism MUST NOT assume a particular:
//!
//! - quantum processor;
//! - provider;
//! - topology;
//! - number of qubits;
//! - number of devices;
//! - number of retries;
//! - number of shots;
//! - execution duration;
//! - backend;
//! - QEC code;
//! - simulator;
//! - machine architecture.
//!
//! Operational limits come from callers and the authoritative resilience
//! policy/configuration layers.
//!
//! -----------------------------------------------------------------------------
//! WRITE ONCE, SCALE EVERYWHERE
//! -----------------------------------------------------------------------------
//!
//! This namespace imposes no artificial quantum-machine-size limit.
//!
//! There is deliberately no:
//!
//! - MAX_QUBITS;
//! - MAX_DEVICES;
//! - MAX_BACKENDS;
//! - MAX_RETRIES;
//! - MAX_ATTEMPTS;
//! - MAX_SHOTS;
//! - fixed topology;
//! - fixed provider;
//! - fixed physical-qubit numbering;
//! - fixed logical-to-physical mapping.
//!
//! "Infinite" scale means that recovery introduces no artificial finite machine
//! ceiling. Actual execution remains bounded only by the resources,
//! capabilities, budgets, policies, memory, runtime and physical system
//! available to the deployment.
//!
//! -----------------------------------------------------------------------------
//! QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! Recovery implementations normally should not manipulate qubit identities.
//!
//! Whenever a recovery implementation must identify a quantum resource, it
//! MUST use the canonical quantum IR identity types exposed by:
//!
//!     crate::quantum::ir::qubit
//!
//! In particular, implementations must not introduce a recovery-local QubitId.
//!
//! The recovery namespace therefore does not define:
//!
//!     QubitId
//!     PhysicalQubitId
//!     LogicalQubitId
//!
//! as competing identity types.
//!
//! -----------------------------------------------------------------------------
//! SAFETY
//! -----------------------------------------------------------------------------
//!
//! This module is intentionally safe Rust.
//!
//! Requirements:
//!
//! - Rust 2021;
//! - Rust 1.97 / 1.97.1;
//! - no unsafe code;
//! - no unsafe FFI;
//! - no raw pointers;
//! - no provider credentials;
//! - no provider secrets;
//! - no global mutable recovery state;
//! - no hidden threads;
//! - no hidden timers;
//! - no hidden retries;
//! - no hidden hardware access.
//!
//! Each concrete recovery implementation owns only its declared responsibility.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! Recovery must support deterministic execution where requested by the caller.
//!
//! The recovery namespace therefore does not:
//!
//! - generate random operation identifiers;
//! - read the wall clock;
//! - inspect environment variables;
//! - depend on unordered collection iteration;
//! - silently mutate recovery plans;
//! - hide retry loops;
//! - hide backoff waits.
//!
//! Deterministic recovery is achieved by composing:
//!
//!     immutable RecoveryPlan
//!         +
//!     immutable execution context
//!         +
//!     explicit operational limits
//!         +
//!     deterministic recovery implementation
//!         +
//!     deterministic verification
//!
//! -----------------------------------------------------------------------------
//! RECOVERY SEMANTICS
//! -----------------------------------------------------------------------------
//!
//! Recovery mechanisms are deliberately distinct:
//!
//!     retry     != restart
//!     retry     != resume
//!     retry     != rollback
//!     rollback  != checkpoint restore
//!     restart   != retry policy
//!     resume    != restoration of arbitrary quantum state
//!
//! A retry may create a new physical execution.
//!
//! A restart may reconstruct an execution from an accepted boundary.
//!
//! A resume may continue only from a valid resumable boundary.
//!
//! A rollback may restore an accepted execution state only where the runtime
//! and execution model actually support such restoration.
//!
//! Checkpoint recovery may restore/reconstruct only the state represented by a
//! valid checkpoint contract. It must never pretend that an arbitrary unknown
//! quantum state can be serialized and restored merely by copying classical
//! metadata.
//!
//! -----------------------------------------------------------------------------
//! CURRENT MODULES
//! -----------------------------------------------------------------------------
//!
//! `retry`
//!     Executes an already-authorized retry operation.
//!
//! `restart`
//!     Executes provider-independent restart semantics.
//!
//! `resume`
//!     Continues an execution from an explicitly valid resumable boundary.
//!
//! `rollback`
//!     Performs safe rollback through an injected execution/state contract.
//!
//! `checkpoint`
//!     Provides recovery-facing checkpoint orchestration.
//!
//! `recoverer`
//!     Coordinates an immutable recovery plan, validates freshness and
//!     preconditions, acquires ownership, executes actions, and coordinates
//!     verification.
//!
//! -----------------------------------------------------------------------------
//! FUTURE MODULES
//! -----------------------------------------------------------------------------
//!
//! The architecture documents additional recovery capabilities such as:
//!
//!     migration
//!     compensation
//!
//! These MUST NOT be declared here until their actual implementation files and
//! contracts exist. Declaring a Rust module before its source file exists would
//! make the crate fail to compile.
//!
//! When implemented, they should be added as:
//!
//!     pub mod migration;
//!     pub mod compensation;
//!
//! without changing the semantics of the existing modules.
//!
//! -----------------------------------------------------------------------------
//! DEPENDENCY DIRECTION
//! -----------------------------------------------------------------------------
//!
//! Recovery consumes contracts from other resilience/quantum subsystems:
//!
//!     recovery
//!        |
//!        +--> planning
//!        +--> policy contracts
//!        +--> state
//!        +--> checkpoint
//!        +--> verification contracts
//!        +--> coordination contracts
//!        +--> telemetry contracts
//!        +--> history contracts
//!        +--> execution/HAL contracts
//!
//! Recovery must NOT make those subsystems depend on concrete recovery
//! implementations merely to compile.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION CONTRACTS
//! -----------------------------------------------------------------------------
//!
//! `planning::action`
//!     Defines declarative recovery actions.
//!
//! `planning::plan`
//!     Defines the immutable ordered RecoveryPlan consumed by `recoverer`.
//!
//! `policy::retry`
//!     Defines retry authorization and retry policy.
//!
//! `state::*`
//!     Provides current execution/resource state and freshness information.
//!
//! `checkpoint::*`
//!     Owns checkpoint representation, validation and persistence contracts.
//!
//! `verification::*`
//!     Determines whether an adapted/recovered result is semantically and
//!     operationally acceptable.
//!
//! `coordination::*`
//!     Provides ownership and lease contracts for distributed recovery.
//!
//! `telemetry::*`
//!     Receives deterministic recovery lifecycle events.
//!
//! `history::*`
//!     Persists recovery outcomes for future analysis and planning.
//!
//! `quantum::hardware::*`
//!     Remains behind execution/environment contracts. Recovery must not expose
//!     provider-specific hardware types.
//!
//! `quantum::ir::qubit`
//!     Remains the canonical source of quantum resource identity.
//!
//! `quantum::routing`
//!     Owns routing/remapping algorithms; recovery only requests adaptation
//!     through the appropriate contract.
//!
//! `quantum::scheduling`
//!     Owns schedule generation; recovery only requests rescheduling through
//!     the appropriate contract.
//!
//! `quantum::optimization`
//!     Owns optimization; recovery does not duplicate optimization passes.
//!
//! `quantum::error_correction`
//!     Owns QEC implementation; recovery decides whether an available QEC
//!     configuration should be requested, but does not implement the decoder.
//!
//! -----------------------------------------------------------------------------
//! PUBLIC API PRINCIPLE
//! -----------------------------------------------------------------------------
//!
//! The module boundary intentionally re-exports the stable recovery contracts
//! rather than forcing callers to depend on private module layout.
//!
//! Consumers should prefer:
//!
//!     quantum::resilience::recovery::RecoveryPlan
//!
//!     quantum::resilience::recovery::RecoveryAction
//!
//!     quantum::resilience::recovery::RecoveryOperationId
//!
//!     quantum::resilience::recovery::ExecutionId
//!
//! where those contracts are intentionally exposed by this namespace.
//!
//! Concrete implementation modules remain available for advanced integration,
//! testing and dependency injection.
//!
//! -----------------------------------------------------------------------------
//! EXTENSIBILITY
//! -----------------------------------------------------------------------------
//!
//! New recovery implementations must:
//!
//! 1. receive an explicit contract;
//! 2. avoid provider-specific types;
//! 3. avoid hard-coded resource limits;
//! 4. avoid hidden policy;
//! 5. avoid hidden retries;
//! 6. avoid hidden waiting;
//! 7. preserve provenance;
//! 8. expose deterministic outcomes;
//! 9. distinguish unknown execution state from failure;
//! 10. remain verifiable by the verification subsystem.
//!
//! Adding a new recovery strategy should not require changing existing recovery
//! mechanisms.
//!
//! -----------------------------------------------------------------------------
//! IMPORTANT IMPLEMENTATION RULE
//! -----------------------------------------------------------------------------
//!
//! This `mod.rs` is a namespace/composition boundary. It must remain free of:
//!
//! - orchestration algorithms;
//! - recovery loops;
//! - hardware operations;
//! - state mutation;
//! - policy decisions;
//! - I/O;
//! - persistence;
//! - timing;
//! - provider-specific code.
//!
//! The orchestration implementation belongs in `recoverer.rs`.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Concrete recovery mechanisms
// ============================================================================

/// Executes an already-authorized retry operation.
///
/// Retry policy belongs to `resilience::policy::retry`; this module only
/// provides the execution mechanism.
pub mod retry;

/// Executes provider-independent restart semantics.
///
/// Restart is deliberately distinct from retry and must not invent retry
/// policy.
pub mod restart;

/// Executes continuation from an explicitly valid resumable boundary.
///
/// Resume does not imply restoration of arbitrary quantum state.
pub mod resume;

/// Executes safe rollback through an injected execution/state contract.
///
/// Rollback must never be implemented as an assumption that arbitrary quantum
/// state is clonable.
pub mod rollback;

/// Provides recovery-facing checkpoint orchestration.
///
/// Durable checkpoint representation and storage remain owned by the broader
/// checkpoint subsystem.
pub mod checkpoint;

// ============================================================================
// Recovery orchestration
// ============================================================================

/// Coordinates immutable recovery plans with execution, ownership,
/// freshness validation and verification.
pub mod recoverer;

// ============================================================================
// Stable public re-exports
// ============================================================================
//
// Keep the re-export surface intentional. The individual modules remain
// addressable for advanced implementations, while the most important recovery
// contracts are available directly under:
//
//     quantum::resilience::recovery::*
//
// The exact names below correspond to the contracts already established by
// `recoverer.rs` and `planning::*` in the current repository architecture.

// Recovery orchestrator contracts.
pub use recoverer::{
    ActionOutcome,
    ExecutionId,
    PlanFreshness,
    RecoveryEvent,
    RecoveryLimits,
    RecoveryOperationId,
    RecoveryOutcome,
    RecoveryState,
    VerificationOutcome,
};

// Declarative planning contracts.
//
// These are re-exported here because recovery consumers should not need to know
// that an immutable recovery plan originates from the planning namespace.
// The planning subsystem remains the authoritative owner of these types.
pub use crate::quantum::resilience::planning::action::{
    ActionKind,
    RecoveryAction,
};

pub use crate::quantum::resilience::planning::plan::RecoveryPlan;