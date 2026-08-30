//! Zamani Quantum Optimization — Structural Optimization
//!
//! Production-grade structural optimization namespace for the Zamani quantum
//! compiler.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir::QuantumCircuit
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ▼
//! optimization::structure
//!      │
//!      ├── block
//!      ├── region
//!      ├── r#loop
//!      ├── conditional
//!      └── control_flow
//!      │
//!      ▼
//! optimization analyses / rewrites / synthesis
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      ▼
//! quantum::scheduling
//!      │
//!      ▼
//! quantum::hardware
//! ```
//!
//! This module is the authoritative Rust module boundary for structural
//! optimization inside `quantum::optimization`.
//!
//! It composes the structural optimization components without introducing
//! another quantum intermediate representation.
//!
//! # Canonical representation
//!
//! The canonical quantum representation remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! Structural optimization views and metadata must never replace or duplicate
//! the canonical Quantum IR.
//!
//! The structural subsystem may describe:
//!
//! - operation ranges;
//! - optimization blocks;
//! - semantic regions;
//! - loops;
//! - conditionals;
//! - control-flow relationships;
//! - optimization permissions;
//! - structural boundaries;
//! - deterministic traversal;
//! - structural analysis metadata.
//!
//! It must not become an alternative representation of quantum gates,
//! measurements, parameters, qubits, or circuit semantics.
//!
//! # Responsibilities
//!
//! This subsystem owns the structural understanding required by optimization
//! passes.
//!
//! It is responsible for identifying and describing:
//!
//! - contiguous optimization blocks;
//! - semantic regions;
//! - nested regions;
//! - loop bodies;
//! - loop-carried regions;
//! - conditional branches;
//! - classical-control boundaries;
//! - control-flow relationships;
//! - structural containment;
//! - structural ordering;
//! - optimization boundaries;
//! - optimization eligibility metadata.
//!
//! It does NOT own:
//!
//! - gate semantics;
//! - canonical quantum operations;
//! - canonical qubits;
//! - canonical parameters;
//! - circuit execution;
//! - backend communication;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - QEC implementation;
//! - benchmarking;
//! - source parsing;
//! - optimization pass orchestration;
//! - cost-model ownership;
//! - rewrite-rule ownership;
//! - semantic-equivalence proof ownership.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Why this boundary exists
//!
//! A production quantum optimizer cannot safely assume that a circuit is an
//! unrestricted linear sequence of gates.
//!
//! Zamani quantum programs may eventually contain:
//!
//! - ordinary gate sequences;
//! - measurements;
//! - resets;
//! - barriers;
//! - classical dependencies;
//! - conditionals;
//! - loops;
//! - nested control flow;
//! - compiler-generated protected regions;
//! - logical-qubit regions;
//! - fault-tolerant regions;
//! - target-specific protected regions;
//! - future dynamic quantum-control constructs.
//!
//! Structural information therefore has to be available independently of any
//! particular optimization algorithm.
//!
//! For example, an optimizer may discover that:
//!
//! ```text
//! A
//! B
//! A†
//! ```
//!
//! is algebraically simplifiable, but it must not automatically move `A†`
//! across a measurement, reset, classical dependency, conditional boundary,
//! protected region, or other semantic barrier merely because the operations
//! appear adjacent in a storage representation.
//!
//! Structural modules describe the boundaries.
//!
//! Semantic-equivalence infrastructure decides whether a proposed
//! transformation is actually legal.
//!
//! Rewrite infrastructure performs the transformation.
//!
//! This separation is intentional.
//!
//! # Module dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! optimization::circuit
//!     │
//!     ▼
//! optimization::structure
//!     │
//!     ├───────────────┐
//!     │               │
//!     ▼               ▼
//! analysis        rewrite planning
//!     │               │
//!     └───────┬───────┘
//!             ▼
//!       optimization passes
//! ```
//!
//! Structural modules must not depend upward on:
//!
//! - `OptimizationPipeline`;
//! - `OptimizationPass`;
//! - `OptimizationPlanner`;
//! - `OptimizationScheduler`;
//! - routing;
//! - scheduling;
//! - hardware;
//! - runtime;
//! - benchmarking.
//!
//! This keeps the structural layer reusable by all future optimization
//! strategies.
//!
//! # Child modules
//!
//! ## `block`
//!
//! Defines the smallest optimizer-owned structural unit: a deterministic,
//! contiguous optimization block over a canonical circuit.
//!
//! It is intentionally lightweight and suitable for local optimization.
//!
//! ## `region`
//!
//! Defines semantic optimization regions and region hierarchies.
//!
//! Regions provide a structural permission boundary for transformations and
//! support nesting and deterministic containment relationships.
//!
//! ## `r#loop`
//!
//! Defines loop-aware structural information.
//!
//! Rust reserves `loop` as a language keyword, so the Rust module identifier is
//! deliberately declared as `r#loop`.
//!
//! An ergonomic `loop_` alias is provided below.
//!
//! ## `conditional`
//!
//! Defines conditional quantum-program structure and safe metadata for
//! classically controlled branches.
//!
//! Unknown predicates must remain unknown. Structural analysis must never
//! silently evaluate an unavailable predicate as true or false.
//!
//! ## `control_flow`
//!
//! Defines higher-level control-flow structure and relationships across
//! blocks, regions, loops, and conditional constructs.
//!
//! # Empty structures
//!
//! Empty blocks and regions are legitimate structural states.
//!
//! They can represent:
//!
//! - insertion points;
//! - empty branches;
//! - empty loop bodies;
//! - zero-operation regions;
//! - partially constructed compiler structures;
//! - future CFG nodes;
//! - optimization boundaries without operations.
//!
//! Whether a particular optimization transformation accepts an empty
//! structure is the responsibility of that transformation.
//!
//! This module must not globally reject empty structures merely for
//! convenience.
//!
//! # Nesting
//!
//! Structural nesting must be representable without imposing a fixed maximum
//! nesting depth at the type level.
//!
//! Actual resource limits belong to the optimizer's resource-limit subsystem.
//!
//! Therefore this module does not introduce arbitrary constants such as:
//!
//! ```text
//! MAX_BLOCKS
//! MAX_REGIONS
//! MAX_NESTING
//! MAX_LOOP_DEPTH
//! ```
//!
//! Such constants would create artificial limits that conflict with the
//! requirement that Zamani scale from tiny programs to the largest programs
//! permitted by available memory, CPU time, and configured compiler budgets.
//!
//! Resource exhaustion must instead be handled by the surrounding optimization
//! limit system.
//!
//! # Scaling
//!
//! Structural objects are expected to remain lightweight.
//!
//! In particular:
//!
//! - blocks should represent ranges rather than clone operations;
//! - regions should represent ranges and relationships rather than duplicate
//!   the circuit;
//! - structural metadata should be deterministic;
//! - traversal should be lazy where practical;
//! - large structures must not require quadratic copying;
//! - no component may assume that circuit size fits in a small fixed bound;
//! - overflow must be handled explicitly by child implementations.
//!
//! The practical upper bound is therefore determined by:
//!
//! ```text
//! available resources
//! + canonical IR limits
//! + optimization limits
//! + allocator capacity
//! + configured compilation budgets
//! ```
//!
//! rather than by an arbitrary structural-module limit.
//!
//! # Determinism
//!
//! Structural analysis must be deterministic when the optimizer is operating
//! in deterministic mode.
//!
//! The structural namespace therefore does not own or implicitly create:
//!
//! - random-number generators;
//! - global mutable counters;
//! - process-global structural IDs;
//! - time-dependent ordering;
//! - hash-map iteration as a semantic ordering mechanism.
//!
//! Invocation-local identifiers are preferred.
//!
//! If deterministic ordering is required, callers should use explicit ordering
//! rules such as:
//!
//! 1. canonical operation position;
//! 2. structural start position;
//! 3. structural end position;
//! 4. structural kind;
//! 5. invocation-local identifier.
//!
//! # Mutation boundary
//!
//! Structural modules are planning/description infrastructure.
//!
//! They must not bypass the canonical circuit mutation boundary.
//!
//! The intended flow is:
//!
//! ```text
//! canonical QuantumCircuit
//!          │
//!          ▼
//! CircuitView
//!          │
//!          ▼
//! structural analysis
//!          │
//!          ▼
//! optimization plan
//!          │
//!          ▼
//! CircuitEditPlan / CircuitEditor
//!          │
//!          ▼
//! canonical QuantumCircuit
//! ```
//!
//! A structural object must not directly mutate canonical operations unless
//! the owning child module's contract explicitly requires it and routes the
//! mutation through the canonical optimizer editing abstraction.
//!
//! # Semantic safety
//!
//! Structural boundaries are not themselves proofs of equivalence.
//!
//! A region marked as optimization-safe does not mean that every rewrite
//! inside that region is mathematically valid.
//!
//! The complete safety chain is:
//!
//! ```text
//! structural eligibility
//!        │
//!        ▼
//! rewrite preconditions
//!        │
//!        ▼
//! semantic equivalence
//!        │
//!        ▼
//! canonical IR validation
//! ```
//!
//! This prevents structural analysis from accidentally becoming a substitute
//! for the optimizer's semantic verification system.
//!
//! # Measurements and classical control
//!
//! Measurements must be treated as potential semantic boundaries.
//!
//! Structural analysis must preserve the possibility of:
//!
//! ```text
//! quantum operation
//!        │
//!        ▼
//! measurement
//!        │
//!        ▼
//! classical value
//!        │
//!        ▼
//! conditional quantum operation
//! ```
//!
//! A structural optimizer must therefore never assume that two operations
//! separated by a measurement are freely reorderable.
//!
//! Similarly, a conditional branch must retain its predicate relationship to
//! the operations guarded by that predicate.
//!
//! # Loops
//!
//! Loop optimization must distinguish at least:
//!
//! - statically known iteration counts;
//! - dynamically determined iteration counts;
//! - loop-invariant classical information;
//! - loop-carried classical dependencies;
//! - loop-carried quantum state;
//! - measurement-dependent termination;
//! - side-effecting quantum operations.
//!
//! A loop body must never be duplicated, removed, reordered, or hoisted solely
//! because it looks structurally repetitive.
//!
//! Any such transformation requires a semantic proof from the appropriate
//! optimization infrastructure.
//!
//! # Conditionals
//!
//! Conditional optimization must preserve:
//!
//! - predicate identity;
//! - predicate operand ordering;
//! - branch association;
//! - branch-local semantics;
//! - classical dependencies;
//! - measurement-to-control dependencies;
//! - nested branch structure.
//!
//! Unknown predicates must remain unknown.
//!
//! The structural subsystem must not execute classical expressions as part of
//! analysis.
//!
//! # Control flow
//!
//! `control_flow` is the highest-level structural component in this directory.
//!
//! Its role is composition, not ownership of all quantum semantics.
//!
//! It may construct relationships such as:
//!
//! ```text
//! entry
//!   │
//!   ▼
//! block
//!   │
//!   ├──── conditional ────┐
//!   │                     │
//!   ▼                     ▼
//! then region          else region
//!   │                     │
//!   └──────────┬──────────┘
//!              ▼
//!             loop
//!              │
//!              ▼
//!             exit
//! ```
//!
//! The underlying quantum operations remain owned by `quantum::ir`.
//!
//! # Integration with optimization
//!
//! The future optimizer hierarchy is expected to consume this namespace as
//! follows:
//!
//! ```text
//! OptimizationContext
//!       │
//!       ▼
//! CircuitView
//!       │
//!       ▼
//! structure
//!       │
//!       ├── Block
//!       ├── Region
//!       ├── Loop
//!       ├── Conditional
//!       └── ControlFlow
//!       │
//!       ▼
//! Analysis
//!       │
//!       ▼
//! Rewrite / synthesis
//! ```
//!
//! Structural modules therefore do not need to be modified merely because a
//! new optimization pass is added.
//!
//! New passes should consume the stable structural API instead.
//!
//! # Integration with analysis
//!
//! The analysis subsystem may consume structural information for:
//!
//! - dependency analysis;
//! - qubit liveness;
//! - critical-path analysis;
//! - commutation analysis;
//! - control-flow analysis;
//! - region-local gate counts;
//! - optimization eligibility;
//! - loop-carried dependency detection;
//! - branch-local analysis.
//!
//! Structural modules must not depend on those analyses merely to exist.
//!
//! This one-way dependency prevents cycles such as:
//!
//! ```text
//! structure → analysis → structure
//! ```
//!
//! unless a future explicit abstraction layer is introduced.
//!
//! # Integration with rewrite
//!
//! Rewrites may consume structural objects to establish that a candidate
//! transformation is located inside an eligible region.
//!
//! Rewrites remain responsible for:
//!
//! - matching;
//! - preconditions;
//! - replacement construction;
//! - semantic preservation;
//! - cost evaluation;
//! - provenance;
//! - invalidation of affected analyses.
//!
//! Structural modules do not perform these responsibilities.
//!
//! # Integration with routing
//!
//! This module must remain independent of routing.
//!
//! Correct dependency direction:
//!
//! ```text
//! optimization::structure
//!          │
//!          ▼
//! logical optimization
//!          │
//!          ▼
//! routing
//! ```
//!
//! Structural optimization must not inspect or mutate physical topology.
//!
//! A future target-aware optimizer may receive target information through the
//! optimizer's target abstraction, but that target information must not turn
//! this structural namespace into a hardware subsystem.
//!
//! # Integration with scheduling
//!
//! Scheduling consumes the result of logical/physical compilation.
//!
//! Structural optimization may expose enough information for scheduling-aware
//! cost estimation, such as:
//!
//! - region boundaries;
//! - dependency boundaries;
//! - control-flow structure;
//! - critical-path candidates.
//!
//! It must not schedule operations itself.
//!
//! # Integration with error correction
//!
//! Fault-tolerant optimization may use structural regions to identify logical
//! blocks, protected sequences, or QEC-related compiler boundaries.
//!
//! However, this namespace must not implement:
//!
//! - QEC codes;
//! - syndrome extraction;
//! - decoding;
//! - logical-qubit construction;
//! - physical QEC layouts.
//!
//! Those remain owned by `quantum::error_correction`.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes optimization results and may inspect structural
//! statistics.
//!
//! This module must not depend on benchmarking.
//!
//! Correct direction:
//!
//! ```text
//! structure
//!     │
//!     ▼
//! optimization
//!     │
//!     ▼
//! benchmarking
//! ```
//!
//! or, more precisely, benchmarking independently consumes the resulting
//! circuit and optimization metadata.
//!
//! # Integration with frontends
//!
//! Frontends must lower into the canonical Quantum IR.
//!
//! They must not construct optimizer-owned structures as a substitute for
//! canonical IR.
//!
//! Correct direction:
//!
//! ```text
//! source / OpenQASM / external format
//!                │
//!                ▼
//!          quantum::frontend
//!                │
//!                ▼
//!          quantum::ir
//!                │
//!                ▼
//!       quantum::optimization
//!                │
//!                ▼
//!       optimization::structure
//! ```
//!
//! # Integration with algorithms
//!
//! Algorithm modules may construct canonical circuits that later become
//! structurally optimized.
//!
//! They should not depend on a particular structural optimization
//! implementation merely to construct a circuit.
//!
//! Correct direction:
//!
//! ```text
//! quantum::algorithms
//!          │
//!          ▼
//! quantum::ir
//!          │
//!          ▼
//! quantum::optimization
//!          │
//!          ▼
//! structure
//! ```
//!
//! # Integration with the current repository
//!
//! The repository currently contains the following structural implementation
//! files:
//!
//! ```text
//! src/quantum/optimization/structure/
//! ├── block.rs
//! ├── conditional.rs
//! ├── control_flow.rs
//! ├── loop.rs
//! └── region.rs
//! ```
//!
//! This module intentionally declares exactly those existing implementation
//! files.
//!
//! No speculative child file is declared here.
//!
//! This is important because Rust module declarations are part of the
//! compilation graph: declaring a future file before it exists would break the
//! build.
//!
//! When a new structural implementation is introduced, it should first be
//! completed as an independent file and then added here as a single module
//! declaration.
//!
//! # Rust keyword handling
//!
//! `loop.rs` is a valid source filename but `loop` is a Rust keyword.
//!
//! Therefore the correct declaration is:
//!
//! ```rust
//! pub mod r#loop;
//! ```
//!
//! This preserves the existing filename while remaining valid Rust.
//!
//! An ergonomic alias is provided:
//!
//! ```rust
//! pub use r#loop as loop_;
//! ```
//!
//! Both paths refer to the same implementation and do not duplicate code.
//!
//! # Public API policy
//!
//! This file intentionally exposes child modules rather than blindly glob
//! re-exporting every symbol from every implementation.
//!
//! That policy is important for long-term API stability.
//!
//! A child implementation can evolve internally without forcing every public
//! type into the stable `structure` namespace.
//!
//! The preferred paths are therefore:
//!
//! ```text
//! quantum::optimization::structure::block
//! quantum::optimization::structure::region
//! quantum::optimization::structure::r#loop
//! quantum::optimization::structure::loop_
//! quantum::optimization::structure::conditional
//! quantum::optimization::structure::control_flow
//! ```
//!
//! Higher-level stable type re-exports should only be added when a type has
//! been deliberately selected as part of the optimizer's public API.
//!
//! # No unsafe code
//!
//! This module explicitly forbids unsafe code.
//!
//! Structural optimization has no requirement for `unsafe` Rust.
//!
//! ```text
//! unsafe code: forbidden
//! FFI: not owned here
//! raw pointers: not required
//! global mutable state: forbidden
//! ```
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
//! No unstable compiler APIs are required.
//!
//! No external dependency is required merely to compose this module.
//!
//! # Compile-time invariants
//!
//! This module establishes the following invariants:
//!
//! 1. The canonical Quantum IR remains outside this namespace.
//! 2. All five structural implementation files have one authoritative module
//!    declaration.
//! 3. `loop.rs` is declared using the Rust raw identifier syntax.
//! 4. No structural child is duplicated through an inline module.
//! 5. No structural child is conditionally selected based on runtime state.
//! 6. No structural child owns global mutable state through this module.
//! 7. No backend or hardware dependency is introduced here.
//! 8. No benchmark dependency is introduced here.
//! 9. No routing dependency is introduced here.
//! 10. No scheduling dependency is introduced here.
//! 11. Structural APIs remain reusable by future optimization passes.
//!
//! # Future extension rule
//!
//! If future structural capabilities are required, add them as independent
//! modules first.
//!
//! For example:
//!
//! ```text
//! structure/
//! ├── block.rs
//! ├── conditional.rs
//! ├── control_flow.rs
//! ├── loop.rs
//! ├── region.rs
//! ├── dataflow.rs          ← future independent module
//! ├── transaction.rs       ← future independent module
//! └── mod.rs
//! ```
//!
//! The new file should first establish its complete independent contract.
//! Only then should this module receive one additional declaration:
//!
//! ```rust
//! pub mod dataflow;
//! ```
//!
//! This preserves the project's requested workflow in which an individual
//! implementation can be completed before integration.
//!
//! # Testing strategy
//!
//! Structural unit tests belong in their respective child files.
//!
//! This module owns only integration/smoke tests concerning module composition.
//!
//! The integration tests here deliberately avoid depending on implementation
//! details or particular public type names. That prevents this module from
//! becoming coupled to internal APIs and therefore avoids future re-editing
//! merely because an implementation evolves.
//!
//! More comprehensive tests belong under the optimization test subsystem and
//! should cover:
//!
//! - nested region construction;
//! - deterministic ordering;
//! - empty structures;
//! - overflow handling;
//! - malformed structural input;
//! - conditional predicate preservation;
//! - loop-carried dependencies;
//! - control-flow consistency;
//! - interaction with canonical IR validation;
//! - interaction with rewrite planning;
//! - semantic equivalence after transformations.
//!
//! # Complexity policy
//!
//! This module itself performs no circuit-wide structural analysis.
//!
//! Therefore module composition is O(1).
//!
//! Complexity of individual operations belongs to the corresponding child
//! implementation.
//!
//! Child modules must document complexity for operations that traverse or
//! construct structural collections.
//!
//! No child may silently introduce an O(n²) algorithm where an O(n log n) or
//! O(n) deterministic implementation is reasonably available for the same
//! contract.
//!
//! # Resource policy
//!
//! "Scale to infinity given resources" is interpreted as follows:
//!
//! - no arbitrary small structural limits;
//! - no fixed circuit-size assumptions;
//! - no fixed qubit-count assumptions;
//! - no fixed nesting assumptions;
//! - no integer arithmetic that intentionally truncates large structures;
//! - explicit overflow handling in child implementations;
//! - configurable resource limits owned by the optimizer limit subsystem;
//! - graceful failure or bounded degradation when configured limits are
//!   exceeded.
//!
//! No software can literally process an infinite circuit because memory and
//! execution time are finite. The architecture therefore scales until the
//! available resources or explicit compiler limits are exhausted.
//!
//! # Summary
//!
//! This module is deliberately small in executable behavior and large in
//! architectural guarantees.
//!
//! Its job is to establish one stable structural namespace:
//!
//! ```text
//! quantum::optimization::structure
//! ```
//!
//! containing:
//!
//! ```text
//! block
//! region
//! r#loop
//! conditional
//! control_flow
//! ```
//!
//! The implementation details remain in those files.
//!
//! No second IR is introduced.
//! No unsafe code is required.
//! No hardware assumptions are introduced.
//! No optimization algorithm is hard-coded here.
//! No backend I/O is performed.
//! No global state is created.
//!
//! This makes the module suitable as the permanent structural boundary for
//! Zamani's quantum optimizer.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Structural optimization modules
// =============================================================================

