
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_debug_implementations)]

//! # ZUIR Values — Canonical Value Subsystem
//!
//! PURPOSE
//!
//! Orchestrate the public value-related APIs of Zamani's Universal
//! Intermediate Representation (ZUIR). This module is the integration
//! boundary for portable constants, literals, symbols, expressions,
//! and operation-result definitions.
//!
//! STATUS
//!
//! Integration-ready module facade. Freeze after the repository-wide
//! compilation, API compatibility, and integration gates described below
//! have passed.
//!
//! AUTHORITY
//!
//! This module conforms to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/ir/` — canonical IR structure and value ownership.
//! - `grammar/expressions/literals.g4`
//! - `grammar/lexer/numeric-literals.g4`
//! - The applicable normative grammar and semantic specifications.
//!
//! The specifications define semantics. Child modules own their respective
//! data models. This file owns orchestration and deliberate public exports.
//!
//! OWNS
//!
//! - The canonical Rust module boundary for ZUIR values.
//! - Explicit exports of supported value APIs.
//! - The public integration surface between the value-related modules.
//! - Module-level API and contract regression tests.
//!
//! DOES NOT OWN
//!
//! - A second constant, literal, symbol, expression, or result data model.
//! - Global identity allocation, uniqueness, or identifier interning.
//! - Source parsing, lexing, AST construction, or name resolution.
//! - Semantic type definitions, type inference, or type compatibility.
//! - The canonical program, module, function, block, or operation model.
//! - Whole-program SSA, dominance, def-use, effect, or ownership verification.
//! - Constant evaluation, expression interpretation, or optimization.
//! - Target-specific data layouts, device allocation, or execution.
//! - Complete-program serialization, migration, or stable fingerprints.
//!
//! CHILD-MODULE OWNERSHIP
//!
//! - `constants`: canonical constant values and local constant validation.
//! - `literals`: the typed-literal facade over canonical constants.
//! - `symbols`: symbol identity references and declaration metadata.
//! - `expressions`: typed expression structures and local validation.
//! - `results`: operation-local result definitions and ordered collections.
//!
//! Each child remains responsible for its own invariants and error types.
//! This module must not reimplement those invariants or translate their
//! errors into a competing error hierarchy.
//!
//! INTEGRATION CONTRACT
//!
//! Frontend:
//!   Source text -> lexer/parser -> AST -> semantic analysis.
//!
//! Semantic analysis:
//!   Resolves names, establishes types, and creates canonical identities.
//!
//! Values:
//!   Represent semantic constants, literals, symbols, expressions,
//!   and operation results without selecting a physical target.
//!
//! Canonical IR:
//!   Owns program structure, operations, control flow, value definitions,
//!   and cross-operation relationships.
//!
//! Verification:
//!   Checks type compatibility, reference resolution, SSA rules when
//!   adopted by the specification, and complete-program invariants.
//!
//! Lowering:
//!   Converts verified semantics into appropriate domain and target IRs.
//!
//! This module must not bypass these boundaries or claim that local
//! representation validation proves whole-program semantic validity.
//!
//! PUBLIC API POLICY
//!
//! Child modules remain public for compatibility with existing paths.
//! Named re-exports provide a convenient canonical import surface.
//! Wildcard re-exports are prohibited because they expose implementation
//! details and can create ambiguous or accidental public APIs.
//!
//! A public API change requires a compatibility review. Additive exports
//! should be preferred when they do not create conflicting names.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module introduces no fixed limit on the number of values, symbols,
//! expressions, operation results, integer bits, decimal digits, aggregate
//! elements, tensor dimensions, qubits, devices, nodes, or hardware resources.
//!
//! It adds no hardware-specific assumptions and does not use host-sized
//! integers as a substitute for arbitrary-width semantic values.
//!
//! Child modules own dynamic representations and explicit validation
//! budgets. Resource exhaustion must be handled by the responsible operation
//! or subsystem; this facade must not silently impose a semantic ceiling.
//!
//! No finite implementation can guarantee literal infinity. Practical
//! capacity is bounded by available resources, representational limits,
//! and explicitly configured operational policies.
//!
//! DETERMINISM
//!
//! This module adds no global mutable state, hidden registration order,
//! or nondeterministic initialization. Deterministic behavior of values,
//! validation, serialization, and fingerprints is defined by their owners.
//!
//! SAFETY
//!
//! Safe Rust only. This module prohibits unsafe code.
//! Deserialized data must be validated by the relevant child modules and
//! canonical verifier before it is trusted.
//!
//! FREEZE CRITERIA
//!
//! - Every exported name exists and is owned by an authoritative module.
//! - Existing child-module paths and supported APIs remain compatible.
//! - No duplicate representation or circular dependency is introduced.
//! - The crate compiles with the declared Rust toolchain and dependencies.
//! - Module tests and relevant repository integration tests pass.
//! - Public exports are reviewed against actual downstream consumers.
//! - The canonical ZUIR specification and implementation agree.

/// Canonical portable constants and their structural validation.
///
/// Owns exact and arbitrary-width numeric representations, aggregate values,
/// extension values, and explicit validation budgets.
pub mod constants;

/// Typed expression structures and their local validation.
///
/// Expression semantics, name resolution, type compatibility, and evaluation
/// remain the responsibility of their authoritative subsystems.
pub mod expressions;

