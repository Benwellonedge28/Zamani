//! Source-level Zamani attributes.
//!
//! # Architectural role
//!
//! This module defines the source-language representation of an attribute.
//! An attribute is declarative metadata attached to another AST construct.
//!
//! This type is intentionally:
//!
//! - source-oriented;
//! - domain-neutral;
//! - hardware-independent;
//! - deterministic;
//! - namespace-extensible;
//! - free of semantic-resolution state;
//! - free of backend-specific state;
//! - free of machine-size assumptions.
//!
//! # Layering
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! parser
//!     |
//!     v
//! Attribute
//!     |
//!     v
//! AST validation
//!     |
//!     v
//! semantic analysis
//!     |
//!     v
//! ZUIR
//!     |
//!     v
//! domain / target lowering
//! ```
//!
//! This module MUST NOT depend on semantic analysis, ZUIR, quantum IR,
//! hardware, scheduling, routing, calibration, QEC, or backend code.
//!
//! # Determinism
//!
//! Attribute arguments are stored in source order in `Vec` rather than in a
//! hash map. This preserves source ordering and avoids making serialization
//! dependent on hash-map iteration order.
//!
//! # Scalability
//!
//! There are deliberately no language-level maximums for:
//!
//! - namespace depth;
//! - attribute-name length;
//! - argument count;
//! - nested attribute-value depth;
//! - collection sizes.
//!
//! Compiler resource/safety limits belong to configurable compiler policy,
//! not to this AST data model.
//!
//! # Rust compatibility
//!
//! This implementation is intended for Rust 1.97 / 1.97.1 and uses no
//! `unsafe` code.

use core::fmt;

use serde::{Deserialize, Serialize};

/// A source-level Zamani attribute.
///
/// An attribute consists of an extensible, namespaced identity and zero or
/// more ordered arguments.
///
/// Examples of the conceptual source forms supported by this representation:
///
/// ```text
/// @inline
/// @compile::inline
/// @requires(capability = "quantum")
/// @resource(kind = "quantum", size = N)
/// @custom::property("value")
/// ```
///
/// The AST does not interpret what an attribute means. Interpretation belongs
/// to later compiler phases.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Attribute {
    /// Namespaced attribute identity.
    ///
    /// The namespace is represented as an ordered sequence rather than a
    /// single string so that the AST does not need to parse or reinterpret
    /// namespace syntax repeatedly.
    name: AttributeName,

    /// Ordered source arguments.
    ///
    /// A vector is intentional: argument order can be source-significant for
    /// some extensions, and preserving it makes serialization deterministic.
    arguments: Vec<AttributeArgument>,
}

impl Attribute {
    /// Creates a flag-style attribute with no arguments.
    ///
    /// # Examples
    ///
    /// ```text
    /// @inline
    /// ```
    pub fn flag(name: AttributeName) -> Self {
        Self {
            name,
            arguments: Vec::new(),
        }
    }

    /// Creates an attribute with ordered arguments.
    pub fn new(name: AttributeName, arguments: Vec<AttributeArgument>) -> Self {
        Self { name, arguments }
    }

    /// Returns the attribute identity.
    #[must_use]
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// Returns the ordered attribute arguments.
    #[must_use]
    pub fn arguments(&self) -> &[AttributeArgument] {
        &self.arguments
    }

    /// Returns a mutable view of the ordered arguments.
    ///
    /// This is intentionally limited to arguments. The identity itself is
    /// changed through `set_name`, making mutation explicit.
    pub fn arguments_mut(&mut self) -> &mut Vec<AttributeArgument> {
        &mut self.arguments
    }

    /// Replaces the attribute identity.
    pub fn set_name(&mut self, name: AttributeName) {
        self.name = name;
    }

    /// Returns whether this attribute has no arguments.
    #[must_use]
    pub fn is_flag(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Appends one argument while preserving source order.
    pub fn push_argument(&mut self, argument: AttributeArgument) {
        self.arguments.push(argument);
    }

    /// Removes all arguments.
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    /// Performs structural validation only.
    ///
    /// This function deliberately does not resolve:
    ///
    /// - whether the attribute exists;
    /// - whether the namespace is registered;
    /// - whether the target supports it;
    /// - whether its arguments have the correct semantic type;
    /// - whether it is legal for a particular quantum backend.
    ///
    /// Those checks belong to semantic/extension validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        self.name.validate()?;

        for (index, argument) in self.arguments.iter().enumerate() {
            argument
                .validate()
                .map_err(|source| AttributeValidationError::InvalidArgument {
                    index,
                    source: Box::new(source),
                })?;
        }

        Ok(())
    }
}

