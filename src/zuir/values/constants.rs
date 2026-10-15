#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Constant Values
//!
//! Canonical path: `src/zuir/values/constants.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Define portable, typed, serializable constants for the Zamani Universal
//! Intermediate Representation (ZUIR).
//!
//! OWNS
//!
//! - The canonical constant and constant-value data model.
//! - Exact, host-independent integer and decimal representations.
//! - Explicit floating-point format and bit-pattern representation.
//! - Aggregate, byte-string, textual, symbolic, and extension values.
//! - Local structural validation and configurable validation budgets.
//! - Stable local validation error categories.
//!
//! DOES_NOT_OWN
//!
//! - Semantic type definitions or type identity.
//! - Name resolution, type inference, or type compatibility.
//! - Expression evaluation or constant folding.
//! - Arbitrary-precision arithmetic implementations.
//! - Source parsing or literal syntax.
//! - IR operation construction or verification.
//! - Serialization of complete ZUIR programs.
//! - Physical memory allocation or target-specific numeric lowering.
//!
//! AUTHORITY
//!
//! This module must conform to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/types/type_def.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! DEPENDENCIES
//!
//! - `crate::zuir::types::type_def::TypeRef` supplies semantic type references.
//! - Serde supplies serialization traits.
//! - The standard library supplies deterministic ordered maps and errors.
//!
//! INTEGRATION
//!
//! - `src/zuir/values/mod.rs` declares and exports this module.
//! - `values/literals.rs` may parse source literals into this representation.
//! - `values/expressions.rs` may refer to constants in constant expressions.
//! - `types/type_def.rs` remains the owner of semantic type definitions.
//! - `verify/types.rs` validates type compatibility.
//! - `serialization/validation.rs` validates deserialized representations.
//! - A future constant evaluator owns arithmetic and evaluation budgets.
//! - `fingerprint/stable_hash.rs` owns stable content fingerprints.
//! - Dialects may use extension values under their own validation contracts.
//!
//! SCALABILITY
//!
//! No universal integer-width, floating-point-width, aggregate-length,
//! tensor-rank, quantum-resource, or hardware-capacity limit is imposed.
//!
//! Arbitrary-width integers and decimal coefficients are stored as canonical
//! decimal strings. Floating-point bits are stored as canonical hexadecimal
//! strings. This avoids accidental host-width truncation.
//!
//! Collections grow dynamically. A caller may impose an explicit validation
//! budget. This module cannot guarantee literal infinity: all realizations
//! remain subject to available memory, address space, execution time, and
//! other finite implementation resources.
//!
//! SEMANTICS
//!
//! A constant's representation does not establish that its value is valid
//! for its declared semantic type. The owning type verifier must check that
//! relationship. Similarly, representing a symbolic constant does not
//! evaluate it.
//!
//! Unknown extension values remain opaque data. They must not acquire
//! executable meaning without a registered extension contract.
//!
//! FREEZE CRITERIA
//!
//! - No host-width integer truncation.
//! - No implicit floating-point conversion.
//! - No unchecked recursive validation.
//! - No dependence on builders, evaluators, or backends.
//! - Deterministic ordered aggregate representation.
//! - Malformed numeric encodings are rejected.
//! - Validation budgets produce explicit exhaustion errors.
//! - Round-trip serialization tests preserve represented values.
//!
//! STATUS
//!
//! Proposed implementation. Freeze only after the module is declared,
//! repository integration compiles, and the required tests pass.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::zuir::types::type_def::TypeRef;

// =============================================================================
// Constant
// =============================================================================

/// A typed constant in the ZUIR value model.
///
/// The declared type is a semantic type reference. This structure does not
/// resolve that reference or prove that `value` conforms to the type.
///
/// Call [`Constant::validate`] to validate the local representation, then
/// pass the constant to the canonical type verifier for semantic validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constant {
    /// Semantic type expected for this value.
    pub ty: TypeRef,

    /// Represented constant value.
    pub value: ConstantValue,
}

impl Constant {
    /// Construct a constant without claiming semantic type compatibility.
    ///
    /// This constructor deliberately does not resolve `ty`. Call `validate`
    /// and the appropriate semantic verifier before trusting external input.
    #[must_use]
    pub fn new(ty: TypeRef, value: ConstantValue) -> Self {
        Self { ty, value }
    }

