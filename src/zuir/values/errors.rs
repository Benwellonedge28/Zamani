
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Value Errors
//!
//! Canonical path: `src/zuir/values/errors.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Define the shared error contract for portable ZUIR values, constants,
//! literals, and expressions. This module provides value-specific failures
//! without replacing the canonical `crate::zuir::errors` contract.
//!
//! OWNS
//!
//! - Stable value-validation error categories and codes.
//! - Structured, optional context for value-validation failures.
//! - Distinction between invalid representations and operational limits.
//! - Error display and standard Rust error integration.
//! - Conversion into the canonical ZUIR error vocabulary.
//!
//! DOES_NOT_OWN
//!
//! - Source syntax, lexing, parsing, or AST diagnostics.
//! - Semantic type inference, type compatibility, or name resolution.
//! - Constant evaluation or expression interpretation.
//! - Validation algorithms owned by constants.rs, literals.rs,
//!   expressions.rs, or the IR verifier.
//! - Source spans or diagnostic rendering.
//! - Allocation policy or physical resource management.
//!
//! DEPENDENCIES
//!
//! This module depends only on the standard library and the canonical ZUIR
//! error contract. The canonical error module must not depend on this module.
//!
//! SCALABILITY
//!
//! No hardware capacities, value widths, tensor ranks, operation counts,
//! or domain-specific resource ceilings are imposed here. Context strings
//! and paths grow dynamically. Process-level allocation failure remains
//! possible and is not represented as a recoverable guarantee.
//!
//! INTEGRATION
//!
//! - values/mod.rs declares and exports this module.
//! - constants.rs uses this contract for canonical value-validation errors.
//! - literals.rs preserves literal-specific context and underlying causes.
//! - expressions.rs reports expression-validation failures through it.
//! - diagnostics.rs translates canonical ZUIR errors into user-facing
//!   diagnostics; it does not redefine these error categories.
//! - verify/types.rs remains responsible for semantic type compatibility.
//! - serialization/validation.rs validates decoded data before use.
//!
//! FREEZE CRITERIA
//!
//! - Error codes and their meanings are documented and tested.
//! - Resource exhaustion is distinct from invalid representation.
//! - Error conversion does not discard the original error message.
//! - The module introduces no circular dependency with crate::zuir::errors.
//! - Integration tests verify conversions and error chaining.

use core::fmt;
use std::error::Error as StdError;

use serde::{Deserialize, Serialize};

use crate::zuir::errors::{ZuirError, ZuirErrorKind};

/// Result type for operations using the shared value-error contract.
pub type ValueResult<T> = Result<T, ValueError>;

/// Stable classification of a value-related failure.
///
/// The enum is non-exhaustive so downstream crates can match known cases
/// while remaining compatible with future additions.
///
/// Adding a variant must not change the meaning of an existing variant
/// or its stable error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ValueErrorKind {
    /// An integer does not use the required canonical representation.
    InvalidInteger,

    /// A floating-point format identifier is malformed.
    InvalidFloatFormat,

    /// A floating-point bit pattern is malformed.
    InvalidFloatBits,

    /// A floating-point bit pattern is valid in form but noncanonical.
    NonCanonicalFloatBits,

    /// An exact decimal representation is malformed.
    InvalidDecimal,

    /// A byte sequence does not satisfy its encoding contract.
    InvalidByteEncoding,

    /// An extension value has an invalid identity.
    InvalidExtensionIdentity,

    /// An extension value has an invalid payload.
    InvalidExtensionPayload,

    /// An aggregate value violates its structural contract.
    InvalidAggregate,

    /// A literal violates its value representation contract.
    InvalidLiteral,

    /// An expression violates its structural representation contract.
    InvalidExpression,

    /// A qualified name is malformed.
    InvalidQualifiedName,

    /// A field access has an empty or invalid field name.
    InvalidFieldName,

    /// A referenced value has an invalid representation.
    InvalidValueReference,

    /// A referenced symbol has an invalid representation.
    InvalidSymbolReference,

    /// A value's structural representation is malformed.
    MalformedValue,

    /// A required value representation is not supported in this context.
    UnsupportedValueKind,

    /// A type descriptor embedded in a value is malformed.
    InvalidTypeRepresentation,

    /// Validation exceeded its configured node budget.
    NodeBudgetExceeded,

    /// Validation exceeded its configured nesting-depth budget.
    DepthBudgetExceeded,

    /// Validation exceeded its configured text-byte budget.
    TextBudgetExceeded,

    /// A validation counter or derived arithmetic operation overflowed.
    CounterOverflow,

    /// A configured validation budget is invalid or contradictory.
    InvalidValidationBudget,

    /// An internal value-subsystem invariant failed.
    InternalFailure,
}

