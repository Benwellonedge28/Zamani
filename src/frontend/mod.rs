
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Native Frontend
//!
//! File: `src/frontend/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module is the composition root for the native Zamani frontend.
//!
//! It exposes the canonical AST, lexer, parser, and semantic-analysis
//! subsystems through one public module boundary. Each subsystem retains
//! ownership of its implementation, internal module graph, diagnostics,
//! configuration, and lifecycle.
//!
//! This file coordinates module composition. It does not implement a
//! competing lexer, parser, AST, semantic analyzer, or intermediate
//! representation.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! The frontend MUST conform to the repository's language and architecture
//! specifications, including:
//!
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/specification/grammar-authority.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/semantics.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//!
//! These authorities have distinct responsibilities:
//!
//! - Normative specifications define the language's required meaning.
//! - The canonical grammar defines the executable syntax composition.
//! - The AST represents source structure and identity.
//! - The lexer produces canonical tokens and source spans.
//! - The parser recognizes syntax and constructs the canonical AST.
//! - Semantic analysis establishes source-level meaning and validity.
//! - Canonical IR implementations represent computations for subsequent
//!   compilation stages.
//! - Backends and runtimes determine how validated computations are realized.
//!
//! Implementation comments and planned features MUST NOT override normative
//! language specifications.
//!
//! ## OWNS
//!
//! This module owns:
//!
//! - Composition of the native frontend's top-level subsystems.
//! - The canonical Rust module path for the native frontend.
//! - A small, explicit set of frontend convenience re-exports.
//! - Frontend-level architectural and safety invariants.
//! - Module-surface integration tests.
//!
//! ## DOES_NOT_OWN
//!
//! This module does not own:
//!
//! - Lexical rules, token definitions, or token scanning.
//! - Grammar productions, precedence, parsing, or recovery.
//! - AST node definitions, source identity, or AST construction.
//! - Symbol tables, name resolution, type inference, or type checking.
//! - Effect, capability, resource, or contract analysis.
//! - Domain-specific semantic rules.
//! - Diagnostic production or diagnostic ordering algorithms.
//! - Classical, quantum, HDL, or other canonical IR implementations.
//! - Target discovery, resource allocation, placement, routing, or scheduling.
//! - Quantum physical mapping, pulse generation, or error correction.
//! - Backend code generation, deployment, or runtime execution.
//! - The legacy compiler pipeline or its legacy semantic implementation.
//!
//! Every responsibility MUST have one authoritative implementation.
//!
//! ## CANONICAL AST
//!
//! The existing native AST implementation resides under:
//!
//! `src/frontend/ast/node/`
//!
//! The `ast` facade exposes that existing implementation through:
//!
//! `crate::frontend::ast::node`
//!
//! The path attribute below is intentional: it references the existing AST
//! module without copying its implementation or introducing a competing
//! AST representation.
//!
//! AST declarations, expressions, statements, types, node identities,
//! metadata, and source locations remain owned by their canonical modules.
//!
//! ## LEXER
//!
//! `crate::frontend::lexer` owns canonical lexical analysis.
//!
//! Its public contracts define token production, token kinds, token metadata,
//! source spans, lexical diagnostics, dialect behavior, and scanner policy.
//!
//! The parser MUST consume the canonical frontend token representation.
//! Legacy token types MUST NOT be mixed with native token types without an
//! explicit compatibility adapter.
//!
//! ## PARSER
//!
//! `crate::frontend::parser` owns grammar recognition, parser configuration,
//! parser sessions, AST adaptation, recovery, and parse-result reporting.
//!
//! Successful parsing does not imply semantic validity.
//!
//! The parser MUST preserve relevant source information, distinguish complete
//! parsing from incomplete or recovered parsing, and report operational
//! failures explicitly.
//!
//! This composition root MUST NOT redeclare parser-owned child modules.
//!
//! ## SEMANTIC ANALYSIS
//!
//! `crate::frontend::semantic` is the semantic-analysis boundary for the
//! native frontend AST.
//!
//! It owns the semantic API, configuration, context, sessions, pipeline,
//! semantic model, symbol resolution, types, effects, capabilities, resources,
//! contracts, domain validation, diagnostics, traversal, and incremental
//! analysis.
//!
//! The frontend exposes the semantic service and result contracts without
//! reimplementing them.
//!
//! Incomplete, aborted, or failed analysis MUST NOT be reported as a
//! completed, valid semantic result.
//!
//! ## COMPUTATIONAL DOMAINS
//!
//! Zamani is one language with shared semantic foundations.
//!
//! The frontend must support the integration of classical, quantum, HDL,
//! hybrid, AI/ML, tensor, concurrent, distributed, networking, and future
//! computational domains through the owning AST, parser, and semantic
//! extension contracts.
//!
//! This module MUST NOT introduce an exhaustive central list of every
//! operation, device, backend, or computational domain.
//!
//! Domain-specific behavior belongs to the relevant subsystem.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! The architectural objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! The frontend describes source meaning independently of unnecessary
//! assumptions about the physical machine that will execute the program.
//!
//! This module MUST NOT introduce language-level ceilings for:
//!
//! - Qubits, quantum operations, or quantum resources.
//! - CPUs, cores, threads, GPUs, FPGAs, ASICs, or other accelerators.
//! - Memory, storage, registers, or machine widths.
//! - Tensor dimensions, declarations, generic parameters, or functions.
//! - Processes, distributed nodes, devices, or topology size.
//! - The number of computational domains or registered extensions.
//!
//! Symbolic requirements and constraints MUST remain representable until
//! the owning semantic or downstream stage is authorized to evaluate them.
//!
//! Operational budgets, where needed, belong to the lexer, parser, semantic
//! analyzer, or other stage that owns the relevant work. Such budgets are
//! explicit implementation policies, not universal language constraints.
//!
//! Budget exhaustion MUST be reported through the owning subsystem's
//! established error or incomplete-result contract. It MUST NOT silently
//! truncate input or certify incomplete analysis as valid.
//!
//! No finite implementation can process literally infinite input. The
//! scalability objective is to avoid arbitrary universal hardware ceilings
//! and to scale with available resources, representable values, supported
//! semantics, and explicit operational policies.
//!
//! ## IR AND REALIZATION BOUNDARIES
//!
//! Semantic analysis determines source-level meaning without committing to
//! a particular physical target.
//!
//! The canonical quantum IR remains `quantum::ir`. Classical, HDL, and other
//! domain representations remain owned by their respective IR subsystems.
//!
//! This frontend MUST NOT allocate physical qubits, choose devices, assign
//! hardware addresses, schedule execution, route operations, generate
//! pulses, or create a competing intermediate representation.
//!
//! ## LEGACY COMPATIBILITY
//!
//! The existing `crate::semantic`, `crate::parser`, and `crate::lexer`
//! modules belong to the legacy compiler architecture.
//!
//! Their public interfaces MUST NOT be silently replaced by this frontend.
//! Compatibility adapters and compiler-entry-point migration must be
//! implemented and tested in the relevant owning modules.
//!
//! The native frontend and legacy pipeline may coexist during migration,
//! but must not become competing permanent authorities for the same
//! source-language semantics.
//!
//! ## DEPENDENCY DIRECTION
//!
//! The intended dependency direction is:
//!
//! source text
//!     -> frontend::lexer
//!     -> frontend::parser
//!     -> frontend::ast::node
//!     -> frontend::semantic
//!     -> canonical domain IRs
//!     -> lowering and target realization
//!
//! The arrows describe conceptual processing and ownership boundaries;
//! they do not require the AST to depend on the parser or lexer.
//!
//! Lower-level source and AST infrastructure MUST NOT depend on semantic
//! analysis, IR implementations, backends, or runtime execution.
//!
//! ## FREEZE CRITERIA
//!
//! This file may be frozen when:
//!
//! 1. The crate root exposes `frontend` exactly once.
//! 2. The AST facade resolves to the existing canonical AST.
//! 3. The lexer and parser share the canonical token and source contracts.
//! 4. Every declared subsystem and its public API compiles.
//! 5. The semantic facade exports the documented API.
//! 6. No child module is declared twice.
//! 7. The legacy pipeline remains compatible until migration is verified.
//! 8. The dependency direction and IR boundaries are preserved.
//! 9. No unsafe code or artificial hardware-capacity ceiling is introduced.
//! 10. Formatting, compilation, tests, and applicable lint checks pass.
//!
//! Compilation of this file alone does not establish production readiness
//! for every frontend feature or computational domain.