    /// Validate the local value representation using the default policy.
    ///
    /// The default policy imposes no explicit node or text-byte budget.
    /// Call [`Constant::validate_with`] to impose operational limits.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        self.value.validate()
    }

    /// Validate the local value representation under an explicit policy.
    pub fn validate_with(
        &self,
        budget: &ConstantValidationBudget,
    ) -> Result<(), ConstantValidationError> {
        self.value.validate_with(budget)
    }

    /// Borrow the represented value.
    #[must_use]
    pub fn value(&self) -> &ConstantValue {
        &self.value
    }

    /// Borrow the declared semantic type reference.
    #[must_use]
    pub fn ty(&self) -> &TypeRef {
        &self.ty
    }

    /// Consume this constant and return its represented value.
    #[must_use]
    pub fn into_value(self) -> ConstantValue {
        self.value
    }
}

// =============================================================================
// Constant values
// =============================================================================

/// The representation of a constant value.
///
/// This enum is non-exhaustive so future standard representations can be
/// introduced without promising that the current variants cover every
/// computational domain.
///
/// All variants describe values, not target instructions or physical
/// allocations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConstantValue {
    /// No value.
    Unit,

    /// Boolean value.
    Boolean(bool),

    /// Arbitrary-width mathematical integer represented as canonical decimal.
    ///
    /// Valid examples: `"0"`, `"42"`, `"-42"`.
    ///
    /// Invalid examples: `"+42"`, `"042"`, `"-0"`, `" 42"`.
    ///
    /// This is a representation, not an arithmetic engine. Operations on
    /// these values require an evaluator that supports arbitrary precision.
    Integer(ArbitraryInteger),

    /// Floating-point value represented by an explicit format and bit pattern.
    ///
    /// `bits_hex` contains lowercase hexadecimal digits without a prefix.
    /// The format identifier defines the bit layout and interpretation.
    ///
    /// This representation preserves bits without converting through `f32`,
    /// `f64`, a host floating-point parser, or JSON numeric values.
    Float(FloatBits),

    /// Exact decimal value represented as a coefficient and decimal exponent.
    ///
    /// The represented mathematical value is:
    ///
    /// `coefficient × 10^exponent`
    ///
    /// Both fields use canonical signed decimal integer encodings. A later
    /// canonicalization pass may normalize equivalent representations, but
    /// this type does not silently rewrite a supplied value.
    Decimal(ExactDecimal),

    /// Unicode text.
    String(String),

    /// Opaque bytes encoded as lowercase hexadecimal.
    ///
    /// An empty string represents an empty byte sequence. Every other valid
    /// encoding contains an even number of hexadecimal digits.
    BytesHex(String),

    /// Ordered aggregate, such as a tuple.
    ///
    /// Element order is significant and is preserved during serialization.
    Sequence(Vec<ConstantValue>),

    /// Record-like aggregate with deterministic field ordering.
    ///
    /// `BTreeMap` makes iteration and serialized map ordering deterministic.
    Record(BTreeMap<String, ConstantValue>),

    /// A selected variant and its associated payload.
    ///
    /// The variant name is a semantic identifier owned by the relevant type
    /// or dialect. This representation does not resolve that identifier.
    Variant {
        name: String,
        fields: Vec<ConstantValue>,
    },

    /// Symbolic value whose meaning has not been evaluated.
    ///
    /// The expression is an opaque reference to an expression owned by the
    /// expression subsystem. This module does not parse or evaluate it.
    Symbolic {
        expression: String,
    },

    /// A namespaced value defined by a registered extension.
    ///
    /// The payload is opaque. Registration and validation of extension
    /// semantics belong to the extension or dialect subsystem.
    Extension(ExtensionConstant),
}

// =============================================================================
// Numeric representations
// =============================================================================

/// An arbitrary-width integer in canonical decimal notation.
///
/// This representation avoids restricting semantic integers to a host
/// integer width. Its finite string length remains bounded by available
/// memory and any explicitly configured validation budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArbitraryInteger(String);

impl ArbitraryInteger {
    /// Construct an integer from a canonical decimal string.
    pub fn new(value: impl Into<String>) -> Result<Self, ConstantValidationError> {
        let value = value.into();

        if !is_canonical_integer(&value) {
            return Err(ConstantValidationError::InvalidInteger);
        }

        Ok(Self(value))
    }

