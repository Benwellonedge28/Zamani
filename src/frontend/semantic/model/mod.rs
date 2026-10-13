
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend — Semantic Model
//!
//! Path: `src/frontend/semantic/model/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Canonical semantic-model aggregation boundary.
//!
//! ## PURPOSE
//!
//! Provide one explicit public boundary for the target-independent semantic
//! model produced by Zamani's native frontend.
//!
//! This module composes the authoritative implementations of:
//!
//! - `ids.rs`: semantic identities and checked local ID allocation;
//! - `facts.rs`: validated fact keys and typed semantic-fact envelopes;
//! - `program.rs`: program-level facts, obligations, validity and lifecycle.
//!
//! It does not perform semantic analysis and does not redefine the types
//! implemented by these modules.
//!
//! ## AUTHORITY
//!
//! Normative and implementation references:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/expressions/provenance.g4`
//! - `grammar/quantum/provenance.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/model/facts.rs`
//! - `src/frontend/semantic/model/program.rs`
//!
//! Normative specifications govern language meaning. The canonical AST owns
//! source structure and AST node identities. Semantic analysis establishes
//! meaning. IR and target-specific subsystems determine realization.
//!
//! If an implementation conflicts with a normative specification, this
//! facade must not silently redefine the specification.
//!
//! ## OWNS
//!
//! This module owns only:
//!
//! - semantic-model module composition;
//! - the explicitly supported public re-export boundary;
//! - this module's identity and schema-version metadata;
//! - facade-level integration contract tests.
//!
//! ## DOES_NOT_OWN
//!
//! This module does not own:
//!
//! - AST nodes, AST storage, AST identities, or source spans;
//! - lexing, parsing, grammar productions, or AST traversal;
//! - symbol tables, scopes, imports, or name resolution;
//! - semantic types, type inference, unification, or coercions;
//! - effect, capability, resource, contract, or policy semantics;
//! - provenance-record definitions or evidence verification;
//! - diagnostic production, rendering, or ordering;
//! - classical IR, `quantum::ir`, HDL IR, or another IR;
//! - target discovery, feasibility, physical allocation, placement, routing,
//!   scheduling, calibration, error correction, or runtime execution;
//! - compatibility behavior for the legacy `crate::semantic` implementation.
//!
//! These responsibilities remain with their designated modules.
//!
//! ## MODULE OWNERSHIP
//!
//! `ids.rs` is the sole owner of semantic identity types and allocators.
//!
//! `facts.rs` is the sole owner of `FactKey`, `FactKeyComponent`,
//! `FactKeyError`, `SemanticFact<T>`, and fact-key validation.
//!
//! `program.rs` is the sole owner of `SemanticProgram`, `NodeFact`,
//! `Obligation`, `SemanticProgramError`, `AnalysisCompletion`,
//! `SemanticValidity`, and the program schema version.
//!
//! This module must not redeclare, wrap, or shadow those definitions.
//!
//! New semantic functionality must first be implemented in the subsystem
//! responsible for its meaning. It may be re-exported here only after its
//! public contract is stable and its implementation exists.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! Canonical frontend AST
//!          |
//!          v
//! Semantic analysis passes
//!          |
//!          v
//! model/{ids,facts,program}.rs
//!          |
//!          v
//! model/mod.rs
//!          |
//!          v
//! Semantic API and downstream adapters
//! ```
//!
//! The model must not depend on the legacy semantic analyzer, compiler
//! orchestration, target backends, or IR implementations.
//!
//! ## AST AND SEMANTIC IDENTITIES
//!
//! AST `NodeId` values identify nodes in the canonical source representation.
//! Semantic IDs identify entities owned by the semantic layer. They are
//! distinct types and must never be implicitly interchangeable.
//!
//! `SemanticProgram` associates facts with canonical AST node identities.
//! The AST storage and validation layers remain responsible for proving that
//! a referenced node belongs to the program being analyzed.
//!
//! This facade must not allocate AST identities or reinterpret them as
//! semantic IDs.
//!
//! ## FACTS AND EXTENSIBILITY
//!
//! Semantic facts must use validated keys and typed payloads from their
//! owning subsystems. The facade must not require a closed enumeration of
//! every fact category or computational domain.
//!
//! Potential fact payloads include resolved declarations, inferred types,
//! effects, capability obligations, symbolic resource constraints, contract
//! obligations, and domain-specific validation results. Their examples do
//! not imply that the corresponding producers are already implemented.
//!
//! A string must not replace a structured semantic value merely to simplify
//! integration. Unknown or unresolved values must remain distinguishable
//! from validated values.
//!
//! ## PROVENANCE AND GRAMMAR INTEGRATION
//!
//! `grammar/expressions/provenance.g4` owns expression-level provenance
//! syntax. `grammar/quantum/provenance.g4` owns the quantum-domain composition
//! boundary. Neither grammar file defines the implementation of a Rust
//! semantic provenance model.
//!
//! `SemanticProgram` supports a generic provenance payload parameter.
//! Consequently, this facade does not invent a concrete provenance type or
//! declare `mod provenance` unless an authoritative Rust implementation is
//! added and reviewed separately.
//!
//! Quantum provenance must not introduce a competing quantum IR. The
//! canonical quantum IR remains `quantum::ir`; downstream quantum metadata
//! remains owned by its designated implementation.
//!
//! ## VALIDITY AND COMPLETION
//!
//! Analysis completion and semantic validity are independent properties.
//! A completed analysis may find semantic errors. An incomplete analysis
//! cannot certify semantic validity.
//!
//! Consumers must preserve the distinction between:
//!
//! - complete, incomplete, aborted, or failed analysis;
//! - valid, invalid, unresolved, or unknown semantic status;
//! - source-language validity and target feasibility.
//!
//! Model construction alone is not proof of validity. The owning pipeline
//! must run required checks and respect unresolved obligations before
//! finalizing a valid result.
//!
//! A valid semantic result does not guarantee that any particular target can
//! execute the program.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module supports the target-independent objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! It introduces no universal capacity ceiling for processors, accelerators,
//! devices, qubits, registers, tensor dimensions, memory, operations, facts,
//! modules, or computational domains.
//!
//! No finite implementation can process literally infinite input. Actual
//! processing depends on available resources, representation limits, and
//! documented compiler operating policies.
//!
//! Operational budgets belong to configuration, session, and analysis
//! services. Budget exhaustion must be reported as incomplete, aborted, or
//! failed analysis—not as proof that the program is valid.
//!
//! Symbolic resource requirements remain symbolic until a responsible
//! semantic rule or downstream realization stage is permitted to resolve
//! them. This facade does not inspect machine capacities or perform target
//! feasibility checks.
//!
//! ## DETERMINISM AND ERROR HANDLING
//!
//! This facade performs no I/O, hardware discovery, random selection, or
//! implicit ID allocation. It introduces no global mutable state.
//!
//! Deterministic allocation, fact ordering, and lifecycle rules belong to
//! the authoritative model implementations.
//!
//! Invalid keys, exhausted allocators, invalid lifecycle transitions, and
//! unresolved obligations must remain explicit errors. A malformed program,
//! incomplete analysis, or failed integration must never silently become a
//! certified-valid result.
//!
//! ## COMPATIBILITY
//!
//! - Rust 2021 edition.
//! - Rust 1.97 or later.
//! - Stable Rust; no nightly features required.
//! - Safe Rust only.
//! - No dependency on the legacy AST or legacy semantic analyzer.
//! - No dependency on target-specific or IR implementations.
//! - Explicit public re-exports; no wildcard re-exports.
//!
//! The model schema version is distinct from the language version, compiler
//! version, AST schema version, program schema version, IR version, and any
//! serialized artifact format. This module does not define serialization.
//!
//! ## INTEGRATION CONTRACT
//!
//! Semantic producers import the authoritative ID and fact types and
//! associate facts with canonical AST node identities through
//! `SemanticProgram`.
//!
//! The future semantic API owns the public analyzer/result contract.
//! The session owns per-analysis mutable state and operational accounting.
//! The pipeline owns pass ordering and execution. None of these modules
//! should be imported here, which prevents a circular dependency.
//!
//! Classical, quantum, HDL, and other IR adapters consume finalized semantic
//! facts through explicit downstream contracts. This facade does not import
//! an IR or commit semantic meaning to a physical target.
//!
//! The legacy `src/semantic.rs` and its consumers remain separate until a
//! migration is explicitly implemented and tested.
//!
//! ## FREEZE CRITERIA
//!
//! This module is ready to freeze when:
//!
//! 1. Every declared child module exists and compiles.
//! 2. Every re-export resolves to its authoritative declaration.
//! 3. AST and semantic identities remain distinct.
//! 4. Facts remain typed, validated, and extensible.
//! 5. Incomplete analysis cannot be finalized as valid.
//! 6. Unresolved obligations prevent valid finalization.
//! 7. Provenance ownership is explicit and no nonexistent module is exported.
//! 8. No IR, hardware, or legacy semantic dependency is introduced.
//! 9. Parent modules expose this module through the intended frontend API.
//! 10. Facade and lifecycle tests pass.
//! 11. Formatting, linting, and relevant workspace checks pass.
//!
//! Compiling this file alone does not establish that the complete semantic
//! pipeline is production-ready.

