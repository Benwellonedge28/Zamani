
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR schema type vocabulary
//!
//! Path: `src/zuir/schema/types.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Defines the target-independent vocabulary used to describe schemas in
//! ZUIR. Schema types describe the structure and validation requirements of
//! schema values; they are not the compiler's semantic type system.
//!
//! ## OWNERSHIP
//!
//! OWNS:
//! - Qualified schema names.
//! - Primitive and compositional schema types.
//! - Symbolic and explicitly sized integer schema descriptors.
//! - Structural schema validation and deterministic traversal.
//! - Structured validation errors for this module.
//!
//! DOES_NOT_OWN:
//! - Semantic type identity, inference, or type checking.
//! - Physical data layout, ABI, registers, or device allocation.
//! - Attribute policy, metadata policy, or extension registration.
//! - Operation verification, parsing, or source diagnostics.
//! - Serialization of complete ZUIR programs.
//!
//! ## INTEGRATION
//!
//! `schema/attributes.rs` may use `SchemaType` to declare attribute values.
//! `schema/metadata.rs` may use it for validated metadata descriptors.
//! `schema/extensions.rs` may use `QualifiedName` for extension-owned names.
//! Dialect definitions may use it for schema-level operand and result
//! descriptors.
//!
//! Semantic type definitions belong in `crate::zuir::types`, not here.
//!
//! This module deliberately has no dependency on future sibling modules.
//! Its errors are local and can later be wrapped by the canonical ZUIR error
//! API without creating a circular dependency.
//!
//! ## SCALABILITY AND SAFETY
//!
//! - No fixed hardware capacities or maximum type widths are imposed.
//! - Collections are dynamically sized.
//! - Integer widths may be symbolic or explicitly specified.
//! - Zero-width integer descriptors are rejected.
//! - Validation uses an explicit work stack for recursive schema structures.
//! - Allocation failure follows Rust's ordinary allocation behavior; this
//!   module does not claim that finite hardware can provide infinite memory.
//! - Deserialization of untrusted input must additionally use the configured
//!   decoding/resource policy of the enclosing serialization layer.
//!
//! ## DETERMINISM
//!
//! Object fields use `BTreeMap`, providing deterministic key ordering.
//! Union member order is preserved because order may be meaningful to
//! diagnostics and serialized representations. Callers requiring canonical
//! unions must normalize them under a separately specified canonicalization
//! contract.
//!
//! ## FREEZE CRITERIA
//!
//! The schema vocabulary, validation rules, name grammar, and public error
//! behavior are stable. Changes to serialized representations must follow
//! the repository's schema-version and compatibility policy.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroU128;

/// A validated, qualified name used by schema references.
///
/// Each component is an ASCII identifier. Names are represented as segments
/// rather than a delimiter-encoded string so callers do not need to guess
/// escaping rules.
///
/// Examples:
/// - `core::Integer`
/// - `quantum::State`
/// - `vendor_example::ExtendedValue`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QualifiedName {
    segments: Vec<String>,
}

impl QualifiedName {
    /// Construct a qualified name from its individual components.
    pub fn new<I, S>(segments: I) -> Result<Self, SchemaTypeError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let segments: Vec<String> = segments.into_iter().map(Into::into).collect();

        if segments.is_empty() {
            return Err(SchemaTypeError::EmptyQualifiedName);
        }

        for segment in &segments {
            validate_identifier(segment)?;
        }

        Ok(Self { segments })
    }

    /// Parse a qualified name separated by `::`.
    ///
    /// Empty components, leading/trailing separators, and invalid identifier
    /// characters are rejected rather than silently normalized.
    pub fn parse(value: &str) -> Result<Self, SchemaTypeError> {
        if value.is_empty() {
            return Err(SchemaTypeError::EmptyQualifiedName);
        }

        Self::new(value.split("::"))
    }

    /// Return the qualified-name components.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Return the final component.
    #[must_use]
    pub fn local_name(&self) -> &str {
        // Construction and deserialization must preserve the nonempty
        // segment invariant. The fallback is defensive against future
        // internal changes; it does not create a valid empty name.
        self.segments.last().map(String::as_str).unwrap_or("")
    }

    /// Return the number of name components.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// Format this name using the canonical `::` separator.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        self.segments.join("::")
    }

    /// Validate the name invariant, including after deserialization.
    pub fn validate(&self) -> Result<(), SchemaTypeError> {
        if self.segments.is_empty() {
            return Err(SchemaTypeError::EmptyQualifiedName);
        }

        for segment in &self.segments {
            validate_identifier(segment)?;
        }

        Ok(())
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                f.write_str("::")?;
            }
            f.write_str(segment)?;
        }
        Ok(())
    }
}