impl ValueErrorKind {
    /// Returns the stable machine-readable code for this category.
    ///
    /// These identifiers are compatibility contracts. Never reuse a code
    /// for a different meaning.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidInteger =>
                "ZMN-IR-VALUE-INVALID-INTEGER",
            Self::InvalidFloatFormat =>
                "ZMN-IR-VALUE-INVALID-FLOAT-FORMAT",
            Self::InvalidFloatBits =>
                "ZMN-IR-VALUE-INVALID-FLOAT-BITS",
            Self::NonCanonicalFloatBits =>
                "ZMN-IR-VALUE-NONCANONICAL-FLOAT-BITS",
            Self::InvalidDecimal =>
                "ZMN-IR-VALUE-INVALID-DECIMAL",
            Self::InvalidByteEncoding =>
                "ZMN-IR-VALUE-INVALID-BYTE-ENCODING",
            Self::InvalidExtensionIdentity =>
                "ZMN-IR-VALUE-INVALID-EXTENSION-IDENTITY",
            Self::InvalidExtensionPayload =>
                "ZMN-IR-VALUE-INVALID-EXTENSION-PAYLOAD",
            Self::InvalidAggregate =>
                "ZMN-IR-VALUE-INVALID-AGGREGATE",
            Self::InvalidLiteral =>
                "ZMN-IR-VALUE-INVALID-LITERAL",
            Self::InvalidExpression =>
                "ZMN-IR-VALUE-INVALID-EXPRESSION",
            Self::InvalidQualifiedName =>
                "ZMN-IR-VALUE-INVALID-QUALIFIED-NAME",
            Self::InvalidFieldName =>
                "ZMN-IR-VALUE-INVALID-FIELD-NAME",
            Self::InvalidValueReference =>
                "ZMN-IR-VALUE-INVALID-VALUE-REFERENCE",
            Self::InvalidSymbolReference =>
                "ZMN-IR-VALUE-INVALID-SYMBOL-REFERENCE",
            Self::MalformedValue =>
                "ZMN-IR-VALUE-MALFORMED",
            Self::UnsupportedValueKind =>
                "ZMN-IR-VALUE-UNSUPPORTED-KIND",
            Self::InvalidTypeRepresentation =>
                "ZMN-IR-VALUE-INVALID-TYPE-REPRESENTATION",
            Self::NodeBudgetExceeded =>
                "ZMN-IR-VALUE-NODE-BUDGET-EXCEEDED",
            Self::DepthBudgetExceeded =>
                "ZMN-IR-VALUE-DEPTH-BUDGET-EXCEEDED",
            Self::TextBudgetExceeded =>
                "ZMN-IR-VALUE-TEXT-BUDGET-EXCEEDED",
            Self::CounterOverflow =>
                "ZMN-IR-VALUE-COUNTER-OVERFLOW",
            Self::InvalidValidationBudget =>
                "ZMN-IR-VALUE-INVALID-VALIDATION-BUDGET",
            Self::InternalFailure =>
                "ZMN-IR-VALUE-INTERNAL-FAILURE",
        }
    }

    /// Returns true when the failure represents an operational limit
    /// rather than proof that the value is malformed.
    #[must_use]
    pub const fn is_resource_limit(self) -> bool {
        matches!(
            self,
            Self::NodeBudgetExceeded
                | Self::DepthBudgetExceeded
                | Self::TextBudgetExceeded
                | Self::CounterOverflow
        )
    }

    /// Returns true when the failure indicates invalid value representation.
    #[must_use]
    pub const fn is_representation_failure(self) -> bool {
        matches!(
            self,
            Self::InvalidInteger
                | Self::InvalidFloatFormat
                | Self::InvalidFloatBits
                | Self::NonCanonicalFloatBits
                | Self::InvalidDecimal
                | Self::InvalidByteEncoding
                | Self::InvalidExtensionIdentity
                | Self::InvalidExtensionPayload
                | Self::InvalidAggregate
                | Self::InvalidLiteral
                | Self::InvalidExpression
                | Self::InvalidQualifiedName
                | Self::InvalidFieldName
                | Self::InvalidValueReference
                | Self::InvalidSymbolReference
                | Self::MalformedValue
                | Self::InvalidTypeRepresentation
        )
    }
}

