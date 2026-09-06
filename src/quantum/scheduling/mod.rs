//! Zamani Quantum Scheduling
//!
//! Production scheduling subsystem for Zamani's quantum computing stack.
//!
//! # Purpose
//!
//! This module is the **composition root** of:
//!
//! ```text
//! crate::quantum::scheduling
//! ```
//!
//! It owns the namespace and dependency boundaries for quantum scheduling.
//! It does not itself implement scheduling algorithms, hardware execution,
//! routing, quantum semantics, QEC decoding, noise modelling, or serialization.
//!
//! The scheduler answers:
//!
//! > When can an executable quantum operation occur?
//!
//! It does not answer:
//!
//! > What does the quantum program mean?
//!
//! That answer belongs to:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! It also does not answer:
//!
//! > Where should a logical operation execute?
//!
//! That answer belongs to:
//!
//! ```text
//! crate::quantum::routing
//! ```
//!
//! The intended compilation boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      ▼
//! quantum::scheduling
//!      │
//!      ├── dependency analysis
//!      ├── resource analysis
//!      ├── timing analysis
//!      ├── constraints
//!      ├── scheduling policy
//!      ├── scheduling algorithm
//!      ├── verification
//!      └── schedule optimization
//!      │
//!      ▼
//! quantum::error_correction / fault-tolerant lowering
//!      │
//!      ▼
//! quantum::hardware
//!      │
//!      ▼
//! runtime
//! ```
//!
//! # Write once, scale everywhere
//!
//! Zamani quantum programs must describe computation rather than the physical
//! size of the machine on which the computation eventually executes.
//!
//! Consequently, this namespace introduces **no artificial machine-size
//! ceiling**.
//!
//! It must not encode:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_CHANNELS
//! MAX_RESOURCES
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_TOPOLOGY
//! VENDOR_QUBIT_COUNT
//! ```
//!
//! A concrete invocation is naturally bounded by available resources such as:
//!
//! - address space;
//! - memory;
//! - compilation time;
//! - target capacity;
//! - explicit caller limits;
//! - operating-system limits;
//! - execution deadlines;
//! - provider limitations.
//!
//! Those are **runtime/resource constraints**, not semantic limits of Zamani.
//!
//! Therefore:
//!
//! ```text
//! same Zamani program
//!        │
//!        ├── small target
//!        ├── medium target
//!        ├── large target
//!        ├── distributed target
//!        └── future target
//! ```
//!
//! may produce different physical schedules while preserving the same
//! computation semantics.
//!
//! # Canonical qubit identity
//!
//! Scheduling MUST use the canonical qubit identity types owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Specifically:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not define or alias either type.
//!
//! A scheduling-specific identity is not a replacement for a canonical qubit
//! identity.
//!
//! The distinction is fundamental:
//!
//! ```text
//! QubitId
//!     = quantum-program / canonical semantic identity
//!
//! PhysicalQubitId
//!     = physical-target identity
//!
//! ScheduleId
//!     = identity of a schedule artifact
//!
//! OperationId
//!     = canonical IR operation identity
//!
//! ResourceId
//!     = canonical resource identity
//! ```
//!
//! Scheduling must never silently convert a logical qubit into a physical
//! qubit. Logical-to-physical mapping remains the responsibility of routing.
//!
//! # Canonical operation and resource identities
//!
//! Scheduler-owned foundational types must reuse canonical IR identities where
//! the identity belongs to the quantum IR.
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! must not be duplicated by this module.
//!
//! Scheduler-specific identities such as `ScheduleId`, `DependencyId`,
//! `ReservationId`, and `EpochId` are distinct because they describe scheduler
//! artifacts rather than quantum-semantic objects.
//!
//! # Responsibility boundaries
//!
//! ## `types`
//!
//! Foundational scheduler-owned values and identities.
//!
//! This includes scheduler-specific:
//!
//! - schedule identity;
//! - dependency identity;
//! - reservation identity;
//! - scheduling epoch identity;
//! - abstract time;
//! - duration;
//! - priority;
//! - cost;
//! - schedule status;
//! - scheduling phase;
//! - scheduler references.
//!
//! It must not define:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - canonical `OperationId`;
//! - canonical `ResourceId`;
//! - hardware topology;
//! - vendor resources.
//!
//! ## `errors`
//!
//! Canonical scheduling error taxonomy.
//!
//! Errors must remain structured and machine-readable rather than relying on
//! error-message parsing.
//!
//! ## `limits`
//!
//! Explicit invocation/resource limits.
//!
//! Limits here are caller/deployment policy, not universal machine-size
//! constants.
//!
//! ## `config`
//!
//! Immutable configuration describing how one scheduling invocation should
//! operate.
//!
//! Configuration must not contain assumptions about a particular machine size,
//! topology, vendor, or technology.
//!
//! ## `context`
//!
//! Immutable scheduling input context.
//!
//! The context connects:
//!
//! ```text
//! executable program
//! +
//! target information
//! + 
//! timing model
//! +
//! resource model
//! +
//! constraints
//! +
//! policy
//! +
//! objective
//! ```
//!
//! The context is supplied by callers/adapters. Scheduling must not discover
//! hardware by itself.
//!
//! ## `result`
//!
//! Canonical scheduling output.
//!
//! A result may contain:
//!
//! - scheduled operations;
//! - start times;
//! - finish times;
//! - resource reservations;
//! - makespan;
//! - depth;
//! - idle intervals;
//! - critical-path information;
//! - objective metrics;
//! - verification information;
//! - provenance;
//! - diagnostics.
//!
//! ## `ir`
//!
//! Internal scheduling representation.
//!
//! This is deliberately **not** a second quantum semantic IR.
//!
//! The ownership boundary is:
//!
//! ```text
//! quantum::ir
//!       │
//!       ▼
//! scheduling::adapters::ir
//!       │
//!       ▼
//! scheduling::ir
//! ```
//!
//! The scheduling IR exists only to represent scheduling-relevant information
//! efficiently and explicitly.
//!
//! ## `resources`
//!
//! Generic resource modelling.
//!
//! A quantum computer is not merely a set of qubits.
//!
//! Resources may include:
//!
//! - logical qubits;
//! - physical qubits;
//! - ancillas;
//! - control channels;
//! - drive channels;
//! - measurement channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - microwave sources;
//! - optical channels;
//! - communication links;
//! - classical processors;
//! - classical memory;
//! - accelerators;
//! - synchronization resources;
//! - composite resources;
//! - future target-defined resources.
//!
//! Resource counts must be target supplied.
//!
//! ## `timing`
//!
//! Target-independent timing abstractions.
//!
//! Timing must support:
//!
//! - abstract time;
//! - duration;
//! - timing resolution;
//! - alignment;
//! - availability windows;
//! - deadlines;
//! - release times;
//! - timing constraints.
//!
//! No scheduler-level constant may assume nanoseconds, device ticks,
//! pulse-sample periods, or another particular physical clock.
//!
//! ## `policies`
//!
//! Declarative scheduling policies.
//!
//! Examples include:
//!
//! - ASAP;
//! - ALAP;
//! - priority-based;
//! - resource-aware;
//! - hybrid policies.
//!
//! A policy specifies scheduling preferences/constraints. It does not own the
//! core scheduler state.
//!
//! ## `planners`
//!
//! Stable planner contracts and planner implementations.
//!
//! Planners transform a validated scheduling context into a candidate schedule.
//!
//! ## `algorithms`
//!
//! Concrete scheduling algorithms.
//!
//! Examples include:
//!
//! - ASAP;
//! - ALAP;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - adaptive scheduling.
//!
//! Algorithm modules must remain replaceable and must not become hardware
//! vendor implementations.
//!
//! ## `constraints`
//!
//! Generic scheduling constraints.
//!
//! Examples include:
//!
//! - qubit exclusivity;
//! - channel capacity;
//! - measurement constraints;
//! - reset constraints;
//! - control dependencies;
//! - communication constraints;
//! - target-defined constraints.
//!
//! ## `transformations`
//!
//! Scheduling-stage transformations such as:
//!
//! - explicit delays;
//! - alignment;
//! - padding;
//! - optional dynamical decoupling.
//!
//! These transformations must preserve the semantic contract established by
//! the canonical quantum IR.
//!
//! ## `verification`
//!
//! Independent schedule verification.
//!
//! Verification must establish, at minimum:
//!
//! ```text
//! dependencies are respected
//! resources are not illegally oversubscribed
//! timing constraints are respected
//! alignment constraints are respected
//! target capabilities are respected
//! schedule arithmetic is valid
//! scheduled semantics remain equivalent to the input
//! ```
//!
//! A production scheduler must not rely solely on the algorithm's internal
//! invariants.
//!
//! ## `optimization`
//!
//! Scheduling-objective evaluation and schedule-level optimization.
//!
//! Objectives may include:
//!
//! - makespan;
//! - depth;
//! - idle time;
//! - fidelity estimates;
//! - energy;
//! - communication overhead;
//! - multi-objective cost.
//!
//! Objective weights must be explicit configuration, not hidden constants.
//!
//! ## `qec`
//!
//! Scheduling-facing QEC contracts.
//!
//! QEC supplies scheduling requirements such as:
//!
//! - syndrome dependencies;
//! - ancilla requirements;
//! - round constraints;
//! - measurement dependencies;
//! - feedback requirements.
//!
//! QEC decoding and QEC algorithm ownership remain in the quantum error
//! correction subsystem.
//!
//! ## `dynamic`
//!
//! Scheduling contracts for programs whose future operations depend on runtime
//! information.
//!
//! This includes:
//!
//! - classical dependencies;
//! - conditional operations;
//! - measurement-to-feedback latency;
//! - runtime events;
//! - dynamic scheduling.
//!
//! The scheduler must therefore not assume every quantum program is a static
//! DAG known completely before execution.
//!
//! ## `distributed`
//!
//! Distributed scheduling abstractions.
//!
//! These allow the same scheduling model to represent:
//!
//! ```text
//! one device
//!     ↓
//! multi-chip
//!     ↓
//! multi-module
//!     ↓
//! multi-QPU
//!     ↓
//! quantum network
//! ```
//!
//! Distributed communication and synchronization are represented as resources
//! and constraints rather than hidden side effects.
//!
//! ## `adapters`
//!
//! Explicit integration boundaries between scheduling and other quantum
//! subsystems.
//!
//! Important adapters include:
//!
//! ```text
//! adapters::ir
//! adapters::routing
//! adapters::hardware
//! adapters::qec
//! ```
//!
//! Vendor-specific code must remain outside the scheduler core.
//!
//! ## `serialization`
//!
//! Versioned schedule serialization.
//!
//! Serialization must not become a second quantum IR.
//!
//! It serializes scheduler-owned artifacts only.
//!
//! ## `diagnostics`
//!
//! Explanations, traces, and profiling information.
//!
//! A production scheduler should be able to answer questions such as:
//!
//! > Why was this operation delayed?
//!
//! Examples:
//!
//! ```text
//! resource occupied
//! dependency incomplete
//! alignment requirement
//! measurement latency
//! communication latency
//! deadline constraint
//! target availability
//! policy preference
//! ```
//!
//! ## `plugins`
//!
//! Explicit extension points for external scheduling planners/algorithms.
//!
//! Plugins must use stable contracts and must not mutate global scheduler
//! state.
//!
//! ## `stabilizer_scheduler`
//!
//! Compatibility facade for historical stabilizer/QEC scheduling callers.
//!
//! It must not contain a second scheduling algorithm.
//!
//! Historical callers may still use the compatibility configuration while new
//! callers should use the generic QEC scheduling interfaces.
//!
//! ## `tests`
//!
//! Scheduler-specific test composition.
//!
//! The test hierarchy covers:
//!
//! - unit tests;
//! - integration tests;
//! - property tests;
//! - regression tests;
//! - scalability tests;
//! - determinism tests;
//! - fixtures.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                  scheduling::adapters::ir
//!                             │
//!                             ▼
//!                      scheduling::ir
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!          resources       timing        constraints
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                          context
//!                             │
//!                             ▼
//!                          policies
//!                             │
//!                             ▼
//!                          planners
//!                             │
//!                             ▼
//!                         algorithms
//!                             │
//!                             ▼
//!                       transformations
//!                             │
//!                             ▼
//!                        verification
//!                             │
//!                             ▼
//!                         result
//! ```
//!
//! Cross-subsystem integration occurs through adapters:
//!
//! ```text
//! quantum::routing
//!       │
//!       ▼
//! scheduling::adapters::routing
//!       │
//!       ▼
//! scheduling
//!
//! quantum::hardware
//!       │
//!       ▼
//! scheduling::adapters::hardware
//!       │
//!       ▼
//! scheduling
//!
//! quantum::error_correction
//!       │
//!       ▼
//! scheduling::adapters::qec
//!       │
//!       ▼
//! scheduling
//! ```
//!
//! The scheduler must not depend on a concrete vendor backend.
//!
//! # Composition-root rule
//!
//! This file intentionally contains:
//!
//! - module declarations;
//! - subsystem documentation;
//! - test gating;
//! - narrowly justified compatibility exposure.
//!
//! It intentionally does **not** contain:
//!
//! - scheduling algorithms;
//! - quantum gates;
//! - qubit allocation;
//! - topology definitions;
//! - hardware discovery;
//! - QPU connections;
//! - network access;
//! - filesystem access;
//! - random-number generation;
//! - global mutable state;
//! - serialization implementation;
//! - timing algorithms;
//! - resource calendars;
//! - QEC decoding.
//!
//! This keeps the root stable when individual implementations evolve.
//!
//! # Public API policy
//!
//! The module tree is exposed explicitly.
//!
//! Wildcard re-exports such as:
//!
//! ```text
//! pub use algorithms::*;
//! pub use ir::*;
//! pub use hardware::*;
//! ```
//!
//! are deliberately forbidden here.
//!
//! Explicit namespace-qualified access is preferred:
//!
//! ```text
//! crate::quantum::scheduling::types
//! crate::quantum::scheduling::timing
//! crate::quantum::scheduling::resources
//! crate::quantum::scheduling::planners
//! ```
//!
//! This prevents accidental API collisions and allows child modules to evolve
//! without requiring unrelated changes to this composition root.
//!
//! # Safety
//!
//! The scheduling subsystem is safe Rust.
//!
//! This root explicitly forbids unsafe Rust. Child modules should enforce the
//! same boundary independently.
//!
//! No unsafe code, raw-pointer scheduler state, mutable global state, or
//! unsafe FFI belongs in the scheduling namespace.
//!
//! # Rust compatibility
//!
//! This module is intended for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly-only features;
//! - no `unsafe` code.
//!
//! # Scalability
//!
//! The root itself performs no allocation and introduces no size-dependent
//! data structure.
//!
//! Scalability is implemented by the child modules using:
//!
//! - sparse representations;
//! - event-driven scheduling;
//! - dependency graphs;
//! - resource calendars;
//! - checked arithmetic;
//! - incremental analysis where appropriate;
//! - iterative traversal for large graphs;
//! - explicit resource budgets;
//! - optional parallel planning;
//! - distributed planning contracts.
//!
//! The semantic model therefore remains independent of machine size.
//!
//! # Determinism
//!
//! This module performs no scheduling decisions.
//!
//! Determinism belongs to the scheduler configuration/planner contract.
//!
//! A deterministic invocation should be reproducible from the complete input
//! context, including any explicitly supplied seed and target/calibration
//! snapshot.
//!
//! # Thread safety
//!
//! This composition root contains no mutable runtime state.
//!
//! Child modules own their own concurrency contracts.
//!
//! The root deliberately does not introduce global synchronization.
//!
//! # Versioning
//!
//! This module does not invent a second quantum version number.
//!
//! Individual scheduler schemas and APIs own their versioning. Repository-level
//! Rust/package compatibility remains governed by the workspace configuration.
//!
//! # Integration invariant
//!
//! The complete scheduler must preserve the following fundamental separation:
//!
//! ```text
//! IR          = WHAT
//! routing     = WHERE
//! scheduling  = WHEN
//! hardware    = CAN IT EXECUTE?
//! runtime     = EXECUTE
//! ```
//!
//! This separation is what allows a Zamani program to be written once and
//! specialized for targets ranging from very small machines to large and
//! distributed quantum systems.
//!
//! # Module declarations
//!
//! These declarations correspond to the production scheduling architecture.
//!
//! Keep this section synchronized only with actual completed child module
//! boundaries. A directory must not be declared merely because it exists on
//! disk; it must contain a valid Rust module boundary.
//!
//! -----------------------------------------------------------------------------
//! Foundational contracts
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod types;
pub mod errors;
pub mod limits;
pub mod config;
pub mod context;
pub mod result;

