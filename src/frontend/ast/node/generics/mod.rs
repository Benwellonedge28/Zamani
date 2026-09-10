//! Zamani Frontend AST — Generics
//!
//! Canonical module boundary for source-level generic constructs.
//!
//! # Architectural position
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
//! frontend::ast
//!     │
//!     └── node::generics
//!             │
//!             ├── parameter
//!             ├── argument
//!             ├── bound
//!             └── where_clause
//!             │
//!             ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── generic scope
//!     ├── name resolution
//!     ├── constraint resolution
//!     ├── substitution
//!     └── satisfiability
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Purpose
//!
//! This file is the canonical module boundary for Zamani's source-level
//! generic AST subsystem.
//!
//! It owns:
//!
//! - module organization;
//! - public generic-AST exports;
//! - canonical generic namespace stability;
//! - documentation of dependency direction;
//! - generic-subsystem integration contracts;
//! - module-level compile-time safety guarantees;
//! - lightweight namespace/API tests.
//!
//! It deliberately does NOT own a generic AST data structure.
//!
//! The actual representations remain in their dedicated files:
//!
//! ```text
//! parameter.rs       → TypeParameter
//! argument.rs        → GenericArgument
//! bound.rs           → Bound / Bounds aliases
//! where_clause.rs    → WhereClause and its canonical predicates
//! ```
//!
//! # Critical architectural rule
//!
//! This module must remain an integration boundary rather than becoming a
//! second implementation of generics.
//!
//! Do NOT add generic AST structs/enums directly here merely for convenience.
//!
//! In particular, this file must never introduce competing representations
//! such as:
//!
//! ```text
//! GenericParameter
//! GenericBound
//! GenericConstraint
//! GenericArgumentValue
//! QuantumGeneric
//! HardwareGeneric
//! ```
//!
//! when the canonical child modules already provide the representation.
//!
//! Duplicate representations would cause:
//!
//! - competing ownership;
//! - incompatible serialization;
//! - visitor divergence;
//! - validation divergence;
//! - parser ambiguity;
//! - semantic-lowering ambiguity;
//! - unnecessary allocations;
//! - API instability;
//! - migration problems.
//!
//! # Source-level boundary
//!
//! Everything exported by this module represents source-level programmer
//! intent.
//!
//! Generic constructs do NOT resolve:
//!
//! - symbols;
//! - types;
//! - implementations;
//! - capabilities;
//! - resources;
//! - hardware;
//! - quantum topology;
//! - physical qubits;
//! - QPU selection;
//! - scheduling;
//! - routing;
//! - calibration;
//! - error correction;
//! - resilience;
//! - backend selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target instructions.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! # POCO-REAF
//!
//! Generic constructs are fundamental to:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! A generic program describes abstractions and constraints without embedding
//! the size or implementation of the eventual execution system.
//!
//! Consequently this module contains no assumptions about:
//!
//! - CPU width;
//! - GPU width;
//! - FPGA size;
//! - QPU size;
//! - qubit count;
//! - register count;
//! - memory capacity;
//! - machine count;
//! - topology;
//! - instruction-set size;
//! - vendor;
//! - backend;
//! - quantum technology.
//!
//! A generic parameter can therefore describe an abstraction that is later
//! instantiated for a tiny resource set or a substantially larger resource
//! set without changing this AST module.
//!
//! "Infinity" is interpreted architecturally: the AST imposes no artificial
//! finite machine/domain limit. Actual compilation is necessarily bounded by
//! available compiler, runtime, and target resources and by explicitly
//! configured safety/resource policies.
//!
//! # Domain neutrality
//!
//! The generic subsystem is intentionally domain-neutral.
//!
//! A bound may eventually describe an abstraction associated with:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerators;
//! - AI/ML;
//! - HDL;
//! - memory;
//! - effects;
//! - capabilities;
//! - resources;
//! - future computational domains.
//!
//! This module does not interpret any of those domains.
//!
//! For example:
//!
//! ```text
//! Q extends QuantumResource
//! ```
//!
//! is represented through the canonical source-level type-expression system.
//!
//! Whether `QuantumResource` is valid, satisfiable, available on a target, or
//! realizable using physical qubits is determined downstream.
//!
//! # Canonical representations
//!
//! The generic subsystem intentionally reuses the canonical type-expression
//! representation.
//!
//! ```text
//! TypeExpr
//!     │
//!     ├── generic argument
//!     ├── generic bound
//!     ├── generic parameter reference
//!     └── where-clause constraint
//! ```
//!
//! This means that generic syntax automatically benefits from the extensible
//! source-level type system without requiring this module to maintain another
//! parallel hierarchy.
//!
//! The repository's canonical type layer already defines `TypeExpr` and
//! `TypeBounds`; generic bounds should remain expressed through those types.
//! 
//!
//! # Generic parameter
//!
//! `parameter.rs` owns the canonical `TypeParameter` representation.
//!
//! Its intended source representation is conceptually:
//!
//! ```text
//! TypeParameter
//!     ├── TypeParameterName
//!     └── ordered Vec<TypeExpr>
//! ```
//!
//! Bound order is preserved because it is relevant to:
//!
//! - deterministic AST behavior;
//! - diagnostics;
//! - source reconstruction;
//! - serialization;
//! - tooling;
//! - reproducible compilation.
//!
//! The parameter module deliberately does not resolve constraints.
//!
//! # Generic argument
//!
//! `argument.rs` owns the `GenericArgument` source-level façade.
//!
//! The current canonical type representation remains authoritative:
//!
//! ```text
//! TypeExpr::Generic {
//!     base,
//!     arguments,
//! }
//! ```
//!
//! The generic argument module therefore does not introduce a second closed
//! enum of type/value/resource/backend arguments before the language grammar
//! actually requires such a distinction.
//!
//! This preserves compatibility with the existing canonical `TypeExpr`
//! representation.
//!
//! # Generic bounds
//!
//! `bound.rs` owns the generic namespace for canonical bounds.
//!
//! A single bound remains a `TypeExpr`.
//!
//! A collection of bounds remains the canonical `TypeBounds` collection.
//!
//! There is deliberately no closed enum such as:
//!
//! ```text
//! TypeBound::Trait
//! TypeBound::Quantum
//! TypeBound::Hardware
//! ```
//!
//! Such an enum would require the native AST to change whenever a new domain
//! is introduced.
//!
//! # Where clauses
//!
//! `where_clause.rs` owns source-level generic `where` constraints.
//!
//! A where clause remains source syntax and does not perform:
//!
//! - name resolution;
//! - trait resolution;
//! - type inference;
//! - substitution;
//! - satisfiability checking;
//! - implementation selection;
//! - target selection.
//!
//! Those operations belong to semantic analysis.
//!
//! # Parser integration contract
//!
//! The parser may import this module's canonical types:
//!
//! ```text
//! frontend::ast::node::generics::TypeParameter
//! frontend::ast::node::generics::GenericArgument
//! frontend::ast::node::generics::Bound
//! frontend::ast::node::generics::Bounds
//! frontend::ast::node::generics::WhereClause
//! ```
//!
//! The parser owns syntax recognition and error recovery.
//!
//! This module must never import parser state, lexer tokens, parser contexts,
//! or parser implementation details.
//!
//! The intended direction is:
//!
//! ```text
//! lexer → parser → generics AST
//! ```
//!
//! never:
//!
//! ```text
//! generics AST → parser
//! ```
//!
//! # Structural validation contract
//!
//! Structural validation remains delegated to the canonical child AST types.
//!
//! This module must not duplicate recursive validation.
//!
//! In particular, this module must not decide whether a bound is semantically
//! legal. It only provides access to the canonical representations that can be
//! structurally validated by the AST validation layer.
//!
//! # Semantic integration contract
//!
//! Semantic analysis consumes the generic AST through this module's stable
//! exports.
//!
//! The semantic layer is responsible for:
//!
//! - introducing generic scopes;
//! - detecting duplicate generic names;
//! - resolving names;
//! - resolving bounds;
//! - resolving where-clause subjects;
//! - checking constraint compatibility;
//! - checking recursive constraints;
//! - performing substitution;
//! - checking instantiations;
//! - determining satisfiability;
//! - constructing the semantic generic model.
//!
//! None of these operations belong here.
//!
//! # ZUIR integration contract
//!
//! There is intentionally no direct ZUIR dependency.
//!
//! The boundary is:
//!
//! ```text
//! generic AST
//!     │
//!     ▼
//! semantic generic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! This is essential because the frontend AST must not become coupled to one
//! intermediate representation.
//!
//! # Quantum integration contract
//!
//! Quantum programs may use generic abstractions such as:
//!
//! ```text
//! Q extends QuantumResource
//! ```
//!
//! or conceptually:
//!
//! ```text
//! Register<N>
//! State<S>
//! Quantum<Q>
//! ```
//!
//! The generic subsystem does not know what these abstractions mean.
//!
//! In particular, this module does not contain:
//!
//! - qubit IDs;
//! - physical qubit mappings;
//! - coupling graphs;
//! - gate sets;
//! - QEC codes;
//! - syndrome data;
//! - decoders;
//! - schedulers;
//! - routing;
//! - calibration;
//! - pulse data;
//! - noise models;
//! - QIR types;
//! - vendor APIs.
//!
//! Generic source abstractions therefore remain valid across different
//! quantum technologies and hardware sizes.
//!
//! # Resource scalability
//!
//! Generic arity, bound count, nesting depth, and type-expression complexity
//! must not be constrained by constants in this module.
//!
//! There must be no:
//!
//! ```text
//! MAX_GENERIC_PARAMETERS
//! MAX_GENERIC_ARGUMENTS
//! MAX_BOUNDS
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! Safety limits, when required to protect the compiler from pathological
//! input, must be supplied by explicit compilation policies.
//!
//! The AST representation itself must remain unconstrained by artificial
//! machine-size assumptions.
//!
//! # Determinism
//!
//! Generic constructs preserve source ordering.
//!
//! Ordered collections remain ordered throughout the generic AST boundary.
//!
//! This is required for:
//!
//! - deterministic diagnostics;
//! - deterministic serialization;
//! - reproducible builds;
//! - stable tooling;
//! - source reconstruction;
//! - predictable semantic processing.
//!
//! Semantic analysis may later canonicalize constraints for its own purposes,
//! but that canonicalization must not destroy the source AST's representation.
//!
//! # Serialization contract
//!
//! This module introduces no independent serialization format.
//!
//! Serialization remains owned by the actual AST types and the surrounding AST
//! schema/versioning layer.
//!
//! Re-exporting canonical types ensures that adding the generic namespace does
//! not create a second wire representation.
//!
//! The following must remain independent concepts:
//!
//! ```text
//! language version
//! AST schema version
//! generic-module API version
//! compiler version
//! semantic-model version
//! ZUIR version
//! ```
//!
//! They must not be conflated.
//!
//! # Visitor/traversal contract
//!
//! This module does not create a second visitor hierarchy.
//!
//! The canonical AST visitor/traversal system remains authoritative.
//!
//! A traversal encountering generic constructs should reach:
//!
//! ```text
//! TypeParameter
//!     └── ordered bounds
//!
//! GenericArgument
//!     └── underlying TypeExpr
//!
//! WhereClause
//!     └── ordered constraints
//! ```
//!
//! exactly according to source structure.
//!
//! Generic traversal must not become a separate traversal universe.
//!
//! # Dependency contract
//!
//! This module may depend on:
//!
//! - the four canonical generic child modules;
//! - Rust's standard module/re-export machinery.
//!
//! It must not depend on:
//!
//! - lexer implementation;
//! - parser implementation;
//! - semantic analyzer;
//! - compiler driver;
//! - ZUIR implementation;
//! - quantum IR;
//! - hardware IR;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - ZQN;
//! - runtime;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs;
//! - filesystem APIs;
//! - network APIs;
//! - mutable global state.
//!
//! This makes the module independently stable and prevents dependency cycles.
//!
//! # Rust compatibility
//!
//! This file is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Public API stability
//!
//! The generic namespace intentionally provides a concise public API:
//!
//! ```text
//! generics::TypeParameter
//! generics::GenericArgument
//! generics::Bound
//! generics::Bounds
//! generics::WhereClause
//! ```
//!
//! The underlying modules remain available for code that requires more
//! explicit module qualification.
//!
//! This allows downstream compiler phases to depend on stable generic names
//! without knowing the physical organization of implementation files.
//!
//! # Migration contract
//!
//! The repository currently contains an older `types::type_parameter`
//! implementation alongside the generic subsystem.
//!
//! The generic namespace must not create another independent implementation.
//!
//! The intended migration is:
//!
//! ```text
//! legacy type_parameter
//!         │
//!         ▼
//! canonical generics::parameter::TypeParameter
//!         │
//!         ▼
//! stable generics namespace
//! ```
//!
//! Compatibility re-exports may be introduced in the legacy module during a
//! coordinated migration.
//!
//! This `mod.rs` must not itself import the legacy AST to accomplish that
//! migration.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! - all generic child modules are declared exactly once;
//! - canonical generic types are re-exported;
//! - no duplicate AST representation is introduced;
//! - no semantic logic is introduced;
//! - no parser logic is introduced;
//! - no ZUIR dependency is introduced;
//! - no quantum/hardware dependency is introduced;
//! - no fixed generic/resource limit is introduced;
//! - public names are stable;
//! - dependency direction is one-way;
//! - Rust 1.97/1.97.1 compatibility is preserved;
//! - `unsafe` is forbidden;
//! - module-level tests validate namespace wiring;
//! - downstream consumers can depend on this module without knowing the
//!   internal file organization.
//!
//! # =============================================================================
//! # Implementation
//! # =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Generic parameter declarations.
pub mod parameter;

