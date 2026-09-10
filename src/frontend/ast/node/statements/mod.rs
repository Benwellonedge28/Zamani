//! # Zamani Frontend AST — Statements
//!
//! Canonical module boundary for source-level statement nodes in the Zamani
//! frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! ┌─────────────────────────────────────────────┐
//! │ Native Zamani AST                           │
//! │                                             │
//! │ ast/node/statements/mod.rs  ← this module   │
//! │                                             │
//! │   statement.rs                              │
//! │   let_stmt.rs                               │
//! │   const_stmt.rs                             │
//! │   return_stmt.rs                            │
//! │   break_stmt.rs                             │
//! │   continue_stmt.rs                          │
//! │   while_stmt.rs                             │
//! │   for_stmt.rs                               │
//! │   loop_stmt.rs                              │
//! └──────────────────────┬──────────────────────┘
//!                        │
//!                        ▼
//!              structural validation
//!                        │
//!                        ▼
//!                 semantic analysis
//!                        │
//!                        ▼
//!                  semantic model
//!                        │
//!                        ▼
//!                       ZUIR
//!                        │
//!              ┌─────────┼─────────┐
//!              ▼         ▼         ▼
//!          classical   quantum     HDL
//!             IR         IR         IR
//! ```
//!
//! ## Purpose
//!
//! This file is the **module boundary** for the canonical native Zamani
//! statement AST.
//!
//! It owns:
//!
//! - statement-module composition;
//! - public visibility of statement submodules;
//! - stable re-exports of statement AST types;
//! - documentation of the statement-layer dependency boundary;
//! - module-level compile-time safety guarantees.
//!
//! It does **not** implement statement semantics itself.
//!
//! Concrete statement semantics remain in their respective files.
//!
//! ## Current repository contract
//!
//! The current statement subsystem contains these concrete source files:
//!
//! ```text
//! statements/
//! ├── mod.rs              ← this file
//! ├── statement.rs
//! ├── let_stmt.rs
//! ├── const_stmt.rs
//! ├── return_stmt.rs
//! ├── break_stmt.rs
//! ├── continue_stmt.rs
//! ├── while_stmt.rs
//! ├── for_stmt.rs
//! └── loop_stmt.rs
//! ```
//!
//! This module intentionally declares **only files that exist in the
//! repository**.
//!
//! In particular, this file must not declare speculative modules such as:
//!
//! ```text
//! if_stmt.rs
//! expression_stmt.rs
//! match_stmt.rs
//! ```
//!
//! merely because corresponding concepts may eventually be useful.
//!
//! The canonical generic `Statement` representation already provides source
//! level representation for constructs whose concrete implementation belongs
//! elsewhere or whose representation is still evolving.
//!
//! New files should be added only when the corresponding source-language
//! contract has been designed and implemented.
//!
//! ## Native AST boundary
//!
//! Statements are source-language structures.
//!
//! They must remain independent from:
//!
//! - CPU instructions;
//! - GPU instructions;
//! - FPGA instructions;
//! - ASIC implementations;
//! - QPU instructions;
//! - physical qubit IDs;
//! - physical topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - quantum error correction implementation;
//! - noise models;
//! - resilience implementation;
//! - backend queues;
//! - backend credentials;
//! - vendor APIs;
//! - runtime execution state;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target-specific optimization.
//!
//! Those concerns belong to downstream compiler layers.
//!
//! ## POCO-REAF
//!
//! The statement layer is designed for:
//!
//! ```text
//! Program_Once
//!      │
//!      ▼
//! Compile_Once
//!      │
//!      ▼
//! Run_Everywhere
//!      │
//!      ▼
//! Anywhere
//!      │
//!      ▼
//! Forever
//! ```
//!
//! A statement therefore expresses **program intent**, not machine realization.
//!
//! No statement module may introduce assumptions about:
//!
//! - maximum machine size;
//! - maximum number of qubits;
//! - maximum number of CPUs;
//! - maximum number of GPUs;
//! - maximum register count;
//! - fixed topology;
//! - fixed instruction set;
//! - fixed vendor;
//! - fixed backend;
//! - fixed computational domain.
//!
//! ## Scalability
//!
//! This module contains no fixed-size collections, fixed machine dimensions,
//! or machine-specific limits.
//!
//! The actual statement nodes use `NodeId` references and dynamically sized
//! collections where source structure requires them. The surrounding AST graph
//! owns referenced nodes.
//!
//! "Infinity" in the Zamani POCO-REAF requirement means that this layer imposes
//! no artificial finite semantic limit on the size of a program. Actual
//! execution remains bounded only by available resources, address space,
//! storage, compiler policy, and the representational limits of the execution
//! environment.
//!
//! Resource limits used to protect compiler services must be configured by
//! higher-level compiler/session policies. They must not be hidden in this
//! module.
//!
//! ## Dependency direction
//!
//! The permitted dependency direction is:
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! node / node_id / node_kind / metadata
//!        │
//!        ▼
//! statements
//!        │
//!        ▼
//! structural validation
//!        │
//!        ▼
//! semantic analysis
//!        │
//!        ▼
//! semantic model
//!        │
//!        ▼
//! ZUIR
//!        │
//!        ▼
//! domain IR
//!        │
//!        ▼
//! target/backend
//! ```
//!
//! This module must never reverse that direction.
//!
//! In particular:
//!
//! ```text
//! statements → semantic       forbidden
//! statements → ZUIR            forbidden
//! statements → quantum IR      forbidden
//! statements → hardware        forbidden
//! statements → scheduler       forbidden
//! statements → router          forbidden
//! statements → runtime         forbidden
//! ```
//!
//! ## Quantum integration
//!
//! Quantum computation must not cause this module to become a quantum-specific
//! AST.
//!
//! Generic statements can participate in quantum programs through expressions,
//! resources, effects, capabilities, and extension constructs represented by
//! the broader AST architecture.
//!
//! For example:
//!
//! ```text
//! let / const
//!     │
//!     └── generic expression
//!
//! if / while / for / loop
//!     │
//!     └── generic condition/resource expression
//!
//! return
//!     │
//!     └── generic result expression
//!
//! expression statement
//!     │
//!     └── generic operation expression
//! ```
//!
//! A quantum operation may ultimately become a quantum-domain IR operation,
//! while the surrounding statement remains completely unaware of whether the
//! operation executes on:
//!
//! - a simulator;
//! - a superconducting QPU;
//! - a trapped-ion system;
//! - a neutral-atom system;
//! - a photonic system;
//! - another quantum technology;
//! - a future computational substrate.
//!
//! ## External quantum languages
//!
//! External formats such as OpenQASM must not be imported into this module.
//!
//! Their ASTs belong to their respective frontend-format modules and are
//! lowered into Zamani's semantic representation.
//!
//! The architectural direction is:
//!
//! ```text
//! External quantum language
//!          │
//!          ▼
//! External-format AST
//!          │
//!          ▼
//! Zamani semantic representation
//!          │
//!          ▼
//! ZUIR
//! ```
//!
//! Not:
//!
//! ```text
//! OpenQASM AST → native Zamani statements
//! ```
//!
//! where the native statement architecture becomes coupled to external syntax.
//!
//! ## Concrete statement modules
//!
//! ### `statement.rs`
//!
//! Owns the canonical source-level [`Statement`] container and
//! [`StatementKind`] representation, including generic extension statements.
//!
//! This is the central statement abstraction.
//!
//! ### `let_stmt.rs`
//!
//! Owns the concrete source-level `let` binding node.
//!
//! ### `const_stmt.rs`
//!
//! Owns the concrete source-level `const` binding node.
//!
//! ### `return_stmt.rs`
//!
//! Owns the concrete source-level `return` node.
//!
//! ### `break_stmt.rs`
//!
//! Owns the concrete source-level `break` node.
//!
//! ### `continue_stmt.rs`
//!
//! Owns the concrete source-level `continue` node.
//!
//! ### `while_stmt.rs`
//!
//! Owns the concrete source-level `while` loop node.
//!
//! ### `for_stmt.rs`
//!
//! Owns the concrete source-level `for` loop node.
//!
//! ### `loop_stmt.rs`
//!
//! Owns the generic/native loop statement representation where required by the
//! current AST architecture.
//!
//! ## Why concrete modules remain separate
//!
//! Each concrete node has its own:
//!
//! - fields;
//! - constructors;
//! - local validation;
//! - source-span behavior;
//! - `AstNode` implementation;
//! - serialization contract;
//! - tests;
//! - integration contract.
//!
//! The module boundary must not duplicate any of those implementations.
//!
//! This permits an individual file to be completed and tested independently
//! while this `mod.rs` remains a stable composition layer.
//!
//! ## Public API policy
//!
//! The preferred public API is:
//!
//! ```text
//! crate::frontend::ast::node::statements::Statement
//! crate::frontend::ast::node::statements::StatementKind
//! crate::frontend::ast::node::statements::LetStatement
//! crate::frontend::ast::node::statements::ConstStatement
//! ...
//! ```
//!
//! Consumers should not need to know the physical filename in which a
//! statement implementation lives.
//!
//! The submodules nevertheless remain public so tooling, tests, and compiler
//! components can explicitly access a concrete module when required.
//!
//! ## Stable re-export policy
//!
//! Re-exports are intentionally explicit rather than using wildcard imports.
//!
//! This prevents accidental public API expansion when a concrete implementation
//! later adds an internal helper type.
//!
//! A newly added public type therefore does not automatically become part of
//! the stable statement-module API.
//!
//! ## No duplicate abstractions
//!
//! This module must not redefine:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - source spans;
//! - metadata;
//! - expressions;
//! - patterns;
//! - declarations;
//! - types;
//! - semantic types;
//! - quantum resources;
//! - ZUIR operations.
//!
//! Those abstractions already have authoritative locations elsewhere in the
//! repository.
//!
//! The existing concrete statement files already integrate with the common
//! `AstNode`/`Node` infrastructure; this module simply exposes them through one
//! coherent namespace.
//!
//! ## Validation boundary
//!
//! This module performs no graph-wide validation.
//!
//! Local validation belongs to the individual statement types.
//!
//! Graph-wide validation belongs to the AST validation subsystem because only
//! the enclosing AST graph can determine whether every referenced `NodeId`
//! actually exists and has the expected relationship.
//!
//! Conceptually:
//!
//! ```text
//! statement-local validation
//!          │
//!          ▼
//! graph-level AST validation
//!          │
//!          ▼
//! semantic validation
//!          │
//!          ▼
//! domain validation
//!          │
//!          ▼
//! target validation
//! ```
//!
//! ## Traversal boundary
//!
//! Statement nodes expose child `NodeId` references.
//!
//! Traversal infrastructure must own graph traversal.
//!
//! This module must not introduce recursive graph walking or hidden recursion
//! limits.
//!
//! Large and deeply nested programs should be traversable by the repository's
//! traversal infrastructure using an explicit worklist/stack where appropriate.
//!
//! ## Serialization boundary
//!
//! Concrete statement types are responsible for their Serde representation.
//!
//! This module does not introduce a second serialization format.
//!
//! AST-wide schema/version handling belongs to the AST serialization subsystem.
//!
//! The statement representation nevertheless exposes its existing statement
//! schema version through [`STATEMENT_SCHEMA_VERSION`].
//!
//! ## Parser integration contract
//!
//! The parser may depend on this module:
//!
//! ```text
//! lexer → parser → statements
//! ```
//!
//! The parser is responsible for constructing syntactically valid source AST
//! nodes.
//!
//! The statement module must never import parser or lexer implementation
//! details.
//!
//! ## Semantic integration contract
//!
//! Semantic analysis consumes this module:
//!
//! ```text
//! statements → semantic analysis
//! ```
//!
//! Semantic analysis owns:
//!
//! - name resolution;
//! - type resolution;
//! - control-flow semantics;
//! - resource semantics;
//! - effects;
//! - capabilities;
//! - domain interpretation;
//! - generic substitution.
//!
//! None of those concepts should be added to this module merely to simplify a
//! later compiler phase.
//!
//! ## ZUIR integration contract
//!
//! The statement subsystem lowers indirectly:
//!
//! ```text
//! Statement
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future-domain IR
//! ```
//!
//! This module must not import ZUIR.
//!
//! ## Error ownership
//!
//! Statement-local construction and validation errors are re-exported from
//! `statement.rs`.
//!
//! Module composition itself introduces no custom runtime error type.
//!
//! Compilation errors caused by semantic interpretation belong to later
//! compiler layers.
//!
//! ## Thread safety
//!
//! This module contains no mutable global state and no runtime state.
//!
//! Thread-safety therefore follows the concrete AST node types and their
//! dependencies.
//!
//! Immutable AST structures should remain shareable across compiler phases when
//! their constituent types satisfy the corresponding Rust `Send`/`Sync`
//! requirements.
//!
//! ## Security
//!
//! This module:
//!
//! - performs no I/O;
//! - executes no source code;
//! - contains no raw pointers;
//! - contains no `unsafe`;
//! - owns no global mutable state;
//! - contains no unchecked memory operations;
//! - introduces no network access;
//! - introduces no filesystem access.
//!
//! It is therefore safe to use as part of an untrusted-source compiler
//! frontend, subject to the validation/resource policies of the enclosing
//! compiler.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! No unstable language or library features are required.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Concrete statement modules
// -----------------------------------------------------------------------------
//
// Keep these declarations explicit and synchronized with the actual repository
// tree. Do not add speculative modules here.
//
// The declaration order is intentionally foundational-to-control-flow oriented.
// Rust does not require declaration order for correctness, but keeping a stable
// order makes the module boundary easier to audit.

