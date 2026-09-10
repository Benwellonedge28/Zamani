//! # Zamani Frontend AST — Declarations
//!
//! Canonical composition boundary for source-level Zamani declarations.
//!
//! ## Architectural position
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
//! ┌───────────────────────────────┐
//! │ Native Zamani AST             │
//! │                               │
//! │ node/declarations/             │
//! │                               │
//! │  ├── class                    │
//! │  ├── constant                 │
//! │  ├── enum                     │
//! │  ├── extern                   │
//! │  ├── function                 │
//! │  ├── impl                     │
//! │  ├── interface                │
//! │  ├── module                   │
//! │  ├── struct                   │
//! │  ├── trait                    │
//! │  ├── type_alias               │
//! │  └── variable                 │
//! └───────────────┬───────────────┘
//!                 │
//!                 ▼
//!       structural validation
//!                 │
//!                 ▼
//!        semantic analysis
//!                 │
//!                 ▼
//!          semantic model
//!                 │
//!                 ▼
//!                ZUIR
//!                 │
//!                 ▼
//!       domain / target lowering
//!                 │
//!                 ▼
//!       available execution resources
//! ```
//!
//! ## Purpose
//!
//! This module owns **only the declaration namespace** of the native Zamani
//! source-level AST.
//!
//! It is intentionally a composition module. Individual declaration semantics
//! belong to their respective child modules.
//!
//! This module must not become a second declaration implementation layer.
//!
//! ## Declaration inventory
//!
//! The native AST currently exposes these declaration modules:
//!
//! - [`class`]
//! - [`constant`]
//! - [`r#enum`]
//! - [`r#extern`]
//! - [`function`]
//! - [`r#impl`]
//! - [`interface`]
//! - [`module`]
//! - [`r#struct`]
//! - [`r#trait`]
//! - [`type_alias`]
//! - [`variable`]
//!
//! The Rust raw identifiers are required because several source filenames are
//! also Rust language keywords.
//!
//! ## Domain neutrality
//!
//! Declaration nodes describe **Zamani source intent**.
//!
//! They must not directly describe:
//!
//! - CPU instructions;
//! - GPU instructions;
//! - FPGA instructions;
//! - ASIC instructions;
//! - QPU instructions;
//! - physical qubits;
//! - quantum hardware topology;
//! - quantum routing;
//! - quantum scheduling;
//! - calibration data;
//! - error-correction implementations;
//! - resilience implementations;
//! - backend jobs;
//! - provider APIs;
//! - QIR objects;
//! - LLVM objects;
//! - MLIR objects;
//! - vendor SDK objects.
//!
//! A declaration may describe a program that eventually contains classical,
//! quantum, hybrid, HDL, accelerator, distributed, AI, or future computational
//! constructs. The eventual realization belongs downstream of the native AST.
//!
//! ## POCO-REAF
//!
//! This namespace must preserve the Zamani requirement:
//!
//! ```text
//! Program Once
//! Compile Once
//! Run Everywhere
//! Anywhere
//! Forever
//! ```
//!
//! In particular, declarations must not encode a fixed:
//!
//! - machine size;
//! - CPU count;
//! - GPU count;
//! - QPU count;
//! - qubit count;
//! - register width;
//! - topology;
//! - gate set;
//! - instruction set;
//! - vendor;
//! - backend;
//! - execution provider.
//!
//! Resource availability, capability matching, mapping, scheduling,
//! decomposition, optimization and execution strategy are downstream
//! responsibilities.
//!
//! ## Scalability
//!
//! This module contains no fixed-size declaration registry.
//!
//! Adding another declaration module does not require changing an enumeration
//! of all possible computational domains or machine types.
//!
//! The number of declarations in a Zamani program is therefore bounded by
//! available resources and explicit compiler resource policies rather than by
//! a constant in this module.
//!
//! Compiler safety limits, where required, must be configurable policies owned
//! by the appropriate compiler/frontend infrastructure. They are not language
//! semantics and must not be introduced here.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! native AST foundations
//!        │
//!        ▼
//! declarations
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
//! target IR / runtime / hardware
//! ```
//!
//! This module must never reverse that direction.
//!
//! ## Forbidden dependencies
//!
//! This composition module must not depend on:
//!
//! - `crate::semantic`;
//! - `crate::compiler`;
//! - `crate::zuir`;
//! - `crate::quantum::ir`;
//! - `crate::quantum::hardware`;
//! - `crate::quantum::optimization`;
//! - `crate::quantum::scheduling`;
//! - `crate::quantum::routing`;
//! - `crate::quantum::error_correction`;
//! - `crate::quantum::resilience`;
//! - `crate::quantum::zqn`;
//! - runtime implementations;
//! - backend providers;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs.
//!
//! Individual declaration files retain the same architectural boundary.
//!
//! ## Integration contract — parser
//!
//! The parser consumes this namespace through the individual declaration
//! modules.
//!
//! The parser is responsible for constructing source-level declaration nodes
//! and inserting them into the canonical AST storage model.
//!
//! The parser must not perform:
//!
//! - name resolution;
//! - type inference;
//! - resource allocation;
//! - hardware selection;
//! - quantum routing;
//! - quantum scheduling;
//! - backend selection.
//!
//! ## Integration contract — structural validation
//!
//! Structural validation consumes declarations after parsing.
//!
//! Validation is responsible for structural invariants such as:
//!
//! - valid node identity;
//! - valid child references;
//! - valid source spans;
//! - required fields;
//! - valid declaration relationships;
//! - duplicate structural identifiers where detectable without semantic
//!   resolution.
//!
//! Declaration-specific validation remains in each declaration module or the
//! dedicated validation subsystem according to the AST validation architecture.
//!
//! ## Integration contract — semantic analysis
//!
//! Semantic analysis consumes declaration nodes and resolves their meaning.
//!
//! It is responsible for deriving information such as:
//!
//! - symbols;
//! - scopes;
//! - resolved types;
//! - generic substitutions;
//! - callable signatures;
//! - trait/interface relationships;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - domain semantics;
//! - visibility semantics.
//!
//! None of those resolved structures belong in this composition module.
//!
//! ## Integration contract — ZUIR
//!
//! Declarations do not lower directly from this module to ZUIR.
//!
//! The intended path is:
//!
//! ```text
//! declaration AST
//!       │
//!       ▼
//! semantic analysis
//!       │
//!       ▼
//! semantic declaration model
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! This preserves the distinction:
//!
//! ```text
//! AST            = source structure
//! Semantic model = resolved meaning
//! ZUIR           = universal computational semantics
//! Domain IR       = domain-specific realization
//! Target IR       = target realization
//! ```
//!
//! ## Quantum integration
//!
//! Quantum declarations are not given a privileged path through this module.
//!
//! A Zamani function, type, interface, module, implementation or other
//! declaration can participate in quantum computation when its source-level
//! semantics require it.
//!
//! The declaration namespace must not assume that a quantum resource is:
//!
//! - superconducting;
//! - trapped-ion;
//! - neutral-atom;
//! - photonic;
//! - bosonic;
//! - annealing-based;
//! - simulator-backed;
//! - any other particular technology.
//!
//! The actual realization is selected downstream from semantic meaning,
//! capabilities and available resources.
//!
//! ## Extension policy
//!
//! New source-level declaration kinds should be introduced as dedicated
//! modules when they have sufficiently distinct semantics.
//!
//! A new declaration module must:
//!
//! 1. own its source-level representation;
//! 2. preserve canonical AST identity;
//! 3. preserve source information;
//! 4. define its structural invariants;
//! 5. define parser integration;
//! 6. define validation integration;
//! 7. define visitor/traversal integration;
//! 8. define serialization behavior where applicable;
//! 9. define semantic-analysis integration;
//! 10. define its ZUIR semantic fate;
//! 11. define scalability requirements;
//! 12. define determinism requirements;
//! 13. define malformed-input behavior;
//! 14. provide tests.
//!
//! Adding a declaration must not introduce hardware-specific state into this
//! namespace.
//!
//! ## Why there is no `Declaration` mega-enum here
//!
//! This module deliberately does not introduce a second closed declaration
//! enumeration merely to aggregate the child modules.
//!
//! The canonical AST architecture already provides node identity and node-kind
//! infrastructure. Duplicating that classification here would create two
//! competing sources of truth and would make future extensibility harder.
//!
//! Declaration classification therefore remains owned by the canonical AST
//! node-kind system and the individual declaration implementations.
//!
//! ## Why there are no wildcard re-exports
//!
//! Wildcard re-exports would make the public API unstable and could silently
//! introduce name collisions as the declaration namespace evolves.
//!
//! Consumers should import the declaration type from its explicit module:
//!
//! ```text
//! frontend::ast::node::declarations::function
//! frontend::ast::node::declarations::r#struct
//! frontend::ast::node::declarations::r#enum
//! ```
//!
//! This keeps ownership and API boundaries explicit.
//!
//! ## Determinism
//!
//! This module:
//!
//! - allocates no node IDs;
//! - maintains no global state;
//! - accesses no clock;
//! - uses no randomness;
//! - accesses no filesystem;
//! - accesses no network;
//! - accesses no hardware;
//! - performs no backend discovery.
//!
//! Deterministic node identity remains the responsibility of the canonical AST
//! construction infrastructure.
//!
//! ## Security
//!
//! This module contains no `unsafe` code.
//!
//! It performs no pointer manipulation and no unchecked memory access.
//!
//! Malformed source and malformed declaration structures are handled by the
//! parser and validation layers rather than by unsafe operations here.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021.
//!
//! No unstable Rust features are required.
//!
//! `unsafe` code is forbidden explicitly for this module.
//!
//! ## Maintenance contract
//!
//! This file should remain a thin composition root.
//!
//! A declaration implementation must be changed in its own file when its
//! representation changes. This file should only need modification when:
//!
//! - a declaration module is added;
//! - a declaration module is removed;
//! - a declaration module is intentionally renamed;
//! - the namespace architecture itself changes.
//!
//! Changes to the internals of an existing declaration must not require
//! changes here.
//!
//! ## Current declaration modules
//!
//! The repository currently provides the declaration implementations listed
//! below. Keeping these declarations explicit also makes missing module files
//! fail at compilation rather than being silently omitted.
//!
//! ---------------------------------------------------------------------------
//! Module declarations
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

/// Source-level class declarations.
pub mod class;

/// Source-level constant declarations.
pub mod constant;

/// Source-level enumeration declarations.
///
/// `enum` is a Rust keyword, therefore the raw identifier is required.
pub mod r#enum;

/// Source-level external/foreign declarations.
///
/// `extern` is a Rust keyword, therefore the raw identifier is required.
pub mod r#extern;

/// Source-level function declarations.
pub mod function;

/// Source-level implementation declarations.
///
/// `impl` is a Rust keyword, therefore the raw identifier is required.
pub mod r#impl;

/// Source-level interface declarations.
pub mod interface;

/// Source-level module declarations.
pub mod module;

/// Source-level structure declarations.
///
/// `struct` is a Rust keyword, therefore the raw identifier is required.
pub mod r#struct;

/// Source-level trait declarations.
///
/// `trait` is a Rust keyword, therefore the raw identifier is required.
pub mod r#trait;

/// Source-level type-alias declarations.
pub mod type_alias;

/// Source-level variable declarations.
pub mod variable;