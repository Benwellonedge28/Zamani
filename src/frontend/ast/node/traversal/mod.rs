//! # Zamani Frontend AST — Traversal Infrastructure
//!
//! `src/frontend/ast/node/traversal/mod.rs`
//!
//! Canonical module boundary for traversal of the native Zamani AST.
//!
//! ## Architectural role
//!
//! This module is intentionally a **facade**.
//!
//! It owns:
//!
//! - traversal-module organization;
//! - the public traversal API boundary;
//! - stable re-exports of canonical traversal primitives;
//! - traversal documentation and invariants;
//! - separation between traversal and AST semantics.
//!
//! It does **not** implement traversal itself.
//!
//! The canonical implementation currently lives in [`walk`].
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! native Zamani AST
//!      │
//!      ▼
//! ┌──────────────────────────────────────┐
//! │ frontend::ast::node::traversal       │
//! │                                      │
//! │  this module: stable public facade   │
//! │                 │                    │
//! │                 ▼                    │
//! │              walk.rs                 │
//! │       iterative canonical walker     │
//! └─────────────────┬────────────────────┘
//!                   │
//!          ┌────────┼────────┐
//!          ▼        ▼        ▼
//!      analysis  validation tooling
//!          │
//!          ▼
//!     semantic model
//!          │
//!          ▼
//!         ZUIR
//!          │
//!     ┌────┼────┐
//!     ▼    ▼    ▼
//! classical quantum HDL
//!    IR      IR     IR
//! ```
//!
//! ## Critical boundary
//!
//! Traversal is structural infrastructure.
//!
//! This module must never implement or encode:
//!
//! - quantum-gate semantics;
//! - qubit allocation;
//! - physical qubit assignment;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - backend selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR lowering;
//! - target-specific optimization;
//! - runtime execution;
//! - vendor-specific behavior.
//!
//! A quantum node is traversed exactly as a classical node, HDL node,
//! accelerator node, or future extension node: as a structural AST node.
//!
//! This is required for Zamani's:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## POCO-REAF and scalability
//!
//! The traversal API must not impose language-level limits on:
//!
//! - AST node count;
//! - nesting depth;
//! - number of declarations;
//! - number of expressions;
//! - number of statements;
//! - number of resources;
//! - number of quantum resources;
//! - number of operations;
//! - machine size;
//! - processor count;
//! - backend type;
//! - computational domain.
//!
//! Operational resource limits may be supplied by callers through the
//! traversal configuration exposed by [`walk`]. Such limits are compiler
//! safety policies, not Zamani language semantics.
//!
//! In particular, this module must never introduce constants such as:
//!
//! ```text
//! MAX_NODES
//! MAX_DEPTH
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_REGISTERS
//! ```
//!
//! as language or traversal semantics.
//!
//! "Infinity" in POCO-REAF means that traversal introduces no artificial
//! finite machine/program limit. Actual execution remains bounded by the
//! resources available to the compiler process and the representational
//! capacity of the host.
//!
//! ## Iterative traversal
//!
//! Traversal of arbitrarily deep AST structures must not depend on Rust call
//! stack depth.
//!
//! The canonical [`walk`] implementation therefore uses explicit heap-backed
//! traversal state rather than recursive calls for AST depth.
//!
//! This module must remain an API boundary around that implementation rather
//! than reimplementing recursive or iterative traversal itself.
//!
//! ## Ownership
//!
//! The responsibilities are deliberately separated:
//!
//! ```text
//! AST node definitions
//!        │
//!        ▼
//! AST storage / NodeId resolution
//!        │
//!        ▼
//! traversal::walk
//!        │
//!        ▼
//! visitors / analyses / validation / tooling
//! ```
//!
//! This module does not own AST storage.
//!
//! A traversal consumer should not need to know whether the AST is stored in:
//!
//! - an indexed vector;
//! - an arena;
//! - a persistent structure;
//! - an incremental store;
//! - an immutable store;
//! - another future representation.
//!
//! The canonical provider abstraction owns that integration boundary.
//!
//! ## Determinism
//!
//! The traversal contract requires deterministic child ordering.
//!
//! This module does not:
//!
//! - sort children;
//! - inspect hash-map ordering;
//! - introduce randomness;
//! - consult timestamps;
//! - consult hardware;
//! - consult backend state.
//!
//! The AST provider is responsible for exposing direct children in canonical
//! source order.
//!
//! ## Source-order invariant
//!
//! For a node:
//!
//! ```text
//! parent
//! ├── child A
//! ├── child B
//! └── child C
//! ```
//!
//! traversal must observe:
//!
//! ```text
//! parent → A → descendants(A) → B → descendants(B) → C → descendants(C)
//! ```
//!
//! unless a visitor explicitly requests that children be skipped or traversal
//! be stopped.
//!
//! ## Tree and graph safety
//!
//! Native source AST structure is logically tree-shaped even though references
//! may be represented using [`NodeId`].
//!
//! The canonical walker therefore provides explicit revisit policy rather than
//! silently assuming that every `NodeId` reference forms a perfect tree.
//!
//! This is important for malformed, corrupted, generated, or future extension
//! structures.
//!
//! Revisit behavior must remain a traversal policy. It must not become a
//! semantic rule for Zamani programs.
//!
//! ## Cancellation
//!
//! Traversal cancellation belongs at the traversal boundary rather than in
//! individual AST node implementations.
//!
//! The traversal API therefore exposes cancellation through the canonical
//! walker rather than coupling AST nodes to:
//!
//! - async runtimes;
//! - schedulers;
//! - quantum runtimes;
//! - backend execution;
//! - compiler-global state.
//!
//! ## Visitor integration
//!
//! Visitor definitions belong to:
//!
//! ```text
//! src/frontend/ast/node/visitors/
//! ```
//!
//! Traversal belongs here:
//!
//! ```text
//! src/frontend/ast/node/traversal/
//! ```
//!
//! The dependency direction is:
//!
//! ```text
//! node definitions
//!       │
//!       ▼
//! visitors
//!       │
//!       ▼
//! traversal
//!       │
//!       ▼
//! compiler consumers
//! ```
//!
//! Traversal must not create a second incompatible visitor protocol when the
//! canonical visitor infrastructure already exists.
//!
//! ## Canonical implementation
//!
//! [`walk`] is the canonical structural traversal implementation exposed by
//! this module.
//!
//! It is responsible for the actual iterative traversal algorithm and its
//! operational policies.
//!
//! This `mod.rs` deliberately does not duplicate those definitions.
//!
//! ## Duplicate traversal implementations
//!
//! A previous/parallel `preorder.rs` implementation exists in the repository.
//! It defines its own provider, visitor, configuration, error, and traversal
//! abstractions.
//!
//! Those duplicate abstractions must **not** become a second public traversal
//! API through this module.
//!
//! The canonical repository architecture already has visitor infrastructure
//! and a production walker. Consequently, this module intentionally exposes
//! the canonical walker rather than re-exporting competing provider/visitor
//! types.
//!
//! If `preorder.rs` is retained temporarily during migration, it should be
//! treated as a migration candidate rather than as a second authoritative
//! traversal subsystem.
//!
//! The final architecture must have one canonical traversal contract.
//!
//! ## API stability
//!
//! Consumers should preferentially import traversal primitives from this
//! module:
//!
//! ```text
//! frontend::ast::node::traversal::walk
//! ```
//!
//! rather than depending on private implementation details.
//!
//! Re-exports below provide a stable surface while allowing the implementation
//! to evolve internally.
//!
//! ## No unsafe
//!
//! This module contains no unsafe Rust.
//!
//! The traversal subsystem must remain compatible with:
//!
//! - `#![forbid(unsafe_code)]`;
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - stable Rust;
//! - edition 2021;
//! - no nightly-only features.
//!
//! ## Integration contract
//!
//! ### Upstream
//!
//! This module is consumed by the native AST node subsystem.
//!
//! It relies on the canonical node/visitor contracts already defined under:
//!
//! ```text
//! src/frontend/ast/node/
//! ```
//!
//! ### Downstream
//!
//! Consumers include:
//!
//! - structural validation;
//! - AST analysis;
//! - diagnostics tooling;
//! - semantic analysis;
//! - AST transformations;
//! - compiler tooling;
//! - serialization inspection;
//! - future source-level extensions.
//!
//! ### Explicitly excluded downstream ownership
//!
//! Traversal does not own:
//!
//! - semantic resolution;
//! - type inference;
//! - resource allocation;
//! - quantum compilation;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend execution.
//!
//! ## Extension compatibility
//!
//! A new AST node or computational domain must not require modification of
//! this module merely because the node belongs to a new domain.
//!
//! For example:
//!
//! ```text
//! classical node
//! quantum node
//! hybrid node
//! HDL node
//! accelerator node
//! future node
//! ```
//!
//! all participate through the same structural traversal contract.
//!
//! This is a central scalability invariant.
//!
//! ## Testing contract
//!
//! Tests for the traversal implementation belong with the canonical walker.
//!
//! This module should only contain module-level compile/integration guarantees
//! and should not duplicate the walker's behavioral test suite.
//!
//! Required traversal behavior includes:
//!
//! - empty/root-only ASTs;
//! - deterministic child ordering;
//! - deep ASTs;
//! - wide ASTs;
//! - large ASTs;
//! - cancellation;
//! - visitor-controlled child skipping;
//! - visitor-controlled termination;
//! - malformed/missing node references;
//! - revisit detection;
//! - explicitly configured resource limits;
//! - unrestricted traversal;
//! - source-order preservation;
//! - extension-node traversal.
//!
//! ## Definition of done
//!
//! This module is complete when:
//!
//! 1. the canonical traversal implementation is exposed;
//! 2. no duplicate traversal contract is introduced;
//! 3. no semantic-domain dependency is introduced;
//! 4. no hardware dependency is introduced;
//! 5. no fixed machine-size limit is introduced;
//! 6. no fixed quantum-resource limit is introduced;
//! 7. no unsafe code is present;
//! 8. the public API is stable and documented;
//! 9. downstream consumers can depend on this module without knowing the
//!    concrete AST storage strategy;
//! 10. adding a new AST domain does not require changing this facade.
//!
//! ## Dependency rule
//!
//! ```text
//! traversal/mod.rs
//!       │
//!       └── walk.rs
//!             │
//!             ├── canonical visitor contract
//!             ├── canonical NodeId
//!             └── canonical Node
//! ```
//!
//! Never introduce:
//!
//! ```text
//! traversal/mod.rs
//!       ↓
//! quantum backend
//!       ↓
//! hardware
//! ```
//!
//! or:
//!
//! ```text
//! traversal/mod.rs
//!       ↓
//! ZUIR
//! ```
//!
//! The traversal subsystem remains a frontend structural service.
//!
//! ## Rust-version policy
//!
//! The public API in this module deliberately uses ordinary stable Rust module
//! and re-export facilities available on Rust 1.97/1.97.1.
//!
//! No nightly feature gate is required.
//!
//! ## Public surface
//!
//! The canonical walker is exposed both as a module and through selected
//! stable re-exports:
//!
//! ```text
//! traversal::walk::...
//! traversal::WalkConfig
//! traversal::WalkError
//! traversal::WalkResult
//! traversal::AstNodeProvider
//! ```
//!
//! This keeps imports concise while preserving the canonical module namespace.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical iterative AST traversal implementation.
///
/// This is the single authoritative traversal engine exposed by this module.
///
/// All structural AST walking should use this implementation rather than
/// creating another recursive or domain-specific walker.
pub mod walk;