/// Typed literals backed by the canonical constant representation.
///
/// This module must not define a competing literal-value enum.
pub mod literals;

/// Operation-local result definitions and result collections.
///
/// The canonical IR remains responsible for integrating these results into
/// operations and verifying complete-program value relationships.
pub mod results;

/// Symbol references and declaration metadata.
///
/// Name resolution and global symbol-table ownership remain outside this
/// module.
pub mod symbols;

// =============================================================================
// Canonical public API
// =============================================================================

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

pub use constants::{
    ArbitraryInteger,
    Constant,
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
    ExactDecimal,
    ExtensionConstant,
    FloatBits,
};

// -----------------------------------------------------------------------------
// Literals
// -----------------------------------------------------------------------------

pub use literals::{
    Literal,
    LiteralValidationBudget,
    LiteralValidationError,
    LiteralValue,
};

// -----------------------------------------------------------------------------
// Symbols
// -----------------------------------------------------------------------------

pub use symbols::{
    Symbol,
    SymbolKind,
    SymbolLinkage,
    SymbolOrigin,
    SymbolRef,
    SymbolState,
    SymbolValidationBudget,
    SymbolValidationError,
    SymbolVisibility,
};

// -----------------------------------------------------------------------------
// Expressions
// -----------------------------------------------------------------------------

pub use expressions::{
    Expression,
    ExpressionKind,
    ExpressionValidationBudget,
    ExpressionValidationError,
};

// -----------------------------------------------------------------------------
// Operation results
// -----------------------------------------------------------------------------

pub use results::{
    OperationResults,
    ResultCollectionError,
    ResultDefinition,
    ResultIndex,
};

// =============================================================================
// Integration guidance for consumers
// =============================================================================

// Canonical value imports:
//
// use crate::zuir::values::{
//     Constant, ConstantValue, Expression, ExpressionKind, Literal,
//     OperationResults, Symbol, SymbolRef,
// };
//
// For specialized functionality, consumers may use the child-module paths:
//
// use crate::zuir::values::constants::ConstantValidationBudget;
// use crate::zuir::values::expressions::ExpressionValidationBudget;
// use crate::zuir::values::symbols::SymbolValidationBudget;
//
// The module facade deliberately does not:
// - allocate IDs;
// - create operations;
// - resolve symbols;
// - infer or rewrite types;
// - execute expressions;
// - perform whole-program verification;
// - serialize complete programs;
// - lower values to physical machine representations.
//
// If the canonical IR has a distinct authoritative result representation,
// reconcile `results` with that representation before freezing the API.
// Do not maintain two independent definitions of the same IR concept.

// =============================================================================
// Module contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test-only identifiers deliberately demonstrate that result collections
    // are generic. Production consumers must use the repository's canonical
    // identifiers rather than these test-only types.

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestValueId(u64);

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestOperationId(u64);

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestTypeId(u64);

    #[test]
    fn canonical_value_types_are_available_from_the_facade() {
        let integer = ArbitraryInteger::new(
            "12345678901234567890123456789012345678901234567890",
        )
        .expect("the arbitrary-width integer must be valid");

        let value = ConstantValue::Integer(integer);
        let budget = ConstantValidationBudget::unlimited();

        assert!(value.validate_with(&budget).is_ok());
    }

    #[test]
    fn literals_reuse_the_canonical_constant_model() {
        let value = ConstantValue::Boolean(true);

        // Literal construction requires a semantic TypeRef. This test checks
        // the exported API without inventing a type or bypassing type rules.
        let _value: LiteralValue = value;
    }

    #[test]
    fn operation_results_are_available_through_the_facade() {
        let owner = TestOperationId(1);
        let value = TestValueId(2);
        let ty = TestTypeId(3);

        let definition =
            ResultDefinition::new(value.clone(), owner.clone(), ty, 0);

        let mut results = OperationResults::new(owner);

        assert!(results.is_empty());
        assert!(results.insert(definition).is_ok());
        assert_eq!(results.len(), 1);
        assert!(results.validate().is_ok());
        assert!(results.get_by_value(&value).is_some());
    }

    #[test]
    fn result_collection_rejects_a_definition_owned_by_another_operation() {
        let mut results = OperationResults::new(TestOperationId(1));

        let definition = ResultDefinition::new(
            TestValueId(2),
            TestOperationId(9),
            TestTypeId(3),
            0,
        );

        assert_eq!(
            results.insert(definition),
            Err(ResultCollectionError::DefiningOperationMismatch)
        );

        assert!(results.is_empty());
    }

    #[test]
    fn result_indices_are_logical_positions_not_hardware_registers() {
        let owner = TestOperationId(1);

        let first = ResultDefinition::new(
            TestValueId(10),
            owner.clone(),
            TestTypeId(20),
            0,
        );

        let second = ResultDefinition::new(
            TestValueId(11),
            owner.clone(),
            TestTypeId(21),
            1,
        );

        let mut results = OperationResults::new(owner);

        assert!(results.insert(first).is_ok());
        assert!(results.insert(second).is_ok());

        assert_eq!(results.get_by_index(0).map(ResultDefinition::index), Some(0));
        assert_eq!(results.get_by_index(1).map(ResultDefinition::index), Some(1));
    }
}
