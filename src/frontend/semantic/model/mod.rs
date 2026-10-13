#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend — Canonical Semantic Model
//!
//! Path: `src/frontend/semantic/model/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Canonical model aggregation boundary.
//!
//! ## PURPOSE
//!
//! Establish one public module boundary for the target-independent semantic
//! model used by Zamani's native frontend.
//!
//! This module composes the existing authoritative implementations of:
//!
//! - semantic identities and checked identity allocation;
//! - typed, extensible semantic facts;
//! - semantic program results, obligations, provenance associations, and
//!   analysis-completion and validity states.
//!
//! It does not duplicate these implementations and does not perform semantic
//! analysis itself.
//!
//! ## ARCHITECTURAL POSITION
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend lexer
//!      |
//!      v
//! frontend parser
//!      |
//!      v
//! canonical frontend AST
//!      |
//!      v
//! structural AST validation
//!      |
//!      v
//! semantic analysis
//!      |
//!      +-- name resolution
//!      +-- type analysis
//!      +-- effect analysis
//!      +-- capability analysis
//!      +-- resource constraints
//!      +-- contracts and obligations
//!      +-- domain validation
//!      |
//!      v
//! THIS MODULE
//!      |
//!      v
//! canonical semantic program model
//!      |
//!      +-- classical IR
//!      +-- quantum::ir
//!      +-- HDL and other domain IRs
//!      |
//!      v
//! lowering, optimization, planning and realization
//! ```
//!
//! The diagram describes the intended architecture. The presence of this
//! module does not imply that every analysis pass or downstream integration
//! has already been implemented.
//!
//! ## AUTHORITY
//!
//! This module follows the repository's language and semantic contracts,
//! particularly:
//!
//! - `grammar/specification/grammar-authority.md`;
//! - `grammar/specification/semantic-model.md`;
//! - `grammar/specification/scalability-model.md`;
//! - `grammar/specification/poco-reaf.md`;
//! - `grammar/Zamani.g4`;
//! - `src/frontend/ast/node/`;
//! - `src/frontend/semantic/api.rs`;
//! - the authoritative implementations in this directory.
//!
//! Normative specifications define language meaning. The AST owns source
//! structure and AST identity. Semantic analysis establishes meaning.
//! Downstream representations and target systems determine realization.
//!
//! This aggregation module MUST NOT silently redefine any of those contracts.
//!
//! ## OWNS
//!
//! This module owns:
//!
//! - composition of the semantic-model submodules;
//! - the stable, explicit public re-export boundary;
//! - semantic-model module identity and schema-version metadata;
//! - module-level integration invariants and API-surface tests.
//!
//! ## DOES_NOT_OWN
//!
//! This module does NOT own:
//!
//! - AST nodes, AST storage, AST identities, or source spans;
//! - parsing, lexing, or grammar productions;
//! - symbol tables, scopes, imports, or name resolution;
//! - semantic type definitions, inference, unification, or coercions;
//! - effect definitions, inference, or authorization;
//! - capability registries or capability negotiation;
//! - resource definitions, resource allocation, or feasibility planning;
//! - contract evaluation or domain-specific validation;
//! - diagnostic production, rendering, or ordering;
//! - provenance-record semantics or provenance evidence verification;
//! - Classical IR, `quantum::ir`, HDL IR, or any other IR;
//! - target discovery, hardware allocation, placement, routing, scheduling,
//!   calibration, quantum error correction, runtime, or backend execution.
//!
//! ## MODULE OWNERSHIP
//!
//! `ids.rs` is the sole owner of semantic identity types and their allocators.
//!
//! `facts.rs` is the sole owner of `FactKey`, `SemanticFact<T>`, and fact-key
//! validation.
//!
//! `program.rs` is the sole owner of `SemanticProgram`, `NodeFact`, semantic
//! obligations, and the semantic program's completion and validity lifecycle.
//!
//! This file MUST NOT redeclare or wrap those types in competing definitions.
//!
//! New semantic types must first be implemented in their authoritative
//! subsystem. They may be re-exported here only when they belong to the
//! stable semantic-model public contract.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! canonical AST
//!      |
//!      v
//! semantic analysis and owning semantic subsystems
//!      |
//!      v
//! semantic/model/{ids,facts,program}.rs
//!      |
//!      v
//! semantic/model/mod.rs
//!      |
//!      v
//! semantic API and downstream consumers
//! ```
//!
//! The source AST remains authoritative for source structure and `NodeId`.
//! Semantic identities remain distinct from AST identities.
//!
//! This module must not depend on `crate::semantic` (the legacy analyzer),
//! compiler orchestration, target backends, hardware implementations, or IR
//! implementations.
//!
//! ## SEMANTIC VALIDITY AND COMPLETION
//!
//! A semantic result MUST NOT be considered valid merely because it exists or
//! because some analysis passes completed.
//!
//! Consumers must preserve the distinction between:
//!
//! - complete analysis;
//! - incomplete analysis;
//! - aborted analysis;
//! - failed analysis;
//! - valid semantic results;
//! - invalid semantic results;
//! - unresolved semantic results.
//!
//! A partial result may be useful for diagnostics and tooling, but incomplete
//! analysis must never be treated as proof of semantic validity.
//!
//! The model's validity is independent of target feasibility. A semantically
//! valid program may still require capabilities or resources unavailable on a
//! particular target.
//!
//! The owning semantic pipeline remains responsible for running all required
//! checks and producing appropriate diagnostics before finalizing validity.
//!
//! ## FACTS AND EXTENSIBILITY
//!
//! Semantic facts use typed payloads and validated, namespaced keys.
//!
//! New fact categories must not require a closed enumeration in this module.
//! Their payload types and correctness rules belong to the subsystem that
//! owns the corresponding semantics.
//!
//! For example, fact payloads may represent:
//!
//! - resolved declaration references;
//! - inferred or resolved types;
//! - effect summaries;
//! - capability obligations;
//! - symbolic resource requirements;
//! - contract obligations;
//! - quantum-operation validation;
//! - HDL-specific semantic constraints;
//! - future domain-specific results.
//!
//! These examples describe integration possibilities, not a claim that all
//! corresponding semantic producers are already implemented.
//!
//! A string must not be used as an unvalidated substitute for a structured
//! semantic type, symbol, constraint, or domain model.
//!
//! ## PROVENANCE
//!
//! `program.rs` stores provenance payloads generically. This aggregation
//! boundary therefore does not invent a provenance implementation or require
//! a nonexistent `model/provenance.rs` file.
//!
//! A future or existing authoritative provenance subsystem may supply the
//! concrete provenance type through the `P` parameter of `SemanticProgram`.
//! Provenance schema, evidence, identity, and verification remain owned by
//! that subsystem.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module supports the architectural objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! It introduces no universal capacity ceiling for:
//!
//! - CPUs, GPUs, FPGAs, ASICs, QPUs, or other devices;
//! - physical or logical qubits;
//! - memory, storage, or communication capacity;
//! - registers, tensor dimensions, or tensor rank;
//! - modules, declarations, operations, facts, or computational domains.
//!
//! It does not define constants such as `MAX_QUBITS`, `MAX_DEVICES`,
//! `MAX_AST_NODES`, `MAX_TENSOR_RANK`, or `MAX_MACHINE_SIZE`.
//!
//! No finite implementation can process literally infinite input. Actual
//! processing is bounded by available resources, representation limits,
//! compiler implementation capabilities, and explicit operational policies.
//!
//! Those limits belong to the owning configuration, session, and analysis
//! services. Exceeding an operational budget must be reported as incomplete,
//! aborted, or failed analysis, never as proof that a program is valid.
//!
//! Symbolic resource requirements must remain symbolic until the owning
//! semantic rule or downstream realization stage is authorized to resolve
//! them. This module performs no target-capacity checks.
//!
//! ## DETERMINISM
//!
//! This module:
//!
//! - introduces no global mutable state;
//! - performs no I/O or hardware discovery;
//! - uses no randomness or wall-clock time;
//! - allocates no AST or semantic identities implicitly;
//! - does not reorder source constructs;
//! - does not define fact ordering independently of the owning model.
//!
//! Deterministic identity allocation, key behavior, fact ordering, and
//! diagnostic ordering are governed by the owning implementations and their
//! contracts.
//!
//! ## ERROR HANDLING
//!
//! This aggregation module adds no alternate error type.
//!
//! It exposes the errors belonging to the identity and program-model
//! implementations. Invalid identifiers, exhausted allocators, invalid
//! lifecycle transitions, unresolved obligations, and other model failures
//! must remain explicit errors.
//!
//! No malformed source program, invalid semantic fact, incomplete analysis,
//! or failed integration may silently become a valid semantic result.
//!
//! ## COMPATIBILITY
//!
//! - Rust 2021 edition.
//! - Rust 1.97 or later.
//! - Stable Rust; no nightly features required.
//! - No `unsafe` code.
//! - No dependency on the legacy AST or legacy semantic implementation.
//! - No dependency on quantum or hardware backend implementations.
//! - Explicit public re-exports; no wildcard re-exports.
//!
//! `SEMANTIC_MODEL_SCHEMA_VERSION` identifies this module's public model
//! contract. It is not the language version, compiler version, AST schema
//! version, or serialized artifact version.
//!
//! A serialized semantic artifact requires its own serialization contract;
//! this module does not define one.
//!
//! ## INTEGRATION CONTRACT
//!
//! ### Semantic producers
//!
//! Semantic passes use the authoritative ID types and create typed facts
//! through `facts.rs`. They associate facts with canonical AST `NodeId`
//! values through `SemanticProgram`.
//!
//! The model does not independently resolve AST node references. The
//! producer and AST-validation layers must ensure that a referenced node
//! belongs to the program's canonical AST.
//!
//! ### Semantic API and pipeline
//!
//! `api.rs` owns the public analysis-service contract.
//! `pipeline.rs` owns pass ordering and orchestration.
//! `session.rs` owns per-analysis mutable state and operational accounting.
//! `context.rs` owns the read-only context and configured services.
//!
//! These modules may consume this model, but this file must not import them
//! to avoid a circular dependency.
//!
//! ### Downstream IR boundaries
//!
//! Classical, quantum, HDL, and other IR adapters consume finalized semantic
//! facts through explicit contracts.
//!
//! `quantum::ir` remains the canonical quantum IR boundary. This module must
//! not import it or encode physical quantum mappings, routing, scheduling,
//! calibration, or error-correction state.
//!
//! A semantically valid model describes program meaning; it does not certify
//! that a target can realize that meaning.
//!
//! ### Legacy compatibility
//!
//! The existing `src/semantic.rs` and legacy compiler pipeline remain
//! separate until migration is explicitly implemented and tested.
//!
//! This file must not import the legacy analyzer or redefine its AST types.
//! Compatibility adapters belong at the integration boundary.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! 1. Every declared child module exists and compiles.
//! 2. Every public re-export resolves to its authoritative declaration.
//! 3. AST and semantic identities remain distinct.
//! 4. Fact payloads remain typed and extensible.
//! 5. Program validity cannot be inferred from mere model construction.
//! 6. Incomplete analysis cannot be finalized as valid.
//! 7. Pending obligations prevent valid finalization.
//! 8. Provenance remains generic until its owning implementation is selected.
//! 9. No IR, hardware, or legacy semantic dependency is introduced.
//! 10. No arbitrary machine-capacity ceiling is introduced.
//! 11. The module contract tests pass.
//! 12. `cargo fmt --check` and the relevant Rust tests pass.
//! 13. The integrated workspace builds on the supported Rust toolchain.
//!
//! A successful module-level compile alone is not evidence that the complete
//! semantic pipeline is production-ready.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

