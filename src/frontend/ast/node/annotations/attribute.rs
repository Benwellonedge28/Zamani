//! Zamani Native AST — Source Attribute
//!
//! Canonical source-level representation of a Zamani attribute.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! Attribute  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-oriented representation of one attribute.
//!
//! An attribute is metadata attached to another source-level AST construct.
//! The attribute records what the source expresses; it does not determine the
//! eventual implementation of that intent.
//!
//! This module therefore does NOT:
//!
//! - resolve attribute semantics;
//! - resolve names;
//! - perform type checking;
//! - select a target;
//! - select hardware;
//! - allocate resources;
//! - select a quantum backend;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform pulse generation;
//! - perform error correction;
//! - perform resilience;
//! - lower directly to QIR;
//! - lower directly to MLIR;
//! - lower directly to hardware instructions;
//! - execute attribute code.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! Attribute identity and values are intentionally extensible.
//!
//! There is no:
//!
//! - fixed attribute count;
//! - fixed namespace count;
//! - fixed namespace depth;
//! - fixed argument count;
//! - fixed value collection size;
//! - fixed integer width;
//! - fixed machine size;
//! - fixed quantum-resource count;
//! - fixed backend;
//! - fixed vendor;
//! - fixed computational domain.
//!
//! Operational resource limits, when required for untrusted input, belong to
//! configurable compiler policy rather than this source representation.
//!
//! "Infinity" in POCO-REAF means that this representation introduces no
//! artificial finite machine/resource limit. Actual execution remains bounded
//! by available memory, address space, compilation policy, and target
//! capabilities.
//!
//! # Source preservation
//!
//! Attribute arguments retain source order. Duplicate arguments and duplicate
//! attributes are not silently normalized here.
//!
//! Semantic rules such as:
//!
//! - whether duplicate names are legal;
//! - whether ordering matters;
//! - whether a value is required;
//! - whether an attribute is permitted on a particular node;
//! - whether an attribute is deprecated;
//! - whether an attribute is understood by an extension
//!
//! belong to semantic/extension validation.
//!
//! # Extensibility
//!
//! Attribute names are represented as namespace segments plus a terminal name.
//! They are deliberately not a closed Rust enum.
//!
//! Thus a future extension can introduce:
//!
//! ```text
//! zamani::compiler::inline
//! zamani::quantum::resource
//! zamani::tooling::generated
//! extension::future::property
//! vendor::example::feature
//! ```
//!
//! without modifying this core representation.
//!
//! The presence of a namespace does NOT grant semantic authority to that
//! namespace. Registration and interpretation belong to later compiler layers.
//!
//! # Value representation
//!
//! Attribute values remain source-oriented.
//!
//! Numeric values are retained lexically rather than immediately converted to
//! machine-sized integers or floating-point values. This avoids imposing
//! accidental `i64`, `u64`, `f64`, or host-word-size limits on source syntax.
//!
//! Attribute values intentionally do not depend on the general expression AST.
//! This keeps this foundational file independent and prevents dependency
//! cycles.
//!
//! # Determinism
//!
//! Ordered collections use `Vec`.
//!
//! Object-like values use an ordered vector of key/value entries rather than a
//! hash map. This preserves source order and makes canonical serialization
//! deterministic without depending on hash iteration order.
//!
//! # Source spans
//!
//! The canonical Zamani AST `Span` is used for source provenance.
//!
//! `Span` represents a validated half-open byte range:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Source coordinate validation belongs to the canonical source subsystem.
//!
//! # Integration contract
//!
//! ```text
//! parser
//!   │
//!   ▼
//! Attribute
//!   │
//!   ├── attached to AST node
//!   │
//!   ▼
//! structural validation
//!   │
//!   ▼
//! semantic interpretation
//!   │
//!   ▼
//! semantic constraints / metadata
//!   │
//!   ▼
//! ZUIR when computationally relevant
//! ```
//!
//! The attribute module does not depend on its parent AST node. This preserves
//! dependency direction and prevents circular dependencies.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1, edition 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::source::Span;

/// Schema version for this source-level attribute representation.
///
/// This version belongs to the attribute data model. It is intentionally
/// independent from the Zamani language version, compiler version, enclosing
/// AST schema version, serialization format version, and extension versions.
pub const ATTRIBUTE_SCHEMA_VERSION: u16 = 1;

