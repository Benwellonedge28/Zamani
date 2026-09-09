//! # Zamani Native AST — Expressions
//!
//! Canonical module boundary for source-level expressions in the Zamani
//! frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! frontend::ast::node::expressions
//!     │
//!     ├── expression.rs
//!     │     └── canonical Expression node
//!     │
//!     ├── access.rs
//!     ├── assignment.rs
//!     ├── async.rs
//!     ├── await.rs
//!     ├── binary.rs
//!     ├── block.rs
//!     ├── call.rs
//!     ├── cast.rs
//!     ├── closure.rs
//!     ├── conditional.rs
//!     ├── identifier.rs
//!     ├── indexing.rs
//!     ├── lambda.rs
//!     ├── literal.rs
//!     ├── macro.rs
//!     ├── match_expr.rs
//!     ├── spawn.rs
//!     └── unary.rs
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **expression namespace** of the native Zamani source
//! AST.
//!
//! It is intentionally a module/facade boundary rather than a second
//! expression implementation.
//!
//! The concrete expression structures live in their dedicated files.
//! `mod.rs` owns:
//!
//! - module declaration;
//! - public module visibility;
//! - stable expression-module organization;
//! - expression API discoverability;
//! - expression-level architectural documentation;
//! - compile-time integration tests for the module boundary.
//!
//! It does **not** own:
//!
//! - parser implementation;
//! - lexer tokens;
//! - semantic types;
//! - symbol resolution;
//! - ZUIR;
//! - quantum IR;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - backend execution;
//! - runtime state.
//!
//! ## Native AST boundary
//!
//! The native expression AST represents source-language structure.
//!
//! It is not:
//!
//! - ZUIR;
//! - QIR;
//! - LLVM IR;
//! - MLIR;
//! - a quantum hardware IR;
//! - a scheduler representation;
//! - a routing representation;
//! - a backend instruction set.
//!
//! Those representations belong to later compilation stages.
//!
//! ## POCO-REAF
//!
//! The expression namespace participates in Zamani's:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF) architecture.
//!
//! Consequently, no expression module in this namespace may introduce a
//! machine-specific architectural limit.
//!
//! In particular, this module does not define:
//!
//! - maximum expression count;
//! - maximum operand count;
//! - maximum argument count;
//! - maximum register size;
//! - maximum qubit count;
//! - maximum machine size;
//! - maximum hardware width;
//! - maximum backend count;
//! - a fixed gate set;
//! - a fixed processor architecture;
//! - a fixed quantum technology.
//!
//! Compiler safety/resource limits, where required, belong to the appropriate
//! configurable compiler policy layer rather than this module.
//!
//! ## Quantum neutrality
//!
//! Quantum computation is intentionally supported through the generic
//! expression model and extensible source-level mechanisms already defined by
//! the expression nodes.
//!
//! This module therefore must **not** introduce a closed representation such
//! as:
//!
//! ```text
//! enum QuantumGate {
//!     X,
//!     Y,
//!     Z,
//!     H,
//!     CNOT,
//!     ...
//! }
//! ```
//!
//! Such a representation would make the source AST depend on a finite gate
//! vocabulary and would prevent future operations or computational
//! technologies from being introduced independently.
//!
//! Quantum-specific semantics are resolved downstream.
//!
//! ## Existing expression modules
//!
//! The expression namespace currently consists of the following canonical
//! modules:
//!
//! - [`access`]
//! - [`assignment`]
//! - [`async`]
//! - [`await`]
//! - [`binary`]
//! - [`block`]
//! - [`call`]
//! - [`cast`]
//! - [`closure`]
//! - [`conditional`]
//! - [`expression`]
//! - [`identifier`]
//! - [`indexing`]
//! - [`lambda`]
//! - [`literal`]
//! - [`macro`]
//! - [`match_expr`]
//! - [`spawn`]
//! - [`unary`]
//!
//! The list above is the authoritative source-level expression decomposition
//! for the current repository revision.
//!
//! ## Why modules are not wildcard re-exported
//!
//! This facade deliberately exposes the concrete expression modules as stable
//! namespaces instead of flattening every symbol into one namespace.
//!
//! For example:
//!
//! ```text
//! expressions::expression::Expression
//! expressions::binary::BinaryExpression
//! expressions::literal::LiteralExpression
//! ```
//!
//! This prevents unrelated expression types from colliding as the AST evolves
//! and gives future extensions room to grow without turning this file into a
//! fragile global namespace.
//!
//! The canonical `Expression` abstraction remains in [`expression`].
//!
//! Consumers that need a concrete expression type should use the corresponding
//! module explicitly.
//!
//! ## Keyword-named modules
//!
//! Rust keywords are escaped only where necessary:
//!
//! ```text
//! pub mod r#async;
//! pub mod r#await;
//! pub mod r#macro;
//! ```
//!
//! The source files remain named:
//!
//! ```text
//! async.rs
//! await.rs
//! macro.rs
//! ```
//!
//! This preserves the existing repository layout while remaining valid stable
//! Rust.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! node infrastructure
//!        │
//!        ▼
//! expressions
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
//! The reverse direction is forbidden.
//!
//! In particular, expression modules must not depend on:
//!
//! ```text
//! expressions → ZUIR
//! expressions → quantum::ir
//! expressions → hardware
//! expressions → routing
//! expressions → scheduling
//! expressions → QEC
//! expressions → resilience
//! expressions → backend
//! ```
//!
//! Later compiler stages may depend on the expression AST.
//!
//! ## Parser integration
//!
//! The parser is an upstream consumer of this namespace.
//!
//! Its responsibility is to construct the appropriate source-level expression
//! node after parsing the corresponding grammar production.
//!
//! The parser must not use this namespace to perform:
//!
//! - name resolution;
//! - type inference;
//! - overload resolution;
//! - resource allocation;
//! - physical qubit assignment;
//! - hardware selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend selection.
//!
//! The intended boundary is:
//!
//! ```text
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ▼
//! expressions
//!   │
//!   ▼
//! structural validation
//! ```
//!
//! ## Structural validation integration
//!
//! Each expression node owns its local structural invariants.
//!
//! Graph-level validation remains responsible for resolving referenced
//! [`NodeId`] values and validating the complete AST graph.
//!
//! This separation is important because expression nodes use IDs rather than
//! recursively owning arbitrary child nodes.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes validated expression nodes and resolves their
//! meaning.
//!
//! This may include:
//!
//! - names;
//! - symbols;
//! - types;
//! - generic arguments;
//! - operators;
//! - calls;
//! - effects;
//! - capabilities;
//! - resources;
//! - domains;
//! - control-flow meaning;
//! - quantum semantics.
//!
//! None of those semantic facts should be stored in this module merely to make
//! downstream compilation convenient.
//!
//! ## ZUIR integration
//!
//! Expressions lower through semantic analysis into the canonical universal
//! semantic representation used by Zamani.
//!
//! The intended direction is:
//!
//! ```text
//! native AST expression
//!        │
//!        ▼
//! semantic analysis
//!        │
//!        ▼
//! semantic expression
//!        │
//!        ▼
//! ZUIR
//! ```
//!
//! This module must not import ZUIR.
//!
//! ## Quantum integration
//!
//! Quantum frontends may use the generic expression nodes for source constructs
//! involving quantum values, parameters, control expressions, indexing,
//! calls, conditions, blocks and other ordinary language constructs.
//!
//! A genuinely quantum-specific source construct belongs in the appropriate
//! quantum extension layer rather than being added as a backend-specific
//! variant to this module.
//!
//! The native AST therefore remains independent of:
//!
//! - superconducting hardware;
//! - trapped-ion hardware;
//! - neutral-atom hardware;
//! - photonic hardware;
//! - topological hardware;
//! - analog quantum systems;
//! - vendor-specific devices;
//! - simulators;
//! - future quantum technologies.
//!
//! ## External quantum formats
//!
//! External quantum languages such as OpenQASM have their own frontend ASTs.
//!
//! Their architecture remains:
//!
//! ```text
//! external source
//!      │
//!      ▼
//! external-format AST
//!      │
//!      ▼
//! external validation
//!      │
//!      ▼
//! Zamani semantic/source representation
//!      │
//!      ▼
//! ZUIR
//! ```
//!
//! An external format AST must not redefine or replace this native expression
//! namespace.
//!
//! ## Scalability
//!
//! This module introduces no artificial finite scaling boundary.
//!
//! Expression cardinality is determined by the owning AST representation and
//! ultimately by available resources and compiler policy.
//!
//! The expression namespace itself does not assume:
//!
//! ```text
//! 32 qubits
//! 64 qubits
//! 128 qubits
//! 1024 qubits
//! 4096 qubits
//! ```
//!
//! or any equivalent machine-specific value.
//!
//! "Infinity" in the POCO-REAF requirement is interpreted architecturally:
//! this module does not impose a fixed finite machine or program-size limit.
//! Actual execution necessarily remains bounded by the resources available to
//! the compiler, runtime, storage system and target platform.
//!
//! ## Determinism
//!
//! This module contains no global mutable state and performs no registration at
//! runtime.
//!
//! Module declarations are statically known to the compiler.
//!
//! Deterministic AST behavior is therefore owned by the concrete expression
//! nodes, AST graph, parser and compiler policies rather than by hidden state in
//! this facade.
//!
//! ## Serialization
//!
//! Serialization of individual expression structures is owned by those
//! structures and by the AST serialization subsystem.
//!
//! This module must not introduce a second serialization format or duplicate
//! schema-version state.
//!
//! The authoritative AST serialization/versioning layer remains above the
//! individual expression modules.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe` code;
//! - performs no I/O;
//! - executes no user code;
//! - owns no raw pointers;
//! - owns no global mutable state;
//! - performs no unchecked memory access;
//! - performs no hardware interaction;
//! - performs no network access;
//! - performs no filesystem access.
//!
//! Malformed source input is handled by the lexer/parser/validation pipeline,
//! not by adding unsafe or backend-specific behavior here.
//!
//! ## Rust compatibility
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - stable Rust;
//! - edition 2021;
//! - no nightly-only features;
//! - no `unsafe`.
//!
//! `async`, `await`, and `macro` are declared using raw identifiers where
//! required by the Rust language.
//!
//! ## File contract
//!
//! ### Owns
//!
//! - the expression module namespace;
//! - module visibility;
//! - stable organization of expression implementations;
//! - expression-module integration tests.
//!
//! ### Does not own
//!
//! - expression implementations;
//! - source spans;
//! - node IDs;
//! - node metadata;
//! - semantic types;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - execution.
//!
//! ### Consumers
//!
//! - parser;
//! - AST validation;
//! - AST tooling;
//! - semantic analysis;
//! - AST serialization;
//! - later compiler phases.
//!
//! ### Dependencies
//!
//! Only the Rust module system and the expression child modules.
//!
//! No downstream compiler layer is imported here.
//!
//! ### Stability rule
//!
//! A new core expression construct must receive its own module when its
//! implementation is substantial enough to warrant isolation.
//!
//! A domain-specific construct should not be added to this namespace merely
//! because a backend happens to need it.
//!
//! ### No-re-edit integration guarantee
//!
//! This file intentionally contains only the stable expression module boundary.
//!
//! Adding or modifying the implementation of an existing expression module
//! does not require changes here.
//!
//! A future expression module requires one intentional module declaration here,
//! but does not require unrelated existing expression modules to be rewritten.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

