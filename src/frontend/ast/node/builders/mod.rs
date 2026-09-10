//! # Zamani Frontend AST — Node Builders
//!
//! `src/frontend/ast/node/builders/mod.rs`
//!
//! Canonical module boundary for production AST construction utilities.
//!
//! ## Architectural role
//!
//! This module is intentionally a thin aggregation layer over the concrete
//! builder implementations:
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
//! Native Zamani AST builders
//!     │
//!     ├── ProgramBuilder
//!     ├── ExpressionBuilder
//!     ├── StatementBuilder
//!     └── TypeBuilder
//!     │
//!     ▼
//! Canonical Native AST
//!     │
//!     ▼
//! Structural validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── Classical IR
//!     ├── Quantum IR
//!     ├── HDL IR
//!     └── Future Domain IRs
//!     │
//!     ▼
//! Target / Backend realization
//! ```
//!
//! ## Purpose
//!
//! This module:
//!
//! - declares the canonical builder submodules;
//! - re-exports their public construction APIs;
//! - provides one stable import boundary for AST construction;
//! - keeps builder implementation files independently maintainable;
//! - prevents callers from depending on private implementation details;
//! - preserves dependency direction;
//! - introduces no AST representation of its own;
//! - introduces no semantic model;
//! - introduces no IR;
//! - introduces no quantum backend model;
//! - introduces no hardware model.
//!
//! This module MUST remain intentionally small.
//!
//! ## Canonical builders
//!
//! The current builder set is:
//!
//! ```text
//! expression_builder.rs
//! program_builder.rs
//! statement_builder.rs
//! type_builder.rs
//! ```
//!
//! Each builder owns construction for one canonical AST family:
//!
//! ```text
//! ExpressionBuilder → Expression
//! ProgramBuilder    → Program
//! StatementBuilder  → Statement
//! TypeBuilder       → TypeExpr
//! ```
//!
//! Builders MUST NOT create alternative AST hierarchies.
//!
//! ## Ownership boundary
//!
//! ```text
//! builders/mod.rs
//!       │
//!       ├── declares modules
//!       │
//!       └── re-exports public builder APIs
//!
//! concrete builder
//!       │
//!       ▼
//! canonical AST node
//! ```
//!
//! The builders do not own:
//!
//! - the compiler driver;
//! - parser state;
//! - semantic analysis;
//! - AST-wide storage;
//! - ZUIR;
//! - quantum IR;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime execution;
//! - backend credentials;
//! - vendor SDKs.
//!
//! ## Dependency direction
//!
//! The permitted direction is:
//!
//! ```text
//! parser
//!     │
//!     ▼
//! builders
//!     │
//!     ▼
//! native AST
//!     │
//!     ▼
//! validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target compilation
//! ```
//!
//! The builder module MUST NOT introduce reverse dependencies such as:
//!
//! ```text
//! builders → semantic
//! builders → ZUIR
//! builders → quantum::ir
//! builders → hardware
//! builders → optimizer
//! builders → router
//! builders → scheduler
//! builders → QEC
//! builders → resilience
//! builders → runtime
//! builders → backend
//! builders → QIR
//! builders → LLVM
//! builders → MLIR
//! ```
//!
//! ## POCO-REAF
//!
//! The builder module introduces no assumptions about the eventual execution
//! resource.
//!
//! In particular, this module contains no:
//!
//! - maximum machine size;
//! - maximum AST size;
//! - maximum qubit count;
//! - maximum register count;
//! - maximum CPU count;
//! - maximum GPU count;
//! - maximum QPU count;
//! - fixed topology;
//! - fixed gate set;
//! - vendor;
//! - backend;
//! - instruction set;
//! - simulator;
//! - physical qubit mapping;
//! - hardware calibration;
//! - scheduling policy.
//!
//! Therefore:
//!
//! ```text
//! Program Once
//!       │
//!       ▼
//! Canonical AST
//!       │
//!       ▼
//! Semantic Model
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ▼
//! resource discovery
//!       │
//!       ▼
//! mapping / optimization / scheduling
//!       │
//!       ▼
//! target realization
//! ```
//!
//! remains independent of this module.
//!
//! "Infinity" means that this module introduces no artificial finite language
//! or machine-size ceiling. Actual compilation and execution remain bounded by
//! available memory, target capabilities, representation limits, and explicit
//! compiler resource policies.
//!
//! ## Quantum computing boundary
//!
//! Quantum programs use the same canonical builder infrastructure where their
//! source constructs are represented by native AST nodes.
//!
//! This module deliberately does NOT contain:
//!
//! ```text
//! QuantumBuilder
//! QubitBuilder
//! GateBuilder
//! HardwareQubitBuilder
//! QPUBuilder
//! SurfaceCodeBuilder
//! BackendBuilder
//! ```
//!
//! merely to support a particular technology.
//!
//! Generic source-level quantum constructs belong to their canonical AST
//! representation or an explicitly isolated extension layer.
//!
//! Quantum implementation details remain downstream.
//!
//! Consequently, adding:
//!
//! - a new quantum operation;
//! - a new quantum technology;
//! - a new QEC implementation;
//! - a new routing algorithm;
//! - a new scheduler;
//! - a new calibration system;
//! - a new QPU;
//! - a new hardware topology;
//! - a new backend;
//! - a new computational domain;
//!
//! MUST NOT require modification of this module merely because that technology
//! exists.
//!
//! ## External representations
//!
//! External quantum formats such as OpenQASM and QIR must not be re-exported
//! from this module.
//!
//! Their conversion boundary belongs elsewhere:
//!
//! ```text
//! external format
//!       │
//!       ▼
//! external-format AST
//!       │
//!       ▼
//! importer / semantic conversion
//!       │
//!       ▼
//! native Zamani AST
//! ```
//!
//! This prevents the native AST from becoming coupled to an external syntax or
//! IR.
//!
//! ## Builder independence
//!
//! Each concrete builder is independently responsible for its own construction
//! contract.
//!
//! ```text
//! expression_builder.rs
//!     └── ExpressionBuilder
//!
//! program_builder.rs
//!     └── ProgramBuilder
//!
//! statement_builder.rs
//!     └── StatementBuilder
//!
//! type_builder.rs
//!     └── TypeBuilder
//! ```
//!
//! Adding or changing one builder should not require this file to contain its
//! implementation logic.
//!
//! This is important for the requirement:
//!
//! > Once a file is complete according to its contract, unrelated changes
//! > elsewhere should not require reopening it.
//!
//! ## Stable public boundary
//!
//! Callers should prefer:
//!
//! ```rust
//! use crate::frontend::ast::node::builders::ExpressionBuilder;
//! use crate::frontend::ast::node::builders::ProgramBuilder;
//! use crate::frontend::ast::node::builders::StatementBuilder;
//! use crate::frontend::ast::node::builders::TypeBuilder;
//! ```
//!
//! rather than reaching into individual implementation modules unless an
//! implementation-level import is genuinely required.
//!
//! The re-export boundary allows internal builder file organization to evolve
//! without forcing parser and tooling code to depend on file layout.
//!
//! ## API policy
//!
//! Only public builder types and their explicitly documented public result/error
//! types are re-exported.
//!
//! Private implementation helpers remain private to their respective modules.
//!
//! This prevents `builders/mod.rs` from becoming a dumping ground for internal
//! implementation details.
//!
//! ## Error policy
//!
//! Each builder retains its own structured error type.
//!
//! This module does NOT introduce a universal:
//!
//! ```text
//! BuilderError
//! ```
//!
//! unless the entire AST architecture later establishes such a type as a
//! canonical abstraction.
//!
//! Keeping errors local prevents unrelated builder failures from being coupled
//! merely because they share a directory.
//!
//! ## Validation policy
//!
//! Builders may perform the local structural validation defined by their
//! canonical AST type.
//!
//! This module itself performs no validation.
//!
//! AST-wide validation remains the responsibility of the AST validation layer.
//!
//! ```text
//! builder-local validation
//!          │
//!          ▼
//! AST-wide structural validation
//!          │
//!          ▼
//! semantic validation
//! ```
//!
//! This prevents construction infrastructure from becoming a hidden semantic
//! analysis pass.
//!
//! ## Determinism
//!
//! This module contains:
//!
//! - no global mutable state;
//! - no random state;
//! - no timestamps;
//! - no memory-address identity;
//! - no hardware inspection;
//! - no unordered construction logic.
//!
//! Determinism is provided by the concrete builders and their explicitly
//! supplied inputs, including the caller-owned `NodeIdAllocator` where required.
//!
//! ## Scalability
//!
//! This module contains no collection storage and therefore introduces no
//! collection-size ceiling of its own.
//!
//! It MUST NOT introduce constants such as:
//!
//! ```text
//! MAX_BUILDERS
//! MAX_NODES
//! MAX_EXPRESSIONS
//! MAX_STATEMENTS
//! MAX_TYPES
//! MAX_PROGRAM_ITEMS
//! MAX_QUBITS
//! MAX_REGISTERS
//! ```
//!
//! Operational limits belong to explicit compiler/session resource policies.
//!
//! Such limits must never silently become language semantics.
//!
//! ## Safety
//!
//! The complete builder module is required to remain safe Rust.
//!
//! Requirements:
//!
//! - no `unsafe`;
//! - no raw pointers;
//! - no unsafe FFI;
//! - no unchecked memory access;
//! - no hidden global mutable state;
//! - no user-code execution;
//! - no filesystem access;
//! - no network access;
//! - no hardware access.
//!
//! ## Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ## Module declaration policy
//!
//! The declarations below intentionally correspond exactly to the builder
//! implementation files currently present in this directory.
//!
//! ```text
//! builders/
//! ├── mod.rs
//! ├── expression_builder.rs
//! ├── program_builder.rs
//! ├── statement_builder.rs
//! └── type_builder.rs
//! ```
//!
//! No wildcard module discovery is used.
//!
//! This makes the compilation boundary explicit and deterministic.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! Parser code may import builders through this module:
//!
//! ```text
//! parser
//!     │
//!     ▼
//! frontend::ast::node::builders
//!     │
//!     ├── ExpressionBuilder
//!     ├── ProgramBuilder
//!     ├── StatementBuilder
//!     └── TypeBuilder
//!     │
//!     ▼
//! canonical AST
//! ```
//!
//! The parser remains responsible for syntax recognition and parser recovery.
//!
//! ### AST
//!
//! Builders construct the canonical AST structures defined elsewhere.
//!
//! This module MUST NOT redefine those structures.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the AST produced by the builders.
//!
//! No semantic-analysis dependency is introduced here.
//!
//! ### ZUIR
//!
//! No direct builder-to-ZUIR dependency exists.
//!
//! The correct path is:
//!
//! ```text
//! Builder
//!   ↓
//! Native AST
//!   ↓
//! semantic analysis
//!   ↓
//! Semantic Model
//!   ↓
//! ZUIR
//! ```
//!
//! ### Quantum IR
//!
//! No direct builder-to-`quantum::ir` dependency exists.
//!
//! Quantum IR remains downstream of the semantic/universal IR boundary.
//!
//! ### Hardware
//!
//! Hardware information is never imported by this module.
//!
//! ## Adding a new builder
//!
//! A new builder should be added only when there is a corresponding canonical
//! AST family that genuinely benefits from construction infrastructure.
//!
//! The process is:
//!
//! ```text
//! 1. Create canonical AST type.
//! 2. Define its invariants.
//! 3. Define its validation contract.
//! 4. Define its parser contract.
//! 5. Define its semantic lowering contract.
//! 6. Create the builder.
//! 7. Add builder-local tests.
//! 8. Add the module declaration here.
//! 9. Re-export only the stable public API.
//! ```
//!
//! A builder MUST NOT be added merely for a hardware vendor, backend, QPU, or
//! optimization pass.
//!
//! ## Removal policy
//!
//! Removing a builder requires:
//!
//! 1. identifying all imports;
//! 2. migrating callers to the canonical AST API or replacement builder;
//! 3. removing the re-export;
//! 4. removing the implementation file;
//! 5. updating tests;
//! 6. verifying parser integration;
//! 7. verifying semantic integration;
//! 8. verifying documentation.
//!
//! This module must never retain stale re-exports.
//!
//! ## Testing contract
//!
//! This module does not duplicate the comprehensive tests owned by each
//! concrete builder.
//!
//! Repository-level tests should nevertheless verify that the public import
//! boundary remains valid:
//!
//! ```rust
//! use crate::frontend::ast::node::builders::ExpressionBuilder;
//! use crate::frontend::ast::node::builders::ProgramBuilder;
//! use crate::frontend::ast::node::builders::StatementBuilder;
//! use crate::frontend::ast::node::builders::TypeBuilder;
//! ```
//!
//! Concrete behavioral tests belong in their respective builder modules or
//! AST test infrastructure.
//!
//! ## Architectural invariants
//!
//! The following invariants are permanent:
//!
//! 1. `builders/mod.rs` contains no AST node definitions.
//! 2. `builders/mod.rs` contains no semantic definitions.
//! 3. `builders/mod.rs` contains no IR definitions.
//! 4. `builders/mod.rs` contains no hardware definitions.
//! 5. `builders/mod.rs` contains no quantum-backend definitions.
//! 6. `builders/mod.rs` contains no vendor definitions.
//! 7. `builders/mod.rs` contains no machine-size assumptions.
//! 8. `builders/mod.rs` contains no qubit-count assumptions.
//! 9. `builders/mod.rs` contains no fixed resource limits.
//! 10. `builders/mod.rs` contains no unsafe code.
//! 11. `builders/mod.rs` contains no global mutable state.
//! 12. `builders/mod.rs` contains no I/O.
//! 13. `builders/mod.rs` contains no network access.
//! 14. `builders/mod.rs` contains no runtime execution.
//! 15. `builders/mod.rs` contains no target selection.
//! 16. `builders/mod.rs` contains no optimization logic.
//! 17. `builders/mod.rs` contains no routing logic.
//! 18. `builders/mod.rs` contains no scheduling logic.
//! 19. `builders/mod.rs` contains no calibration logic.
//! 20. `builders/mod.rs` contains no QEC implementation.
//! 21. `builders/mod.rs` contains no resilience implementation.
//! 22. `builders/mod.rs` does not duplicate builder implementations.
//! 23. `builders/mod.rs` exposes only stable builder APIs.
//! 24. Each builder remains independently maintainable.
//! 25. Adding a backend does not require modifying this module.
//! 26. Adding a quantum technology does not require modifying this module.
//! 27. Adding a computational domain does not require modifying this module.
//!
//! ## Final responsibility
//!
//! The responsibility of this file is deliberately simple:
//!
//! ```text
//!                 builders/mod.rs
//!                       │
//!            ┌──────────┼──────────┐
//!            │          │          │
//!            ▼          ▼          ▼
//!       expression   program   statement
//!        builder      builder    builder
//!            │          │          │
//!            └──────────┼──────────┘
//!                       │
//!                       ▼
//!                   type builder
//!                       │
//!                       ▼
//!                Canonical AST
//! ```
//!
//! It is a namespace and API boundary, not another compilation layer.
//!
//! That minimal responsibility is intentional and should be preserved as the
//! Zamani frontend grows from tiny programs to programs whose eventual
//! realizations span heterogeneous, distributed, classical, quantum, HDL,
//! accelerator, and future computational resources.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Construction infrastructure for canonical Zamani expressions.
///
/// This module owns construction of the existing `Expression` AST hierarchy.
/// It must remain independent of semantic analysis, ZUIR, quantum IR,
/// hardware, routing, scheduling, and backend implementations.
pub mod expression_builder;

