//! Zamani Quantum Scheduling — Constraint System
//!
//! This module is the public composition boundary for scheduling constraints.
//!
//! # Architectural responsibility
//!
//! `quantum::scheduling::constraints` answers:
//!
//! > "Under what conditions is a proposed quantum schedule legal, valid,
//! > executable, and semantically safe for the selected target and execution
//! > context?"
//!
//! It does NOT answer:
//!
//! - which logical-to-physical mapping should be selected;
//! - how operations should be ordered globally;
//! - which scheduling algorithm should be used;
//! - how hardware is contacted;
//! - how calibration is obtained;
//! - how QEC syndromes are decoded;
//! - how source syntax is parsed;
//! - how quantum semantics are defined;
//! - how pulses are generated;
//! - how a backend executes a job.
//!
//! Those responsibilities belong to the corresponding Zamani subsystems.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! scheduling::constraints
//!        │
//!        ├── qubit constraints
//!        ├── channel constraints
//!        ├── measurement constraints
//!        ├── reset constraints
//!        ├── control/feedback constraints
//!        ├── communication constraints
//!        └── target/custom constraints
//!        │
//!        ▼
//! scheduling planner
//!        │
//!        ▼
//! scheduled program
//! ```
//!
//! The dependency direction MUST NOT be reversed.
//!
//! In particular, this module must not depend on:
//!
//! - scheduling algorithms;
//! - routing algorithms;
//! - optimization passes;
//! - frontend parsers;
//! - backend/vendor SDKs;
//! - credentials;
//! - network clients;
//! - runtime execution;
//! - simulator state;
//! - QEC decoder implementations.
//!
//! # Canonical qubit identities
//!
//! Scheduling constraints operate on the canonical quantum IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No scheduling-local `QubitId` or `PhysicalQubitId` may be introduced.
//!
//! Logical and physical identities are intentionally different types.
//!
//! ```text
//! QubitId
//!     │
//!     └── logical semantic identity
//!
//! PhysicalQubitId
//!     │
//!     └── physical target identity
//! ```
//!
//! The scheduler may consume both identities, but it must never silently
//! convert one into the other.
//!
//! This is consistent with the canonical IR contract, where `quantum::ir`
//! explicitly owns qubit identity and downstream systems consume it. The
//! scheduling subsystem must therefore use the canonical `qubit` module
//! rather than creating competing identity types.
//!
//! # Write once, scale everywhere
//!
//! Constraint definitions MUST NOT contain architectural limits such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_NODES
//! ```
//!
//! A constraint is evaluated against the resources supplied by the scheduling
//! context.
//!
//! Therefore the same constraint implementation can operate on:
//!
//! ```text
//! one qubit
//! ten qubits
//! thousands of qubits
//! millions of qubits
//! distributed quantum systems
//! ```
//!
//! subject only to the resources and explicit limits supplied by the caller.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. Every concrete compilation remains finite because the program,
//! host memory, execution target, and available resources are finite.
//!
//! # Constraint categories
//!
//! The subsystem is divided by responsibility:
//!
//! ```text
//! constraint.rs
//!     generic constraint contract
//!
//! qubit.rs
//!     quantum/physical qubit occupancy constraints
//!
//! channel.rs
//!     control/readout/shared-channel constraints
//!
//! measurement.rs
//!     measurement ordering and resource constraints
//!
//! reset.rs
//!     reset readiness and reuse constraints
//!
//! control.rs
//!     classical conditions and feedback constraints
//!
//! communication.rs
//!     intra-device/inter-device/network communication constraints
//!
//! custom.rs
//!     extensible target/application-defined constraints
//! ```
//!
//! Keeping these responsibilities separate prevents a single constraint file
//! from becoming a monolithic hardware-specific scheduler.
//!
//! # Constraint versus scheduling
//!
//! A constraint answers:
//!
//! ```text
//! "Is this candidate schedule legal?"
//! ```
//!
//! A scheduler answers:
//!
//! ```text
//! "Which legal schedule should we choose?"
//! ```
//!
//! Therefore constraint evaluation must not contain an optimization policy.
//!
//! For example, a qubit constraint may reject overlapping operations on an
//! exclusive physical qubit. It must not decide whether ASAP or ALAP is better.
//!
//! # Constraint versus hardware
//!
//! Hardware owns actual target capabilities and availability.
//!
//! Constraints consume target-derived facts through the scheduling context.
//!
//! Correct dependency direction:
//!
//! ```text
//! quantum::hardware
//!        │
//!        ▼
//! target capabilities / resource description
//!        │
//!        ▼
//! scheduling context
//!        │
//!        ▼
//! scheduling constraints
//! ```
//!
//! A constraint must not directly contact a QPU or vendor SDK.
//!
//! # Constraint versus routing
//!
//! Routing determines:
//!
//! ```text
//! logical qubit -> physical qubit
//! ```
//!
//! Constraints validate or enforce the consequences of that mapping.
//!
//! They must not independently implement routing.
//!
//! # Constraint versus optimization
//!
//! Constraints define feasibility.
//!
//! Optimization defines preference.
//!
//! For example:
//!
//! ```text
//! Constraint:
//!     two operations cannot occupy an exclusive control channel
//!
//! Objective:
//!     minimize total control-channel idle time
//! ```
//!
//! These concepts must remain separate.
//!
//! # Static and dynamic constraints
//!
//! The constraint API must support both:
//!
//! ```text
//! static constraints
//!     known before scheduling
//!
//! dynamic constraints
//!     dependent on runtime events, measurements,
//!     feedback, communication, or changing availability
//! ```
//!
//! A constraint implementation must therefore not assume that the complete
//! execution timeline is known before scheduling begins.
//!
//! # Determinism
//!
//! Constraint evaluation must be deterministic for deterministic input.
//!
//! It must not:
//!
//! - depend on hash-map iteration order;
//! - use hidden randomness;
//! - depend on wall-clock time;
//! - query mutable global state;
//! - use process-global configuration.
//!
//! If a constraint requires randomized behavior, randomness must be explicitly
//! supplied by the higher-level scheduling policy. Constraint checking itself
//! should remain deterministic.
//!
//! # Error model
//!
//! Constraint violations must be represented structurally by the child
//! constraint modules and ultimately surfaced through the scheduling error
//! boundary.
//!
//! Error strings must never be used as machine-readable control flow.
//!
//! # Thread safety
//!
//! Constraint definitions should be safe to share between scheduler workers
//! when their implementations permit it.
//!
//! The module itself owns no mutable global state.
//!
//! Implementations must prefer immutable inputs and local evaluation state.
//!
//! # No unsafe
//!
//! This module enforces the no-unsafe requirement at the module boundary.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is permitted in this module or its child implementations.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no architecture-specific assumptions;
//! - no unsafe code.
//!
//! # Serialization
//!
//! Serialization of concrete constraints belongs to the child modules and,
//! where applicable, the scheduling serialization subsystem.
//!
//! This module must not introduce a competing serialization format.
//!
//! # Public API stability
//!
//! This file is deliberately a composition root.
//!
//! Child modules are public because scheduling integrations may need to
//! construct and inspect individual constraint families.
//!
//! Adding functionality to an existing child module should normally require
//! changing only that child module.
//!
//! Adding a new constraint family should require:
//!
//! 1. creating the new child file;
//! 2. implementing its documented contract;
//! 3. adding one `pub mod` declaration here;
//! 4. adding an explicit re-export only if it is part of the stable public API.
//!
//! Existing child modules should not need to be rewritten merely because a new
//! constraint family is introduced.
//!
//! # No glob exports
//!
//! Glob exports are deliberately avoided.
//!
//! Do not use:
//!
//! ```text
//! pub use constraint::*;
//! ```
//!
//! Explicit exports prevent accidental public API collisions as the subsystem
//! grows.
//!
//! # Constraint evaluation model
//!
//! Conceptually, all constraints participate in:
//!
//! ```text
//! candidate schedule
//!        │
//!        ▼
//! ┌──────────────────────┐
//! │ constraint evaluation│
//! └──────────┬───────────┘
//!            │
//!       ┌────┴────┐
//!       ▼         ▼
//!     valid     violation
//!       │         │
//!       ▼         ▼
//! continue     structured
//! scheduling   diagnostic
//! ```
//!
//! Constraint implementations should support early rejection when possible,
//! while preserving deterministic diagnostics.
//!
//! # Incremental evaluation
//!
//! Production schedulers may evaluate constraints incrementally as operations
//! are inserted into a partial schedule.
//!
//! Therefore constraints should be designed so that a caller can evaluate:
//!
//! ```text
//! complete candidate
//! ```
//!
//! or:
//!
//! ```text
//! partial candidate + proposed operation
//! ```
//!
//! without requiring reconstruction of the entire machine state.
//!
//! # Large-scale scheduling
//!
//! Implementations must avoid algorithms that require materializing a full
//! qubit-by-time matrix or channel-by-time matrix.
//!
//! Prefer representations supplied by the scheduling resource/calendar layer,
//! such as:
//!
//! ```text
//! interval collections
//! resource calendars
//! sparse reservations
//! dependency edges
//! indexed availability
//! ```
//!
//! The constraints layer itself must not impose a particular storage strategy
//! on the scheduler.
//!
//! # Canonical ownership table
//!
//! ```text
//! Concept                         Owner
//! ---------------------------------------------------------------------------
//! Logical qubit identity          quantum::ir::qubit::QubitId
//! Physical qubit identity         quantum::ir::qubit::PhysicalQubitId
//! Quantum semantics               quantum::ir
//! Logical/physical mapping        quantum::routing
//! Scheduling constraints          quantum::scheduling::constraints
//! Scheduling policy               quantum::scheduling::policies
//! Scheduling algorithm            quantum::scheduling::algorithms/planners
//! Hardware capabilities           quantum::hardware
//! Calibration                     quantum::hardware
//! Noise model                     quantum::zqn
//! QEC semantics                   quantum::qec / scheduling::qec boundary
//! Execution                       runtime / hardware
//! ```
//!
//! # Integration contract
//!
//! The intended dependency flow is:
//!
//! ```text
//! quantum::ir
//!       │
//!       ▼
//! routing
//!       │
//!       ▼
//! scheduling context
//!       │
//!       ├───────────────┐
//!       ▼               ▼
//! timing model       resource model
//!       │               │
//!       └───────┬───────┘
//!               ▼
//!       scheduling constraints
//!               │
//!               ▼
//!          scheduler/planner
//!               │
//!               ▼
//!          verification
//! ```
//!
//! The constraint layer must remain usable independently of the concrete
//! scheduler algorithm.
//!
//! # Future extensibility
//!
//! The architecture intentionally leaves room for:
//!
//! - photonic resource constraints;
//! - neutral-atom constraints;
//! - trapped-ion constraints;
//! - superconducting control constraints;
//! - spin-qubit constraints;
//! - annealing constraints;
//! - analog constraints;
//! - measurement-based constraints;
//! - distributed quantum-network constraints;
//! - fault-tolerant logical constraints;
//! - application-defined constraints.
//!
//! Such support must be added through explicit constraint implementations or
//! adapters, not by hard-coding technology assumptions into this module.