// =============================================================================
// Stable public API
// =============================================================================

/// Canonical AST node provider.
///
/// Re-exported from [`walk`] so consumers do not need to depend on the
/// implementation file's internal path.
pub use walk::AstNodeProvider;

/// Canonical cancellation interface.
///
/// Cancellation remains caller-owned and implementation-independent.
pub use walk::Cancellation;

/// A cancellation source that never requests cancellation.
pub use walk::NeverCancelled;

/// Canonical traversal configuration.
///
/// Configuration contains operational policies only; it does not define
/// Zamani language or machine-size semantics.
pub use walk::WalkConfig;

/// Canonical traversal error.
pub use walk::WalkError;

/// Canonical traversal result.
pub use walk::WalkResult;

/// Canonical traversal statistics.
pub use walk::WalkStatistics;

/// Canonical visitor-facing traversal primitive.
///
/// Re-exporting the function keeps consumers independent of the implementation
/// file location while preserving `walk` as the authoritative implementation.
pub use walk::walk;

/// Convenience API for walking an AST and obtaining traversal statistics.
pub use walk::walk_statistics;

// =============================================================================
// Compile-time architectural invariants
// =============================================================================

#[cfg(test)]
mod tests {
    //! Module-level integration tests.
    //!
    //! Behavioral traversal tests remain in `walk.rs`.
    //!
    //! These tests intentionally verify only the facade's public surface so
    //! that this module does not duplicate the canonical traversal test suite.

