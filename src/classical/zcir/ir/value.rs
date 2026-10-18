
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Value Identity and Definition Facade
//!
//! Canonical path: `src/classical/zcir/ir/value.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Expose the canonical ZUIR value identity, value-definition, and value-use
//! APIs through the classical ZCIR integration boundary.
//!
//! This module is a facade over canonical ZUIR types. It deliberately does
//! not introduce a second SSA value struct, value allocator, type system,
//! value-storage model, or definition-use index.
//!
//! ## AUTHORITY
//!
//! The implementation must conform to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/types/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/types.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! These sources have different responsibilities. Grammar defines source
//! syntax; semantic analysis establishes resolved meaning; ZUIR owns the
//! canonical shared IR identities and definitions; ZCIR exposes those
//! representations for classical-computing consumers.
//!
//! ## OWNS
//!
//! This file owns only:
//!
//! - ZCIR-facing aliases for canonical ZUIR value APIs.
//! - The public integration boundary for value identities and definitions.
//! - Tests guarding against accidental duplicate value representations.
//!
//! ## DOES NOT OWN
//!
//! This file does not own:
//!
//! - Value or type ID allocation.
//! - Global uniqueness or persistence of IDs.
//! - A competing value or type representation.
//! - Operation-result storage or block-argument storage.
//! - Semantic type definitions, inference, or compatibility.
//! - SSA construction, dominance, or whole-program verification.
//! - Def-use index construction or mutation tracking.
//! - Source syntax, parsing, AST construction, or name resolution.
//! - Constant evaluation, arithmetic, or optimization.
//! - Physical registers, memory addresses, device placement, or execution.
//!
//! ## CANONICAL VALUE MODEL
//!
//! A value identity is a `ValueId`, not a separate ZCIR-owned object.
//!
//! A value definition is owned by its defining IR construct:
//!
//! - Function parameters are represented by the canonical function model.
//! - Block parameters are represented by `BlockArgument`.
//! - Operation-produced values are represented by `OperationValueResult`.
//!
//! `ValueDefinition` is a derived description of a definition in the
//! canonical definition-use analysis. It does not replace the owning
//! construct or become another serialized source of truth.
//!
//! `ValueUse` and `UseSite` describe uses, while `UseDefIndex` is a derived
//! analysis snapshot. The index must be rebuilt or invalidated whenever a
//! relevant IR mutation makes that snapshot stale.
//!
//! ## TYPE AND IDENTITY CONTRACT
//!
//! `ZcirValue` is an alias of `ValueId`. It cannot be used to introduce a
//! different ID representation or independent allocation sequence.
//!
//! `ZcirValueDefinition` is an alias of the canonical `ValueDefinition`.
//! Definition kind, owner, position, and type are determined by the
//! canonical definition-use model.
//!
//! Type compatibility is not established by possessing a `TypeId`.
//! The applicable semantic and IR verifiers must resolve types and verify
//! their use at the relevant operation, block, function, and region.
//!
//! ## VALIDATION AND TRUST BOUNDARY
//!
//! Constructing or importing a value ID does not establish that the value
//! has a definition, that its definition is unique, or that it is visible
//! at a particular use site.
//!
//! The whole-IR verification pipeline must check, as applicable:
//!
//! - Definition uniqueness within the verified identity scope.
//! - Resolution of every referenced value.
//! - Operation-result and block-argument ownership.
//! - Type existence and compatibility.
//! - SSA and dominance rules required by the adopted IR contract.
//! - Region, function, and control-flow visibility.
//! - Use-def index consistency after transformations.
//!
//! Deserialization and successful local construction are not substitutes
//! for whole-IR verification. Untrusted IR must be verified before it is
//! optimized, lowered, or executed.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This facade introduces no universal limit on values, definitions, uses,
//! blocks, operations, functions, processors, accelerators, or devices.
//!
//! The canonical identity representation and the underlying collections
//! still have finite representational and physical limits. The compiler
//! must report relevant exhaustion or verification failures instead of
//! silently reusing identities, truncating collections, or changing
//! program meaning.
//!
//! No finite implementation promises literal infinity. POCO-REAF requires
//! the absence of unjustified universal capacity ceilings and the
//! preservation of specified semantics when a valid target realization
//! is available.
//!
//! Operational budgets belong to the appropriate compiler, verifier,
//! decoder, or runtime configuration. They must not silently redefine
//! the language's semantic value domain.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `src/zuir/ids.rs` remains the sole owner of `ValueId` and `TypeId`.
//! - `src/zuir/ir/result.rs` remains the sole owner of
//!   `OperationValueResult`.
//! - `src/zuir/ir/block.rs` remains the sole owner of `BlockArgument`.
//! - `src/zuir/ir/use_def.rs` remains the sole owner of definition-use
//!   descriptions, index storage, and index-specific errors.
//! - `src/zuir/ir/operation.rs` remains the sole owner of operation results.
//! - The canonical function model remains the owner of function parameters.
//! - ZCIR builders must construct canonical entities and preserve their IDs.
//! - ZCIR verifiers must use canonical definitions and references.
//! - ZCIR transformations must invalidate or rebuild affected derived
//!   analyses, including definition-use indexes.
//! - ZCIR lowering must preserve value identity relationships and types
//!   until the target-lowering contract explicitly maps them.
//! - `src/quantum/ir/classical/` must retain its existing ownership and
//!   interoperability contracts; this file must not define quantum IDs.
//!
//! ## COMPATIBILITY
//!
//! The aliases are source-level conveniences. They do not create new
//! serialized types or guarantee that every ZUIR API change is compatible.
//! Persistent format compatibility remains governed by the canonical ZUIR
//! serialization and versioning contracts.
//!
//! Do not add fields to a second ZCIR value struct, because no such struct
//! is defined here. If a future requirement needs new canonical value
//! semantics, it must first be designed at the authoritative layer and
//! then exposed through this facade.
//!
//! ## FREEZE CRITERIA
//!
//! This file may be frozen when:
//!
//! - All imported canonical modules exist and compile.
//! - The ZCIR parent module declares and exports this module.
//! - The aliases resolve to the exact canonical types.
//! - Downstream builders, verifiers, and lowering use the same identities.
//! - The focused tests and repository integration gates pass.
//! - No duplicate identity, definition, or analysis model is introduced.

