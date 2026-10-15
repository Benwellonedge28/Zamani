
#![forbid(unsafe_code)]

//! # ZUIR values
//!
//! PURPOSE:
//! Define the module boundary for values represented in Zamani's canonical
//! Universal Intermediate Representation (ZUIR).
//!
//! STATUS:
//! Proposed integration contract. Freeze after the child-module APIs and
//! repository-wide ownership rules have been audited and tested.
//!
//! AUTHORITY:
//! The normative ZUIR specification, canonical identifier contracts,
//! type-system contracts, and the value semantics defined by the IR.
//!
//! OWNS:
//! - The public module structure for ZUIR value-related functionality.
//! - Deliberate exports of value APIs that are approved for external use.
//! - The boundary between value definitions and their implementation modules.
//!
//! DOES_NOT_OWN:
//! - Global ID allocation or canonical ID definitions.
//! - Type definitions, type compatibility, or type inference.
//! - Source parsing, name resolution, or semantic analysis.
//! - Operation definitions, control flow, dominance, or whole-program
//!   use-def verification.
//! - Physical storage, device placement, register allocation, or execution.
//! - Serialization formats or target-specific lowering.
//!
//! MODULE CONTRACT:
//! - Child modules own their respective value representations.
//! - This module must not implement a competing value model.
//! - Re-exports must refer to types owned by their authoritative modules.
//! - Cross-module invariants are enforced by the authoritative verifier.
//!
//! SCALABILITY:
//! This module imposes no fixed count of values, symbols, expressions,
//! constants, or operation results. Actual limits are determined by available
//! resources and explicit policies in the relevant implementation layers.
//!
//! POCO-REAF:
//! Value representations remain independent of a particular processor,
//! quantum device, HDL target, accelerator, cluster, or runtime.
//!
//! COMPATIBILITY:
//! Public re-exports form part of the Rust API. Breaking changes require
//! deliberate compatibility review rather than accidental export changes.
//!
//! SAFETY:
//! No unsafe Rust is permitted in this module.

/// Literal representations, including exact or explicitly bounded values.
///
/// This module owns literal representation, not the language's complete
/// type system or target-specific numeric lowering.
pub mod literals;

/// Symbol declarations and references used by the value model.
///
/// Name resolution remains the responsibility of semantic analysis.
pub mod symbols;

/// Constant values and constant-evaluation results.
///
/// Evaluation policy, recursion protection, and compile-time resource budgets
/// belong to the constant-evaluation implementation and its configuration.
pub mod constants;

/// Value expressions and symbolic expressions.
///
/// Expressions represent computation or unresolved compile-time values;
/// they do not replace the source AST or the canonical operation model.
pub mod expressions;

/// Results produced by IR operations.
///
/// This module should own operation-local result definitions only if that
/// ownership agrees with the repository's canonical IR design. If
/// `crate::zuir::ir::result` is already authoritative, consolidate the two
/// models rather than keeping competing result types.
pub mod results;

// ---------------------------------------------------------------------------
// Stable public exports
// ---------------------------------------------------------------------------

// These exports match the proposed `values/results.rs` implementation from
// the preceding design step. If the actual results.rs defines a different
// public API, reconcile that API before enabling these exports.
pub use results::{
    OperationResults,
    ResultCollectionError,
    ResultDefinition,
    ResultIndex,
};

// Intentionally do not re-export every child-module item with a wildcard.
// Explicit exports keep the public API deliberate and reduce accidental
// coupling to implementation details.
//
// Add named exports from literals, symbols, constants, and expressions only
// after auditing their actual definitions and confirming that their types
// belong in the supported public API.

// ---------------------------------------------------------------------------
// Module-level contract tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::results::{
        OperationResults,
        ResultCollectionError,
        ResultDefinition,
    };

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestValueId(u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestOperationId(u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestTypeId(u64);

    #[test]
    fn result_api_is_available_through_the_values_module() {
        let operation = TestOperationId(1);
        let value = TestValueId(2);
        let ty = TestTypeId(3);

        let definition = ResultDefinition::new(
            value.clone(),
            operation.clone(),
            ty,
            0,
        );

        let mut results = OperationResults::new(operation);
        assert_eq!(results.len(), 0);

        assert!(results.insert(definition).is_ok());
        assert_eq!(results.len(), 1);
        assert!(results.validate().is_ok());
        assert!(results.get_by_value(&value).is_some());
    }

    #[test]
    fn invalid_result_definitions_remain_explicit_errors() {
        let mut results =
            OperationResults::new(TestOperationId(1));

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
}