// -----------------------------------------------------------------------------
// Scheduling IR
// -----------------------------------------------------------------------------

pub mod ir;

// -----------------------------------------------------------------------------
// Resources and timing
// -----------------------------------------------------------------------------

pub mod resources;
pub mod timing;

// -----------------------------------------------------------------------------
// Constraints and scheduling policy
// -----------------------------------------------------------------------------

pub mod constraints;
pub mod policies;

// -----------------------------------------------------------------------------
// Planning and algorithms
// -----------------------------------------------------------------------------

pub mod planners;
pub mod algorithms;

// -----------------------------------------------------------------------------
// Schedule transformations and verification
// -----------------------------------------------------------------------------

pub mod transformations;
pub mod verification;

// -----------------------------------------------------------------------------
// Scheduling optimization
// -----------------------------------------------------------------------------

pub mod optimization;

// -----------------------------------------------------------------------------
// QEC, dynamic, and distributed scheduling
// -----------------------------------------------------------------------------

pub mod qec;
pub mod dynamic;
pub mod distributed;

// -----------------------------------------------------------------------------
// Explicit integration boundaries
// -----------------------------------------------------------------------------

pub mod adapters;

// -----------------------------------------------------------------------------
// Persistence and observability
// -----------------------------------------------------------------------------

pub mod serialization;
pub mod diagnostics;

// -----------------------------------------------------------------------------
// Extension mechanism
// -----------------------------------------------------------------------------

pub mod plugins;

// -----------------------------------------------------------------------------
// Legacy/compatibility boundary
// -----------------------------------------------------------------------------

pub mod stabilizer_scheduler;

// -----------------------------------------------------------------------------
// Scheduler test composition
// -----------------------------------------------------------------------------
//
// Tests are compiled only when the crate is being tested. This prevents the
// production library from carrying test-only module dependencies.

#[cfg(test)]
mod tests;