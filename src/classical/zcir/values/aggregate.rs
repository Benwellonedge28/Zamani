
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Aggregate Values
//!
//! Path: `src/classical/zcir/values/aggregate.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the ZCIR-facing construction, inspection, and validation helpers
//! for aggregate constant values while reusing the canonical ZUIR
//! representation in `crate::zuir::values::constants`.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Convenience constructors for sequences, records, and variants.
//! - Borrowed and consuming accessors for those aggregate variants.
//! - Aggregate-specific entry points to canonical structural validation.
//!
//! This module does not own:
//!
//! - A competing aggregate enum or value storage representation.
//! - Semantic aggregate types, tuple types, record schemas, or variant types.
//! - Type compatibility, field resolution, or variant exhaustiveness.
//! - Expression evaluation, mutation, alias analysis, or memory allocation.
//! - Source literal parsing or source-to-value conversion.
//! - Serialization of complete IR programs.
//! - Physical layout, ABI, target lowering, or runtime storage.
//!
//! ## Canonical representation
//!
//! - Sequences use `ConstantValue::Sequence` and preserve element order.
//! - Records use `ConstantValue::Record` and `BTreeMap` for deterministic
//!   field ordering.
//! - Variants use `ConstantValue::Variant` and preserve payload-field order.
//!
//! These are constant-value representations. They do not, by themselves,
//! prove that a sequence has the expected element types, that a record
//! conforms to a declared schema, or that a variant is declared by its type.
//!
//! The semantic type verifier owns those checks.
//!
//! ## Validation and budgets
//!
//! All structural validation delegates to the canonical ZUIR validator.
//! Its explicit work stack avoids recursively validating nested values.
//!
//! A validation budget can limit the number of visited value nodes and the
//! total UTF-8 text bytes examined. Budget exhaustion is an explicit error,
//! not evidence that the value is semantically invalid.
//!
//! Constructors receive already-created values. Their validation budget
//! limits validation work; it cannot retroactively limit memory already used
//! to construct the supplied values.
//!
//! ## POCO-REAF and scalability
//!
//! This module imposes no universal maximum aggregate length, record-field
//! count, nesting depth, or number of variants. Collections use the canonical
//! dynamically sized representations.
//!
//! Practical capacity remains bounded by available resources, allocation
//! behavior, operational policies, and target capabilities. No finite
//! implementation can guarantee literal infinity.
//!
//! ## Integration contract
//!
//! - `values/mod.rs` declares `pub mod aggregate;` and selectively re-exports
//!   this module's stable public API.
//! - `values/constant.rs` reuses canonical `Constant` and `ConstantValue`.
//! - `values/integer.rs` and `values/float.rs` continue to adapt scalar
//!   representations without introducing alternate aggregate storage.
//! - `types/aggregate.rs` owns semantic aggregate type definitions and
//!   declared schemas.
//! - The type verifier checks element types, field names/types, and variant
//!   membership against declared semantic types.
//! - The frontend parses aggregate syntax and builds AST nodes; this module
//!   does not parse source text.
//! - ZUIR lowering preserves sequence order, record names/order semantics,
//!   variant names, and payload order.
//! - Serialization and deserialization use the canonical ZUIR representation
//!   and validate untrusted values before treating them as trusted.
//! - Evaluators own aggregate operations, mutation, indexing, and runtime
//!   behavior.
//!
//! ## Freeze criteria
//!
//! - No duplicate aggregate representation is introduced.
//! - Record iteration and serialization remain deterministic.
//! - Malformed nested values are rejected by canonical validation.
//! - Explicit budgets are forwarded unchanged.
//! - Accessors preserve ownership and avoid unnecessary cloning.
//! - The module compiles and its tests pass with the repository's declared
//!   Rust toolchain and actual parent-module integration.

use std::collections::BTreeMap;

use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

// =============================================================================
// Canonical public API
// =============================================================================

/// Canonical constant-value representation exposed through ZCIR.
///
/// This is a re-export, not a second value enum.
pub use crate::zuir::values::constants::ConstantValue as ZcirAggregateValue;

