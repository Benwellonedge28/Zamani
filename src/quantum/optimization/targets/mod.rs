//! Zamani Quantum Optimization — Target Namespace
//!
//! This module is the authoritative namespace boundary for optimization
//! targets in:
//!
//! `crate::quantum::optimization::targets`
//!
//! # Architectural role
//!
//! An optimization target describes the representation, operation vocabulary,
//! constraints, and target-specific policy against which logical quantum
//! circuits are optimized.
//!
//! The production dependency direction is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                        quantum::ir
//!                              │
//!                              ▼
//!                    quantum::optimization
//!                              │
//!                              ▼
//!                optimization::targets
//!                 ┌────────────┼────────────┐
//!                 │            │            │
//!                 ▼            ▼            ▼
//!             gate_set    constraints    profiles
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                           target
//!                              │
//!                              ▼
//!                         optimization
//!                           planner
//!                              │
//!                              ▼
//!                          passes
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                         scheduling
//!                              │
//!                              ▼
//!                           hardware
//! ```
//!
//! This module is deliberately a **composition and namespace boundary**.
//!
//! It does not implement:
//!
//! - quantum circuit semantics;
//! - quantum gates;
//! - Quantum IR;
//! - optimization passes;
//! - rewrite algorithms;
//! - synthesis algorithms;
//! - routing;
//! - physical topology;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - backend communication;
//! - QPU execution;
//! - simulators;
//! - benchmarking;
//! - quantum error correction;
//! - quantum algorithms;
//! - source-language parsing;
//! - global mutable state.
//!
//! Those responsibilities remain owned by their respective subsystems.
//!
//! # Canonical Quantum IR
//!
//! The canonical semantic representation remains:
//!
//! `crate::quantum::ir`
//!
//! No target module may introduce another:
//!
//! ```text
//! QuantumCircuit
//! QuantumOperation
//! QuantumGate
//! ```
//!
//! Target modules describe what representations and operations an optimizer
//! should prefer. They do not redefine what a quantum operation means.
//!
//! # Target namespace
//!
//! The namespace is intentionally divided into four independent concerns:
//!
//! ```text
//! targets/
//! ├── mod.rs
//! ├── target.rs
//! ├── gate_set.rs
//! ├── constraints.rs
//! └── profiles.rs
//! ```
//!
//! ## `target.rs`
//!
//! Owns the complete immutable `OptimizationTarget` description.
//!
//! It composes:
//!
//! - target identity;
//! - target kind;
//! - target technology metadata;
//! - gate set;
//! - target constraints;
//! - target-level capabilities;
//! - target metadata;
//! - target validation;
//! - target fingerprinting;
//! - target selection/resolution integration.
//!
//! `target.rs` is the authoritative owner of the complete target object.
//!
//! ## `gate_set.rs`
//!
//! Owns the target operation vocabulary.
//!
//! It answers:
//!
//! > Which operations are accepted by this target, and what declarative
//! > properties and cost information are associated with them?
//!
//! It consumes canonical `quantum::ir::GateKind` where a built-in IR gate is
//! represented and supports stable identifiers for custom/future operations.
//!
//! It does not own decomposition rules.
//!
//! ## `constraints.rs`
//!
//! Owns target-wide constraints.
//!
//! These may describe:
//!
//! - qubit/resource capacities;
//! - operation arity;
//! - dynamic-circuit restrictions;
//! - measurement restrictions;
//! - reset restrictions;
//! - parameter restrictions;
//! - approximation policy;
//! - resource limits;
//! - structural restrictions;
//! - logical/fault-tolerant restrictions;
//! - target-specific optimization constraints.
//!
//! Physical topology remains outside this namespace.
//!
//! ## `profiles.rs`
//!
//! Owns reusable target optimization policies/profiles.
//!
//! Profiles may describe target classes such as:
//!
//! - generic;
//! - simulator;
//! - superconducting;
//! - trapped ion;
//! - neutral atom;
//! - photonic;
//! - logical/fault-tolerant;
//! - custom;
//! - minimum-depth;
//! - minimum-two-qubit;
//! - fault-tolerant;
//! - simulation;
//! - aggressive.
//!
//! Profiles do not own individual optimization algorithms.
//!
//! # Why these files are separate
//!
//! The following concepts must never be conflated:
//!
//! ```text
//! Target
//!     │
//!     ├── Gate set
//!     │
//!     ├── Constraints
//!     │
//!     └── Profile/policy
//! ```
//!
//! A gate set says what operations are accepted.
//!
//! A constraint says what the resulting circuit must satisfy.
//!
//! A profile says how optimization should prioritize transformations.
//!
//! A target combines the applicable declarative information into one immutable
//! optimization-time view.
//!
//! # Public module contract
//!
//! This file intentionally exposes the four target components directly:
//!
//! ```text
//! quantum::optimization::targets::target
//! quantum::optimization::targets::gate_set
//! quantum::optimization::targets::constraints
//! quantum::optimization::targets::profiles
//! ```
//!
//! This makes the namespace stable and discoverable without requiring callers
//! to know the physical file layout.
//!
//! # Integration with `optimization::config`
//!
//! `optimization::config` owns user/compiler-facing optimization configuration.
//!
//! Target selection belongs to configuration/policy, while concrete target
//! construction belongs here.
//!
//! Conceptually:
//!
//! ```text
//! OptimizationConfig
//!        │
//!        ▼
//! TargetSelection
//!        │
//!        ▼
//! targets::target
//!        │
//!        ▼
//! OptimizationTarget
//! ```
//!
//! `targets/mod.rs` deliberately does not resolve configuration itself.
//!
//! That responsibility belongs to `target.rs` and the optimizer configuration
//! layer.
//!
//! # Integration with `optimization::cost`
//!
//! The cost model consumes target information.
//!
//! Conceptually:
//!
//! ```text
//! targets::gate_set
//!        │
//!        ├── operation costs
//!        ├── native operations
//!        └── operation properties
//!
//! targets::constraints
//!        │
//!        └── target resource restrictions
//!
//!              ▼
//!       optimization::cost
//! ```
//!
//! The cost subsystem must not duplicate target gate-cost tables.
//!
//! This namespace therefore provides the authoritative target-side source of
//! operation cost hints.
//!
//! # Integration with `optimization::planner`
//!
//! The planner consumes an immutable `OptimizationTarget`.
//!
//! The planner may ask:
//!
//! ```text
//! target.gate_set()
//! target.constraints()
//! target.profile()
//! target.supports(...)
//! target.is_native(...)
//! target.cost(...)
//! target.capabilities()
//! ```
//!
//! The planner uses that information to determine which optimization passes
//! are appropriate.
//!
//! The target namespace does not choose the pass order.
//!
//! Pass planning remains owned by:
//!
//! `crate::quantum::optimization::planner`
//!
//! # Integration with `optimization::pipeline`
//!
//! The optimization pipeline receives the resolved target through the shared
//! optimization context.
//!
//! Conceptually:
//!
//! ```text
//! OptimizationTarget
//!        │
//!        ▼
//! OptimizationContext
//!        │
//!        ▼
//! OptimizationPipeline
//!        │
//!        ▼
//! OptimizationPass
//! ```
//!
//! The pipeline must treat the target as immutable for one optimization run.
//!
//! A pass must not mutate the target.
//!
//! If a different target is required, a new optimization run/context must be
//! constructed.
//!
//! # Integration with `optimization::synthesis`
//!
//! Synthesis consumes the target gate set as a legal-basis/stopping condition.
//!
//! For example:
//!
//! ```text
//! high-level operation
//!        │
//!        ▼
//! synthesis
//!        │
//!        ▼
//! target gate set
//!        │
//!        ▼
//! target-supported operations
//! ```
//!
//! Target gate sets do not themselves contain synthesis algorithms.
//!
//! This prevents target descriptions from becoming coupled to particular
//! decomposition strategies.
//!
//! # Integration with routing
//!
//! Optimization targets may describe logical operation preferences and
//! constraints, but they must not own physical connectivity.
//!
//! The correct boundary is:
//!
//! ```text
//! OptimizationTarget
//!        │
//!        ▼
//! logical optimization
//!        │
//!        ▼
//! quantum::routing
//!        │
//!        ▼
//! physical topology / mapping
//! ```
//!
//! A future optimizer may consume routing-derived cost information through an
//! explicit abstraction, but this target namespace must never import routing
//! merely to describe an optimization target.
//!
//! # Integration with hardware
//!
//! A hardware backend may provide information from which an optimization target
//! is constructed, but the target is a declarative snapshot.
//!
//! It must not:
//!
//! - query hardware;
//! - open network connections;
//! - acquire calibration;
//! - authenticate;
//! - execute circuits;
//! - mutate backend state.
//!
//! The intended direction is:
//!
//! ```text
//! hardware capabilities
//!        │
//!        ▼
//! OptimizationTarget
//!        │
//!        ▼
//! optimizer
//! ```
//!
//! not:
//!
//! ```text
//! optimizer
//!        │
//!        ▼
//! target
//!        │
//!        ▼
//! hardware API
//! ```
//!
//! # Integration with scheduling
//!
//! Targets may contain declarative information useful to cost estimation, but
//! actual execution timing remains owned by scheduling/hardware.
//!
//! Therefore:
//!
//! ```text
//! target duration/cost hints
//!         ≠
//! physical schedule
//! ```
//!
//! `optimize_depth` may optimize logical dependency depth, but it must not turn
//! target definitions into a scheduling subsystem.
//!
//! # Integration with error correction
//!
//! Logical/fault-tolerant target descriptions may express properties such as:
//!
//! - Clifford+T operation availability;
//! - T-count objectives;
//! - logical operation constraints;
//! - magic-state resource hints.
//!
//! They must not implement QEC algorithms.
//!
//! QEC semantics remain owned by:
//!
//! `crate::quantum::error_correction`
//!
//! The target namespace may describe the target against which QEC-aware
//! optimization is performed.
//!
//! # Integration with algorithms
//!
//! Quantum algorithms construct or consume canonical Quantum IR.
//!
//! They do not need to know the internal organization of target files.
//!
//! The intended direction is:
//!
//! ```text
//! quantum::algorithms
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ▼
//! optimization
//!        │
//!        ▼
//! optimization::targets
//! ```
//!
//! Algorithm implementations must not depend on a concrete hardware target
//! merely to construct a logical algorithm.
//!
//! # Integration with benchmarking
//!
//! This namespace must never depend on benchmarking.
//!
//! Benchmarking may consume:
//!
//! - target identifiers;
//! - target fingerprints;
//! - optimization results;
//! - target metadata;
//! - target cost information.
//!
//! The direction is:
//!
//! ```text
//! optimization
//!      │
//!      ▼
//! OptimizationResult
//!      │
//!      ▼
//! benchmarking
//! ```
//!
//! and not the reverse.
//!
//! This prevents a circular dependency between optimization and benchmarking.
//!
//! # Custom and future quantum technologies
//!
//! The target namespace is deliberately not restricted to today's gate-model
//! hardware.
//!
//! Target kinds and operation identifiers may describe:
//!
//! - generic gate-model computation;
//! - simulators;
//! - emulators;
//! - physical QPUs;
//! - logical fault-tolerant systems;
//! - superconducting systems;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - continuous-variable systems;
//! - analog systems;
//! - annealing systems;
//! - measurement-based systems;
//! - future/custom quantum architectures.
//!
//! New operation types should normally be represented through the existing
//! stable target-operation abstraction rather than requiring this `mod.rs`
//! file to change.
//!
//! This is essential for long-term scalability.
//!
//! # "Infinity" scalability requirement
//!
//! Zamani cannot literally guarantee an infinite circuit because every concrete
//! compilation process is bounded by physical resources such as address space,
//! memory, CPU time, storage, and operating-system limits.
//!
//! The correct production requirement is therefore:
//!
//! > No artificial circuit-size ceiling is introduced by the target namespace.
//! > Concrete resource limits are explicit, configurable, and enforced by the
//! > owning optimizer/resource-management components.
//!
//! Target descriptions are therefore independent of circuit size.
//!
//! The operation vocabulary can be large without imposing a corresponding
//! circuit limit.
//!
//! Existing target files intentionally distinguish configuration-safety limits
//! from circuit-size limits. This namespace preserves that distinction.
//!
//! # Determinism
//!
//! Target construction and access must be deterministic.
//!
//! Existing target components use ordered representations where deterministic
//! iteration/fingerprinting matters.
//!
//! This namespace introduces no:
//!
//! - global mutable state;
//! - ambient randomness;
//! - threads;
//! - time-dependent behavior;
//! - environment-dependent target mutation.
//!
//! If a target requires a random seed for some downstream stochastic optimizer,
//! that seed belongs to optimization configuration/context/provenance rather
//! than this namespace.
//!
//! # Immutability
//!
//! Target objects are intended to be immutable snapshots once constructed.
//!
//! This namespace does not provide mutable global registries or singleton
//! targets.
//!
//! Callers may construct multiple independent targets:
//!
//! ```text
//! target A ──► optimization run A
//!
//! target B ──► optimization run B
//!
//! target C ──► optimization run C
//! ```
//!
//! without cross-run state contamination.
//!
//! # Thread safety
//!
//! This namespace deliberately avoids global state.
//!
//! Concrete target objects should therefore be safely shareable according to
//! the ordinary ownership/thread-safety properties of their fields.
//!
//! The namespace itself does not require `Send`, `Sync`, `Arc`, or any global
//! synchronization primitive.
//!
//! This keeps the module usable in:
//!
//! - single-threaded compilation;
//! - parallel compilation;
//! - incremental compilation;
//! - distributed compilation orchestration;
//! - server-side compiler processes;
//! - embedded compiler tooling.
//!
//! # Serialization
//!
//! Serialization ownership remains in the target component that owns the
//! corresponding data model.
//!
//! This module must not duplicate serialization schemas.
//!
//! The authoritative schema identifiers and versions belong to:
//!
//! - `target.rs` for complete targets;
//! - `gate_set.rs` for gate sets;
//! - `constraints.rs` for constraints;
//! - `profiles.rs` for profiles/policies where applicable.
//!
//! If a future serialization facade is introduced, it should re-export these
//! authoritative schemas rather than defining a second representation.
//!
//! # Validation
//!
//! Target validation is owned by the target components.
//!
//! The expected validation hierarchy is:
//!
//! ```text
//! gate_set validation
//!        │
//!        ▼
//! constraints validation
//!        │
//!        ▼
//! target validation
//!        │
//!        ▼
//! OptimizationContext validation
//! ```
//!
//! `targets/mod.rs` does not duplicate those checks.
//!
//! # Stable public API
//!
//! The preferred public namespace is:
//!
//! ```text
//! crate::quantum::optimization::targets
//! ```
//!
//! Consumers should use explicit child modules when they need a specialized
//! type:
//!
//! ```text
//! crate::quantum::optimization::targets::target
//! crate::quantum::optimization::targets::gate_set
//! crate::quantum::optimization::targets::constraints
//! crate::quantum::optimization::targets::profiles
//! ```
//!
//! This file may provide carefully selected re-exports for commonly used
//! target types, but it must not create duplicate definitions.
//!
//! # Re-export policy
//!
//! Re-exports are intentionally limited to the principal public contracts.
//!
//! Specialized implementation details remain available through their owning
//! modules.
//!
//! This gives callers a stable API while preserving freedom to reorganize
//! internals later.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! No `unsafe` code is permitted.
//!
//! No compiler-specific unstable APIs are used.
//!
//! # Safety policy
//!
//! This namespace contains no unsafe operations.
//!
//! The crate-level safety contract is strengthened here as well so that an
//! accidental future unsafe block/function inside this module fails compilation
//! immediately.
//!
//! # Module declarations
//!
//! These declarations are intentionally explicit.
//!
//! `mod.rs` owns only module composition.
//!
//! The implementation contracts remain inside the four child modules.
//!
//! # Future extension rule
//!
//! When adding another target concern, do not automatically modify every target
//! file.
//!
//! Prefer a new independent module when the concern represents a stable,
//! separately owned abstraction.
//!
//! Examples:
//!
//! ```text
//! targets/
//! ├── capabilities.rs
//! ├── cost_hints.rs
//! ├── dialect.rs
//! ├── features.rs
//! └── compatibility.rs
//! ```
//!
//! Such additions should be introduced only when the abstraction cannot be
//! cleanly represented by the existing target/gate-set/constraint/profile
//! contracts.
//!
//! The new module must then be declared here and integrated through explicit
//! contracts.
//!
//! It must not create a second `OptimizationTarget`.
//!
//! # Dependency rule
//!
//! The target namespace may depend downward on canonical quantum IR types and
//! standard-library/serialization facilities required by its child modules.
//!
//! It must not create upward dependencies on:
//!
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - benchmarking;
//! - runtime;
//! - source parsing.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! optimization::targets
//!      │
//!      ├── gate_set
//!      ├── constraints
//!      ├── profiles
//!      └── target
//! ```
//!
//! # Integration with the parent optimization module
//!
//! The parent:
//!
//! `crate::quantum::optimization`
//!
//! must declare this directory with:
//!
//! ```rust
//! pub mod targets;
//! ```
//!
//! It should not recreate this hierarchy inline.
//!
//! The parent optimization module then exposes the target namespace as:
//!
//! ```text
//! quantum::optimization::targets
//! ```
//!
//! # Integration completion contract
//!
//! This file is considered complete when all of the following are true:
//!
//! 1. `target.rs` is reachable as `optimization::targets::target`.
//! 2. `gate_set.rs` is reachable as `optimization::targets::gate_set`.
//! 3. `constraints.rs` is reachable as `optimization::targets::constraints`.
//! 4. `profiles.rs` is reachable as `optimization::targets::profiles`.
//! 5. No duplicate target abstraction is defined here.
//! 6. No duplicate gate-set abstraction is defined here.
//! 7. No duplicate constraint abstraction is defined here.
//! 8. No duplicate profile abstraction is defined here.
//! 9. The parent optimization module can expose this namespace with
//!    `pub mod targets;`.
//! 10. `planner.rs` can consume the target contracts without requiring changes
//!     to this module.
//! 11. `pipeline.rs` can consume the target contracts without requiring changes
//!     to this module.
//! 12. `cost.rs` can consume gate-set cost information without requiring
//!     changes to this module.
//! 13. `synthesis/*` can consume target gate-set information without requiring
//!     changes to this module.
//! 14. Routing remains outside this namespace.
//! 15. Scheduling remains outside this namespace.
//! 16. Hardware execution remains outside this namespace.
//! 17. Benchmarking remains outside this namespace.
//! 18. QEC remains outside this namespace.
//! 19. No unsafe Rust is introduced.
//! 20. No global mutable state is introduced.
//!
//! # Testing
//!
//! Namespace-level tests should remain minimal.
//!
//! The substantive tests belong to the child modules and integration tests.
//!
//! The tests here verify only that the public namespace and principal contracts
//! remain reachable.
//!
//! This avoids coupling namespace composition tests to implementation details.
//!
//! # Public exports
//!
//! These are intentionally kept small. Callers requiring the complete
//! specialized API can use the explicit child module paths.
//!

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Complete immutable optimization-target description.
///
/// This module owns target identity, target composition, target validation,
/// target capabilities, and target-level integration.
pub mod target;