// -----------------------------------------------------------------------------
// Compiler-enforced safety boundary
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Constraint-family modules
// -----------------------------------------------------------------------------
//
// IMPORTANT:
//
// These declarations are intentionally established now as the permanent
// integration boundary. Each implementation can be completed independently.
// Once these files exist, changing their internals must not require editing
// this module.
//
// Required directory:
//
// src/quantum/scheduling/constraints/
//
// Required files:
//
//     mod.rs
//     constraint.rs
//     qubit.rs
//     channel.rs
//     measurement.rs
//     reset.rs
//     control.rs
//     communication.rs
//     custom.rs
//
// Additional constraint families can be introduced later without changing
// existing families.

/// Generic scheduling-constraint contract.
///
/// This module owns the foundational constraint abstraction used by all
/// specialized constraint families.
pub mod constraint;

/// Logical and physical qubit occupancy constraints.
///
/// This module MUST use the canonical:
///
/// `crate::quantum::ir::qubit::QubitId`
/// `crate::quantum::ir::qubit::PhysicalQubitId`
///
/// It must never define scheduler-local qubit identity types.
pub mod qubit;

/// Control, drive, readout, shared-electronics and other channel constraints.
pub mod channel;

/// Measurement ordering, readiness, grouping and resource constraints.
pub mod measurement;