/// Integer signedness for schema-level integer descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerSignedness {
    /// Non-negative integer values.
    Unsigned,

    /// Signed integer values.
    Signed,

    /// Signedness is defined by a referenced schema or later specialization.
    Unspecified,
}

/// Width information for an integer schema.
///
/// A symbolic width is a constraint/expression name, not a physical register
/// width. Its meaning must be resolved by the owning schema or semantic layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum IntegerWidth {
    /// An explicitly specified positive width.
    Fixed(NonZeroU128),

    /// A symbolic width resolved by the owning schema or semantic layer.
    Symbolic(String),
}

impl IntegerWidth {
    /// Construct an explicit width, rejecting zero.
    pub fn fixed(width: u128) -> Result<Self, SchemaTypeError> {
        NonZeroU128::new(width)
            .map(Self::Fixed)
            .ok_or(SchemaTypeError::ZeroIntegerWidth)
    }

    /// Construct a symbolic width using a validated identifier.
    pub fn symbolic(expression: impl Into<String>) -> Result<Self, SchemaTypeError> {
        let expression = expression.into();
        validate_identifier(&expression)?;
        Ok(Self::Symbolic(expression))
    }

    /// Validate the width descriptor, including after deserialization.
    pub fn validate(&self) -> Result<(), SchemaTypeError> {
        match self {
            Self::Fixed(width) if width.get() == 0 => {
                Err(SchemaTypeError::ZeroIntegerWidth)
            }
            Self::Fixed(_) => Ok(()),
            Self::Symbolic(expression) => validate_identifier(expression),
        }
    }
}

/// The schema-level type vocabulary.
///
/// This type describes the shape of values accepted by a schema. It must not
/// be used as a substitute for resolved semantic types, effects, ownership,
/// resource requirements, or target-specific representations.
///
/// Extension-specific computational types should generally be referenced by
/// qualified name and defined by the owning dialect or semantic type system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SchemaType {
    /// Any schema value accepted by the enclosing validation policy.
    Any,

    /// The absence of a value.
    Null,

    /// A Boolean value.
    Boolean,

    /// A mathematical integer descriptor.
    ///
    /// `width: None` means that the schema does not impose a width.
    /// It does not mean that all targets can physically represent every
    /// possible integer value.
    Integer {
        signedness: IntegerSignedness,
        width: Option<IntegerWidth>,
    },

    /// A numeric value whose precise representation is defined elsewhere.
    Number,

    /// A textual Unicode string.
    String,

    /// An uninterpreted byte sequence.
    Bytes,

    /// A sequence whose elements conform to `element`.
    Array {
        element: Box<SchemaType>,
    },

    /// A mapping with string keys and values conforming to `value`.
    Map {
        value: Box<SchemaType>,
    },

    /// An object with explicitly declared fields.
    ///
    /// Field order is deterministic. Whether additional fields are allowed is
    /// controlled by `allow_additional_fields`.
    Object {
        fields: BTreeMap<String, SchemaType>,
        allow_additional_fields: bool,
    },

    /// A value that may conform to any one of the listed alternatives.
    ///
    /// Empty unions are invalid. This vocabulary does not itself guarantee
    /// that alternatives are disjoint.
    Union {
        variants: Vec<SchemaType>,
    },

    /// A value that may be absent or conform to the wrapped schema.
    Optional {
        value: Box<SchemaType>,
    },

    /// A reference to a named schema or dialect-defined type descriptor.
    ///
    /// Resolution is performed by the registry/semantic layer, not by this
    /// module. Unknown references must not silently gain executable meaning.
    Reference {
        name: QualifiedName,
    },
}

