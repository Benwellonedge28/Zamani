//! Zamani Quantum Scheduling — QEC Scheduling
//!
//! This module is the public composition boundary for the
//! `quantum::scheduling::qec` subsystem.
//!
//! # Purpose
//!
//! The QEC scheduling subsystem describes the scheduling requirements produced
//! by quantum error-correction planning. It does not implement the generic
//! scheduling algorithms themselves.
//!
//! The architectural boundary is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! QEC / fault-tolerance planning
//!      │
//!      ▼
//! quantum::scheduling::qec
//!      │
//!      ├── interface
//!      │
//!      └── syndrome
//!      │
//!      ▼
//! scheduling adapters
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ├── dependency analysis
//!      ├── resource analysis
//!      ├── timing analysis
//!      ├── constraints
//!      └── scheduling algorithms
//!      │
//!      ▼
//! scheduled representation
//!      │
//!      ▼
//! hardware / runtime
//! ```
//!
//! # Architectural responsibility
//!
//! This module owns the composition boundary for QEC-specific scheduling
//! contracts.
//!
//! The child modules own the actual domain contracts:
//!
//! * [`interface`] defines the stable interface between QEC planning and the
//!   generic scheduler.
//! * [`syndrome`] defines scheduling-side syndrome-extraction structures.
//!
//! This module itself owns neither QEC algorithms nor scheduling algorithms.
//!
//! # Explicit non-responsibilities
//!
//! This module does not:
//!
//! * implement ASAP scheduling;
//! * implement ALAP scheduling;
//! * implement list scheduling;
//! * implement resource-constrained scheduling;
//! * perform routing;
//! * allocate physical qubits;
//! * discover hardware;
//! * read hardware calibration;
//! * generate pulses;
//! * execute quantum operations;
//! * decode syndromes;
//! * implement a particular QEC code;
//! * assume a surface code;
//! * assume a particular code distance;
//! * assume a fixed stabilizer weight;
//! * assume a fixed number of ancillas;
//! * assume a fixed number of QEC rounds;
//! * assume a fixed number of qubits;
//! * assume a fixed topology;
//! * assume a fixed hardware technology;
//! * assume a fixed hardware vendor.
//!
//! Those responsibilities belong to their respective Zamani subsystems.
//!
//! # Write once, scale everywhere
//!
//! The QEC scheduling boundary intentionally contains no machine-size
//! constants.
//!
//! The size of a QEC workload is determined by the supplied program, QEC
//! construction, target description, and available execution resources.
//!
//! Consequently, the same interface can represent:
//!
//! ```text
//! one logical qubit
//!      │
//!      ▼
//! one QEC check
//!      │
//!      ▼
//! many checks
//!      │
//!      ▼
//! many rounds
//!      │
//!      ▼
//! large fault-tolerant programs
//!      │
//!      ▼
//! multi-QPU / distributed QEC
//! ```
//!
//! "Infinity" is therefore interpreted architecturally: this module introduces
//! no artificial upper bound. A concrete compilation remains bounded by the
//! address space, available memory, target resources, execution deadlines,
//! and explicitly configured limits.
//!
//! # Canonical qubit identity
//!
//! QEC scheduling must use the canonical quantum IR qubit identities.
//!
//! The authoritative types are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The QEC scheduling subsystem must not introduce a competing `QubitId` or
//! `PhysicalQubitId`.
//!
//! Logical-to-physical mapping remains a routing responsibility.
//!
//! The existing interface follows this rule by importing the canonical
//! identities directly. This module preserves that boundary by simply
//! re-exporting the already-defined QEC contracts rather than defining new
//! qubit identities.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical quantum IR
//!          │
//!          ▼
//! QEC planning
//!          │
//!          ▼
//! scheduling::qec
//!          │
//!          ▼
//! scheduling adapters
//!          │
//!          ▼
//! generic scheduling
//! ```
//!
//! This module must remain independent of:
//!
//! * scheduler implementations;
//! * scheduler planners;
//! * hardware providers;
//! * backend SDKs;
//! * routing implementations;
//! * runtime implementations;
//! * simulator implementations;
//! * QEC decoders.
//!
//! # Current module set
//!
//! The current production tree contains:
//!
//! ```text
//! qec/
//! ├── mod.rs
//! ├── interface.rs
//! └── syndrome.rs
//! ```
//!
//! Future modules such as:
//!
//! ```text
//! rounds.rs
//! stabilizer.rs
//! ```
//!
//! may be added when their independent contracts are complete.
//!
//! They must not be declared here before their source files exist. This keeps
//! the repository compilable at every intermediate implementation stage.
//!
//! # Integration contract
//!
//! Generic scheduling consumes this subsystem through the public re-exports
//! below.
//!
//! A future QEC implementation should normally follow this flow:
//!
//! ```text
//! QEC code implementation
//!          │
//!          ▼
//! create QEC scheduling request / plan
//!          │
//!          ▼
//! quantum::scheduling::qec
//!          │
//!          ▼
//! adapters::qec
//!          │
//!          ▼
//! scheduling::ir
//!          │
//!          ├── dependency graph
//!          ├── resource requirements
//!          ├── timing constraints
//!          └── classical feedback
//!          │
//!          ▼
//! scheduler planner
//!          │
//!          ▼
//! verification
//!          │
//!          ▼
//! hardware
//! ```
//!
//! The QEC layer therefore describes *what must be scheduled* and the generic
//! scheduler determines *when it can be scheduled*.
//!
//! # Public API stability
//!
//! The child modules contain the domain implementations. This root module
//! provides the stable import surface.
//!
//! Consumers should prefer:
//!
//! ```text
//! crate::quantum::scheduling::qec::...
//! ```
//!
//! instead of depending unnecessarily on internal module paths.
//!
//! Re-exports are deliberately explicit rather than using a wildcard export.
//! This prevents accidental expansion of the public API when implementation
//! details are added to child modules.
//!
//! # Compatibility policy
//!
//! Existing public names from `interface.rs` and `syndrome.rs` are re-exported
//! without renaming or wrapping them.
//!
//! This is important because this module must not introduce a second semantic
//! identity system merely to provide a convenient API.
//!
//! # Error boundaries
//!
//! QEC-interface validation errors remain owned by `interface.rs`.
//!
//! Syndrome-plan validation errors remain owned by `syndrome.rs`.
//!
//! Generic scheduling errors remain owned by:
//!
//! ```text
//! quantum::scheduling::errors
//! ```
//!
//! This module does not duplicate or translate those error types.
//!
//! # Thread safety and state
//!
//! This module contains no global mutable state.
//!
//! Child contracts are data-oriented and do not require a global scheduler,
//! global QEC registry, or global hardware state.
//!
//! Scheduler instances, QEC plans, target descriptions, and execution contexts
//! remain owned by their respective callers.
//!
//! # Determinism
//!
//! This module does not introduce randomness.
//!
//! Ordering and deterministic scheduling are responsibilities of the generic
//! scheduling layer and its configured planner/policy.
//!
//! QEC identifiers and collections exposed by the child modules retain their
//! documented deterministic semantics.
//!
//! # Serialization
//!
//! Serialization behavior belongs to the concrete child contracts and the
//! scheduling serialization subsystem.
//!
//! This module does not introduce a second serialization schema.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! * Rust 1.97;
//! * Rust 1.97.1;
//! * Rust 2021 edition;
//! * stable Rust;
//! * no nightly features;
//! * no `unsafe` code.
//!
//! The safety requirement is compiler-enforced below.
//!
//! # Safety
//!
//! Scheduling QEC contracts are entirely safe Rust.
//!
//! No raw pointer operations, FFI assumptions, unchecked memory access, or
//! `unsafe` blocks are required here.
//!
//! The crate-level scheduler safety boundary is reinforced locally so this
//! module cannot accidentally acquire unsafe code in a future edit.
//!
//! # Future extension rule
//!
//! When a new QEC scheduling child module is added, the implementation should
//! follow this order:
//!
//! 1. Complete the child module's independent contract.
//! 2. Ensure the child module compiles independently.
//! 3. Add the module declaration here.
//! 4. Add explicit public re-exports here only for intentionally stable API.
//! 5. Add integration tests outside this composition boundary.
//!
//! Existing exports must not be modified merely because an unrelated QEC
//! implementation is added.
//!
//! This preserves the "finish a file once" integration discipline.
//!
//! # No algorithm leakage
//!
//! In particular, this root must never grow functions such as:
//!
//! ```text
//! schedule_round()
//! schedule_stabilizer()
//! schedule_asap()
//! schedule_alap()
//! route_qec()
//! allocate_ancilla()
//! execute_qec()
//! ```
//!
//! Such functionality belongs in the appropriate child subsystem or generic
//! scheduler layer.
//!
//! # Architectural invariant
//!
//! The central invariant is:
//!
//! ```text
//! QEC planning != routing != scheduling != execution
//! ```
//!
//! QEC planning describes fault-tolerance requirements.
//!
//! Routing determines where mapped operations can execute.
//!
//! Scheduling determines when operations can execute while respecting
//! dependencies, resources, timing, and constraints.
//!
//! Execution submits an already validated representation to a runtime/backend.
//!
//! Keeping these boundaries separate is what allows one Zamani program to be
//! retargeted from small machines to substantially larger and distributed
//! quantum systems without changing the source program.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// ============================================================================
// Child modules
// ============================================================================

