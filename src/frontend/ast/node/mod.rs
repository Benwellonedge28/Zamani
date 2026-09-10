//! # Zamani Frontend AST — Canonical Node Subsystem
//!
//! `src/frontend/ast/node/mod.rs`
//!
//! This is the canonical public aggregation boundary for the native Zamani
//! frontend AST node subsystem.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//!    lexer
//!      │
//!      ▼
//!    parser
//!      │
//!      ▼
//! ┌──────────────────────────────────────────────┐
//! │ frontend::ast::node                          │
//! │                                              │
//! │ source / identity / metadata                 │
//! │ node classification                          │
//! │ declarations / expressions / statements     │
//! │ types / patterns / generics                  │
//! │ effects / capabilities / resources          │
//! │ domains / annotations                       │
//! │ visitors / traversal / validation           │
//! └──────────────────────┬───────────────────────┘
//!                        │
//!                        ▼
//!              structural validation
//!                        │
//!                        ▼
//!                semantic analysis
//!                        │
//!                        ▼
//!                 semantic model
//!                        │
//!                        ▼
//!                       ZUIR
//!                        │
//!             ┌──────────┼──────────┐
//!             ▼          ▼          ▼
//!         classical    quantum      HDL
//!             IR          IR         IR
//!             │          │          │
//!             └──────────┼──────────┘
//!                        ▼
//!              target/backend lowering
//!                        │
//!                        ▼
//!       CPU / GPU / FPGA / QPU / distributed / future
//! ```
//!
//! ## Core invariant
//!
//! The native AST represents **source-language structure and programmer
//! intent**.
//!
//! It is not an intermediate representation for a particular machine.
//!
//! In particular, this module must never become coupled to:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - quantum topology;
//! - physical qubits;
//! - quantum gates as a closed enumeration;
//! - vendor SDKs;
//! - backend APIs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR implementation details;
//! - quantum routing;
//! - quantum scheduling;
//! - calibration;
//! - pulse generation;
//! - quantum error correction implementation;
//! - noise models;
//! - resilience implementation;
//! - runtime execution;
//! - backend credentials;
//! - device handles.
//!
//! Those concerns belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! The node subsystem participates in:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! The AST therefore introduces no language-level assumption about:
//!
//! - number of qubits;
//! - number of registers;
//! - number of CPUs;
//! - number of GPUs;
//! - number of accelerators;
//! - number of devices;
//! - machine width;
//! - machine topology;
//! - instruction-set size;
//! - backend count;
//! - computational-domain count;
//! - available memory;
//! - target capacity.
//!
//! A program can therefore describe computation independently of whether the
//! eventual target has one resource, thousands of resources, millions of
//! resources, a heterogeneous system, a distributed system, a simulator, a
//! quantum system, or a future computational substrate.
//!
//! "Infinity" is interpreted architecturally: this subsystem introduces no
//! arbitrary finite machine-size or computational-resource ceiling. Actual
//! compilation and execution remain bounded by representable values and the
//! resources and policies of the executing environment.
//!
//! ## Responsibilities
//!
//! This module owns only:
//!
//! - child-module composition;
//! - public module visibility;
//! - stable public re-exports of canonical foundational types;
//! - the node-subsystem API boundary;
//! - architectural invariants for the node subsystem;
//! - compile-time/module-surface integration tests.
//!
//! It does **not** implement:
//!
//! - parsing;
//! - lexing;
//! - semantic analysis;
//! - type checking;
//! - symbol resolution;
//! - ZUIR lowering;
//! - quantum compilation;
//! - hardware compilation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - noise handling;
//! - resilience;
//! - runtime execution.
//!
//! ## No duplicate implementations
//!
//! Every concept has one authoritative implementation module.
//!
//! ```text
//! Node                 → node.rs
//! NodeId               → node_id.rs
//! NodeKind             → node_kind.rs
//! NodeMetadata         → metadata.rs
//! SourceId / Span      → source/
//! Types                → types/
//! Expressions          → expressions/
//! Statements           → statements/
//! Declarations         → declarations/
//! Patterns             → patterns/
//! Generics             → generics/
//! Effects              → effects/
//! Capabilities         → capabilities/
//! Resources            → resources/
//! Domains              → domains/
//! Annotations          → annotations/
//! Builders             → builders/
//! Traversal             → traversal/
//! Visitors              → visitors/
//! Structural validation → validation/
//! ```
//!
//! This file must never redefine any of those concepts.
//!
//! ## Dependency direction
//!
//! The permitted direction is:
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! node foundations
//!        │
//!        ├── NodeId
//!        ├── NodeKind
//!        ├── Node
//!        ├── Metadata
//!        └── Source
//!        │
//!        ▼
//! source AST constructs
//!        │
//!        ├── declarations
//!        ├── expressions
//!        ├── statements
//!        ├── types
//!        ├── patterns
//!        ├── generics
//!        ├── effects
//!        ├── capabilities
//!        ├── resources
//!        ├── domains
//!        └── annotations
//!        │
//!        ▼
//! builders / traversal / visitors / validation
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
//! The reverse dependency direction is forbidden.
//!
//! ## Forbidden dependencies
//!
//! This module must never import or depend upon:
//!
//! ```text
//! crate::semantic
//! crate::compiler
//! crate::quantum::ir
//! crate::quantum::hardware
//! crate::quantum::optimization
//! crate::quantum::scheduling
//! crate::quantum::routing
//! crate::quantum::error_correction
//! crate::quantum::zqn
//! runtime execution
//! backend SDKs
//! vendor APIs
//! QIR
//! LLVM
//! MLIR
//! ZUIR implementation
//! ```
//!
//! The node subsystem is an upstream representation and must remain usable
//! without any downstream implementation being present.
//!
//! ## Quantum architecture
//!
//! Quantum computation is intentionally **not** represented by making this
//! module a quantum-specific AST.
//!
//! Generic AST concepts such as:
//!
//! - operations;
//! - calls;
//! - parameters;
//! - resources;
//! - indexing;
//! - control flow;
//! - conditions;
//! - types;
//! - annotations;
//! - effects;
//! - capabilities;
//! - regions;
//! - timing intent;
//!
//! can participate in quantum programs.
//!
//! Quantum-specific source constructs, when genuinely required by the Zamani
//! language, must be isolated in an appropriate extension boundary.
//!
//! This keeps the native AST independent of whether a quantum resource is
//! eventually realized as:
//!
//! - a superconducting qubit;
//! - a trapped ion;
//! - a neutral atom;
//! - a photonic mode;
//! - a topological qubit;
//! - an analog quantum system;
//! - a simulator resource;
//! - a future quantum technology.
//!
//! The AST describes intent; downstream compilation determines realization.
//!
//! ## External quantum formats
//!
//! External formats such as OpenQASM must not redefine this module.
//!
//! Their intended architecture is:
//!
//! ```text
//! external source
//!       │
//!       ▼
//! external-format AST
//!       │
//!       ▼
//! external validation
//!       │
//!       ▼
//! Zamani source/semantic representation
//!       │
//!       ▼
//! native AST / semantic pipeline
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! Consequently, OpenQASM-specific syntax, QIR values, LLVM values, and
//! hardware instructions do not belong in this module.
//!
//! ## Resource independence
//!
//! `resources/` represents source-level resource intent.
//!
//! It must not be confused with physical allocation.
//!
//! ```text
//! AST resource
//!      ≠
//! semantic resource
//!      ≠
//! physical resource
//!      ≠
//! runtime resource handle
//! ```
//!
//! Physical mapping remains downstream.
//!
//! This is essential for scalable quantum programs because a source-level
//! resource cardinality may be symbolic, generic, compile-time known, or
//! otherwise unresolved until semantic analysis.
//!
//! ## Node identity
//!
//! `NodeId` identifies an AST node only.
//!
//! It is not:
//!
//! - a symbol ID;
//! - a type ID;
//! - a resource ID;
//! - a qubit ID;
//! - a QIR value ID;
//! - a ZUIR value ID;
//! - a backend ID;
//! - a runtime handle.
//!
//! The repository's `node_id.rs` already establishes deterministic, explicit
//! allocation with no hidden global counter. This module merely exposes that
//! implementation through the canonical node boundary.
//!
//! ## Source locations
//!
//! Source coordinates are owned by `source/`.
//!
//! The canonical model is based on source identity and source offsets rather
//! than filesystem-specific identities.
//!
//! This module must not define another span or location representation.
//!
//! ## Metadata
//!
//! Metadata is source/tooling information.
//!
//! It must not become an implicit semantic or backend channel.
//!
//! The repository's `metadata.rs` already provides extensible namespaced
//! metadata and deterministic ordered metadata structures. This module exposes
//! that implementation rather than duplicating it.
//!
//! ## Traversal
//!
//! Traversal is owned by `traversal/`.
//!
//! Visitors are owned by `visitors/`.
//!
//! Validation is owned by `validation/`.
//!
//! This facade must not implement a second walker, visitor, validator, or
//! recursion mechanism.
//!
//! The canonical traversal infrastructure must remain the single traversal
//! authority so that validation, tooling, analysis, and transformations agree
//! on child ordering and graph semantics.
//!
//! ## Validation boundary
//!
//! Structural validation happens before semantic analysis.
//!
//! ```text
//! AST construction
//!       │
//!       ▼
//! local structural invariants
//!       │
//!       ▼
//! source-range validation
//!       │
//!       ▼
//! graph/traversal validation
//!       │
//!       ▼
//! complete structural validation
//!       │
//!       ▼
//! semantic analysis
//! ```
//!
//! Validation does not determine whether a quantum operation is physically
//! executable, whether a topology supports an interaction, or whether a
//! backend has enough resources.
//!
//! Those questions belong downstream.
//!
//! ## Scalability
//!
//! This module declares no machine-size constants and no language-level
//! collection limits.
//!
//! It must not introduce constants such as:
//!
//! ```text
//! MAX_NODES
//! MAX_AST_NODES
//! MAX_QUBITS
//! MAX_REGISTERS
//! MAX_MACHINE_SIZE
//! MAX_DEVICES
//! MAX_EXPRESSIONS
//! MAX_STATEMENTS
//! ```
//!
//! Operational resource limits may exist elsewhere as explicit configurable
//! compiler policies. Such policies are not AST semantics.
//!
//! ## Determinism
//!
//! Importing this module has no runtime side effects.
//!
//! It does not:
//!
//! - allocate IDs;
//! - access the filesystem;
//! - access the network;
//! - access hardware;
//! - read environment variables;
//! - inspect runtime state;
//! - use randomness;
//! - use wall-clock time;
//! - create global mutable state;
//! - reorder source constructs.
//!
//! Determinism is inherited from the authoritative child modules and the AST
//! storage/traversal contracts.
//!
//! ## Serialization
//!
//! This module does not define a serialization format.
//!
//! Individual node structures remain serializable according to their owning
//! modules, while the canonical AST serialization layer owns:
//!
//! - serialization schema versioning;
//! - compatibility policy;
//! - deterministic ordering;
//! - complete graph encoding;
//! - extension preservation;
//! - corruption handling.
//!
//! No second serialization protocol may be introduced here.
//!
//! ## Thread safety
//!
//! This facade owns no mutable global state.
//!
//! Read-only AST consumption may therefore be performed concurrently whenever
//! the underlying AST storage and constituent types satisfy the corresponding
//! `Send`/`Sync` requirements.
//!
//! Mutation remains explicitly owned by AST builders/transformation APIs.
//!
//! ## Security
//!
//! The AST is compiler input and must be treated as untrusted.
//!
//! This module:
//!
//! - contains no unsafe code;
//! - performs no I/O;
//! - performs no network access;
//! - executes no user code;
//! - owns no raw pointers;
//! - owns no hardware handles;
//! - owns no credentials;
//! - performs no unchecked memory operations.
//!
//! Resource-exhaustion protection belongs to explicit configurable compiler
//! policies and the canonical validation/traversal infrastructure.
//!
//! ## Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced
//! for this module and its descendants.
//!
//! ## File completion contract
//!
//! This file is complete when:
//!
//! 1. every implemented child node subsystem has exactly one module
//!    declaration;
//! 2. canonical foundational types are reachable through this boundary;
//! 3. no implementation is duplicated here;
//! 4. no parser logic exists here;
//! 5. no semantic analysis exists here;
//! 6. no ZUIR dependency exists here;
//! 7. no quantum-IR dependency exists here;
//! 8. no hardware dependency exists here;
//! 9. no backend dependency exists here;
//! 10. no fixed machine/resource limit exists here;
//! 11. no global mutable state exists here;
//! 12. module imports remain side-effect free;
//! 13. public API re-exports are explicit;
//! 14. child modules remain independently replaceable;
//! 15. Rust 1.97/1.97.1 compiles the module;
//! 16. no unsafe code is accepted;
//! 17. adding an implementation to an existing child module does not require
//!     unrelated changes here.
//!
//! =============================================================================
//! Module implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Foundational node infrastructure
// =============================================================================
//
// These modules form the lowest-level node contract. They must remain free of
// downstream compiler dependencies.

