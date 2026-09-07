//! Zamani Quantum Resilience — Verification Subsystem
//!
//! Path:
//!     crate::quantum::resilience::verification
//!
//! # Purpose
//!
//! This module is the composition boundary for production-grade verification
//! of resilient quantum executions.
//!
//! Verification is the final correctness boundary between:
//!
//! ```text
//! candidate execution
//!        |
//!        v
//! resilience adaptation / recovery / mitigation
//!        |
//!        v
//! verification
//!        |
//!        +-----------------------------+
//!        |                             |
//!        v                             v
//!     ACCEPT                    NOT ACCEPTED
//! ```
//!
//! The verification subsystem answers:
//!
//! > Is there sufficient, internally consistent, policy-compliant evidence
//! > that this execution may be accepted as the result of the requested
//! > Zamani quantum computation?
//!
//! # Architectural ownership
//!
//! This module owns only the composition boundary.
//!
//! It does NOT implement:
//!
//! - canonical quantum IR;
//! - quantum gates;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - noise modelling;
//! - hardware discovery;
//! - backend execution;
//! - mitigation algorithms;
//! - recovery algorithms;
//! - result generation.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! The existing verification components are intentionally separated:
//!
//! ```text
//! verification/
//! ├── mod.rs
//! ├── invariant.rs
//! ├── semantic.rs
//! ├── result.rs
//! ├── confidence.rs
//! ├── provenance.rs
//! ├── acceptance.rs
//! └── verifier.rs
//! ```
//!
//! # Verification architecture
//!
//! ```text
//!                    Canonical Zamani Quantum IR
//!                              |
//!                              v
//!                    Resilient execution attempt
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!      adaptation          recovery           mitigation
//!          |                   |                   |
//!          +-------------------+-------------------+
//!                              |
//!                              v
//!                    Verification request
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!        invariants        semantic           result
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                 +------------+------------+
//!                 |            |            |
//!                 v            v            v
//!            provenance    confidence    recovery
//!                 |            |            |
//!                 +------------+------------+
//!                              |
//!                              v
//!                     aggregate verifier
//!                              |
//!                              v
//!                    acceptance evaluator
//!                              |
//!                  +-----------+-----------+
//!                  |                       |
//!                  v                       v
//!                ACCEPT              NOT ACCEPTED
//! ```
//!
//! # Critical safety rule
//!
//! Verification is authoritative for acceptance.
//!
//! No resilience component may convert an execution into an accepted result
//! merely because:
//!
//! - execution completed;
//! - a backend returned success;
//! - a recovery action completed;
//! - mitigation completed;
//! - a decoder returned a value;
//! - statistical confidence is high;
//! - the result appears plausible;
//! - the execution occurred on a trusted machine.
//!
//! Acceptance requires the mandatory verification dimensions to pass according
//! to the active verification and acceptance policies.
//!
//! # Canonical quantum identity
//!
//! Verification must never introduce a second quantum identity system.
//!
//! Logical qubits are represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubits are represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The canonical identities are owned by `quantum::ir`.
//!
//! This module therefore does not define:
//!
//! ```text
//! ResilienceQubitId
//! ResiliencePhysicalQubitId
//! ```
//!
//! or equivalent aliases that could fragment identity semantics.
//!
//! # Write once, scale everywhere
//!
//! This module introduces no artificial machine-size limit.
//!
//! In particular, it contains no fixed:
//!
//! - qubit count;
//! - logical-qubit count;
//! - physical-qubit count;
//! - operation count;
//! - circuit depth;
//! - backend count;
//! - machine count;
//! - verification-result count;
//! - recovery-cycle count;
//! - shot count;
//! - fidelity threshold;
//! - retry count.
//!
//! Concrete limits belong to explicit execution, resource, security, and
//! resilience policies.
//!
//! Consequently, the verification API can be used for:
//!
//! ```text
//! one logical qubit
//!       |
//!       v
//! small QPU
//!       |
//!       v
//! large QPU
//!       |
//!       v
//! fault-tolerant logical machine
//!       |
//!       v
//! heterogeneous quantum fleet
//!       |
//!       v
//! distributed quantum system
//! ```
//!
//! "Infinity" in the Zamani architecture means:
//!
//! > No finite machine-size ceiling is encoded by this module.
//!
//! Every actual execution remains bounded by the resources available to that
//! execution environment.
//!
//! # Determinism
//!
//! The module declaration and public composition boundary contain no mutable
//! global state and perform no runtime work.
//!
//! The underlying verification implementation is expected to be deterministic
//! for identical:
//!
//! - canonical input;
//! - adapted input;
//! - execution evidence;
//! - hardware/resource snapshot;
//! - verification policy;
//! - acceptance policy;
//! - supplied randomness evidence.
//!
//! Verification implementations must not rely on:
//!
//! - wall-clock time for correctness;
//! - environment variables for correctness;
//! - memory addresses;
//! - process-global mutable state;
//! - unspecified hash-map iteration order;
//! - provider-specific hidden state.
//!
//! If probabilistic verification is required, the relevant seed/evidence must
//! be an explicit part of the verification contract.
//!
//! # Fail-closed architecture
//!
//! Production verification is fail-closed.
//!
//! Unknown or incomplete evidence must not silently become successful evidence.
//!
//! Examples:
//!
//! ```text
//! unknown semantic equivalence  -> not accepted
//! unknown provenance            -> not accepted when required
//! unknown resource identity     -> not accepted when required
//! contradictory evidence        -> not accepted
//! failed invariant              -> not accepted
//! failed result validation      -> not accepted
//! insufficient confidence       -> not accepted
//! ```
//!
//! An explicitly configured analysis/diagnostic mode may expose incomplete
//! evidence, but that result must remain distinguishable from a production
//! acceptance result.
//!
//! # Verification layers
//!
//! ## `invariant`
//!
//! Structural/resource identity invariants.
//!
//! It verifies properties such as:
//!
//! - logical-resource preservation;
//! - physical-resource preservation;
//! - mapping consistency;
//! - uniqueness;
//! - injectivity;
//! - required resources;
//! - forbidden resources;
//! - internally coherent verification input.
//!
//! It does not prove quantum-state equivalence.
//!
//! ## `semantic`
//!
//! Semantic equivalence between the requested computation and the adapted
//! computation.
//!
//! It must reason from canonical quantum semantics rather than introducing a
//! competing quantum representation.
//!
//! It is the appropriate layer for checking whether routing, recompilation,
//! reoptimization, mitigation, or recovery changed the intended computation.
//!
//! ## `result`
//!
//! Execution-result validation.
//!
//! It verifies that the returned execution evidence satisfies the result
//! contract before acceptance is considered.
//!
//! It must not confuse:
//!
//! ```text
//! result exists
//! ```
//!
//! with:
//!
//! ```text
//! result is correct
//! ```
//!
//! ## `confidence`
//!
//! Evidence-confidence assessment.
//!
//! Confidence is evidence about verification; it is not a replacement for
//! semantic verification.
//!
//! High confidence cannot make a failed semantic invariant pass.
//!
//! ## `provenance`
//!
//! Verification provenance and integrity.
//!
//! Provenance should be capable of connecting, where supplied by the caller:
//!
//! ```text
//! source program
//!     |
//!     v
//! canonical IR
//!     |
//!     v
//! optimization
//!     |
//!     v
//! routing
//!     |
//!     v
//! scheduling
//!     |
//!     v
//! execution
//!     |
//!     v
//! fault observations
//!     |
//!     v
//! adaptation / recovery
//!     |
//!     v
//! mitigation / QEC
//!     |
//!     v
//! result
//!     |
//!     v
//! verification
//! ```
//!
//! Provenance must remain explicit rather than reconstructed from human-readable
//! log messages.
//!
//! ## `acceptance`
//!
//! Final acceptance semantics.
//!
//! It converts complete verification evidence into a controlled decision such
//! as:
//!
//! ```text
//! ACCEPT
//! REPEAT
//! ESCALATE
//! REJECT
//! ```
//!
//! It does not perform recovery itself.
//!
//! ## `verifier`
//!
//! Aggregate verification orchestration.
//!
//! It coordinates the verification dimensions and produces the aggregate
//! verification result.
//!
//! It is the primary verification entry point for resilience orchestration.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      |
//!      +----------------------------+
//!      |                            |
//!      v                            v
//! canonical program             canonical identity
//!      |                            |
//!      +--------------+-------------+
//!                     |
//!                     v
//!             resilience verification
//!                     |
//!        +------------+------------+
//!        |            |            |
//!        v            v            v
//!    invariant     semantic     result
//!        |            |            |
//!        +------------+------------+
//!                     |
//!             +-------+-------+
//!             |       |       |
//!             v       v       v
//!        confidence provenance acceptance
//!             |       |       |
//!             +-------+-------+
//!                     |
//!                     v
//!                  verifier
//! ```
//!
//! The child modules must not depend on `mod.rs` through implementation
//! details. `mod.rs` is only the namespace/composition boundary.
//!
//! # Integration with resilience
//!
//! The resilience controller should depend on the aggregate verifier rather
//! than directly constructing an acceptance result.
//!
//! Conceptually:
//!
//! ```text
//! resilience::api
//!        |
//!        v
//! resilience::verification::verifier
//!        |
//!        +--> invariant
//!        +--> semantic
//!        +--> result
//!        +--> confidence
//!        +--> provenance
//!        +--> acceptance
//!        |
//!        v
//! VerificationReport
//!        |
//!        v
//! AcceptanceOutcome
//! ```
//!
//! Recovery and adaptation components may consume verification reports to
//! decide whether another recovery cycle is necessary.
//!
//! They must not bypass the acceptance boundary.
//!
//! # Integration with canonical IR
//!
//! Verification consumes canonical quantum IR contracts.
//!
//! The canonical IR remains authoritative for:
//!
//! - quantum operation identity;
//! - gate identity;
//! - logical qubit identity;
//! - physical qubit identity;
//! - program structure;
//! - canonical semantics.
//!
//! Verification must not define a parallel circuit model merely for convenience.
//!
//! If an adapter is needed, it belongs at the adapter boundary of the subsystem
//! that owns the foreign representation.
//!
//! # Integration with routing
//!
//! Routing may change the physical realization of a logical computation.
//!
//! Verification therefore checks the resulting resource/mapping evidence but
//! does not implement routing.
//!
//! The expected relationship is:
//!
//! ```text
//! quantum::routing
//!       |
//!       v
//! routed candidate
//!       |
//!       v
//! resilience::verification::invariant
//!       |
//!       v
//! resilience::verification::semantic
//! ```
//!
//! `QubitId` and `PhysicalQubitId` remain canonical IR identities.
//!
//! # Integration with scheduling
//!
//! Scheduling may change temporal placement without changing computation
//! semantics.
//!
//! Verification must therefore distinguish:
//!
//! ```text
//! semantic quantum operation
//! ```
//!
//! from:
//!
//! ```text
//! scheduling-only timing artifact
//! ```
//!
//! Scheduling verification remains owned by `quantum::scheduling`.
//!
//! Resilience verification consumes its validated scheduling evidence rather
//! than reproducing scheduling verification algorithms.
//!
//! # Integration with optimization
//!
//! Optimization may change the representation of a computation while
//! preserving semantics.
//!
//! Verification is therefore the final independent boundary for establishing
//! that an optimization/adaptation result is acceptable.
//!
//! The relationship is:
//!
//! ```text
//! quantum::optimization
//!          |
//!          v
//! candidate IR
//!          |
//!          v
//! resilience::verification::semantic
//! ```
//!
//! Optimization must not modify verification rules in order to make its own
//! candidate pass.
//!
//! # Integration with QEC
//!
//! QEC owns error correction and decoding semantics.
//!
//! Verification may consume:
//!
//! - logical-error evidence;
//! - syndrome evidence;
//! - decoder confidence;
//! - logical-resource identity;
//! - QEC execution provenance.
//!
//! It does not implement a QEC decoder.
//!
//! QEC success is evidence, not automatic proof of end-to-end program
//! correctness.
//!
//! # Integration with ZQN
//!
//! ZQN owns canonical quantum fault/noise semantics.
//!
//! Verification may consume canonical ZQN fault evidence, including physical
//! fault classifications and locations.
//!
//! It must not create a competing fault ontology merely for verification.
//!
//! # Integration with hardware
//!
//! Hardware HAL components provide target/resource evidence.
//!
//! Verification consumes those contracts through higher-level resilience
//! request/context objects.
//!
//! This module must never contact a provider directly.
//!
//! It must never contain branches such as:
//!
//! ```text
//! if IBM ...
//! if Rigetti ...
//! if IonQ ...
//! ```
//!
//! Provider-specific behavior belongs behind the hardware/provider abstraction.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume verification reports for metrics such as:
//!
//! - accepted executions;
//! - rejected executions;
//! - verification failures;
//! - semantic failures;
//! - confidence failures;
//! - recovery cycles;
//! - degraded acceptance;
//! - verification overhead.
//!
//! Benchmarking must not redefine what constitutes an accepted production
//! result.
//!
//! # Integration with simulation
//!
//! The same verification contracts should be usable against simulated,
//! emulated, and physical execution evidence.
//!
//! A simulator must not receive a weaker verification architecture merely
//! because its execution is deterministic.
//!
//! # Integration with distributed execution
//!
//! The verification namespace must remain independent of whether the execution
//! occurs on:
//!
//! - one QPU;
//! - multiple QPUs;
//! - a heterogeneous quantum fleet;
//! - distributed logical resources;
//! - a simulator/emulator.
//!
//! Distributed identity and synchronization evidence should be supplied by
//! the relevant execution/coordination subsystem.
//!
//! # Public API policy
//!
//! The child modules remain individually addressable:
//!
//! ```text
//! quantum::resilience::verification::invariant
//! quantum::resilience::verification::semantic
//! quantum::resilience::verification::result
//! quantum::resilience::verification::confidence
//! quantum::resilience::verification::provenance
//! quantum::resilience::verification::acceptance
//! quantum::resilience::verification::verifier
//! ```
//!
//! The root module also re-exports their public items so callers can use the
//! stable verification namespace without depending on file layout:
//!
//! ```text
//! quantum::resilience::verification::VerificationPolicy
//! quantum::resilience::verification::ResilienceVerifier
//! ```
//!
//! Re-exports are intentionally explicit rather than wildcard imports.
//! This prevents accidental public API expansion when implementation files
//! gain new internal types.
//!
//! # Compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Maintenance rule
//!
//! When adding a new verification dimension:
//!
//! 1. create a dedicated child module;
//! 2. define its independent contract;
//! 3. make it deterministic where possible;
//! 4. define its failure semantics;
//! 5. integrate it into `verifier.rs`;
//! 6. integrate its mandatory/optional policy into the verifier/acceptance
//!    contract;
//! 7. add tests;
//! 8. add its public re-export here only after its API is stable.
//!
//! Do not put verification algorithms into this file.
//!
//! # No hidden integration
//!
//! This root module deliberately contains no:
//!
//! - initialization;
//! - global registry;
//! - global singleton;
//! - background thread;
//! - network access;
//! - filesystem access;
//! - environment access;
//! - time access;
//! - random-number generation;
//! - provider access;
//! - hardware access.
//!
//! Therefore simply importing:
//!
//! ```text
//! crate::quantum::resilience::verification
//! ```
//!
//! has no runtime side effects.
//!
//! # Production invariant
//!
//! The most important invariant of this module is:
//!
//! > The existence of a verification module must never itself imply that a
//! > result has been verified.
//!
//! Only an actual verification operation producing a valid verification report
//! and satisfying the acceptance contract may establish acceptance.
//!
//! ============================================================================
//! MODULE DECLARATIONS
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// Verification dimensions
// -----------------------------------------------------------------------------

