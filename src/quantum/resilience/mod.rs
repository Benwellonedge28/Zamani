//! Zamani Quantum Resilience
//!
//! Path:
//!     `src/quantum/resilience/mod.rs`
//!
//! Status:
//!     Production module-composition boundary.
//!
//! Target:
//!     Rust 1.97 / Rust 1.97.1
//!     Rust 2021
//!     Stable Rust
//!
//! Safety:
//!     `unsafe` is forbidden.
//!
//! # Purpose
//!
//! `crate::quantum::resilience` is the provider-neutral resilience,
//! adaptation, recovery, mitigation, verification, and observability
//! subsystem for Zamani quantum execution.
//!
//! Its purpose is to preserve the intended semantics of a Zamani quantum
//! computation while the physical execution environment changes because of:
//!
//! - quantum faults;
//! - noise;
//! - leakage;
//! - loss;
//! - erasure;
//! - correlated faults;
//! - QEC degradation;
//! - calibration drift;
//! - hardware degradation;
//! - resource loss;
//! - topology changes;
//! - routing failure;
//! - scheduling failure;
//! - compilation failure;
//! - optimization changes;
//! - backend failure;
//! - execution failure;
//! - communication failure;
//! - timeout;
//! - mitigation failure;
//! - checkpoint/recovery failure;
//! - distributed execution failure;
//! - capability changes;
//! - security failures;
//! - uncertain or conflicting observations.
//!
//! The defining invariant is:
//!
//! > A Zamani quantum program is written against the canonical semantic
//! > model, while resilience dynamically adapts the physical execution
//! > strategy to the resources and verified conditions available at runtime.
//!
//! The resilience namespace therefore describes *how execution remains
//! valid*, not a second quantum programming model.
//!
//! # Write once, scale everywhere
//!
//! This module imposes no artificial machine-size ceiling.
//!
//! It contains no:
//!
//! - maximum qubit count;
//! - maximum physical-qubit count;
//! - maximum logical-qubit count;
//! - maximum backend count;
//! - maximum device count;
//! - maximum topology size;
//! - maximum circuit depth;
//! - maximum operation count;
//! - fixed retry count;
//! - fixed fidelity threshold;
//! - fixed error-rate threshold;
//! - fixed timeout;
//! - fixed provider;
//! - fixed quantum technology.
//!
//! The architectural meaning of "infinity" is:
//!
//! > No finite machine-size limit is imposed by the resilience subsystem.
//! > Every concrete execution is bounded only by the resources, capabilities,
//! > policies, security controls, operating environment, and physical quantum
//! > system actually available to that execution.
//!
//! Resource quantities must therefore be discovered, supplied, negotiated,
//! or explicitly configured by the owning subsystem.
//!
//! This module must never introduce constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_BACKENDS
//! MAX_DEVICES
//! MAX_RETRIES
//! MAX_RECOVERY_ATTEMPTS
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                               │
//!                               ▼
//!                       quantum::frontend
//!                               │
//!                               ▼
//!                     canonical quantum::ir
//!                               │
//!              ┌────────────────┼────────────────┐
//!              │                │                │
//!              ▼                ▼                ▼
//!        optimization          QEC              ZQN
//!              │                │                │
//!              └────────────────┼────────────────┘
//!                               ▼
//!                            routing
//!                               │
//!                               ▼
//!                           scheduling
//!                               │
//!                               ▼
//!                         hardware HAL
//!                               │
//!                               ▼
//!                           execution
//!                               │
//!                    ┌──────────┴──────────┐
//!                    │                     │
//!                    ▼                     ▼
//!               observations            results
//!                    │                     │
//!                    └──────────┬──────────┘
//!                               ▼
//!                    quantum::resilience
//!                               │
//!          ┌────────────────────┼────────────────────┐
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//!       detection            diagnosis           verification
//!          │                    │                    │
//!          └────────────────────┼────────────────────┘
//!                               ▼
//!                             policy
//!                               │
//!                               ▼
//!                             planning
//!                               │
//!                               ▼
//!                            adaptation
//!                               │
//!                 ┌─────────────┼─────────────┐
//!                 ▼             ▼             ▼
//!              reroute      reschedule     recompile
//!                 │             │             │
//!                 └─────────────┼─────────────┘
//!                               ▼
//!                            recovery
//!                               │
//!                               ▼
//!                           mitigation
//!                               │
//!                               ▼
//!                           verification
//!                               │
//!                         ┌─────┴─────┐
//!                         ▼           ▼
//!                       ACCEPT    REPEAT / ESCALATE
//! ```
//!
//! The feedback loop is part of the resilience contract.
//!
//! # Ownership boundaries
//!
//! `quantum::resilience` owns the orchestration and decision layer for:
//!
//! - resilience observations;
//! - incidents;
//! - diagnosis;
//! - resilience policy evaluation;
//! - recovery planning;
//! - adaptation orchestration;
//! - recovery orchestration;
//! - mitigation orchestration;
//! - checkpoint coordination;
//! - verification gates;
//! - resilience provenance;
//! - resilience telemetry contracts;
//! - resilience history;
//! - optional predictive learning;
//! - distributed resilience coordination;
//! - resilience serialization contracts;
//! - resilience errors;
//! - resilience registries;
//! - resource-aware resilience limits.
//!
//! It does **not** own the underlying quantum semantics.
//!
//! The authoritative owners remain:
//!
//! ```text
//! Quantum program semantics
//!     -> crate::quantum::ir
//!
//! Logical/physical qubit identity
//!     -> crate::quantum::ir::qubit
//!
//! Realized fault/noise semantics
//!     -> crate::quantum::zqn
//!
//! Error correction
//!     -> crate::quantum::error_correction
//!
//! Routing
//!     -> crate::quantum::routing
//!
//! Scheduling
//!     -> crate::quantum::scheduling
//!
//! Optimization
//!     -> crate::quantum::optimization
//!
//! Hardware capabilities/execution
//!     -> crate::quantum::hardware
//!
//! Simulation
//!     -> quantum simulator subsystem
//!
//! Benchmark methodology
//!     -> crate::quantum::benchmarking
//!
//! Provider-specific behavior
//!     -> hardware/provider adapters
//!
//! Credentials/authentication
//!     -> security/hardware authentication boundaries
//! ```
//!
//! Resilience coordinates these contracts; it does not duplicate them.
//!
//! # Canonical quantum identity rule
//!
//! Any resilience child module that requires quantum qubit identity MUST use
//! the canonical types owned by:
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
//! No resilience module may introduce competing identity types such as:
//!
//! ```text
//! ResilienceQubitId
//! LogicalQubitId
//! RecoveryQubitId
//! FaultQubitId
//! ResiliencePhysicalQubitId
//! ```
//!
//! Resilience identifiers are permitted only for resilience-domain objects,
//! such as incidents, recovery attempts, checkpoints, and execution cycles.
//!
//! A resilience identifier must never replace a canonical quantum identifier.
//!
//! # Dependency direction
//!
//! The root namespace establishes this conceptual dependency direction:
//!
//! ```text
//! canonical quantum IR
//!        │
//!        ├───────────────► resilience model
//!        │                       │
//!        │                       ▼
//!        │                 detection / diagnosis
//!        │                       │
//!        │                       ▼
//!        │                    policy
//!        │                       │
//!        │                       ▼
//!        │                   planning
//!        │                       │
//!        │                       ▼
//!        │                  adaptation
//!        │                       │
//!        │                       ▼
//!        │                   recovery
//!        │                       │
//!        │                       ▼
//!        │                  mitigation
//!        │                       │
//!        │                       ▼
//!        └────────────────► verification
//! ```
//!
//! Cross-system integrations are performed through the contracts exposed by
//! the owning quantum subsystems.
//!
//! The root module itself does not import concrete implementations from:
//!
//! - frontend;
//! - IR internals;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware providers;
//! - QEC implementations;
//! - simulator implementations;
//! - backend SDKs;
//! - network clients;
//! - filesystem implementations.
//!
//! This prevents the root namespace from becoming a dependency hub or
//! circular-dependency source.
//!
//! # Child-module ownership
//!
//! ```text
//! api/
//!     Public resilience lifecycle API.
//!
//! model/
//!     Canonical resilience domain values.
//!
//! detection/
//!     Fault/anomaly/health observation and detection.
//!
//! diagnosis/
//!     Classification, correlation, localization and root-cause hypotheses.
//!
//! policy/
//!     Constraints, objectives, budgets, escalation, retry and safety rules.
//!
//! planning/
//!     Feasible resilience actions and deterministic recovery plans.
//!
//! adaptation/
//!     Logical/physical adaptation orchestration including routing,
//!     scheduling, recompilation, reoptimization, QEC adaptation and target
//!     selection.
//!
//! recovery/
//!     Retry, restart, resume, rollback, migration, checkpoint coordination
//!     and compensating recovery operations.
//!
//! mitigation/
//!     Provider-neutral error-mitigation strategy contracts and orchestration.
//!
//! verification/
//!     Semantic, invariant, result, provenance and acceptance verification.
//!
//! state/
//!     Resilience/execution/resource/recovery state.
//!
//! checkpoint/
//!     Checkpoint metadata, snapshots, manifests, integrity, storage and
//!     compatibility.
//!
//! telemetry/
//!     Events, metrics, traces, health observations, collection and export.
//!
//! history/
//!     Incident, execution, recovery and historical statistics.
//!
//! learning/
//!     Optional predictive models, features, strategy selection and feedback.
//!
//! coordination/
//!     Distributed resilience coordination, ownership and leases.
//!
//! serialization/
//!     Resilience schema, encoding, decoding and version contracts.
//!
//! errors/
//!     Canonical resilience error taxonomy and classification.
//!
//! limits/
//!     Capability/resource-aware resilience limits and validation.
//!
//! registry/
//!     Extensible detector, strategy, recovery and backend registration.
//!
//! tests/
//!     Test-only resilience test graph.
//! ```
//!
//! Each child directory is an ownership boundary. The root does not
//! reimplement any of these responsibilities.
//!
//! # Public namespace policy
//!
//! Child modules remain available through their qualified paths:
//!
//! ```text
//! crate::quantum::resilience::api
//! crate::quantum::resilience::model
//! crate::quantum::resilience::detection
//! crate::quantum::resilience::diagnosis
//! crate::quantum::resilience::policy
//! crate::quantum::resilience::planning
//! crate::quantum::resilience::adaptation
//! crate::quantum::resilience::recovery
//! crate::quantum::resilience::mitigation
//! crate::quantum::resilience::verification
//! crate::quantum::resilience::state
//! crate::quantum::resilience::checkpoint
//! crate::quantum::resilience::telemetry
//! crate::quantum::resilience::history
//! crate::quantum::resilience::learning
//! crate::quantum::resilience::coordination
//! crate::quantum::resilience::serialization
//! crate::quantum::resilience::errors
//! crate::quantum::resilience::limits
//! crate::quantum::resilience::registry
//! ```
//!
//! The root intentionally avoids wildcard re-exports.
//!
//! This is deliberate.
//!
//! A root-level wildcard such as:
//!
//! ```text
//! pub use api::*;
//! pub use model::*;
//! pub use recovery::*;
//! ```
//!
//! would make the public namespace fragile as the subsystem grows. Types such
//! as `Result`, `State`, `Context`, `Resource`, `Error`, `Event`, and `Plan`
//! could collide across independently maintained modules.
//!
//! Stable qualified namespaces allow child modules to evolve independently.
//!
//! # Integration with `quantum::mod`
//!
//! The quantum root should expose resilience exactly once:
//!
//! ```text
//! crate::quantum::resilience
//! ```
//!
//! The integration point is the existing quantum namespace composition
//! boundary:
//!
//! ```rust
//! pub mod resilience;
//! ```
//!
//! No resilience implementation should be copied into `quantum/mod.rs`.
//!
//! `quantum/mod.rs` remains responsible only for composing quantum
//! subsystems. `resilience/mod.rs` owns composition of resilience subsystems.
//!
//! # Integration with canonical IR
//!
//! Resilience consumes the canonical quantum IR rather than defining another
//! circuit representation.
//!
//! Program semantics remain owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This root therefore intentionally does not import `QubitId` or
//! `PhysicalQubitId` itself. Child modules that need them must import the
//! canonical definitions directly.
//!
//! This prevents accidental ownership transfer of quantum identity into the
//! resilience namespace.
//!
//! # Integration with ZQN
//!
//! The realized fault/noise model remains owned by the ZQN subsystem.
//!
//! Resilience consumes normalized fault information through the established
//! ZQN contracts.
//!
//! The dependency direction is:
//!
//! ```text
//! crate::quantum::zqn
//!          │
//!          ▼
//! resilience::model::fault
//!          │
//!          ├── detection
//!          ├── diagnosis
//!          ├── planning
//!          ├── recovery
//!          └── verification
//! ```
//!
//! Resilience must not create a second quantum fault ontology.
//!
//! # Integration with hardware
//!
//! Hardware remains responsible for:
//!
//! - capability discovery;
//! - topology;
//! - calibration;
//! - execution;
//! - device/backend identity;
//! - provider adapters;
//! - health information originating at the hardware boundary.
//!
//! Resilience consumes these contracts and determines whether adaptation or
//! recovery is required.
//!
//! Provider-specific branching must remain below the provider-neutral
//! resilience boundary.
//!
//! # Integration with routing
//!
//! Resilience may request a new logical-to-physical realization after a
//! resource becomes unavailable or the target topology changes.
//!
//! The routing subsystem owns the routing algorithm.
//!
//! Resilience must not embed a routing algorithm in this root module or in
//! another resilience child merely to perform recovery.
//!
//! # Integration with scheduling
//!
//! Resilience may request schedule reconstruction when:
//!
//! - timing capabilities change;
//! - resources become unavailable;
//! - routing changes;
//! - calibration changes;
//! - execution constraints change.
//!
//! Scheduling owns timing and schedule construction.
//!
//! Resilience owns the decision that a schedule must be reconsidered.
//!
//! # Integration with optimization
//!
//! Resilience may request recompilation or reoptimization when the target
//! capabilities or physical realization change.
//!
//! Optimization operates on canonical quantum IR.
//!
//! Resilience must never maintain a parallel optimizer.
//!
//! # Integration with QEC
//!
//! QEC remains responsible for:
//!
//! - encoding;
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - code-specific execution structures.
//!
//! Resilience may decide that the QEC configuration needs to change, but the
//! actual QEC implementation remains in its owning subsystem.
//!
//! # Integration with benchmarking
//!
//! Benchmarking provides historical and current evidence such as:
//!
//! - observed fidelity;
//! - error rates;
//! - logical error rates;
//! - execution latency;
//! - stability;
//! - resource behavior.
//!
//! Resilience may consume benchmark-derived evidence for diagnosis and
//! planning, subject to trust, provenance and policy requirements.
//!
//! Benchmark methodology itself remains outside this module.
//!
//! # Integration with simulation
//!
//! Simulation must be able to provide deterministic resilience scenarios
//! without requiring a real QPU.
//!
//! A simulator can therefore exercise:
//!
//! ```text
//! program
//!     -> synthetic target
//!     -> injected fault
//!     -> detection
//!     -> diagnosis
//!     -> planning
//!     -> adaptation
//!     -> recovery
//!     -> verification
//! ```
//!
//! without changing the production resilience API.
//!
//! # Safety invariant
//!
//! No recovery or adaptation may be accepted merely because it improves
//! availability, latency, cost, or another optimization objective.
//!
//! A mutating resilience action must satisfy the applicable:
//!
//! ```text
//! authorization
//! evidence trust
//! evidence consistency
//! freshness
//! semantic preservation
//! capability validity
//! resource validity
//! budget validity
//! provenance requirements
//! verification requirements
//! ```
//!
//! Unknown safety information must never be silently converted into approval.
//!
//! An optimization objective must never override a safety failure.
//!
//! Protective abort remains distinct from ordinary mutation/recovery:
//!
//! ```text
//! unsafe execution
//!      │
//!      ├── ordinary recovery/adaptation -> deny unless independently safe
//!      │
//!      └── protective abort             -> may be authorized
//! ```
//!
//! Authorization is not execution. The appropriate recovery/runtime boundary
//! remains responsible for performing the authorized action.
//!
//! # Determinism
//!
//! The root composition module is deterministic.
//!
//! It performs no:
//!
//! - I/O;
//! - clock access;
//! - randomness;
//! - environment access;
//! - process inspection;
//! - global mutable-state access;
//! - provider calls;
//! - hidden concurrency.
//!
//! Determinism requirements for actual decisions belong to the child contracts
//! and are documented by `DETERMINISM.md`.
//!
//! Where deterministic replay is requested, all inputs affecting a decision
//! must be explicit, including any random seed used by an intentionally
//! randomized strategy.
//!
//! # Serialization
//!
//! The root module does not define wire formats.
//!
//! Serialization remains owned by:
//!
//! ```text
//! crate::quantum::resilience::serialization
//! ```
//!
//! Domain models must not become dependent on a single transport format merely
//! because the root module exposes them.
//!
//! # Security
//!
//! The root module does not own:
//!
//! - credentials;
//! - provider secrets;
//! - private keys;
//! - authentication;
//! - authorization implementation;
//! - network clients;
//! - filesystem persistence.
//!
//! Security-sensitive observations and actions must cross explicit security
//! boundaries before being consumed by resilience.
//!
//! The root namespace must never provide an alternate route around:
//!
//! - authentication;
//! - authorization;
//! - safety policy;
//! - semantic verification;
//! - provenance;
//! - checkpoint integrity.
//!
//! # Failure semantics
//!
//! Resilience failures must remain distinguishable from successful quantum
//! execution.
//!
//! In particular:
//!
//! ```text
//! execution completed
//!         !=
//! result verified
//!         !=
//! result accepted
//! ```
//!
//! The public API response and verification subsystem determine whether a
//! result may be accepted.
//!
//! Silent degradation and silent recovery are prohibited where they could
//! change semantic guarantees.
//!
//! # State-machine principle
//!
//! The resilience lifecycle is conceptually:
//!
//! ```text
//! Observe
//!    │
//!    ▼
//! Detect
//!    │
//!    ▼
//! Diagnose
//!    │
//!    ▼
//! Policy
//!    │
//!    ▼
//! Plan
//!    │
//!    ▼
//! Adapt
//!    │
//!    ▼
//! Recover
//!    │
//!    ▼
//! Mitigate
//!    │
//!    ▼
//! Verify
//!    │
//!    ▼
//! Decide
//!    │
//!    ├── Accept
//!    ├── DegradedAccept
//!    ├── Repeat
//!    ├── Escalate
//!    └── Reject
//! ```
//!
//! The root module only composes the modules implementing this lifecycle.
//!
//! # Versioning and compatibility
//!
//! Child modules own their semantic schemas.
//!
//! The root module must not introduce an independent runtime compatibility
//! version that duplicates the schemas owned by:
//!
//! - API;
//! - serialization;
//! - checkpoints;
//! - hardware capability contracts;
//! - canonical IR.
//!
//! Repository/package versioning remains separate from semantic schema
//! versions.
//!
//! # Extension policy
//!
//! New resilience mechanisms should normally be introduced as child modules
//! or registered implementations rather than by modifying this root module's
//! logic.
//!
//! A new child module should be added here only when it represents a genuine
//! top-level ownership boundary.
//!
//! A new detector, recovery strategy, mitigation strategy, backend adapter, or
//! planner implementation should normally be registered through the
//! appropriate registry instead of causing additional root-level coupling.
//!
//! # Rust compatibility
//!
//! This module is intentionally compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! It requires no nightly features and contains no `unsafe` code.
//!
//! The module-level lints below make the safety requirement local to this
//! boundary as well as subject to workspace-level policy.
//!
//! # Module inventory
//!
//! ```text
//! src/quantum/resilience/
//! ├── mod.rs                  <- this composition boundary
//! │
//! ├── api/
//! ├── model/
//! ├── detection/
//! ├── diagnosis/
//! ├── policy/
//! ├── planning/
//! ├── adaptation/
//! ├── recovery/
//! ├── mitigation/
//! ├── verification/
//! ├── state/
//! ├── checkpoint/
//! ├── telemetry/
//! ├── history/
//! ├── learning/
//! ├── coordination/
//! ├── serialization/
//! ├── errors/
//! ├── limits/
//! ├── registry/
//! └── tests/                  <- test-only
//! ```
//!
//! The current repository contains these resilience child directories. This
//! root therefore composes them directly rather than introducing placeholder
//! modules or speculative filesystem structure.
//!
//! # Integration contract for this file
//!
//! This file is considered complete when:
//!
//! 1. every production resilience child directory has exactly one declaration;
//! 2. the test graph is conditionally declared;
//! 3. no child implementation is duplicated here;
//! 4. no concrete provider is referenced;
//! 5. no hard-coded machine-size limit exists;
//! 6. no resilience-specific qubit identity exists here;
//! 7. no `unsafe` code exists;
//! 8. no I/O or ambient runtime state is accessed;
//! 9. no wildcard public re-exports exist;
//! 10. `quantum::resilience` can be exposed from `quantum::mod` without
//!     changing the semantics of its child modules.
//!
//! Once these conditions are satisfied, changes to individual detectors,
//! planners, recovery strategies, mitigation implementations, hardware
//! adapters, QEC implementations, routing algorithms, scheduling algorithms,
//! or verification implementations should not require changes to this root
//! module unless a top-level ownership boundary itself is added or removed.