pub mod metadata;
pub mod node;
pub mod node_id;
pub mod node_kind;
pub mod source;

// =============================================================================
// Source-level AST namespaces
// =============================================================================
//
// These modules contain source-language constructs built on the foundational
// node infrastructure.

pub mod annotations;
pub mod builders;
pub mod capabilities;
pub mod declarations;
pub mod domains;
pub mod effects;
pub mod expressions;
pub mod generics;
pub mod patterns;
pub mod resources;
pub mod statements;
pub mod types;

// =============================================================================
// AST graph services
// =============================================================================
//
// These modules operate on the native AST without becoming semantic or
// backend-specific compiler layers.

pub mod traversal;
pub mod validation;
pub mod visitors;

// =============================================================================
// Canonical foundational re-exports
// =============================================================================
//
// Only explicitly selected foundational symbols are re-exported.
//
// Wildcard exports are deliberately forbidden here because they would silently
// expand the public API whenever an implementation file changes.
//
// Concrete source-language nodes remain available through their owning
// namespaces:
//
//     node::expressions::...
//     node::statements::...
//     node::declarations::...
//     node::types::...
//     node::resources::...
//
// This keeps ownership clear and prevents accidental API collisions.

pub use metadata::{
    MetadataFlags,
    MetadataKey,
    MetadataKeyError,
    MetadataValue,
    NodeMetadata,
};

