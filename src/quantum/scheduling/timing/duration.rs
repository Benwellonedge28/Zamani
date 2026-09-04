//! Zamani Quantum Scheduling — Canonical Duration Integration
//!
//! Path:
//!     src/quantum/scheduling/timing/duration.rs
//!
//! # Purpose
//!
//! This module defines the scheduling-layer boundary for semantic quantum
//! durations.
//!
//! IMPORTANT:
//!
//! `Duration` is owned by the canonical quantum IR timing subsystem:
//!
//!     crate::quantum::ir::timing::Duration
//!
//! This module MUST NOT define another duration representation.
//!
//! Scheduling consumes semantic durations; it does not redefine their meaning.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      v
//! quantum::ir::timing::Duration
//!      |
//!      v
//! quantum::scheduling::timing::duration
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! dependency/resource scheduling   timing policies
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!              scheduled result
//!                    |
//!                    v
//!              quantum::hardware
//! ```
//!
//! # Ownership
//!
//! The canonical IR timing layer owns:
//!
//! - the semantic meaning of duration;
//! - exact representation;
//! - unit conversion;
//! - checked duration arithmetic;
//! - parsing;
//! - canonical formatting;
//! - semantic equality;
//! - semantic hashing;
//! - serialization meaning.
//!
//! The scheduling layer owns:
//!
//! - when an operation may begin;
//! - when an operation completes;
//! - resource reservations;
//! - temporal dependencies;
//! - resource conflicts;
//! - scheduling policies;
//! - schedule construction;
//! - schedule verification.
//!
//! This module therefore provides the stable scheduling import boundary without
//! creating a second semantic duration type.
//!
//! # Why this is a re-export rather than a new type
//!
//! Defining another `Duration` here would create two competing concepts:
//
//! ```text
//! quantum::ir::timing::Duration
//! quantum::scheduling::timing::Duration
//! ```
//!
//! That would make otherwise equivalent values distinct Rust types and would
//! force adapters to repeatedly convert between identical semantic values.
//!
//! More importantly, it would allow the IR and scheduler to disagree about
//! what a duration means.
//!
//! Zamani must have exactly one canonical semantic duration representation.
//!
//! The scheduling layer therefore imports and re-exports the canonical type.
//!
//! # Scalability
//!
//! This module imposes NO scheduling-size limit.
//!
//! It contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum schedule depth;
//! - maximum machine size;
//! - maximum duration;
//! - fixed resource count;
//! - fixed topology;
//! - fixed number of channels;
//! - vendor-specific clock;
//! - hardware-specific sample period.
//!
//! Any resource or execution limit belongs to the appropriate policy,
//! capability, hardware, or compilation-limit subsystem.
//!
//! The canonical duration representation is an exact `u128` attosecond value.
//! That representation is finite and bounded by the Rust integer type itself,
//! but it is NOT a quantum-machine-size limit.
//!
//! # Exactness
//!
//! Scheduling MUST NOT use floating-point durations.
//!
//! A duration such as:
//!
//!     5.25 ns
//!
//! must remain exactly representable by the canonical timing layer.
//!
//! Hardware-specific timing such as:
//!
//!     dt = 0.222 ns
//!
//! belongs to `quantum::hardware::timing` and the hardware adaptation boundary.
//!
//! The scheduler consumes semantic duration and target timing constraints; it
//! does not reinterpret semantic duration as a provider clock tick.
//!
//! # Hardware integration
//!
//! The intended boundary is:
//!
//! ```text
//! quantum::ir::timing::Duration
//!             |
//!             v
//! scheduling::timing::duration
//!             |
//!             v
//! scheduling context
//!             |
//!             v
//! hardware timing adapter
//!             |
//!             v
//! provider/device timing representation
//! ```
//!
//! Hardware-specific conversion MUST remain explicit and checked.
//!
//! The scheduler must never silently round a semantic duration merely because
//! a particular device has a coarser timing grid.
//!
//! # Routing integration
//!
//! Routing determines WHERE an operation executes.
//!
//! Scheduling determines WHEN it executes.
//!
//! ```text
//! routing
//!     |
//!     | physical qubit/resource assignment
//!     v
//! scheduling
//!     |
//!     | start + duration + resource reservations
//!     v
//! hardware execution
//! ```
//!
//! This duration boundary is therefore usable by scheduling after routing
//! without importing routing-specific types.
//!
//! # Qubit integration
//!
//! This file intentionally does NOT import:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! A duration is not a qubit identity and does not contain an operand list.
//!
//! Objects that associate a duration with an operation MUST use the canonical
//! qubit type through the owning operation/resource model:
//
//!     crate::quantum::ir::qubit::QubitId
//!
//! Keeping that dependency out of this primitive prevents unnecessary coupling
//! and allows the same duration to represent:
//!
//! - quantum-gate execution;
//! - measurement;
//! - reset;
//! - pulse intervals;
//! - analog evolution;
//! - classical processing;
//! - feed-forward latency;
//! - communication latency;
//! - synchronization;
//! - QEC activity;
//! - distributed execution.
//!
//! # QEC integration
//!
//! QEC schedulers may use this type for:
//!
//! - syndrome-extraction duration;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurement duration;
//! - decoder latency;
//! - feedback latency;
//! - inter-round spacing.
//!
//! QEC-specific meaning must remain in the QEC subsystem.
//!
//! This file must not depend on a particular error-correcting code.
//!
//! # Dynamic-circuit integration
//!
//! Dynamic scheduling may use the canonical duration for:
//!
//! - measurement latency;
//! - classical computation latency;
//! - conditional-operation readiness;
//! - feedback latency;
//! - communication latency.
//!
//! A duration does not imply that execution is statically schedulable.
//!
//! Runtime-dependent timing belongs to the dynamic scheduling/runtime layers.
//!
//! # Distributed integration
//!
//! The same duration type is valid for:
//!
//! - local execution;
//! - inter-chip communication;
//! - inter-QPU communication;
//! - synchronization;
//! - distributed quantum networking.
//!
//! Network-specific semantics must remain in the distributed scheduling layer.
//!
//! # Determinism
//!
//! Because the canonical duration is an exact value type, it is suitable for
//! deterministic scheduling.
//!
//! Given identical:
//!
//! - canonical IR;
//! - target description;
//! - resource model;
//! - timing model;
//! - scheduling policy;
//! - constraints;
//! - calibration snapshot;
//! - deterministic scheduler configuration;
//!
//! the same duration values must participate in identical scheduling decisions.
//!
//! This module contains no randomness and no hidden clock access.
//!
//! # Thread safety
//!
//! The canonical duration is a small immutable value type. This re-export adds
//! no mutable state and no synchronization requirements.
//!
//! # Unsafe code
//!
//! Unsafe Rust is forbidden.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependency direction
//!
//! This module may depend on:
//!
//! - `quantum::ir::timing`.
//!
//! It must NOT depend on:
//!
//! - hardware providers;
//! - backend SDKs;
//! - routing implementations;
//! - scheduling algorithms;
//! - QEC implementations;
//! - runtime;
//! - networking;
//! - operating-system clocks;
//! - simulators;
//! - benchmarking implementations.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir::timing
//!          |
//!          v
//! quantum::scheduling::timing::duration
//!          |
//!          v
//! scheduling algorithms
//! ```
//!
//! # API stability
//!
//! The scheduling subsystem should import `Duration` through this module when
//! it wants an explicitly scheduling-owned path:
//
//!     crate::quantum::scheduling::timing::duration::Duration
//!
//! Internally, that name resolves to the canonical IR type.
//!
//! This gives Zamani a stable scheduling namespace without introducing a
//! duplicate semantic representation.
//!
//! # Integration contract
//!
//! `src/quantum/scheduling/timing/mod.rs` should expose this module:
//
//!     pub mod duration;
//!
//! Consumers may then use:
//
//!     use crate::quantum::scheduling::timing::duration::Duration;
//!
//! A scheduling root may additionally re-export the type if desired:
//
//!     pub use timing::duration::Duration;
//!
//! No conversion layer is required because the scheduling type IS the
//! canonical IR duration.
//!
//! # Migration contract
//!
//! Existing scheduling code that defines its own `Duration` must migrate to
//! this canonical type.
//!
//! In particular, scheduling implementations must not retain independent
//! duration structs with:
//!
//! - fixed maximum durations;
//! - fixed unit scales;
//! - `u64`-only ranges;
//! - floating-point values;
//! - provider-specific clock assumptions.
//!
//! Existing hardware scheduling code that currently defines a separate
//! scheduling `Duration` must eventually consume this canonical semantic
//! duration through an explicit hardware/scheduling adapter.
//!
//! # Important distinction
//!
//! There are three different concepts in Zamani:
//
//! ```text
//! Semantic Duration
//!     |
//!     | how much elapsed semantic time
//!     v
//! quantum::ir::timing::Duration
//!
//! Schedule Time
//!     |
//!     | when an operation starts/finishes
//!     v
//! scheduling time-point / interval types
//!
//! Hardware Time
//!     |
//!     | how a target realizes semantic time
//!     v
//! quantum::hardware::timing
//! ```
//!
//! These must never be collapsed into one type.
//!
//! # No hard-coded machine assumptions
//!
//! This file deliberately contains no constants such as:
//
//!     MAX_QUBITS
//!     MAX_CHANNELS
//!     MAX_DURATION
//!     MAX_OPERATIONS
//!     MAX_SCHEDULE_DEPTH
//!
//! Those would incorrectly turn a semantic duration primitive into a hardware
//! policy layer.
//!
//! # Testing contract
//!
//! Because this module is a canonical re-export, its correctness is primarily
//! an integration invariant:
//!
//! 1. The exported `Duration` must be exactly the canonical IR `Duration`.
//! 2. No second duration representation may be introduced here.
//! 3. No unsafe code may exist.
//! 4. No hardware dependency may exist.
//! 5. No qubit dependency may exist.
//! 6. No scheduling global state may exist.
//! 7. No floating-point timing may be introduced.
//!
//! Behavioral tests for construction, parsing, arithmetic, comparison,
//! formatting, serialization, and exactness belong to the canonical IR timing
//! implementation.
//!
//! Scheduling tests should verify that scheduling operations consume this type
//! without conversion or semantic duplication.
//!
//! # Completion criterion
//!
//! This file is complete when scheduling can depend on:
//
//!     scheduling::timing::duration::Duration
//!
//! while the actual semantic type remains:
//
//!     quantum::ir::timing::Duration
//!
//! and no scheduler component needs to reopen this file merely because another
//! scheduling subsystem is added.
//!
//! -----------------------------------------------------------------------------
//! Canonical implementation boundary:
//!
//!     quantum::ir::timing::Duration
//!
//! Scheduling consumes it.
//! Scheduling does not redefine it.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Canonical duration re-export
// -----------------------------------------------------------------------------
//
// DO NOT replace this with a locally-defined Duration.
//
// The canonical semantic duration belongs to quantum::ir::timing.
//
// Keeping this as a direct re-export guarantees that:
//
//     quantum::ir::timing::Duration
//
// and:
//
//     quantum::scheduling::timing::duration::Duration
//
// are the same Rust type.
//
// Consequently, no conversion function, wrapper allocation, or duplicate
// semantic representation is necessary at the scheduling boundary.

pub use crate::quantum::ir::timing::Duration;