/// Target operation vocabulary.
///
/// This module owns supported/native operations, operation properties, aliases,
/// operation costs, and gate-set validation.
pub mod gate_set;

/// Target-wide optimization constraints.
///
/// This module owns resource, semantic, structural, parameter, dynamic-circuit,
/// approximation, and other target-wide restrictions.
pub mod constraints;

/// Reusable optimization policies for target classes.
///
/// This module owns predefined target optimization profiles and policies. It
/// does not own optimization-pass implementations.
pub mod profiles;

// =============================================================================
// Principal public contracts
// =============================================================================

/// Complete optimization target.
pub use target::OptimizationTarget;

/// Stable target identifier.
pub use target::TargetId;

/// Target semantic category.
pub use target::TargetKind;

/// Target construction/validation result.
pub use target::TargetResult;

/// Target construction/validation error.
pub use target::TargetError;

/// Target operation identifier.
pub use gate_set::OperationId;

/// Target gate set.
pub use gate_set::TargetGateSet;

/// Target gate-set result.
pub use gate_set::GateSetResult;

/// Target gate-set error.
pub use gate_set::GateSetError;

/// Target constraints.
pub use constraints::TargetConstraints;

/// Target-constraint result.
pub use constraints::ConstraintsResult;

/// Target-constraint error.
pub use constraints::ConstraintsError;

/// Target optimization profile.
pub use profiles::TargetProfile;

// =============================================================================
// Namespace-level tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_namespace_exposes_principal_contracts() {
        let _: fn(String) -> TargetResult<TargetId> = TargetId::new;
        let _: fn(String) -> GateSetResult<OperationId> = OperationId::new;

        let _ = std::any::TypeId::of::<OptimizationTarget>();
        let _ = std::any::TypeId::of::<TargetKind>();
        let _ = std::any::TypeId::of::<TargetGateSet>();
        let _ = std::any::TypeId::of::<TargetConstraints>();
        let _ = std::any::TypeId::of::<TargetProfile>();
        let _ = std::any::TypeId::of::<TargetError>();
        let _ = std::any::TypeId::of::<GateSetError>();
        let _ = std::any::TypeId::of::<ConstraintsError>();
    }

    #[test]
    fn target_child_modules_are_reachable() {
        let _ = std::any::TypeId::of::<target::OptimizationTarget>();
        let _ = std::any::TypeId::of::<gate_set::TargetGateSet>();
        let _ = std::any::TypeId::of::<constraints::TargetConstraints>();
        let _ = std::any::TypeId::of::<profiles::TargetProfile>();
    }
}