pub mod statement;
pub mod let_stmt;
pub mod const_stmt;
pub mod return_stmt;
pub mod break_stmt;
pub mod continue_stmt;
pub mod while_stmt;
pub mod for_stmt;
pub mod loop_stmt;

// -----------------------------------------------------------------------------
// Canonical statement API
// -----------------------------------------------------------------------------
//
// These are the primary types consumers should normally import from this
// module. Explicit re-exports prevent accidental exposure of unrelated
// implementation helpers.

pub use statement::{
    Statement,
    StatementError,
    StatementKind,
    StatementResult,
    StatementValidationPolicy,
    STATEMENT_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Concrete statement API
// -----------------------------------------------------------------------------
//
// Re-export only the canonical concrete node types. Their implementation,
// validation, serialization and AstNode contracts remain owned by their
// respective files.

pub use let_stmt::LetStatement;
pub use const_stmt::ConstStatement;
pub use return_stmt::ReturnStatement;
pub use break_stmt::BreakStatement;
pub use continue_stmt::ContinueStatement;
pub use while_stmt::WhileStatement;
pub use for_stmt::ForStatement;
pub use loop_stmt::LoopStatement;

// -----------------------------------------------------------------------------
// Module-level tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_statement_api_is_exposed() {
        let _ = StatementValidationPolicy::unrestricted();
        assert_eq!(STATEMENT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn statement_modules_are_publicly_reachable() {
        let _: fn() -> StatementValidationPolicy =
            StatementValidationPolicy::unrestricted;

        let _statement_type: Option<Statement> = None;
        let _statement_kind: Option<StatementKind> = None;

        let _let_statement: Option<LetStatement> = None;
        let _const_statement: Option<ConstStatement> = None;
        let _return_statement: Option<ReturnStatement> = None;
        let _break_statement: Option<BreakStatement> = None;
        let _continue_statement: Option<ContinueStatement> = None;
        let _while_statement: Option<WhileStatement> = None;
        let _for_statement: Option<ForStatement> = None;
        let _loop_statement: Option<LoopStatement> = None;
    }

    #[test]
    fn unrestricted_policy_has_no_hidden_statement_limits() {
        let policy = StatementValidationPolicy::unrestricted();

        assert_eq!(policy.max_children, None);
        assert_eq!(policy.max_extension_namespace_bytes, None);
        assert_eq!(policy.max_extension_name_bytes, None);
    }

    #[test]
    fn extension_statements_remain_open_ended() {
        let kind = StatementKind::Extension {
            namespace: String::from("zamani.example"),
            name: String::from("future-operation"),
            children: Vec::new(),
        };

        assert!(kind.is_extension());
        assert_eq!(kind.child_count(), 0);
    }
}