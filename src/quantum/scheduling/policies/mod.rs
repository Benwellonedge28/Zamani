//! Zamani Quantum Scheduling — Policy Module
//!
//! Production-grade module boundary for scheduling policies.
//!
//! # Responsibility
//!
//! This module is the public composition boundary for
//! `crate::quantum::scheduling::policies`.
//!
//! It:
//!
//! - declares policy submodules;
//! - exposes the stable policy contracts;
//! - keeps policy implementation modules independently maintainable;
//! - provides a single import boundary for the scheduler;
//! - documents the ownership and integration model.
//!
//! It does NOT:
//!
//! - schedule quantum operations;
//! - construct dependency graphs;
//! - allocate physical qubits;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - acquire calibration;
//! - execute workloads;
//! - generate pulses;
//! - implement QEC;
//! - communicate with providers;
//! - define quantum IR;
//! - define `QubitId`;
//! - own hardware capacities;
//! - impose machine-size limits;
//! - contain global mutable state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                              v
//!                         optimization
//!                              |
//!                              v
//!                            routing
//!                              |
//!                              v
//!                    quantum::scheduling
//!                              |
//!                 +------------+------------+
//!                 |                         |
//!                 v                         v
//!             scheduling               policies
//!                 |                         |
//!                 |              +----------+----------+
//!                 |              |          |          |
//!                 |              v          v          v
//!                 |             ASAP       ALAP    Resource-aware
//!                 |              |          |          |
//!                 +--------------+----------+----------+
//!                                |
//!                                v
//!                             planner
//!                                |
//!                                v
//!                             schedule
//!                                |
//!                                v
//!                            verification
//!                                |
//!                                v
//!                            execution
//! ```
//!
//! Policies describe scheduling intent.
//!
//! Planners and algorithms decide how that intent is realized.
//!
//! # Core ownership rule
//!
//! ```text
//! policy = WHAT scheduling should prefer
//! planner = HOW candidate operations are selected
//! resources = WHAT hardware resources exist
//! timing = WHEN resources may be used
//! constraints = WHAT must never be violated
//! verification = WHETHER the resulting schedule is legal
//! execution = ACTUALLY perform the schedule
//! ```
//!
//! Keeping these responsibilities separate is essential for scalability.
//!
//! A policy must never encode assumptions such as:
//!
//! - a fixed number of qubits;
//! - a fixed number of gates;
//! - a fixed topology;
//! - a fixed number of control channels;
//! - a fixed number of measurement channels;
//! - a fixed gate set;
//! - a fixed clock period;
//! - a fixed QEC code distance;
//! - a fixed machine size.
//!
//! Those properties are supplied by the target/resource/timing models.
//!
//! # Canonical quantum identity
//!
//! Scheduling policy code must use the canonical quantum IR identity:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! where a policy implementation actually needs qubit identity.
//!
//! This module deliberately does not import or redefine `QubitId` because
//! policy contracts should remain as hardware- and IR-independent as possible.
//!
//! A policy must never create a competing scheduling-specific `QubitId`.
//!
//! # Policy contract
//!
//! The foundational contract lives in:
//!
//! `policies::policy`
//!
//! It defines the semantic vocabulary used by the rest of the scheduling
//! subsystem, including:
//!
//! - `SchedulingPolicy`;
//! - `SchedulingPolicyKind`;
//! - `SchedulingObjective`;
//! - `TieBreakRule`;
//! - `Determinism`;
//! - custom-policy identification;
//! - policy validation;
//! - policy construction.
//!
//! The policy contract is intentionally independent of concrete scheduling
//! algorithms.
//!
//! # Concrete policies
//!
//! The following modules provide concrete policy behavior:
//!
//! - `asap` — as-soon-as-possible scheduling;
//! - `alap` — as-late-as-possible scheduling;
//! - `priority` — deterministic priority evaluation;
//! - `resource_aware` — resource-pressure-aware scheduling;
//! - `hybrid` — composition of multiple scheduling preferences.
//!
//! These modules consume the foundational contract from `policy.rs`.
//!
//! Adding another policy should normally require adding a new module and
//! registering the implementation through the scheduler/plugin boundary.
//!
//! `mod.rs` should not need semantic changes merely because a new policy is
//! added.
//!
//! # Important distinction from hardware scheduling
//!
//! `crate::quantum::hardware::scheduling` already contains hardware-level
//! scheduling concepts, including a hardware-oriented `SchedulingPolicy`.
//!
//! That type belongs to the hardware scheduling layer.
//!
//! It must not be re-exported here under the same name.
//!
//! The two layers have different responsibilities:
//!
//! ```text
//! quantum::scheduling::policies
//!     = scheduling intent
//!
//! quantum::hardware::scheduling
//!     = physical hardware scheduling constraints
//! ```
//!
//! Conversion between these representations belongs in an adapter/integration
//! layer. It must not be hidden inside this module.
//!
//! This avoids enum-name collisions and prevents the scheduling policy layer
//! from becoming coupled to hardware implementation details.
//!
//! # Important distinction from optimization scheduling
//!
//! `crate::quantum::optimization::scheduler` schedules optimization passes.
//!
//! It must remain separate from quantum execution scheduling.
//!
//! ```text
//! quantum::optimization::scheduler
//!     = order compiler optimization passes
//!
//! quantum::scheduling
//!     = schedule quantum operations for execution
//! ```
//!
//! This module belongs exclusively to the latter.
//!
//! # Integration with the scheduler
//!
//! The intended dependency direction is:
//!
//! ```text
//! policies::policy
//!        ^
//!        |
//! policies::asap
//! policies::alap
//! policies::priority
//! policies::resource_aware
//! policies::hybrid
//!        ^
//!        |
//! scheduling planners
//!        ^
//!        |
//! scheduling algorithms
//!        ^
//!        |
//! scheduling pipeline
//! ```
//!
//! Policy definitions must not depend upward on planners or algorithms.
//!
//! This prevents cyclic module dependencies and allows policy contracts to be
//! completed independently before planner implementation begins.
//!
//! # Integration with `config.rs`
//!
//! `scheduling::config` may store a `SchedulingPolicy` value and associated
//! configuration.
//!
//! The policy module does not own the complete scheduler configuration.
//!
//! Configuration belongs to `config.rs` because configuration also includes:
//!
//! - verification mode;
//! - diagnostics;
//! - resource limits;
//! - timing behavior;
//! - deterministic execution settings;
//! - cancellation;
//! - optimization objectives;
//! - planner selection;
//! - serialization options.
//!
//! The policy is one component of that configuration, not the configuration
//! itself.
//!
//! # Integration with `context.rs`
//!
//! `SchedulingContext` supplies the policy selected for a particular scheduling
//! invocation.
//!
//! The context also supplies the target-dependent information required to
//! evaluate that policy, such as:
//!
//! - dependency information;
//! - timing model;
//! - resource model;
//! - constraints;
//! - calibration snapshot;
//! - target capabilities.
//!
//! Policies must not acquire any of this information themselves.
//!
//! This makes scheduling reproducible and prevents hidden hardware access.
//!
//! # Integration with planners
//!
//! Planners consume policy semantics.
//!
//! For example:
//!
//! ```text
//! SchedulingPolicyKind::AsSoonAsPossible
//!                 |
//!                 v
//!              asap planner
//! ```
//!
//! or:
//!
//! ```text
//! SchedulingPolicyKind::CriticalPathResourceAware
//!                 |
//!                 v
//!        critical-path analysis
//!                 |
//!                 v
//!        resource-aware planner
//! ```
//!
//! The policy selects intent.
//!
//! The planner owns the actual scheduling algorithm.
//!
//! # Integration with algorithms
//!
//! Algorithm implementations should depend on the policy contract rather than
//! directly on this module's re-export surface when they need a precise
//! dependency boundary.
//!
//! This makes the following possible without changing policy semantics:
//!
//! - list scheduling;
//! - critical-path scheduling;
//! - RCPSP-style scheduling;
//! - event-driven scheduling;
//! - adaptive scheduling;
//! - future exact algorithms;
//! - future approximation algorithms;
//! - future distributed scheduling.
//!
//! # Integration with resources
//!
//! Policies do not own resources.
//!
//! A resource-aware policy asks the scheduling context/resource subsystem for
//! resource information.
//!
//! It must never contain fields such as:
//!
//! ```text
//! control_channels: 8
//! measurement_channels: 4
//! qubits: 127
//! ```
//!
//! Such values would make the policy non-portable.
//!
//! The same policy value must be valid for:
//!
//! - one physical qubit;
//! - a small QPU;
//! - a large QPU;
//! - a modular QPU;
//! - a distributed quantum system;
//! - a future quantum architecture.
//!
//! # Integration with timing
//!
//! Policies consume timing information supplied by the scheduler's timing
//! subsystem.
//!
//! They must not hard-code:
//!
//! - nanoseconds;
//! - picoseconds;
//! - device ticks;
//! - clock periods;
//! - pulse durations;
//! - alignment grids.
//!
//! Timing resolution is target-specific.
//!
//! Policy semantics remain target-independent.
//!
//! # Integration with routing
//!
//! Routing answers:
//!
//! > Where should an operation execute?
//!
//! Scheduling policy answers:
//!
//! > What scheduling behavior should be preferred once the executable
//! > workload and resource mapping are known?
//!
//! Therefore policy code must not perform logical-to-physical mapping.
//!
//! Routing information enters scheduling through the scheduling context or an
//! explicit adapter.
//!
//! # Integration with QEC
//!
//! QEC-specific schedulers may select or compose policies, but policy code must
//! remain generic.
//!
//! For example, a QEC workload may benefit from:
//!
//! - critical-path scheduling;
//! - resource-aware scheduling;
//! - deterministic ordering;
//! - deadline-aware scheduling.
//!
//! The policy layer must not assume:
//!
//! - surface codes;
//! - a specific code distance;
//! - a fixed number of stabilizers;
//! - a fixed number of ancillas;
//! - a fixed number of rounds.
//!
//! QEC-specific information belongs to the QEC scheduling adapter.
//!
//! # Integration with distributed scheduling
//!
//! Policies must remain valid when resources span multiple nodes.
//!
//! A future distributed scheduler may use the same policy contract for:
//!
//! - local QPU scheduling;
//! - multi-chip scheduling;
//! - modular scheduling;
//! - multi-QPU scheduling;
//! - quantum-network scheduling.
//!
//! Communication resources, network topology and latency are supplied by the
//! distributed resource model.
//!
//! They are not embedded in the policy.
//!
//! # Integration with verification
//!
//! A policy expresses preference, not permission.
//!
//! The verifier remains authoritative for legality.
//!
//! Therefore:
//!
//! ```text
//! policy preference
//!       |
//!       v
//! planner
//!       |
//!       v
//! candidate schedule
//!       |
//!       v
//! verification
//!       |
//!       +---- invalid -> SchedulingError
//!       |
//!       +---- valid ---> ScheduleResult
//! ```
//!
//! A policy must never bypass:
//!
//! - dependency validation;
//! - resource validation;
//! - timing validation;
//! - target capability validation;
//! - semantic validation.
//!
//! # Integration with diagnostics
//!
//! Policy implementations should expose enough stable semantic information for
//! diagnostics to explain decisions.
//!
//! Diagnostics may report, for example:
//!
//! - selected policy;
//! - selected objective;
//! - determinism mode;
//! - tie-breaking strategy;
//! - why a candidate was preferred.
//!
//! Diagnostic formatting must not become part of scheduling semantics.
//!
//! # Integration with serialization
//!
//! Policy values are intended to be serializable through the scheduler's
//! serialization boundary.
//!
//! Stable machine-readable names are defined by `policy.rs`.
//!
//! Serialization code should use those stable names rather than Rust debug
//! formatting or source-level enum layout.
//!
//! This module deliberately does not introduce a serialization dependency.
//!
//! # Integration with plugins
//!
//! Custom policies are represented semantically by a stable custom-policy
//! identifier.
//!
//! Executable implementation lookup belongs to:
//!
//! `scheduling::plugins`
//!
//! or the appropriate scheduler integration boundary.
//!
//! This module must not store:
//!
//! - function pointers;
//! - global registries;
//! - mutable trait-object state;
//! - provider connections.
//!
//! This keeps policy values suitable for local and distributed scheduling
//! requests.
//!
//! # Determinism
//!
//! The policy layer must support deterministic scheduling.
//!
//! When deterministic mode is selected, the scheduler must ensure that all
//! policy-visible ordering decisions have stable tie-breaking.
//!
//! The policy module itself does not own the ready queue or graph traversal.
//!
//! It only exposes the semantic determinism contract consumed by those
//! components.
//!
//! # Scalability
//!
//! This module intentionally contains no machine-size-dependent allocation.
//!
//! Importing this module does not allocate storage proportional to:
//!
//! - qubit count;
//! - operation count;
//! - resource count;
//! - topology size;
//! - schedule duration.
//!
//! Policy values should remain compact and immutable.
//!
//! Workload-sized data belongs in scheduling contexts, planners and resource
//! models.
//!
//! # Concurrency
//!
//! Policy definitions are intended to be immutable values.
//!
//! Concrete policy implementations should be safe to share across scheduler
//! workers when their contained configuration permits it.
//!
//! This module must not introduce global mutable state.
//!
//! # Error ownership
//!
//! Policy validation errors belong to `policy.rs` and its policy-specific
//! contracts.
//!
//! Execution-time scheduling errors belong to the scheduling error subsystem.
//!
//! Hardware/provider errors belong to their owning hardware layer.
//!
//! This prevents generic scheduling errors from becoming an uncontrolled
//! cross-subsystem error hierarchy.
//!
//! # No hard-coded limits
//!
//! This module deliberately defines no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum schedule depth;
//! - maximum topology size;
//! - maximum number of policy candidates.
//!
//! If a caller needs operational limits, those belong to the scheduling
//! configuration/resource-limit subsystem.
//!
//! This is necessary for the requirement that a Zamani program be written once
//! and specialized to the resources available on the execution target.
//!
//! # Stable public surface
//!
//! The intended public policy imports are:
//!
//! ```text
//! crate::quantum::scheduling::policies::SchedulingPolicy
//! crate::quantum::scheduling::policies::SchedulingPolicyKind
//! crate::quantum::scheduling::policies::SchedulingObjective
//! crate::quantum::scheduling::policies::TieBreakRule
//! crate::quantum::scheduling::policies::Determinism
//! ```
//!
//! Concrete implementation modules remain available through their explicit
//! module paths where appropriate.
//!
//! # Adding a new policy
//!
//! The normal process is:
//!
//! 1. Add the policy implementation module.
//! 2. Implement the existing policy contract.
//! 3. Register it with the planner/algorithm/plugin boundary.
//! 4. Add unit tests.
//! 5. Add integration tests.
//! 6. Add determinism tests.
//! 7. Add scalability tests.
//! 8. Add diagnostics/serialization coverage.
//!
//! `mod.rs` should only need modification if the new policy module needs to be
//! publicly exposed as part of the stable module surface.
//!
//! The semantic policy contract must not be duplicated in the new module.
//!
//! # Compatibility with Rust
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! The crate-level scheduler safety policy is reinforced here with explicit
//! unsafe-code denial.
//!
//! # Module layout
//!
//! ```text
//! policies/
//! ├── mod.rs
//! ├── policy.rs
//! ├── asap.rs
//! ├── alap.rs
//! ├── priority.rs
//! ├── resource_aware.rs
//! └── hybrid.rs
//! ```
//!
//! `policy.rs` is the foundational contract.
//!
//! The remaining modules are concrete strategy implementations.
//!
//! # Why this file is intentionally small in executable code
//!
//! A module root should be a composition boundary rather than a second
//! implementation layer.
//!
//! Putting policy algorithms, resource models, hardware assumptions, or
//! scheduling state into `mod.rs` would make the file a coupling hotspot and
//! violate the requirement that individual scheduler components remain
//! independently complete.
//!
//! Consequently the executable content of this file is deliberately limited
//! to module declarations and stable re-exports.
//!
//! Future scheduler modules can therefore integrate with this file without
//! requiring semantic changes to existing policy implementations.
//!
//! # Integration invariants
//!
//! The following invariants apply to every module exposed here:
//!
//! 1. A policy never changes quantum semantics.
//! 2. A policy never invents hardware capacity.
//! 3. A policy never owns physical topology.
//! 4. A policy never owns canonical qubit identity.
//! 5. A policy never performs provider communication.
//! 6. A policy never bypasses verification.
//! 7. A policy never requires a fixed machine size.
//! 8. A policy never requires a fixed gate set.
//! 9. A policy never requires a fixed timing resolution.
//! 10. A policy never introduces unsafe Rust.
//! 11. A policy must remain compatible with deterministic execution when
//!     deterministic mode is selected.
//! 12. A custom policy must be identified independently of executable
//!     implementation state.
//!
//! These invariants are architectural contracts and should be preserved as the
//! scheduler evolves.
//!
//! # Public API
//!
//! The re-export surface below is deliberately explicit rather than using
//! wildcard exports. This prevents accidental expansion of the public API when
//! implementation details are added to a policy module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

pub mod policy;
pub mod asap;
pub mod alap;
pub mod priority;
pub mod resource_aware;
pub mod hybrid;

pub use policy::{
    Determinism,
    SchedulingObjective,
    SchedulingPolicy,
    SchedulingPolicyKind,
    TieBreakRule,
};

// Re-export the custom-policy identifier when it is part of the foundational
// policy contract. This keeps callers from depending on the internal module
// path while preserving the semantic ownership of policy.rs.
pub use policy::CustomPolicyId;