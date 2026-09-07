//! Zamani Quantum Resilience — Limits Module
//!
//! Path:
//!     src/quantum/resilience/limits/mod.rs
//!
//! Purpose:
//!     Public module boundary for provider-independent, machine-size-
//!     independent resilience resource limits.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This module composes the three independent responsibilities of the
//! resilience limits subsystem:
//
//!     limits.rs
//!         Generic limit semantics and composable constraint sets.
//
//!     resource.rs
//!         Resource demand, availability, reservation and resource identity.
//
//!     validation.rs
//!         Deterministic validation of resource requirements against explicit
//!         limits and known availability.
//
//! This file intentionally contains NO limit implementation logic.
//!
//! The implementation ownership is:
//
//!     quantum::resilience::limits::limits
//!         -> limit semantics
//!
//!     quantum::resilience::limits::resource
//!         -> resource semantics
//!
//!     quantum::resilience::limits::validation
//!         -> validation semantics
//!
//! The parent module is:
//
//!     quantum::resilience
//!
//! and the quantum identity authority remains:
//
//!     quantum::ir::qubit
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! This module imposes no architectural machine-size limit.
//!
//! In particular, this module MUST NOT define:
//
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_RECOVERY_ATTEMPTS
//!     MAX_INCIDENTS
//!     MAX_BACKENDS
//!     MAX_DEVICES
//!     MAX_MEMORY
//!
//! or any equivalent fixed quantum-system ceiling.
//!
//! "Unlimited" means:
//!
//!     this resilience layer imposes no finite limit.
//!
//! It does NOT mean:
//!
//!     physically infinite resources.
//!
//! The effective executable workload is determined by the intersection of:
//
//!     program demand
//!         ∩
//!     resilience policy
//!         ∩
//!     target capabilities
//!         ∩
//!     runtime availability
//!         ∩
//!     security/resource policy
//!         ∩
//!     execution budgets
//!
//! This allows the same Zamani program to remain semantically independent of
//! whether it eventually runs on one qubit, a small QPU, a large QPU, a
//! distributed quantum system, or a future quantum architecture.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! The intended dependency graph is:
//
//!     quantum::ir::qubit
//!              │
//!              │ canonical identities
//!              ▼
//!     quantum::hardware
//!              │
//!              │ target capabilities
//!              ▼
//!     quantum::routing
//!              │
//!              │ physical realization requirements
//!              ▼
//!     quantum::scheduling
//!              │
//!              │ temporal/resource requirements
//!              ▼
//!     quantum::resilience::limits::resource
//!              │
//!              │ resource demand/availability
//!              ▼
//!     quantum::resilience::limits::limits
//!              │
//!              │ explicit resilience constraints
//!              ▼
//!     quantum::resilience::limits::validation
//!              │
//!              │ feasibility result
//!              ▼
//!     quantum::resilience::planning
//!              │
//!              ▼
//!     quantum::resilience::recovery
//!
//! This module does not reverse that dependency direction.
//!
//! It must not import:
//
//!     quantum::hardware
//!     quantum::routing
//!     quantum::scheduling
//!     quantum::optimization
//!     quantum::qec
//!     quantum::frontend
//!     quantum::backend
//!     provider SDKs
//!     network clients
//!     filesystem implementations
//!
//! Those dependencies belong to the concrete consumers of this subsystem.
//!
//! ============================================================================
//! CANONICAL QUANTUM IDENTITY
//! ============================================================================
//!
//! This module does not define or re-export a competing quantum identity type.
//!
//! Whenever a resource implementation needs to identify an actual quantum
//! resource, the canonical identities are:
//
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The resilience limits layer primarily operates on resource quantities and
//! therefore normally does not need to manipulate qubit IDs directly.
//!
//! This separation is intentional:
//
//!     QubitId
//!         = semantic logical identity
//!
//!     PhysicalQubitId
//!         = physical-target identity
//!
//!     resource quantity
//!         = amount of capacity/demand
//!
//!     hardware capability
//!         = what a particular target actually provides
//!
//! No `ResilienceQubitId`, `LogicalQubitId`, or `PhysicalQubit` replacement
//! may be introduced here.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Rust 2021.
//!
//! Supported compiler baseline:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!
//! The complete resilience subsystem is required to remain safe Rust.
//!
//! No unsafe code is permitted.
//!
//! `forbid(unsafe_code)` is placed at this module boundary as an additional
//! compiler-enforced invariant. Child modules independently enforce the same
//! rule so that the guarantee remains true even if this module is reorganized.
//!
//! ============================================================================
//! API OWNERSHIP
//! ============================================================================
//!
//! `mod.rs` owns:
//
//!     - module composition;
//!     - public module visibility;
//!     - stable, intentional facade exports.
//!
//! `mod.rs` does NOT own:
//
//!     - resource arithmetic;
//!     - limit comparison;
//!     - availability evaluation;
//!     - hardware discovery;
//!     - validation algorithms;
//!     - recovery decisions;
//!     - policy decisions;
//!     - scheduling;
//!     - routing;
//!     - QEC;
//!     - mitigation;
//!     - execution.
//!
//! Keeping this file thin prevents the module boundary from becoming another
//! implementation layer.
//!
//! ============================================================================
//! PUBLIC MODULES
//! ============================================================================
//!
//! The order below reflects dependency ownership:
//
//!     limits
//!         foundational constraint semantics
//!
//!     resource
//!         demand/availability semantics built around constraints
//!
//!     validation
//!         validation built around both
//!
//! The declarations are intentionally explicit rather than using generated
//! module discovery or macro-based registration.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing consumers may use:
//
//!     quantum::resilience::limits::limits::*
//!     quantum::resilience::limits::resource::*
//!     quantum::resilience::limits::validation::*
//!
//! This module additionally provides:
//
//!     quantum::resilience::limits::Limit
//!     quantum::resilience::limits::LimitSet
//!
//! and other explicitly selected canonical exports below.
//!
//! The facade intentionally avoids a wildcard re-export of every child symbol.
//! This prevents unrelated future additions from silently changing the public
//! API or creating name collisions.
//!
//! ============================================================================
//! SERIALIZATION / DETERMINISM
//! ============================================================================
//!
//! Limit objects are policy data and may participate in:
//
//!     serialization
//!     provenance
//!     deterministic planning
//!     recovery replay
//!     checkpoint compatibility
//!
//! This module does not define serialization itself.
//!
//! Serialization ownership remains with:
//
//!     quantum::resilience::serialization
//!
//! The types exposed here must therefore remain deterministic and semantically
//! stable without depending on runtime state.
//!
//! ============================================================================
//! VALIDATION SEMANTICS
//! ============================================================================
//!
//! A successful validation means only:
//
//!     supplied demand
//!         is compatible with
//!     supplied resilience limits
//!         and
//!     supplied known resource availability.
//!
//! It does NOT prove:
//
//!     - quantum-program semantic correctness;
//!     - hardware execution correctness;
//!     - QEC correctness;
//!     - routing optimality;
//!     - scheduling optimality;
//!     - result correctness.
//!
//! Those guarantees belong to their respective subsystems.
//!
//! Unknown availability must remain distinct from unlimited capacity.
//!
//! In safety-sensitive validation:
//
//!     Unknown != Unlimited
//!
//! and:
//
//!     policy Unlimited != hardware Unlimited
//!
//! ============================================================================
//! NO I/O
//! ============================================================================
//!
//! The limits module is a pure policy/resource boundary.
//!
//! It must not:
//
//!     - read clocks;
//!     - access files;
//!     - access networks;
//!     - discover hardware;
//!     - query providers;
//!     - acquire locks;
//!     - spawn threads;
//!     - perform random operations;
//!     - mutate global state.
//!
//! This makes limit evaluation deterministic, testable and suitable for
//! distributed and replayable resilience planning.
//!
//! ============================================================================
//! RESOURCE ACCOUNTING
//! ============================================================================
//!
//! Resource quantities are supplied by callers and lower layers.
//!
//! Examples include:
//
//!     logical qubits
//!     physical qubits
//!     operations
//!     circuit depth
//!     shots
//!     recovery attempts
//!     mitigation executions
//!     checkpoints
//!     storage
//!     CPU
//!     GPU
//!     network traffic
//!     telemetry
//!     concurrency
//!     execution time
//!     compilation time
//!     queue time
//!     recovery time
//!     cost
//!     energy
//!
//! The list is extensible through the child resource model.
//!
//! This module must never infer resource counts from fixed constants.
//!
//! ============================================================================
//! LARGE-SCALE PROCESSING
//! ============================================================================
//!
//! The public boundary is compatible with workloads larger than the available
//! in-memory representation because resource requirements and availability can
//! be evaluated incrementally by consumers.
//!
//! This module must not require materializing:
//
//!     every qubit;
//!     every operation;
//!     every backend;
//!     every resource;
//!     every recovery attempt.
//!
//! Concrete collection/storage decisions belong to resource and execution
//! layers.
//!
//! ============================================================================
//! ERROR OWNERSHIP
//! ============================================================================
//!
//! Child modules own their domain errors.
//
//! Higher-level resilience orchestration may translate those errors into the
//! canonical resilience error taxonomy:
//
//!     quantum::resilience::errors
//!
//! This module must not duplicate the global resilience error hierarchy.
//!
//! ============================================================================
//! MODULE COMPOSITION
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Generic, composable resilience-limit semantics.
///
/// This module owns:
///
/// - `Limit`;
/// - `LimitSet`;
/// - resource categories;
/// - limit violations;
/// - limit composition;
/// - bounded/unbounded semantics.
///
/// It does not know anything about a particular QPU, provider or recovery
/// algorithm.
pub mod limits;