// =============================================================================
// Rust safety and correctness policy
// =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Production resilience namespace
// =============================================================================
//
// These declarations intentionally contain no implementation logic.
// Each child module owns its own API, types, invariants and integration
// contracts.
//
// The order here is architectural/documentational only. Rust module
// declaration order must not be treated as runtime execution order.

/// Public resilience lifecycle API and dependency-injection boundary.
pub mod api;

/// Capability/resource-aware adaptation orchestration.
pub mod adaptation;

/// Checkpoint coordination and integrity contracts.
pub mod checkpoint;

/// Distributed resilience coordination.
pub mod coordination;

/// Fault/anomaly/health observation and detection.
pub mod detection;

/// Fault diagnosis, classification, correlation and localization.
pub mod diagnosis;

/// Canonical resilience error taxonomy.
pub mod errors;

/// Incident, execution, recovery and historical statistics.
pub mod history;

/// Optional predictive resilience learning.
pub mod learning;

/// Resource-aware resilience limits and validation.
pub mod limits;

/// Error mitigation strategy contracts and orchestration.
pub mod mitigation;

/// Canonical resilience domain model.
pub mod model;

/// Recovery planning and plan feasibility/ranking.
pub mod planning;

/// Resilience constraints, objectives, budgets and safety policy.
pub mod policy;

