
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Canonical Value Facade
//!
//! Canonical path: `src/classical/zcir/values/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the public value-module boundary for Zamani Classical Intermediate
//! Representation (ZCIR).
//!
//! This module organizes classical-facing adapters for canonical ZUIR values.
//! It does not create a competing constant model, numeric representation,
//! symbolic expression system, or validation framework.
//!
//! ## Authority
//!
//! `crate::zuir::values::constants` is the authoritative owner of:
//!
//! - `Constant` and `ConstantValue`;
//! - `ArbitraryInteger` and `FloatBits`;
//! - `ExactDecimal` and `ExtensionConstant`;
//! - `ConstantValidationBudget`;
//! - `ConstantValidationError`;
//! - canonical constant validation and serialization behavior.
//!
//! ZUIR's expression subsystem owns structured expression representation.
//! The frontend and grammar own source syntax and literal tokenization.
//! Semantic analysis owns name resolution, types, effects, and meaning.
//! Evaluators own computation and constant evaluation.
//! Verifiers own cross-reference, type, and IR invariant checks.
//! Lowering owns target-specific conversion and feasibility decisions.
//!
//! ## Module responsibilities
//!
//! - `integer`: ZCIR-facing arbitrary-width integer helpers.
//! - `float`: ZCIR-facing floating-point bit-pattern helpers.
//! - `aggregate`: ordered sequence, record, and variant helpers.
//! - `symbolic`: opaque symbolic-constant construction and validation.
//!
//! These modules adapt the canonical ZUIR representation. They must not
//! define duplicate value types or silently change represented values.
//!
//! ## Deliberately absent module
//!
//! This module does not declare `constant` because the repository snapshot
//! inspected for this integration did not contain
//! `src/classical/zcir/values/constant.rs`.
//!
//! The canonical `Constant` type already exists in ZUIR. A future
//! `values/constant.rs` may be added only if it has a distinct, justified
//! adapter responsibility and does not duplicate canonical ownership.
//!
//! ## Public API policy
//!
//! Canonical value types are re-exported at this boundary for convenient,
//! consistent imports. Domain-specific helper functions remain under their
//! owning submodule paths, such as `values::integer::...` and
//! `values::symbolic::...`.
//!
//! This prevents name collisions between functions such as
//! `integer::into_constant_value` and `float::into_constant_value`.
//! Callers can always identify which adapter they are using.
//!
//! ## Dependency direction
//!
//! ```text
//! grammar / frontend AST
//!          |
//!          v
//! semantic analysis
//!          |
//!          v
//! canonical ZUIR value model
//!          |
//!          v
//! ZCIR values facade
//!    |     |      |      |
//! integer float aggregate symbolic
//!          |
//!          v
//! ZCIR IR / verification / lowering
//! ```
//!
//! The facade must not depend on builders, evaluators, backends, device
//! allocation, scheduling, or physical execution.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no universal limit on integer width, floating-point
//! encoding length, aggregate size, symbolic payload length, or value count.
//! Its adapters use the canonical representation's dynamically sized data
//! structures and explicit validation policies.
//!
//! An unrestricted validation policy does not imply infinite memory,
//! execution time, address space, or physical resources. Callers processing
//! untrusted input should apply operational budgets at the input, decoding,
//! construction, and validation boundaries as appropriate.
//!
//! ## Error contract
//!
//! Canonical value validation errors are re-exported without wrapping or
//! translation. This preserves error identity and allows callers to
//! distinguish malformed values from configured budget exhaustion.
//!
//! ## Integration contract
//!
//! 1. `src/classical/zcir/mod.rs` must declare the `values` module when the
//!    ZCIR root is converted into a valid Rust module.
//! 2. The ZCIR root itself must be registered from `src/classical/mod.rs`
//!    only after its required module files are valid.
//! 3. Each declared child module must exist in the repository.
//! 4. Consumers should use the canonical types re-exported here or the
//!    owning adapter module's API.
//! 5. Type verification must establish compatibility with declared semantic
//!    types; this facade performs no type inference.
//! 6. Serialization and decoding must validate external values before
//!    treating them as trusted.
//! 7. Lowering must preserve value meaning or return an explicit error.
//!
//! ## Freeze criteria
//!
//! - Only existing child modules are declared.
//! - Canonical ZUIR value types remain the sole source of truth.
//! - No ambiguous helper functions are re-exported at the facade root.
//! - The public module paths are documented and stable.
//! - No target-specific assumptions or unsafe code are introduced.
//! - Unit and repository integration tests pass before the module is frozen.

