//! Zamani Quantum Scheduling — Schedule Verification
//!
//! Production verification boundary for the quantum scheduler.
//!
//! # Purpose
//!
//! This module is the composition root for verification of schedules produced
//! by `crate::quantum::scheduling`.
//!
//! Verification is deliberately separated from scheduling algorithms. A
//! planner is responsible for constructing a candidate schedule; this module
//! is responsible for establishing whether that candidate satisfies the
//! scheduler's declared invariants.
//!
//! The verification pipeline is:
//
//! ```text
//! canonical quantum IR
//!        │
//!        ▼
//! routing / scheduling adapters
//!        │
//!        ▼
//! scheduling IR
//!        │
//!        ▼
//! candidate schedule
//!        │
//!        ▼
//! ┌─────────────────────────────────────────────┐
//! │                  verifier                   │
//! │                                             │
//! │ structural                                  │
//! │ dependency                                  │
//! │ resource                                    │
//! │ timing                                      │
//! │ semantic                                    │
//! └──────────────────────┬──────────────────────┘
//!                        │
//!                        ▼
//!               VerificationReport
//! ```
//!
//! # Verification philosophy
//!
//! A production scheduler must never treat "the planner returned Ok" as proof
//! that a schedule is executable.
//!
//! A candidate schedule is valid only when all enabled verification layers
//! establish their respective invariants.
//!
//! The default production verification path should therefore verify:
//!
//! 1. structural completeness;
//! 2. dependency ordering;
//! 3. resource capacity and exclusivity;
//! 4. timing constraints and alignment;
//! 5. semantic preservation.
//!
//! Additional verification layers may be added without changing the existing
//! verifier contract.
//!
//! # Ownership boundaries
//!
//! This module owns verification of scheduling artifacts.
//!
//! It does NOT own:
//!
//! - Zamani source parsing;
//! - OpenQASM parsing;
//! - canonical quantum semantics;
//! - quantum gate definitions;
//! - canonical `QuantumCircuit`;
//! - canonical `QuantumOperation`;
//! - canonical `Gate`;
//! - canonical `QubitId`;
//! - canonical `PhysicalQubitId`;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - QEC decoding;
//! - optimization;
//! - scheduling policy;
//! - scheduling algorithms.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum identity
//!
//! Verification must never introduce a second qubit identity system.
//!
//! When a verifier needs logical or physical qubit identity it must use the
//! canonical types from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This is consistent with the scheduler IR contract, which explicitly states
//! that scheduling IR does not own `QubitId` or `PhysicalQubitId`. The scheduler
//! IR instead consumes canonical quantum identity through its adapter boundary.
//!
//! # Semantic preservation
//!
//! Scheduling changes *when* operations execute and may materialize explicit
//! timing constructs such as delays, but it must not silently change the
//! computation represented by the canonical quantum IR.
//!
//! The semantic verifier is therefore responsible for detecting discrepancies
//! between the source/canonical operation set and the scheduled representation.
//!
//! Timing-only artifacts must be distinguishable from semantic quantum
//! operations so that legitimate scheduler transformations are not incorrectly
//! rejected.
//!
//! # Static and dynamic programs
//!
//! Verification must support both:
//!
//! - statically schedulable dependency DAGs;
//! - dynamically resolved execution involving measurements, classical
//!   conditions, feedback, or runtime events.
//!
//! A dynamic dependency must not be falsely treated as a statically resolved
//! ordering.
//!
//! Dynamic constraints should be represented by the scheduling IR and verified
//! according to the dynamic execution contract.
//!
//! # Scalability
//!
//! This module contains no artificial machine-size limit.
//!
//! It must not define constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_SCHEDULE_DEPTH
//! MAX_TIME
//! ```
//!
//! The scheduler must scale according to:
//!
//! - available memory;
//! - available CPU;
//! - explicit compiler/user limits;
//! - target resources;
//! - target capabilities;
//! - execution deadlines;
//! - host operating-system limits.
//!
//! "Infinity" in the Zamani architecture means that verification imposes no
//! artificial finite ceiling on machine size.
//!
//! It does NOT mean that finite hardware, finite address space, or finite
//! compilation resources cease to exist.
//!
//! # Complexity
//!
//! Verification implementations should prefer algorithms whose complexity is
//! linear or near-linear in the size of the schedule where possible.
//!
//! For a schedule containing:
//!
//! - `V` operations;
//! - `E` dependency edges;
//! - `R` resource reservations;
//!
//! the dependency verifier should target `O(V + E)` traversal complexity and
//! resource verification should use resource-indexed interval structures rather
//! than constructing a dense time × resource matrix.
//!
//! Verification must never require allocation proportional to:
//!
//! ```text
//! number_of_qubits × scheduling_horizon
//! ```
//!
//! unless an explicitly selected verifier requests such a representation.
//!
//! # Determinism
//!
//! Verification must be deterministic for deterministic input.
//!
//! If the candidate schedule, target snapshot, configuration, and canonical
//! input are identical, verification should produce the same result and stable
//! diagnostic ordering.
//!
//! Parallel verification may be used internally, but externally observable
//! reports must have deterministic ordering.
//!
//! # Concurrency
//!
//! The verification module itself owns no global mutable state.
//!
//! Verifiers should operate on immutable scheduling snapshots whenever possible.
//!
//! Implementations must be safe to call concurrently from multiple scheduler
//! instances without shared mutable global state.
//!
//! No verifier may depend on thread-local mutable state for correctness.
//!
//! # Safety
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! Rust 2021 edition.
//!
//! Stable Rust only.
//!
//! No nightly features.
//!
//! No `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Verification layers
//!
//! The production verification hierarchy is:
//!
//! ```text
//! verification/
//! ├── mod.rs
//! ├── structural.rs
//! ├── dependency.rs
//! ├── resource.rs
//! ├── timing.rs
//! ├── semantic.rs
//! └── verifier.rs
//! ```
//!
//! Each file owns one verification concern.
//!
//! ## structural.rs
//!
//! Verifies structural integrity:
//!
//! - every scheduled operation is structurally valid;
//! - required schedule entries exist;
//! - no operation is duplicated unexpectedly;
//! - schedule references resolve;
//! - resource reservations reference valid operations;
//! - dependency endpoints resolve;
//! - intervals are internally well formed;
//! - mandatory schedule metadata is present.
//!
//! It must not decide whether a schedule is semantically equivalent to the
//! input program. That belongs to `semantic.rs`.
//!
//! ## dependency.rs
//!
//! Verifies precedence constraints:
//!
//! - predecessor operations complete before dependent operations;
//! - dependency endpoints are valid;
//! - static dependency ordering is respected;
//! - cycles are rejected where the scheduling mode requires a DAG;
//! - dynamic dependencies are not incorrectly treated as static;
//! - measurement/classical dependencies are respected according to their
//!   declared readiness semantics.
//!
//! ## resource.rs
//!
//! Verifies resource constraints:
//!
//! - exclusive resources never overlap;
//! - capacity-limited resources do not exceed capacity;
//! - reservations reference valid resources;
//! - operation resource requirements are satisfied;
//! - unavailable resources are not silently consumed;
//! - hierarchical/shared resource constraints are respected;
//! - resource calendars are respected.
//!
//! The verifier must use the resource model supplied by the scheduling
//! context. It must never assume a fixed number of qubits, channels, control
//! lines, resonators, or other hardware resources.
//!
//! ## timing.rs
//!
//! Verifies temporal correctness:
//!
//! - valid start/end times;
//! - valid durations;
//! - no arithmetic overflow;
//! - release times;
//! - deadlines;
//! - temporal windows;
//! - alignment constraints;
//! - timing resolution;
//! - operation-specific timing constraints;
//! - channel-specific timing constraints;
//! - dynamic feedback latency requirements.
//!
//! Timing verification must use the target timing model rather than embedding
//! a particular hardware clock or time unit.
//!
//! ## semantic.rs
//!
//! Verifies that scheduling has not changed quantum semantics.
//!
//! It must account for scheduler-generated timing-only constructs such as
//! delays without treating them as arbitrary semantic substitutions.
//!
//! Depending on the canonical IR contract, semantic verification may compare:
//!
//! - operation identity;
//! - operation ordering where semantically required;
//! - operands;
//! - logical/physical qubit association;
//! - gate identity;
//! - parameters;
//! - measurement targets;
//! - classical dependencies;
//! - conditions;
//! - resets;
//! - control dependencies;
//! - source provenance.
//!
//! The semantic verifier must consume canonical IR contracts rather than
//! defining a competing quantum semantic model.
//!
//! ## verifier.rs
//!
//! Provides the aggregate verifier.
//!
//! It coordinates the individual verification layers and produces one
//! deterministic verification report.
//!
//! It must not duplicate the implementation of the child verifiers.
//!
//! # Verification order
//!
//! The recommended order is:
//!
//! ```text
//! structural
//!     │
//!     ▼
//! dependency
//!     │
//!     ▼
//! resource
//!     │
//!     ▼
//! timing
//!     │
//!     ▼
//! semantic
//! ```
//!
//! Structural verification comes first because later layers rely on valid
//! references.
//!
//! Dependency verification precedes resource and timing verification because
//! those layers may use operation relationships.
//!
//! Semantic verification is intentionally last because it is the strongest
//! cross-boundary check and may require comparing the schedule with canonical
//! source information.
//!
//! An implementation may run independent checks in parallel after structural
//! validation, provided the final diagnostic ordering remains deterministic.
//!
//! # Fail-closed production behavior
//!
//! A production verifier should fail closed.
//!
//! Unknown, malformed, contradictory, or incomplete verification input must
//! produce a verification error rather than being interpreted as valid.
//!
//! In particular:
//!
//! - unknown resource capacity must not be interpreted as unlimited;
//! - unknown timing must not be interpreted as zero duration;
//! - missing dependency endpoints must not be ignored;
//! - missing semantic provenance must not silently establish equivalence;
//! - malformed intervals must not be accepted;
//! - unsupported verification modes must not silently downgrade verification.
//!
//! An explicitly configured analysis-only mode may permit incomplete evidence,
//! but such a result must be distinguishable from a production-valid result.
//!
//! # Diagnostics
//!
//! Verification failures should contain structured information where the child
//! implementation supports it.
//!
//! Useful diagnostic fields include:
//!
//! - verification layer;
//! - operation identity;
//! - canonical operation identity;
//! - logical qubit identity;
//! - physical qubit identity;
//! - resource identity;
//! - dependency identity;
//! - expected value;
//! - observed value;
//! - interval;
//! - constraint identity;
//! - reason;
//! - provenance.
//!
//! Diagnostics must not require parsing human-readable error strings to recover
//! machine-readable information.
//!
//! # Integration with scheduler IR
//!
//! The intended data flow is:
//!
//! ```text
//! crate::quantum::ir
//!          │
//!          ▼
//! scheduling::adapters::ir
//!          │
//!          ▼
//! scheduling::ir
//!          │
//!          ├── operation
//!          ├── dependency
//!          └── graph
//!          │
//!          ▼
//! scheduling::planners
//!          │
//!          ▼
//! candidate schedule
//!          │
//!          ├──────────────┐
//!          │              │
//!          ▼              ▼
//! scheduling::resources  scheduling::timing
//!          │              │
//!          └──────┬───────┘
//!                 ▼
//!       scheduling::verification
//! ```
//!
//! The existing scheduling IR explicitly establishes this separation: it is a
//! normalized scheduling representation rather than a second canonical quantum
//! IR. 
//!
//! # Integration with routing
//!
//! Routing establishes target-compatible logical-to-physical placement.
//!
//! Verification must therefore be capable of checking that scheduled operands
//! agree with the routed representation.
//!
//! The relationship is:
//!
//! ```text
//! quantum::routing
//!       │
//!       ▼
//! scheduling::adapters::routing
//!       │
//!       ▼
//! scheduling::ir
//!       │
//!       ▼
//! scheduling
//!       │
//!       ▼
//! verification::semantic
//! ```
//!
//! Verification must not perform routing itself.
//!
//! # Integration with hardware
//!
//! Hardware supplies the target facts against which a schedule is verified.
//!
//! Conceptually:
//!
//! ```text
//! quantum::hardware
//!       │
//!       ▼
//! scheduling::adapters::hardware
//!       │
//!       ├── capabilities
//!       ├── timing
//!       ├── resources
//!       ├── availability
//!       └── alignment
//!              │
//!              ▼
//!       verification
//! ```
//!
//! Verification must never query a vendor API directly.
//!
//! Hardware I/O belongs to the hardware/provider subsystem.
//!
//! # Integration with QEC
//!
//! QEC-specific scheduling constraints are supplied through the QEC scheduling
//! boundary.
//!
//! Verification may validate:
//!
//! - syndrome extraction ordering;
//! - round dependencies;
//! - ancilla resource requirements;
//! - measurement readiness;
//! - feedback latency;
//! - round timing;
//! - QEC-specific resource constraints.
//!
//! It must not implement a QEC decoder.
//!
//! # Integration with dynamic execution
//!
//! Dynamic schedules may contain operations whose exact execution time depends
//! on runtime events.
//!
//! Verification should distinguish:
//!
//! ```text
//! statically provable constraint
//! ```
//!
//! from:
//!
//! ```text
//! runtime-resolved constraint
//! ```
//!
//! A runtime-resolved constraint must have an explicit contract explaining what
//! must be guaranteed before execution.
//!
//! Verification must not manufacture a static timestamp for an inherently
//! runtime-dependent operation merely to make the candidate appear valid.
//!
//! # Integration with distributed scheduling
//!
//! Distributed schedules may contain:
//!
//! - inter-module operations;
//! - communication reservations;
//! - entanglement-generation windows;
//! - teleportation dependencies;
//! - classical communication latency;
//! - synchronization barriers.
//!
//! These are verified as resources, dependencies, timing constraints, or
//! semantic relationships according to the canonical distributed scheduling
//! contract.
//!
//! The verifier must not assume that all operations execute on one QPU.
//!
//! # Integration with optimization
//!
//! Optimization may consume verification results to determine whether a
//! candidate schedule is acceptable.
//!
//! Verification must not optimize a schedule.
//!
//! The boundary is:
//!
//! ```text
//! optimizer/planner
//!       │
//!       ▼
//! candidate schedule
//!       │
//!       ▼
//! verifier
//!       │
//!       ▼
//! VerificationReport
//!       │
//!       ▼
//! optimizer/planner
//! ```
//!
//! This prevents optimization heuristics from becoming implicit correctness
//! rules.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume verification metrics such as:
//!
//! - verification status;
//! - makespan validity;
//! - resource utilization validity;
//! - constraint counts;
//! - number of violations;
//! - verification time;
//! - schedule size.
//!
//! Benchmarking must not bypass verification when reporting a production-valid
//! schedule.
//!
//! # Integration with diagnostics
//!
//! The diagnostics subsystem may consume structured verification failures to
//! answer questions such as:
//!
//! - Why was operation X delayed?
//! - Which resource caused a conflict?
//! - Which dependency was violated?
//! - Which alignment rule failed?
//! - Why was semantic equivalence rejected?
//!
//! The verifier should therefore expose structured information rather than
//! forcing diagnostics to reverse-engineer messages.
//!
//! # Integration with serialization
//!
//! Serialized schedules must be verified after decoding and before being
//! accepted as executable scheduling artifacts.
//!
//! Recommended flow:
//!
//! ```text
//! bytes
//!   │
//!   ▼
//! decode
//!   │
//!   ▼
//! structural validation
//!   │
//!   ▼
//! full verification
//!   │
//!   ▼
//! executable schedule
//! ```
//!
//! Deserialization alone must never imply validity.
//!
//! # Public API discipline
//!
//! This module should expose the stable verification vocabulary without
//! exposing implementation internals unnecessarily.
//!
//! Prefer explicit re-exports over wildcard exports.
//!
//! This keeps the public API reviewable and prevents accidental coupling to
//! implementation details.
//!
//! # Child-module contracts
//!
//! Every child implementation should independently enforce:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! Child modules must not introduce duplicate quantum semantic types.
//!
//! # Future extension
//!
//! New verification dimensions should receive their own files when they become
//! substantial enough to require independent ownership.
//!
//! Examples include:
//!
//! ```text
//! calibration.rs
//! topology.rs
//! communication.rs
//! qec.rs
//! dynamic.rs
//! distributed.rs
//! security.rs
//! ```
//!
//! A new verifier must:
//!
//! 1. have a single clearly defined responsibility;
//! 2. consume existing scheduler contracts;
//! 3. avoid duplicate canonical IR types;
//! 4. avoid hardware/vendor ownership;
//! 5. avoid global mutable state;
//! 6. remain scalable;
//! 7. have deterministic diagnostics;
//! 8. have unit tests;
//! 9. have property tests where appropriate;
//! 10. be included in the aggregate verifier only after its contract is stable.
//!
//! # Versioning
//!
//! Adding an independent verification layer is normally additive.
//!
//! Changing the meaning of an existing verification result is compatibility
//! sensitive.
//!
//! Removing or renaming a publicly re-exported verifier type is a public API
//! change and must follow the repository's versioning/deprecation policy.
//!
//! # Testing requirements
//!
//! Each child module must provide focused tests for its own invariants.
//!
//! The aggregate verifier must additionally test:
//!
//! - valid schedules;
//! - structurally invalid schedules;
//! - dependency violations;
//! - resource conflicts;
//! - timing violations;
//! - semantic mismatches;
//! - multiple simultaneous violations;
//! - deterministic diagnostic ordering;
//! - empty schedules;
//! - very small schedules;
//! - large schedules;
//! - high fan-out dependency graphs;
//! - high fan-in dependency graphs;
//! - high resource contention;
//! - sparse resource utilization;
//! - dynamic operations;
//! - distributed operations;
//! - QEC schedules;
//! - serialization round trips;
//! - repeated verification of identical input.
//!
//! Scalability tests must increase input size without changing verifier source
//! code or introducing machine-size constants.
//!
//! # Important repository compatibility rule
//!
//! The scheduler IR already defines its own stable module boundary and
//! explicitly states that canonical qubit identities are supplied by
//! `crate::quantum::ir::qubit`. 
//!
//! Verification must preserve that rule.
//!
//! In particular, do not add imports such as:
//!
//! ```rust
//! use crate::quantum::scheduling::QubitId;
//! ```
//!
//! if that would introduce a scheduler-owned qubit identity.
//!
//! Use the canonical path when a verifier needs the type:
//!
//! ```rust
//! use crate::quantum::ir::qubit::QubitId;
//! use crate::quantum::ir::qubit::PhysicalQubitId;
//! ```
//!
//! # Module composition
//!
//! The declarations below are intentionally explicit.
//!
//! The implementation files are the ownership boundaries for each verification
//! concern.
//!
//! This file should remain a composition root and should not grow into a
//! monolithic verifier implementation.
//!
//! # No hidden fallback
//!
//! There must be no implementation here that silently:
//!
//! - ignores a missing verifier;
//! - treats unavailable information as valid;
//! - converts errors into warnings without configuration;
//! - skips semantic verification merely because a schedule is large;
//! - imposes a smaller verification model for large machines.
//!
//! If verification cannot establish a required invariant, the aggregate
//! verifier must report that fact explicitly.
//!
//! # Production acceptance rule
//!
//! A schedule may be marked production-valid only when:
//!
//! ```text
//! structural == pass
//! dependency == pass
//! resource   == pass
//! timing     == pass
//! semantic   == pass
//! ```
//!
//! unless the caller explicitly selected a documented verification profile
//! whose reduced guarantees are represented in the resulting report.
//!
//! A reduced verification profile must never masquerade as full verification.
//!
//! # Summary
//!
//! The verification subsystem answers one question:
//!
//! > Can this candidate schedule be proven, to the level requested by the
//! > selected verification profile, to satisfy the scheduler's structural,
//! > dependency, resource, timing, and semantic invariants?
//!
//! It does not answer:
//!
//! > Is this the best schedule?
//!
//! Optimization answers the latter.
//!
//! It does not answer:
//!
//! > Which physical machine should be selected?
//!
//! Target selection belongs outside verification.
//!
//! It does not answer:
//!
//! > How should the quantum program be transformed?
//!
//! Optimization, synthesis, routing, QEC, and lowering own those concerns.
//!
//! Keeping this boundary strict is what permits Zamani's scheduling verifier to
//! scale from tiny quantum systems to very large and distributed systems
//! without embedding machine-specific assumptions.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Verifies structural integrity of a candidate schedule.
///
/// This module owns reference validity, schedule completeness, duplicate
/// detection, interval well-formedness, and structural invariants.
///
/// It does not verify quantum semantic equivalence.
pub mod structural;