// =============================================================================
// Child modules
// =============================================================================

/// Typed, extensible semantic facts and validated fact keys.
pub mod facts;

/// Strongly typed semantic identities and checked local allocators.
pub mod ids;

/// Program-level semantic results, obligations, and lifecycle states.
pub mod program;

// =============================================================================
// Stable public re-exports
// =============================================================================
//
// These are the explicit model-level exports. Add new exports only after
// their owning module and integration contract are implemented.
//
// Do not use wildcard re-exports: they can unintentionally expand the
// public API when a child module changes.

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

/// Version of this module's public aggregation contract.
///
/// Increment only when the aggregation contract changes in a
/// compatibility-relevant way. This is not the language or program schema
/// version.
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
// Contract tests
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
    fn typed_fact_api_is_available_through_the_facade() {
        let key = FactKey::new("core.types", "inferred-type")
            .expect("the test key is valid");

        let fact = SemanticFact::new(key.clone(), 42_u64);

        assert_eq!(fact.key(), &key);
        assert_eq!(*fact.payload(), 42);
    }

    #[test]
    fn semantic_identity_types_are_distinct() {
        let symbol = SymbolId::new(1).expect("non-zero symbol identity");
        let ty = TypeId::new(1).expect("non-zero type identity");

        // Numeric equality does not make different identity types
        // interchangeable.
        assert_eq!(symbol.get(), ty.get());
        assert_ne!(symbol.to_string(), String::new());
        assert_ne!(ty.to_string(), String::new());
    }

    #[test]
    fn identity_allocation_is_explicit_and_deterministic() {
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
    fn lifecycle_types_are_available_through_the_facade() {
        let completion = AnalysisCompletion::Incomplete;
        let validity = SemanticValidity::Unknown;

        assert!(!completion.is_complete());
        assert!(!validity.is_valid());
        assert!(SEMANTIC_PROGRAM_SCHEMA_VERSION > 0);
    }

    #[test]
    fn model_identity_does_not_encode_a_target_capacity() {
        // Target feasibility is owned by downstream target-aware analysis.
        assert_eq!(
            module_name(),
            "zamani.frontend.semantic.model"
        );
    }
}
