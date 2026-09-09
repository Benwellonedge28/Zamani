//! # Zamani Frontend AST — Module Declaration Compatibility Boundary
//!
//! This module exposes the canonical source-level Zamani `Module` AST node
//! through the declarations namespace without defining a second module AST
//! representation.
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
//! Native Zamani AST
//!     │
//!     ├── program::Module  ← canonical implementation
//!     │
//!     └── declarations::module ← this compatibility boundary
//!                                      │
//!                                      ▼
//!                               same canonical Module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Purpose
//!
//! `Module` is a source-level program/module container. The canonical
//! implementation already belongs to:
//!
//! ```text
//! crate::frontend::ast::node::program::module::Module
//! ```
//!
//! This file deliberately does **not** define another `Module` structure.
//! Instead, it re-exports the canonical implementation so code organized
//! around declaration nodes can access the same authoritative type.
//!
//! This prevents:
//!
//! - duplicate module AST types;
//! - divergent module invariants;
//! - duplicate constructors;
//! - incompatible serialization schemas;
//! - duplicate visitor behavior;
//! - incompatible `NodeId` relationships;
//! - parser ambiguity;
//! - semantic-analysis ambiguity;
//! - ZUIR-lowering ambiguity.
//!
//! ## One canonical representation
//!
//! There must be exactly one authoritative native AST representation of a
//! source-level module.
//!
//! ```text
//! program::module::Module
//!          ▲
//!          │
//!          ├── program API
//!          ├── parser API
//!          ├── semantic API
//!          ├── traversal API
//!          ├── serialization API
//!          └── declarations::module API
//! ```
//!
//! `declarations::module::Module` and
//! `program::module::Module` are therefore the **same Rust type**, not two
//! compatible-but-distinct types.
//!
//! ## Why Module belongs to the program family
//!
//! A module is both a declaration-level concept and a program-organization
//! concept. In the current AST architecture, the canonical ownership decision
//! is that the concrete `Module` implementation belongs to the `program`
//! family.
//!
//! This is preferable to duplicating the type under both:
//!
//! ```text
//! node/program/module.rs
//! node/declarations/module.rs
//! ```
//!
//! because a duplicate would create two competing definitions of the same
//! language construct.
//!
//! The declarations namespace therefore acts only as an API compatibility
//! boundary.
//!
//! ## Domain neutrality
//!
//! This boundary contains no knowledge of:
//!
//! - quantum computing;
//! - qubits;
//! - quantum registers;
//! - quantum gates;
//! - quantum topology;
//! - physical devices;
//! - QEC;
//! - resilience;
//! - calibration;
//! - scheduling;
//! - routing;
//! - backends;
//! - vendors;
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - LLVM;
//! - MLIR;
//! - QIR;
//! - OpenQASM.
//!
//! A module can contain source-level constructs that eventually compile to
//! classical, quantum, hybrid, distributed, HDL, accelerator, or future
//! computational systems without changing this file.
//!
//! ## POCO-REAF
//!
//! The module abstraction does not encode:
//!
//! - machine size;
//! - processor count;
//! - qubit count;
//! - register size;
//! - memory size;
//! - hardware topology;
//! - instruction set;
//! - gate set;
//! - backend;
//! - vendor;
//! - execution environment.
//!
//! Consequently, a source module can participate in compilation across
//! different computational scales without changing its AST representation.
//!
//! ```text
//!                    Same source module
//!                           │
//!                           ▼
//!                     Native AST Module
//!                           │
//!                           ▼
//!                    Semantic Analysis
//!                           │
//!                           ▼
//!                          ZUIR
//!                           │
//!              ┌────────────┼────────────┐
//!              ▼            ▼            ▼
//!          Classical      Quantum       HDL
//!             IR            IR           IR
//!              │            │            │
//!              └────────────┼────────────┘
//!                           ▼
//!                  Target realization
//! ```
//!
//! ## Dependency contract
//!
//! This file depends only on the canonical program-level AST module.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - error correction;
//! - resilience;
//! - calibration;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - MLIR;
//! - QIR;
//! - OpenQASM.
//!
//! ## Integration contract — parser
//!
//! The parser must construct the canonical `Module` type rather than a type
//! defined here.
//!
//! ```text
//! parser
//!    │
//!    ├── module source span
//!    ├── module name
//!    ├── module metadata
//!    └── child NodeIds
//!           │
//!           ▼
//! program::Module
//! ```
//!
//! The declarations namespace may be imported by parser code for organizational
//! convenience, but the underlying Rust type remains the canonical program
//! module.
//!
//! ## Integration contract — AST storage
//!
//! `Module` stores structural relationships using canonical AST identities
//! such as `NodeId`.
//!
//! This file must never introduce a second AST store, arena, registry, or
//! ownership mechanism.
//!
//! ```text
//! declarations::module::Module
//!          │
//!          │ type alias/re-export only
//!          ▼
//! program::module::Module
//!          │
//!          ▼
//! canonical AST storage
//! ```
//!
//! ## Integration contract — structural validation
//!
//! Structural validation operates on the canonical `Module` implementation.
//!
//! This file introduces no additional validation rules because adding a second
//! validation layer here would create a risk that:
//!
//! ```text
//! declarations::Module validation
//!             ≠
//! program::Module validation
//! ```
//!
//! There must instead be one authoritative structural contract.
//!
//! The canonical module implementation is responsible for its local invariants;
//! AST-wide validation is responsible for cross-node relationships.
//!
//! ## Integration contract — semantic analysis
//!
//! Semantic analysis must consume the canonical module type:
//!
//! ```text
//! frontend::ast::node::program::Module
//!                     │
//!                     ▼
//!                 resolver
//!                     │
//!                     ▼
//!             semantic module model
//! ```
//!
//! Semantic analysis must not depend on this file for semantic state.
//!
//! In particular, this module must never introduce:
//!
//! - resolved module symbols;
//! - namespace tables;
//! - resolved imports;
//! - visibility results;
//! - type information;
//! - capability resolution;
//! - resource resolution.
//!
//! Those belong to the semantic model.
//!
//! ## Integration contract — ZUIR
//!
//! This file has no direct ZUIR lowering responsibility.
//!
//! The canonical module is lowered through semantic analysis:
//!
//! ```text
//! AST Module
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! The module container itself primarily provides source organization and
//! child-node ordering. Its children acquire computational meaning during
//! semantic lowering.
//!
//! ## Integration contract — visitors
//!
//! Visitors must operate on the canonical `Module` implementation.
//!
//! This re-export does not introduce a second visitor method such as:
//!
//! ```text
//! visit_declaration_module()
//! ```
//!
//! merely because the type is reachable through the declarations namespace.
//!
//! There must be one canonical visitor identity for the `Module` node.
//!
//! ## Integration contract — traversal
//!
//! Traversal infrastructure must traverse the canonical module's children in
//! their stored source order.
//!
//! This file must not introduce another traversal implementation.
//!
//! ## Integration contract — serialization
//!
//! Serialization is inherited from the canonical `Module` implementation.
//!
//! This file must not define:
//!
//! - another serializer;
//! - another deserializer;
//! - another schema;
//! - another schema version;
//! - another wire representation.
//!
//! There must be one authoritative module serialization contract.
//!
//! Re-exporting the canonical type therefore also preserves its existing
//! `serde` behavior and schema contract.
//!
//! ## Integration contract — source mapping
//!
//! Source mapping is inherited from the canonical `Module` implementation and
//! foundational AST source infrastructure.
//!
//! This file must not reinterpret source spans or source origins.
//!
//! ## Scalability
//!
//! This file introduces no computational resource limit.
//!
//! In particular, it contains no constants for:
//!
//! - maximum modules;
//! - maximum declarations;
//! - maximum module children;
//! - maximum qubits;
//! - maximum resources;
//! - maximum machines;
//! - maximum program size.
//!
//! The canonical module representation remains responsible for scalable
//! collections and source ordering.
//!
//! Operational safety limits belong to configurable compiler policy rather than
//! this compatibility boundary.
//!
//! ## Determinism
//!
//! This file contains:
//!
//! - no global state;
//! - no allocation state;
//! - no randomness;
//! - no time-dependent behavior;
//! - no hash-based dispatch;
//! - no backend discovery.
//!
//! Re-exporting a type cannot change its deterministic behavior.
//!
//! ## Thread safety
//!
//! This module introduces no state of its own.
//!
//! Thread-safety therefore follows exactly the canonical `Module` type and its
//! foundational field types.
//!
//! Read-only AST consumers may share immutable module references when the
//! underlying AST types satisfy the required `Send` and `Sync` bounds.
//!
//! ## Security
//!
//! This boundary introduces no unsafe operations and performs no parsing,
//! allocation based on untrusted values, recursion, indexing, or semantic
//! interpretation.
//!
//! Consequently, malformed-input handling remains centralized in:
//!
//! - lexer;
//! - parser;
//! - canonical AST construction;
//! - structural validation;
//! - serialization validation;
//! - configurable compiler resource policies.
//!
//! ## Migration contract
//!
//! Older code may have expected a module declaration under the declarations
//! namespace:
//!
//! ```text
//! frontend::ast::node::declarations::module::Module
//! ```
//!
//! That path can remain valid through this re-export while the canonical
//! implementation remains:
//!
//! ```text
//! frontend::ast::node::program::module::Module
//! ```
//!
//! This allows migration without maintaining two implementations.
//!
//! The migration rule is:
//!
//! ```text
//! old/alternate import path
//!          │
//!          ▼
//! declarations::module::Module
//!          │
//!          ▼
//! program::module::Module
//!          │
//!          ▼
//! one canonical implementation
//! ```
//!
//! No conversion function is necessary because the types are identical.
//!
//! ## Forbidden implementation
//!
//! The following must **not** be introduced here:
//!
//! ```text
//! pub struct Module { ... }
//! ```
//!
//! A second structure would violate the canonical-AST rule.
//!
//! Likewise, this file must not introduce:
//!
//! ```text
//! enum ModuleKind { ... }
//! struct ModuleSemanticInfo { ... }
//! struct QuantumModule { ... }
//! struct HardwareModule { ... }
//! ```
//!
//! Those would either duplicate existing AST concepts or leak later compiler
//! phases into the native AST.
//!
//! ## File-completion contract
//!
//! This file is complete when:
//!
//! 1. It exposes the canonical `Module` type.
//! 2. It does not define a second `Module` structure.
//! 3. It does not introduce another module schema.
//! 4. It does not introduce another module validator.
//! 5. It does not introduce another module visitor.
//! 6. It does not introduce another module serializer.
//! 7. It does not introduce semantic state.
//! 8. It does not introduce ZUIR state.
//! 9. It does not introduce quantum/hardware state.
//! 10. It contains no unsafe code.
//! 11. It contains no machine-size assumptions.
//! 12. It contains no qubit-count assumptions.
//! 13. It preserves the canonical `Module` API.
//! 14. Existing code can migrate to the canonical program module without
//!     maintaining duplicate implementations.
//!
//! ## Rust compatibility
//!
//! Target toolchains:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! The implementation uses only stable Rust module/re-export facilities.
//!
//! No `unsafe` code is used.
//!
//! ## Public API
//!
//! The public API intentionally consists of a re-export only.
//!
//! This makes the declarations namespace an access path rather than an
//! independent owner of module semantics.
//!
//! ## Implementation
//!
//! Keep this implementation deliberately small. The extensive documentation
//! above records the architectural contract so future contributors do not
//! accidentally recreate a second module AST.

// Re-export the single canonical source-level Module implementation.
//
// `program::module::Module` is authoritative. This file must never define a
// second Module structure.
pub use super::super::program::module::Module;

// Re-export the canonical module schema version when it exists.
//
// Keeping the schema constant on the canonical implementation prevents the
// declarations namespace from creating an independent version space.
pub use super::super::program::module::MODULE_AST_SCHEMA_VERSION;