/// Structural/resource identity invariants.
///
/// This module uses canonical `quantum::ir::qubit` identities and does not
/// define resilience-local qubit identifiers.
pub mod invariant;

/// Semantic equivalence verification against canonical quantum semantics.
pub mod semantic;

/// Execution-result validation.
pub mod result;

/// Verification-confidence assessment.
pub mod confidence;

/// Verification provenance and integrity.
pub mod provenance;

/// Final acceptance-policy evaluation.
pub mod acceptance;

/// Aggregate verification orchestration.
///
/// This is the primary composition boundary used by the resilience controller.
pub mod verifier;

// ============================================================================
// Stable public API
// ============================================================================
//
// Re-exports are intentionally explicit.
//
// IMPORTANT:
// Do not replace these with:
//
//     pub use invariant::*;
//     pub use semantic::*;
//     ...
//
// Wildcard re-exports make the root namespace unstable and can create name
// collisions as individual verification files evolve.
//
// The verifier is the principal public integration boundary. Its public
// contract should therefore be exported explicitly below.
//
// ============================================================================

// -----------------------------------------------------------------------------
// Verifier public contract
// -----------------------------------------------------------------------------
//
// The current verifier owns these core public types:
//
// - VerificationPolicy
// - VerificationStage
// - VerificationState
// - VerificationConfidence
// - VerificationRequest
// - VerificationStageResult
// - VerificationViolation
// - VerificationViolationKind
// - VerificationDecision
// - VerificationReport
// - ResilienceVerifier
// - RESILIENCE_VERIFIER_SCHEMA_ID
// - RESILIENCE_VERIFIER_SCHEMA_VERSION
//
// These are re-exported here so higher-level resilience components do not need
// to depend on the physical file layout of the verification subsystem.