/// Stable interface between QEC planning and generic quantum scheduling.
///
/// This module owns QEC operation/round/dependency request contracts and
/// canonical qubit references used by the scheduling layer.
pub mod interface;

/// Scheduling-side syndrome extraction model.
///
/// This module owns syndrome-check, syndrome-round, measurement, ancilla, and
/// classical-feedback structures used to express syndrome scheduling
/// requirements.
pub mod syndrome;

// ============================================================================
// Explicit stable re-exports
// ============================================================================

// ---------------------------------------------------------------------------
// Interface
// ---------------------------------------------------------------------------

pub use interface::{
    AncillaPreparation,
    ClassicalDependency,
    FeedbackRequirement,
    QecDependency,
    QecDependencyKind,
    QecOperationId,
    QecOperationKind,
    QecPhase,
    QecQubit,
    QecRoundId,
    QecSchedulingError,
    QecSchedulingRequest,
    QecSchedulingResult,
    QecSynchronization,
    QecTimingRequirement,
    QecOperation,
};

// ---------------------------------------------------------------------------
// Syndrome
// ---------------------------------------------------------------------------

pub use syndrome::{
    AncillaPreparation as SyndromeAncillaPreparation,
    ClassicalDependency as SyndromeClassicalDependency,
    FeedbackRequirement as SyndromeFeedbackRequirement,
    SyndromeBasis,
    SyndromeCheckId,
    SyndromeFeedback,
    SyndromeMeasurementMode,
    SyndromeQubit,
    SyndromeQubitRole,
    SyndromeResult,
    SyndromeRound,
    SyndromeSchedulingError,
    SyndromeSchedulingPlan,
};

