//! Zamani Quantum Scheduling — Planner Subsystem
//!
//! This module is the composition and public API boundary for the concrete
//! scheduling planner subsystem.
//!
//! # Responsibility
//!
//! `quantum::scheduling::planners` owns the organization and public exposure of
//! scheduling planner implementations and the stable planner contract.
//!
//! It does NOT itself implement scheduling.
//!
//! Concrete scheduling behaviour belongs to the child modules:
//!
//! ```text
//! quantum::scheduling::planners::planner
//! quantum::scheduling::planners::list
//! quantum::scheduling::planners::critical_path
//! quantum::scheduling::planners::resource_constrained
//! quantum::scheduling::planners::event
//! ```
//!
//! The parent module deliberately contains no scheduling algorithm.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         optimization
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                    SchedulingContext
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!        dependency         resources         timing
//!          graph              model            model
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              │
//!                              ▼
//!                 scheduling::planners
//!                              │
//!       ┌──────────────┬───────┼────────┬───────────────┐
//!       │              │       │        │               │
//!       ▼              ▼       ▼        ▼               ▼
//!     List       CriticalPath  RCPSP   Event       custom/plugin
//!       │              │       │        │               │
//!       └──────────────┴───────┼────────┴───────────────┘
//!                              │
//!                              ▼
//!                     SchedulingResult
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!       verification     transformations    diagnostics
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                       hardware/runtime
//! ```
//!
//! # Planner versus algorithm
//!
//! The stable planner contract is defined in [`planner`].
//!
//! Implementations are intentionally isolated:
//!
//! * [`list`] — list/ready-set based planning;
//! * [`critical_path`] — critical-path-oriented planning;
//! * [`resource_constrained`] — resource-constrained planning;
//! * [`event`] — event-driven planning.
//!
//! Adding another planner must normally require only:
//!
//! 1. adding its source file;
//! 2. implementing [`planner::SchedulingPlanner`];
//! 3. declaring the module here;
//! 4. optionally exporting its public type.
//!
//! Existing planner consumers must not need to be rewritten merely because a
//! new planner exists.
//!
//! # Write once, scale everywhere
//!
//! This module introduces no assumptions about:
//!
//! * qubit count;
//! * physical qubit count;
//! * logical qubit count;
//! * operation count;
//! * circuit depth;
//! * gate arity;
//! * resource count;
//! * channel count;
//! * timing resolution;
//! * topology size;
//! * QEC distance;
//! * number of QPUs;
//! * number of modules;
//! * number of network links;
//! * quantum technology;
//! * vendor;
//! * simulator size.
//!
//! Concrete limits belong to the scheduling context, scheduling limits,
//! target capabilities, deployment policy, or host environment.
//!
//! The planner subsystem therefore remains capable of serving the same
//! canonical Zamani program at different target scales.
//!
//! ```text
//! one qubit
//!     │
//!     ▼
//! small QPU
//!     │
//!     ▼
//! large QPU
//!     │
//!     ▼
//! modular QPU
//!     │
//!     ▼
//! distributed QPU
//!     │
//!     ▼
//! quantum network
//! ```
//!
//! "Infinity" in Zamani's architectural requirement means that this module
//! introduces no artificial finite machine-size ceiling. Every actual compiler
//! invocation is, necessarily, bounded by the resources available to the
//! compiler, operating system, target, and execution environment.
//!
//! # Canonical IR boundary
//!
//! Quantum semantic types remain owned by `quantum::ir`.
//!
//! In particular, the authoritative qubit identity types are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not define or re-export a competing scheduler-local
//! `QubitId`.
//!
//! Planner implementations consume the canonical IR and scheduling context
//! through their respective contracts.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling context / scheduling IR
//!      │
//!      ▼
//! scheduling planners
//!      │
//!      ▼
//! scheduling result
//! ```
//!
//! Planner modules must not become the owner of:
//!
//! * source parsing;
//! * canonical quantum semantics;
//! * logical-to-physical routing;
//! * hardware discovery;
//! * credentials;
//! * network communication;
//! * QPU execution;
//! * vendor SDKs;
//! * calibration acquisition;
//! * QEC decoding;
//! * noise-model ownership;
//! * benchmark ownership.
//!
//! Those concerns remain in their respective subsystems.
//!
//! # Timing boundary
//!
//! Planner implementations must consume timing abstractions supplied by the
//! scheduling subsystem.
//!
//! They must not define their own:
//!
//! * `TimePoint`;
//! * `Duration`;
//! * clock unit;
//! * hardware tick;
//! * sample period;
//! * nanosecond assumption.
//!
//! The canonical scheduling timing modules own those concepts.
//!
//! # Resource boundary
//!
//! Planner implementations consume resource models supplied by
//! `quantum::scheduling::resources`.
//!
//! They must not create independent resource calendars or fixed resource
//! counts merely because a particular algorithm happens to need one.
//!
//! Resource availability can be:
//!
//! * exclusive;
//! * capacity-limited;
//! * shared;
//! * hierarchical;
//! * time-dependent;
//! * dynamically unavailable;
//! * distributed.
//!
//! The planner chooses a schedule under the supplied model.
//!
//! # Routing boundary
//!
//! Routing answers:
//!
//! > WHERE should logical operations execute?
//!
//! Planning answers:
//!
//! > WHEN may those operations execute?
//!
//! Therefore this module does not perform logical-to-physical placement.
//!
//! Where a planner needs physical qubit information, the information must
//! already be present in the scheduling context/mapped representation and must
//! use the canonical `quantum::ir::qubit` identities.
//!
//! # Verification boundary
//!
//! Planner implementations produce candidate schedules.
//!
//! Canonical schedule verification belongs to:
//!
//! ```text
//! quantum::scheduling::verification
//! ```
//!
//! The planner subsystem must not replace that verifier.
//!
//! In particular, planner correctness must ultimately be checked for:
//!
//! * dependency preservation;
//! * resource capacity;
//! * timing constraints;
//! * alignment;
//! * operation coverage;
//! * target compatibility;
//! * dynamic-control dependencies;
//! * communication constraints;
//! * semantic preservation.
//!
//! # Transformation boundary
//!
//! Scheduling transformations remain outside this composition module.
//!
//! Examples include:
//!
//! * explicit delays;
//! * padding;
//! * alignment transformations;
//! * dynamical decoupling;
//! * other target-sensitive schedule transformations.
//!
//! A planner should produce a schedule under its input model rather than
//! silently embedding transformation policy into this module.
//!
//! # Dynamic execution
//!
//! Planner implementations may support workloads containing:
//!
//! * measurements;
//! * classical dependencies;
//! * conditionals;
//! * feedback;
//! * runtime events;
//! * dynamic branches;
//! * runtime-dependent timing.
//!
//! This module does not impose a static-DAG-only restriction.
//!
//! # Distributed execution
//!
//! Planner implementations may also operate on scheduling contexts containing:
//!
//! * multiple chips;
//! * multiple QPUs;
//! * modular systems;
//! * quantum network nodes;
//! * communication resources;
//! * remote-operation dependencies.
//!
//! Distributed topology and communication semantics remain supplied by the
//! appropriate scheduling/routing/resource layers.
//!
//! # Determinism
//!
//! This composition module introduces no global mutable state and no implicit
//! randomness.
//!
//! Deterministic behaviour is an implementation responsibility governed by the
//! supplied scheduling context/configuration.
//!
//! A deterministic planner must use deterministic:
//!
//! * traversal;
//! * candidate ordering;
//! * tie breaking;
//! * resource arbitration;
//! * output construction.
//!
//! A planner must never introduce hidden randomness merely because it is
//! registered or exported through this module.
//!
//! # Thread safety
//!
//! This module does not impose unsafe synchronization mechanisms.
//!
//! Planner implementations should prefer immutable input contexts and
//! instance-owned state. Concrete `Send`/`Sync` properties remain determined by
//! their contained types.
//!
//! No global planner singleton is defined here.
//!
//! # Object safety
//!
//! The stable [`planner::SchedulingPlanner`] contract is intentionally
//! object-safe so planner registries and plugin systems can store heterogeneous
//! implementations.
//!
//! This module does not weaken or replace that contract with an enum containing
//! every possible future planner.
//!
//! # Plugin compatibility
//!
//! External planners should implement the stable planner contract rather than
//! modifying this module's core abstractions.
//!
//! The intended extension direction is:
//!
//! ```text
//! external planner
//!       │
//!       ▼
//! SchedulingPlanner
//!       │
//!       ▼
//! planner registry/plugin boundary
//!       │
//!       ▼
//! SchedulingContext
//!       │
//!       ▼
//! SchedulingResult
//! ```
//!
//! A future plugin should therefore not require the core planner contract to
//! know its concrete type.
//!
//! # API stability
//!
//! The module declarations and explicitly selected re-exports below are part
//! of the public API.
//!
//! Glob exports are intentionally avoided.
//!
//! This prevents an unrelated public symbol added to a child implementation
//! from silently changing the public namespace or creating name collisions.
//!
//! # No speculative modules
//!
//! Only planner modules that currently exist in the repository are declared
//! here.
//!
//! The current repository planner directory contains:
//!
//! ```text
//! critical_path.rs
//! event.rs
//! list.rs
//! planner.rs
//! resource_constrained.rs
//! ```
//!
//! A future module must be declared here only after its implementation and
//! contract exist.
//!
//! This keeps the parent module compilable instead of declaring architectural
//! placeholders that do not physically exist.
//!
//! # Frozen-contract rule
//!
//! The planner contract in [`planner`] should remain stable while new planner
//! implementations are added.
//!
//! Adding a planner implementation should not require modifying unrelated
//! planner implementations.
//!
//! The normal integration sequence is:
//!
//! ```text
//! new planner implementation
//!          │
//!          ▼
//! impl SchedulingPlanner
//!          │
//!          ▼
//! add `pub mod <planner>;` here
//!          │
//!          ▼
//! optional explicit re-export
//!          │
//!          ▼
//! registry/plugin discovery
//! ```
//!
//! # Integration with `quantum::ir::qubit`
//!
//! This module deliberately does not import `QubitId` or
//! `PhysicalQubitId` because the composition boundary does not need to
//! manipulate qubit identities directly.
//!
//! Planner implementations that need qubit identities must import the
//! canonical definitions directly:
//!
//! ```text
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! No planner-local identity type should be introduced.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! * Rust 1.97;
//! * Rust 1.97.1;
//! * Rust 2021 edition;
//! * stable Rust;
//! * no nightly features;
//! * no unsafe code.
//!
//! The safety requirement is compiler-enforced.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Planner modules
// =============================================================================