/// Deterministic contiguous optimization blocks.
///
/// `block.rs` provides the lowest-level structural unit used by local and
/// region-aware optimization.
pub mod block;

/// Classically controlled conditional regions.
///
/// This module preserves predicate and branch structure without executing
/// classical predicates during structural analysis.
pub mod conditional;

/// Higher-level control-flow composition.
///
/// This module relates blocks, regions, loops, and conditionals without taking
/// ownership of canonical quantum semantics.
pub mod control_flow;

/// Loop-aware structural optimization.
///
/// The source file is `loop.rs`; the raw identifier is required because
/// `loop` is a Rust language keyword.
pub mod r#loop;

/// Semantic optimization regions and region hierarchies.
///
/// Regions provide deterministic structural boundaries and containment
/// relationships for optimization planning.
pub mod region;

// =============================================================================
// Ergonomic compatibility alias
// =============================================================================

/// Ergonomic identifier for the implementation in `loop.rs`.
///
/// Rust requires `r#loop` when referring to the actual module because `loop`
/// is a reserved keyword. `loop_` is provided as a stable, readable alias.
///
/// Both names refer to the same module; no implementation is duplicated.
pub use r#loop as loop_;

// =============================================================================
// Controlled structural prelude
// =============================================================================

/// Stable structural-module prelude.
///
/// This prelude exposes module boundaries rather than re-exporting every
/// implementation type. That keeps the public API stable while allowing each
/// child module to evolve internally.
///
/// Typical use:
///
/// ```rust
/// use crate::quantum::optimization::structure::prelude::*;
/// ```
///
/// Then child modules remain available as:
///
/// ```text
/// block
/// conditional
/// control_flow
/// loop_
/// region
/// ```
///
/// The raw identifier remains available through the normal module path:
///
/// ```text
/// crate::quantum::optimization::structure::r#loop
/// ```
pub mod prelude {
    pub use super::block;
    pub use super::conditional;
    pub use super::control_flow;
    pub use super::loop_;
    pub use super::region;
}

// =============================================================================
// Structural architecture tests
// =============================================================================

#[cfg(test)]
mod tests {
    /// Compile-time/module-boundary smoke test.
    ///
    /// The test intentionally references modules rather than implementation
    /// types. This means child modules can evolve their internal APIs without
    /// requiring this file to be rewritten.
    #[test]
    fn all_structural_modules_are_composed() {
        use super::{
            block,
            conditional,
            control_flow,
            loop_,
            region,
        };

        let _ = block;
        let _ = conditional;
        let _ = control_flow;
        let _ = loop_;
        let _ = region;
    }

    /// Confirms that the raw Rust keyword module and ergonomic alias resolve to
    /// the same module namespace.
    #[test]
    fn loop_module_has_stable_alias() {
        use super::{loop_, r#loop};

        let raw: fn() = || {
            let _ = std::any::type_name::<r#loop::LoopAnalysis>();
        };

        let alias: fn() = || {
            let _ = std::any::type_name::<loop_::LoopAnalysis>();
        };

        raw();
        alias();
    }
}