    /// Construct the integer zero.
    #[must_use]
    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    /// Return the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return whether the represented integer is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    /// Validate the representation again.
    ///
    /// This is important for values created through deserialization, which
    /// can bypass the checked constructor.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        if is_canonical_integer(&self.0) {
            Ok(())
        } else {
            Err(ConstantValidationError::InvalidInteger)
        }
    }

    /// Consume the wrapper and return the canonical decimal string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ArbitraryInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A floating-point bit pattern.
///
/// `format` is a nonempty, qualified semantic format name. It is not a
/// vendor name or a promise that a particular target supports the format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FloatBits {
    /// Floating-point semantic format identifier.
    pub format: String,

    /// Exact bit pattern, represented by lowercase hexadecimal digits.
    pub bits_hex: String,
}

impl FloatBits {
    /// Construct and validate a floating-point representation.
    pub fn new(
        format: impl Into<String>,
        bits_hex: impl Into<String>,
    ) -> Result<Self, ConstantValidationError> {
        let value = Self {
            format: format.into(),
            bits_hex: bits_hex.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Validate the format name and bit-pattern encoding.
    ///
    /// The number of bits required by the named format must be checked by
    /// the format registry or semantic type verifier.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        if self.format.trim().is_empty() {
            return Err(ConstantValidationError::InvalidFloatFormat);
        }

        if self.bits_hex.is_empty()
            || !self
                .bits_hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(ConstantValidationError::InvalidFloatBits);
        }

        if self.bits_hex.len() > 1 && self.bits_hex.starts_with('0') {
            return Err(ConstantValidationError::NonCanonicalFloatBits);
        }

        Ok(())
    }
}

/// An exact decimal value.
///
/// `coefficient` is an arbitrary-width integer. `exponent` is an arbitrary-
/// width signed decimal integer, so the representation does not impose a
/// machine-sized exponent ceiling.
///
/// For example, coefficient `"125"` and exponent `"-2"` represent `1.25`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactDecimal {
    /// Arbitrary-width signed coefficient.
    pub coefficient: ArbitraryInteger,

    /// Arbitrary-width signed base-10 exponent.
    pub exponent: ArbitraryInteger,
}

impl ExactDecimal {
    /// Construct and validate an exact decimal value.
    pub fn new(
        coefficient: ArbitraryInteger,
        exponent: ArbitraryInteger,
    ) -> Result<Self, ConstantValidationError> {
        let value = Self {
            coefficient,
            exponent,
        };

        value.validate()?;
        Ok(value)
    }

    /// Validate both arbitrary-width components.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        self.coefficient.validate()?;
        self.exponent.validate()
    }
}

// =============================================================================
// Extension values
// =============================================================================

/// An opaque extension-defined constant.
///
/// Namespace ownership and executable interpretation are established by the
/// extension registry, not by this data structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionConstant {
    /// Qualified extension namespace.
    ///
    /// Example: `"org.example.numeric"`.
    pub namespace: String,

    /// Extension-specific value identifier.
    pub name: String,

    /// Version of the extension-defined value encoding.
    pub version: String,

    /// Opaque payload bytes represented as lowercase hexadecimal.
    pub payload_hex: String,
}

impl ExtensionConstant {
    /// Construct and validate the local extension-value representation.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        payload_hex: impl Into<String>,
    ) -> Result<Self, ConstantValidationError> {
        let value = Self {
            namespace: namespace.into(),
            name: name.into(),
            version: version.into(),
            payload_hex: payload_hex.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Validate extension identifiers and payload encoding.
    ///
    /// This method does not prove that the extension is registered, trusted,
    /// available, compatible, or permitted to execute.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        if self.namespace.trim().is_empty()
            || self.name.trim().is_empty()
            || self.version.trim().is_empty()
        {
            return Err(ConstantValidationError::InvalidExtensionIdentity);
        }

        if !is_lowercase_hex(&self.payload_hex) {
            return Err(ConstantValidationError::InvalidExtensionPayload);
        }

        Ok(())
    }
}

// =============================================================================
// Validation budgets
// =============================================================================