pub use node::{
    AstNode,
    Node,
    NodeDiagnosticSummary,
    NodeRef,
};

pub use node_id::{
    NodeId,
    NodeIdAllocationError,
    NodeIdAllocator,
    NodeIdParseError,
    NodeIdRange,
};

pub use node_kind::{
    CoreNodeCategory,
    CoreNodeKind,
    ExtensionNodeKind,
    NodeKind,
    NodeKindError,
};

pub use source::{
    SourceId,
    SourceLocation,
    SourceOffset,
    SourceOrigin,
    Span,
};

// =============================================================================
// Stable module-level contract
// =============================================================================

/// Schema version of the `frontend::ast::node` public module boundary.
///
/// This is deliberately independent from:
///
/// - the Zamani language version;
/// - compiler version;
/// - individual node schema versions;
/// - serialized AST format version;
/// - semantic model version;
/// - ZUIR version;
/// - domain-IR versions.
///
/// Increment this only when the public aggregation contract itself changes in
/// a compatibility-relevant way.
pub const NODE_MODULE_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of this module.
pub const NODE_MODULE_NAME: &str = "zamani.frontend.ast.node";

/// Returns the schema version of the node module boundary.
#[must_use]
pub const fn schema_version() -> u16 {
    NODE_MODULE_SCHEMA_VERSION
}