/// Aggregate constant construction, access, and validation helpers.
pub mod aggregate;

/// Floating-point bit-pattern construction, access, and validation helpers.
pub mod float;

/// Arbitrary-width integer construction, access, and validation helpers.
pub mod integer;

/// Symbolic constant construction, access, and validation helpers.
pub mod symbolic;

// =============================================================================
// Canonical value types
// =============================================================================

/// Canonical arbitrary-width integer representation.
pub use crate::zuir::values::constants::ArbitraryInteger;

/// Canonical typed constant representation.
pub use crate::zuir::values::constants::Constant;

/// Canonical constant-value representation.
pub use crate::zuir::values::constants::ConstantValue;

/// Canonical operational policy for constant validation.
pub use crate::zuir::values::constants::ConstantValidationBudget;

/// Canonical errors for constant representation validation.
pub use crate::zuir::values::constants::ConstantValidationError;

/// Canonical exact decimal representation.
pub use crate::zuir::values::constants::ExactDecimal;

/// Canonical opaque extension constant representation.
pub use crate::zuir::values::constants::ExtensionConstant;

/// Canonical floating-point format and bit-pattern representation.
pub use crate::zuir::values::constants::FloatBits;

#[cfg(test)]
mod tests {
    use super::{
        aggregate,
        float,
        integer,
        symbolic,
        ArbitraryInteger,
        Constant,
        ConstantValue,
        ConstantValidationBudget,
        ConstantValidationError,
        ExactDecimal,
        ExtensionConstant,
        FloatBits,
    };

    #[test]
    fn canonical_types_are_available_from_the_facade() {
        let integer: ArbitraryInteger =
            ArbitraryInteger::new("42").expect("canonical integer");

        assert_eq!(integer.as_str(), "42");

        let float: FloatBits =
            FloatBits::new("org.zamani.float.example", "1")
                .expect("valid local float encoding");

        assert_eq!(float.bits_hex, "1");

        let decimal = ExactDecimal::new(
            ArbitraryInteger::new("125").expect("coefficient"),
            ArbitraryInteger::new("-2").expect("exponent"),
        )
        .expect("valid exact decimal");

        assert_eq!(decimal.coefficient.as_str(), "125");

        let extension = ExtensionConstant::new(
            "org.example",
            "value",
            "1",
            "a0",
        )
        .expect("valid extension representation");

        assert_eq!(extension.namespace, "org.example");

        let _: Option<Constant> = None;
        let _: Option<ConstantValidationBudget> = None;
        let _: Option<ConstantValidationError> = None;
    }

    #[test]
    fn integer_adapter_uses_the_canonical_value_model() {
        let integer = integer::from_canonical_decimal("123")
            .expect("valid canonical integer");

        let value: ConstantValue = integer::into_constant_value(integer);

        assert!(matches!(
            value,
            ConstantValue::Integer(_)
        ));
    }

    #[test]
    fn float_adapter_uses_the_canonical_value_model() {
        let float = float::from_bit_pattern(
            "org.zamani.float.example",
            "1",
        )
        .expect("valid local float representation");

        let value: ConstantValue = float::into_constant_value(float);

        assert!(matches!(
            value,
            ConstantValue::Float(_)
        ));
    }

    #[test]
    fn aggregate_adapter_uses_the_canonical_value_model() {
        let value = aggregate::sequence(Vec::new())
            .expect("valid empty sequence");

        assert!(matches!(
            value,
            ConstantValue::Sequence(_)
        ));
    }

    #[test]
    fn symbolic_adapter_uses_the_canonical_value_model() {
        let value = symbolic::symbolic_value("x + y")
            .expect("valid symbolic payload");

        assert!(matches!(
            value,
            ConstantValue::Symbolic { .. }
        ));
    }

    #[test]
    fn facade_preserves_canonical_validation_errors() {
        let result = symbolic::symbolic_value("");

        assert_eq!(
            result,
            Err(ConstantValidationError::EmptySymbolicExpression)
        );
    }
}
