//! # Zamani Frontend AST — Program Node Family
//!
//! Canonical module boundary for the source-level program hierarchy of the
//! Zamani frontend AST.
//!
//! This module is intentionally an aggregation boundary. It does not own the
//! implementation of `Program`, `Module`, `Item`, or program attributes.
//! Those responsibilities belong to their dedicated sibling modules.
//!
//! ## Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                            Lexer
//!                              │
//!                              ▼
//!                           Parser
//!                              │
//!                              ▼
//!                    Native Zamani Frontend AST
//!                              │
//!                              ▼
//!                    ast::node::program
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!       Program             Module              Item
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              │
//!                              ▼
//!                     AST node storage
//!                              │
//!                              ▼
//!                    Structural validation
//!                              │
//!                              ▼
//!                     Semantic analysis
//!                              │
//!                              ▼
//!                       Semantic Model
//!                              │
//!                              ▼
//!                             ZUIR
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          ▼                   ▼                   ▼
//!      Classical            Quantum                HDL
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                       Target realization
//!                              │
//!                              ▼
//!                 CPU / GPU / FPGA / QPU /
//!                 distributed / future systems
//! ```
//!
//! ## Purpose
//!
//! This module provides the canonical public aggregation boundary for the
//! program-level AST node family.
//!
//! It owns only:
//!
//! - module declarations;
//! - public re-exports;
//! - program-family API documentation;
//! - program-family compile-time invariants that can be checked without
//!   depending on later compiler stages.
//!
//! It does **not** own:
//!
//! - concrete `Program` implementation;
//! - concrete `Module` implementation;
//! - concrete `Item` implementation;
//! - attribute implementation;
//! - semantic analysis;
//! - name resolution;
//! - type checking;
//! - generic substitution;
//! - capability resolution;
//! - resource allocation;
//! - quantum compilation;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum optimization;
//! - quantum error correction;
//! - resilience;
//! - calibration;
//! - backend selection;
//! - hardware execution.
//!
//! ## POCO-REAF
//!
//! The program-node family is deliberately independent of computational scale.
//!
//! Nothing in this module assumes:
//!
//! - a fixed program size;
//! - a fixed number of modules;
//! - a fixed number of items;
//! - a fixed number of declarations;
//! - a fixed number of statements;
//! - a fixed number of qubits;
//! - a fixed number of classical resources;
//! - a fixed machine size;
//! - a fixed processor topology;
//! - a fixed instruction set;
//! - a fixed gate set;
//! - a fixed quantum technology;
//! - a fixed backend;
//! - a fixed hardware vendor.
//!
//! Consequently, the same source-level program representation can participate
//! in compilation for a tiny target, a large target, a heterogeneous target,
//! a distributed target, a quantum target, or a future computational system.
//!
//! "Infinity" in the Zamani architecture means that this AST boundary does not
//! introduce an artificial finite language-level resource limit. Actual
//! execution remains bounded by the resources and policies of the compilation
//! and execution environment.
//!
//! ## Domain neutrality
//!
//! This module must remain computationally domain-neutral.
//!
//! Quantum programming is supported through generic AST constructs and
//! explicitly registered language extensions. The program container itself
//! must not become quantum-specific.
//!
//! In particular, this module must never introduce types such as:
//!
//! ```text
//! Qubit
//! QuantumRegister
//! PhysicalQubit
//! QuantumBackend
//! QuantumTopology
//! QuantumGate
//! SurfaceCode
//! Decoder
//! Calibration
//! QIRValue
//! LLVMValue
//! MLIROperation
//! ```
//!
//! Such concepts belong to their appropriate semantic, quantum, target, or
//! backend layers.
//!
//! ## Module ownership
//!
//! ```text
//! program/
//! ├── mod.rs          <- this file; aggregation/API boundary
//! ├── program.rs      <- complete source-level Program root
//! ├── module.rs       <- source-level Module container
//! ├── item.rs         <- source-level Item relationship/reference
//! └── attributes.rs   <- source-level attribute model
//! ```
//!
//! Each sibling owns one responsibility.
//!
//! `mod.rs` must not duplicate their data structures.
//!
//! ## Public API
//!
//! The stable public API exposed by this module is:
//!
//! ```text
//! program::Program
//! program::Module
//! program::Item
//! program::Attribute
//! program::AttributeSet
//! program::PROGRAM_AST_SCHEMA_VERSION
//! program::PROGRAM_ITEM_SCHEMA_VERSION
//! ```
//!
//! The attribute names above are re-exported only when they are provided by
//! `attributes.rs`. No duplicate definitions are created here.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! foundational AST nodes
//!        │
//!        ▼
//! program/mod.rs
//!        │
//!        ├── program.rs
//!        ├── module.rs
//!        ├── item.rs
//!        └── attributes.rs
//!        │
//!        ▼
//! AST storage / validation
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
//! domain / target lowering
//! ```
//!
//! This module must never reverse that dependency direction.
//!
//! ## Forbidden dependencies
//!
//! This file must never import or depend upon:
//!
//! - `crate::semantic`;
//! - `crate::compiler`;
//! - `crate::ir_gen`;
//! - `crate::zuir`;
//! - `crate::quantum::ir`;
//! - `crate::quantum::hardware`;
//! - `crate::quantum::optimization`;
//! - `crate::quantum::routing`;
//! - `crate::quantum::scheduling`;
//! - `crate::quantum::resilience`;
//! - `crate::quantum::error_correction`;
//! - runtime modules;
//! - backend providers;
//! - OpenQASM implementations;
//! - QIR implementations;
//! - LLVM;
//! - MLIR;
//! - vendor-specific SDKs.
//!
//! The aggregation boundary should require no imports beyond its own child
//! modules.
//!
//! ## Parser integration contract
//!
//! The parser must consume this module through its public program-family API.
//!
//! The intended relationship is:
//!
//! ```text
//! parser
//!   │
//!   ├── constructs Program
//!   ├── constructs Module
//!   ├── constructs Item
//!   └── constructs attributes
//!          │
//!          ▼
//!     program module
//! ```
//!
//! The parser remains responsible for syntactic construction.
//!
//! This module does not parse tokens and does not contain parser recovery
//! logic.
//!
//! ## AST-storage integration contract
//!
//! `Program`, `Module`, and `Item` use stable node identities rather than
//! embedding an ever-growing collection of concrete AST node types.
//!
//! The surrounding AST storage layer therefore owns resolution:
//!
//! ```text
//! Program
//!   │
//!   └── NodeId
//!         │
//!         ▼
//!     AST storage
//!         │
//!         ▼
//! concrete AST node
//! ```
//!
//! `mod.rs` must not introduce a second storage mechanism.
//!
//! ## Semantic integration contract
//!
//! Semantic analysis consumes the program-family nodes after structural AST
//! validation.
//!
//! The intended flow is:
//!
//! ```text
//! Program / Module / Item
//!          │
//!          ▼
//! AST structural validation
//!          │
//!          ▼
//! name resolution
//!          │
//!          ▼
//! type / effect / capability analysis
//!          │
//!          ▼
//! resource and domain semantics
//!          │
//!          ▼
//! semantic model
//! ```
//!
//! No semantic state is stored by this aggregation module.
//!
//! ## ZUIR integration contract
//!
//! This module does not lower anything to ZUIR.
//!
//! The correct dependency is:
//!
//! ```text
//! program AST
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├── classical realization
//!      ├── quantum realization
//!      ├── HDL realization
//!      └── future-domain realization
//! ```
//!
//! This preserves the architectural distinction:
//!
//! ```text
//! AST            = source structure
//! Semantic Model = resolved meaning
//! ZUIR            = universal computational semantics
//! Domain IR       = domain realization
//! Target IR       = target realization
//! ```
//!
//! ## Quantum integration boundary
//!
//! Quantum source programs are allowed to appear underneath this program
//! hierarchy because a quantum program is still a source-level program.
//!
//! However, this module does not know whether a child eventually represents:
//!
//! - a classical operation;
//! - a quantum operation;
//! - a hybrid operation;
//! - an HDL construct;
//! - an accelerator operation;
//! - a distributed computation;
//! - a future computational construct.
//!
//! That distinction is resolved by the appropriate semantic/domain extension.
//!
//! Therefore adding a new quantum technology must not require changing this
//! module.
//!
//! Adding a new computational domain must not require changing this module.
//!
//! Adding a new hardware backend must not require changing this module.
//!
//! ## Extension compatibility
//!
//! Program-level containers must remain capable of containing extension-defined
//! constructs through the canonical AST node/identity mechanism.
//!
//! An extension may provide its own node implementation and semantic lowering,
//! but it must integrate through the existing program/module/item boundaries.
//!
//! The integration model is:
//!
//! ```text
//! extension
//!    │
//!    ├── extension AST node
//!    ├── extension validation
//!    ├── extension visitor support
//!    ├── extension serialization
//!    └── extension semantic lowering
//!             │
//!             ▼
//!       canonical AST storage
//!             │
//!             ▼
//!       Program / Module / Item
//! ```
//!
//! `mod.rs` must not enumerate every possible future extension.
//!
//! ## Serialization boundary
//!
//! Serialization is implemented by the owning child types and the enclosing
//! AST serialization subsystem.
//!
//! This module does not implement a second serialization format.
//!
//! Program-family serialization must preserve:
//!
//! - stable node identity;
//! - child ordering;
//! - source information;
//! - source-level metadata;
//! - extension identity where supported.
//!
//! It must never serialize:
//!
//! - pointers;
//! - memory addresses;
//! - backend handles;
//! - hardware state;
//! - execution jobs;
//! - calibration state;
//! - semantic side tables.
//!
//! ## Determinism
//!
//! This module contains no mutable global state and generates no identities.
//!
//! Determinism is therefore inherited from the canonical child implementations
//! and the enclosing AST storage.
//!
//! Program ordering remains explicit through the child containers owned by
//! `program.rs` and `module.rs`.
//!
//! No behavior in this module depends on:
//!
//! - wall-clock time;
//! - randomness;
//! - memory addresses;
//! - hardware discovery;
//! - backend ordering;
//! - hash iteration order.
//!
//! ## Scalability
//!
//! This module introduces no artificial maximum for:
//!
//! - modules;
//! - items;
//! - program size;
//! - AST nodes;
//! - quantum resources;
//! - computational domains.
//!
//! It deliberately delegates storage growth to the concrete AST containers and
//! their configured compiler/resource policies.
//!
//! Operational safety limits may exist elsewhere, but those limits are policy
//! rather than language semantics and must not be encoded here.
//!
//! ## Error ownership
//!
//! This module intentionally introduces no new program-specific error type.
//!
//! Construction and mutation errors belong to the concrete types that own the
//! corresponding invariants:
//!
//! ```text
//! Program       -> program.rs
//! Module        -> module.rs
//! Item          -> item.rs
//! Attributes    -> attributes.rs
//! ```
//!
//! This prevents `mod.rs` from becoming a second shared error abstraction that
//! couples otherwise independent node families.
//!
//! ## Visitor and traversal integration
//!
//! Visitors and traversal infrastructure should consume the re-exported
//! program-family types:
//!
//! ```text
//! visitor
//!    │
//!    ▼
//! Program
//!    │
//!    ├── Module
//!    │     └── Item
//!    │
//!    └── Item
//! ```
//!
//! Concrete traversal behavior belongs to the visitor/traversal subsystem and
//! to the concrete child implementations.
//!
//! `mod.rs` must not duplicate traversal algorithms.
//!
//! ## Source preservation
//!
//! Source spans, node identities, origins, and metadata are owned by the
//! concrete program-family types and foundational AST infrastructure.
//!
//! This aggregation boundary does not reinterpret source locations.
//!
//! ## Thread safety
//!
//! No explicit synchronization primitive is required here.
//!
//! The thread-safety properties of the exposed types follow their concrete
//! implementations and foundational AST types.
//!
//! Read-only compiler phases may share immutable AST structures when those
//! constituent types satisfy the required `Send`/`Sync` bounds.
//!
//! ## Security
//!
//! This file performs no unchecked indexing, pointer manipulation, unsafe
//! operations, recursive processing, allocation based on source-controlled
//! values, or semantic interpretation.
//!
//! It therefore provides a deliberately small attack surface.
//!
//! Malformed-input protection remains the responsibility of:
//!
//! - lexer;
//! - parser;
//! - AST structural validation;
//! - serialization validation;
//! - configurable compiler/resource limits.
//!
//! ## Rust compatibility
//!
//! Target toolchains:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! This module uses only stable Rust module/re-export facilities.
//!
//! No `unsafe` code is used.
//!
//! ## API stability
//!
//! This module is intentionally kept small so that adding a new concrete AST
//! node elsewhere does not require modifying the program root unless that node
//! becomes part of the program-family public API.
//!
//! In particular, changing:
//!
//! - a quantum backend;
//! - a quantum technology;
//! - a quantum optimizer;
//! - a routing algorithm;
//! - a scheduler;
//! - a QEC implementation;
//! - a resilience implementation;
//! - a hardware provider;
//! - a target instruction set
//!
//! must not require changing this file.
//!
//! ## File-completion contract
//!
//! This file is considered complete when all of the following hold:
//!
//! 1. Every existing program-family source file is declared exactly once.
//! 2. The public program-family API is re-exported exactly once.
//! 3. No concrete AST structure is duplicated here.
//! 4. No semantic dependency exists.
//! 5. No quantum/hardware dependency exists.
//! 6. No backend dependency exists.
//! 7. No unsafe code exists.
//! 8. No machine-size constant exists.
//! 9. No qubit-count constant exists.
//! 10. No target-specific assumption exists.
//! 11. The module compiles on Rust 1.97/1.97.1.
//! 12. Downstream code can import the program family through one stable path.
//!
//! ## Stable import paths
//!
//! Downstream code should prefer:
//!
//! ```text
//! crate::frontend::ast::node::program::Program
//! crate::frontend::ast::node::program::Module
//! crate::frontend::ast::node::program::Item
//! ```
//!
//! instead of reaching through implementation files when the enclosing AST
//! API exposes this module publicly.
//!
//! This gives the repository a stable API boundary while permitting internal
//! decomposition of individual program-family implementations.
//!
//! ## Implementation rule
//!
//! Keep this file declarative.
//!
//! If functionality appears to belong here, first determine whether it
//! actually belongs in:
//!
//! - `program.rs`;
//! - `module.rs`;
//! - `item.rs`;
//! - `attributes.rs`;
//! - AST storage;
//! - validation;
//! - traversal;
//! - serialization;
//! - semantic analysis;
//! - ZUIR;
//! - a domain-specific extension.
//!
//! The program `mod.rs` must remain an aggregation boundary rather than a
//! monolithic implementation file.