impl SchemaType {
    /// Construct an unconstrained integer schema.
    #[must_use]
    pub const fn integer() -> Self {
        Self::Integer {
            signedness: IntegerSignedness::Unspecified,
            width: None,
        }
    }

    /// Construct a signed or unsigned integer schema without a fixed width.
    #[must_use]
    pub const fn integer_with_signedness(signedness: IntegerSignedness) -> Self {
        Self::Integer {
            signedness,
            width: None,
        }
    }

    /// Construct an integer schema with an explicit positive width.
    pub fn integer_with_width(
        signedness: IntegerSignedness,
        width: u128,
    ) -> Result<Self, SchemaTypeError> {
        Ok(Self::Integer {
            signedness,
            width: Some(IntegerWidth::fixed(width)?),
        })
    }

    /// Construct an integer schema with a symbolic width.
    pub fn integer_with_symbolic_width(
        signedness: IntegerSignedness,
        expression: impl Into<String>,
    ) -> Result<Self, SchemaTypeError> {
        Ok(Self::Integer {
            signedness,
            width: Some(IntegerWidth::symbolic(expression)?),
        })
    }

    /// Construct an array schema.
    #[must_use]
    pub fn array(element: SchemaType) -> Self {
        Self::Array {
            element: Box::new(element),
        }
    }

    /// Construct a map schema.
    #[must_use]
    pub fn map(value: SchemaType) -> Self {
        Self::Map {
            value: Box::new(value),
        }
    }

    /// Construct an optional schema.
    #[must_use]
    pub fn optional(value: SchemaType) -> Self {
        Self::Optional {
            value: Box::new(value),
        }
    }

    /// Construct a named schema reference.
    pub fn reference(name: QualifiedName) -> Result<Self, SchemaTypeError> {
        name.validate()?;
        Ok(Self::Reference { name })
    }

    /// Construct an object schema.
    ///
    /// Field names must be valid identifiers. The resulting map is ordered
    /// deterministically by field name.
    pub fn object<I, K>(
        fields: I,
        allow_additional_fields: bool,
    ) -> Result<Self, SchemaTypeError>
    where
        I: IntoIterator<Item = (K, SchemaType)>,
        K: Into<String>,
    {
        let mut result = BTreeMap::new();

        for (key, value) in fields {
            let key = key.into();
            validate_identifier(&key)?;

            if result.insert(key.clone(), value).is_some() {
                return Err(SchemaTypeError::DuplicateObjectField(key));
            }
        }

        Ok(Self::Object {
            fields: result,
            allow_additional_fields,
        })
    }

    /// Construct a union schema, rejecting an empty variant list.
    pub fn union(variants: Vec<SchemaType>) -> Result<Self, SchemaTypeError> {
        if variants.is_empty() {
            return Err(SchemaTypeError::EmptyUnion);
        }

        Ok(Self::Union { variants })
    }

    /// Validate all locally enforceable schema invariants.
    ///
    /// This method intentionally does not resolve references or evaluate
    /// symbolic widths. Those operations require a registry or semantic
    /// context owned by another module.
    ///
    /// An explicit work stack avoids recursive Rust calls for nested schema
    /// validation. The schema itself must still fit in available memory.
    pub fn validate(&self) -> Result<(), SchemaTypeError> {
        let mut pending = vec![self];

        while let Some(schema_type) = pending.pop() {
            match schema_type {
                Self::Any
                | Self::Null
                | Self::Boolean
                | Self::Number
                | Self::String
                | Self::Bytes => {}

                Self::Integer { width, .. } => {
                    if let Some(width) = width {
                        width.validate()?;
                    }
                }

                Self::Array { element } => pending.push(element),

                Self::Map { value } | Self::Optional { value } => {
                    pending.push(value);
                }

                Self::Object { fields, .. } => {
                    for (name, field_type) in fields {
                        validate_identifier(name)?;
                        pending.push(field_type);
                    }
                }

                Self::Union { variants } => {
                    if variants.is_empty() {
                        return Err(SchemaTypeError::EmptyUnion);
                    }
                    pending.extend(variants.iter());
                }

                Self::Reference { name } => {
                    name.validate()?;
                }
            }
        }

        Ok(())
    }

