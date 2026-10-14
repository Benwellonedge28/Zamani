
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Native Frontend
//!
//! File: `src/frontend/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! This module is the composition root for Zamani's native frontend.
//!
//! It connects the canonical AST, lexer, parser, and semantic-analysis
//! subsystem without duplicating their implementations or introducing
//! competing representations of source-language meaning.
//!
//! The frontend establishes a clear progression:
//!
//! ```text
//! Source text
//!     |
//!     v
//! Canonical lexer
//!     |
//!     v
//! Canonical parser
//!     |
//!     v
//! Canonical frontend AST
//!     |
//!     v
//! Semantic analysis
//!     |
//!     v
//! Validated semantic result
//!     |
//!     v
//! Canonical intermediate representations
//!     |
//!     v
//! Target-specific lowering and execution
//! ```
//!
//! This module composes the existing frontend subsystems. It does not
//! implement a second lexer, parser, AST, semantic analyzer, or IR.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! The frontend conforms to the repository's language and architecture
//! contracts, particularly:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! `grammar/Zamani.g4` is the canonical grammar composition root.
//! Normative specifications define the language rules and meaning.
//! The canonical AST represents source structure and identity.
//! The semantic subsystem determines source-level meaning.
//! Downstream IR and backend components determine realization.
//!
//! This module must not override those authorities.
//!
//! ## OWNS
//!
//! This module owns:
//!
//! - Frontend subsystem composition.
//! - The canonical public path for frontend AST, lexer, parser, and semantics.
//! - Frontend-level convenience exports.
//! - The architectural boundary between source analysis and downstream
//!   compiler stages.
//! - Frontend-wide safety, portability, and integration invariants.
//!
//! ## DOES_NOT_OWN
//!
//! This module does not own:
//!
//! - Lexical rules, token definitions, or token scanning.
//! - Grammar productions, parsing algorithms, or parser recovery.
//! - AST node definitions, AST identity allocation, or AST construction.
//! - Symbol tables, name resolution, or type inference.
//! - Effect, capability, resource, or contract evaluation.
//! - Domain-specific semantic validation.
//! - Semantic diagnostics production or analysis-pass implementation.
//! - Classical, quantum, HDL, or other intermediate representations.
//! - Target discovery, resource allocation, scheduling, or routing.
//! - Quantum physical mapping, pulse generation, or error correction.
//! - Backend code generation or runtime execution.
//! - The legacy implementation in `crate::semantic`.
//!
//! Each responsibility remains with its existing authoritative subsystem.
//!
//! ## CANONICAL AST INTEGRATION
//!
//! The repository's canonical node implementation is located at:
//!
//! `src/frontend/ast/node/mod.rs`
//!
//! The `ast` facade below exposes that implementation at:
//!
//! `crate::frontend::ast::node`
//!
//! The explicit path avoids requiring a second AST implementation or
//! relocating existing node definitions. AST child modules continue to
//! resolve relative to their canonical source files.
//!
//! Do not define a second `Program`, `Node`, `NodeId`, `Span`, expression,
//! statement, declaration, or type representation in this module.
//!
//! ## LEXER INTEGRATION
//!
//! `crate::frontend::lexer` owns canonical tokenization, token kinds,
//! token metadata, lexical source spans, lexical diagnostics, dialect
//! policies, and scanning.
//!
//! Parser and semantic consumers must use the canonical frontend contracts.
//! Legacy lexer integration must occur through explicit compatibility
//! adapters owned by the appropriate subsystem.
//!
//! ## PARSER INTEGRATION
//!
//! `crate::frontend::parser` owns grammar recognition, parser sessions,
//! parsing configuration, recovery, parser diagnostics, and AST adaptation.
//!
//! The parser consumes canonical lexer tokens and constructs the canonical
//! AST. It must not implement a second lexer or a competing AST.
//!
//! Successful parsing does not establish semantic validity.
//!
//! ## SEMANTIC INTEGRATION
//!
//! `crate::frontend::semantic` is the authoritative semantic-analysis
//! subsystem for the native frontend AST.
//!
//! Its own `mod.rs` owns the declarations and exports for:
//!
//! - Public semantic API and analysis lifecycle.
//! - Configuration, context, sessions, and pipeline.
//! - Canonical semantic model.
//! - Symbols and name resolution.
//! - Types, inference, generics, and ownership rules.
//! - Effects, capabilities, resources, and contracts.
//! - Classical, quantum, HDL, hybrid, AI/data, concurrency, and
//!   distributed domain validation.
//! - Shared validation, diagnostics, traversal, and incremental analysis.
//! - Semantic conformance, compatibility, determinism, fuzz, and stress tests.
//!
//! This file must declare `semantic` exactly once and must not redeclare
//! any of those child modules.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! The frontend supports the architectural objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! The frontend must describe source meaning without imposing arbitrary
//! universal limits on hardware capacities or computational domains.
//!
//! In particular, it must not introduce fixed language-level ceilings for:
//!
//! - Qubits or quantum operations.
//! - CPUs, GPUs, FPGAs, ASICs, or other accelerators.
//! - Memory, storage, registers, or machine widths.
//! - Tensor dimensions, declarations, or generic parameters.
//! - Threads, processes, nodes, devices, or topology size.
//! - The number of supported or registered computational domains.
//!
//! Symbolic requirements and constraints must remain representable until
//! the owning semantic or downstream stage is authorized to evaluate them.
//!
//! Compiler operational budgets may be configured in their owning modules.
//! Exhausting such a budget must result in an explicit incomplete, aborted,
//! or failed outcome; it must never silently certify incomplete analysis.
//!
//! No finite implementation can process literally infinite input. The
//! architectural goal is to avoid arbitrary universal hardware ceilings and
//! scale with available resources, representable values, implementation
//! capabilities, and explicitly configured operational policies.
//!
//! ## SEMANTIC AND IR BOUNDARIES
//!
//! The semantic subsystem determines program meaning independently of
//! the target selected for execution.
//!
//! Quantum semantics must preserve `quantum::ir` as the canonical quantum
//! IR boundary. Classical and HDL representations remain owned by their
//! respective canonical IR subsystems.
//!
//! This frontend must not select physical qubits, allocate device memory,
//! place hardware resources, route operations, schedule execution, or
//! introduce a competing IR.
//!
//! ## LEGACY COMPATIBILITY
//!
//! The existing `crate::semantic` module is retained during migration.
//!
//! Adding this frontend must not redirect the legacy compiler pipeline,
//! remove its public interfaces, or silently substitute the native AST.
//!
//! Migration requires verified consumers, compatibility tests, and an
//! explicit integration change in the relevant compiler entry points.
//!
//! ## PUBLIC API
//!
//! Subsystem implementations remain accessible through their canonical
//! module paths. The re-exports below provide a small convenience surface
//! without duplicating the underlying types or implementation.
//!
//! ## FREEZE CRITERIA
//!
//! This file may be frozen when:
//!
//! 1. The crate root declares `frontend` exactly once.
//! 2. All declared frontend modules resolve to their canonical files.
//! 3. The AST facade resolves to the existing `ast/node` implementation.
//! 4. The lexer and parser compile against the same canonical AST contracts.
//! 5. Semantic exports resolve to the existing semantic API.
//! 6. The legacy analyzer remains intact during migration.
//! 7. The module graph contains no duplicate semantic or AST authority.
//! 8. No unsafe code or arbitrary hardware-capacity ceiling is introduced.
//! 9. Formatting, compilation, tests, and lint checks pass.
//!
//! This module's successful compilation is an integration gate, not proof
//! that every semantic feature or downstream backend is production-ready.

// -----------------------------------------------------------------------------
// Canonical AST facade
// -----------------------------------------------------------------------------
//
// The repository has its canonical AST under ast/node/. Expose that existing
// module tree under frontend::ast::node without copying or redefining it.

pub mod ast {
    #[path = "ast/node/mod.rs"]
    pub mod node;
}

// -----------------------------------------------------------------------------
// Frontend subsystems
// -----------------------------------------------------------------------------
//
// Each subsystem owns its own internal module declarations, implementation,
// and API. The frontend composition root declares each subsystem exactly once.

/// Canonical lexical analysis and token contracts.
pub mod lexer;

/// Canonical parsing, grammar integration, and AST adaptation.
pub mod parser;

/// Canonical target-independent semantic analysis.
///
/// This module's own `mod.rs` orchestrates its symbols, types, effects,
/// capabilities, resources, contracts, domains, validation, diagnostics,
/// traversal, incremental analysis, and tests.
pub mod semantic;

// -----------------------------------------------------------------------------
// Canonical public convenience exports
// -----------------------------------------------------------------------------
//
// These are re-exports of existing definitions. They do not introduce new
// implementations or change ownership of the underlying types.

/// Canonical source-level program root.
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