/// Stable, implementation-independent planner contract.
///
/// This module defines the `SchedulingPlanner` trait and its associated
/// metadata/identifier contract. Concrete algorithms must implement that
/// contract rather than changing it.
pub mod planner;

/// List/ready-set scheduling planners.
///
/// This module contains list-based scheduling implementations that operate on
/// the supplied dependency, timing, and resource models.
pub mod list;

/// Critical-path-oriented scheduling planners.
///
/// This module contains critical-path scheduling implementations. It does not
/// own the canonical critical-path analysis type; it consumes the appropriate
/// scheduling analysis contracts.
pub mod critical_path;

/// Resource-constrained scheduling planners.
///
/// This module contains scheduling implementations that explicitly account for
/// finite and time-dependent resource availability.
pub mod resource_constrained;

/// Event-driven scheduling planners.
///
/// This module contains planners that advance scheduling through relevant
/// scheduling events rather than requiring a fixed machine-size timeline.
pub mod event;

// =============================================================================
// Stable public contract exports
// =============================================================================
//
// These are deliberately explicit.
//
// Do not replace these with:
//
//     pub use planner::*;
//
// because a wildcard export would allow unrelated future additions inside
// `planner.rs` to silently change the public API of this parent module.
//
// =============================================================================

/// Stable planner contract.
///
/// This is the primary trait implemented by every concrete scheduling planner.
pub use planner::SchedulingPlanner;