// -----------------------------------------------------------------------------
// Child modules
// -----------------------------------------------------------------------------
//
// These are the four program-family implementation units currently present in
// the repository. Each is declared exactly once here.
//
// Keeping declarations here gives the compiler one canonical module tree:
//
//     program
//       ├── attributes
//       ├── item
//       ├── module
//       └── program
//
// No implementation is duplicated in this file.

/// Source-level attributes associated with program-family AST nodes.
pub mod attributes;

/// Source-level item relationship/reference.
pub mod item;

/// Source-level module/namespace container.
pub mod module;

/// Source-level complete program root.
pub mod program;

// -----------------------------------------------------------------------------
// Stable public re-exports
// -----------------------------------------------------------------------------
//
// Re-export concrete types through this module so downstream consumers do not
// have to depend on implementation-file paths.
//
// These re-exports are intentionally limited to the public API actually owned
// by the child modules. No new abstraction is introduced here.

pub use attributes::*;
pub use item::*;
pub use module::*;
pub use program::*;

// -----------------------------------------------------------------------------
// Compile-time API contract
// -----------------------------------------------------------------------------
//
// Keep these assertions deliberately type-level and dependency-free.
//
// They do not impose a computational limit. They merely make the aggregation
// boundary explicit to the compiler and future maintainers.