    /// Return the number of schema nodes reachable from this schema.
    ///
    /// This is a structural measurement, not a hardware limit. The result is
    /// checked so that overflow is reported rather than wrapped.
    pub fn node_count(&self) -> Result<usize, SchemaTypeError> {
        let mut pending = vec![self];
        let mut count = 0usize;

        while let Some(schema_type) = pending.pop() {
            count = count
                .checked_add(1)
                .ok_or(SchemaTypeError::CountOverflow)?;

            match schema_type {
                Self::Array { element } => pending.push(element),
                Self::Map { value } | Self::Optional { value } => pending.push(value),
                Self::Object { fields, .. } => {
                    pending.extend(fields.values());
                }
                Self::Union { variants } => pending.extend(variants.iter()),
                Self::Any
                | Self::Null
                | Self::Boolean
                | Self::Integer { .. }
                | Self::Number
                | Self::String
                | Self::Bytes
                | Self::Reference { .. } => {}
            }
        }

        Ok(count)
    }
}

/// Errors produced by schema vocabulary construction and validation.
///
/// These errors are deliberately independent of verifier and diagnostic
/// internals. Callers may translate them into canonical ZUIR diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SchemaTypeError {
    /// A qualified name has no components.
    EmptyQualifiedName,

    /// A name component or symbolic expression is not a valid identifier.
    InvalidIdentifier(String),

    /// An integer descriptor specifies a width of zero.
    ZeroIntegerWidth,

    /// An object contains a repeated field name.
    DuplicateObjectField(String),

    /// A union has no alternatives.
    EmptyUnion,

    /// A structural count exceeded the host's representable `usize` range.
    CountOverflow,
}

impl fmt::Display for SchemaTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQualifiedName => {
                f.write_str("qualified schema name must contain at least one component")
            }
            Self::InvalidIdentifier(value) => {
                write!(f, "invalid schema identifier: {value:?}")
            }
            Self::ZeroIntegerWidth => {
                f.write_str("integer schema width must be greater than zero")
            }
            Self::DuplicateObjectField(name) => {
                write!(f, "duplicate object schema field: {name}")
            }
            Self::EmptyUnion => {
                f.write_str("union schema must contain at least one alternative")
            }
            Self::CountOverflow => {
                f.write_str("schema node count exceeds the representable usize range")
            }
        }
    }
}

impl std::error::Error for SchemaTypeError {}

