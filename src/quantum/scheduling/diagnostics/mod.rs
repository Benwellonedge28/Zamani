//! Zamani Quantum Scheduling — Diagnostics
//!
//! This module is the public composition boundary for diagnostics produced by
//! the quantum scheduling subsystem.
//!
//! # Architectural role
//!
//! Scheduling diagnostics answer questions such as:
//!
//! - What did the scheduler do?
//! - Why was an operation scheduled at a particular time?
//! - Why was an operation delayed?
//! - Which dependency prevented an operation from becoming ready?
//! - Which resource caused a conflict?
//! - Which timing or alignment constraint affected a decision?
//! - How much work did the scheduler perform?
//! - Where did planning or verification spend time?
//! - What happened during a dynamic or distributed scheduling epoch?
//!
//! Diagnostics are observational infrastructure.
//!
//! They do **not** own scheduling semantics and do **not** make scheduling
//! decisions.
//!
//! The architectural relationship is:
//!
//! ```text
//!                  Zamani Quantum IR
//!                         │
//!                         ▼
//!                       Routing
//!                         │
//!                         ▼
//!                     Scheduling
//!                         │
//!          ┌──────────────┼──────────────┐
//!          │              │              │
//!          ▼              ▼              ▼
//!       Planning      Verification   Transformation
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         │
//!                         ▼
//!                  Scheduling Diagnostics
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!           trace      explain      profile
//! ```
//!
//! # Submodules
//!
//! ## [`trace`]
//!
//! Structured, machine-readable scheduling events.
//!
//! It records events such as:
//!
//! - planner lifecycle;
//! - algorithm selection;
//! - dependency readiness/waits;
//! - resource conflicts;
//! - timing decisions;
//! - reservations;
//! - operation scheduling;
//! - verification;
//! - transformations;
//! - QEC-related scheduling;
//! - dynamic scheduling;
//! - distributed scheduling;
//! - capacity decisions;
//! - optimization decisions.
//!
//! `trace` is intended for event streams and structured observability.
//!
//! ## [`explain`]
//!
//! Human-oriented explanations of scheduling decisions.
//!
//! It should answer:
//!
//! > Why did this operation end up here?
//!
//! It is appropriate for:
//!
//! - compiler diagnostics;
//! - developer tooling;
//! - IDE/LSP integration;
//! - schedule inspection;
//! - debugging;
//! - user-facing explanations.
//!
//! ## [`profile`]
//!
//! Scheduler performance and planning-cost diagnostics.
//!
//! It should measure scheduler work without becoming the scheduler itself.
//!
//! Examples include:
//!
//! - dependency-analysis time;
//! - resource-analysis time;
//! - planning time;
//! - verification time;
//! - transformation time;
//! - event counts;
//! - conflict counts;
//! - graph sizes;
//! - retained diagnostic volume;
//! - allocation-independent logical counters;
//! - scheduler phase durations.
//!
//! # Ownership boundary
//!
//! This module does **not** define:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum operation identities;
//! - resource identities;
//! - dependency graphs;
//! - scheduling algorithms;
//! - scheduling policies;
//! - timing models;
//! - hardware capabilities;
//! - routing;
//! - QEC algorithms;
//! - runtime execution;
//! - logging backends;
//! - global metrics systems;
//! - global tracing state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical quantum identities
//!
//! Diagnostics must use canonical identities supplied by the quantum IR.
//!
//! Logical and physical qubit identity is owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operation and resource identity must come from the canonical IR identity
//! subsystem rather than being recreated inside diagnostics.
//!
//! This module intentionally imports none of those identities because it is a
//! module composition boundary. The concrete diagnostic implementations import
//! them only where needed.
//!
//! # Stable public API boundary
//!
//! This file deliberately exposes the diagnostic **modules**, rather than
//! individually re-exporting every type from their implementations.
//!
//! Therefore callers use:
//!
//! ```text
//! quantum::scheduling::diagnostics::trace::*
//! quantum::scheduling::diagnostics::explain::*
//! quantum::scheduling::diagnostics::profile::*
//! ```
//!
//! rather than requiring this file to know every public symbol contained in
//! those modules.
//!
//! This is intentional.
//!
//! It provides an important maintenance guarantee:
//!
//! > Adding, removing, renaming, or extending a diagnostic type inside one
//! > implementation module does not require reopening `diagnostics/mod.rs`
//! > merely to update re-exports.
//!
//! The diagnostic implementation files therefore remain independently
//! maintainable.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ├───────────────┐
//!      ▼               │
//! scheduling::types    │
//!      │               │
//!      ├───────────────┤
//!      ▼               │
//! scheduling internals │
//!      │               │
//!      ├───────────────┤
//!      ▼               │
//! diagnostics          │
//!      │               │
//!      ├── trace       │
//!      ├── explain     │
//!      └── profile     │
//!                      │
//!                      ▼
//!                 external consumers
//! ```
//!
//! Diagnostics must not become a dependency required by the core scheduling
//! algorithm merely to function.
//!
//! A production scheduler must be capable of operating with diagnostics
//! disabled, discarded, streamed, bounded, or retained according to the
//! caller's explicit configuration.
//!
//! # Non-interference invariant
//!
//! Diagnostics must never change quantum scheduling semantics.
//!
//! In particular:
//!
//! ```text
//! diagnostics enabled
//!        │
//!        ▼
//! scheduling decisions
//! ```
//!
//! and:
//!
//! ```text
//! diagnostics disabled
//!        │
//!        ▼
//! scheduling decisions
//! ```
//!
//! must produce semantically identical schedules when all other inputs are
//! identical.
//!
//! Diagnostic collection may have host-side performance and memory costs, but
//! those costs must never be used as implicit scheduling input.
//!
//! Diagnostics must not:
//!
//! - mutate the dependency graph;
//! - mutate the quantum IR;
//! - mutate resource availability;
//! - mutate timing constraints;
//! - mutate routing;
//! - allocate quantum hardware resources;
//! - execute quantum operations;
//! - modify QEC semantics;
//! - alter scheduler policy;
//! - alter scheduler objective values;
//! - introduce hidden randomness;
//! - introduce global mutable scheduler state.
//!
//! # Scalability
//!
//! Diagnostics must scale with the schedule being observed and must not impose
//! an artificial machine-size ceiling.
//!
//! This module therefore defines no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_EVENTS
//! MAX_TRACE_SIZE
//! ```
//!
//! There is no scheduler-level concept of a fixed machine size here.
//!
//! A concrete diagnostic collection may be bounded when the caller explicitly
//! requests bounded retention.
//!
//! For example:
//!
//! ```text
//! unlimited logical diagnostic stream
//!             │
//!             ▼
//!       caller-selected sink
//!             │
//!       ┌─────┼─────────┐
//!       ▼     ▼         ▼
//!    discard bounded  stream
//!              │         │
//!              ▼         ▼
//!          finite RAM  external sink
//! ```
//!
//! This distinction is essential for very large quantum programs.
//!
//! A billion-operation schedule must not require the diagnostics layer to
//! retain a billion events in memory merely because tracing is enabled.
//!
//! # Small-to-large execution
//!
//! The same diagnostics interface must work for:
//!
//! ```text
//! one operation
//!      │
//!      ▼
//! one qubit
//!      │
//!      ▼
//! one QPU
//!      │
//!      ▼
//! multi-chip systems
//!      │
//!      ▼
//! distributed quantum systems
//!      │
//!      ▼
//! arbitrarily large systems bounded only by available resources
//! ```
//!
//! The diagnostic API therefore describes events and relationships rather than
//! assuming a particular number of qubits, operations, resources, channels,
//! nodes, QEC rounds, or communication links.
//!
//! # Static versus dynamic scheduling
//!
//! Diagnostics must support both:
//!
//! ```text
//! static compilation
//! ```
//!
//! and:
//!
//! ```text
//! runtime/dynamic rescheduling
//! ```
//!
//! Dynamic scheduling can create multiple planning epochs.
//!
//! Diagnostics must therefore be capable of distinguishing events belonging to
//! different scheduling sessions and epochs without redefining quantum
//! operation identities.
//!
//! The concrete session/epoch identifiers belong to the scheduling subsystem's
//! foundational types and are consumed by the diagnostic implementations.
//!
//! # Distributed scheduling
//!
//! Diagnostics must not assume that a schedule belongs to one physical
//! machine.
//!
//! Events may describe:
//!
//! - local scheduling;
//! - cross-device dependencies;
//! - communication resources;
//! - entanglement generation;
//! - teleportation-related preparation;
//! - classical communication;
//! - synchronization;
//! - distributed scheduling epochs;
//! - node-local planning.
//!
//! The distributed subsystem remains the owner of distributed scheduling
//! semantics. Diagnostics only observe those semantics.
//!
//! # QEC integration
//!
//! QEC scheduling diagnostics are supported through the diagnostic event
//! categories exposed by the implementation modules.
//!
//! Examples include:
//!
//! - syndrome extraction;
//! - stabilizer interactions;
//! - ancilla readiness;
//! - measurement dependencies;
//! - QEC rounds;
//! - recovery dependencies;
//! - decoder readiness;
//! - fault-tolerant scheduling constraints.
//!
//! QEC-specific semantics remain owned by `scheduling::qec` and the broader
//! quantum error-correction subsystem.
//!
//! # Routing integration
//!
//! Routing answers:
//!
//! > Where should the computation execute?
//!
//! Scheduling answers:
//!
//! > When should it execute?
//!
//! Diagnostics may record information from both boundaries, but diagnostics do
//! not perform either operation.
//!
//! The intended flow is:
//!
//! ```text
//! canonical quantum IR
//!        │
//!        ▼
//! routing
//!        │
//!        ▼
//! mapped IR
//!        │
//!        ▼
//! scheduling
//!        │
//!        ▼
//! diagnostics
//! ```
//!
//! # Hardware integration
//!
//! Hardware capabilities are supplied by the hardware subsystem and consumed by
//! scheduling through its established adapter/context boundaries.
//!
//! Diagnostics may report information such as:
//!
//! - timing resolution;
//! - resource availability;
//! - resource conflicts;
//! - alignment constraints;
//! - calibration-derived scheduling decisions;
//! - target compatibility;
//! - communication availability.
//!
//! Diagnostics must not perform hardware discovery or hardware execution.
//!
//! # ZQN/noise integration
//!
//! If scheduling consumes noise or uncertainty information through the quantum
//! noise subsystem, diagnostics may record the resulting scheduling decisions.
//!
//! Diagnostics must not independently invent a second noise model.
//!
//! For example:
//!
//! ```text
//! ZQN
//!  │
//!  ├── duration uncertainty
//!  ├── gate error
//!  ├── drift
//!  ├── crosstalk
//!  └── calibration uncertainty
//!          │
//!          ▼
//!      scheduling
//!          │
//!          ▼
//!      diagnostics
//! ```
//!
//! # Benchmarking integration
//!
//! The benchmarking subsystem may consume diagnostic/profile information to
//! determine scheduling cost and quality.
//!
//! Diagnostics must remain observational so that benchmarking can distinguish:
//!
//! ```text
//! schedule quality
//! ```
//!
//! from:
//!
//! ```text
//! scheduler implementation cost
//! ```
//!
//! Useful values may include:
//!
//! - planning duration;
//! - verification duration;
//! - transformation duration;
//! - dependency count;
//! - resource conflict count;
//! - scheduling event count;
//! - critical-path-related observations;
//! - schedule makespan observations;
//! - resource utilization observations.
//!
//! The canonical schedule result remains owned by the scheduling result layer.
//!
//! # Serialization boundary
//!
//! Diagnostics may be serialized by the scheduling serialization subsystem,
//! but this module does not define the serialization format.
//!
//! This prevents diagnostic implementation details from becoming an accidental
//! wire-format contract.
//!
//! Versioned serialization belongs under:
//!
//! ```text
//! scheduling::serialization
//! ```
//!
//! # Logging boundary
//!
//! Diagnostics are not a logging backend.
//!
//! The distinction is:
//!
//! ```text
//! scheduling decision
//!        │
//!        ▼
//! diagnostic event
//!        │
//!        ├── in-memory consumer
//!        ├── bounded consumer
//!        ├── streaming consumer
//!        ├── logger
//!        ├── debugger
//!        ├── profiler
//!        └── benchmark system
//! ```
//!
//! No global logger or global tracing state is created here.
//!
//! # Thread safety
//!
//! This module itself contains no mutable state.
//!
//! Thread safety and synchronization requirements belong to the concrete
//! diagnostic collector/sink implementations and their callers.
//!
//! A scheduler running concurrently must not obtain correctness from a hidden
//! global diagnostic lock.
//!
//! When deterministic output is requested, diagnostic ordering must be derived
//! from explicit event/session sequencing rather than hash-map iteration order.
//!
//! # Determinism
//!
//! Diagnostic infrastructure must preserve deterministic event ordering when
//! the scheduler itself is deterministic.
//!
//! It must not use:
//!
//! - randomized ordering;
//! - hash-map iteration as semantic ordering;
//! - system clock values as scheduler decisions;
//! - process-global counters;
//! - global mutable state.
//!
//! Host timing measurements used by profiling are observational and must never
//! become scheduler semantics.
//!
//! # Error handling
//!
//! Diagnostic failure must have an explicit policy.
//!
//! A production scheduler must be able to distinguish between:
//!
//! ```text
//! diagnostic failure
//! ```
//!
//! and:
//!
//! ```text
//! scheduling failure
//! ```
//!
//! A diagnostic sink that fails must not silently corrupt the schedule.
//!
//! The concrete diagnostic API is responsible for exposing the appropriate
//! result/error contract. This composition module deliberately does not
//! duplicate those error definitions.
//!
//! # Memory discipline
//!
//! The diagnostics boundary supports both:
//!
//! ```text
//! retained diagnostics
//! ```
//!
//! and:
//!
//! ```text
//! streaming diagnostics
//! ```
//!
//! Therefore consumers should not assume that all events are retained.
//!
//! A bounded collector may intentionally discard or summarize events after its
//! configured retention boundary.
//!
//! The absence of a retained event must not imply that the scheduler never
//! made the corresponding decision.
//!
//! # API stability rule
//!
//! This module should change only when the diagnostics module topology changes.
//!
//! It should **not** be modified merely because:
//!
//! - a new trace event is added;
//! - a new profile metric is added;
//! - an explanation type changes;
//! - a trace field changes;
//! - a diagnostic implementation is optimized;
//! - a new scheduler algorithm is added;
//! - a new hardware target is supported;
//! - a new QEC implementation is added;
//! - a new distributed backend is added.
//!
//! Such changes belong in the relevant implementation module.
//!
//! This keeps the composition boundary stable.
//!
//! # Public module contract
//!
//! The public diagnostic namespace is:
//!
//! ```text
//! quantum::scheduling::diagnostics
//! ├── trace
//! ├── explain
//! └── profile
//! ```
//!
//! The three modules are deliberately public so that higher-level components
//! can select the appropriate diagnostic surface without requiring this module
//! to duplicate their APIs.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! Unsafe code is explicitly forbidden at this module boundary.
//!
//! Child modules are independently required to maintain the same project-wide
//! safety contract.
//!
//! # Integration examples
//!
//! ## Structured tracing
//!
//! ```text
//! scheduling::planners
//!       │
//!       ▼
//! diagnostics::trace
//!       │
//!       ▼
//! trace sink
//! ```
//!
//! ## Human explanation
//!
//! ```text
//! SchedulingResult
//!       │
//!       ▼
//! diagnostics::explain
//!       │
//!       ▼
//! compiler / IDE / CLI
//! ```
//!
//! ## Performance profiling
//!
//! ```text
//! scheduler
//!       │
//!       ▼
//! diagnostics::profile
//!       │
//!       ▼
//! benchmarking / compiler telemetry
//! ```
//!
//! ## Combined diagnostic pipeline
//!
//! ```text
//!                         Scheduler
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!           trace         explain       profile
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                     external consumers
//! ```
//!
//! # Why this module does not re-export every symbol
//!
//! It may be tempting to write:
//!
//! ```text
//! pub use trace::*;
//! pub use explain::*;
//! pub use profile::*;
//! ```
//!
//! That is deliberately avoided.
//!
//! Globally flattening three independently evolving diagnostic APIs creates:
//!
//! - namespace collisions;
//! - accidental public API commitments;
//! - ambiguity between diagnostic concepts;
//! - harder documentation;
//! - unnecessary coupling;
//! - future migration pressure.
//!
//! Instead, the stable boundary is explicit:
//!
//! ```text
//! diagnostics::trace::...
//! diagnostics::explain::...
//! diagnostics::profile::...
//! ```
//!
//! This is safer for a production compiler/runtime ecosystem.
//!
//! # Module declarations
//!
//! These declarations are intentionally the only executable composition logic
//! in this file.
//!
//! Adding a new diagnostic implementation should normally require adding a
//! module declaration here, while modifying an existing implementation should
//! not require changing this file.
//!
//! # Production invariants
//!
//! This module guarantees the following architectural properties:
//!
//! 1. Diagnostics are separated from scheduling decisions.
//! 2. No quantum identity is redefined.
//! 3. No fixed machine-size limit is introduced.
//! 4. No fixed qubit count is introduced.
//! 5. No fixed operation count is introduced.
//! 6. No fixed resource count is introduced.
//! 7. No hardware timing constant is introduced.
//! 8. Static scheduling is supported.
//! 9. Dynamic scheduling is supported.
//! 10. Distributed scheduling can be observed.
//! 11. QEC scheduling can be observed.
//! 12. Routing integration remains external.
//! 13. Hardware integration remains external.
//! 14. Serialization remains external.
//! 15. Benchmarking remains external.
//! 16. Logging remains external.
//! 17. Global mutable diagnostic state is not introduced.
//! 18. Diagnostic implementation APIs remain namespaced.
//! 19. Diagnostic implementation changes do not require this file to be
//!     repeatedly rewritten.
//! 20. Unsafe code is forbidden at this module boundary.
//!
//! # Final architectural rule
//!
//! The diagnostics subsystem observes the scheduler; it never becomes the
//! scheduler.
//!
//! The scheduler remains responsible for:
//!
//! ```text
//! dependencies
//! resources
//! timing
//! constraints
//! planning
//! transformations
//! verification
//! ```
//!
//! Diagnostics remains responsible for:
//!
//! ```text
//! trace
//! explanation
//! profiling
//! ```
//!
//! This separation is what allows the same Zamani program to scale from a tiny
//! target to extremely large quantum systems without embedding target-specific
//! assumptions into the diagnostic architecture.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Structured, machine-readable scheduling events and trace sessions.
///
/// This module observes scheduler activity without owning scheduling
/// semantics.
pub mod trace;

/// Human-readable explanations of scheduling decisions.
///
/// This module converts scheduler state/diagnostic information into explanations
/// suitable for compiler tooling, debugging, inspection, and user-facing
/// diagnostics.
pub mod explain;

/// Scheduler performance and planning-cost profiling.
///
/// This module observes scheduler work and exposes profiling information
/// without becoming part of scheduler correctness.
pub mod profile;