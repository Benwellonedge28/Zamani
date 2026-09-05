//! Zamani Quantum Scheduling — Dynamic Scheduling Namespace
//!
//! Production-grade namespace boundary for dynamic quantum/classical
//! scheduling.
//!
//! # Purpose
//!
//! `quantum::scheduling::dynamic` contains scheduling metadata and contracts
//! required when the execution of a quantum program depends on information
//! that becomes available during compilation or execution.
//!
//! Dynamic scheduling covers, without being limited to:
//!
//! - measurement-dependent execution;
//! - classical dependencies;
//! - conditional quantum operations;
//! - runtime conditions;
//! - classical feedback;
//! - measurement-to-classical processing latency;
//! - runtime-observed readiness;
//! - runtime events;
//! - dynamically released operations;
//! - QEC/decoder-produced classical information;
//! - distributed classical dependencies;
//! - operations whose exact execution time cannot be completely determined
//!   during static compilation.
//!
//! This module is intentionally a namespace and integration boundary.
//! Algorithms and semantic implementations belong to the child modules.
//!
//! # Architectural principle
//!
//! Zamani quantum scheduling answers:
//!
//! > When may an operation execute?
//!
//! Dynamic scheduling extends that question to:
//!
//! > When may an operation execute when some of the information required to
//! > determine its eligibility becomes available only at runtime?
//!
//! The dynamic scheduler therefore sits between canonical semantic IR and
//! target/runtime scheduling:
//!
//! ```text
//!                     Zamani source
//!                           |
//!                           v
//!                   quantum::frontend
//!                           |
//!                           v
//!                    quantum::ir
//!                           |
//!                           v
//!                  optimization
//!                           |
//!                           v
//!                       routing
//!                           |
//!                           v
//!                    scheduling
//!                           |
//!             +-------------+-------------+
//!             |                           |
//!             v                           v
//!        static scheduling          dynamic scheduling
//!                                         |
//!                    +--------------------+--------------------+
//!                    |                    |                    |
//!                    v                    v                    v
//!               classical            conditional           feedback
//!                    |                    |                    |
//!                    +--------------------+--------------------+
//!                                         |
//!                                         v
//!                                  runtime events
//!                                         |
//!                                         v
//!                                  final eligibility
//!                                         |
//!                                         v
//!                                  resource/timing
//!                                  scheduling
//!                                         |
//!                                         v
//!                                     hardware
//! ```
//!
//! # Semantic separation
//!
//! Dynamic scheduling MUST NOT become a second quantum IR.
//!
//! Canonical quantum semantics remain owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! Canonical logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Canonical operation identity remains owned by the IR identity subsystem.
//!
//! This namespace MUST NOT define replacement:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - `QuantumOperation`;
//! - `QuantumCircuit`;
//! - gate semantics;
//! - measurement semantics;
//! - classical language semantics;
//! - hardware resources;
//! - hardware capabilities;
//! - routing semantics.
//!
//! Dynamic scheduling only represents the information necessary to determine
//! execution eligibility and timing.
//!
//! # Child-module ownership
//!
//! ```text
//! dynamic/
//! ├── mod.rs
//! │
//! ├── classical.rs
//! │   Classical readiness and scheduler-visible classical dependencies.
//! │
//! ├── conditional.rs
//! │   Conditional operation eligibility and condition-related scheduling
//! │   metadata.
//! │
//! ├── feedback.rs
//! │   Measurement/classical-result feedback relationships and readiness.
//! │
//! └── runtime.rs
//!     Runtime events and execution-time scheduling state.
//! ```
//!
//! The current repository already contains these four implementation files.
//! `mod.rs` therefore declares exactly those existing modules and does not
//! speculate about future modules.
//!
//! # `classical.rs`
//!
//! Owns scheduler-visible classical dependency information.
//!
//! It models concepts such as:
//!
//! ```text
//! measurement
//!      |
//!      v
//! classical value
//!      |
//!      v
//! classical processing
//!      |
//!      v
//! condition-ready
//! ```
//!
//! It does not execute classical code.
//!
//! Target-dependent classical latency must enter through the scheduler context
//! and target model rather than being hard-coded here.
//!
//! # `conditional.rs`
//!
//! Owns scheduling metadata for conditional execution.
//!
//! It distinguishes, where applicable:
//!
//! - unconditional execution;
//! - compile-time-known conditions;
//! - runtime conditions;
//! - impossible execution paths.
//!
//! The condition itself remains owned by canonical IR control semantics.
//!
//! The dynamic scheduler consumes condition information; it does not redefine
//! the meaning of the condition.
//!
//! # `feedback.rs`
//!
//! Owns feedback-specific scheduling relationships.
//!
//! Typical flow:
//!
//! ```text
//! quantum operation
//!       |
//!       v
//! measurement
//!       |
//!       v
//! classical result
//!       |
//!       v
//! classical processing
//!       |
//!       v
//! feedback readiness
//!       |
//!       v
//! quantum operation
//! ```
//!
//! Feedback latency is target-dependent and must not be represented by a
//! scheduler-wide constant.
//!
//! # `runtime.rs`
//!
//! Owns runtime-dependent scheduling state and event relationships.
//!
//! It exists because a dynamic quantum program may not be statically reducible
//! to one completely known schedule.
//!
//! Examples include:
//!
//! - measurement result arrival;
//! - runtime classical computation completion;
//! - conditional readiness;
//! - feedback completion;
//! - dynamic resource release;
//! - runtime event notification;
//! - distributed classical message arrival;
//! - QEC decoder readiness.
//!
//! The runtime module describes scheduling state. It does not execute the
//! runtime itself.
//!
//! # Static versus dynamic scheduling
//!
//! The namespace supports a continuum rather than a binary model:
//!
//! ```text
//! fully static
//!      |
//!      +---- compile-time conditional
//!      |
//!      +---- target-dependent timing
//!      |
//!      +---- runtime classical dependency
//!      |
//!      +---- runtime measurement dependency
//!      |
//!      +---- runtime feedback
//!      |
//!      +---- runtime resource availability
//!      |
//!      +---- distributed runtime event
//!      |
//! fully dynamic
//! ```
//!
//! A scheduler implementation must not assume that every program is a static
//! DAG with predetermined timestamps.
//!
//! # Dynamic dependency model
//!
//! Dynamic execution can be represented conceptually as:
//!
//! ```text
//! producer(s)
//!      |
//!      v
//! value/event
//!      |
//!      v
//! classical processing
//!      |
//!      v
//! predicate / condition
//!      |
//!      v
//! eligibility
//!      |
//!      v
//! resource availability
//!      |
//!      v
//! timing constraints
//!      |
//!      v
//! executable
//! ```
//!
//! Each stage can introduce a dependency.
//!
//! The scheduler must therefore combine:
//!
//! - quantum dependencies;
//! - classical dependencies;
//! - control dependencies;
//! - resource dependencies;
//! - timing dependencies;
//! - communication dependencies;
//! - QEC dependencies;
//! - runtime-event dependencies.
//!
//! # Runtime readiness
//!
//! Dynamic scheduling must distinguish at least:
//!
//! ```text
//! Pending
//! Ready
//! Unavailable
//! Impossible
//! ```
//!
//! These states have different meanings.
//!
//! `Pending` means information has not arrived yet.
//!
//! `Ready` means the prerequisite is satisfied.
//!
//! `Unavailable` means execution cannot proceed at the current observation
//! point but may become possible later.
//!
//! `Impossible` means the dependency cannot become satisfied and therefore
//! requires explicit handling rather than indefinite waiting.
//!
//! The concrete readiness representation is owned by the child implementation
//! that models it.
//!
//! # No hidden execution
//!
//! This namespace must never:
//!
//! - contact a QPU;
//! - invoke a backend SDK;
//! - perform network I/O;
//! - execute classical programs;
//! - execute quantum gates;
//! - mutate hardware state;
//! - discover hardware;
//! - allocate physical qubits;
//! - perform logical-to-physical routing;
//! - decode QEC syndromes.
//!
//! Those operations belong to their respective subsystems.
//!
//! Dynamic scheduling is a planning and eligibility layer.
//!
//! # Resource independence
//!
//! Dynamic scheduling does not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of classical processors;
//! - a fixed number of controllers;
//! - a fixed number of measurement channels;
//! - a fixed number of feedback channels;
//! - a fixed number of runtime nodes;
//! - a fixed number of QPUs;
//! - a fixed communication topology.
//!
//! Resource information comes from the scheduling context and hardware/runtime
//! adapters.
//!
//! # Timing independence
//!
//! This namespace contains no hardware timing constants.
//!
//! It must not assume:
//!
//! - nanoseconds;
//! - microseconds;
//! - device ticks;
//! - a particular controller clock;
//! - a particular measurement latency;
//! - a particular classical-processing latency;
//! - a particular network latency.
//!
//! Timing information is supplied by the scheduling timing model and target
//! adapters.
//!
//! # No artificial scalability ceiling
//!
//! Dynamic scheduling introduces no architectural maximum for:
//!
//! - quantum operations;
//! - classical dependencies;
//! - conditions;
//! - feedback paths;
//! - runtime events;
//! - branches;
//! - measurements;
//! - qubits;
//! - QEC rounds;
//! - scheduling depth;
//! - distributed nodes;
//! - communication events.
//!
//! "Infinite scalability" is interpreted correctly as:
//!
//! > this namespace introduces no artificial finite machine-size ceiling.
//!
//! A concrete Rust process is necessarily bounded by:
//!
//! - available memory;
//! - address space;
//! - CPU capacity;
//! - explicit compiler policy;
//! - target capacity;
//! - runtime capacity;
//! - operating-system constraints.
//!
//! Those are execution constraints, not architectural limits encoded here.
//!
//! # Identity policy
//!
//! Scheduler-local identities may be defined by child implementations when a
//! scheduler-specific identity is genuinely required.
//!
//! However, canonical quantum identities MUST NOT be duplicated.
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! remain authoritative.
//!
//! Dynamic scheduling must therefore refer to canonical qubit identities
//! instead of defining another qubit namespace.
//!
//! # Operation identity policy
//!
//! Dynamic scheduling metadata refers to canonical IR operations.
//!
//! It must not embed an independent implementation of a quantum operation.
//!
//! This permits the same dynamic scheduler to operate over:
//!
//! - ordinary gates;
//! - measurements;
//! - resets;
//! - logical operations;
//! - pulse operations;
//! - analog operations;
//! - Hamiltonian evolution;
//! - distributed operations;
//! - dialect operations;
//! - future operation classes.
//!
//! # Dependency direction
//!
//! The dependency direction is strictly downstream:
//!
//! ```text
//! quantum::ir
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! control semantics     program semantics
//!      |                    |
//!      +---------+----------+
//!                |
//!                v
//!          scheduling
//!                |
//!                v
//!        scheduling::dynamic
//!                |
//!       +--------+--------+
//!       |        |        |
//!       v        v        v
//!   classical conditional feedback
//!       |        |        |
//!       +--------+--------+
//!                |
//!                v
//!             runtime
//!                |
//!                v
//!         hardware/runtime
//! ```
//!
//! Dynamic scheduling MUST NOT introduce reverse dependencies from canonical IR
//! to scheduling.
//!
//! # Integration with `quantum::scheduling::types`
//!
//! Dynamic child modules may consume scheduler-owned timing values such as:
//!
//! ```text
//! crate::quantum::scheduling::types::TimePoint
//! crate::quantum::scheduling::types::Duration
//! ```
//!
//! These values remain abstract and target-independent.
//!
//! Dynamic scheduling must not reinterpret them as a particular physical unit.
//!
//! # Integration with `quantum::scheduling::context`
//!
//! The dynamic subsystem consumes target/context information through the
//! scheduler context rather than discovering the target itself.
//!
//! Conceptually:
//!
//! ```text
//! hardware capabilities
//!        |
//!        v
//! scheduling adapter
//!        |
//!        v
//! SchedulingContext
//!        |
//!        +-------------------------+
//!        |                         |
//!        v                         v
//! static scheduling       dynamic scheduling
//!                                  |
//!                                  v
//!                         classical/feedback/runtime
//! ```
//!
//! This prevents dynamic scheduling from becoming coupled to a particular
//! hardware provider.
//!
//! # Integration with dependency analysis
//!
//! Dynamic scheduling supplements ordinary dependency analysis.
//!
//! A static dependency can be represented as:
//!
//! ```text
//! A -> B
//! ```
//!
//! A dynamic dependency may instead be:
//!
//! ```text
//! A
//! |
//! v
//! measurement
//! |
//! v
//! classical result
//! |
//! v
//! condition
//! |
//! v
//! B
//! ```
//!
//! The dependency subsystem remains responsible for the canonical scheduling
//! dependency graph. The dynamic namespace supplies additional eligibility
//! information.
//!
//! # Integration with resources
//!
//! A runtime-ready operation is not automatically executable.
//!
//! The complete decision is:
//!
//! ```text
//! dynamic readiness
//!        AND
//! dependency readiness
//!        AND
//! resource availability
//!        AND
//! timing constraints
//!        AND
//! target capability
//!        AND
//! control constraints
//!        =
//! executable
//! ```
//!
//! Therefore dynamic scheduling must not reserve resources directly merely
//! because a condition became ready.
//!
//! Resource reservation remains owned by the planner/resource subsystem.
//!
//! # Integration with planners
//!
//! Planners consume dynamic eligibility information.
//!
//! The intended relationship is:
//!
//! ```text
//! dynamic metadata
//!       |
//!       v
//! planner ready-set
//!       |
//!       v
//! resource feasibility
//!       |
//!       v
//! timing feasibility
//!       |
//!       v
//! scheduling decision
//! ```
//!
//! Dynamic modules do not choose the global scheduling algorithm.
//!
//! ASAP, ALAP, list scheduling, critical-path scheduling, resource-constrained
//! scheduling, and adaptive scheduling remain planner/algorithm concerns.
//!
//! # Integration with conditional scheduling
//!
//! Conditional execution is not equivalent to simply delaying an operation.
//!
//! A scheduler must preserve the distinction between:
//!
//! ```text
//! operation not yet ready
//!
//! operation ready but resource unavailable
//!
//! operation ready but timing-constrained
//!
//! operation belongs to an unselected branch
//!
//! operation is permanently impossible
//! ```
//!
//! The conditional child module owns conditional eligibility metadata so that
//! planners can make these distinctions without changing canonical condition
//! semantics.
//!
//! # Integration with feedback
//!
//! Feedback chains must preserve causality:
//!
//! ```text
//! measurement completion
//!        |
//!        v
//! result availability
//!        |
//!        v
//! classical processing
//!        |
//!        v
//! feedback readiness
//!        |
//!        v
//! controlled quantum operation
//! ```
//!
//! The scheduler must never assume that a measurement result is available at
//! the same instant as measurement start.
//!
//! The actual latency must come from the target/context/runtime model.
//!
//! # Integration with runtime
//!
//! Static compilation may produce a partially resolved schedule.
//!
//! Runtime scheduling may subsequently resolve:
//!
//! - event arrival;
//! - condition result;
//! - classical readiness;
//! - dynamic resource availability;
//! - communication completion;
//! - decoder readiness.
//!
//! The runtime integration must preserve the original semantic operation
//! identity and provenance.
//!
//! # Integration with QEC
//!
//! QEC may produce runtime classical information that controls future quantum
//! operations.
//!
//! The relationship is:
//!
//! ```text
//! QEC subsystem
//!      |
//!      v
//! syndrome / decoder result
//!      |
//!      v
//! dynamic classical dependency
//!      |
//!      v
//! conditional / feedback eligibility
//!      |
//!      v
//! scheduler
//! ```
//!
//! The dynamic subsystem does not decode QEC results.
//!
//! # Integration with distributed scheduling
//!
//! Dynamic dependencies may cross scheduling domains:
//!
//! ```text
//! quantum node A
//!      |
//!      v
//! measurement
//!      |
//!      v
//! classical message
//!      |
//!      v
//! network
//!      |
//!      v
//! quantum node B
//!      |
//!      v
//! conditional operation
//! ```
//!
//! Communication latency and network-resource constraints belong to the
//! distributed/resource/timing subsystems.
//!
//! This namespace only exposes the dependency/readiness boundary needed by
//! those systems.
//!
//! # Determinism
//!
//! Namespace composition must be deterministic.
//!
//! Dynamic child implementations are expected to use deterministic ordering
//! wherever ordering affects:
//!
//! - public iteration;
//! - diagnostics;
//! - serialization;
//! - hashing;
//! - reproducibility;
//! - scheduler arbitration.
//!
//! This file contains no global mutable state and no implicit randomness.
//!
//! # Thread safety
//!
//! `mod.rs` owns no mutable state.
//!
//! The child modules should continue to use ordinary owned Rust values and
//! explicit synchronization boundaries where concurrency is required.
//!
//! The namespace itself therefore introduces no thread-safety hazard.
//!
//! # Error ownership
//!
//! Dynamic modules own their domain-specific validation/readiness errors where
//! necessary.
//!
//! They should integrate with the scheduler's canonical error boundary rather
//! than introducing string-based control flow.
//!
//! `mod.rs` must not define a second scheduler error hierarchy.
//!
//! # Serialization
//!
//! Dynamic metadata may eventually be serialized as part of the complete
//! scheduling representation.
//!
//! Serialization ownership belongs to:
//!
//! ```text
//! quantum::scheduling::serialization
//! ```
//!
//! This namespace does not define a competing serialization format.
//!
//! # Diagnostics
//!
//! Dynamic scheduling should support explanations such as:
//!
//! ```text
//! operation X is waiting for classical value Y
//!
//! operation X is waiting for runtime event Z
//!
//! operation X became ready at T
//!
//! operation X could not start because resource R was unavailable
//! ```
//!
//! The diagnostic/explanation subsystem remains responsible for presentation.
//!
//! # Security and resource policy
//!
//! This namespace must never interpret "no limit" as permission to bypass an
//! explicit compiler, runtime, or security policy.
//!
//! Architectural scalability and operational limits are separate:
//!
//! ```text
//! architecture
//!      !=
//! resource policy
//!      !=
//! hardware capability
//!      !=
//! runtime policy
//! ```
//!
//! An explicit limit supplied by a caller is valid and must be respected.
//! What is prohibited is silently introducing a machine-size limit inside this
//! namespace.
//!
//! # Safety
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! Unsafe Rust is compiler-forbidden below.
//!
//! # API stability
//!
//! `mod.rs` is a public namespace boundary and should change rarely.
//!
//! Adding implementation details to `classical.rs`, `conditional.rs`,
//! `feedback.rs`, or `runtime.rs` must not require changing this file unless:
//!
//! - a new child module is added;
//! - an existing child module is removed;
//! - a public namespace policy changes.
//!
//! In particular, adding a new public type to one child module must not require
//! modifying this file merely to expose it.
//!
//! This intentionally favors stable explicit module paths over broad glob
//! re-exports.
//!
//! # Public API rule
//!
//! Consumers should use explicit paths when ownership matters:
//!
//! ```text
//! crate::quantum::scheduling::dynamic::classical
//! crate::quantum::scheduling::dynamic::conditional
//! crate::quantum::scheduling::dynamic::feedback
//! crate::quantum::scheduling::dynamic::runtime
//! ```
//!
//! This prevents unrelated future additions from silently changing the public
//! namespace through wildcard exports.
//!
//! # No speculative declarations
//!
//! Only physically existing child modules are declared here.
//!
//! Future modules such as:
//!
//! - dynamic/distributed.rs;
//! - dynamic/events.rs;
//! - dynamic/state.rs;
//! - dynamic/transactions.rs;
//! - dynamic/parallel.rs;
//!
//! must not be declared until their implementation and contract exist.
//!
//! This keeps the repository compilable at every intermediate implementation
//! stage.
//!
//! # Testing contract
//!
//! Semantic tests belong to the child implementations and integration-test
//! suites.
//!
//! This namespace only needs to verify:
//!
//! 1. all child modules are reachable;
//! 2. no duplicate namespace definitions exist;
//! 3. the module path is stable;
//! 4. unsafe Rust remains forbidden.
//!
//! # Architectural invariant
//!
//! The most important invariant is:
//!
//! ```text
//! canonical quantum semantics
//!              |
//!              v
//! dynamic scheduling metadata
//!              |
//!              v
//! scheduler/planner
//!              |
//!              v
//! target-aware execution schedule
//! ```
//!
//! Never reverse that dependency.
//!
//! # Implementation
//!
//! Keep this file declarative.
//!
//! Domain logic belongs in the four child modules.
//!
//! ============================================================================
//! Compiler-enforced safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// ============================================================================
// Dynamic scheduling child modules
// ============================================================================