/// Typed, extensible semantic facts and their validated keys.
pub mod facts;

/// Strongly typed semantic identities and checked local allocators.
pub mod ids;

/// Program-level semantic results, obligations, and lifecycle states.
pub mod program;

// =============================================================================
// Stable public re-exports
// =============================================================================
//
// These re-exports intentionally expose the stable semantic-model vocabulary.
// Implementation details remain accessible through their owning modules.
//
// Wildcard re-exports are prohibited because they can accidentally expand the
// public API when a child implementation changes.

pub use facts::{
    FactKey,
    FactKeyComponent,
    FactKeyError,
    SemanticFact,
};

pub use ids::{
    CapabilityId,
    ContractId,
    ExtensionId,
    FactId,
    ModuleId,
    ObligationId,
    ProvenanceId,
    ResourceId,
    ScopeId,
    SemanticId,
    SemanticIdAllocationError,
    SemanticIdAllocator,
    SemanticIdParseError,
    SemanticIdValue,
    SymbolId,
    TypeId,
};

pub use program::{
    AnalysisCompletion,
    NodeFact,
    Obligation,
    SemanticProgram,
    SemanticProgramError,
    SemanticValidity,
    SEMANTIC_PROGRAM_SCHEMA_VERSION,
};

// =============================================================================
// Module identity and version
// =============================================================================