/// An extensible attribute identity.
///
/// The identity is deliberately represented as namespace segments plus a
/// terminal name rather than as a closed enum.
///
/// This permits future compiler/domain extensions without modifying this
/// source type.
///
/// Examples:
///
/// ```text
/// inline
/// compiler::inline
/// quantum::resource
/// user::project::property
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AttributeName {
    /// Namespace segments preceding the terminal name.
    namespace: Vec<String>,

    /// Terminal attribute identifier.
    name: String,
}

impl AttributeName {
    /// Creates an unqualified attribute name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            namespace: Vec::new(),
            name: name.into(),
        }
    }

    /// Creates a namespaced attribute.
    pub fn namespaced<I, S>(namespace: I, name: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            namespace: namespace.into_iter().map(Into::into).collect(),
            name: name.into(),
        }
    }

    /// Returns the namespace segments.
    #[must_use]
    pub fn namespace(&self) -> &[String] {
        &self.namespace
    }

    /// Returns the terminal name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this attribute is namespaced.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        !self.namespace.is_empty()
    }

    /// Returns the complete namespace-qualified identity as components.
    ///
    /// No allocation occurs for the common case where callers only need
    /// individual components.
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.namespace
            .iter()
            .map(String::as_str)
            .chain(core::iter::once(self.name.as_str()))
    }

    /// Validates the structural form of the identity.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        if self.name.is_empty() {
            return Err(AttributeValidationError::EmptyName);
        }

        validate_identifier(&self.name)?;

        for (index, segment) in self.namespace.iter().enumerate() {
            if segment.is_empty() {
                return Err(AttributeValidationError::EmptyNamespaceSegment { index });
            }

            validate_identifier(segment)?;
        }

        Ok(())
    }
}

impl fmt::Display for AttributeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for component in self.components() {
            if !first {
                formatter.write_str("::")?;
            }

            formatter.write_str(component)?;
            first = false;
        }

        Ok(())
    }
}

/// A source-level attribute argument.
///
/// Named and positional arguments are both supported.
///
/// Examples:
///
/// ```text
/// @foo("bar")
/// @foo(value = "bar")
/// @foo(10, mode = "fast")
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttributeArgument {
    /// Positional argument.
    Positional(AttributeValue),

    /// Named argument.
    Named {
        /// Source-level argument name.
        name: String,

        /// Source-level argument value.
        value: AttributeValue,
    },
}

impl AttributeArgument {
    /// Creates a positional argument.
    #[must_use]
    pub fn positional(value: AttributeValue) -> Self {
        Self::Positional(value)
    }

    /// Creates a named argument.
    pub fn named(name: impl Into<String>, value: AttributeValue) -> Self {
        Self::Named {
            name: name.into(),
            value,
        }
    }

    /// Returns the argument's value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        match self {
            Self::Positional(value) => value,
            Self::Named { value, .. } => value,
        }
    }

    /// Returns the named argument identifier when this is a named argument.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Positional(_) => None,
            Self::Named { name, .. } => Some(name),
        }
    }

    /// Returns whether this is a named argument.
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named { .. })
    }

    /// Performs structural validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        if let Self::Named { name, value } = self {
            if name.is_empty() {
                return Err(AttributeValidationError::EmptyArgumentName);
            }

            validate_identifier(name)?;
            value.validate()?;
        } else {
            self.value().validate()?;
        }

        Ok(())
    }
}

/// Source-preserving value representation for attribute arguments.
///
/// This is intentionally separate from Zamani's general expression/literal
/// AST. Attribute arguments are metadata syntax and must not force this file
/// to depend on the complete expression hierarchy.
///
/// Semantic analysis may interpret identifiers, paths, numbers, or strings
/// according to the registered attribute definition.
///
/// This representation does not impose machine-width limits.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttributeValue {
    /// Boolean literal.
    Boolean(bool),

    /// Arbitrary-precision source integer represented lexically.
    ///
    /// The AST intentionally does not parse the value into `i64`, `u64`, or
    /// another machine-sized integer.
    Integer(String),

    /// Exact source representation of a floating/decimal value.
    ///
    /// Semantic analysis determines the appropriate numeric interpretation.
    Float(String),

    /// UTF-8 string value.
    String(String),

    /// Character literal represented in source-independent character form.
    Character(char),

    /// Identifier reference.
    Identifier(String),

    /// Qualified path/reference.
    ///
    /// Each segment is retained separately to preserve structure.
    Path(Vec<String>),

    /// Nested attribute value collection.
    Array(Vec<AttributeValue>),

    /// Key/value object-like value.
    ///
    /// A vector is used instead of a hash map to preserve source ordering and
    /// deterministic serialization.
    Object(Vec<(String, AttributeValue)>),

    /// Explicit null/unit-like value.
    Null,
}