/// Generic arguments at generic-application sites.
pub mod argument;

/// Canonical generic-bound vocabulary and aliases.
pub mod bound;

/// Generic `where` clauses and their source-level constraints.
pub mod where_clause;

// =============================================================================
// Stable public namespace
// =============================================================================

/// Canonical source-level generic parameter.
pub use parameter::TypeParameter;

/// Canonical source-level generic argument.
pub use argument::GenericArgument;

/// Canonical representation of one generic bound.
///
/// This is an alias/re-export of the canonical type-expression representation,
/// not a second AST type.
pub use bound::Bound;

/// Canonical ordered collection of generic bounds.
///
/// This is the existing canonical `TypeBounds` representation.
pub use bound::Bounds;

/// Canonical source-level generic `where` clause.
pub use where_clause::WhereClause;

// =============================================================================
// Explicit module aliases for API discoverability
// =============================================================================

/// Stable module-level API version.
///
/// This version identifies the organization/API surface of this generic
/// namespace. It is deliberately independent of the complete AST schema,
/// language, compiler, semantic model, and ZUIR versions.
pub const GENERICS_MODULE_API_VERSION: u16 = 1;

/// Returns the generic namespace API version.
///
/// A function is provided in addition to the constant so tooling can obtain
/// the version through a stable API without depending on constant naming.
#[must_use]
pub const fn api_version() -> u16 {
    GENERICS_MODULE_API_VERSION
}

