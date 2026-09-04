//! Zamani Quantum Scheduling — Timing Subsystem
//!
//! Path:
//!     src/quantum/scheduling/timing/mod.rs
//!
//! # Purpose
//!
//! This module is the public composition boundary for the scheduling timing
//! subsystem.
//!
//! The scheduling timing subsystem answers:
//!
//!     "When may an operation execute, and under which temporal constraints?"
//!
//! It does NOT define a second semantic timing model.
//!
//! The canonical semantic timing model is owned by:
//!
//!     crate::quantum::ir::timing
//!
//! The scheduling layer consumes those canonical types through the stable
//! scheduling-facing modules:
//!
//!     crate::quantum::scheduling::timing::duration
//!     crate::quantum::scheduling::timing::time
//!
//! # Architectural boundary
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                       quantum::frontend
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                 +------------+-------------+
//!                 |                          |
//!                 v                          v
//!          quantum::ir::timing       quantum::ir::operation
//!                 |                          |
//!                 |                          |
//!                 +------------+-------------+
//!                              |
//!                              v
//!                    quantum::scheduling
//!                              |
//!                              v
//!                scheduling::timing::mod
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!      duration             time            timing policies
//!          |                   |                   |
//!          +-------------------+-------------------+
//!                              |
//!                              v
//!                  dependency/resource scheduler
//!                              |
//!                              v
//!                     hardware adapter
//!                              |
//!                              v
//!                         hardware QPU
//! ```
//!
//! # Ownership
//!
//! The canonical IR timing subsystem owns semantic timing:
//!
//! - `Duration`;
//! - `TimePoint`;
//! - `TimeOffset`;
//! - semantic timing arithmetic;
//! - timing expressions;
//! - semantic timing constraints;
//! - semantic intervals;
//! - timing literals;
//! - semantic timing serialization.
//!
//! The scheduling timing subsystem owns the scheduling-facing organization of
//! those concepts.
//!
//! It provides:
//!
//! - the stable scheduling timing namespace;
//! - timing-module composition;
//! - timing-facing public exports;
//! - timing subsystem documentation;
//! - explicit boundaries between semantic time and physical realization.
//!
//! It does NOT own:
//!
//! - hardware clocks;
//! - hardware sample rates;
//! - DAC/ADC configuration;
//! - pulse generation;
//! - routing;
//! - optimization algorithms;
//! - QEC algorithms;
//! - backend execution;
//! - provider SDKs;
//! - operating-system clocks;
//! - wall-clock time;
//! - scheduler global state.
//!
//! # Single source of truth
//!
//! There must be exactly one semantic definition for each of:
//!
//! ```text
//! Duration
//! TimePoint
//! TimeOffset
//! ```
//!
//! That source of truth is:
//!
//!     crate::quantum::ir::timing
//!
//! Therefore this module MUST NOT define replacements such as:
//!
//! ```text
//! pub struct Duration { ... }
//! pub struct TimePoint { ... }
//! pub struct TimeOffset { ... }
//! ```
//!
//! Doing so would create incompatible timing domains and force unnecessary
//! conversions throughout:
//!
//! - scheduling;
//! - routing integration;
//! - resource calendars;
//! - constraints;
//! - planners;
//! - verification;
//! - QEC;
//! - distributed scheduling;
//! - hardware adapters;
//! - serialization;
//! - diagnostics;
//! - benchmarking.
//!
//! # Stable scheduling façade
//!
//! The intended public paths are:
//!
//! ```text
//! crate::quantum::scheduling::timing::duration::Duration
//! crate::quantum::scheduling::timing::time::TimePoint
//! crate::quantum::scheduling::timing::time::TimeOffset
//! ```
//!
//! These resolve to the canonical IR timing types.
//!
//! Consequently:
//!
//! ```text
//! scheduling::timing::duration::Duration
//!             ==
//! quantum::ir::timing::Duration
//! ```
//!
//! and:
//!
//! ```text
//! scheduling::timing::time::TimePoint
//!             ==
//! quantum::ir::timing::TimePoint
//! ```
//!
//! and:
//!
//! ```text
//! scheduling::timing::time::TimeOffset
//!             ==
//! quantum::ir::timing::TimeOffset
//! ```
//!
//! These are the same Rust types, not merely structurally equivalent types.
//!
//! # Timing layers
//!
//! Zamani intentionally separates three concepts:
//!
//! ```text
//! Semantic timing
//! ----------------
//! What temporal quantity does an operation represent?
//!
//! quantum::ir::timing
//!
//! Scheduling timing
//! -----------------
//! When should the operation execute relative to other operations?
//!
//! quantum::scheduling::timing
//!
//! Hardware timing
//! ----------------
//! How does a particular target physically realize that timing?
//!
//! quantum::hardware::timing
//! ```
//!
//! They must not be collapsed.
//!
//! # Universal-program principle
//!
//! Zamani programs must describe computation rather than a fixed machine.
//!
//! The scheduling timing subsystem therefore MUST NOT contain assumptions such
//! as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_CHANNELS
//! MAX_DEPTH
//! MAX_SCHEDULE_TIME
//! DEFAULT_DT
//! FIXED_CLOCK
//! FIXED_SAMPLE_RATE
//! FIXED_TOPOLOGY
//! ```
//!
//! There is no machine-size parameter in this module.
//!
//! The same timing façade is valid for:
//!
//! ```text
//! one qubit
//!     |
//! small QPU
//!     |
//! large QPU
//!     |
//! fault-tolerant processor
//!     |
//! multi-chip system
//!     |
//! distributed QPU system
//!     |
//! quantum network
//!     |
//! future quantum architecture
//! ```
//!
//! Scaling is determined by the supplied target, resource model, timing model,
//! and available memory/execution resources.
//!
//! # "Infinity" scalability
//!
//! No finite software implementation can literally represent an infinite
//! schedule.
//!
//! Zamani therefore uses the correct engineering interpretation of
//! "scale to infinity":
//!
//! > The scheduler introduces no artificial machine-size ceiling. It scales
//! > until the supplied computational, memory, target, or execution resources
//! > become the limiting factor.
//!
//! This module introduces no additional artificial ceiling.
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
//! Timing is not inherently a qubit concept.
//!
//! Timing can describe:
//!
//! - quantum gates;
//! - measurements;
//! - resets;
//! - pulse intervals;
//! - analog evolution;
//! - classical processing;
//! - synchronization;
//! - feedback latency;
//! - communication latency;
//! - QEC activity;
//! - distributed operations;
//! - resource reservations.
//!
//! Where an operation or resource record contains qubit operands, that owning
//! module MUST use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Timing must remain independent from qubit identity.
//!
//! # Existing timing modules
//!
//! The scheduling timing subsystem currently exposes the following primitive
//! façade modules:
//!
//! ```text
//! timing/
//! ├── mod.rs
//! ├── duration.rs
//! └── time.rs
//! ```
//!
//! `duration.rs` provides the scheduling-facing `Duration` boundary.
//!
//! `time.rs` provides the scheduling-facing:
//!
//! - `TimePoint`;
//! - `TimeOffset`;
//! - `Duration`.
//!
//! Both delegate semantic ownership to `quantum::ir::timing`.
//!
//! Future timing components may be added behind this composition boundary,
//! provided they obey the same ownership rules.
//!
//! # Future extension boundary
//!
//! The following concepts may eventually be represented by additional timing
//! modules if the repository's canonical IR timing model is split further:
//!
//! ```text
//! resolution.rs
//! alignment.rs
//! windows.rs
//! constraints.rs
//! ```
//!
//! However, these modules must be introduced only when their contracts are
//! independently defined and implemented.
//!
//! This `mod.rs` must not fabricate APIs for modules that do not exist.
//!
//! This is important for build correctness: a Rust module declaration creates a
//! compile-time dependency on the corresponding source file.
//!
//! Therefore this file declares only timing modules that are actually part of
//! the current scheduling timing implementation.
//!
//! # Semantic duration
//!
//! `Duration` represents elapsed semantic time.
//!
//! It is not:
//!
//! - `std::time::Duration`;
//! - wall-clock time;
//! - an OS timer;
//! - a backend tick count;
//! - a hardware register value;
//! - a pulse sample count.
//!
//! Hardware-specific timing conversion belongs to the hardware/scheduling
//! adapter boundary.
//!
//! The existing scheduling duration façade already follows this architecture by
//! directly re-exporting the canonical IR duration instead of introducing a
//! duplicate type.
//!
//! # Time point
//!
//! `TimePoint` represents an absolute position in a semantic timing domain.
//!
//! It does not represent:
//!
//! - UNIX time;
//! - system time;
//! - `Instant`;
//! - CPU time;
//! - provider timestamps;
//! - hardware clock-register values.
//!
//! Hardware realization occurs downstream.
//!
//! # Time offset
//!
//! `TimeOffset` represents a signed temporal displacement.
//!
//! It is required for relationships such as:
//!
//! ```text
//! A - B
//! ```
//!
//! where the result may be positive or negative.
//!
//! The canonical IR timing implementation owns the representation and checked
//! arithmetic semantics.
//!
//! # Scheduling integration
//!
//! Scheduling algorithms consume the timing façade through this module.
//!
//! Typical flow:
//!
//! ```text
//! operation
//!     |
//!     +---- dependency information
//!     |
//!     +---- resource requirements
//!     |
//!     +---- Duration
//!     |
//!     +---- timing constraints
//!     |
//!     v
//! scheduler
//!     |
//!     +---- TimePoint
//!     +---- TimeOffset
//!     +---- Duration
//!     |
//!     v
//! schedule result
//! ```
//!
//! Planners such as:
//!
//! - ASAP;
//! - ALAP;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - event-driven scheduling;
//! - adaptive scheduling;
//!
//! must consume these canonical timing types.
//!
//! None may define another timing representation.
//!
//! # Resource integration
//!
//! Resource reservations should use the canonical scheduling timing façade.
//!
//! Conceptually:
//!
//! ```text
//! Reservation
//! ├── operation identity
//! ├── resource identity
//! ├── start: TimePoint
//! ├── duration: Duration
//! └── end: TimePoint
//! ```
//!
//! Resource calendars must not introduce a second clock or timeline type.
//!
//! Resource identity remains owned by the resource subsystem.
//!
//! # Dependency integration
//!
//! Dependency scheduling uses canonical timing values for:
//!
//! - earliest start;
//! - earliest finish;
//! - latest start;
//! - latest finish;
//! - slack;
//! - minimum separation;
//! - maximum separation;
//! - synchronization.
//!
//! The dependency graph itself remains owned by the scheduling IR/dependency
//! subsystem.
//!
//! # Constraint integration
//!
//! Timing constraints may express:
//!
//! - release times;
//! - deadlines;
//! - earliest starts;
//! - latest starts;
//! - earliest finishes;
//! - latest finishes;
//! - minimum separations;
//! - maximum separations;
//! - synchronization points;
//! - alignment requirements;
//! - temporal windows.
//!
//! Constraint semantics must use canonical timing values.
//!
//! Scheduler-level errors belong to:
//!
//!     crate::quantum::scheduling::errors
//!
//! Timing-semantic errors belong to:
//!
//!     crate::quantum::ir::timing
//!
//! This separation prevents the timing façade from becoming a second error
//! hierarchy.
//!
//! # Hardware integration
//!
//! Hardware timing is deliberately downstream:
//!
//! ```text
//! semantic Duration / TimePoint
//!             |
//!             v
//! scheduling timing
//!             |
//!             v
//! hardware timing adapter
//!             |
//!             v
//! target timing resolution
//!             |
//!             v
//! provider representation
//! ```
//!
//! Hardware may expose:
//!
//! - discrete timing ticks;
//! - sample periods;
//! - alignment constraints;
//! - minimum intervals;
//! - maximum intervals;
//! - channel-specific timing;
//! - operation-specific timing;
//! - calibration-dependent timing.
//!
//! Those values belong to `quantum::hardware::timing` and its adapters.
//!
//! This module must never hard-code them.
//!
//! # Routing integration
//!
//! Routing answers:
//!
//!     WHERE?
//!
//! Scheduling answers:
//!
//!     WHEN?
//!
//! The timing façade therefore receives timing requirements after routing or
//! from target-independent scheduling information.
//!
//! ```text
//! quantum::routing
//!       |
//!       | physical placement
//!       v
//! quantum::scheduling
//!       |
//!       | temporal placement
//!       v
//! quantum::hardware
//! ```
//!
//! Routing types do not belong in this module.
//!
//! # QEC integration
//!
//! QEC scheduling may consume canonical timing for:
//!
//! - syndrome extraction;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurement;
//! - decoder latency;
//! - feedback latency;
//! - round spacing;
//! - synchronization.
//!
//! QEC-specific semantics remain in the QEC subsystem.
//!
//! This timing module must not depend on a particular error-correcting code.
//!
//! # Dynamic-circuit integration
//!
//! Dynamic circuits require timing relationships involving runtime events:
//!
//! ```text
//! measurement
//!     |
//!     v
//! classical processing
//!     |
//!     v
//! feedback
//!     |
//!     v
//! conditional quantum operation
//! ```
//!
//! The timing façade provides the temporal value types required by this flow.
//!
//! It does not decide whether an operation can be statically scheduled.
//!
//! Runtime-dependent scheduling remains in the dynamic scheduling/runtime
//! subsystem.
//!
//! # Distributed integration
//!
//! The same timing types may represent:
//!
//! - local operation duration;
//! - synchronization;
//! - communication latency;
//! - entanglement-generation windows;
//! - teleportation latency;
//! - classical communication;
//! - inter-node dependencies.
//!
//! Distributed scheduling owns the interpretation of those relationships.
//!
//! This module remains architecture-neutral.
//!
//! # ZQN integration
//!
//! The repository already contains ZQN scheduling integration.
//!
//! ZQN may associate temporal/noise information with scheduling regions.
//!
//! The conceptual dependency is:
//!
//! ```text
//! canonical timing
//!       |
//!       v
//! ZQN temporal/noise information
//!       |
//!       v
//! scheduling objective
//! ```
//!
//! ZQN must consume canonical timing rather than introduce another timing
//! representation.
//!
//! This module must not depend directly on ZQN.
//!
//! # Benchmarking integration
//!
//! Benchmarking may consume scheduling timing to measure:
//!
//! - compilation time;
//! - scheduling time;
//! - makespan;
//! - circuit depth;
//! - idle time;
//! - resource utilization;
//! - timing overhead;
//! - communication latency;
//! - QEC round duration.
//!
//! Benchmarking remains downstream.
//!
//! This module must not depend on benchmarking.
//!
//! # Serialization integration
//!
//! Schedule serialization must preserve the canonical timing semantics.
//!
//! This module itself does not own serialization.
//!
//! Serialization belongs to the relevant IR/scheduling serialization boundary.
//!
//! Timing values must not be converted to floating-point values merely for
//! serialization.
//!
//! Exact semantic values must remain exact.
//!
//! # Diagnostics integration
//!
//! Diagnostics may format timing values for explanations such as:
//!
//! ```text
//! operation delayed until ...
//! resource becomes available at ...
//! deadline occurs at ...
//! predecessor completes at ...
//! successor begins at ...
//! ```
//!
//! Diagnostic formatting is presentation only.
//!
//! It must never become semantic identity.
//!
//! # Determinism
//!
//! The timing façade introduces:
//!
//! - no randomness;
//! - no global mutable state;
//! - no host-clock access;
//! - no environment-dependent timing;
//! - no provider queries.
//!
//! Therefore identical canonical timing values remain deterministic.
//!
//! Scheduler determinism is enforced by the scheduler/planner layer when
//! multiple operations are simultaneously eligible.
//!
//! # Thread safety
//!
//! This module contains only module composition and re-exports.
//!
//! It introduces no:
//!
//! - mutexes;
//! - atomics;
//! - mutable globals;
//! - thread-local state;
//! - caches;
//! - background threads.
//!
//! Thread safety is inherited from the canonical timing types.
//!
//! # Error ownership
//!
//! This module does not define timing errors.
//!
//! Semantic timing errors remain owned by:
//!
//!     crate::quantum::ir::timing
//!
//! Scheduler-level errors remain owned by:
//!
//!     crate::quantum::scheduling::errors
//!
//! Hardware conversion errors remain owned by the appropriate hardware
//! adaptation layer.
//!
//! This prevents error ownership from becoming fragmented across scheduling
//! timing modules.
//!
//! # Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` is intentionally applied here so that the timing
//! subsystem boundary itself rejects unsafe Rust.
//!
//! Individual timing implementation modules also enforce their own unsafe-code
//! policy.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! quantum::ir::timing
//!          |
//!          v
//! scheduling::timing
//!          |
//!          v
//! scheduling planners/resources/constraints
//!          |
//!          v
//! hardware adapter
//! ```
//!
//! This module must not import:
//!
//! - hardware providers;
//! - backend SDKs;
//! - routing implementations;
//! - scheduling algorithms;
//! - QEC implementations;
//! - runtime;
//! - networking;
//! - simulators;
//! - benchmarking implementations.
//!
//! # No circular dependency
//!
//! The timing façade must never require:
//!
//! ```text
//! scheduling -> hardware -> scheduling
//! ```
//!
//! or:
//!
//! ```text
//! scheduling -> routing -> scheduling
//! ```
//!
//! or:
//!
//! ```text
//! scheduling -> QEC -> scheduling
//! ```
//!
//! Consumers depend on this boundary.
//!
//! This boundary does not depend on consumers.
//!
//! # Public API policy
//!
//! This module intentionally provides a small and stable API.
//!
//! The public surface is:
//!
//! ```text
//! duration
//! time
//! ```
//!
//! plus the canonical timing types re-exported at the timing façade root.
//!
//! Root re-exports are provided for ergonomic use:
//!
//! ```text
//! crate::quantum::scheduling::timing::Duration
//! crate::quantum::scheduling::timing::TimePoint
//! crate::quantum::scheduling::timing::TimeOffset
//! ```
//!
//! They remain aliases to the canonical IR types through the submodules.
//!
//! # Why root re-exports are useful
//!
//! Without root re-exports every scheduler consumer would need to know whether
//! a timing value is exposed through `duration` or `time`.
//!
//! A stable façade permits:
//!
//! ```text
//! use crate::quantum::scheduling::timing::{Duration, TimeOffset, TimePoint};
//! ```
//!
//! while preserving the deeper explicit paths:
//!
//! ```text
//! crate::quantum::scheduling::timing::duration::Duration
//! crate::quantum::scheduling::timing::time::Duration
//! crate::quantum::scheduling::timing::time::TimeOffset
//! crate::quantum::scheduling::timing::time::TimePoint
//! ```
//!
//! All of those names resolve to the canonical IR timing types.
//!
//! # Avoiding duplicate exports
//!
//! `Duration` is exposed by both `duration.rs` and `time.rs` because those files
//! are intentionally stable compatibility façades.
//!
//! This module exports the canonical type exactly once at its own root:
//!
//! ```text
//! pub use duration::Duration;
//! ```
//!
//! `TimePoint` and `TimeOffset` are exported from the `time` façade.
//!
//! This avoids defining another type while retaining ergonomic imports.
//!
//! # Future timing modules
//!
//! If the timing subsystem later gains concrete scheduling-specific modules,
//! they should be introduced without changing the semantic ownership model.
//!
//! Candidates include:
//!
//! ```text
//! resolution
//! alignment
//! windows
//! constraints
//! ```
//!
//! Such modules may contain scheduling-specific structures only when those
//! structures are genuinely distinct from canonical IR timing semantics.
//!
//! For example:
//!
//! ```text
//! TimingResolution
//! ```
//!
//! may describe how a target realizes semantic time.
//!
//! It must not replace:
//!
//! ```text
//! Duration
//! ```
//!
//! Likewise, an alignment policy belongs to scheduling/hardware integration,
//! while the semantic duration remains owned by the IR.
//!
//! # Completion invariant
//!
//! This file is considered complete when all scheduling timing consumers can:
//!
//! 1. import canonical timing values through a stable scheduling namespace;
//! 2. use exactly the same Rust timing types as `quantum::ir::timing`;
//! 3. avoid machine-specific timing assumptions;
//! 4. avoid qubit-specific coupling;
//! 5. avoid unsafe Rust;
//! 6. avoid global mutable state;
//! 7. avoid floating-point semantic timing;
//! 8. remain independent of routing implementations;
//! 9. remain independent of hardware providers;
//! 10. remain independent of scheduling algorithms;
//! 11. remain independent of QEC implementations;
//! 12. remain independently compilable as the timing composition boundary.
//!
//! Adding an ASAP planner, ALAP planner, resource scheduler, QEC scheduler,
//! distributed scheduler, hardware adapter, optimizer, diagnostic subsystem,
//! benchmark subsystem, or runtime must not require reopening this file merely
//! because that subsystem was added.
//!
//! # Migration invariant
//!
//! If a future timing module requires a new public semantic type, first
//! determine whether that type belongs in:
//!
//!     quantum::ir::timing
//!
//! rather than automatically adding it here.
//!
//! The scheduling layer must not become the second owner of semantic time.
//!
//! # No hard-coded machine assumptions
//!
//! This file deliberately contains no values representing:
//!
//! - qubit counts;
//! - operation counts;
//! - channel counts;
//! - machine dimensions;
//! - clock frequencies;
//! - sample periods;
//! - hardware topology;
//! - QEC code distances;
//! - maximum schedule depth;
//! - maximum execution time.
//!
//! Limits belong to explicit limit/capability/configuration layers.
//!
//! # No unsafe
//!
//! Unsafe Rust is forbidden.
//!
//! # Canonical integration summary
//!
//! ```text
//!                  quantum::ir::timing
//!                           |
//!             +-------------+-------------+
//!             |                           |
//!             v                           v
//!       scheduling::timing::       scheduling::timing::
//!          duration                   time
//!             |                           |
//!             +-------------+-------------+
//!                           |
//!                           v
//!                  scheduling subsystem
//!                           |
//!       +-------------------+-------------------+
//!       |                   |                   |
//!       v                   v                   v
//!    resources          constraints          planners
//!       |                   |                   |
//!       +-------------------+-------------------+
//!                           |
//!                           v
//!                     schedule result
//!                           |
//!                           v
//!                    hardware adapter
//! ```
//!
//! # Canonical rule
//!
//! ```text
//! Semantic timing belongs to quantum::ir::timing.
//!
//! Scheduling timing organizes and consumes it.
//!
//! Hardware timing realizes it.
//!
//! No layer creates a competing semantic timing type.
//! ```

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Scheduling-facing canonical duration boundary.
///
/// The actual semantic type is owned by `quantum::ir::timing`.
pub mod duration;

/// Scheduling-facing canonical time boundary.
///
/// This exposes the canonical `TimePoint`, `TimeOffset`, and `Duration`
/// types without defining replacements.
pub mod time;

/// Canonical semantic duration.
///
/// This is exactly the same Rust type as
/// `crate::quantum::ir::timing::Duration`.
pub use duration::Duration;

/// Canonical absolute semantic time coordinate.
///
/// This is exactly the same Rust type as
/// `crate::quantum::ir::timing::TimePoint`.
pub use time::TimePoint;

/// Canonical signed semantic temporal displacement.
///
/// This is exactly the same Rust type as
/// `crate::quantum::ir::timing::TimeOffset`.
pub use time::TimeOffset;