impl fmt::Display for ValueErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidInteger => "invalid integer representation",
            Self::InvalidFloatFormat => "invalid floating-point format",
            Self::InvalidFloatBits => "invalid floating-point bit pattern",
            Self::NonCanonicalFloatBits =>
                "noncanonical floating-point bit pattern",
            Self::InvalidDecimal => "invalid exact decimal representation",
            Self::InvalidByteEncoding => "invalid byte encoding",
            Self::InvalidExtensionIdentity => "invalid extension identity",
            Self::InvalidExtensionPayload => "invalid extension payload",
            Self::InvalidAggregate => "invalid aggregate value",
            Self::InvalidLiteral => "invalid literal",
            Self::InvalidExpression => "invalid expression",
            Self::InvalidQualifiedName => "invalid qualified name",
            Self::InvalidFieldName => "invalid field name",
            Self::InvalidValueReference => "invalid value reference",
            Self::InvalidSymbolReference => "invalid symbol reference",
            Self::MalformedValue => "malformed value",
            Self::UnsupportedValueKind => "unsupported value kind",
            Self::InvalidTypeRepresentation => "invalid type representation",
            Self::NodeBudgetExceeded => "validation node budget exceeded",
            Self::DepthBudgetExceeded => "validation depth budget exceeded",
            Self::TextBudgetExceeded => "validation text budget exceeded",
            Self::CounterOverflow => "validation counter overflow",
            Self::InvalidValidationBudget => "invalid validation budget",
            Self::InternalFailure => "internal value-subsystem failure",
        })
    }
}

/// A path component identifying where a value error occurred.
///
/// These components describe the logical value structure, not source
/// locations, memory addresses, or target-specific indices.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuePathSegment {
    /// A field in an aggregate or record.
    Field(String),

    /// An element in a sequence.
    Element(usize),

    /// An operand of an expression.
    Operand(usize),

    /// A named value component.
    Component(String),

    /// An extension-defined component.
    Extension(String),
}

impl fmt::Display for ValuePathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field(name) => write!(f, "field({name})"),
            Self::Element(index) => write!(f, "element({index})"),
            Self::Operand(index) => write!(f, "operand({index})"),
            Self::Component(name) => write!(f, "component({name})"),
            Self::Extension(name) => write!(f, "extension({name})"),
        }
    }
}

/// Structured context attached to a value error.
///
/// Paths and contextual descriptions are dynamically sized. Callers should
/// avoid embedding secret data or arbitrarily large source fragments in
/// error context.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueErrorContext {
    path: Vec<ValuePathSegment>,
    detail: Option<String>,
}

impl ValueErrorContext {
    /// Creates an empty context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            path: Vec::new(),
            detail: None,
        }
    }

    /// Appends a logical path component.
    #[must_use]
    pub fn with_path_segment(mut self, segment: ValuePathSegment) -> Self {
        self.path.push(segment);
        self
    }

    /// Sets explanatory context.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Returns the logical value path.
    #[must_use]
    pub fn path(&self) -> &[ValuePathSegment] {
        &self.path
    }

    /// Returns the explanatory detail, if present.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

/// A structured error produced by value representation or validation.
///
/// The explanatory message is not a stable machine interface. Consumers
/// should branch on `kind()` or `code()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueError {
    kind: ValueErrorKind,
    message: String,
    context: ValueErrorContext,
}

impl ValueError {
    /// Creates a value error with the default explanation for its category.
    #[must_use]
    pub fn from_kind(kind: ValueErrorKind) -> Self {
        Self {
            kind,
            message: kind.to_string(),
            context: ValueErrorContext::new(),
        }
    }