/// Classical dependency and readiness modelling.
///
/// This module owns scheduler-visible classical signals, classical dependency
/// requirements, readiness states, and related metadata.
///
/// It does not execute classical programs, own hardware resources, or select
/// the global scheduling algorithm.
pub mod classical;

/// Conditional execution scheduling metadata.
///
/// This module owns the scheduler-facing representation of conditional
/// eligibility while consuming canonical IR condition semantics.
pub mod conditional;

/// Measurement/classical feedback scheduling metadata.
///
/// This module owns the relationship between producing events, classical
/// processing, feedback readiness, and dependent quantum operations.
pub mod feedback;

/// Runtime-dependent scheduling metadata.
///
/// This module owns runtime events, dynamic readiness, and execution-time
/// scheduling state without executing the runtime or contacting hardware.
pub mod runtime;

// ============================================================================
// Namespace-level API helpers
// ============================================================================

/// Returns the canonical namespace path for dynamic scheduling.
///
/// This is intentionally a constant diagnostic/tooling contract. It does not
/// create scheduler state and does not encode a hardware assumption.
#[must_use]
pub const fn module_path() -> &'static str {
    "quantum::scheduling::dynamic"
}

/// Returns the canonical logical-qubit identity path used by dynamic
/// scheduling.
///
/// Dynamic scheduling does not define a replacement qubit identity.
#[must_use]
pub const fn logical_qubit_identity_path() -> &'static str {
    "quantum::ir::qubit::QubitId"
}