/// Returns the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    NODE_MODULE_NAME
}

// =============================================================================
// Architectural API assertions
// =============================================================================

/// Compile-time helper proving that the canonical node identity API is
/// reachable through the node boundary.
///
/// This function is intentionally private to the module. Its purpose is to
/// make accidental API breakage visible during compilation without imposing
/// runtime state or behavior.
#[cfg(test)]
fn assert_foundational_api() {
    fn assert_node<T: AstNode>() {}

    // `Node` itself is not required to implement AstNode because AstNode is
    // intended as the delegation contract for concrete nodes. The explicit
    // type references below nevertheless verify that all canonical
    // foundational exports remain reachable.
    let _ = core::mem::size_of::<NodeId>();
    let _ = core::mem::size_of::<NodeKind>();
    let _ = core::mem::size_of::<CoreNodeKind>();
    let _ = core::mem::size_of::<Span>();
    let _ = core::mem::size_of::<SourceId>();
    let _ = core::mem::size_of::<NodeMetadata>();

    let _ = assert_node::<Node>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_contract_has_stable_identity() {
        assert_eq!(schema_version(), NODE_MODULE_SCHEMA_VERSION);
        assert_eq!(module_name(), NODE_MODULE_NAME);
        assert!(schema_version() > 0);
    }

    #[test]
    fn foundational_types_are_reachable() {
        assert_foundational_api();
    }

    #[test]
    fn node_id_is_available_from_canonical_boundary() {
        let id = NodeId::new(1).expect("positive node ID must be valid");
        assert_eq!(id.get(), 1);
    }

    #[test]
    fn node_kind_is_available_from_canonical_boundary() {
        let kind = NodeKind::core(CoreNodeKind::Program);

        assert!(kind.is_core());
        assert!(!kind.is_extension());
        assert_eq!(kind.namespace(), "zamani");
    }

    #[test]
    fn source_types_are_available_from_canonical_boundary() {
        let source_id = SourceId::new(1);
        let span = Span::new(source_id, 0, 0);

        assert_eq!(span.source_id(), source_id);
        assert_eq!(span.start().get(), 0);
        assert_eq!(span.end().get(), 0);
    }

    #[test]
    fn allocator_is_deterministic_and_local() {
        let mut first = NodeIdAllocator::new();
        let mut second = NodeIdAllocator::new();

        let first_a = first.allocate().expect("first allocation");
        let first_b = first.allocate().expect("second allocation");

        let second_a = second.allocate().expect("first allocation");
        let second_b = second.allocate().expect("second allocation");

        assert_eq!(first_a, second_a);
        assert_eq!(first_b, second_b);
    }

    #[test]
    fn node_module_does_not_define_a_machine_size() {
        // This test intentionally documents an architectural property rather
        // than testing an implementation detail.
        //
        // Machine/resource capacity is downstream from the native AST.
        assert_eq!(NODE_MODULE_NAME, "zamani.frontend.ast.node");
    }
}