/// Canonical structural-validation budget for aggregate values.
pub use crate::zuir::values::constants::ConstantValidationBudget
    as ZcirAggregateValidationBudget;

/// Canonical structural-validation errors.
pub use crate::zuir::values::constants::ConstantValidationError
    as ZcirAggregateValidationError;

// =============================================================================
// Construction
// =============================================================================

/// Construct and validate an ordered aggregate sequence.
///
/// Element order is preserved exactly. Elements may have different value
/// variants; semantic homogeneity, if required by the declared type, is the
/// responsibility of the semantic type verifier.
///
/// This constructor uses the canonical validator's default policy, which
/// imposes no explicit node-count or text-byte budget.
///
/// # Errors
///
/// Returns a canonical validation error if any element or nested value has
/// an invalid representation.
pub fn sequence(
    elements: Vec<ConstantValue>,
) -> Result<ConstantValue, ConstantValidationError> {
    sequence_with_budget(elements, &ConstantValidationBudget::unlimited())
}

/// Construct and validate an ordered sequence under an explicit budget.
///
/// The budget limits canonical validation work. It does not limit memory
/// already used to create `elements`.
pub fn sequence_with_budget(
    elements: Vec<ConstantValue>,
    budget: &ConstantValidationBudget,
) -> Result<ConstantValue, ConstantValidationError> {
    let value = ConstantValue::Sequence(elements);
    value.validate_with(budget)?;
    Ok(value)
}

/// Construct and validate a record with deterministic field ordering.
///
/// The `BTreeMap` representation ensures deterministic field iteration.
/// Duplicate field names cannot coexist in the supplied map; callers building
/// a map from source fields should detect duplicate declarations before
/// inserting them if duplicates are invalid in the source language.
///
/// This constructor validates the representation, not conformance to a
/// semantic record type.
pub fn record(
    fields: BTreeMap<String, ConstantValue>,
) -> Result<ConstantValue, ConstantValidationError> {
    record_with_budget(fields, &ConstantValidationBudget::unlimited())
}

/// Construct and validate a record under an explicit operational budget.
pub fn record_with_budget(
    fields: BTreeMap<String, ConstantValue>,
    budget: &ConstantValidationBudget,
) -> Result<ConstantValue, ConstantValidationError> {
    let value = ConstantValue::Record(fields);
    value.validate_with(budget)?;
    Ok(value)
}

/// Construct and validate a named variant and its ordered payload fields.
///
/// `name` must satisfy the canonical nonempty variant-name rule. Whether the
/// variant is declared by a particular semantic type is checked elsewhere.
pub fn variant(
    name: impl Into<String>,
    fields: Vec<ConstantValue>,
) -> Result<ConstantValue, ConstantValidationError> {
    variant_with_budget(
        name,
        fields,
        &ConstantValidationBudget::unlimited(),
    )
}

/// Construct and validate a named variant under an explicit budget.
pub fn variant_with_budget(
    name: impl Into<String>,
    fields: Vec<ConstantValue>,
    budget: &ConstantValidationBudget,
) -> Result<ConstantValue, ConstantValidationError> {
    let value = ConstantValue::Variant {
        name: name.into(),
        fields,
    };
    value.validate_with(budget)?;
    Ok(value)
}

// =============================================================================
// Borrowed accessors
// =============================================================================

/// Borrow the elements of a sequence.
///
/// Returns `None` when `value` is not a sequence. This function does not
/// clone the sequence or validate the elements.
#[must_use]
pub fn as_sequence(value: &ConstantValue) -> Option<&[ConstantValue]> {
    match value {
        ConstantValue::Sequence(elements) => Some(elements.as_slice()),
        _ => None,
    }
}

/// Borrow the ordered fields of a record.
///
/// Returns `None` when `value` is not a record. The returned map retains its
/// canonical ordered-key representation.
#[must_use]
pub fn as_record(
    value: &ConstantValue,
) -> Option<&BTreeMap<String, ConstantValue>> {
    match value {
        ConstantValue::Record(fields) => Some(fields),
        _ => None,
    }
}