    use super::*;

    #[test]
    fn unrestricted_configuration_has_no_artificial_limits() {
        let config = WalkConfig::unrestricted();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert!(config.reject_revisits);
        assert!(config.check_cancellation);
    }

    #[test]
    fn canonical_public_types_are_reachable_from_facade() {
        fn assert_provider<T: AstNodeProvider>() {}

        fn assert_cancellation<T: Cancellation>() {}

        // The generic helper bodies are intentionally not instantiated with a
        // concrete repository type here. Their presence verifies that the
        // facade exports the canonical traits as part of its API.
        let _ = assert_provider::<NeverProvider>;
        let _ = assert_cancellation::<NeverCancelled>;
    }

    #[derive(Debug)]
    struct NeverProvider;

    impl AstNodeProvider for NeverProvider {
        type Error = core::convert::Infallible;

        fn node(
            &self,
            _id: crate::frontend::ast::node::node_id::NodeId,
        ) -> Option<&crate::frontend::ast::node::node::Node> {
            None
        }

        fn children<'a>(
            &'a self,
            _id: crate::frontend::ast::node::node_id::NodeId,
        ) -> Result<
            Box<
                dyn Iterator<
                        Item = crate::frontend::ast::node::node_id::NodeId,
                    > + 'a,
            >,
            Self::Error,
        > {
            Ok(Box::new(core::iter::empty()))
        }
    }
}