impl AttributeValue {
    /// Performs structural validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        match self {
            Self::Integer(value) => {
                if value.is_empty() {
                    return Err(AttributeValidationError::EmptyLiteral);
                }
            }

            Self::Float(value) => {
                if value.is_empty() {
                    return Err(AttributeValidationError::EmptyLiteral);
                }
            }

            Self::Identifier(identifier) => {
                if identifier.is_empty() {
                    return Err(AttributeValidationError::EmptyIdentifier);
                }

                validate_identifier(identifier)?;
            }

            Self::Path(segments) => {
                if segments.is_empty() {
                    return Err(AttributeValidationError::EmptyPath);
                }

                for (index, segment) in segments.iter().enumerate() {
                    if segment.is_empty() {
                        return Err(AttributeValidationError::EmptyPathSegment { index });
                    }

                    validate_identifier(segment)?;
                }
            }

            Self::Array(values) => {
                for value in values {
                    value.validate()?;
                }
            }

            Self::Object(entries) => {
                for (key, value) in entries {
                    if key.is_empty() {
                        return Err(AttributeValidationError::EmptyObjectKey);
                    }

                    validate_identifier(key)?;
                    value.validate()?;
                }
            }

            Self::Boolean(_)
            | Self::String(_)
            | Self::Character(_)
            | Self::Null => {}
        }

        Ok(())
    }
}

/// Structural errors produced by this module.
///
/// These errors intentionally do not include semantic errors such as
/// "unknown attribute", "unsupported backend", or "invalid quantum resource".
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeValidationError {
    /// The terminal attribute name is empty.
    EmptyName,

    /// A namespace segment is empty.
    EmptyNamespaceSegment {
        /// Position of the invalid segment.
        index: usize,
    },

    /// An identifier violates the source-level identifier structure.
    InvalidIdentifier {
        /// The invalid identifier.
        value: String,
    },

    /// A named argument has no name.
    EmptyArgumentName,

    /// An argument contains invalid structure.
    InvalidArgument {
        /// Position of the invalid argument.
        index: usize,

        /// Underlying validation error.
        source: Box<AttributeValidationError>,
    },

    /// An identifier value is empty.
    EmptyIdentifier,

    /// A path has no segments.
    EmptyPath,

    /// A path contains an empty segment.
    EmptyPathSegment {
        /// Position of the invalid segment.
        index: usize,
    },

    /// An object key is empty.
    EmptyObjectKey,

    /// A lexical numeric literal is empty.
    EmptyLiteral,
}

impl fmt::Display for AttributeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("attribute name must not be empty"),

            Self::EmptyNamespaceSegment { index } => {
                write!(formatter, "attribute namespace segment {index} must not be empty")
            }

            Self::InvalidIdentifier { value } => {
                write!(formatter, "invalid attribute identifier `{value}`")
            }

            Self::EmptyArgumentName => {
                formatter.write_str("attribute argument name must not be empty")
            }

            Self::InvalidArgument { index, source } => {
                write!(formatter, "invalid attribute argument {index}: {source}")
            }

            Self::EmptyIdentifier => {
                formatter.write_str("attribute identifier value must not be empty")
            }

            Self::EmptyPath => formatter.write_str("attribute path must contain at least one segment"),

            Self::EmptyPathSegment { index } => {
                write!(formatter, "attribute path segment {index} must not be empty")
            }

            Self::EmptyObjectKey => {
                formatter.write_str("attribute object key must not be empty")
            }

            Self::EmptyLiteral => {
                formatter.write_str("attribute literal must not be empty")
            }
        }
    }
}

impl std::error::Error for AttributeValidationError {}

