//! # Zamani Frontend AST — Patterns
//!
//! Canonical module boundary for source-level pattern nodes in the Zamani
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
//! Native Zamani AST
//!     │
//!     ├── patterns::wildcard
//!     ├── patterns::identifier
//!     ├── patterns::literal
//!     ├── patterns::tuple
//!     ├── patterns::struct
//!     ├── patterns::enum
//!     └── patterns::pattern
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ▼
//! Semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! This module is therefore a **source-AST namespace boundary**.
//!
//! It does not perform parsing, semantic analysis, type checking, resource
//! analysis, quantum compilation, routing, scheduling, hardware mapping,
//! calibration, QEC, resilience, backend selection, or ZUIR lowering.
//!
//! ## Core architectural invariant
//!
//! Patterns describe source-language structure.
//!
//! They must not encode the implementation of the value being matched.
//!
//! Consequently, this module must remain independent of:
//!
//! - machine size;
//! - processor architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - QPU architecture;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - gate set;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime execution;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target-specific instructions.
//!
//! This is required for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! A pattern therefore remains the same source-level construct whether the
//! surrounding program is eventually executed on a tiny machine, a large
//! heterogeneous system, a quantum computer, a distributed system, or a future
//! computational architecture.
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! 1. the public Rust module boundary for pattern nodes;
//! 2. stable re-exports of concrete pattern types;
//! 3. documentation of the pattern-module contract;
//! 4. the namespace through which parser, validation, semantic analysis and
//!    tooling access pattern nodes.
//!
//! Concrete pattern data structures remain owned by their individual files.
//!
//! The aggregate pattern representation, if present in `pattern.rs`, remains
//! owned by `pattern.rs`.
//!
//! This file must not duplicate that representation.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! source/span/node infrastructure
//!          │
//!          ▼
//! concrete pattern nodes
//!          │
//!          ▼
//! patterns::pattern
//!          │
//!          ▼
//! patterns::mod
//!          │
//!          ▼
//! parser / validation / semantic analysis
//!          │
//!          ▼
//! semantic model
//!          │
//!          ▼
//! ZUIR
//! ```
//!
//! This module must never introduce a dependency in the opposite direction.
//!
//! In particular:
//!
//! ```text
//! patterns::mod
//!     ✗→ semantic
//!     ✗→ ZUIR
//!     ✗→ quantum::ir
//!     ✗→ hardware
//!     ✗→ optimizer
//!     ✗→ scheduler
//!     ✗→ router
//!     ✗→ QEC
//!     ✗→ runtime
//!     ✗→ backend
//! ```
//!
//! ## Concrete pattern modules
//!
//! The current repository architecture provides concrete pattern nodes in
//! separate files. The module boundary exposes those implementations without
//! duplicating them.
//!
//! Expected source-level pattern modules include:
//!
//! ```text
//! patterns/
//! ├── mod.rs
//! ├── pattern.rs
//! ├── wildcard.rs
//! ├── identifier.rs
//! ├── literal.rs
//! ├── tuple.rs
//! ├── struct.rs
//! └── enum.rs
//! ```
//!
//! The exact concrete set remains governed by the Zamani grammar and the
//! repository's existing AST implementation. This module must not invent
//! additional pattern variants merely for architectural completeness.
//!
//! ## Pattern hierarchy
//!
//! Conceptually:
//!
//! ```text
//! Pattern
//! ├── WildcardPattern
//! ├── IdentifierPattern
//! ├── LiteralPattern
//! ├── TuplePattern
//! ├── StructPattern
//! └── EnumPattern
//! ```
//!
//! The aggregate `Pattern` representation belongs to `pattern.rs`.
//!
//! Concrete nodes remain independently implementable so that adding or
//! modifying one source-level pattern does not require putting all pattern
//! implementation into this module.
//!
//! ## Why this module should remain small
//!
//! `mod.rs` is intentionally a thin integration boundary.
//!
//! It must not become another monolithic AST file.
//!
//! Do not place concrete node definitions here.
//!
//! Do not place large enums here.
//!
//! Do not place parser functions here.
//!
//! Do not place semantic resolution here.
//!
//! Do not place visitors here.
//!
//! Do not place serialization implementations here.
//!
//! Do not place quantum-specific pattern semantics here.
//!
//! Those responsibilities belong to their respective modules.
//!
//! ## Parser integration
//!
//! The parser consumes this namespace to construct concrete pattern nodes.
//!
//! Conceptually:
//!
//! ```text
//! parser
//!   │
//!   ├── `_`
//!   │      └── WildcardPattern
//!   │
//!   ├── identifier
//!   │      └── IdentifierPattern
//!   │
//!   ├── literal
//!   │      └── LiteralPattern
//!   │
//!   ├── tuple pattern
//!   │      └── TuplePattern
//!   │
//!   ├── struct pattern
//!   │      └── StructPattern
//!   │
//!   └── enum/constructor pattern
//!          └── EnumPattern
//! ```
//!
//! Parser-specific token handling remains outside this module.
//!
//! This module therefore does not depend on lexer token types.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes the pattern structures exposed here.
//!
//! It is responsible for determining:
//!
//! - whether a pattern is valid in its surrounding context;
//! - which names are introduced;
//! - which symbols are referenced;
//! - the type being matched;
//! - ownership semantics;
//! - borrowing semantics where applicable;
//! - resource semantics;
//! - capability constraints;
//! - domain semantics;
//! - quantum-resource semantics where applicable;
//! - exhaustiveness;
//! - reachability;
//! - binding compatibility.
//!
//! None of these meanings belong in this module.
//!
//! ## Quantum integration
//!
//! Patterns must remain quantum-neutral at the AST module boundary.
//!
//! For example:
//!
//! ```text
//! match resource {
//!     q => ...
//! }
//! ```
//!
//! The pattern `q` is an ordinary identifier pattern.
//!
//! Semantic analysis may subsequently determine that `q` represents:
//!
//! - a classical value;
//! - a logical quantum resource;
//! - a physical resource;
//! - a collection of quantum resources;
//! - a distributed resource;
//! - another future computational resource.
//!
//! No quantum-specific information is added to this module merely because
//! semantic analysis may interpret a pattern as binding a quantum resource.
//!
//! ## POCO-REAF and scalability
//!
//! This module contains no language-level resource-size limit.
//!
//! It must not introduce constants such as:
//!
//! ```text
//! MAX_PATTERNS
//! MAX_PATTERN_DEPTH
//! MAX_BINDINGS
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! ```
//!
//! The number of patterns, bindings, resources, match arms and nested source
//! constructs is constrained only by the available compiler resources and
//! explicitly configured compiler safety policies.
//!
//! Compiler resource limits must not become language semantics.
//!
//! ## Determinism
//!
//! Module exposure itself is deterministic.
//!
//! No global mutable registry is introduced here.
//!
//! No runtime-generated identifiers are introduced here.
//!
//! No timestamps, random state, process identifiers, memory addresses or
//! backend state are introduced here.
//!
//! Concrete AST determinism remains the responsibility of the concrete node
//! implementations and canonical AST infrastructure.
//!
//! ## Serialization
//!
//! This module deliberately does not define a second serialization format.
//!
//! Serialization is owned by the canonical AST serialization subsystem.
//!
//! Concrete pattern nodes may derive or implement the serialization traits
//! required by that subsystem.
//!
//! The global AST schema version must remain distinct from individual pattern
//! schema versions where the repository's serialization architecture requires
//! that distinction.
//!
//! ## Traversal and visitors
//!
//! Traversal and visitor implementations belong to the AST traversal/visitor
//! subsystem.
//!
//! Concrete pattern nodes must expose whatever common `AstNode`/`Node` contract
//! that subsystem requires.
//!
//! This module must not create a second visitor hierarchy.
//!
//! A pattern traversal should conceptually be:
//!
//! ```text
//! Pattern
//!     │
//!     ├── concrete leaf pattern
//!     │
//!     └── nested pattern(s)
//! ```
//!
//! Composite pattern nodes own or reference their nested source-AST nodes
//! according to the canonical AST graph ownership model.
//!
//! Leaf patterns such as wildcard and plain identifier patterns have no child
//! pattern nodes.
//!
//! ## Structural validation
//!
//! Local validation belongs to the concrete node implementation.
//!
//! Aggregate validation belongs to the canonical AST validation subsystem.
//!
//! This module must not duplicate validation rules.
//!
//! The validation boundary is:
//!
//! ```text
//! concrete node
//!     └── local invariants
//!
//! pattern aggregate
//!     └── child/variant consistency
//!
//! AST validation
//!     └── graph-wide structural invariants
//!
//! semantic analysis
//!     └── language semantics
//! ```
//!
//! ## Extensibility
//!
//! New source-level pattern constructs must be added by:
//!
//! 1. establishing the grammar production;
//! 2. defining the concrete AST node in its own file;
//! 3. integrating it with the canonical `Node`/`AstNode` infrastructure;
//! 4. defining local invariants;
//! 5. integrating traversal;
//! 6. integrating validation;
//! 7. integrating serialization;
//! 8. integrating semantic lowering;
//! 9. adding parser coverage;
//! 10. adding tests;
//! 11. exposing the node through this module boundary where appropriate.
//!
//! A new backend must never require modification of this module.
//!
//! A new quantum technology must never require modification of this module.
//!
//! A new machine architecture must never require modification of this module.
//!
//! A new machine size must never require modification of this module.
//!
//! ## External quantum languages
//!
//! External languages such as OpenQASM must not be represented by these native
//! pattern types merely because they have similar syntax.
//!
//! Their frontend-specific ASTs belong under their format/frontend boundaries.
//!
//! The integration path is:
//!
//! ```text
//! external language
//!     │
//!     ▼
//! external-format AST
//!     │
//!     ▼
//! format validation
//!     │
//!     ▼
//! Zamani semantic/source representation
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! This prevents an external language from becoming the architectural shape of
//! the native Zamani AST.
//!
//! ## QIR / LLVM / MLIR boundary
//!
//! No QIR, LLVM or MLIR type belongs in this module.
//!
//! Those systems are downstream interoperability/implementation layers.
//!
//! The native frontend boundary remains:
//!
//! ```text
//! Zamani source
//!     ▼
//! Native AST
//!     ▼
//! Semantic Model
//!     ▼
//! ZUIR
//!     ▼
//! Domain/target lowering
//! ```
//!
//! ## Public API policy
//!
//! Public exports in this module should be limited to types that are already
//! part of the canonical pattern API.
//!
//! Re-exporting the concrete nodes provides a stable and discoverable API:
//!
//! ```rust
//! use crate::frontend::ast::node::patterns::IdentifierPattern;
//! use crate::frontend::ast::node::patterns::WildcardPattern;
//! ```
//!
//! The module must not re-export private implementation details merely to make
//! unrelated modules reach into concrete internals.
//!
//! ## Compatibility policy
//!
//! This file should remain stable when implementation details of an individual
//! concrete pattern change, provided its public contract remains compatible.
//!
//! Conversely, adding a new concrete pattern should require only the intended
//! module declaration and public export here plus the corresponding integration
//! work.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.
//!
//! The complete pattern subsystem must remain compatible with the project
//! requirement of safe Rust.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly-only features;
//! - no `unsafe`.
//!
//! ## Integration contract
//!
//! ```text
//! src/frontend/ast/node/patterns/mod.rs
//!     │
//!     ├── pattern.rs
//!     ├── wildcard.rs
//!     ├── identifier.rs
//!     ├── literal.rs
//!     ├── tuple.rs
//!     ├── struct.rs
//!     └── enum.rs
//!
//! src/frontend/ast/node/mod.rs
//!     └── pub mod patterns;
//!
//! parser
//!     └── consumes concrete pattern API
//!
//! AST validation
//!     └── validates concrete/aggregate pattern invariants
//!
//! visitors/traversal
//!     └── traverses pattern nodes
//!
//! semantic analysis
//!     └── resolves binding/type/resource/domain semantics
//!
//! ZUIR lowering
//!     └── consumes semantic representation
//! ```
//!
//! ## Important implementation rule
//!
//! Do not add fallback declarations for files that do not exist.
//!
//! Rust module declarations are compile-time contracts. If a concrete pattern
//! file is not present, declaring it here would make the crate fail to compile.
//!
//! Therefore the declarations below correspond only to the concrete pattern
//! modules established by the current AST architecture.
//!
//! If the repository later introduces another pattern node, its module should
//! be added here in the same commit as the new implementation and its parser,
//! validation, traversal, serialization and semantic integration.
//!
//! ## No duplicate ownership
//!
//! This module does not own:
//!
//! - `Pattern` implementation details;
//! - concrete pattern fields;
//! - parser functions;
//! - semantic symbols;
//! - types;
//! - resource identities;
//! - quantum identities;
//! - hardware identities;
//! - ZUIR nodes.
//!
//! Each of those remains authoritative in its designated subsystem.
//!
//! ## Production-readiness invariant
//!
//! Once this module is integrated successfully, changing an unrelated concrete
//! pattern implementation must not require adding semantic, quantum, hardware,
//! routing, scheduling or backend logic to this file.
//!
//! Likewise, adding a new backend, hardware topology, quantum technology,
//! machine size, or execution target must not require reopening this module.