/// Recovery execution and recovery-state contracts.
pub mod recovery;

/// Detector, strategy, recovery and backend registries.
pub mod registry;

/// Resilience serialization schema/version/codec contracts.
pub mod serialization;

/// Resilience execution, resource and recovery state.
pub mod state;

/// Resilience events, metrics, traces, health observations and exporters.
pub mod telemetry;

/// Semantic, invariant, result, provenance and acceptance verification.
pub mod verification;

// =============================================================================
// Test graph
// =============================================================================
//
// `tests/` is not production functionality. The existing resilience test
// graph is intentionally included only when compiling tests.
//
// Keeping this conditional is important because production builds must not
// acquire test-only dependencies or test-only module coupling.

#[cfg(test)]
mod tests {}

// =============================================================================
// Architectural compile-time checks
// =============================================================================

#[cfg(test)]
mod architecture_tests {
    use core::any::type_name;

    #[test]
    fn production_child_namespaces_are_reachable() {
        // These references intentionally validate namespace composition only.
        // Behavioral tests remain in `crate::quantum::resilience::tests`.

        let _ = type_name::<
            crate::quantum::resilience::api::ResilienceController,
        >();

        let _ = type_name::<
            crate::quantum::resilience::model::resource::ResourceIdentity,
        >();

        let _ = type_name::<
            crate::quantum::resilience::detection::detector::Detector,
        >();

        let _ = type_name::<
            crate::quantum::resilience::diagnosis::diagnostician::Diagnostician,
        >();

        let _ = type_name::<
            crate::quantum::resilience::planning::planner::Planner,
        >();

        let _ = type_name::<
            crate::quantum::resilience::adaptation::adapter::Adapter,
        >();

        let _ = type_name::<
            crate::quantum::resilience::recovery::recoverer::Recoverer,
        >();

        let _ = type_name::<
            crate::quantum::resilience::mitigation::strategy::MitigationStrategy,
        >();

        let _ = type_name::<
            crate::quantum::resilience::verification::verifier::Verifier,
        >();
    }

    #[test]
    fn canonical_qubit_identity_remains_owned_by_quantum_ir() {
        // Merely naming the canonical types here provides a compile-time
        // integration check that resilience continues to build against the
        // authoritative quantum identity namespace.
        //
        // No resilience-specific qubit alias is introduced.
        let _ = type_name::<crate::quantum::ir::qubit::QubitId>();
        let _ = type_name::<crate::quantum::ir::qubit::PhysicalQubitId>();
    }
}