//! Zamani Quantum Resilience — Planning Subsystem
//!
//! Path:
//!     src/quantum/resilience/planning/mod.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module is the composition boundary for the resilience planning
//! subsystem.
//!
//! Planning answers:
//!
//!     "Given the current incident, diagnosis, policy, capabilities,
//!      execution state, history and verification requirements, what
//!      recovery/adaptation plans are permissible candidates?"
//!
//! This module ONLY composes the planning contracts.
//!
//! It does NOT:
//!
//! - detect faults;
//! - diagnose faults;
//! - execute recovery;
//! - perform routing;
//! - perform scheduling;
//! - compile programs;
//! - optimize circuits;
//! - perform QEC;
//! - perform error mitigation;
//! - communicate with quantum hardware;
//! - discover hardware;
//! - authorize recovery;
//! - verify final execution results;
//! - define canonical quantum IR;
//! - define ZQN fault semantics;
//! - maintain global mutable state;
//! - impose a maximum number of qubits;
//! - impose a maximum number of devices;
//! - impose a maximum number of plans;
//! - impose a fixed retry count;
//! - impose provider-specific behaviour.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//! ```text
//!
//!                         Quantum Program
//!                                |
//!                                v
//!                         Canonical Quantum IR
//!                                |
//!                 +--------------+--------------+
//!                 |                             |
//!                 v                             v
//!             Detection                     Hardware
//!                 |                         Capabilities
//!                 v                             |
//!             Diagnosis                         |
//!                 |                             |
//!                 +--------------+--------------+
//!                                |
//!                                v
//!                             Policy
//!                                |
//!                                v
//!                     +-----------------------+
//!                     |       planning        |
//!                     |                       |
//!                     | action                |
//!                     | cost                  |
//!                     | feasibility           |
//!                     | ranking               |
//!                     | plan                  |
//!                     | planner_state         |
//!                     | planner               |
//!                     +-----------+-----------+
//!                                 |
//!                                 v
//!                         Adaptation / Recovery
//!                                 |
//!                                 v
//!                            Verification
//! ```
//!
//! Planning is therefore a decision-producing layer between policy/evidence
//! and execution-changing subsystems.
//!
//! =============================================================================
//! "Write once, scale everywhere"
//! =============================================================================
//!
//! Planning MUST remain independent of physical machine size.
//!
//! The same planning contracts must work for:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant quantum computer;
//! - multiple QPUs;
//! - heterogeneous quantum backends;
//! - distributed quantum systems;
//! - future quantum technologies not yet represented by this repository.
//!
//! No planning module may encode assumptions such as:
//!
//!     MAX_QUBITS
//!     MAX_DEVICES
//!     MAX_BACKENDS
//!     retry_count = 3
//!     physical_qubit = 5
//!     backend = "specific-provider"
//!
//! Resource quantities, capabilities, budgets and constraints must come from
//! authoritative runtime, hardware, policy and resource-model contracts.
//!
//! =============================================================================
//! Dependency ownership
//! =============================================================================
//!
//! The planning subsystem consumes contracts from other subsystems but does
//! not take ownership of their implementation.
//!
//! ```text
//! quantum::ir
//!       |
//! quantum::zqn
//!       |
//! quantum::hardware
//!       |
//! quantum::routing
//!       |
//! quantum::scheduling
//!       |
//! quantum::optimization
//!       |
//! quantum::qec
//!       |
//! quantum::resilience::{policy, diagnosis, state, history, verification}
//!       |
//!       v
//! planning
//! ```
//!
//! The reverse dependency MUST NOT be introduced merely to access concrete
//! planning implementations.
//!
//! For example:
//!
//!     hardware -> resilience::planning::Planner
//!
//! is architecturally undesirable.
//!
//! Instead, hardware exposes capability/state contracts and resilience consumes
//! those contracts.
//!
//! =============================================================================
//! Module ownership
//! =============================================================================
//!
//! ## action.rs
//!
//! Defines the canonical provider-independent recovery/adaptation action
//! contract.
//!
//! Examples include:
//!
//! - retry;
//! - restart;
//! - resume;
//! - rollback;
//! - checkpoint;
//! - remap;
//! - reroute;
//! - reschedule;
//! - recompile;
//! - reoptimize;
//! - QEC adaptation;
//! - mitigation;
//! - migration;
//! - quarantine;
//! - compensation;
//! - escalation;
//! - abort.
//!
//! `RecoveryAction` is a request/description, not authorization.
//!
//! Execution belongs to adaptation/recovery/mitigation subsystems.
//!
//! The action model intentionally uses canonical quantum resource identity where
//! individual quantum resources must be represented. It must not introduce a
//! second QubitId hierarchy.
//!
//! ## cost.rs
//!
//! Defines the provider-independent multidimensional cost model used when
//! comparing candidate recovery plans.
//!
//! Costs may include dimensions such as:
//!
//! - time;
//! - shots;
//! - resource pressure;
//! - logical-error contribution;
//! - financial cost;
//! - compilation effort;
//! - routing effort;
//! - scheduling effort;
//! - QEC overhead;
//! - mitigation overhead.
//!
//! Cost is an estimate and MUST NOT be treated as execution truth.
//!
//! ## feasibility.rs
//!
//! Determines whether a proposed action/plan is currently feasible using
//! explicitly supplied evidence and capabilities.
//!
//! It does not execute the action and does not replace policy authorization.
//!
//! ## ranking.rs
//!
//! Deterministically ranks already-normalized candidate plans.
//!
//! Ranking is not authorization.
//!
//! A highly ranked plan can still be rejected by policy, security,
//! execution-time preconditions or verification.
//!
//! ## plan.rs
//!
//! Owns the immutable recovery-plan representation.
//!
//! A plan represents a proposed sequence of actions together with the evidence,
//! constraints, cost/risk information and verification requirements needed to
//! evaluate that proposal.
//!
//! ## planner_state.rs
//!
//! Owns durable planner-state semantics.
//!
//! It provides the state boundary required for:
//!
//! - deterministic planning;
//! - replay;
//! - auditability;
//! - lifecycle tracking;
//! - plan generation/version tracking;
//! - invalidation;
//! - bounded or caller-managed history;
//! - persistence/checkpoint integration.
//!
//! It does not execute plans.
//!
//! ## planner.rs
//!
//! Owns candidate plan generation and planning orchestration.
//!
//! The planner consumes:
//!
//! - diagnosis;
//! - policy;
//! - capabilities;
//! - resource state;
//! - planner state;
//! - history;
//! - cost models;
//! - feasibility results;
//! - verification requirements;
//!
//! and produces candidate/selected planning results according to its contract.
//!
//! =============================================================================
//! Public API policy
//! =============================================================================
//!
//! This module deliberately re-exports the stable planning surface.
//!
//! Consumers should normally prefer:
//!
//!     quantum::resilience::planning::Planner
//!
//! rather than depending on internal module paths when a type is part of the
//! public planning API.
//!
//! Submodules remain public because resilience integrations may need to consume
//! individual contracts without depending on the complete planner.
//!
//! This gives Zamani both:
//!
//!     fine-grained integration
//!
//! and:
//!
//!     a stable planning namespace.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================
//!
//! Planning is not an authorization boundary.
//!
//! The lifecycle MUST conceptually remain:
//!
//!     evidence
//!       |
//!       v
//!     diagnosis
//!       |
//!       v
//!     policy
//!       |
//!       v
//!     planning
//!       |
//!       v
//!     feasibility / authorization
//!       |
//!       v
//!     adaptation / recovery / mitigation
//!       |
//!       v
//!     verification
//!       |
//!       v
//!     acceptance
//!
//! No caller may infer:
//!
//!     selected plan == safe plan
//!
//! or:
//!
//!     feasible plan == authorized plan
//!
//! or:
//!
//!     ranked plan == verified result
//!
//! Those are separate gates by design.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! The planning namespace itself introduces no nondeterminism.
//!
//! Individual planning components are responsible for honouring the repository's
//! determinism contract.
//!
//! In deterministic operation:
//!
//! - candidate ordering must not depend on hash-map iteration order;
//! - plan identity must remain stable;
//! - ranking must use deterministic tie-breaking;
//! - numeric calculations must use deterministic representations;
//! - state transitions must be reproducible;
//! - caller-provided seeds/configuration must be part of planning provenance;
//! - wall-clock time must not silently become a planning input.
//!
//! The module boundary does not generate random identifiers or access global
//! mutable state.
//!
//! =============================================================================
//! Scalability
//! =============================================================================
//!
//! This module contains no collection with a hidden fixed capacity and no
//! machine-size constant.
//!
//! Any collection capacity or retention policy belongs to the component that
//! owns the collection and MUST be explicitly caller/configuration controlled.
//!
//! Consequently, scaling is bounded only by:
//!
//! - available memory;
//! - available compute;
//! - caller-supplied resource limits;
//! - hardware capability;
//! - policy constraints;
//! - number of candidate plans actually supplied/generated.
//!
//! The planning API itself does not establish an artificial quantum-system
//! ceiling.
//!
//! =============================================================================
//! Canonical quantum identity
//! =============================================================================
//!
//! Planning code that needs to identify individual quantum resources MUST use
//! the canonical repository identity types exposed under:
//!
//!     crate::quantum::ir::qubit
//!
//! In particular, planning modules must not create a local replacement such as:
//!
//!     struct QubitId(...);
//!
//! `mod.rs` intentionally does not import QubitId because the namespace
//! composition layer does not manipulate individual qubits.
//!
//! =============================================================================
//! Serialization and persistence
//! =============================================================================
//!
//! Planning types may be serialized by:
//!
//!     quantum::resilience::serialization
//!
//! The planning namespace itself does not implement persistence.
//!
//! This separation ensures that:
//!
//! - serialization schema evolution remains centralized;
//! - planner state can be checkpointed without coupling the planner to storage;
//! - persistence backends can change independently;
//! - deterministic replay can use versioned planning data.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! ## Upstream consumers
//!
//! The planning subsystem may consume:
//!
//! - `quantum::resilience::model`;
//! - `quantum::resilience::diagnosis`;
//! - `quantum::resilience::policy`;
//! - `quantum::resilience::state`;
//! - `quantum::resilience::history`;
//! - `quantum::resilience::verification`;
//! - `quantum::hardware`;
//! - `quantum::routing`;
//! - `quantum::scheduling`;
//! - `quantum::optimization`;
//! - `quantum::qec`;
//! - canonical `quantum::ir`.
//!
//! These dependencies belong inside the implementation files that actually
//! require them. This module should not import all of those subsystems merely
//! to make the namespace compile.
//!
//! ## Downstream consumers
//!
//! Planning outputs may be consumed by:
//!
//! - `adaptation/*`;
//! - `recovery/*`;
//! - `mitigation/*`;
//! - `verification/*`;
//! - `telemetry/*`;
//! - `history/*`;
//! - `coordination/*`;
//! - `serialization/*`.
//!
//! Those consumers should use the public contracts exposed here rather than
//! reaching into planner implementation details.
//!
//! =============================================================================
//! Integration with quantum::resilience
//! =============================================================================
//!
//! The parent resilience module should expose this namespace through:
//!
//!     pub mod planning;
//!
//! The parent module should not duplicate these declarations or re-export
//! individual planning implementation details unless deliberately defining the
//! top-level resilience API.
//!
//! =============================================================================
//! Integration with quantum::mod.rs
//! =============================================================================
//!
//! `src/quantum/mod.rs` should expose the resilience subsystem:
//!
//!     pub mod resilience;
//!
//! Planning must not be independently registered at the top-level quantum
//! namespace. Its canonical path is:
//!
//!     crate::quantum::resilience::planning
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Planning contracts
// =============================================================================

