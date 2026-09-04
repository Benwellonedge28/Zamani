//! Zamani Quantum Scheduling — Canonical Time Boundary
//!
//! Path:
//!     src/quantum/scheduling/timing/time.rs
//!
//! # Purpose
//!
//! This module exposes the canonical semantic time types to the scheduling
//! subsystem without defining a second timing representation.
//!
//! The authoritative semantic timing implementation belongs to:
//!
//!     crate::quantum::ir::timing
//!
//! This module deliberately acts as the scheduling boundary for:
//!
//! - `TimePoint`;
//! - `TimeOffset`;
//! - `Duration`.
//!
//! # Architectural rule
//!
//! There MUST be exactly one semantic definition of time in Zamani Quantum IR.
//!
//! ```text
//!                     quantum::ir::timing
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!        TimePoint        TimeOffset         Duration
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              │
//!                 ┌────────────┴────────────┐
//!                 │                         │
//!                 ▼                         ▼
//!             scheduling                 hardware
//!                 │                         │
//!                 ▼                         ▼
//!            "WHEN?"                 physical realization
//! ```
//!
//! The scheduler must consume semantic timing. It must not redefine it.
//!
//! # Why this file does not implement `TimePoint`
//!
//! A second implementation would create incompatible timing domains such as:
//!
//! ```text
//! quantum::ir::timing::TimePoint
//! quantum::scheduling::timing::TimePoint
//! ```
//!
//! That would require conversion glue throughout:
//!
//! - constraints;
//! - reservations;
//! - planners;
//! - verification;
//! - optimization;
//! - QEC;
//! - distributed scheduling;
//! - hardware adapters;
//! - serialization;
//! - diagnostics.
//!
//! Such duplication is unnecessary and is dangerous for a production compiler.
//!
//! The IR timing model is already the canonical semantic owner, so scheduling
//! reuses it directly.
//!
//! # TimePoint semantics
//!
//! `TimePoint` represents an absolute semantic position within a timing
//! domain.
//!
//! It does NOT represent:
//!
//! - UNIX time;
//! - wall-clock time;
//! - `std::time::Instant`;
//! - CPU time;
//! - an operating-system timer;
//! - a hardware clock register;
//! - a pulse-generator timestamp;
//! - a backend `dt` tick.
//!
//! The canonical IR timing implementation deliberately distinguishes semantic
//! time from physical clock realization. The scheduler must preserve that
//! distinction.
//!
//! # TimeOffset semantics
//!
//! `TimeOffset` represents a signed displacement between semantic time points.
//!
//! This is important because absolute `TimePoint` values are non-negative,
//! while relationships such as:
//!
//! ```text
//! A - B
//! ```
//!
//! may be negative.
//!
//! The canonical timing subsystem represents signed offsets without reducing
//! the complete `u128` time domain to the smaller signed `i128` domain.
//!
//! Scheduling must use `TimeOffset` for signed temporal relationships rather
//! than encoding negative values into an unsigned `TimePoint`.
//!
//! # Duration semantics
//!
//! `Duration` represents elapsed semantic time.
//!
//! The canonical implementation uses exact attoseconds internally:
//!
//! ```text
//! 1 second = 10^18 attoseconds
//! ```
//!
//! and uses `u128` so that the semantic representation does not introduce a
//! small machine-specific timing ceiling.
//!
//! This does NOT mean that hardware has attosecond physical resolution.
//!
//! Hardware timing resolution is resolved later by the hardware/scheduling
//! boundary.
//!
//! # Hardware timing
//!
//! A target may expose a timing representation such as:
//!
//! ```text
//! dt = target-specific interval
//! ```
//!
//! The scheduler must never replace canonical semantic timing with that
//! backend-specific representation.
//!
//! Instead:
//!
//! ```text
//! semantic TimePoint / Duration
//!             │
//!             ▼
//! scheduling constraints
//!             │
//!             ▼
//! hardware adapter
//!             │
//!             ▼
//! target timing representation
//! ```
//!
//! Backend conversion must be explicit and checked.
//!
//! # Universal-program principle
//!
//! The scheduling time model must not contain:
//!
//! - maximum qubit counts;
//! - maximum operation counts;
//! - maximum schedule depth;
//! - fixed machine sizes;
//! - fixed clock frequencies;
//! - fixed channel counts;
//! - vendor-specific timing constants;
//! - fixed topology dimensions.
//!
//! Therefore there is deliberately no code here such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TIME
//! MAX_OPERATIONS
//! MAX_SCHEDULE_DEPTH
//! DEFAULT_DT
//! ```
//!
//! A Zamani program describes computation semantically.
//!
//! The target determines how that computation is physically realized.
//!
//! # Scalability
//!
//! The type boundary works for:
//!
//! ```text
//! one operation
//!     │
//! small QPU
//!     │
//! large QPU
//!     │
//! fault-tolerant processor
//!     │
//! multi-chip system
//!     │
//! distributed QPU system
//!     │
//! quantum network
//!     │
//! future quantum architecture
//! ```
//!
//! The amount of available hardware is not encoded into these types.
//!
//! `u128` is only the exact representation width of a semantic time
//! coordinate. It is not a limit on:
//!
//! - number of qubits;
//! - number of operations;
//! - number of devices;
//! - number of resources;
//! - number of scheduling events.
//!
//! # No qubit dependency
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A time point does not belong to a qubit.
//!
//! A scheduling operation that has qubit operands should independently use the
//! canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The dependency direction is therefore:
//!
//! ```text
//! QubitId ───────────────┐
//! PhysicalQubitId ───────┤
//!                         ▼
//!                    Operation
//!                         │
//!                         ▼
//!                     TimePoint
//!                         │
//!                         ▼
//!                     Schedule
//! ```
//!
//! and NOT:
//!
//! ```text
//! TimePoint
//!     │
//!     └──> QubitId
//! ```
//!
//! This prevents timing from becoming coupled to a particular quantum
//! resource model.
//!
//! # Integration with scheduler types
//!
//! `src/quantum/scheduling/types.rs` currently contains scheduler-owned
//! definitions named `TimePoint` and `Duration`.
//!
//! Those definitions must not remain as independent semantic types.
//!
//! The integration target is:
//!
//! ```text
//! scheduling::types
//!       │
//!       └── re-export canonical timing values
//!
//! scheduling::timing::time
//!       │
//!       └── re-export canonical timing values
//!
//!                 ▼
//!       quantum::ir::timing
//! ```
//!
//! There must be one source of truth.
//!
//! In particular, scheduling modules must eventually be able to use either:
//!
//! ```text
//! crate::quantum::ir::timing::TimePoint
//! ```
//!
//! or the scheduler's public timing façade:
//!
//! ```text
//! crate::quantum::scheduling::timing::time::TimePoint
//! ```
//!
//! and receive the same Rust type.
//!
//! # Integration with scheduling resources
//!
//! Resource reservations should store the canonical timing types:
//!
//! ```text
//! Reservation
//!     ├── operation identity
//!     ├── resource identity
//!     ├── start: TimePoint
//!     ├── duration: Duration
//!     └── end: TimePoint
//! ```
//!
//! Resource calendars must therefore not create another time representation.
//!
//! # Integration with scheduling constraints
//!
//! Temporal constraints consume:
//!
//! - `TimePoint`;
//! - `TimeOffset`;
//! - `Duration`.
//!
//! Examples include:
//!
//! ```text
//! release time
//! deadline
//! earliest start
//! latest start
//! earliest finish
//! latest finish
//! minimum separation
//! maximum separation
//! synchronization
//! alignment
//! ```
//!
//! Constraint code must use checked canonical arithmetic.
//!
//! # Integration with scheduling planners
//!
//! Planners use this boundary for:
//!
//! - earliest-start calculations;
//! - latest-start calculations;
//! - resource availability;
//! - operation finish times;
//! - critical-path calculations;
//! - slack;
//! - makespan;
//! - event-driven scheduling.
//!
//! No planner should introduce its own `TimePoint` or `Duration`.
//!
//! # Integration with verification
//!
//! Verification must operate on the same canonical timing values used to
//! construct the schedule.
//!
//! This allows invariants such as:
//!
//! ```text
//! finish = start + duration
//! ```
//!
//! and:
//!
//! ```text
//! predecessor.finish <= successor.start
//! ```
//!
//! to be checked without conversion between scheduler-specific timing types.
//!
//! # Integration with QEC
//!
//! QEC scheduling constraints may use the same semantic timing primitives for:
//!
//! - syndrome extraction rounds;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurement;
//! - classical processing;
//! - feedback;
//! - round separation.
//!
//! QEC-specific concepts must remain in the QEC subsystem.
//!
//! This file supplies only the common temporal representation.
//!
//! # Integration with distributed scheduling
//!
//! Distributed scheduling may use the same timing types for:
//!
//! - local execution;
//! - synchronization;
//! - communication latency;
//! - entanglement-generation windows;
//! - teleportation operations;
//! - classical communication;
//! - inter-node dependencies.
//!
//! A distributed node may establish its own timing domain, but such domain
//! semantics belong to the enclosing scheduling/IR structure, not to
//! `TimePoint` itself.
//!
//! # Integration with hardware
//!
//! Hardware adapters consume canonical semantic timing and translate it into
//! target-specific timing.
//!
//! Examples include:
//!
//! ```text
//! TimePoint
//!     ↓
//! target timing resolution
//!     ↓
//! hardware tick
//! ```
//!
//! or:
//!
//! ```text
//! Duration
//!     ↓
//! target sample period
//!     ↓
//! number of samples
//! ```
//!
//! Conversion must detect:
//!
//! - overflow;
//! - underflow;
//! - unsupported precision;
//! - non-integral target ticks;
//! - invalid timing resolution.
//!
//! No conversion through floating-point values should be required for the
//! canonical semantic representation.
//!
//! # Integration with serialization
//!
//! Scheduling serialization must preserve the exact canonical timing values.
//!
//! It must not serialize timing through floating-point representations such as:
//!
//! ```text
//! f32
//! f64
//! ```
//!
//! because doing so can lose exact temporal information.
//!
//! Canonical serialization should preserve the exact integer semantic value
//! exposed by the underlying IR timing implementation.
//!
//! # Integration with diagnostics
//!
//! Diagnostics may format these types for messages such as:
//!
//! ```text
//! operation delayed until T...
//! resource unavailable until T...
//! deadline exceeded at T...
//! ```
//!
//! Formatting is diagnostic output only. It must never become the semantic
//! identity of a timing value.
//!
//! # Integration with optimization
//!
//! Scheduling optimization may consume:
//!
//! - duration;
//! - start time;
//! - finish time;
//! - idle intervals;
//! - makespan;
//! - critical path;
//! - slack.
//!
//! Optimization must operate on canonical values and must not mutate the
//! meaning of `TimePoint` or `Duration`.
//!
//! # Integration with ZQN
//!
//! ZQN may associate uncertainty/noise information with temporal regions.
//!
//! The relationship should be:
//!
//! ```text
//! canonical timing
//!       │
//!       ▼
//! ZQN temporal/noise model
//!       │
//!       ▼
//! scheduling objective
//! ```
//!
//! ZQN must not require a second timing representation.
//!
//! # Thread safety
//!
//! The canonical timing values are plain value types and contain no global
//! mutable state.
//!
//! They can therefore be passed between scheduling analyses and worker
//! contexts according to the normal Rust ownership and `Send`/`Sync` rules.
//!
//! This module itself creates no shared mutable state.
//!
//! # Determinism
//!
//! For identical canonical timing values:
//!
//! ```text
//! equality
//! ordering
//! hashing
//! serialization
//! ```
//!
//! must remain deterministic across supported platforms.
//!
//! Scheduling algorithms are responsible for deterministic arbitration when
//! multiple operations are simultaneously eligible.
//!
//! # Error ownership
//!
//! This module deliberately does not create a second scheduling error type.
//!
//! Timing-specific validation errors belong to the canonical IR timing layer.
//!
//! Scheduler-level failures such as:
//!
//! - resource conflicts;
//! - unschedulable dependency graphs;
//! - deadline violations;
//! - planner failures;
//! - verification failures;
//!
//! belong to `quantum::scheduling::errors`.
//!
//! # Rust compatibility
//!
//! This module is compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! It uses no nightly features and no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Design invariant
//!
//! The most important invariant of this file is:
//!
//! ```text
//! scheduling::timing::time::TimePoint
//!                 ==
//! quantum::ir::timing::TimePoint
//! ```
//!
//! and similarly:
//!
//! ```text
//! scheduling::timing::time::TimeOffset
//!                 ==
//! quantum::ir::timing::TimeOffset
//!
//! scheduling::timing::time::Duration
//!                 ==
//! quantum::ir::timing::Duration
//! ```
//!
//! These are not merely equivalent concepts. They should be the same Rust
//! types.
//!
//! This eliminates conversion boundaries and keeps the entire compiler on one
//! canonical timing model.
//!
//! # Why this file can be considered complete independently
//!
//! This file owns one responsibility:
//!
//! > Expose canonical IR timing semantics at the scheduling boundary.
//!
//! Adding later:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - resource-constrained scheduling;
//! - QEC;
//! - distributed scheduling;
//! - hardware adapters;
//! - dynamic circuits;
//! - optimization;
//! - diagnostics;
//! - serialization;
//!
//! does not require changing this file.
//!
//! Those modules consume this stable boundary.
//!
//! # Public API
//!
//! The complete public API intentionally consists of canonical re-exports.
//!
//! Do not add scheduler-specific timing implementations here.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical semantic elapsed-time representation.
///
/// This is re-exported from the Quantum IR timing subsystem so that scheduling
/// cannot accidentally create a competing `Duration` type.
pub use crate::quantum::ir::timing::Duration;

/// Canonical signed semantic temporal displacement.
///
/// `TimeOffset` is required whenever a scheduling relationship may be
/// negative, such as the difference between two time points.
pub use crate::quantum::ir::timing::TimeOffset;

/// Canonical absolute semantic time coordinate.
///
/// This represents a position in an enclosing semantic timing domain rather
/// than wall-clock or hardware time.
pub use crate::quantum::ir::timing::TimePoint;