pub use verifier::{
    ExecutionIdentity,
    ResilienceVerifier,
    SemanticFingerprint,
    VerificationConfidence,
    VerificationDecision,
    VerificationPolicy,
    VerificationReport,
    VerificationRequest,
    VerificationResourceScope,
    VerificationStage,
    VerificationStageResult,
    VerificationState,
    VerificationViolation,
    VerificationViolationKind,
    RESILIENCE_VERIFIER_SCHEMA_ID,
    RESILIENCE_VERIFIER_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Acceptance public contract
// -----------------------------------------------------------------------------
//
// Acceptance is kept separate from aggregate verification so that:
// - verification evidence can be inspected independently;
// - acceptance remains the final fail-closed boundary;
// - recovery can react to Repeat/Escalate without forging acceptance.

pub use acceptance::{
    AcceptanceEvaluator,
    AcceptanceMode,
    AcceptanceOutcome,
    AcceptanceReason,
    ACCEPTANCE_SCHEMA_ID,
    ACCEPTANCE_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Invariant public contract
// -----------------------------------------------------------------------------
//
// These types describe structural verification only.
// They do not replace canonical quantum IR types.

pub use invariant::{
    InvariantId,
    InvariantMode,
    InvariantPolicy,
    InvariantResult,
    InvariantSeverity,
};

// -----------------------------------------------------------------------------
// Result public contract
// -----------------------------------------------------------------------------
//
// Result verification remains separate from semantic verification.
//
// The result module may evolve its detailed result evidence independently.
// Only stable root-level result types should be promoted here.
//
// NOTE:
// Keep detailed implementation/result evidence accessible through:
//     verification::result::...
// until its public API is intentionally stabilized.
//
// -----------------------------------------------------------------------------
// Confidence public contract
// -----------------------------------------------------------------------------
//
// Confidence has two layers:
// - verifier-level confidence used by the aggregate verifier;
// - detailed confidence analysis implemented by confidence.rs.
//
// The aggregate verifier's `VerificationConfidence` is therefore the canonical
// acceptance-path representation and is already re-exported above.
//
// -----------------------------------------------------------------------------
// Provenance public contract
// -----------------------------------------------------------------------------
//
// Provenance remains available as:
//     verification::provenance::...
//
// Its detailed types are deliberately not wildcard-re-exported here because
// provenance schemas can evolve independently of the aggregate verifier.
//
// ============================================================================
// Compile-time namespace invariants
// ============================================================================
//
// The following type aliases intentionally contain no data and perform no
// runtime work. They make the intended canonical identity boundary explicit
// at this module boundary.
//
// They do NOT create new identity types.
// They are aliases to the canonical IR types.
//
// ============================================================================

/// Canonical logical-qubit identity used by resilience verification.
///
/// This alias exists only as a namespace convenience. It does not create a
/// second identity system.
pub type LogicalQubitId = crate::quantum::ir::qubit::QubitId;

/// Canonical physical-qubit identity used by resilience verification.
///
/// This alias exists only as a namespace convenience. It does not create a
/// second identity system.
pub type PhysicalQubitId = crate::quantum::ir::qubit::PhysicalQubitId;

// ============================================================================
// Integration preconditions
// ============================================================================
//
// Higher-level code integrating this module must satisfy the following:
//
// 1. Canonical IR must be used for logical quantum semantics.
// 2. Canonical `QubitId` and `PhysicalQubitId` must be used for resource
//    identity.
// 3. Hardware-specific information must arrive through the hardware HAL.
// 4. Routing must be performed by the routing subsystem.
// 5. Scheduling must be performed by the scheduling subsystem.
// 6. Optimization must be performed by the optimization subsystem.
// 7. QEC must be performed by the QEC subsystem.
// 8. Fault semantics must come from the canonical ZQN subsystem.
// 9. Execution evidence must be supplied by the runtime/hardware boundary.
// 10. Verification must be run after the final adaptation/recovery candidate
//     has been constructed.
// 11. Acceptance must be determined only by the verification boundary.
// 12. A failed verification result must never be silently converted to success.
//
// ============================================================================
// Integration example
// ============================================================================
//
// Higher-level resilience orchestration should conceptually perform:
//
// ```ignore
//! use crate::quantum::resilience::verification::{
//!     ResilienceVerifier,
//!     VerificationPolicy,
//! };
//
// let verifier = ResilienceVerifier::new(...);
//
// let report = verifier.verify(request, VerificationPolicy::strict())?;
//
// if report.accepted() {
//     // Only now may the result enter the production acceptance path.
// } else {
//     // Delegate to resilience recovery/escalation policy.
// }
// ```
//
// The exact constructor/method signatures remain owned by `verifier.rs`.
// This root module intentionally does not duplicate them.
//
// ============================================================================
// Integration with recovery
// ============================================================================
//
// Recovery must use verification results as evidence:
//
// ```text
//! recovery
//!    |
//!    v
//! candidate
//!    |
//!    v
//! verification
//!    |
//!    +---- accepted ------> result
//!    |
//!    +---- repeat --------> recovery cycle
//!    |
//!    +---- escalate ------> operator/policy boundary
//!    |
//!    +---- reject --------> failure
//! ```
//
// Recovery must never manufacture an `AcceptanceOutcome`.
//
// ============================================================================
// Integration with adaptation
// ============================================================================
//
// Every material adaptation should eventually be represented in the
// verification provenance:
//
// ```text
//! logical program
//!      |
//!      v
//! adaptation
//!      |
//!      +--> remapping
//!      +--> rerouting
//!      +--> rescheduling
//!      +--> recompilation
//!      +--> reoptimization
//!      +--> QEC adaptation
//!      +--> backend migration
//!      |
//!      v
//! verification
// ```
//
// Semantic verification determines whether the resulting computation remains
// compatible with the requested semantics.
//
// ============================================================================
// Integration with mitigation
// ============================================================================
//
// Mitigation may change the execution protocol, number of executions,
// sampling strategy, or result-estimation procedure.
//
// Therefore:
//
// ```text
//! mitigation
//!     |
//!     v
//! candidate result/evidence
//!     |
//!     v
//! result verification
//!     |
//!     v
//! semantic/provenance verification
//!     |
//!     v
//! acceptance
//! ```
//
// Mitigation success is not itself acceptance.
//
// ============================================================================
// Integration with observability
// ============================================================================
//
// Telemetry/observability may consume verification reports, but verification
// must not require an observability implementation.
//
// The dependency therefore remains:
//
// ```text
//! verification -----> structured report
//!                          |
//!                          v
//!                    observability
//! ```
//
// rather than:
//
// ```text
//! verification -----> concrete telemetry backend
//! ```
//
// This preserves provider/platform independence.
//
// ============================================================================
// Integration with persistence
// ============================================================================
//
// Verification reports may be serialized and persisted through the resilience
// serialization boundary.
//
// This module must not directly perform persistence.
//
// ============================================================================
// Testing contract
// ============================================================================
//
// The verification subsystem should be tested at three levels:
//
// 1. child-module unit tests;
// 2. aggregate verifier integration tests;
// 3. end-to-end resilience tests.
//
// The root module itself requires only namespace/integration compilation tests
// because it intentionally contains no runtime algorithm.
//
// Recommended integration coverage:
//
// ```text
//! canonical IR
//!      |
//!      v
//! adaptation
//!      |
//!      v
//! verification
//!      |
//!      v
//! acceptance
//! ```
//
// Test at arbitrary resource sizes generated from the target resource model.
// Do not encode tests that imply a maximum supported machine size.
//
// ============================================================================
// Rust/toolchain contract
// ============================================================================
//
// Required:
//
// - Rust 1.97
// - Rust 1.97.1
// - Rust 2021 edition
// - stable toolchain
// - no nightly features
// - no unsafe
//
// This file contains no unsafe implementation and explicitly forbids unsafe
// code above.
//
// ============================================================================
// End of module
// ============================================================================