/// Aggregate/source-level pattern representation.
///
/// This module owns the canonical pattern aggregate and any common pattern
/// operations defined by the repository.
pub mod pattern;

/// Source-level wildcard pattern (`_`).
pub mod wildcard;

/// Source-level identifier/binding pattern.
pub mod identifier;

/// Source-level literal pattern.
pub mod literal;

/// Source-level tuple pattern.
pub mod tuple;

/// Source-level struct pattern.
pub mod r#struct;

/// Source-level enum/constructor pattern.
pub mod r#enum;

// -----------------------------------------------------------------------------
// Stable public re-exports
// -----------------------------------------------------------------------------
//
// Re-export only concrete public pattern types that are actually provided by
// their respective modules.
//
// Keeping these exports here gives parser, tooling and semantic consumers one
// stable pattern namespace without making this module the owner of their
// implementations.

pub use identifier::IdentifierPattern;
pub use literal::LiteralPattern;
pub use pattern::Pattern;
pub use r#enum::EnumPattern;
pub use r#struct::StructPattern;
pub use tuple::TuplePattern;
pub use wildcard::WildcardPattern;

// -----------------------------------------------------------------------------
// Compile-time integration tests
// -----------------------------------------------------------------------------
//
// These tests intentionally test only module/API integration. Concrete node
// invariants remain tested by their owning files.

#[cfg(test)]
mod tests {
    use super::*;

    /// The pattern module must expose the canonical aggregate type.
    #[test]
    fn exposes_pattern_type() {
        fn assert_pattern_type<T>() {}

        assert_pattern_type::<Pattern>();
    }

    /// The pattern module must expose the concrete pattern types through its
    /// stable public namespace.
    #[test]
    fn exposes_concrete_pattern_types() {
        fn assert_type<T>() {}

        assert_type::<IdentifierPattern>();
        assert_type::<LiteralPattern>();
        assert_type::<TuplePattern>();
        assert_type::<StructPattern>();
        assert_type::<EnumPattern>();
        assert_type::<WildcardPattern>();
    }
}