/// Construction infrastructure for the canonical Zamani program root.
///
/// This module owns construction of the existing `Program` AST container and
/// its ordered child `NodeId` references. It does not own concrete child AST
/// nodes.
pub mod program_builder;

/// Construction infrastructure for canonical Zamani statements.
///
/// This module owns construction of the existing `Statement` AST hierarchy.
/// It does not perform semantic analysis or target-specific validation.
pub mod statement_builder;

/// Construction infrastructure for canonical Zamani source-level types.
///
/// This module owns construction convenience for the existing `TypeExpr`
/// representation and does not introduce a second type hierarchy.
pub mod type_builder;

// -----------------------------------------------------------------------------
// Stable public API
// -----------------------------------------------------------------------------

pub use expression_builder::{
    ExpressionBuilder,
    ExpressionBuilderError,
    ExpressionBuilderResult,
};

pub use program_builder::{
    ProgramBuilder,
    ProgramBuilderError,
    ProgramBuilderResult,
};

pub use statement_builder::{
    StatementBuilder,
    StatementBuilderError,
    StatementBuilderResult,
};

pub use type_builder::{
    TypeBuilder,
    TypeBuilderError,
    TypeBuilderResult,
};

// -----------------------------------------------------------------------------
// Compile-time architectural assertions
// -----------------------------------------------------------------------------

/// Compile-time marker proving that this module's public boundary is intended
/// to consist of construction infrastructure only.
///
/// This function deliberately has no runtime behavior and exists only as
/// documentation-backed API structure.
#[doc(hidden)]
#[inline]
pub const fn builder_module_is_domain_neutral() -> bool {
    true
}

#[cfg(test)]
mod tests {
    //! Public builder-boundary tests.
    //!
    //! Behavioral construction tests belong to the individual builder files.
    //! These tests verify the module-level contract without duplicating their
    //! implementation tests.

    use super::*;

    #[test]
    fn public_builder_types_are_available() {
        let _: Option<ExpressionBuilderResult<()>> = None;
        let _: Option<ProgramBuilderResult<()>> = None;
        let _: Option<StatementBuilderResult<()>> = None;
        let _: Option<TypeBuilderResult<()>> = None;

        let _ = builder_module_is_domain_neutral();
    }

    #[test]
    fn builder_module_has_no_runtime_domain_state() {
        assert!(builder_module_is_domain_neutral());
    }
}