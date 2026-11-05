
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Context.
//!
//! Path:
//!     src/compiler/context/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the public composition boundary for compiler contexts.
//!
//! It connects the independently owned context implementations required by
//! compilation, including:
//!
//!     compilation_context.rs
//!         Shared compilation configuration, source identity, operational
//!         budget access and cancellation.
//!
//!     compilation_state.rs
//!         Compilation lifecycle, phase transitions, stage tracking and
//!         cancellation-aware state management.
//!
//!     dependency_context.rs
//!         Typed compilation units and dependency relationships.
//!
//!     provenance_context.rs
//!         Append-only compilation provenance, transformation lineage,
//!         artifact references and immutable provenance snapshots.
//!
//! This module exposes those contracts through one stable namespace.
//!
//! It does not implement a second context, dependency graph, lifecycle,
//! cancellation mechanism or provenance ledger.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Public context-module composition.
//! - Public context API re-exports.
//! - The integration boundary between context subsystems.
//! - Documentation of context ownership and dependency direction.
//!
//! It does NOT own:
//!
//! - Compiler configuration.
//! - Compilation budgets.
//! - Source-map storage.
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership or borrow analysis.
//! - Effects.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Canonical IR.
//! - Optimization.
//! - Target discovery.
//! - Hardware allocation.
//! - Backend selection.
//! - Code generation.
//! - Artifact serialization.
//! - Incremental invalidation.
//! - Build-cache storage.
//! - Runtime execution.
//! - Distributed scheduling.
//!
//! Those responsibilities remain with their existing authoritative modules.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! The intended dependency direction is:
//!
//!     compiler::configuration
//!                |
//!                v
//!     compilation_context
//!                |
//!          +-----+------+
//!          |            |
//!          v            v
//!     compilation_   dependency_
//!       state         context
//!          |            |
//!          +-----+------+
//!                |
//!                v
//!        provenance_context
//!
//! The context modules may use lower-level configuration contracts.
//!
//! Provenance may refer to compilation identity, source identity,
//! dependency-unit identity, compilation state and opaque artifact metadata.
//!
//! The dependency graph does not own provenance.
//!
//! The lifecycle tracker does not own provenance.
//!
//! The compilation context does not own semantic meaning.
//!
//! No context module may introduce a reverse dependency on compiler
//! orchestration, backend implementations or runtime execution.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module imposes no universal limits on:
//!
//! - CPUs, cores or workers;
//! - GPUs, FPGAs, ASICs or QPUs;
//! - accelerators or processing elements;
//! - nodes, devices or distributed participants;
//! - memory or storage capacity;
//! - qubits or logical qubits;
//! - tensor dimensions or tensor rank;
//! - modules, compilation units or dependencies;
//! - provenance events or lineage relationships;
//! - artifacts or transformations;
//! - compilation stages.
//!
//! This module does not introduce MAX_* constants or arbitrary collection
//! limits.
//!
//! Actual operational limits belong to explicitly configured budgets,
//! available host resources, storage capacity and the relevant target.
//!
//! The absence of a configured budget does not promise infinite physical
//! resources.
//!
//! Collection growth and allocation failures must be handled by the owning
//! implementation and must not be converted into silent truncation.
//!
//! ============================================================================
//! CONCURRENCY AND SHARING
//! ============================================================================
//!
//! The child modules define their own sharing and synchronization contracts.
//!
//! In particular:
//!
//! - CompilationContext shares compilation configuration, budget accounting
//!   and cancellation state.
//! - CompilationStateTracker owns lifecycle synchronization.
//! - DependencyContext owns its dependency snapshot and builder.
//! - ProvenanceContext owns its synchronized provenance ledger.
//!
//! This facade creates no global mutable state and starts no worker threads.
//!
//! Cloning a child context follows that child's documented sharing semantics.
//!
//! ============================================================================
//! GRAMMAR AND FRONTEND BOUNDARY
//! ============================================================================
//!
//! Source-level compilation provenance syntax is owned by:
//!
//!     grammar/compile/provenance.g4
//!
//! Compilation syntax composition is owned by:
//!
//!     grammar/compile/compile.g4
//!
//! The canonical parser composition boundary is:
//!
//!     grammar/antlr/ZamaniParser.g4
//!
//! The complete-language grammar root is:
//!
//!     grammar/Zamani.g4
//!
//! Normative provenance semantics are specified by:
//!
//!     grammar/spec/provenance.md
//!
//! The frontend owns parsing and AST construction.
//!
//! Semantic analysis owns the meaning and validation of source-level
//! provenance declarations.
//!
//! This module owns runtime compiler-context bookkeeping only.
//!
//! It must not parse grammar productions, construct a second AST or interpret
//! source-level provenance semantics.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Parent:
//!
//!     src/compiler/mod.rs
//!
//! Required declarations:
//!
//!     pub mod configuration;
//!     pub mod context;
//!
//! Existing crate entry:
//!
//!     src/lib.rs
//!
//! The existing `pub mod compiler;` declaration is the crate-level entry.
//! No additional context declaration in src/lib.rs is required.
//!
//! Incremental compilation:
//!
//!     src/compiler/incremental.rs
//!
//! Owns fingerprints, cache decisions and invalidation.
//! It may consume stable provenance references and externally calculated
//! digests. It must not delegate fingerprint ownership to this module.
//!
//! Diagnostics:
//!
//!     src/compiler/diagnostics.rs
//!
//! Owns structured diagnostics.
//! Source-map references in provenance remain opaque references; this module
//! does not introduce a competing source-span representation.
//!
//! Parallel compilation:
//!
//!     src/compiler/parallel_build.rs
//!
//! Owns parallel build execution.
//! It may use dependency snapshots and merge provenance snapshots, but this
//! module does not schedule workers.
//!
//! Linking:
//!
//!     src/compiler/linker.rs
//!
//! Owns symbol resolution and linking.
//! It may record provenance events and artifact references.
//!
//! Compiler orchestration:
//!
//!     src/compiler/mod.rs
//!
//! Owns the public compiler entry points and compilation orchestration.
//! It consumes these context APIs but must not duplicate their state.
//!
//! Semantic analysis:
//!
//!     src/frontend/semantic/
//!     src/semantic.rs
//!
//! Remains authoritative for semantic meaning. Provenance references to
//! semantic contracts are opaque identifiers, not replacements for semantic
//! validation.
//!
//! Canonical representations:
//!
//!     src/zuir/
//!     src/classical/
//!     src/quantum/ir/
//!     src/hdl/hdlir/
//!     src/ai/
//!
//! Remain authoritative for their respective representations.
//!
//! Backend and runtime:
//!
//!     src/backend/
//!     src/runtime/
//!
//! Remain authoritative for target-specific code generation and execution.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! Errors remain owned by their respective child modules:
//!
//!     ContextError
//!         Compilation-context and cancellation failures.
//!
//!     CompilationStateError
//!         Invalid lifecycle or phase transitions.
//!
//!     DependencyGraphError
//!         Invalid compilation units or dependency relationships.
//!
//!     ProvenanceError
//!         Invalid provenance metadata, lineage conflicts, ledger failures
//!         and snapshot merge failures.
//!
//! This facade does not flatten structured errors into strings.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! The original module-qualified paths remain available:
//!
//!     compiler::context::compilation_context
//!     compiler::context::compilation_state
//!     compiler::context::dependency_context
//!     compiler::context::provenance_context
//!
//! Re-exports provide convenient access without moving ownership or removing
//! the original paths.
//!
//! ============================================================================
//! MODULE COMPOSITION
//! ============================================================================