/// Stable planner identifier.
///
/// Planner IDs identify planner implementations without encoding machine size,
// topology, timing units, or vendor assumptions.
pub use planner::PlannerId;

/// Immutable planner metadata.
///
/// Metadata describes a planner without coupling callers to its implementation
/// state.
pub use planner::PlannerMetadata;

/// Planner contract version.
///
/// This is the semantic version of the planner API, not the Zamani package
/// version.
pub use planner::PLANNER_CONTRACT_VERSION;

// =============================================================================
// Explicit algorithm exports
// =============================================================================
//
// These exports are intentionally conditional on concrete public types exposed
// by the implementation modules.
//
// The implementation modules remain available through their canonical paths:
//
//     planners::list
//!     planners::critical_path
//!     planners::resource_constrained
//!     planners::event
//
// Concrete implementation APIs remain owned by those modules.
//
// Do not introduce a catch-all enum such as:
//
//     enum PlannerKind {
//         List,
//         CriticalPath,
//         ...
//     }
//
// because that would require this file to change every time a new planner is
// added. Heterogeneous planners belong behind `SchedulingPlanner` and the
// registry/plugin boundary.
// =============================================================================

// =============================================================================
// Public prelude
// =============================================================================
//
// Keep this prelude deliberately narrow.
//
// It exists for callers that need the fundamental planner contract. Algorithm-
// specific implementation types should normally be imported from their
// canonical child modules.
//
// =============================================================================

/// Minimal planner API for consumers that only need to implement or invoke the
/// stable planner contract.
///
/// This does not flatten algorithm-specific types into the parent namespace.
pub mod prelude {
    pub use super::{
        PlannerId,
        PlannerMetadata,
        SchedulingPlanner,
        PLANNER_CONTRACT_VERSION,
    };
}

// =============================================================================
// Compile-time API invariants
// =============================================================================
//
// These assertions intentionally use only stable language facilities and
// require no unsafe code.
//
// The planner contract itself is object-safe by design. Keeping the trait
// behind a trait object is important for registries and plugin systems.
//
// The following helper does not execute and exists only to make the intended
// object-safe boundary explicit to the compiler.
//
// =============================================================================

#[allow(dead_code)]
fn assert_planner_object_safe(
    planner: &dyn SchedulingPlanner,
) -> &dyn SchedulingPlanner {
    planner
}