/// A source-level Zamani attribute.
///
/// # Representation
///
/// An attribute consists of:
///
/// - a namespaced identity;
/// - zero or more ordered arguments;
/// - a source span.
///
/// The type contains no semantic-resolution state.
///
/// # Duplicate attributes
///
/// Duplicate attributes are valid at the representation level. Their
/// interpretation belongs to semantic analysis.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute identity.
    name: AttributeName,

    /// Ordered source arguments.
    arguments: Vec<AttributeArgument>,

    /// Source span covering the complete attribute.
    span: Span,
}

impl Attribute {
    /// Creates a flag-style attribute with no arguments.
    ///
    /// Example:
    ///
    /// ```text
    /// @inline
    /// ```
    #[must_use]
    pub fn flag(name: AttributeName, span: Span) -> Self {
        Self {
            name,
            arguments: Vec::new(),
            span,
        }
    }

    /// Creates an attribute with ordered arguments.
    #[must_use]
    pub fn new(
        name: AttributeName,
        arguments: Vec<AttributeArgument>,
        span: Span,
    ) -> Self {
        Self {
            name,
            arguments,
            span,
        }
    }

    /// Returns the attribute identity.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// Returns the ordered arguments.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> &[AttributeArgument] {
        &self.arguments
    }

    /// Returns mutable access to the ordered arguments.
    ///
    /// This method changes only source representation. It does not perform
    /// semantic interpretation.
    #[inline]
    pub fn arguments_mut(&mut self) -> &mut Vec<AttributeArgument> {
        &mut self.arguments
    }

    /// Returns the complete source span of the attribute.
    #[inline]
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Replaces the attribute identity.
    ///
    /// The caller remains responsible for preserving any parser/semantic
    /// invariants associated with the containing AST.
    pub fn set_name(&mut self, name: AttributeName) {
        self.name = name;
    }

    /// Replaces the source span.
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Appends one source argument.
    ///
    /// Existing source order is preserved.
    pub fn push_argument(&mut self, argument: AttributeArgument) {
        self.arguments.push(argument);
    }

    /// Removes all arguments.
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    /// Returns whether the attribute has no arguments.
    #[inline]
    #[must_use]
    pub fn is_flag(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns whether at least one argument is present.
    #[inline]
    #[must_use]
    pub fn has_arguments(&self) -> bool {
        !self.arguments.is_empty()
    }

    /// Performs structural validation.
    ///
    /// This validates only representation invariants.
    ///
    /// It does NOT determine:
    ///
    /// - whether the attribute is known;
    /// - whether it is legal on the containing AST node;
    /// - whether its values have the correct semantic type;
    /// - whether a target supports it;
    /// - whether a quantum backend supports it.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        self.name.validate()?;

        for (index, argument) in self.arguments.iter().enumerate() {
            argument.validate().map_err(|source| {
                AttributeValidationError::InvalidArgument {
                    index,
                    source: Box::new(source),
                }
            })?;
        }

        Ok(())
    }
}

/// Extensible namespaced attribute identity.
///
/// A name is represented as:
///
/// ```text
/// namespace::segment::...::terminal_name
/// ```
///
/// The representation is structural rather than one preformatted string.
/// This avoids reparsing names during later compiler phases and makes
/// namespace components directly accessible.
///
/// The namespace is open-ended. New extensions do not require changes to this
/// type.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AttributeName {
    /// Zero or more namespace components.
    namespace: Vec<String>,

    /// Terminal attribute identifier.
    name: String,
}

impl AttributeName {
    /// Creates an unqualified attribute name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            namespace: Vec::new(),
            name: name.into(),
        }
    }

    /// Creates a namespaced attribute name.
    #[must_use]
    pub fn namespaced<I, S>(
        namespace: I,
        name: impl Into<String>,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            namespace: namespace.into_iter().map(Into::into).collect(),
            name: name.into(),
        }
    }

    /// Returns the namespace components.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &[String] {
        &self.namespace
    }

    /// Returns the terminal name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the name has a namespace.
    #[inline]
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        !self.namespace.is_empty()
    }

    /// Returns all namespace components followed by the terminal name.
    ///
    /// The iterator does not allocate.
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.namespace
            .iter()
            .map(String::as_str)
            .chain(core::iter::once(self.name.as_str()))
    }

    /// Returns the number of namespace components.
    #[inline]
    #[must_use]
    pub fn namespace_depth(&self) -> usize {
        self.namespace.len()
    }

    /// Validates the structural identifier representation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        validate_identifier(&self.name)?;

        for (index, component) in self.namespace.iter().enumerate() {
            if component.is_empty() {
                return Err(
                    AttributeValidationError::EmptyNamespaceSegment { index },
                );
            }

            validate_identifier(component)?;
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

/// One source-level attribute argument.
///
/// Positional and named arguments are both retained exactly as structural
/// source data.
///
/// Duplicate named arguments are intentionally representable. Their semantic
/// legality is determined later.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttributeArgument {
    /// Positional source argument.
    Positional(AttributeValue),

    /// Named source argument.
    Named {
        /// Source-level argument identifier.
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
    #[must_use]
    pub fn named(
        name: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        Self::Named {
            name: name.into(),
            value,
        }
    }

    /// Returns the argument value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        match self {
            Self::Positional(value) => value,
            Self::Named { value, .. } => value,
        }
    }

    /// Returns mutable access to the argument value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut AttributeValue {
        match self {
            Self::Positional(value) => value,
            Self::Named { value, .. } => value,
        }
    }

    /// Returns the argument name for named arguments.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Positional(_) => None,
            Self::Named { name, .. } => Some(name),
        }
    }

    /// Returns whether this argument is named.
    #[inline]
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named { .. })
    }

    /// Performs structural validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        match self {
            Self::Positional(value) => value.validate(),

            Self::Named { name, value } => {
                validate_identifier(name)?;
                value.validate()
            }
        }
    }
}