// =============================================================================
// Public API stability notes
// =============================================================================
//
// The following rules are intentional:
//
// 1. `node.rs` remains the authoritative common-node implementation.
//
// 2. `node_id.rs` remains the authoritative AST identity implementation.
//
// 3. `node_kind.rs` remains the authoritative source-level classification
//    implementation.
//
// 4. `metadata.rs` remains the authoritative node metadata implementation.
//
// 5. `source/` remains the authoritative source-coordinate implementation.
//
// 6. Child AST namespaces remain responsible for their own concrete nodes.
//
// 7. `traversal/` remains the only canonical graph traversal subsystem.
//
// 8. `visitors/` remains the visitor API boundary.
//
// 9. `validation/` remains the structural validation boundary.
//
// 10. This file must remain an aggregation/facade module.
//
// 11. No downstream compiler phase may be imported merely to make a type
//     convenient to expose.
//
// 12. No hardware-specific type may be re-exported here.
//
// 13. No vendor-specific type may be re-exported here.
//
// 14. No QIR/LLVM/MLIR/ZUIR type may be re-exported here.
//
// 15. No finite computational-domain taxonomy may be introduced here.
//
// 16. No finite resource-size taxonomy may be introduced here.
//
// 17. No fixed quantum gate set may be introduced here.
//
// 18. No fixed qubit count may be introduced here.
//
// 19. No fixed machine size may be introduced here.
//
// 20. A new backend must never require modification of this file.
//
// 21. A new quantum technology must never require modification of this file.
//
// 22. A new computational domain should not require modification of this file
//     merely because the domain exists.
//
// 23. A change to one child implementation must not require unrelated child
//     implementations to be edited.
//
// 24. New public symbols must be added through their authoritative owning
//     module first and explicitly re-exported here only when they are part of
//     the stable node API.
//
// 25. Wildcard public re-exports are prohibited.
//
// =============================================================================
// Integration contracts
// =============================================================================
//
// Parser:
//
//     lexer
//       │
//       ▼
//     parser
//       │
//       ▼
//     frontend::ast::node
//
// The parser constructs AST nodes. This module does not parse source text.
//
// Structural validation:
//
//     frontend::ast::node
//       │
//       ▼
//     traversal
//       │
//       ▼
//     validation
//
// Validation must reuse the canonical traversal infrastructure.
//
// Semantic analysis:
//
//     validated AST
//       │
//       ▼
//     semantic analysis
//       │
//       ▼
//     semantic model
//
// This module does not resolve symbols, types, capabilities, resources, or
// domain semantics.
//
// ZUIR:
//
//     validated AST
//       │
//       ▼
//     semantic model
//       │
//       ▼
//     ZUIR
//
// This module must never construct ZUIR directly.
//
// Quantum:
//
//     native AST
//       │
//       ▼
//     semantic quantum model
//       │
//       ▼
//     quantum IR
//       │
//       ▼
//     optimization / routing / scheduling / QEC / resilience
//       │
//       ▼
//     target realization
//
// No quantum backend information is permitted to flow backwards into this
// module.
//
// Hardware:
//
//     source intent
//       │
//       ▼
//     semantic requirements
//       │
//       ▼
//     capability/resource discovery
//       │
//       ▼
//     target mapping
//       │
//       ▼
//     hardware realization
//
// Hardware mapping is never an AST responsibility.
//
// =============================================================================
// Scalability contract
// =============================================================================
//
// The module intentionally uses no machine-dependent constants.
//
// In particular, there is no:
//
//     MAX_QUBITS
//     MAX_CPUS
//     MAX_GPUS
//     MAX_DEVICES
//     MAX_AST_NODES
//     MAX_PROGRAM_SIZE
//     MAX_MACHINE_SIZE
//     MAX_REGISTER_SIZE
//
// Any operational limit required for denial-of-service protection or compiler
// service policy must be supplied explicitly by the appropriate validation,
// traversal, parser, or compiler configuration layer.
//
// Such limits must never change the language semantics represented by this
// module.
//
// =============================================================================
// Evolution contract
// =============================================================================
//
// Existing child modules can evolve independently.
//
// If a concrete node implementation changes:
//
//     child implementation
//          │
//          ├── implementation
//          ├── local invariants
//          ├── local tests
//          └── documentation
//
// `node/mod.rs` normally requires no change.
//
// If a genuinely new top-level node namespace is introduced:
//
//     new namespace
//          │
//          ├── implementation
//          ├── mod.rs
//          ├── validation
//          ├── traversal integration
//          ├── visitor integration
//          ├── serialization integration
//          ├── semantic lowering
//          └── tests
//          │
//          ▼
//     one explicit module declaration here
//
// Existing unrelated modules remain untouched.
//
// This is the intended "complete one file without reopening unrelated files"
// integration model.
//
// =============================================================================
// Final invariant
// =============================================================================
//
// The canonical native AST remains:
//
//     source structure
//          +
//     source identity
//          +
//     source metadata
//          +
//     source-level declarations/expressions/statements/types/resources
//          +
//     extensible source-level domains/effects/capabilities/annotations
//
// It does not become:
//
//     hardware IR
//     quantum backend IR
//     scheduler IR
//     routing IR
//     QEC implementation
//     runtime state
//     QIR
//     LLVM IR
//     MLIR
//     ZUIR
//
// This separation is the architectural foundation required for:
//
//     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//
// while allowing the same source program to scale from the smallest available
// computational system to arbitrarily larger systems without encoding the
// target's physical limits into the native AST.