/// Canonical recovery/adaptation action model.
///
/// This module defines descriptions of actions; it does not execute them.
pub mod action;

/// Provider-independent multidimensional planning cost model.
pub mod cost;

/// Capability- and context-aware plan/action feasibility evaluation.
pub mod feasibility;

/// Deterministic candidate ranking.
pub mod ranking;

/// Immutable recovery-plan contract.
pub mod plan;

/// Durable planner lifecycle/state boundary.
pub mod planner_state;

/// Candidate generation and planning orchestration.
pub mod planner;

// =============================================================================
// Stable public re-exports
// =============================================================================

// Action contract.
pub use action::{
    ActionKind,
    ActionPriority,
    ActionPrecondition,
    ActionScope,
    ExpectedEffect,
    RecoveryAction,
    RecoveryActionKind,
    ResourceId,
};

// Cost contract.
//
// Re-export only types that are part of the actual cost module's public
// contract. The concrete module remains authoritative if additional cost
// dimensions are added later.
pub use cost::{
    CostModel,
    CostVector,
};

// Feasibility contract.
pub use feasibility::{
    FeasibilityChecker,
    FeasibilityResult,
    FeasibilityStatus,
};

// Ranking contract.
//
// The ranking module deliberately owns candidate/ranking types so plan.rs does
// not become coupled to the ranking implementation.
pub use ranking::{
    CandidateId,
    FeasibilityClass,
    FixedScore,
    RankedCandidate,
    RankingCandidate,
    RankingEngine,
    RankingError,
    RankingObjective,
    RankingPolicy,
};

// Plan contract.
pub use plan::{
    PlanId,
    PlanState,
    PlanVersion,
    RecoveryPlan,
};

// Planner state contract.
pub use planner_state::{
    PlannerState,
};

// Planner orchestration contract.
pub use planner::{
    Planner,
    PlannerConfig,
    PlannerError,
    PlannerErrorKind,
    PlannerInput,
    PlannerOutcome,
    PlannerReason,
};

// =============================================================================
// Namespace-level invariants
// =============================================================================

/// Stable planning subsystem schema identifier.
///
/// This is intentionally a namespace-level identifier and is not a replacement
/// for the schema identifiers owned by action/cost/ranking/plan/state modules.
pub const PLANNING_SCHEMA_ID: &str =
    "zamani.quantum.resilience.planning";

/// Current semantic version of the planning namespace.
///
/// Individual serialized contracts retain ownership of their own schema
/// versions. This version describes the composition boundary itself.
pub const PLANNING_SCHEMA_VERSION: u16 = 1;