/// Source-oriented value representation for attribute arguments.
///
/// Numeric values remain lexical strings so this foundational AST does not
/// impose machine-width restrictions.
///
/// For example, an integer larger than the host's native integer width can
/// still be represented here and interpreted later by semantic analysis.
///
/// The representation is intentionally independent of the general expression
/// AST to keep this foundational module dependency-light and acyclic.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttributeValue {
    /// Boolean literal.
    Boolean(bool),

    /// Integer literal preserved in lexical form.
    ///
    /// This permits arbitrary source precision without forcing `i64`/`u64`.
    Integer(String),

    /// Decimal/floating literal preserved in lexical form.
    ///
    /// Semantic analysis determines its numeric interpretation.
    Float(String),

    /// String literal.
    String(String),

    /// Character literal.
    Character(char),

    /// Identifier reference.
    Identifier(String),

    /// Qualified source path.
    ///
    /// Each path segment is retained separately.
    Path(Vec<String>),

    /// Ordered collection of values.
    Array(Vec<AttributeValue>),

    /// Ordered object-like collection.
    ///
    /// A vector is intentional so source order and duplicate keys are
    /// preserved. Semantic analysis determines whether duplicates are legal.
    Object(Vec<AttributeObjectEntry>),

    /// Explicit null/unit-like metadata value.
    Null,
}

impl AttributeValue {
    /// Returns whether this value is a collection.
    #[must_use]
    pub fn is_collection(&self) -> bool {
        matches!(self, Self::Array(_) | Self::Object(_))
    }

    /// Performs structural validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        match self {
            Self::Boolean(_)
            | Self::String(_)
            | Self::Character(_)
            | Self::Null => Ok(()),

            Self::Integer(value) => {
                validate_non_empty_literal(value)
            }

            Self::Float(value) => {
                validate_non_empty_literal(value)
            }

            Self::Identifier(value) => {
                if value.is_empty() {
                    return Err(
                        AttributeValidationError::EmptyIdentifier,
                    );
                }

                validate_identifier(value)
            }

            Self::Path(segments) => {
                if segments.is_empty() {
                    return Err(AttributeValidationError::EmptyPath);
                }

                for (index, segment) in segments.iter().enumerate() {
                    if segment.is_empty() {
                        return Err(
                            AttributeValidationError::EmptyPathSegment {
                                index,
                            },
                        );
                    }

                    validate_identifier(segment)?;
                }

                Ok(())
            }

            Self::Array(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Object(entries) => {
                for entry in entries {
                    entry.validate()?;
                }

                Ok(())
            }
        }
    }
}

/// One ordered key/value entry inside an [`AttributeValue::Object`].
///
/// The entry is a struct rather than a tuple so its contract remains
/// self-documenting and can evolve without relying on tuple-field positions.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AttributeObjectEntry {
    /// Object key.
    key: String,

    /// Object value.
    value: AttributeValue,
}

impl AttributeObjectEntry {
    /// Creates a validated-by-use object entry.
    ///
    /// Structural validation is available through [`Self::validate`].
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }

    /// Returns the object key.
    #[inline]
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the object value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Returns mutable access to the object value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut AttributeValue {
        &mut self.value
    }

    /// Performs structural validation.
    pub fn validate(&self) -> Result<(), AttributeValidationError> {
        if self.key.is_empty() {
            return Err(AttributeValidationError::EmptyObjectKey);
        }

        validate_identifier(&self.key)?;
        self.value.validate()
    }
}

