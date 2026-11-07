
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Context Facade.
//!
//! Path:
//!     src/compiler/context/mod.rs
//!
//! Rust:
//!     Edition: 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # 1. Purpose
//!
//! This module is the public composition boundary for the compiler's
//! compilation-context subsystem.
//!
//! It exposes four independently owned context implementations:
//!
//! - `compilation_context`
//!   Shared configuration, source identity, budget accounting and
//!   cooperative cancellation.
//!
//! - `compilation_state`
//!   Compilation lifecycle, legal state transitions, phase tracking,
//!   stage tracking and lifecycle snapshots.
//!
//! - `dependency_context`
//!   Compilation-unit identities and immutable dependency relationships.
//!
//! - `provenance_context`
//!   Compilation provenance, transformation lineage, artifact references
//!   and immutable provenance snapshots.
//!
//! This facade provides a consistent public namespace for those contracts.
//!
//! It does not introduce another context implementation, lifecycle tracker,
//! dependency graph, cancellation mechanism or provenance ledger.
//!
//! # 2. Architectural ownership
//!
//! This module owns:
//!
//! - Public context-module composition.
//! - Public context API re-exports.
//! - Public context API-version re-exports.
//! - The integration boundary between context subsystems.
//! - Documentation of context ownership and dependency direction.
//! - Facade-level integration tests.
//!
//! This module does NOT own:
//!
//! - Compiler configuration definitions.
//! - Compilation budget definitions or accounting.
//! - Source-map storage.
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Name resolution.
//! - Ownership or borrow analysis.
//! - Effect semantics.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Canonical intermediate representations.
//! - Optimization.
//! - Target discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Code generation.
//! - Artifact payloads or serialization.
//! - Incremental compilation.
//! - Compilation scheduling.
//! - Runtime execution.
//! - Distributed execution.
//! - Quantum routing.
//! - Quantum error correction.
//!
//! Those responsibilities remain with their authoritative repository
//! subsystems.
//!
//! # 3. Canonical context authorities
//!
//! The child modules are the authoritative owners of their respective APIs.
//!
//! `compilation_context`:
//!     CompilationContext
//!     CompilationContextBuilder
//!     CancellationToken
//!     SourceIdentity
//!     ContextError
//!
//! `compilation_state`:
//!     CompilationStateTracker
//!     CompilationStateSnapshot
//!     CompilationLifecycle
//!     CompilationPhase
//!     CompilationStateError
//!
//! `dependency_context`:
//!     DependencyContext
//!     DependencyContextBuilder
//!     CompilationUnit
//!     CompilationUnitId
//!     DependencyEdge
//!     DependencyKind
//!     DependencyGraphError
//!
//! `provenance_context`:
//!     ProvenanceContext
//!     ProvenanceContextBuilder
//!     ProvenanceSnapshot
//!     ProvenanceEvent
//!     ProvenanceResult
//!     ProvenanceError
//!
//! This facade re-exports these types without transferring ownership
//! or creating competing implementations.
//!
//! # 4. Dependency direction
//!
//! The intended context dependency relationship is:
//!
//!     compiler_types
//!          |
//!          v
//!     configuration
//!          |
//!          v
//!     compilation_context
//!          |
//!          +----------------------+
//!          |                      |
//!          v                      v
//!     compilation_state    dependency_context
//!          |                      |
//!          +-----------+----------+
//!                      |
//!                      v
//!              provenance_context
//!
//! The diagram describes conceptual integration, not a requirement that
//! every child module directly imports every preceding module.
//!
//! Actual dependencies are defined by the child implementations.
//!
//! In particular:
//!
//! - CompilationContext owns shared configuration, budget accounting
//!   and cancellation.
//! - CompilationStateTracker owns lifecycle synchronization.
//! - DependencyContext owns compilation-unit relationships.
//! - ProvenanceContext records lineage and may reference source identity,
//!   dependencies, cancellation and lifecycle information.
//!
//! The facade itself must not introduce dependency cycles.
//!
//! No context module may introduce a reverse dependency on pipeline
//! orchestration, backend implementations or runtime execution.
//!
//! # 5. POCO-REAF and scalability
//!
//! Zamani's portability objective is:
//!
//!     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! The context subsystem must remain independent of temporary physical
//! machine boundaries.
//!
//! This facade imposes no universal limits on:
//!
//! - CPUs, cores or workers;
//! - GPUs, FPGAs, ASICs or QPUs;
//! - Accelerators or processing elements;
//! - Nodes, devices or distributed participants;
//! - Memory or storage capacity;
//! - Physical or logical qubits;
//! - Tensor dimensions or tensor rank;
//! - Modules or compilation units;
//! - Dependency relationships;
//! - Provenance events or lineage relationships;
//! - Artifacts or transformations;
//! - Compilation stages;
//! - Computing domains;
//! - Backend or provider registrations.
//!
//! This facade introduces no arbitrary `MAX_*` constants.
//!
//! It must not silently truncate collections or discard provenance,
//! dependencies, lifecycle information or cancellation state.
//!
//! Actual operational constraints belong to:
//!
//! - Explicitly configured compilation budgets;
//! - Available host resources;
//! - Storage capacity;
//! - Target-specific capabilities;
//! - The representation limits of the relevant implementation.
//!
//! Unlimited configured budgets do not promise infinite physical resources.
//!
//! Allocation failure, synchronization failure and representation overflow
//! must be handled by the owning implementation.
//!
//! # 6. Concurrency and sharing
//!
//! Each child module defines its own synchronization and sharing contract.
//!
//! The facade creates:
//!
//! - No global mutable state;
//! - No background workers;
//! - No implicit threads;
//! - No independent cancellation state;
//! - No additional locks around child implementations.
//!
//! Cloning a child context follows that child's documented semantics.
//!
//! In particular:
//!
//! - Cloned CompilationContext values share their documented configuration,
//!   budget-accounting and cancellation state.
//! - Cloned CompilationStateTracker values share the same synchronized
//!   lifecycle state.
//! - DependencyContext exposes an immutable dependency snapshot.
//! - Cloned ProvenanceContext values share the authoritative provenance
//!   ledger according to its implementation contract.
//!
//! The facade must not weaken those guarantees.
//!
//! # 7. Grammar and frontend boundary
//!
//! Source-level compilation syntax is owned by the grammar and frontend.
//!
//! Relevant grammar authorities include:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/compile/compile.g4
//!     grammar/compile/provenance.g4
//!     grammar/spec/provenance.md
//!
//! The grammar defines source-level compilation and provenance constructs.
//!
//! The frontend owns:
//!
//! - Lexing;
//! - Parsing;
//! - AST construction;
//! - Source locations;
//! - Structural validation;
//! - Semantic integration.
//!
//! The authoritative semantic implementation remains under:
//!
//!     src/frontend/semantic/
//!
//! This context facade represents runtime compiler-operation bookkeeping.
//!
//! It must not:
//!
//! - Parse grammar productions;
//! - Construct a second AST;
//! - Interpret source-level provenance declarations;
//! - Define source-language compilation semantics;
//! - Introduce a second source-span representation.
//!
//! Source-level provenance meaning and runtime provenance recording are
//! separate responsibilities.
//!
//! # 8. Integration contracts
//!
//! ## Parent compiler module
//!
//!     src/compiler/mod.rs
//!
//! The parent declares:
//!
//!     pub mod context;
//!
//! This facade is the public entry point for context APIs.
//!
//! No duplicate context declaration is required in `src/lib.rs`.
//!
//! ## Compiler configuration
//!
//!     src/compiler/configuration/
//!     src/compiler/compiler_types.rs
//!
//! CompilationContext consumes the canonical compiler configuration and
//! the authoritative budget implementation.
//!
//! This facade must not define replacement configuration or budget types.
//!
//! ## Compiler API and sessions
//!
//!     src/compiler/api/
//!
//! API requests and compilation sessions may own or share context values.
//!
//! Sessions must use the authoritative cancellation and lifecycle contracts.
//!
//! They must not create independent competing lifecycle state.
//!
//! ## Pipeline
//!
//!     src/compiler/pipeline/
//!
//! Pipeline execution and orchestration consume CompilationContext and
//! CompilationStateTracker.
//!
//! The context facade does not execute pipeline stages or determine their
//! ordering.
//!
//! Pipeline orchestration owns stage coordination.
//!
//! ## Diagnostics
//!
//!     src/compiler/diagnostics/
//!
//! Diagnostics own structured diagnostic representation and rendering.
//!
//! Context errors remain typed in their respective modules until a
//! consuming layer deliberately converts them into diagnostics.
//!
//! This facade must not introduce a competing diagnostic or source-span
//! representation.
//!
//! ## Dependency analysis
//!
//!     src/compiler/analysis/
//!
//! Semantic analysis discovers authoritative program relationships.
//!
//! DependencyContext stores compilation-unit relationships supplied by
//! appropriate compiler consumers.
//!
//! It does not replace semantic dependency discovery.
//!
//! ## Incremental compilation
//!
//!     src/compiler/incremental/
//!
//! Incremental compilation owns fingerprints, cache decisions,
//! invalidation and persistent cache management.
//!
//! It may consume dependency snapshots and provenance references.
//!
//! This facade does not calculate fingerprints or make cache decisions.
//!
//! ## Parallel compilation
//!
//!     src/compiler/parallel/
//!
//! Parallel compilation owns work scheduling and worker execution.
//!
//! It may consume immutable dependency snapshots and shared cancellation.
//!
//! This facade does not create workers or schedule compilation units.
//!
//! ## Linking
//!
//!     src/compiler/linking/
//!
//! Linking owns symbol resolution, composition, relocation and linking
//! validation.
//!
//! It may record relevant artifact and transformation provenance.
//!
//! This facade does not implement linking.
//!
//! ## Provenance analysis and artifacts
//!
//!     src/compiler/analysis/provenance.rs
//!     src/compiler/artifacts/
//!     src/compiler/security/
//!     src/compiler/reproducibility/
//!
//! These consumers may use provenance snapshots and typed references.
//!
//! Content hashing, cryptographic signing, artifact serialization,
//! supply-chain verification and reproducibility verification remain
//! owned by their respective subsystems.
//!
//! ## Canonical representations
//!
//!     src/zuir/
//!     src/classical/
//!     src/quantum/ir/
//!     src/hdl/hdlir/
//!     src/ai/
//!     src/hybrid/
//!     src/distributed/
//!
//! These modules remain authoritative for their respective computation
//! representations and domain semantics.
//!
//! The context facade may record opaque references to their identities,
//! versions and artifacts.
//!
//! It must not define competing representations.
//!
//! ## Backend and runtime
//!
//!     src/backend/
//!     src/runtime/
//!
//! Backend implementations own target-specific code generation.
//!
//! Runtime implementations own resource acquisition and execution.
//!
//! This facade does not select physical hardware, allocate resources,
//! generate code or execute programs.
//!
//! # 9. Error ownership
//!
//! Errors remain owned by their authoritative child modules:
//!
//! `ContextError`:
//!     Compilation-context construction, configuration integration,
//!     budget and cancellation failures.
//!
//! `CompilationStateError`:
//!     Invalid lifecycle transitions and state-tracking failures.
//!
//! `DependencyGraphError`:
//!     Invalid compilation units, relationships and dependency graphs.
//!
//! `ProvenanceError`:
//!     Invalid provenance metadata, lineage conflicts, ledger failures
//!     and snapshot merge failures.
//!
//! The facade does not flatten these errors into strings or replace
//! them with a generic context error.
//!
//! # 10. API versioning
//!
//! Each context subsystem maintains its own API version.
//!
//! These versions are independent of:
//!
//! - Zamani language version;
//! - Grammar version;
//! - Compiler release version;
//! - AST version;
//! - Semantic-model version;
//! - IR version;
//! - Backend ABI version;
//! - Runtime version.
//!
//! This facade re-exports the child API-version constants.
//!
//! An incompatible change to one child contract must be evaluated against
//! its consumers and must not be hidden by this facade.
//!
//! # 11. Compatibility
//!
//! Existing module-qualified paths remain available:
//!
//!     compiler::context::compilation_context
//!     compiler::context::compilation_state
//!     compiler::context::dependency_context
//!     compiler::context::provenance_context
//!
//! Existing facade type exports remain available.
//!
//! Additional API-version and result-type exports are additive.
//!
//! Child implementations remain the owners of their public types.
//!
//! # 12. Completion invariants
//!
//! This facade is correct only while all the following remain true:
//!
//! - Every declared child module exists.
//! - Every re-export refers to an actual public child item.
//! - Child modules remain the authoritative owners of their contracts.
//! - No duplicate context implementation is introduced.
//! - No circular dependency is introduced.
//! - No source-language semantics are implemented here.
//! - No physical resource ceilings are introduced here.
//! - No unsafe Rust is permitted.
//! - Existing module-qualified paths remain valid.
//! - Facade tests exercise the public exports.
//!
//! The facade must be updated deliberately whenever a child module's
//! public contract changes.