/// Provider-independent resource demand and availability semantics.
///
/// This module owns the representation of:
///
/// - resource requirements;
/// - resource availability;
/// - reservations;
/// - resource quantities;
/// - logical/physical resource references where required.
///
/// It consumes canonical quantum identities from `quantum::ir::qubit` when a
/// quantum resource must be identified.
pub mod resource;

/// Deterministic resource validation.
///
/// This module combines:
///
/// - resource requirements;
/// - known availability;
/// - explicit resilience limits.
///
/// It does not perform hardware discovery, routing, scheduling, optimization,
/// QEC, recovery or I/O.
pub mod validation;

// ============================================================================
// Stable facade exports
// ============================================================================
//
// Only symbols whose ownership is unambiguous and whose names are already part
// of the limits contract are promoted here.
//
// Child-module APIs remain available through their canonical module paths.
//
// Avoiding `pub use limits::*` is deliberate: future additions to child
// modules must not accidentally become part of the root resilience-limits API.

pub use limits::{
    Limit,
    LimitSet,
    LimitViolation,
    ResourceKind,
};

// ============================================================================
// Prelude
// ============================================================================
//
// The prelude is intentionally small and deterministic.
//
// It is useful for consumers such as:
//
//     planning::feasibility
//!     policy::budgets
//!     recovery::*
//!     telemetry::*
//!
//! Specialized resource/validation types should continue to be imported from
//! their owning modules rather than flattened here.