/// Structural validation errors for source-level attributes.
///
/// These errors intentionally do not model semantic errors.
///
/// Examples of errors that belong elsewhere:
///
/// - unknown attribute;
/// - unsupported attribute;
/// - invalid attribute for a declaration;
/// - invalid quantum capability;
/// - unavailable hardware;
/// - invalid target constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeValidationError {
    /// Terminal attribute name is empty.
    EmptyName,

    /// Namespace contains an empty component.
    EmptyNamespaceSegment {
        /// Zero-based namespace position.
        index: usize,
    },

    /// An identifier does not satisfy the source identifier grammar used by
    /// this foundational representation.
    InvalidIdentifier {
        /// Invalid identifier text.
        value: String,
    },

    /// Named argument has an empty identifier.
    EmptyArgumentName,

    /// An argument contains an invalid value.
    InvalidArgument {
        /// Zero-based source argument position.
        index: usize,

        /// Underlying validation error.
        source: Box<AttributeValidationError>,
    },

    /// Identifier value is empty.
    EmptyIdentifier,

    /// Path contains no segments.
    EmptyPath,

    /// Path contains an empty segment.
    EmptyPathSegment {
        /// Zero-based path segment position.
        index: usize,
    },

    /// Object key is empty.
    EmptyObjectKey,

    /// Numeric literal contains no source text.
    EmptyLiteral,
}

impl fmt::Display for AttributeValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "attribute name must not be empty",
                )
            }

            Self::EmptyNamespaceSegment { index } => {
                write!(
                    formatter,
                    "attribute namespace segment {index} must not be empty"
                )
            }

            Self::InvalidIdentifier { value } => {
                write!(
                    formatter,
                    "invalid attribute identifier `{value}`"
                )
            }

            Self::EmptyArgumentName => {
                formatter.write_str(
                    "attribute argument name must not be empty",
                )
            }

            Self::InvalidArgument { index, source } => {
                write!(
                    formatter,
                    "invalid attribute argument {index}: {source}"
                )
            }

            Self::EmptyIdentifier => {
                formatter.write_str(
                    "attribute identifier value must not be empty",
                )
            }

            Self::EmptyPath => {
                formatter.write_str(
                    "attribute path must contain at least one segment",
                )
            }

            Self::EmptyPathSegment { index } => {
                write!(
                    formatter,
                    "attribute path segment {index} must not be empty"
                )
            }

            Self::EmptyObjectKey => {
                formatter.write_str(
                    "attribute object key must not be empty",
                )
            }

            Self::EmptyLiteral => {
                formatter.write_str(
                    "attribute numeric literal must not be empty",
                )
            }
        }
    }
}

impl std::error::Error for AttributeValidationError {}

