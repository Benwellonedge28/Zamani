//! Zamani Frontend AST — Generic Bounds
//!
//! Canonical generic-layer vocabulary for source-level type bounds.
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
//!     ├── generics::parameter::TypeParameter
//!     │       │
//!     │       └── ordered bounds
//!     │
//!     └── types::type_expr::TypeExpr
//!             │
//!             └── canonical representation of one bound
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── constraint resolution
//!     ├── capability interpretation
//!     ├── satisfiability
//!     └── generic substitution
//!     │
//!     ▼
//! semantic type / constraint model
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
//! This module provides the generic subsystem's canonical vocabulary for one
//! source-level bound and the ordered collection of bounds attached to a
//! generic parameter.
//!
//! The critical architectural rule is:
//!
//! ```text
//! Bound = canonical TypeExpr
//! ```
//!
//! There is deliberately no second `TypeBound` struct or enum here.
//!
//! The existing canonical `TypeExpr` representation is the source-level type
//! system. A generic bound is a use of a type expression in a constraint
//! position; it is not a different kind of source-level type expression.
//!
//! This prevents duplicate representations such as:
//!
//! ```text
//! TypeExpr
//! GenericBound
//! TypeBound
//! TraitBound
//! QuantumBound
//! CapabilityBound
//! ```
//!
//! from independently evolving and eventually becoming inconsistent.
//!
//! # Canonical representation
//!
//! A single bound is:
//!
//! ```text
//! TypeExpr
//! ```
//!
//! An ordered collection of bounds is:
//!
//! ```text
//! TypeBounds
//! ```
//!
//! where `TypeBounds` remains owned by the canonical type-expression layer.
//!
//! This module only exposes those canonical representations through the
//! generic namespace.
//!
//! # Why this file exists
//!
//! The generic subsystem needs a stable semantic vocabulary that can be used
//! by generic parameters, generic declarations, visitors, semantic adapters,
//! and future generic tooling without forcing those consumers to know the
//! physical location of the underlying type-expression implementation.
//!
//! The module is therefore intentionally small.
//!
//! It is a namespace/integration boundary, not a new AST representation.
//!
//! # Domain neutrality
//!
//! A bound may constrain any source-level type that the Zamani type system
//! permits.
//!
//! This can eventually include abstractions associated with:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerators;
//! - AI/ML;
//! - HDL;
//! - memory;
//! - resources;
//! - effects;
//! - capabilities;
//! - future computational domains.
//!
//! This file does not know which domain a bound belongs to.
//!
//! In particular, it must never contain a closed representation such as:
//!
//! ```text
//! enum TypeBound {
//!     Trait,
//!     Quantum,
//!     Hardware,
//!     Cpu,
//!     Gpu,
//!     ...
//! }
//! ```
//!
//! Such a representation would require the native AST to change whenever a
//! new computational domain is introduced.
//!
//! # Quantum compatibility
//!
//! Quantum programs can use generic constraints without making the AST
//! quantum-specific.
//!
//! Conceptually:
//!
//! ```text
//! T extends QuantumResource
//! ```
//!
//! is represented as a `TypeExpr` referring to `QuantumResource`.
//!
//! Whether `QuantumResource` is:
//!
//! - a valid type;
//! - a valid generic bound;
//! - a capability;
//! - a resource abstraction;
//! - a semantic constraint;
//! - satisfiable for a target;
//!
//! is determined by later compiler phases.
//!
//! This file does not know about:
//!
//! - qubits;
//! - QPUs;
//! - physical qubits;
//! - coupling graphs;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - noise;
//! - ZQN;
//! - quantum vendors;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! # POCO-REAF
//!
//! A bound describes a source-level constraint rather than a machine
//! realization.
//!
//! Therefore it does not encode:
//!
//! - machine width;
//! - CPU count;
//! - GPU count;
//! - FPGA count;
//! - memory capacity;
//! - qubit count;
//! - topology;
//! - instruction set;
//! - vendor;
//! - backend;
//! - physical resource identifiers.
//!
//! A generic algorithm can consequently remain independent of the size and
//! technology of the eventual execution system.
//!
//! The downstream compiler is responsible for determining whether the target
//! has sufficient capabilities and resources.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_BOUNDS
//! MAX_GENERIC_BOUNDS
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this module.
//!
//! A bound itself is one `TypeExpr` and may therefore contain the same
//! recursively structured type information supported by the canonical type
//! system.
//!
//! A collection of bounds is represented by the canonical `TypeBounds`
//! collection, whose capacity grows according to available compiler resources.
//!
//! Compiler safety limits must remain explicit policies rather than hidden
//! constants in the AST.
//!
//! "Infinity" in the POCO-REAF requirement therefore means that the AST does
//! not impose an artificial finite machine/domain limit; actual compilation
//! remains bounded only by available computational resources and explicitly
//! selected compiler safety policies.
//!
//! # Determinism
//!
//! The underlying `TypeExpr` representation is structural.
//!
//! For collections, `TypeBounds` preserves source order.
//!
//! Consequently:
//!
//! ```text
//! A + B
//! ```
//!
//! remains structurally distinct from:
//!
//! ```text
//! B + A
//! ```
//!
//! even if semantic analysis later determines that the corresponding
//! constraints are mathematically commutative.
//!
//! Source order is important for:
//!
//! - deterministic AST serialization;
//! - diagnostics;
//! - source reconstruction;
//! - reproducible compilation;
//! - stable tooling behavior.
//!
//! # Ownership
//!
//! This module does not own a second copy of bound data.
//!
//! Ownership remains with:
//!
//! ```text
//! TypeExpr
//! TypeBounds
//! ```
//!
//! Re-exporting the canonical representation avoids unnecessary allocations,
//! cloning, conversion layers, and semantic drift.
//!
//! # Dependency contract
//!
//! This module may depend only on the canonical frontend AST type system.
//!
//! Allowed:
//!
//! - Rust standard library;
//! - canonical `types::type_expr`;
//! - canonical `types::bounds`.
//!
//! Forbidden:
//!
//! - lexer implementation;
//! - parser implementation;
//! - semantic analyzer;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - hardware IR;
//! - scheduler;
//! - router;
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
//! - global mutable state.
//!
//! # Integration with `generics::parameter`
//!
//! `generics::parameter::TypeParameter` currently stores its bounds directly
//! as:
//!
//! ```text
//! Vec<TypeExpr>
//! ```
//!
//! That is already the correct canonical semantic representation.
//!
//! This module must therefore NOT force a second `GenericBound` wrapper into
//! `TypeParameter` merely to give the bound a different Rust name.
//!
//! Existing representation:
//!
//! ```text
//! TypeParameter
//!     ├── TypeParameterName
//!     └── Vec<TypeExpr>
//! ```
//!
//! remains valid.
//!
//! The generic namespace may use:
//!
//! ```text
//! generics::bound::Bound
//! ```
//!
//! as a vocabulary alias when a single bound is required by an API.
//!
//! # Integration with `types::bounds`
//!
//! The repository already contains the canonical `TypeBounds` collection.
//!
//! That collection deliberately uses:
//!
//! ```text
//! Vec<TypeExpr>
//! ```
//!
//! and exists specifically to avoid a closed `TypeBound` hierarchy.
//!
//! Therefore this module re-exports that collection instead of defining
//! another one.
//!
//! # Integration with `TypeExpr`
//!
//! `TypeExpr` remains authoritative for:
//!
//! - type-expression structure;
//! - generic parameter references;
//! - nested generic applications;
//! - arrays;
//! - slices;
//! - functions;
//! - references;
//! - pointers;
//! - optionals;
//! - results;
//! - unions;
//! - intersections;
//! - future canonical type-system constructs.
//!
//! A bound automatically gains the expressive capabilities of the canonical
//! type system without requiring this module to be changed.
//!
//! # Parser integration
//!
//! The parser owns syntax.
//!
//! It must parse the source representation and construct the canonical
//! `TypeExpr`.
//!
//! Conceptually:
//!
//! ```text
//! parser
//!   │
//!   ├── parse "extends"
//!   │
//!   └── parse type expression
//!             │
//!             ▼
//!          TypeExpr
//! ```
//!
//! This module must not import parser token types or parser state.
//!
//! Parser recovery remains outside the AST type definitions.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes `Bound`/`TypeExpr` values and determines:
//!
//! - name resolution;
//! - whether the expression names a valid type;
//! - whether it is legal in a bound position;
//! - whether it represents a trait-like constraint;
//! - whether it represents a capability constraint;
//! - whether multiple bounds are compatible;
//! - whether recursive constraints are valid;
//! - whether the generic parameter is satisfiable;
//! - whether an instantiation satisfies every bound;
//! - how substitutions affect the bound.
//!
//! None of those decisions belong in this file.
//!
//! # ZUIR integration
//!
//! There is deliberately no direct ZUIR dependency.
//!
//! The boundary is:
//!
//! ```text
//! Bound / TypeExpr
//!       │
//!       ▼
//! semantic constraint
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! This keeps the frontend AST independent of the downstream universal IR.
//!
//! # Visitor integration
//!
//! A visitor that encounters a generic parameter should traverse its bounds
//! through the existing `TypeExpr` traversal mechanism.
//!
//! This module does not implement a second visitor hierarchy.
//!
//! This avoids a particularly dangerous architecture where:
//!
//! ```text
//! TypeExpr visitor
//! GenericBound visitor
//! TypeBound visitor
//! ```
//!
//! evolve independently.
//!
//! # Serialization integration
//!
//! No custom serialization format is introduced here.
//!
//! `Bound` is the canonical `TypeExpr`, so it inherits the existing
//! serialization contract of `TypeExpr`.
//!
//! `TypeBounds` likewise inherits the canonical collection representation.
//!
//! This guarantees that adding a generic namespace does not silently create a
//! second wire format.
//!
//! AST schema/version management remains owned by the surrounding AST
//! serialization layer.
//!
//! # Validation integration
//!
//! Structural validation remains the responsibility of the canonical
//! `TypeExpr`/`TypeBounds` validation APIs.
//!
//! This module must not duplicate validation rules.
//!
//! In particular, this file must not invent rules such as:
//!
//! ```text
//! a bound must be a trait
//! a bound must be quantum
//! a bound must be named
//! a bound must have one segment
//! ```
//!
//! Those are semantic questions or type-system rules and belong downstream.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no network access;
//! - executes no source code;
//! - performs no dynamic loading;
//! - uses no raw pointers;
//! - uses no `unsafe`;
//! - has no mutable global state;
//! - introduces no unbounded recursive algorithm.
//!
//! The underlying AST validation policy remains responsible for protection
//! against pathological compiler input.
//!
//! # Rust compatibility
//!
//! Required compatibility:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The file intentionally uses only ordinary Rust module/re-export features.
//!
//! # Public API contract
//!
//! This module exposes exactly the canonical representations needed by the
//! generic subsystem:
//!
//! ```text
//! Bound
//! Bounds
//! ```
//!
//! `Bound` is an alias of `TypeExpr`.
//!
//! `Bounds` is an alias of the canonical `TypeBounds` collection.
//!
//! No new data representation is introduced.
//!
//! # Migration contract
//!
//! Legacy code may contain a closed representation resembling:
//!
//! ```text
//! TypeBound::Trait(Identifier)
//! ```
//!
//! That representation must not be imported into this module.
//!
//! Migration must occur at a compatibility/parser/semantic boundary:
//!
//! ```text
//! legacy TypeBound
//!       │
//!       ▼
//! canonical TypeExpr
//!       │
//!       ▼
//! generics::bound::Bound
//!       │
//!       ▼
//! semantic constraint
//! ```
//!
//! The legacy `TypeBound` must not become authoritative again.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - one canonical representation exists for a single bound;
//! - that representation is `TypeExpr`;
//! - one canonical collection exists for multiple bounds;
//! - that collection is `TypeBounds`;
//! - no duplicate bound enum exists;
//! - no quantum-specific bound type exists;
//! - no hardware-specific bound type exists;
//! - no vendor-specific bound type exists;
//! - no fixed number of bounds exists;
//! - no machine-size assumption exists;
//! - no qubit-size assumption exists;
//! - no backend dependency exists;
//! - no parser dependency exists;
//! - no semantic dependency exists;
//! - no ZUIR dependency exists;
//! - serialization remains canonical;
//! - validation remains canonical;
//! - visitors remain canonical;
//! - deterministic ordering remains canonical;
//! - the generic parameter can continue using `Vec<TypeExpr>` without a
//!   forced representation migration;
//! - Rust 1.97/1.97.1 compatibility is preserved;
//! - `unsafe` is forbidden.
//!
//! # Required integration
//!
//! `generics/mod.rs` should expose this module and re-export the aliases:
//!
//! ```text
//! pub mod bound;
//! pub use bound::{Bound, Bounds};
//! ```
//!
//! `generics::parameter::TypeParameter` should continue using its existing
//! canonical `Vec<TypeExpr>` representation unless and until the whole generic
//! subsystem is deliberately migrated to `TypeBounds` in one coordinated API
//! change.
//!
//! That prevents this file from forcing unnecessary edits to an already
//! canonical representation.
//!
//! # =============================================================================
//! # Implementation
//! # =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical source-level representation of one generic bound.
///
/// This is intentionally an alias of the canonical [`TypeExpr`] rather than a
/// wrapper or enum. A generic bound is a type expression occurring in a
/// constraint position.
///
/// This preserves one authoritative type representation across the AST.
pub use super::super::types::type_expr::TypeExpr as Bound;