#[doc(hidden)]
pub const PROGRAM_NODE_FAMILY_API_VERSION: u16 = 1;

/// Returns the stable API version of the program-node family aggregation
/// boundary.
///
/// This version is intentionally separate from:
///
/// - Zamani language version;
/// - individual node schema versions;
/// - AST serialization version;
/// - compiler version;
/// - extension versions.
///
/// Changing an implementation detail must not require changing this version.
///
/// Increment this value only when the public aggregation contract itself
/// changes incompatibly.
#[inline]
#[must_use]
pub const fn api_version() -> u16 {
    PROGRAM_NODE_FAMILY_API_VERSION
}

// -----------------------------------------------------------------------------
// Internal compile-time checks
// -----------------------------------------------------------------------------
//
// The program-family child modules define their own schema versions and
// invariants. This boundary intentionally does not duplicate those checks.
//
// The following test verifies only the aggregation contract.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregation_boundary_has_stable_api_version() {
        assert_eq!(api_version(), PROGRAM_NODE_FAMILY_API_VERSION);
        assert_ne!(PROGRAM_NODE_FAMILY_API_VERSION, 0);
    }

    #[test]
    fn program_family_modules_are_accessible_through_public_boundary() {
        let _ = core::any::type_name::<Program>();
        let _ = core::any::type_name::<Module>();
        let _ = core::any::type_name::<Item>();
    }
}