/// Validates an identifier component.
///
/// This is deliberately a small structural validator rather than a semantic
/// resolver.
///
/// ASCII identifier syntax is used here because the actual Zamani lexer is
/// authoritative for the complete language identifier grammar. Keeping this
/// foundational type conservative prevents it from silently accepting
/// malformed structural identifiers that the parser cannot produce.
///
/// Parser-created values therefore arrive already consistent with the lexer,
/// while programmatic construction receives deterministic validation.
fn validate_identifier(
    value: &str,
) -> Result<(), AttributeValidationError> {
    if value.is_empty() {
        return Err(AttributeValidationError::EmptyName);
    }

    let mut characters = value.chars();

    let first = match characters.next() {
        Some(character) => character,
        None => {
            return Err(AttributeValidationError::EmptyName);
        }
    };

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(AttributeValidationError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    for character in characters {
        if !(character == '_'
            || character.is_ascii_alphanumeric())
        {
            return Err(AttributeValidationError::InvalidIdentifier {
                value: value.to_owned(),
            });
        }
    }

    Ok(())
}

/// Validates a lexical literal without interpreting its numeric semantics.
fn validate_non_empty_literal(
    value: &str,
) -> Result<(), AttributeValidationError> {
    if value.is_empty() {
        Err(AttributeValidationError::EmptyLiteral)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{
        SourceId, SourceOffset,
    };

    fn span() -> Span {
        Span::point(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
        )
    }

    #[test]
    fn unqualified_name_is_deterministic() {
        let name = AttributeName::new("inline");

        assert_eq!(name.name(), "inline");
        assert!(name.namespace().is_empty());
        assert!(!name.is_namespaced());
        assert_eq!(name.namespace_depth(), 0);
        assert_eq!(name.to_string(), "inline");
    }

    #[test]
    fn namespaced_name_preserves_order() {
        let name = AttributeName::namespaced(
            ["zamani", "compiler"],
            "inline",
        );

        assert_eq!(
            name.components().collect::<Vec<_>>(),
            vec!["zamani", "compiler", "inline"]
        );

        assert_eq!(
            name.to_string(),
            "zamani::compiler::inline"
        );
    }

    #[test]
    fn flag_attribute_has_no_arguments() {
        let attribute =
            Attribute::flag(AttributeName::new("inline"), span());

        assert!(attribute.is_flag());
        assert!(!attribute.has_arguments());
        assert_eq!(attribute.arguments().len(), 0);
        assert!(attribute.validate().is_ok());
    }

    #[test]
    fn ordered_arguments_are_preserved() {
        let attribute = Attribute::new(
            AttributeName::new("example"),
            vec![
                AttributeArgument::positional(
                    AttributeValue::String("first".to_owned()),
                ),
                AttributeArgument::named(
                    "mode",
                    AttributeValue::String("fast".to_owned()),
                ),
            ],
            span(),
        );

        assert_eq!(attribute.arguments().len(), 2);
        assert!(!attribute.arguments()[0].is_named());
        assert_eq!(
            attribute.arguments()[1].name(),
            Some("mode")
        );
    }

    #[test]
    fn lexical_integer_does_not_have_machine_width() {
        let value = AttributeValue::Integer(
            "1844674407370955161618446744073709551616"
                .to_owned(),
        );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn nested_values_validate_recursively() {
        let value = AttributeValue::Array(vec![
            AttributeValue::Boolean(true),
            AttributeValue::Object(vec![
                AttributeObjectEntry::new(
                    "mode",
                    AttributeValue::String(
                        "fast".to_owned(),
                    ),
                ),
            ]),
        ]);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn duplicate_object_keys_are_preserved() {
        let value = AttributeValue::Object(vec![
            AttributeObjectEntry::new(
                "key",
                AttributeValue::Integer("1".to_owned()),
            ),
            AttributeObjectEntry::new(
                "key",
                AttributeValue::Integer("2".to_owned()),
            ),
        ]);

        assert!(value.validate().is_ok());

        if let AttributeValue::Object(entries) = value {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].key(), "key");
            assert_eq!(entries[1].key(), "key");
        } else {
            panic!("expected object value");
        }
    }

    #[test]
    fn invalid_namespace_is_rejected() {
        let name =
            AttributeName::namespaced(["zamani", ""], "inline");

        assert_eq!(
            name.validate(),
            Err(
                AttributeValidationError::EmptyNamespaceSegment {
                    index: 1
                }
            )
        );
    }

    #[test]
    fn invalid_terminal_name_is_rejected() {
        let name = AttributeName::new("not-valid");

        assert!(matches!(
            name.validate(),
            Err(
                AttributeValidationError::InvalidIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_path_is_rejected() {
        assert_eq!(
            AttributeValue::Path(Vec::new()).validate(),
            Err(AttributeValidationError::EmptyPath)
        );
    }

    #[test]
    fn empty_named_argument_is_rejected() {
        let argument = AttributeArgument::named(
            "",
            AttributeValue::Null,
        );

        assert_eq!(
            argument.validate(),
            Err(AttributeValidationError::EmptyName)
        );
    }

    #[test]
    fn empty_numeric_literal_is_rejected() {
        assert_eq!(
            AttributeValue::Integer(String::new()).validate(),
            Err(AttributeValidationError::EmptyLiteral)
        );
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let original = Attribute::new(
            AttributeName::namespaced(
                ["zamani", "compiler"],
                "property",
            ),
            vec![
                AttributeArgument::named(
                    "enabled",
                    AttributeValue::Boolean(true),
                ),
                AttributeArgument::positional(
                    AttributeValue::Array(vec![
                        AttributeValue::Integer(
                            "123456789012345678901234567890"
                                .to_owned(),
                        ),
                        AttributeValue::Null,
                    ]),
                ),
            ],
            span(),
        );

        let encoded =
            serde_json::to_string(&original).expect(
                "attribute serialization must succeed",
            );

        let decoded: Attribute =
            serde_json::from_str(&encoded).expect(
                "attribute deserialization must succeed",
            );

        assert_eq!(decoded, original);
    }

    #[test]
    fn validation_is_non_semantic() {
        let attribute = Attribute::flag(
            AttributeName::namespaced(
                ["future", "unknown"],
                "attribute",
            ),
            span(),
        );

        // Unknown/future namespaces remain structurally representable.
        assert!(attribute.validate().is_ok());
    }
}