/// Verifies dependency and precedence constraints.
///
/// This module owns static DAG ordering and explicitly represented dynamic
/// dependency contracts.
pub mod dependency;

/// Verifies resource allocation and resource-capacity constraints.
///
/// This module consumes the scheduler resource model and never assumes a
/// fixed hardware resource count.
pub mod resource;

/// Verifies temporal constraints.
///
/// This includes durations, intervals, release times, deadlines, alignment,
/// resolution, and target-supplied timing rules.
pub mod timing;

/// Verifies semantic preservation between canonical quantum computation and
/// the scheduled representation.
///
/// This module must use canonical quantum IR types rather than defining
/// replacement quantum semantics.
pub mod semantic;

/// Coordinates the individual verification layers and produces the aggregate
/// verification result.
pub mod verifier;

// -----------------------------------------------------------------------------
// Stable public exports
// -----------------------------------------------------------------------------
//
// Keep these explicit rather than using wildcard exports. The concrete names
// below are the intended stable vocabulary for the production verifier API.
//
// If a child implementation uses a different concrete type name, that child
// implementation should expose a compatibility alias in its own file rather
// than forcing this composition root to duplicate or redefine the type.

pub use dependency::{
    DependencyVerification,
    DependencyVerificationError,
    DependencyViolation,
};