/// Borrow a variant name and its ordered payload fields.
///
/// Returns `None` when `value` is not a variant. The variant name is not
/// resolved against a semantic type here.
#[must_use]
pub fn as_variant(
    value: &ConstantValue,
) -> Option<(&str, &[ConstantValue])> {
    match value {
        ConstantValue::Variant { name, fields } => {
            Some((name.as_str(), fields.as_slice()))
        }
        _ => None,
    }
}

// =============================================================================
// Consuming accessors
// =============================================================================

/// Consume a value and extract a sequence without cloning.
///
/// Returns `None` when `value` is not a sequence.
#[must_use]
pub fn into_sequence(value: ConstantValue) -> Option<Vec<ConstantValue>> {
    match value {
        ConstantValue::Sequence(elements) => Some(elements),
        _ => None,
    }
}

/// Consume a value and extract a record without cloning.
///
/// Returns `None` when `value` is not a record.
#[must_use]
pub fn into_record(
    value: ConstantValue,
) -> Option<BTreeMap<String, ConstantValue>> {
    match value {
        ConstantValue::Record(fields) => Some(fields),
        _ => None,
    }
}

/// Consume a value and extract a variant's name and payload without cloning.
///
/// Returns `None` when `value` is not a variant.
#[must_use]
pub fn into_variant(
    value: ConstantValue,
) -> Option<(String, Vec<ConstantValue>)> {
    match value {
        ConstantValue::Variant { name, fields } => Some((name, fields)),
        _ => None,
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validate a sequence or any other canonical constant value.
///
/// Despite the name, this function accepts all `ConstantValue` variants and
/// delegates their validation to the canonical ZUIR implementation.
/// Callers needing to require a sequence must first use [`as_sequence`] or
/// construct one through [`sequence`].
pub fn validate_aggregate(
    value: &ConstantValue,
) -> Result<(), ConstantValidationError> {
    value.validate()
}

/// Validate a canonical constant value under an explicit operational budget.
///
/// This function forwards the supplied budget unchanged to the canonical
/// validator. It does not silently replace or relax the caller's policy.
pub fn validate_aggregate_with_budget(
    value: &ConstantValue,
    budget: &ConstantValidationBudget,
) -> Result<(), ConstantValidationError> {
    value.validate_with(budget)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn integer(value: &str) -> ConstantValue {
        ConstantValue::Integer(
            crate::zuir::values::constants::ArbitraryInteger::new(value)
                .expect("valid canonical integer"),
        )
    }

    #[test]
    fn constructs_empty_sequence() {
        let value = sequence(Vec::new()).expect("empty sequence is valid");

        assert_eq!(as_sequence(&value), Some([].as_slice()));
    }

    #[test]
    fn preserves_sequence_element_order() {
        let elements = vec![
            ConstantValue::Boolean(true),
            integer("2"),
            ConstantValue::String("three".to_owned()),
        ];

        let value = sequence(elements.clone()).expect("valid sequence");

        assert_eq!(as_sequence(&value), Some(elements.as_slice()));
    }

    #[test]
    fn supports_nested_aggregates() {
        let inner = sequence(vec![
            ConstantValue::Boolean(false),
            integer("123456789012345678901234567890"),
        ])
        .expect("valid inner sequence");

        let outer = sequence(vec![
            ConstantValue::String("nested".to_owned()),
            inner,
        ])
        .expect("valid outer sequence");

        assert!(validate_aggregate(&outer).is_ok());
    }

    #[test]
    fn records_use_deterministic_key_order() {
        let mut fields = BTreeMap::new();
        fields.insert("zeta".to_owned(), integer("2"));
        fields.insert("alpha".to_owned(), integer("1"));

        let value = record(fields).expect("valid record");
        let fields = as_record(&value).expect("record variant");

        let keys: Vec<&str> = fields.keys().map(String::as_str).collect();
        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn rejects_empty_record_field_names() {
        let mut fields = BTreeMap::new();
        fields.insert("   ".to_owned(), ConstantValue::Boolean(true));

        assert_eq!(
            record(fields),
            Err(ConstantValidationError::EmptyRecordFieldName)
        );
    }

    #[test]
    fn rejects_invalid_nested_values() {
        // Deserialization or direct public-field construction can bypass
        // checked scalar constructors. The aggregate constructor must still
        // validate the complete nested representation.
        let invalid_float = crate::zuir::values::constants::FloatBits {
            format: "core::binary32".to_owned(),
            bits_hex: "NOT_HEX".to_owned(),
        };

        let nested = ConstantValue::Sequence(vec![
            ConstantValue::Float(invalid_float),
        ]);

        assert_eq!(
            validate_aggregate(&nested),
            Err(ConstantValidationError::InvalidFloatBits)
        );
    }

    #[test]
    fn constructs_and_inspects_variant() {
        let fields = vec![
            ConstantValue::Boolean(true),
            ConstantValue::String("payload".to_owned()),
        ];

        let value =
            variant("SomeValue", fields.clone()).expect("valid variant");

        let (name, borrowed_fields) =
            as_variant(&value).expect("variant value");

        assert_eq!(name, "SomeValue");
        assert_eq!(borrowed_fields, fields.as_slice());
    }

    #[test]
    fn rejects_empty_variant_name() {
        assert_eq!(
            variant("", Vec::new()),
            Err(ConstantValidationError::InvalidVariantName)
        );
    }

    #[test]
    fn consuming_accessors_preserve_values() {
        let expected = vec![integer("1"), integer("2")];
        let value = sequence(expected.clone()).expect("valid sequence");

        assert_eq!(into_sequence(value), Some(expected));
    }

    #[test]
    fn borrowed_accessors_return_none_for_other_variants() {
        let value = ConstantValue::Boolean(true);

        assert!(as_sequence(&value).is_none());
        assert!(as_record(&value).is_none());
        assert!(as_variant(&value).is_none());
    }

    #[test]
    fn forwards_node_budget_exhaustion() {
        let value = sequence(vec![ConstantValue::Boolean(true)])
            .expect("valid sequence");

        let budget = ConstantValidationBudget::new(Some(1), None);

        assert_eq!(
            validate_aggregate_with_budget(&value, &budget),
            Err(ConstantValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn forwards_text_budget_exhaustion() {
        let value = sequence(vec![
            ConstantValue::String("a longer value".to_owned()),
        ])
        .expect("valid sequence");

        let budget = ConstantValidationBudget::new(None, Some(2));

        assert_eq!(
            validate_aggregate_with_budget(&value, &budget),
            Err(ConstantValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn accepts_sufficient_validation_budget() {
        let value = sequence(vec![
            ConstantValue::Boolean(true),
            ConstantValue::String("ok".to_owned()),
        ])
        .expect("valid sequence");

        // Root sequence plus two child values.
        let budget = ConstantValidationBudget::new(Some(3), Some(2));

        assert!(validate_aggregate_with_budget(&value, &budget).is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_aggregate() {
        let mut fields = BTreeMap::new();
        fields.insert("first".to_owned(), integer("1"));
        fields.insert(
            "second".to_owned(),
            ConstantValue::Sequence(vec![
                ConstantValue::Boolean(false),
                ConstantValue::String("value".to_owned()),
            ]),
        );

        let original = record(fields).expect("valid record");

        let encoded =
            serde_json::to_string(&original).expect("serialize aggregate");

        let decoded: ConstantValue =
            serde_json::from_str(&encoded).expect("deserialize aggregate");

        assert_eq!(decoded, original);
        assert!(validate_aggregate(&decoded).is_ok());
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let value = sequence(vec![
            ConstantValue::Boolean(true),
            integer("42"),
        ])
        .expect("valid sequence");

        assert_eq!(
            validate_aggregate(&value),
            validate_aggregate(&value)
        );
    }
}
