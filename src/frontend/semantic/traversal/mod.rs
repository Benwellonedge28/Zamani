//! # Zamani Semantic Traversal
//!
//! File: `src/frontend/semantic/traversal/mod.rs`
//! Status: Semantic traversal public facade and orchestrator.
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only; `unsafe` is forbidden.
//!
//! ## Purpose
//!
//! This module is the public orchestration boundary for traversal services
//! used by semantic analysis. It composes the semantic traversal visitor,
//! canonical AST walker adapter, and dependency-graph facilities.
//!
//! This file owns:
//!
//! - traversal submodule organization;
//! - the public semantic traversal API boundary;
//! - stable re-exports of approved traversal interfaces;
//! - architectural ownership and dependency rules;
//! - compile-time and API-surface invariants.
//!
//! This file does not implement traversal algorithms, enumerate AST children,
//! resolve names, infer types, evaluate effects, validate domains, or construct
//! intermediate representations.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani grammar and specifications
//!                |
//!                v
//!         frontend parser
//!                |
//!                v
//!       canonical frontend AST
//!                |
//!                v
//!   frontend::ast::node::traversal
//!                |
//!                v
//!   frontend::semantic::traversal
//!       |          |          |
//!       v          v          v
//!    visitor      walk    dependency_graph
//!       |          |          |
//!       +----------+----------+
//!                  |
//!                  v
//!       semantic analysis passes
//!                  |
//!                  v
//!        target-independent facts
//!                  |
//!                  v
//!       canonical downstream IRs
//! ```
//!
//! The diagram describes intended ownership and integration. It does not
//! claim that every semantic pass or downstream connection is implemented.
//!
//! ## Normative authorities
//!
//! This module must conform to the repository's applicable language and
//! architecture specifications, particularly:
//!
//! - `grammar/Zamani.g4`;
//! - `grammar/specification/grammar-authority.md`;
//! - `grammar/specification/semantic-model.md`;
//! - `grammar/specification/poco-reaf.md`;
//! - `grammar/specification/scalability-model.md`;
//! - `grammar/specification/portability.md`;
//! - `grammar/specification/compilation-model.md`;
//! - `grammar/specification/extensibility.md`.
//!
//! The root grammar composes source syntax. The semantic model defines
//! program meaning. The POCO-REAF specification defines portability and
//! resource-parametric scalability. This facade must not redefine any of
//! those contracts.
//!
//! If a referenced specification changes, affected integration tests must
//! establish whether this module's public contract remains conformant.
//!
//! ## Canonical AST integration
//!
//! The authoritative source AST is owned by:
//!
//! `crate::frontend::ast::node`
//!
//! The canonical structural traversal implementation is owned by:
//!
//! `crate::frontend::ast::node::traversal`
//!
//! The canonical immutable visitor contract is owned by:
//!
//! `crate::frontend::ast::node::visitors`
//!
//! Semantic traversal must reuse these contracts. It must not introduce
//! another AST node representation, child-provider contract, or incompatible
//! visitor protocol merely to support semantic analysis.
//!
//! The semantic `walk` module may adapt the canonical walker to semantic
//! analysis APIs, but must not independently reimplement its structural
//! traversal algorithm.
//!
//! ## Submodule ownership
//!
//! `visitor`
//!     Defines semantic traversal visitor adapters and semantic-pass hooks.
//!     It does not own AST structure, child enumeration, or traversal order.
//!
//! `walk`
//!     Exposes semantic traversal entry points and configuration while
//!     delegating structural walking to the canonical AST traversal layer.
//!     It does not duplicate the canonical walker.
//!
//! `dependency_graph`
//!     Owns traversal-related dependency graph construction and graph
//!     algorithms, where required by semantic analysis. It does not own
//!     declaration resolution, type rules, or the semantic meaning of cycles.
//!
//! Each submodule must document its own inputs, outputs, errors, dependencies,
//! determinism guarantees, operational budgets, and freeze criteria.
//!
//! ## Dependency direction
//!
//! The permitted dependency direction is:
//!
//! ```text
//! canonical AST nodes
//!        |
//!        v
//! canonical AST visitors and traversal
//!        |
//!        v
//! semantic traversal adapters
//!        |
//!        v
//! semantic analysis passes
//!        |
//!        v
//! semantic model
//! ```
//!
//! This facade must not depend on:
//!
//! - target-specific hardware implementations;
//! - physical resource allocation;
//! - backend selection;
//! - device scheduling or placement;
//! - quantum routing or calibration;
//! - quantum error correction implementations;
//! - runtime device handles;
//! - classical or quantum IR implementation details;
//! - code generation.
//!
//! The semantic subsystem establishes source-level meaning and obligations.
//! Downstream systems determine how that meaning can be realized.
//!
//! ## POCO-REAF and scalability
//!
//! Traversal must not introduce arbitrary language-level ceilings on:
//!
//! - AST node count;
//! - source nesting depth;
//! - declarations or expressions;
//! - child count;
//! - graph vertices or edges;
//! - quantum or classical resource quantities;
//! - tensor dimensions;
//! - hardware capacity;
//! - computational domains;
//! - extension count.
//!
//! The module must not add fixed machine-capacity constants or reinterpret
//! operational budgets as language restrictions.
//!
//! Default traversal configuration should impose no artificial node, edge,
//! or depth ceiling unless the canonical traversal contract explicitly says
//! otherwise. Explicit caller-supplied budgets are permitted for operational
//! safety.
//!
//! A finite process cannot handle literally infinite input. The requirement
//! is that the architecture introduces no arbitrary semantic ceiling beyond
//! representational limits, available resources, configured policies, and
//! implementation capabilities.
//!
//! If traversal is cancelled, encounters a cycle, fails to resolve a node,
//! or exceeds an explicitly configured budget, the caller must be able to
//! distinguish that outcome from complete successful traversal.
//!
//! ## Determinism
//!
//! For equivalent AST input, provider behavior, visitor behavior, and
//! configuration, traversal services must preserve the canonical ordering
//! defined by the AST provider and walker.
//!
//! This facade must not sort children, depend on hash-map iteration order,
//! inject randomness, or silently change traversal policy.
//!
//! Dependency-graph algorithms must document their own deterministic
//! ordering rules, especially when multiple valid topological orders exist.
//!
//! ## Errors and validity
//!
//! This facade must preserve underlying traversal errors rather than convert
//! failures into success, discard them, or replace them with ambiguous strings.
//!
//! A successful structural walk is not, by itself, proof that a program is
//! semantically valid.
//!
//! Semantic passes remain responsible for recording unresolved or invalid
//! facts, diagnostics, and incomplete analysis status in the semantic result.
//!
//! ## Extension and domain neutrality
//!
//! Traversal is structural. It must not require a new central branch whenever
//! Zamani gains a source-level construct or computational domain.
//!
//! Classical, quantum, hybrid, HDL, AI/data, distributed, concurrent, and
//! future constructs must use the same canonical traversal contracts wherever
//! their AST structure permits.
//!
//! Extension-specific interpretation belongs to registered semantic
//! validators or visitors, not to this facade.
//!
//! ## Safety and compatibility
//!
//! This module must:
//!
//! - use safe Rust only;
//! - forbid `unsafe`;
//! - avoid global mutable state;
//! - avoid hardware-specific assumptions;
//! - avoid duplicate AST or visitor definitions;
//! - remain compatible with Rust 2021 and Rust 1.97 or later;
//! - expose only interfaces supported by the companion modules.
//!
//! ## Freeze criteria
//!
//! This file is ready to freeze when:
//!
//! 1. Every declared submodule exists and compiles.
//! 2. Its re-exports match the actual public declarations of those modules.
//! 3. Semantic traversal reuses the canonical AST walker and visitor contract.
//! 4. No duplicate provider, visitor, or traversal configuration is introduced.
//! 5. Module dependency direction is verified.
//! 6. Default configuration does not introduce arbitrary size or depth limits.
//! 7. Traversal failures remain distinguishable from successful completion.
//! 8. The module builds and its public API tests pass on the supported toolchain.
//!
//! A future change to another module must not require editing this facade
//! unless that module's agreed public contract has intentionally changed.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Semantic traversal components
// -----------------------------------------------------------------------------
//
// These sibling modules are part of the agreed traversal subsystem contract.
// They must be supplied in the same integration change if they do not yet
// exist. Do not add placeholder implementations just to satisfy module
// declarations.
//
// visitor.rs:
//     Semantic visitor adapters and pass hooks.
//
// walk.rs:
//     Semantic-facing adapter over the canonical AST walker.
//
// dependency_graph.rs:
//     Traversal-related dependency graph utilities.

pub mod dependency_graph;
pub mod visitor;
pub mod walk;

// -----------------------------------------------------------------------------
// Public semantic walk API
// -----------------------------------------------------------------------------
//
// `walk.rs` owns these semantic-facing names. It must implement them as
// adapters over the canonical AST traversal API, not as a second walker.
//
// Keep the public surface deliberately small. Re-export implementation
// contracts only after their declarations and signatures have been verified
// in the corresponding sibling file.

pub use walk::{
    walk,
    walk_unrestricted,
    walk_with_config,
    walk_with_config_and_cancellation,
    SemanticNodeProvider,
    SemanticWalkConfig,
    SemanticWalkError,
    SemanticWalkResult,
    SemanticWalkStatistics,
};