pub use resource::{
    ResourceVerification,
    ResourceVerificationError,
    ResourceViolation,
};

pub use semantic::{
    SemanticVerification,
    SemanticVerificationError,
    SemanticViolation,
};

pub use structural::{
    StructuralVerification,
    StructuralVerificationError,
    StructuralViolation,
};

pub use timing::{
    TimingVerification,
    TimingVerificationError,
    TimingViolation,
};

pub use verifier::{
    VerificationError,
    VerificationProfile,
    VerificationReport,
    VerificationStatus,
    Verifier,
};

// -----------------------------------------------------------------------------
// Composition-level invariants
// -----------------------------------------------------------------------------
//
// These invariants intentionally live in documentation rather than as runtime
// logic. Runtime logic belongs in verifier.rs and the individual verifier
// modules.
//
// 1. Structural validation precedes reference-dependent validation.
//
// 2. Every operation referenced by a dependency must exist.
//
// 3. Every resource reservation must reference a valid resource.
//
// 4. Every scheduled operation must have a valid temporal representation unless
//    the operation is explicitly runtime-timed.
//
// 5. Exclusive resources must never overlap.
//
// 6. Capacity-limited resources must never exceed their declared capacity.
//
// 7. Dependency constraints must be satisfied.
//
// 8. Timing constraints must be satisfied.
//
// 9. Canonical quantum semantics must remain preserved.
//
// 10. Runtime-dependent constraints must remain explicitly runtime-dependent.
//
// 11. Verification must not create a second qubit identity system.
//
// 12. Verification must not silently downgrade an unknown condition to valid.
//
// 13. Verification must not contain hardware-size constants.
//
// 14. Verification must not perform hardware I/O.
//
// 15. Verification must not mutate global state.
//
// 16. Verification results must be deterministic for deterministic inputs.
//
// 17. Verification diagnostics must be machine-readable where the child
//     verifier exposes structured information.
//
// 18. Full production validity requires all mandatory verification layers to
//     pass.
//
// -----------------------------------------------------------------------------
// Canonical qubit identity reminder
// -----------------------------------------------------------------------------
//
// When implementation files require canonical qubit identifiers, they must
// import:
//
// use crate::quantum::ir::qubit::QubitId;
// use crate::quantum::ir::qubit::PhysicalQubitId;
//
// They must not define:
//
// type QubitId = ...;
// type PhysicalQubitId = ...;
//
// and must not introduce equivalent scheduler-local identities.
//
// This rule follows the scheduler IR contract already established in the
// repository. 
//
// -----------------------------------------------------------------------------
// Dependency direction
// -----------------------------------------------------------------------------
//
// The intended dependency direction is:
//
// canonical quantum IR
//        │
//        ▼
// scheduling adapters
//        │
//        ▼
// scheduling IR
//        │
//        ├───────────────┐
//        ▼               ▼
// resources           timing
//        │               │
//        └───────┬───────┘
//                ▼
//             planners
//                │
//                ▼
//         candidate schedule
//                │
//                ▼
//            verification
//
// Verification may consume the artifacts produced by these systems, but those
// systems must not depend on verifier implementation details.
//
// -----------------------------------------------------------------------------
// Why this file contains no algorithm implementation
// -----------------------------------------------------------------------------
//
// `mod.rs` is deliberately a composition boundary.
//
// Putting graph traversal, interval checking, semantic comparison, resource
// allocation, or timing arithmetic here would make verification difficult to
// maintain and would force unrelated changes into the same file.
//
// The production strategy is:
//
// structural.rs  -> structural correctness
// dependency.rs  -> dependency correctness
// resource.rs    -> resource correctness
// timing.rs      -> temporal correctness
// semantic.rs    -> quantum semantic correctness
// verifier.rs    -> aggregation and public verification contract
//
// This allows each file to be completed and tested against a stable contract
// without making the composition root responsible for implementation details.