// =============================================================================
// Architectural assertions
// =============================================================================

/// Marker documenting the intended dependency direction.
///
/// This is a zero-sized, zero-state type rather than a runtime registry.
/// Generic AST modules must remain source-level and domain-neutral.
///
/// The type exists only as an explicit API/documentation anchor for tools and
/// downstream documentation that need to identify the generic AST boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GenericAstBoundary;

impl GenericAstBoundary {
    /// Returns the canonical boundary name.
    #[must_use]
    pub const fn name() -> &'static str {
        "zamani.frontend.ast.generics"
    }

    /// Returns whether this boundary is source-level.
    #[must_use]
    pub const fn is_source_level() -> bool {
        true
    }

    /// Returns whether this boundary is target-independent.
    #[must_use]
    pub const fn is_target_independent() -> bool {
        true
    }

    /// Returns whether this boundary has a fixed machine-size assumption.
    #[must_use]
    pub const fn has_fixed_machine_size() -> bool {
        false
    }

    /// Returns whether this boundary has a fixed quantum-resource assumption.
    #[must_use]
    pub const fn has_fixed_quantum_resource_size() -> bool {
        false
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_exports_canonical_generic_parameter() {
        let parameter = TypeParameter::new("T");

        assert_eq!(parameter.name_str(), "T");
        assert!(parameter.is_unbounded());
        assert_eq!(parameter.bound_count(), 0);
    }

    #[test]
    fn namespace_exports_generic_argument() {
        use crate::frontend::ast::node::types::type_expr::TypeExpr;

        let expression = TypeExpr::Identifier(
            crate::frontend::ast::node::types::type_expr::TypePath::single("T"),
        );

        let argument = GenericArgument::new(expression.clone());

        assert_eq!(argument.as_type_expr(), &expression);
    }

    #[test]
    fn namespace_exports_bound_alias_without_creating_a_second_type() {
        use crate::frontend::ast::node::types::type_expr::TypeExpr;

        let bound: Bound = TypeExpr::Identifier(
            crate::frontend::ast::node::types::type_expr::TypePath::single(
                "QuantumResource",
            ),
        );

        assert_eq!(bound.to_source_string(), "QuantumResource");
    }

    #[test]
    fn namespace_exports_bounds_alias() {
        let bounds = Bounds::new();

        assert!(bounds.is_empty());
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(api_version(), GENERICS_MODULE_API_VERSION);
        assert_eq!(GENERICS_MODULE_API_VERSION, 1);
    }

    #[test]
    fn boundary_is_source_level_and_target_independent() {
        assert_eq!(
            GenericAstBoundary::name(),
            "zamani.frontend.ast.generics"
        );
        assert!(GenericAstBoundary::is_source_level());
        assert!(GenericAstBoundary::is_target_independent());
        assert!(!GenericAstBoundary::has_fixed_machine_size());
        assert!(!GenericAstBoundary::has_fixed_quantum_resource_size());
    }

    #[test]
    fn generic_parameter_has_no_machine_size_limit_in_this_boundary() {
        let parameter = TypeParameter::new("Q");

        assert_eq!(parameter.name_str(), "Q");
        assert_eq!(parameter.bound_count(), 0);
    }
}