//! Zamani Quantum Resilience — Diagnosis namespace and integration boundary.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/mod.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module is the composition boundary for the diagnosis subsystem of
//! `quantum::resilience`.
//!
//! Diagnosis answers:
//!
//!     "What does the available evidence most strongly indicate?"
//!
//! Diagnosis does NOT:
//!
//! - execute recovery;
//! - authorize recovery;
//! - select a backend;
//! - change routing;
//! - change scheduling;
//! - recompile a program;
//! - mutate hardware;
//! - modify QEC state;
//! - redefine ZQN fault semantics;
//! - establish semantic result correctness.
//!
//! Those responsibilities belong to downstream resilience and quantum
//! subsystems.
//!
//! The existing diagnosis architecture deliberately separates:
//!
//!     classifier
//!     correlation
//!     localization
//!     root-cause analysis
//!     confidence analysis
//!     diagnosis orchestration
//!
//! This parent module only composes those contracts.
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//! ```text
//!                         Zamani Program
//!                               |
//!                               v
//!                        quantum::ir
//!                               |
//!                canonical quantum semantics
//!                               |
//!              +----------------+----------------+
//!              |                                 |
//!              v                                 v
//!       quantum::zqn                    quantum::hardware
//!       fault/noise semantics           capability/state
//!              |                                 |
//!              +----------------+----------------+
//!                               |
//!                               v
//!                           detection
//!                               |
//!                               v
//!                    DetectionSignal / Output
//!                               |
//!                               v
//!                   +-------------------------+
//!                   |       diagnosis         |
//!                   |                         |
//!                   | classifier              |
//!                   | correlation             |
//!                   | localization            |
//!                   | root_cause              |
//!                   | confidence              |
//!                   | diagnostician            |
//!                   +------------+------------+
//!                                |
//!                                v
//!                            Diagnosis
//!                                |
//!                                v
//!                             policy
//!                                |
//!                                v
//!                            planning
//!                                |
//!                                v
//!                           adaptation
//!                                |
//!                                v
//!                            recovery
//!                                |
//!                                v
//!                           mitigation
//!                                |
//!                                v
//!                          verification
//! ```
//!
//! The diagnosis namespace is therefore downstream of evidence collection and
//! upstream of policy and recovery decisions.
//!
//! =============================================================================
//! Module ownership
//! =============================================================================
//!
//! `classifier.rs`
//! ----------------
//! Converts normalized detection classifications into provider-neutral
//! diagnosis findings.
//!
//! It answers:
//!
//!     "What semantic category does this detection signal represent?"
//!
//! It does not establish causality or execute recovery.
//!
//! `correlation.rs`
//! ----------------
//! Correlates multiple observations or findings into higher-level relationships.
//!
//! Correlation is especially important for large systems where many apparently
//! independent observations can originate from one underlying incident.
//!
//! `localization.rs`
//! -----------------
//! Determines the resource or execution scope associated with evidence when
//! such information is available.
//!
//! If quantum resource identity is required, this module must use the canonical
//! identity types from:
//!
//!     crate::quantum::ir::qubit
//!
//! It must never introduce a resilience-specific replacement for `QubitId` or
//! `PhysicalQubitId`.
//!
//! `root_cause.rs`
//! ---------------
//! Represents causal hypotheses and their supporting evidence.
//!
//! A hypothesis must remain distinguishable from proven causality.
//!
//! `confidence.rs`
//! ----------------
//! Evaluates or combines evidentiary confidence according to explicit inputs.
//!
//! Confidence must not silently become:
//!
//!     probability of failure
//!     fidelity
//!     severity
//!     priority
//!     retry count
//!
//! Those are different semantic quantities.
//!
//! `diagnostician.rs`
//! ------------------
//! Defines the stable composition/orchestration boundary and immutable
//! diagnosis result contracts.
//!
//! The existing diagnostician contract intentionally does not depend on
//! concrete classifier, correlation, localization, root-cause, or confidence
//! implementations. That design is preserved here.
//!
//! =============================================================================
//! Dependency direction
//! =============================================================================
//!
//! The intended dependency graph is:
//!
//! ```text
//! quantum::ir::qubit
//!        |
//!        v
//! resilience::model
//!        |
//!        +------------------+
//!        |                  |
//!        v                  v
//!    detection          ZQN-derived evidence
//!        |                  |
//!        +---------+--------+
//!                  |
//!                  v
//!              diagnosis
//!                  |
//!                  v
//!               policy
//!                  |
//!                  v
//!              planning
//!                  |
//!                  v
//!        adaptation / recovery
//!                  |
//!                  v
//!             verification
//! ```
//!
//! More specifically:
//!
//!     detection
//!         -> diagnosis
//!         -> policy
//!         -> planning
//!         -> adaptation
//!         -> recovery
//!         -> verification
//!
//! Diagnosis must not depend on concrete implementations from those
//! downstream decision layers.
//!
//! This prevents circular dependencies and allows diagnosis contributors to be
//! replaced independently.
//!
//! =============================================================================
//! Canonical quantum identity rule
//! =============================================================================
//!
//! This module deliberately defines no quantum resource identity.
//!
//! If any child diagnosis implementation requires quantum qubit identity, the
//! canonical types are:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No diagnosis module may introduce:
//!
//!     DiagnosisQubitId
//!     ResilienceQubitId
//!     LogicalQubitId
//!     DiagnosisPhysicalQubitId
//!
//! or an equivalent duplicate identity abstraction.
//!
//! This preserves one canonical identity model across:
//!
//!     IR
//!     routing
//!     scheduling
//!     hardware
//!     QEC
//!     resilience
//!
//! and is essential for write-once/scale-everywhere execution.
//!
//! =============================================================================
//! Canonical fault semantics
//! =============================================================================
//!
//! ZQN remains authoritative for quantum fault and noise semantics.
//!
//! Diagnosis is an interpretation layer over normalized evidence. It must not
//! create a second physical fault ontology merely because a diagnosis module
//! needs a convenient representation.
//!
//! The intended direction is:
//!
//!     quantum::zqn
//!          |
//!          v
//!     detection
//!          |
//!          v
//!     diagnosis
//!          |
//!          v
//!     planning / recovery
//!
//! A diagnosis category is therefore not a replacement for a canonical ZQN
//! fault.
//!
//! =============================================================================
//! Write once, scale everywhere
//! =============================================================================
//!
//! This module imposes no artificial machine-size limit.
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_RESOURCES
//!     MAX_BACKENDS
//!     MAX_INCIDENTS
//!     MAX_FINDINGS
//!     MAX_DETECTORS
//!     MAX_DIAGNOSES
//!
//! and no fixed hardware topology.
//!
//! "Infinity" is interpreted architecturally:
//!
//! > The diagnosis namespace does not impose a finite machine-size ceiling.
//!
//! Concrete executions are naturally bounded by:
//!
//! - available memory;
//! - CPU/GPU resources;
//! - runtime limits;
//! - explicitly configured policy budgets;
//! - hardware capabilities;
//! - network capacity;
//! - storage;
//! - physical execution limits.
//!
//! Those limits must be supplied by their owning subsystems rather than being
//! embedded in this module.
//!
//! Diagnosis must therefore remain valid for:
//!
//!     one qubit
//!     small QPU
//!     large QPU
//!     logical fault-tolerant machine
//!     heterogeneous quantum system
//!     distributed quantum execution fabric
//!
//! The number of resources must never be encoded in a type or compile-time
//! array in this namespace.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! The parent module performs no computation with ambient state.
//!
//! It does not access:
//!
//! - system time;
//! - environment variables;
//! - filesystem;
//! - network;
//! - process IDs;
//! - thread IDs;
//! - memory addresses;
//! - random generators;
//! - global mutable state.
//!
//! Determinism is inherited from the child contracts and the caller-supplied
//! evidence ordering/configuration.
//!
//! If deterministic diagnosis is requested, the complete input evidence,
//! contributor configuration, contributor ordering, and any explicit random
//! seed must be part of the execution contract.
//!
//! =============================================================================
//! Immutability
//! =============================================================================
//!
//! A completed diagnosis is an interpretation of a particular evidence
//! snapshot.
//!
//! Downstream systems may retain it for:
//!
//!     policy
//!     planning
//!     recovery
//!     verification
//!     telemetry
//!     history
//!     audit
//!     deterministic replay
//!
//! Therefore a diagnosis result must not depend on mutable ambient state after
//! completion.
//!
//! The parent namespace does not enforce the concrete ownership representation;
//! that belongs to `diagnostician.rs` and its contributor contracts.
//!
//! =============================================================================
//! Security boundary
//! =============================================================================
//!
//! Diagnosis evidence is not automatically trustworthy merely because it is
//! represented by a type-safe Rust value.
//!
//! Evidence may require:
//!
//!     source authentication
//!     integrity verification
//!     freshness validation
//!     provenance
//!     trust evaluation
//!     authorization
//!
//! Those concerns belong to the appropriate telemetry, runtime, hardware, and
//! security boundaries.
//!
//! A diagnosis result must never itself grant:
//!
//!     hardware access
//!     backend migration permission
//!     recovery permission
//!     credential access
//!     policy bypass
//!     verification bypass
//!
//! Security-related evidence may be classified by diagnosis, but enforcement
//! remains outside this namespace.
//!
//! =============================================================================
//! Failure semantics
//! =============================================================================
//!
//! Diagnosis must be able to represent uncertainty.
//!
//! In particular:
//!
//!     unknown
//!     inconclusive
//!     insufficient evidence
//!     conflicting evidence
//!
//! must not automatically become a stronger diagnosis merely because a
//! downstream recovery action would be convenient.
//!
//! An inconclusive diagnosis is a valid semantic outcome when the evidence is
//! insufficient.
//!
//! Errors caused by invalid configuration or invalid API input use the
//! canonical resilience error contract owned by:
//!
//!     crate::quantum::resilience::errors
//!
//! This parent module does not redefine those errors.
//!
//! =============================================================================
//! Integration with detection
//! =============================================================================
//!
//! Detection is upstream of diagnosis.
//!
//! The expected contract is conceptually:
//!
//!     detection
//!         |
//!         | DetectionSignal / DetectionOutput
//!         v
//!     diagnosis::classifier
//!         |
//!         v
//!     diagnosis::diagnostician
//!
//! Diagnosis must not poll detectors, read hardware, or perform hidden I/O.
//!
//! Detection remains responsible for observing conditions.
//!
//! Diagnosis remains responsible for interpreting those observations.
//!
//! =============================================================================
//! Integration with model
//! =============================================================================
//!
//! Diagnosis may consume resilience model values from:
//!
//!     crate::quantum::resilience::model
//!
//! including resource, fault, health, degradation, incident, severity, and
//! confidence contracts where appropriate.
//!
//! The model remains below decision-making.
//!
//! Diagnosis must not duplicate those semantic types simply to shorten imports.
//!
//! =============================================================================
//! Integration with hardware
//! =============================================================================
//!
//! Hardware integration is indirect.
//!
//! Diagnosis may consume normalized hardware observations supplied by detection
//! and model contracts.
//!
//! It must not:
//!
//! - call provider SDKs;
//! - query devices directly;
//! - inspect provider-specific state;
//! - hard-code device topology;
//! - hard-code physical qubit counts;
//! - perform calibration;
//! - modify hardware.
//!
//! Hardware remains owned by:
//!
//!     crate::quantum::hardware
//!
//! =============================================================================
//! Integration with QEC
//! =============================================================================
//!
//! QEC-related evidence may arrive through detection.
//!
//! Diagnosis can classify or correlate:
//!
//!     syndrome anomalies
//!     decoder warnings
//!     logical-error indicators
//!     leakage indicators
//!     erasure indicators
//!
//! but diagnosis must not become a QEC decoder.
//!
//! QEC implementation remains owned by the QEC subsystem.
//!
//! =============================================================================
//! Integration with routing and scheduling
//! =============================================================================
//!
//! Diagnosis does not perform routing or scheduling.
//!
//! It may identify:
//!
//!     routing-related condition
//!     scheduling-related condition
//!     resource-localized degradation
//!
//! Downstream adaptation is responsible for requesting:
//!
//!     rerouting
//!     remapping
//!     rescheduling
//!
//! from the owning quantum subsystems.
//!
//! This prevents diagnosis from acquiring duplicate routing/topology logic.
//!
//! =============================================================================
//! Integration with optimization and compilation
//! =============================================================================
//!
//! Diagnosis may identify conditions that justify downstream recompilation or
//! reoptimization, but it must not invoke compiler or optimizer implementations
//! directly.
//!
//! The intended direction is:
//!
//!     diagnosis
//!         |
//!         v
//!     policy / planning
//!         |
//!         v
//!     adaptation
//!         |
//!         +--> compiler
//!         +--> optimization
//!
//! =============================================================================
//! Integration with policy
//! =============================================================================
//!
//! Diagnosis produces evidence and interpretation.
//!
//! Policy decides what is permitted.
//!
//! Therefore diagnosis must never encode:
//!
//!     retry now
//!     migrate now
//!     abort now
//!     increase QEC
//!     change backend
//!
//! Those are policy/planning decisions.
//!
//! =============================================================================
//! Integration with planning
//! =============================================================================
//!
//! Planning consumes diagnosis results together with:
//!
//!     constraints
//!     objectives
//!     budgets
//!     capabilities
//!     current state
//!     policy
//!
//! to construct recovery/adaptation plans.
//!
//! Diagnosis must not rank or execute recovery plans.
//!
//! =============================================================================
//! Integration with recovery
//! =============================================================================
//!
//! Recovery consumes plans produced downstream of diagnosis.
//!
//! Diagnosis must not:
//!
//!     retry execution
//!     restart jobs
//!     rollback checkpoints
//!     migrate execution
//!     resume execution
//!
//! This keeps diagnosis side-effect free.
//!
//! =============================================================================
//! Integration with verification
//! =============================================================================
//!
//! Diagnosis is not result verification.
//!
//! A diagnosis can indicate a likely execution problem, but it cannot by itself
//! establish whether the final quantum result is semantically correct.
//!
//! Verification remains authoritative for acceptance.
//!
//! =============================================================================
//! Integration with telemetry and history
//! =============================================================================
//!
//! Telemetry may feed detection and may record diagnosis outcomes.
//!
//! History may retain diagnosis results for:
//!
//!     replay
//!     statistics
//!     future planning
//!     reliability analysis
//!
//! Diagnosis itself must not require a telemetry transport or persistent
//! database.
//!
//! =============================================================================
//! Extensibility
//! =============================================================================
//!
//! New diagnosis contributors should be added as independent child modules.
//!
//! For example:
//!
//!     future_predictive.rs
//!     temporal.rs
//!     causal.rs
//!     topology.rs
//!
//! A future contributor should:
//!
//! 1. define a distinct ownership boundary;
//! 2. consume existing canonical contracts;
//! 3. avoid duplicate quantum identities;
//! 4. avoid provider-specific assumptions;
//! 5. expose a stable contributor contract;
//! 6. integrate through the diagnostician contract;
//! 7. avoid modifying unrelated diagnosis modules.
//!
//! Adding a contributor must not require rewriting this module's architectural
//! principles.
//!
//! =============================================================================
//! Public namespace policy
//! =============================================================================
//!
//! The child modules are exposed through stable module-qualified paths:
//!
//!     quantum::resilience::diagnosis::classifier
//!     quantum::resilience::diagnosis::correlation
//!     quantum::resilience::diagnosis::confidence
//!     quantum::resilience::diagnosis::diagnostician
//!     quantum::resilience::diagnosis::localization
//!     quantum::resilience::diagnosis::root_cause
//!
//! Wildcard re-exports are intentionally avoided.
//!
//! This prevents accidental API collisions with common names such as:
//!
//!     Result
//!     State
//!     Context
//!     Confidence
//!     Resource
//!     Event
//!     Error
//!
//! It also avoids creating unnecessary long-term compatibility obligations for
//! implementation details.
//!
//! The child modules remain independently usable through their own stable
//! namespaces.
//!
//! =============================================================================
//! Integration contract for each existing file
//! =============================================================================
//!
//! `classifier.rs`
//!
//!     Input:
//!         detection::detector contracts
//!
//!     Output:
//!         diagnosis findings
//!
//!     Must not depend on:
//!         recovery, policy, routing, scheduling, hardware SDKs
//!
//! `correlation.rs`
//!
//!     Input:
//!         normalized evidence/findings
//!
//!     Output:
//!         correlated interpretation
//!
//!     Must not execute actions.
//!
//! `localization.rs`
//!
//!     Input:
//!         evidence and canonical resource information
//!
//!     Output:
//!         localized diagnosis scope
//!
//!     Quantum identity:
//!         quantum::ir::qubit
//!
//! `root_cause.rs`
//!
//!     Input:
//!         correlated/localized evidence
//!
//!     Output:
//!         causal hypotheses
//!
//!     Must preserve uncertainty.
//!
//! `confidence.rs`
//!
//!     Input:
//!         evidentiary claims
//!
//!     Output:
//!         explicit confidence analysis
//!
//!     Must not introduce hidden thresholds.
//!
//! `diagnostician.rs`
//!
//!     Input:
//!         detection evidence and contributor contracts
//!
//!     Output:
//!         immutable diagnosis
//!
//!     Must remain the composition/orchestration boundary.
//!
//! =============================================================================
//! Production invariants
//! =============================================================================
//!
//! This namespace must remain:
//!
//!     provider-neutral
//!     backend-neutral
//!     hardware-size-neutral
//!     topology-neutral
//!     qubit-count-neutral
//!     deterministic when configured deterministically
//!     side-effect free at the namespace level
//!     free of unsafe Rust
//!     free of hidden I/O
//!     free of hidden randomness
//!     free of global mutable state
//!
//! The diagnosis namespace must never become an implicit execution engine.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! Target:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021 edition
//!     stable Rust
//!     no nightly features
//!     no unsafe code
//!
//! The explicit lint policy below prevents accidental introduction of unsafe
//! operations or common production-quality hazards into this module.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