/// Reset ordering, readiness and qubit-reuse constraints.
pub mod reset;

/// Classical-control, conditional-operation and feedback constraints.
pub mod control;

/// Inter-device, inter-module and quantum-network communication constraints.
pub mod communication;

/// Application-, target-, research-, and plugin-defined constraints.
pub mod custom;

// -----------------------------------------------------------------------------
// Stable explicit re-exports
// -----------------------------------------------------------------------------
//
// Re-exports are intentionally narrow.
//
// The canonical implementation remains owned by each child module.
// `mod.rs` only exposes stable high-value API symbols.
//
// IMPORTANT:
//
// The exact symbols exported below form the integration contract. Child
// implementations should provide these symbols rather than requiring this
// composition root to be rewritten.
//
// The generic constraint contract is the primary stable API.

pub use constraint::{
    Constraint,
    ConstraintContext,
    ConstraintEvaluation,
    ConstraintId,
    ConstraintKind,
    ConstraintResult,
    ConstraintSeverity,
    ConstraintViolation,
};

// -----------------------------------------------------------------------------
// Constraint-family stable exports
// -----------------------------------------------------------------------------
//
// These exports intentionally use the family-level public names rather than
// glob imports. This keeps API ownership explicit and prevents accidental
// collisions as the scheduler grows.