/// Validate an ASCII identifier.
///
/// Names intentionally use a conservative, portable grammar. Source-language
/// Unicode identifiers and dialect-specific naming rules belong to their
/// respective layers; they must not be silently normalized here.
fn validate_identifier(value: &str) -> Result<(), SchemaTypeError> {
    let mut chars = value.chars();

    let valid_first = chars
        .next()
        .map(|ch| ch == '_' || ch.is_ascii_alphabetic())
        .unwrap_or(false);

    let valid_rest = chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric());

    if valid_first && valid_rest {
        Ok(())
    } else {
        Err(SchemaTypeError::InvalidIdentifier(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualified_names_have_a_canonical_representation() {
        let name = QualifiedName::parse("quantum::State")
            .expect("qualified name should be valid");

        assert_eq!(name.segments(), &["quantum", "State"]);
        assert_eq!(name.local_name(), "State");
        assert_eq!(name.canonical_string(), "quantum::State");
        assert_eq!(name.to_string(), "quantum::State");
    }

    #[test]
    fn qualified_names_reject_empty_or_invalid_segments() {
        assert!(matches!(
            QualifiedName::parse(""),
            Err(SchemaTypeError::EmptyQualifiedName)
        ));

        assert!(QualifiedName::parse("quantum::::State").is_err());
        assert!(QualifiedName::parse("quantum::9State").is_err());
        assert!(QualifiedName::parse("quantum::State!").is_err());
    }

    #[test]
    fn zero_integer_width_is_rejected() {
        assert!(matches!(
            IntegerWidth::fixed(0),
            Err(SchemaTypeError::ZeroIntegerWidth)
        ));

        assert!(SchemaType::integer_with_width(
            IntegerSignedness::Unsigned,
            0
        )
        .is_err());
    }

    #[test]
    fn fixed_and_symbolic_integer_widths_are_supported() {
        let fixed = SchemaType::integer_with_width(IntegerSignedness::Signed, 257)
            .expect("positive width should be valid");

        let symbolic =
            SchemaType::integer_with_symbolic_width(IntegerSignedness::Unsigned, "width_n")
                .expect("symbolic width should be valid");

        assert!(fixed.validate().is_ok());
        assert!(symbolic.validate().is_ok());
    }

    #[test]
    fn symbolic_width_rejects_invalid_identifiers() {
        assert!(IntegerWidth::symbolic("").is_err());
        assert!(IntegerWidth::symbolic("width expression").is_err());
        assert!(IntegerWidth::symbolic("2 * width").is_err());
    }

    #[test]
    fn objects_are_ordered_and_reject_duplicate_fields() {
        let schema = SchemaType::object(
            vec![
                ("zeta", SchemaType::String),
                ("alpha", SchemaType::Boolean),
            ],
            false,
        )
        .expect("object schema should be valid");

        let SchemaType::Object { fields, .. } = schema else {
            panic!("expected object schema");
        };

        assert_eq!(
            fields.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["alpha", "zeta"]
        );

        assert!(matches!(
            SchemaType::object(
                vec![
                    ("value", SchemaType::String),
                    ("value", SchemaType::Boolean)
                ],
                false
            ),
            Err(SchemaTypeError::DuplicateObjectField(_))
        ));
    }

    #[test]
    fn empty_union_is_rejected() {
        assert!(matches!(
            SchemaType::union(Vec::new()),
            Err(SchemaTypeError::EmptyUnion)
        ));
    }

    #[test]
    fn nested_schemas_validate_and_report_node_count() {
        let schema = SchemaType::object(
            vec![
                (
                    "items",
                    SchemaType::array(SchemaType::integer()),
                ),
                (
                    "label",
                    SchemaType::optional(SchemaType::String),
                ),
            ],
            false,
        )
        .expect("object schema should be valid");

        assert!(schema.validate().is_ok());

        // Object + array + integer + optional + string.
        assert_eq!(schema.node_count().expect("count should fit"), 5);
    }

    #[test]
    fn references_are_validated_but_not_resolved_here() {
        let name = QualifiedName::parse("future_domain::CustomType")
            .expect("name should be valid");

        let schema =
            SchemaType::reference(name).expect("reference descriptor should be valid");

        assert!(schema.validate().is_ok());
    }

    #[test]
    fn schema_round_trip_preserves_meaning() {
        let schema = SchemaType::object(
            vec![
                (
                    "count",
                    SchemaType::integer_with_symbolic_width(
                        IntegerSignedness::Unsigned,
                        "count_width",
                    )
                    .expect("symbolic width should be valid"),
                ),
                ("payload", SchemaType::array(SchemaType::Bytes)),
            ],
            true,
        )
        .expect("object schema should be valid");

        let encoded = serde_json::to_string(&schema).expect("serialization should succeed");
        let decoded: SchemaType =
            serde_json::from_str(&encoded).expect("deserialization should succeed");

        assert_eq!(decoded, schema);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn deserialized_invalid_schema_is_detected() {
        let malformed: SchemaType = serde_json::from_str(
            r#"{"kind":"union","variants":[]}"#,
        )
        .expect("the wire representation can be decoded");

        assert!(matches!(
            malformed.validate(),
            Err(SchemaTypeError::EmptyUnion)
        ));
    }

    #[test]
    fn large_explicit_integer_width_is_not_truncated() {
        let width = u128::MAX;

        let schema =
            SchemaType::integer_with_width(IntegerSignedness::Unsigned, width)
                .expect("positive width should be accepted");

        let SchemaType::Integer {
            width: Some(IntegerWidth::Fixed(actual)),
            ..
        } = schema
        else {
            panic!("expected fixed integer width");
        };

        assert_eq!(actual.get(), u128::MAX);
    }
}