/// Explicit limits for validating a constant tree.
///
/// `None` means that this policy imposes no explicit limit for that dimension.
/// It does not imply infinite resources.
///
/// Node count includes the root value and every nested value. Text-byte count
/// includes strings, symbolic expressions, numeric encodings, field names,
/// float format names, and extension metadata and payload encodings.
///
/// The declared type graph is not traversed here; semantic type validation
/// belongs to the type subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConstantValidationBudget {
    /// Maximum number of value nodes visited.
    pub max_nodes: Option<usize>,

    /// Maximum combined UTF-8 byte length of validated value text.
    pub max_text_bytes: Option<usize>,
}

impl ConstantValidationBudget {
    /// Create a validation budget with no explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_text_bytes: None,
        }
    }

    /// Create a budget with explicit node and text-byte limits.
    #[must_use]
    pub const fn new(max_nodes: Option<usize>, max_text_bytes: Option<usize>) -> Self {
        Self {
            max_nodes,
            max_text_bytes,
        }
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// A local constant representation validation failure.
///
/// These errors do not indicate that a semantically incompatible constant
/// type has been detected. That decision belongs to the type verifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstantValidationError {
    InvalidInteger,
    InvalidFloatFormat,
    InvalidFloatBits,
    NonCanonicalFloatBits,
    InvalidDecimal,
    InvalidByteEncoding,
    InvalidExtensionIdentity,
    InvalidExtensionPayload,
    InvalidVariantName,
    EmptySymbolicExpression,
    EmptyRecordFieldName,
    NodeBudgetExceeded,
    TextBudgetExceeded,
    CounterOverflow,
}

impl fmt::Display for ConstantValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidInteger => "integer is not in canonical decimal notation",
            Self::InvalidFloatFormat => "floating-point format identifier is empty",
            Self::InvalidFloatBits => "floating-point bit pattern is invalid",
            Self::NonCanonicalFloatBits => "floating-point bit pattern is not canonical",
            Self::InvalidDecimal => "exact decimal representation is invalid",
            Self::InvalidByteEncoding => "byte sequence is not canonical lowercase hexadecimal",
            Self::InvalidExtensionIdentity => "extension constant identity is incomplete",
            Self::InvalidExtensionPayload => "extension payload is not canonical lowercase hexadecimal",
            Self::InvalidVariantName => "variant name is empty",
            Self::EmptySymbolicExpression => "symbolic expression reference is empty",
            Self::EmptyRecordFieldName => "record field name is empty",
            Self::NodeBudgetExceeded => "constant validation node budget exceeded",
            Self::TextBudgetExceeded => "constant validation text-byte budget exceeded",
            Self::CounterOverflow => "constant validation counter overflowed",
        };

        f.write_str(message)
    }
}

impl Error for ConstantValidationError {}

// =============================================================================
// Validation implementation
// =============================================================================

impl ConstantValue {
    /// Validate this value without imposing an explicit work budget.
    ///
    /// Traversal uses an explicit stack, avoiding recursive calls through
    /// arbitrarily nested aggregate values.
    pub fn validate(&self) -> Result<(), ConstantValidationError> {
        self.validate_with(&ConstantValidationBudget::unlimited())
    }