/// Validates a source-level identifier.
///
/// This function intentionally implements only the structural invariant
/// needed by this independent attribute representation.
///
/// Full Zamani identifier semantics should remain centralized in the
/// canonical frontend identifier/path infrastructure. This local check
/// therefore accepts Unicode identifier characters through Rust's Unicode
/// character predicates while rejecting whitespace and punctuation.
///
/// The function does not impose a maximum identifier length.
fn validate_identifier(value: &str) -> Result<(), AttributeValidationError> {
    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return Err(AttributeValidationError::InvalidIdentifier {
            value: value.to_owned(),
        });
    };

    if !is_identifier_start(first) || characters.any(|character| !is_identifier_continue(character))
    {
        return Err(AttributeValidationError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    Ok(())
}

#[inline]
fn is_identifier_start(character: char) -> bool {
    character == '_' || character.is_alphabetic()
}

#[inline]
fn is_identifier_continue(character: char) -> bool {
    character == '_' || character.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_flag_attribute() {
        let attribute = Attribute::flag(AttributeName::new("inline"));

        assert!(attribute.is_flag());
        assert_eq!(attribute.name().name(), "inline");
        assert!(attribute.arguments().is_empty());
    }

    #[test]
    fn preserves_namespace_components() {
        let name = AttributeName::namespaced(["compiler", "optimization"], "inline");

        assert!(name.is_namespaced());
        assert_eq!(
            name.namespace(),
            &["compiler".to_owned(), "optimization".to_owned()]
        );
        assert_eq!(name.name(), "inline");
        assert_eq!(name.to_string(), "compiler::optimization::inline");
    }

    #[test]
    fn preserves_argument_order() {
        let attribute = Attribute::new(
            AttributeName::new("example"),
            vec![
                AttributeArgument::positional(AttributeValue::String("first".into())),
                AttributeArgument::named(
                    "mode",
                    AttributeValue::String("second".into()),
                ),
                AttributeArgument::positional(AttributeValue::Integer("999999999999999999999999".into())),
            ],
        );

        assert_eq!(attribute.arguments().len(), 3);
        assert_eq!(attribute.arguments()[0].name(), None);
        assert_eq!(attribute.arguments()[1].name(), Some("mode"));
    }

    #[test]
    fn accepts_large_lexical_integer_without_machine_width() {
        let value = AttributeValue::Integer(
            "999999999999999999999999999999999999999999999999999999".into(),
        );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn accepts_deeply_structured_values() {
        let value = AttributeValue::Array(vec![
            AttributeValue::Object(vec![
                (
                    "resource".into(),
                    AttributeValue::Path(vec![
                        "quantum".into(),
                        "logical".into(),
                        "resource".into(),
                    ]),
                ),
            ]),
        ]);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_empty_attribute_name() {
        let attribute = Attribute::flag(AttributeName::new(""));

        assert_eq!(
            attribute.validate(),
            Err(AttributeValidationError::EmptyName)
        );
    }

    #[test]
    fn rejects_empty_namespace_segment() {
        let name = AttributeName::namespaced(["compiler", ""], "inline");

        assert_eq!(
            name.validate(),
            Err(AttributeValidationError::EmptyNamespaceSegment { index: 1 })
        );
    }

    #[test]
    fn rejects_invalid_identifier() {
        let name = AttributeName::new("not-valid");

        assert!(matches!(
            name.validate(),
            Err(AttributeValidationError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn rejects_empty_path() {
        let value = AttributeValue::Path(Vec::new());

        assert_eq!(
            value.validate(),
            Err(AttributeValidationError::EmptyPath)
        );
    }

    #[test]
    fn validates_named_arguments() {
        let argument = AttributeArgument::named(
            "requires",
            AttributeValue::String("quantum".into()),
        );

        assert!(argument.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_named_argument() {
        let argument = AttributeArgument::named(
            "not-valid",
            AttributeValue::Boolean(true),
        );

        assert!(matches!(
            argument.validate(),
            Err(AttributeValidationError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn deterministic_equality_is_independent_of_hash_map_order() {
        let first = Attribute::new(
            AttributeName::new("example"),
            vec![
                AttributeArgument::named("a", AttributeValue::Integer("1".into())),
                AttributeArgument::named("b", AttributeValue::Integer("2".into())),
            ],
        );

        let second = Attribute::new(
            AttributeName::new("example"),
            vec![
                AttributeArgument::named("a", AttributeValue::Integer("1".into())),
                AttributeArgument::named("b", AttributeValue::Integer("2".into())),
            ],
        );

        assert_eq!(first, second);
    }
}