// Keep this module itself completely free of unsafe Rust.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Core expression abstraction
// -----------------------------------------------------------------------------
//
// `expression.rs` owns the canonical source-level `Expression` representation.
// It is deliberately declared first because the remaining modules are concrete
// expression forms that participate in that common abstraction.
pub mod expression;

// -----------------------------------------------------------------------------
// Leaf / identity expressions
// -----------------------------------------------------------------------------

pub mod identifier;
pub mod literal;

// -----------------------------------------------------------------------------
// Operator and value expressions
// -----------------------------------------------------------------------------

pub mod binary;
pub mod unary;
pub mod cast;

// -----------------------------------------------------------------------------
// Access / mutation expressions
// -----------------------------------------------------------------------------

pub mod access;
pub mod assignment;
pub mod indexing;

// -----------------------------------------------------------------------------
// Invocation / callable expressions
// -----------------------------------------------------------------------------

pub mod call;
pub mod closure;
pub mod lambda;

// -----------------------------------------------------------------------------
// Control-flow / structured expressions
// -----------------------------------------------------------------------------

pub mod block;
pub mod conditional;
pub mod r#match_expr;

// -----------------------------------------------------------------------------
// Asynchronous / concurrent expressions
// -----------------------------------------------------------------------------

pub mod r#async;
pub mod r#await;
pub mod spawn;