/// Canonical value identity used by ZUIR and ZCIR.
///
/// This alias intentionally adds no new identity semantics or allocator.
pub use crate::zuir::ids::ValueId as ZcirValue;

/// Canonical semantic type identity associated with value definitions.
pub use crate::zuir::ids::TypeId as ZcirValueType;

/// Canonical typed block-argument definition.
pub use crate::zuir::ir::block::BlockArgument as ZcirBlockArgument;

/// Canonical typed operation-result definition.
pub use crate::zuir::ir::result::OperationValueResult as ZcirOperationValueResult;

/// Canonical value-definition description and definition kind.
pub use crate::zuir::ir::use_def::{
    DefinitionKind as ZcirDefinitionKind,
    ValueDefinition as ZcirValueDefinition,
};

/// Canonical value-use description and use-site information.
pub use crate::zuir::ir::use_def::{
    UseSite as ZcirUseSite,
    ValueUse as ZcirValueUse,
};

/// Canonical derived definition-use index.
///
/// This is an analysis snapshot, not authoritative serialized IR state.
pub use crate::zuir::ir::use_def::UseDefIndex as ZcirUseDefIndex;

/// Canonical definition-use analysis error type.
pub use crate::zuir::ir::use_def::UseDefError as ZcirUseDefError;

/// Canonical definition-use analysis result type.
pub type ZcirUseDefResult<T> =
    crate::zuir::ir::use_def::UseDefResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{TypeId, ValueId};
    use crate::zuir::ir::block::BlockArgument;
    use crate::zuir::ir::result::OperationValueResult;
    use crate::zuir::ir::use_def::{
        DefinitionKind, UseDefIndex, ValueDefinition, ValueUse,
    };

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    #[test]
    fn zcir_value_is_the_canonical_value_id() {
        fn accepts_zcir(_: ZcirValue) {}
        fn accepts_zuir(_: ValueId) {}

        let id = value_id(1);

        accepts_zcir(id);
        accepts_zuir(id);

        assert_eq!(
            core::any::type_name::<ZcirValue>(),
            core::any::type_name::<ValueId>(),
        );
    }

    #[test]
    fn zcir_type_identity_is_the_canonical_type_id() {
        fn accepts_zcir(_: ZcirValueType) {}
        fn accepts_zuir(_: TypeId) {}

        let ty = type_id(2);

        accepts_zcir(ty);
        accepts_zuir(ty);

        assert_eq!(
            core::any::type_name::<ZcirValueType>(),
            core::any::type_name::<TypeId>(),
        );
    }

    #[test]
    fn block_arguments_use_canonical_value_and_type_identities() {
        let value = value_id(3);
        let ty = type_id(4);
        let argument = BlockArgument::new(value, ty);

        let zcir_argument: ZcirBlockArgument = argument.clone();

        assert_eq!(zcir_argument.value(), value);
        assert_eq!(zcir_argument.ty(), ty);
        assert_eq!(
            core::any::type_name::<ZcirBlockArgument>(),
            core::any::type_name::<BlockArgument>(),
        );
    }

    #[test]
    fn operation_results_use_canonical_value_and_type_identities() {
        let value = value_id(5);
        let ty = type_id(6);
        let result = OperationValueResult::new(value, ty);

        let zcir_result: ZcirOperationValueResult = result;

        assert_eq!(zcir_result.value(), value);
        assert_eq!(zcir_result.ty(), ty);
        assert_eq!(
            core::any::type_name::<ZcirOperationValueResult>(),
            core::any::type_name::<OperationValueResult>(),
        );
    }

    #[test]
    fn definition_and_use_analysis_types_are_canonical_aliases() {
        fn accepts_definition(_: ZcirValueDefinition) {}
        fn accepts_canonical_definition(_: ValueDefinition) {}
        fn accepts_kind(_: ZcirDefinitionKind) {}
        fn accepts_canonical_kind(_: DefinitionKind) {}
        fn accepts_use(_: ZcirValueUse) {}
        fn accepts_canonical_use(_: ValueUse) {}
        fn accepts_index(_: ZcirUseDefIndex) {}
        fn accepts_canonical_index(_: UseDefIndex) {}

        assert_eq!(
            core::any::type_name::<ZcirValueDefinition>(),
            core::any::type_name::<ValueDefinition>(),
        );
        assert_eq!(
            core::any::type_name::<ZcirValueUse>(),
            core::any::type_name::<ValueUse>(),
        );
        assert_eq!(
            core::any::type_name::<ZcirUseDefIndex>(),
            core::any::type_name::<UseDefIndex>(),
        );

        // Compile-time identity checks: each ZCIR-facing alias is accepted
        // by the API that consumes its canonical ZUIR counterpart.
        let _ = (
            accepts_definition,
            accepts_canonical_definition,
            accepts_kind,
            accepts_canonical_kind,
            accepts_use,
            accepts_canonical_use,
            accepts_index,
            accepts_canonical_index,
        );
    }
}