pub use qubit::{
    QubitConstraint,
    QubitConstraintKind,
};

pub use channel::{
    ChannelConstraint,
    ChannelConstraintKind,
};

pub use measurement::{
    MeasurementConstraint,
    MeasurementConstraintKind,
};

pub use reset::{
    ResetConstraint,
    ResetConstraintKind,
};

pub use control::{
    ControlConstraint,
    ControlConstraintKind,
};

pub use communication::{
    CommunicationConstraint,
    CommunicationConstraintKind,
};

pub use custom::{
    CustomConstraint,
    CustomConstraintKind,
};

// -----------------------------------------------------------------------------
// Public constraint prelude
// -----------------------------------------------------------------------------
//
// This prelude is intentionally small.
//
// It exists for scheduler/planner implementations that need the common
// constraint vocabulary without importing every specialized family.
//
// Specialized APIs should continue to be imported from their owning module.

/// Common imports for scheduling-constraint consumers.
///
/// Example:
///
/// ```ignore
/// use crate::quantum::scheduling::constraints::prelude::{
///     Constraint,
///     ConstraintContext,
///     ConstraintEvaluation,
///     ConstraintId,
///     ConstraintKind,
///     ConstraintResult,
///     ConstraintSeverity,
///     ConstraintViolation,
/// };
/// ```
pub mod prelude {
    pub use super::{
        Constraint,
        ConstraintContext,
        ConstraintEvaluation,
        ConstraintId,
        ConstraintKind,
        ConstraintResult,
        ConstraintSeverity,
        ConstraintViolation,
    };
}

