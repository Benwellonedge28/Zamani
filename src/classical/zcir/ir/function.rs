
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Function Integration
//!
//! Canonical path: `src/classical/zcir/ir/function.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## STATUS
//!
//! Integration adapter design. Repository compilation and integration tests
//! are required before this file can be frozen.
//!
//! ## PURPOSE
//!
//! Expose the canonical ZUIR function representation through the classical
//! intermediate representation (ZCIR) module boundary.
//!
//! A function represents a source-semantic callable independently of the
//! physical machine that will realize it. Its signature, body-region
//! references, linkage, declared effects, and attributes are owned by ZUIR.
//!
//! ZCIR reuses those definitions without introducing a competing function
//! entity, signature, identity, body representation, or effect model.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/functions/functions.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/classical/zcir/types/function.rs`
//!
//! ## OWNS
//!
//! This file owns only the ZCIR-facing public aliases and integration
//! documentation.
//!
//! The following canonical definitions remain owned by ZUIR:
//!
//! - `ZuirFunction` and its function identity.
//! - `FunctionParameter` and `FunctionSignature`.
//! - `FunctionBody` and its region references.
//! - `FunctionLinkage`.
//! - `FunctionError` and function-local validation.
//! - Namespaced function attributes and declared effects.
//! - Serialization and deserialization of function entities.
//!
//! ## DOES NOT OWN
//!
//! This file does not implement:
//!
//! - Source parsing, AST construction, name resolution, or type inference.
//! - A second function entity or function-signature representation.
//! - A second identity allocator or symbol registry.
//! - Region, block, operation, SSA, or control-flow storage.
//! - Whole-program symbol resolution or full-IR verification.
//! - Effect inference or effect authorization.
//! - Calling-convention implementation, ABI selection, or name mangling.
//! - Physical register allocation, memory layout, scheduling, or placement.
//! - Target lowering, executable generation, or runtime execution.
//! - Hardware-specific function limits.
//!
//! ## INPUTS
//!
//! The adapter consumes canonical ZUIR function entities and their
//! already-resolved typed identities. Function signatures contain canonical
//! type IDs; function definitions refer to canonical region IDs.
//!
//! Source declarations, resolved types, effects, contracts, requirements,
//! and provenance must be supplied by the appropriate upstream semantic and
//! IR construction stages.
//!
//! ## OUTPUTS
//!
//! The public ZCIR API exposes aliases for the canonical ZUIR function,
//! signature, parameter, body, linkage, and error types.
//!
//! Aliases preserve exact Rust type identity. They do not wrap, copy,
//! serialize, or transform the underlying representation.
//!
//! ## DEPENDENCIES AND INTEGRATION
//!
//! `src/zuir/ir/function.rs` is the sole implementation authority.
//!
//! `src/zuir/ids.rs` owns FunctionId, SymbolId, ValueId, TypeId, and RegionId.
//! IDs are opaque identity tokens, not array indices or physical addresses.
//!
//! `src/zuir/ir/module.rs` stores function payloads keyed by FunctionId.
//! The module builder or full verifier must confirm that each map key agrees
//! with the embedded `ZuirFunction::id()`.
//!
//! `src/zuir/ir/region.rs` owns regions. The full verifier must check that
//! every referenced body region exists, has the correct owner, and is
//! structurally valid.
//!
//! `src/zuir/schema/attributes.rs` owns attribute keys and values.
//! `src/zuir/schema/types.rs` owns qualified names used by effects and
//! calling-convention declarations.
//!
//! `src/classical/zcir/types/function.rs` handles function *types*. It must
//! not be replaced by this function-entity adapter or used as a substitute
//! for function bodies, linkage, or function identity.
//!
//! `src/classical/zcir/ir/mod.rs` must declare this module and re-export the
//! intended public names. The ZCIR root must then expose the IR boundary.
//! These parent modules were absent at the inspected repository paths and
//! must be integrated deliberately; this file must not pretend to register
//! nonexistent modules.
//!
//! `src/quantum/ir/` and `src/quantum/ir/classical/` retain their existing
//! ownership of quantum-specific and established classical representations.
//! Cross-domain adapters must preserve canonical identities and semantics.
//!
//! ## GRAMMAR AND SEMANTIC MAPPING
//!
//! The canonical root grammar `grammar/Zamani.g4` composes the parser and
//! lexer. `grammar/functions/functions.g4` owns named-function declaration
//! framing and references the canonical parameter, return-type, effects,
//! contract, statement, and type grammar components.
//!
//! The frontend builds AST nodes and semantic analysis resolves source names,
//! types, effects, contracts, capabilities, and resource requirements.
//! This adapter receives the resulting canonical IR entity; it does not
//! interpret grammar productions.
//!
//! The IR preserves declared effects and calling-convention identifiers.
//! Their authorization, meaning, and target support must be established by
//! their respective registries, verifiers, and lowering implementations.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This adapter imposes no fixed maximum number of parameters, results,
//! effects, attributes, functions, regions, processors, devices, or workers.
//! It introduces no fixed register width, memory capacity, or target count.
//!
//! Collection sizes and processing costs remain subject to finite address
//! space, available memory, execution time, and implementation limits.
//! No finite implementation promises literal infinity.
//!
//! Resource budgets belong to explicit construction, verification,
//! compilation, and execution policies. Exhaustion must be reported rather
//! than silently truncating a signature, dropping an effect, or changing
//! program semantics.
//!
//! Portability does not guarantee that every target supports every function.
//! Unsupported effects, types, linkage modes, and calling conventions must
//! produce explicit feasibility or lowering diagnostics.
//!
//! ## DETERMINISM AND TRUST
//!
//! The canonical function implementation uses ordered collections for
//! attributes and effects, supporting deterministic traversal.
//!
//! A decoded function must pass `validate()` and then full-IR verification.
//! Local validation does not prove that referenced types, symbols, or regions
//! exist, or that effects and attributes are authorized.
//!
//! Mutations may invalidate previous verification results. Consumers must
//! revalidate and re-run the applicable full verifier before trusting the
//! modified entity for optimization, lowering, or execution.
//!
//! ## COMPATIBILITY
//!
//! The aliases do not introduce a new serialized representation. The
//! canonical ZUIR serialization and versioning contracts remain authoritative.
//! Any future change to those contracts must be reviewed as a ZUIR change,
//! not silently implemented as a separate ZCIR function schema.
//!
//! ## FREEZE CRITERIA
//!
//! - All aliases resolve to the actual canonical ZUIR definitions.
//! - The parent module tree exports this adapter exactly once.
//! - The adapter introduces no duplicate IR definitions or identities.
//! - Unit tests prove type identity and delegated behavior.
//! - Full-IR verification checks cross-entity references.
//! - Formatting, compilation, tests, and linting pass in the actual checkout.
//! - No unsafe code or unjustified universal capacity limits are introduced.