/// Canonical ordered collection of generic bounds.
///
/// This is intentionally the existing [`TypeBounds`] representation. It
/// preserves source order and stores canonical `TypeExpr` values.
pub use super::super::types::bounds::TypeBounds as Bounds;

#[cfg(test)]
mod tests {
    use super::{Bound, Bounds};
    use crate::frontend::ast::node::types::type_expr::{
        TypeExpr,
        TypeParameterName,
    };

    #[test]
    fn bound_is_canonical_type_expr() {
        let bound: Bound = TypeExpr::GenericParameter(
            TypeParameterName::from("T"),
        );

        assert_eq!(
            bound,
            TypeExpr::GenericParameter(TypeParameterName::from("T"))
        );
    }

    #[test]
    fn bounds_preserve_source_order() {
        let mut bounds = Bounds::new();

        bounds.push(TypeExpr::name("First"));
        bounds.push(TypeExpr::name("Second"));

        assert_eq!(bounds.len(), 2);
        assert_eq!(
            bounds
                .first()
                .expect("first bound must exist")
                .to_source_string(),
            "First"
        );
        assert_eq!(
            bounds
                .last()
                .expect("last bound must exist")
                .to_source_string(),
            "Second"
        );
    }

    #[test]
    fn empty_bounds_are_supported() {
        let bounds = Bounds::new();

        assert!(bounds.is_empty());
        assert_eq!(bounds.len(), 0);
    }

    #[test]
    fn bounds_are_not_hardware_specific() {
        let bound: Bound = TypeExpr::name("QuantumResource");

        assert_eq!(bound.to_source_string(), "QuantumResource");
    }

    #[test]
    fn nested_generic_bound_remains_canonical_type_expr() {
        let bound = TypeExpr::Generic {
            base: Box::new(TypeExpr::name("Resource")),
            arguments: vec![TypeExpr::GenericParameter(
                TypeParameterName::from("R"),
            )],
        };

        let canonical: Bound = bound.clone();

        assert_eq!(canonical, bound);
    }

    #[test]
    fn aliases_do_not_change_structural_identity() {
        let expression = TypeExpr::name("Numeric");
        let bound: Bound = expression.clone();

        assert_eq!(bound, expression);
    }
}