// =============================================================================
// Canonical AST facade
// =============================================================================
//
// Reuse the existing AST implementation. Do not define a second Program,
// Node, NodeId, Span, expression, statement, declaration, or type model.

pub mod ast {
    #[path = "ast/node/mod.rs"]
    pub mod node;
}

// =============================================================================
// Native frontend subsystems
// =============================================================================
//
// Each subsystem owns its internal module tree, public API, and implementation.
// Declare each top-level subsystem exactly once.

/// Canonical lexical analysis, tokens, spans, and lexical diagnostics.
pub mod lexer;

/// Canonical parsing, grammar integration, recovery, and AST adaptation.
pub mod parser;

/// Canonical target-independent semantic analysis.
pub mod semantic;

// =============================================================================
// Convenience exports
// =============================================================================
//
// Re-export canonical definitions instead of introducing wrapper types.
// Subsystem-specific APIs remain available through their owning modules.

/// Canonical source-level program type.
pub use ast::node::program::Program;

/// Public semantic-analysis entry point and lifecycle contracts.
pub use semantic::{
    analyze,
    SemanticApiError,
    SemanticDiagnostic,
    SemanticModelState,
    SemanticResult,
    SemanticService,
    SemanticStatus,
    SEMANTIC_API_VERSION,
};

// =============================================================================
// Frontend identity
// =============================================================================

/// Stable architectural identifier for the native frontend.
pub const FRONTEND_MODULE_NAME: &str = "zamani.frontend";

/// Version of the frontend module-composition contract.
///
/// This is independent of the Zamani language edition, lexer version, parser
/// version, AST schema, semantic API, and compiler package version.
pub const FRONTEND_MODULE_SCHEMA_VERSION: u16 = 1;

/// Return the stable frontend module identifier.
#[must_use]
pub const fn module_name() -> &'static str {
    FRONTEND_MODULE_NAME
}

/// Return the frontend module-composition contract version.
#[must_use]
pub const fn schema_version() -> u16 {
    FRONTEND_MODULE_SCHEMA_VERSION
}

// =============================================================================
// Integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontend_module_identity_is_stable() {
        assert_eq!(module_name(), "zamani.frontend");
        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn canonical_program_type_is_exported() {
        // Compile-time API check: the public Program alias must resolve to
        // the existing canonical AST type.
        fn accepts_program(_: core::marker::PhantomData<Program>) {}

        accepts_program(core::marker::PhantomData);
    }

    #[test]
    fn semantic_api_is_reachable_through_frontend() {
        // Compile-time API check: the public service contract remains
        // available from the frontend composition root.
        fn accepts_service<T: SemanticService + ?Sized>() {}

        accepts_service::<dyn SemanticService>();
    }
}