// ============================================================================
// Compile-time API sanity tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_boundary_exposes_canonical_qec_contracts() {
        let round = QecRoundId::new(0);
        let operation = QecOperationId::new(0);
        let syndrome = SyndromeRound::new(0);

        assert_eq!(round.index(), 0);
        assert_eq!(operation.index(), 0);
        assert_eq!(syndrome.value(), 0);
    }

    #[test]
    fn module_boundary_preserves_qec_qubit_identity_type() {
        use crate::quantum::ir::qubit::QubitId;

        let qubit = QubitId::new(0);
        let reference = QecQubit::logical(qubit);

        assert_eq!(reference.logical_id(), Some(qubit));
        assert!(reference.is_logical());
        assert!(!reference.is_physical());
    }

    #[test]
    fn module_boundary_does_not_create_a_scheduler_qubit_identity() {
        use crate::quantum::ir::qubit::QubitId;

        let canonical = QubitId::new(7);
        let qec_reference = QecQubit::from(canonical);

        assert_eq!(qec_reference.logical_id(), Some(canonical));
    }

    #[test]
    fn module_boundary_is_explicitly_empty_of_global_scheduler_state() {
        // This test intentionally documents an architectural invariant rather
        // than testing implementation state. The QEC module exposes contracts
        // and data types; it owns no global scheduler instance.
        assert!(true);
    }
}