    /// Validate this value under an explicit operational budget.
    ///
    /// Budget exhaustion is reported separately from malformed encodings.
    /// The validator stops as soon as a configured limit is exceeded.
    pub fn validate_with(
        &self,
        budget: &ConstantValidationBudget,
    ) -> Result<(), ConstantValidationError> {
        let mut pending: Vec<&ConstantValue> = vec![self];
        let mut visited_nodes: usize = 0;
        let mut visited_text_bytes: usize = 0;

        while let Some(value) = pending.pop() {
            visited_nodes = visited_nodes
                .checked_add(1)
                .ok_or(ConstantValidationError::CounterOverflow)?;

            if let Some(limit) = budget.max_nodes {
                if visited_nodes > limit {
                    return Err(ConstantValidationError::NodeBudgetExceeded);
                }
            }

            match value {
                Self::Unit | Self::Boolean(_) => {}

                Self::Integer(integer) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        integer.as_str().len(),
                        budget.max_text_bytes,
                    )?;
                    integer.validate()?;
                }

                Self::Float(float) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        float.format.len(),
                        budget.max_text_bytes,
                    )?;
                    add_text_bytes(
                        &mut visited_text_bytes,
                        float.bits_hex.len(),
                        budget.max_text_bytes,
                    )?;
                    float.validate()?;
                }

                Self::Decimal(decimal) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        decimal.coefficient.as_str().len(),
                        budget.max_text_bytes,
                    )?;
                    add_text_bytes(
                        &mut visited_text_bytes,
                        decimal.exponent.as_str().len(),
                        budget.max_text_bytes,
                    )?;
                    decimal
                        .validate()
                        .map_err(|_| ConstantValidationError::InvalidDecimal)?;
                }

                Self::String(text) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        text.len(),
                        budget.max_text_bytes,
                    )?;
                }

                Self::BytesHex(bytes) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        bytes.len(),
                        budget.max_text_bytes,
                    )?;

                    if !is_lowercase_hex(bytes) {
                        return Err(ConstantValidationError::InvalidByteEncoding);
                    }
                }

                Self::Sequence(elements) => {
                    pending.extend(elements.iter());
                }

                Self::Record(fields) => {
                    for (name, field_value) in fields {
                        add_text_bytes(
                            &mut visited_text_bytes,
                            name.len(),
                            budget.max_text_bytes,
                        )?;

                        if name.trim().is_empty() {
                            return Err(ConstantValidationError::EmptyRecordFieldName);
                        }

                        pending.push(field_value);
                    }
                }

                Self::Variant { name, fields } => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        name.len(),
                        budget.max_text_bytes,
                    )?;

                    if name.trim().is_empty() {
                        return Err(ConstantValidationError::InvalidVariantName);
                    }

                    pending.extend(fields.iter());
                }

                Self::Symbolic { expression } => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        expression.len(),
                        budget.max_text_bytes,
                    )?;

                    if expression.trim().is_empty() {
                        return Err(ConstantValidationError::EmptySymbolicExpression);
                    }
                }

                Self::Extension(extension) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        extension.namespace.len(),
                        budget.max_text_bytes,
                    )?;
                    add_text_bytes(
                        &mut visited_text_bytes,
                        extension.name.len(),
                        budget.max_text_bytes,
                    )?;
                    add_text_bytes(
                        &mut visited_text_bytes,
                        extension.version.len(),
                        budget.max_text_bytes,
                    )?;
                    add_text_bytes(
                        &mut visited_text_bytes,
                        extension.payload_hex.len(),
                        budget.max_text_bytes,
                    )?;

                    extension.validate()?;
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Canonical encoding helpers
// =============================================================================

/// Return whether a string is a canonical signed decimal integer.
///
/// The canonical form is `0`, a positive integer without leading zeroes, or
/// a negative integer without leading zeroes. Negative zero is forbidden.
fn is_canonical_integer(value: &str) -> bool {
    if value == "0" {
        return true;
    }

    let digits = match value.strip_prefix('-') {
        Some(rest) => rest,
        None => value,
    };

    if digits.is_empty() || digits.starts_with('0') {
        return false;
    }

    digits.bytes().all(|b| b.is_ascii_digit())
}