    /// Creates a value error with a more specific explanatory message.
    #[must_use]
    pub fn new(
        kind: ValueErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            context: ValueErrorContext::new(),
        }
    }

    /// Returns the stable category.
    #[must_use]
    pub const fn kind(&self) -> ValueErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured context.
    #[must_use]
    pub fn context(&self) -> &ValueErrorContext {
        &self.context
    }

    /// Replaces the structured context.
    #[must_use]
    pub fn with_context(mut self, context: ValueErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Appends a path component to the structured context.
    #[must_use]
    pub fn with_path_segment(mut self, segment: ValuePathSegment) -> Self {
        self.context.path.push(segment);
        self
    }

    /// Adds explanatory detail to the structured context.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.context.detail = Some(detail.into());
        self
    }

    /// Converts this error into the canonical ZUIR error vocabulary.
    ///
    /// The value-specific code and structured context are retained as
    /// explanatory notes. The canonical error kind classifies the failure
    /// for cross-subsystem consumers.
    #[must_use]
    pub fn into_zui_error(self) -> ZuirError {
        let canonical_kind = if self.kind.is_resource_limit() {
            ZuirErrorKind::ResourceExhausted
        } else {
            match self.kind {
                ValueErrorKind::UnsupportedValueKind =>
                    ZuirErrorKind::UnsupportedOperation,

                ValueErrorKind::InvalidValidationBudget =>
                    ZuirErrorKind::ConfigurationInvalid,

                ValueErrorKind::InternalFailure =>
                    ZuirErrorKind::InternalFailure,

                _ => ZuirErrorKind::MalformedRepresentation,
            }
        };

        let mut error = ZuirError::new(canonical_kind, self.message)
            .with_note(format!("value error code: {}", self.kind.code()));

        if !self.context.path.is_empty() {
            let path = self
                .context
                .path
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" -> ");

            error = error.with_note(format!("value path: {path}"));
        }

        if let Some(detail) = self.context.detail {
            error = error.with_note(detail);
        }

        error
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message)?;

        if !self.context.path.is_empty() {
            f.write_str(" at ")?;

            for (index, segment) in self.context.path.iter().enumerate() {
                if index > 0 {
                    f.write_str(" -> ")?;
                }

                write!(f, "{segment}")?;
            }
        }

        if let Some(detail) = &self.context.detail {
            write!(f, ": {detail}")?;
        }

        Ok(())
    }
}

impl StdError for ValueError {}

impl From<ValueError> for ZuirError {
    fn from(error: ValueError) -> Self {
        error.into_zui_error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_error_codes_are_stable_and_namespaced() {
        assert_eq!(
            ValueErrorKind::InvalidInteger.code(),
            "ZMN-IR-VALUE-INVALID-INTEGER"
        );

        assert_eq!(
            ValueErrorKind::NodeBudgetExceeded.code(),
            "ZMN-IR-VALUE-NODE-BUDGET-EXCEEDED"
        );

        assert!(ValueErrorKind::InvalidInteger
            .code()
            .starts_with("ZMN-IR-VALUE-"));
    }

    #[test]
    fn representation_errors_are_distinct_from_budget_errors() {
        assert!(
            ValueErrorKind::InvalidInteger.is_representation_failure()
        );

        assert!(
            !ValueErrorKind::NodeBudgetExceeded.is_representation_failure()
        );

        assert!(ValueErrorKind::NodeBudgetExceeded.is_resource_limit());
        assert!(!ValueErrorKind::InvalidInteger.is_resource_limit());
    }

    #[test]
    fn context_preserves_nested_value_path() {
        let error = ValueError::from_kind(ValueErrorKind::InvalidInteger)
            .with_path_segment(ValuePathSegment::Field("payload".into()))
            .with_path_segment(ValuePathSegment::Element(3))
            .with_detail("integer is not canonical");

        assert_eq!(error.context().path().len(), 2);
        assert_eq!(
            error.context().detail(),
            Some("integer is not canonical")
        );

        let rendered = error.to_string();
        assert!(rendered.contains("field(payload)"));
        assert!(rendered.contains("element(3)"));
    }

    #[test]
    fn conversion_preserves_value_code_and_classifies_exhaustion() {
        let error = ValueError::from_kind(ValueErrorKind::NodeBudgetExceeded);
        let canonical: ZuirError = error.into();

        assert_eq!(canonical.kind(), ZuirErrorKind::ResourceExhausted);
        assert!(
            canonical
                .to_string()
                .contains("ZMN-IR-RESOURCE-EXHAUSTED")
        );

        assert!(
            canonical
                .context()
                .notes()
                .iter()
                .any(|note| note.contains("ZMN-IR-VALUE-NODE-BUDGET-EXCEEDED"))
        );
    }

    #[test]
    fn invalid_representation_converts_to_canonical_ir_error() {
        let error = ValueError::from_kind(ValueErrorKind::InvalidInteger);
        let canonical = error.into_zui_error();

        assert_eq!(
            canonical.kind(),
            ZuirErrorKind::MalformedRepresentation
        );
    }

    #[test]
    fn value_errors_are_serializable() {
        let error = ValueError::from_kind(ValueErrorKind::InvalidAggregate)
            .with_path_segment(ValuePathSegment::Field("items".into()));

        let encoded =
            serde_json::to_string(&error).expect("serialization succeeds");

        let decoded: ValueError =
            serde_json::from_str(&encoded).expect("deserialization succeeds");

        assert_eq!(decoded, error);
    }
}