// ============================================================================
// MODULE COMPOSITION
// ============================================================================

/// Shared compilation configuration, source identity, budget access
/// and cooperative cancellation.
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
    COMPILATION_CONTEXT_API_VERSION,
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
    COMPILATION_STATE_API_VERSION,
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
    DEPENDENCY_CONTEXT_API_VERSION,
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
    ProvenanceResult,
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
    fn all_context_modules_are_exposed_through_the_facade() {
        let cancellation = CancellationToken::new();

        assert!(!cancellation.is_cancelled());

        let state = CompilationStateTracker::new();

        assert!(!state.is_cancelled());

        let dependencies = DependencyContext::new();

        assert!(dependencies.is_empty());
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

        let provenance_id =
            provenance_context::CompilationProvenanceId::new(
                "compilation:module-path-test",
            )
            .expect("valid provenance identifier");

        let provenance =
            provenance_context::ProvenanceContext::new(provenance_id);

        assert_eq!(
            provenance.event_count().expect("provenance event count"),
            0,
        );
    }

    #[test]
    fn provenance_api_is_exposed_through_the_facade() {
        let compilation_id =
            CompilationProvenanceId::new("compilation:facade-test")
                .expect("valid compilation identifier");

        let provenance = ProvenanceContext::new(compilation_id);

        assert_eq!(
            provenance.event_count().expect("ledger count"),
            0,
        );

        assert_eq!(PROVENANCE_CONTEXT_API_VERSION, 1);
    }

    #[test]
    fn context_api_versions_are_exposed() {
        assert_eq!(COMPILATION_CONTEXT_API_VERSION, 1);
        assert_eq!(COMPILATION_STATE_API_VERSION, 1);
        assert_eq!(DEPENDENCY_CONTEXT_API_VERSION, 1);
        assert_eq!(PROVENANCE_CONTEXT_API_VERSION, 1);
    }

    #[test]
    fn provenance_result_is_available_from_the_facade() {
        fn accepts_provenance_result<T>(
            result: ProvenanceResult<T>,
        ) -> ProvenanceResult<T> {
            result
        }

        let compilation_id =
            CompilationProvenanceId::new("compilation:result-test")
                .expect("valid provenance identifier");

        let provenance = ProvenanceContext::new(compilation_id);

        let result: ProvenanceResult<ProvenanceSnapshot> =
            provenance.snapshot();

        let result = accepts_provenance_result(result);

        assert!(result.is_ok());
    }

    #[test]
    fn cloned_context_handles_share_cancellation() {
        let original = CancellationToken::new();
        let shared = original.clone();

        assert!(!original.is_cancelled());
        assert!(!shared.is_cancelled());

        assert!(original.cancel());

        assert!(original.is_cancelled());
        assert!(shared.is_cancelled());
    }

    #[test]
    fn dependency_context_is_usable_without_pipeline_initialization() {
        let dependencies = DependencyContext::new();

        assert!(dependencies.is_empty());
        assert_eq!(dependencies.unit_count(), 0);
        assert_eq!(dependencies.edge_count(), 0);
    }
}