// -----------------------------------------------------------------------------
// Integration invariants
// -----------------------------------------------------------------------------
//
// These invariants are documentation-level contracts deliberately centralized
// here so every child implementation follows the same rules.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 1 — Canonical qubit identity
//
// Every qubit-aware child module must use:
//
//     crate::quantum::ir::qubit::QubitId
//     crate::quantum::ir::qubit::PhysicalQubitId
//
// It must never define:
//
//     struct QubitId(...)
//     struct PhysicalQubitId(...)
//
// locally.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 2 — No implicit logical/physical conversion
//
// A logical QubitId must never be interpreted as a PhysicalQubitId merely
// because both currently use index-compatible representations.
//
// Explicit mapping belongs to routing.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 3 — No machine-size constants
//
// Forbidden examples:
//
//     MAX_QUBITS
//     MAX_CHANNELS
//     MAX_RESOURCES
//     MAX_OPERATIONS
//     MAX_DEPTH
//     SURFACE_CODE_SIZE
//
// Constraint capacity comes from the supplied scheduling context and target
// resource model.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 4 — No vendor coupling
//
// This module must not import vendor SDKs.
//
// Vendor-specific information must arrive through a hardware adapter and
// target/resource capabilities.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 5 — No scheduling policy
//
// Constraints must not choose:
//
//     ASAP
//     ALAP
//     critical-path priority
//     fidelity priority
//     resource priority
//
// Those belong to scheduling policies/objectives.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 6 — No routing
//
// Constraints may require a physical resource, but they must not choose the
// logical-to-physical mapping.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 7 — No execution
//
// Constraint evaluation must not submit, cancel, pause, resume or otherwise
// execute a hardware job.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 8 — No hidden mutable global state
//
// Constraints must not rely on global mutable registries, global hardware
// state, global configuration or process-wide scheduling state.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 9 — Deterministic diagnostics
//
// Given equivalent input context, candidate schedule and constraint
// configuration, diagnostics must be deterministic.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 10 — Checked arithmetic
//
// Constraint implementations must use checked/saturating arithmetic where
// overflow could affect legality or resource accounting.
//
// A scheduling constraint must never silently wrap an integer and thereby
// convert an invalid schedule into an apparently valid one.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 11 — Partial schedules
//
// Constraint implementations should support evaluation against partial
// schedules where practical.
//
// This allows scalable incremental/list/event scheduling.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 12 — Dynamic execution
//
// Constraints must not assume every execution property is statically known.
//
// Runtime measurements, feedback, communication and changing availability may
// introduce dynamic constraints.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 13 — Sparse scalability
//
// Constraints must not require a dense:
//
//     qubit × time
//
// or:
//
//     resource × time
//
// representation.
//
// Large schedules should be representable using sparse reservations and
// intervals.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 14 — Semantic preservation
//
// A scheduling constraint may reject an illegal schedule, but it must not
// modify quantum program semantics.
//
// Transformations such as delay insertion or padding belong to the scheduling
// transformation subsystem.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 15 — Constraint composition
//
// Multiple constraints must be composable:
//
//     qubit
//       + channel
//       + measurement
//       + reset
//       + control
//       + communication
//       + custom
//
// without requiring any specialized constraint to know about all other
// constraint families.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 16 — Extensibility
//
// New hardware technologies and scheduling requirements must be addable as
// new constraint implementations rather than requiring changes to the
// canonical IR.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 17 — Error ownership
//
// Constraint-specific errors belong to their child module.
//
// Cross-subsystem scheduling errors belong to the scheduler error boundary.
//
// This prevents `constraints/mod.rs` from becoming a second global error
// hierarchy.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 18 — Serialization ownership
//
// Constraint serialization must use the scheduling serialization contract
// rather than introducing an unrelated format in this composition root.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 19 — Thread-safe architecture
//
// The module contains no global mutable scheduler state.
//
// Child implementations should use immutable shared configuration where
// possible.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 20 — No unsafe
//
// This entire subtree is subject to:
//
//     #![forbid(unsafe_code)]
//
// No implementation may bypass this requirement.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 21 — Target independence
//
// A constraint must describe a requirement, not a particular machine.
//
// Good:
//
//     "resource capacity is exhausted"
//
// Bad:
//
//     "this machine has exactly eight channels"
//
// -----------------------------------------------------------------------------
//
// INVARIANT 22 — Resource ownership
//
// Constraints consume resource descriptions.
//
// They do not own the canonical hardware resource registry.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 23 — Timing ownership
//
// Constraints may consume timing information:
//
//     start
//     finish
//     duration
//     alignment
//     windows
//
// but timing representation itself belongs to the scheduling timing subsystem.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 24 — QEC separation
//
// QEC-specific constraints may be supplied through the QEC scheduling
// integration boundary.
//
// Generic constraints must remain usable without a surface-code implementation.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 25 — Distributed scalability
//
// Communication constraints must be capable of representing resources across
// multiple devices/modules/nodes without assuming a single-QPU architecture.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 26 — No artificial "infinity"
//
// Do not represent scalability by choosing an enormous finite sentinel such
// as:
//
//     usize::MAX
//
// for an architectural maximum.
//
// "Unlimited" must be represented semantically by the appropriate optional or
// policy-level representation, not by pretending the largest integer is a
// meaningful hardware capacity.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 27 — Resource availability is external
//
// Availability can change because of:
//
//     calibration
//     maintenance
//     degradation
//     reservation
//     runtime state
//
// Constraints must consume the relevant snapshot/context rather than directly
// querying hardware.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 28 — No assumptions about gate arity
//
// Constraints must not assume quantum operations are only one- or two-qubit
// operations.
//
// Arbitrary operation arity must remain representable.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 29 — No assumptions about execution model
//
// Constraints must remain applicable to:
//
//     circuit
//     dynamic circuit
//     pulse-aware execution
//     measurement-based computation
//     analog execution
//     fault-tolerant logical execution
//     distributed quantum execution
//
// where the surrounding IR and target adapters provide the necessary
// information.
//
// -----------------------------------------------------------------------------
//
// INVARIANT 30 — Independent completion
//
// Once the child modules satisfy their individual contracts, additions to:
//
//     policies
//     planners
//     algorithms
//     routing
//     hardware
//     QEC
//     runtime
//
// should not require re-editing this file merely because those modules gained
// functionality.
//
// -----------------------------------------------------------------------------
// End of composition-root contract.
// -----------------------------------------------------------------------------