/// Shared compilation configuration, source identity, budget access
/// and cancellation.
pub mod compilation_context;

/// Compilation lifecycle, phase transitions and stage tracking.
pub mod compilation_state;

/// Typed compilation units and dependency relationships.
pub mod dependency_context;

/// Append-only compilation provenance and transformation lineage.
pub mod provenance_context;

// ============================================================================
// PUBLIC COMPILATION CONTEXT API
// ============================================================================

pub use compilation_context::{
    CancellationToken,
    CompilationContext,
    CompilationContextBuilder,
    ContextError,
    SourceIdentity,
    SourceIdentityError,
    SourceOrigin,
};

// ============================================================================
// PUBLIC COMPILATION STATE API
// ============================================================================

pub use compilation_state::{
    CompilationLifecycle,
    CompilationPhase,
    CompilationStateError,
    CompilationStateSnapshot,
    CompilationStateTracker,
};

// ============================================================================
// PUBLIC DEPENDENCY CONTEXT API
// ============================================================================

pub use dependency_context::{
    CompilationUnit,
    CompilationUnitId,
    CompilationUnitKind,
    DependencyContext,
    DependencyContextBuilder,
    DependencyEdge,
    DependencyGraphError,
    DependencyKind,
};

// ============================================================================
// PUBLIC PROVENANCE CONTEXT API
// ============================================================================

pub use provenance_context::{
    ArtifactKind,
    CompilationProvenanceId,
    ContentDigest,
    ProvenanceActor,
    ProvenanceArtifactReference,
    ProvenanceContext,
    ProvenanceContextBuilder,
    ProvenanceDeterminism,
    ProvenanceError,
    ProvenanceEvent,
    ProvenanceEventId,
    ProvenanceEventKind,
    ProvenanceOutcome,
    ProvenanceProperty,
    ProvenanceSnapshot,
    ProvenanceSourceReference,
    ProvenanceTargetReference,
    SemanticContractReference,
    SourceMapReference,
    PROVENANCE_CONTEXT_API_VERSION,
};

// ============================================================================
// FACADE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_modules_are_exposed_through_the_facade() {
        let cancellation = CancellationToken::new();

        assert!(!cancellation.is_cancelled());

        let state = CompilationStateTracker::new();

        assert!(!state.is_cancelled());

        let dependencies = DependencyContext::new();

        assert!(dependencies.is_empty());
    }

    #[test]
    fn provenance_api_is_exposed_through_the_facade() {
        let compilation_id =
            CompilationProvenanceId::new("compilation:facade-test")
                .expect("valid compilation identifier");

        let provenance = ProvenanceContext::new(compilation_id);

        assert_eq!(
            provenance.event_count().expect("ledger count"),
            0
        );

        assert_eq!(PROVENANCE_CONTEXT_API_VERSION, 1);
    }

    #[test]
    fn original_module_paths_remain_available() {
        let cancellation =
            compilation_context::CancellationToken::new();

        assert!(!cancellation.is_cancelled());

        let dependencies =
            dependency_context::DependencyContext::new();

        assert!(dependencies.is_empty());

        let state =
            compilation_state::CompilationStateTracker::new();

        assert!(!state.is_cancelled());
    }
}