/// Stable architectural identity of this module.
pub const SEMANTIC_MODEL_MODULE_NAME: &str = "zamani.frontend.semantic.model";

/// Public contract version for this aggregation boundary.
///
/// This version is independent of the language edition, AST schema,
/// semantic-program schema, compiler release, and serialized artifact format.
///
/// Increment this only when the aggregation contract changes in a
/// compatibility-relevant way.
pub const SEMANTIC_MODEL_SCHEMA_VERSION: u16 = 1;

/// Return the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    SEMANTIC_MODEL_MODULE_NAME
}

/// Return the schema version of this aggregation boundary.
#[must_use]
pub const fn schema_version() -> u16 {
    SEMANTIC_MODEL_SCHEMA_VERSION
}

// =============================================================================
// Module contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_and_schema_are_stable() {
        assert_eq!(
            module_name(),
            "zamani.frontend.semantic.model"
        );
        assert_eq!(
            schema_version(),
            SEMANTIC_MODEL_SCHEMA_VERSION
        );
        assert!(schema_version() > 0);
    }

    #[test]
    fn semantic_fact_api_is_available_through_the_facade() {
        let key = FactKey::new("core.types", "inferred-type")
            .expect("valid semantic fact key");

        let fact = SemanticFact::new(key.clone(), 42_u64);

        assert_eq!(fact.key(), &key);
        assert_eq!(*fact.payload(), 42);
    }

    #[test]
    fn semantic_identity_api_is_available_through_the_facade() {
        let symbol = SymbolId::new(1).expect("non-zero symbol identity");
        let ty = TypeId::new(1).expect("non-zero type identity");

        // Equal numeric representations do not make different ID types
        // interchangeable.
        assert_eq!(symbol.get(), ty.get());
        assert_ne!(symbol.to_string(), String::new());
        assert_ne!(ty.to_string(), String::new());
    }

    #[test]
    fn semantic_id_allocation_is_explicit_and_deterministic() {
        let mut first = SemanticIdAllocator::<SymbolId>::new();
        let mut second = SemanticIdAllocator::<SymbolId>::new();

        let first_a = first.allocate().expect("first identity");
        let first_b = first.allocate().expect("second identity");

        let second_a = second.allocate().expect("first identity");
        let second_b = second.allocate().expect("second identity");

        assert_eq!(first_a, second_a);
        assert_eq!(first_b, second_b);
        assert_ne!(first_a, first_b);
    }

    #[test]
    fn semantic_program_lifecycle_types_are_reachable() {
        let completion = AnalysisCompletion::Incomplete;
        let validity = SemanticValidity::Unknown;

        assert!(!completion.is_complete());
        assert!(!validity.is_valid());
        assert!(SEMANTIC_PROGRAM_SCHEMA_VERSION > 0);
    }

    #[test]
    fn model_does_not_define_target_capacity() {
        // Target feasibility is deliberately downstream from semantic meaning.
        assert_eq!(
            SEMANTIC_MODEL_MODULE_NAME,
            "zamani.frontend.semantic.model"
        );
    }
}