/// Returns the canonical physical-qubit identity path used by dynamic
/// scheduling.
///
/// Dynamic scheduling does not define a replacement physical-qubit identity.
#[must_use]
pub const fn physical_qubit_identity_path() -> &'static str {
    "quantum::ir::qubit::PhysicalQubitId"
}

/// Returns the canonical operation identity path used by dynamic scheduling.
///
/// The dynamic subsystem refers to canonical IR operations rather than
/// defining another operation identity system.
#[must_use]
pub const fn operation_identity_path() -> &'static str {
    "quantum::ir::core::identity::OperationId"
}

// ============================================================================
// Namespace-level tests
// ============================================================================
//
// These tests intentionally do not instantiate implementation types from the
// child modules. That keeps this composition root independent of changes to
// their internal APIs.
//
// Child modules own their semantic tests.
// Repository integration tests own cross-subsystem behavior.

#[cfg(test)]
mod tests {
    use super::{
        logical_qubit_identity_path,
        module_path,
        operation_identity_path,
        physical_qubit_identity_path,
    };

    #[test]
    fn namespace_path_is_stable() {
        assert_eq!(
            module_path(),
            "quantum::scheduling::dynamic"
        );
    }

    #[test]
    fn logical_qubit_identity_is_canonical() {
        assert_eq!(
            logical_qubit_identity_path(),
            "quantum::ir::qubit::QubitId"
        );
    }

    #[test]
    fn physical_qubit_identity_is_canonical() {
        assert_eq!(
            physical_qubit_identity_path(),
            "quantum::ir::qubit::PhysicalQubitId"
        );
    }

    #[test]
    fn operation_identity_is_canonical() {
        assert_eq!(
            operation_identity_path(),
            "quantum::ir::core::identity::OperationId"
        );
    }
}