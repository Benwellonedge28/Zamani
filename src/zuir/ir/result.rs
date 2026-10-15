#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Operation Result
//!
//! Canonical path: `src/zuir/ir/result.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Represent one typed SSA value definition produced by a ZUIR operation.
//!
//! An operation result associates:
//! - a canonical `ValueId` identifying the produced value;
//! - a canonical `TypeId` identifying its declared semantic type.
//!
//! The result's ordinal position is determined by its position in the
//! owning operation's ordered result collection. This type deliberately
//! does not store a second, independently mutable result index.
//!
//! ## STATUS
//!
//! Proposed canonical implementation. Freeze only after the module
//! declaration, operation integration, serialization fixtures, and
//! repository tests have been checked.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/use_def.rs`, when available
//! - `src/zuir/verify/`, for whole-IR verification
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## OWNS
//!
//! - `OperationValueResult`.
//! - The association between a produced value identity and its declared
//!   semantic type identity.
//! - Read-only access to those identities.
//!
//! ## DOES_NOT_OWN
//!
//! - Global identity allocation or uniqueness across a program.
//! - Semantic type definitions, type inference, or type compatibility.
//! - Operation storage, result ordering, or result collection mutation.
//! - Whole-program SSA verification, dominance, or def-use indexing.
//! - Dialect-specific result rules or operation semantics.
//! - Source parsing, AST construction, or source-name resolution.
//! - Physical registers, memory addresses, qubit placement, signal
//!   allocation, or runtime execution.
//!
//! ## INPUTS AND OUTPUTS
//!
//! Inputs are the canonical, nonzero `ValueId` and `TypeId` types.
//! Construction records these identities without looking them up.
//!
//! Outputs expose the identities through immutable, typed accessors.
//! Whether either identity resolves to a valid definition is established
//! by the enclosing IR and its verifier.
//!
//! ## DEPENDENCIES
//!
//! This module depends only on `crate::zuir::ids` and Serde.
//! It must not depend on `operation.rs`, builders, dialect registries,
//! verifiers, or backend implementations.
//!
//! ## INTEGRATION
//!
//! - `operation.rs` owns an ordered `Vec<OperationValueResult>`.
//! - The operation defines each result's ordinal position.
//! - The whole-IR verifier checks program-wide definition uniqueness,
//!   type existence, operation ownership, and SSA constraints.
//! - The use-def analysis indexes result definitions and operand uses.
//! - Serialization validates decoded IR before trusted processing.
//!
//! ## SCALABILITY
//!
//! This type imposes no result-count or hardware-capacity limit.
//! Collection sizing belongs to the owning operation and remains subject
//! to available address space and system resources.
//!
//! No finite implementation promises literal infinity.
//!
//! ## COMPATIBILITY
//!
//! The serialized field names remain `value` and `ty`, preserving the
//! existing `OperationValueResult` representation. Public fields are
//! intentionally not exposed; use the accessors.
//!
//! Deserialization is not whole-IR verification. Decoded programs must
//! be checked by the authoritative verifier before trusted processing.

use serde::{Deserialize, Serialize};

use crate::zuir::ids::{TypeId, ValueId};

/// One ordered, typed value definition produced by a ZUIR operation.
///
/// The operation containing this result owns its position in the result
/// sequence. A `ValueId` must be unique across the relevant verified IR
/// identity scope; this type does not allocate IDs or establish that
/// global uniqueness.
///
/// `TypeId` identifies the declared semantic type. It does not describe
/// physical layout, ABI representation, register width, memory placement,
/// quantum hardware allocation, or HDL signal allocation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct OperationValueResult {
    value: ValueId,
    ty: TypeId,
}

impl OperationValueResult {
    /// Construct a result from canonical value and type identities.
    ///
    /// This is a structural constructor: it does not resolve either ID.
    /// The whole-IR verifier must establish that the value is defined
    /// exactly once and that the declared type exists and is legal.
    #[must_use]
    pub const fn new(value: ValueId, ty: TypeId) -> Self {
        Self { value, ty }
    }

    /// Return the identity of the value produced by this result.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the declared semantic type identity.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Return the value and type identities together.
    ///
    /// This is a convenience accessor; it does not resolve either ID.
    #[must_use]
    pub const fn parts(&self) -> (ValueId, TypeId) {
        (self.value, self.ty)
    }

    /// Consume the result and return its value and type identities.
    #[must_use]
    pub fn into_parts(self) -> (ValueId, TypeId) {
        (self.value, self.ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    #[test]
    fn constructs_and_exposes_typed_identities() {
        let result = OperationValueResult::new(value_id(1), type_id(2));

        assert_eq!(result.value(), value_id(1));
        assert_eq!(result.ty(), type_id(2));
        assert_eq!(result.parts(), (value_id(1), type_id(2)));
    }

    #[test]
    fn consumes_into_typed_identities() {
        let result = OperationValueResult::new(value_id(3), type_id(4));

        assert_eq!(
            result.into_parts(),
            (value_id(3), type_id(4)),
        );
    }

    #[test]
    fn equality_and_hashing_follow_both_identities() {
        use std::collections::HashSet;

        let first = OperationValueResult::new(value_id(5), type_id(6));
        let same = OperationValueResult::new(value_id(5), type_id(6));
        let different_type =
            OperationValueResult::new(value_id(5), type_id(7));
        let different_value =
            OperationValueResult::new(value_id(8), type_id(6));

        assert_eq!(first, same);
        assert_ne!(first, different_type);
        assert_ne!(first, different_value);

        let unique: HashSet<_> =
            [first, same, different_type, different_value]
                .into_iter()
                .collect();

        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn preserves_existing_serde_field_names_and_round_trips() {
        let result = OperationValueResult::new(value_id(9), type_id(10));

        let encoded =
            serde_json::to_string(&result).expect("serialization succeeds");

        let decoded: OperationValueResult =
            serde_json::from_str(&encoded).expect("deserialization succeeds");

        assert_eq!(decoded, result);

        let json: serde_json::Value =
            serde_json::from_str(&encoded).expect("valid JSON");

        assert_eq!(json["value"], serde_json::json!(9));
        assert_eq!(json["ty"], serde_json::json!(10));
    }

    #[test]
    fn deserialization_rejects_zero_ids() {
        let invalid_value =
            r#"{"value":0,"ty":1}"#;

        let invalid_type =
            r#"{"value":1,"ty":0}"#;

        assert!(
            serde_json::from_str::<OperationValueResult>(invalid_value)
                .is_err()
        );

        assert!(
            serde_json::from_str::<OperationValueResult>(invalid_type)
                .is_err()
        );
    }

    #[test]
    fn supports_large_nonzero_id_values_without_truncation() {
        let result =
            OperationValueResult::new(value_id(u128::MAX), type_id(u128::MAX));

        assert_eq!(result.value().get(), u128::MAX);
        assert_eq!(result.ty().get(), u128::MAX);
    }
}