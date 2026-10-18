
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Module Integration
//!
//! Canonical path: `src/classical/zcir/ir/module.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Establish the module-level representation boundary for Zamani's Classical
//! Intermediate Representation (ZCIR), reusing the canonical ZUIR module
//! implementation instead of introducing a competing representation.
//!
//! A classical module is a target-independent organizational unit containing
//! function declarations or definitions, symbol imports and exports, and
//! descriptive metadata. Its functions use the canonical ZUIR function
//! representation or another explicitly compatible function payload.
//!
//! This file deliberately defines no independent module storage, identity
//! allocator, symbol table, naming grammar, serialization format, or
//! verification implementation.
//!
//! ## Architectural authority
//!
//! This module conforms to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/program.rs`
//!
//! The grammar defines source syntax. The semantic model defines source
//! meaning. ZUIR owns the canonical shared IR representation. ZCIR provides
//! the classical-domain integration boundary.
//!
//! ## Ownership
//!
//! This module owns only its public integration names and associated
//! documentation.
//!
//! The following definitions remain exclusively owned by ZUIR:
//!
//! - `ModuleId`, `FunctionId`, and `SymbolId`.
//! - `ModuleName` and its name-validation rules.
//! - `ModuleError` and module-local validation.
//! - `ZuirModule<F>` and its function, import, export, and metadata storage.
//! - Serialization and deserialization contracts for the module container.
//!
//! ## Does not own
//!
//! This module does not implement:
//!
//! - Parsing, AST construction, name resolution, or semantic analysis.
//! - Source-level type inference or language semantics.
//! - A second classical function or module representation.
//! - Function-body, block, operation, region, SSA, or control-flow semantics.
//! - Whole-program symbol resolution or cross-module linking.
//! - Dialect registration or extension execution.
//! - Resource discovery, physical allocation, scheduling, or execution.
//! - Optimization, target lowering, or backend code generation.
//! - A second source map, diagnostic system, or identity allocator.
//!
//! ## Canonical representation
//!
//! `ZcirModule<F>` is a public alias for the exact `ZuirModule<F>` type.
//! It is not a wrapper and does not introduce a second identity or storage
//! model. Values can cross the ZCIR/ZUIR boundary without converting the
//! module container itself.
//!
//! The caller chooses `F` according to the canonical function contract
//! required by the consuming pipeline. Once the repository's standard
//! `ZuirFunction` is available at the public module path, consumers should
//! use that concrete function type at ordinary compiler boundaries.
//!
//! ## Identity and consistency
//!
//! IDs are supplied by the canonical identity owner. This module does not
//! generate IDs or infer identity from names or collection positions.
//!
//! The canonical module rejects duplicate function IDs instead of silently
//! replacing existing functions. The enclosing program remains responsible
//! for module membership and cross-module identity consistency.
//!
//! Local validation does not prove that function IDs agree with embedded
//! function identities, that imported symbols exist, or that exports are
//! valid. The authoritative full-IR verifier and symbol resolver must check
//! those relationships before optimization, lowering, or execution.
//!
//! ## Determinism
//!
//! The canonical implementation uses ordered collections for function
//! membership, imports, exports, and metadata. This preserves deterministic
//! traversal for a given module state.
//!
//! Deterministic collection order alone does not establish a canonical
//! serialized artifact or a stable cross-build fingerprint. Those guarantees
//! belong to the canonical serialization and fingerprint contracts.
//!
//! ## POCO-REAF and scalability
//!
//! This integration introduces no fixed limit on the number of functions,
//! imports, exports, metadata entries, processors, devices, or operations.
//!
//! Collections grow according to available resources. Actual memory,
//! address-space, execution-time, allocator, and implementation constraints
//! remain finite. Literal infinite computation is not promised.
//!
//! Explicit operational budgets belong to the applicable configuration,
//! construction, verification, serialization, and execution boundaries.
//! Budget exhaustion must be reported explicitly rather than silently
//! truncating a module or changing its meaning.
//!
//! ## Integration contract
//!
//! 1. `src/classical/zcir/ir/mod.rs` declares `pub mod module;`.
//! 2. `src/classical/zcir/ir/mod.rs` re-exports the selected public aliases.
//! 3. `src/classical/zcir/mod.rs` exposes the IR boundary only after the
//!    required child modules and their dependencies compile.
//! 4. `src/classical/mod.rs` registers ZCIR through its existing classical
//!    module boundary; it must not register a duplicate IR root.
//! 5. Builders use the canonical IDs and construct canonical modules.
//! 6. The full verifier checks embedded identities, symbol resolution,
//!    function bodies, types, effects, and cross-entity invariants.
//! 7. Serialization validates decoded modules before downstream use.
//! 8. ZUIR conversion and lowering reuse the canonical container and preserve
//!    the identity and semantic-preservation guarantees of their contracts.
//! 9. Quantum/classical interoperability continues through the existing
//!    `src/quantum/ir/` and `src/quantum/ir/classical/` boundaries.
//! 10. The frontend supplies resolved semantic meaning; this file never
//!     parses grammar productions or performs semantic analysis.
//!
//! ## Freeze criteria
//!
//! - The actual ZUIR module API is reused without duplication.
//! - Public module paths resolve through the repository's real module tree.
//! - The aliases are type-identical to their canonical ZUIR definitions.
//! - Tests cover construction, validation, duplicate insertion, deterministic
//!   access, and integration with canonical identifiers.
//! - Formatting, compilation, tests, and linting pass in the actual checkout.
//!
//! This file is an integration adapter. Its readiness does not imply that
//! the whole ZCIR subsystem or POCO-REAF pipeline is production-ready.

