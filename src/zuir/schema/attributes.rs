
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Attributes
//!
//! Path: `src/zuir/schema/attributes.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the validated, deterministic attribute vocabulary used by ZUIR
//! schema declarations. Attributes describe values attached to operations,
//! types, functions, regions, modules, dialect declarations, and extensions.
//!
//! This module is deliberately independent of concrete IR nodes, builders,
//! operation verification, dialect registries, and execution backends.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/` schema ownership and compatibility contracts
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/Zamani.g4` and its canonical lexer/parser composition
//!
//! Attribute names and values do not define source-language syntax.
//! The grammar owns syntax; semantic analysis owns source meaning; ZUIR
//! schema attributes carry explicitly declared representation metadata.
//!
//! ## OWNS
//!
//! - Validated attribute names and qualified attribute keys.
//! - Attribute values and their schema-level type checking.
//! - Attribute definitions and multiplicity/requiredness policies.
//! - Deterministic attribute collections.
//! - Unknown-attribute handling policies.
//! - Structured local validation errors.
//!
//! ## DOES NOT OWN
//!
//! - Semantic types, type inference, or source name resolution.
//! - Source parsing, AST nodes, or source-language attribute syntax.
//! - Dialect registration or executable extension behavior.
//! - IR operations, regions, functions, or builders.
//! - Physical layout, device allocation, target feasibility, or execution.
//! - Serialization of complete programs or schema migration.
//! - Diagnostic rendering or source-map ownership.
//!
//! ## SCALABILITY
//!
//! Collections are dynamically sized. This module introduces no universal
//! machine, qubit, register-width, tensor-rank, device-count, or operation
//! count limit. Finite processes remain constrained by available resources.
//! Decoders for untrusted data must enforce the caller's configured resource
//! budget before allocating arbitrarily large values.
//!
//! ## DETERMINISM
//!
//! Attribute collections use `BTreeMap`; canonical iteration is key-ordered.
//! Ordered lists remain ordered because order can be semantically meaningful.
//! This module does not claim that arbitrary extension payloads are
//! canonical unless their value representation is canonical.
//!
//! ## FREEZE CRITERIA
//!
//! - Names and values are validated at construction and explicit validation.
//! - Invalid deserialized values are rejected by `validate`.
//! - Requiredness, multiplicity, and unknown-key policies are explicit.
//! - Schema types are reused rather than duplicated.
//! - No unsafe code or target-specific capacity assumptions are introduced.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

use super::types::{QualifiedName, SchemaType, SchemaTypeError};

// =============================================================================
// Names
// =============================================================================

/// A validated local attribute name.
///
/// Attribute names use the same ASCII identifier grammar as
/// `schema/types.rs`. Namespace qualification is represented separately by
/// [`AttributeKey`], avoiding delimiter-escaping ambiguities.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AttributeName(String);

impl AttributeName {
    /// Construct a validated attribute name.
    pub fn new(value: impl Into<String>) -> Result<Self, AttributeError> {
        let value = value.into();
        validate_identifier(&value)?;
        Ok(Self(value))
    }

    /// Return the local name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Recheck the invariant, including after deserialization.
    pub fn validate(&self) -> Result<(), AttributeError> {
        validate_identifier(&self.0)
    }
}

impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A namespace-qualified attribute key.
///
/// The namespace may identify a standard ZUIR namespace or an extension
/// namespace. The registry that owns a namespace is responsible for deciding
/// whether that namespace is authorized in a particular context.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AttributeKey {
    namespace: QualifiedName,
    name: AttributeName,
}

impl AttributeKey {
    /// Construct a key from a namespace and local name.
    pub fn new(
        namespace: QualifiedName,
        name: AttributeName,
    ) -> Result<Self, AttributeError> {
        namespace.validate()?;
        name.validate()?;
        Ok(Self { namespace, name })
    }

    /// Parse a key written as `namespace::local_name`.
    ///
    /// The final segment is the local name; preceding segments form the
    /// namespace. A namespace must contain at least one segment.
    pub fn parse(value: &str) -> Result<Self, AttributeError> {
        let mut segments: Vec<&str> = value.split("::").collect();

        if segments.len() < 2 || segments.iter().any(|segment| segment.is_empty()) {
            return Err(AttributeError::InvalidQualifiedKey(value.to_owned()));
        }

        let local = segments
            .pop()
            .ok_or_else(|| AttributeError::InvalidQualifiedKey(value.to_owned()))?;

        let namespace = QualifiedName::new(segments)?;
        let name = AttributeName::new(local)?;

        Self::new(namespace, name)
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &QualifiedName {
        &self.namespace
    }

    /// Return the local name.
    #[must_use]
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// Return the canonical `namespace::local_name` form.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Validate this key.
    pub fn validate(&self) -> Result<(), AttributeError> {
        self.namespace.validate()?;
        self.name.validate()
    }
}

impl fmt::Display for AttributeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

// =============================================================================
// Values
// =============================================================================

/// A schema-level attribute value.
///
/// These values are declarative data. They do not execute code, invoke a
/// dialect, access hardware, or evaluate arbitrary expressions.
///
/// Numeric literals that exceed the built-in integer range should be encoded
/// using an explicitly specified portable representation in a future schema
/// extension rather than silently truncated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AttributeValue {
    /// No value.
    Null,

    /// Boolean value.
    Boolean(bool),

    /// Signed integer represented as a portable decimal string.
    ///
    /// The string must be a canonical base-10 integer: optional leading `-`,
    /// no leading `+`, no leading zeroes except for zero itself, and no `-0`.
    Integer(String),

    /// Unsigned integer represented as a portable decimal string.
    ///
    /// The string must contain ASCII digits and must not have leading zeroes
    /// unless the value is zero.
    UnsignedInteger(String),

    /// Finite IEEE-754 binary64 value represented by its exact bit pattern.
    ///
    /// Storing bits avoids equality surprises involving NaN and signed zero.
    /// Non-finite values are rejected by the constructor and validator.
    Float64Bits(u64),

    /// Unicode string.
    String(String),

    /// Raw bytes.
    Bytes(Vec<u8>),

    /// Sequence of attribute values. Sequence order is preserved.
    Array(Vec<AttributeValue>),

    /// String-keyed object with deterministic key ordering.
    Object(BTreeMap<String, AttributeValue>),

    /// A qualified schema reference.
    Reference(QualifiedName),
}

impl AttributeValue {
    /// Construct a signed integer from a decimal string.
    pub fn integer(value: impl Into<String>) -> Result<Self, AttributeError> {
        let value = value.into();
        validate_signed_decimal(&value)?;
        Ok(Self::Integer(value))
    }

    /// Construct an unsigned integer from a decimal string.
    pub fn unsigned_integer(value: impl Into<String>) -> Result<Self, AttributeError> {
        let value = value.into();
        validate_unsigned_decimal(&value)?;
        Ok(Self::UnsignedInteger(value))
    }

    /// Construct a finite binary64 value.
    pub fn float64(value: f64) -> Result<Self, AttributeError> {
        if !value.is_finite() {
            return Err(AttributeError::NonFiniteFloat);
        }
        Ok(Self::Float64Bits(value.to_bits()))
    }

    /// Read a finite binary64 value.
    ///
    /// Returns `None` if a malformed serialized value contains NaN or
    /// infinity.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float64Bits(bits) => {
                let value = f64::from_bits(*bits);
                value.is_finite().then_some(value)
            }
            _ => None,
        }
    }

    /// Validate this value recursively without relying on constructor use.
    pub fn validate(&self) -> Result<(), AttributeError> {
        // An explicit work stack avoids recursive Rust calls for deeply
        // nested values. Allocation remains subject to available resources.
        let mut pending: Vec<&AttributeValue> = vec![self];

        while let Some(value) = pending.pop() {
            match value {
                Self::Null | Self::Boolean(_) | Self::String(_) | Self::Bytes(_) => {}

                Self::Integer(value) => validate_signed_decimal(value)?,

                Self::UnsignedInteger(value) => validate_unsigned_decimal(value)?,

                Self::Float64Bits(bits) => {
                    if !f64::from_bits(*bits).is_finite() {
                        return Err(AttributeError::NonFiniteFloat);
                    }
                }

                Self::Array(values) => pending.extend(values.iter()),

                Self::Object(fields) => {
                    for (key, child) in fields {
                        validate_identifier(key)?;
                        pending.push(child);
                    }
                }

                Self::Reference(name) => name.validate()?,
            }
        }

        Ok(())
    }

    /// Check whether this value conforms to a schema type.
    ///
    /// Reference resolution and dialect-specific meaning are deliberately
    /// outside this method. `SchemaType::Any` accepts every value that passes
    /// this value's structural validation.
    pub fn conforms_to(&self, schema: &SchemaType) -> Result<bool, AttributeError> {
        self.validate()?;
        schema.validate().map_err(AttributeError::InvalidSchemaType)?;

        let mut pending: Vec<(&AttributeValue, &SchemaType)> = vec![(self, schema)];

        while let Some((value, ty)) = pending.pop() {
            match ty {
                SchemaType::Any => {}

                SchemaType::Null => {
                    if !matches!(value, Self::Null) {
                        return Ok(false);
                    }
                }

                SchemaType::Boolean => {
                    if !matches!(value, Self::Boolean(_)) {
                        return Ok(false);
                    }
                }

                SchemaType::Integer { signedness, width } => {
                    let integer = match value {
                        Self::Integer(v) => {
                            if matches!(
                                signedness,
                                super::types::IntegerSignedness::Unsigned
                            ) {
                                return Ok(false);
                            }
                            v.as_str()
                        }
                        Self::UnsignedInteger(v) => {
                            if matches!(
                                signedness,
                                super::types::IntegerSignedness::Signed
                            ) {
                                return Ok(false);
                            }
                            v.as_str()
                        }
                        _ => return Ok(false),
                    };

                    if let Some(width) = width {
                        match width {
                            super::types::IntegerWidth::Fixed(bits) => {
                                if !integer_fits_width(
                                    integer,
                                    matches!(value, Self::Integer(_)),
                                    bits.get(),
                                ) {
                                    return Ok(false);
                                }
                            }
                            super::types::IntegerWidth::Symbolic(_) => {
                                // A symbolic width cannot be proven here.
                                // A constraint solver must resolve it.
                                return Err(AttributeError::UnresolvedSymbolicConstraint);
                            }
                        }
                    }
                }

                SchemaType::Number => {
                    if !matches!(
                        value,
                        Self::Integer(_)
                            | Self::UnsignedInteger(_)
                            | Self::Float64Bits(_)
                    ) {
                        return Ok(false);
                    }
                }

                SchemaType::String => {
                    if !matches!(value, Self::String(_)) {
                        return Ok(false);
                    }
                }

                SchemaType::Bytes => {
                    if !matches!(value, Self::Bytes(_)) {
                        return Ok(false);
                    }
                }

                SchemaType::Array { element } => match value {
                    Self::Array(values) => {
                        for item in values {
                            pending.push((item, element));
                        }
                    }
                    _ => return Ok(false),
                },

                SchemaType::Map { value: value_type } => match value {
                    Self::Object(fields) => {
                        for item in fields.values() {
                            pending.push((item, value_type));
                        }
                    }
                    _ => return Ok(false),
                },

                SchemaType::Object {
                    fields: declared,
                    allow_additional_fields,
                } => {
                    let actual = match value {
                        Self::Object(fields) => fields,
                        _ => return Ok(false),
                    };

                    for (name, declared_type) in declared {
                        let Some(actual_value) = actual.get(name) else {
                            return Ok(false);
                        };
                        pending.push((actual_value, declared_type));
                    }

                    if !allow_additional_fields
                        && actual.keys().any(|key| !declared.contains_key(key))
                    {
                        return Ok(false);
                    }
                }

                SchemaType::Union { variants } => {
                    // Avoid recursively calling this method for each branch.
                    // A union is checked independently because alternatives
                    // can contain nested types. The helper uses the same
                    // explicit-stack algorithm.
                    let mut matched = false;
                    for alternative in variants {
                        match value_conforms_to(value, alternative)? {
                            true => {
                                matched = true;
                                break;
                            }
                            false => {}
                        }
                    }
                    if !matched {
                        return Ok(false);
                    }
                }

                SchemaType::Optional { value: inner } => {
                    if !matches!(value, Self::Null) {
                        pending.push((value, inner));
                    }
                }

                SchemaType::Reference { .. } => {
                    // A reference is unresolved at this layer. It must not be
                    // treated as a wildcard or as an executable type.
                    return Err(AttributeError::UnresolvedSchemaReference);
                }
            }
        }

        Ok(true)
    }
}

// =============================================================================
// Attribute definitions and policies
// =============================================================================

/// Attribute multiplicity supported by a schema definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeMultiplicity {
    /// At most one value for this key.
    Single,

    /// Multiple values are allowed and their order is preserved.
    Repeated,
}

/// Policy for keys not declared by a schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownAttributePolicy {
    /// Reject every undeclared key.
    Reject,

    /// Preserve undeclared keys as inert data.
    ///
    /// This does not authorize execution or confer semantics on an extension.
    Preserve,

    /// Ignore undeclared keys during validation.
    ///
    /// Use only where the enclosing compatibility contract explicitly permits
    /// loss of unknown data. This policy is unsuitable for lossless
    /// round-tripping unless the ignored values are preserved elsewhere.
    Ignore,
}

/// Definition of one permitted attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeDefinition {
    key: AttributeKey,
    value_type: SchemaType,
    required: bool,
    multiplicity: AttributeMultiplicity,
    description: Option<String>,
}

impl AttributeDefinition {
    /// Create an attribute definition.
    pub fn new(
        key: AttributeKey,
        value_type: SchemaType,
        required: bool,
        multiplicity: AttributeMultiplicity,
    ) -> Result<Self, AttributeError> {
        key.validate()?;
        value_type
            .validate()
            .map_err(AttributeError::InvalidSchemaType)?;

        Ok(Self {
            key,
            value_type,
            required,
            multiplicity,
            description: None,
        })
    }

    /// Set a human-readable description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the attribute key.
    #[must_use]
    pub fn key(&self) -> &AttributeKey {
        &self.key
    }

    /// Return the schema type.
    #[must_use]
    pub fn value_type(&self) -> &SchemaType {
        &self.value_type
    }

    /// Whether the attribute is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }

    /// Return the multiplicity rule.
    #[must_use]
    pub const fn multiplicity(&self) -> AttributeMultiplicity {
        self.multiplicity
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate this definition, including after deserialization.
    pub fn validate(&self) -> Result<(), AttributeError> {
        self.key.validate()?;
        self.value_type
            .validate()
            .map_err(AttributeError::InvalidSchemaType)
    }
}

/// A validated set of attribute definitions.
///
/// Keys are unique. The BTreeMap gives deterministic iteration and
/// duplicate detection independent of input order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeSchema {
    definitions: BTreeMap<AttributeKey, AttributeDefinition>,
    unknown_policy: UnknownAttributePolicy,
}

impl AttributeSchema {
    /// Create an empty schema with an explicit unknown-key policy.
    #[must_use]
    pub fn new(unknown_policy: UnknownAttributePolicy) -> Self {
        Self {
            definitions: BTreeMap::new(),
            unknown_policy,
        }
    }

    /// Add a definition. Duplicate keys are rejected.
    pub fn insert(
        &mut self,
        definition: AttributeDefinition,
    ) -> Result<(), AttributeError> {
        definition.validate()?;
        let key = definition.key.clone();

        if self.definitions.contains_key(&key) {
            return Err(AttributeError::DuplicateDefinition(key));
        }

        self.definitions.insert(key, definition);
        Ok(())
    }

    /// Look up a definition.
    #[must_use]
    pub fn get(&self, key: &AttributeKey) -> Option<&AttributeDefinition> {
        self.definitions.get(key)
    }

    /// Iterate over definitions in canonical key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &AttributeDefinition)> {
        self.definitions.iter()
    }

    /// Return the unknown-key policy.
    #[must_use]
    pub const fn unknown_policy(&self) -> UnknownAttributePolicy {
        self.unknown_policy
    }

    /// Validate the schema's own invariants.
    pub fn validate(&self) -> Result<(), AttributeError> {
        for (key, definition) in &self.definitions {
            key.validate()?;
            definition.validate()?;

            if key != definition.key() {
                return Err(AttributeError::DefinitionKeyMismatch);
            }
        }
        Ok(())
    }

    /// Validate an attribute collection against this schema.
    pub fn validate_values(
        &self,
        values: &AttributeSet,
    ) -> Result<(), AttributeError> {
        self.validate()?;
        values.validate_structure()?;

        for (key, items) in &values.values {
            match self.definitions.get(key) {
                Some(definition) => {
                    if definition.multiplicity() == AttributeMultiplicity::Single
                        && items.len() > 1
                    {
                        return Err(AttributeError::MultiplicityViolation {
                            key: key.clone(),
                            actual: items.len(),
                        });
                    }

                    for item in items {
                        if !item.conforms_to(definition.value_type())? {
                            return Err(AttributeError::ValueTypeMismatch {
                                key: key.clone(),
                            });
                        }
                    }
                }

                None => match self.unknown_policy {
                    UnknownAttributePolicy::Reject => {
                        return Err(AttributeError::UnknownAttribute(key.clone()));
                    }
                    UnknownAttributePolicy::Preserve | UnknownAttributePolicy::Ignore => {}
                },
            }
        }

        for (key, definition) in &self.definitions {
            if definition.required() && !values.values.contains_key(key) {
                return Err(AttributeError::MissingRequiredAttribute(key.clone()));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Attribute collections
// =============================================================================

/// A deterministic collection of attribute values.
///
/// Each key maps to a vector to preserve multiplicity and value order. A
/// single-valued key is enforced by the associated `AttributeSchema`, not by
/// this container alone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeSet {
    values: BTreeMap<AttributeKey, Vec<AttributeValue>>,
}

impl AttributeSet {
    /// Create an empty attribute set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert one value for a key, preserving insertion order for repeated
    /// values. Schema multiplicity is checked later by `validate_against`.
    pub fn push(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<(), AttributeError> {
        key.validate()?;
        value.validate()?;
        self.values.entry(key).or_default().push(value);
        Ok(())
    }

    /// Replace all values for a key with one value.
    pub fn set(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<Option<Vec<AttributeValue>>, AttributeError> {
        key.validate()?;
        value.validate()?;
        Ok(self.values.insert(key, vec![value]))
    }

    /// Remove all values for a key.
    pub fn remove(&mut self, key: &AttributeKey) -> Option<Vec<AttributeValue>> {
        self.values.remove(key)
    }

    /// Return the values for a key.
    #[must_use]
    pub fn get(&self, key: &AttributeKey) -> Option<&[AttributeValue]> {
        self.values.get(key).map(Vec::as_slice)
    }

    /// Return the first value for a key, if any.
    #[must_use]
    pub fn get_one(&self, key: &AttributeKey) -> Option<&AttributeValue> {
        self.values.get(key).and_then(|values| values.first())
    }

    /// Iterate over all keys and their values in canonical key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &[AttributeValue])> {
        self.values
            .iter()
            .map(|(key, values)| (key, values.as_slice()))
    }

    /// Return the number of distinct keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the collection has no keys.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Validate internal structural invariants.
    pub fn validate_structure(&self) -> Result<(), AttributeError> {
        for (key, values) in &self.values {
            key.validate()?;
            for value in values {
                value.validate()?;
            }
        }
        Ok(())
    }

    /// Validate this collection against a schema.
    pub fn validate_against(
        &self,
        schema: &AttributeSchema,
    ) -> Result<(), AttributeError> {
        schema.validate_values(self)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error produced by attribute construction or schema validation.
///
/// Error variants are structured so callers can match on them instead of
/// parsing display strings. Diagnostic rendering belongs to the diagnostics
/// subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttributeError {
    EmptyIdentifier,
    InvalidIdentifier(String),
    InvalidQualifiedKey(String),
    InvalidSignedInteger(String),
    InvalidUnsignedInteger(String),
    NonFiniteFloat,
    DuplicateDefinition(AttributeKey),
    DefinitionKeyMismatch,
    UnknownAttribute(AttributeKey),
    MissingRequiredAttribute(AttributeKey),
    MultiplicityViolation {
        key: AttributeKey,
        actual: usize,
    },
    ValueTypeMismatch {
        key: AttributeKey,
    },
    UnresolvedSchemaReference,
    UnresolvedSymbolicConstraint,
    InvalidSchemaType(SchemaTypeError),
}

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => f.write_str("identifier must not be empty"),
            Self::InvalidIdentifier(value) => {
                write!(f, "invalid attribute identifier: {value:?}")
            }
            Self::InvalidQualifiedKey(value) => {
                write!(f, "invalid qualified attribute key: {value:?}")
            }
            Self::InvalidSignedInteger(value) => {
                write!(f, "invalid canonical signed integer: {value:?}")
            }
            Self::InvalidUnsignedInteger(value) => {
                write!(f, "invalid canonical unsigned integer: {value:?}")
            }
            Self::NonFiniteFloat => {
                f.write_str("attribute floating-point value must be finite")
            }
            Self::DuplicateDefinition(key) => {
                write!(f, "duplicate attribute definition: {key}")
            }
            Self::DefinitionKeyMismatch => {
                f.write_str("attribute definition map key does not match its definition")
            }
            Self::UnknownAttribute(key) => {
                write!(f, "unknown attribute: {key}")
            }
            Self::MissingRequiredAttribute(key) => {
                write!(f, "missing required attribute: {key}")
            }
            Self::MultiplicityViolation { key, actual } => {
                write!(
                    f,
                    "attribute {key} allows one value but received {actual}"
                )
            }
            Self::ValueTypeMismatch { key } => {
                write!(f, "attribute value does not conform to schema: {key}")
            }
            Self::UnresolvedSchemaReference => {
                f.write_str("schema reference must be resolved before validation")
            }
            Self::UnresolvedSymbolicConstraint => {
                f.write_str("symbolic integer-width constraint must be resolved")
            }
            Self::InvalidSchemaType(error) => {
                write!(f, "invalid attribute schema type: {error}")
            }
        }
    }
}

impl std::error::Error for AttributeError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(value: &str) -> Result<(), AttributeError> {
    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return Err(AttributeError::EmptyIdentifier);
    };

    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return Err(AttributeError::InvalidIdentifier(value.to_owned()));
    }

    Ok(())
}

fn validate_signed_decimal(value: &str) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(AttributeError::InvalidSignedInteger(value.to_owned()));
    }

    let digits = value.strip_prefix('-').unwrap_or(value);

    if digits.is_empty()
        || !digits.bytes().all(|byte| byte.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
        || value == "-0"
    {
        return Err(AttributeError::InvalidSignedInteger(value.to_owned()));
    }

    Ok(())
}

fn validate_unsigned_decimal(value: &str) -> Result<(), AttributeError> {
    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(AttributeError::InvalidUnsignedInteger(value.to_owned()));
    }

    Ok(())
}

fn integer_fits_width(value: &str, signed: bool, width: u128) -> bool {
    // This helper only proves bounds for widths representable by u32-sized
    // decimal parsing. Arbitrarily large widths are not rejected: every
    // finite decimal literal fits an integer width greater than its required
    // bit count, and the required count is computed from the literal.
    if width == 0 {
        return false;
    }

    let negative = value.starts_with('-');
    let digits = value.strip_prefix('-').unwrap_or(value);

    let Ok(magnitude) = digits.parse::<u128>() else {
        // Very large integer strings need arbitrary-precision arithmetic to
        // prove a tight bound. Do not claim that they fit a narrow width.
        // A conservative width test based on decimal digit count is enough
        // to reject clearly too-small widths; callers needing exact proof
        // should use the repository's arbitrary-precision constant layer.
        let decimal_digits = digits.len() as u128;
        let estimated_bits = decimal_digits.saturating_mul(4);
        return if signed {
            width > estimated_bits
        } else {
            !negative && width >= estimated_bits
        };
    };

    if signed {
        if width == 1 {
            return magnitude == 0 && !negative;
        }

        let magnitude_bits = u128::BITS as u128 - magnitude.leading_zeros() as u128;

        if negative {
            magnitude_bits < width
        } else {
            magnitude_bits < width
        }
    } else {
        !negative && (u128::BITS as u128 - magnitude.leading_zeros() as u128) <= width
    }
}

fn value_conforms_to(
    value: &AttributeValue,
    schema: &SchemaType,
) -> Result<bool, AttributeError> {
    // The common non-union path uses the public explicit-stack checker.
    // This helper handles unions by checking each alternative independently.
    if let SchemaType::Union { variants } = schema {
        for variant in variants {
            if value_conforms_to(value, variant)? {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    value.conforms_to(schema)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> AttributeKey {
        AttributeKey::parse(value).expect("valid test key")
    }

    #[test]
    fn qualified_keys_are_validated_and_canonical() {
        let value = AttributeKey::parse("core::optimization_level")
            .expect("valid key");

        assert_eq!(value.canonical_string(), "core::optimization_level");
        assert!(AttributeKey::parse("core::::bad").is_err());
        assert!(AttributeKey::parse("core::").is_err());
        assert!(AttributeKey::parse("single").is_err());
    }

    #[test]
    fn canonical_integer_strings_are_checked() {
        assert!(AttributeValue::integer("-123").is_ok());
        assert!(AttributeValue::integer("0").is_ok());
        assert!(AttributeValue::integer("-0").is_err());
        assert!(AttributeValue::integer("01").is_err());
        assert!(AttributeValue::unsigned_integer("0").is_ok());
        assert!(AttributeValue::unsigned_integer("001").is_err());
        assert!(AttributeValue::unsigned_integer("-1").is_err());
    }

    #[test]
    fn non_finite_floats_are_rejected() {
        assert!(AttributeValue::float64(1.25).is_ok());
        assert!(AttributeValue::float64(f64::NAN).is_err());
        assert!(AttributeValue::float64(f64::INFINITY).is_err());
    }

    #[test]
    fn schema_enforces_requiredness_and_types() {
        let attr_key = key("core::enabled");
        let definition = AttributeDefinition::new(
            attr_key.clone(),
            SchemaType::Boolean,
            true,
            AttributeMultiplicity::Single,
        )
        .expect("valid definition");

        let mut schema = AttributeSchema::new(UnknownAttributePolicy::Reject);
        schema.insert(definition).expect("insert definition");

        let empty = AttributeSet::new();
        assert!(matches!(
            schema.validate_values(&empty),
            Err(AttributeError::MissingRequiredAttribute(_))
        ));

        let mut values = AttributeSet::new();
        values
            .push(attr_key, AttributeValue::Boolean(true))
            .expect("insert value");

        assert!(schema.validate_values(&values).is_ok());
    }

    #[test]
    fn single_valued_attributes_reject_duplicates() {
        let attr_key = key("core::mode");
        let definition = AttributeDefinition::new(
            attr_key.clone(),
            SchemaType::String,
            false,
            AttributeMultiplicity::Single,
        )
        .expect("valid definition");

        let mut schema = AttributeSchema::new(UnknownAttributePolicy::Reject);
        schema.insert(definition).expect("insert definition");

        let mut values = AttributeSet::new();
        values
            .push(attr_key.clone(), AttributeValue::String("a".into()))
            .expect("first value");
        values
            .push(attr_key, AttributeValue::String("b".into()))
            .expect("second value");

        assert!(matches!(
            schema.validate_values(&values),
            Err(AttributeError::MultiplicityViolation { .. })
        ));
    }

    #[test]
    fn unknown_attributes_follow_explicit_policy() {
        let mut values = AttributeSet::new();
        values
            .push(key("vendor_example::flag"), AttributeValue::Boolean(true))
            .expect("insert value");

        let reject = AttributeSchema::new(UnknownAttributePolicy::Reject);
        assert!(matches!(
            reject.validate_values(&values),
            Err(AttributeError::UnknownAttribute(_))
        ));

        let preserve = AttributeSchema::new(UnknownAttributePolicy::Preserve);
        assert!(preserve.validate_values(&values).is_ok());
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn object_keys_are_deterministically_ordered() {
        let mut object = BTreeMap::new();
        object.insert("z".to_owned(), AttributeValue::Boolean(false));
        object.insert("a".to_owned(), AttributeValue::Boolean(true));

        let value = AttributeValue::Object(object);
        value.validate().expect("valid object");

        let AttributeValue::Object(fields) = value else {
            panic!("expected object");
        };

        assert_eq!(
            fields.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["a", "z"]
        );
    }

    #[test]
    fn symbolic_width_is_not_guessed() {
        let schema = SchemaType::integer_with_symbolic_width(
            super::super::types::IntegerSignedness::Unspecified,
            "WIDTH",
        )
        .expect("valid symbolic width");

        let value = AttributeValue::integer("7").expect("integer");
        assert!(matches!(
            value.conforms_to(&schema),
            Err(AttributeError::UnresolvedSymbolicConstraint)
        ));
    }

    #[test]
    fn malformed_deserialized_values_are_caught_by_validation() {
        // Serde can construct public deserialized data without using the
        // constructors. Explicit validation therefore remains mandatory at
        // trust boundaries.
        let value = AttributeValue::Integer("01".to_owned());
        assert!(matches!(
            value.validate(),
            Err(AttributeError::InvalidSignedInteger(_))
        ));
    }
}