// =============================================================================
// Canonical ZUIR function types exposed through ZCIR
// =============================================================================

/// Canonical function entity exposed through the classical IR boundary.
///
/// This is exactly `crate::zuir::ir::function::ZuirFunction`.
pub use crate::zuir::ir::function::ZuirFunction as ZcirFunction;

/// Canonical logical function signature.
pub use crate::zuir::ir::function::FunctionSignature as ZcirFunctionSignature;

/// Canonical function parameter.
pub use crate::zuir::ir::function::FunctionParameter as ZcirFunctionParameter;

/// Canonical declaration/definition body descriptor.
pub use crate::zuir::ir::function::FunctionBody as ZcirFunctionBody;

/// Canonical function linkage classification.
pub use crate::zuir::ir::function::FunctionLinkage as ZcirFunctionLinkage;

/// Canonical function-local error.
pub use crate::zuir::ir::function::FunctionError as ZcirFunctionError;

/// Result type for operations that report canonical function-local errors.
pub type ZcirFunctionResult<T> = Result<T, ZcirFunctionError>;

/// Canonical function entity under an explicitly classical-facing name.
///
/// This is an alias, not a second representation.
pub type ClassicalZcirFunction = ZcirFunction;

/// Canonical function signature under an explicitly classical-facing name.
///
/// This is an alias, not a second signature model.
pub type ClassicalZcirFunctionSignature = ZcirFunctionSignature;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{
        FunctionId,
        RegionId,
        SymbolId,
        TypeId,
        ValueId,
    };
    use crate::zuir::ir::function::{
        FunctionSignature,
        ZuirFunction,
    };

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("function ID must be nonzero")
    }

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("symbol ID must be nonzero")
    }

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("region ID must be nonzero")
    }

    fn type_id(value: u128) -> TypeId {
        TypeId::new(value).expect("type ID must be nonzero")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("value ID must be nonzero")
    }

    #[test]
    fn zcir_function_is_the_canonical_zuir_function_type() {
        fn accepts_zcir(_: ZcirFunction) {}
        fn accepts_zuir(_: ZuirFunction) {}

        let signature = ZcirFunctionSignature::new(Vec::new(), Vec::new());
        let function =
            ZcirFunction::declaration(function_id(1), symbol_id(2), signature);

        accepts_zcir(function.clone());
        accepts_zuir(function.clone());

        assert_eq!(
            core::any::type_name::<ZcirFunction>(),
            core::any::type_name::<ZuirFunction>(),
        );

        assert_eq!(
            core::any::type_name::<ClassicalZcirFunction>(),
            core::any::type_name::<ZuirFunction>(),
        );

        assert_eq!(function.id(), function_id(1));
        assert_eq!(function.symbol(), symbol_id(2));
    }

    #[test]
    fn declaration_preserves_signature_and_has_no_body_regions() {
        let parameter = ZcirFunctionParameter::new(
            value_id(10),
            type_id(20),
            None,
        );

        let signature =
            ZcirFunctionSignature::new(vec![parameter], vec![type_id(30)]);

        let function =
            ZcirFunction::declaration(function_id(1), symbol_id(2), signature);

        assert!(function.body().is_declaration());
        assert!(function.body().regions().is_empty());

        assert_eq!(function.signature().parameters().len(), 1);
        assert_eq!(function.signature().parameters()[0].value(), value_id(10));
        assert_eq!(function.signature().parameters()[0].ty(), type_id(20));
        assert_eq!(function.signature().results(), &[type_id(30)]);

        assert!(function.validate().is_ok());
    }

    #[test]
    fn definition_preserves_canonical_region_references() {
        let signature = ZcirFunctionSignature::new(Vec::new(), Vec::new());

        let function = ZcirFunction::defined(
            function_id(1),
            symbol_id(2),
            signature,
            vec![region_id(3), region_id(4)],
        )
        .expect("valid definition");

        assert!(!function.body().is_declaration());
        assert_eq!(function.body().regions(), &[region_id(3), region_id(4)]);
        assert!(function.validate().is_ok());
    }

    #[test]
    fn invalid_definition_is_rejected_by_canonical_constructor() {
        let signature = ZcirFunctionSignature::new(Vec::new(), Vec::new());

        let result = ZcirFunction::defined(
            function_id(1),
            symbol_id(2),
            signature,
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(ZcirFunctionError::DefinitionWithoutRegions),
        );
    }

    #[test]
    fn duplicate_parameter_values_are_rejected() {
        let first = ZcirFunctionParameter::new(
            value_id(10),
            type_id(20),
            None,
        );

        let second = ZcirFunctionParameter::new(
            value_id(10),
            type_id(21),
            None,
        );

        let signature =
            ZcirFunctionSignature::new(vec![first, second], Vec::new());

        assert_eq!(
            signature.validate(),
            Err(ZcirFunctionError::DuplicateParameterValue(value_id(10))),
        );
    }

    #[test]
    fn function_validation_is_delegated_to_the_canonical_implementation() {
        let signature = ZcirFunctionSignature::new(Vec::new(), Vec::new());

        let function =
            ZcirFunction::declaration(function_id(1), symbol_id(2), signature);

        let canonical: &ZuirFunction = &function;
        assert!(canonical.validate().is_ok());
    }

    #[test]
    fn linkage_and_effect_contracts_are_preserved() {
        let signature = ZcirFunctionSignature::new(Vec::new(), Vec::new());

        let mut function =
            ZcirFunction::declaration(function_id(1), symbol_id(2), signature);

        function.set_linkage(ZcirFunctionLinkage::External);
        assert_eq!(function.linkage(), ZcirFunctionLinkage::External);

        let effect = crate::zuir::schema::types::QualifiedName::new(
            vec!["core", "io"],
        )
        .expect("valid effect name");

        assert!(function.add_effect(effect.clone()).is_ok());
        assert!(function.declares_effect(&effect));
        assert!(function.validate().is_ok());
    }
}