/// Return whether a string is lowercase hexadecimal with an even digit count.
///
/// The empty string is valid, allowing empty byte sequences and payloads.
fn is_lowercase_hex(value: &str) -> bool {
    value.len() % 2 == 0
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Add validated text length using checked arithmetic and an optional budget.
fn add_text_bytes(
    total: &mut usize,
    amount: usize,
    limit: Option<usize>,
) -> Result<(), ConstantValidationError> {
    *total = total
        .checked_add(amount)
        .ok_or(ConstantValidationError::CounterOverflow)?;

    if let Some(limit) = limit {
        if *total > limit {
            return Err(ConstantValidationError::TextBudgetExceeded);
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_integer_accepts_canonical_values() {
        for value in ["0", "1", "42", "-1", "-42", "123456789012345678901234567890"] {
            assert_eq!(ArbitraryInteger::new(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn arbitrary_integer_rejects_noncanonical_values() {
        for value in ["", "+1", "00", "01", "-0", "-01", " 1", "1 ", "--1"] {
            assert!(
                ArbitraryInteger::new(value).is_err(),
                "unexpectedly accepted {value:?}"
            );
        }
    }

    #[test]
    fn deserialized_integer_representation_is_revalidated() {
        // Public fields and deserialization can bypass checked constructors.
        let malformed: ArbitraryInteger =
            serde_json::from_str(r#""0007""#).unwrap();

        assert_eq!(
            malformed.validate(),
            Err(ConstantValidationError::InvalidInteger)
        );
    }

    #[test]
    fn float_bits_preserve_explicit_representation() {
        let value = FloatBits::new("ieee.binary64", "3ff0000000000000").unwrap();

        assert_eq!(value.format, "ieee.binary64");
        assert_eq!(value.bits_hex, "3ff0000000000000");
    }

    #[test]
    fn float_bits_reject_uppercase_and_leading_zeroes() {
        assert!(FloatBits::new("format", "ABCD").is_err());
        assert!(FloatBits::new("format", "00ab").is_err());
        assert!(FloatBits::new("format", "").is_err());
    }

    #[test]
    fn exact_decimal_accepts_arbitrary_width_components() {
        let coefficient =
            ArbitraryInteger::new("123456789012345678901234567890").unwrap();
        let exponent = ArbitraryInteger::new("-100000000000000000000").unwrap();

        let decimal = ExactDecimal::new(coefficient, exponent).unwrap();
        assert_eq!(
            decimal.coefficient.as_str(),
            "123456789012345678901234567890"
        );
        assert_eq!(decimal.exponent.as_str(), "-100000000000000000000");
    }

    #[test]
    fn empty_bytes_are_valid() {
        assert!(ConstantValue::BytesHex(String::new()).validate().is_ok());
    }

    #[test]
    fn byte_encoding_rejects_odd_length_or_uppercase() {
        assert!(ConstantValue::BytesHex("a".to_owned()).validate().is_err());
        assert!(ConstantValue::BytesHex("AB".to_owned()).validate().is_err());
        assert!(ConstantValue::BytesHex("0g".to_owned()).validate().is_err());
    }

    #[test]
    fn records_have_deterministic_field_order() {
        let mut fields = BTreeMap::new();
        fields.insert("z".to_owned(), ConstantValue::Boolean(false));
        fields.insert("a".to_owned(), ConstantValue::Boolean(true));

        let names: Vec<&str> = fields.keys().map(String::as_str).collect();
        assert_eq!(names, vec!["a", "z"]);
    }

    #[test]
    fn nested_values_validate() {
        let value = ConstantValue::Sequence(vec![
            ConstantValue::Boolean(true),
            ConstantValue::Integer(ArbitraryInteger::new("999999999999999999999").unwrap()),
            ConstantValue::Record(BTreeMap::from([(
                "answer".to_owned(),
                ConstantValue::String("valid".to_owned()),
            )])),
        ]);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn node_budget_exhaustion_is_explicit() {
        let value = ConstantValue::Sequence(vec![
            ConstantValue::Boolean(true),
            ConstantValue::Boolean(false),
        ]);

        let budget = ConstantValidationBudget::new(Some(2), None);
        assert_eq!(
            value.validate_with(&budget),
            Err(ConstantValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn text_budget_exhaustion_is_explicit() {
        let value = ConstantValue::String("zamani".to_owned());
        let budget = ConstantValidationBudget::new(None, Some(3));

        assert_eq!(
            value.validate_with(&budget),
            Err(ConstantValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn symbolic_values_require_a_nonempty_reference() {
        assert!(ConstantValue::Symbolic {
            expression: "shape.dimension".to_owned()
        }
        .validate()
        .is_ok());

        assert!(ConstantValue::Symbolic {
            expression: "   ".to_owned()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn extension_payload_is_opaque_but_encoding_is_checked() {
        let extension = ExtensionConstant::new(
            "org.example.numeric",
            "wide_integer",
            "1",
            "00ff",
        )
        .unwrap();

        assert!(ConstantValue::Extension(extension).validate().is_ok());

        assert!(ExtensionConstant::new(
            "org.example.numeric",
            "wide_integer",
            "1",
            "0F",
        )
        .is_err());
    }

    #[test]
    fn serialization_round_trip_preserves_constant_value() {
        let value = ConstantValue::Decimal(
            ExactDecimal::new(
                ArbitraryInteger::new("125").unwrap(),
                ArbitraryInteger::new("-2").unwrap(),
            )
            .unwrap(),
        );

        let encoded = serde_json::to_string(&value).unwrap();
        let decoded: ConstantValue = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, value);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn unit_and_boolean_values_are_valid() {
        assert!(ConstantValue::Unit.validate().is_ok());
        assert!(ConstantValue::Boolean(true).validate().is_ok());
    }
}