/// Canonical ZUIR module container, exposed under the ZCIR name.
///
/// This is the exact canonical type, not a duplicate representation.
///
/// Use `ZcirModule<ZuirFunction>` at compiler boundaries where the canonical
/// function implementation is available. Generic payloads remain useful for
/// focused tests and staged implementation of the IR.
pub use crate::zuir::ir::module::ZuirModule as ZcirModule;

/// Canonical module name, exposed under the ZCIR name.
pub use crate::zuir::ir::module::ModuleName as ZcirModuleName;

/// Canonical module-local error, exposed under the ZCIR name.
pub use crate::zuir::ir::module::ModuleError as ZcirModuleError;

/// Result contract for ZCIR module-local operations.
///
/// This alias preserves the canonical module error rather than introducing
/// a competing ZCIR error representation.
pub type ZcirModuleResult<T> = Result<T, ZcirModuleError>;

/// Concrete module alias for classical functions represented by the
/// canonical ZUIR function implementation.
///
/// This alias is available for consumers that prefer an explicit classical
/// module name. It is exactly the same type as `ZcirModule<ZuirFunction>`.
///
/// The concrete `ZuirFunction` import is intentionally left to the consumer
/// until the repository's public function re-export is finalized.
pub type ClassicalZcirModule<F> = ZcirModule<F>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{FunctionId, ModuleId, SymbolId};
    use crate::zuir::ir::module::ZuirModule;

    fn module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("module identity must be nonzero")
    }

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("function identity must be nonzero")
    }

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("symbol identity must be nonzero")
    }

    #[test]
    fn zcir_module_is_the_canonical_zuir_module_type() {
        fn accepts_zcir(_: ZcirModule<String>) {}
        fn accepts_zuir(_: ZuirModule<String>) {}

        let module =
            ZcirModule::<String>::named(module_id(1), "classical.main")
                .expect("valid module");

        accepts_zcir(module.clone());
        accepts_zuir(module.clone());

        assert_eq!(
            core::any::type_name::<ZcirModule<String>>(),
            core::any::type_name::<ZuirModule<String>>()
        );

        assert_eq!(module.id(), module_id(1));
        assert_eq!(module.name().as_str(), "classical.main");
    }

    #[test]
    fn module_construction_validates_names() {
        let valid = ZcirModule::<String>::named(module_id(1), "classical.math")
            .expect("valid module name");

        assert!(valid.validate().is_ok());

        for invalid_name in [
            "",
            ".classical",
            "classical.",
            "classical..math",
            "classical/math",
            "classical module",
            " classical",
            "classical ",
        ] {
            assert!(
                ZcirModule::<String>::named(module_id(2), invalid_name).is_err(),
                "invalid module name accepted: {invalid_name:?}"
            );
        }
    }

    #[test]
    fn duplicate_function_insertion_does_not_replace_the_original() {
        let mut module =
            ZcirModule::named(module_id(1), "classical.main")
                .expect("valid module");

        module
            .insert_function(function_id(1), String::from("original"))
            .expect("first insertion succeeds");

        let result =
            module.insert_function(function_id(1), String::from("replacement"));

        let (error, rejected_value) =
            result.expect_err("duplicate function identity must be rejected");

        assert_eq!(error, ZcirModuleError::DuplicateFunction(function_id(1)));
        assert_eq!(rejected_value, "replacement");
        assert_eq!(module.function(function_id(1)), Some(&String::from("original")));
        assert_eq!(module.function_count(), 1);
    }

    #[test]
    fn functions_have_deterministic_identity_order() {
        let mut module =
            ZcirModule::named(module_id(1), "classical.main")
                .expect("valid module");

        module
            .insert_function(function_id(30), String::from("thirty"))
            .expect("insertion succeeds");
        module
            .insert_function(function_id(10), String::from("ten"))
            .expect("insertion succeeds");
        module
            .insert_function(function_id(20), String::from("twenty"))
            .expect("insertion succeeds");

        let ids: Vec<_> = module
            .functions()
            .map(|(id, _)| id.get())
            .collect();

        assert_eq!(ids, vec![10, 20, 30]);
    }

    #[test]
    fn imports_exports_and_metadata_use_canonical_storage() {
        let mut module =
            ZcirModule::named(module_id(1), "classical.main")
                .expect("valid module");

        let imported = symbol_id(10);
        let exported = symbol_id(20);

        assert!(module.add_import(imported));
        assert!(!module.add_import(imported));
        assert!(module.has_import(imported));

        assert!(module.add_export(exported));
        assert!(!module.add_export(exported));
        assert!(module.has_export(exported));

        assert_eq!(
            module.set_metadata("documentation", "classical entry point"),
            Ok(None)
        );
        assert_eq!(
            module.metadata("documentation"),
            Some("classical entry point")
        );

        assert!(module.validate().is_ok());
    }

    #[test]
    fn invalid_metadata_keys_are_rejected() {
        let mut module =
            ZcirModule::named(module_id(1), "classical.main")
                .expect("valid module");

        for key in ["", " leading", "trailing ", "   "] {
            assert!(
                module.set_metadata(key, "value").is_err(),
                "invalid metadata key accepted: {key:?}"
            );
        }
    }

    #[test]
    fn mutation_and_removal_preserve_container_invariants() {
        let mut module =
            ZcirModule::named(module_id(1), "classical.main")
                .expect("valid module");

        module
            .insert_function(function_id(1), String::from("before"))
            .expect("insertion succeeds");

        if let Some(function) = module.function_mut(function_id(1)) {
            *function = String::from("after");
        } else {
            panic!("inserted function must exist");
        }

        assert_eq!(
            module.function(function_id(1)).map(String::as_str),
            Some("after")
        );

        assert_eq!(
            module.remove_function(function_id(1)),
            Some(String::from("after"))
        );

        assert!(module.is_empty());
        assert!(module.validate().is_ok());
    }

    #[test]
    fn empty_module_is_valid_and_has_no_implicit_capacity_limit() {
        let module =
            ZcirModule::<String>::named(module_id(1), "classical.empty")
                .expect("valid module");

        assert!(module.is_empty());
        assert_eq!(module.function_count(), 0);
        assert!(module.validate().is_ok());
    }
}