/// Minimal stable import surface for common resilience-limit operations.
pub mod prelude {
    pub use super::{
        Limit,
        LimitSet,
        LimitViolation,
        ResourceKind,
    };
}

// ============================================================================
// Compile-time integration notes
// ============================================================================
//
// The following intended usage patterns are documented here rather than
// implemented as hidden coupling.
//
// -----------------------------------------------------------------------------
// Planning
// -----------------------------------------------------------------------------
//
// `planning::feasibility` should consume:
//
//     crate::quantum::resilience::limits::LimitSet
//
// together with:
//
//     crate::quantum::resilience::limits::resource
//
// and target capabilities supplied by the hardware subsystem.
//
// It must not depend on private fields of these types.
//
// -----------------------------------------------------------------------------
// Policy
// -----------------------------------------------------------------------------
//
// `policy::budgets` should construct explicit `Limit` values.
//
// Example semantic relationship:
//
//     policy limit
//         ∩
//     deployment limit
//         ∩
//     runtime limit
//         = effective limit
//
// `Limit::intersection` is owned by `limits.rs`.
//
// -----------------------------------------------------------------------------
// Recovery
// -----------------------------------------------------------------------------
//
// `recovery::*` must receive attempt/time/resource constraints from policy and
// planning.
//
// It must NOT introduce constants such as:
//
//     retry three times
//!     retry five times
//!     maximum 127 qubits
//
// -----------------------------------------------------------------------------
// Hardware
// -----------------------------------------------------------------------------
//
// `quantum::hardware` owns actual target capacity.
//
// For example, a target may report:
//
//     N physical qubits
//
// where N is discovered from the target.
//
// The resilience limit layer then evaluates:
//
//     requested <= explicit policy limit
//
// and independently:
//
//     requested <= known target availability
//
// The limit layer does not discover N.
//
// -----------------------------------------------------------------------------
// Routing
// -----------------------------------------------------------------------------
//
// `quantum::routing` determines physical realization demand.
//
// The resource layer may represent that demand.
//
// The limits module does not select a route.
//
// -----------------------------------------------------------------------------
// Scheduling
// -----------------------------------------------------------------------------
//
// `quantum::scheduling` determines temporal/concurrency requirements.
//
// The resource layer may represent those requirements.
//
// The limits module does not schedule operations.
//
// -----------------------------------------------------------------------------
// QEC
// -----------------------------------------------------------------------------
//
// QEC remains responsible for encoded/logical resource requirements.
//
// Resilience may validate those requirements against available resources.
//
// This module does not implement QEC.
//
// -----------------------------------------------------------------------------
// Verification
// -----------------------------------------------------------------------------
//
// Successful limit validation is not result verification.
//
// The verification subsystem remains responsible for semantic and execution
// correctness.
//
// ============================================================================
// MAINTENANCE RULE
// ============================================================================
//
// When adding a new child module:
//
//     1. Implement its contract first.
//     2. Keep its dependency direction explicit.
//     3. Add its `pub mod` declaration here.
//     4. Add facade exports only for deliberately stable symbols.
//     5. Do not move implementation logic into this file.
//     6. Do not introduce duplicate resource or qubit types.
//     7. Do not introduce machine-size constants.
//     8. Preserve `#![forbid(unsafe_code)]`.
//     9. Ensure Rust 1.97 / 1.97.1 compatibility.
//    10. Ensure deterministic behavior.
//
// When changing a public child API:
//
//     - update its owning module;
//     - update compatibility documentation;
//     - do not compensate by duplicating the type here.
//
// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================
//
// The following invariants are intentionally recorded at the module boundary:
//
//     INVARIANT 1
//         No fixed quantum-machine size exists here.
//
//     INVARIANT 2
//         `Unlimited` means no constraint imposed by this layer.
//
//     INVARIANT 3
//         `Unlimited` never means infinite hardware.
//
//     INVARIANT 4
//         Unknown resource availability is not unlimited availability.
//
//     INVARIANT 5
//         Policy limits cannot create target capabilities.
//
//     INVARIANT 6
//         Resource identity is not resource capacity.
//
//     INVARIANT 7
//         Logical and physical qubit identities remain distinct.
//
//     INVARIANT 8
//         Canonical qubit identities remain owned by `quantum::ir::qubit`.
//
//     INVARIANT 9
//         Limit evaluation is deterministic for deterministic inputs.
//
//     INVARIANT 10
//         This module performs no I/O or hardware discovery.
//
//     INVARIANT 11
//         Arithmetic overflow must be rejected by the owning implementation.
//
//     INVARIANT 12
//         Validation cannot silently truncate resource quantities.
//
//     INVARIANT 13
//         Limits do not perform routing, scheduling, optimization, QEC or
//         recovery.
//
//     INVARIANT 14
//         No provider-specific assumptions may enter this module.
//
//     INVARIANT 15
//         No unsafe Rust is permitted.
//
//     INVARIANT 16
//         Public facade exports must not create competing API ownership.
//
// ============================================================================
// END OF MODULE
// ============================================================================