// =============================================================================
// Diagnosis contributor modules
// =============================================================================

/// Converts normalized detection classifications into provider-neutral
/// diagnosis findings.
///
/// Classification is semantic interpretation only. It does not establish
/// causality and does not execute recovery.
pub mod classifier;

/// Correlates multiple observations or diagnosis findings into higher-level
/// relationships.
///
/// Correlation is especially important for large and distributed quantum
/// systems where many observations may represent one underlying incident.
pub mod correlation;

/// Evaluates and combines evidentiary confidence according to explicit
/// contracts.
///
/// Confidence analysis does not authorize recovery and does not replace
/// probability, fidelity, severity, or policy.
pub mod confidence;

/// Stable diagnosis composition/orchestration contract.
///
/// This module owns the principal diagnosis request, contributor, finding,
/// diagnosis, and identity contracts used by the surrounding resilience
/// system.
pub mod diagnostician;

/// Determines the resource or execution scope associated with diagnosis
/// evidence.
///
/// Quantum identities remain canonical in `crate::quantum::ir::qubit`.
pub mod localization;

/// Represents causal hypotheses and supporting evidence.
///
/// A hypothesis must not be presented as proven causality unless its evidence
/// contract establishes that level of certainty.
pub mod root_cause;

// =============================================================================
// Namespace boundary invariants
// =============================================================================
//
// There are intentionally no wildcard re-exports here.
//
// Consumers should use:
//
//     quantum::resilience::diagnosis::classifier::...
//     quantum::resilience::diagnosis::correlation::...
//     quantum::resilience::diagnosis::confidence::...
//     quantum::resilience::diagnosis::diagnostician::...
//     quantum::resilience::diagnosis::localization::...
//     quantum::resilience::diagnosis::root_cause::...
//
// This keeps the namespace stable while allowing individual contributors to
// evolve independently.
//
// The parent module contains no runtime initialization, no global registry,
// no I/O, no thread creation, no hardware access, no provider SDK access, and
// no recovery side effects.