// -----------------------------------------------------------------------------
// Macro expressions
// -----------------------------------------------------------------------------

pub mod r#macro;

// =============================================================================
// Compile-time module-boundary tests
// =============================================================================
//
// These tests intentionally verify only that the canonical expression modules
// remain available through this facade. They do not instantiate concrete nodes,
// because construction invariants belong to each concrete expression module.
//
// Keeping these tests here makes accidental module removal or visibility
// regression immediately visible without coupling this facade to the internal
// representation of another expression node.

#[cfg(test)]
mod tests {
    #[test]
    fn all_canonical_expression_modules_are_part_of_the_public_boundary() {
        // The imports below are intentionally explicit. They verify that the
        // public module tree remains stable without flattening all expression
        // symbols into one namespace.
        use super::{
            access,
            assignment,
            binary,
            block,
            call,
            cast,
            closure,
            conditional,
            expression,
            identifier,
            indexing,
            lambda,
            literal,
            r#async,
            r#await,
            r#macro,
            r#match_expr,
            spawn,
            unary,
        };

        // Keep the imported modules observable to the compiler while avoiding
        // assumptions about concrete node internals.
        let _ = (
            access::self,
            assignment::self,
            binary::self,
            block::self,
            call::self,
            cast::self,
            closure::self,
            conditional::self,
            expression::self,
            identifier::self,
            indexing::self,
            lambda::self,
            literal::self,
            r#async::self,
            r#await::self,
            r#macro::self,
            r#match_expr::self,
            spawn::self,
            unary::self,
        );
    }
}